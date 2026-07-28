#![allow(dead_code, non_snake_case, unused_parens, unused_variables)]

use super::state::Instance;
use crate::device::veriloga_generated::GeneratedEvalContext;
pub use crate::device::veriloga_generated::{GeneratedNoiseDescriptor, GeneratedNoiseEndpoint, GeneratedNoiseEvaluation, GeneratedNoiseEvaluationError, GeneratedNoiseEvaluationRef, GeneratedNoiseKind, GeneratedNoiseVisitor};

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
            let v16 = parameters[141];
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
            let v88 = node_potentials[3];
            let v94 = parameters[124];
            let v103 = 8.617086918058125e-5f64;
            let v151 = 3e0f64;
            let v152 = -3e0f64;
            let v158 = parameters[104];
            let v175 = -3e0f64;
            let v178 = parameters[63];
            let v181 = parameters[109];
            let v198 = -3e0f64;
            let v201 = parameters[79];
            let v219 = -3e0f64;
            let v239 = -3e0f64;
            let v258 = -3e0f64;
            let v261 = parameters[26];
            let v264 = parameters[108];
            let v289 = parameters[74];
            let v297 = parameters[69];
            let v300 = parameters[53];
            let v301 = parameters[96];
            let v306 = parameters[55];
            let v307 = parameters[97];
            let v308 = parameters[95];
            let v313 = parameters[54];
            let v314 = parameters[100];
            let v319 = parameters[56];
            let v320 = parameters[101];
            let v324 = parameters[57];
            let v325 = parameters[103];
            let v329 = parameters[58];
            let v331 = parameters[59];
            let v332 = parameters[98];
            let v336 = parameters[121];
            let v338 = parameters[9];
            let v357 = 6.931471805599453e-4f64;
            let v359 = parameters[122];
            let v361 = parameters[10];
            let v380 = 6.931471805599453e-4f64;
            let v382 = parameters[42];
            let v383 = parameters[123];
            let v387 = 1e-6f64;
            let v390 = 5e-1f64;
            let v391 = 5e-7f64;
            let v400 = parameters[8];
            let v401 = 4e0f64;
            let v404 = parameters[120];
            let v416 = parameters[11];
            let v421 = parameters[29];
            let v422 = parameters[102];
            let v427 = parameters[19];
            let v428 = 6e0f64;
            let v429 = parameters[20];
            let v435 = parameters[112];
            let v441 = parameters[30];
            let v442 = parameters[31];
            let v453 = parameters[15];
            let v457 = parameters[16];
            let v461 = parameters[110];
            let v467 = parameters[17];
            let v468 = parameters[18];
            let v475 = parameters[23];
            let v477 = parameters[24];
            let v478 = parameters[106];
            let v484 = parameters[27];
            let v485 = parameters[105];
            let v490 = parameters[25];
            let v491 = parameters[107];
            let v497 = parameters[28];
            let v503 = parameters[111];
            let v508 = parameters[21];
            let v509 = parameters[22];
            let v518 = parameters[136];
            let v519 = parameters[137];
            let v527 = parameters[142];
            let v530 = parameters[144];
            let v536 = -5e-1f64;
            let v539 = parameters[34];
            let v548 = parameters[33];
            let v560 = -5e-1f64;
            let v563 = parameters[36];
            let v572 = parameters[35];
            let v584 = parameters[13];
            let v587 = parameters[12];
            let v590 = parameters[86];
            let v596 = parameters[87];
            let v601 = parameters[88];
            let v606 = parameters[89];
            let v607 = parameters[99];
            let v612 = 3e2f64;
            let v614 = 5.25e2f64;
            let v617 = 7.2e-4f64;
            let v620 = 1.6e-6f64;
            let v625 = 1.081e0f64;
            let v627 = parameters[91];
            let v629 = parameters[133];
            let v630 = parameters[135];
            let v640 = node_potentials[6];
            let v641 = node_potentials[7];
            let v644 = node_potentials[8];
            let v647 = node_potentials[4];
            let v650 = node_potentials[5];
            let v657 = node_potentials[2];
            let v658 = node_potentials[1];
            let v663 = node_potentials[0];
            let v666 = node_potentials[10];
            let v669 = node_potentials[9];
            let v681 = parameters[138];
            let v756 = parameters[140];
            let v767 = 1e2f64;
            let v783 = 2e-1f64;
            let v798 = parameters[61];
            let v799 = parameters[60];
            let v809 = parameters[62];
            let v824 = -1e0f64;
            let v867 = parameters[139];
            let v885 = parameters[72];
            let v901 = 1e-5f64;
            let v905 = 1e-40f64;
            let v921 = -1e0f64;
            let v952 = parameters[73];
            let v960 = -1e0f64;
            let v984 = parameters[75];
            let v1039 = 1.0000000000000002e-2f64;
            let v1043 = 5.000000000000001e-3f64;
            let v1057 = parameters[14];
            let v1063 = 1e-4f64;
            let v1077 = parameters[143];
            let v1088 = parameters[145];
            let v1103 = parameters[146];
            let v1126 = 1e3f64;
            let v1128 = 4e1f64;
            let v1131 = 2.3538526683702e17f64;
            let v1159 = parameters[92];
            let v1261 = 1e-30f64;
            let v1264 = -2e0f64;
            let v1280 = 1.6666666666666666e-1f64;
            let v1286 = -1e-3f64;
            let v1302 = 3.333333333333333e-1f64;
            let v1304 = 2.5e-1f64;
            let v1339 = -2e0f64;
            let v1360 = -1e-3f64;
            let v1401 = parameters[5];
            let v1424 = 1.21e-2f64;
            let v1427 = 6.05e-3f64;
            let v1443 = parameters[83];
            let v1446 = 1e-6f64;
            let v1447 = 1e-12f64;
            let v1448 = -1e0f64;
            let v1450 = -1e0f64;
            let v1453 = -1e0f64;
            let v1456 = 5e-13f64;
            let v1459 = -1e0f64;
            let v1465 = -1e0f64;
            let v1469 = parameters[81];
            let v1473 = parameters[80];
            let v1503 = 1.0000000000000002e-2f64;
            let v1506 = 5.000000000000001e-3f64;
            let v1523 = parameters[38];
            let v1525 = parameters[43];
            let v1528 = parameters[41];
            let v1541 = parameters[40];
            let v1550 = parameters[39];
            let v1557 = parameters[45];
            let v1559 = parameters[44];
            let v1567 = parameters[7];
            let v1587 = parameters[46];
            let v1617 = 1e-7f64;
            let v1643 = parameters[47];
            let v1647 = parameters[48];
            let v1651 = parameters[51];
            let v1655 = parameters[50];
            let v1670 = parameters[49];
            let v1694 = parameters[52];
            let v1747 = parameters[76];
            let v1781 = parameters[84];
            let v1788 = parameters[78];
            let v1792 = parameters[90];
            let v1840 = parameters[6];
            let v1848 = parameters[132];
            let v1852 = parameters[68];
            let v1857 = parameters[77];
            let v1869 = 5.5224904e-23f64;
            let v1878 = 5e0f64;
            let v1884 = 3.2043836e-19f64;
            let v1887 = parameters[129];
            let v1891 = 3.2043836e-19f64;
            let v1897 = parameters[130];
            let v1900 = 3.2043836e-19f64;
            let v1907 = parameters[127];
            let v1909 = parameters[125];
            let v1916 = parameters[128];
            let v1918 = parameters[126];
            let v1923 = 3.2043836e-19f64;
            let v1926 = 3.2043836e-19f64;
            let v1933 = 3.2043836e-19f64;
            let v1936 = 3.2043836e-19f64;
            let v1947 = 3.2043836e-19f64;
            let v3 = if v1 == v2 { 1.0 } else { 0.0 };
            let v616: f64;
            let v1620: f64;
            if v3 != 0.0 {
                v616 = v5;
                v1620 = v4;
            } else {
                v616 = v7;
                v1620 = v6;
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
            let v111: f64;
            if v41 != 0.0 {
                let v46 = v37 + (v39 * ((v2 + (v40.exp())).ln()));
                v111 = v46;
            } else {
                let v52 = v36 + (v39 * ((v2 + ((-v40).exp())).ln()));
                v111 = v52;
            }
            let v53 = v2 / v29;
            let v55 = v2 / v54;
            let v59 = v24.powf((v24 - v57));
            let v60 = v2 / v59;
            let v68 = v61 + (((v62 * v12) * v12) / (v12 + v65));
            let v70 = (v68 - v37) / v39;
            let v71 = if v68 < v37 { 1.0 } else { 0.0 };
            let v131: f64;
            if v71 != 0.0 {
                let v76 = v37 + (v39 * ((v2 + (v70.exp())).ln()));
                v131 = v76;
            } else {
                let v82 = v68 + (v39 * ((v2 + ((-v70).exp())).ln()));
                v131 = v82;
            }
            let v83 = v2 / v61;
            let v84 = v2 / v56;
            let v87 = v2 - (v2 / v85);
            let v89 = if v88 < v0 { 1.0 } else { 0.0 };
            let v93: f64;
            if v89 != 0.0 {
                let v92 = -((v2 - v88).ln());
                v93 = v92;
            } else {
                v93 = v88;
            }
            let v95 = if v93 < v94 { 1.0 } else { 0.0 };
            let v100: f64;
            if v95 != 0.0 {
                v100 = v93;
            } else {
                let v99 = v94 + ((v2 + (v93 - v94)).ln());
                v100 = v99;
            }
            let v101 = v15 + v100;
            let v102 = v101 / v12;
            let v104 = v103 * v101;
            let v106 = v2 / v104;
            let v108 = v106 - (v2 / (v103 * v12));
            let v109 = v101 - v12;
            let v110 = v102.ln();
            let v116 = v111 - (((v30 * v101) * v101) / (v101 + v33));
            let v118 = (v116 - v37) / v39;
            let v119 = if v116 < v37 { 1.0 } else { 0.0 };
            let v534: f64;
            if v119 != 0.0 {
                let v124 = v37 + (v39 * ((v2 + (v118.exp())).ln()));
                v534 = v124;
            } else {
                let v130 = v116 + (v39 * ((v2 + ((-v118).exp())).ln()));
                v534 = v130;
            }
            let v136 = v131 - (((v62 * v101) * v101) / (v101 + v65));
            let v138 = (v136 - v37) / v39;
            let v139 = if v136 < v37 { 1.0 } else { 0.0 };
            let v558: f64;
            if v139 != 0.0 {
                let v144 = v37 + (v39 * ((v2 + (v138.exp())).ln()));
                v558 = v144;
            } else {
                let v150 = v136 + (v39 * ((v2 + ((-v138).exp())).ln()));
                v558 = v150;
            }
            let v157 = v2 - v102;
            let v160 = (((v152 * v104) * v110) + (v54 * v102)) + (v157 * v158);
            let v162 = (v37 - v160) / v104;
            let v163 = if v37 < v160 { 1.0 } else { 0.0 };
            let v281: f64;
            if v163 != 0.0 {
                let v168 = v160 + (v104 * ((v2 + (v162.exp())).ln()));
                v281 = v168;
            } else {
                let v174 = v37 + (v104 * ((v2 + ((-v162).exp())).ln()));
                v281 = v174;
            }
            let v182 = v157 * v181;
            let v183 = (((v175 * v104) * v110) + (v178 * v102)) + v182;
            let v185 = (v37 - v183) / v104;
            let v186 = if v37 < v183 { 1.0 } else { 0.0 };
            let v717: f64;
            if v186 != 0.0 {
                let v191 = v183 + (v104 * ((v2 + (v185.exp())).ln()));
                v717 = v191;
            } else {
                let v197 = v37 + (v104 * ((v2 + ((-v185).exp())).ln()));
                v717 = v197;
            }
            let v204 = (((v198 * v104) * v110) + (v201 * v102)) + v182;
            let v206 = (v37 - v204) / v104;
            let v207 = if v37 < v204 { 1.0 } else { 0.0 };
            let v1790: f64;
            if v207 != 0.0 {
                let v212 = v204 + (v104 * ((v2 + (v206.exp())).ln()));
                v1790 = v212;
            } else {
                let v218 = v37 + (v104 * ((v2 + ((-v206).exp())).ln()));
                v1790 = v218;
            }
            let v222 = v56 * v102;
            let v224 = (((v219 * v104) * v110) + v222) + v182;
            let v226 = (v37 - v224) / v104;
            let v227 = if v37 < v224 { 1.0 } else { 0.0 };
            let v291: f64;
            if v227 != 0.0 {
                let v232 = v224 + (v104 * ((v2 + (v226.exp())).ln()));
                v291 = v232;
            } else {
                let v238 = v37 + (v104 * ((v2 + ((-v226).exp())).ln()));
                v291 = v238;
            }
            let v243 = (((v239 * v104) * v110) + v222) + v182;
            let v245 = (v37 - v243) / v104;
            let v246 = if v37 < v243 { 1.0 } else { 0.0 };
            let v283: f64;
            if v246 != 0.0 {
                let v251 = v243 + (v104 * ((v2 + (v245.exp())).ln()));
                v283 = v251;
            } else {
                let v257 = v37 + (v104 * ((v2 + ((-v245).exp())).ln()));
                v283 = v257;
            }
            let v266 = (((v258 * v104) * v110) + (v261 * v102)) + (v157 * v264);
            let v268 = (v37 - v266) / v104;
            let v269 = if v37 < v266 { 1.0 } else { 0.0 };
            let v1116: f64;
            if v269 != 0.0 {
                let v274 = v266 + (v104 * ((v2 + (v268.exp())).ln()));
                v1116 = v274;
            } else {
                let v280 = v37 + (v104 * ((v2 + ((-v268).exp())).ln()));
                v1116 = v280;
            }
            let v282 = v2 / v281;
            let v284 = v2 / v283;
            let v286 = (v54 * v282).powf(v25);
            let v288 = (v56 * v284).powf(v57);
            let v295 = ((v2 - v289) * ((v56 / v291).powf(v57))) + v289;
            let v296 = v2 / v295;
            let v298 = v297 * v295;
            let v299 = v289 * v296;
            let v304 = v300 * ((v110 * v301).exp());
            let v305 = if v304 < v21 { 1.0 } else { 0.0 };
            let v1703: f64;
            if v305 != 0.0 {
                v1703 = v21;
            } else {
                v1703 = v304;
            }
            let v312 = v306 * ((v110 * (v307 - v308)).exp());
            let v317 = v313 * ((v110 * v314).exp());
            let v318 = if v317 < v21 { 1.0 } else { 0.0 };
            let v1696: f64;
            if v318 != 0.0 {
                v1696 = v21;
            } else {
                v1696 = v317;
            }
            let v323 = v319 * ((v110 * v320).exp());
            let v327 = (v110 * v325).exp();
            let v328 = v324 * v327;
            let v330 = v329 * v327;
            let v335 = v331 * ((v110 * v332).exp());
            let v337 = if v336 != v0 { 1.0 } else { 0.0 };
            let v407: f64;
            if v337 != 0.0 {
                let v341 = v338 * (v2 + (v109 * v336));
                let v343 = (v341 - v2) / v23;
                let v344 = if v341 < v2 { 1.0 } else { 0.0 };
                let v356: f64;
                if v344 != 0.0 {
                    let v349 = v2 + (v23 * ((v2 + (v343.exp())).ln()));
                    v356 = v349;
                } else {
                    let v355 = v341 + (v23 * ((v2 + ((-v343).exp())).ln()));
                    v356 = v355;
                }
                let v358 = v356 - v357;
                v407 = v358;
            } else {
                v407 = v338;
            }
            let v360 = if v359 != v0 { 1.0 } else { 0.0 };
            let v1011: f64;
            if v360 != 0.0 {
                let v364 = v361 * (v2 + (v109 * v359));
                let v366 = (v364 - v2) / v23;
                let v367 = if v364 < v2 { 1.0 } else { 0.0 };
                let v379: f64;
                if v367 != 0.0 {
                    let v372 = v2 + (v23 * ((v2 + (v366.exp())).ln()));
                    v379 = v372;
                } else {
                    let v378 = v364 + (v23 * ((v2 + ((-v366).exp())).ln()));
                    v379 = v378;
                }
                let v381 = v379 - v380;
                v1011 = v381;
            } else {
                v1011 = v361;
            }
            let v386 = v382 * (v2 + (v383 * v109));
            let v388 = v386 * v386;
            let v389 = if v386 < v0 { 1.0 } else { 0.0 };
            let v1539: f64;
            if v389 != 0.0 {
                let v395 = v391 / (((v388 + v387).sqrt()) - v386);
                v1539 = v395;
            } else {
                let v399 = v390 * (((v388 + v387).sqrt()) + v386);
                v1539 = v399;
            }
            let v415 = (v400 * (((v110 * (((v401 - v307) - v308) + v404)) / v407).exp())) * ((((-v158) * v108) / v407).exp());
            let v420 = v416 * ((v110 * (v2 - v307)).exp());
            let v426 = v421 * ((v110 * (v2 - v422)).exp());
            let v437 = (-v435) * v108;
            let v440 = (v427 * ((v110 * (v428 - (v24 * v429))).exp())) * ((v437 / v429).exp());
            let v452 = (v441 * ((v110 * (v428 - (v24 * v442))).exp())) * ((((-v181) * v108) / v442).exp());
            let v456 = v110 * ((v401 - v301) + v404);
            let v463 = (-v461) * v108;
            let v466 = (v453 * ((v456 / v457).exp())) * ((v463 / v457).exp());
            let v474 = (v467 * ((v456 / v468).exp())) * ((v463 / v468).exp());
            let v476 = if v475 == v2 { 1.0 } else { 0.0 };
            let v1138: f64;
            let v1151: f64;
            let v1193: f64;
            if v476 != 0.0 {
                let v483 = v477 * ((((-v478) * v108) / v457).exp());
                let v489 = v484 * (((-v485) * v108).exp());
                let v496 = v490 * ((((-v491) * v108) / v468).exp());
                v1138 = v483;
                v1151 = v489;
                v1193 = v496;
            } else {
                v1138 = v0;
                v1151 = v0;
                v1193 = v0;
            }
            let v507 = (v497 * ((v110 * ((v401 - v422) + v404)).exp())) * (((-v503) * v108).exp());
            let v517 = (v508 * ((v110 * (v428 - (v24 * v509))).exp())) * ((v437 / v509).exp());
            let v526 = (v518 * ((v110 * (v401 / v519)).exp())) * ((v437 / v519).exp());
            let v533 = (v527 * (v102.sqrt())) * ((v530 * v109).exp());
            let v537 = (v534 * v53).powf(v536);
            let v538 = v2 / v286;
            let v547 = (((((((v539 * v534) * v534) * v537) * v538) * v54) * v282) * v53) * v53;
            let v557 = ((((((v548 * v537) * v281) * v281) * v55) * v55) * v286) * ((v539 - v547).exp());
            let v561 = (v558 * v83).powf(v560);
            let v571 = (((((((v563 * v558) * v558) * v561) * (v2 / v288)) * v56) * v284) * v83) * v83;
            let v581 = ((((((v572 * v561) * v283) * v283) * v84) * v84) * v288) * ((v563 - v571).exp());
            let v583 = (v110 * v308).exp();
            let v586 = (v584 * v583) * v296;
            let v589 = (v587 * v583) * v538;
            let v595 = v590 * ((v110 * ((v308 + v307) - v2)).exp());
            let v600 = v596 * ((v110 * (v332 - v2)).exp());
            let v602 = v595 + v600;
            let v605 = (v601 * v602) / (v590 + v596);
            let v611 = v606 * ((v110 * (v607 - v2)).exp());
            let v613 = v101 - v612;
            let v615 = if v101 < v614 { 1.0 } else { 0.0 };
            let v1621: f64;
            if v615 != 0.0 {
                let v624 = v616 * ((v2 + (v617 * v613)) - ((v620 * v613) * v613));
                v1621 = v624;
            } else {
                let v626 = v616 * v625;
                v1621 = v626;
            }
            let v628 = v627 * v583;
            let v631 = if v319 > v0 { 1.0 } else { 0.0 };
            let v1733: f64;
            if v631 != 0.0 {
                let v632 = v2 / v323;
                let v633 = if v632 > v22 { 1.0 } else { 0.0 };
                let v1734: f64;
                if v633 != 0.0 {
                    v1734 = v22;
                } else {
                    v1734 = v632;
                }
                v1733 = v1734;
            } else {
                v1733 = v0;
            }
            let v634 = if v324 > v0 { 1.0 } else { 0.0 };
            let v1735: f64;
            if v634 != 0.0 {
                let v635 = v2 / v328;
                let v636 = if v635 > v22 { 1.0 } else { 0.0 };
                let v1736: f64;
                if v636 != 0.0 {
                    v1736 = v22;
                } else {
                    v1736 = v635;
                }
                v1735 = v1736;
            } else {
                v1735 = v0;
            }
            let v637 = if v329 > v0 { 1.0 } else { 0.0 };
            let v1737: f64;
            if v637 != 0.0 {
                let v638 = v2 / v330;
                let v639 = if v638 > v22 { 1.0 } else { 0.0 };
                let v1738: f64;
                if v639 != 0.0 {
                    v1738 = v22;
                } else {
                    v1738 = v638;
                }
                v1737 = v1738;
            } else {
                v1737 = v0;
            }
            let v643 = v1 * (v640 - v641);
            let v646 = v1 * (v640 - v644);
            let v649 = v1 * (v640 - v647);
            let v652 = v1 * (v650 - v647);
            let v654 = v1 * (v650 - v640);
            let v656 = v1 * (v641 - v644);
            let v660 = v1 * (v658 - v650);
            let v662 = v1 * (v658 - v657);
            let v665 = v1 * (v658 - v663);
            let v674 = ((v654 + v646) - v656) - (v1 * (v666 - v641));
            let v679 = v665 + ((((-v665) + v660) + v674) - (v1 * (v669 - v666)));
            let v680 = v646 * v106;
            let v682 = if v680 < v681 { 1.0 } else { 0.0 };
            let v899: f64;
            if v682 != 0.0 {
                let v683 = v680.exp();
                v899 = v683;
            } else {
                let v687 = (v681.exp()) * (v2 + (v680 - v681));
                v899 = v687;
            }
            let v688 = v649 * v106;
            let v689 = v688 / v407;
            let v690 = if v689 < v681 { 1.0 } else { 0.0 };
            let v1004: f64;
            if v690 != 0.0 {
                let v691 = v689.exp();
                v1004 = v691;
            } else {
                let v695 = (v681.exp()) * (v2 + (v689 - v681));
                v1004 = v695;
            }
            let v696 = v674 * v106;
            let v697 = if v696 < v681 { 1.0 } else { 0.0 };
            let v1390: f64;
            if v697 != 0.0 {
                let v698 = v696.exp();
                v1390 = v698;
            } else {
                let v702 = (v681.exp()) * (v2 + (v696 - v681));
                v1390 = v702;
            }
            let v703 = v654 * v106;
            let v704 = if v703 < v681 { 1.0 } else { 0.0 };
            let v1521: f64;
            if v704 != 0.0 {
                let v705 = v703.exp();
                v1521 = v705;
            } else {
                let v709 = (v681.exp()) * (v2 + (v703 - v681));
                v1521 = v709;
            }
            let v710 = v679 * v106;
            let v711 = if v710 < v681 { 1.0 } else { 0.0 };
            let v1408: f64;
            if v711 != 0.0 {
                let v712 = v710.exp();
                v1408 = v712;
            } else {
                let v716 = (v681.exp()) * (v2 + (v710 - v681));
                v1408 = v716;
            }
            let v719 = (v679 - v717) * v106;
            let v720 = if v719 < v681 { 1.0 } else { 0.0 };
            let v1806: f64;
            if v720 != 0.0 {
                let v721 = v719.exp();
                v1806 = v721;
            } else {
                let v725 = (v681.exp()) * (v2 + (v719 - v681));
                v1806 = v725;
            }
            let v728 = if ((v674 - v717) * v106) < v681 { 1.0 } else { 0.0 };
            if v728 != 0.0 {
            } else {
            }
            let v730 = (v646 - v717) * v106;
            let v731 = if v730 < v681 { 1.0 } else { 0.0 };
            let v745: f64;
            if v731 != 0.0 {
                let v732 = v730.exp();
                v745 = v732;
            } else {
                let v736 = (v681.exp()) * (v2 + (v730 - v681));
                v745 = v736;
            }
            let v738 = (v643 - v717) * v106;
            let v739 = if v738 < v681 { 1.0 } else { 0.0 };
            let v749: f64;
            if v739 != 0.0 {
                let v740 = v738.exp();
                v749 = v740;
            } else {
                let v744 = (v681.exp()) * (v2 + (v738 - v681));
                v749 = v744;
            }
            let v748 = (v2 + (v401 * v745)).sqrt();
            let v752 = (v2 + (v401 * v749)).sqrt();
            let v754 = v2 + v752;
            let v755 = (v24 * v749) / v754;
            let v757 = if v755 < v756 { 1.0 } else { 0.0 };
            let v844: f64;
            if v757 != 0.0 {
                v844 = v756;
            } else {
                v844 = v755;
            }
            let v759 = v748 + v2;
            let v763 = v104 * ((v748 - v752) - ((v759 / v754).ln()));
            let v765 = (v763 + v656) / v335;
            let v766 = if v765 > v0 { 1.0 } else { 0.0 };
            let v955: f64;
            let v968: f64;
            let v983: f64;
            let v1010: f64;
            let v1569: f64;
            let v1605: f64;
            if v766 != 0.0 {
                let v768 = if v643 < v767 { 1.0 } else { 0.0 };
                let v781: f64;
                if v768 != 0.0 {
                    v781 = v643;
                } else {
                    let v772 = v767 + ((v2 + (v643 - v767)).ln());
                    v781 = v772;
                }
                let v775 = (v390 * v765) * v335;
                let v782 = (v717 + ((v24 * v104) * (((v775 * v106) + v2).ln()))) - v781;
                let v784 = v783 * v717;
                let v785 = v784 * v784;
                let v786 = v782 * v782;
                let v787 = if v782 < v0 { 1.0 } else { 0.0 };
                let v797: f64;
                if v787 != 0.0 {
                    let v792 = (v390 * v785) / (((v786 + v785).sqrt()) - v782);
                    v797 = v792;
                } else {
                    let v796 = v390 * (((v786 + v785).sqrt()) + v782);
                    v797 = v796;
                }
                let v800 = v798 * v799;
                let v806 = (v797 * (v797 + v800)) / (v799 * (v797 + (v798 * v335)));
                let v807 = v765 / v806;
                let v810 = (v807 - v2) / v809;
                let v811 = if v807 < v2 { 1.0 } else { 0.0 };
                let v823: f64;
                if v811 != 0.0 {
                    let v816 = v2 + (v809 * ((v2 + (v810.exp())).ln()));
                    v823 = v816;
                } else {
                    let v822 = v807 + (v809 * ((v2 + ((-v810).exp())).ln()));
                    v823 = v822;
                }
                let v831 = v823 / (v2 + (v809 * ((v2 + ((v824 / v809).exp())).ln())));
                let v832 = v797 / v800;
                let v835 = v2 + v832;
                let v842 = (v2 + ((v2 + (((v401 * v831) * v832) * v835)).sqrt())) / ((v24 * v831) * v835);
                let v845 = v844 * v842;
                let v848 = ((v2 - v842) + v845) / (v2 + v845);
                let v850 = (v775 * v848) * v106;
                let v855 = (v24 * v850) + (v844 * ((v844 + v850) + v2));
                let v857 = v390 * (v850 - v2);
                let v859 = (v857 * v857) + v855;
                let v860 = if v850 >= v2 { 1.0 } else { 0.0 };
                let v866: f64;
                if v860 != 0.0 {
                    let v862 = v857 + (v859.sqrt());
                    v866 = v862;
                } else {
                    let v865 = v855 / ((v859.sqrt()) - v857);
                    v866 = v865;
                }
                let v868 = if v866 < v867 { 1.0 } else { 0.0 };
                let v869: f64;
                if v868 != 0.0 {
                    v869 = v867;
                } else {
                    v869 = v866;
                }
                let v874 = (v869 * (v869 + v2)) * ((v717 * v106).exp());
                let v877 = (v390 * v799) * (v765 - v798);
                let v884 = v877 + (((v877 * v877) + (((v799 * v335) * v798) * v765)).sqrt());
                let v886 = if v885 == v0 { 1.0 } else { 0.0 };
                let v969: f64;
                if v886 != 0.0 {
                    let v887 = v291 * v39;
                    v969 = v887;
                } else {
                    let v892 = v291 * (v39 + ((v24 * v765) / (v765 + v806)));
                    v969 = v892;
                }
                let v894 = v798 + v765;
                let v895 = (v798 * v765) / v894;
                let v896 = v798 / v894;
                v955 = v884;
                v968 = v969;
                v983 = v896;
                v1010 = v874;
                v1569 = v848;
                v1605 = v895;
            } else {
                let v898 = (v24 * v745) / v759;
                let v910 = if (if (v656.abs()) < (v901 * v104) { 1.0 } else { 0.0 }) != 0.0 || (if (v763.abs()) < ((v905 * v104) * (v748 + v752)) { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v1570: f64;
                if v910 != 0.0 {
                    let v912 = v390 * (v898 + v844);
                    let v914 = v912 / (v912 + v2);
                    v1570 = v914;
                } else {
                    let v917 = v763 / ((v763 + v646) - v643);
                    v1570 = v917;
                }
                let v918 = v39 * v291;
                let v920 = v2 - (v765 / v798);
                v955 = v656;
                v968 = v918;
                v983 = v920;
                v1010 = v899;
                v1569 = v1570;
                v1605 = v765;
            }
            let v925 = v281 * (v2 - (v151.powf((v921 / v25))));
            let v926 = v39 * v281;
            let v928 = (v649 - v925) / v926;
            let v929 = if v649 < v925 { 1.0 } else { 0.0 };
            let v941: f64;
            if v929 != 0.0 {
                let v934 = v649 - (v926 * ((v2 + (v928.exp())).ln()));
                v941 = v934;
            } else {
                let v940 = v925 - (v926 * ((v2 + ((-v928).exp())).ln()));
                v941 = v940;
            }
            let v944 = v2 - v25;
            let v945 = (v2 - (v941 * v282)).powf(v944);
            let v951 = ((v281 / v944) * (v2 - v945)) + (v151 * (v649 - v941));
            let v953 = if v952 == v2 { 1.0 } else { 0.0 };
            let v965: f64;
            if v953 != 0.0 {
                v965 = v643;
            } else {
                let v954 = if v952 == v24 { 1.0 } else { 0.0 };
                let v966: f64;
                if v954 != 0.0 {
                    let v956 = v643 + v955;
                    v966 = v956;
                } else {
                    v966 = v646;
                }
                v965 = v966;
            }
            let v958 = v2 - v299;
            let v959 = (v24 - v299) / v958;
            let v964 = v291 * (v2 - (v959.powf((v960 / v57))));
            let v970 = (v965 - v964) / v968;
            let v971 = if v965 < v964 { 1.0 } else { 0.0 };
            let v988: f64;
            if v971 != 0.0 {
                let v976 = v965 - (v968 * ((v2 + (v970.exp())).ln()));
                v988 = v976;
            } else {
                let v982 = v964 - (v968 * ((v2 + ((-v970).exp())).ln()));
                v988 = v982;
            }
            let v985 = v983.powf(v984);
            let v986 = v2 - v57;
            let v987 = v291 / v986;
            let v1001 = (v958 * ((v987 * (v2 - (v985 * ((v2 - (v988 / v291)).powf(v986))))) + ((v985 * v959) * (v965 - v988)))) + (v299 * v643);
            let v1003 = (v401 * v415) / v420;
            let v1005 = v1003 * v1004;
            let v1009 = v1005 / (v2 + ((v2 + v1005).sqrt()));
            let v1013 = v1010.powf((v2 / v1011));
            let v1014 = v1003 * v1013;
            let v1018 = v1014 / (v2 + ((v2 + v1014).sqrt()));
            let v1019 = if v627 == v0 { 1.0 } else { 0.0 };
            let v1040: f64;
            if v1019 != 0.0 {
                let v1023 = (v2 + (v951 / v589)) + (v1001 / v586);
                v1040 = v1023;
            } else {
                let v1038 = ((((((v951 / v589) + v2) * v628) * v106).exp()) - (((((-v1001) / v586) * v628) * v106).exp())) / (((v628 * v106).exp()) - v2);
                v1040 = v1038;
            }
            let v1041 = v1040 * v1040;
            let v1042 = if v1040 < v0 { 1.0 } else { 0.0 };
            let v1052: f64;
            if v1042 != 0.0 {
                let v1047 = v1043 / (((v1041 + v1039).sqrt()) - v1040);
                v1052 = v1047;
            } else {
                let v1051 = v390 * (((v1041 + v1039).sqrt()) + v1040);
                v1052 = v1051;
            }
            let v1055 = v2 + (v390 * (v1009 + v1018));
            let v1056 = v1052 * v1055;
            let v1059 = (v1057 * v415) * v1013;
            let v1060 = v415 * v1004;
            let v1062 = (v1060 - v1059) / v1056;
            let v1064 = v649 / v1063;
            let v1065 = if v649 < v0 { 1.0 } else { 0.0 };
            let v1076: f64;
            if v1065 != 0.0 {
                let v1069 = v1063 * ((v2 + (v1064.exp())).ln());
                v1076 = v1069;
            } else {
                let v1075 = v649 + (v1063 * ((v2 + ((-v1064).exp())).ln()));
                v1076 = v1075;
            }
            let v1078 = v1076 / v1077;
            let v1079 = if v1078 < v681 { 1.0 } else { 0.0 };
            let v1085: f64;
            if v1079 != 0.0 {
                let v1080 = v1078.exp();
                v1085 = v1080;
            } else {
                let v1084 = (v681.exp()) * (v2 + (v1078 - v681));
                v1085 = v1084;
            }
            let v1087 = v533 * (v1085 - v2);
            let v1090 = (v649 - v1088) / v23;
            let v1091 = if v649 < v1088 { 1.0 } else { 0.0 };
            let v1104: f64;
            if v1091 != 0.0 {
                let v1096 = v649 - (v23 * ((v2 + (v1090.exp())).ln()));
                v1104 = v1096;
            } else {
                let v1102 = v1088 - (v23 * ((v2 + ((-v1090).exp())).ln()));
                v1104 = v1102;
            }
            let v1106 = v1088 - v1104;
            let v1108 = (v1103 * v1104) * (v1106 * v1106);
            let v1109 = v688 / v457;
            let v1110 = if v1109 < v681 { 1.0 } else { 0.0 };
            let v1135: f64;
            if v1110 != 0.0 {
                let v1111 = v1109.exp();
                v1135 = v1111;
            } else {
                let v1115 = (v681.exp()) * (v2 + (v1109 - v681));
                v1135 = v1115;
            }
            let v1739: f64;
            if v476 != 0.0 {
                let v1118 = (v649 - v1116) * v106;
                let v1119 = if v1118 < v681 { 1.0 } else { 0.0 };
                let v1141: f64;
                if v1119 != 0.0 {
                    let v1120 = v1118.exp();
                    v1141 = v1120;
                } else {
                    let v1124 = (v681.exp()) * (v2 + (v1118 - v681));
                    v1141 = v1124;
                }
                let v1127 = (v1062 / v415) - v1126;
                let v1129 = if v1127 < v1128 { 1.0 } else { 0.0 };
                let v1154: f64;
                if v1129 != 0.0 {
                    let v1130 = v1127.exp();
                    v1154 = v1130;
                } else {
                    let v1134 = v1131 * (v2 + (v1127 - v1128));
                    v1154 = v1134;
                }
                let v1136 = v1135 - v2;
                let v1158 = ((v466 * v1136) + ((((v1138 * v24) * v1136) / (v2 + ((v2 + (v401 * v1141)).sqrt()))) * (v2 + (v1001 / v586)))) + (((v1151 * (v1010 - v2)) * v1154) / (v2 + v1154));
                v1739 = v1158;
            } else {
                let v1160 = if v1159 == v0 { 1.0 } else { 0.0 };
                let v1740: f64;
                if v1160 != 0.0 {
                    let v1162 = v466 * (v1135 - v2);
                    v1740 = v1162;
                } else {
                    let v1173 = v466 * (((v2 - v1159) * (v1135 - v2)) + ((v1159 * ((v1135 + v1010) - v24)) * (v2 + (v1001 / v586))));
                    v1740 = v1173;
                }
                v1739 = v1740;
            }
            let v1174 = v652 * v106;
            let v1175 = v1174 / v468;
            let v1176 = if v1175 < v681 { 1.0 } else { 0.0 };
            let v1190: f64;
            if v1176 != 0.0 {
                let v1177 = v1175.exp();
                v1190 = v1177;
            } else {
                let v1181 = (v681.exp()) * (v2 + (v1175 - v681));
                v1190 = v1181;
            }
            let v1743: f64;
            if v476 != 0.0 {
                let v1183 = (v652 - v1116) * v106;
                let v1184 = if v1183 < v681 { 1.0 } else { 0.0 };
                let v1196: f64;
                if v1184 != 0.0 {
                    let v1185 = v1183.exp();
                    v1196 = v1185;
                } else {
                    let v1189 = (v681.exp()) * (v2 + (v1183 - v681));
                    v1196 = v1189;
                }
                let v1191 = v1190 - v2;
                let v1202 = (v474 * v1191) + (((v1193 * v24) * v1191) / (v2 + ((v2 + (v401 * v1196)).sqrt())));
                v1743 = v1202;
            } else {
                let v1204 = v474 * (v1190 - v2);
                v1743 = v1204;
            }
            let v1205 = v688 / v429;
            let v1206 = if v1205 < v681 { 1.0 } else { 0.0 };
            let v1212: f64;
            if v1206 != 0.0 {
                let v1207 = v1205.exp();
                v1212 = v1207;
            } else {
                let v1211 = (v681.exp()) * (v2 + (v1205 - v681));
                v1212 = v1211;
            }
            let v1214 = v440 * (v1212 - v2);
            let v1215 = v1174 / v509;
            let v1216 = if v1215 < v681 { 1.0 } else { 0.0 };
            let v1222: f64;
            if v1216 != 0.0 {
                let v1217 = v1215.exp();
                v1222 = v1217;
            } else {
                let v1221 = (v681.exp()) * (v2 + (v1215 - v681));
                v1222 = v1221;
            }
            let v1224 = v517 * (v1222 - v2);
            let v1225 = v696 / v442;
            let v1226 = if v1225 < v681 { 1.0 } else { 0.0 };
            let v1232: f64;
            if v1226 != 0.0 {
                let v1227 = v1225.exp();
                v1232 = v1227;
            } else {
                let v1231 = (v681.exp()) * (v2 + (v1225 - v681));
                v1232 = v1231;
            }
            let v1234 = v452 * (v1232 - v2);
            let v1235 = v1174 / v519;
            let v1236 = if v1235 < v681 { 1.0 } else { 0.0 };
            let v1242: f64;
            if v1236 != 0.0 {
                let v1237 = v1235.exp();
                v1242 = v1237;
            } else {
                let v1241 = (v681.exp()) * (v2 + (v1235 - v681));
                v1242 = v1241;
            }
            let v1244 = v526 * (v1242 - v2);
            let v1248 = if (if (if v548 > v0 { 1.0 } else { 0.0 }) != 0.0 && (if v539 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && v1065 != 0.0 { 1.0 } else { 0.0 };
            let v1742: f64;
            if v1248 != 0.0 {
                let v1252 = v547 * (v2 - (v27 / (v24 * v945)));
                let v1253 = if v1252 < v681 { 1.0 } else { 0.0 };
                let v1314: f64;
                if v1253 != 0.0 {
                    let v1254 = v1252.exp();
                    v1314 = v1254;
                } else {
                    let v1258 = (v681.exp()) * (v2 + (v1252 - v681));
                    v1314 = v1258;
                }
                let v1259 = v649 * v282;
                let v1270 = v25 - v2;
                let v1285 = ((v649 * v27) * v547) / (v534 * ((((((v1259 * v1259) + v1261).sqrt()).powf((v1264 - v25))) * ((v25 * ((v2 - (v25 * v25)) - ((v151 * v1259) * v1270))) - (((v428 * v1259) * v1259) * (v1270 + v1259)))) * v1280));
                let v1287 = if v1285 < v1286 { 1.0 } else { 0.0 };
                let v1311: f64;
                if v1287 != 0.0 {
                    let v1288 = if v1285 < v681 { 1.0 } else { 0.0 };
                    let v1295: f64;
                    if v1288 != 0.0 {
                        let v1289 = v1285.exp();
                        v1295 = v1289;
                    } else {
                        let v1293 = (v681.exp()) * (v2 + (v1285 - v681));
                        v1295 = v1293;
                    }
                    let v1299 = (-v649) * (v2 + ((v2 - v1295) / v1285));
                    v1311 = v1299;
                } else {
                    let v1309 = ((v649 * v390) * v1285) * (v2 + ((v1285 * v1302) * (v2 + (v1304 * v1285))));
                    v1311 = v1309;
                }
                let v1317 = (((((v24 * v557) * v1311) * v945) * v1314) * v282) * v28;
                v1742 = v1317;
            } else {
                v1742 = v0;
            }
            let v1322 = if (if (if v572 > v0 { 1.0 } else { 0.0 }) != 0.0 && (if v563 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if v643 < v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let v1490: f64;
            if v1322 != 0.0 {
                let v1323 = v643 * v284;
                let v1325 = (v2 - v1323).powf(v986);
                let v1329 = v571 * (v2 - (v59 / (v24 * v1325)));
                let v1330 = if v1329 < v681 { 1.0 } else { 0.0 };
                let v1386: f64;
                if v1330 != 0.0 {
                    let v1331 = v1329.exp();
                    v1386 = v1331;
                } else {
                    let v1335 = (v681.exp()) * (v2 + (v1329 - v681));
                    v1386 = v1335;
                }
                let v1345 = v57 - v2;
                let v1359 = ((v643 * v59) * v571) / (v558 * ((((((v1323 * v1323) + v1261).sqrt()).powf((v1339 - v57))) * ((v57 * ((v2 - (v57 * v57)) - ((v151 * v1323) * v1345))) - (((v428 * v1323) * v1323) * (v1345 + v1323)))) * v1280));
                let v1361 = if v1359 < v1360 { 1.0 } else { 0.0 };
                let v1383: f64;
                if v1361 != 0.0 {
                    let v1362 = if v1359 < v681 { 1.0 } else { 0.0 };
                    let v1369: f64;
                    if v1362 != 0.0 {
                        let v1363 = v1359.exp();
                        v1369 = v1363;
                    } else {
                        let v1367 = (v681.exp()) * (v2 + (v1359 - v681));
                        v1369 = v1367;
                    }
                    let v1373 = (-v643) * (v2 + ((v2 - v1369) / v1359));
                    v1383 = v1373;
                } else {
                    let v1381 = ((v643 * v390) * v1359) * (v2 + ((v1359 * v1302) * (v2 + (v1304 * v1359))));
                    v1383 = v1381;
                }
                let v1389 = (((((v24 * v581) * v1383) * v1325) * v1386) * v284) * v60;
                v1490 = v1389;
            } else {
                v1490 = v0;
            }
            let v1395 = (v401 * v507) / v426;
            let v1400 = ((v24 * v507) * (v1390 - v2)) / (v2 + ((v2 + (v1395 * v1390)).sqrt()));
            let v1403 = if v8 > v0 { 1.0 } else { 0.0 };
            let v1404 = if (if v1401 > v0 { 1.0 } else { 0.0 }) != 0.0 && v1403 != 0.0 { 1.0 } else { 0.0 };
            let v1494: f64;
            let v1497: f64;
            let v1837: f64;
            if v1404 != 0.0 {
                let v1405 = v1400 * v9;
                let v1415 = (((v8 * v24) * v507) * (v1408 - v2)) / (v2 + ((v2 + (v1395 * v1408)).sqrt()));
                let v1416 = if v1401 == v2 { 1.0 } else { 0.0 };
                let v1441: f64;
                if v1416 != 0.0 {
                    let v1418 = (v8 * v507) * v323;
                    let v1423 = v679 - (v104 * (v24 - ((v1418 * v106).ln())));
                    let v1425 = v1423 * v1423;
                    let v1426 = if v1423 < v0 { 1.0 } else { 0.0 };
                    let v1436: f64;
                    if v1426 != 0.0 {
                        let v1431 = v1427 / (((v1425 + v1424).sqrt()) - v1423);
                        v1436 = v1431;
                    } else {
                        let v1435 = v390 * (((v1425 + v1424).sqrt()) + v1423);
                        v1436 = v1435;
                    }
                    let v1440 = v1436 / ((v1418 + (v1415 * v323)) + v1436);
                    v1441 = v1440;
                } else {
                    v1441 = v2;
                }
                let v1442 = v1441 * v1415;
                v1494 = v1405;
                v1497 = v1442;
                v1837 = v1441;
            } else {
                v1494 = v1400;
                v1497 = v0;
                v1837 = v2;
            }
            let v1444 = if v1443 == v2 { 1.0 } else { 0.0 };
            let v1491: f64;
            if v1444 != 0.0 {
                let v1445 = v654 + v643;
                let v1452 = ((v1448 * v1445) * v1450) * v1445;
                let v1455 = if (v1453 * v1445) < v0 { 1.0 } else { 0.0 };
                let v1481: f64;
                if v1455 != 0.0 {
                    let v1462 = v1456 / (((v1452 + v1447).sqrt()) - (v1459 * v1445));
                    v1481 = v1462;
                } else {
                    let v1468 = v390 * (((v1452 + v1447).sqrt()) + (v1465 * v1445));
                    v1481 = v1468;
                }
                let v1472 = v2 / (v2 - (v87.powf(v1469)));
                let v1474 = v87 * v1473;
                let v1480 = (((v1472 * v1472) * (v87.powf((v1469 - v2)))) * v1469) / v1473;
                let v1482 = if v1481 < v1474 { 1.0 } else { 0.0 };
                let v1492: f64;
                if v1482 != 0.0 {
                    let v1486 = v2 / (v2 - ((v1481 / v1473).powf(v1469)));
                    v1492 = v1486;
                } else {
                    let v1489 = v1472 + ((v1481 - v1474) * v1480);
                    v1492 = v1489;
                }
                v1491 = v1492;
            } else {
                v1491 = v2;
            }
            let v1493 = v1490 * v1491;
            let v1495 = v1494 * v1491;
            let v1496 = v1234 * v1491;
            let v1498 = v1497 * v1491;
            let v1502 = (v2 + (v951 / v589)) + (v1001 / v586);
            let v1504 = v1502 * v1502;
            let v1505 = if v1502 < v0 { 1.0 } else { 0.0 };
            let v1515: f64;
            if v1505 != 0.0 {
                let v1510 = v1506 / (((v1504 + v1503).sqrt()) - v1502);
                v1515 = v1510;
            } else {
                let v1514 = v390 * (((v1504 + v1503).sqrt()) + v1502);
                v1515 = v1514;
            }
            let v1517 = v312 / (v1515 * v1055);
            let v1518 = if v1517 < v21 { 1.0 } else { 0.0 };
            let v1519: f64;
            if v1518 != 0.0 {
                v1519 = v21;
            } else {
                v1519 = v1517;
            }
            let v1520 = v151 * v1519;
            let v1522 = if v1062 > v0 { 1.0 } else { 0.0 };
            let v1729: f64;
            if v1522 != 0.0 {
                let v1524 = if v1523 == v2 { 1.0 } else { 0.0 };
                let v1686: f64;
                if v1524 != 0.0 {
                    let v1526 = if v643 < v1525 { 1.0 } else { 0.0 };
                    let v1687: f64;
                    if v1526 != 0.0 {
                        let v1529 = (-v1062) / v1528;
                        let v1530 = if v1529 < v681 { 1.0 } else { 0.0 };
                        let v1537: f64;
                        if v1530 != 0.0 {
                            let v1531 = v1529.exp();
                            v1537 = v1531;
                        } else {
                            let v1535 = (v681.exp()) * (v2 + (v1529 - v681));
                            v1537 = v1535;
                        }
                        let v1538 = (v1525 - v643) * v1537;
                        let v1543 = (-v1539) * (v1538.powf(v1541));
                        let v1544 = if v1543 < v681 { 1.0 } else { 0.0 };
                        let v1553: f64;
                        if v1544 != 0.0 {
                            let v1545 = v1543.exp();
                            v1553 = v1545;
                        } else {
                            let v1549 = (v681.exp()) * (v2 + (v1543 - v681));
                            v1553 = v1549;
                        }
                        let v1554 = ((v1550 / v1539) * v1538) * v1553;
                        v1687 = v1554;
                    } else {
                        v1687 = v0;
                    }
                    v1686 = v1687;
                } else {
                    let v1555 = if v1523 == v24 { 1.0 } else { 0.0 };
                    let v1688: f64;
                    if v1555 != 0.0 {
                        let v1556 = if v643 < v717 { 1.0 } else { 0.0 };
                        let v1689: f64;
                        if v1556 != 0.0 {
                            let v1561 = (v24 * v1557) / (v1559 * v1559);
                            let v1562 = v717 - v643;
                            let v1566 = ((v24 * (v1562 / v983)) / v1561).sqrt();
                            let v1568 = if v1567 == v0 { 1.0 } else { 0.0 };
                            let v1575: f64;
                            if v1568 != 0.0 {
                                v1575 = v1559;
                            } else {
                                let v1572 = v2 - (v390 * v1569);
                                let v1574 = (v1559 * v1572) * v1572;
                                v1575 = v1574;
                            }
                            let v1581 = (v1566 * v1575) / (((v1566 * v1566) + (v1575 * v1575)).sqrt());
                            let v1582 = v1562 / v1581;
                            let v1583 = v390 * v1581;
                            let v1584 = v1583 * v1561;
                            let v1586 = v1582 + (v1584 * v983);
                            let v1613: f64;
                            if v1568 != 0.0 {
                                v1613 = v1586;
                            } else {
                                let v1588 = v24 * v1587;
                                let v1600 = v1582 - (v1584 * (((v2 + v1587) / (v2 + v1588)) - (v1062 / (v798 * (v2 + (v1588 * (v2 + (v24 * v1569))))))));
                                let v1601 = v1600 - v1586;
                                let v1612 = v390 * ((v1600 + v1586) + (((v1601 * v1601) + ((((v39 * v1582) * v1582) * v1605) / v798)).sqrt()));
                                v1613 = v1612;
                            }
                            let v1615 = (v1613 - v1582) / v1613;
                            let v1618 = if (v1615.abs()) > v1617 { 1.0 } else { 0.0 };
                            let v1690: f64;
                            if v1618 != 0.0 {
                                let v1619 = v1583 / v1615;
                                let v1626 = (-v1621) / v1613;
                                let v1633 = (((v1620 / v1621) * v1613) * v1619) * ((v1626.exp()) - ((v1626 * (v2 + (v1575 / v1619))).exp()));
                                v1690 = v1633;
                            } else {
                                let v1638 = (v1620 * v1575) * (((-v1621) / v1613).exp());
                                v1690 = v1638;
                            }
                            v1689 = v1690;
                        } else {
                            v1689 = v0;
                        }
                        v1688 = v1689;
                    } else {
                        let v1639 = if v1523 == v151 { 1.0 } else { 0.0 };
                        let v1691: f64;
                        if v1639 != 0.0 {
                            let v1640 = if v643 < v1525 { 1.0 } else { 0.0 };
                            let v1692: f64;
                            if v1640 != 0.0 {
                                let v1641 = v1525 - v643;
                                let v1649 = (v1641.powf(v1541)) * ((v2 - (v1062 / (v1643 + v1062))).powf(v1647));
                                let v1650 = if v1567 == v0 { 1.0 } else { 0.0 };
                                let v1674: f64;
                                if v1650 != 0.0 {
                                    v1674 = v1649;
                                } else {
                                    let v1653 = (v1062 - v1651) / v1643;
                                    let v1656 = (v1653 - v2) / v1655;
                                    let v1657 = if v1653 < v2 { 1.0 } else { 0.0 };
                                    let v1669: f64;
                                    if v1657 != 0.0 {
                                        let v1662 = v2 + (v1655 * ((v2 + (v1656.exp())).ln()));
                                        v1669 = v1662;
                                    } else {
                                        let v1668 = v1653 + (v1655 * ((v2 + ((-v1656).exp())).ln()));
                                        v1669 = v1668;
                                    }
                                    let v1672 = v1649 * (v1669.powf(v1670));
                                    v1674 = v1672;
                                }
                                let v1675 = (-v1539) * v1674;
                                let v1676 = if v1675 < v681 { 1.0 } else { 0.0 };
                                let v1684: f64;
                                if v1676 != 0.0 {
                                    let v1677 = v1675.exp();
                                    v1684 = v1677;
                                } else {
                                    let v1681 = (v681.exp()) * (v2 + (v1675 - v681));
                                    v1684 = v1681;
                                }
                                let v1685 = ((v1550 / v1539) * v1641) * v1684;
                                v1692 = v1685;
                            } else {
                                v1692 = v0;
                            }
                            v1691 = v1692;
                        } else {
                            v1691 = v0;
                        }
                        v1688 = v1691;
                    }
                    v1686 = v1688;
                }
                let v1693 = if v1686 > v0 { 1.0 } else { 0.0 };
                let v1730: f64;
                if v1693 != 0.0 {
                    let v1695 = if v1694 == v2 { 1.0 } else { 0.0 };
                    let v1731: f64;
                    if v1695 != 0.0 {
                        let v1697 = v1696 + v1520;
                        let v1705 = ((v104 / (v1062 * v1697)) + ((v1056 / v415) * v466)) + (v1703 / v1697);
                        let v1706 = if v1523 == v151 { 1.0 } else { 0.0 };
                        let v1732: f64;
                        if v1706 != 0.0 {
                            let v1708 = (v1686 - v1705) / v1446;
                            let v1709 = if v1686 < v1705 { 1.0 } else { 0.0 };
                            let v1721: f64;
                            if v1709 != 0.0 {
                                let v1714 = v1686 - (v1446 * ((v2 + (v1708.exp())).ln()));
                                v1721 = v1714;
                            } else {
                                let v1720 = v1705 - (v1446 * ((v2 + ((-v1708).exp())).ln()));
                                v1721 = v1720;
                            }
                            let v1722 = v1062 * v1721;
                            v1732 = v1722;
                        } else {
                            let v1726 = ((v1062 * v1686) * v1705) / (v1686 + v1705);
                            v1732 = v1726;
                        }
                        v1731 = v1732;
                    } else {
                        let v1727 = v1062 * v1686;
                        v1731 = v1727;
                    }
                    v1730 = v1731;
                } else {
                    v1730 = v0;
                }
                v1729 = v1730;
            } else {
                v1729 = v0;
            }
            let v1728 = if v1010 > v0 { 1.0 } else { 0.0 };
            if v1728 != 0.0 {
            } else {
            }
            if v476 != 0.0 {
            } else {
            }
            let v1741 = v1739 + v1214;
            let v1745 = (v1743 + v1224) + v1244;
            let v1746 = if v652 < v925 { 1.0 } else { 0.0 };
            if v1746 != 0.0 {
            } else {
            }
            let v1748 = v595 * v420;
            let v1749 = v39 * v291;
            let v1750 = if v674 < v964 { 1.0 } else { 0.0 };
            if v1750 != 0.0 {
            } else {
            }
            let v1751 = v2 - v1747;
            let v1753 = (v679 - v964) / v1749;
            let v1754 = if v679 < v964 { 1.0 } else { 0.0 };
            let v1766: f64;
            if v1754 != 0.0 {
                let v1759 = v679 - (v1749 * ((v2 + (v1753.exp())).ln()));
                v1766 = v1759;
            } else {
                let v1765 = v964 - (v1749 * ((v2 + ((-v1753).exp())).ln()));
                v1766 = v1765;
            }
            let v1780 = ((v298 * ((v958 * ((v987 * (v2 - ((v2 - (v1766 / v291)).powf(v986)))) + (v959 * (v679 - v1766)))) + (v299 * v679))) * v1751) * v8;
            let v1784 = if (v649 / (v1781 * v104)) < v681 { 1.0 } else { 0.0 };
            if v1784 != 0.0 {
            } else {
            }
            let v1787 = ((v401 * v600) * v104) / v335;
            let v1789 = if v1788 == v0 { 1.0 } else { 0.0 };
            if v1789 != 0.0 {
            } else {
                let v1795 = if (((v674 - v1790) / v1792) * v106) < v681 { 1.0 } else { 0.0 };
                if v1795 != 0.0 {
                } else {
                }
            }
            let v1799 = if (if (if v1401 == v2 { 1.0 } else { 0.0 }) != 0.0 || (if v1401 == v151 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && v1403 != 0.0 { 1.0 } else { 0.0 };
            let v1864: f64;
            if v1799 != 0.0 {
                let v1838: f64;
                if v1789 != 0.0 {
                    let v1800 = v1003 * v1408;
                    let v1807 = v401 * v1806;
                    let v1818 = (((v390 * v8) * v605) * ((v1748 * ((v1800 - v1003) / (v2 + ((v2 + v1800).sqrt())))) + (v1787 * (v1807 / (v2 + ((v2 + v1807).sqrt())))))) / v602;
                    v1838 = v1818;
                } else {
                    let v1820 = (v679 - v1790) * v106;
                    let v1821 = if v1820 < v681 { 1.0 } else { 0.0 };
                    let v1831: f64;
                    if v1821 != 0.0 {
                        let v1822 = v1820.exp();
                        v1831 = v1822;
                    } else {
                        let v1826 = (v681.exp()) * (v2 + (v1820 - v681));
                        v1831 = v1826;
                    }
                    let v1836 = ((((v24 * v8) * v507) * v611) * v1408) / (v2 + ((v2 + (v401 * v1831)).sqrt()));
                    v1838 = v1836;
                }
                let v1839 = v1837 * v1838;
                v1864 = v1839;
            } else {
                v1864 = v0;
            }
            let v1841 = if v1840 == v2 { 1.0 } else { 0.0 };
            if v1841 != 0.0 {
                let v1842 = if v928 < v0 { 1.0 } else { 0.0 };
                if v1842 != 0.0 {
                } else {
                }
            } else {
            }
            if v476 != 0.0 {
            } else {
            }
            let v1845 = ((v1 * v660) / v1696) * v20;
            let v1846 = v2 - v630;
            let v1847 = if v629 > v21 { 1.0 } else { 0.0 };
            if v1847 != 0.0 {
                let v1849 = if v1848 == v0 { 1.0 } else { 0.0 };
                if v1849 != 0.0 {
                } else {
                    let v1851 = if (v1846.abs()) < v1446 { 1.0 } else { 0.0 };
                    if v1851 != 0.0 {
                    } else {
                    }
                }
            } else {
            }
            let v1854 = (v1 * v1852) * v662;
            let v1856 = (0e0f64) * v20;
            let v1859 = (v1 * v1857) * v665;
            let v1861 = (0e0f64) * v20;
            let v1863 = (v1 * v1498) * v20;
            let v1866 = v1 * (v1780 + v1864);
            let v1868 = (0e0f64) * v20;
            if v634 != 0.0 {
            } else {
            }
            if v637 != 0.0 {
            } else {
            }
            let v1870 = v1869 * v101;
            let v1871 = v1870 / v1703;
            let v1872 = v1870 / v1696;
            let v1873 = v1870 * v1733;
            let v1874 = v1870 * v1735;
            let v1875 = v1870 * v1737;
            let v1881 = ((v1870 / v1520) * ((v401 * v1521) + v1878)) * v1302;
            let v1883 = (v1060 + v1059) / v1056;
            let v1886 = v1884 * (v1883.abs());
            let v1888 = if v1887 > v0 { 1.0 } else { 0.0 };
            let v1893: f64;
            if v1888 != 0.0 {
                let v1890 = (v1729 / v1883).abs();
                v1893 = v1890;
            } else {
                v1893 = v0;
            }
            let v1895 = (v1891 * v1729) * (v1893 + v2);
            let v1896 = if v1883 > v0 { 1.0 } else { 0.0 };
            if v1896 != 0.0 {
            } else {
            }
            let v1898 = if v1897 == v2 { 1.0 } else { 0.0 };
            if v1898 != 0.0 {
            } else {
                let v1899 = if v1897 == v24 { 1.0 } else { 0.0 };
                if v1899 != 0.0 {
                } else {
                }
            }
            let v1905 = v1900 * ((((v1741 - v1742) + v1108) + v1087).abs());
            let v1906 = v1739 + v1743;
            let v1911 = v1907 * ((v1906.abs()).powf(v1909));
            let v1912 = if v1906 < v0 { 1.0 } else { 0.0 };
            let v1966: f64;
            if v1912 != 0.0 {
                let v1913 = -v1911;
                v1966 = v1913;
            } else {
                v1966 = v1911;
            }
            let v1915 = (v1214 + v1224) + v1244;
            let v1920 = v1916 * ((v1915.abs()).powf(v1918));
            let v1921 = if v1915 < v0 { 1.0 } else { 0.0 };
            let v1968: f64;
            if v1921 != 0.0 {
                let v1922 = -v1920;
                v1968 = v1922;
            } else {
                v1968 = v1920;
            }
            let v1925 = v1923 * (v1745.abs());
            let v1927 = v1496.abs();
            let v1928 = v1926 * v1927;
            let v1930 = v1907 * (v1927.powf(v1909));
            let v1931 = if v1496 < v0 { 1.0 } else { 0.0 };
            let v1972: f64;
            if v1931 != 0.0 {
                let v1932 = -v1930;
                v1972 = v1932;
            } else {
                v1972 = v1930;
            }
            let v1935 = v1933 * (v1493.abs());
            let v1937 = v1495.abs();
            let v1938 = v1936 * v1937;
            let v1940 = v2 - (v1401 * v8);
            let v1944 = (v1907 * v1940) * ((v1937 / v1940).powf(v1909));
            let v1945 = if v1495 < v0 { 1.0 } else { 0.0 };
            let v1975: f64;
            if v1945 != 0.0 {
                let v1946 = -v1944;
                v1975 = v1946;
            } else {
                v1975 = v1944;
            }
            let v1948 = v1498.abs();
            let v1950 = (v1947 * v1948) * v1401;
            let v1951 = if v8 == v0 { 1.0 } else { 0.0 };
            let v1958: f64;
            if v1951 != 0.0 {
                v1958 = v0;
            } else {
                let v1956 = ((v1907 * v1401) * v8) * ((v1948 / v8).powf(v1909));
                v1958 = v1956;
            }
            let v1957 = if v1498 < v0 { 1.0 } else { 0.0 };
            let v1978: f64;
            if v1957 != 0.0 {
                let v1959 = -v1958;
                v1978 = v1959;
            } else {
                v1978 = v1958;
            }
            let v1960 = v1886 * v20;
            let v1961 = v1895 * v20;
            let v1962 = v1905 * v20;
            let v1963 = v1871 * v20;
            let v1964 = v1872 * v20;
            let v1965 = v1881 * v20;
            let v1967 = v1966 * v20;
            let v1969 = v1968 * v20;
            let v1970 = v1925 * v20;
            let v1971 = v1928 * v20;
            let v1973 = v1972 * v20;
            let v1974 = v1938 * v20;
            let v1976 = v1975 * v20;
            let v1977 = v1950 * v20;
            let v1979 = v1978 * v20;
            let v1996: f64;
            let v1997: f64;
            let v1998: f64;
            let v1999: f64;
            if v476 != 0.0 {
                let v1980 = v1935 * v20;
                v1996 = v2;
                v1997 = v1980;
                v1998 = v0;
                v1999 = v0;
            } else {
                let v1981 = v1935 * v20;
                v1996 = v0;
                v1997 = v0;
                v1998 = v2;
                v1999 = v1981;
            }
            let v2000: f64;
            let v2002: f64;
            let v2004: f64;
            let v2006: f64;
            let v2008: f64;
            let v2010: f64;
            let v2012: f64;
            let v2014: f64;
            let v2016: f64;
            let v2018: f64;
            let v2020: f64;
            let v2022: f64;
            let v2024: f64;
            let v2026: f64;
            let v2028: f64;
            let v2030: f64;
            if v634 != 0.0 {
                let v2001: f64;
                let v2003: f64;
                let v2005: f64;
                let v2007: f64;
                let v2009: f64;
                let v2011: f64;
                let v2013: f64;
                let v2015: f64;
                let v2017: f64;
                let v2019: f64;
                if v637 != 0.0 {
                    let v1982 = v1873 * v20;
                    let v1983 = v1874 * v20;
                    let v1984 = v1875 * v20;
                    v2001 = v2;
                    v2003 = v1982;
                    v2005 = v2;
                    v2007 = v1983;
                    v2009 = v2;
                    v2011 = v1984;
                    v2013 = v0;
                    v2015 = v0;
                    v2017 = v0;
                    v2019 = v0;
                } else {
                    let v1985 = v1873 * v20;
                    let v1986 = v1874 * v20;
                    v2001 = v0;
                    v2003 = v0;
                    v2005 = v0;
                    v2007 = v0;
                    v2009 = v0;
                    v2011 = v0;
                    v2013 = v2;
                    v2015 = v1985;
                    v2017 = v2;
                    v2019 = v1986;
                }
                v2000 = v2001;
                v2002 = v2003;
                v2004 = v2005;
                v2006 = v2007;
                v2008 = v2009;
                v2010 = v2011;
                v2012 = v2013;
                v2014 = v2015;
                v2016 = v2017;
                v2018 = v2019;
                v2020 = v0;
                v2022 = v0;
                v2024 = v0;
                v2026 = v0;
                v2028 = v0;
                v2030 = v0;
            } else {
                let v2021: f64;
                let v2023: f64;
                let v2025: f64;
                let v2027: f64;
                let v2029: f64;
                let v2031: f64;
                if v637 != 0.0 {
                    let v1987 = v1873 * v20;
                    let v1988 = v1875 * v20;
                    v2021 = v2;
                    v2023 = v1987;
                    v2025 = v2;
                    v2027 = v1988;
                    v2029 = v0;
                    v2031 = v0;
                } else {
                    let v1989 = v1873 * v20;
                    v2021 = v0;
                    v2023 = v0;
                    v2025 = v0;
                    v2027 = v0;
                    v2029 = v2;
                    v2031 = v1989;
                }
                v2000 = v0;
                v2002 = v0;
                v2004 = v0;
                v2006 = v0;
                v2008 = v0;
                v2010 = v0;
                v2012 = v0;
                v2014 = v0;
                v2016 = v0;
                v2018 = v0;
                v2020 = v2021;
                v2022 = v2023;
                v2024 = v2025;
                v2026 = v2027;
                v2028 = v2029;
                v2030 = v2031;
            }
            let v1994 = if ((((v1845 + v1856) + v1861) + v1863) + v1868) == v0 { 1.0 } else { 0.0 };
            if v1994 != 0.0 {
            } else {
            }
            let v1995 = if v20 != v2 { 1.0 } else { 0.0 };
            if v1995 != 0.0 {
            } else {
            }
        {
            let psd = v1960;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 0, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 0, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 0, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(0, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = v1961;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 1, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(1, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = v1962;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 2, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 2, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 2, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(2, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = v1963;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 3, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 3, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 3, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(3, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = v1964;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 4, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 4, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 4, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(4, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = v1965;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 5, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 5, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 5, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(5, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = v1967;
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
            let psd = v1969;
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
            let psd = v1970;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 8, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 8, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 8, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(8, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = v1971;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 9, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 9, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 9, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(9, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = v1973;
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
            let psd = v1974;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 11, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 11, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 11, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(11, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = v1976;
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
            let psd = v1977;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 13, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 13, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 13, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(13, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = v1979;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 14, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 14, value: psd }); }
            let exponent: Option<f64> = Some(v2);
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 14, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 14, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(14, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v1996 == 0.0 {
            if !visitor.visit(15, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v1997;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 15, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 15, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 15, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(15, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v1998 == 0.0 {
            if !visitor.visit(16, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v1999;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 16, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 16, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 16, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(16, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v2000 == 0.0 {
            if !visitor.visit(17, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v2002;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 17, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 17, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 17, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(17, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v2004 == 0.0 {
            if !visitor.visit(18, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v2006;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 18, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 18, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 18, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(18, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v2008 == 0.0 {
            if !visitor.visit(19, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v2010;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 19, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 19, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 19, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(19, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v2012 == 0.0 {
            if !visitor.visit(20, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v2014;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 20, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 20, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 20, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(20, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v2016 == 0.0 {
            if !visitor.visit(21, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v2018;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 21, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 21, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 21, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(21, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v2020 == 0.0 {
            if !visitor.visit(22, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v2022;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 22, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 22, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 22, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(22, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v2024 == 0.0 {
            if !visitor.visit(23, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v2026;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 23, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 23, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 23, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(23, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v2028 == 0.0 {
            if !visitor.visit(24, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v2030;
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
