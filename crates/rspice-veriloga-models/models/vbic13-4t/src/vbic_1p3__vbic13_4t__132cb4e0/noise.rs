#![allow(dead_code, non_snake_case, unused_parens, unused_variables)]

use super::state::Instance;
use rspice_veriloga_runtime::GeneratedEvalContext;
pub use rspice_veriloga_runtime::{GeneratedNoiseDescriptor, GeneratedNoiseEndpoint, GeneratedNoiseEvaluation, GeneratedNoiseEvaluationError, GeneratedNoiseEvaluationRef, GeneratedNoiseKind, GeneratedNoiseVisitor};

pub static NOISE_SOURCES: [GeneratedNoiseDescriptor; 15] = [
    GeneratedNoiseDescriptor { mechanism: "WHITE_BI_EI_IBEI_SHOT_NOISE", label: Some("Ibei shot noise"), kind: GeneratedNoiseKind::White, equation: 31, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(8), name: "bi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(9), name: "ei", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_BI_EI_IBEI_FLICKER_NOISE", label: Some("Ibei flicker noise"), kind: GeneratedNoiseKind::Flicker, equation: 32, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(8), name: "bi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(9), name: "ei", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_BX_EI_IBEX_SHOT_NOISE", label: Some("Ibex shot noise"), kind: GeneratedNoiseKind::White, equation: 33, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(7), name: "bx", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(9), name: "ei", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_BX_EI_IBEX_FLICKER_NOISE", label: Some("Ibex flicker noise"), kind: GeneratedNoiseKind::Flicker, equation: 34, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(7), name: "bx", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(9), name: "ei", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_CI_EI_TRANSPORT_CURRENT_SHOT_NOISE", label: Some("transport current shot noise"), kind: GeneratedNoiseKind::White, equation: 35, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(6), name: "ci", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(9), name: "ei", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_BX_BP_IBEP_SHOT_NOISE", label: Some("Ibep shot noise"), kind: GeneratedNoiseKind::White, equation: 36, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(7), name: "bx", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(10), name: "bp", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_BX_BP_IBEP_FLICKER_NOISE", label: Some("Ibep flicker noise"), kind: GeneratedNoiseKind::Flicker, equation: 37, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(7), name: "bx", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(10), name: "bp", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C_CX_RCX_THERMAL_NOISE", label: Some("rcx thermal noise"), kind: GeneratedNoiseKind::White, equation: 38, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(0), name: "c", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(5), name: "cx", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_CX_CI_RCI_THERMAL_NOISE", label: Some("rci thermal noise"), kind: GeneratedNoiseKind::White, equation: 39, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "cx", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(6), name: "ci", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_B_BX_RBX_THERMAL_NOISE", label: Some("rbx thermal noise"), kind: GeneratedNoiseKind::White, equation: 40, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(1), name: "b", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(7), name: "bx", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_BX_BI_RBI_THERMAL_NOISE", label: Some("rbi thermal noise"), kind: GeneratedNoiseKind::White, equation: 41, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(7), name: "bx", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(8), name: "bi", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_E_EI_RE_THERMAL_NOISE", label: Some("re thermal noise"), kind: GeneratedNoiseKind::White, equation: 42, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(2), name: "e", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(9), name: "ei", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_BP_CX_RBP_THERMAL_NOISE", label: Some("rbp thermal noise"), kind: GeneratedNoiseKind::White, equation: 43, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(10), name: "bp", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(5), name: "cx", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_BX_SI_PARASITIC_TRANSPORT_CURRENT_SHOT_NOISE", label: Some("parasitic transport current shot noise"), kind: GeneratedNoiseKind::White, equation: 44, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(7), name: "bx", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(11), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_S_SI_RS_THERMAL_NOISE", label: Some("rs thermal noise"), kind: GeneratedNoiseKind::White, equation: 45, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(3), name: "s", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(11), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
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
            let B = if parameter_given[6] { 1.0 } else { 0.0 };
            let C = 1e0f64;
            let D = if parameter_given[7] { 1.0 } else { 0.0 };
            let E = 1e-2f64;
            let F = if parameter_given[10] { 1.0 } else { 0.0 };
            let G = parameters[10];
            let I = if parameter_given[11] { 1.0 } else { 0.0 };
            let J = parameters[11];
            let L = if parameter_given[3] { 1.0 } else { 0.0 };
            let M = if parameter_given[4] { 1.0 } else { 0.0 };
            let N = -1e0f64;
            let O = if parameter_given[5] { 1.0 } else { 0.0 };
            let P = parameters[5];
            let R = parameters[74];
            let V = parameters[75];
            let Z = parameters[20];
            let AD = parameters[79];
            let AI = 2.7315e2f64;
            let AO = parameters[14];
            let AS = parameters[15];
            let AY = 1.380662e-23f64;
            let AZ = 1.602189e-19f64;
            let BC = parameters[26];
            let BD = parameters[90];
            let BF = parameters[89];
            let BH = parameters[88];
            let BK = parameters[122];
            let BL = parameters[28];
            let BS = parameters[72];
            let BU = 5e-1f64;
            let BV = 4e0f64;
            let BW = parameters[73];
            let BZ = parameters[27];
            let CA = parameters[29];
            let CJ = parameters[31];
            let CK = parameters[33];
            let CT = parameters[54];
            let CU = parameters[123];
            let CV = parameters[56];
            let CX = parameters[114];
            let DD = parameters[58];
            let DE = parameters[124];
            let DF = parameters[59];
            let DM = parameters[61];
            let DP = parameters[115];
            let DW = parameters[63];
            let EF = parameters[64];
            let EJ = parameters[65];
            let EN = parameters[66];
            let EO = parameters[67];
            let EP = parameters[116];
            let EU = parameters[68];
            let EV = parameters[69];
            let FM = if parameter_given[109] { 1.0 } else { 0.0 };
            let FN = parameters[16];
            let FP = parameters[107];
            let FR = if parameter_given[108] { 1.0 } else { 0.0 };
            let FS = parameters[17];
            let FV = if parameter_given[106] { 1.0 } else { 0.0 };
            let FW = parameters[21];
            let FY = parameters[104];
            let GA = if parameter_given[105] { 1.0 } else { 0.0 };
            let GB = parameters[22];
            let GG = if parameter_given[110] { 1.0 } else { 0.0 };
            let GH = parameters[25];
            let HE = 2e0f64;
            let HG = parameters[37];
            let HM = parameters[42];
            let HP = parameters[50];
            let HS = parameters[38];
            let HT = parameters[43];
            let IC = 1e-3f64;
            let IF = 1e3f64;
            let JT = node_potentials[8];
            let JU = node_potentials[9];
            let JW = node_potentials[7];
            let JY = node_potentials[6];
            let KA = node_potentials[5];
            let KD = node_potentials[10];
            let KI = parameters[34];
            let KK = parameters[39];
            let LH = parameters[44];
            let LP = parameters[45];
            let LZ = parameters[46];
            let NP = 1e-4f64;
            let NR = 1e-8f64;
            let OR = parameters[32];
            let OY = 5.0005e-1f64;
            let PF = parameters[55];
            let PR = parameters[57];
            let TX = 2e-2f64;
            let TY = 1.01e0f64;
            let UD = parameters[87];
            let VP = parameters[1];
            let VS = parameters[98];
            let VT = parameters[99];
            let VV = parameters[100];
            let WE = 1e-10f64;
            if B != 0.0 {
            } else {
            }
            if D != 0.0 {
            } else {
            }
            let UY = if F != 0.0 {
                G
            } else {
                let H = ctx.simparam_or("gmin", 1e-12f64);
                H
            };
            let BI = if I != 0.0 {
                J
            } else {
                let K = ctx.simparam_or("pnjmaxi", C);
                K
            };
            let JQ;
            if L != 0.0 {
                JQ = C;
            } else {
                let JR;
                if M != 0.0 {
                    JR = N;
                } else {
                    let JS = if O != 0.0 {
                        P
                    } else {
                        C
                    };
                    JR = JS;
                }
                JQ = JR;
            }
            let Q = parameters[12].ln();
            let S = if R > A { 1.0 } else { 0.0 };
            let U = if S != 0.0 {
                let T = C / R;
                T
            } else {
                A
            };
            let W = if V > A { 1.0 } else { 0.0 };
            let Y = if W != 0.0 {
                let X = C / V;
                X
            } else {
                A
            };
            let AA = if Z > A { 1.0 } else { 0.0 };
            let AC = if AA != 0.0 {
                let AB = C / Z;
                AB
            } else {
                A
            };
            let AE = if AD > A { 1.0 } else { 0.0 };
            let AG = if AE != 0.0 {
                let AF = C / AD;
                AF
            } else {
                A
            };
            let AH = if parameters[80] > A { 1.0 } else { 0.0 };
            if AH != 0.0 {
            } else {
            }
            if AH != 0.0 {
            } else {
            }
            let AJ = AI + parameters[13];
            let AK = temperature + parameters[0];
            let AL = AK - AI;
            let AM = if AL < parameters[8] { 1.0 } else { 0.0 };
            if AM != 0.0 {
            } else {
            }
            let AN = if AL > parameters[9] { 1.0 } else { 0.0 };
            if AN != 0.0 {
            } else {
            }
            let AP = AO + C;
            let AQ = if AL < AP { 1.0 } else { 0.0 };
            let AV;
            if AQ != 0.0 {
                let AR = AO + (((AL - AO) - C).exp());
                AV = AR;
            } else {
                let AT = if AL > (AS - C) { 1.0 } else { 0.0 };
                let AW = if AT != 0.0 {
                    let AU = AS - (((AS - AL) - C).exp());
                    AU
                } else {
                    AL
                };
                AV = AW;
            }
            let AX = AV + AI;
            let BA = (AY * AX) / AZ;
            let BB = AX / AJ;
            let BE = if BD > A { 1.0 } else { 0.0 };
            let QA = if BE != 0.0 {
                let BG = BF * BA;
                let BJ = BG * (((((-BH) / BG).exp()) + (BI / BD)).ln());
                BJ
            } else {
                A
            };
            let BM = BK / BL;
            let BN = -parameters[113];
            let BO = C - BB;
            let BP = BA * BL;
            let BQ = (BC * (BB.powf(BM))) * (((BN * BO) / BP).exp());
            let BR = if BQ > A { 1.0 } else { 0.0 };
            let MY;
            if BR != 0.0 {
                let BT = if (if BS > A { 1.0 } else { 0.0 }) != 0.0 && (if BI > BS { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let MZ = if BT != 0.0 {
                    let BX = BP * ((C + ((((BU * BI) * ((BV / BS).powf(BW))).powf((C / (C - BW)))) / BQ)).ln());
                    BX
                } else {
                    let BY = BP * ((C + (BI / BQ)).ln());
                    BY
                };
                MY = MZ;
            } else {
                MY = A;
            }
            let CB = parameters[125] / CA;
            let CC = -parameters[121];
            let CD = BA * CA;
            let CE = (BZ * (BB.powf(CB))) * (((CC * BO) / CD).exp());
            let CF = if BR != 0.0 && (if CE > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let NG;
            if CF != 0.0 {
                let CG = if S != 0.0 && (if BI > R { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let NH = if CG != 0.0 {
                    let CH = CD * ((C + ((((BU * BI) * ((BV / R).powf(BW))).powf((C / (C - BW)))) / (BQ * CE))).ln());
                    CH
                } else {
                    let CI = CD * ((C + (BI / (BQ * CE))).ln());
                    CI
                };
                NG = NH;
            } else {
                NG = A;
            }
            let CL = BK / CK;
            let CM = -parameters[120];
            let CN = BA * CK;
            let CO = (CJ * (BB.powf(CL))) * (((CM * BO) / CN).exp());
            let CP = if CO > A { 1.0 } else { 0.0 };
            let OJ;
            if CP != 0.0 {
                let CQ = if W != 0.0 && (if BI > V { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let OK = if CQ != 0.0 {
                    let CR = CN * ((C + (((BI * BI) * Y) / CO)).ln());
                    CR
                } else {
                    let CS = CN * ((C + (BI / CO)).ln());
                    CS
                };
                OJ = OK;
            } else {
                OJ = A;
            }
            let CW = CU / CV;
            let CY = -CX;
            let CZ = BA * CV;
            let DA = (CT * (BB.powf(CW))) * (((CY * BO) / CZ).exp());
            let DB = if DA > A { 1.0 } else { 0.0 };
            let PI = if DB != 0.0 {
                let DC = CZ * ((C + (BI / DA)).ln());
                DC
            } else {
                A
            };
            let DG = DE / DF;
            let DH = -parameters[117];
            let DI = BA * DF;
            let DJ = (DD * (BB.powf(DG))) * (((DH * BO) / DI).exp());
            let DK = if DJ > A { 1.0 } else { 0.0 };
            let PN = if DK != 0.0 {
                let DL = DI * ((C + (BI / DJ)).ln());
                DL
            } else {
                A
            };
            let DN = CU / DM;
            let DO = BB.powf(DN);
            let DQ = -DP;
            let DR = BA * DM;
            let DS = ((DQ * BO) / DR).exp();
            let DT = (parameters[60] * DO) * DS;
            let DU = if DT > A { 1.0 } else { 0.0 };
            let SP = if DU != 0.0 {
                let DV = DR * ((C + (BI / DT)).ln());
                DV
            } else {
                A
            };
            let DX = DE / DW;
            let DY = BB.powf(DX);
            let DZ = -parameters[118];
            let EA = BA * DW;
            let EB = ((DZ * BO) / EA).exp();
            let EC = (parameters[62] * DY) * EB;
            let ED = if EC > A { 1.0 } else { 0.0 };
            let SS = if ED != 0.0 {
                let EE = EA * ((C + (BI / EC)).ln());
                EE
            } else {
                A
            };
            let EG = (EF * DO) * DS;
            let EH = if EG > A { 1.0 } else { 0.0 };
            let SV = if EH != 0.0 {
                let EI = DR * ((C + (BI / EG)).ln());
                EI
            } else {
                A
            };
            let EK = (EJ * DY) * EB;
            let EL = if EK > A { 1.0 } else { 0.0 };
            let SZ = if EL != 0.0 {
                let EM = EA * ((C + (BI / EK)).ln());
                EM
            } else {
                A
            };
            let EQ = BA * EO;
            let ER = (EN * (BB.powf((CU / EO)))) * ((((-EP) * BO) / EQ).exp());
            let ES = if ER > A { 1.0 } else { 0.0 };
            let UK = if ES != 0.0 {
                let ET = EQ * ((C + (BI / ER)).ln());
                ET
            } else {
                A
            };
            let EW = BA * EV;
            let EX = (EU * (BB.powf((DE / EV)))) * ((((-parameters[119]) * BO) / EW).exp());
            let EY = if EX > A { 1.0 } else { 0.0 };
            let UM = if EY != 0.0 {
                let EZ = EW * ((C + (BI / EX)).ln());
                EZ
            } else {
                A
            };
            let FA = (AK + node_potentials[4]) - AI;
            let FB = if FA < AP { 1.0 } else { 0.0 };
            let FF;
            if FB != 0.0 {
                let FC = AO + (((FA - AO) - C).exp());
                FF = FC;
            } else {
                let FD = if FA > (AS - C) { 1.0 } else { 0.0 };
                let FG = if FD != 0.0 {
                    let FE = AS - (((AS - FA) - C).exp());
                    FE
                } else {
                    FA
                };
                FF = FG;
            }
            let FH = FF + AI;
            let FI = (AY * FH) / AZ;
            let FJ = FH / AJ;
            let FK = FH - AJ;
            let FL = BS * (FJ.powf(parameters[126]));
            let IB = if FM != 0.0 {
                let FO = FN * (FJ.powf(parameters[109]));
                FO
            } else {
                let FQ = FN * (FJ.powf(FP));
                FQ
            };
            let IH = if FR != 0.0 {
                let FT = FS * (FJ.powf(parameters[108]));
                FT
            } else {
                let FU = FS * (FJ.powf(FP));
                FU
            };
            let IL = if FV != 0.0 {
                let FX = FW * (FJ.powf(parameters[106]));
                FX
            } else {
                let FZ = FW * (FJ.powf(FY));
                FZ
            };
            let IP = if GA != 0.0 {
                let GC = GB * (FJ.powf(parameters[105]));
                GC
            } else {
                let GD = GB * (FJ.powf(FY));
                GD
            };
            let GE = parameters[23] * (FJ.powf(parameters[103]));
            let GF = parameters[24] * (FJ.powf(parameters[111]));
            let IW = if GG != 0.0 {
                let GI = GH * (FJ.powf(parameters[110]));
                GI
            } else {
                let GJ = GH * (FJ.powf(FP));
                GJ
            };
            let GK = parameters[101] * (C + (FK * parameters[132]));
            let GL = C - FJ;
            let GM = BN * GL;
            let GN = (BC * (FJ.powf(BM))) * ((GM / (FI * BL)).exp());
            let GO = (BZ * (FJ.powf(CB))) * (((CC * GL) / (FI * CA)).exp());
            let GP = FI * CK;
            let GQ = (CJ * (FJ.powf(CL))) * (((CM * GL) / GP).exp());
            let GR = FI * CV;
            let GS = (CT * (FJ.powf(CW))) * (((CY * GL) / GR).exp());
            let GT = FI * DF;
            let GU = (DD * (FJ.powf(DG))) * (((DH * GL) / GT).exp());
            let GV = FI * DM;
            let GW = FI * DW;
            let GX = (EF * (FJ.powf(DN))) * (((DQ * GL) / GV).exp());
            let GY = (EJ * (FJ.powf(DX))) * (((DZ * GL) / GW).exp());
            let GZ = C + (FK * parameters[129]);
            let HA = BL * GZ;
            let HB = CA * GZ;
            let HC = parameters[84] * (C + (FK * parameters[127]));
            let HD = parameters[86] * (C + (FK * parameters[128]));
            let HF = HE * (FI / FJ);
            let HH = (3e0f64 * FI) * (FJ.ln());
            let HI = FJ - C;
            let HJ = (((HF * ((((((BU * HG) * FJ) / FI).exp()) - ((((-5e-1f64 * HG) * FJ) / FI).exp())).ln())) * FJ) - HH) - (CX * HI);
            let HK = HE * FI;
            let HL = HJ + (HK * ((BU * (C + ((C + (BV * (((-HJ) / FI).exp()))).sqrt()))).ln()));
            let HN = (((HF * ((((((BU * HM) * FJ) / FI).exp()) - ((((-5e-1f64 * HM) * FJ) / FI).exp())).ln())) * FJ) - HH) - (DP * HI);
            let HO = HN + (HK * ((BU * (C + ((C + (BV * (((-HN) / FI).exp()))).sqrt()))).ln()));
            let HQ = (((HF * ((((((BU * HP) * FJ) / FI).exp()) - ((((-5e-1f64 * HP) * FJ) / FI).exp())).ln())) * FJ) - HH) - (EP * HI);
            let HR = HQ + (HK * ((BU * (C + ((C + (BV * (((-HQ) / FI).exp()))).sqrt()))).ln()));
            let HU = (parameters[19] * (FJ.powf(BK))) * ((GM / FI).exp());
            let HV = parameters[18] * (FJ.powf(parameters[112]));
            let HW = -(BH * (C + (FK * (parameters[91] + (FK * parameters[92])))));
            let HX = (BF * (C + (FK * parameters[93]))) * FI;
            let HY = (HW / HX).exp();
            let HZ = parameters[70] * (C + (FK * parameters[130]));
            let IA = parameters[71] * (C + (FK * parameters[131]));
            let ID = if IB > IC { 1.0 } else { 0.0 };
            let IG = if ID != 0.0 {
                let IE = C / IB;
                IE
            } else {
                IF
            };
            let II = if IH > IC { 1.0 } else { 0.0 };
            let IK = if II != 0.0 {
                let IJ = C / IH;
                IJ
            } else {
                IF
            };
            let IM = if IL > IC { 1.0 } else { 0.0 };
            let IO = if IM != 0.0 {
                let IN = C / IL;
                IN
            } else {
                IF
            };
            let IQ = if IP > IC { 1.0 } else { 0.0 };
            let IS = if IQ != 0.0 {
                let IR = C / IP;
                IR
            } else {
                IF
            };
            let IT = if GE > IC { 1.0 } else { 0.0 };
            let IV = if IT != 0.0 {
                let IU = C / GE;
                IU
            } else {
                IF
            };
            let IX = if IW > IC { 1.0 } else { 0.0 };
            let IZ = if IX != 0.0 {
                let IY = C / IW;
                IY
            } else {
                IF
            };
            let JA = if GF > IC { 1.0 } else { 0.0 };
            let JC = if JA != 0.0 {
                let JB = C / GF;
                JB
            } else {
                IF
            };
            let JD = if GK > IC { 1.0 } else { 0.0 };
            if JD != 0.0 {
            } else {
            }
            let JE = if HZ > A { 1.0 } else { 0.0 };
            let JG = if JE != 0.0 {
                let JF = C / HZ;
                JF
            } else {
                A
            };
            let JH = if IA > A { 1.0 } else { 0.0 };
            let JJ = if JH != 0.0 {
                let JI = C / IA;
                JI
            } else {
                A
            };
            let JK = if FL > A { 1.0 } else { 0.0 };
            let JM = if JK != 0.0 {
                let JL = C / FL;
                JL
            } else {
                A
            };
            let JN = if HV > A { 1.0 } else { 0.0 };
            let JP = if JN != 0.0 {
                let JO = C / HV;
                JO
            } else {
                A
            };
            let JV = JQ * (JT - JU);
            let JX = JQ * (JW - JU);
            let JZ = JQ * (JT - JY);
            let KB = JQ * (JT - KA);
            let KC = JQ * (JW - KA);
            let KE = JQ * (JW - KD);
            let KF = JQ * (KA - JY);
            let KG = JQ * (node_potentials[11] - KD);
            let KH = -HL;
            let KJ = KH * KI;
            let KL = if KK <= A { 1.0 } else { 0.0 };
            let NM;
            if KL != 0.0 {
                let KM = JV + KJ;
                let KN = if KM > A { 1.0 } else { 0.0 };
                let KU;
                let KV;
                if KN != 0.0 {
                    let KO = C - KI;
                    let KP = KO.powf((-HS));
                    let KQ = (HL * (C - (KP * KO))) / (C - HS);
                    let KR = (KM * (C + (((BU * HS) * KM) / (HL * KO)))) * KP;
                    KU = KQ;
                    KV = KR;
                } else {
                    let KS = C - HS;
                    let KT = (HL * (C - ((C - (JV / HL)).powf(KS)))) / KS;
                    KU = KT;
                    KV = A;
                }
                let KW = KU + KV;
                NM = KW;
            } else {
                let KX = (BV * KK) * KK;
                let KY = -5e-1f64 * (KJ + (((KJ * KJ) + KX).sqrt()));
                let KZ = C - HS;
                let LA = JV + KJ;
                let LB = (BU * (LA - (((LA * LA) + KX).sqrt()))) - KJ;
                let LC = C - KI;
                let LD = (JV - LB) + KY;
                let LE = (((KH * ((C - (LB / HL)).powf(KZ))) / KZ) + (((LC.powf((-HS))) * LD) * (C + (((BU * HS) * LD) / (HL * LC))))) - ((KH * ((C - (KY / HL)).powf(KZ))) / KZ);
                NM = LE;
            }
            let LF = -HO;
            let LG = LF * KI;
            let LI = if LH <= A { 1.0 } else { 0.0 };
            let NN;
            if LI != 0.0 {
                let LJ = JZ + LG;
                let LK = if LJ > A { 1.0 } else { 0.0 };
                let LV;
                let LX;
                if LK != 0.0 {
                    let LL = C - KI;
                    let LM = LL.powf((-1e0f64 - HT));
                    let LN = (HO * (C - ((LM * LL) * LL))) / (C - HT);
                    let LO = (LJ * (LL + (((BU * HT) * LJ) / HO))) * LM;
                    LV = LN;
                    LX = LO;
                } else {
                    let LQ = if (if LP > A { 1.0 } else { 0.0 }) != 0.0 && (if JZ < (-LP) { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let LW = if LQ != 0.0 {
                        let LR = C - HT;
                        let LS = (HO * (C - (((C + (LP / HO)).powf(LR)) * (C - ((LR * (JZ + LP)) / (HO + LP)))))) / LR;
                        LS
                    } else {
                        let LT = C - HT;
                        let LU = (HO * (C - ((C - (JZ / HO)).powf(LT)))) / LT;
                        LU
                    };
                    LV = LW;
                    LX = A;
                }
                let LY = LV + LX;
                NN = LY;
            } else {
                let MA = if (if LP > A { 1.0 } else { 0.0 }) != 0.0 && (if LZ > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let NO = if MA != 0.0 {
                    let MB = LP - LG;
                    let MC = (LP + LG) / MB;
                    let MD = MC - C;
                    let ME = (BV * LH) * LH;
                    let MF = MC + C;
                    let MG = (BV * LZ) * LZ;
                    let MH = BU * (((((HE * MC) / ((((MD * MD) + ME).sqrt()) + (((MF * MF) + MG).sqrt()))) * MB) - LP) - LG);
                    let MI = C - HT;
                    let MJ = (((HE * JZ) + LP) + LG) / MB;
                    let MK = MJ - C;
                    let ML = MJ + C;
                    let MM = (HE * MJ) / ((((MK * MK) + ME).sqrt()) + (((ML * ML) + MG).sqrt()));
                    let MN = BU * (((MM * MB) - LP) - LG);
                    let MO = BU * (MM + C);
                    let MP = -HT;
                    let MQ = ((((JZ - MN) + MH) * (((C - MO) * ((C + (LP / HO)).powf(MP))) + (MO * ((C + (LG / HO)).powf(MP))))) + ((HO * (C - ((C - (MN / HO)).powf(MI)))) / MI)) - ((HO * (C - ((C - (MH / HO)).powf(MI)))) / MI);
                    MQ
                } else {
                    let MR = (BV * LH) * LH;
                    let MS = -5e-1f64 * (LG + (((LG * LG) + MR).sqrt()));
                    let MT = C - HT;
                    let MU = JZ + LG;
                    let MV = (BU * (MU - (((MU * MU) + MR).sqrt()))) - LG;
                    let MW = (((LF * ((C - (MV / HO)).powf(MT))) / MT) + (((C - KI).powf((-HT))) * ((JZ - MV) + MS))) - ((LF * ((C - (MS / HO)).powf(MT))) / MT);
                    MW
                };
                NN = NO;
            }
            let MX = C / (HA * FI);
            let NA = if JV < MY { 1.0 } else { 0.0 };
            let ND = if NA != 0.0 {
                let NB = (JV * MX).exp();
                NB
            } else {
                let NC = ((MY * MX).exp()) * (C + ((JV - MY) * MX));
                NC
            };
            let NE = GN * (ND - C);
            let NF = C / (HB * FI);
            let NI = if JZ < NG { 1.0 } else { 0.0 };
            let NL = if NI != 0.0 {
                let NJ = (JZ * NF).exp();
                NJ
            } else {
                let NK = ((NG * NF).exp()) * (C + ((JZ - NG) * NF));
                NK
            };
            let NQ = ((C + (NM * JJ)) + (NN * JG)) - NP;
            let NS = (BU * ((((NQ * NQ) + NR).sqrt()) + NQ)) + NP;
            let NT = (NE * JM) + (((GN * GO) * (NL - C)) * U);
            let NU = if parameters[30] < BU { 1.0 } else { 0.0 };
            let OD;
            if NU != 0.0 {
                let NV = (NS.powf((C / BW))) + (BV * NT);
                let NW = if NV > NR { 1.0 } else { 0.0 };
                let OE = if NW != 0.0 {
                    let NX = BU * (NS + (NV.powf(BW)));
                    NX
                } else {
                    let NY = BU * (NS + (NR.powf(BW)));
                    NY
                };
                OD = OE;
            } else {
                let NZ = C + (BV * NT);
                let OA = if NZ > NR { 1.0 } else { 0.0 };
                let OF = if OA != 0.0 {
                    let OB = (BU * NS) * (C + (NZ.powf(BW)));
                    OB
                } else {
                    let OC = (BU * NS) * (C + (NR.powf(BW)));
                    OC
                };
                OD = OF;
            }
            let OG = NE / OD;
            let OH = if CJ > A { 1.0 } else { 0.0 };
            let TV;
            let UX;
            if OH != 0.0 {
                let OI = C / GP;
                let OL = if KE < OJ { 1.0 } else { 0.0 };
                let OS = if OL != 0.0 {
                    let OM = (KE * OI).exp();
                    OM
                } else {
                    let ON = ((OJ * OI).exp()) * (C + ((KE - OJ) * OI));
                    ON
                };
                let OO = if JZ < OJ { 1.0 } else { 0.0 };
                let OT = if OO != 0.0 {
                    let OP = (JZ * OI).exp();
                    OP
                } else {
                    let OQ = ((OJ * OI).exp()) * (C + ((JZ - OJ) * OI));
                    OQ
                };
                let OU = GQ * (((OR * OS) + ((C - OR) * OT)) - C);
                let OV = C + (BV * (OU * Y));
                let OW = if OV > NR { 1.0 } else { 0.0 };
                let PD = if OW != 0.0 {
                    let OX = BU * (C + (OV.sqrt()));
                    OX
                } else {
                    OY
                };
                let OZ = if KG < OJ { 1.0 } else { 0.0 };
                let PC = if OZ != 0.0 {
                    let PA = (KG * OI).exp();
                    PA
                } else {
                    let PB = ((OJ * OI).exp()) * (C + ((KG - OJ) * OI));
                    PB
                };
                let PE = (OU - (GQ * (PC - C))) / PD;
                TV = PD;
                UX = PE;
            } else {
                TV = C;
                UX = A;
            }
            let PG = if PF == C { 1.0 } else { 0.0 };
            let UO;
            let US;
            if PG != 0.0 {
                let PH = C / GR;
                let PJ = if JV < PI { 1.0 } else { 0.0 };
                let PT = if PJ != 0.0 {
                    let PK = (JV * PH).exp();
                    PK
                } else {
                    let PL = ((PI * PH).exp()) * (C + ((JV - PI) * PH));
                    PL
                };
                let PM = C / GT;
                let PO = if JV < PN { 1.0 } else { 0.0 };
                let PU = if PO != 0.0 {
                    let PP = (JV * PM).exp();
                    PP
                } else {
                    let PQ = ((PN * PM).exp()) * (C + ((JV - PN) * PM));
                    PQ
                };
                let PS = if PR > A { 1.0 } else { 0.0 };
                let QE = if PS != 0.0 {
                    let PV = ((GS * (C + (PR * (NS - C)))) * (PT - C)) + (GU * (PU - C));
                    PV
                } else {
                    let PW = (GS * (PT - C)) + (GU * (PU - C));
                    PW
                };
                let PX = if BH > A { 1.0 } else { 0.0 };
                let UP;
                if PX != 0.0 {
                    let PY = HW - JV;
                    let PZ = C / HX;
                    let QB = if PY < QA { 1.0 } else { 0.0 };
                    let QF = if QB != 0.0 {
                        let QC = (PY * PZ).exp();
                        QC
                    } else {
                        let QD = ((QA * PZ).exp()) * (C + ((PY - QA) * PZ));
                        QD
                    };
                    let QG = QE - (BD * (QF - HY));
                    UP = QG;
                } else {
                    UP = QE;
                }
                UO = UP;
                US = A;
            } else {
                let QH = if PF == A { 1.0 } else { 0.0 };
                let UQ;
                let UT;
                if QH != 0.0 {
                    let QI = C / GR;
                    let QJ = if JX < PI { 1.0 } else { 0.0 };
                    let QQ = if QJ != 0.0 {
                        let QK = (JX * QI).exp();
                        QK
                    } else {
                        let QL = ((PI * QI).exp()) * (C + ((JX - PI) * QI));
                        QL
                    };
                    let QM = C / GT;
                    let QN = if JX < PN { 1.0 } else { 0.0 };
                    let QR = if QN != 0.0 {
                        let QO = (JX * QM).exp();
                        QO
                    } else {
                        let QP = ((PN * QM).exp()) * (C + ((JX - PN) * QM));
                        QP
                    };
                    let QS = (GS * (QQ - C)) + (GU * (QR - C));
                    let QT = if BH > A { 1.0 } else { 0.0 };
                    let UU;
                    if QT != 0.0 {
                        let QU = HW - JV;
                        let QV = C / HX;
                        let QW = if QU < QA { 1.0 } else { 0.0 };
                        let QZ = if QW != 0.0 {
                            let QX = (QU * QV).exp();
                            QX
                        } else {
                            let QY = ((QA * QV).exp()) * (C + ((QU - QA) * QV));
                            QY
                        };
                        let RA = QS - (BD * (QZ - HY));
                        UU = RA;
                    } else {
                        UU = QS;
                    }
                    UQ = A;
                    UT = UU;
                } else {
                    let RB = C / GR;
                    let RC = if JV < PI { 1.0 } else { 0.0 };
                    let RK = if RC != 0.0 {
                        let RD = (JV * RB).exp();
                        RD
                    } else {
                        let RE = ((PI * RB).exp()) * (C + ((JV - PI) * RB));
                        RE
                    };
                    let RF = C / GT;
                    let RG = if JV < PN { 1.0 } else { 0.0 };
                    let RL = if RG != 0.0 {
                        let RH = (JV * RF).exp();
                        RH
                    } else {
                        let RI = ((PN * RF).exp()) * (C + ((JV - PN) * RF));
                        RI
                    };
                    let RJ = if PR > A { 1.0 } else { 0.0 };
                    let RU = if RJ != 0.0 {
                        let RM = PF * (((GS * (C + (PR * (NS - C)))) * (RK - C)) + (GU * (RL - C)));
                        RM
                    } else {
                        let RN = PF * ((GS * (RK - C)) + (GU * (RL - C)));
                        RN
                    };
                    let RO = if BH > A { 1.0 } else { 0.0 };
                    let UR;
                    if RO != 0.0 {
                        let RP = HW - JV;
                        let RQ = C / HX;
                        let RR = if RP < QA { 1.0 } else { 0.0 };
                        let RV = if RR != 0.0 {
                            let RS = (RP * RQ).exp();
                            RS
                        } else {
                            let RT = ((QA * RQ).exp()) * (C + ((RP - QA) * RQ));
                            RT
                        };
                        let RW = RU - ((PF * BD) * (RV - HY));
                        UR = RW;
                    } else {
                        UR = RU;
                    }
                    let RX = if JX < PI { 1.0 } else { 0.0 };
                    let SE = if RX != 0.0 {
                        let RY = (JX * RB).exp();
                        RY
                    } else {
                        let RZ = ((PI * RB).exp()) * (C + ((JX - PI) * RB));
                        RZ
                    };
                    let SA = if JX < PN { 1.0 } else { 0.0 };
                    let SF = if SA != 0.0 {
                        let SB = (JX * RF).exp();
                        SB
                    } else {
                        let SC = ((PN * RF).exp()) * (C + ((JX - PN) * RF));
                        SC
                    };
                    let SD = C - PF;
                    let SG = SD * ((GS * (SE - C)) + (GU * (SF - C)));
                    let UV;
                    if RO != 0.0 {
                        let SH = HW - JV;
                        let SI = C / HX;
                        let SJ = if SH < QA { 1.0 } else { 0.0 };
                        let SM = if SJ != 0.0 {
                            let SK = (SH * SI).exp();
                            SK
                        } else {
                            let SL = ((QA * SI).exp()) * (C + ((SH - QA) * SI));
                            SL
                        };
                        let SN = SG - ((SD * BD) * (SM - HY));
                        UV = SN;
                    } else {
                        UV = SG;
                    }
                    UQ = UR;
                    UT = UV;
                }
                UO = UQ;
                US = UT;
            }
            let SO = C / GV;
            let SQ = if JZ < SP { 1.0 } else { 0.0 };
            if SQ != 0.0 {
            } else {
            }
            let SR = C / GW;
            let ST = if JZ < SS { 1.0 } else { 0.0 };
            if ST != 0.0 {
            } else {
            }
            let SU = if (if EF > A { 1.0 } else { 0.0 }) != 0.0 || (if EJ > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let UW;
            if SU != 0.0 {
                let SW = if KE < SV { 1.0 } else { 0.0 };
                let TD = if SW != 0.0 {
                    let SX = (KE * SO).exp();
                    SX
                } else {
                    let SY = ((SV * SO).exp()) * (C + ((KE - SV) * SO));
                    SY
                };
                let TA = if KE < SZ { 1.0 } else { 0.0 };
                let TE = if TA != 0.0 {
                    let TB = (KE * SR).exp();
                    TB
                } else {
                    let TC = ((SZ * SR).exp()) * (C + ((KE - SZ) * SR));
                    TC
                };
                let TF = (GX * (TD - C)) + (GY * (TE - C));
                UW = TF;
            } else {
                UW = A;
            }
            let TG = JZ / FI;
            let TH = if TG < Q { 1.0 } else { 0.0 };
            let TO = if TH != 0.0 {
                let TI = TG.exp();
                TI
            } else {
                let TJ = (Q.exp()) * (C + (TG - Q));
                TJ
            };
            let TK = KB / FI;
            let TL = if TK < Q { 1.0 } else { 0.0 };
            let TQ = if TL != 0.0 {
                let TM = TK.exp();
                TM
            } else {
                let TN = (Q.exp()) * (C + (TK - Q));
                TN
            };
            let TP = (C + (HU * TO)).sqrt();
            let TR = (C + (HU * TQ)).sqrt();
            let TS = (KF + (FI * ((TP - TR) - (((TP + C) / (TR + C)).ln())))) * IK;
            let TT = (JP * TS) / (IK * (C + (((BU * JP) * AC) * (((KF * KF) + E).sqrt()))));
            let TU = TS / ((C + (TT * TT)).sqrt());
            let TW = if parameters[83] > A { 1.0 } else { 0.0 };
            if TW != 0.0 {
                let TZ = (TX * (HC + C)).powf((C / (TY - HT)));
                let UA = (HO - JZ) - TZ;
                let UB = if ((-HC) * (((BU * ((((UA * UA) + E).sqrt()) + UA)) + TZ).powf((HT - C)))) < Q { 1.0 } else { 0.0 };
                if UB != 0.0 {
                } else {
                }
            } else {
            }
            let UC = if parameters[85] > A { 1.0 } else { 0.0 };
            if UC != 0.0 {
                let UE = (TX * (HD + C)).powf((C / (TY - UD)));
                let UF = (A - KC) - UE;
                let UG = if ((-HD) * (((BU * ((((UF * UF) + E).sqrt()) + UF)) + UE).powf((UD - C)))) < Q { 1.0 } else { 0.0 };
                if UG != 0.0 {
                } else {
                }
            } else {
            }
            let UH = if (if parameters[97] > A { 1.0 } else { 0.0 }) != 0.0 && (if parameters[95] > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            if UH != 0.0 {
                let UI = if parameters[94] > A { 1.0 } else { 0.0 };
                if UI != 0.0 {
                } else {
                }
            } else {
            }
            let UJ = if (if EN > A { 1.0 } else { 0.0 }) != 0.0 || (if EU > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            if UJ != 0.0 {
                let UL = if KG < UK { 1.0 } else { 0.0 };
                if UL != 0.0 {
                } else {
                }
                let UN = if KG < UM { 1.0 } else { 0.0 };
                if UN != 0.0 {
                } else {
                }
            } else {
            }
            let UZ = JQ * (UO + (UY * JV));
            let VA = JQ * (US + (UY * JX));
            let VB = JQ * OG;
            let VC = JQ * (UW + (UY * KE));
            let VD = JQ * TU;
            let VE = JQ * UX;
            let VF = if parameters[49] > A { 1.0 } else { 0.0 };
            if VF != 0.0 {
                let VG = (-HR) * KI;
                let VH = if parameters[52] <= A { 1.0 } else { 0.0 };
                if VH != 0.0 {
                    let VI = if (KG + VG) > A { 1.0 } else { 0.0 };
                    if VI != 0.0 {
                    } else {
                    }
                } else {
                }
            } else {
            }
            if KL != 0.0 {
                let VJ = if (JX + KJ) > A { 1.0 } else { 0.0 };
                if VJ != 0.0 {
                } else {
                }
            } else {
            }
            if LI != 0.0 {
                let VK = if (KE + LG) > A { 1.0 } else { 0.0 };
                if VK != 0.0 {
                } else {
                    let VL = if (if LP > A { 1.0 } else { 0.0 }) != 0.0 && (if KE < (-LP) { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    if VL != 0.0 {
                    } else {
                    }
                }
            } else {
                let VM = if (if LP > A { 1.0 } else { 0.0 }) != 0.0 && (if LZ > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                if VM != 0.0 {
                } else {
                }
            }
            let VN = if NE > A { 1.0 } else { 0.0 };
            if VN != 0.0 {
            } else {
            }
            let VO = if ((JZ * AG) / 1.44e0f64) < Q { 1.0 } else { 0.0 };
            if VO != 0.0 {
            } else {
            }
            let WM;
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
            let XQ;
            let XR;
            let XS;
            if VP != 0.0 {
                let VQ = UZ.abs();
                let VR = 3.204378e-19f64 * VQ;
                let VU = VS * (VQ.powf(VT));
                let VW = VA.abs();
                let VX = 3.204378e-19f64 * VW;
                let VY = VS * (VW.powf(VT));
                let VZ = 3.204378e-19f64 * (VB.abs());
                let WA = VC.abs();
                let WB = 3.204378e-19f64 * WA;
                let WC = VS * (WA.powf(VT));
                let WD = (5.522648e-23f64 * FH) * IG;
                let WF = (5.522648e-23f64 * FH) * (((VD.abs()) + (WE * IK)) / ((KF.abs()) + WE));
                let WG = (5.522648e-23f64 * FH) * IO;
                let WH = ((5.522648e-23f64 * FH) * OD) * IS;
                let WI = (5.522648e-23f64 * FH) * IV;
                let WJ = ((5.522648e-23f64 * FH) * TV) * IZ;
                let WK = 3.204378e-19f64 * (VE.abs());
                let WL = (5.522648e-23f64 * FH) * JC;
                WM = C;
                WN = VR;
                WO = C;
                WP = VU;
                WQ = VV;
                WR = C;
                WS = VX;
                WT = C;
                WU = VY;
                WV = VV;
                WW = C;
                WX = VZ;
                WY = C;
                WZ = WB;
                XA = C;
                XB = WC;
                XC = VV;
                XD = C;
                XE = WD;
                XF = C;
                XG = WF;
                XH = C;
                XI = WG;
                XJ = C;
                XK = WH;
                XL = C;
                XM = WI;
                XN = C;
                XO = WJ;
                XP = C;
                XQ = WK;
                XR = C;
                XS = WL;
            } else {
                WM = A;
                WN = A;
                WO = A;
                WP = A;
                WQ = A;
                WR = A;
                WS = A;
                WT = A;
                WU = A;
                WV = A;
                WW = A;
                WX = A;
                WY = A;
                WZ = A;
                XA = A;
                XB = A;
                XC = A;
                XD = A;
                XE = A;
                XF = A;
                XG = A;
                XH = A;
                XI = A;
                XJ = A;
                XK = A;
                XL = A;
                XM = A;
                XN = A;
                XO = A;
                XP = A;
                XQ = A;
                XR = A;
                XS = A;
            }
        if WM == 0.0 {
            if !visitor.visit(0, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = WN;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 0, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 0, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(0, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if WO == 0.0 {
            if !visitor.visit(1, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = WP;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = Some(WQ);
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(1, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if WR == 0.0 {
            if !visitor.visit(2, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = WS;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 2, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 2, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(2, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if WT == 0.0 {
            if !visitor.visit(3, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = WU;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 3, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = Some(WV);
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 3, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 3, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(3, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if WW == 0.0 {
            if !visitor.visit(4, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = WX;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 4, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 4, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(4, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if WY == 0.0 {
            if !visitor.visit(5, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = WZ;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 5, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 5, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(5, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if XA == 0.0 {
            if !visitor.visit(6, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = XB;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 6, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = Some(XC);
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 6, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 6, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(6, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if XD == 0.0 {
            if !visitor.visit(7, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = XE;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 7, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 7, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(7, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if XF == 0.0 {
            if !visitor.visit(8, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = XG;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 8, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 8, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(8, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if XH == 0.0 {
            if !visitor.visit(9, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = XI;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 9, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 9, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(9, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if XJ == 0.0 {
            if !visitor.visit(10, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = XK;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 10, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 10, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(10, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if XL == 0.0 {
            if !visitor.visit(11, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = XM;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 11, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 11, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(11, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if XN == 0.0 {
            if !visitor.visit(12, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = XO;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 12, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 12, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(12, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if XP == 0.0 {
            if !visitor.visit(13, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = XQ;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 13, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 13, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(13, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if XR == 0.0 {
            if !visitor.visit(14, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = XS;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 14, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 14, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(14, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        Ok(())
    }
}
