#![allow(dead_code, non_snake_case, unused_parens, unused_variables)]

use super::state::Instance;
use rspice_veriloga_runtime::GeneratedEvalContext;
pub use rspice_veriloga_runtime::{GeneratedNoiseDescriptor, GeneratedNoiseEndpoint, GeneratedNoiseEvaluation, GeneratedNoiseEvaluationError, GeneratedNoiseEvaluationRef, GeneratedNoiseKind, GeneratedNoiseVisitor};

use rspice_veriloga_runtime::rspice_limexp;
pub static NOISE_SOURCES: [GeneratedNoiseDescriptor; 6] = [
    GeneratedNoiseDescriptor { mechanism: "WHITE_B_BI_RB", label: Some("rb"), kind: GeneratedNoiseKind::White, equation: 26, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(1), name: "b", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(6), name: "bi", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_CI_C_RCX", label: Some("rcx"), kind: GeneratedNoiseKind::White, equation: 27, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "ci", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(0), name: "c", is_internal: false }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_EI_E_RE", label: Some("re"), kind: GeneratedNoiseKind::White, equation: 28, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(7), name: "ei", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(2), name: "e", is_internal: false }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_BI_EI_FLICKER", label: Some("flicker"), kind: GeneratedNoiseKind::Flicker, equation: 29, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(6), name: "bi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(7), name: "ei", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_BI_EI_IBE", label: Some("ibe"), kind: GeneratedNoiseKind::White, equation: 30, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(6), name: "bi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(7), name: "ei", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_CI_EI_IT", label: Some("it"), kind: GeneratedNoiseKind::White, equation: 31, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "ci", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(7), name: "ei", is_internal: true }, table_len: 0, table_log_interp: false },
];

impl Instance {
    pub fn evaluate_noise_sources(&self, ctx: &GeneratedEvalContext<'_>, visitor: &mut dyn GeneratedNoiseVisitor) -> Result<(), GeneratedNoiseEvaluationError> {
        if !self.multiplicity.is_finite() || self.multiplicity <= 0.0 {
            return Err(GeneratedNoiseEvaluationError::InvalidMultiplicity { value: self.multiplicity });
        }
        let parameters = &self.params.values;
        let temperature = ctx.temperature();
        let node_potentials = [ctx.node_voltage(self.nodes[0]), ctx.node_voltage(self.nodes[1]), ctx.node_voltage(self.nodes[2]), ctx.node_voltage(self.nodes[3]), ctx.node_voltage(self.nodes[4]), ctx.node_voltage(self.nodes[5]), ctx.node_voltage(self.nodes[6]), ctx.node_voltage(self.nodes[7]), ctx.node_voltage(self.nodes[8]), ctx.node_voltage(self.nodes[9])];
        let branch_unknown_flows = [ctx.branch_current(self.branches[0]), ctx.branch_current(self.branches[1]), ctx.branch_current(self.branches[2]), ctx.branch_current(self.branches[3])];
            let A = 0e0f64;
            let B = parameters[110];
            let C = node_potentials[1];
            let D = node_potentials[5];
            let F = node_potentials[6];
            let N = 1.3806226e-23f64;
            let O = 1.602176462e-19f64;
            let R = 5e-1f64;
            let S = parameters[76];
            let T = parameters[77];
            let V = parameters[78];
            let Y = 3e0f64;
            let AA = 1e0f64;
            let AB = parameters[87];
            let AE = parameters[82];
            let AF = parameters[81];
            let AH = parameters[34];
            let AI = parameters[21];
            let AJ = parameters[41];
            let AM = 1e2f64;
            let AO = 1.7314999999999998e2f64;
            let AQ = 6e2f64;
            let AZ = parameters[35];
            let BB = 2e0f64;
            let BJ = 4e0f64;
            let BL = parameters[36];
            let BN = parameters[37];
            let BP = parameters[38];
            let BU = parameters[39];
            let BW = parameters[40];
            let BY = parameters[15];
            let CA = parameters[17];
            let CE = parameters[42];
            let CK = parameters[43];
            let CM = parameters[19];
            let CP = parameters[1];
            let CR = parameters[9];
            let CS = parameters[95];
            let CT = parameters[83];
            let CV = parameters[62];
            let CY = parameters[61];
            let DA = parameters[64];
            let DB = parameters[89];
            let DD = parameters[65];
            let DF = parameters[90];
            let DH = parameters[54];
            let DI = parameters[85];
            let DJ = parameters[86];
            let DO = parameters[99];
            let DQ = parameters[22];
            let DR = parameters[100];
            let DT = parameters[23];
            let DU = parameters[91];
            let DW = parameters[46];
            let EB = parameters[45];
            let EC = parameters[47];
            let EE = parameters[51];
            let EJ = parameters[50];
            let EK = parameters[52];
            let EM = parameters[30];
            let EO = parameters[7];
            let EP = parameters[97];
            let ER = parameters[6];
            let ES = parameters[84];
            let EV = parameters[101];
            let EW = parameters[102];
            let EY = parameters[98];
            let FA = parameters[12];
            let FD = parameters[13];
            let FF = parameters[29];
            let FG = parameters[93];
            let FI = parameters[26];
            let FJ = parameters[92];
            let FL = parameters[28];
            let FM = parameters[94];
            let FO = parameters[104];
            let FP = parameters[103];
            let FQ = parameters[111];
            let FV = 1.7314999999999998e2f64;
            let FX = 6e2f64;
            let IE = parameters[49];
            let IH = parameters[44];
            let IN = 2.4e0f64;
            let IT = 8e1f64;
            let IX = 1e-1f64;
            let JL = 1.921812e0f64;
            let JQ = parameters[48];
            let NA = parameters[67];
            let PA = parameters[11];
            let PR = 6.666e-1f64;
            let QH = 1e-2f64;
            let QP = 3.333333333333333e-1f64;
            let QR = 1e6f64;
            let QX = 2.7e1f64;
            let SB = 1e-20f64;
            let WD = parameters[53];
            let WU = node_potentials[9];
            let YA = 0e0f64;
            let YG = 3.204352924e-19f64;
            let E = B * (C - D);
            let G = B * (F - D);
            let H = B * (F - node_potentials[7]);
            let I = H - G;
            let J = B * (node_potentials[3] - D);
            let K = B * (C - node_potentials[2]);
            let L = C - F;
            let M = parameters[108] + 2.7315e2f64;
            let P = (N * M) / O;
            let Q = parameters[88] * M;
            let U = R * (S + T);
            let W = R * (S + V);
            let X = R * (parameters[79] + V);
            let Z = Y - ((O * parameters[80]) / N);
            let AC = (Z + AA) - AB;
            let AD = Z - 1.5e0f64;
            let AG = S - T;
            let AK = if (if AI > A { 1.0 } else { 0.0 }) != 0.0 && (if AJ > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let DM = if AK != 0.0 {
                AA
            } else {
                A
            };
            let AL = temperature + parameters[109];
            let AN = if AL < 1.7314999999999998e2f64 { 1.0 } else { 0.0 };
            let AR;
            if AN != 0.0 {
                AR = AO;
            } else {
                let AP = if AL > 6e2f64 { 1.0 } else { 0.0 };
                let AS = if AP != 0.0 {
                    AQ
                } else {
                    AL
                };
                AR = AS;
            }
            let AT = (N * AR) / O;
            let AU = AA / AT;
            let AV = AR - M;
            let AW = AR / M;
            let AX = AW.ln();
            let AY = AU * (AW - AA);
            let BA = (R * AZ) / P;
            let BC = BB * P;
            let BD = BC * (((BA.exp()) - ((-BA).exp())).ln());
            let BE = AA - AW;
            let BF = U * BE;
            let BG = (Z * AT) * AX;
            let BH = ((BD * AW) + BF) - BG;
            let BI = BB * AT;
            let BK = BH + (BI * ((R * (AA + ((AA + (BJ * (((-BH) * AU).exp()))).sqrt()))).ln()));
            let BM = AH * ((BL * ((AZ / BK).ln())).exp());
            let BO = (BN * BK) / AZ;
            let BQ = (R * BP) / P;
            let BR = BC * (((BQ.exp()) - ((-BQ).exp())).ln());
            let BS = ((BR * AW) + BF) - BG;
            let BT = BS + (BI * ((R * (AA + ((AA + (BJ * (((-BS) * AU).exp()))).sqrt()))).ln()));
            let BV = AH * ((BU * ((BP / BT).ln())).exp());
            let BX = (BW * BT) / BP;
            let BZ = BY * (((AE * AX) + (T * AY)).exp());
            let CB = R * Z;
            let CC = R * U;
            let CD = CA * (((CB * AX) + (CC * AY)).exp());
            let CF = (R * CE) / P;
            let CG = BC * (((CF.exp()) - ((-CF).exp())).ln());
            let CH = W * BE;
            let CI = ((CG * AW) + CH) - BG;
            let CJ = CI + (BI * ((R * (AA + ((AA + (BJ * (((-CI) * AU).exp()))).sqrt()))).ln()));
            let CL = AJ * ((CK * ((CE / CJ).ln())).exp());
            let CN = V * AY;
            let CO = CM * (((AC * AX) + CN).exp());
            let CQ = CP * (((AF * AX) + (S * AY)).exp());
            let CU = CR * (((CS * AX) - (CT * AY)).exp());
            let CW = AB - Q;
            let CX = CV * ((CW * AX).exp());
            let CZ = AA / (CY * ((AB * AX).exp()));
            let DC = DA * (AA + (DB * AV));
            let DE = if DD > A { 1.0 } else { 0.0 };
            let MP;
            let MT;
            if DE != 0.0 {
                let DG = DD * (AA - (DF * AV));
                MP = DG;
                MT = DA;
            } else {
                MP = DD;
                MT = DC;
            }
            let DK = DH * ((AA + (DI * AV)) + ((DJ * AV) * AV));
            let DL = if parameters[96] == AA { 1.0 } else { 0.0 };
            if DL != 0.0 {
            } else {
            }
            let DN = if DM == AA { 1.0 } else { 0.0 };
            let UV;
            let VE;
            if DN != 0.0 {
                let DP = AI * ((DO * AV).exp());
                let DS = DQ * ((DR * AV).exp());
                UV = DS;
                VE = DP;
            } else {
                UV = DQ;
                VE = AI;
            }
            let DV = DT * ((DU * AX).exp());
            let DX = (R * DW) / P;
            let DY = BC * (((DX.exp()) - ((-DX).exp())).ln());
            let DZ = ((DY * AW) + CH) - BG;
            let EA = DZ + (BI * ((R * (AA + ((AA + (BJ * (((-DZ) * AU).exp()))).sqrt()))).ln()));
            let ED = EB * ((EC * ((DW / EA).ln())).exp());
            let EF = (R * EE) / P;
            let EG = BC * (((EF.exp()) - ((-EF).exp())).ln());
            let EH = ((EG * AW) + (X * BE)) - BG;
            let EI = EH + (BI * ((R * (AA + ((AA + (BJ * (((-EH) * AU).exp()))).sqrt()))).ln()));
            let EL = EJ * ((EK * ((EE / EI).ln())).exp());
            let EN = EM * (((AD * AX) + CN).exp());
            let EQ = EO * ((EP * AX).exp());
            let ET = ER / (((CT * AU) * (((ES * AX).exp()) - AA)).exp());
            let EU = if parameters[0] <= 2e2f64 { 1.0 } else { 0.0 };
            let FB = if EU != 0.0 {
                let EX = AA + (AV * (EV + (EW * AV)));
                EX
            } else {
                let EZ = (EY * AX).exp();
                EZ
            };
            let FC = FA * FB;
            let FE = (FD * FB) * ((AG * AY).exp());
            let FH = FF * ((FG * AX).exp());
            let FK = FI * ((FJ * AX).exp());
            let FN = FL * ((FM * AX).exp());
            let FR = if FO >= FQ { 1.0 } else { 0.0 };
            let FS = if (if FP != A { 1.0 } else { 0.0 }) != 0.0 && FR != 0.0 { 1.0 } else { 0.0 };
            let IB;
            let ID;
            let IL;
            let IR;
            let IV;
            let JT;
            let MN;
            let MR;
            let MY;
            let MZ;
            let NH;
            let NJ;
            let NK;
            let NS;
            let NU;
            let NV;
            let OH;
            let OQ;
            let OU;
            let OY;
            let PG;
            let PS;
            let PT;
            let SN;
            let SV;
            let TG;
            let UT;
            let VC;
            let VJ;
            let VV;
            let VZ;
            let WF;
            let WH;
            let XQ;
            let XS;
            let XY;
            if FS != 0.0 {
                let FT = AL + node_potentials[4];
                let FU = if FT < 1.7314999999999998e2f64 { 1.0 } else { 0.0 };
                let FY;
                if FU != 0.0 {
                    FY = FV;
                } else {
                    let FW = if FT > 6e2f64 { 1.0 } else { 0.0 };
                    let FZ = if FW != 0.0 {
                        FX
                    } else {
                        FT
                    };
                    FY = FZ;
                }
                let GA = (N * FY) / O;
                let GB = AA / GA;
                let GC = FY - M;
                let GD = FY / M;
                let GE = GD.ln();
                let GF = GB * (GD - AA);
                let GG = AA - GD;
                let GH = U * GG;
                let GI = (Z * GA) * GE;
                let GJ = ((BD * GD) + GH) - GI;
                let GK = BB * GA;
                let GL = GJ + (GK * ((R * (AA + ((AA + (BJ * (((-GJ) * GB).exp()))).sqrt()))).ln()));
                let GM = AH * ((BL * ((AZ / GL).ln())).exp());
                let GN = (BN * GL) / AZ;
                let GO = ((BR * GD) + GH) - GI;
                let GP = GO + (GK * ((R * (AA + ((AA + (BJ * (((-GO) * GB).exp()))).sqrt()))).ln()));
                let GQ = AH * ((BU * ((BP / GP).ln())).exp());
                let GR = (BW * GP) / BP;
                let GS = BY * (((AE * GE) + (T * GF)).exp());
                let GT = CA * (((CB * GE) + (CC * GF)).exp());
                let GU = W * GG;
                let GV = ((CG * GD) + GU) - GI;
                let GW = GV + (GK * ((R * (AA + ((AA + (BJ * (((-GV) * GB).exp()))).sqrt()))).ln()));
                let GX = AJ * ((CK * ((CE / GW).ln())).exp());
                let GY = V * GF;
                let GZ = CM * (((AC * GE) + GY).exp());
                let HA = CP * (((AF * GE) + (S * GF)).exp());
                let HB = CR * (((CS * GE) - (CT * GF)).exp());
                let HC = CV * ((CW * GE).exp());
                let HD = AA / (CY * ((AB * GE).exp()));
                let HE = DA * (AA + (DB * GC));
                let MO;
                let MS;
                if DE != 0.0 {
                    let HF = DD * (AA - (DF * GC));
                    MO = HF;
                    MS = DA;
                } else {
                    MO = DD;
                    MS = HE;
                }
                let HG = DH * ((AA + (DI * GC)) + ((DJ * GC) * GC));
                if DL != 0.0 {
                } else {
                }
                let UU;
                let VD;
                if DN != 0.0 {
                    let HH = AI * ((DO * GC).exp());
                    let HI = DQ * ((DR * GC).exp());
                    UU = HI;
                    VD = HH;
                } else {
                    UU = DQ;
                    VD = AI;
                }
                let HJ = DT * ((DU * GE).exp());
                let HK = ((DY * GD) + GU) - GI;
                let HL = HK + (GK * ((R * (AA + ((AA + (BJ * (((-HK) * GB).exp()))).sqrt()))).ln()));
                let HM = EB * ((EC * ((DW / HL).ln())).exp());
                let HN = ((EG * GD) + (X * GG)) - GI;
                let HO = HN + (GK * ((R * (AA + ((AA + (BJ * (((-HN) * GB).exp()))).sqrt()))).ln()));
                let HP = EJ * ((EK * ((EE / HO).ln())).exp());
                let HQ = EM * (((AD * GE) + GY).exp());
                let HR = EO * ((EP * GE).exp());
                let HS = ER / (((CT * GB) * (((ES * GE).exp()) - AA)).exp());
                let HV = if EU != 0.0 {
                    let HT = AA + (GC * (EV + (EW * GC)));
                    HT
                } else {
                    let HU = (EY * GE).exp();
                    HU
                };
                let HW = FA * HV;
                let HX = (FD * HV) * ((AG * GF).exp());
                let HY = FF * ((FG * GE).exp());
                let HZ = FI * ((FJ * GE).exp());
                let IA = FL * ((FM * GE).exp());
                IB = HM;
                ID = GX;
                IL = GW;
                IR = GB;
                IV = GA;
                JT = HL;
                MN = MO;
                MR = MS;
                MY = HC;
                MZ = HD;
                NH = GM;
                NJ = GL;
                NK = GN;
                NS = GQ;
                NU = GP;
                NV = GR;
                OH = HR;
                OQ = HS;
                OU = HG;
                OY = HB;
                PG = HA;
                PS = HX;
                PT = HW;
                SN = GS;
                SV = GT;
                TG = GZ;
                UT = UU;
                VC = VD;
                VJ = HJ;
                VV = HZ;
                VZ = HQ;
                WF = HP;
                WH = HO;
                XQ = IA;
                XS = HY;
                XY = FY;
            } else {
                IB = ED;
                ID = CL;
                IL = CJ;
                IR = AU;
                IV = AT;
                JT = EA;
                MN = MP;
                MR = MT;
                MY = CX;
                MZ = CZ;
                NH = BM;
                NJ = BK;
                NK = BO;
                NS = BV;
                NU = BT;
                NV = BX;
                OH = EQ;
                OQ = ET;
                OU = DK;
                OY = CU;
                PG = CQ;
                PS = FE;
                PT = FC;
                SN = BZ;
                SV = CD;
                TG = CO;
                UT = UV;
                VC = VE;
                VJ = DV;
                VV = FK;
                VZ = EN;
                WF = EL;
                WH = EI;
                XQ = FN;
                XS = FH;
                XY = AR;
            }
            let IC = if IB <= 1e-30f64 { 1.0 } else { 0.0 };
            let LD;
            let WV;
            if IC != 0.0 {
                let IF = ID * IE;
                let IG = ID * (AA - IE);
                let II = if IH < AM { 1.0 } else { 0.0 };
                let WW;
                if II != 0.0 {
                    let IJ = if IG > A { 1.0 } else { 0.0 };
                    let WX;
                    if IJ != 0.0 {
                        let IK = CK / BJ;
                        let IM = IH - IL;
                        let IO = IL * (AA - ((-8.754687373538999e-1f64 / CK).exp()));
                        let IP = IN * IG;
                        let IQ = IG * (((IK - CK) * ((IH / IL).ln())).exp());
                        let IS = (IO - E) * IR;
                        let IU = if IS < IT { 1.0 } else { 0.0 };
                        let IZ = if IU != 0.0 {
                            let IW = IO - (IV * ((AA + (IS.exp())).ln()));
                            IW
                        } else {
                            E
                        };
                        let IY = (IX * IM) + (BJ * IV);
                        let JA = (IM + IZ) / IY;
                        let JB = if JA < IT { 1.0 } else { 0.0 };
                        let JD = if JB != 0.0 {
                            let JC = (-IM) + (IY * (((AA + (JA.exp())).ln()) - (((-(IM + IO)) / IY).exp())));
                            JC
                        } else {
                            IZ
                        };
                        let JE = (AA - (JD / IL)).ln();
                        let JF = AA - CK;
                        let JG = AA - IK;
                        let JH = (((((IG * (AA - ((JE * JF).exp()))) / JF) + ((IQ * (AA - ((((AA - (IZ / IL)).ln()) * JG).exp()))) / JG)) - ((IQ * (AA - ((JE * JG).exp()))) / JG)) * IL) + (IP * (E - IZ));
                        WX = JH;
                    } else {
                        WX = A;
                    }
                    WW = WX;
                } else {
                    let JI = if IG > A { 1.0 } else { 0.0 };
                    let WY = if JI != 0.0 {
                        let JJ = IL * (AA - ((-8.754687373538999e-1f64 / CK).exp()));
                        let JK = (JJ - E) * IR;
                        let JM = JJ - (IV * ((JK + (((JK * JK) + JL).sqrt())) * R));
                        let JN = AA - CK;
                        let JO = IG * (((IL * (AA - ((((AA - (JM / IL)).ln()) * JN).exp()))) / JN) + (IN * (E - JM)));
                        JO
                    } else {
                        A
                    };
                    WW = WY;
                }
                LD = IF;
                WV = WW;
            } else {
                let JP = IB * IE;
                let JR = if JQ < AM { 1.0 } else { 0.0 };
                if JR != 0.0 {
                    let JS = if JP > A { 1.0 } else { 0.0 };
                    if JS != 0.0 {
                        let JU = JQ - JT;
                        let JV = JT * (AA - ((-8.754687373538999e-1f64 / EC).exp()));
                        let JW = (JV - G) * IR;
                        let JX = if JW < IT { 1.0 } else { 0.0 };
                        let JZ = if JX != 0.0 {
                            let JY = JV - (IV * ((AA + (JW.exp())).ln()));
                            JY
                        } else {
                            G
                        };
                        let KA = if ((JU + JZ) / ((IX * JU) + (BJ * IV))) < IT { 1.0 } else { 0.0 };
                        if KA != 0.0 {
                        } else {
                        }
                    } else {
                    }
                } else {
                    let KB = if JP > A { 1.0 } else { 0.0 };
                    if KB != 0.0 {
                    } else {
                    }
                }
                let KC = IB * (AA - IE);
                let WZ;
                if JR != 0.0 {
                    let KD = if KC > A { 1.0 } else { 0.0 };
                    let XA;
                    if KD != 0.0 {
                        let KE = EC / BJ;
                        let KF = JQ - JT;
                        let KG = JT * (AA - ((-8.754687373538999e-1f64 / EC).exp()));
                        let KH = IN * KC;
                        let KI = KC * (((KE - EC) * ((JQ / JT).ln())).exp());
                        let KJ = (KG - E) * IR;
                        let KK = if KJ < IT { 1.0 } else { 0.0 };
                        let KN = if KK != 0.0 {
                            let KL = KG - (IV * ((AA + (KJ.exp())).ln()));
                            KL
                        } else {
                            E
                        };
                        let KM = (IX * KF) + (BJ * IV);
                        let KO = (KF + KN) / KM;
                        let KP = if KO < IT { 1.0 } else { 0.0 };
                        let KR = if KP != 0.0 {
                            let KQ = (-KF) + (KM * (((AA + (KO.exp())).ln()) - (((-(KF + KG)) / KM).exp())));
                            KQ
                        } else {
                            KN
                        };
                        let KS = (AA - (KR / JT)).ln();
                        let KT = AA - EC;
                        let KU = AA - KE;
                        let KV = (((((KC * (AA - ((KS * KT).exp()))) / KT) + ((KI * (AA - ((((AA - (KN / JT)).ln()) * KU).exp()))) / KU)) - ((KI * (AA - ((KS * KU).exp()))) / KU)) * JT) + (KH * (E - KN));
                        XA = KV;
                    } else {
                        XA = A;
                    }
                    WZ = XA;
                } else {
                    let KW = if KC > A { 1.0 } else { 0.0 };
                    let XB = if KW != 0.0 {
                        let KX = JT * (AA - ((-8.754687373538999e-1f64 / EC).exp()));
                        let KY = (KX - E) * IR;
                        let KZ = KX - (IV * ((KY + (((KY * KY) + JL).sqrt())) * R));
                        let LA = AA - EC;
                        let LB = KC * (((JT * (AA - ((((AA - (KZ / JT)).ln()) * LA).exp()))) / LA) + (IN * (E - KZ)));
                        LB
                    } else {
                        A
                    };
                    WZ = XB;
                }
                LD = ID;
                WV = WZ;
            }
            let LC = if IH < AM { 1.0 } else { 0.0 };
            let MD;
            if LC != 0.0 {
                let LE = if LD > A { 1.0 } else { 0.0 };
                let ME;
                if LE != 0.0 {
                    let LF = CK / BJ;
                    let LG = IH - IL;
                    let LH = IL * (AA - ((-8.754687373538999e-1f64 / CK).exp()));
                    let LI = IN * LD;
                    let LJ = LD * (((LF - CK) * ((IH / IL).ln())).exp());
                    let LK = (LH - G) * IR;
                    let LL = if LK < IT { 1.0 } else { 0.0 };
                    let LO = if LL != 0.0 {
                        let LM = LH - (IV * ((AA + (LK.exp())).ln()));
                        LM
                    } else {
                        G
                    };
                    let LN = (IX * LG) + (BJ * IV);
                    let LP = (LG + LO) / LN;
                    let LQ = if LP < IT { 1.0 } else { 0.0 };
                    let LS = if LQ != 0.0 {
                        let LR = (-LG) + (LN * (((AA + (LP.exp())).ln()) - (((-(LG + LH)) / LN).exp())));
                        LR
                    } else {
                        LO
                    };
                    let LT = (AA - (LS / IL)).ln();
                    let LU = AA - CK;
                    let LV = AA - LF;
                    let LW = (((((LD * (AA - ((LT * LU).exp()))) / LU) + ((LJ * (AA - ((((AA - (LO / IL)).ln()) * LV).exp()))) / LV)) - ((LJ * (AA - ((LT * LV).exp()))) / LV)) * IL) + (LI * (G - LO));
                    ME = LW;
                } else {
                    ME = A;
                }
                MD = ME;
            } else {
                let LX = if LD > A { 1.0 } else { 0.0 };
                let MF = if LX != 0.0 {
                    let LY = IL * (AA - ((-8.754687373538999e-1f64 / CK).exp()));
                    let LZ = (LY - G) * IR;
                    let MA = LY - (IV * ((LZ + (((LZ * LZ) + JL).sqrt())) * R));
                    let MB = AA - CK;
                    let MC = LD * (((IL * (AA - ((((AA - (MA / IL)).ln()) * MB).exp()))) / MB) + (IN * (G - MA)));
                    MC
                } else {
                    A
                };
                MD = MF;
            }
            let MG = if LD > A { 1.0 } else { 0.0 };
            let ND = if MG != 0.0 {
                let MH = IL * (AA - ((-8.754687373538999e-1f64 / CK).exp()));
                let MI = (MH - G) * IR;
                let MJ = ((MI * MI) + JL).sqrt();
                let MK = (MI + MJ) * R;
                let ML = MK / MJ;
                let MM = ((LD * (((-CK) * ((AA - ((MH - (IV * MK)) / IL)).ln())).exp())) * ML) + ((IN * LD) * (AA - ML));
                MM
            } else {
                A
            };
            let MV = if DE != 0.0 {
                let MQ = MN - G;
                MQ
            } else {
                let MU = I - MR;
                MU
            };
            let MW = (MV * IR) - AA;
            let MX = (AA + ((MW + (((MW * MW) + JL).sqrt())) / BB)) * IV;
            let NB = (MX - MY) / parameters[63];
            let NC = ((MX * MZ) / ((((AA + ((NA * ((MX / MY).ln())).exp())).ln()) / NA).exp())) * (AA + (R * (NB + (((NB * NB) + parameters[66]).sqrt()))));
            let NE = if (if ND > A { 1.0 } else { 0.0 }) != 0.0 && MG != 0.0 { 1.0 } else { 0.0 };
            let OR;
            let OV;
            if NE != 0.0 {
                let NF = LD / ND;
                let NG = MD / LD;
                OR = NG;
                OV = NF;
            } else {
                OR = A;
                OV = AA;
            }
            let NI = if NH > A { 1.0 } else { 0.0 };
            let NQ = if NI != 0.0 {
                let NL = NJ * (AA - (((-(NK.ln())) / BL).exp()));
                let NM = (NL - H) * IR;
                let NN = NL - (IV * ((NM + (((NM * NM) + JL).sqrt())) * R));
                let NO = AA - BL;
                let NP = NH * (((NJ * (AA - ((((AA - (NN / NJ)).ln()) * NO).exp()))) / NO) + (NK * (H - NN)));
                NP
            } else {
                A
            };
            let NR = NQ / NH;
            let OF;
            let OI;
            let OP;
            if EU != 0.0 {
                let NT = if NS > A { 1.0 } else { 0.0 };
                let OB = if NT != 0.0 {
                    let NW = NU * (AA - (((-(NV.ln())) / BU).exp()));
                    let NX = (NW - H) * IR;
                    let NY = NW - (IV * ((NX + (((NX * NX) + JL).sqrt())) * R));
                    let NZ = AA - BU;
                    let OA = NS * (((NU * (AA - ((((AA - (NY / NU)).ln()) * NZ).exp()))) / NZ) + (NV * (H - NY)));
                    OA
                } else {
                    A
                };
                let OC = OB / NS;
                OF = NU;
                OI = BU;
                OP = OC;
            } else {
                OF = NJ;
                OI = BL;
                OP = NR;
            }
            let OD = if EO == A { 1.0 } else { 0.0 };
            let ON;
            if OD != 0.0 {
                ON = AA;
            } else {
                let OE = parameters[8] * IV;
                let OG = (OF - H) / OE;
                let OJ = OH * (AA - ((OI * ((AA - ((OF - ((OE * (OG + (((OG * OG) + JL).sqrt()))) * R)) / OF)).ln())).exp()));
                let OK = if (OJ.abs()) >= 1e-3f64 { 1.0 } else { 0.0 };
                let OO = if OK != 0.0 {
                    let OL = ((OJ.exp()) - AA) / OJ;
                    OL
                } else {
                    let OM = AA + (OJ * R);
                    OM
                };
                ON = OO;
            }
            let OS = (2e1f64 * ((AA + ((ON * OP) / OQ)) + (OR / parameters[5]))) - AA;
            let OT = 2.5e-2f64 * (AA + ((OS + (((OS * OS) + JL).sqrt())) / BB));
            let OW = (OU + (parameters[55] * (OV - AA))) + (parameters[56] * ((AA / OV) - AA));
            let OX = if parameters[10] == AA { 1.0 } else { 0.0 };
            let PP = if OX != 0.0 {
                let OZ = OY / (AA + ((OW / OU) - AA));
                OZ
            } else {
                OY
            };
            let PB = H / (parameters[3] * IV);
            let PC = if PB > IT { 1.0 } else { 0.0 };
            let PE;
            let PF;
            if PC != 0.0 {
                let PD = AA + (PB - IT);
                PE = PD;
                PF = IT;
            } else {
                PE = AA;
                PF = PB;
            }
            let PH = PG * (PE * (rspice_limexp(PF)));
            let PI = G / (parameters[4] * IV);
            let PJ = if PI > IT { 1.0 } else { 0.0 };
            let PL;
            let PM;
            if PJ != 0.0 {
                let PK = AA + (PI - IT);
                PL = PK;
                PM = IT;
            } else {
                PL = AA;
                PM = PI;
            }
            let PN = PG * (PL * (rspice_limexp(PM)));
            let PO = if FD != A { 1.0 } else { 0.0 };
            let QA;
            let QC;
            if PO != 0.0 {
                let PQ = (PH / PP) + (PN / PA);
                let PU = (PR * (((PH * (PH / NC)) * (PS / PT)).ln())).exp();
                let PV = PQ + PU;
                let PW = (PQ + (PH / PT)) + PU;
                QA = PV;
                QC = PW;
            } else {
                let PX = (PH / PP) + (PN / PA);
                let PY = PX + (PH / PT);
                QA = PX;
                QC = PY;
            }
            let PZ = OT * OT;
            let QB = OT + ((PZ + QA).sqrt());
            let QD = OT + ((PZ + QC).sqrt());
            let QE = if ((QC - QA).abs()) > 1e-8f64 { 1.0 } else { 0.0 };
            let QK = if QE != 0.0 {
                let QF = (NC / (AA + parameters[14])) / PH;
                let QG = (AA - (QF * QB)) / (AA + (QF * (QD - QB)));
                let QI = ((((QG * QG) + QH).sqrt()) + QG) / 2.004987562112089e0f64;
                QI
            } else {
                A
            };
            let QJ = if parameters[2] == A { 1.0 } else { 0.0 };
            let SA;
            if QJ != 0.0 {
                let QN = if PO != 0.0 {
                    let QL = (((PH / PP) + (PN / PA)) + (((PH / PT) * QK) * QK)) + ((PR * (((PH * (PH / NC)) * (PS / PT)).ln())).exp());
                    QL
                } else {
                    let QM = ((PH / PP) + (PN / PA)) + (((PH / PT) * QK) * QK);
                    QM
                };
                let QO = OT + ((PZ + QN).sqrt());
                SA = QO;
            } else {
                let QQ = -2e0f64 * OT;
                let QS = if (if CR == QR { 1.0 } else { 0.0 }) != 0.0 && (if FA == QR { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let QV = if QS != 0.0 {
                    A
                } else {
                    let QT = -(((PH / PP) + (PN / PA)) + (((PH / PT) * QK) * QK));
                    QT
                };
                let QU = QQ * QQ;
                let QW = QV - (QU * QP);
                let QY = ((((BB * QQ) * QU) / QX) - ((QQ * QV) * QP)) + (((((-PH) * PH) / NC) * PS) / PT);
                let QZ = (QW * QW) * QW;
                let RA = ((QY * QY) * 2.5e-1f64) + (QZ / QX);
                let RB = if (RA.abs()) < 1e-10f64 { 1.0 } else { 0.0 };
                let RY;
                if RB != 0.0 {
                    let RC = ((Y * QY) / QW) - (QQ * QP);
                    RY = RC;
                } else {
                    let RD = if RA > A { 1.0 } else { 0.0 };
                    let RZ;
                    if RD != 0.0 {
                        let RE = (-QY) * R;
                        let RF = RA.sqrt();
                        let RG = RE + RF;
                        let RH = if RG > A { 1.0 } else { 0.0 };
                        let RO = if RH != 0.0 {
                            let RI = (QP * (RG.ln())).exp();
                            RI
                        } else {
                            let RJ = -((QP * ((-RG).ln())).exp());
                            RJ
                        };
                        let RK = RE - RF;
                        let RL = if RK > A { 1.0 } else { 0.0 };
                        let RP = if RL != 0.0 {
                            let RM = (QP * (RK.ln())).exp();
                            RM
                        } else {
                            let RN = -((QP * ((-RK).ln())).exp());
                            RN
                        };
                        let RQ = (RO + RP) - (QQ * QP);
                        RZ = RQ;
                    } else {
                        let RR = ((-QY) * R) * ((-2.7e1f64 / QZ).sqrt());
                        let RS = RR * RR;
                        let RT = if RR >= A { 1.0 } else { 0.0 };
                        let RW = if RT != 0.0 {
                            let RU = 1.5707963267948966e0f64 - (((RS / (AA - RS)).sqrt()).atan());
                            RU
                        } else {
                            let RV = 1.5707963267948966e0f64 + (((RS / (AA - RS)).sqrt()).atan());
                            RV
                        };
                        let RX = ((((-4e0f64 * QW) * QP).sqrt()) * ((QP * RW).cos())) - (QQ * QP);
                        RZ = RX;
                    }
                    RY = RZ;
                }
                SA = RY;
            }
            let SC = if SA < SB { 1.0 } else { 0.0 };
            let SD = if SC != 0.0 {
                SB
            } else {
                SA
            };
            let SE = PH / SD;
            let SF = PN / SD;
            let SG = if SE < SB { 1.0 } else { 0.0 };
            let SH = if SG != 0.0 {
                SB
            } else {
                SE
            };
            let SI = SH - SF;
            let SJ = if BY > A { 1.0 } else { 0.0 };
            let SZ;
            if SJ != 0.0 {
                let SK = H / (parameters[16] * IV);
                let SL = if SK > IT { 1.0 } else { 0.0 };
                let SO;
                let SP;
                if SL != 0.0 {
                    let SM = AA + (SK - IT);
                    SO = SM;
                    SP = IT;
                } else {
                    SO = AA;
                    SP = SK;
                }
                let SQ = SN * ((SO * (rspice_limexp(SP))) - AA);
                SZ = SQ;
            } else {
                SZ = A;
            }
            let SR = if CA > A { 1.0 } else { 0.0 };
            let TA;
            if SR != 0.0 {
                let SS = H / (parameters[18] * IV);
                let ST = if SS > IT { 1.0 } else { 0.0 };
                let SW;
                let SX;
                if ST != 0.0 {
                    let SU = AA + (SS - IT);
                    SW = SU;
                    SX = IT;
                } else {
                    SW = AA;
                    SX = SS;
                }
                let SY = SV * ((SW * (rspice_limexp(SX))) - AA);
                TA = SY;
            } else {
                TA = A;
            }
            let TB = SZ + TA;
            let TC = if CM > A { 1.0 } else { 0.0 };
            let TK;
            if TC != 0.0 {
                let TD = G / (parameters[20] * IV);
                let TE = if TD > IT { 1.0 } else { 0.0 };
                let TH;
                let TI;
                if TE != 0.0 {
                    let TF = AA + (TD - IT);
                    TH = TF;
                    TI = IT;
                } else {
                    TH = AA;
                    TI = TD;
                }
                let TJ = TG * ((TH * (rspice_limexp(TI))) - AA);
                TK = TJ;
            } else {
                TK = A;
            }
            let TL = TB + TK;
            let UW;
            if LC != 0.0 {
                let TM = if ID > A { 1.0 } else { 0.0 };
                let UX;
                if TM != 0.0 {
                    let TN = CK / BJ;
                    let TO = IH - IL;
                    let TP = IL * (AA - ((-8.754687373538999e-1f64 / CK).exp()));
                    let TQ = IN * ID;
                    let TR = ID * (((TN - CK) * ((IH / IL).ln())).exp());
                    let TS = (TP - G) * IR;
                    let TT = if TS < IT { 1.0 } else { 0.0 };
                    let TZ;
                    let UH;
                    if TT != 0.0 {
                        let TU = TS.exp();
                        let TV = AA + TU;
                        let TW = TU / TV;
                        let TX = TP - (IV * (TV.ln()));
                        TZ = TX;
                        UH = TW;
                    } else {
                        TZ = G;
                        UH = AA;
                    }
                    let TY = (IX * TO) + (BJ * IV);
                    let UA = (TO + TZ) / TY;
                    let UB = if UA < IT { 1.0 } else { 0.0 };
                    let UG;
                    let UI;
                    if UB != 0.0 {
                        let UC = UA.exp();
                        let UD = AA + UC;
                        let UE = UC / UD;
                        let UF = (-TO) + (TY * ((UD.ln()) - (((-(TO + TP)) / TY).exp())));
                        UG = UF;
                        UI = UE;
                    } else {
                        UG = TZ;
                        UI = AA;
                    }
                    let UJ = ((((ID * ((((AA - (UG / IL)).ln()) * (-CK)).exp())) * UH) * UI) + ((TR * ((((AA - (TZ / IL)).ln()) * (-TN)).exp())) * (AA - UI))) + (TQ * (AA - UH));
                    UX = UJ;
                } else {
                    UX = A;
                }
                UW = UX;
            } else {
                let UK = if ID > A { 1.0 } else { 0.0 };
                let UY = if UK != 0.0 {
                    let UL = IL * (AA - ((-8.754687373538999e-1f64 / CK).exp()));
                    let UM = (UL - G) * IR;
                    let UN = ((UM * UM) + JL).sqrt();
                    let UO = (UM + UN) * R;
                    let UP = UO / UN;
                    let UQ = ID * (((((-CK) * ((AA - ((UL - (IV * UO)) / IL)).ln())).exp()) * UP) + (IN * (AA - UP)));
                    UQ
                } else {
                    A
                };
                UW = UY;
            }
            let WR;
            if DN != 0.0 {
                let UR = IL - G;
                let US = if UR > A { 1.0 } else { 0.0 };
                let WS;
                if US != 0.0 {
                    let UZ = UT / UW;
                    let VA = UT / ID;
                    let VB = if UR > VA { 1.0 } else { 0.0 };
                    let VH = if VB != 0.0 {
                        let VF = (VC * (((-UZ) / VA).exp())) * (VA + ((AA + (UZ / VA)) * (UR - VA)));
                        VF
                    } else {
                        let VG = (VC * UR) * (((-UZ) / UR).exp());
                        VG
                    };
                    let VI = SH * VH;
                    WS = VI;
                } else {
                    WS = A;
                }
                WR = WS;
            } else {
                WR = A;
            }
            let VK = if VJ > A { 1.0 } else { 0.0 };
            let VS;
            if VK != 0.0 {
                let VL = (((AA + (NR / parameters[24])) + (OR / parameters[25])) + (SH / PP)) + (SF / PA);
                let VM = VJ / (R * (VL + (((VL * VL) + QH).sqrt())));
                let VN = if TL > A { 1.0 } else { 0.0 };
                let VT;
                if VN != 0.0 {
                    let VO = ((parameters[27] * VM) * TL) * IR;
                    let VP = if VO < 1e-6f64 { 1.0 } else { 0.0 };
                    let VU = if VP != 0.0 {
                        let VQ = VM * (AA - (R * VO));
                        VQ
                    } else {
                        let VR = (VM * ((VO + AA).ln())) / VO;
                        VR
                    };
                    VT = VU;
                } else {
                    VT = VM;
                }
                VS = VT;
            } else {
                VS = A;
            }
            let VW = VS + VV;
            let VX = if EM > A { 1.0 } else { 0.0 };
            let XK = if VX != 0.0 {
                let VY = parameters[31] * IV;
                let WA = VZ * ((rspice_limexp((E / VY))) - (rspice_limexp((J / VY))));
                WA
            } else {
                A
            };
            let WB = if parameters[32] > A { 1.0 } else { 0.0 };
            if WB != 0.0 {
                let WC = if (J / (parameters[33] * IV)) > IT { 1.0 } else { 0.0 };
                if WC != 0.0 {
                } else {
                }
            } else {
            }
            let WE = if WD < AM { 1.0 } else { 0.0 };
            if WE != 0.0 {
                let WG = if WF > A { 1.0 } else { 0.0 };
                if WG != 0.0 {
                    let WI = WD - WH;
                    let WJ = WH * (AA - ((-8.754687373538999e-1f64 / EK).exp()));
                    let WK = (WJ - J) * IR;
                    let WL = if WK < IT { 1.0 } else { 0.0 };
                    let WN = if WL != 0.0 {
                        let WM = WJ - (IV * ((AA + (WK.exp())).ln()));
                        WM
                    } else {
                        J
                    };
                    let WO = if ((WI + WN) / ((IX * WI) + (BJ * IV))) < IT { 1.0 } else { 0.0 };
                    if WO != 0.0 {
                    } else {
                    }
                } else {
                }
            } else {
                let WP = if WF > A { 1.0 } else { 0.0 };
                if WP != 0.0 {
                } else {
                }
            }
            let WQ = if (if FP == AA { 1.0 } else { 0.0 }) != 0.0 && FR != 0.0 { 1.0 } else { 0.0 };
            if WQ != 0.0 {
            } else {
            }
            let WT = if (if parameters[73] != A { 1.0 } else { 0.0 }) != 0.0 && (if DH != A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let XH = if WT != 0.0 {
                WU
            } else {
                SH
            };
            let XC = B * WV;
            let XD = B * (parameters[70] * E);
            let XE = B * (parameters[69] * K);
            let XF = B * TB;
            let XG = B * SI;
            let XI = B * (XH - SF);
            let XJ = B * WR;
            let XL = B * XK;
            let XM = 0e0f64;
            let XN = 0e0f64;
            let XO = 0e0f64;
            let XP = if FL >= FQ { 1.0 } else { 0.0 };
            if XP != 0.0 {
            } else {
            }
            let XR = if FF >= FQ { 1.0 } else { 0.0 };
            if XR != 0.0 {
            } else {
            }
            let XT = if (if DT >= FQ { 1.0 } else { 0.0 }) != 0.0 || (if FI >= FQ { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let YJ = if XT != 0.0 {
                let XU = L / VW;
                XU
            } else {
                A
            };
            let XV = if FP == A { 1.0 } else { 0.0 };
            let XW = if XV != 0.0 || (if parameters[107] == A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            if XW != 0.0 {
            } else {
            }
            let XX = if XV != 0.0 || (if FO < FQ { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            if XX != 0.0 {
            } else {
            }
            let XZ = 5.5224904e-23f64 * XY;
            let YK;
            let YV;
            let YW;
            if XT != 0.0 {
                let YB = XZ / VW;
                YK = YA;
                YV = AA;
                YW = YB;
            } else {
                YK = A;
                YV = A;
                YW = A;
            }
            let YX;
            let YY;
            if XR != 0.0 {
                let YC = XZ / XS;
                YX = AA;
                YY = YC;
            } else {
                YX = A;
                YY = A;
            }
            let YZ;
            let ZA;
            if XP != 0.0 {
                let YD = XZ / XQ;
                YZ = AA;
                ZA = YD;
            } else {
                YZ = A;
                ZA = A;
            }
            let YE = XF.abs();
            let YF = parameters[74] * (YE.powf(parameters[75]));
            let YH = YG * YE;
            let YI = YG * (XI.abs());
            let YL = if ((((((XL + XM) + XN) + XO) + YJ) + YK) + branch_unknown_flows[2]) != A { 1.0 } else { 0.0 };
            if YL != 0.0 {
            } else {
            }
            let YM = (-0e0f64) - (-0e0f64);
            let YN = if (YM.abs()) > (ctx.simparam_or("gmin", A)) { 1.0 } else { 0.0 };
            if YN != 0.0 {
            } else {
                let YO = if YM >= A { 1.0 } else { 0.0 };
                if YO != 0.0 {
                } else {
                }
            }
            let YP = (-0e0f64) - (-0e0f64);
            let YQ = if (YP.abs()) > (ctx.simparam_or("gmin", A)) { 1.0 } else { 0.0 };
            if YQ != 0.0 {
            } else {
                let YR = if YP >= A { 1.0 } else { 0.0 };
                if YR != 0.0 {
                } else {
                }
            }
            let YS = 0e0f64;
            let YT = if (YS.abs()) > (ctx.simparam_or("gmin", A)) { 1.0 } else { 0.0 };
            if YT != 0.0 {
            } else {
                let YU = if YS >= A { 1.0 } else { 0.0 };
                if YU != 0.0 {
                } else {
                }
            }
        if YV == 0.0 {
            if !visitor.visit(0, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = YW;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 0, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 0, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(0, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if YX == 0.0 {
            if !visitor.visit(1, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = YY;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(1, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if YZ == 0.0 {
            if !visitor.visit(2, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = ZA;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 2, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 2, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(2, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = YF;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 3, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = Some(AA);
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 3, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 3, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(3, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = YH;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 4, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 4, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(4, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = YI;
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
