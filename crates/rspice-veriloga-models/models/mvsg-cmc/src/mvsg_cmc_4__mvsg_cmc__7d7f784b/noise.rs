#![allow(dead_code, non_snake_case, unused_parens, unused_variables)]

use super::state::Instance;
use rspice_veriloga_runtime::GeneratedEvalContext;
pub use rspice_veriloga_runtime::{GeneratedNoiseDescriptor, GeneratedNoiseEndpoint, GeneratedNoiseEvaluation, GeneratedNoiseEvaluationError, GeneratedNoiseEvaluationRef, GeneratedNoiseKind, GeneratedNoiseVisitor};

use rspice_veriloga_runtime::Lanes;
pub static NOISE_SOURCES: [GeneratedNoiseDescriptor; 16] = [
    GeneratedNoiseDescriptor { mechanism: "WHITE_GI2P_SI_G_S_SHOT_INT", label: Some("g-s shot int"), kind: GeneratedNoiseKind::White, equation: 178, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(8), name: "gi2p", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(9), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_GI2P_DI_G_D_SHOT_INT", label: Some("g-d shot int"), kind: GeneratedNoiseKind::White, equation: 179, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(8), name: "gi2p", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(5), name: "di", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_GI2P_FPS4_G_S_SHOT_EXT", label: Some("g-s shot ext"), kind: GeneratedNoiseKind::White, equation: 180, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(8), name: "gi2p", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(13), name: "fps4", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_GI2P_FP4_G_D_SHOT_EXT", label: Some("g-d shot ext"), kind: GeneratedNoiseKind::White, equation: 181, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(8), name: "gi2p", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(17), name: "fp4", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_DI_SI_FLICKER", label: Some("flicker"), kind: GeneratedNoiseKind::Flicker, equation: 182, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "di", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(9), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_DI_SI_CHANNEL", label: Some("channel"), kind: GeneratedNoiseKind::White, equation: 183, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "di", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(9), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_SI_FPS1_RFPS1", label: Some("rfps1"), kind: GeneratedNoiseKind::White, equation: 184, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(9), name: "si", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(10), name: "fps1", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_FPS1_FPS2_RFPS2", label: Some("rfps2"), kind: GeneratedNoiseKind::White, equation: 185, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(10), name: "fps1", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(11), name: "fps2", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_FPS2_FPS3_RFPS3", label: Some("rfps3"), kind: GeneratedNoiseKind::White, equation: 186, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(11), name: "fps2", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(12), name: "fps3", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_FPS3_FPS4_RFPS4", label: Some("rfps4"), kind: GeneratedNoiseKind::White, equation: 187, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(12), name: "fps3", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(13), name: "fps4", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_FP1_DI_RFP1", label: Some("rfp1"), kind: GeneratedNoiseKind::White, equation: 188, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(14), name: "fp1", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(5), name: "di", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_FP2_FP1_RFP2", label: Some("rfp2"), kind: GeneratedNoiseKind::White, equation: 189, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(15), name: "fp2", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(14), name: "fp1", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_FP3_FP2_RFP3", label: Some("rfp3"), kind: GeneratedNoiseKind::White, equation: 190, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(16), name: "fp3", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(15), name: "fp2", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_FP4_FP3_RFP4", label: Some("rfp4"), kind: GeneratedNoiseKind::White, equation: 191, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(17), name: "fp4", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(16), name: "fp3", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_SRC_S_RCS", label: Some("rcs"), kind: GeneratedNoiseKind::White, equation: 192, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(19), name: "src", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(2), name: "s", is_internal: false }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_D_DRC_RCD", label: Some("rcd"), kind: GeneratedNoiseKind::White, equation: 193, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(0), name: "d", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(18), name: "drc", is_internal: true }, table_len: 0, table_log_interp: false },
];

impl Instance {
    pub fn evaluate_noise_sources(&self, ctx: &GeneratedEvalContext<'_>, visitor: &mut dyn GeneratedNoiseVisitor) -> Result<(), GeneratedNoiseEvaluationError> {
        if !self.multiplicity.is_finite() || self.multiplicity <= 0.0 {
            return Err(GeneratedNoiseEvaluationError::InvalidMultiplicity { value: self.multiplicity });
        }
        let parameters = &self.params.values;
        let temperature = ctx.temperature();
        let node_potentials = [ctx.node_voltage(self.nodes[0]), ctx.node_voltage(self.nodes[1]), ctx.node_voltage(self.nodes[2]), ctx.node_voltage(self.nodes[3]), ctx.node_voltage(self.nodes[4]), ctx.node_voltage(self.nodes[5]), ctx.node_voltage(self.nodes[6]), ctx.node_voltage(self.nodes[7]), ctx.node_voltage(self.nodes[8]), ctx.node_voltage(self.nodes[9]), ctx.node_voltage(self.nodes[10]), ctx.node_voltage(self.nodes[11]), ctx.node_voltage(self.nodes[12]), ctx.node_voltage(self.nodes[13]), ctx.node_voltage(self.nodes[14]), ctx.node_voltage(self.nodes[15]), ctx.node_voltage(self.nodes[16]), ctx.node_voltage(self.nodes[17]), ctx.node_voltage(self.nodes[18]), ctx.node_voltage(self.nodes[19]), ctx.node_voltage(self.nodes[20]), ctx.node_voltage(self.nodes[21]), ctx.node_voltage(self.nodes[22]), ctx.node_voltage(self.nodes[23]), ctx.node_voltage(self.nodes[24]), ctx.node_voltage(self.nodes[25]), ctx.node_voltage(self.nodes[26]), ctx.node_voltage(self.nodes[27]), ctx.node_voltage(self.nodes[28]), ctx.node_voltage(self.nodes[29])];
            let A = 0e0f64;
            let C = 1e0f64;
            let D = 0.0f64;
            let G = 3.1499999999999773e0f64;
            let I = 1.77315e3f64;
            let K = parameters[30];
            let L = parameters[0];
            let M = parameters[2];
            let O = parameters[31];
            let Q = parameters[29];
            let R = parameters[54];
            let T = parameters[66];
            let W = parameters[353];
            let Y = parameters[48];
            let AC = parameters[49];
            let AE = 1e-1f64;
            let AN = parameters[325];
            let AP = parameters[327];
            let AS = 1.38062e-23f64;
            let AT = 1.60219e-19f64;
            let AZ = 3e0f64;
            let BB = 1e-2f64;
            let BI = parameters[7];
            let BJ = parameters[8];
            let DK = parameters[6];
            let DL = node_potentials[5];
            let DM = node_potentials[9];
            let DO = node_potentials[8];
            let DR = node_potentials[19];
            let DS = node_potentials[0];
            let DU = node_potentials[2];
            let DZ = parameters[53];
            let EA = 5e-1f64;
            let EE = 1e-3f64;
            let EH = parameters[55];
            let EI = parameters[56];
            let EJ = parameters[33];
            let EK = node_potentials[13];
            let EP = parameters[328];
            let ES = 5e1f64;
            let EX = 2e0f64;
            let EZ = node_potentials[23];
            let FE = parameters[338];
            let FG = node_potentials[26];
            let FL = parameters[337];
            let FO = node_potentials[17];
            let FZ = parameters[67];
            let GB = parameters[68];
            let GC = node_potentials[18];
            let GI = node_potentials[7];
            let GJ = node_potentials[10];
            let GP = node_potentials[3];
            let GS = node_potentials[11];
            let HA = node_potentials[12];
            let HT = node_potentials[14];
            let IB = node_potentials[15];
            let IJ = node_potentials[16];
            let IT = parameters[233];
            let IU = parameters[354];
            let IY = parameters[245];
            let IZ = parameters[246];
            let JA = parameters[39];
            let JB = parameters[47];
            let JC = parameters[45];
            let JD = parameters[42];
            let JJ = 2.302585092994046e0f64;
            let JU = parameters[51];
            let OQ = 1e-38f64;
            let OR = 1e-57f64;
            let OS = 4e0f64;
            let PF = parameters[211];
            let PJ = parameters[223];
            let PK = parameters[224];
            let VH = parameters[189];
            let VL = parameters[201];
            let VM = parameters[202];
            let ABJ = parameters[167];
            let ABN = parameters[179];
            let ABO = parameters[180];
            let AHL = parameters[79];
            let AHP = parameters[91];
            let AHQ = parameters[92];
            let ANN = parameters[101];
            let ANR = parameters[113];
            let ANS = parameters[114];
            let ATP = parameters[123];
            let ATT = parameters[135];
            let ATU = parameters[136];
            let AZR = parameters[145];
            let AZV = parameters[157];
            let AZW = parameters[158];
            let BFU = parameters[58];
            let BFV = parameters[59];
            let BFW = parameters[46];
            let BLH = 0.0f64;
            let BLM = 0.0f64;
            let BLR = parameters[70];
            let BLS = parameters[71];
            let BRD = 0.0f64;
            let BRI = 0.0f64;
            let BRM = parameters[1];
            let BRN = parameters[38];
            let BRO = parameters[40];
            let BRP = parameters[41];
            let BRQ = parameters[32];
            let BRR = parameters[34];
            let BRS = parameters[44];
            let BRT = parameters[43];
            let CBN = 0.0f64;
            let CBS = 0.0f64;
            let CCA = parameters[260];
            let CCB = parameters[262];
            let CCC = parameters[261];
            let CCD = parameters[258];
            let CCE = parameters[278];
            let CCF = parameters[277];
            let CCG = parameters[255];
            let CCI = parameters[259];
            let CCK = parameters[276];
            let CCL = parameters[270];
            let CCM = parameters[271];
            let CCN = parameters[269];
            let CCP = parameters[268];
            let CCQ = parameters[256];
            let CCV = 1.9287498479639178e-22f64;
            let CDF = 1.9287498479639178e-22f64;
            let CDM = 1.9287498479639178e-22f64;
            let CDX = 1.9287498479639178e-22f64;
            let CEH = 1.9287498479639178e-22f64;
            let CEQ = 1.9287498479639178e-22f64;
            let CFE = 1.9287498479639178e-22f64;
            let CFN = 1.9287498479639178e-22f64;
            let CGO = 1.9287498479639178e-22f64;
            let CGV = parameters[265];
            let CGW = parameters[267];
            let CGX = parameters[266];
            let CGY = parameters[263];
            let CGZ = parameters[281];
            let CHA = parameters[280];
            let CHB = parameters[264];
            let CHD = parameters[279];
            let CHE = parameters[274];
            let CHF = parameters[275];
            let CHG = parameters[273];
            let CHI = parameters[272];
            let CHL = 1.9287498479639178e-22f64;
            let CHV = 1.9287498479639178e-22f64;
            let CIC = 1.9287498479639178e-22f64;
            let CIN = 1.9287498479639178e-22f64;
            let CIX = 1.9287498479639178e-22f64;
            let CJG = 1.9287498479639178e-22f64;
            let CJU = 1.9287498479639178e-22f64;
            let CKD = 1.9287498479639178e-22f64;
            let CLD = 1.9287498479639178e-22f64;
            let CLK = parameters[285];
            let CLL = parameters[286];
            let CLM = parameters[283];
            let CLR = 1.0f64;
            let CLY = 1.0f64;
            let CMM = parameters[289];
            let CMN = parameters[290];
            let CMO = parameters[287];
            let CMT = 1.0f64;
            let CNA = 1.0f64;
            let CNT = 1.9287498479639178e-22f64;
            let COC = 1.9287498479639178e-22f64;
            let COI = 1.9287498479639178e-22f64;
            let COS = 1.9287498479639178e-22f64;
            let CPB = 1.9287498479639178e-22f64;
            let CPK = 1.9287498479639178e-22f64;
            let CPY = 1.9287498479639178e-22f64;
            let CQH = 1.9287498479639178e-22f64;
            let CRF = 1.9287498479639178e-22f64;
            let CRQ = 1.9287498479639178e-22f64;
            let CRZ = 1.9287498479639178e-22f64;
            let CSF = 1.9287498479639178e-22f64;
            let CSP = 1.9287498479639178e-22f64;
            let CSY = 1.9287498479639178e-22f64;
            let CTH = 1.9287498479639178e-22f64;
            let CTV = 1.9287498479639178e-22f64;
            let CUE = 1.9287498479639178e-22f64;
            let CVC = 1.9287498479639178e-22f64;
            let CVM = 1.0f64;
            let CVT = 1.0f64;
            let CWL = 1.0f64;
            let CWS = 1.0f64;
            let CXI = parameters[294];
            let CXJ = parameters[296];
            let CXK = parameters[295];
            let CXL = parameters[292];
            let CXM = 6e2f64;
            let CXN = parameters[300];
            let CZF = parameters[305];
            let CZS = 1.0f64;
            let CZZ = 1.0f64;
            let DAT = parameters[309];
            let DBH = parameters[317];
            let DBI = parameters[316];
            let DCY = parameters[319];
            let DCZ = parameters[318];
            let DEQ = node_potentials[6];
            let DES = parameters[27];
            let DET = parameters[28];
            let DGP = parameters[352];
            let DJM = 1e0f64;
            let DJN = 1e0f64;
            let DJO = 1e0f64;
            let DJP = 1e0f64;
            let DJQ = 1e0f64;
            let DJR = 1e0f64;
            let DJS = 1e0f64;
            let DJT = 1e0f64;
            let DJU = 1e0f64;
            let DKT = 0e0f64;
            let DLA = 2e0f64;
            let DLB = -1e0f64;
            let DLC = Lanes([0e0f64; 4]);
            let DLD = 0e0f64;
            let DLR = Lanes([0e0f64; 2]);
            let DMF = Lanes([0e0f64; 4]);
            let B = parameters[5] + 2.7315e2f64;
            if D != 0.0 {
            } else {
            }
            let E = (temperature + parameters[3]) + node_potentials[4];
            let F = if E < 3.1499999999999773e0f64 { 1.0 } else { 0.0 };
            let Z;
            let DJV;
            if F != 0.0 {
                Z = G;
                DJV = DKT;
            } else {
                let H = if E > 1.77315e3f64 { 1.0 } else { 0.0 };
                let AA;
                let DJW;
                if H != 0.0 {
                    AA = I;
                    DJW = DKT;
                } else {
                    AA = E;
                    DJW = DJN;
                }
                Z = AA;
                DJV = DJW;
            }
            let J = if parameters[50] == A { 1.0 } else { 0.0 };
            let V;
            let AH;
            if J != 0.0 {
                let N = (K / L) / M;
                let P = (O / L) / M;
                V = N;
                AH = P;
            } else {
                let S = ((K / L) + ((Q * R) / L)) / M;
                let U = ((O / L) + ((Q * T) / L)) / M;
                V = S;
                AH = U;
            }
            let X = if (if V >= W { 1.0 } else { 0.0 }) != 0.0 && (if V > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let DEN;
            if X != 0.0 {
                let AB = Z - B;
                let AD = V * ((C + (Y * AB)) + ((AC * AB) * AB));
                let AF = AE * V;
                let AG = if AD < AF { 1.0 } else { 0.0 };
                let DEO = if AG != 0.0 {
                    AF
                } else {
                    AD
                };
                DEN = DEO;
            } else {
                DEN = A;
            }
            let AI = if (if AH >= W { 1.0 } else { 0.0 }) != 0.0 && (if AH > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let DEL;
            if AI != 0.0 {
                let AJ = Z - B;
                let AK = AH * ((C + (Y * AJ)) + ((AC * AJ) * AJ));
                let AL = AE * AH;
                let AM = if AK < AL { 1.0 } else { 0.0 };
                let DEM = if AM != 0.0 {
                    AL
                } else {
                    AK
                };
                DEL = DEM;
            } else {
                DEL = A;
            }
            let AO = (parameters[324] / M) / AN;
            let AQ = AO * (parameters[326] + ((AP * L) / AN));
            let AR = AO * (((C - AP) * L) / AN);
            let AU = (AS * Z) / AT;
            let DKU = (DJV * AS) / AT;
            let AV = Z - B;
            let AW = C + (parameters[336] * AV);
            let AX = if AW < AE { 1.0 } else { 0.0 };
            let EV = if AX != 0.0 {
                AE
            } else {
                AW
            };
            let AY = Z / B;
            let DKV = DJV / B;
            let BA = (AY * AY) * AY;
            let BC = if (C + (parameters[21] * AV)) < BB { 1.0 } else { 0.0 };
            if BC != 0.0 {
            } else {
            }
            let BD = if (C + (parameters[22] * AV)) < BB { 1.0 } else { 0.0 };
            if BD != 0.0 {
            } else {
            }
            let BE = if (C + (parameters[23] * AV)) < BB { 1.0 } else { 0.0 };
            if BE != 0.0 {
            } else {
            }
            let BF = if (C + (parameters[24] * AV)) < BB { 1.0 } else { 0.0 };
            if BF != 0.0 {
            } else {
            }
            let BG = if (C + (parameters[25] * AV)) < BB { 1.0 } else { 0.0 };
            if BG != 0.0 {
            } else {
            }
            let BH = if (C + (parameters[26] * AV)) < BB { 1.0 } else { 0.0 };
            if BH != 0.0 {
            } else {
            }
            if BC != 0.0 {
            } else {
            }
            if BD != 0.0 {
            } else {
            }
            if BE != 0.0 {
            } else {
            }
            if BF != 0.0 {
            } else {
            }
            if BG != 0.0 {
            } else {
            }
            if BH != 0.0 {
            } else {
            }
            let DKW = DJV * BJ;
            let BK = C + (BJ * AV);
            let BL = if BK < BB { 1.0 } else { 0.0 };
            let BM;
            let DJX;
            if BL != 0.0 {
                BM = BB;
                DJX = DKT;
            } else {
                BM = BK;
                DJX = DKW;
            }
            let BN = BI * BM;
            let DKX = DJX * BI;
            let BO = C + (parameters[82] * AV);
            let BP = if BO < BB { 1.0 } else { 0.0 };
            let BQ = if BP != 0.0 {
                BB
            } else {
                BO
            };
            let BR = parameters[81] * BQ;
            let BS = C + (parameters[104] * AV);
            let BT = if BS < BB { 1.0 } else { 0.0 };
            let BU = if BT != 0.0 {
                BB
            } else {
                BS
            };
            let BV = parameters[103] * BU;
            let BW = C + (parameters[126] * AV);
            let BX = if BW < BB { 1.0 } else { 0.0 };
            let BY = if BX != 0.0 {
                BB
            } else {
                BW
            };
            let BZ = parameters[125] * BY;
            let CA = C + (parameters[148] * AV);
            let CB = if CA < BB { 1.0 } else { 0.0 };
            let CC = if CB != 0.0 {
                BB
            } else {
                CA
            };
            let CD = parameters[147] * CC;
            let CE = if (C + (parameters[87] * AV)) < BB { 1.0 } else { 0.0 };
            if CE != 0.0 {
            } else {
            }
            let CF = if (C + (parameters[109] * AV)) < BB { 1.0 } else { 0.0 };
            if CF != 0.0 {
            } else {
            }
            let CG = if (C + (parameters[131] * AV)) < BB { 1.0 } else { 0.0 };
            if CG != 0.0 {
            } else {
            }
            let CH = if (C + (parameters[153] * AV)) < BB { 1.0 } else { 0.0 };
            if CH != 0.0 {
            } else {
            }
            let CI = if (C + (parameters[89] * AV)) < BB { 1.0 } else { 0.0 };
            if CI != 0.0 {
            } else {
            }
            let CJ = if (C + (parameters[111] * AV)) < BB { 1.0 } else { 0.0 };
            if CJ != 0.0 {
            } else {
            }
            let CK = if (C + (parameters[133] * AV)) < BB { 1.0 } else { 0.0 };
            if CK != 0.0 {
            } else {
            }
            let CL = if (C + (parameters[155] * AV)) < BB { 1.0 } else { 0.0 };
            if CL != 0.0 {
            } else {
            }
            let CM = C + (parameters[170] * AV);
            let CN = if CM < BB { 1.0 } else { 0.0 };
            let CO = if CN != 0.0 {
                BB
            } else {
                CM
            };
            let CP = parameters[169] * CO;
            let CQ = C + (parameters[192] * AV);
            let CR = if CQ < BB { 1.0 } else { 0.0 };
            let CS = if CR != 0.0 {
                BB
            } else {
                CQ
            };
            let CT = parameters[191] * CS;
            let CU = C + (parameters[214] * AV);
            let CV = if CU < BB { 1.0 } else { 0.0 };
            let CW = if CV != 0.0 {
                BB
            } else {
                CU
            };
            let CX = parameters[213] * CW;
            let CY = C + (parameters[236] * AV);
            let CZ = if CY < BB { 1.0 } else { 0.0 };
            let DA = if CZ != 0.0 {
                BB
            } else {
                CY
            };
            let DB = parameters[235] * DA;
            let DC = if (C + (parameters[175] * AV)) < BB { 1.0 } else { 0.0 };
            if DC != 0.0 {
            } else {
            }
            let DD = if (C + (parameters[197] * AV)) < BB { 1.0 } else { 0.0 };
            if DD != 0.0 {
            } else {
            }
            let DE = if (C + (parameters[219] * AV)) < BB { 1.0 } else { 0.0 };
            if DE != 0.0 {
            } else {
            }
            let DF = if (C + (parameters[241] * AV)) < BB { 1.0 } else { 0.0 };
            if DF != 0.0 {
            } else {
            }
            let DG = if (C + (parameters[177] * AV)) < BB { 1.0 } else { 0.0 };
            if DG != 0.0 {
            } else {
            }
            let DH = if (C + (parameters[199] * AV)) < BB { 1.0 } else { 0.0 };
            if DH != 0.0 {
            } else {
            }
            let DI = if (C + (parameters[221] * AV)) < BB { 1.0 } else { 0.0 };
            if DI != 0.0 {
            } else {
            }
            let DJ = if (C + (parameters[243] * AV)) < BB { 1.0 } else { 0.0 };
            if DJ != 0.0 {
            } else {
            }
            let DN = DK * (DL - DM);
            let DKY = (Lanes([DJO, 0.0]) - Lanes([0.0, DJP])) * DK;
            let DP = DK * (DO - DM);
            let DKZ = (Lanes([DJQ, 0.0]) - Lanes([0.0, DJP])) * DK;
            let DQ = if parameters[52] == A { 1.0 } else { 0.0 };
            let EM;
            if DQ != 0.0 {
                let DT = DK * (DR - DS);
                let DV = DK * (DR - DU);
                let DW = if DT <= DV { 1.0 } else { 0.0 };
                let EN = if DW != 0.0 {
                    DV
                } else {
                    DT
                };
                EM = EN;
            } else {
                let DX = DK * (DR - DS);
                let DY = DK * (DR - DU);
                let EG = if DQ != 0.0 {
                    let EB = DX - DY;
                    let EC = EA * ((DX + DY) + (((EB * EB) + DZ).sqrt()));
                    EC
                } else {
                    let ED = DX - DY;
                    let EF = EA * ((DX + DY) + (ED * (((EE / DZ) * ED).tanh())));
                    EF
                };
                EM = EG;
            }
            let EL = DK * (EK - DR);
            let EO = (EH + (C / ((Q * EI) * EJ))) - EM;
            let EQ = if EP == C { 1.0 } else { 0.0 };
            let GA;
            let BRU;
            let DJY;
            if EQ != 0.0 {
                let ER = (((DS - node_potentials[1]) - parameters[331]) - (node_potentials[21] * parameters[335])) / parameters[334];
                let ET = if ER > ES { 1.0 } else { 0.0 };
                if ET != 0.0 {
                } else {
                    let EU = if ER < -5e1f64 { 1.0 } else { 0.0 };
                    if EU != 0.0 {
                    } else {
                    }
                }
                let EW = C + (node_potentials[20] * EV);
                GA = EW;
                BRU = C;
                DJY = DLC;
            } else {
                let EY = if EP == EX { 1.0 } else { 0.0 };
                let BRV;
                let DJZ;
                if EY != 0.0 {
                    let FA = (node_potentials[24] - EZ) / AU;
                    let FB = if FA > ES { 1.0 } else { 0.0 };
                    if FB != 0.0 {
                    } else {
                        let FC = if FA < -5e1f64 { 1.0 } else { 0.0 };
                        if FC != 0.0 {
                        } else {
                        }
                    }
                    let FD = EZ - node_potentials[22];
                    let FF = (FD.abs()) / FE;
                    let DLE = ((Lanes([0.0, DJS]) - Lanes([DJR, 0.0])) * ((DLA * (if FD >= DLD { 1.0 } else { 0.0 })) - DJM)) / FE;
                    let FH = (FG - node_potentials[27]) / AU;
                    let FI = if FH > ES { 1.0 } else { 0.0 };
                    if FI != 0.0 {
                    } else {
                        let FJ = if FH < -5e1f64 { 1.0 } else { 0.0 };
                        if FJ != 0.0 {
                        } else {
                        }
                    }
                    let FK = FG - node_potentials[25];
                    let DLF = ((Lanes([0.0, DJU]) - Lanes([DJT, 0.0])) * ((DLA * (if FK >= DLD { 1.0 } else { 0.0 })) - DJM)) / FL;
                    let FM = (C + FF) + ((FK.abs()) / FL);
                    let FN = C / FM;
                    let DLG = (((Lanes([DLE[0], DLE[1], 0.0, 0.0]) + Lanes([0.0, 0.0, DLF[0], DLF[1]])) * FN) * DLB) / FM;
                    BRV = FN;
                    DJZ = DLG;
                } else {
                    BRV = C;
                    DJZ = DLC;
                }
                GA = C;
                BRU = BRV;
                DJY = DJZ;
            }
            let GE;
            if DQ != 0.0 {
                let FP = DK * (FO - DS);
                let FQ = DK * (FO - DU);
                let FR = if FP <= FQ { 1.0 } else { 0.0 };
                let GF = if FR != 0.0 {
                    FQ
                } else {
                    FP
                };
                GE = GF;
            } else {
                let FS = DK * (FO - DS);
                let FT = DK * (FO - DU);
                let FY = if DQ != 0.0 {
                    let FU = FS - FT;
                    let FV = EA * ((FS + FT) + (((FU * FU) + DZ).sqrt()));
                    FV
                } else {
                    let FW = FS - FT;
                    let FX = EA * ((FS + FT) + (FW * (((EE / DZ) * FW).tanh())));
                    FX
                };
                GE = FY;
            }
            let GD = DK * (GC - FO);
            let GG = (FZ + (C / (((GA * Q) * GB) * EJ))) - GE;
            let GH = if parameters[78] == C { 1.0 } else { 0.0 };
            let AHN;
            let AHO;
            if GH != 0.0 {
                let GK = DK * (GI - GJ);
                let GL = DK * (DU - GJ);
                AHN = GK;
                AHO = GL;
            } else {
                let GM = DK * (DU - GJ);
                let GN = DK * (GI - GJ);
                AHN = GM;
                AHO = GN;
            }
            let GO = DK * (DM - GJ);
            let GQ = DK * (GP - GJ);
            let GR = if parameters[100] == C { 1.0 } else { 0.0 };
            let ANP;
            let ANQ;
            if GR != 0.0 {
                let GT = DK * (GI - GS);
                let GU = DK * (DU - GS);
                ANP = GT;
                ANQ = GU;
            } else {
                let GV = DK * (DU - GS);
                let GW = DK * (GI - GS);
                ANP = GV;
                ANQ = GW;
            }
            let GX = DK * (GJ - GS);
            let GY = DK * (GP - GS);
            let GZ = if parameters[122] == C { 1.0 } else { 0.0 };
            let ATR;
            let ATS;
            if GZ != 0.0 {
                let HB = DK * (GI - HA);
                let HC = DK * (DU - HA);
                ATR = HB;
                ATS = HC;
            } else {
                let HD = DK * (DU - HA);
                let HE = DK * (GI - HA);
                ATR = HD;
                ATS = HE;
            }
            let HF = DK * (GS - HA);
            let HG = DK * (GP - HA);
            let HH = if parameters[144] == C { 1.0 } else { 0.0 };
            let AZT;
            let AZU;
            if HH != 0.0 {
                let HI = DK * (GI - EK);
                let HJ = DK * (DU - EK);
                AZT = HI;
                AZU = HJ;
            } else {
                let HK = DK * (DU - EK);
                let HL = DK * (GI - EK);
                AZT = HK;
                AZU = HL;
            }
            let HM = DK * (HA - EK);
            let HN = DK * (GP - EK);
            let HO = if parameters[166] == C { 1.0 } else { 0.0 };
            let ABL;
            let ABM;
            if HO != 0.0 {
                let HP = DK * (GI - DL);
                let HQ = DK * (DU - DL);
                ABL = HP;
                ABM = HQ;
            } else {
                let HR = DK * (DU - DL);
                let HS = DK * (GI - DL);
                ABL = HR;
                ABM = HS;
            }
            let HU = DK * (HT - DL);
            let HV = DK * (GP - DL);
            let HW = if parameters[188] == C { 1.0 } else { 0.0 };
            let VJ;
            let VK;
            if HW != 0.0 {
                let HX = DK * (GI - HT);
                let HY = DK * (DU - HT);
                VJ = HX;
                VK = HY;
            } else {
                let HZ = DK * (DU - HT);
                let IA = DK * (GI - HT);
                VJ = HZ;
                VK = IA;
            }
            let IC = DK * (IB - HT);
            let ID = DK * (GP - HT);
            let IE = if parameters[210] == C { 1.0 } else { 0.0 };
            let PH;
            let PI;
            if IE != 0.0 {
                let IF = DK * (GI - IB);
                let IG = DK * (DU - IB);
                PH = IF;
                PI = IG;
            } else {
                let IH = DK * (DU - IB);
                let II = DK * (GI - IB);
                PH = IH;
                PI = II;
            }
            let IK = DK * (IJ - IB);
            let IL = DK * (GP - IB);
            let IM = if parameters[232] == C { 1.0 } else { 0.0 };
            let IW;
            let IX;
            if IM != 0.0 {
                let IN = DK * (GI - IJ);
                let IO = DK * (DU - IJ);
                IW = IN;
                IX = IO;
            } else {
                let IP = DK * (DU - IJ);
                let IQ = DK * (GI - IJ);
                IW = IP;
                IX = IQ;
            }
            let IR = DK * (FO - IJ);
            let IS = DK * (GP - IJ);
            let IV = if IT > IU { 1.0 } else { 0.0 };
            if IV != 0.0 {
                let JG = if DQ != 0.0 {
                    let JE = ((IR * IR) + DZ).sqrt();
                    JE
                } else {
                    let JF = IR * (((EE / DZ) * IR).tanh());
                    JF
                };
                let JH = IW - IR;
                let JI = parameters[253] * AU;
                let JK = parameters[248] / (JJ * AU);
                let JL = JK + (parameters[249] * JG);
                let JM = parameters[234] + (parameters[250] * AV);
                let JN = AY.powf(JB);
                let JO = if JA != A { 1.0 } else { 0.0 };
                let JQ = if JO != 0.0 {
                    let JP = JG / ((C + ((JG / JA).powf(IZ))).powf((C / IZ)));
                    JP
                } else {
                    A
                };
                let JR = JM - ((parameters[247] - (JQ * A)) * JG);
                let JS = (EX * JL) * AU;
                let JT = DB * JS;
                let JV = (JU * JI) / EX;
                let JW = JR - JV;
                let KB = if DQ != 0.0 {
                    let JX = IW - JH;
                    let JY = EA * ((IW + JH) + (((JX * JX) + DZ).sqrt()));
                    JY
                } else {
                    let JZ = IW - JH;
                    let KA = EA * ((IW + JH) + (JZ * (((EE / DZ) * JZ).tanh())));
                    KA
                };
                let KC = (KB - JW) / JI;
                let KD = if KC > ES { 1.0 } else { 0.0 };
                let KM;
                if KD != 0.0 {
                    KM = A;
                } else {
                    let KE = if KC < -5e1f64 { 1.0 } else { 0.0 };
                    let KN = if KE != 0.0 {
                        C
                    } else {
                        let KF = C / (C + (KC.exp()));
                        KF
                    };
                    KM = KN;
                }
                let KK = if DQ != 0.0 {
                    let KG = IW - JH;
                    let KH = EA * ((IW + JH) + (((KG * KG) + DZ).sqrt()));
                    KH
                } else {
                    let KI = IW - JH;
                    let KJ = EA * ((IW + JH) + (KI * (((EE / DZ) * KI).tanh())));
                    KJ
                };
                let KL = (JU * AE) * JI;
                let KO = (KK - (JR - (KL * KM))) / JS;
                let KP = if KO > ES { 1.0 } else { 0.0 };
                let KU;
                if KP != 0.0 {
                    let KQ = JT * KO;
                    KU = KQ;
                } else {
                    let KR = if KO < -5e1f64 { 1.0 } else { 0.0 };
                    let KV = if KR != 0.0 {
                        let KS = JT * (KO.exp());
                        KS
                    } else {
                        let KT = JT * ((C + (KO.exp())).ln());
                        KT
                    };
                    KU = KV;
                }
                let KW = parameters[244] * ((C + (JC * B)) / (C + (JC * Z)));
                let KX = (((KW * (C + ((JD * JG) / IT))) / (C + ((parameters[251] * KU) / DB))) * IT) / (IY / (JN * (C + ((parameters[252] * KU) / DB))));
                let KY = (((KX * ((C + (((EX * KU) / DB) / KX)).sqrt())) - KX) * (C - KM)) + (JS * KM);
                let KZ = IR / KY;
                let LE = if DQ != 0.0 {
                    let LA = A - KZ;
                    let LB = EA * (KZ + (((LA * LA) + DZ).sqrt()));
                    LB
                } else {
                    let LC = A - KZ;
                    let LD = EA * (KZ + (LC * (((EE / DZ) * LC).tanh())));
                    LD
                };
                let LF = C / IZ;
                let LG = IR * (C / ((C + (LE.powf(IZ))).powf(LF)));
                let LH = -IR;
                let LI = LH / KY;
                let LN = if DQ != 0.0 {
                    let LJ = A - LI;
                    let LK = EA * (LI + (((LJ * LJ) + DZ).sqrt()));
                    LK
                } else {
                    let LL = A - LI;
                    let LM = EA * (LI + (LL * (((EE / DZ) * LL).tanh())));
                    LM
                };
                let LO = LH * (C / ((C + (LN.powf(IZ))).powf(LF)));
                let LP = (IW - JW) / JI;
                let LQ = if LP > ES { 1.0 } else { 0.0 };
                let LT;
                if LQ != 0.0 {
                    LT = A;
                } else {
                    let LR = if LP < -5e1f64 { 1.0 } else { 0.0 };
                    let LU = if LR != 0.0 {
                        C
                    } else {
                        let LS = C / (C + (LP.exp()));
                        LS
                    };
                    LT = LU;
                }
                let LV = ((JH - LO) - (JR - (KL * LT))) / JS;
                let LW = if LV > ES { 1.0 } else { 0.0 };
                if LW != 0.0 {
                } else {
                    let LX = if LV < -5e1f64 { 1.0 } else { 0.0 };
                    if LX != 0.0 {
                    } else {
                    }
                }
                let LY = (JH - JW) / JI;
                let LZ = if LY > ES { 1.0 } else { 0.0 };
                let MC;
                if LZ != 0.0 {
                    MC = A;
                } else {
                    let MA = if LY < -5e1f64 { 1.0 } else { 0.0 };
                    let MD = if MA != 0.0 {
                        C
                    } else {
                        let MB = C / (C + (LY.exp()));
                        MB
                    };
                    MC = MD;
                }
                let ME = ((IW - LG) - (JR - (KL * MC))) / JS;
                let MF = if ME > ES { 1.0 } else { 0.0 };
                if MF != 0.0 {
                } else {
                    let MG = if ME < -5e1f64 { 1.0 } else { 0.0 };
                    if MG != 0.0 {
                    } else {
                    }
                }
                if DQ != 0.0 {
                } else {
                }
                let MH = (EX * JK) * AU;
                let MI = DB * MH;
                let MJ = JM - JV;
                let MO = if DQ != 0.0 {
                    let MK = IW - JH;
                    let ML = EA * ((IW + JH) + (((MK * MK) + DZ).sqrt()));
                    ML
                } else {
                    let MM = IW - JH;
                    let MN = EA * ((IW + JH) + (MM * (((EE / DZ) * MM).tanh())));
                    MN
                };
                let MP = (MO - MJ) / JI;
                let MQ = if MP > ES { 1.0 } else { 0.0 };
                let MY;
                if MQ != 0.0 {
                    MY = A;
                } else {
                    let MR = if MP < -5e1f64 { 1.0 } else { 0.0 };
                    let MZ = if MR != 0.0 {
                        C
                    } else {
                        let MS = C / (C + (MP.exp()));
                        MS
                    };
                    MY = MZ;
                }
                let MX = if DQ != 0.0 {
                    let MT = IW - JH;
                    let MU = EA * ((IW + JH) + (((MT * MT) + DZ).sqrt()));
                    MU
                } else {
                    let MV = IW - JH;
                    let MW = EA * ((IW + JH) + (MV * (((EE / DZ) * MV).tanh())));
                    MW
                };
                let NA = (MX - (JM - (KL * MY))) / MH;
                let NB = if NA > ES { 1.0 } else { 0.0 };
                let NH;
                if NB != 0.0 {
                    let NC = MI * NA;
                    NH = NC;
                } else {
                    let ND = if NA < -5e1f64 { 1.0 } else { 0.0 };
                    let NI = if ND != 0.0 {
                        let NE = MI * (NA.exp());
                        NE
                    } else {
                        let NF = MI * ((C + (NA.exp())).ln());
                        NF
                    };
                    NH = NI;
                }
                let NG = (KW * IT) / (IY / JN);
                let NJ = (((NG * ((C + (((EX * NH) / DB) / NG)).sqrt())) - NG) * (C - MY)) + (MH * MY);
                let NK = IR / NJ;
                let NP = if DQ != 0.0 {
                    let NL = A - NK;
                    let NM = EA * (NK + (((NL * NL) + DZ).sqrt()));
                    NM
                } else {
                    let NN = A - NK;
                    let NO = EA * (NK + (NN * (((EE / DZ) * NN).tanh())));
                    NO
                };
                let NQ = IR * (C / ((C + (NP.powf(IZ))).powf(LF)));
                let NR = LH / NJ;
                let NW = if DQ != 0.0 {
                    let NS = A - NR;
                    let NT = EA * (NR + (((NS * NS) + DZ).sqrt()));
                    NT
                } else {
                    let NU = A - NR;
                    let NV = EA * (NR + (NU * (((EE / DZ) * NU).tanh())));
                    NV
                };
                let NX = LH * (C / ((C + (NW.powf(IZ))).powf(LF)));
                let NY = (IW - MJ) / JI;
                let NZ = if NY > ES { 1.0 } else { 0.0 };
                let OC;
                if NZ != 0.0 {
                    OC = A;
                } else {
                    let OA = if NY < -5e1f64 { 1.0 } else { 0.0 };
                    let OD = if OA != 0.0 {
                        C
                    } else {
                        let OB = C / (C + (NY.exp()));
                        OB
                    };
                    OC = OD;
                }
                let OE = ((JH - NX) - (JM - (KL * OC))) / MH;
                let OF = if OE > ES { 1.0 } else { 0.0 };
                if OF != 0.0 {
                } else {
                    let OG = if OE < -5e1f64 { 1.0 } else { 0.0 };
                    if OG != 0.0 {
                    } else {
                    }
                }
                let OH = (JH - MJ) / JI;
                let OI = if OH > ES { 1.0 } else { 0.0 };
                let OL;
                if OI != 0.0 {
                    OL = A;
                } else {
                    let OJ = if OH < -5e1f64 { 1.0 } else { 0.0 };
                    let OM = if OJ != 0.0 {
                        C
                    } else {
                        let OK = C / (C + (OH.exp()));
                        OK
                    };
                    OL = OM;
                }
                let ON = ((IW - NQ) - (JM - (KL * OL))) / MH;
                let OO = if ON > ES { 1.0 } else { 0.0 };
                if OO != 0.0 {
                } else {
                    let OP = if ON < -5e1f64 { 1.0 } else { 0.0 };
                    if OP != 0.0 {
                    } else {
                    }
                }
                let OT = if parameters[239] == C { 1.0 } else { 0.0 };
                if OT != 0.0 {
                    let OU = JM - ((JU * EA) * JI);
                    let OV = (IX - OU) / MH;
                    let OW = if OV > ES { 1.0 } else { 0.0 };
                    if OW != 0.0 {
                    } else {
                        let OX = if OV < -5e1f64 { 1.0 } else { 0.0 };
                        if OX != 0.0 {
                        } else {
                        }
                    }
                    let OY = (IS - OU) / MH;
                    let OZ = if OY > ES { 1.0 } else { 0.0 };
                    if OZ != 0.0 {
                    } else {
                        let PA = if OY < -5e1f64 { 1.0 } else { 0.0 };
                        if PA != 0.0 {
                        } else {
                        }
                    }
                } else {
                }
                let PB = if parameters[237] == C { 1.0 } else { 0.0 };
                if PB != 0.0 {
                    let PC = (IW - (JM - ((JU * EA) * JI))) / MH;
                    let PD = if PC > ES { 1.0 } else { 0.0 };
                    if PD != 0.0 {
                    } else {
                        let PE = if PC < -5e1f64 { 1.0 } else { 0.0 };
                        if PE != 0.0 {
                        } else {
                        }
                    }
                } else {
                }
            } else {
            }
            if IM != 0.0 {
            } else {
            }
            let PG = if PF > IU { 1.0 } else { 0.0 };
            if PG != 0.0 {
                let PN = if DQ != 0.0 {
                    let PL = ((IK * IK) + DZ).sqrt();
                    PL
                } else {
                    let PM = IK * (((EE / DZ) * IK).tanh());
                    PM
                };
                let PO = PH - IK;
                let PP = parameters[231] * AU;
                let PQ = parameters[226] / (JJ * AU);
                let PR = PQ + (parameters[227] * PN);
                let PS = parameters[212] + (parameters[228] * AV);
                let PT = AY.powf(JB);
                let PU = if JA != A { 1.0 } else { 0.0 };
                let PW = if PU != 0.0 {
                    let PV = PN / ((C + ((PN / JA).powf(PK))).powf((C / PK)));
                    PV
                } else {
                    A
                };
                let PX = PS - ((parameters[225] - (PW * A)) * PN);
                let PY = (EX * PR) * AU;
                let PZ = CX * PY;
                let QA = (JU * PP) / EX;
                let QB = PX - QA;
                let QG = if DQ != 0.0 {
                    let QC = PH - PO;
                    let QD = EA * ((PH + PO) + (((QC * QC) + DZ).sqrt()));
                    QD
                } else {
                    let QE = PH - PO;
                    let QF = EA * ((PH + PO) + (QE * (((EE / DZ) * QE).tanh())));
                    QF
                };
                let QH = (QG - QB) / PP;
                let QI = if QH > ES { 1.0 } else { 0.0 };
                let QR;
                if QI != 0.0 {
                    QR = A;
                } else {
                    let QJ = if QH < -5e1f64 { 1.0 } else { 0.0 };
                    let QS = if QJ != 0.0 {
                        C
                    } else {
                        let QK = C / (C + (QH.exp()));
                        QK
                    };
                    QR = QS;
                }
                let QP = if DQ != 0.0 {
                    let QL = PH - PO;
                    let QM = EA * ((PH + PO) + (((QL * QL) + DZ).sqrt()));
                    QM
                } else {
                    let QN = PH - PO;
                    let QO = EA * ((PH + PO) + (QN * (((EE / DZ) * QN).tanh())));
                    QO
                };
                let QQ = (JU * AE) * PP;
                let QT = (QP - (PX - (QQ * QR))) / PY;
                let QU = if QT > ES { 1.0 } else { 0.0 };
                let QZ;
                if QU != 0.0 {
                    let QV = PZ * QT;
                    QZ = QV;
                } else {
                    let QW = if QT < -5e1f64 { 1.0 } else { 0.0 };
                    let RA = if QW != 0.0 {
                        let QX = PZ * (QT.exp());
                        QX
                    } else {
                        let QY = PZ * ((C + (QT.exp())).ln());
                        QY
                    };
                    QZ = RA;
                }
                let RB = parameters[222] * ((C + (JC * B)) / (C + (JC * Z)));
                let RC = (((RB * (C + ((JD * PN) / PF))) / (C + ((parameters[229] * QZ) / CX))) * PF) / (PJ / (PT * (C + ((parameters[230] * QZ) / CX))));
                let RD = (((RC * ((C + (((EX * QZ) / CX) / RC)).sqrt())) - RC) * (C - QR)) + (PY * QR);
                let RE = IK / RD;
                let RJ = if DQ != 0.0 {
                    let RF = A - RE;
                    let RG = EA * (RE + (((RF * RF) + DZ).sqrt()));
                    RG
                } else {
                    let RH = A - RE;
                    let RI = EA * (RE + (RH * (((EE / DZ) * RH).tanh())));
                    RI
                };
                let RK = C / PK;
                let RL = IK * (C / ((C + (RJ.powf(PK))).powf(RK)));
                let RM = -IK;
                let RN = RM / RD;
                let RS = if DQ != 0.0 {
                    let RO = A - RN;
                    let RP = EA * (RN + (((RO * RO) + DZ).sqrt()));
                    RP
                } else {
                    let RQ = A - RN;
                    let RR = EA * (RN + (RQ * (((EE / DZ) * RQ).tanh())));
                    RR
                };
                let RT = RM * (C / ((C + (RS.powf(PK))).powf(RK)));
                let RU = (PH - QB) / PP;
                let RV = if RU > ES { 1.0 } else { 0.0 };
                let RY;
                if RV != 0.0 {
                    RY = A;
                } else {
                    let RW = if RU < -5e1f64 { 1.0 } else { 0.0 };
                    let RZ = if RW != 0.0 {
                        C
                    } else {
                        let RX = C / (C + (RU.exp()));
                        RX
                    };
                    RY = RZ;
                }
                let SA = ((PO - RT) - (PX - (QQ * RY))) / PY;
                let SB = if SA > ES { 1.0 } else { 0.0 };
                if SB != 0.0 {
                } else {
                    let SC = if SA < -5e1f64 { 1.0 } else { 0.0 };
                    if SC != 0.0 {
                    } else {
                    }
                }
                let SD = (PO - QB) / PP;
                let SE = if SD > ES { 1.0 } else { 0.0 };
                let SH;
                if SE != 0.0 {
                    SH = A;
                } else {
                    let SF = if SD < -5e1f64 { 1.0 } else { 0.0 };
                    let SI = if SF != 0.0 {
                        C
                    } else {
                        let SG = C / (C + (SD.exp()));
                        SG
                    };
                    SH = SI;
                }
                let SJ = ((PH - RL) - (PX - (QQ * SH))) / PY;
                let SK = if SJ > ES { 1.0 } else { 0.0 };
                if SK != 0.0 {
                } else {
                    let SL = if SJ < -5e1f64 { 1.0 } else { 0.0 };
                    if SL != 0.0 {
                    } else {
                    }
                }
                if DQ != 0.0 {
                } else {
                }
                let SM = (EX * PQ) * AU;
                let SN = CX * SM;
                let SO = PS - QA;
                let ST = if DQ != 0.0 {
                    let SP = PH - PO;
                    let SQ = EA * ((PH + PO) + (((SP * SP) + DZ).sqrt()));
                    SQ
                } else {
                    let SR = PH - PO;
                    let SS = EA * ((PH + PO) + (SR * (((EE / DZ) * SR).tanh())));
                    SS
                };
                let SU = (ST - SO) / PP;
                let SV = if SU > ES { 1.0 } else { 0.0 };
                let TD;
                if SV != 0.0 {
                    TD = A;
                } else {
                    let SW = if SU < -5e1f64 { 1.0 } else { 0.0 };
                    let TE = if SW != 0.0 {
                        C
                    } else {
                        let SX = C / (C + (SU.exp()));
                        SX
                    };
                    TD = TE;
                }
                let TC = if DQ != 0.0 {
                    let SY = PH - PO;
                    let SZ = EA * ((PH + PO) + (((SY * SY) + DZ).sqrt()));
                    SZ
                } else {
                    let TA = PH - PO;
                    let TB = EA * ((PH + PO) + (TA * (((EE / DZ) * TA).tanh())));
                    TB
                };
                let TF = (TC - (PS - (QQ * TD))) / SM;
                let TG = if TF > ES { 1.0 } else { 0.0 };
                let TM;
                if TG != 0.0 {
                    let TH = SN * TF;
                    TM = TH;
                } else {
                    let TI = if TF < -5e1f64 { 1.0 } else { 0.0 };
                    let TN = if TI != 0.0 {
                        let TJ = SN * (TF.exp());
                        TJ
                    } else {
                        let TK = SN * ((C + (TF.exp())).ln());
                        TK
                    };
                    TM = TN;
                }
                let TL = (RB * PF) / (PJ / PT);
                let TO = (((TL * ((C + (((EX * TM) / CX) / TL)).sqrt())) - TL) * (C - TD)) + (SM * TD);
                let TP = IK / TO;
                let TU = if DQ != 0.0 {
                    let TQ = A - TP;
                    let TR = EA * (TP + (((TQ * TQ) + DZ).sqrt()));
                    TR
                } else {
                    let TS = A - TP;
                    let TT = EA * (TP + (TS * (((EE / DZ) * TS).tanh())));
                    TT
                };
                let TV = IK * (C / ((C + (TU.powf(PK))).powf(RK)));
                let TW = RM / TO;
                let UB = if DQ != 0.0 {
                    let TX = A - TW;
                    let TY = EA * (TW + (((TX * TX) + DZ).sqrt()));
                    TY
                } else {
                    let TZ = A - TW;
                    let UA = EA * (TW + (TZ * (((EE / DZ) * TZ).tanh())));
                    UA
                };
                let UC = RM * (C / ((C + (UB.powf(PK))).powf(RK)));
                let UD = (PH - SO) / PP;
                let UE = if UD > ES { 1.0 } else { 0.0 };
                let UH;
                if UE != 0.0 {
                    UH = A;
                } else {
                    let UF = if UD < -5e1f64 { 1.0 } else { 0.0 };
                    let UI = if UF != 0.0 {
                        C
                    } else {
                        let UG = C / (C + (UD.exp()));
                        UG
                    };
                    UH = UI;
                }
                let UJ = ((PO - UC) - (PS - (QQ * UH))) / SM;
                let UK = if UJ > ES { 1.0 } else { 0.0 };
                if UK != 0.0 {
                } else {
                    let UL = if UJ < -5e1f64 { 1.0 } else { 0.0 };
                    if UL != 0.0 {
                    } else {
                    }
                }
                let UM = (PO - SO) / PP;
                let UN = if UM > ES { 1.0 } else { 0.0 };
                let UQ;
                if UN != 0.0 {
                    UQ = A;
                } else {
                    let UO = if UM < -5e1f64 { 1.0 } else { 0.0 };
                    let UR = if UO != 0.0 {
                        C
                    } else {
                        let UP = C / (C + (UM.exp()));
                        UP
                    };
                    UQ = UR;
                }
                let US = ((PH - TV) - (PS - (QQ * UQ))) / SM;
                let UT = if US > ES { 1.0 } else { 0.0 };
                if UT != 0.0 {
                } else {
                    let UU = if US < -5e1f64 { 1.0 } else { 0.0 };
                    if UU != 0.0 {
                    } else {
                    }
                }
                let UV = if parameters[217] == C { 1.0 } else { 0.0 };
                if UV != 0.0 {
                    let UW = PS - ((JU * EA) * PP);
                    let UX = (PI - UW) / SM;
                    let UY = if UX > ES { 1.0 } else { 0.0 };
                    if UY != 0.0 {
                    } else {
                        let UZ = if UX < -5e1f64 { 1.0 } else { 0.0 };
                        if UZ != 0.0 {
                        } else {
                        }
                    }
                    let VA = (IL - UW) / SM;
                    let VB = if VA > ES { 1.0 } else { 0.0 };
                    if VB != 0.0 {
                    } else {
                        let VC = if VA < -5e1f64 { 1.0 } else { 0.0 };
                        if VC != 0.0 {
                        } else {
                        }
                    }
                } else {
                }
                let VD = if parameters[215] == C { 1.0 } else { 0.0 };
                if VD != 0.0 {
                    let VE = (PH - (PS - ((JU * EA) * PP))) / SM;
                    let VF = if VE > ES { 1.0 } else { 0.0 };
                    if VF != 0.0 {
                    } else {
                        let VG = if VE < -5e1f64 { 1.0 } else { 0.0 };
                        if VG != 0.0 {
                        } else {
                        }
                    }
                } else {
                }
            } else {
            }
            if IE != 0.0 {
            } else {
            }
            let VI = if VH > IU { 1.0 } else { 0.0 };
            if VI != 0.0 {
                let VP = if DQ != 0.0 {
                    let VN = ((IC * IC) + DZ).sqrt();
                    VN
                } else {
                    let VO = IC * (((EE / DZ) * IC).tanh());
                    VO
                };
                let VQ = VJ - IC;
                let VR = parameters[209] * AU;
                let VS = parameters[204] / (JJ * AU);
                let VT = VS + (parameters[205] * VP);
                let VU = parameters[190] + (parameters[206] * AV);
                let VV = AY.powf(JB);
                let VW = if JA != A { 1.0 } else { 0.0 };
                let VY = if VW != 0.0 {
                    let VX = VP / ((C + ((VP / JA).powf(VM))).powf((C / VM)));
                    VX
                } else {
                    A
                };
                let VZ = VU - ((parameters[203] - (VY * A)) * VP);
                let WA = (EX * VT) * AU;
                let WB = CT * WA;
                let WC = (JU * VR) / EX;
                let WD = VZ - WC;
                let WI = if DQ != 0.0 {
                    let WE = VJ - VQ;
                    let WF = EA * ((VJ + VQ) + (((WE * WE) + DZ).sqrt()));
                    WF
                } else {
                    let WG = VJ - VQ;
                    let WH = EA * ((VJ + VQ) + (WG * (((EE / DZ) * WG).tanh())));
                    WH
                };
                let WJ = (WI - WD) / VR;
                let WK = if WJ > ES { 1.0 } else { 0.0 };
                let WT;
                if WK != 0.0 {
                    WT = A;
                } else {
                    let WL = if WJ < -5e1f64 { 1.0 } else { 0.0 };
                    let WU = if WL != 0.0 {
                        C
                    } else {
                        let WM = C / (C + (WJ.exp()));
                        WM
                    };
                    WT = WU;
                }
                let WR = if DQ != 0.0 {
                    let WN = VJ - VQ;
                    let WO = EA * ((VJ + VQ) + (((WN * WN) + DZ).sqrt()));
                    WO
                } else {
                    let WP = VJ - VQ;
                    let WQ = EA * ((VJ + VQ) + (WP * (((EE / DZ) * WP).tanh())));
                    WQ
                };
                let WS = (JU * AE) * VR;
                let WV = (WR - (VZ - (WS * WT))) / WA;
                let WW = if WV > ES { 1.0 } else { 0.0 };
                let XB;
                if WW != 0.0 {
                    let WX = WB * WV;
                    XB = WX;
                } else {
                    let WY = if WV < -5e1f64 { 1.0 } else { 0.0 };
                    let XC = if WY != 0.0 {
                        let WZ = WB * (WV.exp());
                        WZ
                    } else {
                        let XA = WB * ((C + (WV.exp())).ln());
                        XA
                    };
                    XB = XC;
                }
                let XD = parameters[200] * ((C + (JC * B)) / (C + (JC * Z)));
                let XE = (((XD * (C + ((JD * VP) / VH))) / (C + ((parameters[207] * XB) / CT))) * VH) / (VL / (VV * (C + ((parameters[208] * XB) / CT))));
                let XF = (((XE * ((C + (((EX * XB) / CT) / XE)).sqrt())) - XE) * (C - WT)) + (WA * WT);
                let XG = IC / XF;
                let XL = if DQ != 0.0 {
                    let XH = A - XG;
                    let XI = EA * (XG + (((XH * XH) + DZ).sqrt()));
                    XI
                } else {
                    let XJ = A - XG;
                    let XK = EA * (XG + (XJ * (((EE / DZ) * XJ).tanh())));
                    XK
                };
                let XM = C / VM;
                let XN = IC * (C / ((C + (XL.powf(VM))).powf(XM)));
                let XO = -IC;
                let XP = XO / XF;
                let XU = if DQ != 0.0 {
                    let XQ = A - XP;
                    let XR = EA * (XP + (((XQ * XQ) + DZ).sqrt()));
                    XR
                } else {
                    let XS = A - XP;
                    let XT = EA * (XP + (XS * (((EE / DZ) * XS).tanh())));
                    XT
                };
                let XV = XO * (C / ((C + (XU.powf(VM))).powf(XM)));
                let XW = (VJ - WD) / VR;
                let XX = if XW > ES { 1.0 } else { 0.0 };
                let YA;
                if XX != 0.0 {
                    YA = A;
                } else {
                    let XY = if XW < -5e1f64 { 1.0 } else { 0.0 };
                    let YB = if XY != 0.0 {
                        C
                    } else {
                        let XZ = C / (C + (XW.exp()));
                        XZ
                    };
                    YA = YB;
                }
                let YC = ((VQ - XV) - (VZ - (WS * YA))) / WA;
                let YD = if YC > ES { 1.0 } else { 0.0 };
                if YD != 0.0 {
                } else {
                    let YE = if YC < -5e1f64 { 1.0 } else { 0.0 };
                    if YE != 0.0 {
                    } else {
                    }
                }
                let YF = (VQ - WD) / VR;
                let YG = if YF > ES { 1.0 } else { 0.0 };
                let YJ;
                if YG != 0.0 {
                    YJ = A;
                } else {
                    let YH = if YF < -5e1f64 { 1.0 } else { 0.0 };
                    let YK = if YH != 0.0 {
                        C
                    } else {
                        let YI = C / (C + (YF.exp()));
                        YI
                    };
                    YJ = YK;
                }
                let YL = ((VJ - XN) - (VZ - (WS * YJ))) / WA;
                let YM = if YL > ES { 1.0 } else { 0.0 };
                if YM != 0.0 {
                } else {
                    let YN = if YL < -5e1f64 { 1.0 } else { 0.0 };
                    if YN != 0.0 {
                    } else {
                    }
                }
                if DQ != 0.0 {
                } else {
                }
                let YO = (EX * VS) * AU;
                let YP = CT * YO;
                let YQ = VU - WC;
                let YV = if DQ != 0.0 {
                    let YR = VJ - VQ;
                    let YS = EA * ((VJ + VQ) + (((YR * YR) + DZ).sqrt()));
                    YS
                } else {
                    let YT = VJ - VQ;
                    let YU = EA * ((VJ + VQ) + (YT * (((EE / DZ) * YT).tanh())));
                    YU
                };
                let YW = (YV - YQ) / VR;
                let YX = if YW > ES { 1.0 } else { 0.0 };
                let ZF;
                if YX != 0.0 {
                    ZF = A;
                } else {
                    let YY = if YW < -5e1f64 { 1.0 } else { 0.0 };
                    let ZG = if YY != 0.0 {
                        C
                    } else {
                        let YZ = C / (C + (YW.exp()));
                        YZ
                    };
                    ZF = ZG;
                }
                let ZE = if DQ != 0.0 {
                    let ZA = VJ - VQ;
                    let ZB = EA * ((VJ + VQ) + (((ZA * ZA) + DZ).sqrt()));
                    ZB
                } else {
                    let ZC = VJ - VQ;
                    let ZD = EA * ((VJ + VQ) + (ZC * (((EE / DZ) * ZC).tanh())));
                    ZD
                };
                let ZH = (ZE - (VU - (WS * ZF))) / YO;
                let ZI = if ZH > ES { 1.0 } else { 0.0 };
                let ZO;
                if ZI != 0.0 {
                    let ZJ = YP * ZH;
                    ZO = ZJ;
                } else {
                    let ZK = if ZH < -5e1f64 { 1.0 } else { 0.0 };
                    let ZP = if ZK != 0.0 {
                        let ZL = YP * (ZH.exp());
                        ZL
                    } else {
                        let ZM = YP * ((C + (ZH.exp())).ln());
                        ZM
                    };
                    ZO = ZP;
                }
                let ZN = (XD * VH) / (VL / VV);
                let ZQ = (((ZN * ((C + (((EX * ZO) / CT) / ZN)).sqrt())) - ZN) * (C - ZF)) + (YO * ZF);
                let ZR = IC / ZQ;
                let ZW = if DQ != 0.0 {
                    let ZS = A - ZR;
                    let ZT = EA * (ZR + (((ZS * ZS) + DZ).sqrt()));
                    ZT
                } else {
                    let ZU = A - ZR;
                    let ZV = EA * (ZR + (ZU * (((EE / DZ) * ZU).tanh())));
                    ZV
                };
                let ZX = IC * (C / ((C + (ZW.powf(VM))).powf(XM)));
                let ZY = XO / ZQ;
                let AAD = if DQ != 0.0 {
                    let ZZ = A - ZY;
                    let AAA = EA * (ZY + (((ZZ * ZZ) + DZ).sqrt()));
                    AAA
                } else {
                    let AAB = A - ZY;
                    let AAC = EA * (ZY + (AAB * (((EE / DZ) * AAB).tanh())));
                    AAC
                };
                let AAE = XO * (C / ((C + (AAD.powf(VM))).powf(XM)));
                let AAF = (VJ - YQ) / VR;
                let AAG = if AAF > ES { 1.0 } else { 0.0 };
                let AAJ;
                if AAG != 0.0 {
                    AAJ = A;
                } else {
                    let AAH = if AAF < -5e1f64 { 1.0 } else { 0.0 };
                    let AAK = if AAH != 0.0 {
                        C
                    } else {
                        let AAI = C / (C + (AAF.exp()));
                        AAI
                    };
                    AAJ = AAK;
                }
                let AAL = ((VQ - AAE) - (VU - (WS * AAJ))) / YO;
                let AAM = if AAL > ES { 1.0 } else { 0.0 };
                if AAM != 0.0 {
                } else {
                    let AAN = if AAL < -5e1f64 { 1.0 } else { 0.0 };
                    if AAN != 0.0 {
                    } else {
                    }
                }
                let AAO = (VQ - YQ) / VR;
                let AAP = if AAO > ES { 1.0 } else { 0.0 };
                let AAS;
                if AAP != 0.0 {
                    AAS = A;
                } else {
                    let AAQ = if AAO < -5e1f64 { 1.0 } else { 0.0 };
                    let AAT = if AAQ != 0.0 {
                        C
                    } else {
                        let AAR = C / (C + (AAO.exp()));
                        AAR
                    };
                    AAS = AAT;
                }
                let AAU = ((VJ - ZX) - (VU - (WS * AAS))) / YO;
                let AAV = if AAU > ES { 1.0 } else { 0.0 };
                if AAV != 0.0 {
                } else {
                    let AAW = if AAU < -5e1f64 { 1.0 } else { 0.0 };
                    if AAW != 0.0 {
                    } else {
                    }
                }
                let AAX = if parameters[195] == C { 1.0 } else { 0.0 };
                if AAX != 0.0 {
                    let AAY = VU - ((JU * EA) * VR);
                    let AAZ = (VK - AAY) / YO;
                    let ABA = if AAZ > ES { 1.0 } else { 0.0 };
                    if ABA != 0.0 {
                    } else {
                        let ABB = if AAZ < -5e1f64 { 1.0 } else { 0.0 };
                        if ABB != 0.0 {
                        } else {
                        }
                    }
                    let ABC = (ID - AAY) / YO;
                    let ABD = if ABC > ES { 1.0 } else { 0.0 };
                    if ABD != 0.0 {
                    } else {
                        let ABE = if ABC < -5e1f64 { 1.0 } else { 0.0 };
                        if ABE != 0.0 {
                        } else {
                        }
                    }
                } else {
                }
                let ABF = if parameters[193] == C { 1.0 } else { 0.0 };
                if ABF != 0.0 {
                    let ABG = (VJ - (VU - ((JU * EA) * VR))) / YO;
                    let ABH = if ABG > ES { 1.0 } else { 0.0 };
                    if ABH != 0.0 {
                    } else {
                        let ABI = if ABG < -5e1f64 { 1.0 } else { 0.0 };
                        if ABI != 0.0 {
                        } else {
                        }
                    }
                } else {
                }
            } else {
            }
            if HW != 0.0 {
            } else {
            }
            let ABK = if ABJ > IU { 1.0 } else { 0.0 };
            if ABK != 0.0 {
                let ABR = if DQ != 0.0 {
                    let ABP = ((HU * HU) + DZ).sqrt();
                    ABP
                } else {
                    let ABQ = HU * (((EE / DZ) * HU).tanh());
                    ABQ
                };
                let ABS = ABL - HU;
                let ABT = parameters[187] * AU;
                let ABU = parameters[182] / (JJ * AU);
                let ABV = ABU + (parameters[183] * ABR);
                let ABW = parameters[168] + (parameters[184] * AV);
                let ABX = AY.powf(JB);
                let ABY = if JA != A { 1.0 } else { 0.0 };
                let ACA = if ABY != 0.0 {
                    let ABZ = ABR / ((C + ((ABR / JA).powf(ABO))).powf((C / ABO)));
                    ABZ
                } else {
                    A
                };
                let ACB = ABW - ((parameters[181] - (ACA * A)) * ABR);
                let ACC = (EX * ABV) * AU;
                let ACD = CP * ACC;
                let ACE = (JU * ABT) / EX;
                let ACF = ACB - ACE;
                let ACK = if DQ != 0.0 {
                    let ACG = ABL - ABS;
                    let ACH = EA * ((ABL + ABS) + (((ACG * ACG) + DZ).sqrt()));
                    ACH
                } else {
                    let ACI = ABL - ABS;
                    let ACJ = EA * ((ABL + ABS) + (ACI * (((EE / DZ) * ACI).tanh())));
                    ACJ
                };
                let ACL = (ACK - ACF) / ABT;
                let ACM = if ACL > ES { 1.0 } else { 0.0 };
                let ACV;
                if ACM != 0.0 {
                    ACV = A;
                } else {
                    let ACN = if ACL < -5e1f64 { 1.0 } else { 0.0 };
                    let ACW = if ACN != 0.0 {
                        C
                    } else {
                        let ACO = C / (C + (ACL.exp()));
                        ACO
                    };
                    ACV = ACW;
                }
                let ACT = if DQ != 0.0 {
                    let ACP = ABL - ABS;
                    let ACQ = EA * ((ABL + ABS) + (((ACP * ACP) + DZ).sqrt()));
                    ACQ
                } else {
                    let ACR = ABL - ABS;
                    let ACS = EA * ((ABL + ABS) + (ACR * (((EE / DZ) * ACR).tanh())));
                    ACS
                };
                let ACU = (JU * AE) * ABT;
                let ACX = (ACT - (ACB - (ACU * ACV))) / ACC;
                let ACY = if ACX > ES { 1.0 } else { 0.0 };
                let ADD;
                if ACY != 0.0 {
                    let ACZ = ACD * ACX;
                    ADD = ACZ;
                } else {
                    let ADA = if ACX < -5e1f64 { 1.0 } else { 0.0 };
                    let ADE = if ADA != 0.0 {
                        let ADB = ACD * (ACX.exp());
                        ADB
                    } else {
                        let ADC = ACD * ((C + (ACX.exp())).ln());
                        ADC
                    };
                    ADD = ADE;
                }
                let ADF = parameters[178] * ((C + (JC * B)) / (C + (JC * Z)));
                let ADG = (((ADF * (C + ((JD * ABR) / ABJ))) / (C + ((parameters[185] * ADD) / CP))) * ABJ) / (ABN / (ABX * (C + ((parameters[186] * ADD) / CP))));
                let ADH = (((ADG * ((C + (((EX * ADD) / CP) / ADG)).sqrt())) - ADG) * (C - ACV)) + (ACC * ACV);
                let ADI = HU / ADH;
                let ADN = if DQ != 0.0 {
                    let ADJ = A - ADI;
                    let ADK = EA * (ADI + (((ADJ * ADJ) + DZ).sqrt()));
                    ADK
                } else {
                    let ADL = A - ADI;
                    let ADM = EA * (ADI + (ADL * (((EE / DZ) * ADL).tanh())));
                    ADM
                };
                let ADO = C / ABO;
                let ADP = HU * (C / ((C + (ADN.powf(ABO))).powf(ADO)));
                let ADQ = -HU;
                let ADR = ADQ / ADH;
                let ADW = if DQ != 0.0 {
                    let ADS = A - ADR;
                    let ADT = EA * (ADR + (((ADS * ADS) + DZ).sqrt()));
                    ADT
                } else {
                    let ADU = A - ADR;
                    let ADV = EA * (ADR + (ADU * (((EE / DZ) * ADU).tanh())));
                    ADV
                };
                let ADX = ADQ * (C / ((C + (ADW.powf(ABO))).powf(ADO)));
                let ADY = (ABL - ACF) / ABT;
                let ADZ = if ADY > ES { 1.0 } else { 0.0 };
                let AEC;
                if ADZ != 0.0 {
                    AEC = A;
                } else {
                    let AEA = if ADY < -5e1f64 { 1.0 } else { 0.0 };
                    let AED = if AEA != 0.0 {
                        C
                    } else {
                        let AEB = C / (C + (ADY.exp()));
                        AEB
                    };
                    AEC = AED;
                }
                let AEE = ((ABS - ADX) - (ACB - (ACU * AEC))) / ACC;
                let AEF = if AEE > ES { 1.0 } else { 0.0 };
                if AEF != 0.0 {
                } else {
                    let AEG = if AEE < -5e1f64 { 1.0 } else { 0.0 };
                    if AEG != 0.0 {
                    } else {
                    }
                }
                let AEH = (ABS - ACF) / ABT;
                let AEI = if AEH > ES { 1.0 } else { 0.0 };
                let AEL;
                if AEI != 0.0 {
                    AEL = A;
                } else {
                    let AEJ = if AEH < -5e1f64 { 1.0 } else { 0.0 };
                    let AEM = if AEJ != 0.0 {
                        C
                    } else {
                        let AEK = C / (C + (AEH.exp()));
                        AEK
                    };
                    AEL = AEM;
                }
                let AEN = ((ABL - ADP) - (ACB - (ACU * AEL))) / ACC;
                let AEO = if AEN > ES { 1.0 } else { 0.0 };
                if AEO != 0.0 {
                } else {
                    let AEP = if AEN < -5e1f64 { 1.0 } else { 0.0 };
                    if AEP != 0.0 {
                    } else {
                    }
                }
                if DQ != 0.0 {
                } else {
                }
                let AEQ = (EX * ABU) * AU;
                let AER = CP * AEQ;
                let AES = ABW - ACE;
                let AEX = if DQ != 0.0 {
                    let AET = ABL - ABS;
                    let AEU = EA * ((ABL + ABS) + (((AET * AET) + DZ).sqrt()));
                    AEU
                } else {
                    let AEV = ABL - ABS;
                    let AEW = EA * ((ABL + ABS) + (AEV * (((EE / DZ) * AEV).tanh())));
                    AEW
                };
                let AEY = (AEX - AES) / ABT;
                let AEZ = if AEY > ES { 1.0 } else { 0.0 };
                let AFH;
                if AEZ != 0.0 {
                    AFH = A;
                } else {
                    let AFA = if AEY < -5e1f64 { 1.0 } else { 0.0 };
                    let AFI = if AFA != 0.0 {
                        C
                    } else {
                        let AFB = C / (C + (AEY.exp()));
                        AFB
                    };
                    AFH = AFI;
                }
                let AFG = if DQ != 0.0 {
                    let AFC = ABL - ABS;
                    let AFD = EA * ((ABL + ABS) + (((AFC * AFC) + DZ).sqrt()));
                    AFD
                } else {
                    let AFE = ABL - ABS;
                    let AFF = EA * ((ABL + ABS) + (AFE * (((EE / DZ) * AFE).tanh())));
                    AFF
                };
                let AFJ = (AFG - (ABW - (ACU * AFH))) / AEQ;
                let AFK = if AFJ > ES { 1.0 } else { 0.0 };
                let AFQ;
                if AFK != 0.0 {
                    let AFL = AER * AFJ;
                    AFQ = AFL;
                } else {
                    let AFM = if AFJ < -5e1f64 { 1.0 } else { 0.0 };
                    let AFR = if AFM != 0.0 {
                        let AFN = AER * (AFJ.exp());
                        AFN
                    } else {
                        let AFO = AER * ((C + (AFJ.exp())).ln());
                        AFO
                    };
                    AFQ = AFR;
                }
                let AFP = (ADF * ABJ) / (ABN / ABX);
                let AFS = (((AFP * ((C + (((EX * AFQ) / CP) / AFP)).sqrt())) - AFP) * (C - AFH)) + (AEQ * AFH);
                let AFT = HU / AFS;
                let AFY = if DQ != 0.0 {
                    let AFU = A - AFT;
                    let AFV = EA * (AFT + (((AFU * AFU) + DZ).sqrt()));
                    AFV
                } else {
                    let AFW = A - AFT;
                    let AFX = EA * (AFT + (AFW * (((EE / DZ) * AFW).tanh())));
                    AFX
                };
                let AFZ = HU * (C / ((C + (AFY.powf(ABO))).powf(ADO)));
                let AGA = ADQ / AFS;
                let AGF = if DQ != 0.0 {
                    let AGB = A - AGA;
                    let AGC = EA * (AGA + (((AGB * AGB) + DZ).sqrt()));
                    AGC
                } else {
                    let AGD = A - AGA;
                    let AGE = EA * (AGA + (AGD * (((EE / DZ) * AGD).tanh())));
                    AGE
                };
                let AGG = ADQ * (C / ((C + (AGF.powf(ABO))).powf(ADO)));
                let AGH = (ABL - AES) / ABT;
                let AGI = if AGH > ES { 1.0 } else { 0.0 };
                let AGL;
                if AGI != 0.0 {
                    AGL = A;
                } else {
                    let AGJ = if AGH < -5e1f64 { 1.0 } else { 0.0 };
                    let AGM = if AGJ != 0.0 {
                        C
                    } else {
                        let AGK = C / (C + (AGH.exp()));
                        AGK
                    };
                    AGL = AGM;
                }
                let AGN = ((ABS - AGG) - (ABW - (ACU * AGL))) / AEQ;
                let AGO = if AGN > ES { 1.0 } else { 0.0 };
                if AGO != 0.0 {
                } else {
                    let AGP = if AGN < -5e1f64 { 1.0 } else { 0.0 };
                    if AGP != 0.0 {
                    } else {
                    }
                }
                let AGQ = (ABS - AES) / ABT;
                let AGR = if AGQ > ES { 1.0 } else { 0.0 };
                let AGU;
                if AGR != 0.0 {
                    AGU = A;
                } else {
                    let AGS = if AGQ < -5e1f64 { 1.0 } else { 0.0 };
                    let AGV = if AGS != 0.0 {
                        C
                    } else {
                        let AGT = C / (C + (AGQ.exp()));
                        AGT
                    };
                    AGU = AGV;
                }
                let AGW = ((ABL - AFZ) - (ABW - (ACU * AGU))) / AEQ;
                let AGX = if AGW > ES { 1.0 } else { 0.0 };
                if AGX != 0.0 {
                } else {
                    let AGY = if AGW < -5e1f64 { 1.0 } else { 0.0 };
                    if AGY != 0.0 {
                    } else {
                    }
                }
                let AGZ = if parameters[173] == C { 1.0 } else { 0.0 };
                if AGZ != 0.0 {
                    let AHA = ABW - ((JU * EA) * ABT);
                    let AHB = (ABM - AHA) / AEQ;
                    let AHC = if AHB > ES { 1.0 } else { 0.0 };
                    if AHC != 0.0 {
                    } else {
                        let AHD = if AHB < -5e1f64 { 1.0 } else { 0.0 };
                        if AHD != 0.0 {
                        } else {
                        }
                    }
                    let AHE = (HV - AHA) / AEQ;
                    let AHF = if AHE > ES { 1.0 } else { 0.0 };
                    if AHF != 0.0 {
                    } else {
                        let AHG = if AHE < -5e1f64 { 1.0 } else { 0.0 };
                        if AHG != 0.0 {
                        } else {
                        }
                    }
                } else {
                }
                let AHH = if parameters[171] == C { 1.0 } else { 0.0 };
                if AHH != 0.0 {
                    let AHI = (ABL - (ABW - ((JU * EA) * ABT))) / AEQ;
                    let AHJ = if AHI > ES { 1.0 } else { 0.0 };
                    if AHJ != 0.0 {
                    } else {
                        let AHK = if AHI < -5e1f64 { 1.0 } else { 0.0 };
                        if AHK != 0.0 {
                        } else {
                        }
                    }
                } else {
                }
            } else {
            }
            if HO != 0.0 {
            } else {
            }
            let AHM = if AHL > IU { 1.0 } else { 0.0 };
            if AHM != 0.0 {
                let AHT = if DQ != 0.0 {
                    let AHR = ((GO * GO) + DZ).sqrt();
                    AHR
                } else {
                    let AHS = GO * (((EE / DZ) * GO).tanh());
                    AHS
                };
                let AHU = AHN - GO;
                let AHV = parameters[99] * AU;
                let AHW = parameters[94] / (JJ * AU);
                let AHX = AHW + (parameters[95] * AHT);
                let AHY = parameters[80] + (parameters[96] * AV);
                let AHZ = AY.powf(JB);
                let AIA = if JA != A { 1.0 } else { 0.0 };
                let AIC = if AIA != 0.0 {
                    let AIB = AHT / ((C + ((AHT / JA).powf(AHQ))).powf((C / AHQ)));
                    AIB
                } else {
                    A
                };
                let AID = AHY - ((parameters[93] - (AIC * A)) * AHT);
                let AIE = (EX * AHX) * AU;
                let AIF = BR * AIE;
                let AIG = (JU * AHV) / EX;
                let AIH = AID - AIG;
                let AIM = if DQ != 0.0 {
                    let AII = AHN - AHU;
                    let AIJ = EA * ((AHN + AHU) + (((AII * AII) + DZ).sqrt()));
                    AIJ
                } else {
                    let AIK = AHN - AHU;
                    let AIL = EA * ((AHN + AHU) + (AIK * (((EE / DZ) * AIK).tanh())));
                    AIL
                };
                let AIN = (AIM - AIH) / AHV;
                let AIO = if AIN > ES { 1.0 } else { 0.0 };
                let AIX;
                if AIO != 0.0 {
                    AIX = A;
                } else {
                    let AIP = if AIN < -5e1f64 { 1.0 } else { 0.0 };
                    let AIY = if AIP != 0.0 {
                        C
                    } else {
                        let AIQ = C / (C + (AIN.exp()));
                        AIQ
                    };
                    AIX = AIY;
                }
                let AIV = if DQ != 0.0 {
                    let AIR = AHN - AHU;
                    let AIS = EA * ((AHN + AHU) + (((AIR * AIR) + DZ).sqrt()));
                    AIS
                } else {
                    let AIT = AHN - AHU;
                    let AIU = EA * ((AHN + AHU) + (AIT * (((EE / DZ) * AIT).tanh())));
                    AIU
                };
                let AIW = (JU * AE) * AHV;
                let AIZ = (AIV - (AID - (AIW * AIX))) / AIE;
                let AJA = if AIZ > ES { 1.0 } else { 0.0 };
                let AJF;
                if AJA != 0.0 {
                    let AJB = AIF * AIZ;
                    AJF = AJB;
                } else {
                    let AJC = if AIZ < -5e1f64 { 1.0 } else { 0.0 };
                    let AJG = if AJC != 0.0 {
                        let AJD = AIF * (AIZ.exp());
                        AJD
                    } else {
                        let AJE = AIF * ((C + (AIZ.exp())).ln());
                        AJE
                    };
                    AJF = AJG;
                }
                let AJH = parameters[90] * ((C + (JC * B)) / (C + (JC * Z)));
                let AJI = (((AJH * (C + ((JD * AHT) / AHL))) / (C + ((parameters[97] * AJF) / BR))) * AHL) / (AHP / (AHZ * (C + ((parameters[98] * AJF) / BR))));
                let AJJ = (((AJI * ((C + (((EX * AJF) / BR) / AJI)).sqrt())) - AJI) * (C - AIX)) + (AIE * AIX);
                let AJK = GO / AJJ;
                let AJP = if DQ != 0.0 {
                    let AJL = A - AJK;
                    let AJM = EA * (AJK + (((AJL * AJL) + DZ).sqrt()));
                    AJM
                } else {
                    let AJN = A - AJK;
                    let AJO = EA * (AJK + (AJN * (((EE / DZ) * AJN).tanh())));
                    AJO
                };
                let AJQ = C / AHQ;
                let AJR = GO * (C / ((C + (AJP.powf(AHQ))).powf(AJQ)));
                let AJS = -GO;
                let AJT = AJS / AJJ;
                let AJY = if DQ != 0.0 {
                    let AJU = A - AJT;
                    let AJV = EA * (AJT + (((AJU * AJU) + DZ).sqrt()));
                    AJV
                } else {
                    let AJW = A - AJT;
                    let AJX = EA * (AJT + (AJW * (((EE / DZ) * AJW).tanh())));
                    AJX
                };
                let AJZ = AJS * (C / ((C + (AJY.powf(AHQ))).powf(AJQ)));
                let AKA = (AHN - AIH) / AHV;
                let AKB = if AKA > ES { 1.0 } else { 0.0 };
                let AKE;
                if AKB != 0.0 {
                    AKE = A;
                } else {
                    let AKC = if AKA < -5e1f64 { 1.0 } else { 0.0 };
                    let AKF = if AKC != 0.0 {
                        C
                    } else {
                        let AKD = C / (C + (AKA.exp()));
                        AKD
                    };
                    AKE = AKF;
                }
                let AKG = ((AHU - AJZ) - (AID - (AIW * AKE))) / AIE;
                let AKH = if AKG > ES { 1.0 } else { 0.0 };
                if AKH != 0.0 {
                } else {
                    let AKI = if AKG < -5e1f64 { 1.0 } else { 0.0 };
                    if AKI != 0.0 {
                    } else {
                    }
                }
                let AKJ = (AHU - AIH) / AHV;
                let AKK = if AKJ > ES { 1.0 } else { 0.0 };
                let AKN;
                if AKK != 0.0 {
                    AKN = A;
                } else {
                    let AKL = if AKJ < -5e1f64 { 1.0 } else { 0.0 };
                    let AKO = if AKL != 0.0 {
                        C
                    } else {
                        let AKM = C / (C + (AKJ.exp()));
                        AKM
                    };
                    AKN = AKO;
                }
                let AKP = ((AHN - AJR) - (AID - (AIW * AKN))) / AIE;
                let AKQ = if AKP > ES { 1.0 } else { 0.0 };
                if AKQ != 0.0 {
                } else {
                    let AKR = if AKP < -5e1f64 { 1.0 } else { 0.0 };
                    if AKR != 0.0 {
                    } else {
                    }
                }
                if DQ != 0.0 {
                } else {
                }
                let AKS = (EX * AHW) * AU;
                let AKT = BR * AKS;
                let AKU = AHY - AIG;
                let AKZ = if DQ != 0.0 {
                    let AKV = AHN - AHU;
                    let AKW = EA * ((AHN + AHU) + (((AKV * AKV) + DZ).sqrt()));
                    AKW
                } else {
                    let AKX = AHN - AHU;
                    let AKY = EA * ((AHN + AHU) + (AKX * (((EE / DZ) * AKX).tanh())));
                    AKY
                };
                let ALA = (AKZ - AKU) / AHV;
                let ALB = if ALA > ES { 1.0 } else { 0.0 };
                let ALJ;
                if ALB != 0.0 {
                    ALJ = A;
                } else {
                    let ALC = if ALA < -5e1f64 { 1.0 } else { 0.0 };
                    let ALK = if ALC != 0.0 {
                        C
                    } else {
                        let ALD = C / (C + (ALA.exp()));
                        ALD
                    };
                    ALJ = ALK;
                }
                let ALI = if DQ != 0.0 {
                    let ALE = AHN - AHU;
                    let ALF = EA * ((AHN + AHU) + (((ALE * ALE) + DZ).sqrt()));
                    ALF
                } else {
                    let ALG = AHN - AHU;
                    let ALH = EA * ((AHN + AHU) + (ALG * (((EE / DZ) * ALG).tanh())));
                    ALH
                };
                let ALL = (ALI - (AHY - (AIW * ALJ))) / AKS;
                let ALM = if ALL > ES { 1.0 } else { 0.0 };
                let ALS;
                if ALM != 0.0 {
                    let ALN = AKT * ALL;
                    ALS = ALN;
                } else {
                    let ALO = if ALL < -5e1f64 { 1.0 } else { 0.0 };
                    let ALT = if ALO != 0.0 {
                        let ALP = AKT * (ALL.exp());
                        ALP
                    } else {
                        let ALQ = AKT * ((C + (ALL.exp())).ln());
                        ALQ
                    };
                    ALS = ALT;
                }
                let ALR = (AJH * AHL) / (AHP / AHZ);
                let ALU = (((ALR * ((C + (((EX * ALS) / BR) / ALR)).sqrt())) - ALR) * (C - ALJ)) + (AKS * ALJ);
                let ALV = GO / ALU;
                let AMA = if DQ != 0.0 {
                    let ALW = A - ALV;
                    let ALX = EA * (ALV + (((ALW * ALW) + DZ).sqrt()));
                    ALX
                } else {
                    let ALY = A - ALV;
                    let ALZ = EA * (ALV + (ALY * (((EE / DZ) * ALY).tanh())));
                    ALZ
                };
                let AMB = GO * (C / ((C + (AMA.powf(AHQ))).powf(AJQ)));
                let AMC = AJS / ALU;
                let AMH = if DQ != 0.0 {
                    let AMD = A - AMC;
                    let AME = EA * (AMC + (((AMD * AMD) + DZ).sqrt()));
                    AME
                } else {
                    let AMF = A - AMC;
                    let AMG = EA * (AMC + (AMF * (((EE / DZ) * AMF).tanh())));
                    AMG
                };
                let AMI = AJS * (C / ((C + (AMH.powf(AHQ))).powf(AJQ)));
                let AMJ = (AHN - AKU) / AHV;
                let AMK = if AMJ > ES { 1.0 } else { 0.0 };
                let AMN;
                if AMK != 0.0 {
                    AMN = A;
                } else {
                    let AML = if AMJ < -5e1f64 { 1.0 } else { 0.0 };
                    let AMO = if AML != 0.0 {
                        C
                    } else {
                        let AMM = C / (C + (AMJ.exp()));
                        AMM
                    };
                    AMN = AMO;
                }
                let AMP = ((AHU - AMI) - (AHY - (AIW * AMN))) / AKS;
                let AMQ = if AMP > ES { 1.0 } else { 0.0 };
                if AMQ != 0.0 {
                } else {
                    let AMR = if AMP < -5e1f64 { 1.0 } else { 0.0 };
                    if AMR != 0.0 {
                    } else {
                    }
                }
                let AMS = (AHU - AKU) / AHV;
                let AMT = if AMS > ES { 1.0 } else { 0.0 };
                let AMW;
                if AMT != 0.0 {
                    AMW = A;
                } else {
                    let AMU = if AMS < -5e1f64 { 1.0 } else { 0.0 };
                    let AMX = if AMU != 0.0 {
                        C
                    } else {
                        let AMV = C / (C + (AMS.exp()));
                        AMV
                    };
                    AMW = AMX;
                }
                let AMY = ((AHN - AMB) - (AHY - (AIW * AMW))) / AKS;
                let AMZ = if AMY > ES { 1.0 } else { 0.0 };
                if AMZ != 0.0 {
                } else {
                    let ANA = if AMY < -5e1f64 { 1.0 } else { 0.0 };
                    if ANA != 0.0 {
                    } else {
                    }
                }
                let ANB = if parameters[85] == C { 1.0 } else { 0.0 };
                if ANB != 0.0 {
                    let ANC = AHY - ((JU * EA) * AHV);
                    let AND = (AHO - ANC) / AKS;
                    let ANE = if AND > ES { 1.0 } else { 0.0 };
                    if ANE != 0.0 {
                    } else {
                        let ANF = if AND < -5e1f64 { 1.0 } else { 0.0 };
                        if ANF != 0.0 {
                        } else {
                        }
                    }
                    let ANG = (GQ - ANC) / AKS;
                    let ANH = if ANG > ES { 1.0 } else { 0.0 };
                    if ANH != 0.0 {
                    } else {
                        let ANI = if ANG < -5e1f64 { 1.0 } else { 0.0 };
                        if ANI != 0.0 {
                        } else {
                        }
                    }
                } else {
                }
                let ANJ = if parameters[83] == C { 1.0 } else { 0.0 };
                if ANJ != 0.0 {
                    let ANK = (AHN - (AHY - ((JU * EA) * AHV))) / AKS;
                    let ANL = if ANK > ES { 1.0 } else { 0.0 };
                    if ANL != 0.0 {
                    } else {
                        let ANM = if ANK < -5e1f64 { 1.0 } else { 0.0 };
                        if ANM != 0.0 {
                        } else {
                        }
                    }
                } else {
                }
            } else {
            }
            if GH != 0.0 {
            } else {
            }
            let ANO = if ANN > IU { 1.0 } else { 0.0 };
            if ANO != 0.0 {
                let ANV = if DQ != 0.0 {
                    let ANT = ((GX * GX) + DZ).sqrt();
                    ANT
                } else {
                    let ANU = GX * (((EE / DZ) * GX).tanh());
                    ANU
                };
                let ANW = ANP - GX;
                let ANX = parameters[121] * AU;
                let ANY = parameters[116] / (JJ * AU);
                let ANZ = ANY + (parameters[117] * ANV);
                let AOA = parameters[102] + (parameters[118] * AV);
                let AOB = AY.powf(JB);
                let AOC = if JA != A { 1.0 } else { 0.0 };
                let AOE = if AOC != 0.0 {
                    let AOD = ANV / ((C + ((ANV / JA).powf(ANS))).powf((C / ANS)));
                    AOD
                } else {
                    A
                };
                let AOF = AOA - ((parameters[115] - (AOE * A)) * ANV);
                let AOG = (EX * ANZ) * AU;
                let AOH = BV * AOG;
                let AOI = (JU * ANX) / EX;
                let AOJ = AOF - AOI;
                let AOO = if DQ != 0.0 {
                    let AOK = ANP - ANW;
                    let AOL = EA * ((ANP + ANW) + (((AOK * AOK) + DZ).sqrt()));
                    AOL
                } else {
                    let AOM = ANP - ANW;
                    let AON = EA * ((ANP + ANW) + (AOM * (((EE / DZ) * AOM).tanh())));
                    AON
                };
                let AOP = (AOO - AOJ) / ANX;
                let AOQ = if AOP > ES { 1.0 } else { 0.0 };
                let AOZ;
                if AOQ != 0.0 {
                    AOZ = A;
                } else {
                    let AOR = if AOP < -5e1f64 { 1.0 } else { 0.0 };
                    let APA = if AOR != 0.0 {
                        C
                    } else {
                        let AOS = C / (C + (AOP.exp()));
                        AOS
                    };
                    AOZ = APA;
                }
                let AOX = if DQ != 0.0 {
                    let AOT = ANP - ANW;
                    let AOU = EA * ((ANP + ANW) + (((AOT * AOT) + DZ).sqrt()));
                    AOU
                } else {
                    let AOV = ANP - ANW;
                    let AOW = EA * ((ANP + ANW) + (AOV * (((EE / DZ) * AOV).tanh())));
                    AOW
                };
                let AOY = (JU * AE) * ANX;
                let APB = (AOX - (AOF - (AOY * AOZ))) / AOG;
                let APC = if APB > ES { 1.0 } else { 0.0 };
                let APH;
                if APC != 0.0 {
                    let APD = AOH * APB;
                    APH = APD;
                } else {
                    let APE = if APB < -5e1f64 { 1.0 } else { 0.0 };
                    let API = if APE != 0.0 {
                        let APF = AOH * (APB.exp());
                        APF
                    } else {
                        let APG = AOH * ((C + (APB.exp())).ln());
                        APG
                    };
                    APH = API;
                }
                let APJ = parameters[112] * ((C + (JC * B)) / (C + (JC * Z)));
                let APK = (((APJ * (C + ((JD * ANV) / ANN))) / (C + ((parameters[119] * APH) / BV))) * ANN) / (ANR / (AOB * (C + ((parameters[120] * APH) / BV))));
                let APL = (((APK * ((C + (((EX * APH) / BV) / APK)).sqrt())) - APK) * (C - AOZ)) + (AOG * AOZ);
                let APM = GX / APL;
                let APR = if DQ != 0.0 {
                    let APN = A - APM;
                    let APO = EA * (APM + (((APN * APN) + DZ).sqrt()));
                    APO
                } else {
                    let APP = A - APM;
                    let APQ = EA * (APM + (APP * (((EE / DZ) * APP).tanh())));
                    APQ
                };
                let APS = C / ANS;
                let APT = GX * (C / ((C + (APR.powf(ANS))).powf(APS)));
                let APU = -GX;
                let APV = APU / APL;
                let AQA = if DQ != 0.0 {
                    let APW = A - APV;
                    let APX = EA * (APV + (((APW * APW) + DZ).sqrt()));
                    APX
                } else {
                    let APY = A - APV;
                    let APZ = EA * (APV + (APY * (((EE / DZ) * APY).tanh())));
                    APZ
                };
                let AQB = APU * (C / ((C + (AQA.powf(ANS))).powf(APS)));
                let AQC = (ANP - AOJ) / ANX;
                let AQD = if AQC > ES { 1.0 } else { 0.0 };
                let AQG;
                if AQD != 0.0 {
                    AQG = A;
                } else {
                    let AQE = if AQC < -5e1f64 { 1.0 } else { 0.0 };
                    let AQH = if AQE != 0.0 {
                        C
                    } else {
                        let AQF = C / (C + (AQC.exp()));
                        AQF
                    };
                    AQG = AQH;
                }
                let AQI = ((ANW - AQB) - (AOF - (AOY * AQG))) / AOG;
                let AQJ = if AQI > ES { 1.0 } else { 0.0 };
                if AQJ != 0.0 {
                } else {
                    let AQK = if AQI < -5e1f64 { 1.0 } else { 0.0 };
                    if AQK != 0.0 {
                    } else {
                    }
                }
                let AQL = (ANW - AOJ) / ANX;
                let AQM = if AQL > ES { 1.0 } else { 0.0 };
                let AQP;
                if AQM != 0.0 {
                    AQP = A;
                } else {
                    let AQN = if AQL < -5e1f64 { 1.0 } else { 0.0 };
                    let AQQ = if AQN != 0.0 {
                        C
                    } else {
                        let AQO = C / (C + (AQL.exp()));
                        AQO
                    };
                    AQP = AQQ;
                }
                let AQR = ((ANP - APT) - (AOF - (AOY * AQP))) / AOG;
                let AQS = if AQR > ES { 1.0 } else { 0.0 };
                if AQS != 0.0 {
                } else {
                    let AQT = if AQR < -5e1f64 { 1.0 } else { 0.0 };
                    if AQT != 0.0 {
                    } else {
                    }
                }
                if DQ != 0.0 {
                } else {
                }
                let AQU = (EX * ANY) * AU;
                let AQV = BV * AQU;
                let AQW = AOA - AOI;
                let ARB = if DQ != 0.0 {
                    let AQX = ANP - ANW;
                    let AQY = EA * ((ANP + ANW) + (((AQX * AQX) + DZ).sqrt()));
                    AQY
                } else {
                    let AQZ = ANP - ANW;
                    let ARA = EA * ((ANP + ANW) + (AQZ * (((EE / DZ) * AQZ).tanh())));
                    ARA
                };
                let ARC = (ARB - AQW) / ANX;
                let ARD = if ARC > ES { 1.0 } else { 0.0 };
                let ARL;
                if ARD != 0.0 {
                    ARL = A;
                } else {
                    let ARE = if ARC < -5e1f64 { 1.0 } else { 0.0 };
                    let ARM = if ARE != 0.0 {
                        C
                    } else {
                        let ARF = C / (C + (ARC.exp()));
                        ARF
                    };
                    ARL = ARM;
                }
                let ARK = if DQ != 0.0 {
                    let ARG = ANP - ANW;
                    let ARH = EA * ((ANP + ANW) + (((ARG * ARG) + DZ).sqrt()));
                    ARH
                } else {
                    let ARI = ANP - ANW;
                    let ARJ = EA * ((ANP + ANW) + (ARI * (((EE / DZ) * ARI).tanh())));
                    ARJ
                };
                let ARN = (ARK - (AOA - (AOY * ARL))) / AQU;
                let ARO = if ARN > ES { 1.0 } else { 0.0 };
                let ARU;
                if ARO != 0.0 {
                    let ARP = AQV * ARN;
                    ARU = ARP;
                } else {
                    let ARQ = if ARN < -5e1f64 { 1.0 } else { 0.0 };
                    let ARV = if ARQ != 0.0 {
                        let ARR = AQV * (ARN.exp());
                        ARR
                    } else {
                        let ARS = AQV * ((C + (ARN.exp())).ln());
                        ARS
                    };
                    ARU = ARV;
                }
                let ART = (APJ * ANN) / (ANR / AOB);
                let ARW = (((ART * ((C + (((EX * ARU) / BV) / ART)).sqrt())) - ART) * (C - ARL)) + (AQU * ARL);
                let ARX = GX / ARW;
                let ASC = if DQ != 0.0 {
                    let ARY = A - ARX;
                    let ARZ = EA * (ARX + (((ARY * ARY) + DZ).sqrt()));
                    ARZ
                } else {
                    let ASA = A - ARX;
                    let ASB = EA * (ARX + (ASA * (((EE / DZ) * ASA).tanh())));
                    ASB
                };
                let ASD = GX * (C / ((C + (ASC.powf(ANS))).powf(APS)));
                let ASE = APU / ARW;
                let ASJ = if DQ != 0.0 {
                    let ASF = A - ASE;
                    let ASG = EA * (ASE + (((ASF * ASF) + DZ).sqrt()));
                    ASG
                } else {
                    let ASH = A - ASE;
                    let ASI = EA * (ASE + (ASH * (((EE / DZ) * ASH).tanh())));
                    ASI
                };
                let ASK = APU * (C / ((C + (ASJ.powf(ANS))).powf(APS)));
                let ASL = (ANP - AQW) / ANX;
                let ASM = if ASL > ES { 1.0 } else { 0.0 };
                let ASP;
                if ASM != 0.0 {
                    ASP = A;
                } else {
                    let ASN = if ASL < -5e1f64 { 1.0 } else { 0.0 };
                    let ASQ = if ASN != 0.0 {
                        C
                    } else {
                        let ASO = C / (C + (ASL.exp()));
                        ASO
                    };
                    ASP = ASQ;
                }
                let ASR = ((ANW - ASK) - (AOA - (AOY * ASP))) / AQU;
                let ASS = if ASR > ES { 1.0 } else { 0.0 };
                if ASS != 0.0 {
                } else {
                    let AST = if ASR < -5e1f64 { 1.0 } else { 0.0 };
                    if AST != 0.0 {
                    } else {
                    }
                }
                let ASU = (ANW - AQW) / ANX;
                let ASV = if ASU > ES { 1.0 } else { 0.0 };
                let ASY;
                if ASV != 0.0 {
                    ASY = A;
                } else {
                    let ASW = if ASU < -5e1f64 { 1.0 } else { 0.0 };
                    let ASZ = if ASW != 0.0 {
                        C
                    } else {
                        let ASX = C / (C + (ASU.exp()));
                        ASX
                    };
                    ASY = ASZ;
                }
                let ATA = ((ANP - ASD) - (AOA - (AOY * ASY))) / AQU;
                let ATB = if ATA > ES { 1.0 } else { 0.0 };
                if ATB != 0.0 {
                } else {
                    let ATC = if ATA < -5e1f64 { 1.0 } else { 0.0 };
                    if ATC != 0.0 {
                    } else {
                    }
                }
                let ATD = if parameters[107] == C { 1.0 } else { 0.0 };
                if ATD != 0.0 {
                    let ATE = AOA - ((JU * EA) * ANX);
                    let ATF = (ANQ - ATE) / AQU;
                    let ATG = if ATF > ES { 1.0 } else { 0.0 };
                    if ATG != 0.0 {
                    } else {
                        let ATH = if ATF < -5e1f64 { 1.0 } else { 0.0 };
                        if ATH != 0.0 {
                        } else {
                        }
                    }
                    let ATI = (GY - ATE) / AQU;
                    let ATJ = if ATI > ES { 1.0 } else { 0.0 };
                    if ATJ != 0.0 {
                    } else {
                        let ATK = if ATI < -5e1f64 { 1.0 } else { 0.0 };
                        if ATK != 0.0 {
                        } else {
                        }
                    }
                } else {
                }
                let ATL = if parameters[105] == C { 1.0 } else { 0.0 };
                if ATL != 0.0 {
                    let ATM = (ANP - (AOA - ((JU * EA) * ANX))) / AQU;
                    let ATN = if ATM > ES { 1.0 } else { 0.0 };
                    if ATN != 0.0 {
                    } else {
                        let ATO = if ATM < -5e1f64 { 1.0 } else { 0.0 };
                        if ATO != 0.0 {
                        } else {
                        }
                    }
                } else {
                }
            } else {
            }
            if GR != 0.0 {
            } else {
            }
            let ATQ = if ATP > IU { 1.0 } else { 0.0 };
            if ATQ != 0.0 {
                let ATX = if DQ != 0.0 {
                    let ATV = ((HF * HF) + DZ).sqrt();
                    ATV
                } else {
                    let ATW = HF * (((EE / DZ) * HF).tanh());
                    ATW
                };
                let ATY = ATR - HF;
                let ATZ = parameters[143] * AU;
                let AUA = parameters[138] / (JJ * AU);
                let AUB = AUA + (parameters[139] * ATX);
                let AUC = parameters[124] + (parameters[140] * AV);
                let AUD = AY.powf(JB);
                let AUE = if JA != A { 1.0 } else { 0.0 };
                let AUG = if AUE != 0.0 {
                    let AUF = ATX / ((C + ((ATX / JA).powf(ATU))).powf((C / ATU)));
                    AUF
                } else {
                    A
                };
                let AUH = AUC - ((parameters[137] - (AUG * A)) * ATX);
                let AUI = (EX * AUB) * AU;
                let AUJ = BZ * AUI;
                let AUK = (JU * ATZ) / EX;
                let AUL = AUH - AUK;
                let AUQ = if DQ != 0.0 {
                    let AUM = ATR - ATY;
                    let AUN = EA * ((ATR + ATY) + (((AUM * AUM) + DZ).sqrt()));
                    AUN
                } else {
                    let AUO = ATR - ATY;
                    let AUP = EA * ((ATR + ATY) + (AUO * (((EE / DZ) * AUO).tanh())));
                    AUP
                };
                let AUR = (AUQ - AUL) / ATZ;
                let AUS = if AUR > ES { 1.0 } else { 0.0 };
                let AVB;
                if AUS != 0.0 {
                    AVB = A;
                } else {
                    let AUT = if AUR < -5e1f64 { 1.0 } else { 0.0 };
                    let AVC = if AUT != 0.0 {
                        C
                    } else {
                        let AUU = C / (C + (AUR.exp()));
                        AUU
                    };
                    AVB = AVC;
                }
                let AUZ = if DQ != 0.0 {
                    let AUV = ATR - ATY;
                    let AUW = EA * ((ATR + ATY) + (((AUV * AUV) + DZ).sqrt()));
                    AUW
                } else {
                    let AUX = ATR - ATY;
                    let AUY = EA * ((ATR + ATY) + (AUX * (((EE / DZ) * AUX).tanh())));
                    AUY
                };
                let AVA = (JU * AE) * ATZ;
                let AVD = (AUZ - (AUH - (AVA * AVB))) / AUI;
                let AVE = if AVD > ES { 1.0 } else { 0.0 };
                let AVJ;
                if AVE != 0.0 {
                    let AVF = AUJ * AVD;
                    AVJ = AVF;
                } else {
                    let AVG = if AVD < -5e1f64 { 1.0 } else { 0.0 };
                    let AVK = if AVG != 0.0 {
                        let AVH = AUJ * (AVD.exp());
                        AVH
                    } else {
                        let AVI = AUJ * ((C + (AVD.exp())).ln());
                        AVI
                    };
                    AVJ = AVK;
                }
                let AVL = parameters[134] * ((C + (JC * B)) / (C + (JC * Z)));
                let AVM = (((AVL * (C + ((JD * ATX) / ATP))) / (C + ((parameters[141] * AVJ) / BZ))) * ATP) / (ATT / (AUD * (C + ((parameters[142] * AVJ) / BZ))));
                let AVN = (((AVM * ((C + (((EX * AVJ) / BZ) / AVM)).sqrt())) - AVM) * (C - AVB)) + (AUI * AVB);
                let AVO = HF / AVN;
                let AVT = if DQ != 0.0 {
                    let AVP = A - AVO;
                    let AVQ = EA * (AVO + (((AVP * AVP) + DZ).sqrt()));
                    AVQ
                } else {
                    let AVR = A - AVO;
                    let AVS = EA * (AVO + (AVR * (((EE / DZ) * AVR).tanh())));
                    AVS
                };
                let AVU = C / ATU;
                let AVV = HF * (C / ((C + (AVT.powf(ATU))).powf(AVU)));
                let AVW = -HF;
                let AVX = AVW / AVN;
                let AWC = if DQ != 0.0 {
                    let AVY = A - AVX;
                    let AVZ = EA * (AVX + (((AVY * AVY) + DZ).sqrt()));
                    AVZ
                } else {
                    let AWA = A - AVX;
                    let AWB = EA * (AVX + (AWA * (((EE / DZ) * AWA).tanh())));
                    AWB
                };
                let AWD = AVW * (C / ((C + (AWC.powf(ATU))).powf(AVU)));
                let AWE = (ATR - AUL) / ATZ;
                let AWF = if AWE > ES { 1.0 } else { 0.0 };
                let AWI;
                if AWF != 0.0 {
                    AWI = A;
                } else {
                    let AWG = if AWE < -5e1f64 { 1.0 } else { 0.0 };
                    let AWJ = if AWG != 0.0 {
                        C
                    } else {
                        let AWH = C / (C + (AWE.exp()));
                        AWH
                    };
                    AWI = AWJ;
                }
                let AWK = ((ATY - AWD) - (AUH - (AVA * AWI))) / AUI;
                let AWL = if AWK > ES { 1.0 } else { 0.0 };
                if AWL != 0.0 {
                } else {
                    let AWM = if AWK < -5e1f64 { 1.0 } else { 0.0 };
                    if AWM != 0.0 {
                    } else {
                    }
                }
                let AWN = (ATY - AUL) / ATZ;
                let AWO = if AWN > ES { 1.0 } else { 0.0 };
                let AWR;
                if AWO != 0.0 {
                    AWR = A;
                } else {
                    let AWP = if AWN < -5e1f64 { 1.0 } else { 0.0 };
                    let AWS = if AWP != 0.0 {
                        C
                    } else {
                        let AWQ = C / (C + (AWN.exp()));
                        AWQ
                    };
                    AWR = AWS;
                }
                let AWT = ((ATR - AVV) - (AUH - (AVA * AWR))) / AUI;
                let AWU = if AWT > ES { 1.0 } else { 0.0 };
                if AWU != 0.0 {
                } else {
                    let AWV = if AWT < -5e1f64 { 1.0 } else { 0.0 };
                    if AWV != 0.0 {
                    } else {
                    }
                }
                if DQ != 0.0 {
                } else {
                }
                let AWW = (EX * AUA) * AU;
                let AWX = BZ * AWW;
                let AWY = AUC - AUK;
                let AXD = if DQ != 0.0 {
                    let AWZ = ATR - ATY;
                    let AXA = EA * ((ATR + ATY) + (((AWZ * AWZ) + DZ).sqrt()));
                    AXA
                } else {
                    let AXB = ATR - ATY;
                    let AXC = EA * ((ATR + ATY) + (AXB * (((EE / DZ) * AXB).tanh())));
                    AXC
                };
                let AXE = (AXD - AWY) / ATZ;
                let AXF = if AXE > ES { 1.0 } else { 0.0 };
                let AXN;
                if AXF != 0.0 {
                    AXN = A;
                } else {
                    let AXG = if AXE < -5e1f64 { 1.0 } else { 0.0 };
                    let AXO = if AXG != 0.0 {
                        C
                    } else {
                        let AXH = C / (C + (AXE.exp()));
                        AXH
                    };
                    AXN = AXO;
                }
                let AXM = if DQ != 0.0 {
                    let AXI = ATR - ATY;
                    let AXJ = EA * ((ATR + ATY) + (((AXI * AXI) + DZ).sqrt()));
                    AXJ
                } else {
                    let AXK = ATR - ATY;
                    let AXL = EA * ((ATR + ATY) + (AXK * (((EE / DZ) * AXK).tanh())));
                    AXL
                };
                let AXP = (AXM - (AUC - (AVA * AXN))) / AWW;
                let AXQ = if AXP > ES { 1.0 } else { 0.0 };
                let AXW;
                if AXQ != 0.0 {
                    let AXR = AWX * AXP;
                    AXW = AXR;
                } else {
                    let AXS = if AXP < -5e1f64 { 1.0 } else { 0.0 };
                    let AXX = if AXS != 0.0 {
                        let AXT = AWX * (AXP.exp());
                        AXT
                    } else {
                        let AXU = AWX * ((C + (AXP.exp())).ln());
                        AXU
                    };
                    AXW = AXX;
                }
                let AXV = (AVL * ATP) / (ATT / AUD);
                let AXY = (((AXV * ((C + (((EX * AXW) / BZ) / AXV)).sqrt())) - AXV) * (C - AXN)) + (AWW * AXN);
                let AXZ = HF / AXY;
                let AYE = if DQ != 0.0 {
                    let AYA = A - AXZ;
                    let AYB = EA * (AXZ + (((AYA * AYA) + DZ).sqrt()));
                    AYB
                } else {
                    let AYC = A - AXZ;
                    let AYD = EA * (AXZ + (AYC * (((EE / DZ) * AYC).tanh())));
                    AYD
                };
                let AYF = HF * (C / ((C + (AYE.powf(ATU))).powf(AVU)));
                let AYG = AVW / AXY;
                let AYL = if DQ != 0.0 {
                    let AYH = A - AYG;
                    let AYI = EA * (AYG + (((AYH * AYH) + DZ).sqrt()));
                    AYI
                } else {
                    let AYJ = A - AYG;
                    let AYK = EA * (AYG + (AYJ * (((EE / DZ) * AYJ).tanh())));
                    AYK
                };
                let AYM = AVW * (C / ((C + (AYL.powf(ATU))).powf(AVU)));
                let AYN = (ATR - AWY) / ATZ;
                let AYO = if AYN > ES { 1.0 } else { 0.0 };
                let AYR;
                if AYO != 0.0 {
                    AYR = A;
                } else {
                    let AYP = if AYN < -5e1f64 { 1.0 } else { 0.0 };
                    let AYS = if AYP != 0.0 {
                        C
                    } else {
                        let AYQ = C / (C + (AYN.exp()));
                        AYQ
                    };
                    AYR = AYS;
                }
                let AYT = ((ATY - AYM) - (AUC - (AVA * AYR))) / AWW;
                let AYU = if AYT > ES { 1.0 } else { 0.0 };
                if AYU != 0.0 {
                } else {
                    let AYV = if AYT < -5e1f64 { 1.0 } else { 0.0 };
                    if AYV != 0.0 {
                    } else {
                    }
                }
                let AYW = (ATY - AWY) / ATZ;
                let AYX = if AYW > ES { 1.0 } else { 0.0 };
                let AZA;
                if AYX != 0.0 {
                    AZA = A;
                } else {
                    let AYY = if AYW < -5e1f64 { 1.0 } else { 0.0 };
                    let AZB = if AYY != 0.0 {
                        C
                    } else {
                        let AYZ = C / (C + (AYW.exp()));
                        AYZ
                    };
                    AZA = AZB;
                }
                let AZC = ((ATR - AYF) - (AUC - (AVA * AZA))) / AWW;
                let AZD = if AZC > ES { 1.0 } else { 0.0 };
                if AZD != 0.0 {
                } else {
                    let AZE = if AZC < -5e1f64 { 1.0 } else { 0.0 };
                    if AZE != 0.0 {
                    } else {
                    }
                }
                let AZF = if parameters[129] == C { 1.0 } else { 0.0 };
                if AZF != 0.0 {
                    let AZG = AUC - ((JU * EA) * ATZ);
                    let AZH = (ATS - AZG) / AWW;
                    let AZI = if AZH > ES { 1.0 } else { 0.0 };
                    if AZI != 0.0 {
                    } else {
                        let AZJ = if AZH < -5e1f64 { 1.0 } else { 0.0 };
                        if AZJ != 0.0 {
                        } else {
                        }
                    }
                    let AZK = (HG - AZG) / AWW;
                    let AZL = if AZK > ES { 1.0 } else { 0.0 };
                    if AZL != 0.0 {
                    } else {
                        let AZM = if AZK < -5e1f64 { 1.0 } else { 0.0 };
                        if AZM != 0.0 {
                        } else {
                        }
                    }
                } else {
                }
                let AZN = if parameters[127] == C { 1.0 } else { 0.0 };
                if AZN != 0.0 {
                    let AZO = (ATR - (AUC - ((JU * EA) * ATZ))) / AWW;
                    let AZP = if AZO > ES { 1.0 } else { 0.0 };
                    if AZP != 0.0 {
                    } else {
                        let AZQ = if AZO < -5e1f64 { 1.0 } else { 0.0 };
                        if AZQ != 0.0 {
                        } else {
                        }
                    }
                } else {
                }
            } else {
            }
            if GZ != 0.0 {
            } else {
            }
            let AZS = if AZR > IU { 1.0 } else { 0.0 };
            if AZS != 0.0 {
                let AZZ = if DQ != 0.0 {
                    let AZX = ((HM * HM) + DZ).sqrt();
                    AZX
                } else {
                    let AZY = HM * (((EE / DZ) * HM).tanh());
                    AZY
                };
                let BAA = AZT - HM;
                let BAB = parameters[165] * AU;
                let BAC = parameters[160] / (JJ * AU);
                let BAD = BAC + (parameters[161] * AZZ);
                let BAE = parameters[146] + (parameters[162] * AV);
                let BAF = AY.powf(JB);
                let BAG = if JA != A { 1.0 } else { 0.0 };
                let BAI = if BAG != 0.0 {
                    let BAH = AZZ / ((C + ((AZZ / JA).powf(AZW))).powf((C / AZW)));
                    BAH
                } else {
                    A
                };
                let BAJ = BAE - ((parameters[159] - (BAI * A)) * AZZ);
                let BAK = (EX * BAD) * AU;
                let BAL = CD * BAK;
                let BAM = (JU * BAB) / EX;
                let BAN = BAJ - BAM;
                let BAS = if DQ != 0.0 {
                    let BAO = AZT - BAA;
                    let BAP = EA * ((AZT + BAA) + (((BAO * BAO) + DZ).sqrt()));
                    BAP
                } else {
                    let BAQ = AZT - BAA;
                    let BAR = EA * ((AZT + BAA) + (BAQ * (((EE / DZ) * BAQ).tanh())));
                    BAR
                };
                let BAT = (BAS - BAN) / BAB;
                let BAU = if BAT > ES { 1.0 } else { 0.0 };
                let BBD;
                if BAU != 0.0 {
                    BBD = A;
                } else {
                    let BAV = if BAT < -5e1f64 { 1.0 } else { 0.0 };
                    let BBE = if BAV != 0.0 {
                        C
                    } else {
                        let BAW = C / (C + (BAT.exp()));
                        BAW
                    };
                    BBD = BBE;
                }
                let BBB = if DQ != 0.0 {
                    let BAX = AZT - BAA;
                    let BAY = EA * ((AZT + BAA) + (((BAX * BAX) + DZ).sqrt()));
                    BAY
                } else {
                    let BAZ = AZT - BAA;
                    let BBA = EA * ((AZT + BAA) + (BAZ * (((EE / DZ) * BAZ).tanh())));
                    BBA
                };
                let BBC = (JU * AE) * BAB;
                let BBF = (BBB - (BAJ - (BBC * BBD))) / BAK;
                let BBG = if BBF > ES { 1.0 } else { 0.0 };
                let BBL;
                if BBG != 0.0 {
                    let BBH = BAL * BBF;
                    BBL = BBH;
                } else {
                    let BBI = if BBF < -5e1f64 { 1.0 } else { 0.0 };
                    let BBM = if BBI != 0.0 {
                        let BBJ = BAL * (BBF.exp());
                        BBJ
                    } else {
                        let BBK = BAL * ((C + (BBF.exp())).ln());
                        BBK
                    };
                    BBL = BBM;
                }
                let BBN = parameters[156] * ((C + (JC * B)) / (C + (JC * Z)));
                let BBO = (((BBN * (C + ((JD * AZZ) / AZR))) / (C + ((parameters[163] * BBL) / CD))) * AZR) / (AZV / (BAF * (C + ((parameters[164] * BBL) / CD))));
                let BBP = (((BBO * ((C + (((EX * BBL) / CD) / BBO)).sqrt())) - BBO) * (C - BBD)) + (BAK * BBD);
                let BBQ = HM / BBP;
                let BBV = if DQ != 0.0 {
                    let BBR = A - BBQ;
                    let BBS = EA * (BBQ + (((BBR * BBR) + DZ).sqrt()));
                    BBS
                } else {
                    let BBT = A - BBQ;
                    let BBU = EA * (BBQ + (BBT * (((EE / DZ) * BBT).tanh())));
                    BBU
                };
                let BBW = C / AZW;
                let BBX = HM * (C / ((C + (BBV.powf(AZW))).powf(BBW)));
                let BBY = -HM;
                let BBZ = BBY / BBP;
                let BCE = if DQ != 0.0 {
                    let BCA = A - BBZ;
                    let BCB = EA * (BBZ + (((BCA * BCA) + DZ).sqrt()));
                    BCB
                } else {
                    let BCC = A - BBZ;
                    let BCD = EA * (BBZ + (BCC * (((EE / DZ) * BCC).tanh())));
                    BCD
                };
                let BCF = BBY * (C / ((C + (BCE.powf(AZW))).powf(BBW)));
                let BCG = (AZT - BAN) / BAB;
                let BCH = if BCG > ES { 1.0 } else { 0.0 };
                let BCK;
                if BCH != 0.0 {
                    BCK = A;
                } else {
                    let BCI = if BCG < -5e1f64 { 1.0 } else { 0.0 };
                    let BCL = if BCI != 0.0 {
                        C
                    } else {
                        let BCJ = C / (C + (BCG.exp()));
                        BCJ
                    };
                    BCK = BCL;
                }
                let BCM = ((BAA - BCF) - (BAJ - (BBC * BCK))) / BAK;
                let BCN = if BCM > ES { 1.0 } else { 0.0 };
                if BCN != 0.0 {
                } else {
                    let BCO = if BCM < -5e1f64 { 1.0 } else { 0.0 };
                    if BCO != 0.0 {
                    } else {
                    }
                }
                let BCP = (BAA - BAN) / BAB;
                let BCQ = if BCP > ES { 1.0 } else { 0.0 };
                let BCT;
                if BCQ != 0.0 {
                    BCT = A;
                } else {
                    let BCR = if BCP < -5e1f64 { 1.0 } else { 0.0 };
                    let BCU = if BCR != 0.0 {
                        C
                    } else {
                        let BCS = C / (C + (BCP.exp()));
                        BCS
                    };
                    BCT = BCU;
                }
                let BCV = ((AZT - BBX) - (BAJ - (BBC * BCT))) / BAK;
                let BCW = if BCV > ES { 1.0 } else { 0.0 };
                if BCW != 0.0 {
                } else {
                    let BCX = if BCV < -5e1f64 { 1.0 } else { 0.0 };
                    if BCX != 0.0 {
                    } else {
                    }
                }
                if DQ != 0.0 {
                } else {
                }
                let BCY = (EX * BAC) * AU;
                let BCZ = CD * BCY;
                let BDA = BAE - BAM;
                let BDF = if DQ != 0.0 {
                    let BDB = AZT - BAA;
                    let BDC = EA * ((AZT + BAA) + (((BDB * BDB) + DZ).sqrt()));
                    BDC
                } else {
                    let BDD = AZT - BAA;
                    let BDE = EA * ((AZT + BAA) + (BDD * (((EE / DZ) * BDD).tanh())));
                    BDE
                };
                let BDG = (BDF - BDA) / BAB;
                let BDH = if BDG > ES { 1.0 } else { 0.0 };
                let BDP;
                if BDH != 0.0 {
                    BDP = A;
                } else {
                    let BDI = if BDG < -5e1f64 { 1.0 } else { 0.0 };
                    let BDQ = if BDI != 0.0 {
                        C
                    } else {
                        let BDJ = C / (C + (BDG.exp()));
                        BDJ
                    };
                    BDP = BDQ;
                }
                let BDO = if DQ != 0.0 {
                    let BDK = AZT - BAA;
                    let BDL = EA * ((AZT + BAA) + (((BDK * BDK) + DZ).sqrt()));
                    BDL
                } else {
                    let BDM = AZT - BAA;
                    let BDN = EA * ((AZT + BAA) + (BDM * (((EE / DZ) * BDM).tanh())));
                    BDN
                };
                let BDR = (BDO - (BAE - (BBC * BDP))) / BCY;
                let BDS = if BDR > ES { 1.0 } else { 0.0 };
                let BDY;
                if BDS != 0.0 {
                    let BDT = BCZ * BDR;
                    BDY = BDT;
                } else {
                    let BDU = if BDR < -5e1f64 { 1.0 } else { 0.0 };
                    let BDZ = if BDU != 0.0 {
                        let BDV = BCZ * (BDR.exp());
                        BDV
                    } else {
                        let BDW = BCZ * ((C + (BDR.exp())).ln());
                        BDW
                    };
                    BDY = BDZ;
                }
                let BDX = (BBN * AZR) / (AZV / BAF);
                let BEA = (((BDX * ((C + (((EX * BDY) / CD) / BDX)).sqrt())) - BDX) * (C - BDP)) + (BCY * BDP);
                let BEB = HM / BEA;
                let BEG = if DQ != 0.0 {
                    let BEC = A - BEB;
                    let BED = EA * (BEB + (((BEC * BEC) + DZ).sqrt()));
                    BED
                } else {
                    let BEE = A - BEB;
                    let BEF = EA * (BEB + (BEE * (((EE / DZ) * BEE).tanh())));
                    BEF
                };
                let BEH = HM * (C / ((C + (BEG.powf(AZW))).powf(BBW)));
                let BEI = BBY / BEA;
                let BEN = if DQ != 0.0 {
                    let BEJ = A - BEI;
                    let BEK = EA * (BEI + (((BEJ * BEJ) + DZ).sqrt()));
                    BEK
                } else {
                    let BEL = A - BEI;
                    let BEM = EA * (BEI + (BEL * (((EE / DZ) * BEL).tanh())));
                    BEM
                };
                let BEO = BBY * (C / ((C + (BEN.powf(AZW))).powf(BBW)));
                let BEP = (AZT - BDA) / BAB;
                let BEQ = if BEP > ES { 1.0 } else { 0.0 };
                let BET;
                if BEQ != 0.0 {
                    BET = A;
                } else {
                    let BER = if BEP < -5e1f64 { 1.0 } else { 0.0 };
                    let BEU = if BER != 0.0 {
                        C
                    } else {
                        let BES = C / (C + (BEP.exp()));
                        BES
                    };
                    BET = BEU;
                }
                let BEV = ((BAA - BEO) - (BAE - (BBC * BET))) / BCY;
                let BEW = if BEV > ES { 1.0 } else { 0.0 };
                if BEW != 0.0 {
                } else {
                    let BEX = if BEV < -5e1f64 { 1.0 } else { 0.0 };
                    if BEX != 0.0 {
                    } else {
                    }
                }
                let BEY = (BAA - BDA) / BAB;
                let BEZ = if BEY > ES { 1.0 } else { 0.0 };
                let BFC;
                if BEZ != 0.0 {
                    BFC = A;
                } else {
                    let BFA = if BEY < -5e1f64 { 1.0 } else { 0.0 };
                    let BFD = if BFA != 0.0 {
                        C
                    } else {
                        let BFB = C / (C + (BEY.exp()));
                        BFB
                    };
                    BFC = BFD;
                }
                let BFE = ((AZT - BEH) - (BAE - (BBC * BFC))) / BCY;
                let BFF = if BFE > ES { 1.0 } else { 0.0 };
                if BFF != 0.0 {
                } else {
                    let BFG = if BFE < -5e1f64 { 1.0 } else { 0.0 };
                    if BFG != 0.0 {
                    } else {
                    }
                }
                let BFH = if parameters[151] == C { 1.0 } else { 0.0 };
                if BFH != 0.0 {
                    let BFI = BAE - ((JU * EA) * BAB);
                    let BFJ = (AZU - BFI) / BCY;
                    let BFK = if BFJ > ES { 1.0 } else { 0.0 };
                    if BFK != 0.0 {
                    } else {
                        let BFL = if BFJ < -5e1f64 { 1.0 } else { 0.0 };
                        if BFL != 0.0 {
                        } else {
                        }
                    }
                    let BFM = (HN - BFI) / BCY;
                    let BFN = if BFM > ES { 1.0 } else { 0.0 };
                    if BFN != 0.0 {
                    } else {
                        let BFO = if BFM < -5e1f64 { 1.0 } else { 0.0 };
                        if BFO != 0.0 {
                        } else {
                        }
                    }
                } else {
                }
                let BFP = if parameters[149] == C { 1.0 } else { 0.0 };
                if BFP != 0.0 {
                    let BFQ = (AZT - (BAE - ((JU * EA) * BAB))) / BCY;
                    let BFR = if BFQ > ES { 1.0 } else { 0.0 };
                    if BFR != 0.0 {
                    } else {
                        let BFS = if BFQ < -5e1f64 { 1.0 } else { 0.0 };
                        if BFS != 0.0 {
                        } else {
                        }
                    }
                } else {
                }
            } else {
            }
            if HH != 0.0 {
            } else {
            }
            let BFT = if J != 0.0 && (if R > IU { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            if BFT != 0.0 {
                let BFZ = if DQ != 0.0 {
                    let BFX = ((EL * EL) + DZ).sqrt();
                    BFX
                } else {
                    let BFY = EL * (((EE / DZ) * EL).tanh());
                    BFY
                };
                let BGA = EO - EL;
                let BGB = parameters[65] * AU;
                let BGC = parameters[61] / (JJ * AU);
                let BGD = BGC + (parameters[62] * BFZ);
                let BGE = EH + (BFW * AV);
                let BGF = AY.powf(JB);
                let BGG = if JA != A { 1.0 } else { 0.0 };
                let BGI = if BGG != 0.0 {
                    let BGH = BFZ / ((C + ((BFZ / JA).powf(BFV))).powf((C / BFV)));
                    BGH
                } else {
                    A
                };
                let BGJ = BGE - ((parameters[60] - (BGI * A)) * BFZ);
                let BGK = (EX * BGD) * AU;
                let BGL = EI * BGK;
                let BGM = (JU * BGB) / EX;
                let BGN = BGJ - BGM;
                let BGS = if DQ != 0.0 {
                    let BGO = EO - BGA;
                    let BGP = EA * ((EO + BGA) + (((BGO * BGO) + DZ).sqrt()));
                    BGP
                } else {
                    let BGQ = EO - BGA;
                    let BGR = EA * ((EO + BGA) + (BGQ * (((EE / DZ) * BGQ).tanh())));
                    BGR
                };
                let BGT = (BGS - BGN) / BGB;
                let BGU = if BGT > ES { 1.0 } else { 0.0 };
                let BHD;
                if BGU != 0.0 {
                    BHD = A;
                } else {
                    let BGV = if BGT < -5e1f64 { 1.0 } else { 0.0 };
                    let BHE = if BGV != 0.0 {
                        C
                    } else {
                        let BGW = C / (C + (BGT.exp()));
                        BGW
                    };
                    BHD = BHE;
                }
                let BHB = if DQ != 0.0 {
                    let BGX = EO - BGA;
                    let BGY = EA * ((EO + BGA) + (((BGX * BGX) + DZ).sqrt()));
                    BGY
                } else {
                    let BGZ = EO - BGA;
                    let BHA = EA * ((EO + BGA) + (BGZ * (((EE / DZ) * BGZ).tanh())));
                    BHA
                };
                let BHC = (JU * AE) * BGB;
                let BHF = (BHB - (BGJ - (BHC * BHD))) / BGK;
                let BHG = if BHF > ES { 1.0 } else { 0.0 };
                let BHL;
                if BHG != 0.0 {
                    let BHH = BGL * BHF;
                    BHL = BHH;
                } else {
                    let BHI = if BHF < -5e1f64 { 1.0 } else { 0.0 };
                    let BHM = if BHI != 0.0 {
                        let BHJ = BGL * (BHF.exp());
                        BHJ
                    } else {
                        let BHK = BGL * ((C + (BHF.exp())).ln());
                        BHK
                    };
                    BHL = BHM;
                }
                let BHN = parameters[57] * ((C + (JC * B)) / (C + (JC * Z)));
                let BHO = (((BHN * (C + ((JD * BFZ) / R))) / (C + ((parameters[63] * BHL) / EI))) * R) / (BFU / (BGF * (C + ((parameters[64] * BHL) / EI))));
                let BHP = (((BHO * ((C + (((EX * BHL) / EI) / BHO)).sqrt())) - BHO) * (C - BHD)) + (BGK * BHD);
                let BHQ = EL / BHP;
                let BHV = if DQ != 0.0 {
                    let BHR = A - BHQ;
                    let BHS = EA * (BHQ + (((BHR * BHR) + DZ).sqrt()));
                    BHS
                } else {
                    let BHT = A - BHQ;
                    let BHU = EA * (BHQ + (BHT * (((EE / DZ) * BHT).tanh())));
                    BHU
                };
                let BHW = C / BFV;
                let BHX = EL * (C / ((C + (BHV.powf(BFV))).powf(BHW)));
                let BHY = -EL;
                let BHZ = BHY / BHP;
                let BIE = if DQ != 0.0 {
                    let BIA = A - BHZ;
                    let BIB = EA * (BHZ + (((BIA * BIA) + DZ).sqrt()));
                    BIB
                } else {
                    let BIC = A - BHZ;
                    let BID = EA * (BHZ + (BIC * (((EE / DZ) * BIC).tanh())));
                    BID
                };
                let BIF = BHY * (C / ((C + (BIE.powf(BFV))).powf(BHW)));
                let BIG = (EO - BGN) / BGB;
                let BIH = if BIG > ES { 1.0 } else { 0.0 };
                let BIK;
                if BIH != 0.0 {
                    BIK = A;
                } else {
                    let BII = if BIG < -5e1f64 { 1.0 } else { 0.0 };
                    let BIL = if BII != 0.0 {
                        C
                    } else {
                        let BIJ = C / (C + (BIG.exp()));
                        BIJ
                    };
                    BIK = BIL;
                }
                let BIM = ((BGA - BIF) - (BGJ - (BHC * BIK))) / BGK;
                let BIN = if BIM > ES { 1.0 } else { 0.0 };
                if BIN != 0.0 {
                } else {
                    let BIO = if BIM < -5e1f64 { 1.0 } else { 0.0 };
                    if BIO != 0.0 {
                    } else {
                    }
                }
                let BIP = (BGA - BGN) / BGB;
                let BIQ = if BIP > ES { 1.0 } else { 0.0 };
                let BIT;
                if BIQ != 0.0 {
                    BIT = A;
                } else {
                    let BIR = if BIP < -5e1f64 { 1.0 } else { 0.0 };
                    let BIU = if BIR != 0.0 {
                        C
                    } else {
                        let BIS = C / (C + (BIP.exp()));
                        BIS
                    };
                    BIT = BIU;
                }
                let BIV = ((EO - BHX) - (BGJ - (BHC * BIT))) / BGK;
                let BIW = if BIV > ES { 1.0 } else { 0.0 };
                if BIW != 0.0 {
                } else {
                    let BIX = if BIV < -5e1f64 { 1.0 } else { 0.0 };
                    if BIX != 0.0 {
                    } else {
                    }
                }
                if DQ != 0.0 {
                } else {
                }
                let BIY = (EX * BGC) * AU;
                let BIZ = EI * BIY;
                let BJA = BGE - BGM;
                let BJF = if DQ != 0.0 {
                    let BJB = EO - BGA;
                    let BJC = EA * ((EO + BGA) + (((BJB * BJB) + DZ).sqrt()));
                    BJC
                } else {
                    let BJD = EO - BGA;
                    let BJE = EA * ((EO + BGA) + (BJD * (((EE / DZ) * BJD).tanh())));
                    BJE
                };
                let BJG = (BJF - BJA) / BGB;
                let BJH = if BJG > ES { 1.0 } else { 0.0 };
                let BJP;
                if BJH != 0.0 {
                    BJP = A;
                } else {
                    let BJI = if BJG < -5e1f64 { 1.0 } else { 0.0 };
                    let BJQ = if BJI != 0.0 {
                        C
                    } else {
                        let BJJ = C / (C + (BJG.exp()));
                        BJJ
                    };
                    BJP = BJQ;
                }
                let BJO = if DQ != 0.0 {
                    let BJK = EO - BGA;
                    let BJL = EA * ((EO + BGA) + (((BJK * BJK) + DZ).sqrt()));
                    BJL
                } else {
                    let BJM = EO - BGA;
                    let BJN = EA * ((EO + BGA) + (BJM * (((EE / DZ) * BJM).tanh())));
                    BJN
                };
                let BJR = (BJO - (BGE - (BHC * BJP))) / BIY;
                let BJS = if BJR > ES { 1.0 } else { 0.0 };
                let BJY;
                if BJS != 0.0 {
                    let BJT = BIZ * BJR;
                    BJY = BJT;
                } else {
                    let BJU = if BJR < -5e1f64 { 1.0 } else { 0.0 };
                    let BJZ = if BJU != 0.0 {
                        let BJV = BIZ * (BJR.exp());
                        BJV
                    } else {
                        let BJW = BIZ * ((C + (BJR.exp())).ln());
                        BJW
                    };
                    BJY = BJZ;
                }
                let BJX = (BHN * R) / (BFU / BGF);
                let BKA = (((BJX * ((C + (((EX * BJY) / EI) / BJX)).sqrt())) - BJX) * (C - BJP)) + (BIY * BJP);
                let BKB = EL / BKA;
                let BKG = if DQ != 0.0 {
                    let BKC = A - BKB;
                    let BKD = EA * (BKB + (((BKC * BKC) + DZ).sqrt()));
                    BKD
                } else {
                    let BKE = A - BKB;
                    let BKF = EA * (BKB + (BKE * (((EE / DZ) * BKE).tanh())));
                    BKF
                };
                let BKH = EL * (C / ((C + (BKG.powf(BFV))).powf(BHW)));
                let BKI = BHY / BKA;
                let BKN = if DQ != 0.0 {
                    let BKJ = A - BKI;
                    let BKK = EA * (BKI + (((BKJ * BKJ) + DZ).sqrt()));
                    BKK
                } else {
                    let BKL = A - BKI;
                    let BKM = EA * (BKI + (BKL * (((EE / DZ) * BKL).tanh())));
                    BKM
                };
                let BKO = BHY * (C / ((C + (BKN.powf(BFV))).powf(BHW)));
                let BKP = (EO - BJA) / BGB;
                let BKQ = if BKP > ES { 1.0 } else { 0.0 };
                let BKT;
                if BKQ != 0.0 {
                    BKT = A;
                } else {
                    let BKR = if BKP < -5e1f64 { 1.0 } else { 0.0 };
                    let BKU = if BKR != 0.0 {
                        C
                    } else {
                        let BKS = C / (C + (BKP.exp()));
                        BKS
                    };
                    BKT = BKU;
                }
                let BKV = ((BGA - BKO) - (BGE - (BHC * BKT))) / BIY;
                let BKW = if BKV > ES { 1.0 } else { 0.0 };
                if BKW != 0.0 {
                } else {
                    let BKX = if BKV < -5e1f64 { 1.0 } else { 0.0 };
                    if BKX != 0.0 {
                    } else {
                    }
                }
                let BKY = (BGA - BJA) / BGB;
                let BKZ = if BKY > ES { 1.0 } else { 0.0 };
                let BLC;
                if BKZ != 0.0 {
                    BLC = A;
                } else {
                    let BLA = if BKY < -5e1f64 { 1.0 } else { 0.0 };
                    let BLD = if BLA != 0.0 {
                        C
                    } else {
                        let BLB = C / (C + (BKY.exp()));
                        BLB
                    };
                    BLC = BLD;
                }
                let BLE = ((EO - BKH) - (BGE - (BHC * BLC))) / BIY;
                let BLF = if BLE > ES { 1.0 } else { 0.0 };
                if BLF != 0.0 {
                } else {
                    let BLG = if BLE < -5e1f64 { 1.0 } else { 0.0 };
                    if BLG != 0.0 {
                    } else {
                    }
                }
                if BLH != 0.0 {
                    let BLI = (A - (BGE - ((JU * EA) * BGB))) / BIY;
                    let BLJ = if BLI > ES { 1.0 } else { 0.0 };
                    if BLJ != 0.0 {
                    } else {
                        let BLK = if BLI < -5e1f64 { 1.0 } else { 0.0 };
                        if BLK != 0.0 {
                        } else {
                        }
                    }
                    if BLJ != 0.0 {
                    } else {
                        let BLL = if BLI < -5e1f64 { 1.0 } else { 0.0 };
                        if BLL != 0.0 {
                        } else {
                        }
                    }
                } else {
                }
                if BLM != 0.0 {
                    let BLN = (EO - (BGE - ((JU * EA) * BGB))) / BIY;
                    let BLO = if BLN > ES { 1.0 } else { 0.0 };
                    if BLO != 0.0 {
                    } else {
                        let BLP = if BLN < -5e1f64 { 1.0 } else { 0.0 };
                        if BLP != 0.0 {
                        } else {
                        }
                    }
                } else {
                }
            } else {
            }
            let BLQ = if J != 0.0 && (if T > IU { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            if BLQ != 0.0 {
                let BLV = if DQ != 0.0 {
                    let BLT = ((GD * GD) + DZ).sqrt();
                    BLT
                } else {
                    let BLU = GD * (((EE / DZ) * GD).tanh());
                    BLU
                };
                let BLW = GG - GD;
                let BLX = parameters[77] * AU;
                let BLY = parameters[73] / (JJ * AU);
                let BLZ = BLY + (parameters[74] * BLV);
                let BMA = FZ + (BFW * AV);
                let BMB = AY.powf(JB);
                let BMC = if JA != A { 1.0 } else { 0.0 };
                let BME = if BMC != 0.0 {
                    let BMD = BLV / ((C + ((BLV / JA).powf(BLS))).powf((C / BLS)));
                    BMD
                } else {
                    A
                };
                let BMF = BMA - ((parameters[72] - (BME * A)) * BLV);
                let BMG = (EX * BLZ) * AU;
                let BMH = GB * BMG;
                let BMI = (JU * BLX) / EX;
                let BMJ = BMF - BMI;
                let BMO = if DQ != 0.0 {
                    let BMK = GG - BLW;
                    let BML = EA * ((GG + BLW) + (((BMK * BMK) + DZ).sqrt()));
                    BML
                } else {
                    let BMM = GG - BLW;
                    let BMN = EA * ((GG + BLW) + (BMM * (((EE / DZ) * BMM).tanh())));
                    BMN
                };
                let BMP = (BMO - BMJ) / BLX;
                let BMQ = if BMP > ES { 1.0 } else { 0.0 };
                let BMZ;
                if BMQ != 0.0 {
                    BMZ = A;
                } else {
                    let BMR = if BMP < -5e1f64 { 1.0 } else { 0.0 };
                    let BNA = if BMR != 0.0 {
                        C
                    } else {
                        let BMS = C / (C + (BMP.exp()));
                        BMS
                    };
                    BMZ = BNA;
                }
                let BMX = if DQ != 0.0 {
                    let BMT = GG - BLW;
                    let BMU = EA * ((GG + BLW) + (((BMT * BMT) + DZ).sqrt()));
                    BMU
                } else {
                    let BMV = GG - BLW;
                    let BMW = EA * ((GG + BLW) + (BMV * (((EE / DZ) * BMV).tanh())));
                    BMW
                };
                let BMY = (JU * AE) * BLX;
                let BNB = (BMX - (BMF - (BMY * BMZ))) / BMG;
                let BNC = if BNB > ES { 1.0 } else { 0.0 };
                let BNH;
                if BNC != 0.0 {
                    let BND = BMH * BNB;
                    BNH = BND;
                } else {
                    let BNE = if BNB < -5e1f64 { 1.0 } else { 0.0 };
                    let BNI = if BNE != 0.0 {
                        let BNF = BMH * (BNB.exp());
                        BNF
                    } else {
                        let BNG = BMH * ((C + (BNB.exp())).ln());
                        BNG
                    };
                    BNH = BNI;
                }
                let BNJ = parameters[69] * ((C + (JC * B)) / (C + (JC * Z)));
                let BNK = (((BNJ * (C + ((JD * BLV) / T))) / (C + ((parameters[75] * BNH) / GB))) * T) / (BLR / (BMB * (C + ((parameters[76] * BNH) / GB))));
                let BNL = (((BNK * ((C + (((EX * BNH) / GB) / BNK)).sqrt())) - BNK) * (C - BMZ)) + (BMG * BMZ);
                let BNM = GD / BNL;
                let BNR = if DQ != 0.0 {
                    let BNN = A - BNM;
                    let BNO = EA * (BNM + (((BNN * BNN) + DZ).sqrt()));
                    BNO
                } else {
                    let BNP = A - BNM;
                    let BNQ = EA * (BNM + (BNP * (((EE / DZ) * BNP).tanh())));
                    BNQ
                };
                let BNS = C / BLS;
                let BNT = GD * (C / ((C + (BNR.powf(BLS))).powf(BNS)));
                let BNU = -GD;
                let BNV = BNU / BNL;
                let BOA = if DQ != 0.0 {
                    let BNW = A - BNV;
                    let BNX = EA * (BNV + (((BNW * BNW) + DZ).sqrt()));
                    BNX
                } else {
                    let BNY = A - BNV;
                    let BNZ = EA * (BNV + (BNY * (((EE / DZ) * BNY).tanh())));
                    BNZ
                };
                let BOB = BNU * (C / ((C + (BOA.powf(BLS))).powf(BNS)));
                let BOC = (GG - BMJ) / BLX;
                let BOD = if BOC > ES { 1.0 } else { 0.0 };
                let BOG;
                if BOD != 0.0 {
                    BOG = A;
                } else {
                    let BOE = if BOC < -5e1f64 { 1.0 } else { 0.0 };
                    let BOH = if BOE != 0.0 {
                        C
                    } else {
                        let BOF = C / (C + (BOC.exp()));
                        BOF
                    };
                    BOG = BOH;
                }
                let BOI = ((BLW - BOB) - (BMF - (BMY * BOG))) / BMG;
                let BOJ = if BOI > ES { 1.0 } else { 0.0 };
                if BOJ != 0.0 {
                } else {
                    let BOK = if BOI < -5e1f64 { 1.0 } else { 0.0 };
                    if BOK != 0.0 {
                    } else {
                    }
                }
                let BOL = (BLW - BMJ) / BLX;
                let BOM = if BOL > ES { 1.0 } else { 0.0 };
                let BOP;
                if BOM != 0.0 {
                    BOP = A;
                } else {
                    let BON = if BOL < -5e1f64 { 1.0 } else { 0.0 };
                    let BOQ = if BON != 0.0 {
                        C
                    } else {
                        let BOO = C / (C + (BOL.exp()));
                        BOO
                    };
                    BOP = BOQ;
                }
                let BOR = ((GG - BNT) - (BMF - (BMY * BOP))) / BMG;
                let BOS = if BOR > ES { 1.0 } else { 0.0 };
                if BOS != 0.0 {
                } else {
                    let BOT = if BOR < -5e1f64 { 1.0 } else { 0.0 };
                    if BOT != 0.0 {
                    } else {
                    }
                }
                if DQ != 0.0 {
                } else {
                }
                let BOU = (EX * BLY) * AU;
                let BOV = GB * BOU;
                let BOW = BMA - BMI;
                let BPB = if DQ != 0.0 {
                    let BOX = GG - BLW;
                    let BOY = EA * ((GG + BLW) + (((BOX * BOX) + DZ).sqrt()));
                    BOY
                } else {
                    let BOZ = GG - BLW;
                    let BPA = EA * ((GG + BLW) + (BOZ * (((EE / DZ) * BOZ).tanh())));
                    BPA
                };
                let BPC = (BPB - BOW) / BLX;
                let BPD = if BPC > ES { 1.0 } else { 0.0 };
                let BPL;
                if BPD != 0.0 {
                    BPL = A;
                } else {
                    let BPE = if BPC < -5e1f64 { 1.0 } else { 0.0 };
                    let BPM = if BPE != 0.0 {
                        C
                    } else {
                        let BPF = C / (C + (BPC.exp()));
                        BPF
                    };
                    BPL = BPM;
                }
                let BPK = if DQ != 0.0 {
                    let BPG = GG - BLW;
                    let BPH = EA * ((GG + BLW) + (((BPG * BPG) + DZ).sqrt()));
                    BPH
                } else {
                    let BPI = GG - BLW;
                    let BPJ = EA * ((GG + BLW) + (BPI * (((EE / DZ) * BPI).tanh())));
                    BPJ
                };
                let BPN = (BPK - (BMA - (BMY * BPL))) / BOU;
                let BPO = if BPN > ES { 1.0 } else { 0.0 };
                let BPU;
                if BPO != 0.0 {
                    let BPP = BOV * BPN;
                    BPU = BPP;
                } else {
                    let BPQ = if BPN < -5e1f64 { 1.0 } else { 0.0 };
                    let BPV = if BPQ != 0.0 {
                        let BPR = BOV * (BPN.exp());
                        BPR
                    } else {
                        let BPS = BOV * ((C + (BPN.exp())).ln());
                        BPS
                    };
                    BPU = BPV;
                }
                let BPT = (BNJ * T) / (BLR / BMB);
                let BPW = (((BPT * ((C + (((EX * BPU) / GB) / BPT)).sqrt())) - BPT) * (C - BPL)) + (BOU * BPL);
                let BPX = GD / BPW;
                let BQC = if DQ != 0.0 {
                    let BPY = A - BPX;
                    let BPZ = EA * (BPX + (((BPY * BPY) + DZ).sqrt()));
                    BPZ
                } else {
                    let BQA = A - BPX;
                    let BQB = EA * (BPX + (BQA * (((EE / DZ) * BQA).tanh())));
                    BQB
                };
                let BQD = GD * (C / ((C + (BQC.powf(BLS))).powf(BNS)));
                let BQE = BNU / BPW;
                let BQJ = if DQ != 0.0 {
                    let BQF = A - BQE;
                    let BQG = EA * (BQE + (((BQF * BQF) + DZ).sqrt()));
                    BQG
                } else {
                    let BQH = A - BQE;
                    let BQI = EA * (BQE + (BQH * (((EE / DZ) * BQH).tanh())));
                    BQI
                };
                let BQK = BNU * (C / ((C + (BQJ.powf(BLS))).powf(BNS)));
                let BQL = (GG - BOW) / BLX;
                let BQM = if BQL > ES { 1.0 } else { 0.0 };
                let BQP;
                if BQM != 0.0 {
                    BQP = A;
                } else {
                    let BQN = if BQL < -5e1f64 { 1.0 } else { 0.0 };
                    let BQQ = if BQN != 0.0 {
                        C
                    } else {
                        let BQO = C / (C + (BQL.exp()));
                        BQO
                    };
                    BQP = BQQ;
                }
                let BQR = ((BLW - BQK) - (BMA - (BMY * BQP))) / BOU;
                let BQS = if BQR > ES { 1.0 } else { 0.0 };
                if BQS != 0.0 {
                } else {
                    let BQT = if BQR < -5e1f64 { 1.0 } else { 0.0 };
                    if BQT != 0.0 {
                    } else {
                    }
                }
                let BQU = (BLW - BOW) / BLX;
                let BQV = if BQU > ES { 1.0 } else { 0.0 };
                let BQY;
                if BQV != 0.0 {
                    BQY = A;
                } else {
                    let BQW = if BQU < -5e1f64 { 1.0 } else { 0.0 };
                    let BQZ = if BQW != 0.0 {
                        C
                    } else {
                        let BQX = C / (C + (BQU.exp()));
                        BQX
                    };
                    BQY = BQZ;
                }
                let BRA = ((GG - BQD) - (BMA - (BMY * BQY))) / BOU;
                let BRB = if BRA > ES { 1.0 } else { 0.0 };
                if BRB != 0.0 {
                } else {
                    let BRC = if BRA < -5e1f64 { 1.0 } else { 0.0 };
                    if BRC != 0.0 {
                    } else {
                    }
                }
                if BRD != 0.0 {
                    let BRE = (A - (BMA - ((JU * EA) * BLX))) / BOU;
                    let BRF = if BRE > ES { 1.0 } else { 0.0 };
                    if BRF != 0.0 {
                    } else {
                        let BRG = if BRE < -5e1f64 { 1.0 } else { 0.0 };
                        if BRG != 0.0 {
                        } else {
                        }
                    }
                    if BRF != 0.0 {
                    } else {
                        let BRH = if BRE < -5e1f64 { 1.0 } else { 0.0 };
                        if BRH != 0.0 {
                        } else {
                        }
                    }
                } else {
                }
                if BRI != 0.0 {
                    let BRJ = (GG - (BMA - ((JU * EA) * BLX))) / BOU;
                    let BRK = if BRJ > ES { 1.0 } else { 0.0 };
                    if BRK != 0.0 {
                    } else {
                        let BRL = if BRJ < -5e1f64 { 1.0 } else { 0.0 };
                        if BRL != 0.0 {
                        } else {
                        }
                    }
                } else {
                }
            } else {
            }
            let BSA;
            let DKA;
            if DQ != 0.0 {
                let DLI = DKY * DN;
                let BRW = ((DN * DN) + DZ).sqrt();
                let DLJ = (DLI + DLI) * (DJM / (DLA * BRW));
                BSA = BRW;
                DKA = DLJ;
            } else {
                let BRX = EE / DZ;
                let BRY = (BRX * DN).tanh();
                let BRZ = DN * BRY;
                let DLH = (DKY * BRY) + (((DKY * BRX) * (DJM - (BRY * BRY))) * DN);
                BSA = BRZ;
                DKA = DLH;
            }
            let BSB = DP - DN;
            let DLK = Lanes([0.0, DKZ[0], DKZ[1]]);
            let DLL = DLK - Lanes([DKY[0], 0.0, DKY[1]]);
            let BSC = BRP * AU;
            let DLM = DKU * BRP;
            let BSD = JJ * AU;
            let BSE = parameters[36] / BSD;
            let DLN = DKA * BRO;
            let BSF = BSE + (BRO * BSA);
            let DLO = Lanes([((((DKU * JJ) * BSE) * DLB) / BSD), 0.0, 0.0]) + Lanes([0.0, DLN[0], DLN[1]]);
            let DLP = DJV * BFW;
            let BSG = parameters[35] + (BFW * AV);
            let BSH = AY.powf(JB);
            let DLQ = DKV * (JB * (AY.powf((JB - DJM))));
            let BSI = if JA != A { 1.0 } else { 0.0 };
            let BSO;
            let DKB;
            if BSI != 0.0 {
                let BSJ = BSA / JA;
                let BSK = C + (BSJ.powf(BRR));
                let BSL = C / BRR;
                let BSM = BSK.powf(BSL);
                let BSN = BSA / BSM;
                let DLS = (DKA - ((((DKA / JA) * (BRR * (BSJ.powf((BRR - DJM))))) * (BSL * (BSK.powf((BSL - DJM))))) * BSN)) / BSM;
                BSO = BSN;
                DKB = DLS;
            } else {
                BSO = A;
                DKB = DLR;
            }
            let BSP = parameters[37] - (BSO * BRN);
            let DLT = (((DKB * BRN) * DLB) * BSA) + (DKA * BSP);
            let BSQ = BSG - (BSP * BSA);
            let DLU = Lanes([DLP, 0.0, 0.0]) - Lanes([0.0, DLT[0], DLT[1]]);
            let BSR = EX * BSF;
            let BSS = BSR * AU;
            let DLV = ((DLO * EX) * AU) + Lanes([(DKU * BSR), 0.0, 0.0]);
            let BST = BN * BSS;
            let DLW = Lanes([(DKX * BSS), 0.0, 0.0]) + (DLV * BN);
            let BSU = (JU * BSC) / EX;
            let BSV = BSQ - BSU;
            let DLX = DLU - Lanes([((DLM * JU) / EX), 0.0, 0.0]);
            let BTD;
            let DKC;
            if DQ != 0.0 {
                let BSW = DP - BSB;
                let DMA = (DLK - DLL) * BSW;
                let BSX = ((BSW * BSW) + DZ).sqrt();
                let BSY = EA * ((DP + BSB) + BSX);
                let DMB = ((DLK + DLL) + ((DMA + DMA) * (DJM / (DLA * BSX)))) * EA;
                BTD = BSY;
                DKC = DMB;
            } else {
                let BSZ = DP - BSB;
                let DLY = DLK - DLL;
                let BTA = EE / DZ;
                let BTB = (BTA * BSZ).tanh();
                let BTC = EA * ((DP + BSB) + (BSZ * BTB));
                let DLZ = ((DLK + DLL) + ((DLY * BTB) + (((DLY * BTA) * (DJM - (BTB * BTB))) * BSZ))) * EA;
                BTD = BTC;
                DKC = DLZ;
            }
            let DMC = Lanes([DLX[0], DLX[1], 0.0, DLX[2]]);
            let BTE = (BTD - BSV) / BSC;
            let DMD = ((Lanes([0.0, DKC[0], DKC[1], DKC[2]]) - DMC) - Lanes([(DLM * BTE), 0.0, 0.0, 0.0])) / BSC;
            let BTF = if BTE > ES { 1.0 } else { 0.0 };
            let BTU;
            let DKD;
            if BTF != 0.0 {
                BTU = A;
                DKD = DMF;
            } else {
                let BTG = if BTE < -5e1f64 { 1.0 } else { 0.0 };
                let BTV;
                let DKE;
                if BTG != 0.0 {
                    BTV = C;
                    DKE = DMF;
                } else {
                    let BTH = BTE.exp();
                    let BTI = C + BTH;
                    let BTJ = C / BTI;
                    let DME = (((DMD * BTH) * BTJ) * DLB) / BTI;
                    BTV = BTJ;
                    DKE = DME;
                }
                BTU = BTV;
                DKD = DKE;
            }
            let BTR;
            let DKF;
            if DQ != 0.0 {
                let BTK = DP - BSB;
                let DMI = (DLK - DLL) * BTK;
                let BTL = ((BTK * BTK) + DZ).sqrt();
                let BTM = EA * ((DP + BSB) + BTL);
                let DMJ = ((DLK + DLL) + ((DMI + DMI) * (DJM / (DLA * BTL)))) * EA;
                BTR = BTM;
                DKF = DMJ;
            } else {
                let BTN = DP - BSB;
                let DMG = DLK - DLL;
                let BTO = EE / DZ;
                let BTP = (BTO * BTN).tanh();
                let BTQ = EA * ((DP + BSB) + (BTN * BTP));
                let DMH = ((DLK + DLL) + ((DMG * BTP) + (((DMG * BTO) * (DJM - (BTP * BTP))) * BTN))) * EA;
                BTR = BTQ;
                DKF = DMH;
            }
            let BTS = JU * AE;
            let BTT = BTS * BSC;
            let DMK = DLM * BTS;
            let DML = Lanes([DLU[0], DLU[1], 0.0, DLU[2]]);
            let BTW = (BTR - (BSQ - (BTT * BTU))) / BSS;
            let DMM = DLV * BTW;
            let DMN = ((Lanes([0.0, DKF[0], DKF[1], DKF[2]]) - (DML - (Lanes([(DMK * BTU), 0.0, 0.0, 0.0]) + (DKD * BTT)))) - Lanes([DMM[0], DMM[1], 0.0, DMM[2]])) / BSS;
            let BTX = if BTW > ES { 1.0 } else { 0.0 };
            let BUG;
            let DKG;
            if BTX != 0.0 {
                let BTY = BST * BTW;
                let DMS = DLW * BTW;
                let DMT = Lanes([DMS[0], DMS[1], 0.0, DMS[2]]) + (DMN * BST);
                BUG = BTY;
                DKG = DMT;
            } else {
                let BTZ = if BTW < -5e1f64 { 1.0 } else { 0.0 };
                let BUH;
                let DKH;
                if BTZ != 0.0 {
                    let BUA = BTW.exp();
                    let BUB = BST * BUA;
                    let DMQ = DLW * BUA;
                    let DMR = Lanes([DMQ[0], DMQ[1], 0.0, DMQ[2]]) + ((DMN * BUA) * BST);
                    BUH = BUB;
                    DKH = DMR;
                } else {
                    let BUC = BTW.exp();
                    let BUD = C + BUC;
                    let BUE = BUD.ln();
                    let BUF = BST * BUE;
                    let DMO = DLW * BUE;
                    let DMP = Lanes([DMO[0], DMO[1], 0.0, DMO[2]]) + (((DMN * BUC) * (DJM / BUD)) * BST);
                    BUH = BUF;
                    DKH = DMP;
                }
                BUG = BUH;
                DKG = DKH;
            }
            let BUI = (BRS * BUG) / BN;
            let BUJ = C + BUI;
            let BUK = BSH * BUJ;
            let BUL = EJ / BUK;
            let DMU = (((Lanes([(DLQ * BUJ), 0.0, 0.0, 0.0]) + ((((DKG * BRS) - Lanes([(DKX * BUI), 0.0, 0.0, 0.0])) / BN) * BSH)) * BUL) * DLB) / BUK;
            let BUM = C + (JC * Z);
            let BUN = (C + (JC * B)) / BUM;
            let BUO = BRQ * BUN;
            let BUP = C + ((JD * BSA) / BRM);
            let DMV = ((DKA * JD) / BRM) * BUO;
            let DMW = Lanes([((((((DJV * JC) * BUN) * DLB) / BUM) * BRQ) * BUP), 0.0, 0.0]) + Lanes([0.0, DMV[0], DMV[1]]);
            let BUQ = (BRT * BUG) / BN;
            let BUR = C + BUQ;
            let BUS = (BUO * BUP) / BUR;
            let DMX = (Lanes([DMW[0], DMW[1], 0.0, DMW[2]]) - ((((DKG * BRT) - Lanes([(DKX * BUQ), 0.0, 0.0, 0.0])) / BN) * BUS)) / BUR;
            let BUT = EX * BTU;
            let BUU = BUT * AU;
            let BUV = C - BTU;
            let DMY = DKD * DLB;
            let BUW = ((BUU * BUL) / BRM) + (BUV * BUS);
            let DMZ = ((((((DKD * EX) * AU) + Lanes([(DKU * BUT), 0.0, 0.0, 0.0])) * BUL) + (DMU * BUU)) / BRM) + ((DMY * BUS) + (DMX * BUV));
            let BUX = (BUS * BRM) / BUL;
            let DNA = ((DMX * BRM) - (DMU * BUX)) / BUL;
            let BUY = (EX * BUG) / BN;
            let BUZ = BUY / BUX;
            let BVA = (C + BUZ).sqrt();
            let BVB = (BUX * BVA) - BUX;
            let BVC = BSS * BTU;
            let DNB = DLV * BTU;
            let DNC = Lanes([DNB[0], DNB[1], 0.0, DNB[2]]) + (DKD * BSS);
            let BVD = (BUX * BUV) + BVC;
            let DND = ((DNA * BUV) + (DMY * BUX)) + DNC;
            let BVE = (BVB * BUV) + BVC;
            let DNE = (((((DNA * BVA) + (((((((DKG * EX) - Lanes([(DKX * BUY), 0.0, 0.0, 0.0])) / BN) - (DNA * BUZ)) / BUX) * (DJM / (DLA * BVA))) * BUX)) - DNA) * BUV) + (DMY * BVB)) + DNC;
            let BVF = DN / BVE;
            let DNF = (Lanes([0.0, DKY[0], 0.0, DKY[1]]) - (DNE * BVF)) / BVE;
            let BVN;
            let DKI;
            if DQ != 0.0 {
                let BVG = A - BVF;
                let DNI = (DNF * DLB) * BVG;
                let BVH = ((BVG * BVG) + DZ).sqrt();
                let BVI = EA * (BVF + BVH);
                let DNJ = (DNF + ((DNI + DNI) * (DJM / (DLA * BVH)))) * EA;
                BVN = BVI;
                DKI = DNJ;
            } else {
                let BVJ = A - BVF;
                let DNG = DNF * DLB;
                let BVK = EE / DZ;
                let BVL = (BVK * BVJ).tanh();
                let BVM = EA * (BVF + (BVJ * BVL));
                let DNH = (DNF + ((DNG * BVL) + (((DNG * BVK) * (DJM - (BVL * BVL))) * BVJ))) * EA;
                BVN = BVM;
                DKI = DNH;
            }
            let DNK = BRR - DJM;
            let BVO = C + (BVN.powf(BRR));
            let BVP = C / BRR;
            let BVQ = BVO.powf(BVP);
            let DNL = BVP - DJM;
            let BVR = C / BVQ;
            let BVS = DN * BVR;
            let DNM = DKY * BVR;
            let DNN = Lanes([0.0, DNM[0], 0.0, DNM[1]]) + ((((((DKI * (BRR * (BVN.powf(DNK)))) * (BVP * (BVO.powf(DNL)))) * BVR) * DLB) / BVQ) * DN);
            let BVT = -DN;
            let DNO = DKY * DLB;
            let BVU = BVT / BVE;
            let DNP = (Lanes([0.0, DNO[0], 0.0, DNO[1]]) - (DNE * BVU)) / BVE;
            let BWC;
            let DKJ;
            if DQ != 0.0 {
                let BVV = A - BVU;
                let DNS = (DNP * DLB) * BVV;
                let BVW = ((BVV * BVV) + DZ).sqrt();
                let BVX = EA * (BVU + BVW);
                let DNT = (DNP + ((DNS + DNS) * (DJM / (DLA * BVW)))) * EA;
                BWC = BVX;
                DKJ = DNT;
            } else {
                let BVY = A - BVU;
                let DNQ = DNP * DLB;
                let BVZ = EE / DZ;
                let BWA = (BVZ * BVY).tanh();
                let BWB = EA * (BVU + (BVY * BWA));
                let DNR = (DNP + ((DNQ * BWA) + (((DNQ * BVZ) * (DJM - (BWA * BWA))) * BVY))) * EA;
                BWC = BWB;
                DKJ = DNR;
            }
            let BWD = C + (BWC.powf(BRR));
            let BWE = BWD.powf(BVP);
            let BWF = C / BWE;
            let BWG = BVT * BWF;
            let DNU = DNO * BWF;
            let DNV = Lanes([0.0, DNU[0], 0.0, DNU[1]]) + ((((((DKJ * (BRR * (BWC.powf(DNK)))) * (BVP * (BWD.powf(DNL)))) * BWF) * DLB) / BWE) * BVT);
            let DNW = Lanes([0.0, 0.0, DKZ[0], DKZ[1]]);
            let BWH = (DP - BSV) / BSC;
            let DNX = ((DNW - DMC) - Lanes([(DLM * BWH), 0.0, 0.0, 0.0])) / BSC;
            let BWI = if BWH > ES { 1.0 } else { 0.0 };
            let BWN;
            let DKK;
            if BWI != 0.0 {
                BWN = A;
                DKK = DMF;
            } else {
                let BWJ = if BWH < -5e1f64 { 1.0 } else { 0.0 };
                let BWO;
                let DKL;
                if BWJ != 0.0 {
                    BWO = C;
                    DKL = DMF;
                } else {
                    let BWK = BWH.exp();
                    let BWL = C + BWK;
                    let BWM = C / BWL;
                    let DNY = (((DNX * BWK) * BWM) * DLB) / BWL;
                    BWO = BWM;
                    DKL = DNY;
                }
                BWN = BWO;
                DKK = DKL;
            }
            let DNZ = Lanes([0.0, DLL[0], DLL[1], DLL[2]]);
            let BWP = ((BSB - BWG) - (BSQ - (BTT * BWN))) / BSS;
            let DOA = DLV * BWP;
            let DOB = (((DNZ - DNV) - (DML - (Lanes([(DMK * BWN), 0.0, 0.0, 0.0]) + (DKK * BTT)))) - Lanes([DOA[0], DOA[1], 0.0, DOA[2]])) / BSS;
            let BWQ = if BWP > ES { 1.0 } else { 0.0 };
            let BXR;
            let DKM;
            if BWQ != 0.0 {
                let BWR = BST * BWP;
                let DOG = DLW * BWP;
                let DOH = Lanes([DOG[0], DOG[1], 0.0, DOG[2]]) + (DOB * BST);
                BXR = BWR;
                DKM = DOH;
            } else {
                let BWS = if BWP < -5e1f64 { 1.0 } else { 0.0 };
                let BXS;
                let DKN;
                if BWS != 0.0 {
                    let BWT = BWP.exp();
                    let BWU = BST * BWT;
                    let DOE = DLW * BWT;
                    let DOF = Lanes([DOE[0], DOE[1], 0.0, DOE[2]]) + ((DOB * BWT) * BST);
                    BXS = BWU;
                    DKN = DOF;
                } else {
                    let BWV = BWP.exp();
                    let BWW = C + BWV;
                    let BWX = BWW.ln();
                    let BWY = BST * BWX;
                    let DOC = DLW * BWX;
                    let DOD = Lanes([DOC[0], DOC[1], 0.0, DOC[2]]) + (((DOB * BWV) * (DJM / BWW)) * BST);
                    BXS = BWY;
                    DKN = DOD;
                }
                BXR = BXS;
                DKM = DKN;
            }
            let BWZ = (BSB - BSV) / BSC;
            let DOI = ((DNZ - DMC) - Lanes([(DLM * BWZ), 0.0, 0.0, 0.0])) / BSC;
            let BXA = if BWZ > ES { 1.0 } else { 0.0 };
            let BXF;
            let DKO;
            if BXA != 0.0 {
                BXF = A;
                DKO = DMF;
            } else {
                let BXB = if BWZ < -5e1f64 { 1.0 } else { 0.0 };
                let BXG;
                let DKP;
                if BXB != 0.0 {
                    BXG = C;
                    DKP = DMF;
                } else {
                    let BXC = BWZ.exp();
                    let BXD = C + BXC;
                    let BXE = C / BXD;
                    let DOJ = (((DOI * BXC) * BXE) * DLB) / BXD;
                    BXG = BXE;
                    DKP = DOJ;
                }
                BXF = BXG;
                DKO = DKP;
            }
            let BXH = ((DP - BVS) - (BSQ - (BTT * BXF))) / BSS;
            let DOK = DLV * BXH;
            let DOL = (((DNW - DNN) - (DML - (Lanes([(DMK * BXF), 0.0, 0.0, 0.0]) + (DKO * BTT)))) - Lanes([DOK[0], DOK[1], 0.0, DOK[2]])) / BSS;
            let BXI = if BXH > ES { 1.0 } else { 0.0 };
            let BXT;
            let DKQ;
            if BXI != 0.0 {
                let BXJ = BST * BXH;
                let DOQ = DLW * BXH;
                let DOR = Lanes([DOQ[0], DOQ[1], 0.0, DOQ[2]]) + (DOL * BST);
                BXT = BXJ;
                DKQ = DOR;
            } else {
                let BXK = if BXH < -5e1f64 { 1.0 } else { 0.0 };
                let BXU;
                let DKR;
                if BXK != 0.0 {
                    let BXL = BXH.exp();
                    let BXM = BST * BXL;
                    let DOO = DLW * BXL;
                    let DOP = Lanes([DOO[0], DOO[1], 0.0, DOO[2]]) + ((DOL * BXL) * BST);
                    BXU = BXM;
                    DKR = DOP;
                } else {
                    let BXN = BXH.exp();
                    let BXO = C + BXN;
                    let BXP = BXO.ln();
                    let BXQ = BST * BXP;
                    let DOM = DLW * BXP;
                    let DON = Lanes([DOM[0], DOM[1], 0.0, DOM[2]]) + (((DOL * BXN) * (DJM / BXO)) * BST);
                    BXU = BXQ;
                    DKR = DON;
                }
                BXT = BXU;
                DKQ = DKR;
            }
            let BXV = (BXR - BXT) / BN;
            let BXW = BXV / BVD;
            let DOS = ((((DKM - DKQ) - Lanes([(DKX * BXV), 0.0, 0.0, 0.0])) / BN) - (DND * BXW)) / BVD;
            let BYB;
            let DKS;
            if DQ != 0.0 {
                let DOU = DOS * BXW;
                let BXX = ((BXW * BXW) + DZ).sqrt();
                let DOV = (DOU + DOU) * (DJM / (DLA * BXX));
                BYB = BXX;
                DKS = DOV;
            } else {
                let BXY = EE / DZ;
                let BXZ = (BXY * BXW).tanh();
                let BYA = BXW * BXZ;
                let DOT = (DOS * BXZ) + (((DOS * BXY) * (DJM - (BXZ * BXZ))) * BXW);
                BYB = BYA;
                DKS = DOT;
            }
            let BYC = C + (BYB.powf(BRR));
            let BYD = BYC.powf(BVP);
            let BYE = BXW / BYD;
            let BYF = BUW * BYE;
            let BYG = (DK * L) * M;
            let BYH = BYG * EA;
            let BYI = BYH * (BXR + BXT);
            let BYJ = BYI * BYF;
            let BYK = BYJ * BRU;
            let DOW = ((((DKM + DKQ) * BYH) * BYF) + (((DMZ * BYE) + (((DOS - (((DKS * (BRR * (BYB.powf(DNK)))) * (BVP * (BYC.powf(DNL)))) * BYE)) / BYD) * BUW)) * BYI)) * BRU;
            let DOX = DJY * BYJ;
            let DOY = Lanes([DOW[0], DOW[1], DOW[2], DOW[3], 0.0, 0.0, 0.0, 0.0]) + Lanes([0.0, 0.0, 0.0, 0.0, DOX[0], DOX[1], DOX[2], DOX[3]]);
            let BYL = (EX * BSE) * AU;
            let BYM = BN * BYL;
            let BYN = BSG - BSU;
            let BYS = if DQ != 0.0 {
                let BYO = DP - BSB;
                let BYP = EA * ((DP + BSB) + (((BYO * BYO) + DZ).sqrt()));
                BYP
            } else {
                let BYQ = DP - BSB;
                let BYR = EA * ((DP + BSB) + (BYQ * (((EE / DZ) * BYQ).tanh())));
                BYR
            };
            let BYT = (BYS - BYN) / BSC;
            let BYU = if BYT > ES { 1.0 } else { 0.0 };
            let BZC;
            if BYU != 0.0 {
                BZC = A;
            } else {
                let BYV = if BYT < -5e1f64 { 1.0 } else { 0.0 };
                let BZD = if BYV != 0.0 {
                    C
                } else {
                    let BYW = C / (C + (BYT.exp()));
                    BYW
                };
                BZC = BZD;
            }
            let BZB = if DQ != 0.0 {
                let BYX = DP - BSB;
                let BYY = EA * ((DP + BSB) + (((BYX * BYX) + DZ).sqrt()));
                BYY
            } else {
                let BYZ = DP - BSB;
                let BZA = EA * ((DP + BSB) + (BYZ * (((EE / DZ) * BYZ).tanh())));
                BZA
            };
            let BZE = (BZB - (BSG - (BTT * BZC))) / BYL;
            let BZF = if BZE > ES { 1.0 } else { 0.0 };
            let BZL;
            if BZF != 0.0 {
                let BZG = BYM * BZE;
                BZL = BZG;
            } else {
                let BZH = if BZE < -5e1f64 { 1.0 } else { 0.0 };
                let BZM = if BZH != 0.0 {
                    let BZI = BYM * (BZE.exp());
                    BZI
                } else {
                    let BZJ = BYM * ((C + (BZE.exp())).ln());
                    BZJ
                };
                BZL = BZM;
            }
            let BZK = (BUO * BRM) / (EJ / BSH);
            let BZN = (((BZK * ((C + (((EX * BZL) / BN) / BZK)).sqrt())) - BZK) * (C - BZC)) + (BYL * BZC);
            let BZO = DN / BZN;
            let BZT = if DQ != 0.0 {
                let BZP = A - BZO;
                let BZQ = EA * (BZO + (((BZP * BZP) + DZ).sqrt()));
                BZQ
            } else {
                let BZR = A - BZO;
                let BZS = EA * (BZO + (BZR * (((EE / DZ) * BZR).tanh())));
                BZS
            };
            let BZU = DN * (C / ((C + (BZT.powf(BRR))).powf(BVP)));
            let BZV = BVT / BZN;
            let CAA = if DQ != 0.0 {
                let BZW = A - BZV;
                let BZX = EA * (BZV + (((BZW * BZW) + DZ).sqrt()));
                BZX
            } else {
                let BZY = A - BZV;
                let BZZ = EA * (BZV + (BZY * (((EE / DZ) * BZY).tanh())));
                BZZ
            };
            let CAB = BVT * (C / ((C + (CAA.powf(BRR))).powf(BVP)));
            let CAC = (DP - BYN) / BSC;
            let CAD = if CAC > ES { 1.0 } else { 0.0 };
            let CAG;
            if CAD != 0.0 {
                CAG = A;
            } else {
                let CAE = if CAC < -5e1f64 { 1.0 } else { 0.0 };
                let CAH = if CAE != 0.0 {
                    C
                } else {
                    let CAF = C / (C + (CAC.exp()));
                    CAF
                };
                CAG = CAH;
            }
            let CAI = ((BSB - CAB) - (BSG - (BTT * CAG))) / BYL;
            let CAJ = if CAI > ES { 1.0 } else { 0.0 };
            let CBA;
            if CAJ != 0.0 {
                let CAK = BYM * CAI;
                CBA = CAK;
            } else {
                let CAL = if CAI < -5e1f64 { 1.0 } else { 0.0 };
                let CBB = if CAL != 0.0 {
                    let CAM = BYM * (CAI.exp());
                    CAM
                } else {
                    let CAN = BYM * ((C + (CAI.exp())).ln());
                    CAN
                };
                CBA = CBB;
            }
            let CAO = (BSB - BYN) / BSC;
            let CAP = if CAO > ES { 1.0 } else { 0.0 };
            let CAS;
            if CAP != 0.0 {
                CAS = A;
            } else {
                let CAQ = if CAO < -5e1f64 { 1.0 } else { 0.0 };
                let CAT = if CAQ != 0.0 {
                    C
                } else {
                    let CAR = C / (C + (CAO.exp()));
                    CAR
                };
                CAS = CAT;
            }
            let CAU = ((DP - BZU) - (BSG - (BTT * CAS))) / BYL;
            let CAV = if CAU > ES { 1.0 } else { 0.0 };
            let CBD;
            if CAV != 0.0 {
                let CAW = BYM * CAU;
                CBD = CAW;
            } else {
                let CAX = if CAU < -5e1f64 { 1.0 } else { 0.0 };
                let CBE = if CAX != 0.0 {
                    let CAY = BYM * (CAU.exp());
                    CAY
                } else {
                    let CAZ = BYM * ((C + (CAU.exp())).ln());
                    CAZ
                };
                CBD = CBE;
            }
            let CBC = (CBA * CBA) + OQ;
            let CBF = (CBD * CBD) + OQ;
            let CBG = (CBA * CBD) + OQ;
            let CBH = CBC + CBF;
            let CBI = (EX * ((((EX * ((CBC * CBA) + OR)) + (AZ * ((CBF * CBD) + OR))) + ((OS * CBC) * CBD)) + ((6e0f64 * CBF) * CBA))) / (1.5e1f64 * (CBH + (EX * CBG)));
            let CBJ = L * M;
            let CBK = (CBJ * BRM) * DK;
            let CBL = (CBK * (((6.666666666666666e-1f64 * (CBH + CBG)) / ((CBA + CBD) + 2e-19f64)) - CBI)) * BRU;
            let CBM = (CBK * CBI) * BRU;
            if CBN != 0.0 {
                let CBO = (A - (BSG - ((JU * EA) * BSC))) / BYL;
                let CBP = if CBO > ES { 1.0 } else { 0.0 };
                if CBP != 0.0 {
                } else {
                    let CBQ = if CBO < -5e1f64 { 1.0 } else { 0.0 };
                    if CBQ != 0.0 {
                    } else {
                    }
                }
                if CBP != 0.0 {
                } else {
                    let CBR = if CBO < -5e1f64 { 1.0 } else { 0.0 };
                    if CBR != 0.0 {
                    } else {
                    }
                }
            } else {
            }
            if CBS != 0.0 {
                let CBT = (DP - (BSG - ((JU * EA) * BSC))) / BYL;
                let CBU = if CBT > ES { 1.0 } else { 0.0 };
                if CBU != 0.0 {
                } else {
                    let CBV = if CBT < -5e1f64 { 1.0 } else { 0.0 };
                    if CBV != 0.0 {
                    } else {
                    }
                }
            } else {
            }
            let CBW = if parameters[322] == A { 1.0 } else { 0.0 };
            if CBW != 0.0 {
            } else {
            }
            let CBX = DO - DL;
            let CBY = if parameters[254] == C { 1.0 } else { 0.0 };
            let DFO;
            let DFQ;
            let DFS;
            let DFW;
            let DFY;
            let DGA;
            let DGD;
            let DGE;
            let DGF;
            let DGH;
            let DGI;
            let DGJ;
            if CBY != 0.0 {
                let CBZ = DK * (DO - EK);
                let CCH = C - CCG;
                let CCJ = CCH * CCI;
                let CCO = CCH * CCN;
                let CCR = (parameters[257] / AU) * (-CCQ);
                let CCS = if CCR > ES { 1.0 } else { 0.0 };
                let CCX;
                if CCS != 0.0 {
                    let CCT = 5.184705528587072e21f64 * (C + (CCR - ES));
                    CCX = CCT;
                } else {
                    let CCU = if CCR < -5e1f64 { 1.0 } else { 0.0 };
                    let CCY = if CCU != 0.0 {
                        CCV
                    } else {
                        let CCW = CCR.exp();
                        CCW
                    };
                    CCX = CCY;
                }
                let CCZ = -CBZ;
                let CDA = (CCE * (CCZ - CCF)) + CCR;
                let CDB = ((-CCE) * CCF) + CCR;
                let CDC = if CDA > ES { 1.0 } else { 0.0 };
                let CDH;
                if CDC != 0.0 {
                    let CDD = 5.184705528587072e21f64 * (C + (CDA - ES));
                    CDH = CDD;
                } else {
                    let CDE = if CDA < -5e1f64 { 1.0 } else { 0.0 };
                    let CDI = if CDE != 0.0 {
                        CDF
                    } else {
                        let CDG = CDA.exp();
                        CDG
                    };
                    CDH = CDI;
                }
                let CDJ = if CDB > ES { 1.0 } else { 0.0 };
                let CDO;
                if CDJ != 0.0 {
                    let CDK = 5.184705528587072e21f64 * (C + (CDB - ES));
                    CDO = CDK;
                } else {
                    let CDL = if CDB < -5e1f64 { 1.0 } else { 0.0 };
                    let CDP = if CDL != 0.0 {
                        CDM
                    } else {
                        let CDN = CDB.exp();
                        CDN
                    };
                    CDO = CDP;
                }
                let CDQ = CDH - CDO;
                let CDR = (BYG * CCJ) * BA;
                let CDS = CCD / AU;
                let CDT = (CDS * CBZ) + CCR;
                let CDU = if CDT > ES { 1.0 } else { 0.0 };
                let CDZ;
                if CDU != 0.0 {
                    let CDV = 5.184705528587072e21f64 * (C + (CDT - ES));
                    CDZ = CDV;
                } else {
                    let CDW = if CDT < -5e1f64 { 1.0 } else { 0.0 };
                    let CEA = if CDW != 0.0 {
                        CDX
                    } else {
                        let CDY = CDT.exp();
                        CDY
                    };
                    CDZ = CEA;
                }
                let CEB = if CCC == C { 1.0 } else { 0.0 };
                let CGS;
                if CEB != 0.0 {
                    let CEC = CDR * ((CDZ - (CCK * CDQ)) - CCX);
                    CGS = CEC;
                } else {
                    let CED = (CCE * ((-CCA) - CCF)) + CCR;
                    let CEE = if CED > ES { 1.0 } else { 0.0 };
                    let CEJ;
                    if CEE != 0.0 {
                        let CEF = 5.184705528587072e21f64 * (C + (CED - ES));
                        CEJ = CEF;
                    } else {
                        let CEG = if CED < -5e1f64 { 1.0 } else { 0.0 };
                        let CEK = if CEG != 0.0 {
                            CEH
                        } else {
                            let CEI = CED.exp();
                            CEI
                        };
                        CEJ = CEK;
                    }
                    let CEL = CEJ - CDO;
                    let CEM = (CDS * CCA) + CCR;
                    let CEN = if CEM > ES { 1.0 } else { 0.0 };
                    let CES;
                    if CEN != 0.0 {
                        let CEO = 5.184705528587072e21f64 * (C + (CEM - ES));
                        CES = CEO;
                    } else {
                        let CEP = if CEM < -5e1f64 { 1.0 } else { 0.0 };
                        let CET = if CEP != 0.0 {
                            CEQ
                        } else {
                            let CER = CEM.exp();
                            CER
                        };
                        CES = CET;
                    }
                    let CEU = CCK * CEL;
                    let CEV = (CES - CEU) - CCX;
                    let CEW = CCK * CDQ;
                    let CEX = CDR * ((CDZ - CEW) - CCX);
                    let CEY = if CCC > A { 1.0 } else { 0.0 };
                    let CGA;
                    if CEY != 0.0 {
                        let CEZ = (CCC * CCD) / AU;
                        let CFA = (CEZ * CCA) + CCR;
                        let CFB = if CFA > ES { 1.0 } else { 0.0 };
                        let CFG;
                        if CFB != 0.0 {
                            let CFC = 5.184705528587072e21f64 * (C + (CFA - ES));
                            CFG = CFC;
                        } else {
                            let CFD = if CFA < -5e1f64 { 1.0 } else { 0.0 };
                            let CFH = if CFD != 0.0 {
                                CFE
                            } else {
                                let CFF = CFA.exp();
                                CFF
                            };
                            CFG = CFH;
                        }
                        let CFI = (CFG - CEU) - CCX;
                        let CFJ = (CEZ * CBZ) + CCR;
                        let CFK = if CFJ > ES { 1.0 } else { 0.0 };
                        let CFP;
                        if CFK != 0.0 {
                            let CFL = 5.184705528587072e21f64 * (C + (CFJ - ES));
                            CFP = CFL;
                        } else {
                            let CFM = if CFJ < -5e1f64 { 1.0 } else { 0.0 };
                            let CFQ = if CFM != 0.0 {
                                CFN
                            } else {
                                let CFO = CFJ.exp();
                                CFO
                            };
                            CFP = CFQ;
                        }
                        let CFR = ((CDR * CEV) / CFI) * ((CFP - CEW) - CCX);
                        CGA = CFR;
                    } else {
                        let CFS = CDR * CEV;
                        CGA = CFS;
                    }
                    let CFT = (CCB * CCB) * AU;
                    let CFU = (CBZ - (CCA - (CFT / EX))) / CFT;
                    let CFV = if CFU > ES { 1.0 } else { 0.0 };
                    let CFY;
                    if CFV != 0.0 {
                        CFY = A;
                    } else {
                        let CFW = if CFU < -5e1f64 { 1.0 } else { 0.0 };
                        let CFZ = if CFW != 0.0 {
                            C
                        } else {
                            let CFX = C / (C + (CFU.exp()));
                            CFX
                        };
                        CFY = CFZ;
                    }
                    let CGB = (CFY * CEX) + ((C - CFY) * CGA);
                    CGS = CGB;
                }
                let CGC = CBZ / CCL;
                let CGF = if DQ != 0.0 {
                    let CGD = ((CGC * CGC) + DZ).sqrt();
                    CGD
                } else {
                    let CGE = CGC * (((EE / DZ) * CGC).tanh());
                    CGE
                };
                let CGG = C / CCM;
                let CGH = ((-DK) * L) * M;
                let CGI = (CGH * CCO) * BA;
                let CGJ = CCP / AU;
                let CGK = CGJ * (CCZ / ((C + (CGF.powf(CCM))).powf(CGG)));
                let CGL = if CGK > ES { 1.0 } else { 0.0 };
                let CGQ;
                if CGL != 0.0 {
                    let CGM = 5.184705528587072e21f64 * (C + (CGK - ES));
                    CGQ = CGM;
                } else {
                    let CGN = if CGK < -5e1f64 { 1.0 } else { 0.0 };
                    let CGR = if CGN != 0.0 {
                        CGO
                    } else {
                        let CGP = CGK.exp();
                        CGP
                    };
                    CGQ = CGR;
                }
                let CGT = CGS + (CGI * (CGQ - C));
                let CGU = DK * (DO - FO);
                let CHC = CCH * CHB;
                let CHH = CCH * CHG;
                let CHN;
                if CCS != 0.0 {
                    let CHJ = 5.184705528587072e21f64 * (C + (CCR - ES));
                    CHN = CHJ;
                } else {
                    let CHK = if CCR < -5e1f64 { 1.0 } else { 0.0 };
                    let CHO = if CHK != 0.0 {
                        CHL
                    } else {
                        let CHM = CCR.exp();
                        CHM
                    };
                    CHN = CHO;
                }
                let CHP = -CGU;
                let CHQ = (CGZ * (CHP - CHA)) + CCR;
                let CHR = ((-CGZ) * CHA) + CCR;
                let CHS = if CHQ > ES { 1.0 } else { 0.0 };
                let CHX;
                if CHS != 0.0 {
                    let CHT = 5.184705528587072e21f64 * (C + (CHQ - ES));
                    CHX = CHT;
                } else {
                    let CHU = if CHQ < -5e1f64 { 1.0 } else { 0.0 };
                    let CHY = if CHU != 0.0 {
                        CHV
                    } else {
                        let CHW = CHQ.exp();
                        CHW
                    };
                    CHX = CHY;
                }
                let CHZ = if CHR > ES { 1.0 } else { 0.0 };
                let CIE;
                if CHZ != 0.0 {
                    let CIA = 5.184705528587072e21f64 * (C + (CHR - ES));
                    CIE = CIA;
                } else {
                    let CIB = if CHR < -5e1f64 { 1.0 } else { 0.0 };
                    let CIF = if CIB != 0.0 {
                        CIC
                    } else {
                        let CID = CHR.exp();
                        CID
                    };
                    CIE = CIF;
                }
                let CIG = CHX - CIE;
                let CIH = (BYG * CHC) * BA;
                let CII = CGY / AU;
                let CIJ = (CII * CGU) + CCR;
                let CIK = if CIJ > ES { 1.0 } else { 0.0 };
                let CIP;
                if CIK != 0.0 {
                    let CIL = 5.184705528587072e21f64 * (C + (CIJ - ES));
                    CIP = CIL;
                } else {
                    let CIM = if CIJ < -5e1f64 { 1.0 } else { 0.0 };
                    let CIQ = if CIM != 0.0 {
                        CIN
                    } else {
                        let CIO = CIJ.exp();
                        CIO
                    };
                    CIP = CIQ;
                }
                let CIR = if CGX == C { 1.0 } else { 0.0 };
                let CLH;
                if CIR != 0.0 {
                    let CIS = CIH * ((CIP - (CHD * CIG)) - CHN);
                    CLH = CIS;
                } else {
                    let CIT = (CGZ * ((-CGV) - CHA)) + CCR;
                    let CIU = if CIT > ES { 1.0 } else { 0.0 };
                    let CIZ;
                    if CIU != 0.0 {
                        let CIV = 5.184705528587072e21f64 * (C + (CIT - ES));
                        CIZ = CIV;
                    } else {
                        let CIW = if CIT < -5e1f64 { 1.0 } else { 0.0 };
                        let CJA = if CIW != 0.0 {
                            CIX
                        } else {
                            let CIY = CIT.exp();
                            CIY
                        };
                        CIZ = CJA;
                    }
                    let CJB = CIZ - CIE;
                    let CJC = (CII * CGV) + CCR;
                    let CJD = if CJC > ES { 1.0 } else { 0.0 };
                    let CJI;
                    if CJD != 0.0 {
                        let CJE = 5.184705528587072e21f64 * (C + (CJC - ES));
                        CJI = CJE;
                    } else {
                        let CJF = if CJC < -5e1f64 { 1.0 } else { 0.0 };
                        let CJJ = if CJF != 0.0 {
                            CJG
                        } else {
                            let CJH = CJC.exp();
                            CJH
                        };
                        CJI = CJJ;
                    }
                    let CJK = CHD * CJB;
                    let CJL = (CJI - CJK) - CHN;
                    let CJM = CHD * CIG;
                    let CJN = CIH * ((CIP - CJM) - CHN);
                    let CJO = if CGX > A { 1.0 } else { 0.0 };
                    let CKQ;
                    if CJO != 0.0 {
                        let CJP = (CGX * CGY) / AU;
                        let CJQ = (CJP * CGV) + CCR;
                        let CJR = if CJQ > ES { 1.0 } else { 0.0 };
                        let CJW;
                        if CJR != 0.0 {
                            let CJS = 5.184705528587072e21f64 * (C + (CJQ - ES));
                            CJW = CJS;
                        } else {
                            let CJT = if CJQ < -5e1f64 { 1.0 } else { 0.0 };
                            let CJX = if CJT != 0.0 {
                                CJU
                            } else {
                                let CJV = CJQ.exp();
                                CJV
                            };
                            CJW = CJX;
                        }
                        let CJY = (CJW - CJK) - CHN;
                        let CJZ = (CJP * CGU) + CCR;
                        let CKA = if CJZ > ES { 1.0 } else { 0.0 };
                        let CKF;
                        if CKA != 0.0 {
                            let CKB = 5.184705528587072e21f64 * (C + (CJZ - ES));
                            CKF = CKB;
                        } else {
                            let CKC = if CJZ < -5e1f64 { 1.0 } else { 0.0 };
                            let CKG = if CKC != 0.0 {
                                CKD
                            } else {
                                let CKE = CJZ.exp();
                                CKE
                            };
                            CKF = CKG;
                        }
                        let CKH = ((CIH * CJL) / CJY) * ((CKF - CJM) - CHN);
                        CKQ = CKH;
                    } else {
                        let CKI = CIH * CJL;
                        CKQ = CKI;
                    }
                    let CKJ = (CGW * CGW) * AU;
                    let CKK = (CGU - (CGV - (CKJ / EX))) / CKJ;
                    let CKL = if CKK > ES { 1.0 } else { 0.0 };
                    let CKO;
                    if CKL != 0.0 {
                        CKO = A;
                    } else {
                        let CKM = if CKK < -5e1f64 { 1.0 } else { 0.0 };
                        let CKP = if CKM != 0.0 {
                            C
                        } else {
                            let CKN = C / (C + (CKK.exp()));
                            CKN
                        };
                        CKO = CKP;
                    }
                    let CKR = (CKO * CJN) + ((C - CKO) * CKQ);
                    CLH = CKR;
                }
                let CKS = CGU / CHE;
                let CKV = if DQ != 0.0 {
                    let CKT = ((CKS * CKS) + DZ).sqrt();
                    CKT
                } else {
                    let CKU = CKS * (((EE / DZ) * CKS).tanh());
                    CKU
                };
                let CKW = C / CHF;
                let CKX = (CGH * CHH) * BA;
                let CKY = CHI / AU;
                let CKZ = CKY * (CHP / ((C + (CKV.powf(CHF))).powf(CKW)));
                let CLA = if CKZ > ES { 1.0 } else { 0.0 };
                let CLF;
                if CLA != 0.0 {
                    let CLB = 5.184705528587072e21f64 * (C + (CKZ - ES));
                    CLF = CLB;
                } else {
                    let CLC = if CKZ < -5e1f64 { 1.0 } else { 0.0 };
                    let CLG = if CLC != 0.0 {
                        CLD
                    } else {
                        let CLE = CKZ.exp();
                        CLE
                    };
                    CLF = CLG;
                }
                let CLI = CLH + (CKX * (CLF - C));
                let CLJ = if parameters[282] == C { 1.0 } else { 0.0 };
                if CLJ != 0.0 {
                    if CCS != 0.0 {
                    } else {
                        let CLN = if CCR < -5e1f64 { 1.0 } else { 0.0 };
                        if CLN != 0.0 {
                        } else {
                        }
                    }
                    if CDC != 0.0 {
                    } else {
                        let CLO = if CDA < -5e1f64 { 1.0 } else { 0.0 };
                        if CLO != 0.0 {
                        } else {
                        }
                    }
                    if CDJ != 0.0 {
                    } else {
                        let CLP = if CDB < -5e1f64 { 1.0 } else { 0.0 };
                        if CLP != 0.0 {
                        } else {
                        }
                    }
                    if CDU != 0.0 {
                    } else {
                        let CLQ = if CDT < -5e1f64 { 1.0 } else { 0.0 };
                        if CLQ != 0.0 {
                        } else {
                        }
                    }
                    if CLR != 0.0 {
                    } else {
                        let CLS = (CCE * ((-CCA) - CCF)) + CCR;
                        let CLT = if CLS > ES { 1.0 } else { 0.0 };
                        if CLT != 0.0 {
                        } else {
                            let CLU = if CLS < -5e1f64 { 1.0 } else { 0.0 };
                            if CLU != 0.0 {
                            } else {
                            }
                        }
                        let CLV = (CDS * CCA) + CCR;
                        let CLW = if CLV > ES { 1.0 } else { 0.0 };
                        if CLW != 0.0 {
                        } else {
                            let CLX = if CLV < -5e1f64 { 1.0 } else { 0.0 };
                            if CLX != 0.0 {
                            } else {
                            }
                        }
                        if CLY != 0.0 {
                            if CLW != 0.0 {
                            } else {
                                let CLZ = if CLV < -5e1f64 { 1.0 } else { 0.0 };
                                if CLZ != 0.0 {
                                } else {
                                }
                            }
                            if CDU != 0.0 {
                            } else {
                                let CMA = if CDT < -5e1f64 { 1.0 } else { 0.0 };
                                if CMA != 0.0 {
                                } else {
                                }
                            }
                        } else {
                        }
                        let CMB = (CCB * CCB) * AU;
                        let CMC = (CBZ - (CCA - (CMB / EX))) / CMB;
                        let CMD = if CMC > ES { 1.0 } else { 0.0 };
                        if CMD != 0.0 {
                        } else {
                            let CME = if CMC < -5e1f64 { 1.0 } else { 0.0 };
                            if CME != 0.0 {
                            } else {
                            }
                        }
                    }
                    let CMF = CBZ / CLK;
                    let CMI = if DQ != 0.0 {
                        let CMG = ((CMF * CMF) + DZ).sqrt();
                        CMG
                    } else {
                        let CMH = CMF * (((EE / DZ) * CMF).tanh());
                        CMH
                    };
                    let CMJ = (CLM / AU) * (CCZ / ((C + (CMI.powf(CLL))).powf((C / CLL))));
                    let CMK = if CMJ > ES { 1.0 } else { 0.0 };
                    if CMK != 0.0 {
                    } else {
                        let CML = if CMJ < -5e1f64 { 1.0 } else { 0.0 };
                        if CML != 0.0 {
                        } else {
                        }
                    }
                    if CCS != 0.0 {
                    } else {
                        let CMP = if CCR < -5e1f64 { 1.0 } else { 0.0 };
                        if CMP != 0.0 {
                        } else {
                        }
                    }
                    if CHS != 0.0 {
                    } else {
                        let CMQ = if CHQ < -5e1f64 { 1.0 } else { 0.0 };
                        if CMQ != 0.0 {
                        } else {
                        }
                    }
                    if CHZ != 0.0 {
                    } else {
                        let CMR = if CHR < -5e1f64 { 1.0 } else { 0.0 };
                        if CMR != 0.0 {
                        } else {
                        }
                    }
                    if CIK != 0.0 {
                    } else {
                        let CMS = if CIJ < -5e1f64 { 1.0 } else { 0.0 };
                        if CMS != 0.0 {
                        } else {
                        }
                    }
                    if CMT != 0.0 {
                    } else {
                        let CMU = (CGZ * ((-CGV) - CHA)) + CCR;
                        let CMV = if CMU > ES { 1.0 } else { 0.0 };
                        if CMV != 0.0 {
                        } else {
                            let CMW = if CMU < -5e1f64 { 1.0 } else { 0.0 };
                            if CMW != 0.0 {
                            } else {
                            }
                        }
                        let CMX = (CII * CGV) + CCR;
                        let CMY = if CMX > ES { 1.0 } else { 0.0 };
                        if CMY != 0.0 {
                        } else {
                            let CMZ = if CMX < -5e1f64 { 1.0 } else { 0.0 };
                            if CMZ != 0.0 {
                            } else {
                            }
                        }
                        if CNA != 0.0 {
                            if CMY != 0.0 {
                            } else {
                                let CNB = if CMX < -5e1f64 { 1.0 } else { 0.0 };
                                if CNB != 0.0 {
                                } else {
                                }
                            }
                            if CIK != 0.0 {
                            } else {
                                let CNC = if CIJ < -5e1f64 { 1.0 } else { 0.0 };
                                if CNC != 0.0 {
                                } else {
                                }
                            }
                        } else {
                        }
                        let CND = (CGW * CGW) * AU;
                        let CNE = (CGU - (CGV - (CND / EX))) / CND;
                        let CNF = if CNE > ES { 1.0 } else { 0.0 };
                        if CNF != 0.0 {
                        } else {
                            let CNG = if CNE < -5e1f64 { 1.0 } else { 0.0 };
                            if CNG != 0.0 {
                            } else {
                            }
                        }
                    }
                    let CNH = CGU / CMM;
                    let CNK = if DQ != 0.0 {
                        let CNI = ((CNH * CNH) + DZ).sqrt();
                        CNI
                    } else {
                        let CNJ = CNH * (((EE / DZ) * CNH).tanh());
                        CNJ
                    };
                    let CNL = (CMO / AU) * (CHP / ((C + (CNK.powf(CMN))).powf((C / CMN))));
                    let CNM = if CNL > ES { 1.0 } else { 0.0 };
                    if CNM != 0.0 {
                    } else {
                        let CNN = if CNL < -5e1f64 { 1.0 } else { 0.0 };
                        if CNN != 0.0 {
                        } else {
                        }
                    }
                } else {
                }
                let CNO = if CCG != A { 1.0 } else { 0.0 };
                let DFP;
                let DFR;
                let DFT;
                let DFX;
                let DFZ;
                let DGB;
                if CNO != 0.0 {
                    let CNP = CCG * CCI;
                    let CNQ = CCG * CCN;
                    let CNV;
                    if CCS != 0.0 {
                        let CNR = 5.184705528587072e21f64 * (C + (CCR - ES));
                        CNV = CNR;
                    } else {
                        let CNS = if CCR < -5e1f64 { 1.0 } else { 0.0 };
                        let CNW = if CNS != 0.0 {
                            CNT
                        } else {
                            let CNU = CCR.exp();
                            CNU
                        };
                        CNV = CNW;
                    }
                    let CNX = -DP;
                    let CNY = (CCE * (CNX - CCF)) + CCR;
                    let CNZ = if CNY > ES { 1.0 } else { 0.0 };
                    let COE;
                    if CNZ != 0.0 {
                        let COA = 5.184705528587072e21f64 * (C + (CNY - ES));
                        COE = COA;
                    } else {
                        let COB = if CNY < -5e1f64 { 1.0 } else { 0.0 };
                        let COF = if COB != 0.0 {
                            COC
                        } else {
                            let COD = CNY.exp();
                            COD
                        };
                        COE = COF;
                    }
                    let COK;
                    if CDJ != 0.0 {
                        let COG = 5.184705528587072e21f64 * (C + (CDB - ES));
                        COK = COG;
                    } else {
                        let COH = if CDB < -5e1f64 { 1.0 } else { 0.0 };
                        let COL = if COH != 0.0 {
                            COI
                        } else {
                            let COJ = CDB.exp();
                            COJ
                        };
                        COK = COL;
                    }
                    let COM = COE - COK;
                    let CON = (BYG * CNP) * BA;
                    let COO = (CDS * DP) + CCR;
                    let COP = if COO > ES { 1.0 } else { 0.0 };
                    let COU;
                    if COP != 0.0 {
                        let COQ = 5.184705528587072e21f64 * (C + (COO - ES));
                        COU = COQ;
                    } else {
                        let COR = if COO < -5e1f64 { 1.0 } else { 0.0 };
                        let COV = if COR != 0.0 {
                            COS
                        } else {
                            let COT = COO.exp();
                            COT
                        };
                        COU = COV;
                    }
                    let CRJ;
                    if CEB != 0.0 {
                        let COW = CON * ((COU - (CCK * COM)) - CNV);
                        CRJ = COW;
                    } else {
                        let COX = (CCE * ((-CCA) - CCF)) + CCR;
                        let COY = if COX > ES { 1.0 } else { 0.0 };
                        let CPD;
                        if COY != 0.0 {
                            let COZ = 5.184705528587072e21f64 * (C + (COX - ES));
                            CPD = COZ;
                        } else {
                            let CPA = if COX < -5e1f64 { 1.0 } else { 0.0 };
                            let CPE = if CPA != 0.0 {
                                CPB
                            } else {
                                let CPC = COX.exp();
                                CPC
                            };
                            CPD = CPE;
                        }
                        let CPF = CPD - COK;
                        let CPG = (CDS * CCA) + CCR;
                        let CPH = if CPG > ES { 1.0 } else { 0.0 };
                        let CPM;
                        if CPH != 0.0 {
                            let CPI = 5.184705528587072e21f64 * (C + (CPG - ES));
                            CPM = CPI;
                        } else {
                            let CPJ = if CPG < -5e1f64 { 1.0 } else { 0.0 };
                            let CPN = if CPJ != 0.0 {
                                CPK
                            } else {
                                let CPL = CPG.exp();
                                CPL
                            };
                            CPM = CPN;
                        }
                        let CPO = CCK * CPF;
                        let CPP = (CPM - CPO) - CNV;
                        let CPQ = CCK * COM;
                        let CPR = CON * ((COU - CPQ) - CNV);
                        let CPS = if CCC > A { 1.0 } else { 0.0 };
                        let CQU;
                        if CPS != 0.0 {
                            let CPT = (CCC * CCD) / AU;
                            let CPU = (CPT * CCA) + CCR;
                            let CPV = if CPU > ES { 1.0 } else { 0.0 };
                            let CQA;
                            if CPV != 0.0 {
                                let CPW = 5.184705528587072e21f64 * (C + (CPU - ES));
                                CQA = CPW;
                            } else {
                                let CPX = if CPU < -5e1f64 { 1.0 } else { 0.0 };
                                let CQB = if CPX != 0.0 {
                                    CPY
                                } else {
                                    let CPZ = CPU.exp();
                                    CPZ
                                };
                                CQA = CQB;
                            }
                            let CQC = (CQA - CPO) - CNV;
                            let CQD = (CPT * DP) + CCR;
                            let CQE = if CQD > ES { 1.0 } else { 0.0 };
                            let CQJ;
                            if CQE != 0.0 {
                                let CQF = 5.184705528587072e21f64 * (C + (CQD - ES));
                                CQJ = CQF;
                            } else {
                                let CQG = if CQD < -5e1f64 { 1.0 } else { 0.0 };
                                let CQK = if CQG != 0.0 {
                                    CQH
                                } else {
                                    let CQI = CQD.exp();
                                    CQI
                                };
                                CQJ = CQK;
                            }
                            let CQL = ((CON * CPP) / CQC) * ((CQJ - CPQ) - CNV);
                            CQU = CQL;
                        } else {
                            let CQM = CON * CPP;
                            CQU = CQM;
                        }
                        let CQN = (CCB * CCB) * AU;
                        let CQO = (DP - (CCA - (CQN / EX))) / CQN;
                        let CQP = if CQO > ES { 1.0 } else { 0.0 };
                        let CQS;
                        if CQP != 0.0 {
                            CQS = A;
                        } else {
                            let CQQ = if CQO < -5e1f64 { 1.0 } else { 0.0 };
                            let CQT = if CQQ != 0.0 {
                                C
                            } else {
                                let CQR = C / (C + (CQO.exp()));
                                CQR
                            };
                            CQS = CQT;
                        }
                        let CQV = (CQS * CPR) + ((C - CQS) * CQU);
                        CRJ = CQV;
                    }
                    let CQW = DP / CCL;
                    let CQZ = if DQ != 0.0 {
                        let CQX = ((CQW * CQW) + DZ).sqrt();
                        CQX
                    } else {
                        let CQY = CQW * (((EE / DZ) * CQW).tanh());
                        CQY
                    };
                    let CRA = (CGH * CNQ) * BA;
                    let CRB = CGJ * (CNX / ((C + (CQZ.powf(CCM))).powf(CGG)));
                    let CRC = if CRB > ES { 1.0 } else { 0.0 };
                    let CRH;
                    if CRC != 0.0 {
                        let CRD = 5.184705528587072e21f64 * (C + (CRB - ES));
                        CRH = CRD;
                    } else {
                        let CRE = if CRB < -5e1f64 { 1.0 } else { 0.0 };
                        let CRI = if CRE != 0.0 {
                            CRF
                        } else {
                            let CRG = CRB.exp();
                            CRG
                        };
                        CRH = CRI;
                    }
                    let CRK = CRJ + (CRA * (CRH - C));
                    let CRL = DK * CBX;
                    let CRM = CCG * CHB;
                    let CRN = CCG * CHG;
                    let CRS;
                    if CCS != 0.0 {
                        let CRO = 5.184705528587072e21f64 * (C + (CCR - ES));
                        CRS = CRO;
                    } else {
                        let CRP = if CCR < -5e1f64 { 1.0 } else { 0.0 };
                        let CRT = if CRP != 0.0 {
                            CRQ
                        } else {
                            let CRR = CCR.exp();
                            CRR
                        };
                        CRS = CRT;
                    }
                    let CRU = -CRL;
                    let CRV = (CGZ * (CRU - CHA)) + CCR;
                    let CRW = if CRV > ES { 1.0 } else { 0.0 };
                    let CSB;
                    if CRW != 0.0 {
                        let CRX = 5.184705528587072e21f64 * (C + (CRV - ES));
                        CSB = CRX;
                    } else {
                        let CRY = if CRV < -5e1f64 { 1.0 } else { 0.0 };
                        let CSC = if CRY != 0.0 {
                            CRZ
                        } else {
                            let CSA = CRV.exp();
                            CSA
                        };
                        CSB = CSC;
                    }
                    let CSH;
                    if CHZ != 0.0 {
                        let CSD = 5.184705528587072e21f64 * (C + (CHR - ES));
                        CSH = CSD;
                    } else {
                        let CSE = if CHR < -5e1f64 { 1.0 } else { 0.0 };
                        let CSI = if CSE != 0.0 {
                            CSF
                        } else {
                            let CSG = CHR.exp();
                            CSG
                        };
                        CSH = CSI;
                    }
                    let CSJ = CSB - CSH;
                    let CSK = (BYG * CRM) * BA;
                    let CSL = (CII * CRL) + CCR;
                    let CSM = if CSL > ES { 1.0 } else { 0.0 };
                    let CSR;
                    if CSM != 0.0 {
                        let CSN = 5.184705528587072e21f64 * (C + (CSL - ES));
                        CSR = CSN;
                    } else {
                        let CSO = if CSL < -5e1f64 { 1.0 } else { 0.0 };
                        let CSS = if CSO != 0.0 {
                            CSP
                        } else {
                            let CSQ = CSL.exp();
                            CSQ
                        };
                        CSR = CSS;
                    }
                    let CVG;
                    if CIR != 0.0 {
                        let CST = CSK * ((CSR - (CHD * CSJ)) - CRS);
                        CVG = CST;
                    } else {
                        let CSU = (CGZ * ((-CGV) - CHA)) + CCR;
                        let CSV = if CSU > ES { 1.0 } else { 0.0 };
                        let CTA;
                        if CSV != 0.0 {
                            let CSW = 5.184705528587072e21f64 * (C + (CSU - ES));
                            CTA = CSW;
                        } else {
                            let CSX = if CSU < -5e1f64 { 1.0 } else { 0.0 };
                            let CTB = if CSX != 0.0 {
                                CSY
                            } else {
                                let CSZ = CSU.exp();
                                CSZ
                            };
                            CTA = CTB;
                        }
                        let CTC = CTA - CSH;
                        let CTD = (CII * CGV) + CCR;
                        let CTE = if CTD > ES { 1.0 } else { 0.0 };
                        let CTJ;
                        if CTE != 0.0 {
                            let CTF = 5.184705528587072e21f64 * (C + (CTD - ES));
                            CTJ = CTF;
                        } else {
                            let CTG = if CTD < -5e1f64 { 1.0 } else { 0.0 };
                            let CTK = if CTG != 0.0 {
                                CTH
                            } else {
                                let CTI = CTD.exp();
                                CTI
                            };
                            CTJ = CTK;
                        }
                        let CTL = CHD * CTC;
                        let CTM = (CTJ - CTL) - CRS;
                        let CTN = CHD * CSJ;
                        let CTO = CSK * ((CSR - CTN) - CRS);
                        let CTP = if CGX > A { 1.0 } else { 0.0 };
                        let CUR;
                        if CTP != 0.0 {
                            let CTQ = (CGX * CGY) / AU;
                            let CTR = (CTQ * CGV) + CCR;
                            let CTS = if CTR > ES { 1.0 } else { 0.0 };
                            let CTX;
                            if CTS != 0.0 {
                                let CTT = 5.184705528587072e21f64 * (C + (CTR - ES));
                                CTX = CTT;
                            } else {
                                let CTU = if CTR < -5e1f64 { 1.0 } else { 0.0 };
                                let CTY = if CTU != 0.0 {
                                    CTV
                                } else {
                                    let CTW = CTR.exp();
                                    CTW
                                };
                                CTX = CTY;
                            }
                            let CTZ = (CTX - CTL) - CRS;
                            let CUA = (CTQ * CRL) + CCR;
                            let CUB = if CUA > ES { 1.0 } else { 0.0 };
                            let CUG;
                            if CUB != 0.0 {
                                let CUC = 5.184705528587072e21f64 * (C + (CUA - ES));
                                CUG = CUC;
                            } else {
                                let CUD = if CUA < -5e1f64 { 1.0 } else { 0.0 };
                                let CUH = if CUD != 0.0 {
                                    CUE
                                } else {
                                    let CUF = CUA.exp();
                                    CUF
                                };
                                CUG = CUH;
                            }
                            let CUI = ((CSK * CTM) / CTZ) * ((CUG - CTN) - CRS);
                            CUR = CUI;
                        } else {
                            let CUJ = CSK * CTM;
                            CUR = CUJ;
                        }
                        let CUK = (CGW * CGW) * AU;
                        let CUL = (CRL - (CGV - (CUK / EX))) / CUK;
                        let CUM = if CUL > ES { 1.0 } else { 0.0 };
                        let CUP;
                        if CUM != 0.0 {
                            CUP = A;
                        } else {
                            let CUN = if CUL < -5e1f64 { 1.0 } else { 0.0 };
                            let CUQ = if CUN != 0.0 {
                                C
                            } else {
                                let CUO = C / (C + (CUL.exp()));
                                CUO
                            };
                            CUP = CUQ;
                        }
                        let CUS = (CUP * CTO) + ((C - CUP) * CUR);
                        CVG = CUS;
                    }
                    let CUT = CRL / CHE;
                    let CUW = if DQ != 0.0 {
                        let CUU = ((CUT * CUT) + DZ).sqrt();
                        CUU
                    } else {
                        let CUV = CUT * (((EE / DZ) * CUT).tanh());
                        CUV
                    };
                    let CUX = (CGH * CRN) * BA;
                    let CUY = CKY * (CRU / ((C + (CUW.powf(CHF))).powf(CKW)));
                    let CUZ = if CUY > ES { 1.0 } else { 0.0 };
                    let CVE;
                    if CUZ != 0.0 {
                        let CVA = 5.184705528587072e21f64 * (C + (CUY - ES));
                        CVE = CVA;
                    } else {
                        let CVB = if CUY < -5e1f64 { 1.0 } else { 0.0 };
                        let CVF = if CVB != 0.0 {
                            CVC
                        } else {
                            let CVD = CUY.exp();
                            CVD
                        };
                        CVE = CVF;
                    }
                    let CVH = CVG + (CUX * (CVE - C));
                    if CLJ != 0.0 {
                        if CCS != 0.0 {
                        } else {
                            let CVI = if CCR < -5e1f64 { 1.0 } else { 0.0 };
                            if CVI != 0.0 {
                            } else {
                            }
                        }
                        if CNZ != 0.0 {
                        } else {
                            let CVJ = if CNY < -5e1f64 { 1.0 } else { 0.0 };
                            if CVJ != 0.0 {
                            } else {
                            }
                        }
                        if CDJ != 0.0 {
                        } else {
                            let CVK = if CDB < -5e1f64 { 1.0 } else { 0.0 };
                            if CVK != 0.0 {
                            } else {
                            }
                        }
                        if COP != 0.0 {
                        } else {
                            let CVL = if COO < -5e1f64 { 1.0 } else { 0.0 };
                            if CVL != 0.0 {
                            } else {
                            }
                        }
                        if CVM != 0.0 {
                        } else {
                            let CVN = (CCE * ((-CCA) - CCF)) + CCR;
                            let CVO = if CVN > ES { 1.0 } else { 0.0 };
                            if CVO != 0.0 {
                            } else {
                                let CVP = if CVN < -5e1f64 { 1.0 } else { 0.0 };
                                if CVP != 0.0 {
                                } else {
                                }
                            }
                            let CVQ = (CDS * CCA) + CCR;
                            let CVR = if CVQ > ES { 1.0 } else { 0.0 };
                            if CVR != 0.0 {
                            } else {
                                let CVS = if CVQ < -5e1f64 { 1.0 } else { 0.0 };
                                if CVS != 0.0 {
                                } else {
                                }
                            }
                            if CVT != 0.0 {
                                if CVR != 0.0 {
                                } else {
                                    let CVU = if CVQ < -5e1f64 { 1.0 } else { 0.0 };
                                    if CVU != 0.0 {
                                    } else {
                                    }
                                }
                                if COP != 0.0 {
                                } else {
                                    let CVV = if COO < -5e1f64 { 1.0 } else { 0.0 };
                                    if CVV != 0.0 {
                                    } else {
                                    }
                                }
                            } else {
                            }
                            let CVW = (CCB * CCB) * AU;
                            let CVX = (DP - (CCA - (CVW / EX))) / CVW;
                            let CVY = if CVX > ES { 1.0 } else { 0.0 };
                            if CVY != 0.0 {
                            } else {
                                let CVZ = if CVX < -5e1f64 { 1.0 } else { 0.0 };
                                if CVZ != 0.0 {
                                } else {
                                }
                            }
                        }
                        let CWA = DP / CLK;
                        let CWD = if DQ != 0.0 {
                            let CWB = ((CWA * CWA) + DZ).sqrt();
                            CWB
                        } else {
                            let CWC = CWA * (((EE / DZ) * CWA).tanh());
                            CWC
                        };
                        let CWE = (CLM / AU) * (CNX / ((C + (CWD.powf(CLL))).powf((C / CLL))));
                        let CWF = if CWE > ES { 1.0 } else { 0.0 };
                        if CWF != 0.0 {
                        } else {
                            let CWG = if CWE < -5e1f64 { 1.0 } else { 0.0 };
                            if CWG != 0.0 {
                            } else {
                            }
                        }
                        if CCS != 0.0 {
                        } else {
                            let CWH = if CCR < -5e1f64 { 1.0 } else { 0.0 };
                            if CWH != 0.0 {
                            } else {
                            }
                        }
                        if CRW != 0.0 {
                        } else {
                            let CWI = if CRV < -5e1f64 { 1.0 } else { 0.0 };
                            if CWI != 0.0 {
                            } else {
                            }
                        }
                        if CHZ != 0.0 {
                        } else {
                            let CWJ = if CHR < -5e1f64 { 1.0 } else { 0.0 };
                            if CWJ != 0.0 {
                            } else {
                            }
                        }
                        if CSM != 0.0 {
                        } else {
                            let CWK = if CSL < -5e1f64 { 1.0 } else { 0.0 };
                            if CWK != 0.0 {
                            } else {
                            }
                        }
                        if CWL != 0.0 {
                        } else {
                            let CWM = (CGZ * ((-CGV) - CHA)) + CCR;
                            let CWN = if CWM > ES { 1.0 } else { 0.0 };
                            if CWN != 0.0 {
                            } else {
                                let CWO = if CWM < -5e1f64 { 1.0 } else { 0.0 };
                                if CWO != 0.0 {
                                } else {
                                }
                            }
                            let CWP = (CII * CGV) + CCR;
                            let CWQ = if CWP > ES { 1.0 } else { 0.0 };
                            if CWQ != 0.0 {
                            } else {
                                let CWR = if CWP < -5e1f64 { 1.0 } else { 0.0 };
                                if CWR != 0.0 {
                                } else {
                                }
                            }
                            if CWS != 0.0 {
                                if CWQ != 0.0 {
                                } else {
                                    let CWT = if CWP < -5e1f64 { 1.0 } else { 0.0 };
                                    if CWT != 0.0 {
                                    } else {
                                    }
                                }
                                if CSM != 0.0 {
                                } else {
                                    let CWU = if CSL < -5e1f64 { 1.0 } else { 0.0 };
                                    if CWU != 0.0 {
                                    } else {
                                    }
                                }
                            } else {
                            }
                            let CWV = (CGW * CGW) * AU;
                            let CWW = (CRL - (CGV - (CWV / EX))) / CWV;
                            let CWX = if CWW > ES { 1.0 } else { 0.0 };
                            if CWX != 0.0 {
                            } else {
                                let CWY = if CWW < -5e1f64 { 1.0 } else { 0.0 };
                                if CWY != 0.0 {
                                } else {
                                }
                            }
                        }
                        let CWZ = CRL / CMM;
                        let CXC = if DQ != 0.0 {
                            let CXA = ((CWZ * CWZ) + DZ).sqrt();
                            CXA
                        } else {
                            let CXB = CWZ * (((EE / DZ) * CWZ).tanh());
                            CXB
                        };
                        let CXD = (CMO / AU) * (CRU / ((C + (CXC.powf(CMN))).powf((C / CMN))));
                        let CXE = if CXD > ES { 1.0 } else { 0.0 };
                        if CXE != 0.0 {
                        } else {
                            let CXF = if CXD < -5e1f64 { 1.0 } else { 0.0 };
                            if CXF != 0.0 {
                            } else {
                            }
                        }
                    } else {
                    }
                    DFP = CRK;
                    DFR = CON;
                    DFT = CRA;
                    DFX = CVH;
                    DFZ = CSK;
                    DGB = CUX;
                } else {
                    DFP = A;
                    DFR = A;
                    DFT = A;
                    DFX = A;
                    DFZ = A;
                    DGB = A;
                }
                DFO = DFP;
                DFQ = DFR;
                DFS = DFT;
                DFW = DFX;
                DFY = DFZ;
                DGA = DGB;
                DGD = CGT;
                DGE = CDR;
                DGF = CGI;
                DGH = CLI;
                DGI = CIH;
                DGJ = CKX;
            } else {
                DFO = A;
                DFQ = A;
                DFS = A;
                DFW = A;
                DFY = A;
                DGA = A;
                DGD = A;
                DGE = A;
                DGF = A;
                DGH = A;
                DGI = A;
                DGJ = A;
            }
            let CXG = if parameters[291] == C { 1.0 } else { 0.0 };
            if CXG != 0.0 {
                let CXH = DK * (DO - GI);
                let CXO = A / AU;
                let CXP = CXO * -0e0f64;
                let CXQ = if CXP > ES { 1.0 } else { 0.0 };
                if CXQ != 0.0 {
                } else {
                    let CXR = if CXP < -5e1f64 { 1.0 } else { 0.0 };
                    if CXR != 0.0 {
                    } else {
                    }
                }
                let CXS = -CXH;
                let CXT = OS * (CXS - CXM);
                let CXU = CXT + CXP;
                let CXV = -2.4e3f64 + CXP;
                let CXW = if CXU > ES { 1.0 } else { 0.0 };
                if CXW != 0.0 {
                } else {
                    let CXX = if CXU < -5e1f64 { 1.0 } else { 0.0 };
                    if CXX != 0.0 {
                    } else {
                    }
                }
                let CXY = if CXV > ES { 1.0 } else { 0.0 };
                if CXY != 0.0 {
                } else {
                    let CXZ = if CXV < -5e1f64 { 1.0 } else { 0.0 };
                    if CXZ != 0.0 {
                    } else {
                    }
                }
                let CYA = CXL / AU;
                let CYB = (CYA * CXH) + CXP;
                let CYC = if CYB > ES { 1.0 } else { 0.0 };
                if CYC != 0.0 {
                } else {
                    let CYD = if CYB < -5e1f64 { 1.0 } else { 0.0 };
                    if CYD != 0.0 {
                    } else {
                    }
                }
                let CYE = if CXK == C { 1.0 } else { 0.0 };
                if CYE != 0.0 {
                } else {
                    let CYF = (OS * ((-CXI) - CXM)) + CXP;
                    let CYG = if CYF > ES { 1.0 } else { 0.0 };
                    if CYG != 0.0 {
                    } else {
                        let CYH = if CYF < -5e1f64 { 1.0 } else { 0.0 };
                        if CYH != 0.0 {
                        } else {
                        }
                    }
                    let CYI = (CYA * CXI) + CXP;
                    let CYJ = if CYI > ES { 1.0 } else { 0.0 };
                    if CYJ != 0.0 {
                    } else {
                        let CYK = if CYI < -5e1f64 { 1.0 } else { 0.0 };
                        if CYK != 0.0 {
                        } else {
                        }
                    }
                    let CYL = if CXK > A { 1.0 } else { 0.0 };
                    if CYL != 0.0 {
                        let CYM = (CXK * CXL) / AU;
                        let CYN = (CYM * CXI) + CXP;
                        let CYO = if CYN > ES { 1.0 } else { 0.0 };
                        if CYO != 0.0 {
                        } else {
                            let CYP = if CYN < -5e1f64 { 1.0 } else { 0.0 };
                            if CYP != 0.0 {
                            } else {
                            }
                        }
                        let CYQ = (CYM * CXH) + CXP;
                        let CYR = if CYQ > ES { 1.0 } else { 0.0 };
                        if CYR != 0.0 {
                        } else {
                            let CYS = if CYQ < -5e1f64 { 1.0 } else { 0.0 };
                            if CYS != 0.0 {
                            } else {
                            }
                        }
                    } else {
                    }
                    let CYT = (CXJ * CXJ) * AU;
                    let CYU = (CXH - (CXI - (CYT / EX))) / CYT;
                    let CYV = if CYU > ES { 1.0 } else { 0.0 };
                    if CYV != 0.0 {
                    } else {
                        let CYW = if CYU < -5e1f64 { 1.0 } else { 0.0 };
                        if CYW != 0.0 {
                        } else {
                        }
                    }
                }
                let CYX = CXH / parameters[299];
                let CZA = if DQ != 0.0 {
                    let CYY = ((CYX * CYX) + DZ).sqrt();
                    CYY
                } else {
                    let CYZ = CYX * (((EE / DZ) * CYX).tanh());
                    CYZ
                };
                let CZB = (parameters[297] / AU) * (CXS / ((C + (CZA.powf(CXN))).powf((C / CXN))));
                let CZC = if CZB > ES { 1.0 } else { 0.0 };
                if CZC != 0.0 {
                } else {
                    let CZD = if CZB < -5e1f64 { 1.0 } else { 0.0 };
                    if CZD != 0.0 {
                    } else {
                    }
                }
                let CZE = if parameters[301] == C { 1.0 } else { 0.0 };
                if CZE != 0.0 {
                    let CZG = CXO * -0e0f64;
                    let CZH = if CZG > ES { 1.0 } else { 0.0 };
                    if CZH != 0.0 {
                    } else {
                        let CZI = if CZG < -5e1f64 { 1.0 } else { 0.0 };
                        if CZI != 0.0 {
                        } else {
                        }
                    }
                    let CZJ = CXT + CZG;
                    let CZK = -2.4e3f64 + CZG;
                    let CZL = if CZJ > ES { 1.0 } else { 0.0 };
                    if CZL != 0.0 {
                    } else {
                        let CZM = if CZJ < -5e1f64 { 1.0 } else { 0.0 };
                        if CZM != 0.0 {
                        } else {
                        }
                    }
                    let CZN = if CZK > ES { 1.0 } else { 0.0 };
                    if CZN != 0.0 {
                    } else {
                        let CZO = if CZK < -5e1f64 { 1.0 } else { 0.0 };
                        if CZO != 0.0 {
                        } else {
                        }
                    }
                    let CZP = (CXO * CXH) + CZG;
                    let CZQ = if CZP > ES { 1.0 } else { 0.0 };
                    if CZQ != 0.0 {
                    } else {
                        let CZR = if CZP < -5e1f64 { 1.0 } else { 0.0 };
                        if CZR != 0.0 {
                        } else {
                        }
                    }
                    if CZS != 0.0 {
                    } else {
                        let CZT = -2.404e3f64 + CZG;
                        let CZU = if CZT > ES { 1.0 } else { 0.0 };
                        if CZU != 0.0 {
                        } else {
                            let CZV = if CZT < -5e1f64 { 1.0 } else { 0.0 };
                            if CZV != 0.0 {
                            } else {
                            }
                        }
                        let CZW = CXO + CZG;
                        let CZX = if CZW > ES { 1.0 } else { 0.0 };
                        if CZX != 0.0 {
                        } else {
                            let CZY = if CZW < -5e1f64 { 1.0 } else { 0.0 };
                            if CZY != 0.0 {
                            } else {
                            }
                        }
                        if CZZ != 0.0 {
                            let DAA = 0e0f64 / AU;
                            let DAB = DAA + CZG;
                            let DAC = if DAB > ES { 1.0 } else { 0.0 };
                            if DAC != 0.0 {
                            } else {
                                let DAD = if DAB < -5e1f64 { 1.0 } else { 0.0 };
                                if DAD != 0.0 {
                                } else {
                                }
                            }
                            let DAE = (DAA * CXH) + CZG;
                            let DAF = if DAE > ES { 1.0 } else { 0.0 };
                            if DAF != 0.0 {
                            } else {
                                let DAG = if DAE < -5e1f64 { 1.0 } else { 0.0 };
                                if DAG != 0.0 {
                                } else {
                                }
                            }
                        } else {
                        }
                        let DAH = 1e2f64 * AU;
                        let DAI = (CXH - (C - (DAH / EX))) / DAH;
                        let DAJ = if DAI > ES { 1.0 } else { 0.0 };
                        if DAJ != 0.0 {
                        } else {
                            let DAK = if DAI < -5e1f64 { 1.0 } else { 0.0 };
                            if DAK != 0.0 {
                            } else {
                            }
                        }
                    }
                    let DAL = CXH / parameters[304];
                    let DAO = if DQ != 0.0 {
                        let DAM = ((DAL * DAL) + DZ).sqrt();
                        DAM
                    } else {
                        let DAN = DAL * (((EE / DZ) * DAL).tanh());
                        DAN
                    };
                    let DAP = (parameters[302] / AU) * (CXS / ((C + (DAO.powf(CZF))).powf((C / CZF))));
                    let DAQ = if DAP > ES { 1.0 } else { 0.0 };
                    if DAQ != 0.0 {
                    } else {
                        let DAR = if DAP < -5e1f64 { 1.0 } else { 0.0 };
                        if DAR != 0.0 {
                        } else {
                        }
                    }
                } else {
                }
                let DAS = if CXH <= (parameters[308] * parameters[306]) { 1.0 } else { 0.0 };
                if DAS != 0.0 {
                } else {
                    let DAU = if DAT >= C { 1.0 } else { 0.0 };
                    if DAU != 0.0 {
                        let DAV = if DAT >= EX { 1.0 } else { 0.0 };
                        if DAV != 0.0 {
                            let DAW = if DAT >= AZ { 1.0 } else { 0.0 };
                            if DAW != 0.0 {
                                let DAX = if DAT >= OS { 1.0 } else { 0.0 };
                                if DAX != 0.0 {
                                    let DAY = if DAT >= 5e0f64 { 1.0 } else { 0.0 };
                                    if DAY != 0.0 {
                                    } else {
                                    }
                                } else {
                                }
                            } else {
                            }
                        } else {
                        }
                    } else {
                    }
                }
                let DAZ = if (if parameters[310] != A { 1.0 } else { 0.0 }) != 0.0 && (if parameters[311] != A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                if DAZ != 0.0 {
                } else {
                }
            } else {
            }
            let DBA = DK * ((DR - GC) + (DR - DO));
            let DBB = DK * ((GC - DR) + (GC - DO));
            let DBC = if parameters[312] == C { 1.0 } else { 0.0 };
            if DBC != 0.0 {
                let DBD = if parameters[313] == A { 1.0 } else { 0.0 };
                let DBG;
                let DCX;
                if DBD != 0.0 {
                    let DBE = DK * ((DU - DS) + (DU - DO));
                    let DBF = DK * ((DS - DU) + (DS - DO));
                    DBG = DBE;
                    DCX = DBF;
                } else {
                    DBG = DBA;
                    DCX = DBB;
                }
                let DBJ = A / AU;
                let DBK = DBJ * (-CCQ);
                let DBL = if DBK > ES { 1.0 } else { 0.0 };
                if DBL != 0.0 {
                } else {
                    let DBM = if DBK < -5e1f64 { 1.0 } else { 0.0 };
                    if DBM != 0.0 {
                    } else {
                    }
                }
                let DBN = -DBG;
                let DBO = (DBH * (DBN - DBI)) + DBK;
                let DBP = ((-DBH) * DBI) + DBK;
                let DBQ = if DBO > ES { 1.0 } else { 0.0 };
                if DBQ != 0.0 {
                } else {
                    let DBR = if DBO < -5e1f64 { 1.0 } else { 0.0 };
                    if DBR != 0.0 {
                    } else {
                    }
                }
                let DBS = if DBP > ES { 1.0 } else { 0.0 };
                if DBS != 0.0 {
                } else {
                    let DBT = if DBP < -5e1f64 { 1.0 } else { 0.0 };
                    if DBT != 0.0 {
                    } else {
                    }
                }
                let DBU = (DBJ * DBG) + DBK;
                let DBV = if DBU > ES { 1.0 } else { 0.0 };
                if DBV != 0.0 {
                } else {
                    let DBW = if DBU < -5e1f64 { 1.0 } else { 0.0 };
                    if DBW != 0.0 {
                    } else {
                    }
                }
                let DBX = if CCC == C { 1.0 } else { 0.0 };
                if DBX != 0.0 {
                } else {
                    let DBY = (DBH * ((-CCA) - DBI)) + DBK;
                    let DBZ = if DBY > ES { 1.0 } else { 0.0 };
                    if DBZ != 0.0 {
                    } else {
                        let DCA = if DBY < -5e1f64 { 1.0 } else { 0.0 };
                        if DCA != 0.0 {
                        } else {
                        }
                    }
                    let DCB = (DBJ * CCA) + DBK;
                    let DCC = if DCB > ES { 1.0 } else { 0.0 };
                    if DCC != 0.0 {
                    } else {
                        let DCD = if DCB < -5e1f64 { 1.0 } else { 0.0 };
                        if DCD != 0.0 {
                        } else {
                        }
                    }
                    let DCE = if CCC > A { 1.0 } else { 0.0 };
                    if DCE != 0.0 {
                        let DCF = (CCC * A) / AU;
                        let DCG = (DCF * CCA) + DBK;
                        let DCH = if DCG > ES { 1.0 } else { 0.0 };
                        if DCH != 0.0 {
                        } else {
                            let DCI = if DCG < -5e1f64 { 1.0 } else { 0.0 };
                            if DCI != 0.0 {
                            } else {
                            }
                        }
                        let DCJ = (DCF * DBG) + DBK;
                        let DCK = if DCJ > ES { 1.0 } else { 0.0 };
                        if DCK != 0.0 {
                        } else {
                            let DCL = if DCJ < -5e1f64 { 1.0 } else { 0.0 };
                            if DCL != 0.0 {
                            } else {
                            }
                        }
                    } else {
                    }
                    let DCM = (CCB * CCB) * AU;
                    let DCN = (DBG - (CCA - (DCM / EX))) / DCM;
                    let DCO = if DCN > ES { 1.0 } else { 0.0 };
                    if DCO != 0.0 {
                    } else {
                        let DCP = if DCN < -5e1f64 { 1.0 } else { 0.0 };
                        if DCP != 0.0 {
                        } else {
                        }
                    }
                }
                let DCQ = DBG / CCL;
                let DCT = if DQ != 0.0 {
                    let DCR = ((DCQ * DCQ) + DZ).sqrt();
                    DCR
                } else {
                    let DCS = DCQ * (((EE / DZ) * DCQ).tanh());
                    DCS
                };
                let DCU = (CCP / AU) * (DBN / ((C + (DCT.powf(CCM))).powf((C / CCM))));
                let DCV = if DCU > ES { 1.0 } else { 0.0 };
                if DCV != 0.0 {
                } else {
                    let DCW = if DCU < -5e1f64 { 1.0 } else { 0.0 };
                    if DCW != 0.0 {
                    } else {
                    }
                }
                if DBL != 0.0 {
                } else {
                    let DDA = if DBK < -5e1f64 { 1.0 } else { 0.0 };
                    if DDA != 0.0 {
                    } else {
                    }
                }
                let DDB = -DCX;
                let DDC = (DCY * (DDB - DCZ)) + DBK;
                let DDD = ((-DCY) * DCZ) + DBK;
                let DDE = if DDC > ES { 1.0 } else { 0.0 };
                if DDE != 0.0 {
                } else {
                    let DDF = if DDC < -5e1f64 { 1.0 } else { 0.0 };
                    if DDF != 0.0 {
                    } else {
                    }
                }
                let DDG = if DDD > ES { 1.0 } else { 0.0 };
                if DDG != 0.0 {
                } else {
                    let DDH = if DDD < -5e1f64 { 1.0 } else { 0.0 };
                    if DDH != 0.0 {
                    } else {
                    }
                }
                let DDI = (DBJ * DCX) + DBK;
                let DDJ = if DDI > ES { 1.0 } else { 0.0 };
                if DDJ != 0.0 {
                } else {
                    let DDK = if DDI < -5e1f64 { 1.0 } else { 0.0 };
                    if DDK != 0.0 {
                    } else {
                    }
                }
                let DDL = if CGX == C { 1.0 } else { 0.0 };
                if DDL != 0.0 {
                } else {
                    let DDM = (DCY * ((-CGV) - DCZ)) + DBK;
                    let DDN = if DDM > ES { 1.0 } else { 0.0 };
                    if DDN != 0.0 {
                    } else {
                        let DDO = if DDM < -5e1f64 { 1.0 } else { 0.0 };
                        if DDO != 0.0 {
                        } else {
                        }
                    }
                    let DDP = (DBJ * CGV) + DBK;
                    let DDQ = if DDP > ES { 1.0 } else { 0.0 };
                    if DDQ != 0.0 {
                    } else {
                        let DDR = if DDP < -5e1f64 { 1.0 } else { 0.0 };
                        if DDR != 0.0 {
                        } else {
                        }
                    }
                    let DDS = if CGX > A { 1.0 } else { 0.0 };
                    if DDS != 0.0 {
                        let DDT = (CGX * A) / AU;
                        let DDU = (DDT * CGV) + DBK;
                        let DDV = if DDU > ES { 1.0 } else { 0.0 };
                        if DDV != 0.0 {
                        } else {
                            let DDW = if DDU < -5e1f64 { 1.0 } else { 0.0 };
                            if DDW != 0.0 {
                            } else {
                            }
                        }
                        let DDX = (DDT * DCX) + DBK;
                        let DDY = if DDX > ES { 1.0 } else { 0.0 };
                        if DDY != 0.0 {
                        } else {
                            let DDZ = if DDX < -5e1f64 { 1.0 } else { 0.0 };
                            if DDZ != 0.0 {
                            } else {
                            }
                        }
                    } else {
                    }
                    let DEA = (CGW * CGW) * AU;
                    let DEB = (DCX - (CGV - (DEA / EX))) / DEA;
                    let DEC = if DEB > ES { 1.0 } else { 0.0 };
                    if DEC != 0.0 {
                    } else {
                        let DED = if DEB < -5e1f64 { 1.0 } else { 0.0 };
                        if DED != 0.0 {
                        } else {
                        }
                    }
                }
                let DEE = DCX / CHE;
                let DEH = if DQ != 0.0 {
                    let DEF = ((DEE * DEE) + DZ).sqrt();
                    DEF
                } else {
                    let DEG = DEE * (((EE / DZ) * DEE).tanh());
                    DEG
                };
                let DEI = (CHI / AU) * (DDB / ((C + (DEH.powf(CHF))).powf((C / CHF))));
                let DEJ = if DEI > ES { 1.0 } else { 0.0 };
                if DEJ != 0.0 {
                } else {
                    let DEK = if DEI < -5e1f64 { 1.0 } else { 0.0 };
                    if DEK != 0.0 {
                    } else {
                    }
                }
                if DBD != 0.0 {
                } else {
                }
            } else {
            }
            if AI != 0.0 {
            } else {
            }
            if X != 0.0 {
            } else {
            }
            let DEP = if (if AQ >= W { 1.0 } else { 0.0 }) != 0.0 && (if AQ > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            if DEP != 0.0 {
            } else {
            }
            let DER = if (if AR >= W { 1.0 } else { 0.0 }) != 0.0 && (if AR > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            if DER != 0.0 {
            } else {
            }
            let DEU = ((DEQ - DU) - DES) / DET;
            let DEV = if DEU > ES { 1.0 } else { 0.0 };
            if DEV != 0.0 {
            } else {
                let DEW = if DEU < -5e1f64 { 1.0 } else { 0.0 };
                if DEW != 0.0 {
                } else {
                }
            }
            let DEX = ((DEQ - DS) - DES) / DET;
            let DEY = if DEX > ES { 1.0 } else { 0.0 };
            if DEY != 0.0 {
            } else {
                let DEZ = if DEX < -5e1f64 { 1.0 } else { 0.0 };
                if DEZ != 0.0 {
                } else {
                }
            }
            let DFA = ((DU - DS) - DES) / DET;
            let DFB = if DFA > ES { 1.0 } else { 0.0 };
            if DFB != 0.0 {
            } else {
                let DFC = if DFA < -5e1f64 { 1.0 } else { 0.0 };
                if DFC != 0.0 {
                } else {
                }
            }
            let DFD = ((GP - DU) - DES) / DET;
            let DFE = if DFD > ES { 1.0 } else { 0.0 };
            if DFE != 0.0 {
            } else {
                let DFF = if DFD < -5e1f64 { 1.0 } else { 0.0 };
                if DFF != 0.0 {
                } else {
                }
            }
            let DFG = ((GP - DS) - DES) / DET;
            let DFH = if DFG > ES { 1.0 } else { 0.0 };
            if DFH != 0.0 {
            } else {
                let DFI = if DFG < -5e1f64 { 1.0 } else { 0.0 };
                if DFI != 0.0 {
                } else {
                }
            }
            let DFJ = ((DEQ - GP) - DES) / DET;
            let DFK = if DFJ > ES { 1.0 } else { 0.0 };
            if DFK != 0.0 {
            } else {
                let DFL = if DFJ < -5e1f64 { 1.0 } else { 0.0 };
                if DFL != 0.0 {
                } else {
                }
            }
            let DFM = if parameters[347] == C { 1.0 } else { 0.0 };
            let DHL;
            let DHM;
            let DHN;
            let DHO;
            let DHP;
            let DHQ;
            let DHR;
            let DHS;
            let DHT;
            let DHU;
            let DHV;
            let DHW;
            let DHX;
            let DHY;
            let DIA;
            let DIC;
            let DIE;
            let DIG;
            let DII;
            let DIK;
            let DIM;
            let DIO;
            let DIQ;
            let DIS;
            let DIU;
            let DIW;
            let DIY;
            let DJA;
            let DJC;
            let DJE;
            let DJG;
            let DJI;
            let DJK;
            if DFM != 0.0 {
                let DFN = parameters[348] * AT;
                let DFU = DFN * ((DFO + (EX * (DFQ + DFS))).abs());
                let DFV = parameters[349] * AT;
                let DGC = DFV * ((DFW + (EX * (DFY + DGA))).abs());
                let DGG = DFN * ((DGD + (EX * (DGE + DGF))).abs());
                let DGK = DFV * ((DGH + (EX * (DGI + DGJ))).abs());
                let DGL = (parameters[350] * (CBJ / BRM)) * (((BYK.abs()) / CBJ).powf(parameters[351]));
                let DGM = if BYK < A { 1.0 } else { 0.0 };
                let DGO = if DGM != 0.0 {
                    let DGN = -DGL;
                    DGN
                } else {
                    DGL
                };
                let DGQ = (((5.52248e-23f64 * Z) * DOY[2]) * (CBL + CBM)) / (CBK * BI);
                let DGR = if Q != A { 1.0 } else { 0.0 };
                let DGS = if AHM != 0.0 && DGR != 0.0 { 1.0 } else { 0.0 };
                let DHZ;
                let DIB;
                if DGS != 0.0 {
                    let DGT = (5.52248e-23f64 * Z) / ((Q * AHL) / CBJ);
                    DHZ = C;
                    DIB = DGT;
                } else {
                    DHZ = A;
                    DIB = A;
                }
                let DGU = if ANO != 0.0 && DGR != 0.0 { 1.0 } else { 0.0 };
                let DID;
                let DIF;
                if DGU != 0.0 {
                    let DGV = (5.52248e-23f64 * Z) / ((Q * ANN) / CBJ);
                    DID = C;
                    DIF = DGV;
                } else {
                    DID = A;
                    DIF = A;
                }
                let DGW = if ATQ != 0.0 && DGR != 0.0 { 1.0 } else { 0.0 };
                let DIH;
                let DIJ;
                if DGW != 0.0 {
                    let DGX = (5.52248e-23f64 * Z) / ((Q * ATP) / CBJ);
                    DIH = C;
                    DIJ = DGX;
                } else {
                    DIH = A;
                    DIJ = A;
                }
                let DGY = if AZS != 0.0 && DGR != 0.0 { 1.0 } else { 0.0 };
                let DIL;
                let DIN;
                if DGY != 0.0 {
                    let DGZ = (5.52248e-23f64 * Z) / ((Q * AZR) / CBJ);
                    DIL = C;
                    DIN = DGZ;
                } else {
                    DIL = A;
                    DIN = A;
                }
                let DHA = if ABK != 0.0 && DGR != 0.0 { 1.0 } else { 0.0 };
                let DIP;
                let DIR;
                if DHA != 0.0 {
                    let DHB = (5.52248e-23f64 * Z) / ((Q * ABJ) / CBJ);
                    DIP = C;
                    DIR = DHB;
                } else {
                    DIP = A;
                    DIR = A;
                }
                let DHC = if VI != 0.0 && DGR != 0.0 { 1.0 } else { 0.0 };
                let DIT;
                let DIV;
                if DHC != 0.0 {
                    let DHD = (5.52248e-23f64 * Z) / ((Q * VH) / CBJ);
                    DIT = C;
                    DIV = DHD;
                } else {
                    DIT = A;
                    DIV = A;
                }
                let DHE = if PG != 0.0 && DGR != 0.0 { 1.0 } else { 0.0 };
                let DIX;
                let DIZ;
                if DHE != 0.0 {
                    let DHF = (5.52248e-23f64 * Z) / ((Q * PF) / CBJ);
                    DIX = C;
                    DIZ = DHF;
                } else {
                    DIX = A;
                    DIZ = A;
                }
                let DHG = if IV != 0.0 && DGR != 0.0 { 1.0 } else { 0.0 };
                let DJB;
                let DJD;
                if DHG != 0.0 {
                    let DHH = (5.52248e-23f64 * Z) / ((Q * IT) / CBJ);
                    DJB = C;
                    DJD = DHH;
                } else {
                    DJB = A;
                    DJD = A;
                }
                let DJF;
                let DJH;
                if X != 0.0 {
                    let DHI = (5.52248e-23f64 * Z) / DEN;
                    DJF = C;
                    DJH = DHI;
                } else {
                    DJF = A;
                    DJH = A;
                }
                let DJJ;
                let DJL;
                if AI != 0.0 {
                    let DHJ = (5.52248e-23f64 * Z) / DEL;
                    DJJ = C;
                    DJL = DHJ;
                } else {
                    DJJ = A;
                    DJL = A;
                }
                DHL = C;
                DHM = DFU;
                DHN = C;
                DHO = DGC;
                DHP = C;
                DHQ = DGG;
                DHR = C;
                DHS = DGK;
                DHT = C;
                DHU = DGO;
                DHV = DGP;
                DHW = C;
                DHX = DGQ;
                DHY = DHZ;
                DIA = DIB;
                DIC = DID;
                DIE = DIF;
                DIG = DIH;
                DII = DIJ;
                DIK = DIL;
                DIM = DIN;
                DIO = DIP;
                DIQ = DIR;
                DIS = DIT;
                DIU = DIV;
                DIW = DIX;
                DIY = DIZ;
                DJA = DJB;
                DJC = DJD;
                DJE = DJF;
                DJG = DJH;
                DJI = DJJ;
                DJK = DJL;
            } else {
                DHL = A;
                DHM = A;
                DHN = A;
                DHO = A;
                DHP = A;
                DHQ = A;
                DHR = A;
                DHS = A;
                DHT = A;
                DHU = A;
                DHV = A;
                DHW = A;
                DHX = A;
                DHY = A;
                DIA = A;
                DIC = A;
                DIE = A;
                DIG = A;
                DII = A;
                DIK = A;
                DIM = A;
                DIO = A;
                DIQ = A;
                DIS = A;
                DIU = A;
                DIW = A;
                DIY = A;
                DJA = A;
                DJC = A;
                DJE = A;
                DJG = A;
                DJI = A;
                DJK = A;
            }
            if AI != 0.0 {
            } else {
            }
            if X != 0.0 {
            } else {
            }
            let DHK = if parameters[320] > A { 1.0 } else { 0.0 };
            if DHK != 0.0 {
            } else {
            }
        if DHL == 0.0 {
            if !visitor.visit(0, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = DHM;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 0, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 0, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(0, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if DHN == 0.0 {
            if !visitor.visit(1, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = DHO;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(1, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if DHP == 0.0 {
            if !visitor.visit(2, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = DHQ;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 2, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 2, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(2, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if DHR == 0.0 {
            if !visitor.visit(3, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = DHS;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 3, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 3, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(3, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if DHT == 0.0 {
            if !visitor.visit(4, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = DHU;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 4, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = Some(DHV);
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 4, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 4, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(4, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if DHW == 0.0 {
            if !visitor.visit(5, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = DHX;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 5, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 5, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(5, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if DHY == 0.0 {
            if !visitor.visit(6, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = DIA;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 6, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 6, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(6, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if DIC == 0.0 {
            if !visitor.visit(7, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = DIE;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 7, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 7, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(7, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if DIG == 0.0 {
            if !visitor.visit(8, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = DII;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 8, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 8, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(8, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if DIK == 0.0 {
            if !visitor.visit(9, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = DIM;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 9, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 9, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(9, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if DIO == 0.0 {
            if !visitor.visit(10, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = DIQ;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 10, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 10, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(10, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if DIS == 0.0 {
            if !visitor.visit(11, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = DIU;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 11, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 11, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(11, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if DIW == 0.0 {
            if !visitor.visit(12, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = DIY;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 12, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 12, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(12, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if DJA == 0.0 {
            if !visitor.visit(13, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = DJC;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 13, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 13, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(13, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if DJE == 0.0 {
            if !visitor.visit(14, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = DJG;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 14, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 14, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(14, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if DJI == 0.0 {
            if !visitor.visit(15, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = DJK;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 15, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 15, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(15, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        Ok(())
    }
}
