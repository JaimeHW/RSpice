#![allow(dead_code, non_snake_case, unused_imports, unused_mut, unused_parens, unused_variables)]

use super::state::Instance;
use rspice_veriloga_runtime::{GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, Lanes, rspice_eval_ddt, rspice_eval_idt, rspice_limexp, rspice_limited_exp, rspice_limited_exp_derivative};
impl Instance {
    pub fn stamp(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedStamper<'_>) {
        let parameters = &self.params.values;
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
            let slot = match operator { 6568 => 0usize, 6570 => 1usize, 6572 => 2usize, 6574 => 3usize, 6576 => 4usize, 6578 => 5usize, 6580 => 6usize, 6585 => 7usize, 6589 => 8usize, _ => usize::MAX };
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
            let C = node_potentials[4];
            let E = 1.3806503e-23f64;
            let F = 1.602176462e-19f64;
            let J = parameters[53];
            let K = parameters[90];
            let M = parameters[1];
            let N = parameters[91];
            let P = parameters[2];
            let Q = parameters[68];
            let S = parameters[6];
            let T = parameters[92];
            let V = parameters[7];
            let W = parameters[67];
            let Y = parameters[8];
            let Z = parameters[66];
            let AB = parameters[9];
            let AC = parameters[69];
            let AE = parameters[10];
            let AF = parameters[93];
            let AH = parameters[11];
            let AI = parameters[78];
            let AL = 1e0f64;
            let AQ = parameters[12];
            let AT = parameters[94];
            let AU = parameters[95];
            let BA = parameters[13];
            let BD = parameters[42];
            let BI = parameters[44];
            let BL = parameters[31];
            let BM = parameters[79];
            let BO = parameters[72];
            let BT = parameters[33];
            let BW = parameters[34];
            let BX = parameters[80];
            let CD = parameters[35];
            let CG = parameters[36];
            let CH = parameters[73];
            let CM = parameters[37];
            let CQ = parameters[38];
            let CV = parameters[39];
            let CZ = parameters[45];
            let DB = parameters[46];
            let DD = parameters[47];
            let DE = parameters[74];
            let DJ = parameters[48];
            let DM = parameters[49];
            let DR = parameters[50];
            let DU = parameters[81];
            let DY = parameters[41];
            let DZ = parameters[82];
            let EB = parameters[98];
            let EC = parameters[102];
            let EE = parameters[99];
            let EF = parameters[103];
            let EH = 2e0f64;
            let EK = 5e-1f64;
            let EL = parameters[17];
            let EV = 3e0f64;
            let FC = 4e0f64;
            let FJ = parameters[24];
            let GA = parameters[28];
            let GR = parameters[16];
            let GT = parameters[18];
            let GV = parameters[21];
            let GX = parameters[25];
            let HA = parameters[23];
            let HC = parameters[27];
            let HE = parameters[29];
            let HG = parameters[4];
            let HJ = parameters[3];
            let HK = parameters[70];
            let HQ = parameters[51];
            let HU = parameters[52];
            let IB = parameters[54];
            let IF = parameters[55];
            let IM = parameters[5];
            let IQ = parameters[59];
            let IU = parameters[60];
            let IZ = node_potentials[8];
            let JA = node_potentials[9];
            let JC = node_potentials[7];
            let JE = node_potentials[6];
            let JG = node_potentials[5];
            let JI = node_potentials[10];
            let JL = parameters[14];
            let JN = parameters[19];
            let KK = -5e-1f64;
            let LT = -5e-1f64;
            let MJ = parameters[26];
            let MW = parameters[85];
            let NQ = parameters[86];
            let PG = -5e-1f64;
            let SN = -5e-1f64;
            let TE = parameters[30];
            let TG = node_potentials[11];
            let UD = -5e-1f64;
            let VH = 1e-4f64;
            let VN = parameters[89];
            let WE = parameters[43];
            let WO = parameters[32];
            let XB = parameters[100];
            let ZQ = parameters[40];
            let ZT = 1e-2f64;
            let AAH = node_potentials[0];
            let ABD = node_potentials[1];
            let ABJ = node_potentials[2];
            let ACB = node_potentials[3];
            let ACI = parameters[56];
            let ACJ = parameters[57];
            let ACL = parameters[58];
            let ACM = 1.44e0f64;
            let ACX = parameters[61];
            let ACY = parameters[22];
            let ADH = parameters[87];
            let ADK = parameters[15];
            let ADM = parameters[20];
            let AER = parameters[83];
            let AEU = parameters[84];
            let AFG = 1e0f64;
            let AFH = 1e0f64;
            let AFI = 1e0f64;
            let AFJ = 1e0f64;
            let AFK = 1e0f64;
            let AFL = 1e0f64;
            let AFM = 1e0f64;
            let AFN = 1e0f64;
            let AFO = 1e0f64;
            let AFP = 1e0f64;
            let AFQ = 1e0f64;
            let AFR = 1e0f64;
            let AFS = 1e0f64;
            let AHX = -1e0f64;
            let AJB = 2e0f64;
            let AJR = 0e0f64;
            let AKN = Lanes([0e0f64; 3]);
            let ALD = Lanes([0e0f64; 3]);
            let AMN = Lanes([0e0f64; 3]);
            let ANV = Lanes([0e0f64; 3]);
            let ANZ = Lanes([0e0f64; 3]);
            let APJ = Lanes([0e0f64; 5]);
            let APK = Lanes([0e0f64; 6]);
            let AQT = Lanes([0e0f64; 4]);
            let ARA = Lanes([0e0f64; 3]);
            let ARF = Lanes([0e0f64; 4]);
            let ARP = Lanes([0e0f64; 3]);
            let ARS = Lanes([0e0f64; 5]);
            let ARW = Lanes([0e0f64; 3]);
            let ARZ = Lanes([0e0f64; 6]);
            let ASG = Lanes([0e0f64; 3]);
            let AUJ = ddt_scale();
            let B = 2.7315e2f64 + parameters[0];
            let D = (temperature + parameters[105]) + C;
            let G = (E * D) / F;
            let AHM = (AFH * E) / F;
            let H = D / B;
            let AHN = AFH / B;
            let I = D - B;
            let L = J * (H.powf(K));
            let AHO = (AHN * (K * (H.powf((K - AFG))))) * J;
            let O = M * (H.powf(N));
            let AHP = (AHN * (N * (H.powf((N - AFG))))) * M;
            let R = P * (H.powf(Q));
            let AHQ = (AHN * (Q * (H.powf((Q - AFG))))) * P;
            let U = S * (H.powf(T));
            let AHR = (AHN * (T * (H.powf((T - AFG))))) * S;
            let X = V * (H.powf(W));
            let AHS = (AHN * (W * (H.powf((W - AFG))))) * V;
            let AA = Y * (H.powf(Z));
            let AHT = (AHN * (Z * (H.powf((Z - AFG))))) * Y;
            let AD = AB * (H.powf(AC));
            let AHU = (AHN * (AC * (H.powf((AC - AFG))))) * AB;
            let AG = AE * (H.powf(AF));
            let AHV = (AHN * (AF * (H.powf((AF - AFG))))) * AE;
            let AJ = H.powf(AI);
            let AHW = AHN * (AI * (H.powf((AI - AFG))));
            let AK = -parameters[71];
            let AM = AL - H;
            let AHY = AHN * AHX;
            let AN = (AK * AM) / G;
            let AO = AN.exp();
            let AHZ = (((AHY * AK) - (AHM * AN)) / G) * AO;
            let AP = AJ * AO;
            let AR = AL / AQ;
            let AS = AH * (AP.powf(AR));
            let AIA = (((AHW * AO) + (AHZ * AJ)) * (AR * (AP.powf((AR - AFG))))) * AH;
            let AV = H.powf(AU);
            let AW = -parameters[96];
            let AX = (AW * AM) / G;
            let AY = AX.exp();
            let AZ = AV * AY;
            let BB = AL / BA;
            let BC = AT * (AZ.powf(BB));
            let AIB = ((((AHN * (AU * (H.powf((AU - AFG))))) * AY) + (((((AHY * AW) - (AHM * AX)) / G) * AY) * AV)) * (BB * (AZ.powf((BB - AFG))))) * AT;
            let BE = -parameters[97];
            let BF = (BE * AM) / G;
            let BG = BF.exp();
            let BH = AJ * BG;
            let BJ = AL / BI;
            let BK = BD * (BH.powf(BJ));
            let AIC = (((AHW * BG) + (((((AHY * BE) - (AHM * BF)) / G) * BG) * AJ)) * (BJ * (BH.powf((BJ - AFG))))) * BD;
            let BN = H.powf(BM);
            let AID = AHN * (BM * (H.powf((BM - AFG))));
            let BP = -BO;
            let BQ = (BP * AM) / G;
            let BR = BQ.exp();
            let BS = BN * BR;
            let BU = AL / BT;
            let BV = BL * (BS.powf(BU));
            let AIE = (((AID * BR) + (((((AHY * BP) - (AHM * BQ)) / G) * BR) * BN)) * (BU * (BS.powf((BU - AFG))))) * BL;
            let BY = H.powf(BX);
            let AIF = AHN * (BX * (H.powf((BX - AFG))));
            let BZ = -parameters[75];
            let CA = (BZ * AM) / G;
            let CB = CA.exp();
            let CC = BY * CB;
            let CE = AL / CD;
            let CF = BW * (CC.powf(CE));
            let AIG = (((AIF * CB) + (((((AHY * BZ) - (AHM * CA)) / G) * CB) * BY)) * (CE * (CC.powf((CE - AFG))))) * BW;
            let CI = -CH;
            let CJ = (CI * AM) / G;
            let CK = CJ.exp();
            let CL = BN * CK;
            let AIH = AID * CK;
            let AII = ((((AHY * CI) - (AHM * CJ)) / G) * CK) * BN;
            let CN = AL / CM;
            let CO = CL.powf(CN);
            let AIJ = CN * (CL.powf((CN - AFG)));
            let CP = CG * CO;
            let AIK = ((AIH + AII) * AIJ) * CG;
            let CR = -parameters[76];
            let CS = (CR * AM) / G;
            let CT = CS.exp();
            let CU = BY * CT;
            let AIL = AIF * CT;
            let AIM = ((((AHY * CR) - (AHM * CS)) / G) * CT) * BY;
            let CW = AL / CV;
            let CX = CU.powf(CW);
            let AIN = CW * (CU.powf((CW - AFG)));
            let CY = CQ * CX;
            let AIO = ((AIL + AIM) * AIN) * CQ;
            let DA = CZ * CO;
            let AIP = ((AIH + AII) * AIJ) * CZ;
            let DC = DB * CX;
            let AIQ = ((AIL + AIM) * AIN) * DB;
            let DF = -DE;
            let DG = (DF * AM) / G;
            let DH = DG.exp();
            let DI = BN * DH;
            let DK = AL / DJ;
            let DL = DD * (DI.powf(DK));
            let AIR = (((AID * DH) + (((((AHY * DF) - (AHM * DG)) / G) * DH) * BN)) * (DK * (DI.powf((DK - AFG))))) * DD;
            let DN = -parameters[77];
            let DO = (DN * AM) / G;
            let DP = DO.exp();
            let DQ = BY * DP;
            let DS = AL / DR;
            let DT = DM * (DQ.powf(DS));
            let AIS = (((AIF * DP) + (((((AHY * DN) - (AHM * DO)) / G) * DP) * BY)) * (DS * (DQ.powf((DS - AFG))))) * DM;
            let AIT = AFH * DU;
            let DV = AL + (I * DU);
            let DW = AQ * DV;
            let AIU = AIT * AQ;
            let DX = BA * DV;
            let AIV = AIT * BA;
            let EA = DY * (AL + (I * DZ));
            let AIW = (AFH * DZ) * DY;
            let ED = parameters[101] + (I * EC);
            let EG = EE * (AL + (I * EF));
            let EI = G / H;
            let EJ = EH * EI;
            let AIX = ((AHM - (AHN * EI)) / H) * EH;
            let EM = EK * EL;
            let EN = (EM * H) / G;
            let EO = EN.exp();
            let EP = -5e-1f64 * EL;
            let EQ = (EP * H) / G;
            let ER = EQ.exp();
            let ES = EO - ER;
            let ET = ES.ln();
            let EU = EJ * ET;
            let EW = EV * G;
            let EX = H.ln();
            let EY = EW * EX;
            let AIY = ((AHM * EV) * EX) + ((AHN * (AFG / H)) * EW);
            let EZ = H - AL;
            let FA = ((EU * H) - EY) - (BO * EZ);
            let AIZ = (((((AIX * ET) + (((((((AHN * EM) - (AHM * EN)) / G) * EO) - ((((AHN * EP) - (AHM * EQ)) / G) * ER)) * (AFG / ES)) * EJ)) * H) + (AHN * EU)) - AIY) - (AHN * BO);
            let FB = EH * G;
            let AJA = AHM * EH;
            let FD = (-FA) / G;
            let FE = FD.exp();
            let FF = (AL + (FC * FE)).sqrt();
            let FG = EK * (AL + FF);
            let FH = FG.ln();
            let FI = FA + (FB * FH);
            let AJC = AIZ + ((AJA * FH) + (((((((((AIZ * AHX) - (AHM * FD)) / G) * FE) * FC) * (AFG / (AJB * FF))) * EK) * (AFG / FG)) * FB));
            let FK = EK * FJ;
            let FL = (FK * H) / G;
            let FM = FL.exp();
            let FN = -5e-1f64 * FJ;
            let FO = (FN * H) / G;
            let FP = FO.exp();
            let FQ = FM - FP;
            let FR = FQ.ln();
            let FS = EJ * FR;
            let FT = ((FS * H) - EY) - (CH * EZ);
            let AJD = (((((AIX * FR) + (((((((AHN * FK) - (AHM * FL)) / G) * FM) - ((((AHN * FN) - (AHM * FO)) / G) * FP)) * (AFG / FQ)) * EJ)) * H) + (AHN * FS)) - AIY) - (AHN * CH);
            let FU = (-FT) / G;
            let FV = FU.exp();
            let FW = (AL + (FC * FV)).sqrt();
            let FX = EK * (AL + FW);
            let FY = FX.ln();
            let FZ = FT + (FB * FY);
            let AJE = AJD + ((AJA * FY) + (((((((((AJD * AHX) - (AHM * FU)) / G) * FV) * FC) * (AFG / (AJB * FW))) * EK) * (AFG / FX)) * FB));
            let GB = EK * GA;
            let GC = (GB * H) / G;
            let GD = GC.exp();
            let GE = -5e-1f64 * GA;
            let GF = (GE * H) / G;
            let GG = GF.exp();
            let GH = GD - GG;
            let GI = GH.ln();
            let GJ = EJ * GI;
            let GK = ((GJ * H) - EY) - (DE * EZ);
            let AJF = (((((AIX * GI) + (((((((AHN * GB) - (AHM * GC)) / G) * GD) - ((((AHN * GE) - (AHM * GF)) / G) * GG)) * (AFG / GH)) * EJ)) * H) + (AHN * GJ)) - AIY) - (AHN * DE);
            let GL = (-GK) / G;
            let GM = GL.exp();
            let GN = (AL + (FC * GM)).sqrt();
            let GO = EK * (AL + GN);
            let GP = GO.ln();
            let GQ = GK + (FB * GP);
            let AJG = AJF + ((AJA * GP) + (((((((((AJF * AHX) - (AHM * GL)) / G) * GM) * FC) * (AFG / (AJB * GN))) * EK) * (AFG / GO)) * FB));
            let GS = EL / FI;
            let GU = GR * (GS.powf(GT));
            let AJH = ((((AJC * GS) * AHX) / FI) * (GT * (GS.powf((GT - AFG))))) * GR;
            let GW = FJ / FZ;
            let GY = GW.powf(GX);
            let AJI = (((AJE * GW) * AHX) / FZ) * (GX * (GW.powf((GX - AFG))));
            let GZ = GV * GY;
            let AJJ = AJI * GV;
            let HB = HA * GY;
            let AJK = AJI * HA;
            let HD = GA / GQ;
            let HF = HC * (HD.powf(HE));
            let AJL = ((((AJG * HD) * AHX) / GQ) * (HE * (HD.powf((HE - AFG))))) * HC;
            let HH = HG * AJ;
            let HI = HH * AO;
            let AJM = ((AHW * HG) * AO) + (AHZ * HH);
            let HL = HJ * (H.powf(HK));
            let AJN = (AHN * (HK * (H.powf((HK - AFG))))) * HJ;
            let HM = -(EB * (AL + (I * ED)));
            let AJO = (((AFH * ED) + ((AFH * EC) * I)) * EB) * AHX;
            let HN = EG * G;
            let AJP = (((AFH * EF) * EE) * G) + (AHM * EG);
            let HO = HM / HN;
            let HP = HO.exp();
            let AJQ = ((AJO - (AJP * HO)) / HN) * HP;
            let HR = if HQ > A { 1.0 } else { 0.0 };
            let HT = if HR != 0.0 {
                let HS = AL / HQ;
                HS
            } else {
                A
            };
            let HV = if HU > A { 1.0 } else { 0.0 };
            let HX = if HV != 0.0 {
                let HW = AL / HU;
                HW
            } else {
                A
            };
            let HY = if J > A { 1.0 } else { 0.0 };
            let IA;
            let AFT;
            if HY != 0.0 {
                let HZ = AL / L;
                let AJS = ((AHO * HZ) * AHX) / L;
                IA = HZ;
                AFT = AJS;
            } else {
                IA = A;
                AFT = AJR;
            }
            let IC = if IB > A { 1.0 } else { 0.0 };
            let IE = if IC != 0.0 {
                let ID = AL / IB;
                ID
            } else {
                A
            };
            let IG = if IF > A { 1.0 } else { 0.0 };
            let II = if IG != 0.0 {
                let IH = AL / IF;
                IH
            } else {
                A
            };
            let IJ = if HJ > A { 1.0 } else { 0.0 };
            let IL;
            let AFU;
            if IJ != 0.0 {
                let IK = AL / HL;
                let AJT = ((AJN * IK) * AHX) / HL;
                IL = IK;
                AFU = AJT;
            } else {
                IL = A;
                AFU = AJR;
            }
            let IN = if IM > A { 1.0 } else { 0.0 };
            let IP = if IN != 0.0 {
                let IO = AL / IM;
                IO
            } else {
                A
            };
            let IR = if IQ > A { 1.0 } else { 0.0 };
            let IT = if IR != 0.0 {
                let IS = AL / IQ;
                IS
            } else {
                A
            };
            let IV = if IU > A { 1.0 } else { 0.0 };
            let IX = if IV != 0.0 {
                let IW = AL / IU;
                IW
            } else {
                A
            };
            let IY = if IV != 0.0 {
                A
            } else {
                AL
            };
            let JB = IZ - JA;
            let AJU = Lanes([AFI, 0.0]) - Lanes([0.0, AFJ]);
            let JD = JC - JA;
            let AJV = Lanes([AFK, 0.0]) - Lanes([0.0, AFJ]);
            let JF = IZ - JE;
            let AJW = Lanes([0.0, AFI]) - Lanes([AFL, 0.0]);
            let JH = IZ - JG;
            let AJX = Lanes([0.0, AFI]) - Lanes([AFM, 0.0]);
            let JJ = JC - JI;
            let AJY = Lanes([AFK, 0.0]) - Lanes([0.0, AFN]);
            let JK = -FI;
            let AJZ = AJC * AHX;
            let JM = JK * JL;
            let AKA = AJZ * JL;
            let JO = if JN <= A { 1.0 } else { 0.0 };
            let VD;
            let AFV;
            if JO != 0.0 {
                let JP = JB + JM;
                let AKK = Lanes([0.0, AJU[0], AJU[1]]);
                let AKL = AKK + Lanes([AKA, 0.0, 0.0]);
                let JQ = if JP > A { 1.0 } else { 0.0 };
                let KF;
                let KG;
                let AFW;
                let AFX;
                if JQ != 0.0 {
                    let JR = AL - JL;
                    let JS = JR.powf((-1e0f64 - GT));
                    let JT = AL - ((JS * JR) * JR);
                    let JU = AL - GT;
                    let JV = (FI * JT) / JU;
                    let JW = EK * GT;
                    let JX = (JW * JP) / FI;
                    let JY = JR + JX;
                    let JZ = (JP * JY) * JS;
                    let AKO = ((AKL * JY) + ((((AKL * JW) - Lanes([(AJC * JX), 0.0, 0.0])) / FI) * JP)) * JS;
                    let AKP = Lanes([((AJC * JT) / JU), 0.0, 0.0]);
                    KF = JV;
                    KG = JZ;
                    AFW = AKP;
                    AFX = AKO;
                } else {
                    let KA = JB / FI;
                    let KB = AL - KA;
                    let KC = AL - GT;
                    let KD = AL - (KB.powf(KC));
                    let KE = (FI * KD) / KC;
                    let AKM = (Lanes([(AJC * KD), 0.0, 0.0]) + ((((((AKK - Lanes([(AJC * KA), 0.0, 0.0])) / FI) * AHX) * (KC * (KB.powf((KC - AFG))))) * AHX) * FI)) / KC;
                    KF = KE;
                    KG = A;
                    AFW = AKM;
                    AFX = AKN;
                }
                let KH = KF + KG;
                let AKQ = AFW + AFX;
                VD = KH;
                AFV = AKQ;
            } else {
                let AKB = AKA * JM;
                let KI = (FC * JN) * JN;
                let KJ = ((JM * JM) + KI).sqrt();
                let KL = KK * (JM + KJ);
                let AKC = (AKA + ((AKB + AKB) * (AFG / (AJB * KJ)))) * KK;
                let KM = KL / FI;
                let KN = AL - KM;
                let KO = AL - GT;
                let KP = KN.powf(KO);
                let AKD = KO - AFG;
                let KQ = JB + JM;
                let AKE = Lanes([0.0, AJU[0], AJU[1]]);
                let AKF = Lanes([AKA, 0.0, 0.0]);
                let AKG = AKE + AKF;
                let AKH = AKG * KQ;
                let KR = ((KQ * KQ) + KI).sqrt();
                let KS = (EK * (KQ - KR)) - JM;
                let AKI = ((AKG - ((AKH + AKH) * (AFG / (AJB * KR)))) * EK) - AKF;
                let KT = KS / FI;
                let KU = AL - KT;
                let KV = KU.powf(KO);
                let KW = (AL - JL).powf((-GT));
                let KX = (((JK * KV) / KO) + (KW * ((JB - KS) + KL))) - ((JK * KP) / KO);
                let AKJ = (((Lanes([(AJZ * KV), 0.0, 0.0]) + (((((AKI - Lanes([(AJC * KT), 0.0, 0.0])) / FI) * AHX) * (KO * (KU.powf(AKD)))) * JK)) / KO) + (((AKE - AKI) + Lanes([AKC, 0.0, 0.0])) * KW)) - Lanes([(((AJZ * KP) + (((((AKC - (AJC * KM)) / FI) * AHX) * (KO * (KN.powf(AKD)))) * JK)) / KO), 0.0, 0.0]);
                VD = KX;
                AFV = AKJ;
            }
            let ACU;
            let AFY;
            if JO != 0.0 {
                let KY = JD + JM;
                let ALA = Lanes([0.0, AJV[0], AJV[1]]);
                let ALB = ALA + Lanes([AKA, 0.0, 0.0]);
                let KZ = if KY > A { 1.0 } else { 0.0 };
                let LO;
                let LP;
                let AFZ;
                let AGA;
                if KZ != 0.0 {
                    let LA = AL - JL;
                    let LB = LA.powf((-1e0f64 - GT));
                    let LC = AL - ((LB * LA) * LA);
                    let LD = AL - GT;
                    let LE = (FI * LC) / LD;
                    let LF = EK * GT;
                    let LG = (LF * KY) / FI;
                    let LH = LA + LG;
                    let LI = (KY * LH) * LB;
                    let ALE = ((ALB * LH) + ((((ALB * LF) - Lanes([(AJC * LG), 0.0, 0.0])) / FI) * KY)) * LB;
                    let ALF = Lanes([((AJC * LC) / LD), 0.0, 0.0]);
                    LO = LE;
                    LP = LI;
                    AFZ = ALF;
                    AGA = ALE;
                } else {
                    let LJ = JD / FI;
                    let LK = AL - LJ;
                    let LL = AL - GT;
                    let LM = AL - (LK.powf(LL));
                    let LN = (FI * LM) / LL;
                    let ALC = (Lanes([(AJC * LM), 0.0, 0.0]) + ((((((ALA - Lanes([(AJC * LJ), 0.0, 0.0])) / FI) * AHX) * (LL * (LK.powf((LL - AFG))))) * AHX) * FI)) / LL;
                    LO = LN;
                    LP = A;
                    AFZ = ALC;
                    AGA = ALD;
                }
                let LQ = LO + LP;
                let ALG = AFZ + AGA;
                ACU = LQ;
                AFY = ALG;
            } else {
                let AKR = AKA * JM;
                let LR = (FC * JN) * JN;
                let LS = ((JM * JM) + LR).sqrt();
                let LU = LT * (JM + LS);
                let AKS = (AKA + ((AKR + AKR) * (AFG / (AJB * LS)))) * LT;
                let LV = LU / FI;
                let LW = AL - LV;
                let LX = AL - GT;
                let LY = LW.powf(LX);
                let AKT = LX - AFG;
                let LZ = JD + JM;
                let AKU = Lanes([0.0, AJV[0], AJV[1]]);
                let AKV = Lanes([AKA, 0.0, 0.0]);
                let AKW = AKU + AKV;
                let AKX = AKW * LZ;
                let MA = ((LZ * LZ) + LR).sqrt();
                let MB = (EK * (LZ - MA)) - JM;
                let AKY = ((AKW - ((AKX + AKX) * (AFG / (AJB * MA)))) * EK) - AKV;
                let MC = MB / FI;
                let MD = AL - MC;
                let ME = MD.powf(LX);
                let MF = (AL - JL).powf((-GT));
                let MG = (((JK * ME) / LX) + (MF * ((JD - MB) + LU))) - ((JK * LY) / LX);
                let AKZ = (((Lanes([(AJZ * ME), 0.0, 0.0]) + (((((AKY - Lanes([(AJC * MC), 0.0, 0.0])) / FI) * AHX) * (LX * (MD.powf(AKT)))) * JK)) / LX) + (((AKU - AKY) + Lanes([AKS, 0.0, 0.0])) * MF)) - Lanes([(((AJZ * LY) + (((((AKS - (AJC * LV)) / FI) * AHX) * (LX * (LW.powf(AKT)))) * JK)) / LX), 0.0, 0.0]);
                ACU = MG;
                AFY = AKZ;
            }
            let MH = -FZ;
            let ALH = AJE * AHX;
            let MI = MH * JL;
            let ALI = ALH * JL;
            let MK = if MJ <= A { 1.0 } else { 0.0 };
            let VE;
            let AGB;
            if MK != 0.0 {
                let ML = JF + MI;
                let AMI = Lanes([0.0, AJW[0], AJW[1]]);
                let AMJ = AMI + Lanes([ALI, 0.0, 0.0]);
                let MM = if ML > A { 1.0 } else { 0.0 };
                let NM;
                let NO;
                let AGC;
                let AGD;
                if MM != 0.0 {
                    let MN = AL - JL;
                    let MO = MN.powf((-1e0f64 - GX));
                    let MP = AL - ((MO * MN) * MN);
                    let MQ = AL - GX;
                    let MR = (FZ * MP) / MQ;
                    let MS = EK * GX;
                    let MT = (MS * ML) / FZ;
                    let MU = MN + MT;
                    let MV = (ML * MU) * MO;
                    let AMO = ((AMJ * MU) + ((((AMJ * MS) - Lanes([(AJE * MT), 0.0, 0.0])) / FZ) * ML)) * MO;
                    let AMP = Lanes([((AJE * MP) / MQ), 0.0, 0.0]);
                    NM = MR;
                    NO = MV;
                    AGC = AMP;
                    AGD = AMO;
                } else {
                    let MX = if (if MW > A { 1.0 } else { 0.0 }) != 0.0 && (if JF < (-MW) { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let NN;
                    let AGE;
                    if MX != 0.0 {
                        let MY = MW / FZ;
                        let MZ = AL + MY;
                        let NA = AL - GX;
                        let NB = MZ.powf(NA);
                        let AML = AJW * NA;
                        let NC = FZ + MW;
                        let ND = (NA * (JF + MW)) / NC;
                        let NE = AL - ND;
                        let NF = AL - (NB * NE);
                        let NG = (FZ * NF) / NA;
                        let AMM = (Lanes([(AJE * NF), 0.0, 0.0]) + (((Lanes([(((((AJE * MY) * AHX) / FZ) * (NA * (MZ.powf((NA - AFG))))) * NE), 0.0, 0.0]) + ((((Lanes([0.0, AML[0], AML[1]]) - Lanes([(AJE * ND), 0.0, 0.0])) / NC) * AHX) * NB)) * AHX) * FZ)) / NA;
                        NN = NG;
                        AGE = AMM;
                    } else {
                        let NH = JF / FZ;
                        let NI = AL - NH;
                        let NJ = AL - GX;
                        let NK = AL - (NI.powf(NJ));
                        let NL = (FZ * NK) / NJ;
                        let AMK = (Lanes([(AJE * NK), 0.0, 0.0]) + ((((((AMI - Lanes([(AJE * NH), 0.0, 0.0])) / FZ) * AHX) * (NJ * (NI.powf((NJ - AFG))))) * AHX) * FZ)) / NJ;
                        NN = NL;
                        AGE = AMK;
                    }
                    NM = NN;
                    NO = A;
                    AGC = AGE;
                    AGD = AMN;
                }
                let NP = NM + NO;
                let AMQ = AGC + AGD;
                VE = NP;
                AGB = AMQ;
            } else {
                let NR = if (if MW > A { 1.0 } else { 0.0 }) != 0.0 && (if NQ > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let VF;
                let AGF;
                if NR != 0.0 {
                    let NS = MW - MI;
                    let ALS = ALI * AHX;
                    let NT = (MW + MI) / NS;
                    let ALT = (ALI - (ALS * NT)) / NS;
                    let NU = NT - AL;
                    let ALU = ALT * NU;
                    let NV = (FC * MJ) * MJ;
                    let NW = ((NU * NU) + NV).sqrt();
                    let NX = NT + AL;
                    let ALV = ALT * NX;
                    let NY = (FC * NQ) * NQ;
                    let NZ = ((NX * NX) + NY).sqrt();
                    let OA = NW + NZ;
                    let OB = (EH * NT) / OA;
                    let OC = EK * (((OB * NS) - MW) - MI);
                    let ALW = ((((((ALT * EH) - ((((ALU + ALU) * (AFG / (AJB * NW))) + ((ALV + ALV) * (AFG / (AJB * NZ)))) * OB)) / OA) * NS) + (ALS * OB)) - ALI) * EK;
                    let OD = OC / FZ;
                    let OE = AL - OD;
                    let OF = AL - GX;
                    let ALX = OF - AFG;
                    let OG = AL - (OE.powf(OF));
                    let ALY = AJW * EH;
                    let ALZ = Lanes([ALI, 0.0, 0.0]);
                    let OH = (((EH * JF) + MW) + MI) / NS;
                    let AMA = ((Lanes([0.0, ALY[0], ALY[1]]) + ALZ) - Lanes([(ALS * OH), 0.0, 0.0])) / NS;
                    let OI = OH - AL;
                    let AMB = AMA * OI;
                    let OJ = ((OI * OI) + NV).sqrt();
                    let OK = OH + AL;
                    let AMC = AMA * OK;
                    let OL = ((OK * OK) + NY).sqrt();
                    let OM = OJ + OL;
                    let ON = (EH * OH) / OM;
                    let AMD = ((AMA * EH) - ((((AMB + AMB) * (AFG / (AJB * OJ))) + ((AMC + AMC) * (AFG / (AJB * OL)))) * ON)) / OM;
                    let OO = EK * (((ON * NS) - MW) - MI);
                    let AME = (((AMD * NS) + Lanes([(ALS * ON), 0.0, 0.0])) - ALZ) * EK;
                    let OP = OO / FZ;
                    let OQ = AL - OP;
                    let OR = AL - (OQ.powf(OF));
                    let OS = EK * (ON + AL);
                    let AMF = AMD * EK;
                    let OT = MW / FZ;
                    let OU = AL + OT;
                    let OV = -GX;
                    let OW = OU.powf(OV);
                    let AMG = OV - AFG;
                    let OX = MI / FZ;
                    let OY = AL + OX;
                    let OZ = OY.powf(OV);
                    let PA = AL - OS;
                    let PB = (PA * OW) + (OS * OZ);
                    let PC = (JF - OO) + OC;
                    let PD = ((PC * PB) + ((FZ * OR) / OF)) - ((FZ * OG) / OF);
                    let AMH = (((((Lanes([0.0, AJW[0], AJW[1]]) - AME) + Lanes([ALW, 0.0, 0.0])) * PB) + (((((AMF * AHX) * OW) + Lanes([(((((AJE * OT) * AHX) / FZ) * (OV * (OU.powf(AMG)))) * PA), 0.0, 0.0])) + ((AMF * OZ) + Lanes([((((ALI - (AJE * OX)) / FZ) * (OV * (OY.powf(AMG)))) * OS), 0.0, 0.0]))) * PC)) + ((Lanes([(AJE * OR), 0.0, 0.0]) + ((((((AME - Lanes([(AJE * OP), 0.0, 0.0])) / FZ) * AHX) * (OF * (OQ.powf(ALX)))) * AHX) * FZ)) / OF)) - Lanes([(((AJE * OG) + ((((((ALW - (AJE * OD)) / FZ) * AHX) * (OF * (OE.powf(ALX)))) * AHX) * FZ)) / OF), 0.0, 0.0]);
                    VF = PD;
                    AGF = AMH;
                } else {
                    let ALJ = ALI * MI;
                    let PE = (FC * MJ) * MJ;
                    let PF = ((MI * MI) + PE).sqrt();
                    let PH = PG * (MI + PF);
                    let ALK = (ALI + ((ALJ + ALJ) * (AFG / (AJB * PF)))) * PG;
                    let PI = PH / FZ;
                    let PJ = AL - PI;
                    let PK = AL - GX;
                    let PL = PJ.powf(PK);
                    let ALL = PK - AFG;
                    let PM = JF + MI;
                    let ALM = Lanes([0.0, AJW[0], AJW[1]]);
                    let ALN = Lanes([ALI, 0.0, 0.0]);
                    let ALO = ALM + ALN;
                    let ALP = ALO * PM;
                    let PN = ((PM * PM) + PE).sqrt();
                    let PO = (EK * (PM - PN)) - MI;
                    let ALQ = ((ALO - ((ALP + ALP) * (AFG / (AJB * PN)))) * EK) - ALN;
                    let PP = PO / FZ;
                    let PQ = AL - PP;
                    let PR = PQ.powf(PK);
                    let PS = (AL - JL).powf((-GX));
                    let PT = (((MH * PR) / PK) + (PS * ((JF - PO) + PH))) - ((MH * PL) / PK);
                    let ALR = (((Lanes([(ALH * PR), 0.0, 0.0]) + (((((ALQ - Lanes([(AJE * PP), 0.0, 0.0])) / FZ) * AHX) * (PK * (PQ.powf(ALL)))) * MH)) / PK) + (((ALM - ALQ) + Lanes([ALK, 0.0, 0.0])) * PS)) - Lanes([(((ALH * PL) + (((((ALK - (AJE * PI)) / FZ) * AHX) * (PK * (PJ.powf(ALL)))) * MH)) / PK), 0.0, 0.0]);
                    VF = PT;
                    AGF = ALR;
                }
                VE = VF;
                AGB = AGF;
            }
            let ADB;
            let AGG;
            if MK != 0.0 {
                let PU = JJ + MI;
                let ANQ = Lanes([0.0, AJY[0], AJY[1]]);
                let ANR = ANQ + Lanes([ALI, 0.0, 0.0]);
                let PV = if PU > A { 1.0 } else { 0.0 };
                let QU;
                let QW;
                let AGH;
                let AGI;
                if PV != 0.0 {
                    let PW = AL - JL;
                    let PX = PW.powf((-1e0f64 - GX));
                    let PY = AL - ((PX * PW) * PW);
                    let PZ = AL - GX;
                    let QA = (FZ * PY) / PZ;
                    let QB = EK * GX;
                    let QC = (QB * PU) / FZ;
                    let QD = PW + QC;
                    let QE = (PU * QD) * PX;
                    let ANW = ((ANR * QD) + ((((ANR * QB) - Lanes([(AJE * QC), 0.0, 0.0])) / FZ) * PU)) * PX;
                    let ANX = Lanes([((AJE * PY) / PZ), 0.0, 0.0]);
                    QU = QA;
                    QW = QE;
                    AGH = ANX;
                    AGI = ANW;
                } else {
                    let QF = if (if MW > A { 1.0 } else { 0.0 }) != 0.0 && (if JJ < (-MW) { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let QV;
                    let AGJ;
                    if QF != 0.0 {
                        let QG = MW / FZ;
                        let QH = AL + QG;
                        let QI = AL - GX;
                        let QJ = QH.powf(QI);
                        let ANT = AJY * QI;
                        let QK = FZ + MW;
                        let QL = (QI * (JJ + MW)) / QK;
                        let QM = AL - QL;
                        let QN = AL - (QJ * QM);
                        let QO = (FZ * QN) / QI;
                        let ANU = (Lanes([(AJE * QN), 0.0, 0.0]) + (((Lanes([(((((AJE * QG) * AHX) / FZ) * (QI * (QH.powf((QI - AFG))))) * QM), 0.0, 0.0]) + ((((Lanes([0.0, ANT[0], ANT[1]]) - Lanes([(AJE * QL), 0.0, 0.0])) / QK) * AHX) * QJ)) * AHX) * FZ)) / QI;
                        QV = QO;
                        AGJ = ANU;
                    } else {
                        let QP = JJ / FZ;
                        let QQ = AL - QP;
                        let QR = AL - GX;
                        let QS = AL - (QQ.powf(QR));
                        let QT = (FZ * QS) / QR;
                        let ANS = (Lanes([(AJE * QS), 0.0, 0.0]) + ((((((ANQ - Lanes([(AJE * QP), 0.0, 0.0])) / FZ) * AHX) * (QR * (QQ.powf((QR - AFG))))) * AHX) * FZ)) / QR;
                        QV = QT;
                        AGJ = ANS;
                    }
                    QU = QV;
                    QW = A;
                    AGH = AGJ;
                    AGI = ANV;
                }
                let QX = QU + QW;
                let ANY = AGH + AGI;
                ADB = QX;
                AGG = ANY;
            } else {
                let QY = if (if MW > A { 1.0 } else { 0.0 }) != 0.0 && (if NQ > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let ADC;
                let AGK;
                if QY != 0.0 {
                    let QZ = MW - MI;
                    let ANA = ALI * AHX;
                    let RA = (MW + MI) / QZ;
                    let ANB = (ALI - (ANA * RA)) / QZ;
                    let RB = RA - AL;
                    let ANC = ANB * RB;
                    let RC = (FC * MJ) * MJ;
                    let RD = ((RB * RB) + RC).sqrt();
                    let RE = RA + AL;
                    let AND = ANB * RE;
                    let RF = (FC * NQ) * NQ;
                    let RG = ((RE * RE) + RF).sqrt();
                    let RH = RD + RG;
                    let RI = (EH * RA) / RH;
                    let RJ = EK * (((RI * QZ) - MW) - MI);
                    let ANE = ((((((ANB * EH) - ((((ANC + ANC) * (AFG / (AJB * RD))) + ((AND + AND) * (AFG / (AJB * RG)))) * RI)) / RH) * QZ) + (ANA * RI)) - ALI) * EK;
                    let RK = RJ / FZ;
                    let RL = AL - RK;
                    let RM = AL - GX;
                    let ANF = RM - AFG;
                    let RN = AL - (RL.powf(RM));
                    let ANG = AJY * EH;
                    let ANH = Lanes([ALI, 0.0, 0.0]);
                    let RO = (((EH * JJ) + MW) + MI) / QZ;
                    let ANI = ((Lanes([0.0, ANG[0], ANG[1]]) + ANH) - Lanes([(ANA * RO), 0.0, 0.0])) / QZ;
                    let RP = RO - AL;
                    let ANJ = ANI * RP;
                    let RQ = ((RP * RP) + RC).sqrt();
                    let RR = RO + AL;
                    let ANK = ANI * RR;
                    let RS = ((RR * RR) + RF).sqrt();
                    let RT = RQ + RS;
                    let RU = (EH * RO) / RT;
                    let ANL = ((ANI * EH) - ((((ANJ + ANJ) * (AFG / (AJB * RQ))) + ((ANK + ANK) * (AFG / (AJB * RS)))) * RU)) / RT;
                    let RV = EK * (((RU * QZ) - MW) - MI);
                    let ANM = (((ANL * QZ) + Lanes([(ANA * RU), 0.0, 0.0])) - ANH) * EK;
                    let RW = RV / FZ;
                    let RX = AL - RW;
                    let RY = AL - (RX.powf(RM));
                    let RZ = EK * (RU + AL);
                    let ANN = ANL * EK;
                    let SA = MW / FZ;
                    let SB = AL + SA;
                    let SC = -GX;
                    let SD = SB.powf(SC);
                    let ANO = SC - AFG;
                    let SE = MI / FZ;
                    let SF = AL + SE;
                    let SG = SF.powf(SC);
                    let SH = AL - RZ;
                    let SI = (SH * SD) + (RZ * SG);
                    let SJ = (JJ - RV) + RJ;
                    let SK = ((SJ * SI) + ((FZ * RY) / RM)) - ((FZ * RN) / RM);
                    let ANP = (((((Lanes([0.0, AJY[0], AJY[1]]) - ANM) + Lanes([ANE, 0.0, 0.0])) * SI) + (((((ANN * AHX) * SD) + Lanes([(((((AJE * SA) * AHX) / FZ) * (SC * (SB.powf(ANO)))) * SH), 0.0, 0.0])) + ((ANN * SG) + Lanes([((((ALI - (AJE * SE)) / FZ) * (SC * (SF.powf(ANO)))) * RZ), 0.0, 0.0]))) * SJ)) + ((Lanes([(AJE * RY), 0.0, 0.0]) + ((((((ANM - Lanes([(AJE * RW), 0.0, 0.0])) / FZ) * AHX) * (RM * (RX.powf(ANF)))) * AHX) * FZ)) / RM)) - Lanes([(((AJE * RN) + ((((((ANE - (AJE * RK)) / FZ) * AHX) * (RM * (RL.powf(ANF)))) * AHX) * FZ)) / RM), 0.0, 0.0]);
                    ADC = SK;
                    AGK = ANP;
                } else {
                    let AMR = ALI * MI;
                    let SL = (FC * MJ) * MJ;
                    let SM = ((MI * MI) + SL).sqrt();
                    let SO = SN * (MI + SM);
                    let AMS = (ALI + ((AMR + AMR) * (AFG / (AJB * SM)))) * SN;
                    let SP = SO / FZ;
                    let SQ = AL - SP;
                    let SR = AL - GX;
                    let SS = SQ.powf(SR);
                    let AMT = SR - AFG;
                    let ST = JJ + MI;
                    let AMU = Lanes([0.0, AJY[0], AJY[1]]);
                    let AMV = Lanes([ALI, 0.0, 0.0]);
                    let AMW = AMU + AMV;
                    let AMX = AMW * ST;
                    let SU = ((ST * ST) + SL).sqrt();
                    let SV = (EK * (ST - SU)) - MI;
                    let AMY = ((AMW - ((AMX + AMX) * (AFG / (AJB * SU)))) * EK) - AMV;
                    let SW = SV / FZ;
                    let SX = AL - SW;
                    let SY = SX.powf(SR);
                    let SZ = (AL - JL).powf((-GX));
                    let TA = (((MH * SY) / SR) + (SZ * ((JJ - SV) + SO))) - ((MH * SS) / SR);
                    let AMZ = (((Lanes([(ALH * SY), 0.0, 0.0]) + (((((AMY - Lanes([(AJE * SW), 0.0, 0.0])) / FZ) * AHX) * (SR * (SX.powf(AMT)))) * MH)) / SR) + (((AMU - AMY) + Lanes([AMS, 0.0, 0.0])) * SZ)) - Lanes([(((ALH * SS) + (((((AMS - (AJE * SP)) / FZ) * AHX) * (SR * (SQ.powf(AMT)))) * MH)) / SR), 0.0, 0.0]);
                    ADC = TA;
                    AGK = AMZ;
                }
                ADB = ADC;
                AGG = AGK;
            }
            let TB = if HC > A { 1.0 } else { 0.0 };
            let ADF;
            let AGL;
            if TB != 0.0 {
                let TC = -GQ;
                let AOA = AJG * AHX;
                let TD = TC * JL;
                let AOB = AOA * JL;
                let TF = if TE <= A { 1.0 } else { 0.0 };
                let ADG;
                let AGM;
                if TF != 0.0 {
                    let TH = TG - JI;
                    let AOM = Lanes([0.0, AFO]) - Lanes([AFN, 0.0]);
                    let TI = TH + TD;
                    let AON = Lanes([0.0, AOM[0], AOM[1]]);
                    let AOO = AON + Lanes([AOB, 0.0, 0.0]);
                    let TJ = if TI > A { 1.0 } else { 0.0 };
                    let TY;
                    let TZ;
                    let AGN;
                    let AGO;
                    if TJ != 0.0 {
                        let TK = AL - JL;
                        let TL = TK.powf((-1e0f64 - HE));
                        let TM = AL - ((TL * TK) * TK);
                        let TN = AL - HE;
                        let TO = (GQ * TM) / TN;
                        let TP = EK * HE;
                        let TQ = (TP * TI) / GQ;
                        let TR = TK + TQ;
                        let TS = (TI * TR) * TL;
                        let AOQ = ((AOO * TR) + ((((AOO * TP) - Lanes([(AJG * TQ), 0.0, 0.0])) / GQ) * TI)) * TL;
                        let AOR = Lanes([((AJG * TM) / TN), 0.0, 0.0]);
                        TY = TO;
                        TZ = TS;
                        AGN = AOR;
                        AGO = AOQ;
                    } else {
                        let TT = TH / GQ;
                        let TU = AL - TT;
                        let TV = AL - HE;
                        let TW = AL - (TU.powf(TV));
                        let TX = (GQ * TW) / TV;
                        let AOP = (Lanes([(AJG * TW), 0.0, 0.0]) + ((((((AON - Lanes([(AJG * TT), 0.0, 0.0])) / GQ) * AHX) * (TV * (TU.powf((TV - AFG))))) * AHX) * GQ)) / TV;
                        TY = TX;
                        TZ = A;
                        AGN = AOP;
                        AGO = ANZ;
                    }
                    let UA = TY + TZ;
                    let AOS = AGN + AGO;
                    ADG = UA;
                    AGM = AOS;
                } else {
                    let AOC = AOB * TD;
                    let UB = (FC * TE) * TE;
                    let UC = ((TD * TD) + UB).sqrt();
                    let UE = UD * (TD + UC);
                    let AOD = (AOB + ((AOC + AOC) * (AFG / (AJB * UC)))) * UD;
                    let UF = UE / GQ;
                    let UG = AL - UF;
                    let UH = AL - HE;
                    let UI = UG.powf(UH);
                    let AOE = UH - AFG;
                    let UJ = TG - JI;
                    let AOF = Lanes([0.0, AFO]) - Lanes([AFN, 0.0]);
                    let UK = UJ + TD;
                    let AOG = Lanes([0.0, AOF[0], AOF[1]]);
                    let AOH = Lanes([AOB, 0.0, 0.0]);
                    let AOI = AOG + AOH;
                    let AOJ = AOI * UK;
                    let UL = ((UK * UK) + UB).sqrt();
                    let UM = (EK * (UK - UL)) - TD;
                    let AOK = ((AOI - ((AOJ + AOJ) * (AFG / (AJB * UL)))) * EK) - AOH;
                    let UN = UM / GQ;
                    let UO = AL - UN;
                    let UP = UO.powf(UH);
                    let UQ = (AL - JL).powf((-HE));
                    let UR = (((TC * UP) / UH) + (UQ * ((UJ - UM) + UE))) - ((TC * UI) / UH);
                    let AOL = (((Lanes([(AOA * UP), 0.0, 0.0]) + (((((AOK - Lanes([(AJG * UN), 0.0, 0.0])) / GQ) * AHX) * (UH * (UO.powf(AOE)))) * TC)) / UH) + (((AOG - AOK) + Lanes([AOD, 0.0, 0.0])) * UQ)) - Lanes([(((AOA * UI) + (((((AOD - (AJG * UF)) / GQ) * AHX) * (UH * (UG.powf(AOE)))) * TC)) / UH), 0.0, 0.0]);
                    ADG = UR;
                    AGM = AOL;
                }
                ADF = ADG;
                AGL = AGM;
            } else {
                ADF = A;
                AGL = ANZ;
            }
            let US = DW * G;
            let UT = JB / US;
            let AOT = Lanes([0.0, AJU[0], AJU[1]]);
            let UU = rspice_limexp(UT);
            let UV = UU - AL;
            let UW = AS * UV;
            let AOU = Lanes([(AIA * UV), 0.0, 0.0]) + ((((AOT - Lanes([(((AIU * G) + (AHM * DW)) * UT), 0.0, 0.0])) / US) * UU) * AS);
            let UX = DX * G;
            let UY = JF / UX;
            let AOV = Lanes([0.0, AJW[0], AJW[1]]);
            let UZ = rspice_limexp(UY);
            let VA = AS * BC;
            let VB = UZ - AL;
            let VC = VA * VB;
            let AOW = Lanes([(((AIA * BC) + (AIB * AS)) * VB), 0.0, 0.0]) + ((((AOV - Lanes([(((AIV * G) + (AHM * DX)) * UY), 0.0, 0.0])) / UX) * UZ) * VA);
            let AOX = AFV * HX;
            let AOY = AGB * HT;
            let VG = (AL + (VD * HX)) + (VE * HT);
            let AOZ = Lanes([AOX[0], 0.0, AOX[1], AOX[2]]) + Lanes([AOY[0], AOY[1], AOY[2], 0.0]);
            let VI = VG - VH;
            let APA = AOZ * VI;
            let VJ = ((VI * VI) + 1e-8f64).sqrt();
            let APB = (((APA + APA) * (AFG / (AJB * VJ))) + AOZ) * EK;
            let VK = (EK * ((VJ + VG) - VH)) + VH;
            let APC = (AOU * IA) + Lanes([(AFT * UW), 0.0, 0.0]);
            let APD = AOW * IE;
            let VL = (UW * IA) + (VC * IE);
            let APE = Lanes([APC[0], 0.0, APC[1], APC[2]]) + Lanes([APD[0], APD[1], APD[2], 0.0]);
            let VM = if parameters[88] < EK { 1.0 } else { 0.0 };
            let VV;
            let AGP;
            if VM != 0.0 {
                let VO = AL / VN;
                let VP = (VK.powf(VO)) + (FC * VL);
                let VQ = EK * (VK + (VP.powf(VN)));
                let APG = (APB + (((APB * (VO * (VK.powf((VO - AFG))))) + (APE * FC)) * (VN * (VP.powf((VN - AFG)))))) * EK;
                VV = VQ;
                AGP = APG;
            } else {
                let VR = EK * VK;
                let VS = AL + (FC * VL);
                let VT = AL + (VS.powf(VN));
                let VU = VR * VT;
                let APF = ((APB * EK) * VT) + (((APE * FC) * (VN * (VS.powf((VN - AFG))))) * VR);
                VV = VU;
                AGP = APF;
            }
            let VW = VC / VV;
            let APH = (Lanes([AOW[0], AOW[1], AOW[2], 0.0]) - (AGP * VW)) / VV;
            let VX = UW / VV;
            let API = (Lanes([AOU[0], 0.0, AOU[1], AOU[2]]) - (AGP * VX)) / VV;
            let VY = if BD > A { 1.0 } else { 0.0 };
            let ABN;
            let ADD;
            let AEC;
            let AGQ;
            let AGR;
            let AGS;
            if VY != 0.0 {
                let VZ = BI * G;
                let APL = AHM * BI;
                let WA = JJ / VZ;
                let WB = rspice_limexp(WA);
                let WC = JF / VZ;
                let WD = rspice_limexp(WC);
                let APM = (((Lanes([0.0, AJY[0], AJY[1]]) - Lanes([(APL * WA), 0.0, 0.0])) / VZ) * WB) * WE;
                let WF = AL - WE;
                let APN = (((AOV - Lanes([(APL * WC), 0.0, 0.0])) / VZ) * WD) * WF;
                let WG = ((WE * WB) + (WF * WD)) - AL;
                let WH = BK * WG;
                let APO = Lanes([(AIC * WG), 0.0, 0.0, 0.0, 0.0]) + ((Lanes([APM[0], 0.0, APM[1], 0.0, APM[2]]) + Lanes([APN[0], APN[1], 0.0, APN[2], 0.0])) * BK);
                let WI = (AL + (FC * (WH * II))).sqrt();
                let WJ = EK * (AL + WI);
                let APP = (((APO * II) * FC) * (AFG / (AJB * WI))) * EK;
                let APQ = Lanes([0.0, AFO]) - Lanes([AFN, 0.0]);
                let WK = (TG - JI) / VZ;
                let WL = rspice_limexp(WK);
                let WM = WL - AL;
                let APR = Lanes([(AIC * WM), 0.0, 0.0]) + ((((Lanes([0.0, APQ[0], APQ[1]]) - Lanes([(APL * WK), 0.0, 0.0])) / VZ) * WL) * BK);
                let WN = (WH - (BK * WM)) / WJ;
                let APS = APP * WN;
                let APT = ((Lanes([APO[0], APO[1], APO[2], APO[3], APO[4], 0.0]) - Lanes([APR[0], 0.0, 0.0, 0.0, APR[1], APR[2]])) - Lanes([APS[0], APS[1], APS[2], APS[3], APS[4], 0.0])) / WJ;
                ABN = WJ;
                ADD = WH;
                AEC = WN;
                AGQ = APP;
                AGR = APO;
                AGS = APT;
            } else {
                ABN = AL;
                ADD = A;
                AEC = A;
                AGQ = APJ;
                AGR = APJ;
                AGS = APK;
            }
            let WP = if WO == AL { 1.0 } else { 0.0 };
            let ADO;
            let ADU;
            let AGT;
            let AGU;
            if WP != 0.0 {
                let WQ = BT * G;
                let WR = JB / WQ;
                let WS = rspice_limexp(WR);
                let AQK = ((AOT - Lanes([((AHM * BT) * WR), 0.0, 0.0])) / WQ) * WS;
                let WT = CD * G;
                let WU = JB / WT;
                let WV = rspice_limexp(WU);
                let AQL = ((AOT - Lanes([((AHM * CD) * WU), 0.0, 0.0])) / WT) * WV;
                let WW = if EB > A { 1.0 } else { 0.0 };
                let ADP;
                let AGV;
                if WW != 0.0 {
                    let WX = (HM - JB) / HN;
                    let WY = rspice_limexp(WX);
                    let WZ = WS - AL;
                    let XA = WV - AL;
                    let XC = ((BV * WZ) + (CF * XA)) - (XB * (WY - HP));
                    let AQN = ((Lanes([(AIE * WZ), 0.0, 0.0]) + (AQK * BV)) + (Lanes([(AIG * XA), 0.0, 0.0]) + (AQL * CF))) - ((((((Lanes([AJO, 0.0, 0.0]) - AOT) - Lanes([(AJP * WX), 0.0, 0.0])) / HN) * WY) - Lanes([AJQ, 0.0, 0.0])) * XB);
                    ADP = XC;
                    AGV = AQN;
                } else {
                    let XD = WS - AL;
                    let XE = WV - AL;
                    let XF = (BV * XD) + (CF * XE);
                    let AQM = (Lanes([(AIE * XD), 0.0, 0.0]) + (AQK * BV)) + (Lanes([(AIG * XE), 0.0, 0.0]) + (AQL * CF));
                    ADP = XF;
                    AGV = AQM;
                }
                ADO = ADP;
                ADU = A;
                AGT = AGV;
                AGU = ALD;
            } else {
                let XG = if WO == A { 1.0 } else { 0.0 };
                let ADQ;
                let ADV;
                let AGW;
                let AGX;
                if XG != 0.0 {
                    let XH = BT * G;
                    let XI = JD / XH;
                    let AQF = Lanes([0.0, AJV[0], AJV[1]]);
                    let XJ = rspice_limexp(XI);
                    let AQG = ((AQF - Lanes([((AHM * BT) * XI), 0.0, 0.0])) / XH) * XJ;
                    let XK = CD * G;
                    let XL = JD / XK;
                    let XM = rspice_limexp(XL);
                    let AQH = ((AQF - Lanes([((AHM * CD) * XL), 0.0, 0.0])) / XK) * XM;
                    let XN = if EB > A { 1.0 } else { 0.0 };
                    let ADW;
                    let AGY;
                    if XN != 0.0 {
                        let XO = (HM - JD) / HN;
                        let XP = rspice_limexp(XO);
                        let XQ = XJ - AL;
                        let XR = XM - AL;
                        let XS = ((BV * XQ) + (CF * XR)) - (XB * (XP - HP));
                        let AQJ = ((Lanes([(AIE * XQ), 0.0, 0.0]) + (AQG * BV)) + (Lanes([(AIG * XR), 0.0, 0.0]) + (AQH * CF))) - ((((((Lanes([AJO, 0.0, 0.0]) - AQF) - Lanes([(AJP * XO), 0.0, 0.0])) / HN) * XP) - Lanes([AJQ, 0.0, 0.0])) * XB);
                        ADW = XS;
                        AGY = AQJ;
                    } else {
                        let XT = XJ - AL;
                        let XU = XM - AL;
                        let XV = (BV * XT) + (CF * XU);
                        let AQI = (Lanes([(AIE * XT), 0.0, 0.0]) + (AQG * BV)) + (Lanes([(AIG * XU), 0.0, 0.0]) + (AQH * CF));
                        ADW = XV;
                        AGY = AQI;
                    }
                    ADQ = A;
                    ADV = ADW;
                    AGW = AKN;
                    AGX = AGY;
                } else {
                    let XW = BT * G;
                    let APU = AHM * BT;
                    let XX = JB / XW;
                    let XY = rspice_limexp(XX);
                    let APV = ((AOT - Lanes([(APU * XX), 0.0, 0.0])) / XW) * XY;
                    let XZ = CD * G;
                    let APW = AHM * CD;
                    let YA = JB / XZ;
                    let YB = rspice_limexp(YA);
                    let APX = ((AOT - Lanes([(APW * YA), 0.0, 0.0])) / XZ) * YB;
                    let YC = if EB > A { 1.0 } else { 0.0 };
                    let ADR;
                    let AGZ;
                    if YC != 0.0 {
                        let YD = (HM - JB) / HN;
                        let YE = rspice_limexp(YD);
                        let YF = XY - AL;
                        let YG = YB - AL;
                        let YH = WO * (((BV * YF) + (CF * YG)) - (XB * (YE - HP)));
                        let APZ = (((Lanes([(AIE * YF), 0.0, 0.0]) + (APV * BV)) + (Lanes([(AIG * YG), 0.0, 0.0]) + (APX * CF))) - ((((((Lanes([AJO, 0.0, 0.0]) - AOT) - Lanes([(AJP * YD), 0.0, 0.0])) / HN) * YE) - Lanes([AJQ, 0.0, 0.0])) * XB)) * WO;
                        ADR = YH;
                        AGZ = APZ;
                    } else {
                        let YI = XY - AL;
                        let YJ = YB - AL;
                        let YK = WO * ((BV * YI) + (CF * YJ));
                        let APY = ((Lanes([(AIE * YI), 0.0, 0.0]) + (APV * BV)) + (Lanes([(AIG * YJ), 0.0, 0.0]) + (APX * CF))) * WO;
                        ADR = YK;
                        AGZ = APY;
                    }
                    let YL = JD / XW;
                    let AQA = Lanes([0.0, AJV[0], AJV[1]]);
                    let YM = rspice_limexp(YL);
                    let AQB = ((AQA - Lanes([(APU * YL), 0.0, 0.0])) / XW) * YM;
                    let YN = JD / XZ;
                    let YO = rspice_limexp(YN);
                    let AQC = ((AQA - Lanes([(APW * YN), 0.0, 0.0])) / XZ) * YO;
                    let ADX;
                    let AHA;
                    if YC != 0.0 {
                        let YP = (HM - JD) / HN;
                        let YQ = rspice_limexp(YP);
                        let YR = AL - WO;
                        let YS = YM - AL;
                        let YT = YO - AL;
                        let YU = YR * (((BV * YS) + (CF * YT)) - (XB * (YQ - HP)));
                        let AQE = (((Lanes([(AIE * YS), 0.0, 0.0]) + (AQB * BV)) + (Lanes([(AIG * YT), 0.0, 0.0]) + (AQC * CF))) - ((((((Lanes([AJO, 0.0, 0.0]) - AQA) - Lanes([(AJP * YP), 0.0, 0.0])) / HN) * YQ) - Lanes([AJQ, 0.0, 0.0])) * XB)) * YR;
                        ADX = YU;
                        AHA = AQE;
                    } else {
                        let YV = AL - WO;
                        let YW = YM - AL;
                        let YX = YO - AL;
                        let YY = YV * ((BV * YW) + (CF * YX));
                        let AQD = ((Lanes([(AIE * YW), 0.0, 0.0]) + (AQB * BV)) + (Lanes([(AIG * YX), 0.0, 0.0]) + (AQC * CF))) * YV;
                        ADX = YY;
                        AHA = AQD;
                    }
                    ADQ = ADR;
                    ADV = ADX;
                    AGW = AGZ;
                    AGX = AHA;
                }
                ADO = ADQ;
                ADU = ADV;
                AGT = AGW;
                AGU = AGX;
            }
            let YZ = CM * G;
            let AQO = AHM * CM;
            let ZA = JF / YZ;
            let ZB = rspice_limexp(ZA);
            let ZC = CV * G;
            let AQP = AHM * CV;
            let ZD = JF / ZC;
            let ZE = rspice_limexp(ZD);
            let ZF = ZB - AL;
            let ZG = ZE - AL;
            let ZH = (CP * ZF) + (CY * ZG);
            let AQQ = (Lanes([(AIK * ZF), 0.0, 0.0]) + ((((AOV - Lanes([(AQO * ZA), 0.0, 0.0])) / YZ) * ZB) * CP)) + (Lanes([(AIO * ZG), 0.0, 0.0]) + ((((AOV - Lanes([(AQP * ZD), 0.0, 0.0])) / ZC) * ZE) * CY));
            let ZI = if (if CZ > A { 1.0 } else { 0.0 }) != 0.0 || (if DB > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let ADY;
            let AHB;
            if ZI != 0.0 {
                let ZJ = JJ / YZ;
                let AQR = Lanes([0.0, AJY[0], AJY[1]]);
                let ZK = rspice_limexp(ZJ);
                let ZL = JJ / ZC;
                let ZM = rspice_limexp(ZL);
                let ZN = ZK - AL;
                let ZO = ZM - AL;
                let ZP = (DA * ZN) + (DC * ZO);
                let AQS = (Lanes([(AIP * ZN), 0.0, 0.0]) + ((((AQR - Lanes([(AQO * ZJ), 0.0, 0.0])) / YZ) * ZK) * DA)) + (Lanes([(AIQ * ZO), 0.0, 0.0]) + ((((AQR - Lanes([(AQP * ZL), 0.0, 0.0])) / ZC) * ZM) * DC));
                ADY = ZP;
                AHB = AQS;
            } else {
                ADY = A;
                AHB = ANV;
            }
            let ZR = if ZQ > A { 1.0 } else { 0.0 };
            let AAE;
            let AHC;
            if ZR != 0.0 {
                let ZS = FZ - JF;
                let AQU = Lanes([AJE, 0.0, 0.0]) - AOV;
                let AQV = AQU * ZS;
                let ZU = ((ZS * ZS) + ZT).sqrt();
                let ZV = EK * (ZU + ZS);
                let AQW = (((AQV + AQV) * (AFG / (AJB * ZU))) + AQU) * EK;
                let ZW = ZQ * ZV;
                let ZX = -EA;
                let ZY = GX - AL;
                let ZZ = ZV.powf(ZY);
                let AAA = rspice_limexp((ZX * ZZ));
                let AAB = ZW * AAA;
                let AAC = (VX - VW) - ZH;
                let AAD = AAC * AAB;
                let AQX = (((AQW * ZQ) * AAA) + (((Lanes([((AIW * AHX) * ZZ), 0.0, 0.0]) + ((AQW * (ZY * (ZV.powf((ZY - AFG))))) * ZX)) * AAA) * ZW)) * AAC;
                let AQY = (((API - APH) - Lanes([AQQ[0], AQQ[1], AQQ[2], 0.0])) * AAB) + Lanes([AQX[0], AQX[1], AQX[2], 0.0]);
                AAE = AAD;
                AHC = AQY;
            } else {
                AAE = A;
                AHC = AQT;
            }
            let AAF = ZH - AAE;
            let AQZ = Lanes([AQQ[0], AQQ[1], AQQ[2], 0.0]) - AHC;
            let AAG = if M > A { 1.0 } else { 0.0 };
            let AEE;
            let AHD;
            if AAG != 0.0 {
                let ARB = Lanes([AFP, 0.0]) - Lanes([0.0, AFM]);
                let AAI = (AAH - JG) / O;
                let ARC = (Lanes([ARB[0], 0.0, ARB[1]]) - Lanes([0.0, (AHP * AAI), 0.0])) / O;
                AEE = AAI;
                AHD = ARC;
            } else {
                AEE = A;
                AHD = ARA;
            }
            let AAJ = JF / G;
            let AAK = rspice_limexp(AAJ);
            let AAL = JH / G;
            let AAM = rspice_limexp(AAL);
            let AAN = (AL + (HI * AAK)).sqrt();
            let ARD = (Lanes([(AJM * AAK), 0.0, 0.0]) + ((((AOV - Lanes([(AHM * AAJ), 0.0, 0.0])) / G) * AAK) * HI)) * (AFG / (AJB * AAN));
            let AAO = (AL + (HI * AAM)).sqrt();
            let ARE = (Lanes([(AJM * AAM), 0.0, 0.0]) + ((((Lanes([0.0, AJX[0], AJX[1]]) - Lanes([(AHM * AAL), 0.0, 0.0])) / G) * AAM) * HI)) * (AFG / (AJB * AAO));
            let AAP = if P > A { 1.0 } else { 0.0 };
            let AEG;
            let AHE;
            if AAP != 0.0 {
                let AAQ = AAO + AL;
                let AAR = (AAN + AL) / AAQ;
                let ARG = ARE * AAR;
                let ARH = Lanes([ARD[0], 0.0, ARD[1], ARD[2]]);
                let AAS = JG - JE;
                let ARI = Lanes([AFM, 0.0]) - Lanes([0.0, AFL]);
                let AAT = (AAN - AAO) - (AAR.ln());
                let AAU = (AAS + (G * AAT)) / R;
                let ARJ = ((Lanes([0.0, ARI[0], ARI[1], 0.0]) + (Lanes([(AHM * AAT), 0.0, 0.0, 0.0]) + (((ARH - Lanes([ARE[0], ARE[1], 0.0, ARE[2]])) - (((ARH - Lanes([ARG[0], ARG[1], 0.0, ARG[2]])) / AAQ) * (AFG / AAR))) * G))) - Lanes([(AHQ * AAU), 0.0, 0.0, 0.0])) / R;
                let AAV = IL * R;
                let AAW = (EK * IL) * IP;
                let ARK = ARI * AAS;
                let AAX = ((AAS * AAS) + ZT).sqrt();
                let ARL = ((ARK + ARK) * (AFG / (AJB * AAX))) * AAW;
                let AAY = AL + (AAW * AAX);
                let AAZ = (AAV * AAU) / AAY;
                let ARM = (Lanes([(((AFU * EK) * IP) * AAX), 0.0, 0.0]) + Lanes([0.0, ARL[0], ARL[1]])) * AAZ;
                let ARN = (((Lanes([(((AFU * R) + (AHQ * IL)) * AAU), 0.0, 0.0, 0.0]) + (ARJ * AAV)) - Lanes([ARM[0], ARM[1], ARM[2], 0.0])) / AAY) * AAZ;
                let ABA = (AL + (AAZ * AAZ)).sqrt();
                let ABB = AAU / ABA;
                let ARO = (ARJ - (((ARN + ARN) * (AFG / (AJB * ABA))) * ABB)) / ABA;
                AEG = ABB;
                AHE = ARO;
            } else {
                AEG = A;
                AHE = ARF;
            }
            let ABC = if S > A { 1.0 } else { 0.0 };
            let AEI;
            let AHF;
            if ABC != 0.0 {
                let ARQ = Lanes([AFQ, 0.0]) - Lanes([0.0, AFK]);
                let ABE = (ABD - JC) / U;
                let ARR = (Lanes([ARQ[0], 0.0, ARQ[1]]) - Lanes([0.0, (AHR * ABE), 0.0])) / U;
                AEI = ABE;
                AHF = ARR;
            } else {
                AEI = A;
                AHF = ARP;
            }
            let ABF = if V > A { 1.0 } else { 0.0 };
            let AEK;
            let AHG;
            if ABF != 0.0 {
                let ABG = JC - IZ;
                let ART = (Lanes([AFK, 0.0]) - Lanes([0.0, AFI])) * VV;
                let ARU = AGP * ABG;
                let ABH = (ABG * VV) / X;
                let ARV = ((Lanes([0.0, 0.0, ART[0], ART[1], 0.0]) + Lanes([ARU[0], ARU[1], 0.0, ARU[2], ARU[3]])) - Lanes([(AHS * ABH), 0.0, 0.0, 0.0, 0.0])) / X;
                AEK = ABH;
                AHG = ARV;
            } else {
                AEK = A;
                AHG = ARS;
            }
            let ABI = if Y > A { 1.0 } else { 0.0 };
            let AEM;
            let AHH;
            if ABI != 0.0 {
                let ARX = Lanes([AFR, 0.0]) - Lanes([0.0, AFJ]);
                let ABK = (ABJ - JA) / AA;
                let ARY = (Lanes([ARX[0], 0.0, ARX[1]]) - Lanes([0.0, (AHT * ABK), 0.0])) / AA;
                AEM = ABK;
                AHH = ARY;
            } else {
                AEM = A;
                AHH = ARW;
            }
            let ABL = if AE > A { 1.0 } else { 0.0 };
            let AEO;
            let AHI;
            if ABL != 0.0 {
                let ABM = JI - JG;
                let ASA = (Lanes([0.0, AFN]) - Lanes([AFM, 0.0])) * ABN;
                let ASB = AGQ * ABM;
                let ABO = (ABM * ABN) / AG;
                let ASC = ((Lanes([0.0, ASA[0], 0.0, 0.0, 0.0, ASA[1]]) + Lanes([ASB[0], 0.0, ASB[1], ASB[2], ASB[3], ASB[4]])) - Lanes([(AHV * ABO), 0.0, 0.0, 0.0, 0.0, 0.0])) / AG;
                AEO = ABO;
                AHI = ASC;
            } else {
                AEO = A;
                AHI = ARZ;
            }
            let ABP = if (if DD > A { 1.0 } else { 0.0 }) != 0.0 || (if DM > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let AEB;
            let AHJ;
            if ABP != 0.0 {
                let ABQ = TG - JI;
                let ASD = Lanes([0.0, AFO]) - Lanes([AFN, 0.0]);
                let ABR = DJ * G;
                let ABS = ABQ / ABR;
                let ASE = Lanes([0.0, ASD[0], ASD[1]]);
                let ABT = rspice_limexp(ABS);
                let ABU = DR * G;
                let ABV = ABQ / ABU;
                let ABW = rspice_limexp(ABV);
                let ABX = ABT - AL;
                let ABY = ABW - AL;
                let ABZ = (DL * ABX) + (DT * ABY);
                let ASF = (Lanes([(AIR * ABX), 0.0, 0.0]) + ((((ASE - Lanes([((AHM * DJ) * ABS), 0.0, 0.0])) / ABR) * ABT) * DL)) + (Lanes([(AIS * ABY), 0.0, 0.0]) + ((((ASE - Lanes([((AHM * DR) * ABV), 0.0, 0.0])) / ABU) * ABW) * DT));
                AEB = ABZ;
                AHJ = ASF;
            } else {
                AEB = A;
                AHJ = ANZ;
            }
            let ACA = if AB > A { 1.0 } else { 0.0 };
            let ADZ;
            let AHK;
            if ACA != 0.0 {
                let ASH = Lanes([AFS, 0.0]) - Lanes([0.0, AFO]);
                let ACC = (ACB - TG) / AD;
                let ASI = (Lanes([ASH[0], 0.0, ASH[1]]) - Lanes([0.0, (AHU * ACC), 0.0])) / AD;
                ADZ = ACC;
                AHK = ASI;
            } else {
                ADZ = A;
                AHK = ASG;
            }
            let ACD = if UW > A { 1.0 } else { 0.0 };
            let ACE = if ACD != 0.0 {
                AL
            } else {
                A
            };
            let ACF = (UW * ACE) * IX;
            let ASJ = (AOU * ACE) * IX;
            let ACG = ACF + AL;
            let ACH = ACF / ACG;
            let ACK = ACI * (AL + (ACJ * VK));
            let ACN = rspice_limexp(((JF * IT) / ACM));
            let ACO = ACL * ACN;
            let ASK = ((ASJ - (ASJ * ACH)) / ACG) * ACH;
            let ACP = IY + (ACH * ACH);
            let ASL = ((((AJW * IT) / ACM) * ACN) * ACL) * ACP;
            let ASM = (ASK + ASK) * ACO;
            let ACQ = AL + ((ACO * ACP) * ACE);
            let ACR = ACK * ACQ;
            let ASN = (Lanes([(AJH * VD), 0.0, 0.0]) + (AFV * GU)) * WO;
            let ASO = AOU * ACR;
            let ACS = (ACR * UW) / VV;
            let ACT = ((GU * VD) * WO) + ACS;
            let ASP = Lanes([ASN[0], 0.0, ASN[1], ASN[2]]) + ((((((((APB * ACJ) * ACI) * ACQ) + (((Lanes([0.0, ASL[0], ASL[1], 0.0]) + Lanes([ASM[0], 0.0, ASM[1], ASM[2]])) * ACE) * ACK)) * UW) + Lanes([ASO[0], 0.0, ASO[1], ASO[2]])) - (AGP * ACS)) / VV);
            let ACV = AL - WO;
            let ACW = (GU * ACU) * ACV;
            let ASQ = (Lanes([(AJH * ACU), 0.0, 0.0]) + (AFY * GU)) * ACV;
            let ACZ = ((GZ * VE) + (ACX * VC)) + (ACY * AAN);
            let ASR = ((Lanes([(AJJ * VE), 0.0, 0.0]) + (AGB * GZ)) + (AOW * ACX)) + (ARD * ACY);
            let ADA = ACY * AAO;
            let ASS = ARE * ACY;
            let AST = Lanes([(AJK * ADB), 0.0, 0.0]) + (AGG * HB);
            let ADE = (HB * ADB) + (ACX * ADD);
            let ASU = Lanes([AST[0], 0.0, AST[1], 0.0, AST[2]]) + (AGR * ACX);
            let ADI = TG - JI;
            let ASV = Lanes([0.0, AFO]) - Lanes([AFN, 0.0]);
            let ASW = ASV * ADH;
            let ADJ = (HF * ADF) + (ADH * ADI);
            let ASX = (Lanes([(AJL * ADF), 0.0, 0.0]) + (AGL * HF)) + Lanes([0.0, ASW[0], ASW[1]]);
            let ADL = (ABD - ABJ) * ADK;
            let ASY = (Lanes([AFQ, 0.0]) - Lanes([0.0, AFR])) * ADK;
            let ADN = (ABD - AAH) * ADM;
            let ASZ = (Lanes([0.0, AFQ]) - Lanes([AFP, 0.0])) * ADM;
            let ATA = AJU * ADO;
            let ATB = (AGT * JB) + Lanes([0.0, ATA[0], ATA[1]]);
            let ATC = AJW * AAF;
            let ADS = VX - VW;
            let ADT = JE - JA;
            let ATD = (Lanes([AFL, 0.0]) - Lanes([0.0, AFJ])) * ADS;
            let ATE = (Lanes([ATB[0], 0.0, ATB[1], ATB[2]]) + ((AQZ * JF) + Lanes([0.0, ATC[0], ATC[1], 0.0]))) + (((API - APH) * ADT) + Lanes([0.0, ATD[0], 0.0, ATD[1]]));
            let ATF = AJV * ADU;
            let ATG = (AGU * JD) + Lanes([0.0, ATF[0], ATF[1]]);
            let ATH = Lanes([ATE[0], ATE[1], 0.0, ATE[2], ATE[3]]) + Lanes([ATG[0], 0.0, ATG[1], 0.0, ATG[2]]);
            let ATI = AJY * ADY;
            let ATJ = (AHB * JJ) + Lanes([0.0, ATI[0], ATI[1]]);
            let ATK = Lanes([ATH[0], ATH[1], ATH[2], ATH[3], ATH[4], 0.0]) + Lanes([ATJ[0], 0.0, ATJ[1], 0.0, 0.0, ATJ[2]]);
            let AEA = ACB - TG;
            let ATL = (Lanes([AFS, 0.0]) - Lanes([0.0, AFO])) * ADZ;
            let ATM = (AHK * AEA) + Lanes([ATL[0], 0.0, ATL[1]]);
            let ATN = ASV * AEB;
            let ATO = (AHJ * ADI) + Lanes([0.0, ATN[0], ATN[1]]);
            let AED = JC - TG;
            let ATP = (Lanes([AFK, 0.0]) - Lanes([0.0, AFO])) * AEC;
            let ATQ = (AGS * AED) + Lanes([0.0, 0.0, ATP[0], 0.0, 0.0, ATP[1]]);
            let ATR = ((Lanes([0.0, ATK[0], ATK[1], ATK[2], ATK[3], ATK[4], ATK[5], 0.0]) + Lanes([ATM[0], ATM[1], 0.0, 0.0, 0.0, 0.0, 0.0, ATM[2]])) + Lanes([0.0, ATO[0], 0.0, 0.0, 0.0, 0.0, ATO[1], ATO[2]])) + Lanes([0.0, ATQ[0], ATQ[1], ATQ[2], ATQ[3], 0.0, ATQ[4], ATQ[5]]);
            let AEF = AAH - JG;
            let ATS = (Lanes([AFP, 0.0]) - Lanes([0.0, AFM])) * AEE;
            let ATT = (AHD * AEF) + Lanes([ATS[0], 0.0, ATS[1]]);
            let AEH = JG - JE;
            let ATU = (Lanes([AFM, 0.0]) - Lanes([0.0, AFL])) * AEG;
            let ATV = (AHE * AEH) + Lanes([0.0, ATU[0], ATU[1], 0.0]);
            let ATW = (Lanes([0.0, ATR[0], ATR[1], 0.0, ATR[2], ATR[3], ATR[4], ATR[5], ATR[6], ATR[7]]) + Lanes([ATT[0], 0.0, ATT[1], ATT[2], 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])) + Lanes([0.0, 0.0, ATV[0], ATV[1], ATV[2], 0.0, ATV[3], 0.0, 0.0, 0.0]);
            let AEJ = ABD - JC;
            let ATX = (Lanes([AFQ, 0.0]) - Lanes([0.0, AFK])) * AEI;
            let ATY = (AHF * AEJ) + Lanes([ATX[0], 0.0, ATX[1]]);
            let AEL = JC - IZ;
            let ATZ = (Lanes([AFK, 0.0]) - Lanes([0.0, AFI])) * AEK;
            let AUA = (AHG * AEL) + Lanes([0.0, 0.0, ATZ[0], ATZ[1], 0.0]);
            let AUB = (Lanes([ATW[0], 0.0, ATW[1], ATW[2], ATW[3], ATW[4], ATW[5], ATW[6], ATW[7], ATW[8], ATW[9]]) + Lanes([0.0, ATY[0], 0.0, ATY[1], 0.0, 0.0, ATY[2], 0.0, 0.0, 0.0, 0.0])) + Lanes([0.0, 0.0, 0.0, AUA[0], 0.0, AUA[1], AUA[2], AUA[3], AUA[4], 0.0, 0.0]);
            let AEN = ABJ - JA;
            let AUC = (Lanes([AFR, 0.0]) - Lanes([0.0, AFJ])) * AEM;
            let AUD = (AHH * AEN) + Lanes([AUC[0], 0.0, AUC[1]]);
            let AEP = JI - JG;
            let AUE = (Lanes([0.0, AFN]) - Lanes([AFM, 0.0])) * AEO;
            let AUF = (AHI * AEP) + Lanes([0.0, AUE[0], 0.0, 0.0, 0.0, AUE[1]]);
            let AEQ = -((((((((((((((ADO * JB) + (AAF * JF)) + (ADS * ADT)) + (ADU * JD)) + (ADY * JJ)) + (ADZ * AEA)) + (AEB * ADI)) + (AEC * AED)) + (AEE * AEF)) + (AEG * AEH)) + (AEI * AEJ)) + (AEK * AEL)) + (AEM * AEN)) + (AEO * AEP));
            let AUG = ((Lanes([AUB[0], AUB[1], 0.0, AUB[2], AUB[3], AUB[4], AUB[5], AUB[6], AUB[7], AUB[8], AUB[9], AUB[10]]) + Lanes([0.0, 0.0, AUD[0], 0.0, AUD[1], 0.0, 0.0, 0.0, 0.0, AUD[2], 0.0, 0.0])) + Lanes([0.0, 0.0, 0.0, 0.0, AUF[0], AUF[1], AUF[2], AUF[3], AUF[4], 0.0, AUF[5], 0.0])) * AHX;
            let AES = if AER > A { 1.0 } else { 0.0 };
            let AFE;
            let AHL;
            if AES != 0.0 {
                let AET = C / AER;
                let AUH = AFH / AER;
                AFE = AET;
                AHL = AUH;
            } else {
                AFE = A;
                AHL = AJR;
            }
            let AEV = C * AEU;
            let AUI = AFH * AEU;
            let AEW = ddt(6568, ACT);
            let AUK = ASP * AUJ;
            let AEX = ddt(6570, ACW);
            let AUL = ASQ * AUJ;
            let AEY = ddt(6572, ACZ);
            let AUM = ASR * AUJ;
            let AEZ = ddt(6574, ADA);
            let AUN = ASS * AUJ;
            let AFA = ddt(6576, ADE);
            let AUO = ASU * AUJ;
            let AFB = ddt(6578, ADL);
            let AUP = ASY * AUJ;
            let AFC = ddt(6580, ADN);
            let AUQ = ASZ * AUJ;
            let AFD = ddt(6585, ADJ);
            let AUR = ASX * AUJ;
            let AFF = ddt(6589, AEV);
            let AUS = AGT[0];
            let AUT = AGT[1];
            let AUU = AGT[2];
            let AUV = AGU[0];
            let AUW = AGU[1];
            let AUX = AGU[2];
            let AUY = API[0];
            let AUZ = API[1];
            let AVA = API[2];
            let AVB = API[3];
            let AVC = APH[0];
            let AVD = APH[1];
            let AVE = APH[2];
            let AVF = APH[3];
            let AVG = AQZ[0];
            let AVH = AQZ[1];
            let AVI = AQZ[2];
            let AVJ = AQZ[3];
            let AVK = AHB[0];
            let AVL = AHB[1];
            let AVM = AHB[2];
            let AVN = AHD[0];
            let AVO = AHD[1];
            let AVP = AHD[2];
            let AVQ = AHE[0];
            let AVR = AHE[1];
            let AVS = AHE[2];
            let AVT = AHE[3];
            let AVU = AHF[0];
            let AVV = AHF[1];
            let AVW = AHF[2];
            let AVX = AHG[0];
            let AVY = AHG[1];
            let AVZ = AHG[2];
            let AWA = AHG[3];
            let AWB = AHG[4];
            let AWC = AHH[0];
            let AWD = AHH[1];
            let AWE = AHH[2];
            let AWF = AHI[0];
            let AWG = AHI[1];
            let AWH = AHI[2];
            let AWI = AHI[3];
            let AWJ = AHI[4];
            let AWK = AHI[5];
            let AWL = AUK[0];
            let AWM = AUK[1];
            let AWN = AUK[2];
            let AWO = AUK[3];
            let AWP = AUL[0];
            let AWQ = AUL[1];
            let AWR = AUL[2];
            let AWS = AUM[0];
            let AWT = AUM[1];
            let AWU = AUM[2];
            let AWV = AUN[0];
            let AWW = AUN[1];
            let AWX = AUN[2];
            let AWY = AUO[0];
            let AWZ = AUO[1];
            let AXA = AUO[2];
            let AXB = AUO[3];
            let AXC = AUO[4];
            let AXD = AUP[0];
            let AXE = AUP[1];
            let AXF = AUQ[0];
            let AXG = AUQ[1];
            let AXH = AHJ[0];
            let AXI = AHJ[1];
            let AXJ = AHJ[2];
            let AXK = AGS[0];
            let AXL = AGS[1];
            let AXM = AGS[2];
            let AXN = AGS[3];
            let AXO = AGS[4];
            let AXP = AGS[5];
            let AXQ = AHK[0];
            let AXR = AHK[1];
            let AXS = AHK[2];
            let AXT = AUR[0];
            let AXU = AUR[1];
            let AXV = AUR[2];
            let AXW = AHL;
            let AXX = AUG[0];
            let AXY = AUG[1];
            let AXZ = AUG[2];
            let AYA = AUG[3];
            let AYB = AUG[4];
            let AYC = AUG[5];
            let AYD = AUG[6];
            let AYE = AUG[7];
            let AYF = AUG[8];
            let AYG = AUG[9];
            let AYH = AUG[10];
            let AYI = AUG[11];
            let AYJ = (AUI * AUJ);
            let AYK = ASP[0];
            let AYL = ASP[1];
            let AYM = ASP[2];
            let AYN = ASP[3];
            let AYO = ASQ[0];
            let AYP = ASQ[1];
            let AYQ = ASQ[2];
            let AYR = ASR[0];
            let AYS = ASR[1];
            let AYT = ASR[2];
            let AYU = ASS[0];
            let AYV = ASS[1];
            let AYW = ASS[2];
            let AYX = ASU[0];
            let AYY = ASU[1];
            let AYZ = ASU[2];
            let AZA = ASU[3];
            let AZB = ASU[4];
            let AZC = ASY[0];
            let AZD = ASY[1];
            let AZE = ASZ[0];
            let AZF = ASZ[1];
            let AZG = ASX[0];
            let AZH = ASX[1];
            let AZI = ASX[2];
            let AZJ = AUI;
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(8),
            Some(9),
            multiplicity * (ADO),
            [4, 8, 9],
            [AUS, AUT, AUU],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(7),
            Some(9),
            multiplicity * (ADU),
            [4, 7, 9],
            [AUV, AUW, AUX],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(6),
            Some(9),
            multiplicity * (VX),
            [4, 6, 8, 9],
            [AUY, AUZ, AVA, AVB],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(9),
            Some(6),
            multiplicity * (VW),
            [4, 6, 8, 9],
            [AVC, AVD, AVE, AVF],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(8),
            Some(6),
            multiplicity * (AAF),
            [4, 6, 8, 9],
            [AVG, AVH, AVI, AVJ],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(7),
            Some(10),
            multiplicity * (ADY),
            [4, 7, 10],
            [AVK, AVL, AVM],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(0),
            Some(5),
            multiplicity * (AEE),
            [0, 4, 5],
            [AVN, AVO, AVP],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(5),
            Some(6),
            multiplicity * (AEG),
            [4, 5, 6, 8],
            [AVQ, AVR, AVS, AVT],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(1),
            Some(7),
            multiplicity * (AEI),
            [1, 4, 7],
            [AVU, AVV, AVW],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(7),
            Some(8),
            multiplicity * (AEK),
            [4, 6, 7, 8, 9],
            [AVX, AVY, AVZ, AWA, AWB],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(2),
            Some(9),
            multiplicity * (AEM),
            [2, 4, 9],
            [AWC, AWD, AWE],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(10),
            Some(5),
            multiplicity * (AEO),
            [4, 5, 6, 7, 8, 10],
            [AWF, AWG, AWH, AWI, AWJ, AWK],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(8),
            Some(9),
            multiplicity * (AEW),
            [4, 6, 8, 9],
            [AWL, AWM, AWN, AWO],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(7),
            Some(9),
            multiplicity * (AEX),
            [4, 7, 9],
            [AWP, AWQ, AWR],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(8),
            Some(6),
            multiplicity * (AEY),
            [4, 6, 8],
            [AWS, AWT, AWU],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(8),
            Some(5),
            multiplicity * (AEZ),
            [4, 5, 8],
            [AWV, AWW, AWX],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(7),
            Some(10),
            multiplicity * (AFA),
            [4, 6, 7, 8, 10],
            [AWY, AWZ, AXA, AXB, AXC],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(1),
            Some(2),
            multiplicity * (AFB),
            [1, 2],
            [AXD, AXE],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(1),
            Some(0),
            multiplicity * (AFC),
            [0, 1],
            [AXF, AXG],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(11),
            Some(10),
            multiplicity * (AEB),
            [4, 10, 11],
            [AXH, AXI, AXJ],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(7),
            Some(11),
            multiplicity * (AEC),
            [4, 6, 7, 8, 10, 11],
            [AXK, AXL, AXM, AXN, AXO, AXP],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(3),
            Some(11),
            multiplicity * (ADZ),
            [3, 4, 11],
            [AXQ, AXR, AXS],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(11),
            Some(10),
            multiplicity * (AFD),
            [4, 10, 11],
            [AXT, AXU, AXV],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(4),
            None,
            multiplicity * (AFE),
            [4],
            [AXW],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<12, 0>(
            Some(4),
            None,
            multiplicity * (AEQ),
            [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
            [AXX, AXY, AXZ, AYA, AYB, AYC, AYD, AYE, AYF, AYG, AYH, AYI],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(4),
            None,
            multiplicity * (AFF),
            [4],
            [AYJ],
            [],
            [],
            multiplicity,
        );
        self.canonical_reactive[0] = ADO;
        self.canonical_reactive[1] = ADU;
        self.canonical_reactive[2] = VX;
        self.canonical_reactive[3] = VW;
        self.canonical_reactive[4] = AAF;
        self.canonical_reactive[5] = ADY;
        self.canonical_reactive[6] = AEE;
        self.canonical_reactive[7] = AEG;
        self.canonical_reactive[8] = AEI;
        self.canonical_reactive[9] = AEK;
        self.canonical_reactive[10] = AEM;
        self.canonical_reactive[11] = AEO;
        self.canonical_reactive[12] = ACT;
        self.canonical_reactive[13] = AYK;
        self.canonical_reactive[14] = AYL;
        self.canonical_reactive[15] = AYM;
        self.canonical_reactive[16] = AYN;
        self.canonical_reactive[17] = ACW;
        self.canonical_reactive[18] = AYO;
        self.canonical_reactive[19] = AYP;
        self.canonical_reactive[20] = AYQ;
        self.canonical_reactive[21] = ACZ;
        self.canonical_reactive[22] = AYR;
        self.canonical_reactive[23] = AYS;
        self.canonical_reactive[24] = AYT;
        self.canonical_reactive[25] = ADA;
        self.canonical_reactive[26] = AYU;
        self.canonical_reactive[27] = AYV;
        self.canonical_reactive[28] = AYW;
        self.canonical_reactive[29] = ADE;
        self.canonical_reactive[30] = AYX;
        self.canonical_reactive[31] = AYY;
        self.canonical_reactive[32] = AYZ;
        self.canonical_reactive[33] = AZA;
        self.canonical_reactive[34] = AZB;
        self.canonical_reactive[35] = ADL;
        self.canonical_reactive[36] = AZC;
        self.canonical_reactive[37] = AZD;
        self.canonical_reactive[38] = ADN;
        self.canonical_reactive[39] = AZE;
        self.canonical_reactive[40] = AZF;
        self.canonical_reactive[41] = AEB;
        self.canonical_reactive[42] = AEC;
        self.canonical_reactive[43] = ADZ;
        self.canonical_reactive[44] = ADJ;
        self.canonical_reactive[45] = AZG;
        self.canonical_reactive[46] = AZH;
        self.canonical_reactive[47] = AZI;
        self.canonical_reactive[48] = AFE;
        self.canonical_reactive[49] = AEQ;
        self.canonical_reactive[50] = AEV;
        self.canonical_reactive[51] = AZJ;
    }

    pub fn stamp_reactive(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedReactiveStamper<'_>) {
        let multiplicity = self.multiplicity;
        let cached = &*self.canonical_reactive;
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(8),
            Some(9),
            &[4, 6, 8, 9],
            &[cached[13], cached[14], cached[15], cached[16]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(7),
            Some(9),
            &[4, 7, 9],
            &[cached[18], cached[19], cached[20]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(8),
            Some(6),
            &[4, 6, 8],
            &[cached[22], cached[23], cached[24]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(8),
            Some(5),
            &[4, 5, 8],
            &[cached[26], cached[27], cached[28]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(7),
            Some(10),
            &[4, 6, 7, 8, 10],
            &[cached[30], cached[31], cached[32], cached[33], cached[34]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(1),
            Some(2),
            &[1, 2],
            &[cached[36], cached[37]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(1),
            Some(0),
            &[0, 1],
            &[cached[39], cached[40]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(11),
            Some(10),
            &[4, 10, 11],
            &[cached[45], cached[46], cached[47]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(4),
            None,
            &[4],
            &[cached[51]],
            &[],
            &[],
            multiplicity,
        );
    }

}
