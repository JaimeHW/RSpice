#![allow(dead_code, non_snake_case, unused_parens, unused_variables)]

use super::state::Instance;
use rspice_veriloga_runtime::GeneratedEvalContext;
pub use rspice_veriloga_runtime::{GeneratedNoiseDescriptor, GeneratedNoiseEndpoint, GeneratedNoiseEvaluation, GeneratedNoiseEvaluationError, GeneratedNoiseEvaluationRef, GeneratedNoiseKind, GeneratedNoiseVisitor};

use rspice_veriloga_runtime::{Lanes, rspice_limexp};
pub static NOISE_SOURCES: [GeneratedNoiseDescriptor; 16] = [
    GeneratedNoiseDescriptor { mechanism: "WHITE_GGI_GDI_RGD", label: Some("Rgd"), kind: GeneratedNoiseKind::White, equation: 25, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(13), name: "ggi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(10), name: "gdi", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_GI_GGI_RG", label: Some("Rg"), kind: GeneratedNoiseKind::White, equation: 29, is_current: false, branch_ordinal: Some(8), pos: GeneratedNoiseEndpoint { local_node: Some(7), name: "gi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(13), name: "ggi", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_SI_SII_RS", label: Some("Rs"), kind: GeneratedNoiseKind::White, equation: 33, is_current: false, branch_ordinal: Some(12), pos: GeneratedNoiseEndpoint { local_node: Some(8), name: "si", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(9), name: "sii", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_DI_DII_RD", label: Some("Rd"), kind: GeneratedNoiseKind::White, equation: 37, is_current: false, branch_ordinal: Some(16), pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "di", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(6), name: "dii", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_DI_SI_IDS_NOISE", label: Some("Ids noise"), kind: GeneratedNoiseKind::White, equation: 43, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "di", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(8), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_DI_SI_IDS_FLICKER", label: Some("Ids flicker"), kind: GeneratedNoiseKind::Flicker, equation: 44, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "di", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(8), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_IA_GND_CORRELATED_NOISE", label: Some("correlated noise"), kind: GeneratedNoiseKind::White, equation: 45, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(17), name: "ia", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: None, name: "0", is_internal: false }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_IB_GND_CORRELATED_NOISE", label: Some("correlated noise"), kind: GeneratedNoiseKind::White, equation: 47, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(18), name: "ib", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: None, name: "0", is_internal: false }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_DI_SI_DRAIN", label: Some("drain"), kind: GeneratedNoiseKind::White, equation: 52, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "di", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(8), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_GI_SI_GATE", label: Some("gate"), kind: GeneratedNoiseKind::Flicker, equation: 53, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(7), name: "gi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(8), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_DI_SI_FLICKER", label: Some("flicker"), kind: GeneratedNoiseKind::Flicker, equation: 54, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "di", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(8), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_DI_SI_FLICKER", label: Some("flicker"), kind: GeneratedNoiseKind::Flicker, equation: 55, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "di", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(8), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_GSI_SI_SHOT", label: Some("shot"), kind: GeneratedNoiseKind::White, equation: 58, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(11), name: "gsi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(8), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_GDI_DI_SHOT", label: Some("shot"), kind: GeneratedNoiseKind::White, equation: 59, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(10), name: "gdi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(5), name: "di", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_GSI_SI_FLICKER", label: Some("flicker"), kind: GeneratedNoiseKind::Flicker, equation: 60, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(11), name: "gsi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(8), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_GDI_DI_FLICKER", label: Some("flicker"), kind: GeneratedNoiseKind::Flicker, equation: 61, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(10), name: "gdi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(5), name: "di", is_internal: true }, table_len: 0, table_log_interp: false },
];

impl Instance {
    pub fn evaluate_noise_sources(&self, ctx: &GeneratedEvalContext<'_>, visitor: &mut dyn GeneratedNoiseVisitor) -> Result<(), GeneratedNoiseEvaluationError> {
        if !self.multiplicity.is_finite() || self.multiplicity <= 0.0 {
            return Err(GeneratedNoiseEvaluationError::InvalidMultiplicity { value: self.multiplicity });
        }
        let parameters = &self.params.values;
        let parameter_given = &*self.param_given;
        let temperature = ctx.temperature();
        let node_potentials = [ctx.node_voltage(self.nodes[0]), ctx.node_voltage(self.nodes[1]), ctx.node_voltage(self.nodes[2]), ctx.node_voltage(self.nodes[3]), ctx.node_voltage(self.nodes[4]), ctx.node_voltage(self.nodes[5]), ctx.node_voltage(self.nodes[6]), ctx.node_voltage(self.nodes[7]), ctx.node_voltage(self.nodes[8]), ctx.node_voltage(self.nodes[9]), ctx.node_voltage(self.nodes[10]), ctx.node_voltage(self.nodes[11]), ctx.node_voltage(self.nodes[12]), ctx.node_voltage(self.nodes[13]), ctx.node_voltage(self.nodes[14]), ctx.node_voltage(self.nodes[15]), ctx.node_voltage(self.nodes[16]), ctx.node_voltage(self.nodes[17]), ctx.node_voltage(self.nodes[18])];
            let A = 0e0f64;
            let B = node_potentials[8];
            let D = node_potentials[5];
            let J = node_potentials[16];
            let K = if parameter_given[3] { 1.0 } else { 0.0 };
            let L = 2.7315e2f64;
            let O = if parameter_given[100] { 1.0 } else { 0.0 };
            let Q = 3.0015e2f64;
            let R = parameters[1];
            let T = node_potentials[3];
            let AB = 1e0f64;
            let AD = parameters[8];
            let AE = parameters[68];
            let AG = parameters[20];
            let AH = parameters[80];
            let AJ = parameters[26];
            let AL = parameters[9];
            let AM = parameters[78];
            let AO = parameters[45];
            let AQ = parameters[21];
            let AR = parameters[81];
            let AT = parameters[4];
            let AU = 4e0f64;
            let AV = parameters[6];
            let AY = 5e-1f64;
            let BA = parameters[43];
            let BB = parameters[19];
            let BE = parameters[64];
            let BF = parameters[11];
            let BJ = parameters[69];
            let BN = parameters[13];
            let BO = parameters[70];
            let BR = parameters[10];
            let BS = parameters[15];
            let BU = parameters[22];
            let CD = parameters[12];
            let CN = parameters[14];
            let CT = parameters[16];
            let DD = parameters[17];
            let DH = parameters[23];
            let DN = 2e0f64;
            let EB = 3e0f64;
            let FK = parameters[65];
            let FP = parameters[47];
            let FQ = parameters[48];
            let FT = parameters[50];
            let GH = parameters[5];
            let GN = parameters[83];
            let GQ = parameters[84];
            let GT = parameters[85];
            let HG = parameters[42];
            let IB = parameters[51];
            let ID = parameters[0];
            let IE = 1.3806503e-23f64;
            let IH = parameters[46];
            let IP = parameters[7];
            let IU = parameters[98];
            let IW = parameters[87];
            let IZ = parameters[86];
            let JE = parameters[90];
            let JG = parameters[91];
            let JI = parameters[92];
            let NC = 1e0f64;
            let ND = 1e0f64;
            let NE = 1e0f64;
            let NF = 1e0f64;
            let NG = 1e0f64;
            let NH = 1e0f64;
            let NI = 1e0f64;
            let NV = -1e0f64;
            let NZ = 0e0f64;
            let OA = 0e0f64;
            let OB = 2e0f64;
            let PF = Lanes([0e0f64; 6]);
            let C = node_potentials[12] - B;
            let NT = Lanes([0.0, ND]) - Lanes([NE, 0.0]);
            let E = node_potentials[10] - D;
            let NU = Lanes([0.0, NF]) - Lanes([NG, 0.0]);
            let F = -E;
            let NW = NU * NV;
            let G = D - B;
            let NX = Lanes([NG, 0.0]) - Lanes([0.0, NE]);
            let H = node_potentials[11] - B;
            let I = node_potentials[4] - B;
            let NY = Lanes([NH, 0.0]) - Lanes([0.0, NE]);
            let S = if K != 0.0 {
                let M = parameters[3] + L;
                M
            } else {
                let N = temperature + parameters[2];
                N
            };
            let X = if O != 0.0 {
                let P = parameters[100] + L;
                P
            } else {
                Q
            };
            let V;
            let NJ;
            if R != 0.0 {
                let OC = NI * ((OB * (if T >= OA { 1.0 } else { 0.0 })) - NC);
                let U = S + (T.abs());
                V = U;
                NJ = OC;
            } else {
                V = S;
                NJ = NZ;
            }
            let W = V * 8.617333262e-5f64;
            let Y = V - X;
            let Z = Y.abs();
            let OD = NJ * ((OB * (if Y >= OA { 1.0 } else { 0.0 })) - NC);
            let AA = if (if Z > A { 1.0 } else { 0.0 }) != 0.0 || (if parameters[66] > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let BQ;
            let BV;
            let CR;
            let CU;
            let GK;
            let HT;
            let NK;
            let NL;
            let NM;
            let NN;
            if AA != 0.0 {
                let AC = Z.abs();
                let OE = OD * ((OB * (if Z >= OA { 1.0 } else { 0.0 })) - NC);
                let AF = AD * (AB + (AE * AC));
                let OF = (OE * AE) * AD;
                let AI = AG * (AB + (AH * AC));
                let OG = (OE * AH) * AG;
                let AK = AJ * (AB + (parameters[72] * AC));
                let OH = OD * AM;
                let AN = AL + (AM * Z);
                let AP = AO + (parameters[79] * Z);
                let OI = OD * AR;
                let AS = AQ + (AR * Z);
                let AW = if (if (if AT == AB { 1.0 } else { 0.0 }) != 0.0 || (if AT == AU { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if AV == AU { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                if AW != 0.0 {
                } else {
                }
                BQ = AN;
                BV = AS;
                CR = AF;
                CU = AI;
                GK = AP;
                HT = AK;
                NK = OH;
                NL = OI;
                NM = OF;
                NN = OG;
            } else {
                BQ = AL;
                BV = AQ;
                CR = AD;
                CU = AG;
                GK = AO;
                HT = AJ;
                NK = NZ;
                NL = NZ;
                NM = NZ;
                NN = NZ;
            }
            let AX = if (if (if parameter_given[43] { 1.0 } else { 0.0 }) == 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if parameter_given[44] { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let GJ = if AX != 0.0 {
                let AZ = (AY / parameters[44]) / W;
                AZ
            } else {
                BA
            };
            let BC = BB * G;
            let BD = BC.cosh();
            let OJ = NY * BE;
            let OK = ((NX * BB) * (BC.sinh())) * BD;
            let BG = 1e-12f64 + (BD * BD);
            let BH = parameters[18] / BG;
            let BI = BF * (AB + BH);
            let BK = Z.abs();
            let OL = OD * ((OB * (if Z >= OA { 1.0 } else { 0.0 })) - NC);
            let BL = AB + (BJ * BK);
            let BM = BI * BL;
            let OM = (((((OK + OK) * BH) * NV) / BG) * BF) * BL;
            let ON = Lanes([0.0, OM[0], OM[1]]) + Lanes([((OL * BJ) * BI), 0.0, 0.0]);
            let BP = BN * (AB + (BO * BK));
            let OO = (OL * BO) * BN;
            let BT = (BS * G).tanh();
            let OP = ((NX * BS) * (NC - (BT * BT))) * BR;
            let OQ = Lanes([NK, 0.0, 0.0]) + Lanes([0.0, OP[0], OP[1]]);
            let OR = Lanes([OQ[0], 0.0, OQ[1], OQ[2]]) - Lanes([0.0, OJ[0], 0.0, OJ[1]]);
            let BW = F - BV;
            let OS = Lanes([0.0, NW[0], NW[1]]) - Lanes([NL, 0.0, 0.0]);
            let BX = BU * BW;
            let OT = ((OS * BU) * BW) + (OS * BX);
            let BY = (((BQ - BR) + (BR * BT)) - (BE * I)) - (BX * BW);
            let BZ = AB + (AM * BK);
            let CA = BY * BZ;
            let OU = ((Lanes([OR[0], OR[1], OR[2], OR[3], 0.0]) - Lanes([OT[0], 0.0, OT[1], 0.0, OT[2]])) * BZ) + Lanes([((OL * AM) * BY), 0.0, 0.0, 0.0, 0.0]);
            let CB = C - CA;
            let OV = Lanes([0.0, 0.0, 0.0, NT[0], 0.0, NT[1]]) - Lanes([OU[0], OU[1], OU[2], OU[3], OU[4], 0.0]);
            let CC = CB * CB;
            let OW = OV * CB;
            let OX = OW + OW;
            let OY = ON * CB;
            let CE = CD * CC;
            let OZ = OX * CD;
            let CF = BP * CB;
            let CG = ((BM * CB) + CE) + (CF * CC);
            let PA = ((Lanes([OY[0], 0.0, OY[1], OY[2], 0.0, 0.0]) + (OV * BM)) + OZ) + (((Lanes([(OO * CB), 0.0, 0.0, 0.0, 0.0, 0.0]) + (OV * BP)) * CC) + (OX * CF));
            let CH = CG.tanh();
            let PB = PA * (NC - (CH * CH));
            let CI = AB + CH;
            let CJ = rspice_limexp(CG);
            let CK = rspice_limexp((-CG));
            let CL = (AY * (CJ - CK)).tanh();
            let PC = (((PA * CJ) - ((PA * NV) * CK)) * AY) * (NC - (CL * CL));
            let CM = AB + CL;
            let CO = CN + (BS * CI);
            let PD = NX * CO;
            let CP = (CO * G).tanh();
            let PE = (((PB * BS) * G) + Lanes([0.0, 0.0, PD[0], PD[1], 0.0, 0.0])) * (NC - (CP * CP));
            let CQ = if AT == A { 1.0 } else { 0.0 };
            let FV;
            let NO;
            if CQ != 0.0 {
                let CS = CR * CI;
                let QV = NX * CT;
                let CV = rspice_limexp(BW);
                let QW = Lanes([(NN * CV), 0.0, 0.0]) + ((OS * CV) * CU);
                let QX = (Lanes([0.0, QV[0], QV[1], 0.0]) + Lanes([QW[0], QW[1], 0.0, QW[2]])) * (CS * CP);
                let QY = ((((Lanes([(NM * CI), 0.0, 0.0, 0.0, 0.0, 0.0]) + (PB * CR)) * CP) + (PE * CS)) * ((AB + (CT * G)) + (CU * CV))) + Lanes([QX[0], 0.0, QX[1], QX[2], QX[3], 0.0]);
                FV = CM;
                NO = QY;
            } else {
                let CW = if AT == AB { 1.0 } else { 0.0 };
                let FW;
                let NP;
                if CW != 0.0 {
                    let CX = E - CA;
                    let QK = Lanes([0.0, 0.0, NU[0], 0.0, NU[1]]) - OU;
                    let CY = CX * CX;
                    let QL = QK * CX;
                    let QM = QL + QL;
                    let CZ = CY * CX;
                    let QN = ON * CX;
                    let DA = (((BM * CX) + (CD * CY)) + (BP * CZ)).tanh();
                    let QO = (((Lanes([QN[0], 0.0, QN[1], QN[2], 0.0]) + (QK * BM)) + (QM * CD)) + (Lanes([(OO * CZ), 0.0, 0.0, 0.0, 0.0]) + (((QM * CX) + (QK * CY)) * BP))) * (NC - (DA * DA));
                    let DB = AB + DA;
                    let DC = CN + (BS * DB);
                    let DE = CT + (DD * CI);
                    let DF = CR * CI;
                    let DG = AB + CP;
                    let QP = NX * DE;
                    let DI = rspice_limexp((DH * (G - BV)));
                    let QQ = Lanes([(NN * DI), 0.0, 0.0]) + ((((Lanes([0.0, NX[0], NX[1]]) - Lanes([NL, 0.0, 0.0])) * DH) * DI) * CU);
                    let DJ = CT + (DD * DB);
                    let QR = NX * DC;
                    let DK = (DC * G).tanh();
                    let DL = CR * DB;
                    let DM = AB - DK;
                    let QS = NX * DJ;
                    let QT = ((((Lanes([(NM * DB), 0.0, 0.0, 0.0, 0.0]) + (QO * CR)) * DM) + ((((((QO * BS) * G) + Lanes([0.0, 0.0, QR[0], QR[1], 0.0])) * (NC - (DK * DK))) * NV) * DL)) * (AB - (DJ * G))) + (((((QO * DD) * G) + Lanes([0.0, 0.0, QS[0], QS[1], 0.0])) * NV) * (DL * DM));
                    let QU = ((((((Lanes([(NM * CI), 0.0, 0.0, 0.0, 0.0, 0.0]) + (PB * CR)) * DG) + (PE * DF)) * ((AB + (DE * G)) + (CU * DI))) + (((((PB * DD) * G) + Lanes([0.0, 0.0, QP[0], QP[1], 0.0, 0.0])) + Lanes([QQ[0], 0.0, QQ[1], QQ[2], 0.0, 0.0])) * (DF * DG))) - Lanes([QT[0], QT[1], QT[2], QT[3], QT[4], 0.0])) * AY;
                    FW = CM;
                    NP = QU;
                } else {
                    let DO = if AT == DN { 1.0 } else { 0.0 };
                    let FX;
                    let NQ;
                    if DO != 0.0 {
                        let DP = BP * CC;
                        let DQ = (CB + CE) + (DP * CB);
                        let DR = BM * DQ;
                        let QD = ON * DQ;
                        let QE = Lanes([QD[0], 0.0, QD[1], QD[2], 0.0, 0.0]) + (((OV + OZ) + (((Lanes([(OO * CC), 0.0, 0.0, 0.0, 0.0, 0.0]) + (OX * BP)) * CB) + (OV * DP))) * BM);
                        let DS = rspice_limexp(DR);
                        let DT = rspice_limexp((-DR));
                        let DU = (AY * (DS - DT)).tanh();
                        let QF = (((QE * DS) - ((QE * NV) * DT)) * AY) * (NC - (DU * DU));
                        let DV = AB + DU;
                        let DW = CN + (BS * DV);
                        let QG = NX * DW;
                        let DX = (DW * G).tanh();
                        let DY = CT + (DD * DV);
                        let DZ = CR * DV;
                        let QH = NX * DY;
                        let EA = rspice_limexp((DH * BW));
                        let QI = Lanes([(NN * EA), 0.0, 0.0]) + (((OS * DH) * EA) * CU);
                        let QJ = ((((Lanes([(NM * DV), 0.0, 0.0, 0.0, 0.0, 0.0]) + (QF * CR)) * DX) + (((((QF * BS) * G) + Lanes([0.0, 0.0, QG[0], QG[1], 0.0, 0.0])) * (NC - (DX * DX))) * DZ)) * ((AB + (DY * G)) + (CU * EA))) + (((((QF * DD) * G) + Lanes([0.0, 0.0, QH[0], QH[1], 0.0, 0.0])) + Lanes([QI[0], 0.0, QI[1], 0.0, QI[2], 0.0])) * (DZ * DX));
                        FX = DV;
                        NQ = QJ;
                    } else {
                        let EC = if AT == EB { 1.0 } else { 0.0 };
                        let FY;
                        let NR;
                        if EC != 0.0 {
                            let ED = BP * CC;
                            let EE = (CB + CE) + (ED * CB);
                            let EF = BM * EE;
                            let PN = ON * EE;
                            let PO = Lanes([PN[0], 0.0, PN[1], PN[2], 0.0, 0.0]) + (((OV + OZ) + (((Lanes([(OO * CC), 0.0, 0.0, 0.0, 0.0, 0.0]) + (OX * BP)) * CB) + (OV * ED))) * BM);
                            let EG = E - CA;
                            let PP = Lanes([0.0, 0.0, NU[0], 0.0, NU[1]]) - OU;
                            let EH = EG * EG;
                            let PQ = PP * EG;
                            let PR = PQ + PQ;
                            let EI = BP * EG;
                            let EJ = (EG + (CD * EH)) + (EI * EH);
                            let EK = BM * EJ;
                            let PS = ON * EJ;
                            let PT = Lanes([PS[0], 0.0, PS[1], PS[2], 0.0]) + (((PP + (PR * CD)) + (((Lanes([(OO * EG), 0.0, 0.0, 0.0, 0.0]) + (PP * BP)) * EH) + (PR * EI))) * BM);
                            let EL = rspice_limexp(EF);
                            let EM = rspice_limexp((-EF));
                            let EN = (AY * (EL - EM)).tanh();
                            let PU = (((PO * EL) - ((PO * NV) * EM)) * AY) * (NC - (EN * EN));
                            let EO = AB + EN;
                            let EP = rspice_limexp(EK);
                            let EQ = rspice_limexp((-EK));
                            let ER = (AY * (EP - EQ)).tanh();
                            let PV = (((PT * EP) - ((PT * NV) * EQ)) * AY) * (NC - (ER * ER));
                            let ES = AB + ER;
                            let ET = CN + (BS * EO);
                            let EU = CN + (BS * ES);
                            let PW = NX * ET;
                            let EV = (ET * G).tanh();
                            let PX = NX * EU;
                            let EW = (EU * G).tanh();
                            let EX = CT + (DD * ES);
                            let EY = CT + (DD * EO);
                            let EZ = CR * EO;
                            let FA = AB + EV;
                            let PY = NX * EY;
                            let FB = rspice_limexp((DH * (G - BV)));
                            let PZ = Lanes([(NN * FB), 0.0, 0.0]) + ((((Lanes([0.0, NX[0], NX[1]]) - Lanes([NL, 0.0, 0.0])) * DH) * FB) * CU);
                            let FC = CR * ES;
                            let FD = AB - EW;
                            let QA = NX * EX;
                            let QB = ((((Lanes([(NM * ES), 0.0, 0.0, 0.0, 0.0]) + (PV * CR)) * FD) + ((((((PV * BS) * G) + Lanes([0.0, 0.0, PX[0], PX[1], 0.0])) * (NC - (EW * EW))) * NV) * FC)) * (AB - (EX * G))) + (((((PV * DD) * G) + Lanes([0.0, 0.0, QA[0], QA[1], 0.0])) * NV) * (FC * FD));
                            let QC = ((((((Lanes([(NM * EO), 0.0, 0.0, 0.0, 0.0, 0.0]) + (PU * CR)) * FA) + (((((PU * BS) * G) + Lanes([0.0, 0.0, PW[0], PW[1], 0.0, 0.0])) * (NC - (EV * EV))) * EZ)) * ((AB + (EY * G)) + (CU * FB))) + (((((PU * DD) * G) + Lanes([0.0, 0.0, PY[0], PY[1], 0.0, 0.0])) + Lanes([PZ[0], 0.0, PZ[1], PZ[2], 0.0, 0.0])) * (EZ * FA))) - Lanes([QB[0], QB[1], QB[2], QB[3], QB[4], 0.0])) * AY;
                            FY = EO;
                            NR = QC;
                        } else {
                            let FE = if AT == AU { 1.0 } else { 0.0 };
                            let NS = if FE != 0.0 {
                                let FF = CT + (DD * CI);
                                let PG = PC * BS;
                                let FG = CN + (BS * CM);
                                let PH = NX * FG;
                                let FH = (FG * G).tanh();
                                let PI = NY * FG;
                                let FI = (FG * I).tanh();
                                let FJ = CR * CI;
                                let FL = FH + (FK * FI);
                                let PJ = NY * FK;
                                let FM = G + (FK * I);
                                let PK = (Lanes([0.0, NX[0], NX[1]]) + Lanes([PJ[0], 0.0, PJ[1]])) * FF;
                                let FN = rspice_limexp((DH * (G - BV)));
                                let PL = Lanes([(NN * FN), 0.0, 0.0]) + ((((Lanes([0.0, NX[0], NX[1]]) - Lanes([NL, 0.0, 0.0])) * DH) * FN) * CU);
                                let PM = ((((Lanes([(NM * CI), 0.0, 0.0, 0.0, 0.0, 0.0]) + (PB * CR)) * FL) + (((((PG * G) + Lanes([0.0, 0.0, PH[0], PH[1], 0.0, 0.0])) * (NC - (FH * FH))) + ((((PG * I) + Lanes([0.0, PI[0], 0.0, PI[1], 0.0, 0.0])) * (NC - (FI * FI))) * FK)) * FJ)) * ((AB + (FF * FM)) + (CU * FN))) + (((((PB * DD) * FM) + Lanes([0.0, PK[0], PK[1], PK[2], 0.0, 0.0])) + Lanes([PL[0], 0.0, PL[1], PL[2], 0.0, 0.0])) * (FJ * FL));
                                PM
                            } else {
                                PF
                            };
                            FY = CM;
                            NR = NS;
                        }
                        FX = FY;
                        NQ = NR;
                    }
                    FW = FX;
                    NP = NQ;
                }
                FV = FW;
                NO = NP;
            }
            let FO = if (if CQ != 0.0 || (if AT == AB { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if AT == AU { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let GC;
            let GF;
            if FO != 0.0 {
                let FR = FQ * CI;
                let FS = FP + FR;
                let FU = FT + FR;
                GC = FU;
                GF = FS;
            } else {
                let FZ = FQ * FV;
                let GA = FP + FZ;
                let GB = FT + FZ;
                GC = GB;
                GF = GA;
            }
            let GD = AB + (parameters[76] * BK);
            let GE = GC * GD;
            let GG = GF * GD;
            let GI = if GH == A { 1.0 } else { 0.0 };
            let HE;
            let HF;
            let HH;
            let HK;
            let HM;
            let HN;
            let HO;
            if GI != 0.0 {
                let GL = rspice_limexp((GJ * ((-1e0f64 * GK).tanh())));
                let GM = H - GK;
                let GO = (-H) - GN;
                let GP = E - GK;
                let GR = F - GQ;
                HE = GO;
                HF = A;
                HH = GM;
                HK = GL;
                HM = GR;
                HN = A;
                HO = GP;
            } else {
                let GS = rspice_limexp(((-GJ) * GK));
                let GU = -GT;
                let GV = rspice_limexp((GU * GN));
                let GW = rspice_limexp((GU * GQ));
                let GX = if GH == AB { 1.0 } else { 0.0 };
                let HI;
                let HP;
                if GX != 0.0 {
                    let GY = (H - GK).tanh();
                    let GZ = (E - GK).tanh();
                    HI = GY;
                    HP = GZ;
                } else {
                    let HA = H - GK;
                    let HB = E - GK;
                    HI = HA;
                    HP = HB;
                }
                let HC = (-H) - GN;
                let HD = F - GQ;
                HE = HC;
                HF = GV;
                HH = HI;
                HK = GS;
                HM = HD;
                HN = GW;
                HO = HP;
            }
            let HJ = 1e-3f64 * parameters[82];
            let HL = HG * (((rspice_limexp((GJ * HH))) - (HJ * ((rspice_limexp((GT * HE))) - HF))) - HK);
            let HQ = HG * (((rspice_limexp((GJ * HO))) - (HJ * ((rspice_limexp((GT * HM))) - HN))) - HK);
            let HR = if AV == A { 1.0 } else { 0.0 };
            if HR != 0.0 {
            } else {
                let HS = if AV == AB { 1.0 } else { 0.0 };
                if HS != 0.0 {
                } else {
                    let HU = if AV == DN { 1.0 } else { 0.0 };
                    if HU != 0.0 {
                    } else {
                        let HV = if AV == EB { 1.0 } else { 0.0 };
                        if HV != 0.0 {
                        } else {
                            let HW = if AV == AU { 1.0 } else { 0.0 };
                            if HW != 0.0 {
                            } else {
                            }
                        }
                    }
                }
            }
            let HX = if (if AV == DN { 1.0 } else { 0.0 }) != 0.0 || (if AV == AU { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            if HX != 0.0 {
            } else {
            }
            let HY = if parameters[58] > A { 1.0 } else { 0.0 };
            if HY != 0.0 {
            } else {
            }
            let HZ = if (if parameters[63] > A { 1.0 } else { 0.0 }) != 0.0 || (if parameters[62] > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            if HZ != 0.0 {
            } else {
            }
            let IA = if parameters[60] > A { 1.0 } else { 0.0 };
            if IA != 0.0 {
            } else {
            }
            let IC = if IB > A { 1.0 } else { 0.0 };
            if IC != 0.0 {
            } else {
            }
            let JS;
            let JT;
            if ID != 0.0 {
                let IF = (5.5226012e-23f64 * V) * IB;
                JS = AB;
                JT = IF;
            } else {
                JS = A;
                JT = A;
            }
            let IG = if parameters[49] > A { 1.0 } else { 0.0 };
            if IG != 0.0 {
            } else {
            }
            let II = if IH > A { 1.0 } else { 0.0 };
            let JU;
            let JW;
            if II != 0.0 {
                let JV;
                let JX;
                if ID != 0.0 {
                    let IJ = (5.5226012e-23f64 * V) * IH;
                    JV = AB;
                    JX = IJ;
                } else {
                    JV = A;
                    JX = A;
                }
                JU = JV;
                JW = JX;
            } else {
                JU = A;
                JW = A;
            }
            let IK = if FT > A { 1.0 } else { 0.0 };
            let JY;
            let KA;
            if IK != 0.0 {
                let JZ;
                let KB;
                if ID != 0.0 {
                    let IL = (5.5226012e-23f64 * V) * GE;
                    JZ = AB;
                    KB = IL;
                } else {
                    JZ = A;
                    KB = A;
                }
                JY = JZ;
                KA = KB;
            } else {
                JY = A;
                KA = A;
            }
            let IM = if (if FP > A { 1.0 } else { 0.0 }) != 0.0 || (if FQ > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let KC;
            let KE;
            if IM != 0.0 {
                let KD;
                let KF;
                if ID != 0.0 {
                    let IN = (5.5226012e-23f64 * V) * GG;
                    KD = AB;
                    KF = IN;
                } else {
                    KD = A;
                    KF = A;
                }
                KC = KD;
                KE = KF;
            } else {
                KC = A;
                KE = A;
            }
            let QZ = NO[5];
            let IO = QZ / (AB + (QZ * FT));
            let IQ = if IP == A { 1.0 } else { 0.0 };
            let KG;
            let KI;
            let KK;
            let KM;
            let KO;
            let KQ;
            let KT;
            let KW;
            let KZ;
            let LC;
            let LF;
            let LI;
            let LL;
            let LO;
            let LR;
            let LU;
            let LX;
            let MA;
            let ME;
            let MI;
            if IQ != 0.0 {
                let IR = (J.abs()) + (HQ.abs());
                let IS = (((parameters[99] * AU) * IE) * V) * (((((((parameters[93] + L) * (AB + (((parameters[95] * CI) * (CP.abs())) * (AB + (CT * G))))) / V) * IR) + ((parameters[94] * IR) * IR)).abs()).sqrt());
                let KH;
                let KJ;
                let KL;
                let KN;
                let KP;
                if ID != 0.0 {
                    let IT = IS * parameters[96];
                    KH = AB;
                    KJ = IS;
                    KL = AB;
                    KN = IT;
                    KP = IU;
                } else {
                    KH = A;
                    KJ = A;
                    KL = A;
                    KN = A;
                    KP = A;
                }
                KG = KH;
                KI = KJ;
                KK = KL;
                KM = KN;
                KO = KP;
                KQ = A;
                KT = A;
                KW = A;
                KZ = A;
                LC = A;
                LF = A;
                LI = A;
                LL = A;
                LO = A;
                LR = A;
                LU = A;
                LX = A;
                MA = A;
                ME = A;
                MI = A;
            } else {
                let IV = if IP == AB { 1.0 } else { 0.0 };
                let KR;
                let KU;
                let KX;
                let LA;
                let LD;
                let LG;
                let LJ;
                let LM;
                let LP;
                let LS;
                let LV;
                let LY;
                let MB;
                let MF;
                let MJ;
                if IV != 0.0 {
                    let KS;
                    let KV;
                    let KY;
                    let LB;
                    let LE;
                    let LH;
                    let LK;
                    let LN;
                    let LQ;
                    let LT;
                    let LW;
                    let LZ;
                    let MC;
                    let MG;
                    let MK;
                    if ID != 0.0 {
                        let IX = ((5.5226012e-23f64 * V) * IO) * IW;
                        let IY = if IO > A { 1.0 } else { 0.0 };
                        let JC = if IY != 0.0 {
                            let JA = (((((HT * HT) * AU) * IE) * V) * IZ) / IO;
                            JA
                        } else {
                            A
                        };
                        let JB = (((5.5226012e-23f64 * V) * parameters[88]) * HT) * ((IW * IZ).sqrt());
                        let JD = (((5.5226012e-23f64 * V) * IO) * IW) * parameters[89];
                        let JF = if JE > A { 1.0 } else { 0.0 };
                        let MD;
                        let MH;
                        let ML;
                        if JF != 0.0 {
                            let JH = JE * (J.powf(JG));
                            MD = AB;
                            MH = JH;
                            ML = JI;
                        } else {
                            MD = A;
                            MH = A;
                            ML = A;
                        }
                        KS = AB;
                        KV = JB;
                        KY = AB;
                        LB = JB;
                        LE = AB;
                        LH = IX;
                        LK = AB;
                        LN = JC;
                        LQ = DN;
                        LT = AB;
                        LW = JD;
                        LZ = AB;
                        MC = MD;
                        MG = MH;
                        MK = ML;
                    } else {
                        KS = A;
                        KV = A;
                        KY = A;
                        LB = A;
                        LE = A;
                        LH = A;
                        LK = A;
                        LN = A;
                        LQ = A;
                        LT = A;
                        LW = A;
                        LZ = A;
                        MC = A;
                        MG = A;
                        MK = A;
                    }
                    KR = KS;
                    KU = KV;
                    KX = KY;
                    LA = LB;
                    LD = LE;
                    LG = LH;
                    LJ = LK;
                    LM = LN;
                    LP = LQ;
                    LS = LT;
                    LV = LW;
                    LY = LZ;
                    MB = MC;
                    MF = MG;
                    MJ = MK;
                } else {
                    let JJ = if IP == DN { 1.0 } else { 0.0 };
                    if JJ != 0.0 {
                    } else {
                    }
                    KR = A;
                    KU = A;
                    KX = A;
                    LA = A;
                    LD = A;
                    LG = A;
                    LJ = A;
                    LM = A;
                    LP = A;
                    LS = A;
                    LV = A;
                    LY = A;
                    MB = A;
                    MF = A;
                    MJ = A;
                }
                KG = A;
                KI = A;
                KK = A;
                KM = A;
                KO = A;
                KQ = KR;
                KT = KU;
                KW = KX;
                KZ = LA;
                LC = LD;
                LF = LG;
                LI = LJ;
                LL = LM;
                LO = LP;
                LR = LS;
                LU = LV;
                LX = LY;
                MA = MB;
                ME = MF;
                MI = MJ;
            }
            let MM;
            let MN;
            let MO;
            let MP;
            let MQ;
            let MS;
            let MU;
            let MW;
            let MY;
            let NA;
            if ID != 0.0 {
                let JK = HL.abs();
                let JL = 3.204352924e-19f64 * JK;
                let JM = HQ.abs();
                let JN = 3.204352924e-19f64 * JM;
                let JO = if JE > A { 1.0 } else { 0.0 };
                let MR;
                let MT;
                let MV;
                let MX;
                let MZ;
                let NB;
                if JO != 0.0 {
                    let JP = JE * (JK.powf(JG));
                    let JQ = JE * (JM.powf(JG));
                    MR = AB;
                    MT = JP;
                    MV = JI;
                    MX = AB;
                    MZ = JQ;
                    NB = JI;
                } else {
                    MR = A;
                    MT = A;
                    MV = A;
                    MX = A;
                    MZ = A;
                    NB = A;
                }
                MM = AB;
                MN = JL;
                MO = AB;
                MP = JN;
                MQ = MR;
                MS = MT;
                MU = MV;
                MW = MX;
                MY = MZ;
                NA = NB;
            } else {
                MM = A;
                MN = A;
                MO = A;
                MP = A;
                MQ = A;
                MS = A;
                MU = A;
                MW = A;
                MY = A;
                NA = A;
            }
            let JR = if R == AB { 1.0 } else { 0.0 };
            if JR != 0.0 {
            } else {
            }
        if JS == 0.0 {
            if !visitor.visit(0, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = JT;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 0, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 0, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(0, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if JU == 0.0 {
            if !visitor.visit(1, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = JW;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd / self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(1, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if JY == 0.0 {
            if !visitor.visit(2, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = KA;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 2, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd / self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 2, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(2, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if KC == 0.0 {
            if !visitor.visit(3, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = KE;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 3, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd / self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 3, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(3, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if KG == 0.0 {
            if !visitor.visit(4, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = KI;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 4, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 4, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(4, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if KK == 0.0 {
            if !visitor.visit(5, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = KM;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 5, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = Some(KO);
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 5, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 5, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(5, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if KQ == 0.0 {
            if !visitor.visit(6, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = KT;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 6, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 6, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(6, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if KW == 0.0 {
            if !visitor.visit(7, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = KZ;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 7, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 7, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(7, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if LC == 0.0 {
            if !visitor.visit(8, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = LF;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 8, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 8, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(8, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if LI == 0.0 {
            if !visitor.visit(9, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = LL;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 9, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = Some(LO);
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 9, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 9, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(9, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if LR == 0.0 {
            if !visitor.visit(10, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = LU;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 10, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = Some(LX);
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 10, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 10, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(10, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if MA == 0.0 {
            if !visitor.visit(11, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = ME;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 11, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = Some(MI);
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 11, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 11, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(11, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if MM == 0.0 {
            if !visitor.visit(12, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = MN;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 12, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 12, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(12, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if MO == 0.0 {
            if !visitor.visit(13, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = MP;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 13, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 13, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(13, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if MQ == 0.0 {
            if !visitor.visit(14, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = MS;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 14, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = Some(MU);
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 14, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 14, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(14, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if MW == 0.0 {
            if !visitor.visit(15, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = MY;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 15, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = Some(NA);
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 15, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 15, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(15, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        Ok(())
    }
}
