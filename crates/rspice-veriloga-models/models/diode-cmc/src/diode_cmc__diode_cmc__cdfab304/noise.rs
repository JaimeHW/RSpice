#![allow(dead_code, non_snake_case, unused_parens, unused_variables)]

use super::state::Instance;
use rspice_veriloga_runtime::GeneratedEvalContext;
pub use rspice_veriloga_runtime::{GeneratedNoiseDescriptor, GeneratedNoiseEndpoint, GeneratedNoiseEvaluation, GeneratedNoiseEvaluationError, GeneratedNoiseEvaluationRef, GeneratedNoiseKind, GeneratedNoiseVisitor};

pub static NOISE_SOURCES: [GeneratedNoiseDescriptor; 3] = [
    GeneratedNoiseDescriptor { mechanism: "WHITE_A_AIK_SHOT", label: Some("shot"), kind: GeneratedNoiseKind::White, equation: 0, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(0), name: "a", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(2), name: "aik", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_A_AIK_FLICKER", label: Some("flicker"), kind: GeneratedNoiseKind::Flicker, equation: 1, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(0), name: "a", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(2), name: "aik", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_AIK_K_THERMAL", label: Some("thermal"), kind: GeneratedNoiseKind::White, equation: 2, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(2), name: "aik", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(1), name: "k", is_internal: false }, table_len: 0, table_log_interp: false },
];

impl Instance {
    pub fn evaluate_noise_sources(&self, ctx: &GeneratedEvalContext<'_>, visitor: &mut dyn GeneratedNoiseVisitor) -> Result<(), GeneratedNoiseEvaluationError> {
        if !self.multiplicity.is_finite() || self.multiplicity <= 0.0 {
            return Err(GeneratedNoiseEvaluationError::InvalidMultiplicity { value: self.multiplicity });
        }
        let parameters = &self.params.values;
        let parameter_given = &*self.param_given;
        let temperature = ctx.temperature();
        let node_potentials = [ctx.node_voltage(self.nodes[0]), ctx.node_voltage(self.nodes[1]), ctx.node_voltage(self.nodes[2]), ctx.node_voltage(self.nodes[3]), ctx.node_voltage(self.nodes[4]), ctx.node_voltage(self.nodes[5])];
            let A = 0e0f64;
            let B = 1.0447941624768001e-10f64;
            let C = parameters[6];
            let E = -2.5e2f64;
            let H = parameters[96];
            let J = -2.5e2f64;
            let L = parameters[5];
            let M = 1e-12f64;
            let P = parameters[8];
            let S = parameters[9];
            let T = 1e-18f64;
            let W = parameters[10];
            let Z = parameters[11];
            let AA = 5e-2f64;
            let AD = parameters[12];
            let AG = parameters[13];
            let AJ = parameters[14];
            let AL = 9.5e-1f64;
            let AP = parameters[15];
            let AU = parameters[16];
            let AZ = parameters[17];
            let BA = parameters[18];
            let BB = parameters[19];
            let BC = parameters[20];
            let BF = parameters[21];
            let BI = parameters[22];
            let BL = parameters[23];
            let BO = parameters[24];
            let BR = parameters[25];
            let BU = parameters[26];
            let BV = 1e-9f64;
            let BY = parameters[27];
            let CB = parameters[28];
            let CE = parameters[29];
            let CH = parameters[30];
            let CK = parameters[31];
            let CL = 1e-2f64;
            let CO = parameters[32];
            let CR = parameters[33];
            let CU = parameters[34];
            let CX = parameters[35];
            let DA = parameters[36];
            let DD = parameters[43];
            let DE = 1e-1f64;
            let DH = parameters[44];
            let DK = parameters[45];
            let DN = parameters[46];
            let DQ = parameters[47];
            let DT = parameters[48];
            let DW = parameters[7];
            let DX = parameters[49];
            let EA = parameters[50];
            let ED = parameters[51];
            let EG = parameters[52];
            let EJ = parameters[53];
            let EM = parameters[55];
            let EP = parameters[54];
            let ET = parameters[63];
            let EW = parameters[64];
            let EZ = parameters[65];
            let FC = parameters[66];
            let FF = parameters[67];
            let FI = parameters[68];
            let FL = parameters[69];
            let FO = parameters[70];
            let FR = parameters[71];
            let FU = parameters[72];
            let FW = -2.5e2f64;
            let FY = parameters[73];
            let GA = -2.5e2f64;
            let GC = parameters[74];
            let GF = parameters[75];
            let GI = parameters[76];
            let GL = parameters[77];
            let GO = parameters[78];
            let GR = 5e-1f64;
            let GT = 1e0f64;
            let GU = parameters[82];
            let GX = parameters[83];
            let HA = 2.7315e2f64;
            let HF = 1.6021918e-19f64;
            let HG = 8.61726105451295e-5f64;
            let HL = 7.02e-4f64;
            let HM = 1.108e3f64;
            let HS = 2e0f64;
            let JG = 2.9214664e-1f64;
            let JH = 5.178164370971076e-1f64;
            let JI = 3e0f64;
            let JJ = 2.6992878119627894e-1f64;
            let JK = 4.3792457880372104e-1f64;
            let JR = 3.2e1f64;
            let JS = 9.1093826e-31f64;
            let KO = 1e1f64;
            let LG = 1e6f64;
            let LK = 1e-3f64;
            let LT = parameters[94];
            let LV = parameters[99];
            let LZ = parameters[100];
            let MD = parameters[101];
            let MK = 1e8f64;
            let MW = 2.3025850929940458e2f64;
            let NA = 1e-100f64;
            let NB = 3.333333333333333e-1f64;
            let ND = 1e100f64;
            let OR = 4e0f64;
            let OV = 1e-6f64;
            let PG = 2e-1f64;
            let PQ = parameters[85];
            let PS = parameters[86];
            let ZT = 6.66666666666667e-1f64;
            let AAH = 3.75e-1f64;
            let ABX = parameters[80];
            let CGA = 1.0f64;
            let CGJ = -1.000000082740371e-11f64;
            let CZU = 1.0f64;
            let DAD = -5.000000413701855e-12f64;
            let DMC = 1e-21f64;
            let DMU = node_potentials[0];
            let EKX = parameters[91];
            let ELA = parameters[90];
            let ELD = parameters[98];
            let ELG = parameters[79];
            let ELI = parameters[92];
            let ELV = parameters[95];
            let D = if C > -2.5e2f64 { 1.0 } else { 0.0 };
            let F = if D != 0.0 {
                C
            } else {
                E
            };
            let G = if (if (if parameter_given[6] { 1.0 } else { 0.0 }) == 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if parameter_given[96] { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let HB;
            if G != 0.0 {
                let I = if H > -2.5e2f64 { 1.0 } else { 0.0 };
                let K = if I != 0.0 {
                    H
                } else {
                    J
                };
                HB = K;
            } else {
                HB = F;
            }
            let N = if L > M { 1.0 } else { 0.0 };
            let O = if N != 0.0 {
                L
            } else {
                M
            };
            let Q = if P > M { 1.0 } else { 0.0 };
            let R = if Q != 0.0 {
                P
            } else {
                M
            };
            let U = if S > T { 1.0 } else { 0.0 };
            let V = if U != 0.0 {
                S
            } else {
                T
            };
            let X = if W > T { 1.0 } else { 0.0 };
            let Y = if X != 0.0 {
                W
            } else {
                T
            };
            let AB = if Z > AA { 1.0 } else { 0.0 };
            let AC = if AB != 0.0 {
                Z
            } else {
                AA
            };
            let AE = if AD > AA { 1.0 } else { 0.0 };
            let AF = if AE != 0.0 {
                AD
            } else {
                AA
            };
            let AH = if AG > AA { 1.0 } else { 0.0 };
            let AI = if AH != 0.0 {
                AG
            } else {
                AA
            };
            let AK = if AJ > AA { 1.0 } else { 0.0 };
            let AO;
            if AK != 0.0 {
                let AM = if AJ < AL { 1.0 } else { 0.0 };
                let AN = if AM != 0.0 {
                    AJ
                } else {
                    AL
                };
                AO = AN;
            } else {
                AO = AA;
            }
            let AQ = if AP > AA { 1.0 } else { 0.0 };
            let AT;
            if AQ != 0.0 {
                let AR = if AP < AL { 1.0 } else { 0.0 };
                let AS = if AR != 0.0 {
                    AP
                } else {
                    AL
                };
                AT = AS;
            } else {
                AT = AA;
            }
            let AV = if AU > AA { 1.0 } else { 0.0 };
            let AY;
            if AV != 0.0 {
                let AW = if AU < AL { 1.0 } else { 0.0 };
                let AX = if AW != 0.0 {
                    AU
                } else {
                    AL
                };
                AY = AX;
            } else {
                AY = AA;
            }
            let BD = if BC > A { 1.0 } else { 0.0 };
            let BE = if BD != 0.0 {
                BC
            } else {
                A
            };
            let BG = if BF > A { 1.0 } else { 0.0 };
            let BH = if BG != 0.0 {
                BF
            } else {
                A
            };
            let BJ = if BI > A { 1.0 } else { 0.0 };
            let BK = if BJ != 0.0 {
                BI
            } else {
                A
            };
            let BM = if BL > A { 1.0 } else { 0.0 };
            let BN = if BM != 0.0 {
                BL
            } else {
                A
            };
            let BP = if BO > A { 1.0 } else { 0.0 };
            let BQ = if BP != 0.0 {
                BO
            } else {
                A
            };
            let BS = if BR > A { 1.0 } else { 0.0 };
            let BT = if BS != 0.0 {
                BR
            } else {
                A
            };
            let BW = if BU > BV { 1.0 } else { 0.0 };
            let BX = if BW != 0.0 {
                BU
            } else {
                BV
            };
            let BZ = if BY > BV { 1.0 } else { 0.0 };
            let CA = if BZ != 0.0 {
                BY
            } else {
                BV
            };
            let CC = if CB > A { 1.0 } else { 0.0 };
            let CD = if CC != 0.0 {
                CB
            } else {
                A
            };
            let CF = if CE > A { 1.0 } else { 0.0 };
            let CG = if CF != 0.0 {
                CE
            } else {
                A
            };
            let CI = if CH > A { 1.0 } else { 0.0 };
            let CJ = if CI != 0.0 {
                CH
            } else {
                A
            };
            let CM = if CK > CL { 1.0 } else { 0.0 };
            let CN = if CM != 0.0 {
                CK
            } else {
                CL
            };
            let CP = if CO > CL { 1.0 } else { 0.0 };
            let CQ = if CP != 0.0 {
                CO
            } else {
                CL
            };
            let CS = if CR > CL { 1.0 } else { 0.0 };
            let CT = if CS != 0.0 {
                CR
            } else {
                CL
            };
            let CV = if CU > A { 1.0 } else { 0.0 };
            let CW = if CV != 0.0 {
                CU
            } else {
                A
            };
            let CY = if CX > A { 1.0 } else { 0.0 };
            let CZ = if CY != 0.0 {
                CX
            } else {
                A
            };
            let DB = if DA > A { 1.0 } else { 0.0 };
            let DC = if DB != 0.0 {
                DA
            } else {
                A
            };
            let DF = if DD > DE { 1.0 } else { 0.0 };
            let DG = if DF != 0.0 {
                DD
            } else {
                DE
            };
            let DI = if DH > DE { 1.0 } else { 0.0 };
            let DJ = if DI != 0.0 {
                DH
            } else {
                DE
            };
            let DL = if DK > DE { 1.0 } else { 0.0 };
            let DM = if DL != 0.0 {
                DK
            } else {
                DE
            };
            let DO = if DN > DE { 1.0 } else { 0.0 };
            let DP = if DO != 0.0 {
                DN
            } else {
                DE
            };
            let DR = if DQ > DE { 1.0 } else { 0.0 };
            let DS = if DR != 0.0 {
                DQ
            } else {
                DE
            };
            let DU = if DT > DE { 1.0 } else { 0.0 };
            let DV = if DU != 0.0 {
                DT
            } else {
                DE
            };
            let DY = if DX > A { 1.0 } else { 0.0 };
            let DZ = if DY != 0.0 {
                DX
            } else {
                A
            };
            let EB = if EA > A { 1.0 } else { 0.0 };
            let EC = if EB != 0.0 {
                EA
            } else {
                A
            };
            let EE = if ED > A { 1.0 } else { 0.0 };
            let EF = if EE != 0.0 {
                ED
            } else {
                A
            };
            let EH = if EG > A { 1.0 } else { 0.0 };
            let EI = if EH != 0.0 {
                EG
            } else {
                A
            };
            let EK = if EJ > A { 1.0 } else { 0.0 };
            let EL = if EK != 0.0 {
                EJ
            } else {
                A
            };
            let EN = if EM > DE { 1.0 } else { 0.0 };
            let EO = if EN != 0.0 {
                EM
            } else {
                DE
            };
            let EQ = if EP > A { 1.0 } else { 0.0 };
            let ER = if EQ != 0.0 {
                EP
            } else {
                A
            };
            let ES = if parameters[56] > A { 1.0 } else { 0.0 };
            if ES != 0.0 {
            } else {
            }
            let EU = if ET > DE { 1.0 } else { 0.0 };
            let EV = if EU != 0.0 {
                ET
            } else {
                DE
            };
            let EX = if EW > DE { 1.0 } else { 0.0 };
            let EY = if EX != 0.0 {
                EW
            } else {
                DE
            };
            let FA = if EZ > DE { 1.0 } else { 0.0 };
            let FB = if FA != 0.0 {
                EZ
            } else {
                DE
            };
            let FD = if FC > A { 1.0 } else { 0.0 };
            let FE = if FD != 0.0 {
                FC
            } else {
                A
            };
            let FG = if FF > A { 1.0 } else { 0.0 };
            let FH = if FG != 0.0 {
                FF
            } else {
                A
            };
            let FJ = if FI > A { 1.0 } else { 0.0 };
            let FK = if FJ != 0.0 {
                FI
            } else {
                A
            };
            let FM = if FL > A { 1.0 } else { 0.0 };
            let FN = if FM != 0.0 {
                FL
            } else {
                A
            };
            let FP = if FO > A { 1.0 } else { 0.0 };
            let FQ = if FP != 0.0 {
                FO
            } else {
                A
            };
            let FS = if FR > A { 1.0 } else { 0.0 };
            let FT = if FS != 0.0 {
                FR
            } else {
                A
            };
            let FV = if FU > -2.5e2f64 { 1.0 } else { 0.0 };
            let FX = if FV != 0.0 {
                FU
            } else {
                FW
            };
            let FZ = if FY > -2.5e2f64 { 1.0 } else { 0.0 };
            let GB = if FZ != 0.0 {
                FY
            } else {
                GA
            };
            let GD = if GC > A { 1.0 } else { 0.0 };
            let GE = if GD != 0.0 {
                GC
            } else {
                A
            };
            let GG = if GF > A { 1.0 } else { 0.0 };
            let GH = if GG != 0.0 {
                GF
            } else {
                A
            };
            let GJ = if GI > DE { 1.0 } else { 0.0 };
            let GK = if GJ != 0.0 {
                GI
            } else {
                DE
            };
            let GM = if GL > A { 1.0 } else { 0.0 };
            let GN = if GM != 0.0 {
                GL
            } else {
                A
            };
            let GP = if GO > A { 1.0 } else { 0.0 };
            let GQ = if GP != 0.0 {
                GO
            } else {
                A
            };
            let GS = if parameters[81] > GR { 1.0 } else { 0.0 };
            let OT = if GS != 0.0 {
                GT
            } else {
                A
            };
            let GV = if GU > GR { 1.0 } else { 0.0 };
            let GW = if GV != 0.0 {
                GU
            } else {
                GR
            };
            let GY = if GX > A { 1.0 } else { 0.0 };
            let GZ = if GY != 0.0 {
                GX
            } else {
                A
            };
            let HC = HA + HB;
            let HD = if (temperature + parameters[102]) >= 2.3149999999999977e1f64 { (temperature + parameters[102]) } else { 2.3149999999999977e1f64 };
            let HE = HD / HC;
            let HH = HG * HC;
            let HI = GT / HH;
            let HJ = HG * HD;
            let HK = GT / HJ;
            let HN = (-((HL * HC) * HC)) / (HM + HC);
            let HO = (-((HL * HD) * HD)) / (HM + HD);
            let HP = AZ + HO;
            let HQ = BA + HO;
            let HR = BB + HO;
            let HT = GK / HS;
            let HU = HE.powf(HT);
            let HV = GR * (((AZ + HN) * HI) - (HP * HK));
            let HW = HU * (HV.exp());
            let HX = GR * (((BA + HN) * HI) - (HQ * HK));
            let HY = HU * (HX.exp());
            let HZ = GR * (((BB + HN) * HI) - (HR * HK));
            let IA = HU * (HZ.exp());
            let IB = (HE.powf((HT / EV))) * ((HV / EV).exp());
            let IC = (HE.powf((HT / EY))) * ((HX / EY).exp());
            let ID = (HE.powf((HT / FB))) * ((HZ / FB).exp());
            let IE = (BE * IB) * IB;
            let IF = (BH * IC) * IC;
            let IG = (BK * ID) * ID;
            let IH = HS * HJ;
            let II = (AC * HE) - (IH * (HW.ln()));
            let IJ = (AF * HE) - (IH * (HY.ln()));
            let IK = (AI * HE) - (IH * (IA.ln()));
            let IL = II + (HJ * ((GT + (((AA - II) * HK).exp())).ln()));
            let IM = IJ + (HJ * ((GT + (((AA - IJ) * HK).exp())).ln()));
            let IN = IK + (HJ * ((GT + (((AA - IK) * HK).exp())).ln()));
            let IO = GT - AO;
            let IP = GT - AT;
            let IQ = GT - AY;
            let IR = GT / IO;
            let IS = GT / IP;
            let IT = GT / IQ;
            let IU = R * ((AC * (GT / IL)).powf(AO));
            let IV = V * ((AF * (GT / IM)).powf(AT));
            let IW = Y * ((AI * (GT / IN)).powf(AY));
            let IX = B / R;
            let IY = (BX * B) / V;
            let IZ = (CA * B) / Y;
            let JA = GT / IX;
            let JB = GT / IY;
            let JC = GT / IZ;
            let JD = GT / AC;
            let JE = GT / AF;
            let JF = GT / AI;
            let JL = if (GR * HP) >= HJ { (GR * HP) } else { HJ };
            let JM = if (GR * HQ) >= HJ { (GR * HQ) } else { HJ };
            let JN = if (GR * HR) >= HJ { (GR * HR) } else { HJ };
            let JO = JL * HK;
            let JP = JM * HK;
            let JQ = JN * HK;
            let JT = (((((JR * CN) * JS) * HF) * ((JL * JL) * JL)).sqrt()) / 3.1637150399999996e-34f64;
            let JU = (((((JR * CQ) * JS) * HF) * ((JM * JM) * JM)).sqrt()) / 3.1637150399999996e-34f64;
            let JV = (((((JR * CT) * JS) * HF) * ((JN * JN) * JN)).sqrt()) / 3.1637150399999996e-34f64;
            let JW = HD - HC;
            let JX = parameters[37] * (GT + (parameters[40] * JW));
            let JY = parameters[38] * (GT + (parameters[41] * JW));
            let JZ = parameters[39] * (GT + (parameters[42] * JW));
            let KA = if JX > A { 1.0 } else { 0.0 };
            let KB = if KA != 0.0 {
                JX
            } else {
                A
            };
            let KC = if JY > A { 1.0 } else { 0.0 };
            let KD = if KC != 0.0 {
                JY
            } else {
                A
            };
            let KE = if JZ > A { 1.0 } else { 0.0 };
            let KF = if KE != 0.0 {
                JZ
            } else {
                A
            };
            let KG = (DW - GT) / DW;
            let KH = GT / (GT - (KG.powf(DP)));
            let KI = GT / (GT - (KG.powf(DS)));
            let KJ = GT / (GT - (KG.powf(DV)));
            let KK = DG * (GT + (JW * (parameters[57] + (JW * parameters[58]))));
            let KL = DJ * (GT + (JW * (parameters[59] + (JW * parameters[60]))));
            let KM = DM * (GT + (JW * (parameters[61] + (JW * parameters[62]))));
            let KN = if KK <= DE { 1.0 } else { 0.0 };
            let KV;
            let ABW;
            if KN != 0.0 {
                KV = KO;
                ABW = DE;
            } else {
                let KP = GT / KK;
                KV = KP;
                ABW = KK;
            }
            let KQ = if KL <= DE { 1.0 } else { 0.0 };
            let KX;
            let AFI;
            if KQ != 0.0 {
                KX = KO;
                AFI = DE;
            } else {
                let KR = GT / KL;
                KX = KR;
                AFI = KL;
            }
            let KS = if KM <= DE { 1.0 } else { 0.0 };
            let KZ;
            let AIS;
            if KS != 0.0 {
                KZ = KO;
                AIS = DE;
            } else {
                let KT = GT / KM;
                KZ = KT;
                AIS = KM;
            }
            let KU = GT - (CL * GQ);
            let KW = ((-((KH * KH) * (KG.powf((DP - GT))))) * DP) * KV;
            let KY = ((-((KI * KI) * (KG.powf((DS - GT))))) * DS) * KX;
            let LA = ((-((KJ * KJ) * (KG.powf((DV - GT))))) * DV) * KZ;
            let LB = HE.powf(EL);
            let LC = DZ * LB;
            let LD = EF * LB;
            let LE = EC * LB;
            let LF = EI * LB;
            let LH = parameters[87] * LG;
            let LI = parameters[89] * LG;
            let LJ = parameters[88] * LG;
            let LL = 1.45e16f64 * IB;
            let LM = LL * LL;
            let LN = HE.powf(-1.5e0f64);
            let LO = (1.4500000000000002e-1f64 * LN) / HK;
            let LP = (5e-2f64 * LN) / HK;
            let LQ = EV / HK;
            let LR = (LH / (LM / LH)).ln();
            let LS = LQ * LR;
            let LU = LQ * (LR + (LT / (((parameters[93] * (HE.powf(parameters[97]))) * (((HS * LO) * LP) / (LO + LP))).sqrt())));
            let LW = if LV > A { 1.0 } else { 0.0 };
            let LX = if LW != 0.0 {
                LV
            } else {
                A
            };
            let LY = (((LX * GN) * GN) * KU) * KU;
            let MA = if LZ > A { 1.0 } else { 0.0 };
            let MB = if MA != 0.0 {
                LZ
            } else {
                A
            };
            let MC = (MB * GN) * KU;
            let ME = if MD > A { 1.0 } else { 0.0 };
            let MF = if ME != 0.0 {
                MD
            } else {
                A
            };
            let MG = (MF * GN) * KU;
            let MH = IE * LY;
            let MI = if MH > A { 1.0 } else { 0.0 };
            let MR = if MI != 0.0 {
                let MJ = (HJ * (((O / MH) + GT).ln())) * EV;
                MJ
            } else {
                MK
            };
            let ML = IF * MC;
            let MM = if ML > A { 1.0 } else { 0.0 };
            let MS = if MM != 0.0 {
                let MN = (HJ * (((O / ML) + GT).ln())) * EY;
                MN
            } else {
                MK
            };
            let MO = IG * MG;
            let MP = if MO > A { 1.0 } else { 0.0 };
            let MT = if MP != 0.0 {
                let MQ = (HJ * (((O / MO) + GT).ln())) * FB;
                MQ
            } else {
                MK
            };
            let MU = if (if MR <= MS { MR } else { MS }) <= MT { (if MR <= MS { MR } else { MS }) } else { MT };
            let MV = MU * HK;
            let MX = if (MV.abs()) < MW { 1.0 } else { 0.0 };
            let TE;
            if MX != 0.0 {
                let MY = MV.exp();
                TE = MY;
            } else {
                let MZ = if MV < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                let TF = if MZ != 0.0 {
                    let NC = NA / (GT + ((-2.3025850929940458e2f64 - MV) * (GT + (GR * ((-2.3025850929940458e2f64 - MV) * (GT + ((-2.3025850929940458e2f64 - MV) * NB)))))));
                    NC
                } else {
                    let NE = MV - MW;
                    let NF = ND * (GT + (NE * (GT + (GR * (NE * (GT + (NE * NB)))))));
                    NF
                };
                TE = TF;
            }
            let NG = if LY == A { 1.0 } else { 0.0 };
            let NP;
            let NT;
            if NG != 0.0 {
                let NH = IM + IN;
                let NI = AF + AI;
                NP = NH;
                NT = NI;
            } else {
                NP = IL;
                NT = AC;
            }
            let NJ = if MC == A { 1.0 } else { 0.0 };
            let NQ;
            let NU;
            if NJ != 0.0 {
                let NK = IL + IN;
                let NL = AC + AI;
                NQ = NK;
                NU = NL;
            } else {
                NQ = IM;
                NU = AF;
            }
            let NM = if MG == A { 1.0 } else { 0.0 };
            let NR;
            let NV;
            if NM != 0.0 {
                let NN = IL + IM;
                let NO = AC + AF;
                NR = NN;
                NV = NO;
            } else {
                NR = IN;
                NV = AI;
            }
            let NS = if (if NP <= NQ { NP } else { NQ }) <= NR { (if NP <= NQ { NP } else { NQ }) } else { NR };
            let NW = (if (if NT <= NU { NT } else { NU }) <= NV { (if NT <= NU { NT } else { NU }) } else { NV }) - AA;
            let NX = if (if LY > FH { 1.0 } else { 0.0 }) != 0.0 && (if FH > T { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            if NX != 0.0 {
            } else {
            }
            let NY = if LY < FE { 1.0 } else { 0.0 };
            if NY != 0.0 {
            } else {
            }
            let NZ = if (if MC > FN { 1.0 } else { 0.0 }) != 0.0 && (if FN > M { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            if NZ != 0.0 {
            } else {
            }
            let OA = if MC < FK { 1.0 } else { 0.0 };
            if OA != 0.0 {
            } else {
            }
            let OB = if (if MG > FT { 1.0 } else { 0.0 }) != 0.0 && (if FT > M { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            if OB != 0.0 {
            } else {
            }
            let OC = if MG < FQ { 1.0 } else { 0.0 };
            if OC != 0.0 {
            } else {
            }
            let OD = if HD > (GB + HA) { 1.0 } else { 0.0 };
            if OD != 0.0 {
            } else {
            }
            let OE = if HD < (FX + HA) { 1.0 } else { 0.0 };
            if OE != 0.0 {
            } else {
            }
            let OF = (MH + ML) + MO;
            let OG = if (LY * LC) > A { 1.0 } else { 0.0 };
            let OJ = if OG != 0.0 {
                let OH = LY / LC;
                OH
            } else {
                A
            };
            let OI = if (MC * LE) > A { 1.0 } else { 0.0 };
            let OM = if OI != 0.0 {
                let OK = (MC / LE) + OJ;
                OK
            } else {
                OJ
            };
            let OL = if (MG * LD) > A { 1.0 } else { 0.0 };
            let OO = if OL != 0.0 {
                let ON = (MG / LD) + OM;
                ON
            } else {
                OM
            };
            let OP = if OO > A { 1.0 } else { 0.0 };
            let EMB = if OP != 0.0 {
                let OQ = (GT / OO) + LF;
                OQ
            } else {
                LF
            };
            let OS = if ((OR * LT) * 1e-7f64) > A { 1.0 } else { 0.0 };
            if OS != 0.0 {
            } else {
            }
            let OU = if OT > 9e-1f64 { 1.0 } else { 0.0 };
            let PA;
            let DKP;
            if OU != 0.0 {
                let OW = if LY > A { 1.0 } else { 0.0 };
                let OX = if MG > A { 1.0 } else { 0.0 };
                let OY = if MC > A { 1.0 } else { 0.0 };
                let OZ = if (if (if (if (if ((EV - FB).abs()) > OV { 1.0 } else { 0.0 }) != 0.0 && OW != 0.0 { 1.0 } else { 0.0 }) != 0.0 && OX != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if (if (if ((EV - EY).abs()) > OV { 1.0 } else { 0.0 }) != 0.0 && OW != 0.0 { 1.0 } else { 0.0 }) != 0.0 && OY != 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if (if (if ((FB - EY).abs()) > OV { 1.0 } else { 0.0 }) != 0.0 && OX != 0.0 { 1.0 } else { 0.0 }) != 0.0 && OY != 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let PB;
                let DKQ;
                if OZ != 0.0 {
                    PB = A;
                    DKQ = GT;
                } else {
                    let DKT = if OW != 0.0 {
                        EV
                    } else {
                        GT
                    };
                    let DKS = if OX != 0.0 {
                        FB
                    } else {
                        DKT
                    };
                    let DKR = if OY != 0.0 {
                        EY
                    } else {
                        DKS
                    };
                    PB = OT;
                    DKQ = DKR;
                }
                PA = PB;
                DKP = DKQ;
            } else {
                PA = OT;
                DKP = GT;
            }
            let PC = if PA == GT { 1.0 } else { 0.0 };
            let DNA;
            let DNC;
            let DNH;
            let DNJ;
            let DNO;
            let DNQ;
            let DNV;
            let DNX;
            let DOD;
            let DOE;
            let DON;
            let DOP;
            let DOY;
            let DPC;
            let DPG;
            let EKL;
            if PC != 0.0 {
                let PD = -4e-1f64 * GW;
                let PE = -6.5e-1f64 * GW;
                let PF = -8e-1f64 * GW;
                let PH = if (if (if NG != 0.0 && NJ != 0.0 { 1.0 } else { 0.0 }) != 0.0 && NM != 0.0 { 1.0 } else { 0.0 }) == 0.0 { 1.0 } else { 0.0 };
                let YX;
                let ZB;
                let ZD;
                let ZN;
                let ABH;
                let ABZ;
                let ACP;
                let AFZ;
                if PH != 0.0 {
                    let PI = if PD < MU { 1.0 } else { 0.0 };
                    let XW;
                    let YA;
                    let YE;
                    let YI;
                    if PI != 0.0 {
                        let PJ = GR * (PD * HK);
                        let PK = if (PJ.abs()) < MW { 1.0 } else { 0.0 };
                        let YJ;
                        if PK != 0.0 {
                            let PL = PJ.exp();
                            YJ = PL;
                        } else {
                            let PM = if PJ < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            let YK = if PM != 0.0 {
                                let PN = NA / (GT + ((-2.3025850929940458e2f64 - PJ) * (GT + (GR * ((-2.3025850929940458e2f64 - PJ) * (GT + ((-2.3025850929940458e2f64 - PJ) * NB)))))));
                                PN
                            } else {
                                let PO = PJ - MW;
                                let PP = ND * (GT + (PO * (GT + (GR * (PO * (GT + (PO * NB)))))));
                                PP
                            };
                            YJ = YK;
                        }
                        let PR = if EV < PQ { 1.0 } else { 0.0 };
                        let QM;
                        let QN;
                        if PR != 0.0 {
                            let PT = EV - (PS * LS);
                            let PU = (PQ - ((PS * (PD - LS)) + EV)) - CL;
                            let PV = (OR * PQ) * CL;
                            let PW = if PV > A { 1.0 } else { 0.0 };
                            let PY = if PW != 0.0 {
                                PV
                            } else {
                                let PX = -PV;
                                PX
                            };
                            let PZ = ((PQ - (GR * (PU + (((PU * PU) + PY).sqrt())))) - EV) - CL;
                            let QA = (OR * EV) * CL;
                            let QB = if QA > A { 1.0 } else { 0.0 };
                            let QD = if QB != 0.0 {
                                QA
                            } else {
                                let QC = -QA;
                                QC
                            };
                            let QE = EV + (GR * (PZ + (((PZ * PZ) + QD).sqrt())));
                            let QF = (PQ - PT) - CL;
                            let QH = if PW != 0.0 {
                                PV
                            } else {
                                let QG = -PV;
                                QG
                            };
                            let QI = ((PQ - (GR * (QF + (((QF * QF) + QH).sqrt())))) - EV) - CL;
                            let QK = if QB != 0.0 {
                                QA
                            } else {
                                let QJ = -QA;
                                QJ
                            };
                            let QL = EV + (GR * (QI + (((QI * QI) + QK).sqrt())));
                            QM = QE;
                            QN = QL;
                        } else {
                            QM = EV;
                            QN = EV;
                        }
                        let QO = HK * ((PD / QM) + ((LS * (QM - QN)) / (QN * PQ)));
                        let QP = if (QO.abs()) < MW { 1.0 } else { 0.0 };
                        let XX;
                        if QP != 0.0 {
                            let QQ = QO.exp();
                            XX = QQ;
                        } else {
                            let QR = if QO < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            let XY = if QR != 0.0 {
                                let QS = NA / (GT + ((-2.3025850929940458e2f64 - QO) * (GT + (GR * ((-2.3025850929940458e2f64 - QO) * (GT + ((-2.3025850929940458e2f64 - QO) * NB)))))));
                                QS
                            } else {
                                let QT = QO - MW;
                                let QU = ND * (GT + (QT * (GT + (GR * (QT * (GT + (QT * NB)))))));
                                QU
                            };
                            XX = XY;
                        }
                        let QV = (EY / HK) * ((LI / (LM / LI)).ln());
                        let QW = if EY < PQ { 1.0 } else { 0.0 };
                        let RQ;
                        let RR;
                        if QW != 0.0 {
                            let QX = EY - (PS * QV);
                            let QY = (PQ - ((PS * (PD - QV)) + EY)) - CL;
                            let QZ = (OR * PQ) * CL;
                            let RA = if QZ > A { 1.0 } else { 0.0 };
                            let RC = if RA != 0.0 {
                                QZ
                            } else {
                                let RB = -QZ;
                                RB
                            };
                            let RD = ((PQ - (GR * (QY + (((QY * QY) + RC).sqrt())))) - EY) - CL;
                            let RE = (OR * EY) * CL;
                            let RF = if RE > A { 1.0 } else { 0.0 };
                            let RH = if RF != 0.0 {
                                RE
                            } else {
                                let RG = -RE;
                                RG
                            };
                            let RI = EY + (GR * (RD + (((RD * RD) + RH).sqrt())));
                            let RJ = (PQ - QX) - CL;
                            let RL = if RA != 0.0 {
                                QZ
                            } else {
                                let RK = -QZ;
                                RK
                            };
                            let RM = ((PQ - (GR * (RJ + (((RJ * RJ) + RL).sqrt())))) - EY) - CL;
                            let RO = if RF != 0.0 {
                                RE
                            } else {
                                let RN = -RE;
                                RN
                            };
                            let RP = EY + (GR * (RM + (((RM * RM) + RO).sqrt())));
                            RQ = RI;
                            RR = RP;
                        } else {
                            RQ = EY;
                            RR = EY;
                        }
                        let RS = HK * ((PD / RQ) + ((QV * (RQ - RR)) / (RR * PQ)));
                        let RT = if (RS.abs()) < MW { 1.0 } else { 0.0 };
                        let YB;
                        if RT != 0.0 {
                            let RU = RS.exp();
                            YB = RU;
                        } else {
                            let RV = if RS < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            let YC = if RV != 0.0 {
                                let RW = NA / (GT + ((-2.3025850929940458e2f64 - RS) * (GT + (GR * ((-2.3025850929940458e2f64 - RS) * (GT + ((-2.3025850929940458e2f64 - RS) * NB)))))));
                                RW
                            } else {
                                let RX = RS - MW;
                                let RY = ND * (GT + (RX * (GT + (GR * (RX * (GT + (RX * NB)))))));
                                RY
                            };
                            YB = YC;
                        }
                        let RZ = (FB / HK) * ((LJ / (LM / LJ)).ln());
                        let SA = if FB < PQ { 1.0 } else { 0.0 };
                        let SU;
                        let SV;
                        if SA != 0.0 {
                            let SB = FB - (PS * RZ);
                            let SC = (PQ - ((PS * (PD - RZ)) + FB)) - CL;
                            let SD = (OR * PQ) * CL;
                            let SE = if SD > A { 1.0 } else { 0.0 };
                            let SG = if SE != 0.0 {
                                SD
                            } else {
                                let SF = -SD;
                                SF
                            };
                            let SH = ((PQ - (GR * (SC + (((SC * SC) + SG).sqrt())))) - FB) - CL;
                            let SI = (OR * FB) * CL;
                            let SJ = if SI > A { 1.0 } else { 0.0 };
                            let SL = if SJ != 0.0 {
                                SI
                            } else {
                                let SK = -SI;
                                SK
                            };
                            let SM = FB + (GR * (SH + (((SH * SH) + SL).sqrt())));
                            let SN = (PQ - SB) - CL;
                            let SP = if SE != 0.0 {
                                SD
                            } else {
                                let SO = -SD;
                                SO
                            };
                            let SQ = ((PQ - (GR * (SN + (((SN * SN) + SP).sqrt())))) - FB) - CL;
                            let SS = if SJ != 0.0 {
                                SI
                            } else {
                                let SR = -SI;
                                SR
                            };
                            let ST = FB + (GR * (SQ + (((SQ * SQ) + SS).sqrt())));
                            SU = SM;
                            SV = ST;
                        } else {
                            SU = FB;
                            SV = FB;
                        }
                        let SW = HK * ((PD / SU) + ((RZ * (SU - SV)) / (SV * PQ)));
                        let SX = if (SW.abs()) < MW { 1.0 } else { 0.0 };
                        let YF;
                        if SX != 0.0 {
                            let SY = SW.exp();
                            YF = SY;
                        } else {
                            let SZ = if SW < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            let YG = if SZ != 0.0 {
                                let TA = NA / (GT + ((-2.3025850929940458e2f64 - SW) * (GT + (GR * ((-2.3025850929940458e2f64 - SW) * (GT + ((-2.3025850929940458e2f64 - SW) * NB)))))));
                                TA
                            } else {
                                let TB = SW - MW;
                                let TC = ND * (GT + (TB * (GT + (GR * (TB * (GT + (TB * NB)))))));
                                TC
                            };
                            YF = YG;
                        }
                        XW = XX;
                        YA = YB;
                        YE = YF;
                        YI = YJ;
                    } else {
                        let TD = PD - MU;
                        let TG = ((GT + (TD * HK)) * TE).sqrt();
                        let TH = if EV < PQ { 1.0 } else { 0.0 };
                        let UG;
                        let UH;
                        let UQ;
                        if TH != 0.0 {
                            let TI = EV - (PS * LS);
                            let TJ = (PQ - ((PS * (MU - LS)) + EV)) - CL;
                            let TK = (OR * PQ) * CL;
                            let TL = if TK > A { 1.0 } else { 0.0 };
                            let TN = if TL != 0.0 {
                                TK
                            } else {
                                let TM = -TK;
                                TM
                            };
                            let TO = ((TJ * TJ) + TN).sqrt();
                            let TP = GR * (GT + (TJ / TO));
                            let TQ = ((PQ - (GR * (TJ + TO))) - EV) - CL;
                            let TR = (OR * EV) * CL;
                            let TS = if TR > A { 1.0 } else { 0.0 };
                            let TU = if TS != 0.0 {
                                TR
                            } else {
                                let TT = -TR;
                                TT
                            };
                            let TV = ((TQ * TQ) + TU).sqrt();
                            let TW = GR * (GT + (TQ / TV));
                            let TX = EV + (GR * (TQ + TV));
                            let TY = (PQ - TI) - CL;
                            let UA = if TL != 0.0 {
                                TK
                            } else {
                                let TZ = -TK;
                                TZ
                            };
                            let UB = ((PQ - (GR * (TY + (((TY * TY) + UA).sqrt())))) - EV) - CL;
                            let UD = if TS != 0.0 {
                                TR
                            } else {
                                let UC = -TR;
                                UC
                            };
                            let UE = EV + (GR * (UB + (((UB * UB) + UD).sqrt())));
                            let UF = (PS * TP) * TW;
                            UG = TX;
                            UH = UE;
                            UQ = UF;
                        } else {
                            UG = EV;
                            UH = EV;
                            UQ = A;
                        }
                        let UI = UH * PQ;
                        let UJ = HK * ((MU / UG) + ((LS * (UG - UH)) / UI));
                        let UK = if (UJ.abs()) < MW { 1.0 } else { 0.0 };
                        let UR;
                        if UK != 0.0 {
                            let UL = UJ.exp();
                            UR = UL;
                        } else {
                            let UM = if UJ < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            let US = if UM != 0.0 {
                                let UN = NA / (GT + ((-2.3025850929940458e2f64 - UJ) * (GT + (GR * ((-2.3025850929940458e2f64 - UJ) * (GT + ((-2.3025850929940458e2f64 - UJ) * NB)))))));
                                UN
                            } else {
                                let UO = UJ - MW;
                                let UP = ND * (GT + (UO * (GT + (GR * (UO * (GT + (UO * NB)))))));
                                UP
                            };
                            UR = US;
                        }
                        let UT = (GT + (TD * (HK * (((UG - (MU * UQ)) / (UG * UG)) + ((LS * UQ) / UI))))) * UR;
                        let UU = (EY / HK) * ((LI / (LM / LI)).ln());
                        let UV = if EY < PQ { 1.0 } else { 0.0 };
                        let VU;
                        let VV;
                        let WE;
                        if UV != 0.0 {
                            let UW = EY - (PS * UU);
                            let UX = (PQ - ((PS * (MU - UU)) + EY)) - CL;
                            let UY = (OR * PQ) * CL;
                            let UZ = if UY > A { 1.0 } else { 0.0 };
                            let VB = if UZ != 0.0 {
                                UY
                            } else {
                                let VA = -UY;
                                VA
                            };
                            let VC = ((UX * UX) + VB).sqrt();
                            let VD = GR * (GT + (UX / VC));
                            let VE = ((PQ - (GR * (UX + VC))) - EY) - CL;
                            let VF = (OR * EY) * CL;
                            let VG = if VF > A { 1.0 } else { 0.0 };
                            let VI = if VG != 0.0 {
                                VF
                            } else {
                                let VH = -VF;
                                VH
                            };
                            let VJ = ((VE * VE) + VI).sqrt();
                            let VK = GR * (GT + (VE / VJ));
                            let VL = EY + (GR * (VE + VJ));
                            let VM = (PQ - UW) - CL;
                            let VO = if UZ != 0.0 {
                                UY
                            } else {
                                let VN = -UY;
                                VN
                            };
                            let VP = ((PQ - (GR * (VM + (((VM * VM) + VO).sqrt())))) - EY) - CL;
                            let VR = if VG != 0.0 {
                                VF
                            } else {
                                let VQ = -VF;
                                VQ
                            };
                            let VS = EY + (GR * (VP + (((VP * VP) + VR).sqrt())));
                            let VT = (PS * VD) * VK;
                            VU = VL;
                            VV = VS;
                            WE = VT;
                        } else {
                            VU = EY;
                            VV = EY;
                            WE = A;
                        }
                        let VW = VV * PQ;
                        let VX = HK * ((MU / VU) + ((UU * (VU - VV)) / VW));
                        let VY = if (VX.abs()) < MW { 1.0 } else { 0.0 };
                        let WF;
                        if VY != 0.0 {
                            let VZ = VX.exp();
                            WF = VZ;
                        } else {
                            let WA = if VX < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            let WG = if WA != 0.0 {
                                let WB = NA / (GT + ((-2.3025850929940458e2f64 - VX) * (GT + (GR * ((-2.3025850929940458e2f64 - VX) * (GT + ((-2.3025850929940458e2f64 - VX) * NB)))))));
                                WB
                            } else {
                                let WC = VX - MW;
                                let WD = ND * (GT + (WC * (GT + (GR * (WC * (GT + (WC * NB)))))));
                                WD
                            };
                            WF = WG;
                        }
                        let WH = (GT + (TD * (HK * (((VU - (MU * WE)) / (VU * VU)) + ((UU * WE) / VW))))) * WF;
                        let WI = (FB / HK) * ((LJ / (LM / LJ)).ln());
                        let WJ = if FB < PQ { 1.0 } else { 0.0 };
                        let XI;
                        let XJ;
                        let XS;
                        if WJ != 0.0 {
                            let WK = FB - (PS * WI);
                            let WL = (PQ - ((PS * (MU - WI)) + FB)) - CL;
                            let WM = (OR * PQ) * CL;
                            let WN = if WM > A { 1.0 } else { 0.0 };
                            let WP = if WN != 0.0 {
                                WM
                            } else {
                                let WO = -WM;
                                WO
                            };
                            let WQ = ((WL * WL) + WP).sqrt();
                            let WR = GR * (GT + (WL / WQ));
                            let WS = ((PQ - (GR * (WL + WQ))) - FB) - CL;
                            let WT = (OR * FB) * CL;
                            let WU = if WT > A { 1.0 } else { 0.0 };
                            let WW = if WU != 0.0 {
                                WT
                            } else {
                                let WV = -WT;
                                WV
                            };
                            let WX = ((WS * WS) + WW).sqrt();
                            let WY = GR * (GT + (WS / WX));
                            let WZ = FB + (GR * (WS + WX));
                            let XA = (PQ - WK) - CL;
                            let XC = if WN != 0.0 {
                                WM
                            } else {
                                let XB = -WM;
                                XB
                            };
                            let XD = ((PQ - (GR * (XA + (((XA * XA) + XC).sqrt())))) - FB) - CL;
                            let XF = if WU != 0.0 {
                                WT
                            } else {
                                let XE = -WT;
                                XE
                            };
                            let XG = FB + (GR * (XD + (((XD * XD) + XF).sqrt())));
                            let XH = (PS * WR) * WY;
                            XI = WZ;
                            XJ = XG;
                            XS = XH;
                        } else {
                            XI = FB;
                            XJ = FB;
                            XS = A;
                        }
                        let XK = XJ * PQ;
                        let XL = HK * ((MU / XI) + ((WI * (XI - XJ)) / XK));
                        let XM = if (XL.abs()) < MW { 1.0 } else { 0.0 };
                        let XT;
                        if XM != 0.0 {
                            let XN = XL.exp();
                            XT = XN;
                        } else {
                            let XO = if XL < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            let XU = if XO != 0.0 {
                                let XP = NA / (GT + ((-2.3025850929940458e2f64 - XL) * (GT + (GR * ((-2.3025850929940458e2f64 - XL) * (GT + ((-2.3025850929940458e2f64 - XL) * NB)))))));
                                XP
                            } else {
                                let XQ = XL - MW;
                                let XR = ND * (GT + (XQ * (GT + (GR * (XQ * (GT + (XQ * NB)))))));
                                XR
                            };
                            XT = XU;
                        }
                        let XV = (GT + (TD * (HK * (((XI - (MU * XS)) / (XI * XI)) + ((WI * XS) / XK))))) * XT;
                        XW = UT;
                        YA = WH;
                        YE = XV;
                        YI = TG;
                    }
                    let XZ = XW - GT;
                    let YD = YA - GT;
                    let YH = YE - GT;
                    let YL = GT / YI;
                    let YM = if PD > A { 1.0 } else { 0.0 };
                    let YP = if YM != 0.0 {
                        let YN = HS * (HJ * (((HS + YL) + (((YL + GT) * (YL + JI)).sqrt())).ln()));
                        YN
                    } else {
                        let YO = (-PD) + (HS * (HJ * ((((HS * YI) + GT) + (((GT + YI) * (GT + (JI * YI))).sqrt())).ln())));
                        YO
                    };
                    let YQ = NS - YP;
                    let YR = PD - YQ;
                    let YS = GR * ((PD + YQ) - (((YR * YR) + ((OR * HJ) * HJ)).sqrt()));
                    let YT = PD - NW;
                    let YU = GR * ((PD + NW) - (((YT * YT) + ((OR * HH) * HH)).sqrt()));
                    let YV = GR * (PD - (((PD * PD) + 4e-12f64).sqrt()));
                    YX = XZ;
                    ZB = YS;
                    ZD = YP;
                    ZN = YI;
                    ABH = YU;
                    ABZ = YV;
                    ACP = YD;
                    AFZ = YH;
                } else {
                    YX = A;
                    ZB = A;
                    ZD = A;
                    ZN = A;
                    ABH = A;
                    ABZ = A;
                    ACP = A;
                    AFZ = A;
                }
                let AJI;
                if NG != 0.0 {
                    AJI = A;
                } else {
                    let YW = if IO == GR { 1.0 } else { 0.0 };
                    if YW != 0.0 {
                    } else {
                    }
                    let YY = IE * YX;
                    let YZ = if CD == A { 1.0 } else { 0.0 };
                    let ZA = if (if BN == A { 1.0 } else { 0.0 }) != 0.0 && YZ != 0.0 { 1.0 } else { 0.0 };
                    let ZQ;
                    let ZR;
                    let AAE;
                    let ABD;
                    let ACI;
                    if ZA != 0.0 {
                        ZQ = A;
                        ZR = A;
                        AAE = A;
                        ABD = A;
                        ACI = A;
                    } else {
                        let ZC = IL - ZB;
                        let ZE = GT - ((GT - (ZD / ZC)).sqrt());
                        let ZF = if AO == GR { 1.0 } else { 0.0 };
                        let ZH = if ZF != 0.0 {
                            A
                        } else {
                            let ZG = ((((ZE * ZE) * (ZE.ln())) / (GT - ZE)) + ZE) * (GT - (HS * AO));
                            ZG
                        };
                        let ZI = ZE + ZH;
                        let ZL = if ZF != 0.0 {
                            let ZJ = (ZC * JD).sqrt();
                            ZJ
                        } else {
                            let ZK = (ZC * JD).powf(AO);
                            ZK
                        };
                        let ZM = IX * ZL;
                        let ZO = HW * ((ZN - GT) * ZM);
                        let ZP = BN * (ZO * ZI);
                        ZQ = ZM;
                        ZR = ZC;
                        AAE = ZI;
                        ABD = ZO;
                        ACI = ZP;
                    }
                    let ACJ;
                    if YZ != 0.0 {
                        ACJ = A;
                    } else {
                        let ZS = JT * ((ZQ * IO) / ZR);
                        let ZU = (ZT * JO) / ZS;
                        let ZV = ZU * ZU;
                        let ZW = ZV * ZV;
                        let ZX = (ZW / (ZW + GT)).sqrt();
                        let ZY = (ZX.abs()).sqrt();
                        let ZZ = ZX * ZY;
                        let AAA = (-AO) * IR;
                        let AAB = if AAA == -1e0f64 { 1.0 } else { 0.0 };
                        let AAF = if AAB != 0.0 {
                            let AAC = GT / (GT + (ZS * ZZ));
                            AAC
                        } else {
                            let AAD = (GT + (ZS * ZZ)).powf(AAA);
                            AAD
                        };
                        let AAG = (AAE * AAF) / (AAE + AAF);
                        let AAI = (AAH * (ZS / ZY)).sqrt();
                        let AAJ = (((JO * ZU) * ZY) - (JO * ZX)) + (GR * (ZS * ZZ));
                        let AAK = (((HS * (ZU * ZY)) - ZX) - GT) * AAI;
                        let AAL = AAK * AAK;
                        let AAM = if AAK > A { 1.0 } else { 0.0 };
                        let AAT = if AAM != 0.0 {
                            let AAN = GT / (GT + (JH * AAK));
                            AAN
                        } else {
                            let AAO = GT / (GT - (JH * AAK));
                            AAO
                        };
                        let AAP = (-AAL) + AAJ;
                        let AAQ = if AAP > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                        let AAV = if AAQ != 0.0 {
                            let AAR = AAP.exp();
                            AAR
                        } else {
                            let AAS = NA / (GT + ((-2.3025850929940458e2f64 - AAP) * (GT + (GR * ((-2.3025850929940458e2f64 - AAP) * (GT + ((-2.3025850929940458e2f64 - AAP) * NB)))))));
                            AAS
                        };
                        let AAU = AAT * AAT;
                        let AAW = (((JG * AAT) + (JJ * AAU)) + (JK * (AAU * AAT))) * AAV;
                        let ABC;
                        if AAM != 0.0 {
                            ABC = AAW;
                        } else {
                            let AAX = if AAJ > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            let ABA = if AAX != 0.0 {
                                let AAY = AAJ.exp();
                                AAY
                            } else {
                                let AAZ = NA / (GT + ((-2.3025850929940458e2f64 - AAJ) * (GT + (GR * ((-2.3025850929940458e2f64 - AAJ) * (GT + ((-2.3025850929940458e2f64 - AAJ) * NB)))))));
                                AAZ
                            };
                            let ABB = (HS * ABA) - AAW;
                            ABC = ABB;
                        }
                        let ABE = CD * ((ABD * (8.86226925452758e-1f64 * ((JO * ABC) / AAI))) * AAG);
                        ACJ = ABE;
                    }
                    let ABF = if CW == A { 1.0 } else { 0.0 };
                    let ACK;
                    if ABF != 0.0 {
                        ACK = A;
                    } else {
                        let ABG = if AO == GR { 1.0 } else { 0.0 };
                        let ABK = if ABG != 0.0 {
                            let ABI = ((AC - ABH) * JD).sqrt();
                            ABI
                        } else {
                            let ABJ = ((AC - ABH) * JD).powf(AO);
                            ABJ
                        };
                        let ABL = IR * (((AC - ABH) * JA) / ABK);
                        let ABM = (-KB) / ABL;
                        let ABN = if (ABM.abs()) < MW { 1.0 } else { 0.0 };
                        let ABT;
                        if ABN != 0.0 {
                            let ABO = ABM.exp();
                            ABT = ABO;
                        } else {
                            let ABP = if ABM < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            let ABU = if ABP != 0.0 {
                                let ABQ = NA / (GT + ((-2.3025850929940458e2f64 - ABM) * (GT + (GR * ((-2.3025850929940458e2f64 - ABM) * (GT + ((-2.3025850929940458e2f64 - ABM) * NB)))))));
                                ABQ
                            } else {
                                let ABR = ABM - MW;
                                let ABS = ND * (GT + (ABR * (GT + (GR * (ABR * (GT + (ABR * NB)))))));
                                ABS
                            };
                            ABT = ABU;
                        }
                        let ABV = CW * (((PD * ABL) * ABL) * ABT);
                        ACK = ABV;
                    }
                    let ABY = if (if ABW > LG { 1.0 } else { 0.0 }) != 0.0 || (if ABX == A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let ACL;
                    if ABY != 0.0 {
                        ACL = GT;
                    } else {
                        let ACA = if ABZ > ((-KG) * ABW) { 1.0 } else { 0.0 };
                        let ACM;
                        if ACA != 0.0 {
                            let ACB = if DP == OR { 1.0 } else { 0.0 };
                            let ACF = if ACB != 0.0 {
                                let ACC = (ABZ * KV).abs();
                                let ACD = ((ACC * ACC) * ACC) * ACC;
                                ACD
                            } else {
                                let ACE = ((ABZ * KV).abs()).powf(DP);
                                ACE
                            };
                            let ACG = GT / (GT - ACF);
                            ACM = ACG;
                        } else {
                            let ACH = KH + ((ABZ + (KG * ABW)) * KW);
                            ACM = ACH;
                        }
                        ACL = ACM;
                    }
                    let ACN = (((YY + ACI) + ACJ) + ACK) * ACL;
                    AJI = ACN;
                }
                let AJJ;
                if NJ != 0.0 {
                    AJJ = A;
                } else {
                    let ACO = if IP == GR { 1.0 } else { 0.0 };
                    if ACO != 0.0 {
                    } else {
                    }
                    let ACQ = IF * ACP;
                    let ACR = if CG == A { 1.0 } else { 0.0 };
                    let ACS = if (if BQ == A { 1.0 } else { 0.0 }) != 0.0 && ACR != 0.0 { 1.0 } else { 0.0 };
                    let ADF;
                    let ADG;
                    let ADS;
                    let AEQ;
                    let AFS;
                    if ACS != 0.0 {
                        ADF = A;
                        ADG = A;
                        ADS = A;
                        AEQ = A;
                        AFS = A;
                    } else {
                        let ACT = IM - ZB;
                        let ACU = GT - ((GT - (ZD / ACT)).sqrt());
                        let ACV = if AT == GR { 1.0 } else { 0.0 };
                        let ACX = if ACV != 0.0 {
                            A
                        } else {
                            let ACW = ((((ACU * ACU) * (ACU.ln())) / (GT - ACU)) + ACU) * (GT - (HS * AT));
                            ACW
                        };
                        let ACY = ACU + ACX;
                        let ADB = if ACV != 0.0 {
                            let ACZ = (ACT * JE).sqrt();
                            ACZ
                        } else {
                            let ADA = (ACT * JE).powf(AT);
                            ADA
                        };
                        let ADC = IY * ADB;
                        let ADD = HY * ((ZN - GT) * ADC);
                        let ADE = BQ * (ADD * ACY);
                        ADF = ADC;
                        ADG = ACT;
                        ADS = ACY;
                        AEQ = ADD;
                        AFS = ADE;
                    }
                    let AFT;
                    if ACR != 0.0 {
                        AFT = A;
                    } else {
                        let ADH = JU * ((ADF * IP) / ADG);
                        let ADI = (ZT * JP) / ADH;
                        let ADJ = ADI * ADI;
                        let ADK = ADJ * ADJ;
                        let ADL = (ADK / (ADK + GT)).sqrt();
                        let ADM = (ADL.abs()).sqrt();
                        let ADN = ADL * ADM;
                        let ADO = (-AT) * IS;
                        let ADP = if ADO == -1e0f64 { 1.0 } else { 0.0 };
                        let ADT = if ADP != 0.0 {
                            let ADQ = GT / (GT + (ADH * ADN));
                            ADQ
                        } else {
                            let ADR = (GT + (ADH * ADN)).powf(ADO);
                            ADR
                        };
                        let ADU = (ADS * ADT) / (ADS + ADT);
                        let ADV = (AAH * (ADH / ADM)).sqrt();
                        let ADW = (((JP * ADI) * ADM) - (JP * ADL)) + (GR * (ADH * ADN));
                        let ADX = (((HS * (ADI * ADM)) - ADL) - GT) * ADV;
                        let ADY = ADX * ADX;
                        let ADZ = if ADX > A { 1.0 } else { 0.0 };
                        let AEG = if ADZ != 0.0 {
                            let AEA = GT / (GT + (JH * ADX));
                            AEA
                        } else {
                            let AEB = GT / (GT - (JH * ADX));
                            AEB
                        };
                        let AEC = (-ADY) + ADW;
                        let AED = if AEC > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                        let AEI = if AED != 0.0 {
                            let AEE = AEC.exp();
                            AEE
                        } else {
                            let AEF = NA / (GT + ((-2.3025850929940458e2f64 - AEC) * (GT + (GR * ((-2.3025850929940458e2f64 - AEC) * (GT + ((-2.3025850929940458e2f64 - AEC) * NB)))))));
                            AEF
                        };
                        let AEH = AEG * AEG;
                        let AEJ = (((JG * AEG) + (JJ * AEH)) + (JK * (AEH * AEG))) * AEI;
                        let AEP;
                        if ADZ != 0.0 {
                            AEP = AEJ;
                        } else {
                            let AEK = if ADW > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            let AEN = if AEK != 0.0 {
                                let AEL = ADW.exp();
                                AEL
                            } else {
                                let AEM = NA / (GT + ((-2.3025850929940458e2f64 - ADW) * (GT + (GR * ((-2.3025850929940458e2f64 - ADW) * (GT + ((-2.3025850929940458e2f64 - ADW) * NB)))))));
                                AEM
                            };
                            let AEO = (HS * AEN) - AEJ;
                            AEP = AEO;
                        }
                        let AER = CG * ((AEQ * (8.86226925452758e-1f64 * ((JP * AEP) / ADV))) * ADU);
                        AFT = AER;
                    }
                    let AES = if CZ == A { 1.0 } else { 0.0 };
                    let AFU;
                    if AES != 0.0 {
                        AFU = A;
                    } else {
                        let AET = if AT == GR { 1.0 } else { 0.0 };
                        let AEW = if AET != 0.0 {
                            let AEU = ((AF - ABH) * JE).sqrt();
                            AEU
                        } else {
                            let AEV = ((AF - ABH) * JE).powf(AT);
                            AEV
                        };
                        let AEX = IS * (((AF - ABH) * JB) / AEW);
                        let AEY = (-KD) / AEX;
                        let AEZ = if (AEY.abs()) < MW { 1.0 } else { 0.0 };
                        let AFF;
                        if AEZ != 0.0 {
                            let AFA = AEY.exp();
                            AFF = AFA;
                        } else {
                            let AFB = if AEY < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            let AFG = if AFB != 0.0 {
                                let AFC = NA / (GT + ((-2.3025850929940458e2f64 - AEY) * (GT + (GR * ((-2.3025850929940458e2f64 - AEY) * (GT + ((-2.3025850929940458e2f64 - AEY) * NB)))))));
                                AFC
                            } else {
                                let AFD = AEY - MW;
                                let AFE = ND * (GT + (AFD * (GT + (GR * (AFD * (GT + (AFD * NB)))))));
                                AFE
                            };
                            AFF = AFG;
                        }
                        let AFH = CZ * (((PD * AEX) * AEX) * AFF);
                        AFU = AFH;
                    }
                    let AFJ = if (if AFI > LG { 1.0 } else { 0.0 }) != 0.0 || (if ABX == A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let AFV;
                    if AFJ != 0.0 {
                        AFV = GT;
                    } else {
                        let AFK = if ABZ > ((-KG) * AFI) { 1.0 } else { 0.0 };
                        let AFW;
                        if AFK != 0.0 {
                            let AFL = if DS == OR { 1.0 } else { 0.0 };
                            let AFP = if AFL != 0.0 {
                                let AFM = (ABZ * KX).abs();
                                let AFN = ((AFM * AFM) * AFM) * AFM;
                                AFN
                            } else {
                                let AFO = ((ABZ * KX).abs()).powf(DS);
                                AFO
                            };
                            let AFQ = GT / (GT - AFP);
                            AFW = AFQ;
                        } else {
                            let AFR = KI + ((ABZ + (KG * AFI)) * KY);
                            AFW = AFR;
                        }
                        AFV = AFW;
                    }
                    let AFX = (((ACQ + AFS) + AFT) + AFU) * AFV;
                    AJJ = AFX;
                }
                let AJK;
                if NM != 0.0 {
                    AJK = A;
                } else {
                    let AFY = if IQ == GR { 1.0 } else { 0.0 };
                    if AFY != 0.0 {
                    } else {
                    }
                    let AGA = IG * AFZ;
                    let AGB = if CJ == A { 1.0 } else { 0.0 };
                    let AGC = if (if BT == A { 1.0 } else { 0.0 }) != 0.0 && AGB != 0.0 { 1.0 } else { 0.0 };
                    let AGP;
                    let AGQ;
                    let AHC;
                    let AIA;
                    let AJC;
                    if AGC != 0.0 {
                        AGP = A;
                        AGQ = A;
                        AHC = A;
                        AIA = A;
                        AJC = A;
                    } else {
                        let AGD = IN - ZB;
                        let AGE = GT - ((GT - (ZD / AGD)).sqrt());
                        let AGF = if AY == GR { 1.0 } else { 0.0 };
                        let AGH = if AGF != 0.0 {
                            A
                        } else {
                            let AGG = ((((AGE * AGE) * (AGE.ln())) / (GT - AGE)) + AGE) * (GT - (HS * AY));
                            AGG
                        };
                        let AGI = AGE + AGH;
                        let AGL = if AGF != 0.0 {
                            let AGJ = (AGD * JF).sqrt();
                            AGJ
                        } else {
                            let AGK = (AGD * JF).powf(AY);
                            AGK
                        };
                        let AGM = IZ * AGL;
                        let AGN = IA * ((ZN - GT) * AGM);
                        let AGO = BT * (AGN * AGI);
                        AGP = AGM;
                        AGQ = AGD;
                        AHC = AGI;
                        AIA = AGN;
                        AJC = AGO;
                    }
                    let AJD;
                    if AGB != 0.0 {
                        AJD = A;
                    } else {
                        let AGR = JV * ((AGP * IQ) / AGQ);
                        let AGS = (ZT * JQ) / AGR;
                        let AGT = AGS * AGS;
                        let AGU = AGT * AGT;
                        let AGV = (AGU / (AGU + GT)).sqrt();
                        let AGW = (AGV.abs()).sqrt();
                        let AGX = AGV * AGW;
                        let AGY = (-AY) * IT;
                        let AGZ = if AGY == -1e0f64 { 1.0 } else { 0.0 };
                        let AHD = if AGZ != 0.0 {
                            let AHA = GT / (GT + (AGR * AGX));
                            AHA
                        } else {
                            let AHB = (GT + (AGR * AGX)).powf(AGY);
                            AHB
                        };
                        let AHE = (AHC * AHD) / (AHC + AHD);
                        let AHF = (AAH * (AGR / AGW)).sqrt();
                        let AHG = (((JQ * AGS) * AGW) - (JQ * AGV)) + (GR * (AGR * AGX));
                        let AHH = (((HS * (AGS * AGW)) - AGV) - GT) * AHF;
                        let AHI = AHH * AHH;
                        let AHJ = if AHH > A { 1.0 } else { 0.0 };
                        let AHQ = if AHJ != 0.0 {
                            let AHK = GT / (GT + (JH * AHH));
                            AHK
                        } else {
                            let AHL = GT / (GT - (JH * AHH));
                            AHL
                        };
                        let AHM = (-AHI) + AHG;
                        let AHN = if AHM > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                        let AHS = if AHN != 0.0 {
                            let AHO = AHM.exp();
                            AHO
                        } else {
                            let AHP = NA / (GT + ((-2.3025850929940458e2f64 - AHM) * (GT + (GR * ((-2.3025850929940458e2f64 - AHM) * (GT + ((-2.3025850929940458e2f64 - AHM) * NB)))))));
                            AHP
                        };
                        let AHR = AHQ * AHQ;
                        let AHT = (((JG * AHQ) + (JJ * AHR)) + (JK * (AHR * AHQ))) * AHS;
                        let AHZ;
                        if AHJ != 0.0 {
                            AHZ = AHT;
                        } else {
                            let AHU = if AHG > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            let AHX = if AHU != 0.0 {
                                let AHV = AHG.exp();
                                AHV
                            } else {
                                let AHW = NA / (GT + ((-2.3025850929940458e2f64 - AHG) * (GT + (GR * ((-2.3025850929940458e2f64 - AHG) * (GT + ((-2.3025850929940458e2f64 - AHG) * NB)))))));
                                AHW
                            };
                            let AHY = (HS * AHX) - AHT;
                            AHZ = AHY;
                        }
                        let AIB = CJ * ((AIA * (8.86226925452758e-1f64 * ((JQ * AHZ) / AHF))) * AHE);
                        AJD = AIB;
                    }
                    let AIC = if DC == A { 1.0 } else { 0.0 };
                    let AJE;
                    if AIC != 0.0 {
                        AJE = A;
                    } else {
                        let AID = if AY == GR { 1.0 } else { 0.0 };
                        let AIG = if AID != 0.0 {
                            let AIE = ((AI - ABH) * JF).sqrt();
                            AIE
                        } else {
                            let AIF = ((AI - ABH) * JF).powf(AY);
                            AIF
                        };
                        let AIH = IT * (((AI - ABH) * JC) / AIG);
                        let AII = (-KF) / AIH;
                        let AIJ = if (AII.abs()) < MW { 1.0 } else { 0.0 };
                        let AIP;
                        if AIJ != 0.0 {
                            let AIK = AII.exp();
                            AIP = AIK;
                        } else {
                            let AIL = if AII < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            let AIQ = if AIL != 0.0 {
                                let AIM = NA / (GT + ((-2.3025850929940458e2f64 - AII) * (GT + (GR * ((-2.3025850929940458e2f64 - AII) * (GT + ((-2.3025850929940458e2f64 - AII) * NB)))))));
                                AIM
                            } else {
                                let AIN = AII - MW;
                                let AIO = ND * (GT + (AIN * (GT + (GR * (AIN * (GT + (AIN * NB)))))));
                                AIO
                            };
                            AIP = AIQ;
                        }
                        let AIR = DC * (((PD * AIH) * AIH) * AIP);
                        AJE = AIR;
                    }
                    let AIT = if (if AIS > LG { 1.0 } else { 0.0 }) != 0.0 || (if ABX == A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let AJF;
                    if AIT != 0.0 {
                        AJF = GT;
                    } else {
                        let AIU = if ABZ > ((-KG) * AIS) { 1.0 } else { 0.0 };
                        let AJG;
                        if AIU != 0.0 {
                            let AIV = if DV == OR { 1.0 } else { 0.0 };
                            let AIZ = if AIV != 0.0 {
                                let AIW = (ABZ * KZ).abs();
                                let AIX = ((AIW * AIW) * AIW) * AIW;
                                AIX
                            } else {
                                let AIY = ((ABZ * KZ).abs()).powf(DV);
                                AIY
                            };
                            let AJA = GT / (GT - AIZ);
                            AJG = AJA;
                        } else {
                            let AJB = KJ + ((ABZ + (KG * AIS)) * LA);
                            AJG = AJB;
                        }
                        AJF = AJG;
                    }
                    let AJH = (((AGA + AJC) + AJD) + AJE) * AJF;
                    AJK = AJH;
                }
                let AJL = ((LY * AJI) + (MC * AJJ)) + (MG * AJK);
                let ASX;
                let ATB;
                let ATD;
                let ATN;
                let AVF;
                let AVV;
                let AWL;
                let AZU;
                if PH != 0.0 {
                    let AJM = if PE < MU { 1.0 } else { 0.0 };
                    let ARW;
                    let ASA;
                    let ASE;
                    let ASI;
                    if AJM != 0.0 {
                        let AJN = GR * (PE * HK);
                        let AJO = if (AJN.abs()) < MW { 1.0 } else { 0.0 };
                        let ASJ;
                        if AJO != 0.0 {
                            let AJP = AJN.exp();
                            ASJ = AJP;
                        } else {
                            let AJQ = if AJN < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            let ASK = if AJQ != 0.0 {
                                let AJR = NA / (GT + ((-2.3025850929940458e2f64 - AJN) * (GT + (GR * ((-2.3025850929940458e2f64 - AJN) * (GT + ((-2.3025850929940458e2f64 - AJN) * NB)))))));
                                AJR
                            } else {
                                let AJS = AJN - MW;
                                let AJT = ND * (GT + (AJS * (GT + (GR * (AJS * (GT + (AJS * NB)))))));
                                AJT
                            };
                            ASJ = ASK;
                        }
                        let AJU = if EV < PQ { 1.0 } else { 0.0 };
                        let AKO;
                        let AKP;
                        if AJU != 0.0 {
                            let AJV = EV - (PS * LS);
                            let AJW = (PQ - ((PS * (PE - LS)) + EV)) - CL;
                            let AJX = (OR * PQ) * CL;
                            let AJY = if AJX > A { 1.0 } else { 0.0 };
                            let AKA = if AJY != 0.0 {
                                AJX
                            } else {
                                let AJZ = -AJX;
                                AJZ
                            };
                            let AKB = ((PQ - (GR * (AJW + (((AJW * AJW) + AKA).sqrt())))) - EV) - CL;
                            let AKC = (OR * EV) * CL;
                            let AKD = if AKC > A { 1.0 } else { 0.0 };
                            let AKF = if AKD != 0.0 {
                                AKC
                            } else {
                                let AKE = -AKC;
                                AKE
                            };
                            let AKG = EV + (GR * (AKB + (((AKB * AKB) + AKF).sqrt())));
                            let AKH = (PQ - AJV) - CL;
                            let AKJ = if AJY != 0.0 {
                                AJX
                            } else {
                                let AKI = -AJX;
                                AKI
                            };
                            let AKK = ((PQ - (GR * (AKH + (((AKH * AKH) + AKJ).sqrt())))) - EV) - CL;
                            let AKM = if AKD != 0.0 {
                                AKC
                            } else {
                                let AKL = -AKC;
                                AKL
                            };
                            let AKN = EV + (GR * (AKK + (((AKK * AKK) + AKM).sqrt())));
                            AKO = AKG;
                            AKP = AKN;
                        } else {
                            AKO = EV;
                            AKP = EV;
                        }
                        let AKQ = HK * ((PE / AKO) + ((LS * (AKO - AKP)) / (AKP * PQ)));
                        let AKR = if (AKQ.abs()) < MW { 1.0 } else { 0.0 };
                        let ARX;
                        if AKR != 0.0 {
                            let AKS = AKQ.exp();
                            ARX = AKS;
                        } else {
                            let AKT = if AKQ < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            let ARY = if AKT != 0.0 {
                                let AKU = NA / (GT + ((-2.3025850929940458e2f64 - AKQ) * (GT + (GR * ((-2.3025850929940458e2f64 - AKQ) * (GT + ((-2.3025850929940458e2f64 - AKQ) * NB)))))));
                                AKU
                            } else {
                                let AKV = AKQ - MW;
                                let AKW = ND * (GT + (AKV * (GT + (GR * (AKV * (GT + (AKV * NB)))))));
                                AKW
                            };
                            ARX = ARY;
                        }
                        let AKX = (EY / HK) * ((LI / (LM / LI)).ln());
                        let AKY = if EY < PQ { 1.0 } else { 0.0 };
                        let ALS;
                        let ALT;
                        if AKY != 0.0 {
                            let AKZ = EY - (PS * AKX);
                            let ALA = (PQ - ((PS * (PE - AKX)) + EY)) - CL;
                            let ALB = (OR * PQ) * CL;
                            let ALC = if ALB > A { 1.0 } else { 0.0 };
                            let ALE = if ALC != 0.0 {
                                ALB
                            } else {
                                let ALD = -ALB;
                                ALD
                            };
                            let ALF = ((PQ - (GR * (ALA + (((ALA * ALA) + ALE).sqrt())))) - EY) - CL;
                            let ALG = (OR * EY) * CL;
                            let ALH = if ALG > A { 1.0 } else { 0.0 };
                            let ALJ = if ALH != 0.0 {
                                ALG
                            } else {
                                let ALI = -ALG;
                                ALI
                            };
                            let ALK = EY + (GR * (ALF + (((ALF * ALF) + ALJ).sqrt())));
                            let ALL = (PQ - AKZ) - CL;
                            let ALN = if ALC != 0.0 {
                                ALB
                            } else {
                                let ALM = -ALB;
                                ALM
                            };
                            let ALO = ((PQ - (GR * (ALL + (((ALL * ALL) + ALN).sqrt())))) - EY) - CL;
                            let ALQ = if ALH != 0.0 {
                                ALG
                            } else {
                                let ALP = -ALG;
                                ALP
                            };
                            let ALR = EY + (GR * (ALO + (((ALO * ALO) + ALQ).sqrt())));
                            ALS = ALK;
                            ALT = ALR;
                        } else {
                            ALS = EY;
                            ALT = EY;
                        }
                        let ALU = HK * ((PE / ALS) + ((AKX * (ALS - ALT)) / (ALT * PQ)));
                        let ALV = if (ALU.abs()) < MW { 1.0 } else { 0.0 };
                        let ASB;
                        if ALV != 0.0 {
                            let ALW = ALU.exp();
                            ASB = ALW;
                        } else {
                            let ALX = if ALU < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            let ASC = if ALX != 0.0 {
                                let ALY = NA / (GT + ((-2.3025850929940458e2f64 - ALU) * (GT + (GR * ((-2.3025850929940458e2f64 - ALU) * (GT + ((-2.3025850929940458e2f64 - ALU) * NB)))))));
                                ALY
                            } else {
                                let ALZ = ALU - MW;
                                let AMA = ND * (GT + (ALZ * (GT + (GR * (ALZ * (GT + (ALZ * NB)))))));
                                AMA
                            };
                            ASB = ASC;
                        }
                        let AMB = (FB / HK) * ((LJ / (LM / LJ)).ln());
                        let AMC = if FB < PQ { 1.0 } else { 0.0 };
                        let AMW;
                        let AMX;
                        if AMC != 0.0 {
                            let AMD = FB - (PS * AMB);
                            let AME = (PQ - ((PS * (PE - AMB)) + FB)) - CL;
                            let AMF = (OR * PQ) * CL;
                            let AMG = if AMF > A { 1.0 } else { 0.0 };
                            let AMI = if AMG != 0.0 {
                                AMF
                            } else {
                                let AMH = -AMF;
                                AMH
                            };
                            let AMJ = ((PQ - (GR * (AME + (((AME * AME) + AMI).sqrt())))) - FB) - CL;
                            let AMK = (OR * FB) * CL;
                            let AML = if AMK > A { 1.0 } else { 0.0 };
                            let AMN = if AML != 0.0 {
                                AMK
                            } else {
                                let AMM = -AMK;
                                AMM
                            };
                            let AMO = FB + (GR * (AMJ + (((AMJ * AMJ) + AMN).sqrt())));
                            let AMP = (PQ - AMD) - CL;
                            let AMR = if AMG != 0.0 {
                                AMF
                            } else {
                                let AMQ = -AMF;
                                AMQ
                            };
                            let AMS = ((PQ - (GR * (AMP + (((AMP * AMP) + AMR).sqrt())))) - FB) - CL;
                            let AMU = if AML != 0.0 {
                                AMK
                            } else {
                                let AMT = -AMK;
                                AMT
                            };
                            let AMV = FB + (GR * (AMS + (((AMS * AMS) + AMU).sqrt())));
                            AMW = AMO;
                            AMX = AMV;
                        } else {
                            AMW = FB;
                            AMX = FB;
                        }
                        let AMY = HK * ((PE / AMW) + ((AMB * (AMW - AMX)) / (AMX * PQ)));
                        let AMZ = if (AMY.abs()) < MW { 1.0 } else { 0.0 };
                        let ASF;
                        if AMZ != 0.0 {
                            let ANA = AMY.exp();
                            ASF = ANA;
                        } else {
                            let ANB = if AMY < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            let ASG = if ANB != 0.0 {
                                let ANC = NA / (GT + ((-2.3025850929940458e2f64 - AMY) * (GT + (GR * ((-2.3025850929940458e2f64 - AMY) * (GT + ((-2.3025850929940458e2f64 - AMY) * NB)))))));
                                ANC
                            } else {
                                let AND = AMY - MW;
                                let ANE = ND * (GT + (AND * (GT + (GR * (AND * (GT + (AND * NB)))))));
                                ANE
                            };
                            ASF = ASG;
                        }
                        ARW = ARX;
                        ASA = ASB;
                        ASE = ASF;
                        ASI = ASJ;
                    } else {
                        let ANF = PE - MU;
                        let ANG = ((GT + (ANF * HK)) * TE).sqrt();
                        let ANH = if EV < PQ { 1.0 } else { 0.0 };
                        let AOG;
                        let AOH;
                        let AOQ;
                        if ANH != 0.0 {
                            let ANI = EV - (PS * LS);
                            let ANJ = (PQ - ((PS * (MU - LS)) + EV)) - CL;
                            let ANK = (OR * PQ) * CL;
                            let ANL = if ANK > A { 1.0 } else { 0.0 };
                            let ANN = if ANL != 0.0 {
                                ANK
                            } else {
                                let ANM = -ANK;
                                ANM
                            };
                            let ANO = ((ANJ * ANJ) + ANN).sqrt();
                            let ANP = GR * (GT + (ANJ / ANO));
                            let ANQ = ((PQ - (GR * (ANJ + ANO))) - EV) - CL;
                            let ANR = (OR * EV) * CL;
                            let ANS = if ANR > A { 1.0 } else { 0.0 };
                            let ANU = if ANS != 0.0 {
                                ANR
                            } else {
                                let ANT = -ANR;
                                ANT
                            };
                            let ANV = ((ANQ * ANQ) + ANU).sqrt();
                            let ANW = GR * (GT + (ANQ / ANV));
                            let ANX = EV + (GR * (ANQ + ANV));
                            let ANY = (PQ - ANI) - CL;
                            let AOA = if ANL != 0.0 {
                                ANK
                            } else {
                                let ANZ = -ANK;
                                ANZ
                            };
                            let AOB = ((PQ - (GR * (ANY + (((ANY * ANY) + AOA).sqrt())))) - EV) - CL;
                            let AOD = if ANS != 0.0 {
                                ANR
                            } else {
                                let AOC = -ANR;
                                AOC
                            };
                            let AOE = EV + (GR * (AOB + (((AOB * AOB) + AOD).sqrt())));
                            let AOF = (PS * ANP) * ANW;
                            AOG = ANX;
                            AOH = AOE;
                            AOQ = AOF;
                        } else {
                            AOG = EV;
                            AOH = EV;
                            AOQ = A;
                        }
                        let AOI = AOH * PQ;
                        let AOJ = HK * ((MU / AOG) + ((LS * (AOG - AOH)) / AOI));
                        let AOK = if (AOJ.abs()) < MW { 1.0 } else { 0.0 };
                        let AOR;
                        if AOK != 0.0 {
                            let AOL = AOJ.exp();
                            AOR = AOL;
                        } else {
                            let AOM = if AOJ < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            let AOS = if AOM != 0.0 {
                                let AON = NA / (GT + ((-2.3025850929940458e2f64 - AOJ) * (GT + (GR * ((-2.3025850929940458e2f64 - AOJ) * (GT + ((-2.3025850929940458e2f64 - AOJ) * NB)))))));
                                AON
                            } else {
                                let AOO = AOJ - MW;
                                let AOP = ND * (GT + (AOO * (GT + (GR * (AOO * (GT + (AOO * NB)))))));
                                AOP
                            };
                            AOR = AOS;
                        }
                        let AOT = (GT + (ANF * (HK * (((AOG - (MU * AOQ)) / (AOG * AOG)) + ((LS * AOQ) / AOI))))) * AOR;
                        let AOU = (EY / HK) * ((LI / (LM / LI)).ln());
                        let AOV = if EY < PQ { 1.0 } else { 0.0 };
                        let APU;
                        let APV;
                        let AQE;
                        if AOV != 0.0 {
                            let AOW = EY - (PS * AOU);
                            let AOX = (PQ - ((PS * (MU - AOU)) + EY)) - CL;
                            let AOY = (OR * PQ) * CL;
                            let AOZ = if AOY > A { 1.0 } else { 0.0 };
                            let APB = if AOZ != 0.0 {
                                AOY
                            } else {
                                let APA = -AOY;
                                APA
                            };
                            let APC = ((AOX * AOX) + APB).sqrt();
                            let APD = GR * (GT + (AOX / APC));
                            let APE = ((PQ - (GR * (AOX + APC))) - EY) - CL;
                            let APF = (OR * EY) * CL;
                            let APG = if APF > A { 1.0 } else { 0.0 };
                            let API = if APG != 0.0 {
                                APF
                            } else {
                                let APH = -APF;
                                APH
                            };
                            let APJ = ((APE * APE) + API).sqrt();
                            let APK = GR * (GT + (APE / APJ));
                            let APL = EY + (GR * (APE + APJ));
                            let APM = (PQ - AOW) - CL;
                            let APO = if AOZ != 0.0 {
                                AOY
                            } else {
                                let APN = -AOY;
                                APN
                            };
                            let APP = ((PQ - (GR * (APM + (((APM * APM) + APO).sqrt())))) - EY) - CL;
                            let APR = if APG != 0.0 {
                                APF
                            } else {
                                let APQ = -APF;
                                APQ
                            };
                            let APS = EY + (GR * (APP + (((APP * APP) + APR).sqrt())));
                            let APT = (PS * APD) * APK;
                            APU = APL;
                            APV = APS;
                            AQE = APT;
                        } else {
                            APU = EY;
                            APV = EY;
                            AQE = A;
                        }
                        let APW = APV * PQ;
                        let APX = HK * ((MU / APU) + ((AOU * (APU - APV)) / APW));
                        let APY = if (APX.abs()) < MW { 1.0 } else { 0.0 };
                        let AQF;
                        if APY != 0.0 {
                            let APZ = APX.exp();
                            AQF = APZ;
                        } else {
                            let AQA = if APX < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            let AQG = if AQA != 0.0 {
                                let AQB = NA / (GT + ((-2.3025850929940458e2f64 - APX) * (GT + (GR * ((-2.3025850929940458e2f64 - APX) * (GT + ((-2.3025850929940458e2f64 - APX) * NB)))))));
                                AQB
                            } else {
                                let AQC = APX - MW;
                                let AQD = ND * (GT + (AQC * (GT + (GR * (AQC * (GT + (AQC * NB)))))));
                                AQD
                            };
                            AQF = AQG;
                        }
                        let AQH = (GT + (ANF * (HK * (((APU - (MU * AQE)) / (APU * APU)) + ((AOU * AQE) / APW))))) * AQF;
                        let AQI = (FB / HK) * ((LJ / (LM / LJ)).ln());
                        let AQJ = if FB < PQ { 1.0 } else { 0.0 };
                        let ARI;
                        let ARJ;
                        let ARS;
                        if AQJ != 0.0 {
                            let AQK = FB - (PS * AQI);
                            let AQL = (PQ - ((PS * (MU - AQI)) + FB)) - CL;
                            let AQM = (OR * PQ) * CL;
                            let AQN = if AQM > A { 1.0 } else { 0.0 };
                            let AQP = if AQN != 0.0 {
                                AQM
                            } else {
                                let AQO = -AQM;
                                AQO
                            };
                            let AQQ = ((AQL * AQL) + AQP).sqrt();
                            let AQR = GR * (GT + (AQL / AQQ));
                            let AQS = ((PQ - (GR * (AQL + AQQ))) - FB) - CL;
                            let AQT = (OR * FB) * CL;
                            let AQU = if AQT > A { 1.0 } else { 0.0 };
                            let AQW = if AQU != 0.0 {
                                AQT
                            } else {
                                let AQV = -AQT;
                                AQV
                            };
                            let AQX = ((AQS * AQS) + AQW).sqrt();
                            let AQY = GR * (GT + (AQS / AQX));
                            let AQZ = FB + (GR * (AQS + AQX));
                            let ARA = (PQ - AQK) - CL;
                            let ARC = if AQN != 0.0 {
                                AQM
                            } else {
                                let ARB = -AQM;
                                ARB
                            };
                            let ARD = ((PQ - (GR * (ARA + (((ARA * ARA) + ARC).sqrt())))) - FB) - CL;
                            let ARF = if AQU != 0.0 {
                                AQT
                            } else {
                                let ARE = -AQT;
                                ARE
                            };
                            let ARG = FB + (GR * (ARD + (((ARD * ARD) + ARF).sqrt())));
                            let ARH = (PS * AQR) * AQY;
                            ARI = AQZ;
                            ARJ = ARG;
                            ARS = ARH;
                        } else {
                            ARI = FB;
                            ARJ = FB;
                            ARS = A;
                        }
                        let ARK = ARJ * PQ;
                        let ARL = HK * ((MU / ARI) + ((AQI * (ARI - ARJ)) / ARK));
                        let ARM = if (ARL.abs()) < MW { 1.0 } else { 0.0 };
                        let ART;
                        if ARM != 0.0 {
                            let ARN = ARL.exp();
                            ART = ARN;
                        } else {
                            let ARO = if ARL < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            let ARU = if ARO != 0.0 {
                                let ARP = NA / (GT + ((-2.3025850929940458e2f64 - ARL) * (GT + (GR * ((-2.3025850929940458e2f64 - ARL) * (GT + ((-2.3025850929940458e2f64 - ARL) * NB)))))));
                                ARP
                            } else {
                                let ARQ = ARL - MW;
                                let ARR = ND * (GT + (ARQ * (GT + (GR * (ARQ * (GT + (ARQ * NB)))))));
                                ARR
                            };
                            ART = ARU;
                        }
                        let ARV = (GT + (ANF * (HK * (((ARI - (MU * ARS)) / (ARI * ARI)) + ((AQI * ARS) / ARK))))) * ART;
                        ARW = AOT;
                        ASA = AQH;
                        ASE = ARV;
                        ASI = ANG;
                    }
                    let ARZ = ARW - GT;
                    let ASD = ASA - GT;
                    let ASH = ASE - GT;
                    let ASL = GT / ASI;
                    let ASM = if PE > A { 1.0 } else { 0.0 };
                    let ASP = if ASM != 0.0 {
                        let ASN = HS * (HJ * (((HS + ASL) + (((ASL + GT) * (ASL + JI)).sqrt())).ln()));
                        ASN
                    } else {
                        let ASO = (-PE) + (HS * (HJ * ((((HS * ASI) + GT) + (((GT + ASI) * (GT + (JI * ASI))).sqrt())).ln())));
                        ASO
                    };
                    let ASQ = NS - ASP;
                    let ASR = PE - ASQ;
                    let ASS = GR * ((PE + ASQ) - (((ASR * ASR) + ((OR * HJ) * HJ)).sqrt()));
                    let AST = PE - NW;
                    let ASU = GR * ((PE + NW) - (((AST * AST) + ((OR * HH) * HH)).sqrt()));
                    let ASV = GR * (PE - (((PE * PE) + 4e-12f64).sqrt()));
                    ASX = ARZ;
                    ATB = ASS;
                    ATD = ASP;
                    ATN = ASI;
                    AVF = ASU;
                    AVV = ASV;
                    AWL = ASD;
                    AZU = ASH;
                } else {
                    ASX = A;
                    ATB = A;
                    ATD = A;
                    ATN = A;
                    AVF = A;
                    AVV = A;
                    AWL = A;
                    AZU = A;
                }
                let BDC;
                if NG != 0.0 {
                    BDC = A;
                } else {
                    let ASW = if IO == GR { 1.0 } else { 0.0 };
                    if ASW != 0.0 {
                    } else {
                    }
                    let ASY = IE * ASX;
                    let ASZ = if CD == A { 1.0 } else { 0.0 };
                    let ATA = if (if BN == A { 1.0 } else { 0.0 }) != 0.0 && ASZ != 0.0 { 1.0 } else { 0.0 };
                    let ATQ;
                    let ATR;
                    let AUD;
                    let AVB;
                    let AWE;
                    if ATA != 0.0 {
                        ATQ = A;
                        ATR = A;
                        AUD = A;
                        AVB = A;
                        AWE = A;
                    } else {
                        let ATC = IL - ATB;
                        let ATE = GT - ((GT - (ATD / ATC)).sqrt());
                        let ATF = if AO == GR { 1.0 } else { 0.0 };
                        let ATH = if ATF != 0.0 {
                            A
                        } else {
                            let ATG = ((((ATE * ATE) * (ATE.ln())) / (GT - ATE)) + ATE) * (GT - (HS * AO));
                            ATG
                        };
                        let ATI = ATE + ATH;
                        let ATL = if ATF != 0.0 {
                            let ATJ = (ATC * JD).sqrt();
                            ATJ
                        } else {
                            let ATK = (ATC * JD).powf(AO);
                            ATK
                        };
                        let ATM = IX * ATL;
                        let ATO = HW * ((ATN - GT) * ATM);
                        let ATP = BN * (ATO * ATI);
                        ATQ = ATM;
                        ATR = ATC;
                        AUD = ATI;
                        AVB = ATO;
                        AWE = ATP;
                    }
                    let AWF;
                    if ASZ != 0.0 {
                        AWF = A;
                    } else {
                        let ATS = JT * ((ATQ * IO) / ATR);
                        let ATT = (ZT * JO) / ATS;
                        let ATU = ATT * ATT;
                        let ATV = ATU * ATU;
                        let ATW = (ATV / (ATV + GT)).sqrt();
                        let ATX = (ATW.abs()).sqrt();
                        let ATY = ATW * ATX;
                        let ATZ = (-AO) * IR;
                        let AUA = if ATZ == -1e0f64 { 1.0 } else { 0.0 };
                        let AUE = if AUA != 0.0 {
                            let AUB = GT / (GT + (ATS * ATY));
                            AUB
                        } else {
                            let AUC = (GT + (ATS * ATY)).powf(ATZ);
                            AUC
                        };
                        let AUF = (AUD * AUE) / (AUD + AUE);
                        let AUG = (AAH * (ATS / ATX)).sqrt();
                        let AUH = (((JO * ATT) * ATX) - (JO * ATW)) + (GR * (ATS * ATY));
                        let AUI = (((HS * (ATT * ATX)) - ATW) - GT) * AUG;
                        let AUJ = AUI * AUI;
                        let AUK = if AUI > A { 1.0 } else { 0.0 };
                        let AUR = if AUK != 0.0 {
                            let AUL = GT / (GT + (JH * AUI));
                            AUL
                        } else {
                            let AUM = GT / (GT - (JH * AUI));
                            AUM
                        };
                        let AUN = (-AUJ) + AUH;
                        let AUO = if AUN > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                        let AUT = if AUO != 0.0 {
                            let AUP = AUN.exp();
                            AUP
                        } else {
                            let AUQ = NA / (GT + ((-2.3025850929940458e2f64 - AUN) * (GT + (GR * ((-2.3025850929940458e2f64 - AUN) * (GT + ((-2.3025850929940458e2f64 - AUN) * NB)))))));
                            AUQ
                        };
                        let AUS = AUR * AUR;
                        let AUU = (((JG * AUR) + (JJ * AUS)) + (JK * (AUS * AUR))) * AUT;
                        let AVA;
                        if AUK != 0.0 {
                            AVA = AUU;
                        } else {
                            let AUV = if AUH > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            let AUY = if AUV != 0.0 {
                                let AUW = AUH.exp();
                                AUW
                            } else {
                                let AUX = NA / (GT + ((-2.3025850929940458e2f64 - AUH) * (GT + (GR * ((-2.3025850929940458e2f64 - AUH) * (GT + ((-2.3025850929940458e2f64 - AUH) * NB)))))));
                                AUX
                            };
                            let AUZ = (HS * AUY) - AUU;
                            AVA = AUZ;
                        }
                        let AVC = CD * ((AVB * (8.86226925452758e-1f64 * ((JO * AVA) / AUG))) * AUF);
                        AWF = AVC;
                    }
                    let AVD = if CW == A { 1.0 } else { 0.0 };
                    let AWG;
                    if AVD != 0.0 {
                        AWG = A;
                    } else {
                        let AVE = if AO == GR { 1.0 } else { 0.0 };
                        let AVI = if AVE != 0.0 {
                            let AVG = ((AC - AVF) * JD).sqrt();
                            AVG
                        } else {
                            let AVH = ((AC - AVF) * JD).powf(AO);
                            AVH
                        };
                        let AVJ = IR * (((AC - AVF) * JA) / AVI);
                        let AVK = (-KB) / AVJ;
                        let AVL = if (AVK.abs()) < MW { 1.0 } else { 0.0 };
                        let AVR;
                        if AVL != 0.0 {
                            let AVM = AVK.exp();
                            AVR = AVM;
                        } else {
                            let AVN = if AVK < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            let AVS = if AVN != 0.0 {
                                let AVO = NA / (GT + ((-2.3025850929940458e2f64 - AVK) * (GT + (GR * ((-2.3025850929940458e2f64 - AVK) * (GT + ((-2.3025850929940458e2f64 - AVK) * NB)))))));
                                AVO
                            } else {
                                let AVP = AVK - MW;
                                let AVQ = ND * (GT + (AVP * (GT + (GR * (AVP * (GT + (AVP * NB)))))));
                                AVQ
                            };
                            AVR = AVS;
                        }
                        let AVT = CW * (((PE * AVJ) * AVJ) * AVR);
                        AWG = AVT;
                    }
                    let AVU = if (if ABW > LG { 1.0 } else { 0.0 }) != 0.0 || (if ABX == A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let AWH;
                    if AVU != 0.0 {
                        AWH = GT;
                    } else {
                        let AVW = if AVV > ((-KG) * ABW) { 1.0 } else { 0.0 };
                        let AWI;
                        if AVW != 0.0 {
                            let AVX = if DP == OR { 1.0 } else { 0.0 };
                            let AWB = if AVX != 0.0 {
                                let AVY = (AVV * KV).abs();
                                let AVZ = ((AVY * AVY) * AVY) * AVY;
                                AVZ
                            } else {
                                let AWA = ((AVV * KV).abs()).powf(DP);
                                AWA
                            };
                            let AWC = GT / (GT - AWB);
                            AWI = AWC;
                        } else {
                            let AWD = KH + ((AVV + (KG * ABW)) * KW);
                            AWI = AWD;
                        }
                        AWH = AWI;
                    }
                    let AWJ = (((ASY + AWE) + AWF) + AWG) * AWH;
                    BDC = AWJ;
                }
                let BDD;
                if NJ != 0.0 {
                    BDD = A;
                } else {
                    let AWK = if IP == GR { 1.0 } else { 0.0 };
                    if AWK != 0.0 {
                    } else {
                    }
                    let AWM = IF * AWL;
                    let AWN = if CG == A { 1.0 } else { 0.0 };
                    let AWO = if (if BQ == A { 1.0 } else { 0.0 }) != 0.0 && AWN != 0.0 { 1.0 } else { 0.0 };
                    let AXB;
                    let AXC;
                    let AXO;
                    let AYM;
                    let AZN;
                    if AWO != 0.0 {
                        AXB = A;
                        AXC = A;
                        AXO = A;
                        AYM = A;
                        AZN = A;
                    } else {
                        let AWP = IM - ATB;
                        let AWQ = GT - ((GT - (ATD / AWP)).sqrt());
                        let AWR = if AT == GR { 1.0 } else { 0.0 };
                        let AWT = if AWR != 0.0 {
                            A
                        } else {
                            let AWS = ((((AWQ * AWQ) * (AWQ.ln())) / (GT - AWQ)) + AWQ) * (GT - (HS * AT));
                            AWS
                        };
                        let AWU = AWQ + AWT;
                        let AWX = if AWR != 0.0 {
                            let AWV = (AWP * JE).sqrt();
                            AWV
                        } else {
                            let AWW = (AWP * JE).powf(AT);
                            AWW
                        };
                        let AWY = IY * AWX;
                        let AWZ = HY * ((ATN - GT) * AWY);
                        let AXA = BQ * (AWZ * AWU);
                        AXB = AWY;
                        AXC = AWP;
                        AXO = AWU;
                        AYM = AWZ;
                        AZN = AXA;
                    }
                    let AZO;
                    if AWN != 0.0 {
                        AZO = A;
                    } else {
                        let AXD = JU * ((AXB * IP) / AXC);
                        let AXE = (ZT * JP) / AXD;
                        let AXF = AXE * AXE;
                        let AXG = AXF * AXF;
                        let AXH = (AXG / (AXG + GT)).sqrt();
                        let AXI = (AXH.abs()).sqrt();
                        let AXJ = AXH * AXI;
                        let AXK = (-AT) * IS;
                        let AXL = if AXK == -1e0f64 { 1.0 } else { 0.0 };
                        let AXP = if AXL != 0.0 {
                            let AXM = GT / (GT + (AXD * AXJ));
                            AXM
                        } else {
                            let AXN = (GT + (AXD * AXJ)).powf(AXK);
                            AXN
                        };
                        let AXQ = (AXO * AXP) / (AXO + AXP);
                        let AXR = (AAH * (AXD / AXI)).sqrt();
                        let AXS = (((JP * AXE) * AXI) - (JP * AXH)) + (GR * (AXD * AXJ));
                        let AXT = (((HS * (AXE * AXI)) - AXH) - GT) * AXR;
                        let AXU = AXT * AXT;
                        let AXV = if AXT > A { 1.0 } else { 0.0 };
                        let AYC = if AXV != 0.0 {
                            let AXW = GT / (GT + (JH * AXT));
                            AXW
                        } else {
                            let AXX = GT / (GT - (JH * AXT));
                            AXX
                        };
                        let AXY = (-AXU) + AXS;
                        let AXZ = if AXY > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                        let AYE = if AXZ != 0.0 {
                            let AYA = AXY.exp();
                            AYA
                        } else {
                            let AYB = NA / (GT + ((-2.3025850929940458e2f64 - AXY) * (GT + (GR * ((-2.3025850929940458e2f64 - AXY) * (GT + ((-2.3025850929940458e2f64 - AXY) * NB)))))));
                            AYB
                        };
                        let AYD = AYC * AYC;
                        let AYF = (((JG * AYC) + (JJ * AYD)) + (JK * (AYD * AYC))) * AYE;
                        let AYL;
                        if AXV != 0.0 {
                            AYL = AYF;
                        } else {
                            let AYG = if AXS > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            let AYJ = if AYG != 0.0 {
                                let AYH = AXS.exp();
                                AYH
                            } else {
                                let AYI = NA / (GT + ((-2.3025850929940458e2f64 - AXS) * (GT + (GR * ((-2.3025850929940458e2f64 - AXS) * (GT + ((-2.3025850929940458e2f64 - AXS) * NB)))))));
                                AYI
                            };
                            let AYK = (HS * AYJ) - AYF;
                            AYL = AYK;
                        }
                        let AYN = CG * ((AYM * (8.86226925452758e-1f64 * ((JP * AYL) / AXR))) * AXQ);
                        AZO = AYN;
                    }
                    let AYO = if CZ == A { 1.0 } else { 0.0 };
                    let AZP;
                    if AYO != 0.0 {
                        AZP = A;
                    } else {
                        let AYP = if AT == GR { 1.0 } else { 0.0 };
                        let AYS = if AYP != 0.0 {
                            let AYQ = ((AF - AVF) * JE).sqrt();
                            AYQ
                        } else {
                            let AYR = ((AF - AVF) * JE).powf(AT);
                            AYR
                        };
                        let AYT = IS * (((AF - AVF) * JB) / AYS);
                        let AYU = (-KD) / AYT;
                        let AYV = if (AYU.abs()) < MW { 1.0 } else { 0.0 };
                        let AZB;
                        if AYV != 0.0 {
                            let AYW = AYU.exp();
                            AZB = AYW;
                        } else {
                            let AYX = if AYU < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            let AZC = if AYX != 0.0 {
                                let AYY = NA / (GT + ((-2.3025850929940458e2f64 - AYU) * (GT + (GR * ((-2.3025850929940458e2f64 - AYU) * (GT + ((-2.3025850929940458e2f64 - AYU) * NB)))))));
                                AYY
                            } else {
                                let AYZ = AYU - MW;
                                let AZA = ND * (GT + (AYZ * (GT + (GR * (AYZ * (GT + (AYZ * NB)))))));
                                AZA
                            };
                            AZB = AZC;
                        }
                        let AZD = CZ * (((PE * AYT) * AYT) * AZB);
                        AZP = AZD;
                    }
                    let AZE = if (if AFI > LG { 1.0 } else { 0.0 }) != 0.0 || (if ABX == A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let AZQ;
                    if AZE != 0.0 {
                        AZQ = GT;
                    } else {
                        let AZF = if AVV > ((-KG) * AFI) { 1.0 } else { 0.0 };
                        let AZR;
                        if AZF != 0.0 {
                            let AZG = if DS == OR { 1.0 } else { 0.0 };
                            let AZK = if AZG != 0.0 {
                                let AZH = (AVV * KX).abs();
                                let AZI = ((AZH * AZH) * AZH) * AZH;
                                AZI
                            } else {
                                let AZJ = ((AVV * KX).abs()).powf(DS);
                                AZJ
                            };
                            let AZL = GT / (GT - AZK);
                            AZR = AZL;
                        } else {
                            let AZM = KI + ((AVV + (KG * AFI)) * KY);
                            AZR = AZM;
                        }
                        AZQ = AZR;
                    }
                    let AZS = (((AWM + AZN) + AZO) + AZP) * AZQ;
                    BDD = AZS;
                }
                let BDE;
                if NM != 0.0 {
                    BDE = A;
                } else {
                    let AZT = if IQ == GR { 1.0 } else { 0.0 };
                    if AZT != 0.0 {
                    } else {
                    }
                    let AZV = IG * AZU;
                    let AZW = if CJ == A { 1.0 } else { 0.0 };
                    let AZX = if (if BT == A { 1.0 } else { 0.0 }) != 0.0 && AZW != 0.0 { 1.0 } else { 0.0 };
                    let BAK;
                    let BAL;
                    let BAX;
                    let BBV;
                    let BCW;
                    if AZX != 0.0 {
                        BAK = A;
                        BAL = A;
                        BAX = A;
                        BBV = A;
                        BCW = A;
                    } else {
                        let AZY = IN - ATB;
                        let AZZ = GT - ((GT - (ATD / AZY)).sqrt());
                        let BAA = if AY == GR { 1.0 } else { 0.0 };
                        let BAC = if BAA != 0.0 {
                            A
                        } else {
                            let BAB = ((((AZZ * AZZ) * (AZZ.ln())) / (GT - AZZ)) + AZZ) * (GT - (HS * AY));
                            BAB
                        };
                        let BAD = AZZ + BAC;
                        let BAG = if BAA != 0.0 {
                            let BAE = (AZY * JF).sqrt();
                            BAE
                        } else {
                            let BAF = (AZY * JF).powf(AY);
                            BAF
                        };
                        let BAH = IZ * BAG;
                        let BAI = IA * ((ATN - GT) * BAH);
                        let BAJ = BT * (BAI * BAD);
                        BAK = BAH;
                        BAL = AZY;
                        BAX = BAD;
                        BBV = BAI;
                        BCW = BAJ;
                    }
                    let BCX;
                    if AZW != 0.0 {
                        BCX = A;
                    } else {
                        let BAM = JV * ((BAK * IQ) / BAL);
                        let BAN = (ZT * JQ) / BAM;
                        let BAO = BAN * BAN;
                        let BAP = BAO * BAO;
                        let BAQ = (BAP / (BAP + GT)).sqrt();
                        let BAR = (BAQ.abs()).sqrt();
                        let BAS = BAQ * BAR;
                        let BAT = (-AY) * IT;
                        let BAU = if BAT == -1e0f64 { 1.0 } else { 0.0 };
                        let BAY = if BAU != 0.0 {
                            let BAV = GT / (GT + (BAM * BAS));
                            BAV
                        } else {
                            let BAW = (GT + (BAM * BAS)).powf(BAT);
                            BAW
                        };
                        let BAZ = (BAX * BAY) / (BAX + BAY);
                        let BBA = (AAH * (BAM / BAR)).sqrt();
                        let BBB = (((JQ * BAN) * BAR) - (JQ * BAQ)) + (GR * (BAM * BAS));
                        let BBC = (((HS * (BAN * BAR)) - BAQ) - GT) * BBA;
                        let BBD = BBC * BBC;
                        let BBE = if BBC > A { 1.0 } else { 0.0 };
                        let BBL = if BBE != 0.0 {
                            let BBF = GT / (GT + (JH * BBC));
                            BBF
                        } else {
                            let BBG = GT / (GT - (JH * BBC));
                            BBG
                        };
                        let BBH = (-BBD) + BBB;
                        let BBI = if BBH > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                        let BBN = if BBI != 0.0 {
                            let BBJ = BBH.exp();
                            BBJ
                        } else {
                            let BBK = NA / (GT + ((-2.3025850929940458e2f64 - BBH) * (GT + (GR * ((-2.3025850929940458e2f64 - BBH) * (GT + ((-2.3025850929940458e2f64 - BBH) * NB)))))));
                            BBK
                        };
                        let BBM = BBL * BBL;
                        let BBO = (((JG * BBL) + (JJ * BBM)) + (JK * (BBM * BBL))) * BBN;
                        let BBU;
                        if BBE != 0.0 {
                            BBU = BBO;
                        } else {
                            let BBP = if BBB > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            let BBS = if BBP != 0.0 {
                                let BBQ = BBB.exp();
                                BBQ
                            } else {
                                let BBR = NA / (GT + ((-2.3025850929940458e2f64 - BBB) * (GT + (GR * ((-2.3025850929940458e2f64 - BBB) * (GT + ((-2.3025850929940458e2f64 - BBB) * NB)))))));
                                BBR
                            };
                            let BBT = (HS * BBS) - BBO;
                            BBU = BBT;
                        }
                        let BBW = CJ * ((BBV * (8.86226925452758e-1f64 * ((JQ * BBU) / BBA))) * BAZ);
                        BCX = BBW;
                    }
                    let BBX = if DC == A { 1.0 } else { 0.0 };
                    let BCY;
                    if BBX != 0.0 {
                        BCY = A;
                    } else {
                        let BBY = if AY == GR { 1.0 } else { 0.0 };
                        let BCB = if BBY != 0.0 {
                            let BBZ = ((AI - AVF) * JF).sqrt();
                            BBZ
                        } else {
                            let BCA = ((AI - AVF) * JF).powf(AY);
                            BCA
                        };
                        let BCC = IT * (((AI - AVF) * JC) / BCB);
                        let BCD = (-KF) / BCC;
                        let BCE = if (BCD.abs()) < MW { 1.0 } else { 0.0 };
                        let BCK;
                        if BCE != 0.0 {
                            let BCF = BCD.exp();
                            BCK = BCF;
                        } else {
                            let BCG = if BCD < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            let BCL = if BCG != 0.0 {
                                let BCH = NA / (GT + ((-2.3025850929940458e2f64 - BCD) * (GT + (GR * ((-2.3025850929940458e2f64 - BCD) * (GT + ((-2.3025850929940458e2f64 - BCD) * NB)))))));
                                BCH
                            } else {
                                let BCI = BCD - MW;
                                let BCJ = ND * (GT + (BCI * (GT + (GR * (BCI * (GT + (BCI * NB)))))));
                                BCJ
                            };
                            BCK = BCL;
                        }
                        let BCM = DC * (((PE * BCC) * BCC) * BCK);
                        BCY = BCM;
                    }
                    let BCN = if (if AIS > LG { 1.0 } else { 0.0 }) != 0.0 || (if ABX == A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let BCZ;
                    if BCN != 0.0 {
                        BCZ = GT;
                    } else {
                        let BCO = if AVV > ((-KG) * AIS) { 1.0 } else { 0.0 };
                        let BDA;
                        if BCO != 0.0 {
                            let BCP = if DV == OR { 1.0 } else { 0.0 };
                            let BCT = if BCP != 0.0 {
                                let BCQ = (AVV * KZ).abs();
                                let BCR = ((BCQ * BCQ) * BCQ) * BCQ;
                                BCR
                            } else {
                                let BCS = ((AVV * KZ).abs()).powf(DV);
                                BCS
                            };
                            let BCU = GT / (GT - BCT);
                            BDA = BCU;
                        } else {
                            let BCV = KJ + ((AVV + (KG * AIS)) * LA);
                            BDA = BCV;
                        }
                        BCZ = BDA;
                    }
                    let BDB = (((AZV + BCW) + BCX) + BCY) * BCZ;
                    BDE = BDB;
                }
                let BDF = ((LY * BDC) + (MC * BDD)) + (MG * BDE);
                let BMR;
                let BMV;
                let BMX;
                let BNH;
                let BOZ;
                let BPP;
                let BQF;
                let BTO;
                if PH != 0.0 {
                    let BDG = if PF < MU { 1.0 } else { 0.0 };
                    let BLQ;
                    let BLU;
                    let BLY;
                    let BMC;
                    if BDG != 0.0 {
                        let BDH = GR * (PF * HK);
                        let BDI = if (BDH.abs()) < MW { 1.0 } else { 0.0 };
                        let BMD;
                        if BDI != 0.0 {
                            let BDJ = BDH.exp();
                            BMD = BDJ;
                        } else {
                            let BDK = if BDH < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            let BME = if BDK != 0.0 {
                                let BDL = NA / (GT + ((-2.3025850929940458e2f64 - BDH) * (GT + (GR * ((-2.3025850929940458e2f64 - BDH) * (GT + ((-2.3025850929940458e2f64 - BDH) * NB)))))));
                                BDL
                            } else {
                                let BDM = BDH - MW;
                                let BDN = ND * (GT + (BDM * (GT + (GR * (BDM * (GT + (BDM * NB)))))));
                                BDN
                            };
                            BMD = BME;
                        }
                        let BDO = if EV < PQ { 1.0 } else { 0.0 };
                        let BEI;
                        let BEJ;
                        if BDO != 0.0 {
                            let BDP = EV - (PS * LS);
                            let BDQ = (PQ - ((PS * (PF - LS)) + EV)) - CL;
                            let BDR = (OR * PQ) * CL;
                            let BDS = if BDR > A { 1.0 } else { 0.0 };
                            let BDU = if BDS != 0.0 {
                                BDR
                            } else {
                                let BDT = -BDR;
                                BDT
                            };
                            let BDV = ((PQ - (GR * (BDQ + (((BDQ * BDQ) + BDU).sqrt())))) - EV) - CL;
                            let BDW = (OR * EV) * CL;
                            let BDX = if BDW > A { 1.0 } else { 0.0 };
                            let BDZ = if BDX != 0.0 {
                                BDW
                            } else {
                                let BDY = -BDW;
                                BDY
                            };
                            let BEA = EV + (GR * (BDV + (((BDV * BDV) + BDZ).sqrt())));
                            let BEB = (PQ - BDP) - CL;
                            let BED = if BDS != 0.0 {
                                BDR
                            } else {
                                let BEC = -BDR;
                                BEC
                            };
                            let BEE = ((PQ - (GR * (BEB + (((BEB * BEB) + BED).sqrt())))) - EV) - CL;
                            let BEG = if BDX != 0.0 {
                                BDW
                            } else {
                                let BEF = -BDW;
                                BEF
                            };
                            let BEH = EV + (GR * (BEE + (((BEE * BEE) + BEG).sqrt())));
                            BEI = BEA;
                            BEJ = BEH;
                        } else {
                            BEI = EV;
                            BEJ = EV;
                        }
                        let BEK = HK * ((PF / BEI) + ((LS * (BEI - BEJ)) / (BEJ * PQ)));
                        let BEL = if (BEK.abs()) < MW { 1.0 } else { 0.0 };
                        let BLR;
                        if BEL != 0.0 {
                            let BEM = BEK.exp();
                            BLR = BEM;
                        } else {
                            let BEN = if BEK < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            let BLS = if BEN != 0.0 {
                                let BEO = NA / (GT + ((-2.3025850929940458e2f64 - BEK) * (GT + (GR * ((-2.3025850929940458e2f64 - BEK) * (GT + ((-2.3025850929940458e2f64 - BEK) * NB)))))));
                                BEO
                            } else {
                                let BEP = BEK - MW;
                                let BEQ = ND * (GT + (BEP * (GT + (GR * (BEP * (GT + (BEP * NB)))))));
                                BEQ
                            };
                            BLR = BLS;
                        }
                        let BER = (EY / HK) * ((LI / (LM / LI)).ln());
                        let BES = if EY < PQ { 1.0 } else { 0.0 };
                        let BFM;
                        let BFN;
                        if BES != 0.0 {
                            let BET = EY - (PS * BER);
                            let BEU = (PQ - ((PS * (PF - BER)) + EY)) - CL;
                            let BEV = (OR * PQ) * CL;
                            let BEW = if BEV > A { 1.0 } else { 0.0 };
                            let BEY = if BEW != 0.0 {
                                BEV
                            } else {
                                let BEX = -BEV;
                                BEX
                            };
                            let BEZ = ((PQ - (GR * (BEU + (((BEU * BEU) + BEY).sqrt())))) - EY) - CL;
                            let BFA = (OR * EY) * CL;
                            let BFB = if BFA > A { 1.0 } else { 0.0 };
                            let BFD = if BFB != 0.0 {
                                BFA
                            } else {
                                let BFC = -BFA;
                                BFC
                            };
                            let BFE = EY + (GR * (BEZ + (((BEZ * BEZ) + BFD).sqrt())));
                            let BFF = (PQ - BET) - CL;
                            let BFH = if BEW != 0.0 {
                                BEV
                            } else {
                                let BFG = -BEV;
                                BFG
                            };
                            let BFI = ((PQ - (GR * (BFF + (((BFF * BFF) + BFH).sqrt())))) - EY) - CL;
                            let BFK = if BFB != 0.0 {
                                BFA
                            } else {
                                let BFJ = -BFA;
                                BFJ
                            };
                            let BFL = EY + (GR * (BFI + (((BFI * BFI) + BFK).sqrt())));
                            BFM = BFE;
                            BFN = BFL;
                        } else {
                            BFM = EY;
                            BFN = EY;
                        }
                        let BFO = HK * ((PF / BFM) + ((BER * (BFM - BFN)) / (BFN * PQ)));
                        let BFP = if (BFO.abs()) < MW { 1.0 } else { 0.0 };
                        let BLV;
                        if BFP != 0.0 {
                            let BFQ = BFO.exp();
                            BLV = BFQ;
                        } else {
                            let BFR = if BFO < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            let BLW = if BFR != 0.0 {
                                let BFS = NA / (GT + ((-2.3025850929940458e2f64 - BFO) * (GT + (GR * ((-2.3025850929940458e2f64 - BFO) * (GT + ((-2.3025850929940458e2f64 - BFO) * NB)))))));
                                BFS
                            } else {
                                let BFT = BFO - MW;
                                let BFU = ND * (GT + (BFT * (GT + (GR * (BFT * (GT + (BFT * NB)))))));
                                BFU
                            };
                            BLV = BLW;
                        }
                        let BFV = (FB / HK) * ((LJ / (LM / LJ)).ln());
                        let BFW = if FB < PQ { 1.0 } else { 0.0 };
                        let BGQ;
                        let BGR;
                        if BFW != 0.0 {
                            let BFX = FB - (PS * BFV);
                            let BFY = (PQ - ((PS * (PF - BFV)) + FB)) - CL;
                            let BFZ = (OR * PQ) * CL;
                            let BGA = if BFZ > A { 1.0 } else { 0.0 };
                            let BGC = if BGA != 0.0 {
                                BFZ
                            } else {
                                let BGB = -BFZ;
                                BGB
                            };
                            let BGD = ((PQ - (GR * (BFY + (((BFY * BFY) + BGC).sqrt())))) - FB) - CL;
                            let BGE = (OR * FB) * CL;
                            let BGF = if BGE > A { 1.0 } else { 0.0 };
                            let BGH = if BGF != 0.0 {
                                BGE
                            } else {
                                let BGG = -BGE;
                                BGG
                            };
                            let BGI = FB + (GR * (BGD + (((BGD * BGD) + BGH).sqrt())));
                            let BGJ = (PQ - BFX) - CL;
                            let BGL = if BGA != 0.0 {
                                BFZ
                            } else {
                                let BGK = -BFZ;
                                BGK
                            };
                            let BGM = ((PQ - (GR * (BGJ + (((BGJ * BGJ) + BGL).sqrt())))) - FB) - CL;
                            let BGO = if BGF != 0.0 {
                                BGE
                            } else {
                                let BGN = -BGE;
                                BGN
                            };
                            let BGP = FB + (GR * (BGM + (((BGM * BGM) + BGO).sqrt())));
                            BGQ = BGI;
                            BGR = BGP;
                        } else {
                            BGQ = FB;
                            BGR = FB;
                        }
                        let BGS = HK * ((PF / BGQ) + ((BFV * (BGQ - BGR)) / (BGR * PQ)));
                        let BGT = if (BGS.abs()) < MW { 1.0 } else { 0.0 };
                        let BLZ;
                        if BGT != 0.0 {
                            let BGU = BGS.exp();
                            BLZ = BGU;
                        } else {
                            let BGV = if BGS < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            let BMA = if BGV != 0.0 {
                                let BGW = NA / (GT + ((-2.3025850929940458e2f64 - BGS) * (GT + (GR * ((-2.3025850929940458e2f64 - BGS) * (GT + ((-2.3025850929940458e2f64 - BGS) * NB)))))));
                                BGW
                            } else {
                                let BGX = BGS - MW;
                                let BGY = ND * (GT + (BGX * (GT + (GR * (BGX * (GT + (BGX * NB)))))));
                                BGY
                            };
                            BLZ = BMA;
                        }
                        BLQ = BLR;
                        BLU = BLV;
                        BLY = BLZ;
                        BMC = BMD;
                    } else {
                        let BGZ = PF - MU;
                        let BHA = ((GT + (BGZ * HK)) * TE).sqrt();
                        let BHB = if EV < PQ { 1.0 } else { 0.0 };
                        let BIA;
                        let BIB;
                        let BIK;
                        if BHB != 0.0 {
                            let BHC = EV - (PS * LS);
                            let BHD = (PQ - ((PS * (MU - LS)) + EV)) - CL;
                            let BHE = (OR * PQ) * CL;
                            let BHF = if BHE > A { 1.0 } else { 0.0 };
                            let BHH = if BHF != 0.0 {
                                BHE
                            } else {
                                let BHG = -BHE;
                                BHG
                            };
                            let BHI = ((BHD * BHD) + BHH).sqrt();
                            let BHJ = GR * (GT + (BHD / BHI));
                            let BHK = ((PQ - (GR * (BHD + BHI))) - EV) - CL;
                            let BHL = (OR * EV) * CL;
                            let BHM = if BHL > A { 1.0 } else { 0.0 };
                            let BHO = if BHM != 0.0 {
                                BHL
                            } else {
                                let BHN = -BHL;
                                BHN
                            };
                            let BHP = ((BHK * BHK) + BHO).sqrt();
                            let BHQ = GR * (GT + (BHK / BHP));
                            let BHR = EV + (GR * (BHK + BHP));
                            let BHS = (PQ - BHC) - CL;
                            let BHU = if BHF != 0.0 {
                                BHE
                            } else {
                                let BHT = -BHE;
                                BHT
                            };
                            let BHV = ((PQ - (GR * (BHS + (((BHS * BHS) + BHU).sqrt())))) - EV) - CL;
                            let BHX = if BHM != 0.0 {
                                BHL
                            } else {
                                let BHW = -BHL;
                                BHW
                            };
                            let BHY = EV + (GR * (BHV + (((BHV * BHV) + BHX).sqrt())));
                            let BHZ = (PS * BHJ) * BHQ;
                            BIA = BHR;
                            BIB = BHY;
                            BIK = BHZ;
                        } else {
                            BIA = EV;
                            BIB = EV;
                            BIK = A;
                        }
                        let BIC = BIB * PQ;
                        let BID = HK * ((MU / BIA) + ((LS * (BIA - BIB)) / BIC));
                        let BIE = if (BID.abs()) < MW { 1.0 } else { 0.0 };
                        let BIL;
                        if BIE != 0.0 {
                            let BIF = BID.exp();
                            BIL = BIF;
                        } else {
                            let BIG = if BID < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            let BIM = if BIG != 0.0 {
                                let BIH = NA / (GT + ((-2.3025850929940458e2f64 - BID) * (GT + (GR * ((-2.3025850929940458e2f64 - BID) * (GT + ((-2.3025850929940458e2f64 - BID) * NB)))))));
                                BIH
                            } else {
                                let BII = BID - MW;
                                let BIJ = ND * (GT + (BII * (GT + (GR * (BII * (GT + (BII * NB)))))));
                                BIJ
                            };
                            BIL = BIM;
                        }
                        let BIN = (GT + (BGZ * (HK * (((BIA - (MU * BIK)) / (BIA * BIA)) + ((LS * BIK) / BIC))))) * BIL;
                        let BIO = (EY / HK) * ((LI / (LM / LI)).ln());
                        let BIP = if EY < PQ { 1.0 } else { 0.0 };
                        let BJO;
                        let BJP;
                        let BJY;
                        if BIP != 0.0 {
                            let BIQ = EY - (PS * BIO);
                            let BIR = (PQ - ((PS * (MU - BIO)) + EY)) - CL;
                            let BIS = (OR * PQ) * CL;
                            let BIT = if BIS > A { 1.0 } else { 0.0 };
                            let BIV = if BIT != 0.0 {
                                BIS
                            } else {
                                let BIU = -BIS;
                                BIU
                            };
                            let BIW = ((BIR * BIR) + BIV).sqrt();
                            let BIX = GR * (GT + (BIR / BIW));
                            let BIY = ((PQ - (GR * (BIR + BIW))) - EY) - CL;
                            let BIZ = (OR * EY) * CL;
                            let BJA = if BIZ > A { 1.0 } else { 0.0 };
                            let BJC = if BJA != 0.0 {
                                BIZ
                            } else {
                                let BJB = -BIZ;
                                BJB
                            };
                            let BJD = ((BIY * BIY) + BJC).sqrt();
                            let BJE = GR * (GT + (BIY / BJD));
                            let BJF = EY + (GR * (BIY + BJD));
                            let BJG = (PQ - BIQ) - CL;
                            let BJI = if BIT != 0.0 {
                                BIS
                            } else {
                                let BJH = -BIS;
                                BJH
                            };
                            let BJJ = ((PQ - (GR * (BJG + (((BJG * BJG) + BJI).sqrt())))) - EY) - CL;
                            let BJL = if BJA != 0.0 {
                                BIZ
                            } else {
                                let BJK = -BIZ;
                                BJK
                            };
                            let BJM = EY + (GR * (BJJ + (((BJJ * BJJ) + BJL).sqrt())));
                            let BJN = (PS * BIX) * BJE;
                            BJO = BJF;
                            BJP = BJM;
                            BJY = BJN;
                        } else {
                            BJO = EY;
                            BJP = EY;
                            BJY = A;
                        }
                        let BJQ = BJP * PQ;
                        let BJR = HK * ((MU / BJO) + ((BIO * (BJO - BJP)) / BJQ));
                        let BJS = if (BJR.abs()) < MW { 1.0 } else { 0.0 };
                        let BJZ;
                        if BJS != 0.0 {
                            let BJT = BJR.exp();
                            BJZ = BJT;
                        } else {
                            let BJU = if BJR < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            let BKA = if BJU != 0.0 {
                                let BJV = NA / (GT + ((-2.3025850929940458e2f64 - BJR) * (GT + (GR * ((-2.3025850929940458e2f64 - BJR) * (GT + ((-2.3025850929940458e2f64 - BJR) * NB)))))));
                                BJV
                            } else {
                                let BJW = BJR - MW;
                                let BJX = ND * (GT + (BJW * (GT + (GR * (BJW * (GT + (BJW * NB)))))));
                                BJX
                            };
                            BJZ = BKA;
                        }
                        let BKB = (GT + (BGZ * (HK * (((BJO - (MU * BJY)) / (BJO * BJO)) + ((BIO * BJY) / BJQ))))) * BJZ;
                        let BKC = (FB / HK) * ((LJ / (LM / LJ)).ln());
                        let BKD = if FB < PQ { 1.0 } else { 0.0 };
                        let BLC;
                        let BLD;
                        let BLM;
                        if BKD != 0.0 {
                            let BKE = FB - (PS * BKC);
                            let BKF = (PQ - ((PS * (MU - BKC)) + FB)) - CL;
                            let BKG = (OR * PQ) * CL;
                            let BKH = if BKG > A { 1.0 } else { 0.0 };
                            let BKJ = if BKH != 0.0 {
                                BKG
                            } else {
                                let BKI = -BKG;
                                BKI
                            };
                            let BKK = ((BKF * BKF) + BKJ).sqrt();
                            let BKL = GR * (GT + (BKF / BKK));
                            let BKM = ((PQ - (GR * (BKF + BKK))) - FB) - CL;
                            let BKN = (OR * FB) * CL;
                            let BKO = if BKN > A { 1.0 } else { 0.0 };
                            let BKQ = if BKO != 0.0 {
                                BKN
                            } else {
                                let BKP = -BKN;
                                BKP
                            };
                            let BKR = ((BKM * BKM) + BKQ).sqrt();
                            let BKS = GR * (GT + (BKM / BKR));
                            let BKT = FB + (GR * (BKM + BKR));
                            let BKU = (PQ - BKE) - CL;
                            let BKW = if BKH != 0.0 {
                                BKG
                            } else {
                                let BKV = -BKG;
                                BKV
                            };
                            let BKX = ((PQ - (GR * (BKU + (((BKU * BKU) + BKW).sqrt())))) - FB) - CL;
                            let BKZ = if BKO != 0.0 {
                                BKN
                            } else {
                                let BKY = -BKN;
                                BKY
                            };
                            let BLA = FB + (GR * (BKX + (((BKX * BKX) + BKZ).sqrt())));
                            let BLB = (PS * BKL) * BKS;
                            BLC = BKT;
                            BLD = BLA;
                            BLM = BLB;
                        } else {
                            BLC = FB;
                            BLD = FB;
                            BLM = A;
                        }
                        let BLE = BLD * PQ;
                        let BLF = HK * ((MU / BLC) + ((BKC * (BLC - BLD)) / BLE));
                        let BLG = if (BLF.abs()) < MW { 1.0 } else { 0.0 };
                        let BLN;
                        if BLG != 0.0 {
                            let BLH = BLF.exp();
                            BLN = BLH;
                        } else {
                            let BLI = if BLF < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            let BLO = if BLI != 0.0 {
                                let BLJ = NA / (GT + ((-2.3025850929940458e2f64 - BLF) * (GT + (GR * ((-2.3025850929940458e2f64 - BLF) * (GT + ((-2.3025850929940458e2f64 - BLF) * NB)))))));
                                BLJ
                            } else {
                                let BLK = BLF - MW;
                                let BLL = ND * (GT + (BLK * (GT + (GR * (BLK * (GT + (BLK * NB)))))));
                                BLL
                            };
                            BLN = BLO;
                        }
                        let BLP = (GT + (BGZ * (HK * (((BLC - (MU * BLM)) / (BLC * BLC)) + ((BKC * BLM) / BLE))))) * BLN;
                        BLQ = BIN;
                        BLU = BKB;
                        BLY = BLP;
                        BMC = BHA;
                    }
                    let BLT = BLQ - GT;
                    let BLX = BLU - GT;
                    let BMB = BLY - GT;
                    let BMF = GT / BMC;
                    let BMG = if PF > A { 1.0 } else { 0.0 };
                    let BMJ = if BMG != 0.0 {
                        let BMH = HS * (HJ * (((HS + BMF) + (((BMF + GT) * (BMF + JI)).sqrt())).ln()));
                        BMH
                    } else {
                        let BMI = (-PF) + (HS * (HJ * ((((HS * BMC) + GT) + (((GT + BMC) * (GT + (JI * BMC))).sqrt())).ln())));
                        BMI
                    };
                    let BMK = NS - BMJ;
                    let BML = PF - BMK;
                    let BMM = GR * ((PF + BMK) - (((BML * BML) + ((OR * HJ) * HJ)).sqrt()));
                    let BMN = PF - NW;
                    let BMO = GR * ((PF + NW) - (((BMN * BMN) + ((OR * HH) * HH)).sqrt()));
                    let BMP = GR * (PF - (((PF * PF) + 4e-12f64).sqrt()));
                    BMR = BLT;
                    BMV = BMM;
                    BMX = BMJ;
                    BNH = BMC;
                    BOZ = BMO;
                    BPP = BMP;
                    BQF = BLX;
                    BTO = BMB;
                } else {
                    BMR = A;
                    BMV = A;
                    BMX = A;
                    BNH = A;
                    BOZ = A;
                    BPP = A;
                    BQF = A;
                    BTO = A;
                }
                let BWW;
                if NG != 0.0 {
                    BWW = A;
                } else {
                    let BMQ = if IO == GR { 1.0 } else { 0.0 };
                    if BMQ != 0.0 {
                    } else {
                    }
                    let BMS = IE * BMR;
                    let BMT = if CD == A { 1.0 } else { 0.0 };
                    let BMU = if (if BN == A { 1.0 } else { 0.0 }) != 0.0 && BMT != 0.0 { 1.0 } else { 0.0 };
                    let BNK;
                    let BNL;
                    let BNX;
                    let BOV;
                    let BPY;
                    if BMU != 0.0 {
                        BNK = A;
                        BNL = A;
                        BNX = A;
                        BOV = A;
                        BPY = A;
                    } else {
                        let BMW = IL - BMV;
                        let BMY = GT - ((GT - (BMX / BMW)).sqrt());
                        let BMZ = if AO == GR { 1.0 } else { 0.0 };
                        let BNB = if BMZ != 0.0 {
                            A
                        } else {
                            let BNA = ((((BMY * BMY) * (BMY.ln())) / (GT - BMY)) + BMY) * (GT - (HS * AO));
                            BNA
                        };
                        let BNC = BMY + BNB;
                        let BNF = if BMZ != 0.0 {
                            let BND = (BMW * JD).sqrt();
                            BND
                        } else {
                            let BNE = (BMW * JD).powf(AO);
                            BNE
                        };
                        let BNG = IX * BNF;
                        let BNI = HW * ((BNH - GT) * BNG);
                        let BNJ = BN * (BNI * BNC);
                        BNK = BNG;
                        BNL = BMW;
                        BNX = BNC;
                        BOV = BNI;
                        BPY = BNJ;
                    }
                    let BPZ;
                    if BMT != 0.0 {
                        BPZ = A;
                    } else {
                        let BNM = JT * ((BNK * IO) / BNL);
                        let BNN = (ZT * JO) / BNM;
                        let BNO = BNN * BNN;
                        let BNP = BNO * BNO;
                        let BNQ = (BNP / (BNP + GT)).sqrt();
                        let BNR = (BNQ.abs()).sqrt();
                        let BNS = BNQ * BNR;
                        let BNT = (-AO) * IR;
                        let BNU = if BNT == -1e0f64 { 1.0 } else { 0.0 };
                        let BNY = if BNU != 0.0 {
                            let BNV = GT / (GT + (BNM * BNS));
                            BNV
                        } else {
                            let BNW = (GT + (BNM * BNS)).powf(BNT);
                            BNW
                        };
                        let BNZ = (BNX * BNY) / (BNX + BNY);
                        let BOA = (AAH * (BNM / BNR)).sqrt();
                        let BOB = (((JO * BNN) * BNR) - (JO * BNQ)) + (GR * (BNM * BNS));
                        let BOC = (((HS * (BNN * BNR)) - BNQ) - GT) * BOA;
                        let BOD = BOC * BOC;
                        let BOE = if BOC > A { 1.0 } else { 0.0 };
                        let BOL = if BOE != 0.0 {
                            let BOF = GT / (GT + (JH * BOC));
                            BOF
                        } else {
                            let BOG = GT / (GT - (JH * BOC));
                            BOG
                        };
                        let BOH = (-BOD) + BOB;
                        let BOI = if BOH > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                        let BON = if BOI != 0.0 {
                            let BOJ = BOH.exp();
                            BOJ
                        } else {
                            let BOK = NA / (GT + ((-2.3025850929940458e2f64 - BOH) * (GT + (GR * ((-2.3025850929940458e2f64 - BOH) * (GT + ((-2.3025850929940458e2f64 - BOH) * NB)))))));
                            BOK
                        };
                        let BOM = BOL * BOL;
                        let BOO = (((JG * BOL) + (JJ * BOM)) + (JK * (BOM * BOL))) * BON;
                        let BOU;
                        if BOE != 0.0 {
                            BOU = BOO;
                        } else {
                            let BOP = if BOB > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            let BOS = if BOP != 0.0 {
                                let BOQ = BOB.exp();
                                BOQ
                            } else {
                                let BOR = NA / (GT + ((-2.3025850929940458e2f64 - BOB) * (GT + (GR * ((-2.3025850929940458e2f64 - BOB) * (GT + ((-2.3025850929940458e2f64 - BOB) * NB)))))));
                                BOR
                            };
                            let BOT = (HS * BOS) - BOO;
                            BOU = BOT;
                        }
                        let BOW = CD * ((BOV * (8.86226925452758e-1f64 * ((JO * BOU) / BOA))) * BNZ);
                        BPZ = BOW;
                    }
                    let BOX = if CW == A { 1.0 } else { 0.0 };
                    let BQA;
                    if BOX != 0.0 {
                        BQA = A;
                    } else {
                        let BOY = if AO == GR { 1.0 } else { 0.0 };
                        let BPC = if BOY != 0.0 {
                            let BPA = ((AC - BOZ) * JD).sqrt();
                            BPA
                        } else {
                            let BPB = ((AC - BOZ) * JD).powf(AO);
                            BPB
                        };
                        let BPD = IR * (((AC - BOZ) * JA) / BPC);
                        let BPE = (-KB) / BPD;
                        let BPF = if (BPE.abs()) < MW { 1.0 } else { 0.0 };
                        let BPL;
                        if BPF != 0.0 {
                            let BPG = BPE.exp();
                            BPL = BPG;
                        } else {
                            let BPH = if BPE < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            let BPM = if BPH != 0.0 {
                                let BPI = NA / (GT + ((-2.3025850929940458e2f64 - BPE) * (GT + (GR * ((-2.3025850929940458e2f64 - BPE) * (GT + ((-2.3025850929940458e2f64 - BPE) * NB)))))));
                                BPI
                            } else {
                                let BPJ = BPE - MW;
                                let BPK = ND * (GT + (BPJ * (GT + (GR * (BPJ * (GT + (BPJ * NB)))))));
                                BPK
                            };
                            BPL = BPM;
                        }
                        let BPN = CW * (((PF * BPD) * BPD) * BPL);
                        BQA = BPN;
                    }
                    let BPO = if (if ABW > LG { 1.0 } else { 0.0 }) != 0.0 || (if ABX == A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let BQB;
                    if BPO != 0.0 {
                        BQB = GT;
                    } else {
                        let BPQ = if BPP > ((-KG) * ABW) { 1.0 } else { 0.0 };
                        let BQC;
                        if BPQ != 0.0 {
                            let BPR = if DP == OR { 1.0 } else { 0.0 };
                            let BPV = if BPR != 0.0 {
                                let BPS = (BPP * KV).abs();
                                let BPT = ((BPS * BPS) * BPS) * BPS;
                                BPT
                            } else {
                                let BPU = ((BPP * KV).abs()).powf(DP);
                                BPU
                            };
                            let BPW = GT / (GT - BPV);
                            BQC = BPW;
                        } else {
                            let BPX = KH + ((BPP + (KG * ABW)) * KW);
                            BQC = BPX;
                        }
                        BQB = BQC;
                    }
                    let BQD = (((BMS + BPY) + BPZ) + BQA) * BQB;
                    BWW = BQD;
                }
                let BWX;
                if NJ != 0.0 {
                    BWX = A;
                } else {
                    let BQE = if IP == GR { 1.0 } else { 0.0 };
                    if BQE != 0.0 {
                    } else {
                    }
                    let BQG = IF * BQF;
                    let BQH = if CG == A { 1.0 } else { 0.0 };
                    let BQI = if (if BQ == A { 1.0 } else { 0.0 }) != 0.0 && BQH != 0.0 { 1.0 } else { 0.0 };
                    let BQV;
                    let BQW;
                    let BRI;
                    let BSG;
                    let BTH;
                    if BQI != 0.0 {
                        BQV = A;
                        BQW = A;
                        BRI = A;
                        BSG = A;
                        BTH = A;
                    } else {
                        let BQJ = IM - BMV;
                        let BQK = GT - ((GT - (BMX / BQJ)).sqrt());
                        let BQL = if AT == GR { 1.0 } else { 0.0 };
                        let BQN = if BQL != 0.0 {
                            A
                        } else {
                            let BQM = ((((BQK * BQK) * (BQK.ln())) / (GT - BQK)) + BQK) * (GT - (HS * AT));
                            BQM
                        };
                        let BQO = BQK + BQN;
                        let BQR = if BQL != 0.0 {
                            let BQP = (BQJ * JE).sqrt();
                            BQP
                        } else {
                            let BQQ = (BQJ * JE).powf(AT);
                            BQQ
                        };
                        let BQS = IY * BQR;
                        let BQT = HY * ((BNH - GT) * BQS);
                        let BQU = BQ * (BQT * BQO);
                        BQV = BQS;
                        BQW = BQJ;
                        BRI = BQO;
                        BSG = BQT;
                        BTH = BQU;
                    }
                    let BTI;
                    if BQH != 0.0 {
                        BTI = A;
                    } else {
                        let BQX = JU * ((BQV * IP) / BQW);
                        let BQY = (ZT * JP) / BQX;
                        let BQZ = BQY * BQY;
                        let BRA = BQZ * BQZ;
                        let BRB = (BRA / (BRA + GT)).sqrt();
                        let BRC = (BRB.abs()).sqrt();
                        let BRD = BRB * BRC;
                        let BRE = (-AT) * IS;
                        let BRF = if BRE == -1e0f64 { 1.0 } else { 0.0 };
                        let BRJ = if BRF != 0.0 {
                            let BRG = GT / (GT + (BQX * BRD));
                            BRG
                        } else {
                            let BRH = (GT + (BQX * BRD)).powf(BRE);
                            BRH
                        };
                        let BRK = (BRI * BRJ) / (BRI + BRJ);
                        let BRL = (AAH * (BQX / BRC)).sqrt();
                        let BRM = (((JP * BQY) * BRC) - (JP * BRB)) + (GR * (BQX * BRD));
                        let BRN = (((HS * (BQY * BRC)) - BRB) - GT) * BRL;
                        let BRO = BRN * BRN;
                        let BRP = if BRN > A { 1.0 } else { 0.0 };
                        let BRW = if BRP != 0.0 {
                            let BRQ = GT / (GT + (JH * BRN));
                            BRQ
                        } else {
                            let BRR = GT / (GT - (JH * BRN));
                            BRR
                        };
                        let BRS = (-BRO) + BRM;
                        let BRT = if BRS > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                        let BRY = if BRT != 0.0 {
                            let BRU = BRS.exp();
                            BRU
                        } else {
                            let BRV = NA / (GT + ((-2.3025850929940458e2f64 - BRS) * (GT + (GR * ((-2.3025850929940458e2f64 - BRS) * (GT + ((-2.3025850929940458e2f64 - BRS) * NB)))))));
                            BRV
                        };
                        let BRX = BRW * BRW;
                        let BRZ = (((JG * BRW) + (JJ * BRX)) + (JK * (BRX * BRW))) * BRY;
                        let BSF;
                        if BRP != 0.0 {
                            BSF = BRZ;
                        } else {
                            let BSA = if BRM > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            let BSD = if BSA != 0.0 {
                                let BSB = BRM.exp();
                                BSB
                            } else {
                                let BSC = NA / (GT + ((-2.3025850929940458e2f64 - BRM) * (GT + (GR * ((-2.3025850929940458e2f64 - BRM) * (GT + ((-2.3025850929940458e2f64 - BRM) * NB)))))));
                                BSC
                            };
                            let BSE = (HS * BSD) - BRZ;
                            BSF = BSE;
                        }
                        let BSH = CG * ((BSG * (8.86226925452758e-1f64 * ((JP * BSF) / BRL))) * BRK);
                        BTI = BSH;
                    }
                    let BSI = if CZ == A { 1.0 } else { 0.0 };
                    let BTJ;
                    if BSI != 0.0 {
                        BTJ = A;
                    } else {
                        let BSJ = if AT == GR { 1.0 } else { 0.0 };
                        let BSM = if BSJ != 0.0 {
                            let BSK = ((AF - BOZ) * JE).sqrt();
                            BSK
                        } else {
                            let BSL = ((AF - BOZ) * JE).powf(AT);
                            BSL
                        };
                        let BSN = IS * (((AF - BOZ) * JB) / BSM);
                        let BSO = (-KD) / BSN;
                        let BSP = if (BSO.abs()) < MW { 1.0 } else { 0.0 };
                        let BSV;
                        if BSP != 0.0 {
                            let BSQ = BSO.exp();
                            BSV = BSQ;
                        } else {
                            let BSR = if BSO < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            let BSW = if BSR != 0.0 {
                                let BSS = NA / (GT + ((-2.3025850929940458e2f64 - BSO) * (GT + (GR * ((-2.3025850929940458e2f64 - BSO) * (GT + ((-2.3025850929940458e2f64 - BSO) * NB)))))));
                                BSS
                            } else {
                                let BST = BSO - MW;
                                let BSU = ND * (GT + (BST * (GT + (GR * (BST * (GT + (BST * NB)))))));
                                BSU
                            };
                            BSV = BSW;
                        }
                        let BSX = CZ * (((PF * BSN) * BSN) * BSV);
                        BTJ = BSX;
                    }
                    let BSY = if (if AFI > LG { 1.0 } else { 0.0 }) != 0.0 || (if ABX == A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let BTK;
                    if BSY != 0.0 {
                        BTK = GT;
                    } else {
                        let BSZ = if BPP > ((-KG) * AFI) { 1.0 } else { 0.0 };
                        let BTL;
                        if BSZ != 0.0 {
                            let BTA = if DS == OR { 1.0 } else { 0.0 };
                            let BTE = if BTA != 0.0 {
                                let BTB = (BPP * KX).abs();
                                let BTC = ((BTB * BTB) * BTB) * BTB;
                                BTC
                            } else {
                                let BTD = ((BPP * KX).abs()).powf(DS);
                                BTD
                            };
                            let BTF = GT / (GT - BTE);
                            BTL = BTF;
                        } else {
                            let BTG = KI + ((BPP + (KG * AFI)) * KY);
                            BTL = BTG;
                        }
                        BTK = BTL;
                    }
                    let BTM = (((BQG + BTH) + BTI) + BTJ) * BTK;
                    BWX = BTM;
                }
                let BWY;
                if NM != 0.0 {
                    BWY = A;
                } else {
                    let BTN = if IQ == GR { 1.0 } else { 0.0 };
                    if BTN != 0.0 {
                    } else {
                    }
                    let BTP = IG * BTO;
                    let BTQ = if CJ == A { 1.0 } else { 0.0 };
                    let BTR = if (if BT == A { 1.0 } else { 0.0 }) != 0.0 && BTQ != 0.0 { 1.0 } else { 0.0 };
                    let BUE;
                    let BUF;
                    let BUR;
                    let BVP;
                    let BWQ;
                    if BTR != 0.0 {
                        BUE = A;
                        BUF = A;
                        BUR = A;
                        BVP = A;
                        BWQ = A;
                    } else {
                        let BTS = IN - BMV;
                        let BTT = GT - ((GT - (BMX / BTS)).sqrt());
                        let BTU = if AY == GR { 1.0 } else { 0.0 };
                        let BTW = if BTU != 0.0 {
                            A
                        } else {
                            let BTV = ((((BTT * BTT) * (BTT.ln())) / (GT - BTT)) + BTT) * (GT - (HS * AY));
                            BTV
                        };
                        let BTX = BTT + BTW;
                        let BUA = if BTU != 0.0 {
                            let BTY = (BTS * JF).sqrt();
                            BTY
                        } else {
                            let BTZ = (BTS * JF).powf(AY);
                            BTZ
                        };
                        let BUB = IZ * BUA;
                        let BUC = IA * ((BNH - GT) * BUB);
                        let BUD = BT * (BUC * BTX);
                        BUE = BUB;
                        BUF = BTS;
                        BUR = BTX;
                        BVP = BUC;
                        BWQ = BUD;
                    }
                    let BWR;
                    if BTQ != 0.0 {
                        BWR = A;
                    } else {
                        let BUG = JV * ((BUE * IQ) / BUF);
                        let BUH = (ZT * JQ) / BUG;
                        let BUI = BUH * BUH;
                        let BUJ = BUI * BUI;
                        let BUK = (BUJ / (BUJ + GT)).sqrt();
                        let BUL = (BUK.abs()).sqrt();
                        let BUM = BUK * BUL;
                        let BUN = (-AY) * IT;
                        let BUO = if BUN == -1e0f64 { 1.0 } else { 0.0 };
                        let BUS = if BUO != 0.0 {
                            let BUP = GT / (GT + (BUG * BUM));
                            BUP
                        } else {
                            let BUQ = (GT + (BUG * BUM)).powf(BUN);
                            BUQ
                        };
                        let BUT = (BUR * BUS) / (BUR + BUS);
                        let BUU = (AAH * (BUG / BUL)).sqrt();
                        let BUV = (((JQ * BUH) * BUL) - (JQ * BUK)) + (GR * (BUG * BUM));
                        let BUW = (((HS * (BUH * BUL)) - BUK) - GT) * BUU;
                        let BUX = BUW * BUW;
                        let BUY = if BUW > A { 1.0 } else { 0.0 };
                        let BVF = if BUY != 0.0 {
                            let BUZ = GT / (GT + (JH * BUW));
                            BUZ
                        } else {
                            let BVA = GT / (GT - (JH * BUW));
                            BVA
                        };
                        let BVB = (-BUX) + BUV;
                        let BVC = if BVB > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                        let BVH = if BVC != 0.0 {
                            let BVD = BVB.exp();
                            BVD
                        } else {
                            let BVE = NA / (GT + ((-2.3025850929940458e2f64 - BVB) * (GT + (GR * ((-2.3025850929940458e2f64 - BVB) * (GT + ((-2.3025850929940458e2f64 - BVB) * NB)))))));
                            BVE
                        };
                        let BVG = BVF * BVF;
                        let BVI = (((JG * BVF) + (JJ * BVG)) + (JK * (BVG * BVF))) * BVH;
                        let BVO;
                        if BUY != 0.0 {
                            BVO = BVI;
                        } else {
                            let BVJ = if BUV > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            let BVM = if BVJ != 0.0 {
                                let BVK = BUV.exp();
                                BVK
                            } else {
                                let BVL = NA / (GT + ((-2.3025850929940458e2f64 - BUV) * (GT + (GR * ((-2.3025850929940458e2f64 - BUV) * (GT + ((-2.3025850929940458e2f64 - BUV) * NB)))))));
                                BVL
                            };
                            let BVN = (HS * BVM) - BVI;
                            BVO = BVN;
                        }
                        let BVQ = CJ * ((BVP * (8.86226925452758e-1f64 * ((JQ * BVO) / BUU))) * BUT);
                        BWR = BVQ;
                    }
                    let BVR = if DC == A { 1.0 } else { 0.0 };
                    let BWS;
                    if BVR != 0.0 {
                        BWS = A;
                    } else {
                        let BVS = if AY == GR { 1.0 } else { 0.0 };
                        let BVV = if BVS != 0.0 {
                            let BVT = ((AI - BOZ) * JF).sqrt();
                            BVT
                        } else {
                            let BVU = ((AI - BOZ) * JF).powf(AY);
                            BVU
                        };
                        let BVW = IT * (((AI - BOZ) * JC) / BVV);
                        let BVX = (-KF) / BVW;
                        let BVY = if (BVX.abs()) < MW { 1.0 } else { 0.0 };
                        let BWE;
                        if BVY != 0.0 {
                            let BVZ = BVX.exp();
                            BWE = BVZ;
                        } else {
                            let BWA = if BVX < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            let BWF = if BWA != 0.0 {
                                let BWB = NA / (GT + ((-2.3025850929940458e2f64 - BVX) * (GT + (GR * ((-2.3025850929940458e2f64 - BVX) * (GT + ((-2.3025850929940458e2f64 - BVX) * NB)))))));
                                BWB
                            } else {
                                let BWC = BVX - MW;
                                let BWD = ND * (GT + (BWC * (GT + (GR * (BWC * (GT + (BWC * NB)))))));
                                BWD
                            };
                            BWE = BWF;
                        }
                        let BWG = DC * (((PF * BVW) * BVW) * BWE);
                        BWS = BWG;
                    }
                    let BWH = if (if AIS > LG { 1.0 } else { 0.0 }) != 0.0 || (if ABX == A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let BWT;
                    if BWH != 0.0 {
                        BWT = GT;
                    } else {
                        let BWI = if BPP > ((-KG) * AIS) { 1.0 } else { 0.0 };
                        let BWU;
                        if BWI != 0.0 {
                            let BWJ = if DV == OR { 1.0 } else { 0.0 };
                            let BWN = if BWJ != 0.0 {
                                let BWK = (BPP * KZ).abs();
                                let BWL = ((BWK * BWK) * BWK) * BWK;
                                BWL
                            } else {
                                let BWM = ((BPP * KZ).abs()).powf(DV);
                                BWM
                            };
                            let BWO = GT / (GT - BWN);
                            BWU = BWO;
                        } else {
                            let BWP = KJ + ((BPP + (KG * AIS)) * LA);
                            BWU = BWP;
                        }
                        BWT = BWU;
                    }
                    let BWV = (((BTP + BWQ) + BWR) + BWS) * BWT;
                    BWY = BWV;
                }
                let BWZ = ((LY * BWW) + (MC * BWX)) + (MG * BWY);
                let CGL;
                let CGP;
                let CGR;
                let CHB;
                let CIT;
                let CJJ;
                let CJZ;
                let CNI;
                if PH != 0.0 {
                    let BXA = if DE < MU { 1.0 } else { 0.0 };
                    let CFK;
                    let CFO;
                    let CFS;
                    let CFW;
                    if BXA != 0.0 {
                        let BXB = GR * (DE * HK);
                        let BXC = if (BXB.abs()) < MW { 1.0 } else { 0.0 };
                        let CFX;
                        if BXC != 0.0 {
                            let BXD = BXB.exp();
                            CFX = BXD;
                        } else {
                            let BXE = if BXB < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            let CFY = if BXE != 0.0 {
                                let BXF = NA / (GT + ((-2.3025850929940458e2f64 - BXB) * (GT + (GR * ((-2.3025850929940458e2f64 - BXB) * (GT + ((-2.3025850929940458e2f64 - BXB) * NB)))))));
                                BXF
                            } else {
                                let BXG = BXB - MW;
                                let BXH = ND * (GT + (BXG * (GT + (GR * (BXG * (GT + (BXG * NB)))))));
                                BXH
                            };
                            CFX = CFY;
                        }
                        let BXI = if EV < PQ { 1.0 } else { 0.0 };
                        let BYC;
                        let BYD;
                        if BXI != 0.0 {
                            let BXJ = EV - (PS * LS);
                            let BXK = (PQ - ((PS * (DE - LS)) + EV)) - CL;
                            let BXL = (OR * PQ) * CL;
                            let BXM = if BXL > A { 1.0 } else { 0.0 };
                            let BXO = if BXM != 0.0 {
                                BXL
                            } else {
                                let BXN = -BXL;
                                BXN
                            };
                            let BXP = ((PQ - (GR * (BXK + (((BXK * BXK) + BXO).sqrt())))) - EV) - CL;
                            let BXQ = (OR * EV) * CL;
                            let BXR = if BXQ > A { 1.0 } else { 0.0 };
                            let BXT = if BXR != 0.0 {
                                BXQ
                            } else {
                                let BXS = -BXQ;
                                BXS
                            };
                            let BXU = EV + (GR * (BXP + (((BXP * BXP) + BXT).sqrt())));
                            let BXV = (PQ - BXJ) - CL;
                            let BXX = if BXM != 0.0 {
                                BXL
                            } else {
                                let BXW = -BXL;
                                BXW
                            };
                            let BXY = ((PQ - (GR * (BXV + (((BXV * BXV) + BXX).sqrt())))) - EV) - CL;
                            let BYA = if BXR != 0.0 {
                                BXQ
                            } else {
                                let BXZ = -BXQ;
                                BXZ
                            };
                            let BYB = EV + (GR * (BXY + (((BXY * BXY) + BYA).sqrt())));
                            BYC = BXU;
                            BYD = BYB;
                        } else {
                            BYC = EV;
                            BYD = EV;
                        }
                        let BYE = HK * ((DE / BYC) + ((LS * (BYC - BYD)) / (BYD * PQ)));
                        let BYF = if (BYE.abs()) < MW { 1.0 } else { 0.0 };
                        let CFL;
                        if BYF != 0.0 {
                            let BYG = BYE.exp();
                            CFL = BYG;
                        } else {
                            let BYH = if BYE < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            let CFM = if BYH != 0.0 {
                                let BYI = NA / (GT + ((-2.3025850929940458e2f64 - BYE) * (GT + (GR * ((-2.3025850929940458e2f64 - BYE) * (GT + ((-2.3025850929940458e2f64 - BYE) * NB)))))));
                                BYI
                            } else {
                                let BYJ = BYE - MW;
                                let BYK = ND * (GT + (BYJ * (GT + (GR * (BYJ * (GT + (BYJ * NB)))))));
                                BYK
                            };
                            CFL = CFM;
                        }
                        let BYL = (EY / HK) * ((LI / (LM / LI)).ln());
                        let BYM = if EY < PQ { 1.0 } else { 0.0 };
                        let BZG;
                        let BZH;
                        if BYM != 0.0 {
                            let BYN = EY - (PS * BYL);
                            let BYO = (PQ - ((PS * (DE - BYL)) + EY)) - CL;
                            let BYP = (OR * PQ) * CL;
                            let BYQ = if BYP > A { 1.0 } else { 0.0 };
                            let BYS = if BYQ != 0.0 {
                                BYP
                            } else {
                                let BYR = -BYP;
                                BYR
                            };
                            let BYT = ((PQ - (GR * (BYO + (((BYO * BYO) + BYS).sqrt())))) - EY) - CL;
                            let BYU = (OR * EY) * CL;
                            let BYV = if BYU > A { 1.0 } else { 0.0 };
                            let BYX = if BYV != 0.0 {
                                BYU
                            } else {
                                let BYW = -BYU;
                                BYW
                            };
                            let BYY = EY + (GR * (BYT + (((BYT * BYT) + BYX).sqrt())));
                            let BYZ = (PQ - BYN) - CL;
                            let BZB = if BYQ != 0.0 {
                                BYP
                            } else {
                                let BZA = -BYP;
                                BZA
                            };
                            let BZC = ((PQ - (GR * (BYZ + (((BYZ * BYZ) + BZB).sqrt())))) - EY) - CL;
                            let BZE = if BYV != 0.0 {
                                BYU
                            } else {
                                let BZD = -BYU;
                                BZD
                            };
                            let BZF = EY + (GR * (BZC + (((BZC * BZC) + BZE).sqrt())));
                            BZG = BYY;
                            BZH = BZF;
                        } else {
                            BZG = EY;
                            BZH = EY;
                        }
                        let BZI = HK * ((DE / BZG) + ((BYL * (BZG - BZH)) / (BZH * PQ)));
                        let BZJ = if (BZI.abs()) < MW { 1.0 } else { 0.0 };
                        let CFP;
                        if BZJ != 0.0 {
                            let BZK = BZI.exp();
                            CFP = BZK;
                        } else {
                            let BZL = if BZI < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            let CFQ = if BZL != 0.0 {
                                let BZM = NA / (GT + ((-2.3025850929940458e2f64 - BZI) * (GT + (GR * ((-2.3025850929940458e2f64 - BZI) * (GT + ((-2.3025850929940458e2f64 - BZI) * NB)))))));
                                BZM
                            } else {
                                let BZN = BZI - MW;
                                let BZO = ND * (GT + (BZN * (GT + (GR * (BZN * (GT + (BZN * NB)))))));
                                BZO
                            };
                            CFP = CFQ;
                        }
                        let BZP = (FB / HK) * ((LJ / (LM / LJ)).ln());
                        let BZQ = if FB < PQ { 1.0 } else { 0.0 };
                        let CAK;
                        let CAL;
                        if BZQ != 0.0 {
                            let BZR = FB - (PS * BZP);
                            let BZS = (PQ - ((PS * (DE - BZP)) + FB)) - CL;
                            let BZT = (OR * PQ) * CL;
                            let BZU = if BZT > A { 1.0 } else { 0.0 };
                            let BZW = if BZU != 0.0 {
                                BZT
                            } else {
                                let BZV = -BZT;
                                BZV
                            };
                            let BZX = ((PQ - (GR * (BZS + (((BZS * BZS) + BZW).sqrt())))) - FB) - CL;
                            let BZY = (OR * FB) * CL;
                            let BZZ = if BZY > A { 1.0 } else { 0.0 };
                            let CAB = if BZZ != 0.0 {
                                BZY
                            } else {
                                let CAA = -BZY;
                                CAA
                            };
                            let CAC = FB + (GR * (BZX + (((BZX * BZX) + CAB).sqrt())));
                            let CAD = (PQ - BZR) - CL;
                            let CAF = if BZU != 0.0 {
                                BZT
                            } else {
                                let CAE = -BZT;
                                CAE
                            };
                            let CAG = ((PQ - (GR * (CAD + (((CAD * CAD) + CAF).sqrt())))) - FB) - CL;
                            let CAI = if BZZ != 0.0 {
                                BZY
                            } else {
                                let CAH = -BZY;
                                CAH
                            };
                            let CAJ = FB + (GR * (CAG + (((CAG * CAG) + CAI).sqrt())));
                            CAK = CAC;
                            CAL = CAJ;
                        } else {
                            CAK = FB;
                            CAL = FB;
                        }
                        let CAM = HK * ((DE / CAK) + ((BZP * (CAK - CAL)) / (CAL * PQ)));
                        let CAN = if (CAM.abs()) < MW { 1.0 } else { 0.0 };
                        let CFT;
                        if CAN != 0.0 {
                            let CAO = CAM.exp();
                            CFT = CAO;
                        } else {
                            let CAP = if CAM < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            let CFU = if CAP != 0.0 {
                                let CAQ = NA / (GT + ((-2.3025850929940458e2f64 - CAM) * (GT + (GR * ((-2.3025850929940458e2f64 - CAM) * (GT + ((-2.3025850929940458e2f64 - CAM) * NB)))))));
                                CAQ
                            } else {
                                let CAR = CAM - MW;
                                let CAS = ND * (GT + (CAR * (GT + (GR * (CAR * (GT + (CAR * NB)))))));
                                CAS
                            };
                            CFT = CFU;
                        }
                        CFK = CFL;
                        CFO = CFP;
                        CFS = CFT;
                        CFW = CFX;
                    } else {
                        let CAT = DE - MU;
                        let CAU = ((GT + (CAT * HK)) * TE).sqrt();
                        let CAV = if EV < PQ { 1.0 } else { 0.0 };
                        let CBU;
                        let CBV;
                        let CCE;
                        if CAV != 0.0 {
                            let CAW = EV - (PS * LS);
                            let CAX = (PQ - ((PS * (MU - LS)) + EV)) - CL;
                            let CAY = (OR * PQ) * CL;
                            let CAZ = if CAY > A { 1.0 } else { 0.0 };
                            let CBB = if CAZ != 0.0 {
                                CAY
                            } else {
                                let CBA = -CAY;
                                CBA
                            };
                            let CBC = ((CAX * CAX) + CBB).sqrt();
                            let CBD = GR * (GT + (CAX / CBC));
                            let CBE = ((PQ - (GR * (CAX + CBC))) - EV) - CL;
                            let CBF = (OR * EV) * CL;
                            let CBG = if CBF > A { 1.0 } else { 0.0 };
                            let CBI = if CBG != 0.0 {
                                CBF
                            } else {
                                let CBH = -CBF;
                                CBH
                            };
                            let CBJ = ((CBE * CBE) + CBI).sqrt();
                            let CBK = GR * (GT + (CBE / CBJ));
                            let CBL = EV + (GR * (CBE + CBJ));
                            let CBM = (PQ - CAW) - CL;
                            let CBO = if CAZ != 0.0 {
                                CAY
                            } else {
                                let CBN = -CAY;
                                CBN
                            };
                            let CBP = ((PQ - (GR * (CBM + (((CBM * CBM) + CBO).sqrt())))) - EV) - CL;
                            let CBR = if CBG != 0.0 {
                                CBF
                            } else {
                                let CBQ = -CBF;
                                CBQ
                            };
                            let CBS = EV + (GR * (CBP + (((CBP * CBP) + CBR).sqrt())));
                            let CBT = (PS * CBD) * CBK;
                            CBU = CBL;
                            CBV = CBS;
                            CCE = CBT;
                        } else {
                            CBU = EV;
                            CBV = EV;
                            CCE = A;
                        }
                        let CBW = CBV * PQ;
                        let CBX = HK * ((MU / CBU) + ((LS * (CBU - CBV)) / CBW));
                        let CBY = if (CBX.abs()) < MW { 1.0 } else { 0.0 };
                        let CCF;
                        if CBY != 0.0 {
                            let CBZ = CBX.exp();
                            CCF = CBZ;
                        } else {
                            let CCA = if CBX < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            let CCG = if CCA != 0.0 {
                                let CCB = NA / (GT + ((-2.3025850929940458e2f64 - CBX) * (GT + (GR * ((-2.3025850929940458e2f64 - CBX) * (GT + ((-2.3025850929940458e2f64 - CBX) * NB)))))));
                                CCB
                            } else {
                                let CCC = CBX - MW;
                                let CCD = ND * (GT + (CCC * (GT + (GR * (CCC * (GT + (CCC * NB)))))));
                                CCD
                            };
                            CCF = CCG;
                        }
                        let CCH = (GT + (CAT * (HK * (((CBU - (MU * CCE)) / (CBU * CBU)) + ((LS * CCE) / CBW))))) * CCF;
                        let CCI = (EY / HK) * ((LI / (LM / LI)).ln());
                        let CCJ = if EY < PQ { 1.0 } else { 0.0 };
                        let CDI;
                        let CDJ;
                        let CDS;
                        if CCJ != 0.0 {
                            let CCK = EY - (PS * CCI);
                            let CCL = (PQ - ((PS * (MU - CCI)) + EY)) - CL;
                            let CCM = (OR * PQ) * CL;
                            let CCN = if CCM > A { 1.0 } else { 0.0 };
                            let CCP = if CCN != 0.0 {
                                CCM
                            } else {
                                let CCO = -CCM;
                                CCO
                            };
                            let CCQ = ((CCL * CCL) + CCP).sqrt();
                            let CCR = GR * (GT + (CCL / CCQ));
                            let CCS = ((PQ - (GR * (CCL + CCQ))) - EY) - CL;
                            let CCT = (OR * EY) * CL;
                            let CCU = if CCT > A { 1.0 } else { 0.0 };
                            let CCW = if CCU != 0.0 {
                                CCT
                            } else {
                                let CCV = -CCT;
                                CCV
                            };
                            let CCX = ((CCS * CCS) + CCW).sqrt();
                            let CCY = GR * (GT + (CCS / CCX));
                            let CCZ = EY + (GR * (CCS + CCX));
                            let CDA = (PQ - CCK) - CL;
                            let CDC = if CCN != 0.0 {
                                CCM
                            } else {
                                let CDB = -CCM;
                                CDB
                            };
                            let CDD = ((PQ - (GR * (CDA + (((CDA * CDA) + CDC).sqrt())))) - EY) - CL;
                            let CDF = if CCU != 0.0 {
                                CCT
                            } else {
                                let CDE = -CCT;
                                CDE
                            };
                            let CDG = EY + (GR * (CDD + (((CDD * CDD) + CDF).sqrt())));
                            let CDH = (PS * CCR) * CCY;
                            CDI = CCZ;
                            CDJ = CDG;
                            CDS = CDH;
                        } else {
                            CDI = EY;
                            CDJ = EY;
                            CDS = A;
                        }
                        let CDK = CDJ * PQ;
                        let CDL = HK * ((MU / CDI) + ((CCI * (CDI - CDJ)) / CDK));
                        let CDM = if (CDL.abs()) < MW { 1.0 } else { 0.0 };
                        let CDT;
                        if CDM != 0.0 {
                            let CDN = CDL.exp();
                            CDT = CDN;
                        } else {
                            let CDO = if CDL < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            let CDU = if CDO != 0.0 {
                                let CDP = NA / (GT + ((-2.3025850929940458e2f64 - CDL) * (GT + (GR * ((-2.3025850929940458e2f64 - CDL) * (GT + ((-2.3025850929940458e2f64 - CDL) * NB)))))));
                                CDP
                            } else {
                                let CDQ = CDL - MW;
                                let CDR = ND * (GT + (CDQ * (GT + (GR * (CDQ * (GT + (CDQ * NB)))))));
                                CDR
                            };
                            CDT = CDU;
                        }
                        let CDV = (GT + (CAT * (HK * (((CDI - (MU * CDS)) / (CDI * CDI)) + ((CCI * CDS) / CDK))))) * CDT;
                        let CDW = (FB / HK) * ((LJ / (LM / LJ)).ln());
                        let CDX = if FB < PQ { 1.0 } else { 0.0 };
                        let CEW;
                        let CEX;
                        let CFG;
                        if CDX != 0.0 {
                            let CDY = FB - (PS * CDW);
                            let CDZ = (PQ - ((PS * (MU - CDW)) + FB)) - CL;
                            let CEA = (OR * PQ) * CL;
                            let CEB = if CEA > A { 1.0 } else { 0.0 };
                            let CED = if CEB != 0.0 {
                                CEA
                            } else {
                                let CEC = -CEA;
                                CEC
                            };
                            let CEE = ((CDZ * CDZ) + CED).sqrt();
                            let CEF = GR * (GT + (CDZ / CEE));
                            let CEG = ((PQ - (GR * (CDZ + CEE))) - FB) - CL;
                            let CEH = (OR * FB) * CL;
                            let CEI = if CEH > A { 1.0 } else { 0.0 };
                            let CEK = if CEI != 0.0 {
                                CEH
                            } else {
                                let CEJ = -CEH;
                                CEJ
                            };
                            let CEL = ((CEG * CEG) + CEK).sqrt();
                            let CEM = GR * (GT + (CEG / CEL));
                            let CEN = FB + (GR * (CEG + CEL));
                            let CEO = (PQ - CDY) - CL;
                            let CEQ = if CEB != 0.0 {
                                CEA
                            } else {
                                let CEP = -CEA;
                                CEP
                            };
                            let CER = ((PQ - (GR * (CEO + (((CEO * CEO) + CEQ).sqrt())))) - FB) - CL;
                            let CET = if CEI != 0.0 {
                                CEH
                            } else {
                                let CES = -CEH;
                                CES
                            };
                            let CEU = FB + (GR * (CER + (((CER * CER) + CET).sqrt())));
                            let CEV = (PS * CEF) * CEM;
                            CEW = CEN;
                            CEX = CEU;
                            CFG = CEV;
                        } else {
                            CEW = FB;
                            CEX = FB;
                            CFG = A;
                        }
                        let CEY = CEX * PQ;
                        let CEZ = HK * ((MU / CEW) + ((CDW * (CEW - CEX)) / CEY));
                        let CFA = if (CEZ.abs()) < MW { 1.0 } else { 0.0 };
                        let CFH;
                        if CFA != 0.0 {
                            let CFB = CEZ.exp();
                            CFH = CFB;
                        } else {
                            let CFC = if CEZ < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            let CFI = if CFC != 0.0 {
                                let CFD = NA / (GT + ((-2.3025850929940458e2f64 - CEZ) * (GT + (GR * ((-2.3025850929940458e2f64 - CEZ) * (GT + ((-2.3025850929940458e2f64 - CEZ) * NB)))))));
                                CFD
                            } else {
                                let CFE = CEZ - MW;
                                let CFF = ND * (GT + (CFE * (GT + (GR * (CFE * (GT + (CFE * NB)))))));
                                CFF
                            };
                            CFH = CFI;
                        }
                        let CFJ = (GT + (CAT * (HK * (((CEW - (MU * CFG)) / (CEW * CEW)) + ((CDW * CFG) / CEY))))) * CFH;
                        CFK = CCH;
                        CFO = CDV;
                        CFS = CFJ;
                        CFW = CAU;
                    }
                    let CFN = CFK - GT;
                    let CFR = CFO - GT;
                    let CFV = CFS - GT;
                    let CFZ = GT / CFW;
                    let CGD = if CGA != 0.0 {
                        let CGB = HS * (HJ * (((HS + CFZ) + (((CFZ + GT) * (CFZ + JI)).sqrt())).ln()));
                        CGB
                    } else {
                        let CGC = -1e-1f64 + (HS * (HJ * ((((HS * CFW) + GT) + (((GT + CFW) * (GT + (JI * CFW))).sqrt())).ln())));
                        CGC
                    };
                    let CGE = NS - CGD;
                    let CGF = DE - CGE;
                    let CGG = GR * ((DE + CGE) - (((CGF * CGF) + ((OR * HJ) * HJ)).sqrt()));
                    let CGH = DE - NW;
                    let CGI = GR * ((DE + NW) - (((CGH * CGH) + ((OR * HH) * HH)).sqrt()));
                    CGL = CFN;
                    CGP = CGG;
                    CGR = CGD;
                    CHB = CFW;
                    CIT = CGI;
                    CJJ = CGJ;
                    CJZ = CFR;
                    CNI = CFV;
                } else {
                    CGL = A;
                    CGP = A;
                    CGR = A;
                    CHB = A;
                    CIT = A;
                    CJJ = A;
                    CJZ = A;
                    CNI = A;
                }
                let CQQ;
                if NG != 0.0 {
                    CQQ = A;
                } else {
                    let CGK = if IO == GR { 1.0 } else { 0.0 };
                    if CGK != 0.0 {
                    } else {
                    }
                    let CGM = IE * CGL;
                    let CGN = if CD == A { 1.0 } else { 0.0 };
                    let CGO = if (if BN == A { 1.0 } else { 0.0 }) != 0.0 && CGN != 0.0 { 1.0 } else { 0.0 };
                    let CHE;
                    let CHF;
                    let CHR;
                    let CIP;
                    let CJS;
                    if CGO != 0.0 {
                        CHE = A;
                        CHF = A;
                        CHR = A;
                        CIP = A;
                        CJS = A;
                    } else {
                        let CGQ = IL - CGP;
                        let CGS = GT - ((GT - (CGR / CGQ)).sqrt());
                        let CGT = if AO == GR { 1.0 } else { 0.0 };
                        let CGV = if CGT != 0.0 {
                            A
                        } else {
                            let CGU = ((((CGS * CGS) * (CGS.ln())) / (GT - CGS)) + CGS) * (GT - (HS * AO));
                            CGU
                        };
                        let CGW = CGS + CGV;
                        let CGZ = if CGT != 0.0 {
                            let CGX = (CGQ * JD).sqrt();
                            CGX
                        } else {
                            let CGY = (CGQ * JD).powf(AO);
                            CGY
                        };
                        let CHA = IX * CGZ;
                        let CHC = HW * ((CHB - GT) * CHA);
                        let CHD = BN * (CHC * CGW);
                        CHE = CHA;
                        CHF = CGQ;
                        CHR = CGW;
                        CIP = CHC;
                        CJS = CHD;
                    }
                    let CJT;
                    if CGN != 0.0 {
                        CJT = A;
                    } else {
                        let CHG = JT * ((CHE * IO) / CHF);
                        let CHH = (ZT * JO) / CHG;
                        let CHI = CHH * CHH;
                        let CHJ = CHI * CHI;
                        let CHK = (CHJ / (CHJ + GT)).sqrt();
                        let CHL = (CHK.abs()).sqrt();
                        let CHM = CHK * CHL;
                        let CHN = (-AO) * IR;
                        let CHO = if CHN == -1e0f64 { 1.0 } else { 0.0 };
                        let CHS = if CHO != 0.0 {
                            let CHP = GT / (GT + (CHG * CHM));
                            CHP
                        } else {
                            let CHQ = (GT + (CHG * CHM)).powf(CHN);
                            CHQ
                        };
                        let CHT = (CHR * CHS) / (CHR + CHS);
                        let CHU = (AAH * (CHG / CHL)).sqrt();
                        let CHV = (((JO * CHH) * CHL) - (JO * CHK)) + (GR * (CHG * CHM));
                        let CHW = (((HS * (CHH * CHL)) - CHK) - GT) * CHU;
                        let CHX = CHW * CHW;
                        let CHY = if CHW > A { 1.0 } else { 0.0 };
                        let CIF = if CHY != 0.0 {
                            let CHZ = GT / (GT + (JH * CHW));
                            CHZ
                        } else {
                            let CIA = GT / (GT - (JH * CHW));
                            CIA
                        };
                        let CIB = (-CHX) + CHV;
                        let CIC = if CIB > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                        let CIH = if CIC != 0.0 {
                            let CID = CIB.exp();
                            CID
                        } else {
                            let CIE = NA / (GT + ((-2.3025850929940458e2f64 - CIB) * (GT + (GR * ((-2.3025850929940458e2f64 - CIB) * (GT + ((-2.3025850929940458e2f64 - CIB) * NB)))))));
                            CIE
                        };
                        let CIG = CIF * CIF;
                        let CII = (((JG * CIF) + (JJ * CIG)) + (JK * (CIG * CIF))) * CIH;
                        let CIO;
                        if CHY != 0.0 {
                            CIO = CII;
                        } else {
                            let CIJ = if CHV > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            let CIM = if CIJ != 0.0 {
                                let CIK = CHV.exp();
                                CIK
                            } else {
                                let CIL = NA / (GT + ((-2.3025850929940458e2f64 - CHV) * (GT + (GR * ((-2.3025850929940458e2f64 - CHV) * (GT + ((-2.3025850929940458e2f64 - CHV) * NB)))))));
                                CIL
                            };
                            let CIN = (HS * CIM) - CII;
                            CIO = CIN;
                        }
                        let CIQ = CD * ((CIP * (8.86226925452758e-1f64 * ((JO * CIO) / CHU))) * CHT);
                        CJT = CIQ;
                    }
                    let CIR = if CW == A { 1.0 } else { 0.0 };
                    let CJU;
                    if CIR != 0.0 {
                        CJU = A;
                    } else {
                        let CIS = if AO == GR { 1.0 } else { 0.0 };
                        let CIW = if CIS != 0.0 {
                            let CIU = ((AC - CIT) * JD).sqrt();
                            CIU
                        } else {
                            let CIV = ((AC - CIT) * JD).powf(AO);
                            CIV
                        };
                        let CIX = IR * (((AC - CIT) * JA) / CIW);
                        let CIY = (-KB) / CIX;
                        let CIZ = if (CIY.abs()) < MW { 1.0 } else { 0.0 };
                        let CJF;
                        if CIZ != 0.0 {
                            let CJA = CIY.exp();
                            CJF = CJA;
                        } else {
                            let CJB = if CIY < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            let CJG = if CJB != 0.0 {
                                let CJC = NA / (GT + ((-2.3025850929940458e2f64 - CIY) * (GT + (GR * ((-2.3025850929940458e2f64 - CIY) * (GT + ((-2.3025850929940458e2f64 - CIY) * NB)))))));
                                CJC
                            } else {
                                let CJD = CIY - MW;
                                let CJE = ND * (GT + (CJD * (GT + (GR * (CJD * (GT + (CJD * NB)))))));
                                CJE
                            };
                            CJF = CJG;
                        }
                        let CJH = CW * (((DE * CIX) * CIX) * CJF);
                        CJU = CJH;
                    }
                    let CJI = if (if ABW > LG { 1.0 } else { 0.0 }) != 0.0 || (if ABX == A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let CJV;
                    if CJI != 0.0 {
                        CJV = GT;
                    } else {
                        let CJK = if CJJ > ((-KG) * ABW) { 1.0 } else { 0.0 };
                        let CJW;
                        if CJK != 0.0 {
                            let CJL = if DP == OR { 1.0 } else { 0.0 };
                            let CJP = if CJL != 0.0 {
                                let CJM = (CJJ * KV).abs();
                                let CJN = ((CJM * CJM) * CJM) * CJM;
                                CJN
                            } else {
                                let CJO = ((CJJ * KV).abs()).powf(DP);
                                CJO
                            };
                            let CJQ = GT / (GT - CJP);
                            CJW = CJQ;
                        } else {
                            let CJR = KH + ((CJJ + (KG * ABW)) * KW);
                            CJW = CJR;
                        }
                        CJV = CJW;
                    }
                    let CJX = (((CGM + CJS) + CJT) + CJU) * CJV;
                    CQQ = CJX;
                }
                let CQR;
                if NJ != 0.0 {
                    CQR = A;
                } else {
                    let CJY = if IP == GR { 1.0 } else { 0.0 };
                    if CJY != 0.0 {
                    } else {
                    }
                    let CKA = IF * CJZ;
                    let CKB = if CG == A { 1.0 } else { 0.0 };
                    let CKC = if (if BQ == A { 1.0 } else { 0.0 }) != 0.0 && CKB != 0.0 { 1.0 } else { 0.0 };
                    let CKP;
                    let CKQ;
                    let CLC;
                    let CMA;
                    let CNB;
                    if CKC != 0.0 {
                        CKP = A;
                        CKQ = A;
                        CLC = A;
                        CMA = A;
                        CNB = A;
                    } else {
                        let CKD = IM - CGP;
                        let CKE = GT - ((GT - (CGR / CKD)).sqrt());
                        let CKF = if AT == GR { 1.0 } else { 0.0 };
                        let CKH = if CKF != 0.0 {
                            A
                        } else {
                            let CKG = ((((CKE * CKE) * (CKE.ln())) / (GT - CKE)) + CKE) * (GT - (HS * AT));
                            CKG
                        };
                        let CKI = CKE + CKH;
                        let CKL = if CKF != 0.0 {
                            let CKJ = (CKD * JE).sqrt();
                            CKJ
                        } else {
                            let CKK = (CKD * JE).powf(AT);
                            CKK
                        };
                        let CKM = IY * CKL;
                        let CKN = HY * ((CHB - GT) * CKM);
                        let CKO = BQ * (CKN * CKI);
                        CKP = CKM;
                        CKQ = CKD;
                        CLC = CKI;
                        CMA = CKN;
                        CNB = CKO;
                    }
                    let CNC;
                    if CKB != 0.0 {
                        CNC = A;
                    } else {
                        let CKR = JU * ((CKP * IP) / CKQ);
                        let CKS = (ZT * JP) / CKR;
                        let CKT = CKS * CKS;
                        let CKU = CKT * CKT;
                        let CKV = (CKU / (CKU + GT)).sqrt();
                        let CKW = (CKV.abs()).sqrt();
                        let CKX = CKV * CKW;
                        let CKY = (-AT) * IS;
                        let CKZ = if CKY == -1e0f64 { 1.0 } else { 0.0 };
                        let CLD = if CKZ != 0.0 {
                            let CLA = GT / (GT + (CKR * CKX));
                            CLA
                        } else {
                            let CLB = (GT + (CKR * CKX)).powf(CKY);
                            CLB
                        };
                        let CLE = (CLC * CLD) / (CLC + CLD);
                        let CLF = (AAH * (CKR / CKW)).sqrt();
                        let CLG = (((JP * CKS) * CKW) - (JP * CKV)) + (GR * (CKR * CKX));
                        let CLH = (((HS * (CKS * CKW)) - CKV) - GT) * CLF;
                        let CLI = CLH * CLH;
                        let CLJ = if CLH > A { 1.0 } else { 0.0 };
                        let CLQ = if CLJ != 0.0 {
                            let CLK = GT / (GT + (JH * CLH));
                            CLK
                        } else {
                            let CLL = GT / (GT - (JH * CLH));
                            CLL
                        };
                        let CLM = (-CLI) + CLG;
                        let CLN = if CLM > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                        let CLS = if CLN != 0.0 {
                            let CLO = CLM.exp();
                            CLO
                        } else {
                            let CLP = NA / (GT + ((-2.3025850929940458e2f64 - CLM) * (GT + (GR * ((-2.3025850929940458e2f64 - CLM) * (GT + ((-2.3025850929940458e2f64 - CLM) * NB)))))));
                            CLP
                        };
                        let CLR = CLQ * CLQ;
                        let CLT = (((JG * CLQ) + (JJ * CLR)) + (JK * (CLR * CLQ))) * CLS;
                        let CLZ;
                        if CLJ != 0.0 {
                            CLZ = CLT;
                        } else {
                            let CLU = if CLG > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            let CLX = if CLU != 0.0 {
                                let CLV = CLG.exp();
                                CLV
                            } else {
                                let CLW = NA / (GT + ((-2.3025850929940458e2f64 - CLG) * (GT + (GR * ((-2.3025850929940458e2f64 - CLG) * (GT + ((-2.3025850929940458e2f64 - CLG) * NB)))))));
                                CLW
                            };
                            let CLY = (HS * CLX) - CLT;
                            CLZ = CLY;
                        }
                        let CMB = CG * ((CMA * (8.86226925452758e-1f64 * ((JP * CLZ) / CLF))) * CLE);
                        CNC = CMB;
                    }
                    let CMC = if CZ == A { 1.0 } else { 0.0 };
                    let CND;
                    if CMC != 0.0 {
                        CND = A;
                    } else {
                        let CMD = if AT == GR { 1.0 } else { 0.0 };
                        let CMG = if CMD != 0.0 {
                            let CME = ((AF - CIT) * JE).sqrt();
                            CME
                        } else {
                            let CMF = ((AF - CIT) * JE).powf(AT);
                            CMF
                        };
                        let CMH = IS * (((AF - CIT) * JB) / CMG);
                        let CMI = (-KD) / CMH;
                        let CMJ = if (CMI.abs()) < MW { 1.0 } else { 0.0 };
                        let CMP;
                        if CMJ != 0.0 {
                            let CMK = CMI.exp();
                            CMP = CMK;
                        } else {
                            let CML = if CMI < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            let CMQ = if CML != 0.0 {
                                let CMM = NA / (GT + ((-2.3025850929940458e2f64 - CMI) * (GT + (GR * ((-2.3025850929940458e2f64 - CMI) * (GT + ((-2.3025850929940458e2f64 - CMI) * NB)))))));
                                CMM
                            } else {
                                let CMN = CMI - MW;
                                let CMO = ND * (GT + (CMN * (GT + (GR * (CMN * (GT + (CMN * NB)))))));
                                CMO
                            };
                            CMP = CMQ;
                        }
                        let CMR = CZ * (((DE * CMH) * CMH) * CMP);
                        CND = CMR;
                    }
                    let CMS = if (if AFI > LG { 1.0 } else { 0.0 }) != 0.0 || (if ABX == A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let CNE;
                    if CMS != 0.0 {
                        CNE = GT;
                    } else {
                        let CMT = if CJJ > ((-KG) * AFI) { 1.0 } else { 0.0 };
                        let CNF;
                        if CMT != 0.0 {
                            let CMU = if DS == OR { 1.0 } else { 0.0 };
                            let CMY = if CMU != 0.0 {
                                let CMV = (CJJ * KX).abs();
                                let CMW = ((CMV * CMV) * CMV) * CMV;
                                CMW
                            } else {
                                let CMX = ((CJJ * KX).abs()).powf(DS);
                                CMX
                            };
                            let CMZ = GT / (GT - CMY);
                            CNF = CMZ;
                        } else {
                            let CNA = KI + ((CJJ + (KG * AFI)) * KY);
                            CNF = CNA;
                        }
                        CNE = CNF;
                    }
                    let CNG = (((CKA + CNB) + CNC) + CND) * CNE;
                    CQR = CNG;
                }
                let CQS;
                if NM != 0.0 {
                    CQS = A;
                } else {
                    let CNH = if IQ == GR { 1.0 } else { 0.0 };
                    if CNH != 0.0 {
                    } else {
                    }
                    let CNJ = IG * CNI;
                    let CNK = if CJ == A { 1.0 } else { 0.0 };
                    let CNL = if (if BT == A { 1.0 } else { 0.0 }) != 0.0 && CNK != 0.0 { 1.0 } else { 0.0 };
                    let CNY;
                    let CNZ;
                    let COL;
                    let CPJ;
                    let CQK;
                    if CNL != 0.0 {
                        CNY = A;
                        CNZ = A;
                        COL = A;
                        CPJ = A;
                        CQK = A;
                    } else {
                        let CNM = IN - CGP;
                        let CNN = GT - ((GT - (CGR / CNM)).sqrt());
                        let CNO = if AY == GR { 1.0 } else { 0.0 };
                        let CNQ = if CNO != 0.0 {
                            A
                        } else {
                            let CNP = ((((CNN * CNN) * (CNN.ln())) / (GT - CNN)) + CNN) * (GT - (HS * AY));
                            CNP
                        };
                        let CNR = CNN + CNQ;
                        let CNU = if CNO != 0.0 {
                            let CNS = (CNM * JF).sqrt();
                            CNS
                        } else {
                            let CNT = (CNM * JF).powf(AY);
                            CNT
                        };
                        let CNV = IZ * CNU;
                        let CNW = IA * ((CHB - GT) * CNV);
                        let CNX = BT * (CNW * CNR);
                        CNY = CNV;
                        CNZ = CNM;
                        COL = CNR;
                        CPJ = CNW;
                        CQK = CNX;
                    }
                    let CQL;
                    if CNK != 0.0 {
                        CQL = A;
                    } else {
                        let COA = JV * ((CNY * IQ) / CNZ);
                        let COB = (ZT * JQ) / COA;
                        let COC = COB * COB;
                        let COD = COC * COC;
                        let COE = (COD / (COD + GT)).sqrt();
                        let COF = (COE.abs()).sqrt();
                        let COG = COE * COF;
                        let COH = (-AY) * IT;
                        let COI = if COH == -1e0f64 { 1.0 } else { 0.0 };
                        let COM = if COI != 0.0 {
                            let COJ = GT / (GT + (COA * COG));
                            COJ
                        } else {
                            let COK = (GT + (COA * COG)).powf(COH);
                            COK
                        };
                        let CON = (COL * COM) / (COL + COM);
                        let COO = (AAH * (COA / COF)).sqrt();
                        let COP = (((JQ * COB) * COF) - (JQ * COE)) + (GR * (COA * COG));
                        let COQ = (((HS * (COB * COF)) - COE) - GT) * COO;
                        let COR = COQ * COQ;
                        let COS = if COQ > A { 1.0 } else { 0.0 };
                        let COZ = if COS != 0.0 {
                            let COT = GT / (GT + (JH * COQ));
                            COT
                        } else {
                            let COU = GT / (GT - (JH * COQ));
                            COU
                        };
                        let COV = (-COR) + COP;
                        let COW = if COV > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                        let CPB = if COW != 0.0 {
                            let COX = COV.exp();
                            COX
                        } else {
                            let COY = NA / (GT + ((-2.3025850929940458e2f64 - COV) * (GT + (GR * ((-2.3025850929940458e2f64 - COV) * (GT + ((-2.3025850929940458e2f64 - COV) * NB)))))));
                            COY
                        };
                        let CPA = COZ * COZ;
                        let CPC = (((JG * COZ) + (JJ * CPA)) + (JK * (CPA * COZ))) * CPB;
                        let CPI;
                        if COS != 0.0 {
                            CPI = CPC;
                        } else {
                            let CPD = if COP > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            let CPG = if CPD != 0.0 {
                                let CPE = COP.exp();
                                CPE
                            } else {
                                let CPF = NA / (GT + ((-2.3025850929940458e2f64 - COP) * (GT + (GR * ((-2.3025850929940458e2f64 - COP) * (GT + ((-2.3025850929940458e2f64 - COP) * NB)))))));
                                CPF
                            };
                            let CPH = (HS * CPG) - CPC;
                            CPI = CPH;
                        }
                        let CPK = CJ * ((CPJ * (8.86226925452758e-1f64 * ((JQ * CPI) / COO))) * CON);
                        CQL = CPK;
                    }
                    let CPL = if DC == A { 1.0 } else { 0.0 };
                    let CQM;
                    if CPL != 0.0 {
                        CQM = A;
                    } else {
                        let CPM = if AY == GR { 1.0 } else { 0.0 };
                        let CPP = if CPM != 0.0 {
                            let CPN = ((AI - CIT) * JF).sqrt();
                            CPN
                        } else {
                            let CPO = ((AI - CIT) * JF).powf(AY);
                            CPO
                        };
                        let CPQ = IT * (((AI - CIT) * JC) / CPP);
                        let CPR = (-KF) / CPQ;
                        let CPS = if (CPR.abs()) < MW { 1.0 } else { 0.0 };
                        let CPY;
                        if CPS != 0.0 {
                            let CPT = CPR.exp();
                            CPY = CPT;
                        } else {
                            let CPU = if CPR < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            let CPZ = if CPU != 0.0 {
                                let CPV = NA / (GT + ((-2.3025850929940458e2f64 - CPR) * (GT + (GR * ((-2.3025850929940458e2f64 - CPR) * (GT + ((-2.3025850929940458e2f64 - CPR) * NB)))))));
                                CPV
                            } else {
                                let CPW = CPR - MW;
                                let CPX = ND * (GT + (CPW * (GT + (GR * (CPW * (GT + (CPW * NB)))))));
                                CPX
                            };
                            CPY = CPZ;
                        }
                        let CQA = DC * (((DE * CPQ) * CPQ) * CPY);
                        CQM = CQA;
                    }
                    let CQB = if (if AIS > LG { 1.0 } else { 0.0 }) != 0.0 || (if ABX == A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let CQN;
                    if CQB != 0.0 {
                        CQN = GT;
                    } else {
                        let CQC = if CJJ > ((-KG) * AIS) { 1.0 } else { 0.0 };
                        let CQO;
                        if CQC != 0.0 {
                            let CQD = if DV == OR { 1.0 } else { 0.0 };
                            let CQH = if CQD != 0.0 {
                                let CQE = (CJJ * KZ).abs();
                                let CQF = ((CQE * CQE) * CQE) * CQE;
                                CQF
                            } else {
                                let CQG = ((CJJ * KZ).abs()).powf(DV);
                                CQG
                            };
                            let CQI = GT / (GT - CQH);
                            CQO = CQI;
                        } else {
                            let CQJ = KJ + ((CJJ + (KG * AIS)) * LA);
                            CQO = CQJ;
                        }
                        CQN = CQO;
                    }
                    let CQP = (((CNJ + CQK) + CQL) + CQM) * CQN;
                    CQS = CQP;
                }
                let CQT = ((LY * CQQ) + (MC * CQR)) + (MG * CQS);
                let DAF;
                let DAJ;
                let DAL;
                let DAV;
                let DCN;
                let DDD;
                let DDT;
                let DHC;
                if PH != 0.0 {
                    let CQU = if PG < MU { 1.0 } else { 0.0 };
                    let CZE;
                    let CZI;
                    let CZM;
                    let CZQ;
                    if CQU != 0.0 {
                        let CQV = GR * (PG * HK);
                        let CQW = if (CQV.abs()) < MW { 1.0 } else { 0.0 };
                        let CZR;
                        if CQW != 0.0 {
                            let CQX = CQV.exp();
                            CZR = CQX;
                        } else {
                            let CQY = if CQV < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            let CZS = if CQY != 0.0 {
                                let CQZ = NA / (GT + ((-2.3025850929940458e2f64 - CQV) * (GT + (GR * ((-2.3025850929940458e2f64 - CQV) * (GT + ((-2.3025850929940458e2f64 - CQV) * NB)))))));
                                CQZ
                            } else {
                                let CRA = CQV - MW;
                                let CRB = ND * (GT + (CRA * (GT + (GR * (CRA * (GT + (CRA * NB)))))));
                                CRB
                            };
                            CZR = CZS;
                        }
                        let CRC = if EV < PQ { 1.0 } else { 0.0 };
                        let CRW;
                        let CRX;
                        if CRC != 0.0 {
                            let CRD = EV - (PS * LS);
                            let CRE = (PQ - ((PS * (PG - LS)) + EV)) - CL;
                            let CRF = (OR * PQ) * CL;
                            let CRG = if CRF > A { 1.0 } else { 0.0 };
                            let CRI = if CRG != 0.0 {
                                CRF
                            } else {
                                let CRH = -CRF;
                                CRH
                            };
                            let CRJ = ((PQ - (GR * (CRE + (((CRE * CRE) + CRI).sqrt())))) - EV) - CL;
                            let CRK = (OR * EV) * CL;
                            let CRL = if CRK > A { 1.0 } else { 0.0 };
                            let CRN = if CRL != 0.0 {
                                CRK
                            } else {
                                let CRM = -CRK;
                                CRM
                            };
                            let CRO = EV + (GR * (CRJ + (((CRJ * CRJ) + CRN).sqrt())));
                            let CRP = (PQ - CRD) - CL;
                            let CRR = if CRG != 0.0 {
                                CRF
                            } else {
                                let CRQ = -CRF;
                                CRQ
                            };
                            let CRS = ((PQ - (GR * (CRP + (((CRP * CRP) + CRR).sqrt())))) - EV) - CL;
                            let CRU = if CRL != 0.0 {
                                CRK
                            } else {
                                let CRT = -CRK;
                                CRT
                            };
                            let CRV = EV + (GR * (CRS + (((CRS * CRS) + CRU).sqrt())));
                            CRW = CRO;
                            CRX = CRV;
                        } else {
                            CRW = EV;
                            CRX = EV;
                        }
                        let CRY = HK * ((PG / CRW) + ((LS * (CRW - CRX)) / (CRX * PQ)));
                        let CRZ = if (CRY.abs()) < MW { 1.0 } else { 0.0 };
                        let CZF;
                        if CRZ != 0.0 {
                            let CSA = CRY.exp();
                            CZF = CSA;
                        } else {
                            let CSB = if CRY < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            let CZG = if CSB != 0.0 {
                                let CSC = NA / (GT + ((-2.3025850929940458e2f64 - CRY) * (GT + (GR * ((-2.3025850929940458e2f64 - CRY) * (GT + ((-2.3025850929940458e2f64 - CRY) * NB)))))));
                                CSC
                            } else {
                                let CSD = CRY - MW;
                                let CSE = ND * (GT + (CSD * (GT + (GR * (CSD * (GT + (CSD * NB)))))));
                                CSE
                            };
                            CZF = CZG;
                        }
                        let CSF = (EY / HK) * ((LI / (LM / LI)).ln());
                        let CSG = if EY < PQ { 1.0 } else { 0.0 };
                        let CTA;
                        let CTB;
                        if CSG != 0.0 {
                            let CSH = EY - (PS * CSF);
                            let CSI = (PQ - ((PS * (PG - CSF)) + EY)) - CL;
                            let CSJ = (OR * PQ) * CL;
                            let CSK = if CSJ > A { 1.0 } else { 0.0 };
                            let CSM = if CSK != 0.0 {
                                CSJ
                            } else {
                                let CSL = -CSJ;
                                CSL
                            };
                            let CSN = ((PQ - (GR * (CSI + (((CSI * CSI) + CSM).sqrt())))) - EY) - CL;
                            let CSO = (OR * EY) * CL;
                            let CSP = if CSO > A { 1.0 } else { 0.0 };
                            let CSR = if CSP != 0.0 {
                                CSO
                            } else {
                                let CSQ = -CSO;
                                CSQ
                            };
                            let CSS = EY + (GR * (CSN + (((CSN * CSN) + CSR).sqrt())));
                            let CST = (PQ - CSH) - CL;
                            let CSV = if CSK != 0.0 {
                                CSJ
                            } else {
                                let CSU = -CSJ;
                                CSU
                            };
                            let CSW = ((PQ - (GR * (CST + (((CST * CST) + CSV).sqrt())))) - EY) - CL;
                            let CSY = if CSP != 0.0 {
                                CSO
                            } else {
                                let CSX = -CSO;
                                CSX
                            };
                            let CSZ = EY + (GR * (CSW + (((CSW * CSW) + CSY).sqrt())));
                            CTA = CSS;
                            CTB = CSZ;
                        } else {
                            CTA = EY;
                            CTB = EY;
                        }
                        let CTC = HK * ((PG / CTA) + ((CSF * (CTA - CTB)) / (CTB * PQ)));
                        let CTD = if (CTC.abs()) < MW { 1.0 } else { 0.0 };
                        let CZJ;
                        if CTD != 0.0 {
                            let CTE = CTC.exp();
                            CZJ = CTE;
                        } else {
                            let CTF = if CTC < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            let CZK = if CTF != 0.0 {
                                let CTG = NA / (GT + ((-2.3025850929940458e2f64 - CTC) * (GT + (GR * ((-2.3025850929940458e2f64 - CTC) * (GT + ((-2.3025850929940458e2f64 - CTC) * NB)))))));
                                CTG
                            } else {
                                let CTH = CTC - MW;
                                let CTI = ND * (GT + (CTH * (GT + (GR * (CTH * (GT + (CTH * NB)))))));
                                CTI
                            };
                            CZJ = CZK;
                        }
                        let CTJ = (FB / HK) * ((LJ / (LM / LJ)).ln());
                        let CTK = if FB < PQ { 1.0 } else { 0.0 };
                        let CUE;
                        let CUF;
                        if CTK != 0.0 {
                            let CTL = FB - (PS * CTJ);
                            let CTM = (PQ - ((PS * (PG - CTJ)) + FB)) - CL;
                            let CTN = (OR * PQ) * CL;
                            let CTO = if CTN > A { 1.0 } else { 0.0 };
                            let CTQ = if CTO != 0.0 {
                                CTN
                            } else {
                                let CTP = -CTN;
                                CTP
                            };
                            let CTR = ((PQ - (GR * (CTM + (((CTM * CTM) + CTQ).sqrt())))) - FB) - CL;
                            let CTS = (OR * FB) * CL;
                            let CTT = if CTS > A { 1.0 } else { 0.0 };
                            let CTV = if CTT != 0.0 {
                                CTS
                            } else {
                                let CTU = -CTS;
                                CTU
                            };
                            let CTW = FB + (GR * (CTR + (((CTR * CTR) + CTV).sqrt())));
                            let CTX = (PQ - CTL) - CL;
                            let CTZ = if CTO != 0.0 {
                                CTN
                            } else {
                                let CTY = -CTN;
                                CTY
                            };
                            let CUA = ((PQ - (GR * (CTX + (((CTX * CTX) + CTZ).sqrt())))) - FB) - CL;
                            let CUC = if CTT != 0.0 {
                                CTS
                            } else {
                                let CUB = -CTS;
                                CUB
                            };
                            let CUD = FB + (GR * (CUA + (((CUA * CUA) + CUC).sqrt())));
                            CUE = CTW;
                            CUF = CUD;
                        } else {
                            CUE = FB;
                            CUF = FB;
                        }
                        let CUG = HK * ((PG / CUE) + ((CTJ * (CUE - CUF)) / (CUF * PQ)));
                        let CUH = if (CUG.abs()) < MW { 1.0 } else { 0.0 };
                        let CZN;
                        if CUH != 0.0 {
                            let CUI = CUG.exp();
                            CZN = CUI;
                        } else {
                            let CUJ = if CUG < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            let CZO = if CUJ != 0.0 {
                                let CUK = NA / (GT + ((-2.3025850929940458e2f64 - CUG) * (GT + (GR * ((-2.3025850929940458e2f64 - CUG) * (GT + ((-2.3025850929940458e2f64 - CUG) * NB)))))));
                                CUK
                            } else {
                                let CUL = CUG - MW;
                                let CUM = ND * (GT + (CUL * (GT + (GR * (CUL * (GT + (CUL * NB)))))));
                                CUM
                            };
                            CZN = CZO;
                        }
                        CZE = CZF;
                        CZI = CZJ;
                        CZM = CZN;
                        CZQ = CZR;
                    } else {
                        let CUN = PG - MU;
                        let CUO = ((GT + (CUN * HK)) * TE).sqrt();
                        let CUP = if EV < PQ { 1.0 } else { 0.0 };
                        let CVO;
                        let CVP;
                        let CVY;
                        if CUP != 0.0 {
                            let CUQ = EV - (PS * LS);
                            let CUR = (PQ - ((PS * (MU - LS)) + EV)) - CL;
                            let CUS = (OR * PQ) * CL;
                            let CUT = if CUS > A { 1.0 } else { 0.0 };
                            let CUV = if CUT != 0.0 {
                                CUS
                            } else {
                                let CUU = -CUS;
                                CUU
                            };
                            let CUW = ((CUR * CUR) + CUV).sqrt();
                            let CUX = GR * (GT + (CUR / CUW));
                            let CUY = ((PQ - (GR * (CUR + CUW))) - EV) - CL;
                            let CUZ = (OR * EV) * CL;
                            let CVA = if CUZ > A { 1.0 } else { 0.0 };
                            let CVC = if CVA != 0.0 {
                                CUZ
                            } else {
                                let CVB = -CUZ;
                                CVB
                            };
                            let CVD = ((CUY * CUY) + CVC).sqrt();
                            let CVE = GR * (GT + (CUY / CVD));
                            let CVF = EV + (GR * (CUY + CVD));
                            let CVG = (PQ - CUQ) - CL;
                            let CVI = if CUT != 0.0 {
                                CUS
                            } else {
                                let CVH = -CUS;
                                CVH
                            };
                            let CVJ = ((PQ - (GR * (CVG + (((CVG * CVG) + CVI).sqrt())))) - EV) - CL;
                            let CVL = if CVA != 0.0 {
                                CUZ
                            } else {
                                let CVK = -CUZ;
                                CVK
                            };
                            let CVM = EV + (GR * (CVJ + (((CVJ * CVJ) + CVL).sqrt())));
                            let CVN = (PS * CUX) * CVE;
                            CVO = CVF;
                            CVP = CVM;
                            CVY = CVN;
                        } else {
                            CVO = EV;
                            CVP = EV;
                            CVY = A;
                        }
                        let CVQ = CVP * PQ;
                        let CVR = HK * ((MU / CVO) + ((LS * (CVO - CVP)) / CVQ));
                        let CVS = if (CVR.abs()) < MW { 1.0 } else { 0.0 };
                        let CVZ;
                        if CVS != 0.0 {
                            let CVT = CVR.exp();
                            CVZ = CVT;
                        } else {
                            let CVU = if CVR < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            let CWA = if CVU != 0.0 {
                                let CVV = NA / (GT + ((-2.3025850929940458e2f64 - CVR) * (GT + (GR * ((-2.3025850929940458e2f64 - CVR) * (GT + ((-2.3025850929940458e2f64 - CVR) * NB)))))));
                                CVV
                            } else {
                                let CVW = CVR - MW;
                                let CVX = ND * (GT + (CVW * (GT + (GR * (CVW * (GT + (CVW * NB)))))));
                                CVX
                            };
                            CVZ = CWA;
                        }
                        let CWB = (GT + (CUN * (HK * (((CVO - (MU * CVY)) / (CVO * CVO)) + ((LS * CVY) / CVQ))))) * CVZ;
                        let CWC = (EY / HK) * ((LI / (LM / LI)).ln());
                        let CWD = if EY < PQ { 1.0 } else { 0.0 };
                        let CXC;
                        let CXD;
                        let CXM;
                        if CWD != 0.0 {
                            let CWE = EY - (PS * CWC);
                            let CWF = (PQ - ((PS * (MU - CWC)) + EY)) - CL;
                            let CWG = (OR * PQ) * CL;
                            let CWH = if CWG > A { 1.0 } else { 0.0 };
                            let CWJ = if CWH != 0.0 {
                                CWG
                            } else {
                                let CWI = -CWG;
                                CWI
                            };
                            let CWK = ((CWF * CWF) + CWJ).sqrt();
                            let CWL = GR * (GT + (CWF / CWK));
                            let CWM = ((PQ - (GR * (CWF + CWK))) - EY) - CL;
                            let CWN = (OR * EY) * CL;
                            let CWO = if CWN > A { 1.0 } else { 0.0 };
                            let CWQ = if CWO != 0.0 {
                                CWN
                            } else {
                                let CWP = -CWN;
                                CWP
                            };
                            let CWR = ((CWM * CWM) + CWQ).sqrt();
                            let CWS = GR * (GT + (CWM / CWR));
                            let CWT = EY + (GR * (CWM + CWR));
                            let CWU = (PQ - CWE) - CL;
                            let CWW = if CWH != 0.0 {
                                CWG
                            } else {
                                let CWV = -CWG;
                                CWV
                            };
                            let CWX = ((PQ - (GR * (CWU + (((CWU * CWU) + CWW).sqrt())))) - EY) - CL;
                            let CWZ = if CWO != 0.0 {
                                CWN
                            } else {
                                let CWY = -CWN;
                                CWY
                            };
                            let CXA = EY + (GR * (CWX + (((CWX * CWX) + CWZ).sqrt())));
                            let CXB = (PS * CWL) * CWS;
                            CXC = CWT;
                            CXD = CXA;
                            CXM = CXB;
                        } else {
                            CXC = EY;
                            CXD = EY;
                            CXM = A;
                        }
                        let CXE = CXD * PQ;
                        let CXF = HK * ((MU / CXC) + ((CWC * (CXC - CXD)) / CXE));
                        let CXG = if (CXF.abs()) < MW { 1.0 } else { 0.0 };
                        let CXN;
                        if CXG != 0.0 {
                            let CXH = CXF.exp();
                            CXN = CXH;
                        } else {
                            let CXI = if CXF < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            let CXO = if CXI != 0.0 {
                                let CXJ = NA / (GT + ((-2.3025850929940458e2f64 - CXF) * (GT + (GR * ((-2.3025850929940458e2f64 - CXF) * (GT + ((-2.3025850929940458e2f64 - CXF) * NB)))))));
                                CXJ
                            } else {
                                let CXK = CXF - MW;
                                let CXL = ND * (GT + (CXK * (GT + (GR * (CXK * (GT + (CXK * NB)))))));
                                CXL
                            };
                            CXN = CXO;
                        }
                        let CXP = (GT + (CUN * (HK * (((CXC - (MU * CXM)) / (CXC * CXC)) + ((CWC * CXM) / CXE))))) * CXN;
                        let CXQ = (FB / HK) * ((LJ / (LM / LJ)).ln());
                        let CXR = if FB < PQ { 1.0 } else { 0.0 };
                        let CYQ;
                        let CYR;
                        let CZA;
                        if CXR != 0.0 {
                            let CXS = FB - (PS * CXQ);
                            let CXT = (PQ - ((PS * (MU - CXQ)) + FB)) - CL;
                            let CXU = (OR * PQ) * CL;
                            let CXV = if CXU > A { 1.0 } else { 0.0 };
                            let CXX = if CXV != 0.0 {
                                CXU
                            } else {
                                let CXW = -CXU;
                                CXW
                            };
                            let CXY = ((CXT * CXT) + CXX).sqrt();
                            let CXZ = GR * (GT + (CXT / CXY));
                            let CYA = ((PQ - (GR * (CXT + CXY))) - FB) - CL;
                            let CYB = (OR * FB) * CL;
                            let CYC = if CYB > A { 1.0 } else { 0.0 };
                            let CYE = if CYC != 0.0 {
                                CYB
                            } else {
                                let CYD = -CYB;
                                CYD
                            };
                            let CYF = ((CYA * CYA) + CYE).sqrt();
                            let CYG = GR * (GT + (CYA / CYF));
                            let CYH = FB + (GR * (CYA + CYF));
                            let CYI = (PQ - CXS) - CL;
                            let CYK = if CXV != 0.0 {
                                CXU
                            } else {
                                let CYJ = -CXU;
                                CYJ
                            };
                            let CYL = ((PQ - (GR * (CYI + (((CYI * CYI) + CYK).sqrt())))) - FB) - CL;
                            let CYN = if CYC != 0.0 {
                                CYB
                            } else {
                                let CYM = -CYB;
                                CYM
                            };
                            let CYO = FB + (GR * (CYL + (((CYL * CYL) + CYN).sqrt())));
                            let CYP = (PS * CXZ) * CYG;
                            CYQ = CYH;
                            CYR = CYO;
                            CZA = CYP;
                        } else {
                            CYQ = FB;
                            CYR = FB;
                            CZA = A;
                        }
                        let CYS = CYR * PQ;
                        let CYT = HK * ((MU / CYQ) + ((CXQ * (CYQ - CYR)) / CYS));
                        let CYU = if (CYT.abs()) < MW { 1.0 } else { 0.0 };
                        let CZB;
                        if CYU != 0.0 {
                            let CYV = CYT.exp();
                            CZB = CYV;
                        } else {
                            let CYW = if CYT < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            let CZC = if CYW != 0.0 {
                                let CYX = NA / (GT + ((-2.3025850929940458e2f64 - CYT) * (GT + (GR * ((-2.3025850929940458e2f64 - CYT) * (GT + ((-2.3025850929940458e2f64 - CYT) * NB)))))));
                                CYX
                            } else {
                                let CYY = CYT - MW;
                                let CYZ = ND * (GT + (CYY * (GT + (GR * (CYY * (GT + (CYY * NB)))))));
                                CYZ
                            };
                            CZB = CZC;
                        }
                        let CZD = (GT + (CUN * (HK * (((CYQ - (MU * CZA)) / (CYQ * CYQ)) + ((CXQ * CZA) / CYS))))) * CZB;
                        CZE = CWB;
                        CZI = CXP;
                        CZM = CZD;
                        CZQ = CUO;
                    }
                    let CZH = CZE - GT;
                    let CZL = CZI - GT;
                    let CZP = CZM - GT;
                    let CZT = GT / CZQ;
                    let CZX = if CZU != 0.0 {
                        let CZV = HS * (HJ * (((HS + CZT) + (((CZT + GT) * (CZT + JI)).sqrt())).ln()));
                        CZV
                    } else {
                        let CZW = -2e-1f64 + (HS * (HJ * ((((HS * CZQ) + GT) + (((GT + CZQ) * (GT + (JI * CZQ))).sqrt())).ln())));
                        CZW
                    };
                    let CZY = NS - CZX;
                    let CZZ = PG - CZY;
                    let DAA = GR * ((PG + CZY) - (((CZZ * CZZ) + ((OR * HJ) * HJ)).sqrt()));
                    let DAB = PG - NW;
                    let DAC = GR * ((PG + NW) - (((DAB * DAB) + ((OR * HH) * HH)).sqrt()));
                    DAF = CZH;
                    DAJ = DAA;
                    DAL = CZX;
                    DAV = CZQ;
                    DCN = DAC;
                    DDD = DAD;
                    DDT = CZL;
                    DHC = CZP;
                } else {
                    DAF = A;
                    DAJ = A;
                    DAL = A;
                    DAV = A;
                    DCN = A;
                    DDD = A;
                    DDT = A;
                    DHC = A;
                }
                let DKK;
                if NG != 0.0 {
                    DKK = A;
                } else {
                    let DAE = if IO == GR { 1.0 } else { 0.0 };
                    if DAE != 0.0 {
                    } else {
                    }
                    let DAG = IE * DAF;
                    let DAH = if CD == A { 1.0 } else { 0.0 };
                    let DAI = if (if BN == A { 1.0 } else { 0.0 }) != 0.0 && DAH != 0.0 { 1.0 } else { 0.0 };
                    let DAY;
                    let DAZ;
                    let DBL;
                    let DCJ;
                    let DDM;
                    if DAI != 0.0 {
                        DAY = A;
                        DAZ = A;
                        DBL = A;
                        DCJ = A;
                        DDM = A;
                    } else {
                        let DAK = IL - DAJ;
                        let DAM = GT - ((GT - (DAL / DAK)).sqrt());
                        let DAN = if AO == GR { 1.0 } else { 0.0 };
                        let DAP = if DAN != 0.0 {
                            A
                        } else {
                            let DAO = ((((DAM * DAM) * (DAM.ln())) / (GT - DAM)) + DAM) * (GT - (HS * AO));
                            DAO
                        };
                        let DAQ = DAM + DAP;
                        let DAT = if DAN != 0.0 {
                            let DAR = (DAK * JD).sqrt();
                            DAR
                        } else {
                            let DAS = (DAK * JD).powf(AO);
                            DAS
                        };
                        let DAU = IX * DAT;
                        let DAW = HW * ((DAV - GT) * DAU);
                        let DAX = BN * (DAW * DAQ);
                        DAY = DAU;
                        DAZ = DAK;
                        DBL = DAQ;
                        DCJ = DAW;
                        DDM = DAX;
                    }
                    let DDN;
                    if DAH != 0.0 {
                        DDN = A;
                    } else {
                        let DBA = JT * ((DAY * IO) / DAZ);
                        let DBB = (ZT * JO) / DBA;
                        let DBC = DBB * DBB;
                        let DBD = DBC * DBC;
                        let DBE = (DBD / (DBD + GT)).sqrt();
                        let DBF = (DBE.abs()).sqrt();
                        let DBG = DBE * DBF;
                        let DBH = (-AO) * IR;
                        let DBI = if DBH == -1e0f64 { 1.0 } else { 0.0 };
                        let DBM = if DBI != 0.0 {
                            let DBJ = GT / (GT + (DBA * DBG));
                            DBJ
                        } else {
                            let DBK = (GT + (DBA * DBG)).powf(DBH);
                            DBK
                        };
                        let DBN = (DBL * DBM) / (DBL + DBM);
                        let DBO = (AAH * (DBA / DBF)).sqrt();
                        let DBP = (((JO * DBB) * DBF) - (JO * DBE)) + (GR * (DBA * DBG));
                        let DBQ = (((HS * (DBB * DBF)) - DBE) - GT) * DBO;
                        let DBR = DBQ * DBQ;
                        let DBS = if DBQ > A { 1.0 } else { 0.0 };
                        let DBZ = if DBS != 0.0 {
                            let DBT = GT / (GT + (JH * DBQ));
                            DBT
                        } else {
                            let DBU = GT / (GT - (JH * DBQ));
                            DBU
                        };
                        let DBV = (-DBR) + DBP;
                        let DBW = if DBV > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                        let DCB = if DBW != 0.0 {
                            let DBX = DBV.exp();
                            DBX
                        } else {
                            let DBY = NA / (GT + ((-2.3025850929940458e2f64 - DBV) * (GT + (GR * ((-2.3025850929940458e2f64 - DBV) * (GT + ((-2.3025850929940458e2f64 - DBV) * NB)))))));
                            DBY
                        };
                        let DCA = DBZ * DBZ;
                        let DCC = (((JG * DBZ) + (JJ * DCA)) + (JK * (DCA * DBZ))) * DCB;
                        let DCI;
                        if DBS != 0.0 {
                            DCI = DCC;
                        } else {
                            let DCD = if DBP > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            let DCG = if DCD != 0.0 {
                                let DCE = DBP.exp();
                                DCE
                            } else {
                                let DCF = NA / (GT + ((-2.3025850929940458e2f64 - DBP) * (GT + (GR * ((-2.3025850929940458e2f64 - DBP) * (GT + ((-2.3025850929940458e2f64 - DBP) * NB)))))));
                                DCF
                            };
                            let DCH = (HS * DCG) - DCC;
                            DCI = DCH;
                        }
                        let DCK = CD * ((DCJ * (8.86226925452758e-1f64 * ((JO * DCI) / DBO))) * DBN);
                        DDN = DCK;
                    }
                    let DCL = if CW == A { 1.0 } else { 0.0 };
                    let DDO;
                    if DCL != 0.0 {
                        DDO = A;
                    } else {
                        let DCM = if AO == GR { 1.0 } else { 0.0 };
                        let DCQ = if DCM != 0.0 {
                            let DCO = ((AC - DCN) * JD).sqrt();
                            DCO
                        } else {
                            let DCP = ((AC - DCN) * JD).powf(AO);
                            DCP
                        };
                        let DCR = IR * (((AC - DCN) * JA) / DCQ);
                        let DCS = (-KB) / DCR;
                        let DCT = if (DCS.abs()) < MW { 1.0 } else { 0.0 };
                        let DCZ;
                        if DCT != 0.0 {
                            let DCU = DCS.exp();
                            DCZ = DCU;
                        } else {
                            let DCV = if DCS < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            let DDA = if DCV != 0.0 {
                                let DCW = NA / (GT + ((-2.3025850929940458e2f64 - DCS) * (GT + (GR * ((-2.3025850929940458e2f64 - DCS) * (GT + ((-2.3025850929940458e2f64 - DCS) * NB)))))));
                                DCW
                            } else {
                                let DCX = DCS - MW;
                                let DCY = ND * (GT + (DCX * (GT + (GR * (DCX * (GT + (DCX * NB)))))));
                                DCY
                            };
                            DCZ = DDA;
                        }
                        let DDB = CW * (((PG * DCR) * DCR) * DCZ);
                        DDO = DDB;
                    }
                    let DDC = if (if ABW > LG { 1.0 } else { 0.0 }) != 0.0 || (if ABX == A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let DDP;
                    if DDC != 0.0 {
                        DDP = GT;
                    } else {
                        let DDE = if DDD > ((-KG) * ABW) { 1.0 } else { 0.0 };
                        let DDQ;
                        if DDE != 0.0 {
                            let DDF = if DP == OR { 1.0 } else { 0.0 };
                            let DDJ = if DDF != 0.0 {
                                let DDG = (DDD * KV).abs();
                                let DDH = ((DDG * DDG) * DDG) * DDG;
                                DDH
                            } else {
                                let DDI = ((DDD * KV).abs()).powf(DP);
                                DDI
                            };
                            let DDK = GT / (GT - DDJ);
                            DDQ = DDK;
                        } else {
                            let DDL = KH + ((DDD + (KG * ABW)) * KW);
                            DDQ = DDL;
                        }
                        DDP = DDQ;
                    }
                    let DDR = (((DAG + DDM) + DDN) + DDO) * DDP;
                    DKK = DDR;
                }
                let DKL;
                if NJ != 0.0 {
                    DKL = A;
                } else {
                    let DDS = if IP == GR { 1.0 } else { 0.0 };
                    if DDS != 0.0 {
                    } else {
                    }
                    let DDU = IF * DDT;
                    let DDV = if CG == A { 1.0 } else { 0.0 };
                    let DDW = if (if BQ == A { 1.0 } else { 0.0 }) != 0.0 && DDV != 0.0 { 1.0 } else { 0.0 };
                    let DEJ;
                    let DEK;
                    let DEW;
                    let DFU;
                    let DGV;
                    if DDW != 0.0 {
                        DEJ = A;
                        DEK = A;
                        DEW = A;
                        DFU = A;
                        DGV = A;
                    } else {
                        let DDX = IM - DAJ;
                        let DDY = GT - ((GT - (DAL / DDX)).sqrt());
                        let DDZ = if AT == GR { 1.0 } else { 0.0 };
                        let DEB = if DDZ != 0.0 {
                            A
                        } else {
                            let DEA = ((((DDY * DDY) * (DDY.ln())) / (GT - DDY)) + DDY) * (GT - (HS * AT));
                            DEA
                        };
                        let DEC = DDY + DEB;
                        let DEF = if DDZ != 0.0 {
                            let DED = (DDX * JE).sqrt();
                            DED
                        } else {
                            let DEE = (DDX * JE).powf(AT);
                            DEE
                        };
                        let DEG = IY * DEF;
                        let DEH = HY * ((DAV - GT) * DEG);
                        let DEI = BQ * (DEH * DEC);
                        DEJ = DEG;
                        DEK = DDX;
                        DEW = DEC;
                        DFU = DEH;
                        DGV = DEI;
                    }
                    let DGW;
                    if DDV != 0.0 {
                        DGW = A;
                    } else {
                        let DEL = JU * ((DEJ * IP) / DEK);
                        let DEM = (ZT * JP) / DEL;
                        let DEN = DEM * DEM;
                        let DEO = DEN * DEN;
                        let DEP = (DEO / (DEO + GT)).sqrt();
                        let DEQ = (DEP.abs()).sqrt();
                        let DER = DEP * DEQ;
                        let DES = (-AT) * IS;
                        let DET = if DES == -1e0f64 { 1.0 } else { 0.0 };
                        let DEX = if DET != 0.0 {
                            let DEU = GT / (GT + (DEL * DER));
                            DEU
                        } else {
                            let DEV = (GT + (DEL * DER)).powf(DES);
                            DEV
                        };
                        let DEY = (DEW * DEX) / (DEW + DEX);
                        let DEZ = (AAH * (DEL / DEQ)).sqrt();
                        let DFA = (((JP * DEM) * DEQ) - (JP * DEP)) + (GR * (DEL * DER));
                        let DFB = (((HS * (DEM * DEQ)) - DEP) - GT) * DEZ;
                        let DFC = DFB * DFB;
                        let DFD = if DFB > A { 1.0 } else { 0.0 };
                        let DFK = if DFD != 0.0 {
                            let DFE = GT / (GT + (JH * DFB));
                            DFE
                        } else {
                            let DFF = GT / (GT - (JH * DFB));
                            DFF
                        };
                        let DFG = (-DFC) + DFA;
                        let DFH = if DFG > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                        let DFM = if DFH != 0.0 {
                            let DFI = DFG.exp();
                            DFI
                        } else {
                            let DFJ = NA / (GT + ((-2.3025850929940458e2f64 - DFG) * (GT + (GR * ((-2.3025850929940458e2f64 - DFG) * (GT + ((-2.3025850929940458e2f64 - DFG) * NB)))))));
                            DFJ
                        };
                        let DFL = DFK * DFK;
                        let DFN = (((JG * DFK) + (JJ * DFL)) + (JK * (DFL * DFK))) * DFM;
                        let DFT;
                        if DFD != 0.0 {
                            DFT = DFN;
                        } else {
                            let DFO = if DFA > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            let DFR = if DFO != 0.0 {
                                let DFP = DFA.exp();
                                DFP
                            } else {
                                let DFQ = NA / (GT + ((-2.3025850929940458e2f64 - DFA) * (GT + (GR * ((-2.3025850929940458e2f64 - DFA) * (GT + ((-2.3025850929940458e2f64 - DFA) * NB)))))));
                                DFQ
                            };
                            let DFS = (HS * DFR) - DFN;
                            DFT = DFS;
                        }
                        let DFV = CG * ((DFU * (8.86226925452758e-1f64 * ((JP * DFT) / DEZ))) * DEY);
                        DGW = DFV;
                    }
                    let DFW = if CZ == A { 1.0 } else { 0.0 };
                    let DGX;
                    if DFW != 0.0 {
                        DGX = A;
                    } else {
                        let DFX = if AT == GR { 1.0 } else { 0.0 };
                        let DGA = if DFX != 0.0 {
                            let DFY = ((AF - DCN) * JE).sqrt();
                            DFY
                        } else {
                            let DFZ = ((AF - DCN) * JE).powf(AT);
                            DFZ
                        };
                        let DGB = IS * (((AF - DCN) * JB) / DGA);
                        let DGC = (-KD) / DGB;
                        let DGD = if (DGC.abs()) < MW { 1.0 } else { 0.0 };
                        let DGJ;
                        if DGD != 0.0 {
                            let DGE = DGC.exp();
                            DGJ = DGE;
                        } else {
                            let DGF = if DGC < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            let DGK = if DGF != 0.0 {
                                let DGG = NA / (GT + ((-2.3025850929940458e2f64 - DGC) * (GT + (GR * ((-2.3025850929940458e2f64 - DGC) * (GT + ((-2.3025850929940458e2f64 - DGC) * NB)))))));
                                DGG
                            } else {
                                let DGH = DGC - MW;
                                let DGI = ND * (GT + (DGH * (GT + (GR * (DGH * (GT + (DGH * NB)))))));
                                DGI
                            };
                            DGJ = DGK;
                        }
                        let DGL = CZ * (((PG * DGB) * DGB) * DGJ);
                        DGX = DGL;
                    }
                    let DGM = if (if AFI > LG { 1.0 } else { 0.0 }) != 0.0 || (if ABX == A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let DGY;
                    if DGM != 0.0 {
                        DGY = GT;
                    } else {
                        let DGN = if DDD > ((-KG) * AFI) { 1.0 } else { 0.0 };
                        let DGZ;
                        if DGN != 0.0 {
                            let DGO = if DS == OR { 1.0 } else { 0.0 };
                            let DGS = if DGO != 0.0 {
                                let DGP = (DDD * KX).abs();
                                let DGQ = ((DGP * DGP) * DGP) * DGP;
                                DGQ
                            } else {
                                let DGR = ((DDD * KX).abs()).powf(DS);
                                DGR
                            };
                            let DGT = GT / (GT - DGS);
                            DGZ = DGT;
                        } else {
                            let DGU = KI + ((DDD + (KG * AFI)) * KY);
                            DGZ = DGU;
                        }
                        DGY = DGZ;
                    }
                    let DHA = (((DDU + DGV) + DGW) + DGX) * DGY;
                    DKL = DHA;
                }
                let DKM;
                if NM != 0.0 {
                    DKM = A;
                } else {
                    let DHB = if IQ == GR { 1.0 } else { 0.0 };
                    if DHB != 0.0 {
                    } else {
                    }
                    let DHD = IG * DHC;
                    let DHE = if CJ == A { 1.0 } else { 0.0 };
                    let DHF = if (if BT == A { 1.0 } else { 0.0 }) != 0.0 && DHE != 0.0 { 1.0 } else { 0.0 };
                    let DHS;
                    let DHT;
                    let DIF;
                    let DJD;
                    let DKE;
                    if DHF != 0.0 {
                        DHS = A;
                        DHT = A;
                        DIF = A;
                        DJD = A;
                        DKE = A;
                    } else {
                        let DHG = IN - DAJ;
                        let DHH = GT - ((GT - (DAL / DHG)).sqrt());
                        let DHI = if AY == GR { 1.0 } else { 0.0 };
                        let DHK = if DHI != 0.0 {
                            A
                        } else {
                            let DHJ = ((((DHH * DHH) * (DHH.ln())) / (GT - DHH)) + DHH) * (GT - (HS * AY));
                            DHJ
                        };
                        let DHL = DHH + DHK;
                        let DHO = if DHI != 0.0 {
                            let DHM = (DHG * JF).sqrt();
                            DHM
                        } else {
                            let DHN = (DHG * JF).powf(AY);
                            DHN
                        };
                        let DHP = IZ * DHO;
                        let DHQ = IA * ((DAV - GT) * DHP);
                        let DHR = BT * (DHQ * DHL);
                        DHS = DHP;
                        DHT = DHG;
                        DIF = DHL;
                        DJD = DHQ;
                        DKE = DHR;
                    }
                    let DKF;
                    if DHE != 0.0 {
                        DKF = A;
                    } else {
                        let DHU = JV * ((DHS * IQ) / DHT);
                        let DHV = (ZT * JQ) / DHU;
                        let DHW = DHV * DHV;
                        let DHX = DHW * DHW;
                        let DHY = (DHX / (DHX + GT)).sqrt();
                        let DHZ = (DHY.abs()).sqrt();
                        let DIA = DHY * DHZ;
                        let DIB = (-AY) * IT;
                        let DIC = if DIB == -1e0f64 { 1.0 } else { 0.0 };
                        let DIG = if DIC != 0.0 {
                            let DID = GT / (GT + (DHU * DIA));
                            DID
                        } else {
                            let DIE = (GT + (DHU * DIA)).powf(DIB);
                            DIE
                        };
                        let DIH = (DIF * DIG) / (DIF + DIG);
                        let DII = (AAH * (DHU / DHZ)).sqrt();
                        let DIJ = (((JQ * DHV) * DHZ) - (JQ * DHY)) + (GR * (DHU * DIA));
                        let DIK = (((HS * (DHV * DHZ)) - DHY) - GT) * DII;
                        let DIL = DIK * DIK;
                        let DIM = if DIK > A { 1.0 } else { 0.0 };
                        let DIT = if DIM != 0.0 {
                            let DIN = GT / (GT + (JH * DIK));
                            DIN
                        } else {
                            let DIO = GT / (GT - (JH * DIK));
                            DIO
                        };
                        let DIP = (-DIL) + DIJ;
                        let DIQ = if DIP > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                        let DIV = if DIQ != 0.0 {
                            let DIR = DIP.exp();
                            DIR
                        } else {
                            let DIS = NA / (GT + ((-2.3025850929940458e2f64 - DIP) * (GT + (GR * ((-2.3025850929940458e2f64 - DIP) * (GT + ((-2.3025850929940458e2f64 - DIP) * NB)))))));
                            DIS
                        };
                        let DIU = DIT * DIT;
                        let DIW = (((JG * DIT) + (JJ * DIU)) + (JK * (DIU * DIT))) * DIV;
                        let DJC;
                        if DIM != 0.0 {
                            DJC = DIW;
                        } else {
                            let DIX = if DIJ > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            let DJA = if DIX != 0.0 {
                                let DIY = DIJ.exp();
                                DIY
                            } else {
                                let DIZ = NA / (GT + ((-2.3025850929940458e2f64 - DIJ) * (GT + (GR * ((-2.3025850929940458e2f64 - DIJ) * (GT + ((-2.3025850929940458e2f64 - DIJ) * NB)))))));
                                DIZ
                            };
                            let DJB = (HS * DJA) - DIW;
                            DJC = DJB;
                        }
                        let DJE = CJ * ((DJD * (8.86226925452758e-1f64 * ((JQ * DJC) / DII))) * DIH);
                        DKF = DJE;
                    }
                    let DJF = if DC == A { 1.0 } else { 0.0 };
                    let DKG;
                    if DJF != 0.0 {
                        DKG = A;
                    } else {
                        let DJG = if AY == GR { 1.0 } else { 0.0 };
                        let DJJ = if DJG != 0.0 {
                            let DJH = ((AI - DCN) * JF).sqrt();
                            DJH
                        } else {
                            let DJI = ((AI - DCN) * JF).powf(AY);
                            DJI
                        };
                        let DJK = IT * (((AI - DCN) * JC) / DJJ);
                        let DJL = (-KF) / DJK;
                        let DJM = if (DJL.abs()) < MW { 1.0 } else { 0.0 };
                        let DJS;
                        if DJM != 0.0 {
                            let DJN = DJL.exp();
                            DJS = DJN;
                        } else {
                            let DJO = if DJL < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            let DJT = if DJO != 0.0 {
                                let DJP = NA / (GT + ((-2.3025850929940458e2f64 - DJL) * (GT + (GR * ((-2.3025850929940458e2f64 - DJL) * (GT + ((-2.3025850929940458e2f64 - DJL) * NB)))))));
                                DJP
                            } else {
                                let DJQ = DJL - MW;
                                let DJR = ND * (GT + (DJQ * (GT + (GR * (DJQ * (GT + (DJQ * NB)))))));
                                DJR
                            };
                            DJS = DJT;
                        }
                        let DJU = DC * (((PG * DJK) * DJK) * DJS);
                        DKG = DJU;
                    }
                    let DJV = if (if AIS > LG { 1.0 } else { 0.0 }) != 0.0 || (if ABX == A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let DKH;
                    if DJV != 0.0 {
                        DKH = GT;
                    } else {
                        let DJW = if DDD > ((-KG) * AIS) { 1.0 } else { 0.0 };
                        let DKI;
                        if DJW != 0.0 {
                            let DJX = if DV == OR { 1.0 } else { 0.0 };
                            let DKB = if DJX != 0.0 {
                                let DJY = (DDD * KZ).abs();
                                let DJZ = ((DJY * DJY) * DJY) * DJY;
                                DJZ
                            } else {
                                let DKA = ((DDD * KZ).abs()).powf(DV);
                                DKA
                            };
                            let DKC = GT / (GT - DKB);
                            DKI = DKC;
                        } else {
                            let DKD = KJ + ((DDD + (KG * AIS)) * LA);
                            DKI = DKD;
                        }
                        DKH = DKI;
                    }
                    let DKJ = (((DHD + DKE) + DKF) + DKG) * DKH;
                    DKM = DKJ;
                }
                let DKN = ((LY * DKK) + (MC * DKL)) + (MG * DKM);
                let DKO = DE * HK;
                let DKU = CQT - (OF * (((DKO * DKP).exp()) - GT));
                let DKV = DKN - (OF * ((((PG * HK) * DKP).exp()) - GT));
                let DME;
                let DMG;
                let DNK;
                let DNY;
                let DOF;
                if PH != 0.0 {
                    let DKW = if (if CQT > A { 1.0 } else { 0.0 }) != 0.0 && (if DKN > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let DLB;
                    let DLD;
                    if DKW != 0.0 {
                        let DKX = if (if (if (if (DKU / CQT) > LK { 1.0 } else { 0.0 }) != 0.0 || (if (DKV / DKN) > LK { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if DKU > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if DKV > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                        let DLC;
                        let DLE;
                        if DKX != 0.0 {
                            let DKY = (HJ * ((DKU / DKV).ln())) / -1e-1f64;
                            let DKZ = DKU / (((DKO * DKY).exp()) - GT);
                            DLC = DKZ;
                            DLE = DKY;
                        } else {
                            DLC = A;
                            DLE = GT;
                        }
                        DLB = DLC;
                        DLD = DLE;
                    } else {
                        DLB = A;
                        DLD = GT;
                    }
                    let DLA = PD * HK;
                    let DLF = (AJL - (OF * (((DLA * DKP).exp()) - GT))) - (DLB * (((DLA * DLD).exp()) - GT));
                    let DLG = PE * HK;
                    let DLH = (BDF - (OF * (((DLG * DKP).exp()) - GT))) - (DLB * (((DLG * DLD).exp()) - GT));
                    let DLI = PF * HK;
                    let DLJ = (BWZ - (OF * (((DLI * DKP).exp()) - GT))) - (DLB * (((DLI * DLD).exp()) - GT));
                    let DLK = if (if (if AJL < A { 1.0 } else { 0.0 }) != 0.0 && (if BDF < A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if BWZ < A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let DMH;
                    let DNZ;
                    let DOG;
                    if DLK != 0.0 {
                        let DLL = if (if (if (if (if (if (DLF / AJL) > LK { 1.0 } else { 0.0 }) != 0.0 || (if (DLH / BDF) > LK { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if (DLJ / BWZ) > LK { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if DLF < A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if DLH < A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if DLJ < A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                        let DMI;
                        let DOA;
                        let DOH;
                        if DLL != 0.0 {
                            let DLM = DLF / DLH;
                            let DLN = PD - PE;
                            let DLO = PE - PD;
                            let DLP = (((-HJ) * (DLM.ln())) / DLN) + (((HJ * (DLM - GT)) * ((DLM.powf((PE / DLO))) - GT)) / ((((DLM.powf((PD / DLN))) * DLO) + (DLM * PD)) - PE));
                            let DLQ = if ((DLI * DLP).abs()) < OV { 1.0 } else { 0.0 };
                            let DMJ;
                            let DOB;
                            let DOI;
                            if DLQ != 0.0 {
                                let DLR = DLJ * ((GT / PF) + ((GR * HK) * DLP));
                                let DLS = (((-5e-1f64 * DLJ) * DLP) * HK) / PF;
                                DMJ = DLR;
                                DOB = GT;
                                DOI = DLS;
                            } else {
                                let DLT = (-DLJ) / (((((-PF) * HK) * DLP).exp()) - GT);
                                DMJ = DLT;
                                DOB = A;
                                DOI = DLP;
                            }
                            DMI = DMJ;
                            DOA = DOB;
                            DOH = DOI;
                        } else {
                            DMI = A;
                            DOA = A;
                            DOH = GT;
                        }
                        DMH = DMI;
                        DNZ = DOA;
                        DOG = DOH;
                    } else {
                        DMH = A;
                        DNZ = A;
                        DOG = GT;
                    }
                    DME = DLB;
                    DMG = DMH;
                    DNK = DLD;
                    DNY = DNZ;
                    DOF = DOG;
                } else {
                    DME = A;
                    DMG = A;
                    DNK = GT;
                    DNY = A;
                    DOF = GT;
                }
                let DLU = LY * IU;
                let DLV = MC * IV;
                let DLW = MG * IW;
                let DLX = GZ * ((DLU + DLV) + DLW);
                let DLY = if DLU <= DLX { 1.0 } else { 0.0 };
                let DOZ = if DLY != 0.0 {
                    A
                } else {
                    GT
                };
                let DLZ = if DLV <= DLX { 1.0 } else { 0.0 };
                let DPD = if DLZ != 0.0 {
                    A
                } else {
                    GT
                };
                let DMA = if DLW <= DLX { 1.0 } else { 0.0 };
                let DPH = if DMA != 0.0 {
                    A
                } else {
                    GT
                };
                let DML;
                let DMO;
                let DMR;
                if PH != 0.0 {
                    let DMB = GR * O;
                    let DMD = (DMB / (OF + DMC)).ln();
                    let DMF = (DMB / (DME + DMC)).ln();
                    let DMK = (DMB / ((DMG.abs()) + DMC)).ln();
                    DML = DMD;
                    DMO = DMF;
                    DMR = DMK;
                } else {
                    DML = A;
                    DMO = A;
                    DMR = A;
                }
                let DMM = if DML <= MW { DML } else { MW };
                let DMN = DMM.exp();
                let DMP = if DMO <= MW { DMO } else { MW };
                let DMQ = DMP.exp();
                let DMS = if DMR <= MW { DMR } else { MW };
                let DMT = DMS.exp();
                DNA = DMM;
                DNC = DMN;
                DNH = OF;
                DNJ = DNK;
                DNO = DMP;
                DNQ = DMQ;
                DNV = DME;
                DNX = DNY;
                DOD = DMG;
                DOE = DOF;
                DON = DMS;
                DOP = DMT;
                DOY = DOZ;
                DPC = DPD;
                DPG = DPH;
                EKL = DAF;
            } else {
                DNA = A;
                DNC = A;
                DNH = A;
                DNJ = GT;
                DNO = A;
                DNQ = A;
                DNV = A;
                DNX = A;
                DOD = A;
                DOE = GT;
                DON = A;
                DOP = A;
                DOY = GT;
                DPC = GT;
                DPG = GT;
                EKL = A;
            }
            let DMV = DMU - node_potentials[2];
            let EKK;
            let ELX;
            let ELY;
            if PC != 0.0 {
                let DMW = DMV * HK;
                let DMX = DMW * DKP;
                let DMY = if DMX < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                let DNG;
                if DMY != 0.0 {
                    let DMZ = NA / ((-2.3025850929940458e2f64 - DMX) + GT);
                    DNG = DMZ;
                } else {
                    let DNB = if DMX > DNA { 1.0 } else { 0.0 };
                    let DNF = if DNB != 0.0 {
                        let DND = DNC * ((DMX - DNA) + GT);
                        DND
                    } else {
                        let DNE = DMX.exp();
                        DNE
                    };
                    DNG = DNF;
                }
                let DNI = DNH * (DNG - GT);
                let DNL = DMW * DNJ;
                let DNM = if DNL < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                let DNU;
                if DNM != 0.0 {
                    let DNN = NA / ((-2.3025850929940458e2f64 - DNL) + GT);
                    DNU = DNN;
                } else {
                    let DNP = if DNL > DNO { 1.0 } else { 0.0 };
                    let DNT = if DNP != 0.0 {
                        let DNR = DNQ * ((DNL - DNO) + GT);
                        DNR
                    } else {
                        let DNS = DNL.exp();
                        DNS
                    };
                    DNU = DNT;
                }
                let DNW = DNV * (DNU - GT);
                let DOC = if DNX > A { 1.0 } else { 0.0 };
                let DOV;
                if DOC != 0.0 {
                    let DOJ = DMV * (DOD + (DMV * DOE));
                    DOV = DOJ;
                } else {
                    let DOK = ((-DMV) * HK) * DOE;
                    let DOL = if DOK < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                    let DOT;
                    if DOL != 0.0 {
                        let DOM = NA / ((-2.3025850929940458e2f64 - DOK) + GT);
                        DOT = DOM;
                    } else {
                        let DOO = if DOK > DON { 1.0 } else { 0.0 };
                        let DOS = if DOO != 0.0 {
                            let DOQ = DOP * ((DOK - DON) + GT);
                            DOQ
                        } else {
                            let DOR = DOK.exp();
                            DOR
                        };
                        DOT = DOS;
                    }
                    let DOU = (-DOD) * (DOT - GT);
                    DOV = DOU;
                }
                let DOW = (DNI + DNW) + DOV;
                let DOX = DNW + DOV;
                let DPA = if DOY > GR { 1.0 } else { 0.0 };
                if DPA != 0.0 {
                    let DPB = if IO == GR { 1.0 } else { 0.0 };
                    if DPB != 0.0 {
                    } else {
                    }
                } else {
                }
                let DPE = if DPC > GR { 1.0 } else { 0.0 };
                if DPE != 0.0 {
                    let DPF = if IP == GR { 1.0 } else { 0.0 };
                    if DPF != 0.0 {
                    } else {
                    }
                } else {
                }
                let DPI = if DPG > GR { 1.0 } else { 0.0 };
                if DPI != 0.0 {
                    let DPJ = if IQ == GR { 1.0 } else { 0.0 };
                    if DPJ != 0.0 {
                    } else {
                    }
                } else {
                }
                EKK = EKL;
                ELX = DOW;
                ELY = DOX;
            } else {
                let DPK = if (if (if NG != 0.0 && NJ != 0.0 { 1.0 } else { 0.0 }) != 0.0 && NM != 0.0 { 1.0 } else { 0.0 }) == 0.0 { 1.0 } else { 0.0 };
                let DYW;
                let DZA;
                let DZC;
                let DZM;
                let EBE;
                let EBU;
                let ECL;
                let EFV;
                if DPK != 0.0 {
                    let DPL = if DMV < MU { 1.0 } else { 0.0 };
                    let DXV;
                    let DXZ;
                    let DYD;
                    let DYH;
                    if DPL != 0.0 {
                        let DPM = GR * (DMV * HK);
                        let DPN = if (DPM.abs()) < MW { 1.0 } else { 0.0 };
                        let DYI;
                        if DPN != 0.0 {
                            let DPO = DPM.exp();
                            DYI = DPO;
                        } else {
                            let DPP = if DPM < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            let DYJ = if DPP != 0.0 {
                                let DPQ = NA / (GT + ((-2.3025850929940458e2f64 - DPM) * (GT + (GR * ((-2.3025850929940458e2f64 - DPM) * (GT + ((-2.3025850929940458e2f64 - DPM) * NB)))))));
                                DPQ
                            } else {
                                let DPR = DPM - MW;
                                let DPS = ND * (GT + (DPR * (GT + (GR * (DPR * (GT + (DPR * NB)))))));
                                DPS
                            };
                            DYI = DYJ;
                        }
                        let DPT = if EV < PQ { 1.0 } else { 0.0 };
                        let DQN;
                        let DQO;
                        if DPT != 0.0 {
                            let DPU = EV - (PS * LS);
                            let DPV = (PQ - ((PS * (DMV - LS)) + EV)) - CL;
                            let DPW = (OR * PQ) * CL;
                            let DPX = if DPW > A { 1.0 } else { 0.0 };
                            let DPZ = if DPX != 0.0 {
                                DPW
                            } else {
                                let DPY = -DPW;
                                DPY
                            };
                            let DQA = ((PQ - (GR * (DPV + (((DPV * DPV) + DPZ).sqrt())))) - EV) - CL;
                            let DQB = (OR * EV) * CL;
                            let DQC = if DQB > A { 1.0 } else { 0.0 };
                            let DQE = if DQC != 0.0 {
                                DQB
                            } else {
                                let DQD = -DQB;
                                DQD
                            };
                            let DQF = EV + (GR * (DQA + (((DQA * DQA) + DQE).sqrt())));
                            let DQG = (PQ - DPU) - CL;
                            let DQI = if DPX != 0.0 {
                                DPW
                            } else {
                                let DQH = -DPW;
                                DQH
                            };
                            let DQJ = ((PQ - (GR * (DQG + (((DQG * DQG) + DQI).sqrt())))) - EV) - CL;
                            let DQL = if DQC != 0.0 {
                                DQB
                            } else {
                                let DQK = -DQB;
                                DQK
                            };
                            let DQM = EV + (GR * (DQJ + (((DQJ * DQJ) + DQL).sqrt())));
                            DQN = DQF;
                            DQO = DQM;
                        } else {
                            DQN = EV;
                            DQO = EV;
                        }
                        let DQP = HK * ((DMV / DQN) + ((LS * (DQN - DQO)) / (DQO * PQ)));
                        let DQQ = if (DQP.abs()) < MW { 1.0 } else { 0.0 };
                        let DXW;
                        if DQQ != 0.0 {
                            let DQR = DQP.exp();
                            DXW = DQR;
                        } else {
                            let DQS = if DQP < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            let DXX = if DQS != 0.0 {
                                let DQT = NA / (GT + ((-2.3025850929940458e2f64 - DQP) * (GT + (GR * ((-2.3025850929940458e2f64 - DQP) * (GT + ((-2.3025850929940458e2f64 - DQP) * NB)))))));
                                DQT
                            } else {
                                let DQU = DQP - MW;
                                let DQV = ND * (GT + (DQU * (GT + (GR * (DQU * (GT + (DQU * NB)))))));
                                DQV
                            };
                            DXW = DXX;
                        }
                        let DQW = (EY / HK) * ((LI / (LM / LI)).ln());
                        let DQX = if EY < PQ { 1.0 } else { 0.0 };
                        let DRR;
                        let DRS;
                        if DQX != 0.0 {
                            let DQY = EY - (PS * DQW);
                            let DQZ = (PQ - ((PS * (DMV - DQW)) + EY)) - CL;
                            let DRA = (OR * PQ) * CL;
                            let DRB = if DRA > A { 1.0 } else { 0.0 };
                            let DRD = if DRB != 0.0 {
                                DRA
                            } else {
                                let DRC = -DRA;
                                DRC
                            };
                            let DRE = ((PQ - (GR * (DQZ + (((DQZ * DQZ) + DRD).sqrt())))) - EY) - CL;
                            let DRF = (OR * EY) * CL;
                            let DRG = if DRF > A { 1.0 } else { 0.0 };
                            let DRI = if DRG != 0.0 {
                                DRF
                            } else {
                                let DRH = -DRF;
                                DRH
                            };
                            let DRJ = EY + (GR * (DRE + (((DRE * DRE) + DRI).sqrt())));
                            let DRK = (PQ - DQY) - CL;
                            let DRM = if DRB != 0.0 {
                                DRA
                            } else {
                                let DRL = -DRA;
                                DRL
                            };
                            let DRN = ((PQ - (GR * (DRK + (((DRK * DRK) + DRM).sqrt())))) - EY) - CL;
                            let DRP = if DRG != 0.0 {
                                DRF
                            } else {
                                let DRO = -DRF;
                                DRO
                            };
                            let DRQ = EY + (GR * (DRN + (((DRN * DRN) + DRP).sqrt())));
                            DRR = DRJ;
                            DRS = DRQ;
                        } else {
                            DRR = EY;
                            DRS = EY;
                        }
                        let DRT = HK * ((DMV / DRR) + ((DQW * (DRR - DRS)) / (DRS * PQ)));
                        let DRU = if (DRT.abs()) < MW { 1.0 } else { 0.0 };
                        let DYA;
                        if DRU != 0.0 {
                            let DRV = DRT.exp();
                            DYA = DRV;
                        } else {
                            let DRW = if DRT < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            let DYB = if DRW != 0.0 {
                                let DRX = NA / (GT + ((-2.3025850929940458e2f64 - DRT) * (GT + (GR * ((-2.3025850929940458e2f64 - DRT) * (GT + ((-2.3025850929940458e2f64 - DRT) * NB)))))));
                                DRX
                            } else {
                                let DRY = DRT - MW;
                                let DRZ = ND * (GT + (DRY * (GT + (GR * (DRY * (GT + (DRY * NB)))))));
                                DRZ
                            };
                            DYA = DYB;
                        }
                        let DSA = (FB / HK) * ((LJ / (LM / LJ)).ln());
                        let DSB = if FB < PQ { 1.0 } else { 0.0 };
                        let DSV;
                        let DSW;
                        if DSB != 0.0 {
                            let DSC = FB - (PS * DSA);
                            let DSD = (PQ - ((PS * (DMV - DSA)) + FB)) - CL;
                            let DSE = (OR * PQ) * CL;
                            let DSF = if DSE > A { 1.0 } else { 0.0 };
                            let DSH = if DSF != 0.0 {
                                DSE
                            } else {
                                let DSG = -DSE;
                                DSG
                            };
                            let DSI = ((PQ - (GR * (DSD + (((DSD * DSD) + DSH).sqrt())))) - FB) - CL;
                            let DSJ = (OR * FB) * CL;
                            let DSK = if DSJ > A { 1.0 } else { 0.0 };
                            let DSM = if DSK != 0.0 {
                                DSJ
                            } else {
                                let DSL = -DSJ;
                                DSL
                            };
                            let DSN = FB + (GR * (DSI + (((DSI * DSI) + DSM).sqrt())));
                            let DSO = (PQ - DSC) - CL;
                            let DSQ = if DSF != 0.0 {
                                DSE
                            } else {
                                let DSP = -DSE;
                                DSP
                            };
                            let DSR = ((PQ - (GR * (DSO + (((DSO * DSO) + DSQ).sqrt())))) - FB) - CL;
                            let DST = if DSK != 0.0 {
                                DSJ
                            } else {
                                let DSS = -DSJ;
                                DSS
                            };
                            let DSU = FB + (GR * (DSR + (((DSR * DSR) + DST).sqrt())));
                            DSV = DSN;
                            DSW = DSU;
                        } else {
                            DSV = FB;
                            DSW = FB;
                        }
                        let DSX = HK * ((DMV / DSV) + ((DSA * (DSV - DSW)) / (DSW * PQ)));
                        let DSY = if (DSX.abs()) < MW { 1.0 } else { 0.0 };
                        let DYE;
                        if DSY != 0.0 {
                            let DSZ = DSX.exp();
                            DYE = DSZ;
                        } else {
                            let DTA = if DSX < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            let DYF = if DTA != 0.0 {
                                let DTB = NA / (GT + ((-2.3025850929940458e2f64 - DSX) * (GT + (GR * ((-2.3025850929940458e2f64 - DSX) * (GT + ((-2.3025850929940458e2f64 - DSX) * NB)))))));
                                DTB
                            } else {
                                let DTC = DSX - MW;
                                let DTD = ND * (GT + (DTC * (GT + (GR * (DTC * (GT + (DTC * NB)))))));
                                DTD
                            };
                            DYE = DYF;
                        }
                        DXV = DXW;
                        DXZ = DYA;
                        DYD = DYE;
                        DYH = DYI;
                    } else {
                        let DTE = DMV - MU;
                        let DTF = ((GT + (DTE * HK)) * TE).sqrt();
                        let DTG = if EV < PQ { 1.0 } else { 0.0 };
                        let DUF;
                        let DUG;
                        let DUP;
                        if DTG != 0.0 {
                            let DTH = EV - (PS * LS);
                            let DTI = (PQ - ((PS * (MU - LS)) + EV)) - CL;
                            let DTJ = (OR * PQ) * CL;
                            let DTK = if DTJ > A { 1.0 } else { 0.0 };
                            let DTM = if DTK != 0.0 {
                                DTJ
                            } else {
                                let DTL = -DTJ;
                                DTL
                            };
                            let DTN = ((DTI * DTI) + DTM).sqrt();
                            let DTO = GR * (GT + (DTI / DTN));
                            let DTP = ((PQ - (GR * (DTI + DTN))) - EV) - CL;
                            let DTQ = (OR * EV) * CL;
                            let DTR = if DTQ > A { 1.0 } else { 0.0 };
                            let DTT = if DTR != 0.0 {
                                DTQ
                            } else {
                                let DTS = -DTQ;
                                DTS
                            };
                            let DTU = ((DTP * DTP) + DTT).sqrt();
                            let DTV = GR * (GT + (DTP / DTU));
                            let DTW = EV + (GR * (DTP + DTU));
                            let DTX = (PQ - DTH) - CL;
                            let DTZ = if DTK != 0.0 {
                                DTJ
                            } else {
                                let DTY = -DTJ;
                                DTY
                            };
                            let DUA = ((PQ - (GR * (DTX + (((DTX * DTX) + DTZ).sqrt())))) - EV) - CL;
                            let DUC = if DTR != 0.0 {
                                DTQ
                            } else {
                                let DUB = -DTQ;
                                DUB
                            };
                            let DUD = EV + (GR * (DUA + (((DUA * DUA) + DUC).sqrt())));
                            let DUE = (PS * DTO) * DTV;
                            DUF = DTW;
                            DUG = DUD;
                            DUP = DUE;
                        } else {
                            DUF = EV;
                            DUG = EV;
                            DUP = A;
                        }
                        let DUH = DUG * PQ;
                        let DUI = HK * ((MU / DUF) + ((LS * (DUF - DUG)) / DUH));
                        let DUJ = if (DUI.abs()) < MW { 1.0 } else { 0.0 };
                        let DUQ;
                        if DUJ != 0.0 {
                            let DUK = DUI.exp();
                            DUQ = DUK;
                        } else {
                            let DUL = if DUI < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            let DUR = if DUL != 0.0 {
                                let DUM = NA / (GT + ((-2.3025850929940458e2f64 - DUI) * (GT + (GR * ((-2.3025850929940458e2f64 - DUI) * (GT + ((-2.3025850929940458e2f64 - DUI) * NB)))))));
                                DUM
                            } else {
                                let DUN = DUI - MW;
                                let DUO = ND * (GT + (DUN * (GT + (GR * (DUN * (GT + (DUN * NB)))))));
                                DUO
                            };
                            DUQ = DUR;
                        }
                        let DUS = (GT + (DTE * (HK * (((DUF - (MU * DUP)) / (DUF * DUF)) + ((LS * DUP) / DUH))))) * DUQ;
                        let DUT = (EY / HK) * ((LI / (LM / LI)).ln());
                        let DUU = if EY < PQ { 1.0 } else { 0.0 };
                        let DVT;
                        let DVU;
                        let DWD;
                        if DUU != 0.0 {
                            let DUV = EY - (PS * DUT);
                            let DUW = (PQ - ((PS * (MU - DUT)) + EY)) - CL;
                            let DUX = (OR * PQ) * CL;
                            let DUY = if DUX > A { 1.0 } else { 0.0 };
                            let DVA = if DUY != 0.0 {
                                DUX
                            } else {
                                let DUZ = -DUX;
                                DUZ
                            };
                            let DVB = ((DUW * DUW) + DVA).sqrt();
                            let DVC = GR * (GT + (DUW / DVB));
                            let DVD = ((PQ - (GR * (DUW + DVB))) - EY) - CL;
                            let DVE = (OR * EY) * CL;
                            let DVF = if DVE > A { 1.0 } else { 0.0 };
                            let DVH = if DVF != 0.0 {
                                DVE
                            } else {
                                let DVG = -DVE;
                                DVG
                            };
                            let DVI = ((DVD * DVD) + DVH).sqrt();
                            let DVJ = GR * (GT + (DVD / DVI));
                            let DVK = EY + (GR * (DVD + DVI));
                            let DVL = (PQ - DUV) - CL;
                            let DVN = if DUY != 0.0 {
                                DUX
                            } else {
                                let DVM = -DUX;
                                DVM
                            };
                            let DVO = ((PQ - (GR * (DVL + (((DVL * DVL) + DVN).sqrt())))) - EY) - CL;
                            let DVQ = if DVF != 0.0 {
                                DVE
                            } else {
                                let DVP = -DVE;
                                DVP
                            };
                            let DVR = EY + (GR * (DVO + (((DVO * DVO) + DVQ).sqrt())));
                            let DVS = (PS * DVC) * DVJ;
                            DVT = DVK;
                            DVU = DVR;
                            DWD = DVS;
                        } else {
                            DVT = EY;
                            DVU = EY;
                            DWD = A;
                        }
                        let DVV = DVU * PQ;
                        let DVW = HK * ((MU / DVT) + ((DUT * (DVT - DVU)) / DVV));
                        let DVX = if (DVW.abs()) < MW { 1.0 } else { 0.0 };
                        let DWE;
                        if DVX != 0.0 {
                            let DVY = DVW.exp();
                            DWE = DVY;
                        } else {
                            let DVZ = if DVW < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            let DWF = if DVZ != 0.0 {
                                let DWA = NA / (GT + ((-2.3025850929940458e2f64 - DVW) * (GT + (GR * ((-2.3025850929940458e2f64 - DVW) * (GT + ((-2.3025850929940458e2f64 - DVW) * NB)))))));
                                DWA
                            } else {
                                let DWB = DVW - MW;
                                let DWC = ND * (GT + (DWB * (GT + (GR * (DWB * (GT + (DWB * NB)))))));
                                DWC
                            };
                            DWE = DWF;
                        }
                        let DWG = (GT + (DTE * (HK * (((DVT - (MU * DWD)) / (DVT * DVT)) + ((DUT * DWD) / DVV))))) * DWE;
                        let DWH = (FB / HK) * ((LJ / (LM / LJ)).ln());
                        let DWI = if FB < PQ { 1.0 } else { 0.0 };
                        let DXH;
                        let DXI;
                        let DXR;
                        if DWI != 0.0 {
                            let DWJ = FB - (PS * DWH);
                            let DWK = (PQ - ((PS * (MU - DWH)) + FB)) - CL;
                            let DWL = (OR * PQ) * CL;
                            let DWM = if DWL > A { 1.0 } else { 0.0 };
                            let DWO = if DWM != 0.0 {
                                DWL
                            } else {
                                let DWN = -DWL;
                                DWN
                            };
                            let DWP = ((DWK * DWK) + DWO).sqrt();
                            let DWQ = GR * (GT + (DWK / DWP));
                            let DWR = ((PQ - (GR * (DWK + DWP))) - FB) - CL;
                            let DWS = (OR * FB) * CL;
                            let DWT = if DWS > A { 1.0 } else { 0.0 };
                            let DWV = if DWT != 0.0 {
                                DWS
                            } else {
                                let DWU = -DWS;
                                DWU
                            };
                            let DWW = ((DWR * DWR) + DWV).sqrt();
                            let DWX = GR * (GT + (DWR / DWW));
                            let DWY = FB + (GR * (DWR + DWW));
                            let DWZ = (PQ - DWJ) - CL;
                            let DXB = if DWM != 0.0 {
                                DWL
                            } else {
                                let DXA = -DWL;
                                DXA
                            };
                            let DXC = ((PQ - (GR * (DWZ + (((DWZ * DWZ) + DXB).sqrt())))) - FB) - CL;
                            let DXE = if DWT != 0.0 {
                                DWS
                            } else {
                                let DXD = -DWS;
                                DXD
                            };
                            let DXF = FB + (GR * (DXC + (((DXC * DXC) + DXE).sqrt())));
                            let DXG = (PS * DWQ) * DWX;
                            DXH = DWY;
                            DXI = DXF;
                            DXR = DXG;
                        } else {
                            DXH = FB;
                            DXI = FB;
                            DXR = A;
                        }
                        let DXJ = DXI * PQ;
                        let DXK = HK * ((MU / DXH) + ((DWH * (DXH - DXI)) / DXJ));
                        let DXL = if (DXK.abs()) < MW { 1.0 } else { 0.0 };
                        let DXS;
                        if DXL != 0.0 {
                            let DXM = DXK.exp();
                            DXS = DXM;
                        } else {
                            let DXN = if DXK < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            let DXT = if DXN != 0.0 {
                                let DXO = NA / (GT + ((-2.3025850929940458e2f64 - DXK) * (GT + (GR * ((-2.3025850929940458e2f64 - DXK) * (GT + ((-2.3025850929940458e2f64 - DXK) * NB)))))));
                                DXO
                            } else {
                                let DXP = DXK - MW;
                                let DXQ = ND * (GT + (DXP * (GT + (GR * (DXP * (GT + (DXP * NB)))))));
                                DXQ
                            };
                            DXS = DXT;
                        }
                        let DXU = (GT + (DTE * (HK * (((DXH - (MU * DXR)) / (DXH * DXH)) + ((DWH * DXR) / DXJ))))) * DXS;
                        DXV = DUS;
                        DXZ = DWG;
                        DYD = DXU;
                        DYH = DTF;
                    }
                    let DXY = DXV - GT;
                    let DYC = DXZ - GT;
                    let DYG = DYD - GT;
                    let DYK = GT / DYH;
                    let DYL = if DMV > A { 1.0 } else { 0.0 };
                    let DYO = if DYL != 0.0 {
                        let DYM = HS * (HJ * (((HS + DYK) + (((DYK + GT) * (DYK + JI)).sqrt())).ln()));
                        DYM
                    } else {
                        let DYN = (-DMV) + (HS * (HJ * ((((HS * DYH) + GT) + (((GT + DYH) * (GT + (JI * DYH))).sqrt())).ln())));
                        DYN
                    };
                    let DYP = NS - DYO;
                    let DYQ = DMV - DYP;
                    let DYR = GR * ((DMV + DYP) - (((DYQ * DYQ) + ((OR * HJ) * HJ)).sqrt()));
                    let DYS = DMV - NW;
                    let DYT = GR * ((DMV + NW) - (((DYS * DYS) + ((OR * HH) * HH)).sqrt()));
                    let DYU = GR * (DMV - (((DMV * DMV) + 4e-12f64).sqrt()));
                    DYW = DXY;
                    DZA = DYR;
                    DZC = DYO;
                    DZM = DYH;
                    EBE = DYT;
                    EBU = DYU;
                    ECL = DYC;
                    EFV = DYG;
                } else {
                    DYW = A;
                    DZA = A;
                    DZC = A;
                    DZM = A;
                    EBE = A;
                    EBU = A;
                    ECL = A;
                    EFV = A;
                }
                let EJE;
                let EJI;
                if NG != 0.0 {
                    EJE = A;
                    EJI = A;
                } else {
                    let DYV = if IO == GR { 1.0 } else { 0.0 };
                    if DYV != 0.0 {
                    } else {
                    }
                    let DYX = IE * DYW;
                    let DYY = if CD == A { 1.0 } else { 0.0 };
                    let DYZ = if (if BN == A { 1.0 } else { 0.0 }) != 0.0 && DYY != 0.0 { 1.0 } else { 0.0 };
                    let DZP;
                    let DZQ;
                    let EAC;
                    let EBA;
                    let ECD;
                    if DYZ != 0.0 {
                        DZP = A;
                        DZQ = A;
                        EAC = A;
                        EBA = A;
                        ECD = A;
                    } else {
                        let DZB = IL - DZA;
                        let DZD = GT - ((GT - (DZC / DZB)).sqrt());
                        let DZE = if AO == GR { 1.0 } else { 0.0 };
                        let DZG = if DZE != 0.0 {
                            A
                        } else {
                            let DZF = ((((DZD * DZD) * (DZD.ln())) / (GT - DZD)) + DZD) * (GT - (HS * AO));
                            DZF
                        };
                        let DZH = DZD + DZG;
                        let DZK = if DZE != 0.0 {
                            let DZI = (DZB * JD).sqrt();
                            DZI
                        } else {
                            let DZJ = (DZB * JD).powf(AO);
                            DZJ
                        };
                        let DZL = IX * DZK;
                        let DZN = HW * ((DZM - GT) * DZL);
                        let DZO = BN * (DZN * DZH);
                        DZP = DZL;
                        DZQ = DZB;
                        EAC = DZH;
                        EBA = DZN;
                        ECD = DZO;
                    }
                    let ECE;
                    if DYY != 0.0 {
                        ECE = A;
                    } else {
                        let DZR = JT * ((DZP * IO) / DZQ);
                        let DZS = (ZT * JO) / DZR;
                        let DZT = DZS * DZS;
                        let DZU = DZT * DZT;
                        let DZV = (DZU / (DZU + GT)).sqrt();
                        let DZW = (DZV.abs()).sqrt();
                        let DZX = DZV * DZW;
                        let DZY = (-AO) * IR;
                        let DZZ = if DZY == -1e0f64 { 1.0 } else { 0.0 };
                        let EAD = if DZZ != 0.0 {
                            let EAA = GT / (GT + (DZR * DZX));
                            EAA
                        } else {
                            let EAB = (GT + (DZR * DZX)).powf(DZY);
                            EAB
                        };
                        let EAE = (EAC * EAD) / (EAC + EAD);
                        let EAF = (AAH * (DZR / DZW)).sqrt();
                        let EAG = (((JO * DZS) * DZW) - (JO * DZV)) + (GR * (DZR * DZX));
                        let EAH = (((HS * (DZS * DZW)) - DZV) - GT) * EAF;
                        let EAI = EAH * EAH;
                        let EAJ = if EAH > A { 1.0 } else { 0.0 };
                        let EAQ = if EAJ != 0.0 {
                            let EAK = GT / (GT + (JH * EAH));
                            EAK
                        } else {
                            let EAL = GT / (GT - (JH * EAH));
                            EAL
                        };
                        let EAM = (-EAI) + EAG;
                        let EAN = if EAM > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                        let EAS = if EAN != 0.0 {
                            let EAO = EAM.exp();
                            EAO
                        } else {
                            let EAP = NA / (GT + ((-2.3025850929940458e2f64 - EAM) * (GT + (GR * ((-2.3025850929940458e2f64 - EAM) * (GT + ((-2.3025850929940458e2f64 - EAM) * NB)))))));
                            EAP
                        };
                        let EAR = EAQ * EAQ;
                        let EAT = (((JG * EAQ) + (JJ * EAR)) + (JK * (EAR * EAQ))) * EAS;
                        let EAZ;
                        if EAJ != 0.0 {
                            EAZ = EAT;
                        } else {
                            let EAU = if EAG > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            let EAX = if EAU != 0.0 {
                                let EAV = EAG.exp();
                                EAV
                            } else {
                                let EAW = NA / (GT + ((-2.3025850929940458e2f64 - EAG) * (GT + (GR * ((-2.3025850929940458e2f64 - EAG) * (GT + ((-2.3025850929940458e2f64 - EAG) * NB)))))));
                                EAW
                            };
                            let EAY = (HS * EAX) - EAT;
                            EAZ = EAY;
                        }
                        let EBB = CD * ((EBA * (8.86226925452758e-1f64 * ((JO * EAZ) / EAF))) * EAE);
                        ECE = EBB;
                    }
                    let EBC = if CW == A { 1.0 } else { 0.0 };
                    let ECF;
                    if EBC != 0.0 {
                        ECF = A;
                    } else {
                        let EBD = if AO == GR { 1.0 } else { 0.0 };
                        let EBH = if EBD != 0.0 {
                            let EBF = ((AC - EBE) * JD).sqrt();
                            EBF
                        } else {
                            let EBG = ((AC - EBE) * JD).powf(AO);
                            EBG
                        };
                        let EBI = IR * (((AC - EBE) * JA) / EBH);
                        let EBJ = (-KB) / EBI;
                        let EBK = if (EBJ.abs()) < MW { 1.0 } else { 0.0 };
                        let EBQ;
                        if EBK != 0.0 {
                            let EBL = EBJ.exp();
                            EBQ = EBL;
                        } else {
                            let EBM = if EBJ < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            let EBR = if EBM != 0.0 {
                                let EBN = NA / (GT + ((-2.3025850929940458e2f64 - EBJ) * (GT + (GR * ((-2.3025850929940458e2f64 - EBJ) * (GT + ((-2.3025850929940458e2f64 - EBJ) * NB)))))));
                                EBN
                            } else {
                                let EBO = EBJ - MW;
                                let EBP = ND * (GT + (EBO * (GT + (GR * (EBO * (GT + (EBO * NB)))))));
                                EBP
                            };
                            EBQ = EBR;
                        }
                        let EBS = CW * (((DMV * EBI) * EBI) * EBQ);
                        ECF = EBS;
                    }
                    let EBT = if (if ABW > LG { 1.0 } else { 0.0 }) != 0.0 || (if ABX == A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let ECG;
                    if EBT != 0.0 {
                        ECG = GT;
                    } else {
                        let EBV = if EBU > ((-KG) * ABW) { 1.0 } else { 0.0 };
                        let ECH;
                        if EBV != 0.0 {
                            let EBW = if DP == OR { 1.0 } else { 0.0 };
                            let ECA = if EBW != 0.0 {
                                let EBX = (EBU * KV).abs();
                                let EBY = ((EBX * EBX) * EBX) * EBX;
                                EBY
                            } else {
                                let EBZ = ((EBU * KV).abs()).powf(DP);
                                EBZ
                            };
                            let ECB = GT / (GT - ECA);
                            ECH = ECB;
                        } else {
                            let ECC = KH + ((EBU + (KG * ABW)) * KW);
                            ECH = ECC;
                        }
                        ECG = ECH;
                    }
                    let ECI = (((DYX + ECD) + ECE) + ECF) * ECG;
                    let ECJ = ((ECD + ECE) + ECF) * ECG;
                    EJE = ECI;
                    EJI = ECJ;
                }
                let EJF;
                let EJJ;
                if NJ != 0.0 {
                    EJF = A;
                    EJJ = A;
                } else {
                    let ECK = if IP == GR { 1.0 } else { 0.0 };
                    if ECK != 0.0 {
                    } else {
                    }
                    let ECM = IF * ECL;
                    let ECN = if CG == A { 1.0 } else { 0.0 };
                    let ECO = if (if BQ == A { 1.0 } else { 0.0 }) != 0.0 && ECN != 0.0 { 1.0 } else { 0.0 };
                    let EDB;
                    let EDC;
                    let EDO;
                    let EEM;
                    let EFN;
                    if ECO != 0.0 {
                        EDB = A;
                        EDC = A;
                        EDO = A;
                        EEM = A;
                        EFN = A;
                    } else {
                        let ECP = IM - DZA;
                        let ECQ = GT - ((GT - (DZC / ECP)).sqrt());
                        let ECR = if AT == GR { 1.0 } else { 0.0 };
                        let ECT = if ECR != 0.0 {
                            A
                        } else {
                            let ECS = ((((ECQ * ECQ) * (ECQ.ln())) / (GT - ECQ)) + ECQ) * (GT - (HS * AT));
                            ECS
                        };
                        let ECU = ECQ + ECT;
                        let ECX = if ECR != 0.0 {
                            let ECV = (ECP * JE).sqrt();
                            ECV
                        } else {
                            let ECW = (ECP * JE).powf(AT);
                            ECW
                        };
                        let ECY = IY * ECX;
                        let ECZ = HY * ((DZM - GT) * ECY);
                        let EDA = BQ * (ECZ * ECU);
                        EDB = ECY;
                        EDC = ECP;
                        EDO = ECU;
                        EEM = ECZ;
                        EFN = EDA;
                    }
                    let EFO;
                    if ECN != 0.0 {
                        EFO = A;
                    } else {
                        let EDD = JU * ((EDB * IP) / EDC);
                        let EDE = (ZT * JP) / EDD;
                        let EDF = EDE * EDE;
                        let EDG = EDF * EDF;
                        let EDH = (EDG / (EDG + GT)).sqrt();
                        let EDI = (EDH.abs()).sqrt();
                        let EDJ = EDH * EDI;
                        let EDK = (-AT) * IS;
                        let EDL = if EDK == -1e0f64 { 1.0 } else { 0.0 };
                        let EDP = if EDL != 0.0 {
                            let EDM = GT / (GT + (EDD * EDJ));
                            EDM
                        } else {
                            let EDN = (GT + (EDD * EDJ)).powf(EDK);
                            EDN
                        };
                        let EDQ = (EDO * EDP) / (EDO + EDP);
                        let EDR = (AAH * (EDD / EDI)).sqrt();
                        let EDS = (((JP * EDE) * EDI) - (JP * EDH)) + (GR * (EDD * EDJ));
                        let EDT = (((HS * (EDE * EDI)) - EDH) - GT) * EDR;
                        let EDU = EDT * EDT;
                        let EDV = if EDT > A { 1.0 } else { 0.0 };
                        let EEC = if EDV != 0.0 {
                            let EDW = GT / (GT + (JH * EDT));
                            EDW
                        } else {
                            let EDX = GT / (GT - (JH * EDT));
                            EDX
                        };
                        let EDY = (-EDU) + EDS;
                        let EDZ = if EDY > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                        let EEE = if EDZ != 0.0 {
                            let EEA = EDY.exp();
                            EEA
                        } else {
                            let EEB = NA / (GT + ((-2.3025850929940458e2f64 - EDY) * (GT + (GR * ((-2.3025850929940458e2f64 - EDY) * (GT + ((-2.3025850929940458e2f64 - EDY) * NB)))))));
                            EEB
                        };
                        let EED = EEC * EEC;
                        let EEF = (((JG * EEC) + (JJ * EED)) + (JK * (EED * EEC))) * EEE;
                        let EEL;
                        if EDV != 0.0 {
                            EEL = EEF;
                        } else {
                            let EEG = if EDS > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            let EEJ = if EEG != 0.0 {
                                let EEH = EDS.exp();
                                EEH
                            } else {
                                let EEI = NA / (GT + ((-2.3025850929940458e2f64 - EDS) * (GT + (GR * ((-2.3025850929940458e2f64 - EDS) * (GT + ((-2.3025850929940458e2f64 - EDS) * NB)))))));
                                EEI
                            };
                            let EEK = (HS * EEJ) - EEF;
                            EEL = EEK;
                        }
                        let EEN = CG * ((EEM * (8.86226925452758e-1f64 * ((JP * EEL) / EDR))) * EDQ);
                        EFO = EEN;
                    }
                    let EEO = if CZ == A { 1.0 } else { 0.0 };
                    let EFP;
                    if EEO != 0.0 {
                        EFP = A;
                    } else {
                        let EEP = if AT == GR { 1.0 } else { 0.0 };
                        let EES = if EEP != 0.0 {
                            let EEQ = ((AF - EBE) * JE).sqrt();
                            EEQ
                        } else {
                            let EER = ((AF - EBE) * JE).powf(AT);
                            EER
                        };
                        let EET = IS * (((AF - EBE) * JB) / EES);
                        let EEU = (-KD) / EET;
                        let EEV = if (EEU.abs()) < MW { 1.0 } else { 0.0 };
                        let EFB;
                        if EEV != 0.0 {
                            let EEW = EEU.exp();
                            EFB = EEW;
                        } else {
                            let EEX = if EEU < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            let EFC = if EEX != 0.0 {
                                let EEY = NA / (GT + ((-2.3025850929940458e2f64 - EEU) * (GT + (GR * ((-2.3025850929940458e2f64 - EEU) * (GT + ((-2.3025850929940458e2f64 - EEU) * NB)))))));
                                EEY
                            } else {
                                let EEZ = EEU - MW;
                                let EFA = ND * (GT + (EEZ * (GT + (GR * (EEZ * (GT + (EEZ * NB)))))));
                                EFA
                            };
                            EFB = EFC;
                        }
                        let EFD = CZ * (((DMV * EET) * EET) * EFB);
                        EFP = EFD;
                    }
                    let EFE = if (if AFI > LG { 1.0 } else { 0.0 }) != 0.0 || (if ABX == A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let EFQ;
                    if EFE != 0.0 {
                        EFQ = GT;
                    } else {
                        let EFF = if EBU > ((-KG) * AFI) { 1.0 } else { 0.0 };
                        let EFR;
                        if EFF != 0.0 {
                            let EFG = if DS == OR { 1.0 } else { 0.0 };
                            let EFK = if EFG != 0.0 {
                                let EFH = (EBU * KX).abs();
                                let EFI = ((EFH * EFH) * EFH) * EFH;
                                EFI
                            } else {
                                let EFJ = ((EBU * KX).abs()).powf(DS);
                                EFJ
                            };
                            let EFL = GT / (GT - EFK);
                            EFR = EFL;
                        } else {
                            let EFM = KI + ((EBU + (KG * AFI)) * KY);
                            EFR = EFM;
                        }
                        EFQ = EFR;
                    }
                    let EFS = (((ECM + EFN) + EFO) + EFP) * EFQ;
                    let EFT = ((EFN + EFO) + EFP) * EFQ;
                    EJF = EFS;
                    EJJ = EFT;
                }
                let EJG;
                let EJK;
                if NM != 0.0 {
                    EJG = A;
                    EJK = A;
                } else {
                    let EFU = if IQ == GR { 1.0 } else { 0.0 };
                    if EFU != 0.0 {
                    } else {
                    }
                    let EFW = IG * EFV;
                    let EFX = if CJ == A { 1.0 } else { 0.0 };
                    let EFY = if (if BT == A { 1.0 } else { 0.0 }) != 0.0 && EFX != 0.0 { 1.0 } else { 0.0 };
                    let EGL;
                    let EGM;
                    let EGY;
                    let EHW;
                    let EIX;
                    if EFY != 0.0 {
                        EGL = A;
                        EGM = A;
                        EGY = A;
                        EHW = A;
                        EIX = A;
                    } else {
                        let EFZ = IN - DZA;
                        let EGA = GT - ((GT - (DZC / EFZ)).sqrt());
                        let EGB = if AY == GR { 1.0 } else { 0.0 };
                        let EGD = if EGB != 0.0 {
                            A
                        } else {
                            let EGC = ((((EGA * EGA) * (EGA.ln())) / (GT - EGA)) + EGA) * (GT - (HS * AY));
                            EGC
                        };
                        let EGE = EGA + EGD;
                        let EGH = if EGB != 0.0 {
                            let EGF = (EFZ * JF).sqrt();
                            EGF
                        } else {
                            let EGG = (EFZ * JF).powf(AY);
                            EGG
                        };
                        let EGI = IZ * EGH;
                        let EGJ = IA * ((DZM - GT) * EGI);
                        let EGK = BT * (EGJ * EGE);
                        EGL = EGI;
                        EGM = EFZ;
                        EGY = EGE;
                        EHW = EGJ;
                        EIX = EGK;
                    }
                    let EIY;
                    if EFX != 0.0 {
                        EIY = A;
                    } else {
                        let EGN = JV * ((EGL * IQ) / EGM);
                        let EGO = (ZT * JQ) / EGN;
                        let EGP = EGO * EGO;
                        let EGQ = EGP * EGP;
                        let EGR = (EGQ / (EGQ + GT)).sqrt();
                        let EGS = (EGR.abs()).sqrt();
                        let EGT = EGR * EGS;
                        let EGU = (-AY) * IT;
                        let EGV = if EGU == -1e0f64 { 1.0 } else { 0.0 };
                        let EGZ = if EGV != 0.0 {
                            let EGW = GT / (GT + (EGN * EGT));
                            EGW
                        } else {
                            let EGX = (GT + (EGN * EGT)).powf(EGU);
                            EGX
                        };
                        let EHA = (EGY * EGZ) / (EGY + EGZ);
                        let EHB = (AAH * (EGN / EGS)).sqrt();
                        let EHC = (((JQ * EGO) * EGS) - (JQ * EGR)) + (GR * (EGN * EGT));
                        let EHD = (((HS * (EGO * EGS)) - EGR) - GT) * EHB;
                        let EHE = EHD * EHD;
                        let EHF = if EHD > A { 1.0 } else { 0.0 };
                        let EHM = if EHF != 0.0 {
                            let EHG = GT / (GT + (JH * EHD));
                            EHG
                        } else {
                            let EHH = GT / (GT - (JH * EHD));
                            EHH
                        };
                        let EHI = (-EHE) + EHC;
                        let EHJ = if EHI > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                        let EHO = if EHJ != 0.0 {
                            let EHK = EHI.exp();
                            EHK
                        } else {
                            let EHL = NA / (GT + ((-2.3025850929940458e2f64 - EHI) * (GT + (GR * ((-2.3025850929940458e2f64 - EHI) * (GT + ((-2.3025850929940458e2f64 - EHI) * NB)))))));
                            EHL
                        };
                        let EHN = EHM * EHM;
                        let EHP = (((JG * EHM) + (JJ * EHN)) + (JK * (EHN * EHM))) * EHO;
                        let EHV;
                        if EHF != 0.0 {
                            EHV = EHP;
                        } else {
                            let EHQ = if EHC > -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            let EHT = if EHQ != 0.0 {
                                let EHR = EHC.exp();
                                EHR
                            } else {
                                let EHS = NA / (GT + ((-2.3025850929940458e2f64 - EHC) * (GT + (GR * ((-2.3025850929940458e2f64 - EHC) * (GT + ((-2.3025850929940458e2f64 - EHC) * NB)))))));
                                EHS
                            };
                            let EHU = (HS * EHT) - EHP;
                            EHV = EHU;
                        }
                        let EHX = CJ * ((EHW * (8.86226925452758e-1f64 * ((JQ * EHV) / EHB))) * EHA);
                        EIY = EHX;
                    }
                    let EHY = if DC == A { 1.0 } else { 0.0 };
                    let EIZ;
                    if EHY != 0.0 {
                        EIZ = A;
                    } else {
                        let EHZ = if AY == GR { 1.0 } else { 0.0 };
                        let EIC = if EHZ != 0.0 {
                            let EIA = ((AI - EBE) * JF).sqrt();
                            EIA
                        } else {
                            let EIB = ((AI - EBE) * JF).powf(AY);
                            EIB
                        };
                        let EID = IT * (((AI - EBE) * JC) / EIC);
                        let EIE = (-KF) / EID;
                        let EIF = if (EIE.abs()) < MW { 1.0 } else { 0.0 };
                        let EIL;
                        if EIF != 0.0 {
                            let EIG = EIE.exp();
                            EIL = EIG;
                        } else {
                            let EIH = if EIE < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                            let EIM = if EIH != 0.0 {
                                let EII = NA / (GT + ((-2.3025850929940458e2f64 - EIE) * (GT + (GR * ((-2.3025850929940458e2f64 - EIE) * (GT + ((-2.3025850929940458e2f64 - EIE) * NB)))))));
                                EII
                            } else {
                                let EIJ = EIE - MW;
                                let EIK = ND * (GT + (EIJ * (GT + (GR * (EIJ * (GT + (EIJ * NB)))))));
                                EIK
                            };
                            EIL = EIM;
                        }
                        let EIN = DC * (((DMV * EID) * EID) * EIL);
                        EIZ = EIN;
                    }
                    let EIO = if (if AIS > LG { 1.0 } else { 0.0 }) != 0.0 || (if ABX == A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let EJA;
                    if EIO != 0.0 {
                        EJA = GT;
                    } else {
                        let EIP = if EBU > ((-KG) * AIS) { 1.0 } else { 0.0 };
                        let EJB;
                        if EIP != 0.0 {
                            let EIQ = if DV == OR { 1.0 } else { 0.0 };
                            let EIU = if EIQ != 0.0 {
                                let EIR = (EBU * KZ).abs();
                                let EIS = ((EIR * EIR) * EIR) * EIR;
                                EIS
                            } else {
                                let EIT = ((EBU * KZ).abs()).powf(DV);
                                EIT
                            };
                            let EIV = GT / (GT - EIU);
                            EJB = EIV;
                        } else {
                            let EIW = KJ + ((EBU + (KG * AIS)) * LA);
                            EJB = EIW;
                        }
                        EJA = EJB;
                    }
                    let EJC = (((EFW + EIX) + EIY) + EIZ) * EJA;
                    let EJD = ((EIX + EIY) + EIZ) * EJA;
                    EJG = EJC;
                    EJK = EJD;
                }
                let EJH = ((LY * EJE) + (MC * EJF)) + (MG * EJG);
                let EJL = ((LY * EJI) + (MC * EJJ)) + (MG * EJK);
                EKK = DYW;
                ELX = EJH;
                ELY = EJL;
            }
            let EJM = DMU - node_potentials[1];
            let EJN = if (if EJM > GE { 1.0 } else { 0.0 }) != 0.0 && (if GE > M { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            if EJN != 0.0 {
            } else {
            }
            let EJO = if (if EJM < (-1e0f64 * GH) { 1.0 } else { 0.0 }) != 0.0 && (if GH > M { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            if EJO != 0.0 {
            } else {
            }
            let EJP = if parameters[84] > A { 1.0 } else { 0.0 };
            if EJP != 0.0 {
                let EJQ = if EV < PQ { 1.0 } else { 0.0 };
                let EKO;
                let EKP;
                if EJQ != 0.0 {
                    let EJR = EV - (PS * LU);
                    let EJS = (PQ - ((PS * (DMV - LU)) + EV)) - CL;
                    let EJT = (OR * PQ) * CL;
                    let EJU = if EJT > A { 1.0 } else { 0.0 };
                    let EJW = if EJU != 0.0 {
                        EJT
                    } else {
                        let EJV = -EJT;
                        EJV
                    };
                    let EJX = ((PQ - (GR * (EJS + (((EJS * EJS) + EJW).sqrt())))) - EV) - CL;
                    let EJY = (OR * EV) * CL;
                    let EJZ = if EJY > A { 1.0 } else { 0.0 };
                    let EKB = if EJZ != 0.0 {
                        EJY
                    } else {
                        let EKA = -EJY;
                        EKA
                    };
                    let EKC = EV + (GR * (EJX + (((EJX * EJX) + EKB).sqrt())));
                    let EKD = (PQ - EJR) - CL;
                    let EKF = if EJU != 0.0 {
                        EJT
                    } else {
                        let EKE = -EJT;
                        EKE
                    };
                    let EKG = ((PQ - (GR * (EKD + (((EKD * EKD) + EKF).sqrt())))) - EV) - CL;
                    let EKI = if EJZ != 0.0 {
                        EJY
                    } else {
                        let EKH = -EJY;
                        EKH
                    };
                    let EKJ = EV + (GR * (EKG + (((EKG * EKG) + EKI).sqrt())));
                    EKO = EKC;
                    EKP = EKJ;
                } else {
                    EKO = EV;
                    EKP = EV;
                }
                let EKM = LU - LS;
                let EKN = if (DMV - EKM) > A { 1.0 } else { 0.0 };
                let ELL;
                if EKN != 0.0 {
                    let EKQ = HK * (((DMV / EKO) - (EKM / EKO)) + ((LU * (EKO - EKP)) / (EKP * PQ)));
                    let EKR = if (EKQ.abs()) < MW { 1.0 } else { 0.0 };
                    let ELM;
                    if EKR != 0.0 {
                        let EKS = EKQ.exp();
                        ELM = EKS;
                    } else {
                        let EKT = if EKQ < -2.3025850929940458e2f64 { 1.0 } else { 0.0 };
                        let ELN = if EKT != 0.0 {
                            let EKU = NA / (GT + ((-2.3025850929940458e2f64 - EKQ) * (GT + (GR * ((-2.3025850929940458e2f64 - EKQ) * (GT + ((-2.3025850929940458e2f64 - EKQ) * NB)))))));
                            EKU
                        } else {
                            let EKV = EKQ - MW;
                            let EKW = ND * (GT + (EKV * (GT + (GR * (EKV * (GT + (EKV * NB)))))));
                            EKW
                        };
                        ELM = ELN;
                    }
                    ELL = ELM;
                } else {
                    ELL = GT;
                }
                let EKY = if EKX == A { 1.0 } else { 0.0 };
                let EKZ = if EKY != 0.0 || (if DMV < LS { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let ELF = if EKZ != 0.0 {
                    let ELB = EKK * ELA;
                    ELB
                } else {
                    let ELC = DMV - LS;
                    let ELE = (EKK * ELA) * (((((-EKX) * ELC) * ELC) * ((ELD * ((HC / HD).ln())).exp())).exp());
                    ELE
                };
                let ELH = if ELF > ELG { 1.0 } else { 0.0 };
                if ELH != 0.0 {
                } else {
                }
                let ELJ = if ELI > A { 1.0 } else { 0.0 };
                if ELJ != 0.0 {
                } else {
                }
                let ELK = if EKY != 0.0 || (if DMV < LU { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let ELR = if ELK != 0.0 {
                    let ELO = ELL * ELA;
                    ELO
                } else {
                    let ELP = DMV - LU;
                    let ELQ = (ELL * ELA) * (((((-EKX) * ELP) * ELP) * ((ELD * ((HC / HD).ln())).exp())).exp());
                    ELQ
                };
                let ELS = if ELR > ELG { 1.0 } else { 0.0 };
                if ELS != 0.0 {
                } else {
                }
                if ELJ != 0.0 {
                } else {
                }
                let ELT = 6e-1f64 - DMV;
                let ELU = if (GR * (ELT + (((ELT * ELT) + 4e-6f64).sqrt()))) < A { 1.0 } else { 0.0 };
                if ELU != 0.0 {
                } else {
                }
                if OS != 0.0 {
                } else {
                }
                let ELW = if ELV > A { 1.0 } else { 0.0 };
                if ELW != 0.0 {
                } else {
                }
            } else {
            }
            let ELZ = 3.2043836e-19f64 * (((ELX - ELY) + (HS * OF)) + (ELY.abs()));
            let EMA = ER * ((ELX.abs()).powf(EO));
            let EMC = if EMB >= parameters[4] { 1.0 } else { 0.0 };
            let EMD = if (if EMB > A { 1.0 } else { 0.0 }) != 0.0 && EMC != 0.0 { 1.0 } else { 0.0 };
            let EMF = if EMD != 0.0 {
                let EME = (5.522602e-23f64 * HD) / EMB;
                EME
            } else {
                A
            };
            if EMD != 0.0 {
            } else {
            }
            let EMG = if EJP != 0.0 && (if ELI > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            if EMG != 0.0 {
            } else {
            }
            let EMH = if EJP != 0.0 && (if ELV > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            if EMH != 0.0 {
            } else {
            }
            if PC != 0.0 {
            } else {
            }
            if EMC != 0.0 {
            } else {
            }
        {
            let psd = ELZ;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 0, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 0, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(0, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = EMA;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = Some(GT);
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(1, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = EMF;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 2, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 2, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(2, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        Ok(())
    }
}
