#![allow(dead_code, non_snake_case, unused_parens, unused_variables)]

use super::state::Instance;
use rspice_veriloga_runtime::GeneratedEvalContext;
pub use rspice_veriloga_runtime::{GeneratedNoiseDescriptor, GeneratedNoiseEndpoint, GeneratedNoiseEvaluation, GeneratedNoiseEvaluationError, GeneratedNoiseEvaluationRef, GeneratedNoiseKind, GeneratedNoiseVisitor};

pub static NOISE_SOURCES: [GeneratedNoiseDescriptor; 1] = [
    GeneratedNoiseDescriptor { mechanism: "WHITE_A_K_SHOT", label: Some("shot"), kind: GeneratedNoiseKind::White, equation: 2, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(0), name: "A", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(1), name: "K", is_internal: false }, table_len: 0, table_log_interp: false },
];

impl Instance {
    pub fn evaluate_noise_sources(&self, ctx: &GeneratedEvalContext<'_>, visitor: &mut dyn GeneratedNoiseVisitor) -> Result<(), GeneratedNoiseEvaluationError> {
        if !self.multiplicity.is_finite() || self.multiplicity <= 0.0 {
            return Err(GeneratedNoiseEvaluationError::InvalidMultiplicity { value: self.multiplicity });
        }
        let parameters = &self.params.values;
        let temperature = ctx.temperature();
        let node_potentials = [ctx.node_voltage(self.nodes[0]), ctx.node_voltage(self.nodes[1])];
            let A = 0e0f64;
            let B = 1.0447941624768001e-10f64;
            let C = 5e-1f64;
            let E = 1e0f64;
            let G = 1.6021918e-19f64;
            let H = 8.61726105451295e-5f64;
            let K = 7.02e-4f64;
            let L = 1.108e3f64;
            let N = parameters[24];
            let P = parameters[25];
            let R = parameters[26];
            let T = parameters[21];
            let V = parameters[22];
            let X = parameters[23];
            let AC = parameters[15];
            let AE = parameters[16];
            let AG = parameters[17];
            let AL = parameters[18];
            let AN = parameters[19];
            let AP = parameters[20];
            let AR = 2.9214664e-1f64;
            let AS = 5.178164370971076e-1f64;
            let AT = 2e0f64;
            let AU = 3e0f64;
            let AV = 2.6992878119627894e-1f64;
            let AW = 4.3792457880372104e-1f64;
            let AY = parameters[53];
            let BA = parameters[54];
            let BC = parameters[55];
            let BE = parameters[50];
            let BG = parameters[51];
            let BI = parameters[52];
            let BN = parameters[56];
            let BO = parameters[57];
            let BP = parameters[58];
            let BU = 5e-2f64;
            let BZ = 9.5e-1f64;
            let DJ = 3.2e1f64;
            let DK = 9.1093826e-31f64;
            let DY = parameters[3];
            let EB = parameters[4];
            let EE = parameters[5];
            let EH = parameters[6];
            let EM = parameters[12];
            let EO = 1e8f64;
            let FA = 2.3025850929940458e2f64;
            let FE = 1e-100f64;
            let FF = 3.333333333333333e-1f64;
            let FH = 1e100f64;
            let FX = 1e-1f64;
            let GE = parameters[63];
            let GI = 2e-1f64;
            let GK = 4e0f64;
            let HR = parameters[30];
            let HS = parameters[35];
            let IN = 6.66666666666667e-1f64;
            let JB = 3.75e-1f64;
            let JZ = parameters[41];
            let KR = 1e3f64;
            let LC = parameters[10];
            let LK = parameters[31];
            let LL = parameters[36];
            let NR = parameters[42];
            let OY = parameters[32];
            let OZ = parameters[37];
            let RF = parameters[43];
            let AQR = 1.0f64;
            let ARC = -1.000000082740371e-11f64;
            let BCK = 1.0f64;
            let BCV = -5.000000413701855e-12f64;
            let BNS = 1e-3f64;
            let BOY = 1e-21f64;
            let CDR = parameters[60];
            let CDT = parameters[61];
            let CDY = 0e0f64;
            let D = if parameters[62] > C { 1.0 } else { 0.0 };
            let GC = if D != 0.0 {
                E
            } else {
                A
            };
            let F = 2.7315e2f64 + parameters[13];
            let I = H * F;
            let J = E / I;
            let M = (-((K * F) * F)) / (L + F);
            let O = N + M;
            let Q = P + M;
            let S = R + M;
            let U = E - T;
            let W = E - V;
            let Y = E - X;
            let Z = E / U;
            let AA = E / W;
            let AB = E / Y;
            let AD = B / AC;
            let AF = (parameters[33] * B) / AE;
            let AH = (parameters[34] * B) / AG;
            let AI = E / AD;
            let AJ = E / AF;
            let AK = E / AH;
            let AM = E / AL;
            let AO = E / AN;
            let AQ = E / AP;
            let AX = E - (E / parameters[14]);
            let AZ = E / (E - (AX.powf(AY)));
            let BB = E / (E - (AX.powf(BA)));
            let BD = E / (E - (AX.powf(BC)));
            let BF = E / BE;
            let BH = E / BG;
            let BJ = E / BI;
            let BK = ((-((AZ * AZ) * (AX.powf((AY - E))))) * AY) * BF;
            let BL = ((-((BB * BB) * (AX.powf((BA - E))))) * BA) * BH;
            let BM = ((-((BD * BD) * (AX.powf((BC - E))))) * BC) * BJ;
            let BQ = if (if (if (if BN != E { 1.0 } else { 0.0 }) != 0.0 || (if BO != E { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if BP != E { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if parameters[59] != E { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let BR = if BQ != 0.0 {
                E
            } else {
                A
            };
            let BS = if BR == E { 1.0 } else { 0.0 };
            let CDX;
            if BS != 0.0 {
                let BT = if (AG * BN) > 1e-18f64 { 1.0 } else { 0.0 };
                if BT != 0.0 {
                } else {
                }
                let BV = if (AP * BO) > BU { 1.0 } else { 0.0 };
                if BV != 0.0 {
                } else {
                }
                let BW = X * BP;
                let BX = if BW > BU { 1.0 } else { 0.0 };
                let BY = if BX != 0.0 {
                    BW
                } else {
                    BU
                };
                let CA = if BY < BZ { 1.0 } else { 0.0 };
                let CC;
                if CA != 0.0 {
                    let CB = if BX != 0.0 {
                        BW
                    } else {
                        BU
                    };
                    CC = CB;
                } else {
                    CC = BZ;
                }
                let CD = E - CC;
                CDX = CD;
            } else {
                CDX = CDY;
            }
            let CE = if ((temperature + parameters[2]) + parameters[9]) >= 2.3149999999999977e1f64 { ((temperature + parameters[2]) + parameters[9]) } else { 2.3149999999999977e1f64 };
            let CF = CE / F;
            let CG = H * CE;
            let CH = E / CG;
            let CI = (-((K * CE) * CE)) / (L + CE);
            let CJ = N + CI;
            let CK = P + CI;
            let CL = R + CI;
            let CM = CF * (CF.sqrt());
            let CN = CM * ((C * ((O * J) - (CJ * CH))).exp());
            let CO = CM * ((C * ((Q * J) - (CK * CH))).exp());
            let CP = CM * ((C * ((S * J) - (CL * CH))).exp());
            let CQ = (parameters[27] * CN) * CN;
            let CR = (parameters[28] * CO) * CO;
            let CS = (parameters[29] * CP) * CP;
            let CT = AT * CG;
            let CU = (AL * CF) - (CT * (CN.ln()));
            let CV = (AN * CF) - (CT * (CO.ln()));
            let CW = (AP * CF) - (CT * (CP.ln()));
            let CX = CU + (CG * ((E + (((BU - CU) * CH).exp())).ln()));
            let CY = CV + (CG * ((E + (((BU - CV) * CH).exp())).ln()));
            let CZ = CW + (CG * ((E + (((BU - CW) * CH).exp())).ln()));
            let DA = AC * ((AL * (E / CX)).powf(T));
            let DB = AE * ((AN * (E / CY)).powf(V));
            let DC = AG * ((AP * (E / CZ)).powf(X));
            let DD = if (C * CJ) >= CG { (C * CJ) } else { CG };
            let DE = if (C * CK) >= CG { (C * CK) } else { CG };
            let DF = if (C * CL) >= CG { (C * CL) } else { CG };
            let DG = DD * CH;
            let DH = DE * CH;
            let DI = DF * CH;
            let DL = (((((DJ * parameters[38]) * DK) * G) * ((DD * DD) * DD)).sqrt()) / 3.1637150399999996e-34f64;
            let DM = (((((DJ * parameters[39]) * DK) * G) * ((DE * DE) * DE)).sqrt()) / 3.1637150399999996e-34f64;
            let DN = (((((DJ * parameters[40]) * DK) * G) * ((DF * DF) * DF)).sqrt()) / 3.1637150399999996e-34f64;
            let DO = CE - F;
            let DP = parameters[44] * (E + (parameters[47] * DO));
            let DQ = parameters[45] * (E + (parameters[48] * DO));
            let DR = parameters[46] * (E + (parameters[49] * DO));
            let DS = if DP > A { 1.0 } else { 0.0 };
            let DT = if DS != 0.0 {
                DP
            } else {
                A
            };
            let DU = if DQ > A { 1.0 } else { 0.0 };
            let DV = if DU != 0.0 {
                DQ
            } else {
                A
            };
            let DW = if DR > A { 1.0 } else { 0.0 };
            let DX = if DW != 0.0 {
                DR
            } else {
                A
            };
            if BS != 0.0 {
            } else {
            }
            let DZ = if DY > A { 1.0 } else { 0.0 };
            let EA = if DZ != 0.0 {
                DY
            } else {
                A
            };
            let EC = if EB > A { 1.0 } else { 0.0 };
            let ED = if EC != 0.0 {
                EB
            } else {
                A
            };
            let EF = if EE > A { 1.0 } else { 0.0 };
            let EG = if EF != 0.0 {
                EE
            } else {
                A
            };
            let EI = if EH > A { 1.0 } else { 0.0 };
            let EJ = if EI != 0.0 {
                EH
            } else {
                A
            };
            let EK = CQ * EA;
            let EL = if EK > A { 1.0 } else { 0.0 };
            let EV = if EL != 0.0 {
                let EN = CG * (((EM / EK) + E).ln());
                EN
            } else {
                EO
            };
            let EP = CR * ED;
            let EQ = if EP > A { 1.0 } else { 0.0 };
            let EW = if EQ != 0.0 {
                let ER = CG * (((EM / EP) + E).ln());
                ER
            } else {
                EO
            };
            let ES = CS * EG;
            let ET = if ES > A { 1.0 } else { 0.0 };
            let EX = if ET != 0.0 {
                let EU = CG * (((EM / ES) + E).ln());
                EU
            } else {
                EO
            };
            let EY = if (if EV <= EW { EV } else { EW }) <= EX { (if EV <= EW { EV } else { EW }) } else { EX };
            let EZ = EY * CH;
            let FB = if (EZ.abs()) < FA { 1.0 } else { 0.0 };
            let GW;
            if FB != 0.0 {
                let FC = EZ.exp();
                GW = FC;
            } else {
                let FD = if EZ < A { 1.0 } else { 0.0 };
                let GX = if FD != 0.0 {
                    let FG = FE / (E + ((-2.3025850929940458e2f64 - EZ) * (E + (C * ((-2.3025850929940458e2f64 - EZ) * (E + ((-2.3025850929940458e2f64 - EZ) * FF)))))));
                    FG
                } else {
                    let FI = EZ - FA;
                    let FJ = FH * (E + (FI * (E + (C * (FI * (E + (FI * FF)))))));
                    FJ
                };
                GW = GX;
            }
            let FK = if EA == A { 1.0 } else { 0.0 };
            let FT;
            let FY;
            if FK != 0.0 {
                let FL = CY + CZ;
                let FM = AN + AP;
                FT = FL;
                FY = FM;
            } else {
                FT = CX;
                FY = AL;
            }
            let FN = if ED == A { 1.0 } else { 0.0 };
            let FU;
            let FZ;
            if FN != 0.0 {
                let FO = CX + CZ;
                let FP = AL + AP;
                FU = FO;
                FZ = FP;
            } else {
                FU = CY;
                FZ = AN;
            }
            let FQ = if EG == A { 1.0 } else { 0.0 };
            let FV;
            let GA;
            if FQ != 0.0 {
                let FR = CX + CY;
                let FS = AL + AN;
                FV = FR;
                GA = FS;
            } else {
                FV = CZ;
                GA = AP;
            }
            let FW = if (if FT <= FU { FT } else { FU }) <= FV { (if FT <= FU { FT } else { FU }) } else { FV };
            let GB = (if (if FY <= FZ { FY } else { FZ }) <= GA { (if FY <= FZ { FY } else { FZ }) } else { GA }) - BU;
            let GD = if GC == E { 1.0 } else { 0.0 };
            let BPU;
            let BPW;
            let BQB;
            let BQD;
            let BQI;
            let BQK;
            let BQP;
            let BQR;
            let BQX;
            let BQY;
            let BRH;
            let BRJ;
            let BRR;
            let BRV;
            let BRZ;
            if GD != 0.0 {
                let GF = -4e-1f64 * GE;
                let GG = -6.5e-1f64 * GE;
                let GH = -8e-1f64 * GE;
                let GJ = if (if (if FK != 0.0 && FN != 0.0 { 1.0 } else { 0.0 }) != 0.0 && FQ != 0.0 { 1.0 } else { 0.0 }) == 0.0 { 1.0 } else { 0.0 };
                let HP;
                let HV;
                let HX;
                let IH;
                let KC;
                let KT;
                if GJ != 0.0 {
                    let GL = if GF < EY { 1.0 } else { 0.0 };
                    let HB;
                    let HE;
                    let HG;
                    if GL != 0.0 {
                        let GM = GF * CH;
                        let GN = if ((-5e-1f64 * GM).abs()) < FA { 1.0 } else { 0.0 };
                        let GS;
                        if GN != 0.0 {
                            let GO = (-5e-1f64 * GM).exp();
                            GS = GO;
                        } else {
                            let GP = if (-5e-1f64 * GM) < A { 1.0 } else { 0.0 };
                            let GT = if GP != 0.0 {
                                let GQ = FE / (E + ((-2.3025850929940458e2f64 - (-5e-1f64 * GM)) * (E + (C * ((-2.3025850929940458e2f64 - (-5e-1f64 * GM)) * (E + ((-2.3025850929940458e2f64 - (-5e-1f64 * GM)) * FF)))))));
                                GQ
                            } else {
                                let GR = FH * (E + (((-5e-1f64 * GM) - FA) * (E + (C * (((-5e-1f64 * GM) - FA) * (E + (((-5e-1f64 * GM) - FA) * FF)))))));
                                GR
                            };
                            GS = GT;
                        }
                        let GU = E / GS;
                        let GV = GU * GU;
                        HB = GV;
                        HE = GS;
                        HG = GU;
                    } else {
                        let GY = (E + ((GF - EY) * CH)) * GW;
                        let GZ = GY.sqrt();
                        let HA = E / GZ;
                        HB = GY;
                        HE = HA;
                        HG = GZ;
                    }
                    let HC = HB - E;
                    let HD = if GF > A { 1.0 } else { 0.0 };
                    let HI = if HD != 0.0 {
                        let HF = AT * (CG * (((AT + HE) + (((HE + E) * (HE + AU)).sqrt())).ln()));
                        HF
                    } else {
                        let HH = (-GF) + (AT * (CG * ((((AT * HG) + E) + (((E + HG) * (E + (AU * HG))).sqrt())).ln())));
                        HH
                    };
                    let HJ = FW - HI;
                    let HK = GF - HJ;
                    let HL = C * ((GF + HJ) - (((HK * HK) + ((GK * CG) * CG)).sqrt()));
                    let HM = GF - GB;
                    let HN = C * ((GF + GB) - (((HM * HM) + ((GK * I) * I)).sqrt()));
                    let HO = C * (GF - (((GF * GF) + 4e-12f64).sqrt()));
                    HP = HC;
                    HV = HL;
                    HX = HI;
                    IH = HG;
                    KC = HN;
                    KT = HO;
                } else {
                    HP = A;
                    HV = A;
                    HX = A;
                    IH = A;
                    KC = A;
                    KT = A;
                }
                let MB;
                let MD;
                let MQ;
                let NP;
                let SL;
                if FK != 0.0 {
                    MB = A;
                    MD = A;
                    MQ = A;
                    NP = A;
                    SL = A;
                } else {
                    let HQ = CQ * HP;
                    let HT = if HS == A { 1.0 } else { 0.0 };
                    let HU = if (if HR == A { 1.0 } else { 0.0 }) != 0.0 && HT != 0.0 { 1.0 } else { 0.0 };
                    let IK;
                    let IL;
                    let IY;
                    let JX;
                    let LD;
                    if HU != 0.0 {
                        IK = A;
                        IL = A;
                        IY = A;
                        JX = A;
                        LD = A;
                    } else {
                        let HW = CX - HV;
                        let HY = E - ((E - (HX / HW)).sqrt());
                        let HZ = if T == C { 1.0 } else { 0.0 };
                        let IB = if HZ != 0.0 {
                            A
                        } else {
                            let IA = ((((HY * HY) * (HY.ln())) / (E - HY)) + HY) * (E - (AT * T));
                            IA
                        };
                        let IC = HY + IB;
                        let IF = if HZ != 0.0 {
                            let ID = (HW * AM).sqrt();
                            ID
                        } else {
                            let IE = (HW * AM).powf(T);
                            IE
                        };
                        let IG = AD * IF;
                        let II = CN * ((IH - E) * IG);
                        let IJ = HR * (II * IC);
                        IK = IG;
                        IL = HW;
                        IY = IC;
                        JX = II;
                        LD = IJ;
                    }
                    let LE;
                    if HT != 0.0 {
                        LE = A;
                    } else {
                        let IM = DL * ((IK * U) / IL);
                        let IO = (IN * DG) / IM;
                        let IP = IO * IO;
                        let IQ = IP * IP;
                        let IR = (IQ / (IQ + E)).sqrt();
                        let IS = IR.sqrt();
                        let IT = IR * IS;
                        let IU = (-T) * Z;
                        let IV = if IU == -1e0f64 { 1.0 } else { 0.0 };
                        let IZ = if IV != 0.0 {
                            let IW = E / (E + (IM * IT));
                            IW
                        } else {
                            let IX = (E + (IM * IT)).powf(IU);
                            IX
                        };
                        let JA = (IY * IZ) / (IY + IZ);
                        let JC = (JB * (IM / IS)).sqrt();
                        let JD = (((DG * IO) * IS) - (DG * IR)) + (C * (IM * IT));
                        let JE = (((AT * (IO * IS)) - IR) - E) * JC;
                        let JF = JE * JE;
                        let JG = if JE > A { 1.0 } else { 0.0 };
                        let JN = if JG != 0.0 {
                            let JH = E / (E + (AS * JE));
                            JH
                        } else {
                            let JI = E / (E - (AS * JE));
                            JI
                        };
                        let JJ = (-JF) + JD;
                        let JK = if JJ > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                        let JP = if JK != 0.0 {
                            let JL = JJ.exp();
                            JL
                        } else {
                            let JM = FE / (E + ((-2.3025850929940458e2f64 - JJ) * (E + (C * ((-2.3025850929940458e2f64 - JJ) * (E + ((-2.3025850929940458e2f64 - JJ) * FF)))))));
                            JM
                        };
                        let JO = JN * JN;
                        let JQ = (((AR * JN) + (AV * JO)) + (AW * (JO * JN))) * JP;
                        let JW;
                        if JG != 0.0 {
                            JW = JQ;
                        } else {
                            let JR = if JD > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            let JU = if JR != 0.0 {
                                let JS = JD.exp();
                                JS
                            } else {
                                let JT = FE / (E + ((-2.3025850929940458e2f64 - JD) * (E + (C * ((-2.3025850929940458e2f64 - JD) * (E + ((-2.3025850929940458e2f64 - JD) * FF)))))));
                                JT
                            };
                            let JV = (AT * JU) - JQ;
                            JW = JV;
                        }
                        let JY = HS * ((JX * (8.86226925452758e-1f64 * ((DG * JW) / JC))) * JA);
                        LE = JY;
                    }
                    let KA = if JZ == A { 1.0 } else { 0.0 };
                    let LF;
                    if KA != 0.0 {
                        LF = A;
                    } else {
                        let KB = if T == C { 1.0 } else { 0.0 };
                        let KF = if KB != 0.0 {
                            let KD = ((AL - KC) * AM).sqrt();
                            KD
                        } else {
                            let KE = ((AL - KC) * AM).powf(T);
                            KE
                        };
                        let KG = Z * (((AL - KC) * AI) / KF);
                        let KH = (-DT) / KG;
                        let KI = if (KH.abs()) < FA { 1.0 } else { 0.0 };
                        let KO;
                        if KI != 0.0 {
                            let KJ = KH.exp();
                            KO = KJ;
                        } else {
                            let KK = if KH < A { 1.0 } else { 0.0 };
                            let KP = if KK != 0.0 {
                                let KL = FE / (E + ((-2.3025850929940458e2f64 - KH) * (E + (C * ((-2.3025850929940458e2f64 - KH) * (E + ((-2.3025850929940458e2f64 - KH) * FF)))))));
                                KL
                            } else {
                                let KM = KH - FA;
                                let KN = FH * (E + (KM * (E + (C * (KM * (E + (KM * FF)))))));
                                KN
                            };
                            KO = KP;
                        }
                        let KQ = JZ * (((GF * KG) * KG) * KO);
                        LF = KQ;
                    }
                    let KS = if BE > KR { 1.0 } else { 0.0 };
                    let LG;
                    if KS != 0.0 {
                        LG = E;
                    } else {
                        let KU = if KT > ((-AX) * BE) { 1.0 } else { 0.0 };
                        let LH;
                        if KU != 0.0 {
                            let KV = if AY == GK { 1.0 } else { 0.0 };
                            let KZ = if KV != 0.0 {
                                let KW = KT * BF;
                                let KX = ((KW * KW) * KW) * KW;
                                KX
                            } else {
                                let KY = ((KT * BF).abs()).powf(AY);
                                KY
                            };
                            let LA = E / (E - KZ);
                            LH = LA;
                        } else {
                            let LB = AZ + ((KT + (AX * BE)) * BK);
                            LH = LB;
                        }
                        LG = LH;
                    }
                    let LI = (LC * (((HQ + LD) + LE) + LF)) * LG;
                    MB = IK;
                    MD = IL;
                    MQ = IY;
                    NP = JX;
                    SL = LI;
                }
                let PP;
                let PR;
                let QE;
                let RD;
                let SM;
                if FN != 0.0 {
                    PP = MB;
                    PR = MD;
                    QE = MQ;
                    RD = NP;
                    SM = A;
                } else {
                    let LJ = CR * HP;
                    let LM = if LL == A { 1.0 } else { 0.0 };
                    let LN = if (if LK == A { 1.0 } else { 0.0 }) != 0.0 && LM != 0.0 { 1.0 } else { 0.0 };
                    let MA;
                    let MC;
                    let MP;
                    let NO;
                    let OR;
                    if LN != 0.0 {
                        MA = MB;
                        MC = MD;
                        MP = MQ;
                        NO = NP;
                        OR = A;
                    } else {
                        let LO = CY - HV;
                        let LP = E - ((E - (HX / LO)).sqrt());
                        let LQ = if V == C { 1.0 } else { 0.0 };
                        let LS = if LQ != 0.0 {
                            A
                        } else {
                            let LR = ((((LP * LP) * (LP.ln())) / (E - LP)) + LP) * (E - (AT * V));
                            LR
                        };
                        let LT = LP + LS;
                        let LW = if LQ != 0.0 {
                            let LU = (LO * AO).sqrt();
                            LU
                        } else {
                            let LV = (LO * AO).powf(V);
                            LV
                        };
                        let LX = AF * LW;
                        let LY = CO * ((IH - E) * LX);
                        let LZ = LK * (LY * LT);
                        MA = LX;
                        MC = LO;
                        MP = LT;
                        NO = LY;
                        OR = LZ;
                    }
                    let OS;
                    if LM != 0.0 {
                        OS = A;
                    } else {
                        let ME = DM * ((MA * W) / MC);
                        let MF = (IN * DH) / ME;
                        let MG = MF * MF;
                        let MH = MG * MG;
                        let MI = (MH / (MH + E)).sqrt();
                        let MJ = MI.sqrt();
                        let MK = MI * MJ;
                        let ML = (-V) * AA;
                        let MM = if ML == -1e0f64 { 1.0 } else { 0.0 };
                        let MR = if MM != 0.0 {
                            let MN = E / (E + (ME * MK));
                            MN
                        } else {
                            let MO = (E + (ME * MK)).powf(ML);
                            MO
                        };
                        let MS = (MP * MR) / (MP + MR);
                        let MT = (JB * (ME / MJ)).sqrt();
                        let MU = (((DH * MF) * MJ) - (DH * MI)) + (C * (ME * MK));
                        let MV = (((AT * (MF * MJ)) - MI) - E) * MT;
                        let MW = MV * MV;
                        let MX = if MV > A { 1.0 } else { 0.0 };
                        let NE = if MX != 0.0 {
                            let MY = E / (E + (AS * MV));
                            MY
                        } else {
                            let MZ = E / (E - (AS * MV));
                            MZ
                        };
                        let NA = (-MW) + MU;
                        let NB = if NA > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                        let NG = if NB != 0.0 {
                            let NC = NA.exp();
                            NC
                        } else {
                            let ND = FE / (E + ((-2.3025850929940458e2f64 - NA) * (E + (C * ((-2.3025850929940458e2f64 - NA) * (E + ((-2.3025850929940458e2f64 - NA) * FF)))))));
                            ND
                        };
                        let NF = NE * NE;
                        let NH = (((AR * NE) + (AV * NF)) + (AW * (NF * NE))) * NG;
                        let NN;
                        if MX != 0.0 {
                            NN = NH;
                        } else {
                            let NI = if MU > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            let NL = if NI != 0.0 {
                                let NJ = MU.exp();
                                NJ
                            } else {
                                let NK = FE / (E + ((-2.3025850929940458e2f64 - MU) * (E + (C * ((-2.3025850929940458e2f64 - MU) * (E + ((-2.3025850929940458e2f64 - MU) * FF)))))));
                                NK
                            };
                            let NM = (AT * NL) - NH;
                            NN = NM;
                        }
                        let NQ = LL * ((NO * (8.86226925452758e-1f64 * ((DH * NN) / MT))) * MS);
                        OS = NQ;
                    }
                    let NS = if NR == A { 1.0 } else { 0.0 };
                    let OT;
                    if NS != 0.0 {
                        OT = A;
                    } else {
                        let NT = if V == C { 1.0 } else { 0.0 };
                        let NW = if NT != 0.0 {
                            let NU = ((AN - KC) * AO).sqrt();
                            NU
                        } else {
                            let NV = ((AN - KC) * AO).powf(V);
                            NV
                        };
                        let NX = AA * (((AN - KC) * AJ) / NW);
                        let NY = (-DV) / NX;
                        let NZ = if (NY.abs()) < FA { 1.0 } else { 0.0 };
                        let OF;
                        if NZ != 0.0 {
                            let OA = NY.exp();
                            OF = OA;
                        } else {
                            let OB = if NY < A { 1.0 } else { 0.0 };
                            let OG = if OB != 0.0 {
                                let OC = FE / (E + ((-2.3025850929940458e2f64 - NY) * (E + (C * ((-2.3025850929940458e2f64 - NY) * (E + ((-2.3025850929940458e2f64 - NY) * FF)))))));
                                OC
                            } else {
                                let OD = NY - FA;
                                let OE = FH * (E + (OD * (E + (C * (OD * (E + (OD * FF)))))));
                                OE
                            };
                            OF = OG;
                        }
                        let OH = NR * (((GF * NX) * NX) * OF);
                        OT = OH;
                    }
                    let OI = if BG > KR { 1.0 } else { 0.0 };
                    let OU;
                    if OI != 0.0 {
                        OU = E;
                    } else {
                        let OJ = if KT > ((-AX) * BG) { 1.0 } else { 0.0 };
                        let OV;
                        if OJ != 0.0 {
                            let OK = if BA == GK { 1.0 } else { 0.0 };
                            let OO = if OK != 0.0 {
                                let OL = KT * BH;
                                let OM = ((OL * OL) * OL) * OL;
                                OM
                            } else {
                                let ON = ((KT * BH).abs()).powf(BA);
                                ON
                            };
                            let OP = E / (E - OO);
                            OV = OP;
                        } else {
                            let OQ = BB + ((KT + (AX * BG)) * BL);
                            OV = OQ;
                        }
                        OU = OV;
                    }
                    let OW = (LC * (((LJ + OR) + OS) + OT)) * OU;
                    PP = MA;
                    PR = MC;
                    QE = MP;
                    RD = NO;
                    SM = OW;
                }
                let SN;
                let UL;
                let UN;
                let VA;
                let VZ;
                if FQ != 0.0 {
                    SN = A;
                    UL = PP;
                    UN = PR;
                    VA = QE;
                    VZ = RD;
                } else {
                    let OX = CS * HP;
                    let PA = if OZ == A { 1.0 } else { 0.0 };
                    let PB = if (if OY == A { 1.0 } else { 0.0 }) != 0.0 && PA != 0.0 { 1.0 } else { 0.0 };
                    let PO;
                    let PQ;
                    let QD;
                    let RC;
                    let SF;
                    if PB != 0.0 {
                        PO = PP;
                        PQ = PR;
                        QD = QE;
                        RC = RD;
                        SF = A;
                    } else {
                        let PC = CZ - HV;
                        let PD = E - ((E - (HX / PC)).sqrt());
                        let PE = if X == C { 1.0 } else { 0.0 };
                        let PG = if PE != 0.0 {
                            A
                        } else {
                            let PF = ((((PD * PD) * (PD.ln())) / (E - PD)) + PD) * (E - (AT * X));
                            PF
                        };
                        let PH = PD + PG;
                        let PK = if PE != 0.0 {
                            let PI = (PC * AQ).sqrt();
                            PI
                        } else {
                            let PJ = (PC * AQ).powf(X);
                            PJ
                        };
                        let PL = AH * PK;
                        let PM = CP * ((IH - E) * PL);
                        let PN = OY * (PM * PH);
                        PO = PL;
                        PQ = PC;
                        QD = PH;
                        RC = PM;
                        SF = PN;
                    }
                    let SG;
                    if PA != 0.0 {
                        SG = A;
                    } else {
                        let PS = DN * ((PO * Y) / PQ);
                        let PT = (IN * DI) / PS;
                        let PU = PT * PT;
                        let PV = PU * PU;
                        let PW = (PV / (PV + E)).sqrt();
                        let PX = PW.sqrt();
                        let PY = PW * PX;
                        let PZ = (-X) * AB;
                        let QA = if PZ == -1e0f64 { 1.0 } else { 0.0 };
                        let QF = if QA != 0.0 {
                            let QB = E / (E + (PS * PY));
                            QB
                        } else {
                            let QC = (E + (PS * PY)).powf(PZ);
                            QC
                        };
                        let QG = (QD * QF) / (QD + QF);
                        let QH = (JB * (PS / PX)).sqrt();
                        let QI = (((DI * PT) * PX) - (DI * PW)) + (C * (PS * PY));
                        let QJ = (((AT * (PT * PX)) - PW) - E) * QH;
                        let QK = QJ * QJ;
                        let QL = if QJ > A { 1.0 } else { 0.0 };
                        let QS = if QL != 0.0 {
                            let QM = E / (E + (AS * QJ));
                            QM
                        } else {
                            let QN = E / (E - (AS * QJ));
                            QN
                        };
                        let QO = (-QK) + QI;
                        let QP = if QO > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                        let QU = if QP != 0.0 {
                            let QQ = QO.exp();
                            QQ
                        } else {
                            let QR = FE / (E + ((-2.3025850929940458e2f64 - QO) * (E + (C * ((-2.3025850929940458e2f64 - QO) * (E + ((-2.3025850929940458e2f64 - QO) * FF)))))));
                            QR
                        };
                        let QT = QS * QS;
                        let QV = (((AR * QS) + (AV * QT)) + (AW * (QT * QS))) * QU;
                        let RB;
                        if QL != 0.0 {
                            RB = QV;
                        } else {
                            let QW = if QI > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            let QZ = if QW != 0.0 {
                                let QX = QI.exp();
                                QX
                            } else {
                                let QY = FE / (E + ((-2.3025850929940458e2f64 - QI) * (E + (C * ((-2.3025850929940458e2f64 - QI) * (E + ((-2.3025850929940458e2f64 - QI) * FF)))))));
                                QY
                            };
                            let RA = (AT * QZ) - QV;
                            RB = RA;
                        }
                        let RE = OZ * ((RC * (8.86226925452758e-1f64 * ((DI * RB) / QH))) * QG);
                        SG = RE;
                    }
                    let RG = if RF == A { 1.0 } else { 0.0 };
                    let SH;
                    if RG != 0.0 {
                        SH = A;
                    } else {
                        let RH = if X == C { 1.0 } else { 0.0 };
                        let RK = if RH != 0.0 {
                            let RI = ((AP - KC) * AQ).sqrt();
                            RI
                        } else {
                            let RJ = ((AP - KC) * AQ).powf(X);
                            RJ
                        };
                        let RL = AB * (((AP - KC) * AK) / RK);
                        let RM = (-DX) / RL;
                        let RN = if (RM.abs()) < FA { 1.0 } else { 0.0 };
                        let RT;
                        if RN != 0.0 {
                            let RO = RM.exp();
                            RT = RO;
                        } else {
                            let RP = if RM < A { 1.0 } else { 0.0 };
                            let RU = if RP != 0.0 {
                                let RQ = FE / (E + ((-2.3025850929940458e2f64 - RM) * (E + (C * ((-2.3025850929940458e2f64 - RM) * (E + ((-2.3025850929940458e2f64 - RM) * FF)))))));
                                RQ
                            } else {
                                let RR = RM - FA;
                                let RS = FH * (E + (RR * (E + (C * (RR * (E + (RR * FF)))))));
                                RS
                            };
                            RT = RU;
                        }
                        let RV = RF * (((GF * RL) * RL) * RT);
                        SH = RV;
                    }
                    let RW = if BI > KR { 1.0 } else { 0.0 };
                    let SI;
                    if RW != 0.0 {
                        SI = E;
                    } else {
                        let RX = if KT > ((-AX) * BI) { 1.0 } else { 0.0 };
                        let SJ;
                        if RX != 0.0 {
                            let RY = if BC == GK { 1.0 } else { 0.0 };
                            let SC = if RY != 0.0 {
                                let RZ = KT * BJ;
                                let SA = ((RZ * RZ) * RZ) * RZ;
                                SA
                            } else {
                                let SB = ((KT * BJ).abs()).powf(BC);
                                SB
                            };
                            let SD = E / (E - SC);
                            SJ = SD;
                        } else {
                            let SE = BD + ((KT + (AX * BI)) * BM);
                            SJ = SE;
                        }
                        SI = SJ;
                    }
                    let SK = (LC * (((OX + SF) + SG) + SH)) * SI;
                    SN = SK;
                    UL = PO;
                    UN = PQ;
                    VA = QD;
                    VZ = RC;
                }
                let SO = ((EA * SL) + (ED * SM)) + (EG * SN);
                let TR;
                let TV;
                let TX;
                let UH;
                let WD;
                let WT;
                if GJ != 0.0 {
                    let SP = if GG < EY { 1.0 } else { 0.0 };
                    let TD;
                    let TG;
                    let TI;
                    if SP != 0.0 {
                        let SQ = GG * CH;
                        let SR = if ((-5e-1f64 * SQ).abs()) < FA { 1.0 } else { 0.0 };
                        let SW;
                        if SR != 0.0 {
                            let SS = (-5e-1f64 * SQ).exp();
                            SW = SS;
                        } else {
                            let ST = if (-5e-1f64 * SQ) < A { 1.0 } else { 0.0 };
                            let SX = if ST != 0.0 {
                                let SU = FE / (E + ((-2.3025850929940458e2f64 - (-5e-1f64 * SQ)) * (E + (C * ((-2.3025850929940458e2f64 - (-5e-1f64 * SQ)) * (E + ((-2.3025850929940458e2f64 - (-5e-1f64 * SQ)) * FF)))))));
                                SU
                            } else {
                                let SV = FH * (E + (((-5e-1f64 * SQ) - FA) * (E + (C * (((-5e-1f64 * SQ) - FA) * (E + (((-5e-1f64 * SQ) - FA) * FF)))))));
                                SV
                            };
                            SW = SX;
                        }
                        let SY = E / SW;
                        let SZ = SY * SY;
                        TD = SZ;
                        TG = SW;
                        TI = SY;
                    } else {
                        let TA = (E + ((GG - EY) * CH)) * GW;
                        let TB = TA.sqrt();
                        let TC = E / TB;
                        TD = TA;
                        TG = TC;
                        TI = TB;
                    }
                    let TE = TD - E;
                    let TF = if GG > A { 1.0 } else { 0.0 };
                    let TK = if TF != 0.0 {
                        let TH = AT * (CG * (((AT + TG) + (((TG + E) * (TG + AU)).sqrt())).ln()));
                        TH
                    } else {
                        let TJ = (-GG) + (AT * (CG * ((((AT * TI) + E) + (((E + TI) * (E + (AU * TI))).sqrt())).ln())));
                        TJ
                    };
                    let TL = FW - TK;
                    let TM = GG - TL;
                    let TN = C * ((GG + TL) - (((TM * TM) + ((GK * CG) * CG)).sqrt()));
                    let TO = GG - GB;
                    let TP = C * ((GG + GB) - (((TO * TO) + ((GK * I) * I)).sqrt()));
                    let TQ = C * (GG - (((GG * GG) + 4e-12f64).sqrt()));
                    TR = TE;
                    TV = TN;
                    TX = TK;
                    UH = TI;
                    WD = TP;
                    WT = TQ;
                } else {
                    TR = HP;
                    TV = HV;
                    TX = A;
                    UH = IH;
                    WD = A;
                    WT = KT;
                }
                let XY;
                let YA;
                let YN;
                let ZM;
                let AEE;
                if FK != 0.0 {
                    XY = UL;
                    YA = UN;
                    YN = VA;
                    ZM = VZ;
                    AEE = A;
                } else {
                    let TS = CQ * TR;
                    let TT = if HS == A { 1.0 } else { 0.0 };
                    let TU = if (if HR == A { 1.0 } else { 0.0 }) != 0.0 && TT != 0.0 { 1.0 } else { 0.0 };
                    let UK;
                    let UM;
                    let UZ;
                    let VY;
                    let XC;
                    if TU != 0.0 {
                        UK = UL;
                        UM = UN;
                        UZ = VA;
                        VY = VZ;
                        XC = A;
                    } else {
                        let TW = CX - TV;
                        let TY = E - ((E - (TX / TW)).sqrt());
                        let TZ = if T == C { 1.0 } else { 0.0 };
                        let UB = if TZ != 0.0 {
                            A
                        } else {
                            let UA = ((((TY * TY) * (TY.ln())) / (E - TY)) + TY) * (E - (AT * T));
                            UA
                        };
                        let UC = TY + UB;
                        let UF = if TZ != 0.0 {
                            let UD = (TW * AM).sqrt();
                            UD
                        } else {
                            let UE = (TW * AM).powf(T);
                            UE
                        };
                        let UG = AD * UF;
                        let UI = CN * ((UH - E) * UG);
                        let UJ = HR * (UI * UC);
                        UK = UG;
                        UM = TW;
                        UZ = UC;
                        VY = UI;
                        XC = UJ;
                    }
                    let XD;
                    if TT != 0.0 {
                        XD = A;
                    } else {
                        let UO = DL * ((UK * U) / UM);
                        let UP = (IN * DG) / UO;
                        let UQ = UP * UP;
                        let UR = UQ * UQ;
                        let US = (UR / (UR + E)).sqrt();
                        let UT = US.sqrt();
                        let UU = US * UT;
                        let UV = (-T) * Z;
                        let UW = if UV == -1e0f64 { 1.0 } else { 0.0 };
                        let VB = if UW != 0.0 {
                            let UX = E / (E + (UO * UU));
                            UX
                        } else {
                            let UY = (E + (UO * UU)).powf(UV);
                            UY
                        };
                        let VC = (UZ * VB) / (UZ + VB);
                        let VD = (JB * (UO / UT)).sqrt();
                        let VE = (((DG * UP) * UT) - (DG * US)) + (C * (UO * UU));
                        let VF = (((AT * (UP * UT)) - US) - E) * VD;
                        let VG = VF * VF;
                        let VH = if VF > A { 1.0 } else { 0.0 };
                        let VO = if VH != 0.0 {
                            let VI = E / (E + (AS * VF));
                            VI
                        } else {
                            let VJ = E / (E - (AS * VF));
                            VJ
                        };
                        let VK = (-VG) + VE;
                        let VL = if VK > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                        let VQ = if VL != 0.0 {
                            let VM = VK.exp();
                            VM
                        } else {
                            let VN = FE / (E + ((-2.3025850929940458e2f64 - VK) * (E + (C * ((-2.3025850929940458e2f64 - VK) * (E + ((-2.3025850929940458e2f64 - VK) * FF)))))));
                            VN
                        };
                        let VP = VO * VO;
                        let VR = (((AR * VO) + (AV * VP)) + (AW * (VP * VO))) * VQ;
                        let VX;
                        if VH != 0.0 {
                            VX = VR;
                        } else {
                            let VS = if VE > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            let VV = if VS != 0.0 {
                                let VT = VE.exp();
                                VT
                            } else {
                                let VU = FE / (E + ((-2.3025850929940458e2f64 - VE) * (E + (C * ((-2.3025850929940458e2f64 - VE) * (E + ((-2.3025850929940458e2f64 - VE) * FF)))))));
                                VU
                            };
                            let VW = (AT * VV) - VR;
                            VX = VW;
                        }
                        let WA = HS * ((VY * (8.86226925452758e-1f64 * ((DG * VX) / VD))) * VC);
                        XD = WA;
                    }
                    let WB = if JZ == A { 1.0 } else { 0.0 };
                    let XE;
                    if WB != 0.0 {
                        XE = A;
                    } else {
                        let WC = if T == C { 1.0 } else { 0.0 };
                        let WG = if WC != 0.0 {
                            let WE = ((AL - WD) * AM).sqrt();
                            WE
                        } else {
                            let WF = ((AL - WD) * AM).powf(T);
                            WF
                        };
                        let WH = Z * (((AL - WD) * AI) / WG);
                        let WI = (-DT) / WH;
                        let WJ = if (WI.abs()) < FA { 1.0 } else { 0.0 };
                        let WP;
                        if WJ != 0.0 {
                            let WK = WI.exp();
                            WP = WK;
                        } else {
                            let WL = if WI < A { 1.0 } else { 0.0 };
                            let WQ = if WL != 0.0 {
                                let WM = FE / (E + ((-2.3025850929940458e2f64 - WI) * (E + (C * ((-2.3025850929940458e2f64 - WI) * (E + ((-2.3025850929940458e2f64 - WI) * FF)))))));
                                WM
                            } else {
                                let WN = WI - FA;
                                let WO = FH * (E + (WN * (E + (C * (WN * (E + (WN * FF)))))));
                                WO
                            };
                            WP = WQ;
                        }
                        let WR = JZ * (((GG * WH) * WH) * WP);
                        XE = WR;
                    }
                    let WS = if BE > KR { 1.0 } else { 0.0 };
                    let XF;
                    if WS != 0.0 {
                        XF = E;
                    } else {
                        let WU = if WT > ((-AX) * BE) { 1.0 } else { 0.0 };
                        let XG;
                        if WU != 0.0 {
                            let WV = if AY == GK { 1.0 } else { 0.0 };
                            let WZ = if WV != 0.0 {
                                let WW = WT * BF;
                                let WX = ((WW * WW) * WW) * WW;
                                WX
                            } else {
                                let WY = ((WT * BF).abs()).powf(AY);
                                WY
                            };
                            let XA = E / (E - WZ);
                            XG = XA;
                        } else {
                            let XB = AZ + ((WT + (AX * BE)) * BK);
                            XG = XB;
                        }
                        XF = XG;
                    }
                    let XH = (LC * (((TS + XC) + XD) + XE)) * XF;
                    XY = UK;
                    YA = UM;
                    YN = UZ;
                    ZM = VY;
                    AEE = XH;
                }
                let ABJ;
                let ABL;
                let ABY;
                let ACX;
                let AEF;
                if FN != 0.0 {
                    ABJ = XY;
                    ABL = YA;
                    ABY = YN;
                    ACX = ZM;
                    AEF = A;
                } else {
                    let XI = CR * TR;
                    let XJ = if LL == A { 1.0 } else { 0.0 };
                    let XK = if (if LK == A { 1.0 } else { 0.0 }) != 0.0 && XJ != 0.0 { 1.0 } else { 0.0 };
                    let XX;
                    let XZ;
                    let YM;
                    let ZL;
                    let AAN;
                    if XK != 0.0 {
                        XX = XY;
                        XZ = YA;
                        YM = YN;
                        ZL = ZM;
                        AAN = A;
                    } else {
                        let XL = CY - TV;
                        let XM = E - ((E - (TX / XL)).sqrt());
                        let XN = if V == C { 1.0 } else { 0.0 };
                        let XP = if XN != 0.0 {
                            A
                        } else {
                            let XO = ((((XM * XM) * (XM.ln())) / (E - XM)) + XM) * (E - (AT * V));
                            XO
                        };
                        let XQ = XM + XP;
                        let XT = if XN != 0.0 {
                            let XR = (XL * AO).sqrt();
                            XR
                        } else {
                            let XS = (XL * AO).powf(V);
                            XS
                        };
                        let XU = AF * XT;
                        let XV = CO * ((UH - E) * XU);
                        let XW = LK * (XV * XQ);
                        XX = XU;
                        XZ = XL;
                        YM = XQ;
                        ZL = XV;
                        AAN = XW;
                    }
                    let AAO;
                    if XJ != 0.0 {
                        AAO = A;
                    } else {
                        let YB = DM * ((XX * W) / XZ);
                        let YC = (IN * DH) / YB;
                        let YD = YC * YC;
                        let YE = YD * YD;
                        let YF = (YE / (YE + E)).sqrt();
                        let YG = YF.sqrt();
                        let YH = YF * YG;
                        let YI = (-V) * AA;
                        let YJ = if YI == -1e0f64 { 1.0 } else { 0.0 };
                        let YO = if YJ != 0.0 {
                            let YK = E / (E + (YB * YH));
                            YK
                        } else {
                            let YL = (E + (YB * YH)).powf(YI);
                            YL
                        };
                        let YP = (YM * YO) / (YM + YO);
                        let YQ = (JB * (YB / YG)).sqrt();
                        let YR = (((DH * YC) * YG) - (DH * YF)) + (C * (YB * YH));
                        let YS = (((AT * (YC * YG)) - YF) - E) * YQ;
                        let YT = YS * YS;
                        let YU = if YS > A { 1.0 } else { 0.0 };
                        let ZB = if YU != 0.0 {
                            let YV = E / (E + (AS * YS));
                            YV
                        } else {
                            let YW = E / (E - (AS * YS));
                            YW
                        };
                        let YX = (-YT) + YR;
                        let YY = if YX > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                        let ZD = if YY != 0.0 {
                            let YZ = YX.exp();
                            YZ
                        } else {
                            let ZA = FE / (E + ((-2.3025850929940458e2f64 - YX) * (E + (C * ((-2.3025850929940458e2f64 - YX) * (E + ((-2.3025850929940458e2f64 - YX) * FF)))))));
                            ZA
                        };
                        let ZC = ZB * ZB;
                        let ZE = (((AR * ZB) + (AV * ZC)) + (AW * (ZC * ZB))) * ZD;
                        let ZK;
                        if YU != 0.0 {
                            ZK = ZE;
                        } else {
                            let ZF = if YR > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            let ZI = if ZF != 0.0 {
                                let ZG = YR.exp();
                                ZG
                            } else {
                                let ZH = FE / (E + ((-2.3025850929940458e2f64 - YR) * (E + (C * ((-2.3025850929940458e2f64 - YR) * (E + ((-2.3025850929940458e2f64 - YR) * FF)))))));
                                ZH
                            };
                            let ZJ = (AT * ZI) - ZE;
                            ZK = ZJ;
                        }
                        let ZN = LL * ((ZL * (8.86226925452758e-1f64 * ((DH * ZK) / YQ))) * YP);
                        AAO = ZN;
                    }
                    let ZO = if NR == A { 1.0 } else { 0.0 };
                    let AAP;
                    if ZO != 0.0 {
                        AAP = A;
                    } else {
                        let ZP = if V == C { 1.0 } else { 0.0 };
                        let ZS = if ZP != 0.0 {
                            let ZQ = ((AN - WD) * AO).sqrt();
                            ZQ
                        } else {
                            let ZR = ((AN - WD) * AO).powf(V);
                            ZR
                        };
                        let ZT = AA * (((AN - WD) * AJ) / ZS);
                        let ZU = (-DV) / ZT;
                        let ZV = if (ZU.abs()) < FA { 1.0 } else { 0.0 };
                        let AAB;
                        if ZV != 0.0 {
                            let ZW = ZU.exp();
                            AAB = ZW;
                        } else {
                            let ZX = if ZU < A { 1.0 } else { 0.0 };
                            let AAC = if ZX != 0.0 {
                                let ZY = FE / (E + ((-2.3025850929940458e2f64 - ZU) * (E + (C * ((-2.3025850929940458e2f64 - ZU) * (E + ((-2.3025850929940458e2f64 - ZU) * FF)))))));
                                ZY
                            } else {
                                let ZZ = ZU - FA;
                                let AAA = FH * (E + (ZZ * (E + (C * (ZZ * (E + (ZZ * FF)))))));
                                AAA
                            };
                            AAB = AAC;
                        }
                        let AAD = NR * (((GG * ZT) * ZT) * AAB);
                        AAP = AAD;
                    }
                    let AAE = if BG > KR { 1.0 } else { 0.0 };
                    let AAQ;
                    if AAE != 0.0 {
                        AAQ = E;
                    } else {
                        let AAF = if WT > ((-AX) * BG) { 1.0 } else { 0.0 };
                        let AAR;
                        if AAF != 0.0 {
                            let AAG = if BA == GK { 1.0 } else { 0.0 };
                            let AAK = if AAG != 0.0 {
                                let AAH = WT * BH;
                                let AAI = ((AAH * AAH) * AAH) * AAH;
                                AAI
                            } else {
                                let AAJ = ((WT * BH).abs()).powf(BA);
                                AAJ
                            };
                            let AAL = E / (E - AAK);
                            AAR = AAL;
                        } else {
                            let AAM = BB + ((WT + (AX * BG)) * BL);
                            AAR = AAM;
                        }
                        AAQ = AAR;
                    }
                    let AAS = (LC * (((XI + AAN) + AAO) + AAP)) * AAQ;
                    ABJ = XX;
                    ABL = XZ;
                    ABY = YM;
                    ACX = ZL;
                    AEF = AAS;
                }
                let AEG;
                let AGE;
                let AGG;
                let AGT;
                let AHS;
                if FQ != 0.0 {
                    AEG = A;
                    AGE = ABJ;
                    AGG = ABL;
                    AGT = ABY;
                    AHS = ACX;
                } else {
                    let AAT = CS * TR;
                    let AAU = if OZ == A { 1.0 } else { 0.0 };
                    let AAV = if (if OY == A { 1.0 } else { 0.0 }) != 0.0 && AAU != 0.0 { 1.0 } else { 0.0 };
                    let ABI;
                    let ABK;
                    let ABX;
                    let ACW;
                    let ADY;
                    if AAV != 0.0 {
                        ABI = ABJ;
                        ABK = ABL;
                        ABX = ABY;
                        ACW = ACX;
                        ADY = A;
                    } else {
                        let AAW = CZ - TV;
                        let AAX = E - ((E - (TX / AAW)).sqrt());
                        let AAY = if X == C { 1.0 } else { 0.0 };
                        let ABA = if AAY != 0.0 {
                            A
                        } else {
                            let AAZ = ((((AAX * AAX) * (AAX.ln())) / (E - AAX)) + AAX) * (E - (AT * X));
                            AAZ
                        };
                        let ABB = AAX + ABA;
                        let ABE = if AAY != 0.0 {
                            let ABC = (AAW * AQ).sqrt();
                            ABC
                        } else {
                            let ABD = (AAW * AQ).powf(X);
                            ABD
                        };
                        let ABF = AH * ABE;
                        let ABG = CP * ((UH - E) * ABF);
                        let ABH = OY * (ABG * ABB);
                        ABI = ABF;
                        ABK = AAW;
                        ABX = ABB;
                        ACW = ABG;
                        ADY = ABH;
                    }
                    let ADZ;
                    if AAU != 0.0 {
                        ADZ = A;
                    } else {
                        let ABM = DN * ((ABI * Y) / ABK);
                        let ABN = (IN * DI) / ABM;
                        let ABO = ABN * ABN;
                        let ABP = ABO * ABO;
                        let ABQ = (ABP / (ABP + E)).sqrt();
                        let ABR = ABQ.sqrt();
                        let ABS = ABQ * ABR;
                        let ABT = (-X) * AB;
                        let ABU = if ABT == -1e0f64 { 1.0 } else { 0.0 };
                        let ABZ = if ABU != 0.0 {
                            let ABV = E / (E + (ABM * ABS));
                            ABV
                        } else {
                            let ABW = (E + (ABM * ABS)).powf(ABT);
                            ABW
                        };
                        let ACA = (ABX * ABZ) / (ABX + ABZ);
                        let ACB = (JB * (ABM / ABR)).sqrt();
                        let ACC = (((DI * ABN) * ABR) - (DI * ABQ)) + (C * (ABM * ABS));
                        let ACD = (((AT * (ABN * ABR)) - ABQ) - E) * ACB;
                        let ACE = ACD * ACD;
                        let ACF = if ACD > A { 1.0 } else { 0.0 };
                        let ACM = if ACF != 0.0 {
                            let ACG = E / (E + (AS * ACD));
                            ACG
                        } else {
                            let ACH = E / (E - (AS * ACD));
                            ACH
                        };
                        let ACI = (-ACE) + ACC;
                        let ACJ = if ACI > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                        let ACO = if ACJ != 0.0 {
                            let ACK = ACI.exp();
                            ACK
                        } else {
                            let ACL = FE / (E + ((-2.3025850929940458e2f64 - ACI) * (E + (C * ((-2.3025850929940458e2f64 - ACI) * (E + ((-2.3025850929940458e2f64 - ACI) * FF)))))));
                            ACL
                        };
                        let ACN = ACM * ACM;
                        let ACP = (((AR * ACM) + (AV * ACN)) + (AW * (ACN * ACM))) * ACO;
                        let ACV;
                        if ACF != 0.0 {
                            ACV = ACP;
                        } else {
                            let ACQ = if ACC > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            let ACT = if ACQ != 0.0 {
                                let ACR = ACC.exp();
                                ACR
                            } else {
                                let ACS = FE / (E + ((-2.3025850929940458e2f64 - ACC) * (E + (C * ((-2.3025850929940458e2f64 - ACC) * (E + ((-2.3025850929940458e2f64 - ACC) * FF)))))));
                                ACS
                            };
                            let ACU = (AT * ACT) - ACP;
                            ACV = ACU;
                        }
                        let ACY = OZ * ((ACW * (8.86226925452758e-1f64 * ((DI * ACV) / ACB))) * ACA);
                        ADZ = ACY;
                    }
                    let ACZ = if RF == A { 1.0 } else { 0.0 };
                    let AEA;
                    if ACZ != 0.0 {
                        AEA = A;
                    } else {
                        let ADA = if X == C { 1.0 } else { 0.0 };
                        let ADD = if ADA != 0.0 {
                            let ADB = ((AP - WD) * AQ).sqrt();
                            ADB
                        } else {
                            let ADC = ((AP - WD) * AQ).powf(X);
                            ADC
                        };
                        let ADE = AB * (((AP - WD) * AK) / ADD);
                        let ADF = (-DX) / ADE;
                        let ADG = if (ADF.abs()) < FA { 1.0 } else { 0.0 };
                        let ADM;
                        if ADG != 0.0 {
                            let ADH = ADF.exp();
                            ADM = ADH;
                        } else {
                            let ADI = if ADF < A { 1.0 } else { 0.0 };
                            let ADN = if ADI != 0.0 {
                                let ADJ = FE / (E + ((-2.3025850929940458e2f64 - ADF) * (E + (C * ((-2.3025850929940458e2f64 - ADF) * (E + ((-2.3025850929940458e2f64 - ADF) * FF)))))));
                                ADJ
                            } else {
                                let ADK = ADF - FA;
                                let ADL = FH * (E + (ADK * (E + (C * (ADK * (E + (ADK * FF)))))));
                                ADL
                            };
                            ADM = ADN;
                        }
                        let ADO = RF * (((GG * ADE) * ADE) * ADM);
                        AEA = ADO;
                    }
                    let ADP = if BI > KR { 1.0 } else { 0.0 };
                    let AEB;
                    if ADP != 0.0 {
                        AEB = E;
                    } else {
                        let ADQ = if WT > ((-AX) * BI) { 1.0 } else { 0.0 };
                        let AEC;
                        if ADQ != 0.0 {
                            let ADR = if BC == GK { 1.0 } else { 0.0 };
                            let ADV = if ADR != 0.0 {
                                let ADS = WT * BJ;
                                let ADT = ((ADS * ADS) * ADS) * ADS;
                                ADT
                            } else {
                                let ADU = ((WT * BJ).abs()).powf(BC);
                                ADU
                            };
                            let ADW = E / (E - ADV);
                            AEC = ADW;
                        } else {
                            let ADX = BD + ((WT + (AX * BI)) * BM);
                            AEC = ADX;
                        }
                        AEB = AEC;
                    }
                    let AED = (LC * (((AAT + ADY) + ADZ) + AEA)) * AEB;
                    AEG = AED;
                    AGE = ABI;
                    AGG = ABK;
                    AGT = ABX;
                    AHS = ACW;
                }
                let AEH = ((EA * AEE) + (ED * AEF)) + (EG * AEG);
                let AFK;
                let AFO;
                let AFQ;
                let AGA;
                let AHW;
                let AIM;
                if GJ != 0.0 {
                    let AEI = if GH < EY { 1.0 } else { 0.0 };
                    let AEW;
                    let AEZ;
                    let AFB;
                    if AEI != 0.0 {
                        let AEJ = GH * CH;
                        let AEK = if ((-5e-1f64 * AEJ).abs()) < FA { 1.0 } else { 0.0 };
                        let AEP;
                        if AEK != 0.0 {
                            let AEL = (-5e-1f64 * AEJ).exp();
                            AEP = AEL;
                        } else {
                            let AEM = if (-5e-1f64 * AEJ) < A { 1.0 } else { 0.0 };
                            let AEQ = if AEM != 0.0 {
                                let AEN = FE / (E + ((-2.3025850929940458e2f64 - (-5e-1f64 * AEJ)) * (E + (C * ((-2.3025850929940458e2f64 - (-5e-1f64 * AEJ)) * (E + ((-2.3025850929940458e2f64 - (-5e-1f64 * AEJ)) * FF)))))));
                                AEN
                            } else {
                                let AEO = FH * (E + (((-5e-1f64 * AEJ) - FA) * (E + (C * (((-5e-1f64 * AEJ) - FA) * (E + (((-5e-1f64 * AEJ) - FA) * FF)))))));
                                AEO
                            };
                            AEP = AEQ;
                        }
                        let AER = E / AEP;
                        let AES = AER * AER;
                        AEW = AES;
                        AEZ = AEP;
                        AFB = AER;
                    } else {
                        let AET = (E + ((GH - EY) * CH)) * GW;
                        let AEU = AET.sqrt();
                        let AEV = E / AEU;
                        AEW = AET;
                        AEZ = AEV;
                        AFB = AEU;
                    }
                    let AEX = AEW - E;
                    let AEY = if GH > A { 1.0 } else { 0.0 };
                    let AFD = if AEY != 0.0 {
                        let AFA = AT * (CG * (((AT + AEZ) + (((AEZ + E) * (AEZ + AU)).sqrt())).ln()));
                        AFA
                    } else {
                        let AFC = (-GH) + (AT * (CG * ((((AT * AFB) + E) + (((E + AFB) * (E + (AU * AFB))).sqrt())).ln())));
                        AFC
                    };
                    let AFE = FW - AFD;
                    let AFF = GH - AFE;
                    let AFG = C * ((GH + AFE) - (((AFF * AFF) + ((GK * CG) * CG)).sqrt()));
                    let AFH = GH - GB;
                    let AFI = C * ((GH + GB) - (((AFH * AFH) + ((GK * I) * I)).sqrt()));
                    let AFJ = C * (GH - (((GH * GH) + 4e-12f64).sqrt()));
                    AFK = AEX;
                    AFO = AFG;
                    AFQ = AFD;
                    AGA = AFB;
                    AHW = AFI;
                    AIM = AFJ;
                } else {
                    AFK = TR;
                    AFO = TV;
                    AFQ = A;
                    AGA = UH;
                    AHW = A;
                    AIM = WT;
                }
                let AJR;
                let AJT;
                let AKG;
                let ALF;
                let APX;
                if FK != 0.0 {
                    AJR = AGE;
                    AJT = AGG;
                    AKG = AGT;
                    ALF = AHS;
                    APX = A;
                } else {
                    let AFL = CQ * AFK;
                    let AFM = if HS == A { 1.0 } else { 0.0 };
                    let AFN = if (if HR == A { 1.0 } else { 0.0 }) != 0.0 && AFM != 0.0 { 1.0 } else { 0.0 };
                    let AGD;
                    let AGF;
                    let AGS;
                    let AHR;
                    let AIV;
                    if AFN != 0.0 {
                        AGD = AGE;
                        AGF = AGG;
                        AGS = AGT;
                        AHR = AHS;
                        AIV = A;
                    } else {
                        let AFP = CX - AFO;
                        let AFR = E - ((E - (AFQ / AFP)).sqrt());
                        let AFS = if T == C { 1.0 } else { 0.0 };
                        let AFU = if AFS != 0.0 {
                            A
                        } else {
                            let AFT = ((((AFR * AFR) * (AFR.ln())) / (E - AFR)) + AFR) * (E - (AT * T));
                            AFT
                        };
                        let AFV = AFR + AFU;
                        let AFY = if AFS != 0.0 {
                            let AFW = (AFP * AM).sqrt();
                            AFW
                        } else {
                            let AFX = (AFP * AM).powf(T);
                            AFX
                        };
                        let AFZ = AD * AFY;
                        let AGB = CN * ((AGA - E) * AFZ);
                        let AGC = HR * (AGB * AFV);
                        AGD = AFZ;
                        AGF = AFP;
                        AGS = AFV;
                        AHR = AGB;
                        AIV = AGC;
                    }
                    let AIW;
                    if AFM != 0.0 {
                        AIW = A;
                    } else {
                        let AGH = DL * ((AGD * U) / AGF);
                        let AGI = (IN * DG) / AGH;
                        let AGJ = AGI * AGI;
                        let AGK = AGJ * AGJ;
                        let AGL = (AGK / (AGK + E)).sqrt();
                        let AGM = AGL.sqrt();
                        let AGN = AGL * AGM;
                        let AGO = (-T) * Z;
                        let AGP = if AGO == -1e0f64 { 1.0 } else { 0.0 };
                        let AGU = if AGP != 0.0 {
                            let AGQ = E / (E + (AGH * AGN));
                            AGQ
                        } else {
                            let AGR = (E + (AGH * AGN)).powf(AGO);
                            AGR
                        };
                        let AGV = (AGS * AGU) / (AGS + AGU);
                        let AGW = (JB * (AGH / AGM)).sqrt();
                        let AGX = (((DG * AGI) * AGM) - (DG * AGL)) + (C * (AGH * AGN));
                        let AGY = (((AT * (AGI * AGM)) - AGL) - E) * AGW;
                        let AGZ = AGY * AGY;
                        let AHA = if AGY > A { 1.0 } else { 0.0 };
                        let AHH = if AHA != 0.0 {
                            let AHB = E / (E + (AS * AGY));
                            AHB
                        } else {
                            let AHC = E / (E - (AS * AGY));
                            AHC
                        };
                        let AHD = (-AGZ) + AGX;
                        let AHE = if AHD > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                        let AHJ = if AHE != 0.0 {
                            let AHF = AHD.exp();
                            AHF
                        } else {
                            let AHG = FE / (E + ((-2.3025850929940458e2f64 - AHD) * (E + (C * ((-2.3025850929940458e2f64 - AHD) * (E + ((-2.3025850929940458e2f64 - AHD) * FF)))))));
                            AHG
                        };
                        let AHI = AHH * AHH;
                        let AHK = (((AR * AHH) + (AV * AHI)) + (AW * (AHI * AHH))) * AHJ;
                        let AHQ;
                        if AHA != 0.0 {
                            AHQ = AHK;
                        } else {
                            let AHL = if AGX > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            let AHO = if AHL != 0.0 {
                                let AHM = AGX.exp();
                                AHM
                            } else {
                                let AHN = FE / (E + ((-2.3025850929940458e2f64 - AGX) * (E + (C * ((-2.3025850929940458e2f64 - AGX) * (E + ((-2.3025850929940458e2f64 - AGX) * FF)))))));
                                AHN
                            };
                            let AHP = (AT * AHO) - AHK;
                            AHQ = AHP;
                        }
                        let AHT = HS * ((AHR * (8.86226925452758e-1f64 * ((DG * AHQ) / AGW))) * AGV);
                        AIW = AHT;
                    }
                    let AHU = if JZ == A { 1.0 } else { 0.0 };
                    let AIX;
                    if AHU != 0.0 {
                        AIX = A;
                    } else {
                        let AHV = if T == C { 1.0 } else { 0.0 };
                        let AHZ = if AHV != 0.0 {
                            let AHX = ((AL - AHW) * AM).sqrt();
                            AHX
                        } else {
                            let AHY = ((AL - AHW) * AM).powf(T);
                            AHY
                        };
                        let AIA = Z * (((AL - AHW) * AI) / AHZ);
                        let AIB = (-DT) / AIA;
                        let AIC = if (AIB.abs()) < FA { 1.0 } else { 0.0 };
                        let AII;
                        if AIC != 0.0 {
                            let AID = AIB.exp();
                            AII = AID;
                        } else {
                            let AIE = if AIB < A { 1.0 } else { 0.0 };
                            let AIJ = if AIE != 0.0 {
                                let AIF = FE / (E + ((-2.3025850929940458e2f64 - AIB) * (E + (C * ((-2.3025850929940458e2f64 - AIB) * (E + ((-2.3025850929940458e2f64 - AIB) * FF)))))));
                                AIF
                            } else {
                                let AIG = AIB - FA;
                                let AIH = FH * (E + (AIG * (E + (C * (AIG * (E + (AIG * FF)))))));
                                AIH
                            };
                            AII = AIJ;
                        }
                        let AIK = JZ * (((GH * AIA) * AIA) * AII);
                        AIX = AIK;
                    }
                    let AIL = if BE > KR { 1.0 } else { 0.0 };
                    let AIY;
                    if AIL != 0.0 {
                        AIY = E;
                    } else {
                        let AIN = if AIM > ((-AX) * BE) { 1.0 } else { 0.0 };
                        let AIZ;
                        if AIN != 0.0 {
                            let AIO = if AY == GK { 1.0 } else { 0.0 };
                            let AIS = if AIO != 0.0 {
                                let AIP = AIM * BF;
                                let AIQ = ((AIP * AIP) * AIP) * AIP;
                                AIQ
                            } else {
                                let AIR = ((AIM * BF).abs()).powf(AY);
                                AIR
                            };
                            let AIT = E / (E - AIS);
                            AIZ = AIT;
                        } else {
                            let AIU = AZ + ((AIM + (AX * BE)) * BK);
                            AIZ = AIU;
                        }
                        AIY = AIZ;
                    }
                    let AJA = (LC * (((AFL + AIV) + AIW) + AIX)) * AIY;
                    AJR = AGD;
                    AJT = AGF;
                    AKG = AGS;
                    ALF = AHR;
                    APX = AJA;
                }
                let ANC;
                let ANE;
                let ANR;
                let AOQ;
                let APY;
                if FN != 0.0 {
                    ANC = AJR;
                    ANE = AJT;
                    ANR = AKG;
                    AOQ = ALF;
                    APY = A;
                } else {
                    let AJB = CR * AFK;
                    let AJC = if LL == A { 1.0 } else { 0.0 };
                    let AJD = if (if LK == A { 1.0 } else { 0.0 }) != 0.0 && AJC != 0.0 { 1.0 } else { 0.0 };
                    let AJQ;
                    let AJS;
                    let AKF;
                    let ALE;
                    let AMG;
                    if AJD != 0.0 {
                        AJQ = AJR;
                        AJS = AJT;
                        AKF = AKG;
                        ALE = ALF;
                        AMG = A;
                    } else {
                        let AJE = CY - AFO;
                        let AJF = E - ((E - (AFQ / AJE)).sqrt());
                        let AJG = if V == C { 1.0 } else { 0.0 };
                        let AJI = if AJG != 0.0 {
                            A
                        } else {
                            let AJH = ((((AJF * AJF) * (AJF.ln())) / (E - AJF)) + AJF) * (E - (AT * V));
                            AJH
                        };
                        let AJJ = AJF + AJI;
                        let AJM = if AJG != 0.0 {
                            let AJK = (AJE * AO).sqrt();
                            AJK
                        } else {
                            let AJL = (AJE * AO).powf(V);
                            AJL
                        };
                        let AJN = AF * AJM;
                        let AJO = CO * ((AGA - E) * AJN);
                        let AJP = LK * (AJO * AJJ);
                        AJQ = AJN;
                        AJS = AJE;
                        AKF = AJJ;
                        ALE = AJO;
                        AMG = AJP;
                    }
                    let AMH;
                    if AJC != 0.0 {
                        AMH = A;
                    } else {
                        let AJU = DM * ((AJQ * W) / AJS);
                        let AJV = (IN * DH) / AJU;
                        let AJW = AJV * AJV;
                        let AJX = AJW * AJW;
                        let AJY = (AJX / (AJX + E)).sqrt();
                        let AJZ = AJY.sqrt();
                        let AKA = AJY * AJZ;
                        let AKB = (-V) * AA;
                        let AKC = if AKB == -1e0f64 { 1.0 } else { 0.0 };
                        let AKH = if AKC != 0.0 {
                            let AKD = E / (E + (AJU * AKA));
                            AKD
                        } else {
                            let AKE = (E + (AJU * AKA)).powf(AKB);
                            AKE
                        };
                        let AKI = (AKF * AKH) / (AKF + AKH);
                        let AKJ = (JB * (AJU / AJZ)).sqrt();
                        let AKK = (((DH * AJV) * AJZ) - (DH * AJY)) + (C * (AJU * AKA));
                        let AKL = (((AT * (AJV * AJZ)) - AJY) - E) * AKJ;
                        let AKM = AKL * AKL;
                        let AKN = if AKL > A { 1.0 } else { 0.0 };
                        let AKU = if AKN != 0.0 {
                            let AKO = E / (E + (AS * AKL));
                            AKO
                        } else {
                            let AKP = E / (E - (AS * AKL));
                            AKP
                        };
                        let AKQ = (-AKM) + AKK;
                        let AKR = if AKQ > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                        let AKW = if AKR != 0.0 {
                            let AKS = AKQ.exp();
                            AKS
                        } else {
                            let AKT = FE / (E + ((-2.3025850929940458e2f64 - AKQ) * (E + (C * ((-2.3025850929940458e2f64 - AKQ) * (E + ((-2.3025850929940458e2f64 - AKQ) * FF)))))));
                            AKT
                        };
                        let AKV = AKU * AKU;
                        let AKX = (((AR * AKU) + (AV * AKV)) + (AW * (AKV * AKU))) * AKW;
                        let ALD;
                        if AKN != 0.0 {
                            ALD = AKX;
                        } else {
                            let AKY = if AKK > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            let ALB = if AKY != 0.0 {
                                let AKZ = AKK.exp();
                                AKZ
                            } else {
                                let ALA = FE / (E + ((-2.3025850929940458e2f64 - AKK) * (E + (C * ((-2.3025850929940458e2f64 - AKK) * (E + ((-2.3025850929940458e2f64 - AKK) * FF)))))));
                                ALA
                            };
                            let ALC = (AT * ALB) - AKX;
                            ALD = ALC;
                        }
                        let ALG = LL * ((ALE * (8.86226925452758e-1f64 * ((DH * ALD) / AKJ))) * AKI);
                        AMH = ALG;
                    }
                    let ALH = if NR == A { 1.0 } else { 0.0 };
                    let AMI;
                    if ALH != 0.0 {
                        AMI = A;
                    } else {
                        let ALI = if V == C { 1.0 } else { 0.0 };
                        let ALL = if ALI != 0.0 {
                            let ALJ = ((AN - AHW) * AO).sqrt();
                            ALJ
                        } else {
                            let ALK = ((AN - AHW) * AO).powf(V);
                            ALK
                        };
                        let ALM = AA * (((AN - AHW) * AJ) / ALL);
                        let ALN = (-DV) / ALM;
                        let ALO = if (ALN.abs()) < FA { 1.0 } else { 0.0 };
                        let ALU;
                        if ALO != 0.0 {
                            let ALP = ALN.exp();
                            ALU = ALP;
                        } else {
                            let ALQ = if ALN < A { 1.0 } else { 0.0 };
                            let ALV = if ALQ != 0.0 {
                                let ALR = FE / (E + ((-2.3025850929940458e2f64 - ALN) * (E + (C * ((-2.3025850929940458e2f64 - ALN) * (E + ((-2.3025850929940458e2f64 - ALN) * FF)))))));
                                ALR
                            } else {
                                let ALS = ALN - FA;
                                let ALT = FH * (E + (ALS * (E + (C * (ALS * (E + (ALS * FF)))))));
                                ALT
                            };
                            ALU = ALV;
                        }
                        let ALW = NR * (((GH * ALM) * ALM) * ALU);
                        AMI = ALW;
                    }
                    let ALX = if BG > KR { 1.0 } else { 0.0 };
                    let AMJ;
                    if ALX != 0.0 {
                        AMJ = E;
                    } else {
                        let ALY = if AIM > ((-AX) * BG) { 1.0 } else { 0.0 };
                        let AMK;
                        if ALY != 0.0 {
                            let ALZ = if BA == GK { 1.0 } else { 0.0 };
                            let AMD = if ALZ != 0.0 {
                                let AMA = AIM * BH;
                                let AMB = ((AMA * AMA) * AMA) * AMA;
                                AMB
                            } else {
                                let AMC = ((AIM * BH).abs()).powf(BA);
                                AMC
                            };
                            let AME = E / (E - AMD);
                            AMK = AME;
                        } else {
                            let AMF = BB + ((AIM + (AX * BG)) * BL);
                            AMK = AMF;
                        }
                        AMJ = AMK;
                    }
                    let AML = (LC * (((AJB + AMG) + AMH) + AMI)) * AMJ;
                    ANC = AJQ;
                    ANE = AJS;
                    ANR = AKF;
                    AOQ = ALE;
                    APY = AML;
                }
                let APZ;
                let ARX;
                let ARZ;
                let ASM;
                let ATL;
                if FQ != 0.0 {
                    APZ = A;
                    ARX = ANC;
                    ARZ = ANE;
                    ASM = ANR;
                    ATL = AOQ;
                } else {
                    let AMM = CS * AFK;
                    let AMN = if OZ == A { 1.0 } else { 0.0 };
                    let AMO = if (if OY == A { 1.0 } else { 0.0 }) != 0.0 && AMN != 0.0 { 1.0 } else { 0.0 };
                    let ANB;
                    let AND;
                    let ANQ;
                    let AOP;
                    let APR;
                    if AMO != 0.0 {
                        ANB = ANC;
                        AND = ANE;
                        ANQ = ANR;
                        AOP = AOQ;
                        APR = A;
                    } else {
                        let AMP = CZ - AFO;
                        let AMQ = E - ((E - (AFQ / AMP)).sqrt());
                        let AMR = if X == C { 1.0 } else { 0.0 };
                        let AMT = if AMR != 0.0 {
                            A
                        } else {
                            let AMS = ((((AMQ * AMQ) * (AMQ.ln())) / (E - AMQ)) + AMQ) * (E - (AT * X));
                            AMS
                        };
                        let AMU = AMQ + AMT;
                        let AMX = if AMR != 0.0 {
                            let AMV = (AMP * AQ).sqrt();
                            AMV
                        } else {
                            let AMW = (AMP * AQ).powf(X);
                            AMW
                        };
                        let AMY = AH * AMX;
                        let AMZ = CP * ((AGA - E) * AMY);
                        let ANA = OY * (AMZ * AMU);
                        ANB = AMY;
                        AND = AMP;
                        ANQ = AMU;
                        AOP = AMZ;
                        APR = ANA;
                    }
                    let APS;
                    if AMN != 0.0 {
                        APS = A;
                    } else {
                        let ANF = DN * ((ANB * Y) / AND);
                        let ANG = (IN * DI) / ANF;
                        let ANH = ANG * ANG;
                        let ANI = ANH * ANH;
                        let ANJ = (ANI / (ANI + E)).sqrt();
                        let ANK = ANJ.sqrt();
                        let ANL = ANJ * ANK;
                        let ANM = (-X) * AB;
                        let ANN = if ANM == -1e0f64 { 1.0 } else { 0.0 };
                        let ANS = if ANN != 0.0 {
                            let ANO = E / (E + (ANF * ANL));
                            ANO
                        } else {
                            let ANP = (E + (ANF * ANL)).powf(ANM);
                            ANP
                        };
                        let ANT = (ANQ * ANS) / (ANQ + ANS);
                        let ANU = (JB * (ANF / ANK)).sqrt();
                        let ANV = (((DI * ANG) * ANK) - (DI * ANJ)) + (C * (ANF * ANL));
                        let ANW = (((AT * (ANG * ANK)) - ANJ) - E) * ANU;
                        let ANX = ANW * ANW;
                        let ANY = if ANW > A { 1.0 } else { 0.0 };
                        let AOF = if ANY != 0.0 {
                            let ANZ = E / (E + (AS * ANW));
                            ANZ
                        } else {
                            let AOA = E / (E - (AS * ANW));
                            AOA
                        };
                        let AOB = (-ANX) + ANV;
                        let AOC = if AOB > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                        let AOH = if AOC != 0.0 {
                            let AOD = AOB.exp();
                            AOD
                        } else {
                            let AOE = FE / (E + ((-2.3025850929940458e2f64 - AOB) * (E + (C * ((-2.3025850929940458e2f64 - AOB) * (E + ((-2.3025850929940458e2f64 - AOB) * FF)))))));
                            AOE
                        };
                        let AOG = AOF * AOF;
                        let AOI = (((AR * AOF) + (AV * AOG)) + (AW * (AOG * AOF))) * AOH;
                        let AOO;
                        if ANY != 0.0 {
                            AOO = AOI;
                        } else {
                            let AOJ = if ANV > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            let AOM = if AOJ != 0.0 {
                                let AOK = ANV.exp();
                                AOK
                            } else {
                                let AOL = FE / (E + ((-2.3025850929940458e2f64 - ANV) * (E + (C * ((-2.3025850929940458e2f64 - ANV) * (E + ((-2.3025850929940458e2f64 - ANV) * FF)))))));
                                AOL
                            };
                            let AON = (AT * AOM) - AOI;
                            AOO = AON;
                        }
                        let AOR = OZ * ((AOP * (8.86226925452758e-1f64 * ((DI * AOO) / ANU))) * ANT);
                        APS = AOR;
                    }
                    let AOS = if RF == A { 1.0 } else { 0.0 };
                    let APT;
                    if AOS != 0.0 {
                        APT = A;
                    } else {
                        let AOT = if X == C { 1.0 } else { 0.0 };
                        let AOW = if AOT != 0.0 {
                            let AOU = ((AP - AHW) * AQ).sqrt();
                            AOU
                        } else {
                            let AOV = ((AP - AHW) * AQ).powf(X);
                            AOV
                        };
                        let AOX = AB * (((AP - AHW) * AK) / AOW);
                        let AOY = (-DX) / AOX;
                        let AOZ = if (AOY.abs()) < FA { 1.0 } else { 0.0 };
                        let APF;
                        if AOZ != 0.0 {
                            let APA = AOY.exp();
                            APF = APA;
                        } else {
                            let APB = if AOY < A { 1.0 } else { 0.0 };
                            let APG = if APB != 0.0 {
                                let APC = FE / (E + ((-2.3025850929940458e2f64 - AOY) * (E + (C * ((-2.3025850929940458e2f64 - AOY) * (E + ((-2.3025850929940458e2f64 - AOY) * FF)))))));
                                APC
                            } else {
                                let APD = AOY - FA;
                                let APE = FH * (E + (APD * (E + (C * (APD * (E + (APD * FF)))))));
                                APE
                            };
                            APF = APG;
                        }
                        let APH = RF * (((GH * AOX) * AOX) * APF);
                        APT = APH;
                    }
                    let API = if BI > KR { 1.0 } else { 0.0 };
                    let APU;
                    if API != 0.0 {
                        APU = E;
                    } else {
                        let APJ = if AIM > ((-AX) * BI) { 1.0 } else { 0.0 };
                        let APV;
                        if APJ != 0.0 {
                            let APK = if BC == GK { 1.0 } else { 0.0 };
                            let APO = if APK != 0.0 {
                                let APL = AIM * BJ;
                                let APM = ((APL * APL) * APL) * APL;
                                APM
                            } else {
                                let APN = ((AIM * BJ).abs()).powf(BC);
                                APN
                            };
                            let APP = E / (E - APO);
                            APV = APP;
                        } else {
                            let APQ = BD + ((AIM + (AX * BI)) * BM);
                            APV = APQ;
                        }
                        APU = APV;
                    }
                    let APW = (LC * (((AMM + APR) + APS) + APT)) * APU;
                    APZ = APW;
                    ARX = ANB;
                    ARZ = AND;
                    ASM = ANQ;
                    ATL = AOP;
                }
                let AQA = ((EA * APX) + (ED * APY)) + (EG * APZ);
                let ARD;
                let ARH;
                let ARJ;
                let ART;
                let ATP;
                let AUF;
                if GJ != 0.0 {
                    let AQB = if FX < EY { 1.0 } else { 0.0 };
                    let AQP;
                    let AQS;
                    let AQU;
                    if AQB != 0.0 {
                        let AQC = FX * CH;
                        let AQD = if ((-5e-1f64 * AQC).abs()) < FA { 1.0 } else { 0.0 };
                        let AQI;
                        if AQD != 0.0 {
                            let AQE = (-5e-1f64 * AQC).exp();
                            AQI = AQE;
                        } else {
                            let AQF = if (-5e-1f64 * AQC) < A { 1.0 } else { 0.0 };
                            let AQJ = if AQF != 0.0 {
                                let AQG = FE / (E + ((-2.3025850929940458e2f64 - (-5e-1f64 * AQC)) * (E + (C * ((-2.3025850929940458e2f64 - (-5e-1f64 * AQC)) * (E + ((-2.3025850929940458e2f64 - (-5e-1f64 * AQC)) * FF)))))));
                                AQG
                            } else {
                                let AQH = FH * (E + (((-5e-1f64 * AQC) - FA) * (E + (C * (((-5e-1f64 * AQC) - FA) * (E + (((-5e-1f64 * AQC) - FA) * FF)))))));
                                AQH
                            };
                            AQI = AQJ;
                        }
                        let AQK = E / AQI;
                        let AQL = AQK * AQK;
                        AQP = AQL;
                        AQS = AQI;
                        AQU = AQK;
                    } else {
                        let AQM = (E + ((FX - EY) * CH)) * GW;
                        let AQN = AQM.sqrt();
                        let AQO = E / AQN;
                        AQP = AQM;
                        AQS = AQO;
                        AQU = AQN;
                    }
                    let AQQ = AQP - E;
                    let AQW = if AQR != 0.0 {
                        let AQT = AT * (CG * (((AT + AQS) + (((AQS + E) * (AQS + AU)).sqrt())).ln()));
                        AQT
                    } else {
                        let AQV = -1e-1f64 + (AT * (CG * ((((AT * AQU) + E) + (((E + AQU) * (E + (AU * AQU))).sqrt())).ln())));
                        AQV
                    };
                    let AQX = FW - AQW;
                    let AQY = FX - AQX;
                    let AQZ = C * ((FX + AQX) - (((AQY * AQY) + ((GK * CG) * CG)).sqrt()));
                    let ARA = FX - GB;
                    let ARB = C * ((FX + GB) - (((ARA * ARA) + ((GK * I) * I)).sqrt()));
                    ARD = AQQ;
                    ARH = AQZ;
                    ARJ = AQW;
                    ART = AQU;
                    ATP = ARB;
                    AUF = ARC;
                } else {
                    ARD = AFK;
                    ARH = AFO;
                    ARJ = A;
                    ART = AGA;
                    ATP = A;
                    AUF = AIM;
                }
                let AVK;
                let AVM;
                let AVZ;
                let AWY;
                let BBQ;
                if FK != 0.0 {
                    AVK = ARX;
                    AVM = ARZ;
                    AVZ = ASM;
                    AWY = ATL;
                    BBQ = A;
                } else {
                    let ARE = CQ * ARD;
                    let ARF = if HS == A { 1.0 } else { 0.0 };
                    let ARG = if (if HR == A { 1.0 } else { 0.0 }) != 0.0 && ARF != 0.0 { 1.0 } else { 0.0 };
                    let ARW;
                    let ARY;
                    let ASL;
                    let ATK;
                    let AUO;
                    if ARG != 0.0 {
                        ARW = ARX;
                        ARY = ARZ;
                        ASL = ASM;
                        ATK = ATL;
                        AUO = A;
                    } else {
                        let ARI = CX - ARH;
                        let ARK = E - ((E - (ARJ / ARI)).sqrt());
                        let ARL = if T == C { 1.0 } else { 0.0 };
                        let ARN = if ARL != 0.0 {
                            A
                        } else {
                            let ARM = ((((ARK * ARK) * (ARK.ln())) / (E - ARK)) + ARK) * (E - (AT * T));
                            ARM
                        };
                        let ARO = ARK + ARN;
                        let ARR = if ARL != 0.0 {
                            let ARP = (ARI * AM).sqrt();
                            ARP
                        } else {
                            let ARQ = (ARI * AM).powf(T);
                            ARQ
                        };
                        let ARS = AD * ARR;
                        let ARU = CN * ((ART - E) * ARS);
                        let ARV = HR * (ARU * ARO);
                        ARW = ARS;
                        ARY = ARI;
                        ASL = ARO;
                        ATK = ARU;
                        AUO = ARV;
                    }
                    let AUP;
                    if ARF != 0.0 {
                        AUP = A;
                    } else {
                        let ASA = DL * ((ARW * U) / ARY);
                        let ASB = (IN * DG) / ASA;
                        let ASC = ASB * ASB;
                        let ASD = ASC * ASC;
                        let ASE = (ASD / (ASD + E)).sqrt();
                        let ASF = ASE.sqrt();
                        let ASG = ASE * ASF;
                        let ASH = (-T) * Z;
                        let ASI = if ASH == -1e0f64 { 1.0 } else { 0.0 };
                        let ASN = if ASI != 0.0 {
                            let ASJ = E / (E + (ASA * ASG));
                            ASJ
                        } else {
                            let ASK = (E + (ASA * ASG)).powf(ASH);
                            ASK
                        };
                        let ASO = (ASL * ASN) / (ASL + ASN);
                        let ASP = (JB * (ASA / ASF)).sqrt();
                        let ASQ = (((DG * ASB) * ASF) - (DG * ASE)) + (C * (ASA * ASG));
                        let ASR = (((AT * (ASB * ASF)) - ASE) - E) * ASP;
                        let ASS = ASR * ASR;
                        let AST = if ASR > A { 1.0 } else { 0.0 };
                        let ATA = if AST != 0.0 {
                            let ASU = E / (E + (AS * ASR));
                            ASU
                        } else {
                            let ASV = E / (E - (AS * ASR));
                            ASV
                        };
                        let ASW = (-ASS) + ASQ;
                        let ASX = if ASW > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                        let ATC = if ASX != 0.0 {
                            let ASY = ASW.exp();
                            ASY
                        } else {
                            let ASZ = FE / (E + ((-2.3025850929940458e2f64 - ASW) * (E + (C * ((-2.3025850929940458e2f64 - ASW) * (E + ((-2.3025850929940458e2f64 - ASW) * FF)))))));
                            ASZ
                        };
                        let ATB = ATA * ATA;
                        let ATD = (((AR * ATA) + (AV * ATB)) + (AW * (ATB * ATA))) * ATC;
                        let ATJ;
                        if AST != 0.0 {
                            ATJ = ATD;
                        } else {
                            let ATE = if ASQ > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            let ATH = if ATE != 0.0 {
                                let ATF = ASQ.exp();
                                ATF
                            } else {
                                let ATG = FE / (E + ((-2.3025850929940458e2f64 - ASQ) * (E + (C * ((-2.3025850929940458e2f64 - ASQ) * (E + ((-2.3025850929940458e2f64 - ASQ) * FF)))))));
                                ATG
                            };
                            let ATI = (AT * ATH) - ATD;
                            ATJ = ATI;
                        }
                        let ATM = HS * ((ATK * (8.86226925452758e-1f64 * ((DG * ATJ) / ASP))) * ASO);
                        AUP = ATM;
                    }
                    let ATN = if JZ == A { 1.0 } else { 0.0 };
                    let AUQ;
                    if ATN != 0.0 {
                        AUQ = A;
                    } else {
                        let ATO = if T == C { 1.0 } else { 0.0 };
                        let ATS = if ATO != 0.0 {
                            let ATQ = ((AL - ATP) * AM).sqrt();
                            ATQ
                        } else {
                            let ATR = ((AL - ATP) * AM).powf(T);
                            ATR
                        };
                        let ATT = Z * (((AL - ATP) * AI) / ATS);
                        let ATU = (-DT) / ATT;
                        let ATV = if (ATU.abs()) < FA { 1.0 } else { 0.0 };
                        let AUB;
                        if ATV != 0.0 {
                            let ATW = ATU.exp();
                            AUB = ATW;
                        } else {
                            let ATX = if ATU < A { 1.0 } else { 0.0 };
                            let AUC = if ATX != 0.0 {
                                let ATY = FE / (E + ((-2.3025850929940458e2f64 - ATU) * (E + (C * ((-2.3025850929940458e2f64 - ATU) * (E + ((-2.3025850929940458e2f64 - ATU) * FF)))))));
                                ATY
                            } else {
                                let ATZ = ATU - FA;
                                let AUA = FH * (E + (ATZ * (E + (C * (ATZ * (E + (ATZ * FF)))))));
                                AUA
                            };
                            AUB = AUC;
                        }
                        let AUD = JZ * (((FX * ATT) * ATT) * AUB);
                        AUQ = AUD;
                    }
                    let AUE = if BE > KR { 1.0 } else { 0.0 };
                    let AUR;
                    if AUE != 0.0 {
                        AUR = E;
                    } else {
                        let AUG = if AUF > ((-AX) * BE) { 1.0 } else { 0.0 };
                        let AUS;
                        if AUG != 0.0 {
                            let AUH = if AY == GK { 1.0 } else { 0.0 };
                            let AUL = if AUH != 0.0 {
                                let AUI = AUF * BF;
                                let AUJ = ((AUI * AUI) * AUI) * AUI;
                                AUJ
                            } else {
                                let AUK = ((AUF * BF).abs()).powf(AY);
                                AUK
                            };
                            let AUM = E / (E - AUL);
                            AUS = AUM;
                        } else {
                            let AUN = AZ + ((AUF + (AX * BE)) * BK);
                            AUS = AUN;
                        }
                        AUR = AUS;
                    }
                    let AUT = (LC * (((ARE + AUO) + AUP) + AUQ)) * AUR;
                    AVK = ARW;
                    AVM = ARY;
                    AVZ = ASL;
                    AWY = ATK;
                    BBQ = AUT;
                }
                let AYV;
                let AYX;
                let AZK;
                let BAJ;
                let BBR;
                if FN != 0.0 {
                    AYV = AVK;
                    AYX = AVM;
                    AZK = AVZ;
                    BAJ = AWY;
                    BBR = A;
                } else {
                    let AUU = CR * ARD;
                    let AUV = if LL == A { 1.0 } else { 0.0 };
                    let AUW = if (if LK == A { 1.0 } else { 0.0 }) != 0.0 && AUV != 0.0 { 1.0 } else { 0.0 };
                    let AVJ;
                    let AVL;
                    let AVY;
                    let AWX;
                    let AXZ;
                    if AUW != 0.0 {
                        AVJ = AVK;
                        AVL = AVM;
                        AVY = AVZ;
                        AWX = AWY;
                        AXZ = A;
                    } else {
                        let AUX = CY - ARH;
                        let AUY = E - ((E - (ARJ / AUX)).sqrt());
                        let AUZ = if V == C { 1.0 } else { 0.0 };
                        let AVB = if AUZ != 0.0 {
                            A
                        } else {
                            let AVA = ((((AUY * AUY) * (AUY.ln())) / (E - AUY)) + AUY) * (E - (AT * V));
                            AVA
                        };
                        let AVC = AUY + AVB;
                        let AVF = if AUZ != 0.0 {
                            let AVD = (AUX * AO).sqrt();
                            AVD
                        } else {
                            let AVE = (AUX * AO).powf(V);
                            AVE
                        };
                        let AVG = AF * AVF;
                        let AVH = CO * ((ART - E) * AVG);
                        let AVI = LK * (AVH * AVC);
                        AVJ = AVG;
                        AVL = AUX;
                        AVY = AVC;
                        AWX = AVH;
                        AXZ = AVI;
                    }
                    let AYA;
                    if AUV != 0.0 {
                        AYA = A;
                    } else {
                        let AVN = DM * ((AVJ * W) / AVL);
                        let AVO = (IN * DH) / AVN;
                        let AVP = AVO * AVO;
                        let AVQ = AVP * AVP;
                        let AVR = (AVQ / (AVQ + E)).sqrt();
                        let AVS = AVR.sqrt();
                        let AVT = AVR * AVS;
                        let AVU = (-V) * AA;
                        let AVV = if AVU == -1e0f64 { 1.0 } else { 0.0 };
                        let AWA = if AVV != 0.0 {
                            let AVW = E / (E + (AVN * AVT));
                            AVW
                        } else {
                            let AVX = (E + (AVN * AVT)).powf(AVU);
                            AVX
                        };
                        let AWB = (AVY * AWA) / (AVY + AWA);
                        let AWC = (JB * (AVN / AVS)).sqrt();
                        let AWD = (((DH * AVO) * AVS) - (DH * AVR)) + (C * (AVN * AVT));
                        let AWE = (((AT * (AVO * AVS)) - AVR) - E) * AWC;
                        let AWF = AWE * AWE;
                        let AWG = if AWE > A { 1.0 } else { 0.0 };
                        let AWN = if AWG != 0.0 {
                            let AWH = E / (E + (AS * AWE));
                            AWH
                        } else {
                            let AWI = E / (E - (AS * AWE));
                            AWI
                        };
                        let AWJ = (-AWF) + AWD;
                        let AWK = if AWJ > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                        let AWP = if AWK != 0.0 {
                            let AWL = AWJ.exp();
                            AWL
                        } else {
                            let AWM = FE / (E + ((-2.3025850929940458e2f64 - AWJ) * (E + (C * ((-2.3025850929940458e2f64 - AWJ) * (E + ((-2.3025850929940458e2f64 - AWJ) * FF)))))));
                            AWM
                        };
                        let AWO = AWN * AWN;
                        let AWQ = (((AR * AWN) + (AV * AWO)) + (AW * (AWO * AWN))) * AWP;
                        let AWW;
                        if AWG != 0.0 {
                            AWW = AWQ;
                        } else {
                            let AWR = if AWD > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            let AWU = if AWR != 0.0 {
                                let AWS = AWD.exp();
                                AWS
                            } else {
                                let AWT = FE / (E + ((-2.3025850929940458e2f64 - AWD) * (E + (C * ((-2.3025850929940458e2f64 - AWD) * (E + ((-2.3025850929940458e2f64 - AWD) * FF)))))));
                                AWT
                            };
                            let AWV = (AT * AWU) - AWQ;
                            AWW = AWV;
                        }
                        let AWZ = LL * ((AWX * (8.86226925452758e-1f64 * ((DH * AWW) / AWC))) * AWB);
                        AYA = AWZ;
                    }
                    let AXA = if NR == A { 1.0 } else { 0.0 };
                    let AYB;
                    if AXA != 0.0 {
                        AYB = A;
                    } else {
                        let AXB = if V == C { 1.0 } else { 0.0 };
                        let AXE = if AXB != 0.0 {
                            let AXC = ((AN - ATP) * AO).sqrt();
                            AXC
                        } else {
                            let AXD = ((AN - ATP) * AO).powf(V);
                            AXD
                        };
                        let AXF = AA * (((AN - ATP) * AJ) / AXE);
                        let AXG = (-DV) / AXF;
                        let AXH = if (AXG.abs()) < FA { 1.0 } else { 0.0 };
                        let AXN;
                        if AXH != 0.0 {
                            let AXI = AXG.exp();
                            AXN = AXI;
                        } else {
                            let AXJ = if AXG < A { 1.0 } else { 0.0 };
                            let AXO = if AXJ != 0.0 {
                                let AXK = FE / (E + ((-2.3025850929940458e2f64 - AXG) * (E + (C * ((-2.3025850929940458e2f64 - AXG) * (E + ((-2.3025850929940458e2f64 - AXG) * FF)))))));
                                AXK
                            } else {
                                let AXL = AXG - FA;
                                let AXM = FH * (E + (AXL * (E + (C * (AXL * (E + (AXL * FF)))))));
                                AXM
                            };
                            AXN = AXO;
                        }
                        let AXP = NR * (((FX * AXF) * AXF) * AXN);
                        AYB = AXP;
                    }
                    let AXQ = if BG > KR { 1.0 } else { 0.0 };
                    let AYC;
                    if AXQ != 0.0 {
                        AYC = E;
                    } else {
                        let AXR = if AUF > ((-AX) * BG) { 1.0 } else { 0.0 };
                        let AYD;
                        if AXR != 0.0 {
                            let AXS = if BA == GK { 1.0 } else { 0.0 };
                            let AXW = if AXS != 0.0 {
                                let AXT = AUF * BH;
                                let AXU = ((AXT * AXT) * AXT) * AXT;
                                AXU
                            } else {
                                let AXV = ((AUF * BH).abs()).powf(BA);
                                AXV
                            };
                            let AXX = E / (E - AXW);
                            AYD = AXX;
                        } else {
                            let AXY = BB + ((AUF + (AX * BG)) * BL);
                            AYD = AXY;
                        }
                        AYC = AYD;
                    }
                    let AYE = (LC * (((AUU + AXZ) + AYA) + AYB)) * AYC;
                    AYV = AVJ;
                    AYX = AVL;
                    AZK = AVY;
                    BAJ = AWX;
                    BBR = AYE;
                }
                let BBS;
                let BDQ;
                let BDS;
                let BEF;
                let BFE;
                if FQ != 0.0 {
                    BBS = A;
                    BDQ = AYV;
                    BDS = AYX;
                    BEF = AZK;
                    BFE = BAJ;
                } else {
                    let AYF = CS * ARD;
                    let AYG = if OZ == A { 1.0 } else { 0.0 };
                    let AYH = if (if OY == A { 1.0 } else { 0.0 }) != 0.0 && AYG != 0.0 { 1.0 } else { 0.0 };
                    let AYU;
                    let AYW;
                    let AZJ;
                    let BAI;
                    let BBK;
                    if AYH != 0.0 {
                        AYU = AYV;
                        AYW = AYX;
                        AZJ = AZK;
                        BAI = BAJ;
                        BBK = A;
                    } else {
                        let AYI = CZ - ARH;
                        let AYJ = E - ((E - (ARJ / AYI)).sqrt());
                        let AYK = if X == C { 1.0 } else { 0.0 };
                        let AYM = if AYK != 0.0 {
                            A
                        } else {
                            let AYL = ((((AYJ * AYJ) * (AYJ.ln())) / (E - AYJ)) + AYJ) * (E - (AT * X));
                            AYL
                        };
                        let AYN = AYJ + AYM;
                        let AYQ = if AYK != 0.0 {
                            let AYO = (AYI * AQ).sqrt();
                            AYO
                        } else {
                            let AYP = (AYI * AQ).powf(X);
                            AYP
                        };
                        let AYR = AH * AYQ;
                        let AYS = CP * ((ART - E) * AYR);
                        let AYT = OY * (AYS * AYN);
                        AYU = AYR;
                        AYW = AYI;
                        AZJ = AYN;
                        BAI = AYS;
                        BBK = AYT;
                    }
                    let BBL;
                    if AYG != 0.0 {
                        BBL = A;
                    } else {
                        let AYY = DN * ((AYU * Y) / AYW);
                        let AYZ = (IN * DI) / AYY;
                        let AZA = AYZ * AYZ;
                        let AZB = AZA * AZA;
                        let AZC = (AZB / (AZB + E)).sqrt();
                        let AZD = AZC.sqrt();
                        let AZE = AZC * AZD;
                        let AZF = (-X) * AB;
                        let AZG = if AZF == -1e0f64 { 1.0 } else { 0.0 };
                        let AZL = if AZG != 0.0 {
                            let AZH = E / (E + (AYY * AZE));
                            AZH
                        } else {
                            let AZI = (E + (AYY * AZE)).powf(AZF);
                            AZI
                        };
                        let AZM = (AZJ * AZL) / (AZJ + AZL);
                        let AZN = (JB * (AYY / AZD)).sqrt();
                        let AZO = (((DI * AYZ) * AZD) - (DI * AZC)) + (C * (AYY * AZE));
                        let AZP = (((AT * (AYZ * AZD)) - AZC) - E) * AZN;
                        let AZQ = AZP * AZP;
                        let AZR = if AZP > A { 1.0 } else { 0.0 };
                        let AZY = if AZR != 0.0 {
                            let AZS = E / (E + (AS * AZP));
                            AZS
                        } else {
                            let AZT = E / (E - (AS * AZP));
                            AZT
                        };
                        let AZU = (-AZQ) + AZO;
                        let AZV = if AZU > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                        let BAA = if AZV != 0.0 {
                            let AZW = AZU.exp();
                            AZW
                        } else {
                            let AZX = FE / (E + ((-2.3025850929940458e2f64 - AZU) * (E + (C * ((-2.3025850929940458e2f64 - AZU) * (E + ((-2.3025850929940458e2f64 - AZU) * FF)))))));
                            AZX
                        };
                        let AZZ = AZY * AZY;
                        let BAB = (((AR * AZY) + (AV * AZZ)) + (AW * (AZZ * AZY))) * BAA;
                        let BAH;
                        if AZR != 0.0 {
                            BAH = BAB;
                        } else {
                            let BAC = if AZO > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            let BAF = if BAC != 0.0 {
                                let BAD = AZO.exp();
                                BAD
                            } else {
                                let BAE = FE / (E + ((-2.3025850929940458e2f64 - AZO) * (E + (C * ((-2.3025850929940458e2f64 - AZO) * (E + ((-2.3025850929940458e2f64 - AZO) * FF)))))));
                                BAE
                            };
                            let BAG = (AT * BAF) - BAB;
                            BAH = BAG;
                        }
                        let BAK = OZ * ((BAI * (8.86226925452758e-1f64 * ((DI * BAH) / AZN))) * AZM);
                        BBL = BAK;
                    }
                    let BAL = if RF == A { 1.0 } else { 0.0 };
                    let BBM;
                    if BAL != 0.0 {
                        BBM = A;
                    } else {
                        let BAM = if X == C { 1.0 } else { 0.0 };
                        let BAP = if BAM != 0.0 {
                            let BAN = ((AP - ATP) * AQ).sqrt();
                            BAN
                        } else {
                            let BAO = ((AP - ATP) * AQ).powf(X);
                            BAO
                        };
                        let BAQ = AB * (((AP - ATP) * AK) / BAP);
                        let BAR = (-DX) / BAQ;
                        let BAS = if (BAR.abs()) < FA { 1.0 } else { 0.0 };
                        let BAY;
                        if BAS != 0.0 {
                            let BAT = BAR.exp();
                            BAY = BAT;
                        } else {
                            let BAU = if BAR < A { 1.0 } else { 0.0 };
                            let BAZ = if BAU != 0.0 {
                                let BAV = FE / (E + ((-2.3025850929940458e2f64 - BAR) * (E + (C * ((-2.3025850929940458e2f64 - BAR) * (E + ((-2.3025850929940458e2f64 - BAR) * FF)))))));
                                BAV
                            } else {
                                let BAW = BAR - FA;
                                let BAX = FH * (E + (BAW * (E + (C * (BAW * (E + (BAW * FF)))))));
                                BAX
                            };
                            BAY = BAZ;
                        }
                        let BBA = RF * (((FX * BAQ) * BAQ) * BAY);
                        BBM = BBA;
                    }
                    let BBB = if BI > KR { 1.0 } else { 0.0 };
                    let BBN;
                    if BBB != 0.0 {
                        BBN = E;
                    } else {
                        let BBC = if AUF > ((-AX) * BI) { 1.0 } else { 0.0 };
                        let BBO;
                        if BBC != 0.0 {
                            let BBD = if BC == GK { 1.0 } else { 0.0 };
                            let BBH = if BBD != 0.0 {
                                let BBE = AUF * BJ;
                                let BBF = ((BBE * BBE) * BBE) * BBE;
                                BBF
                            } else {
                                let BBG = ((AUF * BJ).abs()).powf(BC);
                                BBG
                            };
                            let BBI = E / (E - BBH);
                            BBO = BBI;
                        } else {
                            let BBJ = BD + ((AUF + (AX * BI)) * BM);
                            BBO = BBJ;
                        }
                        BBN = BBO;
                    }
                    let BBP = (LC * (((AYF + BBK) + BBL) + BBM)) * BBN;
                    BBS = BBP;
                    BDQ = AYU;
                    BDS = AYW;
                    BEF = AZJ;
                    BFE = BAI;
                }
                let BBT = ((EA * BBQ) + (ED * BBR)) + (EG * BBS);
                let BCW;
                let BDA;
                let BDC;
                let BDM;
                let BFI;
                let BFY;
                if GJ != 0.0 {
                    let BBU = if GI < EY { 1.0 } else { 0.0 };
                    let BCI;
                    let BCL;
                    let BCN;
                    if BBU != 0.0 {
                        let BBV = GI * CH;
                        let BBW = if ((-5e-1f64 * BBV).abs()) < FA { 1.0 } else { 0.0 };
                        let BCB;
                        if BBW != 0.0 {
                            let BBX = (-5e-1f64 * BBV).exp();
                            BCB = BBX;
                        } else {
                            let BBY = if (-5e-1f64 * BBV) < A { 1.0 } else { 0.0 };
                            let BCC = if BBY != 0.0 {
                                let BBZ = FE / (E + ((-2.3025850929940458e2f64 - (-5e-1f64 * BBV)) * (E + (C * ((-2.3025850929940458e2f64 - (-5e-1f64 * BBV)) * (E + ((-2.3025850929940458e2f64 - (-5e-1f64 * BBV)) * FF)))))));
                                BBZ
                            } else {
                                let BCA = FH * (E + (((-5e-1f64 * BBV) - FA) * (E + (C * (((-5e-1f64 * BBV) - FA) * (E + (((-5e-1f64 * BBV) - FA) * FF)))))));
                                BCA
                            };
                            BCB = BCC;
                        }
                        let BCD = E / BCB;
                        let BCE = BCD * BCD;
                        BCI = BCE;
                        BCL = BCB;
                        BCN = BCD;
                    } else {
                        let BCF = (E + ((GI - EY) * CH)) * GW;
                        let BCG = BCF.sqrt();
                        let BCH = E / BCG;
                        BCI = BCF;
                        BCL = BCH;
                        BCN = BCG;
                    }
                    let BCJ = BCI - E;
                    let BCP = if BCK != 0.0 {
                        let BCM = AT * (CG * (((AT + BCL) + (((BCL + E) * (BCL + AU)).sqrt())).ln()));
                        BCM
                    } else {
                        let BCO = -2e-1f64 + (AT * (CG * ((((AT * BCN) + E) + (((E + BCN) * (E + (AU * BCN))).sqrt())).ln())));
                        BCO
                    };
                    let BCQ = FW - BCP;
                    let BCR = GI - BCQ;
                    let BCS = C * ((GI + BCQ) - (((BCR * BCR) + ((GK * CG) * CG)).sqrt()));
                    let BCT = GI - GB;
                    let BCU = C * ((GI + GB) - (((BCT * BCT) + ((GK * I) * I)).sqrt()));
                    BCW = BCJ;
                    BDA = BCS;
                    BDC = BCP;
                    BDM = BCN;
                    BFI = BCU;
                    BFY = BCV;
                } else {
                    BCW = ARD;
                    BDA = ARH;
                    BDC = A;
                    BDM = ART;
                    BFI = A;
                    BFY = AUF;
                }
                let BHD;
                let BHF;
                let BHS;
                let BIR;
                let BNJ;
                if FK != 0.0 {
                    BHD = BDQ;
                    BHF = BDS;
                    BHS = BEF;
                    BIR = BFE;
                    BNJ = A;
                } else {
                    let BCX = CQ * BCW;
                    let BCY = if HS == A { 1.0 } else { 0.0 };
                    let BCZ = if (if HR == A { 1.0 } else { 0.0 }) != 0.0 && BCY != 0.0 { 1.0 } else { 0.0 };
                    let BDP;
                    let BDR;
                    let BEE;
                    let BFD;
                    let BGH;
                    if BCZ != 0.0 {
                        BDP = BDQ;
                        BDR = BDS;
                        BEE = BEF;
                        BFD = BFE;
                        BGH = A;
                    } else {
                        let BDB = CX - BDA;
                        let BDD = E - ((E - (BDC / BDB)).sqrt());
                        let BDE = if T == C { 1.0 } else { 0.0 };
                        let BDG = if BDE != 0.0 {
                            A
                        } else {
                            let BDF = ((((BDD * BDD) * (BDD.ln())) / (E - BDD)) + BDD) * (E - (AT * T));
                            BDF
                        };
                        let BDH = BDD + BDG;
                        let BDK = if BDE != 0.0 {
                            let BDI = (BDB * AM).sqrt();
                            BDI
                        } else {
                            let BDJ = (BDB * AM).powf(T);
                            BDJ
                        };
                        let BDL = AD * BDK;
                        let BDN = CN * ((BDM - E) * BDL);
                        let BDO = HR * (BDN * BDH);
                        BDP = BDL;
                        BDR = BDB;
                        BEE = BDH;
                        BFD = BDN;
                        BGH = BDO;
                    }
                    let BGI;
                    if BCY != 0.0 {
                        BGI = A;
                    } else {
                        let BDT = DL * ((BDP * U) / BDR);
                        let BDU = (IN * DG) / BDT;
                        let BDV = BDU * BDU;
                        let BDW = BDV * BDV;
                        let BDX = (BDW / (BDW + E)).sqrt();
                        let BDY = BDX.sqrt();
                        let BDZ = BDX * BDY;
                        let BEA = (-T) * Z;
                        let BEB = if BEA == -1e0f64 { 1.0 } else { 0.0 };
                        let BEG = if BEB != 0.0 {
                            let BEC = E / (E + (BDT * BDZ));
                            BEC
                        } else {
                            let BED = (E + (BDT * BDZ)).powf(BEA);
                            BED
                        };
                        let BEH = (BEE * BEG) / (BEE + BEG);
                        let BEI = (JB * (BDT / BDY)).sqrt();
                        let BEJ = (((DG * BDU) * BDY) - (DG * BDX)) + (C * (BDT * BDZ));
                        let BEK = (((AT * (BDU * BDY)) - BDX) - E) * BEI;
                        let BEL = BEK * BEK;
                        let BEM = if BEK > A { 1.0 } else { 0.0 };
                        let BET = if BEM != 0.0 {
                            let BEN = E / (E + (AS * BEK));
                            BEN
                        } else {
                            let BEO = E / (E - (AS * BEK));
                            BEO
                        };
                        let BEP = (-BEL) + BEJ;
                        let BEQ = if BEP > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                        let BEV = if BEQ != 0.0 {
                            let BER = BEP.exp();
                            BER
                        } else {
                            let BES = FE / (E + ((-2.3025850929940458e2f64 - BEP) * (E + (C * ((-2.3025850929940458e2f64 - BEP) * (E + ((-2.3025850929940458e2f64 - BEP) * FF)))))));
                            BES
                        };
                        let BEU = BET * BET;
                        let BEW = (((AR * BET) + (AV * BEU)) + (AW * (BEU * BET))) * BEV;
                        let BFC;
                        if BEM != 0.0 {
                            BFC = BEW;
                        } else {
                            let BEX = if BEJ > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            let BFA = if BEX != 0.0 {
                                let BEY = BEJ.exp();
                                BEY
                            } else {
                                let BEZ = FE / (E + ((-2.3025850929940458e2f64 - BEJ) * (E + (C * ((-2.3025850929940458e2f64 - BEJ) * (E + ((-2.3025850929940458e2f64 - BEJ) * FF)))))));
                                BEZ
                            };
                            let BFB = (AT * BFA) - BEW;
                            BFC = BFB;
                        }
                        let BFF = HS * ((BFD * (8.86226925452758e-1f64 * ((DG * BFC) / BEI))) * BEH);
                        BGI = BFF;
                    }
                    let BFG = if JZ == A { 1.0 } else { 0.0 };
                    let BGJ;
                    if BFG != 0.0 {
                        BGJ = A;
                    } else {
                        let BFH = if T == C { 1.0 } else { 0.0 };
                        let BFL = if BFH != 0.0 {
                            let BFJ = ((AL - BFI) * AM).sqrt();
                            BFJ
                        } else {
                            let BFK = ((AL - BFI) * AM).powf(T);
                            BFK
                        };
                        let BFM = Z * (((AL - BFI) * AI) / BFL);
                        let BFN = (-DT) / BFM;
                        let BFO = if (BFN.abs()) < FA { 1.0 } else { 0.0 };
                        let BFU;
                        if BFO != 0.0 {
                            let BFP = BFN.exp();
                            BFU = BFP;
                        } else {
                            let BFQ = if BFN < A { 1.0 } else { 0.0 };
                            let BFV = if BFQ != 0.0 {
                                let BFR = FE / (E + ((-2.3025850929940458e2f64 - BFN) * (E + (C * ((-2.3025850929940458e2f64 - BFN) * (E + ((-2.3025850929940458e2f64 - BFN) * FF)))))));
                                BFR
                            } else {
                                let BFS = BFN - FA;
                                let BFT = FH * (E + (BFS * (E + (C * (BFS * (E + (BFS * FF)))))));
                                BFT
                            };
                            BFU = BFV;
                        }
                        let BFW = JZ * (((GI * BFM) * BFM) * BFU);
                        BGJ = BFW;
                    }
                    let BFX = if BE > KR { 1.0 } else { 0.0 };
                    let BGK;
                    if BFX != 0.0 {
                        BGK = E;
                    } else {
                        let BFZ = if BFY > ((-AX) * BE) { 1.0 } else { 0.0 };
                        let BGL;
                        if BFZ != 0.0 {
                            let BGA = if AY == GK { 1.0 } else { 0.0 };
                            let BGE = if BGA != 0.0 {
                                let BGB = BFY * BF;
                                let BGC = ((BGB * BGB) * BGB) * BGB;
                                BGC
                            } else {
                                let BGD = ((BFY * BF).abs()).powf(AY);
                                BGD
                            };
                            let BGF = E / (E - BGE);
                            BGL = BGF;
                        } else {
                            let BGG = AZ + ((BFY + (AX * BE)) * BK);
                            BGL = BGG;
                        }
                        BGK = BGL;
                    }
                    let BGM = (LC * (((BCX + BGH) + BGI) + BGJ)) * BGK;
                    BHD = BDP;
                    BHF = BDR;
                    BHS = BEE;
                    BIR = BFD;
                    BNJ = BGM;
                }
                let BKO;
                let BKQ;
                let BLD;
                let BMC;
                let BNK;
                if FN != 0.0 {
                    BKO = BHD;
                    BKQ = BHF;
                    BLD = BHS;
                    BMC = BIR;
                    BNK = A;
                } else {
                    let BGN = CR * BCW;
                    let BGO = if LL == A { 1.0 } else { 0.0 };
                    let BGP = if (if LK == A { 1.0 } else { 0.0 }) != 0.0 && BGO != 0.0 { 1.0 } else { 0.0 };
                    let BHC;
                    let BHE;
                    let BHR;
                    let BIQ;
                    let BJS;
                    if BGP != 0.0 {
                        BHC = BHD;
                        BHE = BHF;
                        BHR = BHS;
                        BIQ = BIR;
                        BJS = A;
                    } else {
                        let BGQ = CY - BDA;
                        let BGR = E - ((E - (BDC / BGQ)).sqrt());
                        let BGS = if V == C { 1.0 } else { 0.0 };
                        let BGU = if BGS != 0.0 {
                            A
                        } else {
                            let BGT = ((((BGR * BGR) * (BGR.ln())) / (E - BGR)) + BGR) * (E - (AT * V));
                            BGT
                        };
                        let BGV = BGR + BGU;
                        let BGY = if BGS != 0.0 {
                            let BGW = (BGQ * AO).sqrt();
                            BGW
                        } else {
                            let BGX = (BGQ * AO).powf(V);
                            BGX
                        };
                        let BGZ = AF * BGY;
                        let BHA = CO * ((BDM - E) * BGZ);
                        let BHB = LK * (BHA * BGV);
                        BHC = BGZ;
                        BHE = BGQ;
                        BHR = BGV;
                        BIQ = BHA;
                        BJS = BHB;
                    }
                    let BJT;
                    if BGO != 0.0 {
                        BJT = A;
                    } else {
                        let BHG = DM * ((BHC * W) / BHE);
                        let BHH = (IN * DH) / BHG;
                        let BHI = BHH * BHH;
                        let BHJ = BHI * BHI;
                        let BHK = (BHJ / (BHJ + E)).sqrt();
                        let BHL = BHK.sqrt();
                        let BHM = BHK * BHL;
                        let BHN = (-V) * AA;
                        let BHO = if BHN == -1e0f64 { 1.0 } else { 0.0 };
                        let BHT = if BHO != 0.0 {
                            let BHP = E / (E + (BHG * BHM));
                            BHP
                        } else {
                            let BHQ = (E + (BHG * BHM)).powf(BHN);
                            BHQ
                        };
                        let BHU = (BHR * BHT) / (BHR + BHT);
                        let BHV = (JB * (BHG / BHL)).sqrt();
                        let BHW = (((DH * BHH) * BHL) - (DH * BHK)) + (C * (BHG * BHM));
                        let BHX = (((AT * (BHH * BHL)) - BHK) - E) * BHV;
                        let BHY = BHX * BHX;
                        let BHZ = if BHX > A { 1.0 } else { 0.0 };
                        let BIG = if BHZ != 0.0 {
                            let BIA = E / (E + (AS * BHX));
                            BIA
                        } else {
                            let BIB = E / (E - (AS * BHX));
                            BIB
                        };
                        let BIC = (-BHY) + BHW;
                        let BID = if BIC > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                        let BII = if BID != 0.0 {
                            let BIE = BIC.exp();
                            BIE
                        } else {
                            let BIF = FE / (E + ((-2.3025850929940458e2f64 - BIC) * (E + (C * ((-2.3025850929940458e2f64 - BIC) * (E + ((-2.3025850929940458e2f64 - BIC) * FF)))))));
                            BIF
                        };
                        let BIH = BIG * BIG;
                        let BIJ = (((AR * BIG) + (AV * BIH)) + (AW * (BIH * BIG))) * BII;
                        let BIP;
                        if BHZ != 0.0 {
                            BIP = BIJ;
                        } else {
                            let BIK = if BHW > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            let BIN = if BIK != 0.0 {
                                let BIL = BHW.exp();
                                BIL
                            } else {
                                let BIM = FE / (E + ((-2.3025850929940458e2f64 - BHW) * (E + (C * ((-2.3025850929940458e2f64 - BHW) * (E + ((-2.3025850929940458e2f64 - BHW) * FF)))))));
                                BIM
                            };
                            let BIO = (AT * BIN) - BIJ;
                            BIP = BIO;
                        }
                        let BIS = LL * ((BIQ * (8.86226925452758e-1f64 * ((DH * BIP) / BHV))) * BHU);
                        BJT = BIS;
                    }
                    let BIT = if NR == A { 1.0 } else { 0.0 };
                    let BJU;
                    if BIT != 0.0 {
                        BJU = A;
                    } else {
                        let BIU = if V == C { 1.0 } else { 0.0 };
                        let BIX = if BIU != 0.0 {
                            let BIV = ((AN - BFI) * AO).sqrt();
                            BIV
                        } else {
                            let BIW = ((AN - BFI) * AO).powf(V);
                            BIW
                        };
                        let BIY = AA * (((AN - BFI) * AJ) / BIX);
                        let BIZ = (-DV) / BIY;
                        let BJA = if (BIZ.abs()) < FA { 1.0 } else { 0.0 };
                        let BJG;
                        if BJA != 0.0 {
                            let BJB = BIZ.exp();
                            BJG = BJB;
                        } else {
                            let BJC = if BIZ < A { 1.0 } else { 0.0 };
                            let BJH = if BJC != 0.0 {
                                let BJD = FE / (E + ((-2.3025850929940458e2f64 - BIZ) * (E + (C * ((-2.3025850929940458e2f64 - BIZ) * (E + ((-2.3025850929940458e2f64 - BIZ) * FF)))))));
                                BJD
                            } else {
                                let BJE = BIZ - FA;
                                let BJF = FH * (E + (BJE * (E + (C * (BJE * (E + (BJE * FF)))))));
                                BJF
                            };
                            BJG = BJH;
                        }
                        let BJI = NR * (((GI * BIY) * BIY) * BJG);
                        BJU = BJI;
                    }
                    let BJJ = if BG > KR { 1.0 } else { 0.0 };
                    let BJV;
                    if BJJ != 0.0 {
                        BJV = E;
                    } else {
                        let BJK = if BFY > ((-AX) * BG) { 1.0 } else { 0.0 };
                        let BJW;
                        if BJK != 0.0 {
                            let BJL = if BA == GK { 1.0 } else { 0.0 };
                            let BJP = if BJL != 0.0 {
                                let BJM = BFY * BH;
                                let BJN = ((BJM * BJM) * BJM) * BJM;
                                BJN
                            } else {
                                let BJO = ((BFY * BH).abs()).powf(BA);
                                BJO
                            };
                            let BJQ = E / (E - BJP);
                            BJW = BJQ;
                        } else {
                            let BJR = BB + ((BFY + (AX * BG)) * BL);
                            BJW = BJR;
                        }
                        BJV = BJW;
                    }
                    let BJX = (LC * (((BGN + BJS) + BJT) + BJU)) * BJV;
                    BKO = BHC;
                    BKQ = BHE;
                    BLD = BHR;
                    BMC = BIQ;
                    BNK = BJX;
                }
                let BNL;
                if FQ != 0.0 {
                    BNL = A;
                } else {
                    let BJY = CS * BCW;
                    let BJZ = if OZ == A { 1.0 } else { 0.0 };
                    let BKA = if (if OY == A { 1.0 } else { 0.0 }) != 0.0 && BJZ != 0.0 { 1.0 } else { 0.0 };
                    let BKN;
                    let BKP;
                    let BLC;
                    let BMB;
                    let BND;
                    if BKA != 0.0 {
                        BKN = BKO;
                        BKP = BKQ;
                        BLC = BLD;
                        BMB = BMC;
                        BND = A;
                    } else {
                        let BKB = CZ - BDA;
                        let BKC = E - ((E - (BDC / BKB)).sqrt());
                        let BKD = if X == C { 1.0 } else { 0.0 };
                        let BKF = if BKD != 0.0 {
                            A
                        } else {
                            let BKE = ((((BKC * BKC) * (BKC.ln())) / (E - BKC)) + BKC) * (E - (AT * X));
                            BKE
                        };
                        let BKG = BKC + BKF;
                        let BKJ = if BKD != 0.0 {
                            let BKH = (BKB * AQ).sqrt();
                            BKH
                        } else {
                            let BKI = (BKB * AQ).powf(X);
                            BKI
                        };
                        let BKK = AH * BKJ;
                        let BKL = CP * ((BDM - E) * BKK);
                        let BKM = OY * (BKL * BKG);
                        BKN = BKK;
                        BKP = BKB;
                        BLC = BKG;
                        BMB = BKL;
                        BND = BKM;
                    }
                    let BNE;
                    if BJZ != 0.0 {
                        BNE = A;
                    } else {
                        let BKR = DN * ((BKN * Y) / BKP);
                        let BKS = (IN * DI) / BKR;
                        let BKT = BKS * BKS;
                        let BKU = BKT * BKT;
                        let BKV = (BKU / (BKU + E)).sqrt();
                        let BKW = BKV.sqrt();
                        let BKX = BKV * BKW;
                        let BKY = (-X) * AB;
                        let BKZ = if BKY == -1e0f64 { 1.0 } else { 0.0 };
                        let BLE = if BKZ != 0.0 {
                            let BLA = E / (E + (BKR * BKX));
                            BLA
                        } else {
                            let BLB = (E + (BKR * BKX)).powf(BKY);
                            BLB
                        };
                        let BLF = (BLC * BLE) / (BLC + BLE);
                        let BLG = (JB * (BKR / BKW)).sqrt();
                        let BLH = (((DI * BKS) * BKW) - (DI * BKV)) + (C * (BKR * BKX));
                        let BLI = (((AT * (BKS * BKW)) - BKV) - E) * BLG;
                        let BLJ = BLI * BLI;
                        let BLK = if BLI > A { 1.0 } else { 0.0 };
                        let BLR = if BLK != 0.0 {
                            let BLL = E / (E + (AS * BLI));
                            BLL
                        } else {
                            let BLM = E / (E - (AS * BLI));
                            BLM
                        };
                        let BLN = (-BLJ) + BLH;
                        let BLO = if BLN > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                        let BLT = if BLO != 0.0 {
                            let BLP = BLN.exp();
                            BLP
                        } else {
                            let BLQ = FE / (E + ((-2.3025850929940458e2f64 - BLN) * (E + (C * ((-2.3025850929940458e2f64 - BLN) * (E + ((-2.3025850929940458e2f64 - BLN) * FF)))))));
                            BLQ
                        };
                        let BLS = BLR * BLR;
                        let BLU = (((AR * BLR) + (AV * BLS)) + (AW * (BLS * BLR))) * BLT;
                        let BMA;
                        if BLK != 0.0 {
                            BMA = BLU;
                        } else {
                            let BLV = if BLH > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            let BLY = if BLV != 0.0 {
                                let BLW = BLH.exp();
                                BLW
                            } else {
                                let BLX = FE / (E + ((-2.3025850929940458e2f64 - BLH) * (E + (C * ((-2.3025850929940458e2f64 - BLH) * (E + ((-2.3025850929940458e2f64 - BLH) * FF)))))));
                                BLX
                            };
                            let BLZ = (AT * BLY) - BLU;
                            BMA = BLZ;
                        }
                        let BMD = OZ * ((BMB * (8.86226925452758e-1f64 * ((DI * BMA) / BLG))) * BLF);
                        BNE = BMD;
                    }
                    let BME = if RF == A { 1.0 } else { 0.0 };
                    let BNF;
                    if BME != 0.0 {
                        BNF = A;
                    } else {
                        let BMF = if X == C { 1.0 } else { 0.0 };
                        let BMI = if BMF != 0.0 {
                            let BMG = ((AP - BFI) * AQ).sqrt();
                            BMG
                        } else {
                            let BMH = ((AP - BFI) * AQ).powf(X);
                            BMH
                        };
                        let BMJ = AB * (((AP - BFI) * AK) / BMI);
                        let BMK = (-DX) / BMJ;
                        let BML = if (BMK.abs()) < FA { 1.0 } else { 0.0 };
                        let BMR;
                        if BML != 0.0 {
                            let BMM = BMK.exp();
                            BMR = BMM;
                        } else {
                            let BMN = if BMK < A { 1.0 } else { 0.0 };
                            let BMS = if BMN != 0.0 {
                                let BMO = FE / (E + ((-2.3025850929940458e2f64 - BMK) * (E + (C * ((-2.3025850929940458e2f64 - BMK) * (E + ((-2.3025850929940458e2f64 - BMK) * FF)))))));
                                BMO
                            } else {
                                let BMP = BMK - FA;
                                let BMQ = FH * (E + (BMP * (E + (C * (BMP * (E + (BMP * FF)))))));
                                BMQ
                            };
                            BMR = BMS;
                        }
                        let BMT = RF * (((GI * BMJ) * BMJ) * BMR);
                        BNF = BMT;
                    }
                    let BMU = if BI > KR { 1.0 } else { 0.0 };
                    let BNG;
                    if BMU != 0.0 {
                        BNG = E;
                    } else {
                        let BMV = if BFY > ((-AX) * BI) { 1.0 } else { 0.0 };
                        let BNH;
                        if BMV != 0.0 {
                            let BMW = if BC == GK { 1.0 } else { 0.0 };
                            let BNA = if BMW != 0.0 {
                                let BMX = BFY * BJ;
                                let BMY = ((BMX * BMX) * BMX) * BMX;
                                BMY
                            } else {
                                let BMZ = ((BFY * BJ).abs()).powf(BC);
                                BMZ
                            };
                            let BNB = E / (E - BNA);
                            BNH = BNB;
                        } else {
                            let BNC = BD + ((BFY + (AX * BI)) * BM);
                            BNH = BNC;
                        }
                        BNG = BNH;
                    }
                    let BNI = (LC * (((BJY + BND) + BNE) + BNF)) * BNG;
                    BNL = BNI;
                }
                let BNM = ((EA * BNJ) + (ED * BNK)) + (EG * BNL);
                let BNN = (EK + EP) + ES;
                let BNO = FX * CH;
                let BNP = BBT - (BNN * ((BNO.exp()) - E));
                let BNQ = BNM - (BNN * (((GI * CH).exp()) - E));
                let BPA;
                let BPC;
                let BQE;
                let BQS;
                let BQZ;
                if GJ != 0.0 {
                    let BNR = if (if BBT > A { 1.0 } else { 0.0 }) != 0.0 && (if BNM > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let BNX;
                    let BNZ;
                    if BNR != 0.0 {
                        let BNT = if (if (if (if (if (BNP / BBT) > BNS { 1.0 } else { 0.0 }) != 0.0 || (if (BNQ / BNM) > BNS { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if BNP > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if BNQ > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if BNQ > BNP { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                        let BNY;
                        let BOA;
                        if BNT != 0.0 {
                            let BNU = (CG * ((BNP / BNQ).ln())) / -1e-1f64;
                            let BNV = BNP / (((BNO * BNU).exp()) - E);
                            BNY = BNV;
                            BOA = BNU;
                        } else {
                            BNY = A;
                            BOA = E;
                        }
                        BNX = BNY;
                        BNZ = BOA;
                    } else {
                        BNX = A;
                        BNZ = E;
                    }
                    let BNW = GF * CH;
                    let BOB = (SO - (BNN * ((BNW.exp()) - E))) - (BNX * (((BNW * BNZ).exp()) - E));
                    let BOC = GG * CH;
                    let BOD = (AEH - (BNN * ((BOC.exp()) - E))) - (BNX * (((BOC * BNZ).exp()) - E));
                    let BOE = GH * CH;
                    let BOF = (AQA - (BNN * ((BOE.exp()) - E))) - (BNX * (((BOE * BNZ).exp()) - E));
                    let BOG = if (if (if SO < A { 1.0 } else { 0.0 }) != 0.0 && (if AEH < A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if AQA < A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let BPD;
                    let BQT;
                    let BRA;
                    if BOG != 0.0 {
                        let BOH = if (if (if (if (if (if (BOB / SO) > BNS { 1.0 } else { 0.0 }) != 0.0 || (if (BOD / AEH) > BNS { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if (BOF / AQA) > BNS { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if BOB < A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if BOD < A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if BOF < A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                        let BPE;
                        let BQU;
                        let BRB;
                        if BOH != 0.0 {
                            let BOI = BOB / BOD;
                            let BOJ = GF - GG;
                            let BOK = GG - GF;
                            let BOL = (((-CG) * (BOI.ln())) / BOJ) + (((CG * (BOI - E)) * ((BOI.powf((GG / BOK))) - E)) / ((((BOI.powf((GF / BOJ))) * BOK) + (BOI * GF)) - GG));
                            let BOM = if ((BOE * BOL).abs()) < 1e-6f64 { 1.0 } else { 0.0 };
                            let BPF;
                            let BQV;
                            let BRC;
                            if BOM != 0.0 {
                                let BON = BOF * ((E / GH) + ((C * CH) * BOL));
                                let BOO = (((-5e-1f64 * BOF) * BOL) * CH) / GH;
                                BPF = BON;
                                BQV = E;
                                BRC = BOO;
                            } else {
                                let BOP = (-BOF) / (((((-GH) * CH) * BOL).exp()) - E);
                                BPF = BOP;
                                BQV = A;
                                BRC = BOL;
                            }
                            BPE = BPF;
                            BQU = BQV;
                            BRB = BRC;
                        } else {
                            BPE = A;
                            BQU = A;
                            BRB = E;
                        }
                        BPD = BPE;
                        BQT = BQU;
                        BRA = BRB;
                    } else {
                        BPD = A;
                        BQT = A;
                        BRA = E;
                    }
                    BPA = BNX;
                    BPC = BPD;
                    BQE = BNZ;
                    BQS = BQT;
                    BQZ = BRA;
                } else {
                    BPA = A;
                    BPC = A;
                    BQE = E;
                    BQS = A;
                    BQZ = E;
                }
                let BOQ = EA * DA;
                let BOR = ED * DB;
                let BOS = EG * DC;
                let BOT = parameters[64] * ((BOQ + BOR) + BOS);
                let BOU = if BOQ <= BOT { 1.0 } else { 0.0 };
                let BRS = if BOU != 0.0 {
                    A
                } else {
                    E
                };
                let BOV = if BOR <= BOT { 1.0 } else { 0.0 };
                let BRW = if BOV != 0.0 {
                    A
                } else {
                    E
                };
                let BOW = if BOS <= BOT { 1.0 } else { 0.0 };
                let BSA = if BOW != 0.0 {
                    A
                } else {
                    E
                };
                let BPH;
                let BPK;
                let BPN;
                if GJ != 0.0 {
                    let BOX = C * EM;
                    let BOZ = (BOX / (BNN + BOY)).ln();
                    let BPB = (BOX / (BPA + BOY)).ln();
                    let BPG = (BOX / ((BPC.abs()) + BOY)).ln();
                    BPH = BOZ;
                    BPK = BPB;
                    BPN = BPG;
                } else {
                    BPH = A;
                    BPK = A;
                    BPN = A;
                }
                let BPI = if BPH <= FA { BPH } else { FA };
                let BPJ = BPI.exp();
                let BPL = if BPK <= FA { BPK } else { FA };
                let BPM = BPL.exp();
                let BPO = if BPN <= FA { BPN } else { FA };
                let BPP = BPO.exp();
                BPU = BPI;
                BPW = BPJ;
                BQB = BNN;
                BQD = BQE;
                BQI = BPL;
                BQK = BPM;
                BQP = BPA;
                BQR = BQS;
                BQX = BPC;
                BQY = BQZ;
                BRH = BPO;
                BRJ = BPP;
                BRR = BRS;
                BRV = BRW;
                BRZ = BSA;
            } else {
                BPU = A;
                BPW = A;
                BQB = A;
                BQD = E;
                BQI = A;
                BQK = A;
                BQP = A;
                BQR = A;
                BQX = A;
                BQY = E;
                BRH = A;
                BRJ = A;
                BRR = E;
                BRV = E;
                BRZ = E;
            }
            let BPQ = parameters[1] * (node_potentials[0] - node_potentials[1]);
            let CEF;
            if GD != 0.0 {
                let BPR = BPQ * CH;
                let BPS = if BPR < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                let BQA;
                if BPS != 0.0 {
                    let BPT = FE / ((-2.3025850929940458e2f64 - BPR) + E);
                    BQA = BPT;
                } else {
                    let BPV = if BPR > BPU { 1.0 } else { 0.0 };
                    let BPZ = if BPV != 0.0 {
                        let BPX = BPW * ((BPR - BPU) + E);
                        BPX
                    } else {
                        let BPY = BPR.exp();
                        BPY
                    };
                    BQA = BPZ;
                }
                let BQC = BQB * (BQA - E);
                let BQF = BPR * BQD;
                let BQG = if BQF < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                let BQO;
                if BQG != 0.0 {
                    let BQH = FE / ((-2.3025850929940458e2f64 - BQF) + E);
                    BQO = BQH;
                } else {
                    let BQJ = if BQF > BQI { 1.0 } else { 0.0 };
                    let BQN = if BQJ != 0.0 {
                        let BQL = BQK * ((BQF - BQI) + E);
                        BQL
                    } else {
                        let BQM = BQF.exp();
                        BQM
                    };
                    BQO = BQN;
                }
                let BQQ = BQP * (BQO - E);
                let BQW = if BQR > A { 1.0 } else { 0.0 };
                let BRP;
                if BQW != 0.0 {
                    let BRD = BPQ * (BQX + (BPQ * BQY));
                    BRP = BRD;
                } else {
                    let BRE = ((-BPQ) * CH) * BQY;
                    let BRF = if BRE < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                    let BRN;
                    if BRF != 0.0 {
                        let BRG = FE / ((-2.3025850929940458e2f64 - BRE) + E);
                        BRN = BRG;
                    } else {
                        let BRI = if BRE > BRH { 1.0 } else { 0.0 };
                        let BRM = if BRI != 0.0 {
                            let BRK = BRJ * ((BRE - BRH) + E);
                            BRK
                        } else {
                            let BRL = BRE.exp();
                            BRL
                        };
                        BRN = BRM;
                    }
                    let BRO = (-BQX) * (BRN - E);
                    BRP = BRO;
                }
                let BRQ = (BQC + BQQ) + BRP;
                let BRT = if BRR > C { 1.0 } else { 0.0 };
                if BRT != 0.0 {
                    let BRU = if U == C { 1.0 } else { 0.0 };
                    if BRU != 0.0 {
                    } else {
                    }
                } else {
                }
                let BRX = if BRV > C { 1.0 } else { 0.0 };
                if BRX != 0.0 {
                    let BRY = if W == C { 1.0 } else { 0.0 };
                    if BRY != 0.0 {
                    } else {
                    }
                } else {
                }
                let BSB = if BRZ > C { 1.0 } else { 0.0 };
                if BSB != 0.0 {
                    let BSC = if Y == C { 1.0 } else { 0.0 };
                    if BSC != 0.0 {
                    } else {
                    }
                } else {
                }
                CEF = BRQ;
            } else {
                let BSD = if (if (if FK != 0.0 && FN != 0.0 { 1.0 } else { 0.0 }) != 0.0 && FQ != 0.0 { 1.0 } else { 0.0 }) == 0.0 { 1.0 } else { 0.0 };
                let BTG;
                let BTK;
                let BTM;
                let BTW;
                let BVO;
                let BWE;
                if BSD != 0.0 {
                    let BSE = if BPQ < EY { 1.0 } else { 0.0 };
                    let BSS;
                    let BSV;
                    let BSX;
                    if BSE != 0.0 {
                        let BSF = BPQ * CH;
                        let BSG = if ((-5e-1f64 * BSF).abs()) < FA { 1.0 } else { 0.0 };
                        let BSL;
                        if BSG != 0.0 {
                            let BSH = (-5e-1f64 * BSF).exp();
                            BSL = BSH;
                        } else {
                            let BSI = if (-5e-1f64 * BSF) < A { 1.0 } else { 0.0 };
                            let BSM = if BSI != 0.0 {
                                let BSJ = FE / (E + ((-2.3025850929940458e2f64 - (-5e-1f64 * BSF)) * (E + (C * ((-2.3025850929940458e2f64 - (-5e-1f64 * BSF)) * (E + ((-2.3025850929940458e2f64 - (-5e-1f64 * BSF)) * FF)))))));
                                BSJ
                            } else {
                                let BSK = FH * (E + (((-5e-1f64 * BSF) - FA) * (E + (C * (((-5e-1f64 * BSF) - FA) * (E + (((-5e-1f64 * BSF) - FA) * FF)))))));
                                BSK
                            };
                            BSL = BSM;
                        }
                        let BSN = E / BSL;
                        let BSO = BSN * BSN;
                        BSS = BSO;
                        BSV = BSL;
                        BSX = BSN;
                    } else {
                        let BSP = (E + ((BPQ - EY) * CH)) * GW;
                        let BSQ = BSP.sqrt();
                        let BSR = E / BSQ;
                        BSS = BSP;
                        BSV = BSR;
                        BSX = BSQ;
                    }
                    let BST = BSS - E;
                    let BSU = if BPQ > A { 1.0 } else { 0.0 };
                    let BSZ = if BSU != 0.0 {
                        let BSW = AT * (CG * (((AT + BSV) + (((BSV + E) * (BSV + AU)).sqrt())).ln()));
                        BSW
                    } else {
                        let BSY = (-BPQ) + (AT * (CG * ((((AT * BSX) + E) + (((E + BSX) * (E + (AU * BSX))).sqrt())).ln())));
                        BSY
                    };
                    let BTA = FW - BSZ;
                    let BTB = BPQ - BTA;
                    let BTC = C * ((BPQ + BTA) - (((BTB * BTB) + ((GK * CG) * CG)).sqrt()));
                    let BTD = BPQ - GB;
                    let BTE = C * ((BPQ + GB) - (((BTD * BTD) + ((GK * I) * I)).sqrt()));
                    let BTF = C * (BPQ - (((BPQ * BPQ) + 4e-12f64).sqrt()));
                    BTG = BST;
                    BTK = BTC;
                    BTM = BSZ;
                    BTW = BSX;
                    BVO = BTE;
                    BWE = BTF;
                } else {
                    BTG = A;
                    BTK = A;
                    BTM = A;
                    BTW = A;
                    BVO = A;
                    BWE = A;
                }
                let BXK;
                let BXM;
                let BXZ;
                let BYY;
                let CEB;
                if FK != 0.0 {
                    BXK = A;
                    BXM = A;
                    BXZ = A;
                    BYY = A;
                    CEB = A;
                } else {
                    let BTH = CQ * BTG;
                    let BTI = if HS == A { 1.0 } else { 0.0 };
                    let BTJ = if (if HR == A { 1.0 } else { 0.0 }) != 0.0 && BTI != 0.0 { 1.0 } else { 0.0 };
                    let BTZ;
                    let BUA;
                    let BUM;
                    let BVK;
                    let BWN;
                    if BTJ != 0.0 {
                        BTZ = A;
                        BUA = A;
                        BUM = A;
                        BVK = A;
                        BWN = A;
                    } else {
                        let BTL = CX - BTK;
                        let BTN = E - ((E - (BTM / BTL)).sqrt());
                        let BTO = if T == C { 1.0 } else { 0.0 };
                        let BTQ = if BTO != 0.0 {
                            A
                        } else {
                            let BTP = ((((BTN * BTN) * (BTN.ln())) / (E - BTN)) + BTN) * (E - (AT * T));
                            BTP
                        };
                        let BTR = BTN + BTQ;
                        let BTU = if BTO != 0.0 {
                            let BTS = (BTL * AM).sqrt();
                            BTS
                        } else {
                            let BTT = (BTL * AM).powf(T);
                            BTT
                        };
                        let BTV = AD * BTU;
                        let BTX = CN * ((BTW - E) * BTV);
                        let BTY = HR * (BTX * BTR);
                        BTZ = BTV;
                        BUA = BTL;
                        BUM = BTR;
                        BVK = BTX;
                        BWN = BTY;
                    }
                    let BWO;
                    if BTI != 0.0 {
                        BWO = A;
                    } else {
                        let BUB = DL * ((BTZ * U) / BUA);
                        let BUC = (IN * DG) / BUB;
                        let BUD = BUC * BUC;
                        let BUE = BUD * BUD;
                        let BUF = (BUE / (BUE + E)).sqrt();
                        let BUG = BUF.sqrt();
                        let BUH = BUF * BUG;
                        let BUI = (-T) * Z;
                        let BUJ = if BUI == -1e0f64 { 1.0 } else { 0.0 };
                        let BUN = if BUJ != 0.0 {
                            let BUK = E / (E + (BUB * BUH));
                            BUK
                        } else {
                            let BUL = (E + (BUB * BUH)).powf(BUI);
                            BUL
                        };
                        let BUO = (BUM * BUN) / (BUM + BUN);
                        let BUP = (JB * (BUB / BUG)).sqrt();
                        let BUQ = (((DG * BUC) * BUG) - (DG * BUF)) + (C * (BUB * BUH));
                        let BUR = (((AT * (BUC * BUG)) - BUF) - E) * BUP;
                        let BUS = BUR * BUR;
                        let BUT = if BUR > A { 1.0 } else { 0.0 };
                        let BVA = if BUT != 0.0 {
                            let BUU = E / (E + (AS * BUR));
                            BUU
                        } else {
                            let BUV = E / (E - (AS * BUR));
                            BUV
                        };
                        let BUW = (-BUS) + BUQ;
                        let BUX = if BUW > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                        let BVC = if BUX != 0.0 {
                            let BUY = BUW.exp();
                            BUY
                        } else {
                            let BUZ = FE / (E + ((-2.3025850929940458e2f64 - BUW) * (E + (C * ((-2.3025850929940458e2f64 - BUW) * (E + ((-2.3025850929940458e2f64 - BUW) * FF)))))));
                            BUZ
                        };
                        let BVB = BVA * BVA;
                        let BVD = (((AR * BVA) + (AV * BVB)) + (AW * (BVB * BVA))) * BVC;
                        let BVJ;
                        if BUT != 0.0 {
                            BVJ = BVD;
                        } else {
                            let BVE = if BUQ > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            let BVH = if BVE != 0.0 {
                                let BVF = BUQ.exp();
                                BVF
                            } else {
                                let BVG = FE / (E + ((-2.3025850929940458e2f64 - BUQ) * (E + (C * ((-2.3025850929940458e2f64 - BUQ) * (E + ((-2.3025850929940458e2f64 - BUQ) * FF)))))));
                                BVG
                            };
                            let BVI = (AT * BVH) - BVD;
                            BVJ = BVI;
                        }
                        let BVL = HS * ((BVK * (8.86226925452758e-1f64 * ((DG * BVJ) / BUP))) * BUO);
                        BWO = BVL;
                    }
                    let BVM = if JZ == A { 1.0 } else { 0.0 };
                    let BWP;
                    if BVM != 0.0 {
                        BWP = A;
                    } else {
                        let BVN = if T == C { 1.0 } else { 0.0 };
                        let BVR = if BVN != 0.0 {
                            let BVP = ((AL - BVO) * AM).sqrt();
                            BVP
                        } else {
                            let BVQ = ((AL - BVO) * AM).powf(T);
                            BVQ
                        };
                        let BVS = Z * (((AL - BVO) * AI) / BVR);
                        let BVT = (-DT) / BVS;
                        let BVU = if (BVT.abs()) < FA { 1.0 } else { 0.0 };
                        let BWA;
                        if BVU != 0.0 {
                            let BVV = BVT.exp();
                            BWA = BVV;
                        } else {
                            let BVW = if BVT < A { 1.0 } else { 0.0 };
                            let BWB = if BVW != 0.0 {
                                let BVX = FE / (E + ((-2.3025850929940458e2f64 - BVT) * (E + (C * ((-2.3025850929940458e2f64 - BVT) * (E + ((-2.3025850929940458e2f64 - BVT) * FF)))))));
                                BVX
                            } else {
                                let BVY = BVT - FA;
                                let BVZ = FH * (E + (BVY * (E + (C * (BVY * (E + (BVY * FF)))))));
                                BVZ
                            };
                            BWA = BWB;
                        }
                        let BWC = JZ * (((BPQ * BVS) * BVS) * BWA);
                        BWP = BWC;
                    }
                    let BWD = if BE > KR { 1.0 } else { 0.0 };
                    let BWQ;
                    if BWD != 0.0 {
                        BWQ = E;
                    } else {
                        let BWF = if BWE > ((-AX) * BE) { 1.0 } else { 0.0 };
                        let BWR;
                        if BWF != 0.0 {
                            let BWG = if AY == GK { 1.0 } else { 0.0 };
                            let BWK = if BWG != 0.0 {
                                let BWH = BWE * BF;
                                let BWI = ((BWH * BWH) * BWH) * BWH;
                                BWI
                            } else {
                                let BWJ = ((BWE * BF).abs()).powf(AY);
                                BWJ
                            };
                            let BWL = E / (E - BWK);
                            BWR = BWL;
                        } else {
                            let BWM = AZ + ((BWE + (AX * BE)) * BK);
                            BWR = BWM;
                        }
                        BWQ = BWR;
                    }
                    let BWS = (LC * (((BTH + BWN) + BWO) + BWP)) * BWQ;
                    let BWT = if U == C { 1.0 } else { 0.0 };
                    if BWT != 0.0 {
                    } else {
                    }
                    BXK = BTZ;
                    BXM = BUA;
                    BXZ = BUM;
                    BYY = BVK;
                    CEB = BWS;
                }
                let CAW;
                let CAY;
                let CBL;
                let CCK;
                let CEC;
                if FN != 0.0 {
                    CAW = BXK;
                    CAY = BXM;
                    CBL = BXZ;
                    CCK = BYY;
                    CEC = A;
                } else {
                    let BWU = CR * BTG;
                    let BWV = if LL == A { 1.0 } else { 0.0 };
                    let BWW = if (if LK == A { 1.0 } else { 0.0 }) != 0.0 && BWV != 0.0 { 1.0 } else { 0.0 };
                    let BXJ;
                    let BXL;
                    let BXY;
                    let BYX;
                    let BZZ;
                    if BWW != 0.0 {
                        BXJ = BXK;
                        BXL = BXM;
                        BXY = BXZ;
                        BYX = BYY;
                        BZZ = A;
                    } else {
                        let BWX = CY - BTK;
                        let BWY = E - ((E - (BTM / BWX)).sqrt());
                        let BWZ = if V == C { 1.0 } else { 0.0 };
                        let BXB = if BWZ != 0.0 {
                            A
                        } else {
                            let BXA = ((((BWY * BWY) * (BWY.ln())) / (E - BWY)) + BWY) * (E - (AT * V));
                            BXA
                        };
                        let BXC = BWY + BXB;
                        let BXF = if BWZ != 0.0 {
                            let BXD = (BWX * AO).sqrt();
                            BXD
                        } else {
                            let BXE = (BWX * AO).powf(V);
                            BXE
                        };
                        let BXG = AF * BXF;
                        let BXH = CO * ((BTW - E) * BXG);
                        let BXI = LK * (BXH * BXC);
                        BXJ = BXG;
                        BXL = BWX;
                        BXY = BXC;
                        BYX = BXH;
                        BZZ = BXI;
                    }
                    let CAA;
                    if BWV != 0.0 {
                        CAA = A;
                    } else {
                        let BXN = DM * ((BXJ * W) / BXL);
                        let BXO = (IN * DH) / BXN;
                        let BXP = BXO * BXO;
                        let BXQ = BXP * BXP;
                        let BXR = (BXQ / (BXQ + E)).sqrt();
                        let BXS = BXR.sqrt();
                        let BXT = BXR * BXS;
                        let BXU = (-V) * AA;
                        let BXV = if BXU == -1e0f64 { 1.0 } else { 0.0 };
                        let BYA = if BXV != 0.0 {
                            let BXW = E / (E + (BXN * BXT));
                            BXW
                        } else {
                            let BXX = (E + (BXN * BXT)).powf(BXU);
                            BXX
                        };
                        let BYB = (BXY * BYA) / (BXY + BYA);
                        let BYC = (JB * (BXN / BXS)).sqrt();
                        let BYD = (((DH * BXO) * BXS) - (DH * BXR)) + (C * (BXN * BXT));
                        let BYE = (((AT * (BXO * BXS)) - BXR) - E) * BYC;
                        let BYF = BYE * BYE;
                        let BYG = if BYE > A { 1.0 } else { 0.0 };
                        let BYN = if BYG != 0.0 {
                            let BYH = E / (E + (AS * BYE));
                            BYH
                        } else {
                            let BYI = E / (E - (AS * BYE));
                            BYI
                        };
                        let BYJ = (-BYF) + BYD;
                        let BYK = if BYJ > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                        let BYP = if BYK != 0.0 {
                            let BYL = BYJ.exp();
                            BYL
                        } else {
                            let BYM = FE / (E + ((-2.3025850929940458e2f64 - BYJ) * (E + (C * ((-2.3025850929940458e2f64 - BYJ) * (E + ((-2.3025850929940458e2f64 - BYJ) * FF)))))));
                            BYM
                        };
                        let BYO = BYN * BYN;
                        let BYQ = (((AR * BYN) + (AV * BYO)) + (AW * (BYO * BYN))) * BYP;
                        let BYW;
                        if BYG != 0.0 {
                            BYW = BYQ;
                        } else {
                            let BYR = if BYD > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            let BYU = if BYR != 0.0 {
                                let BYS = BYD.exp();
                                BYS
                            } else {
                                let BYT = FE / (E + ((-2.3025850929940458e2f64 - BYD) * (E + (C * ((-2.3025850929940458e2f64 - BYD) * (E + ((-2.3025850929940458e2f64 - BYD) * FF)))))));
                                BYT
                            };
                            let BYV = (AT * BYU) - BYQ;
                            BYW = BYV;
                        }
                        let BYZ = LL * ((BYX * (8.86226925452758e-1f64 * ((DH * BYW) / BYC))) * BYB);
                        CAA = BYZ;
                    }
                    let BZA = if NR == A { 1.0 } else { 0.0 };
                    let CAB;
                    if BZA != 0.0 {
                        CAB = A;
                    } else {
                        let BZB = if V == C { 1.0 } else { 0.0 };
                        let BZE = if BZB != 0.0 {
                            let BZC = ((AN - BVO) * AO).sqrt();
                            BZC
                        } else {
                            let BZD = ((AN - BVO) * AO).powf(V);
                            BZD
                        };
                        let BZF = AA * (((AN - BVO) * AJ) / BZE);
                        let BZG = (-DV) / BZF;
                        let BZH = if (BZG.abs()) < FA { 1.0 } else { 0.0 };
                        let BZN;
                        if BZH != 0.0 {
                            let BZI = BZG.exp();
                            BZN = BZI;
                        } else {
                            let BZJ = if BZG < A { 1.0 } else { 0.0 };
                            let BZO = if BZJ != 0.0 {
                                let BZK = FE / (E + ((-2.3025850929940458e2f64 - BZG) * (E + (C * ((-2.3025850929940458e2f64 - BZG) * (E + ((-2.3025850929940458e2f64 - BZG) * FF)))))));
                                BZK
                            } else {
                                let BZL = BZG - FA;
                                let BZM = FH * (E + (BZL * (E + (C * (BZL * (E + (BZL * FF)))))));
                                BZM
                            };
                            BZN = BZO;
                        }
                        let BZP = NR * (((BPQ * BZF) * BZF) * BZN);
                        CAB = BZP;
                    }
                    let BZQ = if BG > KR { 1.0 } else { 0.0 };
                    let CAC;
                    if BZQ != 0.0 {
                        CAC = E;
                    } else {
                        let BZR = if BWE > ((-AX) * BG) { 1.0 } else { 0.0 };
                        let CAD;
                        if BZR != 0.0 {
                            let BZS = if BA == GK { 1.0 } else { 0.0 };
                            let BZW = if BZS != 0.0 {
                                let BZT = BWE * BH;
                                let BZU = ((BZT * BZT) * BZT) * BZT;
                                BZU
                            } else {
                                let BZV = ((BWE * BH).abs()).powf(BA);
                                BZV
                            };
                            let BZX = E / (E - BZW);
                            CAD = BZX;
                        } else {
                            let BZY = BB + ((BWE + (AX * BG)) * BL);
                            CAD = BZY;
                        }
                        CAC = CAD;
                    }
                    let CAE = (LC * (((BWU + BZZ) + CAA) + CAB)) * CAC;
                    let CAF = if W == C { 1.0 } else { 0.0 };
                    if CAF != 0.0 {
                    } else {
                    }
                    CAW = BXJ;
                    CAY = BXL;
                    CBL = BXY;
                    CCK = BYX;
                    CEC = CAE;
                }
                let CED;
                if FQ != 0.0 {
                    CED = A;
                } else {
                    let CAG = CS * BTG;
                    let CAH = if OZ == A { 1.0 } else { 0.0 };
                    let CAI = if (if OY == A { 1.0 } else { 0.0 }) != 0.0 && CAH != 0.0 { 1.0 } else { 0.0 };
                    let CAV;
                    let CAX;
                    let CBK;
                    let CCJ;
                    let CDL;
                    if CAI != 0.0 {
                        CAV = CAW;
                        CAX = CAY;
                        CBK = CBL;
                        CCJ = CCK;
                        CDL = A;
                    } else {
                        let CAJ = CZ - BTK;
                        let CAK = E - ((E - (BTM / CAJ)).sqrt());
                        let CAL = if X == C { 1.0 } else { 0.0 };
                        let CAN = if CAL != 0.0 {
                            A
                        } else {
                            let CAM = ((((CAK * CAK) * (CAK.ln())) / (E - CAK)) + CAK) * (E - (AT * X));
                            CAM
                        };
                        let CAO = CAK + CAN;
                        let CAR = if CAL != 0.0 {
                            let CAP = (CAJ * AQ).sqrt();
                            CAP
                        } else {
                            let CAQ = (CAJ * AQ).powf(X);
                            CAQ
                        };
                        let CAS = AH * CAR;
                        let CAT = CP * ((BTW - E) * CAS);
                        let CAU = OY * (CAT * CAO);
                        CAV = CAS;
                        CAX = CAJ;
                        CBK = CAO;
                        CCJ = CAT;
                        CDL = CAU;
                    }
                    let CDM;
                    if CAH != 0.0 {
                        CDM = A;
                    } else {
                        let CAZ = DN * ((CAV * Y) / CAX);
                        let CBA = (IN * DI) / CAZ;
                        let CBB = CBA * CBA;
                        let CBC = CBB * CBB;
                        let CBD = (CBC / (CBC + E)).sqrt();
                        let CBE = CBD.sqrt();
                        let CBF = CBD * CBE;
                        let CBG = (-X) * AB;
                        let CBH = if CBG == -1e0f64 { 1.0 } else { 0.0 };
                        let CBM = if CBH != 0.0 {
                            let CBI = E / (E + (CAZ * CBF));
                            CBI
                        } else {
                            let CBJ = (E + (CAZ * CBF)).powf(CBG);
                            CBJ
                        };
                        let CBN = (CBK * CBM) / (CBK + CBM);
                        let CBO = (JB * (CAZ / CBE)).sqrt();
                        let CBP = (((DI * CBA) * CBE) - (DI * CBD)) + (C * (CAZ * CBF));
                        let CBQ = (((AT * (CBA * CBE)) - CBD) - E) * CBO;
                        let CBR = CBQ * CBQ;
                        let CBS = if CBQ > A { 1.0 } else { 0.0 };
                        let CBZ = if CBS != 0.0 {
                            let CBT = E / (E + (AS * CBQ));
                            CBT
                        } else {
                            let CBU = E / (E - (AS * CBQ));
                            CBU
                        };
                        let CBV = (-CBR) + CBP;
                        let CBW = if CBV > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                        let CCB = if CBW != 0.0 {
                            let CBX = CBV.exp();
                            CBX
                        } else {
                            let CBY = FE / (E + ((-2.3025850929940458e2f64 - CBV) * (E + (C * ((-2.3025850929940458e2f64 - CBV) * (E + ((-2.3025850929940458e2f64 - CBV) * FF)))))));
                            CBY
                        };
                        let CCA = CBZ * CBZ;
                        let CCC = (((AR * CBZ) + (AV * CCA)) + (AW * (CCA * CBZ))) * CCB;
                        let CCI;
                        if CBS != 0.0 {
                            CCI = CCC;
                        } else {
                            let CCD = if CBP > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            let CCG = if CCD != 0.0 {
                                let CCE = CBP.exp();
                                CCE
                            } else {
                                let CCF = FE / (E + ((-2.3025850929940458e2f64 - CBP) * (E + (C * ((-2.3025850929940458e2f64 - CBP) * (E + ((-2.3025850929940458e2f64 - CBP) * FF)))))));
                                CCF
                            };
                            let CCH = (AT * CCG) - CCC;
                            CCI = CCH;
                        }
                        let CCL = OZ * ((CCJ * (8.86226925452758e-1f64 * ((DI * CCI) / CBO))) * CBN);
                        CDM = CCL;
                    }
                    let CCM = if RF == A { 1.0 } else { 0.0 };
                    let CDN;
                    if CCM != 0.0 {
                        CDN = A;
                    } else {
                        let CCN = if X == C { 1.0 } else { 0.0 };
                        let CCQ = if CCN != 0.0 {
                            let CCO = ((AP - BVO) * AQ).sqrt();
                            CCO
                        } else {
                            let CCP = ((AP - BVO) * AQ).powf(X);
                            CCP
                        };
                        let CCR = AB * (((AP - BVO) * AK) / CCQ);
                        let CCS = (-DX) / CCR;
                        let CCT = if (CCS.abs()) < FA { 1.0 } else { 0.0 };
                        let CCZ;
                        if CCT != 0.0 {
                            let CCU = CCS.exp();
                            CCZ = CCU;
                        } else {
                            let CCV = if CCS < A { 1.0 } else { 0.0 };
                            let CDA = if CCV != 0.0 {
                                let CCW = FE / (E + ((-2.3025850929940458e2f64 - CCS) * (E + (C * ((-2.3025850929940458e2f64 - CCS) * (E + ((-2.3025850929940458e2f64 - CCS) * FF)))))));
                                CCW
                            } else {
                                let CCX = CCS - FA;
                                let CCY = FH * (E + (CCX * (E + (C * (CCX * (E + (CCX * FF)))))));
                                CCY
                            };
                            CCZ = CDA;
                        }
                        let CDB = RF * (((BPQ * CCR) * CCR) * CCZ);
                        CDN = CDB;
                    }
                    let CDC = if BI > KR { 1.0 } else { 0.0 };
                    let CDO;
                    if CDC != 0.0 {
                        CDO = E;
                    } else {
                        let CDD = if BWE > ((-AX) * BI) { 1.0 } else { 0.0 };
                        let CDP;
                        if CDD != 0.0 {
                            let CDE = if BC == GK { 1.0 } else { 0.0 };
                            let CDI = if CDE != 0.0 {
                                let CDF = BWE * BJ;
                                let CDG = ((CDF * CDF) * CDF) * CDF;
                                CDG
                            } else {
                                let CDH = ((BWE * BJ).abs()).powf(BC);
                                CDH
                            };
                            let CDJ = E / (E - CDI);
                            CDP = CDJ;
                        } else {
                            let CDK = BD + ((BWE + (AX * BI)) * BM);
                            CDP = CDK;
                        }
                        CDO = CDP;
                    }
                    let CDQ = (LC * (((CAG + CDL) + CDM) + CDN)) * CDO;
                    if BS != 0.0 {
                        let CDS = if BPQ < CDR { 1.0 } else { 0.0 };
                        if CDS != 0.0 {
                            let CDU = if ((BPQ - CDR) / CDT) < -3.7e1f64 { 1.0 } else { 0.0 };
                            if CDU != 0.0 {
                            } else {
                            }
                        } else {
                            let CDV = if ((BPQ - CDR) / CDT) > 3.7e1f64 { 1.0 } else { 0.0 };
                            if CDV != 0.0 {
                            } else {
                            }
                        }
                        let CDW = if Y == C { 1.0 } else { 0.0 };
                        if CDW != 0.0 {
                        } else {
                        }
                        let CDZ = if CDX == C { 1.0 } else { 0.0 };
                        if CDZ != 0.0 {
                        } else {
                        }
                    } else {
                        let CEA = if Y == C { 1.0 } else { 0.0 };
                        if CEA != 0.0 {
                        } else {
                        }
                    }
                    CED = CDQ;
                }
                let CEE = ((EA * CEB) + (ED * CEC)) + (EG * CED);
                CEF = CEE;
            }
            let CEG = (EJ * parameters[7]) * (3.2043836e-19f64 * (CEF.abs()));
            if GD != 0.0 {
            } else {
            }
        {
            let psd = CEG;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 0, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 0, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(0, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        Ok(())
    }
}
