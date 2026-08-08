#![allow(dead_code, non_snake_case, unused_parens, unused_variables)]

use super::state::Instance;
use rspice_veriloga_runtime::GeneratedEvalContext;
pub use rspice_veriloga_runtime::{GeneratedNoiseDescriptor, GeneratedNoiseEndpoint, GeneratedNoiseEvaluation, GeneratedNoiseEvaluationError, GeneratedNoiseEvaluationRef, GeneratedNoiseKind, GeneratedNoiseVisitor};

use rspice_veriloga_runtime::rspice_limexp;
pub static NOISE_SOURCES: [GeneratedNoiseDescriptor; 19] = [
    GeneratedNoiseDescriptor { mechanism: "WHITE_B_BP_RBX", label: Some("rbx"), kind: GeneratedNoiseKind::White, equation: 47, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(1), name: "b", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(7), name: "bp", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_BP_BI_RBI", label: Some("rbi"), kind: GeneratedNoiseKind::White, equation: 48, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(7), name: "bp", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(8), name: "bi", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_CI_C_RCX", label: Some("rcx"), kind: GeneratedNoiseKind::White, equation: 49, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "ci", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(0), name: "c", is_internal: false }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_EI_E_RE", label: Some("re"), kind: GeneratedNoiseKind::White, equation: 50, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(6), name: "ei", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(2), name: "e", is_internal: false }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_SI_S_RSU", label: Some("rsu"), kind: GeneratedNoiseKind::White, equation: 51, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(9), name: "si", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(3), name: "s", is_internal: false }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_BI_EI_FLICKER", label: Some("flicker"), kind: GeneratedNoiseKind::Flicker, equation: 52, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(8), name: "bi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(6), name: "ei", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_BP_EI_FLICKER", label: Some("flicker"), kind: GeneratedNoiseKind::Flicker, equation: 53, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(7), name: "bp", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(6), name: "ei", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_EI_E_FLICKER_RE", label: Some("flicker_re"), kind: GeneratedNoiseKind::Flicker, equation: 54, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(6), name: "ei", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(2), name: "e", is_internal: false }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_BI_EI_IBEBTB", label: Some("ibebtb"), kind: GeneratedNoiseKind::White, equation: 55, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(8), name: "bi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(6), name: "ei", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_BP_EI_IBEP", label: Some("ibep"), kind: GeneratedNoiseKind::White, equation: 56, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(7), name: "bp", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(6), name: "ei", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_CI_BI_IAVL", label: Some("iavl"), kind: GeneratedNoiseKind::White, equation: 57, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "ci", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(8), name: "bi", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_BI_CI_IBCI", label: Some("ibci"), kind: GeneratedNoiseKind::White, equation: 58, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(8), name: "bi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(5), name: "ci", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_BI_CI_IBCBTB", label: Some("ibcbtb"), kind: GeneratedNoiseKind::White, equation: 59, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(8), name: "bi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(5), name: "ci", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_BP_CI_IJBCX", label: Some("ijbcx"), kind: GeneratedNoiseKind::White, equation: 60, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(7), name: "bp", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(5), name: "ci", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_SI_CI_IJSC", label: Some("ijsc"), kind: GeneratedNoiseKind::White, equation: 61, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(9), name: "si", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(5), name: "ci", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_N1_GND_IBEI", label: Some("ibei"), kind: GeneratedNoiseKind::White, equation: 62, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(13), name: "n1", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: None, name: "0", is_internal: false }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_N2_GND_IT", label: Some("it"), kind: GeneratedNoiseKind::White, equation: 67, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(14), name: "n2", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: None, name: "0", is_internal: false }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_CI_EI_IT", label: Some("it"), kind: GeneratedNoiseKind::White, equation: 70, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "ci", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(6), name: "ei", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_BI_EI_IBEI", label: Some("ibei"), kind: GeneratedNoiseKind::White, equation: 71, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(8), name: "bi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(6), name: "ei", is_internal: true }, table_len: 0, table_log_interp: false },
];

impl Instance {
    pub fn evaluate_noise_sources(&self, ctx: &GeneratedEvalContext<'_>, visitor: &mut dyn GeneratedNoiseVisitor) -> Result<(), GeneratedNoiseEvaluationError> {
        if !self.multiplicity.is_finite() || self.multiplicity <= 0.0 {
            return Err(GeneratedNoiseEvaluationError::InvalidMultiplicity { value: self.multiplicity });
        }
        let parameters = &self.params.values;
        let temperature = ctx.temperature();
        let node_potentials = [ctx.node_voltage(self.nodes[0]), ctx.node_voltage(self.nodes[1]), ctx.node_voltage(self.nodes[2]), ctx.node_voltage(self.nodes[3]), ctx.node_voltage(self.nodes[4]), ctx.node_voltage(self.nodes[5]), ctx.node_voltage(self.nodes[6]), ctx.node_voltage(self.nodes[7]), ctx.node_voltage(self.nodes[8]), ctx.node_voltage(self.nodes[9]), ctx.node_voltage(self.nodes[10]), ctx.node_voltage(self.nodes[11]), ctx.node_voltage(self.nodes[12]), ctx.node_voltage(self.nodes[13]), ctx.node_voltage(self.nodes[14])];
        let branch_unknown_flows = [ctx.branch_current(self.branches[0]), ctx.branch_current(self.branches[1]), ctx.branch_current(self.branches[2]), ctx.branch_current(self.branches[3]), ctx.branch_current(self.branches[4]), ctx.branch_current(self.branches[5])];
            let A = 0e0f64;
            let B = parameters[148];
            let C = node_potentials[8];
            let D = node_potentials[6];
            let F = node_potentials[5];
            let I = node_potentials[7];
            let L = node_potentials[1];
            let Q = parameters[0];
            let R = 3.1e2f64;
            let T = 1.6021918e-19f64;
            let U = 1.3806226e-23f64;
            let V = 1.602176634e-19f64;
            let W = 1.380649e-23f64;
            let AC = 3e2f64;
            let AF = 1e0f64;
            let AH = parameters[121];
            let AJ = parameters[122];
            let AM = parameters[117];
            let AO = parameters[118];
            let AP = parameters[119];
            let AQ = 5e-1f64;
            let AV = parameters[120];
            let AZ = parameters[130];
            let BD = parameters[52];
            let BE = parameters[106];
            let BJ = parameters[104];
            let BL = parameters[22];
            let BP = 7e-1f64;
            let BQ = parameters[32];
            let BR = parameters[47];
            let BU = parameters[86];
            let BW = parameters[88];
            let BX = parameters[87];
            let BY = parameters[66];
            let CA = parameters[115];
            let CB = 1e-2f64;
            let CC = parameters[116];
            let CI = 1e9f64;
            let CJ = 1.7e8f64;
            let CO = 6e0f64;
            let CU = 7.314999999999998e1f64;
            let CW = 6e2f64;
            let DK = parameters[39];
            let DM = 2e0f64;
            let DN = parameters[40];
            let DP = 4e0f64;
            let DR = parameters[41];
            let DT = parameters[42];
            let DX = parameters[14];
            let DY = parameters[124];
            let ED = parameters[16];
            let EE = parameters[17];
            let EJ = parameters[48];
            let EM = parameters[49];
            let EO = parameters[50];
            let ES = 2.4e0f64;
            let ET = parameters[23];
            let EX = parameters[2];
            let FB = parameters[1];
            let FC = parameters[123];
            let FF = parameters[10];
            let FG = parameters[126];
            let FI = parameters[8];
            let FJ = 1e-5f64;
            let FL = parameters[9];
            let FM = parameters[125];
            let FN = parameters[127];
            let FQ = parameters[3];
            let FT = parameters[4];
            let FW = parameters[6];
            let FZ = parameters[75];
            let GC = parameters[74];
            let GE = parameters[79];
            let GG = parameters[133];
            let GI = parameters[78];
            let GJ = parameters[132];
            let GL = parameters[128];
            let GM = parameters[129];
            let GO = parameters[69];
            let GP = parameters[71];
            let GU = parameters[139];
            let GW = parameters[33];
            let GX = parameters[140];
            let GZ = parameters[37];
            let HB = parameters[38];
            let HK = parameters[89];
            let HL = parameters[134];
            let HN = parameters[43];
            let HP = parameters[44];
            let HS = parameters[45];
            let HU = parameters[46];
            let HY = parameters[18];
            let IA = parameters[20];
            let IB = parameters[21];
            let IE = parameters[27];
            let II = parameters[29];
            let IW = parameters[28];
            let JA = parameters[30];
            let JB = parameters[31];
            let JD = 1.0f64;
            let JE = parameters[53];
            let JH = parameters[54];
            let JJ = parameters[55];
            let JS = parameters[25];
            let JU = parameters[57];
            let JW = parameters[58];
            let JZ = parameters[59];
            let KB = 2.4e0f64;
            let KC = 0.0f64;
            let KE = -2.4e0f64;
            let KJ = parameters[60];
            let KP = parameters[99];
            let KS = parameters[63];
            let KU = parameters[62];
            let KY = parameters[64];
            let LG = parameters[96];
            let LH = parameters[136];
            let LJ = parameters[90];
            let LK = parameters[135];
            let LM = parameters[95];
            let LN = parameters[137];
            let LP = parameters[142];
            let LQ = parameters[141];
            let LR = parameters[149];
            let LX = 7.314999999999998e1f64;
            let LZ = 6e2f64;
            let PH = 1.0f64;
            let PW = 2.4e0f64;
            let PX = 0.0f64;
            let PZ = -2.4e0f64;
            let RA = 8e1f64;
            let SD = 1.921812e0f64;
            let SM = parameters[51];
            let SN = 1e2f64;
            let TK = 1e-1f64;
            let US = 1e-3f64;
            let VG = 5e-2f64;
            let WI = parameters[77];
            let WL = parameters[85];
            let WZ = 1e-6f64;
            let XE = parameters[70];
            let XG = parameters[83];
            let XK = -1e10f64;
            let XM = parameters[84];
            let XN = parameters[82];
            let XP = parameters[73];
            let XV = parameters[72];
            let XZ = 5e-3f64;
            let YO = 2.5e-1f64;
            let ZL = parameters[5];
            let ZX = parameters[7];
            let AAR = -1e10f64;
            let AEF = -1e10f64;
            let AGT = parameters[35];
            let AHI = parameters[34];
            let AKA = parameters[56];
            let AME = parameters[61];
            let ANO = parameters[65];
            let APP = node_potentials[2];
            let AQV = parameters[102];
            let ARC = 0e0f64;
            let E = B * (C - D);
            let G = B * (C - F);
            let H = E - G;
            let J = B * (I - D);
            let K = B * (I - F);
            let M = L - F;
            let N = B * M;
            let O = B * (node_potentials[9] - F);
            let P = B * (node_potentials[3] - node_potentials[0]);
            let S = if Q <= R { 1.0 } else { 0.0 };
            let Z;
            let AA;
            if S != 0.0 {
                Z = U;
                AA = T;
            } else {
                Z = W;
                AA = V;
            }
            let X = ctx.simparam_or("gmin", A);
            let Y = parameters[146] + 2.7315e2f64;
            let AB = Z / AA;
            let AD = AB * AC;
            let AE = AB * Y;
            let AG = AF / AE;
            let AI = (AH * Y) * (Y.ln());
            let AK = AJ * Y;
            let AL = parameters[131] * Y;
            let AN = (AM + AI) + AK;
            let AR = (AN + ((AO + AI) + AK)) * AQ;
            let AS = (AN + ((AP + AI) + AK)) * AQ;
            let AT = (AM + AO) * AQ;
            let AU = (AM + AP) * AQ;
            let AW = (AV + AP) * AQ;
            let AX = 3e0f64 - (AH / AB);
            let AY = AX + AF;
            let BA = AY - AZ;
            let BB = AY - parameters[138];
            let BC = AX - 1.5e0f64;
            let BF = (AF - parameters[107]) * (BD + BE);
            let BG = if BF >= BE { 1.0 } else { 0.0 };
            let JO;
            let JQ;
            let AQM;
            if BG != 0.0 {
                let BH = BF - BE;
                let BI = BD - BH;
                JO = BH;
                JQ = BI;
                AQM = BE;
            } else {
                JO = A;
                JQ = BD;
                AQM = BF;
            }
            let BK = BJ - (parameters[105] * BJ);
            let BM = if BL != A { 1.0 } else { 0.0 };
            let AHR = if BM != 0.0 {
                let BN = AF / BL;
                BN
            } else {
                A
            };
            let BO = if Q <= AC { 1.0 } else { 0.0 };
            let IF = if BO != 0.0 {
                A
            } else {
                BP
            };
            let BS = if BR > A { 1.0 } else { 0.0 };
            let BT = if (if BQ > A { 1.0 } else { 0.0 }) != 0.0 && BS != 0.0 { 1.0 } else { 0.0 };
            let GS = if BT != 0.0 {
                AF
            } else {
                A
            };
            let BV = if BU != A { 1.0 } else { 0.0 };
            let APU;
            if BV != 0.0 {
                let BZ = if (if (if BW == A { 1.0 } else { 0.0 }) != 0.0 && (if BX == A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if BY == A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let APV = if BZ != 0.0 {
                    A
                } else {
                    BU
                };
                APU = APV;
            } else {
                APU = BU;
            }
            let CD = if (if CA >= CB { 1.0 } else { 0.0 }) != 0.0 || (if CC >= CB { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let YE;
            let YG;
            let YJ;
            let YL;
            let YR;
            let YT;
            let YW;
            let YY;
            if CD != 0.0 {
                let CE = AQ * (CA - CC);
                let CF = if CC < CA { 1.0 } else { 0.0 };
                let CG;
                let CK;
                if CF != 0.0 {
                    CG = CC;
                    CK = CA;
                } else {
                    CG = CA;
                    CK = CC;
                }
                let CH = if CG < CB { 1.0 } else { 0.0 };
                let YH;
                let YS;
                let YU;
                let YX;
                let YZ;
                if CH != 0.0 {
                    let CL = (AF + CK).ln();
                    YH = CL;
                    YS = CJ;
                    YU = CI;
                    YX = CJ;
                    YZ = CI;
                } else {
                    let CM = AF / CA;
                    let CN = AF / CC;
                    let CP = CA / CO;
                    let CQ = CC / CO;
                    let CR = ((AF + CA) / (AF + CC)).ln();
                    YH = CR;
                    YS = CP;
                    YU = CN;
                    YX = CQ;
                    YZ = CM;
                }
                YE = CE;
                YG = YH;
                YJ = CG;
                YL = CK;
                YR = YS;
                YT = YU;
                YW = YX;
                YY = YZ;
            } else {
                YE = A;
                YG = A;
                YJ = CC;
                YL = CA;
                YR = CJ;
                YT = CI;
                YW = CJ;
                YY = CI;
            }
            let CS = temperature + parameters[147];
            let CT = if CS < 7.314999999999998e1f64 { 1.0 } else { 0.0 };
            let CX;
            if CT != 0.0 {
                CX = CU;
            } else {
                let CV = if CS > 6e2f64 { 1.0 } else { 0.0 };
                let CY = if CV != 0.0 {
                    CW
                } else {
                    CS
                };
                CX = CY;
            }
            let CZ = AB * CX;
            let DA = AF / CZ;
            let DB = CX - Y;
            let DC = Y / CX;
            let DD = CX / Y;
            let DE = DD.ln();
            let DF = (AH * CX) * (CX.ln());
            let DG = AJ * CX;
            let DH = (AM + DF) + DG;
            let DI = (DH + ((AO + DF) + DG)) * AQ;
            let DJ = (DH + ((AP + DF) + DG)) * AQ;
            let DL = if DK > A { 1.0 } else { 0.0 };
            let EY;
            let IQ;
            let RZ;
            if DL != 0.0 {
                let DO = ((((DM * AE) * (((((DN * AQ) * AG).exp()) - (((-5e-1f64 * DN) * AG).exp())).ln())) * DD) + (AT * (AF - DD))) - ((AX * CZ) * DE);
                let DQ = DO + ((DM * CZ) * ((AQ * (AF + ((AF + (DP * (((-DO) * DA).exp()))).sqrt()))).ln()));
                let DS = DK * ((DR * ((DN / DQ).ln())).exp());
                let DU = DT.abs();
                let DV = if DT > A { 1.0 } else { 0.0 };
                let SA = if DV != 0.0 {
                    let DW = (DT * DQ) / DN;
                    DW
                } else {
                    DU
                };
                EY = DQ;
                IQ = DS;
                RZ = SA;
            } else {
                EY = DN;
                IQ = DK;
                RZ = DT;
            }
            let DZ = AO * AG;
            let EA = AF - DC;
            let EB = ((DY * DE) + (DZ * EA)).exp();
            let EC = DX * EB;
            let EF = AX / EE;
            let EG = AT * AG;
            let EH = EG * EA;
            let EI = ED * (((EF * DE) + (EH / EE)).exp());
            let HE;
            let HG;
            let SZ;
            if BS != 0.0 {
                let EK = ((((DM * AE) * (((((EJ * AQ) * AG).exp()) - (((-5e-1f64 * EJ) * AG).exp())).ln())) * DD) + (AU * (AF - DD))) - ((AX * CZ) * DE);
                let EL = EK + ((DM * CZ) * ((AQ * (AF + ((AF + (DP * (((-EK) * DA).exp()))).sqrt()))).ln()));
                let EN = BR * ((EM * ((EJ / EL).ln())).exp());
                let EP = EO.abs();
                let EQ = if EO > A { 1.0 } else { 0.0 };
                let TA = if EQ != 0.0 {
                    let ER = (EO * EL) / EJ;
                    ER
                } else {
                    EP
                };
                HE = EL;
                HG = EN;
                SZ = TA;
            } else {
                HE = EJ;
                HG = BR;
                SZ = EO;
            }
            let SY = if BO != 0.0 {
                ES
            } else {
                SZ
            };
            let EU = AP * AG;
            let EV = EU * EA;
            let EW = ET * (((BA * DE) + EV).exp());
            let EZ = EY / DN;
            let FA = EX * (DM - ((DR * (EZ.ln())).exp()));
            let FD = AM * AG;
            let FE = FB * (((FC * DE) + (FD * EA)).exp());
            let FH = FF * ((FG * DE).exp());
            let FK = if BO != 0.0 && (if ((FI - AF).abs()) < FJ { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let UW = if FK != 0.0 {
                let FO = FL * (((FM * DA) * (((FN * DE).exp()) - AF)).exp());
                FO
            } else {
                let FP = FI * (((FM * DA) * (((FN * DE).exp()) - AF)).exp());
                FP
            };
            let FR = FM * AG;
            let FS = FQ * ((FR * EA).exp());
            let FU = (AM - AO) * AG;
            let FV = FT * ((FU * EA).exp());
            let FX = (AM - AP) * AG;
            let FY = FW * ((FX * EA).exp());
            let GA = AZ - AL;
            let GB = FZ * ((GA * DE).exp());
            let GD = AF / (GC * ((AZ * DE).exp()));
            let GF = if GE > A { 1.0 } else { 0.0 };
            let VU;
            let VY;
            if GF != 0.0 {
                let GH = GE * (AF - (GG * DB));
                VU = GH;
                VY = GI;
            } else {
                let GK = GI * (AF + (GJ * DB));
                VU = GE;
                VY = GK;
            }
            let GN = BY * ((AF + (GL * DB)) + ((GM * DB) * DB));
            let GQ = AZ - AF;
            let GR = GP * ((GQ * DE).exp());
            let GT = if GS == AF { 1.0 } else { 0.0 };
            let AGY;
            let AHE;
            if GT != 0.0 {
                let GV = BQ * ((GU * DB).exp());
                let GY = GW * ((GX * DB).exp());
                AGY = GY;
                AHE = GV;
            } else {
                AGY = GW;
                AHE = BQ;
            }
            let HA = if (if GZ > A { 1.0 } else { 0.0 }) != 0.0 && (if G < A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let AGJ;
            let AGO;
            if HA != 0.0 {
                let HC = if BS != 0.0 && (if EJ > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let AGK;
                let AGP;
                if HC != 0.0 {
                    let HD = AS / DJ;
                    let HF = HE / EJ;
                    let HH = (((HD.sqrt()) * HF) * HG) / BR;
                    let HI = (GZ * HH) * HF;
                    let HJ = HB / (HH * HD);
                    AGK = HI;
                    AGP = HJ;
                } else {
                    AGK = GZ;
                    AGP = HB;
                }
                AGJ = AGK;
                AGO = AGP;
            } else {
                AGJ = A;
                AGO = AF;
            }
            let HM = HK * ((HL * DE).exp());
            let HO = if HN > A { 1.0 } else { 0.0 };
            let IK;
            let IM;
            let AJD;
            if HO != 0.0 {
                let HQ = ((((DM * AE) * (((((HP * AQ) * AG).exp()) - (((-5e-1f64 * HP) * AG).exp())).ln())) * DD) + (AT * (AF - DD))) - ((AX * CZ) * DE);
                let HR = HQ + ((DM * CZ) * ((AQ * (AF + ((AF + (DP * (((-HQ) * DA).exp()))).sqrt()))).ln()));
                let HT = HN * ((HS * ((HP / HR).ln())).exp());
                let HV = HU.abs();
                let HW = if HU > A { 1.0 } else { 0.0 };
                let AJE = if HW != 0.0 {
                    let HX = (HU * HR) / HP;
                    HX
                } else {
                    HV
                };
                IK = HR;
                IM = HT;
                AJD = AJE;
            } else {
                IK = HP;
                IM = HN;
                AJD = HU;
            }
            let HZ = HY * EB;
            let IC = AX / IB;
            let ID = IA * (((IC * DE) + (EH / IB)).exp());
            let IG = if (if IE > A { 1.0 } else { 0.0 }) != 0.0 && (if (if J < IF { 1.0 } else { 0.0 }) != 0.0 || (if E < IF { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let AJQ;
            let AJT;
            if IG != 0.0 {
                let IH = AR / DI;
                let IJ = if (if (if II == AF { 1.0 } else { 0.0 }) != 0.0 && HO != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if HP > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let IT;
                let IX;
                if IJ != 0.0 {
                    let IL = IK / HP;
                    let IN = (((IM / HN) * (IH.sqrt())) * IL) * IL;
                    let IO = ((HN / IM) * (IH.powf(-1.5e0f64))) / IL;
                    IT = IN;
                    IX = IO;
                } else {
                    let IP = if (if (if II == A { 1.0 } else { 0.0 }) != 0.0 && DL != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if DN > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let IU;
                    let IY;
                    if IP != 0.0 {
                        let IR = (((IQ / DK) * (IH.sqrt())) * EZ) * EZ;
                        let IS = ((DK / IQ) * (IH.powf(-1.5e0f64))) / EZ;
                        IU = IR;
                        IY = IS;
                    } else {
                        IU = AF;
                        IY = AF;
                    }
                    IT = IU;
                    IX = IY;
                }
                let IV = IE * IT;
                let IZ = IW * IX;
                AJQ = IV;
                AJT = IZ;
            } else {
                AJQ = A;
                AJT = AF;
            }
            let JC = JA * (((-(EY - DN)) / JB).exp());
            let JN;
            let AKG;
            let AKN;
            if JD != 0.0 {
                let JF = ((((DM * AE) * (((((JE * AQ) * AG).exp()) - (((-5e-1f64 * JE) * AG).exp())).ln())) * DD) + (AU * (AF - DD))) - ((AX * CZ) * DE);
                let JG = JF + ((DM * CZ) * ((AQ * (AF + ((AF + (DP * (((-JF) * DA).exp()))).sqrt()))).ln()));
                let JI = (JH * ((JE / JG).ln())).exp();
                let JK = JJ.abs();
                let JL = if JJ > A { 1.0 } else { 0.0 };
                let AKO = if JL != 0.0 {
                    let JM = (JJ * JG) / JE;
                    JM
                } else {
                    JK
                };
                JN = JI;
                AKG = JG;
                AKN = AKO;
            } else {
                JN = AF;
                AKG = JE;
                AKN = JJ;
            }
            let AKM = if BO != 0.0 {
                ES
            } else {
                AKN
            };
            let JP = JN * JO;
            let JR = JN * JQ;
            let JT = JS * (((BB * DE) + EV).exp());
            let LA;
            let AMK;
            let AMS;
            let ANC;
            if BO != 0.0 {
                let JV = if JU > A { 1.0 } else { 0.0 };
                let AML;
                let AMT;
                let AND;
                if JV != 0.0 {
                    let JX = ((((DM * AE) * (((((JW * AQ) * AG).exp()) - (((-5e-1f64 * JW) * AG).exp())).ln())) * DD) + (AW * (AF - DD))) - ((AX * CZ) * DE);
                    let JY = JX + ((DM * CZ) * ((AQ * (AF + ((AF + (DP * (((-JX) * DA).exp()))).sqrt()))).ln()));
                    let KA = JU * ((JZ * ((JW / JY).ln())).exp());
                    let ANE = if KC != 0.0 {
                        let KD = (-2.4e0f64 * JY) / JW;
                        KD
                    } else {
                        KB
                    };
                    AML = KA;
                    AMT = JY;
                    AND = ANE;
                } else {
                    AML = JU;
                    AMT = JW;
                    AND = KE;
                }
                LA = ES;
                AMK = AML;
                AMS = AMT;
                ANC = AND;
            } else {
                let KF = if JU > A { 1.0 } else { 0.0 };
                let AMM;
                let AMU;
                let ANF;
                if KF != 0.0 {
                    let KG = ((((DM * AE) * (((((JW * AQ) * AG).exp()) - (((-5e-1f64 * JW) * AG).exp())).ln())) * DD) + (AW * (AF - DD))) - ((AX * CZ) * DE);
                    let KH = KG + ((DM * CZ) * ((AQ * (AF + ((AF + (DP * (((-KG) * DA).exp()))).sqrt()))).ln()));
                    let KI = JU * ((JZ * ((JW / KH).ln())).exp());
                    let KK = -KJ;
                    let KL = KK.abs();
                    let KM = if KK > A { 1.0 } else { 0.0 };
                    let ANG = if KM != 0.0 {
                        let KN = (KK * KH) / JW;
                        KN
                    } else {
                        KL
                    };
                    AMM = KI;
                    AMU = KH;
                    ANF = ANG;
                } else {
                    let KO = -KJ;
                    AMM = JU;
                    AMU = JW;
                    ANF = KO;
                }
                LA = KJ;
                AMK = AMM;
                AMS = AMU;
                ANC = ANF;
            }
            let KQ = AV * AG;
            let KR = KP * (((BC * DE) + (KQ * EA)).exp());
            let KT = if KS > A { 1.0 } else { 0.0 };
            let ANT;
            let ANZ;
            let AOG;
            if KT != 0.0 {
                let KV = if KU > A { 1.0 } else { 0.0 };
                let ANU;
                let AOA;
                let AOH;
                if KV != 0.0 {
                    let KW = ((((DM * AE) * (((((KS * AQ) * AG).exp()) - (((-5e-1f64 * KS) * AG).exp())).ln())) * DD) + (AW * (AF - DD))) - ((AX * CZ) * DE);
                    let KX = KW + ((DM * CZ) * ((AQ * (AF + ((AF + (DP * (((-KW) * DA).exp()))).sqrt()))).ln()));
                    let KZ = KU * ((KY * ((KS / KX).ln())).exp());
                    let LB = -LA;
                    let LC = LB.abs();
                    let LD = if LB > A { 1.0 } else { 0.0 };
                    let AOI = if LD != 0.0 {
                        let LE = (LB * KX) / KS;
                        LE
                    } else {
                        LC
                    };
                    ANU = KZ;
                    AOA = KX;
                    AOH = AOI;
                } else {
                    let LF = -LA;
                    ANU = KU;
                    AOA = KS;
                    AOH = LF;
                }
                ANT = ANU;
                ANZ = AOA;
                AOG = AOH;
            } else {
                ANT = KU;
                ANZ = KS;
                AOG = LA;
            }
            let LI = LG * ((LH * DE).exp());
            let LL = LJ * ((LK * DE).exp());
            let LO = LM * ((LN * DE).exp());
            let LS = if LP >= LR { 1.0 } else { 0.0 };
            let LT = if LP > A { 1.0 } else { 0.0 };
            let LU = if (if (if LQ != A { 1.0 } else { 0.0 }) != 0.0 && LS != 0.0 { 1.0 } else { 0.0 }) != 0.0 && LT != 0.0 { 1.0 } else { 0.0 };
            let QY;
            let RD;
            let RL;
            let RP;
            let RQ;
            let RT;
            let RV;
            let RW;
            let SP;
            let SS;
            let SU;
            let UQ;
            let UU;
            let UZ;
            let VQ;
            let VS;
            let VW;
            let WG;
            let WH;
            let WP;
            let XR;
            let ZM;
            let ZN;
            let AGA;
            let AGG;
            let AGL;
            let AGW;
            let AHC;
            let AHT;
            let AIL;
            let AIT;
            let AIX;
            let AIZ;
            let AJA;
            let AJO;
            let AJR;
            let AJY;
            let AKC;
            let AKE;
            let AKI;
            let ALA;
            let ALE;
            let AMG;
            let AMO;
            let AMW;
            let ANQ;
            let ANW;
            let AOC;
            let AOW;
            let APN;
            let APQ;
            let APS;
            let ARA;
            if LU != 0.0 {
                let LV = CS + node_potentials[4];
                let LW = if LV < 7.314999999999998e1f64 { 1.0 } else { 0.0 };
                let MA;
                if LW != 0.0 {
                    MA = LX;
                } else {
                    let LY = if LV > 6e2f64 { 1.0 } else { 0.0 };
                    let MB = if LY != 0.0 {
                        LZ
                    } else {
                        LV
                    };
                    MA = MB;
                }
                let MC = AB * MA;
                let MD = AF / MC;
                let ME = MA - Y;
                let MF = Y / MA;
                let MG = MA / Y;
                let MH = MG.ln();
                let MI = (AH * MA) * (MA.ln());
                let MJ = AJ * MA;
                let MK = (AM + MI) + MJ;
                let ML = (MK + ((AO + MI) + MJ)) * AQ;
                let MM = (MK + ((AP + MI) + MJ)) * AQ;
                let NG;
                let OX;
                let RX;
                if DL != 0.0 {
                    let MN = ((((DM * AE) * (((((DN * AQ) * AG).exp()) - (((-5e-1f64 * DN) * AG).exp())).ln())) * MG) + (AT * (AF - MG))) - ((AX * MC) * MH);
                    let MO = MN + ((DM * MC) * ((AQ * (AF + ((AF + (DP * (((-MN) * MD).exp()))).sqrt()))).ln()));
                    let MP = DK * ((DR * ((DN / MO).ln())).exp());
                    let MQ = DT.abs();
                    let MR = if DT > A { 1.0 } else { 0.0 };
                    let RY = if MR != 0.0 {
                        let MS = (DT * MO) / DN;
                        MS
                    } else {
                        MQ
                    };
                    NG = MO;
                    OX = MP;
                    RX = RY;
                } else {
                    NG = DN;
                    OX = DK;
                    RX = DT;
                }
                let MT = AF - MF;
                let MU = ((DY * MH) + (DZ * MT)).exp();
                let MV = DX * MU;
                let MW = EG * MT;
                let MX = ED * (((EF * MH) + (MW / EE)).exp());
                let OA;
                let OC;
                let SW;
                if BS != 0.0 {
                    let MY = ((((DM * AE) * (((((EJ * AQ) * AG).exp()) - (((-5e-1f64 * EJ) * AG).exp())).ln())) * MG) + (AU * (AF - MG))) - ((AX * MC) * MH);
                    let MZ = MY + ((DM * MC) * ((AQ * (AF + ((AF + (DP * (((-MY) * MD).exp()))).sqrt()))).ln()));
                    let NA = BR * ((EM * ((EJ / MZ).ln())).exp());
                    let NB = EO.abs();
                    let NC = if EO > A { 1.0 } else { 0.0 };
                    let SX = if NC != 0.0 {
                        let ND = (EO * MZ) / EJ;
                        ND
                    } else {
                        NB
                    };
                    OA = MZ;
                    OC = NA;
                    SW = SX;
                } else {
                    OA = EJ;
                    OC = BR;
                    SW = EO;
                }
                let SV = if BO != 0.0 {
                    ES
                } else {
                    SW
                };
                let NE = EU * MT;
                let NF = ET * (((BA * MH) + NE).exp());
                let NH = NG / DN;
                let NI = EX * (DM - ((DR * (NH.ln())).exp()));
                let NJ = FB * (((FC * MH) + (FD * MT)).exp());
                let NK = FF * ((FG * MH).exp());
                let UV = if FK != 0.0 {
                    let NL = FL * (((FM * MD) * (((FN * MH).exp()) - AF)).exp());
                    NL
                } else {
                    let NM = FI * (((FM * MD) * (((FN * MH).exp()) - AF)).exp());
                    NM
                };
                let NN = FQ * ((FR * MT).exp());
                let NO = FT * ((FU * MT).exp());
                let NP = FW * ((FX * MT).exp());
                let NQ = FZ * ((GA * MH).exp());
                let NR = AF / (GC * ((AZ * MH).exp()));
                let VT;
                let VX;
                if GF != 0.0 {
                    let NS = GE * (AF - (GG * ME));
                    VT = NS;
                    VX = GI;
                } else {
                    let NT = GI * (AF + (GJ * ME));
                    VT = GE;
                    VX = NT;
                }
                let NU = BY * ((AF + (GL * ME)) + ((GM * ME) * ME));
                let NV = GP * ((GQ * MH).exp());
                let AGX;
                let AHD;
                if GT != 0.0 {
                    let NW = BQ * ((GU * ME).exp());
                    let NX = GW * ((GX * ME).exp());
                    AGX = NX;
                    AHD = NW;
                } else {
                    AGX = GW;
                    AHD = BQ;
                }
                let AGH;
                let AGM;
                if HA != 0.0 {
                    let NY = if BS != 0.0 && (if EJ > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let AGI;
                    let AGN;
                    if NY != 0.0 {
                        let NZ = AS / MM;
                        let OB = OA / EJ;
                        let OD = (((NZ.sqrt()) * OB) * OC) / BR;
                        let OE = (GZ * OD) * OB;
                        let OF = HB / (OD * NZ);
                        AGI = OE;
                        AGN = OF;
                    } else {
                        AGI = GZ;
                        AGN = HB;
                    }
                    AGH = AGI;
                    AGM = AGN;
                } else {
                    AGH = A;
                    AGM = AF;
                }
                let OG = HK * ((HL * MH).exp());
                let OR;
                let OT;
                let AJB;
                if HO != 0.0 {
                    let OH = ((((DM * AE) * (((((HP * AQ) * AG).exp()) - (((-5e-1f64 * HP) * AG).exp())).ln())) * MG) + (AT * (AF - MG))) - ((AX * MC) * MH);
                    let OI = OH + ((DM * MC) * ((AQ * (AF + ((AF + (DP * (((-OH) * MD).exp()))).sqrt()))).ln()));
                    let OJ = HN * ((HS * ((HP / OI).ln())).exp());
                    let OK = HU.abs();
                    let OL = if HU > A { 1.0 } else { 0.0 };
                    let AJC = if OL != 0.0 {
                        let OM = (HU * OI) / HP;
                        OM
                    } else {
                        OK
                    };
                    OR = OI;
                    OT = OJ;
                    AJB = AJC;
                } else {
                    OR = HP;
                    OT = HN;
                    AJB = HU;
                }
                let ON = HY * MU;
                let OO = IA * (((IC * MH) + (MW / IB)).exp());
                let AJP;
                let AJS;
                if IG != 0.0 {
                    let OP = AR / ML;
                    let OQ = if (if (if II == AF { 1.0 } else { 0.0 }) != 0.0 && HO != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if HP > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let PA;
                    let PD;
                    if OQ != 0.0 {
                        let OS = OR / HP;
                        let OU = (((OT / HN) * (OP.sqrt())) * OS) * OS;
                        let OV = ((HN / OT) * (OP.powf(-1.5e0f64))) / OS;
                        PA = OU;
                        PD = OV;
                    } else {
                        let OW = if (if (if II == A { 1.0 } else { 0.0 }) != 0.0 && DL != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if DN > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                        let PB;
                        let PE;
                        if OW != 0.0 {
                            let OY = (((OX / DK) * (OP.sqrt())) * NH) * NH;
                            let OZ = ((DK / OX) * (OP.powf(-1.5e0f64))) / NH;
                            PB = OY;
                            PE = OZ;
                        } else {
                            PB = AF;
                            PE = AF;
                        }
                        PA = PB;
                        PD = PE;
                    }
                    let PC = IE * PA;
                    let PF = IW * PD;
                    AJP = PC;
                    AJS = PF;
                } else {
                    AJP = A;
                    AJS = AF;
                }
                let PG = JA * (((-(NG - DN)) / JB).exp());
                let PO;
                let AKF;
                let AKK;
                if PH != 0.0 {
                    let PI = ((((DM * AE) * (((((JE * AQ) * AG).exp()) - (((-5e-1f64 * JE) * AG).exp())).ln())) * MG) + (AU * (AF - MG))) - ((AX * MC) * MH);
                    let PJ = PI + ((DM * MC) * ((AQ * (AF + ((AF + (DP * (((-PI) * MD).exp()))).sqrt()))).ln()));
                    let PK = (JH * ((JE / PJ).ln())).exp();
                    let PL = JJ.abs();
                    let PM = if JJ > A { 1.0 } else { 0.0 };
                    let AKL = if PM != 0.0 {
                        let PN = (JJ * PJ) / JE;
                        PN
                    } else {
                        PL
                    };
                    PO = PK;
                    AKF = PJ;
                    AKK = AKL;
                } else {
                    PO = AF;
                    AKF = JE;
                    AKK = JJ;
                }
                let AKJ = if BO != 0.0 {
                    ES
                } else {
                    AKK
                };
                let PP = PO * JO;
                let PQ = PO * JQ;
                let PR = JS * (((BB * MH) + NE).exp());
                let QO;
                let AMH;
                let AMP;
                let AMX;
                if BO != 0.0 {
                    let PS = if JU > A { 1.0 } else { 0.0 };
                    let AMI;
                    let AMQ;
                    let AMY;
                    if PS != 0.0 {
                        let PT = ((((DM * AE) * (((((JW * AQ) * AG).exp()) - (((-5e-1f64 * JW) * AG).exp())).ln())) * MG) + (AW * (AF - MG))) - ((AX * MC) * MH);
                        let PU = PT + ((DM * MC) * ((AQ * (AF + ((AF + (DP * (((-PT) * MD).exp()))).sqrt()))).ln()));
                        let PV = JU * ((JZ * ((JW / PU).ln())).exp());
                        let AMZ = if PX != 0.0 {
                            let PY = (-2.4e0f64 * PU) / JW;
                            PY
                        } else {
                            PW
                        };
                        AMI = PV;
                        AMQ = PU;
                        AMY = AMZ;
                    } else {
                        AMI = JU;
                        AMQ = JW;
                        AMY = PZ;
                    }
                    QO = ES;
                    AMH = AMI;
                    AMP = AMQ;
                    AMX = AMY;
                } else {
                    let QA = if JU > A { 1.0 } else { 0.0 };
                    let AMJ;
                    let AMR;
                    let ANA;
                    if QA != 0.0 {
                        let QB = ((((DM * AE) * (((((JW * AQ) * AG).exp()) - (((-5e-1f64 * JW) * AG).exp())).ln())) * MG) + (AW * (AF - MG))) - ((AX * MC) * MH);
                        let QC = QB + ((DM * MC) * ((AQ * (AF + ((AF + (DP * (((-QB) * MD).exp()))).sqrt()))).ln()));
                        let QD = JU * ((JZ * ((JW / QC).ln())).exp());
                        let QE = -KJ;
                        let QF = QE.abs();
                        let QG = if QE > A { 1.0 } else { 0.0 };
                        let ANB = if QG != 0.0 {
                            let QH = (QE * QC) / JW;
                            QH
                        } else {
                            QF
                        };
                        AMJ = QD;
                        AMR = QC;
                        ANA = ANB;
                    } else {
                        let QI = -KJ;
                        AMJ = JU;
                        AMR = JW;
                        ANA = QI;
                    }
                    QO = KJ;
                    AMH = AMJ;
                    AMP = AMR;
                    AMX = ANA;
                }
                let QJ = KP * (((BC * MH) + (KQ * MT)).exp());
                let ANR;
                let ANX;
                let AOD;
                if KT != 0.0 {
                    let QK = if KU > A { 1.0 } else { 0.0 };
                    let ANS;
                    let ANY;
                    let AOE;
                    if QK != 0.0 {
                        let QL = ((((DM * AE) * (((((KS * AQ) * AG).exp()) - (((-5e-1f64 * KS) * AG).exp())).ln())) * MG) + (AW * (AF - MG))) - ((AX * MC) * MH);
                        let QM = QL + ((DM * MC) * ((AQ * (AF + ((AF + (DP * (((-QL) * MD).exp()))).sqrt()))).ln()));
                        let QN = KU * ((KY * ((KS / QM).ln())).exp());
                        let QP = -QO;
                        let QQ = QP.abs();
                        let QR = if QP > A { 1.0 } else { 0.0 };
                        let AOF = if QR != 0.0 {
                            let QS = (QP * QM) / KS;
                            QS
                        } else {
                            QQ
                        };
                        ANS = QN;
                        ANY = QM;
                        AOE = AOF;
                    } else {
                        let QT = -QO;
                        ANS = KU;
                        ANY = KS;
                        AOE = QT;
                    }
                    ANR = ANS;
                    ANX = ANY;
                    AOD = AOE;
                } else {
                    ANR = KU;
                    ANX = KS;
                    AOD = QO;
                }
                let QU = LG * ((LH * MH).exp());
                let QV = LJ * ((LK * MH).exp());
                let QW = LM * ((LN * MH).exp());
                QY = MC;
                RD = MV;
                RL = MX;
                RP = NJ;
                RQ = MD;
                RT = OX;
                RV = NG;
                RW = RX;
                SP = OC;
                SS = OA;
                SU = SV;
                UQ = NK;
                UU = UV;
                UZ = NI;
                VQ = NU;
                VS = VT;
                VW = VX;
                WG = NQ;
                WH = NR;
                WP = NN;
                XR = NV;
                ZM = NO;
                ZN = NP;
                AGA = NF;
                AGG = AGH;
                AGL = AGM;
                AGW = AGX;
                AHC = AHD;
                AHT = OG;
                AIL = ON;
                AIT = OO;
                AIX = OT;
                AIZ = OR;
                AJA = AJB;
                AJO = AJP;
                AJR = AJS;
                AJY = PG;
                AKC = PQ;
                AKE = AKF;
                AKI = AKJ;
                ALA = PR;
                ALE = PP;
                AMG = AMH;
                AMO = AMP;
                AMW = AMX;
                ANQ = ANR;
                ANW = ANX;
                AOC = AOD;
                AOW = QJ;
                APN = QW;
                APQ = QU;
                APS = QV;
                ARA = MA;
            } else {
                QY = CZ;
                RD = EC;
                RL = EI;
                RP = FE;
                RQ = DA;
                RT = IQ;
                RV = EY;
                RW = RZ;
                SP = HG;
                SS = HE;
                SU = SY;
                UQ = FH;
                UU = UW;
                UZ = FA;
                VQ = GN;
                VS = VU;
                VW = VY;
                WG = GB;
                WH = GD;
                WP = FS;
                XR = GR;
                ZM = FV;
                ZN = FY;
                AGA = EW;
                AGG = AGJ;
                AGL = AGO;
                AGW = AGY;
                AHC = AHE;
                AHT = HM;
                AIL = HZ;
                AIT = ID;
                AIX = IM;
                AIZ = IK;
                AJA = AJD;
                AJO = AJQ;
                AJR = AJT;
                AJY = JC;
                AKC = JR;
                AKE = AKG;
                AKI = AKM;
                ALA = JT;
                ALE = JP;
                AMG = AMK;
                AMO = AMS;
                AMW = ANC;
                ANQ = ANT;
                ANW = ANZ;
                AOC = AOG;
                AOW = KR;
                APN = LO;
                APQ = LI;
                APS = LL;
                ARA = CX;
            }
            let QX = if DX > A { 1.0 } else { 0.0 };
            let AHX;
            if QX != 0.0 {
                let QZ = E / (parameters[15] * QY);
                let RB = if QZ > RA { 1.0 } else { 0.0 };
                let RE;
                let RF;
                if RB != 0.0 {
                    let RC = AF + (QZ - RA);
                    RE = RC;
                    RF = RA;
                } else {
                    RE = AF;
                    RF = QZ;
                }
                let RG = RD * ((RE * (rspice_limexp(RF))) - AF);
                AHX = RG;
            } else {
                AHX = A;
            }
            let RH = if ED > A { 1.0 } else { 0.0 };
            let APX;
            if RH != 0.0 {
                let RI = E / (EE * QY);
                let RJ = if RI > RA { 1.0 } else { 0.0 };
                let RM;
                let RN;
                if RJ != 0.0 {
                    let RK = AF + (RI - RA);
                    RM = RK;
                    RN = RA;
                } else {
                    RM = AF;
                    RN = RI;
                }
                let RO = RL * ((RM * (rspice_limexp(RN))) - AF);
                APX = RO;
            } else {
                APX = A;
            }
            let RR = RP * (rspice_limexp(((E * RQ) / parameters[13])));
            let RS = RP * (rspice_limexp((G * RQ)));
            let RU = if RT > A { 1.0 } else { 0.0 };
            let VC;
            let AFS;
            if RU != 0.0 {
                let SB = RV * (AF - (((-(RW.ln())) / DR).exp()));
                let SC = (SB - E) * RQ;
                let SE = ((SC * SC) + SD).sqrt();
                let SF = (SC + SE) * AQ;
                let SG = SB - (QY * SF);
                let SH = SF / SE;
                let SI = (AF - (SG / RV)).ln();
                let SJ = RT * (((((-DR) * SI).exp()) * SH) + (RW * (AF - SH)));
                let SK = AF - DR;
                let SL = RT * (((RV * (AF - ((SI * SK).exp()))) / SK) + (RW * (E - SG)));
                VC = SL;
                AFS = SJ;
            } else {
                VC = A;
                AFS = A;
            }
            let SO = if SM < SN { 1.0 } else { 0.0 };
            let VD;
            let AFT;
            if SO != 0.0 {
                let SQ = if SP > A { 1.0 } else { 0.0 };
                let VE;
                let AFU;
                if SQ != 0.0 {
                    let SR = EM / DP;
                    let ST = SM - SS;
                    let TB = SS * (AF - (((-(SU.ln())) / EM).exp()));
                    let TC = SU * SP;
                    let TD = SP * (((SR - EM) * ((SM / SS).ln())).exp());
                    let TE = (TB - G) * RQ;
                    let TF = if TE < RA { 1.0 } else { 0.0 };
                    let TM;
                    let TY;
                    if TF != 0.0 {
                        let TG = TE.exp();
                        let TH = AF + TG;
                        let TI = TG / TH;
                        let TJ = TB - (QY * (TH.ln()));
                        TM = TJ;
                        TY = TI;
                    } else {
                        TM = G;
                        TY = AF;
                    }
                    let TL = (TK * ST) + (DP * QY);
                    let TN = (ST + TM) / TL;
                    let TO = if TN < RA { 1.0 } else { 0.0 };
                    let TU;
                    let TZ;
                    if TO != 0.0 {
                        let TP = TN.exp();
                        let TQ = AF + TP;
                        let TR = TP / TQ;
                        let TS = (-ST) + (TL * ((TQ.ln()) - (((-(ST + TB)) / TL).exp())));
                        TU = TS;
                        TZ = TR;
                    } else {
                        TU = TM;
                        TZ = AF;
                    }
                    let TT = (AF - (TM / SS)).ln();
                    let TV = (AF - (TU / SS)).ln();
                    let TW = AF - EM;
                    let TX = AF - SR;
                    let UA = ((((SP * ((TV * (-EM)).exp())) * TY) * TZ) + ((TD * ((TT * (-SR)).exp())) * (AF - TZ))) + (TC * (AF - TY));
                    let UB = (((((SP * (AF - ((TV * TW).exp()))) / TW) + ((TD * (AF - ((TT * TX).exp()))) / TX)) - ((TD * (AF - ((TV * TX).exp()))) / TX)) * SS) + (TC * (G - TM));
                    VE = UB;
                    AFU = UA;
                } else {
                    VE = A;
                    AFU = A;
                }
                VD = VE;
                AFT = AFU;
            } else {
                let UC = if SP > A { 1.0 } else { 0.0 };
                let VF;
                let AFV;
                if UC != 0.0 {
                    let UD = SS * (AF - (((-(SU.ln())) / EM).exp()));
                    let UE = (UD - G) * RQ;
                    let UF = ((UE * UE) + SD).sqrt();
                    let UG = (UE + UF) * AQ;
                    let UH = UD - (QY * UG);
                    let UI = UG / UF;
                    let UJ = (AF - (UH / SS)).ln();
                    let UK = SP * (((((-EM) * UJ).exp()) * UI) + (SU * (AF - UI)));
                    let UL = AF - EM;
                    let UM = SP * (((SS * (AF - ((UJ * UL).exp()))) / UL) + (SU * (G - UH)));
                    VF = UM;
                    AFV = UK;
                } else {
                    VF = A;
                    AFV = A;
                }
                VD = VF;
                AFT = AFV;
            }
            let UN = if FF > A { 1.0 } else { 0.0 };
            let VA;
            if UN != 0.0 {
                let UO = parameters[11] * QY;
                let UP = (RV - E) / UO;
                let UR = UQ * (AF - ((DR * ((AF - ((RV - ((UO * (UP + (((UP * UP) + SD).sqrt()))) * AQ)) / RV)).ln())).exp()));
                let UT = if (UR.abs()) > US { 1.0 } else { 0.0 };
                let VB = if UT != 0.0 {
                    let UX = (UU * ((UR.exp()) - AF)) / UR;
                    UX
                } else {
                    let UY = UU * (AF + (UR * AQ));
                    UY
                };
                VA = VB;
            } else {
                VA = UU;
            }
            let VH = VG * UZ;
            let VI = (((UZ + (VA * VC)) + (parameters[12] * VD)) / VH) - AF;
            let VJ = VH * (AF + ((VI + (((VI * VI) + SD).sqrt())) * AQ));
            let VK = SS * (AF - ((-8.754687373538999e-1f64 / EM).exp()));
            let VL = (VK - G) * RQ;
            let VM = ((VL * VL) + SD).sqrt();
            let VN = (VL + VM) * AQ;
            let VO = VN / VM;
            let VP = ((((-EM) * ((AF - ((VK - (QY * VN)) / SS)).ln())).exp()) * VO) + (ES * (AF - VO));
            let VR = (VQ + (parameters[67] * ((AF / VP) - AF))) + (parameters[68] * (VP - AF));
            let WA = if GF != 0.0 {
                let VV = VS - G;
                VV
            } else {
                let VZ = H - VW;
                VZ
            };
            let WF = if BO != 0.0 {
                let WB = (WA - QY) * RQ;
                let WC = QY + (QY * ((WB + (((WB * WB) + SD).sqrt())) * AQ));
                WC
            } else {
                let WD = WA / AD;
                let WE = AD * ((WD + (((WD * WD) + parameters[80]).sqrt())) * AQ);
                WE
            };
            let WJ = (WF - WG) / parameters[76];
            let WK = ((WF * WH) / ((((AF + ((WI * ((WF / WG).ln())).exp())).ln()) / WI).exp())) * (AF + (AQ * (WJ + (((WJ * WJ) + parameters[81]).sqrt()))));
            let WM = if (if VR > A { 1.0 } else { 0.0 }) != 0.0 || (if WL > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let WR;
            if WM != 0.0 {
                let WN = AQ * VJ;
                let WS = if BO != 0.0 {
                    let WO = WN + ((((WN * WN) + (VR * RR)) + (WL * RS)).sqrt());
                    WO
                } else {
                    let WQ = WN + ((((WN * WN) + ((WP * VQ) * RR)) + (WL * RS)).sqrt());
                    WQ
                };
                WR = WS;
            } else {
                WR = VJ;
            }
            let WT = RR / WR;
            let WU = RS / WR;
            let WV = VR * WT;
            let WW = if Q >= R { 1.0 } else { 0.0 };
            let ZK = if WW != 0.0 {
                let WX = (WP * VQ) * WT;
                WX
            } else {
                let WY = WP * WV;
                WY
            };
            let XA = WZ * WK;
            let XB = if Q >= 3.2e2f64 { 1.0 } else { 0.0 };
            let XC = if (if WT >= XA { 1.0 } else { 0.0 }) != 0.0 || XB != 0.0 { 1.0 } else { 0.0 };
            let ZS;
            let ZU;
            let AHQ;
            if XC != 0.0 {
                let XD = WT / WK;
                let XF = ((GO * ((XE * (XD.ln())).exp())) * WT) / (AF + XE);
                let XH = if XG < (VG * (FZ / GC)) { 1.0 } else { 0.0 };
                let XS;
                if XH != 0.0 {
                    XS = A;
                } else {
                    let XI = (WT - WK) / XG;
                    let XJ = if XI < -1e10f64 { 1.0 } else { 0.0 };
                    let XL = if XJ != 0.0 {
                        XK
                    } else {
                        XI
                    };
                    let XO = XN * ((-2e0f64 / (XL + (((XL * XL) + XM).sqrt()))).exp());
                    XS = XO;
                }
                let XQ = AF - XP;
                let XT = (XQ * XR) * (((XS * RQ).exp()) - AF);
                let XU = AF - (AF / XD);
                let XW = (XU + (((XU * XU) + XV).sqrt())) / (AF + ((AF + XV).sqrt()));
                let XX = ((XS - XN) * RQ).exp();
                let XY = ((XR * XW) * XW) * XX;
                let YA = if (if (if (if CA < CB { 1.0 } else { 0.0 }) != 0.0 && (if CC < CB { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if (XW * CA) < XZ { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if (XW * CC) < XZ { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let ZI;
                if YA != 0.0 {
                    let YB = (XP * XY) * WT;
                    ZI = YB;
                } else {
                    let YC = AF - XW;
                    let YD = YC - AF;
                    let YF = if (YE.abs()) > US { 1.0 } else { 0.0 };
                    let ZE;
                    if YF != 0.0 {
                        let YI = (YD * YG).exp();
                        let YK = if YJ < CB { 1.0 } else { 0.0 };
                        let ZF = if YK != 0.0 {
                            let YM = (AF - YI) / (YI * YL);
                            let YN = YL * YM;
                            let YP = ((DM * ((YN * (AQ + ((YO * YL) * YM))) - (AQ * ((AF + YN).ln())))) / YL) / YL;
                            YP
                        } else {
                            let YQ = (YI - AF) / (CC - (YI * CA));
                            let YV = YR * YT;
                            let ZA = YW * YY;
                            let ZB = ((((((AF + (CC * YQ)).ln()) * (AQ - YV)) * YT) + ((YV + (YR * YQ)) * YQ)) - (((((AF + (CA * YQ)).ln()) * (AQ - ZA)) * YY) + ((ZA + (YW * YQ)) * YQ))) / YE;
                            ZB
                        };
                        ZE = ZF;
                    } else {
                        let ZC = (AF - YC) / (AF + (YC * CA));
                        let ZD = ((ZC * ZC) * (AF + ((YR * DM) * ZC))) / (AF + (CA * ZC));
                        ZE = ZD;
                    }
                    let ZG = (((XP * XR) * XX) * ZE) * WT;
                    ZI = ZG;
                }
                let ZH = (XT * WT) + ((XQ * XY) * WT);
                let ZT;
                let ZV;
                if WW != 0.0 {
                    let ZJ = ((WV + ZH) + XF) + ZI;
                    let ZO = ((ZK + (ZL * ZH)) + (ZM * XF)) + (ZN * ZI);
                    ZT = ZO;
                    ZV = ZJ;
                } else {
                    let ZP = (((WP * WV) + ZH) + (ZM * XF)) + (ZN * ZI);
                    let ZQ = ((WV + ZH) + XF) + ZI;
                    ZT = ZP;
                    ZV = ZQ;
                }
                ZS = ZT;
                ZU = ZV;
                AHQ = ZH;
            } else {
                ZS = ZK;
                ZU = WV;
                AHQ = A;
            }
            let ZR = WL * WU;
            let ZW = if (if WW != 0.0 && (if ZS > ((ctx.simparam_or("reltol", FJ)) * WR) { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if BO != 0.0 && (if ZU > ((ctx.simparam_or("reltol", FJ)) * WR) { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let AFM;
            let AFN;
            let AFP;
            let AHO;
            if ZW != 0.0 {
                let ZY = (VJ + ((WV * ZS).sqrt())) + (ZX * ZR);
                let mut ZZ = 0.0;
                let mut AAA = 0.0;
                let mut AAB = 0.0;
                ZZ = ZY;
                AAA = ZY;
                AAB = A;
                loop {
                    let AAC = if (if (ZZ.abs()) >= ((ctx.simparam_or("reltol", FJ)) * (AAA.abs())) { 1.0 } else { 0.0 }) != 0.0 && (if AAB <= SN { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    if AAC == 0.0 {
                        break;
                    }
                    let AAD = RR / AAA;
                    let AAE = RS / AAA;
                    let AAF = VR * AAD;
                    let ADC;
                    let ADE;
                    if WW != 0.0 {
                        let AAG = WP * VQ;
                        let AAH = AAG * AAD;
                        ADC = AAH;
                        ADE = AAG;
                    } else {
                        let AAI = WP * AAF;
                        let AAJ = WP * VR;
                        ADC = AAI;
                        ADE = AAJ;
                    }
                    let AAK = if (if AAD >= XA { 1.0 } else { 0.0 }) != 0.0 || XB != 0.0 { 1.0 } else { 0.0 };
                    let ADJ;
                    let ADL;
                    if AAK != 0.0 {
                        let AAL = AAD / WK;
                        let AAM = GO * ((XE * (AAL.ln())).exp());
                        let AAN = (AAM * AAD) / (AF + XE);
                        let AAO = if XG < (VG * (FZ / GC)) { 1.0 } else { 0.0 };
                        let AAZ;
                        let ABC;
                        if AAO != 0.0 {
                            AAZ = A;
                            ABC = A;
                        } else {
                            let AAP = (AAD - WK) / XG;
                            let AAQ = if AAP < -1e10f64 { 1.0 } else { 0.0 };
                            let AAS = if AAQ != 0.0 {
                                AAR
                            } else {
                                AAP
                            };
                            let AAT = ((AAS * AAS) + XM).sqrt();
                            let AAU = AAS + AAT;
                            let AAV = XN * ((-2e0f64 / AAU).exp());
                            let AAW = (DM * AAV) / ((XG * AAT) * AAU);
                            AAZ = AAV;
                            ABC = AAW;
                        }
                        let AAX = AF - XP;
                        let AAY = AAX * XR;
                        let ABA = (AAZ * RQ).exp();
                        let ABB = AAY * (ABA - AF);
                        let ABD = ABB + ((((AAY * AAD) * ABA) * RQ) * ABC);
                        let ABE = AF - (AF / AAL);
                        let ABF = ((ABE * ABE) + XV).sqrt();
                        let ABG = (ABE + ABF) / (AF + ((AF + XV).sqrt()));
                        let ABH = ((AAZ - XN) * RQ).exp();
                        let ABI = ((XR * ABG) * ABG) * ABH;
                        let ABJ = ABI * ((AF + (DM / (AAL * ABF))) + ((RQ * AAD) * ABC));
                        let ABK = if (if (if (if CA < CB { 1.0 } else { 0.0 }) != 0.0 && (if CC < CB { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if (ABG * CA) < XZ { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if (ABG * CC) < XZ { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                        let ADA;
                        let ADB;
                        if ABK != 0.0 {
                            let ABL = (XP * ABI) * AAD;
                            let ABM = XP * ABJ;
                            ADA = ABL;
                            ADB = ABM;
                        } else {
                            let ABN = AF - ABG;
                            let ABO = ABN - AF;
                            let ABP = (ABO * (AF - ABE)) / (ABF * AAD);
                            let ABQ = if (YE.abs()) > US { 1.0 } else { 0.0 };
                            let ACR;
                            let ACV;
                            if ABQ != 0.0 {
                                let ABR = (ABO * YG).exp();
                                let ABS = if YJ < CB { 1.0 } else { 0.0 };
                                let ACS;
                                let ACW;
                                if ABS != 0.0 {
                                    let ABT = ABR * YL;
                                    let ABU = (AF - ABR) / ABT;
                                    let ABV = YL * ABU;
                                    let ABW = AF + ABV;
                                    let ABX = ((DM * ((ABV * (AQ + ((YO * YL) * ABU))) - (AQ * (ABW.ln())))) / YL) / YL;
                                    let ABY = (((AF + ABW) * ABU) * (((-YG) * ABP) / ABT)) / ABW;
                                    ACS = ABX;
                                    ACW = ABY;
                                } else {
                                    let ABZ = CC - (ABR * CA);
                                    let ACA = (ABR - AF) / ABZ;
                                    let ACB = AF + (CC * ACA);
                                    let ACC = YR * YT;
                                    let ACD = AQ - ACC;
                                    let ACE = YR * ACA;
                                    let ACF = AF + (CA * ACA);
                                    let ACG = YW * YY;
                                    let ACH = AQ - ACG;
                                    let ACI = YW * ACA;
                                    let ACJ = (((((ACB.ln()) * ACD) * YT) + ((ACC + ACE) * ACA)) - ((((ACF.ln()) * ACH) * YY) + ((ACG + ACI) * ACA))) / YE;
                                    let ACK = (((((ACD / ACB) + ACC) + (ACE * DM)) - (((ACH / ACF) + ACG) + (ACI * DM))) * (((((-2e0f64 * YE) / (ABZ * ABZ)) * ABR) * YG) * ABP)) / YE;
                                    ACS = ACJ;
                                    ACW = ACK;
                                }
                                ACR = ACS;
                                ACV = ACW;
                            } else {
                                let ACL = AF + (ABN * CA);
                                let ACM = (AF - ABN) / ACL;
                                let ACN = AF + (CA * ACM);
                                let ACO = ((ACM * ACM) * (AF + ((YR * DM) * ACM))) / ACN;
                                let ACP = (ACM * (AF + (AF / (ACN * ACN)))) * (((-ABP) * ACN) / ACL);
                                ACR = ACO;
                                ACV = ACP;
                            }
                            let ACQ = (XP * XR) * ABH;
                            let ACT = ACQ * ACR;
                            let ACU = ACT * AAD;
                            let ACX = (ACT + ((ACU * ABC) * RQ)) + ((ACQ * AAD) * ACV);
                            ADA = ACU;
                            ADB = ACX;
                        }
                        let ACY = AAX * ABJ;
                        let ACZ = (ABB * AAD) + ((AAX * ABI) * AAD);
                        let ADK;
                        let ADM;
                        if WW != 0.0 {
                            let ADD = ((ADC + (ZL * ACZ)) + (ZM * AAN)) + (ZN * ADA);
                            let ADF = ((ADE + (ZL * (ABD + ACY))) + (ZM * AAM)) + (ZN * ADB);
                            ADK = ADD;
                            ADM = ADF;
                        } else {
                            let ADG = (((WP * AAF) + ACZ) + (ZM * AAN)) + (ZN * ADA);
                            let ADH = (((WP * VR) + (ABD + ACY)) + (ZM * AAM)) + (ZN * ADB);
                            ADK = ADG;
                            ADM = ADH;
                        }
                        ADJ = ADK;
                        ADL = ADM;
                    } else {
                        ADJ = ADC;
                        ADL = ADE;
                    }
                    let ADI = (ZX * WL) * AAE;
                    let ADN = (-(AAA - ((VJ + ADJ) + ADI))) / (AF + (((ADL * AAD) + ADI) / AAA));
                    let ADO = (3e-1f64 * AAA).abs();
                    let ADP = if (ADN.abs()) > ADO { 1.0 } else { 0.0 };
                    let ADS;
                    if ADP != 0.0 {
                        let ADQ = if ADN >= A { 1.0 } else { 0.0 };
                        let ADT = if ADQ != 0.0 {
                            ADO
                        } else {
                            let ADR = -ADO;
                            ADR
                        };
                        ADS = ADT;
                    } else {
                        ADS = ADN;
                    }
                    let ADU = AAA + ADS;
                    let ADV = AAB + AF;
                    ZZ = ADS;
                    AAA = ADU;
                    AAB = ADV;
                }
                let ADW = RR / AAA;
                let ADX = RS / AAA;
                let ADY = VR * ADW;
                if WW != 0.0 {
                } else {
                }
                let ADZ = if (if ADW >= XA { 1.0 } else { 0.0 }) != 0.0 || XB != 0.0 { 1.0 } else { 0.0 };
                let AFQ;
                let AHP;
                if ADZ != 0.0 {
                    let AEA = ADW / WK;
                    let AEB = ((GO * ((XE * (AEA.ln())).exp())) * ADW) / (AF + XE);
                    let AEC = if XG < (VG * (FZ / GC)) { 1.0 } else { 0.0 };
                    let AEJ;
                    if AEC != 0.0 {
                        AEJ = A;
                    } else {
                        let AED = (ADW - WK) / XG;
                        let AEE = if AED < -1e10f64 { 1.0 } else { 0.0 };
                        let AEG = if AEE != 0.0 {
                            AEF
                        } else {
                            AED
                        };
                        let AEH = XN * ((-2e0f64 / (AEG + (((AEG * AEG) + XM).sqrt()))).exp());
                        AEJ = AEH;
                    }
                    let AEI = AF - XP;
                    let AEK = (AEI * XR) * (((AEJ * RQ).exp()) - AF);
                    let AEL = AF - (AF / AEA);
                    let AEM = (AEL + (((AEL * AEL) + XV).sqrt())) / (AF + ((AF + XV).sqrt()));
                    let AEN = ((AEJ - XN) * RQ).exp();
                    let AEO = ((XR * AEM) * AEM) * AEN;
                    let AEP = if (if (if (if CA < CB { 1.0 } else { 0.0 }) != 0.0 && (if CC < CB { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if (AEM * CA) < XZ { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if (AEM * CC) < XZ { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let AFJ;
                    if AEP != 0.0 {
                        let AEQ = (XP * AEO) * ADW;
                        AFJ = AEQ;
                    } else {
                        let AER = AF - AEM;
                        let AES = AER - AF;
                        let AET = if (YE.abs()) > US { 1.0 } else { 0.0 };
                        let AFF;
                        if AET != 0.0 {
                            let AEU = (AES * YG).exp();
                            let AEV = if YJ < CB { 1.0 } else { 0.0 };
                            let AFG = if AEV != 0.0 {
                                let AEW = (AF - AEU) / (AEU * YL);
                                let AEX = YL * AEW;
                                let AEY = ((DM * ((AEX * (AQ + ((YO * YL) * AEW))) - (AQ * ((AF + AEX).ln())))) / YL) / YL;
                                AEY
                            } else {
                                let AEZ = (AEU - AF) / (CC - (AEU * CA));
                                let AFA = YR * YT;
                                let AFB = YW * YY;
                                let AFC = ((((((AF + (CC * AEZ)).ln()) * (AQ - AFA)) * YT) + ((AFA + (YR * AEZ)) * AEZ)) - (((((AF + (CA * AEZ)).ln()) * (AQ - AFB)) * YY) + ((AFB + (YW * AEZ)) * AEZ))) / YE;
                                AFC
                            };
                            AFF = AFG;
                        } else {
                            let AFD = (AF - AER) / (AF + (AER * CA));
                            let AFE = ((AFD * AFD) * (AF + ((YR * DM) * AFD))) / (AF + (CA * AFD));
                            AFF = AFE;
                        }
                        let AFH = (((XP * XR) * AEN) * AFF) * ADW;
                        AFJ = AFH;
                    }
                    let AFI = (AEK * ADW) + ((AEI * AEO) * ADW);
                    let AFR = if WW != 0.0 {
                        let AFK = ((ADY + AFI) + AEB) + AFJ;
                        AFK
                    } else {
                        let AFL = ((ADY + AFI) + AEB) + AFJ;
                        AFL
                    };
                    AFQ = AFR;
                    AHP = AFI;
                } else {
                    AFQ = ADY;
                    AHP = A;
                }
                AFM = ADW;
                AFN = ADX;
                AFP = AFQ;
                AHO = AHP;
            } else {
                AFM = WT;
                AFN = WU;
                AFP = ZU;
                AHO = AHQ;
            }
            let AFO = AFM - AFN;
            let AFW = if ET > A { 1.0 } else { 0.0 };
            let APG;
            if AFW != 0.0 {
                let AFX = G / (parameters[24] * QY);
                let AFY = if AFX > RA { 1.0 } else { 0.0 };
                let AGB;
                let AGC;
                if AFY != 0.0 {
                    let AFZ = AF + (AFX - RA);
                    AGB = AFZ;
                    AGC = RA;
                } else {
                    AGB = AF;
                    AGC = AFX;
                }
                let AGD = AGA * ((AGB * (rspice_limexp(AGC))) - AF);
                APG = AGD;
            } else {
                APG = A;
            }
            let AQE;
            if HA != 0.0 {
                let AGE = if (if SP > A { 1.0 } else { 0.0 }) != 0.0 && (if SS > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let AQF = if AGE != 0.0 {
                    let AGF = (((AF / EM) - AF) * ((AFT / SP).ln())).exp();
                    let AGQ = (((-AGG) * G) / (SS * AGF)) * (((-AGL) * AGF).exp());
                    AGQ
                } else {
                    A
                };
                AQE = AQF;
            } else {
                AQE = A;
            }
            let APC;
            if GT != 0.0 {
                let AGR = SS - G;
                let AGS = if AGR > A { 1.0 } else { 0.0 };
                let APD;
                if AGS != 0.0 {
                    let AGU = if AGT > A { 1.0 } else { 0.0 };
                    let AHF = if AGU != 0.0 {
                        let AGV = (TK * ((((((AFT / SP) / TK).exp()) - DM) + (DM * (((AF - (AFM / (((AGT * WG) * WH) + (parameters[36] * AFM)))) / TK).cosh()))).ln())).sqrt();
                        AGV
                    } else {
                        AF
                    };
                    let AGZ = AGW / AFT;
                    let AHA = AGW / SP;
                    let AHB = if AGR > AHA { 1.0 } else { 0.0 };
                    let AHK = if AHB != 0.0 {
                        let AHG = (AHC * (((-AGZ) / (AHA * AHF)).exp())) * (AHA + ((AF + (AGZ / AHA)) * (AGR - AHA)));
                        AHG
                    } else {
                        let AHH = (AHC * AGR) * (((-AGZ) / (AGR * AHF)).exp());
                        AHH
                    };
                    let AHJ = if AHI > A { 1.0 } else { 0.0 };
                    let APE = if AHJ != 0.0 {
                        let AHL = AF - (AHI * AHK);
                        let AHM = (AFM * AHK) / (AQ * (AHL + (((AHL * AHL) + 1e-4f64).sqrt())));
                        AHM
                    } else {
                        let AHN = AFM * AHK;
                        AHN
                    };
                    APD = APE;
                } else {
                    APD = A;
                }
                APC = APD;
            } else {
                APC = A;
            }
            let AHS = AHO * AHR;
            let AHU = if AHT > A { 1.0 } else { 0.0 };
            let APK;
            if AHU != 0.0 {
                let AHV = AF + (((VC + VD) + AFP) / ((AF + parameters[92]) * UZ));
                let AHW = AHT / (AQ * (AHV + (((AHV * AHV) + CB).sqrt())));
                let AHY = if AHX > A { 1.0 } else { 0.0 };
                let AIE;
                if AHY != 0.0 {
                    let AHZ = ((AHW * AHX) * parameters[91]) * RQ;
                    let AIA = if AHZ < WZ { 1.0 } else { 0.0 };
                    let AIF = if AIA != 0.0 {
                        let AIB = AHW * (AF - (AQ * AHZ));
                        AIB
                    } else {
                        let AIC = (AHW * ((AF + AHZ).ln())) / AHZ;
                        AIC
                    };
                    AIE = AIF;
                } else {
                    AIE = AHW;
                }
                let AID = if AFP > A { 1.0 } else { 0.0 };
                let APL = if AID != 0.0 {
                    let AIG = (AIE * (VC + (AFP * parameters[94]))) / (VC + AFP);
                    AIG
                } else {
                    AIE
                };
                APK = APL;
            } else {
                APK = A;
            }
            let AIH = if HY > A { 1.0 } else { 0.0 };
            let APH;
            if AIH != 0.0 {
                let AII = J / (parameters[19] * QY);
                let AIJ = if AII > RA { 1.0 } else { 0.0 };
                let AIM;
                let AIN;
                if AIJ != 0.0 {
                    let AIK = AF + (AII - RA);
                    AIM = AIK;
                    AIN = RA;
                } else {
                    AIM = AF;
                    AIN = AII;
                }
                let AIO = AIL * ((AIM * (rspice_limexp(AIN))) - AF);
                APH = AIO;
            } else {
                APH = A;
            }
            let AIP = if IA > A { 1.0 } else { 0.0 };
            let AQG;
            if AIP != 0.0 {
                let AIQ = J / (IB * QY);
                let AIR = if AIQ > RA { 1.0 } else { 0.0 };
                let AIU;
                let AIV;
                if AIR != 0.0 {
                    let AIS = AF + (AIQ - RA);
                    AIU = AIS;
                    AIV = RA;
                } else {
                    AIU = AF;
                    AIV = AIQ;
                }
                let AIW = AIT * ((AIU * (rspice_limexp(AIV))) - AF);
                AQG = AIW;
            } else {
                AQG = A;
            }
            let AIY = if AIX > A { 1.0 } else { 0.0 };
            let AJM = if AIY != 0.0 {
                let AJF = AIZ * (AF - (((-(AJA.ln())) / HS).exp()));
                let AJG = (AJF - J) * RQ;
                let AJH = ((AJG * AJG) + SD).sqrt();
                let AJI = (AJG + AJH) * AQ;
                let AJJ = AJI / AJH;
                let AJK = AIX * (((((-HS) * ((AF - ((AJF - (QY * AJI)) / AIZ)).ln())).exp()) * AJJ) + (AJA * (AF - AJJ)));
                AJK
            } else {
                A
            };
            let AQB;
            if IG != 0.0 {
                let AJL = if (if (if II == AF { 1.0 } else { 0.0 }) != 0.0 && AIY != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if AIZ > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let AQC;
                if AJL != 0.0 {
                    let AJN = ((AF - (AF / HS)) * ((AJM / AIX).ln())).exp();
                    let AJU = (((-(J / AIZ)) * AJO) * AJN) * (((-AJR) / AJN).exp());
                    AQC = AJU;
                } else {
                    let AJV = if (if (if II == A { 1.0 } else { 0.0 }) != 0.0 && RU != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if RV > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let AQD = if AJV != 0.0 {
                        let AJW = ((AF - (AF / DR)) * ((AFS / RT).ln())).exp();
                        let AJX = (((-(E / RV)) * AJO) * AJW) * (((-AJR) / AJW).exp());
                        AJX
                    } else {
                        A
                    };
                    AQC = AQD;
                }
                AQB = AQC;
            } else {
                AQB = A;
            }
            let AJZ = AJY * (((E / JB).exp()) - AF);
            let AKB = if AKA < SN { 1.0 } else { 0.0 };
            if AKB != 0.0 {
                let AKD = if AKC > A { 1.0 } else { 0.0 };
                if AKD != 0.0 {
                    let AKH = AKA - AKE;
                    let AKP = AKE * (AF - (((-(AKI.ln())) / JH).exp()));
                    let AKQ = (AKP - K) * RQ;
                    let AKR = if AKQ < RA { 1.0 } else { 0.0 };
                    let AKT = if AKR != 0.0 {
                        let AKS = AKP - (QY * ((AF + (AKQ.exp())).ln()));
                        AKS
                    } else {
                        K
                    };
                    let AKU = if ((AKH + AKT) / ((TK * AKH) + (DP * QY))) < RA { 1.0 } else { 0.0 };
                    if AKU != 0.0 {
                    } else {
                    }
                } else {
                }
            } else {
                let AKV = if AKC > A { 1.0 } else { 0.0 };
                if AKV != 0.0 {
                } else {
                }
            }
            let AKW = if JS > A { 1.0 } else { 0.0 };
            let API;
            if AKW != 0.0 {
                let AKX = K / (parameters[26] * QY);
                let AKY = if AKX > RA { 1.0 } else { 0.0 };
                let ALB;
                let ALC;
                if AKY != 0.0 {
                    let AKZ = AF + (AKX - RA);
                    ALB = AKZ;
                    ALC = RA;
                } else {
                    ALB = AF;
                    ALC = AKX;
                }
                let ALD = ALA * ((ALB * (rspice_limexp(ALC))) - AF);
                API = ALD;
            } else {
                API = A;
            }
            let AQH;
            if AKB != 0.0 {
                let ALF = if ALE > A { 1.0 } else { 0.0 };
                let AQI;
                if ALF != 0.0 {
                    let ALG = JH / DP;
                    let ALH = AKA - AKE;
                    let ALI = AKE * (AF - (((-(AKI.ln())) / JH).exp()));
                    let ALJ = AKI * ALE;
                    let ALK = ALE * (((ALG - JH) * ((AKA / AKE).ln())).exp());
                    let ALL = (ALI - N) * RQ;
                    let ALM = if ALL < RA { 1.0 } else { 0.0 };
                    let ALP = if ALM != 0.0 {
                        let ALN = ALI - (QY * ((AF + (ALL.exp())).ln()));
                        ALN
                    } else {
                        N
                    };
                    let ALO = (TK * ALH) + (DP * QY);
                    let ALQ = (ALH + ALP) / ALO;
                    let ALR = if ALQ < RA { 1.0 } else { 0.0 };
                    let ALT = if ALR != 0.0 {
                        let ALS = (-ALH) + (ALO * (((AF + (ALQ.exp())).ln()) - (((-(ALH + ALI)) / ALO).exp())));
                        ALS
                    } else {
                        ALP
                    };
                    let ALU = (AF - (ALT / AKE)).ln();
                    let ALV = AF - JH;
                    let ALW = AF - ALG;
                    let ALX = (((((ALE * (AF - ((ALU * ALV).exp()))) / ALV) + ((ALK * (AF - ((((AF - (ALP / AKE)).ln()) * ALW).exp()))) / ALW)) - ((ALK * (AF - ((ALU * ALW).exp()))) / ALW)) * AKE) + (ALJ * (N - ALP));
                    AQI = ALX;
                } else {
                    AQI = A;
                }
                AQH = AQI;
            } else {
                let ALY = if ALE > A { 1.0 } else { 0.0 };
                let AQJ = if ALY != 0.0 {
                    let ALZ = AKE * (AF - (((-(AKI.ln())) / JH).exp()));
                    let AMA = (ALZ - N) * RQ;
                    let AMB = ALZ - (QY * ((AMA + (((AMA * AMA) + SD).sqrt())) * AQ));
                    let AMC = AF - JH;
                    let AMD = ALE * (((AKE * (AF - ((((AF - (AMB / AKE)).ln()) * AMC).exp()))) / AMC) + (AKI * (N - AMB)));
                    AMD
                } else {
                    A
                };
                AQH = AQJ;
            }
            let AMF = if AME < SN { 1.0 } else { 0.0 };
            if AMF != 0.0 {
                let AMN = if AMG > A { 1.0 } else { 0.0 };
                if AMN != 0.0 {
                    let AMV = AME - AMO;
                    let ANH = AMO * (AF - (((-(AMW.ln())) / JZ).exp()));
                    let ANI = (ANH - O) * RQ;
                    let ANJ = if ANI < RA { 1.0 } else { 0.0 };
                    let ANL = if ANJ != 0.0 {
                        let ANK = ANH - (QY * ((AF + (ANI.exp())).ln()));
                        ANK
                    } else {
                        O
                    };
                    let ANM = if ((AMV + ANL) / ((TK * AMV) + (DP * QY))) < RA { 1.0 } else { 0.0 };
                    if ANM != 0.0 {
                    } else {
                    }
                } else {
                }
            } else {
                let ANN = if AMG > A { 1.0 } else { 0.0 };
                if ANN != 0.0 {
                } else {
                }
            }
            if KT != 0.0 {
                let ANP = if ANO < SN { 1.0 } else { 0.0 };
                if ANP != 0.0 {
                    let ANV = if ANQ > A { 1.0 } else { 0.0 };
                    if ANV != 0.0 {
                        let AOB = ANO - ANW;
                        let AOJ = ANW * (AF - (((-(AOC.ln())) / KY).exp()));
                        let AOK = (AOJ - P) * RQ;
                        let AOL = if AOK < RA { 1.0 } else { 0.0 };
                        let AON = if AOL != 0.0 {
                            let AOM = AOJ - (QY * ((AF + (AOK.exp())).ln()));
                            AOM
                        } else {
                            P
                        };
                        let AOO = if ((AOB + AON) / ((TK * AOB) + (DP * QY))) < RA { 1.0 } else { 0.0 };
                        if AOO != 0.0 {
                        } else {
                        }
                    } else {
                    }
                } else {
                    let AOP = if ANQ > A { 1.0 } else { 0.0 };
                    if AOP != 0.0 {
                    } else {
                    }
                }
            } else {
            }
            let AOQ = if parameters[97] > A { 1.0 } else { 0.0 };
            if AOQ != 0.0 {
                let AOR = if parameters[101] > A { 1.0 } else { 0.0 };
                if AOR != 0.0 {
                } else {
                }
            } else {
            }
            let AOS = if KP > A { 1.0 } else { 0.0 };
            let APJ;
            if AOS != 0.0 {
                let AOT = O / (parameters[100] * QY);
                let AOU = if AOT > RA { 1.0 } else { 0.0 };
                let AOX;
                let AOY;
                if AOU != 0.0 {
                    let AOV = AF + (AOT - RA);
                    AOX = AOV;
                    AOY = RA;
                } else {
                    AOX = AF;
                    AOY = AOT;
                }
                let AOZ = AOW * ((AOX * (rspice_limexp(AOY))) - AF);
                APJ = AOZ;
            } else {
                APJ = A;
            }
            let APA = if LS != 0.0 && LT != 0.0 { 1.0 } else { 0.0 };
            if APA != 0.0 {
                let APB = if LQ == AF { 1.0 } else { 0.0 };
                if APB != 0.0 {
                } else {
                    let APF = if LQ == DM { 1.0 } else { 0.0 };
                    if APF != 0.0 {
                        let APM = if (if APK >= LR { 1.0 } else { 0.0 }) != 0.0 && (if APK > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                        if APM != 0.0 {
                        } else {
                        }
                        let APO = if (if APN >= LR { 1.0 } else { 0.0 }) != 0.0 && (if APN > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                        if APO != 0.0 {
                        } else {
                        }
                        let APR = if (if APQ >= LR { 1.0 } else { 0.0 }) != 0.0 && (if APQ > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                        if APR != 0.0 {
                        } else {
                        }
                        let APT = if (if APS >= LR { 1.0 } else { 0.0 }) != 0.0 && (if APS > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                        if APT != 0.0 {
                        } else {
                        }
                    } else {
                    }
                }
            } else {
            }
            let APW = if APU != A { 1.0 } else { 0.0 };
            if APW != 0.0 {
            } else {
            }
            let APY = if (if HK >= LR { 1.0 } else { 0.0 }) != 0.0 && (if HK > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            if APY != 0.0 {
                let APZ = if parameters[93] > A { 1.0 } else { 0.0 };
                if APZ != 0.0 {
                } else {
                }
            } else {
            }
            let AQA = if II == AF { 1.0 } else { 0.0 };
            if AQA != 0.0 {
            } else {
            }
            let AQK = B * AQH;
            let AQL = 0e0f64;
            let AQN = AQM * M;
            let AQO = 0e0f64;
            let AQP = if (if LJ >= LR { 1.0 } else { 0.0 }) != 0.0 && (if LJ > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let ASC = if AQP != 0.0 {
                let AQQ = (L - I) / APS;
                AQQ
            } else {
                A
            };
            let AQR = if (if LM >= LR { 1.0 } else { 0.0 }) != 0.0 && (if LM > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            if AQR != 0.0 {
            } else {
            }
            let AQS = if (if LG >= LR { 1.0 } else { 0.0 }) != 0.0 && (if LG > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            if AQS != 0.0 {
            } else {
            }
            let AQT = BK * (L - APP);
            let AQU = 0e0f64;
            if XB != 0.0 {
                if AOS != 0.0 {
                } else {
                }
            } else {
                if WW != 0.0 {
                } else {
                }
            }
            let AQW = if (if AQV >= LR { 1.0 } else { 0.0 }) != 0.0 && (if AQV > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            if AQW != 0.0 {
                let AQX = if parameters[103] > A { 1.0 } else { 0.0 };
                if AQX != 0.0 {
                } else {
                }
            } else {
            }
            let AQY = if (if (if LQ >= AF { 1.0 } else { 0.0 }) != 0.0 && LS != 0.0 { 1.0 } else { 0.0 }) != 0.0 && LT != 0.0 { 1.0 } else { 0.0 };
            if AQY != 0.0 {
                let AQZ = if parameters[145] > A { 1.0 } else { 0.0 };
                if AQZ != 0.0 {
                } else {
                }
            } else {
            }
            let ARB = (DP * Z) * ARA;
            let ASD;
            let ASP;
            let ASQ;
            if AQP != 0.0 {
                let ARD = ARB / APS;
                ASD = ARC;
                ASP = AF;
                ASQ = ARD;
            } else {
                ASD = A;
                ASP = A;
                ASQ = A;
            }
            let ASR;
            let ASS;
            if APY != 0.0 {
                let ARE = ARB / APK;
                ASR = AF;
                ASS = ARE;
            } else {
                ASR = A;
                ASS = A;
            }
            let AST;
            let ASU;
            if AQS != 0.0 {
                let ARF = ARB / APQ;
                AST = AF;
                ASU = ARF;
            } else {
                AST = A;
                ASU = A;
            }
            let ASV;
            let ASW;
            if AQR != 0.0 {
                let ARG = ARB / APN;
                ASV = AF;
                ASW = ARG;
            } else {
                ASV = A;
                ASW = A;
            }
            let ASX;
            let ASY;
            if AQW != 0.0 {
                let ARH = ARB / AQV;
                ASX = AF;
                ASY = ARH;
            } else {
                ASX = A;
                ASY = A;
            }
            let ARI = parameters[110] * (((AHX + APH).abs()).powf(parameters[111]));
            let ARJ = if parameters[112] == -1e0f64 { 1.0 } else { 0.0 };
            let ASZ;
            let ATA;
            let ATB;
            let ATC;
            let ATD;
            let ATE;
            if ARJ != 0.0 {
                ASZ = AF;
                ATA = ARI;
                ATB = AF;
                ATC = A;
                ATD = A;
                ATE = A;
            } else {
                ASZ = A;
                ATA = A;
                ATB = A;
                ATC = AF;
                ATD = ARI;
                ATE = AF;
            }
            let ATF;
            let ATG;
            let ATH;
            if AQR != 0.0 {
                let ARK = parameters[113] * ((((D - APP) / APN).abs()).powf(parameters[114]));
                ATF = AF;
                ATG = ARK;
                ATH = AF;
            } else {
                ATF = A;
                ATG = A;
                ATH = A;
            }
            let ARL = DM * AA;
            let ATI;
            let ATJ;
            if XB != 0.0 {
                let ARM = ARL * (AQB.abs());
                ATI = AF;
                ATJ = ARM;
            } else {
                ATI = A;
                ATJ = A;
            }
            let ARN = ARL * (APH.abs());
            let ARO = ARL * APC;
            let ARP = ARL * (APG.abs());
            let ARQ = ARL * (AQE.abs());
            let ARR = ARL * (API.abs());
            let ARS = ARL * (APJ.abs());
            let ART = if (if parameters[109] == AF { 1.0 } else { 0.0 }) != 0.0 && (if (if BW > A { 1.0 } else { 0.0 }) != 0.0 && (if BX > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let ATK;
            let ATL;
            let ATM;
            let ATN;
            let ATO;
            let ATP;
            let ATQ;
            let ATR;
            if ART != 0.0 {
                let ARU = if AHX > A { 1.0 } else { 0.0 };
                let ARW = if ARU != 0.0 {
                    let ARV = AFO / AHX;
                    ARV
                } else {
                    CI
                };
                let ARX = if (ARW * ((DM * BX) - (BW * BW))) > A { 1.0 } else { 0.0 };
                if ARX != 0.0 {
                } else {
                }
                let ARY = ARL * (AHX.abs());
                let ARZ = ARL * (AFO.abs());
                ATK = AF;
                ATL = ARY;
                ATM = AF;
                ATN = ARZ;
                ATO = A;
                ATP = A;
                ATQ = A;
                ATR = A;
            } else {
                let ASA = ARL * (AFO.abs());
                let ASB = ARL * (AHX.abs());
                ATK = A;
                ATL = A;
                ATM = A;
                ATN = A;
                ATO = AF;
                ATP = ASA;
                ATQ = AF;
                ATR = ASB;
            }
            let ASE = if (((((AQL + AQO) + ASC) + AQU) + ASD) + branch_unknown_flows[1]) != A { 1.0 } else { 0.0 };
            if ASE != 0.0 {
            } else {
            }
            let ASF = (((((-0e0f64) - 0e0f64) - ((-0e0f64) - 0e0f64)) + (-0e0f64)) - (-0e0f64)) + ((-0e0f64) - 0e0f64);
            let ASG = if (ASF.abs()) > X { 1.0 } else { 0.0 };
            if ASG != 0.0 {
            } else {
                let ASH = if ASF >= A { 1.0 } else { 0.0 };
                if ASH != 0.0 {
                } else {
                }
            }
            let ASI = -0e0f64;
            let ASJ = ((((-0e0f64) - (-0e0f64)) - ASI) + (-0e0f64)) + (-0e0f64);
            let ASK = if (ASJ.abs()) > X { 1.0 } else { 0.0 };
            if ASK != 0.0 {
            } else {
                let ASL = if ASJ >= A { 1.0 } else { 0.0 };
                if ASL != 0.0 {
                } else {
                }
            }
            let ASM = 0e0f64 - ASI;
            let ASN = if (ASM.abs()) > X { 1.0 } else { 0.0 };
            if ASN != 0.0 {
            } else {
                let ASO = if ASM >= A { 1.0 } else { 0.0 };
                if ASO != 0.0 {
                } else {
                }
            }
        if ASP == 0.0 {
            if !visitor.visit(0, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = ASQ;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 0, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 0, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(0, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if ASR == 0.0 {
            if !visitor.visit(1, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = ASS;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(1, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if AST == 0.0 {
            if !visitor.visit(2, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = ASU;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 2, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 2, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(2, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if ASV == 0.0 {
            if !visitor.visit(3, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = ASW;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 3, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 3, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(3, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if ASX == 0.0 {
            if !visitor.visit(4, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = ASY;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 4, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 4, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(4, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if ASZ == 0.0 {
            if !visitor.visit(5, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = ATA;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 5, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = Some(ATB);
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 5, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 5, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(5, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if ATC == 0.0 {
            if !visitor.visit(6, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = ATD;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 6, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = Some(ATE);
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 6, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 6, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(6, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if ATF == 0.0 {
            if !visitor.visit(7, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = ATG;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 7, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = Some(ATH);
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 7, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 7, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(7, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if ATI == 0.0 {
            if !visitor.visit(8, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = ATJ;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 8, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 8, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(8, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = ARN;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 9, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 9, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(9, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = ARO;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 10, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 10, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(10, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = ARP;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 11, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 11, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(11, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = ARQ;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 12, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 12, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(12, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = ARR;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 13, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 13, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(13, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = ARS;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 14, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 14, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(14, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if ATK == 0.0 {
            if !visitor.visit(15, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = ATL;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 15, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 15, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(15, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if ATM == 0.0 {
            if !visitor.visit(16, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = ATN;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 16, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 16, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(16, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if ATO == 0.0 {
            if !visitor.visit(17, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = ATP;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 17, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 17, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(17, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if ATQ == 0.0 {
            if !visitor.visit(18, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = ATR;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 18, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 18, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(18, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        Ok(())
    }
}
