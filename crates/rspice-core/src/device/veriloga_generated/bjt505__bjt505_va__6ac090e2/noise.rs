#![allow(dead_code, non_snake_case, unused_parens, unused_variables)]

use super::state::Instance;
use crate::device::veriloga_generated::GeneratedEvalContext;
pub use crate::device::veriloga_generated::{GeneratedNoiseDescriptor, GeneratedNoiseEndpoint, GeneratedNoiseEvaluation, GeneratedNoiseEvaluationError, GeneratedNoiseEvaluationRef, GeneratedNoiseKind, GeneratedNoiseVisitor};

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
            let v0 = 0e0f64;
            let v1 = parameters[3];
            let v2 = 1e0f64;
            let v4 = 7.03e7f64;
            let v5 = 1.23e8f64;
            let v6 = 1.58e8f64;
            let v7 = 2.04e8f64;
            let v8 = parameters[33];
            let v10 = parameters[4];
            let v11 = 2.7315e2f64;
            let v13 = temperature;
            let v14 = parameters[0];
            let v16 = parameters[150];
            let v18 = 1e-12f64;
            let v20 = parameters[1];
            let v23 = parameters[134];
            let v25 = 1e-3f64;
            let v26 = 2e0f64;
            let v27 = parameters[67];
            let v31 = parameters[114];
            let v32 = parameters[115];
            let v35 = parameters[116];
            let v39 = 5e-2f64;
            let v41 = 1e-1f64;
            let v56 = parameters[66];
            let v58 = parameters[71];
            let v59 = parameters[72];
            let v63 = parameters[117];
            let v64 = parameters[118];
            let v67 = parameters[119];
            let v87 = parameters[83];
            let v91 = 8.617086918058125e-5f64;
            let v139 = 3e0f64;
            let v140 = -3e0f64;
            let v146 = parameters[105];
            let v163 = -3e0f64;
            let v166 = parameters[64];
            let v169 = parameters[110];
            let v186 = -3e0f64;
            let v189 = parameters[80];
            let v207 = -3e0f64;
            let v227 = -3e0f64;
            let v246 = -3e0f64;
            let v249 = parameters[27];
            let v252 = parameters[109];
            let v269 = -3e0f64;
            let v272 = parameters[138];
            let v275 = parameters[140];
            let v301 = parameters[139];
            let v302 = parameters[75];
            let v310 = parameters[70];
            let v313 = parameters[54];
            let v314 = parameters[97];
            let v319 = parameters[56];
            let v320 = parameters[98];
            let v321 = parameters[96];
            let v326 = parameters[55];
            let v327 = parameters[101];
            let v332 = parameters[57];
            let v333 = parameters[102];
            let v337 = parameters[58];
            let v338 = parameters[104];
            let v342 = parameters[59];
            let v344 = parameters[60];
            let v345 = parameters[99];
            let v349 = parameters[122];
            let v351 = parameters[10];
            let v370 = 6.931471805599453e-4f64;
            let v372 = parameters[123];
            let v374 = parameters[11];
            let v393 = 6.931471805599453e-4f64;
            let v395 = parameters[43];
            let v396 = parameters[124];
            let v400 = 1e-6f64;
            let v403 = 5e-1f64;
            let v404 = 5e-7f64;
            let v413 = parameters[9];
            let v414 = 4e0f64;
            let v417 = parameters[121];
            let v429 = parameters[12];
            let v434 = parameters[30];
            let v435 = parameters[103];
            let v440 = parameters[20];
            let v441 = 6e0f64;
            let v442 = parameters[21];
            let v448 = parameters[113];
            let v454 = parameters[31];
            let v455 = parameters[32];
            let v466 = parameters[16];
            let v470 = parameters[17];
            let v474 = parameters[111];
            let v480 = parameters[18];
            let v481 = parameters[19];
            let v488 = parameters[24];
            let v490 = parameters[25];
            let v491 = parameters[107];
            let v497 = parameters[28];
            let v498 = parameters[106];
            let v503 = parameters[26];
            let v504 = parameters[108];
            let v510 = parameters[29];
            let v516 = parameters[112];
            let v521 = parameters[22];
            let v522 = parameters[23];
            let v531 = parameters[145];
            let v532 = parameters[146];
            let v540 = parameters[151];
            let v543 = parameters[153];
            let v549 = -5e-1f64;
            let v552 = parameters[35];
            let v561 = parameters[34];
            let v573 = -5e-1f64;
            let v576 = parameters[37];
            let v585 = parameters[36];
            let v597 = parameters[14];
            let v600 = parameters[13];
            let v603 = parameters[133];
            let v604 = parameters[141];
            let v613 = parameters[135];
            let v618 = parameters[87];
            let v624 = parameters[88];
            let v629 = parameters[89];
            let v634 = parameters[90];
            let v635 = parameters[100];
            let v640 = 3e2f64;
            let v642 = 5.25e2f64;
            let v645 = 7.2e-4f64;
            let v648 = 1.6e-6f64;
            let v653 = 1.081e0f64;
            let v655 = parameters[92];
            let v666 = node_potentials[6];
            let v667 = node_potentials[7];
            let v670 = node_potentials[8];
            let v673 = node_potentials[4];
            let v676 = node_potentials[5];
            let v681 = node_potentials[3];
            let v686 = node_potentials[2];
            let v687 = node_potentials[1];
            let v692 = node_potentials[0];
            let v695 = node_potentials[10];
            let v698 = node_potentials[9];
            let v712 = parameters[147];
            let v808 = parameters[149];
            let v819 = 1e2f64;
            let v835 = 2e-1f64;
            let v850 = parameters[62];
            let v851 = parameters[61];
            let v861 = parameters[63];
            let v876 = -1e0f64;
            let v919 = parameters[148];
            let v937 = parameters[73];
            let v953 = 1e-5f64;
            let v957 = 1e-40f64;
            let v973 = -1e0f64;
            let v1004 = parameters[74];
            let v1012 = -1e0f64;
            let v1036 = parameters[76];
            let v1091 = 1.0000000000000002e-2f64;
            let v1095 = 5.000000000000001e-3f64;
            let v1109 = parameters[15];
            let v1115 = 1e-4f64;
            let v1129 = parameters[152];
            let v1140 = parameters[154];
            let v1155 = parameters[155];
            let v1178 = 1e3f64;
            let v1180 = 4e1f64;
            let v1183 = 2.3538526683702e17f64;
            let v1211 = parameters[93];
            let v1313 = 1e-30f64;
            let v1316 = -2e0f64;
            let v1332 = 1.6666666666666666e-1f64;
            let v1338 = -1e-3f64;
            let v1354 = 3.333333333333333e-1f64;
            let v1356 = 2.5e-1f64;
            let v1391 = -2e0f64;
            let v1412 = -1e-3f64;
            let v1453 = parameters[8];
            let v1455 = parameters[143];
            let v1463 = parameters[144];
            let v1504 = parameters[5];
            let v1558 = 1.21e-2f64;
            let v1561 = 6.05e-3f64;
            let v1580 = parameters[84];
            let v1583 = 1e-6f64;
            let v1584 = 1e-12f64;
            let v1585 = -1e0f64;
            let v1587 = -1e0f64;
            let v1590 = -1e0f64;
            let v1593 = 5e-13f64;
            let v1596 = -1e0f64;
            let v1602 = -1e0f64;
            let v1606 = parameters[82];
            let v1610 = parameters[81];
            let v1640 = 1.0000000000000002e-2f64;
            let v1643 = 5.000000000000001e-3f64;
            let v1660 = parameters[39];
            let v1662 = parameters[44];
            let v1665 = parameters[42];
            let v1678 = parameters[41];
            let v1687 = parameters[40];
            let v1694 = parameters[46];
            let v1696 = parameters[45];
            let v1704 = parameters[7];
            let v1724 = parameters[47];
            let v1754 = 1e-7f64;
            let v1780 = parameters[48];
            let v1784 = parameters[49];
            let v1788 = parameters[52];
            let v1792 = parameters[51];
            let v1807 = parameters[50];
            let v1831 = parameters[53];
            let v1867 = parameters[77];
            let v1901 = -1e0f64;
            let v1907 = parameters[85];
            let v1914 = parameters[79];
            let v1918 = parameters[91];
            let v1966 = parameters[6];
            let v1988 = parameters[69];
            let v1993 = parameters[78];
            let v2011 = 5.5224904e-23f64;
            let v2020 = 5e0f64;
            let v2026 = 3.2043836e-19f64;
            let v2029 = parameters[130];
            let v2033 = 3.2043836e-19f64;
            let v2039 = parameters[131];
            let v2042 = 3.2043836e-19f64;
            let v2049 = parameters[128];
            let v2051 = parameters[126];
            let v2058 = parameters[129];
            let v2060 = parameters[127];
            let v2065 = 3.2043836e-19f64;
            let v2068 = 3.2043836e-19f64;
            let v2075 = 3.2043836e-19f64;
            let v2078 = 3.2043836e-19f64;
            let v2089 = 3.2043836e-19f64;
            let v2102 = 3.2043836e-19f64;
            let v2105 = 3.2043836e-19f64;
            let v2108 = 3.2043836e-19f64;
            let v3 = if v1 == v2 { 1.0 } else { 0.0 };
            let v644: f64;
            let v1757: f64;
            if v3 != 0.0 {
                v644 = v5;
                v1757 = v4;
            } else {
                v644 = v7;
                v1757 = v6;
            }
            let v9 = v2 - v8;
            let v12 = v10 + v11;
            let v15 = v13 + v14;
            let v17 = if v16 == v0 { 1.0 } else { 0.0 };
            let v19: f64;
            if v17 != 0.0 {
                v19 = v18;
            } else {
                v19 = v16;
            }
            let v21 = v19 * v20;
            let v22 = v2 / v21;
            let v24 = if v23 > v0 { 1.0 } else { 0.0 };
            if v24 != 0.0 {
            } else {
            }
            let v29 = v26.powf((v26 - v27));
            let v30 = v2 / v29;
            let v38 = v31 + (((v32 * v12) * v12) / (v12 + v35));
            let v42 = (v38 - v39) / v41;
            let v43 = if v38 < v39 { 1.0 } else { 0.0 };
            let v99: f64;
            if v43 != 0.0 {
                let v48 = v39 + (v41 * ((v2 + (v42.exp())).ln()));
                v99 = v48;
            } else {
                let v54 = v38 + (v41 * ((v2 + ((-v42).exp())).ln()));
                v99 = v54;
            }
            let v55 = v2 / v31;
            let v57 = v2 / v56;
            let v61 = v26.powf((v26 - v59));
            let v62 = v2 / v61;
            let v70 = v63 + (((v64 * v12) * v12) / (v12 + v67));
            let v72 = (v70 - v39) / v41;
            let v73 = if v70 < v39 { 1.0 } else { 0.0 };
            let v119: f64;
            if v73 != 0.0 {
                let v78 = v39 + (v41 * ((v2 + (v72.exp())).ln()));
                v119 = v78;
            } else {
                let v84 = v70 + (v41 * ((v2 + ((-v72).exp())).ln()));
                v119 = v84;
            }
            let v85 = v2 / v63;
            let v86 = v2 / v58;
            let v89 = v2 - (v2 / v87);
            let v90 = v15 / v12;
            let v92 = v91 * v15;
            let v94 = v2 / v92;
            let v96 = v94 - (v2 / (v91 * v12));
            let v97 = v15 - v12;
            let v98 = v90.ln();
            let v104 = v99 - (((v32 * v15) * v15) / (v15 + v35));
            let v106 = (v104 - v39) / v41;
            let v107 = if v104 < v39 { 1.0 } else { 0.0 };
            let v547: f64;
            if v107 != 0.0 {
                let v112 = v39 + (v41 * ((v2 + (v106.exp())).ln()));
                v547 = v112;
            } else {
                let v118 = v104 + (v41 * ((v2 + ((-v106).exp())).ln()));
                v547 = v118;
            }
            let v124 = v119 - (((v64 * v15) * v15) / (v15 + v67));
            let v126 = (v124 - v39) / v41;
            let v127 = if v124 < v39 { 1.0 } else { 0.0 };
            let v571: f64;
            if v127 != 0.0 {
                let v132 = v39 + (v41 * ((v2 + (v126.exp())).ln()));
                v571 = v132;
            } else {
                let v138 = v124 + (v41 * ((v2 + ((-v126).exp())).ln()));
                v571 = v138;
            }
            let v145 = v2 - v90;
            let v148 = (((v140 * v92) * v98) + (v56 * v90)) + (v145 * v146);
            let v150 = (v39 - v148) / v92;
            let v151 = if v39 < v148 { 1.0 } else { 0.0 };
            let v292: f64;
            if v151 != 0.0 {
                let v156 = v148 + (v92 * ((v2 + (v150.exp())).ln()));
                v292 = v156;
            } else {
                let v162 = v39 + (v92 * ((v2 + ((-v150).exp())).ln()));
                v292 = v162;
            }
            let v170 = v145 * v169;
            let v171 = (((v163 * v92) * v98) + (v166 * v90)) + v170;
            let v173 = (v39 - v171) / v92;
            let v174 = if v39 < v171 { 1.0 } else { 0.0 };
            let v769: f64;
            if v174 != 0.0 {
                let v179 = v171 + (v92 * ((v2 + (v173.exp())).ln()));
                v769 = v179;
            } else {
                let v185 = v39 + (v92 * ((v2 + ((-v173).exp())).ln()));
                v769 = v185;
            }
            let v192 = (((v186 * v92) * v98) + (v189 * v90)) + v170;
            let v194 = (v39 - v192) / v92;
            let v195 = if v39 < v192 { 1.0 } else { 0.0 };
            let v1916: f64;
            if v195 != 0.0 {
                let v200 = v192 + (v92 * ((v2 + (v194.exp())).ln()));
                v1916 = v200;
            } else {
                let v206 = v39 + (v92 * ((v2 + ((-v194).exp())).ln()));
                v1916 = v206;
            }
            let v210 = v58 * v90;
            let v212 = (((v207 * v92) * v98) + v210) + v170;
            let v214 = (v39 - v212) / v92;
            let v215 = if v39 < v212 { 1.0 } else { 0.0 };
            let v304: f64;
            if v215 != 0.0 {
                let v220 = v212 + (v92 * ((v2 + (v214.exp())).ln()));
                v304 = v220;
            } else {
                let v226 = v39 + (v92 * ((v2 + ((-v214).exp())).ln()));
                v304 = v226;
            }
            let v231 = (((v227 * v92) * v98) + v210) + v170;
            let v233 = (v39 - v231) / v92;
            let v234 = if v39 < v231 { 1.0 } else { 0.0 };
            let v294: f64;
            if v234 != 0.0 {
                let v239 = v231 + (v92 * ((v2 + (v233.exp())).ln()));
                v294 = v239;
            } else {
                let v245 = v39 + (v92 * ((v2 + ((-v233).exp())).ln()));
                v294 = v245;
            }
            let v254 = (((v246 * v92) * v98) + (v249 * v90)) + (v145 * v252);
            let v256 = (v39 - v254) / v92;
            let v257 = if v39 < v254 { 1.0 } else { 0.0 };
            let v1168: f64;
            if v257 != 0.0 {
                let v262 = v254 + (v92 * ((v2 + (v256.exp())).ln()));
                v1168 = v262;
            } else {
                let v268 = v39 + (v92 * ((v2 + ((-v256).exp())).ln()));
                v1168 = v268;
            }
            let v277 = (((v269 * v92) * v98) + (v272 * v90)) + (v145 * v275);
            let v279 = (v39 - v277) / v92;
            let v280 = if v39 < v277 { 1.0 } else { 0.0 };
            let v300: f64;
            if v280 != 0.0 {
                let v285 = v277 + (v92 * ((v2 + (v279.exp())).ln()));
                v300 = v285;
            } else {
                let v291 = v39 + (v92 * ((v2 + ((-v279).exp())).ln()));
                v300 = v291;
            }
            let v293 = v2 / v292;
            let v295 = v2 / v294;
            let v297 = (v56 * v293).powf(v27);
            let v299 = (v58 * v295).powf(v59);
            let v308 = ((v2 - v302) * ((v58 / v304).powf(v59))) + v302;
            let v309 = v2 / v308;
            let v311 = v310 * v308;
            let v312 = v302 * v309;
            let v317 = v313 * ((v98 * v314).exp());
            let v318 = if v317 < v21 { 1.0 } else { 0.0 };
            let v1840: f64;
            if v318 != 0.0 {
                v1840 = v21;
            } else {
                v1840 = v317;
            }
            let v325 = v319 * ((v98 * (v320 - v321)).exp());
            let v330 = v326 * ((v98 * v327).exp());
            let v331 = if v330 < v21 { 1.0 } else { 0.0 };
            let v1833: f64;
            if v331 != 0.0 {
                v1833 = v21;
            } else {
                v1833 = v330;
            }
            let v336 = v332 * ((v98 * v333).exp());
            let v340 = (v98 * v338).exp();
            let v341 = v337 * v340;
            let v343 = v342 * v340;
            let v348 = v344 * ((v98 * v345).exp());
            let v350 = if v349 != v0 { 1.0 } else { 0.0 };
            let v420: f64;
            if v350 != 0.0 {
                let v354 = v351 * (v2 + (v97 * v349));
                let v356 = (v354 - v2) / v25;
                let v357 = if v354 < v2 { 1.0 } else { 0.0 };
                let v369: f64;
                if v357 != 0.0 {
                    let v362 = v2 + (v25 * ((v2 + (v356.exp())).ln()));
                    v369 = v362;
                } else {
                    let v368 = v354 + (v25 * ((v2 + ((-v356).exp())).ln()));
                    v369 = v368;
                }
                let v371 = v369 - v370;
                v420 = v371;
            } else {
                v420 = v351;
            }
            let v373 = if v372 != v0 { 1.0 } else { 0.0 };
            let v1063: f64;
            if v373 != 0.0 {
                let v377 = v374 * (v2 + (v97 * v372));
                let v379 = (v377 - v2) / v25;
                let v380 = if v377 < v2 { 1.0 } else { 0.0 };
                let v392: f64;
                if v380 != 0.0 {
                    let v385 = v2 + (v25 * ((v2 + (v379.exp())).ln()));
                    v392 = v385;
                } else {
                    let v391 = v377 + (v25 * ((v2 + ((-v379).exp())).ln()));
                    v392 = v391;
                }
                let v394 = v392 - v393;
                v1063 = v394;
            } else {
                v1063 = v374;
            }
            let v399 = v395 * (v2 + (v396 * v97));
            let v401 = v399 * v399;
            let v402 = if v399 < v0 { 1.0 } else { 0.0 };
            let v1676: f64;
            if v402 != 0.0 {
                let v408 = v404 / (((v401 + v400).sqrt()) - v399);
                v1676 = v408;
            } else {
                let v412 = v403 * (((v401 + v400).sqrt()) + v399);
                v1676 = v412;
            }
            let v428 = (v413 * (((v98 * (((v414 - v320) - v321) + v417)) / v420).exp())) * ((((-v146) * v96) / v420).exp());
            let v433 = v429 * ((v98 * (v2 - v320)).exp());
            let v439 = v434 * ((v98 * (v2 - v435)).exp());
            let v450 = (-v448) * v96;
            let v453 = (v440 * ((v98 * (v441 - (v26 * v442))).exp())) * ((v450 / v442).exp());
            let v465 = (v454 * ((v98 * (v441 - (v26 * v455))).exp())) * ((((-v169) * v96) / v455).exp());
            let v469 = v98 * ((v414 - v314) + v417);
            let v476 = (-v474) * v96;
            let v479 = (v466 * ((v469 / v470).exp())) * ((v476 / v470).exp());
            let v487 = (v480 * ((v469 / v481).exp())) * ((v476 / v481).exp());
            let v489 = if v488 == v2 { 1.0 } else { 0.0 };
            let v1190: f64;
            let v1203: f64;
            let v1245: f64;
            if v489 != 0.0 {
                let v496 = v490 * ((((-v491) * v96) / v470).exp());
                let v502 = v497 * (((-v498) * v96).exp());
                let v509 = v503 * ((((-v504) * v96) / v481).exp());
                v1190 = v496;
                v1203 = v502;
                v1245 = v509;
            } else {
                v1190 = v0;
                v1203 = v0;
                v1245 = v0;
            }
            let v520 = (v510 * ((v98 * ((v414 - v435) + v417)).exp())) * (((-v516) * v96).exp());
            let v530 = (v521 * ((v98 * (v441 - (v26 * v522))).exp())) * ((v450 / v522).exp());
            let v539 = (v531 * ((v98 * (v414 / v532)).exp())) * ((v450 / v532).exp());
            let v546 = (v540 * (v90.sqrt())) * ((v543 * v97).exp());
            let v550 = (v547 * v55).powf(v549);
            let v551 = v2 / v297;
            let v560 = (((((((v552 * v547) * v547) * v550) * v551) * v56) * v293) * v55) * v55;
            let v570 = ((((((v561 * v550) * v292) * v292) * v57) * v57) * v297) * ((v552 - v560).exp());
            let v574 = (v571 * v85).powf(v573);
            let v584 = (((((((v576 * v571) * v571) * v574) * (v2 / v299)) * v58) * v295) * v85) * v85;
            let v594 = ((((((v585 * v574) * v294) * v294) * v86) * v86) * v299) * ((v576 - v584).exp());
            let v596 = (v98 * v321).exp();
            let v599 = (v597 * v596) * v309;
            let v602 = (v600 * v596) * v551;
            let v612 = (v603 * ((v98 * (v414 - v604)).exp())) * (((-v275) * v96).exp());
            let v617 = v613 * ((v98 * (v2 - v604)).exp());
            let v623 = v618 * ((v98 * ((v321 + v320) - v2)).exp());
            let v628 = v624 * ((v98 * (v345 - v2)).exp());
            let v630 = v623 + v628;
            let v633 = (v629 * v630) / (v618 + v624);
            let v639 = v634 * ((v98 * (v635 - v2)).exp());
            let v641 = v15 - v640;
            let v643 = if v15 < v642 { 1.0 } else { 0.0 };
            let v1758: f64;
            if v643 != 0.0 {
                let v652 = v644 * ((v2 + (v645 * v641)) - ((v648 * v641) * v641));
                v1758 = v652;
            } else {
                let v654 = v644 * v653;
                v1758 = v654;
            }
            let v656 = v655 * v596;
            let v657 = if v332 > v0 { 1.0 } else { 0.0 };
            let v2000: f64;
            if v657 != 0.0 {
                let v658 = v2 / v336;
                let v659 = if v658 > v22 { 1.0 } else { 0.0 };
                let v2001: f64;
                if v659 != 0.0 {
                    v2001 = v22;
                } else {
                    v2001 = v658;
                }
                v2000 = v2001;
            } else {
                v2000 = v0;
            }
            let v660 = if v337 > v0 { 1.0 } else { 0.0 };
            let v2007: f64;
            if v660 != 0.0 {
                let v661 = v2 / v341;
                let v662 = if v661 > v22 { 1.0 } else { 0.0 };
                let v2008: f64;
                if v662 != 0.0 {
                    v2008 = v22;
                } else {
                    v2008 = v661;
                }
                v2007 = v2008;
            } else {
                v2007 = v0;
            }
            let v663 = if v342 > v0 { 1.0 } else { 0.0 };
            let v2009: f64;
            if v663 != 0.0 {
                let v664 = v2 / v343;
                let v665 = if v664 > v22 { 1.0 } else { 0.0 };
                let v2010: f64;
                if v665 != 0.0 {
                    v2010 = v22;
                } else {
                    v2010 = v664;
                }
                v2009 = v2010;
            } else {
                v2009 = v0;
            }
            let v669 = v1 * (v666 - v667);
            let v672 = v1 * (v666 - v670);
            let v675 = v1 * (v666 - v673);
            let v678 = v1 * (v676 - v673);
            let v680 = v1 * (v676 - v666);
            let v683 = v1 * (v681 - v667);
            let v685 = v1 * (v667 - v670);
            let v689 = v1 * (v687 - v676);
            let v691 = v1 * (v687 - v686);
            let v694 = v1 * (v687 - v692);
            let v697 = v1 * (v695 - v667);
            let v700 = v1 * (v698 - v695);
            let v703 = ((v680 + v672) - v685) - v697;
            let v708 = v694 + ((((-v694) + v689) + v703) - v700);
            let v709 = v683 - v697;
            let v710 = v709 - v700;
            let v711 = v672 * v94;
            let v713 = if v711 < v712 { 1.0 } else { 0.0 };
            let v951: f64;
            if v713 != 0.0 {
                let v714 = v711.exp();
                v951 = v714;
            } else {
                let v718 = (v712.exp()) * (v2 + (v711 - v712));
                v951 = v718;
            }
            let v719 = v675 * v94;
            let v720 = v719 / v420;
            let v721 = if v720 < v712 { 1.0 } else { 0.0 };
            let v1056: f64;
            if v721 != 0.0 {
                let v722 = v720.exp();
                v1056 = v722;
            } else {
                let v726 = (v712.exp()) * (v2 + (v720 - v712));
                v1056 = v726;
            }
            let v727 = v703 * v94;
            let v728 = if v727 < v712 { 1.0 } else { 0.0 };
            let v1442: f64;
            if v728 != 0.0 {
                let v729 = v727.exp();
                v1442 = v729;
            } else {
                let v733 = (v712.exp()) * (v2 + (v727 - v712));
                v1442 = v733;
            }
            let v734 = v680 * v94;
            let v735 = if v734 < v712 { 1.0 } else { 0.0 };
            let v1658: f64;
            if v735 != 0.0 {
                let v736 = v734.exp();
                v1658 = v736;
            } else {
                let v740 = (v712.exp()) * (v2 + (v734 - v712));
                v1658 = v740;
            }
            let v741 = v708 * v94;
            let v742 = if v741 < v712 { 1.0 } else { 0.0 };
            let v1513: f64;
            if v742 != 0.0 {
                let v743 = v741.exp();
                v1513 = v743;
            } else {
                let v747 = (v712.exp()) * (v2 + (v741 - v712));
                v1513 = v747;
            }
            let v748 = v683 * v94;
            let v749 = if v748 < v712 { 1.0 } else { 0.0 };
            let v1458: f64;
            if v749 != 0.0 {
                let v750 = v748.exp();
                v1458 = v750;
            } else {
                let v754 = (v712.exp()) * (v2 + (v748 - v712));
                v1458 = v754;
            }
            let v755 = v710 * v94;
            let v756 = if v755 < v712 { 1.0 } else { 0.0 };
            let v1525: f64;
            if v756 != 0.0 {
                let v757 = v755.exp();
                v1525 = v757;
            } else {
                let v761 = (v712.exp()) * (v2 + (v755 - v712));
                v1525 = v761;
            }
            let v762 = v709 * v94;
            let v763 = if v762 < v712 { 1.0 } else { 0.0 };
            let v1474: f64;
            if v763 != 0.0 {
                let v764 = v762.exp();
                v1474 = v764;
            } else {
                let v768 = (v712.exp()) * (v2 + (v762 - v712));
                v1474 = v768;
            }
            let v771 = (v708 - v769) * v94;
            let v772 = if v771 < v712 { 1.0 } else { 0.0 };
            let v1932: f64;
            if v772 != 0.0 {
                let v773 = v771.exp();
                v1932 = v773;
            } else {
                let v777 = (v712.exp()) * (v2 + (v771 - v712));
                v1932 = v777;
            }
            let v780 = if ((v703 - v769) * v94) < v712 { 1.0 } else { 0.0 };
            if v780 != 0.0 {
            } else {
            }
            let v782 = (v672 - v769) * v94;
            let v783 = if v782 < v712 { 1.0 } else { 0.0 };
            let v797: f64;
            if v783 != 0.0 {
                let v784 = v782.exp();
                v797 = v784;
            } else {
                let v788 = (v712.exp()) * (v2 + (v782 - v712));
                v797 = v788;
            }
            let v790 = (v669 - v769) * v94;
            let v791 = if v790 < v712 { 1.0 } else { 0.0 };
            let v801: f64;
            if v791 != 0.0 {
                let v792 = v790.exp();
                v801 = v792;
            } else {
                let v796 = (v712.exp()) * (v2 + (v790 - v712));
                v801 = v796;
            }
            let v800 = (v2 + (v414 * v797)).sqrt();
            let v804 = (v2 + (v414 * v801)).sqrt();
            let v806 = v2 + v804;
            let v807 = (v26 * v801) / v806;
            let v809 = if v807 < v808 { 1.0 } else { 0.0 };
            let v896: f64;
            if v809 != 0.0 {
                v896 = v808;
            } else {
                v896 = v807;
            }
            let v811 = v800 + v2;
            let v815 = v92 * ((v800 - v804) - ((v811 / v806).ln()));
            let v817 = (v815 + v685) / v348;
            let v818 = if v817 > v0 { 1.0 } else { 0.0 };
            let v1007: f64;
            let v1020: f64;
            let v1035: f64;
            let v1062: f64;
            let v1706: f64;
            let v1742: f64;
            if v818 != 0.0 {
                let v820 = if v669 < v819 { 1.0 } else { 0.0 };
                let v833: f64;
                if v820 != 0.0 {
                    v833 = v669;
                } else {
                    let v824 = v819 + ((v2 + (v669 - v819)).ln());
                    v833 = v824;
                }
                let v827 = (v403 * v817) * v348;
                let v834 = (v769 + ((v26 * v92) * (((v827 * v94) + v2).ln()))) - v833;
                let v836 = v835 * v769;
                let v837 = v836 * v836;
                let v838 = v834 * v834;
                let v839 = if v834 < v0 { 1.0 } else { 0.0 };
                let v849: f64;
                if v839 != 0.0 {
                    let v844 = (v403 * v837) / (((v838 + v837).sqrt()) - v834);
                    v849 = v844;
                } else {
                    let v848 = v403 * (((v838 + v837).sqrt()) + v834);
                    v849 = v848;
                }
                let v852 = v850 * v851;
                let v858 = (v849 * (v849 + v852)) / (v851 * (v849 + (v850 * v348)));
                let v859 = v817 / v858;
                let v862 = (v859 - v2) / v861;
                let v863 = if v859 < v2 { 1.0 } else { 0.0 };
                let v875: f64;
                if v863 != 0.0 {
                    let v868 = v2 + (v861 * ((v2 + (v862.exp())).ln()));
                    v875 = v868;
                } else {
                    let v874 = v859 + (v861 * ((v2 + ((-v862).exp())).ln()));
                    v875 = v874;
                }
                let v883 = v875 / (v2 + (v861 * ((v2 + ((v876 / v861).exp())).ln())));
                let v884 = v849 / v852;
                let v887 = v2 + v884;
                let v894 = (v2 + ((v2 + (((v414 * v883) * v884) * v887)).sqrt())) / ((v26 * v883) * v887);
                let v897 = v896 * v894;
                let v900 = ((v2 - v894) + v897) / (v2 + v897);
                let v902 = (v827 * v900) * v94;
                let v907 = (v26 * v902) + (v896 * ((v896 + v902) + v2));
                let v909 = v403 * (v902 - v2);
                let v911 = (v909 * v909) + v907;
                let v912 = if v902 >= v2 { 1.0 } else { 0.0 };
                let v918: f64;
                if v912 != 0.0 {
                    let v914 = v909 + (v911.sqrt());
                    v918 = v914;
                } else {
                    let v917 = v907 / ((v911.sqrt()) - v909);
                    v918 = v917;
                }
                let v920 = if v918 < v919 { 1.0 } else { 0.0 };
                let v921: f64;
                if v920 != 0.0 {
                    v921 = v919;
                } else {
                    v921 = v918;
                }
                let v926 = (v921 * (v921 + v2)) * ((v769 * v94).exp());
                let v929 = (v403 * v851) * (v817 - v850);
                let v936 = v929 + (((v929 * v929) + (((v851 * v348) * v850) * v817)).sqrt());
                let v938 = if v937 == v0 { 1.0 } else { 0.0 };
                let v1021: f64;
                if v938 != 0.0 {
                    let v939 = v304 * v41;
                    v1021 = v939;
                } else {
                    let v944 = v304 * (v41 + ((v26 * v817) / (v817 + v858)));
                    v1021 = v944;
                }
                let v946 = v850 + v817;
                let v947 = (v850 * v817) / v946;
                let v948 = v850 / v946;
                v1007 = v936;
                v1020 = v1021;
                v1035 = v948;
                v1062 = v926;
                v1706 = v900;
                v1742 = v947;
            } else {
                let v950 = (v26 * v797) / v811;
                let v962 = if (if (v685.abs()) < (v953 * v92) { 1.0 } else { 0.0 }) != 0.0 || (if (v815.abs()) < ((v957 * v92) * (v800 + v804)) { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v1707: f64;
                if v962 != 0.0 {
                    let v964 = v403 * (v950 + v896);
                    let v966 = v964 / (v964 + v2);
                    v1707 = v966;
                } else {
                    let v969 = v815 / ((v815 + v672) - v669);
                    v1707 = v969;
                }
                let v970 = v41 * v304;
                let v972 = v2 - (v817 / v850);
                v1007 = v685;
                v1020 = v970;
                v1035 = v972;
                v1062 = v951;
                v1706 = v1707;
                v1742 = v817;
            }
            let v977 = v292 * (v2 - (v139.powf((v973 / v27))));
            let v978 = v41 * v292;
            let v980 = (v675 - v977) / v978;
            let v981 = if v675 < v977 { 1.0 } else { 0.0 };
            let v993: f64;
            if v981 != 0.0 {
                let v986 = v675 - (v978 * ((v2 + (v980.exp())).ln()));
                v993 = v986;
            } else {
                let v992 = v977 - (v978 * ((v2 + ((-v980).exp())).ln()));
                v993 = v992;
            }
            let v996 = v2 - v27;
            let v997 = (v2 - (v993 * v293)).powf(v996);
            let v1003 = ((v292 / v996) * (v2 - v997)) + (v139 * (v675 - v993));
            let v1005 = if v1004 == v2 { 1.0 } else { 0.0 };
            let v1017: f64;
            if v1005 != 0.0 {
                v1017 = v669;
            } else {
                let v1006 = if v1004 == v26 { 1.0 } else { 0.0 };
                let v1018: f64;
                if v1006 != 0.0 {
                    let v1008 = v669 + v1007;
                    v1018 = v1008;
                } else {
                    v1018 = v672;
                }
                v1017 = v1018;
            }
            let v1010 = v2 - v312;
            let v1011 = (v26 - v312) / v1010;
            let v1016 = v304 * (v2 - (v1011.powf((v1012 / v59))));
            let v1022 = (v1017 - v1016) / v1020;
            let v1023 = if v1017 < v1016 { 1.0 } else { 0.0 };
            let v1040: f64;
            if v1023 != 0.0 {
                let v1028 = v1017 - (v1020 * ((v2 + (v1022.exp())).ln()));
                v1040 = v1028;
            } else {
                let v1034 = v1016 - (v1020 * ((v2 + ((-v1022).exp())).ln()));
                v1040 = v1034;
            }
            let v1037 = v1035.powf(v1036);
            let v1038 = v2 - v59;
            let v1039 = v304 / v1038;
            let v1053 = (v1010 * ((v1039 * (v2 - (v1037 * ((v2 - (v1040 / v304)).powf(v1038))))) + ((v1037 * v1011) * (v1017 - v1040)))) + (v312 * v669);
            let v1055 = (v414 * v428) / v433;
            let v1057 = v1055 * v1056;
            let v1061 = v1057 / (v2 + ((v2 + v1057).sqrt()));
            let v1065 = v1062.powf((v2 / v1063));
            let v1066 = v1055 * v1065;
            let v1070 = v1066 / (v2 + ((v2 + v1066).sqrt()));
            let v1071 = if v655 == v0 { 1.0 } else { 0.0 };
            let v1092: f64;
            if v1071 != 0.0 {
                let v1075 = (v2 + (v1003 / v602)) + (v1053 / v599);
                v1092 = v1075;
            } else {
                let v1090 = ((((((v1003 / v602) + v2) * v656) * v94).exp()) - (((((-v1053) / v599) * v656) * v94).exp())) / (((v656 * v94).exp()) - v2);
                v1092 = v1090;
            }
            let v1093 = v1092 * v1092;
            let v1094 = if v1092 < v0 { 1.0 } else { 0.0 };
            let v1104: f64;
            if v1094 != 0.0 {
                let v1099 = v1095 / (((v1093 + v1091).sqrt()) - v1092);
                v1104 = v1099;
            } else {
                let v1103 = v403 * (((v1093 + v1091).sqrt()) + v1092);
                v1104 = v1103;
            }
            let v1107 = v2 + (v403 * (v1061 + v1070));
            let v1108 = v1104 * v1107;
            let v1111 = (v1109 * v428) * v1065;
            let v1112 = v428 * v1056;
            let v1114 = (v1112 - v1111) / v1108;
            let v1116 = v675 / v1115;
            let v1117 = if v675 < v0 { 1.0 } else { 0.0 };
            let v1128: f64;
            if v1117 != 0.0 {
                let v1121 = v1115 * ((v2 + (v1116.exp())).ln());
                v1128 = v1121;
            } else {
                let v1127 = v675 + (v1115 * ((v2 + ((-v1116).exp())).ln()));
                v1128 = v1127;
            }
            let v1130 = v1128 / v1129;
            let v1131 = if v1130 < v712 { 1.0 } else { 0.0 };
            let v1137: f64;
            if v1131 != 0.0 {
                let v1132 = v1130.exp();
                v1137 = v1132;
            } else {
                let v1136 = (v712.exp()) * (v2 + (v1130 - v712));
                v1137 = v1136;
            }
            let v1139 = v546 * (v1137 - v2);
            let v1142 = (v675 - v1140) / v25;
            let v1143 = if v675 < v1140 { 1.0 } else { 0.0 };
            let v1156: f64;
            if v1143 != 0.0 {
                let v1148 = v675 - (v25 * ((v2 + (v1142.exp())).ln()));
                v1156 = v1148;
            } else {
                let v1154 = v1140 - (v25 * ((v2 + ((-v1142).exp())).ln()));
                v1156 = v1154;
            }
            let v1158 = v1140 - v1156;
            let v1160 = (v1155 * v1156) * (v1158 * v1158);
            let v1161 = v719 / v470;
            let v1162 = if v1161 < v712 { 1.0 } else { 0.0 };
            let v1187: f64;
            if v1162 != 0.0 {
                let v1163 = v1161.exp();
                v1187 = v1163;
            } else {
                let v1167 = (v712.exp()) * (v2 + (v1161 - v712));
                v1187 = v1167;
            }
            let v1972: f64;
            if v489 != 0.0 {
                let v1170 = (v675 - v1168) * v94;
                let v1171 = if v1170 < v712 { 1.0 } else { 0.0 };
                let v1193: f64;
                if v1171 != 0.0 {
                    let v1172 = v1170.exp();
                    v1193 = v1172;
                } else {
                    let v1176 = (v712.exp()) * (v2 + (v1170 - v712));
                    v1193 = v1176;
                }
                let v1179 = (v1114 / v428) - v1178;
                let v1181 = if v1179 < v1180 { 1.0 } else { 0.0 };
                let v1206: f64;
                if v1181 != 0.0 {
                    let v1182 = v1179.exp();
                    v1206 = v1182;
                } else {
                    let v1186 = v1183 * (v2 + (v1179 - v1180));
                    v1206 = v1186;
                }
                let v1188 = v1187 - v2;
                let v1210 = ((v479 * v1188) + ((((v1190 * v26) * v1188) / (v2 + ((v2 + (v414 * v1193)).sqrt()))) * (v2 + (v1053 / v599)))) + (((v1203 * (v1062 - v2)) * v1206) / (v2 + v1206));
                v1972 = v1210;
            } else {
                let v1212 = if v1211 == v0 { 1.0 } else { 0.0 };
                let v1973: f64;
                if v1212 != 0.0 {
                    let v1214 = v479 * (v1187 - v2);
                    v1973 = v1214;
                } else {
                    let v1225 = v479 * (((v2 - v1211) * (v1187 - v2)) + ((v1211 * ((v1187 + v1062) - v26)) * (v2 + (v1053 / v599))));
                    v1973 = v1225;
                }
                v1972 = v1973;
            }
            let v1226 = v678 * v94;
            let v1227 = v1226 / v481;
            let v1228 = if v1227 < v712 { 1.0 } else { 0.0 };
            let v1242: f64;
            if v1228 != 0.0 {
                let v1229 = v1227.exp();
                v1242 = v1229;
            } else {
                let v1233 = (v712.exp()) * (v2 + (v1227 - v712));
                v1242 = v1233;
            }
            let v1969: f64;
            if v489 != 0.0 {
                let v1235 = (v678 - v1168) * v94;
                let v1236 = if v1235 < v712 { 1.0 } else { 0.0 };
                let v1248: f64;
                if v1236 != 0.0 {
                    let v1237 = v1235.exp();
                    v1248 = v1237;
                } else {
                    let v1241 = (v712.exp()) * (v2 + (v1235 - v712));
                    v1248 = v1241;
                }
                let v1243 = v1242 - v2;
                let v1254 = (v487 * v1243) + (((v1245 * v26) * v1243) / (v2 + ((v2 + (v414 * v1248)).sqrt())));
                v1969 = v1254;
            } else {
                let v1256 = v487 * (v1242 - v2);
                v1969 = v1256;
            }
            let v1257 = v719 / v442;
            let v1258 = if v1257 < v712 { 1.0 } else { 0.0 };
            let v1264: f64;
            if v1258 != 0.0 {
                let v1259 = v1257.exp();
                v1264 = v1259;
            } else {
                let v1263 = (v712.exp()) * (v2 + (v1257 - v712));
                v1264 = v1263;
            }
            let v1266 = v453 * (v1264 - v2);
            let v1267 = v1226 / v522;
            let v1268 = if v1267 < v712 { 1.0 } else { 0.0 };
            let v1274: f64;
            if v1268 != 0.0 {
                let v1269 = v1267.exp();
                v1274 = v1269;
            } else {
                let v1273 = (v712.exp()) * (v2 + (v1267 - v712));
                v1274 = v1273;
            }
            let v1276 = v530 * (v1274 - v2);
            let v1277 = v727 / v455;
            let v1278 = if v1277 < v712 { 1.0 } else { 0.0 };
            let v1284: f64;
            if v1278 != 0.0 {
                let v1279 = v1277.exp();
                v1284 = v1279;
            } else {
                let v1283 = (v712.exp()) * (v2 + (v1277 - v712));
                v1284 = v1283;
            }
            let v1286 = v465 * (v1284 - v2);
            let v1287 = v1226 / v532;
            let v1288 = if v1287 < v712 { 1.0 } else { 0.0 };
            let v1294: f64;
            if v1288 != 0.0 {
                let v1289 = v1287.exp();
                v1294 = v1289;
            } else {
                let v1293 = (v712.exp()) * (v2 + (v1287 - v712));
                v1294 = v1293;
            }
            let v1296 = v539 * (v1294 - v2);
            let v1300 = if (if (if v561 > v0 { 1.0 } else { 0.0 }) != 0.0 && (if v552 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && v1117 != 0.0 { 1.0 } else { 0.0 };
            let v1975: f64;
            if v1300 != 0.0 {
                let v1304 = v560 * (v2 - (v29 / (v26 * v997)));
                let v1305 = if v1304 < v712 { 1.0 } else { 0.0 };
                let v1366: f64;
                if v1305 != 0.0 {
                    let v1306 = v1304.exp();
                    v1366 = v1306;
                } else {
                    let v1310 = (v712.exp()) * (v2 + (v1304 - v712));
                    v1366 = v1310;
                }
                let v1311 = v675 * v293;
                let v1322 = v27 - v2;
                let v1337 = ((v675 * v29) * v560) / (v547 * ((((((v1311 * v1311) + v1313).sqrt()).powf((v1316 - v27))) * ((v27 * ((v2 - (v27 * v27)) - ((v139 * v1311) * v1322))) - (((v441 * v1311) * v1311) * (v1322 + v1311)))) * v1332));
                let v1339 = if v1337 < v1338 { 1.0 } else { 0.0 };
                let v1363: f64;
                if v1339 != 0.0 {
                    let v1340 = if v1337 < v712 { 1.0 } else { 0.0 };
                    let v1347: f64;
                    if v1340 != 0.0 {
                        let v1341 = v1337.exp();
                        v1347 = v1341;
                    } else {
                        let v1345 = (v712.exp()) * (v2 + (v1337 - v712));
                        v1347 = v1345;
                    }
                    let v1351 = (-v675) * (v2 + ((v2 - v1347) / v1337));
                    v1363 = v1351;
                } else {
                    let v1361 = ((v675 * v403) * v1337) * (v2 + ((v1337 * v1354) * (v2 + (v1356 * v1337))));
                    v1363 = v1361;
                }
                let v1369 = (((((v26 * v570) * v1363) * v997) * v1366) * v293) * v30;
                v1975 = v1369;
            } else {
                v1975 = v0;
            }
            let v1374 = if (if (if v585 > v0 { 1.0 } else { 0.0 }) != 0.0 && (if v576 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if v669 < v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let v1627: f64;
            if v1374 != 0.0 {
                let v1375 = v669 * v295;
                let v1377 = (v2 - v1375).powf(v1038);
                let v1381 = v584 * (v2 - (v61 / (v26 * v1377)));
                let v1382 = if v1381 < v712 { 1.0 } else { 0.0 };
                let v1438: f64;
                if v1382 != 0.0 {
                    let v1383 = v1381.exp();
                    v1438 = v1383;
                } else {
                    let v1387 = (v712.exp()) * (v2 + (v1381 - v712));
                    v1438 = v1387;
                }
                let v1397 = v59 - v2;
                let v1411 = ((v669 * v61) * v584) / (v571 * ((((((v1375 * v1375) + v1313).sqrt()).powf((v1391 - v59))) * ((v59 * ((v2 - (v59 * v59)) - ((v139 * v1375) * v1397))) - (((v441 * v1375) * v1375) * (v1397 + v1375)))) * v1332));
                let v1413 = if v1411 < v1412 { 1.0 } else { 0.0 };
                let v1435: f64;
                if v1413 != 0.0 {
                    let v1414 = if v1411 < v712 { 1.0 } else { 0.0 };
                    let v1421: f64;
                    if v1414 != 0.0 {
                        let v1415 = v1411.exp();
                        v1421 = v1415;
                    } else {
                        let v1419 = (v712.exp()) * (v2 + (v1411 - v712));
                        v1421 = v1419;
                    }
                    let v1425 = (-v669) * (v2 + ((v2 - v1421) / v1411));
                    v1435 = v1425;
                } else {
                    let v1433 = ((v669 * v403) * v1411) * (v2 + ((v1411 * v1354) * (v2 + (v1356 * v1411))));
                    v1435 = v1433;
                }
                let v1441 = (((((v26 * v594) * v1435) * v1377) * v1438) * v295) * v62;
                v1627 = v1441;
            } else {
                v1627 = v0;
            }
            let v1444 = v1442 - v2;
            let v1447 = (v414 * v520) / v439;
            let v1452 = ((v26 * v520) * v1444) / (v2 + ((v2 + (v1447 * v1442)).sqrt()));
            let v1454 = if v1453 == v2 { 1.0 } else { 0.0 };
            let v1509: f64;
            let v1977: f64;
            if v1454 != 0.0 {
                let v1462 = v414 * (v612 / v617);
                let v1470 = (((v1455 * v26) * v612) * (v951 - v1458)) / (v2 + ((v2 + (v1462 * (v951 + (v1463 * v1458)))).sqrt()));
                let v1483 = ((((v2 - v1455) * v26) * v612) * (v1442 - v1474)) / (v2 + ((v2 + (v1462 * (v1442 + (v1463 * v1474)))).sqrt()));
                v1509 = v1483;
                v1977 = v1470;
            } else {
                let v1489 = v414 * (v612 / v617);
                let v1494 = (((v1455 * v26) * v612) * (v951 - v2)) / (v2 + ((v2 + (v1489 * v951)).sqrt()));
                let v1503 = ((((v2 - v1455) * v26) * v612) * v1444) / (v2 + ((v2 + (v1489 * v1442)).sqrt()));
                v1509 = v1503;
                v1977 = v1494;
            }
            let v1506 = if v8 > v0 { 1.0 } else { 0.0 };
            let v1507 = if (if v1504 > v0 { 1.0 } else { 0.0 }) != 0.0 && v1506 != 0.0 { 1.0 } else { 0.0 };
            let v1631: f64;
            let v1634: f64;
            let v1963: f64;
            let v1976: f64;
            let v1978: f64;
            if v1507 != 0.0 {
                let v1508 = v1452 * v9;
                let v1510 = v1509 * v9;
                let v1514 = v1513 - v2;
                let v1520 = (((v8 * v26) * v520) * v1514) / (v2 + ((v2 + (v1447 * v1513)).sqrt()));
                let v1571: f64;
                if v1454 != 0.0 {
                    let v1536 = (((((v2 - v1455) * v8) * v26) * v612) * (v1513 - v1525)) / (v2 + ((v2 + (((v414 * v612) / v617) * (v1513 + (v1463 * v1525)))).sqrt()));
                    v1571 = v1536;
                } else {
                    let v1548 = (((((v2 - v1455) * v8) * v26) * v612) * v1514) / (v2 + ((v2 + (((v414 * v612) / v617) * v1513)).sqrt()));
                    v1571 = v1548;
                }
                let v1549 = if v1504 == v2 { 1.0 } else { 0.0 };
                let v1577: f64;
                if v1549 != 0.0 {
                    let v1552 = (v8 * (v520 + v612)) * v336;
                    let v1557 = v708 - (v92 * (v26 - ((v1552 * v94).ln())));
                    let v1559 = v1557 * v1557;
                    let v1560 = if v1557 < v0 { 1.0 } else { 0.0 };
                    let v1570: f64;
                    if v1560 != 0.0 {
                        let v1565 = v1561 / (((v1559 + v1558).sqrt()) - v1557);
                        v1570 = v1565;
                    } else {
                        let v1569 = v403 * (((v1559 + v1558).sqrt()) + v1557);
                        v1570 = v1569;
                    }
                    let v1576 = v1570 / ((v1552 + ((v1520 + v1571) * v336)) + v1570);
                    v1577 = v1576;
                } else {
                    v1577 = v2;
                }
                let v1578 = v1577 * v1520;
                let v1579 = v1577 * v1571;
                v1631 = v1508;
                v1634 = v1578;
                v1963 = v1577;
                v1976 = v1510;
                v1978 = v1579;
            } else {
                v1631 = v1452;
                v1634 = v0;
                v1963 = v2;
                v1976 = v1509;
                v1978 = v0;
            }
            let v1581 = if v1580 == v2 { 1.0 } else { 0.0 };
            let v1628: f64;
            if v1581 != 0.0 {
                let v1582 = v680 + v669;
                let v1589 = ((v1585 * v1582) * v1587) * v1582;
                let v1592 = if (v1590 * v1582) < v0 { 1.0 } else { 0.0 };
                let v1618: f64;
                if v1592 != 0.0 {
                    let v1599 = v1593 / (((v1589 + v1584).sqrt()) - (v1596 * v1582));
                    v1618 = v1599;
                } else {
                    let v1605 = v403 * (((v1589 + v1584).sqrt()) + (v1602 * v1582));
                    v1618 = v1605;
                }
                let v1609 = v2 / (v2 - (v89.powf(v1606)));
                let v1611 = v89 * v1610;
                let v1617 = (((v1609 * v1609) * (v89.powf((v1606 - v2)))) * v1606) / v1610;
                let v1619 = if v1618 < v1611 { 1.0 } else { 0.0 };
                let v1629: f64;
                if v1619 != 0.0 {
                    let v1623 = v2 / (v2 - ((v1618 / v1610).powf(v1606)));
                    v1629 = v1623;
                } else {
                    let v1626 = v1609 + ((v1618 - v1611) * v1617);
                    v1629 = v1626;
                }
                v1628 = v1629;
            } else {
                v1628 = v2;
            }
            let v1630 = v1627 * v1628;
            let v1632 = v1631 * v1628;
            let v1633 = v1286 * v1628;
            let v1635 = v1634 * v1628;
            let v1639 = (v2 + (v1003 / v602)) + (v1053 / v599);
            let v1641 = v1639 * v1639;
            let v1642 = if v1639 < v0 { 1.0 } else { 0.0 };
            let v1652: f64;
            if v1642 != 0.0 {
                let v1647 = v1643 / (((v1641 + v1640).sqrt()) - v1639);
                v1652 = v1647;
            } else {
                let v1651 = v403 * (((v1641 + v1640).sqrt()) + v1639);
                v1652 = v1651;
            }
            let v1654 = v325 / (v1652 * v1107);
            let v1655 = if v1654 < v21 { 1.0 } else { 0.0 };
            let v1656: f64;
            if v1655 != 0.0 {
                v1656 = v21;
            } else {
                v1656 = v1654;
            }
            let v1657 = v139 * v1656;
            let v1659 = if v1114 > v0 { 1.0 } else { 0.0 };
            let v1981: f64;
            if v1659 != 0.0 {
                let v1661 = if v1660 == v2 { 1.0 } else { 0.0 };
                let v1823: f64;
                if v1661 != 0.0 {
                    let v1663 = if v669 < v1662 { 1.0 } else { 0.0 };
                    let v1824: f64;
                    if v1663 != 0.0 {
                        let v1666 = (-v1114) / v1665;
                        let v1667 = if v1666 < v712 { 1.0 } else { 0.0 };
                        let v1674: f64;
                        if v1667 != 0.0 {
                            let v1668 = v1666.exp();
                            v1674 = v1668;
                        } else {
                            let v1672 = (v712.exp()) * (v2 + (v1666 - v712));
                            v1674 = v1672;
                        }
                        let v1675 = (v1662 - v669) * v1674;
                        let v1680 = (-v1676) * (v1675.powf(v1678));
                        let v1681 = if v1680 < v712 { 1.0 } else { 0.0 };
                        let v1690: f64;
                        if v1681 != 0.0 {
                            let v1682 = v1680.exp();
                            v1690 = v1682;
                        } else {
                            let v1686 = (v712.exp()) * (v2 + (v1680 - v712));
                            v1690 = v1686;
                        }
                        let v1691 = ((v1687 / v1676) * v1675) * v1690;
                        v1824 = v1691;
                    } else {
                        v1824 = v0;
                    }
                    v1823 = v1824;
                } else {
                    let v1692 = if v1660 == v26 { 1.0 } else { 0.0 };
                    let v1825: f64;
                    if v1692 != 0.0 {
                        let v1693 = if v669 < v769 { 1.0 } else { 0.0 };
                        let v1826: f64;
                        if v1693 != 0.0 {
                            let v1698 = (v26 * v1694) / (v1696 * v1696);
                            let v1699 = v769 - v669;
                            let v1703 = ((v26 * (v1699 / v1035)) / v1698).sqrt();
                            let v1705 = if v1704 == v0 { 1.0 } else { 0.0 };
                            let v1712: f64;
                            if v1705 != 0.0 {
                                v1712 = v1696;
                            } else {
                                let v1709 = v2 - (v403 * v1706);
                                let v1711 = (v1696 * v1709) * v1709;
                                v1712 = v1711;
                            }
                            let v1718 = (v1703 * v1712) / (((v1703 * v1703) + (v1712 * v1712)).sqrt());
                            let v1719 = v1699 / v1718;
                            let v1720 = v403 * v1718;
                            let v1721 = v1720 * v1698;
                            let v1723 = v1719 + (v1721 * v1035);
                            let v1750: f64;
                            if v1705 != 0.0 {
                                v1750 = v1723;
                            } else {
                                let v1725 = v26 * v1724;
                                let v1737 = v1719 - (v1721 * (((v2 + v1724) / (v2 + v1725)) - (v1114 / (v850 * (v2 + (v1725 * (v2 + (v26 * v1706))))))));
                                let v1738 = v1737 - v1723;
                                let v1749 = v403 * ((v1737 + v1723) + (((v1738 * v1738) + ((((v41 * v1719) * v1719) * v1742) / v850)).sqrt()));
                                v1750 = v1749;
                            }
                            let v1752 = (v1750 - v1719) / v1750;
                            let v1755 = if (v1752.abs()) > v1754 { 1.0 } else { 0.0 };
                            let v1827: f64;
                            if v1755 != 0.0 {
                                let v1756 = v1720 / v1752;
                                let v1763 = (-v1758) / v1750;
                                let v1770 = (((v1757 / v1758) * v1750) * v1756) * ((v1763.exp()) - ((v1763 * (v2 + (v1712 / v1756))).exp()));
                                v1827 = v1770;
                            } else {
                                let v1775 = (v1757 * v1712) * (((-v1758) / v1750).exp());
                                v1827 = v1775;
                            }
                            v1826 = v1827;
                        } else {
                            v1826 = v0;
                        }
                        v1825 = v1826;
                    } else {
                        let v1776 = if v1660 == v139 { 1.0 } else { 0.0 };
                        let v1828: f64;
                        if v1776 != 0.0 {
                            let v1777 = if v669 < v1662 { 1.0 } else { 0.0 };
                            let v1829: f64;
                            if v1777 != 0.0 {
                                let v1778 = v1662 - v669;
                                let v1786 = (v1778.powf(v1678)) * ((v2 - (v1114 / (v1780 + v1114))).powf(v1784));
                                let v1787 = if v1704 == v0 { 1.0 } else { 0.0 };
                                let v1811: f64;
                                if v1787 != 0.0 {
                                    v1811 = v1786;
                                } else {
                                    let v1790 = (v1114 - v1788) / v1780;
                                    let v1793 = (v1790 - v2) / v1792;
                                    let v1794 = if v1790 < v2 { 1.0 } else { 0.0 };
                                    let v1806: f64;
                                    if v1794 != 0.0 {
                                        let v1799 = v2 + (v1792 * ((v2 + (v1793.exp())).ln()));
                                        v1806 = v1799;
                                    } else {
                                        let v1805 = v1790 + (v1792 * ((v2 + ((-v1793).exp())).ln()));
                                        v1806 = v1805;
                                    }
                                    let v1809 = v1786 * (v1806.powf(v1807));
                                    v1811 = v1809;
                                }
                                let v1812 = (-v1676) * v1811;
                                let v1813 = if v1812 < v712 { 1.0 } else { 0.0 };
                                let v1821: f64;
                                if v1813 != 0.0 {
                                    let v1814 = v1812.exp();
                                    v1821 = v1814;
                                } else {
                                    let v1818 = (v712.exp()) * (v2 + (v1812 - v712));
                                    v1821 = v1818;
                                }
                                let v1822 = ((v1687 / v1676) * v1778) * v1821;
                                v1829 = v1822;
                            } else {
                                v1829 = v0;
                            }
                            v1828 = v1829;
                        } else {
                            v1828 = v0;
                        }
                        v1825 = v1828;
                    }
                    v1823 = v1825;
                }
                let v1830 = if v1823 > v0 { 1.0 } else { 0.0 };
                let v1982: f64;
                if v1830 != 0.0 {
                    let v1832 = if v1831 == v2 { 1.0 } else { 0.0 };
                    let v1983: f64;
                    if v1832 != 0.0 {
                        let v1834 = v1833 + v1657;
                        let v1842 = ((v92 / (v1114 * v1834)) + ((v1108 / v428) * v479)) + (v1840 / v1834);
                        let v1843 = if v1660 == v139 { 1.0 } else { 0.0 };
                        let v1984: f64;
                        if v1843 != 0.0 {
                            let v1845 = (v1823 - v1842) / v1583;
                            let v1846 = if v1823 < v1842 { 1.0 } else { 0.0 };
                            let v1858: f64;
                            if v1846 != 0.0 {
                                let v1851 = v1823 - (v1583 * ((v2 + (v1845.exp())).ln()));
                                v1858 = v1851;
                            } else {
                                let v1857 = v1842 - (v1583 * ((v2 + ((-v1845).exp())).ln()));
                                v1858 = v1857;
                            }
                            let v1859 = v1114 * v1858;
                            v1984 = v1859;
                        } else {
                            let v1863 = ((v1114 * v1823) * v1842) / (v1823 + v1842);
                            v1984 = v1863;
                        }
                        v1983 = v1984;
                    } else {
                        let v1864 = v1114 * v1823;
                        v1983 = v1864;
                    }
                    v1982 = v1983;
                } else {
                    v1982 = v0;
                }
                v1981 = v1982;
            } else {
                v1981 = v0;
            }
            let v1865 = if v1062 > v0 { 1.0 } else { 0.0 };
            if v1865 != 0.0 {
            } else {
            }
            let v1866 = if v678 < v977 { 1.0 } else { 0.0 };
            if v1866 != 0.0 {
            } else {
            }
            let v1868 = v623 * v433;
            let v1869 = v41 * v304;
            let v1870 = if v703 < v1016 { 1.0 } else { 0.0 };
            if v1870 != 0.0 {
            } else {
            }
            let v1871 = v2 - v1867;
            let v1873 = (v708 - v1016) / v1869;
            let v1874 = if v708 < v1016 { 1.0 } else { 0.0 };
            let v1886: f64;
            if v1874 != 0.0 {
                let v1879 = v708 - (v1869 * ((v2 + (v1873.exp())).ln()));
                v1886 = v1879;
            } else {
                let v1885 = v1016 - (v1869 * ((v2 + ((-v1873).exp())).ln()));
                v1886 = v1885;
            }
            let v1900 = ((v311 * ((v1010 * ((v1039 * (v2 - ((v2 - (v1886 / v304)).powf(v1038)))) + (v1011 * (v708 - v1886)))) + (v312 * v708))) * v1871) * v8;
            let v1906 = if v683 < (v300 * (v2 - (v26.powf((v1901 / v301))))) { 1.0 } else { 0.0 };
            if v1906 != 0.0 {
            } else {
            }
            let v1910 = if (v675 / (v1907 * v92)) < v712 { 1.0 } else { 0.0 };
            if v1910 != 0.0 {
            } else {
            }
            let v1913 = ((v414 * v628) * v92) / v348;
            let v1915 = if v1914 == v0 { 1.0 } else { 0.0 };
            if v1915 != 0.0 {
            } else {
                let v1921 = if (((v703 - v1916) / v1918) * v94) < v712 { 1.0 } else { 0.0 };
                if v1921 != 0.0 {
                } else {
                }
            }
            let v1925 = if (if (if v1504 == v2 { 1.0 } else { 0.0 }) != 0.0 || (if v1504 == v139 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && v1506 != 0.0 { 1.0 } else { 0.0 };
            let v2002: f64;
            if v1925 != 0.0 {
                let v1964: f64;
                if v1915 != 0.0 {
                    let v1926 = v1055 * v1513;
                    let v1933 = v414 * v1932;
                    let v1944 = (((v403 * v8) * v633) * ((v1868 * ((v1926 - v1055) / (v2 + ((v2 + v1926).sqrt())))) + (v1913 * (v1933 / (v2 + ((v2 + v1933).sqrt())))))) / v630;
                    v1964 = v1944;
                } else {
                    let v1946 = (v708 - v1916) * v94;
                    let v1947 = if v1946 < v712 { 1.0 } else { 0.0 };
                    let v1957: f64;
                    if v1947 != 0.0 {
                        let v1948 = v1946.exp();
                        v1957 = v1948;
                    } else {
                        let v1952 = (v712.exp()) * (v2 + (v1946 - v712));
                        v1957 = v1952;
                    }
                    let v1962 = ((((v26 * v8) * v520) * v639) * v1513) / (v2 + ((v2 + (v414 * v1957)).sqrt()));
                    v1964 = v1962;
                }
                let v1965 = v1963 * v1964;
                v2002 = v1965;
            } else {
                v2002 = v0;
            }
            let v1967 = if v1966 == v2 { 1.0 } else { 0.0 };
            if v1967 != 0.0 {
                let v1968 = if v980 < v0 { 1.0 } else { 0.0 };
                if v1968 != 0.0 {
                } else {
                }
            } else {
            }
            let v1971 = (v1969 + v1276) + v1296;
            let v1974 = v1972 + v1266;
            if v489 != 0.0 {
            } else {
            }
            let v1980 = (v1 * v1978) * v20;
            let v1987 = ((v1 * v689) / v1833) * v20;
            let v1990 = (v1 * v1988) * v691;
            let v1992 = (0e0f64) * v20;
            let v1995 = (v1 * v1993) * v694;
            let v1997 = (0e0f64) * v20;
            let v1999 = (v1 * v1635) * v20;
            let v2004 = v1 * (v1900 + v2002);
            let v2006 = (0e0f64) * v20;
            if v660 != 0.0 {
            } else {
            }
            if v663 != 0.0 {
            } else {
            }
            let v2012 = v2011 * v15;
            let v2013 = v2012 / v1840;
            let v2014 = v2012 / v1833;
            let v2015 = v2012 * v2000;
            let v2016 = v2012 * v2007;
            let v2017 = v2012 * v2009;
            let v2023 = ((v2012 / v1657) * ((v414 * v1658) + v2020)) * v1354;
            let v2025 = (v1112 + v1111) / v1108;
            let v2028 = v2026 * (v2025.abs());
            let v2030 = if v2029 > v0 { 1.0 } else { 0.0 };
            let v2035: f64;
            if v2030 != 0.0 {
                let v2032 = (v1981 / v2025).abs();
                v2035 = v2032;
            } else {
                v2035 = v0;
            }
            let v2037 = (v2033 * v1981) * (v2035 + v2);
            let v2038 = if v2025 > v0 { 1.0 } else { 0.0 };
            if v2038 != 0.0 {
            } else {
            }
            let v2040 = if v2039 == v2 { 1.0 } else { 0.0 };
            if v2040 != 0.0 {
            } else {
                let v2041 = if v2039 == v26 { 1.0 } else { 0.0 };
                if v2041 != 0.0 {
                } else {
                }
            }
            let v2047 = v2042 * ((((v1974 - v1975) + v1160) + v1139).abs());
            let v2048 = v1972 + v1969;
            let v2053 = v2049 * ((v2048.abs()).powf(v2051));
            let v2054 = if v2048 < v0 { 1.0 } else { 0.0 };
            let v2117: f64;
            if v2054 != 0.0 {
                let v2055 = -v2053;
                v2117 = v2055;
            } else {
                v2117 = v2053;
            }
            let v2057 = (v1266 + v1276) + v1296;
            let v2062 = v2058 * ((v2057.abs()).powf(v2060));
            let v2063 = if v2057 < v0 { 1.0 } else { 0.0 };
            let v2119: f64;
            if v2063 != 0.0 {
                let v2064 = -v2062;
                v2119 = v2064;
            } else {
                v2119 = v2062;
            }
            let v2067 = v2065 * (v1971.abs());
            let v2069 = v1633.abs();
            let v2070 = v2068 * v2069;
            let v2072 = v2049 * (v2069.powf(v2051));
            let v2073 = if v1633 < v0 { 1.0 } else { 0.0 };
            let v2123: f64;
            if v2073 != 0.0 {
                let v2074 = -v2072;
                v2123 = v2074;
            } else {
                v2123 = v2072;
            }
            let v2077 = v2075 * (v1630.abs());
            let v2079 = v1632.abs();
            let v2080 = v2078 * v2079;
            let v2082 = v2 - (v1504 * v8);
            let v2086 = (v2049 * v2082) * ((v2079 / v2082).powf(v2051));
            let v2087 = if v1632 < v0 { 1.0 } else { 0.0 };
            let v2126: f64;
            if v2087 != 0.0 {
                let v2088 = -v2086;
                v2126 = v2088;
            } else {
                v2126 = v2086;
            }
            let v2090 = v1635.abs();
            let v2092 = (v2089 * v2090) * v1504;
            let v2093 = if v8 == v0 { 1.0 } else { 0.0 };
            let v2100: f64;
            if v2093 != 0.0 {
                v2100 = v0;
            } else {
                let v2098 = ((v2049 * v1504) * v8) * ((v2090 / v8).powf(v2051));
                v2100 = v2098;
            }
            let v2099 = if v1635 < v0 { 1.0 } else { 0.0 };
            let v2129: f64;
            if v2099 != 0.0 {
                let v2101 = -v2100;
                v2129 = v2101;
            } else {
                v2129 = v2100;
            }
            let v2104 = v2102 * (v1977.abs());
            let v2107 = v2105 * (v1976.abs());
            let v2110 = v2108 * (v1978.abs());
            let v2111 = v2028 * v20;
            let v2112 = v2037 * v20;
            let v2113 = v2047 * v20;
            let v2114 = v2013 * v20;
            let v2115 = v2014 * v20;
            let v2116 = v2023 * v20;
            let v2118 = v2117 * v20;
            let v2120 = v2119 * v20;
            let v2121 = v2067 * v20;
            let v2122 = v2070 * v20;
            let v2124 = v2123 * v20;
            let v2125 = v2080 * v20;
            let v2127 = v2126 * v20;
            let v2128 = v2092 * v20;
            let v2130 = v2129 * v20;
            let v2151: f64;
            let v2152: f64;
            let v2153: f64;
            let v2154: f64;
            if v489 != 0.0 {
                let v2131 = v2077 * v20;
                v2151 = v2;
                v2152 = v2131;
                v2153 = v0;
                v2154 = v0;
            } else {
                let v2132 = v2077 * v20;
                v2151 = v0;
                v2152 = v0;
                v2153 = v2;
                v2154 = v2132;
            }
            let v2133 = v2104 * v20;
            let v2134 = v2107 * v20;
            let v2135 = v2110 * v20;
            let v2155: f64;
            let v2157: f64;
            let v2159: f64;
            let v2161: f64;
            let v2163: f64;
            let v2165: f64;
            let v2167: f64;
            let v2169: f64;
            let v2171: f64;
            let v2173: f64;
            let v2175: f64;
            let v2177: f64;
            let v2179: f64;
            let v2181: f64;
            let v2183: f64;
            let v2185: f64;
            if v660 != 0.0 {
                let v2156: f64;
                let v2158: f64;
                let v2160: f64;
                let v2162: f64;
                let v2164: f64;
                let v2166: f64;
                let v2168: f64;
                let v2170: f64;
                let v2172: f64;
                let v2174: f64;
                if v663 != 0.0 {
                    let v2136 = v2015 * v20;
                    let v2137 = v2016 * v20;
                    let v2138 = v2017 * v20;
                    v2156 = v2;
                    v2158 = v2136;
                    v2160 = v2;
                    v2162 = v2137;
                    v2164 = v2;
                    v2166 = v2138;
                    v2168 = v0;
                    v2170 = v0;
                    v2172 = v0;
                    v2174 = v0;
                } else {
                    let v2139 = v2015 * v20;
                    let v2140 = v2016 * v20;
                    v2156 = v0;
                    v2158 = v0;
                    v2160 = v0;
                    v2162 = v0;
                    v2164 = v0;
                    v2166 = v0;
                    v2168 = v2;
                    v2170 = v2139;
                    v2172 = v2;
                    v2174 = v2140;
                }
                v2155 = v2156;
                v2157 = v2158;
                v2159 = v2160;
                v2161 = v2162;
                v2163 = v2164;
                v2165 = v2166;
                v2167 = v2168;
                v2169 = v2170;
                v2171 = v2172;
                v2173 = v2174;
                v2175 = v0;
                v2177 = v0;
                v2179 = v0;
                v2181 = v0;
                v2183 = v0;
                v2185 = v0;
            } else {
                let v2176: f64;
                let v2178: f64;
                let v2180: f64;
                let v2182: f64;
                let v2184: f64;
                let v2186: f64;
                if v663 != 0.0 {
                    let v2141 = v2015 * v20;
                    let v2142 = v2017 * v20;
                    v2176 = v2;
                    v2178 = v2141;
                    v2180 = v2;
                    v2182 = v2142;
                    v2184 = v0;
                    v2186 = v0;
                } else {
                    let v2143 = v2015 * v20;
                    v2176 = v0;
                    v2178 = v0;
                    v2180 = v0;
                    v2182 = v0;
                    v2184 = v2;
                    v2186 = v2143;
                }
                v2155 = v0;
                v2157 = v0;
                v2159 = v0;
                v2161 = v0;
                v2163 = v0;
                v2165 = v0;
                v2167 = v0;
                v2169 = v0;
                v2171 = v0;
                v2173 = v0;
                v2175 = v2176;
                v2177 = v2178;
                v2179 = v2180;
                v2181 = v2182;
                v2183 = v2184;
                v2185 = v2186;
            }
            let v2149 = if (((((v1980 + v1987) + v1992) + v1997) + v1999) + v2006) == v0 { 1.0 } else { 0.0 };
            if v2149 != 0.0 {
            } else {
            }
            let v2150 = if v20 != v2 { 1.0 } else { 0.0 };
            if v2150 != 0.0 {
            } else {
            }
        {
            let psd = v2111;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 0, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 0, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 0, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(0, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = v2112;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 1, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(1, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = v2113;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 2, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 2, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 2, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(2, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = v2114;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 3, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 3, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 3, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(3, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = v2115;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 4, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 4, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 4, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(4, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = v2116;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 5, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 5, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 5, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(5, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = v2118;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 6, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 6, value: psd }); }
            let exponent: Option<f64> = Some(v2);
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 6, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 6, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(6, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = v2120;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 7, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 7, value: psd }); }
            let exponent: Option<f64> = Some(v2);
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 7, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 7, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(7, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = v2121;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 8, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 8, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 8, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(8, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = v2122;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 9, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 9, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 9, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(9, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = v2124;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 10, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 10, value: psd }); }
            let exponent: Option<f64> = Some(v2);
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 10, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 10, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(10, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = v2125;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 11, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 11, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 11, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(11, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = v2127;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 12, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 12, value: psd }); }
            let exponent: Option<f64> = Some(v2);
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 12, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 12, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(12, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = v2128;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 13, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 13, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 13, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(13, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = v2130;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 14, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 14, value: psd }); }
            let exponent: Option<f64> = Some(v2);
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 14, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 14, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(14, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v2151 == 0.0 {
            if !visitor.visit(15, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v2152;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 15, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 15, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 15, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(15, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v2153 == 0.0 {
            if !visitor.visit(16, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v2154;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 16, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 16, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 16, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(16, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = v2133;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 17, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 17, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 17, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(17, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = v2134;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 18, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 18, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 18, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(18, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = v2135;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 19, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 19, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 19, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(19, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v2155 == 0.0 {
            if !visitor.visit(20, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v2157;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 20, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 20, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 20, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(20, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v2159 == 0.0 {
            if !visitor.visit(21, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v2161;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 21, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 21, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 21, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(21, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v2163 == 0.0 {
            if !visitor.visit(22, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v2165;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 22, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 22, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 22, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(22, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v2167 == 0.0 {
            if !visitor.visit(23, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v2169;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 23, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 23, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 23, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(23, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v2171 == 0.0 {
            if !visitor.visit(24, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v2173;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 24, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 24, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 24, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(24, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v2175 == 0.0 {
            if !visitor.visit(25, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v2177;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 25, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 25, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 25, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(25, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v2179 == 0.0 {
            if !visitor.visit(26, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v2181;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 26, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 26, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 26, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(26, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v2183 == 0.0 {
            if !visitor.visit(27, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v2185;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 27, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 27, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 27, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(27, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        Ok(())
    }
}
