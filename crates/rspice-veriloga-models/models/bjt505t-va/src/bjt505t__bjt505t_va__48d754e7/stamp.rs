#![allow(dead_code, non_snake_case, unused_imports, unused_mut, unused_parens, unused_variables)]

use super::state::Instance;
use rspice_veriloga_runtime::{GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, Lanes, rspice_eval_ddt, rspice_eval_idt, rspice_limexp, rspice_limited_exp, rspice_limited_exp_derivative};
impl Instance {
    pub fn stamp(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedStamper<'_>) {
        let parameters = &self.params.values;
        let multiplicity = self.multiplicity;
        let temperature = ctx.temperature();
        let node_potentials = [ctx.node_voltage(self.nodes[0]), ctx.node_voltage(self.nodes[1]), ctx.node_voltage(self.nodes[2]), ctx.node_voltage(self.nodes[3]), ctx.node_voltage(self.nodes[4]), ctx.node_voltage(self.nodes[5]), ctx.node_voltage(self.nodes[6]), ctx.node_voltage(self.nodes[7]), ctx.node_voltage(self.nodes[8]), ctx.node_voltage(self.nodes[9]), ctx.node_voltage(self.nodes[10]), ctx.node_voltage(self.nodes[11]), ctx.node_voltage(self.nodes[12])];
        let ddt_scale_value = self.ddt_coefficients.derivative_scale;
        let ddt_scale = move || ddt_scale_value;
        let ddt_state = self.stamp_state.as_mut();
        let ddt_active = self.ddt_coefficients.active;
        let ddt_coefficients = self.ddt_coefficients;
        let mut ddt = |operator: usize, value: f64| -> f64 {
            let _ = operator;
            let slot = match operator { 13541 => 0usize, 13609 => 1usize, 13615 => 2usize, 13625 => 3usize, 13631 => 4usize, 13637 => 5usize, 13645 => 6usize, 13653 => 7usize, 13673 => 8usize, 13692 => 9usize, 13963 => 10usize, _ => usize::MAX };
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
            let I = parameters[33];
            let N = parameters[154];
            let P = 1e-12f64;
            let R = parameters[1];
            let U = parameters[134];
            let W = 1e-3f64;
            let X = 2e0f64;
            let Y = parameters[67];
            let AB = parameters[114];
            let AC = parameters[115];
            let AD = parameters[116];
            let AF = 5e-2f64;
            let AG = 1e-1f64;
            let AM = parameters[66];
            let AO = parameters[71];
            let AP = parameters[72];
            let AS = parameters[117];
            let AT = parameters[118];
            let AU = parameters[119];
            let BD = node_potentials[4];
            let BI = parameters[125];
            let BP = 8.617086918058125e-5f64;
            let CV = 3e0f64;
            let CW = -3e0f64;
            let CZ = parameters[105];
            let DL = -3e0f64;
            let DN = parameters[64];
            let DO = parameters[110];
            let EB = -3e0f64;
            let ED = parameters[80];
            let EP = -3e0f64;
            let FD = -3e0f64;
            let FQ = -3e0f64;
            let FS = parameters[27];
            let FT = parameters[109];
            let GF = -3e0f64;
            let GH = parameters[138];
            let GI = parameters[140];
            let HC = parameters[65];
            let HE = parameters[137];
            let HH = parameters[139];
            let HJ = parameters[75];
            let HP = parameters[70];
            let HS = parameters[54];
            let HT = parameters[97];
            let HX = parameters[56];
            let HY = parameters[98];
            let HZ = parameters[96];
            let ID = parameters[55];
            let IE = parameters[101];
            let II = parameters[57];
            let IJ = parameters[102];
            let IM = parameters[58];
            let IN = parameters[104];
            let IQ = parameters[59];
            let IS = parameters[60];
            let IT = parameters[99];
            let IW = parameters[122];
            let IY = parameters[10];
            let JK = parameters[123];
            let JM = parameters[11];
            let JY = parameters[43];
            let JZ = parameters[124];
            let KB = 1e-6f64;
            let KE = 5e-1f64;
            let KK = parameters[9];
            let KL = 4e0f64;
            let KM = parameters[121];
            let KW = parameters[12];
            let LA = parameters[30];
            let LB = parameters[103];
            let LF = parameters[20];
            let LG = 6e0f64;
            let LH = parameters[21];
            let LP = parameters[31];
            let LQ = parameters[32];
            let LX = parameters[16];
            let MA = parameters[17];
            let MH = parameters[18];
            let MI = parameters[19];
            let MO = parameters[25];
            let MS = parameters[28];
            let MW = parameters[26];
            let NA = parameters[29];
            let NH = parameters[22];
            let NI = parameters[23];
            let NO = parameters[149];
            let NP = parameters[150];
            let NV = parameters[155];
            let NY = parameters[157];
            let OD = -5e-1f64;
            let OG = parameters[35];
            let OM = parameters[34];
            let OV = -5e-1f64;
            let OY = parameters[37];
            let PE = parameters[36];
            let PM = parameters[14];
            let PP = parameters[13];
            let PS = parameters[133];
            let PT = parameters[141];
            let QA = parameters[142];
            let QF = parameters[135];
            let QJ = parameters[136];
            let QN = parameters[86];
            let QU = parameters[87];
            let QY = parameters[88];
            let RC = parameters[89];
            let RG = parameters[90];
            let RN = 7.2e-4f64;
            let RO = 1.6e-6f64;
            let RS = parameters[92];
            let RU = parameters[146];
            let RV = parameters[148];
            let SG = node_potentials[7];
            let SH = node_potentials[8];
            let SJ = node_potentials[9];
            let SL = node_potentials[5];
            let SN = node_potentials[6];
            let SS = node_potentials[2];
            let SU = node_potentials[1];
            let SY = node_potentials[11];
            let TH = parameters[151];
            let WB = parameters[153];
            let WJ = 1e2f64;
            let WU = 2e-1f64;
            let XF = parameters[62];
            let XG = parameters[61];
            let XM = parameters[63];
            let YX = parameters[152];
            let AAW = parameters[74];
            let ABV = parameters[76];
            let ADK = 1.0000000000000002e-2f64;
            let ADW = parameters[15];
            let AEB = 1e-4f64;
            let AEL = parameters[156];
            let AEU = parameters[158];
            let AFD = parameters[159];
            let AFX = 4e1f64;
            let AGA = 2.3538526683702e17f64;
            let AGT = parameters[93];
            let AJQ = 1e-30f64;
            let AJZ = 1.6666666666666666e-1f64;
            let AKQ = 3.333333333333333e-1f64;
            let AKS = 2.5e-1f64;
            let ANP = parameters[143];
            let ANW = parameters[144];
            let APG = parameters[5];
            let AQO = 1.21e-2f64;
            let ARG = 1e-6f64;
            let ARH = 1e-12f64;
            let ARI = -1e0f64;
            let ARJ = -1e0f64;
            let ARO = -1e0f64;
            let ARS = -1e0f64;
            let ARU = parameters[82];
            let ARW = parameters[81];
            let ASR = 1.0000000000000002e-2f64;
            let ATK = parameters[39];
            let ATM = parameters[44];
            let ATO = parameters[42];
            let ATZ = parameters[41];
            let AUG = parameters[40];
            let AUN = parameters[45];
            let AUS = parameters[7];
            let AVG = parameters[47];
            let AWQ = parameters[48];
            let AWU = parameters[49];
            let AWZ = parameters[51];
            let AXJ = parameters[50];
            let BAN = parameters[68];
            let BBH = parameters[77];
            let BDZ = parameters[85];
            let BEX = parameters[91];
            let BHJ = parameters[95];
            let BHN = parameters[94];
            let BIC = -1e0f64;
            let BII = parameters[147];
            let BJA = -1e0f64;
            let BKI = 0e0f64;
            let BKL = 0e0f64;
            let BKV = parameters[131];
            let BLA = parameters[132];
            let BLI = 0e0f64;
            let BLJ = node_potentials[12];
            let BLP = 0e0f64;
            let BLQ = 0e0f64;
            let BLR = 0e0f64;
            let BLS = 0e0f64;
            let BLT = 0e0f64;
            let BLU = 0e0f64;
            let BLV = 0e0f64;
            let BLW = 0e0f64;
            let BLX = 0e0f64;
            let BLY = 0e0f64;
            let BLZ = 0e0f64;
            let BMA = 0e0f64;
            let BMB = 0e0f64;
            let BMC = 0e0f64;
            let BMD = 0e0f64;
            let BME = 0e0f64;
            let BMF = 0e0f64;
            let BMG = 0e0f64;
            let BMH = 0e0f64;
            let BMI = 0e0f64;
            let BMJ = 0e0f64;
            let BMK = 0e0f64;
            let BML = 0e0f64;
            let BMM = 0e0f64;
            let BMN = 0e0f64;
            let BMO = 0e0f64;
            let BMP = 0e0f64;
            let BOB = 1e0f64;
            let BOC = 1e0f64;
            let BOD = 1e0f64;
            let BOE = 1e0f64;
            let BOF = 1e0f64;
            let BOG = 1e0f64;
            let BOH = 1e0f64;
            let BOI = 1e0f64;
            let BOJ = 1e0f64;
            let BOK = 1e0f64;
            let BOL = 1e0f64;
            let BOM = 1e0f64;
            let BON = 1e0f64;
            let BOO = 1e0f64;
            let BUG = -1e0f64;
            let BWM = 0e0f64;
            let BXF = 2e0f64;
            let CBW = Lanes([0e0f64; 3]);
            let CCK = Lanes([0e0f64; 4]);
            let CIB = Lanes([0e0f64; 3]);
            let CKC = Lanes([0e0f64; 10]);
            let CKW = Lanes([0e0f64; 3]);
            let CLU = Lanes([0e0f64; 5]);
            let CSJ = Lanes([0e0f64; 6]);
            let CTE = Lanes([0e0f64; 4]);
            let CTR = ddt_scale();
            let CVE = Lanes([0e0f64; 3]);
            let CVH = Lanes([0e0f64; 3]);
            let D = if B == C { 1.0 } else { 0.0 };
            let RM;
            let AVW;
            if D != 0.0 {
                RM = F;
                AVW = E;
            } else {
                RM = H;
                AVW = G;
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
            let V = if U > A { 1.0 } else { 0.0 };
            let APE = if V != 0.0 {
                M
            } else {
                A
            };
            let Z = X.powf((X - Y));
            let AA = C / Z;
            let AE = AB + (((AC * K) * K) / (K + AD));
            let AH = (AE - AF) / AG;
            let AI = if AE < AF { 1.0 } else { 0.0 };
            let BV = if AI != 0.0 {
                let AJ = AF + (AG * ((C + (AH.exp())).ln()));
                AJ
            } else {
                let AK = AE + (AG * ((C + ((-AH).exp())).ln()));
                AK
            };
            let AL = C / AB;
            let AN = C / AM;
            let AQ = X.powf((X - AP));
            let AR = C / AQ;
            let AV = AS + (((AT * K) * K) / (K + AU));
            let AW = (AV - AF) / AG;
            let AX = if AV < AF { 1.0 } else { 0.0 };
            let CI = if AX != 0.0 {
                let AY = AF + (AG * ((C + (AW.exp())).ln()));
                AY
            } else {
                let AZ = AV + (AG * ((C + ((-AW).exp())).ln()));
                AZ
            };
            let BA = C / AS;
            let BB = C / AO;
            let BC = C - (C / parameters[83]);
            let BE = if BD < A { 1.0 } else { 0.0 };
            let BH;
            let BOP;
            if BE != 0.0 {
                let BF = C - BD;
                let BG = -(BF.ln());
                let BUH = ((BOC * BUG) * (BOB / BF)) * BUG;
                BH = BG;
                BOP = BUH;
            } else {
                BH = BD;
                BOP = BOC;
            }
            let BJ = if BH < BI { 1.0 } else { 0.0 };
            let BM;
            let BOQ;
            if BJ != 0.0 {
                BM = BH;
                BOQ = BOP;
            } else {
                let BK = C + (BH - BI);
                let BUI = BOP * (BOB / BK);
                let BL = BI + (BK.ln());
                BM = BL;
                BOQ = BUI;
            }
            let BN = L + BM;
            let BO = BN / K;
            let BUJ = BOQ / K;
            let BQ = BP * BN;
            let BUK = BOQ * BP;
            let BR = C / BQ;
            let BUL = ((BUK * BR) * BUG) / BQ;
            let BS = BR - (C / (BP * K));
            let BT = BN - K;
            let BU = BO.ln();
            let BUM = BUJ * (BOB / BO);
            let BW = AC * BN;
            let BX = BN + AD;
            let BY = (BW * BN) / BX;
            let BZ = BV - BY;
            let BUN = (((((BOQ * AC) * BN) + (BOQ * BW)) - (BOQ * BY)) / BX) * BUG;
            let CA = (BZ - AF) / AG;
            let BUO = BUN / AG;
            let CB = if BZ < AF { 1.0 } else { 0.0 };
            let OB;
            let BOR;
            if CB != 0.0 {
                let CC = CA.exp();
                let CD = C + CC;
                let BUQ = ((BUO * CC) * (BOB / CD)) * AG;
                let CE = AF + (AG * (CD.ln()));
                OB = CE;
                BOR = BUQ;
            } else {
                let CF = (-CA).exp();
                let CG = C + CF;
                let CH = BZ + (AG * (CG.ln()));
                let BUP = BUN + ((((BUO * BUG) * CF) * (BOB / CG)) * AG);
                OB = CH;
                BOR = BUP;
            }
            let CJ = AT * BN;
            let CK = BN + AU;
            let CL = (CJ * BN) / CK;
            let CM = CI - CL;
            let BUR = (((((BOQ * AT) * BN) + (BOQ * CJ)) - (BOQ * CL)) / CK) * BUG;
            let CN = (CM - AF) / AG;
            let BUS = BUR / AG;
            let CO = if CM < AF { 1.0 } else { 0.0 };
            let OT;
            let BOS;
            if CO != 0.0 {
                let CP = CN.exp();
                let CQ = C + CP;
                let BUU = ((BUS * CP) * (BOB / CQ)) * AG;
                let CR = AF + (AG * (CQ.ln()));
                OT = CR;
                BOS = BUU;
            } else {
                let CS = (-CN).exp();
                let CT = C + CS;
                let CU = CM + (AG * (CT.ln()));
                let BUT = BUR + ((((BUS * BUG) * CS) * (BOB / CT)) * AG);
                OT = CU;
                BOS = BUT;
            }
            let CX = CW * BQ;
            let CY = C - BO;
            let BUV = BUJ * BUG;
            let DA = ((CX * BU) + (AM * BO)) + (CY * CZ);
            let BUW = ((((BUK * CW) * BU) + (BUM * CX)) + (BUJ * AM)) + (BUV * CZ);
            let DB = (AF - DA) / BQ;
            let BUX = ((BUW * BUG) - (BUK * DB)) / BQ;
            let DC = if AF < DA { 1.0 } else { 0.0 };
            let GU;
            let BOT;
            if DC != 0.0 {
                let DD = DB.exp();
                let DE = C + DD;
                let DF = DE.ln();
                let DG = DA + (BQ * DF);
                let BUZ = BUW + ((BUK * DF) + (((BUX * DD) * (BOB / DE)) * BQ));
                GU = DG;
                BOT = BUZ;
            } else {
                let DH = (-DB).exp();
                let DI = C + DH;
                let DJ = DI.ln();
                let BUY = (BUK * DJ) + ((((BUX * BUG) * DH) * (BOB / DI)) * BQ);
                let DK = AF + (BQ * DJ);
                GU = DK;
                BOT = BUY;
            }
            let DM = DL * BQ;
            let DP = CY * DO;
            let BVA = BUV * DO;
            let DQ = ((DM * BU) + (DN * BO)) + DP;
            let BVB = ((((BUK * DL) * BU) + (BUM * DM)) + (BUJ * DN)) + BVA;
            let DR = (AF - DQ) / BQ;
            let BVC = ((BVB * BUG) - (BUK * DR)) / BQ;
            let DS = if AF < DQ { 1.0 } else { 0.0 };
            let UW;
            let BOU;
            if DS != 0.0 {
                let DT = DR.exp();
                let DU = C + DT;
                let DV = DU.ln();
                let DW = DQ + (BQ * DV);
                let BVE = BVB + ((BUK * DV) + (((BVC * DT) * (BOB / DU)) * BQ));
                UW = DW;
                BOU = BVE;
            } else {
                let DX = (-DR).exp();
                let DY = C + DX;
                let DZ = DY.ln();
                let BVD = (BUK * DZ) + ((((BVC * BUG) * DX) * (BOB / DY)) * BQ);
                let EA = AF + (BQ * DZ);
                UW = EA;
                BOU = BVD;
            }
            let EC = EB * BQ;
            let EE = ((EC * BU) + (ED * BO)) + DP;
            let BVF = ((((BUK * EB) * BU) + (BUM * EC)) + (BUJ * ED)) + BVA;
            let EF = (AF - EE) / BQ;
            let BVG = ((BVF * BUG) - (BUK * EF)) / BQ;
            let EG = if AF < EE { 1.0 } else { 0.0 };
            let BEW;
            let BOV;
            if EG != 0.0 {
                let EH = EF.exp();
                let EI = C + EH;
                let EJ = EI.ln();
                let EK = EE + (BQ * EJ);
                let BVI = BVF + ((BUK * EJ) + (((BVG * EH) * (BOB / EI)) * BQ));
                BEW = EK;
                BOV = BVI;
            } else {
                let EL = (-EF).exp();
                let EM = C + EL;
                let EN = EM.ln();
                let BVH = (BUK * EN) + ((((BVG * BUG) * EL) * (BOB / EM)) * BQ);
                let EO = AF + (BQ * EN);
                BEW = EO;
                BOV = BVH;
            }
            let EQ = EP * BQ;
            let ER = AO * BO;
            let BVJ = BUJ * AO;
            let ES = ((EQ * BU) + ER) + DP;
            let BVK = ((((BUK * EP) * BU) + (BUM * EQ)) + BVJ) + BVA;
            let ET = (AF - ES) / BQ;
            let BVL = ((BVK * BUG) - (BUK * ET)) / BQ;
            let EU = if AF < ES { 1.0 } else { 0.0 };
            let HL;
            let BOW;
            if EU != 0.0 {
                let EV = ET.exp();
                let EW = C + EV;
                let EX = EW.ln();
                let EY = ES + (BQ * EX);
                let BVN = BVK + ((BUK * EX) + (((BVL * EV) * (BOB / EW)) * BQ));
                HL = EY;
                BOW = BVN;
            } else {
                let EZ = (-ET).exp();
                let FA = C + EZ;
                let FB = FA.ln();
                let BVM = (BUK * FB) + ((((BVL * BUG) * EZ) * (BOB / FA)) * BQ);
                let FC = AF + (BQ * FB);
                HL = FC;
                BOW = BVM;
            }
            let FE = FD * BQ;
            let FF = ((FE * BU) + ER) + DP;
            let BVO = ((((BUK * FD) * BU) + (BUM * FE)) + BVJ) + BVA;
            let FG = (AF - FF) / BQ;
            let BVP = ((BVO * BUG) - (BUK * FG)) / BQ;
            let FH = if AF < FF { 1.0 } else { 0.0 };
            let GW;
            let BOX;
            if FH != 0.0 {
                let FI = FG.exp();
                let FJ = C + FI;
                let FK = FJ.ln();
                let FL = FF + (BQ * FK);
                let BVR = BVO + ((BUK * FK) + (((BVP * FI) * (BOB / FJ)) * BQ));
                GW = FL;
                BOX = BVR;
            } else {
                let FM = (-FG).exp();
                let FN = C + FM;
                let FO = FN.ln();
                let BVQ = (BUK * FO) + ((((BVP * BUG) * FM) * (BOB / FN)) * BQ);
                let FP = AF + (BQ * FO);
                GW = FP;
                BOX = BVQ;
            }
            let FR = FQ * BQ;
            let FU = ((FR * BU) + (FS * BO)) + (CY * FT);
            let BVS = ((((BUK * FQ) * BU) + (BUM * FR)) + (BUJ * FS)) + (BUV * FT);
            let FV = (AF - FU) / BQ;
            let BVT = ((BVS * BUG) - (BUK * FV)) / BQ;
            let FW = if AF < FU { 1.0 } else { 0.0 };
            let AFO;
            let BOY;
            if FW != 0.0 {
                let FX = FV.exp();
                let FY = C + FX;
                let FZ = FY.ln();
                let GA = FU + (BQ * FZ);
                let BVV = BVS + ((BUK * FZ) + (((BVT * FX) * (BOB / FY)) * BQ));
                AFO = GA;
                BOY = BVV;
            } else {
                let GB = (-FV).exp();
                let GC = C + GB;
                let GD = GC.ln();
                let BVU = (BUK * GD) + ((((BVT * BUG) * GB) * (BOB / GC)) * BQ);
                let GE = AF + (BQ * GD);
                AFO = GE;
                BOY = BVU;
            }
            let GG = GF * BQ;
            let GJ = ((GG * BU) + (GH * BO)) + (CY * GI);
            let BVW = ((((BUK * GF) * BU) + (BUM * GG)) + (BUJ * GH)) + (BUV * GI);
            let GK = (AF - GJ) / BQ;
            let BVX = ((BVW * BUG) - (BUK * GK)) / BQ;
            let GL = if AF < GJ { 1.0 } else { 0.0 };
            let HF;
            let BOZ;
            if GL != 0.0 {
                let GM = GK.exp();
                let GN = C + GM;
                let GO = GN.ln();
                let GP = GJ + (BQ * GO);
                let BVZ = BVW + ((BUK * GO) + (((BVX * GM) * (BOB / GN)) * BQ));
                HF = GP;
                BOZ = BVZ;
            } else {
                let GQ = (-GK).exp();
                let GR = C + GQ;
                let GS = GR.ln();
                let BVY = (BUK * GS) + ((((BVX * BUG) * GQ) * (BOB / GR)) * BQ);
                let GT = AF + (BQ * GS);
                HF = GT;
                BOZ = BVY;
            }
            let GV = C / GU;
            let BWA = ((BOT * GV) * BUG) / GU;
            let GX = C / GW;
            let BWB = ((BOX * GX) * BUG) / GW;
            let GY = AM * GV;
            let GZ = GY.powf(Y);
            let BWC = (BWA * AM) * (Y * (GY.powf((Y - BOB))));
            let HA = AO * GX;
            let HB = HA.powf(AP);
            let BWD = AP - BOB;
            let BWE = (BWB * AO) * (AP * (HA.powf(BWD)));
            let HD = HC * GZ;
            let BWF = BWC * HC;
            let HG = GH / HF;
            let HI = HE * (HG.powf(HH));
            let BWG = ((((BOZ * HG) * BUG) / HF) * (HH * (HG.powf((HH - BOB))))) * HE;
            let HK = C - HJ;
            let HM = AO / HL;
            let BWH = ((((BOW * HM) * BUG) / HL) * (AP * (HM.powf(BWD)))) * HK;
            let HN = (HK * (HM.powf(AP))) + HJ;
            let HO = C / HN;
            let BWI = ((BWH * HO) * BUG) / HN;
            let HQ = HP * HN;
            let BWJ = BWH * HP;
            let HR = HJ * HO;
            let BWK = BWI * HJ;
            let HU = (BU * HT).exp();
            let HV = HS * HU;
            let BWL = ((BUM * HT) * HU) * HS;
            let HW = if HV < S { 1.0 } else { 0.0 };
            let AYL;
            let BPA;
            if HW != 0.0 {
                AYL = S;
                BPA = BWM;
            } else {
                AYL = HV;
                BPA = BWL;
            }
            let IA = HY - HZ;
            let IB = (BU * IA).exp();
            let IC = HX * IB;
            let BWN = ((BUM * IA) * IB) * HX;
            let IF = (BU * IE).exp();
            let IG = ID * IF;
            let BWO = ((BUM * IE) * IF) * ID;
            let IH = if IG < S { 1.0 } else { 0.0 };
            let AYG;
            let BPB;
            if IH != 0.0 {
                AYG = S;
                BPB = BWM;
            } else {
                AYG = IG;
                BPB = BWO;
            }
            let IK = (BU * IJ).exp();
            let IL = II * IK;
            let BWP = ((BUM * IJ) * IK) * II;
            let IO = (BU * IN).exp();
            let BWQ = (BUM * IN) * IO;
            let IP = IM * IO;
            let BWR = BWQ * IM;
            let IR = IQ * IO;
            let BWS = BWQ * IQ;
            let IU = (BU * IT).exp();
            let IV = IS * IU;
            let BWT = ((BUM * IT) * IU) * IS;
            let IX = if IW != A { 1.0 } else { 0.0 };
            let KO;
            let BPC;
            if IX != 0.0 {
                let IZ = IY * (C + (BT * IW));
                let BWU = (BOQ * IW) * IY;
                let JA = (IZ - C) / W;
                let BWV = BWU / W;
                let JB = if IZ < C { 1.0 } else { 0.0 };
                let JI;
                let BPD;
                if JB != 0.0 {
                    let JC = JA.exp();
                    let JD = C + JC;
                    let BWX = ((BWV * JC) * (BOB / JD)) * W;
                    let JE = C + (W * (JD.ln()));
                    JI = JE;
                    BPD = BWX;
                } else {
                    let JF = (-JA).exp();
                    let JG = C + JF;
                    let JH = IZ + (W * (JG.ln()));
                    let BWW = BWU + ((((BWV * BUG) * JF) * (BOB / JG)) * W);
                    JI = JH;
                    BPD = BWW;
                }
                let JJ = JI - 6.931471805599453e-4f64;
                KO = JJ;
                BPC = BPD;
            } else {
                KO = IY;
                BPC = BWM;
            }
            let JL = if JK != A { 1.0 } else { 0.0 };
            let ACP;
            let BPE;
            if JL != 0.0 {
                let JN = JM * (C + (BT * JK));
                let BWY = (BOQ * JK) * JM;
                let JO = (JN - C) / W;
                let BWZ = BWY / W;
                let JP = if JN < C { 1.0 } else { 0.0 };
                let JW;
                let BPF;
                if JP != 0.0 {
                    let JQ = JO.exp();
                    let JR = C + JQ;
                    let BXB = ((BWZ * JQ) * (BOB / JR)) * W;
                    let JS = C + (W * (JR.ln()));
                    JW = JS;
                    BPF = BXB;
                } else {
                    let JT = (-JO).exp();
                    let JU = C + JT;
                    let JV = JN + (W * (JU.ln()));
                    let BXA = BWY + ((((BWZ * BUG) * JT) * (BOB / JU)) * W);
                    JW = JV;
                    BPF = BXA;
                }
                let JX = JW - 6.931471805599453e-4f64;
                ACP = JX;
                BPE = BPF;
            } else {
                ACP = JM;
                BPE = BWM;
            }
            let KA = JY * (C + (JZ * BT));
            let BXC = (BOQ * JZ) * JY;
            let KC = KA * KA;
            let BXD = BXC * KA;
            let BXE = BXD + BXD;
            let KD = if KA < A { 1.0 } else { 0.0 };
            let ATX;
            let BPG;
            if KD != 0.0 {
                let KF = (KC + KB).sqrt();
                let KG = KF - KA;
                let KH = 5e-7f64 / KG;
                let BXH = ((((BXE * (BOB / (BXF * KF))) - BXC) * KH) * BUG) / KG;
                ATX = KH;
                BPG = BXH;
            } else {
                let KI = (KC + KB).sqrt();
                let KJ = KE * (KI + KA);
                let BXG = ((BXE * (BOB / (BXF * KI))) + BXC) * KE;
                ATX = KJ;
                BPG = BXG;
            }
            let KN = ((KL - HY) - HZ) + KM;
            let KP = (BU * KN) / KO;
            let KQ = KP.exp();
            let KR = KK * KQ;
            let KS = -CZ;
            let KT = (KS * BS) / KO;
            let KU = KT.exp();
            let KV = KR * KU;
            let BXI = ((((((BUM * KN) - (BPC * KP)) / KO) * KQ) * KK) * KU) + (((((BUL * KS) - (BPC * KT)) / KO) * KU) * KR);
            let KX = C - HY;
            let KY = (BU * KX).exp();
            let KZ = KW * KY;
            let BXJ = ((BUM * KX) * KY) * KW;
            let LC = C - LB;
            let LD = (BU * LC).exp();
            let LE = LA * LD;
            let BXK = ((BUM * LC) * LD) * LA;
            let LI = LG - (X * LH);
            let LJ = (BU * LI).exp();
            let LK = LF * LJ;
            let LL = -parameters[113];
            let LM = LL * BS;
            let BXL = BUL * LL;
            let LN = (LM / LH).exp();
            let LO = LK * LN;
            let BXM = ((((BUM * LI) * LJ) * LF) * LN) + (((BXL / LH) * LN) * LK);
            let LR = LG - (X * LQ);
            let LS = (BU * LR).exp();
            let LT = LP * LS;
            let LU = -DO;
            let LV = ((LU * BS) / LQ).exp();
            let LW = LT * LV;
            let BXN = ((((BUM * LR) * LS) * LP) * LV) + ((((BUL * LU) / LQ) * LV) * LT);
            let LY = (KL - HT) + KM;
            let LZ = BU * LY;
            let BXO = BUM * LY;
            let MB = (LZ / MA).exp();
            let MC = LX * MB;
            let MD = -parameters[111];
            let ME = MD * BS;
            let BXP = BUL * MD;
            let MF = (ME / MA).exp();
            let MG = MC * MF;
            let BXQ = ((((BXO / MA) * MB) * LX) * MF) + (((BXP / MA) * MF) * MC);
            let MJ = (LZ / MI).exp();
            let MK = MH * MJ;
            let ML = (ME / MI).exp();
            let MM = MK * ML;
            let BXR = ((((BXO / MI) * MJ) * MH) * ML) + (((BXP / MI) * ML) * MK);
            let MN = if parameters[24] == C { 1.0 } else { 0.0 };
            let AGE;
            let AGM;
            let AHR;
            let BPH;
            let BPI;
            let BPJ;
            if MN != 0.0 {
                let MP = -parameters[107];
                let MQ = ((MP * BS) / MA).exp();
                let MR = MO * MQ;
                let BXS = (((BUL * MP) / MA) * MQ) * MO;
                let MT = -parameters[106];
                let MU = (MT * BS).exp();
                let MV = MS * MU;
                let BXT = ((BUL * MT) * MU) * MS;
                let MX = -parameters[108];
                let MY = ((MX * BS) / MI).exp();
                let MZ = MW * MY;
                let BXU = (((BUL * MX) / MI) * MY) * MW;
                AGE = MR;
                AGM = MV;
                AHR = MZ;
                BPH = BXS;
                BPI = BXT;
                BPJ = BXU;
            } else {
                AGE = A;
                AGM = A;
                AHR = A;
                BPH = BWM;
                BPI = BWM;
                BPJ = BWM;
            }
            let NB = (KL - LB) + KM;
            let NC = (BU * NB).exp();
            let ND = NA * NC;
            let NE = -parameters[112];
            let NF = (NE * BS).exp();
            let NG = ND * NF;
            let BXV = ((((BUM * NB) * NC) * NA) * NF) + (((BUL * NE) * NF) * ND);
            let NJ = LG - (X * NI);
            let NK = (BU * NJ).exp();
            let NL = NH * NK;
            let NM = (LM / NI).exp();
            let NN = NL * NM;
            let BXW = ((((BUM * NJ) * NK) * NH) * NM) + (((BXL / NI) * NM) * NL);
            let NQ = KL / NP;
            let NR = (BU * NQ).exp();
            let NS = NO * NR;
            let NT = (LM / NP).exp();
            let NU = NS * NT;
            let BXX = ((((BUM * NQ) * NR) * NO) * NT) + (((BXL / NP) * NT) * NS);
            let NW = BO.sqrt();
            let NX = NV * NW;
            let NZ = (NY * BT).exp();
            let OA = NX * NZ;
            let BXY = (((BUJ * (BOB / (BXF * NW))) * NV) * NZ) + (((BOQ * NY) * NZ) * NX);
            let OC = OB * AL;
            let OE = OC.powf(OD);
            let BXZ = (BOR * AL) * (OD * (OC.powf(-1.5e0f64)));
            let OF = C / GZ;
            let BYA = ((BWC * OF) * BUG) / GZ;
            let OH = OG * OB;
            let OI = OH * OB;
            let OJ = OI * OE;
            let OK = (OJ * OF) * AM;
            let OL = ((OK * GV) * AL) * AL;
            let BYB = (((((((((((BOR * OG) * OB) + (BOR * OH)) * OE) + (BXZ * OI)) * OF) + (BYA * OJ)) * AM) * GV) + (BWA * OK)) * AL) * AL;
            let ON = OM * OE;
            let OO = ON * GU;
            let OP = ((OO * GU) * AN) * AN;
            let OQ = OP * GZ;
            let OR = (OG - OL).exp();
            let OS = OQ * OR;
            let BYC = ((((((((((BXZ * OM) * GU) + (BOT * ON)) * GU) + (BOT * OO)) * AN) * AN) * GZ) + (BWC * OP)) * OR) + (((BYB * BUG) * OR) * OQ);
            let OU = OT * BA;
            let OW = OU.powf(OV);
            let BYD = (BOS * BA) * (OV * (OU.powf(-1.5e0f64)));
            let OX = C / HB;
            let OZ = OY * OT;
            let PA = OZ * OT;
            let PB = PA * OW;
            let PC = (PB * OX) * AO;
            let PD = ((PC * GX) * BA) * BA;
            let BYE = (((((((((((BOS * OY) * OT) + (BOS * OZ)) * OW) + (BYD * PA)) * OX) + ((((BWE * OX) * BUG) / HB) * PB)) * AO) * GX) + (BWB * PC)) * BA) * BA;
            let PF = PE * OW;
            let PG = PF * GW;
            let PH = ((PG * GW) * BB) * BB;
            let PI = PH * HB;
            let PJ = (OY - PD).exp();
            let PK = PI * PJ;
            let BYF = ((((((((((BYD * PE) * GW) + (BOX * PF)) * GW) + (BOX * PG)) * BB) * BB) * HB) + (BWE * PH)) * PJ) + (((BYE * BUG) * PJ) * PI);
            let PL = (BU * HZ).exp();
            let BYG = (BUM * HZ) * PL;
            let PN = PM * PL;
            let PO = PN * HO;
            let BYH = ((BYG * PM) * HO) + (BWI * PN);
            let PQ = PP * PL;
            let PR = PQ * OF;
            let BYI = ((BYG * PP) * OF) + (BYA * PQ);
            let PU = KL - PT;
            let PV = (BU * PU).exp();
            let PW = PS * PV;
            let PX = -GI;
            let PY = (PX * BS).exp();
            let BYJ = (BUL * PX) * PY;
            let PZ = PW * PY;
            let BYK = ((((BUM * PU) * PV) * PS) * PY) + (BYJ * PW);
            let QB = 3.5e0f64 - (KE * QA);
            let QC = (BU * QB).exp();
            let QD = U * QC;
            let QE = QD * PY;
            let BYL = ((((BUM * QB) * QC) * U) * PY) + (BYJ * QD);
            let QG = C - PT;
            let QH = (BU * QG).exp();
            let QI = QF * QH;
            let BYM = ((BUM * QG) * QH) * QF;
            let QK = C - QA;
            let QL = (BU * QK).exp();
            let QM = QJ * QL;
            let BYN = ((BUM * QK) * QL) * QJ;
            let QO = HY - X;
            let QP = (BU * QO).exp();
            let QQ = QN * QP;
            let QR = -parameters[120];
            let QS = (QR * BS).exp();
            let QT = QQ * QS;
            let BYO = ((((BUM * QO) * QP) * QN) * QS) + (((BUL * QR) * QS) * QQ);
            let QV = (HZ + HY) - C;
            let QW = (BU * QV).exp();
            let QX = QU * QW;
            let BYP = ((BUM * QV) * QW) * QU;
            let QZ = IT - C;
            let RA = (BU * QZ).exp();
            let RB = QY * RA;
            let BYQ = ((BUM * QZ) * RA) * QY;
            let RD = QX + RB;
            let BYR = BYP + BYQ;
            let RE = QU + QY;
            let RF = (RC * RD) / RE;
            let BYS = (BYR * RC) / RE;
            let RH = parameters[100] - C;
            let RI = (BU * RH).exp();
            let RJ = RG * RI;
            let BYT = ((BUM * RH) * RI) * RG;
            let RK = BN - 3e2f64;
            let RL = if BN < 5.25e2f64 { 1.0 } else { 0.0 };
            let AVX;
            let BPK;
            if RL != 0.0 {
                let RP = RO * RK;
                let RQ = RM * ((C + (RN * RK)) - (RP * RK));
                let BYU = ((BOQ * RN) - (((BOQ * RO) * RK) + (BOQ * RP))) * RM;
                AVX = RQ;
                BPK = BYU;
            } else {
                let RR = RM * 1.081e0f64;
                AVX = RR;
                BPK = BWM;
            }
            let RT = RS * PL;
            let BYV = BYG * RS;
            let RW = RU * ((L / K).powf(RV));
            let RX = if II > A { 1.0 } else { 0.0 };
            let AZO;
            let BPL;
            if RX != 0.0 {
                let RY = C / IL;
                let BYW = ((BWP * RY) * BUG) / IL;
                let RZ = if RY > T { 1.0 } else { 0.0 };
                let AZP;
                let BPM;
                if RZ != 0.0 {
                    AZP = T;
                    BPM = BWM;
                } else {
                    AZP = RY;
                    BPM = BYW;
                }
                AZO = AZP;
                BPL = BPM;
            } else {
                AZO = A;
                BPL = BWM;
            }
            let SA = if IM > A { 1.0 } else { 0.0 };
            let AZR;
            let BPN;
            if SA != 0.0 {
                let SB = C / IP;
                let BYX = ((BWR * SB) * BUG) / IP;
                let SC = if SB > T { 1.0 } else { 0.0 };
                let AZS;
                let BPO;
                if SC != 0.0 {
                    AZS = T;
                    BPO = BWM;
                } else {
                    AZS = SB;
                    BPO = BYX;
                }
                AZR = AZS;
                BPN = BPO;
            } else {
                AZR = A;
                BPN = BWM;
            }
            let SD = if IQ > A { 1.0 } else { 0.0 };
            let AZU;
            let BPP;
            if SD != 0.0 {
                let SE = C / IR;
                let BYY = ((BWS * SE) * BUG) / IR;
                let SF = if SE > T { 1.0 } else { 0.0 };
                let AZV;
                let BPQ;
                if SF != 0.0 {
                    AZV = T;
                    BPQ = BWM;
                } else {
                    AZV = SE;
                    BPQ = BYY;
                }
                AZU = AZV;
                BPP = BPQ;
            } else {
                AZU = A;
                BPP = BWM;
            }
            let SI = B * (SG - SH);
            let BYZ = (Lanes([BOD, 0.0]) - Lanes([0.0, BOE])) * B;
            let SK = B * (SG - SJ);
            let BZA = (Lanes([BOD, 0.0]) - Lanes([0.0, BOF])) * B;
            let SM = B * (SG - SL);
            let BZB = (Lanes([0.0, BOD]) - Lanes([BOG, 0.0])) * B;
            let SO = B * (SN - SL);
            let BZC = (Lanes([0.0, BOH]) - Lanes([BOG, 0.0])) * B;
            let SP = B * (SN - SG);
            let BZD = (Lanes([BOH, 0.0]) - Lanes([0.0, BOD])) * B;
            let SQ = B * (node_potentials[3] - SH);
            let BZE = (Lanes([BOI, 0.0]) - Lanes([0.0, BOE])) * B;
            let SR = B * (SH - SJ);
            let BZF = (Lanes([BOE, 0.0]) - Lanes([0.0, BOF])) * B;
            let ST = B * (SS - SL);
            let BZG = (Lanes([BOJ, 0.0]) - Lanes([0.0, BOG])) * B;
            let SV = B * (SU - SN);
            let BZH = (Lanes([BOK, 0.0]) - Lanes([0.0, BOH])) * B;
            let SW = B * (SU - SS);
            let BZI = (Lanes([BOK, 0.0]) - Lanes([0.0, BOJ])) * B;
            let SX = B * (SU - node_potentials[0]);
            let BZJ = (Lanes([0.0, BOK]) - Lanes([BOL, 0.0])) * B;
            let SZ = B * (SY - SH);
            let BZK = (Lanes([0.0, BOM]) - Lanes([BOE, 0.0])) * B;
            let TA = B * (node_potentials[10] - SY);
            let BZL = (Lanes([BON, 0.0]) - Lanes([0.0, BOM])) * B;
            let BZM = Lanes([BZD[0], BZD[1], 0.0]) + Lanes([0.0, BZA[0], BZA[1]]);
            let BZN = Lanes([BZM[0], BZM[1], 0.0, BZM[2]]) - Lanes([0.0, 0.0, BZF[0], BZF[1]]);
            let TB = ((SP + SK) - SR) - SZ;
            let BZO = Lanes([BZN[0], BZN[1], BZN[2], BZN[3], 0.0]) - Lanes([0.0, 0.0, BZK[0], 0.0, BZK[1]]);
            let BZP = BZJ * BUG;
            let BZQ = Lanes([BZP[0], BZP[1], 0.0]) + Lanes([0.0, BZH[0], BZH[1]]);
            let BZR = Lanes([BZQ[0], BZQ[1], BZQ[2], 0.0, 0.0, 0.0, 0.0]) + Lanes([0.0, 0.0, BZO[0], BZO[1], BZO[2], BZO[3], BZO[4]]);
            let TC = (((-SX) + SV) + TB) - TA;
            let BZS = Lanes([BZR[0], BZR[1], BZR[2], BZR[3], BZR[4], BZR[5], 0.0, BZR[6]]) - Lanes([0.0, 0.0, 0.0, 0.0, 0.0, 0.0, BZL[0], BZL[1]]);
            let TD = SX + TC;
            let BZT = Lanes([BZJ[0], BZJ[1], 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]) + BZS;
            let TE = SQ - SZ;
            let BZU = Lanes([BZE[0], BZE[1], 0.0]) - Lanes([0.0, BZK[0], BZK[1]]);
            let TF = TE - TA;
            let BZV = Lanes([BZU[0], BZU[1], 0.0, BZU[2]]) - Lanes([0.0, 0.0, BZL[0], BZL[1]]);
            let TG = SK * BR;
            let BZW = BZA * BR;
            let BZX = Lanes([0.0, BZW[0], BZW[1]]) + Lanes([(BUL * SK), 0.0, 0.0]);
            let TI = if TG < TH { 1.0 } else { 0.0 };
            let ZT;
            let BPR;
            if TI != 0.0 {
                let TJ = TG.exp();
                let BZZ = BZX * TJ;
                ZT = TJ;
                BPR = BZZ;
            } else {
                let TK = TH.exp();
                let TL = TK * (C + (TG - TH));
                let BZY = BZX * TK;
                ZT = TL;
                BPR = BZY;
            }
            let TM = SM * BR;
            let CAA = BZB * BR;
            let CAB = Lanes([0.0, CAA[0], CAA[1]]) + Lanes([(BUL * SM), 0.0, 0.0]);
            let TN = TM / KO;
            let CAC = (CAB - Lanes([(BPC * TN), 0.0, 0.0])) / KO;
            let TO = if TN < TH { 1.0 } else { 0.0 };
            let ACJ;
            let BPS;
            if TO != 0.0 {
                let TP = TN.exp();
                let CAE = CAC * TP;
                ACJ = TP;
                BPS = CAE;
            } else {
                let TQ = TH.exp();
                let TR = TQ * (C + (TN - TH));
                let CAD = CAC * TQ;
                ACJ = TR;
                BPS = CAD;
            }
            let TS = TB * BR;
            let CAF = BZO * BR;
            let CAG = Lanes([0.0, CAF[0], CAF[1], CAF[2], CAF[3], CAF[4]]) + Lanes([(BUL * TB), 0.0, 0.0, 0.0, 0.0, 0.0]);
            let TT = if TS < TH { 1.0 } else { 0.0 };
            let AMY;
            let BPT;
            if TT != 0.0 {
                let TU = TS.exp();
                let CAI = CAG * TU;
                AMY = TU;
                BPT = CAI;
            } else {
                let TV = TH.exp();
                let TW = TV * (C + (TS - TH));
                let CAH = CAG * TV;
                AMY = TW;
                BPT = CAH;
            }
            let TX = SP * BR;
            let CAJ = BZD * BR;
            let CAK = Lanes([0.0, CAJ[0], CAJ[1]]) + Lanes([(BUL * SP), 0.0, 0.0]);
            let TY = if TX < TH { 1.0 } else { 0.0 };
            let ATG;
            let BPU;
            if TY != 0.0 {
                let TZ = TX.exp();
                let CAM = CAK * TZ;
                ATG = TZ;
                BPU = CAM;
            } else {
                let UA = TH.exp();
                let UB = UA * (C + (TX - TH));
                let CAL = CAK * UA;
                ATG = UB;
                BPU = CAL;
            }
            let UC = TD * BR;
            let CAN = BZT * BR;
            let CAO = Lanes([CAN[0], CAN[1], 0.0, CAN[2], CAN[3], CAN[4], CAN[5], CAN[6], CAN[7]]) + Lanes([0.0, 0.0, (BUL * TD), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
            let UD = if UC < TH { 1.0 } else { 0.0 };
            let APO;
            let BPV;
            if UD != 0.0 {
                let UE = UC.exp();
                let CAQ = CAO * UE;
                APO = UE;
                BPV = CAQ;
            } else {
                let UF = TH.exp();
                let UG = UF * (C + (UC - TH));
                let CAP = CAO * UF;
                APO = UG;
                BPV = CAP;
            }
            let UH = SQ * BR;
            let CAR = BZE * BR;
            let CAS = Lanes([CAR[0], 0.0, CAR[1]]) + Lanes([0.0, (BUL * SQ), 0.0]);
            let UI = if UH < TH { 1.0 } else { 0.0 };
            let ANS;
            let BPW;
            if UI != 0.0 {
                let UJ = UH.exp();
                let CAU = CAS * UJ;
                ANS = UJ;
                BPW = CAU;
            } else {
                let UK = TH.exp();
                let UL = UK * (C + (UH - TH));
                let CAT = CAS * UK;
                ANS = UL;
                BPW = CAT;
            }
            let UM = TF * BR;
            let CAV = BZV * BR;
            let CAW = Lanes([CAV[0], 0.0, CAV[1], CAV[2], CAV[3]]) + Lanes([0.0, (BUL * TF), 0.0, 0.0, 0.0]);
            let UN = if UM < TH { 1.0 } else { 0.0 };
            let APV;
            let BPX;
            if UN != 0.0 {
                let UO = UM.exp();
                let CAY = CAW * UO;
                APV = UO;
                BPX = CAY;
            } else {
                let UP = TH.exp();
                let UQ = UP * (C + (UM - TH));
                let CAX = CAW * UP;
                APV = UQ;
                BPX = CAX;
            }
            let UR = TE * BR;
            let CAZ = BZU * BR;
            let CBA = Lanes([CAZ[0], 0.0, CAZ[1], CAZ[2]]) + Lanes([0.0, (BUL * TE), 0.0, 0.0]);
            let US = if UR < TH { 1.0 } else { 0.0 };
            let AOD;
            let BPY;
            if US != 0.0 {
                let UT = UR.exp();
                let CBC = CBA * UT;
                AOD = UT;
                BPY = CBC;
            } else {
                let UU = TH.exp();
                let UV = UU * (C + (UR - TH));
                let CBB = CBA * UU;
                AOD = UV;
                BPY = CBB;
            }
            let UX = TD - UW;
            let CBD = Lanes([BZT[0], BZT[1], 0.0, BZT[2], BZT[3], BZT[4], BZT[5], BZT[6], BZT[7]]);
            let UY = UX * BR;
            let CBE = ((CBD - Lanes([0.0, 0.0, BOU, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])) * BR) + Lanes([0.0, 0.0, (BUL * UX), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
            let UZ = if UY < TH { 1.0 } else { 0.0 };
            let BFQ;
            let BPZ;
            if UZ != 0.0 {
                let VA = UY.exp();
                let CBG = CBE * VA;
                BFQ = VA;
                BPZ = CBG;
            } else {
                let VB = TH.exp();
                let VC = VB * (C + (UY - TH));
                let CBF = CBE * VB;
                BFQ = VC;
                BPZ = CBF;
            }
            let VD = TB - UW;
            let CBH = Lanes([0.0, BZO[0], BZO[1], BZO[2], BZO[3], BZO[4]]);
            let VE = VD * BR;
            let CBI = ((CBH - Lanes([BOU, 0.0, 0.0, 0.0, 0.0, 0.0])) * BR) + Lanes([(BUL * VD), 0.0, 0.0, 0.0, 0.0, 0.0]);
            let VF = if VE < TH { 1.0 } else { 0.0 };
            let ANA;
            let BQA;
            if VF != 0.0 {
                let VG = VE.exp();
                let CBK = CBI * VG;
                ANA = VG;
                BQA = CBK;
            } else {
                let VH = TH.exp();
                let VI = VH * (C + (VE - TH));
                let CBJ = CBI * VH;
                ANA = VI;
                BQA = CBJ;
            }
            let VJ = SK - UW;
            let VK = VJ * BR;
            let CBL = ((Lanes([0.0, BZA[0], BZA[1]]) - Lanes([BOU, 0.0, 0.0])) * BR) + Lanes([(BUL * VJ), 0.0, 0.0]);
            let VL = if VK < TH { 1.0 } else { 0.0 };
            let VV;
            let BQB;
            if VL != 0.0 {
                let VM = VK.exp();
                let CBN = CBL * VM;
                VV = VM;
                BQB = CBN;
            } else {
                let VN = TH.exp();
                let VO = VN * (C + (VK - TH));
                let CBM = CBL * VN;
                VV = VO;
                BQB = CBM;
            }
            let VP = SI - UW;
            let CBO = Lanes([0.0, BYZ[0], BYZ[1]]);
            let CBP = Lanes([BOU, 0.0, 0.0]);
            let VQ = VP * BR;
            let CBQ = ((CBO - CBP) * BR) + Lanes([(BUL * VP), 0.0, 0.0]);
            let VR = if VQ < TH { 1.0 } else { 0.0 };
            let VX;
            let BQC;
            if VR != 0.0 {
                let VS = VQ.exp();
                let CBS = CBQ * VS;
                VX = VS;
                BQC = CBS;
            } else {
                let VT = TH.exp();
                let VU = VT * (C + (VQ - TH));
                let CBR = CBQ * VT;
                VX = VU;
                BQC = CBR;
            }
            let VW = (C + (KL * VV)).sqrt();
            let CBT = (BQB * KL) * (BOB / (BXF * VW));
            let VY = (C + (KL * VX)).sqrt();
            let CBU = (BQC * KL) * (BOB / (BXF * VY));
            let VZ = C + VY;
            let WA = (X * VX) / VZ;
            let CBV = ((BQC * X) - (CBU * WA)) / VZ;
            let WC = if WA < WB { 1.0 } else { 0.0 };
            let YG;
            let BQD;
            if WC != 0.0 {
                YG = WB;
                BQD = CBW;
            } else {
                YG = WA;
                BQD = CBV;
            }
            let CBX = Lanes([CBT[0], CBT[1], 0.0, CBT[2]]);
            let WD = VW + C;
            let WE = WD / VZ;
            let CBY = CBU * WE;
            let WF = (VW - VY) - (WE.ln());
            let WG = BQ * WF;
            let CBZ = Lanes([(BUK * WF), 0.0, 0.0, 0.0]) + (((CBX - Lanes([CBU[0], CBU[1], CBU[2], 0.0])) - (((CBX - Lanes([CBY[0], CBY[1], CBY[2], 0.0])) / VZ) * (BOB / WE))) * BQ);
            let CCA = Lanes([0.0, 0.0, BZF[0], BZF[1]]);
            let WH = (WG + SR) / IV;
            let CCB = ((CBZ + CCA) - Lanes([(BWT * WH), 0.0, 0.0, 0.0])) / IV;
            let WI = if WH > A { 1.0 } else { 0.0 };
            let AAZ;
            let ABI;
            let ABU;
            let ACO;
            let AUU;
            let AVP;
            let BEP;
            let BQE;
            let BQF;
            let BQG;
            let BQH;
            let BQI;
            let BQJ;
            let BQK;
            if WI != 0.0 {
                let WK = if SI < WJ { 1.0 } else { 0.0 };
                let WS;
                let BQL;
                if WK != 0.0 {
                    WS = SI;
                    BQL = BYZ;
                } else {
                    let WL = C + (SI - WJ);
                    let CCL = BYZ * (BOB / WL);
                    let WM = WJ + (WL.ln());
                    WS = WM;
                    BQL = CCL;
                }
                let WN = X * BQ;
                let WO = KE * WH;
                let WP = WO * IV;
                let CCM = ((CCB * KE) * IV) + Lanes([(BWT * WO), 0.0, 0.0, 0.0]);
                let WQ = (WP * BR) + C;
                let WR = WQ.ln();
                let WT = (UW + (WN * WR)) - WS;
                let CCN = (Lanes([BOU, 0.0, 0.0, 0.0]) + (Lanes([((BUK * X) * WR), 0.0, 0.0, 0.0]) + ((((CCM * BR) + Lanes([(BUL * WP), 0.0, 0.0, 0.0])) * (BOB / WQ)) * WN))) - Lanes([0.0, BQL[0], BQL[1], 0.0]);
                let WV = WU * UW;
                let WW = WV * WV;
                let CCO = (BOU * WU) * WV;
                let CCP = CCO + CCO;
                let WX = WT * WT;
                let CCQ = CCN * WT;
                let CCR = CCQ + CCQ;
                let WY = if WT < A { 1.0 } else { 0.0 };
                let XE;
                let BQM;
                if WY != 0.0 {
                    let WZ = (WX + WW).sqrt();
                    let XA = WZ - WT;
                    let XB = (KE * WW) / XA;
                    let CCT = (Lanes([(CCP * KE), 0.0, 0.0, 0.0]) - ((((CCR + Lanes([CCP, 0.0, 0.0, 0.0])) * (BOB / (BXF * WZ))) - CCN) * XB)) / XA;
                    XE = XB;
                    BQM = CCT;
                } else {
                    let XC = (WX + WW).sqrt();
                    let XD = KE * (XC + WT);
                    let CCS = (((CCR + Lanes([CCP, 0.0, 0.0, 0.0])) * (BOB / (BXF * XC))) + CCN) * KE;
                    XE = XD;
                    BQM = CCS;
                }
                let XH = XF * XG;
                let XI = XE + XH;
                let XJ = XG * (XE + (XF * IV));
                let XK = (XE * XI) / XJ;
                let CCU = (((BQM * XI) + (BQM * XE)) - (((BQM + Lanes([(BWT * XF), 0.0, 0.0, 0.0])) * XG) * XK)) / XJ;
                let XL = WH / XK;
                let CCV = (CCB - (CCU * XL)) / XK;
                let XN = (XL - C) / XM;
                let CCW = CCV / XM;
                let XO = if XL < C { 1.0 } else { 0.0 };
                let XV;
                let BQN;
                if XO != 0.0 {
                    let XP = XN.exp();
                    let XQ = C + XP;
                    let CCY = ((CCW * XP) * (BOB / XQ)) * XM;
                    let XR = C + (XM * (XQ.ln()));
                    XV = XR;
                    BQN = CCY;
                } else {
                    let XS = (-XN).exp();
                    let XT = C + XS;
                    let XU = XL + (XM * (XT.ln()));
                    let CCX = CCV + ((((CCW * BUG) * XS) * (BOB / XT)) * XM);
                    XV = XU;
                    BQN = CCX;
                }
                let XW = C + (XM * ((C + ((-1e0f64 / XM).exp())).ln()));
                let XX = XV / XW;
                let CCZ = BQN / XW;
                let XY = XE / XH;
                let CDA = BQM / XH;
                let XZ = KL * XX;
                let YA = XZ * XY;
                let YB = C + XY;
                let YC = (C + (YA * YB)).sqrt();
                let YD = X * XX;
                let YE = YD * YB;
                let YF = (C + YC) / YE;
                let CDB = (((((((CCZ * KL) * XY) + (CDA * XZ)) * YB) + (CDA * YA)) * (BOB / (BXF * YC))) - ((((CCZ * X) * YB) + (CDA * YD)) * YF)) / YE;
                let YH = YG * YF;
                let CDC = BQD * YF;
                let CDD = Lanes([CDC[0], CDC[1], CDC[2], 0.0]) + (CDB * YG);
                let YI = C + YH;
                let YJ = ((C - YF) + YH) / YI;
                let CDE = (((CDB * BUG) + CDD) - (CDD * YJ)) / YI;
                let YK = WP * YJ;
                let YL = YK * BR;
                let CDF = (((CCM * YJ) + (CDE * WP)) * BR) + Lanes([(BUL * YK), 0.0, 0.0, 0.0]);
                let YM = (YG + YL) + C;
                let CDG = BQD * YM;
                let YN = (X * YL) + (YG * YM);
                let CDH = (CDF * X) + (Lanes([CDG[0], CDG[1], CDG[2], 0.0]) + ((Lanes([BQD[0], BQD[1], BQD[2], 0.0]) + CDF) * YG));
                let YO = KE * (YL - C);
                let CDI = CDF * KE;
                let CDJ = CDI * YO;
                let YP = (YO * YO) + YN;
                let CDK = (CDJ + CDJ) + CDH;
                let YQ = if YL >= C { 1.0 } else { 0.0 };
                let YW;
                let BQO;
                if YQ != 0.0 {
                    let YR = YP.sqrt();
                    let YS = YO + YR;
                    let CDM = CDI + (CDK * (BOB / (BXF * YR)));
                    YW = YS;
                    BQO = CDM;
                } else {
                    let YT = YP.sqrt();
                    let YU = YT - YO;
                    let YV = YN / YU;
                    let CDL = (CDH - (((CDK * (BOB / (BXF * YT))) - CDI) * YV)) / YU;
                    YW = YV;
                    BQO = CDL;
                }
                let YY = if YW < YX { 1.0 } else { 0.0 };
                let YZ;
                let BQP;
                if YY != 0.0 {
                    YZ = YX;
                    BQP = CCK;
                } else {
                    YZ = YW;
                    BQP = BQO;
                }
                let ZA = YZ + C;
                let ZB = YZ * ZA;
                let ZC = (UW * BR).exp();
                let ZD = ZB * ZC;
                let CDN = (((BQP * ZA) + (BQP * YZ)) * ZC) + Lanes([((((BOU * BR) + (BUL * UW)) * ZC) * ZB), 0.0, 0.0, 0.0]);
                let ZE = KE * XG;
                let ZF = ZE * (WH - XF);
                let CDO = CCB * ZE;
                let ZG = (XG * IV) * XF;
                let CDP = CDO * ZF;
                let ZH = ((ZF * ZF) + (ZG * WH)).sqrt();
                let ZI = ZF + ZH;
                let CDQ = CDO + (((CDP + CDP) + (Lanes([(((BWT * XG) * XF) * WH), 0.0, 0.0, 0.0]) + (CCB * ZG))) * (BOB / (BXF * ZH)));
                let ZJ = if parameters[73] == A { 1.0 } else { 0.0 };
                let ABJ;
                let BQQ;
                if ZJ != 0.0 {
                    let ZK = HL * AG;
                    let CDS = Lanes([(BOW * AG), 0.0, 0.0, 0.0]);
                    ABJ = ZK;
                    BQQ = CDS;
                } else {
                    let ZL = WH + XK;
                    let ZM = (X * WH) / ZL;
                    let ZN = AG + ZM;
                    let ZO = HL * ZN;
                    let CDR = Lanes([(BOW * ZN), 0.0, 0.0, 0.0]) + ((((CCB * X) - ((CCB + CCU) * ZM)) / ZL) * HL);
                    ABJ = ZO;
                    BQQ = CDR;
                }
                let ZP = XF + WH;
                let ZQ = (XF * WH) / ZP;
                let CDT = ((CCB * XF) - (CCB * ZQ)) / ZP;
                let ZR = XF / ZP;
                let CDU = ((CCB * ZR) * BUG) / ZP;
                AAZ = ZI;
                ABI = ABJ;
                ABU = ZR;
                ACO = ZD;
                AUU = YJ;
                AVP = ZQ;
                BEP = YZ;
                BQE = CDQ;
                BQF = BQQ;
                BQG = CDU;
                BQH = CDN;
                BQI = CDE;
                BQJ = CDT;
                BQK = BQP;
            } else {
                let ZS = (X * VV) / WD;
                let CCC = ((BQB * X) - (CBT * ZS)) / WD;
                let ZU = if (if (SR.abs()) < (1e-5f64 * BQ) { 1.0 } else { 0.0 }) != 0.0 || (if (WG.abs()) < ((1e-40f64 * BQ) * (VW + VY)) { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let AUV;
                let BQR;
                if ZU != 0.0 {
                    let ZV = KE * (ZS + YG);
                    let CCE = (Lanes([CCC[0], CCC[1], 0.0, CCC[2]]) + Lanes([BQD[0], BQD[1], BQD[2], 0.0])) * KE;
                    let ZW = ZV + C;
                    let ZX = ZV / ZW;
                    let CCF = (CCE - (CCE * ZX)) / ZW;
                    AUV = ZX;
                    BQR = CCF;
                } else {
                    let ZY = (WG + SK) - SI;
                    let ZZ = WG / ZY;
                    let CCD = (CBZ - (((CBZ + Lanes([0.0, BZA[0], 0.0, BZA[1]])) - Lanes([0.0, BYZ[0], BYZ[1], 0.0])) * ZZ)) / ZY;
                    AUV = ZZ;
                    BQR = CCD;
                }
                let AAA = AG * HL;
                let AAB = C - (WH / XF);
                let CCG = (CCB / XF) * BUG;
                let CCH = Lanes([(BOW * AG), 0.0, 0.0, 0.0]);
                let CCI = Lanes([BPR[0], BPR[1], 0.0, BPR[2]]);
                let CCJ = Lanes([CCC[0], CCC[1], 0.0, CCC[2]]);
                AAZ = SR;
                ABI = AAA;
                ABU = AAB;
                ACO = ZT;
                AUU = AUV;
                AVP = WH;
                BEP = ZS;
                BQE = CCA;
                BQF = CCH;
                BQG = CCG;
                BQH = CCI;
                BQI = BQR;
                BQJ = CCB;
                BQK = CCJ;
            }
            let AAC = C - (CV.powf((-1e0f64 / Y)));
            let AAD = GU * AAC;
            let CDV = BOT * AAC;
            let AAE = AG * GU;
            let CDW = BOT * AG;
            let CDX = Lanes([0.0, BZB[0], BZB[1]]);
            let CDY = Lanes([CDV, 0.0, 0.0]);
            let AAF = (SM - AAD) / AAE;
            let CDZ = ((CDX - CDY) - Lanes([(CDW * AAF), 0.0, 0.0])) / AAE;
            let AAG = if SM < AAD { 1.0 } else { 0.0 };
            let AAP;
            let BQS;
            if AAG != 0.0 {
                let AAH = AAF.exp();
                let AAI = C + AAH;
                let AAJ = AAI.ln();
                let AAK = SM - (AAE * AAJ);
                let CEB = CDX - (Lanes([(CDW * AAJ), 0.0, 0.0]) + (((CDZ * AAH) * (BOB / AAI)) * AAE));
                AAP = AAK;
                BQS = CEB;
            } else {
                let AAL = (-AAF).exp();
                let AAM = C + AAL;
                let AAN = AAM.ln();
                let AAO = AAD - (AAE * AAN);
                let CEA = CDY - (Lanes([(CDW * AAN), 0.0, 0.0]) + ((((CDZ * BUG) * AAL) * (BOB / AAM)) * AAE));
                AAP = AAO;
                BQS = CEA;
            }
            let AAQ = C - (AAP * GV);
            let CEC = ((BQS * GV) + Lanes([(BWA * AAP), 0.0, 0.0])) * BUG;
            let AAR = C - Y;
            let AAS = AAQ.powf(AAR);
            let CED = AAR - BOB;
            let CEE = CEC * (AAR * (AAQ.powf(CED)));
            let AAT = GU / AAR;
            let CEF = BOT / AAR;
            let AAU = C - AAS;
            let AAV = (AAT * AAU) + (CV * (SM - AAP));
            let CEG = (Lanes([(CEF * AAU), 0.0, 0.0]) + ((CEE * BUG) * AAT)) + ((CDX - BQS) * CV);
            let AAX = if AAW == C { 1.0 } else { 0.0 };
            let ABG;
            let BQT;
            if AAX != 0.0 {
                let CEJ = Lanes([0.0, BYZ[0], BYZ[1], 0.0]);
                ABG = SI;
                BQT = CEJ;
            } else {
                let AAY = if AAW == X { 1.0 } else { 0.0 };
                let ABH;
                let BQU;
                if AAY != 0.0 {
                    let ABA = SI + AAZ;
                    let CEI = Lanes([0.0, BYZ[0], BYZ[1], 0.0]) + BQE;
                    ABH = ABA;
                    BQU = CEI;
                } else {
                    let CEH = Lanes([0.0, BZA[0], 0.0, BZA[1]]);
                    ABH = SK;
                    BQU = CEH;
                }
                ABG = ABH;
                BQT = BQU;
            }
            let CEK = BWK * BUG;
            let ABB = C - HR;
            let ABC = (X - HR) / ABB;
            let CEL = (CEK - (CEK * ABC)) / ABB;
            let ABD = -1e0f64 / AP;
            let ABE = C - (ABC.powf(ABD));
            let ABF = HL * ABE;
            let CEM = (BOW * ABE) + (((CEL * (ABD * (ABC.powf((ABD - BOB))))) * BUG) * HL);
            let CEN = Lanes([CEM, 0.0, 0.0, 0.0]);
            let ABK = (ABG - ABF) / ABI;
            let CEO = ((BQT - CEN) - (BQF * ABK)) / ABI;
            let ABL = if ABG < ABF { 1.0 } else { 0.0 };
            let ABZ;
            let BQV;
            if ABL != 0.0 {
                let ABM = ABK.exp();
                let ABN = C + ABM;
                let ABO = ABN.ln();
                let ABP = ABG - (ABI * ABO);
                let CEQ = BQT - ((BQF * ABO) + (((CEO * ABM) * (BOB / ABN)) * ABI));
                ABZ = ABP;
                BQV = CEQ;
            } else {
                let ABQ = (-ABK).exp();
                let ABR = C + ABQ;
                let ABS = ABR.ln();
                let ABT = ABF - (ABI * ABS);
                let CEP = CEN - ((BQF * ABS) + ((((CEO * BUG) * ABQ) * (BOB / ABR)) * ABI));
                ABZ = ABT;
                BQV = CEP;
            }
            let ABW = ABU.powf(ABV);
            let CER = BQG * (ABV * (ABU.powf((ABV - BOB))));
            let ABX = C - AP;
            let ABY = HL / ABX;
            let CES = BOW / ABX;
            let ACA = ABZ / HL;
            let ACB = C - ACA;
            let ACC = ACB.powf(ABX);
            let CET = ABX - BOB;
            let ACD = C - (ABW * ACC);
            let ACE = ABW * ABC;
            let ACF = ABG - ABZ;
            let ACG = (ABY * ACD) + (ACE * ACF);
            let CEU = BYZ * HR;
            let CEV = Lanes([(BWK * SI), 0.0, 0.0]) + Lanes([0.0, CEU[0], CEU[1]]);
            let ACH = (ABB * ACG) + (HR * SI);
            let CEW = (Lanes([(CEK * ACG), 0.0, 0.0, 0.0]) + (((Lanes([(CES * ACD), 0.0, 0.0, 0.0]) + ((((CER * ACC) + (((((BQV - Lanes([(BOW * ACA), 0.0, 0.0, 0.0])) / HL) * BUG) * (ABX * (ACB.powf(CET)))) * ABW)) * BUG) * ABY)) + ((((CER * ABC) + Lanes([(CEL * ABW), 0.0, 0.0, 0.0])) * ACF) + ((BQT - BQV) * ACE))) * ABB)) + Lanes([CEV[0], CEV[1], CEV[2], 0.0]);
            let ACI = (KL * KV) / KZ;
            let CEX = ((BXI * KL) - (BXJ * ACI)) / KZ;
            let ACK = ACI * ACJ;
            let CEY = Lanes([(CEX * ACJ), 0.0, 0.0]) + (BPS * ACI);
            let ACL = (C + ACK).sqrt();
            let CEZ = CEY * (BOB / (BXF * ACL));
            let ACM = C + ACL;
            let ACN = ACK / ACM;
            let CFA = (CEY - (CEZ * ACN)) / ACM;
            let ACQ = C / ACP;
            let ACR = ACO.powf(ACQ);
            let CFB = ACO.ln();
            let CFC = (BQH * (ACQ * (ACO.powf((ACQ - BOB))))) + Lanes([((((BPE * ACQ) * BUG) / ACP) * (ACR * CFB)), 0.0, 0.0, 0.0]);
            let ACS = ACI * ACR;
            let CFD = Lanes([(CEX * ACR), 0.0, 0.0, 0.0]) + (CFC * ACI);
            let ACT = (C + ACS).sqrt();
            let ACU = C + ACT;
            let ACV = ACS / ACU;
            let CFE = (CFD - ((CFD * (BOB / (BXF * ACT))) * ACV)) / ACU;
            let ACW = if RS == A { 1.0 } else { 0.0 };
            let ADL;
            let BQW;
            if ACW != 0.0 {
                let ACX = AAV / PR;
                let CFI = (CEG - Lanes([(BYI * ACX), 0.0, 0.0])) / PR;
                let ACY = ACH / PO;
                let CFJ = (CEW - Lanes([(BYH * ACY), 0.0, 0.0, 0.0])) / PO;
                let ACZ = (C + ACX) + ACY;
                let CFK = Lanes([CFI[0], CFI[1], CFI[2], 0.0, 0.0]) + Lanes([CFJ[0], 0.0, CFJ[1], CFJ[2], CFJ[3]]);
                ADL = ACZ;
                BQW = CFK;
            } else {
                let ADA = AAV / PR;
                let ADB = ADA + C;
                let ADC = ADB * RT;
                let ADD = (-ACH) / PO;
                let ADE = ADD * RT;
                let ADF = (ADC * BR).exp();
                let CFF = ((((((CEG - Lanes([(BYI * ADA), 0.0, 0.0])) / PR) * RT) + Lanes([(BYV * ADB), 0.0, 0.0])) * BR) + Lanes([(BUL * ADC), 0.0, 0.0])) * ADF;
                let ADG = (ADE * BR).exp();
                let CFG = (((((((CEW * BUG) - Lanes([(BYH * ADD), 0.0, 0.0, 0.0])) / PO) * RT) + Lanes([(BYV * ADD), 0.0, 0.0, 0.0])) * BR) + Lanes([(BUL * ADE), 0.0, 0.0, 0.0])) * ADG;
                let ADH = (RT * BR).exp();
                let ADI = ADH - C;
                let ADJ = (ADF - ADG) / ADI;
                let CFH = ((Lanes([CFF[0], CFF[1], CFF[2], 0.0, 0.0]) - Lanes([CFG[0], 0.0, CFG[1], CFG[2], CFG[3]])) - Lanes([((((BYV * BR) + (BUL * RT)) * ADH) * ADJ), 0.0, 0.0, 0.0, 0.0])) / ADI;
                ADL = ADJ;
                BQW = CFH;
            }
            let ADM = ADL * ADL;
            let CFL = BQW * ADL;
            let CFM = CFL + CFL;
            let ADN = if ADL < A { 1.0 } else { 0.0 };
            let ADT;
            let BQX;
            if ADN != 0.0 {
                let ADO = (ADM + ADK).sqrt();
                let ADP = ADO - ADL;
                let ADQ = 5.000000000000001e-3f64 / ADP;
                let CFO = ((((CFM * (BOB / (BXF * ADO))) - BQW) * ADQ) * BUG) / ADP;
                ADT = ADQ;
                BQX = CFO;
            } else {
                let ADR = (ADM + ADK).sqrt();
                let ADS = KE * (ADR + ADL);
                let CFN = ((CFM * (BOB / (BXF * ADR))) + BQW) * KE;
                ADT = ADS;
                BQX = CFN;
            }
            let CFP = (Lanes([CFA[0], CFA[1], CFA[2], 0.0, 0.0]) + Lanes([CFE[0], 0.0, CFE[1], CFE[2], CFE[3]])) * KE;
            let ADU = C + (KE * (ACN + ACV));
            let ADV = ADT * ADU;
            let CFQ = (BQX * ADU) + (CFP * ADT);
            let ADX = ADW * KV;
            let ADY = ADX * ACR;
            let CFR = Lanes([((BXI * ADW) * ACR), 0.0, 0.0, 0.0]) + (CFC * ADX);
            let ADZ = KV * ACJ;
            let CFS = Lanes([(BXI * ACJ), 0.0, 0.0]) + (BPS * KV);
            let CFT = Lanes([CFS[0], CFS[1], CFS[2], 0.0, 0.0]);
            let CFU = Lanes([CFR[0], 0.0, CFR[1], CFR[2], CFR[3]]);
            let AEA = (ADZ - ADY) / ADV;
            let CFV = ((CFT - CFU) - (CFQ * AEA)) / ADV;
            let AEC = SM / AEB;
            let CFW = BZB / AEB;
            let AED = if SM < A { 1.0 } else { 0.0 };
            let AEK;
            let BQY;
            if AED != 0.0 {
                let AEE = AEC.exp();
                let AEF = C + AEE;
                let AEG = AEB * (AEF.ln());
                let CFY = ((CFW * AEE) * (BOB / AEF)) * AEB;
                AEK = AEG;
                BQY = CFY;
            } else {
                let AEH = (-AEC).exp();
                let AEI = C + AEH;
                let AEJ = SM + (AEB * (AEI.ln()));
                let CFX = BZB + ((((CFW * BUG) * AEH) * (BOB / AEI)) * AEB);
                AEK = AEJ;
                BQY = CFX;
            }
            let AEM = AEK / AEL;
            let CFZ = BQY / AEL;
            let AEN = if AEM < TH { 1.0 } else { 0.0 };
            let AER;
            let BQZ;
            if AEN != 0.0 {
                let AEO = AEM.exp();
                let CGB = CFZ * AEO;
                AER = AEO;
                BQZ = CGB;
            } else {
                let AEP = TH.exp();
                let AEQ = AEP * (C + (AEM - TH));
                let CGA = CFZ * AEP;
                AER = AEQ;
                BQZ = CGA;
            }
            let AES = AER - C;
            let AET = OA * AES;
            let CGC = BQZ * OA;
            let CGD = Lanes([(BXY * AES), 0.0, 0.0]) + Lanes([0.0, CGC[0], CGC[1]]);
            let AEV = (SM - AEU) / W;
            let CGE = BZB / W;
            let AEW = if SM < AEU { 1.0 } else { 0.0 };
            let AFE;
            let BRA;
            if AEW != 0.0 {
                let AEX = AEV.exp();
                let AEY = C + AEX;
                let AEZ = SM - (W * (AEY.ln()));
                let CGG = BZB - (((CGE * AEX) * (BOB / AEY)) * W);
                AFE = AEZ;
                BRA = CGG;
            } else {
                let AFA = (-AEV).exp();
                let AFB = C + AFA;
                let AFC = AEU - (W * (AFB.ln()));
                let CGF = ((((CGE * BUG) * AFA) * (BOB / AFB)) * W) * BUG;
                AFE = AFC;
                BRA = CGF;
            }
            let AFF = AFD * AFE;
            let AFG = AEU - AFE;
            let AFH = AFG * AFG;
            let AFI = AFF * AFH;
            let CGH = ((BRA * AFD) * AFH) + (((BRA * BUG) * (X * AFG)) * AFF);
            let AFJ = TM / MA;
            let CGI = CAB / MA;
            let AFK = if AFJ < TH { 1.0 } else { 0.0 };
            let AGC;
            let BRB;
            if AFK != 0.0 {
                let AFL = AFJ.exp();
                let CGK = CGI * AFL;
                AGC = AFL;
                BRB = CGK;
            } else {
                let AFM = TH.exp();
                let AFN = AFM * (C + (AFJ - TH));
                let CGJ = CGI * AFM;
                AGC = AFN;
                BRB = CGJ;
            }
            let AZX;
            let BRC;
            if MN != 0.0 {
                let AFP = SM - AFO;
                let AFQ = AFP * BR;
                let CGQ = ((CDX - Lanes([BOY, 0.0, 0.0])) * BR) + Lanes([(BUL * AFP), 0.0, 0.0]);
                let AFR = if AFQ < TH { 1.0 } else { 0.0 };
                let AGG;
                let BRD;
                if AFR != 0.0 {
                    let AFS = AFQ.exp();
                    let CGS = CGQ * AFS;
                    AGG = AFS;
                    BRD = CGS;
                } else {
                    let AFT = TH.exp();
                    let AFU = AFT * (C + (AFQ - TH));
                    let CGR = CGQ * AFT;
                    AGG = AFU;
                    BRD = CGR;
                }
                let AFV = AEA / KV;
                let CGT = (CFV - Lanes([(BXI * AFV), 0.0, 0.0, 0.0, 0.0])) / KV;
                let AFW = AFV - 1e3f64;
                let AFY = if AFW < AFX { 1.0 } else { 0.0 };
                let AGP;
                let BRE;
                if AFY != 0.0 {
                    let AFZ = AFW.exp();
                    let CGV = CGT * AFZ;
                    AGP = AFZ;
                    BRE = CGV;
                } else {
                    let AGB = AGA * (C + (AFW - AFX));
                    let CGU = CGT * AGA;
                    AGP = AGB;
                    BRE = CGU;
                }
                let AGD = AGC - C;
                let CGW = Lanes([(BXQ * AGD), 0.0, 0.0]) + (BRB * MG);
                let AGF = AGE * X;
                let AGH = (C + (KL * AGG)).sqrt();
                let AGI = C + AGH;
                let AGJ = (AGF * AGD) / AGI;
                let AGK = ACH / PO;
                let AGL = C + AGK;
                let CGX = (((Lanes([((BPH * X) * AGD), 0.0, 0.0]) + (BRB * AGF)) - (((BRD * KL) * (BOB / (BXF * AGH))) * AGJ)) / AGI) * AGL;
                let CGY = ((CEW - Lanes([(BYH * AGK), 0.0, 0.0, 0.0])) / PO) * AGJ;
                let AGN = ACO - C;
                let AGO = AGM * AGN;
                let CGZ = (Lanes([(BPI * AGN), 0.0, 0.0, 0.0]) + (BQH * AGM)) * AGP;
                let AGQ = C + AGP;
                let AGR = (AGO * AGP) / AGQ;
                let AGS = ((MG * AGD) + (AGJ * AGL)) + AGR;
                let CHA = (Lanes([CGW[0], CGW[1], CGW[2], 0.0, 0.0]) + (Lanes([CGX[0], CGX[1], CGX[2], 0.0, 0.0]) + Lanes([CGY[0], 0.0, CGY[1], CGY[2], CGY[3]]))) + (((Lanes([CGZ[0], 0.0, CGZ[1], CGZ[2], CGZ[3]]) + (BRE * AGO)) - (BRE * AGR)) / AGQ);
                AZX = AGS;
                BRC = CHA;
            } else {
                let AGU = if AGT == A { 1.0 } else { 0.0 };
                let AZY;
                let BRF;
                if AGU != 0.0 {
                    let AGV = AGC - C;
                    let AGW = MG * AGV;
                    let CGO = Lanes([(BXQ * AGV), 0.0, 0.0]) + (BRB * MG);
                    let CGP = Lanes([CGO[0], CGO[1], CGO[2], 0.0, 0.0]);
                    AZY = AGW;
                    BRF = CGP;
                } else {
                    let AGX = C - AGT;
                    let CGL = BRB * AGX;
                    let AGY = AGT * ((AGC + ACO) - X);
                    let AGZ = ACH / PO;
                    let AHA = C + AGZ;
                    let CGM = ((CEW - Lanes([(BYH * AGZ), 0.0, 0.0, 0.0])) / PO) * AGY;
                    let AHB = (AGX * (AGC - C)) + (AGY * AHA);
                    let AHC = MG * AHB;
                    let CGN = Lanes([(BXQ * AHB), 0.0, 0.0, 0.0, 0.0]) + ((Lanes([CGL[0], CGL[1], CGL[2], 0.0, 0.0]) + ((((Lanes([BRB[0], BRB[1], BRB[2], 0.0, 0.0]) + Lanes([BQH[0], 0.0, BQH[1], BQH[2], BQH[3]])) * AGT) * AHA) + Lanes([CGM[0], 0.0, CGM[1], CGM[2], CGM[3]]))) * MG);
                    AZY = AHC;
                    BRF = CGN;
                }
                AZX = AZY;
                BRC = BRF;
            }
            let AHD = SO * BR;
            let CHB = BZC * BR;
            let CHC = Lanes([0.0, CHB[0], CHB[1]]) + Lanes([(BUL * SO), 0.0, 0.0]);
            let AHE = AHD / MI;
            let CHD = CHC / MI;
            let AHF = if AHE < TH { 1.0 } else { 0.0 };
            let AHP;
            let BRG;
            if AHF != 0.0 {
                let AHG = AHE.exp();
                let CHF = CHD * AHG;
                AHP = AHG;
                BRG = CHF;
            } else {
                let AHH = TH.exp();
                let AHI = AHH * (C + (AHE - TH));
                let CHE = CHD * AHH;
                AHP = AHI;
                BRG = CHE;
            }
            let BAC;
            let BRH;
            if MN != 0.0 {
                let AHJ = SO - AFO;
                let AHK = AHJ * BR;
                let CHH = ((Lanes([0.0, BZC[0], BZC[1]]) - Lanes([BOY, 0.0, 0.0])) * BR) + Lanes([(BUL * AHJ), 0.0, 0.0]);
                let AHL = if AHK < TH { 1.0 } else { 0.0 };
                let AHT;
                let BRI;
                if AHL != 0.0 {
                    let AHM = AHK.exp();
                    let CHJ = CHH * AHM;
                    AHT = AHM;
                    BRI = CHJ;
                } else {
                    let AHN = TH.exp();
                    let AHO = AHN * (C + (AHK - TH));
                    let CHI = CHH * AHN;
                    AHT = AHO;
                    BRI = CHI;
                }
                let AHQ = AHP - C;
                let AHS = AHR * X;
                let AHU = (C + (KL * AHT)).sqrt();
                let AHV = C + AHU;
                let AHW = (AHS * AHQ) / AHV;
                let AHX = (MM * AHQ) + AHW;
                let CHK = (Lanes([(BXR * AHQ), 0.0, 0.0]) + (BRG * MM)) + (((Lanes([((BPJ * X) * AHQ), 0.0, 0.0]) + (BRG * AHS)) - (((BRI * KL) * (BOB / (BXF * AHU))) * AHW)) / AHV);
                BAC = AHX;
                BRH = CHK;
            } else {
                let AHY = AHP - C;
                let AHZ = MM * AHY;
                let CHG = Lanes([(BXR * AHY), 0.0, 0.0]) + (BRG * MM);
                BAC = AHZ;
                BRH = CHG;
            }
            let AIA = TM / LH;
            let CHL = CAB / LH;
            let AIB = if AIA < TH { 1.0 } else { 0.0 };
            let AIF;
            let BRJ;
            if AIB != 0.0 {
                let AIC = AIA.exp();
                let CHN = CHL * AIC;
                AIF = AIC;
                BRJ = CHN;
            } else {
                let AID = TH.exp();
                let AIE = AID * (C + (AIA - TH));
                let CHM = CHL * AID;
                AIF = AIE;
                BRJ = CHM;
            }
            let AIG = AIF - C;
            let AIH = LO * AIG;
            let CHO = Lanes([(BXM * AIG), 0.0, 0.0]) + (BRJ * LO);
            let AII = AHD / NI;
            let CHP = CHC / NI;
            let AIJ = if AII < TH { 1.0 } else { 0.0 };
            let AIN;
            let BRK;
            if AIJ != 0.0 {
                let AIK = AII.exp();
                let CHR = CHP * AIK;
                AIN = AIK;
                BRK = CHR;
            } else {
                let AIL = TH.exp();
                let AIM = AIL * (C + (AII - TH));
                let CHQ = CHP * AIL;
                AIN = AIM;
                BRK = CHQ;
            }
            let AIO = AIN - C;
            let AIP = NN * AIO;
            let CHS = Lanes([(BXW * AIO), 0.0, 0.0]) + (BRK * NN);
            let AIQ = TS / LQ;
            let CHT = CAG / LQ;
            let AIR = if AIQ < TH { 1.0 } else { 0.0 };
            let AIV;
            let BRL;
            if AIR != 0.0 {
                let AIS = AIQ.exp();
                let CHV = CHT * AIS;
                AIV = AIS;
                BRL = CHV;
            } else {
                let AIT = TH.exp();
                let AIU = AIT * (C + (AIQ - TH));
                let CHU = CHT * AIT;
                AIV = AIU;
                BRL = CHU;
            }
            let AIW = AIV - C;
            let AIX = LW * AIW;
            let CHW = Lanes([(BXN * AIW), 0.0, 0.0, 0.0, 0.0, 0.0]) + (BRL * LW);
            let AIY = AHD / NP;
            let CHX = CHC / NP;
            let AIZ = if AIY < TH { 1.0 } else { 0.0 };
            let AJD;
            let BRM;
            if AIZ != 0.0 {
                let AJA = AIY.exp();
                let CHZ = CHX * AJA;
                AJD = AJA;
                BRM = CHZ;
            } else {
                let AJB = TH.exp();
                let AJC = AJB * (C + (AIY - TH));
                let CHY = CHX * AJB;
                AJD = AJC;
                BRM = CHY;
            }
            let AJE = AJD - C;
            let AJF = NU * AJE;
            let CIA = Lanes([(BXX * AJE), 0.0, 0.0]) + (BRM * NU);
            let AJG = if (if (if OM > A { 1.0 } else { 0.0 }) != 0.0 && (if OG > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && AED != 0.0 { 1.0 } else { 0.0 };
            let AZZ;
            let BRN;
            if AJG != 0.0 {
                let AJH = X * AAS;
                let AJI = Z / AJH;
                let AJJ = C - AJI;
                let AJK = OL * AJJ;
                let CIC = Lanes([(BYB * AJJ), 0.0, 0.0]) + ((((((CEE * X) * AJI) * BUG) / AJH) * BUG) * OL);
                let AJL = if AJK < TH { 1.0 } else { 0.0 };
                let ALA;
                let BRO;
                if AJL != 0.0 {
                    let AJM = AJK.exp();
                    let CIE = CIC * AJM;
                    ALA = AJM;
                    BRO = CIE;
                } else {
                    let AJN = TH.exp();
                    let AJO = AJN * (C + (AJK - TH));
                    let CID = CIC * AJN;
                    ALA = AJO;
                    BRO = CID;
                }
                let AJP = SM * GV;
                let CIF = BZB * GV;
                let CIG = Lanes([0.0, CIF[0], CIF[1]]) + Lanes([(BWA * SM), 0.0, 0.0]);
                let CIH = CIG * AJP;
                let AJR = ((AJP * AJP) + AJQ).sqrt();
                let AJS = -2e0f64 - Y;
                let AJT = AJR.powf(AJS);
                let AJU = Y - C;
                let AJV = LG * AJP;
                let AJW = AJV * AJP;
                let AJX = AJU + AJP;
                let AJY = (Y * ((C - (Y * Y)) - ((CV * AJP) * AJU))) - (AJW * AJX);
                let AKA = (AJT * AJY) * AJZ;
                let AKB = SM * Z;
                let CII = (BZB * Z) * OL;
                let AKC = OB * AKA;
                let AKD = (AKB * OL) / AKC;
                let CIJ = ((Lanes([0.0, CII[0], CII[1]]) + Lanes([(BYB * AKB), 0.0, 0.0])) - ((Lanes([(BOR * AKA), 0.0, 0.0]) + (((((((CIH + CIH) * (BOB / (BXF * AJR))) * (AJS * (AJR.powf((AJS - BOB))))) * AJY) + ((((((CIG * CV) * AJU) * BUG) * Y) - (((((CIG * LG) * AJP) + (CIG * AJV)) * AJX) + (CIG * AJW))) * AJT)) * AJZ) * OB)) * AKD)) / AKC;
                let AKE = if AKD < -1e-3f64 { 1.0 } else { 0.0 };
                let AKX;
                let BRP;
                if AKE != 0.0 {
                    let AKF = if AKD < TH { 1.0 } else { 0.0 };
                    let AKK;
                    let BRQ;
                    if AKF != 0.0 {
                        let AKG = AKD.exp();
                        let CIN = CIJ * AKG;
                        AKK = AKG;
                        BRQ = CIN;
                    } else {
                        let AKH = TH.exp();
                        let AKI = AKH * (C + (AKD - TH));
                        let CIM = CIJ * AKH;
                        AKK = AKI;
                        BRQ = CIM;
                    }
                    let AKJ = -SM;
                    let AKL = (C - AKK) / AKD;
                    let AKM = C + AKL;
                    let AKN = AKJ * AKM;
                    let CIO = (BZB * BUG) * AKM;
                    let CIP = Lanes([0.0, CIO[0], CIO[1]]) + ((((BRQ * BUG) - (CIJ * AKL)) / AKD) * AKJ);
                    AKX = AKN;
                    BRP = CIP;
                } else {
                    let AKO = SM * KE;
                    let AKP = AKO * AKD;
                    let CIK = (BZB * KE) * AKD;
                    let AKR = AKD * AKQ;
                    let AKT = C + (AKS * AKD);
                    let AKU = C + (AKR * AKT);
                    let AKV = AKP * AKU;
                    let CIL = ((Lanes([0.0, CIK[0], CIK[1]]) + (CIJ * AKO)) * AKU) + ((((CIJ * AKQ) * AKT) + ((CIJ * AKS) * AKR)) * AKP);
                    AKX = AKV;
                    BRP = CIL;
                }
                let AKW = X * OS;
                let AKY = AKW * AKX;
                let AKZ = AKY * AAS;
                let ALB = AKZ * ALA;
                let ALC = (ALB * GV) * AA;
                let CIQ = (((((((Lanes([((BYC * X) * AKX), 0.0, 0.0]) + (BRP * AKW)) * AAS) + (CEE * AKY)) * ALA) + (BRO * AKZ)) * GV) + Lanes([(BWA * ALB), 0.0, 0.0])) * AA;
                AZZ = ALC;
                BRN = CIQ;
            } else {
                AZZ = A;
                BRN = CIB;
            }
            let ALD = if (if (if PE > A { 1.0 } else { 0.0 }) != 0.0 && (if OY > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if SI < A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let ASF;
            let BRR;
            if ALD != 0.0 {
                let ALE = SI * GX;
                let CIR = BYZ * GX;
                let CIS = Lanes([0.0, CIR[0], CIR[1]]) + Lanes([(BWB * SI), 0.0, 0.0]);
                let ALF = C - ALE;
                let ALG = ALF.powf(ABX);
                let CIT = (CIS * BUG) * (ABX * (ALF.powf(CET)));
                let ALH = X * ALG;
                let ALI = AQ / ALH;
                let ALJ = C - ALI;
                let ALK = PD * ALJ;
                let CIU = Lanes([(BYE * ALJ), 0.0, 0.0]) + ((((((CIT * X) * ALI) * BUG) / ALH) * BUG) * PD);
                let ALL = if ALK < TH { 1.0 } else { 0.0 };
                let AMV;
                let BRS;
                if ALL != 0.0 {
                    let ALM = ALK.exp();
                    let CIW = CIU * ALM;
                    AMV = ALM;
                    BRS = CIW;
                } else {
                    let ALN = TH.exp();
                    let ALO = ALN * (C + (ALK - TH));
                    let CIV = CIU * ALN;
                    AMV = ALO;
                    BRS = CIV;
                }
                let CIX = CIS * ALE;
                let ALP = ((ALE * ALE) + AJQ).sqrt();
                let ALQ = -2e0f64 - AP;
                let ALR = ALP.powf(ALQ);
                let ALS = AP - C;
                let ALT = LG * ALE;
                let ALU = ALT * ALE;
                let ALV = ALS + ALE;
                let ALW = (AP * ((C - (AP * AP)) - ((CV * ALE) * ALS))) - (ALU * ALV);
                let ALX = (ALR * ALW) * AJZ;
                let ALY = SI * AQ;
                let CIY = (BYZ * AQ) * PD;
                let ALZ = OT * ALX;
                let AMA = (ALY * PD) / ALZ;
                let CIZ = ((Lanes([0.0, CIY[0], CIY[1]]) + Lanes([(BYE * ALY), 0.0, 0.0])) - ((Lanes([(BOS * ALX), 0.0, 0.0]) + (((((((CIX + CIX) * (BOB / (BXF * ALP))) * (ALQ * (ALP.powf((ALQ - BOB))))) * ALW) + ((((((CIS * CV) * ALS) * BUG) * AP) - (((((CIS * LG) * ALE) + (CIS * ALT)) * ALV) + (CIS * ALU))) * ALR)) * AJZ) * OT)) * AMA)) / ALZ;
                let AMB = if AMA < -1e-3f64 { 1.0 } else { 0.0 };
                let AMS;
                let BRT;
                if AMB != 0.0 {
                    let AMC = if AMA < TH { 1.0 } else { 0.0 };
                    let AMH;
                    let BRU;
                    if AMC != 0.0 {
                        let AMD = AMA.exp();
                        let CJD = CIZ * AMD;
                        AMH = AMD;
                        BRU = CJD;
                    } else {
                        let AME = TH.exp();
                        let AMF = AME * (C + (AMA - TH));
                        let CJC = CIZ * AME;
                        AMH = AMF;
                        BRU = CJC;
                    }
                    let AMG = -SI;
                    let AMI = (C - AMH) / AMA;
                    let AMJ = C + AMI;
                    let AMK = AMG * AMJ;
                    let CJE = (BYZ * BUG) * AMJ;
                    let CJF = Lanes([0.0, CJE[0], CJE[1]]) + ((((BRU * BUG) - (CIZ * AMI)) / AMA) * AMG);
                    AMS = AMK;
                    BRT = CJF;
                } else {
                    let AML = SI * KE;
                    let AMM = AML * AMA;
                    let CJA = (BYZ * KE) * AMA;
                    let AMN = AMA * AKQ;
                    let AMO = C + (AKS * AMA);
                    let AMP = C + (AMN * AMO);
                    let AMQ = AMM * AMP;
                    let CJB = ((Lanes([0.0, CJA[0], CJA[1]]) + (CIZ * AML)) * AMP) + ((((CIZ * AKQ) * AMO) + ((CIZ * AKS) * AMN)) * AMM);
                    AMS = AMQ;
                    BRT = CJB;
                }
                let AMR = X * PK;
                let AMT = AMR * AMS;
                let AMU = AMT * ALG;
                let AMW = AMU * AMV;
                let AMX = (AMW * GX) * AR;
                let CJG = (((((((Lanes([((BYF * X) * AMS), 0.0, 0.0]) + (BRT * AMR)) * ALG) + (CIT * AMT)) * AMV) + (BRS * AMU)) * GX) + Lanes([(BWB * AMW), 0.0, 0.0])) * AR;
                ASF = AMX;
                BRR = CJG;
            } else {
                ASF = A;
                BRR = CBW;
            }
            let AMZ = ACI * AMY;
            let CJH = Lanes([(CEX * AMY), 0.0, 0.0, 0.0, 0.0, 0.0]) + (BPT * ACI);
            let ANB = KL * ANA;
            let CJI = BQA * KL;
            let ANC = (C + AMZ).sqrt();
            let AND = C + ANC;
            let ANE = (AMZ - ACI) / AND;
            let CJJ = ((CJH - Lanes([CEX, 0.0, 0.0, 0.0, 0.0, 0.0])) - ((CJH * (BOB / (BXF * ANC))) * ANE)) / AND;
            let ANF = (C + ANB).sqrt();
            let ANG = C + ANF;
            let ANH = ANB / ANG;
            let CJK = (CJI - ((CJI * (BOB / (BXF * ANF))) * ANH)) / ANG;
            let ANI = X * NG;
            let CJL = BXV * X;
            let ANJ = AMY - C;
            let ANK = (KL * NG) / LE;
            let CJM = ((BXV * KL) - (BXK * ANK)) / LE;
            let ANL = (C + (ANK * AMY)).sqrt();
            let ANM = C + ANL;
            let ANN = (ANI * ANJ) / ANM;
            let CJN = ((Lanes([(CJL * ANJ), 0.0, 0.0, 0.0, 0.0, 0.0]) + (BPT * ANI)) - (((Lanes([(CJM * AMY), 0.0, 0.0, 0.0, 0.0, 0.0]) + (BPT * ANK)) * (BOB / (BXF * ANL))) * ANN)) / ANM;
            let ANO = if parameters[8] == C { 1.0 } else { 0.0 };
            let APK;
            let BAI;
            let BRV;
            let BRW;
            if ANO != 0.0 {
                let ANQ = ANP * X;
                let ANR = ANQ * PZ;
                let ANT = ZT - ANS;
                let CJT = Lanes([0.0, BPR[0], BPR[1], 0.0, BPR[2]]);
                let ANU = PZ / QI;
                let ANV = KL * ANU;
                let CJU = ((BYK - (BYM * ANU)) / QI) * KL;
                let CJV = BPW * ANW;
                let ANX = ZT + (ANW * ANS);
                let ANY = (C + (ANV * ANX)).sqrt();
                let ANZ = C + ANY;
                let AOA = (ANR * ANT) / ANZ;
                let CJW = ((Lanes([0.0, ((BYK * ANQ) * ANT), 0.0, 0.0, 0.0]) + ((CJT - Lanes([BPW[0], BPW[1], 0.0, BPW[2], 0.0])) * ANR)) - (((Lanes([0.0, (CJU * ANX), 0.0, 0.0, 0.0]) + ((CJT + Lanes([CJV[0], CJV[1], 0.0, CJV[2], 0.0])) * ANV)) * (BOB / (BXF * ANY))) * AOA)) / ANZ;
                let AOB = (C - ANP) * X;
                let AOC = AOB * PZ;
                let AOE = AMY - AOD;
                let CJX = Lanes([0.0, BPT[0], BPT[1], BPT[2], BPT[3], BPT[4], BPT[5]]);
                let CJY = BPY * ANW;
                let AOF = AMY + (ANW * AOD);
                let AOG = (C + (ANV * AOF)).sqrt();
                let AOH = C + AOG;
                let AOI = (AOC * AOE) / AOH;
                let CJZ = ((Lanes([0.0, ((BYK * AOB) * AOE), 0.0, 0.0, 0.0, 0.0, 0.0]) + ((CJX - Lanes([BPY[0], BPY[1], 0.0, 0.0, BPY[2], 0.0, BPY[3]])) * AOC)) - (((Lanes([0.0, (CJU * AOF), 0.0, 0.0, 0.0, 0.0, 0.0]) + ((CJX + Lanes([CJY[0], CJY[1], 0.0, 0.0, CJY[2], 0.0, CJY[3]])) * ANV)) * (BOB / (BXF * AOG))) * AOI)) / AOH;
                APK = AOI;
                BAI = AOA;
                BRV = CJZ;
                BRW = CJW;
            } else {
                let AOJ = ANP * X;
                let AOK = AOJ * PZ;
                let AOL = ZT - C;
                let AOM = PZ / QI;
                let AON = KL * AOM;
                let CJO = ((BYK - (BYM * AOM)) / QI) * KL;
                let AOO = (C + (AON * ZT)).sqrt();
                let AOP = C + AOO;
                let AOQ = (AOK * AOL) / AOP;
                let CJP = ((Lanes([((BYK * AOJ) * AOL), 0.0, 0.0]) + (BPR * AOK)) - (((Lanes([(CJO * ZT), 0.0, 0.0]) + (BPR * AON)) * (BOB / (BXF * AOO))) * AOQ)) / AOP;
                let AOR = (C - ANP) * X;
                let AOS = AOR * PZ;
                let AOT = (C + (AON * AMY)).sqrt();
                let AOU = C + AOT;
                let AOV = (AOS * ANJ) / AOU;
                let CJQ = ((Lanes([((BYK * AOR) * ANJ), 0.0, 0.0, 0.0, 0.0, 0.0]) + (BPT * AOS)) - (((Lanes([(CJO * AMY), 0.0, 0.0, 0.0, 0.0, 0.0]) + (BPT * AON)) * (BOB / (BXF * AOT))) * AOV)) / AOU;
                let CJR = Lanes([0.0, CJQ[0], CJQ[1], CJQ[2], CJQ[3], CJQ[4], CJQ[5]]);
                let CJS = Lanes([0.0, CJP[0], CJP[1], 0.0, CJP[2]]);
                APK = AOV;
                BAI = AOQ;
                BRV = CJR;
                BRW = CJS;
            }
            let AOW = X * QE;
            let AOX = ANS - C;
            let AOY = ANW * KL;
            let AOZ = QE / QM;
            let APA = AOY * AOZ;
            let APB = (C + (APA * ANS)).sqrt();
            let APC = C + APB;
            let APD = (AOW * AOX) / APC;
            let CKA = BZE * APE;
            let APF = APD + (SQ * APE);
            let CKB = (((Lanes([0.0, ((BYL * X) * AOX), 0.0]) + (BPW * AOW)) - (((Lanes([0.0, ((((BYL - (BYN * AOZ)) / QM) * AOY) * ANS), 0.0]) + (BPW * APA)) * (BOB / (BXF * APB))) * APD)) / APC) + Lanes([CKA[0], 0.0, CKA[1]]);
            let APH = if I > A { 1.0 } else { 0.0 };
            let API = if (if APG > A { 1.0 } else { 0.0 }) != 0.0 && APH != 0.0 { 1.0 } else { 0.0 };
            let ASJ;
            let ASM;
            let BAG;
            let BAK;
            let BGM;
            let BRX;
            let BRY;
            let BRZ;
            let BSA;
            let BSB;
            if API != 0.0 {
                let APJ = ANN * J;
                let CKD = CJN * J;
                let APL = APK * J;
                let CKE = BRV * J;
                let APM = I * X;
                let APN = APM * NG;
                let APP = APO - C;
                let APQ = (C + (ANK * APO)).sqrt();
                let APR = C + APQ;
                let APS = (APN * APP) / APR;
                let CKF = ((Lanes([0.0, 0.0, ((BXV * APM) * APP), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]) + (BPV * APN)) - (((Lanes([0.0, 0.0, (CJM * APO), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]) + (BPV * ANK)) * (BOB / (BXF * APQ))) * APS)) / APR;
                let AQX;
                let BSC;
                if ANO != 0.0 {
                    let APT = ((C - ANP) * I) * X;
                    let APU = APT * PZ;
                    let APW = APO - APV;
                    let CKI = Lanes([BPV[0], BPV[1], 0.0, BPV[2], BPV[3], BPV[4], BPV[5], BPV[6], BPV[7], BPV[8]]);
                    let APX = (KL * PZ) / QI;
                    let CKJ = BPX * ANW;
                    let APY = APO + (ANW * APV);
                    let APZ = (C + (APX * APY)).sqrt();
                    let AQA = C + APZ;
                    let AQB = (APU * APW) / AQA;
                    let CKK = ((Lanes([0.0, 0.0, 0.0, ((BYK * APT) * APW), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]) + ((CKI - Lanes([0.0, 0.0, BPX[0], BPX[1], 0.0, 0.0, BPX[2], 0.0, BPX[3], BPX[4]])) * APU)) - (((Lanes([0.0, 0.0, 0.0, ((((BYK * KL) - (BYM * APX)) / QI) * APY), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]) + ((CKI + Lanes([0.0, 0.0, CKJ[0], CKJ[1], 0.0, 0.0, CKJ[2], 0.0, CKJ[3], CKJ[4]])) * APX)) * (BOB / (BXF * APZ))) * AQB)) / AQA;
                    AQX = AQB;
                    BSC = CKK;
                } else {
                    let AQC = ((C - ANP) * I) * X;
                    let AQD = AQC * PZ;
                    let AQE = (KL * PZ) / QI;
                    let AQF = (C + (AQE * APO)).sqrt();
                    let AQG = C + AQF;
                    let AQH = (AQD * APP) / AQG;
                    let CKG = ((Lanes([0.0, 0.0, ((BYK * AQC) * APP), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]) + (BPV * AQD)) - (((Lanes([0.0, 0.0, ((((BYK * KL) - (BYM * AQE)) / QI) * APO), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]) + (BPV * AQE)) * (BOB / (BXF * AQF))) * AQH)) / AQG;
                    let CKH = Lanes([CKG[0], CKG[1], 0.0, CKG[2], CKG[3], CKG[4], CKG[5], CKG[6], CKG[7], CKG[8]]);
                    AQX = AQH;
                    BSC = CKH;
                }
                let AQI = if APG == C { 1.0 } else { 0.0 };
                let ARB;
                let BSD;
                if AQI != 0.0 {
                    let AQJ = I * (NG + PZ);
                    let AQK = AQJ * IL;
                    let CKL = (((BXV + BYK) * I) * IL) + (BWP * AQJ);
                    let AQL = AQK * BR;
                    let AQM = X - (AQL.ln());
                    let AQN = TD - (BQ * AQM);
                    let CKM = CBD - Lanes([0.0, 0.0, ((BUK * AQM) + (((((CKL * BR) + (BUL * AQK)) * (BOB / AQL)) * BUG) * BQ)), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
                    let AQP = AQN * AQN;
                    let CKN = CKM * AQN;
                    let CKO = CKN + CKN;
                    let AQQ = if AQN < A { 1.0 } else { 0.0 };
                    let AQW;
                    let BSE;
                    if AQQ != 0.0 {
                        let AQR = (AQP + AQO).sqrt();
                        let AQS = AQR - AQN;
                        let AQT = 6.05e-3f64 / AQS;
                        let CKQ = ((((CKO * (BOB / (BXF * AQR))) - CKM) * AQT) * BUG) / AQS;
                        AQW = AQT;
                        BSE = CKQ;
                    } else {
                        let AQU = (AQP + AQO).sqrt();
                        let AQV = KE * (AQU + AQN);
                        let CKP = ((CKO * (BOB / (BXF * AQU))) + CKM) * KE;
                        AQW = AQV;
                        BSE = CKP;
                    }
                    let AQY = APS + AQX;
                    let AQZ = (AQK + (AQY * IL)) + AQW;
                    let CKR = Lanes([BSE[0], BSE[1], 0.0, BSE[2], BSE[3], BSE[4], BSE[5], BSE[6], BSE[7], BSE[8]]);
                    let ARA = AQW / AQZ;
                    let CKS = (CKR - (((Lanes([0.0, 0.0, 0.0, CKL, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]) + (((Lanes([CKF[0], CKF[1], 0.0, CKF[2], CKF[3], CKF[4], CKF[5], CKF[6], CKF[7], CKF[8]]) + BSC) * IL) + Lanes([0.0, 0.0, 0.0, (BWP * AQY), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]))) + CKR) * ARA)) / AQZ;
                    ARB = ARA;
                    BSD = CKS;
                } else {
                    ARB = C;
                    BSD = CKC;
                }
                let ARC = ARB * APS;
                let CKT = CKF * ARB;
                let CKU = (BSD * APS) + Lanes([CKT[0], CKT[1], 0.0, CKT[2], CKT[3], CKT[4], CKT[5], CKT[6], CKT[7], CKT[8]]);
                let ARD = ARB * AQX;
                let CKV = (BSD * AQX) + (BSC * ARB);
                ASJ = APJ;
                ASM = ARC;
                BAG = APL;
                BAK = ARD;
                BGM = ARB;
                BRX = CKD;
                BRY = CKU;
                BRZ = CKE;
                BSA = CKV;
                BSB = BSD;
            } else {
                ASJ = ANN;
                ASM = A;
                BAG = APK;
                BAK = A;
                BGM = C;
                BRX = CJN;
                BRY = CKC;
                BRZ = BRV;
                BSA = CKC;
                BSB = CKC;
            }
            let ARE = if parameters[84] == C { 1.0 } else { 0.0 };
            let ASG;
            let BSF;
            if ARE != 0.0 {
                let ARF = SP + SI;
                let CKX = Lanes([BZD[0], BZD[1], 0.0]) + Lanes([0.0, BYZ[0], BYZ[1]]);
                let ARK = (ARI * ARF) * ARJ;
                let ARL = ARK * ARF;
                let CKY = (((CKX * ARI) * ARJ) * ARF) + (CKX * ARK);
                let ARM = if (-1e0f64 * ARF) < A { 1.0 } else { 0.0 };
                let ARZ;
                let BSG;
                if ARM != 0.0 {
                    let ARN = (ARL + ARH).sqrt();
                    let ARP = ARN - (ARO * ARF);
                    let ARQ = 5e-13f64 / ARP;
                    let CLA = ((((CKY * (BOB / (BXF * ARN))) - (CKX * ARO)) * ARQ) * BUG) / ARP;
                    ARZ = ARQ;
                    BSG = CLA;
                } else {
                    let ARR = (ARL + ARH).sqrt();
                    let ART = KE * (ARR + (ARS * ARF));
                    let CKZ = ((CKY * (BOB / (BXF * ARR))) + (CKX * ARS)) * KE;
                    ARZ = ART;
                    BSG = CKZ;
                }
                let ARV = C / (C - (BC.powf(ARU)));
                let ARX = BC * ARW;
                let ARY = (((ARV * ARV) * (BC.powf((ARU - C)))) * ARU) / ARW;
                let ASA = if ARZ < ARX { 1.0 } else { 0.0 };
                let ASH;
                let BSH;
                if ASA != 0.0 {
                    let ASB = ARZ / ARW;
                    let ASC = C - (ASB.powf(ARU));
                    let ASD = C / ASC;
                    let CLC = (((((BSG / ARW) * (ARU * (ASB.powf((ARU - BOB))))) * BUG) * ASD) * BUG) / ASC;
                    ASH = ASD;
                    BSH = CLC;
                } else {
                    let CLB = BSG * ARY;
                    let ASE = ARV + ((ARZ - ARX) * ARY);
                    ASH = ASE;
                    BSH = CLB;
                }
                ASG = ASH;
                BSF = BSH;
            } else {
                ASG = C;
                BSF = CKW;
            }
            let ASI = ASF * ASG;
            let CLD = BRR * ASG;
            let CLE = BSF * ASF;
            let CLF = Lanes([CLD[0], 0.0, CLD[1], CLD[2]]) + Lanes([0.0, CLE[0], CLE[1], CLE[2]]);
            let ASK = ASJ * ASG;
            let CLG = BSF * ASJ;
            let CLH = (BRX * ASG) + Lanes([0.0, CLG[0], CLG[1], CLG[2], 0.0, 0.0]);
            let ASL = AIX * ASG;
            let CLI = BSF * AIX;
            let CLJ = (CHW * ASG) + Lanes([0.0, CLI[0], CLI[1], CLI[2], 0.0, 0.0]);
            let ASN = ASM * ASG;
            let CLK = BSF * ASM;
            let CLL = (BRY * ASG) + Lanes([0.0, 0.0, 0.0, 0.0, CLK[0], CLK[1], CLK[2], 0.0, 0.0, 0.0]);
            let ASO = AAV / PR;
            let CLM = (CEG - Lanes([(BYI * ASO), 0.0, 0.0])) / PR;
            let ASP = ACH / PO;
            let CLN = (CEW - Lanes([(BYH * ASP), 0.0, 0.0, 0.0])) / PO;
            let ASQ = (C + ASO) + ASP;
            let CLO = Lanes([CLM[0], CLM[1], CLM[2], 0.0, 0.0]) + Lanes([CLN[0], 0.0, CLN[1], CLN[2], CLN[3]]);
            let ASS = ASQ * ASQ;
            let CLP = CLO * ASQ;
            let CLQ = CLP + CLP;
            let AST = if ASQ < A { 1.0 } else { 0.0 };
            let ASZ;
            let BSI;
            if AST != 0.0 {
                let ASU = (ASS + ASR).sqrt();
                let ASV = ASU - ASQ;
                let ASW = 5.000000000000001e-3f64 / ASV;
                let CLS = ((((CLQ * (BOB / (BXF * ASU))) - CLO) * ASW) * BUG) / ASV;
                ASZ = ASW;
                BSI = CLS;
            } else {
                let ASX = (ASS + ASR).sqrt();
                let ASY = KE * (ASX + ASQ);
                let CLR = ((CLQ * (BOB / (BXF * ASX))) + CLO) * KE;
                ASZ = ASY;
                BSI = CLR;
            }
            let ATA = ASZ * ADU;
            let ATB = IC / ATA;
            let CLT = (Lanes([BWN, 0.0, 0.0, 0.0, 0.0]) - (((BSI * ADU) + (CFP * ASZ)) * ATB)) / ATA;
            let ATC = if ATB < S { 1.0 } else { 0.0 };
            let ATD;
            let BSJ;
            if ATC != 0.0 {
                ATD = S;
                BSJ = CLU;
            } else {
                ATD = ATB;
                BSJ = CLT;
            }
            let ATE = CV * ATD;
            let CLV = BSJ * CV;
            let ATF = X * BQ;
            let ATH = ATG - C;
            let CLW = (Lanes([((BUK * X) * ATH), 0.0, 0.0]) + (BPU * ATF)) + Lanes([0.0, BZD[0], BZD[1]]);
            let ATI = ((ATF * ATH) + SP) / ATE;
            let CLX = CLV * ATI;
            let CLY = (Lanes([CLW[0], 0.0, CLW[1], CLW[2], 0.0, 0.0]) - Lanes([CLX[0], CLX[1], 0.0, CLX[2], CLX[3], CLX[4]])) / ATE;
            let ATJ = if AEA > A { 1.0 } else { 0.0 };
            let AZI;
            let BSK;
            if ATJ != 0.0 {
                let ATL = if ATK == C { 1.0 } else { 0.0 };
                let AXX;
                let BSL;
                if ATL != 0.0 {
                    let ATN = if SI < ATM { 1.0 } else { 0.0 };
                    let AXY;
                    let BSM;
                    if ATN != 0.0 {
                        let ATP = (-AEA) / ATO;
                        let CNN = (CFV * BUG) / ATO;
                        let ATQ = if ATP < TH { 1.0 } else { 0.0 };
                        let ATV;
                        let BSN;
                        if ATQ != 0.0 {
                            let ATR = ATP.exp();
                            let CNP = CNN * ATR;
                            ATV = ATR;
                            BSN = CNP;
                        } else {
                            let ATS = TH.exp();
                            let ATT = ATS * (C + (ATP - TH));
                            let CNO = CNN * ATS;
                            ATV = ATT;
                            BSN = CNO;
                        }
                        let ATU = ATM - SI;
                        let ATW = ATU * ATV;
                        let CNQ = (BYZ * BUG) * ATV;
                        let CNR = Lanes([0.0, 0.0, CNQ[0], CNQ[1], 0.0]) + (BSN * ATU);
                        let ATY = -ATX;
                        let AUA = ATW.powf(ATZ);
                        let AUB = ATY * AUA;
                        let CNS = Lanes([((BPG * BUG) * AUA), 0.0, 0.0, 0.0, 0.0]) + ((CNR * (ATZ * (ATW.powf((ATZ - BOB))))) * ATY);
                        let AUC = if AUB < TH { 1.0 } else { 0.0 };
                        let AUJ;
                        let BSO;
                        if AUC != 0.0 {
                            let AUD = AUB.exp();
                            let CNU = CNS * AUD;
                            AUJ = AUD;
                            BSO = CNU;
                        } else {
                            let AUE = TH.exp();
                            let AUF = AUE * (C + (AUB - TH));
                            let CNT = CNS * AUE;
                            AUJ = AUF;
                            BSO = CNT;
                        }
                        let AUH = AUG / ATX;
                        let AUI = AUH * ATW;
                        let AUK = AUI * AUJ;
                        let CNV = ((Lanes([((((BPG * AUH) * BUG) / ATX) * ATW), 0.0, 0.0, 0.0, 0.0]) + (CNR * AUH)) * AUJ) + (BSO * AUI);
                        AXY = AUK;
                        BSM = CNV;
                    } else {
                        AXY = A;
                        BSM = CLU;
                    }
                    AXX = AXY;
                    BSL = BSM;
                } else {
                    let AUL = if ATK == X { 1.0 } else { 0.0 };
                    let AXZ;
                    let BSP;
                    if AUL != 0.0 {
                        let AUM = if SI < UW { 1.0 } else { 0.0 };
                        let AYA;
                        let BSQ;
                        if AUM != 0.0 {
                            let AUO = (X * parameters[46]) / (AUN * AUN);
                            let AUP = UW - SI;
                            let CMN = CBP - CBO;
                            let AUQ = AUP / ABU;
                            let CMO = Lanes([CMN[0], CMN[1], CMN[2], 0.0]);
                            let AUR = ((X * AUQ) / AUO).sqrt();
                            let CMP = ((((CMO - (BQG * AUQ)) / ABU) * X) / AUO) * (BOB / (BXF * AUR));
                            let AUT = if AUS == A { 1.0 } else { 0.0 };
                            let AUZ;
                            let BSR;
                            if AUT != 0.0 {
                                AUZ = AUN;
                                BSR = CCK;
                            } else {
                                let AUW = C - (KE * AUU);
                                let CMQ = (BQI * KE) * BUG;
                                let AUX = AUN * AUW;
                                let AUY = AUX * AUW;
                                let CMR = ((CMQ * AUN) * AUW) + (CMQ * AUX);
                                AUZ = AUY;
                                BSR = CMR;
                            }
                            let CMS = CMP * AUR;
                            let CMT = BSR * AUZ;
                            let AVA = ((AUR * AUR) + (AUZ * AUZ)).sqrt();
                            let AVB = (AUR * AUZ) / AVA;
                            let CMU = (((CMP * AUZ) + (BSR * AUR)) - ((((CMS + CMS) + (CMT + CMT)) * (BOB / (BXF * AVA))) * AVB)) / AVA;
                            let AVC = AUP / AVB;
                            let CMV = (CMO - (CMU * AVC)) / AVB;
                            let AVD = KE * AVB;
                            let CMW = CMU * KE;
                            let AVE = AVD * AUO;
                            let CMX = CMW * AUO;
                            let AVF = AVC + (AVE * ABU);
                            let CMY = CMV + ((CMX * ABU) + (BQG * AVE));
                            let AVS;
                            let BSS;
                            if AUT != 0.0 {
                                let CNG = Lanes([CMY[0], 0.0, CMY[1], CMY[2], CMY[3]]);
                                AVS = AVF;
                                BSS = CNG;
                            } else {
                                let AVH = X * AVG;
                                let AVI = XF * (C + (AVH * (C + (X * AUU))));
                                let AVJ = AEA / AVI;
                                let CMZ = (((BQI * X) * AVH) * XF) * AVJ;
                                let AVK = ((C + AVG) / (C + AVH)) - AVJ;
                                let CNA = CMX * AVK;
                                let AVL = AVC - (AVE * AVK);
                                let CNB = Lanes([CMV[0], 0.0, CMV[1], CMV[2], CMV[3]]) - (Lanes([CNA[0], 0.0, CNA[1], CNA[2], CNA[3]]) + ((((CFV - Lanes([CMZ[0], 0.0, CMZ[1], CMZ[2], CMZ[3]])) / AVI) * BUG) * AVE));
                                let AVM = AVL - AVF;
                                let CNC = Lanes([CMY[0], 0.0, CMY[1], CMY[2], CMY[3]]);
                                let CND = (CNB - CNC) * AVM;
                                let AVN = AG * AVC;
                                let AVO = AVN * AVC;
                                let CNE = (((((CMV * AG) * AVC) + (CMV * AVN)) * AVP) + (BQJ * AVO)) / XF;
                                let AVQ = ((AVM * AVM) + ((AVO * AVP) / XF)).sqrt();
                                let AVR = KE * ((AVL + AVF) + AVQ);
                                let CNF = ((CNB + CNC) + (((CND + CND) + Lanes([CNE[0], 0.0, CNE[1], CNE[2], CNE[3]])) * (BOB / (BXF * AVQ)))) * KE;
                                AVS = AVR;
                                BSS = CNF;
                            }
                            let AVT = (AVS - AVC) / AVS;
                            let CNH = ((BSS - Lanes([CMV[0], 0.0, CMV[1], CMV[2], CMV[3]])) - (BSS * AVT)) / AVS;
                            let AVU = if (AVT.abs()) > 1e-7f64 { 1.0 } else { 0.0 };
                            let AYB;
                            let BST;
                            if AVU != 0.0 {
                                let AVV = AVD / AVT;
                                let CNK = (Lanes([CMW[0], 0.0, CMW[1], CMW[2], CMW[3]]) - (CNH * AVV)) / AVT;
                                let AVY = AVW / AVX;
                                let AVZ = AVY * AVS;
                                let AWA = AVZ * AVV;
                                let AWB = (-AVX) / AVS;
                                let CNL = (Lanes([(BPK * BUG), 0.0, 0.0, 0.0, 0.0]) - (BSS * AWB)) / AVS;
                                let AWC = AWB.exp();
                                let AWD = AUZ / AVV;
                                let AWE = C + AWD;
                                let AWF = (AWB * AWE).exp();
                                let AWG = AWC - AWF;
                                let AWH = AWA * AWG;
                                let CNM = ((((Lanes([((((BPK * AVY) * BUG) / AVX) * AVS), 0.0, 0.0, 0.0, 0.0]) + (BSS * AVY)) * AVV) + (CNK * AVZ)) * AWG) + (((CNL * AWC) - (((CNL * AWE) + (((Lanes([BSR[0], 0.0, BSR[1], BSR[2], BSR[3]]) - (CNK * AWD)) / AVV) * AWB)) * AWF)) * AWA);
                                AYB = AWH;
                                BST = CNM;
                            } else {
                                let AWI = AVW * AUZ;
                                let AWJ = (-AVX) / AVS;
                                let AWK = AWJ.exp();
                                let AWL = AWI * AWK;
                                let CNI = (BSR * AVW) * AWK;
                                let CNJ = Lanes([CNI[0], 0.0, CNI[1], CNI[2], CNI[3]]) + ((((Lanes([(BPK * BUG), 0.0, 0.0, 0.0, 0.0]) - (BSS * AWJ)) / AVS) * AWK) * AWI);
                                AYB = AWL;
                                BST = CNJ;
                            }
                            AYA = AYB;
                            BSQ = BST;
                        } else {
                            AYA = A;
                            BSQ = CLU;
                        }
                        AXZ = AYA;
                        BSP = BSQ;
                    } else {
                        let AWM = if ATK == CV { 1.0 } else { 0.0 };
                        let AYC;
                        let BSU;
                        if AWM != 0.0 {
                            let AWN = if SI < ATM { 1.0 } else { 0.0 };
                            let AYD;
                            let BSV;
                            if AWN != 0.0 {
                                let AWO = ATM - SI;
                                let CLZ = BYZ * BUG;
                                let AWP = AWO.powf(ATZ);
                                let AWR = AWQ + AEA;
                                let AWS = AEA / AWR;
                                let AWT = C - AWS;
                                let AWV = AWT.powf(AWU);
                                let AWW = AWP * AWV;
                                let CMA = (CLZ * (ATZ * (AWO.powf((ATZ - BOB))))) * AWV;
                                let CMB = Lanes([0.0, 0.0, CMA[0], CMA[1], 0.0]) + (((((CFV - (CFV * AWS)) / AWR) * BUG) * (AWU * (AWT.powf((AWU - BOB))))) * AWP);
                                let AWX = if AUS == A { 1.0 } else { 0.0 };
                                let AXN;
                                let BSW;
                                if AWX != 0.0 {
                                    AXN = AWW;
                                    BSW = CMB;
                                } else {
                                    let AWY = (AEA - parameters[52]) / AWQ;
                                    let CMC = CFV / AWQ;
                                    let AXA = (AWY - C) / AWZ;
                                    let CMD = CMC / AWZ;
                                    let AXB = if AWY < C { 1.0 } else { 0.0 };
                                    let AXI;
                                    let BSX;
                                    if AXB != 0.0 {
                                        let AXC = AXA.exp();
                                        let AXD = C + AXC;
                                        let CMF = ((CMD * AXC) * (BOB / AXD)) * AWZ;
                                        let AXE = C + (AWZ * (AXD.ln()));
                                        AXI = AXE;
                                        BSX = CMF;
                                    } else {
                                        let AXF = (-AXA).exp();
                                        let AXG = C + AXF;
                                        let AXH = AWY + (AWZ * (AXG.ln()));
                                        let CME = CMC + ((((CMD * BUG) * AXF) * (BOB / AXG)) * AWZ);
                                        AXI = AXH;
                                        BSX = CME;
                                    }
                                    let AXK = AXI.powf(AXJ);
                                    let AXL = AWW * AXK;
                                    let CMG = (CMB * AXK) + ((BSX * (AXJ * (AXI.powf((AXJ - BOB))))) * AWW);
                                    AXN = AXL;
                                    BSW = CMG;
                                }
                                let AXM = -ATX;
                                let AXO = AXM * AXN;
                                let CMH = Lanes([((BPG * BUG) * AXN), 0.0, 0.0, 0.0, 0.0]) + (BSW * AXM);
                                let AXP = if AXO < TH { 1.0 } else { 0.0 };
                                let AXV;
                                let BSY;
                                if AXP != 0.0 {
                                    let AXQ = AXO.exp();
                                    let CMJ = CMH * AXQ;
                                    AXV = AXQ;
                                    BSY = CMJ;
                                } else {
                                    let AXR = TH.exp();
                                    let AXS = AXR * (C + (AXO - TH));
                                    let CMI = CMH * AXR;
                                    AXV = AXS;
                                    BSY = CMI;
                                }
                                let AXT = AUG / ATX;
                                let AXU = AXT * AWO;
                                let CMK = CLZ * AXT;
                                let AXW = AXU * AXV;
                                let CML = (Lanes([((((BPG * AXT) * BUG) / ATX) * AWO), 0.0, 0.0]) + Lanes([0.0, CMK[0], CMK[1]])) * AXV;
                                let CMM = Lanes([CML[0], 0.0, CML[1], CML[2], 0.0]) + (BSY * AXU);
                                AYD = AXW;
                                BSV = CMM;
                            } else {
                                AYD = A;
                                BSV = CLU;
                            }
                            AYC = AYD;
                            BSU = BSV;
                        } else {
                            AYC = A;
                            BSU = CLU;
                        }
                        AXZ = AYC;
                        BSP = BSU;
                    }
                    AXX = AXZ;
                    BSL = BSP;
                }
                let AYE = if AXX > A { 1.0 } else { 0.0 };
                let AZJ;
                let BSZ;
                if AYE != 0.0 {
                    let AYF = if parameters[53] == C { 1.0 } else { 0.0 };
                    let AZK;
                    let BTA;
                    if AYF != 0.0 {
                        let AYH = AYG + ATE;
                        let CNX = Lanes([BPB, 0.0, 0.0, 0.0, 0.0]) + CLV;
                        let AYI = AEA * AYH;
                        let AYJ = BQ / AYI;
                        let AYK = ADV / KV;
                        let AYM = AYL / AYH;
                        let AYN = (AYJ + (AYK * MG)) + AYM;
                        let CNY = (((Lanes([BUK, 0.0, 0.0, 0.0, 0.0]) - (((CFV * AYH) + (CNX * AEA)) * AYJ)) / AYI) + ((((CFQ - Lanes([(BXI * AYK), 0.0, 0.0, 0.0, 0.0])) / KV) * MG) + Lanes([(BXQ * AYK), 0.0, 0.0, 0.0, 0.0]))) + ((Lanes([BPA, 0.0, 0.0, 0.0, 0.0]) - (CNX * AYM)) / AYH);
                        let AYO = if ATK == CV { 1.0 } else { 0.0 };
                        let AZL;
                        let BTB;
                        if AYO != 0.0 {
                            let AYP = (AXX - AYN) / ARG;
                            let COA = (BSL - CNY) / ARG;
                            let AYQ = if AXX < AYN { 1.0 } else { 0.0 };
                            let AYX;
                            let BTC;
                            if AYQ != 0.0 {
                                let AYR = AYP.exp();
                                let AYS = C + AYR;
                                let AYT = AXX - (ARG * (AYS.ln()));
                                let COC = BSL - (((COA * AYR) * (BOB / AYS)) * ARG);
                                AYX = AYT;
                                BTC = COC;
                            } else {
                                let AYU = (-AYP).exp();
                                let AYV = C + AYU;
                                let AYW = AYN - (ARG * (AYV.ln()));
                                let COB = CNY - ((((COA * BUG) * AYU) * (BOB / AYV)) * ARG);
                                AYX = AYW;
                                BTC = COB;
                            }
                            let AYY = AEA * AYX;
                            let COD = (CFV * AYX) + (BTC * AEA);
                            AZL = AYY;
                            BTB = COD;
                        } else {
                            let AYZ = AEA * AXX;
                            let AZA = AXX + AYN;
                            let AZB = (AYZ * AYN) / AZA;
                            let CNZ = (((((CFV * AXX) + (BSL * AEA)) * AYN) + (CNY * AYZ)) - ((BSL + CNY) * AZB)) / AZA;
                            AZL = AZB;
                            BTB = CNZ;
                        }
                        AZK = AZL;
                        BTA = BTB;
                    } else {
                        let AZC = AEA * AXX;
                        let CNW = (CFV * AXX) + (BSL * AEA);
                        AZK = AZC;
                        BTA = CNW;
                    }
                    AZJ = AZK;
                    BSZ = BTA;
                } else {
                    AZJ = A;
                    BSZ = CLU;
                }
                AZI = AZJ;
                BSK = BSZ;
            } else {
                AZI = A;
                BSK = CLU;
            }
            let AZD = if ACO > A { 1.0 } else { 0.0 };
            let AZF;
            let BTD;
            if AZD != 0.0 {
                let AZE = BQ * CFB;
                let COF = Lanes([(BUK * CFB), 0.0, 0.0, 0.0]) + ((BQH * (BOB / ACO)) * BQ);
                AZF = AZE;
                BTD = COF;
            } else {
                let COE = Lanes([0.0, BZA[0], 0.0, BZA[1]]);
                AZF = SK;
                BTD = COE;
            }
            let BAB;
            let BTE;
            if MN != 0.0 {
                let COH = Lanes([BYZ[0], BYZ[1], 0.0]);
                BAB = SI;
                BTE = COH;
            } else {
                let COG = Lanes([BZA[0], 0.0, BZA[1]]);
                BAB = SK;
                BTE = COG;
            }
            let AZG = SM - AZF;
            let AZH = AZF - SI;
            let COI = (CCB * AZH) + ((BTD - Lanes([0.0, BYZ[0], BYZ[1], 0.0])) * WH);
            let COJ = BTD * AZI;
            let COK = (((CFV * AZG) + ((Lanes([0.0, BZB[0], BZB[1], 0.0, 0.0]) - Lanes([BTD[0], 0.0, BTD[1], BTD[2], BTD[3]])) * AEA)) + Lanes([COI[0], 0.0, COI[1], COI[2], COI[3]])) - ((BSK * AZF) + Lanes([COJ[0], 0.0, COJ[1], COJ[2], COJ[3]]));
            let COL = BZG * ST;
            let COM = COL + COL;
            let AZM = (ST * ST) / AYL;
            let CON = (Lanes([COM[0], 0.0, COM[1]]) - Lanes([0.0, (BPA * AZM), 0.0])) / AYL;
            let COO = Lanes([0.0, COK[0], COK[1], COK[2], COK[3], COK[4]]) + Lanes([CON[0], CON[1], CON[2], 0.0, 0.0, 0.0]);
            let AZN = TC * TC;
            let COP = BZS * TC;
            let COQ = (COP + COP) * AZO;
            let COR = Lanes([COQ[0], COQ[1], 0.0, COQ[2], COQ[3], COQ[4], COQ[5], COQ[6], COQ[7]]) + Lanes([0.0, 0.0, (BPL * AZN), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
            let AZQ = TA * TA;
            let COS = BZL * TA;
            let COT = (COS + COS) * AZR;
            let COU = Lanes([0.0, COT[0], COT[1]]) + Lanes([(BPN * AZQ), 0.0, 0.0]);
            let AZT = SZ * SZ;
            let COV = BZK * SZ;
            let COW = (COV + COV) * AZU;
            let COX = Lanes([0.0, COW[0], COW[1]]) + Lanes([(BPP * AZT), 0.0, 0.0]);
            let COY = BZH * SV;
            let COZ = COY + COY;
            let AZW = (SV * SV) / AYG;
            let CPA = (Lanes([COZ[0], 0.0, COZ[1]]) - Lanes([0.0, (BPB * AZW), 0.0])) / AYG;
            let CPB = BZD * ATI;
            let CPC = (CLY * SP) + Lanes([0.0, 0.0, CPB[0], CPB[1], 0.0, 0.0]);
            let CPD = BZB * M;
            let BAA = ((((AZX + AIH) + (M * SM)) - AZZ) + AFI) + AET;
            let CPE = ((((BRC + Lanes([CHO[0], CHO[1], CHO[2], 0.0, 0.0])) + Lanes([0.0, CPD[0], CPD[1], 0.0, 0.0])) - Lanes([BRN[0], BRN[1], BRN[2], 0.0, 0.0])) + Lanes([0.0, CGH[0], CGH[1], 0.0, 0.0])) + Lanes([CGD[0], CGD[1], CGD[2], 0.0, 0.0]);
            let CPF = BZB * BAA;
            let CPG = (CPE * SM) + Lanes([0.0, CPF[0], CPF[1], 0.0, 0.0]);
            let CPH = CLF * BAB;
            let CPI = BTE * ASI;
            let CPJ = Lanes([CPH[0], CPH[1], CPH[2], CPH[3], 0.0]) + Lanes([0.0, 0.0, CPI[0], CPI[1], CPI[2]]);
            let BAD = (BAC + AIP) + AJF;
            let CPK = (BRH + CHS) + CIA;
            let CPL = BZC * BAD;
            let CPM = (CPK * SO) + Lanes([0.0, CPL[0], CPL[1]]);
            let BAE = M * TB;
            let CPN = BZO * M;
            let BAF = (ASK + ASL) + BAE;
            let CPO = Lanes([0.0, CPN[0], CPN[1], CPN[2], CPN[3], CPN[4]]);
            let CPP = BZO * BAF;
            let CPQ = (((CLH + CLJ) + CPO) * TB) + Lanes([0.0, CPP[0], CPP[1], CPP[2], CPP[3], CPP[4]]);
            let CPR = ((((((((Lanes([0.0, 0.0, COO[0], COO[1], COO[2], 0.0, COO[3], COO[4], COO[5], 0.0, 0.0]) + Lanes([COR[0], COR[1], 0.0, COR[2], 0.0, COR[3], COR[4], COR[5], COR[6], COR[7], COR[8]])) + Lanes([0.0, 0.0, 0.0, COU[0], 0.0, 0.0, 0.0, 0.0, 0.0, COU[1], COU[2]])) + Lanes([0.0, 0.0, 0.0, COX[0], 0.0, 0.0, 0.0, COX[1], 0.0, 0.0, COX[2]])) + Lanes([0.0, CPA[0], 0.0, CPA[1], 0.0, CPA[2], 0.0, 0.0, 0.0, 0.0, 0.0])) + Lanes([0.0, 0.0, 0.0, CPC[0], CPC[1], CPC[2], CPC[3], CPC[4], CPC[5], 0.0, 0.0])) + Lanes([0.0, 0.0, 0.0, CPG[0], CPG[1], 0.0, CPG[2], CPG[3], CPG[4], 0.0, 0.0])) - Lanes([0.0, 0.0, 0.0, CPJ[0], 0.0, CPJ[1], CPJ[2], CPJ[3], CPJ[4], 0.0, 0.0])) + Lanes([0.0, 0.0, 0.0, CPM[0], CPM[1], CPM[2], 0.0, 0.0, 0.0, 0.0, 0.0])) + Lanes([0.0, 0.0, 0.0, CPQ[0], 0.0, CPQ[1], CPQ[2], CPQ[3], CPQ[4], 0.0, CPQ[5]]);
            let CPS = BZT * ASN;
            let CPT = (CLL * TD) + Lanes([CPS[0], CPS[1], 0.0, 0.0, CPS[2], CPS[3], CPS[4], CPS[5], CPS[6], CPS[7]]);
            let BAH = TB - TE;
            let CPU = (Lanes([0.0, BZO[0], BZO[1], BZO[2], BZO[3], BZO[4]]) - Lanes([BZU[0], 0.0, 0.0, BZU[1], 0.0, BZU[2]])) * BAG;
            let CPV = (BRZ * BAH) + Lanes([CPU[0], 0.0, CPU[1], CPU[2], CPU[3], CPU[4], CPU[5]]);
            let BAJ = SI - SQ;
            let CPW = (Lanes([0.0, BYZ[0], BYZ[1]]) - Lanes([BZE[0], 0.0, BZE[1]])) * BAI;
            let CPX = (BRW * BAJ) + Lanes([CPW[0], 0.0, CPW[1], CPW[2], 0.0]);
            let BAL = TD - TF;
            let CPY = (Lanes([BZT[0], BZT[1], 0.0, BZT[2], BZT[3], BZT[4], BZT[5], BZT[6], BZT[7]]) - Lanes([0.0, 0.0, BZV[0], 0.0, 0.0, BZV[1], 0.0, BZV[2], BZV[3]])) * BAK;
            let CPZ = (BSA * BAL) + Lanes([CPY[0], CPY[1], CPY[2], 0.0, CPY[3], CPY[4], CPY[5], CPY[6], CPY[7], CPY[8]]);
            let CQA = BZE * APF;
            let CQB = (CKB * SQ) + Lanes([CQA[0], 0.0, CQA[1]]);
            let BAM = (((((((((((((((((AEA * AZG) + (WH * AZH)) - (AZI * AZF)) + AZM) + (AZN * AZO)) + (AZQ * AZR)) + (AZT * AZU)) + AZW) + (ATI * SP)) + (BAA * SM)) - (ASI * BAB)) + (BAD * SO)) + (BAF * TB)) + (ASN * TD)) + (BAG * BAH)) + (BAI * BAJ)) + (BAK * BAL)) + (APF * SQ);
            let CQC = ((((Lanes([CPR[0], CPR[1], CPR[2], 0.0, CPR[3], CPR[4], CPR[5], CPR[6], CPR[7], CPR[8], CPR[9], CPR[10]]) + Lanes([CPT[0], CPT[1], 0.0, CPT[2], CPT[3], 0.0, CPT[4], CPT[5], CPT[6], CPT[7], CPT[8], CPT[9]])) + Lanes([0.0, 0.0, 0.0, CPV[0], CPV[1], 0.0, CPV[2], CPV[3], CPV[4], CPV[5], 0.0, CPV[6]])) + Lanes([0.0, 0.0, 0.0, CPX[0], CPX[1], 0.0, 0.0, CPX[2], CPX[3], CPX[4], 0.0, 0.0])) + Lanes([CPZ[0], CPZ[1], 0.0, CPZ[2], CPZ[3], 0.0, CPZ[4], CPZ[5], CPZ[6], CPZ[7], CPZ[8], CPZ[9]])) + Lanes([0.0, 0.0, 0.0, CQB[0], CQB[1], 0.0, 0.0, 0.0, CQB[2], 0.0, 0.0, 0.0]);
            let BAO = C - BAN;
            let BAP = BAO * HD;
            let CQD = BWF * BAO;
            let BAQ = BAP * AAV;
            let CQE = Lanes([(CQD * AAV), 0.0, 0.0]) + (CEG * BAP);
            let CQF = Lanes([0.0, BZC[0], BZC[1]]);
            let CQG = Lanes([CDV, 0.0, 0.0]);
            let BAR = (SO - AAD) / AAE;
            let CQH = ((CQF - CQG) - Lanes([(CDW * BAR), 0.0, 0.0])) / AAE;
            let BAS = if SO < AAD { 1.0 } else { 0.0 };
            let BBC;
            let BTF;
            if BAS != 0.0 {
                let BAT = BAR.exp();
                let BAU = C + BAT;
                let BAV = BAU.ln();
                let BAW = SO - (AAE * BAV);
                let CQJ = CQF - (Lanes([(CDW * BAV), 0.0, 0.0]) + (((CQH * BAT) * (BOB / BAU)) * AAE));
                BBC = BAW;
                BTF = CQJ;
            } else {
                let BAX = (-BAR).exp();
                let BAY = C + BAX;
                let BAZ = BAY.ln();
                let BBA = AAD - (AAE * BAZ);
                let CQI = CQG - (Lanes([(CDW * BAZ), 0.0, 0.0]) + ((((CQH * BUG) * BAX) * (BOB / BAY)) * AAE));
                BBC = BBA;
                BTF = CQI;
            }
            let BBB = BAN * HD;
            let BBD = C - (BBC * GV);
            let BBE = C - (BBD.powf(AAR));
            let BBF = (AAT * BBE) + (CV * (SO - BBC));
            let BBG = BBB * BBF;
            let CQK = Lanes([((BWF * BAN) * BBF), 0.0, 0.0]) + (((Lanes([(CEF * BBE), 0.0, 0.0]) + ((((((BTF * GV) + Lanes([(BWA * BBC), 0.0, 0.0])) * BUG) * (AAR * (BBD.powf(CED)))) * BUG) * AAT)) + ((CQF - BTF) * CV)) * BBB);
            let BBI = BBH * HQ;
            let BBJ = BBI * ACH;
            let CQL = Lanes([((BWJ * BBH) * ACH), 0.0, 0.0, 0.0]) + (CEW * BBI);
            let BBK = QX * KZ;
            let CQM = (BYP * KZ) + (BXJ * QX);
            let BBL = KE * BBK;
            let CQN = CQM * KE;
            let BBM = BBL * ACN;
            let BBN = BBM * ASZ;
            let CQO = (Lanes([(CQN * ACN), 0.0, 0.0]) + (CFA * BBL)) * ASZ;
            let CQP = Lanes([CQO[0], CQO[1], CQO[2], 0.0, 0.0]) + (BSI * BBM);
            let BBO = BBL * ACV;
            let BBP = BBO * ASZ;
            let CQQ = (Lanes([(CQN * ACV), 0.0, 0.0, 0.0]) + (CFE * BBL)) * ASZ;
            let CQR = Lanes([CQQ[0], 0.0, CQQ[1], CQQ[2], CQQ[3]]) + (BSI * BBO);
            let BBQ = AG * HL;
            let CQS = BOW * AG;
            let CQT = Lanes([CEM, 0.0, 0.0, 0.0, 0.0, 0.0]);
            let BBR = (TB - ABF) / BBQ;
            let CQU = ((CBH - CQT) - Lanes([(CQS * BBR), 0.0, 0.0, 0.0, 0.0, 0.0])) / BBQ;
            let BBS = if TB < ABF { 1.0 } else { 0.0 };
            let BCB;
            let BTG;
            if BBS != 0.0 {
                let BBT = BBR.exp();
                let BBU = C + BBT;
                let BBV = BBU.ln();
                let BBW = TB - (BBQ * BBV);
                let CQW = CBH - (Lanes([(CQS * BBV), 0.0, 0.0, 0.0, 0.0, 0.0]) + (((CQU * BBT) * (BOB / BBU)) * BBQ));
                BCB = BBW;
                BTG = CQW;
            } else {
                let BBX = (-BBR).exp();
                let BBY = C + BBX;
                let BBZ = BBY.ln();
                let BCA = ABF - (BBQ * BBZ);
                let CQV = CQT - (Lanes([(CQS * BBZ), 0.0, 0.0, 0.0, 0.0, 0.0]) + ((((CQU * BUG) * BBX) * (BOB / BBY)) * BBQ));
                BCB = BCA;
                BTG = CQV;
            }
            let BCC = BCB / HL;
            let BCD = C - BCC;
            let BCE = C - (BCD.powf(ABX));
            let BCF = TB - BCB;
            let BCG = (ABY * BCE) + (ABC * BCF);
            let CQX = BZO * HR;
            let BCH = (ABB * BCG) + (HR * TB);
            let BCI = C - BBH;
            let BCJ = ((HQ * BCH) * BCI) * J;
            let CQY = ((Lanes([(BWJ * BCH), 0.0, 0.0, 0.0, 0.0, 0.0]) + (((Lanes([(CEK * BCG), 0.0, 0.0, 0.0, 0.0, 0.0]) + (((Lanes([(CES * BCE), 0.0, 0.0, 0.0, 0.0, 0.0]) + ((((((BTG - Lanes([(BOW * BCC), 0.0, 0.0, 0.0, 0.0, 0.0])) / HL) * BUG) * (ABX * (BCD.powf(CET)))) * BUG) * ABY)) + (Lanes([(CEL * BCF), 0.0, 0.0, 0.0, 0.0, 0.0]) + ((CBH - BTG) * ABC))) * ABB)) + (Lanes([(BWK * TB), 0.0, 0.0, 0.0, 0.0, 0.0]) + Lanes([0.0, CQX[0], CQX[1], CQX[2], CQX[3], CQX[4]]))) * HQ)) * BCI) * J;
            let CQZ = Lanes([0.0, 0.0, CEM, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
            let BCK = (TD - ABF) / BBQ;
            let CRA = ((CBD - CQZ) - Lanes([0.0, 0.0, (CQS * BCK), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])) / BBQ;
            let BCL = if TD < ABF { 1.0 } else { 0.0 };
            let BCU;
            let BTH;
            if BCL != 0.0 {
                let BCM = BCK.exp();
                let BCN = C + BCM;
                let BCO = BCN.ln();
                let BCP = TD - (BBQ * BCO);
                let CRC = CBD - (Lanes([0.0, 0.0, (CQS * BCO), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]) + (((CRA * BCM) * (BOB / BCN)) * BBQ));
                BCU = BCP;
                BTH = CRC;
            } else {
                let BCQ = (-BCK).exp();
                let BCR = C + BCQ;
                let BCS = BCR.ln();
                let BCT = ABF - (BBQ * BCS);
                let CRB = CQZ - (Lanes([0.0, 0.0, (CQS * BCS), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]) + ((((CRA * BUG) * BCQ) * (BOB / BCR)) * BBQ));
                BCU = BCT;
                BTH = CRB;
            }
            let BCV = BCU / HL;
            let BCW = C - BCV;
            let BCX = C - (BCW.powf(ABX));
            let BCY = TD - BCU;
            let BCZ = (ABY * BCX) + (ABC * BCY);
            let CRD = BZT * HR;
            let BDA = (ABB * BCZ) + (HR * TD);
            let BDB = ((HQ * BDA) * BCI) * I;
            let CRE = ((Lanes([0.0, 0.0, (BWJ * BDA), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]) + (((Lanes([0.0, 0.0, (CEK * BCZ), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]) + (((Lanes([0.0, 0.0, (CES * BCX), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]) + ((((((BTH - Lanes([0.0, 0.0, (BOW * BCV), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])) / HL) * BUG) * (ABX * (BCW.powf(CET)))) * BUG) * ABY)) + (Lanes([0.0, 0.0, (CEL * BCY), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]) + ((CBD - BTH) * ABC))) * ABB)) + (Lanes([0.0, 0.0, (BWK * TD), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]) + Lanes([CRD[0], CRD[1], 0.0, CRD[2], CRD[3], CRD[4], CRD[5], CRD[6], CRD[7]]))) * HQ)) * BCI) * I;
            let BDC = AG * HF;
            let CRF = BOZ * AG;
            let BDD = C - (X.powf((-1e0f64 / HH)));
            let BDE = HF * BDD;
            let CRG = Lanes([BZE[0], 0.0, BZE[1]]);
            let CRH = Lanes([0.0, (BOZ * BDD), 0.0]);
            let BDF = (SQ - BDE) / BDC;
            let CRI = ((CRG - CRH) - Lanes([0.0, (CRF * BDF), 0.0])) / BDC;
            let BDG = if SQ < BDE { 1.0 } else { 0.0 };
            let BDR;
            let BTI;
            if BDG != 0.0 {
                let BDH = BDF.exp();
                let BDI = C + BDH;
                let BDJ = BDI.ln();
                let BDK = SQ - (BDC * BDJ);
                let CRK = CRG - (Lanes([0.0, (CRF * BDJ), 0.0]) + (((CRI * BDH) * (BOB / BDI)) * BDC));
                BDR = BDK;
                BTI = CRK;
            } else {
                let BDL = (-BDF).exp();
                let BDM = C + BDL;
                let BDN = BDM.ln();
                let BDO = BDE - (BDC * BDN);
                let CRJ = CRH - (Lanes([0.0, (CRF * BDN), 0.0]) + ((((CRI * BUG) * BDL) * (BOB / BDM)) * BDC));
                BDR = BDO;
                BTI = CRJ;
            }
            let BDP = C - HH;
            let BDQ = HF / BDP;
            let BDS = BDR / HF;
            let BDT = C - BDS;
            let BDU = C - (BDT.powf(BDP));
            let BDV = (BDQ * BDU) + (X * (SQ - BDR));
            let BDW = HI * BDV;
            let CRL = Lanes([0.0, (BWG * BDV), 0.0]) + (((Lanes([0.0, ((BOZ / BDP) * BDU), 0.0]) + ((((((BTI - Lanes([0.0, (BOZ * BDS), 0.0])) / HF) * BUG) * (BDP * (BDT.powf((BDP - BOB))))) * BUG) * BDQ)) + ((CRG - BTI) * X)) * HI);
            let BDX = QT * KZ;
            let BDY = KV / KZ;
            let BEA = C / BDZ;
            let BEB = BDY.powf(BEA);
            let BEC = BDX * BEB;
            let CRM = (((BYO * KZ) + (BXJ * QT)) * BEB) + ((((BXI - (BXJ * BDY)) / KZ) * (BEA * (BDY.powf((BEA - BOB))))) * BDX);
            let BED = BDZ * BQ;
            let CRN = BUK * BDZ;
            let BEE = SM / BED;
            let CRO = (CDX - Lanes([(CRN * BEE), 0.0, 0.0])) / BED;
            let BEF = if BEE < TH { 1.0 } else { 0.0 };
            let BEJ;
            let BTJ;
            if BEF != 0.0 {
                let BEG = BEE.exp();
                let CRQ = CRO * BEG;
                BEJ = BEG;
                BTJ = CRQ;
            } else {
                let BEH = TH.exp();
                let BEI = BEH * (C + (BEE - TH));
                let CRP = CRO * BEH;
                BEJ = BEI;
                BTJ = CRP;
            }
            let BEK = BEC * BEJ;
            let CRR = Lanes([(CRM * BEJ), 0.0, 0.0]) + (BTJ * BEC);
            let BEL = KL * RB;
            let BEM = (BEL * BQ) / IV;
            let CRS = ((((BYQ * KL) * BQ) + (BUK * BEL)) - (BWT * BEM)) / IV;
            let BEN = KE * BEM;
            let BEO = BEN * AUU;
            let BEQ = (BEP + YG) + X;
            let BER = BEO * BEQ;
            let CRT = ((Lanes([((CRS * KE) * AUU), 0.0, 0.0, 0.0]) + (BQI * BEN)) * BEQ) + ((BQK + Lanes([BQD[0], BQD[1], BQD[2], 0.0])) * BEO);
            let BES = if parameters[79] == A { 1.0 } else { 0.0 };
            let BFK;
            let BTK;
            if BES != 0.0 {
                let BET = RF * KE;
                let BEU = (BBK * ANE) + (BEM * ANH);
                let BEV = (BET * BEU) / RD;
                let CRY = ((Lanes([((BYS * KE) * BEU), 0.0, 0.0, 0.0, 0.0, 0.0]) + (((Lanes([(CQM * ANE), 0.0, 0.0, 0.0, 0.0, 0.0]) + (CJJ * BBK)) + (Lanes([(CRS * ANH), 0.0, 0.0, 0.0, 0.0, 0.0]) + (CJK * BEM))) * BET)) - Lanes([(BYR * BEV), 0.0, 0.0, 0.0, 0.0, 0.0])) / RD;
                BFK = BEV;
                BTK = CRY;
            } else {
                let BEY = (TB - BEW) / BEX;
                let BEZ = BEY * BR;
                let CRU = (((CBH - Lanes([BOV, 0.0, 0.0, 0.0, 0.0, 0.0])) / BEX) * BR) + Lanes([(BUL * BEY), 0.0, 0.0, 0.0, 0.0, 0.0]);
                let BFA = if BEZ < TH { 1.0 } else { 0.0 };
                let BFF;
                let BTL;
                if BFA != 0.0 {
                    let BFB = BEZ.exp();
                    let CRW = CRU * BFB;
                    BFF = BFB;
                    BTL = CRW;
                } else {
                    let BFC = TH.exp();
                    let BFD = BFC * (C + (BEZ - TH));
                    let CRV = CRU * BFC;
                    BFF = BFD;
                    BTL = CRV;
                }
                let BFE = ANI * RJ;
                let BFG = (C + (KL * BFF)).sqrt();
                let BFH = C + BFG;
                let BFI = (BFE * AMY) / BFH;
                let CRX = ((Lanes([(((CJL * RJ) + (BYT * ANI)) * AMY), 0.0, 0.0, 0.0, 0.0, 0.0]) + (BPT * BFE)) - (((BTL * KL) * (BOB / (BXF * BFG))) * BFI)) / BFH;
                BFK = BFI;
                BTK = CRX;
            }
            let BFJ = if (if (if APG == C { 1.0 } else { 0.0 }) != 0.0 || (if APG == CV { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && APH != 0.0 { 1.0 } else { 0.0 };
            let BJZ;
            let BKD;
            let BTM;
            let BTN;
            if BFJ != 0.0 {
                let BFL = BFK * J;
                let CRZ = BTK * J;
                let BGN;
                let BTO;
                if BES != 0.0 {
                    let BFM = ACI * APO;
                    let CSE = Lanes([0.0, 0.0, (CEX * APO), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]) + (BPV * ACI);
                    let BFN = (C + BFM).sqrt();
                    let BFO = C + BFN;
                    let BFP = (BFM - ACI) / BFO;
                    let BFR = KL * BFQ;
                    let CSF = BPZ * KL;
                    let BFS = (C + BFR).sqrt();
                    let BFT = C + BFS;
                    let BFU = BFR / BFT;
                    let BFV = KE * I;
                    let BFW = BFV * RF;
                    let BFX = (BBK * BFP) + (BEM * BFU);
                    let BFY = (BFW * BFX) / RD;
                    let CSG = ((Lanes([0.0, 0.0, ((BYS * BFV) * BFX), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]) + (((Lanes([0.0, 0.0, (CQM * BFP), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]) + ((((CSE - Lanes([0.0, 0.0, CEX, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])) - ((CSE * (BOB / (BXF * BFN))) * BFP)) / BFO) * BBK)) + (Lanes([0.0, 0.0, (CRS * BFU), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]) + (((CSF - ((CSF * (BOB / (BXF * BFS))) * BFU)) / BFT) * BEM))) * BFW)) - Lanes([0.0, 0.0, (BYR * BFY), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])) / RD;
                    BGN = BFY;
                    BTO = CSG;
                } else {
                    let BFZ = TD - BEW;
                    let BGA = BFZ * BR;
                    let CSA = ((CBD - Lanes([0.0, 0.0, BOV, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])) * BR) + Lanes([0.0, 0.0, (BUL * BFZ), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
                    let BGB = if BGA < TH { 1.0 } else { 0.0 };
                    let BGI;
                    let BTP;
                    if BGB != 0.0 {
                        let BGC = BGA.exp();
                        let CSC = CSA * BGC;
                        BGI = BGC;
                        BTP = CSC;
                    } else {
                        let BGD = TH.exp();
                        let BGE = BGD * (C + (BGA - TH));
                        let CSB = CSA * BGD;
                        BGI = BGE;
                        BTP = CSB;
                    }
                    let BGF = X * I;
                    let BGG = BGF * NG;
                    let BGH = BGG * RJ;
                    let BGJ = (C + (KL * BGI)).sqrt();
                    let BGK = C + BGJ;
                    let BGL = (BGH * APO) / BGK;
                    let CSD = ((Lanes([0.0, 0.0, ((((BXV * BGF) * RJ) + (BYT * BGG)) * APO), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]) + (BPV * BGH)) - (((BTP * KL) * (BOB / (BXF * BGJ))) * BGL)) / BGK;
                    BGN = BGL;
                    BTO = CSD;
                }
                let BGO = BGM * BGN;
                let CSH = BTO * BGM;
                let CSI = (BSB * BGN) + Lanes([CSH[0], CSH[1], 0.0, CSH[2], CSH[3], CSH[4], CSH[5], CSH[6], CSH[7], CSH[8]]);
                BJZ = BGO;
                BKD = BFL;
                BTM = CSI;
                BTN = CRZ;
            } else {
                BJZ = A;
                BKD = BFK;
                BTM = CKC;
                BTN = BTK;
            }
            let BGP = if parameters[6] == C { 1.0 } else { 0.0 };
            let BJC;
            let BJD;
            let BJI;
            let BJN;
            let BTQ;
            let BTR;
            let BTS;
            let BTT;
            if BGP != 0.0 {
                let BGQ = -Y;
                let CSK = CEC * (BGQ * (AAQ.powf((BGQ - BOB))));
                let BGR = (AAQ.powf(BGQ)) - CV;
                let BGS = if AAF < A { 1.0 } else { 0.0 };
                let BGZ;
                let BTU;
                if BGS != 0.0 {
                    let BGT = AAF.exp();
                    let BGU = C + BGT;
                    let BGV = C / BGU;
                    let CSN = (((CDZ * BGT) * BGV) * BUG) / BGU;
                    BGZ = BGV;
                    BTU = CSN;
                } else {
                    let BGW = (-AAF).exp();
                    let CSL = (CDZ * BUG) * BGW;
                    let BGX = C + BGW;
                    let BGY = BGW / BGX;
                    let CSM = (CSL - (CSL * BGY)) / BGX;
                    BGZ = BGY;
                    BTU = CSM;
                }
                let BHA = (BGR * BGZ) + CV;
                let CSO = Lanes([(CQD * BHA), 0.0, 0.0]) + (((CSK * BGZ) + (BTU * BGR)) * BAP);
                let BHB = (ACK * BR) / KO;
                let BHC = KE / ACL;
                let BHD = BHB * BHC;
                let BHE = BBL * ASZ;
                let CSP = ((((((CEY * BR) + Lanes([(BUL * ACK), 0.0, 0.0])) - Lanes([(BPC * BHB), 0.0, 0.0])) / KO) * BHC) + ((((CEZ * BHC) * BUG) / ACL) * BHB)) * BHE;
                let BHF = BEK / BED;
                let CSQ = (CRR - Lanes([(CRN * BHF), 0.0, 0.0])) / BED;
                let BHG = WU * SP;
                let BHH = ((BAP * BHA) + (BHE * BHD)) + BHF;
                let BHI = BHG * BHH;
                let CSR = (BZD * WU) * BHH;
                let CSS = ((Lanes([CSO[0], CSO[1], CSO[2], 0.0, 0.0]) + (((Lanes([(CQN * ASZ), 0.0, 0.0, 0.0, 0.0]) + (BSI * BBL)) * BHD) + Lanes([CSP[0], CSP[1], CSP[2], 0.0, 0.0]))) + Lanes([CSQ[0], CSQ[1], CSQ[2], 0.0, 0.0])) * BHG;
                let CST = Lanes([0.0, 0.0, CSR[0], CSR[1], 0.0, 0.0]) + Lanes([CSS[0], CSS[1], 0.0, CSS[2], CSS[3], CSS[4]]);
                let BHK = C - BHJ;
                let BHL = BHK * BEK;
                let CSU = CRR * BHK;
                let CSV = CRR * BHJ;
                let BHM = BBN + (BHJ * BEK);
                let CSW = CQP + Lanes([CSV[0], CSV[1], CSV[2], 0.0, 0.0]);
                let BHO = (BHN * BHM) + BBP;
                let CSX = (CSW * BHN) + CQR;
                let BHP = C - BHN;
                let BHQ = BHP * BHM;
                let CSY = CSW * BHP;
                BJC = BHQ;
                BJD = BHL;
                BJI = BHO;
                BJN = BHI;
                BTQ = CSY;
                BTR = CSU;
                BTS = CSX;
                BTT = CST;
            } else {
                BJC = BBN;
                BJD = BEK;
                BJI = BBP;
                BJN = A;
                BTQ = CQP;
                BTR = CRR;
                BTS = CQR;
                BTT = CSJ;
            }
            let BHR = (B * WH) * R;
            let CSZ = (CCB * B) * R;
            let BHS = (B * AEA) * R;
            let CTA = (CFV * B) * R;
            let BHT = (B * BAD) * R;
            let CTB = (CPK * B) * R;
            let BHU = (B * BAA) * R;
            let CTC = (CPE * B) * R;
            let BNA;
            let BNB;
            let BTV;
            let BTW;
            if MN != 0.0 {
                let BHV = (B * (-ASI)) * R;
                let CTF = ((CLF * BUG) * B) * R;
                BNA = BHV;
                BNB = A;
                BTV = CTF;
                BTW = CTE;
            } else {
                let BHW = (B * (-ASI)) * R;
                let CTD = ((CLF * BUG) * B) * R;
                BNA = A;
                BNB = BHW;
                BTV = CTE;
                BTW = CTD;
            }
            let BHX = (B * BAG) * R;
            let CTG = (BRZ * B) * R;
            let BHY = (B * BAI) * R;
            let CTH = (BRW * B) * R;
            let BHZ = (B * BAK) * R;
            let CTI = (BSA * B) * R;
            let BIA = (B * APF) * R;
            let CTJ = (CKB * B) * R;
            let BIB = (B * ATI) * R;
            let CTK = (CLY * B) * R;
            let BID = (B * (BIC * AZI)) * R;
            let CTL = ((BSK * BIC) * B) * R;
            let CTM = BZG * B;
            let BIE = (B * ST) / AYL;
            let BIF = BIE * R;
            let CTN = ((Lanes([CTM[0], 0.0, CTM[1]]) - Lanes([0.0, (BPA * BIE), 0.0])) / AYL) * R;
            let CTO = BZH * B;
            let BIG = (B * SV) / AYG;
            let BIH = BIG * R;
            let CTP = ((Lanes([CTO[0], 0.0, CTO[1]]) - Lanes([0.0, (BPB * BIG), 0.0])) / AYG) * R;
            let BIJ = BII * BD;
            let CTQ = BOC * BII;
            let BIK = ddt(13541, BIJ) * R;
            let CTS = (CTQ * CTR) * R;
            let BNQ = BIJ * R;
            let CTT = CTQ * R;
            let BIL = C - RV;
            let BIM = if RU > S { 1.0 } else { 0.0 };
            let BIX;
            let BTX;
            if BIM != 0.0 {
                let BIN = if parameters[145] == A { 1.0 } else { 0.0 };
                let BIY;
                let BTY;
                if BIN != 0.0 {
                    let BIO = (BD / RW) * R;
                    let CTX = (BOC / RW) * R;
                    BIY = BIO;
                    BTY = CTX;
                } else {
                    let BIP = if (BIL.abs()) < ARG { 1.0 } else { 0.0 };
                    let BIZ;
                    let BTZ;
                    if BIP != 0.0 {
                        let BIQ = (L / RW) * R;
                        let BIR = C + (BD / L);
                        let BIS = BIQ * (BIR.ln());
                        let CTW = ((BOC / L) * (BOB / BIR)) * BIQ;
                        BIZ = BIS;
                        BTZ = CTW;
                    } else {
                        let BIT = (L / (BIL * RW)) * R;
                        let BIU = C + (BD / L);
                        let BIV = BIT * ((BIU.powf(BIL)) - C);
                        let CTV = ((BOC / L) * (BIL * (BIU.powf((BIL - BOB))))) * BIT;
                        BIZ = BIV;
                        BTZ = CTV;
                    }
                    BIY = BIZ;
                    BTY = BTZ;
                }
                BIX = BIY;
                BTX = BTY;
            } else {
                let BIW = BD / Q;
                let CTU = BOC / Q;
                BIX = BIW;
                BTX = CTU;
            }
            let BJB = (BJA * BAM) * R;
            let CTY = (CQC * BJA) * R;
            let BJE = B * ((BAQ + BJC) + BJD);
            let CTZ = ((Lanes([CQE[0], CQE[1], CQE[2], 0.0, 0.0]) + BTQ) + Lanes([BTR[0], BTR[1], BTR[2], 0.0, 0.0])) * B;
            let BJF = ddt(13609, BJE) * R;
            let CUA = (CTZ * CTR) * R;
            let BNR = BJE * R;
            let CUB = CTZ * R;
            let BJG = B * BBG;
            let CUC = CQK * B;
            let BJH = ddt(13615, BJG) * R;
            let CUD = (CUC * CTR) * R;
            let BNS = BJG * R;
            let CUE = CUC * R;
            let BJJ = B * ((BBJ + BJI) + BER);
            let CUF = ((Lanes([CQL[0], 0.0, CQL[1], CQL[2], CQL[3]]) + BTS) + Lanes([CRT[0], 0.0, CRT[1], CRT[2], CRT[3]])) * B;
            let BJK = ddt(13625, BJJ) * R;
            let CUG = (CUF * CTR) * R;
            let BNT = BJJ * R;
            let CUH = CUF * R;
            let BJL = B * BDW;
            let CUI = CRL * B;
            let BJM = ddt(13631, BJL) * R;
            let CUJ = (CUI * CTR) * R;
            let BNU = BJL * R;
            let CUK = CUI * R;
            let BJO = B * BJN;
            let CUL = BTT * B;
            let BJP = ddt(13637, BJO) * R;
            let CUM = (CUL * CTR) * R;
            let BNV = BJO * R;
            let CUN = CUL * R;
            let BJQ = B * parameters[69];
            let BJR = BJQ * SW;
            let CUO = BZI * BJQ;
            let BJS = ddt(13645, BJR) * R;
            let CUP = (CUO * CTR) * R;
            let BNW = BJR * R;
            let CUQ = CUO * R;
            let BJT = B * parameters[78];
            let BJU = BJT * SX;
            let CUR = BZJ * BJT;
            let BJV = ddt(13653, BJU) * R;
            let CUS = (CUR * CTR) * R;
            let BNX = BJU * R;
            let CUT = CUR * R;
            let BJW = (B * ASN) * R;
            let CUU = (CLL * B) * R;
            let BJX = B * TC;
            let CUV = (BZS * B) * AZO;
            let BJY = (BJX * AZO) * R;
            let CUW = (Lanes([CUV[0], CUV[1], 0.0, CUV[2], CUV[3], CUV[4], CUV[5], CUV[6], CUV[7]]) + Lanes([0.0, 0.0, (BPL * BJX), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])) * R;
            let BKA = B * (BDB + BJZ);
            let CUX = (Lanes([CRE[0], CRE[1], 0.0, CRE[2], CRE[3], CRE[4], CRE[5], CRE[6], CRE[7], CRE[8]]) + BTM) * B;
            let BKB = ddt(13673, BKA) * R;
            let CUY = (CUX * CTR) * R;
            let BNY = BKA * R;
            let CUZ = CUX * R;
            let BKC = (B * ((ASL + BAE) + ASK)) * R;
            let CVA = (((CLJ + CPO) + CLH) * B) * R;
            let BKE = B * (BCJ + BKD);
            let CVB = (CQY + BTN) * B;
            let BKF = ddt(13692, BKE) * R;
            let CVC = (CVB * CTR) * R;
            let BNZ = BKE * R;
            let CVD = CVB * R;
            let BNC;
            let BND;
            let BUA;
            if SA != 0.0 {
                let BKG = B * TA;
                let CVF = (BZL * B) * AZR;
                let BKH = (BKG * AZR) * R;
                let CVG = (Lanes([0.0, CVF[0], CVF[1]]) + Lanes([(BPN * BKG), 0.0, 0.0])) * R;
                BNC = BKH;
                BND = A;
                BUA = CVG;
            } else {
                BNC = A;
                BND = BKI;
                BUA = CVE;
            }
            let BNE;
            let BNF;
            let BUB;
            if SD != 0.0 {
                let BKJ = B * SZ;
                let CVI = (BZK * B) * AZU;
                let BKK = (BKJ * AZU) * R;
                let CVJ = (Lanes([0.0, CVI[0], CVI[1]]) + Lanes([(BPP * BKJ), 0.0, 0.0])) * R;
                BNE = BKK;
                BNF = A;
                BUB = CVJ;
            } else {
                BNE = A;
                BNF = BKL;
                BUB = CVH;
            }
            let BKM = (ADZ + ADY) / ADV;
            let CVK = ((CFT + CFU) - (CFQ * BKM)) / ADV;
            let BKN = if parameters[130] > A { 1.0 } else { 0.0 };
            let BKQ;
            let BUC;
            if BKN != 0.0 {
                let BKO = AZI / BKM;
                let BKP = BKO.abs();
                let CVL = ((BSK - (CVK * BKO)) / BKM) * ((BXF * (if BKO >= 0e0f64 { 1.0 } else { 0.0 })) - BOB);
                BKQ = BKP;
                BUC = CVL;
            } else {
                BKQ = A;
                BUC = CLU;
            }
            let BKR = if BKM > A { 1.0 } else { 0.0 };
            let BKX;
            let BUD;
            if BKR != 0.0 {
                let BKS = (BJC + BJI) / BKM;
                let CVN = ((BTQ + BTS) - (CVK * BKS)) / BKM;
                BKX = BKS;
                BUD = CVN;
            } else {
                let BKT = QX * ASZ;
                let BKU = BKT * ADV;
                let CVM = ((Lanes([(BYP * ASZ), 0.0, 0.0, 0.0, 0.0]) + (BSI * QX)) * ADV) + (CFQ * BKT);
                BKX = BKU;
                BUD = CVM;
            }
            let BKW = if BKV == C { 1.0 } else { 0.0 };
            let BLK;
            let BUE;
            if BKW != 0.0 {
                let BKY = BHN * BKX;
                let CVP = BUD * BHN;
                BLK = BKY;
                BUE = CVP;
            } else {
                let BKZ = if BKV == X { 1.0 } else { 0.0 };
                let BLL;
                let BUF;
                if BKZ != 0.0 {
                    let BLB = BLA * BKX;
                    let CVO = BUD * BLA;
                    BLL = BLB;
                    BUF = CVO;
                } else {
                    BLL = A;
                    BUF = CLU;
                }
                BLK = BLL;
                BUE = BUF;
            }
            let BLC = if (AZX + BAC) < A { 1.0 } else { 0.0 };
            if BLC != 0.0 {
            } else {
            }
            let BLD = if ((AIH + AIP) + AJF) < A { 1.0 } else { 0.0 };
            if BLD != 0.0 {
            } else {
            }
            let BLE = if ASL < A { 1.0 } else { 0.0 };
            if BLE != 0.0 {
            } else {
            }
            let BLF = if ASK < A { 1.0 } else { 0.0 };
            if BLF != 0.0 {
            } else {
            }
            let BLG = if I == A { 1.0 } else { 0.0 };
            if BLG != 0.0 {
            } else {
            }
            let BLH = if ASN < A { 1.0 } else { 0.0 };
            if BLH != 0.0 {
            } else {
            }
            let BLM = ddt(13963, BLJ);
            let BLN = BLK * BLM;
            let CVQ = BUE * BLM;
            let CVR = Lanes([CVQ[0], CVQ[1], CVQ[2], CVQ[3], CVQ[4], 0.0]) + Lanes([0.0, 0.0, 0.0, 0.0, 0.0, ((BOO * CTR) * BLK)]);
            let BOA = BLK * BLJ;
            let CVS = BUE * BLJ;
            let CVT = Lanes([CVS[0], CVS[1], CVS[2], CVS[3], CVS[4], 0.0]) + Lanes([0.0, 0.0, 0.0, 0.0, 0.0, (BOO * BLK)]);
            let BLO = BKQ * BLJ;
            let CVU = BUC * BLJ;
            let CVV = Lanes([CVU[0], CVU[1], CVU[2], CVU[3], CVU[4], 0.0]) + Lanes([0.0, 0.0, 0.0, 0.0, 0.0, (BOO * BKQ)]);
            let BNG;
            let BNH;
            if MN != 0.0 {
                BNG = BMD;
                BNH = A;
            } else {
                BNG = A;
                BNH = BME;
            }
            let BMQ;
            let BMS;
            let BMU;
            let BMW;
            let BNI;
            let BNK;
            let BNM;
            let BNO;
            if SA != 0.0 {
                let BMR;
                let BMT;
                let BNJ;
                let BNL;
                let BNN;
                if SD != 0.0 {
                    BMR = BMI;
                    BMT = A;
                    BNJ = BMJ;
                    BNL = BMK;
                    BNN = A;
                } else {
                    BMR = A;
                    BMT = BML;
                    BNJ = A;
                    BNL = A;
                    BNN = BMM;
                }
                BMQ = BMR;
                BMS = BMT;
                BMU = A;
                BMW = A;
                BNI = BNJ;
                BNK = BNL;
                BNM = BNN;
                BNO = A;
            } else {
                let BMV;
                let BMX;
                let BNP;
                if SD != 0.0 {
                    BMV = BMN;
                    BMX = A;
                    BNP = BMO;
                } else {
                    BMV = A;
                    BMX = BMP;
                    BNP = A;
                }
                BMQ = A;
                BMS = A;
                BMU = BMV;
                BMW = BMX;
                BNI = A;
                BNK = A;
                BNM = A;
                BNO = BNP;
            }
            let BMY = if (((((BHZ + BIH) + BJS) + BJV) + BJW) + BKB) == A { 1.0 } else { 0.0 };
            if BMY != 0.0 {
            } else {
            }
            let BMZ = if R != C { 1.0 } else { 0.0 };
            if BMZ != 0.0 {
            } else {
            }
            let CVW = CSZ[0];
            let CVX = CSZ[1];
            let CVY = CSZ[2];
            let CVZ = CSZ[3];
            let CWA = CTA[0];
            let CWB = CTA[1];
            let CWC = CTA[2];
            let CWD = CTA[3];
            let CWE = CTA[4];
            let CWF = CTB[0];
            let CWG = CTB[1];
            let CWH = CTB[2];
            let CWI = CTC[0];
            let CWJ = CTC[1];
            let CWK = CTC[2];
            let CWL = CTC[3];
            let CWM = CTC[4];
            let CWN = BTV[0];
            let CWO = BTV[1];
            let CWP = BTV[2];
            let CWQ = BTV[3];
            let CWR = BTW[0];
            let CWS = BTW[1];
            let CWT = BTW[2];
            let CWU = BTW[3];
            let CWV = CTG[0];
            let CWW = CTG[1];
            let CWX = CTG[2];
            let CWY = CTG[3];
            let CWZ = CTG[4];
            let CXA = CTG[5];
            let CXB = CTG[6];
            let CXC = CTH[0];
            let CXD = CTH[1];
            let CXE = CTH[2];
            let CXF = CTH[3];
            let CXG = CTH[4];
            let CXH = CTI[0];
            let CXI = CTI[1];
            let CXJ = CTI[2];
            let CXK = CTI[3];
            let CXL = CTI[4];
            let CXM = CTI[5];
            let CXN = CTI[6];
            let CXO = CTI[7];
            let CXP = CTI[8];
            let CXQ = CTI[9];
            let CXR = CTJ[0];
            let CXS = CTJ[1];
            let CXT = CTJ[2];
            let CXU = CTK[0];
            let CXV = CTK[1];
            let CXW = CTK[2];
            let CXX = CTK[3];
            let CXY = CTK[4];
            let CXZ = CTK[5];
            let CYA = CTL[0];
            let CYB = CTL[1];
            let CYC = CTL[2];
            let CYD = CTL[3];
            let CYE = CTL[4];
            let CYF = CTN[0];
            let CYG = CTN[1];
            let CYH = CTN[2];
            let CYI = CTP[0];
            let CYJ = CTP[1];
            let CYK = CTP[2];
            let CYL = BTX;
            let CYM = CTS;
            let CYN = CTY[0];
            let CYO = CTY[1];
            let CYP = CTY[2];
            let CYQ = CTY[3];
            let CYR = CTY[4];
            let CYS = CTY[5];
            let CYT = CTY[6];
            let CYU = CTY[7];
            let CYV = CTY[8];
            let CYW = CTY[9];
            let CYX = CTY[10];
            let CYY = CTY[11];
            let CYZ = CUA[0];
            let CZA = CUA[1];
            let CZB = CUA[2];
            let CZC = CUA[3];
            let CZD = CUA[4];
            let CZE = CUD[0];
            let CZF = CUD[1];
            let CZG = CUD[2];
            let CZH = CUG[0];
            let CZI = CUG[1];
            let CZJ = CUG[2];
            let CZK = CUG[3];
            let CZL = CUG[4];
            let CZM = CUJ[0];
            let CZN = CUJ[1];
            let CZO = CUJ[2];
            let CZP = CUM[0];
            let CZQ = CUM[1];
            let CZR = CUM[2];
            let CZS = CUM[3];
            let CZT = CUM[4];
            let CZU = CUM[5];
            let CZV = CUP[0];
            let CZW = CUP[1];
            let CZX = CUS[0];
            let CZY = CUS[1];
            let CZZ = CUU[0];
            let DAA = CUU[1];
            let DAB = CUU[2];
            let DAC = CUU[3];
            let DAD = CUU[4];
            let DAE = CUU[5];
            let DAF = CUU[6];
            let DAG = CUU[7];
            let DAH = CUU[8];
            let DAI = CUU[9];
            let DAJ = CUW[0];
            let DAK = CUW[1];
            let DAL = CUW[2];
            let DAM = CUW[3];
            let DAN = CUW[4];
            let DAO = CUW[5];
            let DAP = CUW[6];
            let DAQ = CUW[7];
            let DAR = CUW[8];
            let DAS = CUY[0];
            let DAT = CUY[1];
            let DAU = CUY[2];
            let DAV = CUY[3];
            let DAW = CUY[4];
            let DAX = CUY[5];
            let DAY = CUY[6];
            let DAZ = CUY[7];
            let DBA = CUY[8];
            let DBB = CUY[9];
            let DBC = CVA[0];
            let DBD = CVA[1];
            let DBE = CVA[2];
            let DBF = CVA[3];
            let DBG = CVA[4];
            let DBH = CVA[5];
            let DBI = CVC[0];
            let DBJ = CVC[1];
            let DBK = CVC[2];
            let DBL = CVC[3];
            let DBM = CVC[4];
            let DBN = CVC[5];
            let DBO = BUA[0];
            let DBP = BUA[1];
            let DBQ = BUA[2];
            let DBR = BUB[0];
            let DBS = BUB[1];
            let DBT = BUB[2];
            let DBU = BOO;
            let DBV = CVR[0];
            let DBW = CVR[1];
            let DBX = CVR[2];
            let DBY = CVR[3];
            let DBZ = CVR[4];
            let DCA = CVR[5];
            let DCB = CVV[0];
            let DCC = CVV[1];
            let DCD = CVV[2];
            let DCE = CVV[3];
            let DCF = CVV[4];
            let DCG = CVV[5];
            let DCH = CTT;
            let DCI = CUB[0];
            let DCJ = CUB[1];
            let DCK = CUB[2];
            let DCL = CUB[3];
            let DCM = CUB[4];
            let DCN = CUE[0];
            let DCO = CUE[1];
            let DCP = CUE[2];
            let DCQ = CUH[0];
            let DCR = CUH[1];
            let DCS = CUH[2];
            let DCT = CUH[3];
            let DCU = CUH[4];
            let DCV = CUK[0];
            let DCW = CUK[1];
            let DCX = CUK[2];
            let DCY = CUN[0];
            let DCZ = CUN[1];
            let DDA = CUN[2];
            let DDB = CUN[3];
            let DDC = CUN[4];
            let DDD = CUN[5];
            let DDE = CUQ[0];
            let DDF = CUQ[1];
            let DDG = CUT[0];
            let DDH = CUT[1];
            let DDI = CUZ[0];
            let DDJ = CUZ[1];
            let DDK = CUZ[2];
            let DDL = CUZ[3];
            let DDM = CUZ[4];
            let DDN = CUZ[5];
            let DDO = CUZ[6];
            let DDP = CUZ[7];
            let DDQ = CUZ[8];
            let DDR = CUZ[9];
            let DDS = CVD[0];
            let DDT = CVD[1];
            let DDU = CVD[2];
            let DDV = CVD[3];
            let DDW = CVD[4];
            let DDX = CVD[5];
            let DDY = CVT[0];
            let DDZ = CVT[1];
            let DEA = CVT[2];
            let DEB = CVT[3];
            let DEC = CVT[4];
            let DED = CVT[5];
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(8),
            Some(9),
            multiplicity * (BHR),
            [4, 7, 8, 9],
            [CVW, CVX, CVY, CVZ],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(9),
            Some(5),
            multiplicity * (BHS),
            [4, 5, 7, 8, 9],
            [CWA, CWB, CWC, CWD, CWE],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(6),
            Some(5),
            multiplicity * (BHT),
            [4, 5, 6],
            [CWF, CWG, CWH],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(7),
            Some(5),
            multiplicity * (BHU),
            [4, 5, 7, 8, 9],
            [CWI, CWJ, CWK, CWL, CWM],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(7),
            Some(8),
            multiplicity * (BNA),
            [4, 6, 7, 8],
            [CWN, CWO, CWP, CWQ],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(7),
            Some(9),
            multiplicity * (BNB),
            [4, 6, 7, 8],
            [CWR, CWS, CWT, CWU],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(6),
            Some(3),
            multiplicity * (BHX),
            [3, 4, 6, 7, 8, 9, 11],
            [CWV, CWW, CWX, CWY, CWZ, CXA, CXB],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(7),
            Some(3),
            multiplicity * (BHY),
            [3, 4, 7, 8, 9],
            [CXC, CXD, CXE, CXF, CXG],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(1),
            Some(3),
            multiplicity * (BHZ),
            [0, 1, 3, 4, 6, 7, 8, 9, 10, 11],
            [CXH, CXI, CXJ, CXK, CXL, CXM, CXN, CXO, CXP, CXQ],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(3),
            Some(8),
            multiplicity * (BIA),
            [3, 4, 8],
            [CXR, CXS, CXT],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(6),
            Some(7),
            multiplicity * (BIB),
            [4, 5, 6, 7, 8, 9],
            [CXU, CXV, CXW, CXX, CXY, CXZ],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(7),
            Some(9),
            multiplicity * (BID),
            [4, 5, 7, 8, 9],
            [CYA, CYB, CYC, CYD, CYE],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(2),
            Some(5),
            multiplicity * (BIF),
            [2, 4, 5],
            [CYF, CYG, CYH],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(1),
            Some(6),
            multiplicity * (BIH),
            [1, 4, 6],
            [CYI, CYJ, CYK],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(4),
            None,
            multiplicity * (BIX),
            [4],
            [CYL],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(4),
            None,
            multiplicity * (BIK),
            [4],
            [CYM],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<12, 0>(
            Some(4),
            None,
            multiplicity * (BJB),
            [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
            [CYN, CYO, CYP, CYQ, CYR, CYS, CYT, CYU, CYV, CYW, CYX, CYY],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(7),
            Some(5),
            multiplicity * (BJF),
            [4, 5, 7, 8, 9],
            [CYZ, CZA, CZB, CZC, CZD],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(6),
            Some(5),
            multiplicity * (BJH),
            [4, 5, 6],
            [CZE, CZF, CZG],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(7),
            Some(9),
            multiplicity * (BJK),
            [4, 5, 7, 8, 9],
            [CZH, CZI, CZJ, CZK, CZL],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(3),
            Some(8),
            multiplicity * (BJM),
            [3, 4, 8],
            [CZM, CZN, CZO],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(6),
            Some(7),
            multiplicity * (BJP),
            [4, 5, 6, 7, 8, 9],
            [CZP, CZQ, CZR, CZS, CZT, CZU],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(1),
            Some(2),
            multiplicity * (BJS),
            [1, 2],
            [CZV, CZW],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(1),
            Some(0),
            multiplicity * (BJV),
            [0, 1],
            [CZX, CZY],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(1),
            Some(10),
            multiplicity * (BJW),
            [0, 1, 3, 4, 6, 7, 8, 9, 10, 11],
            [CZZ, DAA, DAB, DAC, DAD, DAE, DAF, DAG, DAH, DAI],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(0),
            Some(10),
            multiplicity * (BJY),
            [0, 1, 4, 6, 7, 8, 9, 10, 11],
            [DAJ, DAK, DAL, DAM, DAN, DAO, DAP, DAQ, DAR],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(1),
            Some(10),
            multiplicity * (BKB),
            [0, 1, 3, 4, 6, 7, 8, 9, 10, 11],
            [DAS, DAT, DAU, DAV, DAW, DAX, DAY, DAZ, DBA, DBB],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(6),
            Some(11),
            multiplicity * (BKC),
            [4, 6, 7, 8, 9, 11],
            [DBC, DBD, DBE, DBF, DBG, DBH],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(6),
            Some(11),
            multiplicity * (BKF),
            [4, 6, 7, 8, 9, 11],
            [DBI, DBJ, DBK, DBL, DBM, DBN],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(10),
            Some(11),
            multiplicity * (BNC),
            [4, 10, 11],
            [DBO, DBP, DBQ],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(10), Some(11), 0, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            0,
            BND,
            [],
            [],
            [],
            [],
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(11),
            Some(8),
            multiplicity * (BNE),
            [4, 8, 11],
            [DBR, DBS, DBT],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(11), Some(8), 1, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            1,
            BNF,
            [],
            [],
            [],
            [],
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(12),
            None,
            multiplicity * (BLI),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(12),
            None,
            multiplicity * (BLJ),
            [12],
            [DBU],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(7),
            Some(5),
            multiplicity * (BLN),
            [4, 5, 7, 8, 9, 12],
            [DBV, DBW, DBX, DBY, DBZ, DCA],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(9),
            Some(7),
            multiplicity * (BLO),
            [4, 5, 7, 8, 9, 12],
            [DCB, DCC, DCD, DCE, DCF, DCG],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(9),
            Some(5),
            multiplicity * (BLJ),
            [12],
            [DBU],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(9),
            Some(7),
            multiplicity * (BLP),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(7),
            Some(5),
            multiplicity * (BLQ),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(2),
            Some(5),
            multiplicity * (BLR),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(1),
            Some(6),
            multiplicity * (BLS),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(6),
            Some(7),
            multiplicity * (BLT),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(7),
            Some(5),
            multiplicity * (BLU),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(6),
            Some(5),
            multiplicity * (BLV),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(6),
            Some(5),
            multiplicity * (BLW),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(6),
            Some(11),
            multiplicity * (BLX),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(6),
            Some(11),
            multiplicity * (BLY),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(6),
            Some(11),
            multiplicity * (BLZ),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(6),
            Some(11),
            multiplicity * (BMA),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(1),
            Some(10),
            multiplicity * (BMB),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(1),
            Some(10),
            multiplicity * (BMC),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(8),
            Some(7),
            multiplicity * (BNG),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(9),
            Some(7),
            multiplicity * (BNH),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(7),
            Some(3),
            multiplicity * (BMF),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(6),
            Some(3),
            multiplicity * (BMG),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(1),
            Some(3),
            multiplicity * (BMH),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(0),
            Some(10),
            multiplicity * (BMQ),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(10),
            Some(11),
            multiplicity * (BNI),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(11),
            Some(8),
            multiplicity * (BNK),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(0),
            Some(10),
            multiplicity * (BMS),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(10),
            Some(8),
            multiplicity * (BNM),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(0),
            Some(11),
            multiplicity * (BMU),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(11),
            Some(8),
            multiplicity * (BNO),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(0),
            Some(8),
            multiplicity * (BMW),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        self.canonical_reactive[0] = BHR;
        self.canonical_reactive[1] = BHS;
        self.canonical_reactive[2] = BHT;
        self.canonical_reactive[3] = BHU;
        self.canonical_reactive[4] = BNA;
        self.canonical_reactive[5] = BNB;
        self.canonical_reactive[6] = BHX;
        self.canonical_reactive[7] = BHY;
        self.canonical_reactive[8] = BHZ;
        self.canonical_reactive[9] = BIA;
        self.canonical_reactive[10] = BIB;
        self.canonical_reactive[11] = BID;
        self.canonical_reactive[12] = BIF;
        self.canonical_reactive[13] = BIH;
        self.canonical_reactive[14] = BIX;
        self.canonical_reactive[15] = BNQ;
        self.canonical_reactive[16] = DCH;
        self.canonical_reactive[17] = BJB;
        self.canonical_reactive[18] = BNR;
        self.canonical_reactive[19] = DCI;
        self.canonical_reactive[20] = DCJ;
        self.canonical_reactive[21] = DCK;
        self.canonical_reactive[22] = DCL;
        self.canonical_reactive[23] = DCM;
        self.canonical_reactive[24] = BNS;
        self.canonical_reactive[25] = DCN;
        self.canonical_reactive[26] = DCO;
        self.canonical_reactive[27] = DCP;
        self.canonical_reactive[28] = BNT;
        self.canonical_reactive[29] = DCQ;
        self.canonical_reactive[30] = DCR;
        self.canonical_reactive[31] = DCS;
        self.canonical_reactive[32] = DCT;
        self.canonical_reactive[33] = DCU;
        self.canonical_reactive[34] = BNU;
        self.canonical_reactive[35] = DCV;
        self.canonical_reactive[36] = DCW;
        self.canonical_reactive[37] = DCX;
        self.canonical_reactive[38] = BNV;
        self.canonical_reactive[39] = DCY;
        self.canonical_reactive[40] = DCZ;
        self.canonical_reactive[41] = DDA;
        self.canonical_reactive[42] = DDB;
        self.canonical_reactive[43] = DDC;
        self.canonical_reactive[44] = DDD;
        self.canonical_reactive[45] = BNW;
        self.canonical_reactive[46] = DDE;
        self.canonical_reactive[47] = DDF;
        self.canonical_reactive[48] = BNX;
        self.canonical_reactive[49] = DDG;
        self.canonical_reactive[50] = DDH;
        self.canonical_reactive[51] = BJW;
        self.canonical_reactive[52] = BJY;
        self.canonical_reactive[53] = BNY;
        self.canonical_reactive[54] = DDI;
        self.canonical_reactive[55] = DDJ;
        self.canonical_reactive[56] = DDK;
        self.canonical_reactive[57] = DDL;
        self.canonical_reactive[58] = DDM;
        self.canonical_reactive[59] = DDN;
        self.canonical_reactive[60] = DDO;
        self.canonical_reactive[61] = DDP;
        self.canonical_reactive[62] = DDQ;
        self.canonical_reactive[63] = DDR;
        self.canonical_reactive[64] = BKC;
        self.canonical_reactive[65] = BNZ;
        self.canonical_reactive[66] = DDS;
        self.canonical_reactive[67] = DDT;
        self.canonical_reactive[68] = DDU;
        self.canonical_reactive[69] = DDV;
        self.canonical_reactive[70] = DDW;
        self.canonical_reactive[71] = DDX;
        self.canonical_reactive[72] = BNC;
        self.canonical_reactive[73] = BND;
        self.canonical_reactive[74] = BNE;
        self.canonical_reactive[75] = BNF;
        self.canonical_reactive[76] = BLI;
        self.canonical_reactive[77] = BLJ;
        self.canonical_reactive[78] = BOA;
        self.canonical_reactive[79] = DDY;
        self.canonical_reactive[80] = DDZ;
        self.canonical_reactive[81] = DEA;
        self.canonical_reactive[82] = DEB;
        self.canonical_reactive[83] = DEC;
        self.canonical_reactive[84] = DED;
        self.canonical_reactive[85] = BLO;
        self.canonical_reactive[86] = BLJ;
        self.canonical_reactive[87] = BLP;
        self.canonical_reactive[88] = BLQ;
        self.canonical_reactive[89] = BLR;
        self.canonical_reactive[90] = BLS;
        self.canonical_reactive[91] = BLT;
        self.canonical_reactive[92] = BLU;
        self.canonical_reactive[93] = BLV;
        self.canonical_reactive[94] = BLW;
        self.canonical_reactive[95] = BLX;
        self.canonical_reactive[96] = BLY;
        self.canonical_reactive[97] = BLZ;
        self.canonical_reactive[98] = BMA;
        self.canonical_reactive[99] = BMB;
        self.canonical_reactive[100] = BMC;
        self.canonical_reactive[101] = BNG;
        self.canonical_reactive[102] = BNH;
        self.canonical_reactive[103] = BMF;
        self.canonical_reactive[104] = BMG;
        self.canonical_reactive[105] = BMH;
        self.canonical_reactive[106] = BMQ;
        self.canonical_reactive[107] = BNI;
        self.canonical_reactive[108] = BNK;
        self.canonical_reactive[109] = BMS;
        self.canonical_reactive[110] = BNM;
        self.canonical_reactive[111] = BMU;
        self.canonical_reactive[112] = BNO;
        self.canonical_reactive[113] = BMW;
    }

    pub fn stamp_reactive(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedReactiveStamper<'_>) {
        let multiplicity = self.multiplicity;
        let cached = &*self.canonical_reactive;
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(4),
            None,
            &[4],
            &[cached[16]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(7),
            Some(5),
            &[4, 5, 7, 8, 9],
            &[cached[19], cached[20], cached[21], cached[22], cached[23]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(6),
            Some(5),
            &[4, 5, 6],
            &[cached[25], cached[26], cached[27]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(7),
            Some(9),
            &[4, 5, 7, 8, 9],
            &[cached[29], cached[30], cached[31], cached[32], cached[33]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(3),
            Some(8),
            &[3, 4, 8],
            &[cached[35], cached[36], cached[37]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(6),
            Some(7),
            &[4, 5, 6, 7, 8, 9],
            &[cached[39], cached[40], cached[41], cached[42], cached[43], cached[44]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(1),
            Some(2),
            &[1, 2],
            &[cached[46], cached[47]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(1),
            Some(0),
            &[0, 1],
            &[cached[49], cached[50]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(1),
            Some(10),
            &[0, 1, 3, 4, 6, 7, 8, 9, 10, 11],
            &[cached[54], cached[55], cached[56], cached[57], cached[58], cached[59], cached[60], cached[61], cached[62], cached[63]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(6),
            Some(11),
            &[4, 6, 7, 8, 9, 11],
            &[cached[66], cached[67], cached[68], cached[69], cached[70], cached[71]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(7),
            Some(5),
            &[4, 5, 7, 8, 9, 12],
            &[cached[79], cached[80], cached[81], cached[82], cached[83], cached[84]],
            &[],
            &[],
            multiplicity,
        );
    }

}
