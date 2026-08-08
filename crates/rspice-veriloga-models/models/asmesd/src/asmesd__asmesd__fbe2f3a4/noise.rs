#![allow(dead_code, non_snake_case, unused_parens, unused_variables)]

use super::state::Instance;
use rspice_veriloga_runtime::GeneratedEvalContext;
pub use rspice_veriloga_runtime::{GeneratedNoiseDescriptor, GeneratedNoiseEndpoint, GeneratedNoiseEvaluation, GeneratedNoiseEvaluationError, GeneratedNoiseEvaluationRef, GeneratedNoiseKind, GeneratedNoiseVisitor};

pub static NOISE_SOURCES: [GeneratedNoiseDescriptor; 6] = [
    GeneratedNoiseDescriptor { mechanism: "WHITE_B_BI_RB", label: Some("Rb"), kind: GeneratedNoiseKind::White, equation: 24, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(1), name: "b", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(5), name: "bi", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_E_EI_RE", label: Some("Re"), kind: GeneratedNoiseKind::White, equation: 27, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(2), name: "e", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(6), name: "ei", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C_CI_RC", label: Some("Rc"), kind: GeneratedNoiseKind::White, equation: 30, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(0), name: "c", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(4), name: "ci", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_BI_EI_FLICKER_IBE", label: Some("flicker_Ibe"), kind: GeneratedNoiseKind::Flicker, equation: 44, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "bi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(6), name: "ei", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_BI_EI_IBE", label: Some("Ibe"), kind: GeneratedNoiseKind::White, equation: 45, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "bi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(6), name: "ei", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_CI_EI_IT", label: Some("It"), kind: GeneratedNoiseKind::White, equation: 46, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(4), name: "ci", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(6), name: "ei", is_internal: true }, table_len: 0, table_log_interp: false },
];

impl Instance {
    pub fn evaluate_noise_sources(&self, ctx: &GeneratedEvalContext<'_>, visitor: &mut dyn GeneratedNoiseVisitor) -> Result<(), GeneratedNoiseEvaluationError> {
        if !self.multiplicity.is_finite() || self.multiplicity <= 0.0 {
            return Err(GeneratedNoiseEvaluationError::InvalidMultiplicity { value: self.multiplicity });
        }
        let parameters = &self.params.values;
        let temperature = ctx.temperature();
        let node_potentials = [ctx.node_voltage(self.nodes[0]), ctx.node_voltage(self.nodes[1]), ctx.node_voltage(self.nodes[2]), ctx.node_voltage(self.nodes[3]), ctx.node_voltage(self.nodes[4]), ctx.node_voltage(self.nodes[5]), ctx.node_voltage(self.nodes[6]), ctx.node_voltage(self.nodes[7]), ctx.node_voltage(self.nodes[8]), ctx.node_voltage(self.nodes[9])];
            let A = 0e0f64;
            let D = 1.7314999999999998e2f64;
            let G = 1.3e3f64;
            let I = 1.7314999999999998e2f64;
            let L = 1e0f64;
            let M = 0.0f64;
            let P = parameters[29];
            let Q = node_potentials[5];
            let R = node_potentials[4];
            let Z = parameters[53];
            let AD = parameters[62];
            let AH = parameters[54];
            let AL = parameters[63];
            let AT = parameters[59];
            let AV = parameters[65];
            let BB = 3.0015e2f64;
            let BH = 1.6021918e-19f64;
            let BM = node_potentials[2];
            let BO = node_potentials[6];
            let BQ = node_potentials[1];
            let BZ = 8e1f64;
            let CF = 3.7e1f64;
            let CS = parameters[8];
            let CV = parameters[4];
            let DE = parameters[57];
            let FK = 2e0f64;
            let FO = parameters[49];
            let FP = parameters[51];
            let FQ = parameters[12];
            let FS = parameters[66];
            let FU = parameters[14];
            let FY = parameters[31];
            let GB = parameters[13];
            let GD = parameters[67];
            let GF = parameters[15];
            let GI = parameters[24];
            let GO = parameters[30];
            let GX = parameters[46];
            let HU = parameters[28];
            let HV = parameters[27];
            let IA = -1e0f64;
            let ID = 3.2043836e-19f64;
            let B = (temperature + node_potentials[3]) + parameters[45];
            let C = if B > 1.7314999999999998e2f64 { 1.0 } else { 0.0 };
            let E = if C != 0.0 {
                B
            } else {
                D
            };
            let F = if 1.3e3f64 < E { 1.0 } else { 0.0 };
            let K;
            if F != 0.0 {
                K = G;
            } else {
                let H = if B > 1.7314999999999998e2f64 { 1.0 } else { 0.0 };
                let J = if H != 0.0 {
                    B
                } else {
                    I
                };
                K = J;
            }
            if M != 0.0 {
            } else {
            }
            let N = if K > parameters[26] { 1.0 } else { 0.0 };
            if N != 0.0 {
            } else {
            }
            let O = parameters[43] * parameters[42];
            let S = P * (Q - R);
            let T = parameters[25] + 2.7315e2f64;
            let U = 8.6170869e-5f64 * K;
            let V = K / T;
            let W = V.ln();
            let X = (parameters[77] * W).exp();
            let Y = (parameters[52] * X) * (L + (parameters[79] * ((-(if S <= A { S } else { A })).powf(parameters[80]))));
            let AA = if Z > A { 1.0 } else { 0.0 };
            let AC = if AA != 0.0 {
                let AB = L / Z;
                AB
            } else {
                A
            };
            let AE = if AD > A { 1.0 } else { 0.0 };
            let AG = if AE != 0.0 {
                let AF = L / AD;
                AF
            } else {
                A
            };
            let AI = if AH > A { 1.0 } else { 0.0 };
            let AK = if AI != 0.0 {
                let AJ = L / AH;
                AJ
            } else {
                A
            };
            let AM = if AL > A { 1.0 } else { 0.0 };
            let AO = if AM != 0.0 {
                let AN = L / AL;
                AN
            } else {
                A
            };
            let AP = V - L;
            let AQ = (parameters[22] * W) + ((parameters[21] * AP) / U);
            let AR = parameters[0] * (AQ.exp());
            let AS = parameters[2] * ((parameters[23] * W).exp());
            let AU = (parameters[58] * ((AQ / AT).exp())) / X;
            let AW = (parameters[64] * ((AQ / AV).exp())) / X;
            let AX = parameters[47] * (L + (parameters[7] * AP));
            let AY = parameters[5] * (L + (parameters[6] * AP));
            let AZ = parameters[9] * (L + (parameters[10] * AP));
            let BA = parameters[56] * (L + (parameters[55] * AP));
            let BC = T / BB;
            let BD = K / BB;
            let BE = (-(1.16e0f64 - (((7.02e-4f64 * K) * K) / (1.108e3f64 + K)))) / (1.3806226e-23f64 * (K + K));
            let BF = -(U + U);
            let BG = 1.5e0f64 * (BD.ln());
            let BI = BF * (BG + (BH * (BE + 1.3454442398941469e20f64)));
            let BJ = (BD * ((parameters[17] - BI) / BC)) + BI;
            let BK = BF * (BG + (BH * (BE + 1.3454442398941469e20f64)));
            let BL = (BD * ((parameters[70] - BK) / BC)) + BK;
            let BN = P * (BM - R);
            let BP = P * (Q - BO);
            let BR = P * (BQ - R);
            let BS = P * (BQ - Q);
            let BT = P * (BM - BO);
            let BU = if AR > A { 1.0 } else { 0.0 };
            let FF;
            if BU != 0.0 {
                let BV = BP / (parameters[1] * U);
                let BW = parameters[11] * U;
                let BX = ((-BP) - AY) / BW;
                let BY = (-AY) / BW;
                let CA = if BV > BZ { 1.0 } else { 0.0 };
                let CC;
                let CD;
                if CA != 0.0 {
                    let CB = L + (BV - BZ);
                    CC = CB;
                    CD = BZ;
                } else {
                    CC = L;
                    CD = BV;
                }
                let CE = CC * (CD.exp());
                let CG = if BX >= CF { 1.0 } else { 0.0 };
                let CO;
                if CG != 0.0 {
                    CO = BX;
                } else {
                    let CH = if BX <= -3.7e1f64 { 1.0 } else { 0.0 };
                    let CP = if CH != 0.0 {
                        let CI = BX.exp();
                        CI
                    } else {
                        let CJ = ((BX.exp()) + L).ln();
                        CJ
                    };
                    CO = CP;
                }
                let CK = if BY >= CF { 1.0 } else { 0.0 };
                let CQ;
                if CK != 0.0 {
                    CQ = BY;
                } else {
                    let CL = if BY <= -3.7e1f64 { 1.0 } else { 0.0 };
                    let CR = if CL != 0.0 {
                        let CM = BY.exp();
                        CM
                    } else {
                        let CN = ((BY.exp()) + L).ln();
                        CN
                    };
                    CQ = CR;
                }
                let CT = (AR * (CE - L)) - ((AX * (CO - CQ)) / (L + (CS * ((BP.abs()).powf(AZ)))));
                FF = CT;
            } else {
                FF = A;
            }
            let CU = if AS > A { 1.0 } else { 0.0 };
            let FG;
            if CU != 0.0 {
                let CW = ((-1e0f64 * BP) * CV) / ((parameters[3] * U) * (if (CV - BP) >= 1e-3f64 { (CV - BP) } else { 1e-3f64 }));
                let CX = if CW > BZ { 1.0 } else { 0.0 };
                let CZ;
                let DA;
                if CX != 0.0 {
                    let CY = L + (CW - BZ);
                    CZ = CY;
                    DA = BZ;
                } else {
                    CZ = L;
                    DA = CW;
                }
                let DB = AS * ((CZ * (DA.exp())) - L);
                FG = DB;
            } else {
                FG = A;
            }
            let DC = if AU > A { 1.0 } else { 0.0 };
            let FH;
            if DC != 0.0 {
                let DD = BP / (AT * U);
                let DF = DE * U;
                let DG = ((-BP) - AY) / DF;
                let DH = (-AY) / DF;
                let DI = if DD > BZ { 1.0 } else { 0.0 };
                let DK;
                let DL;
                if DI != 0.0 {
                    let DJ = L + (DD - BZ);
                    DK = DJ;
                    DL = BZ;
                } else {
                    DK = L;
                    DL = DD;
                }
                let DM = DK * (DL.exp());
                let DN = if DG >= CF { 1.0 } else { 0.0 };
                let DV;
                if DN != 0.0 {
                    DV = DG;
                } else {
                    let DO = if DG <= -3.7e1f64 { 1.0 } else { 0.0 };
                    let DW = if DO != 0.0 {
                        let DP = DG.exp();
                        DP
                    } else {
                        let DQ = ((DG.exp()) + L).ln();
                        DQ
                    };
                    DV = DW;
                }
                let DR = if DH >= CF { 1.0 } else { 0.0 };
                let DX;
                if DR != 0.0 {
                    DX = DH;
                } else {
                    let DS = if DH <= -3.7e1f64 { 1.0 } else { 0.0 };
                    let DY = if DS != 0.0 {
                        let DT = DH.exp();
                        DT
                    } else {
                        let DU = ((DH.exp()) + L).ln();
                        DU
                    };
                    DX = DY;
                }
                let DZ = (AU * (DM - L)) - ((A * (DV - DX)) / (L + (CS * ((BP.abs()).powf(AZ)))));
                FH = DZ;
            } else {
                FH = A;
            }
            let FJ;
            if BU != 0.0 {
                let EA = S / (parameters[61] * U);
                let EB = DE * U;
                let EC = ((-S) - AY) / EB;
                let ED = (-AY) / EB;
                let EE = if EA > BZ { 1.0 } else { 0.0 };
                let EG;
                let EH;
                if EE != 0.0 {
                    let EF = L + (EA - BZ);
                    EG = EF;
                    EH = BZ;
                } else {
                    EG = L;
                    EH = EA;
                }
                let EI = EG * (EH.exp());
                let EJ = if EC >= CF { 1.0 } else { 0.0 };
                let ER;
                if EJ != 0.0 {
                    ER = EC;
                } else {
                    let EK = if EC <= -3.7e1f64 { 1.0 } else { 0.0 };
                    let ES = if EK != 0.0 {
                        let EL = EC.exp();
                        EL
                    } else {
                        let EM = ((EC.exp()) + L).ln();
                        EM
                    };
                    ER = ES;
                }
                let EN = if ED >= CF { 1.0 } else { 0.0 };
                let ET;
                if EN != 0.0 {
                    ET = ED;
                } else {
                    let EO = if ED <= -3.7e1f64 { 1.0 } else { 0.0 };
                    let EU = if EO != 0.0 {
                        let EP = ED.exp();
                        EP
                    } else {
                        let EQ = ((ED.exp()) + L).ln();
                        EQ
                    };
                    ET = EU;
                }
                let EV = (AR * (EI - L)) - ((BA * (ER - ET)) / (L + (CS * ((S.abs()).powf(AZ)))));
                FJ = EV;
            } else {
                FJ = A;
            }
            let EW = if AW > A { 1.0 } else { 0.0 };
            if EW != 0.0 {
                let EX = DE * U;
                let EY = ((-S) - AY) / EX;
                let EZ = (-AY) / EX;
                let FA = if (S / (AV * U)) > BZ { 1.0 } else { 0.0 };
                if FA != 0.0 {
                } else {
                }
                let FB = if EY >= CF { 1.0 } else { 0.0 };
                if FB != 0.0 {
                } else {
                    let FC = if EY <= -3.7e1f64 { 1.0 } else { 0.0 };
                    if FC != 0.0 {
                    } else {
                    }
                }
                let FD = if EZ >= CF { 1.0 } else { 0.0 };
                if FD != 0.0 {
                } else {
                    let FE = if EZ <= -3.7e1f64 { 1.0 } else { 0.0 };
                    if FE != 0.0 {
                    } else {
                    }
                }
            } else {
            }
            let FI = ((FF - FG) / Y) + FH;
            let FL = (FK * ((L - (BP * AG)) - (S * AC))) / (L + (((L + (4e0f64 * ((FF * (AK * (L + (S * parameters[81])))) + (FJ * AO)))).abs()).powf(parameters[82])));
            let FM = FJ * FL;
            let FN = FF * FL;
            let FR = (FQ * ((W * parameters[37]).exp())) * ((L + (((BS / parameters[48]).abs()).powf(FO))).powf((L / FO)));
            let FT = FS * ((W * parameters[78]).exp());
            let FV = (FU * ((W * parameters[38]).exp())) * ((L + (((BT / parameters[50]).abs()).powf(FP))).powf((L / FP)));
            let FW = if parameters[32] == L { 1.0 } else { 0.0 };
            let GA = if FW != 0.0 {
                let FX = FR / (L + (((node_potentials[8].abs()) / parameters[20]).powf(parameters[44])));
                FX
            } else {
                FR
            };
            let FZ = if FY == L { 1.0 } else { 0.0 };
            let GZ;
            let HG;
            let HN;
            if FZ != 0.0 {
                let GC = GA + GB;
                let GE = FT + GD;
                let GG = FV + GF;
                GZ = GC;
                HG = GG;
                HN = GE;
            } else {
                GZ = GA;
                HG = FV;
                HN = FT;
            }
            let GH = if BN <= A { 1.0 } else { 0.0 };
            if GH != 0.0 {
            } else {
            }
            let GJ = if (BP + ((-BJ) * GI)) > A { 1.0 } else { 0.0 };
            if GJ != 0.0 {
            } else {
            }
            let GK = (-BL) * GI;
            let GL = if (BR + GK) > A { 1.0 } else { 0.0 };
            if GL != 0.0 {
            } else {
            }
            let GM = if (S + GK) > A { 1.0 } else { 0.0 };
            if GM != 0.0 {
            } else {
            }
            let GN = if (if parameters[68] != A { 1.0 } else { 0.0 }) != 0.0 && (if parameters[19] != A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            if GN != 0.0 {
            } else {
            }
            let GP = if parameters[33] > A { 1.0 } else { 0.0 };
            let GQ = if (if GO == L { 1.0 } else { 0.0 }) != 0.0 && GP != 0.0 { 1.0 } else { 0.0 };
            if GQ != 0.0 {
            } else {
                let GR = if (if (if GO == FK { 1.0 } else { 0.0 }) != 0.0 && GP != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if parameters[35] > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                if GR != 0.0 {
                } else {
                    let GS = if GO == -1e0f64 { 1.0 } else { 0.0 };
                    if GS != 0.0 {
                    } else {
                    }
                }
            }
            let GT = 5.5224904e-23f64 * K;
            let GU = (FQ + (FY * GB)) / O;
            let GV = (FU + (FY * GF)) / O;
            let GW = (FS + (FY * GD)) / O;
            let GY = if (if GU > A { 1.0 } else { 0.0 }) != 0.0 && (if GU >= GX { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let IG;
            let IH;
            if GY != 0.0 {
                let HA = GZ / O;
                let HB = if HA > GX { 1.0 } else { 0.0 };
                if HB != 0.0 {
                } else {
                }
                let HC = if HA >= GX { 1.0 } else { 0.0 };
                let HE = if HC != 0.0 {
                    let HD = GT / HA;
                    HD
                } else {
                    A
                };
                IG = L;
                IH = HE;
            } else {
                IG = A;
                IH = A;
            }
            let HF = if (if GV > A { 1.0 } else { 0.0 }) != 0.0 && (if GV >= GX { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let II;
            let IJ;
            if HF != 0.0 {
                let HH = HG / O;
                let HI = if HH > GX { 1.0 } else { 0.0 };
                if HI != 0.0 {
                } else {
                }
                let HJ = if HH >= GX { 1.0 } else { 0.0 };
                let HL = if HJ != 0.0 {
                    let HK = GT / HH;
                    HK
                } else {
                    A
                };
                II = L;
                IJ = HL;
            } else {
                II = A;
                IJ = A;
            }
            let HM = if (if GW > A { 1.0 } else { 0.0 }) != 0.0 && (if GW >= GX { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let IK;
            let IL;
            if HM != 0.0 {
                let HO = HN / O;
                let HP = if HO > GX { 1.0 } else { 0.0 };
                if HP != 0.0 {
                } else {
                }
                let HQ = if HO >= GX { 1.0 } else { 0.0 };
                let HS = if HQ != 0.0 {
                    let HR = GT / HO;
                    HR
                } else {
                    A
                };
                IK = L;
                IL = HS;
            } else {
                IK = A;
                IL = A;
            }
            let HT = P * FI;
            let HW = if (if (if HU > A { 1.0 } else { 0.0 }) != 0.0 && (if HV > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) > A { 1.0 } else { 0.0 };
            let HY = if HW != 0.0 {
                let HX = HV * ((FI.abs()).powf(HU));
                HX
            } else {
                A
            };
            let HZ = if HT >= A { 1.0 } else { 0.0 };
            let IB = if HZ != 0.0 {
                L
            } else {
                IA
            };
            let IC = IB * HY;
            let IE = ID * (FI.abs());
            let IF = ID * ((FN - FM).abs());
        if IG == 0.0 {
            if !visitor.visit(0, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = IH;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 0, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 0, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(0, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if II == 0.0 {
            if !visitor.visit(1, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = IJ;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(1, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if IK == 0.0 {
            if !visitor.visit(2, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = IL;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 2, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 2, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(2, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = IC;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 3, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = Some(L);
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 3, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 3, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(3, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = IE;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 4, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 4, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(4, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = IF;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 5, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 5, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(5, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        Ok(())
    }
}
