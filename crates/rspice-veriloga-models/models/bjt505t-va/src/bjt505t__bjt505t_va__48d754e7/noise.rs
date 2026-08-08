#![allow(dead_code, non_snake_case, unused_parens, unused_variables)]

use super::state::Instance;
use rspice_veriloga_runtime::GeneratedEvalContext;
pub use rspice_veriloga_runtime::{GeneratedNoiseDescriptor, GeneratedNoiseEndpoint, GeneratedNoiseEvaluation, GeneratedNoiseEvaluationError, GeneratedNoiseEvaluationRef, GeneratedNoiseKind, GeneratedNoiseVisitor};

pub static NOISE_SOURCES: [GeneratedNoiseDescriptor; 28] = [
    GeneratedNoiseDescriptor { mechanism: "WHITE_NOI_GND_IN", label: Some("in"), kind: GeneratedNoiseKind::White, equation: 33, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(12), name: "noi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: None, name: "0", is_internal: false }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C2_B2_IAVL", label: Some("iavl"), kind: GeneratedNoiseKind::White, equation: 38, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(9), name: "c2", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(7), name: "b2", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_B2_E1_IB2E1", label: Some("ib2e1"), kind: GeneratedNoiseKind::White, equation: 39, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(7), name: "b2", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(5), name: "e1", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_E_E1_RE", label: Some("re"), kind: GeneratedNoiseKind::White, equation: 40, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(2), name: "e", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(5), name: "e1", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_B_B1_RBC", label: Some("rbc"), kind: GeneratedNoiseKind::White, equation: 41, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(1), name: "b", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(6), name: "b1", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_B1_B2_RBV", label: Some("rbv"), kind: GeneratedNoiseKind::White, equation: 42, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(6), name: "b1", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(7), name: "b2", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_B2_E1_IB2E1_F", label: Some("ib2e1_f"), kind: GeneratedNoiseKind::Flicker, equation: 43, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(7), name: "b2", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(5), name: "e1", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_B1_E1_IB1E1_F", label: Some("ib1e1_f"), kind: GeneratedNoiseKind::Flicker, equation: 44, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(6), name: "b1", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(5), name: "e1", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_B1_E1_IB1E1", label: Some("ib1e1"), kind: GeneratedNoiseKind::White, equation: 45, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(6), name: "b1", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(5), name: "e1", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_B1_C4_IB3", label: Some("ib3"), kind: GeneratedNoiseKind::White, equation: 46, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(6), name: "b1", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(11), name: "c4", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_B1_C4_IB3_F", label: Some("ib3_f"), kind: GeneratedNoiseKind::Flicker, equation: 47, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(6), name: "b1", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(11), name: "c4", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_B1_C4_IEX", label: Some("iex"), kind: GeneratedNoiseKind::White, equation: 48, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(6), name: "b1", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(11), name: "c4", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_B1_C4_IEX_F", label: Some("iex_f"), kind: GeneratedNoiseKind::Flicker, equation: 49, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(6), name: "b1", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(11), name: "c4", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_B_C3_XIEX", label: Some("xiex"), kind: GeneratedNoiseKind::White, equation: 50, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(1), name: "b", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(10), name: "c3", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_B_C3_XIEX_F", label: Some("xiex_f"), kind: GeneratedNoiseKind::Flicker, equation: 51, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(1), name: "b", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(10), name: "c3", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C1_B2_IZTCB", label: Some("iztcb"), kind: GeneratedNoiseKind::White, equation: 52, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(8), name: "c1", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(7), name: "b2", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C2_B2_IZTCB", label: Some("iztcb"), kind: GeneratedNoiseKind::White, equation: 53, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(9), name: "c2", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(7), name: "b2", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_B2_S_ISUB_INT", label: Some("isub_int"), kind: GeneratedNoiseKind::White, equation: 54, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(7), name: "b2", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(3), name: "s", is_internal: false }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_B1_S_ISUB", label: Some("isub"), kind: GeneratedNoiseKind::White, equation: 55, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(6), name: "b1", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(3), name: "s", is_internal: false }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_B_S_XISUB", label: Some("xisub"), kind: GeneratedNoiseKind::White, equation: 56, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(1), name: "b", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(3), name: "s", is_internal: false }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C_C3_RCC", label: Some("rcc"), kind: GeneratedNoiseKind::White, equation: 57, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(0), name: "c", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(10), name: "c3", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C3_C4_RCBLX", label: Some("rcblx"), kind: GeneratedNoiseKind::White, equation: 58, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(10), name: "c3", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(11), name: "c4", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C4_C1_RCBLI", label: Some("rcbli"), kind: GeneratedNoiseKind::White, equation: 59, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(11), name: "c4", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(8), name: "c1", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C_C3_RCC", label: Some("rcc"), kind: GeneratedNoiseKind::White, equation: 60, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(0), name: "c", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(10), name: "c3", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C3_C1_RCBLX", label: Some("rcblx"), kind: GeneratedNoiseKind::White, equation: 61, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(10), name: "c3", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(8), name: "c1", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C_C4_RCC", label: Some("rcc"), kind: GeneratedNoiseKind::White, equation: 62, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(0), name: "c", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(11), name: "c4", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C4_C1_RCBLI", label: Some("rcbli"), kind: GeneratedNoiseKind::White, equation: 63, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(11), name: "c4", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(8), name: "c1", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C_C1_RCC", label: Some("rcc"), kind: GeneratedNoiseKind::White, equation: 64, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(0), name: "c", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(8), name: "c1", is_internal: true }, table_len: 0, table_log_interp: false },
];

impl Instance {
    pub fn evaluate_noise_sources(&self, ctx: &GeneratedEvalContext<'_>, visitor: &mut dyn GeneratedNoiseVisitor) -> Result<(), GeneratedNoiseEvaluationError> {
        if !self.multiplicity.is_finite() || self.multiplicity <= 0.0 {
            return Err(GeneratedNoiseEvaluationError::InvalidMultiplicity { value: self.multiplicity });
        }
        let parameters = &self.params.values;
        let temperature = ctx.temperature();
        let node_potentials = [ctx.node_voltage(self.nodes[0]), ctx.node_voltage(self.nodes[1]), ctx.node_voltage(self.nodes[2]), ctx.node_voltage(self.nodes[3]), ctx.node_voltage(self.nodes[4]), ctx.node_voltage(self.nodes[5]), ctx.node_voltage(self.nodes[6]), ctx.node_voltage(self.nodes[7]), ctx.node_voltage(self.nodes[8]), ctx.node_voltage(self.nodes[9]), ctx.node_voltage(self.nodes[10]), ctx.node_voltage(self.nodes[11]), ctx.node_voltage(self.nodes[12])];
            let A = 0e0f64;
            let B = parameters[3];
            let C = 1e0f64;
            let E = 7.03e7f64;
            let F = 1.23e8f64;
            let G = 1.58e8f64;
            let H = 2.04e8f64;
            let I = parameters[33];
            let M = parameters[154];
            let O = 1e-12f64;
            let Q = parameters[1];
            let U = 1e-3f64;
            let V = 2e0f64;
            let W = parameters[67];
            let Z = parameters[114];
            let AA = parameters[115];
            let AB = parameters[116];
            let AD = 5e-2f64;
            let AE = 1e-1f64;
            let AK = parameters[66];
            let AM = parameters[71];
            let AN = parameters[72];
            let AQ = parameters[117];
            let AR = parameters[118];
            let AS = parameters[119];
            let BB = node_potentials[4];
            let BF = parameters[125];
            let BL = 8.617086918058125e-5f64;
            let CD = 3e0f64;
            let CF = parameters[105];
            let CL = parameters[110];
            let DN = parameters[140];
            let EA = parameters[75];
            let EG = parameters[97];
            let EJ = parameters[98];
            let EK = parameters[96];
            let EO = parameters[57];
            let EQ = parameters[58];
            let ET = parameters[59];
            let EV = parameters[99];
            let EX = parameters[122];
            let EZ = parameters[10];
            let FH = parameters[123];
            let FJ = parameters[11];
            let FS = 1e-6f64;
            let FV = 5e-1f64;
            let FY = 4e0f64;
            let FZ = parameters[121];
            let GD = parameters[103];
            let GF = 6e0f64;
            let GG = parameters[21];
            let GJ = parameters[32];
            let GM = parameters[17];
            let GP = parameters[19];
            let GW = parameters[23];
            let GY = parameters[150];
            let HE = parameters[35];
            let HG = parameters[34];
            let HK = parameters[37];
            let HM = parameters[36];
            let HR = parameters[141];
            let HU = parameters[87];
            let HW = parameters[88];
            let IG = parameters[92];
            let IR = node_potentials[7];
            let IS = node_potentials[8];
            let IU = node_potentials[9];
            let IW = node_potentials[5];
            let IY = node_potentials[6];
            let JD = node_potentials[1];
            let JH = node_potentials[11];
            let JP = parameters[151];
            let LQ = parameters[153];
            let LW = 1e2f64;
            let MJ = parameters[62];
            let MK = parameters[61];
            let MO = parameters[63];
            let NJ = parameters[152];
            let ON = parameters[74];
            let PV = 1.0000000000000002e-2f64;
            let QH = 1e-4f64;
            let QT = parameters[158];
            let RL = 4e1f64;
            let RW = parameters[93];
            let TT = 1e-30f64;
            let TV = 1.6666666666666666e-1f64;
            let UD = 3.333333333333333e-1f64;
            let UE = 2.5e-1f64;
            let VH = parameters[143];
            let VK = parameters[144];
            let VR = parameters[5];
            let WG = 1.21e-2f64;
            let WT = 1e-6f64;
            let WU = 1e-12f64;
            let WZ = parameters[82];
            let XB = parameters[81];
            let XS = 1.0000000000000002e-2f64;
            let YE = parameters[39];
            let YG = parameters[44];
            let YP = parameters[41];
            let YU = parameters[40];
            let YZ = parameters[45];
            let ZD = parameters[7];
            let ZP = parameters[47];
            let AAH = parameters[48];
            let AAL = parameters[51];
            let AEY = parameters[131];
            let AFD = parameters[128];
            let AFE = parameters[126];
            let D = if B == C { 1.0 } else { 0.0 };
            let ID;
            let ZZ;
            if D != 0.0 {
                ID = F;
                ZZ = E;
            } else {
                ID = H;
                ZZ = G;
            }
            let J = C - I;
            let K = parameters[4] + 2.7315e2f64;
            let L = temperature + parameters[0];
            let N = if M == A { 1.0 } else { 0.0 };
            let P = if N != 0.0 {
                O
            } else {
                M
            };
            let R = P * Q;
            let S = C / R;
            let T = if parameters[134] > A { 1.0 } else { 0.0 };
            if T != 0.0 {
            } else {
            }
            let X = V.powf((V - W));
            let Y = C / X;
            let AC = Z + (((AA * K) * K) / (K + AB));
            let AF = (AC - AD) / AE;
            let AG = if AC < AD { 1.0 } else { 0.0 };
            let BR = if AG != 0.0 {
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
            let BX = if AV != 0.0 {
                let AW = AD + (AE * ((C + (AU.exp())).ln()));
                AW
            } else {
                let AX = AT + (AE * ((C + ((-AU).exp())).ln()));
                AX
            };
            let AY = C / AQ;
            let AZ = C / AM;
            let BA = C - (C / parameters[83]);
            let BC = if BB < A { 1.0 } else { 0.0 };
            let BE = if BC != 0.0 {
                let BD = -((C - BB).ln());
                BD
            } else {
                BB
            };
            let BG = if BE < BF { 1.0 } else { 0.0 };
            let BI = if BG != 0.0 {
                BE
            } else {
                let BH = BF + ((C + (BE - BF)).ln());
                BH
            };
            let BJ = L + BI;
            let BK = BJ / K;
            let BM = BL * BJ;
            let BN = C / BM;
            let BO = BN - (C / (BL * K));
            let BP = BJ - K;
            let BQ = BK.ln();
            let BS = BR - (((AA * BJ) * BJ) / (BJ + AB));
            let BT = (BS - AD) / AE;
            let BU = if BS < AD { 1.0 } else { 0.0 };
            let HB = if BU != 0.0 {
                let BV = AD + (AE * ((C + (BT.exp())).ln()));
                BV
            } else {
                let BW = BS + (AE * ((C + ((-BT).exp())).ln()));
                BW
            };
            let BY = BX - (((AR * BJ) * BJ) / (BJ + AS));
            let BZ = (BY - AD) / AE;
            let CA = if BY < AD { 1.0 } else { 0.0 };
            let HI = if CA != 0.0 {
                let CB = AD + (AE * ((C + (BZ.exp())).ln()));
                CB
            } else {
                let CC = BY + (AE * ((C + ((-BZ).exp())).ln()));
                CC
            };
            let CE = C - BK;
            let CG = (((-3e0f64 * BM) * BQ) + (AK * BK)) + (CE * CF);
            let CH = (AD - CG) / BM;
            let CI = if AD < CG { 1.0 } else { 0.0 };
            let DT = if CI != 0.0 {
                let CJ = CG + (BM * ((C + (CH.exp())).ln()));
                CJ
            } else {
                let CK = AD + (BM * ((C + ((-CH).exp())).ln()));
                CK
            };
            let CM = CE * CL;
            let CN = (((-3e0f64 * BM) * BQ) + (parameters[64] * BK)) + CM;
            let CO = (AD - CN) / BM;
            let CP = if AD < CN { 1.0 } else { 0.0 };
            let KW = if CP != 0.0 {
                let CQ = CN + (BM * ((C + (CO.exp())).ln()));
                CQ
            } else {
                let CR = AD + (BM * ((C + ((-CO).exp())).ln()));
                CR
            };
            let CS = (((-3e0f64 * BM) * BQ) + (parameters[80] * BK)) + CM;
            let CT = (AD - CS) / BM;
            let CU = if AD < CS { 1.0 } else { 0.0 };
            let ADE = if CU != 0.0 {
                let CV = CS + (BM * ((C + (CT.exp())).ln()));
                CV
            } else {
                let CW = AD + (BM * ((C + ((-CT).exp())).ln()));
                CW
            };
            let CX = AM * BK;
            let CY = (((-3e0f64 * BM) * BQ) + CX) + CM;
            let CZ = (AD - CY) / BM;
            let DA = if AD < CY { 1.0 } else { 0.0 };
            let EB = if DA != 0.0 {
                let DB = CY + (BM * ((C + (CZ.exp())).ln()));
                DB
            } else {
                let DC = AD + (BM * ((C + ((-CZ).exp())).ln()));
                DC
            };
            let DD = (((-3e0f64 * BM) * BQ) + CX) + CM;
            let DE = (AD - DD) / BM;
            let DF = if AD < DD { 1.0 } else { 0.0 };
            let DV = if DF != 0.0 {
                let DG = DD + (BM * ((C + (DE.exp())).ln()));
                DG
            } else {
                let DH = AD + (BM * ((C + ((-DE).exp())).ln()));
                DH
            };
            let DI = (((-3e0f64 * BM) * BQ) + (parameters[27] * BK)) + (CE * parameters[109]);
            let DJ = (AD - DI) / BM;
            let DK = if AD < DI { 1.0 } else { 0.0 };
            let RF = if DK != 0.0 {
                let DL = DI + (BM * ((C + (DJ.exp())).ln()));
                DL
            } else {
                let DM = AD + (BM * ((C + ((-DJ).exp())).ln()));
                DM
            };
            let DO = (((-3e0f64 * BM) * BQ) + (parameters[138] * BK)) + (CE * DN);
            let DP = (AD - DO) / BM;
            let DQ = if AD < DO { 1.0 } else { 0.0 };
            let DZ = if DQ != 0.0 {
                let DR = DO + (BM * ((C + (DP.exp())).ln()));
                DR
            } else {
                let DS = AD + (BM * ((C + ((-DP).exp())).ln()));
                DS
            };
            let DU = C / DT;
            let DW = C / DV;
            let DX = (AK * DU).powf(W);
            let DY = (AM * DW).powf(AN);
            let EC = ((C - EA) * ((AM / EB).powf(AN))) + EA;
            let ED = C / EC;
            let EE = parameters[70] * EC;
            let EF = EA * ED;
            let EH = parameters[54] * ((BQ * EG).exp());
            let EI = if EH < R { 1.0 } else { 0.0 };
            let ABK = if EI != 0.0 {
                R
            } else {
                EH
            };
            let EL = parameters[56] * ((BQ * (EJ - EK)).exp());
            let EM = parameters[55] * ((BQ * parameters[101]).exp());
            let EN = if EM < R { 1.0 } else { 0.0 };
            let ABI = if EN != 0.0 {
                R
            } else {
                EM
            };
            let EP = EO * ((BQ * parameters[102]).exp());
            let ER = (BQ * parameters[104]).exp();
            let ES = EQ * ER;
            let EU = ET * ER;
            let EW = parameters[60] * ((BQ * EV).exp());
            let EY = if EX != A { 1.0 } else { 0.0 };
            let GA;
            if EY != 0.0 {
                let FA = EZ * (C + (BP * EX));
                let FB = (FA - C) / U;
                let FC = if FA < C { 1.0 } else { 0.0 };
                let FF = if FC != 0.0 {
                    let FD = C + (U * ((C + (FB.exp())).ln()));
                    FD
                } else {
                    let FE = FA + (U * ((C + ((-FB).exp())).ln()));
                    FE
                };
                let FG = FF - 6.931471805599453e-4f64;
                GA = FG;
            } else {
                GA = EZ;
            }
            let FI = if FH != A { 1.0 } else { 0.0 };
            let PO;
            if FI != 0.0 {
                let FK = FJ * (C + (BP * FH));
                let FL = (FK - C) / U;
                let FM = if FK < C { 1.0 } else { 0.0 };
                let FP = if FM != 0.0 {
                    let FN = C + (U * ((C + (FL.exp())).ln()));
                    FN
                } else {
                    let FO = FK + (U * ((C + ((-FL).exp())).ln()));
                    FO
                };
                let FQ = FP - 6.931471805599453e-4f64;
                PO = FQ;
            } else {
                PO = FJ;
            }
            let FR = parameters[43] * (C + (parameters[124] * BP));
            let FT = FR * FR;
            let FU = if FR < A { 1.0 } else { 0.0 };
            let YO = if FU != 0.0 {
                let FW = 5e-7f64 / (((FT + FS).sqrt()) - FR);
                FW
            } else {
                let FX = FV * (((FT + FS).sqrt()) + FR);
                FX
            };
            let GB = (parameters[9] * (((BQ * (((FY - EJ) - EK) + FZ)) / GA).exp())) * ((((-CF) * BO) / GA).exp());
            let GC = parameters[12] * ((BQ * (C - EJ)).exp());
            let GE = parameters[30] * ((BQ * (C - GD)).exp());
            let GH = (-parameters[113]) * BO;
            let GI = (parameters[20] * ((BQ * (GF - (V * GG))).exp())) * ((GH / GG).exp());
            let GK = (parameters[31] * ((BQ * (GF - (V * GJ))).exp())) * ((((-CL) * BO) / GJ).exp());
            let GL = BQ * ((FY - EG) + FZ);
            let GN = (-parameters[111]) * BO;
            let GO = (parameters[16] * ((GL / GM).exp())) * ((GN / GM).exp());
            let GQ = (parameters[18] * ((GL / GP).exp())) * ((GN / GP).exp());
            let GR = if parameters[24] == C { 1.0 } else { 0.0 };
            let RR;
            let RT;
            let SL;
            if GR != 0.0 {
                let GS = parameters[25] * ((((-parameters[107]) * BO) / GM).exp());
                let GT = parameters[28] * (((-parameters[106]) * BO).exp());
                let GU = parameters[26] * ((((-parameters[108]) * BO) / GP).exp());
                RR = GS;
                RT = GT;
                SL = GU;
            } else {
                RR = A;
                RT = A;
                SL = A;
            }
            let GV = (parameters[29] * ((BQ * ((FY - GD) + FZ)).exp())) * (((-parameters[112]) * BO).exp());
            let GX = (parameters[22] * ((BQ * (GF - (V * GW))).exp())) * ((GH / GW).exp());
            let GZ = (parameters[149] * ((BQ * (FY / GY)).exp())) * ((GH / GY).exp());
            let HA = (parameters[155] * (BK.sqrt())) * ((parameters[157] * BP).exp());
            let HC = (HB * AJ).powf(-5e-1f64);
            let HD = C / DX;
            let HF = (((((((HE * HB) * HB) * HC) * HD) * AK) * DU) * AJ) * AJ;
            let HH = ((((((HG * HC) * DT) * DT) * AL) * AL) * DX) * ((HE - HF).exp());
            let HJ = (HI * AY).powf(-5e-1f64);
            let HL = (((((((HK * HI) * HI) * HJ) * (C / DY)) * AM) * DW) * AY) * AY;
            let HN = ((((((HM * HJ) * DV) * DV) * AZ) * AZ) * DY) * ((HK - HL).exp());
            let HO = (BQ * EK).exp();
            let HP = (parameters[14] * HO) * ED;
            let HQ = (parameters[13] * HO) * HD;
            let HS = (parameters[133] * ((BQ * (FY - HR)).exp())) * (((-DN) * BO).exp());
            let HT = parameters[135] * ((BQ * (C - HR)).exp());
            let HV = HU * ((BQ * ((EK + EJ) - C)).exp());
            let HX = HW * ((BQ * (EV - C)).exp());
            let HY = HV + HX;
            let HZ = (parameters[89] * HY) / (HU + HW);
            let IA = parameters[90] * ((BQ * (parameters[100] - C)).exp());
            let IB = BJ - 3e2f64;
            let IC = if BJ < 5.25e2f64 { 1.0 } else { 0.0 };
            let AAA = if IC != 0.0 {
                let IE = ID * ((C + (7.2e-4f64 * IB)) - ((1.6e-6f64 * IB) * IB));
                IE
            } else {
                let IF = ID * 1.081e0f64;
                IF
            };
            let IH = IG * HO;
            let II = if EO > A { 1.0 } else { 0.0 };
            let ACA;
            if II != 0.0 {
                let IJ = C / EP;
                let IK = if IJ > S { 1.0 } else { 0.0 };
                let ACB = if IK != 0.0 {
                    S
                } else {
                    IJ
                };
                ACA = ACB;
            } else {
                ACA = A;
            }
            let IL = if EQ > A { 1.0 } else { 0.0 };
            let ACC;
            if IL != 0.0 {
                let IM = C / ES;
                let IN = if IM > S { 1.0 } else { 0.0 };
                let ACD = if IN != 0.0 {
                    S
                } else {
                    IM
                };
                ACC = ACD;
            } else {
                ACC = A;
            }
            let IO = if ET > A { 1.0 } else { 0.0 };
            let ACE;
            if IO != 0.0 {
                let IP = C / EU;
                let IQ = if IP > S { 1.0 } else { 0.0 };
                let ACF = if IQ != 0.0 {
                    S
                } else {
                    IP
                };
                ACE = ACF;
            } else {
                ACE = A;
            }
            let IT = B * (IR - IS);
            let IV = B * (IR - IU);
            let IX = B * (IR - IW);
            let IZ = B * (IY - IW);
            let JA = B * (IY - IR);
            let JB = B * (node_potentials[3] - IS);
            let JC = B * (IS - IU);
            let JE = B * (JD - IY);
            let JF = B * (JD - node_potentials[2]);
            let JG = B * (JD - node_potentials[0]);
            let JI = B * (JH - IS);
            let JJ = B * (node_potentials[10] - JH);
            let JK = ((JA + IV) - JC) - JI;
            let JL = JG + ((((-JG) + JE) + JK) - JJ);
            let JM = JB - JI;
            let JN = JM - JJ;
            let JO = IV * BN;
            let JQ = if JO < JP { 1.0 } else { 0.0 };
            let NW = if JQ != 0.0 {
                let JR = JO.exp();
                JR
            } else {
                let JS = (JP.exp()) * (C + (JO - JP));
                JS
            };
            let JT = IX * BN;
            let JU = JT / GA;
            let JV = if JU < JP { 1.0 } else { 0.0 };
            let PK = if JV != 0.0 {
                let JW = JU.exp();
                JW
            } else {
                let JX = (JP.exp()) * (C + (JU - JP));
                JX
            };
            let JY = JK * BN;
            let JZ = if JY < JP { 1.0 } else { 0.0 };
            let VC = if JZ != 0.0 {
                let KA = JY.exp();
                KA
            } else {
                let KB = (JP.exp()) * (C + (JY - JP));
                KB
            };
            let KC = JA * BN;
            let KD = if KC < JP { 1.0 } else { 0.0 };
            let YC = if KD != 0.0 {
                let KE = KC.exp();
                KE
            } else {
                let KF = (JP.exp()) * (C + (KC - JP));
                KF
            };
            let KG = JL * BN;
            let KH = if KG < JP { 1.0 } else { 0.0 };
            let VX = if KH != 0.0 {
                let KI = KG.exp();
                KI
            } else {
                let KJ = (JP.exp()) * (C + (KG - JP));
                KJ
            };
            let KK = JB * BN;
            let KL = if KK < JP { 1.0 } else { 0.0 };
            let VI = if KL != 0.0 {
                let KM = KK.exp();
                KM
            } else {
                let KN = (JP.exp()) * (C + (KK - JP));
                KN
            };
            let KO = JN * BN;
            let KP = if KO < JP { 1.0 } else { 0.0 };
            let WA = if KP != 0.0 {
                let KQ = KO.exp();
                KQ
            } else {
                let KR = (JP.exp()) * (C + (KO - JP));
                KR
            };
            let KS = JM * BN;
            let KT = if KS < JP { 1.0 } else { 0.0 };
            let VM = if KT != 0.0 {
                let KU = KS.exp();
                KU
            } else {
                let KV = (JP.exp()) * (C + (KS - JP));
                KV
            };
            let KX = (JL - KW) * BN;
            let KY = if KX < JP { 1.0 } else { 0.0 };
            let ADI = if KY != 0.0 {
                let KZ = KX.exp();
                KZ
            } else {
                let LA = (JP.exp()) * (C + (KX - JP));
                LA
            };
            let LB = if ((JK - KW) * BN) < JP { 1.0 } else { 0.0 };
            if LB != 0.0 {
            } else {
            }
            let LC = (IV - KW) * BN;
            let LD = if LC < JP { 1.0 } else { 0.0 };
            let LK = if LD != 0.0 {
                let LE = LC.exp();
                LE
            } else {
                let LF = (JP.exp()) * (C + (LC - JP));
                LF
            };
            let LG = (IT - KW) * BN;
            let LH = if LG < JP { 1.0 } else { 0.0 };
            let LM = if LH != 0.0 {
                let LI = LG.exp();
                LI
            } else {
                let LJ = (JP.exp()) * (C + (LG - JP));
                LJ
            };
            let LL = (C + (FY * LK)).sqrt();
            let LN = (C + (FY * LM)).sqrt();
            let LO = C + LN;
            let LP = (V * LM) / LO;
            let LR = if LP < LQ { 1.0 } else { 0.0 };
            let MY = if LR != 0.0 {
                LQ
            } else {
                LP
            };
            let LS = LL + C;
            let LT = BM * ((LL - LN) - ((LS / LO).ln()));
            let LU = (LT + JC) / EW;
            let LV = if LU > A { 1.0 } else { 0.0 };
            let OQ;
            let OX;
            let PD;
            let PN;
            let ZF;
            let ZT;
            if LV != 0.0 {
                let LX = if IT < LW { 1.0 } else { 0.0 };
                let MA = if LX != 0.0 {
                    IT
                } else {
                    let LY = LW + ((C + (IT - LW)).ln());
                    LY
                };
                let LZ = (FV * LU) * EW;
                let MB = (KW + ((V * BM) * (((LZ * BN) + C).ln()))) - MA;
                let MC = 2e-1f64 * KW;
                let MD = MC * MC;
                let ME = MB * MB;
                let MF = if MB < A { 1.0 } else { 0.0 };
                let MI = if MF != 0.0 {
                    let MG = (FV * MD) / (((ME + MD).sqrt()) - MB);
                    MG
                } else {
                    let MH = FV * (((ME + MD).sqrt()) + MB);
                    MH
                };
                let ML = MJ * MK;
                let MM = (MI * (MI + ML)) / (MK * (MI + (MJ * EW)));
                let MN = LU / MM;
                let MP = (MN - C) / MO;
                let MQ = if MN < C { 1.0 } else { 0.0 };
                let MT = if MQ != 0.0 {
                    let MR = C + (MO * ((C + (MP.exp())).ln()));
                    MR
                } else {
                    let MS = MN + (MO * ((C + ((-MP).exp())).ln()));
                    MS
                };
                let MU = MT / (C + (MO * ((C + ((-1e0f64 / MO).exp())).ln())));
                let MV = MI / ML;
                let MW = C + MV;
                let MX = (C + ((C + (((FY * MU) * MV) * MW)).sqrt())) / ((V * MU) * MW);
                let MZ = MY * MX;
                let NA = ((C - MX) + MZ) / (C + MZ);
                let NB = (LZ * NA) * BN;
                let NC = (V * NB) + (MY * ((MY + NB) + C));
                let ND = FV * (NB - C);
                let NE = (ND * ND) + NC;
                let NF = if NB >= C { 1.0 } else { 0.0 };
                let NI = if NF != 0.0 {
                    let NG = ND + (NE.sqrt());
                    NG
                } else {
                    let NH = NC / ((NE.sqrt()) - ND);
                    NH
                };
                let NK = if NI < NJ { 1.0 } else { 0.0 };
                let NL = if NK != 0.0 {
                    NJ
                } else {
                    NI
                };
                let NM = (NL * (NL + C)) * ((KW * BN).exp());
                let NN = (FV * MK) * (LU - MJ);
                let NO = NN + (((NN * NN) + (((MK * EW) * MJ) * LU)).sqrt());
                let NP = if parameters[73] == A { 1.0 } else { 0.0 };
                let OY = if NP != 0.0 {
                    let NQ = EB * AE;
                    NQ
                } else {
                    let NR = EB * (AE + ((V * LU) / (LU + MM)));
                    NR
                };
                let NS = MJ + LU;
                let NT = (MJ * LU) / NS;
                let NU = MJ / NS;
                OQ = NO;
                OX = OY;
                PD = NU;
                PN = NM;
                ZF = NA;
                ZT = NT;
            } else {
                let NV = (V * LK) / LS;
                let NX = if (if (JC.abs()) < (1e-5f64 * BM) { 1.0 } else { 0.0 }) != 0.0 || (if (LT.abs()) < ((1e-40f64 * BM) * (LL + LN)) { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let ZG = if NX != 0.0 {
                    let NY = FV * (NV + MY);
                    let NZ = NY / (NY + C);
                    NZ
                } else {
                    let OA = LT / ((LT + IV) - IT);
                    OA
                };
                let OB = AE * EB;
                let OC = C - (LU / MJ);
                OQ = JC;
                OX = OB;
                PD = OC;
                PN = NW;
                ZF = ZG;
                ZT = LU;
            }
            let OD = DT * (C - (CD.powf((-1e0f64 / W))));
            let OE = AE * DT;
            let OF = (IX - OD) / OE;
            let OG = if IX < OD { 1.0 } else { 0.0 };
            let OJ = if OG != 0.0 {
                let OH = IX - (OE * ((C + (OF.exp())).ln()));
                OH
            } else {
                let OI = OD - (OE * ((C + ((-OF).exp())).ln()));
                OI
            };
            let OK = C - W;
            let OL = (C - (OJ * DU)).powf(OK);
            let OM = ((DT / OK) * (C - OL)) + (CD * (IX - OJ));
            let OO = if ON == C { 1.0 } else { 0.0 };
            let OV;
            if OO != 0.0 {
                OV = IT;
            } else {
                let OP = if ON == V { 1.0 } else { 0.0 };
                let OW = if OP != 0.0 {
                    let OR = IT + OQ;
                    OR
                } else {
                    IV
                };
                OV = OW;
            }
            let OS = C - EF;
            let OT = (V - EF) / OS;
            let OU = EB * (C - (OT.powf((-1e0f64 / AN))));
            let OZ = (OV - OU) / OX;
            let PA = if OV < OU { 1.0 } else { 0.0 };
            let PH = if PA != 0.0 {
                let PB = OV - (OX * ((C + (OZ.exp())).ln()));
                PB
            } else {
                let PC = OU - (OX * ((C + ((-OZ).exp())).ln()));
                PC
            };
            let PE = PD.powf(parameters[76]);
            let PF = C - AN;
            let PG = EB / PF;
            let PI = (OS * ((PG * (C - (PE * ((C - (PH / EB)).powf(PF))))) + ((PE * OT) * (OV - PH)))) + (EF * IT);
            let PJ = (FY * GB) / GC;
            let PL = PJ * PK;
            let PM = PL / (C + ((C + PL).sqrt()));
            let PP = PN.powf((C / PO));
            let PQ = PJ * PP;
            let PR = PQ / (C + ((C + PQ).sqrt()));
            let PS = if IG == A { 1.0 } else { 0.0 };
            let PW = if PS != 0.0 {
                let PT = (C + (OM / HQ)) + (PI / HP);
                PT
            } else {
                let PU = ((((((OM / HQ) + C) * IH) * BN).exp()) - (((((-PI) / HP) * IH) * BN).exp())) / (((IH * BN).exp()) - C);
                PU
            };
            let PX = PW * PW;
            let PY = if PW < A { 1.0 } else { 0.0 };
            let QB = if PY != 0.0 {
                let PZ = 5.000000000000001e-3f64 / (((PX + PV).sqrt()) - PW);
                PZ
            } else {
                let QA = FV * (((PX + PV).sqrt()) + PW);
                QA
            };
            let QC = C + (FV * (PM + PR));
            let QD = QB * QC;
            let QE = (parameters[15] * GB) * PP;
            let QF = GB * PK;
            let QG = (QF - QE) / QD;
            let QI = IX / QH;
            let QJ = if IX < A { 1.0 } else { 0.0 };
            let QM = if QJ != 0.0 {
                let QK = QH * ((C + (QI.exp())).ln());
                QK
            } else {
                let QL = IX + (QH * ((C + ((-QI).exp())).ln()));
                QL
            };
            let QN = QM / parameters[156];
            let QO = if QN < JP { 1.0 } else { 0.0 };
            let QR = if QO != 0.0 {
                let QP = QN.exp();
                QP
            } else {
                let QQ = (JP.exp()) * (C + (QN - JP));
                QQ
            };
            let QS = HA * (QR - C);
            let QU = (IX - QT) / U;
            let QV = if IX < QT { 1.0 } else { 0.0 };
            let QY = if QV != 0.0 {
                let QW = IX - (U * ((C + (QU.exp())).ln()));
                QW
            } else {
                let QX = QT - (U * ((C + ((-QU).exp())).ln()));
                QX
            };
            let QZ = QT - QY;
            let RA = (parameters[159] * QY) * (QZ * QZ);
            let RB = JT / GM;
            let RC = if RB < JP { 1.0 } else { 0.0 };
            let RP = if RC != 0.0 {
                let RD = RB.exp();
                RD
            } else {
                let RE = (JP.exp()) * (C + (RB - JP));
                RE
            };
            let ACG;
            if GR != 0.0 {
                let RG = (IX - RF) * BN;
                let RH = if RG < JP { 1.0 } else { 0.0 };
                let RS = if RH != 0.0 {
                    let RI = RG.exp();
                    RI
                } else {
                    let RJ = (JP.exp()) * (C + (RG - JP));
                    RJ
                };
                let RK = (QG / GB) - 1e3f64;
                let RM = if RK < RL { 1.0 } else { 0.0 };
                let RU = if RM != 0.0 {
                    let RN = RK.exp();
                    RN
                } else {
                    let RO = 2.3538526683702e17f64 * (C + (RK - RL));
                    RO
                };
                let RQ = RP - C;
                let RV = ((GO * RQ) + ((((RR * V) * RQ) / (C + ((C + (FY * RS)).sqrt()))) * (C + (PI / HP)))) + (((RT * (PN - C)) * RU) / (C + RU));
                ACG = RV;
            } else {
                let RX = if RW == A { 1.0 } else { 0.0 };
                let ACH = if RX != 0.0 {
                    let RY = GO * (RP - C);
                    RY
                } else {
                    let RZ = GO * (((C - RW) * (RP - C)) + ((RW * ((RP + PN) - V)) * (C + (PI / HP))));
                    RZ
                };
                ACG = ACH;
            }
            let SA = IZ * BN;
            let SB = SA / GP;
            let SC = if SB < JP { 1.0 } else { 0.0 };
            let SJ = if SC != 0.0 {
                let SD = SB.exp();
                SD
            } else {
                let SE = (JP.exp()) * (C + (SB - JP));
                SE
            };
            let ACK;
            if GR != 0.0 {
                let SF = (IZ - RF) * BN;
                let SG = if SF < JP { 1.0 } else { 0.0 };
                let SM = if SG != 0.0 {
                    let SH = SF.exp();
                    SH
                } else {
                    let SI = (JP.exp()) * (C + (SF - JP));
                    SI
                };
                let SK = SJ - C;
                let SN = (GQ * SK) + (((SL * V) * SK) / (C + ((C + (FY * SM)).sqrt())));
                ACK = SN;
            } else {
                let SO = GQ * (SJ - C);
                ACK = SO;
            }
            let SP = JT / GG;
            let SQ = if SP < JP { 1.0 } else { 0.0 };
            let ST = if SQ != 0.0 {
                let SR = SP.exp();
                SR
            } else {
                let SS = (JP.exp()) * (C + (SP - JP));
                SS
            };
            let SU = GI * (ST - C);
            let SV = SA / GW;
            let SW = if SV < JP { 1.0 } else { 0.0 };
            let SZ = if SW != 0.0 {
                let SX = SV.exp();
                SX
            } else {
                let SY = (JP.exp()) * (C + (SV - JP));
                SY
            };
            let TA = GX * (SZ - C);
            let TB = JY / GJ;
            let TC = if TB < JP { 1.0 } else { 0.0 };
            let TF = if TC != 0.0 {
                let TD = TB.exp();
                TD
            } else {
                let TE = (JP.exp()) * (C + (TB - JP));
                TE
            };
            let TG = GK * (TF - C);
            let TH = SA / GY;
            let TI = if TH < JP { 1.0 } else { 0.0 };
            let TL = if TI != 0.0 {
                let TJ = TH.exp();
                TJ
            } else {
                let TK = (JP.exp()) * (C + (TH - JP));
                TK
            };
            let TM = GZ * (TL - C);
            let TN = if (if (if HG > A { 1.0 } else { 0.0 }) != 0.0 && (if HE > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && QJ != 0.0 { 1.0 } else { 0.0 };
            let ACJ;
            if TN != 0.0 {
                let TO = HF * (C - (X / (V * OL)));
                let TP = if TO < JP { 1.0 } else { 0.0 };
                let UH = if TP != 0.0 {
                    let TQ = TO.exp();
                    TQ
                } else {
                    let TR = (JP.exp()) * (C + (TO - JP));
                    TR
                };
                let TS = IX * DU;
                let TU = W - C;
                let TW = ((IX * X) * HF) / (HB * ((((((TS * TS) + TT).sqrt()).powf((-2e0f64 - W))) * ((W * ((C - (W * W)) - ((CD * TS) * TU))) - (((GF * TS) * TS) * (TU + TS)))) * TV));
                let TX = if TW < -1e-3f64 { 1.0 } else { 0.0 };
                let UG;
                if TX != 0.0 {
                    let TY = if TW < JP { 1.0 } else { 0.0 };
                    let UB = if TY != 0.0 {
                        let TZ = TW.exp();
                        TZ
                    } else {
                        let UA = (JP.exp()) * (C + (TW - JP));
                        UA
                    };
                    let UC = (-IX) * (C + ((C - UB) / TW));
                    UG = UC;
                } else {
                    let UF = ((IX * FV) * TW) * (C + ((TW * UD) * (C + (UE * TW))));
                    UG = UF;
                }
                let UI = (((((V * HH) * UG) * OL) * UH) * DU) * Y;
                ACJ = UI;
            } else {
                ACJ = A;
            }
            let UJ = if (if (if HM > A { 1.0 } else { 0.0 }) != 0.0 && (if HK > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if IT < A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let XI;
            if UJ != 0.0 {
                let UK = IT * DW;
                let UL = (C - UK).powf(PF);
                let UM = HL * (C - (AO / (V * UL)));
                let UN = if UM < JP { 1.0 } else { 0.0 };
                let VA = if UN != 0.0 {
                    let UO = UM.exp();
                    UO
                } else {
                    let UP = (JP.exp()) * (C + (UM - JP));
                    UP
                };
                let UQ = AN - C;
                let UR = ((IT * AO) * HL) / (HI * ((((((UK * UK) + TT).sqrt()).powf((-2e0f64 - AN))) * ((AN * ((C - (AN * AN)) - ((CD * UK) * UQ))) - (((GF * UK) * UK) * (UQ + UK)))) * TV));
                let US = if UR < -1e-3f64 { 1.0 } else { 0.0 };
                let UZ;
                if US != 0.0 {
                    let UT = if UR < JP { 1.0 } else { 0.0 };
                    let UW = if UT != 0.0 {
                        let UU = UR.exp();
                        UU
                    } else {
                        let UV = (JP.exp()) * (C + (UR - JP));
                        UV
                    };
                    let UX = (-IT) * (C + ((C - UW) / UR));
                    UZ = UX;
                } else {
                    let UY = ((IT * FV) * UR) * (C + ((UR * UD) * (C + (UE * UR))));
                    UZ = UY;
                }
                let VB = (((((V * HN) * UZ) * UL) * VA) * DW) * AP;
                XI = VB;
            } else {
                XI = A;
            }
            let VD = VC - C;
            let VE = (FY * GV) / GE;
            let VF = ((V * GV) * VD) / (C + ((C + (VE * VC)).sqrt()));
            let VG = if parameters[8] == C { 1.0 } else { 0.0 };
            let VV;
            let ACN;
            if VG != 0.0 {
                let VJ = FY * (HS / HT);
                let VL = (((VH * V) * HS) * (NW - VI)) / (C + ((C + (VJ * (NW + (VK * VI)))).sqrt()));
                let VN = ((((C - VH) * V) * HS) * (VC - VM)) / (C + ((C + (VJ * (VC + (VK * VM)))).sqrt()));
                VV = VN;
                ACN = VL;
            } else {
                let VO = FY * (HS / HT);
                let VP = (((VH * V) * HS) * (NW - C)) / (C + ((C + (VO * NW)).sqrt()));
                let VQ = ((((C - VH) * V) * HS) * VD) / (C + ((C + (VO * VC)).sqrt()));
                VV = VQ;
                ACN = VP;
            }
            let VS = if I > A { 1.0 } else { 0.0 };
            let VT = if (if VR > A { 1.0 } else { 0.0 }) != 0.0 && VS != 0.0 { 1.0 } else { 0.0 };
            let XM;
            let XP;
            let ACM;
            let ACO;
            let ADR;
            if VT != 0.0 {
                let VU = VF * J;
                let VW = VV * J;
                let VY = VX - C;
                let VZ = (((I * V) * GV) * VY) / (C + ((C + (VE * VX)).sqrt()));
                let WM = if VG != 0.0 {
                    let WB = (((((C - VH) * I) * V) * HS) * (VX - WA)) / (C + ((C + (((FY * HS) / HT) * (VX + (VK * WA)))).sqrt()));
                    WB
                } else {
                    let WC = (((((C - VH) * I) * V) * HS) * VY) / (C + ((C + (((FY * HS) / HT) * VX)).sqrt()));
                    WC
                };
                let WD = if VR == C { 1.0 } else { 0.0 };
                let WO;
                if WD != 0.0 {
                    let WE = (I * (GV + HS)) * EP;
                    let WF = JL - (BM * (V - ((WE * BN).ln())));
                    let WH = WF * WF;
                    let WI = if WF < A { 1.0 } else { 0.0 };
                    let WL = if WI != 0.0 {
                        let WJ = 6.05e-3f64 / (((WH + WG).sqrt()) - WF);
                        WJ
                    } else {
                        let WK = FV * (((WH + WG).sqrt()) + WF);
                        WK
                    };
                    let WN = WL / ((WE + ((VZ + WM) * EP)) + WL);
                    WO = WN;
                } else {
                    WO = C;
                }
                let WP = WO * VZ;
                let WQ = WO * WM;
                XM = VU;
                XP = WP;
                ACM = VW;
                ACO = WQ;
                ADR = WO;
            } else {
                XM = VF;
                XP = A;
                ACM = VV;
                ACO = A;
                ADR = C;
            }
            let WR = if parameters[84] == C { 1.0 } else { 0.0 };
            let XJ;
            if WR != 0.0 {
                let WS = JA + IT;
                let WV = ((-1e0f64 * WS) * -1e0f64) * WS;
                let WW = if (-1e0f64 * WS) < A { 1.0 } else { 0.0 };
                let XE = if WW != 0.0 {
                    let WX = 5e-13f64 / (((WV + WU).sqrt()) - (-1e0f64 * WS));
                    WX
                } else {
                    let WY = FV * (((WV + WU).sqrt()) + (-1e0f64 * WS));
                    WY
                };
                let XA = C / (C - (BA.powf(WZ)));
                let XC = BA * XB;
                let XD = (((XA * XA) * (BA.powf((WZ - C)))) * WZ) / XB;
                let XF = if XE < XC { 1.0 } else { 0.0 };
                let XK = if XF != 0.0 {
                    let XG = C / (C - ((XE / XB).powf(WZ)));
                    XG
                } else {
                    let XH = XA + ((XE - XC) * XD);
                    XH
                };
                XJ = XK;
            } else {
                XJ = C;
            }
            let XL = XI * XJ;
            let XN = XM * XJ;
            let XO = TG * XJ;
            let XQ = XP * XJ;
            let XR = (C + (OM / HQ)) + (PI / HP);
            let XT = XR * XR;
            let XU = if XR < A { 1.0 } else { 0.0 };
            let XX = if XU != 0.0 {
                let XV = 5.000000000000001e-3f64 / (((XT + XS).sqrt()) - XR);
                XV
            } else {
                let XW = FV * (((XT + XS).sqrt()) + XR);
                XW
            };
            let XY = EL / (XX * QC);
            let XZ = if XY < R { 1.0 } else { 0.0 };
            let YA = if XZ != 0.0 {
                R
            } else {
                XY
            };
            let YB = CD * YA;
            let YD = if QG > A { 1.0 } else { 0.0 };
            let ABW;
            if YD != 0.0 {
                let YF = if YE == C { 1.0 } else { 0.0 };
                let AAZ;
                if YF != 0.0 {
                    let YH = if IT < YG { 1.0 } else { 0.0 };
                    let ABA;
                    if YH != 0.0 {
                        let YI = (-QG) / parameters[42];
                        let YJ = if YI < JP { 1.0 } else { 0.0 };
                        let YM = if YJ != 0.0 {
                            let YK = YI.exp();
                            YK
                        } else {
                            let YL = (JP.exp()) * (C + (YI - JP));
                            YL
                        };
                        let YN = (YG - IT) * YM;
                        let YQ = (-YO) * (YN.powf(YP));
                        let YR = if YQ < JP { 1.0 } else { 0.0 };
                        let YV = if YR != 0.0 {
                            let YS = YQ.exp();
                            YS
                        } else {
                            let YT = (JP.exp()) * (C + (YQ - JP));
                            YT
                        };
                        let YW = ((YU / YO) * YN) * YV;
                        ABA = YW;
                    } else {
                        ABA = A;
                    }
                    AAZ = ABA;
                } else {
                    let YX = if YE == V { 1.0 } else { 0.0 };
                    let ABB;
                    if YX != 0.0 {
                        let YY = if IT < KW { 1.0 } else { 0.0 };
                        let ABC;
                        if YY != 0.0 {
                            let ZA = (V * parameters[46]) / (YZ * YZ);
                            let ZB = KW - IT;
                            let ZC = ((V * (ZB / PD)) / ZA).sqrt();
                            let ZE = if ZD == A { 1.0 } else { 0.0 };
                            let ZJ = if ZE != 0.0 {
                                YZ
                            } else {
                                let ZH = C - (FV * ZF);
                                let ZI = (YZ * ZH) * ZH;
                                ZI
                            };
                            let ZK = (ZC * ZJ) / (((ZC * ZC) + (ZJ * ZJ)).sqrt());
                            let ZL = ZB / ZK;
                            let ZM = FV * ZK;
                            let ZN = ZM * ZA;
                            let ZO = ZL + (ZN * PD);
                            let ZV = if ZE != 0.0 {
                                ZO
                            } else {
                                let ZQ = V * ZP;
                                let ZR = ZL - (ZN * (((C + ZP) / (C + ZQ)) - (QG / (MJ * (C + (ZQ * (C + (V * ZF))))))));
                                let ZS = ZR - ZO;
                                let ZU = FV * ((ZR + ZO) + (((ZS * ZS) + ((((AE * ZL) * ZL) * ZT) / MJ)).sqrt()));
                                ZU
                            };
                            let ZW = (ZV - ZL) / ZV;
                            let ZX = if (ZW.abs()) > 1e-7f64 { 1.0 } else { 0.0 };
                            let ABD = if ZX != 0.0 {
                                let ZY = ZM / ZW;
                                let AAB = (-AAA) / ZV;
                                let AAC = (((ZZ / AAA) * ZV) * ZY) * ((AAB.exp()) - ((AAB * (C + (ZJ / ZY))).exp()));
                                AAC
                            } else {
                                let AAD = (ZZ * ZJ) * (((-AAA) / ZV).exp());
                                AAD
                            };
                            ABC = ABD;
                        } else {
                            ABC = A;
                        }
                        ABB = ABC;
                    } else {
                        let AAE = if YE == CD { 1.0 } else { 0.0 };
                        let ABE;
                        if AAE != 0.0 {
                            let AAF = if IT < YG { 1.0 } else { 0.0 };
                            let ABF;
                            if AAF != 0.0 {
                                let AAG = YG - IT;
                                let AAI = (AAG.powf(YP)) * ((C - (QG / (AAH + QG))).powf(parameters[49]));
                                let AAJ = if ZD == A { 1.0 } else { 0.0 };
                                let AAS;
                                if AAJ != 0.0 {
                                    AAS = AAI;
                                } else {
                                    let AAK = (QG - parameters[52]) / AAH;
                                    let AAM = (AAK - C) / AAL;
                                    let AAN = if AAK < C { 1.0 } else { 0.0 };
                                    let AAQ = if AAN != 0.0 {
                                        let AAO = C + (AAL * ((C + (AAM.exp())).ln()));
                                        AAO
                                    } else {
                                        let AAP = AAK + (AAL * ((C + ((-AAM).exp())).ln()));
                                        AAP
                                    };
                                    let AAR = AAI * (AAQ.powf(parameters[50]));
                                    AAS = AAR;
                                }
                                let AAT = (-YO) * AAS;
                                let AAU = if AAT < JP { 1.0 } else { 0.0 };
                                let AAX = if AAU != 0.0 {
                                    let AAV = AAT.exp();
                                    AAV
                                } else {
                                    let AAW = (JP.exp()) * (C + (AAT - JP));
                                    AAW
                                };
                                let AAY = ((YU / YO) * AAG) * AAX;
                                ABF = AAY;
                            } else {
                                ABF = A;
                            }
                            ABE = ABF;
                        } else {
                            ABE = A;
                        }
                        ABB = ABE;
                    }
                    AAZ = ABB;
                }
                let ABG = if AAZ > A { 1.0 } else { 0.0 };
                let ABX;
                if ABG != 0.0 {
                    let ABH = if parameters[53] == C { 1.0 } else { 0.0 };
                    let ABY;
                    if ABH != 0.0 {
                        let ABJ = ABI + YB;
                        let ABL = ((BM / (QG * ABJ)) + ((QD / GB) * GO)) + (ABK / ABJ);
                        let ABM = if YE == CD { 1.0 } else { 0.0 };
                        let ABZ;
                        if ABM != 0.0 {
                            let ABN = (AAZ - ABL) / WT;
                            let ABO = if AAZ < ABL { 1.0 } else { 0.0 };
                            let ABR = if ABO != 0.0 {
                                let ABP = AAZ - (WT * ((C + (ABN.exp())).ln()));
                                ABP
                            } else {
                                let ABQ = ABL - (WT * ((C + ((-ABN).exp())).ln()));
                                ABQ
                            };
                            let ABS = QG * ABR;
                            ABZ = ABS;
                        } else {
                            let ABT = ((QG * AAZ) * ABL) / (AAZ + ABL);
                            ABZ = ABT;
                        }
                        ABY = ABZ;
                    } else {
                        let ABU = QG * AAZ;
                        ABY = ABU;
                    }
                    ABX = ABY;
                } else {
                    ABX = A;
                }
                ABW = ABX;
            } else {
                ABW = A;
            }
            let ABV = if PN > A { 1.0 } else { 0.0 };
            if ABV != 0.0 {
            } else {
            }
            if GR != 0.0 {
            } else {
            }
            let ACI = ACG + SU;
            let ACL = (ACK + TA) + TM;
            let ACP = if IZ < OD { 1.0 } else { 0.0 };
            if ACP != 0.0 {
            } else {
            }
            let ACQ = HV * GC;
            let ACR = AE * EB;
            let ACS = if JK < OU { 1.0 } else { 0.0 };
            if ACS != 0.0 {
            } else {
            }
            let ACT = C - parameters[77];
            let ACU = (JL - OU) / ACR;
            let ACV = if JL < OU { 1.0 } else { 0.0 };
            let ACY = if ACV != 0.0 {
                let ACW = JL - (ACR * ((C + (ACU.exp())).ln()));
                ACW
            } else {
                let ACX = OU - (ACR * ((C + ((-ACU).exp())).ln()));
                ACX
            };
            let ACZ = ((EE * ((OS * ((PG * (C - ((C - (ACY / EB)).powf(PF)))) + (OT * (JL - ACY)))) + (EF * JL))) * ACT) * I;
            let ADA = if JB < (DZ * (C - (V.powf((-1e0f64 / parameters[139]))))) { 1.0 } else { 0.0 };
            if ADA != 0.0 {
            } else {
            }
            let ADB = if (IX / (parameters[85] * BM)) < JP { 1.0 } else { 0.0 };
            if ADB != 0.0 {
            } else {
            }
            let ADC = ((FY * HX) * BM) / EW;
            let ADD = if parameters[79] == A { 1.0 } else { 0.0 };
            if ADD != 0.0 {
            } else {
                let ADF = if (((JK - ADE) / parameters[91]) * BN) < JP { 1.0 } else { 0.0 };
                if ADF != 0.0 {
                } else {
                }
            }
            let ADG = if (if (if VR == C { 1.0 } else { 0.0 }) != 0.0 || (if VR == CD { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && VS != 0.0 { 1.0 } else { 0.0 };
            let AEH;
            if ADG != 0.0 {
                let ADS;
                if ADD != 0.0 {
                    let ADH = PJ * VX;
                    let ADJ = FY * ADI;
                    let ADK = (((FV * I) * HZ) * ((ACQ * ((ADH - PJ) / (C + ((C + ADH).sqrt())))) + (ADC * (ADJ / (C + ((C + ADJ).sqrt())))))) / HY;
                    ADS = ADK;
                } else {
                    let ADL = (JL - ADE) * BN;
                    let ADM = if ADL < JP { 1.0 } else { 0.0 };
                    let ADP = if ADM != 0.0 {
                        let ADN = ADL.exp();
                        ADN
                    } else {
                        let ADO = (JP.exp()) * (C + (ADL - JP));
                        ADO
                    };
                    let ADQ = ((((V * I) * GV) * IA) * VX) / (C + ((C + (FY * ADP)).sqrt()));
                    ADS = ADQ;
                }
                let ADT = ADR * ADS;
                AEH = ADT;
            } else {
                AEH = A;
            }
            let ADU = if parameters[6] == C { 1.0 } else { 0.0 };
            if ADU != 0.0 {
                let ADV = if OF < A { 1.0 } else { 0.0 };
                if ADV != 0.0 {
                } else {
                }
            } else {
            }
            if GR != 0.0 {
            } else {
            }
            let ADW = (B * ACO) * Q;
            let ADX = ((B * JE) / ABI) * Q;
            let ADY = C - parameters[148];
            let ADZ = if parameters[146] > R { 1.0 } else { 0.0 };
            if ADZ != 0.0 {
                let AEA = if parameters[145] == A { 1.0 } else { 0.0 };
                if AEA != 0.0 {
                } else {
                    let AEB = if (ADY.abs()) < WT { 1.0 } else { 0.0 };
                    if AEB != 0.0 {
                    } else {
                    }
                }
            } else {
            }
            let AEC = (B * parameters[69]) * JF;
            let AED = 0e0f64 * Q;
            let AEE = (B * parameters[78]) * JG;
            let AEF = 0e0f64 * Q;
            let AEG = (B * XQ) * Q;
            let AEI = B * (ACZ + AEH);
            let AEJ = 0e0f64 * Q;
            if IL != 0.0 {
            } else {
            }
            if IO != 0.0 {
            } else {
            }
            let AEK = 5.5224904e-23f64 * BJ;
            let AEL = AEK / ABK;
            let AEM = AEK / ABI;
            let AEN = AEK * ACA;
            let AEO = AEK * ACC;
            let AEP = AEK * ACE;
            let AEQ = ((AEK / YB) * ((FY * YC) + 5e0f64)) * UD;
            let AER = (QF + QE) / QD;
            let AES = 3.2043836e-19f64 * (AER.abs());
            let AET = if parameters[130] > A { 1.0 } else { 0.0 };
            let AEV = if AET != 0.0 {
                let AEU = (ABW / AER).abs();
                AEU
            } else {
                A
            };
            let AEW = (3.2043836e-19f64 * ABW) * (AEV + C);
            let AEX = if AER > A { 1.0 } else { 0.0 };
            if AEX != 0.0 {
            } else {
            }
            let AEZ = if AEY == C { 1.0 } else { 0.0 };
            if AEZ != 0.0 {
            } else {
                let AFA = if AEY == V { 1.0 } else { 0.0 };
                if AFA != 0.0 {
                } else {
                }
            }
            let AFB = 3.2043836e-19f64 * ((((ACI - ACJ) + RA) + QS).abs());
            let AFC = ACG + ACK;
            let AFF = AFD * ((AFC.abs()).powf(AFE));
            let AFG = if AFC < A { 1.0 } else { 0.0 };
            let AGP = if AFG != 0.0 {
                let AFH = -AFF;
                AFH
            } else {
                AFF
            };
            let AFI = (SU + TA) + TM;
            let AFJ = parameters[129] * ((AFI.abs()).powf(parameters[127]));
            let AFK = if AFI < A { 1.0 } else { 0.0 };
            let AGR = if AFK != 0.0 {
                let AFL = -AFJ;
                AFL
            } else {
                AFJ
            };
            let AFM = 3.2043836e-19f64 * (ACL.abs());
            let AFN = XO.abs();
            let AFO = 3.2043836e-19f64 * AFN;
            let AFP = AFD * (AFN.powf(AFE));
            let AFQ = if XO < A { 1.0 } else { 0.0 };
            let AGV = if AFQ != 0.0 {
                let AFR = -AFP;
                AFR
            } else {
                AFP
            };
            let AFS = 3.2043836e-19f64 * (XL.abs());
            let AFT = XN.abs();
            let AFU = 3.2043836e-19f64 * AFT;
            let AFV = C - (VR * I);
            let AFW = (AFD * AFV) * ((AFT / AFV).powf(AFE));
            let AFX = if XN < A { 1.0 } else { 0.0 };
            let AGY = if AFX != 0.0 {
                let AFY = -AFW;
                AFY
            } else {
                AFW
            };
            let AFZ = XQ.abs();
            let AGA = (3.2043836e-19f64 * AFZ) * VR;
            let AGB = if I == A { 1.0 } else { 0.0 };
            let AGE = if AGB != 0.0 {
                A
            } else {
                let AGC = ((AFD * VR) * I) * ((AFZ / I).powf(AFE));
                AGC
            };
            let AGD = if XQ < A { 1.0 } else { 0.0 };
            let AHB = if AGD != 0.0 {
                let AGF = -AGE;
                AGF
            } else {
                AGE
            };
            let AGG = 3.2043836e-19f64 * (ACN.abs());
            let AGH = 3.2043836e-19f64 * (ACM.abs());
            let AGI = 3.2043836e-19f64 * (ACO.abs());
            let AGJ = AES * Q;
            let AGK = AEW * Q;
            let AGL = AFB * Q;
            let AGM = AEL * Q;
            let AGN = AEM * Q;
            let AGO = AEQ * Q;
            let AGQ = AGP * Q;
            let AGS = AGR * Q;
            let AGT = AFM * Q;
            let AGU = AFO * Q;
            let AGW = AGV * Q;
            let AGX = AFU * Q;
            let AGZ = AGY * Q;
            let AHA = AGA * Q;
            let AHC = AHB * Q;
            let AHS;
            let AHT;
            let AHU;
            let AHV;
            if GR != 0.0 {
                let AHD = AFS * Q;
                AHS = C;
                AHT = AHD;
                AHU = A;
                AHV = A;
            } else {
                let AHE = AFS * Q;
                AHS = A;
                AHT = A;
                AHU = C;
                AHV = AHE;
            }
            let AHF = AGG * Q;
            let AHG = AGH * Q;
            let AHH = AGI * Q;
            let AHW;
            let AHY;
            let AIA;
            let AIC;
            let AIE;
            let AIG;
            let AII;
            let AIK;
            let AIM;
            let AIO;
            let AIQ;
            let AIS;
            let AIU;
            let AIW;
            let AIY;
            let AJA;
            if IL != 0.0 {
                let AHX;
                let AHZ;
                let AIB;
                let AID;
                let AIF;
                let AIH;
                let AIJ;
                let AIL;
                let AIN;
                let AIP;
                if IO != 0.0 {
                    let AHI = AEN * Q;
                    let AHJ = AEO * Q;
                    let AHK = AEP * Q;
                    AHX = C;
                    AHZ = AHI;
                    AIB = C;
                    AID = AHJ;
                    AIF = C;
                    AIH = AHK;
                    AIJ = A;
                    AIL = A;
                    AIN = A;
                    AIP = A;
                } else {
                    let AHL = AEN * Q;
                    let AHM = AEO * Q;
                    AHX = A;
                    AHZ = A;
                    AIB = A;
                    AID = A;
                    AIF = A;
                    AIH = A;
                    AIJ = C;
                    AIL = AHL;
                    AIN = C;
                    AIP = AHM;
                }
                AHW = AHX;
                AHY = AHZ;
                AIA = AIB;
                AIC = AID;
                AIE = AIF;
                AIG = AIH;
                AII = AIJ;
                AIK = AIL;
                AIM = AIN;
                AIO = AIP;
                AIQ = A;
                AIS = A;
                AIU = A;
                AIW = A;
                AIY = A;
                AJA = A;
            } else {
                let AIR;
                let AIT;
                let AIV;
                let AIX;
                let AIZ;
                let AJB;
                if IO != 0.0 {
                    let AHN = AEN * Q;
                    let AHO = AEP * Q;
                    AIR = C;
                    AIT = AHN;
                    AIV = C;
                    AIX = AHO;
                    AIZ = A;
                    AJB = A;
                } else {
                    let AHP = AEN * Q;
                    AIR = A;
                    AIT = A;
                    AIV = A;
                    AIX = A;
                    AIZ = C;
                    AJB = AHP;
                }
                AHW = A;
                AHY = A;
                AIA = A;
                AIC = A;
                AIE = A;
                AIG = A;
                AII = A;
                AIK = A;
                AIM = A;
                AIO = A;
                AIQ = AIR;
                AIS = AIT;
                AIU = AIV;
                AIW = AIX;
                AIY = AIZ;
                AJA = AJB;
            }
            let AHQ = if (((((ADW + ADX) + AED) + AEF) + AEG) + AEJ) == A { 1.0 } else { 0.0 };
            if AHQ != 0.0 {
            } else {
            }
            let AHR = if Q != C { 1.0 } else { 0.0 };
            if AHR != 0.0 {
            } else {
            }
        {
            let psd = AGJ;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 0, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 0, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(0, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = AGK;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(1, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = AGL;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 2, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 2, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(2, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = AGM;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 3, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 3, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(3, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = AGN;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 4, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 4, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(4, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = AGO;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 5, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 5, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(5, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = AGQ;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 6, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = Some(C);
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 6, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 6, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(6, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = AGS;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 7, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = Some(C);
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 7, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 7, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(7, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = AGT;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 8, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 8, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(8, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = AGU;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 9, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 9, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(9, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = AGW;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 10, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = Some(C);
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 10, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 10, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(10, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = AGX;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 11, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 11, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(11, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = AGZ;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 12, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = Some(C);
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 12, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 12, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(12, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = AHA;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 13, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 13, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(13, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = AHC;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 14, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = Some(C);
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 14, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 14, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(14, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if AHS == 0.0 {
            if !visitor.visit(15, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = AHT;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 15, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 15, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(15, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if AHU == 0.0 {
            if !visitor.visit(16, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = AHV;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 16, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 16, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(16, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = AHF;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 17, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 17, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(17, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = AHG;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 18, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 18, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(18, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = AHH;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 19, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 19, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(19, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if AHW == 0.0 {
            if !visitor.visit(20, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = AHY;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 20, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 20, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(20, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if AIA == 0.0 {
            if !visitor.visit(21, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = AIC;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 21, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 21, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(21, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if AIE == 0.0 {
            if !visitor.visit(22, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = AIG;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 22, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 22, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(22, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if AII == 0.0 {
            if !visitor.visit(23, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = AIK;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 23, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 23, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(23, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if AIM == 0.0 {
            if !visitor.visit(24, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = AIO;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 24, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 24, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(24, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if AIQ == 0.0 {
            if !visitor.visit(25, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = AIS;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 25, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 25, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(25, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if AIU == 0.0 {
            if !visitor.visit(26, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = AIW;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 26, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 26, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(26, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if AIY == 0.0 {
            if !visitor.visit(27, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = AJA;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 27, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 27, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(27, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        Ok(())
    }
}
