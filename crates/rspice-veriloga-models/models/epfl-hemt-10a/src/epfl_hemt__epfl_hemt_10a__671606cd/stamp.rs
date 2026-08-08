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
            let slot = 0usize;
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
            let B = 1.602e-19f64;
            let C = 1e0f64;
            let D = 2e0f64;
            let E = 8.353992494899963e17f64;
            let F = parameters[34];
            let H = node_potentials[4];
            let J = parameters[6];
            let L = 9.09e-4f64;
            let P = parameters[12];
            let S = 3.333333333333333e-1f64;
            let T = 5e-1f64;
            let V = parameters[22];
            let W = parameters[27];
            let Y = 2.59e-2f64;
            let AA = parameters[3];
            let AB = parameters[0];
            let AD = 6.434283176858164e24f64;
            let AE = 3.204e-19f64;
            let AJ = parameters[11];
            let AL = 1e-38f64;
            let AP = node_potentials[6];
            let AU = parameters[31];
            let BE = parameters[20];
            let BF = parameters[21];
            let BO = parameters[7];
            let BP = parameters[9];
            let BW = 3e0f64;
            let BX = 1.4142135623730951e0f64;
            let BZ = 6e0f64;
            let CY = 6.666e-1f64;
            let DL = 6.666666666666666e-1f64;
            let DN = 3.333333333333333e-1f64;
            let DQ = 3.333333333333333e-1f64;
            let DS = -6.666666666666666e-1f64;
            let DW = 6.666666666666666e-1f64;
            let DY = 3.333333333333333e-1f64;
            let EA = 3.333333333333333e-1f64;
            let EC = -6.666666666666666e-1f64;
            let EG = 3.333333333333333e-1f64;
            let EI = -6.666666666666666e-1f64;
            let EK = 6.666666666666666e-1f64;
            let EM = 3.333333333333333e-1f64;
            let ES = 5e1f64;
            let EY = 6.666666666666666e-1f64;
            let FA = 3.333333333333333e-1f64;
            let FD = 3.333333333333333e-1f64;
            let FF = -6.666666666666666e-1f64;
            let FJ = 6.666666666666666e-1f64;
            let FL = 3.333333333333333e-1f64;
            let FN = 3.333333333333333e-1f64;
            let FP = -6.666666666666666e-1f64;
            let FT = 3.333333333333333e-1f64;
            let FV = -6.666666666666666e-1f64;
            let FX = 6.666666666666666e-1f64;
            let FZ = 3.333333333333333e-1f64;
            let GK = 6.666666666666666e-1f64;
            let GM = 3.333333333333333e-1f64;
            let GP = 3.333333333333333e-1f64;
            let GR = -6.666666666666666e-1f64;
            let GV = 6.666666666666666e-1f64;
            let GX = 3.333333333333333e-1f64;
            let GZ = 3.333333333333333e-1f64;
            let HB = -6.666666666666666e-1f64;
            let HF = 6.666666666666666e-1f64;
            let HH = 3.333333333333333e-1f64;
            let HJ = 3.333333333333333e-1f64;
            let HL = -6.666666666666666e-1f64;
            let HP = 3.333333333333333e-1f64;
            let HR = -6.666666666666666e-1f64;
            let HT = 6.666666666666666e-1f64;
            let HV = 3.333333333333333e-1f64;
            let IN = parameters[30];
            let IO = parameters[24];
            let IQ = parameters[26];
            let IR = parameters[15];
            let JA = 4e0f64;
            let JG = parameters[17];
            let KI = parameters[32];
            let LC = 6.666666666666666e-1f64;
            let LE = 3.333333333333333e-1f64;
            let LH = 3.333333333333333e-1f64;
            let LJ = -6.666666666666666e-1f64;
            let LN = 6.666666666666666e-1f64;
            let LP = 3.333333333333333e-1f64;
            let LR = 3.333333333333333e-1f64;
            let LT = -6.666666666666666e-1f64;
            let LX = 3.333333333333333e-1f64;
            let LZ = -6.666666666666666e-1f64;
            let MB = 6.666666666666666e-1f64;
            let MD = 3.333333333333333e-1f64;
            let MO = 6.666666666666666e-1f64;
            let MQ = 3.333333333333333e-1f64;
            let MT = 3.333333333333333e-1f64;
            let MV = -6.666666666666666e-1f64;
            let MZ = 6.666666666666666e-1f64;
            let NB = 3.333333333333333e-1f64;
            let ND = 3.333333333333333e-1f64;
            let NF = -6.666666666666666e-1f64;
            let NJ = 3.333333333333333e-1f64;
            let NL = -6.666666666666666e-1f64;
            let NN = 6.666666666666666e-1f64;
            let NP = 3.333333333333333e-1f64;
            let OA = 6.666666666666666e-1f64;
            let OC = 3.333333333333333e-1f64;
            let OF = 3.333333333333333e-1f64;
            let OH = -6.666666666666666e-1f64;
            let OL = 6.666666666666666e-1f64;
            let ON = 3.333333333333333e-1f64;
            let OP = 3.333333333333333e-1f64;
            let OR = -6.666666666666666e-1f64;
            let OV = 6.666666666666666e-1f64;
            let OX = 3.333333333333333e-1f64;
            let OZ = 3.333333333333333e-1f64;
            let PB = -6.666666666666666e-1f64;
            let PF = 3.333333333333333e-1f64;
            let PH = -6.666666666666666e-1f64;
            let PJ = 6.666666666666666e-1f64;
            let PL = 3.333333333333333e-1f64;
            let QK = 8e-1f64;
            let QL = 1.2e0f64;
            let SC = parameters[39];
            let SD = parameters[44];
            let SF = parameters[45];
            let SG = parameters[38];
            let SH = parameters[46];
            let SQ = 9.6e-1f64;
            let ST = parameters[47];
            let TB = parameters[42];
            let TD = parameters[41];
            let TN = 0e0f64;
            let TO = parameters[35];
            let TS = parameters[36];
            let TW = 1e9f64;
            let UD = 1e0f64;
            let UE = 1e0f64;
            let UF = 1e0f64;
            let UG = 1e0f64;
            let UH = 1e0f64;
            let UW = -1e0f64;
            let VL = 2e0f64;
            let AAX = Lanes([0e0f64; 4]);
            let ABF = 0e0f64;
            let ABL = ddt_scale();
            let G = if F == C { 1.0 } else { 0.0 };
            if G != 0.0 {
            } else {
            }
            let I = temperature + H;
            let K = I / J;
            let UV = UE / J;
            let M = I + 8.3e2f64;
            let N = (L * (I * I)) / M;
            let O = 5.618214e-19f64 - (N * B);
            let UX = (((((UE * (D * I)) * L) - (UE * N)) / M) * B) * UW;
            let Q = ((8.5e0f64 * P) + (8.9e0f64 * (C - P))) * 8.85418e-12f64;
            let R = if F != C { 1.0 } else { 0.0 };
            let U = if R != 0.0 {
                S
            } else {
                T
            };
            let X = V * (K.powf(W));
            let UY = (UV * (W * (K.powf((W - UD))))) * V;
            let Z = Y * K;
            let UZ = UV * Y;
            let AC = C / Z;
            let VA = ((UZ * AC) * UW) / Z;
            let AF = AE * Z;
            let AG = (-O) / AF;
            let AH = AG.exp();
            let AI = AD * AH;
            let AK = AJ / AI;
            let AM = if AK >= AL { AK } else { AL };
            let AN = AM.ln();
            let VB = (((((((((UX * UW) - ((UZ * AE) * AG)) / AF) * AH) * AD) * AK) * UW) / AI) * (if AK >= AL { 1.0 } else { 0.0 })) * (UD / AM);
            let AO = Q / parameters[5];
            let AQ = node_potentials[1] - AP;
            let VC = Lanes([UF, 0.0]) - Lanes([0.0, UG]);
            let AR = AP - AP;
            let VD = UG - UG;
            let AS = node_potentials[5] - AP;
            let VE = Lanes([UH, 0.0]) - Lanes([0.0, UG]);
            let AT = AS - AR;
            let VF = Lanes([0.0, VD]);
            let VG = VE - VF;
            let AV = AU * AT;
            let VH = VG * AU;
            let AW = if AV > 8e1f64 { 1.0 } else { 0.0 };
            let BB;
            let UI;
            if AW != 0.0 {
                BB = AV;
                UI = VH;
            } else {
                let AX = AV.exp();
                let AY = C + AX;
                let AZ = AY.ln();
                let VI = (VH * AX) * (UD / AY);
                BB = AZ;
                UI = VI;
            }
            let BA = D / AU;
            let VJ = (UI * BA) - VG;
            let BC = ((BA * BB) - AT) - (BA * 6.931471805599453e-1f64);
            let BD = -(AR + (T * (AT - BC)));
            let VK = (VF + ((VG - VJ) * T)) * UW;
            let VM = ((VJ * BE) - (VK * BF)) / AO;
            let BG = C + ((((parameters[18] + parameters[19]) + (BE * BC)) - (BF * BD)) / AO);
            let BH = BG - C;
            let VN = VM * BH;
            let BI = ((BH * BH) + 6.250000000000001e-4f64).sqrt();
            let BJ = T * ((BG + C) + BI);
            let BK = BJ * Z;
            let VO = ((VM + ((VN + VN) * (UD / (VL * BI)))) * T) * Z;
            let VP = Lanes([0.0, VO[0], VO[1]]) + Lanes([(UZ * BJ), 0.0, 0.0]);
            let BL = C / BK;
            let VQ = ((VP * BL) * UW) / BK;
            let BM = AS * BL;
            let VR = VE * BL;
            let VS = Lanes([0.0, VR[0], VR[1]]) + (VQ * AS);
            let VT = VC * BL;
            let VU = VQ * AQ;
            let BN = AR * BL;
            let VV = Lanes([0.0, 0.0, (VD * BL)]) + (VQ * AR);
            let VW = VQ * BO;
            let BQ = -(parameters[8] + (BP * BD));
            let BR = BQ * BC;
            let VX = ((((VK * BP) * UW) * BC) + (VJ * BQ)) * BL;
            let VY = Lanes([0.0, VX[0], VX[1]]) + (VQ * BR);
            let BS = ((AQ * BL) - (BO * BL)) - (BR * BL);
            let VZ = ((Lanes([VT[0], 0.0, 0.0, VT[1]]) + Lanes([0.0, VU[0], VU[1], VU[2]])) - Lanes([0.0, VW[0], VW[1], VW[2]])) - Lanes([0.0, VY[0], VY[1], VY[2]]);
            let BT = 2.52482255208e-29f64 * AJ;
            let BU = (BT * AC).sqrt();
            let BV = BU / AO;
            let WA = ((VA * BT) * (UD / (VL * BU))) / AO;
            let BY = (T * BS) - (BW * (C + (BV / BX)));
            let WB = (VZ * T) - Lanes([0.0, ((WA / BX) * BW), 0.0, 0.0]);
            let WC = WB * BY;
            let CA = ((BY * BY) + (BZ * BS)).sqrt();
            let CB = BY + CA;
            let WD = WB + (((WC + WC) + (VZ * BZ)) * (UD / (VL * CA)));
            let CC = if BS < A { 1.0 } else { 0.0 };
            let CN;
            let UJ;
            if CC != 0.0 {
                let CD = (BS - CB) / BV;
                let WJ = (((VZ - WD) - Lanes([0.0, (WA * CD), 0.0, 0.0])) / BV) * CD;
                let CE = (C - CB) + (CD * CD);
                let CF = if CE >= AL { CE } else { AL };
                let CG = -(CF.ln());
                let WK = ((((WD * UW) + (WJ + WJ)) * (if CE >= AL { 1.0 } else { 0.0 })) * (UD / CF)) * UW;
                CN = CG;
                UJ = WK;
            } else {
                let CH = -CB;
                let CI = rspice_limited_exp(CH);
                let WE = (WD * UW) * (rspice_limited_exp_derivative(CH));
                let CJ = T * BV;
                let WF = WA * T;
                let WG = WF * CJ;
                let CK = (((BS - C) + CI) + (CJ * CJ)).sqrt();
                let CL = CK - CJ;
                let WH = ((((VZ + WE) + Lanes([0.0, (WG + WG), 0.0, 0.0])) * (UD / (VL * CK))) - Lanes([0.0, WF, 0.0, 0.0])) * CL;
                let CM = ((CL * CL) + C) - CI;
                let WI = (WH + WH) - WE;
                CN = CM;
                UJ = WI;
            }
            let CO = CN - C;
            let WL = UJ * CO;
            let CP = ((CO * CO) + 1e0f64).sqrt();
            let CQ = (T * ((CN + C) + CP)).sqrt();
            let WM = ((UJ + ((WL + WL) * (UD / (VL * CP)))) * T) * (UD / (VL * CQ));
            let CR = D * CQ;
            let CS = BV / CR;
            let WN = Lanes([0.0, WA, 0.0, 0.0]);
            let WO = (WN - ((WM * D) * CS)) / CR;
            let CT = C + CS;
            let CU = (D * CT) * AO;
            let CV = CU * Z;
            let WP = (((WO * D) * AO) * Z) + Lanes([0.0, (UZ * CU), 0.0, 0.0]);
            let CW = parameters[13] / Z;
            let CX = E * Z;
            let WQ = UZ * E;
            let CZ = CX.powf(CY);
            let DA = CW * CZ;
            let WR = ((((UZ * CW) * UW) / Z) * CZ) + ((WQ * (CY * (CX.powf(-3.3340000000000003e-1f64)))) * CW);
            let DB = B * Z;
            let DC = O / DB;
            let DD = (CN - (((AJ / 1.8e25f64).ln()) + DC)) + AN;
            let WS = Lanes([0.0, VB, 0.0, 0.0]);
            let WT = (UJ - Lanes([0.0, ((UX - ((UZ * B) * DC)) / DB), 0.0, 0.0])) + WS;
            let DE = DD - BN;
            let WU = Lanes([0.0, VV[0], VV[1], VV[2]]);
            let WV = WT - WU;
            let DF = AO * CT;
            let DG = B / DF;
            let DH = (DG * CX) / Z;
            let WW = (((((((WO * AO) * DG) * UW) / DF) * CX) + Lanes([0.0, (WQ * DG), 0.0, 0.0])) - Lanes([0.0, (UZ * DH), 0.0, 0.0])) / Z;
            let DI = if DE <= A { 1.0 } else { 0.0 };
            let IB;
            let UK;
            if DI != 0.0 {
                let DJ = DE.exp();
                let DK = BZ / DJ;
                let XG = (((WV * DJ) * DK) * UW) / DJ;
                let DM = DL * DA;
                let DO = DK.powf(DN);
                let DP = C + DE;
                let DR = DQ * DA;
                let DT = DK.powf(DS);
                let DU = (DP + (DK.ln())) - (DR * DT);
                let DV = ((DK + DH) + (DM * DO)) / DU;
                let XH = (((XG + WW) + (Lanes([0.0, ((WR * DL) * DO), 0.0, 0.0]) + ((XG * (DN * (DK.powf(-6.666666666666667e-1f64)))) * DM))) - (((WV + (XG * (UD / DK))) - (Lanes([0.0, ((WR * DQ) * DT), 0.0, 0.0]) + ((XG * (DS * (DK.powf(-1.6666666666666665e0f64)))) * DR))) * DV)) / DU;
                let DX = DW * DA;
                let DZ = DV.powf(DY);
                let EB = EA * DA;
                let ED = DV.powf(EC);
                let EE = (DP + (DV.ln())) - (EB * ED);
                let EF = ((DV + DH) + (DX * DZ)) / EE;
                let XI = (((XH + WW) + (Lanes([0.0, ((WR * DW) * DZ), 0.0, 0.0]) + ((XH * (DY * (DV.powf(-6.666666666666667e-1f64)))) * DX))) - (((WV + (XH * (UD / DV))) - (Lanes([0.0, ((WR * EA) * ED), 0.0, 0.0]) + ((XH * (EC * (DV.powf(-1.6666666666666665e0f64)))) * EB))) * EF)) / EE;
                let EH = EG * DA;
                let EJ = EF.powf(EI);
                let EL = EK * DA;
                let EN = EF.powf(EM);
                let EO = (EF + DH) + (EL * EN);
                let EP = ((DP + (EF.ln())) - (EH * EJ)) / EO;
                let EQ = B * CX;
                let ER = (EQ * EP) / CV;
                let XJ = ((Lanes([0.0, ((WQ * B) * EP), 0.0, 0.0]) + (((((WV + (XI * (UD / EF))) - (Lanes([0.0, ((WR * EG) * EJ), 0.0, 0.0]) + ((XI * (EI * (EF.powf(-1.6666666666666665e0f64)))) * EH))) - (((XI + WW) + (Lanes([0.0, ((WR * EK) * EN), 0.0, 0.0]) + ((XI * (EM * (EF.powf(-6.666666666666667e-1f64)))) * EL))) * EP)) / EO) * EQ)) - (WP * ER)) / CV;
                IB = ER;
                UK = XJ;
            } else {
                let ET = if DE < ES { 1.0 } else { 0.0 };
                let IC;
                let UL;
                if ET != 0.0 {
                    let EU = DH + DA;
                    let EV = DE / EU;
                    let EW = EV + T;
                    let EX = C / EW;
                    let XC = ((((WV - ((WW + Lanes([0.0, WR, 0.0, 0.0])) * EV)) / EU) * EX) * UW) / EW;
                    let EZ = EY * DA;
                    let FB = EX.powf(FA);
                    let FC = C + DE;
                    let FE = FD * DA;
                    let FG = EX.powf(FF);
                    let FH = (FC + (EX.ln())) - (FE * FG);
                    let FI = ((EX + DH) + (EZ * FB)) / FH;
                    let XD = (((XC + WW) + (Lanes([0.0, ((WR * EY) * FB), 0.0, 0.0]) + ((XC * (FA * (EX.powf(-6.666666666666667e-1f64)))) * EZ))) - (((WV + (XC * (UD / EX))) - (Lanes([0.0, ((WR * FD) * FG), 0.0, 0.0]) + ((XC * (FF * (EX.powf(-1.6666666666666665e0f64)))) * FE))) * FI)) / FH;
                    let FK = FJ * DA;
                    let FM = FI.powf(FL);
                    let FO = FN * DA;
                    let FQ = FI.powf(FP);
                    let FR = (FC + (FI.ln())) - (FO * FQ);
                    let FS = ((FI + DH) + (FK * FM)) / FR;
                    let XE = (((XD + WW) + (Lanes([0.0, ((WR * FJ) * FM), 0.0, 0.0]) + ((XD * (FL * (FI.powf(-6.666666666666667e-1f64)))) * FK))) - (((WV + (XD * (UD / FI))) - (Lanes([0.0, ((WR * FN) * FQ), 0.0, 0.0]) + ((XD * (FP * (FI.powf(-1.6666666666666665e0f64)))) * FO))) * FS)) / FR;
                    let FU = FT * DA;
                    let FW = FS.powf(FV);
                    let FY = FX * DA;
                    let GA = FS.powf(FZ);
                    let GB = (FS + DH) + (FY * GA);
                    let GC = ((FC + (FS.ln())) - (FU * FW)) / GB;
                    let GD = B * CX;
                    let GE = (GD * GC) / CV;
                    let XF = ((Lanes([0.0, ((WQ * B) * GC), 0.0, 0.0]) + (((((WV + (XE * (UD / FS))) - (Lanes([0.0, ((WR * FT) * FW), 0.0, 0.0]) + ((XE * (FV * (FS.powf(-1.6666666666666665e0f64)))) * FU))) - (((XE + WW) + (Lanes([0.0, ((WR * FX) * GA), 0.0, 0.0]) + ((XE * (FZ * (FS.powf(-6.666666666666667e-1f64)))) * FY))) * GC)) / GB) * GD)) - (WP * GE)) / CV;
                    IC = GE;
                    UL = XF;
                } else {
                    let GF = DH + DA;
                    let GG = DE / GF;
                    let GH = GG + T;
                    let GI = BW / GH;
                    let WX = ((((WV - ((WW + Lanes([0.0, WR, 0.0, 0.0])) * GG)) / GF) * GI) * UW) / GH;
                    let GJ = C + DH;
                    let GL = GK * DA;
                    let GN = GI.powf(GM);
                    let GO = T + DE;
                    let GQ = GP * DA;
                    let GS = GI.powf(GR);
                    let GT = GO - (GQ * GS);
                    let GU = (GJ + (GL * GN)) / GT;
                    let WY = ((WW + (Lanes([0.0, ((WR * GK) * GN), 0.0, 0.0]) + ((WX * (GM * (GI.powf(-6.666666666666667e-1f64)))) * GL))) - ((WV - (Lanes([0.0, ((WR * GP) * GS), 0.0, 0.0]) + ((WX * (GR * (GI.powf(-1.6666666666666665e0f64)))) * GQ))) * GU)) / GT;
                    let GW = GV * DA;
                    let GY = GU.powf(GX);
                    let HA = GZ * DA;
                    let HC = GU.powf(HB);
                    let HD = GO - (HA * HC);
                    let HE = (GJ + (GW * GY)) / HD;
                    let WZ = ((WW + (Lanes([0.0, ((WR * GV) * GY), 0.0, 0.0]) + ((WY * (GX * (GU.powf(-6.666666666666667e-1f64)))) * GW))) - ((WV - (Lanes([0.0, ((WR * GZ) * HC), 0.0, 0.0]) + ((WY * (HB * (GU.powf(-1.6666666666666665e0f64)))) * HA))) * HE)) / HD;
                    let HG = HF * DA;
                    let HI = HE.powf(HH);
                    let HK = HJ * DA;
                    let HM = HE.powf(HL);
                    let HN = GO - (HK * HM);
                    let HO = (GJ + (HG * HI)) / HN;
                    let XA = ((WW + (Lanes([0.0, ((WR * HF) * HI), 0.0, 0.0]) + ((WZ * (HH * (HE.powf(-6.666666666666667e-1f64)))) * HG))) - ((WV - (Lanes([0.0, ((WR * HJ) * HM), 0.0, 0.0]) + ((WZ * (HL * (HE.powf(-1.6666666666666665e0f64)))) * HK))) * HO)) / HN;
                    let HQ = HP * DA;
                    let HS = HO.powf(HR);
                    let HU = HT * DA;
                    let HW = HO.powf(HV);
                    let HX = GJ + (HU * HW);
                    let HY = ((C + DE) - (HQ * HS)) / HX;
                    let HZ = B * CX;
                    let IA = (HZ * HY) / CV;
                    let XB = ((Lanes([0.0, ((WQ * B) * HY), 0.0, 0.0]) + ((((WV - (Lanes([0.0, ((WR * HP) * HS), 0.0, 0.0]) + ((XA * (HR * (HO.powf(-1.6666666666666665e0f64)))) * HQ))) - ((WW + (Lanes([0.0, ((WR * HT) * HW), 0.0, 0.0]) + ((XA * (HV * (HO.powf(-6.666666666666667e-1f64)))) * HU))) * HY)) / HX) * HZ)) - (WP * IA)) / CV;
                    IC = IA;
                    UL = XB;
                }
                IB = IC;
                UK = UL;
            }
            let ID = D * IB;
            let XK = UK * D;
            let IE = CN - ID;
            let XL = UJ - XK;
            let IF = IE - C;
            let XM = XL * IF;
            let IG = ((IF * IF) + 1e0f64).sqrt();
            let IH = (T * ((IE + C) + IG)).sqrt();
            let II = CQ + IH;
            let IJ = BV / II;
            let XN = (WN - ((WM + (((XL + ((XM + XM) * (UD / (VL * IG)))) * T) * (UD / (VL * IH)))) * IJ)) / II;
            let IK = C + IJ;
            let IL = AO / (parameters[14] * 7.8802202e-11f64);
            let IM = BS - CN;
            let XO = VZ - UJ;
            let XP = IN - UD;
            let XQ = VK * IO;
            let IP = parameters[23] + (IO * BD);
            let XR = IQ - UD;
            let IS = -parameters[28];
            let IT = (IR * (K.powf(IS))) * AB;
            let IU = (D * BK) / IT;
            let XS = ((VP * D) - Lanes([((((UV * (IS * (K.powf((IS - UD))))) * IR) * AB) * IU), 0.0, 0.0])) / IT;
            let IV = IU * IU;
            let XT = XS * IU;
            let XU = XT + XT;
            let IW = D * IU;
            let XV = XS * D;
            let IX = D + IU;
            let IY = IW * IB;
            let XW = XV * IB;
            let XX = Lanes([0.0, XW[0], XW[1], XW[2]]) + (UK * IW);
            let IZ = (IB * IB) + IB;
            let XY = XV * IZ;
            let XZ = Lanes([0.0, XS[0], XS[1], XS[2]]);
            let YA = XS * IX;
            let YB = YA + YA;
            let JB = ((IX * IX) + (JA * IY)).sqrt();
            let JC = (IX + IY) + JB;
            let JD = (IW * IZ) / JC;
            let YC = ((Lanes([0.0, XY[0], XY[1], XY[2]]) + (((UK * ID) + UK) * IW)) - (((XZ + XX) + ((Lanes([0.0, YB[0], YB[1], YB[2]]) + (XX * JA)) * (UD / (VL * JB)))) * JD)) / JC;
            let JE = IB - JD;
            let YD = UK - YC;
            let JF = JE * JE;
            let YE = YD * JE;
            let YF = YE + YE;
            let JH = D - JG;
            let JI = (D * JD) + (JD.ln());
            let YG = XS * JE;
            let JJ = C + (IU * JE);
            let JK = IU * JH;
            let YH = (XS * JH) * JE;
            let JL = 1e-1f64 + (JK * JE);
            let JM = ((D * IV) * JH) * JH;
            let YI = (((XU * D) * JH) * JH) * JF;
            let JN = (JM * JF) / JL;
            let YJ = XU * JF;
            let JO = ((C + JN) + (IV * JF)).sqrt();
            let JP = (JI * JJ) / JO;
            let JQ = ((CN - AN) - JP) - BN;
            let YK = ((UJ - WS) - ((((((YC * D) + (YC * (UD / JD))) * JJ) + ((Lanes([0.0, YG[0], YG[1], YG[2]]) + (YD * IU)) * JI)) - ((((((Lanes([0.0, YI[0], YI[1], YI[2]]) + (YF * JM)) - ((Lanes([0.0, YH[0], YH[1], YH[2]]) + (YD * JK)) * JN)) / JL) + (Lanes([0.0, YJ[0], YJ[1], YJ[2]]) + (YF * IV))) * (UD / (VL * JO))) * JP)) / JO)) - WU;
            let JR = JQ - BW;
            let YL = YK * JR;
            let JS = ((JR * JR) + 4e0f64).sqrt();
            let JT = T * ((JQ + BW) + JS);
            let YM = (YK + ((YL + YL) * (UD / (VL * JS)))) * T;
            let JU = parameters[16] / JG;
            let JV = IB + C;
            let JW = (JU * ((JA * JD) + JG)) / JV;
            let JX = BM - BN;
            let JY = JA * JW;
            let YN = ((((YC * JA) * JU) - (UK * JW)) / JV) * JA;
            let JZ = JY / JT;
            let KA = (C + JZ).sqrt();
            let KB = JX * KA;
            let YO = (VS - VV) * KA;
            let YP = Lanes([0.0, YO[0], YO[1], YO[2]]) + ((((YN - (YM * JZ)) / JT) * (UD / (VL * KA))) * JX);
            let KC = KB + JT;
            let YQ = (YP + YM) * KC;
            let KD = JY * JT;
            let YR = (YN * JT) + (YM * JY);
            let KE = ((KC * KC) + KD).sqrt();
            let KF = KB - JT;
            let YS = (YP - YM) * KF;
            let KG = ((KF * KF) + KD).sqrt();
            let KH = (T * (KE - KG)) + BN;
            let YT = (((((YQ + YQ) + YR) * (UD / (VL * KE))) - (((YS + YS) + YR) * (UD / (VL * KG)))) * T) + WU;
            let KJ = ((T * IU) * AB) / KI;
            let KK = BM - KH;
            let KL = KJ * KK;
            let YU = (((XS * T) * AB) / KI) * KK;
            let YV = Lanes([0.0, YU[0], YU[1], YU[2]]) + ((Lanes([0.0, VS[0], VS[1], VS[2]]) - YT) * KJ);
            let KM = KI / (AB - (D * KI));
            let KN = parameters[33] * KI;
            let YW = YV * KL;
            let KO = D * KM;
            let KP = (((KL * KL) + (KO * KL)) + C).sqrt();
            let KQ = KM + C;
            let KR = ((KM + KL) + KP) / KQ;
            let KS = KN * (KR.ln());
            let YX = (((YV + (((YW + YW) + (YV * KO)) * (UD / (VL * KP)))) / KQ) * (UD / KR)) * KN;
            let KT = DD - KH;
            let YY = WT - YT;
            let KU = (D * IK) * AO;
            let KV = KU * Z;
            let YZ = (((XN * D) * AO) * Z) + Lanes([0.0, (UZ * KU), 0.0, 0.0]);
            let KW = AO * IK;
            let KX = B / KW;
            let KY = (KX * CX) / Z;
            let ZA = (((((((XN * AO) * KX) * UW) / KW) * CX) + Lanes([0.0, (WQ * KX), 0.0, 0.0])) - Lanes([0.0, (UZ * KY), 0.0, 0.0])) / Z;
            let KZ = if KT <= A { 1.0 } else { 0.0 };
            let PR;
            let UM;
            if KZ != 0.0 {
                let LA = KT.exp();
                let LB = BZ / LA;
                let ZK = (((YY * LA) * LB) * UW) / LA;
                let LD = LC * DA;
                let LF = LB.powf(LE);
                let LG = C + KT;
                let LI = LH * DA;
                let LK = LB.powf(LJ);
                let LL = (LG + (LB.ln())) - (LI * LK);
                let LM = ((LB + KY) + (LD * LF)) / LL;
                let ZL = (((ZK + ZA) + (Lanes([0.0, ((WR * LC) * LF), 0.0, 0.0]) + ((ZK * (LE * (LB.powf(-6.666666666666667e-1f64)))) * LD))) - (((YY + (ZK * (UD / LB))) - (Lanes([0.0, ((WR * LH) * LK), 0.0, 0.0]) + ((ZK * (LJ * (LB.powf(-1.6666666666666665e0f64)))) * LI))) * LM)) / LL;
                let LO = LN * DA;
                let LQ = LM.powf(LP);
                let LS = LR * DA;
                let LU = LM.powf(LT);
                let LV = (LG + (LM.ln())) - (LS * LU);
                let LW = ((LM + KY) + (LO * LQ)) / LV;
                let ZM = (((ZL + ZA) + (Lanes([0.0, ((WR * LN) * LQ), 0.0, 0.0]) + ((ZL * (LP * (LM.powf(-6.666666666666667e-1f64)))) * LO))) - (((YY + (ZL * (UD / LM))) - (Lanes([0.0, ((WR * LR) * LU), 0.0, 0.0]) + ((ZL * (LT * (LM.powf(-1.6666666666666665e0f64)))) * LS))) * LW)) / LV;
                let LY = LX * DA;
                let MA = LW.powf(LZ);
                let MC = MB * DA;
                let ME = LW.powf(MD);
                let MF = (LW + KY) + (MC * ME);
                let MG = ((LG + (LW.ln())) - (LY * MA)) / MF;
                let MH = B * CX;
                let MI = (MH * MG) / KV;
                let ZN = ((Lanes([0.0, ((WQ * B) * MG), 0.0, 0.0]) + (((((YY + (ZM * (UD / LW))) - (Lanes([0.0, ((WR * LX) * MA), 0.0, 0.0]) + ((ZM * (LZ * (LW.powf(-1.6666666666666665e0f64)))) * LY))) - (((ZM + ZA) + (Lanes([0.0, ((WR * MB) * ME), 0.0, 0.0]) + ((ZM * (MD * (LW.powf(-6.666666666666667e-1f64)))) * MC))) * MG)) / MF) * MH)) - (YZ * MI)) / KV;
                PR = MI;
                UM = ZN;
            } else {
                let MJ = if KT < ES { 1.0 } else { 0.0 };
                let PS;
                let UN;
                if MJ != 0.0 {
                    let MK = KY + DA;
                    let ML = KT / MK;
                    let MM = ML + T;
                    let MN = C / MM;
                    let ZG = ((((YY - ((ZA + Lanes([0.0, WR, 0.0, 0.0])) * ML)) / MK) * MN) * UW) / MM;
                    let MP = MO * DA;
                    let MR = MN.powf(MQ);
                    let MS = C + KT;
                    let MU = MT * DA;
                    let MW = MN.powf(MV);
                    let MX = (MS + (MN.ln())) - (MU * MW);
                    let MY = ((MN + KY) + (MP * MR)) / MX;
                    let ZH = (((ZG + ZA) + (Lanes([0.0, ((WR * MO) * MR), 0.0, 0.0]) + ((ZG * (MQ * (MN.powf(-6.666666666666667e-1f64)))) * MP))) - (((YY + (ZG * (UD / MN))) - (Lanes([0.0, ((WR * MT) * MW), 0.0, 0.0]) + ((ZG * (MV * (MN.powf(-1.6666666666666665e0f64)))) * MU))) * MY)) / MX;
                    let NA = MZ * DA;
                    let NC = MY.powf(NB);
                    let NE = ND * DA;
                    let NG = MY.powf(NF);
                    let NH = (MS + (MY.ln())) - (NE * NG);
                    let NI = ((MY + KY) + (NA * NC)) / NH;
                    let ZI = (((ZH + ZA) + (Lanes([0.0, ((WR * MZ) * NC), 0.0, 0.0]) + ((ZH * (NB * (MY.powf(-6.666666666666667e-1f64)))) * NA))) - (((YY + (ZH * (UD / MY))) - (Lanes([0.0, ((WR * ND) * NG), 0.0, 0.0]) + ((ZH * (NF * (MY.powf(-1.6666666666666665e0f64)))) * NE))) * NI)) / NH;
                    let NK = NJ * DA;
                    let NM = NI.powf(NL);
                    let NO = NN * DA;
                    let NQ = NI.powf(NP);
                    let NR = (NI + KY) + (NO * NQ);
                    let NS = ((MS + (NI.ln())) - (NK * NM)) / NR;
                    let NT = B * CX;
                    let NU = (NT * NS) / KV;
                    let ZJ = ((Lanes([0.0, ((WQ * B) * NS), 0.0, 0.0]) + (((((YY + (ZI * (UD / NI))) - (Lanes([0.0, ((WR * NJ) * NM), 0.0, 0.0]) + ((ZI * (NL * (NI.powf(-1.6666666666666665e0f64)))) * NK))) - (((ZI + ZA) + (Lanes([0.0, ((WR * NN) * NQ), 0.0, 0.0]) + ((ZI * (NP * (NI.powf(-6.666666666666667e-1f64)))) * NO))) * NS)) / NR) * NT)) - (YZ * NU)) / KV;
                    PS = NU;
                    UN = ZJ;
                } else {
                    let NV = KY + DA;
                    let NW = KT / NV;
                    let NX = NW + T;
                    let NY = BW / NX;
                    let ZB = ((((YY - ((ZA + Lanes([0.0, WR, 0.0, 0.0])) * NW)) / NV) * NY) * UW) / NX;
                    let NZ = C + KY;
                    let OB = OA * DA;
                    let OD = NY.powf(OC);
                    let OE = T + KT;
                    let OG = OF * DA;
                    let OI = NY.powf(OH);
                    let OJ = OE - (OG * OI);
                    let OK = (NZ + (OB * OD)) / OJ;
                    let ZC = ((ZA + (Lanes([0.0, ((WR * OA) * OD), 0.0, 0.0]) + ((ZB * (OC * (NY.powf(-6.666666666666667e-1f64)))) * OB))) - ((YY - (Lanes([0.0, ((WR * OF) * OI), 0.0, 0.0]) + ((ZB * (OH * (NY.powf(-1.6666666666666665e0f64)))) * OG))) * OK)) / OJ;
                    let OM = OL * DA;
                    let OO = OK.powf(ON);
                    let OQ = OP * DA;
                    let OS = OK.powf(OR);
                    let OT = OE - (OQ * OS);
                    let OU = (NZ + (OM * OO)) / OT;
                    let ZD = ((ZA + (Lanes([0.0, ((WR * OL) * OO), 0.0, 0.0]) + ((ZC * (ON * (OK.powf(-6.666666666666667e-1f64)))) * OM))) - ((YY - (Lanes([0.0, ((WR * OP) * OS), 0.0, 0.0]) + ((ZC * (OR * (OK.powf(-1.6666666666666665e0f64)))) * OQ))) * OU)) / OT;
                    let OW = OV * DA;
                    let OY = OU.powf(OX);
                    let PA = OZ * DA;
                    let PC = OU.powf(PB);
                    let PD = OE - (PA * PC);
                    let PE = (NZ + (OW * OY)) / PD;
                    let ZE = ((ZA + (Lanes([0.0, ((WR * OV) * OY), 0.0, 0.0]) + ((ZD * (OX * (OU.powf(-6.666666666666667e-1f64)))) * OW))) - ((YY - (Lanes([0.0, ((WR * OZ) * PC), 0.0, 0.0]) + ((ZD * (PB * (OU.powf(-1.6666666666666665e0f64)))) * PA))) * PE)) / PD;
                    let PG = PF * DA;
                    let PI = PE.powf(PH);
                    let PK = PJ * DA;
                    let PM = PE.powf(PL);
                    let PN = NZ + (PK * PM);
                    let PO = ((C + KT) - (PG * PI)) / PN;
                    let PP = B * CX;
                    let PQ = (PP * PO) / KV;
                    let ZF = ((Lanes([0.0, ((WQ * B) * PO), 0.0, 0.0]) + ((((YY - (Lanes([0.0, ((WR * PF) * PI), 0.0, 0.0]) + ((ZE * (PH * (PE.powf(-1.6666666666666665e0f64)))) * PG))) - ((ZA + (Lanes([0.0, ((WR * PJ) * PM), 0.0, 0.0]) + ((ZE * (PL * (PE.powf(-6.666666666666667e-1f64)))) * PK))) * PO)) / PN) * PP)) - (YZ * PQ)) / KV;
                    PS = PQ;
                    UN = ZF;
                }
                PR = PS;
                UM = UN;
            }
            let PT = (CN - IB) - PR;
            let ZO = (UJ - UK) - UM;
            let PU = PT - C;
            let ZP = ZO * PU;
            let PV = ((PU * PU) + 1e0f64).sqrt();
            let PW = (T * ((PT + C) + PV)).sqrt();
            let PX = CQ + PW;
            let PY = BV / PX;
            let ZQ = (WN - ((WM + (((ZO + ((ZP + ZP) * (UD / (VL * PV)))) * T) * (UD / (VL * PW)))) * PY)) / PX;
            let PZ = C + PY;
            let QA = IB - PR;
            let ZR = UK - UM;
            let QB = QA * QA;
            let ZS = ZR * QA;
            let QC = JV + PR;
            let ZT = UK + UM;
            let QD = C / QC;
            let ZU = ((ZT * QD) * UW) / QC;
            let QE = QB * QD;
            let ZV = ((ZS + ZS) * QD) + (ZU * QB);
            let QF = PZ - C;
            let QG = (IB + PR) + (S * QE);
            let QH = IM - (QF * QG);
            let QI = S * PZ;
            let ZW = ZQ * S;
            let QJ = QE * QD;
            let ZX = (ZV * QD) + (ZU * QE);
            let QM = T * ((C + (QK * IB)) + (QL * PR));
            let QN = (ID + PR) + (QM * QJ);
            let QO = T * ((C + (QL * IB)) + (QK * PR));
            let QP = (IB + (D * PR)) + (QO * QJ);
            let QQ = BK * QH;
            let ZY = VP * QH;
            let ZZ = Lanes([0.0, ZY[0], ZY[1], ZY[2]]) + ((XO - ((ZQ * QG) + ((ZT + (ZV * S)) * QF))) * BK);
            let AAA = ZZ * QQ;
            let QR = ((QQ * QQ) + 2.5000000000000005e-3f64).sqrt();
            let QS = T * (QQ + QR);
            let AAB = (ZZ + ((AAA + AAA) * (UD / (VL * QR)))) * T;
            let QT = (QI * QN) + (QI * QP);
            let QU = BK * QT;
            let AAC = VP * QT;
            let AAD = Lanes([0.0, AAC[0], AAC[1], AAC[2]]) + ((((ZW * QN) + (((XK + UM) + (((((UK * QK) + (UM * QL)) * T) * QJ) + (ZX * QM))) * QI)) + ((ZW * QP) + (((UK + (UM * D)) + (((((UK * QL) + (UM * QK)) * T) * QJ) + (ZX * QO))) * QI))) * BK);
            let QV = IL * (QS + (U * QU));
            let QW = QU / QS;
            let QX = T * (C + QW);
            let QY = QX.powf(IN);
            let QZ = QV.powf(IQ);
            let AAE = XQ * QZ;
            let RA = parameters[25] / QY;
            let AAF = (Lanes([0.0, 0.0, AAE[0], AAE[1]]) + ((((AAB + (AAD * U)) * IL) * (IQ * (QV.powf(XR)))) * IP)) + (((((((AAD - (AAB * QW)) / QS) * T) * (IN * (QX.powf(XP)))) * RA) * UW) / QY);
            let RB = C + ((IP * QZ) + RA);
            let RC = RB - C;
            let AAG = AAF * RC;
            let RD = ((RC * RC) + 5.625e-7f64).sqrt();
            let RE = T * ((RB + C) + RD);
            let AAH = (AAF + ((AAG + AAG) * (UD / (VL * RD)))) * T;
            let RF = IU / RE;
            let RG = D * RF;
            let RH = RG * QA;
            let AAI = ((((XZ - (AAH * RF)) / RE) * D) * QA) + (ZR * RG);
            let RI = RH * RH;
            let AAJ = AAI * RH;
            let RJ = (C + RI).sqrt();
            let AAK = (AAJ + AAJ) * (UD / (VL * RJ));
            let RK = if RH != A { 1.0 } else { 0.0 };
            let RQ;
            let UO;
            if RK != 0.0 {
                let RL = C / RH;
                let RM = RH.asinh();
                let RN = T * (RJ + (RL * RM));
                let AAM = (AAK + (((((AAI * RL) * UW) / RH) * RM) + ((AAI * (UD / ((UD + RI).sqrt()))) * RL))) * T;
                RQ = RN;
                UO = AAM;
            } else {
                let RO = C / RJ;
                let RP = T * (RJ + RO);
                let AAL = (AAK + (((AAK * RO) * UW) / RJ)) * T;
                RQ = RP;
                UO = AAL;
            }
            let RR = RE * RQ;
            let AAN = (AAH * RQ) + (UO * RE);
            let RS = X / RR;
            let RT = D * parameters[4];
            let RU = RT * PZ;
            let RV = AB - KS;
            let AAO = YX * UW;
            let RW = ((RU * RS) * AA) / RV;
            let RX = RW * AO;
            let RY = RX * BK;
            let AAP = VP * RX;
            let RZ = RY * BK;
            let AAQ = VP * RY;
            let SA = QA * QC;
            let SB = RZ * SA;
            let AAR = ((((((((((((ZQ * RT) * RS) + (((Lanes([0.0, UY, 0.0, 0.0]) - (AAN * RS)) / RR) * RU)) * AA) - (AAO * RW)) / RV) * AO) * BK) + Lanes([0.0, AAP[0], AAP[1], AAP[2]])) * BK) + Lanes([0.0, AAQ[0], AAQ[1], AAQ[2]])) * SA) + (((ZR * QC) + (ZT * QA)) * RZ);
            let SE = C - (SD * (K - C));
            let AAS = (UV * SD) * UW;
            let SI = (B * parameters[37]) * AA;
            let SJ = SI * (SG * (K.powf(SH)));
            let AAT = ((UV * (SH * (K.powf((SH - UD))))) * SG) * SI;
            let SK = parameters[1] / SJ;
            let SL = SK * SE;
            let AAU = ((((AAT * SK) * UW) / SJ) * SE) + (AAS * SK);
            let SM = parameters[2] / SJ;
            let SN = SM * SE;
            let AAV = ((((AAT * SM) * UW) / SJ) * SE) + (AAS * SM);
            let SO = SI * (SC * (SE.powf(SF)));
            let SP = SB / SO;
            let AAW = (AAR - Lanes([0.0, ((((AAS * (SF * (SE.powf((SF - UD))))) * SC) * SI) * SP), 0.0, 0.0])) / SO;
            let SR = if SP >= SQ { 1.0 } else { 0.0 };
            let SS;
            let UP;
            if SR != 0.0 {
                SS = SQ;
                UP = AAX;
            } else {
                SS = SP;
                UP = AAW;
            }
            let SU = C - (SS.powf(ST));
            let SV = C / ST;
            let SW = SU.powf(SV);
            let AAY = ((UP * (ST * (SS.powf((ST - UD))))) * UW) * (SV * (SU.powf((SV - UD))));
            let SX = parameters[40] / AA;
            let SY = parameters[43] / AA;
            let SZ = SL / SW;
            let TA = SN / SW;
            let TC = (SY * (C + (TB * K))) + SZ;
            let AAZ = Lanes([0.0, ((UV * TB) * SY), 0.0, 0.0]) + ((Lanes([0.0, AAU, 0.0, 0.0]) - (AAY * SZ)) / SW);
            let TE = (SX * (C + (TD * K))) + TA;
            let ABA = Lanes([0.0, ((UV * TD) * SX), 0.0, 0.0]) + ((Lanes([0.0, AAV, 0.0, 0.0]) - (AAY * TA)) / SW);
            let TF = V / RR;
            let TG = ((TF * AO) * AA) / RV;
            let TH = TG * QU;
            let TI = TE + TC;
            let TJ = C + (TH * TI);
            let TK = SB / TJ;
            let ABB = (AAR - ((((((((((((AAN * TF) * UW) / RR) * AO) * AA) - (AAO * TG)) / RV) * QU) + (AAD * TG)) * TI) + ((ABA + AAZ) * TH)) * TK)) / TJ;
            let TL = TC * TK;
            let ABC = (AAZ * TK) + (ABB * TC);
            let TM = TE * TK;
            let ABD = (ABA * TK) + (ABB * TE);
            let TP = if TO != A { 1.0 } else { 0.0 };
            let TY;
            let TZ;
            let UA;
            let UB;
            let UC;
            let UQ;
            let UR;
            let US;
            let UT;
            let UU;
            if TP != 0.0 {
                let ABG = VE * TK;
                let TQ = TK * TK;
                let ABH = ABB * TK;
                let ABI = ABH + ABH;
                let TR = -(((TK * AS) + (TQ * TC)) + (TQ * TE));
                let ABJ = ((((ABB * AS) + Lanes([0.0, 0.0, ABG[0], ABG[1]])) + ((ABI * TC) + (AAZ * TQ))) + ((ABI * TE) + (ABA * TQ))) * UW;
                let TT = TS * H;
                let ABK = UE * TS;
                let TU = ddt(4922, TT);
                let ABM = ABK * ABL;
                let TV = H / TO;
                let ABN = UE / TO;
                TY = TU;
                TZ = TR;
                UA = TV;
                UB = A;
                UC = TT;
                UQ = ABM;
                UR = ABJ;
                US = ABN;
                UT = ABF;
                UU = ABK;
            } else {
                let TX = H * TW;
                let ABE = UE * TW;
                TY = A;
                TZ = A;
                UA = A;
                UB = TX;
                UC = A;
                UQ = ABF;
                UR = AAX;
                US = ABF;
                UT = ABE;
                UU = ABF;
            }
            let ABO = ABD[0];
            let ABP = ABD[1];
            let ABQ = ABD[2];
            let ABR = ABD[3];
            let ABS = ABB[0];
            let ABT = ABB[1];
            let ABU = ABB[2];
            let ABV = ABB[3];
            let ABW = ABC[0];
            let ABX = ABC[1];
            let ABY = ABC[2];
            let ABZ = ABC[3];
            let ACA = UQ;
            let ACB = UR[0];
            let ACC = UR[1];
            let ACD = UR[2];
            let ACE = UR[3];
            let ACF = US;
            let ACG = UT;
            let ACH = UU;
        stamper.stamp_potential_branch_local(Some(6), Some(2), 0, multiplicity);
        stamper.stamp_potential_sparse_local::<4, 0>(
            0,
            TM,
            [1, 4, 5, 6],
            [ABO, ABP, ABQ, ABR],
            [],
            [],
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(5),
            Some(6),
            multiplicity * (TK),
            [1, 4, 5, 6],
            [ABS, ABT, ABU, ABV],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(0), Some(5), 1, multiplicity);
        stamper.stamp_potential_sparse_local::<4, 0>(
            1,
            TL,
            [1, 4, 5, 6],
            [ABW, ABX, ABY, ABZ],
            [],
            [],
        );
        stamper.stamp_potential_branch_local(Some(3), Some(9), 2, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            2,
            TN,
            [],
            [],
            [],
            [],
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(4),
            None,
            multiplicity * (TY),
            [4],
            [ACA],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(4),
            None,
            multiplicity * (TZ),
            [1, 4, 5, 6],
            [ACB, ACC, ACD, ACE],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(4),
            None,
            multiplicity * (UA),
            [4],
            [ACF],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(4),
            None,
            multiplicity * (UB),
            [4],
            [ACG],
            [],
            [],
            multiplicity,
        );
        self.canonical_reactive[0] = TM;
        self.canonical_reactive[1] = TK;
        self.canonical_reactive[2] = TL;
        self.canonical_reactive[3] = TN;
        self.canonical_reactive[4] = UC;
        self.canonical_reactive[5] = ACH;
        self.canonical_reactive[6] = TZ;
        self.canonical_reactive[7] = UA;
        self.canonical_reactive[8] = UB;
    }

    pub fn stamp_reactive(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedReactiveStamper<'_>) {
        let multiplicity = self.multiplicity;
        let cached = &*self.canonical_reactive;
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(4),
            None,
            &[4],
            &[cached[5]],
            &[],
            &[],
            multiplicity,
        );
    }

}
