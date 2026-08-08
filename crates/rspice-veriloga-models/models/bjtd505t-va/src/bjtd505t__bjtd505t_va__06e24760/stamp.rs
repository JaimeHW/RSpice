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
            let slot = match operator { 12393 => 0usize, 12461 => 1usize, 12467 => 2usize, 12477 => 3usize, 12483 => 4usize, 12491 => 5usize, 12499 => 6usize, 12519 => 7usize, 12538 => 8usize, 12791 => 9usize, _ => usize::MAX };
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
            let B = parameters[3];
            let C = 1e0f64;
            let E = 7.03e7f64;
            let F = 1.23e8f64;
            let G = 1.58e8f64;
            let H = 2.04e8f64;
            let I = parameters[32];
            let N = parameters[141];
            let P = 1e-12f64;
            let R = parameters[1];
            let U = 1e-3f64;
            let V = 2e0f64;
            let W = parameters[66];
            let Z = parameters[113];
            let AA = parameters[114];
            let AB = parameters[115];
            let AD = 5e-2f64;
            let AE = 1e-1f64;
            let AK = parameters[65];
            let AM = parameters[70];
            let AN = parameters[71];
            let AQ = parameters[116];
            let AR = parameters[117];
            let AS = parameters[118];
            let BB = node_potentials[3];
            let BG = parameters[124];
            let BN = 8.617086918058125e-5f64;
            let CT = 3e0f64;
            let CU = -3e0f64;
            let CX = parameters[104];
            let DJ = -3e0f64;
            let DL = parameters[63];
            let DM = parameters[109];
            let DZ = -3e0f64;
            let EB = parameters[79];
            let EN = -3e0f64;
            let FB = -3e0f64;
            let FO = -3e0f64;
            let FQ = parameters[26];
            let FR = parameters[108];
            let GL = parameters[64];
            let GN = parameters[74];
            let GT = parameters[69];
            let GW = parameters[53];
            let GX = parameters[96];
            let HB = parameters[55];
            let HC = parameters[97];
            let HD = parameters[95];
            let HH = parameters[54];
            let HI = parameters[100];
            let HM = parameters[56];
            let HN = parameters[101];
            let HQ = parameters[57];
            let HR = parameters[103];
            let HU = parameters[58];
            let HW = parameters[59];
            let HX = parameters[98];
            let IA = parameters[121];
            let IC = parameters[9];
            let IO = parameters[122];
            let IQ = parameters[10];
            let JC = parameters[42];
            let JD = parameters[123];
            let JF = 1e-6f64;
            let JI = 5e-1f64;
            let JO = parameters[8];
            let JP = 4e0f64;
            let JQ = parameters[120];
            let KA = parameters[11];
            let KE = parameters[29];
            let KF = parameters[102];
            let KJ = parameters[19];
            let KK = 6e0f64;
            let KL = parameters[20];
            let KT = parameters[30];
            let KU = parameters[31];
            let LB = parameters[15];
            let LE = parameters[16];
            let LL = parameters[17];
            let LM = parameters[18];
            let LS = parameters[24];
            let LW = parameters[27];
            let MA = parameters[25];
            let ME = parameters[28];
            let ML = parameters[21];
            let MM = parameters[22];
            let MS = parameters[136];
            let MT = parameters[137];
            let MZ = parameters[142];
            let NC = parameters[144];
            let NH = -5e-1f64;
            let NK = parameters[34];
            let NQ = parameters[33];
            let NZ = -5e-1f64;
            let OC = parameters[36];
            let OI = parameters[35];
            let OQ = parameters[13];
            let OT = parameters[12];
            let OW = parameters[85];
            let PD = parameters[86];
            let PH = parameters[87];
            let PL = parameters[88];
            let PP = parameters[89];
            let PW = 7.2e-4f64;
            let PX = 1.6e-6f64;
            let QB = parameters[91];
            let QD = parameters[133];
            let QE = parameters[135];
            let QP = node_potentials[6];
            let QQ = node_potentials[7];
            let QS = node_potentials[8];
            let QU = node_potentials[4];
            let QW = node_potentials[5];
            let RA = node_potentials[2];
            let RC = node_potentials[1];
            let RG = node_potentials[10];
            let RN = parameters[138];
            let TS = parameters[140];
            let UA = 1e2f64;
            let UL = 2e-1f64;
            let UW = parameters[61];
            let UX = parameters[60];
            let VD = parameters[62];
            let WO = parameters[139];
            let YN = parameters[73];
            let ZM = parameters[75];
            let ABB = 1.0000000000000002e-2f64;
            let ABN = parameters[14];
            let ABS = 1e-4f64;
            let ACC = parameters[143];
            let ACL = parameters[145];
            let ACU = parameters[146];
            let ADO = 4e1f64;
            let ADR = 2.3538526683702e17f64;
            let AEK = parameters[92];
            let AHH = 1e-30f64;
            let AHQ = 1.6666666666666666e-1f64;
            let AIH = 3.333333333333333e-1f64;
            let AIJ = 2.5e-1f64;
            let ALF = parameters[5];
            let ALW = 1.21e-2f64;
            let AML = 1e-6f64;
            let AMM = 1e-12f64;
            let AMN = -1e0f64;
            let AMO = -1e0f64;
            let AMT = -1e0f64;
            let AMX = -1e0f64;
            let AMZ = parameters[81];
            let ANB = parameters[80];
            let ANW = 1.0000000000000002e-2f64;
            let AOP = parameters[38];
            let AOR = parameters[43];
            let AOT = parameters[41];
            let APE = parameters[40];
            let APL = parameters[39];
            let APS = parameters[44];
            let APX = parameters[7];
            let AQL = parameters[46];
            let ARV = parameters[47];
            let ARZ = parameters[48];
            let ASE = parameters[50];
            let ASO = parameters[49];
            let AVM = parameters[67];
            let AWG = parameters[76];
            let AYD = parameters[84];
            let AZB = parameters[90];
            let BBN = parameters[94];
            let BBR = parameters[93];
            let BCC = -1e0f64;
            let BCI = parameters[134];
            let BDA = -1e0f64;
            let BEG = 0e0f64;
            let BEJ = 0e0f64;
            let BET = parameters[130];
            let BEY = parameters[131];
            let BFG = 0e0f64;
            let BFH = node_potentials[11];
            let BFN = 0e0f64;
            let BFO = 0e0f64;
            let BFP = 0e0f64;
            let BFQ = 0e0f64;
            let BFR = 0e0f64;
            let BFS = 0e0f64;
            let BFT = 0e0f64;
            let BFU = 0e0f64;
            let BFV = 0e0f64;
            let BFW = 0e0f64;
            let BFX = 0e0f64;
            let BFY = 0e0f64;
            let BFZ = 0e0f64;
            let BGA = 0e0f64;
            let BGB = 0e0f64;
            let BGC = 0e0f64;
            let BGD = 0e0f64;
            let BGE = 0e0f64;
            let BGF = 0e0f64;
            let BGG = 0e0f64;
            let BGH = 0e0f64;
            let BGI = 0e0f64;
            let BGJ = 0e0f64;
            let BGK = 0e0f64;
            let BHV = 1e0f64;
            let BHW = 1e0f64;
            let BHX = 1e0f64;
            let BHY = 1e0f64;
            let BHZ = 1e0f64;
            let BIA = 1e0f64;
            let BIB = 1e0f64;
            let BIC = 1e0f64;
            let BID = 1e0f64;
            let BIE = 1e0f64;
            let BIF = 1e0f64;
            let BIG = 1e0f64;
            let BIH = 1e0f64;
            let BNP = -1e0f64;
            let BPQ = 0e0f64;
            let BQJ = 2e0f64;
            let BUG = Lanes([0e0f64; 3]);
            let BUU = Lanes([0e0f64; 4]);
            let CAL = Lanes([0e0f64; 3]);
            let CBY = Lanes([0e0f64; 9]);
            let CCJ = Lanes([0e0f64; 3]);
            let CDH = Lanes([0e0f64; 5]);
            let CJF = Lanes([0e0f64; 6]);
            let CKA = Lanes([0e0f64; 4]);
            let CKJ = ddt_scale();
            let CLT = Lanes([0e0f64; 3]);
            let CLW = Lanes([0e0f64; 3]);
            let D = if B == C { 1.0 } else { 0.0 };
            let PV;
            let ARB;
            if D != 0.0 {
                PV = F;
                ARB = E;
            } else {
                PV = H;
                ARB = G;
            }
            let J = C - I;
            let K = parameters[4] + 2.7315e2f64;
            let L = temperature + parameters[0];
            let M = ctx.simparam_or("gmin", A);
            let O = if N == A { 1.0 } else { 0.0 };
            let Q = if O != 0.0 {
                P
            } else {
                N
            };
            let S = Q * R;
            let T = C / S;
            let X = V.powf((V - W));
            let Y = C / X;
            let AC = Z + (((AA * K) * K) / (K + AB));
            let AF = (AC - AD) / AE;
            let AG = if AC < AD { 1.0 } else { 0.0 };
            let BT = if AG != 0.0 {
                let AH = AD + (AE * ((C + (AF.exp())).ln()));
                AH
            } else {
                let AI = AC + (AE * ((C + ((-AF).exp())).ln()));
                AI
            };
            let AJ = C / Z;
            let AL = C / AK;
            let AO = V.powf((V - AN));
            let AP = C / AO;
            let AT = AQ + (((AR * K) * K) / (K + AS));
            let AU = (AT - AD) / AE;
            let AV = if AT < AD { 1.0 } else { 0.0 };
            let CG = if AV != 0.0 {
                let AW = AD + (AE * ((C + (AU.exp())).ln()));
                AW
            } else {
                let AX = AT + (AE * ((C + ((-AU).exp())).ln()));
                AX
            };
            let AY = C / AQ;
            let AZ = C / AM;
            let BA = C - (C / parameters[82]);
            let BC = if BB < A { 1.0 } else { 0.0 };
            let BF;
            let BII;
            if BC != 0.0 {
                let BD = C - BB;
                let BE = -(BD.ln());
                let BNQ = ((BHW * BNP) * (BHV / BD)) * BNP;
                BF = BE;
                BII = BNQ;
            } else {
                BF = BB;
                BII = BHW;
            }
            let BH = if BF < BG { 1.0 } else { 0.0 };
            let BK;
            let BIJ;
            if BH != 0.0 {
                BK = BF;
                BIJ = BII;
            } else {
                let BI = C + (BF - BG);
                let BNR = BII * (BHV / BI);
                let BJ = BG + (BI.ln());
                BK = BJ;
                BIJ = BNR;
            }
            let BL = L + BK;
            let BM = BL / K;
            let BNS = BIJ / K;
            let BO = BN * BL;
            let BNT = BIJ * BN;
            let BP = C / BO;
            let BNU = ((BNT * BP) * BNP) / BO;
            let BQ = BP - (C / (BN * K));
            let BR = BL - K;
            let BS = BM.ln();
            let BNV = BNS * (BHV / BM);
            let BU = AA * BL;
            let BV = BL + AB;
            let BW = (BU * BL) / BV;
            let BX = BT - BW;
            let BNW = (((((BIJ * AA) * BL) + (BIJ * BU)) - (BIJ * BW)) / BV) * BNP;
            let BY = (BX - AD) / AE;
            let BNX = BNW / AE;
            let BZ = if BX < AD { 1.0 } else { 0.0 };
            let NF;
            let BIK;
            if BZ != 0.0 {
                let CA = BY.exp();
                let CB = C + CA;
                let BNZ = ((BNX * CA) * (BHV / CB)) * AE;
                let CC = AD + (AE * (CB.ln()));
                NF = CC;
                BIK = BNZ;
            } else {
                let CD = (-BY).exp();
                let CE = C + CD;
                let CF = BX + (AE * (CE.ln()));
                let BNY = BNW + ((((BNX * BNP) * CD) * (BHV / CE)) * AE);
                NF = CF;
                BIK = BNY;
            }
            let CH = AR * BL;
            let CI = BL + AS;
            let CJ = (CH * BL) / CI;
            let CK = CG - CJ;
            let BOA = (((((BIJ * AR) * BL) + (BIJ * CH)) - (BIJ * CJ)) / CI) * BNP;
            let CL = (CK - AD) / AE;
            let BOB = BOA / AE;
            let CM = if CK < AD { 1.0 } else { 0.0 };
            let NX;
            let BIL;
            if CM != 0.0 {
                let CN = CL.exp();
                let CO = C + CN;
                let BOD = ((BOB * CN) * (BHV / CO)) * AE;
                let CP = AD + (AE * (CO.ln()));
                NX = CP;
                BIL = BOD;
            } else {
                let CQ = (-CL).exp();
                let CR = C + CQ;
                let CS = CK + (AE * (CR.ln()));
                let BOC = BOA + ((((BOB * BNP) * CQ) * (BHV / CR)) * AE);
                NX = CS;
                BIL = BOC;
            }
            let CV = CU * BO;
            let CW = C - BM;
            let BOE = BNS * BNP;
            let CY = ((CV * BS) + (AK * BM)) + (CW * CX);
            let BOF = ((((BNT * CU) * BS) + (BNV * CV)) + (BNS * AK)) + (BOE * CX);
            let CZ = (AD - CY) / BO;
            let BOG = ((BOF * BNP) - (BNT * CZ)) / BO;
            let DA = if AD < CY { 1.0 } else { 0.0 };
            let GD;
            let BIM;
            if DA != 0.0 {
                let DB = CZ.exp();
                let DC = C + DB;
                let DD = DC.ln();
                let DE = CY + (BO * DD);
                let BOI = BOF + ((BNT * DD) + (((BOG * DB) * (BHV / DC)) * BO));
                GD = DE;
                BIM = BOI;
            } else {
                let DF = (-CZ).exp();
                let DG = C + DF;
                let DH = DG.ln();
                let BOH = (BNT * DH) + ((((BOG * BNP) * DF) * (BHV / DG)) * BO);
                let DI = AD + (BO * DH);
                GD = DI;
                BIM = BOH;
            }
            let DK = DJ * BO;
            let DN = CW * DM;
            let BOJ = BOE * DM;
            let DO = ((DK * BS) + (DL * BM)) + DN;
            let BOK = ((((BNT * DJ) * BS) + (BNV * DK)) + (BNS * DL)) + BOJ;
            let DP = (AD - DO) / BO;
            let BOL = ((BOK * BNP) - (BNT * DP)) / BO;
            let DQ = if AD < DO { 1.0 } else { 0.0 };
            let SN;
            let BIN;
            if DQ != 0.0 {
                let DR = DP.exp();
                let DS = C + DR;
                let DT = DS.ln();
                let DU = DO + (BO * DT);
                let BON = BOK + ((BNT * DT) + (((BOL * DR) * (BHV / DS)) * BO));
                SN = DU;
                BIN = BON;
            } else {
                let DV = (-DP).exp();
                let DW = C + DV;
                let DX = DW.ln();
                let BOM = (BNT * DX) + ((((BOL * BNP) * DV) * (BHV / DW)) * BO);
                let DY = AD + (BO * DX);
                SN = DY;
                BIN = BOM;
            }
            let EA = DZ * BO;
            let EC = ((EA * BS) + (EB * BM)) + DN;
            let BOO = ((((BNT * DZ) * BS) + (BNV * EA)) + (BNS * EB)) + BOJ;
            let ED = (AD - EC) / BO;
            let BOP = ((BOO * BNP) - (BNT * ED)) / BO;
            let EE = if AD < EC { 1.0 } else { 0.0 };
            let AZA;
            let BIO;
            if EE != 0.0 {
                let EF = ED.exp();
                let EG = C + EF;
                let EH = EG.ln();
                let EI = EC + (BO * EH);
                let BOR = BOO + ((BNT * EH) + (((BOP * EF) * (BHV / EG)) * BO));
                AZA = EI;
                BIO = BOR;
            } else {
                let EJ = (-ED).exp();
                let EK = C + EJ;
                let EL = EK.ln();
                let BOQ = (BNT * EL) + ((((BOP * BNP) * EJ) * (BHV / EK)) * BO);
                let EM = AD + (BO * EL);
                AZA = EM;
                BIO = BOQ;
            }
            let EO = EN * BO;
            let EP = AM * BM;
            let BOS = BNS * AM;
            let EQ = ((EO * BS) + EP) + DN;
            let BOT = ((((BNT * EN) * BS) + (BNV * EO)) + BOS) + BOJ;
            let ER = (AD - EQ) / BO;
            let BOU = ((BOT * BNP) - (BNT * ER)) / BO;
            let ES = if AD < EQ { 1.0 } else { 0.0 };
            let GP;
            let BIP;
            if ES != 0.0 {
                let ET = ER.exp();
                let EU = C + ET;
                let EV = EU.ln();
                let EW = EQ + (BO * EV);
                let BOW = BOT + ((BNT * EV) + (((BOU * ET) * (BHV / EU)) * BO));
                GP = EW;
                BIP = BOW;
            } else {
                let EX = (-ER).exp();
                let EY = C + EX;
                let EZ = EY.ln();
                let BOV = (BNT * EZ) + ((((BOU * BNP) * EX) * (BHV / EY)) * BO);
                let FA = AD + (BO * EZ);
                GP = FA;
                BIP = BOV;
            }
            let FC = FB * BO;
            let FD = ((FC * BS) + EP) + DN;
            let BOX = ((((BNT * FB) * BS) + (BNV * FC)) + BOS) + BOJ;
            let FE = (AD - FD) / BO;
            let BOY = ((BOX * BNP) - (BNT * FE)) / BO;
            let FF = if AD < FD { 1.0 } else { 0.0 };
            let GF;
            let BIQ;
            if FF != 0.0 {
                let FG = FE.exp();
                let FH = C + FG;
                let FI = FH.ln();
                let FJ = FD + (BO * FI);
                let BPA = BOX + ((BNT * FI) + (((BOY * FG) * (BHV / FH)) * BO));
                GF = FJ;
                BIQ = BPA;
            } else {
                let FK = (-FE).exp();
                let FL = C + FK;
                let FM = FL.ln();
                let BOZ = (BNT * FM) + ((((BOY * BNP) * FK) * (BHV / FL)) * BO);
                let FN = AD + (BO * FM);
                GF = FN;
                BIQ = BOZ;
            }
            let FP = FO * BO;
            let FS = ((FP * BS) + (FQ * BM)) + (CW * FR);
            let BPB = ((((BNT * FO) * BS) + (BNV * FP)) + (BNS * FQ)) + (BOE * FR);
            let FT = (AD - FS) / BO;
            let BPC = ((BPB * BNP) - (BNT * FT)) / BO;
            let FU = if AD < FS { 1.0 } else { 0.0 };
            let ADF;
            let BIR;
            if FU != 0.0 {
                let FV = FT.exp();
                let FW = C + FV;
                let FX = FW.ln();
                let FY = FS + (BO * FX);
                let BPE = BPB + ((BNT * FX) + (((BPC * FV) * (BHV / FW)) * BO));
                ADF = FY;
                BIR = BPE;
            } else {
                let FZ = (-FT).exp();
                let GA = C + FZ;
                let GB = GA.ln();
                let BPD = (BNT * GB) + ((((BPC * BNP) * FZ) * (BHV / GA)) * BO);
                let GC = AD + (BO * GB);
                ADF = GC;
                BIR = BPD;
            }
            let GE = C / GD;
            let BPF = ((BIM * GE) * BNP) / GD;
            let GG = C / GF;
            let BPG = ((BIQ * GG) * BNP) / GF;
            let GH = AK * GE;
            let GI = GH.powf(W);
            let BPH = (BPF * AK) * (W * (GH.powf((W - BHV))));
            let GJ = AM * GG;
            let GK = GJ.powf(AN);
            let BPI = AN - BHV;
            let BPJ = (BPG * AM) * (AN * (GJ.powf(BPI)));
            let GM = GL * GI;
            let BPK = BPH * GL;
            let GO = C - GN;
            let GQ = AM / GP;
            let BPL = ((((BIP * GQ) * BNP) / GP) * (AN * (GQ.powf(BPI)))) * GO;
            let GR = (GO * (GQ.powf(AN))) + GN;
            let GS = C / GR;
            let BPM = ((BPL * GS) * BNP) / GR;
            let GU = GT * GR;
            let BPN = BPL * GT;
            let GV = GN * GS;
            let BPO = BPM * GN;
            let GY = (BS * GX).exp();
            let GZ = GW * GY;
            let BPP = ((BNV * GX) * GY) * GW;
            let HA = if GZ < S { 1.0 } else { 0.0 };
            let ATQ;
            let BIS;
            if HA != 0.0 {
                ATQ = S;
                BIS = BPQ;
            } else {
                ATQ = GZ;
                BIS = BPP;
            }
            let HE = HC - HD;
            let HF = (BS * HE).exp();
            let HG = HB * HF;
            let BPR = ((BNV * HE) * HF) * HB;
            let HJ = (BS * HI).exp();
            let HK = HH * HJ;
            let BPS = ((BNV * HI) * HJ) * HH;
            let HL = if HK < S { 1.0 } else { 0.0 };
            let ATL;
            let BIT;
            if HL != 0.0 {
                ATL = S;
                BIT = BPQ;
            } else {
                ATL = HK;
                BIT = BPS;
            }
            let HO = (BS * HN).exp();
            let HP = HM * HO;
            let BPT = ((BNV * HN) * HO) * HM;
            let HS = (BS * HR).exp();
            let BPU = (BNV * HR) * HS;
            let HT = HQ * HS;
            let BPV = BPU * HQ;
            let HV = HU * HS;
            let BPW = BPU * HU;
            let HY = (BS * HX).exp();
            let HZ = HW * HY;
            let BPX = ((BNV * HX) * HY) * HW;
            let IB = if IA != A { 1.0 } else { 0.0 };
            let JS;
            let BIU;
            if IB != 0.0 {
                let ID = IC * (C + (BR * IA));
                let BPY = (BIJ * IA) * IC;
                let IE = (ID - C) / U;
                let BPZ = BPY / U;
                let IF = if ID < C { 1.0 } else { 0.0 };
                let IM;
                let BIV;
                if IF != 0.0 {
                    let IG = IE.exp();
                    let IH = C + IG;
                    let BQB = ((BPZ * IG) * (BHV / IH)) * U;
                    let II = C + (U * (IH.ln()));
                    IM = II;
                    BIV = BQB;
                } else {
                    let IJ = (-IE).exp();
                    let IK = C + IJ;
                    let IL = ID + (U * (IK.ln()));
                    let BQA = BPY + ((((BPZ * BNP) * IJ) * (BHV / IK)) * U);
                    IM = IL;
                    BIV = BQA;
                }
                let IN = IM - 6.931471805599453e-4f64;
                JS = IN;
                BIU = BIV;
            } else {
                JS = IC;
                BIU = BPQ;
            }
            let IP = if IO != A { 1.0 } else { 0.0 };
            let AAG;
            let BIW;
            if IP != 0.0 {
                let IR = IQ * (C + (BR * IO));
                let BQC = (BIJ * IO) * IQ;
                let IS = (IR - C) / U;
                let BQD = BQC / U;
                let IT = if IR < C { 1.0 } else { 0.0 };
                let JA;
                let BIX;
                if IT != 0.0 {
                    let IU = IS.exp();
                    let IV = C + IU;
                    let BQF = ((BQD * IU) * (BHV / IV)) * U;
                    let IW = C + (U * (IV.ln()));
                    JA = IW;
                    BIX = BQF;
                } else {
                    let IX = (-IS).exp();
                    let IY = C + IX;
                    let IZ = IR + (U * (IY.ln()));
                    let BQE = BQC + ((((BQD * BNP) * IX) * (BHV / IY)) * U);
                    JA = IZ;
                    BIX = BQE;
                }
                let JB = JA - 6.931471805599453e-4f64;
                AAG = JB;
                BIW = BIX;
            } else {
                AAG = IQ;
                BIW = BPQ;
            }
            let JE = JC * (C + (JD * BR));
            let BQG = (BIJ * JD) * JC;
            let JG = JE * JE;
            let BQH = BQG * JE;
            let BQI = BQH + BQH;
            let JH = if JE < A { 1.0 } else { 0.0 };
            let APC;
            let BIY;
            if JH != 0.0 {
                let JJ = (JG + JF).sqrt();
                let JK = JJ - JE;
                let JL = 5e-7f64 / JK;
                let BQL = ((((BQI * (BHV / (BQJ * JJ))) - BQG) * JL) * BNP) / JK;
                APC = JL;
                BIY = BQL;
            } else {
                let JM = (JG + JF).sqrt();
                let JN = JI * (JM + JE);
                let BQK = ((BQI * (BHV / (BQJ * JM))) + BQG) * JI;
                APC = JN;
                BIY = BQK;
            }
            let JR = ((JP - HC) - HD) + JQ;
            let JT = (BS * JR) / JS;
            let JU = JT.exp();
            let JV = JO * JU;
            let JW = -CX;
            let JX = (JW * BQ) / JS;
            let JY = JX.exp();
            let JZ = JV * JY;
            let BQM = ((((((BNV * JR) - (BIU * JT)) / JS) * JU) * JO) * JY) + (((((BNU * JW) - (BIU * JX)) / JS) * JY) * JV);
            let KB = C - HC;
            let KC = (BS * KB).exp();
            let KD = KA * KC;
            let BQN = ((BNV * KB) * KC) * KA;
            let KG = C - KF;
            let KH = (BS * KG).exp();
            let KI = KE * KH;
            let BQO = ((BNV * KG) * KH) * KE;
            let KM = KK - (V * KL);
            let KN = (BS * KM).exp();
            let KO = KJ * KN;
            let KP = -parameters[112];
            let KQ = KP * BQ;
            let BQP = BNU * KP;
            let KR = (KQ / KL).exp();
            let KS = KO * KR;
            let BQQ = ((((BNV * KM) * KN) * KJ) * KR) + (((BQP / KL) * KR) * KO);
            let KV = KK - (V * KU);
            let KW = (BS * KV).exp();
            let KX = KT * KW;
            let KY = -DM;
            let KZ = ((KY * BQ) / KU).exp();
            let LA = KX * KZ;
            let BQR = ((((BNV * KV) * KW) * KT) * KZ) + ((((BNU * KY) / KU) * KZ) * KX);
            let LC = (JP - GX) + JQ;
            let LD = BS * LC;
            let BQS = BNV * LC;
            let LF = (LD / LE).exp();
            let LG = LB * LF;
            let LH = -parameters[110];
            let LI = LH * BQ;
            let BQT = BNU * LH;
            let LJ = (LI / LE).exp();
            let LK = LG * LJ;
            let BQU = ((((BQS / LE) * LF) * LB) * LJ) + (((BQT / LE) * LJ) * LG);
            let LN = (LD / LM).exp();
            let LO = LL * LN;
            let LP = (LI / LM).exp();
            let LQ = LO * LP;
            let BQV = ((((BQS / LM) * LN) * LL) * LP) + (((BQT / LM) * LP) * LO);
            let LR = if parameters[23] == C { 1.0 } else { 0.0 };
            let ADV;
            let AED;
            let AFI;
            let BIZ;
            let BJA;
            let BJB;
            if LR != 0.0 {
                let LT = -parameters[106];
                let LU = ((LT * BQ) / LE).exp();
                let LV = LS * LU;
                let BQW = (((BNU * LT) / LE) * LU) * LS;
                let LX = -parameters[105];
                let LY = (LX * BQ).exp();
                let LZ = LW * LY;
                let BQX = ((BNU * LX) * LY) * LW;
                let MB = -parameters[107];
                let MC = ((MB * BQ) / LM).exp();
                let MD = MA * MC;
                let BQY = (((BNU * MB) / LM) * MC) * MA;
                ADV = LV;
                AED = LZ;
                AFI = MD;
                BIZ = BQW;
                BJA = BQX;
                BJB = BQY;
            } else {
                ADV = A;
                AED = A;
                AFI = A;
                BIZ = BPQ;
                BJA = BPQ;
                BJB = BPQ;
            }
            let MF = (JP - KF) + JQ;
            let MG = (BS * MF).exp();
            let MH = ME * MG;
            let MI = -parameters[111];
            let MJ = (MI * BQ).exp();
            let MK = MH * MJ;
            let BQZ = ((((BNV * MF) * MG) * ME) * MJ) + (((BNU * MI) * MJ) * MH);
            let MN = KK - (V * MM);
            let MO = (BS * MN).exp();
            let MP = ML * MO;
            let MQ = (KQ / MM).exp();
            let MR = MP * MQ;
            let BRA = ((((BNV * MN) * MO) * ML) * MQ) + (((BQP / MM) * MQ) * MP);
            let MU = JP / MT;
            let MV = (BS * MU).exp();
            let MW = MS * MV;
            let MX = (KQ / MT).exp();
            let MY = MW * MX;
            let BRB = ((((BNV * MU) * MV) * MS) * MX) + (((BQP / MT) * MX) * MW);
            let NA = BM.sqrt();
            let NB = MZ * NA;
            let ND = (NC * BR).exp();
            let NE = NB * ND;
            let BRC = (((BNS * (BHV / (BQJ * NA))) * MZ) * ND) + (((BIJ * NC) * ND) * NB);
            let NG = NF * AJ;
            let NI = NG.powf(NH);
            let BRD = (BIK * AJ) * (NH * (NG.powf(-1.5e0f64)));
            let NJ = C / GI;
            let BRE = ((BPH * NJ) * BNP) / GI;
            let NL = NK * NF;
            let NM = NL * NF;
            let NN = NM * NI;
            let NO = (NN * NJ) * AK;
            let NP = ((NO * GE) * AJ) * AJ;
            let BRF = (((((((((((BIK * NK) * NF) + (BIK * NL)) * NI) + (BRD * NM)) * NJ) + (BRE * NN)) * AK) * GE) + (BPF * NO)) * AJ) * AJ;
            let NR = NQ * NI;
            let NS = NR * GD;
            let NT = ((NS * GD) * AL) * AL;
            let NU = NT * GI;
            let NV = (NK - NP).exp();
            let NW = NU * NV;
            let BRG = ((((((((((BRD * NQ) * GD) + (BIM * NR)) * GD) + (BIM * NS)) * AL) * AL) * GI) + (BPH * NT)) * NV) + (((BRF * BNP) * NV) * NU);
            let NY = NX * AY;
            let OA = NY.powf(NZ);
            let BRH = (BIL * AY) * (NZ * (NY.powf(-1.5e0f64)));
            let OB = C / GK;
            let OD = OC * NX;
            let OE = OD * NX;
            let OF = OE * OA;
            let OG = (OF * OB) * AM;
            let OH = ((OG * GG) * AY) * AY;
            let BRI = (((((((((((BIL * OC) * NX) + (BIL * OD)) * OA) + (BRH * OE)) * OB) + ((((BPJ * OB) * BNP) / GK) * OF)) * AM) * GG) + (BPG * OG)) * AY) * AY;
            let OJ = OI * OA;
            let OK = OJ * GF;
            let OL = ((OK * GF) * AZ) * AZ;
            let OM = OL * GK;
            let ON = (OC - OH).exp();
            let OO = OM * ON;
            let BRJ = ((((((((((BRH * OI) * GF) + (BIQ * OJ)) * GF) + (BIQ * OK)) * AZ) * AZ) * GK) + (BPJ * OL)) * ON) + (((BRI * BNP) * ON) * OM);
            let OP = (BS * HD).exp();
            let BRK = (BNV * HD) * OP;
            let OR = OQ * OP;
            let OS = OR * GS;
            let BRL = ((BRK * OQ) * GS) + (BPM * OR);
            let OU = OT * OP;
            let OV = OU * NJ;
            let BRM = ((BRK * OT) * NJ) + (BRE * OU);
            let OX = HC - V;
            let OY = (BS * OX).exp();
            let OZ = OW * OY;
            let PA = -parameters[119];
            let PB = (PA * BQ).exp();
            let PC = OZ * PB;
            let BRN = ((((BNV * OX) * OY) * OW) * PB) + (((BNU * PA) * PB) * OZ);
            let PE = (HD + HC) - C;
            let PF = (BS * PE).exp();
            let PG = PD * PF;
            let BRO = ((BNV * PE) * PF) * PD;
            let PI = HX - C;
            let PJ = (BS * PI).exp();
            let PK = PH * PJ;
            let BRP = ((BNV * PI) * PJ) * PH;
            let PM = PG + PK;
            let BRQ = BRO + BRP;
            let PN = PD + PH;
            let PO = (PL * PM) / PN;
            let BRR = (BRQ * PL) / PN;
            let PQ = parameters[99] - C;
            let PR = (BS * PQ).exp();
            let PS = PP * PR;
            let BRS = ((BNV * PQ) * PR) * PP;
            let PT = BL - 3e2f64;
            let PU = if BL < 5.25e2f64 { 1.0 } else { 0.0 };
            let ARC;
            let BJC;
            if PU != 0.0 {
                let PY = PX * PT;
                let PZ = PV * ((C + (PW * PT)) - (PY * PT));
                let BRT = ((BIJ * PW) - (((BIJ * PX) * PT) + (BIJ * PY))) * PV;
                ARC = PZ;
                BJC = BRT;
            } else {
                let QA = PV * 1.081e0f64;
                ARC = QA;
                BJC = BPQ;
            }
            let QC = QB * OP;
            let BRU = BRK * QB;
            let QF = QD * ((L / K).powf(QE));
            let QG = if HM > A { 1.0 } else { 0.0 };
            let AUT;
            let BJD;
            if QG != 0.0 {
                let QH = C / HP;
                let BRV = ((BPT * QH) * BNP) / HP;
                let QI = if QH > T { 1.0 } else { 0.0 };
                let AUU;
                let BJE;
                if QI != 0.0 {
                    AUU = T;
                    BJE = BPQ;
                } else {
                    AUU = QH;
                    BJE = BRV;
                }
                AUT = AUU;
                BJD = BJE;
            } else {
                AUT = A;
                BJD = BPQ;
            }
            let QJ = if HQ > A { 1.0 } else { 0.0 };
            let AUW;
            let BJF;
            if QJ != 0.0 {
                let QK = C / HT;
                let BRW = ((BPV * QK) * BNP) / HT;
                let QL = if QK > T { 1.0 } else { 0.0 };
                let AUX;
                let BJG;
                if QL != 0.0 {
                    AUX = T;
                    BJG = BPQ;
                } else {
                    AUX = QK;
                    BJG = BRW;
                }
                AUW = AUX;
                BJF = BJG;
            } else {
                AUW = A;
                BJF = BPQ;
            }
            let QM = if HU > A { 1.0 } else { 0.0 };
            let AUZ;
            let BJH;
            if QM != 0.0 {
                let QN = C / HV;
                let BRX = ((BPW * QN) * BNP) / HV;
                let QO = if QN > T { 1.0 } else { 0.0 };
                let AVA;
                let BJI;
                if QO != 0.0 {
                    AVA = T;
                    BJI = BPQ;
                } else {
                    AVA = QN;
                    BJI = BRX;
                }
                AUZ = AVA;
                BJH = BJI;
            } else {
                AUZ = A;
                BJH = BPQ;
            }
            let QR = B * (QP - QQ);
            let BRY = (Lanes([BHX, 0.0]) - Lanes([0.0, BHY])) * B;
            let QT = B * (QP - QS);
            let BRZ = (Lanes([BHX, 0.0]) - Lanes([0.0, BHZ])) * B;
            let QV = B * (QP - QU);
            let BSA = (Lanes([0.0, BHX]) - Lanes([BIA, 0.0])) * B;
            let QX = B * (QW - QU);
            let BSB = (Lanes([0.0, BIB]) - Lanes([BIA, 0.0])) * B;
            let QY = B * (QW - QP);
            let BSC = (Lanes([BIB, 0.0]) - Lanes([0.0, BHX])) * B;
            let QZ = B * (QQ - QS);
            let BSD = (Lanes([BHY, 0.0]) - Lanes([0.0, BHZ])) * B;
            let RB = B * (RA - QU);
            let BSE = (Lanes([BIC, 0.0]) - Lanes([0.0, BIA])) * B;
            let RD = B * (RC - QW);
            let BSF = (Lanes([BID, 0.0]) - Lanes([0.0, BIB])) * B;
            let RE = B * (RC - RA);
            let BSG = (Lanes([BID, 0.0]) - Lanes([0.0, BIC])) * B;
            let RF = B * (RC - node_potentials[0]);
            let BSH = (Lanes([0.0, BID]) - Lanes([BIE, 0.0])) * B;
            let RH = B * (RG - QQ);
            let BSI = (Lanes([0.0, BIF]) - Lanes([BHY, 0.0])) * B;
            let RI = B * (node_potentials[9] - RG);
            let BSJ = (Lanes([BIG, 0.0]) - Lanes([0.0, BIF])) * B;
            let BSK = Lanes([BSC[0], BSC[1], 0.0]) + Lanes([0.0, BRZ[0], BRZ[1]]);
            let BSL = Lanes([BSK[0], BSK[1], 0.0, BSK[2]]) - Lanes([0.0, 0.0, BSD[0], BSD[1]]);
            let RJ = ((QY + QT) - QZ) - RH;
            let BSM = Lanes([BSL[0], BSL[1], BSL[2], BSL[3], 0.0]) - Lanes([0.0, 0.0, BSI[0], 0.0, BSI[1]]);
            let BSN = BSH * BNP;
            let BSO = Lanes([BSN[0], BSN[1], 0.0]) + Lanes([0.0, BSF[0], BSF[1]]);
            let BSP = Lanes([BSO[0], BSO[1], BSO[2], 0.0, 0.0, 0.0, 0.0]) + Lanes([0.0, 0.0, BSM[0], BSM[1], BSM[2], BSM[3], BSM[4]]);
            let RK = (((-RF) + RD) + RJ) - RI;
            let BSQ = Lanes([BSP[0], BSP[1], BSP[2], BSP[3], BSP[4], BSP[5], 0.0, BSP[6]]) - Lanes([0.0, 0.0, 0.0, 0.0, 0.0, 0.0, BSJ[0], BSJ[1]]);
            let RL = RF + RK;
            let BSR = Lanes([BSH[0], BSH[1], 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]) + BSQ;
            let RM = QT * BP;
            let BSS = BRZ * BP;
            let BST = Lanes([0.0, BSS[0], BSS[1]]) + Lanes([(BNU * QT), 0.0, 0.0]);
            let RO = if RM < RN { 1.0 } else { 0.0 };
            let XK;
            let BJJ;
            if RO != 0.0 {
                let RP = RM.exp();
                let BSV = BST * RP;
                XK = RP;
                BJJ = BSV;
            } else {
                let RQ = RN.exp();
                let RR = RQ * (C + (RM - RN));
                let BSU = BST * RQ;
                XK = RR;
                BJJ = BSU;
            }
            let RS = QV * BP;
            let BSW = BSA * BP;
            let BSX = Lanes([0.0, BSW[0], BSW[1]]) + Lanes([(BNU * QV), 0.0, 0.0]);
            let RT = RS / JS;
            let BSY = (BSX - Lanes([(BIU * RT), 0.0, 0.0])) / JS;
            let RU = if RT < RN { 1.0 } else { 0.0 };
            let AAA;
            let BJK;
            if RU != 0.0 {
                let RV = RT.exp();
                let BTA = BSY * RV;
                AAA = RV;
                BJK = BTA;
            } else {
                let RW = RN.exp();
                let RX = RW * (C + (RT - RN));
                let BSZ = BSY * RW;
                AAA = RX;
                BJK = BSZ;
            }
            let RY = RJ * BP;
            let BTB = BSM * BP;
            let BTC = Lanes([0.0, BTB[0], BTB[1], BTB[2], BTB[3], BTB[4]]) + Lanes([(BNU * RJ), 0.0, 0.0, 0.0, 0.0, 0.0]);
            let RZ = if RY < RN { 1.0 } else { 0.0 };
            let AKP;
            let BJL;
            if RZ != 0.0 {
                let SA = RY.exp();
                let BTE = BTC * SA;
                AKP = SA;
                BJL = BTE;
            } else {
                let SB = RN.exp();
                let SC = SB * (C + (RY - RN));
                let BTD = BTC * SB;
                AKP = SC;
                BJL = BTD;
            }
            let SD = QY * BP;
            let BTF = BSC * BP;
            let BTG = Lanes([0.0, BTF[0], BTF[1]]) + Lanes([(BNU * QY), 0.0, 0.0]);
            let SE = if SD < RN { 1.0 } else { 0.0 };
            let AOL;
            let BJM;
            if SE != 0.0 {
                let SF = SD.exp();
                let BTI = BTG * SF;
                AOL = SF;
                BJM = BTI;
            } else {
                let SG = RN.exp();
                let SH = SG * (C + (SD - RN));
                let BTH = BTG * SG;
                AOL = SH;
                BJM = BTH;
            }
            let SI = RL * BP;
            let BTJ = BSR * BP;
            let BTK = Lanes([BTJ[0], BTJ[1], 0.0, BTJ[2], BTJ[3], BTJ[4], BTJ[5], BTJ[6], BTJ[7]]) + Lanes([0.0, 0.0, (BNU * RL), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
            let SJ = if SI < RN { 1.0 } else { 0.0 };
            let ALL;
            let BJN;
            if SJ != 0.0 {
                let SK = SI.exp();
                let BTM = BTK * SK;
                ALL = SK;
                BJN = BTM;
            } else {
                let SL = RN.exp();
                let SM = SL * (C + (SI - RN));
                let BTL = BTK * SL;
                ALL = SM;
                BJN = BTL;
            }
            let SO = RL - SN;
            let BTN = Lanes([BSR[0], BSR[1], 0.0, BSR[2], BSR[3], BSR[4], BSR[5], BSR[6], BSR[7]]);
            let SP = SO * BP;
            let BTO = ((BTN - Lanes([0.0, 0.0, BIN, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])) * BP) + Lanes([0.0, 0.0, (BNU * SO), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
            let SQ = if SP < RN { 1.0 } else { 0.0 };
            let AZU;
            let BJO;
            if SQ != 0.0 {
                let SR = SP.exp();
                let BTQ = BTO * SR;
                AZU = SR;
                BJO = BTQ;
            } else {
                let SS = RN.exp();
                let ST = SS * (C + (SP - RN));
                let BTP = BTO * SS;
                AZU = ST;
                BJO = BTP;
            }
            let SU = RJ - SN;
            let BTR = Lanes([0.0, BSM[0], BSM[1], BSM[2], BSM[3], BSM[4]]);
            let SV = SU * BP;
            let BTS = ((BTR - Lanes([BIN, 0.0, 0.0, 0.0, 0.0, 0.0])) * BP) + Lanes([(BNU * SU), 0.0, 0.0, 0.0, 0.0, 0.0]);
            let SW = if SV < RN { 1.0 } else { 0.0 };
            let AKR;
            let BJP;
            if SW != 0.0 {
                let SX = SV.exp();
                let BTU = BTS * SX;
                AKR = SX;
                BJP = BTU;
            } else {
                let SY = RN.exp();
                let SZ = SY * (C + (SV - RN));
                let BTT = BTS * SY;
                AKR = SZ;
                BJP = BTT;
            }
            let TA = QT - SN;
            let TB = TA * BP;
            let BTV = ((Lanes([0.0, BRZ[0], BRZ[1]]) - Lanes([BIN, 0.0, 0.0])) * BP) + Lanes([(BNU * TA), 0.0, 0.0]);
            let TC = if TB < RN { 1.0 } else { 0.0 };
            let TM;
            let BJQ;
            if TC != 0.0 {
                let TD = TB.exp();
                let BTX = BTV * TD;
                TM = TD;
                BJQ = BTX;
            } else {
                let TE = RN.exp();
                let TF = TE * (C + (TB - RN));
                let BTW = BTV * TE;
                TM = TF;
                BJQ = BTW;
            }
            let TG = QR - SN;
            let BTY = Lanes([0.0, BRY[0], BRY[1]]);
            let BTZ = Lanes([BIN, 0.0, 0.0]);
            let TH = TG * BP;
            let BUA = ((BTY - BTZ) * BP) + Lanes([(BNU * TG), 0.0, 0.0]);
            let TI = if TH < RN { 1.0 } else { 0.0 };
            let TO;
            let BJR;
            if TI != 0.0 {
                let TJ = TH.exp();
                let BUC = BUA * TJ;
                TO = TJ;
                BJR = BUC;
            } else {
                let TK = RN.exp();
                let TL = TK * (C + (TH - RN));
                let BUB = BUA * TK;
                TO = TL;
                BJR = BUB;
            }
            let TN = (C + (JP * TM)).sqrt();
            let BUD = (BJQ * JP) * (BHV / (BQJ * TN));
            let TP = (C + (JP * TO)).sqrt();
            let BUE = (BJR * JP) * (BHV / (BQJ * TP));
            let TQ = C + TP;
            let TR = (V * TO) / TQ;
            let BUF = ((BJR * V) - (BUE * TR)) / TQ;
            let TT = if TR < TS { 1.0 } else { 0.0 };
            let VX;
            let BJS;
            if TT != 0.0 {
                VX = TS;
                BJS = BUG;
            } else {
                VX = TR;
                BJS = BUF;
            }
            let BUH = Lanes([BUD[0], BUD[1], 0.0, BUD[2]]);
            let TU = TN + C;
            let TV = TU / TQ;
            let BUI = BUE * TV;
            let TW = (TN - TP) - (TV.ln());
            let TX = BO * TW;
            let BUJ = Lanes([(BNT * TW), 0.0, 0.0, 0.0]) + (((BUH - Lanes([BUE[0], BUE[1], BUE[2], 0.0])) - (((BUH - Lanes([BUI[0], BUI[1], BUI[2], 0.0])) / TQ) * (BHV / TV))) * BO);
            let BUK = Lanes([0.0, 0.0, BSD[0], BSD[1]]);
            let TY = (TX + QZ) / HZ;
            let BUL = ((BUJ + BUK) - Lanes([(BPX * TY), 0.0, 0.0, 0.0])) / HZ;
            let TZ = if TY > A { 1.0 } else { 0.0 };
            let YQ;
            let YZ;
            let ZL;
            let AAF;
            let APZ;
            let AQU;
            let AYT;
            let BJT;
            let BJU;
            let BJV;
            let BJW;
            let BJX;
            let BJY;
            let BJZ;
            if TZ != 0.0 {
                let UB = if QR < UA { 1.0 } else { 0.0 };
                let UJ;
                let BKA;
                if UB != 0.0 {
                    UJ = QR;
                    BKA = BRY;
                } else {
                    let UC = C + (QR - UA);
                    let BUV = BRY * (BHV / UC);
                    let UD = UA + (UC.ln());
                    UJ = UD;
                    BKA = BUV;
                }
                let UE = V * BO;
                let UF = JI * TY;
                let UG = UF * HZ;
                let BUW = ((BUL * JI) * HZ) + Lanes([(BPX * UF), 0.0, 0.0, 0.0]);
                let UH = (UG * BP) + C;
                let UI = UH.ln();
                let UK = (SN + (UE * UI)) - UJ;
                let BUX = (Lanes([BIN, 0.0, 0.0, 0.0]) + (Lanes([((BNT * V) * UI), 0.0, 0.0, 0.0]) + ((((BUW * BP) + Lanes([(BNU * UG), 0.0, 0.0, 0.0])) * (BHV / UH)) * UE))) - Lanes([0.0, BKA[0], BKA[1], 0.0]);
                let UM = UL * SN;
                let UN = UM * UM;
                let BUY = (BIN * UL) * UM;
                let BUZ = BUY + BUY;
                let UO = UK * UK;
                let BVA = BUX * UK;
                let BVB = BVA + BVA;
                let UP = if UK < A { 1.0 } else { 0.0 };
                let UV;
                let BKB;
                if UP != 0.0 {
                    let UQ = (UO + UN).sqrt();
                    let UR = UQ - UK;
                    let US = (JI * UN) / UR;
                    let BVD = (Lanes([(BUZ * JI), 0.0, 0.0, 0.0]) - ((((BVB + Lanes([BUZ, 0.0, 0.0, 0.0])) * (BHV / (BQJ * UQ))) - BUX) * US)) / UR;
                    UV = US;
                    BKB = BVD;
                } else {
                    let UT = (UO + UN).sqrt();
                    let UU = JI * (UT + UK);
                    let BVC = (((BVB + Lanes([BUZ, 0.0, 0.0, 0.0])) * (BHV / (BQJ * UT))) + BUX) * JI;
                    UV = UU;
                    BKB = BVC;
                }
                let UY = UW * UX;
                let UZ = UV + UY;
                let VA = UX * (UV + (UW * HZ));
                let VB = (UV * UZ) / VA;
                let BVE = (((BKB * UZ) + (BKB * UV)) - (((BKB + Lanes([(BPX * UW), 0.0, 0.0, 0.0])) * UX) * VB)) / VA;
                let VC = TY / VB;
                let BVF = (BUL - (BVE * VC)) / VB;
                let VE = (VC - C) / VD;
                let BVG = BVF / VD;
                let VF = if VC < C { 1.0 } else { 0.0 };
                let VM;
                let BKC;
                if VF != 0.0 {
                    let VG = VE.exp();
                    let VH = C + VG;
                    let BVI = ((BVG * VG) * (BHV / VH)) * VD;
                    let VI = C + (VD * (VH.ln()));
                    VM = VI;
                    BKC = BVI;
                } else {
                    let VJ = (-VE).exp();
                    let VK = C + VJ;
                    let VL = VC + (VD * (VK.ln()));
                    let BVH = BVF + ((((BVG * BNP) * VJ) * (BHV / VK)) * VD);
                    VM = VL;
                    BKC = BVH;
                }
                let VN = C + (VD * ((C + ((-1e0f64 / VD).exp())).ln()));
                let VO = VM / VN;
                let BVJ = BKC / VN;
                let VP = UV / UY;
                let BVK = BKB / UY;
                let VQ = JP * VO;
                let VR = VQ * VP;
                let VS = C + VP;
                let VT = (C + (VR * VS)).sqrt();
                let VU = V * VO;
                let VV = VU * VS;
                let VW = (C + VT) / VV;
                let BVL = (((((((BVJ * JP) * VP) + (BVK * VQ)) * VS) + (BVK * VR)) * (BHV / (BQJ * VT))) - ((((BVJ * V) * VS) + (BVK * VU)) * VW)) / VV;
                let VY = VX * VW;
                let BVM = BJS * VW;
                let BVN = Lanes([BVM[0], BVM[1], BVM[2], 0.0]) + (BVL * VX);
                let VZ = C + VY;
                let WA = ((C - VW) + VY) / VZ;
                let BVO = (((BVL * BNP) + BVN) - (BVN * WA)) / VZ;
                let WB = UG * WA;
                let WC = WB * BP;
                let BVP = (((BUW * WA) + (BVO * UG)) * BP) + Lanes([(BNU * WB), 0.0, 0.0, 0.0]);
                let WD = (VX + WC) + C;
                let BVQ = BJS * WD;
                let WE = (V * WC) + (VX * WD);
                let BVR = (BVP * V) + (Lanes([BVQ[0], BVQ[1], BVQ[2], 0.0]) + ((Lanes([BJS[0], BJS[1], BJS[2], 0.0]) + BVP) * VX));
                let WF = JI * (WC - C);
                let BVS = BVP * JI;
                let BVT = BVS * WF;
                let WG = (WF * WF) + WE;
                let BVU = (BVT + BVT) + BVR;
                let WH = if WC >= C { 1.0 } else { 0.0 };
                let WN;
                let BKD;
                if WH != 0.0 {
                    let WI = WG.sqrt();
                    let WJ = WF + WI;
                    let BVW = BVS + (BVU * (BHV / (BQJ * WI)));
                    WN = WJ;
                    BKD = BVW;
                } else {
                    let WK = WG.sqrt();
                    let WL = WK - WF;
                    let WM = WE / WL;
                    let BVV = (BVR - (((BVU * (BHV / (BQJ * WK))) - BVS) * WM)) / WL;
                    WN = WM;
                    BKD = BVV;
                }
                let WP = if WN < WO { 1.0 } else { 0.0 };
                let WQ;
                let BKE;
                if WP != 0.0 {
                    WQ = WO;
                    BKE = BUU;
                } else {
                    WQ = WN;
                    BKE = BKD;
                }
                let WR = WQ + C;
                let WS = WQ * WR;
                let WT = (SN * BP).exp();
                let WU = WS * WT;
                let BVX = (((BKE * WR) + (BKE * WQ)) * WT) + Lanes([((((BIN * BP) + (BNU * SN)) * WT) * WS), 0.0, 0.0, 0.0]);
                let WV = JI * UX;
                let WW = WV * (TY - UW);
                let BVY = BUL * WV;
                let WX = (UX * HZ) * UW;
                let BVZ = BVY * WW;
                let WY = ((WW * WW) + (WX * TY)).sqrt();
                let WZ = WW + WY;
                let BWA = BVY + (((BVZ + BVZ) + (Lanes([(((BPX * UX) * UW) * TY), 0.0, 0.0, 0.0]) + (BUL * WX))) * (BHV / (BQJ * WY)));
                let XA = if parameters[72] == A { 1.0 } else { 0.0 };
                let ZA;
                let BKF;
                if XA != 0.0 {
                    let XB = GP * AE;
                    let BWC = Lanes([(BIP * AE), 0.0, 0.0, 0.0]);
                    ZA = XB;
                    BKF = BWC;
                } else {
                    let XC = TY + VB;
                    let XD = (V * TY) / XC;
                    let XE = AE + XD;
                    let XF = GP * XE;
                    let BWB = Lanes([(BIP * XE), 0.0, 0.0, 0.0]) + ((((BUL * V) - ((BUL + BVE) * XD)) / XC) * GP);
                    ZA = XF;
                    BKF = BWB;
                }
                let XG = UW + TY;
                let XH = (UW * TY) / XG;
                let BWD = ((BUL * UW) - (BUL * XH)) / XG;
                let XI = UW / XG;
                let BWE = ((BUL * XI) * BNP) / XG;
                YQ = WZ;
                YZ = ZA;
                ZL = XI;
                AAF = WU;
                APZ = WA;
                AQU = XH;
                AYT = WQ;
                BJT = BWA;
                BJU = BKF;
                BJV = BWE;
                BJW = BVX;
                BJX = BVO;
                BJY = BWD;
                BJZ = BKE;
            } else {
                let XJ = (V * TM) / TU;
                let BUM = ((BJQ * V) - (BUD * XJ)) / TU;
                let XL = if (if (QZ.abs()) < (1e-5f64 * BO) { 1.0 } else { 0.0 }) != 0.0 || (if (TX.abs()) < ((1e-40f64 * BO) * (TN + TP)) { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let AQA;
                let BKG;
                if XL != 0.0 {
                    let XM = JI * (XJ + VX);
                    let BUO = (Lanes([BUM[0], BUM[1], 0.0, BUM[2]]) + Lanes([BJS[0], BJS[1], BJS[2], 0.0])) * JI;
                    let XN = XM + C;
                    let XO = XM / XN;
                    let BUP = (BUO - (BUO * XO)) / XN;
                    AQA = XO;
                    BKG = BUP;
                } else {
                    let XP = (TX + QT) - QR;
                    let XQ = TX / XP;
                    let BUN = (BUJ - (((BUJ + Lanes([0.0, BRZ[0], 0.0, BRZ[1]])) - Lanes([0.0, BRY[0], BRY[1], 0.0])) * XQ)) / XP;
                    AQA = XQ;
                    BKG = BUN;
                }
                let XR = AE * GP;
                let XS = C - (TY / UW);
                let BUQ = (BUL / UW) * BNP;
                let BUR = Lanes([(BIP * AE), 0.0, 0.0, 0.0]);
                let BUS = Lanes([BJJ[0], BJJ[1], 0.0, BJJ[2]]);
                let BUT = Lanes([BUM[0], BUM[1], 0.0, BUM[2]]);
                YQ = QZ;
                YZ = XR;
                ZL = XS;
                AAF = XK;
                APZ = AQA;
                AQU = TY;
                AYT = XJ;
                BJT = BUK;
                BJU = BUR;
                BJV = BUQ;
                BJW = BUS;
                BJX = BKG;
                BJY = BUL;
                BJZ = BUT;
            }
            let XT = C - (CT.powf((-1e0f64 / W)));
            let XU = GD * XT;
            let BWF = BIM * XT;
            let XV = AE * GD;
            let BWG = BIM * AE;
            let BWH = Lanes([0.0, BSA[0], BSA[1]]);
            let BWI = Lanes([BWF, 0.0, 0.0]);
            let XW = (QV - XU) / XV;
            let BWJ = ((BWH - BWI) - Lanes([(BWG * XW), 0.0, 0.0])) / XV;
            let XX = if QV < XU { 1.0 } else { 0.0 };
            let YG;
            let BKH;
            if XX != 0.0 {
                let XY = XW.exp();
                let XZ = C + XY;
                let YA = XZ.ln();
                let YB = QV - (XV * YA);
                let BWL = BWH - (Lanes([(BWG * YA), 0.0, 0.0]) + (((BWJ * XY) * (BHV / XZ)) * XV));
                YG = YB;
                BKH = BWL;
            } else {
                let YC = (-XW).exp();
                let YD = C + YC;
                let YE = YD.ln();
                let YF = XU - (XV * YE);
                let BWK = BWI - (Lanes([(BWG * YE), 0.0, 0.0]) + ((((BWJ * BNP) * YC) * (BHV / YD)) * XV));
                YG = YF;
                BKH = BWK;
            }
            let YH = C - (YG * GE);
            let BWM = ((BKH * GE) + Lanes([(BPF * YG), 0.0, 0.0])) * BNP;
            let YI = C - W;
            let YJ = YH.powf(YI);
            let BWN = YI - BHV;
            let BWO = BWM * (YI * (YH.powf(BWN)));
            let YK = GD / YI;
            let BWP = BIM / YI;
            let YL = C - YJ;
            let YM = (YK * YL) + (CT * (QV - YG));
            let BWQ = (Lanes([(BWP * YL), 0.0, 0.0]) + ((BWO * BNP) * YK)) + ((BWH - BKH) * CT);
            let YO = if YN == C { 1.0 } else { 0.0 };
            let YX;
            let BKI;
            if YO != 0.0 {
                let BWT = Lanes([0.0, BRY[0], BRY[1], 0.0]);
                YX = QR;
                BKI = BWT;
            } else {
                let YP = if YN == V { 1.0 } else { 0.0 };
                let YY;
                let BKJ;
                if YP != 0.0 {
                    let YR = QR + YQ;
                    let BWS = Lanes([0.0, BRY[0], BRY[1], 0.0]) + BJT;
                    YY = YR;
                    BKJ = BWS;
                } else {
                    let BWR = Lanes([0.0, BRZ[0], 0.0, BRZ[1]]);
                    YY = QT;
                    BKJ = BWR;
                }
                YX = YY;
                BKI = BKJ;
            }
            let BWU = BPO * BNP;
            let YS = C - GV;
            let YT = (V - GV) / YS;
            let BWV = (BWU - (BWU * YT)) / YS;
            let YU = -1e0f64 / AN;
            let YV = C - (YT.powf(YU));
            let YW = GP * YV;
            let BWW = (BIP * YV) + (((BWV * (YU * (YT.powf((YU - BHV))))) * BNP) * GP);
            let BWX = Lanes([BWW, 0.0, 0.0, 0.0]);
            let ZB = (YX - YW) / YZ;
            let BWY = ((BKI - BWX) - (BJU * ZB)) / YZ;
            let ZC = if YX < YW { 1.0 } else { 0.0 };
            let ZQ;
            let BKK;
            if ZC != 0.0 {
                let ZD = ZB.exp();
                let ZE = C + ZD;
                let ZF = ZE.ln();
                let ZG = YX - (YZ * ZF);
                let BXA = BKI - ((BJU * ZF) + (((BWY * ZD) * (BHV / ZE)) * YZ));
                ZQ = ZG;
                BKK = BXA;
            } else {
                let ZH = (-ZB).exp();
                let ZI = C + ZH;
                let ZJ = ZI.ln();
                let ZK = YW - (YZ * ZJ);
                let BWZ = BWX - ((BJU * ZJ) + ((((BWY * BNP) * ZH) * (BHV / ZI)) * YZ));
                ZQ = ZK;
                BKK = BWZ;
            }
            let ZN = ZL.powf(ZM);
            let BXB = BJV * (ZM * (ZL.powf((ZM - BHV))));
            let ZO = C - AN;
            let ZP = GP / ZO;
            let BXC = BIP / ZO;
            let ZR = ZQ / GP;
            let ZS = C - ZR;
            let ZT = ZS.powf(ZO);
            let BXD = ZO - BHV;
            let ZU = C - (ZN * ZT);
            let ZV = ZN * YT;
            let ZW = YX - ZQ;
            let ZX = (ZP * ZU) + (ZV * ZW);
            let BXE = BRY * GV;
            let BXF = Lanes([(BPO * QR), 0.0, 0.0]) + Lanes([0.0, BXE[0], BXE[1]]);
            let ZY = (YS * ZX) + (GV * QR);
            let BXG = (Lanes([(BWU * ZX), 0.0, 0.0, 0.0]) + (((Lanes([(BXC * ZU), 0.0, 0.0, 0.0]) + ((((BXB * ZT) + (((((BKK - Lanes([(BIP * ZR), 0.0, 0.0, 0.0])) / GP) * BNP) * (ZO * (ZS.powf(BXD)))) * ZN)) * BNP) * ZP)) + ((((BXB * YT) + Lanes([(BWV * ZN), 0.0, 0.0, 0.0])) * ZW) + ((BKI - BKK) * ZV))) * YS)) + Lanes([BXF[0], BXF[1], BXF[2], 0.0]);
            let ZZ = (JP * JZ) / KD;
            let BXH = ((BQM * JP) - (BQN * ZZ)) / KD;
            let AAB = ZZ * AAA;
            let BXI = Lanes([(BXH * AAA), 0.0, 0.0]) + (BJK * ZZ);
            let AAC = (C + AAB).sqrt();
            let BXJ = BXI * (BHV / (BQJ * AAC));
            let AAD = C + AAC;
            let AAE = AAB / AAD;
            let BXK = (BXI - (BXJ * AAE)) / AAD;
            let AAH = C / AAG;
            let AAI = AAF.powf(AAH);
            let BXL = AAF.ln();
            let BXM = (BJW * (AAH * (AAF.powf((AAH - BHV))))) + Lanes([((((BIW * AAH) * BNP) / AAG) * (AAI * BXL)), 0.0, 0.0, 0.0]);
            let AAJ = ZZ * AAI;
            let BXN = Lanes([(BXH * AAI), 0.0, 0.0, 0.0]) + (BXM * ZZ);
            let AAK = (C + AAJ).sqrt();
            let AAL = C + AAK;
            let AAM = AAJ / AAL;
            let BXO = (BXN - ((BXN * (BHV / (BQJ * AAK))) * AAM)) / AAL;
            let AAN = if QB == A { 1.0 } else { 0.0 };
            let ABC;
            let BKL;
            if AAN != 0.0 {
                let AAO = YM / OV;
                let BXS = (BWQ - Lanes([(BRM * AAO), 0.0, 0.0])) / OV;
                let AAP = ZY / OS;
                let BXT = (BXG - Lanes([(BRL * AAP), 0.0, 0.0, 0.0])) / OS;
                let AAQ = (C + AAO) + AAP;
                let BXU = Lanes([BXS[0], BXS[1], BXS[2], 0.0, 0.0]) + Lanes([BXT[0], 0.0, BXT[1], BXT[2], BXT[3]]);
                ABC = AAQ;
                BKL = BXU;
            } else {
                let AAR = YM / OV;
                let AAS = AAR + C;
                let AAT = AAS * QC;
                let AAU = (-ZY) / OS;
                let AAV = AAU * QC;
                let AAW = (AAT * BP).exp();
                let BXP = ((((((BWQ - Lanes([(BRM * AAR), 0.0, 0.0])) / OV) * QC) + Lanes([(BRU * AAS), 0.0, 0.0])) * BP) + Lanes([(BNU * AAT), 0.0, 0.0])) * AAW;
                let AAX = (AAV * BP).exp();
                let BXQ = (((((((BXG * BNP) - Lanes([(BRL * AAU), 0.0, 0.0, 0.0])) / OS) * QC) + Lanes([(BRU * AAU), 0.0, 0.0, 0.0])) * BP) + Lanes([(BNU * AAV), 0.0, 0.0, 0.0])) * AAX;
                let AAY = (QC * BP).exp();
                let AAZ = AAY - C;
                let ABA = (AAW - AAX) / AAZ;
                let BXR = ((Lanes([BXP[0], BXP[1], BXP[2], 0.0, 0.0]) - Lanes([BXQ[0], 0.0, BXQ[1], BXQ[2], BXQ[3]])) - Lanes([((((BRU * BP) + (BNU * QC)) * AAY) * ABA), 0.0, 0.0, 0.0, 0.0])) / AAZ;
                ABC = ABA;
                BKL = BXR;
            }
            let ABD = ABC * ABC;
            let BXV = BKL * ABC;
            let BXW = BXV + BXV;
            let ABE = if ABC < A { 1.0 } else { 0.0 };
            let ABK;
            let BKM;
            if ABE != 0.0 {
                let ABF = (ABD + ABB).sqrt();
                let ABG = ABF - ABC;
                let ABH = 5.000000000000001e-3f64 / ABG;
                let BXY = ((((BXW * (BHV / (BQJ * ABF))) - BKL) * ABH) * BNP) / ABG;
                ABK = ABH;
                BKM = BXY;
            } else {
                let ABI = (ABD + ABB).sqrt();
                let ABJ = JI * (ABI + ABC);
                let BXX = ((BXW * (BHV / (BQJ * ABI))) + BKL) * JI;
                ABK = ABJ;
                BKM = BXX;
            }
            let BXZ = (Lanes([BXK[0], BXK[1], BXK[2], 0.0, 0.0]) + Lanes([BXO[0], 0.0, BXO[1], BXO[2], BXO[3]])) * JI;
            let ABL = C + (JI * (AAE + AAM));
            let ABM = ABK * ABL;
            let BYA = (BKM * ABL) + (BXZ * ABK);
            let ABO = ABN * JZ;
            let ABP = ABO * AAI;
            let BYB = Lanes([((BQM * ABN) * AAI), 0.0, 0.0, 0.0]) + (BXM * ABO);
            let ABQ = JZ * AAA;
            let BYC = Lanes([(BQM * AAA), 0.0, 0.0]) + (BJK * JZ);
            let BYD = Lanes([BYC[0], BYC[1], BYC[2], 0.0, 0.0]);
            let BYE = Lanes([BYB[0], 0.0, BYB[1], BYB[2], BYB[3]]);
            let ABR = (ABQ - ABP) / ABM;
            let BYF = ((BYD - BYE) - (BYA * ABR)) / ABM;
            let ABT = QV / ABS;
            let BYG = BSA / ABS;
            let ABU = if QV < A { 1.0 } else { 0.0 };
            let ACB;
            let BKN;
            if ABU != 0.0 {
                let ABV = ABT.exp();
                let ABW = C + ABV;
                let ABX = ABS * (ABW.ln());
                let BYI = ((BYG * ABV) * (BHV / ABW)) * ABS;
                ACB = ABX;
                BKN = BYI;
            } else {
                let ABY = (-ABT).exp();
                let ABZ = C + ABY;
                let ACA = QV + (ABS * (ABZ.ln()));
                let BYH = BSA + ((((BYG * BNP) * ABY) * (BHV / ABZ)) * ABS);
                ACB = ACA;
                BKN = BYH;
            }
            let ACD = ACB / ACC;
            let BYJ = BKN / ACC;
            let ACE = if ACD < RN { 1.0 } else { 0.0 };
            let ACI;
            let BKO;
            if ACE != 0.0 {
                let ACF = ACD.exp();
                let BYL = BYJ * ACF;
                ACI = ACF;
                BKO = BYL;
            } else {
                let ACG = RN.exp();
                let ACH = ACG * (C + (ACD - RN));
                let BYK = BYJ * ACG;
                ACI = ACH;
                BKO = BYK;
            }
            let ACJ = ACI - C;
            let ACK = NE * ACJ;
            let BYM = BKO * NE;
            let BYN = Lanes([(BRC * ACJ), 0.0, 0.0]) + Lanes([0.0, BYM[0], BYM[1]]);
            let ACM = (QV - ACL) / U;
            let BYO = BSA / U;
            let ACN = if QV < ACL { 1.0 } else { 0.0 };
            let ACV;
            let BKP;
            if ACN != 0.0 {
                let ACO = ACM.exp();
                let ACP = C + ACO;
                let ACQ = QV - (U * (ACP.ln()));
                let BYQ = BSA - (((BYO * ACO) * (BHV / ACP)) * U);
                ACV = ACQ;
                BKP = BYQ;
            } else {
                let ACR = (-ACM).exp();
                let ACS = C + ACR;
                let ACT = ACL - (U * (ACS.ln()));
                let BYP = ((((BYO * BNP) * ACR) * (BHV / ACS)) * U) * BNP;
                ACV = ACT;
                BKP = BYP;
            }
            let ACW = ACU * ACV;
            let ACX = ACL - ACV;
            let ACY = ACX * ACX;
            let ACZ = ACW * ACY;
            let BYR = ((BKP * ACU) * ACY) + (((BKP * BNP) * (V * ACX)) * ACW);
            let ADA = RS / LE;
            let BYS = BSX / LE;
            let ADB = if ADA < RN { 1.0 } else { 0.0 };
            let ADT;
            let BKQ;
            if ADB != 0.0 {
                let ADC = ADA.exp();
                let BYU = BYS * ADC;
                ADT = ADC;
                BKQ = BYU;
            } else {
                let ADD = RN.exp();
                let ADE = ADD * (C + (ADA - RN));
                let BYT = BYS * ADD;
                ADT = ADE;
                BKQ = BYT;
            }
            let AVC;
            let BKR;
            if LR != 0.0 {
                let ADG = QV - ADF;
                let ADH = ADG * BP;
                let BZA = ((BWH - Lanes([BIR, 0.0, 0.0])) * BP) + Lanes([(BNU * ADG), 0.0, 0.0]);
                let ADI = if ADH < RN { 1.0 } else { 0.0 };
                let ADX;
                let BKS;
                if ADI != 0.0 {
                    let ADJ = ADH.exp();
                    let BZC = BZA * ADJ;
                    ADX = ADJ;
                    BKS = BZC;
                } else {
                    let ADK = RN.exp();
                    let ADL = ADK * (C + (ADH - RN));
                    let BZB = BZA * ADK;
                    ADX = ADL;
                    BKS = BZB;
                }
                let ADM = ABR / JZ;
                let BZD = (BYF - Lanes([(BQM * ADM), 0.0, 0.0, 0.0, 0.0])) / JZ;
                let ADN = ADM - 1e3f64;
                let ADP = if ADN < ADO { 1.0 } else { 0.0 };
                let AEG;
                let BKT;
                if ADP != 0.0 {
                    let ADQ = ADN.exp();
                    let BZF = BZD * ADQ;
                    AEG = ADQ;
                    BKT = BZF;
                } else {
                    let ADS = ADR * (C + (ADN - ADO));
                    let BZE = BZD * ADR;
                    AEG = ADS;
                    BKT = BZE;
                }
                let ADU = ADT - C;
                let BZG = Lanes([(BQU * ADU), 0.0, 0.0]) + (BKQ * LK);
                let ADW = ADV * V;
                let ADY = (C + (JP * ADX)).sqrt();
                let ADZ = C + ADY;
                let AEA = (ADW * ADU) / ADZ;
                let AEB = ZY / OS;
                let AEC = C + AEB;
                let BZH = (((Lanes([((BIZ * V) * ADU), 0.0, 0.0]) + (BKQ * ADW)) - (((BKS * JP) * (BHV / (BQJ * ADY))) * AEA)) / ADZ) * AEC;
                let BZI = ((BXG - Lanes([(BRL * AEB), 0.0, 0.0, 0.0])) / OS) * AEA;
                let AEE = AAF - C;
                let AEF = AED * AEE;
                let BZJ = (Lanes([(BJA * AEE), 0.0, 0.0, 0.0]) + (BJW * AED)) * AEG;
                let AEH = C + AEG;
                let AEI = (AEF * AEG) / AEH;
                let AEJ = ((LK * ADU) + (AEA * AEC)) + AEI;
                let BZK = (Lanes([BZG[0], BZG[1], BZG[2], 0.0, 0.0]) + (Lanes([BZH[0], BZH[1], BZH[2], 0.0, 0.0]) + Lanes([BZI[0], 0.0, BZI[1], BZI[2], BZI[3]]))) + (((Lanes([BZJ[0], 0.0, BZJ[1], BZJ[2], BZJ[3]]) + (BKT * AEF)) - (BKT * AEI)) / AEH);
                AVC = AEJ;
                BKR = BZK;
            } else {
                let AEL = if AEK == A { 1.0 } else { 0.0 };
                let AVD;
                let BKU;
                if AEL != 0.0 {
                    let AEM = ADT - C;
                    let AEN = LK * AEM;
                    let BYY = Lanes([(BQU * AEM), 0.0, 0.0]) + (BKQ * LK);
                    let BYZ = Lanes([BYY[0], BYY[1], BYY[2], 0.0, 0.0]);
                    AVD = AEN;
                    BKU = BYZ;
                } else {
                    let AEO = C - AEK;
                    let BYV = BKQ * AEO;
                    let AEP = AEK * ((ADT + AAF) - V);
                    let AEQ = ZY / OS;
                    let AER = C + AEQ;
                    let BYW = ((BXG - Lanes([(BRL * AEQ), 0.0, 0.0, 0.0])) / OS) * AEP;
                    let AES = (AEO * (ADT - C)) + (AEP * AER);
                    let AET = LK * AES;
                    let BYX = Lanes([(BQU * AES), 0.0, 0.0, 0.0, 0.0]) + ((Lanes([BYV[0], BYV[1], BYV[2], 0.0, 0.0]) + ((((Lanes([BKQ[0], BKQ[1], BKQ[2], 0.0, 0.0]) + Lanes([BJW[0], 0.0, BJW[1], BJW[2], BJW[3]])) * AEK) * AER) + Lanes([BYW[0], 0.0, BYW[1], BYW[2], BYW[3]]))) * LK);
                    AVD = AET;
                    BKU = BYX;
                }
                AVC = AVD;
                BKR = BKU;
            }
            let AEU = QX * BP;
            let BZL = BSB * BP;
            let BZM = Lanes([0.0, BZL[0], BZL[1]]) + Lanes([(BNU * QX), 0.0, 0.0]);
            let AEV = AEU / LM;
            let BZN = BZM / LM;
            let AEW = if AEV < RN { 1.0 } else { 0.0 };
            let AFG;
            let BKV;
            if AEW != 0.0 {
                let AEX = AEV.exp();
                let BZP = BZN * AEX;
                AFG = AEX;
                BKV = BZP;
            } else {
                let AEY = RN.exp();
                let AEZ = AEY * (C + (AEV - RN));
                let BZO = BZN * AEY;
                AFG = AEZ;
                BKV = BZO;
            }
            let AVH;
            let BKW;
            if LR != 0.0 {
                let AFA = QX - ADF;
                let AFB = AFA * BP;
                let BZR = ((Lanes([0.0, BSB[0], BSB[1]]) - Lanes([BIR, 0.0, 0.0])) * BP) + Lanes([(BNU * AFA), 0.0, 0.0]);
                let AFC = if AFB < RN { 1.0 } else { 0.0 };
                let AFK;
                let BKX;
                if AFC != 0.0 {
                    let AFD = AFB.exp();
                    let BZT = BZR * AFD;
                    AFK = AFD;
                    BKX = BZT;
                } else {
                    let AFE = RN.exp();
                    let AFF = AFE * (C + (AFB - RN));
                    let BZS = BZR * AFE;
                    AFK = AFF;
                    BKX = BZS;
                }
                let AFH = AFG - C;
                let AFJ = AFI * V;
                let AFL = (C + (JP * AFK)).sqrt();
                let AFM = C + AFL;
                let AFN = (AFJ * AFH) / AFM;
                let AFO = (LQ * AFH) + AFN;
                let BZU = (Lanes([(BQV * AFH), 0.0, 0.0]) + (BKV * LQ)) + (((Lanes([((BJB * V) * AFH), 0.0, 0.0]) + (BKV * AFJ)) - (((BKX * JP) * (BHV / (BQJ * AFL))) * AFN)) / AFM);
                AVH = AFO;
                BKW = BZU;
            } else {
                let AFP = AFG - C;
                let AFQ = LQ * AFP;
                let BZQ = Lanes([(BQV * AFP), 0.0, 0.0]) + (BKV * LQ);
                AVH = AFQ;
                BKW = BZQ;
            }
            let AFR = RS / KL;
            let BZV = BSX / KL;
            let AFS = if AFR < RN { 1.0 } else { 0.0 };
            let AFW;
            let BKY;
            if AFS != 0.0 {
                let AFT = AFR.exp();
                let BZX = BZV * AFT;
                AFW = AFT;
                BKY = BZX;
            } else {
                let AFU = RN.exp();
                let AFV = AFU * (C + (AFR - RN));
                let BZW = BZV * AFU;
                AFW = AFV;
                BKY = BZW;
            }
            let AFX = AFW - C;
            let AFY = KS * AFX;
            let BZY = Lanes([(BQQ * AFX), 0.0, 0.0]) + (BKY * KS);
            let AFZ = AEU / MM;
            let BZZ = BZM / MM;
            let AGA = if AFZ < RN { 1.0 } else { 0.0 };
            let AGE;
            let BKZ;
            if AGA != 0.0 {
                let AGB = AFZ.exp();
                let CAB = BZZ * AGB;
                AGE = AGB;
                BKZ = CAB;
            } else {
                let AGC = RN.exp();
                let AGD = AGC * (C + (AFZ - RN));
                let CAA = BZZ * AGC;
                AGE = AGD;
                BKZ = CAA;
            }
            let AGF = AGE - C;
            let AGG = MR * AGF;
            let CAC = Lanes([(BRA * AGF), 0.0, 0.0]) + (BKZ * MR);
            let AGH = RY / KU;
            let CAD = BTC / KU;
            let AGI = if AGH < RN { 1.0 } else { 0.0 };
            let AGM;
            let BLA;
            if AGI != 0.0 {
                let AGJ = AGH.exp();
                let CAF = CAD * AGJ;
                AGM = AGJ;
                BLA = CAF;
            } else {
                let AGK = RN.exp();
                let AGL = AGK * (C + (AGH - RN));
                let CAE = CAD * AGK;
                AGM = AGL;
                BLA = CAE;
            }
            let AGN = AGM - C;
            let AGO = LA * AGN;
            let CAG = Lanes([(BQR * AGN), 0.0, 0.0, 0.0, 0.0, 0.0]) + (BLA * LA);
            let AGP = AEU / MT;
            let CAH = BZM / MT;
            let AGQ = if AGP < RN { 1.0 } else { 0.0 };
            let AGU;
            let BLB;
            if AGQ != 0.0 {
                let AGR = AGP.exp();
                let CAJ = CAH * AGR;
                AGU = AGR;
                BLB = CAJ;
            } else {
                let AGS = RN.exp();
                let AGT = AGS * (C + (AGP - RN));
                let CAI = CAH * AGS;
                AGU = AGT;
                BLB = CAI;
            }
            let AGV = AGU - C;
            let AGW = MY * AGV;
            let CAK = Lanes([(BRB * AGV), 0.0, 0.0]) + (BLB * MY);
            let AGX = if (if (if NQ > A { 1.0 } else { 0.0 }) != 0.0 && (if NK > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && ABU != 0.0 { 1.0 } else { 0.0 };
            let AVE;
            let BLC;
            if AGX != 0.0 {
                let AGY = V * YJ;
                let AGZ = X / AGY;
                let AHA = C - AGZ;
                let AHB = NP * AHA;
                let CAM = Lanes([(BRF * AHA), 0.0, 0.0]) + ((((((BWO * V) * AGZ) * BNP) / AGY) * BNP) * NP);
                let AHC = if AHB < RN { 1.0 } else { 0.0 };
                let AIR;
                let BLD;
                if AHC != 0.0 {
                    let AHD = AHB.exp();
                    let CAO = CAM * AHD;
                    AIR = AHD;
                    BLD = CAO;
                } else {
                    let AHE = RN.exp();
                    let AHF = AHE * (C + (AHB - RN));
                    let CAN = CAM * AHE;
                    AIR = AHF;
                    BLD = CAN;
                }
                let AHG = QV * GE;
                let CAP = BSA * GE;
                let CAQ = Lanes([0.0, CAP[0], CAP[1]]) + Lanes([(BPF * QV), 0.0, 0.0]);
                let CAR = CAQ * AHG;
                let AHI = ((AHG * AHG) + AHH).sqrt();
                let AHJ = -2e0f64 - W;
                let AHK = AHI.powf(AHJ);
                let AHL = W - C;
                let AHM = KK * AHG;
                let AHN = AHM * AHG;
                let AHO = AHL + AHG;
                let AHP = (W * ((C - (W * W)) - ((CT * AHG) * AHL))) - (AHN * AHO);
                let AHR = (AHK * AHP) * AHQ;
                let AHS = QV * X;
                let CAS = (BSA * X) * NP;
                let AHT = NF * AHR;
                let AHU = (AHS * NP) / AHT;
                let CAT = ((Lanes([0.0, CAS[0], CAS[1]]) + Lanes([(BRF * AHS), 0.0, 0.0])) - ((Lanes([(BIK * AHR), 0.0, 0.0]) + (((((((CAR + CAR) * (BHV / (BQJ * AHI))) * (AHJ * (AHI.powf((AHJ - BHV))))) * AHP) + ((((((CAQ * CT) * AHL) * BNP) * W) - (((((CAQ * KK) * AHG) + (CAQ * AHM)) * AHO) + (CAQ * AHN))) * AHK)) * AHQ) * NF)) * AHU)) / AHT;
                let AHV = if AHU < -1e-3f64 { 1.0 } else { 0.0 };
                let AIO;
                let BLE;
                if AHV != 0.0 {
                    let AHW = if AHU < RN { 1.0 } else { 0.0 };
                    let AIB;
                    let BLF;
                    if AHW != 0.0 {
                        let AHX = AHU.exp();
                        let CAX = CAT * AHX;
                        AIB = AHX;
                        BLF = CAX;
                    } else {
                        let AHY = RN.exp();
                        let AHZ = AHY * (C + (AHU - RN));
                        let CAW = CAT * AHY;
                        AIB = AHZ;
                        BLF = CAW;
                    }
                    let AIA = -QV;
                    let AIC = (C - AIB) / AHU;
                    let AID = C + AIC;
                    let AIE = AIA * AID;
                    let CAY = (BSA * BNP) * AID;
                    let CAZ = Lanes([0.0, CAY[0], CAY[1]]) + ((((BLF * BNP) - (CAT * AIC)) / AHU) * AIA);
                    AIO = AIE;
                    BLE = CAZ;
                } else {
                    let AIF = QV * JI;
                    let AIG = AIF * AHU;
                    let CAU = (BSA * JI) * AHU;
                    let AII = AHU * AIH;
                    let AIK = C + (AIJ * AHU);
                    let AIL = C + (AII * AIK);
                    let AIM = AIG * AIL;
                    let CAV = ((Lanes([0.0, CAU[0], CAU[1]]) + (CAT * AIF)) * AIL) + ((((CAT * AIH) * AIK) + ((CAT * AIJ) * AII)) * AIG);
                    AIO = AIM;
                    BLE = CAV;
                }
                let AIN = V * NW;
                let AIP = AIN * AIO;
                let AIQ = AIP * YJ;
                let AIS = AIQ * AIR;
                let AIT = (AIS * GE) * Y;
                let CBA = (((((((Lanes([((BRG * V) * AIO), 0.0, 0.0]) + (BLE * AIN)) * YJ) + (BWO * AIP)) * AIR) + (BLD * AIQ)) * GE) + Lanes([(BPF * AIS), 0.0, 0.0])) * Y;
                AVE = AIT;
                BLC = CBA;
            } else {
                AVE = A;
                BLC = CAL;
            }
            let AIU = if (if (if OI > A { 1.0 } else { 0.0 }) != 0.0 && (if OC > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if QR < A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let ANK;
            let BLG;
            if AIU != 0.0 {
                let AIV = QR * GG;
                let CBB = BRY * GG;
                let CBC = Lanes([0.0, CBB[0], CBB[1]]) + Lanes([(BPG * QR), 0.0, 0.0]);
                let AIW = C - AIV;
                let AIX = AIW.powf(ZO);
                let CBD = (CBC * BNP) * (ZO * (AIW.powf(BXD)));
                let AIY = V * AIX;
                let AIZ = AO / AIY;
                let AJA = C - AIZ;
                let AJB = OH * AJA;
                let CBE = Lanes([(BRI * AJA), 0.0, 0.0]) + ((((((CBD * V) * AIZ) * BNP) / AIY) * BNP) * OH);
                let AJC = if AJB < RN { 1.0 } else { 0.0 };
                let AKM;
                let BLH;
                if AJC != 0.0 {
                    let AJD = AJB.exp();
                    let CBG = CBE * AJD;
                    AKM = AJD;
                    BLH = CBG;
                } else {
                    let AJE = RN.exp();
                    let AJF = AJE * (C + (AJB - RN));
                    let CBF = CBE * AJE;
                    AKM = AJF;
                    BLH = CBF;
                }
                let CBH = CBC * AIV;
                let AJG = ((AIV * AIV) + AHH).sqrt();
                let AJH = -2e0f64 - AN;
                let AJI = AJG.powf(AJH);
                let AJJ = AN - C;
                let AJK = KK * AIV;
                let AJL = AJK * AIV;
                let AJM = AJJ + AIV;
                let AJN = (AN * ((C - (AN * AN)) - ((CT * AIV) * AJJ))) - (AJL * AJM);
                let AJO = (AJI * AJN) * AHQ;
                let AJP = QR * AO;
                let CBI = (BRY * AO) * OH;
                let AJQ = NX * AJO;
                let AJR = (AJP * OH) / AJQ;
                let CBJ = ((Lanes([0.0, CBI[0], CBI[1]]) + Lanes([(BRI * AJP), 0.0, 0.0])) - ((Lanes([(BIL * AJO), 0.0, 0.0]) + (((((((CBH + CBH) * (BHV / (BQJ * AJG))) * (AJH * (AJG.powf((AJH - BHV))))) * AJN) + ((((((CBC * CT) * AJJ) * BNP) * AN) - (((((CBC * KK) * AIV) + (CBC * AJK)) * AJM) + (CBC * AJL))) * AJI)) * AHQ) * NX)) * AJR)) / AJQ;
                let AJS = if AJR < -1e-3f64 { 1.0 } else { 0.0 };
                let AKJ;
                let BLI;
                if AJS != 0.0 {
                    let AJT = if AJR < RN { 1.0 } else { 0.0 };
                    let AJY;
                    let BLJ;
                    if AJT != 0.0 {
                        let AJU = AJR.exp();
                        let CBN = CBJ * AJU;
                        AJY = AJU;
                        BLJ = CBN;
                    } else {
                        let AJV = RN.exp();
                        let AJW = AJV * (C + (AJR - RN));
                        let CBM = CBJ * AJV;
                        AJY = AJW;
                        BLJ = CBM;
                    }
                    let AJX = -QR;
                    let AJZ = (C - AJY) / AJR;
                    let AKA = C + AJZ;
                    let AKB = AJX * AKA;
                    let CBO = (BRY * BNP) * AKA;
                    let CBP = Lanes([0.0, CBO[0], CBO[1]]) + ((((BLJ * BNP) - (CBJ * AJZ)) / AJR) * AJX);
                    AKJ = AKB;
                    BLI = CBP;
                } else {
                    let AKC = QR * JI;
                    let AKD = AKC * AJR;
                    let CBK = (BRY * JI) * AJR;
                    let AKE = AJR * AIH;
                    let AKF = C + (AIJ * AJR);
                    let AKG = C + (AKE * AKF);
                    let AKH = AKD * AKG;
                    let CBL = ((Lanes([0.0, CBK[0], CBK[1]]) + (CBJ * AKC)) * AKG) + ((((CBJ * AIH) * AKF) + ((CBJ * AIJ) * AKE)) * AKD);
                    AKJ = AKH;
                    BLI = CBL;
                }
                let AKI = V * OO;
                let AKK = AKI * AKJ;
                let AKL = AKK * AIX;
                let AKN = AKL * AKM;
                let AKO = (AKN * GG) * AP;
                let CBQ = (((((((Lanes([((BRJ * V) * AKJ), 0.0, 0.0]) + (BLI * AKI)) * AIX) + (CBD * AKK)) * AKM) + (BLH * AKL)) * GG) + Lanes([(BPG * AKN), 0.0, 0.0])) * AP;
                ANK = AKO;
                BLG = CBQ;
            } else {
                ANK = A;
                BLG = BUG;
            }
            let AKQ = ZZ * AKP;
            let CBR = Lanes([(BXH * AKP), 0.0, 0.0, 0.0, 0.0, 0.0]) + (BJL * ZZ);
            let AKS = JP * AKR;
            let CBS = BJP * JP;
            let AKT = (C + AKQ).sqrt();
            let AKU = C + AKT;
            let AKV = (AKQ - ZZ) / AKU;
            let CBT = ((CBR - Lanes([BXH, 0.0, 0.0, 0.0, 0.0, 0.0])) - ((CBR * (BHV / (BQJ * AKT))) * AKV)) / AKU;
            let AKW = (C + AKS).sqrt();
            let AKX = C + AKW;
            let AKY = AKS / AKX;
            let CBU = (CBS - ((CBS * (BHV / (BQJ * AKW))) * AKY)) / AKX;
            let AKZ = V * MK;
            let CBV = BQZ * V;
            let ALA = AKP - C;
            let ALB = (JP * MK) / KI;
            let CBW = ((BQZ * JP) - (BQO * ALB)) / KI;
            let ALC = (C + (ALB * AKP)).sqrt();
            let ALD = C + ALC;
            let ALE = (AKZ * ALA) / ALD;
            let CBX = ((Lanes([(CBV * ALA), 0.0, 0.0, 0.0, 0.0, 0.0]) + (BJL * AKZ)) - (((Lanes([(CBW * AKP), 0.0, 0.0, 0.0, 0.0, 0.0]) + (BJL * ALB)) * (BHV / (BQJ * ALC))) * ALE)) / ALD;
            let ALG = if I > A { 1.0 } else { 0.0 };
            let ALH = if (if ALF > A { 1.0 } else { 0.0 }) != 0.0 && ALG != 0.0 { 1.0 } else { 0.0 };
            let ANO;
            let ANR;
            let BAQ;
            let BLK;
            let BLL;
            let BLM;
            if ALH != 0.0 {
                let ALI = ALE * J;
                let CBZ = CBX * J;
                let ALJ = I * V;
                let ALK = ALJ * MK;
                let ALM = ALL - C;
                let ALN = (C + (ALB * ALL)).sqrt();
                let ALO = C + ALN;
                let ALP = (ALK * ALM) / ALO;
                let CCA = ((Lanes([0.0, 0.0, ((BQZ * ALJ) * ALM), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]) + (BJN * ALK)) - (((Lanes([0.0, 0.0, (CBW * ALL), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]) + (BJN * ALB)) * (BHV / (BQJ * ALN))) * ALP)) / ALO;
                let ALQ = if ALF == C { 1.0 } else { 0.0 };
                let AMH;
                let BLN;
                if ALQ != 0.0 {
                    let ALR = I * MK;
                    let ALS = ALR * HP;
                    let CCB = ((BQZ * I) * HP) + (BPT * ALR);
                    let ALT = ALS * BP;
                    let ALU = V - (ALT.ln());
                    let ALV = RL - (BO * ALU);
                    let CCC = BTN - Lanes([0.0, 0.0, ((BNT * ALU) + (((((CCB * BP) + (BNU * ALS)) * (BHV / ALT)) * BNP) * BO)), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
                    let ALX = ALV * ALV;
                    let CCD = CCC * ALV;
                    let CCE = CCD + CCD;
                    let ALY = if ALV < A { 1.0 } else { 0.0 };
                    let AME;
                    let BLO;
                    if ALY != 0.0 {
                        let ALZ = (ALX + ALW).sqrt();
                        let AMA = ALZ - ALV;
                        let AMB = 6.05e-3f64 / AMA;
                        let CCG = ((((CCE * (BHV / (BQJ * ALZ))) - CCC) * AMB) * BNP) / AMA;
                        AME = AMB;
                        BLO = CCG;
                    } else {
                        let AMC = (ALX + ALW).sqrt();
                        let AMD = JI * (AMC + ALV);
                        let CCF = ((CCE * (BHV / (BQJ * AMC))) + CCC) * JI;
                        AME = AMD;
                        BLO = CCF;
                    }
                    let AMF = (ALS + (ALP * HP)) + AME;
                    let AMG = AME / AMF;
                    let CCH = (BLO - (((Lanes([0.0, 0.0, CCB, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]) + ((CCA * HP) + Lanes([0.0, 0.0, (BPT * ALP), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]))) + BLO) * AMG)) / AMF;
                    AMH = AMG;
                    BLN = CCH;
                } else {
                    AMH = C;
                    BLN = CBY;
                }
                let AMI = AMH * ALP;
                let CCI = (BLN * ALP) + (CCA * AMH);
                ANO = ALI;
                ANR = AMI;
                BAQ = AMH;
                BLK = CBZ;
                BLL = CCI;
                BLM = BLN;
            } else {
                ANO = ALE;
                ANR = A;
                BAQ = C;
                BLK = CBX;
                BLL = CBY;
                BLM = CBY;
            }
            let AMJ = if parameters[83] == C { 1.0 } else { 0.0 };
            let ANL;
            let BLP;
            if AMJ != 0.0 {
                let AMK = QY + QR;
                let CCK = Lanes([BSC[0], BSC[1], 0.0]) + Lanes([0.0, BRY[0], BRY[1]]);
                let AMP = (AMN * AMK) * AMO;
                let AMQ = AMP * AMK;
                let CCL = (((CCK * AMN) * AMO) * AMK) + (CCK * AMP);
                let AMR = if (-1e0f64 * AMK) < A { 1.0 } else { 0.0 };
                let ANE;
                let BLQ;
                if AMR != 0.0 {
                    let AMS = (AMQ + AMM).sqrt();
                    let AMU = AMS - (AMT * AMK);
                    let AMV = 5e-13f64 / AMU;
                    let CCN = ((((CCL * (BHV / (BQJ * AMS))) - (CCK * AMT)) * AMV) * BNP) / AMU;
                    ANE = AMV;
                    BLQ = CCN;
                } else {
                    let AMW = (AMQ + AMM).sqrt();
                    let AMY = JI * (AMW + (AMX * AMK));
                    let CCM = ((CCL * (BHV / (BQJ * AMW))) + (CCK * AMX)) * JI;
                    ANE = AMY;
                    BLQ = CCM;
                }
                let ANA = C / (C - (BA.powf(AMZ)));
                let ANC = BA * ANB;
                let AND = (((ANA * ANA) * (BA.powf((AMZ - C)))) * AMZ) / ANB;
                let ANF = if ANE < ANC { 1.0 } else { 0.0 };
                let ANM;
                let BLR;
                if ANF != 0.0 {
                    let ANG = ANE / ANB;
                    let ANH = C - (ANG.powf(AMZ));
                    let ANI = C / ANH;
                    let CCP = (((((BLQ / ANB) * (AMZ * (ANG.powf((AMZ - BHV))))) * BNP) * ANI) * BNP) / ANH;
                    ANM = ANI;
                    BLR = CCP;
                } else {
                    let CCO = BLQ * AND;
                    let ANJ = ANA + ((ANE - ANC) * AND);
                    ANM = ANJ;
                    BLR = CCO;
                }
                ANL = ANM;
                BLP = BLR;
            } else {
                ANL = C;
                BLP = CCJ;
            }
            let ANN = ANK * ANL;
            let CCQ = BLG * ANL;
            let CCR = BLP * ANK;
            let CCS = Lanes([CCQ[0], 0.0, CCQ[1], CCQ[2]]) + Lanes([0.0, CCR[0], CCR[1], CCR[2]]);
            let ANP = ANO * ANL;
            let CCT = BLP * ANO;
            let CCU = (BLK * ANL) + Lanes([0.0, CCT[0], CCT[1], CCT[2], 0.0, 0.0]);
            let ANQ = AGO * ANL;
            let CCV = BLP * AGO;
            let CCW = (CAG * ANL) + Lanes([0.0, CCV[0], CCV[1], CCV[2], 0.0, 0.0]);
            let ANS = ANR * ANL;
            let CCX = BLP * ANR;
            let CCY = (BLL * ANL) + Lanes([0.0, 0.0, 0.0, CCX[0], CCX[1], CCX[2], 0.0, 0.0, 0.0]);
            let ANT = YM / OV;
            let CCZ = (BWQ - Lanes([(BRM * ANT), 0.0, 0.0])) / OV;
            let ANU = ZY / OS;
            let CDA = (BXG - Lanes([(BRL * ANU), 0.0, 0.0, 0.0])) / OS;
            let ANV = (C + ANT) + ANU;
            let CDB = Lanes([CCZ[0], CCZ[1], CCZ[2], 0.0, 0.0]) + Lanes([CDA[0], 0.0, CDA[1], CDA[2], CDA[3]]);
            let ANX = ANV * ANV;
            let CDC = CDB * ANV;
            let CDD = CDC + CDC;
            let ANY = if ANV < A { 1.0 } else { 0.0 };
            let AOE;
            let BLS;
            if ANY != 0.0 {
                let ANZ = (ANX + ANW).sqrt();
                let AOA = ANZ - ANV;
                let AOB = 5.000000000000001e-3f64 / AOA;
                let CDF = ((((CDD * (BHV / (BQJ * ANZ))) - CDB) * AOB) * BNP) / AOA;
                AOE = AOB;
                BLS = CDF;
            } else {
                let AOC = (ANX + ANW).sqrt();
                let AOD = JI * (AOC + ANV);
                let CDE = ((CDD * (BHV / (BQJ * AOC))) + CDB) * JI;
                AOE = AOD;
                BLS = CDE;
            }
            let AOF = AOE * ABL;
            let AOG = HG / AOF;
            let CDG = (Lanes([BPR, 0.0, 0.0, 0.0, 0.0]) - (((BLS * ABL) + (BXZ * AOE)) * AOG)) / AOF;
            let AOH = if AOG < S { 1.0 } else { 0.0 };
            let AOI;
            let BLT;
            if AOH != 0.0 {
                AOI = S;
                BLT = CDH;
            } else {
                AOI = AOG;
                BLT = CDG;
            }
            let AOJ = CT * AOI;
            let CDI = BLT * CT;
            let AOK = V * BO;
            let AOM = AOL - C;
            let CDJ = (Lanes([((BNT * V) * AOM), 0.0, 0.0]) + (BJM * AOK)) + Lanes([0.0, BSC[0], BSC[1]]);
            let AON = ((AOK * AOM) + QY) / AOJ;
            let CDK = CDI * AON;
            let CDL = (Lanes([CDJ[0], 0.0, CDJ[1], CDJ[2], 0.0, 0.0]) - Lanes([CDK[0], CDK[1], 0.0, CDK[2], CDK[3], CDK[4]])) / AOJ;
            let AOO = if ABR > A { 1.0 } else { 0.0 };
            let AUN;
            let BLU;
            if AOO != 0.0 {
                let AOQ = if AOP == C { 1.0 } else { 0.0 };
                let ATC;
                let BLV;
                if AOQ != 0.0 {
                    let AOS = if QR < AOR { 1.0 } else { 0.0 };
                    let ATD;
                    let BLW;
                    if AOS != 0.0 {
                        let AOU = (-ABR) / AOT;
                        let CFA = (BYF * BNP) / AOT;
                        let AOV = if AOU < RN { 1.0 } else { 0.0 };
                        let APA;
                        let BLX;
                        if AOV != 0.0 {
                            let AOW = AOU.exp();
                            let CFC = CFA * AOW;
                            APA = AOW;
                            BLX = CFC;
                        } else {
                            let AOX = RN.exp();
                            let AOY = AOX * (C + (AOU - RN));
                            let CFB = CFA * AOX;
                            APA = AOY;
                            BLX = CFB;
                        }
                        let AOZ = AOR - QR;
                        let APB = AOZ * APA;
                        let CFD = (BRY * BNP) * APA;
                        let CFE = Lanes([0.0, 0.0, CFD[0], CFD[1], 0.0]) + (BLX * AOZ);
                        let APD = -APC;
                        let APF = APB.powf(APE);
                        let APG = APD * APF;
                        let CFF = Lanes([((BIY * BNP) * APF), 0.0, 0.0, 0.0, 0.0]) + ((CFE * (APE * (APB.powf((APE - BHV))))) * APD);
                        let APH = if APG < RN { 1.0 } else { 0.0 };
                        let APO;
                        let BLY;
                        if APH != 0.0 {
                            let API = APG.exp();
                            let CFH = CFF * API;
                            APO = API;
                            BLY = CFH;
                        } else {
                            let APJ = RN.exp();
                            let APK = APJ * (C + (APG - RN));
                            let CFG = CFF * APJ;
                            APO = APK;
                            BLY = CFG;
                        }
                        let APM = APL / APC;
                        let APN = APM * APB;
                        let APP = APN * APO;
                        let CFI = ((Lanes([((((BIY * APM) * BNP) / APC) * APB), 0.0, 0.0, 0.0, 0.0]) + (CFE * APM)) * APO) + (BLY * APN);
                        ATD = APP;
                        BLW = CFI;
                    } else {
                        ATD = A;
                        BLW = CDH;
                    }
                    ATC = ATD;
                    BLV = BLW;
                } else {
                    let APQ = if AOP == V { 1.0 } else { 0.0 };
                    let ATE;
                    let BLZ;
                    if APQ != 0.0 {
                        let APR = if QR < SN { 1.0 } else { 0.0 };
                        let ATF;
                        let BMA;
                        if APR != 0.0 {
                            let APT = (V * parameters[45]) / (APS * APS);
                            let APU = SN - QR;
                            let CEA = BTZ - BTY;
                            let APV = APU / ZL;
                            let CEB = Lanes([CEA[0], CEA[1], CEA[2], 0.0]);
                            let APW = ((V * APV) / APT).sqrt();
                            let CEC = ((((CEB - (BJV * APV)) / ZL) * V) / APT) * (BHV / (BQJ * APW));
                            let APY = if APX == A { 1.0 } else { 0.0 };
                            let AQE;
                            let BMB;
                            if APY != 0.0 {
                                AQE = APS;
                                BMB = BUU;
                            } else {
                                let AQB = C - (JI * APZ);
                                let CED = (BJX * JI) * BNP;
                                let AQC = APS * AQB;
                                let AQD = AQC * AQB;
                                let CEE = ((CED * APS) * AQB) + (CED * AQC);
                                AQE = AQD;
                                BMB = CEE;
                            }
                            let CEF = CEC * APW;
                            let CEG = BMB * AQE;
                            let AQF = ((APW * APW) + (AQE * AQE)).sqrt();
                            let AQG = (APW * AQE) / AQF;
                            let CEH = (((CEC * AQE) + (BMB * APW)) - ((((CEF + CEF) + (CEG + CEG)) * (BHV / (BQJ * AQF))) * AQG)) / AQF;
                            let AQH = APU / AQG;
                            let CEI = (CEB - (CEH * AQH)) / AQG;
                            let AQI = JI * AQG;
                            let CEJ = CEH * JI;
                            let AQJ = AQI * APT;
                            let CEK = CEJ * APT;
                            let AQK = AQH + (AQJ * ZL);
                            let CEL = CEI + ((CEK * ZL) + (BJV * AQJ));
                            let AQX;
                            let BMC;
                            if APY != 0.0 {
                                let CET = Lanes([CEL[0], 0.0, CEL[1], CEL[2], CEL[3]]);
                                AQX = AQK;
                                BMC = CET;
                            } else {
                                let AQM = V * AQL;
                                let AQN = UW * (C + (AQM * (C + (V * APZ))));
                                let AQO = ABR / AQN;
                                let CEM = (((BJX * V) * AQM) * UW) * AQO;
                                let AQP = ((C + AQL) / (C + AQM)) - AQO;
                                let CEN = CEK * AQP;
                                let AQQ = AQH - (AQJ * AQP);
                                let CEO = Lanes([CEI[0], 0.0, CEI[1], CEI[2], CEI[3]]) - (Lanes([CEN[0], 0.0, CEN[1], CEN[2], CEN[3]]) + ((((BYF - Lanes([CEM[0], 0.0, CEM[1], CEM[2], CEM[3]])) / AQN) * BNP) * AQJ));
                                let AQR = AQQ - AQK;
                                let CEP = Lanes([CEL[0], 0.0, CEL[1], CEL[2], CEL[3]]);
                                let CEQ = (CEO - CEP) * AQR;
                                let AQS = AE * AQH;
                                let AQT = AQS * AQH;
                                let CER = (((((CEI * AE) * AQH) + (CEI * AQS)) * AQU) + (BJY * AQT)) / UW;
                                let AQV = ((AQR * AQR) + ((AQT * AQU) / UW)).sqrt();
                                let AQW = JI * ((AQQ + AQK) + AQV);
                                let CES = ((CEO + CEP) + (((CEQ + CEQ) + Lanes([CER[0], 0.0, CER[1], CER[2], CER[3]])) * (BHV / (BQJ * AQV)))) * JI;
                                AQX = AQW;
                                BMC = CES;
                            }
                            let AQY = (AQX - AQH) / AQX;
                            let CEU = ((BMC - Lanes([CEI[0], 0.0, CEI[1], CEI[2], CEI[3]])) - (BMC * AQY)) / AQX;
                            let AQZ = if (AQY.abs()) > 1e-7f64 { 1.0 } else { 0.0 };
                            let ATG;
                            let BMD;
                            if AQZ != 0.0 {
                                let ARA = AQI / AQY;
                                let CEX = (Lanes([CEJ[0], 0.0, CEJ[1], CEJ[2], CEJ[3]]) - (CEU * ARA)) / AQY;
                                let ARD = ARB / ARC;
                                let ARE = ARD * AQX;
                                let ARF = ARE * ARA;
                                let ARG = (-ARC) / AQX;
                                let CEY = (Lanes([(BJC * BNP), 0.0, 0.0, 0.0, 0.0]) - (BMC * ARG)) / AQX;
                                let ARH = ARG.exp();
                                let ARI = AQE / ARA;
                                let ARJ = C + ARI;
                                let ARK = (ARG * ARJ).exp();
                                let ARL = ARH - ARK;
                                let ARM = ARF * ARL;
                                let CEZ = ((((Lanes([((((BJC * ARD) * BNP) / ARC) * AQX), 0.0, 0.0, 0.0, 0.0]) + (BMC * ARD)) * ARA) + (CEX * ARE)) * ARL) + (((CEY * ARH) - (((CEY * ARJ) + (((Lanes([BMB[0], 0.0, BMB[1], BMB[2], BMB[3]]) - (CEX * ARI)) / ARA) * ARG)) * ARK)) * ARF);
                                ATG = ARM;
                                BMD = CEZ;
                            } else {
                                let ARN = ARB * AQE;
                                let ARO = (-ARC) / AQX;
                                let ARP = ARO.exp();
                                let ARQ = ARN * ARP;
                                let CEV = (BMB * ARB) * ARP;
                                let CEW = Lanes([CEV[0], 0.0, CEV[1], CEV[2], CEV[3]]) + ((((Lanes([(BJC * BNP), 0.0, 0.0, 0.0, 0.0]) - (BMC * ARO)) / AQX) * ARP) * ARN);
                                ATG = ARQ;
                                BMD = CEW;
                            }
                            ATF = ATG;
                            BMA = BMD;
                        } else {
                            ATF = A;
                            BMA = CDH;
                        }
                        ATE = ATF;
                        BLZ = BMA;
                    } else {
                        let ARR = if AOP == CT { 1.0 } else { 0.0 };
                        let ATH;
                        let BME;
                        if ARR != 0.0 {
                            let ARS = if QR < AOR { 1.0 } else { 0.0 };
                            let ATI;
                            let BMF;
                            if ARS != 0.0 {
                                let ART = AOR - QR;
                                let CDM = BRY * BNP;
                                let ARU = ART.powf(APE);
                                let ARW = ARV + ABR;
                                let ARX = ABR / ARW;
                                let ARY = C - ARX;
                                let ASA = ARY.powf(ARZ);
                                let ASB = ARU * ASA;
                                let CDN = (CDM * (APE * (ART.powf((APE - BHV))))) * ASA;
                                let CDO = Lanes([0.0, 0.0, CDN[0], CDN[1], 0.0]) + (((((BYF - (BYF * ARX)) / ARW) * BNP) * (ARZ * (ARY.powf((ARZ - BHV))))) * ARU);
                                let ASC = if APX == A { 1.0 } else { 0.0 };
                                let ASS;
                                let BMG;
                                if ASC != 0.0 {
                                    ASS = ASB;
                                    BMG = CDO;
                                } else {
                                    let ASD = (ABR - parameters[51]) / ARV;
                                    let CDP = BYF / ARV;
                                    let ASF = (ASD - C) / ASE;
                                    let CDQ = CDP / ASE;
                                    let ASG = if ASD < C { 1.0 } else { 0.0 };
                                    let ASN;
                                    let BMH;
                                    if ASG != 0.0 {
                                        let ASH = ASF.exp();
                                        let ASI = C + ASH;
                                        let CDS = ((CDQ * ASH) * (BHV / ASI)) * ASE;
                                        let ASJ = C + (ASE * (ASI.ln()));
                                        ASN = ASJ;
                                        BMH = CDS;
                                    } else {
                                        let ASK = (-ASF).exp();
                                        let ASL = C + ASK;
                                        let ASM = ASD + (ASE * (ASL.ln()));
                                        let CDR = CDP + ((((CDQ * BNP) * ASK) * (BHV / ASL)) * ASE);
                                        ASN = ASM;
                                        BMH = CDR;
                                    }
                                    let ASP = ASN.powf(ASO);
                                    let ASQ = ASB * ASP;
                                    let CDT = (CDO * ASP) + ((BMH * (ASO * (ASN.powf((ASO - BHV))))) * ASB);
                                    ASS = ASQ;
                                    BMG = CDT;
                                }
                                let ASR = -APC;
                                let AST = ASR * ASS;
                                let CDU = Lanes([((BIY * BNP) * ASS), 0.0, 0.0, 0.0, 0.0]) + (BMG * ASR);
                                let ASU = if AST < RN { 1.0 } else { 0.0 };
                                let ATA;
                                let BMI;
                                if ASU != 0.0 {
                                    let ASV = AST.exp();
                                    let CDW = CDU * ASV;
                                    ATA = ASV;
                                    BMI = CDW;
                                } else {
                                    let ASW = RN.exp();
                                    let ASX = ASW * (C + (AST - RN));
                                    let CDV = CDU * ASW;
                                    ATA = ASX;
                                    BMI = CDV;
                                }
                                let ASY = APL / APC;
                                let ASZ = ASY * ART;
                                let CDX = CDM * ASY;
                                let ATB = ASZ * ATA;
                                let CDY = (Lanes([((((BIY * ASY) * BNP) / APC) * ART), 0.0, 0.0]) + Lanes([0.0, CDX[0], CDX[1]])) * ATA;
                                let CDZ = Lanes([CDY[0], 0.0, CDY[1], CDY[2], 0.0]) + (BMI * ASZ);
                                ATI = ATB;
                                BMF = CDZ;
                            } else {
                                ATI = A;
                                BMF = CDH;
                            }
                            ATH = ATI;
                            BME = BMF;
                        } else {
                            ATH = A;
                            BME = CDH;
                        }
                        ATE = ATH;
                        BLZ = BME;
                    }
                    ATC = ATE;
                    BLV = BLZ;
                }
                let ATJ = if ATC > A { 1.0 } else { 0.0 };
                let AUO;
                let BMJ;
                if ATJ != 0.0 {
                    let ATK = if parameters[52] == C { 1.0 } else { 0.0 };
                    let AUP;
                    let BMK;
                    if ATK != 0.0 {
                        let ATM = ATL + AOJ;
                        let CFK = Lanes([BIT, 0.0, 0.0, 0.0, 0.0]) + CDI;
                        let ATN = ABR * ATM;
                        let ATO = BO / ATN;
                        let ATP = ABM / JZ;
                        let ATR = ATQ / ATM;
                        let ATS = (ATO + (ATP * LK)) + ATR;
                        let CFL = (((Lanes([BNT, 0.0, 0.0, 0.0, 0.0]) - (((BYF * ATM) + (CFK * ABR)) * ATO)) / ATN) + ((((BYA - Lanes([(BQM * ATP), 0.0, 0.0, 0.0, 0.0])) / JZ) * LK) + Lanes([(BQU * ATP), 0.0, 0.0, 0.0, 0.0]))) + ((Lanes([BIS, 0.0, 0.0, 0.0, 0.0]) - (CFK * ATR)) / ATM);
                        let ATT = if AOP == CT { 1.0 } else { 0.0 };
                        let AUQ;
                        let BML;
                        if ATT != 0.0 {
                            let ATU = (ATC - ATS) / AML;
                            let CFN = (BLV - CFL) / AML;
                            let ATV = if ATC < ATS { 1.0 } else { 0.0 };
                            let AUC;
                            let BMM;
                            if ATV != 0.0 {
                                let ATW = ATU.exp();
                                let ATX = C + ATW;
                                let ATY = ATC - (AML * (ATX.ln()));
                                let CFP = BLV - (((CFN * ATW) * (BHV / ATX)) * AML);
                                AUC = ATY;
                                BMM = CFP;
                            } else {
                                let ATZ = (-ATU).exp();
                                let AUA = C + ATZ;
                                let AUB = ATS - (AML * (AUA.ln()));
                                let CFO = CFL - ((((CFN * BNP) * ATZ) * (BHV / AUA)) * AML);
                                AUC = AUB;
                                BMM = CFO;
                            }
                            let AUD = ABR * AUC;
                            let CFQ = (BYF * AUC) + (BMM * ABR);
                            AUQ = AUD;
                            BML = CFQ;
                        } else {
                            let AUE = ABR * ATC;
                            let AUF = ATC + ATS;
                            let AUG = (AUE * ATS) / AUF;
                            let CFM = (((((BYF * ATC) + (BLV * ABR)) * ATS) + (CFL * AUE)) - ((BLV + CFL) * AUG)) / AUF;
                            AUQ = AUG;
                            BML = CFM;
                        }
                        AUP = AUQ;
                        BMK = BML;
                    } else {
                        let AUH = ABR * ATC;
                        let CFJ = (BYF * ATC) + (BLV * ABR);
                        AUP = AUH;
                        BMK = CFJ;
                    }
                    AUO = AUP;
                    BMJ = BMK;
                } else {
                    AUO = A;
                    BMJ = CDH;
                }
                AUN = AUO;
                BLU = BMJ;
            } else {
                AUN = A;
                BLU = CDH;
            }
            let AUI = if AAF > A { 1.0 } else { 0.0 };
            let AUK;
            let BMN;
            if AUI != 0.0 {
                let AUJ = BO * BXL;
                let CFS = Lanes([(BNT * BXL), 0.0, 0.0, 0.0]) + ((BJW * (BHV / AAF)) * BO);
                AUK = AUJ;
                BMN = CFS;
            } else {
                let CFR = Lanes([0.0, BRZ[0], 0.0, BRZ[1]]);
                AUK = QT;
                BMN = CFR;
            }
            let AVG;
            let BMO;
            if LR != 0.0 {
                let CFU = Lanes([BRY[0], BRY[1], 0.0]);
                AVG = QR;
                BMO = CFU;
            } else {
                let CFT = Lanes([BRZ[0], 0.0, BRZ[1]]);
                AVG = QT;
                BMO = CFT;
            }
            let AUL = QV - AUK;
            let AUM = AUK - QR;
            let CFV = (BUL * AUM) + ((BMN - Lanes([0.0, BRY[0], BRY[1], 0.0])) * TY);
            let CFW = BMN * AUN;
            let CFX = (((BYF * AUL) + ((Lanes([0.0, BSA[0], BSA[1], 0.0, 0.0]) - Lanes([BMN[0], 0.0, BMN[1], BMN[2], BMN[3]])) * ABR)) + Lanes([CFV[0], 0.0, CFV[1], CFV[2], CFV[3]])) - ((BLU * AUK) + Lanes([CFW[0], 0.0, CFW[1], CFW[2], CFW[3]]));
            let CFY = BSE * RB;
            let CFZ = CFY + CFY;
            let AUR = (RB * RB) / ATQ;
            let CGA = (Lanes([CFZ[0], 0.0, CFZ[1]]) - Lanes([0.0, (BIS * AUR), 0.0])) / ATQ;
            let CGB = Lanes([0.0, CFX[0], CFX[1], CFX[2], CFX[3], CFX[4]]) + Lanes([CGA[0], CGA[1], CGA[2], 0.0, 0.0, 0.0]);
            let AUS = RK * RK;
            let CGC = BSQ * RK;
            let CGD = (CGC + CGC) * AUT;
            let CGE = Lanes([CGD[0], CGD[1], 0.0, CGD[2], CGD[3], CGD[4], CGD[5], CGD[6], CGD[7]]) + Lanes([0.0, 0.0, (BJD * AUS), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
            let AUV = RI * RI;
            let CGF = BSJ * RI;
            let CGG = (CGF + CGF) * AUW;
            let CGH = Lanes([0.0, CGG[0], CGG[1]]) + Lanes([(BJF * AUV), 0.0, 0.0]);
            let AUY = RH * RH;
            let CGI = BSI * RH;
            let CGJ = (CGI + CGI) * AUZ;
            let CGK = Lanes([0.0, CGJ[0], CGJ[1]]) + Lanes([(BJH * AUY), 0.0, 0.0]);
            let CGL = BSF * RD;
            let CGM = CGL + CGL;
            let AVB = (RD * RD) / ATL;
            let CGN = (Lanes([CGM[0], 0.0, CGM[1]]) - Lanes([0.0, (BIT * AVB), 0.0])) / ATL;
            let CGO = BSC * AON;
            let CGP = (CDL * QY) + Lanes([0.0, 0.0, CGO[0], CGO[1], 0.0, 0.0]);
            let CGQ = BSA * M;
            let AVF = ((((AVC + AFY) + (M * QV)) - AVE) + ACZ) + ACK;
            let CGR = ((((BKR + Lanes([BZY[0], BZY[1], BZY[2], 0.0, 0.0])) + Lanes([0.0, CGQ[0], CGQ[1], 0.0, 0.0])) - Lanes([BLC[0], BLC[1], BLC[2], 0.0, 0.0])) + Lanes([0.0, BYR[0], BYR[1], 0.0, 0.0])) + Lanes([BYN[0], BYN[1], BYN[2], 0.0, 0.0]);
            let CGS = BSA * AVF;
            let CGT = (CGR * QV) + Lanes([0.0, CGS[0], CGS[1], 0.0, 0.0]);
            let CGU = CCS * AVG;
            let CGV = BMO * ANN;
            let CGW = Lanes([CGU[0], CGU[1], CGU[2], CGU[3], 0.0]) + Lanes([0.0, 0.0, CGV[0], CGV[1], CGV[2]]);
            let AVI = (AVH + AGG) + AGW;
            let CGX = (BKW + CAC) + CAK;
            let CGY = BSB * AVI;
            let CGZ = (CGX * QX) + Lanes([0.0, CGY[0], CGY[1]]);
            let AVJ = M * RJ;
            let CHA = BSM * M;
            let AVK = (ANP + ANQ) + AVJ;
            let CHB = Lanes([0.0, CHA[0], CHA[1], CHA[2], CHA[3], CHA[4]]);
            let CHC = BSM * AVK;
            let CHD = (((CCU + CCW) + CHB) * RJ) + Lanes([0.0, CHC[0], CHC[1], CHC[2], CHC[3], CHC[4]]);
            let CHE = BSR * ANS;
            let CHF = (CCY * RL) + Lanes([CHE[0], CHE[1], 0.0, CHE[2], CHE[3], CHE[4], CHE[5], CHE[6], CHE[7]]);
            let AVL = (((((((((((((ABR * AUL) + (TY * AUM)) - (AUN * AUK)) + AUR) + (AUS * AUT)) + (AUV * AUW)) + (AUY * AUZ)) + AVB) + (AON * QY)) + (AVF * QV)) - (ANN * AVG)) + (AVI * QX)) + (AVK * RJ)) + (ANS * RL);
            let CHG = (((((((((Lanes([0.0, 0.0, CGB[0], CGB[1], CGB[2], 0.0, CGB[3], CGB[4], CGB[5], 0.0, 0.0]) + Lanes([CGE[0], CGE[1], 0.0, CGE[2], 0.0, CGE[3], CGE[4], CGE[5], CGE[6], CGE[7], CGE[8]])) + Lanes([0.0, 0.0, 0.0, CGH[0], 0.0, 0.0, 0.0, 0.0, 0.0, CGH[1], CGH[2]])) + Lanes([0.0, 0.0, 0.0, CGK[0], 0.0, 0.0, 0.0, CGK[1], 0.0, 0.0, CGK[2]])) + Lanes([0.0, CGN[0], 0.0, CGN[1], 0.0, CGN[2], 0.0, 0.0, 0.0, 0.0, 0.0])) + Lanes([0.0, 0.0, 0.0, CGP[0], CGP[1], CGP[2], CGP[3], CGP[4], CGP[5], 0.0, 0.0])) + Lanes([0.0, 0.0, 0.0, CGT[0], CGT[1], 0.0, CGT[2], CGT[3], CGT[4], 0.0, 0.0])) - Lanes([0.0, 0.0, 0.0, CGW[0], 0.0, CGW[1], CGW[2], CGW[3], CGW[4], 0.0, 0.0])) + Lanes([0.0, 0.0, 0.0, CGZ[0], CGZ[1], CGZ[2], 0.0, 0.0, 0.0, 0.0, 0.0])) + Lanes([0.0, 0.0, 0.0, CHD[0], 0.0, CHD[1], CHD[2], CHD[3], CHD[4], 0.0, CHD[5]])) + Lanes([CHF[0], CHF[1], 0.0, CHF[2], 0.0, CHF[3], CHF[4], CHF[5], CHF[6], CHF[7], CHF[8]]);
            let AVN = C - AVM;
            let AVO = AVN * GM;
            let CHH = BPK * AVN;
            let AVP = AVO * YM;
            let CHI = Lanes([(CHH * YM), 0.0, 0.0]) + (BWQ * AVO);
            let CHJ = Lanes([0.0, BSB[0], BSB[1]]);
            let CHK = Lanes([BWF, 0.0, 0.0]);
            let AVQ = (QX - XU) / XV;
            let CHL = ((CHJ - CHK) - Lanes([(BWG * AVQ), 0.0, 0.0])) / XV;
            let AVR = if QX < XU { 1.0 } else { 0.0 };
            let AWB;
            let BMP;
            if AVR != 0.0 {
                let AVS = AVQ.exp();
                let AVT = C + AVS;
                let AVU = AVT.ln();
                let AVV = QX - (XV * AVU);
                let CHN = CHJ - (Lanes([(BWG * AVU), 0.0, 0.0]) + (((CHL * AVS) * (BHV / AVT)) * XV));
                AWB = AVV;
                BMP = CHN;
            } else {
                let AVW = (-AVQ).exp();
                let AVX = C + AVW;
                let AVY = AVX.ln();
                let AVZ = XU - (XV * AVY);
                let CHM = CHK - (Lanes([(BWG * AVY), 0.0, 0.0]) + ((((CHL * BNP) * AVW) * (BHV / AVX)) * XV));
                AWB = AVZ;
                BMP = CHM;
            }
            let AWA = AVM * GM;
            let AWC = C - (AWB * GE);
            let AWD = C - (AWC.powf(YI));
            let AWE = (YK * AWD) + (CT * (QX - AWB));
            let AWF = AWA * AWE;
            let CHO = Lanes([((BPK * AVM) * AWE), 0.0, 0.0]) + (((Lanes([(BWP * AWD), 0.0, 0.0]) + ((((((BMP * GE) + Lanes([(BPF * AWB), 0.0, 0.0])) * BNP) * (YI * (AWC.powf(BWN)))) * BNP) * YK)) + ((CHJ - BMP) * CT)) * AWA);
            let AWH = AWG * GU;
            let AWI = AWH * ZY;
            let CHP = Lanes([((BPN * AWG) * ZY), 0.0, 0.0, 0.0]) + (BXG * AWH);
            let AWJ = PG * KD;
            let CHQ = (BRO * KD) + (BQN * PG);
            let AWK = JI * AWJ;
            let CHR = CHQ * JI;
            let AWL = AWK * AAE;
            let AWM = AWL * AOE;
            let CHS = (Lanes([(CHR * AAE), 0.0, 0.0]) + (BXK * AWK)) * AOE;
            let CHT = Lanes([CHS[0], CHS[1], CHS[2], 0.0, 0.0]) + (BLS * AWL);
            let AWN = AWK * AAM;
            let AWO = AWN * AOE;
            let CHU = (Lanes([(CHR * AAM), 0.0, 0.0, 0.0]) + (BXO * AWK)) * AOE;
            let CHV = Lanes([CHU[0], 0.0, CHU[1], CHU[2], CHU[3]]) + (BLS * AWN);
            let AWP = AE * GP;
            let CHW = BIP * AE;
            let CHX = Lanes([BWW, 0.0, 0.0, 0.0, 0.0, 0.0]);
            let AWQ = (RJ - YW) / AWP;
            let CHY = ((BTR - CHX) - Lanes([(CHW * AWQ), 0.0, 0.0, 0.0, 0.0, 0.0])) / AWP;
            let AWR = if RJ < YW { 1.0 } else { 0.0 };
            let AXA;
            let BMQ;
            if AWR != 0.0 {
                let AWS = AWQ.exp();
                let AWT = C + AWS;
                let AWU = AWT.ln();
                let AWV = RJ - (AWP * AWU);
                let CIA = BTR - (Lanes([(CHW * AWU), 0.0, 0.0, 0.0, 0.0, 0.0]) + (((CHY * AWS) * (BHV / AWT)) * AWP));
                AXA = AWV;
                BMQ = CIA;
            } else {
                let AWW = (-AWQ).exp();
                let AWX = C + AWW;
                let AWY = AWX.ln();
                let AWZ = YW - (AWP * AWY);
                let CHZ = CHX - (Lanes([(CHW * AWY), 0.0, 0.0, 0.0, 0.0, 0.0]) + ((((CHY * BNP) * AWW) * (BHV / AWX)) * AWP));
                AXA = AWZ;
                BMQ = CHZ;
            }
            let AXB = AXA / GP;
            let AXC = C - AXB;
            let AXD = C - (AXC.powf(ZO));
            let AXE = RJ - AXA;
            let AXF = (ZP * AXD) + (YT * AXE);
            let CIB = BSM * GV;
            let AXG = (YS * AXF) + (GV * RJ);
            let AXH = C - AWG;
            let AXI = ((GU * AXG) * AXH) * J;
            let CIC = ((Lanes([(BPN * AXG), 0.0, 0.0, 0.0, 0.0, 0.0]) + (((Lanes([(BWU * AXF), 0.0, 0.0, 0.0, 0.0, 0.0]) + (((Lanes([(BXC * AXD), 0.0, 0.0, 0.0, 0.0, 0.0]) + ((((((BMQ - Lanes([(BIP * AXB), 0.0, 0.0, 0.0, 0.0, 0.0])) / GP) * BNP) * (ZO * (AXC.powf(BXD)))) * BNP) * ZP)) + (Lanes([(BWV * AXE), 0.0, 0.0, 0.0, 0.0, 0.0]) + ((BTR - BMQ) * YT))) * YS)) + (Lanes([(BPO * RJ), 0.0, 0.0, 0.0, 0.0, 0.0]) + Lanes([0.0, CIB[0], CIB[1], CIB[2], CIB[3], CIB[4]]))) * GU)) * AXH) * J;
            let CID = Lanes([0.0, 0.0, BWW, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
            let AXJ = (RL - YW) / AWP;
            let CIE = ((BTN - CID) - Lanes([0.0, 0.0, (CHW * AXJ), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])) / AWP;
            let AXK = if RL < YW { 1.0 } else { 0.0 };
            let AXT;
            let BMR;
            if AXK != 0.0 {
                let AXL = AXJ.exp();
                let AXM = C + AXL;
                let AXN = AXM.ln();
                let AXO = RL - (AWP * AXN);
                let CIG = BTN - (Lanes([0.0, 0.0, (CHW * AXN), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]) + (((CIE * AXL) * (BHV / AXM)) * AWP));
                AXT = AXO;
                BMR = CIG;
            } else {
                let AXP = (-AXJ).exp();
                let AXQ = C + AXP;
                let AXR = AXQ.ln();
                let AXS = YW - (AWP * AXR);
                let CIF = CID - (Lanes([0.0, 0.0, (CHW * AXR), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]) + ((((CIE * BNP) * AXP) * (BHV / AXQ)) * AWP));
                AXT = AXS;
                BMR = CIF;
            }
            let AXU = AXT / GP;
            let AXV = C - AXU;
            let AXW = C - (AXV.powf(ZO));
            let AXX = RL - AXT;
            let AXY = (ZP * AXW) + (YT * AXX);
            let CIH = BSR * GV;
            let AXZ = (YS * AXY) + (GV * RL);
            let AYA = ((GU * AXZ) * AXH) * I;
            let CII = ((Lanes([0.0, 0.0, (BPN * AXZ), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]) + (((Lanes([0.0, 0.0, (BWU * AXY), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]) + (((Lanes([0.0, 0.0, (BXC * AXW), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]) + ((((((BMR - Lanes([0.0, 0.0, (BIP * AXU), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])) / GP) * BNP) * (ZO * (AXV.powf(BXD)))) * BNP) * ZP)) + (Lanes([0.0, 0.0, (BWV * AXX), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]) + ((BTN - BMR) * YT))) * YS)) + (Lanes([0.0, 0.0, (BPO * RL), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]) + Lanes([CIH[0], CIH[1], 0.0, CIH[2], CIH[3], CIH[4], CIH[5], CIH[6], CIH[7]]))) * GU)) * AXH) * I;
            let AYB = PC * KD;
            let AYC = JZ / KD;
            let AYE = C / AYD;
            let AYF = AYC.powf(AYE);
            let AYG = AYB * AYF;
            let CIJ = (((BRN * KD) + (BQN * PC)) * AYF) + ((((BQM - (BQN * AYC)) / KD) * (AYE * (AYC.powf((AYE - BHV))))) * AYB);
            let AYH = AYD * BO;
            let CIK = BNT * AYD;
            let AYI = QV / AYH;
            let CIL = (BWH - Lanes([(CIK * AYI), 0.0, 0.0])) / AYH;
            let AYJ = if AYI < RN { 1.0 } else { 0.0 };
            let AYN;
            let BMS;
            if AYJ != 0.0 {
                let AYK = AYI.exp();
                let CIN = CIL * AYK;
                AYN = AYK;
                BMS = CIN;
            } else {
                let AYL = RN.exp();
                let AYM = AYL * (C + (AYI - RN));
                let CIM = CIL * AYL;
                AYN = AYM;
                BMS = CIM;
            }
            let AYO = AYG * AYN;
            let CIO = Lanes([(CIJ * AYN), 0.0, 0.0]) + (BMS * AYG);
            let AYP = JP * PK;
            let AYQ = (AYP * BO) / HZ;
            let CIP = ((((BRP * JP) * BO) + (BNT * AYP)) - (BPX * AYQ)) / HZ;
            let AYR = JI * AYQ;
            let AYS = AYR * APZ;
            let AYU = (AYT + VX) + V;
            let AYV = AYS * AYU;
            let CIQ = ((Lanes([((CIP * JI) * APZ), 0.0, 0.0, 0.0]) + (BJX * AYR)) * AYU) + ((BJZ + Lanes([BJS[0], BJS[1], BJS[2], 0.0])) * AYS);
            let AYW = if parameters[78] == A { 1.0 } else { 0.0 };
            let AZO;
            let BMT;
            if AYW != 0.0 {
                let AYX = PO * JI;
                let AYY = (AWJ * AKV) + (AYQ * AKY);
                let AYZ = (AYX * AYY) / PM;
                let CIV = ((Lanes([((BRR * JI) * AYY), 0.0, 0.0, 0.0, 0.0, 0.0]) + (((Lanes([(CHQ * AKV), 0.0, 0.0, 0.0, 0.0, 0.0]) + (CBT * AWJ)) + (Lanes([(CIP * AKY), 0.0, 0.0, 0.0, 0.0, 0.0]) + (CBU * AYQ))) * AYX)) - Lanes([(BRQ * AYZ), 0.0, 0.0, 0.0, 0.0, 0.0])) / PM;
                AZO = AYZ;
                BMT = CIV;
            } else {
                let AZC = (RJ - AZA) / AZB;
                let AZD = AZC * BP;
                let CIR = (((BTR - Lanes([BIO, 0.0, 0.0, 0.0, 0.0, 0.0])) / AZB) * BP) + Lanes([(BNU * AZC), 0.0, 0.0, 0.0, 0.0, 0.0]);
                let AZE = if AZD < RN { 1.0 } else { 0.0 };
                let AZJ;
                let BMU;
                if AZE != 0.0 {
                    let AZF = AZD.exp();
                    let CIT = CIR * AZF;
                    AZJ = AZF;
                    BMU = CIT;
                } else {
                    let AZG = RN.exp();
                    let AZH = AZG * (C + (AZD - RN));
                    let CIS = CIR * AZG;
                    AZJ = AZH;
                    BMU = CIS;
                }
                let AZI = AKZ * PS;
                let AZK = (C + (JP * AZJ)).sqrt();
                let AZL = C + AZK;
                let AZM = (AZI * AKP) / AZL;
                let CIU = ((Lanes([(((CBV * PS) + (BRS * AKZ)) * AKP), 0.0, 0.0, 0.0, 0.0, 0.0]) + (BJL * AZI)) - (((BMU * JP) * (BHV / (BQJ * AZK))) * AZM)) / AZL;
                AZO = AZM;
                BMT = CIU;
            }
            let AZN = if (if (if ALF == C { 1.0 } else { 0.0 }) != 0.0 || (if ALF == CT { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && ALG != 0.0 { 1.0 } else { 0.0 };
            let BDX;
            let BEB;
            let BMV;
            let BMW;
            if AZN != 0.0 {
                let AZP = AZO * J;
                let CIW = BMT * J;
                let BAR;
                let BMX;
                if AYW != 0.0 {
                    let AZQ = ZZ * ALL;
                    let CJB = Lanes([0.0, 0.0, (BXH * ALL), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]) + (BJN * ZZ);
                    let AZR = (C + AZQ).sqrt();
                    let AZS = C + AZR;
                    let AZT = (AZQ - ZZ) / AZS;
                    let AZV = JP * AZU;
                    let CJC = BJO * JP;
                    let AZW = (C + AZV).sqrt();
                    let AZX = C + AZW;
                    let AZY = AZV / AZX;
                    let AZZ = JI * I;
                    let BAA = AZZ * PO;
                    let BAB = (AWJ * AZT) + (AYQ * AZY);
                    let BAC = (BAA * BAB) / PM;
                    let CJD = ((Lanes([0.0, 0.0, ((BRR * AZZ) * BAB), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]) + (((Lanes([0.0, 0.0, (CHQ * AZT), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]) + ((((CJB - Lanes([0.0, 0.0, BXH, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])) - ((CJB * (BHV / (BQJ * AZR))) * AZT)) / AZS) * AWJ)) + (Lanes([0.0, 0.0, (CIP * AZY), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]) + (((CJC - ((CJC * (BHV / (BQJ * AZW))) * AZY)) / AZX) * AYQ))) * BAA)) - Lanes([0.0, 0.0, (BRQ * BAC), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])) / PM;
                    BAR = BAC;
                    BMX = CJD;
                } else {
                    let BAD = RL - AZA;
                    let BAE = BAD * BP;
                    let CIX = ((BTN - Lanes([0.0, 0.0, BIO, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])) * BP) + Lanes([0.0, 0.0, (BNU * BAD), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
                    let BAF = if BAE < RN { 1.0 } else { 0.0 };
                    let BAM;
                    let BMY;
                    if BAF != 0.0 {
                        let BAG = BAE.exp();
                        let CIZ = CIX * BAG;
                        BAM = BAG;
                        BMY = CIZ;
                    } else {
                        let BAH = RN.exp();
                        let BAI = BAH * (C + (BAE - RN));
                        let CIY = CIX * BAH;
                        BAM = BAI;
                        BMY = CIY;
                    }
                    let BAJ = V * I;
                    let BAK = BAJ * MK;
                    let BAL = BAK * PS;
                    let BAN = (C + (JP * BAM)).sqrt();
                    let BAO = C + BAN;
                    let BAP = (BAL * ALL) / BAO;
                    let CJA = ((Lanes([0.0, 0.0, ((((BQZ * BAJ) * PS) + (BRS * BAK)) * ALL), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]) + (BJN * BAL)) - (((BMY * JP) * (BHV / (BQJ * BAN))) * BAP)) / BAO;
                    BAR = BAP;
                    BMX = CJA;
                }
                let BAS = BAQ * BAR;
                let CJE = (BLM * BAR) + (BMX * BAQ);
                BDX = BAS;
                BEB = AZP;
                BMV = CJE;
                BMW = CIW;
            } else {
                BDX = A;
                BEB = AZO;
                BMV = CBY;
                BMW = BMT;
            }
            let BAT = if parameters[6] == C { 1.0 } else { 0.0 };
            let BDC;
            let BDD;
            let BDI;
            let BDL;
            let BMZ;
            let BNA;
            let BNB;
            let BNC;
            if BAT != 0.0 {
                let BAU = -W;
                let CJG = BWM * (BAU * (YH.powf((BAU - BHV))));
                let BAV = (YH.powf(BAU)) - CT;
                let BAW = if XW < A { 1.0 } else { 0.0 };
                let BBD;
                let BND;
                if BAW != 0.0 {
                    let BAX = XW.exp();
                    let BAY = C + BAX;
                    let BAZ = C / BAY;
                    let CJJ = (((BWJ * BAX) * BAZ) * BNP) / BAY;
                    BBD = BAZ;
                    BND = CJJ;
                } else {
                    let BBA = (-XW).exp();
                    let CJH = (BWJ * BNP) * BBA;
                    let BBB = C + BBA;
                    let BBC = BBA / BBB;
                    let CJI = (CJH - (CJH * BBC)) / BBB;
                    BBD = BBC;
                    BND = CJI;
                }
                let BBE = (BAV * BBD) + CT;
                let CJK = Lanes([(CHH * BBE), 0.0, 0.0]) + (((CJG * BBD) + (BND * BAV)) * AVO);
                let BBF = (AAB * BP) / JS;
                let BBG = JI / AAC;
                let BBH = BBF * BBG;
                let BBI = AWK * AOE;
                let CJL = ((((((BXI * BP) + Lanes([(BNU * AAB), 0.0, 0.0])) - Lanes([(BIU * BBF), 0.0, 0.0])) / JS) * BBG) + ((((BXJ * BBG) * BNP) / AAC) * BBF)) * BBI;
                let BBJ = AYO / AYH;
                let CJM = (CIO - Lanes([(CIK * BBJ), 0.0, 0.0])) / AYH;
                let BBK = UL * QY;
                let BBL = ((AVO * BBE) + (BBI * BBH)) + BBJ;
                let BBM = BBK * BBL;
                let CJN = (BSC * UL) * BBL;
                let CJO = ((Lanes([CJK[0], CJK[1], CJK[2], 0.0, 0.0]) + (((Lanes([(CHR * AOE), 0.0, 0.0, 0.0, 0.0]) + (BLS * AWK)) * BBH) + Lanes([CJL[0], CJL[1], CJL[2], 0.0, 0.0]))) + Lanes([CJM[0], CJM[1], CJM[2], 0.0, 0.0])) * BBK;
                let CJP = Lanes([0.0, 0.0, CJN[0], CJN[1], 0.0, 0.0]) + Lanes([CJO[0], CJO[1], 0.0, CJO[2], CJO[3], CJO[4]]);
                let BBO = C - BBN;
                let BBP = BBO * AYO;
                let CJQ = CIO * BBO;
                let CJR = CIO * BBN;
                let BBQ = AWM + (BBN * AYO);
                let CJS = CHT + Lanes([CJR[0], CJR[1], CJR[2], 0.0, 0.0]);
                let BBS = (BBR * BBQ) + AWO;
                let CJT = (CJS * BBR) + CHV;
                let BBT = C - BBR;
                let BBU = BBT * BBQ;
                let CJU = CJS * BBT;
                BDC = BBU;
                BDD = BBP;
                BDI = BBS;
                BDL = BBM;
                BMZ = CJU;
                BNA = CJQ;
                BNB = CJT;
                BNC = CJP;
            } else {
                BDC = AWM;
                BDD = AYO;
                BDI = AWO;
                BDL = A;
                BMZ = CHT;
                BNA = CIO;
                BNB = CHV;
                BNC = CJF;
            }
            let BBV = (B * TY) * R;
            let CJV = (BUL * B) * R;
            let BBW = (B * ABR) * R;
            let CJW = (BYF * B) * R;
            let BBX = (B * AVI) * R;
            let CJX = (CGX * B) * R;
            let BBY = (B * AVF) * R;
            let CJY = (CGR * B) * R;
            let BGV;
            let BGW;
            let BNE;
            let BNF;
            if LR != 0.0 {
                let BBZ = (B * (-ANN)) * R;
                let CKB = ((CCS * BNP) * B) * R;
                BGV = BBZ;
                BGW = A;
                BNE = CKB;
                BNF = CKA;
            } else {
                let BCA = (B * (-ANN)) * R;
                let CJZ = ((CCS * BNP) * B) * R;
                BGV = A;
                BGW = BCA;
                BNE = CKA;
                BNF = CJZ;
            }
            let BCB = (B * AON) * R;
            let CKC = (CDL * B) * R;
            let BCD = (B * (BCC * AUN)) * R;
            let CKD = ((BLU * BCC) * B) * R;
            let CKE = BSE * B;
            let BCE = (B * RB) / ATQ;
            let BCF = BCE * R;
            let CKF = ((Lanes([CKE[0], 0.0, CKE[1]]) - Lanes([0.0, (BIS * BCE), 0.0])) / ATQ) * R;
            let CKG = BSF * B;
            let BCG = (B * RD) / ATL;
            let BCH = BCG * R;
            let CKH = ((Lanes([CKG[0], 0.0, CKG[1]]) - Lanes([0.0, (BIT * BCG), 0.0])) / ATL) * R;
            let BCJ = BCI * BB;
            let CKI = BHW * BCI;
            let BCK = ddt(12393, BCJ) * R;
            let CKK = (CKI * CKJ) * R;
            let BHL = BCJ * R;
            let CKL = CKI * R;
            let BCL = C - QE;
            let BCM = if QD > S { 1.0 } else { 0.0 };
            let BCX;
            let BNG;
            if BCM != 0.0 {
                let BCN = if parameters[132] == A { 1.0 } else { 0.0 };
                let BCY;
                let BNH;
                if BCN != 0.0 {
                    let BCO = (BB / QF) * R;
                    let CKP = (BHW / QF) * R;
                    BCY = BCO;
                    BNH = CKP;
                } else {
                    let BCP = if (BCL.abs()) < AML { 1.0 } else { 0.0 };
                    let BCZ;
                    let BNI;
                    if BCP != 0.0 {
                        let BCQ = (L / QF) * R;
                        let BCR = C + (BB / L);
                        let BCS = BCQ * (BCR.ln());
                        let CKO = ((BHW / L) * (BHV / BCR)) * BCQ;
                        BCZ = BCS;
                        BNI = CKO;
                    } else {
                        let BCT = (L / (BCL * QF)) * R;
                        let BCU = C + (BB / L);
                        let BCV = BCT * ((BCU.powf(BCL)) - C);
                        let CKN = ((BHW / L) * (BCL * (BCU.powf((BCL - BHV))))) * BCT;
                        BCZ = BCV;
                        BNI = CKN;
                    }
                    BCY = BCZ;
                    BNH = BNI;
                }
                BCX = BCY;
                BNG = BNH;
            } else {
                let BCW = BB / Q;
                let CKM = BHW / Q;
                BCX = BCW;
                BNG = CKM;
            }
            let BDB = (BDA * AVL) * R;
            let CKQ = (CHG * BDA) * R;
            let BDE = B * ((AVP + BDC) + BDD);
            let CKR = ((Lanes([CHI[0], CHI[1], CHI[2], 0.0, 0.0]) + BMZ) + Lanes([BNA[0], BNA[1], BNA[2], 0.0, 0.0])) * B;
            let BDF = ddt(12461, BDE) * R;
            let CKS = (CKR * CKJ) * R;
            let BHM = BDE * R;
            let CKT = CKR * R;
            let BDG = B * AWF;
            let CKU = CHO * B;
            let BDH = ddt(12467, BDG) * R;
            let CKV = (CKU * CKJ) * R;
            let BHN = BDG * R;
            let CKW = CKU * R;
            let BDJ = B * ((AWI + BDI) + AYV);
            let CKX = ((Lanes([CHP[0], 0.0, CHP[1], CHP[2], CHP[3]]) + BNB) + Lanes([CIQ[0], 0.0, CIQ[1], CIQ[2], CIQ[3]])) * B;
            let BDK = ddt(12477, BDJ) * R;
            let CKY = (CKX * CKJ) * R;
            let BHO = BDJ * R;
            let CKZ = CKX * R;
            let BDM = B * BDL;
            let CLA = BNC * B;
            let BDN = ddt(12483, BDM) * R;
            let CLB = (CLA * CKJ) * R;
            let BHP = BDM * R;
            let CLC = CLA * R;
            let BDO = B * parameters[68];
            let BDP = BDO * RE;
            let CLD = BSG * BDO;
            let BDQ = ddt(12491, BDP) * R;
            let CLE = (CLD * CKJ) * R;
            let BHQ = BDP * R;
            let CLF = CLD * R;
            let BDR = B * parameters[77];
            let BDS = BDR * RF;
            let CLG = BSH * BDR;
            let BDT = ddt(12499, BDS) * R;
            let CLH = (CLG * CKJ) * R;
            let BHR = BDS * R;
            let CLI = CLG * R;
            let BDU = (B * ANS) * R;
            let CLJ = (CCY * B) * R;
            let BDV = B * RK;
            let CLK = (BSQ * B) * AUT;
            let BDW = (BDV * AUT) * R;
            let CLL = (Lanes([CLK[0], CLK[1], 0.0, CLK[2], CLK[3], CLK[4], CLK[5], CLK[6], CLK[7]]) + Lanes([0.0, 0.0, (BJD * BDV), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])) * R;
            let BDY = B * (AYA + BDX);
            let CLM = (CII + BMV) * B;
            let BDZ = ddt(12519, BDY) * R;
            let CLN = (CLM * CKJ) * R;
            let BHS = BDY * R;
            let CLO = CLM * R;
            let BEA = (B * ((ANQ + AVJ) + ANP)) * R;
            let CLP = (((CCW + CHB) + CCU) * B) * R;
            let BEC = B * (AXI + BEB);
            let CLQ = (CIC + BMW) * B;
            let BED = ddt(12538, BEC) * R;
            let CLR = (CLQ * CKJ) * R;
            let BHT = BEC * R;
            let CLS = CLQ * R;
            let BGX;
            let BGY;
            let BNJ;
            if QJ != 0.0 {
                let BEE = B * RI;
                let CLU = (BSJ * B) * AUW;
                let BEF = (BEE * AUW) * R;
                let CLV = (Lanes([0.0, CLU[0], CLU[1]]) + Lanes([(BJF * BEE), 0.0, 0.0])) * R;
                BGX = BEF;
                BGY = A;
                BNJ = CLV;
            } else {
                BGX = A;
                BGY = BEG;
                BNJ = CLT;
            }
            let BGZ;
            let BHA;
            let BNK;
            if QM != 0.0 {
                let BEH = B * RH;
                let CLX = (BSI * B) * AUZ;
                let BEI = (BEH * AUZ) * R;
                let CLY = (Lanes([0.0, CLX[0], CLX[1]]) + Lanes([(BJH * BEH), 0.0, 0.0])) * R;
                BGZ = BEI;
                BHA = A;
                BNK = CLY;
            } else {
                BGZ = A;
                BHA = BEJ;
                BNK = CLW;
            }
            let BEK = (ABQ + ABP) / ABM;
            let CLZ = ((BYD + BYE) - (BYA * BEK)) / ABM;
            let BEL = if parameters[129] > A { 1.0 } else { 0.0 };
            let BEO;
            let BNL;
            if BEL != 0.0 {
                let BEM = AUN / BEK;
                let BEN = BEM.abs();
                let CMA = ((BLU - (CLZ * BEM)) / BEK) * ((BQJ * (if BEM >= 0e0f64 { 1.0 } else { 0.0 })) - BHV);
                BEO = BEN;
                BNL = CMA;
            } else {
                BEO = A;
                BNL = CDH;
            }
            let BEP = if BEK > A { 1.0 } else { 0.0 };
            let BEV;
            let BNM;
            if BEP != 0.0 {
                let BEQ = (BDC + BDI) / BEK;
                let CMC = ((BMZ + BNB) - (CLZ * BEQ)) / BEK;
                BEV = BEQ;
                BNM = CMC;
            } else {
                let BER = PG * AOE;
                let BES = BER * ABM;
                let CMB = ((Lanes([(BRO * AOE), 0.0, 0.0, 0.0, 0.0]) + (BLS * PG)) * ABM) + (BYA * BER);
                BEV = BES;
                BNM = CMB;
            }
            let BEU = if BET == C { 1.0 } else { 0.0 };
            let BFI;
            let BNN;
            if BEU != 0.0 {
                let BEW = BBR * BEV;
                let CME = BNM * BBR;
                BFI = BEW;
                BNN = CME;
            } else {
                let BEX = if BET == V { 1.0 } else { 0.0 };
                let BFJ;
                let BNO;
                if BEX != 0.0 {
                    let BEZ = BEY * BEV;
                    let CMD = BNM * BEY;
                    BFJ = BEZ;
                    BNO = CMD;
                } else {
                    BFJ = A;
                    BNO = CDH;
                }
                BFI = BFJ;
                BNN = BNO;
            }
            let BFA = if (AVC + AVH) < A { 1.0 } else { 0.0 };
            if BFA != 0.0 {
            } else {
            }
            let BFB = if ((AFY + AGG) + AGW) < A { 1.0 } else { 0.0 };
            if BFB != 0.0 {
            } else {
            }
            let BFC = if ANQ < A { 1.0 } else { 0.0 };
            if BFC != 0.0 {
            } else {
            }
            let BFD = if ANP < A { 1.0 } else { 0.0 };
            if BFD != 0.0 {
            } else {
            }
            let BFE = if I == A { 1.0 } else { 0.0 };
            if BFE != 0.0 {
            } else {
            }
            let BFF = if ANS < A { 1.0 } else { 0.0 };
            if BFF != 0.0 {
            } else {
            }
            let BFK = ddt(12791, BFH);
            let BFL = BFI * BFK;
            let CMF = BNN * BFK;
            let CMG = Lanes([CMF[0], CMF[1], CMF[2], CMF[3], CMF[4], 0.0]) + Lanes([0.0, 0.0, 0.0, 0.0, 0.0, ((BIH * CKJ) * BFI)]);
            let BHU = BFI * BFH;
            let CMH = BNN * BFH;
            let CMI = Lanes([CMH[0], CMH[1], CMH[2], CMH[3], CMH[4], 0.0]) + Lanes([0.0, 0.0, 0.0, 0.0, 0.0, (BIH * BFI)]);
            let BFM = BEO * BFH;
            let CMJ = BNL * BFH;
            let CMK = Lanes([CMJ[0], CMJ[1], CMJ[2], CMJ[3], CMJ[4], 0.0]) + Lanes([0.0, 0.0, 0.0, 0.0, 0.0, (BIH * BEO)]);
            let BHB;
            let BHC;
            if LR != 0.0 {
                BHB = BGB;
                BHC = A;
            } else {
                BHB = A;
                BHC = BGC;
            }
            let BGL;
            let BGN;
            let BGP;
            let BGR;
            let BHD;
            let BHF;
            let BHH;
            let BHJ;
            if QJ != 0.0 {
                let BGM;
                let BGO;
                let BHE;
                let BHG;
                let BHI;
                if QM != 0.0 {
                    BGM = BGD;
                    BGO = A;
                    BHE = BGE;
                    BHG = BGF;
                    BHI = A;
                } else {
                    BGM = A;
                    BGO = BGG;
                    BHE = A;
                    BHG = A;
                    BHI = BGH;
                }
                BGL = BGM;
                BGN = BGO;
                BGP = A;
                BGR = A;
                BHD = BHE;
                BHF = BHG;
                BHH = BHI;
                BHJ = A;
            } else {
                let BGQ;
                let BGS;
                let BHK;
                if QM != 0.0 {
                    BGQ = BGI;
                    BGS = A;
                    BHK = BGJ;
                } else {
                    BGQ = A;
                    BGS = BGK;
                    BHK = A;
                }
                BGL = A;
                BGN = A;
                BGP = BGQ;
                BGR = BGS;
                BHD = A;
                BHF = A;
                BHH = A;
                BHJ = BHK;
            }
            let BGT = if ((((BCH + BDQ) + BDT) + BDU) + BDZ) == A { 1.0 } else { 0.0 };
            if BGT != 0.0 {
            } else {
            }
            let BGU = if R != C { 1.0 } else { 0.0 };
            if BGU != 0.0 {
            } else {
            }
            let CML = CJV[0];
            let CMM = CJV[1];
            let CMN = CJV[2];
            let CMO = CJV[3];
            let CMP = CJW[0];
            let CMQ = CJW[1];
            let CMR = CJW[2];
            let CMS = CJW[3];
            let CMT = CJW[4];
            let CMU = CJX[0];
            let CMV = CJX[1];
            let CMW = CJX[2];
            let CMX = CJY[0];
            let CMY = CJY[1];
            let CMZ = CJY[2];
            let CNA = CJY[3];
            let CNB = CJY[4];
            let CNC = BNE[0];
            let CND = BNE[1];
            let CNE = BNE[2];
            let CNF = BNE[3];
            let CNG = BNF[0];
            let CNH = BNF[1];
            let CNI = BNF[2];
            let CNJ = BNF[3];
            let CNK = CKC[0];
            let CNL = CKC[1];
            let CNM = CKC[2];
            let CNN = CKC[3];
            let CNO = CKC[4];
            let CNP = CKC[5];
            let CNQ = CKD[0];
            let CNR = CKD[1];
            let CNS = CKD[2];
            let CNT = CKD[3];
            let CNU = CKD[4];
            let CNV = CKF[0];
            let CNW = CKF[1];
            let CNX = CKF[2];
            let CNY = CKH[0];
            let CNZ = CKH[1];
            let COA = CKH[2];
            let COB = BNG;
            let COC = CKK;
            let COD = CKQ[0];
            let COE = CKQ[1];
            let COF = CKQ[2];
            let COG = CKQ[3];
            let COH = CKQ[4];
            let COI = CKQ[5];
            let COJ = CKQ[6];
            let COK = CKQ[7];
            let COL = CKQ[8];
            let COM = CKQ[9];
            let CON = CKQ[10];
            let COO = CKS[0];
            let COP = CKS[1];
            let COQ = CKS[2];
            let COR = CKS[3];
            let COS = CKS[4];
            let COT = CKV[0];
            let COU = CKV[1];
            let COV = CKV[2];
            let COW = CKY[0];
            let COX = CKY[1];
            let COY = CKY[2];
            let COZ = CKY[3];
            let CPA = CKY[4];
            let CPB = CLB[0];
            let CPC = CLB[1];
            let CPD = CLB[2];
            let CPE = CLB[3];
            let CPF = CLB[4];
            let CPG = CLB[5];
            let CPH = CLE[0];
            let CPI = CLE[1];
            let CPJ = CLH[0];
            let CPK = CLH[1];
            let CPL = CLJ[0];
            let CPM = CLJ[1];
            let CPN = CLJ[2];
            let CPO = CLJ[3];
            let CPP = CLJ[4];
            let CPQ = CLJ[5];
            let CPR = CLJ[6];
            let CPS = CLJ[7];
            let CPT = CLJ[8];
            let CPU = CLL[0];
            let CPV = CLL[1];
            let CPW = CLL[2];
            let CPX = CLL[3];
            let CPY = CLL[4];
            let CPZ = CLL[5];
            let CQA = CLL[6];
            let CQB = CLL[7];
            let CQC = CLL[8];
            let CQD = CLN[0];
            let CQE = CLN[1];
            let CQF = CLN[2];
            let CQG = CLN[3];
            let CQH = CLN[4];
            let CQI = CLN[5];
            let CQJ = CLN[6];
            let CQK = CLN[7];
            let CQL = CLN[8];
            let CQM = CLP[0];
            let CQN = CLP[1];
            let CQO = CLP[2];
            let CQP = CLP[3];
            let CQQ = CLP[4];
            let CQR = CLP[5];
            let CQS = CLR[0];
            let CQT = CLR[1];
            let CQU = CLR[2];
            let CQV = CLR[3];
            let CQW = CLR[4];
            let CQX = CLR[5];
            let CQY = BNJ[0];
            let CQZ = BNJ[1];
            let CRA = BNJ[2];
            let CRB = BNK[0];
            let CRC = BNK[1];
            let CRD = BNK[2];
            let CRE = BIH;
            let CRF = CMG[0];
            let CRG = CMG[1];
            let CRH = CMG[2];
            let CRI = CMG[3];
            let CRJ = CMG[4];
            let CRK = CMG[5];
            let CRL = CMK[0];
            let CRM = CMK[1];
            let CRN = CMK[2];
            let CRO = CMK[3];
            let CRP = CMK[4];
            let CRQ = CMK[5];
            let CRR = CKL;
            let CRS = CKT[0];
            let CRT = CKT[1];
            let CRU = CKT[2];
            let CRV = CKT[3];
            let CRW = CKT[4];
            let CRX = CKW[0];
            let CRY = CKW[1];
            let CRZ = CKW[2];
            let CSA = CKZ[0];
            let CSB = CKZ[1];
            let CSC = CKZ[2];
            let CSD = CKZ[3];
            let CSE = CKZ[4];
            let CSF = CLC[0];
            let CSG = CLC[1];
            let CSH = CLC[2];
            let CSI = CLC[3];
            let CSJ = CLC[4];
            let CSK = CLC[5];
            let CSL = CLF[0];
            let CSM = CLF[1];
            let CSN = CLI[0];
            let CSO = CLI[1];
            let CSP = CLO[0];
            let CSQ = CLO[1];
            let CSR = CLO[2];
            let CSS = CLO[3];
            let CST = CLO[4];
            let CSU = CLO[5];
            let CSV = CLO[6];
            let CSW = CLO[7];
            let CSX = CLO[8];
            let CSY = CLS[0];
            let CSZ = CLS[1];
            let CTA = CLS[2];
            let CTB = CLS[3];
            let CTC = CLS[4];
            let CTD = CLS[5];
            let CTE = CMI[0];
            let CTF = CMI[1];
            let CTG = CMI[2];
            let CTH = CMI[3];
            let CTI = CMI[4];
            let CTJ = CMI[5];
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(7),
            Some(8),
            multiplicity * (BBV),
            [3, 6, 7, 8],
            [CML, CMM, CMN, CMO],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(8),
            Some(4),
            multiplicity * (BBW),
            [3, 4, 6, 7, 8],
            [CMP, CMQ, CMR, CMS, CMT],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(5),
            Some(4),
            multiplicity * (BBX),
            [3, 4, 5],
            [CMU, CMV, CMW],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(6),
            Some(4),
            multiplicity * (BBY),
            [3, 4, 6, 7, 8],
            [CMX, CMY, CMZ, CNA, CNB],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(6),
            Some(7),
            multiplicity * (BGV),
            [3, 5, 6, 7],
            [CNC, CND, CNE, CNF],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(6),
            Some(8),
            multiplicity * (BGW),
            [3, 5, 6, 7],
            [CNG, CNH, CNI, CNJ],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(5),
            Some(6),
            multiplicity * (BCB),
            [3, 4, 5, 6, 7, 8],
            [CNK, CNL, CNM, CNN, CNO, CNP],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(6),
            Some(8),
            multiplicity * (BCD),
            [3, 4, 6, 7, 8],
            [CNQ, CNR, CNS, CNT, CNU],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(2),
            Some(4),
            multiplicity * (BCF),
            [2, 3, 4],
            [CNV, CNW, CNX],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(1),
            Some(5),
            multiplicity * (BCH),
            [1, 3, 5],
            [CNY, CNZ, COA],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(3),
            None,
            multiplicity * (BCX),
            [3],
            [COB],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(3),
            None,
            multiplicity * (BCK),
            [3],
            [COC],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<11, 0>(
            Some(3),
            None,
            multiplicity * (BDB),
            [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
            [COD, COE, COF, COG, COH, COI, COJ, COK, COL, COM, CON],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(6),
            Some(4),
            multiplicity * (BDF),
            [3, 4, 6, 7, 8],
            [COO, COP, COQ, COR, COS],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(5),
            Some(4),
            multiplicity * (BDH),
            [3, 4, 5],
            [COT, COU, COV],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(6),
            Some(8),
            multiplicity * (BDK),
            [3, 4, 6, 7, 8],
            [COW, COX, COY, COZ, CPA],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(5),
            Some(6),
            multiplicity * (BDN),
            [3, 4, 5, 6, 7, 8],
            [CPB, CPC, CPD, CPE, CPF, CPG],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(1),
            Some(2),
            multiplicity * (BDQ),
            [1, 2],
            [CPH, CPI],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(1),
            Some(0),
            multiplicity * (BDT),
            [0, 1],
            [CPJ, CPK],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(1),
            Some(9),
            multiplicity * (BDU),
            [0, 1, 3, 5, 6, 7, 8, 9, 10],
            [CPL, CPM, CPN, CPO, CPP, CPQ, CPR, CPS, CPT],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(0),
            Some(9),
            multiplicity * (BDW),
            [0, 1, 3, 5, 6, 7, 8, 9, 10],
            [CPU, CPV, CPW, CPX, CPY, CPZ, CQA, CQB, CQC],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(1),
            Some(9),
            multiplicity * (BDZ),
            [0, 1, 3, 5, 6, 7, 8, 9, 10],
            [CQD, CQE, CQF, CQG, CQH, CQI, CQJ, CQK, CQL],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(5),
            Some(10),
            multiplicity * (BEA),
            [3, 5, 6, 7, 8, 10],
            [CQM, CQN, CQO, CQP, CQQ, CQR],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(5),
            Some(10),
            multiplicity * (BED),
            [3, 5, 6, 7, 8, 10],
            [CQS, CQT, CQU, CQV, CQW, CQX],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(9),
            Some(10),
            multiplicity * (BGX),
            [3, 9, 10],
            [CQY, CQZ, CRA],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(9), Some(10), 0, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            0,
            BGY,
            [],
            [],
            [],
            [],
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(10),
            Some(7),
            multiplicity * (BGZ),
            [3, 7, 10],
            [CRB, CRC, CRD],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(10), Some(7), 1, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            1,
            BHA,
            [],
            [],
            [],
            [],
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(11),
            None,
            multiplicity * (BFG),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(11),
            None,
            multiplicity * (BFH),
            [11],
            [CRE],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(6),
            Some(4),
            multiplicity * (BFL),
            [3, 4, 6, 7, 8, 11],
            [CRF, CRG, CRH, CRI, CRJ, CRK],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(8),
            Some(6),
            multiplicity * (BFM),
            [3, 4, 6, 7, 8, 11],
            [CRL, CRM, CRN, CRO, CRP, CRQ],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(8),
            Some(4),
            multiplicity * (BFH),
            [11],
            [CRE],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(8),
            Some(6),
            multiplicity * (BFN),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(6),
            Some(4),
            multiplicity * (BFO),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(2),
            Some(4),
            multiplicity * (BFP),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(1),
            Some(5),
            multiplicity * (BFQ),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(5),
            Some(6),
            multiplicity * (BFR),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(6),
            Some(4),
            multiplicity * (BFS),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(5),
            Some(4),
            multiplicity * (BFT),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(5),
            Some(4),
            multiplicity * (BFU),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(5),
            Some(10),
            multiplicity * (BFV),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(5),
            Some(10),
            multiplicity * (BFW),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(5),
            Some(10),
            multiplicity * (BFX),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(5),
            Some(10),
            multiplicity * (BFY),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(1),
            Some(9),
            multiplicity * (BFZ),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(1),
            Some(9),
            multiplicity * (BGA),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(7),
            Some(6),
            multiplicity * (BHB),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(8),
            Some(6),
            multiplicity * (BHC),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(0),
            Some(9),
            multiplicity * (BGL),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(9),
            Some(10),
            multiplicity * (BHD),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(10),
            Some(7),
            multiplicity * (BHF),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(0),
            Some(9),
            multiplicity * (BGN),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(9),
            Some(7),
            multiplicity * (BHH),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(0),
            Some(10),
            multiplicity * (BGP),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(10),
            Some(7),
            multiplicity * (BHJ),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(0),
            Some(7),
            multiplicity * (BGR),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        self.canonical_reactive[0] = BBV;
        self.canonical_reactive[1] = BBW;
        self.canonical_reactive[2] = BBX;
        self.canonical_reactive[3] = BBY;
        self.canonical_reactive[4] = BGV;
        self.canonical_reactive[5] = BGW;
        self.canonical_reactive[6] = BCB;
        self.canonical_reactive[7] = BCD;
        self.canonical_reactive[8] = BCF;
        self.canonical_reactive[9] = BCH;
        self.canonical_reactive[10] = BCX;
        self.canonical_reactive[11] = BHL;
        self.canonical_reactive[12] = CRR;
        self.canonical_reactive[13] = BDB;
        self.canonical_reactive[14] = BHM;
        self.canonical_reactive[15] = CRS;
        self.canonical_reactive[16] = CRT;
        self.canonical_reactive[17] = CRU;
        self.canonical_reactive[18] = CRV;
        self.canonical_reactive[19] = CRW;
        self.canonical_reactive[20] = BHN;
        self.canonical_reactive[21] = CRX;
        self.canonical_reactive[22] = CRY;
        self.canonical_reactive[23] = CRZ;
        self.canonical_reactive[24] = BHO;
        self.canonical_reactive[25] = CSA;
        self.canonical_reactive[26] = CSB;
        self.canonical_reactive[27] = CSC;
        self.canonical_reactive[28] = CSD;
        self.canonical_reactive[29] = CSE;
        self.canonical_reactive[30] = BHP;
        self.canonical_reactive[31] = CSF;
        self.canonical_reactive[32] = CSG;
        self.canonical_reactive[33] = CSH;
        self.canonical_reactive[34] = CSI;
        self.canonical_reactive[35] = CSJ;
        self.canonical_reactive[36] = CSK;
        self.canonical_reactive[37] = BHQ;
        self.canonical_reactive[38] = CSL;
        self.canonical_reactive[39] = CSM;
        self.canonical_reactive[40] = BHR;
        self.canonical_reactive[41] = CSN;
        self.canonical_reactive[42] = CSO;
        self.canonical_reactive[43] = BDU;
        self.canonical_reactive[44] = BDW;
        self.canonical_reactive[45] = BHS;
        self.canonical_reactive[46] = CSP;
        self.canonical_reactive[47] = CSQ;
        self.canonical_reactive[48] = CSR;
        self.canonical_reactive[49] = CSS;
        self.canonical_reactive[50] = CST;
        self.canonical_reactive[51] = CSU;
        self.canonical_reactive[52] = CSV;
        self.canonical_reactive[53] = CSW;
        self.canonical_reactive[54] = CSX;
        self.canonical_reactive[55] = BEA;
        self.canonical_reactive[56] = BHT;
        self.canonical_reactive[57] = CSY;
        self.canonical_reactive[58] = CSZ;
        self.canonical_reactive[59] = CTA;
        self.canonical_reactive[60] = CTB;
        self.canonical_reactive[61] = CTC;
        self.canonical_reactive[62] = CTD;
        self.canonical_reactive[63] = BGX;
        self.canonical_reactive[64] = BGY;
        self.canonical_reactive[65] = BGZ;
        self.canonical_reactive[66] = BHA;
        self.canonical_reactive[67] = BFG;
        self.canonical_reactive[68] = BFH;
        self.canonical_reactive[69] = BHU;
        self.canonical_reactive[70] = CTE;
        self.canonical_reactive[71] = CTF;
        self.canonical_reactive[72] = CTG;
        self.canonical_reactive[73] = CTH;
        self.canonical_reactive[74] = CTI;
        self.canonical_reactive[75] = CTJ;
        self.canonical_reactive[76] = BFM;
        self.canonical_reactive[77] = BFH;
        self.canonical_reactive[78] = BFN;
        self.canonical_reactive[79] = BFO;
        self.canonical_reactive[80] = BFP;
        self.canonical_reactive[81] = BFQ;
        self.canonical_reactive[82] = BFR;
        self.canonical_reactive[83] = BFS;
        self.canonical_reactive[84] = BFT;
        self.canonical_reactive[85] = BFU;
        self.canonical_reactive[86] = BFV;
        self.canonical_reactive[87] = BFW;
        self.canonical_reactive[88] = BFX;
        self.canonical_reactive[89] = BFY;
        self.canonical_reactive[90] = BFZ;
        self.canonical_reactive[91] = BGA;
        self.canonical_reactive[92] = BHB;
        self.canonical_reactive[93] = BHC;
        self.canonical_reactive[94] = BGL;
        self.canonical_reactive[95] = BHD;
        self.canonical_reactive[96] = BHF;
        self.canonical_reactive[97] = BGN;
        self.canonical_reactive[98] = BHH;
        self.canonical_reactive[99] = BGP;
        self.canonical_reactive[100] = BHJ;
        self.canonical_reactive[101] = BGR;
    }

    pub fn stamp_reactive(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedReactiveStamper<'_>) {
        let multiplicity = self.multiplicity;
        let cached = &*self.canonical_reactive;
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(3),
            None,
            &[3],
            &[cached[12]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(6),
            Some(4),
            &[3, 4, 6, 7, 8],
            &[cached[15], cached[16], cached[17], cached[18], cached[19]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(5),
            Some(4),
            &[3, 4, 5],
            &[cached[21], cached[22], cached[23]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(6),
            Some(8),
            &[3, 4, 6, 7, 8],
            &[cached[25], cached[26], cached[27], cached[28], cached[29]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(5),
            Some(6),
            &[3, 4, 5, 6, 7, 8],
            &[cached[31], cached[32], cached[33], cached[34], cached[35], cached[36]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(1),
            Some(2),
            &[1, 2],
            &[cached[38], cached[39]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(1),
            Some(0),
            &[0, 1],
            &[cached[41], cached[42]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(1),
            Some(9),
            &[0, 1, 3, 5, 6, 7, 8, 9, 10],
            &[cached[46], cached[47], cached[48], cached[49], cached[50], cached[51], cached[52], cached[53], cached[54]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(5),
            Some(10),
            &[3, 5, 6, 7, 8, 10],
            &[cached[57], cached[58], cached[59], cached[60], cached[61], cached[62]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(6),
            Some(4),
            &[3, 4, 6, 7, 8, 11],
            &[cached[70], cached[71], cached[72], cached[73], cached[74], cached[75]],
            &[],
            &[],
            multiplicity,
        );
    }

}
