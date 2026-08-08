#![allow(dead_code, non_snake_case, unused_parens, unused_variables)]

use super::state::Instance;
use rspice_veriloga_runtime::GeneratedEvalContext;
pub use rspice_veriloga_runtime::{GeneratedNoiseDescriptor, GeneratedNoiseEndpoint, GeneratedNoiseEvaluation, GeneratedNoiseEvaluationError, GeneratedNoiseEvaluationRef, GeneratedNoiseKind, GeneratedNoiseVisitor};

pub static NOISE_SOURCES: [GeneratedNoiseDescriptor; 7] = [
    GeneratedNoiseDescriptor { mechanism: "WHITE_GI_CI_IGC", label: Some("Igc"), kind: GeneratedNoiseKind::White, equation: 15, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(4), name: "gi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(5), name: "ci", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_GI_BI_IGOV", label: Some("Igov"), kind: GeneratedNoiseKind::White, equation: 16, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(4), name: "gi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(1), name: "bi", is_internal: false }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_G_GII_RGSAL", label: Some("rgsal"), kind: GeneratedNoiseKind::White, equation: 17, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(0), name: "g", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(3), name: "gii", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_GII_GI_RGPV", label: Some("rgpv"), kind: GeneratedNoiseKind::White, equation: 18, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(3), name: "gii", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(4), name: "gi", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_BI_B_REND", label: Some("rend"), kind: GeneratedNoiseKind::White, equation: 19, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(1), name: "bi", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(2), name: "b", is_internal: false }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_CI_BI_RSUB", label: Some("rsub"), kind: GeneratedNoiseKind::White, equation: 20, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "ci", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(1), name: "bi", is_internal: false }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_CI_BI_RAC", label: Some("rac"), kind: GeneratedNoiseKind::White, equation: 21, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "ci", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(1), name: "bi", is_internal: false }, table_len: 0, table_log_interp: false },
];

impl Instance {
    pub fn evaluate_noise_sources(&self, ctx: &GeneratedEvalContext<'_>, visitor: &mut dyn GeneratedNoiseVisitor) -> Result<(), GeneratedNoiseEvaluationError> {
        if !self.multiplicity.is_finite() || self.multiplicity <= 0.0 {
            return Err(GeneratedNoiseEvaluationError::InvalidMultiplicity { value: self.multiplicity });
        }
        let parameters = &self.params.values;
        let temperature = ctx.temperature();
        let node_potentials = [ctx.node_voltage(self.nodes[0]), ctx.node_voltage(self.nodes[1]), ctx.node_voltage(self.nodes[2]), ctx.node_voltage(self.nodes[3]), ctx.node_voltage(self.nodes[4]), ctx.node_voltage(self.nodes[5]), ctx.node_voltage(self.nodes[6])];
            let A = 0e0f64;
            let B = 1e3f64;
            let D = parameters[19];
            let F = 2e0f64;
            let G = parameters[24];
            let H = parameters[29];
            let K = parameters[30];
            let M = 6.666666666666666e-1f64;
            let O = parameters[17];
            let S = 3.333333333333333e-1f64;
            let T = parameters[48];
            let V = 5e-1f64;
            let Y = parameters[11];
            let AA = -2.73e2f64;
            let AC = parameters[8];
            let AE = parameters[9];
            let AG = 2.7315e2f64;
            let AQ = 1e2f64;
            let AS = 1e0f64;
            let BA = 4e0f64;
            let BC = parameters[1];
            let BD = parameters[0];
            let BN = 1e-3f64;
            let BS = 6e0f64;
            let BY = 7.071067811865475e-1f64;
            let CB = 1e-5f64;
            let CI = 1.25e0f64;
            let CJ = 4.6051701859880916e2f64;
            let CM = 1e-200f64;
            let CP = parameters[16];
            let CQ = 3e0f64;
            let CV = 1.2e1f64;
            let DO = 2e1f64;
            let EF = parameters[66];
            let EJ = parameters[49];
            let EK = 1e12f64;
            let EL = parameters[53];
            let ER = parameters[50];
            let ET = parameters[51];
            let EV = 1.05457168e-34f64;
            let EY = parameters[59];
            let FA = parameters[58];
            let FC = parameters[64];
            let FE = parameters[63];
            let FK = 1e-1f64;
            let FL = node_potentials[4];
            let FO = 1e-16f64;
            let FQ = parameters[28];
            let FV = 1e-32f64;
            let GA = parameters[25];
            let GD = 1e-6f64;
            let HN = 1.666666666666667e-1f64;
            let HT = 1e1f64;
            let HV = 6.4e1f64;
            let IE = 2.3025850929940458e2f64;
            let IH = 1e100f64;
            let IQ = 7.324648775608221e-1f64;
            let IV = 1e-100f64;
            let IZ = 2.5e-1f64;
            let JE = 5e0f64;
            let JP = 1e-40f64;
            let JV = 1e-120f64;
            let KV = parameters[18];
            let RE = node_potentials[6];
            let ZP = 1e-2f64;
            let ADH = 1.0f64;
            let AFE = 1.0f64;
            let AHE = 0.0f64;
            let AIU = 0.0f64;
            let C = if parameters[7] != B { 1.0 } else { 0.0 };
            if C != 0.0 {
            } else {
            }
            let E = (3.453e-11f64 * (parameters[20] / 3.9e0f64)) / D;
            let I = ((3.348580862e-29f64 * H).sqrt()) / E;
            let J = ((3.348580862e-29f64 * parameters[54]).sqrt()) / E;
            let L = if K > A { 1.0 } else { 0.0 };
            let GQ;
            if L != 0.0 {
                let N = (2.3807972e0f64 * K) * (E.powf(M));
                let P = if O < A { 1.0 } else { 0.0 };
                let GR = if P != 0.0 {
                    let Q = 1.2514650134837189e0f64 * N;
                    Q
                } else {
                    N
                };
                GQ = GR;
            } else {
                GQ = A;
            }
            let R = if O < A { 1.0 } else { 0.0 };
            let YZ = if R != 0.0 {
                let U = S * T;
                U
            } else {
                let W = V * T;
                W
            };
            let X = D / 1e-9f64;
            let Z = if Y > -2.73e2f64 { 1.0 } else { 0.0 };
            let AB = if Z != 0.0 {
                Y
            } else {
                AA
            };
            let AD = if AB < AC { 1.0 } else { 0.0 };
            if AD != 0.0 {
            } else {
            }
            let AF = if AB > AE { 1.0 } else { 0.0 };
            if AF != 0.0 {
            } else {
            }
            let AH = AG + AB;
            let AI = (temperature + parameters[3]) - AG;
            let AJ = if AI < AC { 1.0 } else { 0.0 };
            if AJ != 0.0 {
            } else {
            }
            let AK = if AI > AE { 1.0 } else { 0.0 };
            if AK != 0.0 {
            } else {
            }
            let AL = AI + AG;
            let AM = AL * AL;
            let AN = AL / AH;
            let AO = AH / AL;
            let AP = (AL * 1.3806505e-23f64) / 1.6021918e-19f64;
            let AR = (AQ * AP) * AP;
            let AT = AS / AP;
            let AU = parameters[23] + ((AL - AH) * parameters[42]);
            let AV = parameters[36] * (AO.powf(parameters[43]));
            let AW = parameters[37] * (AO.powf(parameters[44]));
            let AX = parameters[38] * (AO.powf(parameters[45]));
            let AY = parameters[39] * (AO.powf(parameters[46]));
            let AZ = parameters[40] * (AN.powf(parameters[47]));
            let BB = 5.522602e-23f64 * AL;
            let BE = if BC < parameters[12] { 1.0 } else { 0.0 };
            if BE != 0.0 {
            } else {
            }
            let BF = if BC > parameters[13] { 1.0 } else { 0.0 };
            if BF != 0.0 {
            } else {
            }
            let BG = if BD < parameters[14] { 1.0 } else { 0.0 };
            if BG != 0.0 {
            } else {
            }
            let BH = if BD > parameters[15] { 1.0 } else { 0.0 };
            if BH != 0.0 {
            } else {
            }
            let BI = BC + parameters[31];
            let BJ = BD + parameters[32];
            let BK = if BI <= A { 1.0 } else { 0.0 };
            if BK != 0.0 {
            } else {
            }
            let BL = if BJ <= A { 1.0 } else { 0.0 };
            if BL != 0.0 {
            } else {
            }
            let BM = 1.179e0f64 - (AL * (9.025e-5f64 + (AL * 3.05e-7f64)));
            let BO = (if ((((1.045e0f64 + (4.5e-4f64 * AL)) * ((5.23e-1f64 + (1.4e-3f64 * AL)) - (1.48e-6f64 * AM))) * AM) / 9e4f64) >= BN { ((((1.045e0f64 + (4.5e-4f64 * AL)) * ((5.23e-1f64 + (1.4e-3f64 * AL)) - (1.48e-6f64 * AM))) * AM) / 9e4f64) } else { BN }).sqrt();
            let BP = AS / ((2.5e25f64 * BO) * (BO.sqrt()));
            let BQ = F * AP;
            let BR = BM + (BQ * ((G * BP).ln()));
            let BT = BM + (BS * AP);
            let BU = AT.sqrt();
            let BV = I * BU;
            let BW = BV * BV;
            let BX = AS / BW;
            let BZ = AS + (BV * BY);
            let CA = AS / BZ;
            let CC = CB * BZ;
            let CD = (BM + (BQ * ((H * BP).ln()))) * AT;
            let CE = J * BU;
            let CF = CE * CE;
            let CG = AS + (CE * BY);
            let CH = CB * CG;
            let CK = if CD < CJ { 1.0 } else { 0.0 };
            let LB = if CK != 0.0 {
                let CL = (-CD).exp();
                CL
            } else {
                let CN = CD - CJ;
                let CO = CM / (AS + (CN * (AS + ((V * CN) * (AS + (CN * S))))));
                CO
            };
            let DX;
            let DZ;
            let EB;
            let ED;
            let EH;
            if CP != 0.0 {
                let CR = (AV * BD) / ((CQ + ((parameters[2] - AS) * 9e0f64)) * BC);
                let CS = AW / (BD * BC);
                let CT = BD + parameters[33];
                let CU = AX / (F * CT);
                let CW = (AY * BC) / (CV * CT);
                let CX = if CR > BN { 1.0 } else { 0.0 };
                let DA;
                if CX != 0.0 {
                    let CY = if CR < B { 1.0 } else { 0.0 };
                    let CZ = if CY != 0.0 {
                        CR
                    } else {
                        B
                    };
                    DA = CZ;
                } else {
                    DA = BN;
                }
                let DB = if CS > BN { 1.0 } else { 0.0 };
                let DE;
                if DB != 0.0 {
                    let DC = if CS < AQ { 1.0 } else { 0.0 };
                    let DD = if DC != 0.0 {
                        CS
                    } else {
                        AQ
                    };
                    DE = DD;
                } else {
                    DE = BN;
                }
                let DF = if CU > BN { 1.0 } else { 0.0 };
                let DI;
                if DF != 0.0 {
                    let DG = if CU < B { 1.0 } else { 0.0 };
                    let DH = if DG != 0.0 {
                        CU
                    } else {
                        B
                    };
                    DI = DH;
                } else {
                    DI = BN;
                }
                let DJ = if CW > BN { 1.0 } else { 0.0 };
                let DM;
                if DJ != 0.0 {
                    let DK = if CW < B { 1.0 } else { 0.0 };
                    let DL = if DK != 0.0 {
                        CW
                    } else {
                        B
                    };
                    DM = DL;
                } else {
                    DM = BN;
                }
                let DN = if AZ > BN { 1.0 } else { 0.0 };
                let DR;
                if DN != 0.0 {
                    let DP = if AZ < DO { 1.0 } else { 0.0 };
                    let DQ = if DP != 0.0 {
                        AZ
                    } else {
                        DO
                    };
                    DR = DQ;
                } else {
                    DR = BN;
                }
                let DS = AS / DA;
                let DT = AS / DE;
                let DU = AS / DI;
                let DV = AS / DM;
                let DW = ((CV * DR) * BD) / BC;
                DX = DS;
                DZ = DT;
                EB = DU;
                ED = DV;
                EH = DW;
            } else {
                DX = A;
                DZ = A;
                EB = A;
                ED = A;
                EH = A;
            }
            let DY = BB * DX;
            let EA = BB * DZ;
            let EC = BB * EB;
            let EE = BB * ED;
            let EG = if EF == A { 1.0 } else { 0.0 };
            let AKC = if EG != 0.0 {
                A
            } else {
                let EI = BB * EH;
                EI
            };
            let AAG;
            let AAI;
            let ACJ;
            let ACT;
            let ACW;
            let ADJ;
            let ADL;
            let ADT;
            let AEG;
            let AEQ;
            let AET;
            let AFN;
            let AGC;
            let AGE;
            let AHN;
            let AJD;
            if EJ != 0.0 {
                let EM = AN.powf(parameters[52]);
                let EN = (((parameters[55] * BJ) * BI) * EK) * EM;
                let EO = ((((F * parameters[56]) * EL) * BJ) * EK) * EM;
                let EP = (((parameters[60] * BJ) * BI) * EK) * EM;
                let EQ = ((((F * parameters[61]) * EL) * BJ) * EK) * EM;
                let ES = AS / ER;
                let EU = AS / ET;
                let EW = ((1.3333333333333333e0f64 * ((2.918995620956536e-49f64 * ER).sqrt())) / EV) * D;
                let EX = ((1.3333333333333333e0f64 * ((2.918995620956536e-49f64 * ET).sqrt())) / EV) * D;
                let EZ = if EY < A { 1.0 } else { 0.0 };
                let AEU = if EZ != 0.0 {
                    let FB = (-4.95e-1f64 * FA) / EY;
                    FB
                } else {
                    A
                };
                let FD = if FC < A { 1.0 } else { 0.0 };
                let ACX = if FD != 0.0 {
                    let FF = (-4.95e-1f64 * FE) / FC;
                    FF
                } else {
                    A
                };
                let FG = V * ((O * BR) + BM);
                let FH = V * ((O * BT) + BM);
                let FI = parameters[57] * AP;
                let FJ = parameters[62] * AP;
                AAG = EO;
                AAI = EQ;
                ACJ = FJ;
                ACT = EU;
                ACW = ACX;
                ADJ = FH;
                ADL = FG;
                ADT = EX;
                AEG = FI;
                AEQ = ES;
                AET = AEU;
                AFN = EW;
                AGC = EN;
                AGE = EP;
                AHN = EX;
                AJD = EW;
            } else {
                AAG = A;
                AAI = A;
                ACJ = A;
                ACT = FK;
                ACW = A;
                ADJ = A;
                ADL = A;
                ADT = A;
                AEG = A;
                AEQ = FK;
                AET = A;
                AFN = A;
                AGC = A;
                AGE = A;
                AHN = A;
                AJD = A;
            }
            let FM = FL - node_potentials[5];
            let FN = O * (FM - parameters[27]);
            let FP = if FN > FO { 1.0 } else { 0.0 };
            let FY;
            if FP != 0.0 {
                let FR = V * (FN + (((FN * FN) + FQ).sqrt()));
                FY = FR;
            } else {
                let FS = A - FN;
                let FT = if FS > FO { 1.0 } else { 0.0 };
                let FX = if FT != 0.0 {
                    let FU = (V * FQ) / (FS + (((FS * FS) + FQ).sqrt()));
                    FU
                } else {
                    let FW = V * (FN + ((FV + FQ).sqrt()));
                    FW
                };
                FY = FX;
            }
            let FZ = AS + (parameters[26] * FY);
            let GB = GA - FZ;
            let GC = if GB > FO { 1.0 } else { 0.0 };
            let GK;
            if GC != 0.0 {
                let GE = GA - (V * (GB + (((GB * GB) + GD).sqrt())));
                GK = GE;
            } else {
                let GF = FZ - GA;
                let GG = if GF > FO { 1.0 } else { 0.0 };
                let GJ = if GG != 0.0 {
                    let GH = GA - (5e-7f64 / (GF + (((GF * GF) + GD).sqrt())));
                    GH
                } else {
                    let GI = GA - (V * (GB + 1e-3f64));
                    GI
                };
                GK = GJ;
            }
            let GL = G * GK;
            let GM = GL / 1e23f64;
            let GN = BM + (BQ * ((GL * BP).ln()));
            let GO = ((3.348580862e-29f64 * GL).sqrt()) / E;
            let GV;
            let HC;
            if L != 0.0 {
                let GP = ((GO * GO) * GN).sqrt();
                let GS = (7.5e-1f64 * GQ) * (GP.powf(M));
                let GT = GN + GS;
                let GU = GO * (AS + ((1.3333333333333333e0f64 * GS) / GP));
                GV = GU;
                HC = GT;
            } else {
                GV = GO;
                HC = GN;
            }
            let GW = GV * BU;
            let GX = GW * GW;
            let GY = AS / GX;
            let GZ = AS + (GW * BY);
            let HA = AS / GZ;
            let HB = CB * GZ;
            let HD = HC * AT;
            let HE = if HD < CJ { 1.0 } else { 0.0 };
            let HO = if HE != 0.0 {
                let HF = (-HD).exp();
                HF
            } else {
                let HG = HD - CJ;
                let HH = CM / (AS + (HG * (AS + ((V * HG) * (AS + (HG * S))))));
                HH
            };
            let HI = CI + (GW * 7.324648775608221e-1f64);
            let HJ = CI + (CE * 7.324648775608221e-1f64);
            let HK = O * (FM - AU);
            let HL = HK * AT;
            let HM = if (HL.abs()) <= HB { 1.0 } else { 0.0 };
            let KX;
            if HM != 0.0 {
                let HP = (HL * HA) * (AS + (((HL * (AS - HO)) * GW) * (((HA * HA) * HN) * BY)));
                KX = HP;
            } else {
                let HQ = if HL < (-HB) { 1.0 } else { 0.0 };
                let KY;
                if HQ != 0.0 {
                    let HR = -HL;
                    let HS = (CI * HR) * HA;
                    let HU = HS - BS;
                    let HW = V * ((HS + HT) - (((HU * HU) + HV).sqrt()));
                    let HX = HR - HW;
                    let HY = (HX * HX) + (GX * (HW + AS));
                    let HZ = (F * HX) - GX;
                    let IA = (-HW) + ((HY * GY).ln());
                    let IB = HY + HZ;
                    let IC = (IB * IB) + ((((V * HZ) * HZ) - HY) * IA);
                    let ID = HW + (((HY * IB) * IA) / (IC + (((((IB * IA) * IA) / IC) * HZ) * (((HZ * HZ) * S) - HY))));
                    let IF = if ID < IE { 1.0 } else { 0.0 };
                    let IK = if IF != 0.0 {
                        let IG = ID.exp();
                        IG
                    } else {
                        let II = ID - IE;
                        let IJ = IH * (AS + (II * (AS + ((V * II) * (AS + (II * S))))));
                        IJ
                    };
                    let IL = HR - ID;
                    let IM = HO * (AS / IK);
                    let IN = (F * IL) + (GX * (((IK - AS) - IM) + HO));
                    let IO = F * ((IL * IL) - (GX * ((((IK - ID) - AS) + IM) + (HO * (ID - AS)))));
                    let IP = (-ID) - (IO / (IN + (((IN * IN) - (IO * (F - (GX * (IK + IM))))).sqrt())));
                    KY = IP;
                } else {
                    let IR = AS / (CI + (GW * IQ));
                    let IS = -((HL * HA) * (AS + (((((GZ * CI) * IR) - AS) * IR) * HL)));
                    let IT = if IS > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                    let IX = if IT != 0.0 {
                        let IU = IS.exp();
                        IU
                    } else {
                        let IW = IV / (AS + ((-2.3025850929940458e2f64 - IS) * (AS + ((V * (-2.3025850929940458e2f64 - IS)) * (AS + ((-2.3025850929940458e2f64 - IS) * S))))));
                        IW
                    };
                    let IY = GX * V;
                    let JA = (HL + IY) - (GW * (((HL + (GX * IZ)) - (AS - IX)).sqrt()));
                    let JB = HD + CQ;
                    let JC = JB - JA;
                    let JD = if JC > FO { 1.0 } else { 0.0 };
                    let JL;
                    if JD != 0.0 {
                        let JF = JB - (V * (JC + (((JC * JC) + JE).sqrt())));
                        JL = JF;
                    } else {
                        let JG = JA - JB;
                        let JH = if JG > FO { 1.0 } else { 0.0 };
                        let JK = if JH != 0.0 {
                            let JI = JB - (2.5e0f64 / (JG + (((JG * JG) + JE).sqrt())));
                            JI
                        } else {
                            let JJ = JB - (V * (JC + 2.23606797749979e0f64));
                            JJ
                        };
                        JL = JK;
                    }
                    let JM = JL - (V * (JB - (((JB * JB) + JE).sqrt())));
                    let JN = HL - JM;
                    let JO = (-JM).exp();
                    let JQ = if JP >= ((JN * JN) - (GX * (((JO + JM) - AS) - (HO * (JM + AS))))) { JP } else { ((JN * JN) - (GX * (((JO + JM) - AS) - (HO * (JM + AS))))) };
                    let JR = AS - (IY * JO);
                    let JS = (F * JN) + (GX * ((AS - JO) - HO));
                    let JT = (HD - JM) + ((JQ / GX).ln());
                    let JU = JQ + JS;
                    let JW = if (JT.abs()) < JV { 1.0 } else { 0.0 };
                    let KA = if JW != 0.0 {
                        JM
                    } else {
                        let JX = JQ * JR;
                        let JY = (JU * JU) + ((((V * JS) * JS) - JX) * JT);
                        let JZ = JM + (((JQ * JU) * JT) / (JY + (((((JU * JT) * JT) / JY) * JS) * (((JS * JS) * S) - JX))));
                        JZ
                    };
                    let KB = if KA < IE { 1.0 } else { 0.0 };
                    let KN;
                    let KP;
                    if KB != 0.0 {
                        let KC = KA.exp();
                        let KD = AS / KC;
                        let KE = HO * KC;
                        KN = KD;
                        KP = KE;
                    } else {
                        let KF = if KA > (HD - IE) { 1.0 } else { 0.0 };
                        let KO;
                        let KQ;
                        if KF != 0.0 {
                            let KG = (KA - HD).exp();
                            let KH = HO / KG;
                            KO = KH;
                            KQ = KG;
                        } else {
                            let KI = (HD - KA) - IE;
                            let KJ = IV / (AS + (KI * (AS + ((V * KI) * (AS + (KI * S))))));
                            let KK = KA - IE;
                            let KL = IV / (AS + (KK * (AS + ((V * KK) * (AS + (KK * S))))));
                            KO = KL;
                            KQ = KJ;
                        }
                        KN = KO;
                        KP = KQ;
                    }
                    let KM = HL - KA;
                    let KR = (F * KM) + (GX * (((AS - KN) + KP) - HO));
                    let KS = F * ((KM * KM) - (GX * ((((KN + KA) - AS) + KP) - (HO * (KA + AS)))));
                    let KT = KA + (KS / (KR + (((KR * KR) - (KS * (F - (GX * (KN + KP))))).sqrt())));
                    KY = KT;
                }
                KX = KY;
            }
            let KU = if H < 1e27f64 { 1.0 } else { 0.0 };
            let QW;
            let QY;
            if KU != 0.0 {
                let KW = (-O) * KV;
                let KZ = (KW * (HK - (KX * AP))) * AT;
                let LA = if (KZ.abs()) <= CC { 1.0 } else { 0.0 };
                let NX;
                if LA != 0.0 {
                    let LC = (KZ * CA) * (AS + (((KZ * (AS - LB)) * BV) * (((CA * CA) * HN) * BY)));
                    NX = LC;
                } else {
                    let LD = if KZ < (-CC) { 1.0 } else { 0.0 };
                    let NY;
                    if LD != 0.0 {
                        let LE = -KZ;
                        let LF = (CI * LE) * CA;
                        let LG = LF - BS;
                        let LH = V * ((LF + HT) - (((LG * LG) + HV).sqrt()));
                        let LI = LE - LH;
                        let LJ = (LI * LI) + (BW * (LH + AS));
                        let LK = (F * LI) - BW;
                        let LL = (-LH) + ((LJ * BX).ln());
                        let LM = LJ + LK;
                        let LN = (LM * LM) + ((((V * LK) * LK) - LJ) * LL);
                        let LO = LH + (((LJ * LM) * LL) / (LN + (((((LM * LL) * LL) / LN) * LK) * (((LK * LK) * S) - LJ))));
                        let LP = if LO < IE { 1.0 } else { 0.0 };
                        let LT = if LP != 0.0 {
                            let LQ = LO.exp();
                            LQ
                        } else {
                            let LR = LO - IE;
                            let LS = IH * (AS + (LR * (AS + ((V * LR) * (AS + (LR * S))))));
                            LS
                        };
                        let LU = LE - LO;
                        let LV = LB * (AS / LT);
                        let LW = (F * LU) + (BW * (((LT - AS) - LV) + LB));
                        let LX = F * ((LU * LU) - (BW * ((((LT - LO) - AS) + LV) + (LB * (LO - AS)))));
                        let LY = (-LO) - (LX / (LW + (((LW * LW) - (LX * (F - (BW * (LT + LV))))).sqrt())));
                        NY = LY;
                    } else {
                        let LZ = AS / (CI + (BV * IQ));
                        let MA = -((KZ * CA) * (AS + (((((BZ * CI) * LZ) - AS) * LZ) * KZ)));
                        let MB = if MA > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                        let ME = if MB != 0.0 {
                            let MC = MA.exp();
                            MC
                        } else {
                            let MD = IV / (AS + ((-2.3025850929940458e2f64 - MA) * (AS + ((V * (-2.3025850929940458e2f64 - MA)) * (AS + ((-2.3025850929940458e2f64 - MA) * S))))));
                            MD
                        };
                        let MF = BW * V;
                        let MG = (KZ + MF) - (BV * (((KZ + (BW * IZ)) - (AS - ME)).sqrt()));
                        let MH = CD + CQ;
                        let MI = MH - MG;
                        let MJ = if MI > FO { 1.0 } else { 0.0 };
                        let MQ;
                        if MJ != 0.0 {
                            let MK = MH - (V * (MI + (((MI * MI) + JE).sqrt())));
                            MQ = MK;
                        } else {
                            let ML = MG - MH;
                            let MM = if ML > FO { 1.0 } else { 0.0 };
                            let MP = if MM != 0.0 {
                                let MN = MH - (2.5e0f64 / (ML + (((ML * ML) + JE).sqrt())));
                                MN
                            } else {
                                let MO = MH - (V * (MI + 2.23606797749979e0f64));
                                MO
                            };
                            MQ = MP;
                        }
                        let MR = MQ - (V * (MH - (((MH * MH) + JE).sqrt())));
                        let MS = KZ - MR;
                        let MT = (-MR).exp();
                        let MU = if JP >= ((MS * MS) - (BW * (((MT + MR) - AS) - (LB * (MR + AS))))) { JP } else { ((MS * MS) - (BW * (((MT + MR) - AS) - (LB * (MR + AS))))) };
                        let MV = AS - (MF * MT);
                        let MW = (F * MS) + (BW * ((AS - MT) - LB));
                        let MX = (CD - MR) + ((MU / BW).ln());
                        let MY = MU + MW;
                        let MZ = if (MX.abs()) < JV { 1.0 } else { 0.0 };
                        let ND = if MZ != 0.0 {
                            MR
                        } else {
                            let NA = MU * MV;
                            let NB = (MY * MY) + ((((V * MW) * MW) - NA) * MX);
                            let NC = MR + (((MU * MY) * MX) / (NB + (((((MY * MX) * MX) / NB) * MW) * (((MW * MW) * S) - NA))));
                            NC
                        };
                        let NE = if ND < IE { 1.0 } else { 0.0 };
                        let NQ;
                        let NS;
                        if NE != 0.0 {
                            let NF = ND.exp();
                            let NG = AS / NF;
                            let NH = LB * NF;
                            NQ = NG;
                            NS = NH;
                        } else {
                            let NI = if ND > (CD - IE) { 1.0 } else { 0.0 };
                            let NR;
                            let NT;
                            if NI != 0.0 {
                                let NJ = (ND - CD).exp();
                                let NK = LB / NJ;
                                NR = NK;
                                NT = NJ;
                            } else {
                                let NL = (CD - ND) - IE;
                                let NM = IV / (AS + (NL * (AS + ((V * NL) * (AS + (NL * S))))));
                                let NN = ND - IE;
                                let NO = IV / (AS + (NN * (AS + ((V * NN) * (AS + (NN * S))))));
                                NR = NO;
                                NT = NM;
                            }
                            NQ = NR;
                            NS = NT;
                        }
                        let NP = KZ - ND;
                        let NU = (F * NP) + (BW * (((AS - NQ) + NS) - LB));
                        let NV = F * ((NP * NP) - (BW * ((((NQ + ND) - AS) + NS) - (LB * (ND + AS)))));
                        let NW = ND + (NV / (NU + (((NU * NU) - (NV * (F - (BW * (NQ + NS))))).sqrt())));
                        NY = NW;
                    }
                    NX = NY;
                }
                let NZ = (HK - ((KW * NX) * AP)) / AP;
                let OA = if (NZ.abs()) <= HB { 1.0 } else { 0.0 };
                let QZ;
                if OA != 0.0 {
                    let OB = (NZ * HA) * (AS + (((NZ * (AS - HO)) * GW) * (((HA * HA) * HN) * BY)));
                    QZ = OB;
                } else {
                    let OC = if NZ < (-HB) { 1.0 } else { 0.0 };
                    let RA;
                    if OC != 0.0 {
                        let OD = -NZ;
                        let OE = (CI * OD) * HA;
                        let OF = OE - BS;
                        let OG = V * ((OE + HT) - (((OF * OF) + HV).sqrt()));
                        let OH = OD - OG;
                        let OI = (OH * OH) + (GX * (OG + AS));
                        let OJ = (F * OH) - GX;
                        let OK = (-OG) + ((OI * GY).ln());
                        let OL = OI + OJ;
                        let OM = (OL * OL) + ((((V * OJ) * OJ) - OI) * OK);
                        let ON = OG + (((OI * OL) * OK) / (OM + (((((OL * OK) * OK) / OM) * OJ) * (((OJ * OJ) * S) - OI))));
                        let OO = if ON < IE { 1.0 } else { 0.0 };
                        let OS = if OO != 0.0 {
                            let OP = ON.exp();
                            OP
                        } else {
                            let OQ = ON - IE;
                            let OR = IH * (AS + (OQ * (AS + ((V * OQ) * (AS + (OQ * S))))));
                            OR
                        };
                        let OT = OD - ON;
                        let OU = HO * (AS / OS);
                        let OV = (F * OT) + (GX * (((OS - AS) - OU) + HO));
                        let OW = F * ((OT * OT) - (GX * ((((OS - ON) - AS) + OU) + (HO * (ON - AS)))));
                        let OX = (-ON) - (OW / (OV + (((OV * OV) - (OW * (F - (GX * (OS + OU))))).sqrt())));
                        RA = OX;
                    } else {
                        let OY = AS / (CI + (GW * IQ));
                        let OZ = -((NZ * HA) * (AS + (((((GZ * CI) * OY) - AS) * OY) * NZ)));
                        let PA = if OZ > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                        let PD = if PA != 0.0 {
                            let PB = OZ.exp();
                            PB
                        } else {
                            let PC = IV / (AS + ((-2.3025850929940458e2f64 - OZ) * (AS + ((V * (-2.3025850929940458e2f64 - OZ)) * (AS + ((-2.3025850929940458e2f64 - OZ) * S))))));
                            PC
                        };
                        let PE = GX * V;
                        let PF = (NZ + PE) - (GW * (((NZ + (GX * IZ)) - (AS - PD)).sqrt()));
                        let PG = HD + CQ;
                        let PH = PG - PF;
                        let PI = if PH > FO { 1.0 } else { 0.0 };
                        let PP;
                        if PI != 0.0 {
                            let PJ = PG - (V * (PH + (((PH * PH) + JE).sqrt())));
                            PP = PJ;
                        } else {
                            let PK = PF - PG;
                            let PL = if PK > FO { 1.0 } else { 0.0 };
                            let PO = if PL != 0.0 {
                                let PM = PG - (2.5e0f64 / (PK + (((PK * PK) + JE).sqrt())));
                                PM
                            } else {
                                let PN = PG - (V * (PH + 2.23606797749979e0f64));
                                PN
                            };
                            PP = PO;
                        }
                        let PQ = PP - (V * (PG - (((PG * PG) + JE).sqrt())));
                        let PR = NZ - PQ;
                        let PS = (-PQ).exp();
                        let PT = if JP >= ((PR * PR) - (GX * (((PS + PQ) - AS) - (HO * (PQ + AS))))) { JP } else { ((PR * PR) - (GX * (((PS + PQ) - AS) - (HO * (PQ + AS))))) };
                        let PU = AS - (PE * PS);
                        let PV = (F * PR) + (GX * ((AS - PS) - HO));
                        let PW = (HD - PQ) + ((PT / GX).ln());
                        let PX = PT + PV;
                        let PY = if (PW.abs()) < JV { 1.0 } else { 0.0 };
                        let QC = if PY != 0.0 {
                            PQ
                        } else {
                            let PZ = PT * PU;
                            let QA = (PX * PX) + ((((V * PV) * PV) - PZ) * PW);
                            let QB = PQ + (((PT * PX) * PW) / (QA + (((((PX * PW) * PW) / QA) * PV) * (((PV * PV) * S) - PZ))));
                            QB
                        };
                        let QD = if QC < IE { 1.0 } else { 0.0 };
                        let QP;
                        let QR;
                        if QD != 0.0 {
                            let QE = QC.exp();
                            let QF = AS / QE;
                            let QG = HO * QE;
                            QP = QF;
                            QR = QG;
                        } else {
                            let QH = if QC > (HD - IE) { 1.0 } else { 0.0 };
                            let QQ;
                            let QS;
                            if QH != 0.0 {
                                let QI = (QC - HD).exp();
                                let QJ = HO / QI;
                                QQ = QJ;
                                QS = QI;
                            } else {
                                let QK = (HD - QC) - IE;
                                let QL = IV / (AS + (QK * (AS + ((V * QK) * (AS + (QK * S))))));
                                let QM = QC - IE;
                                let QN = IV / (AS + (QM * (AS + ((V * QM) * (AS + (QM * S))))));
                                QQ = QN;
                                QS = QL;
                            }
                            QP = QQ;
                            QR = QS;
                        }
                        let QO = NZ - QC;
                        let QT = (F * QO) + (GX * (((AS - QP) + QR) - HO));
                        let QU = F * ((QO * QO) - (GX * ((((QP + QC) - AS) + QR) - (HO * (QC + AS)))));
                        let QV = QC + (QU / (QT + (((QT * QT) - (QU * (F - (GX * (QP + QR))))).sqrt())));
                        RA = QV;
                    }
                    QZ = RA;
                }
                QW = NZ;
                QY = QZ;
            } else {
                QW = HL;
                QY = KX;
            }
            let QX = if (if QW <= A { 1.0 } else { 0.0 }) != 0.0 || (if parameters[21] < AS { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            if QX != 0.0 {
            } else {
                let RB = if QY < IE { 1.0 } else { 0.0 };
                if RB != 0.0 {
                } else {
                    let RC = if QY > (HD - IE) { 1.0 } else { 0.0 };
                    if RC != 0.0 {
                    } else {
                    }
                }
                let RD = if QY < CB { 1.0 } else { 0.0 };
                if RD != 0.0 {
                } else {
                }
            }
            let RF = HK + RE;
            let RG = RF * AT;
            let RH = if (RG.abs()) <= HB { 1.0 } else { 0.0 };
            let SY;
            if RH != 0.0 {
                let RI = RG / GZ;
                SY = RI;
            } else {
                let RJ = if RG > HB { 1.0 } else { 0.0 };
                let SZ;
                if RJ != 0.0 {
                    let RK = (RG / GZ) * (AS + (((((GZ * CI) / HI) - AS) / HI) * RG));
                    let RL = if RK < CJ { 1.0 } else { 0.0 };
                    let RP = if RL != 0.0 {
                        let RM = (-RK).exp();
                        RM
                    } else {
                        let RN = RK - CJ;
                        let RO = CM / (AS + (RN * (AS + ((V * RN) * (AS + (RN * S))))));
                        RO
                    };
                    let RQ = V * GX;
                    let RR = (RG + RQ) - (GW * (((RG + (IZ * GX)) - (AS - RP)).sqrt()));
                    let RS = if RR < CJ { 1.0 } else { 0.0 };
                    let RW = if RS != 0.0 {
                        let RT = (-RR).exp();
                        RT
                    } else {
                        let RU = RR - CJ;
                        let RV = CM / (AS + (RU * (AS + ((V * RU) * (AS + (RU * S))))));
                        RV
                    };
                    let RX = RG - RR;
                    let RY = (F * RX) + (GX * (AS - RW));
                    let RZ = (RX * RX) - (GX * ((RR - AS) + RW));
                    let SA = RR + ((F * RZ) / (RY + (((RY * RY) - ((BA * (AS - (RQ * RW))) * RZ)).sqrt())));
                    SZ = SA;
                } else {
                    let SB = -RG;
                    let SC = (CI * SB) / GZ;
                    let SD = SC - BS;
                    let SE = V * ((SC + HT) - (((SD * SD) + HV).sqrt()));
                    let SF = SB - SE;
                    let SG = (SF * SF) + (GX * (SE + AS));
                    let SH = (F * SF) - GX;
                    let SI = ((SG / GX).ln()) - SE;
                    let SJ = SG + SH;
                    let SK = (SJ * SJ) + ((((V * SH) * SH) - SG) * SI);
                    let SL = SE + (((SG * SJ) * SI) / (SK + (((((SJ * SI) * SI) / SK) * SH) * (((SH * SH) * S) - SG))));
                    let SM = if (SL.abs()) < IE { 1.0 } else { 0.0 };
                    let SS;
                    if SM != 0.0 {
                        let SN = SL.exp();
                        SS = SN;
                    } else {
                        let SO = if SL < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                        let ST = if SO != 0.0 {
                            let SP = IV / (AS + ((-2.3025850929940458e2f64 - SL) * (AS + ((V * (-2.3025850929940458e2f64 - SL)) * (AS + ((-2.3025850929940458e2f64 - SL) * S))))));
                            SP
                        } else {
                            let SQ = SL - IE;
                            let SR = IH * (AS + (SQ * (AS + ((V * SQ) * (AS + (SQ * S))))));
                            SR
                        };
                        SS = ST;
                    }
                    let SU = SB - SL;
                    let SV = (F * SU) + (GX * (SS - AS));
                    let SW = (SU * SU) + (GX * ((SL + AS) - SS));
                    let SX = -(SL + ((F * SW) / (SV + (((SV * SV) - ((BA * (AS - ((V * GX) * SS))) * SW)).sqrt()))));
                    SZ = SX;
                }
                SY = SZ;
            }
            let TA = SY * AP;
            let XV;
            if KU != 0.0 {
                let TB = (-O) * KV;
                let TC = (TB * (HK - TA)) * AT;
                let TD = if (TC.abs()) <= CC { 1.0 } else { 0.0 };
                let VZ;
                if TD != 0.0 {
                    let TE = (TC * CA) * (AS + (((TC * (AS - LB)) * BV) * (((CA * CA) * HN) * BY)));
                    VZ = TE;
                } else {
                    let TF = if TC < (-CC) { 1.0 } else { 0.0 };
                    let WA;
                    if TF != 0.0 {
                        let TG = -TC;
                        let TH = (CI * TG) * CA;
                        let TI = TH - BS;
                        let TJ = V * ((TH + HT) - (((TI * TI) + HV).sqrt()));
                        let TK = TG - TJ;
                        let TL = (TK * TK) + (BW * (TJ + AS));
                        let TM = (F * TK) - BW;
                        let TN = (-TJ) + ((TL * BX).ln());
                        let TO = TL + TM;
                        let TP = (TO * TO) + ((((V * TM) * TM) - TL) * TN);
                        let TQ = TJ + (((TL * TO) * TN) / (TP + (((((TO * TN) * TN) / TP) * TM) * (((TM * TM) * S) - TL))));
                        let TR = if TQ < IE { 1.0 } else { 0.0 };
                        let TV = if TR != 0.0 {
                            let TS = TQ.exp();
                            TS
                        } else {
                            let TT = TQ - IE;
                            let TU = IH * (AS + (TT * (AS + ((V * TT) * (AS + (TT * S))))));
                            TU
                        };
                        let TW = TG - TQ;
                        let TX = LB * (AS / TV);
                        let TY = (F * TW) + (BW * (((TV - AS) - TX) + LB));
                        let TZ = F * ((TW * TW) - (BW * ((((TV - TQ) - AS) + TX) + (LB * (TQ - AS)))));
                        let UA = (-TQ) - (TZ / (TY + (((TY * TY) - (TZ * (F - (BW * (TV + TX))))).sqrt())));
                        WA = UA;
                    } else {
                        let UB = AS / (CI + (BV * IQ));
                        let UC = -((TC * CA) * (AS + (((((BZ * CI) * UB) - AS) * UB) * TC)));
                        let UD = if UC > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                        let UG = if UD != 0.0 {
                            let UE = UC.exp();
                            UE
                        } else {
                            let UF = IV / (AS + ((-2.3025850929940458e2f64 - UC) * (AS + ((V * (-2.3025850929940458e2f64 - UC)) * (AS + ((-2.3025850929940458e2f64 - UC) * S))))));
                            UF
                        };
                        let UH = BW * V;
                        let UI = (TC + UH) - (BV * (((TC + (BW * IZ)) - (AS - UG)).sqrt()));
                        let UJ = CD + CQ;
                        let UK = UJ - UI;
                        let UL = if UK > FO { 1.0 } else { 0.0 };
                        let US;
                        if UL != 0.0 {
                            let UM = UJ - (V * (UK + (((UK * UK) + JE).sqrt())));
                            US = UM;
                        } else {
                            let UN = UI - UJ;
                            let UO = if UN > FO { 1.0 } else { 0.0 };
                            let UR = if UO != 0.0 {
                                let UP = UJ - (2.5e0f64 / (UN + (((UN * UN) + JE).sqrt())));
                                UP
                            } else {
                                let UQ = UJ - (V * (UK + 2.23606797749979e0f64));
                                UQ
                            };
                            US = UR;
                        }
                        let UT = US - (V * (UJ - (((UJ * UJ) + JE).sqrt())));
                        let UU = TC - UT;
                        let UV = (-UT).exp();
                        let UW = if JP >= ((UU * UU) - (BW * (((UV + UT) - AS) - (LB * (UT + AS))))) { JP } else { ((UU * UU) - (BW * (((UV + UT) - AS) - (LB * (UT + AS))))) };
                        let UX = AS - (UH * UV);
                        let UY = (F * UU) + (BW * ((AS - UV) - LB));
                        let UZ = (CD - UT) + ((UW / BW).ln());
                        let VA = UW + UY;
                        let VB = if (UZ.abs()) < JV { 1.0 } else { 0.0 };
                        let VF = if VB != 0.0 {
                            UT
                        } else {
                            let VC = UW * UX;
                            let VD = (VA * VA) + ((((V * UY) * UY) - VC) * UZ);
                            let VE = UT + (((UW * VA) * UZ) / (VD + (((((VA * UZ) * UZ) / VD) * UY) * (((UY * UY) * S) - VC))));
                            VE
                        };
                        let VG = if VF < IE { 1.0 } else { 0.0 };
                        let VS;
                        let VU;
                        if VG != 0.0 {
                            let VH = VF.exp();
                            let VI = AS / VH;
                            let VJ = LB * VH;
                            VS = VI;
                            VU = VJ;
                        } else {
                            let VK = if VF > (CD - IE) { 1.0 } else { 0.0 };
                            let VT;
                            let VV;
                            if VK != 0.0 {
                                let VL = (VF - CD).exp();
                                let VM = LB / VL;
                                VT = VM;
                                VV = VL;
                            } else {
                                let VN = (CD - VF) - IE;
                                let VO = IV / (AS + (VN * (AS + ((V * VN) * (AS + (VN * S))))));
                                let VP = VF - IE;
                                let VQ = IV / (AS + (VP * (AS + ((V * VP) * (AS + (VP * S))))));
                                VT = VQ;
                                VV = VO;
                            }
                            VS = VT;
                            VU = VV;
                        }
                        let VR = TC - VF;
                        let VW = (F * VR) + (BW * (((AS - VS) + VU) - LB));
                        let VX = F * ((VR * VR) - (BW * ((((VS + VF) - AS) + VU) - (LB * (VF + AS)))));
                        let VY = VF + (VX / (VW + (((VW * VW) - (VX * (F - (BW * (VS + VU))))).sqrt())));
                        WA = VY;
                    }
                    VZ = WA;
                }
                let WB = (RF - ((TB * VZ) * AP)) / AP;
                let WC = if (WB.abs()) <= HB { 1.0 } else { 0.0 };
                let XT;
                if WC != 0.0 {
                    let WD = WB / GZ;
                    XT = WD;
                } else {
                    let WE = if WB > HB { 1.0 } else { 0.0 };
                    let XU;
                    if WE != 0.0 {
                        let WF = (WB / GZ) * (AS + (((((GZ * CI) / HI) - AS) / HI) * WB));
                        let WG = if WF < CJ { 1.0 } else { 0.0 };
                        let WK = if WG != 0.0 {
                            let WH = (-WF).exp();
                            WH
                        } else {
                            let WI = WF - CJ;
                            let WJ = CM / (AS + (WI * (AS + ((V * WI) * (AS + (WI * S))))));
                            WJ
                        };
                        let WL = V * GX;
                        let WM = (WB + WL) - (GW * (((WB + (IZ * GX)) - (AS - WK)).sqrt()));
                        let WN = if WM < CJ { 1.0 } else { 0.0 };
                        let WR = if WN != 0.0 {
                            let WO = (-WM).exp();
                            WO
                        } else {
                            let WP = WM - CJ;
                            let WQ = CM / (AS + (WP * (AS + ((V * WP) * (AS + (WP * S))))));
                            WQ
                        };
                        let WS = WB - WM;
                        let WT = (F * WS) + (GX * (AS - WR));
                        let WU = (WS * WS) - (GX * ((WM - AS) + WR));
                        let WV = WM + ((F * WU) / (WT + (((WT * WT) - ((BA * (AS - (WL * WR))) * WU)).sqrt())));
                        XU = WV;
                    } else {
                        let WW = -WB;
                        let WX = (CI * WW) / GZ;
                        let WY = WX - BS;
                        let WZ = V * ((WX + HT) - (((WY * WY) + HV).sqrt()));
                        let XA = WW - WZ;
                        let XB = (XA * XA) + (GX * (WZ + AS));
                        let XC = (F * XA) - GX;
                        let XD = ((XB / GX).ln()) - WZ;
                        let XE = XB + XC;
                        let XF = (XE * XE) + ((((V * XC) * XC) - XB) * XD);
                        let XG = WZ + (((XB * XE) * XD) / (XF + (((((XE * XD) * XD) / XF) * XC) * (((XC * XC) * S) - XB))));
                        let XH = if (XG.abs()) < IE { 1.0 } else { 0.0 };
                        let XN;
                        if XH != 0.0 {
                            let XI = XG.exp();
                            XN = XI;
                        } else {
                            let XJ = if XG < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            let XO = if XJ != 0.0 {
                                let XK = IV / (AS + ((-2.3025850929940458e2f64 - XG) * (AS + ((V * (-2.3025850929940458e2f64 - XG)) * (AS + ((-2.3025850929940458e2f64 - XG) * S))))));
                                XK
                            } else {
                                let XL = XG - IE;
                                let XM = IH * (AS + (XL * (AS + ((V * XL) * (AS + (XL * S))))));
                                XM
                            };
                            XN = XO;
                        }
                        let XP = WW - XG;
                        let XQ = (F * XP) + (GX * (XN - AS));
                        let XR = (XP * XP) + (GX * ((XG + AS) - XN));
                        let XS = -(XG + ((F * XR) / (XQ + (((XQ * XQ) - ((BA * (AS - ((V * GX) * XN))) * XR)).sqrt()))));
                        XU = XS;
                    }
                    XT = XU;
                }
                XV = XT;
            } else {
                XV = SY;
            }
            let XW = if XV < IE { 1.0 } else { 0.0 };
            let YD;
            if XW != 0.0 {
                let XX = AS / (XV.exp());
                YD = XX;
            } else {
                let XY = if XV > (HD - IE) { 1.0 } else { 0.0 };
                let YE = if XY != 0.0 {
                    let XZ = HO * ((HD - XV).exp());
                    XZ
                } else {
                    let YA = XV - IE;
                    let YB = IV / (AS + (YA * (AS + ((V * YA) * (AS + (YA * S))))));
                    YB
                };
                YD = YE;
            }
            let YC = if XV < (-HB) { 1.0 } else { 0.0 };
            let YJ;
            if YC != 0.0 {
                let YF = -(((YD + XV) - AS).sqrt());
                YJ = YF;
            } else {
                let YG = if (XV.abs()) <= HB { 1.0 } else { 0.0 };
                let YK = if YG != 0.0 {
                    let YH = (BY * XV) * ((AS - ((S * XV) * (AS - (IZ * XV)))).sqrt());
                    YH
                } else {
                    let YI = ((XV - AS) + YD).sqrt();
                    YI
                };
                YJ = YK;
            }
            let YL = (AP * YJ) * GW;
            let YM = AS + GM;
            let YN = AS + (3.7e-1f64 * X);
            let YO = (((((((1.62e0f64 * YM) * YM) * YN) * YN) * AO) * (AO.sqrt())) * AP) * AP;
            let YP = -YL;
            let YQ = YL - YP;
            let YR = if YQ > FO { 1.0 } else { 0.0 };
            let YY;
            if YR != 0.0 {
                let YS = YP + (V * (YQ + (((YQ * YQ) + YO).sqrt())));
                YY = YS;
            } else {
                let YT = YP - YL;
                let YU = if YT > FO { 1.0 } else { 0.0 };
                let YX = if YU != 0.0 {
                    let YV = YP + ((V * YO) / (YT + (((YT * YT) + YO).sqrt())));
                    YV
                } else {
                    let YW = YP + (V * (YQ + ((FV + YO).sqrt())));
                    YW
                };
                YY = YX;
            }
            let ZA = -RE;
            let ZB = ZA - RE;
            let ZC = if ZB > FO { 1.0 } else { 0.0 };
            let ZJ;
            if ZC != 0.0 {
                let ZD = RE + (V * (ZB + (((ZB * ZB) + YO).sqrt())));
                ZJ = ZD;
            } else {
                let ZE = RE - ZA;
                let ZF = if ZE > FO { 1.0 } else { 0.0 };
                let ZI = if ZF != 0.0 {
                    let ZG = RE + ((V * YO) / (ZE + (((ZE * ZE) + YO).sqrt())));
                    ZG
                } else {
                    let ZH = RE + (V * (ZB + ((FV + YO).sqrt())));
                    ZH
                };
                ZJ = ZI;
            }
            let ZK = YY + (YZ * ZJ);
            let ZL = if GQ > A { 1.0 } else { 0.0 };
            let ZX = if ZL != 0.0 {
                let ZM = E / (AS + (GQ * (((ZK * ZK) + AR).powf(-1.666666666666667e-1f64))));
                ZM
            } else {
                E
            };
            let ZN = HT - QY;
            let ZO = if ZN > FO { 1.0 } else { 0.0 };
            let ZW;
            if ZO != 0.0 {
                let ZQ = HT - (V * (ZN + (((ZN * ZN) + ZP).sqrt())));
                ZW = ZQ;
            } else {
                let ZR = QY - HT;
                let ZS = if ZR > FO { 1.0 } else { 0.0 };
                let ZV = if ZS != 0.0 {
                    let ZT = HT - (5e-3f64 / (ZR + (((ZR * ZR) + ZP).sqrt())));
                    ZT
                } else {
                    let ZU = HT - (V * (ZN + 1e-1f64));
                    ZU
                };
                ZW = ZV;
            }
            let ZY = (EH * ((GV * ZX) * ((AP * ((-1e0f64 * ZW).exp())).sqrt()))) / (AS + (parameters[41] * (V * ((-HK) + (((HK * HK) + 4e-2f64).sqrt())))));
            let ZZ = if EF == F { 1.0 } else { 0.0 };
            let AKB = if ZZ != 0.0 {
                let AAA = BB * ZY;
                AAA
            } else {
                AKC
            };
            let AAB = if (KV * O) == -1e0f64 { 1.0 } else { 0.0 };
            let AAE = if AAB != 0.0 {
                let AAC = KV * BM;
                AAC
            } else {
                A
            };
            let AAD = FL - node_potentials[1];
            let AAF = (O * (AAD - AAE)) * AT;
            let AAH = if AAG > A { 1.0 } else { 0.0 };
            let AAJ = if AAI > A { 1.0 } else { 0.0 };
            let AAK = if AAH != 0.0 || AAJ != 0.0 { 1.0 } else { 0.0 };
            let AAL = if (if EJ != A { 1.0 } else { 0.0 }) != 0.0 && AAK != 0.0 { 1.0 } else { 0.0 };
            let ACI;
            let ADI;
            if AAL != 0.0 {
                let AAM = if (AAF.abs()) <= CH { 1.0 } else { 0.0 };
                let ACD;
                if AAM != 0.0 {
                    let AAN = AAF / CG;
                    ACD = AAN;
                } else {
                    let AAO = if AAF > CH { 1.0 } else { 0.0 };
                    let ACE;
                    if AAO != 0.0 {
                        let AAP = (AAF / CG) * (AS + (((((CG * CI) / HJ) - AS) / HJ) * AAF));
                        let AAQ = if AAP < CJ { 1.0 } else { 0.0 };
                        let AAU = if AAQ != 0.0 {
                            let AAR = (-AAP).exp();
                            AAR
                        } else {
                            let AAS = AAP - CJ;
                            let AAT = CM / (AS + (AAS * (AS + ((V * AAS) * (AS + (AAS * S))))));
                            AAT
                        };
                        let AAV = V * CF;
                        let AAW = (AAF + AAV) - (CE * (((AAF + (IZ * CF)) - (AS - AAU)).sqrt()));
                        let AAX = if AAW < CJ { 1.0 } else { 0.0 };
                        let ABB = if AAX != 0.0 {
                            let AAY = (-AAW).exp();
                            AAY
                        } else {
                            let AAZ = AAW - CJ;
                            let ABA = CM / (AS + (AAZ * (AS + ((V * AAZ) * (AS + (AAZ * S))))));
                            ABA
                        };
                        let ABC = AAF - AAW;
                        let ABD = (F * ABC) + (CF * (AS - ABB));
                        let ABE = (ABC * ABC) - (CF * ((AAW - AS) + ABB));
                        let ABF = AAW + ((F * ABE) / (ABD + (((ABD * ABD) - ((BA * (AS - (AAV * ABB))) * ABE)).sqrt())));
                        ACE = ABF;
                    } else {
                        let ABG = -AAF;
                        let ABH = (CI * ABG) / CG;
                        let ABI = ABH - BS;
                        let ABJ = V * ((ABH + HT) - (((ABI * ABI) + HV).sqrt()));
                        let ABK = ABG - ABJ;
                        let ABL = (ABK * ABK) + (CF * (ABJ + AS));
                        let ABM = (F * ABK) - CF;
                        let ABN = ((ABL / CF).ln()) - ABJ;
                        let ABO = ABL + ABM;
                        let ABP = (ABO * ABO) + ((((V * ABM) * ABM) - ABL) * ABN);
                        let ABQ = ABJ + (((ABL * ABO) * ABN) / (ABP + (((((ABO * ABN) * ABN) / ABP) * ABM) * (((ABM * ABM) * S) - ABL))));
                        let ABR = if (ABQ.abs()) < IE { 1.0 } else { 0.0 };
                        let ABX;
                        if ABR != 0.0 {
                            let ABS = ABQ.exp();
                            ABX = ABS;
                        } else {
                            let ABT = if ABQ < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            let ABY = if ABT != 0.0 {
                                let ABU = IV / (AS + ((-2.3025850929940458e2f64 - ABQ) * (AS + ((V * (-2.3025850929940458e2f64 - ABQ)) * (AS + ((-2.3025850929940458e2f64 - ABQ) * S))))));
                                ABU
                            } else {
                                let ABV = ABQ - IE;
                                let ABW = IH * (AS + (ABV * (AS + ((V * ABV) * (AS + (ABV * S))))));
                                ABW
                            };
                            ABX = ABY;
                        }
                        let ABZ = ABG - ABQ;
                        let ACA = (F * ABZ) + (CF * (ABX - AS));
                        let ACB = (ABZ * ABZ) + (CF * ((ABQ + AS) - ABX));
                        let ACC = -(ABQ + ((F * ACB) / (ACA + (((ACA * ACA) - ((BA * (AS - ((V * CF) * ABX))) * ACB)).sqrt()))));
                        ACE = ACC;
                    }
                    ACD = ACE;
                }
                let ACF = AP * (AAF - ACD);
                ACI = ACF;
                ADI = ACD;
            } else {
                ACI = A;
                ADI = A;
            }
            let AJS;
            let AJV;
            if EJ != 0.0 {
                let AJW;
                if AAK != 0.0 {
                    let ACG = O * AAD;
                    let ACH = if (if KV == AS { 1.0 } else { 0.0 }) != 0.0 && AAJ != 0.0 { 1.0 } else { 0.0 };
                    let AFW;
                    if ACH != 0.0 {
                        let ACK = (O * ACI) + ACJ;
                        let ACL = A - ACK;
                        let ACM = if ACL > FO { 1.0 } else { 0.0 };
                        let ACS;
                        if ACM != 0.0 {
                            let ACN = ACK + (V * (ACL + (((ACL * ACL) + ZP).sqrt())));
                            ACS = ACN;
                        } else {
                            let ACO = if ACK > FO { 1.0 } else { 0.0 };
                            let ACR = if ACO != 0.0 {
                                let ACP = ACK + (5e-3f64 / (ACK + (((ACK * ACK) + ZP).sqrt())));
                                ACP
                            } else {
                                let ACQ = ACK + (V * (ACL + 1e-1f64));
                                ACQ
                            };
                            ACS = ACR;
                        }
                        let ACU = (((ACI * ACI) + GD).sqrt()) * ACT;
                        let ACV = if FC < A { 1.0 } else { 0.0 };
                        let ADU;
                        if ACV != 0.0 {
                            let ACY = ACW - ACU;
                            let ACZ = if ACY > FO { 1.0 } else { 0.0 };
                            let ADG;
                            if ACZ != 0.0 {
                                let ADA = ACW - (V * (ACY + (((ACY * ACY) + GD).sqrt())));
                                ADG = ADA;
                            } else {
                                let ADB = ACU - ACW;
                                let ADC = if ADB > FO { 1.0 } else { 0.0 };
                                let ADF = if ADC != 0.0 {
                                    let ADD = ACW - (5e-7f64 / (ADB + (((ADB * ADB) + GD).sqrt())));
                                    ADD
                                } else {
                                    let ADE = ACW - (V * (ACY + 1e-3f64));
                                    ADE
                                };
                                ADG = ADF;
                            }
                            ADU = ADG;
                        } else {
                            ADU = ACU;
                        }
                        let ADN = if ADH != 0.0 {
                            let ADK = -((O * ADI) + (((BM - ADJ) + ACS) * AT));
                            ADK
                        } else {
                            let ADM = -((O * ADI) + (((BM - ADL) + ACS) * AT));
                            ADM
                        };
                        let ADO = if ADN < IE { 1.0 } else { 0.0 };
                        let AEE = if ADO != 0.0 {
                            let ADP = (AS + (ADN.exp())).ln();
                            ADP
                        } else {
                            ADN
                        };
                        let ADQ = ADN + ((O * ACG) * AT);
                        let ADR = if ADQ < IE { 1.0 } else { 0.0 };
                        let AED = if ADR != 0.0 {
                            let ADS = (AS + (ADQ.exp())).ln();
                            ADS
                        } else {
                            ADQ
                        };
                        let ADV = ADT * (-1.5e0f64 + (ADU * (FE + (FC * ADU))));
                        let ADW = if ADV > A { 1.0 } else { 0.0 };
                        let AEB;
                        if ADW != 0.0 {
                            let ADX = AS + (ADV * (AS + ((V * ADV) * (AS + (ADV * S)))));
                            AEB = ADX;
                        } else {
                            let ADY = if ADV > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            let AEC = if ADY != 0.0 {
                                let ADZ = ADV.exp();
                                ADZ
                            } else {
                                let AEA = IV / (AS + ((-2.3025850929940458e2f64 - ADV) * (AS + ((V * (-2.3025850929940458e2f64 - ADV)) * (AS + ((-2.3025850929940458e2f64 - ADV) * S))))));
                                AEA
                            };
                            AEB = AEC;
                        }
                        let AEF = ((AAI * AEB) * O) * (AED - AEE);
                        AFW = AEF;
                    } else {
                        AFW = A;
                    }
                    let AJX;
                    if AAH != 0.0 {
                        let AEH = (O * ACI) + AEG;
                        let AEI = if AEH > FO { 1.0 } else { 0.0 };
                        let AEP;
                        if AEI != 0.0 {
                            let AEJ = AEH - (V * (AEH + (((AEH * AEH) + ZP).sqrt())));
                            AEP = AEJ;
                        } else {
                            let AEK = A - AEH;
                            let AEL = if AEK > FO { 1.0 } else { 0.0 };
                            let AEO = if AEL != 0.0 {
                                let AEM = AEH - (5e-3f64 / (AEK + (((AEK * AEK) + ZP).sqrt())));
                                AEM
                            } else {
                                let AEN = AEH - (V * (AEH + 1e-1f64));
                                AEN
                            };
                            AEP = AEO;
                        }
                        let AER = (((ACI * ACI) + GD).sqrt()) * AEQ;
                        let AES = if EY < A { 1.0 } else { 0.0 };
                        let AFO;
                        if AES != 0.0 {
                            let AEV = AET - AER;
                            let AEW = if AEV > FO { 1.0 } else { 0.0 };
                            let AFD;
                            if AEW != 0.0 {
                                let AEX = AET - (V * (AEV + (((AEV * AEV) + GD).sqrt())));
                                AFD = AEX;
                            } else {
                                let AEY = AER - AET;
                                let AEZ = if AEY > FO { 1.0 } else { 0.0 };
                                let AFC = if AEZ != 0.0 {
                                    let AFA = AET - (5e-7f64 / (AEY + (((AEY * AEY) + GD).sqrt())));
                                    AFA
                                } else {
                                    let AFB = AET - (V * (AEV + 1e-3f64));
                                    AFB
                                };
                                AFD = AFC;
                            }
                            AFO = AFD;
                        } else {
                            AFO = AER;
                        }
                        let AFH = if AFE != 0.0 {
                            let AFF = (O * ADI) + ((AEP - ADJ) * AT);
                            AFF
                        } else {
                            let AFG = (O * ADI) + ((AEP - ADL) * AT);
                            AFG
                        };
                        let AFI = if AFH < IE { 1.0 } else { 0.0 };
                        let AFZ = if AFI != 0.0 {
                            let AFJ = (AS + (AFH.exp())).ln();
                            AFJ
                        } else {
                            AFH
                        };
                        let AFK = AFH - ((O * ACG) * AT);
                        let AFL = if AFK < IE { 1.0 } else { 0.0 };
                        let AGA = if AFL != 0.0 {
                            let AFM = (AS + (AFK.exp())).ln();
                            AFM
                        } else {
                            AFK
                        };
                        let AFP = AFN * (-1.5e0f64 + (AFO * (FA + (EY * AFO))));
                        let AFQ = if (AFP.abs()) < IE { 1.0 } else { 0.0 };
                        let AFX;
                        if AFQ != 0.0 {
                            let AFR = AFP.exp();
                            AFX = AFR;
                        } else {
                            let AFS = if AFP < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            let AFY = if AFS != 0.0 {
                                let AFT = IV / (AS + ((-2.3025850929940458e2f64 - AFP) * (AS + ((V * (-2.3025850929940458e2f64 - AFP)) * (AS + ((-2.3025850929940458e2f64 - AFP) * S))))));
                                AFT
                            } else {
                                let AFU = AFP - IE;
                                let AFV = IH * (AS + (AFU * (AS + ((V * AFU) * (AS + (AFU * S))))));
                                AFV
                            };
                            AFX = AFY;
                        }
                        let AGB = AFW + (((AAG * AFX) * O) * (AFZ - AGA));
                        AJX = AGB;
                    } else {
                        AJX = AFW;
                    }
                    AJW = AJX;
                } else {
                    AJW = A;
                }
                let AGD = if AGC > A { 1.0 } else { 0.0 };
                let AGF = if AGE > A { 1.0 } else { 0.0 };
                let AGG = if AGD != 0.0 || AGF != 0.0 { 1.0 } else { 0.0 };
                let AJT;
                if AGG != 0.0 {
                    let AGH = O * FM;
                    let AGI = (QW - XV) * AP;
                    let AGJ = if (if KV == AS { 1.0 } else { 0.0 }) != 0.0 && AGF != 0.0 { 1.0 } else { 0.0 };
                    let AJM;
                    if AGJ != 0.0 {
                        let AGK = (O * AGI) + ACJ;
                        let AGL = A - AGK;
                        let AGM = if AGL > FO { 1.0 } else { 0.0 };
                        let AGS;
                        if AGM != 0.0 {
                            let AGN = AGK + (V * (AGL + (((AGL * AGL) + ZP).sqrt())));
                            AGS = AGN;
                        } else {
                            let AGO = if AGK > FO { 1.0 } else { 0.0 };
                            let AGR = if AGO != 0.0 {
                                let AGP = AGK + (5e-3f64 / (AGK + (((AGK * AGK) + ZP).sqrt())));
                                AGP
                            } else {
                                let AGQ = AGK + (V * (AGL + 1e-1f64));
                                AGQ
                            };
                            AGS = AGR;
                        }
                        let AGT = (((AGI * AGI) + GD).sqrt()) * ACT;
                        let AGU = if FC < A { 1.0 } else { 0.0 };
                        let AHO;
                        if AGU != 0.0 {
                            let AGV = ACW - AGT;
                            let AGW = if AGV > FO { 1.0 } else { 0.0 };
                            let AHD;
                            if AGW != 0.0 {
                                let AGX = ACW - (V * (AGV + (((AGV * AGV) + GD).sqrt())));
                                AHD = AGX;
                            } else {
                                let AGY = AGT - ACW;
                                let AGZ = if AGY > FO { 1.0 } else { 0.0 };
                                let AHC = if AGZ != 0.0 {
                                    let AHA = ACW - (5e-7f64 / (AGY + (((AGY * AGY) + GD).sqrt())));
                                    AHA
                                } else {
                                    let AHB = ACW - (V * (AGV + 1e-3f64));
                                    AHB
                                };
                                AHD = AHC;
                            }
                            AHO = AHD;
                        } else {
                            AHO = AGT;
                        }
                        let AHH = if AHE != 0.0 {
                            let AHF = -((O * XV) + (((BM - ADJ) + AGS) * AT));
                            AHF
                        } else {
                            let AHG = -((O * XV) + (((BM - ADL) + AGS) * AT));
                            AHG
                        };
                        let AHI = if AHH < IE { 1.0 } else { 0.0 };
                        let AHY = if AHI != 0.0 {
                            let AHJ = (AS + (AHH.exp())).ln();
                            AHJ
                        } else {
                            AHH
                        };
                        let AHK = AHH + ((O * AGH) * AT);
                        let AHL = if AHK < IE { 1.0 } else { 0.0 };
                        let AHX = if AHL != 0.0 {
                            let AHM = (AS + (AHK.exp())).ln();
                            AHM
                        } else {
                            AHK
                        };
                        let AHP = AHN * (-1.5e0f64 + (AHO * (FE + (FC * AHO))));
                        let AHQ = if AHP > A { 1.0 } else { 0.0 };
                        let AHV;
                        if AHQ != 0.0 {
                            let AHR = AS + (AHP * (AS + ((V * AHP) * (AS + (AHP * S)))));
                            AHV = AHR;
                        } else {
                            let AHS = if AHP > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            let AHW = if AHS != 0.0 {
                                let AHT = AHP.exp();
                                AHT
                            } else {
                                let AHU = IV / (AS + ((-2.3025850929940458e2f64 - AHP) * (AS + ((V * (-2.3025850929940458e2f64 - AHP)) * (AS + ((-2.3025850929940458e2f64 - AHP) * S))))));
                                AHU
                            };
                            AHV = AHW;
                        }
                        let AHZ = ((AGE * AHV) * O) * (AHX - AHY);
                        AJM = AHZ;
                    } else {
                        AJM = A;
                    }
                    let AJU;
                    if AGD != 0.0 {
                        let AIA = (O * AGI) + AEG;
                        let AIB = if AIA > FO { 1.0 } else { 0.0 };
                        let AII;
                        if AIB != 0.0 {
                            let AIC = AIA - (V * (AIA + (((AIA * AIA) + ZP).sqrt())));
                            AII = AIC;
                        } else {
                            let AID = A - AIA;
                            let AIE = if AID > FO { 1.0 } else { 0.0 };
                            let AIH = if AIE != 0.0 {
                                let AIF = AIA - (5e-3f64 / (AID + (((AID * AID) + ZP).sqrt())));
                                AIF
                            } else {
                                let AIG = AIA - (V * (AIA + 1e-1f64));
                                AIG
                            };
                            AII = AIH;
                        }
                        let AIJ = (((AGI * AGI) + GD).sqrt()) * AEQ;
                        let AIK = if EY < A { 1.0 } else { 0.0 };
                        let AJE;
                        if AIK != 0.0 {
                            let AIL = AET - AIJ;
                            let AIM = if AIL > FO { 1.0 } else { 0.0 };
                            let AIT;
                            if AIM != 0.0 {
                                let AIN = AET - (V * (AIL + (((AIL * AIL) + GD).sqrt())));
                                AIT = AIN;
                            } else {
                                let AIO = AIJ - AET;
                                let AIP = if AIO > FO { 1.0 } else { 0.0 };
                                let AIS = if AIP != 0.0 {
                                    let AIQ = AET - (5e-7f64 / (AIO + (((AIO * AIO) + GD).sqrt())));
                                    AIQ
                                } else {
                                    let AIR = AET - (V * (AIL + 1e-3f64));
                                    AIR
                                };
                                AIT = AIS;
                            }
                            AJE = AIT;
                        } else {
                            AJE = AIJ;
                        }
                        let AIX = if AIU != 0.0 {
                            let AIV = (O * XV) + ((AII - ADJ) * AT);
                            AIV
                        } else {
                            let AIW = (O * XV) + ((AII - ADL) * AT);
                            AIW
                        };
                        let AIY = if AIX < IE { 1.0 } else { 0.0 };
                        let AJP = if AIY != 0.0 {
                            let AIZ = (AS + (AIX.exp())).ln();
                            AIZ
                        } else {
                            AIX
                        };
                        let AJA = AIX - ((O * AGH) * AT);
                        let AJB = if AJA < IE { 1.0 } else { 0.0 };
                        let AJQ = if AJB != 0.0 {
                            let AJC = (AS + (AJA.exp())).ln();
                            AJC
                        } else {
                            AJA
                        };
                        let AJF = AJD * (-1.5e0f64 + (AJE * (FA + (EY * AJE))));
                        let AJG = if (AJF.abs()) < IE { 1.0 } else { 0.0 };
                        let AJN;
                        if AJG != 0.0 {
                            let AJH = AJF.exp();
                            AJN = AJH;
                        } else {
                            let AJI = if AJF < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            let AJO = if AJI != 0.0 {
                                let AJJ = IV / (AS + ((-2.3025850929940458e2f64 - AJF) * (AS + ((V * (-2.3025850929940458e2f64 - AJF)) * (AS + ((-2.3025850929940458e2f64 - AJF) * S))))));
                                AJJ
                            } else {
                                let AJK = AJF - IE;
                                let AJL = IH * (AS + (AJK * (AS + ((V * AJK) * (AS + (AJK * S))))));
                                AJL
                            };
                            AJN = AJO;
                        }
                        let AJR = AJM + (((AGC * AJN) * O) * (AJP - AJQ));
                        AJU = AJR;
                    } else {
                        AJU = AJM;
                    }
                    AJT = AJU;
                } else {
                    AJT = A;
                }
                AJS = AJT;
                AJV = AJW;
            } else {
                AJS = A;
                AJV = A;
            }
            if CP != 0.0 {
            } else {
            }
            let AJY = if ((AJS + AJV).abs()) > parameters[65] { 1.0 } else { 0.0 };
            if AJY != 0.0 {
            } else {
            }
            let AKE;
            let AKF;
            let AKG;
            let AKH;
            if EJ != 0.0 {
                let AJZ = 3.2043836e-19f64 * (AJS.abs());
                let AKA = 3.2043836e-19f64 * (AJV.abs());
                AKE = AS;
                AKF = AJZ;
                AKG = AS;
                AKH = AKA;
            } else {
                AKE = A;
                AKF = A;
                AKG = A;
                AKH = A;
            }
            let AKI;
            let AKJ;
            let AKK;
            let AKL;
            let AKM;
            let AKN;
            let AKO;
            let AKP;
            let AKQ;
            let AKR;
            if CP != 0.0 {
                AKI = AS;
                AKJ = DY;
                AKK = AS;
                AKL = EA;
                AKM = AS;
                AKN = EC;
                AKO = AS;
                AKP = EE;
                AKQ = AS;
                AKR = AKB;
            } else {
                AKI = A;
                AKJ = A;
                AKK = A;
                AKL = A;
                AKM = A;
                AKN = A;
                AKO = A;
                AKP = A;
                AKQ = A;
                AKR = A;
            }
            let AKD = if ((node_potentials[0] - node_potentials[2]).abs()) > parameters[10] { 1.0 } else { 0.0 };
            if AKD != 0.0 {
            } else {
            }
            if CP != 0.0 {
            } else {
            }
        if AKE == 0.0 {
            if !visitor.visit(0, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = AKF;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 0, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 0, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(0, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if AKG == 0.0 {
            if !visitor.visit(1, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = AKH;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(1, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if AKI == 0.0 {
            if !visitor.visit(2, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = AKJ;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 2, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 2, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(2, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if AKK == 0.0 {
            if !visitor.visit(3, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = AKL;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 3, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 3, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(3, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if AKM == 0.0 {
            if !visitor.visit(4, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = AKN;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 4, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 4, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(4, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if AKO == 0.0 {
            if !visitor.visit(5, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = AKP;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 5, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 5, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(5, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if AKQ == 0.0 {
            if !visitor.visit(6, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = AKR;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 6, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 6, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(6, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        Ok(())
    }
}
