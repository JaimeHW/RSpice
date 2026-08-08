#![allow(dead_code, non_snake_case, unused_parens, unused_variables)]

use super::state::Instance;
use rspice_veriloga_runtime::GeneratedEvalContext;
pub use rspice_veriloga_runtime::{GeneratedNoiseDescriptor, GeneratedNoiseEndpoint, GeneratedNoiseEvaluation, GeneratedNoiseEvaluationError, GeneratedNoiseEvaluationRef, GeneratedNoiseKind, GeneratedNoiseVisitor};

use rspice_veriloga_runtime::{Lanes, rspice_limexp};
pub static NOISE_SOURCES: [GeneratedNoiseDescriptor; 16] = [
    GeneratedNoiseDescriptor { mechanism: "WHITE_GI_GDI_RGD", label: Some("Rgd"), kind: GeneratedNoiseKind::White, equation: 20, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(4), name: "gi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(7), name: "gdi", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_G_GI_RG", label: Some("Rg"), kind: GeneratedNoiseKind::White, equation: 26, is_current: false, branch_ordinal: Some(7), pos: GeneratedNoiseEndpoint { local_node: Some(1), name: "g", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(4), name: "gi", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_SI_SII_RS", label: Some("Rs"), kind: GeneratedNoiseKind::White, equation: 30, is_current: false, branch_ordinal: Some(11), pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "si", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(6), name: "sii", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_DI_D_RD", label: Some("Rd"), kind: GeneratedNoiseKind::White, equation: 35, is_current: false, branch_ordinal: Some(16), pos: GeneratedNoiseEndpoint { local_node: Some(3), name: "di", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(0), name: "d", is_internal: false }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_DI_SI_IDS_NOISE", label: Some("Ids noise"), kind: GeneratedNoiseKind::White, equation: 38, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(3), name: "di", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(5), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_DI_SI_IDS_FLICKER", label: Some("Ids flicker"), kind: GeneratedNoiseKind::Flicker, equation: 39, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(3), name: "di", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(5), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_IA_GND_CORRELATED_NOISE", label: Some("correlated noise"), kind: GeneratedNoiseKind::White, equation: 40, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(14), name: "ia", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: None, name: "0", is_internal: false }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_IB_GND_CORRELATED_NOISE", label: Some("correlated noise"), kind: GeneratedNoiseKind::White, equation: 42, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(15), name: "ib", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: None, name: "0", is_internal: false }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_DI_SI_DRAIN", label: Some("drain"), kind: GeneratedNoiseKind::White, equation: 47, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(3), name: "di", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(5), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_GI_SI_GATE", label: Some("gate"), kind: GeneratedNoiseKind::Flicker, equation: 48, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(4), name: "gi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(5), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_DI_SI_FLICKER", label: Some("flicker"), kind: GeneratedNoiseKind::Flicker, equation: 49, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(3), name: "di", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(5), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_DI_SI_FLICKER", label: Some("flicker"), kind: GeneratedNoiseKind::Flicker, equation: 50, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(3), name: "di", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(5), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_GSI_SI_SHOT", label: Some("shot"), kind: GeneratedNoiseKind::White, equation: 53, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(8), name: "gsi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(5), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_GDI_DI_SHOT", label: Some("shot"), kind: GeneratedNoiseKind::White, equation: 54, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(7), name: "gdi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(3), name: "di", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_GSI_SI_FLICKER", label: Some("flicker"), kind: GeneratedNoiseKind::Flicker, equation: 55, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(8), name: "gsi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(5), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_GDI_DI_FLICKER", label: Some("flicker"), kind: GeneratedNoiseKind::Flicker, equation: 56, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(7), name: "gdi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(3), name: "di", is_internal: true }, table_len: 0, table_log_interp: false },
];

impl Instance {
    pub fn evaluate_noise_sources(&self, ctx: &GeneratedEvalContext<'_>, visitor: &mut dyn GeneratedNoiseVisitor) -> Result<(), GeneratedNoiseEvaluationError> {
        if !self.multiplicity.is_finite() || self.multiplicity <= 0.0 {
            return Err(GeneratedNoiseEvaluationError::InvalidMultiplicity { value: self.multiplicity });
        }
        let parameters = &self.params.values;
        let parameter_given = &*self.param_given;
        let temperature = ctx.temperature();
        let node_potentials = [ctx.node_voltage(self.nodes[0]), ctx.node_voltage(self.nodes[1]), ctx.node_voltage(self.nodes[2]), ctx.node_voltage(self.nodes[3]), ctx.node_voltage(self.nodes[4]), ctx.node_voltage(self.nodes[5]), ctx.node_voltage(self.nodes[6]), ctx.node_voltage(self.nodes[7]), ctx.node_voltage(self.nodes[8]), ctx.node_voltage(self.nodes[9]), ctx.node_voltage(self.nodes[10]), ctx.node_voltage(self.nodes[11]), ctx.node_voltage(self.nodes[12]), ctx.node_voltage(self.nodes[13]), ctx.node_voltage(self.nodes[14]), ctx.node_voltage(self.nodes[15])];
            let A = 0e0f64;
            let B = node_potentials[5];
            let D = node_potentials[3];
            let I = node_potentials[13];
            let J = if parameter_given[3] { 1.0 } else { 0.0 };
            let K = 2.7315e2f64;
            let N = if parameter_given[85] { 1.0 } else { 0.0 };
            let P = 3.0015e2f64;
            let Q = parameters[1];
            let S = node_potentials[11];
            let Z = parameters[57];
            let AC = 1e0f64;
            let AD = parameters[8];
            let AE = parameters[59];
            let AG = parameters[11];
            let AH = parameters[60];
            let AJ = parameters[20];
            let AK = parameters[63];
            let AM = parameters[25];
            let AO = parameters[9];
            let AP = parameters[68];
            let AR = parameters[41];
            let AT = parameters[21];
            let AU = parameters[70];
            let AX = 5e-1f64;
            let AZ = parameters[39];
            let BA = parameters[19];
            let BJ = parameters[10];
            let BK = parameters[15];
            let BM = parameters[22];
            let BT = parameters[12];
            let BV = parameters[13];
            let CB = parameters[14];
            let CE = parameters[4];
            let CI = parameters[16];
            let CR = parameters[17];
            let DA = 2e0f64;
            let ER = parameters[43];
            let ES = parameters[44];
            let EV = parameters[46];
            let FI = parameters[5];
            let FV = parameters[38];
            let GD = parameters[6];
            let GL = parameters[47];
            let GN = parameters[0];
            let GO = 4e0f64;
            let GP = 1.3806503e-23f64;
            let GS = parameters[42];
            let HC = parameters[7];
            let HH = parameters[83];
            let HJ = parameters[72];
            let HM = parameters[71];
            let HR = parameters[75];
            let HT = parameters[76];
            let HV = parameters[77];
            let LR = 1e0f64;
            let LS = 1e0f64;
            let LT = 1e0f64;
            let LU = 1e0f64;
            let LV = 1e0f64;
            let LW = 1e0f64;
            let MJ = -1e0f64;
            let MM = 0e0f64;
            let MN = 0e0f64;
            let MO = 2e0f64;
            let NN = Lanes([0e0f64; 5]);
            let C = node_potentials[8] - B;
            let MH = Lanes([0.0, LS]) - Lanes([LT, 0.0]);
            let E = node_potentials[4] - D;
            let MI = Lanes([0.0, LU]) - Lanes([LV, 0.0]);
            let F = -E;
            let MK = MI * MJ;
            let G = D - B;
            let ML = Lanes([LV, 0.0]) - Lanes([0.0, LT]);
            let H = node_potentials[7] - D;
            let R = if J != 0.0 {
                let L = parameters[3] + K;
                L
            } else {
                let M = temperature + parameters[2];
                M
            };
            let W = if N != 0.0 {
                let O = parameters[85] + K;
                O
            } else {
                P
            };
            let U;
            let LX;
            if Q != 0.0 {
                let MP = LW * ((MO * (if S >= MN { 1.0 } else { 0.0 })) - LR);
                let T = R + (S.abs());
                U = T;
                LX = MP;
            } else {
                U = R;
                LX = MM;
            }
            let V = U * 8.617333262e-5f64;
            let X = U - W;
            let Y = X.abs();
            let MQ = LX * ((MO * (if X >= MN { 1.0 } else { 0.0 })) - LR);
            let AA = if Z > A { 1.0 } else { 0.0 };
            let AB = if (if Y > A { 1.0 } else { 0.0 }) != 0.0 || AA != 0.0 { 1.0 } else { 0.0 };
            let BD;
            let BI;
            let BO;
            let CG;
            let CJ;
            let FL;
            let GG;
            let LY;
            let LZ;
            let MA;
            let MB;
            let MC;
            if AB != 0.0 {
                let AF = AD * (AC + (AE * Y));
                let MR = (MQ * AE) * AD;
                let AI = AG * (AC + (AH * Y));
                let MS = (MQ * AH) * AG;
                let AL = AJ * (AC + (AK * Y));
                let MT = (MQ * AK) * AJ;
                let AN = AM * (AC + (parameters[61] * Y));
                let MU = MQ * AP;
                let AQ = AO + (AP * Y);
                let AS = AR + (parameters[69] * Y);
                let MV = MQ * AU;
                let AV = AT + (AU * Y);
                BD = AI;
                BI = AQ;
                BO = AV;
                CG = AF;
                CJ = AL;
                FL = AS;
                GG = AN;
                LY = MS;
                LZ = MU;
                MA = MV;
                MB = MR;
                MC = MT;
            } else {
                BD = AG;
                BI = AO;
                BO = AT;
                CG = AD;
                CJ = AJ;
                FL = AR;
                GG = AM;
                LY = MM;
                LZ = MM;
                MA = MM;
                MB = MM;
                MC = MM;
            }
            let AW = if (if (if parameter_given[39] { 1.0 } else { 0.0 }) == 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if parameter_given[40] { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let FK = if AW != 0.0 {
                let AY = (AX / parameters[40]) / V;
                AY
            } else {
                AZ
            };
            let BB = BA * G;
            let BC = BB.cosh();
            let BE = BC * BC;
            let MW = ((ML * BA) * (BB.sinh())) * BC;
            let BF = parameters[18] / BE;
            let BG = AC + BF;
            let BH = BD * BG;
            let MX = ((((MW + MW) * BF) * MJ) / BE) * BD;
            let MY = Lanes([0.0, 0.0, (LY * BG)]) + Lanes([MX[0], MX[1], 0.0]);
            let BL = (BK * G).tanh();
            let MZ = ((ML * BK) * (LR - (BL * BL))) * BJ;
            let NA = Lanes([0.0, 0.0, LZ]) + Lanes([MZ[0], MZ[1], 0.0]);
            let BN = BM * (F - AT);
            let BP = F - BO;
            let NB = Lanes([MK[0], MK[1], 0.0]) - Lanes([0.0, 0.0, MA]);
            let NC = (MK * BM) * BP;
            let ND = Lanes([NC[0], NC[1], 0.0]) + (NB * BN);
            let BQ = ((BI - BJ) + (BJ * BL)) - (BN * BP);
            let NE = Lanes([NA[0], 0.0, NA[1], NA[2]]) - Lanes([ND[0], ND[1], 0.0, ND[2]]);
            let BR = C - BQ;
            let NF = Lanes([0.0, 0.0, MH[0], MH[1], 0.0]) - Lanes([NE[0], NE[1], NE[2], 0.0, NE[3]]);
            let BS = BR * BR;
            let NG = NF * BR;
            let NH = NG + NG;
            let NI = MY * BR;
            let BU = BT * BS;
            let NJ = NH * BT;
            let BW = BV * BR;
            let BX = ((BH * BR) + BU) + (BW * BS);
            let BY = BX.tanh();
            let NK = (((Lanes([NI[0], 0.0, NI[1], 0.0, NI[2]]) + (NF * BH)) + NJ) + (((NF * BV) * BS) + (NH * BW))) * (LR - (BY * BY));
            let BZ = AC + BY;
            let CA = AC + ((AX * ((rspice_limexp(BX)) - (rspice_limexp((-BX))))).tanh());
            let CC = CB + (BK * BZ);
            let NL = ML * CC;
            let CD = (CC * G).tanh();
            let NM = (((NK * BK) * G) + Lanes([NL[0], 0.0, NL[1], 0.0, 0.0])) * (LR - (CD * CD));
            let CF = if CE == A { 1.0 } else { 0.0 };
            let EX;
            let MD;
            if CF != 0.0 {
                let CH = CG * BZ;
                let OW = ML * CI;
                let CK = rspice_limexp(BP);
                let OX = Lanes([0.0, 0.0, (MC * CK)]) + ((NB * CK) * CJ);
                let OY = (Lanes([OW[0], 0.0, OW[1], 0.0]) + Lanes([OX[0], OX[1], 0.0, OX[2]])) * (CH * CD);
                let OZ = ((((Lanes([0.0, 0.0, 0.0, 0.0, (MB * BZ)]) + (NK * CG)) * CD) + (NM * CH)) * ((AC + (CI * G)) + (CJ * CK))) + Lanes([OY[0], OY[1], OY[2], 0.0, OY[3]]);
                EX = CA;
                MD = OZ;
            } else {
                let CL = if CE == AC { 1.0 } else { 0.0 };
                let EY;
                let ME;
                if CL != 0.0 {
                    let CM = E - BQ;
                    let OL = Lanes([MI[0], MI[1], 0.0, 0.0]) - NE;
                    let CN = CM * CM;
                    let OM = OL * CM;
                    let ON = OM + OM;
                    let OO = MY * CM;
                    let CO = (((BH * CM) + (BT * CN)) + (BV * (CN * CM))).tanh();
                    let OP = (((Lanes([OO[0], 0.0, OO[1], OO[2]]) + (OL * BH)) + (ON * BT)) + (((ON * CM) + (OL * CN)) * BV)) * (LR - (CO * CO));
                    let CP = AC + CO;
                    let CQ = CB + (BK * CP);
                    let CS = CI + (CR * BZ);
                    let CT = CG * BZ;
                    let CU = AC + CD;
                    let OQ = ML * CS;
                    let CV = rspice_limexp((G - BO));
                    let OR = Lanes([0.0, 0.0, (MC * CV)]) + (((Lanes([ML[0], ML[1], 0.0]) - Lanes([0.0, 0.0, MA])) * CV) * CJ);
                    let CW = CI + (CR * CP);
                    let OS = ML * CQ;
                    let CX = (CQ * G).tanh();
                    let CY = CG * CP;
                    let CZ = AC - CX;
                    let OT = ML * CW;
                    let OU = ((((Lanes([0.0, 0.0, 0.0, (MB * CP)]) + (OP * CG)) * CZ) + ((((((OP * BK) * G) + Lanes([OS[0], 0.0, OS[1], 0.0])) * (LR - (CX * CX))) * MJ) * CY)) * (AC - (CW * G))) + (((((OP * CR) * G) + Lanes([OT[0], 0.0, OT[1], 0.0])) * MJ) * (CY * CZ));
                    let OV = ((((((Lanes([0.0, 0.0, 0.0, 0.0, (MB * BZ)]) + (NK * CG)) * CU) + (NM * CT)) * ((AC + (CS * G)) + (CJ * CV))) + (((((NK * CR) * G) + Lanes([OQ[0], 0.0, OQ[1], 0.0, 0.0])) + Lanes([OR[0], 0.0, OR[1], 0.0, OR[2]])) * (CT * CU))) - Lanes([OU[0], OU[1], OU[2], 0.0, OU[3]])) * AX;
                    EY = CA;
                    ME = OV;
                } else {
                    let DB = if CE == DA { 1.0 } else { 0.0 };
                    let EZ;
                    let MF;
                    if DB != 0.0 {
                        let DC = BV * BS;
                        let DD = (BR + BU) + (DC * BR);
                        let DE = BH * DD;
                        let OE = MY * DD;
                        let OF = Lanes([OE[0], 0.0, OE[1], 0.0, OE[2]]) + (((NF + NJ) + (((NH * BV) * BR) + (NF * DC))) * BH);
                        let DF = rspice_limexp(DE);
                        let DG = rspice_limexp((-DE));
                        let DH = (AX * (DF - DG)).tanh();
                        let OG = (((OF * DF) - ((OF * MJ) * DG)) * AX) * (LR - (DH * DH));
                        let DI = AC + DH;
                        let DJ = CB + (BK * DI);
                        let OH = ML * DJ;
                        let DK = (DJ * G).tanh();
                        let DL = CI + (CR * DI);
                        let DM = CG * DI;
                        let OI = ML * DL;
                        let DN = rspice_limexp(BP);
                        let OJ = Lanes([0.0, 0.0, (MC * DN)]) + ((NB * DN) * CJ);
                        let OK = ((((Lanes([0.0, 0.0, 0.0, 0.0, (MB * DI)]) + (OG * CG)) * DK) + (((((OG * BK) * G) + Lanes([OH[0], 0.0, OH[1], 0.0, 0.0])) * (LR - (DK * DK))) * DM)) * ((AC + (DL * G)) + (CJ * DN))) + (((((OG * CR) * G) + Lanes([OI[0], 0.0, OI[1], 0.0, 0.0])) + Lanes([OJ[0], OJ[1], 0.0, 0.0, OJ[2]])) * (DM * DK));
                        EZ = DI;
                        MF = OK;
                    } else {
                        let DO = if CE == 3e0f64 { 1.0 } else { 0.0 };
                        let FA;
                        let MG;
                        if DO != 0.0 {
                            let DP = BV * BS;
                            let DQ = (BR + BU) + (DP * BR);
                            let DR = BH * DQ;
                            let NO = MY * DQ;
                            let NP = Lanes([NO[0], 0.0, NO[1], 0.0, NO[2]]) + (((NF + NJ) + (((NH * BV) * BR) + (NF * DP))) * BH);
                            let DS = E - BQ;
                            let NQ = Lanes([MI[0], MI[1], 0.0, 0.0]) - NE;
                            let DT = DS * DS;
                            let NR = NQ * DS;
                            let NS = NR + NR;
                            let DU = BV * DS;
                            let DV = (DS + (BT * DT)) + (DU * DT);
                            let DW = BH * DV;
                            let NT = MY * DV;
                            let NU = Lanes([NT[0], 0.0, NT[1], NT[2]]) + (((NQ + (NS * BT)) + (((NQ * BV) * DT) + (NS * DU))) * BH);
                            let DX = rspice_limexp(DR);
                            let DY = rspice_limexp((-DR));
                            let DZ = (AX * (DX - DY)).tanh();
                            let NV = (((NP * DX) - ((NP * MJ) * DY)) * AX) * (LR - (DZ * DZ));
                            let EA = AC + DZ;
                            let EB = rspice_limexp(DW);
                            let EC = rspice_limexp((-DW));
                            let ED = (AX * (EB - EC)).tanh();
                            let NW = (((NU * EB) - ((NU * MJ) * EC)) * AX) * (LR - (ED * ED));
                            let EE = AC + ED;
                            let EF = CB + (BK * EA);
                            let EG = CB + (BK * EE);
                            let NX = ML * EF;
                            let EH = (EF * G).tanh();
                            let NY = ML * EG;
                            let EI = (EG * G).tanh();
                            let EJ = CI + (CR * EE);
                            let EK = CI + (CR * EA);
                            let EL = CG * EA;
                            let EM = AC + EH;
                            let NZ = ML * EK;
                            let EN = rspice_limexp((G - BO));
                            let OA = Lanes([0.0, 0.0, (MC * EN)]) + (((Lanes([ML[0], ML[1], 0.0]) - Lanes([0.0, 0.0, MA])) * EN) * CJ);
                            let EO = CG * EE;
                            let EP = AC - EI;
                            let OB = ML * EJ;
                            let OC = ((((Lanes([0.0, 0.0, 0.0, (MB * EE)]) + (NW * CG)) * EP) + ((((((NW * BK) * G) + Lanes([NY[0], 0.0, NY[1], 0.0])) * (LR - (EI * EI))) * MJ) * EO)) * (AC - (EJ * G))) + (((((NW * CR) * G) + Lanes([OB[0], 0.0, OB[1], 0.0])) * MJ) * (EO * EP));
                            let OD = ((((((Lanes([0.0, 0.0, 0.0, 0.0, (MB * EA)]) + (NV * CG)) * EM) + (((((NV * BK) * G) + Lanes([NX[0], 0.0, NX[1], 0.0, 0.0])) * (LR - (EH * EH))) * EL)) * ((AC + (EK * G)) + (CJ * EN))) + (((((NV * CR) * G) + Lanes([NZ[0], 0.0, NZ[1], 0.0, 0.0])) + Lanes([OA[0], 0.0, OA[1], 0.0, OA[2]])) * (EL * EM))) - Lanes([OC[0], OC[1], OC[2], 0.0, OC[3]])) * AX;
                            FA = EA;
                            MG = OD;
                        } else {
                            FA = CA;
                            MG = NN;
                        }
                        EZ = FA;
                        MF = MG;
                    }
                    EY = EZ;
                    ME = MF;
                }
                EX = EY;
                MD = ME;
            }
            let EQ = if CF != 0.0 || (if CE == AC { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let FF;
            let FH;
            if EQ != 0.0 {
                let ET = ES * BZ;
                let EU = ER + ET;
                let EW = EV + ET;
                FF = EW;
                FH = EU;
            } else {
                let FB = ES * EX;
                let FC = ER + FB;
                let FD = EV + FB;
                FF = FD;
                FH = FC;
            }
            let FE = if Y != 0.0 || AA != 0.0 { 1.0 } else { 0.0 };
            let GX = if FE != 0.0 {
                let FG = FF * (AC + (parameters[66] * Y));
                FG
            } else {
                FF
            };
            let FJ = if FI == A { 1.0 } else { 0.0 };
            let FW;
            let FY;
            let GA;
            if FJ != 0.0 {
                let FM = rspice_limexp((FK * ((-1e0f64 * FL).tanh())));
                let FN = C - FL;
                let FO = H - FL;
                FW = FN;
                FY = FM;
                GA = FO;
            } else {
                let FP = rspice_limexp(((-FK) * FL));
                let FQ = if FI == AC { 1.0 } else { 0.0 };
                let FX;
                let GB;
                if FQ != 0.0 {
                    let FR = (C - FL).tanh();
                    let FS = (H - FL).tanh();
                    FX = FR;
                    GB = FS;
                } else {
                    let FT = C - FL;
                    let FU = H - FL;
                    FX = FT;
                    GB = FU;
                }
                FW = FX;
                FY = FP;
                GA = GB;
            }
            let FZ = FV * ((rspice_limexp((FK * FW))) - FY);
            let GC = FV * ((rspice_limexp((FK * GA))) - FY);
            let GE = if GD == A { 1.0 } else { 0.0 };
            if GE != 0.0 {
            } else {
                let GF = if GD == AC { 1.0 } else { 0.0 };
                if GF != 0.0 {
                } else {
                    let GH = if GD == DA { 1.0 } else { 0.0 };
                    if GH != 0.0 {
                    } else {
                    }
                }
            }
            let GI = if GD == DA { 1.0 } else { 0.0 };
            if GI != 0.0 {
            } else {
            }
            let GJ = if parameters[53] > A { 1.0 } else { 0.0 };
            if GJ != 0.0 {
            } else {
            }
            let GK = if parameters[55] > A { 1.0 } else { 0.0 };
            if GK != 0.0 {
            } else {
            }
            let GM = if GL > A { 1.0 } else { 0.0 };
            let IF;
            let IH;
            if GM != 0.0 {
                let IG;
                let II;
                if GN != 0.0 {
                    let GQ = (5.5226012e-23f64 * U) * GL;
                    IG = AC;
                    II = GQ;
                } else {
                    IG = A;
                    II = A;
                }
                IF = IG;
                IH = II;
            } else {
                IF = A;
                IH = A;
            }
            let GR = if parameters[45] > A { 1.0 } else { 0.0 };
            if GR != 0.0 {
            } else {
            }
            let GT = if GS > A { 1.0 } else { 0.0 };
            let IJ;
            let IL;
            if GT != 0.0 {
                let IK;
                let IM;
                if GN != 0.0 {
                    let GU = (5.5226012e-23f64 * U) * GS;
                    IK = AC;
                    IM = GU;
                } else {
                    IK = A;
                    IM = A;
                }
                IJ = IK;
                IL = IM;
            } else {
                let GV = if parameters[50] > A { 1.0 } else { 0.0 };
                if GV != 0.0 {
                } else {
                }
                IJ = A;
                IL = A;
            }
            let GW = if EV > A { 1.0 } else { 0.0 };
            let IN;
            let IP;
            if GW != 0.0 {
                let IO;
                let IQ;
                if GN != 0.0 {
                    let GY = (5.5226012e-23f64 * U) * GX;
                    IO = AC;
                    IQ = GY;
                } else {
                    IO = A;
                    IQ = A;
                }
                IN = IO;
                IP = IQ;
            } else {
                IN = A;
                IP = A;
            }
            let GZ = if (if ER > A { 1.0 } else { 0.0 }) != 0.0 || (if ES > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let IR;
            let IT;
            if GZ != 0.0 {
                let IS;
                let IU;
                if GN != 0.0 {
                    let HA = (5.5226012e-23f64 * U) * FH;
                    IS = AC;
                    IU = HA;
                } else {
                    IS = A;
                    IU = A;
                }
                IR = IS;
                IT = IU;
            } else {
                let HB = if parameters[48] > A { 1.0 } else { 0.0 };
                if HB != 0.0 {
                } else {
                }
                IR = A;
                IT = A;
            }
            let HD = if HC == A { 1.0 } else { 0.0 };
            let IV;
            let IX;
            let IZ;
            let JB;
            let JD;
            let JF;
            let JI;
            let JL;
            let JO;
            let JR;
            let JU;
            let JX;
            let KA;
            let KD;
            let KG;
            let KJ;
            let KM;
            let KP;
            let KT;
            let KX;
            if HD != 0.0 {
                let HE = (I.abs()) + (GC.abs());
                let HF = (((parameters[84] * GO) * GP) * U) * (((((((parameters[78] + K) * (AC + (((parameters[80] * BZ) * (CD.abs())) * (AC + (CI * G))))) / U) * HE) + ((parameters[79] * HE) * HE)).abs()).sqrt());
                let IW;
                let IY;
                let JA;
                let JC;
                let JE;
                if GN != 0.0 {
                    let HG = HF * parameters[81];
                    IW = AC;
                    IY = HF;
                    JA = AC;
                    JC = HG;
                    JE = HH;
                } else {
                    IW = A;
                    IY = A;
                    JA = A;
                    JC = A;
                    JE = A;
                }
                IV = IW;
                IX = IY;
                IZ = JA;
                JB = JC;
                JD = JE;
                JF = A;
                JI = A;
                JL = A;
                JO = A;
                JR = A;
                JU = A;
                JX = A;
                KA = A;
                KD = A;
                KG = A;
                KJ = A;
                KM = A;
                KP = A;
                KT = A;
                KX = A;
            } else {
                let HI = if HC == AC { 1.0 } else { 0.0 };
                let JG;
                let JJ;
                let JM;
                let JP;
                let JS;
                let JV;
                let JY;
                let KB;
                let KE;
                let KH;
                let KK;
                let KN;
                let KQ;
                let KU;
                let KY;
                if HI != 0.0 {
                    let JH;
                    let JK;
                    let JN;
                    let JQ;
                    let JT;
                    let JW;
                    let JZ;
                    let KC;
                    let KF;
                    let KI;
                    let KL;
                    let KO;
                    let KR;
                    let KV;
                    let KZ;
                    if GN != 0.0 {
                        let PA = MD[1];
                        let HK = ((5.5226012e-23f64 * U) * PA) * HJ;
                        let HL = if PA > A { 1.0 } else { 0.0 };
                        let HP = if HL != 0.0 {
                            let HN = (((((GG * GG) * GO) * GP) * U) * HM) / PA;
                            HN
                        } else {
                            A
                        };
                        let HO = (((5.5226012e-23f64 * U) * parameters[73]) * GG) * ((HJ * HM).sqrt());
                        let HQ = (((5.5226012e-23f64 * U) * PA) * HJ) * parameters[74];
                        let HS = if HR > A { 1.0 } else { 0.0 };
                        let KS;
                        let KW;
                        let LA;
                        if HS != 0.0 {
                            let HU = HR * (I.powf(HT));
                            KS = AC;
                            KW = HU;
                            LA = HV;
                        } else {
                            KS = A;
                            KW = A;
                            LA = A;
                        }
                        JH = AC;
                        JK = HO;
                        JN = AC;
                        JQ = HO;
                        JT = AC;
                        JW = HK;
                        JZ = AC;
                        KC = HP;
                        KF = DA;
                        KI = AC;
                        KL = HQ;
                        KO = AC;
                        KR = KS;
                        KV = KW;
                        KZ = LA;
                    } else {
                        JH = A;
                        JK = A;
                        JN = A;
                        JQ = A;
                        JT = A;
                        JW = A;
                        JZ = A;
                        KC = A;
                        KF = A;
                        KI = A;
                        KL = A;
                        KO = A;
                        KR = A;
                        KV = A;
                        KZ = A;
                    }
                    JG = JH;
                    JJ = JK;
                    JM = JN;
                    JP = JQ;
                    JS = JT;
                    JV = JW;
                    JY = JZ;
                    KB = KC;
                    KE = KF;
                    KH = KI;
                    KK = KL;
                    KN = KO;
                    KQ = KR;
                    KU = KV;
                    KY = KZ;
                } else {
                    let HW = if HC == DA { 1.0 } else { 0.0 };
                    if HW != 0.0 {
                    } else {
                    }
                    JG = A;
                    JJ = A;
                    JM = A;
                    JP = A;
                    JS = A;
                    JV = A;
                    JY = A;
                    KB = A;
                    KE = A;
                    KH = A;
                    KK = A;
                    KN = A;
                    KQ = A;
                    KU = A;
                    KY = A;
                }
                IV = A;
                IX = A;
                IZ = A;
                JB = A;
                JD = A;
                JF = JG;
                JI = JJ;
                JL = JM;
                JO = JP;
                JR = JS;
                JU = JV;
                JX = JY;
                KA = KB;
                KD = KE;
                KG = KH;
                KJ = KK;
                KM = KN;
                KP = KQ;
                KT = KU;
                KX = KY;
            }
            let LB;
            let LC;
            let LD;
            let LE;
            let LF;
            let LH;
            let LJ;
            let LL;
            let LN;
            let LP;
            if GN != 0.0 {
                let HX = FZ.abs();
                let HY = 3.204352924e-19f64 * HX;
                let HZ = GC.abs();
                let IA = 3.204352924e-19f64 * HZ;
                let IB = if HR > A { 1.0 } else { 0.0 };
                let LG;
                let LI;
                let LK;
                let LM;
                let LO;
                let LQ;
                if IB != 0.0 {
                    let IC = HR * (HX.powf(HT));
                    let ID = HR * (HZ.powf(HT));
                    LG = AC;
                    LI = IC;
                    LK = HV;
                    LM = AC;
                    LO = ID;
                    LQ = HV;
                } else {
                    LG = A;
                    LI = A;
                    LK = A;
                    LM = A;
                    LO = A;
                    LQ = A;
                }
                LB = AC;
                LC = HY;
                LD = AC;
                LE = IA;
                LF = LG;
                LH = LI;
                LJ = LK;
                LL = LM;
                LN = LO;
                LP = LQ;
            } else {
                LB = A;
                LC = A;
                LD = A;
                LE = A;
                LF = A;
                LH = A;
                LJ = A;
                LL = A;
                LN = A;
                LP = A;
            }
            let IE = if Q != 0.0 && Z != 0.0 { 1.0 } else { 0.0 };
            if IE != 0.0 {
            } else {
            }
        if IF == 0.0 {
            if !visitor.visit(0, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = IH;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 0, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 0, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(0, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if IJ == 0.0 {
            if !visitor.visit(1, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = IL;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd / self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(1, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if IN == 0.0 {
            if !visitor.visit(2, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = IP;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 2, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd / self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 2, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(2, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if IR == 0.0 {
            if !visitor.visit(3, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = IT;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 3, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd / self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 3, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(3, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if IV == 0.0 {
            if !visitor.visit(4, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = IX;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 4, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 4, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(4, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if IZ == 0.0 {
            if !visitor.visit(5, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = JB;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 5, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = Some(JD);
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 5, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 5, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(5, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if JF == 0.0 {
            if !visitor.visit(6, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = JI;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 6, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 6, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(6, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if JL == 0.0 {
            if !visitor.visit(7, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = JO;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 7, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 7, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(7, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if JR == 0.0 {
            if !visitor.visit(8, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = JU;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 8, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 8, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(8, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if JX == 0.0 {
            if !visitor.visit(9, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = KA;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 9, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = Some(KD);
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 9, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 9, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(9, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if KG == 0.0 {
            if !visitor.visit(10, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = KJ;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 10, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = Some(KM);
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 10, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 10, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(10, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if KP == 0.0 {
            if !visitor.visit(11, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = KT;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 11, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = Some(KX);
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 11, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 11, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(11, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if LB == 0.0 {
            if !visitor.visit(12, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = LC;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 12, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 12, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(12, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if LD == 0.0 {
            if !visitor.visit(13, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = LE;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 13, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 13, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(13, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if LF == 0.0 {
            if !visitor.visit(14, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = LH;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 14, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = Some(LJ);
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 14, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 14, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(14, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if LL == 0.0 {
            if !visitor.visit(15, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = LN;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 15, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = Some(LP);
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 15, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 15, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(15, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        Ok(())
    }
}
