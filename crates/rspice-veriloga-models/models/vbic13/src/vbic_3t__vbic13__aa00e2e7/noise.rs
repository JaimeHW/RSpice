#![allow(dead_code, non_snake_case, unused_parens, unused_variables)]

use super::state::Instance;
use rspice_veriloga_runtime::GeneratedEvalContext;
pub use rspice_veriloga_runtime::{GeneratedNoiseDescriptor, GeneratedNoiseEndpoint, GeneratedNoiseEvaluation, GeneratedNoiseEvaluationError, GeneratedNoiseEvaluationRef, GeneratedNoiseKind, GeneratedNoiseVisitor};

pub static NOISE_SOURCES: [GeneratedNoiseDescriptor; 13] = [
    GeneratedNoiseDescriptor { mechanism: "WHITE_BI_EI_IBEI_SHOT_NOISE", label: Some("Ibei shot noise"), kind: GeneratedNoiseKind::White, equation: 27, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(7), name: "bi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(8), name: "ei", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_BI_EI_IBEI_FLICKER_NOISE", label: Some("Ibei flicker noise"), kind: GeneratedNoiseKind::Flicker, equation: 28, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(7), name: "bi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(8), name: "ei", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_BX_EI_IBEX_SHOT_NOISE", label: Some("Ibex shot noise"), kind: GeneratedNoiseKind::White, equation: 29, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(6), name: "bx", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(8), name: "ei", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_BX_EI_IBEX_FLICKER_NOISE", label: Some("Ibex flicker noise"), kind: GeneratedNoiseKind::Flicker, equation: 30, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(6), name: "bx", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(8), name: "ei", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_CI_EI_TRANSPORT_CURRENT_SHOT_NOISE", label: Some("transport current shot noise"), kind: GeneratedNoiseKind::White, equation: 31, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "ci", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(8), name: "ei", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_BX_BP_IBEP_SHOT_NOISE", label: Some("Ibep shot noise"), kind: GeneratedNoiseKind::White, equation: 32, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(6), name: "bx", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(9), name: "bp", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_BX_BP_IBEP_FLICKER_NOISE", label: Some("Ibep flicker noise"), kind: GeneratedNoiseKind::Flicker, equation: 33, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(6), name: "bx", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(9), name: "bp", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C_CX_RCX_THERMAL_NOISE", label: Some("rcx thermal noise"), kind: GeneratedNoiseKind::White, equation: 34, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(0), name: "c", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(4), name: "cx", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_CX_CI_RCI_THERMAL_NOISE", label: Some("rci thermal noise"), kind: GeneratedNoiseKind::White, equation: 35, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(4), name: "cx", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(5), name: "ci", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_B_BX_RBX_THERMAL_NOISE", label: Some("rbx thermal noise"), kind: GeneratedNoiseKind::White, equation: 36, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(1), name: "b", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(6), name: "bx", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_BX_BI_RBI_THERMAL_NOISE", label: Some("rbi thermal noise"), kind: GeneratedNoiseKind::White, equation: 37, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(6), name: "bx", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(7), name: "bi", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_E_EI_RE_THERMAL_NOISE", label: Some("re thermal noise"), kind: GeneratedNoiseKind::White, equation: 38, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(2), name: "e", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(8), name: "ei", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_BP_CX_RBP_THERMAL_NOISE", label: Some("rbp thermal noise"), kind: GeneratedNoiseKind::White, equation: 39, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(9), name: "bp", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(4), name: "cx", is_internal: true }, table_len: 0, table_log_interp: false },
];

impl Instance {
    pub fn evaluate_noise_sources(&self, ctx: &GeneratedEvalContext<'_>, visitor: &mut dyn GeneratedNoiseVisitor) -> Result<(), GeneratedNoiseEvaluationError> {
        if !self.multiplicity.is_finite() || self.multiplicity <= 0.0 {
            return Err(GeneratedNoiseEvaluationError::InvalidMultiplicity { value: self.multiplicity });
        }
        let parameters = &self.params.values;
        let parameter_given = &*self.param_given;
        let temperature = ctx.temperature();
        let node_potentials = [ctx.node_voltage(self.nodes[0]), ctx.node_voltage(self.nodes[1]), ctx.node_voltage(self.nodes[2]), ctx.node_voltage(self.nodes[3]), ctx.node_voltage(self.nodes[4]), ctx.node_voltage(self.nodes[5]), ctx.node_voltage(self.nodes[6]), ctx.node_voltage(self.nodes[7]), ctx.node_voltage(self.nodes[8]), ctx.node_voltage(self.nodes[9]), ctx.node_voltage(self.nodes[10]), ctx.node_voltage(self.nodes[11])];
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
            let EN = parameters[67];
            let EP = parameters[69];
            let FD = if parameter_given[109] { 1.0 } else { 0.0 };
            let FE = parameters[16];
            let FG = parameters[107];
            let FI = if parameter_given[108] { 1.0 } else { 0.0 };
            let FJ = parameters[17];
            let FM = if parameter_given[106] { 1.0 } else { 0.0 };
            let FN = parameters[21];
            let FP = parameters[104];
            let FR = if parameter_given[105] { 1.0 } else { 0.0 };
            let FS = parameters[22];
            let FX = if parameter_given[110] { 1.0 } else { 0.0 };
            let FY = parameters[25];
            let GV = 2e0f64;
            let GX = parameters[37];
            let HD = parameters[42];
            let HG = parameters[38];
            let HH = parameters[43];
            let HQ = 1e-3f64;
            let HT = 1e3f64;
            let JF = node_potentials[7];
            let JG = node_potentials[8];
            let JI = node_potentials[6];
            let JK = node_potentials[5];
            let JM = node_potentials[4];
            let JS = parameters[34];
            let JU = parameters[39];
            let KR = parameters[44];
            let KZ = parameters[45];
            let LJ = parameters[46];
            let MZ = 1e-4f64;
            let NB = 1e-8f64;
            let OB = parameters[32];
            let OH = 5.0005e-1f64;
            let OI = parameters[55];
            let OU = parameters[57];
            let TB = 2e-2f64;
            let TC = 1.01e0f64;
            let TH = parameters[87];
            let UI = parameters[1];
            let UL = parameters[98];
            let UM = parameters[99];
            let UO = parameters[100];
            let UX = 1e-10f64;
            if B != 0.0 {
            } else {
            }
            if D != 0.0 {
            } else {
            }
            let TW = if F != 0.0 {
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
            let JC;
            if L != 0.0 {
                JC = C;
            } else {
                let JD;
                if M != 0.0 {
                    JD = N;
                } else {
                    let JE = if O != 0.0 {
                        P
                    } else {
                        C
                    };
                    JD = JE;
                }
                JC = JD;
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
            let PD = if BE != 0.0 {
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
            let MI;
            if BR != 0.0 {
                let BT = if (if BS > A { 1.0 } else { 0.0 }) != 0.0 && (if BI > BS { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let MJ = if BT != 0.0 {
                    let BX = BP * ((C + ((((BU * BI) * ((BV / BS).powf(BW))).powf((C / (C - BW)))) / BQ)).ln());
                    BX
                } else {
                    let BY = BP * ((C + (BI / BQ)).ln());
                    BY
                };
                MI = MJ;
            } else {
                MI = A;
            }
            let CB = parameters[125] / CA;
            let CC = -parameters[121];
            let CD = BA * CA;
            let CE = (BZ * (BB.powf(CB))) * (((CC * BO) / CD).exp());
            let CF = if BR != 0.0 && (if CE > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let MQ;
            if CF != 0.0 {
                let CG = if S != 0.0 && (if BI > R { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let MR = if CG != 0.0 {
                    let CH = CD * ((C + ((((BU * BI) * ((BV / R).powf(BW))).powf((C / (C - BW)))) / (BQ * CE))).ln());
                    CH
                } else {
                    let CI = CD * ((C + (BI / (BQ * CE))).ln());
                    CI
                };
                MQ = MR;
            } else {
                MQ = A;
            }
            let CL = BK / CK;
            let CM = -parameters[120];
            let CN = BA * CK;
            let CO = (CJ * (BB.powf(CL))) * (((CM * BO) / CN).exp());
            let CP = if CO > A { 1.0 } else { 0.0 };
            let NT;
            if CP != 0.0 {
                let CQ = if W != 0.0 && (if BI > V { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let NU = if CQ != 0.0 {
                    let CR = CN * ((C + (((BI * BI) * Y) / CO)).ln());
                    CR
                } else {
                    let CS = CN * ((C + (BI / CO)).ln());
                    CS
                };
                NT = NU;
            } else {
                NT = A;
            }
            let CW = CU / CV;
            let CY = -CX;
            let CZ = BA * CV;
            let DA = (CT * (BB.powf(CW))) * (((CY * BO) / CZ).exp());
            let DB = if DA > A { 1.0 } else { 0.0 };
            let OL = if DB != 0.0 {
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
            let OQ = if DK != 0.0 {
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
            let RS = if DU != 0.0 {
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
            let RV = if ED != 0.0 {
                let EE = EA * ((C + (BI / EC)).ln());
                EE
            } else {
                A
            };
            let EG = (EF * DO) * DS;
            let EH = if EG > A { 1.0 } else { 0.0 };
            let RY = if EH != 0.0 {
                let EI = DR * ((C + (BI / EG)).ln());
                EI
            } else {
                A
            };
            let EK = (EJ * DY) * EB;
            let EL = if EK > A { 1.0 } else { 0.0 };
            let SC = if EL != 0.0 {
                let EM = EA * ((C + (BI / EK)).ln());
                EM
            } else {
                A
            };
            let EO = if ((parameters[66] * (BB.powf((CU / EN)))) * ((((-parameters[116]) * BO) / (BA * EN)).exp())) > A { 1.0 } else { 0.0 };
            if EO != 0.0 {
            } else {
            }
            let EQ = if ((parameters[68] * (BB.powf((DE / EP)))) * ((((-parameters[119]) * BO) / (BA * EP)).exp())) > A { 1.0 } else { 0.0 };
            if EQ != 0.0 {
            } else {
            }
            let ER = (AK + node_potentials[3]) - AI;
            let ES = if ER < AP { 1.0 } else { 0.0 };
            let EW;
            if ES != 0.0 {
                let ET = AO + (((ER - AO) - C).exp());
                EW = ET;
            } else {
                let EU = if ER > (AS - C) { 1.0 } else { 0.0 };
                let EX = if EU != 0.0 {
                    let EV = AS - (((AS - ER) - C).exp());
                    EV
                } else {
                    ER
                };
                EW = EX;
            }
            let EY = EW + AI;
            let EZ = (AY * EY) / AZ;
            let FA = EY / AJ;
            let FB = EY - AJ;
            let FC = BS * (FA.powf(parameters[126]));
            let HP = if FD != 0.0 {
                let FF = FE * (FA.powf(parameters[109]));
                FF
            } else {
                let FH = FE * (FA.powf(FG));
                FH
            };
            let HV = if FI != 0.0 {
                let FK = FJ * (FA.powf(parameters[108]));
                FK
            } else {
                let FL = FJ * (FA.powf(FG));
                FL
            };
            let HZ = if FM != 0.0 {
                let FO = FN * (FA.powf(parameters[106]));
                FO
            } else {
                let FQ = FN * (FA.powf(FP));
                FQ
            };
            let ID = if FR != 0.0 {
                let FT = FS * (FA.powf(parameters[105]));
                FT
            } else {
                let FU = FS * (FA.powf(FP));
                FU
            };
            let FV = parameters[23] * (FA.powf(parameters[103]));
            let FW = parameters[24] * (FA.powf(parameters[111]));
            let IK = if FX != 0.0 {
                let FZ = FY * (FA.powf(parameters[110]));
                FZ
            } else {
                let GA = FY * (FA.powf(FG));
                GA
            };
            let GB = parameters[101] * (C + (FB * parameters[132]));
            let GC = C - FA;
            let GD = BN * GC;
            let GE = (BC * (FA.powf(BM))) * ((GD / (EZ * BL)).exp());
            let GF = (BZ * (FA.powf(CB))) * (((CC * GC) / (EZ * CA)).exp());
            let GG = EZ * CK;
            let GH = (CJ * (FA.powf(CL))) * (((CM * GC) / GG).exp());
            let GI = EZ * CV;
            let GJ = (CT * (FA.powf(CW))) * (((CY * GC) / GI).exp());
            let GK = EZ * DF;
            let GL = (DD * (FA.powf(DG))) * (((DH * GC) / GK).exp());
            let GM = EZ * DM;
            let GN = EZ * DW;
            let GO = (EF * (FA.powf(DN))) * (((DQ * GC) / GM).exp());
            let GP = (EJ * (FA.powf(DX))) * (((DZ * GC) / GN).exp());
            let GQ = C + (FB * parameters[129]);
            let GR = BL * GQ;
            let GS = CA * GQ;
            let GT = parameters[84] * (C + (FB * parameters[127]));
            let GU = parameters[86] * (C + (FB * parameters[128]));
            let GW = GV * (EZ / FA);
            let GY = (3e0f64 * EZ) * (FA.ln());
            let GZ = FA - C;
            let HA = (((GW * ((((((BU * GX) * FA) / EZ).exp()) - ((((-5e-1f64 * GX) * FA) / EZ).exp())).ln())) * FA) - GY) - (CX * GZ);
            let HB = GV * EZ;
            let HC = HA + (HB * ((BU * (C + ((C + (BV * (((-HA) / EZ).exp()))).sqrt()))).ln()));
            let HE = (((GW * ((((((BU * HD) * FA) / EZ).exp()) - ((((-5e-1f64 * HD) * FA) / EZ).exp())).ln())) * FA) - GY) - (DP * GZ);
            let HF = HE + (HB * ((BU * (C + ((C + (BV * (((-HE) / EZ).exp()))).sqrt()))).ln()));
            let HI = (parameters[19] * (FA.powf(BK))) * ((GD / EZ).exp());
            let HJ = parameters[18] * (FA.powf(parameters[112]));
            let HK = -(BH * (C + (FB * (parameters[91] + (FB * parameters[92])))));
            let HL = (BF * (C + (FB * parameters[93]))) * EZ;
            let HM = (HK / HL).exp();
            let HN = parameters[70] * (C + (FB * parameters[130]));
            let HO = parameters[71] * (C + (FB * parameters[131]));
            let HR = if HP > HQ { 1.0 } else { 0.0 };
            let HU = if HR != 0.0 {
                let HS = C / HP;
                HS
            } else {
                HT
            };
            let HW = if HV > HQ { 1.0 } else { 0.0 };
            let HY = if HW != 0.0 {
                let HX = C / HV;
                HX
            } else {
                HT
            };
            let IA = if HZ > HQ { 1.0 } else { 0.0 };
            let IC = if IA != 0.0 {
                let IB = C / HZ;
                IB
            } else {
                HT
            };
            let IE = if ID > HQ { 1.0 } else { 0.0 };
            let IG = if IE != 0.0 {
                let IF = C / ID;
                IF
            } else {
                HT
            };
            let IH = if FV > HQ { 1.0 } else { 0.0 };
            let IJ = if IH != 0.0 {
                let II = C / FV;
                II
            } else {
                HT
            };
            let IL = if IK > HQ { 1.0 } else { 0.0 };
            let IN = if IL != 0.0 {
                let IM = C / IK;
                IM
            } else {
                HT
            };
            let IO = if FW > HQ { 1.0 } else { 0.0 };
            if IO != 0.0 {
            } else {
            }
            let IP = if GB > HQ { 1.0 } else { 0.0 };
            if IP != 0.0 {
            } else {
            }
            let IQ = if HN > A { 1.0 } else { 0.0 };
            let IS = if IQ != 0.0 {
                let IR = C / HN;
                IR
            } else {
                A
            };
            let IT = if HO > A { 1.0 } else { 0.0 };
            let IV = if IT != 0.0 {
                let IU = C / HO;
                IU
            } else {
                A
            };
            let IW = if FC > A { 1.0 } else { 0.0 };
            let IY = if IW != 0.0 {
                let IX = C / FC;
                IX
            } else {
                A
            };
            let IZ = if HJ > A { 1.0 } else { 0.0 };
            let JB = if IZ != 0.0 {
                let JA = C / HJ;
                JA
            } else {
                A
            };
            let JH = JC * (JF - JG);
            let JJ = JC * (JI - JG);
            let JL = JC * (JF - JK);
            let JN = JC * (JF - JM);
            let JO = JC * (JI - JM);
            let JP = JC * (JI - node_potentials[9]);
            let JQ = JC * (JM - JK);
            let JR = -HC;
            let JT = JR * JS;
            let JV = if JU <= A { 1.0 } else { 0.0 };
            let MW;
            if JV != 0.0 {
                let JW = JH + JT;
                let JX = if JW > A { 1.0 } else { 0.0 };
                let KE;
                let KF;
                if JX != 0.0 {
                    let JY = C - JS;
                    let JZ = JY.powf((-HG));
                    let KA = (HC * (C - (JZ * JY))) / (C - HG);
                    let KB = (JW * (C + (((BU * HG) * JW) / (HC * JY)))) * JZ;
                    KE = KA;
                    KF = KB;
                } else {
                    let KC = C - HG;
                    let KD = (HC * (C - ((C - (JH / HC)).powf(KC)))) / KC;
                    KE = KD;
                    KF = A;
                }
                let KG = KE + KF;
                MW = KG;
            } else {
                let KH = (BV * JU) * JU;
                let KI = -5e-1f64 * (JT + (((JT * JT) + KH).sqrt()));
                let KJ = C - HG;
                let KK = JH + JT;
                let KL = (BU * (KK - (((KK * KK) + KH).sqrt()))) - JT;
                let KM = C - JS;
                let KN = (JH - KL) + KI;
                let KO = (((JR * ((C - (KL / HC)).powf(KJ))) / KJ) + (((KM.powf((-HG))) * KN) * (C + (((BU * HG) * KN) / (HC * KM))))) - ((JR * ((C - (KI / HC)).powf(KJ))) / KJ);
                MW = KO;
            }
            let KP = -HF;
            let KQ = KP * JS;
            let KS = if KR <= A { 1.0 } else { 0.0 };
            let MX;
            if KS != 0.0 {
                let KT = JL + KQ;
                let KU = if KT > A { 1.0 } else { 0.0 };
                let LF;
                let LH;
                if KU != 0.0 {
                    let KV = C - JS;
                    let KW = KV.powf((-1e0f64 - HH));
                    let KX = (HF * (C - ((KW * KV) * KV))) / (C - HH);
                    let KY = (KT * (KV + (((BU * HH) * KT) / HF))) * KW;
                    LF = KX;
                    LH = KY;
                } else {
                    let LA = if (if KZ > A { 1.0 } else { 0.0 }) != 0.0 && (if JL < (-KZ) { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let LG = if LA != 0.0 {
                        let LB = C - HH;
                        let LC = (HF * (C - (((C + (KZ / HF)).powf(LB)) * (C - ((LB * (JL + KZ)) / (HF + KZ)))))) / LB;
                        LC
                    } else {
                        let LD = C - HH;
                        let LE = (HF * (C - ((C - (JL / HF)).powf(LD)))) / LD;
                        LE
                    };
                    LF = LG;
                    LH = A;
                }
                let LI = LF + LH;
                MX = LI;
            } else {
                let LK = if (if KZ > A { 1.0 } else { 0.0 }) != 0.0 && (if LJ > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let MY = if LK != 0.0 {
                    let LL = KZ - KQ;
                    let LM = (KZ + KQ) / LL;
                    let LN = LM - C;
                    let LO = (BV * KR) * KR;
                    let LP = LM + C;
                    let LQ = (BV * LJ) * LJ;
                    let LR = BU * (((((GV * LM) / ((((LN * LN) + LO).sqrt()) + (((LP * LP) + LQ).sqrt()))) * LL) - KZ) - KQ);
                    let LS = C - HH;
                    let LT = (((GV * JL) + KZ) + KQ) / LL;
                    let LU = LT - C;
                    let LV = LT + C;
                    let LW = (GV * LT) / ((((LU * LU) + LO).sqrt()) + (((LV * LV) + LQ).sqrt()));
                    let LX = BU * (((LW * LL) - KZ) - KQ);
                    let LY = BU * (LW + C);
                    let LZ = -HH;
                    let MA = ((((JL - LX) + LR) * (((C - LY) * ((C + (KZ / HF)).powf(LZ))) + (LY * ((C + (KQ / HF)).powf(LZ))))) + ((HF * (C - ((C - (LX / HF)).powf(LS)))) / LS)) - ((HF * (C - ((C - (LR / HF)).powf(LS)))) / LS);
                    MA
                } else {
                    let MB = (BV * KR) * KR;
                    let MC = -5e-1f64 * (KQ + (((KQ * KQ) + MB).sqrt()));
                    let MD = C - HH;
                    let ME = JL + KQ;
                    let MF = (BU * (ME - (((ME * ME) + MB).sqrt()))) - KQ;
                    let MG = (((KP * ((C - (MF / HF)).powf(MD))) / MD) + (((C - JS).powf((-HH))) * ((JL - MF) + MC))) - ((KP * ((C - (MC / HF)).powf(MD))) / MD);
                    MG
                };
                MX = MY;
            }
            let MH = C / (GR * EZ);
            let MK = if JH < MI { 1.0 } else { 0.0 };
            let MN = if MK != 0.0 {
                let ML = (JH * MH).exp();
                ML
            } else {
                let MM = ((MI * MH).exp()) * (C + ((JH - MI) * MH));
                MM
            };
            let MO = GE * (MN - C);
            let MP = C / (GS * EZ);
            let MS = if JL < MQ { 1.0 } else { 0.0 };
            let MV = if MS != 0.0 {
                let MT = (JL * MP).exp();
                MT
            } else {
                let MU = ((MQ * MP).exp()) * (C + ((JL - MQ) * MP));
                MU
            };
            let NA = ((C + (MW * IV)) + (MX * IS)) - MZ;
            let NC = (BU * ((((NA * NA) + NB).sqrt()) + NA)) + MZ;
            let ND = (MO * IY) + (((GE * GF) * (MV - C)) * U);
            let NE = if parameters[30] < BU { 1.0 } else { 0.0 };
            let NN;
            if NE != 0.0 {
                let NF = (NC.powf((C / BW))) + (BV * ND);
                let NG = if NF > NB { 1.0 } else { 0.0 };
                let NO = if NG != 0.0 {
                    let NH = BU * (NC + (NF.powf(BW)));
                    NH
                } else {
                    let NI = BU * (NC + (NB.powf(BW)));
                    NI
                };
                NN = NO;
            } else {
                let NJ = C + (BV * ND);
                let NK = if NJ > NB { 1.0 } else { 0.0 };
                let NP = if NK != 0.0 {
                    let NL = (BU * NC) * (C + (NJ.powf(BW)));
                    NL
                } else {
                    let NM = (BU * NC) * (C + (NB.powf(BW)));
                    NM
                };
                NN = NP;
            }
            let NQ = MO / NN;
            let NR = if CJ > A { 1.0 } else { 0.0 };
            let SY;
            if NR != 0.0 {
                let NS = C / GG;
                let NV = if JP < NT { 1.0 } else { 0.0 };
                let OC = if NV != 0.0 {
                    let NW = (JP * NS).exp();
                    NW
                } else {
                    let NX = ((NT * NS).exp()) * (C + ((JP - NT) * NS));
                    NX
                };
                let NY = if JL < NT { 1.0 } else { 0.0 };
                let OD = if NY != 0.0 {
                    let NZ = (JL * NS).exp();
                    NZ
                } else {
                    let OA = ((NT * NS).exp()) * (C + ((JL - NT) * NS));
                    OA
                };
                let OE = C + (BV * ((GH * (((OB * OC) + ((C - OB) * OD)) - C)) * Y));
                let OF = if OE > NB { 1.0 } else { 0.0 };
                let SZ = if OF != 0.0 {
                    let OG = BU * (C + (OE.sqrt()));
                    OG
                } else {
                    OH
                };
                SY = SZ;
            } else {
                SY = C;
            }
            let OJ = if OI == C { 1.0 } else { 0.0 };
            let TN;
            let TR;
            if OJ != 0.0 {
                let OK = C / GI;
                let OM = if JH < OL { 1.0 } else { 0.0 };
                let OW = if OM != 0.0 {
                    let ON = (JH * OK).exp();
                    ON
                } else {
                    let OO = ((OL * OK).exp()) * (C + ((JH - OL) * OK));
                    OO
                };
                let OP = C / GK;
                let OR = if JH < OQ { 1.0 } else { 0.0 };
                let OX = if OR != 0.0 {
                    let OS = (JH * OP).exp();
                    OS
                } else {
                    let OT = ((OQ * OP).exp()) * (C + ((JH - OQ) * OP));
                    OT
                };
                let OV = if OU > A { 1.0 } else { 0.0 };
                let PH = if OV != 0.0 {
                    let OY = ((GJ * (C + (OU * (NC - C)))) * (OW - C)) + (GL * (OX - C));
                    OY
                } else {
                    let OZ = (GJ * (OW - C)) + (GL * (OX - C));
                    OZ
                };
                let PA = if BH > A { 1.0 } else { 0.0 };
                let TO;
                if PA != 0.0 {
                    let PB = HK - JH;
                    let PC = C / HL;
                    let PE = if PB < PD { 1.0 } else { 0.0 };
                    let PI = if PE != 0.0 {
                        let PF = (PB * PC).exp();
                        PF
                    } else {
                        let PG = ((PD * PC).exp()) * (C + ((PB - PD) * PC));
                        PG
                    };
                    let PJ = PH - (BD * (PI - HM));
                    TO = PJ;
                } else {
                    TO = PH;
                }
                TN = TO;
                TR = A;
            } else {
                let PK = if OI == A { 1.0 } else { 0.0 };
                let TP;
                let TS;
                if PK != 0.0 {
                    let PL = C / GI;
                    let PM = if JJ < OL { 1.0 } else { 0.0 };
                    let PT = if PM != 0.0 {
                        let PN = (JJ * PL).exp();
                        PN
                    } else {
                        let PO = ((OL * PL).exp()) * (C + ((JJ - OL) * PL));
                        PO
                    };
                    let PP = C / GK;
                    let PQ = if JJ < OQ { 1.0 } else { 0.0 };
                    let PU = if PQ != 0.0 {
                        let PR = (JJ * PP).exp();
                        PR
                    } else {
                        let PS = ((OQ * PP).exp()) * (C + ((JJ - OQ) * PP));
                        PS
                    };
                    let PV = (GJ * (PT - C)) + (GL * (PU - C));
                    let PW = if BH > A { 1.0 } else { 0.0 };
                    let TT;
                    if PW != 0.0 {
                        let PX = HK - JH;
                        let PY = C / HL;
                        let PZ = if PX < PD { 1.0 } else { 0.0 };
                        let QC = if PZ != 0.0 {
                            let QA = (PX * PY).exp();
                            QA
                        } else {
                            let QB = ((PD * PY).exp()) * (C + ((PX - PD) * PY));
                            QB
                        };
                        let QD = PV - (BD * (QC - HM));
                        TT = QD;
                    } else {
                        TT = PV;
                    }
                    TP = A;
                    TS = TT;
                } else {
                    let QE = C / GI;
                    let QF = if JH < OL { 1.0 } else { 0.0 };
                    let QN = if QF != 0.0 {
                        let QG = (JH * QE).exp();
                        QG
                    } else {
                        let QH = ((OL * QE).exp()) * (C + ((JH - OL) * QE));
                        QH
                    };
                    let QI = C / GK;
                    let QJ = if JH < OQ { 1.0 } else { 0.0 };
                    let QO = if QJ != 0.0 {
                        let QK = (JH * QI).exp();
                        QK
                    } else {
                        let QL = ((OQ * QI).exp()) * (C + ((JH - OQ) * QI));
                        QL
                    };
                    let QM = if OU > A { 1.0 } else { 0.0 };
                    let QX = if QM != 0.0 {
                        let QP = OI * (((GJ * (C + (OU * (NC - C)))) * (QN - C)) + (GL * (QO - C)));
                        QP
                    } else {
                        let QQ = OI * ((GJ * (QN - C)) + (GL * (QO - C)));
                        QQ
                    };
                    let QR = if BH > A { 1.0 } else { 0.0 };
                    let TQ;
                    if QR != 0.0 {
                        let QS = HK - JH;
                        let QT = C / HL;
                        let QU = if QS < PD { 1.0 } else { 0.0 };
                        let QY = if QU != 0.0 {
                            let QV = (QS * QT).exp();
                            QV
                        } else {
                            let QW = ((PD * QT).exp()) * (C + ((QS - PD) * QT));
                            QW
                        };
                        let QZ = QX - ((OI * BD) * (QY - HM));
                        TQ = QZ;
                    } else {
                        TQ = QX;
                    }
                    let RA = if JJ < OL { 1.0 } else { 0.0 };
                    let RH = if RA != 0.0 {
                        let RB = (JJ * QE).exp();
                        RB
                    } else {
                        let RC = ((OL * QE).exp()) * (C + ((JJ - OL) * QE));
                        RC
                    };
                    let RD = if JJ < OQ { 1.0 } else { 0.0 };
                    let RI = if RD != 0.0 {
                        let RE = (JJ * QI).exp();
                        RE
                    } else {
                        let RF = ((OQ * QI).exp()) * (C + ((JJ - OQ) * QI));
                        RF
                    };
                    let RG = C - OI;
                    let RJ = RG * ((GJ * (RH - C)) + (GL * (RI - C)));
                    let TU;
                    if QR != 0.0 {
                        let RK = HK - JH;
                        let RL = C / HL;
                        let RM = if RK < PD { 1.0 } else { 0.0 };
                        let RP = if RM != 0.0 {
                            let RN = (RK * RL).exp();
                            RN
                        } else {
                            let RO = ((PD * RL).exp()) * (C + ((RK - PD) * RL));
                            RO
                        };
                        let RQ = RJ - ((RG * BD) * (RP - HM));
                        TU = RQ;
                    } else {
                        TU = RJ;
                    }
                    TP = TQ;
                    TS = TU;
                }
                TN = TP;
                TR = TS;
            }
            let RR = C / GM;
            let RT = if JL < RS { 1.0 } else { 0.0 };
            if RT != 0.0 {
            } else {
            }
            let RU = C / GN;
            let RW = if JL < RV { 1.0 } else { 0.0 };
            if RW != 0.0 {
            } else {
            }
            let RX = if (if EF > A { 1.0 } else { 0.0 }) != 0.0 || (if EJ > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let TV;
            if RX != 0.0 {
                let RZ = if JP < RY { 1.0 } else { 0.0 };
                let SG = if RZ != 0.0 {
                    let SA = (JP * RR).exp();
                    SA
                } else {
                    let SB = ((RY * RR).exp()) * (C + ((JP - RY) * RR));
                    SB
                };
                let SD = if JP < SC { 1.0 } else { 0.0 };
                let SH = if SD != 0.0 {
                    let SE = (JP * RU).exp();
                    SE
                } else {
                    let SF = ((SC * RU).exp()) * (C + ((JP - SC) * RU));
                    SF
                };
                let SI = (GO * (SG - C)) + (GP * (SH - C));
                TV = SI;
            } else {
                TV = A;
            }
            let SJ = JL / EZ;
            let SK = if SJ < Q { 1.0 } else { 0.0 };
            let SR = if SK != 0.0 {
                let SL = SJ.exp();
                SL
            } else {
                let SM = (Q.exp()) * (C + (SJ - Q));
                SM
            };
            let SN = JN / EZ;
            let SO = if SN < Q { 1.0 } else { 0.0 };
            let ST = if SO != 0.0 {
                let SP = SN.exp();
                SP
            } else {
                let SQ = (Q.exp()) * (C + (SN - Q));
                SQ
            };
            let SS = (C + (HI * SR)).sqrt();
            let SU = (C + (HI * ST)).sqrt();
            let SV = (JQ + (EZ * ((SS - SU) - (((SS + C) / (SU + C)).ln())))) * HY;
            let SW = (JB * SV) / (HY * (C + (((BU * JB) * AC) * (((JQ * JQ) + E).sqrt()))));
            let SX = SV / ((C + (SW * SW)).sqrt());
            let TA = if parameters[83] > A { 1.0 } else { 0.0 };
            if TA != 0.0 {
                let TD = (TB * (GT + C)).powf((C / (TC - HH)));
                let TE = (HF - JL) - TD;
                let TF = if ((-GT) * (((BU * ((((TE * TE) + E).sqrt()) + TE)) + TD).powf((HH - C)))) < Q { 1.0 } else { 0.0 };
                if TF != 0.0 {
                } else {
                }
            } else {
            }
            let TG = if parameters[85] > A { 1.0 } else { 0.0 };
            if TG != 0.0 {
                let TI = (TB * (GU + C)).powf((C / (TC - TH)));
                let TJ = (A - JO) - TI;
                let TK = if ((-GU) * (((BU * ((((TJ * TJ) + E).sqrt()) + TJ)) + TI).powf((TH - C)))) < Q { 1.0 } else { 0.0 };
                if TK != 0.0 {
                } else {
                }
            } else {
            }
            let TL = if (if parameters[97] > A { 1.0 } else { 0.0 }) != 0.0 && (if parameters[95] > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            if TL != 0.0 {
                let TM = if parameters[94] > A { 1.0 } else { 0.0 };
                if TM != 0.0 {
                } else {
                }
            } else {
            }
            let TX = JC * (TN + (TW * JH));
            let TY = JC * (TR + (TW * JJ));
            let TZ = JC * NQ;
            let UA = JC * (TV + (TW * JP));
            let UB = JC * SX;
            if JV != 0.0 {
                let UC = if (JJ + JT) > A { 1.0 } else { 0.0 };
                if UC != 0.0 {
                } else {
                }
            } else {
            }
            if KS != 0.0 {
                let UD = if (JP + KQ) > A { 1.0 } else { 0.0 };
                if UD != 0.0 {
                } else {
                    let UE = if (if KZ > A { 1.0 } else { 0.0 }) != 0.0 && (if JP < (-KZ) { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    if UE != 0.0 {
                    } else {
                    }
                }
            } else {
                let UF = if (if KZ > A { 1.0 } else { 0.0 }) != 0.0 && (if LJ > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                if UF != 0.0 {
                } else {
                }
            }
            let UG = if MO > A { 1.0 } else { 0.0 };
            if UG != 0.0 {
            } else {
            }
            let UH = if ((JL * AG) / 1.44e0f64) < Q { 1.0 } else { 0.0 };
            if UH != 0.0 {
            } else {
            }
            let VD;
            let VE;
            let VF;
            let VG;
            let VH;
            let VI;
            let VJ;
            let VK;
            let VL;
            let VM;
            let VN;
            let VO;
            let VP;
            let VQ;
            let VR;
            let VS;
            let VT;
            let VU;
            let VV;
            let VW;
            let VX;
            let VY;
            let VZ;
            let WA;
            let WB;
            let WC;
            let WD;
            let WE;
            let WF;
            if UI != 0.0 {
                let UJ = TX.abs();
                let UK = 3.204378e-19f64 * UJ;
                let UN = UL * (UJ.powf(UM));
                let UP = TY.abs();
                let UQ = 3.204378e-19f64 * UP;
                let UR = UL * (UP.powf(UM));
                let US = 3.204378e-19f64 * (TZ.abs());
                let UT = UA.abs();
                let UU = 3.204378e-19f64 * UT;
                let UV = UL * (UT.powf(UM));
                let UW = (5.522648e-23f64 * EY) * HU;
                let UY = (5.522648e-23f64 * EY) * (((UB.abs()) + (UX * HY)) / ((JQ.abs()) + UX));
                let UZ = (5.522648e-23f64 * EY) * IC;
                let VA = ((5.522648e-23f64 * EY) * NN) * IG;
                let VB = (5.522648e-23f64 * EY) * IJ;
                let VC = ((5.522648e-23f64 * EY) * SY) * IN;
                VD = C;
                VE = UK;
                VF = C;
                VG = UN;
                VH = UO;
                VI = C;
                VJ = UQ;
                VK = C;
                VL = UR;
                VM = UO;
                VN = C;
                VO = US;
                VP = C;
                VQ = UU;
                VR = C;
                VS = UV;
                VT = UO;
                VU = C;
                VV = UW;
                VW = C;
                VX = UY;
                VY = C;
                VZ = UZ;
                WA = C;
                WB = VA;
                WC = C;
                WD = VB;
                WE = C;
                WF = VC;
            } else {
                VD = A;
                VE = A;
                VF = A;
                VG = A;
                VH = A;
                VI = A;
                VJ = A;
                VK = A;
                VL = A;
                VM = A;
                VN = A;
                VO = A;
                VP = A;
                VQ = A;
                VR = A;
                VS = A;
                VT = A;
                VU = A;
                VV = A;
                VW = A;
                VX = A;
                VY = A;
                VZ = A;
                WA = A;
                WB = A;
                WC = A;
                WD = A;
                WE = A;
                WF = A;
            }
        if VD == 0.0 {
            if !visitor.visit(0, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = VE;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 0, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 0, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(0, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if VF == 0.0 {
            if !visitor.visit(1, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = VG;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = Some(VH);
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(1, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if VI == 0.0 {
            if !visitor.visit(2, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = VJ;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 2, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 2, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(2, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if VK == 0.0 {
            if !visitor.visit(3, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = VL;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 3, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = Some(VM);
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 3, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 3, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(3, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if VN == 0.0 {
            if !visitor.visit(4, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = VO;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 4, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 4, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(4, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if VP == 0.0 {
            if !visitor.visit(5, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = VQ;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 5, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 5, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(5, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if VR == 0.0 {
            if !visitor.visit(6, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = VS;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 6, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = Some(VT);
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 6, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 6, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(6, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if VU == 0.0 {
            if !visitor.visit(7, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = VV;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 7, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 7, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(7, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if VW == 0.0 {
            if !visitor.visit(8, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = VX;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 8, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 8, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(8, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if VY == 0.0 {
            if !visitor.visit(9, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = VZ;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 9, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 9, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(9, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if WA == 0.0 {
            if !visitor.visit(10, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = WB;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 10, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 10, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(10, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if WC == 0.0 {
            if !visitor.visit(11, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = WD;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 11, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 11, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(11, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if WE == 0.0 {
            if !visitor.visit(12, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = WF;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 12, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 12, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(12, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        Ok(())
    }
}
