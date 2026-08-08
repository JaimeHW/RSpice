#![allow(dead_code, non_snake_case, unused_parens, unused_variables)]

use super::state::Instance;
use rspice_veriloga_runtime::GeneratedEvalContext;
pub use rspice_veriloga_runtime::{GeneratedNoiseDescriptor, GeneratedNoiseEndpoint, GeneratedNoiseEvaluation, GeneratedNoiseEvaluationError, GeneratedNoiseEvaluationRef, GeneratedNoiseKind, GeneratedNoiseVisitor};

pub static NOISE_SOURCES: [GeneratedNoiseDescriptor; 2] = [
    GeneratedNoiseDescriptor { mechanism: "WHITE_N1_N2_THERMAL", label: Some("thermal"), kind: GeneratedNoiseKind::White, equation: 5, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(0), name: "n1", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(1), name: "n2", is_internal: false }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_N1_N2_FLICKER", label: Some("flicker"), kind: GeneratedNoiseKind::Flicker, equation: 6, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(0), name: "n1", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(1), name: "n2", is_internal: false }, table_len: 0, table_log_interp: false },
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
        let node_potentials = [ctx.node_voltage(self.nodes[0]), ctx.node_voltage(self.nodes[1]), ctx.node_voltage(self.nodes[2])];
            let A = 0e0f64;
            let C = if parameter_given[10] { 1.0 } else { 0.0 };
            let D = parameters[10];
            let E = 1e0f64;
            let G = if parameter_given[11] { 1.0 } else { 0.0 };
            let H = 1e-2f64;
            let K = if parameter_given[14] { 1.0 } else { 0.0 };
            let L = parameters[14];
            let Q = 2.7315e2f64;
            let V = parameters[3];
            let W = parameters[4];
            let Y = parameters[23];
            let AA = 5e-1f64;
            let AC = if parameter_given[1] { 1.0 } else { 0.0 };
            let AD = if parameter_given[2] { 1.0 } else { 0.0 };
            let AF = parameters[2];
            let AG = parameters[1];
            let AI = parameters[0];
            let AK = parameters[22];
            let AM = 1e99f64;
            let AT = parameters[17];
            let DB = parameters[25];
            let DK = parameters[24];
            let EA = parameters[29];
            let EB = parameters[27];
            let EE = parameters[37];
            let EF = parameters[38];
            let EH = parameters[39];
            let EJ = parameters[40];
            let FH = 2e0f64;
            let FP = parameters[7];
            let FR = parameters[35];
            let FU = parameters[36];
            let HK = parameters[31];
            let HV = parameters[32];
            let B = if parameters[15] != 1.002e3f64 { 1.0 } else { 0.0 };
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
                let I = E - (H * parameters[11]);
                I
            } else {
                let J = E - (H * (ctx.simparam_or("shrink", A)));
                J
            };
            let FF = if K != 0.0 {
                L
            } else {
                let M = ctx.simparam_or("rthresh", 1e-3f64);
                M
            };
            let P = (N * O) * 1e6f64;
            let R = Q + parameters[16];
            let S = (temperature + parameters[5]) - Q;
            let T = if S < parameters[12] { 1.0 } else { 0.0 };
            if T != 0.0 {
            } else {
            }
            let U = if S > parameters[13] { 1.0 } else { 0.0 };
            if U != 0.0 {
            } else {
            }
            let X = if V != 0.0 && W != 0.0 { 1.0 } else { 0.0 };
            let AO;
            if X != 0.0 {
                AO = Y;
            } else {
                let Z = if V != 0.0 || W != 0.0 { 1.0 } else { 0.0 };
                let AP = if Z != 0.0 {
                    let AB = Y * AA;
                    AB
                } else {
                    A
                };
                AO = AP;
            }
            let AE = if (if AC != 0.0 && AD != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if (if parameter_given[0] { 1.0 } else { 0.0 }) == 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let CH;
            let CR;
            let DC;
            let DO;
            let EO;
            let GI;
            if AE != 0.0 {
                let AH = if (if AF == A { 1.0 } else { 0.0 }) != 0.0 || (if AG == A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let CI;
                let CS;
                let DD;
                let DP;
                let EP;
                let GJ;
                if AH != 0.0 {
                    let AJ = AI * P;
                    let AL = AJ + AK;
                    CI = A;
                    CS = AJ;
                    DD = A;
                    DP = A;
                    EP = AL;
                    GJ = AM;
                } else {
                    let AN = AG * P;
                    let AQ = AN + AO;
                    let AR = if AQ < A { 1.0 } else { 0.0 };
                    if AR != 0.0 {
                    } else {
                    }
                    let AS = if AQ > A { 1.0 } else { 0.0 };
                    let CT;
                    let DQ;
                    let EQ;
                    let GK;
                    if AS != 0.0 {
                        let AU = (AT / AF) * AQ;
                        let AV = AU - AK;
                        let AW = if AV <= A { 1.0 } else { 0.0 };
                        if AW != 0.0 {
                        } else {
                        }
                        let AX = E / AF;
                        CT = AV;
                        DQ = AF;
                        EQ = AU;
                        GK = AX;
                    } else {
                        let AY = AI * P;
                        let AZ = AY + AK;
                        CT = AY;
                        DQ = A;
                        EQ = AZ;
                        GK = AM;
                    }
                    CI = AN;
                    CS = CT;
                    DD = AQ;
                    DP = DQ;
                    EP = EQ;
                    GJ = GK;
                }
                CH = CI;
                CR = CS;
                DC = DD;
                DO = DP;
                EO = EP;
                GI = GJ;
            } else {
                let BA = if AD != 0.0 && (if AC == 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let CJ;
                let CU;
                let DE;
                let DR;
                let ER;
                let GL;
                if BA != 0.0 {
                    let BB = if AF == A { 1.0 } else { 0.0 };
                    let CK;
                    let CV;
                    let DF;
                    let DS;
                    let ES;
                    let GM;
                    if BB != 0.0 {
                        let BC = AI * P;
                        let BD = BC + AK;
                        CK = A;
                        CV = BC;
                        DF = A;
                        DS = A;
                        ES = BD;
                        GM = AM;
                    } else {
                        let BE = if AI == A { 1.0 } else { 0.0 };
                        let CL;
                        let CW;
                        let DG;
                        let DT;
                        let ET;
                        let GN;
                        if BE != 0.0 {
                            let BF = AG * P;
                            let BG = BF + AO;
                            CL = BF;
                            CW = A;
                            DG = BG;
                            DT = AM;
                            ET = A;
                            GN = A;
                        } else {
                            let BH = AI * P;
                            let BI = BH + AK;
                            let BJ = if BI < A { 1.0 } else { 0.0 };
                            if BJ != 0.0 {
                            } else {
                            }
                            let BK = if BI > A { 1.0 } else { 0.0 };
                            let CM;
                            let DH;
                            let DU;
                            let GO;
                            if BK != 0.0 {
                                let BL = (AF / AT) * BI;
                                let BM = BL - AO;
                                let BN = if BM <= A { 1.0 } else { 0.0 };
                                if BN != 0.0 {
                                } else {
                                }
                                let BO = E / AF;
                                CM = BM;
                                DH = BL;
                                DU = AF;
                                GO = BO;
                            } else {
                                let BP = AG * P;
                                let BQ = BP + AO;
                                CM = BP;
                                DH = BQ;
                                DU = AM;
                                GO = A;
                            }
                            CL = CM;
                            CW = BH;
                            DG = DH;
                            DT = DU;
                            ET = BI;
                            GN = GO;
                        }
                        CK = CL;
                        CV = CW;
                        DF = DG;
                        DS = DT;
                        ES = ET;
                        GM = GN;
                    }
                    CJ = CK;
                    CU = CV;
                    DE = DF;
                    DR = DS;
                    ER = ES;
                    GL = GM;
                } else {
                    let BR = if AI == A { 1.0 } else { 0.0 };
                    let CN;
                    let CX;
                    let DI;
                    let DV;
                    let EU;
                    let GP;
                    if BR != 0.0 {
                        let BS = AG * P;
                        let BT = BS + AO;
                        CN = BS;
                        CX = A;
                        DI = BT;
                        DV = AM;
                        EU = A;
                        GP = A;
                    } else {
                        let BU = if AG == A { 1.0 } else { 0.0 };
                        let CO;
                        let CY;
                        let DJ;
                        let DW;
                        let EV;
                        let GQ;
                        if BU != 0.0 {
                            let BV = AI * P;
                            let BW = BV + AK;
                            CO = A;
                            CY = BV;
                            DJ = A;
                            DW = A;
                            EV = BW;
                            GQ = AM;
                        } else {
                            let BX = AI * P;
                            let BY = BX + AK;
                            let BZ = if BY < A { 1.0 } else { 0.0 };
                            if BZ != 0.0 {
                            } else {
                            }
                            let CA = AG * P;
                            let CB = CA + AO;
                            let CC = if BY > A { 1.0 } else { 0.0 };
                            let DX;
                            let GR;
                            if CC != 0.0 {
                                let CD = if CB < A { 1.0 } else { 0.0 };
                                if CD != 0.0 {
                                } else {
                                }
                                let CE = if CB > A { 1.0 } else { 0.0 };
                                let DY;
                                let GS;
                                if CE != 0.0 {
                                    let CF = AT * (CB / BY);
                                    let CG = E / CF;
                                    DY = CF;
                                    GS = CG;
                                } else {
                                    DY = A;
                                    GS = AM;
                                }
                                DX = DY;
                                GR = GS;
                            } else {
                                DX = AM;
                                GR = A;
                            }
                            CO = CA;
                            CY = BX;
                            DJ = CB;
                            DW = DX;
                            EV = BY;
                            GQ = GR;
                        }
                        CN = CO;
                        CX = CY;
                        DI = DJ;
                        DV = DW;
                        EU = EV;
                        GP = GQ;
                    }
                    CJ = CN;
                    CU = CX;
                    DE = DI;
                    DR = DV;
                    ER = EU;
                    GL = GP;
                }
                CH = CJ;
                CR = CU;
                DC = DE;
                DO = DR;
                EO = ER;
                GI = GL;
            }
            let CP = if CH < parameters[18] { 1.0 } else { 0.0 };
            if CP != 0.0 {
            } else {
            }
            let CQ = if CH > parameters[19] { 1.0 } else { 0.0 };
            if CQ != 0.0 {
            } else {
            }
            let CZ = if CR < parameters[20] { 1.0 } else { 0.0 };
            if CZ != 0.0 {
            } else {
            }
            let DA = if CR > parameters[21] { 1.0 } else { 0.0 };
            if DA != 0.0 {
            } else {
            }
            let DN = if DB != 0.0 {
                let DL = DC + DK;
                DL
            } else {
                let DM = CH + DK;
                DM
            };
            let DZ = if DO > A { 1.0 } else { 0.0 };
            let EC = if (if EA > A { 1.0 } else { 0.0 }) != 0.0 || (if EB > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let ED = if (if (if DN <= A { 1.0 } else { 0.0 }) != 0.0 && DZ != 0.0 { 1.0 } else { 0.0 }) != 0.0 && EC != 0.0 { 1.0 } else { 0.0 };
            if ED != 0.0 {
            } else {
            }
            let EG = if DC > A { 1.0 } else { 0.0 };
            let EX;
            let FB;
            if EG != 0.0 {
                let EY;
                let FC;
                if X != 0.0 {
                    let EI = EE + (EH / DC);
                    let EK = EF + (EJ / DC);
                    EY = EI;
                    FC = EK;
                } else {
                    let EL = if V != 0.0 || W != 0.0 { 1.0 } else { 0.0 };
                    let EZ;
                    let FD;
                    if EL != 0.0 {
                        let EM = EE + ((AA * EH) / DC);
                        let EN = EF + ((AA * EJ) / DC);
                        EZ = EM;
                        FD = EN;
                    } else {
                        EZ = EE;
                        FD = EF;
                    }
                    EY = EZ;
                    FC = FD;
                }
                EX = EY;
                FB = FC;
            } else {
                EX = EE;
                FB = EF;
            }
            let EW = if EO > A { 1.0 } else { 0.0 };
            let GB;
            let GC;
            if EW != 0.0 {
                let FA = EX + (parameters[41] / EO);
                let FE = FB + (parameters[42] / EO);
                GB = FA;
                GC = FE;
            } else {
                GB = EX;
                GC = FB;
            }
            let FG = if DO > (FF / multiplicity) { 1.0 } else { 0.0 };
            if FG != 0.0 {
            } else {
            }
            let FM;
            if X != 0.0 {
                let FI = FH * (CH + CR);
                FM = FI;
            } else {
                let FJ = if V != 0.0 || W != 0.0 { 1.0 } else { 0.0 };
                let FN = if FJ != 0.0 {
                    let FK = (FH * CH) + CR;
                    FK
                } else {
                    let FL = FH * CH;
                    FL
                };
                FM = FN;
            }
            let FO = (parameters[44] + (parameters[45] * FM)) + (parameters[46] * (CH * CR));
            if E != 0.0 {
            } else {
            }
            let FQ = S + (FP * node_potentials[2]);
            let FS = if FQ < (FR + E) { 1.0 } else { 0.0 };
            let FX;
            if FS != 0.0 {
                let FT = FR + (((FQ - FR) - E).exp());
                FX = FT;
            } else {
                let FV = if FQ > (FU - E) { 1.0 } else { 0.0 };
                let FY = if FV != 0.0 {
                    let FW = FU - (((FU - FQ) - E).exp());
                    FW
                } else {
                    FQ
                };
                FX = FY;
            }
            let FZ = FX + Q;
            let GA = FZ - R;
            let GD = E + (GA * (GB + (GA * GC)));
            let GE = if GD < 1.1e-1f64 { 1.0 } else { 0.0 };
            let GG = if GE != 0.0 {
                let GF = H + (1e-1f64 * (((1e1f64 * (GD - H)) - E).exp()));
                GF
            } else {
                GD
            };
            let GH = DO * GG;
            let GT = GI / GG;
            let GU = (E + (GA * parameters[43])) * parameters[30];
            let GV = if GU < A { 1.0 } else { 0.0 };
            let HJ = if GV != 0.0 {
                A
            } else {
                GU
            };
            let GW = node_potentials[0] - node_potentials[1];
            let GX = if DZ != 0.0 && EC != 0.0 { 1.0 } else { 0.0 };
            let HC = if GX != 0.0 {
                let GY = GW / DN;
                let GZ = parameters[28] * GY;
                let HA = parameters[26] * (GY.abs());
                let HB = (((E - EA) - EB) + (EA * ((E + (GZ * GZ)).sqrt()))) + (EB * ((E + ((HA * HA) * HA)).powf(3.333333333333333e-1f64)));
                HB
            } else {
                E
            };
            let HD = GW / (GH * HC);
            if EW != 0.0 {
                let HE = if ((HD / EO).abs()) > parameters[34] { 1.0 } else { 0.0 };
                if HE != 0.0 {
                } else {
                }
            } else {
            }
            if FP != 0.0 {
            } else {
            }
            if FP != 0.0 {
            } else {
            }
            let HF = if GI > A { 1.0 } else { 0.0 };
            let HG = if (if parameters[6] != 0.0 && DZ != 0.0 { 1.0 } else { 0.0 }) != 0.0 && HF != 0.0 { 1.0 } else { 0.0 };
            let HS;
            let HT;
            if HG != 0.0 {
                let HH = ((5.522602e-23f64 * FZ) * GT) / HC;
                let HI = if (if parameters[33] != 0.0 && EG != 0.0 { 1.0 } else { 0.0 }) != 0.0 && EW != 0.0 { 1.0 } else { 0.0 };
                let HP;
                if HI != 0.0 {
                    let HL = ((HJ * (((HD / EO).abs()).powf(HK))) * EO) / DC;
                    HP = HL;
                } else {
                    let HM = if (if CH > A { 1.0 } else { 0.0 }) != 0.0 && (if CR > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let HQ = if HM != 0.0 {
                        let HN = ((HJ * (((HD / CR).abs()).powf(HK))) * CR) / CH;
                        HN
                    } else {
                        A
                    };
                    HP = HQ;
                }
                let HO = if HD < A { 1.0 } else { 0.0 };
                let HU = if HO != 0.0 {
                    let HR = -HP;
                    HR
                } else {
                    HP
                };
                HS = HH;
                HT = HU;
            } else {
                HS = A;
                HT = A;
            }
            let HW = if DZ != 0.0 && HF != 0.0 { 1.0 } else { 0.0 };
            if HW != 0.0 {
                let HX = 0e0f64;
                let HY = (GW * 0e0f64) / (FO * HC);
                let HZ = E - (GW * HY);
                let IA = if HZ != A { 1.0 } else { 0.0 };
                let IC = if IA != 0.0 {
                    let IB = (HX + (HD * HY)) / HZ;
                    IB
                } else {
                    AM
                };
                let ID = if IC != A { 1.0 } else { 0.0 };
                if ID != 0.0 {
                } else {
                }
            } else {
            }
        {
            let psd = HS;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 0, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 0, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(0, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = HT;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = Some(HV);
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(1, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        Ok(())
    }
}
