#![allow(dead_code, non_snake_case, unused_parens, unused_variables)]

use super::state::Instance;
use crate::device::veriloga_generated::GeneratedEvalContext;
pub use crate::device::veriloga_generated::{GeneratedNoiseDescriptor, GeneratedNoiseEndpoint, GeneratedNoiseEvaluation, GeneratedNoiseEvaluationError, GeneratedNoiseEvaluationRef, GeneratedNoiseKind, GeneratedNoiseVisitor};

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
            let v0 = 0e0f64;
            let v1 = parameters[3];
            let v2 = 1e0f64;
            let v4 = 7.03e7f64;
            let v5 = 1.23e8f64;
            let v6 = 1.58e8f64;
            let v7 = 2.04e8f64;
            let v8 = parameters[32];
            let v10 = parameters[4];
            let v11 = 2.7315e2f64;
            let v13 = temperature;
            let v14 = parameters[0];
            let v16 = parameters[137];
            let v18 = 1e-12f64;
            let v20 = parameters[1];
            let v23 = 1e-3f64;
            let v24 = 2e0f64;
            let v25 = parameters[66];
            let v29 = parameters[113];
            let v30 = parameters[114];
            let v33 = parameters[115];
            let v37 = 5e-2f64;
            let v39 = 1e-1f64;
            let v54 = parameters[65];
            let v56 = parameters[70];
            let v57 = parameters[71];
            let v61 = parameters[116];
            let v62 = parameters[117];
            let v65 = parameters[118];
            let v85 = parameters[82];
            let v89 = 8.617086918058125e-5f64;
            let v137 = 3e0f64;
            let v138 = -3e0f64;
            let v144 = parameters[104];
            let v161 = -3e0f64;
            let v164 = parameters[63];
            let v167 = parameters[109];
            let v184 = -3e0f64;
            let v187 = parameters[79];
            let v205 = -3e0f64;
            let v225 = -3e0f64;
            let v244 = -3e0f64;
            let v247 = parameters[26];
            let v250 = parameters[108];
            let v275 = parameters[74];
            let v283 = parameters[69];
            let v286 = parameters[53];
            let v287 = parameters[96];
            let v292 = parameters[55];
            let v293 = parameters[97];
            let v294 = parameters[95];
            let v299 = parameters[54];
            let v300 = parameters[100];
            let v305 = parameters[56];
            let v306 = parameters[101];
            let v310 = parameters[57];
            let v311 = parameters[103];
            let v315 = parameters[58];
            let v317 = parameters[59];
            let v318 = parameters[98];
            let v322 = parameters[121];
            let v324 = parameters[9];
            let v343 = 6.931471805599453e-4f64;
            let v345 = parameters[122];
            let v347 = parameters[10];
            let v366 = 6.931471805599453e-4f64;
            let v368 = parameters[42];
            let v369 = parameters[123];
            let v373 = 1e-6f64;
            let v376 = 5e-1f64;
            let v377 = 5e-7f64;
            let v386 = parameters[8];
            let v387 = 4e0f64;
            let v390 = parameters[120];
            let v402 = parameters[11];
            let v407 = parameters[29];
            let v408 = parameters[102];
            let v413 = parameters[19];
            let v414 = 6e0f64;
            let v415 = parameters[20];
            let v421 = parameters[112];
            let v427 = parameters[30];
            let v428 = parameters[31];
            let v439 = parameters[15];
            let v443 = parameters[16];
            let v447 = parameters[110];
            let v453 = parameters[17];
            let v454 = parameters[18];
            let v461 = parameters[23];
            let v463 = parameters[24];
            let v464 = parameters[106];
            let v470 = parameters[27];
            let v471 = parameters[105];
            let v476 = parameters[25];
            let v477 = parameters[107];
            let v483 = parameters[28];
            let v489 = parameters[111];
            let v494 = parameters[21];
            let v495 = parameters[22];
            let v504 = parameters[132];
            let v505 = parameters[133];
            let v513 = parameters[138];
            let v516 = parameters[140];
            let v522 = -5e-1f64;
            let v525 = parameters[34];
            let v534 = parameters[33];
            let v546 = -5e-1f64;
            let v549 = parameters[36];
            let v558 = parameters[35];
            let v570 = parameters[13];
            let v573 = parameters[12];
            let v576 = parameters[86];
            let v582 = parameters[87];
            let v587 = parameters[88];
            let v592 = parameters[89];
            let v593 = parameters[99];
            let v598 = 3e2f64;
            let v600 = 5.25e2f64;
            let v603 = 7.2e-4f64;
            let v606 = 1.6e-6f64;
            let v611 = 1.081e0f64;
            let v613 = parameters[91];
            let v624 = node_potentials[5];
            let v625 = node_potentials[6];
            let v628 = node_potentials[7];
            let v631 = node_potentials[3];
            let v634 = node_potentials[4];
            let v641 = node_potentials[2];
            let v642 = node_potentials[1];
            let v647 = node_potentials[0];
            let v650 = node_potentials[9];
            let v653 = node_potentials[8];
            let v665 = parameters[134];
            let v740 = parameters[136];
            let v751 = 1e2f64;
            let v767 = 2e-1f64;
            let v782 = parameters[61];
            let v783 = parameters[60];
            let v793 = parameters[62];
            let v808 = -1e0f64;
            let v851 = parameters[135];
            let v869 = parameters[72];
            let v885 = 1e-5f64;
            let v889 = 1e-40f64;
            let v905 = -1e0f64;
            let v936 = parameters[73];
            let v944 = -1e0f64;
            let v968 = parameters[75];
            let v1023 = 1.0000000000000002e-2f64;
            let v1027 = 5.000000000000001e-3f64;
            let v1041 = parameters[14];
            let v1047 = 1e-4f64;
            let v1061 = parameters[139];
            let v1072 = parameters[141];
            let v1087 = parameters[142];
            let v1110 = 1e3f64;
            let v1112 = 4e1f64;
            let v1115 = 2.3538526683702e17f64;
            let v1143 = parameters[92];
            let v1245 = 1e-30f64;
            let v1248 = -2e0f64;
            let v1264 = 1.6666666666666666e-1f64;
            let v1270 = -1e-3f64;
            let v1286 = 3.333333333333333e-1f64;
            let v1288 = 2.5e-1f64;
            let v1323 = -2e0f64;
            let v1344 = -1e-3f64;
            let v1385 = parameters[5];
            let v1408 = 1.21e-2f64;
            let v1411 = 6.05e-3f64;
            let v1427 = parameters[83];
            let v1430 = 1e-6f64;
            let v1431 = 1e-12f64;
            let v1432 = -1e0f64;
            let v1434 = -1e0f64;
            let v1437 = -1e0f64;
            let v1440 = 5e-13f64;
            let v1443 = -1e0f64;
            let v1449 = -1e0f64;
            let v1453 = parameters[81];
            let v1457 = parameters[80];
            let v1487 = 1.0000000000000002e-2f64;
            let v1490 = 5.000000000000001e-3f64;
            let v1507 = parameters[38];
            let v1509 = parameters[43];
            let v1512 = parameters[41];
            let v1525 = parameters[40];
            let v1534 = parameters[39];
            let v1541 = parameters[45];
            let v1543 = parameters[44];
            let v1551 = parameters[7];
            let v1571 = parameters[46];
            let v1601 = 1e-7f64;
            let v1627 = parameters[47];
            let v1631 = parameters[48];
            let v1635 = parameters[51];
            let v1639 = parameters[50];
            let v1654 = parameters[49];
            let v1678 = parameters[52];
            let v1714 = parameters[76];
            let v1748 = parameters[84];
            let v1755 = parameters[78];
            let v1759 = parameters[90];
            let v1807 = parameters[6];
            let v1824 = parameters[68];
            let v1829 = parameters[77];
            let v1847 = 5.5224904e-23f64;
            let v1856 = 5e0f64;
            let v1862 = 3.2043836e-19f64;
            let v1865 = parameters[129];
            let v1869 = 3.2043836e-19f64;
            let v1875 = parameters[130];
            let v1878 = 3.2043836e-19f64;
            let v1885 = parameters[127];
            let v1887 = parameters[125];
            let v1894 = parameters[128];
            let v1896 = parameters[126];
            let v1901 = 3.2043836e-19f64;
            let v1904 = 3.2043836e-19f64;
            let v1911 = 3.2043836e-19f64;
            let v1914 = 3.2043836e-19f64;
            let v1925 = 3.2043836e-19f64;
            let v3 = if v1 == v2 { 1.0 } else { 0.0 };
            let v602: f64;
            let v1604: f64;
            if v3 != 0.0 {
                v602 = v5;
                v1604 = v4;
            } else {
                v602 = v7;
                v1604 = v6;
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
            let v27 = v24.powf((v24 - v25));
            let v28 = v2 / v27;
            let v36 = v29 + (((v30 * v12) * v12) / (v12 + v33));
            let v40 = (v36 - v37) / v39;
            let v41 = if v36 < v37 { 1.0 } else { 0.0 };
            let v97: f64;
            if v41 != 0.0 {
                let v46 = v37 + (v39 * ((v2 + (v40.exp())).ln()));
                v97 = v46;
            } else {
                let v52 = v36 + (v39 * ((v2 + ((-v40).exp())).ln()));
                v97 = v52;
            }
            let v53 = v2 / v29;
            let v55 = v2 / v54;
            let v59 = v24.powf((v24 - v57));
            let v60 = v2 / v59;
            let v68 = v61 + (((v62 * v12) * v12) / (v12 + v65));
            let v70 = (v68 - v37) / v39;
            let v71 = if v68 < v37 { 1.0 } else { 0.0 };
            let v117: f64;
            if v71 != 0.0 {
                let v76 = v37 + (v39 * ((v2 + (v70.exp())).ln()));
                v117 = v76;
            } else {
                let v82 = v68 + (v39 * ((v2 + ((-v70).exp())).ln()));
                v117 = v82;
            }
            let v83 = v2 / v61;
            let v84 = v2 / v56;
            let v87 = v2 - (v2 / v85);
            let v88 = v15 / v12;
            let v90 = v89 * v15;
            let v92 = v2 / v90;
            let v94 = v92 - (v2 / (v89 * v12));
            let v95 = v15 - v12;
            let v96 = v88.ln();
            let v102 = v97 - (((v30 * v15) * v15) / (v15 + v33));
            let v104 = (v102 - v37) / v39;
            let v105 = if v102 < v37 { 1.0 } else { 0.0 };
            let v520: f64;
            if v105 != 0.0 {
                let v110 = v37 + (v39 * ((v2 + (v104.exp())).ln()));
                v520 = v110;
            } else {
                let v116 = v102 + (v39 * ((v2 + ((-v104).exp())).ln()));
                v520 = v116;
            }
            let v122 = v117 - (((v62 * v15) * v15) / (v15 + v65));
            let v124 = (v122 - v37) / v39;
            let v125 = if v122 < v37 { 1.0 } else { 0.0 };
            let v544: f64;
            if v125 != 0.0 {
                let v130 = v37 + (v39 * ((v2 + (v124.exp())).ln()));
                v544 = v130;
            } else {
                let v136 = v122 + (v39 * ((v2 + ((-v124).exp())).ln()));
                v544 = v136;
            }
            let v143 = v2 - v88;
            let v146 = (((v138 * v90) * v96) + (v54 * v88)) + (v143 * v144);
            let v148 = (v37 - v146) / v90;
            let v149 = if v37 < v146 { 1.0 } else { 0.0 };
            let v267: f64;
            if v149 != 0.0 {
                let v154 = v146 + (v90 * ((v2 + (v148.exp())).ln()));
                v267 = v154;
            } else {
                let v160 = v37 + (v90 * ((v2 + ((-v148).exp())).ln()));
                v267 = v160;
            }
            let v168 = v143 * v167;
            let v169 = (((v161 * v90) * v96) + (v164 * v88)) + v168;
            let v171 = (v37 - v169) / v90;
            let v172 = if v37 < v169 { 1.0 } else { 0.0 };
            let v701: f64;
            if v172 != 0.0 {
                let v177 = v169 + (v90 * ((v2 + (v171.exp())).ln()));
                v701 = v177;
            } else {
                let v183 = v37 + (v90 * ((v2 + ((-v171).exp())).ln()));
                v701 = v183;
            }
            let v190 = (((v184 * v90) * v96) + (v187 * v88)) + v168;
            let v192 = (v37 - v190) / v90;
            let v193 = if v37 < v190 { 1.0 } else { 0.0 };
            let v1757: f64;
            if v193 != 0.0 {
                let v198 = v190 + (v90 * ((v2 + (v192.exp())).ln()));
                v1757 = v198;
            } else {
                let v204 = v37 + (v90 * ((v2 + ((-v192).exp())).ln()));
                v1757 = v204;
            }
            let v208 = v56 * v88;
            let v210 = (((v205 * v90) * v96) + v208) + v168;
            let v212 = (v37 - v210) / v90;
            let v213 = if v37 < v210 { 1.0 } else { 0.0 };
            let v277: f64;
            if v213 != 0.0 {
                let v218 = v210 + (v90 * ((v2 + (v212.exp())).ln()));
                v277 = v218;
            } else {
                let v224 = v37 + (v90 * ((v2 + ((-v212).exp())).ln()));
                v277 = v224;
            }
            let v229 = (((v225 * v90) * v96) + v208) + v168;
            let v231 = (v37 - v229) / v90;
            let v232 = if v37 < v229 { 1.0 } else { 0.0 };
            let v269: f64;
            if v232 != 0.0 {
                let v237 = v229 + (v90 * ((v2 + (v231.exp())).ln()));
                v269 = v237;
            } else {
                let v243 = v37 + (v90 * ((v2 + ((-v231).exp())).ln()));
                v269 = v243;
            }
            let v252 = (((v244 * v90) * v96) + (v247 * v88)) + (v143 * v250);
            let v254 = (v37 - v252) / v90;
            let v255 = if v37 < v252 { 1.0 } else { 0.0 };
            let v1100: f64;
            if v255 != 0.0 {
                let v260 = v252 + (v90 * ((v2 + (v254.exp())).ln()));
                v1100 = v260;
            } else {
                let v266 = v37 + (v90 * ((v2 + ((-v254).exp())).ln()));
                v1100 = v266;
            }
            let v268 = v2 / v267;
            let v270 = v2 / v269;
            let v272 = (v54 * v268).powf(v25);
            let v274 = (v56 * v270).powf(v57);
            let v281 = ((v2 - v275) * ((v56 / v277).powf(v57))) + v275;
            let v282 = v2 / v281;
            let v284 = v283 * v281;
            let v285 = v275 * v282;
            let v290 = v286 * ((v96 * v287).exp());
            let v291 = if v290 < v21 { 1.0 } else { 0.0 };
            let v1687: f64;
            if v291 != 0.0 {
                v1687 = v21;
            } else {
                v1687 = v290;
            }
            let v298 = v292 * ((v96 * (v293 - v294)).exp());
            let v303 = v299 * ((v96 * v300).exp());
            let v304 = if v303 < v21 { 1.0 } else { 0.0 };
            let v1680: f64;
            if v304 != 0.0 {
                v1680 = v21;
            } else {
                v1680 = v303;
            }
            let v309 = v305 * ((v96 * v306).exp());
            let v313 = (v96 * v311).exp();
            let v314 = v310 * v313;
            let v316 = v315 * v313;
            let v321 = v317 * ((v96 * v318).exp());
            let v323 = if v322 != v0 { 1.0 } else { 0.0 };
            let v393: f64;
            if v323 != 0.0 {
                let v327 = v324 * (v2 + (v95 * v322));
                let v329 = (v327 - v2) / v23;
                let v330 = if v327 < v2 { 1.0 } else { 0.0 };
                let v342: f64;
                if v330 != 0.0 {
                    let v335 = v2 + (v23 * ((v2 + (v329.exp())).ln()));
                    v342 = v335;
                } else {
                    let v341 = v327 + (v23 * ((v2 + ((-v329).exp())).ln()));
                    v342 = v341;
                }
                let v344 = v342 - v343;
                v393 = v344;
            } else {
                v393 = v324;
            }
            let v346 = if v345 != v0 { 1.0 } else { 0.0 };
            let v995: f64;
            if v346 != 0.0 {
                let v350 = v347 * (v2 + (v95 * v345));
                let v352 = (v350 - v2) / v23;
                let v353 = if v350 < v2 { 1.0 } else { 0.0 };
                let v365: f64;
                if v353 != 0.0 {
                    let v358 = v2 + (v23 * ((v2 + (v352.exp())).ln()));
                    v365 = v358;
                } else {
                    let v364 = v350 + (v23 * ((v2 + ((-v352).exp())).ln()));
                    v365 = v364;
                }
                let v367 = v365 - v366;
                v995 = v367;
            } else {
                v995 = v347;
            }
            let v372 = v368 * (v2 + (v369 * v95));
            let v374 = v372 * v372;
            let v375 = if v372 < v0 { 1.0 } else { 0.0 };
            let v1523: f64;
            if v375 != 0.0 {
                let v381 = v377 / (((v374 + v373).sqrt()) - v372);
                v1523 = v381;
            } else {
                let v385 = v376 * (((v374 + v373).sqrt()) + v372);
                v1523 = v385;
            }
            let v401 = (v386 * (((v96 * (((v387 - v293) - v294) + v390)) / v393).exp())) * ((((-v144) * v94) / v393).exp());
            let v406 = v402 * ((v96 * (v2 - v293)).exp());
            let v412 = v407 * ((v96 * (v2 - v408)).exp());
            let v423 = (-v421) * v94;
            let v426 = (v413 * ((v96 * (v414 - (v24 * v415))).exp())) * ((v423 / v415).exp());
            let v438 = (v427 * ((v96 * (v414 - (v24 * v428))).exp())) * ((((-v167) * v94) / v428).exp());
            let v442 = v96 * ((v387 - v287) + v390);
            let v449 = (-v447) * v94;
            let v452 = (v439 * ((v442 / v443).exp())) * ((v449 / v443).exp());
            let v460 = (v453 * ((v442 / v454).exp())) * ((v449 / v454).exp());
            let v462 = if v461 == v2 { 1.0 } else { 0.0 };
            let v1122: f64;
            let v1135: f64;
            let v1177: f64;
            if v462 != 0.0 {
                let v469 = v463 * ((((-v464) * v94) / v443).exp());
                let v475 = v470 * (((-v471) * v94).exp());
                let v482 = v476 * ((((-v477) * v94) / v454).exp());
                v1122 = v469;
                v1135 = v475;
                v1177 = v482;
            } else {
                v1122 = v0;
                v1135 = v0;
                v1177 = v0;
            }
            let v493 = (v483 * ((v96 * ((v387 - v408) + v390)).exp())) * (((-v489) * v94).exp());
            let v503 = (v494 * ((v96 * (v414 - (v24 * v495))).exp())) * ((v423 / v495).exp());
            let v512 = (v504 * ((v96 * (v387 / v505)).exp())) * ((v423 / v505).exp());
            let v519 = (v513 * (v88.sqrt())) * ((v516 * v95).exp());
            let v523 = (v520 * v53).powf(v522);
            let v524 = v2 / v272;
            let v533 = (((((((v525 * v520) * v520) * v523) * v524) * v54) * v268) * v53) * v53;
            let v543 = ((((((v534 * v523) * v267) * v267) * v55) * v55) * v272) * ((v525 - v533).exp());
            let v547 = (v544 * v83).powf(v546);
            let v557 = (((((((v549 * v544) * v544) * v547) * (v2 / v274)) * v56) * v270) * v83) * v83;
            let v567 = ((((((v558 * v547) * v269) * v269) * v84) * v84) * v274) * ((v549 - v557).exp());
            let v569 = (v96 * v294).exp();
            let v572 = (v570 * v569) * v282;
            let v575 = (v573 * v569) * v524;
            let v581 = v576 * ((v96 * ((v294 + v293) - v2)).exp());
            let v586 = v582 * ((v96 * (v318 - v2)).exp());
            let v588 = v581 + v586;
            let v591 = (v587 * v588) / (v576 + v582);
            let v597 = v592 * ((v96 * (v593 - v2)).exp());
            let v599 = v15 - v598;
            let v601 = if v15 < v600 { 1.0 } else { 0.0 };
            let v1605: f64;
            if v601 != 0.0 {
                let v610 = v602 * ((v2 + (v603 * v599)) - ((v606 * v599) * v599));
                v1605 = v610;
            } else {
                let v612 = v602 * v611;
                v1605 = v612;
            }
            let v614 = v613 * v569;
            let v615 = if v305 > v0 { 1.0 } else { 0.0 };
            let v1836: f64;
            if v615 != 0.0 {
                let v616 = v2 / v309;
                let v617 = if v616 > v22 { 1.0 } else { 0.0 };
                let v1837: f64;
                if v617 != 0.0 {
                    v1837 = v22;
                } else {
                    v1837 = v616;
                }
                v1836 = v1837;
            } else {
                v1836 = v0;
            }
            let v618 = if v310 > v0 { 1.0 } else { 0.0 };
            let v1843: f64;
            if v618 != 0.0 {
                let v619 = v2 / v314;
                let v620 = if v619 > v22 { 1.0 } else { 0.0 };
                let v1844: f64;
                if v620 != 0.0 {
                    v1844 = v22;
                } else {
                    v1844 = v619;
                }
                v1843 = v1844;
            } else {
                v1843 = v0;
            }
            let v621 = if v315 > v0 { 1.0 } else { 0.0 };
            let v1845: f64;
            if v621 != 0.0 {
                let v622 = v2 / v316;
                let v623 = if v622 > v22 { 1.0 } else { 0.0 };
                let v1846: f64;
                if v623 != 0.0 {
                    v1846 = v22;
                } else {
                    v1846 = v622;
                }
                v1845 = v1846;
            } else {
                v1845 = v0;
            }
            let v627 = v1 * (v624 - v625);
            let v630 = v1 * (v624 - v628);
            let v633 = v1 * (v624 - v631);
            let v636 = v1 * (v634 - v631);
            let v638 = v1 * (v634 - v624);
            let v640 = v1 * (v625 - v628);
            let v644 = v1 * (v642 - v634);
            let v646 = v1 * (v642 - v641);
            let v649 = v1 * (v642 - v647);
            let v658 = ((v638 + v630) - v640) - (v1 * (v650 - v625));
            let v663 = v649 + ((((-v649) + v644) + v658) - (v1 * (v653 - v650)));
            let v664 = v630 * v92;
            let v666 = if v664 < v665 { 1.0 } else { 0.0 };
            let v883: f64;
            if v666 != 0.0 {
                let v667 = v664.exp();
                v883 = v667;
            } else {
                let v671 = (v665.exp()) * (v2 + (v664 - v665));
                v883 = v671;
            }
            let v672 = v633 * v92;
            let v673 = v672 / v393;
            let v674 = if v673 < v665 { 1.0 } else { 0.0 };
            let v988: f64;
            if v674 != 0.0 {
                let v675 = v673.exp();
                v988 = v675;
            } else {
                let v679 = (v665.exp()) * (v2 + (v673 - v665));
                v988 = v679;
            }
            let v680 = v658 * v92;
            let v681 = if v680 < v665 { 1.0 } else { 0.0 };
            let v1374: f64;
            if v681 != 0.0 {
                let v682 = v680.exp();
                v1374 = v682;
            } else {
                let v686 = (v665.exp()) * (v2 + (v680 - v665));
                v1374 = v686;
            }
            let v687 = v638 * v92;
            let v688 = if v687 < v665 { 1.0 } else { 0.0 };
            let v1505: f64;
            if v688 != 0.0 {
                let v689 = v687.exp();
                v1505 = v689;
            } else {
                let v693 = (v665.exp()) * (v2 + (v687 - v665));
                v1505 = v693;
            }
            let v694 = v663 * v92;
            let v695 = if v694 < v665 { 1.0 } else { 0.0 };
            let v1392: f64;
            if v695 != 0.0 {
                let v696 = v694.exp();
                v1392 = v696;
            } else {
                let v700 = (v665.exp()) * (v2 + (v694 - v665));
                v1392 = v700;
            }
            let v703 = (v663 - v701) * v92;
            let v704 = if v703 < v665 { 1.0 } else { 0.0 };
            let v1773: f64;
            if v704 != 0.0 {
                let v705 = v703.exp();
                v1773 = v705;
            } else {
                let v709 = (v665.exp()) * (v2 + (v703 - v665));
                v1773 = v709;
            }
            let v712 = if ((v658 - v701) * v92) < v665 { 1.0 } else { 0.0 };
            if v712 != 0.0 {
            } else {
            }
            let v714 = (v630 - v701) * v92;
            let v715 = if v714 < v665 { 1.0 } else { 0.0 };
            let v729: f64;
            if v715 != 0.0 {
                let v716 = v714.exp();
                v729 = v716;
            } else {
                let v720 = (v665.exp()) * (v2 + (v714 - v665));
                v729 = v720;
            }
            let v722 = (v627 - v701) * v92;
            let v723 = if v722 < v665 { 1.0 } else { 0.0 };
            let v733: f64;
            if v723 != 0.0 {
                let v724 = v722.exp();
                v733 = v724;
            } else {
                let v728 = (v665.exp()) * (v2 + (v722 - v665));
                v733 = v728;
            }
            let v732 = (v2 + (v387 * v729)).sqrt();
            let v736 = (v2 + (v387 * v733)).sqrt();
            let v738 = v2 + v736;
            let v739 = (v24 * v733) / v738;
            let v741 = if v739 < v740 { 1.0 } else { 0.0 };
            let v828: f64;
            if v741 != 0.0 {
                v828 = v740;
            } else {
                v828 = v739;
            }
            let v743 = v732 + v2;
            let v747 = v90 * ((v732 - v736) - ((v743 / v738).ln()));
            let v749 = (v747 + v640) / v321;
            let v750 = if v749 > v0 { 1.0 } else { 0.0 };
            let v939: f64;
            let v952: f64;
            let v967: f64;
            let v994: f64;
            let v1553: f64;
            let v1589: f64;
            if v750 != 0.0 {
                let v752 = if v627 < v751 { 1.0 } else { 0.0 };
                let v765: f64;
                if v752 != 0.0 {
                    v765 = v627;
                } else {
                    let v756 = v751 + ((v2 + (v627 - v751)).ln());
                    v765 = v756;
                }
                let v759 = (v376 * v749) * v321;
                let v766 = (v701 + ((v24 * v90) * (((v759 * v92) + v2).ln()))) - v765;
                let v768 = v767 * v701;
                let v769 = v768 * v768;
                let v770 = v766 * v766;
                let v771 = if v766 < v0 { 1.0 } else { 0.0 };
                let v781: f64;
                if v771 != 0.0 {
                    let v776 = (v376 * v769) / (((v770 + v769).sqrt()) - v766);
                    v781 = v776;
                } else {
                    let v780 = v376 * (((v770 + v769).sqrt()) + v766);
                    v781 = v780;
                }
                let v784 = v782 * v783;
                let v790 = (v781 * (v781 + v784)) / (v783 * (v781 + (v782 * v321)));
                let v791 = v749 / v790;
                let v794 = (v791 - v2) / v793;
                let v795 = if v791 < v2 { 1.0 } else { 0.0 };
                let v807: f64;
                if v795 != 0.0 {
                    let v800 = v2 + (v793 * ((v2 + (v794.exp())).ln()));
                    v807 = v800;
                } else {
                    let v806 = v791 + (v793 * ((v2 + ((-v794).exp())).ln()));
                    v807 = v806;
                }
                let v815 = v807 / (v2 + (v793 * ((v2 + ((v808 / v793).exp())).ln())));
                let v816 = v781 / v784;
                let v819 = v2 + v816;
                let v826 = (v2 + ((v2 + (((v387 * v815) * v816) * v819)).sqrt())) / ((v24 * v815) * v819);
                let v829 = v828 * v826;
                let v832 = ((v2 - v826) + v829) / (v2 + v829);
                let v834 = (v759 * v832) * v92;
                let v839 = (v24 * v834) + (v828 * ((v828 + v834) + v2));
                let v841 = v376 * (v834 - v2);
                let v843 = (v841 * v841) + v839;
                let v844 = if v834 >= v2 { 1.0 } else { 0.0 };
                let v850: f64;
                if v844 != 0.0 {
                    let v846 = v841 + (v843.sqrt());
                    v850 = v846;
                } else {
                    let v849 = v839 / ((v843.sqrt()) - v841);
                    v850 = v849;
                }
                let v852 = if v850 < v851 { 1.0 } else { 0.0 };
                let v853: f64;
                if v852 != 0.0 {
                    v853 = v851;
                } else {
                    v853 = v850;
                }
                let v858 = (v853 * (v853 + v2)) * ((v701 * v92).exp());
                let v861 = (v376 * v783) * (v749 - v782);
                let v868 = v861 + (((v861 * v861) + (((v783 * v321) * v782) * v749)).sqrt());
                let v870 = if v869 == v0 { 1.0 } else { 0.0 };
                let v953: f64;
                if v870 != 0.0 {
                    let v871 = v277 * v39;
                    v953 = v871;
                } else {
                    let v876 = v277 * (v39 + ((v24 * v749) / (v749 + v790)));
                    v953 = v876;
                }
                let v878 = v782 + v749;
                let v879 = (v782 * v749) / v878;
                let v880 = v782 / v878;
                v939 = v868;
                v952 = v953;
                v967 = v880;
                v994 = v858;
                v1553 = v832;
                v1589 = v879;
            } else {
                let v882 = (v24 * v729) / v743;
                let v894 = if (if (v640.abs()) < (v885 * v90) { 1.0 } else { 0.0 }) != 0.0 || (if (v747.abs()) < ((v889 * v90) * (v732 + v736)) { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v1554: f64;
                if v894 != 0.0 {
                    let v896 = v376 * (v882 + v828);
                    let v898 = v896 / (v896 + v2);
                    v1554 = v898;
                } else {
                    let v901 = v747 / ((v747 + v630) - v627);
                    v1554 = v901;
                }
                let v902 = v39 * v277;
                let v904 = v2 - (v749 / v782);
                v939 = v640;
                v952 = v902;
                v967 = v904;
                v994 = v883;
                v1553 = v1554;
                v1589 = v749;
            }
            let v909 = v267 * (v2 - (v137.powf((v905 / v25))));
            let v910 = v39 * v267;
            let v912 = (v633 - v909) / v910;
            let v913 = if v633 < v909 { 1.0 } else { 0.0 };
            let v925: f64;
            if v913 != 0.0 {
                let v918 = v633 - (v910 * ((v2 + (v912.exp())).ln()));
                v925 = v918;
            } else {
                let v924 = v909 - (v910 * ((v2 + ((-v912).exp())).ln()));
                v925 = v924;
            }
            let v928 = v2 - v25;
            let v929 = (v2 - (v925 * v268)).powf(v928);
            let v935 = ((v267 / v928) * (v2 - v929)) + (v137 * (v633 - v925));
            let v937 = if v936 == v2 { 1.0 } else { 0.0 };
            let v949: f64;
            if v937 != 0.0 {
                v949 = v627;
            } else {
                let v938 = if v936 == v24 { 1.0 } else { 0.0 };
                let v950: f64;
                if v938 != 0.0 {
                    let v940 = v627 + v939;
                    v950 = v940;
                } else {
                    v950 = v630;
                }
                v949 = v950;
            }
            let v942 = v2 - v285;
            let v943 = (v24 - v285) / v942;
            let v948 = v277 * (v2 - (v943.powf((v944 / v57))));
            let v954 = (v949 - v948) / v952;
            let v955 = if v949 < v948 { 1.0 } else { 0.0 };
            let v972: f64;
            if v955 != 0.0 {
                let v960 = v949 - (v952 * ((v2 + (v954.exp())).ln()));
                v972 = v960;
            } else {
                let v966 = v948 - (v952 * ((v2 + ((-v954).exp())).ln()));
                v972 = v966;
            }
            let v969 = v967.powf(v968);
            let v970 = v2 - v57;
            let v971 = v277 / v970;
            let v985 = (v942 * ((v971 * (v2 - (v969 * ((v2 - (v972 / v277)).powf(v970))))) + ((v969 * v943) * (v949 - v972)))) + (v285 * v627);
            let v987 = (v387 * v401) / v406;
            let v989 = v987 * v988;
            let v993 = v989 / (v2 + ((v2 + v989).sqrt()));
            let v997 = v994.powf((v2 / v995));
            let v998 = v987 * v997;
            let v1002 = v998 / (v2 + ((v2 + v998).sqrt()));
            let v1003 = if v613 == v0 { 1.0 } else { 0.0 };
            let v1024: f64;
            if v1003 != 0.0 {
                let v1007 = (v2 + (v935 / v575)) + (v985 / v572);
                v1024 = v1007;
            } else {
                let v1022 = ((((((v935 / v575) + v2) * v614) * v92).exp()) - (((((-v985) / v572) * v614) * v92).exp())) / (((v614 * v92).exp()) - v2);
                v1024 = v1022;
            }
            let v1025 = v1024 * v1024;
            let v1026 = if v1024 < v0 { 1.0 } else { 0.0 };
            let v1036: f64;
            if v1026 != 0.0 {
                let v1031 = v1027 / (((v1025 + v1023).sqrt()) - v1024);
                v1036 = v1031;
            } else {
                let v1035 = v376 * (((v1025 + v1023).sqrt()) + v1024);
                v1036 = v1035;
            }
            let v1039 = v2 + (v376 * (v993 + v1002));
            let v1040 = v1036 * v1039;
            let v1043 = (v1041 * v401) * v997;
            let v1044 = v401 * v988;
            let v1046 = (v1044 - v1043) / v1040;
            let v1048 = v633 / v1047;
            let v1049 = if v633 < v0 { 1.0 } else { 0.0 };
            let v1060: f64;
            if v1049 != 0.0 {
                let v1053 = v1047 * ((v2 + (v1048.exp())).ln());
                v1060 = v1053;
            } else {
                let v1059 = v633 + (v1047 * ((v2 + ((-v1048).exp())).ln()));
                v1060 = v1059;
            }
            let v1062 = v1060 / v1061;
            let v1063 = if v1062 < v665 { 1.0 } else { 0.0 };
            let v1069: f64;
            if v1063 != 0.0 {
                let v1064 = v1062.exp();
                v1069 = v1064;
            } else {
                let v1068 = (v665.exp()) * (v2 + (v1062 - v665));
                v1069 = v1068;
            }
            let v1071 = v519 * (v1069 - v2);
            let v1074 = (v633 - v1072) / v23;
            let v1075 = if v633 < v1072 { 1.0 } else { 0.0 };
            let v1088: f64;
            if v1075 != 0.0 {
                let v1080 = v633 - (v23 * ((v2 + (v1074.exp())).ln()));
                v1088 = v1080;
            } else {
                let v1086 = v1072 - (v23 * ((v2 + ((-v1074).exp())).ln()));
                v1088 = v1086;
            }
            let v1090 = v1072 - v1088;
            let v1092 = (v1087 * v1088) * (v1090 * v1090);
            let v1093 = v672 / v443;
            let v1094 = if v1093 < v665 { 1.0 } else { 0.0 };
            let v1119: f64;
            if v1094 != 0.0 {
                let v1095 = v1093.exp();
                v1119 = v1095;
            } else {
                let v1099 = (v665.exp()) * (v2 + (v1093 - v665));
                v1119 = v1099;
            }
            let v1813: f64;
            if v462 != 0.0 {
                let v1102 = (v633 - v1100) * v92;
                let v1103 = if v1102 < v665 { 1.0 } else { 0.0 };
                let v1125: f64;
                if v1103 != 0.0 {
                    let v1104 = v1102.exp();
                    v1125 = v1104;
                } else {
                    let v1108 = (v665.exp()) * (v2 + (v1102 - v665));
                    v1125 = v1108;
                }
                let v1111 = (v1046 / v401) - v1110;
                let v1113 = if v1111 < v1112 { 1.0 } else { 0.0 };
                let v1138: f64;
                if v1113 != 0.0 {
                    let v1114 = v1111.exp();
                    v1138 = v1114;
                } else {
                    let v1118 = v1115 * (v2 + (v1111 - v1112));
                    v1138 = v1118;
                }
                let v1120 = v1119 - v2;
                let v1142 = ((v452 * v1120) + ((((v1122 * v24) * v1120) / (v2 + ((v2 + (v387 * v1125)).sqrt()))) * (v2 + (v985 / v572)))) + (((v1135 * (v994 - v2)) * v1138) / (v2 + v1138));
                v1813 = v1142;
            } else {
                let v1144 = if v1143 == v0 { 1.0 } else { 0.0 };
                let v1814: f64;
                if v1144 != 0.0 {
                    let v1146 = v452 * (v1119 - v2);
                    v1814 = v1146;
                } else {
                    let v1157 = v452 * (((v2 - v1143) * (v1119 - v2)) + ((v1143 * ((v1119 + v994) - v24)) * (v2 + (v985 / v572))));
                    v1814 = v1157;
                }
                v1813 = v1814;
            }
            let v1158 = v636 * v92;
            let v1159 = v1158 / v454;
            let v1160 = if v1159 < v665 { 1.0 } else { 0.0 };
            let v1174: f64;
            if v1160 != 0.0 {
                let v1161 = v1159.exp();
                v1174 = v1161;
            } else {
                let v1165 = (v665.exp()) * (v2 + (v1159 - v665));
                v1174 = v1165;
            }
            let v1810: f64;
            if v462 != 0.0 {
                let v1167 = (v636 - v1100) * v92;
                let v1168 = if v1167 < v665 { 1.0 } else { 0.0 };
                let v1180: f64;
                if v1168 != 0.0 {
                    let v1169 = v1167.exp();
                    v1180 = v1169;
                } else {
                    let v1173 = (v665.exp()) * (v2 + (v1167 - v665));
                    v1180 = v1173;
                }
                let v1175 = v1174 - v2;
                let v1186 = (v460 * v1175) + (((v1177 * v24) * v1175) / (v2 + ((v2 + (v387 * v1180)).sqrt())));
                v1810 = v1186;
            } else {
                let v1188 = v460 * (v1174 - v2);
                v1810 = v1188;
            }
            let v1189 = v672 / v415;
            let v1190 = if v1189 < v665 { 1.0 } else { 0.0 };
            let v1196: f64;
            if v1190 != 0.0 {
                let v1191 = v1189.exp();
                v1196 = v1191;
            } else {
                let v1195 = (v665.exp()) * (v2 + (v1189 - v665));
                v1196 = v1195;
            }
            let v1198 = v426 * (v1196 - v2);
            let v1199 = v1158 / v495;
            let v1200 = if v1199 < v665 { 1.0 } else { 0.0 };
            let v1206: f64;
            if v1200 != 0.0 {
                let v1201 = v1199.exp();
                v1206 = v1201;
            } else {
                let v1205 = (v665.exp()) * (v2 + (v1199 - v665));
                v1206 = v1205;
            }
            let v1208 = v503 * (v1206 - v2);
            let v1209 = v680 / v428;
            let v1210 = if v1209 < v665 { 1.0 } else { 0.0 };
            let v1216: f64;
            if v1210 != 0.0 {
                let v1211 = v1209.exp();
                v1216 = v1211;
            } else {
                let v1215 = (v665.exp()) * (v2 + (v1209 - v665));
                v1216 = v1215;
            }
            let v1218 = v438 * (v1216 - v2);
            let v1219 = v1158 / v505;
            let v1220 = if v1219 < v665 { 1.0 } else { 0.0 };
            let v1226: f64;
            if v1220 != 0.0 {
                let v1221 = v1219.exp();
                v1226 = v1221;
            } else {
                let v1225 = (v665.exp()) * (v2 + (v1219 - v665));
                v1226 = v1225;
            }
            let v1228 = v512 * (v1226 - v2);
            let v1232 = if (if (if v534 > v0 { 1.0 } else { 0.0 }) != 0.0 && (if v525 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && v1049 != 0.0 { 1.0 } else { 0.0 };
            let v1816: f64;
            if v1232 != 0.0 {
                let v1236 = v533 * (v2 - (v27 / (v24 * v929)));
                let v1237 = if v1236 < v665 { 1.0 } else { 0.0 };
                let v1298: f64;
                if v1237 != 0.0 {
                    let v1238 = v1236.exp();
                    v1298 = v1238;
                } else {
                    let v1242 = (v665.exp()) * (v2 + (v1236 - v665));
                    v1298 = v1242;
                }
                let v1243 = v633 * v268;
                let v1254 = v25 - v2;
                let v1269 = ((v633 * v27) * v533) / (v520 * ((((((v1243 * v1243) + v1245).sqrt()).powf((v1248 - v25))) * ((v25 * ((v2 - (v25 * v25)) - ((v137 * v1243) * v1254))) - (((v414 * v1243) * v1243) * (v1254 + v1243)))) * v1264));
                let v1271 = if v1269 < v1270 { 1.0 } else { 0.0 };
                let v1295: f64;
                if v1271 != 0.0 {
                    let v1272 = if v1269 < v665 { 1.0 } else { 0.0 };
                    let v1279: f64;
                    if v1272 != 0.0 {
                        let v1273 = v1269.exp();
                        v1279 = v1273;
                    } else {
                        let v1277 = (v665.exp()) * (v2 + (v1269 - v665));
                        v1279 = v1277;
                    }
                    let v1283 = (-v633) * (v2 + ((v2 - v1279) / v1269));
                    v1295 = v1283;
                } else {
                    let v1293 = ((v633 * v376) * v1269) * (v2 + ((v1269 * v1286) * (v2 + (v1288 * v1269))));
                    v1295 = v1293;
                }
                let v1301 = (((((v24 * v543) * v1295) * v929) * v1298) * v268) * v28;
                v1816 = v1301;
            } else {
                v1816 = v0;
            }
            let v1306 = if (if (if v558 > v0 { 1.0 } else { 0.0 }) != 0.0 && (if v549 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if v627 < v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let v1474: f64;
            if v1306 != 0.0 {
                let v1307 = v627 * v270;
                let v1309 = (v2 - v1307).powf(v970);
                let v1313 = v557 * (v2 - (v59 / (v24 * v1309)));
                let v1314 = if v1313 < v665 { 1.0 } else { 0.0 };
                let v1370: f64;
                if v1314 != 0.0 {
                    let v1315 = v1313.exp();
                    v1370 = v1315;
                } else {
                    let v1319 = (v665.exp()) * (v2 + (v1313 - v665));
                    v1370 = v1319;
                }
                let v1329 = v57 - v2;
                let v1343 = ((v627 * v59) * v557) / (v544 * ((((((v1307 * v1307) + v1245).sqrt()).powf((v1323 - v57))) * ((v57 * ((v2 - (v57 * v57)) - ((v137 * v1307) * v1329))) - (((v414 * v1307) * v1307) * (v1329 + v1307)))) * v1264));
                let v1345 = if v1343 < v1344 { 1.0 } else { 0.0 };
                let v1367: f64;
                if v1345 != 0.0 {
                    let v1346 = if v1343 < v665 { 1.0 } else { 0.0 };
                    let v1353: f64;
                    if v1346 != 0.0 {
                        let v1347 = v1343.exp();
                        v1353 = v1347;
                    } else {
                        let v1351 = (v665.exp()) * (v2 + (v1343 - v665));
                        v1353 = v1351;
                    }
                    let v1357 = (-v627) * (v2 + ((v2 - v1353) / v1343));
                    v1367 = v1357;
                } else {
                    let v1365 = ((v627 * v376) * v1343) * (v2 + ((v1343 * v1286) * (v2 + (v1288 * v1343))));
                    v1367 = v1365;
                }
                let v1373 = (((((v24 * v567) * v1367) * v1309) * v1370) * v270) * v60;
                v1474 = v1373;
            } else {
                v1474 = v0;
            }
            let v1379 = (v387 * v493) / v412;
            let v1384 = ((v24 * v493) * (v1374 - v2)) / (v2 + ((v2 + (v1379 * v1374)).sqrt()));
            let v1387 = if v8 > v0 { 1.0 } else { 0.0 };
            let v1388 = if (if v1385 > v0 { 1.0 } else { 0.0 }) != 0.0 && v1387 != 0.0 { 1.0 } else { 0.0 };
            let v1478: f64;
            let v1481: f64;
            let v1804: f64;
            if v1388 != 0.0 {
                let v1389 = v1384 * v9;
                let v1399 = (((v8 * v24) * v493) * (v1392 - v2)) / (v2 + ((v2 + (v1379 * v1392)).sqrt()));
                let v1400 = if v1385 == v2 { 1.0 } else { 0.0 };
                let v1425: f64;
                if v1400 != 0.0 {
                    let v1402 = (v8 * v493) * v309;
                    let v1407 = v663 - (v90 * (v24 - ((v1402 * v92).ln())));
                    let v1409 = v1407 * v1407;
                    let v1410 = if v1407 < v0 { 1.0 } else { 0.0 };
                    let v1420: f64;
                    if v1410 != 0.0 {
                        let v1415 = v1411 / (((v1409 + v1408).sqrt()) - v1407);
                        v1420 = v1415;
                    } else {
                        let v1419 = v376 * (((v1409 + v1408).sqrt()) + v1407);
                        v1420 = v1419;
                    }
                    let v1424 = v1420 / ((v1402 + (v1399 * v309)) + v1420);
                    v1425 = v1424;
                } else {
                    v1425 = v2;
                }
                let v1426 = v1425 * v1399;
                v1478 = v1389;
                v1481 = v1426;
                v1804 = v1425;
            } else {
                v1478 = v1384;
                v1481 = v0;
                v1804 = v2;
            }
            let v1428 = if v1427 == v2 { 1.0 } else { 0.0 };
            let v1475: f64;
            if v1428 != 0.0 {
                let v1429 = v638 + v627;
                let v1436 = ((v1432 * v1429) * v1434) * v1429;
                let v1439 = if (v1437 * v1429) < v0 { 1.0 } else { 0.0 };
                let v1465: f64;
                if v1439 != 0.0 {
                    let v1446 = v1440 / (((v1436 + v1431).sqrt()) - (v1443 * v1429));
                    v1465 = v1446;
                } else {
                    let v1452 = v376 * (((v1436 + v1431).sqrt()) + (v1449 * v1429));
                    v1465 = v1452;
                }
                let v1456 = v2 / (v2 - (v87.powf(v1453)));
                let v1458 = v87 * v1457;
                let v1464 = (((v1456 * v1456) * (v87.powf((v1453 - v2)))) * v1453) / v1457;
                let v1466 = if v1465 < v1458 { 1.0 } else { 0.0 };
                let v1476: f64;
                if v1466 != 0.0 {
                    let v1470 = v2 / (v2 - ((v1465 / v1457).powf(v1453)));
                    v1476 = v1470;
                } else {
                    let v1473 = v1456 + ((v1465 - v1458) * v1464);
                    v1476 = v1473;
                }
                v1475 = v1476;
            } else {
                v1475 = v2;
            }
            let v1477 = v1474 * v1475;
            let v1479 = v1478 * v1475;
            let v1480 = v1218 * v1475;
            let v1482 = v1481 * v1475;
            let v1486 = (v2 + (v935 / v575)) + (v985 / v572);
            let v1488 = v1486 * v1486;
            let v1489 = if v1486 < v0 { 1.0 } else { 0.0 };
            let v1499: f64;
            if v1489 != 0.0 {
                let v1494 = v1490 / (((v1488 + v1487).sqrt()) - v1486);
                v1499 = v1494;
            } else {
                let v1498 = v376 * (((v1488 + v1487).sqrt()) + v1486);
                v1499 = v1498;
            }
            let v1501 = v298 / (v1499 * v1039);
            let v1502 = if v1501 < v21 { 1.0 } else { 0.0 };
            let v1503: f64;
            if v1502 != 0.0 {
                v1503 = v21;
            } else {
                v1503 = v1501;
            }
            let v1504 = v137 * v1503;
            let v1506 = if v1046 > v0 { 1.0 } else { 0.0 };
            let v1817: f64;
            if v1506 != 0.0 {
                let v1508 = if v1507 == v2 { 1.0 } else { 0.0 };
                let v1670: f64;
                if v1508 != 0.0 {
                    let v1510 = if v627 < v1509 { 1.0 } else { 0.0 };
                    let v1671: f64;
                    if v1510 != 0.0 {
                        let v1513 = (-v1046) / v1512;
                        let v1514 = if v1513 < v665 { 1.0 } else { 0.0 };
                        let v1521: f64;
                        if v1514 != 0.0 {
                            let v1515 = v1513.exp();
                            v1521 = v1515;
                        } else {
                            let v1519 = (v665.exp()) * (v2 + (v1513 - v665));
                            v1521 = v1519;
                        }
                        let v1522 = (v1509 - v627) * v1521;
                        let v1527 = (-v1523) * (v1522.powf(v1525));
                        let v1528 = if v1527 < v665 { 1.0 } else { 0.0 };
                        let v1537: f64;
                        if v1528 != 0.0 {
                            let v1529 = v1527.exp();
                            v1537 = v1529;
                        } else {
                            let v1533 = (v665.exp()) * (v2 + (v1527 - v665));
                            v1537 = v1533;
                        }
                        let v1538 = ((v1534 / v1523) * v1522) * v1537;
                        v1671 = v1538;
                    } else {
                        v1671 = v0;
                    }
                    v1670 = v1671;
                } else {
                    let v1539 = if v1507 == v24 { 1.0 } else { 0.0 };
                    let v1672: f64;
                    if v1539 != 0.0 {
                        let v1540 = if v627 < v701 { 1.0 } else { 0.0 };
                        let v1673: f64;
                        if v1540 != 0.0 {
                            let v1545 = (v24 * v1541) / (v1543 * v1543);
                            let v1546 = v701 - v627;
                            let v1550 = ((v24 * (v1546 / v967)) / v1545).sqrt();
                            let v1552 = if v1551 == v0 { 1.0 } else { 0.0 };
                            let v1559: f64;
                            if v1552 != 0.0 {
                                v1559 = v1543;
                            } else {
                                let v1556 = v2 - (v376 * v1553);
                                let v1558 = (v1543 * v1556) * v1556;
                                v1559 = v1558;
                            }
                            let v1565 = (v1550 * v1559) / (((v1550 * v1550) + (v1559 * v1559)).sqrt());
                            let v1566 = v1546 / v1565;
                            let v1567 = v376 * v1565;
                            let v1568 = v1567 * v1545;
                            let v1570 = v1566 + (v1568 * v967);
                            let v1597: f64;
                            if v1552 != 0.0 {
                                v1597 = v1570;
                            } else {
                                let v1572 = v24 * v1571;
                                let v1584 = v1566 - (v1568 * (((v2 + v1571) / (v2 + v1572)) - (v1046 / (v782 * (v2 + (v1572 * (v2 + (v24 * v1553))))))));
                                let v1585 = v1584 - v1570;
                                let v1596 = v376 * ((v1584 + v1570) + (((v1585 * v1585) + ((((v39 * v1566) * v1566) * v1589) / v782)).sqrt()));
                                v1597 = v1596;
                            }
                            let v1599 = (v1597 - v1566) / v1597;
                            let v1602 = if (v1599.abs()) > v1601 { 1.0 } else { 0.0 };
                            let v1674: f64;
                            if v1602 != 0.0 {
                                let v1603 = v1567 / v1599;
                                let v1610 = (-v1605) / v1597;
                                let v1617 = (((v1604 / v1605) * v1597) * v1603) * ((v1610.exp()) - ((v1610 * (v2 + (v1559 / v1603))).exp()));
                                v1674 = v1617;
                            } else {
                                let v1622 = (v1604 * v1559) * (((-v1605) / v1597).exp());
                                v1674 = v1622;
                            }
                            v1673 = v1674;
                        } else {
                            v1673 = v0;
                        }
                        v1672 = v1673;
                    } else {
                        let v1623 = if v1507 == v137 { 1.0 } else { 0.0 };
                        let v1675: f64;
                        if v1623 != 0.0 {
                            let v1624 = if v627 < v1509 { 1.0 } else { 0.0 };
                            let v1676: f64;
                            if v1624 != 0.0 {
                                let v1625 = v1509 - v627;
                                let v1633 = (v1625.powf(v1525)) * ((v2 - (v1046 / (v1627 + v1046))).powf(v1631));
                                let v1634 = if v1551 == v0 { 1.0 } else { 0.0 };
                                let v1658: f64;
                                if v1634 != 0.0 {
                                    v1658 = v1633;
                                } else {
                                    let v1637 = (v1046 - v1635) / v1627;
                                    let v1640 = (v1637 - v2) / v1639;
                                    let v1641 = if v1637 < v2 { 1.0 } else { 0.0 };
                                    let v1653: f64;
                                    if v1641 != 0.0 {
                                        let v1646 = v2 + (v1639 * ((v2 + (v1640.exp())).ln()));
                                        v1653 = v1646;
                                    } else {
                                        let v1652 = v1637 + (v1639 * ((v2 + ((-v1640).exp())).ln()));
                                        v1653 = v1652;
                                    }
                                    let v1656 = v1633 * (v1653.powf(v1654));
                                    v1658 = v1656;
                                }
                                let v1659 = (-v1523) * v1658;
                                let v1660 = if v1659 < v665 { 1.0 } else { 0.0 };
                                let v1668: f64;
                                if v1660 != 0.0 {
                                    let v1661 = v1659.exp();
                                    v1668 = v1661;
                                } else {
                                    let v1665 = (v665.exp()) * (v2 + (v1659 - v665));
                                    v1668 = v1665;
                                }
                                let v1669 = ((v1534 / v1523) * v1625) * v1668;
                                v1676 = v1669;
                            } else {
                                v1676 = v0;
                            }
                            v1675 = v1676;
                        } else {
                            v1675 = v0;
                        }
                        v1672 = v1675;
                    }
                    v1670 = v1672;
                }
                let v1677 = if v1670 > v0 { 1.0 } else { 0.0 };
                let v1818: f64;
                if v1677 != 0.0 {
                    let v1679 = if v1678 == v2 { 1.0 } else { 0.0 };
                    let v1819: f64;
                    if v1679 != 0.0 {
                        let v1681 = v1680 + v1504;
                        let v1689 = ((v90 / (v1046 * v1681)) + ((v1040 / v401) * v452)) + (v1687 / v1681);
                        let v1690 = if v1507 == v137 { 1.0 } else { 0.0 };
                        let v1820: f64;
                        if v1690 != 0.0 {
                            let v1692 = (v1670 - v1689) / v1430;
                            let v1693 = if v1670 < v1689 { 1.0 } else { 0.0 };
                            let v1705: f64;
                            if v1693 != 0.0 {
                                let v1698 = v1670 - (v1430 * ((v2 + (v1692.exp())).ln()));
                                v1705 = v1698;
                            } else {
                                let v1704 = v1689 - (v1430 * ((v2 + ((-v1692).exp())).ln()));
                                v1705 = v1704;
                            }
                            let v1706 = v1046 * v1705;
                            v1820 = v1706;
                        } else {
                            let v1710 = ((v1046 * v1670) * v1689) / (v1670 + v1689);
                            v1820 = v1710;
                        }
                        v1819 = v1820;
                    } else {
                        let v1711 = v1046 * v1670;
                        v1819 = v1711;
                    }
                    v1818 = v1819;
                } else {
                    v1818 = v0;
                }
                v1817 = v1818;
            } else {
                v1817 = v0;
            }
            let v1712 = if v994 > v0 { 1.0 } else { 0.0 };
            if v1712 != 0.0 {
            } else {
            }
            let v1713 = if v636 < v909 { 1.0 } else { 0.0 };
            if v1713 != 0.0 {
            } else {
            }
            let v1715 = v581 * v406;
            let v1716 = v39 * v277;
            let v1717 = if v658 < v948 { 1.0 } else { 0.0 };
            if v1717 != 0.0 {
            } else {
            }
            let v1718 = v2 - v1714;
            let v1720 = (v663 - v948) / v1716;
            let v1721 = if v663 < v948 { 1.0 } else { 0.0 };
            let v1733: f64;
            if v1721 != 0.0 {
                let v1726 = v663 - (v1716 * ((v2 + (v1720.exp())).ln()));
                v1733 = v1726;
            } else {
                let v1732 = v948 - (v1716 * ((v2 + ((-v1720).exp())).ln()));
                v1733 = v1732;
            }
            let v1747 = ((v284 * ((v942 * ((v971 * (v2 - ((v2 - (v1733 / v277)).powf(v970)))) + (v943 * (v663 - v1733)))) + (v285 * v663))) * v1718) * v8;
            let v1751 = if (v633 / (v1748 * v90)) < v665 { 1.0 } else { 0.0 };
            if v1751 != 0.0 {
            } else {
            }
            let v1754 = ((v387 * v586) * v90) / v321;
            let v1756 = if v1755 == v0 { 1.0 } else { 0.0 };
            if v1756 != 0.0 {
            } else {
                let v1762 = if (((v658 - v1757) / v1759) * v92) < v665 { 1.0 } else { 0.0 };
                if v1762 != 0.0 {
                } else {
                }
            }
            let v1766 = if (if (if v1385 == v2 { 1.0 } else { 0.0 }) != 0.0 || (if v1385 == v137 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && v1387 != 0.0 { 1.0 } else { 0.0 };
            let v1838: f64;
            if v1766 != 0.0 {
                let v1805: f64;
                if v1756 != 0.0 {
                    let v1767 = v987 * v1392;
                    let v1774 = v387 * v1773;
                    let v1785 = (((v376 * v8) * v591) * ((v1715 * ((v1767 - v987) / (v2 + ((v2 + v1767).sqrt())))) + (v1754 * (v1774 / (v2 + ((v2 + v1774).sqrt())))))) / v588;
                    v1805 = v1785;
                } else {
                    let v1787 = (v663 - v1757) * v92;
                    let v1788 = if v1787 < v665 { 1.0 } else { 0.0 };
                    let v1798: f64;
                    if v1788 != 0.0 {
                        let v1789 = v1787.exp();
                        v1798 = v1789;
                    } else {
                        let v1793 = (v665.exp()) * (v2 + (v1787 - v665));
                        v1798 = v1793;
                    }
                    let v1803 = ((((v24 * v8) * v493) * v597) * v1392) / (v2 + ((v2 + (v387 * v1798)).sqrt()));
                    v1805 = v1803;
                }
                let v1806 = v1804 * v1805;
                v1838 = v1806;
            } else {
                v1838 = v0;
            }
            let v1808 = if v1807 == v2 { 1.0 } else { 0.0 };
            if v1808 != 0.0 {
                let v1809 = if v912 < v0 { 1.0 } else { 0.0 };
                if v1809 != 0.0 {
                } else {
                }
            } else {
            }
            let v1812 = (v1810 + v1208) + v1228;
            let v1815 = v1813 + v1198;
            if v462 != 0.0 {
            } else {
            }
            let v1823 = ((v1 * v644) / v1680) * v20;
            let v1826 = (v1 * v1824) * v646;
            let v1828 = (0e0f64) * v20;
            let v1831 = (v1 * v1829) * v649;
            let v1833 = (0e0f64) * v20;
            let v1835 = (v1 * v1482) * v20;
            let v1840 = v1 * (v1747 + v1838);
            let v1842 = (0e0f64) * v20;
            if v618 != 0.0 {
            } else {
            }
            if v621 != 0.0 {
            } else {
            }
            let v1848 = v1847 * v15;
            let v1849 = v1848 / v1687;
            let v1850 = v1848 / v1680;
            let v1851 = v1848 * v1836;
            let v1852 = v1848 * v1843;
            let v1853 = v1848 * v1845;
            let v1859 = ((v1848 / v1504) * ((v387 * v1505) + v1856)) * v1286;
            let v1861 = (v1044 + v1043) / v1040;
            let v1864 = v1862 * (v1861.abs());
            let v1866 = if v1865 > v0 { 1.0 } else { 0.0 };
            let v1871: f64;
            if v1866 != 0.0 {
                let v1868 = (v1817 / v1861).abs();
                v1871 = v1868;
            } else {
                v1871 = v0;
            }
            let v1873 = (v1869 * v1817) * (v1871 + v2);
            let v1874 = if v1861 > v0 { 1.0 } else { 0.0 };
            if v1874 != 0.0 {
            } else {
            }
            let v1876 = if v1875 == v2 { 1.0 } else { 0.0 };
            if v1876 != 0.0 {
            } else {
                let v1877 = if v1875 == v24 { 1.0 } else { 0.0 };
                if v1877 != 0.0 {
                } else {
                }
            }
            let v1883 = v1878 * ((((v1815 - v1816) + v1092) + v1071).abs());
            let v1884 = v1813 + v1810;
            let v1889 = v1885 * ((v1884.abs()).powf(v1887));
            let v1890 = if v1884 < v0 { 1.0 } else { 0.0 };
            let v1944: f64;
            if v1890 != 0.0 {
                let v1891 = -v1889;
                v1944 = v1891;
            } else {
                v1944 = v1889;
            }
            let v1893 = (v1198 + v1208) + v1228;
            let v1898 = v1894 * ((v1893.abs()).powf(v1896));
            let v1899 = if v1893 < v0 { 1.0 } else { 0.0 };
            let v1946: f64;
            if v1899 != 0.0 {
                let v1900 = -v1898;
                v1946 = v1900;
            } else {
                v1946 = v1898;
            }
            let v1903 = v1901 * (v1812.abs());
            let v1905 = v1480.abs();
            let v1906 = v1904 * v1905;
            let v1908 = v1885 * (v1905.powf(v1887));
            let v1909 = if v1480 < v0 { 1.0 } else { 0.0 };
            let v1950: f64;
            if v1909 != 0.0 {
                let v1910 = -v1908;
                v1950 = v1910;
            } else {
                v1950 = v1908;
            }
            let v1913 = v1911 * (v1477.abs());
            let v1915 = v1479.abs();
            let v1916 = v1914 * v1915;
            let v1918 = v2 - (v1385 * v8);
            let v1922 = (v1885 * v1918) * ((v1915 / v1918).powf(v1887));
            let v1923 = if v1479 < v0 { 1.0 } else { 0.0 };
            let v1953: f64;
            if v1923 != 0.0 {
                let v1924 = -v1922;
                v1953 = v1924;
            } else {
                v1953 = v1922;
            }
            let v1926 = v1482.abs();
            let v1928 = (v1925 * v1926) * v1385;
            let v1929 = if v8 == v0 { 1.0 } else { 0.0 };
            let v1936: f64;
            if v1929 != 0.0 {
                v1936 = v0;
            } else {
                let v1934 = ((v1885 * v1385) * v8) * ((v1926 / v8).powf(v1887));
                v1936 = v1934;
            }
            let v1935 = if v1482 < v0 { 1.0 } else { 0.0 };
            let v1956: f64;
            if v1935 != 0.0 {
                let v1937 = -v1936;
                v1956 = v1937;
            } else {
                v1956 = v1936;
            }
            let v1938 = v1864 * v20;
            let v1939 = v1873 * v20;
            let v1940 = v1883 * v20;
            let v1941 = v1849 * v20;
            let v1942 = v1850 * v20;
            let v1943 = v1859 * v20;
            let v1945 = v1944 * v20;
            let v1947 = v1946 * v20;
            let v1948 = v1903 * v20;
            let v1949 = v1906 * v20;
            let v1951 = v1950 * v20;
            let v1952 = v1916 * v20;
            let v1954 = v1953 * v20;
            let v1955 = v1928 * v20;
            let v1957 = v1956 * v20;
            let v1974: f64;
            let v1975: f64;
            let v1976: f64;
            let v1977: f64;
            if v462 != 0.0 {
                let v1958 = v1913 * v20;
                v1974 = v2;
                v1975 = v1958;
                v1976 = v0;
                v1977 = v0;
            } else {
                let v1959 = v1913 * v20;
                v1974 = v0;
                v1975 = v0;
                v1976 = v2;
                v1977 = v1959;
            }
            let v1978: f64;
            let v1980: f64;
            let v1982: f64;
            let v1984: f64;
            let v1986: f64;
            let v1988: f64;
            let v1990: f64;
            let v1992: f64;
            let v1994: f64;
            let v1996: f64;
            let v1998: f64;
            let v2000: f64;
            let v2002: f64;
            let v2004: f64;
            let v2006: f64;
            let v2008: f64;
            if v618 != 0.0 {
                let v1979: f64;
                let v1981: f64;
                let v1983: f64;
                let v1985: f64;
                let v1987: f64;
                let v1989: f64;
                let v1991: f64;
                let v1993: f64;
                let v1995: f64;
                let v1997: f64;
                if v621 != 0.0 {
                    let v1960 = v1851 * v20;
                    let v1961 = v1852 * v20;
                    let v1962 = v1853 * v20;
                    v1979 = v2;
                    v1981 = v1960;
                    v1983 = v2;
                    v1985 = v1961;
                    v1987 = v2;
                    v1989 = v1962;
                    v1991 = v0;
                    v1993 = v0;
                    v1995 = v0;
                    v1997 = v0;
                } else {
                    let v1963 = v1851 * v20;
                    let v1964 = v1852 * v20;
                    v1979 = v0;
                    v1981 = v0;
                    v1983 = v0;
                    v1985 = v0;
                    v1987 = v0;
                    v1989 = v0;
                    v1991 = v2;
                    v1993 = v1963;
                    v1995 = v2;
                    v1997 = v1964;
                }
                v1978 = v1979;
                v1980 = v1981;
                v1982 = v1983;
                v1984 = v1985;
                v1986 = v1987;
                v1988 = v1989;
                v1990 = v1991;
                v1992 = v1993;
                v1994 = v1995;
                v1996 = v1997;
                v1998 = v0;
                v2000 = v0;
                v2002 = v0;
                v2004 = v0;
                v2006 = v0;
                v2008 = v0;
            } else {
                let v1999: f64;
                let v2001: f64;
                let v2003: f64;
                let v2005: f64;
                let v2007: f64;
                let v2009: f64;
                if v621 != 0.0 {
                    let v1965 = v1851 * v20;
                    let v1966 = v1853 * v20;
                    v1999 = v2;
                    v2001 = v1965;
                    v2003 = v2;
                    v2005 = v1966;
                    v2007 = v0;
                    v2009 = v0;
                } else {
                    let v1967 = v1851 * v20;
                    v1999 = v0;
                    v2001 = v0;
                    v2003 = v0;
                    v2005 = v0;
                    v2007 = v2;
                    v2009 = v1967;
                }
                v1978 = v0;
                v1980 = v0;
                v1982 = v0;
                v1984 = v0;
                v1986 = v0;
                v1988 = v0;
                v1990 = v0;
                v1992 = v0;
                v1994 = v0;
                v1996 = v0;
                v1998 = v1999;
                v2000 = v2001;
                v2002 = v2003;
                v2004 = v2005;
                v2006 = v2007;
                v2008 = v2009;
            }
            let v1972 = if ((((v1823 + v1828) + v1833) + v1835) + v1842) == v0 { 1.0 } else { 0.0 };
            if v1972 != 0.0 {
            } else {
            }
            let v1973 = if v20 != v2 { 1.0 } else { 0.0 };
            if v1973 != 0.0 {
            } else {
            }
        {
            let psd = v1938;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 0, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 0, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 0, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(0, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = v1939;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 1, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(1, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = v1940;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 2, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 2, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 2, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(2, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = v1941;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 3, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 3, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 3, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(3, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = v1942;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 4, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 4, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 4, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(4, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = v1943;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 5, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 5, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 5, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(5, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = v1945;
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
            let psd = v1947;
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
            let psd = v1948;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 8, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 8, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 8, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(8, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = v1949;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 9, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 9, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 9, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(9, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = v1951;
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
            let psd = v1952;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 11, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 11, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 11, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(11, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = v1954;
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
            let psd = v1955;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 13, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 13, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 13, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(13, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = v1957;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 14, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 14, value: psd }); }
            let exponent: Option<f64> = Some(v2);
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 14, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 14, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(14, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v1974 == 0.0 {
            if !visitor.visit(15, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v1975;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 15, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 15, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 15, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(15, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v1976 == 0.0 {
            if !visitor.visit(16, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v1977;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 16, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 16, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 16, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(16, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v1978 == 0.0 {
            if !visitor.visit(17, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v1980;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 17, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 17, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 17, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(17, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v1982 == 0.0 {
            if !visitor.visit(18, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v1984;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 18, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 18, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 18, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(18, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v1986 == 0.0 {
            if !visitor.visit(19, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v1988;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 19, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 19, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 19, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(19, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v1990 == 0.0 {
            if !visitor.visit(20, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v1992;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 20, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 20, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 20, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(20, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v1994 == 0.0 {
            if !visitor.visit(21, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v1996;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 21, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 21, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 21, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(21, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v1998 == 0.0 {
            if !visitor.visit(22, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v2000;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 22, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 22, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 22, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(22, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v2002 == 0.0 {
            if !visitor.visit(23, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v2004;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 23, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 23, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 23, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(23, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v2006 == 0.0 {
            if !visitor.visit(24, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v2008;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 24, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 24, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 24, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(24, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        Ok(())
    }
}
