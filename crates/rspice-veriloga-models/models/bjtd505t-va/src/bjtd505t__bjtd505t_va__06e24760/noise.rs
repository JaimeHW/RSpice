#![allow(dead_code, non_snake_case, unused_parens, unused_variables)]

use super::state::Instance;
use rspice_veriloga_runtime::GeneratedEvalContext;
pub use rspice_veriloga_runtime::{GeneratedNoiseDescriptor, GeneratedNoiseEndpoint, GeneratedNoiseEvaluation, GeneratedNoiseEvaluationError, GeneratedNoiseEvaluationRef, GeneratedNoiseKind, GeneratedNoiseVisitor};

pub static NOISE_SOURCES: [GeneratedNoiseDescriptor; 25] = [
    GeneratedNoiseDescriptor { mechanism: "WHITE_NOI_GND_IN", label: Some("in"), kind: GeneratedNoiseKind::White, equation: 28, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(11), name: "noi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: None, name: "0", is_internal: false }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C2_B2_IAVL", label: Some("iavl"), kind: GeneratedNoiseKind::White, equation: 33, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(8), name: "c2", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(6), name: "b2", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_B2_E1_IB2E1", label: Some("ib2e1"), kind: GeneratedNoiseKind::White, equation: 34, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(6), name: "b2", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(4), name: "e1", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_E_E1_RE", label: Some("re"), kind: GeneratedNoiseKind::White, equation: 35, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(2), name: "e", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(4), name: "e1", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_B_B1_RBC", label: Some("rbc"), kind: GeneratedNoiseKind::White, equation: 36, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(1), name: "b", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(5), name: "b1", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_B1_B2_RBV", label: Some("rbv"), kind: GeneratedNoiseKind::White, equation: 37, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "b1", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(6), name: "b2", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_B2_E1_IB2E1_F", label: Some("ib2e1_f"), kind: GeneratedNoiseKind::Flicker, equation: 38, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(6), name: "b2", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(4), name: "e1", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_B1_E1_IB1E1_F", label: Some("ib1e1_f"), kind: GeneratedNoiseKind::Flicker, equation: 39, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "b1", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(4), name: "e1", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_B1_E1_IB1E1", label: Some("ib1e1"), kind: GeneratedNoiseKind::White, equation: 40, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "b1", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(4), name: "e1", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_B1_C4_IB3", label: Some("ib3"), kind: GeneratedNoiseKind::White, equation: 41, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "b1", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(10), name: "c4", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_B1_C4_IB3_F", label: Some("ib3_f"), kind: GeneratedNoiseKind::Flicker, equation: 42, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "b1", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(10), name: "c4", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_B1_C4_IEX", label: Some("iex"), kind: GeneratedNoiseKind::White, equation: 43, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "b1", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(10), name: "c4", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_B1_C4_IEX_F", label: Some("iex_f"), kind: GeneratedNoiseKind::Flicker, equation: 44, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "b1", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(10), name: "c4", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_B_C3_XIEX", label: Some("xiex"), kind: GeneratedNoiseKind::White, equation: 45, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(1), name: "b", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(9), name: "c3", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_B_C3_XIEX_F", label: Some("xiex_f"), kind: GeneratedNoiseKind::Flicker, equation: 46, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(1), name: "b", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(9), name: "c3", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C1_B2_IZTCB", label: Some("iztcb"), kind: GeneratedNoiseKind::White, equation: 47, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(7), name: "c1", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(6), name: "b2", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C2_B2_IZTCB", label: Some("iztcb"), kind: GeneratedNoiseKind::White, equation: 48, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(8), name: "c2", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(6), name: "b2", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C_C3_RCC", label: Some("rcc"), kind: GeneratedNoiseKind::White, equation: 49, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(0), name: "c", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(9), name: "c3", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C3_C4_RCBLX", label: Some("rcblx"), kind: GeneratedNoiseKind::White, equation: 50, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(9), name: "c3", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(10), name: "c4", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C4_C1_RCBLI", label: Some("rcbli"), kind: GeneratedNoiseKind::White, equation: 51, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(10), name: "c4", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(7), name: "c1", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C_C3_RCC", label: Some("rcc"), kind: GeneratedNoiseKind::White, equation: 52, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(0), name: "c", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(9), name: "c3", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C3_C1_RCBLX", label: Some("rcblx"), kind: GeneratedNoiseKind::White, equation: 53, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(9), name: "c3", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(7), name: "c1", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C_C4_RCC", label: Some("rcc"), kind: GeneratedNoiseKind::White, equation: 54, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(0), name: "c", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(10), name: "c4", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C4_C1_RCBLI", label: Some("rcbli"), kind: GeneratedNoiseKind::White, equation: 55, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(10), name: "c4", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(7), name: "c1", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C_C1_RCC", label: Some("rcc"), kind: GeneratedNoiseKind::White, equation: 56, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(0), name: "c", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(7), name: "c1", is_internal: true }, table_len: 0, table_log_interp: false },
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
            let I = parameters[32];
            let M = parameters[141];
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
            let BA = node_potentials[3];
            let BE = parameters[124];
            let BK = 8.617086918058125e-5f64;
            let CC = 3e0f64;
            let CE = parameters[104];
            let CK = parameters[109];
            let DS = parameters[74];
            let DY = parameters[96];
            let EB = parameters[97];
            let EC = parameters[95];
            let EG = parameters[56];
            let EI = parameters[57];
            let EL = parameters[58];
            let EN = parameters[98];
            let EP = parameters[121];
            let ER = parameters[9];
            let EZ = parameters[122];
            let FB = parameters[10];
            let FK = 1e-6f64;
            let FN = 5e-1f64;
            let FQ = 4e0f64;
            let FR = parameters[120];
            let FV = parameters[102];
            let FX = 6e0f64;
            let FY = parameters[20];
            let GB = parameters[31];
            let GE = parameters[16];
            let GH = parameters[18];
            let GO = parameters[22];
            let GQ = parameters[137];
            let GW = parameters[34];
            let GY = parameters[33];
            let HC = parameters[36];
            let HE = parameters[35];
            let HJ = parameters[86];
            let HL = parameters[87];
            let HV = parameters[91];
            let IG = node_potentials[6];
            let IH = node_potentials[7];
            let IJ = node_potentials[8];
            let IL = node_potentials[4];
            let IN = node_potentials[5];
            let IR = node_potentials[1];
            let IV = node_potentials[10];
            let IZ = parameters[138];
            let KO = parameters[140];
            let KU = 1e2f64;
            let LH = parameters[61];
            let LI = parameters[60];
            let LM = parameters[62];
            let MH = parameters[139];
            let NL = parameters[73];
            let OT = 1.0000000000000002e-2f64;
            let PF = 1e-4f64;
            let PR = parameters[145];
            let QJ = 4e1f64;
            let QU = parameters[92];
            let SR = 1e-30f64;
            let ST = 1.6666666666666666e-1f64;
            let TB = 3.333333333333333e-1f64;
            let TC = 2.5e-1f64;
            let UD = parameters[5];
            let UM = 1.21e-2f64;
            let UX = 1e-6f64;
            let UY = 1e-12f64;
            let VD = parameters[81];
            let VF = parameters[80];
            let VW = 1.0000000000000002e-2f64;
            let WI = parameters[38];
            let WK = parameters[43];
            let WT = parameters[40];
            let WY = parameters[39];
            let XD = parameters[44];
            let XH = parameters[7];
            let XT = parameters[46];
            let YL = parameters[47];
            let YP = parameters[50];
            let ACX = parameters[130];
            let ADC = parameters[127];
            let ADD = parameters[125];
            let D = if B == C { 1.0 } else { 0.0 };
            let HS;
            let YD;
            if D != 0.0 {
                HS = F;
                YD = E;
            } else {
                HS = H;
                YD = G;
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
            let BQ = if AF != 0.0 {
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
            let BW = if AU != 0.0 {
                let AV = AC + (AD * ((C + (AT.exp())).ln()));
                AV
            } else {
                let AW = AS + (AD * ((C + ((-AT).exp())).ln()));
                AW
            };
            let AX = C / AP;
            let AY = C / AL;
            let AZ = C - (C / parameters[82]);
            let BB = if BA < A { 1.0 } else { 0.0 };
            let BD = if BB != 0.0 {
                let BC = -((C - BA).ln());
                BC
            } else {
                BA
            };
            let BF = if BD < BE { 1.0 } else { 0.0 };
            let BH = if BF != 0.0 {
                BD
            } else {
                let BG = BE + ((C + (BD - BE)).ln());
                BG
            };
            let BI = L + BH;
            let BJ = BI / K;
            let BL = BK * BI;
            let BM = C / BL;
            let BN = BM - (C / (BK * K));
            let BO = BI - K;
            let BP = BJ.ln();
            let BR = BQ - (((Z * BI) * BI) / (BI + AA));
            let BS = (BR - AC) / AD;
            let BT = if BR < AC { 1.0 } else { 0.0 };
            let GT = if BT != 0.0 {
                let BU = AC + (AD * ((C + (BS.exp())).ln()));
                BU
            } else {
                let BV = BR + (AD * ((C + ((-BS).exp())).ln()));
                BV
            };
            let BX = BW - (((AQ * BI) * BI) / (BI + AR));
            let BY = (BX - AC) / AD;
            let BZ = if BX < AC { 1.0 } else { 0.0 };
            let HA = if BZ != 0.0 {
                let CA = AC + (AD * ((C + (BY.exp())).ln()));
                CA
            } else {
                let CB = BX + (AD * ((C + ((-BY).exp())).ln()));
                CB
            };
            let CD = C - BJ;
            let CF = (((-3e0f64 * BL) * BP) + (AJ * BJ)) + (CD * CE);
            let CG = (AC - CF) / BL;
            let CH = if AC < CF { 1.0 } else { 0.0 };
            let DM = if CH != 0.0 {
                let CI = CF + (BL * ((C + (CG.exp())).ln()));
                CI
            } else {
                let CJ = AC + (BL * ((C + ((-CG).exp())).ln()));
                CJ
            };
            let CL = CD * CK;
            let CM = (((-3e0f64 * BL) * BP) + (parameters[63] * BJ)) + CL;
            let CN = (AC - CM) / BL;
            let CO = if AC < CM { 1.0 } else { 0.0 };
            let JU = if CO != 0.0 {
                let CP = CM + (BL * ((C + (CN.exp())).ln()));
                CP
            } else {
                let CQ = AC + (BL * ((C + ((-CN).exp())).ln()));
                CQ
            };
            let CR = (((-3e0f64 * BL) * BP) + (parameters[79] * BJ)) + CL;
            let CS = (AC - CR) / BL;
            let CT = if AC < CR { 1.0 } else { 0.0 };
            let ABE = if CT != 0.0 {
                let CU = CR + (BL * ((C + (CS.exp())).ln()));
                CU
            } else {
                let CV = AC + (BL * ((C + ((-CS).exp())).ln()));
                CV
            };
            let CW = AL * BJ;
            let CX = (((-3e0f64 * BL) * BP) + CW) + CL;
            let CY = (AC - CX) / BL;
            let CZ = if AC < CX { 1.0 } else { 0.0 };
            let DT = if CZ != 0.0 {
                let DA = CX + (BL * ((C + (CY.exp())).ln()));
                DA
            } else {
                let DB = AC + (BL * ((C + ((-CY).exp())).ln()));
                DB
            };
            let DC = (((-3e0f64 * BL) * BP) + CW) + CL;
            let DD = (AC - DC) / BL;
            let DE = if AC < DC { 1.0 } else { 0.0 };
            let DO = if DE != 0.0 {
                let DF = DC + (BL * ((C + (DD.exp())).ln()));
                DF
            } else {
                let DG = AC + (BL * ((C + ((-DD).exp())).ln()));
                DG
            };
            let DH = (((-3e0f64 * BL) * BP) + (parameters[26] * BJ)) + (CD * parameters[108]);
            let DI = (AC - DH) / BL;
            let DJ = if AC < DH { 1.0 } else { 0.0 };
            let QD = if DJ != 0.0 {
                let DK = DH + (BL * ((C + (DI.exp())).ln()));
                DK
            } else {
                let DL = AC + (BL * ((C + ((-DI).exp())).ln()));
                DL
            };
            let DN = C / DM;
            let DP = C / DO;
            let DQ = (AJ * DN).powf(V);
            let DR = (AL * DP).powf(AM);
            let DU = ((C - DS) * ((AL / DT).powf(AM))) + DS;
            let DV = C / DU;
            let DW = parameters[69] * DU;
            let DX = DS * DV;
            let DZ = parameters[53] * ((BP * DY).exp());
            let EA = if DZ < R { 1.0 } else { 0.0 };
            let ZO = if EA != 0.0 {
                R
            } else {
                DZ
            };
            let ED = parameters[55] * ((BP * (EB - EC)).exp());
            let EE = parameters[54] * ((BP * parameters[100]).exp());
            let EF = if EE < R { 1.0 } else { 0.0 };
            let ZM = if EF != 0.0 {
                R
            } else {
                EE
            };
            let EH = EG * ((BP * parameters[101]).exp());
            let EJ = (BP * parameters[103]).exp();
            let EK = EI * EJ;
            let EM = EL * EJ;
            let EO = parameters[59] * ((BP * EN).exp());
            let EQ = if EP != A { 1.0 } else { 0.0 };
            let FS;
            if EQ != 0.0 {
                let ES = ER * (C + (BO * EP));
                let ET = (ES - C) / T;
                let EU = if ES < C { 1.0 } else { 0.0 };
                let EX = if EU != 0.0 {
                    let EV = C + (T * ((C + (ET.exp())).ln()));
                    EV
                } else {
                    let EW = ES + (T * ((C + ((-ET).exp())).ln()));
                    EW
                };
                let EY = EX - 6.931471805599453e-4f64;
                FS = EY;
            } else {
                FS = ER;
            }
            let FA = if EZ != A { 1.0 } else { 0.0 };
            let OM;
            if FA != 0.0 {
                let FC = FB * (C + (BO * EZ));
                let FD = (FC - C) / T;
                let FE = if FC < C { 1.0 } else { 0.0 };
                let FH = if FE != 0.0 {
                    let FF = C + (T * ((C + (FD.exp())).ln()));
                    FF
                } else {
                    let FG = FC + (T * ((C + ((-FD).exp())).ln()));
                    FG
                };
                let FI = FH - 6.931471805599453e-4f64;
                OM = FI;
            } else {
                OM = FB;
            }
            let FJ = parameters[42] * (C + (parameters[123] * BO));
            let FL = FJ * FJ;
            let FM = if FJ < A { 1.0 } else { 0.0 };
            let WS = if FM != 0.0 {
                let FO = 5e-7f64 / (((FL + FK).sqrt()) - FJ);
                FO
            } else {
                let FP = FN * (((FL + FK).sqrt()) + FJ);
                FP
            };
            let FT = (parameters[8] * (((BP * (((FQ - EB) - EC) + FR)) / FS).exp())) * ((((-CE) * BN) / FS).exp());
            let FU = parameters[11] * ((BP * (C - EB)).exp());
            let FW = parameters[29] * ((BP * (C - FV)).exp());
            let FZ = (-parameters[112]) * BN;
            let GA = (parameters[19] * ((BP * (FX - (U * FY))).exp())) * ((FZ / FY).exp());
            let GC = (parameters[30] * ((BP * (FX - (U * GB))).exp())) * ((((-CK) * BN) / GB).exp());
            let GD = BP * ((FQ - DY) + FR);
            let GF = (-parameters[110]) * BN;
            let GG = (parameters[15] * ((GD / GE).exp())) * ((GF / GE).exp());
            let GI = (parameters[17] * ((GD / GH).exp())) * ((GF / GH).exp());
            let GJ = if parameters[23] == C { 1.0 } else { 0.0 };
            let QP;
            let QR;
            let RJ;
            if GJ != 0.0 {
                let GK = parameters[24] * ((((-parameters[106]) * BN) / GE).exp());
                let GL = parameters[27] * (((-parameters[105]) * BN).exp());
                let GM = parameters[25] * ((((-parameters[107]) * BN) / GH).exp());
                QP = GK;
                QR = GL;
                RJ = GM;
            } else {
                QP = A;
                QR = A;
                RJ = A;
            }
            let GN = (parameters[28] * ((BP * ((FQ - FV) + FR)).exp())) * (((-parameters[111]) * BN).exp());
            let GP = (parameters[21] * ((BP * (FX - (U * GO))).exp())) * ((FZ / GO).exp());
            let GR = (parameters[136] * ((BP * (FQ / GQ)).exp())) * ((FZ / GQ).exp());
            let GS = (parameters[142] * (BJ.sqrt())) * ((parameters[144] * BO).exp());
            let GU = (GT * AI).powf(-5e-1f64);
            let GV = C / DQ;
            let GX = (((((((GW * GT) * GT) * GU) * GV) * AJ) * DN) * AI) * AI;
            let GZ = ((((((GY * GU) * DM) * DM) * AK) * AK) * DQ) * ((GW - GX).exp());
            let HB = (HA * AX).powf(-5e-1f64);
            let HD = (((((((HC * HA) * HA) * HB) * (C / DR)) * AL) * DP) * AX) * AX;
            let HF = ((((((HE * HB) * DO) * DO) * AY) * AY) * DR) * ((HC - HD).exp());
            let HG = (BP * EC).exp();
            let HH = (parameters[13] * HG) * DV;
            let HI = (parameters[12] * HG) * GV;
            let HK = HJ * ((BP * ((EC + EB) - C)).exp());
            let HM = HL * ((BP * (EN - C)).exp());
            let HN = HK + HM;
            let HO = (parameters[88] * HN) / (HJ + HL);
            let HP = parameters[89] * ((BP * (parameters[99] - C)).exp());
            let HQ = BI - 3e2f64;
            let HR = if BI < 5.25e2f64 { 1.0 } else { 0.0 };
            let YE = if HR != 0.0 {
                let HT = HS * ((C + (7.2e-4f64 * HQ)) - ((1.6e-6f64 * HQ) * HQ));
                HT
            } else {
                let HU = HS * 1.081e0f64;
                HU
            };
            let HW = HV * HG;
            let HX = if EG > A { 1.0 } else { 0.0 };
            let AAE;
            if HX != 0.0 {
                let HY = C / EH;
                let HZ = if HY > S { 1.0 } else { 0.0 };
                let AAF = if HZ != 0.0 {
                    S
                } else {
                    HY
                };
                AAE = AAF;
            } else {
                AAE = A;
            }
            let IA = if EI > A { 1.0 } else { 0.0 };
            let AAG;
            if IA != 0.0 {
                let IB = C / EK;
                let IC = if IB > S { 1.0 } else { 0.0 };
                let AAH = if IC != 0.0 {
                    S
                } else {
                    IB
                };
                AAG = AAH;
            } else {
                AAG = A;
            }
            let ID = if EL > A { 1.0 } else { 0.0 };
            let AAI;
            if ID != 0.0 {
                let IE = C / EM;
                let IF = if IE > S { 1.0 } else { 0.0 };
                let AAJ = if IF != 0.0 {
                    S
                } else {
                    IE
                };
                AAI = AAJ;
            } else {
                AAI = A;
            }
            let II = B * (IG - IH);
            let IK = B * (IG - IJ);
            let IM = B * (IG - IL);
            let IO = B * (IN - IL);
            let IP = B * (IN - IG);
            let IQ = B * (IH - IJ);
            let IS = B * (IR - IN);
            let IT = B * (IR - node_potentials[2]);
            let IU = B * (IR - node_potentials[0]);
            let IW = ((IP + IK) - IQ) - (B * (IV - IH));
            let IX = IU + ((((-IU) + IS) + IW) - (B * (node_potentials[9] - IV)));
            let IY = IK * BM;
            let JA = if IY < IZ { 1.0 } else { 0.0 };
            let MU = if JA != 0.0 {
                let JB = IY.exp();
                JB
            } else {
                let JC = (IZ.exp()) * (C + (IY - IZ));
                JC
            };
            let JD = IM * BM;
            let JE = JD / FS;
            let JF = if JE < IZ { 1.0 } else { 0.0 };
            let OI = if JF != 0.0 {
                let JG = JE.exp();
                JG
            } else {
                let JH = (IZ.exp()) * (C + (JE - IZ));
                JH
            };
            let JI = IW * BM;
            let JJ = if JI < IZ { 1.0 } else { 0.0 };
            let UA = if JJ != 0.0 {
                let JK = JI.exp();
                JK
            } else {
                let JL = (IZ.exp()) * (C + (JI - IZ));
                JL
            };
            let JM = IP * BM;
            let JN = if JM < IZ { 1.0 } else { 0.0 };
            let WG = if JN != 0.0 {
                let JO = JM.exp();
                JO
            } else {
                let JP = (IZ.exp()) * (C + (JM - IZ));
                JP
            };
            let JQ = IX * BM;
            let JR = if JQ < IZ { 1.0 } else { 0.0 };
            let UH = if JR != 0.0 {
                let JS = JQ.exp();
                JS
            } else {
                let JT = (IZ.exp()) * (C + (JQ - IZ));
                JT
            };
            let JV = (IX - JU) * BM;
            let JW = if JV < IZ { 1.0 } else { 0.0 };
            let ABI = if JW != 0.0 {
                let JX = JV.exp();
                JX
            } else {
                let JY = (IZ.exp()) * (C + (JV - IZ));
                JY
            };
            let JZ = if ((IW - JU) * BM) < IZ { 1.0 } else { 0.0 };
            if JZ != 0.0 {
            } else {
            }
            let KA = (IK - JU) * BM;
            let KB = if KA < IZ { 1.0 } else { 0.0 };
            let KI = if KB != 0.0 {
                let KC = KA.exp();
                KC
            } else {
                let KD = (IZ.exp()) * (C + (KA - IZ));
                KD
            };
            let KE = (II - JU) * BM;
            let KF = if KE < IZ { 1.0 } else { 0.0 };
            let KK = if KF != 0.0 {
                let KG = KE.exp();
                KG
            } else {
                let KH = (IZ.exp()) * (C + (KE - IZ));
                KH
            };
            let KJ = (C + (FQ * KI)).sqrt();
            let KL = (C + (FQ * KK)).sqrt();
            let KM = C + KL;
            let KN = (U * KK) / KM;
            let KP = if KN < KO { 1.0 } else { 0.0 };
            let LW = if KP != 0.0 {
                KO
            } else {
                KN
            };
            let KQ = KJ + C;
            let KR = BL * ((KJ - KL) - ((KQ / KM).ln()));
            let KS = (KR + IQ) / EO;
            let KT = if KS > A { 1.0 } else { 0.0 };
            let NO;
            let NV;
            let OB;
            let OL;
            let XJ;
            let XX;
            if KT != 0.0 {
                let KV = if II < KU { 1.0 } else { 0.0 };
                let KY = if KV != 0.0 {
                    II
                } else {
                    let KW = KU + ((C + (II - KU)).ln());
                    KW
                };
                let KX = (FN * KS) * EO;
                let KZ = (JU + ((U * BL) * (((KX * BM) + C).ln()))) - KY;
                let LA = 2e-1f64 * JU;
                let LB = LA * LA;
                let LC = KZ * KZ;
                let LD = if KZ < A { 1.0 } else { 0.0 };
                let LG = if LD != 0.0 {
                    let LE = (FN * LB) / (((LC + LB).sqrt()) - KZ);
                    LE
                } else {
                    let LF = FN * (((LC + LB).sqrt()) + KZ);
                    LF
                };
                let LJ = LH * LI;
                let LK = (LG * (LG + LJ)) / (LI * (LG + (LH * EO)));
                let LL = KS / LK;
                let LN = (LL - C) / LM;
                let LO = if LL < C { 1.0 } else { 0.0 };
                let LR = if LO != 0.0 {
                    let LP = C + (LM * ((C + (LN.exp())).ln()));
                    LP
                } else {
                    let LQ = LL + (LM * ((C + ((-LN).exp())).ln()));
                    LQ
                };
                let LS = LR / (C + (LM * ((C + ((-1e0f64 / LM).exp())).ln())));
                let LT = LG / LJ;
                let LU = C + LT;
                let LV = (C + ((C + (((FQ * LS) * LT) * LU)).sqrt())) / ((U * LS) * LU);
                let LX = LW * LV;
                let LY = ((C - LV) + LX) / (C + LX);
                let LZ = (KX * LY) * BM;
                let MA = (U * LZ) + (LW * ((LW + LZ) + C));
                let MB = FN * (LZ - C);
                let MC = (MB * MB) + MA;
                let MD = if LZ >= C { 1.0 } else { 0.0 };
                let MG = if MD != 0.0 {
                    let ME = MB + (MC.sqrt());
                    ME
                } else {
                    let MF = MA / ((MC.sqrt()) - MB);
                    MF
                };
                let MI = if MG < MH { 1.0 } else { 0.0 };
                let MJ = if MI != 0.0 {
                    MH
                } else {
                    MG
                };
                let MK = (MJ * (MJ + C)) * ((JU * BM).exp());
                let ML = (FN * LI) * (KS - LH);
                let MM = ML + (((ML * ML) + (((LI * EO) * LH) * KS)).sqrt());
                let MN = if parameters[72] == A { 1.0 } else { 0.0 };
                let NW = if MN != 0.0 {
                    let MO = DT * AD;
                    MO
                } else {
                    let MP = DT * (AD + ((U * KS) / (KS + LK)));
                    MP
                };
                let MQ = LH + KS;
                let MR = (LH * KS) / MQ;
                let MS = LH / MQ;
                NO = MM;
                NV = NW;
                OB = MS;
                OL = MK;
                XJ = LY;
                XX = MR;
            } else {
                let MT = (U * KI) / KQ;
                let MV = if (if (IQ.abs()) < (1e-5f64 * BL) { 1.0 } else { 0.0 }) != 0.0 || (if (KR.abs()) < ((1e-40f64 * BL) * (KJ + KL)) { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let XK = if MV != 0.0 {
                    let MW = FN * (MT + LW);
                    let MX = MW / (MW + C);
                    MX
                } else {
                    let MY = KR / ((KR + IK) - II);
                    MY
                };
                let MZ = AD * DT;
                let NA = C - (KS / LH);
                NO = IQ;
                NV = MZ;
                OB = NA;
                OL = MU;
                XJ = XK;
                XX = KS;
            }
            let NB = DM * (C - (CC.powf((-1e0f64 / V))));
            let NC = AD * DM;
            let ND = (IM - NB) / NC;
            let NE = if IM < NB { 1.0 } else { 0.0 };
            let NH = if NE != 0.0 {
                let NF = IM - (NC * ((C + (ND.exp())).ln()));
                NF
            } else {
                let NG = NB - (NC * ((C + ((-ND).exp())).ln()));
                NG
            };
            let NI = C - V;
            let NJ = (C - (NH * DN)).powf(NI);
            let NK = ((DM / NI) * (C - NJ)) + (CC * (IM - NH));
            let NM = if NL == C { 1.0 } else { 0.0 };
            let NT;
            if NM != 0.0 {
                NT = II;
            } else {
                let NN = if NL == U { 1.0 } else { 0.0 };
                let NU = if NN != 0.0 {
                    let NP = II + NO;
                    NP
                } else {
                    IK
                };
                NT = NU;
            }
            let NQ = C - DX;
            let NR = (U - DX) / NQ;
            let NS = DT * (C - (NR.powf((-1e0f64 / AM))));
            let NX = (NT - NS) / NV;
            let NY = if NT < NS { 1.0 } else { 0.0 };
            let OF = if NY != 0.0 {
                let NZ = NT - (NV * ((C + (NX.exp())).ln()));
                NZ
            } else {
                let OA = NS - (NV * ((C + ((-NX).exp())).ln()));
                OA
            };
            let OC = OB.powf(parameters[75]);
            let OD = C - AM;
            let OE = DT / OD;
            let OG = (NQ * ((OE * (C - (OC * ((C - (OF / DT)).powf(OD))))) + ((OC * NR) * (NT - OF)))) + (DX * II);
            let OH = (FQ * FT) / FU;
            let OJ = OH * OI;
            let OK = OJ / (C + ((C + OJ).sqrt()));
            let ON = OL.powf((C / OM));
            let OO = OH * ON;
            let OP = OO / (C + ((C + OO).sqrt()));
            let OQ = if HV == A { 1.0 } else { 0.0 };
            let OU = if OQ != 0.0 {
                let OR = (C + (NK / HI)) + (OG / HH);
                OR
            } else {
                let OS = ((((((NK / HI) + C) * HW) * BM).exp()) - (((((-OG) / HH) * HW) * BM).exp())) / (((HW * BM).exp()) - C);
                OS
            };
            let OV = OU * OU;
            let OW = if OU < A { 1.0 } else { 0.0 };
            let OZ = if OW != 0.0 {
                let OX = 5.000000000000001e-3f64 / (((OV + OT).sqrt()) - OU);
                OX
            } else {
                let OY = FN * (((OV + OT).sqrt()) + OU);
                OY
            };
            let PA = C + (FN * (OK + OP));
            let PB = OZ * PA;
            let PC = (parameters[14] * FT) * ON;
            let PD = FT * OI;
            let PE = (PD - PC) / PB;
            let PG = IM / PF;
            let PH = if IM < A { 1.0 } else { 0.0 };
            let PK = if PH != 0.0 {
                let PI = PF * ((C + (PG.exp())).ln());
                PI
            } else {
                let PJ = IM + (PF * ((C + ((-PG).exp())).ln()));
                PJ
            };
            let PL = PK / parameters[143];
            let PM = if PL < IZ { 1.0 } else { 0.0 };
            let PP = if PM != 0.0 {
                let PN = PL.exp();
                PN
            } else {
                let PO = (IZ.exp()) * (C + (PL - IZ));
                PO
            };
            let PQ = GS * (PP - C);
            let PS = (IM - PR) / T;
            let PT = if IM < PR { 1.0 } else { 0.0 };
            let PW = if PT != 0.0 {
                let PU = IM - (T * ((C + (PS.exp())).ln()));
                PU
            } else {
                let PV = PR - (T * ((C + ((-PS).exp())).ln()));
                PV
            };
            let PX = PR - PW;
            let PY = (parameters[146] * PW) * (PX * PX);
            let PZ = JD / GE;
            let QA = if PZ < IZ { 1.0 } else { 0.0 };
            let QN = if QA != 0.0 {
                let QB = PZ.exp();
                QB
            } else {
                let QC = (IZ.exp()) * (C + (PZ - IZ));
                QC
            };
            let AAK;
            if GJ != 0.0 {
                let QE = (IM - QD) * BM;
                let QF = if QE < IZ { 1.0 } else { 0.0 };
                let QQ = if QF != 0.0 {
                    let QG = QE.exp();
                    QG
                } else {
                    let QH = (IZ.exp()) * (C + (QE - IZ));
                    QH
                };
                let QI = (PE / FT) - 1e3f64;
                let QK = if QI < QJ { 1.0 } else { 0.0 };
                let QS = if QK != 0.0 {
                    let QL = QI.exp();
                    QL
                } else {
                    let QM = 2.3538526683702e17f64 * (C + (QI - QJ));
                    QM
                };
                let QO = QN - C;
                let QT = ((GG * QO) + ((((QP * U) * QO) / (C + ((C + (FQ * QQ)).sqrt()))) * (C + (OG / HH)))) + (((QR * (OL - C)) * QS) / (C + QS));
                AAK = QT;
            } else {
                let QV = if QU == A { 1.0 } else { 0.0 };
                let AAL = if QV != 0.0 {
                    let QW = GG * (QN - C);
                    QW
                } else {
                    let QX = GG * (((C - QU) * (QN - C)) + ((QU * ((QN + OL) - U)) * (C + (OG / HH))));
                    QX
                };
                AAK = AAL;
            }
            let QY = IO * BM;
            let QZ = QY / GH;
            let RA = if QZ < IZ { 1.0 } else { 0.0 };
            let RH = if RA != 0.0 {
                let RB = QZ.exp();
                RB
            } else {
                let RC = (IZ.exp()) * (C + (QZ - IZ));
                RC
            };
            let AAO;
            if GJ != 0.0 {
                let RD = (IO - QD) * BM;
                let RE = if RD < IZ { 1.0 } else { 0.0 };
                let RK = if RE != 0.0 {
                    let RF = RD.exp();
                    RF
                } else {
                    let RG = (IZ.exp()) * (C + (RD - IZ));
                    RG
                };
                let RI = RH - C;
                let RL = (GI * RI) + (((RJ * U) * RI) / (C + ((C + (FQ * RK)).sqrt())));
                AAO = RL;
            } else {
                let RM = GI * (RH - C);
                AAO = RM;
            }
            let RN = JD / FY;
            let RO = if RN < IZ { 1.0 } else { 0.0 };
            let RR = if RO != 0.0 {
                let RP = RN.exp();
                RP
            } else {
                let RQ = (IZ.exp()) * (C + (RN - IZ));
                RQ
            };
            let RS = GA * (RR - C);
            let RT = QY / GO;
            let RU = if RT < IZ { 1.0 } else { 0.0 };
            let RX = if RU != 0.0 {
                let RV = RT.exp();
                RV
            } else {
                let RW = (IZ.exp()) * (C + (RT - IZ));
                RW
            };
            let RY = GP * (RX - C);
            let RZ = JI / GB;
            let SA = if RZ < IZ { 1.0 } else { 0.0 };
            let SD = if SA != 0.0 {
                let SB = RZ.exp();
                SB
            } else {
                let SC = (IZ.exp()) * (C + (RZ - IZ));
                SC
            };
            let SE = GC * (SD - C);
            let SF = QY / GQ;
            let SG = if SF < IZ { 1.0 } else { 0.0 };
            let SJ = if SG != 0.0 {
                let SH = SF.exp();
                SH
            } else {
                let SI = (IZ.exp()) * (C + (SF - IZ));
                SI
            };
            let SK = GR * (SJ - C);
            let SL = if (if (if GY > A { 1.0 } else { 0.0 }) != 0.0 && (if GW > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && PH != 0.0 { 1.0 } else { 0.0 };
            let AAN;
            if SL != 0.0 {
                let SM = GX * (C - (W / (U * NJ)));
                let SN = if SM < IZ { 1.0 } else { 0.0 };
                let TF = if SN != 0.0 {
                    let SO = SM.exp();
                    SO
                } else {
                    let SP = (IZ.exp()) * (C + (SM - IZ));
                    SP
                };
                let SQ = IM * DN;
                let SS = V - C;
                let SU = ((IM * W) * GX) / (GT * ((((((SQ * SQ) + SR).sqrt()).powf((-2e0f64 - V))) * ((V * ((C - (V * V)) - ((CC * SQ) * SS))) - (((FX * SQ) * SQ) * (SS + SQ)))) * ST));
                let SV = if SU < -1e-3f64 { 1.0 } else { 0.0 };
                let TE;
                if SV != 0.0 {
                    let SW = if SU < IZ { 1.0 } else { 0.0 };
                    let SZ = if SW != 0.0 {
                        let SX = SU.exp();
                        SX
                    } else {
                        let SY = (IZ.exp()) * (C + (SU - IZ));
                        SY
                    };
                    let TA = (-IM) * (C + ((C - SZ) / SU));
                    TE = TA;
                } else {
                    let TD = ((IM * FN) * SU) * (C + ((SU * TB) * (C + (TC * SU))));
                    TE = TD;
                }
                let TG = (((((U * GZ) * TE) * NJ) * TF) * DN) * X;
                AAN = TG;
            } else {
                AAN = A;
            }
            let TH = if (if (if HE > A { 1.0 } else { 0.0 }) != 0.0 && (if HC > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if II < A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let VM;
            if TH != 0.0 {
                let TI = II * DP;
                let TJ = (C - TI).powf(OD);
                let TK = HD * (C - (AN / (U * TJ)));
                let TL = if TK < IZ { 1.0 } else { 0.0 };
                let TY = if TL != 0.0 {
                    let TM = TK.exp();
                    TM
                } else {
                    let TN = (IZ.exp()) * (C + (TK - IZ));
                    TN
                };
                let TO = AM - C;
                let TP = ((II * AN) * HD) / (HA * ((((((TI * TI) + SR).sqrt()).powf((-2e0f64 - AM))) * ((AM * ((C - (AM * AM)) - ((CC * TI) * TO))) - (((FX * TI) * TI) * (TO + TI)))) * ST));
                let TQ = if TP < -1e-3f64 { 1.0 } else { 0.0 };
                let TX;
                if TQ != 0.0 {
                    let TR = if TP < IZ { 1.0 } else { 0.0 };
                    let TU = if TR != 0.0 {
                        let TS = TP.exp();
                        TS
                    } else {
                        let TT = (IZ.exp()) * (C + (TP - IZ));
                        TT
                    };
                    let TV = (-II) * (C + ((C - TU) / TP));
                    TX = TV;
                } else {
                    let TW = ((II * FN) * TP) * (C + ((TP * TB) * (C + (TC * TP))));
                    TX = TW;
                }
                let TZ = (((((U * HF) * TX) * TJ) * TY) * DP) * AO;
                VM = TZ;
            } else {
                VM = A;
            }
            let UB = (FQ * GN) / FW;
            let UC = ((U * GN) * (UA - C)) / (C + ((C + (UB * UA)).sqrt()));
            let UE = if I > A { 1.0 } else { 0.0 };
            let UF = if (if UD > A { 1.0 } else { 0.0 }) != 0.0 && UE != 0.0 { 1.0 } else { 0.0 };
            let VQ;
            let VT;
            let ABR;
            if UF != 0.0 {
                let UG = UC * J;
                let UI = (((I * U) * GN) * (UH - C)) / (C + ((C + (UB * UH)).sqrt()));
                let UJ = if UD == C { 1.0 } else { 0.0 };
                let UT;
                if UJ != 0.0 {
                    let UK = (I * GN) * EH;
                    let UL = IX - (BL * (U - ((UK * BM).ln())));
                    let UN = UL * UL;
                    let UO = if UL < A { 1.0 } else { 0.0 };
                    let UR = if UO != 0.0 {
                        let UP = 6.05e-3f64 / (((UN + UM).sqrt()) - UL);
                        UP
                    } else {
                        let UQ = FN * (((UN + UM).sqrt()) + UL);
                        UQ
                    };
                    let US = UR / ((UK + (UI * EH)) + UR);
                    UT = US;
                } else {
                    UT = C;
                }
                let UU = UT * UI;
                VQ = UG;
                VT = UU;
                ABR = UT;
            } else {
                VQ = UC;
                VT = A;
                ABR = C;
            }
            let UV = if parameters[83] == C { 1.0 } else { 0.0 };
            let VN;
            if UV != 0.0 {
                let UW = IP + II;
                let UZ = ((-1e0f64 * UW) * -1e0f64) * UW;
                let VA = if (-1e0f64 * UW) < A { 1.0 } else { 0.0 };
                let VI = if VA != 0.0 {
                    let VB = 5e-13f64 / (((UZ + UY).sqrt()) - (-1e0f64 * UW));
                    VB
                } else {
                    let VC = FN * (((UZ + UY).sqrt()) + (-1e0f64 * UW));
                    VC
                };
                let VE = C / (C - (AZ.powf(VD)));
                let VG = AZ * VF;
                let VH = (((VE * VE) * (AZ.powf((VD - C)))) * VD) / VF;
                let VJ = if VI < VG { 1.0 } else { 0.0 };
                let VO = if VJ != 0.0 {
                    let VK = C / (C - ((VI / VF).powf(VD)));
                    VK
                } else {
                    let VL = VE + ((VI - VG) * VH);
                    VL
                };
                VN = VO;
            } else {
                VN = C;
            }
            let VP = VM * VN;
            let VR = VQ * VN;
            let VS = SE * VN;
            let VU = VT * VN;
            let VV = (C + (NK / HI)) + (OG / HH);
            let VX = VV * VV;
            let VY = if VV < A { 1.0 } else { 0.0 };
            let WB = if VY != 0.0 {
                let VZ = 5.000000000000001e-3f64 / (((VX + VW).sqrt()) - VV);
                VZ
            } else {
                let WA = FN * (((VX + VW).sqrt()) + VV);
                WA
            };
            let WC = ED / (WB * PA);
            let WD = if WC < R { 1.0 } else { 0.0 };
            let WE = if WD != 0.0 {
                R
            } else {
                WC
            };
            let WF = CC * WE;
            let WH = if PE > A { 1.0 } else { 0.0 };
            let AAA;
            if WH != 0.0 {
                let WJ = if WI == C { 1.0 } else { 0.0 };
                let ZD;
                if WJ != 0.0 {
                    let WL = if II < WK { 1.0 } else { 0.0 };
                    let ZE;
                    if WL != 0.0 {
                        let WM = (-PE) / parameters[41];
                        let WN = if WM < IZ { 1.0 } else { 0.0 };
                        let WQ = if WN != 0.0 {
                            let WO = WM.exp();
                            WO
                        } else {
                            let WP = (IZ.exp()) * (C + (WM - IZ));
                            WP
                        };
                        let WR = (WK - II) * WQ;
                        let WU = (-WS) * (WR.powf(WT));
                        let WV = if WU < IZ { 1.0 } else { 0.0 };
                        let WZ = if WV != 0.0 {
                            let WW = WU.exp();
                            WW
                        } else {
                            let WX = (IZ.exp()) * (C + (WU - IZ));
                            WX
                        };
                        let XA = ((WY / WS) * WR) * WZ;
                        ZE = XA;
                    } else {
                        ZE = A;
                    }
                    ZD = ZE;
                } else {
                    let XB = if WI == U { 1.0 } else { 0.0 };
                    let ZF;
                    if XB != 0.0 {
                        let XC = if II < JU { 1.0 } else { 0.0 };
                        let ZG;
                        if XC != 0.0 {
                            let XE = (U * parameters[45]) / (XD * XD);
                            let XF = JU - II;
                            let XG = ((U * (XF / OB)) / XE).sqrt();
                            let XI = if XH == A { 1.0 } else { 0.0 };
                            let XN = if XI != 0.0 {
                                XD
                            } else {
                                let XL = C - (FN * XJ);
                                let XM = (XD * XL) * XL;
                                XM
                            };
                            let XO = (XG * XN) / (((XG * XG) + (XN * XN)).sqrt());
                            let XP = XF / XO;
                            let XQ = FN * XO;
                            let XR = XQ * XE;
                            let XS = XP + (XR * OB);
                            let XZ = if XI != 0.0 {
                                XS
                            } else {
                                let XU = U * XT;
                                let XV = XP - (XR * (((C + XT) / (C + XU)) - (PE / (LH * (C + (XU * (C + (U * XJ))))))));
                                let XW = XV - XS;
                                let XY = FN * ((XV + XS) + (((XW * XW) + ((((AD * XP) * XP) * XX) / LH)).sqrt()));
                                XY
                            };
                            let YA = (XZ - XP) / XZ;
                            let YB = if (YA.abs()) > 1e-7f64 { 1.0 } else { 0.0 };
                            let ZH = if YB != 0.0 {
                                let YC = XQ / YA;
                                let YF = (-YE) / XZ;
                                let YG = (((YD / YE) * XZ) * YC) * ((YF.exp()) - ((YF * (C + (XN / YC))).exp()));
                                YG
                            } else {
                                let YH = (YD * XN) * (((-YE) / XZ).exp());
                                YH
                            };
                            ZG = ZH;
                        } else {
                            ZG = A;
                        }
                        ZF = ZG;
                    } else {
                        let YI = if WI == CC { 1.0 } else { 0.0 };
                        let ZI;
                        if YI != 0.0 {
                            let YJ = if II < WK { 1.0 } else { 0.0 };
                            let ZJ;
                            if YJ != 0.0 {
                                let YK = WK - II;
                                let YM = (YK.powf(WT)) * ((C - (PE / (YL + PE))).powf(parameters[48]));
                                let YN = if XH == A { 1.0 } else { 0.0 };
                                let YW;
                                if YN != 0.0 {
                                    YW = YM;
                                } else {
                                    let YO = (PE - parameters[51]) / YL;
                                    let YQ = (YO - C) / YP;
                                    let YR = if YO < C { 1.0 } else { 0.0 };
                                    let YU = if YR != 0.0 {
                                        let YS = C + (YP * ((C + (YQ.exp())).ln()));
                                        YS
                                    } else {
                                        let YT = YO + (YP * ((C + ((-YQ).exp())).ln()));
                                        YT
                                    };
                                    let YV = YM * (YU.powf(parameters[49]));
                                    YW = YV;
                                }
                                let YX = (-WS) * YW;
                                let YY = if YX < IZ { 1.0 } else { 0.0 };
                                let ZB = if YY != 0.0 {
                                    let YZ = YX.exp();
                                    YZ
                                } else {
                                    let ZA = (IZ.exp()) * (C + (YX - IZ));
                                    ZA
                                };
                                let ZC = ((WY / WS) * YK) * ZB;
                                ZJ = ZC;
                            } else {
                                ZJ = A;
                            }
                            ZI = ZJ;
                        } else {
                            ZI = A;
                        }
                        ZF = ZI;
                    }
                    ZD = ZF;
                }
                let ZK = if ZD > A { 1.0 } else { 0.0 };
                let AAB;
                if ZK != 0.0 {
                    let ZL = if parameters[52] == C { 1.0 } else { 0.0 };
                    let AAC;
                    if ZL != 0.0 {
                        let ZN = ZM + WF;
                        let ZP = ((BL / (PE * ZN)) + ((PB / FT) * GG)) + (ZO / ZN);
                        let ZQ = if WI == CC { 1.0 } else { 0.0 };
                        let AAD;
                        if ZQ != 0.0 {
                            let ZR = (ZD - ZP) / UX;
                            let ZS = if ZD < ZP { 1.0 } else { 0.0 };
                            let ZV = if ZS != 0.0 {
                                let ZT = ZD - (UX * ((C + (ZR.exp())).ln()));
                                ZT
                            } else {
                                let ZU = ZP - (UX * ((C + ((-ZR).exp())).ln()));
                                ZU
                            };
                            let ZW = PE * ZV;
                            AAD = ZW;
                        } else {
                            let ZX = ((PE * ZD) * ZP) / (ZD + ZP);
                            AAD = ZX;
                        }
                        AAC = AAD;
                    } else {
                        let ZY = PE * ZD;
                        AAC = ZY;
                    }
                    AAB = AAC;
                } else {
                    AAB = A;
                }
                AAA = AAB;
            } else {
                AAA = A;
            }
            let ZZ = if OL > A { 1.0 } else { 0.0 };
            if ZZ != 0.0 {
            } else {
            }
            if GJ != 0.0 {
            } else {
            }
            let AAM = AAK + RS;
            let AAP = (AAO + RY) + SK;
            let AAQ = if IO < NB { 1.0 } else { 0.0 };
            if AAQ != 0.0 {
            } else {
            }
            let AAR = HK * FU;
            let AAS = AD * DT;
            let AAT = if IW < NS { 1.0 } else { 0.0 };
            if AAT != 0.0 {
            } else {
            }
            let AAU = C - parameters[76];
            let AAV = (IX - NS) / AAS;
            let AAW = if IX < NS { 1.0 } else { 0.0 };
            let AAZ = if AAW != 0.0 {
                let AAX = IX - (AAS * ((C + (AAV.exp())).ln()));
                AAX
            } else {
                let AAY = NS - (AAS * ((C + ((-AAV).exp())).ln()));
                AAY
            };
            let ABA = ((DW * ((NQ * ((OE * (C - ((C - (AAZ / DT)).powf(OD)))) + (NR * (IX - AAZ)))) + (DX * IX))) * AAU) * I;
            let ABB = if (IM / (parameters[84] * BL)) < IZ { 1.0 } else { 0.0 };
            if ABB != 0.0 {
            } else {
            }
            let ABC = ((FQ * HM) * BL) / EO;
            let ABD = if parameters[78] == A { 1.0 } else { 0.0 };
            if ABD != 0.0 {
            } else {
                let ABF = if (((IW - ABE) / parameters[90]) * BM) < IZ { 1.0 } else { 0.0 };
                if ABF != 0.0 {
                } else {
                }
            }
            let ABG = if (if (if UD == C { 1.0 } else { 0.0 }) != 0.0 || (if UD == CC { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && UE != 0.0 { 1.0 } else { 0.0 };
            let ACG;
            if ABG != 0.0 {
                let ABS;
                if ABD != 0.0 {
                    let ABH = OH * UH;
                    let ABJ = FQ * ABI;
                    let ABK = (((FN * I) * HO) * ((AAR * ((ABH - OH) / (C + ((C + ABH).sqrt())))) + (ABC * (ABJ / (C + ((C + ABJ).sqrt())))))) / HN;
                    ABS = ABK;
                } else {
                    let ABL = (IX - ABE) * BM;
                    let ABM = if ABL < IZ { 1.0 } else { 0.0 };
                    let ABP = if ABM != 0.0 {
                        let ABN = ABL.exp();
                        ABN
                    } else {
                        let ABO = (IZ.exp()) * (C + (ABL - IZ));
                        ABO
                    };
                    let ABQ = ((((U * I) * GN) * HP) * UH) / (C + ((C + (FQ * ABP)).sqrt()));
                    ABS = ABQ;
                }
                let ABT = ABR * ABS;
                ACG = ABT;
            } else {
                ACG = A;
            }
            let ABU = if parameters[6] == C { 1.0 } else { 0.0 };
            if ABU != 0.0 {
                let ABV = if ND < A { 1.0 } else { 0.0 };
                if ABV != 0.0 {
                } else {
                }
            } else {
            }
            if GJ != 0.0 {
            } else {
            }
            let ABW = ((B * IS) / ZM) * Q;
            let ABX = C - parameters[135];
            let ABY = if parameters[133] > R { 1.0 } else { 0.0 };
            if ABY != 0.0 {
                let ABZ = if parameters[132] == A { 1.0 } else { 0.0 };
                if ABZ != 0.0 {
                } else {
                    let ACA = if (ABX.abs()) < UX { 1.0 } else { 0.0 };
                    if ACA != 0.0 {
                    } else {
                    }
                }
            } else {
            }
            let ACB = (B * parameters[68]) * IT;
            let ACC = 0e0f64 * Q;
            let ACD = (B * parameters[77]) * IU;
            let ACE = 0e0f64 * Q;
            let ACF = (B * VU) * Q;
            let ACH = B * (ABA + ACG);
            let ACI = 0e0f64 * Q;
            if IA != 0.0 {
            } else {
            }
            if ID != 0.0 {
            } else {
            }
            let ACJ = 5.5224904e-23f64 * BI;
            let ACK = ACJ / ZO;
            let ACL = ACJ / ZM;
            let ACM = ACJ * AAE;
            let ACN = ACJ * AAG;
            let ACO = ACJ * AAI;
            let ACP = ((ACJ / WF) * ((FQ * WG) + 5e0f64)) * TB;
            let ACQ = (PD + PC) / PB;
            let ACR = 3.2043836e-19f64 * (ACQ.abs());
            let ACS = if parameters[129] > A { 1.0 } else { 0.0 };
            let ACU = if ACS != 0.0 {
                let ACT = (AAA / ACQ).abs();
                ACT
            } else {
                A
            };
            let ACV = (3.2043836e-19f64 * AAA) * (ACU + C);
            let ACW = if ACQ > A { 1.0 } else { 0.0 };
            if ACW != 0.0 {
            } else {
            }
            let ACY = if ACX == C { 1.0 } else { 0.0 };
            if ACY != 0.0 {
            } else {
                let ACZ = if ACX == U { 1.0 } else { 0.0 };
                if ACZ != 0.0 {
                } else {
                }
            }
            let ADA = 3.2043836e-19f64 * ((((AAM - AAN) + PY) + PQ).abs());
            let ADB = AAK + AAO;
            let ADE = ADC * ((ADB.abs()).powf(ADD));
            let ADF = if ADB < A { 1.0 } else { 0.0 };
            let AEL = if ADF != 0.0 {
                let ADG = -ADE;
                ADG
            } else {
                ADE
            };
            let ADH = (RS + RY) + SK;
            let ADI = parameters[128] * ((ADH.abs()).powf(parameters[126]));
            let ADJ = if ADH < A { 1.0 } else { 0.0 };
            let AEN = if ADJ != 0.0 {
                let ADK = -ADI;
                ADK
            } else {
                ADI
            };
            let ADL = 3.2043836e-19f64 * (AAP.abs());
            let ADM = VS.abs();
            let ADN = 3.2043836e-19f64 * ADM;
            let ADO = ADC * (ADM.powf(ADD));
            let ADP = if VS < A { 1.0 } else { 0.0 };
            let AER = if ADP != 0.0 {
                let ADQ = -ADO;
                ADQ
            } else {
                ADO
            };
            let ADR = 3.2043836e-19f64 * (VP.abs());
            let ADS = VR.abs();
            let ADT = 3.2043836e-19f64 * ADS;
            let ADU = C - (UD * I);
            let ADV = (ADC * ADU) * ((ADS / ADU).powf(ADD));
            let ADW = if VR < A { 1.0 } else { 0.0 };
            let AEU = if ADW != 0.0 {
                let ADX = -ADV;
                ADX
            } else {
                ADV
            };
            let ADY = VU.abs();
            let ADZ = (3.2043836e-19f64 * ADY) * UD;
            let AEA = if I == A { 1.0 } else { 0.0 };
            let AED = if AEA != 0.0 {
                A
            } else {
                let AEB = ((ADC * UD) * I) * ((ADY / I).powf(ADD));
                AEB
            };
            let AEC = if VU < A { 1.0 } else { 0.0 };
            let AEX = if AEC != 0.0 {
                let AEE = -AED;
                AEE
            } else {
                AED
            };
            let AEF = ACR * Q;
            let AEG = ACV * Q;
            let AEH = ADA * Q;
            let AEI = ACK * Q;
            let AEJ = ACL * Q;
            let AEK = ACP * Q;
            let AEM = AEL * Q;
            let AEO = AEN * Q;
            let AEP = ADL * Q;
            let AEQ = ADN * Q;
            let AES = AER * Q;
            let AET = ADT * Q;
            let AEV = AEU * Q;
            let AEW = ADZ * Q;
            let AEY = AEX * Q;
            let AFL;
            let AFM;
            let AFN;
            let AFO;
            if GJ != 0.0 {
                let AEZ = ADR * Q;
                AFL = C;
                AFM = AEZ;
                AFN = A;
                AFO = A;
            } else {
                let AFA = ADR * Q;
                AFL = A;
                AFM = A;
                AFN = C;
                AFO = AFA;
            }
            let AFP;
            let AFR;
            let AFT;
            let AFV;
            let AFX;
            let AFZ;
            let AGB;
            let AGD;
            let AGF;
            let AGH;
            let AGJ;
            let AGL;
            let AGN;
            let AGP;
            let AGR;
            let AGT;
            if IA != 0.0 {
                let AFQ;
                let AFS;
                let AFU;
                let AFW;
                let AFY;
                let AGA;
                let AGC;
                let AGE;
                let AGG;
                let AGI;
                if ID != 0.0 {
                    let AFB = ACM * Q;
                    let AFC = ACN * Q;
                    let AFD = ACO * Q;
                    AFQ = C;
                    AFS = AFB;
                    AFU = C;
                    AFW = AFC;
                    AFY = C;
                    AGA = AFD;
                    AGC = A;
                    AGE = A;
                    AGG = A;
                    AGI = A;
                } else {
                    let AFE = ACM * Q;
                    let AFF = ACN * Q;
                    AFQ = A;
                    AFS = A;
                    AFU = A;
                    AFW = A;
                    AFY = A;
                    AGA = A;
                    AGC = C;
                    AGE = AFE;
                    AGG = C;
                    AGI = AFF;
                }
                AFP = AFQ;
                AFR = AFS;
                AFT = AFU;
                AFV = AFW;
                AFX = AFY;
                AFZ = AGA;
                AGB = AGC;
                AGD = AGE;
                AGF = AGG;
                AGH = AGI;
                AGJ = A;
                AGL = A;
                AGN = A;
                AGP = A;
                AGR = A;
                AGT = A;
            } else {
                let AGK;
                let AGM;
                let AGO;
                let AGQ;
                let AGS;
                let AGU;
                if ID != 0.0 {
                    let AFG = ACM * Q;
                    let AFH = ACO * Q;
                    AGK = C;
                    AGM = AFG;
                    AGO = C;
                    AGQ = AFH;
                    AGS = A;
                    AGU = A;
                } else {
                    let AFI = ACM * Q;
                    AGK = A;
                    AGM = A;
                    AGO = A;
                    AGQ = A;
                    AGS = C;
                    AGU = AFI;
                }
                AFP = A;
                AFR = A;
                AFT = A;
                AFV = A;
                AFX = A;
                AFZ = A;
                AGB = A;
                AGD = A;
                AGF = A;
                AGH = A;
                AGJ = AGK;
                AGL = AGM;
                AGN = AGO;
                AGP = AGQ;
                AGR = AGS;
                AGT = AGU;
            }
            let AFJ = if ((((ABW + ACC) + ACE) + ACF) + ACI) == A { 1.0 } else { 0.0 };
            if AFJ != 0.0 {
            } else {
            }
            let AFK = if Q != C { 1.0 } else { 0.0 };
            if AFK != 0.0 {
            } else {
            }
        {
            let psd = AEF;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 0, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 0, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(0, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = AEG;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(1, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = AEH;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 2, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 2, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(2, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = AEI;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 3, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 3, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(3, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = AEJ;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 4, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 4, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(4, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = AEK;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 5, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 5, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(5, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = AEM;
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
            let psd = AEO;
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
            let psd = AEP;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 8, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 8, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(8, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = AEQ;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 9, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 9, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(9, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = AES;
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
            let psd = AET;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 11, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 11, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(11, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = AEV;
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
            let psd = AEW;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 13, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 13, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(13, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = AEY;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 14, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = Some(C);
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 14, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 14, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(14, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if AFL == 0.0 {
            if !visitor.visit(15, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = AFM;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 15, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 15, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(15, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if AFN == 0.0 {
            if !visitor.visit(16, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = AFO;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 16, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 16, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(16, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if AFP == 0.0 {
            if !visitor.visit(17, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = AFR;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 17, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 17, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(17, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if AFT == 0.0 {
            if !visitor.visit(18, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = AFV;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 18, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 18, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(18, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if AFX == 0.0 {
            if !visitor.visit(19, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = AFZ;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 19, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 19, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(19, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if AGB == 0.0 {
            if !visitor.visit(20, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = AGD;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 20, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 20, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(20, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if AGF == 0.0 {
            if !visitor.visit(21, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = AGH;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 21, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 21, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(21, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if AGJ == 0.0 {
            if !visitor.visit(22, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = AGL;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 22, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 22, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(22, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if AGN == 0.0 {
            if !visitor.visit(23, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = AGP;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 23, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 23, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(23, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if AGR == 0.0 {
            if !visitor.visit(24, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = AGT;
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
