#![allow(dead_code, non_snake_case, unused_parens, unused_variables)]

use super::state::Instance;
use rspice_veriloga_runtime::GeneratedEvalContext;
pub use rspice_veriloga_runtime::{GeneratedNoiseDescriptor, GeneratedNoiseEndpoint, GeneratedNoiseEvaluation, GeneratedNoiseEvaluationError, GeneratedNoiseEvaluationRef, GeneratedNoiseKind, GeneratedNoiseVisitor};

pub static NOISE_SOURCES: [GeneratedNoiseDescriptor; 8] = [
    GeneratedNoiseDescriptor { mechanism: "FLICKER_DP_SP_IFLICK", label: Some("iflick"), kind: GeneratedNoiseKind::Flicker, equation: 13, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(11), name: "dp", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(12), name: "sp", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_N_GND_INTERNAL", label: Some("internal"), kind: GeneratedNoiseKind::White, equation: 15, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(7), name: "n", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: None, name: "0", is_internal: false }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_DP_SP_IDS", label: Some("ids"), kind: GeneratedNoiseKind::White, equation: 16, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(11), name: "dp", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(12), name: "sp", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_SP_S_ISOURCE", label: Some("isource"), kind: GeneratedNoiseKind::White, equation: 20, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(12), name: "sp", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(2), name: "s", is_internal: false }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_D_DP_IDRAIN", label: Some("idrain"), kind: GeneratedNoiseKind::White, equation: 21, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(0), name: "d", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(11), name: "dp", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_GP_DP_IIGD", label: Some("iigd"), kind: GeneratedNoiseKind::White, equation: 22, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "gp", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(11), name: "dp", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_GP_SP_IIGS", label: Some("iigs"), kind: GeneratedNoiseKind::White, equation: 23, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "gp", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(12), name: "sp", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_GP_BP_IIGB", label: Some("iigb"), kind: GeneratedNoiseKind::White, equation: 24, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "gp", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(6), name: "bp", is_internal: true }, table_len: 0, table_log_interp: false },
];

impl Instance {
    pub fn evaluate_noise_sources(&self, ctx: &GeneratedEvalContext<'_>, visitor: &mut dyn GeneratedNoiseVisitor) -> Result<(), GeneratedNoiseEvaluationError> {
        if !self.multiplicity.is_finite() || self.multiplicity <= 0.0 {
            return Err(GeneratedNoiseEvaluationError::InvalidMultiplicity { value: self.multiplicity });
        }
        let parameters = &self.params.values;
        let parameter_given = &*self.param_given;
        let temperature = ctx.temperature();
        let node_potentials = [ctx.node_voltage(self.nodes[0]), ctx.node_voltage(self.nodes[1]), ctx.node_voltage(self.nodes[2]), ctx.node_voltage(self.nodes[3]), ctx.node_voltage(self.nodes[4]), ctx.node_voltage(self.nodes[5]), ctx.node_voltage(self.nodes[6]), ctx.node_voltage(self.nodes[7]), ctx.node_voltage(self.nodes[8]), ctx.node_voltage(self.nodes[9]), ctx.node_voltage(self.nodes[10]), ctx.node_voltage(self.nodes[11]), ctx.node_voltage(self.nodes[12])];
            let A = 0e0f64;
            let B = 1e-12f64;
            let C = 5e2f64;
            let D = 2e2f64;
            let E = parameters[24];
            let F = 1e0f64;
            let G = 5e-1f64;
            let H = if parameter_given[172] { 1.0 } else { 0.0 };
            let I = if parameter_given[173] { 1.0 } else { 0.0 };
            let J = if parameter_given[174] { 1.0 } else { 0.0 };
            let K = if parameter_given[9] { 1.0 } else { 0.0 };
            let L = parameters[239];
            let O = parameters[17];
            let P = parameters[207];
            let R = parameters[18];
            let S = parameters[228];
            let V = parameters[165];
            let W = parameters[167];
            let Y = parameters[162];
            let Z = parameters[164];
            let AL = if parameter_given[177] { 1.0 } else { 0.0 };
            let AM = parameters[177];
            let AN = parameters[227];
            let AO = parameters[230];
            let AR = 2e0f64;
            let AS = 1e-1f64;
            let AX = 4e0f64;
            let AY = 8e0f64;
            let AZ = 1.0f64;
            let BA = 0.0f64;
            let BB = 1.0f64;
            let BC = 0.0f64;
            let BD = 3e0f64;
            let BE = 0.0f64;
            let BQ = 1e-50f64;
            let BS = 1e-2f64;
            let BU = 1e-6f64;
            let BZ = 1e-4f64;
            let CH = 1e4f64;
            let CJ = 1e1f64;
            let CK = 2.7315e2f64;
            let CN = parameters[0];
            let CO = parameters[5];
            let CQ = 1e6f64;
            let DI = 4e25f64;
            let DJ = 1.0f64;
            let DK = -4e25f64;
            let DO = 4e25f64;
            let DP = 1.0f64;
            let DQ = -4e25f64;
            let DZ = 1e-3f64;
            let EH = parameters[32];
            let EP = parameters[6];
            let EQ = parameters[7];
            let ER = parameters[8];
            let FO = parameters[168];
            let FP = parameters[170];
            let FZ = parameters[58];
            let GE = 1.6021918e-19f64;
            let GH = 1.034943e-10f64;
            let GO = parameters[246];
            let GS = 1.04e16f64;
            let GV = parameters[75];
            let GY = parameters[115];
            let HB = parameters[25];
            let HD = parameters[2];
            let HK = parameters[46];
            let HP = parameters[33];
            let HQ = node_potentials[5];
            let HR = node_potentials[12];
            let HT = node_potentials[11];
            let HV = node_potentials[6];
            let HX = node_potentials[2];
            let HZ = node_potentials[0];
            let ID = node_potentials[4];
            let IG = 1e-9f64;
            let II = -1e0f64;
            let IP = parameters[31];
            let IQ = 5e0f64;
            let IS = 6e0f64;
            let IU = temperature;
            let JB = 1.3806226e-23f64;
            let KA = 1.8e0f64;
            let KM = parameters[38];
            let KO = 1e2f64;
            let LF = parameters[49];
            let LW = 1.414213562373095e0f64;
            let MD = parameters[226];
            let ME = 3.453133e-11f64;
            let MH = parameters[229];
            let MU = parameters[254];
            let MV = parameters[255];
            let NH = 1.0f64;
            let NI = 0.0f64;
            let NJ = 0.0f64;
            let NK = 1.0f64;
            let NL = 0.0f64;
            let OE = parameters[216];
            let OT = 5e-2f64;
            let OV = 2.0000000000000004e-2f64;
            let OW = 1.0f64;
            let OX = -2.0000000000000004e-2f64;
            let PD = parameters[193];
            let PE = parameters[195];
            let PF = parameters[194];
            let QJ = 2e-3f64;
            let QK = 1.0f64;
            let QL = -2e-3f64;
            let QV = parameters[55];
            let QX = parameters[297];
            let RA = 2.5e-1f64;
            let RC = 5e-3f64;
            let RF = -1e0f64;
            let RV = parameters[57];
            let SC = parameters[72];
            let TA = 2e-1f64;
            let TB = 1.0f64;
            let TC = -2e-1f64;
            let TL = parameters[29];
            let UL = 2.220446049250313e-15f64;
            let UQ = 8e-4f64;
            let VC = 1e-8f64;
            let WS = 0.0f64;
            let XL = -1e-1f64;
            let XV = -1e0f64;
            let YC = 2.220446049250313e-15f64;
            let YS = 2.220446049250313e-15f64;
            let AAA = 1e-13f64;
            let ACK = 2.220446049250313e-15f64;
            let ADF = 1.5e-1f64;
            let ADK = 1.0f64;
            let ADL = 1.0f64;
            let ADM = 0.0f64;
            let ADN = 0.0f64;
            let ADO = 0.0f64;
            let AEK = 2.220446049250313e-15f64;
            let AEZ = 2.220446049250313e-15f64;
            let AGN = 1.0f64;
            let AGO = 1e-10f64;
            let AHN = 1.0f64;
            let AHP = 0.0f64;
            let AHQ = 1e-10f64;
            let AIP = 0.0f64;
            let AIZ = 1.0f64;
            let AJA = 1.0f64;
            let AJB = 0.0f64;
            let AJC = 0.0f64;
            let AJD = 0.0f64;
            let AJR = parameters[15];
            let AJS = 2e-1f64;
            let AJV = parameters[136];
            let AKL = 2.220446049250313e-15f64;
            let AKS = 3e-2f64;
            let AMK = parameters[123];
            let AMR = parameters[16];
            let AMX = parameters[27];
            let AND = 1.0f64;
            let AOK = 8e1f64;
            let AOM = 5.540622384e34f64;
            let APM = 2e1f64;
            let ATG = 2.5e1f64;
            let ATH = 4e1f64;
            let AUU = 0e0f64;
            let AUW = 0e0f64;
            let AWX = 5e-13f64;
            let AXH = 2.220446049250313e-15f64;
            let AXW = 2.220446049250313e-15f64;
            let BKG = 1e-18f64;
            let BKS = parameters[178];
            let BKY = parameters[176];
            let BLH = 1e9f64;
            let BMC = parameters[217];
            let BMG = 2.220446049250313e-15f64;
            let BMJ = 1.034943e-12f64;
            let BNA = parameters[85];
            let BNB = parameters[84];
            let BNH = 3.9e0f64;
            let BNI = 1.17e1f64;
            let BNO = 1e11f64;
            let BOI = parameters[114];
            let BPS = 1.1e0f64;
            let BPY = parameters[240];
            let BPZ = parameters[241];
            let BRD = 1.0f64;
            let BRE = 0.0f64;
            let BRF = 1.0f64;
            let BRG = 0.0f64;
            let BRH = 0.0f64;
            let BSD = parameters[160];
            let BSL = -1e0f64;
            let BTS = 1.0f64;
            let BTT = 0.0f64;
            let BTU = 0.0f64;
            let BTV = 1.0f64;
            let BTW = 0.0f64;
            let BWJ = parameters[209];
            let BWL = parameters[208];
            let BWR = parameters[212];
            let BWS = parameters[260];
            let BXO = parameters[265];
            let BYF = parameters[273];
            let BZA = parameters[198];
            let BZB = parameters[199];
            let BZC = parameters[200];
            let BZS = parameters[45];
            let BZX = parameters[175];
            let CAB = 1e0f64;
            let CAC = 0e0f64;
            let CAL = -0e0f64;
            let CAX = parameters[39];
            let CBG = 9e0f64;
            let CBO = 3.333333333333333e-1f64;
            let CBQ = 1.2e1f64;
            let CBZ = 2.220446049250313e-15f64;
            let CCE = 2.220446049250313e-15f64;
            let CCN = parameters[30];
            let CDY = 2.9693154855771e-1f64;
            let CDZ = 6.115288895133179e-3f64;
            let CED = 7.07106781186548e-1f64;
            let CEE = 1.78800506338833e-2f64;
            let CEF = 6.36964918866352e-5f64;
            let CFA = 4.1e1f64;
            let CFI = -1e0f64;
            let CGL = 0e0f64;
            let CGM = 1e0f64;
            let CGR = -0e0f64;
            let CHY = 2.220446049250313e-15f64;
            let CID = 2.220446049250313e-15f64;
            let CKT = 4.1e1f64;
            let CLB = -1e0f64;
            let CMK = parameters[174];
            let CML = parameters[173];
            let CNF = parameters[223];
            let CNG = parameters[224];
            let CPQ = parameters[303];
            let CSL = parameters[312];
            let CSO = parameters[314];
            let CSS = parameters[311];
            let CSU = parameters[322];
            let CSW = parameters[320];
            let CSX = parameters[321];
            let CSY = parameters[325];
            let CTA = parameters[330];
            let CTB = parameters[331];
            let CTC = parameters[328];
            let CTD = parameters[329];
            let CTE = parameters[326];
            let CTF = parameters[327];
            let CUI = parameters[313];
            let CUN = parameters[310];
            let CWD = parameters[221];
            let M = if L != A { 1.0 } else { 0.0 };
            let AJ;
            if M != 0.0 {
                let N = if parameters[274] <= A { 1.0 } else { 0.0 };
                let AK = if N != 0.0 {
                    F
                } else {
                    A
                };
                AJ = AK;
            } else {
                AJ = A;
            }
            let AH;
            if O != 0.0 {
                let Q = if P <= A { 1.0 } else { 0.0 };
                let AI = if Q != 0.0 {
                    F
                } else {
                    AJ
                };
                AH = AI;
            } else {
                AH = AJ;
            }
            let AF;
            if R != 0.0 {
                let T = if S <= A { 1.0 } else { 0.0 };
                let AG = if T != 0.0 {
                    F
                } else {
                    AH
                };
                AF = AG;
            } else {
                AF = AH;
            }
            let AD;
            if R != 0.0 {
                let U = if parameters[201] <= A { 1.0 } else { 0.0 };
                let AE = if U != 0.0 {
                    F
                } else {
                    AF
                };
                AD = AE;
            } else {
                AD = AF;
            }
            let X = if (if V == A { 1.0 } else { 0.0 }) != 0.0 && (if W < A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let AC = if X != 0.0 {
                F
            } else {
                AD
            };
            let AA = if (if Y == A { 1.0 } else { 0.0 }) != 0.0 && (if Z < A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let AB = if AA != 0.0 {
                F
            } else {
                AC
            };
            if AB != 0.0 {
            } else {
            }
            let AQ = if AL != 0.0 {
                AM
            } else {
                let AP = 5e9f64 / (AN * AO);
                AP
            };
            let AT = if (if AQ < 2.1e0f64 { 1.0 } else { 0.0 }) != 0.0 && 1.0f64 != 0.0 { 1.0 } else { 0.0 };
            let BKR;
            if AT != 0.0 {
                let AU = 2.1e0f64 - AQ;
                let AV = AU * AU;
                let AW = (AV * AV) + 1.0000000000000005e-4f64;
                let BP;
                if AZ != 0.0 {
                    let BK;
                    if BA != 0.0 {
                        BK = F;
                    } else {
                        let BL;
                        if BB != 0.0 {
                            BL = AR;
                        } else {
                            let BM;
                            if BC != 0.0 {
                                BM = BD;
                            } else {
                                let BN = if BE != 0.0 {
                                    AX
                                } else {
                                    A
                                };
                                BM = BN;
                            }
                            BL = BM;
                        }
                        BK = BL;
                    }
                    let mut BF = 0.0;
                    let mut BH = 0.0;
                    BF = A;
                    BH = AW;
                    loop {
                        let BG = if BF < BK { 1.0 } else { 0.0 };
                        if BG == 0.0 {
                            break;
                        }
                        let BI = BH.sqrt();
                        let BJ = BF + F;
                        BF = BJ;
                        BH = BI;
                    }
                    BP = BH;
                } else {
                    let BO = AW.powf(2.5e-1f64);
                    BP = BO;
                }
                let BR = 2.1e0f64 - ((AU * AS) * (F / (BP + BQ)));
                BKR = BR;
            } else {
                BKR = AQ;
            }
            let BT = parameters[34] * BS;
            let BV = parameters[59] / BU;
            let BW = parameters[101] * BS;
            let BX = parameters[192] / BU;
            let BY = parameters[219] * BS;
            let CA = parameters[218] / BZ;
            let CB = parameters[220] / BZ;
            let CC = parameters[231] / BU;
            let CD = parameters[40] / BU;
            let CE = parameters[236] / BU;
            let CF = parameters[197] / BS;
            let CG = parameters[307] / BU;
            let CI = parameters[189] * CH;
            let CL = parameters[222] + CK;
            let CM = parameters[9] + CK;
            let CP = parameters[1] / CO;
            let CR = CN * CQ;
            let CS = CP * CQ;
            let CT = CS * CR;
            let CU = parameters[62] / (CT.powf(parameters[63]));
            let CV = CN + CU;
            let CW = parameters[64] / (CT.powf(parameters[65]));
            let CX = CV * CQ;
            let CY = (CP + CU) * CQ;
            let CZ = ((parameters[147] / BU) * (F + (parameters[148] / (CX.powf(parameters[149]))))) * (F + (parameters[150] / (CY.powf(parameters[151]))));
            let DA = AR * ((parameters[152] * (F + (parameters[154] / (CX.powf(parameters[155]))))) * (F + (parameters[156] / (CY.powf(parameters[157])))));
            let DB = DA * parameters[153];
            let DC = (CP - (AR * parameters[41])) - DB;
            let DD = (CP - (AR * parameters[42])) - DB;
            let DE = DC * CO;
            let DF = DD * CO;
            let DG = (parameters[11] + (parameters[304] * parameters[12])) + (parameters[305] * parameters[13]);
            let DH = (((AO / BU) + ((parameters[306] / BU) * DG)) - 1e21f64) - 1e4f64;
            let DL = if DJ != 0.0 {
                DI
            } else {
                DK
            };
            let DM = 1e21f64 + (G * (DH + (((DH * DH) + DL).sqrt())));
            let DN = ((BV + (CG * DG)) - 1e21f64) - 1e4f64;
            let DR = if DP != 0.0 {
                DO
            } else {
                DQ
            };
            let DS = 1e21f64 + (G * (DN + (((DN * DN) + DR).sqrt())));
            let DT = (parameters[86] * (CR.powf(parameters[88]))) * (F + (parameters[90] / (CR.powf(parameters[91]))));
            let DU = (parameters[87] * (CR.powf(parameters[89]))) * (F + (parameters[92] / (CR.powf(parameters[93]))));
            let DV = (parameters[289] * (CR.powf(parameters[291]))) * (F + (parameters[293] / (CR.powf(parameters[294]))));
            let DW = (parameters[290] * (CR.powf(parameters[292]))) * (F + (parameters[295] / (CR.powf(parameters[296]))));
            let DX = (parameters[106] * (F + (parameters[107] / (CR.powf(parameters[110]))))) * (F + (parameters[108] / (CS.powf(parameters[109]))));
            let DY = (parameters[283] * (F + (parameters[285] / (CR.powf(parameters[286]))))) * (F + (parameters[287] / (CS.powf(parameters[288]))));
            let EA = CC * DZ;
            let EB = ((CC * (F + (parameters[232] / (CR.powf(parameters[233]))))) - CE) - EA;
            let EC = (AX * CE) * EA;
            let ED = if EC > A { 1.0 } else { 0.0 };
            let EF = if ED != 0.0 {
                EC
            } else {
                let EE = -EC;
                EE
            };
            let EG = CE + (G * (EB + (((EB * EB) + EF).sqrt())));
            let GK;
            if EH != 0.0 {
                let EI = ((EG * (F + (parameters[234] / (CS.powf(parameters[235]))))) - CE) - EA;
                let EK = if ED != 0.0 {
                    EC
                } else {
                    let EJ = -EC;
                    EJ
                };
                let EL = CE + (G * (EI + (((EI * EI) + EK).sqrt())));
                GK = EL;
            } else {
                GK = EG;
            }
            let EM = DS * (F + (parameters[60] / (CS.powf(parameters[61]))));
            let EN = G * CN;
            let EO = AR / ((F / (parameters[43] + EN)) + (F / (parameters[44] + EN)));
            let ES = if (if (if EP > A { 1.0 } else { 0.0 }) != 0.0 && (if EQ > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if (if CO == F { 1.0 } else { 0.0 }) != 0.0 || (if (if CO > F { 1.0 } else { 0.0 }) != 0.0 && (if ER > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let FA;
            if ES != 0.0 {
                let mut ET = 0.0;
                let mut EV = 0.0;
                ET = A;
                EV = A;
                loop {
                    let EU = if ET < CO { 1.0 } else { 0.0 };
                    if EU == 0.0 {
                        break;
                    }
                    let EW = ET * (ER + CN);
                    let EX = (EV + (F / ((EP + EN) + EW))) + (F / ((EQ + EN) + EW));
                    let EY = ET + F;
                    ET = EY;
                    EV = EX;
                }
                let EZ = (AR * CO) / EV;
                FA = EZ;
            } else {
                FA = A;
            }
            let FB = if FA > A { 1.0 } else { 0.0 };
            let FR;
            let GB;
            if FB != 0.0 {
                let FC = F / (F + parameters[166]);
                let FD = V / FA;
                let FE = if W == A { 1.0 } else { 0.0 };
                let FF = if (if FD == A { 1.0 } else { 0.0 }) != 0.0 && FE != 0.0 { 1.0 } else { 0.0 };
                let FH = if FF != 0.0 {
                    F
                } else {
                    let FG = FD.powf(W);
                    FG
                };
                let FI = V / EO;
                let FJ = if (if FI == A { 1.0 } else { 0.0 }) != 0.0 && FE != 0.0 { 1.0 } else { 0.0 };
                let FL = if FJ != 0.0 {
                    F
                } else {
                    let FK = FI.powf(W);
                    FK
                };
                let FM = (EM * (F + (FC * FH))) / (F + (FC * FL));
                let FN = F / (F + parameters[169]);
                let FQ = (DM * (F + (FN * ((FO / FA).powf(FP))))) / (F + (FN * ((FO / EO).powf(FP))));
                FR = FQ;
                GB = FM;
            } else {
                FR = DM;
                GB = EM;
            }
            let FS = BX / FR;
            let FT = (FS - (F + (parameters[190] / (CS.powf(parameters[191]))))) - BS;
            let FU = (AX * FS) * BS;
            let FV = if FU > A { 1.0 } else { 0.0 };
            let FX = if FV != 0.0 {
                FU
            } else {
                let FW = -FU;
                FW
            };
            let FY = FR * (FS - (G * (FT + (((FT * FT) + FX).sqrt()))));
            let GA = if (if CN > FZ { 1.0 } else { 0.0 }) != 0.0 || (if FZ <= A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let GF = if GA != 0.0 {
                let GC = ((FY * (CN - FZ)) + (GB * FZ)) / CN;
                GC
            } else {
                let GD = GB + (((GB - FY) * (FZ - CN)) / FZ);
                GD
            };
            let GG = GE * GF;
            let GI = GG * GH;
            let GJ = AR * GI;
            let GL = (GE * GK) * GH;
            let GM = L * (CR.powf((-parameters[242])));
            let GN = parameters[243] * (CR.powf((-parameters[244])));
            let GP = GO * ((CR + parameters[248]).powf((-parameters[247])));
            let GQ = if (if CN <= (AR * FZ) { 1.0 } else { 0.0 }) != 0.0 && (if FZ > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let JU = if GQ != 0.0 {
                let GR = ((((AR * GB) - (((GB - FY) * CN) / FZ)) - FY) / FY).ln();
                GR
            } else {
                A
            };
            let GT = 5.1702525384001115e-2f64 * ((GF / GS).ln());
            let GU = 5.1702525384001115e-2f64 * ((FY / GS).ln());
            let GW = ((F + (F / CR)).powf(parameters[77])) * GV;
            let GX = parameters[116] * CR;
            let GZ = (((GX * GY) / (GX + GY)) + parameters[117]) + BQ;
            let HA = F + ((CR.powf(parameters[179])) * parameters[180]);
            let HC = if HB == F { 1.0 } else { 0.0 };
            if HC != 0.0 {
                let HE = if ((parameters[48] * (parameters[3] + (DC / (BD * HD)))) / ((HD * (CN - parameters[4])) * CO)) > DZ { 1.0 } else { 0.0 };
                if HE != 0.0 {
                } else {
                }
            } else {
            }
            let HF = F + (parameters[131] / (CS.powf(parameters[132])));
            let HG = parameters[125] * (F + (parameters[126] / (CR.powf(parameters[127]))));
            let HH = CR / (CR + parameters[124]);
            let HI = parameters[118] * (F + (parameters[120] / (CR.powf(parameters[121]))));
            let HJ = parameters[119] * (F + (parameters[122] / CR));
            let HL = ((CH * DF) * HK) / (CR.powf(parameters[47]));
            let HM = parameters[133] * (F + (parameters[134] / (CR.powf(parameters[135]))));
            let HN = parameters[128] * (F + (parameters[129] / (CR.powf(parameters[130]))));
            let HO = (1.2919089961638799e9f64 / GF).sqrt();
            let HS = HP * (HQ - HR);
            let HU = HP * (HT - HR);
            let HW = HP * (HV - HR);
            let HY = HP * (HQ - HX);
            let IA = HP * (HZ - HX);
            let IB = HP * (HV - HX);
            let IC = if parameters[28] != 0.0 && (if parameters[237] > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let IW;
            if IC != 0.0 {
                let IE = if ID > A { 1.0 } else { 0.0 };
                let IF = if IE != 0.0 {
                    ID
                } else {
                    A
                };
                IW = IF;
            } else {
                IW = A;
            }
            if E != 0.0 {
            } else {
            }
            let IH = if HU >= A { 1.0 } else { 0.0 };
            let MY;
            let OA;
            let OB;
            let CAD;
            let CAE;
            let CAH;
            let CAI;
            let CMP;
            let CPF;
            if IH != 0.0 {
                MY = HW;
                OA = HU;
                OB = HS;
                CAD = F;
                CAE = A;
                CAH = HY;
                CAI = IA;
                CMP = F;
                CPF = IB;
            } else {
                let IJ = HS - HU;
                let IK = -HU;
                let IL = HW - HU;
                let IM = HY - IA;
                let IN = -IA;
                let IO = IB - IA;
                MY = IL;
                OA = IK;
                OB = IJ;
                CAD = A;
                CAE = F;
                CAH = IM;
                CAI = IN;
                CMP = II;
                CPF = IO;
            }
            let IR = if IP >= IQ { 1.0 } else { 0.0 };
            if IR != 0.0 {
            } else {
            }
            let IT = if IP >= IS { 1.0 } else { 0.0 };
            if IT != 0.0 {
            } else {
            }
            let IV = if K != 0.0 {
                CM
            } else {
                IU
            };
            let IX = (IV + parameters[10]) + IW;
            let IY = parameters[37] - (CL * (9.025e-5f64 + (CL * 1e-7f64)));
            let IZ = IX - CL;
            let JA = (IY - (parameters[35] * IZ)) - (parameters[36] * ((IX * IX) - (CL * CL)));
            let JC = GE / (JB * IX);
            let JD = JC * JC;
            let JE = F / JC;
            let JF = GE / (JB * CL);
            let JG = IX / CL;
            let JH = ((parameters[249] * (F + (parameters[95] / (CS.powf(parameters[96]))))) * (F + (parameters[97] / (CR.powf(parameters[98]))))) * (F + (parameters[99] / (CT.powf(parameters[100]))));
            let JI = ((parameters[276] * (F + (parameters[277] / (CS.powf(parameters[278]))))) * (F + (parameters[281] / (CR.powf(parameters[282]))))) * (F + (parameters[279] / (CT.powf(parameters[280]))));
            let JQ;
            let JS;
            if FB != 0.0 {
                let JJ = F / (F + parameters[163]);
                let JK = F + (JJ * ((Y / FA).powf(Z)));
                let JL = F + (JJ * ((Y / EO).powf(Z)));
                let JM = (JH * JK) / JL;
                let JN = (JI * JK) / JL;
                JQ = JM;
                JS = JN;
            } else {
                JQ = JH;
                JS = JI;
            }
            let JO = JG - F;
            let JP = JG.powf(((parameters[111] * (F + (parameters[112] / (CR.powf(parameters[113]))))) + ((parameters[253] * JO) * JO)));
            let JR = JP / JQ;
            let JT = JP / JS;
            let JV = JU * JE;
            let JW = (((F + (parameters[181] / (CR.powf(parameters[182])))) * (F + (parameters[185] / (CR.powf(parameters[186]))))) * (F + (parameters[187] / (CS.powf(parameters[188]))))) * (F + (parameters[183] / (CT.powf(parameters[184]))));
            let JX = (G * (JW + (((JW * JW) + 4e-6f64).sqrt()))) + 1e-13f64;
            let JY = if JX < A { 1.0 } else { 0.0 };
            let JZ = if JY != 0.0 {
                A
            } else {
                JX
            };
            let KB = 4e-1f64 * JG;
            let KC = (AS * JG) * JG;
            let KD = F - JG;
            let KE = BS * ((JZ * BT) / (((1.8000000000000002e-2f64 + (KB * BS)) + (KC * BS)) - ((BW * (F + (parameters[102] / (CR.powf(parameters[103]))))) * KD)));
            let KF = JA.sqrt();
            let KG = JA * KF;
            let KH = (GS * (JG * (JG.sqrt()))) * (((((-JA) / AR) * JC) + ((IY / AR) * JF)).exp());
            let KI = (((3.2043836e-19f64 * CZ) * GH).sqrt()) * (JE.sqrt());
            let KJ = KI * KI;
            let KK = KH * KH;
            let KL = KK * (F / (CZ * CZ));
            let KN = (KM / (parameters[251] + parameters[252])) * CN;
            let KP = ((KM * DZ) + 2.2204460492503132e-17f64).abs();
            let KQ = if KM > A { 1.0 } else { 0.0 };
            let LD;
            if KQ != 0.0 {
                let KR = (KM - KN) - KP;
                let KS = (AX * KM) * KP;
                let KT = if KS > A { 1.0 } else { 0.0 };
                let KV = if KT != 0.0 {
                    KS
                } else {
                    let KU = -KS;
                    KU
                };
                let KW = KM - (G * (KR + (((KR * KR) + KV).sqrt())));
                LD = KW;
            } else {
                let KX = (KN - KM) - KP;
                let KY = (AX * KM) * KP;
                let KZ = if KY > A { 1.0 } else { 0.0 };
                let LB = if KZ != 0.0 {
                    KY
                } else {
                    let LA = -KY;
                    LA
                };
                let LC = KM + (G * (KX + (((KX * KX) + LB).sqrt())));
                LD = LC;
            }
            let LE = CN - (AR * LD);
            let LG = -LF;
            let LH = LG * (F + (parameters[52] / (CR.powf(parameters[53]))));
            let LI = -(LF + (parameters[54] * CR));
            let LJ = ((LG * (F + (parameters[50] / (CR.powf(parameters[51]))))) - LH) - B;
            let LK = (AX * LH) * B;
            let LL = if LK > A { 1.0 } else { 0.0 };
            let LN = if LL != 0.0 {
                LK
            } else {
                let LM = -LK;
                LM
            };
            let LO = ((LH + (G * (LJ + (((LJ * LJ) + LN).sqrt())))) - LI) - B;
            let LP = (AX * LI) * B;
            let LQ = if LP > A { 1.0 } else { 0.0 };
            let LS = if LQ != 0.0 {
                LP
            } else {
                let LR = -LP;
                LR
            };
            let LT = -(LI + (G * (LO + (((LO * LO) + LS).sqrt()))));
            let LU = AR * JE;
            let LV = LU * ((FY / KH).ln());
            let LX = (GG * LW) * (((GH / GG) * JE).sqrt());
            let LY = ((AR * GL) * JE).sqrt();
            let LZ = KH / FY;
            let MA = LZ * LZ;
            let MB = KH / GK;
            let MC = MB * MB;
            let MF = ME / MD;
            let MG = MD / ME;
            let MI = ME / MH;
            let MJ = MH / ME;
            let MK = (-1.6021918e-19f64 * FY) * AN;
            let ML = GH / AN;
            let MM = F / ML;
            let MN = MJ + MM;
            let MO = if DC < IG { 1.0 } else { 0.0 };
            let MT = if MO != 0.0 {
                F
            } else {
                A
            };
            let MP = if DD < IG { 1.0 } else { 0.0 };
            let MS = if MP != 0.0 {
                F
            } else {
                MT
            };
            let MQ = if LE < IG { 1.0 } else { 0.0 };
            let MR = if MQ != 0.0 {
                F
            } else {
                MS
            };
            if MR != 0.0 {
            } else {
            }
            let MW = MV * G;
            let MX = if MU > MW { 1.0 } else { 0.0 };
            let MZ = if MX != 0.0 {
                MW
            } else {
                MU
            };
            let NA = if MY > MZ { 1.0 } else { 0.0 };
            let OC;
            let OD;
            if NA != 0.0 {
                let NB = MY - MZ;
                let NC = MV - MZ;
                let ND = NB * NB;
                let NE = NC * NC;
                let NF = ((NE * NE) * NE) * NE;
                let NG = (((ND * ND) * ND) * ND) + NF;
                let NW;
                if NH != 0.0 {
                    let NR;
                    if NI != 0.0 {
                        NR = F;
                    } else {
                        let NS;
                        if NJ != 0.0 {
                            NS = AR;
                        } else {
                            let NT;
                            if NK != 0.0 {
                                NT = BD;
                            } else {
                                let NU = if NL != 0.0 {
                                    AX
                                } else {
                                    A
                                };
                                NT = NU;
                            }
                            NS = NT;
                        }
                        NR = NS;
                    }
                    let mut NM = 0.0;
                    let mut NO = 0.0;
                    NM = A;
                    NO = NG;
                    loop {
                        let NN = if NM < NR { 1.0 } else { 0.0 };
                        if NN == 0.0 {
                            break;
                        }
                        let NP = NO.sqrt();
                        let NQ = NM + F;
                        NM = NQ;
                        NO = NP;
                    }
                    NW = NO;
                } else {
                    let NV = NG.powf(1.25e-1f64);
                    NW = NV;
                }
                let NX = F / (NW + BQ);
                let NY = ((NC * NF) * NX) / (NG + BQ);
                let NZ = MZ + ((NB * NC) * NX);
                OC = NZ;
                OD = NY;
            } else {
                OC = MY;
                OD = F;
            }
            let OF = (AR * ((OD * OA) / AR)) / OE;
            let OG = OE / (F + (OF * (5e-1f64 + (OF * (1.6666666666666666e-1f64 + (OF * (4.1666666666666664e-2f64 + (OF * (8.333333333333333e-3f64 + (OF * (1.388888888888889e-3f64 + (OF * 1.984126984126984e-4f64))))))))))));
            let OH = if OG < B { 1.0 } else { 0.0 };
            let OI = if OH != 0.0 {
                B
            } else {
                OG
            };
            let OJ = OC + OI;
            let OK = OA + (AR * OI);
            let OL = OB + OI;
            let OM = (AR * GG) * GH;
            let ON = (OM * MG) * MG;
            let OO = OB - LT;
            let OP = F + ((AR / ON) * ((OO - JE) - OC));
            let OQ = (G * (OP + (((OP * OP) + 4e-6f64).sqrt()))) + 1e-13f64;
            let OR = if OQ < A { 1.0 } else { 0.0 };
            let OS = if OR != 0.0 {
                A
            } else {
                OQ
            };
            let OU = (((OO + (ON * (F - ((OS + BQ).sqrt())))) - LV) - AS) - OT;
            let OY = if OW != 0.0 {
                OV
            } else {
                OX
            };
            let OZ = OA / (AS + (G * (OU + (((OU * OU) + OY).sqrt()))));
            let PA = OZ * OZ;
            let PB = F - (F / ((((F + OZ) + PA) + (PA * OZ)) + (PA * PA)));
            let PC = PB * PB;
            let PG = if (if (if PD == A { 1.0 } else { 0.0 }) != 0.0 && (if PE == A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if PF == A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let PJ = if PG != 0.0 {
                A
            } else {
                F
            };
            let PH = GT + LT;
            let PI = PH + (((OM * GT).sqrt()) / MF);
            let PK = if PJ == A { 1.0 } else { 0.0 };
            let QO;
            let QZ;
            let SF;
            if PK != 0.0 {
                let PL = ((LX * MG) * MG) * LX;
                QO = MG;
                QZ = PL;
                SF = MF;
            } else {
                let PM = ((OB - OC) - PI) + PF;
                let PN = (G * (PM + (((PM * PM) + 4e-8f64).sqrt()))) + 1.0000000000000002e-14f64;
                let PO = if PN < A { 1.0 } else { 0.0 };
                let PP = if PO != 0.0 {
                    A
                } else {
                    PN
                };
                let PQ = F / PP;
                let PR = AR * (PI.abs());
                let PS = (LT - PI) + PF;
                let PT = if PS > PR { 1.0 } else { 0.0 };
                let PU = if PT != 0.0 {
                    PS
                } else {
                    PR
                };
                let PV = F / PU;
                let PW = (PV - PQ) - BZ;
                let PX = (AX * PV) * BZ;
                let PY = if PX > A { 1.0 } else { 0.0 };
                let QA = if PY != 0.0 {
                    PX
                } else {
                    let PZ = -PX;
                    PZ
                };
                let QB = (PD * (PV - (G * (PW + (((PW * PW) + QA).sqrt()))))) + PE;
                let QC = if (QB * 1e12f64) < MD { 1.0 } else { 0.0 };
                let QD = if QC != 0.0 {
                    A
                } else {
                    QB
                };
                let QE = MD + QD;
                let QF = ME / QE;
                let QG = QE / ME;
                let QH = ((LX * LX) * QG) * QG;
                QO = QG;
                QZ = QH;
                SF = QF;
            }
            let QI = (G - OJ) - DZ;
            let QM = if QK != 0.0 {
                QJ
            } else {
                QL
            };
            let QN = (GJ * GT).sqrt();
            let QP = (PH + (QN * QO)) + JV;
            let QQ = 9.5e-1f64 * GT;
            let QR = (QQ - (G - (G * (QI + (((QI * QI) + QM).sqrt()))))) - DZ;
            let QS = GT - (QQ - (G * (QR + (((QR * QR) + ((3.8e0f64 * GT) * DZ)).sqrt()))));
            let QT = QS.sqrt();
            let QU = if FZ != A { 1.0 } else { 0.0 };
            let SG = if QU != 0.0 {
                let QW = ((QP - ((GU + LT) + (((((3.2043836e-19f64 * FY) * GH) * GU).sqrt()) * QO))) * (((GH * QO) * ((AR * AN) / (FZ * FZ))) * (QV - GT))) * ((parameters[66] + ((parameters[68] / FZ) * QS)) + (parameters[67] * OK));
                QW
            } else {
                A
            };
            let QY = if QX != A { 1.0 } else { 0.0 };
            let RT;
            if QY != 0.0 {
                let RB = ((JE - ((QZ * JC) * RA)) + LT) + BQ;
                let RD = (OL - RB) - RC;
                let RE = if RB >= A { 1.0 } else { 0.0 };
                let RG = if RE != 0.0 {
                    F
                } else {
                    RF
                };
                let RH = (RB + (G * (RD + (((RD * RD) + (((RG * AX) * RB) * RC)).sqrt())))) - LT;
                let RI = F + (((JC * RH) - F) * (((AX / QZ) * JE) * JE));
                let RJ = (G * (RI + (((RI * RI) + 4e-6f64).sqrt()))) + 1e-13f64;
                let RK = if RJ < A { 1.0 } else { 0.0 };
                let RL = if RK != 0.0 {
                    A
                } else {
                    RJ
                };
                let RM = (GT - (RH + (((QZ * G) * JC) * (F - ((RL + 2.220446049250313e-15f64).sqrt()))))) - RC;
                let RN = (AX * GT) * RC;
                let RO = if RN > A { 1.0 } else { 0.0 };
                let RQ = if RO != 0.0 {
                    RN
                } else {
                    let RP = -RN;
                    RP
                };
                let RR = GT + (QX * ((GT - (G * (RM + (((RM * RM) + RQ).sqrt())))) - GT));
                RT = RR;
            } else {
                RT = GT;
            }
            let RS = QO * GH;
            let RU = QV - RT;
            let RW = CN - RV;
            let RX = (((RS * AN) * AR) * RU) / (RW * RW);
            let RY = (G * (OC + (((OC * OC) + 4e-6f64).sqrt()))) + 1e-13f64;
            let RZ = if RY < A { 1.0 } else { 0.0 };
            let SA = if RZ != 0.0 {
                A
            } else {
                RY
            };
            let SB = RX * (((parameters[69] + ((parameters[71] / CN) * QS)) + (parameters[70] * OK)) + (parameters[250] * SA));
            let SD = if SC > A { 1.0 } else { 0.0 };
            let SI = if SD != 0.0 {
                let SE = (((JA + LV) - (AR * parameters[74])) + (parameters[73] * OK)) * ((SC * AN) / (EN + parameters[56]));
                SE
            } else {
                A
            };
            let SH = SB + SG;
            let SJ = ((SH + ((QN * (QO - (F / (SF + (CI / DC))))) + (parameters[104] / CS))) + SI) + CW;
            let SK = QP - SJ;
            let SL = if GV == A { 1.0 } else { 0.0 };
            let SM = if SL != 0.0 {
                A
            } else {
                F
            };
            let SN = if SM == A { 1.0 } else { 0.0 };
            let TF;
            if SN != 0.0 {
                TF = A;
            } else {
                let SO = OL - parameters[76];
                let SP = if SO < -3e0f64 { 1.0 } else { 0.0 };
                let ST;
                if SP != 0.0 {
                    ST = A;
                } else {
                    let SQ = if SO < A { 1.0 } else { 0.0 };
                    let SU = if SQ != 0.0 {
                        let SR = F + (SO * (F + (SO * (3.333333333333333e-1f64 + (SO * 3.7037037037037035e-2f64)))));
                        SR
                    } else {
                        let SS = F + (SO * (F + (SO * (3.333333333333333e-1f64 + (SO * (4.02052934513951e-2f64 + (SO * 1.48148111111111e-1f64)))))));
                        SS
                    };
                    ST = SU;
                }
                let SV = ST - F;
                let SW = (G * (SV + (((SV * SV) + 4.000000000000001e-2f64).sqrt()))) + 1.0000000000000001e-11f64;
                let SX = if SW < A { 1.0 } else { 0.0 };
                let SY = if SX != 0.0 {
                    A
                } else {
                    SW
                };
                let SZ = (F - (SY * GW)) - OT;
                let TD = if TB != 0.0 {
                    TA
                } else {
                    TC
                };
                let TE = F - (G * (SZ + (((SZ * SZ) + TD).sqrt())));
                TF = TE;
            }
            let TG = (OO + SJ) - TF;
            let TH = JE * ((FY / GK).ln());
            let TI = (LT - SJ) + TF;
            let TJ = LX * QO;
            let TK = TJ * TJ;
            let TO = if TL != 0.0 {
                let TM = OJ + TH;
                TM
            } else {
                let TN = OC + TH;
                TN
            };
            let TP = if TO < A { 1.0 } else { 0.0 };
            if TP != 0.0 {
                let TQ = GK / FY;
                let TR = TQ + F;
                let TS = (JE - TO) + (TQ * (JE + TO));
                let TT = ((LY * LY) * MJ) * MJ;
                let TU = TT * JC;
                let TV = ((AR * TS) * TR) - TU;
                let TW = if ((TV * TV) - (((AX * TR) * TR) * (((TS * TS) + (TU * TO)) + TT))) >= BQ { 1.0 } else { 0.0 };
                if TW != 0.0 {
                } else {
                }
            } else {
                let TX = LX * LX;
                let TY = -(JE + (AR * TO));
                let TZ = F + ((TX * JC) / ((LY * LY) * JC));
                let UA = (((TX * MJ) * MJ) * JC) - ((AR * TY) * TZ);
                let UB = if ((UA * UA) - ((((AX * TZ) * TZ) * TY) * TY)) >= BQ { 1.0 } else { 0.0 };
                if UB != 0.0 {
                } else {
                }
            }
            let UC = AR / JC;
            let UD = UC * ((GK / KH).ln());
            let UE = ((LY * LY) * MN) * MN;
            let UF = -TO;
            let UG = UE * JC;
            let UH = (AR * UF) + UG;
            let UI = UF * UF;
            let UJ = (UH * UH) - (AX * (UI + UE));
            let UK = if UJ >= 2.220446049250313e-15f64 { 1.0 } else { 0.0 };
            let UM = if UK != 0.0 {
                UJ
            } else {
                UL
            };
            let UN = (UH - (UM.sqrt())) / AR;
            let UO = (((UI / UE) / MC).ln()) / (JC + (AR / UF));
            let UP = if UN < UD { 1.0 } else { 0.0 };
            let WJ;
            if UP != 0.0 {
                WJ = UN;
            } else {
                let UR = (UO - UN) - UQ;
                let US = (AX * UO) * UQ;
                let UT = if US > A { 1.0 } else { 0.0 };
                let UV = if UT != 0.0 {
                    US
                } else {
                    let UU = -US;
                    UU
                };
                let UW = UO - (G * (UR + (((UR * UR) + UV).sqrt())));
                WJ = UW;
            }
            let mut UX = 0.0;
            let mut UZ = 0.0;
            let mut WK = 0.0;
            let mut WT = 0.0;
            UX = A;
            UZ = WJ;
            WK = A;
            WT = A;
            loop {
                let UY = if UX < C { 1.0 } else { 0.0 };
                if UY == 0.0 {
                    break;
                }
                let VA = JC * UZ;
                let VB = (-VA).exp();
                let VD = if UZ > VC { 1.0 } else { 0.0 };
                let VM;
                let WC;
                if VD != 0.0 {
                    let VE = VA.exp();
                    let VF = (-LY) * ((((VB + VA) - F) + (MC * (VE - F))).sqrt());
                    let VG = (GL / VF) * (((-VB) + F) + (MC * VE));
                    VM = VF;
                    WC = VG;
                } else {
                    let VH = if UZ < -1e-8f64 { 1.0 } else { 0.0 };
                    let VN;
                    let WD;
                    if VH != 0.0 {
                        let VI = LY * (((VB + VA) - F).sqrt());
                        let VJ = (GL / VI) * ((-VB) + F);
                        VN = VI;
                        WD = VJ;
                    } else {
                        let VK = ((-((GL / JC).sqrt())) * JC) * UZ;
                        let VL = -((GL * JC).sqrt());
                        VN = VK;
                        WD = VL;
                    }
                    VM = VN;
                    WC = WD;
                }
                let VO = ((VM * VM) + 4e-12f64).sqrt();
                let VP = G * (F + (VM / VO));
                let VQ = (G * (VM + VO)) + 1e-16f64;
                let VR = if VQ < A { 1.0 } else { 0.0 };
                let VT;
                let WB;
                if VR != 0.0 {
                    VT = A;
                    WB = A;
                } else {
                    VT = VQ;
                    WB = VP;
                }
                let VS = -MK;
                let VU = (VS - VT) - IG;
                let VV = (AX * VS) * IG;
                let VW = if VV > A { 1.0 } else { 0.0 };
                let VY = if VW != 0.0 {
                    VV
                } else {
                    let VX = -VV;
                    VX
                };
                let VZ = ((VU * VU) + VY).sqrt();
                let WA = VS - (G * (VU + VZ));
                let WE = ((((WA * WA) / AR) / GH) / GE) / FY;
                let WF = UZ - (((((-UZ) + (VM / MI)) - TO) + WE) / ((-1e0f64 + (WC / MI)) + (((AR * WE) * (WB * (WC * (G * (F + (VU / VZ)))))) / WA)));
                let WG = if ((WF - UZ).abs()) < DZ { 1.0 } else { 0.0 };
                let WH = if WG != 0.0 {
                    C
                } else {
                    UX
                };
                let WI = WH + F;
                UX = WI;
                UZ = WF;
                WK = WE;
                WT = VM;
            }
            let WL = if (((1.2919089961638799e9f64 * WK) / FY).sqrt()) > (9.9e-1f64 * AN) { 1.0 } else { 0.0 };
            let XS;
            let ACH;
            let ACQ;
            let AMD;
            if WL != 0.0 {
                let WM = F / SF;
                let WN = F / MI;
                let WO = F / ((WM + MM) + WN);
                let WP = (WM * (WO * (UF + ((WN + (G * MM)) * (-MK))))) / (F - (WO * WM));
                let WQ = TI + WP;
                let WR = TG - (parameters[298] * WP);
                XS = WQ;
                ACH = WR;
                ACQ = WP;
                AMD = WR;
            } else {
                XS = TI;
                ACH = TG;
                ACQ = A;
                AMD = TG;
            }
            let AMZ;
            let ANA;
            let ANB;
            let AUQ;
            let AWG;
            let AXA;
            let AXE;
            let BLV;
            if WS != 0.0 {
                let WU = ((((-MK) * MM) / AR) + JE) - (WT * MM);
                AMZ = A;
                ANA = A;
                ANB = TO;
                AUQ = A;
                AWG = A;
                AXA = A;
                AXE = WU;
                BLV = A;
            } else {
                let XT;
                if TP != 0.0 {
                    let mut WV = 0.0;
                    let mut XC = 0.0;
                    WV = F;
                    XC = A;
                    loop {
                        let WW = if WV <= C { 1.0 } else { 0.0 };
                        if WW == 0.0 {
                            break;
                        }
                        let WX = MI / (3.3163543761348e-29f64 * GK);
                        let WY = F + (MI * MM);
                        let WZ = AR * WX;
                        let XA = WZ * SF;
                        let XB = XA * SF;
                        let XD = (AR * MI) * SF;
                        let XE = ((XD * AR) * WX) * SF;
                        let XF = ((((MI * MI) + ((((WY * WY) - ((AX * WX) * (MI * ((((G * (-MK)) * MM) + JE) + TO)))) * SF) * SF)) + (XD * (WY + (WZ * MK)))) + (XE * XC)).sqrt();
                        let XG = F / XB;
                        let XH = (-(XG * ((((MI + (WY * SF)) + (XA * MK)) + (XB * XC)) - XF))) / (XG * (XB - (XE / (AR * XF))));
                        let XI = if (XH.abs()) < B { 1.0 } else { 0.0 };
                        let XM;
                        let XQ;
                        if XI != 0.0 {
                            XM = XH;
                            XQ = C;
                        } else {
                            let XJ = if XH > AS { 1.0 } else { 0.0 };
                            let XN;
                            if XJ != 0.0 {
                                XN = AS;
                            } else {
                                let XK = if XH < -1e-1f64 { 1.0 } else { 0.0 };
                                let XO = if XK != 0.0 {
                                    XL
                                } else {
                                    XH
                                };
                                XN = XO;
                            }
                            XM = XN;
                            XQ = WV;
                        }
                        let XP = XC + XM;
                        let XR = XQ + F;
                        WV = XR;
                        XC = XP;
                    }
                    XT = XC;
                } else {
                    XT = A;
                }
                let XU = if OB < (XS + XT) { 1.0 } else { 0.0 };
                let AGM;
                let AXB;
                if XU != 0.0 {
                    let XW = if (((1.2919089961638799e9f64 * WK) / FY).sqrt()) < AN { 1.0 } else { 0.0 };
                    let AAQ;
                    if XW != 0.0 {
                        let XX = UF + 2.220446049250313e-15f64;
                        let XY = (AR * XX) + UG;
                        let XZ = XX * XX;
                        let YA = (XY * XY) - (AX * (XZ + UE));
                        let YB = if YA >= 2.220446049250313e-15f64 { 1.0 } else { 0.0 };
                        let YD = if YB != 0.0 {
                            YA
                        } else {
                            YC
                        };
                        let YE = (XY - (YD.sqrt())) / AR;
                        let YF = (((XZ / UE) / MC).ln()) / (JC + (AR / XX));
                        let YG = if YE < UD { 1.0 } else { 0.0 };
                        let AAR;
                        if YG != 0.0 {
                            AAR = YE;
                        } else {
                            let YH = (YF - YE) - UQ;
                            let YI = (AX * YF) * UQ;
                            let YJ = if YI > A { 1.0 } else { 0.0 };
                            let YL = if YJ != 0.0 {
                                YI
                            } else {
                                let YK = -YI;
                                YK
                            };
                            let YM = YF - (G * (YH + (((YH * YH) + YL).sqrt())));
                            AAR = YM;
                        }
                        AAQ = AAR;
                    } else {
                        let YN = -(TO - (((MK / AR) * AN) / GH));
                        let YO = (AR * YN) + UG;
                        let YP = YN * YN;
                        let YQ = (YO * YO) - (AX * (YP + UE));
                        let YR = if YQ >= 2.220446049250313e-15f64 { 1.0 } else { 0.0 };
                        let YT = if YR != 0.0 {
                            YQ
                        } else {
                            YS
                        };
                        let YU = (YO - (YT.sqrt())) / AR;
                        let YV = (((YP / UE) / MC).ln()) / (JC + (AR / YN));
                        let YW = if YU < UD { 1.0 } else { 0.0 };
                        let AAS;
                        if YW != 0.0 {
                            AAS = YU;
                        } else {
                            let YX = (YV - YU) - UQ;
                            let YY = (AX * YV) * UQ;
                            let YZ = if YY > A { 1.0 } else { 0.0 };
                            let ZB = if YZ != 0.0 {
                                YY
                            } else {
                                let ZA = -YY;
                                ZA
                            };
                            let ZC = YV - (G * (YX + (((YX * YX) + ZB).sqrt())));
                            AAS = ZC;
                        }
                        AAQ = AAS;
                    }
                    let ZD = if (((1.2919089961638799e9f64 * WK) / FY).sqrt()) < AN { 1.0 } else { 0.0 };
                    let ACE;
                    if ZD != 0.0 {
                        let mut ZE = 0.0;
                        let mut ZG = 0.0;
                        let mut ACF = 0.0;
                        ZE = A;
                        ZG = AAQ;
                        ACF = A;
                        loop {
                            let ZF = if ZE < C { 1.0 } else { 0.0 };
                            if ZF == 0.0 {
                                break;
                            }
                            let ZH = JC * ZG;
                            let ZI = (-ZH).exp();
                            let ZJ = if ZG > VC { 1.0 } else { 0.0 };
                            let ZS;
                            let AAJ;
                            if ZJ != 0.0 {
                                let ZK = ZH.exp();
                                let ZL = (-LY) * ((((ZI + ZH) - F) + (MC * (ZK - F))).sqrt());
                                let ZM = (GL / ZL) * (((-ZI) + F) + (MC * ZK));
                                ZS = ZL;
                                AAJ = ZM;
                            } else {
                                let ZN = if ZG < -1e-8f64 { 1.0 } else { 0.0 };
                                let ZT;
                                let AAK;
                                if ZN != 0.0 {
                                    let ZO = LY * (((ZI + ZH) - F).sqrt());
                                    let ZP = (GL / ZO) * ((-ZI) + F);
                                    ZT = ZO;
                                    AAK = ZP;
                                } else {
                                    let ZQ = ((-((GL / JC).sqrt())) * JC) * ZG;
                                    let ZR = -((GL * JC).sqrt());
                                    ZT = ZQ;
                                    AAK = ZR;
                                }
                                ZS = ZT;
                                AAJ = AAK;
                            }
                            let ZU = ((ZS * ZS) + 4.0000000000000004e-20f64).sqrt();
                            let ZV = G * (F + (ZS / ZU));
                            let ZW = (G * (ZS + ZU)) + 1.0000000000000001e-20f64;
                            let ZX = if ZW < A { 1.0 } else { 0.0 };
                            let ZZ;
                            let AAI;
                            if ZX != 0.0 {
                                ZZ = A;
                                AAI = A;
                            } else {
                                ZZ = ZW;
                                AAI = ZV;
                            }
                            let ZY = -MK;
                            let AAB = (ZY - ZZ) - AAA;
                            let AAC = (AX * ZY) * AAA;
                            let AAD = if AAC > A { 1.0 } else { 0.0 };
                            let AAF = if AAD != 0.0 {
                                AAC
                            } else {
                                let AAE = -AAC;
                                AAE
                            };
                            let AAG = ((AAB * AAB) + AAF).sqrt();
                            let AAH = ZY - (G * (AAB + AAG));
                            let AAL = ((((AAH * AAH) / AR) / GH) / GE) / FY;
                            let AAM = ZG - (((((-ZG) + (ZS / MI)) - TO) + AAL) / ((-1e0f64 + (AAJ / MI)) + (((AR * AAL) * (AAI * (AAJ * (G * (F + (AAB / AAG)))))) / AAH)));
                            let AAN = if ((AAM - ZG).abs()) < DZ { 1.0 } else { 0.0 };
                            let AAO = if AAN != 0.0 {
                                C
                            } else {
                                ZE
                            };
                            let AAP = AAO + F;
                            ZE = AAP;
                            ZG = AAM;
                            ACF = ZS;
                        }
                        ACE = ACF;
                    } else {
                        let mut AAT = 0.0;
                        let mut AAV = 0.0;
                        let mut ACG = 0.0;
                        AAT = A;
                        AAV = AAQ;
                        ACG = A;
                        loop {
                            let AAU = if AAT < C { 1.0 } else { 0.0 };
                            if AAU == 0.0 {
                                break;
                            }
                            let AAW = JC * AAV;
                            let AAX = (-AAW).exp();
                            let AAY = if AAV > VC { 1.0 } else { 0.0 };
                            let ABH;
                            let ABX;
                            if AAY != 0.0 {
                                let AAZ = AAW.exp();
                                let ABA = (-LY) * ((((AAX + AAW) - F) + (MC * (AAZ - F))).sqrt());
                                let ABB = (GL / ABA) * (((-AAX) + F) + (MC * AAZ));
                                ABH = ABA;
                                ABX = ABB;
                            } else {
                                let ABC = if AAV < -1e-8f64 { 1.0 } else { 0.0 };
                                let ABI;
                                let ABY;
                                if ABC != 0.0 {
                                    let ABD = LY * (((AAX + AAW) - F).sqrt());
                                    let ABE = (GL / ABD) * ((-AAX) + F);
                                    ABI = ABD;
                                    ABY = ABE;
                                } else {
                                    let ABF = ((-((GL / JC).sqrt())) * JC) * AAV;
                                    let ABG = -((GL * JC).sqrt());
                                    ABI = ABF;
                                    ABY = ABG;
                                }
                                ABH = ABI;
                                ABX = ABY;
                            }
                            let ABJ = ((ABH * ABH) + 4.0000000000000004e-20f64).sqrt();
                            let ABK = G * (F + (ABH / ABJ));
                            let ABL = (G * (ABH + ABJ)) + 1.0000000000000001e-20f64;
                            let ABM = if ABL < A { 1.0 } else { 0.0 };
                            let ABO;
                            let ABW;
                            if ABM != 0.0 {
                                ABO = A;
                                ABW = A;
                            } else {
                                ABO = ABL;
                                ABW = ABK;
                            }
                            let ABN = -MK;
                            let ABP = (ABN - ABO) - AAA;
                            let ABQ = (AX * ABN) * AAA;
                            let ABR = if ABQ > A { 1.0 } else { 0.0 };
                            let ABT = if ABR != 0.0 {
                                ABQ
                            } else {
                                let ABS = -ABQ;
                                ABS
                            };
                            let ABU = ((ABP * ABP) + ABT).sqrt();
                            let ABV = ABN - (G * (ABP + ABU));
                            let ABZ = ((((ABV * ABV) / AR) / GH) / GE) / FY;
                            let ACA = AAV - ((((((A - AAV) + (ABH / MI)) + (((ABH + (MK / AR)) * AN) / GH)) - TO) + ABZ) / (((-1e0f64 + (ABX / MI)) + ((ABX * AN) / GH)) + (((AR * ABZ) * (ABW * (ABX * (G * (F + (ABP / ABU)))))) / ABV)));
                            let ACB = if ((ACA - AAV).abs()) < DZ { 1.0 } else { 0.0 };
                            let ACC = if ACB != 0.0 {
                                C
                            } else {
                                AAT
                            };
                            let ACD = ACC + F;
                            AAT = ACD;
                            AAV = ACA;
                            ACG = ABH;
                        }
                        ACE = ACG;
                    }
                    AGM = ACE;
                    AXB = XV;
                } else {
                    AGM = A;
                    AXB = A;
                }
                let ACI = F + ((AX * ((JC * (ACH - OC)) - F)) / (TK * JD));
                let ACJ = if ACI >= 2.220446049250313e-15f64 { 1.0 } else { 0.0 };
                let ACL = if ACJ != 0.0 {
                    ACI
                } else {
                    ACK
                };
                let ACM = ACH + (((TK * JC) * G) * (F - (ACL.sqrt())));
                let ACN = AN / GH;
                let ACO = F / MI;
                let ACP = F / (((F / SF) + ACN) + ACO);
                let ACR = OB - ACQ;
                let ACS = if ACR <= SK { 1.0 } else { 0.0 };
                let ADA;
                if ACS != 0.0 {
                    let ACT = if ACM > A { 1.0 } else { 0.0 };
                    let ACV = if ACT != 0.0 {
                        let ACU = ((((GE * FY) * AR) * GH) * ACM).sqrt();
                        ACU
                    } else {
                        A
                    };
                    let ACW = if MK <= ACV { 1.0 } else { 0.0 };
                    let ACX = if ACW != 0.0 {
                        MK
                    } else {
                        ACV
                    };
                    let ACY = ACP * ((ACH - TO) + ((ACO + (G * ACN)) * (-ACX)));
                    ADA = ACY;
                } else {
                    let ACZ = ACP * ((ACH - TO) + ((ACO + (G * ACN)) * (-MK)));
                    ADA = ACZ;
                }
                let ADB = ACH - (ADA / SF);
                let ADC = if ACR > SK { 1.0 } else { 0.0 };
                let AEB;
                if ADC != 0.0 {
                    let ADD = ACH - ACQ;
                    let ADE = (((((F / MA) / QZ) * ADD) * ADD).ln()) / (JC + (AR / ADD));
                    let ADG = ADE - ADF;
                    let ADH = if (if ADB > ADG { 1.0 } else { 0.0 }) != 0.0 && 1.0f64 != 0.0 { 1.0 } else { 0.0 };
                    let AEC;
                    if ADH != 0.0 {
                        let ADI = (ADB - ADE) + ADF;
                        let ADJ = (ADI * ADI) + 2.25e-2f64;
                        let ADZ;
                        if ADK != 0.0 {
                            let ADU;
                            if ADL != 0.0 {
                                ADU = F;
                            } else {
                                let ADV;
                                if ADM != 0.0 {
                                    ADV = AR;
                                } else {
                                    let ADW;
                                    if ADN != 0.0 {
                                        ADW = BD;
                                    } else {
                                        let ADX = if ADO != 0.0 {
                                            AX
                                        } else {
                                            A
                                        };
                                        ADW = ADX;
                                    }
                                    ADV = ADW;
                                }
                                ADU = ADV;
                            }
                            let mut ADP = 0.0;
                            let mut ADR = 0.0;
                            ADP = A;
                            ADR = ADJ;
                            loop {
                                let ADQ = if ADP < ADU { 1.0 } else { 0.0 };
                                if ADQ == 0.0 {
                                    break;
                                }
                                let ADS = ADR.sqrt();
                                let ADT = ADP + F;
                                ADP = ADT;
                                ADR = ADS;
                            }
                            ADZ = ADR;
                        } else {
                            let ADY = ADJ.sqrt();
                            ADZ = ADY;
                        }
                        let AEA = ADG + ((ADI * ADF) * (F / (ADZ + BQ)));
                        AEC = AEA;
                    } else {
                        AEC = ADB;
                    }
                    AEB = AEC;
                } else {
                    AEB = ADB;
                }
                let AED = if AEB > A { 1.0 } else { 0.0 };
                let AEF = if AED != 0.0 {
                    let AEE = ((1.2919089961638799e9f64 * AEB) / FY).sqrt();
                    AEE
                } else {
                    A
                };
                let AEG = if AEF < AN { 1.0 } else { 0.0 };
                let AEH = if AEG != 0.0 {
                    F
                } else {
                    AR
                };
                let AEI = if AEH == F { 1.0 } else { 0.0 };
                let AGH;
                if AEI != 0.0 {
                    let AEJ = if UJ >= 2.220446049250313e-15f64 { 1.0 } else { 0.0 };
                    let AEL = if AEJ != 0.0 {
                        UJ
                    } else {
                        AEK
                    };
                    let AEM = (UH - (AEL.sqrt())) / AR;
                    let AEN = if AEM < UD { 1.0 } else { 0.0 };
                    let AGI;
                    if AEN != 0.0 {
                        AGI = AEM;
                    } else {
                        let AEO = (UO - AEM) - UQ;
                        let AEP = (AX * UO) * UQ;
                        let AEQ = if AEP > A { 1.0 } else { 0.0 };
                        let AES = if AEQ != 0.0 {
                            AEP
                        } else {
                            let AER = -AEP;
                            AER
                        };
                        let AET = UO - (G * (AEO + (((AEO * AEO) + AES).sqrt())));
                        AGI = AET;
                    }
                    AGH = AGI;
                } else {
                    let AEU = -((TO - AEB) - (((MK / AR) * AN) / GH));
                    let AEV = (AR * AEU) + UG;
                    let AEW = AEU * AEU;
                    let AEX = (AEV * AEV) - (AX * (AEW + UE));
                    let AEY = if AEX >= 2.220446049250313e-15f64 { 1.0 } else { 0.0 };
                    let AFA = if AEY != 0.0 {
                        AEX
                    } else {
                        AEZ
                    };
                    let AFB = (AEV - (AFA.sqrt())) / AR;
                    let AFC = (((AEW / UE) / MC).ln()) / (JC + (AR / AEU));
                    let AFD = if AFB < UD { 1.0 } else { 0.0 };
                    let AGJ;
                    if AFD != 0.0 {
                        AGJ = AFB;
                    } else {
                        let AFE = (AFC - AFB) - UQ;
                        let AFF = (AX * AFC) * UQ;
                        let AFG = if AFF > A { 1.0 } else { 0.0 };
                        let AFI = if AFG != 0.0 {
                            AFF
                        } else {
                            let AFH = -AFF;
                            AFH
                        };
                        let AFJ = AFC - (G * (AFE + (((AFE * AFE) + AFI).sqrt())));
                        AGJ = AFJ;
                    }
                    AGH = AGJ;
                }
                let AFK = if AEI != 0.0 && A != 0.0 { 1.0 } else { 0.0 };
                let AIR;
                let AIT;
                let AUR;
                if AFK != 0.0 {
                    let mut AFL = 0.0;
                    let mut AFN = 0.0;
                    let mut AGL = 0.0;
                    AFL = A;
                    AFN = AGH;
                    AGL = AGM;
                    loop {
                        let AFM = if AFL < C { 1.0 } else { 0.0 };
                        if AFM == 0.0 {
                            break;
                        }
                        let AFO = JC * AFN;
                        let AFP = (-AFO).exp();
                        let AFQ = if AFN > VC { 1.0 } else { 0.0 };
                        let AFZ;
                        let AGB;
                        if AFQ != 0.0 {
                            let AFR = AFO.exp();
                            let AFS = (-LY) * ((((AFP + AFO) - F) + (MC * (AFR - F))).sqrt());
                            let AFT = (GL / AFS) * (((-AFP) + F) + (MC * AFR));
                            AFZ = AFS;
                            AGB = AFT;
                        } else {
                            let AFU = if AFN < -1e-8f64 { 1.0 } else { 0.0 };
                            let AGA;
                            let AGC;
                            if AFU != 0.0 {
                                let AFV = LY * (((AFP + AFO) - F).sqrt());
                                let AFW = (GL / AFV) * ((-AFP) + F);
                                AGA = AFV;
                                AGC = AFW;
                            } else {
                                let AFX = ((-((GL / JC).sqrt())) * JC) * AFN;
                                let AFY = -((GL * JC).sqrt());
                                AGA = AFX;
                                AGC = AFY;
                            }
                            AFZ = AGA;
                            AGB = AGC;
                        }
                        let AGD = AFN - ((((-AFN) + (AFZ / MI)) - TO) / (-1e0f64 + (AGB / MI)));
                        let AGE = if ((AGD - AFN).abs()) < DZ { 1.0 } else { 0.0 };
                        let AGF = if AGE != 0.0 {
                            C
                        } else {
                            AFL
                        };
                        let AGG = AGF + F;
                        AFL = AGG;
                        AFN = AGD;
                        AGL = AFZ;
                    }
                    let AGK = TO + AFN;
                    AIR = AGK;
                    AIT = AGL;
                    AUR = A;
                } else {
                    let AHL;
                    let AHM;
                    if AGN != 0.0 {
                        AHL = ADB;
                        AHM = AGO;
                    } else {
                        AHL = AEB;
                        AHM = DZ;
                    }
                    let mut AGP = 0.0;
                    let mut AGR = 0.0;
                    let mut AHO = 0.0;
                    AGP = A;
                    AGR = AGH;
                    AHO = AGM;
                    loop {
                        let AGQ = if AGP < C { 1.0 } else { 0.0 };
                        if AGQ == 0.0 {
                            break;
                        }
                        let AGS = JC * AGR;
                        let AGT = (-AGS).exp();
                        let AGU = if AGR > VC { 1.0 } else { 0.0 };
                        let AHD;
                        let AHF;
                        if AGU != 0.0 {
                            let AGV = AGS.exp();
                            let AGW = (-LY) * ((((AGT + AGS) - F) + (MC * (AGV - F))).sqrt());
                            let AGX = (GL / AGW) * (((-AGT) + F) + (MC * AGV));
                            AHD = AGW;
                            AHF = AGX;
                        } else {
                            let AGY = if AGR < -1e-8f64 { 1.0 } else { 0.0 };
                            let AHE;
                            let AHG;
                            if AGY != 0.0 {
                                let AGZ = LY * (((AGT + AGS) - F).sqrt());
                                let AHA = (GL / AGZ) * ((-AGT) + F);
                                AHE = AGZ;
                                AHG = AHA;
                            } else {
                                let AHB = ((-((GL / JC).sqrt())) * JC) * AGR;
                                let AHC = -((GL * JC).sqrt());
                                AHE = AHB;
                                AHG = AHC;
                            }
                            AHD = AHE;
                            AHF = AHG;
                        }
                        let AHH = AGR - (((((AHL - AGR) + (AHD / MI)) + (((AHD + (MK / AR)) * AN) / GH)) - TO) / ((-1e0f64 + (AHF / MI)) + ((AHF * AN) / GH)));
                        let AHI = if ((AHH - AGR).abs()) < AHM { 1.0 } else { 0.0 };
                        let AHJ = if AHI != 0.0 {
                            C
                        } else {
                            AGP
                        };
                        let AHK = AHJ + F;
                        AGP = AHK;
                        AGR = AHH;
                        AHO = AHD;
                    }
                    let AUT = if AHN != 0.0 {
                        AHO
                    } else {
                        A
                    };
                    let AIN;
                    let AIO;
                    if AHP != 0.0 {
                        AIN = ADB;
                        AIO = AHQ;
                    } else {
                        AIN = AEB;
                        AIO = DZ;
                    }
                    let mut AHR = 0.0;
                    let mut AHT = 0.0;
                    let mut AIQ = 0.0;
                    AHR = A;
                    AHT = AGR;
                    AIQ = AHO;
                    loop {
                        let AHS = if AHR < C { 1.0 } else { 0.0 };
                        if AHS == 0.0 {
                            break;
                        }
                        let AHU = JC * AHT;
                        let AHV = (-AHU).exp();
                        let AHW = if AHT > VC { 1.0 } else { 0.0 };
                        let AIF;
                        let AIH;
                        if AHW != 0.0 {
                            let AHX = AHU.exp();
                            let AHY = (-LY) * ((((AHV + AHU) - F) + (MC * (AHX - F))).sqrt());
                            let AHZ = (GL / AHY) * (((-AHV) + F) + (MC * AHX));
                            AIF = AHY;
                            AIH = AHZ;
                        } else {
                            let AIA = if AHT < -1e-8f64 { 1.0 } else { 0.0 };
                            let AIG;
                            let AII;
                            if AIA != 0.0 {
                                let AIB = LY * (((AHV + AHU) - F).sqrt());
                                let AIC = (GL / AIB) * ((-AHV) + F);
                                AIG = AIB;
                                AII = AIC;
                            } else {
                                let AID = ((-((GL / JC).sqrt())) * JC) * AHT;
                                let AIE = -((GL * JC).sqrt());
                                AIG = AID;
                                AII = AIE;
                            }
                            AIF = AIG;
                            AIH = AII;
                        }
                        let AIJ = AHT - (((((AIN - AHT) + (AIF / MI)) + (((AIF + (MK / AR)) * AN) / GH)) - TO) / ((-1e0f64 + (AIH / MI)) + ((AIH * AN) / GH)));
                        let AIK = if ((AIJ - AHT).abs()) < AIO { 1.0 } else { 0.0 };
                        let AIL = if AIK != 0.0 {
                            C
                        } else {
                            AHR
                        };
                        let AIM = AIL + F;
                        AHR = AIM;
                        AHT = AIJ;
                        AIQ = AIF;
                    }
                    let AUS = if AIP != 0.0 {
                        AIQ
                    } else {
                        AUT
                    };
                    AIR = AHT;
                    AIT = AIQ;
                    AUR = AUS;
                }
                let AIS = (TO + AIR) - BS;
                let AIU = AIS - (AIT / MI);
                let AIV = AEB - ADF;
                let AIW = if (if AIU > AIV { 1.0 } else { 0.0 }) != 0.0 && 1.0f64 != 0.0 { 1.0 } else { 0.0 };
                let AJQ;
                if AIW != 0.0 {
                    let AIX = (AIU - AEB) + ADF;
                    let AIY = (AIX * AIX) + 2.25e-2f64;
                    let AJO;
                    if AIZ != 0.0 {
                        let AJJ;
                        if AJA != 0.0 {
                            AJJ = F;
                        } else {
                            let AJK;
                            if AJB != 0.0 {
                                AJK = AR;
                            } else {
                                let AJL;
                                if AJC != 0.0 {
                                    AJL = BD;
                                } else {
                                    let AJM = if AJD != 0.0 {
                                        AX
                                    } else {
                                        A
                                    };
                                    AJL = AJM;
                                }
                                AJK = AJL;
                            }
                            AJJ = AJK;
                        }
                        let mut AJE = 0.0;
                        let mut AJG = 0.0;
                        AJE = A;
                        AJG = AIY;
                        loop {
                            let AJF = if AJE < AJJ { 1.0 } else { 0.0 };
                            if AJF == 0.0 {
                                break;
                            }
                            let AJH = AJG.sqrt();
                            let AJI = AJE + F;
                            AJE = AJI;
                            AJG = AJH;
                        }
                        AJO = AJG;
                    } else {
                        let AJN = AIY.sqrt();
                        AJO = AJN;
                    }
                    let AJP = AIV + ((AIX * ADF) * (F / (AJO + BQ)));
                    AJQ = AJP;
                } else {
                    AJQ = AIU;
                }
                AMZ = AEB;
                ANA = AJQ;
                ANB = AIS;
                AUQ = AUR;
                AWG = ADB;
                AXA = AXB;
                AXE = A;
                BLV = AIT;
            }
            let AJT = if (if AJR == F { 1.0 } else { 0.0 }) != 0.0 && (if OB > (XS + AJS) { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let AUK;
            let AWE;
            let BSS;
            let BTJ;
            let BUV;
            let BUY;
            let BVD;
            if AJT != 0.0 {
                let AJU = ((OL - HM) + SJ) - TF;
                let AJW = (((3.2043836e-19f64 * FY) * GH) / JC).sqrt();
                let AJX = (KK / FY) / FY;
                let AJY = ((AJW * AJW) / SF) / SF;
                let AJZ = (AJY * JC) / AR;
                let AKA = ((((F / AJX) / AJY) * (AJU * AJU)).ln()) / (JC + (AR / AJU));
                let AKB = (AKA - (AJU + (AJZ * (F - ((F + ((AX * ((JC * AJU) - F)) / ((AJZ * JC) * AR))).sqrt()))))) - AJV;
                let AKC = AKA - (G * (AKB + (((AKB * AKB) + ((AX * AJV) * AKA)).sqrt())));
                let AKD = JC * AKC;
                let AKE = AKD - F;
                let AKF = AKE + (AJX * (AKD.exp()));
                let AKG = if (if AKF > A { 1.0 } else { 0.0 }) != 0.0 && (if AKE > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let AUL;
                let AWF;
                let BUW;
                let BVE;
                if AKG != 0.0 {
                    let AKH = -JC;
                    let AKI = (((((AR * DC) / JC) * 3.0000000000000002e-2f64) * (AJW * ((AKF.sqrt()) - (AKE.sqrt())))) * (-(((AKH * OK).exp()) - F))) / LE;
                    let AKJ = F + ((AX * ((JC * ACH) - F)) / (TK * JD));
                    let AKK = if AKJ < 2.220446049250313e-15f64 { 1.0 } else { 0.0 };
                    let AKM = if AKK != 0.0 {
                        AKL
                    } else {
                        AKJ
                    };
                    let AKN = ACH + (((TK * JC) * G) * (F - (AKM.sqrt())));
                    let AKO = AKN - AKC;
                    let AKP = if AKO < A { 1.0 } else { 0.0 };
                    let AKQ = if AKP != 0.0 {
                        A
                    } else {
                        AKO
                    };
                    let AKR = 1.3e0f64 * AKQ;
                    let AKT = (AKR - OK) - AKS;
                    let AKU = AKR - (G * (AKT + (((AKT * AKT) + ((AX * AKR) * AKS)).sqrt())));
                    let AKV = if AKU > AKQ { 1.0 } else { 0.0 };
                    let AKW = if AKV != 0.0 {
                        AKQ
                    } else {
                        AKU
                    };
                    let AKX = MD * KO;
                    let AKY = DE * KO;
                    let AKZ = LE * KO;
                    let ALA = if parameters[26] == A { 1.0 } else { 0.0 };
                    let AMU;
                    if ALA != 0.0 {
                        AMU = A;
                    } else {
                        let ALB = ((parameters[141] * GE) * AKY) * AKZ;
                        let ALC = ALB / KF;
                        let ALD = (-(((((parameters[144] * OJ) + SB) + SG) + JA) + parameters[143])) / AKX;
                        let mut ALE = 0.0;
                        let mut ALW = 0.0;
                        ALE = A;
                        ALW = A;
                        loop {
                            let ALF = if ALE <= 9.9e1f64 { 1.0 } else { 0.0 };
                            if ALF == 0.0 {
                                break;
                            }
                            let ALG = (AMD + OI) - ((AKW * (ALE / KO)) + AKC);
                            let ALH = F - (ALG / 4.12e0f64);
                            let ALI = ALD + (ALG / AKX);
                            let ALJ = ALI * ALI;
                            let ALK = (G * (ALH + (((ALH * ALH) + 4e-6f64).sqrt()))) + 1e-13f64;
                            let ALL = if ALK < A { 1.0 } else { 0.0 };
                            let ALM = if ALL != 0.0 {
                                A
                            } else {
                                ALK
                            };
                            let ALN = parameters[142] * (F - ((ALM.sqrt()) * ALM));
                            let ALO = (-ALN) / ALI;
                            let ALP = if ALO < -3.4e1f64 { 1.0 } else { 0.0 };
                            let ALT = if ALP != 0.0 {
                                A
                            } else {
                                let ALQ = ALO.exp();
                                ALQ
                            };
                            let ALR = (((RA * ALC) * ALN) * ALN) * 7.38905609893065e0f64;
                            let ALS = if ((AR * ALI) + ALN) < A { 1.0 } else { 0.0 };
                            let ALX;
                            if ALS != 0.0 {
                                ALX = ALR;
                            } else {
                                let ALU = (ALB * ALJ) * ALT;
                                let ALV = if (if ALU < ALR { 1.0 } else { 0.0 }) != 0.0 || (if ALI < A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                let ALY = if ALV != 0.0 {
                                    ALR
                                } else {
                                    ALU
                                };
                                ALX = ALY;
                            }
                            let ALZ = ALW + ALX;
                            let AMA = if ALX < IG { 1.0 } else { 0.0 };
                            let AMB = if AMA != 0.0 {
                                KO
                            } else {
                                ALE
                            };
                            let AMC = AMB + F;
                            ALE = AMC;
                            ALW = ALZ;
                        }
                        AMU = ALW;
                    }
                    let AME = if (if HI <= A { 1.0 } else { 0.0 }) != 0.0 || (if KE <= A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let AMT;
                    if AME != 0.0 {
                        AMT = A;
                    } else {
                        let AMF = SF * SF;
                        let AMG = F + (((AR / GI) * AMF) * ((AJU - JE) - (HG * OJ)));
                        let AMH = (G * (AMG + (((AMG * AMG) + 4e-6f64).sqrt()))) + 1e-13f64;
                        let AMI = if AMH < A { 1.0 } else { 0.0 };
                        let AMJ = if AMI != 0.0 {
                            A
                        } else {
                            AMH
                        };
                        let AML = ((AMK * OK) + AKC) - ((HH * HF) * ((AJU * HN) + ((GI / AMF) * (F - ((AMJ + BQ).sqrt())))));
                        let AMM = (G * (AML + (((AML * AML) + 4e-4f64).sqrt()))) + 1e-12f64;
                        let AMN = if AMM < A { 1.0 } else { 0.0 };
                        let AMO = if AMN != 0.0 {
                            A
                        } else {
                            AMM
                        };
                        let AMP = AMO + BQ;
                        let AMQ = ((HI * AMP) * AKI) * (((-HJ) / AMP).exp());
                        AMT = AMQ;
                    }
                    let AMS = if AMR == F { 1.0 } else { 0.0 };
                    let AUM;
                    if AMS != 0.0 {
                        let AMV = AKC - ((parameters[139] * JE) * ((F + ((AMT + AMU) * (2.1633307652783932e-2f64 / ((((GE * AN) * DE) * ((AKH * parameters[140]).exp())) * (4.1046315303568966e26f64 + (2.4665765749313358e0f64 * FY)))))).ln()));
                        let AMW = (-(((3.3163543761348e-29f64 * FY) * JE).sqrt())) * ((((((AKH * AMV).exp()) - F) + (JC * AMV)).sqrt()) - (((((AKH * AKC).exp()) - F) + AKD).sqrt()));
                        let AUN = if AMX != 0.0 {
                            let AMY = IG * node_potentials[10];
                            AMY
                        } else {
                            AMW
                        };
                        AUM = AUN;
                    } else {
                        AUM = A;
                    }
                    AUL = AUM;
                    AWF = AKN;
                    BUW = AMT;
                    BVE = AKC;
                } else {
                    AUL = A;
                    AWF = AWG;
                    BUW = A;
                    BVE = A;
                }
                AUK = AUL;
                AWE = AWF;
                BSS = AJX;
                BTJ = AJW;
                BUV = BUW;
                BUY = AJU;
                BVD = BVE;
            } else {
                AUK = A;
                AWE = AWG;
                BSS = KL;
                BTJ = KI;
                BUV = A;
                BUY = A;
                BVD = A;
            }
            let ANC = ANB - TO;
            let AUO;
            let AUP;
            let AUV;
            if AND != 0.0 {
                let ANE = -MK;
                let ANF = -3.7477e0f64 * MK;
                AUO = MK;
                AUP = ANF;
                AUV = ANE;
            } else {
                let ANG = 1.5e0f64 * MK;
                let ANH = -ANG;
                let ANI = -4.8303e0f64 * MK;
                AUO = ANG;
                AUP = ANI;
                AUV = ANH;
            }
            let mut ANJ = 0.0;
            let mut ANL = 0.0;
            let mut AOF = 0.0;
            let mut AOG = 0.0;
            let mut AQY = 0.0;
            let mut AUX = 0.0;
            let mut AVD = 0.0;
            let mut AVG = 0.0;
            let mut AWA = 0.0;
            let mut BKB = 0.0;
            let mut BLU = 0.0;
            let mut BMM = 0.0;
            ANJ = F;
            ANL = ANC;
            AOF = AMZ;
            AOG = ANA;
            AQY = A;
            AUX = A;
            AVD = A;
            AVG = A;
            AWA = ANA;
            BKB = A;
            BLU = BLV;
            BMM = A;
            loop {
                let ANK = if ANJ <= C { 1.0 } else { 0.0 };
                if ANK == 0.0 {
                    break;
                }
                let ANM = JC * ANL;
                let ANN = (-ANM).exp();
                let ANO = if ANL < -1e-8f64 { 1.0 } else { 0.0 };
                let ANZ;
                let AOC;
                if ANO != 0.0 {
                    let ANP = ANM.exp();
                    let ANQ = LY * ((((ANN + ANM) - F) + (MC * (ANP - F))).sqrt());
                    let ANR = (GL * (((-ANN) + F) + (MC * ANP))) / ANQ;
                    ANZ = ANQ;
                    AOC = ANR;
                } else {
                    let ANS = if ANL > 1e-9f64 { 1.0 } else { 0.0 };
                    let AOA;
                    let AOD;
                    if ANS != 0.0 {
                        let ANT = ANM.exp();
                        let ANU = (-LY) * ((((ANN + ANM) - F) + (MC * ((ANT - ANM) - F))).sqrt());
                        let ANV = (GL * (((-ANN) + F) + (MC * (ANT - F)))) / ANU;
                        AOA = ANU;
                        AOD = ANV;
                    } else {
                        let ANW = -LY;
                        let ANX = (ANW * ANM) / 1.4142135623730951e0f64;
                        let ANY = (ANW * JC) / 1.4142135623730951e0f64;
                        AOA = ANX;
                        AOD = ANY;
                    }
                    ANZ = AOA;
                    AOC = AOD;
                }
                let AOB = ((ANL - (ANZ / MI)) + OC) + TH;
                let AOE = F - (AOC / MI);
                let AOH = AOF - AOG;
                let AOI = JC * AOH;
                let AOJ = -AOI;
                let AOL = if AOJ >= AOK { 1.0 } else { 0.0 };
                let AOQ;
                let AOT;
                if AOL != 0.0 {
                    let AON = AOM * ((F + AOJ) - AOK);
                    AOQ = AON;
                    AOT = AOM;
                } else {
                    let AOO = AOJ.exp();
                    AOQ = AOO;
                    AOT = AOO;
                }
                let AOP = if AOH < -1e-8f64 { 1.0 } else { 0.0 };
                let ARA;
                let ARC;
                let ARJ;
                let ARL;
                let ARO;
                let ARQ;
                if AOP != 0.0 {
                    let AOR = ((AOQ + AOI) - F).sqrt();
                    let AOS = LX * AOR;
                    let AOU = ((LX * JC) * ((-AOT) + F)) / (AR * AOR);
                    let AOV = -AOU;
                    ARA = A;
                    ARC = AOS;
                    ARJ = A;
                    ARL = AOU;
                    ARO = A;
                    ARQ = AOV;
                } else {
                    let AOW = if AOH > 1e-8f64 { 1.0 } else { 0.0 };
                    let ARB;
                    let ARD;
                    let ARK;
                    let ARM;
                    let ARP;
                    let ARR;
                    if AOW != 0.0 {
                        let AOX = ((AOQ + AOI) - F).sqrt();
                        let AOY = -LX;
                        let AOZ = AOY * AOX;
                        let APA = ((AOY * JC) * ((-AOT) + F)) / (AR * AOX);
                        let APB = -APA;
                        let APC = AOI.exp();
                        let APD = (JC * AOG).exp();
                        let APE = LX * LX;
                        let APF = (((AOZ * AOZ) / APE) + (((AR * MA) * APD) * ((APC - AOI) - F))).sqrt();
                        let APG = AR * AOZ;
                        let APH = ((AR * JC) * MA) * APD;
                        let API = AR * APF;
                        let APJ = (AOY * APF) - AOZ;
                        let APK = (AOY * ((((APG * APA) / APE) + (APH * (APC - F))) / API)) - APA;
                        let APL = (AOY * ((((APG * APB) / APE) - (APH * AOI)) / API)) - APB;
                        ARB = APJ;
                        ARD = AOZ;
                        ARK = APK;
                        ARM = APA;
                        ARP = APL;
                        ARR = APB;
                    } else {
                        let APN = -LX;
                        let APO = (APN * AOI) / 1.4142135623730951e0f64;
                        let APP = (APN * JC) / 1.4142135623730951e0f64;
                        let APQ = -APP;
                        ARB = A;
                        ARD = APO;
                        ARK = A;
                        ARM = APP;
                        ARP = A;
                        ARR = APQ;
                    }
                    ARA = ARB;
                    ARC = ARD;
                    ARJ = ARK;
                    ARL = ARM;
                    ARO = ARP;
                    ARQ = ARR;
                }
                let APR = AOB - AOG;
                let APS = JC * APR;
                let APT = -APS;
                let APU = if APT >= AOK { 1.0 } else { 0.0 };
                let APY;
                let AQB;
                if APU != 0.0 {
                    let APV = AOM * ((F + APT) - AOK);
                    APY = APV;
                    AQB = AOM;
                } else {
                    let APW = APT.exp();
                    APY = APW;
                    AQB = APW;
                }
                let APX = if APR < -1e-8f64 { 1.0 } else { 0.0 };
                let ARE;
                let ARG;
                let ARS;
                let ARU;
                let ARX;
                let ARZ;
                if APX != 0.0 {
                    let APZ = ((APY + APS) - F).sqrt();
                    let AQA = LX * APZ;
                    let AQC = ((LX * JC) * ((-AQB) + F)) / (AR * APZ);
                    let AQD = -AQC;
                    ARE = A;
                    ARG = AQA;
                    ARS = A;
                    ARU = AQD;
                    ARX = A;
                    ARZ = AQC;
                } else {
                    let AQE = if APR > 1e-8f64 { 1.0 } else { 0.0 };
                    let ARF;
                    let ARH;
                    let ART;
                    let ARV;
                    let ARY;
                    let ASA;
                    if AQE != 0.0 {
                        let AQF = ((APY + APS) - F).sqrt();
                        let AQG = -LX;
                        let AQH = AQG * AQF;
                        let AQI = ((AQG * JC) * ((-AQB) + F)) / (AR * AQF);
                        let AQJ = -AQI;
                        let AQK = APS.exp();
                        let AQL = (JC * AOG).exp();
                        let AQM = LX * LX;
                        let AQN = (((AQH * AQH) / AQM) + (((AR * MA) * AQL) * ((AQK - APS) - F))).sqrt();
                        let AQO = AR * AQH;
                        let AQP = ((AR * JC) * MA) * AQL;
                        let AQQ = AR * AQN;
                        let AQR = (AQG * AQN) - AQH;
                        let AQS = (AQG * ((((AQO * AQI) / AQM) + (AQP * (AQK - F))) / AQQ)) - AQI;
                        let AQT = (AQG * ((((AQO * AQJ) / AQM) - (AQP * APS)) / AQQ)) - AQJ;
                        ARF = AQR;
                        ARH = AQH;
                        ART = AQT;
                        ARV = AQJ;
                        ARY = AQS;
                        ASA = AQI;
                    } else {
                        let AQU = -LX;
                        let AQV = (AQU * APS) / 1.4142135623730951e0f64;
                        let AQW = (AQU * JC) / 1.4142135623730951e0f64;
                        let AQX = -AQW;
                        ARF = A;
                        ARH = AQV;
                        ART = A;
                        ARV = AQX;
                        ARY = A;
                        ASA = AQW;
                    }
                    ARE = ARF;
                    ARG = ARH;
                    ARS = ART;
                    ARU = ARV;
                    ARX = ARY;
                    ARZ = ASA;
                }
                let AQZ = if AQY == F { 1.0 } else { 0.0 };
                let AUD;
                let AUF;
                let AUG;
                let AUH;
                let AUI;
                let AUY;
                if AQZ != 0.0 {
                    AUD = C;
                    AUF = ANL;
                    AUG = AOF;
                    AUH = AOG;
                    AUI = AQY;
                    AUY = ANJ;
                } else {
                    let ARI = (AOF - ACH) - ((((((ANZ + ARA) + ARC) + ARE) + ARG) + AUK) / SF);
                    let ARN = F - ((ARJ + ARL) / SF);
                    let ARW = (-(((ARO + ARQ) + ARS) + ARU)) / SF;
                    let ASB = (-(AOC + ((ARX + ARZ) * AOE))) / SF;
                    let ASC = if ANZ <= AUO { 1.0 } else { 0.0 };
                    if ASC != 0.0 {
                    } else {
                        let ASD = if ANZ <= AUP { 1.0 } else { 0.0 };
                        if ASD != 0.0 {
                        } else {
                        }
                    }
                    let ASE = (-AUQ) / MK;
                    let ASF = (ARC + (-(MK + ((F / (F + ((-(ASE * AUU)).exp()))) * AUV)))) / ML;
                    let ASG = ARL / ML;
                    let ASH = ARQ / ML;
                    let ASI = A / ML;
                    let ASJ = (ARG + ((F / (F + ((-(ASE * AUW)).exp()))) * AUV)) / ML;
                    let ASK = ARU / ML;
                    let ASL = (ARZ * AOE) / ML;
                    let ASM = ARN * ASH;
                    let ASN = ARN * ASI;
                    let ASO = ARW * ASG;
                    let ASP = ASB * ASG;
                    let ASQ = (((ASM * ASL) - (ASN * ASK)) - (ASO * ASL)) + (ASP * ASK);
                    let ASR = if ASQ > A { 1.0 } else { 0.0 };
                    let ASU = if ASR != 0.0 {
                        let ASS = F / (ASQ + BQ);
                        ASS
                    } else {
                        let AST = F / (ASQ - BQ);
                        AST
                    };
                    let ASV = -ASU;
                    let ASW = ASV * (((((ASH * ASL) - (ASI * ASK)) * ARI) + (((ASB * ASK) - (ARW * ASL)) * ASF)) + (((ARW * ASI) - (ASB * ASH)) * ASJ));
                    let ASX = ASV * (((((-ASG) * ASL) * ARI) + ((ARN * ASL) * ASF)) + ((ASP - ASN) * ASJ));
                    let ASY = ASV * ((((ASG * ASK) * ARI) + (((-ARN) * ASK) * ASF)) + ((ASM - ASO) * ASJ));
                    let ASZ = ASW.abs();
                    let ATA = ASX.abs();
                    let ATB = if ASZ < ATA { 1.0 } else { 0.0 };
                    let ATC = if ATB != 0.0 {
                        ATA
                    } else {
                        ASZ
                    };
                    let ATD = ASY.abs();
                    let ATE = if ATC < ATD { 1.0 } else { 0.0 };
                    let ATL = if ATE != 0.0 {
                        ATD
                    } else {
                        ATC
                    };
                    let ATF = if ANJ > AOK { 1.0 } else { 0.0 };
                    let ATM;
                    if ATF != 0.0 {
                        ATM = ATG;
                    } else {
                        let ATI = if ANJ > ATH { 1.0 } else { 0.0 };
                        let ATN;
                        if ATI != 0.0 {
                            ATN = ATG;
                        } else {
                            let ATJ = if ANJ > APM { 1.0 } else { 0.0 };
                            let ATO;
                            if ATJ != 0.0 {
                                ATO = ATG;
                            } else {
                                let ATK = if ANJ > CJ { 1.0 } else { 0.0 };
                                let ATP = if ATK != 0.0 {
                                    IQ
                                } else {
                                    F
                                };
                                ATO = ATP;
                            }
                            ATN = ATO;
                        }
                        ATM = ATN;
                    }
                    let ATQ = AS / ATM;
                    let ATR = if ATL > ATQ { 1.0 } else { 0.0 };
                    let ATW;
                    let ATY;
                    let AUA;
                    if ATR != 0.0 {
                        let ATS = ATQ / ATL;
                        let ATT = ASW * ATS;
                        let ATU = ASX * ATS;
                        let ATV = ASY * ATS;
                        ATW = ATT;
                        ATY = ATU;
                        AUA = ATV;
                    } else {
                        ATW = ASW;
                        ATY = ASX;
                        AUA = ASY;
                    }
                    let ATX = AOF + ATW;
                    let ATZ = AOG + ATY;
                    let AUB = ANL + AUA;
                    let AUC = if ATL < (B * ATM) { 1.0 } else { 0.0 };
                    let AUJ = if AUC != 0.0 {
                        F
                    } else {
                        AQY
                    };
                    AUD = ANJ;
                    AUF = AUB;
                    AUG = ATX;
                    AUH = ATZ;
                    AUI = AUJ;
                    AUY = AUX;
                }
                let AUE = AUD + F;
                ANJ = AUE;
                ANL = AUF;
                AOF = AUG;
                AOG = AUH;
                AQY = AUI;
                AUX = AUY;
                AVD = ARA;
                AVG = ARE;
                AWA = AOB;
                BKB = ARC;
                BLU = ANZ;
                BMM = ARG;
            }
            let AUZ = if AUX > A { 1.0 } else { 0.0 };
            let AVA;
            let BJC;
            if AUZ != 0.0 {
                AVA = AUX;
                BJC = A;
            } else {
                AVA = ANJ;
                BJC = AUX;
            }
            let AVB = if AVA > C { 1.0 } else { 0.0 };
            let AVC;
            let AVZ;
            let AWB;
            let AWC;
            if AVB != 0.0 {
                AVC = AMZ;
                AVZ = ANA;
                AWB = ANA;
                AWC = ANC;
            } else {
                AVC = AOF;
                AVZ = AWA;
                AWB = AOG;
                AWC = ANL;
            }
            if AVB != 0.0 {
            } else {
            }
            let AVE = -AVD;
            let AVF = if AVE <= BQ { 1.0 } else { 0.0 };
            let AVJ;
            let BKP;
            if AVF != 0.0 {
                AVJ = BQ;
                BKP = F;
            } else {
                AVJ = AVE;
                BKP = A;
            }
            let AVH = -AVG;
            let AVI = if AVH <= BQ { 1.0 } else { 0.0 };
            let BOY = if AVI != 0.0 {
                BQ
            } else {
                AVH
            };
            let AVK = AVJ * QO;
            let AVL = SF * SF;
            let AVM = GI / AVL;
            let AVN = F + ((AR / AVM) * (ACH - JE));
            let AVO = (G * (AVN + (((AVN * AVN) + 1.0000000000000002e-2f64).sqrt()))) + 5.0000000000000005e-12f64;
            let AVP = if AVO < A { 1.0 } else { 0.0 };
            let AVQ = if AVP != 0.0 {
                A
            } else {
                AVO
            };
            let AVR = ACH + (AVM * (F - (AVQ.sqrt())));
            let AVS = (G * (AVR + (((AVR * AVR) + 4e-4f64).sqrt()))) + 1e-12f64;
            let AVT = if AVS < A { 1.0 } else { 0.0 };
            let AVU = if AVT != 0.0 {
                A
            } else {
                AVS
            };
            let AVV = (OA / AVU) + BQ;
            let AVW = F + ((AVV.powf((GZ - F))) * AVV);
            let AVX = OA / ((AVW.powf(((F / GZ) - F))) * AVW);
            let AVY = if AVX < A { 1.0 } else { 0.0 };
            let BBP;
            let BBR;
            let BBT;
            let BLX;
            if AVY != 0.0 {
                BBP = AVC;
                BBR = AVZ;
                BBT = AWC;
                BLX = A;
            } else {
                let AWD = if 0.0f64 != 0.0 || (if AVK < B { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let BBQ;
                let BBS;
                let BBU;
                let BLY;
                if AWD != 0.0 {
                    BBQ = A;
                    BBS = A;
                    BBU = TO;
                    BLY = A;
                } else {
                    let AWH = AWE - AVC;
                    let AWI = if AWH >= A { 1.0 } else { 0.0 };
                    let AWJ = if AWI != 0.0 {
                        AWH
                    } else {
                        A
                    };
                    let AWK = ((1.15e0f64 * AWJ) - AVX) - AKS;
                    let AWL = (AX * (1.15e0f64 * AWJ)) * AKS;
                    let AWM = if AWL > A { 1.0 } else { 0.0 };
                    let AWO = if AWM != 0.0 {
                        AWL
                    } else {
                        let AWN = -AWL;
                        AWN
                    };
                    let AWP = (1.15e0f64 * AWJ) - (G * (AWK + (((AWK * AWK) + AWO).sqrt())));
                    let AWQ = if AWP <= AWJ { 1.0 } else { 0.0 };
                    let AWR = if AWQ != 0.0 {
                        AWP
                    } else {
                        AWJ
                    };
                    let AWS = if AWR < A { 1.0 } else { 0.0 };
                    let AWU;
                    if AWS != 0.0 {
                        AWU = A;
                    } else {
                        let AWT = if AWR > AVX { 1.0 } else { 0.0 };
                        let AWV = if AWT != 0.0 {
                            AVX
                        } else {
                            AWR
                        };
                        AWU = AWV;
                    }
                    let AWW = AVC + AWU;
                    let AWY = if AWW < AWX { 1.0 } else { 0.0 };
                    let AWZ = if AWY != 0.0 {
                        AWX
                    } else {
                        AWW
                    };
                    let AXC = if AXA == -1e0f64 { 1.0 } else { 0.0 };
                    let AXD = if AXC != 0.0 {
                        AVC
                    } else {
                        AWZ
                    };
                    let AXF = if AXD < AXE { 1.0 } else { 0.0 };
                    let AZT;
                    if AXF != 0.0 {
                        let AXG = if UJ >= 2.220446049250313e-15f64 { 1.0 } else { 0.0 };
                        let AXI = if AXG != 0.0 {
                            UJ
                        } else {
                            AXH
                        };
                        let AXJ = (UH - (AXI.sqrt())) / AR;
                        let AXK = if AXJ < UD { 1.0 } else { 0.0 };
                        let AZU;
                        if AXK != 0.0 {
                            AZU = AXJ;
                        } else {
                            let AXL = (UO - AXJ) - UQ;
                            let AXM = (AX * UO) * UQ;
                            let AXN = if AXM > A { 1.0 } else { 0.0 };
                            let AXP = if AXN != 0.0 {
                                AXM
                            } else {
                                let AXO = -AXM;
                                AXO
                            };
                            let AXQ = UO - (G * (AXL + (((AXL * AXL) + AXP).sqrt())));
                            AZU = AXQ;
                        }
                        AZT = AZU;
                    } else {
                        let AXR = -((TO - AXD) - ((MK / AR) * MM));
                        let AXS = (AR * AXR) + UG;
                        let AXT = AXR * AXR;
                        let AXU = (AXS * AXS) - (AX * (AXT + UE));
                        let AXV = if AXU >= 2.220446049250313e-15f64 { 1.0 } else { 0.0 };
                        let AXX = if AXV != 0.0 {
                            AXU
                        } else {
                            AXW
                        };
                        let AXY = (AXS - (AXX.sqrt())) / AR;
                        let AXZ = (((AXT / UE) / MC).ln()) / (JC + (AR / AXR));
                        let AYA = if AXY < UD { 1.0 } else { 0.0 };
                        let AZV;
                        if AYA != 0.0 {
                            AZV = AXY;
                        } else {
                            let AYB = (AXZ - AXY) - UQ;
                            let AYC = (AX * AXZ) * UQ;
                            let AYD = if AYC > A { 1.0 } else { 0.0 };
                            let AYF = if AYD != 0.0 {
                                AYC
                            } else {
                                let AYE = -AYC;
                                AYE
                            };
                            let AYG = AXZ - (G * (AYB + (((AYB * AYB) + AYF).sqrt())));
                            AZV = AYG;
                        }
                        AZT = AZV;
                    }
                    let AYH = if AXF != 0.0 && A != 0.0 { 1.0 } else { 0.0 };
                    let BBN;
                    let BBV;
                    let BLZ;
                    if AYH != 0.0 {
                        let mut AYI = 0.0;
                        let mut AYK = 0.0;
                        let mut AZX = 0.0;
                        AYI = A;
                        AYK = AZT;
                        AZX = A;
                        loop {
                            let AYJ = if AYI < D { 1.0 } else { 0.0 };
                            if AYJ == 0.0 {
                                break;
                            }
                            let AYL = JC * AYK;
                            let AYM = (-AYL).exp();
                            let AYN = if AYK > VC { 1.0 } else { 0.0 };
                            let AYW;
                            let AZM;
                            if AYN != 0.0 {
                                let AYO = AYL.exp();
                                let AYP = (-LY) * ((((AYM + AYL) - F) + (MC * (AYO - F))).sqrt());
                                let AYQ = (GL / AYP) * (((-AYM) + F) + (MC * AYO));
                                AYW = AYP;
                                AZM = AYQ;
                            } else {
                                let AYR = if AYK < -1e-8f64 { 1.0 } else { 0.0 };
                                let AYX;
                                let AZN;
                                if AYR != 0.0 {
                                    let AYS = LY * (((AYM + AYL) - F).sqrt());
                                    let AYT = (GL / AYS) * ((-AYM) + F);
                                    AYX = AYS;
                                    AZN = AYT;
                                } else {
                                    let AYU = ((-((GL / JC).sqrt())) * JC) * AYK;
                                    let AYV = -((GL * JC).sqrt());
                                    AYX = AYU;
                                    AZN = AYV;
                                }
                                AYW = AYX;
                                AZM = AZN;
                            }
                            let AYY = ((AYW * AYW) + 4e-12f64).sqrt();
                            let AYZ = G * (F + (AYW / AYY));
                            let AZA = (G * (AYW + AYY)) + 1e-16f64;
                            let AZB = if AZA < A { 1.0 } else { 0.0 };
                            let AZD;
                            let AZL;
                            if AZB != 0.0 {
                                AZD = A;
                                AZL = A;
                            } else {
                                AZD = AZA;
                                AZL = AYZ;
                            }
                            let AZC = -MK;
                            let AZE = (AZC - AZD) - IG;
                            let AZF = (AX * AZC) * IG;
                            let AZG = if AZF > A { 1.0 } else { 0.0 };
                            let AZI = if AZG != 0.0 {
                                AZF
                            } else {
                                let AZH = -AZF;
                                AZH
                            };
                            let AZJ = ((AZE * AZE) + AZI).sqrt();
                            let AZK = AZC - (G * (AZE + AZJ));
                            let AZO = ((((AZK * AZK) / AR) / GH) / GE) / FY;
                            let AZP = AYK - (((((-AYK) + (AYW / MI)) - TO) + AZO) / ((-1e0f64 + (AZM / MI)) + (((AR * AZO) * (AZL * (AZM * (G * (F + (AZE / AZJ)))))) / AZK)));
                            let AZQ = if ((AZP - AYK).abs()) < B { 1.0 } else { 0.0 };
                            let AZR = if AZQ != 0.0 {
                                D
                            } else {
                                AYI
                            };
                            let AZS = AZR + F;
                            AYI = AZS;
                            AYK = AZP;
                            AZX = AYW;
                        }
                        let AZW = TO + AYK;
                        let AZY = AZW - (AZX / MI);
                        BBN = AZY;
                        BBV = AZW;
                        BLZ = AZX;
                    } else {
                        let mut AZZ = 0.0;
                        let mut BAB = 0.0;
                        let mut BBL = 0.0;
                        AZZ = A;
                        BAB = AZT;
                        BBL = A;
                        loop {
                            let BAA = if AZZ < D { 1.0 } else { 0.0 };
                            if BAA == 0.0 {
                                break;
                            }
                            let BAC = JC * BAB;
                            let BAD = (-BAC).exp();
                            let BAE = if BAB > VC { 1.0 } else { 0.0 };
                            let BAN;
                            let BBD;
                            if BAE != 0.0 {
                                let BAF = BAC.exp();
                                let BAG = (-LY) * ((((BAD + BAC) - F) + (MC * (BAF - F))).sqrt());
                                let BAH = (GL / BAG) * (((-BAD) + F) + (MC * BAF));
                                BAN = BAG;
                                BBD = BAH;
                            } else {
                                let BAI = if BAB < -1e-8f64 { 1.0 } else { 0.0 };
                                let BAO;
                                let BBE;
                                if BAI != 0.0 {
                                    let BAJ = LY * (((BAD + BAC) - F).sqrt());
                                    let BAK = (GL / BAJ) * ((-BAD) + F);
                                    BAO = BAJ;
                                    BBE = BAK;
                                } else {
                                    let BAL = ((-((GL / JC).sqrt())) * JC) * BAB;
                                    let BAM = -((GL * JC).sqrt());
                                    BAO = BAL;
                                    BBE = BAM;
                                }
                                BAN = BAO;
                                BBD = BBE;
                            }
                            let BAP = ((BAN * BAN) + 4e-12f64).sqrt();
                            let BAQ = G * (F + (BAN / BAP));
                            let BAR = (G * (BAN + BAP)) + 1e-16f64;
                            let BAS = if BAR < A { 1.0 } else { 0.0 };
                            let BAU;
                            let BBC;
                            if BAS != 0.0 {
                                BAU = A;
                                BBC = A;
                            } else {
                                BAU = BAR;
                                BBC = BAQ;
                            }
                            let BAT = -MK;
                            let BAV = (BAT - BAU) - IG;
                            let BAW = (AX * BAT) * IG;
                            let BAX = if BAW > A { 1.0 } else { 0.0 };
                            let BAZ = if BAX != 0.0 {
                                BAW
                            } else {
                                let BAY = -BAW;
                                BAY
                            };
                            let BBA = ((BAV * BAV) + BAZ).sqrt();
                            let BBB = BAT - (G * (BAV + BBA));
                            let BBF = ((((BBB * BBB) / AR) / GH) / GE) / FY;
                            let BBG = BAB - ((((((AXD - BAB) + (BAN / MI)) + ((BAN + (MK / AR)) * MM)) - TO) + BBF) / (((-1e0f64 + (BBD / MI)) + (BBD * MM)) + (((AR * BBF) * (BBC * (BBD * (G * (F + (BAV / BBA)))))) / BBB)));
                            let BBH = if ((BBG - BAB).abs()) < B { 1.0 } else { 0.0 };
                            let BBI = if BBH != 0.0 {
                                D
                            } else {
                                AZZ
                            };
                            let BBJ = BBI + F;
                            AZZ = BBJ;
                            BAB = BBG;
                            BBL = BAN;
                        }
                        let BBK = TO + BAB;
                        let BBM = BBK - (BBL / MI);
                        BBN = BBM;
                        BBV = BBK;
                        BLZ = BBL;
                    }
                    BBQ = AXD;
                    BBS = BBN;
                    BBU = BBV;
                    BLY = BLZ;
                }
                BBP = BBQ;
                BBR = BBS;
                BBT = BBU;
                BLX = BLY;
            }
            let BBO = if AVK < B { 1.0 } else { 0.0 };
            let BCB;
            let BCC;
            let BCD;
            let BCE;
            if BBO != 0.0 {
                BCB = AVC;
                BCC = AVZ;
                BCD = AWC;
                BCE = AWB;
            } else {
                let BBW = BBT - TO;
                let BBX = if BBR < BBP { 1.0 } else { 0.0 };
                let BBY = if BBX != 0.0 {
                    BBR
                } else {
                    BBP
                };
                BCB = BBP;
                BCC = BBR;
                BCD = BBW;
                BCE = BBY;
            }
            let BBZ = if AXA < A { 1.0 } else { 0.0 };
            let BCA = if BBZ != 0.0 {
                F
            } else {
                A
            };
            let mut BCF = 0.0;
            let mut BCH = 0.0;
            let mut BDB = 0.0;
            let mut BDC = 0.0;
            let mut BFR = 0.0;
            let mut BJB = 0.0;
            let mut BJL = 0.0;
            let mut BJN = 0.0;
            let mut BJR = 0.0;
            let mut BKA = 0.0;
            let mut BLW = 0.0;
            let mut BMO = 0.0;
            BCF = F;
            BCH = BCD;
            BDB = BCB;
            BDC = BCE;
            BFR = BCA;
            BJB = BJC;
            BJL = BCC;
            BJN = A;
            BJR = A;
            BKA = A;
            BLW = BLX;
            BMO = A;
            loop {
                let BCG = if BCF <= D { 1.0 } else { 0.0 };
                if BCG == 0.0 {
                    break;
                }
                let BCI = JC * BCH;
                let BCJ = (-BCI).exp();
                let BCK = if BCH < -1e-8f64 { 1.0 } else { 0.0 };
                let BCV;
                let BCY;
                if BCK != 0.0 {
                    let BCL = BCI.exp();
                    let BCM = LY * ((((BCJ + BCI) - F) + (MC * (BCL - F))).sqrt());
                    let BCN = (GL * (((-BCJ) + F) + (MC * BCL))) / BCM;
                    BCV = BCM;
                    BCY = BCN;
                } else {
                    let BCO = if BCH > 1e-9f64 { 1.0 } else { 0.0 };
                    let BCW;
                    let BCZ;
                    if BCO != 0.0 {
                        let BCP = BCI.exp();
                        let BCQ = (-LY) * ((((BCJ + BCI) - F) + (MC * ((BCP - BCI) - F))).sqrt());
                        let BCR = (GL * (((-BCJ) + F) + (MC * (BCP - F)))) / BCQ;
                        BCW = BCQ;
                        BCZ = BCR;
                    } else {
                        let BCS = -LY;
                        let BCT = (BCS * BCI) / 1.4142135623730951e0f64;
                        let BCU = (BCS * JC) / 1.4142135623730951e0f64;
                        BCW = BCT;
                        BCZ = BCU;
                    }
                    BCV = BCW;
                    BCY = BCZ;
                }
                let BCX = ((BCH - (BCV / MI)) + OC) + TH;
                let BDA = F - (BCY / MI);
                let BDD = BDB - BDC;
                let BDE = JC * BDD;
                let BDF = -BDE;
                let BDG = if BDF >= AOK { 1.0 } else { 0.0 };
                let BDK;
                let BDN;
                if BDG != 0.0 {
                    let BDH = AOM * ((F + BDF) - AOK);
                    BDK = BDH;
                    BDN = AOM;
                } else {
                    let BDI = BDF.exp();
                    BDK = BDI;
                    BDN = BDI;
                }
                let BDJ = if BDD < -1e-8f64 { 1.0 } else { 0.0 };
                let BFT;
                let BFV;
                let BGC;
                let BGE;
                let BGH;
                let BGJ;
                if BDJ != 0.0 {
                    let BDL = ((BDK + BDE) - F).sqrt();
                    let BDM = LX * BDL;
                    let BDO = ((LX * JC) * ((-BDN) + F)) / (AR * BDL);
                    let BDP = -BDO;
                    BFT = A;
                    BFV = BDM;
                    BGC = A;
                    BGE = BDO;
                    BGH = A;
                    BGJ = BDP;
                } else {
                    let BDQ = if BDD > 1e-8f64 { 1.0 } else { 0.0 };
                    let BFU;
                    let BFW;
                    let BGD;
                    let BGF;
                    let BGI;
                    let BGK;
                    if BDQ != 0.0 {
                        let BDR = ((BDK + BDE) - F).sqrt();
                        let BDS = -LX;
                        let BDT = BDS * BDR;
                        let BDU = ((BDS * JC) * ((-BDN) + F)) / (AR * BDR);
                        let BDV = -BDU;
                        let BDW = BDE.exp();
                        let BDX = (JC * (BDC - AVX)).exp();
                        let BDY = LX * LX;
                        let BDZ = (((BDT * BDT) / BDY) + (((AR * MA) * BDX) * ((BDW - BDE) - F))).sqrt();
                        let BEA = AR * BDT;
                        let BEB = ((AR * JC) * MA) * BDX;
                        let BEC = AR * BDZ;
                        let BED = (BDS * BDZ) - BDT;
                        let BEE = (BDS * ((((BEA * BDU) / BDY) + (BEB * (BDW - F))) / BEC)) - BDU;
                        let BEF = (BDS * ((((BEA * BDV) / BDY) - (BEB * BDE)) / BEC)) - BDV;
                        BFU = BED;
                        BFW = BDT;
                        BGD = BEE;
                        BGF = BDU;
                        BGI = BEF;
                        BGK = BDV;
                    } else {
                        let BEG = -LX;
                        let BEH = (BEG * BDE) / 1.4142135623730951e0f64;
                        let BEI = (BEG * JC) / 1.4142135623730951e0f64;
                        let BEJ = -BEI;
                        BFU = A;
                        BFW = BEH;
                        BGD = A;
                        BGF = BEI;
                        BGI = A;
                        BGK = BEJ;
                    }
                    BFT = BFU;
                    BFV = BFW;
                    BGC = BGD;
                    BGE = BGF;
                    BGH = BGI;
                    BGJ = BGK;
                }
                let BEK = BCX - BDC;
                let BEL = JC * BEK;
                let BEM = -BEL;
                let BEN = if BEM >= AOK { 1.0 } else { 0.0 };
                let BER;
                let BEU;
                if BEN != 0.0 {
                    let BEO = AOM * ((F + BEM) - AOK);
                    BER = BEO;
                    BEU = AOM;
                } else {
                    let BEP = BEM.exp();
                    BER = BEP;
                    BEU = BEP;
                }
                let BEQ = if BEK < -1e-8f64 { 1.0 } else { 0.0 };
                let BFX;
                let BFZ;
                let BGL;
                let BGN;
                let BGQ;
                let BGS;
                if BEQ != 0.0 {
                    let BES = ((BER + BEL) - F).sqrt();
                    let BET = LX * BES;
                    let BEV = ((LX * JC) * ((-BEU) + F)) / (AR * BES);
                    let BEW = -BEV;
                    BFX = A;
                    BFZ = BET;
                    BGL = A;
                    BGN = BEW;
                    BGQ = A;
                    BGS = BEV;
                } else {
                    let BEX = if BEK > 1e-8f64 { 1.0 } else { 0.0 };
                    let BFY;
                    let BGA;
                    let BGM;
                    let BGO;
                    let BGR;
                    let BGT;
                    if BEX != 0.0 {
                        let BEY = ((BER + BEL) - F).sqrt();
                        let BEZ = -LX;
                        let BFA = BEZ * BEY;
                        let BFB = ((BEZ * JC) * ((-BEU) + F)) / (AR * BEY);
                        let BFC = -BFB;
                        let BFD = BEL.exp();
                        let BFE = (JC * (BDC - AVX)).exp();
                        let BFF = LX * LX;
                        let BFG = (((BFA * BFA) / BFF) + (((AR * MA) * BFE) * ((BFD - BEL) - F))).sqrt();
                        let BFH = AR * BFA;
                        let BFI = ((AR * JC) * MA) * BFE;
                        let BFJ = AR * BFG;
                        let BFK = (BEZ * BFG) - BFA;
                        let BFL = (BEZ * ((((BFH * BFB) / BFF) + (BFI * (BFD - F))) / BFJ)) - BFB;
                        let BFM = (BEZ * ((((BFH * BFC) / BFF) - (BFI * BEL)) / BFJ)) - BFC;
                        BFY = BFK;
                        BGA = BFA;
                        BGM = BFM;
                        BGO = BFC;
                        BGR = BFL;
                        BGT = BFB;
                    } else {
                        let BFN = -LX;
                        let BFO = (BFN * BEL) / 1.4142135623730951e0f64;
                        let BFP = (BFN * JC) / 1.4142135623730951e0f64;
                        let BFQ = -BFP;
                        BFY = A;
                        BGA = BFO;
                        BGM = A;
                        BGO = BFQ;
                        BGR = A;
                        BGT = BFP;
                    }
                    BFX = BFY;
                    BFZ = BGA;
                    BGL = BGM;
                    BGN = BGO;
                    BGQ = BGR;
                    BGS = BGT;
                }
                let BFS = if BFR == F { 1.0 } else { 0.0 };
                let BIU;
                let BIW;
                let BIX;
                let BIY;
                let BIZ;
                let BJD;
                if BFS != 0.0 {
                    BIU = D;
                    BIW = BCH;
                    BIX = BDB;
                    BIY = BDC;
                    BIZ = BFR;
                    BJD = BCF;
                } else {
                    let BGB = (BDB - ACH) - ((((((BCV + BFT) + BFV) + BFX) + BFZ) + AUK) / SF);
                    let BGG = F - ((BGC + BGE) / SF);
                    let BGP = (-(((BGH + BGJ) + BGL) + BGN)) / SF;
                    let BGU = (-(BCY + ((BGQ + BGS) * BDA))) / SF;
                    let BGV = if BCV <= AUO { 1.0 } else { 0.0 };
                    if BGV != 0.0 {
                    } else {
                        let BGW = if BCV <= AUP { 1.0 } else { 0.0 };
                        if BGW != 0.0 {
                        } else {
                        }
                    }
                    let BGX = (-AUQ) / MK;
                    let BGY = (BFV + (-(MK + ((F / (F + ((-(BGX * AUU)).exp()))) * AUV)))) / ML;
                    let BGZ = BGE / ML;
                    let BHA = BGJ / ML;
                    let BHB = A / ML;
                    let BHC = (BFZ + ((F / (F + ((-(BGX * AUW)).exp()))) * AUV)) / ML;
                    let BHD = BGN / ML;
                    let BHE = (BGS * BDA) / ML;
                    let BHF = BGG * BHA;
                    let BHG = BGG * BHB;
                    let BHH = BGP * BGZ;
                    let BHI = BGU * BGZ;
                    let BHJ = (((BHF * BHE) - (BHG * BHD)) - (BHH * BHE)) + (BHI * BHD);
                    let BHK = if BHJ > A { 1.0 } else { 0.0 };
                    let BHN = if BHK != 0.0 {
                        let BHL = F / (BHJ + BQ);
                        BHL
                    } else {
                        let BHM = F / (BHJ - BQ);
                        BHM
                    };
                    let BHO = -BHN;
                    let BHP = BHO * (((((BHA * BHE) - (BHB * BHD)) * BGB) + (((BGU * BHD) - (BGP * BHE)) * BGY)) + (((BGP * BHB) - (BGU * BHA)) * BHC));
                    let BHQ = BHO * (((((-BGZ) * BHE) * BGB) + ((BGG * BHE) * BGY)) + ((BHI - BHG) * BHC));
                    let BHR = BHO * ((((BGZ * BHD) * BGB) + (((-BGG) * BHD) * BGY)) + ((BHF - BHH) * BHC));
                    let BHS = BHP.abs();
                    let BHT = BHQ.abs();
                    let BHU = if BHS < BHT { 1.0 } else { 0.0 };
                    let BHV = if BHU != 0.0 {
                        BHT
                    } else {
                        BHS
                    };
                    let BHW = BHR.abs();
                    let BHX = if BHV < BHW { 1.0 } else { 0.0 };
                    let BIC = if BHX != 0.0 {
                        BHW
                    } else {
                        BHV
                    };
                    let BHY = if BCF > AOK { 1.0 } else { 0.0 };
                    let BID;
                    if BHY != 0.0 {
                        BID = ATG;
                    } else {
                        let BHZ = if BCF > ATH { 1.0 } else { 0.0 };
                        let BIE;
                        if BHZ != 0.0 {
                            BIE = ATG;
                        } else {
                            let BIA = if BCF > APM { 1.0 } else { 0.0 };
                            let BIF;
                            if BIA != 0.0 {
                                BIF = ATG;
                            } else {
                                let BIB = if BCF > CJ { 1.0 } else { 0.0 };
                                let BIG = if BIB != 0.0 {
                                    IQ
                                } else {
                                    F
                                };
                                BIF = BIG;
                            }
                            BIE = BIF;
                        }
                        BID = BIE;
                    }
                    let BIH = AS / BID;
                    let BII = if BIC > BIH { 1.0 } else { 0.0 };
                    let BIN;
                    let BIP;
                    let BIR;
                    if BII != 0.0 {
                        let BIJ = BIH / BIC;
                        let BIK = BHP * BIJ;
                        let BIL = BHQ * BIJ;
                        let BIM = BHR * BIJ;
                        BIN = BIK;
                        BIP = BIL;
                        BIR = BIM;
                    } else {
                        BIN = BHP;
                        BIP = BHQ;
                        BIR = BHR;
                    }
                    let BIO = BDB + BIN;
                    let BIQ = BDC + BIP;
                    let BIS = BCH + BIR;
                    let BIT = if BIC < (B * BID) { 1.0 } else { 0.0 };
                    let BJA = if BIT != 0.0 {
                        F
                    } else {
                        BFR
                    };
                    BIU = BCF;
                    BIW = BIS;
                    BIX = BIO;
                    BIY = BIQ;
                    BIZ = BJA;
                    BJD = BJB;
                }
                let BIV = BIU + F;
                BCF = BIV;
                BCH = BIW;
                BDB = BIX;
                BDC = BIY;
                BFR = BIZ;
                BJB = BJD;
                BJL = BCX;
                BJN = BFT;
                BJR = BFX;
                BKA = BFV;
                BLW = BCV;
                BMO = BFZ;
            }
            let BJE = if BJB > A { 1.0 } else { 0.0 };
            let BJF = if BJE != 0.0 {
                BJB
            } else {
                BCF
            };
            let BJG = if BJF > D { 1.0 } else { 0.0 };
            let BJH;
            let BJK;
            let BNF;
            if BJG != 0.0 {
                BJH = BCB;
                BJK = BCC;
                BNF = BCD;
            } else {
                BJH = BDB;
                BJK = BJL;
                BNF = BCH;
            }
            if BJG != 0.0 {
            } else {
            }
            let BJI = BJH - AVC;
            let BJJ = if (if AXA <= -1e0f64 { 1.0 } else { 0.0 }) != 0.0 || (if AVC < A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let BKO = if BJJ != 0.0 {
                F
            } else {
                BKP
            };
            let BJM = BJK - AVZ;
            let BJO = BJN - AVD;
            let BJP = BJN + AVD;
            let BJQ = BJO - (((JC * BJP) * BJI) * G);
            let BJS = BJR + AVG;
            let BJT = (BJR - AVG) - (((JC * BJS) * BJM) * G);
            let BJU = if OA == A { 1.0 } else { 0.0 };
            let BJV = if (if BJQ < A { 1.0 } else { 0.0 }) != 0.0 || BJU != 0.0 { 1.0 } else { 0.0 };
            let BJX = if BJV != 0.0 {
                A
            } else {
                BJQ
            };
            let BJW = if (if BJT < A { 1.0 } else { 0.0 }) != 0.0 || BJU != 0.0 { 1.0 } else { 0.0 };
            let BJY = if BJW != 0.0 {
                A
            } else {
                BJT
            };
            let BJZ = BJX + BJY;
            let BKC = BKA + BKB;
            let BKD = -5e-1f64 * BKC;
            let BKE = BJI + B;
            let BKF = -BJO;
            let BKH = if (-BKF) < BKG { 1.0 } else { 0.0 };
            let BKI = if BKH != 0.0 {
                A
            } else {
                BKF
            };
            let BKJ = JC * SF;
            let BKK = F - (((F + ((AR * (-BKI)) / ((BKJ * BKE) * BKE))) * BKE) / AVK);
            let BKL = if BKK <= A { 1.0 } else { 0.0 };
            let CON = if BKL != 0.0 {
                A
            } else {
                BKK
            };
            let BKM = -5e-1f64 * BJP;
            let BKN = -5e-1f64 * BJS;
            let BKQ = if BKO == A { 1.0 } else { 0.0 };
            let BLO;
            let COB;
            if BKQ != 0.0 {
                let BKT = if (if BKR < 2.220446049250313e-15f64 { 1.0 } else { 0.0 }) != 0.0 && (if BKS < 2.220446049250313e-15f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let BLM;
                let COC;
                if BKT != 0.0 {
                    let BKU = AVC + OK;
                    let BKV = if BJH > (BKU - 2.220446049250313e-15f64) { 1.0 } else { 0.0 };
                    let COD = if BKV != 0.0 {
                        let BKW = BKU - 2.220446049250313e-15f64;
                        BKW
                    } else {
                        BJH
                    };
                    BLM = A;
                    COC = COD;
                } else {
                    let BKX = GH / ((BKR * GG) + ((BKS * AVJ) / AN));
                    let BKZ = (BKY * (OA + AVC)) + ((F - BKY) * BJH);
                    let BLA = AVC + OK;
                    let BLB = if BKZ > (BLA - 2.220446049250313e-15f64) { 1.0 } else { 0.0 };
                    let BLD = if BLB != 0.0 {
                        let BLC = BLA - 2.220446049250313e-15f64;
                        BLC
                    } else {
                        BKZ
                    };
                    let BLE = BLD - BJH;
                    let BLF = (G * (BLE + (((BLE * BLE) + 4e-6f64).sqrt()))) + 1e-13f64;
                    let BLG = if BLF < A { 1.0 } else { 0.0 };
                    let BLI = if BLG != 0.0 {
                        A
                    } else {
                        BLF
                    };
                    let BLJ = (AR * (GG / GH)) * BLI;
                    let BLK = ((((AR * (BJZ / (JC * AVJ))) + (BLJ * BKX)) + (BLH * BKX)) / LE) * BKX;
                    let BLL = PC * (G * ((-BLK) + (((BLK * BLK) + (((AX * (BLJ + BLH)) * BKX) * BKX)).sqrt())));
                    BLM = BLL;
                    COC = BLD;
                }
                let BLN = BLM * HA;
                BLO = BLN;
                COB = COC;
            } else {
                BLO = A;
                COB = A;
            }
            let BLP = LE - BLO;
            let BLQ = if BLP < IG { 1.0 } else { 0.0 };
            let BOE = if BLQ != 0.0 {
                IG
            } else {
                BLP
            };
            let BLR = -DF;
            let BLS = BLR * LE;
            let BLT = BLS * (BKM + BKN);
            let BMA = ((G * (BLU + BLW)) * LE) * DF;
            let BMB = OA - BJI;
            let BMD = (AR * (BMB / AR)) / BMC;
            let BME = BMC / (F + (BMD * (5e-1f64 + (BMD * (1.6666666666666666e-1f64 + (BMD * (4.1666666666666664e-2f64 + (BMD * (8.333333333333333e-3f64 + (BMD * (1.388888888888889e-3f64 + (BMD * 1.984126984126984e-4f64))))))))))));
            let BMF = if BME < 2.220446049250313e-15f64 { 1.0 } else { 0.0 };
            let BMH = if BMF != 0.0 {
                BMG
            } else {
                BME
            };
            let BMI = AVC + BMH;
            let BMK = BJN / CH;
            let BML = BJR / CH;
            let BMN = BMM / CH;
            let BMP = BMO / CH;
            let BMQ = BKM / CH;
            let BMR = BKN / CH;
            let BMS = BKD / CH;
            let BMT = MH * KO;
            let BMU = (parameters[81] * (F + (parameters[82] / (CR.powf(parameters[83]))))) / BMJ;
            let BMV = (parameters[78] * (F + (parameters[79] / (CR.powf(parameters[80]))))) / BMJ;
            let BMW = (G * (BJI + (((BJI * BJI) + 4e-12f64).sqrt()))) + 1e-16f64;
            let BMX = if BMW < A { 1.0 } else { 0.0 };
            let BMY = if BMX != 0.0 {
                A
            } else {
                BMW
            };
            let BMZ = OE.sqrt();
            let BNC = F + (parameters[300] / (CR.powf(parameters[301])));
            let BND = ((BMV * BMS) + (BMU * (BMQ - ((parameters[299] * BNC) * BML)))) / (F + ((((((BMY * BMY) + OE).sqrt()) - BMZ).powf(BNA)) * BNB));
            let BNK;
            let BNQ;
            let BNR;
            if EH != 0.0 {
                let BNE = (AVZ + BJK) * G;
                let BNG = (AWC + BNF) * G;
                let BNJ = BND + ((BNH * ((BNE - BNG) - TO)) / (BNI * BMT));
                BNK = BNJ;
                BNQ = BNE;
                BNR = BNG;
            } else {
                BNK = BND;
                BNQ = A;
                BNR = A;
            }
            let BNL = (G * (BNK + (((BNK * BNK) + 3.6e7f64).sqrt()))) + 3e-7f64;
            let BNM = if BNL < A { 1.0 } else { 0.0 };
            let BNN = if BNM != 0.0 {
                A
            } else {
                BNL
            };
            let BNP = (F / (((F / (DT + ((DU * (BMQ / GE)) / BNO))) + (JR * (BNN.powf(parameters[94])))) + ((BNN.powf(DX)) / parameters[105]))) * BZ;
            let BNX;
            if EH != 0.0 {
                let BNS = (BNH * (BNQ - BNR)) / (BNI * BMT);
                BNX = BNS;
            } else {
                let BNT = (G * (BJM + (((BJM * BJM) + 4e-12f64).sqrt()))) + 1e-16f64;
                let BNU = if BNT < A { 1.0 } else { 0.0 };
                let BNV = if BNU != 0.0 {
                    A
                } else {
                    BNT
                };
                let BNW = ((BMV * (-5e-1f64 * (BMP + BMN))) + (BMU * (BMR - ((parameters[302] * BNC) * BMK)))) / (F + ((((((BNV * BNV) + OE).sqrt()) - BMZ).powf(BNA)) * BNB));
                BNX = BNW;
            }
            let BNY = (G * (BNX + (((BNX * BNX) + 3.6e3f64).sqrt()))) + 3e-9f64;
            let BNZ = if BNY < A { 1.0 } else { 0.0 };
            let BOA = if BNZ != 0.0 {
                A
            } else {
                BNY
            };
            let BOB = (F / (((F / (DV + ((DW * (BMR / GE)) / BNO))) + (JT * (BOA.powf(parameters[275])))) + ((BOA.powf(DY)) / parameters[284]))) * BZ;
            let BOC = AJS * KE;
            let BOD = BOC / BNP;
            let BOF = BJX / ((JC * (AVJ + BQ)) * BOE);
            let BOG = ((BOF * BOF) + (BOD * BOD)).sqrt();
            let BOH = (BNP * BOG) / KE;
            let BOJ = if (if 9.999999999999978e-1f64 <= BOI { 1.0 } else { 0.0 }) != 0.0 && (if BOI <= 1.0000000000000022e0f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let BOM;
            if BOJ != 0.0 {
                BOM = F;
            } else {
                let BOK = if (if 1.9999999999999978e0f64 <= BOI { 1.0 } else { 0.0 }) != 0.0 && (if BOI <= 2.000000000000002e0f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let BON = if BOK != 0.0 {
                    BOH
                } else {
                    let BOL = BOH.powf((BOI - F));
                    BOL
                };
                BOM = BON;
            }
            let BOO = F + (BOH * BOM);
            let BOP = if (if 9.999999999999978e-1f64 <= BOI { 1.0 } else { 0.0 }) != 0.0 && (if BOI <= 1.0000000000000022e0f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let BOU;
            if BOP != 0.0 {
                let BOQ = F / BOO;
                BOU = BOQ;
            } else {
                let BOR = if (if 1.9999999999999978e0f64 <= BOI { 1.0 } else { 0.0 }) != 0.0 && (if BOI <= 2.000000000000002e0f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let BOV = if BOR != 0.0 {
                    let BOS = F / (BOO.sqrt());
                    BOS
                } else {
                    let BOT = BOO * (BOO.powf(((-1e0f64 / BOI) - F)));
                    BOT
                };
                BOU = BOV;
            }
            let BOW = BNP * BOU;
            let BOX = BOC / BOB;
            let BOZ = BJY / ((JC * (BOY + BQ)) * BOE);
            let BPA = (BOB * (((BOZ * BOZ) + (BOX * BOX)).sqrt())) / KE;
            let BPB = if (if 9.999999999999978e-1f64 <= BOI { 1.0 } else { 0.0 }) != 0.0 && (if BOI <= 1.0000000000000022e0f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let BPE;
            if BPB != 0.0 {
                BPE = F;
            } else {
                let BPC = if (if 1.9999999999999978e0f64 <= BOI { 1.0 } else { 0.0 }) != 0.0 && (if BOI <= 2.000000000000002e0f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let BPF = if BPC != 0.0 {
                    BPA
                } else {
                    let BPD = BPA.powf((BOI - F));
                    BPD
                };
                BPE = BPF;
            }
            let BPG = F + (BPA * BPE);
            let BPH = if (if 9.999999999999978e-1f64 <= BOI { 1.0 } else { 0.0 }) != 0.0 && (if BOI <= 1.0000000000000022e0f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let BPM;
            if BPH != 0.0 {
                let BPI = F / BPG;
                BPM = BPI;
            } else {
                let BPJ = if (if 1.9999999999999978e0f64 <= BOI { 1.0 } else { 0.0 }) != 0.0 && (if BOI <= 2.000000000000002e0f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let BPN = if BPJ != 0.0 {
                    let BPK = F / (BPG.sqrt());
                    BPK
                } else {
                    let BPL = BPG * (BPG.powf(((-1e0f64 / BOI) - F)));
                    BPL
                };
                BPM = BPN;
            }
            let BPO = BOB * BPM;
            let BPP = (DE * JE) / BLP;
            let BPQ = ((BPP * BJX) * BOW) + ((BPP * BJY) * BPO);
            let BQD;
            if M != 0.0 {
                let BPR = (AR * (G * BMB)) / BS;
                let BPT = AVC + (BS / (F + (BPR * (5e-1f64 + (BPR * (1.6666666666666666e-1f64 + (BPR * (4.1666666666666664e-2f64 + (BPR * (8.333333333333333e-3f64 + (BPR * (1.388888888888889e-3f64 + (BPR * 1.984126984126984e-4f64)))))))))))));
                let BPU = BPS - BPT;
                let BPV = (G * (BPU + (((BPU * BPU) + 1.0000000000000002e-2f64).sqrt()))) + 5.0000000000000005e-12f64;
                let BPW = if BPV < A { 1.0 } else { 0.0 };
                let BPX = if BPW != 0.0 {
                    A
                } else {
                    BPV
                };
                let BQA = ((BKJ * GM) * (BPX.powf(BPY))) * ((F + (OK * BPZ)) + ((OK * GN) * (BPT - OJ)));
                BQD = BQA;
            } else {
                BQD = A;
            }
            let BQB = if GO != A { 1.0 } else { 0.0 };
            let BQE = if BQB != 0.0 {
                let BQC = (BKJ * GP) * OK;
                BQC
            } else {
                A
            };
            let BQF = BQD + BQE;
            let BQG = if BQF > A { 1.0 } else { 0.0 };
            let BQV;
            let BRU;
            let BRX;
            if BQG != 0.0 {
                let BQH = (BPP * (BJI * BQF)) * BOW;
                let BQI = F / (F + (((-parameters[245]) * TO).exp()));
                let BQJ = (F - BQI) * BQH;
                BQV = BQH;
                BRU = BQI;
                BRX = BQJ;
            } else {
                BQV = A;
                BRU = A;
                BRX = A;
            }
            let BQR;
            if M != 0.0 {
                let BQK = (AR * (G * (OA - BJM))) / BS;
                let BQL = AVZ + (BS / (F + (BQK * (5e-1f64 + (BQK * (1.6666666666666666e-1f64 + (BQK * (4.1666666666666664e-2f64 + (BQK * (8.333333333333333e-3f64 + (BQK * (1.388888888888889e-3f64 + (BQK * 1.984126984126984e-4f64)))))))))))));
                let BQM = BPS - BQL;
                let BQN = (G * (BQM + (((BQM * BQM) + 1.0000000000000002e-2f64).sqrt()))) + 5.0000000000000005e-12f64;
                let BQO = if BQN < A { 1.0 } else { 0.0 };
                let BQP = if BQO != 0.0 {
                    A
                } else {
                    BQN
                };
                let BQQ = ((BKJ * GM) * (BQP.powf(BPY))) * ((F + (OK * BPZ)) + ((OK * GN) * (BQL - OJ)));
                BQR = BQQ;
            } else {
                BQR = A;
            }
            let BQS = BQR + BQE;
            let BQT = if BQS > A { 1.0 } else { 0.0 };
            let BRY;
            if BQT != 0.0 {
                let BQU = (BPP * (BJM * BQS)) * BPO;
                let BQW = BQV * OT;
                let BQX = BQV - BQW;
                let BQY = if (if BQU > BQX { 1.0 } else { 0.0 }) != 0.0 && (if BQW >= A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let BRV;
                if BQY != 0.0 {
                    let BQZ = (BQU - BQV) + BQW;
                    let BRA = BQZ * BQZ;
                    let BRB = BQW * BQW;
                    let BRC = (BRA * BRA) + (BRB * BRB);
                    let BRS;
                    if BRD != 0.0 {
                        let BRN;
                        if BRE != 0.0 {
                            BRN = F;
                        } else {
                            let BRO;
                            if BRF != 0.0 {
                                BRO = AR;
                            } else {
                                let BRP;
                                if BRG != 0.0 {
                                    BRP = BD;
                                } else {
                                    let BRQ = if BRH != 0.0 {
                                        AX
                                    } else {
                                        A
                                    };
                                    BRP = BRQ;
                                }
                                BRO = BRP;
                            }
                            BRN = BRO;
                        }
                        let mut BRI = 0.0;
                        let mut BRK = 0.0;
                        BRI = A;
                        BRK = BRC;
                        loop {
                            let BRJ = if BRI < BRN { 1.0 } else { 0.0 };
                            if BRJ == 0.0 {
                                break;
                            }
                            let BRL = BRK.sqrt();
                            let BRM = BRI + F;
                            BRI = BRM;
                            BRK = BRL;
                        }
                        BRS = BRK;
                    } else {
                        let BRR = BRC.powf(2.5e-1f64);
                        BRS = BRR;
                    }
                    let BRT = BQX + ((BQZ * BQW) * (F / (BRS + BQ)));
                    BRV = BRT;
                } else {
                    BRV = BQU;
                }
                let BRW = BRU * BRV;
                BRY = BRW;
            } else {
                BRY = A;
            }
            let BRZ = BPQ + (BRX + BRY);
            let BSA = if parameters[22] != A { 1.0 } else { 0.0 };
            let BVK;
            if BSA != 0.0 {
                let BSB = CV - RV;
                let BSC = (((((AR * RU) * RS) * HO) * (F / (BSB * BSB))) * QT) * (parameters[158] + (parameters[159] * OK));
                let BSE = ((OL - LT) + (BSD - (parameters[161] * OA))) + BSC;
                let BSF = (KJ * QO) * QO;
                let BSG = (BSF * JC) * G;
                let BSH = (BSG * JC) * AR;
                let BSI = ((((JE - (BSF * (JC * RA))) + LT) - BSD) - BSC) + BQ;
                let BSJ = (OL - BSI) - RC;
                let BSK = if BSI >= A { 1.0 } else { 0.0 };
                let BSM = if BSK != 0.0 {
                    F
                } else {
                    BSL
                };
                let BSN = F + (((JC * (((((BSI + (G * (BSJ + (((BSJ * BSJ) + (((BSM * AX) * BSI) * RC)).sqrt())))) - LT) + BSD) + BSC) - OJ)) - F) * (AX / BSH));
                let BSO = (G * (BSN + (((BSN * BSN) + 4e-4f64).sqrt()))) + 1e-12f64;
                let BSP = if BSO < A { 1.0 } else { 0.0 };
                let BSQ = if BSP != 0.0 {
                    A
                } else {
                    BSO
                };
                let BSR = BSE + (BSG * (F - ((BSQ + BQ).sqrt())));
                let BST = ((((F / BSS) / BSF) * (BSE * BSE)).ln()) * (F / (JC + (AR / (BSE + BQ))));
                let BSU = (BST - BSR) - AJV;
                let BSV = (BSU * BSU) + ((AX * AJV) * BST);
                let BSW = (G * (BSV + (((BSV * BSV) + 4e-12f64).sqrt()))) + 1e-16f64;
                let BSX = if BSW < A { 1.0 } else { 0.0 };
                let BSY = if BSX != 0.0 {
                    A
                } else {
                    BSW
                };
                let BSZ = BST - (G * (BSU + (BSY.sqrt())));
                let BTA = (JC * (BSZ - OJ)) - F;
                let BTB = BTA + (BSS * ((JC * BSZ).exp()));
                let BTC = (G * (BTB + (((BTB * BTB) + 4e-4f64).sqrt()))) + 1e-12f64;
                let BTD = if BTC < A { 1.0 } else { 0.0 };
                let BTE = if BTD != 0.0 {
                    A
                } else {
                    BTC
                };
                let BTF = (BTE + 2.220446049250313e-15f64).sqrt();
                let BTG = (G * (BTA + (((BTA * BTA) + 4e-4f64).sqrt()))) + 1e-12f64;
                let BTH = if BTG < A { 1.0 } else { 0.0 };
                let BTI = if BTH != 0.0 {
                    A
                } else {
                    BTG
                };
                let BTK = BTJ * (BTF - ((BTI + 2.220446049250313e-15f64).sqrt()));
                let BTL = BSR - BSZ;
                let BTM = (G * (BTL + (((BTL * BTL) + 4.000000000000001e-2f64).sqrt()))) + 1.0000000000000001e-11f64;
                let BTN = if BTM < A { 1.0 } else { 0.0 };
                let BTO = if BTN != 0.0 {
                    A
                } else {
                    BTM
                };
                let BTP = OA / (BTO + 2.220446049250313e-15f64);
                let BTQ = BTP * BTP;
                let BTR = (((BTQ * BTQ) * BTQ) * BTQ) + 1e0f64;
                let BUH;
                if BTS != 0.0 {
                    let BUC;
                    if BTT != 0.0 {
                        BUC = F;
                    } else {
                        let BUD;
                        if BTU != 0.0 {
                            BUD = AR;
                        } else {
                            let BUE;
                            if BTV != 0.0 {
                                BUE = BD;
                            } else {
                                let BUF = if BTW != 0.0 {
                                    AX
                                } else {
                                    A
                                };
                                BUE = BUF;
                            }
                            BUD = BUE;
                        }
                        BUC = BUD;
                    }
                    let mut BTX = 0.0;
                    let mut BTZ = 0.0;
                    BTX = A;
                    BTZ = BTR;
                    loop {
                        let BTY = if BTX < BUC { 1.0 } else { 0.0 };
                        if BTY == 0.0 {
                            break;
                        }
                        let BUA = BTZ.sqrt();
                        let BUB = BTX + F;
                        BTX = BUB;
                        BTZ = BUA;
                    }
                    BUH = BTZ;
                } else {
                    let BUG = BTR.powf(1.25e-1f64);
                    BUH = BUG;
                }
                let BUI = BRZ + ((((((DA * CO) * JE) * BOW) * BTK) * (BTP * (F / (BUH + BQ)))) / BOE);
                BVK = BUI;
            } else {
                BVK = BRZ;
            }
            let BUJ = if parameters[23] != A { 1.0 } else { 0.0 };
            let BUK = if (if parameters[20] != A { 1.0 } else { 0.0 }) != 0.0 && BUJ != 0.0 { 1.0 } else { 0.0 };
            let COR;
            let COU;
            let COX;
            let CPD;
            if BUK != 0.0 {
                let BUL = AVK * AVK;
                let BUM = BUL - ((LU * QO) * BJZ);
                let BUN = (G * (BUL + (((BUL * BUL) + 4e-6f64).sqrt()))) + 1e-13f64;
                let BUO = if BUN < A { 1.0 } else { 0.0 };
                let BUR = if BUO != 0.0 {
                    A
                } else {
                    BUN
                };
                let BUP = (G * (BUM + (((BUM * BUM) + 4e-6f64).sqrt()))) + 1e-13f64;
                let BUQ = if BUP < A { 1.0 } else { 0.0 };
                let BUS = if BUQ != 0.0 {
                    A
                } else {
                    BUP
                };
                let BUT = BUR - BUS;
                let BUU = if (if AVJ < 2.220446049250313e-15f64 { 1.0 } else { 0.0 }) != 0.0 || (if BUT < 2.220446049250313e-15f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let COS = if BUU != 0.0 {
                    A
                } else {
                    F
                };
                COR = COS;
                COU = BUS;
                COX = BUR;
                CPD = BUT;
            } else {
                COR = A;
                COU = A;
                COX = A;
                CPD = A;
            }
            let BUX = if BUV > A { 1.0 } else { 0.0 };
            let BVM;
            if BUX != 0.0 {
                let BUZ = F + (((AR / GI) * AVL) * ((BUY - JE) - (HG * OJ)));
                let BVA = (G * (BUZ + (((BUZ * BUZ) + 4e-6f64).sqrt()))) + 1e-13f64;
                let BVB = if BVA < A { 1.0 } else { 0.0 };
                let BVC = if BVB != 0.0 {
                    A
                } else {
                    BVA
                };
                let BVF = ((AMK * OK) + BVD) - ((HH * HF) * ((BUY * HN) + (AVM * (F - ((BVC + BQ).sqrt())))));
                let BVG = (G * (BVF + (((BVF * BVF) + 4e-4f64).sqrt()))) + 1e-12f64;
                let BVH = if BVG < A { 1.0 } else { 0.0 };
                let BVI = if BVH != 0.0 {
                    A
                } else {
                    BVG
                };
                let BVJ = BVI + BQ;
                let BVL = ((HI * BVJ) * BVK) * (((-HJ) / BVJ).exp());
                BVM = BVL;
            } else {
                BVM = BUV;
            }
            let BVN = if (if BKQ != 0.0 && (if BVM > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if parameters[145] != A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            if BVN != 0.0 {
                let BVO = (JC * AVC) - F;
                let BVP = if ((G * (BVO + (((BVO * BVO) + 4.000000000000001e-2f64).sqrt()))) + 1.0000000000000001e-11f64) < A { 1.0 } else { 0.0 };
                if BVP != 0.0 {
                } else {
                }
                let BVQ = (JC * BJH) - F;
                let BVR = if ((G * (BVQ + (((BVQ * BVQ) + 4.000000000000001e-2f64).sqrt()))) + 1.0000000000000001e-11f64) < A { 1.0 } else { 0.0 };
                if BVR != 0.0 {
                } else {
                }
            } else {
            }
            let BVS = MD * KO;
            let BVT = SF / CH;
            let BVU = LE * KO;
            let BVV = DE * KO;
            let BVW = BOG / KO;
            let BVX = LX / CH;
            let BVY = if O == A { 1.0 } else { 0.0 };
            let CQS;
            let CQW;
            let CQX;
            let CRA;
            let CRD;
            if BVY != 0.0 {
                CQS = A;
                CQW = A;
                CQX = A;
                CRA = A;
                CRD = A;
            } else {
                let CQY;
                if BKQ != 0.0 {
                    let BVZ = ((F + (BVW / P)) * (((OL - (parameters[256] * LT)) + ((((-parameters[258]) * OC) + (parameters[206] * (SJ - TF))) / BVU)) - (((BMI + OK) - 2.220446049250313e-15f64) * parameters[205]))) / BVS;
                    let BWA = (G * (BVZ + (((BVZ * BVZ) + 4e-4f64).sqrt()))) + 1e-12f64;
                    let BWB = if BWA < A { 1.0 } else { 0.0 };
                    let BWG = if BWB != 0.0 {
                        A
                    } else {
                        BWA
                    };
                    let BWC = (G * (OL + (((OL * OL) + 4e-6f64).sqrt()))) + 1e-13f64;
                    let BWD = if BWC < A { 1.0 } else { 0.0 };
                    let BWE = if BWD != 0.0 {
                        A
                    } else {
                        BWC
                    };
                    let BWF = (BWE - OE) / AS;
                    let BWH = BWG * (F - (F / (F + (BWF * BWF))));
                    let BWI = BVU * BVV;
                    let BWK = BWJ / (BWJ + BWI);
                    let BWM = BWL / (BWL + OK);
                    let BWN = ((-parameters[204]) * KG) * (F / ((BWH * BWH) + BQ));
                    let BWO = if BWN < -3.4e1f64 { 1.0 } else { 0.0 };
                    let CQZ = if BWO != 0.0 {
                        A
                    } else {
                        let BWP = (BWK * BWM) * (((((BWN.exp()) * (((parameters[203] / KF) * GE) * BWI)) * (((BMQ + (BVT * B)) / BVX).powf(parameters[257]))) * BWH) * BWH);
                        BWP
                    };
                    CQY = CQZ;
                } else {
                    CQY = A;
                }
                let BWQ = -parameters[211];
                let BWT = BWS * OB;
                let BWU = (F / BVS) / BVS;
                let BWV = ((parameters[210] / CQ) * BVV) * (CR.powf(parameters[259]));
                let BWW = (BWV * ((BVS * ((BWQ * OB) + BWR)).exp())) * ((BWT * BWT) * BWU);
                let BWX = if BWT >= A { 1.0 } else { 0.0 };
                let CRE = if BWX != 0.0 {
                    let BWY = BWW * -1e0f64;
                    BWY
                } else {
                    BWW
                };
                let BWZ = OB - OA;
                let BXA = BWS * BWZ;
                let BXB = (BWV * ((BVS * ((BWQ * BWZ) + BWR)).exp())) * ((BXA * BXA) * BWU);
                let BXC = if BXA >= A { 1.0 } else { 0.0 };
                let CRB = if BXC != 0.0 {
                    let BXD = BXB * -1e0f64;
                    BXD
                } else {
                    BXB
                };
                let BXE = -OB;
                let BXF = (((BXE + (parameters[261] * OC)) + LT) + parameters[215]) / BVS;
                let BXG = (G * (BXF + (((BXF * BXF) + 4e-4f64).sqrt()))) + 1e-12f64;
                let BXH = if BXG < A { 1.0 } else { 0.0 };
                let BXI = if BXH != 0.0 {
                    A
                } else {
                    BXG
                };
                let BXJ = BXI + BQ;
                let BXK = (-parameters[214]) / (BXJ.powf(parameters[263]));
                let BXL = if BXK < -3.4e1f64 { 1.0 } else { 0.0 };
                let CQT;
                if BXL != 0.0 {
                    CQT = A;
                } else {
                    let BXM = BXK.exp();
                    let BXN = CR + parameters[264];
                    let BXP = BXN * DZ;
                    let BXQ = (BXN - BXO) - BXP;
                    let BXR = (AX * BXO) * BXP;
                    let BXS = if BXR > A { 1.0 } else { 0.0 };
                    let BXU = if BXS != 0.0 {
                        BXR
                    } else {
                        let BXT = -BXR;
                        BXT
                    };
                    let BXV = (((((BXO + (G * (BXQ + (((BXQ * BXQ) + BXU).sqrt())))) * parameters[213]) / CQ) * BVV) * (BXJ.powf(parameters[262]))) * BXM;
                    let BXW = (((BXE + (parameters[269] * OC)) + LT) + parameters[268]) / BVS;
                    let BXX = (G * (BXW + (((BXW * BXW) + 4e-4f64).sqrt()))) + 1e-12f64;
                    let BXY = if BXX < A { 1.0 } else { 0.0 };
                    let BXZ = if BXY != 0.0 {
                        A
                    } else {
                        BXX
                    };
                    let BYA = BXZ + BQ;
                    let BYB = (-parameters[267]) / (BYA.powf(parameters[271]));
                    let BYC = if BYB < -3.4e1f64 { 1.0 } else { 0.0 };
                    let BYQ;
                    if BYC != 0.0 {
                        BYQ = A;
                    } else {
                        let BYD = BYB.exp();
                        let BYE = CR + parameters[272];
                        let BYG = BYE * DZ;
                        let BYH = (BYE - BYF) - BYG;
                        let BYI = (AX * BYF) * BYG;
                        let BYJ = if BYI > A { 1.0 } else { 0.0 };
                        let BYL = if BYJ != 0.0 {
                            BYI
                        } else {
                            let BYK = -BYI;
                            BYK
                        };
                        let BYM = (((((BYF + (G * (BYH + (((BYH * BYH) + BYL).sqrt())))) * parameters[266]) / CQ) * BVV) * (BYA.powf(parameters[270]))) * BYD;
                        BYQ = BYM;
                    }
                    let BYN = -BXV;
                    let BYO = BYN * DZ;
                    let BYP = if BYO < BQ { 1.0 } else { 0.0 };
                    let BYS = if BYP != 0.0 {
                        BQ
                    } else {
                        BYO
                    };
                    let BYR = -BYQ;
                    let BYT = (BYN - BYR) - BYS;
                    let BYU = (AX * BYR) * BYS;
                    let BYV = if BYU > A { 1.0 } else { 0.0 };
                    let BYX = if BYV != 0.0 {
                        BYU
                    } else {
                        let BYW = -BYU;
                        BYW
                    };
                    let BYY = -(BYR + (G * (BYT + (((BYT * BYT) + BYX).sqrt()))));
                    CQT = BYY;
                }
                CQS = CQT;
                CQW = G;
                CQX = CQY;
                CRA = CRB;
                CRD = CRE;
            }
            let BYZ = if R == A { 1.0 } else { 0.0 };
            if BYZ != 0.0 {
            } else {
                let BZD = (((BZA * (OA + BZB)) - OB) - (SH * BZC)) / S;
                let BZE = (G * (BZD + (((BZD * BZD) + 4e-4f64).sqrt()))) + 1e-12f64;
                let BZF = if BZE < A { 1.0 } else { 0.0 };
                let BZG = if BZF != 0.0 {
                    A
                } else {
                    BZE
                };
                let BZH = if (((-CF) * KG) / (BZG + BQ)) < -3.4e1f64 { 1.0 } else { 0.0 };
                if BZH != 0.0 {
                } else {
                }
            }
            if BYZ != 0.0 {
            } else {
                let BZI = (((BZA * ((-OA) + BZB)) - (OB - OA)) - (SH * BZC)) / S;
                let BZJ = (G * (BZI + (((BZI * BZI) + 4e-4f64).sqrt()))) + 1e-12f64;
                let BZK = if BZJ < A { 1.0 } else { 0.0 };
                let BZL = if BZK != 0.0 {
                    A
                } else {
                    BZJ
                };
                let BZM = if (((-CF) * KG) / (BZL + BQ)) < -3.4e1f64 { 1.0 } else { 0.0 };
                if BZM != 0.0 {
                } else {
                }
            }
            let BZN = if BKO != A { 1.0 } else { 0.0 };
            let CNZ;
            let CPW;
            if BZN != 0.0 {
                let BZO = OA + AVC;
                let BZP = (BKY * BZO) + ((F - BKY) * BJH);
                let BZQ = if BZP > (BZO - 2.220446049250313e-15f64) { 1.0 } else { 0.0 };
                let COA = if BZQ != 0.0 {
                    let BZR = BZO - 2.220446049250313e-15f64;
                    BZR
                } else {
                    BZP
                };
                CNZ = COA;
                CPW = A;
            } else {
                let BZT = if BZS != A { 1.0 } else { 0.0 };
                let CPX;
                if BZT != 0.0 {
                    let BZU = if BJZ > 1e-15f64 { 1.0 } else { 0.0 };
                    let CPY = if BZU != 0.0 {
                        let BZV = ((BJZ * JE) / LE) / AVJ;
                        BZV
                    } else {
                        A
                    };
                    CPX = CPY;
                } else {
                    CPX = A;
                }
                CNZ = COB;
                CPW = CPX;
            }
            let BZW = F / MF;
            let BZY = if BZX > A { 1.0 } else { 0.0 };
            let BZZ = if (if (if parameters[19] >= F { 1.0 } else { 0.0 }) != 0.0 && BZY != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if CD > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let CPI;
            let CPL;
            let CQL;
            let CQO;
            if BZZ != 0.0 {
                let CAA = LX * ((CD / GF).sqrt());
                let CAF = CAD + (CAC * CAE);
                let CAG = CAE + (CAC * CAD);
                let CAW = if CAB != 0.0 {
                    let CAJ = (CAD * CAH) + (CAE * (CAH - CAI));
                    CAJ
                } else {
                    A
                };
                let CAV = if CAC != 0.0 {
                    let CAK = (CAE * CAH) + (CAD * (CAH - CAI));
                    CAK
                } else {
                    CAW
                };
                let CAM = if CAL > MZ { 1.0 } else { 0.0 };
                let CAR = if CAM != 0.0 {
                    let CAN = MV - MZ;
                    let CAO = (CAL - MZ) / CAN;
                    let CAP = CAO * CAO;
                    let CAQ = MZ + (CAN * (F - (F / ((((F + CAO) + CAP) + (CAP * CAO)) + (CAP * CAP)))));
                    CAQ
                } else {
                    CAL
                };
                let CAS = (-CAR) - B;
                let CAT = CAA * BZW;
                let CAU = CAT * CAT;
                let CAY = (-CAV) + CAX;
                let CAZ = UC * ((CD / KH).ln());
                let CBA = -CAS;
                let CBB = if CAY < CBA { 1.0 } else { 0.0 };
                let CGG;
                let CLR;
                let CLY;
                let CMB;
                if CBB != 0.0 {
                    let CBC = MF / (JC * CAA);
                    let CBD = AR + (4.242640687119285e0f64 * CBC);
                    let CBE = ((AY * CBD) * CBD) * CBD;
                    let CBF = JA - CAZ;
                    let CBH = (CBG * CBC) * ((JC * (CAY + CAS)) - AR);
                    let CBI = 9.899494936611664e0f64 - CBH;
                    let CBJ = CBI * CBI;
                    let CBK = if CBE < (CBJ * VC) { 1.0 } else { 0.0 };
                    let CBN = if CBK != 0.0 {
                        let CBL = ((-9.899494936611664e0f64 + CBI) + ((G * CBE) / CBI)) + CBH;
                        CBL
                    } else {
                        let CBM = (-9.899494936611664e0f64 + ((CBE + CBJ).sqrt())) + CBH;
                        CBM
                    };
                    let CBP = CBN.powf(CBO);
                    let CBR = ((((((-5.65685424949238e0f64 - (CBQ * CBC)) + (AR * CBP)) + ((LW * CBP) * CBP)) / CBP) * JE) - CAS) + CAS;
                    let CBS = CBR / CBF;
                    let CBT = MF * (CAY - ((CBR / ((F + (CBS * CBS)).sqrt())) - CAS));
                    CGG = CBT;
                    CLR = A;
                    CLY = A;
                    CMB = A;
                } else {
                    let CBU = CAY + CAS;
                    let CBV = (JC * CBU) - F;
                    let CBW = CAU * JD;
                    let CBX = F + ((AX * (CBV + 4.9787068367863944e-2f64)) / CBW);
                    let CBY = if CBX < 2.220446049250313e-15f64 { 1.0 } else { 0.0 };
                    let CCB = if CBY != 0.0 {
                        CBZ
                    } else {
                        CBX
                    };
                    let CCA = (CAU * JC) / AR;
                    let CCC = F + ((AX * (CBV + ((-(JC * ((CAY + (CCA * (F - (CCB.sqrt())))) + CAS))).exp()))) / CBW);
                    let CCD = if CCC < 2.220446049250313e-15f64 { 1.0 } else { 0.0 };
                    let CCF = if CCD != 0.0 {
                        CCE
                    } else {
                        CCC
                    };
                    let CCG = JC * ((CAY + (CCA * (F - (CCF.sqrt())))) + CAS);
                    let CCH = if CCG < BD { 1.0 } else { 0.0 };
                    let CDC = if CCH != 0.0 {
                        let CCI = 7.071067811865476e-1f64 + (F / (JC * CAT));
                        let CCJ = (-5.151950988020902e1f64 - ((-1.047839336957922e-1f64 * CCI) / 5.286687693921294e-4f64)) + (((-CBU) / CAT) / 1.8773541122053122e-2f64);
                        let CCK = ((2.8160311683079683e-2f64 * CCI) - 1.0979672760764175e-2f64) / 7.930031540881942e-4f64;
                        let CCL = ((CCJ * CCJ) + ((CCK * CCK) * CCK)).sqrt();
                        let CCM = JC * ((((((((-CCJ) + CCL).powf(CBO)) + (-((CCJ + CCL).powf(CBO)))) - -3.7209791878387604e0f64) * JE) - CAS) + CAS);
                        CCM
                    } else {
                        CCG
                    };
                    let CCO = if CCN > A { 1.0 } else { 0.0 };
                    let CDJ;
                    if CCO != 0.0 {
                        let CCP = KH / CD;
                        let CCQ = CCP * CCP;
                        let CCR = JC * (CBU + AS);
                        let CCS = (CCQ * (((JC * CBA).exp()) + BQ)) * CBW;
                        let CCT = (CCQ * CBW).ln();
                        let CCU = JC * CAS;
                        let CCV = (CCR - ((((CCS + (CCR * CCR)).ln()) - CCT) + CCU)) - F;
                        let CCW = AX * CCR;
                        let CCX = if CCW > A { 1.0 } else { 0.0 };
                        let CCZ = if CCX != 0.0 {
                            CCW
                        } else {
                            let CCY = -CCW;
                            CCY
                        };
                        let CDA = (CCR - (CCR - (G * (CCV + (((CCV * CCV) + CCZ).sqrt()))))) + (JC * AS);
                        let CDB = (((CCS + (CDA * CDA)).ln()) - CCT) + CCU;
                        let CDD = (CDB - CDC) - 6.0000000000000005e-2f64;
                        let CDE = (AX * CDB) * 6.0000000000000005e-2f64;
                        let CDF = if CDE > A { 1.0 } else { 0.0 };
                        let CDH = if CDF != 0.0 {
                            CDE
                        } else {
                            let CDG = -CDE;
                            CDG
                        };
                        let CDI = CDB - (G * (CDD + (((CDD * CDD) + CDH).sqrt())));
                        CDJ = CDI;
                    } else {
                        CDJ = CDC;
                    }
                    let CDK = (CDJ / JC) - CAS;
                    let CDL = if ((CDJ - F) + ((-CDJ).exp())) < 2.220446049250313e-15f64 { 1.0 } else { 0.0 };
                    if CDL != 0.0 {
                    } else {
                    }
                    let CDM = MF * (CAY - CDK);
                    let CDN = if CCN == F { 1.0 } else { 0.0 };
                    let CGH;
                    let CLS;
                    let CLZ;
                    let CMC;
                    if CDN != 0.0 {
                        let CDO = (JC * CBA).exp();
                        let CDP = KH / CD;
                        let CDQ = CDP * CDP;
                        let CDR = CDQ * CDO;
                        let mut CDS = 0.0;
                        let mut CDU = 0.0;
                        let mut CEY = 0.0;
                        let mut CFU = 0.0;
                        let mut CFX = 0.0;
                        let mut CGC = 0.0;
                        let mut CGD = 0.0;
                        CDS = F;
                        CDU = CDK;
                        CEY = A;
                        CFU = CDJ;
                        CFX = A;
                        CGC = A;
                        CGD = A;
                        loop {
                            let CDT = if CDS <= 4.1e1f64 { 1.0 } else { 0.0 };
                            if CDT == 0.0 {
                                break;
                            }
                            let CDV = JC * (CDU + CAS);
                            let CDW = if CDV < IQ { 1.0 } else { 0.0 };
                            let CEU;
                            let CEW;
                            let CFY;
                            let CGE;
                            if CDW != 0.0 {
                                let CDX = CDV * CDV;
                                let CEA = (CDX * CDV) * (CDY + (CDV * (-7.053654284009761e-2f64 + (CDV * CDZ))));
                                let CEB = CDV * IQ;
                                let CEC = (CDR * CEA) * CEA;
                                let CEG = CDV * (CED + (CDV * (-1.17851130197758e-1f64 + (CDV * (CEE + (CDV * (-1.63730162779191e-3f64 + (CDV * CEF))))))));
                                let CEH = (((CEG * CEG) + CEC) + BQ).sqrt();
                                let CEI = ((((JC * (CED + (CDV * (-2.35702260395516e-1f64 + (CDV * (5.3640151901649905e-2f64 + (CDV * (-6.54920651116764e-3f64 + (CEB * CEF))))))))) * AR) * CEG) + ((((CDR * JC) * AR) * CEA) * (CDX * (8.907946456731299e-1f64 + (CDV * (-2.8214617136039044e-1f64 + (CEB * CDZ))))))) / (CEH + CEH);
                                CEU = CEH;
                                CEW = CEI;
                                CFY = CEG;
                                CGE = CEC;
                            } else {
                                let CEJ = if CDV < AOK { 1.0 } else { 0.0 };
                                let CEQ;
                                let CES;
                                if CEJ != 0.0 {
                                    let CEK = CDV.exp();
                                    let CEL = CDR * (CEK - F);
                                    let CEM = (CDR * JC) * CEK;
                                    CEQ = CEL;
                                    CES = CEM;
                                } else {
                                    let CEN = (JC * CDU).exp();
                                    let CEO = CDQ * (CEN - CDO);
                                    let CEP = (CDQ * JC) * CEN;
                                    CEQ = CEO;
                                    CES = CEP;
                                }
                                let CER = ((CDV - F) + CEQ).sqrt();
                                let CET = ((JC + CES) / CER) * G;
                                CEU = CER;
                                CEW = CET;
                                CFY = CFX;
                                CGE = CEQ;
                            }
                            let CEV = (CAY - CDU) - (CAT * CEU);
                            let CEX = -1e0f64 - (CAT * CEW);
                            let CEZ = if CEY == F { 1.0 } else { 0.0 };
                            let CFO;
                            let CFQ;
                            let CFR;
                            if CEZ != 0.0 {
                                CFO = CFA;
                                CFQ = CDU;
                                CFR = CEY;
                            } else {
                                let CFB = (-CEV) / CEX;
                                let CFC = CDU.abs();
                                let CFD = if F >= CFC { 1.0 } else { 0.0 };
                                let CFE = if CFD != 0.0 {
                                    F
                                } else {
                                    CFC
                                };
                                let CFF = 5e-2f64 * (F + CFE);
                                let CFG = if (CFB.abs()) > CFF { 1.0 } else { 0.0 };
                                let CFL;
                                if CFG != 0.0 {
                                    let CFH = if CFB >= A { 1.0 } else { 0.0 };
                                    let CFJ = if CFH != 0.0 {
                                        F
                                    } else {
                                        CFI
                                    };
                                    let CFK = CFF * CFJ;
                                    CFL = CFK;
                                } else {
                                    CFL = CFB;
                                }
                                let CFM = CDU + CFL;
                                let CFN = if (if (CFL.abs()) <= B { 1.0 } else { 0.0 }) != 0.0 && (if (CEV.abs()) <= VC { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                let CFS = if CFN != 0.0 {
                                    F
                                } else {
                                    CEY
                                };
                                CFO = CDS;
                                CFQ = CFM;
                                CFR = CFS;
                            }
                            let CFP = CFO + F;
                            CDS = CFP;
                            CDU = CFQ;
                            CEY = CFR;
                            CFU = CDV;
                            CFX = CFY;
                            CGC = CEU;
                            CGD = CGE;
                        }
                        let CFT = if CEY == A { 1.0 } else { 0.0 };
                        if CFT != 0.0 {
                        } else {
                        }
                        let CFV = if CFU < IQ { 1.0 } else { 0.0 };
                        let CGB;
                        if CFV != 0.0 {
                            let CFW = if CFU < BD { 1.0 } else { 0.0 };
                            if CFW != 0.0 {
                            } else {
                            }
                            let CFZ = CFX + 2.220446049250313e-15f64;
                            CGB = CFZ;
                        } else {
                            let CGA = (CFU - F).sqrt();
                            CGB = CGA;
                        }
                        let CGF = (CAA * CGB) + ((CAA * CGD) * (F / (CGC + CGB)));
                        CGH = CGF;
                        CLS = CFX;
                        CLZ = CGC;
                        CMC = CGD;
                    } else {
                        CGH = CDM;
                        CLS = A;
                        CLZ = A;
                        CMC = A;
                    }
                    CGG = CGH;
                    CLR = CLS;
                    CLY = CLZ;
                    CMB = CMC;
                }
                let CGI = DF * BZX;
                let CQN = if CAF != 0.0 {
                    let CGJ = CGI * CGG;
                    CGJ
                } else {
                    A
                };
                let CQQ = if CAG != 0.0 {
                    let CGK = CGI * CGG;
                    CGK
                } else {
                    A
                };
                let CGN = (CGL * CAD) + CAE;
                let CGO = (CGL * CAE) + CAD;
                let CHA = if CGL != 0.0 {
                    let CGP = (CAD * CAH) + (CAE * (CAH - CAI));
                    CGP
                } else {
                    CAV
                };
                let CGZ = if CGM != 0.0 {
                    let CGQ = (CAE * CAH) + (CAD * (CAH - CAI));
                    CGQ
                } else {
                    CHA
                };
                let CGS = if CGR > MZ { 1.0 } else { 0.0 };
                let CGX = if CGS != 0.0 {
                    let CGT = MV - MZ;
                    let CGU = (CGR - MZ) / CGT;
                    let CGV = CGU * CGU;
                    let CGW = MZ + (CGT * (F - (F / ((((F + CGU) + CGV) + (CGV * CGU)) + (CGV * CGV)))));
                    CGW
                } else {
                    CGR
                };
                let CGY = (-CGX) - B;
                let CHB = (-CGZ) + CAX;
                let CHC = -CGY;
                let CHD = if CHB < CHC { 1.0 } else { 0.0 };
                let CMF;
                if CHD != 0.0 {
                    let CHE = MF / (JC * CAA);
                    let CHF = AR + (4.242640687119285e0f64 * CHE);
                    let CHG = ((AY * CHF) * CHF) * CHF;
                    let CHH = JA - CAZ;
                    let CHI = (CBG * CHE) * ((JC * (CHB + CGY)) - AR);
                    let CHJ = 9.899494936611664e0f64 - CHI;
                    let CHK = CHJ * CHJ;
                    let CHL = if CHG < (CHK * VC) { 1.0 } else { 0.0 };
                    let CHO = if CHL != 0.0 {
                        let CHM = ((-9.899494936611664e0f64 + CHJ) + ((G * CHG) / CHJ)) + CHI;
                        CHM
                    } else {
                        let CHN = (-9.899494936611664e0f64 + ((CHG + CHK).sqrt())) + CHI;
                        CHN
                    };
                    let CHP = CHO.powf(CBO);
                    let CHQ = ((((((-5.65685424949238e0f64 - (CBQ * CHE)) + (AR * CHP)) + ((LW * CHP) * CHP)) / CHP) * JE) - CGY) + CGY;
                    let CHR = CHQ / CHH;
                    let CHS = MF * (CHB - ((CHQ / ((F + (CHR * CHR)).sqrt())) - CGY));
                    CMF = CHS;
                } else {
                    let CHT = CHB + CGY;
                    let CHU = (JC * CHT) - F;
                    let CHV = CAU * JD;
                    let CHW = F + ((AX * (CHU + 4.9787068367863944e-2f64)) / CHV);
                    let CHX = if CHW < 2.220446049250313e-15f64 { 1.0 } else { 0.0 };
                    let CIA = if CHX != 0.0 {
                        CHY
                    } else {
                        CHW
                    };
                    let CHZ = (CAU * JC) / AR;
                    let CIB = F + ((AX * (CHU + ((-(JC * ((CHB + (CHZ * (F - (CIA.sqrt())))) + CGY))).exp()))) / CHV);
                    let CIC = if CIB < 2.220446049250313e-15f64 { 1.0 } else { 0.0 };
                    let CIE = if CIC != 0.0 {
                        CID
                    } else {
                        CIB
                    };
                    let CIF = JC * ((CHB + (CHZ * (F - (CIE.sqrt())))) + CGY);
                    let CIG = if CIF < BD { 1.0 } else { 0.0 };
                    let CJA = if CIG != 0.0 {
                        let CIH = 7.071067811865476e-1f64 + (F / (JC * CAT));
                        let CII = (-5.151950988020902e1f64 - ((-1.047839336957922e-1f64 * CIH) / 5.286687693921294e-4f64)) + (((-CHT) / CAT) / 1.8773541122053122e-2f64);
                        let CIJ = ((2.8160311683079683e-2f64 * CIH) - 1.0979672760764175e-2f64) / 7.930031540881942e-4f64;
                        let CIK = ((CII * CII) + ((CIJ * CIJ) * CIJ)).sqrt();
                        let CIL = JC * ((((((((-CII) + CIK).powf(CBO)) + (-((CII + CIK).powf(CBO)))) - -3.7209791878387604e0f64) * JE) - CGY) + CGY);
                        CIL
                    } else {
                        CIF
                    };
                    let CIM = if CCN > A { 1.0 } else { 0.0 };
                    let CJH;
                    if CIM != 0.0 {
                        let CIN = KH / CD;
                        let CIO = CIN * CIN;
                        let CIP = JC * (CHT + AS);
                        let CIQ = (CIO * (((JC * CHC).exp()) + BQ)) * CHV;
                        let CIR = (CIO * CHV).ln();
                        let CIS = JC * CGY;
                        let CIT = (CIP - ((((CIQ + (CIP * CIP)).ln()) - CIR) + CIS)) - F;
                        let CIU = AX * CIP;
                        let CIV = if CIU > A { 1.0 } else { 0.0 };
                        let CIX = if CIV != 0.0 {
                            CIU
                        } else {
                            let CIW = -CIU;
                            CIW
                        };
                        let CIY = (CIP - (CIP - (G * (CIT + (((CIT * CIT) + CIX).sqrt()))))) + (JC * AS);
                        let CIZ = (((CIQ + (CIY * CIY)).ln()) - CIR) + CIS;
                        let CJB = (CIZ - CJA) - 6.0000000000000005e-2f64;
                        let CJC = (AX * CIZ) * 6.0000000000000005e-2f64;
                        let CJD = if CJC > A { 1.0 } else { 0.0 };
                        let CJF = if CJD != 0.0 {
                            CJC
                        } else {
                            let CJE = -CJC;
                            CJE
                        };
                        let CJG = CIZ - (G * (CJB + (((CJB * CJB) + CJF).sqrt())));
                        CJH = CJG;
                    } else {
                        CJH = CJA;
                    }
                    let CJI = (CJH / JC) - CGY;
                    let CJJ = if ((CJH - F) + ((-CJH).exp())) < 2.220446049250313e-15f64 { 1.0 } else { 0.0 };
                    if CJJ != 0.0 {
                    } else {
                    }
                    let CJK = MF * (CHB - CJI);
                    let CJL = if CCN == F { 1.0 } else { 0.0 };
                    let CMG;
                    if CJL != 0.0 {
                        let CJM = (JC * CHC).exp();
                        let CJN = KH / CD;
                        let CJO = CJN * CJN;
                        let CJP = CJO * CJM;
                        let mut CJQ = 0.0;
                        let mut CJS = 0.0;
                        let mut CKR = 0.0;
                        let mut CLN = 0.0;
                        let mut CLQ = 0.0;
                        let mut CLX = 0.0;
                        let mut CMA = 0.0;
                        CJQ = F;
                        CJS = CJI;
                        CKR = A;
                        CLN = CJH;
                        CLQ = CLR;
                        CLX = CLY;
                        CMA = CMB;
                        loop {
                            let CJR = if CJQ <= 4.1e1f64 { 1.0 } else { 0.0 };
                            if CJR == 0.0 {
                                break;
                            }
                            let CJT = JC * (CJS + CGY);
                            let CJU = if CJT < IQ { 1.0 } else { 0.0 };
                            let CKN;
                            let CKP;
                            let CLT;
                            let CMD;
                            if CJU != 0.0 {
                                let CJV = CJT * CJT;
                                let CJW = (CJV * CJT) * (CDY + (CJT * (-7.053654284009761e-2f64 + (CJT * CDZ))));
                                let CJX = CJT * IQ;
                                let CJY = (CJP * CJW) * CJW;
                                let CJZ = CJT * (CED + (CJT * (-1.17851130197758e-1f64 + (CJT * (CEE + (CJT * (-1.63730162779191e-3f64 + (CJT * CEF))))))));
                                let CKA = (((CJZ * CJZ) + CJY) + BQ).sqrt();
                                let CKB = ((((JC * (CED + (CJT * (-2.35702260395516e-1f64 + (CJT * (5.3640151901649905e-2f64 + (CJT * (-6.54920651116764e-3f64 + (CJX * CEF))))))))) * AR) * CJZ) + ((((CJP * JC) * AR) * CJW) * (CJV * (8.907946456731299e-1f64 + (CJT * (-2.8214617136039044e-1f64 + (CJX * CDZ))))))) / (CKA + CKA);
                                CKN = CKA;
                                CKP = CKB;
                                CLT = CJZ;
                                CMD = CJY;
                            } else {
                                let CKC = if CJT < AOK { 1.0 } else { 0.0 };
                                let CKJ;
                                let CKL;
                                if CKC != 0.0 {
                                    let CKD = CJT.exp();
                                    let CKE = CJP * (CKD - F);
                                    let CKF = (CJP * JC) * CKD;
                                    CKJ = CKE;
                                    CKL = CKF;
                                } else {
                                    let CKG = (JC * CJS).exp();
                                    let CKH = CJO * (CKG - CJM);
                                    let CKI = (CJO * JC) * CKG;
                                    CKJ = CKH;
                                    CKL = CKI;
                                }
                                let CKK = ((CJT - F) + CKJ).sqrt();
                                let CKM = ((JC + CKL) / CKK) * G;
                                CKN = CKK;
                                CKP = CKM;
                                CLT = CLQ;
                                CMD = CKJ;
                            }
                            let CKO = (CHB - CJS) - (CAT * CKN);
                            let CKQ = -1e0f64 - (CAT * CKP);
                            let CKS = if CKR == F { 1.0 } else { 0.0 };
                            let CLH;
                            let CLJ;
                            let CLK;
                            if CKS != 0.0 {
                                CLH = CKT;
                                CLJ = CJS;
                                CLK = CKR;
                            } else {
                                let CKU = (-CKO) / CKQ;
                                let CKV = CJS.abs();
                                let CKW = if F >= CKV { 1.0 } else { 0.0 };
                                let CKX = if CKW != 0.0 {
                                    F
                                } else {
                                    CKV
                                };
                                let CKY = 5e-2f64 * (F + CKX);
                                let CKZ = if (CKU.abs()) > CKY { 1.0 } else { 0.0 };
                                let CLE;
                                if CKZ != 0.0 {
                                    let CLA = if CKU >= A { 1.0 } else { 0.0 };
                                    let CLC = if CLA != 0.0 {
                                        F
                                    } else {
                                        CLB
                                    };
                                    let CLD = CKY * CLC;
                                    CLE = CLD;
                                } else {
                                    CLE = CKU;
                                }
                                let CLF = CJS + CLE;
                                let CLG = if (if (CLE.abs()) <= B { 1.0 } else { 0.0 }) != 0.0 && (if (CKO.abs()) <= VC { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                let CLL = if CLG != 0.0 {
                                    F
                                } else {
                                    CKR
                                };
                                CLH = CJQ;
                                CLJ = CLF;
                                CLK = CLL;
                            }
                            let CLI = CLH + F;
                            CJQ = CLI;
                            CJS = CLJ;
                            CKR = CLK;
                            CLN = CJT;
                            CLQ = CLT;
                            CLX = CKN;
                            CMA = CMD;
                        }
                        let CLM = if CKR == A { 1.0 } else { 0.0 };
                        if CLM != 0.0 {
                        } else {
                        }
                        let CLO = if CLN < IQ { 1.0 } else { 0.0 };
                        let CLW;
                        if CLO != 0.0 {
                            let CLP = if CLN < BD { 1.0 } else { 0.0 };
                            if CLP != 0.0 {
                            } else {
                            }
                            let CLU = CLQ + 2.220446049250313e-15f64;
                            CLW = CLU;
                        } else {
                            let CLV = (CLN - F).sqrt();
                            CLW = CLV;
                        }
                        let CME = (CAA * CLW) + ((CAA * CMA) * (F / (CLX + CLW)));
                        CMG = CME;
                    } else {
                        CMG = CJK;
                    }
                    CMF = CMG;
                }
                let CQM = if CGN != 0.0 {
                    let CMH = CGI * CMF;
                    CMH
                } else {
                    CQN
                };
                let CQP = if CGO != 0.0 {
                    let CMI = CGI * CMF;
                    CMI
                } else {
                    CQQ
                };
                let CMJ = (CAE * J) + (CAD * I);
                let CPJ = if CMJ != 0.0 {
                    let CMM = (-(((CAE * CMK) + (CAD * CML)) * BLR)) * (OB - OA);
                    CMM
                } else {
                    A
                };
                let CMN = (CAD * J) + (CAE * I);
                let CPM = if CMN != 0.0 {
                    let CMO = (-(((CAD * CMK) + (CAE * CML)) * BLR)) * OB;
                    CMO
                } else {
                    A
                };
                CPI = CPJ;
                CPL = CPM;
                CQL = CQM;
                CQO = CQP;
            } else {
                let CMQ = if CMP == F { 1.0 } else { 0.0 };
                let CMR = if I == 0.0 { 1.0 } else { 0.0 };
                let CMS = if CMP != F { 1.0 } else { 0.0 };
                let CMT = if J == 0.0 { 1.0 } else { 0.0 };
                let CMU = if (if CMQ != 0.0 && CMR != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if CMS != 0.0 && CMT != 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let CMX;
                if CMU != 0.0 {
                    let CMY = if BZY != 0.0 {
                        let CMV = ((-MF) * BZX) * DF;
                        CMV
                    } else {
                        A
                    };
                    CMX = CMY;
                } else {
                    let CMW = ((CAE * CMK) + (CAD * CML)) * BLR;
                    CMX = CMW;
                }
                let CMZ = (-CMX) * (OB - OA);
                let CNA = if (if CMQ != 0.0 && CMT != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if CMS != 0.0 && CMR != 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let CND = if CNA != 0.0 {
                    let CNB = ((-MF) * BZX) * DF;
                    CNB
                } else {
                    let CNC = ((CAD * CMK) + (CAE * CML)) * BLR;
                    CNC
                };
                let CNE = (-CND) * OB;
                CPI = CMZ;
                CPL = CNE;
                CQL = A;
                CQO = A;
            }
            let CVT;
            let CVW;
            if E != 0.0 {
                let CVU = if BKQ != 0.0 {
                    let CNH = (((CNF * CNG) * BOE) * BOE) / ((((BOW * AVK) * CNF) + ((CNG * BOE) * BOE)) + BQ);
                    CNH
                } else {
                    let CNI = CNF + BQ;
                    CNI
                };
                let CNJ = (parameters[225] * SF) / CH;
                CVT = CVU;
                CVW = CNJ;
            } else {
                CVT = A;
                CVW = A;
            }
            let CNK = if BKO == 0.0 { 1.0 } else { 0.0 };
            let CNL = if (if parameters[21] != A { 1.0 } else { 0.0 }) != 0.0 && CNK != 0.0 { 1.0 } else { 0.0 };
            let CRJ;
            if CNL != 0.0 {
                let CNM = AVJ / GE;
                let CNN = (((SF + (AVJ / (AVC - OC))) + CB) * JE) / GE;
                let CNO = ((((-2e0f64 * BLT) / GE) / BOE) / DF) - CNM;
                let CNP = CNO - CNM;
                let CNQ = if (CNP.abs()) > 2.220446049250313e-15f64 { 1.0 } else { 0.0 };
                let CNW = if CNQ != 0.0 {
                    let CNR = CNM + CNN;
                    let CNS = CNO + CNN;
                    let CNT = (((F / CNR) / CNS) + (((((AR * BY) * BOG) * BOW) / CNP) * ((CNS / CNR).ln()))) + (((((BY * BOG) * BOW) * BY) * BOG) * BOW);
                    CNT
                } else {
                    let CNU = CNM + CNN;
                    let CNV = (((F / CNU) / (CNO + CNN)) + ((((AR * BY) * BOG) * BOW) / CNU)) + (((((BY * BOG) * BOW) * BY) * BOG) * BOW);
                    CNV
                };
                let CNX = (((BVK * BVK) * CA) / ((BOE * JC) * DE)) * CNW;
                CRJ = CNX;
            } else {
                CRJ = A;
            }
            let CNY = if BUJ != 0.0 && CNK != 0.0 { 1.0 } else { 0.0 };
            let CPC;
            let CRU;
            if CNY != 0.0 {
                let COE = (BNP * ((CNZ - AVC) / BOE)) / 1e5f64;
                let COF = if (if 9.999999999999978e-1f64 <= BOI { 1.0 } else { 0.0 }) != 0.0 && (if BOI <= 1.0000000000000022e0f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let COI;
                if COF != 0.0 {
                    COI = F;
                } else {
                    let COG = if (if 1.9999999999999978e0f64 <= BOI { 1.0 } else { 0.0 }) != 0.0 && (if BOI <= 2.000000000000002e0f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let COJ = if COG != 0.0 {
                        COE
                    } else {
                        let COH = COE.powf((BOI - F));
                        COH
                    };
                    COI = COJ;
                }
                let COK = F + (COE * COI);
                let COL = (BNP * COK) * (COK.powf(((-1e0f64 / BOI) - F)));
                let COM = (BOW + COL) / AR;
                let COO = CON * CON;
                let COP = BD * CON;
                let COQ = ((((DE * SF) * AVK) * BOW) * ((((((F + COP) + (IS * COO)) * COL) * COL) + ((((BD + (AX * CON)) + (BD * COO)) * COL) * BOW)) + ((((IS + COP) + COO) * BOW) * BOW))) / ((((1.5e1f64 * BOE) * (F + CON)) * COM) * COM);
                CPC = COQ;
                CRU = COL;
            } else {
                CPC = A;
                CRU = A;
            }
            let COT = if (if BUK != 0.0 && (if COR == F { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && CNK != 0.0 { 1.0 } else { 0.0 };
            let CRQ;
            let CRV;
            let CRY;
            let CSB;
            if COT != 0.0 {
                let COV = COU.sqrt();
                let COW = AVK + COV;
                let COY = (((4.2e1f64 * COX) * COU) + (AX * ((COX * COX) + (COU * COU)))) + (((APM * COV) * AVK) * (COX + COU));
                let COZ = COW * COW;
                let CPA = COY / ((COZ * COZ) * COW);
                let CPB = ((DE / BOE) * BOW) * SF;
                let CPE = ((3.872983346207417e0f64 * CPD) * ((COX + ((AX * AVK) * COV)) + COU)) / ((IS * COW) * (((((CPC / (CPB * AVK)) * COW) * AVK) * COY).sqrt()));
                CRQ = CPB;
                CRV = COV;
                CRY = CPA;
                CSB = CPE;
            } else {
                CRQ = B;
                CRV = A;
                CRY = A;
                CSB = A;
            }
            let CQJ = if H != 0.0 {
                let CPG = ((-parameters[172]) * CN) * (CAH - CPF);
                CPG
            } else {
                A
            };
            let CPH = (2.1983327444149834e-11f64 * DF) * ((F + (parameters[171] / MD)).ln());
            let CPK = CPI + (CPH * (CAH - CAI));
            let CPN = CPL + (CPH * CAH);
            let CPO = BLS * (-5e-1f64 * BKC);
            let CPP = BLS * (-5e-1f64 * (BMM + BMO));
            let CPS;
            let CPT;
            if CPQ != 0.0 {
                CPS = BLT;
                CPT = A;
            } else {
                let CPR = (BLT + CPO) + CPP;
                CPS = CPR;
                CPT = BMA;
            }
            let CQI = if E != 0.0 {
                A
            } else {
                let CPU = (-CPT) - CPS;
                CPU
            };
            let CPV = if BZS == A { 1.0 } else { 0.0 };
            let CQF;
            if CPV != 0.0 {
                CQF = A;
            } else {
                let CPZ = (CPW * LE) + AVC;
                let CQA = if CPZ > CNZ { 1.0 } else { 0.0 };
                let CQC = if CQA != 0.0 {
                    CNZ
                } else {
                    CPZ
                };
                let CQB = OA + AVC;
                let CQD = (((CQB - ((BKY * CQB) + ((F - BKY) * CQC))) / BZS) - CPW) * ((GH * DF) * (((2.069886e-10f64 / GG).sqrt()) * 1.3e0f64));
                CQF = CQD;
            }
            let CQE = if HK != A { 1.0 } else { 0.0 };
            let CQK = if CQE != 0.0 {
                let CQG = CQF + (HL * OC);
                CQG
            } else {
                CQF
            };
            let CQH = if parameters[14] == F { 1.0 } else { 0.0 };
            let CRK = if CQH != 0.0 {
                let CQR = CQI + (((((CPK + CPN) - CQJ) - CQK) - CQL) - CQO);
                CQR
            } else {
                CQI
            };
            let CQU = -CQS;
            let CQV = if CMP == F { 1.0 } else { 0.0 };
            let CVY = if CQV != 0.0 {
                let CRC = (CQW * CQX) - CRA;
                CRC
            } else {
                let CRF = ((F - CQW) * CQX) - CRD;
                CRF
            };
            let CVZ = if CQV != 0.0 {
                let CRG = ((F - CQW) * CQX) - CRD;
                CRG
            } else {
                let CRH = (CQW * CQX) - CRA;
                CRH
            };
            if CQV != 0.0 {
            } else {
            }
            if CQV != 0.0 {
            } else {
            }
            let CRI = 5.5224904e-23f64 * IX;
            let CRL = HP * 0e0f64;
            let CRM = HP * 0e0f64;
            let CRN = if CMP > A { 1.0 } else { 0.0 };
            let CRO = if CRN != 0.0 {
                CRM
            } else {
                CRL
            };
            let CSI;
            let CSJ;
            if COT != 0.0 {
                let CRP = ((BU * SF) * DF) * LE;
                let CRR = (((1.898893985185185e-20f64 * JE) * CRO) * CRO) / CRQ;
                let CRS = if (if CPD > 2.220446049250313e-15f64 { 1.0 } else { 0.0 }) != 0.0 && (if OA > 2.220446049250313e-15f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let CRZ = if CRS != 0.0 {
                    let CRT = BNP / BOW;
                    let CRW = CRT + (((6.666666666666667e-1f64 * (((BNP / CRU) - CRT) / OA)) * ((COX + (AVK * CRV)) + COU)) / (AVK + CRV));
                    CRW
                } else {
                    let CRX = BNP / CRU;
                    CRX
                };
                let CSA = (CRR * CRY) * CRZ;
                let CSC = if CSA < A { 1.0 } else { 0.0 };
                let CSD = if CSC != 0.0 {
                    A
                } else {
                    CSA
                };
                let CSE = if (-CRO) > CRP { 1.0 } else { 0.0 };
                let CSF = if CSE != 0.0 {
                    CSD
                } else {
                    A
                };
                let CSG = if CSE != 0.0 {
                    CSB
                } else {
                    A
                };
                CSI = CSG;
                CSJ = CSF;
            } else {
                CSI = A;
                CSJ = A;
            }
            let CSH = CRI * CPC;
            let CSK = if (if CSH > A { 1.0 } else { 0.0 }) != 0.0 && (if CSJ > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            if CSK != 0.0 {
            } else {
            }
            if CRN != 0.0 {
            } else {
            }
            if CRN != 0.0 {
            } else {
            }
            let CSM = if CSL == F { 1.0 } else { 0.0 };
            let CWF;
            if CSM != 0.0 {
                let CSN = parameters[315] / BU;
                let CSP = if CSO > A { 1.0 } else { 0.0 };
                let CSR = if CSP != 0.0 {
                    let CSQ = CSO * parameters[308];
                    CSQ
                } else {
                    A
                };
                let CST = HP * (HR - HX);
                let CSV = ((CSU * CSU) + (KM * KM)).sqrt();
                let CSZ = parameters[324] + (CSY * IZ);
                let CTG = ((parameters[317] / CH) / (JG.powf(CSW))) * (F + (CTA / (CR.powf(CTB))));
                let CTH = ((((parameters[319] / KO) / (((KA + KB) + KC) - (CSX * KD))) * (F + (CTE / (CS.powf(CTF))))) * (F + (CTC / (CR.powf(CTD))))) + BQ;
                let CTI = CTG * (CST / CSS);
                let CTJ = if CST >= A { 1.0 } else { 0.0 };
                let CTO = if CTJ != 0.0 {
                    let CTK = CTI / CTH;
                    CTK
                } else {
                    let CTL = (-CTI) / CTH;
                    CTL
                };
                let CTM = if (if 9.999999999999978e-1f64 <= CSZ { 1.0 } else { 0.0 }) != 0.0 && (if CSZ <= 1.0000000000000022e0f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let CTQ;
                if CTM != 0.0 {
                    CTQ = F;
                } else {
                    let CTN = if (if 1.9999999999999978e0f64 <= CSZ { 1.0 } else { 0.0 }) != 0.0 && (if CSZ <= 2.000000000000002e0f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let CTR = if CTN != 0.0 {
                        CTO
                    } else {
                        let CTP = CTO.powf((CSZ - F));
                        CTP
                    };
                    CTQ = CTR;
                }
                let CTS = F + (CTO * CTQ);
                let CTT = if (if 9.999999999999978e-1f64 <= CSZ { 1.0 } else { 0.0 }) != 0.0 && (if CSZ <= 1.0000000000000022e0f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let CTY;
                if CTT != 0.0 {
                    let CTU = F / CTS;
                    CTY = CTU;
                } else {
                    let CTV = if (if 1.9999999999999978e0f64 <= CSZ { 1.0 } else { 0.0 }) != 0.0 && (if CSZ <= 2.000000000000002e0f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let CTZ = if CTV != 0.0 {
                        let CTW = F / (CTS.sqrt());
                        CTW
                    } else {
                        let CTX = CTS * (CTS.powf(((-1e0f64 / CSZ) - F)));
                        CTX
                    };
                    CTY = CTZ;
                }
                let CUA = (((GE / CSS) * CSV) * (CTG * CTY)) * CSN;
                let CUB = if CUA <= A { 1.0 } else { 0.0 };
                let CUC = if CUB != 0.0 {
                    BQ
                } else {
                    CUA
                };
                let CUD = ((F / CUC) / DE) + CSR;
                let CUE = if (if CUD > BZ { 1.0 } else { 0.0 }) != 0.0 && BUJ != 0.0 { 1.0 } else { 0.0 };
                let CUG = if CUE != 0.0 {
                    let CUF = F / CUD;
                    CUF
                } else {
                    A
                };
                let CUH = if CUD < BZ { 1.0 } else { 0.0 };
                if CUH != 0.0 {
                } else {
                }
                CWF = CUG;
            } else {
                CWF = A;
            }
            let CUJ = if CUI == F { 1.0 } else { 0.0 };
            let CWH;
            if CUJ != 0.0 {
                let CUK = if CSO > A { 1.0 } else { 0.0 };
                let CUM = if CUK != 0.0 {
                    let CUL = CSO * parameters[309];
                    CUL
                } else {
                    A
                };
                let CUO = HP * (HZ - HT);
                let CUP = ((CSU * CSU) + (KM * KM)).sqrt();
                let CUQ = parameters[323] + (CSY * IZ);
                let CUR = ((parameters[316] / CH) / (JG.powf(CSW))) * (F + (CTA / (CR.powf(CTB))));
                let CUS = ((((parameters[318] / KO) / (((KA + KB) + KC) - (CSX * KD))) * (F + (CTE / (CS.powf(CTF))))) * (F + (CTC / (CR.powf(CTD))))) + BQ;
                let CUT = CUR * (CUO / CUN);
                let CUU = if CUO >= A { 1.0 } else { 0.0 };
                let CUZ = if CUU != 0.0 {
                    let CUV = CUT / CUS;
                    CUV
                } else {
                    let CUW = (-CUT) / CUS;
                    CUW
                };
                let CUX = if (if 9.999999999999978e-1f64 <= CUQ { 1.0 } else { 0.0 }) != 0.0 && (if CUQ <= 1.0000000000000022e0f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let CVB;
                if CUX != 0.0 {
                    CVB = F;
                } else {
                    let CUY = if (if 1.9999999999999978e0f64 <= CUQ { 1.0 } else { 0.0 }) != 0.0 && (if CUQ <= 2.000000000000002e0f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let CVC = if CUY != 0.0 {
                        CUZ
                    } else {
                        let CVA = CUZ.powf((CUQ - F));
                        CVA
                    };
                    CVB = CVC;
                }
                let CVD = F + (CUZ * CVB);
                let CVE = if (if 9.999999999999978e-1f64 <= CUQ { 1.0 } else { 0.0 }) != 0.0 && (if CUQ <= 1.0000000000000022e0f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let CVJ;
                if CVE != 0.0 {
                    let CVF = F / CVD;
                    CVJ = CVF;
                } else {
                    let CVG = if (if 1.9999999999999978e0f64 <= CUQ { 1.0 } else { 0.0 }) != 0.0 && (if CUQ <= 2.000000000000002e0f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let CVK = if CVG != 0.0 {
                        let CVH = F / (CVD.sqrt());
                        CVH
                    } else {
                        let CVI = CVD * (CVD.powf(((-1e0f64 / CUQ) - F)));
                        CVI
                    };
                    CVJ = CVK;
                }
                let CVL = (((GE / CUN) * CUP) * (CUR * CVJ)) * CD;
                let CVM = if CVL <= A { 1.0 } else { 0.0 };
                let CVN = if CVM != 0.0 {
                    BQ
                } else {
                    CVL
                };
                let CVO = ((F / CVN) / DE) + CUM;
                let CVP = if (if CVO > BZ { 1.0 } else { 0.0 }) != 0.0 && BUJ != 0.0 { 1.0 } else { 0.0 };
                let CVR = if CVP != 0.0 {
                    let CVQ = F / CVO;
                    CVQ
                } else {
                    A
                };
                let CVS = if CVO < BZ { 1.0 } else { 0.0 };
                if CVS != 0.0 {
                } else {
                }
                CWH = CVR;
            } else {
                CWH = A;
            }
            if E != 0.0 {
                let CVV = if CVT < BKG { 1.0 } else { 0.0 };
                if CVV != 0.0 {
                } else {
                }
                let CVX = if CVW < BKG { 1.0 } else { 0.0 };
                if CVX != 0.0 {
                } else {
                }
            } else {
            }
            if CQV != 0.0 {
            } else {
                if E != 0.0 {
                } else {
                }
            }
            if IC != 0.0 {
            } else {
            }
            let CWA = if CMP != F { 1.0 } else { 0.0 };
            if CWA != 0.0 {
            } else {
            }
            let CWB = if IP >= AY { 1.0 } else { 0.0 };
            if CWB != 0.0 {
            } else {
            }
            if CSL != 0.0 {
            } else {
            }
            if CUI != 0.0 {
            } else {
            }
            let CWC = CMP * CRJ;
            let CWE = (F - (CSI * CSI)) * CSH;
            let CWN;
            let CWO;
            if CSL != 0.0 {
                let CWG = CRI * CWF;
                CWN = F;
                CWO = CWG;
            } else {
                CWN = A;
                CWO = A;
            }
            let CWP;
            let CWQ;
            if CUI != 0.0 {
                let CWI = CRI * CWH;
                CWP = F;
                CWQ = CWI;
            } else {
                CWP = A;
                CWQ = A;
            }
            let CWJ = 3.2043836e-19f64 * CVY;
            let CWK = 3.2043836e-19f64 * CVZ;
            let CWL = 3.2043836e-19f64 * CQU;
            if HB != 0.0 {
            } else {
            }
            if IC != 0.0 {
            } else {
            }
            let CWM = if (if AMX != 0.0 && AJR != 0.0 { 1.0 } else { 0.0 }) != 0.0 && AMR != 0.0 { 1.0 } else { 0.0 };
            if CWM != 0.0 {
            } else {
            }
            if E != 0.0 {
            } else {
            }
        {
            let psd = CWC;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 0, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = Some(CWD);
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 0, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 0, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(0, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = CSH;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(1, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = CWE;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 2, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 2, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(2, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if CWN == 0.0 {
            if !visitor.visit(3, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = CWO;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 3, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 3, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(3, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if CWP == 0.0 {
            if !visitor.visit(4, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = CWQ;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 4, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 4, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(4, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = CWJ;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 5, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 5, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(5, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = CWK;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 6, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 6, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(6, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = CWL;
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
