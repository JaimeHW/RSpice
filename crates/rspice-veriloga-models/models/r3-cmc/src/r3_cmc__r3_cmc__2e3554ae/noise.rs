#![allow(dead_code, non_snake_case, unused_parens, unused_variables)]

use super::state::Instance;
use rspice_veriloga_runtime::GeneratedEvalContext;
pub use rspice_veriloga_runtime::{GeneratedNoiseDescriptor, GeneratedNoiseEndpoint, GeneratedNoiseEvaluation, GeneratedNoiseEvaluationError, GeneratedNoiseEvaluationRef, GeneratedNoiseKind, GeneratedNoiseVisitor};

pub static NOISE_SOURCES: [GeneratedNoiseDescriptor; 6] = [
    GeneratedNoiseDescriptor { mechanism: "WHITE_I2_I1_BODY_THERMAL_NOISE", label: Some("body thermal noise"), kind: GeneratedNoiseKind::White, equation: 12, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "i2", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(4), name: "i1", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_I2_I1_BODY_1_F_NOISE", label: Some("body 1/f noise"), kind: GeneratedNoiseKind::Flicker, equation: 13, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "i2", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(4), name: "i1", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_N1_I1_END_1_RESISTANCE_THERMAL_NOISE", label: Some("end 1 resistance thermal noise"), kind: GeneratedNoiseKind::White, equation: 14, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(0), name: "n1", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(4), name: "i1", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_N2_I2_END_2_RESISTANCE_THERMAL_NOISE", label: Some("end 2 resistance thermal noise"), kind: GeneratedNoiseKind::White, equation: 15, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(2), name: "n2", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(5), name: "i2", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_NC_I1_END_1_PARASITIC_SHOT_NOISE", label: Some("end 1 parasitic shot noise"), kind: GeneratedNoiseKind::White, equation: 16, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(1), name: "nc", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(4), name: "i1", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_NC_I2_END_2_PARASITIC_SHOT_NOISE", label: Some("end 2 parasitic shot noise"), kind: GeneratedNoiseKind::White, equation: 17, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(1), name: "nc", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(5), name: "i2", is_internal: true }, table_len: 0, table_log_interp: false },
];

impl Instance {
    pub fn evaluate_noise_sources(&self, ctx: &GeneratedEvalContext<'_>, visitor: &mut dyn GeneratedNoiseVisitor) -> Result<(), GeneratedNoiseEvaluationError> {
        if !self.multiplicity.is_finite() || self.multiplicity <= 0.0 {
            return Err(GeneratedNoiseEvaluationError::InvalidMultiplicity { value: self.multiplicity });
        }
        let parameters = &self.params.values;
        let multiplicity = self.multiplicity;
        let temperature = ctx.temperature();
        let node_potentials = [ctx.node_voltage(self.nodes[0]), ctx.node_voltage(self.nodes[1]), ctx.node_voltage(self.nodes[2]), ctx.node_voltage(self.nodes[3]), ctx.node_voltage(self.nodes[4]), ctx.node_voltage(self.nodes[5])];
            let A = 0e0f64;
            let C = 1e0f64;
            let F = multiplicity;
            let H = 1e-2f64;
            let K = 2.7315e2f64;
            let Q = parameters[35];
            let U = parameters[36];
            let AA = 1.3806505e-23f64;
            let AB = 1.60217653e-19f64;
            let AQ = 2e0f64;
            let AR = parameters[5];
            let AT = parameters[8];
            let AX = 5e-1f64;
            let BB = parameters[127];
            let BC = parameters[16];
            let BD = parameters[119];
            let BE = parameters[122];
            let BF = parameters[125];
            let BJ = parameters[120];
            let BK = parameters[123];
            let BL = parameters[126];
            let BO = parameters[118];
            let BP = parameters[121];
            let BQ = parameters[124];
            let CJ = parameters[53];
            let CP = 1e-1f64;
            let CT = 1e4f64;
            let CV = parameters[15];
            let DG = parameters[63];
            let DI = parameters[64];
            let DN = parameters[47];
            let DP = parameters[46];
            let DX = 4e0f64;
            let EG = 1e-99f64;
            let EI = parameters[66];
            let EL = parameters[67];
            let EP = parameters[109];
            let ET = parameters[72];
            let EU = parameters[79];
            let EY = node_potentials[5];
            let EZ = node_potentials[4];
            let FB = node_potentials[1];
            let FR = 1e1f64;
            let GA = parameters[69];
            let GC = parameters[90];
            let GD = parameters[91];
            let GE = parameters[70];
            let GG = parameters[27];
            let GI = parameters[76];
            let GK = parameters[77];
            let GW = parameters[73];
            let GX = 3e0f64;
            let HC = parameters[80];
            let HJ = parameters[83];
            let HO = parameters[85];
            let HR = parameters[84];
            let HT = parameters[60];
            let HW = parameters[62];
            let HX = parameters[61];
            let IB = parameters[65];
            let II = 1e3f64;
            let IL = 1e5f64;
            let IO = -1e0f64;
            let JR = 1.5e0f64;
            let KA = 3.333333333333333e-1f64;
            let KE = 2.5e-1f64;
            let KM = 1e-6f64;
            let QV = parameters[33];
            let QY = parameters[34];
            let RE = 4e-2f64;
            let RL = parameters[68];
            let RN = parameters[75];
            let RU = parameters[82];
            let SL = parameters[26];
            let SQ = parameters[13];
            let SR = parameters[89];
            let SZ = parameters[88];
            let B = if 1.003e3f64 != parameters[20] { 1.0 } else { 0.0 };
            if B != 0.0 {
            } else {
            }
            let D = if C != parameters[17] { 1.0 } else { 0.0 };
            if D != 0.0 {
            } else {
            }
            let E = if C < parameters[18] { 1.0 } else { 0.0 };
            if E != 0.0 {
            } else {
            }
            let G = ctx.simparam_or("gmin", A);
            let I = ((C - (H * parameters[23])) * parameters[22]) * 1e6f64;
            let J = I * I;
            let L = K + parameters[28];
            if C != 0.0 {
            } else {
            }
            let M = temperature + parameters[9];
            let N = M - K;
            let O = if N < parameters[24] { 1.0 } else { 0.0 };
            if O != 0.0 {
            } else {
            }
            let P = if N > parameters[25] { 1.0 } else { 0.0 };
            if P != 0.0 {
            } else {
            }
            let R = Q + C;
            let S = if N < R { 1.0 } else { 0.0 };
            let X;
            if S != 0.0 {
                let T = Q + (((N - Q) - C).exp());
                X = T;
            } else {
                let V = if N > (U - C) { 1.0 } else { 0.0 };
                let Y = if V != 0.0 {
                    let W = U - (((U - N) - C).exp());
                    W
                } else {
                    N
                };
                X = Y;
            }
            let Z = X + K;
            let AC = (AA * Z) / AB;
            let AD = Z / L;
            let AE = Z - L;
            let AF = parameters[0] * I;
            let AG = parameters[1] * I;
            let AH = if AF < parameters[31] { 1.0 } else { 0.0 };
            if AH != 0.0 {
            } else {
            }
            let AI = if AF > parameters[32] { 1.0 } else { 0.0 };
            if AI != 0.0 {
            } else {
            }
            let AJ = if AG < parameters[29] { 1.0 } else { 0.0 };
            if AJ != 0.0 {
            } else {
            }
            let AK = if AG > parameters[30] { 1.0 } else { 0.0 };
            if AK != 0.0 {
            } else {
            }
            let AL = 0e0f64 * J;
            let AM = parameters[4] * I;
            let AN = 0e0f64 * J;
            let AO = parameters[7] * I;
            let AP = AG * AF;
            let AS = if AR > A { 1.0 } else { 0.0 };
            let AU = if AT > A { 1.0 } else { 0.0 };
            let AV = AS + AU;
            let AW = (AQ * AG) + (AV * AF);
            let AY = AX * AV;
            let AZ = (((AF + parameters[38]) + (parameters[39] / AF)) + (parameters[42] * (C - (((-AF) / parameters[41]).exp())))) / (C - ((parameters[40] * (parameters[2] * I)) / AP));
            let BA = AG + (AY * (parameters[43] + (parameters[44] / AF)));
            let BG;
            let BM;
            if BB != 0.0 {
                BG = BA;
                BM = AZ;
            } else {
                BG = AG;
                BM = AF;
            }
            let CB;
            let CE;
            let EB;
            if BC != 0.0 {
                let BH = F * BG;
                let BI = (AZ + (BD * BE)) + ((parameters[11] * BF) / (BH.sqrt()));
                let BN = (BA + (BJ * BK)) + ((parameters[12] * BL) / ((F * BM).sqrt()));
                let BR = (H * ((BO * BP) + ((parameters[10] * BQ) / ((BH * BM).sqrt())))).exp();
                CB = BI;
                CE = BN;
                EB = BR;
            } else {
                let BS = if (if BD != A { 1.0 } else { 0.0 }) != 0.0 && (if (if BF > A { 1.0 } else { 0.0 }) != 0.0 || (if BE > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let CC = if BS != 0.0 {
                    let BT = BF / ((F * BG).sqrt());
                    let BU = AZ + (BD * (((BE * BE) + (BT * BT)).sqrt()));
                    BU
                } else {
                    AZ
                };
                let BV = if (if BJ != A { 1.0 } else { 0.0 }) != 0.0 && (if (if BL > A { 1.0 } else { 0.0 }) != 0.0 || (if BK > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let CF = if BV != 0.0 {
                    let BW = BL / ((F * BM).sqrt());
                    let BX = BA + (BJ * (((BK * BK) + (BW * BW)).sqrt()));
                    BX
                } else {
                    BA
                };
                let BY = if (if BO != A { 1.0 } else { 0.0 }) != 0.0 && (if (if BQ > A { 1.0 } else { 0.0 }) != 0.0 || (if BP > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let EC = if BY != 0.0 {
                    let BZ = BQ / (((F * BG) * BM).sqrt());
                    let CA = ((H * BO) * (((BP * BP) + (BZ * BZ)).sqrt())).exp();
                    CA
                } else {
                    C
                };
                CB = CC;
                CE = CF;
                EB = EC;
            }
            let CD = if CB <= A { 1.0 } else { 0.0 };
            if CD != 0.0 {
            } else {
            }
            let CG = if CE <= A { 1.0 } else { 0.0 };
            if CG != 0.0 {
            } else {
            }
            let CH = CE + parameters[45];
            let CI = if CH <= A { 1.0 } else { 0.0 };
            if CI != 0.0 {
            } else {
            }
            let CK;
            let CM;
            if CJ != 0.0 {
                CK = CB;
                CM = CE;
            } else {
                CK = AF;
                CM = AG;
            }
            let CL = C / (CK.powf(parameters[56]));
            let CN = C / (CM.powf(parameters[58]));
            let CO = (((parameters[54] * (C + (parameters[55] * CL))) * (C + (parameters[57] * CN))) * (C + ((parameters[59] * CL) * CN))) * (C + (AE * (parameters[103] + (AE * parameters[104]))));
            let CQ = if CO > CP { 1.0 } else { 0.0 };
            let CR = if CQ != 0.0 {
                CO
            } else {
                CP
            };
            let CS = CR.sqrt();
            let CU = CS / (CR + CT);
            let CX = if CV != 0.0 {
                A
            } else {
                let CW = parameters[49] + ((((parameters[50] * CM) + (parameters[51] * CK)) + parameters[52]) / (CM * CK));
                CW
            };
            let CY = if CX < CU { 1.0 } else { 0.0 };
            let DD;
            let ED;
            if CY != 0.0 {
                let CZ = if CX > A { 1.0 } else { 0.0 };
                let DA = if CZ != 0.0 {
                    CX
                } else {
                    A
                };
                let DB = CU * CU;
                DD = DB;
                ED = DA;
            } else {
                let DC = CX * CX;
                DD = DC;
                ED = CX;
            }
            let DE = CR * AX;
            let DF = (AX / DD) - DE;
            let DH = if DG > C { 1.0 } else { 0.0 };
            let IT;
            let LL;
            if DH != 0.0 {
                let DJ = DF - ((AQ * DI) / DD);
                let DK = (1.666666666666667e-1f64 / DD) - DE;
                IT = DJ;
                LL = DK;
            } else {
                let DL = if DG > A { 1.0 } else { 0.0 };
                let IU = if DL != 0.0 {
                    let DM = DF - (((AQ * DI) / DD).sqrt());
                    DM
                } else {
                    DF
                };
                IT = IU;
                LL = A;
            }
            let DO = DN / (C + (parameters[48] / CE));
            let IW;
            let ML;
            if DH != 0.0 {
                let DQ = DP * AC;
                let DR = if DG > AQ { 1.0 } else { 0.0 };
                let DU = if DR != 0.0 {
                    let DS = (5.5e-1f64 * AC) * (C + (((-DO) / AC).exp()));
                    DS
                } else {
                    let DT = 1.1e0f64 * AC;
                    DT
                };
                IW = DQ;
                ML = DU;
            } else {
                let DV = if DG > A { 1.0 } else { 0.0 };
                let IX;
                let MM;
                if DV != 0.0 {
                    let DW = (AQ * DP) * AC;
                    let DY = (DX * DO) * DO;
                    IX = DW;
                    MM = DY;
                } else {
                    let DZ = DP * AC;
                    let EA = (DX * DO) * DO;
                    IX = DZ;
                    MM = EA;
                }
                IW = IX;
                ML = MM;
            }
            let EE = C - (ED * CS);
            let EF = ((parameters[37] * EB) * (CE / CB)) * EE;
            let EH = if EF <= EG { 1.0 } else { 0.0 };
            if EH != 0.0 {
            } else {
            }
            let EJ = if EI > A { 1.0 } else { 0.0 };
            let EK = if EJ != 0.0 && AS != 0.0 { 1.0 } else { 0.0 };
            let SK = if EK != 0.0 {
                let EM = (EI + (EL / AF)) / AR;
                EM
            } else {
                A
            };
            let EN = if EJ != 0.0 && AU != 0.0 { 1.0 } else { 0.0 };
            let SO = if EN != 0.0 {
                let EO = (EI + (EL / AF)) / AT;
                EO
            } else {
                A
            };
            let QM = if CV != 0.0 {
                A
            } else {
                let EQ = (((parameters[110] + (parameters[111] * AW)) + (parameters[112] * AP)) + (parameters[113] * (AR + AT))) * (AD.powf(EP));
                EQ
            };
            let ER = (parameters[93] + (parameters[97] / CB)) + ((AY * (parameters[95] + (parameters[99] / CB))) / CE);
            let ES = (parameters[94] + (parameters[98] / CB)) + ((AY * (parameters[96] + (parameters[100] / CB))) / CE);
            let EV = (ET * AL) + (EU * AM);
            let EW = (ET * AN) + (EU * AO);
            let EX = -parameters[21];
            let FA = EX * (EY - EZ);
            let FC = EX * (FB - EZ);
            let FD = EX * (FB - EY);
            let FE = (M + node_potentials[3]) - K;
            let FF = if FE < R { 1.0 } else { 0.0 };
            let FJ;
            if FF != 0.0 {
                let FG = Q + (((FE - Q) - C).exp());
                FJ = FG;
            } else {
                let FH = if FE > (U - C) { 1.0 } else { 0.0 };
                let FK = if FH != 0.0 {
                    let FI = U - (((U - FE) - C).exp());
                    FI
                } else {
                    FE
                };
                FJ = FK;
            }
            let FL = FJ + K;
            let FM = (AA * FL) / AB;
            let FN = FL / L;
            let FO = FL - L;
            let FP = C + (FO * (ER + (FO * ES)));
            let FQ = if FP < 1.1e-1f64 { 1.0 } else { 0.0 };
            let FT = if FQ != 0.0 {
                let FS = H + (CP * (((FR * (FP - H)) - C).exp()));
                FS
            } else {
                FP
            };
            let NQ = if DG != 0.0 {
                let FU = C / ((EF * EE) * FT);
                FU
            } else {
                let FV = C / (EF * FT);
                FV
            };
            let FW = C + (FO * (parameters[101] + (FO * parameters[102])));
            let FX = if FW < 1.1e-1f64 { 1.0 } else { 0.0 };
            let SN = if FX != 0.0 {
                let FY = H + (CP * (((FR * (FW - H)) - C).exp()));
                FY
            } else {
                FW
            };
            let FZ = FN.powf(parameters[92]);
            let GB = if GA > A { 1.0 } else { 0.0 };
            let GN;
            let OC;
            if GB != 0.0 {
                let GF = GA * ((((((-GC) * (C - FN)) / FM) + (GD * (FN.ln()))) / GE).exp());
                let GH = (GE * FM) * ((C + (GG / GF)).ln());
                GN = GF;
                OC = GH;
            } else {
                GN = A;
                OC = A;
            }
            let GJ = if GI > A { 1.0 } else { 0.0 };
            let GP;
            let OK;
            if GJ != 0.0 {
                let GL = GI * ((((((-GC) * (C - FN)) / FM) + (GD * (FN.ln()))) / GK).exp());
                let GM = (GK * FM) * ((C + (GG / GL)).ln());
                GP = GL;
                OK = GM;
            } else {
                GP = A;
                OK = A;
            }
            let GO = AL * GN;
            let GQ = AM * GP;
            let GR = GO + GQ;
            let GS = AN * GN;
            let GT = AO * GP;
            let GU = GS + GT;
            let GV = if ET > A { 1.0 } else { 0.0 };
            let RG;
            let RK;
            if GV != 0.0 {
                let GY = ((((AQ * (FM / FN)) * ((((((AX * GW) * FN) / FM).exp()) - ((((-5e-1f64 * GW) * FN) / FM).exp())).ln())) * FN) - ((GX * FM) * (FN.ln()))) - (GC * (FN - C));
                let GZ = GY + ((AQ * FM) * ((AX * (C + ((C + (DX * (((-GY) / FM).exp()))).sqrt()))).ln()));
                let HA = ET * ((GW / GZ).powf(parameters[74]));
                RG = HA;
                RK = GZ;
            } else {
                RG = A;
                RK = GW;
            }
            let HB = if EU > A { 1.0 } else { 0.0 };
            let RH;
            let RS;
            if HB != 0.0 {
                let HD = ((((AQ * (FM / FN)) * ((((((AX * HC) * FN) / FM).exp()) - ((((-5e-1f64 * HC) * FN) / FM).exp())).ln())) * FN) - ((GX * FM) * (FN.ln()))) - (GC * (FN - C));
                let HE = HD + ((AQ * FM) * ((AX * (C + ((C + (DX * (((-HD) / FM).exp()))).sqrt()))).ln()));
                let HF = EU * ((HC / HE).powf(parameters[81]));
                RH = HF;
                RS = HE;
            } else {
                RH = A;
                RS = HC;
            }
            let HG = (C + (FO * parameters[108])) * parameters[86];
            let HH = if HG > A { 1.0 } else { 0.0 };
            let HI = if HH != 0.0 {
                HG
            } else {
                A
            };
            let HK = if HJ > A { 1.0 } else { 0.0 };
            let OT;
            let OX;
            let OZ;
            if HK != 0.0 {
                let HL = HJ * (C + (FO * (parameters[105] + (FO * parameters[106]))));
                let HM = if HL > A { 1.0 } else { 0.0 };
                let HN = if HM != 0.0 {
                    HL
                } else {
                    A
                };
                let HP = HO * (C + (parameters[107] * FO));
                let HQ = HP * FM;
                let HS = HQ * (((((-HN) / HQ).exp()) + (GG / HR)).ln());
                OT = HN;
                OX = HP;
                OZ = HS;
            } else {
                OT = HJ;
                OX = HO;
                OZ = C;
            }
            let HU = if CV == 0.0 { 1.0 } else { 0.0 };
            let HV = if (if HT > A { 1.0 } else { 0.0 }) != 0.0 && HU != 0.0 { 1.0 } else { 0.0 };
            let IJ;
            let JO;
            let MA;
            let MD;
            let MG;
            if HV != 0.0 {
                let IA;
                let IC;
                if HW != 0.0 {
                    let HY = (HX * FZ) * FT;
                    let HZ = (HT * FZ) * FT;
                    IA = HY;
                    IC = HZ;
                } else {
                    IA = HX;
                    IC = HT;
                }
                let ID = (((IA * IA) + ((((DX * IB) * IB) * IC) * IC)).sqrt()) - ((AQ * IB) * IC);
                let IE = (IB * ID) / IC;
                let IF = (((ID * ID) / (IC * IC)) + (DX * IE)).sqrt();
                let IG = IC - IA;
                let IH = C / IC;
                IJ = IG;
                JO = IH;
                MA = ID;
                MD = IE;
                MG = IF;
            } else {
                IJ = II;
                JO = A;
                MA = A;
                MD = A;
                MG = A;
            }
            let IK = CH * IJ;
            let IM = if IK > IL { 1.0 } else { 0.0 };
            let JQ = if IM != 0.0 {
                IL
            } else {
                IK
            };
            let IN = if FA < A { 1.0 } else { 0.0 };
            let IS;
            let JB;
            let NW;
            if IN != 0.0 {
                let IP = -FD;
                let IQ = -FA;
                IS = IP;
                JB = IQ;
                NW = IO;
            } else {
                let IR = -FC;
                IS = IR;
                JB = FA;
                NW = C;
            }
            let IV = if IS > IT { 1.0 } else { 0.0 };
            let JA = if IV != 0.0 {
                let IY = IT - (IW * ((C + (((IT - IS) / IW).exp())).ln()));
                IY
            } else {
                let IZ = IS - (IW * ((C + (((IS - IT) / IW).exp())).ln()));
                IZ
            };
            let JK;
            if DG != 0.0 {
                let JC = IT - JA;
                let JD = if JB < JC { 1.0 } else { 0.0 };
                let JE = if JD != 0.0 {
                    JB
                } else {
                    JC
                };
                let JF = if JA < (-4e-1f64 * (CR + JE)) { 1.0 } else { 0.0 };
                let JL;
                if JF != 0.0 {
                    let JG = if JD != 0.0 {
                        JB
                    } else {
                        JC
                    };
                    let JH = -4e-1f64 * (CR + JG);
                    JL = JH;
                } else {
                    JL = JA;
                }
                JK = JL;
            } else {
                let JI = if JA < (-4e-1f64 * CR) { 1.0 } else { 0.0 };
                let JM = if JI != 0.0 {
                    let JJ = -4e-1f64 * CR;
                    JJ
                } else {
                    JA
                };
                JK = JM;
            }
            let JN = CR + (AQ * JK);
            let JP = if JO > A { 1.0 } else { 0.0 };
            let LT;
            if JP != 0.0 {
                let JS = ((DX * JQ) * JQ) / DD;
                let JT = (((DD * JN) * JN) - JN) * JS;
                let JU = (-1e0f64 + ((GX * DD) * JN)) * JS;
                let JV = (DD * (2.25e0f64 + (JN / JQ))) * JS;
                let JW = ((JR * DD) / JQ) * JS;
                let JX = JW * JW;
                let JY = -JV;
                let JZ = (JW * JU) - (DX * JT);
                let KB = JZ - ((JY * JY) * KA);
                let KC = ((((DX * JV) * JT) - (JU * JU)) - (JT * JX)) - ((JY * (JZ + (AQ * KB))) / 9e0f64);
                let KD = ((KB * KB) * KB) / 2.7e1f64;
                let KF = (((KE * KC) * KC) + KD).sqrt();
                let KG = if KC < A { 1.0 } else { 0.0 };
                let KL;
                let KS;
                if KG != 0.0 {
                    let KH = (-5e-1f64 * KC) + KF;
                    let KI = (-KD) / KH;
                    KL = KH;
                    KS = KI;
                } else {
                    let KJ = (-5e-1f64 * KC) - KF;
                    let KK = (-KD) / KJ;
                    KL = KK;
                    KS = KJ;
                }
                let KN = if KL > KM { 1.0 } else { 0.0 };
                let KY;
                if KN != 0.0 {
                    let KO = KL.powf(KA);
                    KY = KO;
                } else {
                    let KP = if KL < -1e-6f64 { 1.0 } else { 0.0 };
                    let KZ = if KP != 0.0 {
                        let KQ = -((-KL).powf(KA));
                        KQ
                    } else {
                        let KR = CT * KL;
                        KR
                    };
                    KY = KZ;
                }
                let KT = if KS > KM { 1.0 } else { 0.0 };
                let LA;
                if KT != 0.0 {
                    let KU = KS.powf(KA);
                    LA = KU;
                } else {
                    let KV = if KS < -1e-6f64 { 1.0 } else { 0.0 };
                    let LB = if KV != 0.0 {
                        let KW = -((-KS).powf(KA));
                        KW
                    } else {
                        let KX = CT * KS;
                        KX
                    };
                    LA = LB;
                }
                let LC = KE * JX;
                let LD = ((LC - JV) + ((KY + LA) - (JY * KA))).sqrt();
                let LE = ((7.5e-1f64 * JX) - (LD * LD)) - (AQ * JV);
                let LF = (((JW * JV) - (AQ * JU)) - (LC * JW)) / LD;
                let LG = LE + LF;
                let LH = if LG > A { 1.0 } else { 0.0 };
                let LU = if LH != 0.0 {
                    let LI = (-2.5e-1f64 * JW) + (AX * ((LG.sqrt()) + LD));
                    LI
                } else {
                    let LJ = LE - LF;
                    let LK = (-2.5e-1f64 * JW) + (AX * (((((LJ * LJ) + 1e-4f64).sqrt()).sqrt()) - LD));
                    LK
                };
                LT = LU;
            } else {
                let LM = if JK > LL { 1.0 } else { 0.0 };
                let LV = if LM != 0.0 {
                    let LN = DF - JK;
                    let LO = DD * LN;
                    let LP = ((AQ * (C - (AQ * LO))) * LN) / ((C - (GX * LO)) + ((C - (JR * LO)).sqrt()));
                    LP
                } else {
                    let LQ = (GX * DD) * JN;
                    let LR = ((C - LQ) + ((C + LQ).sqrt())) / (4.5e0f64 * DD);
                    LR
                };
                LT = LV;
            }
            let LS = if DH != 0.0 && (if ED > 1e-9f64 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let NO;
            let NS;
            let NX;
            if LS != 0.0 {
                let LW = LT + AC;
                let LX = JN + LT;
                let LY = ED * (LX.sqrt());
                let MJ = if JP != 0.0 {
                    let LZ = LW / CH;
                    let MB = (AX * (LZ - MA)) * JO;
                    let MC = (AX * (LZ + MA)) * JO;
                    let ME = ((MB * MB) + MD).sqrt();
                    let MF = ((MC * MC) + MD).sqrt();
                    let MH = ((((AQ * LY) * (C - LY)) * (C - (((((AX * ((MB / ME) + (MC / MF))) * JO) / CH) * LW) / (C + ((ME + MF) - MG))))) / LW).sqrt();
                    MH
                } else {
                    let MI = (((AQ * LY) * (C - LY)) / LW).sqrt();
                    MI
                };
                let MK = ((DD * LX) / (MJ * MJ)) - LW;
                let MN = DN + LW;
                let MO = ML + ((DN * LT) / MN);
                let MP = (DX * MO) * MO;
                let MQ = (AQ * JB) * LW;
                let MR = JB - LW;
                let MS = MR * MR;
                let MT = JB + LW;
                let MU = MT * MT;
                let MV = MQ / (((MS + MP).sqrt()) + ((MU + MP).sqrt()));
                let MW = if DG > AQ { 1.0 } else { 0.0 };
                let NA = if MW != 0.0 {
                    let MX = ML + ((DN * MV) / MN);
                    let MY = (DX * MX) * MX;
                    let MZ = MQ / (((MS + MY).sqrt()) + ((MU + MY).sqrt()));
                    MZ
                } else {
                    MV
                };
                let NB = C - (MJ * ((MK + NA).sqrt()));
                let NT = if JP != 0.0 {
                    let NC = NA / CH;
                    let ND = (AX * (NC - MA)) * JO;
                    let NE = (AX * (NC + MA)) * JO;
                    let NF = ((((ND * ND) + MD).sqrt()) + (((NE * NE) + MD).sqrt())) - MG;
                    NF
                } else {
                    A
                };
                NO = NB;
                NS = NT;
                NX = NA;
            } else {
                let NG = JB - LT;
                let NH = JB + LT;
                let NI = ((AQ * JB) * LT) / ((((NG * NG) + ML).sqrt()) + (((NH * NH) + ML).sqrt()));
                let NU = if JP != 0.0 {
                    let NJ = NI / CH;
                    let NK = (AX * (NJ - MA)) * JO;
                    let NL = (AX * (NJ + MA)) * JO;
                    let NM = ((((NK * NK) + MD).sqrt()) + (((NL * NL) + MD).sqrt())) - MG;
                    NM
                } else {
                    A
                };
                let NN = C - (ED * ((JN + NI).sqrt()));
                NO = NN;
                NS = NU;
                NX = NI;
            }
            let NP = if NO < DI { 1.0 } else { 0.0 };
            let NR = if NP != 0.0 {
                DI
            } else {
                NO
            };
            let NV = (NQ * NR) / (C + NS);
            let NY = (NW * NV) * NX;
            let NZ = if GR > A { 1.0 } else { 0.0 };
            let QK;
            let TI;
            let TJ;
            if NZ != 0.0 {
                let OA = if GO > A { 1.0 } else { 0.0 };
                let OQ;
                if OA != 0.0 {
                    let OB = C / (GE * FM);
                    let OD = if FC < OC { 1.0 } else { 0.0 };
                    let OG = if OD != 0.0 {
                        let OE = (FC * OB).exp();
                        OE
                    } else {
                        let OF = ((OC * OB).exp()) * (C + ((FC - OC) * OB));
                        OF
                    };
                    let OH = GO * (OG - C);
                    OQ = OH;
                } else {
                    OQ = A;
                }
                let OI = if GQ > A { 1.0 } else { 0.0 };
                let OR;
                if OI != 0.0 {
                    let OJ = C / (GK * FM);
                    let OL = if FC < OK { 1.0 } else { 0.0 };
                    let OO = if OL != 0.0 {
                        let OM = (FC * OJ).exp();
                        OM
                    } else {
                        let ON = ((OK * OJ).exp()) * (C + ((FC - OK) * OJ));
                        ON
                    };
                    let OP = GQ * (OO - C);
                    OR = OP;
                } else {
                    OR = A;
                }
                let OS = OQ + OR;
                let OU = if OT > A { 1.0 } else { 0.0 };
                let PF;
                if OU != 0.0 {
                    let OV = -OT;
                    let OW = OV - FC;
                    let OY = C / (OX * FM);
                    let PA = if OW < OZ { 1.0 } else { 0.0 };
                    let PD = if PA != 0.0 {
                        let PB = (OW * OY).exp();
                        PB
                    } else {
                        let PC = ((OZ * OY).exp()) * (C + ((OW - OZ) * OY));
                        PC
                    };
                    let PE = (-HR) * (PD - ((OV * OY).exp()));
                    PF = PE;
                } else {
                    PF = A;
                }
                let PG = (OS + PF) + (G * FC);
                QK = PG;
                TI = OS;
                TJ = PF;
            } else {
                QK = A;
                TI = A;
                TJ = A;
            }
            let PH = if GU > A { 1.0 } else { 0.0 };
            let QL;
            let TL;
            let TM;
            if PH != 0.0 {
                let PI = if GS > A { 1.0 } else { 0.0 };
                let PW;
                if PI != 0.0 {
                    let PJ = C / (GE * FM);
                    let PK = if FD < OC { 1.0 } else { 0.0 };
                    let PN = if PK != 0.0 {
                        let PL = (FD * PJ).exp();
                        PL
                    } else {
                        let PM = ((OC * PJ).exp()) * (C + ((FD - OC) * PJ));
                        PM
                    };
                    let PO = GS * (PN - C);
                    PW = PO;
                } else {
                    PW = A;
                }
                let PP = if GT > A { 1.0 } else { 0.0 };
                let PX;
                if PP != 0.0 {
                    let PQ = C / (GK * FM);
                    let PR = if FD < OK { 1.0 } else { 0.0 };
                    let PU = if PR != 0.0 {
                        let PS = (FD * PQ).exp();
                        PS
                    } else {
                        let PT = ((OK * PQ).exp()) * (C + ((FD - OK) * PQ));
                        PT
                    };
                    let PV = GT * (PU - C);
                    PX = PV;
                } else {
                    PX = A;
                }
                let PY = PW + PX;
                let PZ = if OT > A { 1.0 } else { 0.0 };
                let QI;
                if PZ != 0.0 {
                    let QA = -OT;
                    let QB = QA - FD;
                    let QC = C / (OX * FM);
                    let QD = if QB < OZ { 1.0 } else { 0.0 };
                    let QG = if QD != 0.0 {
                        let QE = (QB * QC).exp();
                        QE
                    } else {
                        let QF = ((OZ * QC).exp()) * (C + ((QB - OZ) * QC));
                        QF
                    };
                    let QH = (-HR) * (QG - ((QA * QC).exp()));
                    QI = QH;
                } else {
                    QI = A;
                }
                let QJ = (PY + QI) + (G * FD);
                QL = QJ;
                TL = PY;
                TM = QI;
            } else {
                QL = A;
                TL = A;
                TM = A;
            }
            let QN = if QM > A { 1.0 } else { 0.0 };
            let QO = if (if QN != 0.0 && parameters[14] != 0.0 { 1.0 } else { 0.0 }) != 0.0 && HU != 0.0 { 1.0 } else { 0.0 };
            if QO != 0.0 {
                let QP = if EP == A { 1.0 } else { 0.0 };
                if QP != 0.0 {
                } else {
                    if S != 0.0 {
                    } else {
                        let QQ = if N > (U - C) { 1.0 } else { 0.0 };
                        if QQ != 0.0 {
                        } else {
                        }
                    }
                    let QR = if ((EP + C).abs()) > CP { 1.0 } else { 0.0 };
                    if QR != 0.0 {
                    } else {
                    }
                }
            } else {
            }
            let QS = EX * NY;
            let QT = EX * QK;
            let QU = EX * QL;
            let QW = if ((QS / CB).abs()) > QV { 1.0 } else { 0.0 };
            if QW != 0.0 {
            } else {
            }
            let QX = if ((QT / CB).abs()) > QV { 1.0 } else { 0.0 };
            if QX != 0.0 {
            } else {
            }
            let QZ = if (FC.abs()) > QY { 1.0 } else { 0.0 };
            if QZ != 0.0 {
            } else {
            }
            let RA = if ((QU / CB).abs()) > QV { 1.0 } else { 0.0 };
            if RA != 0.0 {
            } else {
            }
            let RB = if (FD.abs()) > QY { 1.0 } else { 0.0 };
            if RB != 0.0 {
            } else {
            }
            let RC = if EV > A { 1.0 } else { 0.0 };
            if RC != 0.0 {
                let RP = if DG != 0.0 {
                    let RD = FC + DF;
                    let RF = AX * ((FC - DF) + (((RD * RD) + RE).sqrt()));
                    RF
                } else {
                    FC
                };
                let RI = AM * RH;
                let RJ = if (AL * RG) > A { 1.0 } else { 0.0 };
                if RJ != 0.0 {
                    let RM = (-RK) * RL;
                    let RO = if RN <= A { 1.0 } else { 0.0 };
                    if RO != 0.0 {
                        let RQ = if (RP + RM) > A { 1.0 } else { 0.0 };
                        if RQ != 0.0 {
                        } else {
                        }
                    } else {
                    }
                } else {
                }
                let RR = if RI > A { 1.0 } else { 0.0 };
                if RR != 0.0 {
                    let RT = (-RS) * RL;
                    let RV = if RU <= A { 1.0 } else { 0.0 };
                    if RV != 0.0 {
                        let RW = if (RP + RT) > A { 1.0 } else { 0.0 };
                        if RW != 0.0 {
                        } else {
                        }
                    } else {
                    }
                } else {
                }
            } else {
            }
            let RX = if EW > A { 1.0 } else { 0.0 };
            if RX != 0.0 {
                let SE = if DG != 0.0 {
                    let RY = FD + DF;
                    let RZ = AX * ((FD - DF) + (((RY * RY) + RE).sqrt()));
                    RZ
                } else {
                    FD
                };
                let SA = AO * RH;
                let SB = if (AN * RG) > A { 1.0 } else { 0.0 };
                if SB != 0.0 {
                    let SC = (-RK) * RL;
                    let SD = if RN <= A { 1.0 } else { 0.0 };
                    if SD != 0.0 {
                        let SF = if (SE + SC) > A { 1.0 } else { 0.0 };
                        if SF != 0.0 {
                        } else {
                        }
                    } else {
                    }
                } else {
                }
                let SG = if SA > A { 1.0 } else { 0.0 };
                if SG != 0.0 {
                    let SH = (-RS) * RL;
                    let SI = if RU <= A { 1.0 } else { 0.0 };
                    if SI != 0.0 {
                        let SJ = if (SE + SH) > A { 1.0 } else { 0.0 };
                        if SJ != 0.0 {
                        } else {
                        }
                    } else {
                    }
                } else {
                }
            } else {
            }
            let SM = if (SK / F) <= SL { 1.0 } else { 0.0 };
            if SM != 0.0 {
            } else {
            }
            let SP = if (SO / F) <= SL { 1.0 } else { 0.0 };
            if SP != 0.0 {
            } else {
            }
            let TS;
            let TT;
            let TU;
            let TV;
            let TW;
            let TX;
            let TY;
            let TZ;
            let UA;
            let UB;
            let UD;
            let UF;
            let UH;
            if SQ != 0.0 {
                let ST;
                let SU;
                if SR != 0.0 {
                    ST = CB;
                    SU = CE;
                } else {
                    ST = AF;
                    SU = AG;
                }
                let SS = (5.522602e-23f64 * FL) * NV;
                let SV = ((HI * (((QS / ST).abs()).powf(parameters[87]))) * ST) / SU;
                let SW = if QS < A { 1.0 } else { 0.0 };
                let SY = if SW != 0.0 {
                    let SX = -SV;
                    SX
                } else {
                    SV
                };
                let TA = if SK > A { 1.0 } else { 0.0 };
                let TC = if TA != 0.0 {
                    let TB = C / (SK * SN);
                    TB
                } else {
                    A
                };
                let TD = (5.522602e-23f64 * FL) * TC;
                let TE = if SO > A { 1.0 } else { 0.0 };
                let TG = if TE != 0.0 {
                    let TF = C / (SO * SN);
                    TF
                } else {
                    A
                };
                let TH = (5.522602e-23f64 * FL) * TG;
                let UC;
                let UE;
                if NZ != 0.0 {
                    let TK = 3.20435306e-19f64 * (((TI + (AQ * GR)).abs()) + (TJ.abs()));
                    UC = C;
                    UE = TK;
                } else {
                    UC = A;
                    UE = A;
                }
                let UG;
                let UI;
                if PH != 0.0 {
                    let TN = 3.20435306e-19f64 * (((TL + (AQ * GU)).abs()) + (TM.abs()));
                    UG = C;
                    UI = TN;
                } else {
                    UG = A;
                    UI = A;
                }
                TS = C;
                TT = SS;
                TU = C;
                TV = SY;
                TW = SZ;
                TX = C;
                TY = TD;
                TZ = C;
                UA = TH;
                UB = UC;
                UD = UE;
                UF = UG;
                UH = UI;
            } else {
                TS = A;
                TT = A;
                TU = A;
                TV = A;
                TW = A;
                TX = A;
                TY = A;
                TZ = A;
                UA = A;
                UB = A;
                UD = A;
                UF = A;
                UH = A;
            }
            let TP = if QN != 0.0 {
                let TO = C / QM;
                TO
            } else {
                A
            };
            let TQ = 0e0f64;
            let TR = if ((TQ + ((0e0f64 * TP) * (QS + (FA * TQ)))).abs()) > EG { 1.0 } else { 0.0 };
            if TR != 0.0 {
            } else {
            }
        if TS == 0.0 {
            if !visitor.visit(0, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = TT;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 0, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 0, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(0, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if TU == 0.0 {
            if !visitor.visit(1, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = TV;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = Some(TW);
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(1, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if TX == 0.0 {
            if !visitor.visit(2, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = TY;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 2, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 2, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(2, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if TZ == 0.0 {
            if !visitor.visit(3, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = UA;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 3, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 3, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(3, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if UB == 0.0 {
            if !visitor.visit(4, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = UD;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 4, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 4, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(4, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if UF == 0.0 {
            if !visitor.visit(5, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = UH;
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
