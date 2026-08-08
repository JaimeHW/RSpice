#![allow(dead_code, non_snake_case, unused_parens, unused_variables)]

use super::state::Instance;
use rspice_veriloga_runtime::GeneratedEvalContext;
pub use rspice_veriloga_runtime::{GeneratedNoiseDescriptor, GeneratedNoiseEndpoint, GeneratedNoiseEvaluation, GeneratedNoiseEvaluationError, GeneratedNoiseEvaluationRef, GeneratedNoiseKind, GeneratedNoiseVisitor};

pub static NOISE_SOURCES: [GeneratedNoiseDescriptor; 2] = [
    GeneratedNoiseDescriptor { mechanism: "WHITE_N1_N2_THERMAL", label: Some("thermal"), kind: GeneratedNoiseKind::White, equation: 1, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(0), name: "n1", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(1), name: "n2", is_internal: false }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_N1_N2_FLICKER", label: Some("flicker"), kind: GeneratedNoiseKind::Flicker, equation: 2, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(0), name: "n1", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(1), name: "n2", is_internal: false }, table_len: 0, table_log_interp: false },
];

impl Instance {
    pub fn evaluate_noise_sources(&self, ctx: &GeneratedEvalContext<'_>, visitor: &mut dyn GeneratedNoiseVisitor) -> Result<(), GeneratedNoiseEvaluationError> {
        if !self.multiplicity.is_finite() || self.multiplicity <= 0.0 {
            return Err(GeneratedNoiseEvaluationError::InvalidMultiplicity { value: self.multiplicity });
        }
        let parameters = &self.params.values;
        let parameter_given = &*self.param_given;
        let multiplicity = self.multiplicity;
        let temperature = ctx.temperature();
        let node_potentials = [ctx.node_voltage(self.nodes[0]), ctx.node_voltage(self.nodes[1])];
            let A = 0e0f64;
            let C = if parameter_given[9] { 1.0 } else { 0.0 };
            let D = parameters[9];
            let E = 1e0f64;
            let G = if parameter_given[10] { 1.0 } else { 0.0 };
            let H = 1e-2f64;
            let K = if parameter_given[13] { 1.0 } else { 0.0 };
            let L = parameters[13];
            let Q = 2.7315e2f64;
            let V = parameters[34];
            let Y = parameters[35];
            let AH = parameters[3];
            let AI = parameters[4];
            let AK = parameters[22];
            let AM = 5e-1f64;
            let AO = if parameter_given[1] { 1.0 } else { 0.0 };
            let AP = if parameter_given[2] { 1.0 } else { 0.0 };
            let AR = parameters[2];
            let AS = parameters[1];
            let AU = parameters[0];
            let AW = parameters[21];
            let AY = 1e99f64;
            let BF = parameters[16];
            let DN = parameters[24];
            let DW = parameters[23];
            let EM = parameters[28];
            let EN = parameters[26];
            let EQ = parameters[36];
            let ER = parameters[37];
            let ET = parameters[38];
            let EV = parameters[39];
            let HA = parameters[30];
            let HL = parameters[31];
            let B = if parameters[14] != 1.002e3f64 { 1.0 } else { 0.0 };
            if B != 0.0 {
            } else {
            }
            let O = if C != 0.0 {
                D
            } else {
                let F = ctx.simparam_or("scale", E);
                F
            };
            let N = if G != 0.0 {
                let I = E - (H * parameters[10]);
                I
            } else {
                let J = E - (H * (ctx.simparam_or("shrink", A)));
                J
            };
            let FR = if K != 0.0 {
                L
            } else {
                let M = ctx.simparam_or("rthresh", 1e-3f64);
                M
            };
            let P = (N * O) * 1e6f64;
            let R = Q + parameters[15];
            let S = (temperature + parameters[5]) - Q;
            let T = if S < parameters[11] { 1.0 } else { 0.0 };
            if T != 0.0 {
            } else {
            }
            let U = if S > parameters[12] { 1.0 } else { 0.0 };
            if U != 0.0 {
            } else {
            }
            let W = if S < (V + E) { 1.0 } else { 0.0 };
            let AB;
            if W != 0.0 {
                let X = V + (((S - V) - E).exp());
                AB = X;
            } else {
                let Z = if S > (Y - E) { 1.0 } else { 0.0 };
                let AC = if Z != 0.0 {
                    let AA = Y - (((Y - S) - E).exp());
                    AA
                } else {
                    S
                };
                AB = AC;
            }
            let AD = AB + Q;
            let AE = AD - R;
            let AF = (E + (AE * parameters[42])) * parameters[29];
            let AG = if AF < A { 1.0 } else { 0.0 };
            let GZ = if AG != 0.0 {
                A
            } else {
                AF
            };
            let AJ = if AH != 0.0 && AI != 0.0 { 1.0 } else { 0.0 };
            let BA;
            if AJ != 0.0 {
                BA = AK;
            } else {
                let AL = if AH != 0.0 || AI != 0.0 { 1.0 } else { 0.0 };
                let BB = if AL != 0.0 {
                    let AN = AK * AM;
                    AN
                } else {
                    A
                };
                BA = BB;
            }
            let AQ = if (if AO != 0.0 && AP != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if (if parameter_given[0] { 1.0 } else { 0.0 }) == 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let CT;
            let DD;
            let DO;
            let EA;
            let FA;
            let GA;
            if AQ != 0.0 {
                let AT = if (if AR == A { 1.0 } else { 0.0 }) != 0.0 || (if AS == A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let CU;
                let DE;
                let DP;
                let EB;
                let FB;
                let GB;
                if AT != 0.0 {
                    let AV = AU * P;
                    let AX = AV + AW;
                    CU = A;
                    DE = AV;
                    DP = A;
                    EB = A;
                    FB = AX;
                    GB = AY;
                } else {
                    let AZ = AS * P;
                    let BC = AZ + BA;
                    let BD = if BC < A { 1.0 } else { 0.0 };
                    if BD != 0.0 {
                    } else {
                    }
                    let BE = if BC > A { 1.0 } else { 0.0 };
                    let DF;
                    let EC;
                    let FC;
                    let GC;
                    if BE != 0.0 {
                        let BG = (BF / AR) * BC;
                        let BH = BG - AW;
                        let BI = if BH <= A { 1.0 } else { 0.0 };
                        if BI != 0.0 {
                        } else {
                        }
                        let BJ = E / AR;
                        DF = BH;
                        EC = AR;
                        FC = BG;
                        GC = BJ;
                    } else {
                        let BK = AU * P;
                        let BL = BK + AW;
                        DF = BK;
                        EC = A;
                        FC = BL;
                        GC = AY;
                    }
                    CU = AZ;
                    DE = DF;
                    DP = BC;
                    EB = EC;
                    FB = FC;
                    GB = GC;
                }
                CT = CU;
                DD = DE;
                DO = DP;
                EA = EB;
                FA = FB;
                GA = GB;
            } else {
                let BM = if AP != 0.0 && (if AO == 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let CV;
                let DG;
                let DQ;
                let ED;
                let FD;
                let GD;
                if BM != 0.0 {
                    let BN = if AR == A { 1.0 } else { 0.0 };
                    let CW;
                    let DH;
                    let DR;
                    let EE;
                    let FE;
                    let GE;
                    if BN != 0.0 {
                        let BO = AU * P;
                        let BP = BO + AW;
                        CW = A;
                        DH = BO;
                        DR = A;
                        EE = A;
                        FE = BP;
                        GE = AY;
                    } else {
                        let BQ = if AU == A { 1.0 } else { 0.0 };
                        let CX;
                        let DI;
                        let DS;
                        let EF;
                        let FF;
                        let GF;
                        if BQ != 0.0 {
                            let BR = AS * P;
                            let BS = BR + BA;
                            CX = BR;
                            DI = A;
                            DS = BS;
                            EF = AY;
                            FF = A;
                            GF = A;
                        } else {
                            let BT = AU * P;
                            let BU = BT + AW;
                            let BV = if BU < A { 1.0 } else { 0.0 };
                            if BV != 0.0 {
                            } else {
                            }
                            let BW = if BU > A { 1.0 } else { 0.0 };
                            let CY;
                            let DT;
                            let EG;
                            let GG;
                            if BW != 0.0 {
                                let BX = (AR / BF) * BU;
                                let BY = BX - BA;
                                let BZ = if BY <= A { 1.0 } else { 0.0 };
                                if BZ != 0.0 {
                                } else {
                                }
                                let CA = E / AR;
                                CY = BY;
                                DT = BX;
                                EG = AR;
                                GG = CA;
                            } else {
                                let CB = AS * P;
                                let CC = CB + BA;
                                CY = CB;
                                DT = CC;
                                EG = AY;
                                GG = A;
                            }
                            CX = CY;
                            DI = BT;
                            DS = DT;
                            EF = EG;
                            FF = BU;
                            GF = GG;
                        }
                        CW = CX;
                        DH = DI;
                        DR = DS;
                        EE = EF;
                        FE = FF;
                        GE = GF;
                    }
                    CV = CW;
                    DG = DH;
                    DQ = DR;
                    ED = EE;
                    FD = FE;
                    GD = GE;
                } else {
                    let CD = if AU == A { 1.0 } else { 0.0 };
                    let CZ;
                    let DJ;
                    let DU;
                    let EH;
                    let FG;
                    let GH;
                    if CD != 0.0 {
                        let CE = AS * P;
                        let CF = CE + BA;
                        CZ = CE;
                        DJ = A;
                        DU = CF;
                        EH = AY;
                        FG = A;
                        GH = A;
                    } else {
                        let CG = if AS == A { 1.0 } else { 0.0 };
                        let DA;
                        let DK;
                        let DV;
                        let EI;
                        let FH;
                        let GI;
                        if CG != 0.0 {
                            let CH = AU * P;
                            let CI = CH + AW;
                            DA = A;
                            DK = CH;
                            DV = A;
                            EI = A;
                            FH = CI;
                            GI = AY;
                        } else {
                            let CJ = AU * P;
                            let CK = CJ + AW;
                            let CL = if CK < A { 1.0 } else { 0.0 };
                            if CL != 0.0 {
                            } else {
                            }
                            let CM = AS * P;
                            let CN = CM + BA;
                            let CO = if CK > A { 1.0 } else { 0.0 };
                            let EJ;
                            let GJ;
                            if CO != 0.0 {
                                let CP = if CN < A { 1.0 } else { 0.0 };
                                if CP != 0.0 {
                                } else {
                                }
                                let CQ = if CN > A { 1.0 } else { 0.0 };
                                let EK;
                                let GK;
                                if CQ != 0.0 {
                                    let CR = BF * (CN / CK);
                                    let CS = E / CR;
                                    EK = CR;
                                    GK = CS;
                                } else {
                                    EK = A;
                                    GK = AY;
                                }
                                EJ = EK;
                                GJ = GK;
                            } else {
                                EJ = AY;
                                GJ = A;
                            }
                            DA = CM;
                            DK = CJ;
                            DV = CN;
                            EI = EJ;
                            FH = CK;
                            GI = GJ;
                        }
                        CZ = DA;
                        DJ = DK;
                        DU = DV;
                        EH = EI;
                        FG = FH;
                        GH = GI;
                    }
                    CV = CZ;
                    DG = DJ;
                    DQ = DU;
                    ED = EH;
                    FD = FG;
                    GD = GH;
                }
                CT = CV;
                DD = DG;
                DO = DQ;
                EA = ED;
                FA = FD;
                GA = GD;
            }
            let DB = if CT < parameters[17] { 1.0 } else { 0.0 };
            if DB != 0.0 {
            } else {
            }
            let DC = if CT > parameters[18] { 1.0 } else { 0.0 };
            if DC != 0.0 {
            } else {
            }
            let DL = if DD < parameters[19] { 1.0 } else { 0.0 };
            if DL != 0.0 {
            } else {
            }
            let DM = if DD > parameters[20] { 1.0 } else { 0.0 };
            if DM != 0.0 {
            } else {
            }
            let DZ = if DN != 0.0 {
                let DX = DO + DW;
                DX
            } else {
                let DY = CT + DW;
                DY
            };
            let EL = if EA > A { 1.0 } else { 0.0 };
            let EO = if (if EM > A { 1.0 } else { 0.0 }) != 0.0 || (if EN > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let EP = if (if (if DZ <= A { 1.0 } else { 0.0 }) != 0.0 && EL != 0.0 { 1.0 } else { 0.0 }) != 0.0 && EO != 0.0 { 1.0 } else { 0.0 };
            if EP != 0.0 {
            } else {
            }
            let ES = if DO > A { 1.0 } else { 0.0 };
            let FJ;
            let FN;
            if ES != 0.0 {
                let FK;
                let FO;
                if AJ != 0.0 {
                    let EU = EQ + (ET / DO);
                    let EW = ER + (EV / DO);
                    FK = EU;
                    FO = EW;
                } else {
                    let EX = if AH != 0.0 || AI != 0.0 { 1.0 } else { 0.0 };
                    let FL;
                    let FP;
                    if EX != 0.0 {
                        let EY = EQ + ((AM * ET) / DO);
                        let EZ = ER + ((AM * EV) / DO);
                        FL = EY;
                        FP = EZ;
                    } else {
                        FL = EQ;
                        FP = ER;
                    }
                    FK = FL;
                    FO = FP;
                }
                FJ = FK;
                FN = FO;
            } else {
                FJ = EQ;
                FN = ER;
            }
            let FI = if FA > A { 1.0 } else { 0.0 };
            let FT;
            let FU;
            if FI != 0.0 {
                let FM = FJ + (parameters[40] / FA);
                let FQ = FN + (parameters[41] / FA);
                FT = FM;
                FU = FQ;
            } else {
                FT = FJ;
                FU = FN;
            }
            let FS = if EA > (FR / multiplicity) { 1.0 } else { 0.0 };
            if FS != 0.0 {
            } else {
            }
            let FV = E + (AE * (FT + (AE * FU)));
            let FW = if FV < 1.1e-1f64 { 1.0 } else { 0.0 };
            let FY = if FW != 0.0 {
                let FX = H + (1e-1f64 * (((1e1f64 * (FV - H)) - E).exp()));
                FX
            } else {
                FV
            };
            let FZ = EA * FY;
            let GL = GA / FY;
            let GM = node_potentials[0] - node_potentials[1];
            let GN = if EL != 0.0 && EO != 0.0 { 1.0 } else { 0.0 };
            let GS = if GN != 0.0 {
                let GO = GM / DZ;
                let GP = parameters[27] * GO;
                let GQ = parameters[25] * (GO.abs());
                let GR = (((E - EM) - EN) + (EM * ((E + (GP * GP)).sqrt()))) + (EN * ((E + ((GQ * GQ) * GQ)).powf(3.333333333333333e-1f64)));
                GR
            } else {
                E
            };
            let GT = GM / (FZ * GS);
            if FI != 0.0 {
                let GU = if ((GT / FA).abs()) > parameters[33] { 1.0 } else { 0.0 };
                if GU != 0.0 {
                } else {
                }
            } else {
            }
            let GV = if GA > A { 1.0 } else { 0.0 };
            let GW = if (if parameters[6] != 0.0 && EL != 0.0 { 1.0 } else { 0.0 }) != 0.0 && GV != 0.0 { 1.0 } else { 0.0 };
            let HI;
            let HJ;
            if GW != 0.0 {
                let GX = ((5.522602e-23f64 * AD) * GL) / GS;
                let GY = if (if parameters[32] != 0.0 && ES != 0.0 { 1.0 } else { 0.0 }) != 0.0 && FI != 0.0 { 1.0 } else { 0.0 };
                let HF;
                if GY != 0.0 {
                    let HB = ((GZ * (((GT / FA).abs()).powf(HA))) * FA) / DO;
                    HF = HB;
                } else {
                    let HC = if (if CT > A { 1.0 } else { 0.0 }) != 0.0 && (if DD > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let HG = if HC != 0.0 {
                        let HD = ((GZ * (((GT / DD).abs()).powf(HA))) * DD) / CT;
                        HD
                    } else {
                        A
                    };
                    HF = HG;
                }
                let HE = if GT < A { 1.0 } else { 0.0 };
                let HK = if HE != 0.0 {
                    let HH = -HF;
                    HH
                } else {
                    HF
                };
                HI = GX;
                HJ = HK;
            } else {
                HI = A;
                HJ = A;
            }
            let HM = if EL != 0.0 && GV != 0.0 { 1.0 } else { 0.0 };
            if HM != 0.0 {
                let HN = if 0e0f64 != A { 1.0 } else { 0.0 };
                if HN != 0.0 {
                } else {
                }
            } else {
            }
        {
            let psd = HI;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 0, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 0, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(0, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = HJ;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = Some(HL);
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(1, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        Ok(())
    }
}
