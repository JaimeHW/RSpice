#![allow(dead_code, non_snake_case, unused_parens, unused_variables)]

use super::state::Instance;
use rspice_veriloga_runtime::GeneratedEvalContext;
pub use rspice_veriloga_runtime::{GeneratedNoiseDescriptor, GeneratedNoiseEndpoint, GeneratedNoiseEvaluation, GeneratedNoiseEvaluationError, GeneratedNoiseEvaluationRef, GeneratedNoiseKind, GeneratedNoiseVisitor};

pub static NOISE_SOURCES: [GeneratedNoiseDescriptor; 28] = [
    GeneratedNoiseDescriptor { mechanism: "WHITE_NOI_GND_IN", label: Some("in"), kind: GeneratedNoiseKind::White, equation: 30, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(11), name: "noi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: None, name: "0", is_internal: false }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C2_B2_IAVL", label: Some("iavl"), kind: GeneratedNoiseKind::White, equation: 35, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(8), name: "c2", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(6), name: "b2", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_B2_E1_IB2E1", label: Some("ib2e1"), kind: GeneratedNoiseKind::White, equation: 36, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(6), name: "b2", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(4), name: "e1", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_E_E1_RE", label: Some("re"), kind: GeneratedNoiseKind::White, equation: 37, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(2), name: "e", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(4), name: "e1", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_B_B1_RBC", label: Some("rbc"), kind: GeneratedNoiseKind::White, equation: 38, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(1), name: "b", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(5), name: "b1", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_B1_B2_RBV", label: Some("rbv"), kind: GeneratedNoiseKind::White, equation: 39, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "b1", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(6), name: "b2", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_B2_E1_IB2E1_F", label: Some("ib2e1_f"), kind: GeneratedNoiseKind::Flicker, equation: 40, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(6), name: "b2", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(4), name: "e1", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_B1_E1_IB1E1_F", label: Some("ib1e1_f"), kind: GeneratedNoiseKind::Flicker, equation: 41, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "b1", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(4), name: "e1", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_B1_E1_IB1E1", label: Some("ib1e1"), kind: GeneratedNoiseKind::White, equation: 42, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "b1", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(4), name: "e1", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_B1_C4_IB3", label: Some("ib3"), kind: GeneratedNoiseKind::White, equation: 43, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "b1", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(10), name: "c4", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_B1_C4_IB3_F", label: Some("ib3_f"), kind: GeneratedNoiseKind::Flicker, equation: 44, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "b1", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(10), name: "c4", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_B1_C4_IEX", label: Some("iex"), kind: GeneratedNoiseKind::White, equation: 45, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "b1", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(10), name: "c4", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_B1_C4_IEX_F", label: Some("iex_f"), kind: GeneratedNoiseKind::Flicker, equation: 46, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "b1", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(10), name: "c4", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_B_C3_XIEX", label: Some("xiex"), kind: GeneratedNoiseKind::White, equation: 47, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(1), name: "b", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(9), name: "c3", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_B_C3_XIEX_F", label: Some("xiex_f"), kind: GeneratedNoiseKind::Flicker, equation: 48, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(1), name: "b", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(9), name: "c3", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C1_B2_IZTCB", label: Some("iztcb"), kind: GeneratedNoiseKind::White, equation: 49, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(7), name: "c1", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(6), name: "b2", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C2_B2_IZTCB", label: Some("iztcb"), kind: GeneratedNoiseKind::White, equation: 50, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(8), name: "c2", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(6), name: "b2", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_B2_S_ISUB_INT", label: Some("isub_int"), kind: GeneratedNoiseKind::White, equation: 51, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(6), name: "b2", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(3), name: "s", is_internal: false }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_B1_S_ISUB", label: Some("isub"), kind: GeneratedNoiseKind::White, equation: 52, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "b1", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(3), name: "s", is_internal: false }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_B_S_XISUB", label: Some("xisub"), kind: GeneratedNoiseKind::White, equation: 53, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(1), name: "b", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(3), name: "s", is_internal: false }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C_C3_RCC", label: Some("rcc"), kind: GeneratedNoiseKind::White, equation: 54, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(0), name: "c", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(9), name: "c3", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C3_C4_RCBLX", label: Some("rcblx"), kind: GeneratedNoiseKind::White, equation: 55, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(9), name: "c3", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(10), name: "c4", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C4_C1_RCBLI", label: Some("rcbli"), kind: GeneratedNoiseKind::White, equation: 56, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(10), name: "c4", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(7), name: "c1", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C_C3_RCC", label: Some("rcc"), kind: GeneratedNoiseKind::White, equation: 57, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(0), name: "c", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(9), name: "c3", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C3_C1_RCBLX", label: Some("rcblx"), kind: GeneratedNoiseKind::White, equation: 58, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(9), name: "c3", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(7), name: "c1", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C_C4_RCC", label: Some("rcc"), kind: GeneratedNoiseKind::White, equation: 59, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(0), name: "c", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(10), name: "c4", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C4_C1_RCBLI", label: Some("rcbli"), kind: GeneratedNoiseKind::White, equation: 60, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(10), name: "c4", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(7), name: "c1", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C_C1_RCC", label: Some("rcc"), kind: GeneratedNoiseKind::White, equation: 61, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(0), name: "c", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(7), name: "c1", is_internal: true }, table_len: 0, table_log_interp: false },
];

impl Instance {
    pub fn evaluate_noise_sources(&self, ctx: &GeneratedEvalContext<'_>, visitor: &mut dyn GeneratedNoiseVisitor) -> Result<(), GeneratedNoiseEvaluationError> {
        if !self.multiplicity.is_finite() || self.multiplicity <= 0.0 {
            return Err(GeneratedNoiseEvaluationError::InvalidMultiplicity { value: self.multiplicity });
        }
        let parameters = &self.params.values;
        let temperature = ctx.temperature();
        let node_potentials = [ctx.node_voltage(self.nodes[0]), ctx.node_voltage(self.nodes[1]), ctx.node_voltage(self.nodes[2]), ctx.node_voltage(self.nodes[3]), ctx.node_voltage(self.nodes[4]), ctx.node_voltage(self.nodes[5]), ctx.node_voltage(self.nodes[6]), ctx.node_voltage(self.nodes[7]), ctx.node_voltage(self.nodes[8]), ctx.node_voltage(self.nodes[9]), ctx.node_voltage(self.nodes[10]), ctx.node_voltage(self.nodes[11])];
            let A = 0e0f64;
            let B = parameters[3];
            let C = 1e0f64;
            let E = 7.03e7f64;
            let F = 1.23e8f64;
            let G = 1.58e8f64;
            let H = 2.04e8f64;
            let I = parameters[33];
            let M = parameters[150];
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
            let BC = 8.617086918058125e-5f64;
            let BU = 3e0f64;
            let BW = parameters[105];
            let CC = parameters[110];
            let DE = parameters[140];
            let DR = parameters[75];
            let DX = parameters[97];
            let EA = parameters[98];
            let EB = parameters[96];
            let EF = parameters[57];
            let EH = parameters[58];
            let EK = parameters[59];
            let EM = parameters[99];
            let EO = parameters[122];
            let EQ = parameters[10];
            let EY = parameters[123];
            let FA = parameters[11];
            let FJ = 1e-6f64;
            let FM = 5e-1f64;
            let FP = 4e0f64;
            let FQ = parameters[121];
            let FU = parameters[103];
            let FW = 6e0f64;
            let FX = parameters[21];
            let GA = parameters[32];
            let GD = parameters[17];
            let GG = parameters[19];
            let GN = parameters[23];
            let GP = parameters[146];
            let GV = parameters[35];
            let GX = parameters[34];
            let HB = parameters[37];
            let HD = parameters[36];
            let HI = parameters[141];
            let HL = parameters[87];
            let HN = parameters[88];
            let HX = parameters[92];
            let II = node_potentials[6];
            let IJ = node_potentials[7];
            let IL = node_potentials[8];
            let IN = node_potentials[4];
            let IP = node_potentials[5];
            let IU = node_potentials[1];
            let IY = node_potentials[10];
            let JG = parameters[147];
            let LH = parameters[149];
            let LN = 1e2f64;
            let MA = parameters[62];
            let MB = parameters[61];
            let MF = parameters[63];
            let NA = parameters[148];
            let OE = parameters[74];
            let PM = 1.0000000000000002e-2f64;
            let PY = 1e-4f64;
            let QK = parameters[154];
            let RC = 4e1f64;
            let RN = parameters[93];
            let TK = 1e-30f64;
            let TM = 1.6666666666666666e-1f64;
            let TU = 3.333333333333333e-1f64;
            let TV = 2.5e-1f64;
            let UY = parameters[143];
            let VB = parameters[144];
            let VI = parameters[5];
            let VX = 1.21e-2f64;
            let WK = 1e-6f64;
            let WL = 1e-12f64;
            let WQ = parameters[82];
            let WS = parameters[81];
            let XJ = 1.0000000000000002e-2f64;
            let XV = parameters[39];
            let XX = parameters[44];
            let YG = parameters[41];
            let YL = parameters[40];
            let YQ = parameters[45];
            let YU = parameters[7];
            let ZG = parameters[47];
            let ZY = parameters[48];
            let AAC = parameters[51];
            let AEL = parameters[131];
            let AEQ = parameters[128];
            let AER = parameters[126];
            let D = if B == C { 1.0 } else { 0.0 };
            let HU;
            let ZQ;
            if D != 0.0 {
                HU = F;
                ZQ = E;
            } else {
                HU = H;
                ZQ = G;
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
            let BI = if AG != 0.0 {
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
            let BO = if AV != 0.0 {
                let AW = AD + (AE * ((C + (AU.exp())).ln()));
                AW
            } else {
                let AX = AT + (AE * ((C + ((-AU).exp())).ln()));
                AX
            };
            let AY = C / AQ;
            let AZ = C / AM;
            let BA = C - (C / parameters[83]);
            let BB = L / K;
            let BD = BC * L;
            let BE = C / BD;
            let BF = BE - (C / (BC * K));
            let BG = L - K;
            let BH = BB.ln();
            let BJ = BI - (((AA * L) * L) / (L + AB));
            let BK = (BJ - AD) / AE;
            let BL = if BJ < AD { 1.0 } else { 0.0 };
            let GS = if BL != 0.0 {
                let BM = AD + (AE * ((C + (BK.exp())).ln()));
                BM
            } else {
                let BN = BJ + (AE * ((C + ((-BK).exp())).ln()));
                BN
            };
            let BP = BO - (((AR * L) * L) / (L + AS));
            let BQ = (BP - AD) / AE;
            let BR = if BP < AD { 1.0 } else { 0.0 };
            let GZ = if BR != 0.0 {
                let BS = AD + (AE * ((C + (BQ.exp())).ln()));
                BS
            } else {
                let BT = BP + (AE * ((C + ((-BQ).exp())).ln()));
                BT
            };
            let BV = C - BB;
            let BX = (((-3e0f64 * BD) * BH) + (AK * BB)) + (BV * BW);
            let BY = (AD - BX) / BD;
            let BZ = if AD < BX { 1.0 } else { 0.0 };
            let DK = if BZ != 0.0 {
                let CA = BX + (BD * ((C + (BY.exp())).ln()));
                CA
            } else {
                let CB = AD + (BD * ((C + ((-BY).exp())).ln()));
                CB
            };
            let CD = BV * CC;
            let CE = (((-3e0f64 * BD) * BH) + (parameters[64] * BB)) + CD;
            let CF = (AD - CE) / BD;
            let CG = if AD < CE { 1.0 } else { 0.0 };
            let KN = if CG != 0.0 {
                let CH = CE + (BD * ((C + (CF.exp())).ln()));
                CH
            } else {
                let CI = AD + (BD * ((C + ((-CF).exp())).ln()));
                CI
            };
            let CJ = (((-3e0f64 * BD) * BH) + (parameters[80] * BB)) + CD;
            let CK = (AD - CJ) / BD;
            let CL = if AD < CJ { 1.0 } else { 0.0 };
            let ACC = if CL != 0.0 {
                let CM = CJ + (BD * ((C + (CK.exp())).ln()));
                CM
            } else {
                let CN = AD + (BD * ((C + ((-CK).exp())).ln()));
                CN
            };
            let CO = AM * BB;
            let CP = (((-3e0f64 * BD) * BH) + CO) + CD;
            let CQ = (AD - CP) / BD;
            let CR = if AD < CP { 1.0 } else { 0.0 };
            let DS = if CR != 0.0 {
                let CS = CP + (BD * ((C + (CQ.exp())).ln()));
                CS
            } else {
                let CT = AD + (BD * ((C + ((-CQ).exp())).ln()));
                CT
            };
            let CU = (((-3e0f64 * BD) * BH) + CO) + CD;
            let CV = (AD - CU) / BD;
            let CW = if AD < CU { 1.0 } else { 0.0 };
            let DM = if CW != 0.0 {
                let CX = CU + (BD * ((C + (CV.exp())).ln()));
                CX
            } else {
                let CY = AD + (BD * ((C + ((-CV).exp())).ln()));
                CY
            };
            let CZ = (((-3e0f64 * BD) * BH) + (parameters[27] * BB)) + (BV * parameters[109]);
            let DA = (AD - CZ) / BD;
            let DB = if AD < CZ { 1.0 } else { 0.0 };
            let QW = if DB != 0.0 {
                let DC = CZ + (BD * ((C + (DA.exp())).ln()));
                DC
            } else {
                let DD = AD + (BD * ((C + ((-DA).exp())).ln()));
                DD
            };
            let DF = (((-3e0f64 * BD) * BH) + (parameters[138] * BB)) + (BV * DE);
            let DG = (AD - DF) / BD;
            let DH = if AD < DF { 1.0 } else { 0.0 };
            let DQ = if DH != 0.0 {
                let DI = DF + (BD * ((C + (DG.exp())).ln()));
                DI
            } else {
                let DJ = AD + (BD * ((C + ((-DG).exp())).ln()));
                DJ
            };
            let DL = C / DK;
            let DN = C / DM;
            let DO = (AK * DL).powf(W);
            let DP = (AM * DN).powf(AN);
            let DT = ((C - DR) * ((AM / DS).powf(AN))) + DR;
            let DU = C / DT;
            let DV = parameters[70] * DT;
            let DW = DR * DU;
            let DY = parameters[54] * ((BH * DX).exp());
            let DZ = if DY < R { 1.0 } else { 0.0 };
            let ABB = if DZ != 0.0 {
                R
            } else {
                DY
            };
            let EC = parameters[56] * ((BH * (EA - EB)).exp());
            let ED = parameters[55] * ((BH * parameters[101]).exp());
            let EE = if ED < R { 1.0 } else { 0.0 };
            let AAZ = if EE != 0.0 {
                R
            } else {
                ED
            };
            let EG = EF * ((BH * parameters[102]).exp());
            let EI = (BH * parameters[104]).exp();
            let EJ = EH * EI;
            let EL = EK * EI;
            let EN = parameters[60] * ((BH * EM).exp());
            let EP = if EO != A { 1.0 } else { 0.0 };
            let FR;
            if EP != 0.0 {
                let ER = EQ * (C + (BG * EO));
                let ES = (ER - C) / U;
                let ET = if ER < C { 1.0 } else { 0.0 };
                let EW = if ET != 0.0 {
                    let EU = C + (U * ((C + (ES.exp())).ln()));
                    EU
                } else {
                    let EV = ER + (U * ((C + ((-ES).exp())).ln()));
                    EV
                };
                let EX = EW - 6.931471805599453e-4f64;
                FR = EX;
            } else {
                FR = EQ;
            }
            let EZ = if EY != A { 1.0 } else { 0.0 };
            let PF;
            if EZ != 0.0 {
                let FB = FA * (C + (BG * EY));
                let FC = (FB - C) / U;
                let FD = if FB < C { 1.0 } else { 0.0 };
                let FG = if FD != 0.0 {
                    let FE = C + (U * ((C + (FC.exp())).ln()));
                    FE
                } else {
                    let FF = FB + (U * ((C + ((-FC).exp())).ln()));
                    FF
                };
                let FH = FG - 6.931471805599453e-4f64;
                PF = FH;
            } else {
                PF = FA;
            }
            let FI = parameters[43] * (C + (parameters[124] * BG));
            let FK = FI * FI;
            let FL = if FI < A { 1.0 } else { 0.0 };
            let YF = if FL != 0.0 {
                let FN = 5e-7f64 / (((FK + FJ).sqrt()) - FI);
                FN
            } else {
                let FO = FM * (((FK + FJ).sqrt()) + FI);
                FO
            };
            let FS = (parameters[9] * (((BH * (((FP - EA) - EB) + FQ)) / FR).exp())) * ((((-BW) * BF) / FR).exp());
            let FT = parameters[12] * ((BH * (C - EA)).exp());
            let FV = parameters[30] * ((BH * (C - FU)).exp());
            let FY = (-parameters[113]) * BF;
            let FZ = (parameters[20] * ((BH * (FW - (V * FX))).exp())) * ((FY / FX).exp());
            let GB = (parameters[31] * ((BH * (FW - (V * GA))).exp())) * ((((-CC) * BF) / GA).exp());
            let GC = BH * ((FP - DX) + FQ);
            let GE = (-parameters[111]) * BF;
            let GF = (parameters[16] * ((GC / GD).exp())) * ((GE / GD).exp());
            let GH = (parameters[18] * ((GC / GG).exp())) * ((GE / GG).exp());
            let GI = if parameters[24] == C { 1.0 } else { 0.0 };
            let RI;
            let RK;
            let SC;
            if GI != 0.0 {
                let GJ = parameters[25] * ((((-parameters[107]) * BF) / GD).exp());
                let GK = parameters[28] * (((-parameters[106]) * BF).exp());
                let GL = parameters[26] * ((((-parameters[108]) * BF) / GG).exp());
                RI = GJ;
                RK = GK;
                SC = GL;
            } else {
                RI = A;
                RK = A;
                SC = A;
            }
            let GM = (parameters[29] * ((BH * ((FP - FU) + FQ)).exp())) * (((-parameters[112]) * BF).exp());
            let GO = (parameters[22] * ((BH * (FW - (V * GN))).exp())) * ((FY / GN).exp());
            let GQ = (parameters[145] * ((BH * (FP / GP)).exp())) * ((FY / GP).exp());
            let GR = (parameters[151] * (BB.sqrt())) * ((parameters[153] * BG).exp());
            let GT = (GS * AJ).powf(-5e-1f64);
            let GU = C / DO;
            let GW = (((((((GV * GS) * GS) * GT) * GU) * AK) * DL) * AJ) * AJ;
            let GY = ((((((GX * GT) * DK) * DK) * AL) * AL) * DO) * ((GV - GW).exp());
            let HA = (GZ * AY).powf(-5e-1f64);
            let HC = (((((((HB * GZ) * GZ) * HA) * (C / DP)) * AM) * DN) * AY) * AY;
            let HE = ((((((HD * HA) * DM) * DM) * AZ) * AZ) * DP) * ((HB - HC).exp());
            let HF = (BH * EB).exp();
            let HG = (parameters[14] * HF) * DU;
            let HH = (parameters[13] * HF) * GU;
            let HJ = (parameters[133] * ((BH * (FP - HI)).exp())) * (((-DE) * BF).exp());
            let HK = parameters[135] * ((BH * (C - HI)).exp());
            let HM = HL * ((BH * ((EB + EA) - C)).exp());
            let HO = HN * ((BH * (EM - C)).exp());
            let HP = HM + HO;
            let HQ = (parameters[89] * HP) / (HL + HN);
            let HR = parameters[90] * ((BH * (parameters[100] - C)).exp());
            let HS = L - 3e2f64;
            let HT = if L < 5.25e2f64 { 1.0 } else { 0.0 };
            let ZR = if HT != 0.0 {
                let HV = HU * ((C + (7.2e-4f64 * HS)) - ((1.6e-6f64 * HS) * HS));
                HV
            } else {
                let HW = HU * 1.081e0f64;
                HW
            };
            let HY = HX * HF;
            let HZ = if EF > A { 1.0 } else { 0.0 };
            let ADO;
            if HZ != 0.0 {
                let IA = C / EG;
                let IB = if IA > S { 1.0 } else { 0.0 };
                let ADP = if IB != 0.0 {
                    S
                } else {
                    IA
                };
                ADO = ADP;
            } else {
                ADO = A;
            }
            let IC = if EH > A { 1.0 } else { 0.0 };
            let ADT;
            if IC != 0.0 {
                let ID = C / EJ;
                let IE = if ID > S { 1.0 } else { 0.0 };
                let ADU = if IE != 0.0 {
                    S
                } else {
                    ID
                };
                ADT = ADU;
            } else {
                ADT = A;
            }
            let IF = if EK > A { 1.0 } else { 0.0 };
            let ADV;
            if IF != 0.0 {
                let IG = C / EL;
                let IH = if IG > S { 1.0 } else { 0.0 };
                let ADW = if IH != 0.0 {
                    S
                } else {
                    IG
                };
                ADV = ADW;
            } else {
                ADV = A;
            }
            let IK = B * (II - IJ);
            let IM = B * (II - IL);
            let IO = B * (II - IN);
            let IQ = B * (IP - IN);
            let IR = B * (IP - II);
            let IS = B * (node_potentials[3] - IJ);
            let IT = B * (IJ - IL);
            let IV = B * (IU - IP);
            let IW = B * (IU - node_potentials[2]);
            let IX = B * (IU - node_potentials[0]);
            let IZ = B * (IY - IJ);
            let JA = B * (node_potentials[9] - IY);
            let JB = ((IR + IM) - IT) - IZ;
            let JC = IX + ((((-IX) + IV) + JB) - JA);
            let JD = IS - IZ;
            let JE = JD - JA;
            let JF = IM * BE;
            let JH = if JF < JG { 1.0 } else { 0.0 };
            let NN = if JH != 0.0 {
                let JI = JF.exp();
                JI
            } else {
                let JJ = (JG.exp()) * (C + (JF - JG));
                JJ
            };
            let JK = IO * BE;
            let JL = JK / FR;
            let JM = if JL < JG { 1.0 } else { 0.0 };
            let PB = if JM != 0.0 {
                let JN = JL.exp();
                JN
            } else {
                let JO = (JG.exp()) * (C + (JL - JG));
                JO
            };
            let JP = JB * BE;
            let JQ = if JP < JG { 1.0 } else { 0.0 };
            let UT = if JQ != 0.0 {
                let JR = JP.exp();
                JR
            } else {
                let JS = (JG.exp()) * (C + (JP - JG));
                JS
            };
            let JT = IR * BE;
            let JU = if JT < JG { 1.0 } else { 0.0 };
            let XT = if JU != 0.0 {
                let JV = JT.exp();
                JV
            } else {
                let JW = (JG.exp()) * (C + (JT - JG));
                JW
            };
            let JX = JC * BE;
            let JY = if JX < JG { 1.0 } else { 0.0 };
            let VO = if JY != 0.0 {
                let JZ = JX.exp();
                JZ
            } else {
                let KA = (JG.exp()) * (C + (JX - JG));
                KA
            };
            let KB = IS * BE;
            let KC = if KB < JG { 1.0 } else { 0.0 };
            let UZ = if KC != 0.0 {
                let KD = KB.exp();
                KD
            } else {
                let KE = (JG.exp()) * (C + (KB - JG));
                KE
            };
            let KF = JE * BE;
            let KG = if KF < JG { 1.0 } else { 0.0 };
            let VR = if KG != 0.0 {
                let KH = KF.exp();
                KH
            } else {
                let KI = (JG.exp()) * (C + (KF - JG));
                KI
            };
            let KJ = JD * BE;
            let KK = if KJ < JG { 1.0 } else { 0.0 };
            let VD = if KK != 0.0 {
                let KL = KJ.exp();
                KL
            } else {
                let KM = (JG.exp()) * (C + (KJ - JG));
                KM
            };
            let KO = (JC - KN) * BE;
            let KP = if KO < JG { 1.0 } else { 0.0 };
            let ACG = if KP != 0.0 {
                let KQ = KO.exp();
                KQ
            } else {
                let KR = (JG.exp()) * (C + (KO - JG));
                KR
            };
            let KS = if ((JB - KN) * BE) < JG { 1.0 } else { 0.0 };
            if KS != 0.0 {
            } else {
            }
            let KT = (IM - KN) * BE;
            let KU = if KT < JG { 1.0 } else { 0.0 };
            let LB = if KU != 0.0 {
                let KV = KT.exp();
                KV
            } else {
                let KW = (JG.exp()) * (C + (KT - JG));
                KW
            };
            let KX = (IK - KN) * BE;
            let KY = if KX < JG { 1.0 } else { 0.0 };
            let LD = if KY != 0.0 {
                let KZ = KX.exp();
                KZ
            } else {
                let LA = (JG.exp()) * (C + (KX - JG));
                LA
            };
            let LC = (C + (FP * LB)).sqrt();
            let LE = (C + (FP * LD)).sqrt();
            let LF = C + LE;
            let LG = (V * LD) / LF;
            let LI = if LG < LH { 1.0 } else { 0.0 };
            let MP = if LI != 0.0 {
                LH
            } else {
                LG
            };
            let LJ = LC + C;
            let LK = BD * ((LC - LE) - ((LJ / LF).ln()));
            let LL = (LK + IT) / EN;
            let LM = if LL > A { 1.0 } else { 0.0 };
            let OH;
            let OO;
            let OU;
            let PE;
            let YW;
            let ZK;
            if LM != 0.0 {
                let LO = if IK < LN { 1.0 } else { 0.0 };
                let LR = if LO != 0.0 {
                    IK
                } else {
                    let LP = LN + ((C + (IK - LN)).ln());
                    LP
                };
                let LQ = (FM * LL) * EN;
                let LS = (KN + ((V * BD) * (((LQ * BE) + C).ln()))) - LR;
                let LT = 2e-1f64 * KN;
                let LU = LT * LT;
                let LV = LS * LS;
                let LW = if LS < A { 1.0 } else { 0.0 };
                let LZ = if LW != 0.0 {
                    let LX = (FM * LU) / (((LV + LU).sqrt()) - LS);
                    LX
                } else {
                    let LY = FM * (((LV + LU).sqrt()) + LS);
                    LY
                };
                let MC = MA * MB;
                let MD = (LZ * (LZ + MC)) / (MB * (LZ + (MA * EN)));
                let ME = LL / MD;
                let MG = (ME - C) / MF;
                let MH = if ME < C { 1.0 } else { 0.0 };
                let MK = if MH != 0.0 {
                    let MI = C + (MF * ((C + (MG.exp())).ln()));
                    MI
                } else {
                    let MJ = ME + (MF * ((C + ((-MG).exp())).ln()));
                    MJ
                };
                let ML = MK / (C + (MF * ((C + ((-1e0f64 / MF).exp())).ln())));
                let MM = LZ / MC;
                let MN = C + MM;
                let MO = (C + ((C + (((FP * ML) * MM) * MN)).sqrt())) / ((V * ML) * MN);
                let MQ = MP * MO;
                let MR = ((C - MO) + MQ) / (C + MQ);
                let MS = (LQ * MR) * BE;
                let MT = (V * MS) + (MP * ((MP + MS) + C));
                let MU = FM * (MS - C);
                let MV = (MU * MU) + MT;
                let MW = if MS >= C { 1.0 } else { 0.0 };
                let MZ = if MW != 0.0 {
                    let MX = MU + (MV.sqrt());
                    MX
                } else {
                    let MY = MT / ((MV.sqrt()) - MU);
                    MY
                };
                let NB = if MZ < NA { 1.0 } else { 0.0 };
                let NC = if NB != 0.0 {
                    NA
                } else {
                    MZ
                };
                let ND = (NC * (NC + C)) * ((KN * BE).exp());
                let NE = (FM * MB) * (LL - MA);
                let NF = NE + (((NE * NE) + (((MB * EN) * MA) * LL)).sqrt());
                let NG = if parameters[73] == A { 1.0 } else { 0.0 };
                let OP = if NG != 0.0 {
                    let NH = DS * AE;
                    NH
                } else {
                    let NI = DS * (AE + ((V * LL) / (LL + MD)));
                    NI
                };
                let NJ = MA + LL;
                let NK = (MA * LL) / NJ;
                let NL = MA / NJ;
                OH = NF;
                OO = OP;
                OU = NL;
                PE = ND;
                YW = MR;
                ZK = NK;
            } else {
                let NM = (V * LB) / LJ;
                let NO = if (if (IT.abs()) < (1e-5f64 * BD) { 1.0 } else { 0.0 }) != 0.0 || (if (LK.abs()) < ((1e-40f64 * BD) * (LC + LE)) { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let YX = if NO != 0.0 {
                    let NP = FM * (NM + MP);
                    let NQ = NP / (NP + C);
                    NQ
                } else {
                    let NR = LK / ((LK + IM) - IK);
                    NR
                };
                let NS = AE * DS;
                let NT = C - (LL / MA);
                OH = IT;
                OO = NS;
                OU = NT;
                PE = NN;
                YW = YX;
                ZK = LL;
            }
            let NU = DK * (C - (BU.powf((-1e0f64 / W))));
            let NV = AE * DK;
            let NW = (IO - NU) / NV;
            let NX = if IO < NU { 1.0 } else { 0.0 };
            let OA = if NX != 0.0 {
                let NY = IO - (NV * ((C + (NW.exp())).ln()));
                NY
            } else {
                let NZ = NU - (NV * ((C + ((-NW).exp())).ln()));
                NZ
            };
            let OB = C - W;
            let OC = (C - (OA * DL)).powf(OB);
            let OD = ((DK / OB) * (C - OC)) + (BU * (IO - OA));
            let OF = if OE == C { 1.0 } else { 0.0 };
            let OM;
            if OF != 0.0 {
                OM = IK;
            } else {
                let OG = if OE == V { 1.0 } else { 0.0 };
                let ON = if OG != 0.0 {
                    let OI = IK + OH;
                    OI
                } else {
                    IM
                };
                OM = ON;
            }
            let OJ = C - DW;
            let OK = (V - DW) / OJ;
            let OL = DS * (C - (OK.powf((-1e0f64 / AN))));
            let OQ = (OM - OL) / OO;
            let OR = if OM < OL { 1.0 } else { 0.0 };
            let OY = if OR != 0.0 {
                let OS = OM - (OO * ((C + (OQ.exp())).ln()));
                OS
            } else {
                let OT = OL - (OO * ((C + ((-OQ).exp())).ln()));
                OT
            };
            let OV = OU.powf(parameters[76]);
            let OW = C - AN;
            let OX = DS / OW;
            let OZ = (OJ * ((OX * (C - (OV * ((C - (OY / DS)).powf(OW))))) + ((OV * OK) * (OM - OY)))) + (DW * IK);
            let PA = (FP * FS) / FT;
            let PC = PA * PB;
            let PD = PC / (C + ((C + PC).sqrt()));
            let PG = PE.powf((C / PF));
            let PH = PA * PG;
            let PI = PH / (C + ((C + PH).sqrt()));
            let PJ = if HX == A { 1.0 } else { 0.0 };
            let PN = if PJ != 0.0 {
                let PK = (C + (OD / HH)) + (OZ / HG);
                PK
            } else {
                let PL = ((((((OD / HH) + C) * HY) * BE).exp()) - (((((-OZ) / HG) * HY) * BE).exp())) / (((HY * BE).exp()) - C);
                PL
            };
            let PO = PN * PN;
            let PP = if PN < A { 1.0 } else { 0.0 };
            let PS = if PP != 0.0 {
                let PQ = 5.000000000000001e-3f64 / (((PO + PM).sqrt()) - PN);
                PQ
            } else {
                let PR = FM * (((PO + PM).sqrt()) + PN);
                PR
            };
            let PT = C + (FM * (PD + PI));
            let PU = PS * PT;
            let PV = (parameters[15] * FS) * PG;
            let PW = FS * PB;
            let PX = (PW - PV) / PU;
            let PZ = IO / PY;
            let QA = if IO < A { 1.0 } else { 0.0 };
            let QD = if QA != 0.0 {
                let QB = PY * ((C + (PZ.exp())).ln());
                QB
            } else {
                let QC = IO + (PY * ((C + ((-PZ).exp())).ln()));
                QC
            };
            let QE = QD / parameters[152];
            let QF = if QE < JG { 1.0 } else { 0.0 };
            let QI = if QF != 0.0 {
                let QG = QE.exp();
                QG
            } else {
                let QH = (JG.exp()) * (C + (QE - JG));
                QH
            };
            let QJ = GR * (QI - C);
            let QL = (IO - QK) / U;
            let QM = if IO < QK { 1.0 } else { 0.0 };
            let QP = if QM != 0.0 {
                let QN = IO - (U * ((C + (QL.exp())).ln()));
                QN
            } else {
                let QO = QK - (U * ((C + ((-QL).exp())).ln()));
                QO
            };
            let QQ = QK - QP;
            let QR = (parameters[155] * QP) * (QQ * QQ);
            let QS = JK / GD;
            let QT = if QS < JG { 1.0 } else { 0.0 };
            let RG = if QT != 0.0 {
                let QU = QS.exp();
                QU
            } else {
                let QV = (JG.exp()) * (C + (QS - JG));
                QV
            };
            let ACW;
            if GI != 0.0 {
                let QX = (IO - QW) * BE;
                let QY = if QX < JG { 1.0 } else { 0.0 };
                let RJ = if QY != 0.0 {
                    let QZ = QX.exp();
                    QZ
                } else {
                    let RA = (JG.exp()) * (C + (QX - JG));
                    RA
                };
                let RB = (PX / FS) - 1e3f64;
                let RD = if RB < RC { 1.0 } else { 0.0 };
                let RL = if RD != 0.0 {
                    let RE = RB.exp();
                    RE
                } else {
                    let RF = 2.3538526683702e17f64 * (C + (RB - RC));
                    RF
                };
                let RH = RG - C;
                let RM = ((GF * RH) + ((((RI * V) * RH) / (C + ((C + (FP * RJ)).sqrt()))) * (C + (OZ / HG)))) + (((RK * (PE - C)) * RL) / (C + RL));
                ACW = RM;
            } else {
                let RO = if RN == A { 1.0 } else { 0.0 };
                let ACX = if RO != 0.0 {
                    let RP = GF * (RG - C);
                    RP
                } else {
                    let RQ = GF * (((C - RN) * (RG - C)) + ((RN * ((RG + PE) - V)) * (C + (OZ / HG))));
                    RQ
                };
                ACW = ACX;
            }
            let RR = IQ * BE;
            let RS = RR / GG;
            let RT = if RS < JG { 1.0 } else { 0.0 };
            let SA = if RT != 0.0 {
                let RU = RS.exp();
                RU
            } else {
                let RV = (JG.exp()) * (C + (RS - JG));
                RV
            };
            let ACU;
            if GI != 0.0 {
                let RW = (IQ - QW) * BE;
                let RX = if RW < JG { 1.0 } else { 0.0 };
                let SD = if RX != 0.0 {
                    let RY = RW.exp();
                    RY
                } else {
                    let RZ = (JG.exp()) * (C + (RW - JG));
                    RZ
                };
                let SB = SA - C;
                let SE = (GH * SB) + (((SC * V) * SB) / (C + ((C + (FP * SD)).sqrt())));
                ACU = SE;
            } else {
                let SF = GH * (SA - C);
                ACU = SF;
            }
            let SG = JK / FX;
            let SH = if SG < JG { 1.0 } else { 0.0 };
            let SK = if SH != 0.0 {
                let SI = SG.exp();
                SI
            } else {
                let SJ = (JG.exp()) * (C + (SG - JG));
                SJ
            };
            let SL = FZ * (SK - C);
            let SM = RR / GN;
            let SN = if SM < JG { 1.0 } else { 0.0 };
            let SQ = if SN != 0.0 {
                let SO = SM.exp();
                SO
            } else {
                let SP = (JG.exp()) * (C + (SM - JG));
                SP
            };
            let SR = GO * (SQ - C);
            let SS = JP / GA;
            let ST = if SS < JG { 1.0 } else { 0.0 };
            let SW = if ST != 0.0 {
                let SU = SS.exp();
                SU
            } else {
                let SV = (JG.exp()) * (C + (SS - JG));
                SV
            };
            let SX = GB * (SW - C);
            let SY = RR / GP;
            let SZ = if SY < JG { 1.0 } else { 0.0 };
            let TC = if SZ != 0.0 {
                let TA = SY.exp();
                TA
            } else {
                let TB = (JG.exp()) * (C + (SY - JG));
                TB
            };
            let TD = GQ * (TC - C);
            let TE = if (if (if GX > A { 1.0 } else { 0.0 }) != 0.0 && (if GV > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && QA != 0.0 { 1.0 } else { 0.0 };
            let ACZ;
            if TE != 0.0 {
                let TF = GW * (C - (X / (V * OC)));
                let TG = if TF < JG { 1.0 } else { 0.0 };
                let TY = if TG != 0.0 {
                    let TH = TF.exp();
                    TH
                } else {
                    let TI = (JG.exp()) * (C + (TF - JG));
                    TI
                };
                let TJ = IO * DL;
                let TL = W - C;
                let TN = ((IO * X) * GW) / (GS * ((((((TJ * TJ) + TK).sqrt()).powf((-2e0f64 - W))) * ((W * ((C - (W * W)) - ((BU * TJ) * TL))) - (((FW * TJ) * TJ) * (TL + TJ)))) * TM));
                let TO = if TN < -1e-3f64 { 1.0 } else { 0.0 };
                let TX;
                if TO != 0.0 {
                    let TP = if TN < JG { 1.0 } else { 0.0 };
                    let TS = if TP != 0.0 {
                        let TQ = TN.exp();
                        TQ
                    } else {
                        let TR = (JG.exp()) * (C + (TN - JG));
                        TR
                    };
                    let TT = (-IO) * (C + ((C - TS) / TN));
                    TX = TT;
                } else {
                    let TW = ((IO * FM) * TN) * (C + ((TN * TU) * (C + (TV * TN))));
                    TX = TW;
                }
                let TZ = (((((V * GY) * TX) * OC) * TY) * DL) * Y;
                ACZ = TZ;
            } else {
                ACZ = A;
            }
            let UA = if (if (if HD > A { 1.0 } else { 0.0 }) != 0.0 && (if HB > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if IK < A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let WZ;
            if UA != 0.0 {
                let UB = IK * DN;
                let UC = (C - UB).powf(OW);
                let UD = HC * (C - (AO / (V * UC)));
                let UE = if UD < JG { 1.0 } else { 0.0 };
                let UR = if UE != 0.0 {
                    let UF = UD.exp();
                    UF
                } else {
                    let UG = (JG.exp()) * (C + (UD - JG));
                    UG
                };
                let UH = AN - C;
                let UI = ((IK * AO) * HC) / (GZ * ((((((UB * UB) + TK).sqrt()).powf((-2e0f64 - AN))) * ((AN * ((C - (AN * AN)) - ((BU * UB) * UH))) - (((FW * UB) * UB) * (UH + UB)))) * TM));
                let UJ = if UI < -1e-3f64 { 1.0 } else { 0.0 };
                let UQ;
                if UJ != 0.0 {
                    let UK = if UI < JG { 1.0 } else { 0.0 };
                    let UN = if UK != 0.0 {
                        let UL = UI.exp();
                        UL
                    } else {
                        let UM = (JG.exp()) * (C + (UI - JG));
                        UM
                    };
                    let UO = (-IK) * (C + ((C - UN) / UI));
                    UQ = UO;
                } else {
                    let UP = ((IK * FM) * UI) * (C + ((UI * TU) * (C + (TV * UI))));
                    UQ = UP;
                }
                let US = (((((V * HE) * UQ) * UC) * UR) * DN) * AP;
                WZ = US;
            } else {
                WZ = A;
            }
            let UU = UT - C;
            let UV = (FP * GM) / FV;
            let UW = ((V * GM) * UU) / (C + ((C + (UV * UT)).sqrt()));
            let UX = if parameters[8] == C { 1.0 } else { 0.0 };
            let VM;
            let ADB;
            if UX != 0.0 {
                let VA = FP * (HJ / HK);
                let VC = (((UY * V) * HJ) * (NN - UZ)) / (C + ((C + (VA * (NN + (VB * UZ)))).sqrt()));
                let VE = ((((C - UY) * V) * HJ) * (UT - VD)) / (C + ((C + (VA * (UT + (VB * VD)))).sqrt()));
                VM = VE;
                ADB = VC;
            } else {
                let VF = FP * (HJ / HK);
                let VG = (((UY * V) * HJ) * (NN - C)) / (C + ((C + (VF * NN)).sqrt()));
                let VH = ((((C - UY) * V) * HJ) * UU) / (C + ((C + (VF * UT)).sqrt()));
                VM = VH;
                ADB = VG;
            }
            let VJ = if I > A { 1.0 } else { 0.0 };
            let VK = if (if VI > A { 1.0 } else { 0.0 }) != 0.0 && VJ != 0.0 { 1.0 } else { 0.0 };
            let XD;
            let XG;
            let ACP;
            let ADA;
            let ADC;
            if VK != 0.0 {
                let VL = UW * J;
                let VN = VM * J;
                let VP = VO - C;
                let VQ = (((I * V) * GM) * VP) / (C + ((C + (UV * VO)).sqrt()));
                let WD = if UX != 0.0 {
                    let VS = (((((C - UY) * I) * V) * HJ) * (VO - VR)) / (C + ((C + (((FP * HJ) / HK) * (VO + (VB * VR)))).sqrt()));
                    VS
                } else {
                    let VT = (((((C - UY) * I) * V) * HJ) * VP) / (C + ((C + (((FP * HJ) / HK) * VO)).sqrt()));
                    VT
                };
                let VU = if VI == C { 1.0 } else { 0.0 };
                let WF;
                if VU != 0.0 {
                    let VV = (I * (GM + HJ)) * EG;
                    let VW = JC - (BD * (V - ((VV * BE).ln())));
                    let VY = VW * VW;
                    let VZ = if VW < A { 1.0 } else { 0.0 };
                    let WC = if VZ != 0.0 {
                        let WA = 6.05e-3f64 / (((VY + VX).sqrt()) - VW);
                        WA
                    } else {
                        let WB = FM * (((VY + VX).sqrt()) + VW);
                        WB
                    };
                    let WE = WC / ((VV + ((VQ + WD) * EG)) + WC);
                    WF = WE;
                } else {
                    WF = C;
                }
                let WG = WF * VQ;
                let WH = WF * WD;
                XD = VL;
                XG = WG;
                ACP = WF;
                ADA = VN;
                ADC = WH;
            } else {
                XD = UW;
                XG = A;
                ACP = C;
                ADA = VM;
                ADC = A;
            }
            let WI = if parameters[84] == C { 1.0 } else { 0.0 };
            let XA;
            if WI != 0.0 {
                let WJ = IR + IK;
                let WM = ((-1e0f64 * WJ) * -1e0f64) * WJ;
                let WN = if (-1e0f64 * WJ) < A { 1.0 } else { 0.0 };
                let WV = if WN != 0.0 {
                    let WO = 5e-13f64 / (((WM + WL).sqrt()) - (-1e0f64 * WJ));
                    WO
                } else {
                    let WP = FM * (((WM + WL).sqrt()) + (-1e0f64 * WJ));
                    WP
                };
                let WR = C / (C - (BA.powf(WQ)));
                let WT = BA * WS;
                let WU = (((WR * WR) * (BA.powf((WQ - C)))) * WQ) / WS;
                let WW = if WV < WT { 1.0 } else { 0.0 };
                let XB = if WW != 0.0 {
                    let WX = C / (C - ((WV / WS).powf(WQ)));
                    WX
                } else {
                    let WY = WR + ((WV - WT) * WU);
                    WY
                };
                XA = XB;
            } else {
                XA = C;
            }
            let XC = WZ * XA;
            let XE = XD * XA;
            let XF = SX * XA;
            let XH = XG * XA;
            let XI = (C + (OD / HH)) + (OZ / HG);
            let XK = XI * XI;
            let XL = if XI < A { 1.0 } else { 0.0 };
            let XO = if XL != 0.0 {
                let XM = 5.000000000000001e-3f64 / (((XK + XJ).sqrt()) - XI);
                XM
            } else {
                let XN = FM * (((XK + XJ).sqrt()) + XI);
                XN
            };
            let XP = EC / (XO * PT);
            let XQ = if XP < R { 1.0 } else { 0.0 };
            let XR = if XQ != 0.0 {
                R
            } else {
                XP
            };
            let XS = BU * XR;
            let XU = if PX > A { 1.0 } else { 0.0 };
            let ADE;
            if XU != 0.0 {
                let XW = if XV == C { 1.0 } else { 0.0 };
                let AAQ;
                if XW != 0.0 {
                    let XY = if IK < XX { 1.0 } else { 0.0 };
                    let AAR;
                    if XY != 0.0 {
                        let XZ = (-PX) / parameters[42];
                        let YA = if XZ < JG { 1.0 } else { 0.0 };
                        let YD = if YA != 0.0 {
                            let YB = XZ.exp();
                            YB
                        } else {
                            let YC = (JG.exp()) * (C + (XZ - JG));
                            YC
                        };
                        let YE = (XX - IK) * YD;
                        let YH = (-YF) * (YE.powf(YG));
                        let YI = if YH < JG { 1.0 } else { 0.0 };
                        let YM = if YI != 0.0 {
                            let YJ = YH.exp();
                            YJ
                        } else {
                            let YK = (JG.exp()) * (C + (YH - JG));
                            YK
                        };
                        let YN = ((YL / YF) * YE) * YM;
                        AAR = YN;
                    } else {
                        AAR = A;
                    }
                    AAQ = AAR;
                } else {
                    let YO = if XV == V { 1.0 } else { 0.0 };
                    let AAS;
                    if YO != 0.0 {
                        let YP = if IK < KN { 1.0 } else { 0.0 };
                        let AAT;
                        if YP != 0.0 {
                            let YR = (V * parameters[46]) / (YQ * YQ);
                            let YS = KN - IK;
                            let YT = ((V * (YS / OU)) / YR).sqrt();
                            let YV = if YU == A { 1.0 } else { 0.0 };
                            let ZA = if YV != 0.0 {
                                YQ
                            } else {
                                let YY = C - (FM * YW);
                                let YZ = (YQ * YY) * YY;
                                YZ
                            };
                            let ZB = (YT * ZA) / (((YT * YT) + (ZA * ZA)).sqrt());
                            let ZC = YS / ZB;
                            let ZD = FM * ZB;
                            let ZE = ZD * YR;
                            let ZF = ZC + (ZE * OU);
                            let ZM = if YV != 0.0 {
                                ZF
                            } else {
                                let ZH = V * ZG;
                                let ZI = ZC - (ZE * (((C + ZG) / (C + ZH)) - (PX / (MA * (C + (ZH * (C + (V * YW))))))));
                                let ZJ = ZI - ZF;
                                let ZL = FM * ((ZI + ZF) + (((ZJ * ZJ) + ((((AE * ZC) * ZC) * ZK) / MA)).sqrt()));
                                ZL
                            };
                            let ZN = (ZM - ZC) / ZM;
                            let ZO = if (ZN.abs()) > 1e-7f64 { 1.0 } else { 0.0 };
                            let AAU = if ZO != 0.0 {
                                let ZP = ZD / ZN;
                                let ZS = (-ZR) / ZM;
                                let ZT = (((ZQ / ZR) * ZM) * ZP) * ((ZS.exp()) - ((ZS * (C + (ZA / ZP))).exp()));
                                ZT
                            } else {
                                let ZU = (ZQ * ZA) * (((-ZR) / ZM).exp());
                                ZU
                            };
                            AAT = AAU;
                        } else {
                            AAT = A;
                        }
                        AAS = AAT;
                    } else {
                        let ZV = if XV == BU { 1.0 } else { 0.0 };
                        let AAV;
                        if ZV != 0.0 {
                            let ZW = if IK < XX { 1.0 } else { 0.0 };
                            let AAW;
                            if ZW != 0.0 {
                                let ZX = XX - IK;
                                let ZZ = (ZX.powf(YG)) * ((C - (PX / (ZY + PX))).powf(parameters[49]));
                                let AAA = if YU == A { 1.0 } else { 0.0 };
                                let AAJ;
                                if AAA != 0.0 {
                                    AAJ = ZZ;
                                } else {
                                    let AAB = (PX - parameters[52]) / ZY;
                                    let AAD = (AAB - C) / AAC;
                                    let AAE = if AAB < C { 1.0 } else { 0.0 };
                                    let AAH = if AAE != 0.0 {
                                        let AAF = C + (AAC * ((C + (AAD.exp())).ln()));
                                        AAF
                                    } else {
                                        let AAG = AAB + (AAC * ((C + ((-AAD).exp())).ln()));
                                        AAG
                                    };
                                    let AAI = ZZ * (AAH.powf(parameters[50]));
                                    AAJ = AAI;
                                }
                                let AAK = (-YF) * AAJ;
                                let AAL = if AAK < JG { 1.0 } else { 0.0 };
                                let AAO = if AAL != 0.0 {
                                    let AAM = AAK.exp();
                                    AAM
                                } else {
                                    let AAN = (JG.exp()) * (C + (AAK - JG));
                                    AAN
                                };
                                let AAP = ((YL / YF) * ZX) * AAO;
                                AAW = AAP;
                            } else {
                                AAW = A;
                            }
                            AAV = AAW;
                        } else {
                            AAV = A;
                        }
                        AAS = AAV;
                    }
                    AAQ = AAS;
                }
                let AAX = if AAQ > A { 1.0 } else { 0.0 };
                let ADF;
                if AAX != 0.0 {
                    let AAY = if parameters[53] == C { 1.0 } else { 0.0 };
                    let ADG;
                    if AAY != 0.0 {
                        let ABA = AAZ + XS;
                        let ABC = ((BD / (PX * ABA)) + ((PU / FS) * GF)) + (ABB / ABA);
                        let ABD = if XV == BU { 1.0 } else { 0.0 };
                        let ADH;
                        if ABD != 0.0 {
                            let ABE = (AAQ - ABC) / WK;
                            let ABF = if AAQ < ABC { 1.0 } else { 0.0 };
                            let ABI = if ABF != 0.0 {
                                let ABG = AAQ - (WK * ((C + (ABE.exp())).ln()));
                                ABG
                            } else {
                                let ABH = ABC - (WK * ((C + ((-ABE).exp())).ln()));
                                ABH
                            };
                            let ABJ = PX * ABI;
                            ADH = ABJ;
                        } else {
                            let ABK = ((PX * AAQ) * ABC) / (AAQ + ABC);
                            ADH = ABK;
                        }
                        ADG = ADH;
                    } else {
                        let ABL = PX * AAQ;
                        ADG = ABL;
                    }
                    ADF = ADG;
                } else {
                    ADF = A;
                }
                ADE = ADF;
            } else {
                ADE = A;
            }
            let ABM = if PE > A { 1.0 } else { 0.0 };
            if ABM != 0.0 {
            } else {
            }
            let ABN = if IQ < NU { 1.0 } else { 0.0 };
            if ABN != 0.0 {
            } else {
            }
            let ABO = HM * FT;
            let ABP = AE * DS;
            let ABQ = if JB < OL { 1.0 } else { 0.0 };
            if ABQ != 0.0 {
            } else {
            }
            let ABR = C - parameters[77];
            let ABS = (JC - OL) / ABP;
            let ABT = if JC < OL { 1.0 } else { 0.0 };
            let ABW = if ABT != 0.0 {
                let ABU = JC - (ABP * ((C + (ABS.exp())).ln()));
                ABU
            } else {
                let ABV = OL - (ABP * ((C + ((-ABS).exp())).ln()));
                ABV
            };
            let ABX = ((DV * ((OJ * ((OX * (C - ((C - (ABW / DS)).powf(OW)))) + (OK * (JC - ABW)))) + (DW * JC))) * ABR) * I;
            let ABY = if IS < (DQ * (C - (V.powf((-1e0f64 / parameters[139]))))) { 1.0 } else { 0.0 };
            if ABY != 0.0 {
            } else {
            }
            let ABZ = if (IO / (parameters[85] * BD)) < JG { 1.0 } else { 0.0 };
            if ABZ != 0.0 {
            } else {
            }
            let ACA = ((FP * HO) * BD) / EN;
            let ACB = if parameters[79] == A { 1.0 } else { 0.0 };
            if ACB != 0.0 {
            } else {
                let ACD = if (((JB - ACC) / parameters[91]) * BE) < JG { 1.0 } else { 0.0 };
                if ACD != 0.0 {
                } else {
                }
            }
            let ACE = if (if (if VI == C { 1.0 } else { 0.0 }) != 0.0 || (if VI == BU { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && VJ != 0.0 { 1.0 } else { 0.0 };
            let ADQ;
            if ACE != 0.0 {
                let ACQ;
                if ACB != 0.0 {
                    let ACF = PA * VO;
                    let ACH = FP * ACG;
                    let ACI = (((FM * I) * HQ) * ((ABO * ((ACF - PA) / (C + ((C + ACF).sqrt())))) + (ACA * (ACH / (C + ((C + ACH).sqrt())))))) / HP;
                    ACQ = ACI;
                } else {
                    let ACJ = (JC - ACC) * BE;
                    let ACK = if ACJ < JG { 1.0 } else { 0.0 };
                    let ACN = if ACK != 0.0 {
                        let ACL = ACJ.exp();
                        ACL
                    } else {
                        let ACM = (JG.exp()) * (C + (ACJ - JG));
                        ACM
                    };
                    let ACO = ((((V * I) * GM) * HR) * VO) / (C + ((C + (FP * ACN)).sqrt()));
                    ACQ = ACO;
                }
                let ACR = ACP * ACQ;
                ADQ = ACR;
            } else {
                ADQ = A;
            }
            let ACS = if parameters[6] == C { 1.0 } else { 0.0 };
            if ACS != 0.0 {
                let ACT = if NW < A { 1.0 } else { 0.0 };
                if ACT != 0.0 {
                } else {
                }
            } else {
            }
            let ACV = (ACU + SR) + TD;
            let ACY = ACW + SL;
            if GI != 0.0 {
            } else {
            }
            let ADD = (B * ADC) * Q;
            let ADI = ((B * IV) / AAZ) * Q;
            let ADJ = (B * parameters[69]) * IW;
            let ADK = 0e0f64 * Q;
            let ADL = (B * parameters[78]) * IX;
            let ADM = 0e0f64 * Q;
            let ADN = (B * XH) * Q;
            let ADR = B * (ABX + ADQ);
            let ADS = 0e0f64 * Q;
            if IC != 0.0 {
            } else {
            }
            if IF != 0.0 {
            } else {
            }
            let ADX = 5.5224904e-23f64 * L;
            let ADY = ADX / ABB;
            let ADZ = ADX / AAZ;
            let AEA = ADX * ADO;
            let AEB = ADX * ADT;
            let AEC = ADX * ADV;
            let AED = ((ADX / XS) * ((FP * XT) + 5e0f64)) * TU;
            let AEE = (PW + PV) / PU;
            let AEF = 3.2043836e-19f64 * (AEE.abs());
            let AEG = if parameters[130] > A { 1.0 } else { 0.0 };
            let AEI = if AEG != 0.0 {
                let AEH = (ADE / AEE).abs();
                AEH
            } else {
                A
            };
            let AEJ = (3.2043836e-19f64 * ADE) * (AEI + C);
            let AEK = if AEE > A { 1.0 } else { 0.0 };
            if AEK != 0.0 {
            } else {
            }
            let AEM = if AEL == C { 1.0 } else { 0.0 };
            if AEM != 0.0 {
            } else {
                let AEN = if AEL == V { 1.0 } else { 0.0 };
                if AEN != 0.0 {
                } else {
                }
            }
            let AEO = 3.2043836e-19f64 * ((((ACY - ACZ) + QR) + QJ).abs());
            let AEP = ACW + ACU;
            let AES = AEQ * ((AEP.abs()).powf(AER));
            let AET = if AEP < A { 1.0 } else { 0.0 };
            let AGC = if AET != 0.0 {
                let AEU = -AES;
                AEU
            } else {
                AES
            };
            let AEV = (SL + SR) + TD;
            let AEW = parameters[129] * ((AEV.abs()).powf(parameters[127]));
            let AEX = if AEV < A { 1.0 } else { 0.0 };
            let AGE = if AEX != 0.0 {
                let AEY = -AEW;
                AEY
            } else {
                AEW
            };
            let AEZ = 3.2043836e-19f64 * (ACV.abs());
            let AFA = XF.abs();
            let AFB = 3.2043836e-19f64 * AFA;
            let AFC = AEQ * (AFA.powf(AER));
            let AFD = if XF < A { 1.0 } else { 0.0 };
            let AGI = if AFD != 0.0 {
                let AFE = -AFC;
                AFE
            } else {
                AFC
            };
            let AFF = 3.2043836e-19f64 * (XC.abs());
            let AFG = XE.abs();
            let AFH = 3.2043836e-19f64 * AFG;
            let AFI = C - (VI * I);
            let AFJ = (AEQ * AFI) * ((AFG / AFI).powf(AER));
            let AFK = if XE < A { 1.0 } else { 0.0 };
            let AGL = if AFK != 0.0 {
                let AFL = -AFJ;
                AFL
            } else {
                AFJ
            };
            let AFM = XH.abs();
            let AFN = (3.2043836e-19f64 * AFM) * VI;
            let AFO = if I == A { 1.0 } else { 0.0 };
            let AFR = if AFO != 0.0 {
                A
            } else {
                let AFP = ((AEQ * VI) * I) * ((AFM / I).powf(AER));
                AFP
            };
            let AFQ = if XH < A { 1.0 } else { 0.0 };
            let AGO = if AFQ != 0.0 {
                let AFS = -AFR;
                AFS
            } else {
                AFR
            };
            let AFT = 3.2043836e-19f64 * (ADB.abs());
            let AFU = 3.2043836e-19f64 * (ADA.abs());
            let AFV = 3.2043836e-19f64 * (ADC.abs());
            let AFW = AEF * Q;
            let AFX = AEJ * Q;
            let AFY = AEO * Q;
            let AFZ = ADY * Q;
            let AGA = ADZ * Q;
            let AGB = AED * Q;
            let AGD = AGC * Q;
            let AGF = AGE * Q;
            let AGG = AEZ * Q;
            let AGH = AFB * Q;
            let AGJ = AGI * Q;
            let AGK = AFH * Q;
            let AGM = AGL * Q;
            let AGN = AFN * Q;
            let AGP = AGO * Q;
            let AHF;
            let AHG;
            let AHH;
            let AHI;
            if GI != 0.0 {
                let AGQ = AFF * Q;
                AHF = C;
                AHG = AGQ;
                AHH = A;
                AHI = A;
            } else {
                let AGR = AFF * Q;
                AHF = A;
                AHG = A;
                AHH = C;
                AHI = AGR;
            }
            let AGS = AFT * Q;
            let AGT = AFU * Q;
            let AGU = AFV * Q;
            let AHJ;
            let AHL;
            let AHN;
            let AHP;
            let AHR;
            let AHT;
            let AHV;
            let AHX;
            let AHZ;
            let AIB;
            let AID;
            let AIF;
            let AIH;
            let AIJ;
            let AIL;
            let AIN;
            if IC != 0.0 {
                let AHK;
                let AHM;
                let AHO;
                let AHQ;
                let AHS;
                let AHU;
                let AHW;
                let AHY;
                let AIA;
                let AIC;
                if IF != 0.0 {
                    let AGV = AEA * Q;
                    let AGW = AEB * Q;
                    let AGX = AEC * Q;
                    AHK = C;
                    AHM = AGV;
                    AHO = C;
                    AHQ = AGW;
                    AHS = C;
                    AHU = AGX;
                    AHW = A;
                    AHY = A;
                    AIA = A;
                    AIC = A;
                } else {
                    let AGY = AEA * Q;
                    let AGZ = AEB * Q;
                    AHK = A;
                    AHM = A;
                    AHO = A;
                    AHQ = A;
                    AHS = A;
                    AHU = A;
                    AHW = C;
                    AHY = AGY;
                    AIA = C;
                    AIC = AGZ;
                }
                AHJ = AHK;
                AHL = AHM;
                AHN = AHO;
                AHP = AHQ;
                AHR = AHS;
                AHT = AHU;
                AHV = AHW;
                AHX = AHY;
                AHZ = AIA;
                AIB = AIC;
                AID = A;
                AIF = A;
                AIH = A;
                AIJ = A;
                AIL = A;
                AIN = A;
            } else {
                let AIE;
                let AIG;
                let AII;
                let AIK;
                let AIM;
                let AIO;
                if IF != 0.0 {
                    let AHA = AEA * Q;
                    let AHB = AEC * Q;
                    AIE = C;
                    AIG = AHA;
                    AII = C;
                    AIK = AHB;
                    AIM = A;
                    AIO = A;
                } else {
                    let AHC = AEA * Q;
                    AIE = A;
                    AIG = A;
                    AII = A;
                    AIK = A;
                    AIM = C;
                    AIO = AHC;
                }
                AHJ = A;
                AHL = A;
                AHN = A;
                AHP = A;
                AHR = A;
                AHT = A;
                AHV = A;
                AHX = A;
                AHZ = A;
                AIB = A;
                AID = AIE;
                AIF = AIG;
                AIH = AII;
                AIJ = AIK;
                AIL = AIM;
                AIN = AIO;
            }
            let AHD = if (((((ADD + ADI) + ADK) + ADM) + ADN) + ADS) == A { 1.0 } else { 0.0 };
            if AHD != 0.0 {
            } else {
            }
            let AHE = if Q != C { 1.0 } else { 0.0 };
            if AHE != 0.0 {
            } else {
            }
        {
            let psd = AFW;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 0, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 0, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(0, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = AFX;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(1, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = AFY;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 2, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 2, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(2, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = AFZ;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 3, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 3, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(3, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = AGA;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 4, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 4, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(4, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = AGB;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 5, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 5, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(5, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = AGD;
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
            let psd = AGF;
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
            let psd = AGG;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 8, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 8, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(8, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = AGH;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 9, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 9, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(9, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = AGJ;
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
            let psd = AGK;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 11, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 11, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(11, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = AGM;
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
            let psd = AGN;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 13, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 13, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(13, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = AGP;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 14, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = Some(C);
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 14, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 14, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(14, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if AHF == 0.0 {
            if !visitor.visit(15, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = AHG;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 15, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 15, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(15, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if AHH == 0.0 {
            if !visitor.visit(16, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = AHI;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 16, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 16, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(16, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = AGS;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 17, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 17, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(17, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = AGT;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 18, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 18, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(18, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = AGU;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 19, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 19, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(19, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if AHJ == 0.0 {
            if !visitor.visit(20, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = AHL;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 20, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 20, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(20, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if AHN == 0.0 {
            if !visitor.visit(21, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = AHP;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 21, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 21, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(21, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if AHR == 0.0 {
            if !visitor.visit(22, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = AHT;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 22, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 22, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(22, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if AHV == 0.0 {
            if !visitor.visit(23, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = AHX;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 23, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 23, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(23, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if AHZ == 0.0 {
            if !visitor.visit(24, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = AIB;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 24, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 24, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(24, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if AID == 0.0 {
            if !visitor.visit(25, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = AIF;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 25, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 25, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(25, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if AIH == 0.0 {
            if !visitor.visit(26, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = AIJ;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 26, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 26, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(26, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if AIL == 0.0 {
            if !visitor.visit(27, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = AIN;
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
