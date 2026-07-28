#![allow(dead_code, non_snake_case, unused_parens, unused_variables)]

use super::state::Instance;
use crate::device::veriloga_generated::GeneratedEvalContext;
pub use crate::device::veriloga_generated::{GeneratedNoiseDescriptor, GeneratedNoiseEndpoint, GeneratedNoiseEvaluation, GeneratedNoiseEvaluationError, GeneratedNoiseEvaluationRef, GeneratedNoiseKind, GeneratedNoiseVisitor};

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
            let v16 = parameters[154];
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
            let v90 = node_potentials[4];
            let v96 = parameters[125];
            let v105 = 8.617086918058125e-5f64;
            let v153 = 3e0f64;
            let v154 = -3e0f64;
            let v160 = parameters[105];
            let v177 = -3e0f64;
            let v180 = parameters[64];
            let v183 = parameters[110];
            let v200 = -3e0f64;
            let v203 = parameters[80];
            let v221 = -3e0f64;
            let v241 = -3e0f64;
            let v260 = -3e0f64;
            let v263 = parameters[27];
            let v266 = parameters[109];
            let v283 = -3e0f64;
            let v286 = parameters[138];
            let v289 = parameters[140];
            let v315 = parameters[139];
            let v316 = parameters[75];
            let v324 = parameters[70];
            let v327 = parameters[54];
            let v328 = parameters[97];
            let v333 = parameters[56];
            let v334 = parameters[98];
            let v335 = parameters[96];
            let v340 = parameters[55];
            let v341 = parameters[101];
            let v346 = parameters[57];
            let v347 = parameters[102];
            let v351 = parameters[58];
            let v352 = parameters[104];
            let v356 = parameters[59];
            let v358 = parameters[60];
            let v359 = parameters[99];
            let v363 = parameters[122];
            let v365 = parameters[10];
            let v384 = 6.931471805599453e-4f64;
            let v386 = parameters[123];
            let v388 = parameters[11];
            let v407 = 6.931471805599453e-4f64;
            let v409 = parameters[43];
            let v410 = parameters[124];
            let v414 = 1e-6f64;
            let v417 = 5e-1f64;
            let v418 = 5e-7f64;
            let v427 = parameters[9];
            let v428 = 4e0f64;
            let v431 = parameters[121];
            let v443 = parameters[12];
            let v448 = parameters[30];
            let v449 = parameters[103];
            let v454 = parameters[20];
            let v455 = 6e0f64;
            let v456 = parameters[21];
            let v462 = parameters[113];
            let v468 = parameters[31];
            let v469 = parameters[32];
            let v480 = parameters[16];
            let v484 = parameters[17];
            let v488 = parameters[111];
            let v494 = parameters[18];
            let v495 = parameters[19];
            let v502 = parameters[24];
            let v504 = parameters[25];
            let v505 = parameters[107];
            let v511 = parameters[28];
            let v512 = parameters[106];
            let v517 = parameters[26];
            let v518 = parameters[108];
            let v524 = parameters[29];
            let v530 = parameters[112];
            let v535 = parameters[22];
            let v536 = parameters[23];
            let v545 = parameters[149];
            let v546 = parameters[150];
            let v554 = parameters[155];
            let v557 = parameters[157];
            let v563 = -5e-1f64;
            let v566 = parameters[35];
            let v575 = parameters[34];
            let v587 = -5e-1f64;
            let v590 = parameters[37];
            let v599 = parameters[36];
            let v611 = parameters[14];
            let v614 = parameters[13];
            let v617 = parameters[133];
            let v618 = parameters[141];
            let v627 = parameters[135];
            let v632 = parameters[87];
            let v638 = parameters[88];
            let v643 = parameters[89];
            let v648 = parameters[90];
            let v649 = parameters[100];
            let v654 = 3e2f64;
            let v656 = 5.25e2f64;
            let v659 = 7.2e-4f64;
            let v662 = 1.6e-6f64;
            let v667 = 1.081e0f64;
            let v669 = parameters[92];
            let v671 = parameters[146];
            let v672 = parameters[148];
            let v682 = node_potentials[7];
            let v683 = node_potentials[8];
            let v686 = node_potentials[9];
            let v689 = node_potentials[5];
            let v692 = node_potentials[6];
            let v697 = node_potentials[3];
            let v702 = node_potentials[2];
            let v703 = node_potentials[1];
            let v708 = node_potentials[0];
            let v711 = node_potentials[11];
            let v714 = node_potentials[10];
            let v728 = parameters[151];
            let v824 = parameters[153];
            let v835 = 1e2f64;
            let v851 = 2e-1f64;
            let v866 = parameters[62];
            let v867 = parameters[61];
            let v877 = parameters[63];
            let v892 = -1e0f64;
            let v935 = parameters[152];
            let v953 = parameters[73];
            let v969 = 1e-5f64;
            let v973 = 1e-40f64;
            let v989 = -1e0f64;
            let v1020 = parameters[74];
            let v1028 = -1e0f64;
            let v1052 = parameters[76];
            let v1107 = 1.0000000000000002e-2f64;
            let v1111 = 5.000000000000001e-3f64;
            let v1125 = parameters[15];
            let v1131 = 1e-4f64;
            let v1145 = parameters[156];
            let v1156 = parameters[158];
            let v1171 = parameters[159];
            let v1194 = 1e3f64;
            let v1196 = 4e1f64;
            let v1199 = 2.3538526683702e17f64;
            let v1227 = parameters[93];
            let v1329 = 1e-30f64;
            let v1332 = -2e0f64;
            let v1348 = 1.6666666666666666e-1f64;
            let v1354 = -1e-3f64;
            let v1370 = 3.333333333333333e-1f64;
            let v1372 = 2.5e-1f64;
            let v1407 = -2e0f64;
            let v1428 = -1e-3f64;
            let v1469 = parameters[8];
            let v1471 = parameters[143];
            let v1479 = parameters[144];
            let v1520 = parameters[5];
            let v1574 = 1.21e-2f64;
            let v1577 = 6.05e-3f64;
            let v1596 = parameters[84];
            let v1599 = 1e-6f64;
            let v1600 = 1e-12f64;
            let v1601 = -1e0f64;
            let v1603 = -1e0f64;
            let v1606 = -1e0f64;
            let v1609 = 5e-13f64;
            let v1612 = -1e0f64;
            let v1618 = -1e0f64;
            let v1622 = parameters[82];
            let v1626 = parameters[81];
            let v1656 = 1.0000000000000002e-2f64;
            let v1659 = 5.000000000000001e-3f64;
            let v1676 = parameters[39];
            let v1678 = parameters[44];
            let v1681 = parameters[42];
            let v1694 = parameters[41];
            let v1703 = parameters[40];
            let v1710 = parameters[46];
            let v1712 = parameters[45];
            let v1720 = parameters[7];
            let v1740 = parameters[47];
            let v1770 = 1e-7f64;
            let v1796 = parameters[48];
            let v1800 = parameters[49];
            let v1804 = parameters[52];
            let v1808 = parameters[51];
            let v1823 = parameters[50];
            let v1847 = parameters[53];
            let v1903 = parameters[77];
            let v1937 = -1e0f64;
            let v1943 = parameters[85];
            let v1950 = parameters[79];
            let v1954 = parameters[91];
            let v2002 = parameters[6];
            let v2012 = parameters[145];
            let v2016 = parameters[69];
            let v2021 = parameters[78];
            let v2033 = 5.5224904e-23f64;
            let v2042 = 5e0f64;
            let v2048 = 3.2043836e-19f64;
            let v2051 = parameters[130];
            let v2055 = 3.2043836e-19f64;
            let v2061 = parameters[131];
            let v2064 = 3.2043836e-19f64;
            let v2071 = parameters[128];
            let v2073 = parameters[126];
            let v2080 = parameters[129];
            let v2082 = parameters[127];
            let v2087 = 3.2043836e-19f64;
            let v2090 = 3.2043836e-19f64;
            let v2097 = 3.2043836e-19f64;
            let v2100 = 3.2043836e-19f64;
            let v2111 = 3.2043836e-19f64;
            let v2124 = 3.2043836e-19f64;
            let v2127 = 3.2043836e-19f64;
            let v2130 = 3.2043836e-19f64;
            let v3 = if v1 == v2 { 1.0 } else { 0.0 };
            let v658: f64;
            let v1773: f64;
            if v3 != 0.0 {
                v658 = v5;
                v1773 = v4;
            } else {
                v658 = v7;
                v1773 = v6;
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
            let v113: f64;
            if v43 != 0.0 {
                let v48 = v39 + (v41 * ((v2 + (v42.exp())).ln()));
                v113 = v48;
            } else {
                let v54 = v38 + (v41 * ((v2 + ((-v42).exp())).ln()));
                v113 = v54;
            }
            let v55 = v2 / v31;
            let v57 = v2 / v56;
            let v61 = v26.powf((v26 - v59));
            let v62 = v2 / v61;
            let v70 = v63 + (((v64 * v12) * v12) / (v12 + v67));
            let v72 = (v70 - v39) / v41;
            let v73 = if v70 < v39 { 1.0 } else { 0.0 };
            let v133: f64;
            if v73 != 0.0 {
                let v78 = v39 + (v41 * ((v2 + (v72.exp())).ln()));
                v133 = v78;
            } else {
                let v84 = v70 + (v41 * ((v2 + ((-v72).exp())).ln()));
                v133 = v84;
            }
            let v85 = v2 / v63;
            let v86 = v2 / v58;
            let v89 = v2 - (v2 / v87);
            let v91 = if v90 < v0 { 1.0 } else { 0.0 };
            let v95: f64;
            if v91 != 0.0 {
                let v94 = -((v2 - v90).ln());
                v95 = v94;
            } else {
                v95 = v90;
            }
            let v97 = if v95 < v96 { 1.0 } else { 0.0 };
            let v102: f64;
            if v97 != 0.0 {
                v102 = v95;
            } else {
                let v101 = v96 + ((v2 + (v95 - v96)).ln());
                v102 = v101;
            }
            let v103 = v15 + v102;
            let v104 = v103 / v12;
            let v106 = v105 * v103;
            let v108 = v2 / v106;
            let v110 = v108 - (v2 / (v105 * v12));
            let v111 = v103 - v12;
            let v112 = v104.ln();
            let v118 = v113 - (((v32 * v103) * v103) / (v103 + v35));
            let v120 = (v118 - v39) / v41;
            let v121 = if v118 < v39 { 1.0 } else { 0.0 };
            let v561: f64;
            if v121 != 0.0 {
                let v126 = v39 + (v41 * ((v2 + (v120.exp())).ln()));
                v561 = v126;
            } else {
                let v132 = v118 + (v41 * ((v2 + ((-v120).exp())).ln()));
                v561 = v132;
            }
            let v138 = v133 - (((v64 * v103) * v103) / (v103 + v67));
            let v140 = (v138 - v39) / v41;
            let v141 = if v138 < v39 { 1.0 } else { 0.0 };
            let v585: f64;
            if v141 != 0.0 {
                let v146 = v39 + (v41 * ((v2 + (v140.exp())).ln()));
                v585 = v146;
            } else {
                let v152 = v138 + (v41 * ((v2 + ((-v140).exp())).ln()));
                v585 = v152;
            }
            let v159 = v2 - v104;
            let v162 = (((v154 * v106) * v112) + (v56 * v104)) + (v159 * v160);
            let v164 = (v39 - v162) / v106;
            let v165 = if v39 < v162 { 1.0 } else { 0.0 };
            let v306: f64;
            if v165 != 0.0 {
                let v170 = v162 + (v106 * ((v2 + (v164.exp())).ln()));
                v306 = v170;
            } else {
                let v176 = v39 + (v106 * ((v2 + ((-v164).exp())).ln()));
                v306 = v176;
            }
            let v184 = v159 * v183;
            let v185 = (((v177 * v106) * v112) + (v180 * v104)) + v184;
            let v187 = (v39 - v185) / v106;
            let v188 = if v39 < v185 { 1.0 } else { 0.0 };
            let v785: f64;
            if v188 != 0.0 {
                let v193 = v185 + (v106 * ((v2 + (v187.exp())).ln()));
                v785 = v193;
            } else {
                let v199 = v39 + (v106 * ((v2 + ((-v187).exp())).ln()));
                v785 = v199;
            }
            let v206 = (((v200 * v106) * v112) + (v203 * v104)) + v184;
            let v208 = (v39 - v206) / v106;
            let v209 = if v39 < v206 { 1.0 } else { 0.0 };
            let v1952: f64;
            if v209 != 0.0 {
                let v214 = v206 + (v106 * ((v2 + (v208.exp())).ln()));
                v1952 = v214;
            } else {
                let v220 = v39 + (v106 * ((v2 + ((-v208).exp())).ln()));
                v1952 = v220;
            }
            let v224 = v58 * v104;
            let v226 = (((v221 * v106) * v112) + v224) + v184;
            let v228 = (v39 - v226) / v106;
            let v229 = if v39 < v226 { 1.0 } else { 0.0 };
            let v318: f64;
            if v229 != 0.0 {
                let v234 = v226 + (v106 * ((v2 + (v228.exp())).ln()));
                v318 = v234;
            } else {
                let v240 = v39 + (v106 * ((v2 + ((-v228).exp())).ln()));
                v318 = v240;
            }
            let v245 = (((v241 * v106) * v112) + v224) + v184;
            let v247 = (v39 - v245) / v106;
            let v248 = if v39 < v245 { 1.0 } else { 0.0 };
            let v308: f64;
            if v248 != 0.0 {
                let v253 = v245 + (v106 * ((v2 + (v247.exp())).ln()));
                v308 = v253;
            } else {
                let v259 = v39 + (v106 * ((v2 + ((-v247).exp())).ln()));
                v308 = v259;
            }
            let v268 = (((v260 * v106) * v112) + (v263 * v104)) + (v159 * v266);
            let v270 = (v39 - v268) / v106;
            let v271 = if v39 < v268 { 1.0 } else { 0.0 };
            let v1184: f64;
            if v271 != 0.0 {
                let v276 = v268 + (v106 * ((v2 + (v270.exp())).ln()));
                v1184 = v276;
            } else {
                let v282 = v39 + (v106 * ((v2 + ((-v270).exp())).ln()));
                v1184 = v282;
            }
            let v291 = (((v283 * v106) * v112) + (v286 * v104)) + (v159 * v289);
            let v293 = (v39 - v291) / v106;
            let v294 = if v39 < v291 { 1.0 } else { 0.0 };
            let v314: f64;
            if v294 != 0.0 {
                let v299 = v291 + (v106 * ((v2 + (v293.exp())).ln()));
                v314 = v299;
            } else {
                let v305 = v39 + (v106 * ((v2 + ((-v293).exp())).ln()));
                v314 = v305;
            }
            let v307 = v2 / v306;
            let v309 = v2 / v308;
            let v311 = (v56 * v307).powf(v27);
            let v313 = (v58 * v309).powf(v59);
            let v322 = ((v2 - v316) * ((v58 / v318).powf(v59))) + v316;
            let v323 = v2 / v322;
            let v325 = v324 * v322;
            let v326 = v316 * v323;
            let v331 = v327 * ((v112 * v328).exp());
            let v332 = if v331 < v21 { 1.0 } else { 0.0 };
            let v1856: f64;
            if v332 != 0.0 {
                v1856 = v21;
            } else {
                v1856 = v331;
            }
            let v339 = v333 * ((v112 * (v334 - v335)).exp());
            let v344 = v340 * ((v112 * v341).exp());
            let v345 = if v344 < v21 { 1.0 } else { 0.0 };
            let v1849: f64;
            if v345 != 0.0 {
                v1849 = v21;
            } else {
                v1849 = v344;
            }
            let v350 = v346 * ((v112 * v347).exp());
            let v354 = (v112 * v352).exp();
            let v355 = v351 * v354;
            let v357 = v356 * v354;
            let v362 = v358 * ((v112 * v359).exp());
            let v364 = if v363 != v0 { 1.0 } else { 0.0 };
            let v434: f64;
            if v364 != 0.0 {
                let v368 = v365 * (v2 + (v111 * v363));
                let v370 = (v368 - v2) / v25;
                let v371 = if v368 < v2 { 1.0 } else { 0.0 };
                let v383: f64;
                if v371 != 0.0 {
                    let v376 = v2 + (v25 * ((v2 + (v370.exp())).ln()));
                    v383 = v376;
                } else {
                    let v382 = v368 + (v25 * ((v2 + ((-v370).exp())).ln()));
                    v383 = v382;
                }
                let v385 = v383 - v384;
                v434 = v385;
            } else {
                v434 = v365;
            }
            let v387 = if v386 != v0 { 1.0 } else { 0.0 };
            let v1079: f64;
            if v387 != 0.0 {
                let v391 = v388 * (v2 + (v111 * v386));
                let v393 = (v391 - v2) / v25;
                let v394 = if v391 < v2 { 1.0 } else { 0.0 };
                let v406: f64;
                if v394 != 0.0 {
                    let v399 = v2 + (v25 * ((v2 + (v393.exp())).ln()));
                    v406 = v399;
                } else {
                    let v405 = v391 + (v25 * ((v2 + ((-v393).exp())).ln()));
                    v406 = v405;
                }
                let v408 = v406 - v407;
                v1079 = v408;
            } else {
                v1079 = v388;
            }
            let v413 = v409 * (v2 + (v410 * v111));
            let v415 = v413 * v413;
            let v416 = if v413 < v0 { 1.0 } else { 0.0 };
            let v1692: f64;
            if v416 != 0.0 {
                let v422 = v418 / (((v415 + v414).sqrt()) - v413);
                v1692 = v422;
            } else {
                let v426 = v417 * (((v415 + v414).sqrt()) + v413);
                v1692 = v426;
            }
            let v442 = (v427 * (((v112 * (((v428 - v334) - v335) + v431)) / v434).exp())) * ((((-v160) * v110) / v434).exp());
            let v447 = v443 * ((v112 * (v2 - v334)).exp());
            let v453 = v448 * ((v112 * (v2 - v449)).exp());
            let v464 = (-v462) * v110;
            let v467 = (v454 * ((v112 * (v455 - (v26 * v456))).exp())) * ((v464 / v456).exp());
            let v479 = (v468 * ((v112 * (v455 - (v26 * v469))).exp())) * ((((-v183) * v110) / v469).exp());
            let v483 = v112 * ((v428 - v328) + v431);
            let v490 = (-v488) * v110;
            let v493 = (v480 * ((v483 / v484).exp())) * ((v490 / v484).exp());
            let v501 = (v494 * ((v483 / v495).exp())) * ((v490 / v495).exp());
            let v503 = if v502 == v2 { 1.0 } else { 0.0 };
            let v1206: f64;
            let v1219: f64;
            let v1261: f64;
            if v503 != 0.0 {
                let v510 = v504 * ((((-v505) * v110) / v484).exp());
                let v516 = v511 * (((-v512) * v110).exp());
                let v523 = v517 * ((((-v518) * v110) / v495).exp());
                v1206 = v510;
                v1219 = v516;
                v1261 = v523;
            } else {
                v1206 = v0;
                v1219 = v0;
                v1261 = v0;
            }
            let v534 = (v524 * ((v112 * ((v428 - v449) + v431)).exp())) * (((-v530) * v110).exp());
            let v544 = (v535 * ((v112 * (v455 - (v26 * v536))).exp())) * ((v464 / v536).exp());
            let v553 = (v545 * ((v112 * (v428 / v546)).exp())) * ((v464 / v546).exp());
            let v560 = (v554 * (v104.sqrt())) * ((v557 * v111).exp());
            let v564 = (v561 * v55).powf(v563);
            let v565 = v2 / v311;
            let v574 = (((((((v566 * v561) * v561) * v564) * v565) * v56) * v307) * v55) * v55;
            let v584 = ((((((v575 * v564) * v306) * v306) * v57) * v57) * v311) * ((v566 - v574).exp());
            let v588 = (v585 * v85).powf(v587);
            let v598 = (((((((v590 * v585) * v585) * v588) * (v2 / v313)) * v58) * v309) * v85) * v85;
            let v608 = ((((((v599 * v588) * v308) * v308) * v86) * v86) * v313) * ((v590 - v598).exp());
            let v610 = (v112 * v335).exp();
            let v613 = (v611 * v610) * v323;
            let v616 = (v614 * v610) * v565;
            let v626 = (v617 * ((v112 * (v428 - v618)).exp())) * (((-v289) * v110).exp());
            let v631 = v627 * ((v112 * (v2 - v618)).exp());
            let v637 = v632 * ((v112 * ((v335 + v334) - v2)).exp());
            let v642 = v638 * ((v112 * (v359 - v2)).exp());
            let v644 = v637 + v642;
            let v647 = (v643 * v644) / (v632 + v638);
            let v653 = v648 * ((v112 * (v649 - v2)).exp());
            let v655 = v103 - v654;
            let v657 = if v103 < v656 { 1.0 } else { 0.0 };
            let v1774: f64;
            if v657 != 0.0 {
                let v666 = v658 * ((v2 + (v659 * v655)) - ((v662 * v655) * v655));
                v1774 = v666;
            } else {
                let v668 = v658 * v667;
                v1774 = v668;
            }
            let v670 = v669 * v610;
            let v673 = if v346 > v0 { 1.0 } else { 0.0 };
            let v1886: f64;
            if v673 != 0.0 {
                let v674 = v2 / v350;
                let v675 = if v674 > v22 { 1.0 } else { 0.0 };
                let v1887: f64;
                if v675 != 0.0 {
                    v1887 = v22;
                } else {
                    v1887 = v674;
                }
                v1886 = v1887;
            } else {
                v1886 = v0;
            }
            let v676 = if v351 > v0 { 1.0 } else { 0.0 };
            let v1888: f64;
            if v676 != 0.0 {
                let v677 = v2 / v355;
                let v678 = if v677 > v22 { 1.0 } else { 0.0 };
                let v1889: f64;
                if v678 != 0.0 {
                    v1889 = v22;
                } else {
                    v1889 = v677;
                }
                v1888 = v1889;
            } else {
                v1888 = v0;
            }
            let v679 = if v356 > v0 { 1.0 } else { 0.0 };
            let v1890: f64;
            if v679 != 0.0 {
                let v680 = v2 / v357;
                let v681 = if v680 > v22 { 1.0 } else { 0.0 };
                let v1891: f64;
                if v681 != 0.0 {
                    v1891 = v22;
                } else {
                    v1891 = v680;
                }
                v1890 = v1891;
            } else {
                v1890 = v0;
            }
            let v685 = v1 * (v682 - v683);
            let v688 = v1 * (v682 - v686);
            let v691 = v1 * (v682 - v689);
            let v694 = v1 * (v692 - v689);
            let v696 = v1 * (v692 - v682);
            let v699 = v1 * (v697 - v683);
            let v701 = v1 * (v683 - v686);
            let v705 = v1 * (v703 - v692);
            let v707 = v1 * (v703 - v702);
            let v710 = v1 * (v703 - v708);
            let v713 = v1 * (v711 - v683);
            let v716 = v1 * (v714 - v711);
            let v719 = ((v696 + v688) - v701) - v713;
            let v724 = v710 + ((((-v710) + v705) + v719) - v716);
            let v725 = v699 - v713;
            let v726 = v725 - v716;
            let v727 = v688 * v108;
            let v729 = if v727 < v728 { 1.0 } else { 0.0 };
            let v967: f64;
            if v729 != 0.0 {
                let v730 = v727.exp();
                v967 = v730;
            } else {
                let v734 = (v728.exp()) * (v2 + (v727 - v728));
                v967 = v734;
            }
            let v735 = v691 * v108;
            let v736 = v735 / v434;
            let v737 = if v736 < v728 { 1.0 } else { 0.0 };
            let v1072: f64;
            if v737 != 0.0 {
                let v738 = v736.exp();
                v1072 = v738;
            } else {
                let v742 = (v728.exp()) * (v2 + (v736 - v728));
                v1072 = v742;
            }
            let v743 = v719 * v108;
            let v744 = if v743 < v728 { 1.0 } else { 0.0 };
            let v1458: f64;
            if v744 != 0.0 {
                let v745 = v743.exp();
                v1458 = v745;
            } else {
                let v749 = (v728.exp()) * (v2 + (v743 - v728));
                v1458 = v749;
            }
            let v750 = v696 * v108;
            let v751 = if v750 < v728 { 1.0 } else { 0.0 };
            let v1674: f64;
            if v751 != 0.0 {
                let v752 = v750.exp();
                v1674 = v752;
            } else {
                let v756 = (v728.exp()) * (v2 + (v750 - v728));
                v1674 = v756;
            }
            let v757 = v724 * v108;
            let v758 = if v757 < v728 { 1.0 } else { 0.0 };
            let v1529: f64;
            if v758 != 0.0 {
                let v759 = v757.exp();
                v1529 = v759;
            } else {
                let v763 = (v728.exp()) * (v2 + (v757 - v728));
                v1529 = v763;
            }
            let v764 = v699 * v108;
            let v765 = if v764 < v728 { 1.0 } else { 0.0 };
            let v1474: f64;
            if v765 != 0.0 {
                let v766 = v764.exp();
                v1474 = v766;
            } else {
                let v770 = (v728.exp()) * (v2 + (v764 - v728));
                v1474 = v770;
            }
            let v771 = v726 * v108;
            let v772 = if v771 < v728 { 1.0 } else { 0.0 };
            let v1541: f64;
            if v772 != 0.0 {
                let v773 = v771.exp();
                v1541 = v773;
            } else {
                let v777 = (v728.exp()) * (v2 + (v771 - v728));
                v1541 = v777;
            }
            let v778 = v725 * v108;
            let v779 = if v778 < v728 { 1.0 } else { 0.0 };
            let v1490: f64;
            if v779 != 0.0 {
                let v780 = v778.exp();
                v1490 = v780;
            } else {
                let v784 = (v728.exp()) * (v2 + (v778 - v728));
                v1490 = v784;
            }
            let v787 = (v724 - v785) * v108;
            let v788 = if v787 < v728 { 1.0 } else { 0.0 };
            let v1968: f64;
            if v788 != 0.0 {
                let v789 = v787.exp();
                v1968 = v789;
            } else {
                let v793 = (v728.exp()) * (v2 + (v787 - v728));
                v1968 = v793;
            }
            let v796 = if ((v719 - v785) * v108) < v728 { 1.0 } else { 0.0 };
            if v796 != 0.0 {
            } else {
            }
            let v798 = (v688 - v785) * v108;
            let v799 = if v798 < v728 { 1.0 } else { 0.0 };
            let v813: f64;
            if v799 != 0.0 {
                let v800 = v798.exp();
                v813 = v800;
            } else {
                let v804 = (v728.exp()) * (v2 + (v798 - v728));
                v813 = v804;
            }
            let v806 = (v685 - v785) * v108;
            let v807 = if v806 < v728 { 1.0 } else { 0.0 };
            let v817: f64;
            if v807 != 0.0 {
                let v808 = v806.exp();
                v817 = v808;
            } else {
                let v812 = (v728.exp()) * (v2 + (v806 - v728));
                v817 = v812;
            }
            let v816 = (v2 + (v428 * v813)).sqrt();
            let v820 = (v2 + (v428 * v817)).sqrt();
            let v822 = v2 + v820;
            let v823 = (v26 * v817) / v822;
            let v825 = if v823 < v824 { 1.0 } else { 0.0 };
            let v912: f64;
            if v825 != 0.0 {
                v912 = v824;
            } else {
                v912 = v823;
            }
            let v827 = v816 + v2;
            let v831 = v106 * ((v816 - v820) - ((v827 / v822).ln()));
            let v833 = (v831 + v701) / v362;
            let v834 = if v833 > v0 { 1.0 } else { 0.0 };
            let v1023: f64;
            let v1036: f64;
            let v1051: f64;
            let v1078: f64;
            let v1722: f64;
            let v1758: f64;
            if v834 != 0.0 {
                let v836 = if v685 < v835 { 1.0 } else { 0.0 };
                let v849: f64;
                if v836 != 0.0 {
                    v849 = v685;
                } else {
                    let v840 = v835 + ((v2 + (v685 - v835)).ln());
                    v849 = v840;
                }
                let v843 = (v417 * v833) * v362;
                let v850 = (v785 + ((v26 * v106) * (((v843 * v108) + v2).ln()))) - v849;
                let v852 = v851 * v785;
                let v853 = v852 * v852;
                let v854 = v850 * v850;
                let v855 = if v850 < v0 { 1.0 } else { 0.0 };
                let v865: f64;
                if v855 != 0.0 {
                    let v860 = (v417 * v853) / (((v854 + v853).sqrt()) - v850);
                    v865 = v860;
                } else {
                    let v864 = v417 * (((v854 + v853).sqrt()) + v850);
                    v865 = v864;
                }
                let v868 = v866 * v867;
                let v874 = (v865 * (v865 + v868)) / (v867 * (v865 + (v866 * v362)));
                let v875 = v833 / v874;
                let v878 = (v875 - v2) / v877;
                let v879 = if v875 < v2 { 1.0 } else { 0.0 };
                let v891: f64;
                if v879 != 0.0 {
                    let v884 = v2 + (v877 * ((v2 + (v878.exp())).ln()));
                    v891 = v884;
                } else {
                    let v890 = v875 + (v877 * ((v2 + ((-v878).exp())).ln()));
                    v891 = v890;
                }
                let v899 = v891 / (v2 + (v877 * ((v2 + ((v892 / v877).exp())).ln())));
                let v900 = v865 / v868;
                let v903 = v2 + v900;
                let v910 = (v2 + ((v2 + (((v428 * v899) * v900) * v903)).sqrt())) / ((v26 * v899) * v903);
                let v913 = v912 * v910;
                let v916 = ((v2 - v910) + v913) / (v2 + v913);
                let v918 = (v843 * v916) * v108;
                let v923 = (v26 * v918) + (v912 * ((v912 + v918) + v2));
                let v925 = v417 * (v918 - v2);
                let v927 = (v925 * v925) + v923;
                let v928 = if v918 >= v2 { 1.0 } else { 0.0 };
                let v934: f64;
                if v928 != 0.0 {
                    let v930 = v925 + (v927.sqrt());
                    v934 = v930;
                } else {
                    let v933 = v923 / ((v927.sqrt()) - v925);
                    v934 = v933;
                }
                let v936 = if v934 < v935 { 1.0 } else { 0.0 };
                let v937: f64;
                if v936 != 0.0 {
                    v937 = v935;
                } else {
                    v937 = v934;
                }
                let v942 = (v937 * (v937 + v2)) * ((v785 * v108).exp());
                let v945 = (v417 * v867) * (v833 - v866);
                let v952 = v945 + (((v945 * v945) + (((v867 * v362) * v866) * v833)).sqrt());
                let v954 = if v953 == v0 { 1.0 } else { 0.0 };
                let v1037: f64;
                if v954 != 0.0 {
                    let v955 = v318 * v41;
                    v1037 = v955;
                } else {
                    let v960 = v318 * (v41 + ((v26 * v833) / (v833 + v874)));
                    v1037 = v960;
                }
                let v962 = v866 + v833;
                let v963 = (v866 * v833) / v962;
                let v964 = v866 / v962;
                v1023 = v952;
                v1036 = v1037;
                v1051 = v964;
                v1078 = v942;
                v1722 = v916;
                v1758 = v963;
            } else {
                let v966 = (v26 * v813) / v827;
                let v978 = if (if (v701.abs()) < (v969 * v106) { 1.0 } else { 0.0 }) != 0.0 || (if (v831.abs()) < ((v973 * v106) * (v816 + v820)) { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v1723: f64;
                if v978 != 0.0 {
                    let v980 = v417 * (v966 + v912);
                    let v982 = v980 / (v980 + v2);
                    v1723 = v982;
                } else {
                    let v985 = v831 / ((v831 + v688) - v685);
                    v1723 = v985;
                }
                let v986 = v41 * v318;
                let v988 = v2 - (v833 / v866);
                v1023 = v701;
                v1036 = v986;
                v1051 = v988;
                v1078 = v967;
                v1722 = v1723;
                v1758 = v833;
            }
            let v993 = v306 * (v2 - (v153.powf((v989 / v27))));
            let v994 = v41 * v306;
            let v996 = (v691 - v993) / v994;
            let v997 = if v691 < v993 { 1.0 } else { 0.0 };
            let v1009: f64;
            if v997 != 0.0 {
                let v1002 = v691 - (v994 * ((v2 + (v996.exp())).ln()));
                v1009 = v1002;
            } else {
                let v1008 = v993 - (v994 * ((v2 + ((-v996).exp())).ln()));
                v1009 = v1008;
            }
            let v1012 = v2 - v27;
            let v1013 = (v2 - (v1009 * v307)).powf(v1012);
            let v1019 = ((v306 / v1012) * (v2 - v1013)) + (v153 * (v691 - v1009));
            let v1021 = if v1020 == v2 { 1.0 } else { 0.0 };
            let v1033: f64;
            if v1021 != 0.0 {
                v1033 = v685;
            } else {
                let v1022 = if v1020 == v26 { 1.0 } else { 0.0 };
                let v1034: f64;
                if v1022 != 0.0 {
                    let v1024 = v685 + v1023;
                    v1034 = v1024;
                } else {
                    v1034 = v688;
                }
                v1033 = v1034;
            }
            let v1026 = v2 - v326;
            let v1027 = (v26 - v326) / v1026;
            let v1032 = v318 * (v2 - (v1027.powf((v1028 / v59))));
            let v1038 = (v1033 - v1032) / v1036;
            let v1039 = if v1033 < v1032 { 1.0 } else { 0.0 };
            let v1056: f64;
            if v1039 != 0.0 {
                let v1044 = v1033 - (v1036 * ((v2 + (v1038.exp())).ln()));
                v1056 = v1044;
            } else {
                let v1050 = v1032 - (v1036 * ((v2 + ((-v1038).exp())).ln()));
                v1056 = v1050;
            }
            let v1053 = v1051.powf(v1052);
            let v1054 = v2 - v59;
            let v1055 = v318 / v1054;
            let v1069 = (v1026 * ((v1055 * (v2 - (v1053 * ((v2 - (v1056 / v318)).powf(v1054))))) + ((v1053 * v1027) * (v1033 - v1056)))) + (v326 * v685);
            let v1071 = (v428 * v442) / v447;
            let v1073 = v1071 * v1072;
            let v1077 = v1073 / (v2 + ((v2 + v1073).sqrt()));
            let v1081 = v1078.powf((v2 / v1079));
            let v1082 = v1071 * v1081;
            let v1086 = v1082 / (v2 + ((v2 + v1082).sqrt()));
            let v1087 = if v669 == v0 { 1.0 } else { 0.0 };
            let v1108: f64;
            if v1087 != 0.0 {
                let v1091 = (v2 + (v1019 / v616)) + (v1069 / v613);
                v1108 = v1091;
            } else {
                let v1106 = ((((((v1019 / v616) + v2) * v670) * v108).exp()) - (((((-v1069) / v613) * v670) * v108).exp())) / (((v670 * v108).exp()) - v2);
                v1108 = v1106;
            }
            let v1109 = v1108 * v1108;
            let v1110 = if v1108 < v0 { 1.0 } else { 0.0 };
            let v1120: f64;
            if v1110 != 0.0 {
                let v1115 = v1111 / (((v1109 + v1107).sqrt()) - v1108);
                v1120 = v1115;
            } else {
                let v1119 = v417 * (((v1109 + v1107).sqrt()) + v1108);
                v1120 = v1119;
            }
            let v1123 = v2 + (v417 * (v1077 + v1086));
            let v1124 = v1120 * v1123;
            let v1127 = (v1125 * v442) * v1081;
            let v1128 = v442 * v1072;
            let v1130 = (v1128 - v1127) / v1124;
            let v1132 = v691 / v1131;
            let v1133 = if v691 < v0 { 1.0 } else { 0.0 };
            let v1144: f64;
            if v1133 != 0.0 {
                let v1137 = v1131 * ((v2 + (v1132.exp())).ln());
                v1144 = v1137;
            } else {
                let v1143 = v691 + (v1131 * ((v2 + ((-v1132).exp())).ln()));
                v1144 = v1143;
            }
            let v1146 = v1144 / v1145;
            let v1147 = if v1146 < v728 { 1.0 } else { 0.0 };
            let v1153: f64;
            if v1147 != 0.0 {
                let v1148 = v1146.exp();
                v1153 = v1148;
            } else {
                let v1152 = (v728.exp()) * (v2 + (v1146 - v728));
                v1153 = v1152;
            }
            let v1155 = v560 * (v1153 - v2);
            let v1158 = (v691 - v1156) / v25;
            let v1159 = if v691 < v1156 { 1.0 } else { 0.0 };
            let v1172: f64;
            if v1159 != 0.0 {
                let v1164 = v691 - (v25 * ((v2 + (v1158.exp())).ln()));
                v1172 = v1164;
            } else {
                let v1170 = v1156 - (v25 * ((v2 + ((-v1158).exp())).ln()));
                v1172 = v1170;
            }
            let v1174 = v1156 - v1172;
            let v1176 = (v1171 * v1172) * (v1174 * v1174);
            let v1177 = v735 / v484;
            let v1178 = if v1177 < v728 { 1.0 } else { 0.0 };
            let v1203: f64;
            if v1178 != 0.0 {
                let v1179 = v1177.exp();
                v1203 = v1179;
            } else {
                let v1183 = (v728.exp()) * (v2 + (v1177 - v728));
                v1203 = v1183;
            }
            let v1892: f64;
            if v503 != 0.0 {
                let v1186 = (v691 - v1184) * v108;
                let v1187 = if v1186 < v728 { 1.0 } else { 0.0 };
                let v1209: f64;
                if v1187 != 0.0 {
                    let v1188 = v1186.exp();
                    v1209 = v1188;
                } else {
                    let v1192 = (v728.exp()) * (v2 + (v1186 - v728));
                    v1209 = v1192;
                }
                let v1195 = (v1130 / v442) - v1194;
                let v1197 = if v1195 < v1196 { 1.0 } else { 0.0 };
                let v1222: f64;
                if v1197 != 0.0 {
                    let v1198 = v1195.exp();
                    v1222 = v1198;
                } else {
                    let v1202 = v1199 * (v2 + (v1195 - v1196));
                    v1222 = v1202;
                }
                let v1204 = v1203 - v2;
                let v1226 = ((v493 * v1204) + ((((v1206 * v26) * v1204) / (v2 + ((v2 + (v428 * v1209)).sqrt()))) * (v2 + (v1069 / v613)))) + (((v1219 * (v1078 - v2)) * v1222) / (v2 + v1222));
                v1892 = v1226;
            } else {
                let v1228 = if v1227 == v0 { 1.0 } else { 0.0 };
                let v1893: f64;
                if v1228 != 0.0 {
                    let v1230 = v493 * (v1203 - v2);
                    v1893 = v1230;
                } else {
                    let v1241 = v493 * (((v2 - v1227) * (v1203 - v2)) + ((v1227 * ((v1203 + v1078) - v26)) * (v2 + (v1069 / v613))));
                    v1893 = v1241;
                }
                v1892 = v1893;
            }
            let v1242 = v694 * v108;
            let v1243 = v1242 / v495;
            let v1244 = if v1243 < v728 { 1.0 } else { 0.0 };
            let v1258: f64;
            if v1244 != 0.0 {
                let v1245 = v1243.exp();
                v1258 = v1245;
            } else {
                let v1249 = (v728.exp()) * (v2 + (v1243 - v728));
                v1258 = v1249;
            }
            let v1896: f64;
            if v503 != 0.0 {
                let v1251 = (v694 - v1184) * v108;
                let v1252 = if v1251 < v728 { 1.0 } else { 0.0 };
                let v1264: f64;
                if v1252 != 0.0 {
                    let v1253 = v1251.exp();
                    v1264 = v1253;
                } else {
                    let v1257 = (v728.exp()) * (v2 + (v1251 - v728));
                    v1264 = v1257;
                }
                let v1259 = v1258 - v2;
                let v1270 = (v501 * v1259) + (((v1261 * v26) * v1259) / (v2 + ((v2 + (v428 * v1264)).sqrt())));
                v1896 = v1270;
            } else {
                let v1272 = v501 * (v1258 - v2);
                v1896 = v1272;
            }
            let v1273 = v735 / v456;
            let v1274 = if v1273 < v728 { 1.0 } else { 0.0 };
            let v1280: f64;
            if v1274 != 0.0 {
                let v1275 = v1273.exp();
                v1280 = v1275;
            } else {
                let v1279 = (v728.exp()) * (v2 + (v1273 - v728));
                v1280 = v1279;
            }
            let v1282 = v467 * (v1280 - v2);
            let v1283 = v1242 / v536;
            let v1284 = if v1283 < v728 { 1.0 } else { 0.0 };
            let v1290: f64;
            if v1284 != 0.0 {
                let v1285 = v1283.exp();
                v1290 = v1285;
            } else {
                let v1289 = (v728.exp()) * (v2 + (v1283 - v728));
                v1290 = v1289;
            }
            let v1292 = v544 * (v1290 - v2);
            let v1293 = v743 / v469;
            let v1294 = if v1293 < v728 { 1.0 } else { 0.0 };
            let v1300: f64;
            if v1294 != 0.0 {
                let v1295 = v1293.exp();
                v1300 = v1295;
            } else {
                let v1299 = (v728.exp()) * (v2 + (v1293 - v728));
                v1300 = v1299;
            }
            let v1302 = v479 * (v1300 - v2);
            let v1303 = v1242 / v546;
            let v1304 = if v1303 < v728 { 1.0 } else { 0.0 };
            let v1310: f64;
            if v1304 != 0.0 {
                let v1305 = v1303.exp();
                v1310 = v1305;
            } else {
                let v1309 = (v728.exp()) * (v2 + (v1303 - v728));
                v1310 = v1309;
            }
            let v1312 = v553 * (v1310 - v2);
            let v1316 = if (if (if v575 > v0 { 1.0 } else { 0.0 }) != 0.0 && (if v566 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && v1133 != 0.0 { 1.0 } else { 0.0 };
            let v1895: f64;
            if v1316 != 0.0 {
                let v1320 = v574 * (v2 - (v29 / (v26 * v1013)));
                let v1321 = if v1320 < v728 { 1.0 } else { 0.0 };
                let v1382: f64;
                if v1321 != 0.0 {
                    let v1322 = v1320.exp();
                    v1382 = v1322;
                } else {
                    let v1326 = (v728.exp()) * (v2 + (v1320 - v728));
                    v1382 = v1326;
                }
                let v1327 = v691 * v307;
                let v1338 = v27 - v2;
                let v1353 = ((v691 * v29) * v574) / (v561 * ((((((v1327 * v1327) + v1329).sqrt()).powf((v1332 - v27))) * ((v27 * ((v2 - (v27 * v27)) - ((v153 * v1327) * v1338))) - (((v455 * v1327) * v1327) * (v1338 + v1327)))) * v1348));
                let v1355 = if v1353 < v1354 { 1.0 } else { 0.0 };
                let v1379: f64;
                if v1355 != 0.0 {
                    let v1356 = if v1353 < v728 { 1.0 } else { 0.0 };
                    let v1363: f64;
                    if v1356 != 0.0 {
                        let v1357 = v1353.exp();
                        v1363 = v1357;
                    } else {
                        let v1361 = (v728.exp()) * (v2 + (v1353 - v728));
                        v1363 = v1361;
                    }
                    let v1367 = (-v691) * (v2 + ((v2 - v1363) / v1353));
                    v1379 = v1367;
                } else {
                    let v1377 = ((v691 * v417) * v1353) * (v2 + ((v1353 * v1370) * (v2 + (v1372 * v1353))));
                    v1379 = v1377;
                }
                let v1385 = (((((v26 * v584) * v1379) * v1013) * v1382) * v307) * v30;
                v1895 = v1385;
            } else {
                v1895 = v0;
            }
            let v1390 = if (if (if v599 > v0 { 1.0 } else { 0.0 }) != 0.0 && (if v590 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if v685 < v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let v1643: f64;
            if v1390 != 0.0 {
                let v1391 = v685 * v309;
                let v1393 = (v2 - v1391).powf(v1054);
                let v1397 = v598 * (v2 - (v61 / (v26 * v1393)));
                let v1398 = if v1397 < v728 { 1.0 } else { 0.0 };
                let v1454: f64;
                if v1398 != 0.0 {
                    let v1399 = v1397.exp();
                    v1454 = v1399;
                } else {
                    let v1403 = (v728.exp()) * (v2 + (v1397 - v728));
                    v1454 = v1403;
                }
                let v1413 = v59 - v2;
                let v1427 = ((v685 * v61) * v598) / (v585 * ((((((v1391 * v1391) + v1329).sqrt()).powf((v1407 - v59))) * ((v59 * ((v2 - (v59 * v59)) - ((v153 * v1391) * v1413))) - (((v455 * v1391) * v1391) * (v1413 + v1391)))) * v1348));
                let v1429 = if v1427 < v1428 { 1.0 } else { 0.0 };
                let v1451: f64;
                if v1429 != 0.0 {
                    let v1430 = if v1427 < v728 { 1.0 } else { 0.0 };
                    let v1437: f64;
                    if v1430 != 0.0 {
                        let v1431 = v1427.exp();
                        v1437 = v1431;
                    } else {
                        let v1435 = (v728.exp()) * (v2 + (v1427 - v728));
                        v1437 = v1435;
                    }
                    let v1441 = (-v685) * (v2 + ((v2 - v1437) / v1427));
                    v1451 = v1441;
                } else {
                    let v1449 = ((v685 * v417) * v1427) * (v2 + ((v1427 * v1370) * (v2 + (v1372 * v1427))));
                    v1451 = v1449;
                }
                let v1457 = (((((v26 * v608) * v1451) * v1393) * v1454) * v309) * v62;
                v1643 = v1457;
            } else {
                v1643 = v0;
            }
            let v1460 = v1458 - v2;
            let v1463 = (v428 * v534) / v453;
            let v1468 = ((v26 * v534) * v1460) / (v2 + ((v2 + (v1463 * v1458)).sqrt()));
            let v1470 = if v1469 == v2 { 1.0 } else { 0.0 };
            let v1525: f64;
            let v1900: f64;
            if v1470 != 0.0 {
                let v1478 = v428 * (v626 / v631);
                let v1486 = (((v1471 * v26) * v626) * (v967 - v1474)) / (v2 + ((v2 + (v1478 * (v967 + (v1479 * v1474)))).sqrt()));
                let v1499 = ((((v2 - v1471) * v26) * v626) * (v1458 - v1490)) / (v2 + ((v2 + (v1478 * (v1458 + (v1479 * v1490)))).sqrt()));
                v1525 = v1499;
                v1900 = v1486;
            } else {
                let v1505 = v428 * (v626 / v631);
                let v1510 = (((v1471 * v26) * v626) * (v967 - v2)) / (v2 + ((v2 + (v1505 * v967)).sqrt()));
                let v1519 = ((((v2 - v1471) * v26) * v626) * v1460) / (v2 + ((v2 + (v1505 * v1458)).sqrt()));
                v1525 = v1519;
                v1900 = v1510;
            }
            let v1522 = if v8 > v0 { 1.0 } else { 0.0 };
            let v1523 = if (if v1520 > v0 { 1.0 } else { 0.0 }) != 0.0 && v1522 != 0.0 { 1.0 } else { 0.0 };
            let v1647: f64;
            let v1650: f64;
            let v1899: f64;
            let v1901: f64;
            let v1999: f64;
            if v1523 != 0.0 {
                let v1524 = v1468 * v9;
                let v1526 = v1525 * v9;
                let v1530 = v1529 - v2;
                let v1536 = (((v8 * v26) * v534) * v1530) / (v2 + ((v2 + (v1463 * v1529)).sqrt()));
                let v1587: f64;
                if v1470 != 0.0 {
                    let v1552 = (((((v2 - v1471) * v8) * v26) * v626) * (v1529 - v1541)) / (v2 + ((v2 + (((v428 * v626) / v631) * (v1529 + (v1479 * v1541)))).sqrt()));
                    v1587 = v1552;
                } else {
                    let v1564 = (((((v2 - v1471) * v8) * v26) * v626) * v1530) / (v2 + ((v2 + (((v428 * v626) / v631) * v1529)).sqrt()));
                    v1587 = v1564;
                }
                let v1565 = if v1520 == v2 { 1.0 } else { 0.0 };
                let v1593: f64;
                if v1565 != 0.0 {
                    let v1568 = (v8 * (v534 + v626)) * v350;
                    let v1573 = v724 - (v106 * (v26 - ((v1568 * v108).ln())));
                    let v1575 = v1573 * v1573;
                    let v1576 = if v1573 < v0 { 1.0 } else { 0.0 };
                    let v1586: f64;
                    if v1576 != 0.0 {
                        let v1581 = v1577 / (((v1575 + v1574).sqrt()) - v1573);
                        v1586 = v1581;
                    } else {
                        let v1585 = v417 * (((v1575 + v1574).sqrt()) + v1573);
                        v1586 = v1585;
                    }
                    let v1592 = v1586 / ((v1568 + ((v1536 + v1587) * v350)) + v1586);
                    v1593 = v1592;
                } else {
                    v1593 = v2;
                }
                let v1594 = v1593 * v1536;
                let v1595 = v1593 * v1587;
                v1647 = v1524;
                v1650 = v1594;
                v1899 = v1526;
                v1901 = v1595;
                v1999 = v1593;
            } else {
                v1647 = v1468;
                v1650 = v0;
                v1899 = v1525;
                v1901 = v0;
                v1999 = v2;
            }
            let v1597 = if v1596 == v2 { 1.0 } else { 0.0 };
            let v1644: f64;
            if v1597 != 0.0 {
                let v1598 = v696 + v685;
                let v1605 = ((v1601 * v1598) * v1603) * v1598;
                let v1608 = if (v1606 * v1598) < v0 { 1.0 } else { 0.0 };
                let v1634: f64;
                if v1608 != 0.0 {
                    let v1615 = v1609 / (((v1605 + v1600).sqrt()) - (v1612 * v1598));
                    v1634 = v1615;
                } else {
                    let v1621 = v417 * (((v1605 + v1600).sqrt()) + (v1618 * v1598));
                    v1634 = v1621;
                }
                let v1625 = v2 / (v2 - (v89.powf(v1622)));
                let v1627 = v89 * v1626;
                let v1633 = (((v1625 * v1625) * (v89.powf((v1622 - v2)))) * v1622) / v1626;
                let v1635 = if v1634 < v1627 { 1.0 } else { 0.0 };
                let v1645: f64;
                if v1635 != 0.0 {
                    let v1639 = v2 / (v2 - ((v1634 / v1626).powf(v1622)));
                    v1645 = v1639;
                } else {
                    let v1642 = v1625 + ((v1634 - v1627) * v1633);
                    v1645 = v1642;
                }
                v1644 = v1645;
            } else {
                v1644 = v2;
            }
            let v1646 = v1643 * v1644;
            let v1648 = v1647 * v1644;
            let v1649 = v1302 * v1644;
            let v1651 = v1650 * v1644;
            let v1655 = (v2 + (v1019 / v616)) + (v1069 / v613);
            let v1657 = v1655 * v1655;
            let v1658 = if v1655 < v0 { 1.0 } else { 0.0 };
            let v1668: f64;
            if v1658 != 0.0 {
                let v1663 = v1659 / (((v1657 + v1656).sqrt()) - v1655);
                v1668 = v1663;
            } else {
                let v1667 = v417 * (((v1657 + v1656).sqrt()) + v1655);
                v1668 = v1667;
            }
            let v1670 = v339 / (v1668 * v1123);
            let v1671 = if v1670 < v21 { 1.0 } else { 0.0 };
            let v1672: f64;
            if v1671 != 0.0 {
                v1672 = v21;
            } else {
                v1672 = v1670;
            }
            let v1673 = v153 * v1672;
            let v1675 = if v1130 > v0 { 1.0 } else { 0.0 };
            let v1882: f64;
            if v1675 != 0.0 {
                let v1677 = if v1676 == v2 { 1.0 } else { 0.0 };
                let v1839: f64;
                if v1677 != 0.0 {
                    let v1679 = if v685 < v1678 { 1.0 } else { 0.0 };
                    let v1840: f64;
                    if v1679 != 0.0 {
                        let v1682 = (-v1130) / v1681;
                        let v1683 = if v1682 < v728 { 1.0 } else { 0.0 };
                        let v1690: f64;
                        if v1683 != 0.0 {
                            let v1684 = v1682.exp();
                            v1690 = v1684;
                        } else {
                            let v1688 = (v728.exp()) * (v2 + (v1682 - v728));
                            v1690 = v1688;
                        }
                        let v1691 = (v1678 - v685) * v1690;
                        let v1696 = (-v1692) * (v1691.powf(v1694));
                        let v1697 = if v1696 < v728 { 1.0 } else { 0.0 };
                        let v1706: f64;
                        if v1697 != 0.0 {
                            let v1698 = v1696.exp();
                            v1706 = v1698;
                        } else {
                            let v1702 = (v728.exp()) * (v2 + (v1696 - v728));
                            v1706 = v1702;
                        }
                        let v1707 = ((v1703 / v1692) * v1691) * v1706;
                        v1840 = v1707;
                    } else {
                        v1840 = v0;
                    }
                    v1839 = v1840;
                } else {
                    let v1708 = if v1676 == v26 { 1.0 } else { 0.0 };
                    let v1841: f64;
                    if v1708 != 0.0 {
                        let v1709 = if v685 < v785 { 1.0 } else { 0.0 };
                        let v1842: f64;
                        if v1709 != 0.0 {
                            let v1714 = (v26 * v1710) / (v1712 * v1712);
                            let v1715 = v785 - v685;
                            let v1719 = ((v26 * (v1715 / v1051)) / v1714).sqrt();
                            let v1721 = if v1720 == v0 { 1.0 } else { 0.0 };
                            let v1728: f64;
                            if v1721 != 0.0 {
                                v1728 = v1712;
                            } else {
                                let v1725 = v2 - (v417 * v1722);
                                let v1727 = (v1712 * v1725) * v1725;
                                v1728 = v1727;
                            }
                            let v1734 = (v1719 * v1728) / (((v1719 * v1719) + (v1728 * v1728)).sqrt());
                            let v1735 = v1715 / v1734;
                            let v1736 = v417 * v1734;
                            let v1737 = v1736 * v1714;
                            let v1739 = v1735 + (v1737 * v1051);
                            let v1766: f64;
                            if v1721 != 0.0 {
                                v1766 = v1739;
                            } else {
                                let v1741 = v26 * v1740;
                                let v1753 = v1735 - (v1737 * (((v2 + v1740) / (v2 + v1741)) - (v1130 / (v866 * (v2 + (v1741 * (v2 + (v26 * v1722))))))));
                                let v1754 = v1753 - v1739;
                                let v1765 = v417 * ((v1753 + v1739) + (((v1754 * v1754) + ((((v41 * v1735) * v1735) * v1758) / v866)).sqrt()));
                                v1766 = v1765;
                            }
                            let v1768 = (v1766 - v1735) / v1766;
                            let v1771 = if (v1768.abs()) > v1770 { 1.0 } else { 0.0 };
                            let v1843: f64;
                            if v1771 != 0.0 {
                                let v1772 = v1736 / v1768;
                                let v1779 = (-v1774) / v1766;
                                let v1786 = (((v1773 / v1774) * v1766) * v1772) * ((v1779.exp()) - ((v1779 * (v2 + (v1728 / v1772))).exp()));
                                v1843 = v1786;
                            } else {
                                let v1791 = (v1773 * v1728) * (((-v1774) / v1766).exp());
                                v1843 = v1791;
                            }
                            v1842 = v1843;
                        } else {
                            v1842 = v0;
                        }
                        v1841 = v1842;
                    } else {
                        let v1792 = if v1676 == v153 { 1.0 } else { 0.0 };
                        let v1844: f64;
                        if v1792 != 0.0 {
                            let v1793 = if v685 < v1678 { 1.0 } else { 0.0 };
                            let v1845: f64;
                            if v1793 != 0.0 {
                                let v1794 = v1678 - v685;
                                let v1802 = (v1794.powf(v1694)) * ((v2 - (v1130 / (v1796 + v1130))).powf(v1800));
                                let v1803 = if v1720 == v0 { 1.0 } else { 0.0 };
                                let v1827: f64;
                                if v1803 != 0.0 {
                                    v1827 = v1802;
                                } else {
                                    let v1806 = (v1130 - v1804) / v1796;
                                    let v1809 = (v1806 - v2) / v1808;
                                    let v1810 = if v1806 < v2 { 1.0 } else { 0.0 };
                                    let v1822: f64;
                                    if v1810 != 0.0 {
                                        let v1815 = v2 + (v1808 * ((v2 + (v1809.exp())).ln()));
                                        v1822 = v1815;
                                    } else {
                                        let v1821 = v1806 + (v1808 * ((v2 + ((-v1809).exp())).ln()));
                                        v1822 = v1821;
                                    }
                                    let v1825 = v1802 * (v1822.powf(v1823));
                                    v1827 = v1825;
                                }
                                let v1828 = (-v1692) * v1827;
                                let v1829 = if v1828 < v728 { 1.0 } else { 0.0 };
                                let v1837: f64;
                                if v1829 != 0.0 {
                                    let v1830 = v1828.exp();
                                    v1837 = v1830;
                                } else {
                                    let v1834 = (v728.exp()) * (v2 + (v1828 - v728));
                                    v1837 = v1834;
                                }
                                let v1838 = ((v1703 / v1692) * v1794) * v1837;
                                v1845 = v1838;
                            } else {
                                v1845 = v0;
                            }
                            v1844 = v1845;
                        } else {
                            v1844 = v0;
                        }
                        v1841 = v1844;
                    }
                    v1839 = v1841;
                }
                let v1846 = if v1839 > v0 { 1.0 } else { 0.0 };
                let v1883: f64;
                if v1846 != 0.0 {
                    let v1848 = if v1847 == v2 { 1.0 } else { 0.0 };
                    let v1884: f64;
                    if v1848 != 0.0 {
                        let v1850 = v1849 + v1673;
                        let v1858 = ((v106 / (v1130 * v1850)) + ((v1124 / v442) * v493)) + (v1856 / v1850);
                        let v1859 = if v1676 == v153 { 1.0 } else { 0.0 };
                        let v1885: f64;
                        if v1859 != 0.0 {
                            let v1861 = (v1839 - v1858) / v1599;
                            let v1862 = if v1839 < v1858 { 1.0 } else { 0.0 };
                            let v1874: f64;
                            if v1862 != 0.0 {
                                let v1867 = v1839 - (v1599 * ((v2 + (v1861.exp())).ln()));
                                v1874 = v1867;
                            } else {
                                let v1873 = v1858 - (v1599 * ((v2 + ((-v1861).exp())).ln()));
                                v1874 = v1873;
                            }
                            let v1875 = v1130 * v1874;
                            v1885 = v1875;
                        } else {
                            let v1879 = ((v1130 * v1839) * v1858) / (v1839 + v1858);
                            v1885 = v1879;
                        }
                        v1884 = v1885;
                    } else {
                        let v1880 = v1130 * v1839;
                        v1884 = v1880;
                    }
                    v1883 = v1884;
                } else {
                    v1883 = v0;
                }
                v1882 = v1883;
            } else {
                v1882 = v0;
            }
            let v1881 = if v1078 > v0 { 1.0 } else { 0.0 };
            if v1881 != 0.0 {
            } else {
            }
            if v503 != 0.0 {
            } else {
            }
            let v1894 = v1892 + v1282;
            let v1898 = (v1896 + v1292) + v1312;
            let v1902 = if v694 < v993 { 1.0 } else { 0.0 };
            if v1902 != 0.0 {
            } else {
            }
            let v1904 = v637 * v447;
            let v1905 = v41 * v318;
            let v1906 = if v719 < v1032 { 1.0 } else { 0.0 };
            if v1906 != 0.0 {
            } else {
            }
            let v1907 = v2 - v1903;
            let v1909 = (v724 - v1032) / v1905;
            let v1910 = if v724 < v1032 { 1.0 } else { 0.0 };
            let v1922: f64;
            if v1910 != 0.0 {
                let v1915 = v724 - (v1905 * ((v2 + (v1909.exp())).ln()));
                v1922 = v1915;
            } else {
                let v1921 = v1032 - (v1905 * ((v2 + ((-v1909).exp())).ln()));
                v1922 = v1921;
            }
            let v1936 = ((v325 * ((v1026 * ((v1055 * (v2 - ((v2 - (v1922 / v318)).powf(v1054)))) + (v1027 * (v724 - v1922)))) + (v326 * v724))) * v1907) * v8;
            let v1942 = if v699 < (v314 * (v2 - (v26.powf((v1937 / v315))))) { 1.0 } else { 0.0 };
            if v1942 != 0.0 {
            } else {
            }
            let v1946 = if (v691 / (v1943 * v106)) < v728 { 1.0 } else { 0.0 };
            if v1946 != 0.0 {
            } else {
            }
            let v1949 = ((v428 * v642) * v106) / v362;
            let v1951 = if v1950 == v0 { 1.0 } else { 0.0 };
            if v1951 != 0.0 {
            } else {
                let v1957 = if (((v719 - v1952) / v1954) * v108) < v728 { 1.0 } else { 0.0 };
                if v1957 != 0.0 {
                } else {
                }
            }
            let v1961 = if (if (if v1520 == v2 { 1.0 } else { 0.0 }) != 0.0 || (if v1520 == v153 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && v1522 != 0.0 { 1.0 } else { 0.0 };
            let v2028: f64;
            if v1961 != 0.0 {
                let v2000: f64;
                if v1951 != 0.0 {
                    let v1962 = v1071 * v1529;
                    let v1969 = v428 * v1968;
                    let v1980 = (((v417 * v8) * v647) * ((v1904 * ((v1962 - v1071) / (v2 + ((v2 + v1962).sqrt())))) + (v1949 * (v1969 / (v2 + ((v2 + v1969).sqrt())))))) / v644;
                    v2000 = v1980;
                } else {
                    let v1982 = (v724 - v1952) * v108;
                    let v1983 = if v1982 < v728 { 1.0 } else { 0.0 };
                    let v1993: f64;
                    if v1983 != 0.0 {
                        let v1984 = v1982.exp();
                        v1993 = v1984;
                    } else {
                        let v1988 = (v728.exp()) * (v2 + (v1982 - v728));
                        v1993 = v1988;
                    }
                    let v1998 = ((((v26 * v8) * v534) * v653) * v1529) / (v2 + ((v2 + (v428 * v1993)).sqrt()));
                    v2000 = v1998;
                }
                let v2001 = v1999 * v2000;
                v2028 = v2001;
            } else {
                v2028 = v0;
            }
            let v2003 = if v2002 == v2 { 1.0 } else { 0.0 };
            if v2003 != 0.0 {
                let v2004 = if v996 < v0 { 1.0 } else { 0.0 };
                if v2004 != 0.0 {
                } else {
                }
            } else {
            }
            if v503 != 0.0 {
            } else {
            }
            let v2006 = (v1 * v1901) * v20;
            let v2009 = ((v1 * v705) / v1849) * v20;
            let v2010 = v2 - v672;
            let v2011 = if v671 > v21 { 1.0 } else { 0.0 };
            if v2011 != 0.0 {
                let v2013 = if v2012 == v0 { 1.0 } else { 0.0 };
                if v2013 != 0.0 {
                } else {
                    let v2015 = if (v2010.abs()) < v1599 { 1.0 } else { 0.0 };
                    if v2015 != 0.0 {
                    } else {
                    }
                }
            } else {
            }
            let v2018 = (v1 * v2016) * v707;
            let v2020 = (0e0f64) * v20;
            let v2023 = (v1 * v2021) * v710;
            let v2025 = (0e0f64) * v20;
            let v2027 = (v1 * v1651) * v20;
            let v2030 = v1 * (v1936 + v2028);
            let v2032 = (0e0f64) * v20;
            if v676 != 0.0 {
            } else {
            }
            if v679 != 0.0 {
            } else {
            }
            let v2034 = v2033 * v103;
            let v2035 = v2034 / v1856;
            let v2036 = v2034 / v1849;
            let v2037 = v2034 * v1886;
            let v2038 = v2034 * v1888;
            let v2039 = v2034 * v1890;
            let v2045 = ((v2034 / v1673) * ((v428 * v1674) + v2042)) * v1370;
            let v2047 = (v1128 + v1127) / v1124;
            let v2050 = v2048 * (v2047.abs());
            let v2052 = if v2051 > v0 { 1.0 } else { 0.0 };
            let v2057: f64;
            if v2052 != 0.0 {
                let v2054 = (v1882 / v2047).abs();
                v2057 = v2054;
            } else {
                v2057 = v0;
            }
            let v2059 = (v2055 * v1882) * (v2057 + v2);
            let v2060 = if v2047 > v0 { 1.0 } else { 0.0 };
            if v2060 != 0.0 {
            } else {
            }
            let v2062 = if v2061 == v2 { 1.0 } else { 0.0 };
            if v2062 != 0.0 {
            } else {
                let v2063 = if v2061 == v26 { 1.0 } else { 0.0 };
                if v2063 != 0.0 {
                } else {
                }
            }
            let v2069 = v2064 * ((((v1894 - v1895) + v1176) + v1155).abs());
            let v2070 = v1892 + v1896;
            let v2075 = v2071 * ((v2070.abs()).powf(v2073));
            let v2076 = if v2070 < v0 { 1.0 } else { 0.0 };
            let v2139: f64;
            if v2076 != 0.0 {
                let v2077 = -v2075;
                v2139 = v2077;
            } else {
                v2139 = v2075;
            }
            let v2079 = (v1282 + v1292) + v1312;
            let v2084 = v2080 * ((v2079.abs()).powf(v2082));
            let v2085 = if v2079 < v0 { 1.0 } else { 0.0 };
            let v2141: f64;
            if v2085 != 0.0 {
                let v2086 = -v2084;
                v2141 = v2086;
            } else {
                v2141 = v2084;
            }
            let v2089 = v2087 * (v1898.abs());
            let v2091 = v1649.abs();
            let v2092 = v2090 * v2091;
            let v2094 = v2071 * (v2091.powf(v2073));
            let v2095 = if v1649 < v0 { 1.0 } else { 0.0 };
            let v2145: f64;
            if v2095 != 0.0 {
                let v2096 = -v2094;
                v2145 = v2096;
            } else {
                v2145 = v2094;
            }
            let v2099 = v2097 * (v1646.abs());
            let v2101 = v1648.abs();
            let v2102 = v2100 * v2101;
            let v2104 = v2 - (v1520 * v8);
            let v2108 = (v2071 * v2104) * ((v2101 / v2104).powf(v2073));
            let v2109 = if v1648 < v0 { 1.0 } else { 0.0 };
            let v2148: f64;
            if v2109 != 0.0 {
                let v2110 = -v2108;
                v2148 = v2110;
            } else {
                v2148 = v2108;
            }
            let v2112 = v1651.abs();
            let v2114 = (v2111 * v2112) * v1520;
            let v2115 = if v8 == v0 { 1.0 } else { 0.0 };
            let v2122: f64;
            if v2115 != 0.0 {
                v2122 = v0;
            } else {
                let v2120 = ((v2071 * v1520) * v8) * ((v2112 / v8).powf(v2073));
                v2122 = v2120;
            }
            let v2121 = if v1651 < v0 { 1.0 } else { 0.0 };
            let v2151: f64;
            if v2121 != 0.0 {
                let v2123 = -v2122;
                v2151 = v2123;
            } else {
                v2151 = v2122;
            }
            let v2126 = v2124 * (v1900.abs());
            let v2129 = v2127 * (v1899.abs());
            let v2132 = v2130 * (v1901.abs());
            let v2133 = v2050 * v20;
            let v2134 = v2059 * v20;
            let v2135 = v2069 * v20;
            let v2136 = v2035 * v20;
            let v2137 = v2036 * v20;
            let v2138 = v2045 * v20;
            let v2140 = v2139 * v20;
            let v2142 = v2141 * v20;
            let v2143 = v2089 * v20;
            let v2144 = v2092 * v20;
            let v2146 = v2145 * v20;
            let v2147 = v2102 * v20;
            let v2149 = v2148 * v20;
            let v2150 = v2114 * v20;
            let v2152 = v2151 * v20;
            let v2173: f64;
            let v2174: f64;
            let v2175: f64;
            let v2176: f64;
            if v503 != 0.0 {
                let v2153 = v2099 * v20;
                v2173 = v2;
                v2174 = v2153;
                v2175 = v0;
                v2176 = v0;
            } else {
                let v2154 = v2099 * v20;
                v2173 = v0;
                v2174 = v0;
                v2175 = v2;
                v2176 = v2154;
            }
            let v2155 = v2126 * v20;
            let v2156 = v2129 * v20;
            let v2157 = v2132 * v20;
            let v2177: f64;
            let v2179: f64;
            let v2181: f64;
            let v2183: f64;
            let v2185: f64;
            let v2187: f64;
            let v2189: f64;
            let v2191: f64;
            let v2193: f64;
            let v2195: f64;
            let v2197: f64;
            let v2199: f64;
            let v2201: f64;
            let v2203: f64;
            let v2205: f64;
            let v2207: f64;
            if v676 != 0.0 {
                let v2178: f64;
                let v2180: f64;
                let v2182: f64;
                let v2184: f64;
                let v2186: f64;
                let v2188: f64;
                let v2190: f64;
                let v2192: f64;
                let v2194: f64;
                let v2196: f64;
                if v679 != 0.0 {
                    let v2158 = v2037 * v20;
                    let v2159 = v2038 * v20;
                    let v2160 = v2039 * v20;
                    v2178 = v2;
                    v2180 = v2158;
                    v2182 = v2;
                    v2184 = v2159;
                    v2186 = v2;
                    v2188 = v2160;
                    v2190 = v0;
                    v2192 = v0;
                    v2194 = v0;
                    v2196 = v0;
                } else {
                    let v2161 = v2037 * v20;
                    let v2162 = v2038 * v20;
                    v2178 = v0;
                    v2180 = v0;
                    v2182 = v0;
                    v2184 = v0;
                    v2186 = v0;
                    v2188 = v0;
                    v2190 = v2;
                    v2192 = v2161;
                    v2194 = v2;
                    v2196 = v2162;
                }
                v2177 = v2178;
                v2179 = v2180;
                v2181 = v2182;
                v2183 = v2184;
                v2185 = v2186;
                v2187 = v2188;
                v2189 = v2190;
                v2191 = v2192;
                v2193 = v2194;
                v2195 = v2196;
                v2197 = v0;
                v2199 = v0;
                v2201 = v0;
                v2203 = v0;
                v2205 = v0;
                v2207 = v0;
            } else {
                let v2198: f64;
                let v2200: f64;
                let v2202: f64;
                let v2204: f64;
                let v2206: f64;
                let v2208: f64;
                if v679 != 0.0 {
                    let v2163 = v2037 * v20;
                    let v2164 = v2039 * v20;
                    v2198 = v2;
                    v2200 = v2163;
                    v2202 = v2;
                    v2204 = v2164;
                    v2206 = v0;
                    v2208 = v0;
                } else {
                    let v2165 = v2037 * v20;
                    v2198 = v0;
                    v2200 = v0;
                    v2202 = v0;
                    v2204 = v0;
                    v2206 = v2;
                    v2208 = v2165;
                }
                v2177 = v0;
                v2179 = v0;
                v2181 = v0;
                v2183 = v0;
                v2185 = v0;
                v2187 = v0;
                v2189 = v0;
                v2191 = v0;
                v2193 = v0;
                v2195 = v0;
                v2197 = v2198;
                v2199 = v2200;
                v2201 = v2202;
                v2203 = v2204;
                v2205 = v2206;
                v2207 = v2208;
            }
            let v2171 = if (((((v2006 + v2009) + v2020) + v2025) + v2027) + v2032) == v0 { 1.0 } else { 0.0 };
            if v2171 != 0.0 {
            } else {
            }
            let v2172 = if v20 != v2 { 1.0 } else { 0.0 };
            if v2172 != 0.0 {
            } else {
            }
        {
            let psd = v2133;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 0, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 0, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 0, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(0, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = v2134;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 1, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(1, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = v2135;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 2, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 2, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 2, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(2, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = v2136;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 3, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 3, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 3, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(3, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = v2137;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 4, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 4, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 4, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(4, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = v2138;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 5, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 5, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 5, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(5, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = v2140;
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
            let psd = v2142;
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
            let psd = v2143;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 8, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 8, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 8, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(8, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = v2144;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 9, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 9, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 9, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(9, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = v2146;
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
            let psd = v2147;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 11, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 11, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 11, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(11, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = v2149;
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
            let psd = v2150;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 13, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 13, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 13, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(13, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = v2152;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 14, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 14, value: psd }); }
            let exponent: Option<f64> = Some(v2);
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 14, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 14, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(14, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v2173 == 0.0 {
            if !visitor.visit(15, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v2174;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 15, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 15, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 15, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(15, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v2175 == 0.0 {
            if !visitor.visit(16, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v2176;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 16, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 16, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 16, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(16, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = v2155;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 17, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 17, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 17, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(17, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = v2156;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 18, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 18, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 18, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(18, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = v2157;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 19, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 19, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 19, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(19, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v2177 == 0.0 {
            if !visitor.visit(20, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v2179;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 20, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 20, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 20, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(20, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v2181 == 0.0 {
            if !visitor.visit(21, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v2183;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 21, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 21, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 21, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(21, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v2185 == 0.0 {
            if !visitor.visit(22, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v2187;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 22, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 22, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 22, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(22, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v2189 == 0.0 {
            if !visitor.visit(23, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v2191;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 23, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 23, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 23, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(23, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v2193 == 0.0 {
            if !visitor.visit(24, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v2195;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 24, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 24, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 24, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(24, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v2197 == 0.0 {
            if !visitor.visit(25, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v2199;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 25, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 25, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 25, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(25, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v2201 == 0.0 {
            if !visitor.visit(26, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v2203;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 26, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 26, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 26, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(26, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v2205 == 0.0 {
            if !visitor.visit(27, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v2207;
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
