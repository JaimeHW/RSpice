#![allow(dead_code, non_snake_case, unused_parens, unused_variables)]

use super::state::Instance;
use rspice_veriloga_runtime::GeneratedEvalContext;
pub use rspice_veriloga_runtime::{GeneratedNoiseDescriptor, GeneratedNoiseEndpoint, GeneratedNoiseEvaluation, GeneratedNoiseEvaluationError, GeneratedNoiseEvaluationRef, GeneratedNoiseKind, GeneratedNoiseVisitor};

pub static NOISE_SOURCES: [GeneratedNoiseDescriptor; 25] = [
    GeneratedNoiseDescriptor { mechanism: "WHITE_NOI_GND_IN", label: Some("in"), kind: GeneratedNoiseKind::White, equation: 25, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(10), name: "noi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: None, name: "0", is_internal: false }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C2_B2_IAVL", label: Some("iavl"), kind: GeneratedNoiseKind::White, equation: 30, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(7), name: "c2", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(5), name: "b2", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_B2_E1_IB2E1", label: Some("ib2e1"), kind: GeneratedNoiseKind::White, equation: 31, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "b2", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(3), name: "e1", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_E_E1_RE", label: Some("re"), kind: GeneratedNoiseKind::White, equation: 32, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(2), name: "e", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(3), name: "e1", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_B_B1_RBC", label: Some("rbc"), kind: GeneratedNoiseKind::White, equation: 33, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(1), name: "b", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(4), name: "b1", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_B1_B2_RBV", label: Some("rbv"), kind: GeneratedNoiseKind::White, equation: 34, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(4), name: "b1", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(5), name: "b2", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_B2_E1_IB2E1_F", label: Some("ib2e1_f"), kind: GeneratedNoiseKind::Flicker, equation: 35, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "b2", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(3), name: "e1", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_B1_E1_IB1E1_F", label: Some("ib1e1_f"), kind: GeneratedNoiseKind::Flicker, equation: 36, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(4), name: "b1", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(3), name: "e1", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_B1_E1_IB1E1", label: Some("ib1e1"), kind: GeneratedNoiseKind::White, equation: 37, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(4), name: "b1", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(3), name: "e1", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_B1_C4_IB3", label: Some("ib3"), kind: GeneratedNoiseKind::White, equation: 38, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(4), name: "b1", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(9), name: "c4", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_B1_C4_IB3_F", label: Some("ib3_f"), kind: GeneratedNoiseKind::Flicker, equation: 39, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(4), name: "b1", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(9), name: "c4", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_B1_C4_IEX", label: Some("iex"), kind: GeneratedNoiseKind::White, equation: 40, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(4), name: "b1", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(9), name: "c4", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_B1_C4_IEX_F", label: Some("iex_f"), kind: GeneratedNoiseKind::Flicker, equation: 41, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(4), name: "b1", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(9), name: "c4", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_B_C3_XIEX", label: Some("xiex"), kind: GeneratedNoiseKind::White, equation: 42, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(1), name: "b", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(8), name: "c3", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_B_C3_XIEX_F", label: Some("xiex_f"), kind: GeneratedNoiseKind::Flicker, equation: 43, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(1), name: "b", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(8), name: "c3", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C1_B2_IZTCB", label: Some("iztcb"), kind: GeneratedNoiseKind::White, equation: 44, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(6), name: "c1", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(5), name: "b2", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C2_B2_IZTCB", label: Some("iztcb"), kind: GeneratedNoiseKind::White, equation: 45, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(7), name: "c2", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(5), name: "b2", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C_C3_RCC", label: Some("rcc"), kind: GeneratedNoiseKind::White, equation: 46, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(0), name: "c", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(8), name: "c3", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C3_C4_RCBLX", label: Some("rcblx"), kind: GeneratedNoiseKind::White, equation: 47, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(8), name: "c3", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(9), name: "c4", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C4_C1_RCBLI", label: Some("rcbli"), kind: GeneratedNoiseKind::White, equation: 48, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(9), name: "c4", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(6), name: "c1", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C_C3_RCC", label: Some("rcc"), kind: GeneratedNoiseKind::White, equation: 49, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(0), name: "c", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(8), name: "c3", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C3_C1_RCBLX", label: Some("rcblx"), kind: GeneratedNoiseKind::White, equation: 50, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(8), name: "c3", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(6), name: "c1", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C_C4_RCC", label: Some("rcc"), kind: GeneratedNoiseKind::White, equation: 51, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(0), name: "c", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(9), name: "c4", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C4_C1_RCBLI", label: Some("rcbli"), kind: GeneratedNoiseKind::White, equation: 52, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(9), name: "c4", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(6), name: "c1", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C_C1_RCC", label: Some("rcc"), kind: GeneratedNoiseKind::White, equation: 53, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(0), name: "c", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(6), name: "c1", is_internal: true }, table_len: 0, table_log_interp: false },
];

impl Instance {
    pub fn evaluate_noise_sources(&self, ctx: &GeneratedEvalContext<'_>, visitor: &mut dyn GeneratedNoiseVisitor) -> Result<(), GeneratedNoiseEvaluationError> {
        if !self.multiplicity.is_finite() || self.multiplicity <= 0.0 {
            return Err(GeneratedNoiseEvaluationError::InvalidMultiplicity { value: self.multiplicity });
        }
        let parameters = &self.params.values;
        let temperature = ctx.temperature();
        let node_potentials = [ctx.node_voltage(self.nodes[0]), ctx.node_voltage(self.nodes[1]), ctx.node_voltage(self.nodes[2]), ctx.node_voltage(self.nodes[3]), ctx.node_voltage(self.nodes[4]), ctx.node_voltage(self.nodes[5]), ctx.node_voltage(self.nodes[6]), ctx.node_voltage(self.nodes[7]), ctx.node_voltage(self.nodes[8]), ctx.node_voltage(self.nodes[9]), ctx.node_voltage(self.nodes[10])];
            let A = 0e0f64;
            let B = parameters[3];
            let C = 1e0f64;
            let E = 7.03e7f64;
            let F = 1.23e8f64;
            let G = 1.58e8f64;
            let H = 2.04e8f64;
            let I = parameters[32];
            let M = parameters[137];
            let O = 1e-12f64;
            let Q = parameters[1];
            let T = 1e-3f64;
            let U = 2e0f64;
            let V = parameters[66];
            let Y = parameters[113];
            let Z = parameters[114];
            let AA = parameters[115];
            let AC = 5e-2f64;
            let AD = 1e-1f64;
            let AJ = parameters[65];
            let AL = parameters[70];
            let AM = parameters[71];
            let AP = parameters[116];
            let AQ = parameters[117];
            let AR = parameters[118];
            let BB = 8.617086918058125e-5f64;
            let BT = 3e0f64;
            let BV = parameters[104];
            let CB = parameters[109];
            let DJ = parameters[74];
            let DP = parameters[96];
            let DS = parameters[97];
            let DT = parameters[95];
            let DX = parameters[56];
            let DZ = parameters[57];
            let EC = parameters[58];
            let EE = parameters[98];
            let EG = parameters[121];
            let EI = parameters[9];
            let EQ = parameters[122];
            let ES = parameters[10];
            let FB = 1e-6f64;
            let FE = 5e-1f64;
            let FH = 4e0f64;
            let FI = parameters[120];
            let FM = parameters[102];
            let FO = 6e0f64;
            let FP = parameters[20];
            let FS = parameters[31];
            let FV = parameters[16];
            let FY = parameters[18];
            let GF = parameters[22];
            let GH = parameters[133];
            let GN = parameters[34];
            let GP = parameters[33];
            let GT = parameters[36];
            let GV = parameters[35];
            let HA = parameters[86];
            let HC = parameters[87];
            let HM = parameters[91];
            let HX = node_potentials[5];
            let HY = node_potentials[6];
            let IA = node_potentials[7];
            let IC = node_potentials[3];
            let IE = node_potentials[4];
            let II = node_potentials[1];
            let IM = node_potentials[9];
            let IQ = parameters[134];
            let KF = parameters[136];
            let KL = 1e2f64;
            let KY = parameters[61];
            let KZ = parameters[60];
            let LD = parameters[62];
            let LY = parameters[135];
            let NC = parameters[73];
            let OK = 1.0000000000000002e-2f64;
            let OW = 1e-4f64;
            let PI = parameters[141];
            let QA = 4e1f64;
            let QL = parameters[92];
            let SI = 1e-30f64;
            let SK = 1.6666666666666666e-1f64;
            let SS = 3.333333333333333e-1f64;
            let ST = 2.5e-1f64;
            let TU = parameters[5];
            let UD = 1.21e-2f64;
            let UO = 1e-6f64;
            let UP = 1e-12f64;
            let UU = parameters[81];
            let UW = parameters[80];
            let VN = 1.0000000000000002e-2f64;
            let VZ = parameters[38];
            let WB = parameters[43];
            let WK = parameters[40];
            let WP = parameters[39];
            let WU = parameters[44];
            let WY = parameters[7];
            let XK = parameters[46];
            let YC = parameters[47];
            let YG = parameters[50];
            let ACK = parameters[130];
            let ACP = parameters[127];
            let ACQ = parameters[125];
            let D = if B == C { 1.0 } else { 0.0 };
            let HJ;
            let XU;
            if D != 0.0 {
                HJ = F;
                XU = E;
            } else {
                HJ = H;
                XU = G;
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
            let W = U.powf((U - V));
            let X = C / W;
            let AB = Y + (((Z * K) * K) / (K + AA));
            let AE = (AB - AC) / AD;
            let AF = if AB < AC { 1.0 } else { 0.0 };
            let BH = if AF != 0.0 {
                let AG = AC + (AD * ((C + (AE.exp())).ln()));
                AG
            } else {
                let AH = AB + (AD * ((C + ((-AE).exp())).ln()));
                AH
            };
            let AI = C / Y;
            let AK = C / AJ;
            let AN = U.powf((U - AM));
            let AO = C / AN;
            let AS = AP + (((AQ * K) * K) / (K + AR));
            let AT = (AS - AC) / AD;
            let AU = if AS < AC { 1.0 } else { 0.0 };
            let BN = if AU != 0.0 {
                let AV = AC + (AD * ((C + (AT.exp())).ln()));
                AV
            } else {
                let AW = AS + (AD * ((C + ((-AT).exp())).ln()));
                AW
            };
            let AX = C / AP;
            let AY = C / AL;
            let AZ = C - (C / parameters[82]);
            let BA = L / K;
            let BC = BB * L;
            let BD = C / BC;
            let BE = BD - (C / (BB * K));
            let BF = L - K;
            let BG = BA.ln();
            let BI = BH - (((Z * L) * L) / (L + AA));
            let BJ = (BI - AC) / AD;
            let BK = if BI < AC { 1.0 } else { 0.0 };
            let GK = if BK != 0.0 {
                let BL = AC + (AD * ((C + (BJ.exp())).ln()));
                BL
            } else {
                let BM = BI + (AD * ((C + ((-BJ).exp())).ln()));
                BM
            };
            let BO = BN - (((AQ * L) * L) / (L + AR));
            let BP = (BO - AC) / AD;
            let BQ = if BO < AC { 1.0 } else { 0.0 };
            let GR = if BQ != 0.0 {
                let BR = AC + (AD * ((C + (BP.exp())).ln()));
                BR
            } else {
                let BS = BO + (AD * ((C + ((-BP).exp())).ln()));
                BS
            };
            let BU = C - BA;
            let BW = (((-3e0f64 * BC) * BG) + (AJ * BA)) + (BU * BV);
            let BX = (AC - BW) / BC;
            let BY = if AC < BW { 1.0 } else { 0.0 };
            let DD = if BY != 0.0 {
                let BZ = BW + (BC * ((C + (BX.exp())).ln()));
                BZ
            } else {
                let CA = AC + (BC * ((C + ((-BX).exp())).ln()));
                CA
            };
            let CC = BU * CB;
            let CD = (((-3e0f64 * BC) * BG) + (parameters[63] * BA)) + CC;
            let CE = (AC - CD) / BC;
            let CF = if AC < CD { 1.0 } else { 0.0 };
            let JL = if CF != 0.0 {
                let CG = CD + (BC * ((C + (CE.exp())).ln()));
                CG
            } else {
                let CH = AC + (BC * ((C + ((-CE).exp())).ln()));
                CH
            };
            let CI = (((-3e0f64 * BC) * BG) + (parameters[79] * BA)) + CC;
            let CJ = (AC - CI) / BC;
            let CK = if AC < CI { 1.0 } else { 0.0 };
            let AAF = if CK != 0.0 {
                let CL = CI + (BC * ((C + (CJ.exp())).ln()));
                CL
            } else {
                let CM = AC + (BC * ((C + ((-CJ).exp())).ln()));
                CM
            };
            let CN = AL * BA;
            let CO = (((-3e0f64 * BC) * BG) + CN) + CC;
            let CP = (AC - CO) / BC;
            let CQ = if AC < CO { 1.0 } else { 0.0 };
            let DK = if CQ != 0.0 {
                let CR = CO + (BC * ((C + (CP.exp())).ln()));
                CR
            } else {
                let CS = AC + (BC * ((C + ((-CP).exp())).ln()));
                CS
            };
            let CT = (((-3e0f64 * BC) * BG) + CN) + CC;
            let CU = (AC - CT) / BC;
            let CV = if AC < CT { 1.0 } else { 0.0 };
            let DF = if CV != 0.0 {
                let CW = CT + (BC * ((C + (CU.exp())).ln()));
                CW
            } else {
                let CX = AC + (BC * ((C + ((-CU).exp())).ln()));
                CX
            };
            let CY = (((-3e0f64 * BC) * BG) + (parameters[26] * BA)) + (BU * parameters[108]);
            let CZ = (AC - CY) / BC;
            let DA = if AC < CY { 1.0 } else { 0.0 };
            let PU = if DA != 0.0 {
                let DB = CY + (BC * ((C + (CZ.exp())).ln()));
                DB
            } else {
                let DC = AC + (BC * ((C + ((-CZ).exp())).ln()));
                DC
            };
            let DE = C / DD;
            let DG = C / DF;
            let DH = (AJ * DE).powf(V);
            let DI = (AL * DG).powf(AM);
            let DL = ((C - DJ) * ((AL / DK).powf(AM))) + DJ;
            let DM = C / DL;
            let DN = parameters[69] * DL;
            let DO = DJ * DM;
            let DQ = parameters[53] * ((BG * DP).exp());
            let DR = if DQ < R { 1.0 } else { 0.0 };
            let ZF = if DR != 0.0 {
                R
            } else {
                DQ
            };
            let DU = parameters[55] * ((BG * (DS - DT)).exp());
            let DV = parameters[54] * ((BG * parameters[100]).exp());
            let DW = if DV < R { 1.0 } else { 0.0 };
            let ZD = if DW != 0.0 {
                R
            } else {
                DV
            };
            let DY = DX * ((BG * parameters[101]).exp());
            let EA = (BG * parameters[103]).exp();
            let EB = DZ * EA;
            let ED = EC * EA;
            let EF = parameters[59] * ((BG * EE).exp());
            let EH = if EG != A { 1.0 } else { 0.0 };
            let FJ;
            if EH != 0.0 {
                let EJ = EI * (C + (BF * EG));
                let EK = (EJ - C) / T;
                let EL = if EJ < C { 1.0 } else { 0.0 };
                let EO = if EL != 0.0 {
                    let EM = C + (T * ((C + (EK.exp())).ln()));
                    EM
                } else {
                    let EN = EJ + (T * ((C + ((-EK).exp())).ln()));
                    EN
                };
                let EP = EO - 6.931471805599453e-4f64;
                FJ = EP;
            } else {
                FJ = EI;
            }
            let ER = if EQ != A { 1.0 } else { 0.0 };
            let OD;
            if ER != 0.0 {
                let ET = ES * (C + (BF * EQ));
                let EU = (ET - C) / T;
                let EV = if ET < C { 1.0 } else { 0.0 };
                let EY = if EV != 0.0 {
                    let EW = C + (T * ((C + (EU.exp())).ln()));
                    EW
                } else {
                    let EX = ET + (T * ((C + ((-EU).exp())).ln()));
                    EX
                };
                let EZ = EY - 6.931471805599453e-4f64;
                OD = EZ;
            } else {
                OD = ES;
            }
            let FA = parameters[42] * (C + (parameters[123] * BF));
            let FC = FA * FA;
            let FD = if FA < A { 1.0 } else { 0.0 };
            let WJ = if FD != 0.0 {
                let FF = 5e-7f64 / (((FC + FB).sqrt()) - FA);
                FF
            } else {
                let FG = FE * (((FC + FB).sqrt()) + FA);
                FG
            };
            let FK = (parameters[8] * (((BG * (((FH - DS) - DT) + FI)) / FJ).exp())) * ((((-BV) * BE) / FJ).exp());
            let FL = parameters[11] * ((BG * (C - DS)).exp());
            let FN = parameters[29] * ((BG * (C - FM)).exp());
            let FQ = (-parameters[112]) * BE;
            let FR = (parameters[19] * ((BG * (FO - (U * FP))).exp())) * ((FQ / FP).exp());
            let FT = (parameters[30] * ((BG * (FO - (U * FS))).exp())) * ((((-CB) * BE) / FS).exp());
            let FU = BG * ((FH - DP) + FI);
            let FW = (-parameters[110]) * BE;
            let FX = (parameters[15] * ((FU / FV).exp())) * ((FW / FV).exp());
            let FZ = (parameters[17] * ((FU / FY).exp())) * ((FW / FY).exp());
            let GA = if parameters[23] == C { 1.0 } else { 0.0 };
            let QG;
            let QI;
            let RA;
            if GA != 0.0 {
                let GB = parameters[24] * ((((-parameters[106]) * BE) / FV).exp());
                let GC = parameters[27] * (((-parameters[105]) * BE).exp());
                let GD = parameters[25] * ((((-parameters[107]) * BE) / FY).exp());
                QG = GB;
                QI = GC;
                RA = GD;
            } else {
                QG = A;
                QI = A;
                RA = A;
            }
            let GE = (parameters[28] * ((BG * ((FH - FM) + FI)).exp())) * (((-parameters[111]) * BE).exp());
            let GG = (parameters[21] * ((BG * (FO - (U * GF))).exp())) * ((FQ / GF).exp());
            let GI = (parameters[132] * ((BG * (FH / GH)).exp())) * ((FQ / GH).exp());
            let GJ = (parameters[138] * (BA.sqrt())) * ((parameters[140] * BF).exp());
            let GL = (GK * AI).powf(-5e-1f64);
            let GM = C / DH;
            let GO = (((((((GN * GK) * GK) * GL) * GM) * AJ) * DE) * AI) * AI;
            let GQ = ((((((GP * GL) * DD) * DD) * AK) * AK) * DH) * ((GN - GO).exp());
            let GS = (GR * AX).powf(-5e-1f64);
            let GU = (((((((GT * GR) * GR) * GS) * (C / DI)) * AL) * DG) * AX) * AX;
            let GW = ((((((GV * GS) * DF) * DF) * AY) * AY) * DI) * ((GT - GU).exp());
            let GX = (BG * DT).exp();
            let GY = (parameters[13] * GX) * DM;
            let GZ = (parameters[12] * GX) * GM;
            let HB = HA * ((BG * ((DT + DS) - C)).exp());
            let HD = HC * ((BG * (EE - C)).exp());
            let HE = HB + HD;
            let HF = (parameters[88] * HE) / (HA + HC);
            let HG = parameters[89] * ((BG * (parameters[99] - C)).exp());
            let HH = L - 3e2f64;
            let HI = if L < 5.25e2f64 { 1.0 } else { 0.0 };
            let XV = if HI != 0.0 {
                let HK = HJ * ((C + (7.2e-4f64 * HH)) - ((1.6e-6f64 * HH) * HH));
                HK
            } else {
                let HL = HJ * 1.081e0f64;
                HL
            };
            let HN = HM * GX;
            let HO = if DX > A { 1.0 } else { 0.0 };
            let ABN;
            if HO != 0.0 {
                let HP = C / DY;
                let HQ = if HP > S { 1.0 } else { 0.0 };
                let ABO = if HQ != 0.0 {
                    S
                } else {
                    HP
                };
                ABN = ABO;
            } else {
                ABN = A;
            }
            let HR = if DZ > A { 1.0 } else { 0.0 };
            let ABS;
            if HR != 0.0 {
                let HS = C / EB;
                let HT = if HS > S { 1.0 } else { 0.0 };
                let ABT = if HT != 0.0 {
                    S
                } else {
                    HS
                };
                ABS = ABT;
            } else {
                ABS = A;
            }
            let HU = if EC > A { 1.0 } else { 0.0 };
            let ABU;
            if HU != 0.0 {
                let HV = C / ED;
                let HW = if HV > S { 1.0 } else { 0.0 };
                let ABV = if HW != 0.0 {
                    S
                } else {
                    HV
                };
                ABU = ABV;
            } else {
                ABU = A;
            }
            let HZ = B * (HX - HY);
            let IB = B * (HX - IA);
            let ID = B * (HX - IC);
            let IF = B * (IE - IC);
            let IG = B * (IE - HX);
            let IH = B * (HY - IA);
            let IJ = B * (II - IE);
            let IK = B * (II - node_potentials[2]);
            let IL = B * (II - node_potentials[0]);
            let IN = ((IG + IB) - IH) - (B * (IM - HY));
            let IO = IL + ((((-IL) + IJ) + IN) - (B * (node_potentials[8] - IM)));
            let IP = IB * BD;
            let IR = if IP < IQ { 1.0 } else { 0.0 };
            let ML = if IR != 0.0 {
                let IS = IP.exp();
                IS
            } else {
                let IT = (IQ.exp()) * (C + (IP - IQ));
                IT
            };
            let IU = ID * BD;
            let IV = IU / FJ;
            let IW = if IV < IQ { 1.0 } else { 0.0 };
            let NZ = if IW != 0.0 {
                let IX = IV.exp();
                IX
            } else {
                let IY = (IQ.exp()) * (C + (IV - IQ));
                IY
            };
            let IZ = IN * BD;
            let JA = if IZ < IQ { 1.0 } else { 0.0 };
            let TR = if JA != 0.0 {
                let JB = IZ.exp();
                JB
            } else {
                let JC = (IQ.exp()) * (C + (IZ - IQ));
                JC
            };
            let JD = IG * BD;
            let JE = if JD < IQ { 1.0 } else { 0.0 };
            let VX = if JE != 0.0 {
                let JF = JD.exp();
                JF
            } else {
                let JG = (IQ.exp()) * (C + (JD - IQ));
                JG
            };
            let JH = IO * BD;
            let JI = if JH < IQ { 1.0 } else { 0.0 };
            let TY = if JI != 0.0 {
                let JJ = JH.exp();
                JJ
            } else {
                let JK = (IQ.exp()) * (C + (JH - IQ));
                JK
            };
            let JM = (IO - JL) * BD;
            let JN = if JM < IQ { 1.0 } else { 0.0 };
            let AAJ = if JN != 0.0 {
                let JO = JM.exp();
                JO
            } else {
                let JP = (IQ.exp()) * (C + (JM - IQ));
                JP
            };
            let JQ = if ((IN - JL) * BD) < IQ { 1.0 } else { 0.0 };
            if JQ != 0.0 {
            } else {
            }
            let JR = (IB - JL) * BD;
            let JS = if JR < IQ { 1.0 } else { 0.0 };
            let JZ = if JS != 0.0 {
                let JT = JR.exp();
                JT
            } else {
                let JU = (IQ.exp()) * (C + (JR - IQ));
                JU
            };
            let JV = (HZ - JL) * BD;
            let JW = if JV < IQ { 1.0 } else { 0.0 };
            let KB = if JW != 0.0 {
                let JX = JV.exp();
                JX
            } else {
                let JY = (IQ.exp()) * (C + (JV - IQ));
                JY
            };
            let KA = (C + (FH * JZ)).sqrt();
            let KC = (C + (FH * KB)).sqrt();
            let KD = C + KC;
            let KE = (U * KB) / KD;
            let KG = if KE < KF { 1.0 } else { 0.0 };
            let LN = if KG != 0.0 {
                KF
            } else {
                KE
            };
            let KH = KA + C;
            let KI = BC * ((KA - KC) - ((KH / KD).ln()));
            let KJ = (KI + IH) / EF;
            let KK = if KJ > A { 1.0 } else { 0.0 };
            let NF;
            let NM;
            let NS;
            let OC;
            let XA;
            let XO;
            if KK != 0.0 {
                let KM = if HZ < KL { 1.0 } else { 0.0 };
                let KP = if KM != 0.0 {
                    HZ
                } else {
                    let KN = KL + ((C + (HZ - KL)).ln());
                    KN
                };
                let KO = (FE * KJ) * EF;
                let KQ = (JL + ((U * BC) * (((KO * BD) + C).ln()))) - KP;
                let KR = 2e-1f64 * JL;
                let KS = KR * KR;
                let KT = KQ * KQ;
                let KU = if KQ < A { 1.0 } else { 0.0 };
                let KX = if KU != 0.0 {
                    let KV = (FE * KS) / (((KT + KS).sqrt()) - KQ);
                    KV
                } else {
                    let KW = FE * (((KT + KS).sqrt()) + KQ);
                    KW
                };
                let LA = KY * KZ;
                let LB = (KX * (KX + LA)) / (KZ * (KX + (KY * EF)));
                let LC = KJ / LB;
                let LE = (LC - C) / LD;
                let LF = if LC < C { 1.0 } else { 0.0 };
                let LI = if LF != 0.0 {
                    let LG = C + (LD * ((C + (LE.exp())).ln()));
                    LG
                } else {
                    let LH = LC + (LD * ((C + ((-LE).exp())).ln()));
                    LH
                };
                let LJ = LI / (C + (LD * ((C + ((-1e0f64 / LD).exp())).ln())));
                let LK = KX / LA;
                let LL = C + LK;
                let LM = (C + ((C + (((FH * LJ) * LK) * LL)).sqrt())) / ((U * LJ) * LL);
                let LO = LN * LM;
                let LP = ((C - LM) + LO) / (C + LO);
                let LQ = (KO * LP) * BD;
                let LR = (U * LQ) + (LN * ((LN + LQ) + C));
                let LS = FE * (LQ - C);
                let LT = (LS * LS) + LR;
                let LU = if LQ >= C { 1.0 } else { 0.0 };
                let LX = if LU != 0.0 {
                    let LV = LS + (LT.sqrt());
                    LV
                } else {
                    let LW = LR / ((LT.sqrt()) - LS);
                    LW
                };
                let LZ = if LX < LY { 1.0 } else { 0.0 };
                let MA = if LZ != 0.0 {
                    LY
                } else {
                    LX
                };
                let MB = (MA * (MA + C)) * ((JL * BD).exp());
                let MC = (FE * KZ) * (KJ - KY);
                let MD = MC + (((MC * MC) + (((KZ * EF) * KY) * KJ)).sqrt());
                let ME = if parameters[72] == A { 1.0 } else { 0.0 };
                let NN = if ME != 0.0 {
                    let MF = DK * AD;
                    MF
                } else {
                    let MG = DK * (AD + ((U * KJ) / (KJ + LB)));
                    MG
                };
                let MH = KY + KJ;
                let MI = (KY * KJ) / MH;
                let MJ = KY / MH;
                NF = MD;
                NM = NN;
                NS = MJ;
                OC = MB;
                XA = LP;
                XO = MI;
            } else {
                let MK = (U * JZ) / KH;
                let MM = if (if (IH.abs()) < (1e-5f64 * BC) { 1.0 } else { 0.0 }) != 0.0 || (if (KI.abs()) < ((1e-40f64 * BC) * (KA + KC)) { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let XB = if MM != 0.0 {
                    let MN = FE * (MK + LN);
                    let MO = MN / (MN + C);
                    MO
                } else {
                    let MP = KI / ((KI + IB) - HZ);
                    MP
                };
                let MQ = AD * DK;
                let MR = C - (KJ / KY);
                NF = IH;
                NM = MQ;
                NS = MR;
                OC = ML;
                XA = XB;
                XO = KJ;
            }
            let MS = DD * (C - (BT.powf((-1e0f64 / V))));
            let MT = AD * DD;
            let MU = (ID - MS) / MT;
            let MV = if ID < MS { 1.0 } else { 0.0 };
            let MY = if MV != 0.0 {
                let MW = ID - (MT * ((C + (MU.exp())).ln()));
                MW
            } else {
                let MX = MS - (MT * ((C + ((-MU).exp())).ln()));
                MX
            };
            let MZ = C - V;
            let NA = (C - (MY * DE)).powf(MZ);
            let NB = ((DD / MZ) * (C - NA)) + (BT * (ID - MY));
            let ND = if NC == C { 1.0 } else { 0.0 };
            let NK;
            if ND != 0.0 {
                NK = HZ;
            } else {
                let NE = if NC == U { 1.0 } else { 0.0 };
                let NL = if NE != 0.0 {
                    let NG = HZ + NF;
                    NG
                } else {
                    IB
                };
                NK = NL;
            }
            let NH = C - DO;
            let NI = (U - DO) / NH;
            let NJ = DK * (C - (NI.powf((-1e0f64 / AM))));
            let NO = (NK - NJ) / NM;
            let NP = if NK < NJ { 1.0 } else { 0.0 };
            let NW = if NP != 0.0 {
                let NQ = NK - (NM * ((C + (NO.exp())).ln()));
                NQ
            } else {
                let NR = NJ - (NM * ((C + ((-NO).exp())).ln()));
                NR
            };
            let NT = NS.powf(parameters[75]);
            let NU = C - AM;
            let NV = DK / NU;
            let NX = (NH * ((NV * (C - (NT * ((C - (NW / DK)).powf(NU))))) + ((NT * NI) * (NK - NW)))) + (DO * HZ);
            let NY = (FH * FK) / FL;
            let OA = NY * NZ;
            let OB = OA / (C + ((C + OA).sqrt()));
            let OE = OC.powf((C / OD));
            let OF = NY * OE;
            let OG = OF / (C + ((C + OF).sqrt()));
            let OH = if HM == A { 1.0 } else { 0.0 };
            let OL = if OH != 0.0 {
                let OI = (C + (NB / GZ)) + (NX / GY);
                OI
            } else {
                let OJ = ((((((NB / GZ) + C) * HN) * BD).exp()) - (((((-NX) / GY) * HN) * BD).exp())) / (((HN * BD).exp()) - C);
                OJ
            };
            let OM = OL * OL;
            let ON = if OL < A { 1.0 } else { 0.0 };
            let OQ = if ON != 0.0 {
                let OO = 5.000000000000001e-3f64 / (((OM + OK).sqrt()) - OL);
                OO
            } else {
                let OP = FE * (((OM + OK).sqrt()) + OL);
                OP
            };
            let OR = C + (FE * (OB + OG));
            let OS = OQ * OR;
            let OT = (parameters[14] * FK) * OE;
            let OU = FK * NZ;
            let OV = (OU - OT) / OS;
            let OX = ID / OW;
            let OY = if ID < A { 1.0 } else { 0.0 };
            let PB = if OY != 0.0 {
                let OZ = OW * ((C + (OX.exp())).ln());
                OZ
            } else {
                let PA = ID + (OW * ((C + ((-OX).exp())).ln()));
                PA
            };
            let PC = PB / parameters[139];
            let PD = if PC < IQ { 1.0 } else { 0.0 };
            let PG = if PD != 0.0 {
                let PE = PC.exp();
                PE
            } else {
                let PF = (IQ.exp()) * (C + (PC - IQ));
                PF
            };
            let PH = GJ * (PG - C);
            let PJ = (ID - PI) / T;
            let PK = if ID < PI { 1.0 } else { 0.0 };
            let PN = if PK != 0.0 {
                let PL = ID - (T * ((C + (PJ.exp())).ln()));
                PL
            } else {
                let PM = PI - (T * ((C + ((-PJ).exp())).ln()));
                PM
            };
            let PO = PI - PN;
            let PP = (parameters[142] * PN) * (PO * PO);
            let PQ = IU / FV;
            let PR = if PQ < IQ { 1.0 } else { 0.0 };
            let QE = if PR != 0.0 {
                let PS = PQ.exp();
                PS
            } else {
                let PT = (IQ.exp()) * (C + (PQ - IQ));
                PT
            };
            let AAZ;
            if GA != 0.0 {
                let PV = (ID - PU) * BD;
                let PW = if PV < IQ { 1.0 } else { 0.0 };
                let QH = if PW != 0.0 {
                    let PX = PV.exp();
                    PX
                } else {
                    let PY = (IQ.exp()) * (C + (PV - IQ));
                    PY
                };
                let PZ = (OV / FK) - 1e3f64;
                let QB = if PZ < QA { 1.0 } else { 0.0 };
                let QJ = if QB != 0.0 {
                    let QC = PZ.exp();
                    QC
                } else {
                    let QD = 2.3538526683702e17f64 * (C + (PZ - QA));
                    QD
                };
                let QF = QE - C;
                let QK = ((FX * QF) + ((((QG * U) * QF) / (C + ((C + (FH * QH)).sqrt()))) * (C + (NX / GY)))) + (((QI * (OC - C)) * QJ) / (C + QJ));
                AAZ = QK;
            } else {
                let QM = if QL == A { 1.0 } else { 0.0 };
                let ABA = if QM != 0.0 {
                    let QN = FX * (QE - C);
                    QN
                } else {
                    let QO = FX * (((C - QL) * (QE - C)) + ((QL * ((QE + OC) - U)) * (C + (NX / GY))));
                    QO
                };
                AAZ = ABA;
            }
            let QP = IF * BD;
            let QQ = QP / FY;
            let QR = if QQ < IQ { 1.0 } else { 0.0 };
            let QY = if QR != 0.0 {
                let QS = QQ.exp();
                QS
            } else {
                let QT = (IQ.exp()) * (C + (QQ - IQ));
                QT
            };
            let AAX;
            if GA != 0.0 {
                let QU = (IF - PU) * BD;
                let QV = if QU < IQ { 1.0 } else { 0.0 };
                let RB = if QV != 0.0 {
                    let QW = QU.exp();
                    QW
                } else {
                    let QX = (IQ.exp()) * (C + (QU - IQ));
                    QX
                };
                let QZ = QY - C;
                let RC = (FZ * QZ) + (((RA * U) * QZ) / (C + ((C + (FH * RB)).sqrt())));
                AAX = RC;
            } else {
                let RD = FZ * (QY - C);
                AAX = RD;
            }
            let RE = IU / FP;
            let RF = if RE < IQ { 1.0 } else { 0.0 };
            let RI = if RF != 0.0 {
                let RG = RE.exp();
                RG
            } else {
                let RH = (IQ.exp()) * (C + (RE - IQ));
                RH
            };
            let RJ = FR * (RI - C);
            let RK = QP / GF;
            let RL = if RK < IQ { 1.0 } else { 0.0 };
            let RO = if RL != 0.0 {
                let RM = RK.exp();
                RM
            } else {
                let RN = (IQ.exp()) * (C + (RK - IQ));
                RN
            };
            let RP = GG * (RO - C);
            let RQ = IZ / FS;
            let RR = if RQ < IQ { 1.0 } else { 0.0 };
            let RU = if RR != 0.0 {
                let RS = RQ.exp();
                RS
            } else {
                let RT = (IQ.exp()) * (C + (RQ - IQ));
                RT
            };
            let RV = FT * (RU - C);
            let RW = QP / GH;
            let RX = if RW < IQ { 1.0 } else { 0.0 };
            let SA = if RX != 0.0 {
                let RY = RW.exp();
                RY
            } else {
                let RZ = (IQ.exp()) * (C + (RW - IQ));
                RZ
            };
            let SB = GI * (SA - C);
            let SC = if (if (if GP > A { 1.0 } else { 0.0 }) != 0.0 && (if GN > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && OY != 0.0 { 1.0 } else { 0.0 };
            let ABC;
            if SC != 0.0 {
                let SD = GO * (C - (W / (U * NA)));
                let SE = if SD < IQ { 1.0 } else { 0.0 };
                let SW = if SE != 0.0 {
                    let SF = SD.exp();
                    SF
                } else {
                    let SG = (IQ.exp()) * (C + (SD - IQ));
                    SG
                };
                let SH = ID * DE;
                let SJ = V - C;
                let SL = ((ID * W) * GO) / (GK * ((((((SH * SH) + SI).sqrt()).powf((-2e0f64 - V))) * ((V * ((C - (V * V)) - ((BT * SH) * SJ))) - (((FO * SH) * SH) * (SJ + SH)))) * SK));
                let SM = if SL < -1e-3f64 { 1.0 } else { 0.0 };
                let SV;
                if SM != 0.0 {
                    let SN = if SL < IQ { 1.0 } else { 0.0 };
                    let SQ = if SN != 0.0 {
                        let SO = SL.exp();
                        SO
                    } else {
                        let SP = (IQ.exp()) * (C + (SL - IQ));
                        SP
                    };
                    let SR = (-ID) * (C + ((C - SQ) / SL));
                    SV = SR;
                } else {
                    let SU = ((ID * FE) * SL) * (C + ((SL * SS) * (C + (ST * SL))));
                    SV = SU;
                }
                let SX = (((((U * GQ) * SV) * NA) * SW) * DE) * X;
                ABC = SX;
            } else {
                ABC = A;
            }
            let SY = if (if (if GV > A { 1.0 } else { 0.0 }) != 0.0 && (if GT > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if HZ < A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let VD;
            if SY != 0.0 {
                let SZ = HZ * DG;
                let TA = (C - SZ).powf(NU);
                let TB = GU * (C - (AN / (U * TA)));
                let TC = if TB < IQ { 1.0 } else { 0.0 };
                let TP = if TC != 0.0 {
                    let TD = TB.exp();
                    TD
                } else {
                    let TE = (IQ.exp()) * (C + (TB - IQ));
                    TE
                };
                let TF = AM - C;
                let TG = ((HZ * AN) * GU) / (GR * ((((((SZ * SZ) + SI).sqrt()).powf((-2e0f64 - AM))) * ((AM * ((C - (AM * AM)) - ((BT * SZ) * TF))) - (((FO * SZ) * SZ) * (TF + SZ)))) * SK));
                let TH = if TG < -1e-3f64 { 1.0 } else { 0.0 };
                let TO;
                if TH != 0.0 {
                    let TI = if TG < IQ { 1.0 } else { 0.0 };
                    let TL = if TI != 0.0 {
                        let TJ = TG.exp();
                        TJ
                    } else {
                        let TK = (IQ.exp()) * (C + (TG - IQ));
                        TK
                    };
                    let TM = (-HZ) * (C + ((C - TL) / TG));
                    TO = TM;
                } else {
                    let TN = ((HZ * FE) * TG) * (C + ((TG * SS) * (C + (ST * TG))));
                    TO = TN;
                }
                let TQ = (((((U * GW) * TO) * TA) * TP) * DG) * AO;
                VD = TQ;
            } else {
                VD = A;
            }
            let TS = (FH * GE) / FN;
            let TT = ((U * GE) * (TR - C)) / (C + ((C + (TS * TR)).sqrt()));
            let TV = if I > A { 1.0 } else { 0.0 };
            let TW = if (if TU > A { 1.0 } else { 0.0 }) != 0.0 && TV != 0.0 { 1.0 } else { 0.0 };
            let VH;
            let VK;
            let AAS;
            if TW != 0.0 {
                let TX = TT * J;
                let TZ = (((I * U) * GE) * (TY - C)) / (C + ((C + (TS * TY)).sqrt()));
                let UA = if TU == C { 1.0 } else { 0.0 };
                let UK;
                if UA != 0.0 {
                    let UB = (I * GE) * DY;
                    let UC = IO - (BC * (U - ((UB * BD).ln())));
                    let UE = UC * UC;
                    let UF = if UC < A { 1.0 } else { 0.0 };
                    let UI = if UF != 0.0 {
                        let UG = 6.05e-3f64 / (((UE + UD).sqrt()) - UC);
                        UG
                    } else {
                        let UH = FE * (((UE + UD).sqrt()) + UC);
                        UH
                    };
                    let UJ = UI / ((UB + (TZ * DY)) + UI);
                    UK = UJ;
                } else {
                    UK = C;
                }
                let UL = UK * TZ;
                VH = TX;
                VK = UL;
                AAS = UK;
            } else {
                VH = TT;
                VK = A;
                AAS = C;
            }
            let UM = if parameters[83] == C { 1.0 } else { 0.0 };
            let VE;
            if UM != 0.0 {
                let UN = IG + HZ;
                let UQ = ((-1e0f64 * UN) * -1e0f64) * UN;
                let UR = if (-1e0f64 * UN) < A { 1.0 } else { 0.0 };
                let UZ = if UR != 0.0 {
                    let US = 5e-13f64 / (((UQ + UP).sqrt()) - (-1e0f64 * UN));
                    US
                } else {
                    let UT = FE * (((UQ + UP).sqrt()) + (-1e0f64 * UN));
                    UT
                };
                let UV = C / (C - (AZ.powf(UU)));
                let UX = AZ * UW;
                let UY = (((UV * UV) * (AZ.powf((UU - C)))) * UU) / UW;
                let VA = if UZ < UX { 1.0 } else { 0.0 };
                let VF = if VA != 0.0 {
                    let VB = C / (C - ((UZ / UW).powf(UU)));
                    VB
                } else {
                    let VC = UV + ((UZ - UX) * UY);
                    VC
                };
                VE = VF;
            } else {
                VE = C;
            }
            let VG = VD * VE;
            let VI = VH * VE;
            let VJ = RV * VE;
            let VL = VK * VE;
            let VM = (C + (NB / GZ)) + (NX / GY);
            let VO = VM * VM;
            let VP = if VM < A { 1.0 } else { 0.0 };
            let VS = if VP != 0.0 {
                let VQ = 5.000000000000001e-3f64 / (((VO + VN).sqrt()) - VM);
                VQ
            } else {
                let VR = FE * (((VO + VN).sqrt()) + VM);
                VR
            };
            let VT = DU / (VS * OR);
            let VU = if VT < R { 1.0 } else { 0.0 };
            let VV = if VU != 0.0 {
                R
            } else {
                VT
            };
            let VW = BT * VV;
            let VY = if OV > A { 1.0 } else { 0.0 };
            let ABD;
            if VY != 0.0 {
                let WA = if VZ == C { 1.0 } else { 0.0 };
                let YU;
                if WA != 0.0 {
                    let WC = if HZ < WB { 1.0 } else { 0.0 };
                    let YV;
                    if WC != 0.0 {
                        let WD = (-OV) / parameters[41];
                        let WE = if WD < IQ { 1.0 } else { 0.0 };
                        let WH = if WE != 0.0 {
                            let WF = WD.exp();
                            WF
                        } else {
                            let WG = (IQ.exp()) * (C + (WD - IQ));
                            WG
                        };
                        let WI = (WB - HZ) * WH;
                        let WL = (-WJ) * (WI.powf(WK));
                        let WM = if WL < IQ { 1.0 } else { 0.0 };
                        let WQ = if WM != 0.0 {
                            let WN = WL.exp();
                            WN
                        } else {
                            let WO = (IQ.exp()) * (C + (WL - IQ));
                            WO
                        };
                        let WR = ((WP / WJ) * WI) * WQ;
                        YV = WR;
                    } else {
                        YV = A;
                    }
                    YU = YV;
                } else {
                    let WS = if VZ == U { 1.0 } else { 0.0 };
                    let YW;
                    if WS != 0.0 {
                        let WT = if HZ < JL { 1.0 } else { 0.0 };
                        let YX;
                        if WT != 0.0 {
                            let WV = (U * parameters[45]) / (WU * WU);
                            let WW = JL - HZ;
                            let WX = ((U * (WW / NS)) / WV).sqrt();
                            let WZ = if WY == A { 1.0 } else { 0.0 };
                            let XE = if WZ != 0.0 {
                                WU
                            } else {
                                let XC = C - (FE * XA);
                                let XD = (WU * XC) * XC;
                                XD
                            };
                            let XF = (WX * XE) / (((WX * WX) + (XE * XE)).sqrt());
                            let XG = WW / XF;
                            let XH = FE * XF;
                            let XI = XH * WV;
                            let XJ = XG + (XI * NS);
                            let XQ = if WZ != 0.0 {
                                XJ
                            } else {
                                let XL = U * XK;
                                let XM = XG - (XI * (((C + XK) / (C + XL)) - (OV / (KY * (C + (XL * (C + (U * XA))))))));
                                let XN = XM - XJ;
                                let XP = FE * ((XM + XJ) + (((XN * XN) + ((((AD * XG) * XG) * XO) / KY)).sqrt()));
                                XP
                            };
                            let XR = (XQ - XG) / XQ;
                            let XS = if (XR.abs()) > 1e-7f64 { 1.0 } else { 0.0 };
                            let YY = if XS != 0.0 {
                                let XT = XH / XR;
                                let XW = (-XV) / XQ;
                                let XX = (((XU / XV) * XQ) * XT) * ((XW.exp()) - ((XW * (C + (XE / XT))).exp()));
                                XX
                            } else {
                                let XY = (XU * XE) * (((-XV) / XQ).exp());
                                XY
                            };
                            YX = YY;
                        } else {
                            YX = A;
                        }
                        YW = YX;
                    } else {
                        let XZ = if VZ == BT { 1.0 } else { 0.0 };
                        let YZ;
                        if XZ != 0.0 {
                            let YA = if HZ < WB { 1.0 } else { 0.0 };
                            let ZA;
                            if YA != 0.0 {
                                let YB = WB - HZ;
                                let YD = (YB.powf(WK)) * ((C - (OV / (YC + OV))).powf(parameters[48]));
                                let YE = if WY == A { 1.0 } else { 0.0 };
                                let YN;
                                if YE != 0.0 {
                                    YN = YD;
                                } else {
                                    let YF = (OV - parameters[51]) / YC;
                                    let YH = (YF - C) / YG;
                                    let YI = if YF < C { 1.0 } else { 0.0 };
                                    let YL = if YI != 0.0 {
                                        let YJ = C + (YG * ((C + (YH.exp())).ln()));
                                        YJ
                                    } else {
                                        let YK = YF + (YG * ((C + ((-YH).exp())).ln()));
                                        YK
                                    };
                                    let YM = YD * (YL.powf(parameters[49]));
                                    YN = YM;
                                }
                                let YO = (-WJ) * YN;
                                let YP = if YO < IQ { 1.0 } else { 0.0 };
                                let YS = if YP != 0.0 {
                                    let YQ = YO.exp();
                                    YQ
                                } else {
                                    let YR = (IQ.exp()) * (C + (YO - IQ));
                                    YR
                                };
                                let YT = ((WP / WJ) * YB) * YS;
                                ZA = YT;
                            } else {
                                ZA = A;
                            }
                            YZ = ZA;
                        } else {
                            YZ = A;
                        }
                        YW = YZ;
                    }
                    YU = YW;
                }
                let ZB = if YU > A { 1.0 } else { 0.0 };
                let ABE;
                if ZB != 0.0 {
                    let ZC = if parameters[52] == C { 1.0 } else { 0.0 };
                    let ABF;
                    if ZC != 0.0 {
                        let ZE = ZD + VW;
                        let ZG = ((BC / (OV * ZE)) + ((OS / FK) * FX)) + (ZF / ZE);
                        let ZH = if VZ == BT { 1.0 } else { 0.0 };
                        let ABG;
                        if ZH != 0.0 {
                            let ZI = (YU - ZG) / UO;
                            let ZJ = if YU < ZG { 1.0 } else { 0.0 };
                            let ZM = if ZJ != 0.0 {
                                let ZK = YU - (UO * ((C + (ZI.exp())).ln()));
                                ZK
                            } else {
                                let ZL = ZG - (UO * ((C + ((-ZI).exp())).ln()));
                                ZL
                            };
                            let ZN = OV * ZM;
                            ABG = ZN;
                        } else {
                            let ZO = ((OV * YU) * ZG) / (YU + ZG);
                            ABG = ZO;
                        }
                        ABF = ABG;
                    } else {
                        let ZP = OV * YU;
                        ABF = ZP;
                    }
                    ABE = ABF;
                } else {
                    ABE = A;
                }
                ABD = ABE;
            } else {
                ABD = A;
            }
            let ZQ = if OC > A { 1.0 } else { 0.0 };
            if ZQ != 0.0 {
            } else {
            }
            let ZR = if IF < MS { 1.0 } else { 0.0 };
            if ZR != 0.0 {
            } else {
            }
            let ZS = HB * FL;
            let ZT = AD * DK;
            let ZU = if IN < NJ { 1.0 } else { 0.0 };
            if ZU != 0.0 {
            } else {
            }
            let ZV = C - parameters[76];
            let ZW = (IO - NJ) / ZT;
            let ZX = if IO < NJ { 1.0 } else { 0.0 };
            let AAA = if ZX != 0.0 {
                let ZY = IO - (ZT * ((C + (ZW.exp())).ln()));
                ZY
            } else {
                let ZZ = NJ - (ZT * ((C + ((-ZW).exp())).ln()));
                ZZ
            };
            let AAB = ((DN * ((NH * ((NV * (C - ((C - (AAA / DK)).powf(NU)))) + (NI * (IO - AAA)))) + (DO * IO))) * ZV) * I;
            let AAC = if (ID / (parameters[84] * BC)) < IQ { 1.0 } else { 0.0 };
            if AAC != 0.0 {
            } else {
            }
            let AAD = ((FH * HD) * BC) / EF;
            let AAE = if parameters[78] == A { 1.0 } else { 0.0 };
            if AAE != 0.0 {
            } else {
                let AAG = if (((IN - AAF) / parameters[90]) * BD) < IQ { 1.0 } else { 0.0 };
                if AAG != 0.0 {
                } else {
                }
            }
            let AAH = if (if (if TU == C { 1.0 } else { 0.0 }) != 0.0 || (if TU == BT { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && TV != 0.0 { 1.0 } else { 0.0 };
            let ABP;
            if AAH != 0.0 {
                let AAT;
                if AAE != 0.0 {
                    let AAI = NY * TY;
                    let AAK = FH * AAJ;
                    let AAL = (((FE * I) * HF) * ((ZS * ((AAI - NY) / (C + ((C + AAI).sqrt())))) + (AAD * (AAK / (C + ((C + AAK).sqrt())))))) / HE;
                    AAT = AAL;
                } else {
                    let AAM = (IO - AAF) * BD;
                    let AAN = if AAM < IQ { 1.0 } else { 0.0 };
                    let AAQ = if AAN != 0.0 {
                        let AAO = AAM.exp();
                        AAO
                    } else {
                        let AAP = (IQ.exp()) * (C + (AAM - IQ));
                        AAP
                    };
                    let AAR = ((((U * I) * GE) * HG) * TY) / (C + ((C + (FH * AAQ)).sqrt()));
                    AAT = AAR;
                }
                let AAU = AAS * AAT;
                ABP = AAU;
            } else {
                ABP = A;
            }
            let AAV = if parameters[6] == C { 1.0 } else { 0.0 };
            if AAV != 0.0 {
                let AAW = if MU < A { 1.0 } else { 0.0 };
                if AAW != 0.0 {
                } else {
                }
            } else {
            }
            let AAY = (AAX + RP) + SB;
            let ABB = AAZ + RJ;
            if GA != 0.0 {
            } else {
            }
            let ABH = ((B * IJ) / ZD) * Q;
            let ABI = (B * parameters[68]) * IK;
            let ABJ = 0e0f64 * Q;
            let ABK = (B * parameters[77]) * IL;
            let ABL = 0e0f64 * Q;
            let ABM = (B * VL) * Q;
            let ABQ = B * (AAB + ABP);
            let ABR = 0e0f64 * Q;
            if HR != 0.0 {
            } else {
            }
            if HU != 0.0 {
            } else {
            }
            let ABW = 5.5224904e-23f64 * L;
            let ABX = ABW / ZF;
            let ABY = ABW / ZD;
            let ABZ = ABW * ABN;
            let ACA = ABW * ABS;
            let ACB = ABW * ABU;
            let ACC = ((ABW / VW) * ((FH * VX) + 5e0f64)) * SS;
            let ACD = (OU + OT) / OS;
            let ACE = 3.2043836e-19f64 * (ACD.abs());
            let ACF = if parameters[129] > A { 1.0 } else { 0.0 };
            let ACH = if ACF != 0.0 {
                let ACG = (ABD / ACD).abs();
                ACG
            } else {
                A
            };
            let ACI = (3.2043836e-19f64 * ABD) * (ACH + C);
            let ACJ = if ACD > A { 1.0 } else { 0.0 };
            if ACJ != 0.0 {
            } else {
            }
            let ACL = if ACK == C { 1.0 } else { 0.0 };
            if ACL != 0.0 {
            } else {
                let ACM = if ACK == U { 1.0 } else { 0.0 };
                if ACM != 0.0 {
                } else {
                }
            }
            let ACN = 3.2043836e-19f64 * ((((ABB - ABC) + PP) + PH).abs());
            let ACO = AAZ + AAX;
            let ACR = ACP * ((ACO.abs()).powf(ACQ));
            let ACS = if ACO < A { 1.0 } else { 0.0 };
            let ADY = if ACS != 0.0 {
                let ACT = -ACR;
                ACT
            } else {
                ACR
            };
            let ACU = (RJ + RP) + SB;
            let ACV = parameters[128] * ((ACU.abs()).powf(parameters[126]));
            let ACW = if ACU < A { 1.0 } else { 0.0 };
            let AEA = if ACW != 0.0 {
                let ACX = -ACV;
                ACX
            } else {
                ACV
            };
            let ACY = 3.2043836e-19f64 * (AAY.abs());
            let ACZ = VJ.abs();
            let ADA = 3.2043836e-19f64 * ACZ;
            let ADB = ACP * (ACZ.powf(ACQ));
            let ADC = if VJ < A { 1.0 } else { 0.0 };
            let AEE = if ADC != 0.0 {
                let ADD = -ADB;
                ADD
            } else {
                ADB
            };
            let ADE = 3.2043836e-19f64 * (VG.abs());
            let ADF = VI.abs();
            let ADG = 3.2043836e-19f64 * ADF;
            let ADH = C - (TU * I);
            let ADI = (ACP * ADH) * ((ADF / ADH).powf(ACQ));
            let ADJ = if VI < A { 1.0 } else { 0.0 };
            let AEH = if ADJ != 0.0 {
                let ADK = -ADI;
                ADK
            } else {
                ADI
            };
            let ADL = VL.abs();
            let ADM = (3.2043836e-19f64 * ADL) * TU;
            let ADN = if I == A { 1.0 } else { 0.0 };
            let ADQ = if ADN != 0.0 {
                A
            } else {
                let ADO = ((ACP * TU) * I) * ((ADL / I).powf(ACQ));
                ADO
            };
            let ADP = if VL < A { 1.0 } else { 0.0 };
            let AEK = if ADP != 0.0 {
                let ADR = -ADQ;
                ADR
            } else {
                ADQ
            };
            let ADS = ACE * Q;
            let ADT = ACI * Q;
            let ADU = ACN * Q;
            let ADV = ABX * Q;
            let ADW = ABY * Q;
            let ADX = ACC * Q;
            let ADZ = ADY * Q;
            let AEB = AEA * Q;
            let AEC = ACY * Q;
            let AED = ADA * Q;
            let AEF = AEE * Q;
            let AEG = ADG * Q;
            let AEI = AEH * Q;
            let AEJ = ADM * Q;
            let AEL = AEK * Q;
            let AEY;
            let AEZ;
            let AFA;
            let AFB;
            if GA != 0.0 {
                let AEM = ADE * Q;
                AEY = C;
                AEZ = AEM;
                AFA = A;
                AFB = A;
            } else {
                let AEN = ADE * Q;
                AEY = A;
                AEZ = A;
                AFA = C;
                AFB = AEN;
            }
            let AFC;
            let AFE;
            let AFG;
            let AFI;
            let AFK;
            let AFM;
            let AFO;
            let AFQ;
            let AFS;
            let AFU;
            let AFW;
            let AFY;
            let AGA;
            let AGC;
            let AGE;
            let AGG;
            if HR != 0.0 {
                let AFD;
                let AFF;
                let AFH;
                let AFJ;
                let AFL;
                let AFN;
                let AFP;
                let AFR;
                let AFT;
                let AFV;
                if HU != 0.0 {
                    let AEO = ABZ * Q;
                    let AEP = ACA * Q;
                    let AEQ = ACB * Q;
                    AFD = C;
                    AFF = AEO;
                    AFH = C;
                    AFJ = AEP;
                    AFL = C;
                    AFN = AEQ;
                    AFP = A;
                    AFR = A;
                    AFT = A;
                    AFV = A;
                } else {
                    let AER = ABZ * Q;
                    let AES = ACA * Q;
                    AFD = A;
                    AFF = A;
                    AFH = A;
                    AFJ = A;
                    AFL = A;
                    AFN = A;
                    AFP = C;
                    AFR = AER;
                    AFT = C;
                    AFV = AES;
                }
                AFC = AFD;
                AFE = AFF;
                AFG = AFH;
                AFI = AFJ;
                AFK = AFL;
                AFM = AFN;
                AFO = AFP;
                AFQ = AFR;
                AFS = AFT;
                AFU = AFV;
                AFW = A;
                AFY = A;
                AGA = A;
                AGC = A;
                AGE = A;
                AGG = A;
            } else {
                let AFX;
                let AFZ;
                let AGB;
                let AGD;
                let AGF;
                let AGH;
                if HU != 0.0 {
                    let AET = ABZ * Q;
                    let AEU = ACB * Q;
                    AFX = C;
                    AFZ = AET;
                    AGB = C;
                    AGD = AEU;
                    AGF = A;
                    AGH = A;
                } else {
                    let AEV = ABZ * Q;
                    AFX = A;
                    AFZ = A;
                    AGB = A;
                    AGD = A;
                    AGF = C;
                    AGH = AEV;
                }
                AFC = A;
                AFE = A;
                AFG = A;
                AFI = A;
                AFK = A;
                AFM = A;
                AFO = A;
                AFQ = A;
                AFS = A;
                AFU = A;
                AFW = AFX;
                AFY = AFZ;
                AGA = AGB;
                AGC = AGD;
                AGE = AGF;
                AGG = AGH;
            }
            let AEW = if ((((ABH + ABJ) + ABL) + ABM) + ABR) == A { 1.0 } else { 0.0 };
            if AEW != 0.0 {
            } else {
            }
            let AEX = if Q != C { 1.0 } else { 0.0 };
            if AEX != 0.0 {
            } else {
            }
        {
            let psd = ADS;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 0, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 0, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(0, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = ADT;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(1, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = ADU;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 2, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 2, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(2, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = ADV;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 3, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 3, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(3, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = ADW;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 4, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 4, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(4, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = ADX;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 5, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 5, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(5, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = ADZ;
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
            let psd = AEB;
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
            let psd = AEC;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 8, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 8, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(8, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = AED;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 9, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 9, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(9, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = AEF;
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
            let psd = AEG;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 11, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 11, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(11, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = AEI;
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
            let psd = AEJ;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 13, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 13, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(13, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = AEL;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 14, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = Some(C);
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 14, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 14, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(14, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if AEY == 0.0 {
            if !visitor.visit(15, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = AEZ;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 15, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 15, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(15, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if AFA == 0.0 {
            if !visitor.visit(16, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = AFB;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 16, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 16, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(16, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if AFC == 0.0 {
            if !visitor.visit(17, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = AFE;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 17, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 17, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(17, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if AFG == 0.0 {
            if !visitor.visit(18, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = AFI;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 18, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 18, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(18, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if AFK == 0.0 {
            if !visitor.visit(19, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = AFM;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 19, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 19, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(19, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if AFO == 0.0 {
            if !visitor.visit(20, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = AFQ;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 20, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 20, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(20, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if AFS == 0.0 {
            if !visitor.visit(21, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = AFU;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 21, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 21, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(21, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if AFW == 0.0 {
            if !visitor.visit(22, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = AFY;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 22, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 22, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(22, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if AGA == 0.0 {
            if !visitor.visit(23, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = AGC;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 23, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 23, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(23, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if AGE == 0.0 {
            if !visitor.visit(24, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = AGG;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 24, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 24, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(24, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        Ok(())
    }
}
