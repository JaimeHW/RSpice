#![allow(dead_code, non_snake_case, unused_parens, unused_variables)]

use super::state::Instance;
use crate::device::veriloga_generated::GeneratedEvalContext;
pub use crate::device::veriloga_generated::{GeneratedNoiseDescriptor, GeneratedNoiseEndpoint, GeneratedNoiseEvaluation, GeneratedNoiseEvaluationError, GeneratedNoiseEvaluationRef, GeneratedNoiseKind, GeneratedNoiseVisitor};

pub static NOISE_SOURCES: [GeneratedNoiseDescriptor; 15] = [
    GeneratedNoiseDescriptor { mechanism: "WHITE_BI_EI_IBEI_SHOT_NOISE", label: Some("Ibei shot noise"), kind: GeneratedNoiseKind::White, equation: 31, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(8), name: "bi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(9), name: "ei", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_BI_EI_IBEI_FLICKER_NOISE", label: Some("Ibei flicker noise"), kind: GeneratedNoiseKind::Flicker, equation: 32, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(8), name: "bi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(9), name: "ei", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_BX_EI_IBEX_SHOT_NOISE", label: Some("Ibex shot noise"), kind: GeneratedNoiseKind::White, equation: 33, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(7), name: "bx", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(9), name: "ei", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_BX_EI_IBEX_FLICKER_NOISE", label: Some("Ibex flicker noise"), kind: GeneratedNoiseKind::Flicker, equation: 34, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(7), name: "bx", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(9), name: "ei", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_CI_EI_TRANSPORT_CURRENT_SHOT_NOISE", label: Some("transport current shot noise"), kind: GeneratedNoiseKind::White, equation: 35, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(6), name: "ci", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(9), name: "ei", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_BX_BP_IBEP_SHOT_NOISE", label: Some("Ibep shot noise"), kind: GeneratedNoiseKind::White, equation: 36, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(7), name: "bx", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(10), name: "bp", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_BX_BP_IBEP_FLICKER_NOISE", label: Some("Ibep flicker noise"), kind: GeneratedNoiseKind::Flicker, equation: 37, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(7), name: "bx", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(10), name: "bp", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C_CX_RCX_THERMAL_NOISE", label: Some("rcx thermal noise"), kind: GeneratedNoiseKind::White, equation: 38, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(0), name: "c", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(5), name: "cx", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_CX_CI_RCI_THERMAL_NOISE", label: Some("rci thermal noise"), kind: GeneratedNoiseKind::White, equation: 39, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "cx", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(6), name: "ci", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_B_BX_RBX_THERMAL_NOISE", label: Some("rbx thermal noise"), kind: GeneratedNoiseKind::White, equation: 40, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(1), name: "b", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(7), name: "bx", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_BX_BI_RBI_THERMAL_NOISE", label: Some("rbi thermal noise"), kind: GeneratedNoiseKind::White, equation: 41, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(7), name: "bx", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(8), name: "bi", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_E_EI_RE_THERMAL_NOISE", label: Some("re thermal noise"), kind: GeneratedNoiseKind::White, equation: 42, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(2), name: "e", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(9), name: "ei", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_BP_CX_RBP_THERMAL_NOISE", label: Some("rbp thermal noise"), kind: GeneratedNoiseKind::White, equation: 43, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(10), name: "bp", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(5), name: "cx", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_BX_SI_PARASITIC_TRANSPORT_CURRENT_SHOT_NOISE", label: Some("parasitic transport current shot noise"), kind: GeneratedNoiseKind::White, equation: 44, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(7), name: "bx", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(11), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_S_SI_RS_THERMAL_NOISE", label: Some("rs thermal noise"), kind: GeneratedNoiseKind::White, equation: 45, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(3), name: "s", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(11), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
];

impl Instance {
    pub fn evaluate_noise_sources(&self, ctx: &GeneratedEvalContext<'_>, visitor: &mut dyn GeneratedNoiseVisitor) -> Result<(), GeneratedNoiseEvaluationError> {
        if !self.multiplicity.is_finite() || self.multiplicity <= 0.0 {
            return Err(GeneratedNoiseEvaluationError::InvalidMultiplicity { value: self.multiplicity });
        }
        let parameters = &self.params.values;
        let parameter_given = &*self.param_given;
        let temperature = ctx.temperature();
        let node_potentials = [ctx.node_voltage(self.nodes[0]), ctx.node_voltage(self.nodes[1]), ctx.node_voltage(self.nodes[2]), ctx.node_voltage(self.nodes[3]), ctx.node_voltage(self.nodes[4]), ctx.node_voltage(self.nodes[5]), ctx.node_voltage(self.nodes[6]), ctx.node_voltage(self.nodes[7]), ctx.node_voltage(self.nodes[8]), ctx.node_voltage(self.nodes[9]), ctx.node_voltage(self.nodes[10]), ctx.node_voltage(self.nodes[11]), ctx.node_voltage(self.nodes[12]), ctx.node_voltage(self.nodes[13])];
            let v0 = 0e0f64;
            let v1 = if parameter_given[6] { 1.0 } else { 0.0 };
            let v2 = 1e0f64;
            let v3 = if parameter_given[7] { 1.0 } else { 0.0 };
            let v4 = 1e-2f64;
            let v5 = if parameter_given[10] { 1.0 } else { 0.0 };
            let v6 = parameters[10];
            let v7 = 1e-12f64;
            let v9 = if parameter_given[11] { 1.0 } else { 0.0 };
            let v10 = parameters[11];
            let v12 = if parameter_given[3] { 1.0 } else { 0.0 };
            let v13 = if parameter_given[4] { 1.0 } else { 0.0 };
            let v14 = -1e0f64;
            let v15 = if parameter_given[5] { 1.0 } else { 0.0 };
            let v16 = parameters[5];
            let v17 = parameters[12];
            let v19 = parameters[74];
            let v23 = parameters[75];
            let v27 = parameters[20];
            let v31 = parameters[79];
            let v35 = parameters[80];
            let v37 = 2.7315e2f64;
            let v38 = parameters[13];
            let v40 = temperature;
            let v41 = parameters[0];
            let v44 = parameters[8];
            let v46 = parameters[9];
            let v48 = parameters[14];
            let v55 = parameters[15];
            let v65 = 1.380662e-23f64;
            let v67 = 1.602189e-19f64;
            let v70 = parameters[26];
            let v71 = parameters[90];
            let v73 = parameters[89];
            let v75 = parameters[88];
            let v84 = parameters[122];
            let v85 = parameters[28];
            let v89 = parameters[113];
            let v98 = parameters[72];
            let v102 = 5e-1f64;
            let v104 = 4e0f64;
            let v106 = parameters[73];
            let v120 = parameters[27];
            let v121 = parameters[125];
            let v122 = parameters[29];
            let v126 = parameters[121];
            let v154 = parameters[31];
            let v155 = parameters[33];
            let v159 = parameters[120];
            let v179 = parameters[54];
            let v180 = parameters[123];
            let v181 = parameters[56];
            let v185 = parameters[114];
            let v197 = parameters[58];
            let v198 = parameters[124];
            let v199 = parameters[59];
            let v203 = parameters[117];
            let v215 = parameters[60];
            let v216 = parameters[61];
            let v220 = parameters[115];
            let v232 = parameters[62];
            let v233 = parameters[63];
            let v237 = parameters[118];
            let v249 = parameters[64];
            let v257 = parameters[65];
            let v265 = parameters[66];
            let v266 = parameters[67];
            let v270 = parameters[116];
            let v282 = parameters[68];
            let v283 = parameters[69];
            let v287 = parameters[119];
            let v299 = node_potentials[4];
            let v320 = parameters[126];
            let v323 = if parameter_given[109] { 1.0 } else { 0.0 };
            let v324 = parameters[16];
            let v325 = parameters[109];
            let v328 = parameters[107];
            let v331 = if parameter_given[108] { 1.0 } else { 0.0 };
            let v332 = parameters[17];
            let v333 = parameters[108];
            let v338 = if parameter_given[106] { 1.0 } else { 0.0 };
            let v339 = parameters[21];
            let v340 = parameters[106];
            let v343 = parameters[104];
            let v346 = if parameter_given[105] { 1.0 } else { 0.0 };
            let v347 = parameters[22];
            let v348 = parameters[105];
            let v353 = parameters[23];
            let v354 = parameters[103];
            let v357 = parameters[24];
            let v358 = parameters[111];
            let v361 = if parameter_given[110] { 1.0 } else { 0.0 };
            let v362 = parameters[25];
            let v363 = parameters[110];
            let v368 = parameters[101];
            let v369 = parameters[132];
            let v423 = parameters[129];
            let v428 = parameters[84];
            let v429 = parameters[127];
            let v433 = parameters[86];
            let v434 = parameters[128];
            let v438 = parameters[91];
            let v439 = parameters[92];
            let v445 = parameters[93];
            let v449 = 2e0f64;
            let v452 = parameters[37];
            let v457 = -5e-1f64;
            let v466 = 3e0f64;
            let v486 = parameters[42];
            let v491 = -5e-1f64;
            let v514 = parameters[50];
            let v519 = -5e-1f64;
            let v542 = parameters[38];
            let v543 = parameters[43];
            let v544 = parameters[49];
            let v545 = parameters[19];
            let v551 = parameters[18];
            let v552 = parameters[112];
            let v559 = parameters[70];
            let v560 = parameters[130];
            let v564 = parameters[71];
            let v565 = parameters[131];
            let v570 = 1e-3f64;
            let v573 = 1e3f64;
            let v613 = node_potentials[8];
            let v614 = node_potentials[9];
            let v617 = node_potentials[7];
            let v620 = node_potentials[6];
            let v623 = node_potentials[5];
            let v628 = node_potentials[10];
            let v633 = node_potentials[11];
            let v637 = parameters[34];
            let v639 = parameters[39];
            let v673 = -5e-1f64;
            let v710 = parameters[44];
            let v715 = -1e0f64;
            let v730 = parameters[45];
            let v760 = parameters[46];
            let v840 = -5e-1f64;
            let v909 = 1e-4f64;
            let v912 = 1e-8f64;
            let v921 = parameters[30];
            let v971 = parameters[32];
            let v987 = 5.0005e-1f64;
            let v1003 = parameters[55];
            let v1027 = parameters[57];
            let v1289 = parameters[83];
            let v1291 = 2e-2f64;
            let v1294 = 1.01e0f64;
            let v1311 = parameters[85];
            let v1315 = parameters[87];
            let v1332 = parameters[97];
            let v1334 = parameters[95];
            let v1337 = parameters[94];
            let v1372 = parameters[52];
            let v1389 = 1.44e0f64;
            let v1392 = parameters[1];
            let v1393 = 3.204378e-19f64;
            let v1396 = parameters[98];
            let v1397 = parameters[99];
            let v1400 = parameters[100];
            let v1401 = 3.204378e-19f64;
            let v1406 = 3.204378e-19f64;
            let v1409 = 3.204378e-19f64;
            let v1414 = 5.522648e-23f64;
            let v1417 = 5.522648e-23f64;
            let v1420 = 1e-10f64;
            let v1427 = 5.522648e-23f64;
            let v1430 = 5.522648e-23f64;
            let v1434 = 5.522648e-23f64;
            let v1437 = 5.522648e-23f64;
            let v1441 = 3.204378e-19f64;
            let v1444 = 5.522648e-23f64;
            if v1 != 0.0 {
            } else {
            }
            if v3 != 0.0 {
            } else {
            }
            let v1356: f64;
            if v5 != 0.0 {
                v1356 = v6;
            } else {
                let v8 = ctx.simparam_or("gmin", v7);
                v1356 = v8;
            }
            let v79: f64;
            if v9 != 0.0 {
                v79 = v10;
            } else {
                let v11 = ctx.simparam_or("pnjmaxi", v2);
                v79 = v11;
            }
            let v610: f64;
            if v12 != 0.0 {
                v610 = v2;
            } else {
                let v611: f64;
                if v13 != 0.0 {
                    v611 = v14;
                } else {
                    let v612: f64;
                    if v15 != 0.0 {
                        v612 = v16;
                    } else {
                        v612 = v2;
                    }
                    v611 = v612;
                }
                v610 = v611;
            }
            let v18 = v17.ln();
            let v20 = if v19 > v0 { 1.0 } else { 0.0 };
            let v22: f64;
            if v20 != 0.0 {
                let v21 = v2 / v19;
                v22 = v21;
            } else {
                v22 = v0;
            }
            let v24 = if v23 > v0 { 1.0 } else { 0.0 };
            let v26: f64;
            if v24 != 0.0 {
                let v25 = v2 / v23;
                v26 = v25;
            } else {
                v26 = v0;
            }
            let v28 = if v27 > v0 { 1.0 } else { 0.0 };
            let v30: f64;
            if v28 != 0.0 {
                let v29 = v2 / v27;
                v30 = v29;
            } else {
                v30 = v0;
            }
            let v32 = if v31 > v0 { 1.0 } else { 0.0 };
            let v34: f64;
            if v32 != 0.0 {
                let v33 = v2 / v31;
                v34 = v33;
            } else {
                v34 = v0;
            }
            let v36 = if v35 > v0 { 1.0 } else { 0.0 };
            if v36 != 0.0 {
            } else {
            }
            if v36 != 0.0 {
            } else {
            }
            let v39 = v37 + v38;
            let v42 = v40 + v41;
            let v43 = v42 - v37;
            let v45 = if v43 < v44 { 1.0 } else { 0.0 };
            if v45 != 0.0 {
            } else {
            }
            let v47 = if v43 > v46 { 1.0 } else { 0.0 };
            if v47 != 0.0 {
            } else {
            }
            let v49 = v48 + v2;
            let v50 = if v43 < v49 { 1.0 } else { 0.0 };
            let v62: f64;
            if v50 != 0.0 {
                let v54 = v48 + (((v43 - v48) - v2).exp());
                v62 = v54;
            } else {
                let v57 = if v43 > (v55 - v2) { 1.0 } else { 0.0 };
                let v63: f64;
                if v57 != 0.0 {
                    let v61 = v55 - (((v55 - v43) - v2).exp());
                    v63 = v61;
                } else {
                    v63 = v43;
                }
                v62 = v63;
            }
            let v64 = v62 + v37;
            let v68 = (v65 * v64) / v67;
            let v69 = v64 / v39;
            let v72 = if v71 > v0 { 1.0 } else { 0.0 };
            let v1048: f64;
            if v72 != 0.0 {
                let v74 = v73 * v68;
                let v83 = v74 * (((((-v75) / v74).exp()) + (v79 / v71)).ln());
                v1048 = v83;
            } else {
                v1048 = v0;
            }
            let v86 = v84 / v85;
            let v90 = -v89;
            let v91 = v2 - v69;
            let v93 = v68 * v85;
            let v96 = (v70 * (v69.powf(v86))) * (((v90 * v91) / v93).exp());
            let v97 = if v96 > v0 { 1.0 } else { 0.0 };
            let v871: f64;
            if v97 != 0.0 {
                let v101 = if (if v98 > v0 { 1.0 } else { 0.0 }) != 0.0 && (if v79 > v98 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v872: f64;
                if v101 != 0.0 {
                    let v115 = v93 * ((v2 + ((((v102 * v79) * ((v104 / v98).powf(v106))).powf((v2 / (v2 - v106)))) / v96)).ln());
                    v872 = v115;
                } else {
                    let v119 = v93 * ((v2 + (v79 / v96)).ln());
                    v872 = v119;
                }
                v871 = v872;
            } else {
                v871 = v0;
            }
            let v123 = v121 / v122;
            let v127 = -v126;
            let v129 = v68 * v122;
            let v132 = (v120 * (v69.powf(v123))) * (((v127 * v91) / v129).exp());
            let v134 = if v97 != 0.0 && (if v132 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let v887: f64;
            if v134 != 0.0 {
                let v136 = if v20 != 0.0 && (if v79 > v19 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v888: f64;
                if v136 != 0.0 {
                    let v148 = v129 * ((v2 + ((((v102 * v79) * ((v104 / v19).powf(v106))).powf((v2 / (v2 - v106)))) / (v96 * v132))).ln());
                    v888 = v148;
                } else {
                    let v153 = v129 * ((v2 + (v79 / (v96 * v132))).ln());
                    v888 = v153;
                }
                v887 = v888;
            } else {
                v887 = v0;
            }
            let v156 = v84 / v155;
            let v160 = -v159;
            let v162 = v68 * v155;
            let v165 = (v154 * (v69.powf(v156))) * (((v160 * v91) / v162).exp());
            let v166 = if v165 > v0 { 1.0 } else { 0.0 };
            let v951: f64;
            if v166 != 0.0 {
                let v168 = if v24 != 0.0 && (if v79 > v23 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v952: f64;
                if v168 != 0.0 {
                    let v174 = v162 * ((v2 + (((v79 * v79) * v26) / v165)).ln());
                    v952 = v174;
                } else {
                    let v178 = v162 * ((v2 + (v79 / v165)).ln());
                    v952 = v178;
                }
                v951 = v952;
            } else {
                v951 = v0;
            }
            let v182 = v180 / v181;
            let v186 = -v185;
            let v188 = v68 * v181;
            let v191 = (v179 * (v69.powf(v182))) * (((v186 * v91) / v188).exp());
            let v192 = if v191 > v0 { 1.0 } else { 0.0 };
            let v1006: f64;
            if v192 != 0.0 {
                let v196 = v188 * ((v2 + (v79 / v191)).ln());
                v1006 = v196;
            } else {
                v1006 = v0;
            }
            let v200 = v198 / v199;
            let v204 = -v203;
            let v206 = v68 * v199;
            let v209 = (v197 * (v69.powf(v200))) * (((v204 * v91) / v206).exp());
            let v210 = if v209 > v0 { 1.0 } else { 0.0 };
            let v1017: f64;
            if v210 != 0.0 {
                let v214 = v206 * ((v2 + (v79 / v209)).ln());
                v1017 = v214;
            } else {
                v1017 = v0;
            }
            let v217 = v180 / v216;
            let v218 = v69.powf(v217);
            let v221 = -v220;
            let v223 = v68 * v216;
            let v225 = ((v221 * v91) / v223).exp();
            let v226 = (v215 * v218) * v225;
            let v227 = if v226 > v0 { 1.0 } else { 0.0 };
            let v1208: f64;
            if v227 != 0.0 {
                let v231 = v223 * ((v2 + (v79 / v226)).ln());
                v1208 = v231;
            } else {
                v1208 = v0;
            }
            let v234 = v198 / v233;
            let v235 = v69.powf(v234);
            let v238 = -v237;
            let v240 = v68 * v233;
            let v242 = ((v238 * v91) / v240).exp();
            let v243 = (v232 * v235) * v242;
            let v244 = if v243 > v0 { 1.0 } else { 0.0 };
            let v1211: f64;
            if v244 != 0.0 {
                let v248 = v240 * ((v2 + (v79 / v243)).ln());
                v1211 = v248;
            } else {
                v1211 = v0;
            }
            let v251 = (v249 * v218) * v225;
            let v252 = if v251 > v0 { 1.0 } else { 0.0 };
            let v1216: f64;
            if v252 != 0.0 {
                let v256 = v223 * ((v2 + (v79 / v251)).ln());
                v1216 = v256;
            } else {
                v1216 = v0;
            }
            let v259 = (v257 * v235) * v242;
            let v260 = if v259 > v0 { 1.0 } else { 0.0 };
            let v1226: f64;
            if v260 != 0.0 {
                let v264 = v240 * ((v2 + (v79 / v259)).ln());
                v1226 = v264;
            } else {
                v1226 = v0;
            }
            let v273 = v68 * v266;
            let v276 = (v265 * (v69.powf((v180 / v266)))) * ((((-v270) * v91) / v273).exp());
            let v277 = if v276 > v0 { 1.0 } else { 0.0 };
            let v1342: f64;
            if v277 != 0.0 {
                let v281 = v273 * ((v2 + (v79 / v276)).ln());
                v1342 = v281;
            } else {
                v1342 = v0;
            }
            let v290 = v68 * v283;
            let v293 = (v282 * (v69.powf((v198 / v283)))) * ((((-v287) * v91) / v290).exp());
            let v294 = if v293 > v0 { 1.0 } else { 0.0 };
            let v1344: f64;
            if v294 != 0.0 {
                let v298 = v290 * ((v2 + (v79 / v293)).ln());
                v1344 = v298;
            } else {
                v1344 = v0;
            }
            let v301 = (v42 + v299) - v37;
            let v302 = if v301 < v49 { 1.0 } else { 0.0 };
            let v313: f64;
            if v302 != 0.0 {
                let v306 = v48 + (((v301 - v48) - v2).exp());
                v313 = v306;
            } else {
                let v308 = if v301 > (v55 - v2) { 1.0 } else { 0.0 };
                let v314: f64;
                if v308 != 0.0 {
                    let v312 = v55 - (((v55 - v301) - v2).exp());
                    v314 = v312;
                } else {
                    v314 = v301;
                }
                v313 = v314;
            }
            let v315 = v313 + v37;
            let v317 = (v65 * v315) / v67;
            let v318 = v315 / v39;
            let v319 = v315 - v39;
            let v322 = v98 * (v318.powf(v320));
            let v569: f64;
            if v323 != 0.0 {
                let v327 = v324 * (v318.powf(v325));
                v569 = v327;
            } else {
                let v330 = v324 * (v318.powf(v328));
                v569 = v330;
            }
            let v575: f64;
            if v331 != 0.0 {
                let v335 = v332 * (v318.powf(v333));
                v575 = v335;
            } else {
                let v337 = v332 * (v318.powf(v328));
                v575 = v337;
            }
            let v579: f64;
            if v338 != 0.0 {
                let v342 = v339 * (v318.powf(v340));
                v579 = v342;
            } else {
                let v345 = v339 * (v318.powf(v343));
                v579 = v345;
            }
            let v583: f64;
            if v346 != 0.0 {
                let v350 = v347 * (v318.powf(v348));
                v583 = v350;
            } else {
                let v352 = v347 * (v318.powf(v343));
                v583 = v352;
            }
            let v356 = v353 * (v318.powf(v354));
            let v360 = v357 * (v318.powf(v358));
            let v590: f64;
            if v361 != 0.0 {
                let v365 = v362 * (v318.powf(v363));
                v590 = v365;
            } else {
                let v367 = v362 * (v318.powf(v328));
                v590 = v367;
            }
            let v372 = v368 * (v2 + (v319 * v369));
            let v375 = v2 - v318;
            let v376 = v90 * v375;
            let v380 = (v70 * (v318.powf(v86))) * ((v376 / (v317 * v85)).exp());
            let v387 = (v120 * (v318.powf(v123))) * (((v127 * v375) / (v317 * v122)).exp());
            let v391 = v317 * v155;
            let v394 = (v154 * (v318.powf(v156))) * (((v160 * v375) / v391).exp());
            let v398 = v317 * v181;
            let v401 = (v179 * (v318.powf(v182))) * (((v186 * v375) / v398).exp());
            let v405 = v317 * v199;
            let v408 = (v197 * (v318.powf(v200))) * (((v204 * v375) / v405).exp());
            let v411 = v317 * v216;
            let v416 = v317 * v233;
            let v420 = (v249 * (v318.powf(v217))) * (((v221 * v375) / v411).exp());
            let v422 = (v257 * (v318.powf(v234))) * (((v238 * v375) / v416).exp());
            let v425 = v2 + (v319 * v423);
            let v426 = v85 * v425;
            let v427 = v122 * v425;
            let v432 = v428 * (v2 + (v319 * v429));
            let v437 = v433 * (v2 + (v319 * v434));
            let v451 = v449 * (v317 / v318);
            let v469 = (v466 * v317) * (v318.ln());
            let v471 = v318 - v2;
            let v473 = (((v451 * ((((((v102 * v452) * v318) / v317).exp()) - ((((v457 * v452) * v318) / v317).exp())).ln())) * v318) - v469) - (v185 * v471);
            let v474 = v449 * v317;
            let v485 = v473 + (v474 * ((v102 * (v2 + ((v2 + (v104 * (((-v473) / v317).exp()))).sqrt()))).ln()));
            let v502 = (((v451 * ((((((v102 * v486) * v318) / v317).exp()) - ((((v491 * v486) * v318) / v317).exp())).ln())) * v318) - v469) - (v220 * v471);
            let v513 = v502 + (v474 * ((v102 * (v2 + ((v2 + (v104 * (((-v502) / v317).exp()))).sqrt()))).ln()));
            let v530 = (((v451 * ((((((v102 * v514) * v318) / v317).exp()) - ((((v519 * v514) * v318) / v317).exp())).ln())) * v318) - v469) - (v270 * v471);
            let v541 = v530 + (v474 * ((v102 * (v2 + ((v2 + (v104 * (((-v530) / v317).exp()))).sqrt()))).ln()));
            let v550 = (v545 * (v318.powf(v84))) * ((v376 / v317).exp());
            let v554 = v551 * (v318.powf(v552));
            let v555 = -(v75 * (v2 + (v319 * (v438 + (v319 * v439)))));
            let v556 = (v73 * (v2 + (v319 * v445))) * v317;
            let v558 = (v555 / v556).exp();
            let v563 = v559 * (v2 + (v319 * v560));
            let v568 = v564 * (v2 + (v319 * v565));
            let v571 = if v569 > v570 { 1.0 } else { 0.0 };
            let v574: f64;
            if v571 != 0.0 {
                let v572 = v2 / v569;
                v574 = v572;
            } else {
                v574 = v573;
            }
            let v576 = if v575 > v570 { 1.0 } else { 0.0 };
            let v578: f64;
            if v576 != 0.0 {
                let v577 = v2 / v575;
                v578 = v577;
            } else {
                v578 = v573;
            }
            let v580 = if v579 > v570 { 1.0 } else { 0.0 };
            let v582: f64;
            if v580 != 0.0 {
                let v581 = v2 / v579;
                v582 = v581;
            } else {
                v582 = v573;
            }
            let v584 = if v583 > v570 { 1.0 } else { 0.0 };
            let v586: f64;
            if v584 != 0.0 {
                let v585 = v2 / v583;
                v586 = v585;
            } else {
                v586 = v573;
            }
            let v587 = if v356 > v570 { 1.0 } else { 0.0 };
            let v589: f64;
            if v587 != 0.0 {
                let v588 = v2 / v356;
                v589 = v588;
            } else {
                v589 = v573;
            }
            let v591 = if v590 > v570 { 1.0 } else { 0.0 };
            let v593: f64;
            if v591 != 0.0 {
                let v592 = v2 / v590;
                v593 = v592;
            } else {
                v593 = v573;
            }
            let v594 = if v360 > v570 { 1.0 } else { 0.0 };
            let v596: f64;
            if v594 != 0.0 {
                let v595 = v2 / v360;
                v596 = v595;
            } else {
                v596 = v573;
            }
            let v597 = if v372 > v570 { 1.0 } else { 0.0 };
            if v597 != 0.0 {
            } else {
            }
            let v598 = if v563 > v0 { 1.0 } else { 0.0 };
            let v600: f64;
            if v598 != 0.0 {
                let v599 = v2 / v563;
                v600 = v599;
            } else {
                v600 = v0;
            }
            let v601 = if v568 > v0 { 1.0 } else { 0.0 };
            let v603: f64;
            if v601 != 0.0 {
                let v602 = v2 / v568;
                v603 = v602;
            } else {
                v603 = v0;
            }
            let v604 = if v322 > v0 { 1.0 } else { 0.0 };
            let v606: f64;
            if v604 != 0.0 {
                let v605 = v2 / v322;
                v606 = v605;
            } else {
                v606 = v0;
            }
            let v607 = if v554 > v0 { 1.0 } else { 0.0 };
            let v609: f64;
            if v607 != 0.0 {
                let v608 = v2 / v554;
                v609 = v608;
            } else {
                v609 = v0;
            }
            let v616 = v610 * (v613 - v614);
            let v619 = v610 * (v617 - v614);
            let v622 = v610 * (v613 - v620);
            let v625 = v610 * (v613 - v623);
            let v627 = v610 * (v617 - v623);
            let v630 = v610 * (v617 - v628);
            let v632 = v610 * (v623 - v620);
            let v635 = v610 * (v633 - v628);
            let v636 = -v485;
            let v638 = v636 * v637;
            let v640 = if v639 <= v0 { 1.0 } else { 0.0 };
            let v902: f64;
            if v640 != 0.0 {
                let v641 = v616 + v638;
                let v642 = if v641 > v0 { 1.0 } else { 0.0 };
                let v665: f64;
                let v666: f64;
                if v642 != 0.0 {
                    let v643 = v2 - v637;
                    let v645 = v643.powf((-v542));
                    let v650 = (v485 * (v2 - (v645 * v643))) / (v2 - v542);
                    let v657 = (v641 * (v2 + (((v102 * v542) * v641) / (v485 * v643)))) * v645;
                    v665 = v650;
                    v666 = v657;
                } else {
                    let v660 = v2 - v542;
                    let v664 = (v485 * (v2 - ((v2 - (v616 / v485)).powf(v660)))) / v660;
                    v665 = v664;
                    v666 = v0;
                }
                let v667 = v665 + v666;
                v902 = v667;
            } else {
                let v670 = (v104 * v639) * v639;
                let v675 = v673 * (v638 + (((v638 * v638) + v670).sqrt()));
                let v678 = v2 - v542;
                let v682 = v616 + v638;
                let v688 = (v102 * (v682 - (((v682 * v682) + v670).sqrt()))) - v638;
                let v694 = v2 - v637;
                let v698 = (v616 - v688) + v675;
                let v707 = (((v636 * ((v2 - (v688 / v485)).powf(v678))) / v678) + (((v694.powf((-v542))) * v698) * (v2 + (((v102 * v542) * v698) / (v485 * v694))))) - ((v636 * ((v2 - (v675 / v485)).powf(v678))) / v678);
                v902 = v707;
            }
            let v708 = -v513;
            let v709 = v708 * v637;
            let v711 = if v710 <= v0 { 1.0 } else { 0.0 };
            let v905: f64;
            if v711 != 0.0 {
                let v712 = v622 + v709;
                let v713 = if v712 > v0 { 1.0 } else { 0.0 };
                let v755: f64;
                let v757: f64;
                if v713 != 0.0 {
                    let v714 = v2 - v637;
                    let v717 = v714.powf((v715 - v543));
                    let v723 = (v513 * (v2 - ((v717 * v714) * v714))) / (v2 - v543);
                    let v729 = (v712 * (v714 + (((v102 * v543) * v712) / v513))) * v717;
                    v755 = v723;
                    v757 = v729;
                } else {
                    let v734 = if (if v730 > v0 { 1.0 } else { 0.0 }) != 0.0 && (if v622 < (-v730) { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let v756: f64;
                    if v734 != 0.0 {
                        let v737 = v2 - v543;
                        let v747 = (v513 * (v2 - (((v2 + (v730 / v513)).powf(v737)) * (v2 - ((v737 * (v622 + v730)) / (v513 + v730)))))) / v737;
                        v756 = v747;
                    } else {
                        let v750 = v2 - v543;
                        let v754 = (v513 * (v2 - ((v2 - (v622 / v513)).powf(v750)))) / v750;
                        v756 = v754;
                    }
                    v755 = v756;
                    v757 = v0;
                }
                let v758 = v755 + v757;
                v905 = v758;
            } else {
                let v762 = if (if v730 > v0 { 1.0 } else { 0.0 }) != 0.0 && (if v760 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v906: f64;
                if v762 != 0.0 {
                    let v764 = v730 - v709;
                    let v765 = (v730 + v709) / v764;
                    let v767 = v765 - v2;
                    let v770 = (v104 * v710) * v710;
                    let v773 = v765 + v2;
                    let v776 = (v104 * v760) * v760;
                    let v784 = v102 * (((((v449 * v765) / ((((v767 * v767) + v770).sqrt()) + (((v773 * v773) + v776).sqrt()))) * v764) - v730) - v709);
                    let v787 = v2 - v543;
                    let v795 = (((v449 * v622) + v730) + v709) / v764;
                    let v797 = v795 - v2;
                    let v801 = v795 + v2;
                    let v806 = (v449 * v795) / ((((v797 * v797) + v770).sqrt()) + (((v801 * v801) + v776).sqrt()));
                    let v810 = v102 * (((v806 * v764) - v730) - v709);
                    let v818 = v102 * (v806 + v2);
                    let v821 = -v543;
                    let v834 = ((((v622 - v810) + v784) * (((v2 - v818) * ((v2 + (v730 / v513)).powf(v821))) + (v818 * ((v2 + (v709 / v513)).powf(v821))))) + ((v513 * (v2 - ((v2 - (v810 / v513)).powf(v787)))) / v787)) - ((v513 * (v2 - ((v2 - (v784 / v513)).powf(v787)))) / v787);
                    v906 = v834;
                } else {
                    let v837 = (v104 * v710) * v710;
                    let v842 = v840 * (v709 + (((v709 * v709) + v837).sqrt()));
                    let v845 = v2 - v543;
                    let v849 = v622 + v709;
                    let v855 = (v102 * (v849 - (((v849 * v849) + v837).sqrt()))) - v709;
                    let v868 = (((v708 * ((v2 - (v855 / v513)).powf(v845))) / v845) + (((v2 - v637).powf((-v543))) * ((v622 - v855) + v842))) - ((v708 * ((v2 - (v842 / v513)).powf(v845))) / v845);
                    v906 = v868;
                }
                v905 = v906;
            }
            let v870 = v2 / (v426 * v317);
            let v873 = if v616 < v871 { 1.0 } else { 0.0 };
            let v882: f64;
            if v873 != 0.0 {
                let v875 = (v616 * v870).exp();
                v882 = v875;
            } else {
                let v881 = ((v871 * v870).exp()) * (v2 + ((v616 - v871) * v870));
                v882 = v881;
            }
            let v884 = v380 * (v882 - v2);
            let v886 = v2 / (v427 * v317);
            let v889 = if v622 < v887 { 1.0 } else { 0.0 };
            let v899: f64;
            if v889 != 0.0 {
                let v891 = (v622 * v886).exp();
                v899 = v891;
            } else {
                let v897 = ((v887 * v886).exp()) * (v2 + ((v622 - v887) * v886));
                v899 = v897;
            }
            let v910 = ((v2 + (v902 * v603)) + (v905 * v600)) - v909;
            let v917 = (v102 * ((((v910 * v910) + v912).sqrt()) + v910)) + v909;
            let v920 = (v884 * v606) + (((v380 * v387) * (v899 - v2)) * v22);
            let v922 = if v921 < v102 { 1.0 } else { 0.0 };
            let v945: f64;
            if v922 != 0.0 {
                let v926 = (v917.powf((v2 / v106))) + (v104 * v920);
                let v927 = if v926 > v912 { 1.0 } else { 0.0 };
                let v946: f64;
                if v927 != 0.0 {
                    let v930 = v102 * (v917 + (v926.powf(v106)));
                    v946 = v930;
                } else {
                    let v933 = v102 * (v917 + (v912.powf(v106)));
                    v946 = v933;
                }
                v945 = v946;
            } else {
                let v935 = v2 + (v104 * v920);
                let v936 = if v935 > v912 { 1.0 } else { 0.0 };
                let v947: f64;
                if v936 != 0.0 {
                    let v940 = (v102 * v917) * (v2 + (v935.powf(v106)));
                    v947 = v940;
                } else {
                    let v944 = (v102 * v917) * (v2 + (v912.powf(v106)));
                    v947 = v944;
                }
                v945 = v947;
            }
            let v948 = v884 / v945;
            let v949 = if v154 > v0 { 1.0 } else { 0.0 };
            let v1288: f64;
            let v1355: f64;
            if v949 != 0.0 {
                let v950 = v2 / v391;
                let v953 = if v630 < v951 { 1.0 } else { 0.0 };
                let v972: f64;
                if v953 != 0.0 {
                    let v955 = (v630 * v950).exp();
                    v972 = v955;
                } else {
                    let v961 = ((v951 * v950).exp()) * (v2 + ((v630 - v951) * v950));
                    v972 = v961;
                }
                let v962 = if v622 < v951 { 1.0 } else { 0.0 };
                let v975: f64;
                if v962 != 0.0 {
                    let v964 = (v622 * v950).exp();
                    v975 = v964;
                } else {
                    let v970 = ((v951 * v950).exp()) * (v2 + ((v622 - v951) * v950));
                    v975 = v970;
                }
                let v979 = v394 * (((v971 * v972) + ((v2 - v971) * v975)) - v2);
                let v982 = v2 + (v104 * (v979 * v26));
                let v983 = if v982 > v912 { 1.0 } else { 0.0 };
                let v1001: f64;
                if v983 != 0.0 {
                    let v986 = v102 * (v2 + (v982.sqrt()));
                    v1001 = v986;
                } else {
                    v1001 = v987;
                }
                let v988 = if v635 < v951 { 1.0 } else { 0.0 };
                let v997: f64;
                if v988 != 0.0 {
                    let v990 = (v635 * v950).exp();
                    v997 = v990;
                } else {
                    let v996 = ((v951 * v950).exp()) * (v2 + ((v635 - v951) * v950));
                    v997 = v996;
                }
                let v1002 = (v979 - (v394 * (v997 - v2))) / v1001;
                v1288 = v1001;
                v1355 = v1002;
            } else {
                v1288 = v2;
                v1355 = v0;
            }
            let v1004 = if v1003 == v2 { 1.0 } else { 0.0 };
            let v1346: f64;
            let v1350: f64;
            if v1004 != 0.0 {
                let v1005 = v2 / v398;
                let v1007 = if v616 < v1006 { 1.0 } else { 0.0 };
                let v1033: f64;
                if v1007 != 0.0 {
                    let v1009 = (v616 * v1005).exp();
                    v1033 = v1009;
                } else {
                    let v1015 = ((v1006 * v1005).exp()) * (v2 + ((v616 - v1006) * v1005));
                    v1033 = v1015;
                }
                let v1016 = v2 / v405;
                let v1018 = if v616 < v1017 { 1.0 } else { 0.0 };
                let v1036: f64;
                if v1018 != 0.0 {
                    let v1020 = (v616 * v1016).exp();
                    v1036 = v1020;
                } else {
                    let v1026 = ((v1017 * v1016).exp()) * (v2 + ((v616 - v1017) * v1016));
                    v1036 = v1026;
                }
                let v1028 = if v1027 > v0 { 1.0 } else { 0.0 };
                let v1058: f64;
                if v1028 != 0.0 {
                    let v1039 = ((v401 * (v2 + (v1027 * (v917 - v2)))) * (v1033 - v2)) + (v408 * (v1036 - v2));
                    v1058 = v1039;
                } else {
                    let v1044 = (v401 * (v1033 - v2)) + (v408 * (v1036 - v2));
                    v1058 = v1044;
                }
                let v1045 = if v75 > v0 { 1.0 } else { 0.0 };
                let v1347: f64;
                if v1045 != 0.0 {
                    let v1046 = v555 - v616;
                    let v1047 = v2 / v556;
                    let v1049 = if v1046 < v1048 { 1.0 } else { 0.0 };
                    let v1059: f64;
                    if v1049 != 0.0 {
                        let v1051 = (v1046 * v1047).exp();
                        v1059 = v1051;
                    } else {
                        let v1057 = ((v1048 * v1047).exp()) * (v2 + ((v1046 - v1048) * v1047));
                        v1059 = v1057;
                    }
                    let v1062 = v1058 - (v71 * (v1059 - v558));
                    v1347 = v1062;
                } else {
                    v1347 = v1058;
                }
                v1346 = v1347;
                v1350 = v0;
            } else {
                let v1063 = if v1003 == v0 { 1.0 } else { 0.0 };
                let v1348: f64;
                let v1351: f64;
                if v1063 != 0.0 {
                    let v1064 = v2 / v398;
                    let v1065 = if v619 < v1006 { 1.0 } else { 0.0 };
                    let v1084: f64;
                    if v1065 != 0.0 {
                        let v1067 = (v619 * v1064).exp();
                        v1084 = v1067;
                    } else {
                        let v1073 = ((v1006 * v1064).exp()) * (v2 + ((v619 - v1006) * v1064));
                        v1084 = v1073;
                    }
                    let v1074 = v2 / v405;
                    let v1075 = if v619 < v1017 { 1.0 } else { 0.0 };
                    let v1087: f64;
                    if v1075 != 0.0 {
                        let v1077 = (v619 * v1074).exp();
                        v1087 = v1077;
                    } else {
                        let v1083 = ((v1017 * v1074).exp()) * (v2 + ((v619 - v1017) * v1074));
                        v1087 = v1083;
                    }
                    let v1090 = (v401 * (v1084 - v2)) + (v408 * (v1087 - v2));
                    let v1091 = if v75 > v0 { 1.0 } else { 0.0 };
                    let v1352: f64;
                    if v1091 != 0.0 {
                        let v1092 = v555 - v616;
                        let v1093 = v2 / v556;
                        let v1094 = if v1092 < v1048 { 1.0 } else { 0.0 };
                        let v1103: f64;
                        if v1094 != 0.0 {
                            let v1096 = (v1092 * v1093).exp();
                            v1103 = v1096;
                        } else {
                            let v1102 = ((v1048 * v1093).exp()) * (v2 + ((v1092 - v1048) * v1093));
                            v1103 = v1102;
                        }
                        let v1106 = v1090 - (v71 * (v1103 - v558));
                        v1352 = v1106;
                    } else {
                        v1352 = v1090;
                    }
                    v1348 = v0;
                    v1351 = v1352;
                } else {
                    let v1107 = v2 / v398;
                    let v1108 = if v616 < v1006 { 1.0 } else { 0.0 };
                    let v1132: f64;
                    if v1108 != 0.0 {
                        let v1110 = (v616 * v1107).exp();
                        v1132 = v1110;
                    } else {
                        let v1116 = ((v1006 * v1107).exp()) * (v2 + ((v616 - v1006) * v1107));
                        v1132 = v1116;
                    }
                    let v1117 = v2 / v405;
                    let v1118 = if v616 < v1017 { 1.0 } else { 0.0 };
                    let v1135: f64;
                    if v1118 != 0.0 {
                        let v1120 = (v616 * v1117).exp();
                        v1135 = v1120;
                    } else {
                        let v1126 = ((v1017 * v1117).exp()) * (v2 + ((v616 - v1017) * v1117));
                        v1135 = v1126;
                    }
                    let v1127 = if v1027 > v0 { 1.0 } else { 0.0 };
                    let v1158: f64;
                    if v1127 != 0.0 {
                        let v1139 = v1003 * (((v401 * (v2 + (v1027 * (v917 - v2)))) * (v1132 - v2)) + (v408 * (v1135 - v2)));
                        v1158 = v1139;
                    } else {
                        let v1145 = v1003 * ((v401 * (v1132 - v2)) + (v408 * (v1135 - v2)));
                        v1158 = v1145;
                    }
                    let v1146 = if v75 > v0 { 1.0 } else { 0.0 };
                    let v1349: f64;
                    if v1146 != 0.0 {
                        let v1147 = v555 - v616;
                        let v1148 = v2 / v556;
                        let v1149 = if v1147 < v1048 { 1.0 } else { 0.0 };
                        let v1160: f64;
                        if v1149 != 0.0 {
                            let v1151 = (v1147 * v1148).exp();
                            v1160 = v1151;
                        } else {
                            let v1157 = ((v1048 * v1148).exp()) * (v2 + ((v1147 - v1048) * v1148));
                            v1160 = v1157;
                        }
                        let v1163 = v1158 - ((v1003 * v71) * (v1160 - v558));
                        v1349 = v1163;
                    } else {
                        v1349 = v1158;
                    }
                    let v1164 = if v619 < v1006 { 1.0 } else { 0.0 };
                    let v1183: f64;
                    if v1164 != 0.0 {
                        let v1166 = (v619 * v1107).exp();
                        v1183 = v1166;
                    } else {
                        let v1172 = ((v1006 * v1107).exp()) * (v2 + ((v619 - v1006) * v1107));
                        v1183 = v1172;
                    }
                    let v1173 = if v619 < v1017 { 1.0 } else { 0.0 };
                    let v1186: f64;
                    if v1173 != 0.0 {
                        let v1175 = (v619 * v1117).exp();
                        v1186 = v1175;
                    } else {
                        let v1181 = ((v1017 * v1117).exp()) * (v2 + ((v619 - v1017) * v1117));
                        v1186 = v1181;
                    }
                    let v1182 = v2 - v1003;
                    let v1190 = v1182 * ((v401 * (v1183 - v2)) + (v408 * (v1186 - v2)));
                    let v1353: f64;
                    if v1146 != 0.0 {
                        let v1191 = v555 - v616;
                        let v1192 = v2 / v556;
                        let v1193 = if v1191 < v1048 { 1.0 } else { 0.0 };
                        let v1203: f64;
                        if v1193 != 0.0 {
                            let v1195 = (v1191 * v1192).exp();
                            v1203 = v1195;
                        } else {
                            let v1201 = ((v1048 * v1192).exp()) * (v2 + ((v1191 - v1048) * v1192));
                            v1203 = v1201;
                        }
                        let v1206 = v1190 - ((v1182 * v71) * (v1203 - v558));
                        v1353 = v1206;
                    } else {
                        v1353 = v1190;
                    }
                    v1348 = v1349;
                    v1351 = v1353;
                }
                v1346 = v1348;
                v1350 = v1351;
            }
            let v1207 = v2 / v411;
            let v1209 = if v622 < v1208 { 1.0 } else { 0.0 };
            if v1209 != 0.0 {
            } else {
            }
            let v1210 = v2 / v416;
            let v1212 = if v622 < v1211 { 1.0 } else { 0.0 };
            if v1212 != 0.0 {
            } else {
            }
            let v1215 = if (if v249 > v0 { 1.0 } else { 0.0 }) != 0.0 || (if v257 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let v1354: f64;
            if v1215 != 0.0 {
                let v1217 = if v630 < v1216 { 1.0 } else { 0.0 };
                let v1236: f64;
                if v1217 != 0.0 {
                    let v1219 = (v630 * v1207).exp();
                    v1236 = v1219;
                } else {
                    let v1225 = ((v1216 * v1207).exp()) * (v2 + ((v630 - v1216) * v1207));
                    v1236 = v1225;
                }
                let v1227 = if v630 < v1226 { 1.0 } else { 0.0 };
                let v1239: f64;
                if v1227 != 0.0 {
                    let v1229 = (v630 * v1210).exp();
                    v1239 = v1229;
                } else {
                    let v1235 = ((v1226 * v1210).exp()) * (v2 + ((v630 - v1226) * v1210));
                    v1239 = v1235;
                }
                let v1242 = (v420 * (v1236 - v2)) + (v422 * (v1239 - v2));
                v1354 = v1242;
            } else {
                v1354 = v0;
            }
            let v1243 = v622 / v317;
            let v1244 = if v1243 < v18 { 1.0 } else { 0.0 };
            let v1257: f64;
            if v1244 != 0.0 {
                let v1245 = v1243.exp();
                v1257 = v1245;
            } else {
                let v1249 = (v18.exp()) * (v2 + (v1243 - v18));
                v1257 = v1249;
            }
            let v1250 = v625 / v317;
            let v1251 = if v1250 < v18 { 1.0 } else { 0.0 };
            let v1261: f64;
            if v1251 != 0.0 {
                let v1252 = v1250.exp();
                v1261 = v1252;
            } else {
                let v1256 = (v18.exp()) * (v2 + (v1250 - v18));
                v1261 = v1256;
            }
            let v1260 = (v2 + (v550 * v1257)).sqrt();
            let v1264 = (v2 + (v550 * v1261)).sqrt();
            let v1273 = (v632 + (v317 * ((v1260 - v1264) - (((v1260 + v2) / (v1264 + v2)).ln())))) * v578;
            let v1283 = (v609 * v1273) / (v578 * (v2 + (((v102 * v609) * v30) * (((v632 * v632) + v4).sqrt()))));
            let v1287 = v1273 / ((v2 + (v1283 * v1283)).sqrt());
            let v1290 = if v1289 > v0 { 1.0 } else { 0.0 };
            if v1290 != 0.0 {
                let v1297 = (v1291 * (v432 + v2)).powf((v2 / (v1294 - v543)));
                let v1299 = (v513 - v622) - v1297;
                let v1310 = if ((-v432) * (((v102 * ((((v1299 * v1299) + v4).sqrt()) + v1299)) + v1297).powf((v543 - v2)))) < v18 { 1.0 } else { 0.0 };
                if v1310 != 0.0 {
                } else {
                }
            } else {
            }
            let v1312 = if v1311 > v0 { 1.0 } else { 0.0 };
            if v1312 != 0.0 {
                let v1318 = (v1291 * (v437 + v2)).powf((v2 / (v1294 - v1315)));
                let v1320 = (v0 - v627) - v1318;
                let v1331 = if ((-v437) * (((v102 * ((((v1320 * v1320) + v4).sqrt()) + v1320)) + v1318).powf((v1315 - v2)))) < v18 { 1.0 } else { 0.0 };
                if v1331 != 0.0 {
                } else {
                }
            } else {
            }
            let v1336 = if (if v1332 > v0 { 1.0 } else { 0.0 }) != 0.0 && (if v1334 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            if v1336 != 0.0 {
                let v1338 = if v1337 > v0 { 1.0 } else { 0.0 };
                if v1338 != 0.0 {
                } else {
                }
            } else {
            }
            let v1341 = if (if v265 > v0 { 1.0 } else { 0.0 }) != 0.0 || (if v282 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            if v1341 != 0.0 {
                let v1343 = if v635 < v1342 { 1.0 } else { 0.0 };
                if v1343 != 0.0 {
                } else {
                }
                let v1345 = if v635 < v1344 { 1.0 } else { 0.0 };
                if v1345 != 0.0 {
                } else {
                }
            } else {
            }
            let v1363 = v610 * (v1346 + (v1356 * v616));
            let v1364 = v610 * (v1350 + (v1356 * v619));
            let v1365 = v610 * v948;
            let v1366 = v610 * (v1354 + (v1356 * v630));
            let v1367 = v610 * v1287;
            let v1368 = v610 * v1355;
            let v1369 = if v544 > v0 { 1.0 } else { 0.0 };
            if v1369 != 0.0 {
                let v1371 = (-v541) * v637;
                let v1373 = if v1372 <= v0 { 1.0 } else { 0.0 };
                if v1373 != 0.0 {
                    let v1375 = if (v635 + v1371) > v0 { 1.0 } else { 0.0 };
                    if v1375 != 0.0 {
                    } else {
                    }
                } else {
                }
            } else {
            }
            if v640 != 0.0 {
                let v1377 = if (v619 + v638) > v0 { 1.0 } else { 0.0 };
                if v1377 != 0.0 {
                } else {
                }
            } else {
            }
            if v711 != 0.0 {
                let v1379 = if (v630 + v709) > v0 { 1.0 } else { 0.0 };
                if v1379 != 0.0 {
                } else {
                    let v1383 = if (if v730 > v0 { 1.0 } else { 0.0 }) != 0.0 && (if v630 < (-v730) { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    if v1383 != 0.0 {
                    } else {
                    }
                }
            } else {
                let v1386 = if (if v730 > v0 { 1.0 } else { 0.0 }) != 0.0 && (if v760 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                if v1386 != 0.0 {
                } else {
                }
            }
            let v1387 = if v884 > v0 { 1.0 } else { 0.0 };
            if v1387 != 0.0 {
            } else {
            }
            let v1391 = if ((v622 * v34) / v1389) < v18 { 1.0 } else { 0.0 };
            if v1391 != 0.0 {
            } else {
            }
            let v1447: f64;
            let v1448: f64;
            let v1449: f64;
            let v1450: f64;
            let v1451: f64;
            let v1452: f64;
            let v1453: f64;
            let v1454: f64;
            let v1455: f64;
            let v1456: f64;
            let v1457: f64;
            let v1458: f64;
            let v1459: f64;
            let v1460: f64;
            let v1461: f64;
            let v1462: f64;
            let v1463: f64;
            let v1464: f64;
            let v1465: f64;
            let v1466: f64;
            let v1467: f64;
            let v1468: f64;
            let v1469: f64;
            let v1470: f64;
            let v1471: f64;
            let v1472: f64;
            let v1473: f64;
            let v1474: f64;
            let v1475: f64;
            let v1476: f64;
            let v1477: f64;
            let v1478: f64;
            let v1479: f64;
            if v1392 != 0.0 {
                let v1394 = v1363.abs();
                let v1395 = v1393 * v1394;
                let v1399 = v1396 * (v1394.powf(v1397));
                let v1402 = v1364.abs();
                let v1403 = v1401 * v1402;
                let v1405 = v1396 * (v1402.powf(v1397));
                let v1408 = v1406 * (v1365.abs());
                let v1410 = v1366.abs();
                let v1411 = v1409 * v1410;
                let v1413 = v1396 * (v1410.powf(v1397));
                let v1416 = (v1414 * v315) * v574;
                let v1426 = (v1417 * v315) * (((v1367.abs()) + (v1420 * v578)) / ((v632.abs()) + v1420));
                let v1429 = (v1427 * v315) * v582;
                let v1433 = ((v1430 * v315) * v945) * v586;
                let v1436 = (v1434 * v315) * v589;
                let v1440 = ((v1437 * v315) * v1288) * v593;
                let v1443 = v1441 * (v1368.abs());
                let v1446 = (v1444 * v315) * v596;
                v1447 = v2;
                v1448 = v1395;
                v1449 = v2;
                v1450 = v1399;
                v1451 = v1400;
                v1452 = v2;
                v1453 = v1403;
                v1454 = v2;
                v1455 = v1405;
                v1456 = v1400;
                v1457 = v2;
                v1458 = v1408;
                v1459 = v2;
                v1460 = v1411;
                v1461 = v2;
                v1462 = v1413;
                v1463 = v1400;
                v1464 = v2;
                v1465 = v1416;
                v1466 = v2;
                v1467 = v1426;
                v1468 = v2;
                v1469 = v1429;
                v1470 = v2;
                v1471 = v1433;
                v1472 = v2;
                v1473 = v1436;
                v1474 = v2;
                v1475 = v1440;
                v1476 = v2;
                v1477 = v1443;
                v1478 = v2;
                v1479 = v1446;
            } else {
                v1447 = v0;
                v1448 = v0;
                v1449 = v0;
                v1450 = v0;
                v1451 = v0;
                v1452 = v0;
                v1453 = v0;
                v1454 = v0;
                v1455 = v0;
                v1456 = v0;
                v1457 = v0;
                v1458 = v0;
                v1459 = v0;
                v1460 = v0;
                v1461 = v0;
                v1462 = v0;
                v1463 = v0;
                v1464 = v0;
                v1465 = v0;
                v1466 = v0;
                v1467 = v0;
                v1468 = v0;
                v1469 = v0;
                v1470 = v0;
                v1471 = v0;
                v1472 = v0;
                v1473 = v0;
                v1474 = v0;
                v1475 = v0;
                v1476 = v0;
                v1477 = v0;
                v1478 = v0;
                v1479 = v0;
            }
        if v1447 == 0.0 {
            if !visitor.visit(0, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v1448;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 0, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 0, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 0, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(0, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v1449 == 0.0 {
            if !visitor.visit(1, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v1450;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 1, value: psd }); }
            let exponent: Option<f64> = Some(v1451);
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(1, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v1452 == 0.0 {
            if !visitor.visit(2, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v1453;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 2, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 2, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 2, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(2, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v1454 == 0.0 {
            if !visitor.visit(3, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v1455;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 3, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 3, value: psd }); }
            let exponent: Option<f64> = Some(v1456);
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 3, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 3, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(3, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v1457 == 0.0 {
            if !visitor.visit(4, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v1458;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 4, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 4, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 4, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(4, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v1459 == 0.0 {
            if !visitor.visit(5, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v1460;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 5, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 5, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 5, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(5, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v1461 == 0.0 {
            if !visitor.visit(6, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v1462;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 6, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 6, value: psd }); }
            let exponent: Option<f64> = Some(v1463);
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 6, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 6, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(6, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v1464 == 0.0 {
            if !visitor.visit(7, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v1465;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 7, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 7, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 7, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(7, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v1466 == 0.0 {
            if !visitor.visit(8, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v1467;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 8, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 8, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 8, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(8, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v1468 == 0.0 {
            if !visitor.visit(9, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v1469;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 9, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 9, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 9, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(9, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v1470 == 0.0 {
            if !visitor.visit(10, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v1471;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 10, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 10, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 10, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(10, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v1472 == 0.0 {
            if !visitor.visit(11, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v1473;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 11, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 11, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 11, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(11, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v1474 == 0.0 {
            if !visitor.visit(12, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v1475;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 12, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 12, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 12, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(12, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v1476 == 0.0 {
            if !visitor.visit(13, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v1477;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 13, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 13, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 13, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(13, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v1478 == 0.0 {
            if !visitor.visit(14, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v1479;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 14, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 14, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 14, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(14, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        Ok(())
    }
}
