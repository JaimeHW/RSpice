#![allow(dead_code, non_snake_case, unused_parens, unused_variables)]

use super::state::Instance;
use crate::device::veriloga_generated::GeneratedEvalContext;
pub use crate::device::veriloga_generated::{GeneratedNoiseDescriptor, GeneratedNoiseEndpoint, GeneratedNoiseEvaluation, GeneratedNoiseEvaluationError, GeneratedNoiseEvaluationRef, GeneratedNoiseKind, GeneratedNoiseVisitor};

pub static NOISE_SOURCES: [GeneratedNoiseDescriptor; 13] = [
    GeneratedNoiseDescriptor { mechanism: "WHITE_BI_EI_IBEI_SHOT_NOISE", label: Some("Ibei shot noise"), kind: GeneratedNoiseKind::White, equation: 27, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(7), name: "bi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(8), name: "ei", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_BI_EI_IBEI_FLICKER_NOISE", label: Some("Ibei flicker noise"), kind: GeneratedNoiseKind::Flicker, equation: 28, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(7), name: "bi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(8), name: "ei", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_BX_EI_IBEX_SHOT_NOISE", label: Some("Ibex shot noise"), kind: GeneratedNoiseKind::White, equation: 29, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(6), name: "bx", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(8), name: "ei", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_BX_EI_IBEX_FLICKER_NOISE", label: Some("Ibex flicker noise"), kind: GeneratedNoiseKind::Flicker, equation: 30, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(6), name: "bx", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(8), name: "ei", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_CI_EI_TRANSPORT_CURRENT_SHOT_NOISE", label: Some("transport current shot noise"), kind: GeneratedNoiseKind::White, equation: 31, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "ci", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(8), name: "ei", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_BX_BP_IBEP_SHOT_NOISE", label: Some("Ibep shot noise"), kind: GeneratedNoiseKind::White, equation: 32, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(6), name: "bx", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(9), name: "bp", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_BX_BP_IBEP_FLICKER_NOISE", label: Some("Ibep flicker noise"), kind: GeneratedNoiseKind::Flicker, equation: 33, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(6), name: "bx", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(9), name: "bp", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C_CX_RCX_THERMAL_NOISE", label: Some("rcx thermal noise"), kind: GeneratedNoiseKind::White, equation: 34, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(0), name: "c", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(4), name: "cx", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_CX_CI_RCI_THERMAL_NOISE", label: Some("rci thermal noise"), kind: GeneratedNoiseKind::White, equation: 35, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(4), name: "cx", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(5), name: "ci", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_B_BX_RBX_THERMAL_NOISE", label: Some("rbx thermal noise"), kind: GeneratedNoiseKind::White, equation: 36, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(1), name: "b", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(6), name: "bx", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_BX_BI_RBI_THERMAL_NOISE", label: Some("rbi thermal noise"), kind: GeneratedNoiseKind::White, equation: 37, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(6), name: "bx", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(7), name: "bi", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_E_EI_RE_THERMAL_NOISE", label: Some("re thermal noise"), kind: GeneratedNoiseKind::White, equation: 38, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(2), name: "e", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(8), name: "ei", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_BP_CX_RBP_THERMAL_NOISE", label: Some("rbp thermal noise"), kind: GeneratedNoiseKind::White, equation: 39, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(9), name: "bp", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(4), name: "cx", is_internal: true }, table_len: 0, table_log_interp: false },
];

impl Instance {
    pub fn evaluate_noise_sources(&self, ctx: &GeneratedEvalContext<'_>, visitor: &mut dyn GeneratedNoiseVisitor) -> Result<(), GeneratedNoiseEvaluationError> {
        if !self.multiplicity.is_finite() || self.multiplicity <= 0.0 {
            return Err(GeneratedNoiseEvaluationError::InvalidMultiplicity { value: self.multiplicity });
        }
        let parameters = &self.params.values;
        let parameter_given = &*self.param_given;
        let temperature = ctx.temperature();
        let node_potentials = [ctx.node_voltage(self.nodes[0]), ctx.node_voltage(self.nodes[1]), ctx.node_voltage(self.nodes[2]), ctx.node_voltage(self.nodes[3]), ctx.node_voltage(self.nodes[4]), ctx.node_voltage(self.nodes[5]), ctx.node_voltage(self.nodes[6]), ctx.node_voltage(self.nodes[7]), ctx.node_voltage(self.nodes[8]), ctx.node_voltage(self.nodes[9]), ctx.node_voltage(self.nodes[10]), ctx.node_voltage(self.nodes[11])];
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
            let v278 = parameters[68];
            let v279 = parameters[69];
            let v283 = parameters[119];
            let v291 = node_potentials[3];
            let v312 = parameters[126];
            let v315 = if parameter_given[109] { 1.0 } else { 0.0 };
            let v316 = parameters[16];
            let v317 = parameters[109];
            let v320 = parameters[107];
            let v323 = if parameter_given[108] { 1.0 } else { 0.0 };
            let v324 = parameters[17];
            let v325 = parameters[108];
            let v330 = if parameter_given[106] { 1.0 } else { 0.0 };
            let v331 = parameters[21];
            let v332 = parameters[106];
            let v335 = parameters[104];
            let v338 = if parameter_given[105] { 1.0 } else { 0.0 };
            let v339 = parameters[22];
            let v340 = parameters[105];
            let v345 = parameters[23];
            let v346 = parameters[103];
            let v349 = parameters[24];
            let v350 = parameters[111];
            let v353 = if parameter_given[110] { 1.0 } else { 0.0 };
            let v354 = parameters[25];
            let v355 = parameters[110];
            let v360 = parameters[101];
            let v361 = parameters[132];
            let v415 = parameters[129];
            let v420 = parameters[84];
            let v421 = parameters[127];
            let v425 = parameters[86];
            let v426 = parameters[128];
            let v430 = parameters[91];
            let v431 = parameters[92];
            let v437 = parameters[93];
            let v441 = 2e0f64;
            let v444 = parameters[37];
            let v449 = -5e-1f64;
            let v458 = 3e0f64;
            let v478 = parameters[42];
            let v483 = -5e-1f64;
            let v506 = parameters[38];
            let v507 = parameters[43];
            let v508 = parameters[19];
            let v514 = parameters[18];
            let v515 = parameters[112];
            let v522 = parameters[70];
            let v523 = parameters[130];
            let v527 = parameters[71];
            let v528 = parameters[131];
            let v533 = 1e-3f64;
            let v536 = 1e3f64;
            let v574 = node_potentials[7];
            let v575 = node_potentials[8];
            let v578 = node_potentials[6];
            let v581 = node_potentials[5];
            let v584 = node_potentials[4];
            let v589 = node_potentials[9];
            let v595 = parameters[34];
            let v597 = parameters[39];
            let v631 = -5e-1f64;
            let v668 = parameters[44];
            let v673 = -1e0f64;
            let v688 = parameters[45];
            let v718 = parameters[46];
            let v798 = -5e-1f64;
            let v867 = 1e-4f64;
            let v870 = 1e-8f64;
            let v879 = parameters[30];
            let v929 = parameters[32];
            let v945 = 5.0005e-1f64;
            let v946 = parameters[55];
            let v970 = parameters[57];
            let v1233 = parameters[83];
            let v1235 = 2e-2f64;
            let v1238 = 1.01e0f64;
            let v1255 = parameters[85];
            let v1259 = parameters[87];
            let v1276 = parameters[97];
            let v1278 = parameters[95];
            let v1281 = parameters[94];
            let v1317 = 1.44e0f64;
            let v1320 = parameters[1];
            let v1321 = 3.204378e-19f64;
            let v1324 = parameters[98];
            let v1325 = parameters[99];
            let v1328 = parameters[100];
            let v1329 = 3.204378e-19f64;
            let v1334 = 3.204378e-19f64;
            let v1337 = 3.204378e-19f64;
            let v1342 = 5.522648e-23f64;
            let v1345 = 5.522648e-23f64;
            let v1348 = 1e-10f64;
            let v1355 = 5.522648e-23f64;
            let v1358 = 5.522648e-23f64;
            let v1362 = 5.522648e-23f64;
            let v1365 = 5.522648e-23f64;
            if v1 != 0.0 {
            } else {
            }
            if v3 != 0.0 {
            } else {
            }
            let v1292: f64;
            if v5 != 0.0 {
                v1292 = v6;
            } else {
                let v8 = ctx.simparam_or("gmin", v7);
                v1292 = v8;
            }
            let v79: f64;
            if v9 != 0.0 {
                v79 = v10;
            } else {
                let v11 = ctx.simparam_or("pnjmaxi", v2);
                v79 = v11;
            }
            let v571: f64;
            if v12 != 0.0 {
                v571 = v2;
            } else {
                let v572: f64;
                if v13 != 0.0 {
                    v572 = v14;
                } else {
                    let v573: f64;
                    if v15 != 0.0 {
                        v573 = v16;
                    } else {
                        v573 = v2;
                    }
                    v572 = v573;
                }
                v571 = v572;
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
            let v991: f64;
            if v72 != 0.0 {
                let v74 = v73 * v68;
                let v83 = v74 * (((((-v75) / v74).exp()) + (v79 / v71)).ln());
                v991 = v83;
            } else {
                v991 = v0;
            }
            let v86 = v84 / v85;
            let v90 = -v89;
            let v91 = v2 - v69;
            let v93 = v68 * v85;
            let v96 = (v70 * (v69.powf(v86))) * (((v90 * v91) / v93).exp());
            let v97 = if v96 > v0 { 1.0 } else { 0.0 };
            let v829: f64;
            if v97 != 0.0 {
                let v101 = if (if v98 > v0 { 1.0 } else { 0.0 }) != 0.0 && (if v79 > v98 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v830: f64;
                if v101 != 0.0 {
                    let v115 = v93 * ((v2 + ((((v102 * v79) * ((v104 / v98).powf(v106))).powf((v2 / (v2 - v106)))) / v96)).ln());
                    v830 = v115;
                } else {
                    let v119 = v93 * ((v2 + (v79 / v96)).ln());
                    v830 = v119;
                }
                v829 = v830;
            } else {
                v829 = v0;
            }
            let v123 = v121 / v122;
            let v127 = -v126;
            let v129 = v68 * v122;
            let v132 = (v120 * (v69.powf(v123))) * (((v127 * v91) / v129).exp());
            let v134 = if v97 != 0.0 && (if v132 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let v845: f64;
            if v134 != 0.0 {
                let v136 = if v20 != 0.0 && (if v79 > v19 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v846: f64;
                if v136 != 0.0 {
                    let v148 = v129 * ((v2 + ((((v102 * v79) * ((v104 / v19).powf(v106))).powf((v2 / (v2 - v106)))) / (v96 * v132))).ln());
                    v846 = v148;
                } else {
                    let v153 = v129 * ((v2 + (v79 / (v96 * v132))).ln());
                    v846 = v153;
                }
                v845 = v846;
            } else {
                v845 = v0;
            }
            let v156 = v84 / v155;
            let v160 = -v159;
            let v162 = v68 * v155;
            let v165 = (v154 * (v69.powf(v156))) * (((v160 * v91) / v162).exp());
            let v166 = if v165 > v0 { 1.0 } else { 0.0 };
            let v909: f64;
            if v166 != 0.0 {
                let v168 = if v24 != 0.0 && (if v79 > v23 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v910: f64;
                if v168 != 0.0 {
                    let v174 = v162 * ((v2 + (((v79 * v79) * v26) / v165)).ln());
                    v910 = v174;
                } else {
                    let v178 = v162 * ((v2 + (v79 / v165)).ln());
                    v910 = v178;
                }
                v909 = v910;
            } else {
                v909 = v0;
            }
            let v182 = v180 / v181;
            let v186 = -v185;
            let v188 = v68 * v181;
            let v191 = (v179 * (v69.powf(v182))) * (((v186 * v91) / v188).exp());
            let v192 = if v191 > v0 { 1.0 } else { 0.0 };
            let v949: f64;
            if v192 != 0.0 {
                let v196 = v188 * ((v2 + (v79 / v191)).ln());
                v949 = v196;
            } else {
                v949 = v0;
            }
            let v200 = v198 / v199;
            let v204 = -v203;
            let v206 = v68 * v199;
            let v209 = (v197 * (v69.powf(v200))) * (((v204 * v91) / v206).exp());
            let v210 = if v209 > v0 { 1.0 } else { 0.0 };
            let v960: f64;
            if v210 != 0.0 {
                let v214 = v206 * ((v2 + (v79 / v209)).ln());
                v960 = v214;
            } else {
                v960 = v0;
            }
            let v217 = v180 / v216;
            let v218 = v69.powf(v217);
            let v221 = -v220;
            let v223 = v68 * v216;
            let v225 = ((v221 * v91) / v223).exp();
            let v226 = (v215 * v218) * v225;
            let v227 = if v226 > v0 { 1.0 } else { 0.0 };
            let v1151: f64;
            if v227 != 0.0 {
                let v231 = v223 * ((v2 + (v79 / v226)).ln());
                v1151 = v231;
            } else {
                v1151 = v0;
            }
            let v234 = v198 / v233;
            let v235 = v69.powf(v234);
            let v238 = -v237;
            let v240 = v68 * v233;
            let v242 = ((v238 * v91) / v240).exp();
            let v243 = (v232 * v235) * v242;
            let v244 = if v243 > v0 { 1.0 } else { 0.0 };
            let v1154: f64;
            if v244 != 0.0 {
                let v248 = v240 * ((v2 + (v79 / v243)).ln());
                v1154 = v248;
            } else {
                v1154 = v0;
            }
            let v251 = (v249 * v218) * v225;
            let v252 = if v251 > v0 { 1.0 } else { 0.0 };
            let v1159: f64;
            if v252 != 0.0 {
                let v256 = v223 * ((v2 + (v79 / v251)).ln());
                v1159 = v256;
            } else {
                v1159 = v0;
            }
            let v259 = (v257 * v235) * v242;
            let v260 = if v259 > v0 { 1.0 } else { 0.0 };
            let v1169: f64;
            if v260 != 0.0 {
                let v264 = v240 * ((v2 + (v79 / v259)).ln());
                v1169 = v264;
            } else {
                v1169 = v0;
            }
            let v277 = if ((v265 * (v69.powf((v180 / v266)))) * ((((-v270) * v91) / (v68 * v266)).exp())) > v0 { 1.0 } else { 0.0 };
            if v277 != 0.0 {
            } else {
            }
            let v290 = if ((v278 * (v69.powf((v198 / v279)))) * ((((-v283) * v91) / (v68 * v279)).exp())) > v0 { 1.0 } else { 0.0 };
            if v290 != 0.0 {
            } else {
            }
            let v293 = (v42 + v291) - v37;
            let v294 = if v293 < v49 { 1.0 } else { 0.0 };
            let v305: f64;
            if v294 != 0.0 {
                let v298 = v48 + (((v293 - v48) - v2).exp());
                v305 = v298;
            } else {
                let v300 = if v293 > (v55 - v2) { 1.0 } else { 0.0 };
                let v306: f64;
                if v300 != 0.0 {
                    let v304 = v55 - (((v55 - v293) - v2).exp());
                    v306 = v304;
                } else {
                    v306 = v293;
                }
                v305 = v306;
            }
            let v307 = v305 + v37;
            let v309 = (v65 * v307) / v67;
            let v310 = v307 / v39;
            let v311 = v307 - v39;
            let v314 = v98 * (v310.powf(v312));
            let v532: f64;
            if v315 != 0.0 {
                let v319 = v316 * (v310.powf(v317));
                v532 = v319;
            } else {
                let v322 = v316 * (v310.powf(v320));
                v532 = v322;
            }
            let v538: f64;
            if v323 != 0.0 {
                let v327 = v324 * (v310.powf(v325));
                v538 = v327;
            } else {
                let v329 = v324 * (v310.powf(v320));
                v538 = v329;
            }
            let v542: f64;
            if v330 != 0.0 {
                let v334 = v331 * (v310.powf(v332));
                v542 = v334;
            } else {
                let v337 = v331 * (v310.powf(v335));
                v542 = v337;
            }
            let v546: f64;
            if v338 != 0.0 {
                let v342 = v339 * (v310.powf(v340));
                v546 = v342;
            } else {
                let v344 = v339 * (v310.powf(v335));
                v546 = v344;
            }
            let v348 = v345 * (v310.powf(v346));
            let v352 = v349 * (v310.powf(v350));
            let v553: f64;
            if v353 != 0.0 {
                let v357 = v354 * (v310.powf(v355));
                v553 = v357;
            } else {
                let v359 = v354 * (v310.powf(v320));
                v553 = v359;
            }
            let v364 = v360 * (v2 + (v311 * v361));
            let v367 = v2 - v310;
            let v368 = v90 * v367;
            let v372 = (v70 * (v310.powf(v86))) * ((v368 / (v309 * v85)).exp());
            let v379 = (v120 * (v310.powf(v123))) * (((v127 * v367) / (v309 * v122)).exp());
            let v383 = v309 * v155;
            let v386 = (v154 * (v310.powf(v156))) * (((v160 * v367) / v383).exp());
            let v390 = v309 * v181;
            let v393 = (v179 * (v310.powf(v182))) * (((v186 * v367) / v390).exp());
            let v397 = v309 * v199;
            let v400 = (v197 * (v310.powf(v200))) * (((v204 * v367) / v397).exp());
            let v403 = v309 * v216;
            let v408 = v309 * v233;
            let v412 = (v249 * (v310.powf(v217))) * (((v221 * v367) / v403).exp());
            let v414 = (v257 * (v310.powf(v234))) * (((v238 * v367) / v408).exp());
            let v417 = v2 + (v311 * v415);
            let v418 = v85 * v417;
            let v419 = v122 * v417;
            let v424 = v420 * (v2 + (v311 * v421));
            let v429 = v425 * (v2 + (v311 * v426));
            let v443 = v441 * (v309 / v310);
            let v461 = (v458 * v309) * (v310.ln());
            let v463 = v310 - v2;
            let v465 = (((v443 * ((((((v102 * v444) * v310) / v309).exp()) - ((((v449 * v444) * v310) / v309).exp())).ln())) * v310) - v461) - (v185 * v463);
            let v466 = v441 * v309;
            let v477 = v465 + (v466 * ((v102 * (v2 + ((v2 + (v104 * (((-v465) / v309).exp()))).sqrt()))).ln()));
            let v494 = (((v443 * ((((((v102 * v478) * v310) / v309).exp()) - ((((v483 * v478) * v310) / v309).exp())).ln())) * v310) - v461) - (v220 * v463);
            let v505 = v494 + (v466 * ((v102 * (v2 + ((v2 + (v104 * (((-v494) / v309).exp()))).sqrt()))).ln()));
            let v513 = (v508 * (v310.powf(v84))) * ((v368 / v309).exp());
            let v517 = v514 * (v310.powf(v515));
            let v518 = -(v75 * (v2 + (v311 * (v430 + (v311 * v431)))));
            let v519 = (v73 * (v2 + (v311 * v437))) * v309;
            let v521 = (v518 / v519).exp();
            let v526 = v522 * (v2 + (v311 * v523));
            let v531 = v527 * (v2 + (v311 * v528));
            let v534 = if v532 > v533 { 1.0 } else { 0.0 };
            let v537: f64;
            if v534 != 0.0 {
                let v535 = v2 / v532;
                v537 = v535;
            } else {
                v537 = v536;
            }
            let v539 = if v538 > v533 { 1.0 } else { 0.0 };
            let v541: f64;
            if v539 != 0.0 {
                let v540 = v2 / v538;
                v541 = v540;
            } else {
                v541 = v536;
            }
            let v543 = if v542 > v533 { 1.0 } else { 0.0 };
            let v545: f64;
            if v543 != 0.0 {
                let v544 = v2 / v542;
                v545 = v544;
            } else {
                v545 = v536;
            }
            let v547 = if v546 > v533 { 1.0 } else { 0.0 };
            let v549: f64;
            if v547 != 0.0 {
                let v548 = v2 / v546;
                v549 = v548;
            } else {
                v549 = v536;
            }
            let v550 = if v348 > v533 { 1.0 } else { 0.0 };
            let v552: f64;
            if v550 != 0.0 {
                let v551 = v2 / v348;
                v552 = v551;
            } else {
                v552 = v536;
            }
            let v554 = if v553 > v533 { 1.0 } else { 0.0 };
            let v556: f64;
            if v554 != 0.0 {
                let v555 = v2 / v553;
                v556 = v555;
            } else {
                v556 = v536;
            }
            let v557 = if v352 > v533 { 1.0 } else { 0.0 };
            if v557 != 0.0 {
            } else {
            }
            let v558 = if v364 > v533 { 1.0 } else { 0.0 };
            if v558 != 0.0 {
            } else {
            }
            let v559 = if v526 > v0 { 1.0 } else { 0.0 };
            let v561: f64;
            if v559 != 0.0 {
                let v560 = v2 / v526;
                v561 = v560;
            } else {
                v561 = v0;
            }
            let v562 = if v531 > v0 { 1.0 } else { 0.0 };
            let v564: f64;
            if v562 != 0.0 {
                let v563 = v2 / v531;
                v564 = v563;
            } else {
                v564 = v0;
            }
            let v565 = if v314 > v0 { 1.0 } else { 0.0 };
            let v567: f64;
            if v565 != 0.0 {
                let v566 = v2 / v314;
                v567 = v566;
            } else {
                v567 = v0;
            }
            let v568 = if v517 > v0 { 1.0 } else { 0.0 };
            let v570: f64;
            if v568 != 0.0 {
                let v569 = v2 / v517;
                v570 = v569;
            } else {
                v570 = v0;
            }
            let v577 = v571 * (v574 - v575);
            let v580 = v571 * (v578 - v575);
            let v583 = v571 * (v574 - v581);
            let v586 = v571 * (v574 - v584);
            let v588 = v571 * (v578 - v584);
            let v591 = v571 * (v578 - v589);
            let v593 = v571 * (v584 - v581);
            let v594 = -v477;
            let v596 = v594 * v595;
            let v598 = if v597 <= v0 { 1.0 } else { 0.0 };
            let v860: f64;
            if v598 != 0.0 {
                let v599 = v577 + v596;
                let v600 = if v599 > v0 { 1.0 } else { 0.0 };
                let v623: f64;
                let v624: f64;
                if v600 != 0.0 {
                    let v601 = v2 - v595;
                    let v603 = v601.powf((-v506));
                    let v608 = (v477 * (v2 - (v603 * v601))) / (v2 - v506);
                    let v615 = (v599 * (v2 + (((v102 * v506) * v599) / (v477 * v601)))) * v603;
                    v623 = v608;
                    v624 = v615;
                } else {
                    let v618 = v2 - v506;
                    let v622 = (v477 * (v2 - ((v2 - (v577 / v477)).powf(v618)))) / v618;
                    v623 = v622;
                    v624 = v0;
                }
                let v625 = v623 + v624;
                v860 = v625;
            } else {
                let v628 = (v104 * v597) * v597;
                let v633 = v631 * (v596 + (((v596 * v596) + v628).sqrt()));
                let v636 = v2 - v506;
                let v640 = v577 + v596;
                let v646 = (v102 * (v640 - (((v640 * v640) + v628).sqrt()))) - v596;
                let v652 = v2 - v595;
                let v656 = (v577 - v646) + v633;
                let v665 = (((v594 * ((v2 - (v646 / v477)).powf(v636))) / v636) + (((v652.powf((-v506))) * v656) * (v2 + (((v102 * v506) * v656) / (v477 * v652))))) - ((v594 * ((v2 - (v633 / v477)).powf(v636))) / v636);
                v860 = v665;
            }
            let v666 = -v505;
            let v667 = v666 * v595;
            let v669 = if v668 <= v0 { 1.0 } else { 0.0 };
            let v863: f64;
            if v669 != 0.0 {
                let v670 = v583 + v667;
                let v671 = if v670 > v0 { 1.0 } else { 0.0 };
                let v713: f64;
                let v715: f64;
                if v671 != 0.0 {
                    let v672 = v2 - v595;
                    let v675 = v672.powf((v673 - v507));
                    let v681 = (v505 * (v2 - ((v675 * v672) * v672))) / (v2 - v507);
                    let v687 = (v670 * (v672 + (((v102 * v507) * v670) / v505))) * v675;
                    v713 = v681;
                    v715 = v687;
                } else {
                    let v692 = if (if v688 > v0 { 1.0 } else { 0.0 }) != 0.0 && (if v583 < (-v688) { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let v714: f64;
                    if v692 != 0.0 {
                        let v695 = v2 - v507;
                        let v705 = (v505 * (v2 - (((v2 + (v688 / v505)).powf(v695)) * (v2 - ((v695 * (v583 + v688)) / (v505 + v688)))))) / v695;
                        v714 = v705;
                    } else {
                        let v708 = v2 - v507;
                        let v712 = (v505 * (v2 - ((v2 - (v583 / v505)).powf(v708)))) / v708;
                        v714 = v712;
                    }
                    v713 = v714;
                    v715 = v0;
                }
                let v716 = v713 + v715;
                v863 = v716;
            } else {
                let v720 = if (if v688 > v0 { 1.0 } else { 0.0 }) != 0.0 && (if v718 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v864: f64;
                if v720 != 0.0 {
                    let v722 = v688 - v667;
                    let v723 = (v688 + v667) / v722;
                    let v725 = v723 - v2;
                    let v728 = (v104 * v668) * v668;
                    let v731 = v723 + v2;
                    let v734 = (v104 * v718) * v718;
                    let v742 = v102 * (((((v441 * v723) / ((((v725 * v725) + v728).sqrt()) + (((v731 * v731) + v734).sqrt()))) * v722) - v688) - v667);
                    let v745 = v2 - v507;
                    let v753 = (((v441 * v583) + v688) + v667) / v722;
                    let v755 = v753 - v2;
                    let v759 = v753 + v2;
                    let v764 = (v441 * v753) / ((((v755 * v755) + v728).sqrt()) + (((v759 * v759) + v734).sqrt()));
                    let v768 = v102 * (((v764 * v722) - v688) - v667);
                    let v776 = v102 * (v764 + v2);
                    let v779 = -v507;
                    let v792 = ((((v583 - v768) + v742) * (((v2 - v776) * ((v2 + (v688 / v505)).powf(v779))) + (v776 * ((v2 + (v667 / v505)).powf(v779))))) + ((v505 * (v2 - ((v2 - (v768 / v505)).powf(v745)))) / v745)) - ((v505 * (v2 - ((v2 - (v742 / v505)).powf(v745)))) / v745);
                    v864 = v792;
                } else {
                    let v795 = (v104 * v668) * v668;
                    let v800 = v798 * (v667 + (((v667 * v667) + v795).sqrt()));
                    let v803 = v2 - v507;
                    let v807 = v583 + v667;
                    let v813 = (v102 * (v807 - (((v807 * v807) + v795).sqrt()))) - v667;
                    let v826 = (((v666 * ((v2 - (v813 / v505)).powf(v803))) / v803) + (((v2 - v595).powf((-v507))) * ((v583 - v813) + v800))) - ((v666 * ((v2 - (v800 / v505)).powf(v803))) / v803);
                    v864 = v826;
                }
                v863 = v864;
            }
            let v828 = v2 / (v418 * v309);
            let v831 = if v577 < v829 { 1.0 } else { 0.0 };
            let v840: f64;
            if v831 != 0.0 {
                let v833 = (v577 * v828).exp();
                v840 = v833;
            } else {
                let v839 = ((v829 * v828).exp()) * (v2 + ((v577 - v829) * v828));
                v840 = v839;
            }
            let v842 = v372 * (v840 - v2);
            let v844 = v2 / (v419 * v309);
            let v847 = if v583 < v845 { 1.0 } else { 0.0 };
            let v857: f64;
            if v847 != 0.0 {
                let v849 = (v583 * v844).exp();
                v857 = v849;
            } else {
                let v855 = ((v845 * v844).exp()) * (v2 + ((v583 - v845) * v844));
                v857 = v855;
            }
            let v868 = ((v2 + (v860 * v564)) + (v863 * v561)) - v867;
            let v875 = (v102 * ((((v868 * v868) + v870).sqrt()) + v868)) + v867;
            let v878 = (v842 * v567) + (((v372 * v379) * (v857 - v2)) * v22);
            let v880 = if v879 < v102 { 1.0 } else { 0.0 };
            let v903: f64;
            if v880 != 0.0 {
                let v884 = (v875.powf((v2 / v106))) + (v104 * v878);
                let v885 = if v884 > v870 { 1.0 } else { 0.0 };
                let v904: f64;
                if v885 != 0.0 {
                    let v888 = v102 * (v875 + (v884.powf(v106)));
                    v904 = v888;
                } else {
                    let v891 = v102 * (v875 + (v870.powf(v106)));
                    v904 = v891;
                }
                v903 = v904;
            } else {
                let v893 = v2 + (v104 * v878);
                let v894 = if v893 > v870 { 1.0 } else { 0.0 };
                let v905: f64;
                if v894 != 0.0 {
                    let v898 = (v102 * v875) * (v2 + (v893.powf(v106)));
                    v905 = v898;
                } else {
                    let v902 = (v102 * v875) * (v2 + (v870.powf(v106)));
                    v905 = v902;
                }
                v903 = v905;
            }
            let v906 = v842 / v903;
            let v907 = if v154 > v0 { 1.0 } else { 0.0 };
            let v1231: f64;
            if v907 != 0.0 {
                let v908 = v2 / v383;
                let v911 = if v591 < v909 { 1.0 } else { 0.0 };
                let v930: f64;
                if v911 != 0.0 {
                    let v913 = (v591 * v908).exp();
                    v930 = v913;
                } else {
                    let v919 = ((v909 * v908).exp()) * (v2 + ((v591 - v909) * v908));
                    v930 = v919;
                }
                let v920 = if v583 < v909 { 1.0 } else { 0.0 };
                let v933: f64;
                if v920 != 0.0 {
                    let v922 = (v583 * v908).exp();
                    v933 = v922;
                } else {
                    let v928 = ((v909 * v908).exp()) * (v2 + ((v583 - v909) * v908));
                    v933 = v928;
                }
                let v940 = v2 + (v104 * ((v386 * (((v929 * v930) + ((v2 - v929) * v933)) - v2)) * v26));
                let v941 = if v940 > v870 { 1.0 } else { 0.0 };
                let v1232: f64;
                if v941 != 0.0 {
                    let v944 = v102 * (v2 + (v940.sqrt()));
                    v1232 = v944;
                } else {
                    v1232 = v945;
                }
                v1231 = v1232;
            } else {
                v1231 = v2;
            }
            let v947 = if v946 == v2 { 1.0 } else { 0.0 };
            let v1283: f64;
            let v1287: f64;
            if v947 != 0.0 {
                let v948 = v2 / v390;
                let v950 = if v577 < v949 { 1.0 } else { 0.0 };
                let v976: f64;
                if v950 != 0.0 {
                    let v952 = (v577 * v948).exp();
                    v976 = v952;
                } else {
                    let v958 = ((v949 * v948).exp()) * (v2 + ((v577 - v949) * v948));
                    v976 = v958;
                }
                let v959 = v2 / v397;
                let v961 = if v577 < v960 { 1.0 } else { 0.0 };
                let v979: f64;
                if v961 != 0.0 {
                    let v963 = (v577 * v959).exp();
                    v979 = v963;
                } else {
                    let v969 = ((v960 * v959).exp()) * (v2 + ((v577 - v960) * v959));
                    v979 = v969;
                }
                let v971 = if v970 > v0 { 1.0 } else { 0.0 };
                let v1001: f64;
                if v971 != 0.0 {
                    let v982 = ((v393 * (v2 + (v970 * (v875 - v2)))) * (v976 - v2)) + (v400 * (v979 - v2));
                    v1001 = v982;
                } else {
                    let v987 = (v393 * (v976 - v2)) + (v400 * (v979 - v2));
                    v1001 = v987;
                }
                let v988 = if v75 > v0 { 1.0 } else { 0.0 };
                let v1284: f64;
                if v988 != 0.0 {
                    let v989 = v518 - v577;
                    let v990 = v2 / v519;
                    let v992 = if v989 < v991 { 1.0 } else { 0.0 };
                    let v1002: f64;
                    if v992 != 0.0 {
                        let v994 = (v989 * v990).exp();
                        v1002 = v994;
                    } else {
                        let v1000 = ((v991 * v990).exp()) * (v2 + ((v989 - v991) * v990));
                        v1002 = v1000;
                    }
                    let v1005 = v1001 - (v71 * (v1002 - v521));
                    v1284 = v1005;
                } else {
                    v1284 = v1001;
                }
                v1283 = v1284;
                v1287 = v0;
            } else {
                let v1006 = if v946 == v0 { 1.0 } else { 0.0 };
                let v1285: f64;
                let v1288: f64;
                if v1006 != 0.0 {
                    let v1007 = v2 / v390;
                    let v1008 = if v580 < v949 { 1.0 } else { 0.0 };
                    let v1027: f64;
                    if v1008 != 0.0 {
                        let v1010 = (v580 * v1007).exp();
                        v1027 = v1010;
                    } else {
                        let v1016 = ((v949 * v1007).exp()) * (v2 + ((v580 - v949) * v1007));
                        v1027 = v1016;
                    }
                    let v1017 = v2 / v397;
                    let v1018 = if v580 < v960 { 1.0 } else { 0.0 };
                    let v1030: f64;
                    if v1018 != 0.0 {
                        let v1020 = (v580 * v1017).exp();
                        v1030 = v1020;
                    } else {
                        let v1026 = ((v960 * v1017).exp()) * (v2 + ((v580 - v960) * v1017));
                        v1030 = v1026;
                    }
                    let v1033 = (v393 * (v1027 - v2)) + (v400 * (v1030 - v2));
                    let v1034 = if v75 > v0 { 1.0 } else { 0.0 };
                    let v1289: f64;
                    if v1034 != 0.0 {
                        let v1035 = v518 - v577;
                        let v1036 = v2 / v519;
                        let v1037 = if v1035 < v991 { 1.0 } else { 0.0 };
                        let v1046: f64;
                        if v1037 != 0.0 {
                            let v1039 = (v1035 * v1036).exp();
                            v1046 = v1039;
                        } else {
                            let v1045 = ((v991 * v1036).exp()) * (v2 + ((v1035 - v991) * v1036));
                            v1046 = v1045;
                        }
                        let v1049 = v1033 - (v71 * (v1046 - v521));
                        v1289 = v1049;
                    } else {
                        v1289 = v1033;
                    }
                    v1285 = v0;
                    v1288 = v1289;
                } else {
                    let v1050 = v2 / v390;
                    let v1051 = if v577 < v949 { 1.0 } else { 0.0 };
                    let v1075: f64;
                    if v1051 != 0.0 {
                        let v1053 = (v577 * v1050).exp();
                        v1075 = v1053;
                    } else {
                        let v1059 = ((v949 * v1050).exp()) * (v2 + ((v577 - v949) * v1050));
                        v1075 = v1059;
                    }
                    let v1060 = v2 / v397;
                    let v1061 = if v577 < v960 { 1.0 } else { 0.0 };
                    let v1078: f64;
                    if v1061 != 0.0 {
                        let v1063 = (v577 * v1060).exp();
                        v1078 = v1063;
                    } else {
                        let v1069 = ((v960 * v1060).exp()) * (v2 + ((v577 - v960) * v1060));
                        v1078 = v1069;
                    }
                    let v1070 = if v970 > v0 { 1.0 } else { 0.0 };
                    let v1101: f64;
                    if v1070 != 0.0 {
                        let v1082 = v946 * (((v393 * (v2 + (v970 * (v875 - v2)))) * (v1075 - v2)) + (v400 * (v1078 - v2)));
                        v1101 = v1082;
                    } else {
                        let v1088 = v946 * ((v393 * (v1075 - v2)) + (v400 * (v1078 - v2)));
                        v1101 = v1088;
                    }
                    let v1089 = if v75 > v0 { 1.0 } else { 0.0 };
                    let v1286: f64;
                    if v1089 != 0.0 {
                        let v1090 = v518 - v577;
                        let v1091 = v2 / v519;
                        let v1092 = if v1090 < v991 { 1.0 } else { 0.0 };
                        let v1103: f64;
                        if v1092 != 0.0 {
                            let v1094 = (v1090 * v1091).exp();
                            v1103 = v1094;
                        } else {
                            let v1100 = ((v991 * v1091).exp()) * (v2 + ((v1090 - v991) * v1091));
                            v1103 = v1100;
                        }
                        let v1106 = v1101 - ((v946 * v71) * (v1103 - v521));
                        v1286 = v1106;
                    } else {
                        v1286 = v1101;
                    }
                    let v1107 = if v580 < v949 { 1.0 } else { 0.0 };
                    let v1126: f64;
                    if v1107 != 0.0 {
                        let v1109 = (v580 * v1050).exp();
                        v1126 = v1109;
                    } else {
                        let v1115 = ((v949 * v1050).exp()) * (v2 + ((v580 - v949) * v1050));
                        v1126 = v1115;
                    }
                    let v1116 = if v580 < v960 { 1.0 } else { 0.0 };
                    let v1129: f64;
                    if v1116 != 0.0 {
                        let v1118 = (v580 * v1060).exp();
                        v1129 = v1118;
                    } else {
                        let v1124 = ((v960 * v1060).exp()) * (v2 + ((v580 - v960) * v1060));
                        v1129 = v1124;
                    }
                    let v1125 = v2 - v946;
                    let v1133 = v1125 * ((v393 * (v1126 - v2)) + (v400 * (v1129 - v2)));
                    let v1290: f64;
                    if v1089 != 0.0 {
                        let v1134 = v518 - v577;
                        let v1135 = v2 / v519;
                        let v1136 = if v1134 < v991 { 1.0 } else { 0.0 };
                        let v1146: f64;
                        if v1136 != 0.0 {
                            let v1138 = (v1134 * v1135).exp();
                            v1146 = v1138;
                        } else {
                            let v1144 = ((v991 * v1135).exp()) * (v2 + ((v1134 - v991) * v1135));
                            v1146 = v1144;
                        }
                        let v1149 = v1133 - ((v1125 * v71) * (v1146 - v521));
                        v1290 = v1149;
                    } else {
                        v1290 = v1133;
                    }
                    v1285 = v1286;
                    v1288 = v1290;
                }
                v1283 = v1285;
                v1287 = v1288;
            }
            let v1150 = v2 / v403;
            let v1152 = if v583 < v1151 { 1.0 } else { 0.0 };
            if v1152 != 0.0 {
            } else {
            }
            let v1153 = v2 / v408;
            let v1155 = if v583 < v1154 { 1.0 } else { 0.0 };
            if v1155 != 0.0 {
            } else {
            }
            let v1158 = if (if v249 > v0 { 1.0 } else { 0.0 }) != 0.0 || (if v257 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let v1291: f64;
            if v1158 != 0.0 {
                let v1160 = if v591 < v1159 { 1.0 } else { 0.0 };
                let v1179: f64;
                if v1160 != 0.0 {
                    let v1162 = (v591 * v1150).exp();
                    v1179 = v1162;
                } else {
                    let v1168 = ((v1159 * v1150).exp()) * (v2 + ((v591 - v1159) * v1150));
                    v1179 = v1168;
                }
                let v1170 = if v591 < v1169 { 1.0 } else { 0.0 };
                let v1182: f64;
                if v1170 != 0.0 {
                    let v1172 = (v591 * v1153).exp();
                    v1182 = v1172;
                } else {
                    let v1178 = ((v1169 * v1153).exp()) * (v2 + ((v591 - v1169) * v1153));
                    v1182 = v1178;
                }
                let v1185 = (v412 * (v1179 - v2)) + (v414 * (v1182 - v2));
                v1291 = v1185;
            } else {
                v1291 = v0;
            }
            let v1186 = v583 / v309;
            let v1187 = if v1186 < v18 { 1.0 } else { 0.0 };
            let v1200: f64;
            if v1187 != 0.0 {
                let v1188 = v1186.exp();
                v1200 = v1188;
            } else {
                let v1192 = (v18.exp()) * (v2 + (v1186 - v18));
                v1200 = v1192;
            }
            let v1193 = v586 / v309;
            let v1194 = if v1193 < v18 { 1.0 } else { 0.0 };
            let v1204: f64;
            if v1194 != 0.0 {
                let v1195 = v1193.exp();
                v1204 = v1195;
            } else {
                let v1199 = (v18.exp()) * (v2 + (v1193 - v18));
                v1204 = v1199;
            }
            let v1203 = (v2 + (v513 * v1200)).sqrt();
            let v1207 = (v2 + (v513 * v1204)).sqrt();
            let v1216 = (v593 + (v309 * ((v1203 - v1207) - (((v1203 + v2) / (v1207 + v2)).ln())))) * v541;
            let v1226 = (v570 * v1216) / (v541 * (v2 + (((v102 * v570) * v30) * (((v593 * v593) + v4).sqrt()))));
            let v1230 = v1216 / ((v2 + (v1226 * v1226)).sqrt());
            let v1234 = if v1233 > v0 { 1.0 } else { 0.0 };
            if v1234 != 0.0 {
                let v1241 = (v1235 * (v424 + v2)).powf((v2 / (v1238 - v507)));
                let v1243 = (v505 - v583) - v1241;
                let v1254 = if ((-v424) * (((v102 * ((((v1243 * v1243) + v4).sqrt()) + v1243)) + v1241).powf((v507 - v2)))) < v18 { 1.0 } else { 0.0 };
                if v1254 != 0.0 {
                } else {
                }
            } else {
            }
            let v1256 = if v1255 > v0 { 1.0 } else { 0.0 };
            if v1256 != 0.0 {
                let v1262 = (v1235 * (v429 + v2)).powf((v2 / (v1238 - v1259)));
                let v1264 = (v0 - v588) - v1262;
                let v1275 = if ((-v429) * (((v102 * ((((v1264 * v1264) + v4).sqrt()) + v1264)) + v1262).powf((v1259 - v2)))) < v18 { 1.0 } else { 0.0 };
                if v1275 != 0.0 {
                } else {
                }
            } else {
            }
            let v1280 = if (if v1276 > v0 { 1.0 } else { 0.0 }) != 0.0 && (if v1278 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            if v1280 != 0.0 {
                let v1282 = if v1281 > v0 { 1.0 } else { 0.0 };
                if v1282 != 0.0 {
                } else {
                }
            } else {
            }
            let v1299 = v571 * (v1283 + (v1292 * v577));
            let v1300 = v571 * (v1287 + (v1292 * v580));
            let v1301 = v571 * v906;
            let v1302 = v571 * (v1291 + (v1292 * v591));
            let v1303 = v571 * v1230;
            if v598 != 0.0 {
                let v1305 = if (v580 + v596) > v0 { 1.0 } else { 0.0 };
                if v1305 != 0.0 {
                } else {
                }
            } else {
            }
            if v669 != 0.0 {
                let v1307 = if (v591 + v667) > v0 { 1.0 } else { 0.0 };
                if v1307 != 0.0 {
                } else {
                    let v1311 = if (if v688 > v0 { 1.0 } else { 0.0 }) != 0.0 && (if v591 < (-v688) { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    if v1311 != 0.0 {
                    } else {
                    }
                }
            } else {
                let v1314 = if (if v688 > v0 { 1.0 } else { 0.0 }) != 0.0 && (if v718 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                if v1314 != 0.0 {
                } else {
                }
            }
            let v1315 = if v842 > v0 { 1.0 } else { 0.0 };
            if v1315 != 0.0 {
            } else {
            }
            let v1319 = if ((v583 * v34) / v1317) < v18 { 1.0 } else { 0.0 };
            if v1319 != 0.0 {
            } else {
            }
            let v1369: f64;
            let v1370: f64;
            let v1371: f64;
            let v1372: f64;
            let v1373: f64;
            let v1374: f64;
            let v1375: f64;
            let v1376: f64;
            let v1377: f64;
            let v1378: f64;
            let v1379: f64;
            let v1380: f64;
            let v1381: f64;
            let v1382: f64;
            let v1383: f64;
            let v1384: f64;
            let v1385: f64;
            let v1386: f64;
            let v1387: f64;
            let v1388: f64;
            let v1389: f64;
            let v1390: f64;
            let v1391: f64;
            let v1392: f64;
            let v1393: f64;
            let v1394: f64;
            let v1395: f64;
            let v1396: f64;
            let v1397: f64;
            if v1320 != 0.0 {
                let v1322 = v1299.abs();
                let v1323 = v1321 * v1322;
                let v1327 = v1324 * (v1322.powf(v1325));
                let v1330 = v1300.abs();
                let v1331 = v1329 * v1330;
                let v1333 = v1324 * (v1330.powf(v1325));
                let v1336 = v1334 * (v1301.abs());
                let v1338 = v1302.abs();
                let v1339 = v1337 * v1338;
                let v1341 = v1324 * (v1338.powf(v1325));
                let v1344 = (v1342 * v307) * v537;
                let v1354 = (v1345 * v307) * (((v1303.abs()) + (v1348 * v541)) / ((v593.abs()) + v1348));
                let v1357 = (v1355 * v307) * v545;
                let v1361 = ((v1358 * v307) * v903) * v549;
                let v1364 = (v1362 * v307) * v552;
                let v1368 = ((v1365 * v307) * v1231) * v556;
                v1369 = v2;
                v1370 = v1323;
                v1371 = v2;
                v1372 = v1327;
                v1373 = v1328;
                v1374 = v2;
                v1375 = v1331;
                v1376 = v2;
                v1377 = v1333;
                v1378 = v1328;
                v1379 = v2;
                v1380 = v1336;
                v1381 = v2;
                v1382 = v1339;
                v1383 = v2;
                v1384 = v1341;
                v1385 = v1328;
                v1386 = v2;
                v1387 = v1344;
                v1388 = v2;
                v1389 = v1354;
                v1390 = v2;
                v1391 = v1357;
                v1392 = v2;
                v1393 = v1361;
                v1394 = v2;
                v1395 = v1364;
                v1396 = v2;
                v1397 = v1368;
            } else {
                v1369 = v0;
                v1370 = v0;
                v1371 = v0;
                v1372 = v0;
                v1373 = v0;
                v1374 = v0;
                v1375 = v0;
                v1376 = v0;
                v1377 = v0;
                v1378 = v0;
                v1379 = v0;
                v1380 = v0;
                v1381 = v0;
                v1382 = v0;
                v1383 = v0;
                v1384 = v0;
                v1385 = v0;
                v1386 = v0;
                v1387 = v0;
                v1388 = v0;
                v1389 = v0;
                v1390 = v0;
                v1391 = v0;
                v1392 = v0;
                v1393 = v0;
                v1394 = v0;
                v1395 = v0;
                v1396 = v0;
                v1397 = v0;
            }
        if v1369 == 0.0 {
            if !visitor.visit(0, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v1370;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 0, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 0, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(0, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v1371 == 0.0 {
            if !visitor.visit(1, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v1372;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = Some(v1373);
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(1, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v1374 == 0.0 {
            if !visitor.visit(2, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v1375;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 2, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 2, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(2, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v1376 == 0.0 {
            if !visitor.visit(3, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v1377;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 3, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = Some(v1378);
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 3, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 3, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(3, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v1379 == 0.0 {
            if !visitor.visit(4, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v1380;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 4, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 4, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(4, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v1381 == 0.0 {
            if !visitor.visit(5, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v1382;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 5, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 5, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(5, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v1383 == 0.0 {
            if !visitor.visit(6, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v1384;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 6, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = Some(v1385);
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 6, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 6, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(6, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v1386 == 0.0 {
            if !visitor.visit(7, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v1387;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 7, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 7, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(7, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v1388 == 0.0 {
            if !visitor.visit(8, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v1389;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 8, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 8, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(8, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v1390 == 0.0 {
            if !visitor.visit(9, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v1391;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 9, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 9, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(9, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v1392 == 0.0 {
            if !visitor.visit(10, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v1393;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 10, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 10, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(10, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v1394 == 0.0 {
            if !visitor.visit(11, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v1395;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 11, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 11, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(11, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v1396 == 0.0 {
            if !visitor.visit(12, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v1397;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 12, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 12, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(12, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        Ok(())
    }
}
