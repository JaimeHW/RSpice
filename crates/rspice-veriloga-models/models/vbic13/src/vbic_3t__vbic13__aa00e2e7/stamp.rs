#![allow(dead_code, non_snake_case, unused_imports, unused_mut, unused_parens, unused_variables)]

use super::state::Instance;
use rspice_veriloga_runtime::{GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, Lanes, rspice_eval_ddt, rspice_eval_idt, rspice_limexp, rspice_limited_exp, rspice_limited_exp_derivative};
impl Instance {
    pub fn stamp(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedStamper<'_>) {
        let parameters = &self.params.values;
        let parameter_given = &*self.param_given;
        let multiplicity = self.multiplicity;
        let temperature = ctx.temperature();
        let node_potentials = [ctx.node_voltage(self.nodes[0]), ctx.node_voltage(self.nodes[1]), ctx.node_voltage(self.nodes[2]), ctx.node_voltage(self.nodes[3]), ctx.node_voltage(self.nodes[4]), ctx.node_voltage(self.nodes[5]), ctx.node_voltage(self.nodes[6]), ctx.node_voltage(self.nodes[7]), ctx.node_voltage(self.nodes[8]), ctx.node_voltage(self.nodes[9]), ctx.node_voltage(self.nodes[10]), ctx.node_voltage(self.nodes[11])];
        let ddt_scale_value = self.ddt_coefficients.derivative_scale;
        let ddt_scale = move || ddt_scale_value;
        let ddt_state = self.stamp_state.as_mut();
        let ddt_active = self.ddt_coefficients.active;
        let ddt_coefficients = self.ddt_coefficients;
        let mut ddt = |operator: usize, value: f64| -> f64 {
            let _ = operator;
            let slot = match operator { 9699 => 0usize, 9701 => 1usize, 9703 => 2usize, 9705 => 3usize, 9707 => 4usize, 9709 => 5usize, 9711 => 6usize, 9713 => 7usize, 9715 => 8usize, 9717 => 9usize, _ => usize::MAX };
            rspice_eval_ddt(
                &mut ddt_state.ddt_current,
                &mut ddt_state.ddt_previous,
                &mut ddt_state.ddt_older,
                &mut ddt_state.ddt_initialized,
                &mut ddt_state.ddt_derivative_current,
                &mut ddt_state.ddt_derivative_previous,
                ddt_active,
                ddt_coefficients.derivative_scale,
                ddt_coefficients.previous_value_scale,
                ddt_coefficients.older_value_scale,
                ddt_coefficients.previous_derivative_scale,
                slot,
                value,
            )
        };
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
            let AH = parameters[80];
            let AM = 2.7315e2f64;
            let AS = parameters[14];
            let AW = parameters[15];
            let BC = 1.380662e-23f64;
            let BD = 1.602189e-19f64;
            let BG = parameters[26];
            let BH = parameters[90];
            let BJ = parameters[89];
            let BL = parameters[88];
            let BO = parameters[122];
            let BP = parameters[28];
            let BW = parameters[72];
            let BY = 5e-1f64;
            let BZ = 4e0f64;
            let CA = parameters[73];
            let CD = parameters[27];
            let CE = parameters[29];
            let CN = parameters[31];
            let CO = parameters[33];
            let CX = parameters[54];
            let CY = parameters[123];
            let CZ = parameters[56];
            let DB = parameters[114];
            let DH = parameters[58];
            let DI = parameters[124];
            let DJ = parameters[59];
            let DQ = parameters[60];
            let DR = parameters[61];
            let DU = parameters[115];
            let EB = parameters[62];
            let EC = parameters[63];
            let EL = parameters[64];
            let EP = parameters[65];
            let ET = parameters[67];
            let EV = parameters[69];
            let EX = node_potentials[3];
            let FL = parameters[126];
            let FN = if parameter_given[109] { 1.0 } else { 0.0 };
            let FO = parameters[16];
            let FP = parameters[109];
            let FR = parameters[107];
            let FT = if parameter_given[108] { 1.0 } else { 0.0 };
            let FU = parameters[17];
            let FV = parameters[108];
            let FY = if parameter_given[106] { 1.0 } else { 0.0 };
            let FZ = parameters[21];
            let GA = parameters[106];
            let GC = parameters[104];
            let GE = if parameter_given[105] { 1.0 } else { 0.0 };
            let GF = parameters[22];
            let GG = parameters[105];
            let GJ = parameters[23];
            let GK = parameters[103];
            let GN = if parameter_given[110] { 1.0 } else { 0.0 };
            let GO = parameters[25];
            let GP = parameters[110];
            let GS = parameters[101];
            let GT = parameters[132];
            let IM = parameters[129];
            let IQ = parameters[84];
            let IR = parameters[127];
            let IT = parameters[86];
            let IU = parameters[128];
            let IW = parameters[92];
            let IY = parameters[93];
            let JA = 2e0f64;
            let JD = parameters[37];
            let JN = 3e0f64;
            let KA = parameters[42];
            let KR = parameters[36];
            let KT = parameters[38];
            let KV = parameters[41];
            let KX = parameters[43];
            let LA = parameters[48];
            let LC = parameters[19];
            let LH = parameters[18];
            let LI = parameters[112];
            let LO = parameters[70];
            let LP = parameters[130];
            let LR = parameters[71];
            let LS = parameters[131];
            let LV = 1e-3f64;
            let LY = 1e3f64;
            let NM = node_potentials[7];
            let NN = node_potentials[8];
            let NP = node_potentials[6];
            let NR = node_potentials[5];
            let NT = node_potentials[4];
            let NW = node_potentials[9];
            let NY = node_potentials[1];
            let NZ = node_potentials[2];
            let OC = node_potentials[0];
            let OK = node_potentials[10];
            let OL = node_potentials[11];
            let ON = parameters[34];
            let OP = parameters[39];
            let PN = -5e-1f64;
            let QK = parameters[44];
            let QX = parameters[45];
            let RR = parameters[46];
            let TH = -5e-1f64;
            let UZ = 1e-4f64;
            let VB = 1e-8f64;
            let WN = parameters[32];
            let WX = 5.0005e-1f64;
            let WY = parameters[55];
            let XQ = parameters[57];
            let AFH = parameters[83];
            let AFJ = 2e-2f64;
            let AFL = 1.01e0f64;
            let AGE = parameters[85];
            let AGH = parameters[87];
            let AHA = parameters[97];
            let AHB = parameters[95];
            let AHD = parameters[94];
            let AHF = 1e-1f64;
            let AHM = parameters[96];
            let AJM = -5e-1f64;
            let ANA = -5e-1f64;
            let ANT = 1.44e0f64;
            let ANZ = parameters[76];
            let AOA = parameters[77];
            let AOC = parameters[78];
            let AOL = parameters[81];
            let AOM = parameters[47];
            let AOQ = parameters[35];
            let AOS = parameters[40];
            let AOU = parameters[102];
            let AOW = parameters[82];
            let AOY = 3.333333333333333e-1f64;
            let APP = parameters[1];
            let APQ = 0e0f64;
            let APR = 0e0f64;
            let APS = 0e0f64;
            let APT = 0e0f64;
            let APU = 0e0f64;
            let APV = 0e0f64;
            let APW = 0e0f64;
            let APX = 0e0f64;
            let APY = 0e0f64;
            let APZ = 0e0f64;
            let AQA = 0e0f64;
            let AQB = 0e0f64;
            let AQC = 0e0f64;
            let AQQ = 1e0f64;
            let AQR = 1e0f64;
            let AQS = 1e0f64;
            let AQT = 1e0f64;
            let AQU = 1e0f64;
            let AQV = 1e0f64;
            let AQW = 1e0f64;
            let AQX = 1e0f64;
            let AQY = 1e0f64;
            let AQZ = 1e0f64;
            let ARA = 1e0f64;
            let ARB = 1e0f64;
            let ARC = 1e0f64;
            let AUF = -1e0f64;
            let AWA = 2e0f64;
            let AWP = 0e0f64;
            let AYF = Lanes([0e0f64; 3]);
            let AZP = Lanes([0e0f64; 3]);
            let BAV = Lanes([0e0f64; 5]);
            let BDN = Lanes([0e0f64; 4]);
            let BEJ = Lanes([0e0f64; 4]);
            let BEV = Lanes([0e0f64; 3]);
            let BGI = Lanes([0e0f64; 5]);
            let BGS = Lanes([0e0f64; 4]);
            let BHE = Lanes([0e0f64; 2]);
            let BJP = Lanes([0e0f64; 3]);
            let BLV = ddt_scale();
            if B != 0.0 {
            } else {
            }
            if D != 0.0 {
            } else {
            }
            let AIG = if F != 0.0 {
                G
            } else {
                let H = ctx.simparam_or("gmin", 1e-12f64);
                H
            };
            let BM = if I != 0.0 {
                J
            } else {
                let K = ctx.simparam_or("pnjmaxi", C);
                K
            };
            let NJ;
            if L != 0.0 {
                NJ = C;
            } else {
                let NK;
                if M != 0.0 {
                    NK = N;
                } else {
                    let NL = if O != 0.0 {
                        P
                    } else {
                        C
                    };
                    NK = NL;
                }
                NJ = NK;
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
            let AI = if AH > A { 1.0 } else { 0.0 };
            let AK = if AI != 0.0 {
                let AJ = C / AH;
                AJ
            } else {
                A
            };
            let AL = if AI != 0.0 {
                A
            } else {
                C
            };
            let AN = AM + parameters[13];
            let AO = temperature + parameters[0];
            let AP = AO - AM;
            let AQ = if AP < parameters[8] { 1.0 } else { 0.0 };
            if AQ != 0.0 {
            } else {
            }
            let AR = if AP > parameters[9] { 1.0 } else { 0.0 };
            if AR != 0.0 {
            } else {
            }
            let AT = AS + C;
            let AU = if AP < AT { 1.0 } else { 0.0 };
            let AZ;
            if AU != 0.0 {
                let AV = AS + (((AP - AS) - C).exp());
                AZ = AV;
            } else {
                let AX = if AP > (AW - C) { 1.0 } else { 0.0 };
                let BA = if AX != 0.0 {
                    let AY = AW - (((AW - AP) - C).exp());
                    AY
                } else {
                    AP
                };
                AZ = BA;
            }
            let BB = AZ + AM;
            let BE = (BC * BB) / BD;
            let BF = BB / AN;
            let BI = if BH > A { 1.0 } else { 0.0 };
            let YF = if BI != 0.0 {
                let BK = BJ * BE;
                let BN = BK * (((((-BL) / BK).exp()) + (BM / BH)).ln());
                BN
            } else {
                A
            };
            let BQ = BO / BP;
            let BR = -parameters[113];
            let BS = C - BF;
            let BT = BE * BP;
            let BU = (BG * (BF.powf(BQ))) * (((BR * BS) / BT).exp());
            let BV = if BU > A { 1.0 } else { 0.0 };
            let TX;
            if BV != 0.0 {
                let BX = if (if BW > A { 1.0 } else { 0.0 }) != 0.0 && (if BM > BW { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let TY = if BX != 0.0 {
                    let CB = BT * ((C + ((((BY * BM) * ((BZ / BW).powf(CA))).powf((C / (C - CA)))) / BU)).ln());
                    CB
                } else {
                    let CC = BT * ((C + (BM / BU)).ln());
                    CC
                };
                TX = TY;
            } else {
                TX = A;
            }
            let CF = parameters[125] / CE;
            let CG = -parameters[121];
            let CH = BE * CE;
            let CI = (CD * (BF.powf(CF))) * (((CG * BS) / CH).exp());
            let CJ = if BV != 0.0 && (if CI > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let UK;
            if CJ != 0.0 {
                let CK = if S != 0.0 && (if BM > R { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let UL = if CK != 0.0 {
                    let CL = CH * ((C + ((((BY * BM) * ((BZ / R).powf(CA))).powf((C / (C - CA)))) / (BU * CI))).ln());
                    CL
                } else {
                    let CM = CH * ((C + (BM / (BU * CI))).ln());
                    CM
                };
                UK = UL;
            } else {
                UK = A;
            }
            let CP = BO / CO;
            let CQ = -parameters[120];
            let CR = BE * CO;
            let CS = (CN * (BF.powf(CP))) * (((CQ * BS) / CR).exp());
            let CT = if CS > A { 1.0 } else { 0.0 };
            let VZ;
            if CT != 0.0 {
                let CU = if W != 0.0 && (if BM > V { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let WA = if CU != 0.0 {
                    let CV = CR * ((C + (((BM * BM) * Y) / CS)).ln());
                    CV
                } else {
                    let CW = CR * ((C + (BM / CS)).ln());
                    CW
                };
                VZ = WA;
            } else {
                VZ = A;
            }
            let DA = CY / CZ;
            let DC = -DB;
            let DD = BE * CZ;
            let DE = (CX * (BF.powf(DA))) * (((DC * BS) / DD).exp());
            let DF = if DE > A { 1.0 } else { 0.0 };
            let XB = if DF != 0.0 {
                let DG = DD * ((C + (BM / DE)).ln());
                DG
            } else {
                A
            };
            let DK = DI / DJ;
            let DL = -parameters[117];
            let DM = BE * DJ;
            let DN = (DH * (BF.powf(DK))) * (((DL * BS) / DM).exp());
            let DO = if DN > A { 1.0 } else { 0.0 };
            let XJ = if DO != 0.0 {
                let DP = DM * ((C + (BM / DN)).ln());
                DP
            } else {
                A
            };
            let DS = CY / DR;
            let DT = BF.powf(DS);
            let DV = -DU;
            let DW = BE * DR;
            let DX = ((DV * BS) / DW).exp();
            let DY = (DQ * DT) * DX;
            let DZ = if DY > A { 1.0 } else { 0.0 };
            let ACK = if DZ != 0.0 {
                let EA = DW * ((C + (BM / DY)).ln());
                EA
            } else {
                A
            };
            let ED = DI / EC;
            let EE = BF.powf(ED);
            let EF = -parameters[118];
            let EG = BE * EC;
            let EH = ((EF * BS) / EG).exp();
            let EI = (EB * EE) * EH;
            let EJ = if EI > A { 1.0 } else { 0.0 };
            let ACS = if EJ != 0.0 {
                let EK = EG * ((C + (BM / EI)).ln());
                EK
            } else {
                A
            };
            let EM = (EL * DT) * DX;
            let EN = if EM > A { 1.0 } else { 0.0 };
            let ADF = if EN != 0.0 {
                let EO = DW * ((C + (BM / EM)).ln());
                EO
            } else {
                A
            };
            let EQ = (EP * EE) * EH;
            let ER = if EQ > A { 1.0 } else { 0.0 };
            let ADM = if ER != 0.0 {
                let ES = EG * ((C + (BM / EQ)).ln());
                ES
            } else {
                A
            };
            let EU = if ((parameters[66] * (BF.powf((CY / ET)))) * ((((-parameters[116]) * BS) / (BE * ET)).exp())) > A { 1.0 } else { 0.0 };
            if EU != 0.0 {
            } else {
            }
            let EW = if ((parameters[68] * (BF.powf((DI / EV)))) * ((((-parameters[119]) * BS) / (BE * EV)).exp())) > A { 1.0 } else { 0.0 };
            if EW != 0.0 {
            } else {
            }
            let EY = (AO + EX) - AM;
            let EZ = if EY < AT { 1.0 } else { 0.0 };
            let FF;
            let ARD;
            if EZ != 0.0 {
                let FA = ((EY - AS) - C).exp();
                let AUH = AQR * FA;
                let FB = AS + FA;
                FF = FB;
                ARD = AUH;
            } else {
                let FC = if EY > (AW - C) { 1.0 } else { 0.0 };
                let FG;
                let ARE;
                if FC != 0.0 {
                    let FD = ((AW - EY) - C).exp();
                    let FE = AW - FD;
                    let AUG = ((AQR * AUF) * FD) * AUF;
                    FG = FE;
                    ARE = AUG;
                } else {
                    FG = EY;
                    ARE = AQR;
                }
                FF = FG;
                ARD = ARE;
            }
            let FH = FF + AM;
            let FI = (BC * FH) / BD;
            let AUI = (ARD * BC) / BD;
            let FJ = FH / AN;
            let AUJ = ARD / AN;
            let FK = FH - AN;
            let FM = BW * (FJ.powf(FL));
            let AUK = (AUJ * (FL * (FJ.powf((FL - AQQ))))) * BW;
            let LU;
            let ARF;
            if FN != 0.0 {
                let FQ = FO * (FJ.powf(FP));
                let AUM = (AUJ * (FP * (FJ.powf((FP - AQQ))))) * FO;
                LU = FQ;
                ARF = AUM;
            } else {
                let FS = FO * (FJ.powf(FR));
                let AUL = (AUJ * (FR * (FJ.powf((FR - AQQ))))) * FO;
                LU = FS;
                ARF = AUL;
            }
            let MA;
            let ARG;
            if FT != 0.0 {
                let FW = FU * (FJ.powf(FV));
                let AUO = (AUJ * (FV * (FJ.powf((FV - AQQ))))) * FU;
                MA = FW;
                ARG = AUO;
            } else {
                let FX = FU * (FJ.powf(FR));
                let AUN = (AUJ * (FR * (FJ.powf((FR - AQQ))))) * FU;
                MA = FX;
                ARG = AUN;
            }
            let ME;
            let ARH;
            if FY != 0.0 {
                let GB = FZ * (FJ.powf(GA));
                let AUQ = (AUJ * (GA * (FJ.powf((GA - AQQ))))) * FZ;
                ME = GB;
                ARH = AUQ;
            } else {
                let GD = FZ * (FJ.powf(GC));
                let AUP = (AUJ * (GC * (FJ.powf((GC - AQQ))))) * FZ;
                ME = GD;
                ARH = AUP;
            }
            let MI;
            let ARI;
            if GE != 0.0 {
                let GH = GF * (FJ.powf(GG));
                let AUS = (AUJ * (GG * (FJ.powf((GG - AQQ))))) * GF;
                MI = GH;
                ARI = AUS;
            } else {
                let GI = GF * (FJ.powf(GC));
                let AUR = (AUJ * (GC * (FJ.powf((GC - AQQ))))) * GF;
                MI = GI;
                ARI = AUR;
            }
            let GL = GJ * (FJ.powf(GK));
            let AUT = (AUJ * (GK * (FJ.powf((GK - AQQ))))) * GJ;
            let GM = parameters[24] * (FJ.powf(parameters[111]));
            let MP;
            let ARJ;
            if GN != 0.0 {
                let GQ = GO * (FJ.powf(GP));
                let AUV = (AUJ * (GP * (FJ.powf((GP - AQQ))))) * GO;
                MP = GQ;
                ARJ = AUV;
            } else {
                let GR = GO * (FJ.powf(FR));
                let AUU = (AUJ * (FR * (FJ.powf((FR - AQQ))))) * GO;
                MP = GR;
                ARJ = AUU;
            }
            let GU = GS * (C + (FK * GT));
            let AUW = (ARD * GT) * GS;
            let GV = BG * (FJ.powf(BQ));
            let GW = C - FJ;
            let AUX = AUJ * AUF;
            let GX = BR * GW;
            let AUY = AUX * BR;
            let GY = FI * BP;
            let GZ = GX / GY;
            let HA = GZ.exp();
            let HB = GV * HA;
            let AUZ = (((AUJ * (BQ * (FJ.powf((BQ - AQQ))))) * BG) * HA) + ((((AUY - ((AUI * BP) * GZ)) / GY) * HA) * GV);
            let HC = CD * (FJ.powf(CF));
            let HD = FI * CE;
            let HE = (CG * GW) / HD;
            let HF = HE.exp();
            let HG = HC * HF;
            let AVA = (((AUJ * (CF * (FJ.powf((CF - AQQ))))) * CD) * HF) + (((((AUX * CG) - ((AUI * CE) * HE)) / HD) * HF) * HC);
            let HH = CN * (FJ.powf(CP));
            let HI = FI * CO;
            let AVB = AUI * CO;
            let HJ = (CQ * GW) / HI;
            let HK = HJ.exp();
            let HL = HH * HK;
            let AVC = (((AUJ * (CP * (FJ.powf((CP - AQQ))))) * CN) * HK) + (((((AUX * CQ) - (AVB * HJ)) / HI) * HK) * HH);
            let HM = CX * (FJ.powf(DA));
            let HN = FI * CZ;
            let AVD = AUI * CZ;
            let HO = (DC * GW) / HN;
            let HP = HO.exp();
            let HQ = HM * HP;
            let AVE = (((AUJ * (DA * (FJ.powf((DA - AQQ))))) * CX) * HP) + (((((AUX * DC) - (AVD * HO)) / HN) * HP) * HM);
            let HR = DH * (FJ.powf(DK));
            let HS = FI * DJ;
            let AVF = AUI * DJ;
            let HT = (DL * GW) / HS;
            let HU = HT.exp();
            let HV = HR * HU;
            let AVG = (((AUJ * (DK * (FJ.powf((DK - AQQ))))) * DH) * HU) + (((((AUX * DL) - (AVF * HT)) / HS) * HU) * HR);
            let HW = FJ.powf(DS);
            let AVH = AUJ * (DS * (FJ.powf((DS - AQQ))));
            let HX = DQ * HW;
            let HY = FI * DR;
            let AVI = AUI * DR;
            let HZ = (DV * GW) / HY;
            let IA = HZ.exp();
            let AVJ = (((AUX * DV) - (AVI * HZ)) / HY) * IA;
            let IB = HX * IA;
            let AVK = ((AVH * DQ) * IA) + (AVJ * HX);
            let IC = FJ.powf(ED);
            let AVL = AUJ * (ED * (FJ.powf((ED - AQQ))));
            let ID = EB * IC;
            let IE = FI * EC;
            let AVM = AUI * EC;
            let IF = (EF * GW) / IE;
            let IG = IF.exp();
            let AVN = (((AUX * EF) - (AVM * IF)) / IE) * IG;
            let IH = ID * IG;
            let AVO = ((AVL * EB) * IG) + (AVN * ID);
            let II = EL * HW;
            let IJ = II * IA;
            let AVP = ((AVH * EL) * IA) + (AVJ * II);
            let IK = EP * IC;
            let IL = IK * IG;
            let AVQ = ((AVL * EP) * IG) + (AVN * IK);
            let AVR = ARD * IM;
            let IN = C + (FK * IM);
            let IO = BP * IN;
            let AVS = AVR * BP;
            let IP = CE * IN;
            let AVT = AVR * CE;
            let IS = IQ * (C + (FK * IR));
            let AVU = (ARD * IR) * IQ;
            let IV = IT * (C + (FK * IU));
            let AVV = (ARD * IU) * IT;
            let IX = parameters[91] + (FK * IW);
            let IZ = BJ * (C + (FK * IY));
            let JB = FI / FJ;
            let JC = JA * JB;
            let AVW = ((AUI - (AUJ * JB)) / FJ) * JA;
            let JE = BY * JD;
            let JF = (JE * FJ) / FI;
            let JG = JF.exp();
            let JH = -5e-1f64 * JD;
            let JI = (JH * FJ) / FI;
            let JJ = JI.exp();
            let JK = JG - JJ;
            let JL = JK.ln();
            let JM = JC * JL;
            let JO = JN * FI;
            let JP = FJ.ln();
            let JQ = JO * JP;
            let AVX = ((AUI * JN) * JP) + ((AUJ * (AQQ / FJ)) * JO);
            let JR = FJ - C;
            let JS = ((JM * FJ) - JQ) - (DB * JR);
            let AVY = (((((AVW * JL) + (((((((AUJ * JE) - (AUI * JF)) / FI) * JG) - ((((AUJ * JH) - (AUI * JI)) / FI) * JJ)) * (AQQ / JK)) * JC)) * FJ) + (AUJ * JM)) - AVX) - (AUJ * DB);
            let JT = JA * FI;
            let AVZ = AUI * JA;
            let JU = (-JS) / FI;
            let JV = JU.exp();
            let JW = (C + (BZ * JV)).sqrt();
            let JX = BY * (C + JW);
            let JY = JX.ln();
            let JZ = JS + (JT * JY);
            let AWB = AVY + ((AVZ * JY) + (((((((((AVY * AUF) - (AUI * JU)) / FI) * JV) * BZ) * (AQQ / (AWA * JW))) * BY) * (AQQ / JX)) * JT));
            let KB = BY * KA;
            let KC = (KB * FJ) / FI;
            let KD = KC.exp();
            let KE = -5e-1f64 * KA;
            let KF = (KE * FJ) / FI;
            let KG = KF.exp();
            let KH = KD - KG;
            let KI = KH.ln();
            let KJ = JC * KI;
            let KK = ((KJ * FJ) - JQ) - (DU * JR);
            let AWC = (((((AVW * KI) + (((((((AUJ * KB) - (AUI * KC)) / FI) * KD) - ((((AUJ * KE) - (AUI * KF)) / FI) * KG)) * (AQQ / KH)) * JC)) * FJ) + (AUJ * KJ)) - AVX) - (AUJ * DU);
            let KL = (-KK) / FI;
            let KM = KL.exp();
            let KN = (C + (BZ * KM)).sqrt();
            let KO = BY * (C + KN);
            let KP = KO.ln();
            let KQ = KK + (JT * KP);
            let AWD = AWC + ((AVZ * KP) + (((((((((AWC * AUF) - (AUI * KL)) / FI) * KM) * BZ) * (AQQ / (AWA * KN))) * BY) * (AQQ / KO)) * JT));
            let KS = JD / JZ;
            let KU = KR * (KS.powf(KT));
            let AWE = ((((AWB * KS) * AUF) / JZ) * (KT * (KS.powf((KT - AQQ))))) * KR;
            let KW = KA / KQ;
            let KY = KW.powf(KX);
            let AWF = (((AWD * KW) * AUF) / KQ) * (KX * (KW.powf((KX - AQQ))));
            let KZ = KV * KY;
            let AWG = AWF * KV;
            let LB = LA * KY;
            let AWH = AWF * LA;
            let LD = LC * (FJ.powf(BO));
            let LE = GX / FI;
            let LF = LE.exp();
            let LG = LD * LF;
            let AWI = (((AUJ * (BO * (FJ.powf((BO - AQQ))))) * LC) * LF) + ((((AUY - (AUI * LE)) / FI) * LF) * LD);
            let LJ = LH * (FJ.powf(LI));
            let AWJ = (AUJ * (LI * (FJ.powf((LI - AQQ))))) * LH;
            let LK = -(BL * (C + (FK * IX)));
            let AWK = (((ARD * IX) + ((ARD * IW) * FK)) * BL) * AUF;
            let LL = IZ * FI;
            let AWL = (((ARD * IY) * BJ) * FI) + (AUI * IZ);
            let LM = LK / LL;
            let LN = LM.exp();
            let AWM = ((AWK - (AWL * LM)) / LL) * LN;
            let LQ = LO * (C + (FK * LP));
            let AWN = (ARD * LP) * LO;
            let LT = LR * (C + (FK * LS));
            let AWO = (ARD * LS) * LR;
            let LW = if LU > LV { 1.0 } else { 0.0 };
            let LZ;
            let ARK;
            if LW != 0.0 {
                let LX = C / LU;
                let AWQ = ((ARF * LX) * AUF) / LU;
                LZ = LX;
                ARK = AWQ;
            } else {
                LZ = LY;
                ARK = AWP;
            }
            let MB = if MA > LV { 1.0 } else { 0.0 };
            let MD;
            let ARL;
            if MB != 0.0 {
                let MC = C / MA;
                let AWR = ((ARG * MC) * AUF) / MA;
                MD = MC;
                ARL = AWR;
            } else {
                MD = LY;
                ARL = AWP;
            }
            let MF = if ME > LV { 1.0 } else { 0.0 };
            let MH;
            let ARM;
            if MF != 0.0 {
                let MG = C / ME;
                let AWS = ((ARH * MG) * AUF) / ME;
                MH = MG;
                ARM = AWS;
            } else {
                MH = LY;
                ARM = AWP;
            }
            let MJ = if MI > LV { 1.0 } else { 0.0 };
            let ML;
            let ARN;
            if MJ != 0.0 {
                let MK = C / MI;
                let AWT = ((ARI * MK) * AUF) / MI;
                ML = MK;
                ARN = AWT;
            } else {
                ML = LY;
                ARN = AWP;
            }
            let MM = if GL > LV { 1.0 } else { 0.0 };
            let MO;
            let ARO;
            if MM != 0.0 {
                let MN = C / GL;
                let AWU = ((AUT * MN) * AUF) / GL;
                MO = MN;
                ARO = AWU;
            } else {
                MO = LY;
                ARO = AWP;
            }
            let MQ = if MP > LV { 1.0 } else { 0.0 };
            let MS;
            let ARP;
            if MQ != 0.0 {
                let MR = C / MP;
                let AWV = ((ARJ * MR) * AUF) / MP;
                MS = MR;
                ARP = AWV;
            } else {
                MS = LY;
                ARP = AWP;
            }
            let MT = if GM > LV { 1.0 } else { 0.0 };
            if MT != 0.0 {
            } else {
            }
            let MU = if GU > LV { 1.0 } else { 0.0 };
            let MW;
            let ARQ;
            if MU != 0.0 {
                let MV = C / GU;
                let AWW = ((AUW * MV) * AUF) / GU;
                MW = MV;
                ARQ = AWW;
            } else {
                MW = LY;
                ARQ = AWP;
            }
            let MX = if LQ > A { 1.0 } else { 0.0 };
            let MZ;
            let ARR;
            if MX != 0.0 {
                let MY = C / LQ;
                let AWX = ((AWN * MY) * AUF) / LQ;
                MZ = MY;
                ARR = AWX;
            } else {
                MZ = A;
                ARR = AWP;
            }
            let NA = if LT > A { 1.0 } else { 0.0 };
            let NC;
            let ARS;
            if NA != 0.0 {
                let NB = C / LT;
                let AWY = ((AWO * NB) * AUF) / LT;
                NC = NB;
                ARS = AWY;
            } else {
                NC = A;
                ARS = AWP;
            }
            let ND = if FM > A { 1.0 } else { 0.0 };
            let NF;
            let ART;
            if ND != 0.0 {
                let NE = C / FM;
                let AWZ = ((AUK * NE) * AUF) / FM;
                NF = NE;
                ART = AWZ;
            } else {
                NF = A;
                ART = AWP;
            }
            let NG = if LJ > A { 1.0 } else { 0.0 };
            let NI;
            let ARU;
            if NG != 0.0 {
                let NH = C / LJ;
                let AXA = ((AWJ * NH) * AUF) / LJ;
                NI = NH;
                ARU = AXA;
            } else {
                NI = A;
                ARU = AWP;
            }
            let NO = NJ * (NM - NN);
            let AXB = (Lanes([AQS, 0.0]) - Lanes([0.0, AQT])) * NJ;
            let NQ = NJ * (NP - NN);
            let AXC = (Lanes([AQU, 0.0]) - Lanes([0.0, AQT])) * NJ;
            let NS = NJ * (NM - NR);
            let AXD = (Lanes([0.0, AQS]) - Lanes([AQV, 0.0])) * NJ;
            let NU = NJ * (NM - NT);
            let AXE = (Lanes([0.0, AQS]) - Lanes([AQW, 0.0])) * NJ;
            let NV = NJ * (NP - NT);
            let AXF = (Lanes([0.0, AQU]) - Lanes([AQW, 0.0])) * NJ;
            let NX = NJ * (NP - NW);
            let AXG = (Lanes([AQU, 0.0]) - Lanes([0.0, AQX])) * NJ;
            let OA = NY - NZ;
            let AXH = Lanes([AQY, 0.0]) - Lanes([0.0, AQZ]);
            let OB = NJ * (NR - NN);
            let AXI = (Lanes([AQV, 0.0]) - Lanes([0.0, AQT])) * NJ;
            let OD = NY - OC;
            let AXJ = Lanes([0.0, AQY]) - Lanes([ARA, 0.0]);
            let OE = OC - NT;
            let AXK = Lanes([ARA, 0.0]) - Lanes([0.0, AQW]);
            let OF = NJ * (NT - NR);
            let AXL = (Lanes([AQW, 0.0]) - Lanes([0.0, AQV])) * NJ;
            let OG = NY - NP;
            let AXM = Lanes([AQY, 0.0]) - Lanes([0.0, AQU]);
            let OH = NP - NM;
            let AXN = Lanes([AQU, 0.0]) - Lanes([0.0, AQS]);
            let OI = NZ - NN;
            let AXO = Lanes([AQZ, 0.0]) - Lanes([0.0, AQT]);
            let OJ = NW - NT;
            let AXP = Lanes([0.0, AQX]) - Lanes([AQW, 0.0]);
            let OM = -JZ;
            let AXQ = AWB * AUF;
            let OO = OM * ON;
            let AXR = AXQ * ON;
            let OQ = if OP <= A { 1.0 } else { 0.0 };
            let UW;
            let ARV;
            if OQ != 0.0 {
                let OR = NO + OO;
                let AYC = Lanes([0.0, AXB[0], AXB[1]]);
                let AYD = AYC + Lanes([AXR, 0.0, 0.0]);
                let OS = if OR > A { 1.0 } else { 0.0 };
                let PI;
                let PJ;
                let ARW;
                let ARX;
                if OS != 0.0 {
                    let OT = C - ON;
                    let OU = OT.powf((-KT));
                    let OV = C - (OU * OT);
                    let OW = C - KT;
                    let OX = (JZ * OV) / OW;
                    let OY = BY * KT;
                    let OZ = JZ * OT;
                    let PA = (OY * OR) / OZ;
                    let PB = C + PA;
                    let PC = (OR * PB) * OU;
                    let AYG = ((AYD * PB) + ((((AYD * OY) - Lanes([((AWB * OT) * PA), 0.0, 0.0])) / OZ) * OR)) * OU;
                    let AYH = Lanes([((AWB * OV) / OW), 0.0, 0.0]);
                    PI = OX;
                    PJ = PC;
                    ARW = AYH;
                    ARX = AYG;
                } else {
                    let PD = NO / JZ;
                    let PE = C - PD;
                    let PF = C - KT;
                    let PG = C - (PE.powf(PF));
                    let PH = (JZ * PG) / PF;
                    let AYE = (Lanes([(AWB * PG), 0.0, 0.0]) + ((((((AYC - Lanes([(AWB * PD), 0.0, 0.0])) / JZ) * AUF) * (PF * (PE.powf((PF - AQQ))))) * AUF) * JZ)) / PF;
                    PI = PH;
                    PJ = A;
                    ARW = AYE;
                    ARX = AYF;
                }
                let PK = PI + PJ;
                let AYI = ARW + ARX;
                UW = PK;
                ARV = AYI;
            } else {
                let AXS = AXR * OO;
                let PL = (BZ * OP) * OP;
                let PM = ((OO * OO) + PL).sqrt();
                let PO = PN * (OO + PM);
                let AXT = (AXR + ((AXS + AXS) * (AQQ / (AWA * PM)))) * PN;
                let PP = PO / JZ;
                let PQ = C - PP;
                let PR = C - KT;
                let PS = PQ.powf(PR);
                let AXU = PR - AQQ;
                let PT = NO + OO;
                let AXV = Lanes([0.0, AXB[0], AXB[1]]);
                let AXW = Lanes([AXR, 0.0, 0.0]);
                let AXX = AXV + AXW;
                let AXY = AXX * PT;
                let PU = ((PT * PT) + PL).sqrt();
                let PV = (BY * (PT - PU)) - OO;
                let AXZ = ((AXX - ((AXY + AXY) * (AQQ / (AWA * PU)))) * BY) - AXW;
                let PW = PV / JZ;
                let PX = C - PW;
                let PY = PX.powf(PR);
                let PZ = C - ON;
                let QA = PZ.powf((-KT));
                let QB = (NO - PV) + PO;
                let AYA = (AXV - AXZ) + Lanes([AXT, 0.0, 0.0]);
                let QC = QA * QB;
                let QD = BY * KT;
                let QE = JZ * PZ;
                let QF = (QD * QB) / QE;
                let QG = C + QF;
                let QH = (((OM * PY) / PR) + (QC * QG)) - ((OM * PS) / PR);
                let AYB = (((Lanes([(AXQ * PY), 0.0, 0.0]) + (((((AXZ - Lanes([(AWB * PW), 0.0, 0.0])) / JZ) * AUF) * (PR * (PX.powf(AXU)))) * OM)) / PR) + (((AYA * QA) * QG) + ((((AYA * QD) - Lanes([((AWB * PZ) * QF), 0.0, 0.0])) / QE) * QC))) - Lanes([(((AXQ * PS) + (((((AXT - (AWB * PP)) / JZ) * AUF) * (PR * (PQ.powf(AXU)))) * OM)) / PR), 0.0, 0.0]);
                UW = QH;
                ARV = AYB;
            }
            let QI = -KQ;
            let AYJ = AWD * AUF;
            let QJ = QI * ON;
            let AYK = AYJ * ON;
            let QL = if QK <= A { 1.0 } else { 0.0 };
            let UX;
            let ARY;
            if QL != 0.0 {
                let QM = NS + QJ;
                let AZK = Lanes([0.0, AXD[0], AXD[1]]);
                let AZL = AZK + Lanes([AYK, 0.0, 0.0]);
                let QN = if QM > A { 1.0 } else { 0.0 };
                let RN;
                let RP;
                let ARZ;
                let ASA;
                if QN != 0.0 {
                    let QO = C - ON;
                    let QP = QO.powf((-1e0f64 - KX));
                    let QQ = C - ((QP * QO) * QO);
                    let QR = C - KX;
                    let QS = (KQ * QQ) / QR;
                    let QT = BY * KX;
                    let QU = (QT * QM) / KQ;
                    let QV = QO + QU;
                    let QW = (QM * QV) * QP;
                    let AZQ = ((AZL * QV) + ((((AZL * QT) - Lanes([(AWD * QU), 0.0, 0.0])) / KQ) * QM)) * QP;
                    let AZR = Lanes([((AWD * QQ) / QR), 0.0, 0.0]);
                    RN = QS;
                    RP = QW;
                    ARZ = AZR;
                    ASA = AZQ;
                } else {
                    let QY = if (if QX > A { 1.0 } else { 0.0 }) != 0.0 && (if NS < (-QX) { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let RO;
                    let ASB;
                    if QY != 0.0 {
                        let QZ = QX / KQ;
                        let RA = C + QZ;
                        let RB = C - KX;
                        let RC = RA.powf(RB);
                        let AZN = AXD * RB;
                        let RD = KQ + QX;
                        let RE = (RB * (NS + QX)) / RD;
                        let RF = C - RE;
                        let RG = C - (RC * RF);
                        let RH = (KQ * RG) / RB;
                        let AZO = (Lanes([(AWD * RG), 0.0, 0.0]) + (((Lanes([(((((AWD * QZ) * AUF) / KQ) * (RB * (RA.powf((RB - AQQ))))) * RF), 0.0, 0.0]) + ((((Lanes([0.0, AZN[0], AZN[1]]) - Lanes([(AWD * RE), 0.0, 0.0])) / RD) * AUF) * RC)) * AUF) * KQ)) / RB;
                        RO = RH;
                        ASB = AZO;
                    } else {
                        let RI = NS / KQ;
                        let RJ = C - RI;
                        let RK = C - KX;
                        let RL = C - (RJ.powf(RK));
                        let RM = (KQ * RL) / RK;
                        let AZM = (Lanes([(AWD * RL), 0.0, 0.0]) + ((((((AZK - Lanes([(AWD * RI), 0.0, 0.0])) / KQ) * AUF) * (RK * (RJ.powf((RK - AQQ))))) * AUF) * KQ)) / RK;
                        RO = RM;
                        ASB = AZM;
                    }
                    RN = RO;
                    RP = A;
                    ARZ = ASB;
                    ASA = AZP;
                }
                let RQ = RN + RP;
                let AZS = ARZ + ASA;
                UX = RQ;
                ARY = AZS;
            } else {
                let RS = if (if QX > A { 1.0 } else { 0.0 }) != 0.0 && (if RR > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let UY;
                let ASC;
                if RS != 0.0 {
                    let RT = QX - QJ;
                    let AYU = AYK * AUF;
                    let RU = (QX + QJ) / RT;
                    let AYV = (AYK - (AYU * RU)) / RT;
                    let RV = RU - C;
                    let AYW = AYV * RV;
                    let RW = (BZ * QK) * QK;
                    let RX = ((RV * RV) + RW).sqrt();
                    let RY = RU + C;
                    let AYX = AYV * RY;
                    let RZ = (BZ * RR) * RR;
                    let SA = ((RY * RY) + RZ).sqrt();
                    let SB = RX + SA;
                    let SC = (JA * RU) / SB;
                    let SD = BY * (((SC * RT) - QX) - QJ);
                    let AYY = ((((((AYV * JA) - ((((AYW + AYW) * (AQQ / (AWA * RX))) + ((AYX + AYX) * (AQQ / (AWA * SA)))) * SC)) / SB) * RT) + (AYU * SC)) - AYK) * BY;
                    let SE = SD / KQ;
                    let SF = C - SE;
                    let SG = C - KX;
                    let AYZ = SG - AQQ;
                    let SH = C - (SF.powf(SG));
                    let AZA = AXD * JA;
                    let AZB = Lanes([AYK, 0.0, 0.0]);
                    let SI = (((JA * NS) + QX) + QJ) / RT;
                    let AZC = ((Lanes([0.0, AZA[0], AZA[1]]) + AZB) - Lanes([(AYU * SI), 0.0, 0.0])) / RT;
                    let SJ = SI - C;
                    let AZD = AZC * SJ;
                    let SK = ((SJ * SJ) + RW).sqrt();
                    let SL = SI + C;
                    let AZE = AZC * SL;
                    let SM = ((SL * SL) + RZ).sqrt();
                    let SN = SK + SM;
                    let SO = (JA * SI) / SN;
                    let AZF = ((AZC * JA) - ((((AZD + AZD) * (AQQ / (AWA * SK))) + ((AZE + AZE) * (AQQ / (AWA * SM)))) * SO)) / SN;
                    let SP = BY * (((SO * RT) - QX) - QJ);
                    let AZG = (((AZF * RT) + Lanes([(AYU * SO), 0.0, 0.0])) - AZB) * BY;
                    let SQ = SP / KQ;
                    let SR = C - SQ;
                    let SS = C - (SR.powf(SG));
                    let ST = BY * (SO + C);
                    let AZH = AZF * BY;
                    let SU = QX / KQ;
                    let SV = C + SU;
                    let SW = -KX;
                    let SX = SV.powf(SW);
                    let AZI = SW - AQQ;
                    let SY = QJ / KQ;
                    let SZ = C + SY;
                    let TA = SZ.powf(SW);
                    let TB = C - ST;
                    let TC = (TB * SX) + (ST * TA);
                    let TD = (NS - SP) + SD;
                    let TE = ((TD * TC) + ((KQ * SS) / SG)) - ((KQ * SH) / SG);
                    let AZJ = (((((Lanes([0.0, AXD[0], AXD[1]]) - AZG) + Lanes([AYY, 0.0, 0.0])) * TC) + (((((AZH * AUF) * SX) + Lanes([(((((AWD * SU) * AUF) / KQ) * (SW * (SV.powf(AZI)))) * TB), 0.0, 0.0])) + ((AZH * TA) + Lanes([((((AYK - (AWD * SY)) / KQ) * (SW * (SZ.powf(AZI)))) * ST), 0.0, 0.0]))) * TD)) + ((Lanes([(AWD * SS), 0.0, 0.0]) + ((((((AZG - Lanes([(AWD * SQ), 0.0, 0.0])) / KQ) * AUF) * (SG * (SR.powf(AYZ)))) * AUF) * KQ)) / SG)) - Lanes([(((AWD * SH) + ((((((AYY - (AWD * SE)) / KQ) * AUF) * (SG * (SF.powf(AYZ)))) * AUF) * KQ)) / SG), 0.0, 0.0]);
                    UY = TE;
                    ASC = AZJ;
                } else {
                    let AYL = AYK * QJ;
                    let TF = (BZ * QK) * QK;
                    let TG = ((QJ * QJ) + TF).sqrt();
                    let TI = TH * (QJ + TG);
                    let AYM = (AYK + ((AYL + AYL) * (AQQ / (AWA * TG)))) * TH;
                    let TJ = TI / KQ;
                    let TK = C - TJ;
                    let TL = C - KX;
                    let TM = TK.powf(TL);
                    let AYN = TL - AQQ;
                    let TN = NS + QJ;
                    let AYO = Lanes([0.0, AXD[0], AXD[1]]);
                    let AYP = Lanes([AYK, 0.0, 0.0]);
                    let AYQ = AYO + AYP;
                    let AYR = AYQ * TN;
                    let TO = ((TN * TN) + TF).sqrt();
                    let TP = (BY * (TN - TO)) - QJ;
                    let AYS = ((AYQ - ((AYR + AYR) * (AQQ / (AWA * TO)))) * BY) - AYP;
                    let TQ = TP / KQ;
                    let TR = C - TQ;
                    let TS = TR.powf(TL);
                    let TT = (C - ON).powf((-KX));
                    let TU = (((QI * TS) / TL) + (TT * ((NS - TP) + TI))) - ((QI * TM) / TL);
                    let AYT = (((Lanes([(AYJ * TS), 0.0, 0.0]) + (((((AYS - Lanes([(AWD * TQ), 0.0, 0.0])) / KQ) * AUF) * (TL * (TR.powf(AYN)))) * QI)) / TL) + (((AYO - AYS) + Lanes([AYM, 0.0, 0.0])) * TT)) - Lanes([(((AYJ * TM) + (((((AYM - (AWD * TJ)) / KQ) * AUF) * (TL * (TK.powf(AYN)))) * QI)) / TL), 0.0, 0.0]);
                    UY = TU;
                    ASC = AYT;
                }
                UX = UY;
                ARY = ASC;
            }
            let TV = IO * FI;
            let TW = C / TV;
            let AZT = ((((AVS * FI) + (AUI * IO)) * TW) * AUF) / TV;
            let TZ = if NO < TX { 1.0 } else { 0.0 };
            let UF;
            let ASD;
            if TZ != 0.0 {
                let AZW = AXB * TW;
                let UA = (NO * TW).exp();
                let AZX = (Lanes([0.0, AZW[0], AZW[1]]) + Lanes([(AZT * NO), 0.0, 0.0])) * UA;
                UF = UA;
                ASD = AZX;
            } else {
                let UB = (TX * TW).exp();
                let UC = NO - TX;
                let AZU = AXB * TW;
                let UD = C + (UC * TW);
                let UE = UB * UD;
                let AZV = Lanes([(((AZT * TX) * UB) * UD), 0.0, 0.0]) + ((Lanes([0.0, AZU[0], AZU[1]]) + Lanes([(AZT * UC), 0.0, 0.0])) * UB);
                UF = UE;
                ASD = AZV;
            }
            let UG = UF - C;
            let UH = HB * UG;
            let AZY = Lanes([(AUZ * UG), 0.0, 0.0]) + (ASD * HB);
            let UI = IP * FI;
            let UJ = C / UI;
            let AZZ = ((((AVT * FI) + (AUI * IP)) * UJ) * AUF) / UI;
            let UM = if NS < UK { 1.0 } else { 0.0 };
            let UT;
            let ASE;
            if UM != 0.0 {
                let BAC = AXD * UJ;
                let UN = (NS * UJ).exp();
                let BAD = (Lanes([0.0, BAC[0], BAC[1]]) + Lanes([(AZZ * NS), 0.0, 0.0])) * UN;
                UT = UN;
                ASE = BAD;
            } else {
                let UO = (UK * UJ).exp();
                let UP = NS - UK;
                let BAA = AXD * UJ;
                let UQ = C + (UP * UJ);
                let UR = UO * UQ;
                let BAB = Lanes([(((AZZ * UK) * UO) * UQ), 0.0, 0.0]) + ((Lanes([0.0, BAA[0], BAA[1]]) + Lanes([(AZZ * UP), 0.0, 0.0])) * UO);
                UT = UR;
                ASE = BAB;
            }
            let US = HB * HG;
            let UU = UT - C;
            let UV = US * UU;
            let BAE = Lanes([(((AUZ * HG) + (AVA * HB)) * UU), 0.0, 0.0]) + (ASE * US);
            let BAF = (ARV * NC) + Lanes([(ARS * UW), 0.0, 0.0]);
            let BAG = (ARY * MZ) + Lanes([(ARR * UX), 0.0, 0.0]);
            let BAH = Lanes([BAF[0], 0.0, BAF[1], BAF[2]]) + Lanes([BAG[0], BAG[1], BAG[2], 0.0]);
            let VA = ((C + (UW * NC)) + (UX * MZ)) - UZ;
            let BAI = BAH * VA;
            let VC = ((VA * VA) + VB).sqrt();
            let BAJ = (((BAI + BAI) * (AQQ / (AWA * VC))) + BAH) * BY;
            let VD = (BY * (VC + VA)) + UZ;
            let BAK = (AZY * NF) + Lanes([(ART * UH), 0.0, 0.0]);
            let BAL = BAE * U;
            let VE = (UH * NF) + (UV * U);
            let BAM = Lanes([BAK[0], 0.0, BAK[1], BAK[2]]) + Lanes([BAL[0], BAL[1], BAL[2], 0.0]);
            let VF = if parameters[30] < BY { 1.0 } else { 0.0 };
            let VS;
            let ASF;
            if VF != 0.0 {
                let VG = C / CA;
                let VH = (VD.powf(VG)) + (BZ * VE);
                let BAQ = (BAJ * (VG * (VD.powf((VG - AQQ))))) + (BAM * BZ);
                let VI = if VH > VB { 1.0 } else { 0.0 };
                let VT;
                let ASG;
                if VI != 0.0 {
                    let VJ = BY * (VD + (VH.powf(CA)));
                    let BAS = (BAJ + (BAQ * (CA * (VH.powf((CA - AQQ)))))) * BY;
                    VT = VJ;
                    ASG = BAS;
                } else {
                    let VK = BY * (VD + (VB.powf(CA)));
                    let BAR = BAJ * BY;
                    VT = VK;
                    ASG = BAR;
                }
                VS = VT;
                ASF = ASG;
            } else {
                let BAN = BAM * BZ;
                let VL = C + (BZ * VE);
                let VM = if VL > VB { 1.0 } else { 0.0 };
                let VU;
                let ASH;
                if VM != 0.0 {
                    let VN = BY * VD;
                    let VO = C + (VL.powf(CA));
                    let VP = VN * VO;
                    let BAP = ((BAJ * BY) * VO) + ((BAN * (CA * (VL.powf((CA - AQQ))))) * VN);
                    VU = VP;
                    ASH = BAP;
                } else {
                    let VQ = C + (VB.powf(CA));
                    let VR = (BY * VD) * VQ;
                    let BAO = (BAJ * BY) * VQ;
                    VU = VR;
                    ASH = BAO;
                }
                VS = VU;
                ASF = ASH;
            }
            let VV = UV / VS;
            let BAT = (Lanes([BAE[0], BAE[1], BAE[2], 0.0]) - (ASF * VV)) / VS;
            let VW = UH / VS;
            let BAU = (Lanes([AZY[0], 0.0, AZY[1], AZY[2]]) - (ASF * VW)) / VS;
            let VX = if CN > A { 1.0 } else { 0.0 };
            let AFD;
            let AOP;
            let ASI;
            let ASJ;
            if VX != 0.0 {
                let VY = C / HI;
                let BAW = ((AVB * VY) * AUF) / HI;
                let WB = if NX < VZ { 1.0 } else { 0.0 };
                let WO;
                let ASK;
                if WB != 0.0 {
                    let BAZ = AXG * VY;
                    let WC = (NX * VY).exp();
                    let BBA = (Lanes([0.0, BAZ[0], BAZ[1]]) + Lanes([(BAW * NX), 0.0, 0.0])) * WC;
                    WO = WC;
                    ASK = BBA;
                } else {
                    let WD = (VZ * VY).exp();
                    let WE = NX - VZ;
                    let BAX = AXG * VY;
                    let WF = C + (WE * VY);
                    let WG = WD * WF;
                    let BAY = Lanes([(((BAW * VZ) * WD) * WF), 0.0, 0.0]) + ((Lanes([0.0, BAX[0], BAX[1]]) + Lanes([(BAW * WE), 0.0, 0.0])) * WD);
                    WO = WG;
                    ASK = BAY;
                }
                let WH = if NS < VZ { 1.0 } else { 0.0 };
                let WQ;
                let ASL;
                if WH != 0.0 {
                    let BBD = AXD * VY;
                    let WI = (NS * VY).exp();
                    let BBE = (Lanes([0.0, BBD[0], BBD[1]]) + Lanes([(BAW * NS), 0.0, 0.0])) * WI;
                    WQ = WI;
                    ASL = BBE;
                } else {
                    let WJ = (VZ * VY).exp();
                    let WK = NS - VZ;
                    let BBB = AXD * VY;
                    let WL = C + (WK * VY);
                    let WM = WJ * WL;
                    let BBC = Lanes([(((BAW * VZ) * WJ) * WL), 0.0, 0.0]) + ((Lanes([0.0, BBB[0], BBB[1]]) + Lanes([(BAW * WK), 0.0, 0.0])) * WJ);
                    WQ = WM;
                    ASL = BBC;
                }
                let BBF = ASK * WN;
                let WP = C - WN;
                let BBG = ASL * WP;
                let WR = ((WN * WO) + (WP * WQ)) - C;
                let WS = HL * WR;
                let BBH = Lanes([(AVC * WR), 0.0, 0.0, 0.0, 0.0]) + ((Lanes([BBF[0], 0.0, BBF[1], 0.0, BBF[2]]) + Lanes([BBG[0], BBG[1], 0.0, BBG[2], 0.0])) * HL);
                let BBI = (BBH * Y) * BZ;
                let WT = C + (BZ * (WS * Y));
                let WU = if WT > VB { 1.0 } else { 0.0 };
                let AFE;
                let ASM;
                if WU != 0.0 {
                    let WV = WT.sqrt();
                    let WW = BY * (C + WV);
                    let BBJ = (BBI * (AQQ / (AWA * WV))) * BY;
                    AFE = WW;
                    ASM = BBJ;
                } else {
                    AFE = WX;
                    ASM = BAV;
                }
                AFD = AFE;
                AOP = WS;
                ASI = ASM;
                ASJ = BBH;
            } else {
                AFD = C;
                AOP = A;
                ASI = BAV;
                ASJ = BAV;
            }
            let WZ = if WY == C { 1.0 } else { 0.0 };
            let AHR;
            let AHW;
            let ASN;
            let ASO;
            if WZ != 0.0 {
                let XA = C / HN;
                let BDO = ((AVD * XA) * AUF) / HN;
                let XC = if NO < XB { 1.0 } else { 0.0 };
                let XU;
                let ASP;
                if XC != 0.0 {
                    let BDR = AXB * XA;
                    let XD = (NO * XA).exp();
                    let BDS = (Lanes([0.0, BDR[0], BDR[1]]) + Lanes([(BDO * NO), 0.0, 0.0])) * XD;
                    XU = XD;
                    ASP = BDS;
                } else {
                    let XE = (XB * XA).exp();
                    let XF = NO - XB;
                    let BDP = AXB * XA;
                    let XG = C + (XF * XA);
                    let XH = XE * XG;
                    let BDQ = Lanes([(((BDO * XB) * XE) * XG), 0.0, 0.0]) + ((Lanes([0.0, BDP[0], BDP[1]]) + Lanes([(BDO * XF), 0.0, 0.0])) * XE);
                    XU = XH;
                    ASP = BDQ;
                }
                let XI = C / HS;
                let BDT = ((AVF * XI) * AUF) / HS;
                let XK = if NO < XJ { 1.0 } else { 0.0 };
                let XW;
                let ASQ;
                if XK != 0.0 {
                    let BDW = AXB * XI;
                    let XL = (NO * XI).exp();
                    let BDX = (Lanes([0.0, BDW[0], BDW[1]]) + Lanes([(BDT * NO), 0.0, 0.0])) * XL;
                    XW = XL;
                    ASQ = BDX;
                } else {
                    let XM = (XJ * XI).exp();
                    let XN = NO - XJ;
                    let BDU = AXB * XI;
                    let XO = C + (XN * XI);
                    let XP = XM * XO;
                    let BDV = Lanes([(((BDT * XJ) * XM) * XO), 0.0, 0.0]) + ((Lanes([0.0, BDU[0], BDU[1]]) + Lanes([(BDT * XN), 0.0, 0.0])) * XM);
                    XW = XP;
                    ASQ = BDV;
                }
                let XR = if XQ > A { 1.0 } else { 0.0 };
                let YM;
                let ASR;
                if XR != 0.0 {
                    let XS = C + (XQ * (VD - C));
                    let XT = HQ * XS;
                    let XV = XU - C;
                    let BEA = ASP * XT;
                    let XX = XW - C;
                    let BEB = Lanes([(AVG * XX), 0.0, 0.0]) + (ASQ * HV);
                    let XY = (XT * XV) + (HV * XX);
                    let BEC = (((Lanes([(AVE * XS), 0.0, 0.0, 0.0]) + ((BAJ * XQ) * HQ)) * XV) + Lanes([BEA[0], 0.0, BEA[1], BEA[2]])) + Lanes([BEB[0], 0.0, BEB[1], BEB[2]]);
                    YM = XY;
                    ASR = BEC;
                } else {
                    let XZ = XU - C;
                    let YA = XW - C;
                    let YB = (HQ * XZ) + (HV * YA);
                    let BDY = (Lanes([(AVE * XZ), 0.0, 0.0]) + (ASP * HQ)) + (Lanes([(AVG * YA), 0.0, 0.0]) + (ASQ * HV));
                    let BDZ = Lanes([BDY[0], 0.0, BDY[1], BDY[2]]);
                    YM = YB;
                    ASR = BDZ;
                }
                let YC = if BL > A { 1.0 } else { 0.0 };
                let AHS;
                let ASS;
                if YC != 0.0 {
                    let YD = LK - NO;
                    let BED = Lanes([AWK, 0.0, 0.0]) - Lanes([0.0, AXB[0], AXB[1]]);
                    let YE = C / LL;
                    let BEE = ((AWL * YE) * AUF) / LL;
                    let YG = if YD < YF { 1.0 } else { 0.0 };
                    let YN;
                    let AST;
                    if YG != 0.0 {
                        let YH = (YD * YE).exp();
                        let BEG = ((BED * YE) + Lanes([(BEE * YD), 0.0, 0.0])) * YH;
                        YN = YH;
                        AST = BEG;
                    } else {
                        let YI = (YF * YE).exp();
                        let YJ = YD - YF;
                        let YK = C + (YJ * YE);
                        let YL = YI * YK;
                        let BEF = Lanes([(((BEE * YF) * YI) * YK), 0.0, 0.0]) + (((BED * YE) + Lanes([(BEE * YJ), 0.0, 0.0])) * YI);
                        YN = YL;
                        AST = BEF;
                    }
                    let BEH = (AST - Lanes([AWM, 0.0, 0.0])) * BH;
                    let YO = YM - (BH * (YN - LN));
                    let BEI = ASR - Lanes([BEH[0], 0.0, BEH[1], BEH[2]]);
                    AHS = YO;
                    ASS = BEI;
                } else {
                    AHS = YM;
                    ASS = ASR;
                }
                AHR = AHS;
                AHW = A;
                ASN = ASS;
                ASO = BEJ;
            } else {
                let YP = if WY == A { 1.0 } else { 0.0 };
                let AHT;
                let AHX;
                let ASU;
                let ASV;
                if YP != 0.0 {
                    let YQ = C / HN;
                    let BCV = ((AVD * YQ) * AUF) / HN;
                    let YR = if NQ < XB { 1.0 } else { 0.0 };
                    let ZE;
                    let ASW;
                    if YR != 0.0 {
                        let BCY = AXC * YQ;
                        let YS = (NQ * YQ).exp();
                        let BCZ = (Lanes([0.0, BCY[0], BCY[1]]) + Lanes([(BCV * NQ), 0.0, 0.0])) * YS;
                        ZE = YS;
                        ASW = BCZ;
                    } else {
                        let YT = (XB * YQ).exp();
                        let YU = NQ - XB;
                        let BCW = AXC * YQ;
                        let YV = C + (YU * YQ);
                        let YW = YT * YV;
                        let BCX = Lanes([(((BCV * XB) * YT) * YV), 0.0, 0.0]) + ((Lanes([0.0, BCW[0], BCW[1]]) + Lanes([(BCV * YU), 0.0, 0.0])) * YT);
                        ZE = YW;
                        ASW = BCX;
                    }
                    let YX = C / HS;
                    let BDA = ((AVF * YX) * AUF) / HS;
                    let YY = if NQ < XJ { 1.0 } else { 0.0 };
                    let ZG;
                    let ASX;
                    if YY != 0.0 {
                        let BDD = AXC * YX;
                        let YZ = (NQ * YX).exp();
                        let BDE = (Lanes([0.0, BDD[0], BDD[1]]) + Lanes([(BDA * NQ), 0.0, 0.0])) * YZ;
                        ZG = YZ;
                        ASX = BDE;
                    } else {
                        let ZA = (XJ * YX).exp();
                        let ZB = NQ - XJ;
                        let BDB = AXC * YX;
                        let ZC = C + (ZB * YX);
                        let ZD = ZA * ZC;
                        let BDC = Lanes([(((BDA * XJ) * ZA) * ZC), 0.0, 0.0]) + ((Lanes([0.0, BDB[0], BDB[1]]) + Lanes([(BDA * ZB), 0.0, 0.0])) * ZA);
                        ZG = ZD;
                        ASX = BDC;
                    }
                    let ZF = ZE - C;
                    let ZH = ZG - C;
                    let ZI = (HQ * ZF) + (HV * ZH);
                    let BDF = (Lanes([(AVE * ZF), 0.0, 0.0]) + (ASW * HQ)) + (Lanes([(AVG * ZH), 0.0, 0.0]) + (ASX * HV));
                    let ZJ = if BL > A { 1.0 } else { 0.0 };
                    let AHY;
                    let ASY;
                    if ZJ != 0.0 {
                        let ZK = LK - NO;
                        let BDH = Lanes([AWK, 0.0, 0.0]) - Lanes([0.0, AXB[0], AXB[1]]);
                        let ZL = C / LL;
                        let BDI = ((AWL * ZL) * AUF) / LL;
                        let ZM = if ZK < YF { 1.0 } else { 0.0 };
                        let ZS;
                        let ASZ;
                        if ZM != 0.0 {
                            let ZN = (ZK * ZL).exp();
                            let BDK = ((BDH * ZL) + Lanes([(BDI * ZK), 0.0, 0.0])) * ZN;
                            ZS = ZN;
                            ASZ = BDK;
                        } else {
                            let ZO = (YF * ZL).exp();
                            let ZP = ZK - YF;
                            let ZQ = C + (ZP * ZL);
                            let ZR = ZO * ZQ;
                            let BDJ = Lanes([(((BDI * YF) * ZO) * ZQ), 0.0, 0.0]) + (((BDH * ZL) + Lanes([(BDI * ZP), 0.0, 0.0])) * ZO);
                            ZS = ZR;
                            ASZ = BDJ;
                        }
                        let BDL = (ASZ - Lanes([AWM, 0.0, 0.0])) * BH;
                        let ZT = ZI - (BH * (ZS - LN));
                        let BDM = Lanes([BDF[0], BDF[1], 0.0, BDF[2]]) - Lanes([BDL[0], 0.0, BDL[1], BDL[2]]);
                        AHY = ZT;
                        ASY = BDM;
                    } else {
                        let BDG = Lanes([BDF[0], BDF[1], 0.0, BDF[2]]);
                        AHY = ZI;
                        ASY = BDG;
                    }
                    AHT = A;
                    AHX = AHY;
                    ASU = BDN;
                    ASV = ASY;
                } else {
                    let ZU = C / HN;
                    let BBK = ((AVD * ZU) * AUF) / HN;
                    let ZV = if NO < XB { 1.0 } else { 0.0 };
                    let AAL;
                    let ATA;
                    if ZV != 0.0 {
                        let BBN = AXB * ZU;
                        let ZW = (NO * ZU).exp();
                        let BBO = (Lanes([0.0, BBN[0], BBN[1]]) + Lanes([(BBK * NO), 0.0, 0.0])) * ZW;
                        AAL = ZW;
                        ATA = BBO;
                    } else {
                        let ZX = (XB * ZU).exp();
                        let ZY = NO - XB;
                        let BBL = AXB * ZU;
                        let ZZ = C + (ZY * ZU);
                        let AAA = ZX * ZZ;
                        let BBM = Lanes([(((BBK * XB) * ZX) * ZZ), 0.0, 0.0]) + ((Lanes([0.0, BBL[0], BBL[1]]) + Lanes([(BBK * ZY), 0.0, 0.0])) * ZX);
                        AAL = AAA;
                        ATA = BBM;
                    }
                    let AAB = C / HS;
                    let BBP = ((AVF * AAB) * AUF) / HS;
                    let AAC = if NO < XJ { 1.0 } else { 0.0 };
                    let AAN;
                    let ATB;
                    if AAC != 0.0 {
                        let BBS = AXB * AAB;
                        let AAD = (NO * AAB).exp();
                        let BBT = (Lanes([0.0, BBS[0], BBS[1]]) + Lanes([(BBP * NO), 0.0, 0.0])) * AAD;
                        AAN = AAD;
                        ATB = BBT;
                    } else {
                        let AAE = (XJ * AAB).exp();
                        let AAF = NO - XJ;
                        let BBQ = AXB * AAB;
                        let AAG = C + (AAF * AAB);
                        let AAH = AAE * AAG;
                        let BBR = Lanes([(((BBP * XJ) * AAE) * AAG), 0.0, 0.0]) + ((Lanes([0.0, BBQ[0], BBQ[1]]) + Lanes([(BBP * AAF), 0.0, 0.0])) * AAE);
                        AAN = AAH;
                        ATB = BBR;
                    }
                    let AAI = if XQ > A { 1.0 } else { 0.0 };
                    let ABC;
                    let ATC;
                    if AAI != 0.0 {
                        let AAJ = C + (XQ * (VD - C));
                        let AAK = HQ * AAJ;
                        let AAM = AAL - C;
                        let BBW = ATA * AAK;
                        let AAO = AAN - C;
                        let BBX = Lanes([(AVG * AAO), 0.0, 0.0]) + (ATB * HV);
                        let AAP = WY * ((AAK * AAM) + (HV * AAO));
                        let BBY = ((((Lanes([(AVE * AAJ), 0.0, 0.0, 0.0]) + ((BAJ * XQ) * HQ)) * AAM) + Lanes([BBW[0], 0.0, BBW[1], BBW[2]])) + Lanes([BBX[0], 0.0, BBX[1], BBX[2]])) * WY;
                        ABC = AAP;
                        ATC = BBY;
                    } else {
                        let AAQ = AAL - C;
                        let AAR = AAN - C;
                        let AAS = WY * ((HQ * AAQ) + (HV * AAR));
                        let BBU = ((Lanes([(AVE * AAQ), 0.0, 0.0]) + (ATA * HQ)) + (Lanes([(AVG * AAR), 0.0, 0.0]) + (ATB * HV))) * WY;
                        let BBV = Lanes([BBU[0], 0.0, BBU[1], BBU[2]]);
                        ABC = AAS;
                        ATC = BBV;
                    }
                    let AAT = if BL > A { 1.0 } else { 0.0 };
                    let AHU;
                    let ATD;
                    if AAT != 0.0 {
                        let AAU = LK - NO;
                        let BBZ = Lanes([AWK, 0.0, 0.0]) - Lanes([0.0, AXB[0], AXB[1]]);
                        let AAV = C / LL;
                        let BCA = ((AWL * AAV) * AUF) / LL;
                        let AAW = if AAU < YF { 1.0 } else { 0.0 };
                        let ABE;
                        let ATE;
                        if AAW != 0.0 {
                            let AAX = (AAU * AAV).exp();
                            let BCC = ((BBZ * AAV) + Lanes([(BCA * AAU), 0.0, 0.0])) * AAX;
                            ABE = AAX;
                            ATE = BCC;
                        } else {
                            let AAY = (YF * AAV).exp();
                            let AAZ = AAU - YF;
                            let ABA = C + (AAZ * AAV);
                            let ABB = AAY * ABA;
                            let BCB = Lanes([(((BCA * YF) * AAY) * ABA), 0.0, 0.0]) + (((BBZ * AAV) + Lanes([(BCA * AAZ), 0.0, 0.0])) * AAY);
                            ABE = ABB;
                            ATE = BCB;
                        }
                        let ABD = WY * BH;
                        let BCD = (ATE - Lanes([AWM, 0.0, 0.0])) * ABD;
                        let ABF = ABC - (ABD * (ABE - LN));
                        let BCE = ATC - Lanes([BCD[0], 0.0, BCD[1], BCD[2]]);
                        AHU = ABF;
                        ATD = BCE;
                    } else {
                        AHU = ABC;
                        ATD = ATC;
                    }
                    let ABG = if NQ < XB { 1.0 } else { 0.0 };
                    let ABT;
                    let ATF;
                    if ABG != 0.0 {
                        let BCH = AXC * ZU;
                        let ABH = (NQ * ZU).exp();
                        let BCI = (Lanes([0.0, BCH[0], BCH[1]]) + Lanes([(BBK * NQ), 0.0, 0.0])) * ABH;
                        ABT = ABH;
                        ATF = BCI;
                    } else {
                        let ABI = (XB * ZU).exp();
                        let ABJ = NQ - XB;
                        let BCF = AXC * ZU;
                        let ABK = C + (ABJ * ZU);
                        let ABL = ABI * ABK;
                        let BCG = Lanes([(((BBK * XB) * ABI) * ABK), 0.0, 0.0]) + ((Lanes([0.0, BCF[0], BCF[1]]) + Lanes([(BBK * ABJ), 0.0, 0.0])) * ABI);
                        ABT = ABL;
                        ATF = BCG;
                    }
                    let ABM = if NQ < XJ { 1.0 } else { 0.0 };
                    let ABV;
                    let ATG;
                    if ABM != 0.0 {
                        let BCL = AXC * AAB;
                        let ABN = (NQ * AAB).exp();
                        let BCM = (Lanes([0.0, BCL[0], BCL[1]]) + Lanes([(BBP * NQ), 0.0, 0.0])) * ABN;
                        ABV = ABN;
                        ATG = BCM;
                    } else {
                        let ABO = (XJ * AAB).exp();
                        let ABP = NQ - XJ;
                        let BCJ = AXC * AAB;
                        let ABQ = C + (ABP * AAB);
                        let ABR = ABO * ABQ;
                        let BCK = Lanes([(((BBP * XJ) * ABO) * ABQ), 0.0, 0.0]) + ((Lanes([0.0, BCJ[0], BCJ[1]]) + Lanes([(BBP * ABP), 0.0, 0.0])) * ABO);
                        ABV = ABR;
                        ATG = BCK;
                    }
                    let ABS = C - WY;
                    let ABU = ABT - C;
                    let ABW = ABV - C;
                    let ABX = ABS * ((HQ * ABU) + (HV * ABW));
                    let BCN = ((Lanes([(AVE * ABU), 0.0, 0.0]) + (ATF * HQ)) + (Lanes([(AVG * ABW), 0.0, 0.0]) + (ATG * HV))) * ABS;
                    let AHZ;
                    let ATH;
                    if AAT != 0.0 {
                        let ABY = LK - NO;
                        let BCP = Lanes([AWK, 0.0, 0.0]) - Lanes([0.0, AXB[0], AXB[1]]);
                        let ABZ = C / LL;
                        let BCQ = ((AWL * ABZ) * AUF) / LL;
                        let ACA = if ABY < YF { 1.0 } else { 0.0 };
                        let ACH;
                        let ATI;
                        if ACA != 0.0 {
                            let ACB = (ABY * ABZ).exp();
                            let BCS = ((BCP * ABZ) + Lanes([(BCQ * ABY), 0.0, 0.0])) * ACB;
                            ACH = ACB;
                            ATI = BCS;
                        } else {
                            let ACC = (YF * ABZ).exp();
                            let ACD = ABY - YF;
                            let ACE = C + (ACD * ABZ);
                            let ACF = ACC * ACE;
                            let BCR = Lanes([(((BCQ * YF) * ACC) * ACE), 0.0, 0.0]) + (((BCP * ABZ) + Lanes([(BCQ * ACD), 0.0, 0.0])) * ACC);
                            ACH = ACF;
                            ATI = BCR;
                        }
                        let ACG = ABS * BH;
                        let BCT = (ATI - Lanes([AWM, 0.0, 0.0])) * ACG;
                        let ACI = ABX - (ACG * (ACH - LN));
                        let BCU = Lanes([BCN[0], BCN[1], 0.0, BCN[2]]) - Lanes([BCT[0], 0.0, BCT[1], BCT[2]]);
                        AHZ = ACI;
                        ATH = BCU;
                    } else {
                        let BCO = Lanes([BCN[0], BCN[1], 0.0, BCN[2]]);
                        AHZ = ABX;
                        ATH = BCO;
                    }
                    AHT = AHU;
                    AHX = AHZ;
                    ASU = ATD;
                    ASV = ATH;
                }
                AHR = AHT;
                AHW = AHX;
                ASN = ASU;
                ASO = ASV;
            }
            let ACJ = C / HY;
            let BEK = ((AVI * ACJ) * AUF) / HY;
            let ACL = if NS < ACK { 1.0 } else { 0.0 };
            let ACZ;
            let ATJ;
            if ACL != 0.0 {
                let BEN = AXD * ACJ;
                let ACM = (NS * ACJ).exp();
                let BEO = (Lanes([0.0, BEN[0], BEN[1]]) + Lanes([(BEK * NS), 0.0, 0.0])) * ACM;
                ACZ = ACM;
                ATJ = BEO;
            } else {
                let ACN = (ACK * ACJ).exp();
                let ACO = NS - ACK;
                let BEL = AXD * ACJ;
                let ACP = C + (ACO * ACJ);
                let ACQ = ACN * ACP;
                let BEM = Lanes([(((BEK * ACK) * ACN) * ACP), 0.0, 0.0]) + ((Lanes([0.0, BEL[0], BEL[1]]) + Lanes([(BEK * ACO), 0.0, 0.0])) * ACN);
                ACZ = ACQ;
                ATJ = BEM;
            }
            let ACR = C / IE;
            let BEP = ((AVM * ACR) * AUF) / IE;
            let ACT = if NS < ACS { 1.0 } else { 0.0 };
            let ADB;
            let ATK;
            if ACT != 0.0 {
                let BES = AXD * ACR;
                let ACU = (NS * ACR).exp();
                let BET = (Lanes([0.0, BES[0], BES[1]]) + Lanes([(BEP * NS), 0.0, 0.0])) * ACU;
                ADB = ACU;
                ATK = BET;
            } else {
                let ACV = (ACS * ACR).exp();
                let ACW = NS - ACS;
                let BEQ = AXD * ACR;
                let ACX = C + (ACW * ACR);
                let ACY = ACV * ACX;
                let BER = Lanes([(((BEP * ACS) * ACV) * ACX), 0.0, 0.0]) + ((Lanes([0.0, BEQ[0], BEQ[1]]) + Lanes([(BEP * ACW), 0.0, 0.0])) * ACV);
                ADB = ACY;
                ATK = BER;
            }
            let ADA = ACZ - C;
            let ADC = ADB - C;
            let ADD = (IB * ADA) + (IH * ADC);
            let BEU = (Lanes([(AVK * ADA), 0.0, 0.0]) + (ATJ * IB)) + (Lanes([(AVO * ADC), 0.0, 0.0]) + (ATK * IH));
            let ADE = if (if EL > A { 1.0 } else { 0.0 }) != 0.0 || (if EP > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let AIA;
            let ATL;
            if ADE != 0.0 {
                let ADG = if NX < ADF { 1.0 } else { 0.0 };
                let ADT;
                let ATM;
                if ADG != 0.0 {
                    let BEY = AXG * ACJ;
                    let ADH = (NX * ACJ).exp();
                    let BEZ = (Lanes([0.0, BEY[0], BEY[1]]) + Lanes([(BEK * NX), 0.0, 0.0])) * ADH;
                    ADT = ADH;
                    ATM = BEZ;
                } else {
                    let ADI = (ADF * ACJ).exp();
                    let ADJ = NX - ADF;
                    let BEW = AXG * ACJ;
                    let ADK = C + (ADJ * ACJ);
                    let ADL = ADI * ADK;
                    let BEX = Lanes([(((BEK * ADF) * ADI) * ADK), 0.0, 0.0]) + ((Lanes([0.0, BEW[0], BEW[1]]) + Lanes([(BEK * ADJ), 0.0, 0.0])) * ADI);
                    ADT = ADL;
                    ATM = BEX;
                }
                let ADN = if NX < ADM { 1.0 } else { 0.0 };
                let ADV;
                let ATN;
                if ADN != 0.0 {
                    let BFC = AXG * ACR;
                    let ADO = (NX * ACR).exp();
                    let BFD = (Lanes([0.0, BFC[0], BFC[1]]) + Lanes([(BEP * NX), 0.0, 0.0])) * ADO;
                    ADV = ADO;
                    ATN = BFD;
                } else {
                    let ADP = (ADM * ACR).exp();
                    let ADQ = NX - ADM;
                    let BFA = AXG * ACR;
                    let ADR = C + (ADQ * ACR);
                    let ADS = ADP * ADR;
                    let BFB = Lanes([(((BEP * ADM) * ADP) * ADR), 0.0, 0.0]) + ((Lanes([0.0, BFA[0], BFA[1]]) + Lanes([(BEP * ADQ), 0.0, 0.0])) * ADP);
                    ADV = ADS;
                    ATN = BFB;
                }
                let ADU = ADT - C;
                let ADW = ADV - C;
                let ADX = (IJ * ADU) + (IL * ADW);
                let BFE = (Lanes([(AVP * ADU), 0.0, 0.0]) + (ATM * IJ)) + (Lanes([(AVQ * ADW), 0.0, 0.0]) + (ATN * IL));
                AIA = ADX;
                ATL = BFE;
            } else {
                AIA = A;
                ATL = BEV;
            }
            let ADY = NS / FI;
            let BFF = Lanes([0.0, AXD[0], AXD[1]]);
            let BFG = (BFF - Lanes([(AUI * ADY), 0.0, 0.0])) / FI;
            let ADZ = if ADY < Q { 1.0 } else { 0.0 };
            let AEI;
            let ATO;
            if ADZ != 0.0 {
                let AEA = ADY.exp();
                let BFI = BFG * AEA;
                AEI = AEA;
                ATO = BFI;
            } else {
                let AEB = Q.exp();
                let AEC = AEB * (C + (ADY - Q));
                let BFH = BFG * AEB;
                AEI = AEC;
                ATO = BFH;
            }
            let AED = NU / FI;
            let BFJ = (Lanes([0.0, AXE[0], AXE[1]]) - Lanes([(AUI * AED), 0.0, 0.0])) / FI;
            let AEE = if AED < Q { 1.0 } else { 0.0 };
            let AEK;
            let ATP;
            if AEE != 0.0 {
                let AEF = AED.exp();
                let BFL = BFJ * AEF;
                AEK = AEF;
                ATP = BFL;
            } else {
                let AEG = Q.exp();
                let AEH = AEG * (C + (AED - Q));
                let BFK = BFJ * AEG;
                AEK = AEH;
                ATP = BFK;
            }
            let AEJ = (C + (LG * AEI)).sqrt();
            let BFM = (Lanes([(AWI * AEI), 0.0, 0.0]) + (ATO * LG)) * (AQQ / (AWA * AEJ));
            let AEL = (C + (LG * AEK)).sqrt();
            let BFN = (Lanes([(AWI * AEK), 0.0, 0.0]) + (ATP * LG)) * (AQQ / (AWA * AEL));
            let AEM = OE * LZ;
            let BFO = AXK * LZ;
            let BFP = Lanes([BFO[0], 0.0, BFO[1]]) + Lanes([0.0, (ARK * OE), 0.0]);
            let AEN = AEL + C;
            let AEO = (AEJ + C) / AEN;
            let BFQ = BFN * AEO;
            let BFR = Lanes([BFM[0], 0.0, BFM[1], BFM[2]]);
            let AEP = (AEJ - AEL) - (AEO.ln());
            let AEQ = OF + (FI * AEP);
            let AER = AEQ * MD;
            let BFS = ((Lanes([0.0, AXL[0], AXL[1], 0.0]) + (Lanes([(AUI * AEP), 0.0, 0.0, 0.0]) + (((BFR - Lanes([BFN[0], BFN[1], 0.0, BFN[2]])) - (((BFR - Lanes([BFQ[0], BFQ[1], 0.0, BFQ[2]])) / AEN) * (AQQ / AEO))) * FI))) * MD) + Lanes([(ARL * AEQ), 0.0, 0.0, 0.0]);
            let AES = (BY * NI) * AC;
            let BFT = AXL * OF;
            let AET = ((OF * OF) + E).sqrt();
            let BFU = ((BFT + BFT) * (AQQ / (AWA * AET))) * AES;
            let AEU = C + (AES * AET);
            let AEV = MD * AEU;
            let AEW = (NI * AER) / AEV;
            let BFV = (Lanes([(ARL * AEU), 0.0, 0.0]) + ((Lanes([(((ARU * BY) * AC) * AET), 0.0, 0.0]) + Lanes([0.0, BFU[0], BFU[1]])) * MD)) * AEW;
            let BFW = (((Lanes([(ARU * AER), 0.0, 0.0, 0.0]) + (BFS * NI)) - Lanes([BFV[0], BFV[1], BFV[2], 0.0])) / AEV) * AEW;
            let AEX = (C + (AEW * AEW)).sqrt();
            let AEY = AER / AEX;
            let BFX = (BFS - (((BFW + BFW) * (AQQ / (AWA * AEX))) * AEY)) / AEX;
            let AEZ = OG * MH;
            let BFY = AXM * MH;
            let BFZ = Lanes([BFY[0], 0.0, BFY[1]]) + Lanes([0.0, (ARM * OG), 0.0]);
            let AFA = OH * VS;
            let BGA = AXN * VS;
            let BGB = ASF * OH;
            let AFB = AFA * ML;
            let BGC = ((Lanes([0.0, 0.0, BGA[0], BGA[1], 0.0]) + Lanes([BGB[0], BGB[1], 0.0, BGB[2], BGB[3]])) * ML) + Lanes([(ARN * AFA), 0.0, 0.0, 0.0, 0.0]);
            let AFC = OI * MO;
            let BGD = AXO * MO;
            let BGE = Lanes([BGD[0], 0.0, BGD[1]]) + Lanes([0.0, (ARO * OI), 0.0]);
            let AFF = OJ * AFD;
            let BGF = AXP * AFD;
            let BGG = ASI * OJ;
            let AFG = AFF * MS;
            let BGH = ((Lanes([0.0, BGF[0], 0.0, 0.0, 0.0, BGF[1]]) + Lanes([BGG[0], 0.0, BGG[1], BGG[2], BGG[3], BGG[4]])) * MS) + Lanes([(ARP * AFF), 0.0, 0.0, 0.0, 0.0, 0.0]);
            let AFI = if AFH > A { 1.0 } else { 0.0 };
            let AHO;
            let ATQ;
            if AFI != 0.0 {
                let AFK = AFJ * (IS + C);
                let AFM = C / (AFL - KX);
                let AFN = AFK.powf(AFM);
                let AFO = (KQ - NS) - AFN;
                let BGJ = Lanes([((AVU * AFJ) * (AFM * (AFK.powf((AFM - AQQ))))), 0.0, 0.0]);
                let BGK = (Lanes([AWD, 0.0, 0.0]) - BFF) - BGJ;
                let BGL = BGK * AFO;
                let AFP = ((AFO * AFO) + E).sqrt();
                let AFQ = (BY * (AFP + AFO)) + AFN;
                let BGM = ((((BGL + BGL) * (AQQ / (AWA * AFP))) + BGK) * BY) + BGJ;
                let AFR = -IS;
                let AFS = KX - C;
                let AFT = AFQ.powf(AFS);
                let AFU = AFR * AFT;
                let BGN = Lanes([((AVU * AUF) * AFT), 0.0, 0.0]) + ((BGM * (AFS * (AFQ.powf((AFS - AQQ))))) * AFR);
                let AFV = if AFU < Q { 1.0 } else { 0.0 };
                let AGA;
                let ATR;
                if AFV != 0.0 {
                    let AFW = AFU.exp();
                    let BGP = BGN * AFW;
                    AGA = AFW;
                    ATR = BGP;
                } else {
                    let AFX = Q.exp();
                    let AFY = AFX * (C + (AFU - Q));
                    let BGO = BGN * AFX;
                    AGA = AFY;
                    ATR = BGO;
                }
                let AFZ = AFH * AFQ;
                let AGB = AFZ * AGA;
                let AGC = (OL - VV) - ADD;
                let AGD = AGC * AGB;
                let BGQ = (((BGM * AFH) * AGA) + (ATR * AFZ)) * AGC;
                let BGR = (((Lanes([0.0, 0.0, 0.0, 0.0, ARC]) - Lanes([BAT[0], BAT[1], BAT[2], BAT[3], 0.0])) - Lanes([BEU[0], BEU[1], BEU[2], 0.0, 0.0])) * AGB) + Lanes([BGQ[0], BGQ[1], BGQ[2], 0.0, 0.0]);
                AHO = AGD;
                ATQ = BGR;
            } else {
                AHO = A;
                ATQ = BGI;
            }
            let AGF = if AGE > A { 1.0 } else { 0.0 };
            let AIH;
            let ATS;
            if AGF != 0.0 {
                let AGG = AFJ * (IV + C);
                let AGI = C / (AFL - AGH);
                let AGJ = AGG.powf(AGI);
                let BGT = AXF * AUF;
                let AGK = (A - NV) - AGJ;
                let BGU = Lanes([((AVV * AFJ) * (AGI * (AGG.powf((AGI - AQQ))))), 0.0, 0.0]);
                let BGV = Lanes([0.0, BGT[0], BGT[1]]) - BGU;
                let BGW = BGV * AGK;
                let AGL = ((AGK * AGK) + E).sqrt();
                let AGM = (BY * (AGL + AGK)) + AGJ;
                let BGX = ((((BGW + BGW) * (AQQ / (AWA * AGL))) + BGV) * BY) + BGU;
                let AGN = -IV;
                let AGO = AGH - C;
                let AGP = AGM.powf(AGO);
                let AGQ = AGN * AGP;
                let BGY = Lanes([((AVV * AUF) * AGP), 0.0, 0.0]) + ((BGX * (AGO * (AGM.powf((AGO - AQQ))))) * AGN);
                let AGR = if AGQ < Q { 1.0 } else { 0.0 };
                let AGW;
                let ATT;
                if AGR != 0.0 {
                    let AGS = AGQ.exp();
                    let BHA = BGY * AGS;
                    AGW = AGS;
                    ATT = BHA;
                } else {
                    let AGT = Q.exp();
                    let AGU = AGT * (C + (AGQ - Q));
                    let BGZ = BGY * AGT;
                    AGW = AGU;
                    ATT = BGZ;
                }
                let AGV = AGE * AGM;
                let AGX = AGV * AGW;
                let AGY = -AEM;
                let AGZ = AGY * AGX;
                let BHB = (BFP * AUF) * AGX;
                let BHC = (((BGX * AGE) * AGW) + (ATT * AGV)) * AGY;
                let BHD = Lanes([BHB[0], BHB[1], BHB[2], 0.0]) + Lanes([0.0, BHC[0], BHC[1], BHC[2]]);
                AIH = AGZ;
                ATS = BHD;
            } else {
                AIH = A;
                ATS = BGS;
            }
            let AHC = if (if AHA > A { 1.0 } else { 0.0 }) != 0.0 && (if AHB > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let AHP;
            let ATU;
            if AHC != 0.0 {
                let AHE = if AHD > A { 1.0 } else { 0.0 };
                let AHJ;
                let ATV;
                if AHE != 0.0 {
                    let BHF = (AXD / AHD) * AUF;
                    let AHG = (C - (NS / AHD)) - AHF;
                    let BHG = BHF * AHG;
                    let AHH = ((AHG * AHG) + UZ).sqrt();
                    let AHI = AHB * (AHF + (BY * (AHG + AHH)));
                    let BHH = ((BHF + ((BHG + BHG) * (AQQ / (AWA * AHH)))) * BY) * AHB;
                    AHJ = AHI;
                    ATV = BHH;
                } else {
                    AHJ = AHB;
                    ATV = BHE;
                }
                let AHK = VW / AHJ;
                let BHI = ATV * AHK;
                let AHL = AHK - C;
                let AHN = AHA * (AHL.powf(AHM));
                let BHJ = (((BAU - Lanes([0.0, BHI[0], BHI[1], 0.0])) / AHJ) * (AHM * (AHL.powf((AHM - AQQ))))) * AHA;
                AHP = AHN;
                ATU = BHJ;
            } else {
                AHP = A;
                ATU = BDN;
            }
            let AHQ = (ADD - AHO) - AHP;
            let BHK = (Lanes([BEU[0], BEU[1], BEU[2], 0.0, 0.0]) - ATQ) - Lanes([ATU[0], ATU[1], ATU[2], ATU[3], 0.0]);
            let BHL = AXB * AHR;
            let BHM = (ASN * NO) + Lanes([0.0, 0.0, BHL[0], BHL[1]]);
            let BHN = AXD * AHQ;
            let AHV = OL - VV;
            let BHO = Lanes([0.0, 0.0, 0.0, 0.0, ARC]);
            let BHP = AXI * AHV;
            let BHQ = (Lanes([BHM[0], BHM[1], BHM[2], BHM[3], 0.0]) + ((BHK * NS) + Lanes([0.0, BHN[0], BHN[1], 0.0, 0.0]))) + (((BHO - Lanes([BAT[0], BAT[1], BAT[2], BAT[3], 0.0])) * OB) + Lanes([0.0, BHP[0], 0.0, BHP[1], 0.0]));
            let BHR = AXC * AHW;
            let BHS = (ASO * NQ) + Lanes([0.0, BHR[0], 0.0, BHR[1]]);
            let BHT = Lanes([BHQ[0], BHQ[1], 0.0, BHQ[2], BHQ[3], BHQ[4]]) + Lanes([BHS[0], 0.0, BHS[1], BHS[2], BHS[3], 0.0]);
            let BHU = AXG * AIA;
            let BHV = (ATL * NX) + Lanes([0.0, BHU[0], BHU[1]]);
            let BHW = Lanes([BHT[0], BHT[1], BHT[2], BHT[3], BHT[4], 0.0, BHT[5]]) + Lanes([BHV[0], 0.0, BHV[1], 0.0, 0.0, BHV[2], 0.0]);
            let BHX = AXK * AEM;
            let BHY = (BFP * OE) + Lanes([BHX[0], 0.0, BHX[1]]);
            let BHZ = AXL * AEY;
            let BIA = (BFX * OF) + Lanes([0.0, BHZ[0], BHZ[1], 0.0]);
            let BIB = (Lanes([0.0, BHW[0], 0.0, BHW[1], BHW[2], BHW[3], BHW[4], BHW[5], BHW[6]]) + Lanes([BHY[0], BHY[1], BHY[2], 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])) + Lanes([0.0, BIA[0], BIA[1], BIA[2], 0.0, BIA[3], 0.0, 0.0, 0.0]);
            let BIC = AXM * AEZ;
            let BID = (BFZ * OG) + Lanes([BIC[0], 0.0, BIC[1]]);
            let BIE = AXN * AFB;
            let BIF = (BGC * OH) + Lanes([0.0, 0.0, BIE[0], BIE[1], 0.0]);
            let BIG = (Lanes([BIB[0], 0.0, BIB[1], BIB[2], BIB[3], BIB[4], BIB[5], BIB[6], BIB[7], BIB[8]]) + Lanes([0.0, BID[0], BID[1], 0.0, 0.0, BID[2], 0.0, 0.0, 0.0, 0.0])) + Lanes([0.0, 0.0, BIF[0], 0.0, BIF[1], BIF[2], BIF[3], BIF[4], 0.0, 0.0]);
            let BIH = AXO * AFC;
            let BII = (BGE * OI) + Lanes([BIH[0], 0.0, BIH[1]]);
            let BIJ = AXP * AFG;
            let BIK = (BGH * OJ) + Lanes([0.0, BIJ[0], 0.0, 0.0, 0.0, BIJ[1]]);
            let AIB = -parameters[2];
            let AIC = AIB * (((((((((((AHR * NO) + (AHQ * NS)) + (AHV * OB)) + (AHW * NQ)) + (AIA * NX)) + (AEM * OE)) + (AEY * OF)) + (AEZ * OG)) + (AFB * OH)) + (AFC * OI)) + (AFG * OJ));
            let BIL = ((Lanes([BIG[0], BIG[1], 0.0, BIG[2], BIG[3], BIG[4], BIG[5], BIG[6], BIG[7], BIG[8], BIG[9]]) + Lanes([0.0, 0.0, BII[0], BII[1], 0.0, 0.0, 0.0, 0.0, BII[2], 0.0, 0.0])) + Lanes([0.0, 0.0, 0.0, BIK[0], BIK[1], BIK[2], BIK[3], BIK[4], 0.0, BIK[5], 0.0])) * AIB;
            let AID = EX * MW;
            let BIM = (AQR * MW) + (ARQ * EX);
            let AIE = OL - VW;
            let BIN = BHO - Lanes([BAU[0], BAU[1], BAU[2], BAU[3], 0.0]);
            let AIF = OL - OK;
            let BIO = Lanes([0.0, ARC]) - Lanes([ARB, 0.0]);
            let BIP = AXB * AIG;
            let BIQ = AXC * AIG;
            let BIR = AXG * AIG;
            let BIS = AXD * AIG;
            let BIT = AXF * AIG;
            let AII = NJ * (AHR + (AIG * NO));
            let BIU = (ASN + Lanes([0.0, 0.0, BIP[0], BIP[1]])) * NJ;
            let AIJ = NJ * (AHW + (AIG * NQ));
            let BIV = (ASO + Lanes([0.0, BIQ[0], 0.0, BIQ[1]])) * NJ;
            let AIK = NJ * OL;
            let BIW = ARC * NJ;
            let AIL = NJ * VV;
            let BIX = BAT * NJ;
            let AIM = NJ * (AHQ + (AIG * NS));
            let BIY = (BHK + Lanes([0.0, BIS[0], BIS[1], 0.0, 0.0])) * NJ;
            let AIN = NJ * (AIH + (AIG * NV));
            let BIZ = (ATS + Lanes([0.0, 0.0, BIT[0], BIT[1]])) * NJ;
            let AIO = NJ * (AIA + (AIG * NX));
            let BJA = (ATL + Lanes([0.0, BIR[0], BIR[1]])) * NJ;
            let AIP = NJ * AEY;
            let BJB = BFX * NJ;
            let AOJ;
            let ATW;
            if OQ != 0.0 {
                let AIQ = NQ + OO;
                let BJM = Lanes([0.0, AXC[0], AXC[1]]);
                let BJN = BJM + Lanes([AXR, 0.0, 0.0]);
                let AIR = if AIQ > A { 1.0 } else { 0.0 };
                let AJH;
                let AJI;
                let ATX;
                let ATY;
                if AIR != 0.0 {
                    let AIS = C - ON;
                    let AIT = AIS.powf((-KT));
                    let AIU = C - (AIT * AIS);
                    let AIV = C - KT;
                    let AIW = (JZ * AIU) / AIV;
                    let AIX = BY * KT;
                    let AIY = JZ * AIS;
                    let AIZ = (AIX * AIQ) / AIY;
                    let AJA = C + AIZ;
                    let AJB = (AIQ * AJA) * AIT;
                    let BJQ = ((BJN * AJA) + ((((BJN * AIX) - Lanes([((AWB * AIS) * AIZ), 0.0, 0.0])) / AIY) * AIQ)) * AIT;
                    let BJR = Lanes([((AWB * AIU) / AIV), 0.0, 0.0]);
                    AJH = AIW;
                    AJI = AJB;
                    ATX = BJR;
                    ATY = BJQ;
                } else {
                    let AJC = NQ / JZ;
                    let AJD = C - AJC;
                    let AJE = C - KT;
                    let AJF = C - (AJD.powf(AJE));
                    let AJG = (JZ * AJF) / AJE;
                    let BJO = (Lanes([(AWB * AJF), 0.0, 0.0]) + ((((((BJM - Lanes([(AWB * AJC), 0.0, 0.0])) / JZ) * AUF) * (AJE * (AJD.powf((AJE - AQQ))))) * AUF) * JZ)) / AJE;
                    AJH = AJG;
                    AJI = A;
                    ATX = BJO;
                    ATY = BJP;
                }
                let AJJ = AJH + AJI;
                let BJS = ATX + ATY;
                AOJ = AJJ;
                ATW = BJS;
            } else {
                let BJC = AXR * OO;
                let AJK = (BZ * OP) * OP;
                let AJL = ((OO * OO) + AJK).sqrt();
                let AJN = AJM * (OO + AJL);
                let BJD = (AXR + ((BJC + BJC) * (AQQ / (AWA * AJL)))) * AJM;
                let AJO = AJN / JZ;
                let AJP = C - AJO;
                let AJQ = C - KT;
                let AJR = AJP.powf(AJQ);
                let BJE = AJQ - AQQ;
                let AJS = NQ + OO;
                let BJF = Lanes([0.0, AXC[0], AXC[1]]);
                let BJG = Lanes([AXR, 0.0, 0.0]);
                let BJH = BJF + BJG;
                let BJI = BJH * AJS;
                let AJT = ((AJS * AJS) + AJK).sqrt();
                let AJU = (BY * (AJS - AJT)) - OO;
                let BJJ = ((BJH - ((BJI + BJI) * (AQQ / (AWA * AJT)))) * BY) - BJG;
                let AJV = AJU / JZ;
                let AJW = C - AJV;
                let AJX = AJW.powf(AJQ);
                let AJY = C - ON;
                let AJZ = AJY.powf((-KT));
                let AKA = (NQ - AJU) + AJN;
                let BJK = (BJF - BJJ) + Lanes([BJD, 0.0, 0.0]);
                let AKB = AJZ * AKA;
                let AKC = BY * KT;
                let AKD = JZ * AJY;
                let AKE = (AKC * AKA) / AKD;
                let AKF = C + AKE;
                let AKG = (((OM * AJX) / AJQ) + (AKB * AKF)) - ((OM * AJR) / AJQ);
                let BJL = (((Lanes([(AXQ * AJX), 0.0, 0.0]) + (((((BJJ - Lanes([(AWB * AJV), 0.0, 0.0])) / JZ) * AUF) * (AJQ * (AJW.powf(BJE)))) * OM)) / AJQ) + (((BJK * AJZ) * AKF) + ((((BJK * AKC) - Lanes([((AWB * AJY) * AKE), 0.0, 0.0])) / AKD) * AKB))) - Lanes([(((AXQ * AJR) + (((((BJD - (AWB * AJO)) / JZ) * AUF) * (AJQ * (AJP.powf(BJE)))) * OM)) / AJQ), 0.0, 0.0]);
                AOJ = AKG;
                ATW = BJL;
            }
            let AON;
            let ATZ;
            if QL != 0.0 {
                let AKH = NX + QJ;
                let BKS = Lanes([0.0, AXG[0], AXG[1]]);
                let BKT = BKS + Lanes([AYK, 0.0, 0.0]);
                let AKI = if AKH > A { 1.0 } else { 0.0 };
                let ALH;
                let ALJ;
                let AUA;
                let AUB;
                if AKI != 0.0 {
                    let AKJ = C - ON;
                    let AKK = AKJ.powf((-1e0f64 - KX));
                    let AKL = C - ((AKK * AKJ) * AKJ);
                    let AKM = C - KX;
                    let AKN = (KQ * AKL) / AKM;
                    let AKO = BY * KX;
                    let AKP = (AKO * AKH) / KQ;
                    let AKQ = AKJ + AKP;
                    let AKR = (AKH * AKQ) * AKK;
                    let BKX = ((BKT * AKQ) + ((((BKT * AKO) - Lanes([(AWD * AKP), 0.0, 0.0])) / KQ) * AKH)) * AKK;
                    let BKY = Lanes([((AWD * AKL) / AKM), 0.0, 0.0]);
                    ALH = AKN;
                    ALJ = AKR;
                    AUA = BKY;
                    AUB = BKX;
                } else {
                    let AKS = if (if QX > A { 1.0 } else { 0.0 }) != 0.0 && (if NX < (-QX) { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let ALI;
                    let AUC;
                    if AKS != 0.0 {
                        let AKT = QX / KQ;
                        let AKU = C + AKT;
                        let AKV = C - KX;
                        let AKW = AKU.powf(AKV);
                        let BKV = AXG * AKV;
                        let AKX = KQ + QX;
                        let AKY = (AKV * (NX + QX)) / AKX;
                        let AKZ = C - AKY;
                        let ALA = C - (AKW * AKZ);
                        let ALB = (KQ * ALA) / AKV;
                        let BKW = (Lanes([(AWD * ALA), 0.0, 0.0]) + (((Lanes([(((((AWD * AKT) * AUF) / KQ) * (AKV * (AKU.powf((AKV - AQQ))))) * AKZ), 0.0, 0.0]) + ((((Lanes([0.0, BKV[0], BKV[1]]) - Lanes([(AWD * AKY), 0.0, 0.0])) / AKX) * AUF) * AKW)) * AUF) * KQ)) / AKV;
                        ALI = ALB;
                        AUC = BKW;
                    } else {
                        let ALC = NX / KQ;
                        let ALD = C - ALC;
                        let ALE = C - KX;
                        let ALF = C - (ALD.powf(ALE));
                        let ALG = (KQ * ALF) / ALE;
                        let BKU = (Lanes([(AWD * ALF), 0.0, 0.0]) + ((((((BKS - Lanes([(AWD * ALC), 0.0, 0.0])) / KQ) * AUF) * (ALE * (ALD.powf((ALE - AQQ))))) * AUF) * KQ)) / ALE;
                        ALI = ALG;
                        AUC = BKU;
                    }
                    ALH = ALI;
                    ALJ = A;
                    AUA = AUC;
                    AUB = BEV;
                }
                let ALK = ALH + ALJ;
                let BKZ = AUA + AUB;
                AON = ALK;
                ATZ = BKZ;
            } else {
                let ALL = if (if QX > A { 1.0 } else { 0.0 }) != 0.0 && (if RR > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let AOO;
                let AUD;
                if ALL != 0.0 {
                    let ALM = QX - QJ;
                    let BKC = AYK * AUF;
                    let ALN = (QX + QJ) / ALM;
                    let BKD = (AYK - (BKC * ALN)) / ALM;
                    let ALO = ALN - C;
                    let BKE = BKD * ALO;
                    let ALP = (BZ * QK) * QK;
                    let ALQ = ((ALO * ALO) + ALP).sqrt();
                    let ALR = ALN + C;
                    let BKF = BKD * ALR;
                    let ALS = (BZ * RR) * RR;
                    let ALT = ((ALR * ALR) + ALS).sqrt();
                    let ALU = ALQ + ALT;
                    let ALV = (JA * ALN) / ALU;
                    let ALW = BY * (((ALV * ALM) - QX) - QJ);
                    let BKG = ((((((BKD * JA) - ((((BKE + BKE) * (AQQ / (AWA * ALQ))) + ((BKF + BKF) * (AQQ / (AWA * ALT)))) * ALV)) / ALU) * ALM) + (BKC * ALV)) - AYK) * BY;
                    let ALX = ALW / KQ;
                    let ALY = C - ALX;
                    let ALZ = C - KX;
                    let BKH = ALZ - AQQ;
                    let AMA = C - (ALY.powf(ALZ));
                    let BKI = AXG * JA;
                    let BKJ = Lanes([AYK, 0.0, 0.0]);
                    let AMB = (((JA * NX) + QX) + QJ) / ALM;
                    let BKK = ((Lanes([0.0, BKI[0], BKI[1]]) + BKJ) - Lanes([(BKC * AMB), 0.0, 0.0])) / ALM;
                    let AMC = AMB - C;
                    let BKL = BKK * AMC;
                    let AMD = ((AMC * AMC) + ALP).sqrt();
                    let AME = AMB + C;
                    let BKM = BKK * AME;
                    let AMF = ((AME * AME) + ALS).sqrt();
                    let AMG = AMD + AMF;
                    let AMH = (JA * AMB) / AMG;
                    let BKN = ((BKK * JA) - ((((BKL + BKL) * (AQQ / (AWA * AMD))) + ((BKM + BKM) * (AQQ / (AWA * AMF)))) * AMH)) / AMG;
                    let AMI = BY * (((AMH * ALM) - QX) - QJ);
                    let BKO = (((BKN * ALM) + Lanes([(BKC * AMH), 0.0, 0.0])) - BKJ) * BY;
                    let AMJ = AMI / KQ;
                    let AMK = C - AMJ;
                    let AML = C - (AMK.powf(ALZ));
                    let AMM = BY * (AMH + C);
                    let BKP = BKN * BY;
                    let AMN = QX / KQ;
                    let AMO = C + AMN;
                    let AMP = -KX;
                    let AMQ = AMO.powf(AMP);
                    let BKQ = AMP - AQQ;
                    let AMR = QJ / KQ;
                    let AMS = C + AMR;
                    let AMT = AMS.powf(AMP);
                    let AMU = C - AMM;
                    let AMV = (AMU * AMQ) + (AMM * AMT);
                    let AMW = (NX - AMI) + ALW;
                    let AMX = ((AMW * AMV) + ((KQ * AML) / ALZ)) - ((KQ * AMA) / ALZ);
                    let BKR = (((((Lanes([0.0, AXG[0], AXG[1]]) - BKO) + Lanes([BKG, 0.0, 0.0])) * AMV) + (((((BKP * AUF) * AMQ) + Lanes([(((((AWD * AMN) * AUF) / KQ) * (AMP * (AMO.powf(BKQ)))) * AMU), 0.0, 0.0])) + ((BKP * AMT) + Lanes([((((AYK - (AWD * AMR)) / KQ) * (AMP * (AMS.powf(BKQ)))) * AMM), 0.0, 0.0]))) * AMW)) + ((Lanes([(AWD * AML), 0.0, 0.0]) + ((((((BKO - Lanes([(AWD * AMJ), 0.0, 0.0])) / KQ) * AUF) * (ALZ * (AMK.powf(BKH)))) * AUF) * KQ)) / ALZ)) - Lanes([(((AWD * AMA) + ((((((BKG - (AWD * ALX)) / KQ) * AUF) * (ALZ * (ALY.powf(BKH)))) * AUF) * KQ)) / ALZ), 0.0, 0.0]);
                    AOO = AMX;
                    AUD = BKR;
                } else {
                    let BJT = AYK * QJ;
                    let AMY = (BZ * QK) * QK;
                    let AMZ = ((QJ * QJ) + AMY).sqrt();
                    let ANB = ANA * (QJ + AMZ);
                    let BJU = (AYK + ((BJT + BJT) * (AQQ / (AWA * AMZ)))) * ANA;
                    let ANC = ANB / KQ;
                    let AND = C - ANC;
                    let ANE = C - KX;
                    let ANF = AND.powf(ANE);
                    let BJV = ANE - AQQ;
                    let ANG = NX + QJ;
                    let BJW = Lanes([0.0, AXG[0], AXG[1]]);
                    let BJX = Lanes([AYK, 0.0, 0.0]);
                    let BJY = BJW + BJX;
                    let BJZ = BJY * ANG;
                    let ANH = ((ANG * ANG) + AMY).sqrt();
                    let ANI = (BY * (ANG - ANH)) - QJ;
                    let BKA = ((BJY - ((BJZ + BJZ) * (AQQ / (AWA * ANH)))) * BY) - BJX;
                    let ANJ = ANI / KQ;
                    let ANK = C - ANJ;
                    let ANL = ANK.powf(ANE);
                    let ANM = (C - ON).powf((-KX));
                    let ANN = (((QI * ANL) / ANE) + (ANM * ((NX - ANI) + ANB))) - ((QI * ANF) / ANE);
                    let BKB = (((Lanes([(AYJ * ANL), 0.0, 0.0]) + (((((BKA - Lanes([(AWD * ANJ), 0.0, 0.0])) / KQ) * AUF) * (ANE * (ANK.powf(BJV)))) * QI)) / ANE) + (((BJW - BKA) + Lanes([BJU, 0.0, 0.0])) * ANM)) - Lanes([(((AYJ * ANF) + (((((BJU - (AWD * ANC)) / KQ) * AUF) * (ANE * (AND.powf(BJV)))) * QI)) / ANE), 0.0, 0.0]);
                    AOO = ANN;
                    AUD = BKB;
                }
                AON = AOO;
                ATZ = AUD;
            }
            let ANO = if UH > A { 1.0 } else { 0.0 };
            let ANP = if ANO != 0.0 {
                C
            } else {
                A
            };
            let ANQ = (UH * ANP) * AK;
            let BLA = (AZY * ANP) * AK;
            let ANR = ANQ + C;
            let ANS = ANQ / ANR;
            let BLB = (BLA - (BLA * ANS)) / ANR;
            let ANU = (NS * AG) / ANT;
            let BLC = (AXD * AG) / ANT;
            let ANV = if ANU < Q { 1.0 } else { 0.0 };
            let AOD;
            let AUE;
            if ANV != 0.0 {
                let ANW = ANU.exp();
                let BLE = BLC * ANW;
                AOD = ANW;
                AUE = BLE;
            } else {
                let ANX = Q.exp();
                let ANY = ANX * (C + (ANU - Q));
                let BLD = BLC * ANX;
                AOD = ANY;
                AUE = BLD;
            }
            let AOB = ANZ * (C + (AOA * VD));
            let AOE = AOC * AOD;
            let BLF = BLB * ANS;
            let AOF = AL + (ANS * ANS);
            let BLG = (AUE * AOC) * AOF;
            let BLH = (BLF + BLF) * AOE;
            let AOG = C + ((AOE * AOF) * ANP);
            let AOH = AOB * AOG;
            let BLI = (Lanes([(AWE * UW), 0.0, 0.0]) + (ARV * KU)) * WY;
            let BLJ = AZY * AOH;
            let AOI = (AOH * UH) / VS;
            let AOK = C - WY;
            let BLK = Lanes([(AWH * AON), 0.0, 0.0]) + (ATZ * LB);
            let AOR = OA * AOQ;
            let BLL = AXH * AOQ;
            let AOT = OD * AOS;
            let BLM = AXJ * AOS;
            let AOV = EX * AOU;
            let BLN = AQR * AOU;
            let AOX = AOW * OK;
            let BLO = ARB * AOW;
            let AOZ = (AOW * OL) * AOY;
            let BLP = (ARC * AOW) * AOY;
            let APA = NJ * (((KU * UW) * WY) + AOI);
            let BLQ = (Lanes([BLI[0], 0.0, BLI[1], BLI[2]]) + ((((((((BAJ * AOA) * ANZ) * AOG) + (((Lanes([0.0, BLG[0], BLG[1], 0.0]) + Lanes([BLH[0], 0.0, BLH[1], BLH[2]])) * ANP) * AOB)) * UH) + Lanes([BLJ[0], 0.0, BLJ[1], BLJ[2]])) - (ASF * AOI)) / VS)) * NJ;
            let APB = NJ * ((KU * AOJ) * AOK);
            let BLR = ((Lanes([(AWE * AOJ), 0.0, 0.0]) + (ATW * KU)) * AOK) * NJ;
            let APC = NJ * (((KZ * UX) + (AOL * UV)) + (AOM * AEJ));
            let BLS = (((Lanes([(AWG * UX), 0.0, 0.0]) + (ARY * KZ)) + (BAE * AOL)) + (BFM * AOM)) * NJ;
            let APD = NJ * (AOM * AEL);
            let BLT = (BFN * AOM) * NJ;
            let APE = NJ * ((LB * AON) + (AOL * AOP));
            let BLU = (Lanes([BLK[0], 0.0, BLK[1], 0.0, BLK[2]]) + (ASJ * AOL)) * NJ;
            let APF = ddt(9699, APA);
            let BLW = BLQ * BLV;
            let APG = ddt(9701, APB);
            let BLX = BLR * BLV;
            let APH = ddt(9703, APC);
            let BLY = BLS * BLV;
            let API = ddt(9705, APD);
            let BLZ = BLT * BLV;
            let APJ = ddt(9707, APE);
            let BMA = BLU * BLV;
            let APK = ddt(9709, AOR);
            let BMB = BLL * BLV;
            let APL = ddt(9711, AOT);
            let BMC = BLM * BLV;
            let APM = ddt(9713, AOX);
            let BMD = BLO * BLV;
            let APN = ddt(9715, AOZ);
            let BME = BLP * BLV;
            let APO = ddt(9717, AOV);
            let BMF = BLN * BLV;
            let AQD;
            let AQE;
            let AQF;
            let AQG;
            let AQH;
            let AQI;
            let AQJ;
            let AQK;
            let AQL;
            let AQM;
            let AQN;
            let AQO;
            let AQP;
            if APP != 0.0 {
                AQD = APQ;
                AQE = APR;
                AQF = APS;
                AQG = APT;
                AQH = APU;
                AQI = APV;
                AQJ = APW;
                AQK = APX;
                AQL = APY;
                AQM = APZ;
                AQN = AQA;
                AQO = AQB;
                AQP = AQC;
            } else {
                AQD = A;
                AQE = A;
                AQF = A;
                AQG = A;
                AQH = A;
                AQI = A;
                AQJ = A;
                AQK = A;
                AQL = A;
                AQM = A;
                AQN = A;
                AQO = A;
                AQP = A;
            }
            let BMG = BIU[0];
            let BMH = BIU[1];
            let BMI = BIU[2];
            let BMJ = BIU[3];
            let BMK = BIV[0];
            let BML = BIV[1];
            let BMM = BIV[2];
            let BMN = BIV[3];
            let BMO = BIW;
            let BMP = BIX[0];
            let BMQ = BIX[1];
            let BMR = BIX[2];
            let BMS = BIX[3];
            let BMT = BIY[0];
            let BMU = BIY[1];
            let BMV = BIY[2];
            let BMW = BIY[3];
            let BMX = BIY[4];
            let BMY = BIZ[0];
            let BMZ = BIZ[1];
            let BNA = BIZ[2];
            let BNB = BIZ[3];
            let BNC = BJA[0];
            let BND = BJA[1];
            let BNE = BJA[2];
            let BNF = BFP[0];
            let BNG = BFP[1];
            let BNH = BFP[2];
            let BNI = BJB[0];
            let BNJ = BJB[1];
            let BNK = BJB[2];
            let BNL = BJB[3];
            let BNM = BFZ[0];
            let BNN = BFZ[1];
            let BNO = BFZ[2];
            let BNP = BGC[0];
            let BNQ = BGC[1];
            let BNR = BGC[2];
            let BNS = BGC[3];
            let BNT = BGC[4];
            let BNU = BGE[0];
            let BNV = BGE[1];
            let BNW = BGE[2];
            let BNX = BGH[0];
            let BNY = BGH[1];
            let BNZ = BGH[2];
            let BOA = BGH[3];
            let BOB = BGH[4];
            let BOC = BGH[5];
            let BOD = BIN[0];
            let BOE = BIN[1];
            let BOF = BIN[2];
            let BOG = BIN[3];
            let BOH = BIN[4];
            let BOI = BIO[0];
            let BOJ = BIO[1];
            let BOK = BIM;
            let BOL = BIL[0];
            let BOM = BIL[1];
            let BON = BIL[2];
            let BOO = BIL[3];
            let BOP = BIL[4];
            let BOQ = BIL[5];
            let BOR = BIL[6];
            let BOS = BIL[7];
            let BOT = BIL[8];
            let BOU = BIL[9];
            let BOV = BIL[10];
            let BOW = BLW[0];
            let BOX = BLW[1];
            let BOY = BLW[2];
            let BOZ = BLW[3];
            let BPA = BLX[0];
            let BPB = BLX[1];
            let BPC = BLX[2];
            let BPD = BLY[0];
            let BPE = BLY[1];
            let BPF = BLY[2];
            let BPG = BLZ[0];
            let BPH = BLZ[1];
            let BPI = BLZ[2];
            let BPJ = BMA[0];
            let BPK = BMA[1];
            let BPL = BMA[2];
            let BPM = BMA[3];
            let BPN = BMA[4];
            let BPO = BMB[0];
            let BPP = BMB[1];
            let BPQ = BMC[0];
            let BPR = BMC[1];
            let BPS = BMD;
            let BPT = BME;
            let BPU = BMF;
            let BPV = BLQ[0];
            let BPW = BLQ[1];
            let BPX = BLQ[2];
            let BPY = BLQ[3];
            let BPZ = BLR[0];
            let BQA = BLR[1];
            let BQB = BLR[2];
            let BQC = BLS[0];
            let BQD = BLS[1];
            let BQE = BLS[2];
            let BQF = BLT[0];
            let BQG = BLT[1];
            let BQH = BLT[2];
            let BQI = BLU[0];
            let BQJ = BLU[1];
            let BQK = BLU[2];
            let BQL = BLU[3];
            let BQM = BLU[4];
            let BQN = BLL[0];
            let BQO = BLL[1];
            let BQP = BLM[0];
            let BQQ = BLM[1];
            let BQR = BLO;
            let BQS = BLP;
            let BQT = BLN;
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(7),
            Some(8),
            multiplicity * (AII),
            [3, 5, 7, 8],
            [BMG, BMH, BMI, BMJ],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(6),
            Some(8),
            multiplicity * (AIJ),
            [3, 6, 7, 8],
            [BMK, BML, BMM, BMN],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(5),
            Some(8),
            multiplicity * (AIK),
            [11],
            [BMO],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(8),
            Some(5),
            multiplicity * (AIL),
            [3, 5, 7, 8],
            [BMP, BMQ, BMR, BMS],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(7),
            Some(5),
            multiplicity * (AIM),
            [3, 5, 7, 8, 11],
            [BMT, BMU, BMV, BMW, BMX],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(6),
            Some(4),
            multiplicity * (AIN),
            [0, 3, 4, 6],
            [BMY, BMZ, BNA, BNB],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(6),
            Some(9),
            multiplicity * (AIO),
            [3, 6, 9],
            [BNC, BND, BNE],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(0),
            Some(4),
            multiplicity * (AEM),
            [0, 3, 4],
            [BNF, BNG, BNH],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(4),
            Some(5),
            multiplicity * (AIP),
            [3, 4, 5, 7],
            [BNI, BNJ, BNK, BNL],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(1),
            Some(6),
            multiplicity * (AEZ),
            [1, 3, 6],
            [BNM, BNN, BNO],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(6),
            Some(7),
            multiplicity * (AFB),
            [3, 5, 6, 7, 8],
            [BNP, BNQ, BNR, BNS, BNT],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(2),
            Some(8),
            multiplicity * (AFC),
            [2, 3, 8],
            [BNU, BNV, BNW],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(9),
            Some(4),
            multiplicity * (AFG),
            [3, 4, 5, 6, 7, 9],
            [BNX, BNY, BNZ, BOA, BOB, BOC],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(10),
            None,
            multiplicity * (AIE),
            [3, 5, 7, 8, 11],
            [BOD, BOE, BOF, BOG, BOH],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(11),
            None,
            multiplicity * (AIF),
            [10, 11],
            [BOI, BOJ],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(3),
            None,
            multiplicity * (AID),
            [3],
            [BOK],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<11, 0>(
            Some(3),
            None,
            multiplicity * (AIC),
            [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 11],
            [BOL, BOM, BON, BOO, BOP, BOQ, BOR, BOS, BOT, BOU, BOV],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(7),
            Some(8),
            multiplicity * (APF),
            [3, 5, 7, 8],
            [BOW, BOX, BOY, BOZ],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(6),
            Some(8),
            multiplicity * (APG),
            [3, 6, 8],
            [BPA, BPB, BPC],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(7),
            Some(5),
            multiplicity * (APH),
            [3, 5, 7],
            [BPD, BPE, BPF],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(7),
            Some(4),
            multiplicity * (API),
            [3, 4, 7],
            [BPG, BPH, BPI],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(6),
            Some(9),
            multiplicity * (APJ),
            [3, 5, 6, 7, 9],
            [BPJ, BPK, BPL, BPM, BPN],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(1),
            Some(2),
            multiplicity * (APK),
            [1, 2],
            [BPO, BPP],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(1),
            Some(0),
            multiplicity * (APL),
            [0, 1],
            [BPQ, BPR],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(10),
            None,
            multiplicity * (APM),
            [10],
            [BPS],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(11),
            None,
            multiplicity * (APN),
            [11],
            [BPT],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(3),
            None,
            multiplicity * (APO),
            [3],
            [BPU],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(7),
            Some(8),
            multiplicity * (AQD),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(7),
            Some(8),
            multiplicity * (AQE),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(6),
            Some(8),
            multiplicity * (AQF),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(6),
            Some(8),
            multiplicity * (AQG),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(5),
            Some(8),
            multiplicity * (AQH),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(6),
            Some(9),
            multiplicity * (AQI),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(6),
            Some(9),
            multiplicity * (AQJ),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(0),
            Some(4),
            multiplicity * (AQK),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(4),
            Some(5),
            multiplicity * (AQL),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(1),
            Some(6),
            multiplicity * (AQM),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(6),
            Some(7),
            multiplicity * (AQN),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(2),
            Some(8),
            multiplicity * (AQO),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(9),
            Some(4),
            multiplicity * (AQP),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        self.canonical_reactive[0] = AII;
        self.canonical_reactive[1] = AIJ;
        self.canonical_reactive[2] = AIK;
        self.canonical_reactive[3] = AIL;
        self.canonical_reactive[4] = AIM;
        self.canonical_reactive[5] = AIN;
        self.canonical_reactive[6] = AIO;
        self.canonical_reactive[7] = AEM;
        self.canonical_reactive[8] = AIP;
        self.canonical_reactive[9] = AEZ;
        self.canonical_reactive[10] = AFB;
        self.canonical_reactive[11] = AFC;
        self.canonical_reactive[12] = AFG;
        self.canonical_reactive[13] = AIE;
        self.canonical_reactive[14] = AIF;
        self.canonical_reactive[15] = AID;
        self.canonical_reactive[16] = AIC;
        self.canonical_reactive[17] = APA;
        self.canonical_reactive[18] = BPV;
        self.canonical_reactive[19] = BPW;
        self.canonical_reactive[20] = BPX;
        self.canonical_reactive[21] = BPY;
        self.canonical_reactive[22] = APB;
        self.canonical_reactive[23] = BPZ;
        self.canonical_reactive[24] = BQA;
        self.canonical_reactive[25] = BQB;
        self.canonical_reactive[26] = APC;
        self.canonical_reactive[27] = BQC;
        self.canonical_reactive[28] = BQD;
        self.canonical_reactive[29] = BQE;
        self.canonical_reactive[30] = APD;
        self.canonical_reactive[31] = BQF;
        self.canonical_reactive[32] = BQG;
        self.canonical_reactive[33] = BQH;
        self.canonical_reactive[34] = APE;
        self.canonical_reactive[35] = BQI;
        self.canonical_reactive[36] = BQJ;
        self.canonical_reactive[37] = BQK;
        self.canonical_reactive[38] = BQL;
        self.canonical_reactive[39] = BQM;
        self.canonical_reactive[40] = AOR;
        self.canonical_reactive[41] = BQN;
        self.canonical_reactive[42] = BQO;
        self.canonical_reactive[43] = AOT;
        self.canonical_reactive[44] = BQP;
        self.canonical_reactive[45] = BQQ;
        self.canonical_reactive[46] = AOX;
        self.canonical_reactive[47] = BQR;
        self.canonical_reactive[48] = AOZ;
        self.canonical_reactive[49] = BQS;
        self.canonical_reactive[50] = AOV;
        self.canonical_reactive[51] = BQT;
        self.canonical_reactive[52] = AQD;
        self.canonical_reactive[53] = AQE;
        self.canonical_reactive[54] = AQF;
        self.canonical_reactive[55] = AQG;
        self.canonical_reactive[56] = AQH;
        self.canonical_reactive[57] = AQI;
        self.canonical_reactive[58] = AQJ;
        self.canonical_reactive[59] = AQK;
        self.canonical_reactive[60] = AQL;
        self.canonical_reactive[61] = AQM;
        self.canonical_reactive[62] = AQN;
        self.canonical_reactive[63] = AQO;
        self.canonical_reactive[64] = AQP;
    }

    pub fn stamp_reactive(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedReactiveStamper<'_>) {
        let multiplicity = self.multiplicity;
        let cached = &*self.canonical_reactive;
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(7),
            Some(8),
            &[3, 5, 7, 8],
            &[cached[18], cached[19], cached[20], cached[21]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(6),
            Some(8),
            &[3, 6, 8],
            &[cached[23], cached[24], cached[25]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(7),
            Some(5),
            &[3, 5, 7],
            &[cached[27], cached[28], cached[29]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(7),
            Some(4),
            &[3, 4, 7],
            &[cached[31], cached[32], cached[33]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(6),
            Some(9),
            &[3, 5, 6, 7, 9],
            &[cached[35], cached[36], cached[37], cached[38], cached[39]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(1),
            Some(2),
            &[1, 2],
            &[cached[41], cached[42]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(1),
            Some(0),
            &[0, 1],
            &[cached[44], cached[45]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(10),
            None,
            &[10],
            &[cached[47]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(11),
            None,
            &[11],
            &[cached[49]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(3),
            None,
            &[3],
            &[cached[51]],
            &[],
            &[],
            multiplicity,
        );
    }

}
