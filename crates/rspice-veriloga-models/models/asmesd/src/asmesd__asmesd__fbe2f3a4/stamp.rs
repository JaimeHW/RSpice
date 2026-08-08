#![allow(dead_code, non_snake_case, unused_imports, unused_mut, unused_parens, unused_variables)]

use super::state::Instance;
use rspice_veriloga_runtime::{GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, Lanes, rspice_eval_ddt, rspice_eval_idt, rspice_limexp, rspice_limited_exp, rspice_limited_exp_derivative};
impl Instance {
    pub fn stamp(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedStamper<'_>) {
        let parameters = &self.params.values;
        let multiplicity = self.multiplicity;
        let temperature = ctx.temperature();
        let node_potentials = [ctx.node_voltage(self.nodes[0]), ctx.node_voltage(self.nodes[1]), ctx.node_voltage(self.nodes[2]), ctx.node_voltage(self.nodes[3]), ctx.node_voltage(self.nodes[4]), ctx.node_voltage(self.nodes[5]), ctx.node_voltage(self.nodes[6]), ctx.node_voltage(self.nodes[7]), ctx.node_voltage(self.nodes[8]), ctx.node_voltage(self.nodes[9])];
        let ddt_scale_value = self.ddt_coefficients.derivative_scale;
        let ddt_scale = move || ddt_scale_value;
        let ddt_state = self.stamp_state.as_mut();
        let ddt_active = self.ddt_coefficients.active;
        let ddt_coefficients = self.ddt_coefficients;
        let mut ddt = |operator: usize, value: f64| -> f64 {
            let _ = operator;
            let slot = match operator { 3355 => 0usize, 3525 => 1usize, 3871 => 2usize, 3902 => 3usize, 3909 => 4usize, 4101 => 5usize, 4107 => 6usize, 4113 => 7usize, 4119 => 8usize, 4125 => 9usize, 4131 => 10usize, 4136 => 11usize, 4140 => 12usize, _ => usize::MAX };
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
            let B = node_potentials[3];
            let E = 1.7314999999999998e2f64;
            let H = 1.3e3f64;
            let J = 1.7314999999999998e2f64;
            let M = 1e0f64;
            let N = 0.0f64;
            let Q = parameters[29];
            let R = node_potentials[5];
            let S = node_potentials[4];
            let V = parameters[79];
            let X = parameters[80];
            let AA = 8.6170869e-5f64;
            let AE = parameters[77];
            let AG = parameters[52];
            let AJ = parameters[60];
            let AL = parameters[53];
            let AP = parameters[62];
            let AT = parameters[54];
            let AX = parameters[63];
            let BB = parameters[22];
            let BC = parameters[21];
            let BG = parameters[23];
            let BH = parameters[0];
            let BK = parameters[2];
            let BN = parameters[58];
            let BO = parameters[59];
            let BR = parameters[64];
            let BS = parameters[65];
            let BV = parameters[47];
            let BW = parameters[7];
            let BY = parameters[5];
            let BZ = parameters[6];
            let CB = parameters[9];
            let CC = parameters[10];
            let CE = parameters[56];
            let CF = parameters[55];
            let CH = 3.0015e2f64;
            let CK = 7.02e-4f64;
            let CO = 1.3806226e-23f64;
            let CS = 1.5e0f64;
            let CU = 1.6021918e-19f64;
            let CX = parameters[17];
            let DA = parameters[18];
            let DB = 4e-4f64;
            let DM = parameters[70];
            let DP = parameters[71];
            let DY = parameters[75];
            let EB = parameters[76];
            let EI = node_potentials[2];
            let EK = node_potentials[6];
            let EN = node_potentials[1];
            let EU = parameters[1];
            let EX = parameters[11];
            let FB = 8e1f64;
            let FI = 3.7e1f64;
            let GB = parameters[8];
            let GI = parameters[4];
            let GK = 1e-3f64;
            let GM = -1e0f64;
            let GN = parameters[3];
            let HB = parameters[57];
            let IH = parameters[61];
            let KX = node_potentials[9];
            let KZ = 1e-6f64;
            let LB = parameters[83];
            let LE = 1e-9f64;
            let LR = parameters[81];
            let LT = 4e0f64;
            let LW = parameters[82];
            let LY = 2e0f64;
            let MC = parameters[84];
            let MG = parameters[48];
            let MJ = parameters[49];
            let ML = parameters[50];
            let MO = parameters[51];
            let MQ = parameters[12];
            let MR = parameters[37];
            let MX = parameters[66];
            let MY = parameters[78];
            let NB = parameters[14];
            let NC = parameters[38];
            let NJ = parameters[40];
            let NM = parameters[39];
            let NP = parameters[19];
            let NQ = parameters[41];
            let NT = parameters[73];
            let NZ = node_potentials[8];
            let OC = parameters[20];
            let OE = parameters[44];
            let OH = 0e0f64;
            let OI = parameters[31];
            let OL = parameters[13];
            let ON = parameters[67];
            let OP = parameters[15];
            let PA = 5e-1f64;
            let PF = parameters[24];
            let QW = parameters[72];
            let RU = parameters[68];
            let RY = parameters[30];
            let RZ = parameters[33];
            let SC = -1e0f64;
            let SE = node_potentials[0];
            let SJ = parameters[34];
            let SM = 0e0f64;
            let SN = parameters[35];
            let SP = -1e0f64;
            let SU = node_potentials[7];
            let SZ = parameters[36];
            let TD = -1e0f64;
            let TI = 0e0f64;
            let TJ = 0e0f64;
            let TK = 0e0f64;
            let TU = parameters[46];
            let UC = 0e0f64;
            let UD = 0e0f64;
            let UL = 0e0f64;
            let UM = 0e0f64;
            let UV = 0e0f64;
            let UW = 0e0f64;
            let VV = 0e0f64;
            let VX = 0e0f64;
            let VY = 0e0f64;
            let XU = 1e0f64;
            let XV = 1e0f64;
            let XW = 1e0f64;
            let XX = 1e0f64;
            let XY = 1e0f64;
            let XZ = 1e0f64;
            let YA = 1e0f64;
            let YB = 1e0f64;
            let YC = 1e0f64;
            let YD = 1e0f64;
            let YE = 1e0f64;
            let ABD = 0e0f64;
            let ABG = -1e0f64;
            let ACX = Lanes([0e0f64; 3]);
            let ADI = 0e0f64;
            let ADJ = 2e0f64;
            let AEC = Lanes([0e0f64; 3]);
            let AFF = ddt_scale();
            let AGP = Lanes([0e0f64; 6]);
            let AGQ = 0e0f64;
            let AGR = Lanes([0e0f64; 3]);
            let AHP = Lanes([0e0f64; 3]);
            let AHZ = Lanes([0e0f64; 4]);
            let AIB = Lanes([0e0f64; 7]);
            let AIJ = Lanes([0e0f64; 2]);
            let AIK = 0e0f64;
            let AJL = Lanes([0e0f64; 4]);
            let AJO = Lanes([0e0f64; 3]);
            let AJR = Lanes([0e0f64; 3]);
            let C = (temperature + B) + parameters[45];
            let D = if C > 1.7314999999999998e2f64 { 1.0 } else { 0.0 };
            let F = if D != 0.0 {
                C
            } else {
                E
            };
            let G = if 1.3e3f64 < F { 1.0 } else { 0.0 };
            let L;
            let YF;
            if G != 0.0 {
                L = H;
                YF = ABD;
            } else {
                let I = if C > 1.7314999999999998e2f64 { 1.0 } else { 0.0 };
                let K;
                let YG;
                if I != 0.0 {
                    K = C;
                    YG = XV;
                } else {
                    K = J;
                    YG = ABD;
                }
                L = K;
                YF = YG;
            }
            if N != 0.0 {
            } else {
            }
            let O = if L > parameters[26] { 1.0 } else { 0.0 };
            if O != 0.0 {
            } else {
            }
            let P = parameters[43] * parameters[42];
            let T = R - S;
            let ABE = Lanes([0.0, XW]) - Lanes([XX, 0.0]);
            let U = Q * T;
            let ABF = ABE * Q;
            let W = -(if U <= A { U } else { A });
            let Y = M + (V * (W.powf(X)));
            let Z = parameters[25] + 2.7315e2f64;
            let AB = AA * L;
            let ABH = YF * AA;
            let AC = L / Z;
            let ABI = YF / Z;
            let AD = AC.ln();
            let ABJ = ABI * (XU / AC);
            let AF = (AE * AD).exp();
            let ABK = (ABJ * AE) * AF;
            let AH = AG * AF;
            let AI = AH * Y;
            let ABL = ((((ABF * (if U <= A { 1.0 } else { 0.0 })) * ABG) * (X * (W.powf((X - XU))))) * V) * AH;
            let ABM = Lanes([((ABK * AG) * Y), 0.0, 0.0]) + Lanes([0.0, ABL[0], ABL[1]]);
            let AK = AJ * AF;
            let ABN = ABK * AJ;
            let AM = if AL > A { 1.0 } else { 0.0 };
            let AO = if AM != 0.0 {
                let AN = M / AL;
                AN
            } else {
                A
            };
            let AQ = if AP > A { 1.0 } else { 0.0 };
            let AS = if AQ != 0.0 {
                let AR = M / AP;
                AR
            } else {
                A
            };
            let AU = if AT > A { 1.0 } else { 0.0 };
            let AW = if AU != 0.0 {
                let AV = M / AT;
                AV
            } else {
                A
            };
            let AY = if AX > A { 1.0 } else { 0.0 };
            let BA = if AY != 0.0 {
                let AZ = M / AX;
                AZ
            } else {
                A
            };
            let BD = AC - M;
            let BE = (BC * BD) / AB;
            let BF = (BB * AD) + BE;
            let ABO = (ABJ * BB) + (((ABI * BC) - (ABH * BE)) / AB);
            let BI = BF.exp();
            let BJ = BH * BI;
            let ABP = (ABO * BI) * BH;
            let BL = (BG * AD).exp();
            let BM = BK * BL;
            let ABQ = ((ABJ * BG) * BL) * BK;
            let BP = (BF / BO).exp();
            let BQ = (BN * BP) / AF;
            let ABR = ((((ABO / BO) * BP) * BN) - (ABK * BQ)) / AF;
            let BT = (BF / BS).exp();
            let BU = (BR * BT) / AF;
            let ABS = ((((ABO / BS) * BT) * BR) - (ABK * BU)) / AF;
            let BX = BV * (M + (BW * BD));
            let ABT = (ABI * BW) * BV;
            let CA = BY * (M + (BZ * BD));
            let ABU = (ABI * BZ) * BY;
            let CD = CB * (M + (CC * BD));
            let ABV = (ABI * CC) * CB;
            let CG = CE * (M + (CF * BD));
            let ABW = (ABI * CF) * CE;
            let CI = Z / CH;
            let CJ = L / CH;
            let ABX = YF / CH;
            let CL = CK * L;
            let CM = 1.108e3f64 + L;
            let CN = (CL * L) / CM;
            let ABY = ((((((YF * CK) * L) + (YF * CL)) - (YF * CN)) / CM) * ABG) * ABG;
            let CP = CO * (L + L);
            let CQ = (-(1.16e0f64 - CN)) / CP;
            let ABZ = ((YF + YF) * CO) * CQ;
            let CR = -(AB + AB);
            let ACA = (ABH + ABH) * ABG;
            let CT = CS * (CJ.ln());
            let ACB = (ABX * (XU / CJ)) * CS;
            let CV = CT + (CU * (CQ + 1.3454442398941469e20f64));
            let CW = CR * CV;
            let ACC = (ACA * CV) + ((ACB + (((ABY - ABZ) / CP) * CU)) * CR);
            let CY = (CX - CW) / CI;
            let ACD = (ACC * ABG) / CI;
            let CZ = (CX - CY) / CY;
            let DC = DB * (Z - CH);
            let DD = M + (DA * (DC - CZ));
            let DE = parameters[16] / DD;
            let DF = (CJ * CY) + CW;
            let ACE = ((ABX * CY) + (ACD * CJ)) + ACC;
            let DG = (DF - CY) / CY;
            let DH = DB * (L - CH);
            let ACF = YF * DB;
            let DI = M + (DA * (DH - DG));
            let DJ = DE * DI;
            let ACG = (((((((((ACD * ABG) - (ACD * CZ)) / CY) * ABG) * DA) * DE) * ABG) / DD) * DI) + (((ACF - (((ACE - ACD) - (ACD * DG)) / CY)) * DA) * DE);
            let DK = CT + (CU * (CQ + 1.3454442398941469e20f64));
            let DL = CR * DK;
            let ACH = (ACA * DK) + ((ACB + (((ABY - ABZ) / CP) * CU)) * CR);
            let DN = (DM - DL) / CI;
            let ACI = (ACH * ABG) / CI;
            let DO = (DM - DN) / DN;
            let DQ = M + (DP * (DC - DO));
            let DR = parameters[69] / DQ;
            let DS = (CJ * DN) + DL;
            let ACJ = ((ABX * DN) + (ACI * CJ)) + ACH;
            let DT = (DS - DN) / DN;
            let DU = M + (DP * (DH - DT));
            let DV = DR * DU;
            let ACK = (((((((((ACI * ABG) - (ACI * DO)) / DN) * ABG) * DP) * DR) * ABG) / DQ) * DU) + (((ACF - (((ACJ - ACI) - (ACI * DT)) / DN)) * DP) * DR);
            let DW = CT + (CU * (CQ + 1.3454442398941469e20f64));
            let DX = CR * DW;
            let ACL = (ACA * DW) + ((ACB + (((ABY - ABZ) / CP) * CU)) * CR);
            let DZ = (DY - DX) / CI;
            let ACM = (ACL * ABG) / CI;
            let EA = (DY - DZ) / DZ;
            let EC = M + (EB * (DC - EA));
            let ED = parameters[74] / EC;
            let EE = (CJ * DZ) + DX;
            let ACN = ((ABX * DZ) + (ACM * CJ)) + ACL;
            let EF = (EE - DZ) / DZ;
            let EG = M + (EB * (DH - EF));
            let EH = ED * EG;
            let ACO = (((((((((ACM * ABG) - (ACM * EA)) / DZ) * ABG) * EB) * ED) * ABG) / EC) * EG) + (((ACF - (((ACN - ACM) - (ACM * EF)) / DZ)) * EB) * ED);
            let EJ = Q * (EI - S);
            let ACP = (Lanes([XY, 0.0]) - Lanes([0.0, XX])) * Q;
            let EL = R - EK;
            let ACQ = Lanes([XW, 0.0]) - Lanes([0.0, XZ]);
            let EM = Q * EL;
            let ACR = ACQ * Q;
            let EO = Q * (EN - S);
            let ACS = (Lanes([YA, 0.0]) - Lanes([0.0, XX])) * Q;
            let EP = EN - R;
            let ACT = Lanes([YA, 0.0]) - Lanes([0.0, XW]);
            let EQ = Q * EP;
            let ACU = ACT * Q;
            let ER = EI - EK;
            let ACV = Lanes([XY, 0.0]) - Lanes([0.0, XZ]);
            let ES = Q * ER;
            let ACW = ACV * Q;
            let ET = if BJ > A { 1.0 } else { 0.0 };
            let LI;
            let YH;
            if ET != 0.0 {
                let EV = EU * AB;
                let EW = EM / EV;
                let ACY = (Lanes([0.0, ACR[0], ACR[1]]) - Lanes([((ABH * EU) * EW), 0.0, 0.0])) / EV;
                let ACZ = ACR * ABG;
                let EY = EX * AB;
                let ADA = ABH * EX;
                let EZ = ((-EM) - CA) / EY;
                let ADB = ((Lanes([0.0, ACZ[0], ACZ[1]]) - Lanes([ABU, 0.0, 0.0])) - Lanes([(ADA * EZ), 0.0, 0.0])) / EY;
                let FA = (-CA) / EY;
                let ADC = ((ABU * ABG) - (ADA * FA)) / EY;
                let FC = if EW > FB { 1.0 } else { 0.0 };
                let FE;
                let FF;
                let YI;
                let YJ;
                if FC != 0.0 {
                    let FD = M + (EW - FB);
                    FE = FD;
                    FF = FB;
                    YI = ACY;
                    YJ = ACX;
                } else {
                    FE = M;
                    FF = EW;
                    YI = ACX;
                    YJ = ACY;
                }
                let FG = FF.exp();
                let FH = FE * FG;
                let ADD = (YI * FG) + ((YJ * FG) * FE);
                let FJ = if EZ >= FI { 1.0 } else { 0.0 };
                let FV;
                let YK;
                if FJ != 0.0 {
                    FV = EZ;
                    YK = ADB;
                } else {
                    let FK = if EZ <= -3.7e1f64 { 1.0 } else { 0.0 };
                    let FW;
                    let YL;
                    if FK != 0.0 {
                        let FL = EZ.exp();
                        let ADF = ADB * FL;
                        FW = FL;
                        YL = ADF;
                    } else {
                        let FM = EZ.exp();
                        let FN = FM + M;
                        let FO = FN.ln();
                        let ADE = (ADB * FM) * (XU / FN);
                        FW = FO;
                        YL = ADE;
                    }
                    FV = FW;
                    YK = YL;
                }
                let FP = if FA >= FI { 1.0 } else { 0.0 };
                let FX;
                let YM;
                if FP != 0.0 {
                    FX = FA;
                    YM = ADC;
                } else {
                    let FQ = if FA <= -3.7e1f64 { 1.0 } else { 0.0 };
                    let FY;
                    let YN;
                    if FQ != 0.0 {
                        let FR = FA.exp();
                        let ADH = ADC * FR;
                        FY = FR;
                        YN = ADH;
                    } else {
                        let FS = FA.exp();
                        let FT = FS + M;
                        let FU = FT.ln();
                        let ADG = (ADC * FS) * (XU / FT);
                        FY = FU;
                        YN = ADG;
                    }
                    FX = FY;
                    YM = YN;
                }
                let FZ = FV - FX;
                let GA = FH - M;
                let GC = EM.abs();
                let GD = GC.powf(CD);
                let ADK = (ACR * ((ADJ * (if EM >= ADI { 1.0 } else { 0.0 })) - XU)) * (CD * (GC.powf((CD - XU))));
                let GE = M + (GB * GD);
                let GF = (BX * FZ) / GE;
                let GG = (BJ * GA) - GF;
                let ADL = (Lanes([(ABP * GA), 0.0, 0.0]) + (ADD * BJ)) - (((Lanes([(ABT * FZ), 0.0, 0.0]) + ((YK - Lanes([YM, 0.0, 0.0])) * BX)) - (((Lanes([0.0, ADK[0], ADK[1]]) + Lanes([(ABV * (GD * (GC.ln()))), 0.0, 0.0])) * GB) * GF)) / GE);
                LI = GG;
                YH = ADL;
            } else {
                LI = A;
                YH = ACX;
            }
            let GH = if BM > A { 1.0 } else { 0.0 };
            let LJ;
            let YO;
            if GH != 0.0 {
                let GJ = GI - EM;
                let GL = if GJ >= GK { GJ } else { GK };
                let ADM = (ACR * GM) * GI;
                let GO = GN * AB;
                let GP = GO * GL;
                let ADN = ((ACR * ABG) * (if GJ >= GK { 1.0 } else { 0.0 })) * GO;
                let GQ = ((GM * EM) * GI) / GP;
                let ADO = (Lanes([0.0, ADM[0], ADM[1]]) - ((Lanes([((ABH * GN) * GL), 0.0, 0.0]) + Lanes([0.0, ADN[0], ADN[1]])) * GQ)) / GP;
                let GR = if GQ > FB { 1.0 } else { 0.0 };
                let GT;
                let GU;
                let YP;
                let YQ;
                if GR != 0.0 {
                    let GS = M + (GQ - FB);
                    GT = GS;
                    GU = FB;
                    YP = ADO;
                    YQ = ACX;
                } else {
                    GT = M;
                    GU = GQ;
                    YP = ACX;
                    YQ = ADO;
                }
                let GV = GU.exp();
                let GW = (GT * GV) - M;
                let GX = BM * GW;
                let ADP = Lanes([(ABQ * GW), 0.0, 0.0]) + (((YP * GV) + ((YQ * GV) * GT)) * BM);
                LJ = GX;
                YO = ADP;
            } else {
                LJ = A;
                YO = ACX;
            }
            let GY = if BQ > A { 1.0 } else { 0.0 };
            let LL;
            let YR;
            if GY != 0.0 {
                let GZ = BO * AB;
                let HA = EM / GZ;
                let ADQ = (Lanes([0.0, ACR[0], ACR[1]]) - Lanes([((ABH * BO) * HA), 0.0, 0.0])) / GZ;
                let ADR = ACR * ABG;
                let HC = HB * AB;
                let ADS = ABH * HB;
                let HD = ((-EM) - CA) / HC;
                let ADT = ((Lanes([0.0, ADR[0], ADR[1]]) - Lanes([ABU, 0.0, 0.0])) - Lanes([(ADS * HD), 0.0, 0.0])) / HC;
                let HE = (-CA) / HC;
                let ADU = ((ABU * ABG) - (ADS * HE)) / HC;
                let HF = if HA > FB { 1.0 } else { 0.0 };
                let HH;
                let HI;
                let YS;
                let YT;
                if HF != 0.0 {
                    let HG = M + (HA - FB);
                    HH = HG;
                    HI = FB;
                    YS = ADQ;
                    YT = ACX;
                } else {
                    HH = M;
                    HI = HA;
                    YS = ACX;
                    YT = ADQ;
                }
                let HJ = HI.exp();
                let HK = HH * HJ;
                let ADV = (YS * HJ) + ((YT * HJ) * HH);
                let HL = if HD >= FI { 1.0 } else { 0.0 };
                let HX;
                let YU;
                if HL != 0.0 {
                    HX = HD;
                    YU = ADT;
                } else {
                    let HM = if HD <= -3.7e1f64 { 1.0 } else { 0.0 };
                    let HY;
                    let YV;
                    if HM != 0.0 {
                        let HN = HD.exp();
                        let ADX = ADT * HN;
                        HY = HN;
                        YV = ADX;
                    } else {
                        let HO = HD.exp();
                        let HP = HO + M;
                        let HQ = HP.ln();
                        let ADW = (ADT * HO) * (XU / HP);
                        HY = HQ;
                        YV = ADW;
                    }
                    HX = HY;
                    YU = YV;
                }
                let HR = if HE >= FI { 1.0 } else { 0.0 };
                let HZ;
                let YW;
                if HR != 0.0 {
                    HZ = HE;
                    YW = ADU;
                } else {
                    let HS = if HE <= -3.7e1f64 { 1.0 } else { 0.0 };
                    let IA;
                    let YX;
                    if HS != 0.0 {
                        let HT = HE.exp();
                        let ADZ = ADU * HT;
                        IA = HT;
                        YX = ADZ;
                    } else {
                        let HU = HE.exp();
                        let HV = HU + M;
                        let HW = HV.ln();
                        let ADY = (ADU * HU) * (XU / HV);
                        IA = HW;
                        YX = ADY;
                    }
                    HZ = IA;
                    YW = YX;
                }
                let IB = HK - M;
                let IC = EM.abs();
                let ID = IC.powf(CD);
                let AEA = (ACR * ((ADJ * (if EM >= ADI { 1.0 } else { 0.0 })) - XU)) * (CD * (IC.powf((CD - XU))));
                let IE = M + (GB * ID);
                let IF = (A * (HX - HZ)) / IE;
                let IG = (BQ * IB) - IF;
                let AEB = (Lanes([(ABR * IB), 0.0, 0.0]) + (ADV * BQ)) - ((((YU - Lanes([YW, 0.0, 0.0])) * A) - (((Lanes([0.0, AEA[0], AEA[1]]) + Lanes([(ABV * (ID * (IC.ln()))), 0.0, 0.0])) * GB) * IF)) / IE);
                LL = IG;
                YR = AEB;
            } else {
                LL = A;
                YR = ACX;
            }
            let LN;
            let YY;
            if ET != 0.0 {
                let II = IH * AB;
                let IJ = U / II;
                let AED = (Lanes([0.0, ABF[0], ABF[1]]) - Lanes([((ABH * IH) * IJ), 0.0, 0.0])) / II;
                let AEE = ABF * ABG;
                let IK = HB * AB;
                let AEF = ABH * HB;
                let IL = ((-U) - CA) / IK;
                let AEG = ((Lanes([0.0, AEE[0], AEE[1]]) - Lanes([ABU, 0.0, 0.0])) - Lanes([(AEF * IL), 0.0, 0.0])) / IK;
                let IM = (-CA) / IK;
                let AEH = ((ABU * ABG) - (AEF * IM)) / IK;
                let IN = if IJ > FB { 1.0 } else { 0.0 };
                let IP;
                let IQ;
                let YZ;
                let ZA;
                if IN != 0.0 {
                    let IO = M + (IJ - FB);
                    IP = IO;
                    IQ = FB;
                    YZ = AED;
                    ZA = AEC;
                } else {
                    IP = M;
                    IQ = IJ;
                    YZ = AEC;
                    ZA = AED;
                }
                let IR = IQ.exp();
                let IS = IP * IR;
                let AEI = (YZ * IR) + ((ZA * IR) * IP);
                let IT = if IL >= FI { 1.0 } else { 0.0 };
                let JF;
                let ZB;
                if IT != 0.0 {
                    JF = IL;
                    ZB = AEG;
                } else {
                    let IU = if IL <= -3.7e1f64 { 1.0 } else { 0.0 };
                    let JG;
                    let ZC;
                    if IU != 0.0 {
                        let IV = IL.exp();
                        let AEK = AEG * IV;
                        JG = IV;
                        ZC = AEK;
                    } else {
                        let IW = IL.exp();
                        let IX = IW + M;
                        let IY = IX.ln();
                        let AEJ = (AEG * IW) * (XU / IX);
                        JG = IY;
                        ZC = AEJ;
                    }
                    JF = JG;
                    ZB = ZC;
                }
                let IZ = if IM >= FI { 1.0 } else { 0.0 };
                let JH;
                let ZD;
                if IZ != 0.0 {
                    JH = IM;
                    ZD = AEH;
                } else {
                    let JA = if IM <= -3.7e1f64 { 1.0 } else { 0.0 };
                    let JI;
                    let ZE;
                    if JA != 0.0 {
                        let JB = IM.exp();
                        let AEM = AEH * JB;
                        JI = JB;
                        ZE = AEM;
                    } else {
                        let JC = IM.exp();
                        let JD = JC + M;
                        let JE = JD.ln();
                        let AEL = (AEH * JC) * (XU / JD);
                        JI = JE;
                        ZE = AEL;
                    }
                    JH = JI;
                    ZD = ZE;
                }
                let JJ = JF - JH;
                let JK = IS - M;
                let JL = U.abs();
                let JM = JL.powf(CD);
                let AEN = (ABF * ((ADJ * (if U >= ADI { 1.0 } else { 0.0 })) - XU)) * (CD * (JL.powf((CD - XU))));
                let JN = M + (GB * JM);
                let JO = (CG * JJ) / JN;
                let JP = (BJ * JK) - JO;
                let AEO = (Lanes([(ABP * JK), 0.0, 0.0]) + (AEI * BJ)) - (((Lanes([(ABW * JJ), 0.0, 0.0]) + ((ZB - Lanes([ZD, 0.0, 0.0])) * CG)) - (((Lanes([0.0, AEN[0], AEN[1]]) + Lanes([(ABV * (JM * (JL.ln()))), 0.0, 0.0])) * GB) * JO)) / JN);
                LN = JP;
                YY = AEO;
            } else {
                LN = A;
                YY = AEC;
            }
            let JQ = if BU > A { 1.0 } else { 0.0 };
            let LP;
            let ZF;
            if JQ != 0.0 {
                let JR = BS * AB;
                let JS = U / JR;
                let AEP = (Lanes([0.0, ABF[0], ABF[1]]) - Lanes([((ABH * BS) * JS), 0.0, 0.0])) / JR;
                let AEQ = ABF * ABG;
                let JT = HB * AB;
                let AER = ABH * HB;
                let JU = ((-U) - CA) / JT;
                let AES = ((Lanes([0.0, AEQ[0], AEQ[1]]) - Lanes([ABU, 0.0, 0.0])) - Lanes([(AER * JU), 0.0, 0.0])) / JT;
                let JV = (-CA) / JT;
                let AET = ((ABU * ABG) - (AER * JV)) / JT;
                let JW = if JS > FB { 1.0 } else { 0.0 };
                let JY;
                let JZ;
                let ZG;
                let ZH;
                if JW != 0.0 {
                    let JX = M + (JS - FB);
                    JY = JX;
                    JZ = FB;
                    ZG = AEP;
                    ZH = AEC;
                } else {
                    JY = M;
                    JZ = JS;
                    ZG = AEC;
                    ZH = AEP;
                }
                let KA = JZ.exp();
                let KB = JY * KA;
                let AEU = (ZG * KA) + ((ZH * KA) * JY);
                let KC = if JU >= FI { 1.0 } else { 0.0 };
                let KO;
                let ZI;
                if KC != 0.0 {
                    KO = JU;
                    ZI = AES;
                } else {
                    let KD = if JU <= -3.7e1f64 { 1.0 } else { 0.0 };
                    let KP;
                    let ZJ;
                    if KD != 0.0 {
                        let KE = JU.exp();
                        let AEW = AES * KE;
                        KP = KE;
                        ZJ = AEW;
                    } else {
                        let KF = JU.exp();
                        let KG = KF + M;
                        let KH = KG.ln();
                        let AEV = (AES * KF) * (XU / KG);
                        KP = KH;
                        ZJ = AEV;
                    }
                    KO = KP;
                    ZI = ZJ;
                }
                let KI = if JV >= FI { 1.0 } else { 0.0 };
                let KQ;
                let ZK;
                if KI != 0.0 {
                    KQ = JV;
                    ZK = AET;
                } else {
                    let KJ = if JV <= -3.7e1f64 { 1.0 } else { 0.0 };
                    let KR;
                    let ZL;
                    if KJ != 0.0 {
                        let KK = JV.exp();
                        let AEY = AET * KK;
                        KR = KK;
                        ZL = AEY;
                    } else {
                        let KL = JV.exp();
                        let KM = KL + M;
                        let KN = KM.ln();
                        let AEX = (AET * KL) * (XU / KM);
                        KR = KN;
                        ZL = AEX;
                    }
                    KQ = KR;
                    ZK = ZL;
                }
                let KS = KB - M;
                let KT = U.abs();
                let KU = M + (GB * (KT.powf(CB)));
                let KV = (A * (KO - KQ)) / KU;
                let AEZ = (((ABF * ((ADJ * (if U >= ADI { 1.0 } else { 0.0 })) - XU)) * (CB * (KT.powf((CB - XU))))) * GB) * KV;
                let KW = (BU * KS) - KV;
                let AFA = (Lanes([(ABS * KS), 0.0, 0.0]) + (AEU * BU)) - ((((ZI - Lanes([ZK, 0.0, 0.0])) * A) - Lanes([0.0, AEZ[0], AEZ[1]])) / KU);
                LP = KW;
                ZF = AFA;
            } else {
                LP = A;
                ZF = AEC;
            }
            let AFB = Lanes([ACR[0], ACR[1], 0.0]);
            let AFC = Lanes([0.0, 0.0, YB]);
            let KY = -(EM - KX);
            let AFD = (AFB - AFC) * ABG;
            let LA = KX * KZ;
            let AFE = YB * KZ;
            let LC = LB * ddt(3355, KX);
            let AFG = (YB * AFF) * LB;
            let XM = LB * KX;
            let AFH = YB * LB;
            let LD = EM.abs();
            let LF = if LD >= LE { LD } else { LE };
            let LG = (if KX <= EM { KX } else { EM }) / LF;
            let AFI = ((ACR * ((ADJ * (if EM >= ADI { 1.0 } else { 0.0 })) - XU)) * (if LD >= LE { 1.0 } else { 0.0 })) * LG;
            let LH = LG.abs();
            let AFJ = YH - YO;
            let LK = (LI - LJ) / AI;
            let AFK = ABM * LK;
            let LM = LK + LL;
            let AFL = ((Lanes([AFJ[0], 0.0, AFJ[1], AFJ[2]]) - Lanes([AFK[0], AFK[1], AFK[2], 0.0])) / AI) + Lanes([YR[0], 0.0, YR[1], YR[2]]);
            let LO = LN / AK;
            let LQ = LO + LP;
            let AFM = ((YY - Lanes([(ABN * LO), 0.0, 0.0])) / AK) + ZF;
            let LS = AW * (M + (U * LR));
            let AFN = YH * LS;
            let AFO = ((ABF * LR) * AW) * LI;
            let AFP = YY * BA;
            let AFQ = (ACR * AS) * ABG;
            let AFR = ABF * AO;
            let LU = M + (LT * ((LI * LS) + (LN * BA)));
            let LV = LU.abs();
            let LX = M + (LV.powf(LW));
            let AFS = (Lanes([0.0, AFQ[0], AFQ[1]]) - Lanes([AFR[0], AFR[1], 0.0])) * LY;
            let LZ = (LY * ((M - (EM * AS)) - (U * AO))) / LX;
            let AFT = (Lanes([0.0, AFS[0], AFS[1], AFS[2]]) - ((((((Lanes([AFN[0], 0.0, AFN[1], AFN[2]]) + Lanes([0.0, AFO[0], AFO[1], 0.0])) + Lanes([AFP[0], AFP[1], AFP[2], 0.0])) * LT) * ((ADJ * (if LU >= ADI { 1.0 } else { 0.0 })) - XU)) * (LW * (LV.powf((LW - XU))))) * LZ)) / LX;
            let MA = LN * LZ;
            let AFU = YY * LZ;
            let AFV = Lanes([AFU[0], AFU[1], AFU[2], 0.0]) + (AFT * LN);
            let MB = LI * LZ;
            let AFW = YH * LZ;
            let AFX = Lanes([AFW[0], 0.0, AFW[1], AFW[2]]) + (AFT * LI);
            let AFY = AFX * LH;
            let AFZ = ((((AFB + ((AFC - AFB) * (if KX <= EM { 1.0 } else { 0.0 }))) - Lanes([AFI[0], AFI[1], 0.0])) / LF) * ((ADJ * (if LG >= ADI { 1.0 } else { 0.0 })) - XU)) * MB;
            let MD = M - MC;
            let ME = MD * LI;
            let AGA = (YH * MD) * LZ;
            let AGB = Lanes([AGA[0], 0.0, AGA[1], AGA[2]]) + (AFT * ME);
            let MF = ((MB * LH) * MC) + (ME * LZ);
            let AGC = ((Lanes([AFY[0], AFY[1], AFY[2], AFY[3], 0.0]) + Lanes([0.0, 0.0, AFZ[0], AFZ[1], AFZ[2]])) * MC) + Lanes([AGB[0], AGB[1], AGB[2], AGB[3], 0.0]);
            let MH = EQ / MG;
            let MI = MH.abs();
            let MK = M + (MI.powf(MJ));
            let MM = ES / ML;
            let MN = MM.abs();
            let MP = M + (MN.powf(MO));
            let MS = (AD * MR).exp();
            let MT = MQ * MS;
            let MU = M / MJ;
            let MV = MK.powf(MU);
            let MW = MT * MV;
            let AGD = ((((ACU / MG) * ((ADJ * (if MH >= ADI { 1.0 } else { 0.0 })) - XU)) * (MJ * (MI.powf((MJ - XU))))) * (MU * (MK.powf((MU - XU))))) * MT;
            let AGE = Lanes([0.0, ((((ABJ * MR) * MS) * MQ) * MV), 0.0]) + Lanes([AGD[0], 0.0, AGD[1]]);
            let MZ = (AD * MY).exp();
            let NA = MX * MZ;
            let AGF = ((ABJ * MY) * MZ) * MX;
            let ND = (AD * NC).exp();
            let NE = NB * ND;
            let NF = M / MO;
            let NG = MP.powf(NF);
            let NH = NE * NG;
            let AGG = ((((ACW / ML) * ((ADJ * (if MM >= ADI { 1.0 } else { 0.0 })) - XU)) * (MO * (MN.powf((MO - XU))))) * (NF * (MP.powf((NF - XU))))) * NE;
            let AGH = Lanes([0.0, ((((ABJ * NC) * ND) * NB) * NG), 0.0]) + Lanes([AGG[0], 0.0, AGG[1]]);
            let NI = EN - EI;
            let AGI = Lanes([YA, 0.0]) - Lanes([0.0, XY]);
            let NK = NI / NJ;
            let NL = NK.abs();
            let NN = M + (NL.powf(NM));
            let NO = M / NM;
            let NR = NP * (M + (NQ * ((NN.powf(NO)) - M)));
            let AGJ = (((((AGI / NJ) * ((ADJ * (if NK >= ADI { 1.0 } else { 0.0 })) - XU)) * (NM * (NL.powf((NM - XU))))) * (NO * (NN.powf((NO - XU))))) * NQ) * NP;
            let NS = NR * LI;
            let AGK = AGJ * LI;
            let AGL = YH * NR;
            let AGM = Lanes([AGK[0], AGK[1], 0.0, 0.0, 0.0]) + Lanes([0.0, 0.0, AGL[0], AGL[1], AGL[2]]);
            let NU = NT * MA;
            let AGN = AFV * NT;
            let NV = if parameters[32] == M { 1.0 } else { 0.0 };
            let OK;
            let VZ;
            let WA;
            let WB;
            let WC;
            let XO;
            let ZM;
            let ZN;
            let ZO;
            let ZP;
            let ZQ;
            if NV != 0.0 {
                let NW = LI / AI;
                let AGS = ABM * NW;
                let NX = -NW;
                let NY = NX * NR;
                let AGT = (((Lanes([YH[0], 0.0, YH[1], YH[2]]) - Lanes([AGS[0], AGS[1], AGS[2], 0.0])) / AI) * ABG) * NR;
                let AGU = AGJ * NX;
                let AGV = Lanes([0.0, 0.0, AGT[0], AGT[1], AGT[2], AGT[3]]) + Lanes([AGU[0], AGU[1], 0.0, 0.0, 0.0, 0.0]);
                let OA = ddt(3525, NZ);
                let OB = NR * OA;
                let AGW = AGJ * OA;
                let AGX = Lanes([AGW[0], AGW[1], 0.0]) + Lanes([0.0, 0.0, ((YC * AFF) * NR)]);
                let XN = NR * NZ;
                let AGY = AGJ * NZ;
                let AGZ = Lanes([AGY[0], AGY[1], 0.0]) + Lanes([0.0, 0.0, (YC * NR)]);
                let OD = (NZ.abs()) / OC;
                let OF = M + (OD.powf(OE));
                let OG = MW / OF;
                let AHA = (Lanes([AGE[0], AGE[1], AGE[2], 0.0]) - Lanes([0.0, 0.0, 0.0, ((((YC * ((ADJ * (if NZ >= ADI { 1.0 } else { 0.0 })) - XU)) / OC) * (OE * (OD.powf((OE - XU))))) * OG)])) / OF;
                OK = OG;
                VZ = NY;
                WA = NZ;
                WB = OB;
                WC = A;
                XO = XN;
                ZM = AHA;
                ZN = AGV;
                ZO = YC;
                ZP = AGX;
                ZQ = AGZ;
            } else {
                let AGO = Lanes([AGE[0], AGE[1], AGE[2], 0.0]);
                OK = MW;
                VZ = A;
                WA = A;
                WB = A;
                WC = OH;
                XO = A;
                ZM = AGO;
                ZN = AGP;
                ZO = AGQ;
                ZP = AGR;
                ZQ = AGR;
            }
            let OJ = if OI == M { 1.0 } else { 0.0 };
            let TW;
            let UF;
            let UP;
            let ZR;
            let ZS;
            let ZT;
            if OJ != 0.0 {
                let OM = OK + OL;
                let OO = NA + ON;
                let OQ = NH + OP;
                TW = OM;
                UF = OQ;
                UP = OO;
                ZR = ZM;
                ZS = AGH;
                ZT = AGF;
            } else {
                TW = OK;
                UF = NH;
                UP = NA;
                ZR = ZM;
                ZS = AGH;
                ZT = AGF;
            }
            let OR = if EJ <= A { 1.0 } else { 0.0 };
            let VM;
            let ZU;
            if OR != 0.0 {
                let OS = EH * EE;
                let OT = M - EB;
                let OU = EJ / EE;
                let OV = M - OU;
                let OW = (OT * (OV.ln())).exp();
                let OX = M - OW;
                let OY = (OS * OX) / OT;
                let AHE = (Lanes([0.0, (((ACO * EE) + (ACN * EH)) * OX), 0.0]) + ((((((((Lanes([ACP[0], 0.0, ACP[1]]) - Lanes([0.0, (ACN * OU), 0.0])) / EE) * ABG) * (XU / OV)) * OT) * OW) * ABG) * OS)) / OT;
                VM = OY;
                ZU = AHE;
            } else {
                let OZ = EH * EJ;
                let AHB = ACP * EH;
                let PB = PA * EB;
                let AHC = ACP * PB;
                let PC = (PB * EJ) / EE;
                let PD = M + PC;
                let PE = OZ * PD;
                let AHD = ((Lanes([0.0, (ACO * EJ), 0.0]) + Lanes([AHB[0], 0.0, AHB[1]])) * PD) + (((Lanes([AHC[0], 0.0, AHC[1]]) - Lanes([0.0, (ACN * PC), 0.0])) / EE) * OZ);
                VM = PE;
                ZU = AHD;
            }
            let PG = EM + ((-DF) * PF);
            let AHF = Lanes([0.0, ACR[0], ACR[1]]);
            let AHG = AHF + Lanes([((ACE * ABG) * PF), 0.0, 0.0]);
            let PH = if PG > A { 1.0 } else { 0.0 };
            let PX;
            let PY;
            let ZV;
            let ZW;
            if PH != 0.0 {
                let PI = M - PF;
                let PJ = ((-1e0f64 - DA) * (PI.ln())).exp();
                let PK = M - ((PJ * PI) * PI);
                let PL = M - DA;
                let PM = (DF * PK) / PL;
                let PN = PA * DA;
                let PO = (PN * PG) / DF;
                let PP = PI + PO;
                let PQ = (PG * PP) * PJ;
                let AHI = ((AHG * PP) + ((((AHG * PN) - Lanes([(ACE * PO), 0.0, 0.0])) / DF) * PG)) * PJ;
                let AHJ = Lanes([((ACE * PK) / PL), 0.0, 0.0]);
                PX = PM;
                PY = PQ;
                ZV = AHJ;
                ZW = AHI;
            } else {
                let PR = M - DA;
                let PS = EM / DF;
                let PT = M - PS;
                let PU = (PR * (PT.ln())).exp();
                let PV = M - PU;
                let PW = (DF * PV) / PR;
                let AHH = (Lanes([(ACE * PV), 0.0, 0.0]) + ((((((((AHF - Lanes([(ACE * PS), 0.0, 0.0])) / DF) * ABG) * (XU / PT)) * PR) * PU) * ABG) * DF)) / PR;
                PX = PW;
                PY = A;
                ZV = AHH;
                ZW = ACX;
            }
            let PZ = PX + PY;
            let QA = DJ * PZ;
            let AHK = Lanes([(ACG * PZ), 0.0, 0.0]) + ((ZV + ZW) * DJ);
            let QB = (-DS) * PF;
            let AHL = (ACJ * ABG) * PF;
            let QC = EO + QB;
            let AHM = Lanes([ACS[0], 0.0, ACS[1]]);
            let AHN = AHM + Lanes([0.0, AHL, 0.0]);
            let QD = if QC > A { 1.0 } else { 0.0 };
            let QT;
            let QU;
            let ZX;
            let ZY;
            if QD != 0.0 {
                let QE = M - PF;
                let QF = ((-1e0f64 - DP) * (QE.ln())).exp();
                let QG = M - ((QF * QE) * QE);
                let QH = M - DP;
                let QI = (DS * QG) / QH;
                let QJ = PA * DP;
                let QK = (QJ * QC) / DS;
                let QL = QE + QK;
                let QM = (QC * QL) * QF;
                let AHQ = ((AHN * QL) + ((((AHN * QJ) - Lanes([0.0, (ACJ * QK), 0.0])) / DS) * QC)) * QF;
                let AHR = Lanes([0.0, ((ACJ * QG) / QH), 0.0]);
                QT = QI;
                QU = QM;
                ZX = AHR;
                ZY = AHQ;
            } else {
                let QN = M - DP;
                let QO = EO / DS;
                let QP = M - QO;
                let QQ = (QN * (QP.ln())).exp();
                let QR = M - QQ;
                let QS = (DS * QR) / QN;
                let AHO = (Lanes([0.0, (ACJ * QR), 0.0]) + ((((((((AHM - Lanes([0.0, (ACJ * QO), 0.0])) / DS) * ABG) * (XU / QP)) * QN) * QQ) * ABG) * DS)) / QN;
                QT = QS;
                QU = A;
                ZX = AHO;
                ZY = AHP;
            }
            let QV = QT + QU;
            let QX = M - QW;
            let QY = QX * (DV * QV);
            let AHS = (Lanes([0.0, (ACK * QV), 0.0]) + ((ZX + ZY) * DV)) * QX;
            let QZ = U + QB;
            let AHT = Lanes([0.0, ABF[0], ABF[1]]);
            let AHU = AHT + Lanes([AHL, 0.0, 0.0]);
            let RA = if QZ > A { 1.0 } else { 0.0 };
            let RQ;
            let RR;
            let ZZ;
            let AAA;
            if RA != 0.0 {
                let RB = M - PF;
                let RC = ((-1e0f64 - DP) * (RB.ln())).exp();
                let RD = M - ((RC * RB) * RB);
                let RE = M - DP;
                let RF = (DS * RD) / RE;
                let RG = PA * DP;
                let RH = (RG * QZ) / DS;
                let RI = RB + RH;
                let RJ = (QZ * RI) * RC;
                let AHW = ((AHU * RI) + ((((AHU * RG) - Lanes([(ACJ * RH), 0.0, 0.0])) / DS) * QZ)) * RC;
                let AHX = Lanes([((ACJ * RD) / RE), 0.0, 0.0]);
                RQ = RF;
                RR = RJ;
                ZZ = AHX;
                AAA = AHW;
            } else {
                let RK = M - DP;
                let RL = U / DS;
                let RM = M - RL;
                let RN = (RK * (RM.ln())).exp();
                let RO = M - RN;
                let RP = (DS * RO) / RK;
                let AHV = (Lanes([(ACJ * RO), 0.0, 0.0]) + ((((((((AHT - Lanes([(ACJ * RL), 0.0, 0.0])) / DS) * ABG) * (XU / RM)) * RK) * RN) * ABG) * DS)) / RK;
                RQ = RP;
                RR = A;
                ZZ = AHV;
                AAA = AEC;
            }
            let RS = RQ + RR;
            let RT = QW * (DV * RS);
            let AHY = (Lanes([(ACK * RS), 0.0, 0.0]) + ((ZZ + AAA) * DV)) * QW;
            let RV = if (if RU != A { 1.0 } else { 0.0 }) != 0.0 && (if NP != A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let VP;
            let AAB;
            if RV != 0.0 {
                let RW = (((Q * RU) * 3.141592653589793e0f64) / 1.8e2f64) * NP;
                let RX = RW * MB;
                let AIA = AFX * RW;
                VP = RX;
                AAB = AIA;
            } else {
                VP = A;
                AAB = AHZ;
            }
            let SA = if RZ > A { 1.0 } else { 0.0 };
            let SB = if (if RY == M { 1.0 } else { 0.0 }) != 0.0 && SA != 0.0 { 1.0 } else { 0.0 };
            let WD;
            let WE;
            let WF;
            let WG;
            let WH;
            let WJ;
            let WL;
            let WN;
            let WP;
            let WR;
            let WU;
            let WX;
            let XA;
            let XP;
            let XR;
            let XT;
            let AAC;
            let AAD;
            let AAE;
            let AAF;
            let AAG;
            let AAH;
            let AAI;
            let AAJ;
            let AAK;
            let AAL;
            let AAM;
            let AAN;
            if SB != 0.0 {
                let SD = LM * NI;
                let AIY = AFL * NI;
                let AIZ = AGI * LM;
                let AJA = ((Lanes([0.0, 0.0, AIY[0], AIY[1], AIY[2], AIY[3]]) + Lanes([AIZ[0], AIZ[1], 0.0, 0.0, 0.0, 0.0])) * ((ADJ * (if SD >= ADI { 1.0 } else { 0.0 })) - XU)) * SC;
                let SF = EN - SE;
                let SG = LQ * SF;
                let AJB = AFM * SF;
                let AJC = (Lanes([0.0, YA]) - Lanes([YD, 0.0])) * LQ;
                let AJD = (Lanes([0.0, 0.0, AJB[0], AJB[1], AJB[2]]) + Lanes([AJC[0], AJC[1], 0.0, 0.0, 0.0])) * ((ADJ * (if SG >= ADI { 1.0 } else { 0.0 })) - XU);
                let SH = (SC * (SD.abs())) - (SG.abs());
                let AJE = Lanes([0.0, AJA[0], AJA[1], AJA[2], AJA[3], AJA[4], AJA[5]]) - Lanes([AJD[0], AJD[1], 0.0, AJD[2], AJD[3], AJD[4], 0.0]);
                let SI = B / RZ;
                let AJF = XV / RZ;
                let SK = B * SJ;
                let AJG = XV * SJ;
                let SL = ddt(3871, SK);
                let AJH = AJG * AFF;
                WD = SH;
                WE = SI;
                WF = SL;
                WG = SM;
                WH = A;
                WJ = A;
                WL = A;
                WN = A;
                WP = A;
                WR = A;
                WU = A;
                WX = A;
                XA = A;
                XP = SK;
                XR = A;
                XT = A;
                AAC = AJE;
                AAD = AJF;
                AAE = AJH;
                AAF = AIB;
                AAG = AIJ;
                AAH = ABD;
                AAI = AIK;
                AAJ = AIK;
                AAK = AIB;
                AAL = AJG;
                AAM = ABD;
                AAN = AIK;
            } else {
                let SO = if (if (if RY == LY { 1.0 } else { 0.0 }) != 0.0 && SA != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if SN > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let WI;
                let WK;
                let WM;
                let WO;
                let WQ;
                let WS;
                let WV;
                let WY;
                let XB;
                let XQ;
                let XS;
                let AAO;
                let AAP;
                let AAQ;
                let AAR;
                let AAS;
                let AAT;
                let AAU;
                let AAV;
                if SO != 0.0 {
                    let SQ = LM * NI;
                    let AIL = AFL * NI;
                    let AIM = AGI * LM;
                    let AIN = ((Lanes([0.0, 0.0, AIL[0], AIL[1], AIL[2], AIL[3]]) + Lanes([AIM[0], AIM[1], 0.0, 0.0, 0.0, 0.0])) * ((ADJ * (if SQ >= ADI { 1.0 } else { 0.0 })) - XU)) * SP;
                    let SR = EN - SE;
                    let SS = LQ * SR;
                    let AIO = AFM * SR;
                    let AIP = (Lanes([0.0, YA]) - Lanes([YD, 0.0])) * LQ;
                    let AIQ = (Lanes([0.0, 0.0, AIO[0], AIO[1], AIO[2]]) + Lanes([AIP[0], AIP[1], 0.0, 0.0, 0.0])) * ((ADJ * (if SS >= ADI { 1.0 } else { 0.0 })) - XU);
                    let ST = (SP * (SQ.abs())) - (SS.abs());
                    let AIR = Lanes([0.0, AIN[0], AIN[1], AIN[2], AIN[3], AIN[4], AIN[5]]) - Lanes([AIQ[0], AIQ[1], 0.0, AIQ[2], AIQ[3], AIQ[4], 0.0]);
                    let SV = (B - SU) / RZ;
                    let AIS = (Lanes([XV, 0.0]) - Lanes([0.0, YE])) / RZ;
                    let SW = SJ * B;
                    let AIT = XV * SJ;
                    let SX = ddt(3902, SW);
                    let AIU = AIT * AFF;
                    let SY = SU / SN;
                    let AIV = YE / SN;
                    let TA = SZ * SU;
                    let AIW = YE * SZ;
                    let TB = ddt(3909, TA);
                    let AIX = AIW * AFF;
                    WI = ST;
                    WK = SV;
                    WM = SX;
                    WO = SY;
                    WQ = TB;
                    WS = A;
                    WV = A;
                    WY = A;
                    XB = A;
                    XQ = SW;
                    XS = TA;
                    AAO = AIR;
                    AAP = AIS;
                    AAQ = AIU;
                    AAR = AIV;
                    AAS = AIX;
                    AAT = AIB;
                    AAU = AIT;
                    AAV = AIW;
                } else {
                    let TC = if RY == -1e0f64 { 1.0 } else { 0.0 };
                    let WT;
                    let WW;
                    let WZ;
                    let XC;
                    let AAW;
                    if TC != 0.0 {
                        let TE = LM * NI;
                        let AIC = AFL * NI;
                        let AID = AGI * LM;
                        let AIE = ((Lanes([0.0, 0.0, AIC[0], AIC[1], AIC[2], AIC[3]]) + Lanes([AID[0], AID[1], 0.0, 0.0, 0.0, 0.0])) * ((ADJ * (if TE >= ADI { 1.0 } else { 0.0 })) - XU)) * TD;
                        let TF = EN - SE;
                        let TG = LQ * TF;
                        let AIF = AFM * TF;
                        let AIG = (Lanes([0.0, YA]) - Lanes([YD, 0.0])) * LQ;
                        let AIH = (Lanes([0.0, 0.0, AIF[0], AIF[1], AIF[2]]) + Lanes([AIG[0], AIG[1], 0.0, 0.0, 0.0])) * ((ADJ * (if TG >= ADI { 1.0 } else { 0.0 })) - XU);
                        let TH = (TD * (TE.abs())) - (TG.abs());
                        let AII = Lanes([0.0, AIE[0], AIE[1], AIE[2], AIE[3], AIE[4], AIE[5]]) - Lanes([AIH[0], AIH[1], 0.0, AIH[2], AIH[3], AIH[4], 0.0]);
                        WT = TH;
                        WW = TI;
                        WZ = A;
                        XC = A;
                        AAW = AII;
                    } else {
                        WT = A;
                        WW = A;
                        WZ = TJ;
                        XC = TK;
                        AAW = AIB;
                    }
                    WI = A;
                    WK = A;
                    WM = A;
                    WO = A;
                    WQ = A;
                    WS = WT;
                    WV = WW;
                    WY = WZ;
                    XB = XC;
                    XQ = A;
                    XS = A;
                    AAO = AIB;
                    AAP = AIJ;
                    AAQ = ABD;
                    AAR = AIK;
                    AAS = AIK;
                    AAT = AAW;
                    AAU = ABD;
                    AAV = AIK;
                }
                WD = A;
                WE = A;
                WF = A;
                WG = A;
                WH = WI;
                WJ = WK;
                WL = WM;
                WN = WO;
                WP = WQ;
                WR = WS;
                WU = WV;
                WX = WY;
                XA = XB;
                XP = A;
                XR = XQ;
                XT = XS;
                AAC = AIB;
                AAD = ABD;
                AAE = ABD;
                AAF = AAO;
                AAG = AAP;
                AAH = AAQ;
                AAI = AAR;
                AAJ = AAS;
                AAK = AAT;
                AAL = ABD;
                AAM = AAU;
                AAN = AAV;
            }
            let TL = ctx.simparam_or("gmin", A);
            let TM = TL * EL;
            let AJI = ACQ * TL;
            let TN = ctx.simparam_or("gmin", A);
            let TO = TN * T;
            let AJJ = ABE * TN;
            let TP = ctx.simparam_or("gmin", A);
            let TQ = TP * (S - EK);
            let AJK = (Lanes([XX, 0.0]) - Lanes([0.0, XZ])) * TP;
            let TR = (MQ + (OI * OL)) / P;
            let TS = (NB + (OI * OP)) / P;
            let TT = (MX + (OI * ON)) / P;
            let TV = if (if TR > A { 1.0 } else { 0.0 }) != 0.0 && (if TR >= TU { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let XD;
            let XE;
            let XF;
            let AAX;
            if TV != 0.0 {
                let TX = TW / P;
                let AJM = ZR / P;
                let TY = if TX > TU { 1.0 } else { 0.0 };
                let TZ;
                let AAY;
                if TY != 0.0 {
                    TZ = TX;
                    AAY = AJM;
                } else {
                    TZ = TU;
                    AAY = AJL;
                }
                let UA = EP / TZ;
                let AJN = (Lanes([ACT[0], 0.0, ACT[1], 0.0]) - (AAY * UA)) / TZ;
                let UB = if TX >= TU { 1.0 } else { 0.0 };
                if UB != 0.0 {
                } else {
                }
                XD = UA;
                XE = UC;
                XF = A;
                AAX = AJN;
            } else {
                XD = A;
                XE = A;
                XF = UD;
                AAX = AJL;
            }
            let UE = if (if TS > A { 1.0 } else { 0.0 }) != 0.0 && (if TS >= TU { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let XG;
            let XH;
            let XI;
            let AAZ;
            if UE != 0.0 {
                let UG = UF / P;
                let AJP = ZS / P;
                let UH = if UG > TU { 1.0 } else { 0.0 };
                let UI;
                let ABA;
                if UH != 0.0 {
                    UI = UG;
                    ABA = AJP;
                } else {
                    UI = TU;
                    ABA = AJO;
                }
                let UJ = ER / UI;
                let AJQ = (Lanes([ACV[0], 0.0, ACV[1]]) - (ABA * UJ)) / UI;
                let UK = if UG >= TU { 1.0 } else { 0.0 };
                if UK != 0.0 {
                } else {
                }
                XG = UJ;
                XH = UL;
                XI = A;
                AAZ = AJQ;
            } else {
                XG = A;
                XH = A;
                XI = UM;
                AAZ = AJO;
            }
            let UN = if (if TT > A { 1.0 } else { 0.0 }) != 0.0 && (if TT >= TU { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let XJ;
            let XK;
            let XL;
            let ABB;
            if UN != 0.0 {
                let UO = SE - S;
                let AJS = Lanes([YD, 0.0]) - Lanes([0.0, XX]);
                let UQ = UP / P;
                let AJT = ZT / P;
                let UR = if UQ > TU { 1.0 } else { 0.0 };
                let US;
                let ABC;
                if UR != 0.0 {
                    US = UQ;
                    ABC = AJT;
                } else {
                    US = TU;
                    ABC = ABD;
                }
                let UT = UO / US;
                let AJU = (Lanes([AJS[0], 0.0, AJS[1]]) - Lanes([0.0, (ABC * UT), 0.0])) / US;
                let UU = if UQ >= TU { 1.0 } else { 0.0 };
                if UU != 0.0 {
                } else {
                }
                XJ = UT;
                XK = UV;
                XL = A;
                ABB = AJU;
            } else {
                XJ = A;
                XK = A;
                XL = UW;
                ABB = AJR;
            }
            let UX = Q * LM;
            let UY = UX * P;
            let AJV = (AFL * Q) * P;
            let UZ = (Q * LQ) * P;
            let AJW = (AFM * Q) * P;
            let VA = Q * ((-MA) * P);
            let AJX = ((AFV * ABG) * P) * Q;
            let VB = (Q * MF) * P;
            let AJY = (AGC * Q) * P;
            let VC = (Q * QA) * P;
            let AJZ = (AHK * Q) * P;
            let VD = ddt(4101, VC);
            let AKA = AJZ * AFF;
            let VE = (Q * NS) * P;
            let AKB = (AGM * Q) * P;
            let VF = ddt(4107, VE);
            let AKC = AKB * AFF;
            let VG = (Q * QY) * P;
            let AKD = (AHS * Q) * P;
            let VH = ddt(4113, VG);
            let AKE = AKD * AFF;
            let VI = (Q * RT) * P;
            let AKF = (AHY * Q) * P;
            let VJ = ddt(4119, VI);
            let AKG = AKF * AFF;
            let VK = (Q * NU) * P;
            let AKH = (AGN * Q) * P;
            let VL = ddt(4125, VK);
            let AKI = AKH * AFF;
            let VN = (Q * VM) * P;
            let AKJ = (ZU * Q) * P;
            let VO = ddt(4131, VN);
            let AKK = AKJ * AFF;
            let VQ = (-VP) * P;
            let AKL = (AAB * ABG) * P;
            let VR = ddt(4136, VQ);
            let AKM = AKL * AFF;
            let VS = VP * P;
            let AKN = AAB * P;
            let VT = ddt(4140, VS);
            let AKO = AKN * AFF;
            let VU = if (if (if parameters[28] > A { 1.0 } else { 0.0 }) != 0.0 && (if parameters[27] > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) > A { 1.0 } else { 0.0 };
            if VU != 0.0 {
            } else {
            }
            let VW = if UX >= A { 1.0 } else { 0.0 };
            if VW != 0.0 {
            } else {
            }
            let AKP = AFD[0];
            let AKQ = AFD[1];
            let AKR = AFD[2];
            let AKS = AFE;
            let AKT = AFG;
            let AKU = ZN[0];
            let AKV = ZN[1];
            let AKW = ZN[2];
            let AKX = ZN[3];
            let AKY = ZN[4];
            let AKZ = ZN[5];
            let ALA = ZO;
            let ALB = ZP[0];
            let ALC = ZP[1];
            let ALD = ZP[2];
            let ALE = AAC[0];
            let ALF = AAC[1];
            let ALG = AAC[2];
            let ALH = AAC[3];
            let ALI = AAC[4];
            let ALJ = AAC[5];
            let ALK = AAC[6];
            let ALL = AAD;
            let ALM = AAE;
            let ALN = AAF[0];
            let ALO = AAF[1];
            let ALP = AAF[2];
            let ALQ = AAF[3];
            let ALR = AAF[4];
            let ALS = AAF[5];
            let ALT = AAF[6];
            let ALU = AAG[0];
            let ALV = AAG[1];
            let ALW = AAH;
            let ALX = AAI;
            let ALY = AAJ;
            let ALZ = AAK[0];
            let AMA = AAK[1];
            let AMB = AAK[2];
            let AMC = AAK[3];
            let AMD = AAK[4];
            let AME = AAK[5];
            let AMF = AAK[6];
            let AMG = AJI[0];
            let AMH = AJI[1];
            let AMI = AJJ[0];
            let AMJ = AJJ[1];
            let AMK = AJK[0];
            let AML = AJK[1];
            let AMM = AAX[0];
            let AMN = AAX[1];
            let AMO = AAX[2];
            let AMP = AAX[3];
            let AMQ = AAZ[0];
            let AMR = AAZ[1];
            let AMS = AAZ[2];
            let AMT = ABB[0];
            let AMU = ABB[1];
            let AMV = ABB[2];
            let AMW = AJV[0];
            let AMX = AJV[1];
            let AMY = AJV[2];
            let AMZ = AJV[3];
            let ANA = AJW[0];
            let ANB = AJW[1];
            let ANC = AJW[2];
            let AND = AJX[0];
            let ANE = AJX[1];
            let ANF = AJX[2];
            let ANG = AJX[3];
            let ANH = AJY[0];
            let ANI = AJY[1];
            let ANJ = AJY[2];
            let ANK = AJY[3];
            let ANL = AJY[4];
            let ANM = AKA[0];
            let ANN = AKA[1];
            let ANO = AKA[2];
            let ANP = AKC[0];
            let ANQ = AKC[1];
            let ANR = AKC[2];
            let ANS = AKC[3];
            let ANT = AKC[4];
            let ANU = AKE[0];
            let ANV = AKE[1];
            let ANW = AKE[2];
            let ANX = AKG[0];
            let ANY = AKG[1];
            let ANZ = AKG[2];
            let AOA = AKI[0];
            let AOB = AKI[1];
            let AOC = AKI[2];
            let AOD = AKI[3];
            let AOE = AKK[0];
            let AOF = AKK[1];
            let AOG = AKK[2];
            let AOH = AKM[0];
            let AOI = AKM[1];
            let AOJ = AKM[2];
            let AOK = AKM[3];
            let AOL = AKO[0];
            let AOM = AKO[1];
            let AON = AKO[2];
            let AOO = AKO[3];
            let AOP = AFH;
            let AOQ = ZQ[0];
            let AOR = ZQ[1];
            let AOS = ZQ[2];
            let AOT = AAL;
            let AOU = AAM;
            let AOV = AAN;
            let AOW = AJZ[0];
            let AOX = AJZ[1];
            let AOY = AJZ[2];
            let AOZ = AKB[0];
            let APA = AKB[1];
            let APB = AKB[2];
            let APC = AKB[3];
            let APD = AKB[4];
            let APE = AKD[0];
            let APF = AKD[1];
            let APG = AKD[2];
            let APH = AKF[0];
            let API = AKF[1];
            let APJ = AKF[2];
            let APK = AKH[0];
            let APL = AKH[1];
            let APM = AKH[2];
            let APN = AKH[3];
            let APO = AKJ[0];
            let APP = AKJ[1];
            let APQ = AKJ[2];
            let APR = AKL[0];
            let APS = AKL[1];
            let APT = AKL[2];
            let APU = AKL[3];
            let APV = AKN[0];
            let APW = AKN[1];
            let APX = AKN[2];
            let APY = AKN[3];
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(9),
            None,
            multiplicity * (KY),
            [5, 6, 9],
            [AKP, AKQ, AKR],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(9),
            None,
            multiplicity * (LA),
            [9],
            [AKS],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(9),
            None,
            multiplicity * (LC),
            [9],
            [AKT],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(8),
            None,
            multiplicity * (VZ),
            [1, 2, 3, 4, 5, 6],
            [AKU, AKV, AKW, AKX, AKY, AKZ],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(8),
            None,
            multiplicity * (WA),
            [8],
            [ALA],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(8),
            None,
            multiplicity * (WB),
            [1, 2, 8],
            [ALB, ALC, ALD],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(8), None, 0, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            0,
            WC,
            [],
            [],
            [],
            [],
        );
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(3),
            None,
            multiplicity * (WD),
            [0, 1, 2, 3, 4, 5, 6],
            [ALE, ALF, ALG, ALH, ALI, ALJ, ALK],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(3),
            None,
            multiplicity * (WE),
            [3],
            [ALL],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(3),
            None,
            multiplicity * (WF),
            [3],
            [ALM],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(7), None, 1, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            1,
            WG,
            [],
            [],
            [],
            [],
        );
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(3),
            None,
            multiplicity * (WH),
            [0, 1, 2, 3, 4, 5, 6],
            [ALN, ALO, ALP, ALQ, ALR, ALS, ALT],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(3),
            Some(7),
            multiplicity * (WJ),
            [3, 7],
            [ALU, ALV],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(3),
            None,
            multiplicity * (WL),
            [3],
            [ALW],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(7),
            None,
            multiplicity * (WN),
            [7],
            [ALX],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(7),
            None,
            multiplicity * (WP),
            [7],
            [ALY],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(3),
            None,
            multiplicity * (WR),
            [0, 1, 2, 3, 4, 5, 6],
            [ALZ, AMA, AMB, AMC, AMD, AME, AMF],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(7), None, 2, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            2,
            WU,
            [],
            [],
            [],
            [],
        );
        stamper.stamp_potential_branch_local(Some(3), None, 3, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            3,
            WX,
            [],
            [],
            [],
            [],
        );
        stamper.stamp_potential_branch_local(Some(7), None, 4, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            4,
            XA,
            [],
            [],
            [],
            [],
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(5),
            Some(6),
            multiplicity * (TM),
            [5, 6],
            [AMG, AMH],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(5),
            Some(4),
            multiplicity * (TO),
            [4, 5],
            [AMI, AMJ],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(4),
            Some(6),
            multiplicity * (TQ),
            [4, 6],
            [AMK, AML],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(1),
            Some(5),
            multiplicity * (XD),
            [1, 3, 5, 8],
            [AMM, AMN, AMO, AMP],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(1),
            Some(5),
            multiplicity * (XE),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(1), Some(5), 5, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            5,
            XF,
            [],
            [],
            [],
            [],
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(2),
            Some(6),
            multiplicity * (XG),
            [2, 3, 6],
            [AMQ, AMR, AMS],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(2),
            Some(6),
            multiplicity * (XH),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(2), Some(6), 6, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            6,
            XI,
            [],
            [],
            [],
            [],
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(0),
            Some(4),
            multiplicity * (XJ),
            [0, 3, 4],
            [AMT, AMU, AMV],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(0),
            Some(4),
            multiplicity * (XK),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(0), Some(4), 7, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            7,
            XL,
            [],
            [],
            [],
            [],
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(5),
            Some(6),
            multiplicity * (UY),
            [3, 4, 5, 6],
            [AMW, AMX, AMY, AMZ],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(5),
            Some(4),
            multiplicity * (UZ),
            [3, 4, 5],
            [ANA, ANB, ANC],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(4),
            Some(6),
            multiplicity * (VA),
            [3, 4, 5, 6],
            [AND, ANE, ANF, ANG],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(4),
            Some(6),
            multiplicity * (VB),
            [3, 4, 5, 6, 9],
            [ANH, ANI, ANJ, ANK, ANL],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(5),
            Some(6),
            multiplicity * (VD),
            [3, 5, 6],
            [ANM, ANN, ANO],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(5),
            Some(6),
            multiplicity * (VF),
            [1, 2, 3, 5, 6],
            [ANP, ANQ, ANR, ANS, ANT],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(1),
            Some(4),
            multiplicity * (VH),
            [1, 3, 4],
            [ANU, ANV, ANW],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(5),
            Some(4),
            multiplicity * (VJ),
            [3, 4, 5],
            [ANX, ANY, ANZ],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(5),
            Some(4),
            multiplicity * (VL),
            [3, 4, 5, 6],
            [AOA, AOB, AOC, AOD],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(2),
            Some(4),
            multiplicity * (VO),
            [2, 3, 4],
            [AOE, AOF, AOG],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(5),
            Some(6),
            multiplicity * (VR),
            [3, 4, 5, 6],
            [AOH, AOI, AOJ, AOK],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(5),
            Some(4),
            multiplicity * (VT),
            [3, 4, 5, 6],
            [AOL, AOM, AON, AOO],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(5),
            Some(6),
            multiplicity * (VV),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(5),
            Some(6),
            multiplicity * (VX),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(4),
            Some(6),
            multiplicity * (VY),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        self.canonical_reactive[0] = KY;
        self.canonical_reactive[1] = LA;
        self.canonical_reactive[2] = XM;
        self.canonical_reactive[3] = AOP;
        self.canonical_reactive[4] = VZ;
        self.canonical_reactive[5] = WA;
        self.canonical_reactive[6] = XO;
        self.canonical_reactive[7] = AOQ;
        self.canonical_reactive[8] = AOR;
        self.canonical_reactive[9] = AOS;
        self.canonical_reactive[10] = WC;
        self.canonical_reactive[11] = WD;
        self.canonical_reactive[12] = WE;
        self.canonical_reactive[13] = XP;
        self.canonical_reactive[14] = AOT;
        self.canonical_reactive[15] = WG;
        self.canonical_reactive[16] = WH;
        self.canonical_reactive[17] = WJ;
        self.canonical_reactive[18] = XR;
        self.canonical_reactive[19] = AOU;
        self.canonical_reactive[20] = WN;
        self.canonical_reactive[21] = XT;
        self.canonical_reactive[22] = AOV;
        self.canonical_reactive[23] = WR;
        self.canonical_reactive[24] = WU;
        self.canonical_reactive[25] = WX;
        self.canonical_reactive[26] = XA;
        self.canonical_reactive[27] = TM;
        self.canonical_reactive[28] = TO;
        self.canonical_reactive[29] = TQ;
        self.canonical_reactive[30] = XD;
        self.canonical_reactive[31] = XE;
        self.canonical_reactive[32] = XF;
        self.canonical_reactive[33] = XG;
        self.canonical_reactive[34] = XH;
        self.canonical_reactive[35] = XI;
        self.canonical_reactive[36] = XJ;
        self.canonical_reactive[37] = XK;
        self.canonical_reactive[38] = XL;
        self.canonical_reactive[39] = UY;
        self.canonical_reactive[40] = UZ;
        self.canonical_reactive[41] = VA;
        self.canonical_reactive[42] = VB;
        self.canonical_reactive[43] = VC;
        self.canonical_reactive[44] = AOW;
        self.canonical_reactive[45] = AOX;
        self.canonical_reactive[46] = AOY;
        self.canonical_reactive[47] = VE;
        self.canonical_reactive[48] = AOZ;
        self.canonical_reactive[49] = APA;
        self.canonical_reactive[50] = APB;
        self.canonical_reactive[51] = APC;
        self.canonical_reactive[52] = APD;
        self.canonical_reactive[53] = VG;
        self.canonical_reactive[54] = APE;
        self.canonical_reactive[55] = APF;
        self.canonical_reactive[56] = APG;
        self.canonical_reactive[57] = VI;
        self.canonical_reactive[58] = APH;
        self.canonical_reactive[59] = API;
        self.canonical_reactive[60] = APJ;
        self.canonical_reactive[61] = VK;
        self.canonical_reactive[62] = APK;
        self.canonical_reactive[63] = APL;
        self.canonical_reactive[64] = APM;
        self.canonical_reactive[65] = APN;
        self.canonical_reactive[66] = VN;
        self.canonical_reactive[67] = APO;
        self.canonical_reactive[68] = APP;
        self.canonical_reactive[69] = APQ;
        self.canonical_reactive[70] = VQ;
        self.canonical_reactive[71] = APR;
        self.canonical_reactive[72] = APS;
        self.canonical_reactive[73] = APT;
        self.canonical_reactive[74] = APU;
        self.canonical_reactive[75] = VS;
        self.canonical_reactive[76] = APV;
        self.canonical_reactive[77] = APW;
        self.canonical_reactive[78] = APX;
        self.canonical_reactive[79] = APY;
        self.canonical_reactive[80] = VV;
        self.canonical_reactive[81] = VX;
        self.canonical_reactive[82] = VY;
    }

    pub fn stamp_reactive(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedReactiveStamper<'_>) {
        let multiplicity = self.multiplicity;
        let cached = &*self.canonical_reactive;
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(9),
            None,
            &[9],
            &[cached[3]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(8),
            None,
            &[1, 2, 8],
            &[cached[7], cached[8], cached[9]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(3),
            None,
            &[3],
            &[cached[14]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(3),
            None,
            &[3],
            &[cached[19]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(7),
            None,
            &[7],
            &[cached[22]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(5),
            Some(6),
            &[3, 5, 6],
            &[cached[44], cached[45], cached[46]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(5),
            Some(6),
            &[1, 2, 3, 5, 6],
            &[cached[48], cached[49], cached[50], cached[51], cached[52]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(1),
            Some(4),
            &[1, 3, 4],
            &[cached[54], cached[55], cached[56]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(5),
            Some(4),
            &[3, 4, 5],
            &[cached[58], cached[59], cached[60]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(5),
            Some(4),
            &[3, 4, 5, 6],
            &[cached[62], cached[63], cached[64], cached[65]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(2),
            Some(4),
            &[2, 3, 4],
            &[cached[67], cached[68], cached[69]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(5),
            Some(6),
            &[3, 4, 5, 6],
            &[cached[71], cached[72], cached[73], cached[74]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(5),
            Some(4),
            &[3, 4, 5, 6],
            &[cached[76], cached[77], cached[78], cached[79]],
            &[],
            &[],
            multiplicity,
        );
    }

}
