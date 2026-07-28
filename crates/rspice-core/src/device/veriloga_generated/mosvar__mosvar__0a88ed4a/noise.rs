#![allow(dead_code, non_snake_case, unused_parens, unused_variables)]

use super::state::Instance;
use crate::device::veriloga_generated::GeneratedEvalContext;
pub use crate::device::veriloga_generated::{GeneratedNoiseDescriptor, GeneratedNoiseEndpoint, GeneratedNoiseEvaluation, GeneratedNoiseEvaluationError, GeneratedNoiseEvaluationRef, GeneratedNoiseKind, GeneratedNoiseVisitor};

pub static NOISE_SOURCES: [GeneratedNoiseDescriptor; 7] = [
    GeneratedNoiseDescriptor { mechanism: "WHITE_GI_CI_IGC", label: Some("Igc"), kind: GeneratedNoiseKind::White, equation: 15, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(4), name: "gi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(5), name: "ci", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_GI_BI_IGOV", label: Some("Igov"), kind: GeneratedNoiseKind::White, equation: 16, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(4), name: "gi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(1), name: "bi", is_internal: false }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_G_GII_RGSAL", label: Some("rgsal"), kind: GeneratedNoiseKind::White, equation: 17, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(0), name: "g", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(3), name: "gii", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_GII_GI_RGPV", label: Some("rgpv"), kind: GeneratedNoiseKind::White, equation: 18, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(3), name: "gii", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(4), name: "gi", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_BI_B_REND", label: Some("rend"), kind: GeneratedNoiseKind::White, equation: 19, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(1), name: "bi", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(2), name: "b", is_internal: false }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_CI_BI_RSUB", label: Some("rsub"), kind: GeneratedNoiseKind::White, equation: 20, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "ci", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(1), name: "bi", is_internal: false }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_CI_BI_RAC", label: Some("rac"), kind: GeneratedNoiseKind::White, equation: 21, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "ci", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(1), name: "bi", is_internal: false }, table_len: 0, table_log_interp: false },
];

impl Instance {
    pub fn evaluate_noise_sources(&self, ctx: &GeneratedEvalContext<'_>, visitor: &mut dyn GeneratedNoiseVisitor) -> Result<(), GeneratedNoiseEvaluationError> {
        if !self.multiplicity.is_finite() || self.multiplicity <= 0.0 {
            return Err(GeneratedNoiseEvaluationError::InvalidMultiplicity { value: self.multiplicity });
        }
        let parameters = &self.params.values;
        let temperature = ctx.temperature();
        let node_potentials = [ctx.node_voltage(self.nodes[0]), ctx.node_voltage(self.nodes[1]), ctx.node_voltage(self.nodes[2]), ctx.node_voltage(self.nodes[3]), ctx.node_voltage(self.nodes[4]), ctx.node_voltage(self.nodes[5]), ctx.node_voltage(self.nodes[6])];
            let v0 = 0e0f64;
            let v1 = parameters[7];
            let v2 = 1e3f64;
            let v4 = 3.453e-11f64;
            let v5 = parameters[20];
            let v6 = 3.9e0f64;
            let v9 = parameters[19];
            let v11 = 2e0f64;
            let v12 = 1.6021918e-19f64;
            let v13 = parameters[24];
            let v14 = 3.348580862e-29f64;
            let v15 = parameters[29];
            let v19 = 3.348580862e-29f64;
            let v20 = parameters[54];
            let v24 = parameters[30];
            let v26 = 2.3807972e0f64;
            let v28 = 6.666666666666666e-1f64;
            let v31 = parameters[17];
            let v33 = 1.2514650134837189e0f64;
            let v36 = 3.333333333333333e-1f64;
            let v37 = parameters[48];
            let v39 = 5e-1f64;
            let v41 = 1e-9f64;
            let v43 = parameters[11];
            let v44 = -2.73e2f64;
            let v46 = -2.73e2f64;
            let v48 = parameters[8];
            let v50 = parameters[9];
            let v52 = 2.7315e2f64;
            let v54 = temperature;
            let v55 = parameters[3];
            let v65 = 1.3806505e-23f64;
            let v68 = 1e2f64;
            let v71 = 1e0f64;
            let v73 = parameters[23];
            let v74 = parameters[42];
            let v77 = parameters[43];
            let v79 = parameters[36];
            let v81 = parameters[44];
            let v83 = parameters[37];
            let v85 = parameters[45];
            let v87 = parameters[38];
            let v89 = parameters[46];
            let v91 = parameters[39];
            let v93 = parameters[47];
            let v95 = parameters[40];
            let v97 = 4e0f64;
            let v98 = 5.522602e-23f64;
            let v100 = parameters[1];
            let v101 = parameters[0];
            let v102 = parameters[12];
            let v104 = parameters[13];
            let v106 = parameters[14];
            let v108 = parameters[15];
            let v110 = parameters[31];
            let v112 = parameters[32];
            let v116 = 1.179e0f64;
            let v117 = 9.025e-5f64;
            let v118 = 3.05e-7f64;
            let v123 = 1.045e0f64;
            let v124 = 4.5e-4f64;
            let v127 = 5.23e-1f64;
            let v128 = 1.4e-3f64;
            let v131 = 1.48e-6f64;
            let v136 = 9e4f64;
            let v138 = 1e-3f64;
            let v142 = 2.5e25f64;
            let v155 = 6e0f64;
            let v162 = 7.071067811865475e-1f64;
            let v166 = 1e-5f64;
            let v174 = 1.25e0f64;
            let v175 = 4.6051701859880916e2f64;
            let v179 = 1e-200f64;
            let v189 = parameters[16];
            let v191 = 3e0f64;
            let v192 = parameters[2];
            let v194 = 9e0f64;
            let v201 = parameters[33];
            let v206 = 1.2e1f64;
            let v226 = 2e1f64;
            let v245 = parameters[66];
            let v249 = parameters[49];
            let v250 = parameters[55];
            let v253 = 1e12f64;
            let v255 = parameters[56];
            let v257 = parameters[53];
            let v261 = parameters[60];
            let v265 = parameters[61];
            let v270 = parameters[52];
            let v276 = parameters[50];
            let v278 = parameters[51];
            let v280 = 1.3333333333333333e0f64;
            let v281 = 2.918995620956536e-49f64;
            let v285 = 1.05457168e-34f64;
            let v288 = 1.3333333333333333e0f64;
            let v289 = 2.918995620956536e-49f64;
            let v295 = parameters[59];
            let v297 = -4.95e-1f64;
            let v298 = parameters[58];
            let v301 = parameters[64];
            let v303 = -4.95e-1f64;
            let v304 = parameters[63];
            let v313 = parameters[57];
            let v315 = parameters[62];
            let v317 = 1e-1f64;
            let v318 = parameters[26];
            let v319 = node_potentials[4];
            let v320 = node_potentials[5];
            let v322 = parameters[27];
            let v325 = 1e-16f64;
            let v328 = parameters[28];
            let v341 = 1e-32f64;
            let v350 = parameters[25];
            let v354 = 1e-6f64;
            let v362 = 5e-7f64;
            let v369 = 1e-3f64;
            let v376 = 1e23f64;
            let v382 = 3.348580862e-29f64;
            let v389 = 7.5e-1f64;
            let v396 = 1.3333333333333333e0f64;
            let v423 = 7.324648775608221e-1f64;
            let v426 = 7.324648775608221e-1f64;
            let v435 = 1.666666666666667e-1f64;
            let v451 = 1e1f64;
            let v455 = 6.4e1f64;
            let v491 = 2.3025850929940458e2f64;
            let v494 = 1e100f64;
            let v535 = 7.324648775608221e-1f64;
            let v548 = -2.3025850929940458e2f64;
            let v551 = 1e-100f64;
            let v552 = -2.3025850929940458e2f64;
            let v554 = -2.3025850929940458e2f64;
            let v557 = -2.3025850929940458e2f64;
            let v570 = 2.5e-1f64;
            let v581 = 5e0f64;
            let v589 = 2.5e0f64;
            let v596 = 2.23606797749979e0f64;
            let v611 = 1e-40f64;
            let v634 = 1e-120f64;
            let v716 = 1e27f64;
            let v719 = parameters[18];
            let v837 = -2.3025850929940458e2f64;
            let v840 = -2.3025850929940458e2f64;
            let v842 = -2.3025850929940458e2f64;
            let v845 = -2.3025850929940458e2f64;
            let v875 = 2.5e0f64;
            let v882 = 2.23606797749979e0f64;
            let v1115 = -2.3025850929940458e2f64;
            let v1118 = -2.3025850929940458e2f64;
            let v1120 = -2.3025850929940458e2f64;
            let v1123 = -2.3025850929940458e2f64;
            let v1153 = 2.5e0f64;
            let v1160 = 2.23606797749979e0f64;
            let v1280 = parameters[21];
            let v1290 = node_potentials[6];
            let v1404 = -2.3025850929940458e2f64;
            let v1406 = -2.3025850929940458e2f64;
            let v1408 = -2.3025850929940458e2f64;
            let v1411 = -2.3025850929940458e2f64;
            let v1571 = -2.3025850929940458e2f64;
            let v1574 = -2.3025850929940458e2f64;
            let v1576 = -2.3025850929940458e2f64;
            let v1579 = -2.3025850929940458e2f64;
            let v1609 = 2.5e0f64;
            let v1616 = 2.23606797749979e0f64;
            let v1851 = -2.3025850929940458e2f64;
            let v1853 = -2.3025850929940458e2f64;
            let v1855 = -2.3025850929940458e2f64;
            let v1858 = -2.3025850929940458e2f64;
            let v1946 = 1.62e0f64;
            let v1950 = 3.7e-1f64;
            let v2016 = -1.666666666666667e-1f64;
            let v2021 = -1e0f64;
            let v2025 = 1e-2f64;
            let v2033 = 5e-3f64;
            let v2040 = 1e-1f64;
            let v2055 = 4e-2f64;
            let v2061 = parameters[41];
            let v2068 = -1e0f64;
            let v2071 = node_potentials[1];
            let v2195 = -2.3025850929940458e2f64;
            let v2197 = -2.3025850929940458e2f64;
            let v2199 = -2.3025850929940458e2f64;
            let v2202 = -2.3025850929940458e2f64;
            let v2265 = 5e-3f64;
            let v2272 = 1e-1f64;
            let v2296 = 5e-7f64;
            let v2303 = 1e-3f64;
            let v2309 = 1.0f64;
            let v2338 = -1.5e0f64;
            let v2353 = -2.3025850929940458e2f64;
            let v2356 = -2.3025850929940458e2f64;
            let v2358 = -2.3025850929940458e2f64;
            let v2361 = -2.3025850929940458e2f64;
            let v2390 = 5e-3f64;
            let v2397 = 1e-1f64;
            let v2421 = 5e-7f64;
            let v2428 = 1e-3f64;
            let v2434 = 1.0f64;
            let v2456 = -1.5e0f64;
            let v2466 = -2.3025850929940458e2f64;
            let v2468 = -2.3025850929940458e2f64;
            let v2470 = -2.3025850929940458e2f64;
            let v2473 = -2.3025850929940458e2f64;
            let v2522 = 5e-3f64;
            let v2529 = 1e-1f64;
            let v2550 = 5e-7f64;
            let v2557 = 1e-3f64;
            let v2563 = 0.0f64;
            let v2589 = -1.5e0f64;
            let v2604 = -2.3025850929940458e2f64;
            let v2607 = -2.3025850929940458e2f64;
            let v2609 = -2.3025850929940458e2f64;
            let v2612 = -2.3025850929940458e2f64;
            let v2640 = 5e-3f64;
            let v2647 = 1e-1f64;
            let v2668 = 5e-7f64;
            let v2675 = 1e-3f64;
            let v2681 = 0.0f64;
            let v2703 = -1.5e0f64;
            let v2713 = -2.3025850929940458e2f64;
            let v2715 = -2.3025850929940458e2f64;
            let v2717 = -2.3025850929940458e2f64;
            let v2720 = -2.3025850929940458e2f64;
            let v2748 = node_potentials[0];
            let v2749 = node_potentials[2];
            let v2758 = parameters[65];
            let v2760 = 3.2043836e-19f64;
            let v2763 = 3.2043836e-19f64;
            let v2770 = parameters[10];
            let v3 = if v1 != v2 { 1.0 } else { 0.0 };
            if v3 != 0.0 {
            } else {
            }
            let v10 = (v4 * (v5 / v6)) / v9;
            let v18 = ((v14 * v15).sqrt()) / v10;
            let v23 = ((v19 * v20).sqrt()) / v10;
            let v25 = if v24 > v0 { 1.0 } else { 0.0 };
            let v390: f64;
            if v25 != 0.0 {
                let v30 = (v26 * v24) * (v10.powf(v28));
                let v32 = if v31 < v0 { 1.0 } else { 0.0 };
                let v391: f64;
                if v32 != 0.0 {
                    let v34 = v33 * v30;
                    v391 = v34;
                } else {
                    v391 = v30;
                }
                v390 = v391;
            } else {
                v390 = v0;
            }
            let v35 = if v31 < v0 { 1.0 } else { 0.0 };
            let v1985: f64;
            if v35 != 0.0 {
                let v38 = v36 * v37;
                v1985 = v38;
            } else {
                let v40 = v39 * v37;
                v1985 = v40;
            }
            let v42 = v9 / v41;
            let v45 = if v43 > v44 { 1.0 } else { 0.0 };
            let v47: f64;
            if v45 != 0.0 {
                v47 = v43;
            } else {
                v47 = v46;
            }
            let v49 = if v47 < v48 { 1.0 } else { 0.0 };
            if v49 != 0.0 {
            } else {
            }
            let v51 = if v47 > v50 { 1.0 } else { 0.0 };
            if v51 != 0.0 {
            } else {
            }
            let v53 = v52 + v47;
            let v57 = (v54 + v55) - v52;
            let v58 = if v57 < v48 { 1.0 } else { 0.0 };
            if v58 != 0.0 {
            } else {
            }
            let v59 = if v57 > v50 { 1.0 } else { 0.0 };
            if v59 != 0.0 {
            } else {
            }
            let v60 = v57 + v52;
            let v61 = v60 * v60;
            let v63 = v60 / v53;
            let v64 = v53 / v60;
            let v67 = (v60 * v65) / v12;
            let v70 = (v68 * v67) * v67;
            let v72 = v71 / v67;
            let v76 = v73 + ((v60 - v53) * v74);
            let v80 = v79 * (v64.powf(v77));
            let v84 = v83 * (v64.powf(v81));
            let v88 = v87 * (v64.powf(v85));
            let v92 = v91 * (v64.powf(v89));
            let v96 = v95 * (v63.powf(v93));
            let v99 = v98 * v60;
            let v103 = if v100 < v102 { 1.0 } else { 0.0 };
            if v103 != 0.0 {
            } else {
            }
            let v105 = if v100 > v104 { 1.0 } else { 0.0 };
            if v105 != 0.0 {
            } else {
            }
            let v107 = if v101 < v106 { 1.0 } else { 0.0 };
            if v107 != 0.0 {
            } else {
            }
            let v109 = if v101 > v108 { 1.0 } else { 0.0 };
            if v109 != 0.0 {
            } else {
            }
            let v111 = v100 + v110;
            let v113 = v101 + v112;
            let v114 = if v111 <= v0 { 1.0 } else { 0.0 };
            if v114 != 0.0 {
            } else {
            }
            let v115 = if v113 <= v0 { 1.0 } else { 0.0 };
            if v115 != 0.0 {
            } else {
            }
            let v122 = v116 - (v60 * (v117 + (v60 * v118)));
            let v140 = (if ((((v123 + (v124 * v60)) * ((v127 + (v128 * v60)) - (v131 * v61))) * v61) / v136) >= v138 { ((((v123 + (v124 * v60)) * ((v127 + (v128 * v60)) - (v131 * v61))) * v61) / v136) } else { v138 }).sqrt();
            let v145 = v71 / ((v142 * v140) * (v140.sqrt()));
            let v146 = v11 * v67;
            let v150 = v122 + (v146 * ((v13 * v145).ln()));
            let v157 = v122 + (v155 * v67);
            let v158 = v72.sqrt();
            let v159 = v18 * v158;
            let v160 = v159 * v159;
            let v161 = v71 / v160;
            let v164 = v71 + (v159 * v162);
            let v165 = v71 / v164;
            let v167 = v166 * v164;
            let v168 = (v122 + (v146 * ((v15 * v145).ln()))) * v72;
            let v169 = v23 * v158;
            let v170 = v169 * v169;
            let v172 = v71 + (v169 * v162);
            let v173 = v166 * v172;
            let v176 = if v168 < v175 { 1.0 } else { 0.0 };
            let v733: f64;
            if v176 != 0.0 {
                let v178 = (-v168).exp();
                v733 = v178;
            } else {
                let v180 = v168 - v175;
                let v188 = v179 / (v71 + (v180 * (v71 + ((v39 * v180) * (v71 + (v180 * v36))))));
                v733 = v188;
            }
            let v237: f64;
            let v239: f64;
            let v241: f64;
            let v243: f64;
            let v247: f64;
            if v189 != 0.0 {
                let v198 = (v80 * v101) / ((v191 + ((v192 - v71) * v194)) * v100);
                let v200 = v84 / (v101 * v100);
                let v202 = v101 + v201;
                let v204 = v88 / (v11 * v202);
                let v208 = (v92 * v100) / (v206 * v202);
                let v209 = if v198 > v138 { 1.0 } else { 0.0 };
                let v212: f64;
                if v209 != 0.0 {
                    let v210 = if v198 < v2 { 1.0 } else { 0.0 };
                    let v211: f64;
                    if v210 != 0.0 {
                        v211 = v198;
                    } else {
                        v211 = v2;
                    }
                    v212 = v211;
                } else {
                    v212 = v138;
                }
                let v213 = if v200 > v138 { 1.0 } else { 0.0 };
                let v216: f64;
                if v213 != 0.0 {
                    let v214 = if v200 < v68 { 1.0 } else { 0.0 };
                    let v215: f64;
                    if v214 != 0.0 {
                        v215 = v200;
                    } else {
                        v215 = v68;
                    }
                    v216 = v215;
                } else {
                    v216 = v138;
                }
                let v217 = if v204 > v138 { 1.0 } else { 0.0 };
                let v220: f64;
                if v217 != 0.0 {
                    let v218 = if v204 < v2 { 1.0 } else { 0.0 };
                    let v219: f64;
                    if v218 != 0.0 {
                        v219 = v204;
                    } else {
                        v219 = v2;
                    }
                    v220 = v219;
                } else {
                    v220 = v138;
                }
                let v221 = if v208 > v138 { 1.0 } else { 0.0 };
                let v224: f64;
                if v221 != 0.0 {
                    let v222 = if v208 < v2 { 1.0 } else { 0.0 };
                    let v223: f64;
                    if v222 != 0.0 {
                        v223 = v208;
                    } else {
                        v223 = v2;
                    }
                    v224 = v223;
                } else {
                    v224 = v138;
                }
                let v225 = if v96 > v138 { 1.0 } else { 0.0 };
                let v229: f64;
                if v225 != 0.0 {
                    let v227 = if v96 < v226 { 1.0 } else { 0.0 };
                    let v228: f64;
                    if v227 != 0.0 {
                        v228 = v96;
                    } else {
                        v228 = v226;
                    }
                    v229 = v228;
                } else {
                    v229 = v138;
                }
                let v230 = v71 / v212;
                let v231 = v71 / v216;
                let v232 = v71 / v220;
                let v233 = v71 / v224;
                let v236 = ((v206 * v229) * v101) / v100;
                v237 = v230;
                v239 = v231;
                v241 = v232;
                v243 = v233;
                v247 = v236;
            } else {
                v237 = v0;
                v239 = v0;
                v241 = v0;
                v243 = v0;
                v247 = v0;
            }
            let v238 = v99 * v237;
            let v240 = v99 * v239;
            let v242 = v99 * v241;
            let v244 = v99 * v243;
            let v246 = if v245 == v0 { 1.0 } else { 0.0 };
            let v2767: f64;
            if v246 != 0.0 {
                v2767 = v0;
            } else {
                let v248 = v99 * v247;
                v2767 = v248;
            }
            let v2078: f64;
            let v2080: f64;
            let v2254: f64;
            let v2281: f64;
            let v2284: f64;
            let v2312: f64;
            let v2319: f64;
            let v2337: f64;
            let v2379: f64;
            let v2406: f64;
            let v2409: f64;
            let v2455: f64;
            let v2501: f64;
            let v2503: f64;
            let v2588: f64;
            let v2702: f64;
            if v249 != 0.0 {
                let v271 = v63.powf(v270);
                let v272 = (((v250 * v113) * v111) * v253) * v271;
                let v273 = ((((v11 * v255) * v257) * v113) * v253) * v271;
                let v274 = (((v261 * v113) * v111) * v253) * v271;
                let v275 = ((((v11 * v265) * v257) * v113) * v253) * v271;
                let v277 = v71 / v276;
                let v279 = v71 / v278;
                let v287 = ((v280 * ((v281 * v276).sqrt())) / v285) * v9;
                let v294 = ((v288 * ((v289 * v278).sqrt())) / v285) * v9;
                let v296 = if v295 < v0 { 1.0 } else { 0.0 };
                let v2410: f64;
                if v296 != 0.0 {
                    let v300 = (v297 * v298) / v295;
                    v2410 = v300;
                } else {
                    v2410 = v0;
                }
                let v302 = if v301 < v0 { 1.0 } else { 0.0 };
                let v2285: f64;
                if v302 != 0.0 {
                    let v306 = (v303 * v304) / v301;
                    v2285 = v306;
                } else {
                    v2285 = v0;
                }
                let v309 = v39 * ((v31 * v150) + v122);
                let v312 = v39 * ((v31 * v157) + v122);
                let v314 = v313 * v67;
                let v316 = v315 * v67;
                v2078 = v273;
                v2080 = v275;
                v2254 = v316;
                v2281 = v279;
                v2284 = v2285;
                v2312 = v312;
                v2319 = v309;
                v2337 = v294;
                v2379 = v314;
                v2406 = v277;
                v2409 = v2410;
                v2455 = v287;
                v2501 = v272;
                v2503 = v274;
                v2588 = v294;
                v2702 = v287;
            } else {
                v2078 = v0;
                v2080 = v0;
                v2254 = v0;
                v2281 = v317;
                v2284 = v0;
                v2312 = v0;
                v2319 = v0;
                v2337 = v0;
                v2379 = v0;
                v2406 = v317;
                v2409 = v0;
                v2455 = v0;
                v2501 = v0;
                v2503 = v0;
                v2588 = v0;
                v2702 = v0;
            }
            let v321 = v319 - v320;
            let v324 = v31 * (v321 - v322);
            let v326 = if v324 > v325 { 1.0 } else { 0.0 };
            let v347: f64;
            if v326 != 0.0 {
                let v332 = v39 * (v324 + (((v324 * v324) + v328).sqrt()));
                v347 = v332;
            } else {
                let v333 = v0 - v324;
                let v334 = if v333 > v325 { 1.0 } else { 0.0 };
                let v346: f64;
                if v334 != 0.0 {
                    let v340 = (v39 * v328) / (v333 + (((v333 * v333) + v328).sqrt()));
                    v346 = v340;
                } else {
                    let v345 = v39 * (v324 + ((v341 + v328).sqrt()));
                    v346 = v345;
                }
                v347 = v346;
            }
            let v349 = v71 + (v318 * v347);
            let v351 = v350 - v349;
            let v352 = if v351 > v325 { 1.0 } else { 0.0 };
            let v374: f64;
            if v352 != 0.0 {
                let v359 = v350 - (v39 * (v351 + (((v351 * v351) + v354).sqrt())));
                v374 = v359;
            } else {
                let v360 = v349 - v350;
                let v361 = if v360 > v325 { 1.0 } else { 0.0 };
                let v373: f64;
                if v361 != 0.0 {
                    let v368 = v350 - (v362 / (v360 + (((v360 * v360) + v354).sqrt())));
                    v373 = v368;
                } else {
                    let v372 = v350 - (v39 * (v351 + v369));
                    v373 = v372;
                }
                v374 = v373;
            }
            let v375 = v13 * v374;
            let v377 = v375 / v376;
            let v381 = v122 + (v146 * ((v375 * v145).ln()));
            let v385 = ((v382 * v375).sqrt()) / v10;
            let v401: f64;
            let v409: f64;
            if v25 != 0.0 {
                let v388 = ((v385 * v385) * v381).sqrt();
                let v394 = (v389 * v390) * (v388.powf(v28));
                let v395 = v381 + v394;
                let v400 = v385 * (v71 + ((v396 * v394) / v388));
                v401 = v400;
                v409 = v395;
            } else {
                v401 = v385;
                v409 = v381;
            }
            let v402 = v401 * v158;
            let v403 = v402 * v402;
            let v404 = v71 / v403;
            let v406 = v71 + (v402 * v162);
            let v407 = v71 / v406;
            let v408 = v166 * v406;
            let v410 = v409 * v72;
            let v411 = if v410 < v175 { 1.0 } else { 0.0 };
            let v439: f64;
            if v411 != 0.0 {
                let v413 = (-v410).exp();
                v439 = v413;
            } else {
                let v414 = v410 - v175;
                let v422 = v179 / (v71 + (v414 * (v71 + ((v39 * v414) * (v71 + (v414 * v36))))));
                v439 = v422;
            }
            let v425 = v174 + (v402 * v423);
            let v428 = v174 + (v169 * v426);
            let v430 = v31 * (v321 - v76);
            let v431 = v430 * v72;
            let v433 = if (v431.abs()) <= v408 { 1.0 } else { 0.0 };
            let v721: f64;
            if v433 != 0.0 {
                let v445 = (v431 * v407) * (v71 + (((v431 * (v71 - v439)) * v402) * (((v407 * v407) * v435) * v162)));
                v721 = v445;
            } else {
                let v447 = if v431 < (-v408) { 1.0 } else { 0.0 };
                let v722: f64;
                if v447 != 0.0 {
                    let v448 = -v431;
                    let v450 = (v174 * v448) * v407;
                    let v453 = v450 - v155;
                    let v459 = v39 * ((v450 + v451) - (((v453 * v453) + v455).sqrt()));
                    let v460 = v448 - v459;
                    let v464 = (v460 * v460) + (v403 * (v459 + v71));
                    let v466 = (v11 * v460) - v403;
                    let v470 = (-v459) + ((v464 * v404).ln());
                    let v471 = v464 + v466;
                    let v477 = (v471 * v471) + ((((v39 * v466) * v466) - v464) * v470);
                    let v490 = v459 + (((v464 * v471) * v470) / (v477 + (((((v471 * v470) * v470) / v477) * v466) * (((v466 * v466) * v36) - v464))));
                    let v492 = if v490 < v491 { 1.0 } else { 0.0 };
                    let v504: f64;
                    if v492 != 0.0 {
                        let v493 = v490.exp();
                        v504 = v493;
                    } else {
                        let v495 = v490 - v491;
                        let v503 = v494 * (v71 + (v495 * (v71 + ((v39 * v495) * (v71 + (v495 * v36))))));
                        v504 = v503;
                    }
                    let v506 = v448 - v490;
                    let v507 = v439 * (v71 / v504);
                    let v513 = (v11 * v506) + (v403 * (((v504 - v71) - v507) + v439));
                    let v527 = v11 * ((v506 * v506) - (v403 * ((((v504 - v490) - v71) + v507) + (v439 * (v490 - v71)))));
                    let v534 = (-v490) - (v527 / (v513 + (((v513 * v513) - (v527 * (v11 - (v403 * (v504 + v507))))).sqrt())));
                    v722 = v534;
                } else {
                    let v538 = v71 / (v174 + (v402 * v535));
                    let v547 = -((v431 * v407) * (v71 + (((((v406 * v174) * v538) - v71) * v538) * v431)));
                    let v549 = if v547 > v548 { 1.0 } else { 0.0 };
                    let v566: f64;
                    if v549 != 0.0 {
                        let v550 = v547.exp();
                        v566 = v550;
                    } else {
                        let v565 = v551 / (v71 + ((v552 - v547) * (v71 + ((v39 * (v554 - v547)) * (v71 + ((v557 - v547) * v36))))));
                        v566 = v565;
                    }
                    let v568 = v403 * v39;
                    let v576 = (v431 + v568) - (v402 * (((v431 + (v403 * v570)) - (v71 - v566)).sqrt()));
                    let v577 = v410 + v191;
                    let v578 = v577 - v576;
                    let v579 = if v578 > v325 { 1.0 } else { 0.0 };
                    let v601: f64;
                    if v579 != 0.0 {
                        let v586 = v577 - (v39 * (v578 + (((v578 * v578) + v581).sqrt())));
                        v601 = v586;
                    } else {
                        let v587 = v576 - v577;
                        let v588 = if v587 > v325 { 1.0 } else { 0.0 };
                        let v600: f64;
                        if v588 != 0.0 {
                            let v595 = v577 - (v589 / (v587 + (((v587 * v587) + v581).sqrt())));
                            v600 = v595;
                        } else {
                            let v599 = v577 - (v39 * (v578 + v596));
                            v600 = v599;
                        }
                        v601 = v600;
                    }
                    let v607 = v601 - (v39 * (v577 - (((v577 * v577) + v581).sqrt())));
                    let v608 = v431 - v607;
                    let v610 = (-v607).exp();
                    let v620 = if v611 >= ((v608 * v608) - (v403 * (((v610 + v607) - v71) - (v439 * (v607 + v71))))) { v611 } else { ((v608 * v608) - (v403 * (((v610 + v607) - v71) - (v439 * (v607 + v71))))) };
                    let v622 = v71 - (v568 * v610);
                    let v627 = (v11 * v608) + (v403 * ((v71 - v610) - v439));
                    let v631 = (v410 - v607) + ((v620 / v403).ln());
                    let v632 = v620 + v627;
                    let v635 = if (v631.abs()) < v634 { 1.0 } else { 0.0 };
                    let v656: f64;
                    if v635 != 0.0 {
                        v656 = v607;
                    } else {
                        let v639 = v620 * v622;
                        let v642 = (v632 * v632) + ((((v39 * v627) * v627) - v639) * v631);
                        let v655 = v607 + (((v620 * v632) * v631) / (v642 + (((((v632 * v631) * v631) / v642) * v627) * (((v627 * v627) * v36) - v639))));
                        v656 = v655;
                    }
                    let v657 = if v656 < v491 { 1.0 } else { 0.0 };
                    let v687: f64;
                    let v690: f64;
                    if v657 != 0.0 {
                        let v658 = v656.exp();
                        let v659 = v71 / v658;
                        let v660 = v439 * v658;
                        v687 = v659;
                        v690 = v660;
                    } else {
                        let v662 = if v656 > (v410 - v491) { 1.0 } else { 0.0 };
                        let v688: f64;
                        let v691: f64;
                        if v662 != 0.0 {
                            let v664 = (v656 - v410).exp();
                            let v665 = v439 / v664;
                            v688 = v665;
                            v691 = v664;
                        } else {
                            let v667 = (v410 - v656) - v491;
                            let v675 = v551 / (v71 + (v667 * (v71 + ((v39 * v667) * (v71 + (v667 * v36))))));
                            let v676 = v656 - v491;
                            let v684 = v551 / (v71 + (v676 * (v71 + ((v39 * v676) * (v71 + (v676 * v36))))));
                            v688 = v684;
                            v691 = v675;
                        }
                        v687 = v688;
                        v690 = v691;
                    }
                    let v685 = v431 - v656;
                    let v695 = (v11 * v685) + (v403 * (((v71 - v687) + v690) - v439));
                    let v709 = v11 * ((v685 * v685) - (v403 * ((((v687 + v656) - v71) + v690) - (v439 * (v656 + v71)))));
                    let v715 = v656 + (v709 / (v695 + (((v695 * v695) - (v709 * (v11 - (v403 * (v687 + v690))))).sqrt())));
                    v722 = v715;
                }
                v721 = v722;
            }
            let v717 = if v15 < v716 { 1.0 } else { 0.0 };
            let v1278: f64;
            let v1283: f64;
            if v717 != 0.0 {
                let v720 = (-v31) * v719;
                let v726 = (v720 * (v430 - (v721 * v67))) * v72;
                let v728 = if (v726.abs()) <= v167 { 1.0 } else { 0.0 };
                let v1000: f64;
                if v728 != 0.0 {
                    let v739 = (v726 * v165) * (v71 + (((v726 * (v71 - v733)) * v159) * (((v165 * v165) * v435) * v162)));
                    v1000 = v739;
                } else {
                    let v741 = if v726 < (-v167) { 1.0 } else { 0.0 };
                    let v1001: f64;
                    if v741 != 0.0 {
                        let v742 = -v726;
                        let v744 = (v174 * v742) * v165;
                        let v746 = v744 - v155;
                        let v751 = v39 * ((v744 + v451) - (((v746 * v746) + v455).sqrt()));
                        let v752 = v742 - v751;
                        let v756 = (v752 * v752) + (v160 * (v751 + v71));
                        let v758 = (v11 * v752) - v160;
                        let v762 = (-v751) + ((v756 * v161).ln());
                        let v763 = v756 + v758;
                        let v769 = (v763 * v763) + ((((v39 * v758) * v758) - v756) * v762);
                        let v782 = v751 + (((v756 * v763) * v762) / (v769 + (((((v763 * v762) * v762) / v769) * v758) * (((v758 * v758) * v36) - v756))));
                        let v783 = if v782 < v491 { 1.0 } else { 0.0 };
                        let v794: f64;
                        if v783 != 0.0 {
                            let v784 = v782.exp();
                            v794 = v784;
                        } else {
                            let v785 = v782 - v491;
                            let v793 = v494 * (v71 + (v785 * (v71 + ((v39 * v785) * (v71 + (v785 * v36))))));
                            v794 = v793;
                        }
                        let v796 = v742 - v782;
                        let v797 = v733 * (v71 / v794);
                        let v803 = (v11 * v796) + (v160 * (((v794 - v71) - v797) + v733));
                        let v817 = v11 * ((v796 * v796) - (v160 * ((((v794 - v782) - v71) + v797) + (v733 * (v782 - v71)))));
                        let v824 = (-v782) - (v817 / (v803 + (((v803 * v803) - (v817 * (v11 - (v160 * (v794 + v797))))).sqrt())));
                        v1001 = v824;
                    } else {
                        let v827 = v71 / (v174 + (v159 * v535));
                        let v836 = -((v726 * v165) * (v71 + (((((v164 * v174) * v827) - v71) * v827) * v726)));
                        let v838 = if v836 > v837 { 1.0 } else { 0.0 };
                        let v854: f64;
                        if v838 != 0.0 {
                            let v839 = v836.exp();
                            v854 = v839;
                        } else {
                            let v853 = v551 / (v71 + ((v840 - v836) * (v71 + ((v39 * (v842 - v836)) * (v71 + ((v845 - v836) * v36))))));
                            v854 = v853;
                        }
                        let v856 = v160 * v39;
                        let v863 = (v726 + v856) - (v159 * (((v726 + (v160 * v570)) - (v71 - v854)).sqrt()));
                        let v864 = v168 + v191;
                        let v865 = v864 - v863;
                        let v866 = if v865 > v325 { 1.0 } else { 0.0 };
                        let v887: f64;
                        if v866 != 0.0 {
                            let v872 = v864 - (v39 * (v865 + (((v865 * v865) + v581).sqrt())));
                            v887 = v872;
                        } else {
                            let v873 = v863 - v864;
                            let v874 = if v873 > v325 { 1.0 } else { 0.0 };
                            let v886: f64;
                            if v874 != 0.0 {
                                let v881 = v864 - (v875 / (v873 + (((v873 * v873) + v581).sqrt())));
                                v886 = v881;
                            } else {
                                let v885 = v864 - (v39 * (v865 + v882));
                                v886 = v885;
                            }
                            v887 = v886;
                        }
                        let v893 = v887 - (v39 * (v864 - (((v864 * v864) + v581).sqrt())));
                        let v894 = v726 - v893;
                        let v896 = (-v893).exp();
                        let v905 = if v611 >= ((v894 * v894) - (v160 * (((v896 + v893) - v71) - (v733 * (v893 + v71))))) { v611 } else { ((v894 * v894) - (v160 * (((v896 + v893) - v71) - (v733 * (v893 + v71))))) };
                        let v907 = v71 - (v856 * v896);
                        let v912 = (v11 * v894) + (v160 * ((v71 - v896) - v733));
                        let v916 = (v168 - v893) + ((v905 / v160).ln());
                        let v917 = v905 + v912;
                        let v919 = if (v916.abs()) < v634 { 1.0 } else { 0.0 };
                        let v940: f64;
                        if v919 != 0.0 {
                            v940 = v893;
                        } else {
                            let v923 = v905 * v907;
                            let v926 = (v917 * v917) + ((((v39 * v912) * v912) - v923) * v916);
                            let v939 = v893 + (((v905 * v917) * v916) / (v926 + (((((v917 * v916) * v916) / v926) * v912) * (((v912 * v912) * v36) - v923))));
                            v940 = v939;
                        }
                        let v941 = if v940 < v491 { 1.0 } else { 0.0 };
                        let v971: f64;
                        let v974: f64;
                        if v941 != 0.0 {
                            let v942 = v940.exp();
                            let v943 = v71 / v942;
                            let v944 = v733 * v942;
                            v971 = v943;
                            v974 = v944;
                        } else {
                            let v946 = if v940 > (v168 - v491) { 1.0 } else { 0.0 };
                            let v972: f64;
                            let v975: f64;
                            if v946 != 0.0 {
                                let v948 = (v940 - v168).exp();
                                let v949 = v733 / v948;
                                v972 = v949;
                                v975 = v948;
                            } else {
                                let v951 = (v168 - v940) - v491;
                                let v959 = v551 / (v71 + (v951 * (v71 + ((v39 * v951) * (v71 + (v951 * v36))))));
                                let v960 = v940 - v491;
                                let v968 = v551 / (v71 + (v960 * (v71 + ((v39 * v960) * (v71 + (v960 * v36))))));
                                v972 = v968;
                                v975 = v959;
                            }
                            v971 = v972;
                            v974 = v975;
                        }
                        let v969 = v726 - v940;
                        let v979 = (v11 * v969) + (v160 * (((v71 - v971) + v974) - v733));
                        let v993 = v11 * ((v969 * v969) - (v160 * ((((v971 + v940) - v71) + v974) - (v733 * (v940 + v71)))));
                        let v999 = v940 + (v993 / (v979 + (((v979 * v979) - (v993 * (v11 - (v160 * (v971 + v974))))).sqrt())));
                        v1001 = v999;
                    }
                    v1000 = v1001;
                }
                let v1005 = (v430 - ((v720 * v1000) * v67)) / v67;
                let v1007 = if (v1005.abs()) <= v408 { 1.0 } else { 0.0 };
                let v1284: f64;
                if v1007 != 0.0 {
                    let v1017 = (v1005 * v407) * (v71 + (((v1005 * (v71 - v439)) * v402) * (((v407 * v407) * v435) * v162)));
                    v1284 = v1017;
                } else {
                    let v1019 = if v1005 < (-v408) { 1.0 } else { 0.0 };
                    let v1285: f64;
                    if v1019 != 0.0 {
                        let v1020 = -v1005;
                        let v1022 = (v174 * v1020) * v407;
                        let v1024 = v1022 - v155;
                        let v1029 = v39 * ((v1022 + v451) - (((v1024 * v1024) + v455).sqrt()));
                        let v1030 = v1020 - v1029;
                        let v1034 = (v1030 * v1030) + (v403 * (v1029 + v71));
                        let v1036 = (v11 * v1030) - v403;
                        let v1040 = (-v1029) + ((v1034 * v404).ln());
                        let v1041 = v1034 + v1036;
                        let v1047 = (v1041 * v1041) + ((((v39 * v1036) * v1036) - v1034) * v1040);
                        let v1060 = v1029 + (((v1034 * v1041) * v1040) / (v1047 + (((((v1041 * v1040) * v1040) / v1047) * v1036) * (((v1036 * v1036) * v36) - v1034))));
                        let v1061 = if v1060 < v491 { 1.0 } else { 0.0 };
                        let v1072: f64;
                        if v1061 != 0.0 {
                            let v1062 = v1060.exp();
                            v1072 = v1062;
                        } else {
                            let v1063 = v1060 - v491;
                            let v1071 = v494 * (v71 + (v1063 * (v71 + ((v39 * v1063) * (v71 + (v1063 * v36))))));
                            v1072 = v1071;
                        }
                        let v1074 = v1020 - v1060;
                        let v1075 = v439 * (v71 / v1072);
                        let v1081 = (v11 * v1074) + (v403 * (((v1072 - v71) - v1075) + v439));
                        let v1095 = v11 * ((v1074 * v1074) - (v403 * ((((v1072 - v1060) - v71) + v1075) + (v439 * (v1060 - v71)))));
                        let v1102 = (-v1060) - (v1095 / (v1081 + (((v1081 * v1081) - (v1095 * (v11 - (v403 * (v1072 + v1075))))).sqrt())));
                        v1285 = v1102;
                    } else {
                        let v1105 = v71 / (v174 + (v402 * v535));
                        let v1114 = -((v1005 * v407) * (v71 + (((((v406 * v174) * v1105) - v71) * v1105) * v1005)));
                        let v1116 = if v1114 > v1115 { 1.0 } else { 0.0 };
                        let v1132: f64;
                        if v1116 != 0.0 {
                            let v1117 = v1114.exp();
                            v1132 = v1117;
                        } else {
                            let v1131 = v551 / (v71 + ((v1118 - v1114) * (v71 + ((v39 * (v1120 - v1114)) * (v71 + ((v1123 - v1114) * v36))))));
                            v1132 = v1131;
                        }
                        let v1134 = v403 * v39;
                        let v1141 = (v1005 + v1134) - (v402 * (((v1005 + (v403 * v570)) - (v71 - v1132)).sqrt()));
                        let v1142 = v410 + v191;
                        let v1143 = v1142 - v1141;
                        let v1144 = if v1143 > v325 { 1.0 } else { 0.0 };
                        let v1165: f64;
                        if v1144 != 0.0 {
                            let v1150 = v1142 - (v39 * (v1143 + (((v1143 * v1143) + v581).sqrt())));
                            v1165 = v1150;
                        } else {
                            let v1151 = v1141 - v1142;
                            let v1152 = if v1151 > v325 { 1.0 } else { 0.0 };
                            let v1164: f64;
                            if v1152 != 0.0 {
                                let v1159 = v1142 - (v1153 / (v1151 + (((v1151 * v1151) + v581).sqrt())));
                                v1164 = v1159;
                            } else {
                                let v1163 = v1142 - (v39 * (v1143 + v1160));
                                v1164 = v1163;
                            }
                            v1165 = v1164;
                        }
                        let v1171 = v1165 - (v39 * (v1142 - (((v1142 * v1142) + v581).sqrt())));
                        let v1172 = v1005 - v1171;
                        let v1174 = (-v1171).exp();
                        let v1183 = if v611 >= ((v1172 * v1172) - (v403 * (((v1174 + v1171) - v71) - (v439 * (v1171 + v71))))) { v611 } else { ((v1172 * v1172) - (v403 * (((v1174 + v1171) - v71) - (v439 * (v1171 + v71))))) };
                        let v1185 = v71 - (v1134 * v1174);
                        let v1190 = (v11 * v1172) + (v403 * ((v71 - v1174) - v439));
                        let v1194 = (v410 - v1171) + ((v1183 / v403).ln());
                        let v1195 = v1183 + v1190;
                        let v1197 = if (v1194.abs()) < v634 { 1.0 } else { 0.0 };
                        let v1218: f64;
                        if v1197 != 0.0 {
                            v1218 = v1171;
                        } else {
                            let v1201 = v1183 * v1185;
                            let v1204 = (v1195 * v1195) + ((((v39 * v1190) * v1190) - v1201) * v1194);
                            let v1217 = v1171 + (((v1183 * v1195) * v1194) / (v1204 + (((((v1195 * v1194) * v1194) / v1204) * v1190) * (((v1190 * v1190) * v36) - v1201))));
                            v1218 = v1217;
                        }
                        let v1219 = if v1218 < v491 { 1.0 } else { 0.0 };
                        let v1249: f64;
                        let v1252: f64;
                        if v1219 != 0.0 {
                            let v1220 = v1218.exp();
                            let v1221 = v71 / v1220;
                            let v1222 = v439 * v1220;
                            v1249 = v1221;
                            v1252 = v1222;
                        } else {
                            let v1224 = if v1218 > (v410 - v491) { 1.0 } else { 0.0 };
                            let v1250: f64;
                            let v1253: f64;
                            if v1224 != 0.0 {
                                let v1226 = (v1218 - v410).exp();
                                let v1227 = v439 / v1226;
                                v1250 = v1227;
                                v1253 = v1226;
                            } else {
                                let v1229 = (v410 - v1218) - v491;
                                let v1237 = v551 / (v71 + (v1229 * (v71 + ((v39 * v1229) * (v71 + (v1229 * v36))))));
                                let v1238 = v1218 - v491;
                                let v1246 = v551 / (v71 + (v1238 * (v71 + ((v39 * v1238) * (v71 + (v1238 * v36))))));
                                v1250 = v1246;
                                v1253 = v1237;
                            }
                            v1249 = v1250;
                            v1252 = v1253;
                        }
                        let v1247 = v1005 - v1218;
                        let v1257 = (v11 * v1247) + (v403 * (((v71 - v1249) + v1252) - v439));
                        let v1271 = v11 * ((v1247 * v1247) - (v403 * ((((v1249 + v1218) - v71) + v1252) - (v439 * (v1218 + v71)))));
                        let v1277 = v1218 + (v1271 / (v1257 + (((v1257 * v1257) - (v1271 * (v11 - (v403 * (v1249 + v1252))))).sqrt())));
                        v1285 = v1277;
                    }
                    v1284 = v1285;
                }
                v1278 = v1005;
                v1283 = v1284;
            } else {
                v1278 = v431;
                v1283 = v721;
            }
            let v1282 = if (if v1278 <= v0 { 1.0 } else { 0.0 }) != 0.0 || (if v1280 < v71 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            if v1282 != 0.0 {
            } else {
                let v1286 = if v1283 < v491 { 1.0 } else { 0.0 };
                if v1286 != 0.0 {
                } else {
                    let v1288 = if v1283 > (v410 - v491) { 1.0 } else { 0.0 };
                    if v1288 != 0.0 {
                    } else {
                    }
                }
                let v1289 = if v1283 < v166 { 1.0 } else { 0.0 };
                if v1289 != 0.0 {
                } else {
                }
            }
            let v1291 = v430 + v1290;
            let v1292 = v1291 * v72;
            let v1294 = if (v1292.abs()) <= v408 { 1.0 } else { 0.0 };
            let v1454: f64;
            if v1294 != 0.0 {
                let v1295 = v1292 / v406;
                v1454 = v1295;
            } else {
                let v1296 = if v1292 > v408 { 1.0 } else { 0.0 };
                let v1455: f64;
                if v1296 != 0.0 {
                    let v1304 = (v1292 / v406) * (v71 + (((((v406 * v174) / v425) - v71) / v425) * v1292));
                    let v1305 = if v1304 < v175 { 1.0 } else { 0.0 };
                    let v1317: f64;
                    if v1305 != 0.0 {
                        let v1307 = (-v1304).exp();
                        v1317 = v1307;
                    } else {
                        let v1308 = v1304 - v175;
                        let v1316 = v179 / (v71 + (v1308 * (v71 + ((v39 * v1308) * (v71 + (v1308 * v36))))));
                        v1317 = v1316;
                    }
                    let v1319 = v39 * v403;
                    let v1326 = (v1292 + v1319) - (v402 * (((v1292 + (v570 * v403)) - (v71 - v1317)).sqrt()));
                    let v1327 = if v1326 < v175 { 1.0 } else { 0.0 };
                    let v1339: f64;
                    if v1327 != 0.0 {
                        let v1329 = (-v1326).exp();
                        v1339 = v1329;
                    } else {
                        let v1330 = v1326 - v175;
                        let v1338 = v179 / (v71 + (v1330 * (v71 + ((v39 * v1330) * (v71 + (v1330 * v36))))));
                        v1339 = v1338;
                    }
                    let v1342 = v1292 - v1326;
                    let v1346 = (v11 * v1342) + (v403 * (v71 - v1339));
                    let v1351 = (v1342 * v1342) - (v403 * ((v1326 - v71) + v1339));
                    let v1360 = v1326 + ((v11 * v1351) / (v1346 + (((v1346 * v1346) - ((v97 * (v71 - (v1319 * v1339))) * v1351)).sqrt())));
                    v1455 = v1360;
                } else {
                    let v1361 = -v1292;
                    let v1363 = (v174 * v1361) / v406;
                    let v1365 = v1363 - v155;
                    let v1370 = v39 * ((v1363 + v451) - (((v1365 * v1365) + v455).sqrt()));
                    let v1371 = v1361 - v1370;
                    let v1375 = (v1371 * v1371) + (v403 * (v1370 + v71));
                    let v1377 = (v11 * v1371) - v403;
                    let v1380 = ((v1375 / v403).ln()) - v1370;
                    let v1381 = v1375 + v1377;
                    let v1387 = (v1381 * v1381) + ((((v39 * v1377) * v1377) - v1375) * v1380);
                    let v1400 = v1370 + (((v1375 * v1381) * v1380) / (v1387 + (((((v1381 * v1380) * v1380) / v1387) * v1377) * (((v1377 * v1377) * v36) - v1375))));
                    let v1402 = if (v1400.abs()) < v491 { 1.0 } else { 0.0 };
                    let v1430: f64;
                    if v1402 != 0.0 {
                        let v1403 = v1400.exp();
                        v1430 = v1403;
                    } else {
                        let v1405 = if v1400 < v1404 { 1.0 } else { 0.0 };
                        let v1431: f64;
                        if v1405 != 0.0 {
                            let v1419 = v551 / (v71 + ((v1406 - v1400) * (v71 + ((v39 * (v1408 - v1400)) * (v71 + ((v1411 - v1400) * v36))))));
                            v1431 = v1419;
                        } else {
                            let v1420 = v1400 - v491;
                            let v1428 = v494 * (v71 + (v1420 * (v71 + ((v39 * v1420) * (v71 + (v1420 * v36))))));
                            v1431 = v1428;
                        }
                        v1430 = v1431;
                    }
                    let v1434 = v1361 - v1400;
                    let v1438 = (v11 * v1434) + (v403 * (v1430 - v71));
                    let v1443 = (v1434 * v1434) + (v403 * ((v1400 + v71) - v1430));
                    let v1453 = -(v1400 + ((v11 * v1443) / (v1438 + (((v1438 * v1438) - ((v97 * (v71 - ((v39 * v403) * v1430))) * v1443)).sqrt()))));
                    v1455 = v1453;
                }
                v1454 = v1455;
            }
            let v1456 = v1454 * v67;
            let v1903: f64;
            if v717 != 0.0 {
                let v1458 = (-v31) * v719;
                let v1461 = (v1458 * (v430 - v1456)) * v72;
                let v1463 = if (v1461.abs()) <= v167 { 1.0 } else { 0.0 };
                let v1734: f64;
                if v1463 != 0.0 {
                    let v1473 = (v1461 * v165) * (v71 + (((v1461 * (v71 - v733)) * v159) * (((v165 * v165) * v435) * v162)));
                    v1734 = v1473;
                } else {
                    let v1475 = if v1461 < (-v167) { 1.0 } else { 0.0 };
                    let v1735: f64;
                    if v1475 != 0.0 {
                        let v1476 = -v1461;
                        let v1478 = (v174 * v1476) * v165;
                        let v1480 = v1478 - v155;
                        let v1485 = v39 * ((v1478 + v451) - (((v1480 * v1480) + v455).sqrt()));
                        let v1486 = v1476 - v1485;
                        let v1490 = (v1486 * v1486) + (v160 * (v1485 + v71));
                        let v1492 = (v11 * v1486) - v160;
                        let v1496 = (-v1485) + ((v1490 * v161).ln());
                        let v1497 = v1490 + v1492;
                        let v1503 = (v1497 * v1497) + ((((v39 * v1492) * v1492) - v1490) * v1496);
                        let v1516 = v1485 + (((v1490 * v1497) * v1496) / (v1503 + (((((v1497 * v1496) * v1496) / v1503) * v1492) * (((v1492 * v1492) * v36) - v1490))));
                        let v1517 = if v1516 < v491 { 1.0 } else { 0.0 };
                        let v1528: f64;
                        if v1517 != 0.0 {
                            let v1518 = v1516.exp();
                            v1528 = v1518;
                        } else {
                            let v1519 = v1516 - v491;
                            let v1527 = v494 * (v71 + (v1519 * (v71 + ((v39 * v1519) * (v71 + (v1519 * v36))))));
                            v1528 = v1527;
                        }
                        let v1530 = v1476 - v1516;
                        let v1531 = v733 * (v71 / v1528);
                        let v1537 = (v11 * v1530) + (v160 * (((v1528 - v71) - v1531) + v733));
                        let v1551 = v11 * ((v1530 * v1530) - (v160 * ((((v1528 - v1516) - v71) + v1531) + (v733 * (v1516 - v71)))));
                        let v1558 = (-v1516) - (v1551 / (v1537 + (((v1537 * v1537) - (v1551 * (v11 - (v160 * (v1528 + v1531))))).sqrt())));
                        v1735 = v1558;
                    } else {
                        let v1561 = v71 / (v174 + (v159 * v535));
                        let v1570 = -((v1461 * v165) * (v71 + (((((v164 * v174) * v1561) - v71) * v1561) * v1461)));
                        let v1572 = if v1570 > v1571 { 1.0 } else { 0.0 };
                        let v1588: f64;
                        if v1572 != 0.0 {
                            let v1573 = v1570.exp();
                            v1588 = v1573;
                        } else {
                            let v1587 = v551 / (v71 + ((v1574 - v1570) * (v71 + ((v39 * (v1576 - v1570)) * (v71 + ((v1579 - v1570) * v36))))));
                            v1588 = v1587;
                        }
                        let v1590 = v160 * v39;
                        let v1597 = (v1461 + v1590) - (v159 * (((v1461 + (v160 * v570)) - (v71 - v1588)).sqrt()));
                        let v1598 = v168 + v191;
                        let v1599 = v1598 - v1597;
                        let v1600 = if v1599 > v325 { 1.0 } else { 0.0 };
                        let v1621: f64;
                        if v1600 != 0.0 {
                            let v1606 = v1598 - (v39 * (v1599 + (((v1599 * v1599) + v581).sqrt())));
                            v1621 = v1606;
                        } else {
                            let v1607 = v1597 - v1598;
                            let v1608 = if v1607 > v325 { 1.0 } else { 0.0 };
                            let v1620: f64;
                            if v1608 != 0.0 {
                                let v1615 = v1598 - (v1609 / (v1607 + (((v1607 * v1607) + v581).sqrt())));
                                v1620 = v1615;
                            } else {
                                let v1619 = v1598 - (v39 * (v1599 + v1616));
                                v1620 = v1619;
                            }
                            v1621 = v1620;
                        }
                        let v1627 = v1621 - (v39 * (v1598 - (((v1598 * v1598) + v581).sqrt())));
                        let v1628 = v1461 - v1627;
                        let v1630 = (-v1627).exp();
                        let v1639 = if v611 >= ((v1628 * v1628) - (v160 * (((v1630 + v1627) - v71) - (v733 * (v1627 + v71))))) { v611 } else { ((v1628 * v1628) - (v160 * (((v1630 + v1627) - v71) - (v733 * (v1627 + v71))))) };
                        let v1641 = v71 - (v1590 * v1630);
                        let v1646 = (v11 * v1628) + (v160 * ((v71 - v1630) - v733));
                        let v1650 = (v168 - v1627) + ((v1639 / v160).ln());
                        let v1651 = v1639 + v1646;
                        let v1653 = if (v1650.abs()) < v634 { 1.0 } else { 0.0 };
                        let v1674: f64;
                        if v1653 != 0.0 {
                            v1674 = v1627;
                        } else {
                            let v1657 = v1639 * v1641;
                            let v1660 = (v1651 * v1651) + ((((v39 * v1646) * v1646) - v1657) * v1650);
                            let v1673 = v1627 + (((v1639 * v1651) * v1650) / (v1660 + (((((v1651 * v1650) * v1650) / v1660) * v1646) * (((v1646 * v1646) * v36) - v1657))));
                            v1674 = v1673;
                        }
                        let v1675 = if v1674 < v491 { 1.0 } else { 0.0 };
                        let v1705: f64;
                        let v1708: f64;
                        if v1675 != 0.0 {
                            let v1676 = v1674.exp();
                            let v1677 = v71 / v1676;
                            let v1678 = v733 * v1676;
                            v1705 = v1677;
                            v1708 = v1678;
                        } else {
                            let v1680 = if v1674 > (v168 - v491) { 1.0 } else { 0.0 };
                            let v1706: f64;
                            let v1709: f64;
                            if v1680 != 0.0 {
                                let v1682 = (v1674 - v168).exp();
                                let v1683 = v733 / v1682;
                                v1706 = v1683;
                                v1709 = v1682;
                            } else {
                                let v1685 = (v168 - v1674) - v491;
                                let v1693 = v551 / (v71 + (v1685 * (v71 + ((v39 * v1685) * (v71 + (v1685 * v36))))));
                                let v1694 = v1674 - v491;
                                let v1702 = v551 / (v71 + (v1694 * (v71 + ((v39 * v1694) * (v71 + (v1694 * v36))))));
                                v1706 = v1702;
                                v1709 = v1693;
                            }
                            v1705 = v1706;
                            v1708 = v1709;
                        }
                        let v1703 = v1461 - v1674;
                        let v1713 = (v11 * v1703) + (v160 * (((v71 - v1705) + v1708) - v733));
                        let v1727 = v11 * ((v1703 * v1703) - (v160 * ((((v1705 + v1674) - v71) + v1708) - (v733 * (v1674 + v71)))));
                        let v1733 = v1674 + (v1727 / (v1713 + (((v1713 * v1713) - (v1727 * (v11 - (v160 * (v1705 + v1708))))).sqrt())));
                        v1735 = v1733;
                    }
                    v1734 = v1735;
                }
                let v1739 = (v1291 - ((v1458 * v1734) * v67)) / v67;
                let v1741 = if (v1739.abs()) <= v408 { 1.0 } else { 0.0 };
                let v1901: f64;
                if v1741 != 0.0 {
                    let v1742 = v1739 / v406;
                    v1901 = v1742;
                } else {
                    let v1743 = if v1739 > v408 { 1.0 } else { 0.0 };
                    let v1902: f64;
                    if v1743 != 0.0 {
                        let v1751 = (v1739 / v406) * (v71 + (((((v406 * v174) / v425) - v71) / v425) * v1739));
                        let v1752 = if v1751 < v175 { 1.0 } else { 0.0 };
                        let v1764: f64;
                        if v1752 != 0.0 {
                            let v1754 = (-v1751).exp();
                            v1764 = v1754;
                        } else {
                            let v1755 = v1751 - v175;
                            let v1763 = v179 / (v71 + (v1755 * (v71 + ((v39 * v1755) * (v71 + (v1755 * v36))))));
                            v1764 = v1763;
                        }
                        let v1766 = v39 * v403;
                        let v1773 = (v1739 + v1766) - (v402 * (((v1739 + (v570 * v403)) - (v71 - v1764)).sqrt()));
                        let v1774 = if v1773 < v175 { 1.0 } else { 0.0 };
                        let v1786: f64;
                        if v1774 != 0.0 {
                            let v1776 = (-v1773).exp();
                            v1786 = v1776;
                        } else {
                            let v1777 = v1773 - v175;
                            let v1785 = v179 / (v71 + (v1777 * (v71 + ((v39 * v1777) * (v71 + (v1777 * v36))))));
                            v1786 = v1785;
                        }
                        let v1789 = v1739 - v1773;
                        let v1793 = (v11 * v1789) + (v403 * (v71 - v1786));
                        let v1798 = (v1789 * v1789) - (v403 * ((v1773 - v71) + v1786));
                        let v1807 = v1773 + ((v11 * v1798) / (v1793 + (((v1793 * v1793) - ((v97 * (v71 - (v1766 * v1786))) * v1798)).sqrt())));
                        v1902 = v1807;
                    } else {
                        let v1808 = -v1739;
                        let v1810 = (v174 * v1808) / v406;
                        let v1812 = v1810 - v155;
                        let v1817 = v39 * ((v1810 + v451) - (((v1812 * v1812) + v455).sqrt()));
                        let v1818 = v1808 - v1817;
                        let v1822 = (v1818 * v1818) + (v403 * (v1817 + v71));
                        let v1824 = (v11 * v1818) - v403;
                        let v1827 = ((v1822 / v403).ln()) - v1817;
                        let v1828 = v1822 + v1824;
                        let v1834 = (v1828 * v1828) + ((((v39 * v1824) * v1824) - v1822) * v1827);
                        let v1847 = v1817 + (((v1822 * v1828) * v1827) / (v1834 + (((((v1828 * v1827) * v1827) / v1834) * v1824) * (((v1824 * v1824) * v36) - v1822))));
                        let v1849 = if (v1847.abs()) < v491 { 1.0 } else { 0.0 };
                        let v1877: f64;
                        if v1849 != 0.0 {
                            let v1850 = v1847.exp();
                            v1877 = v1850;
                        } else {
                            let v1852 = if v1847 < v1851 { 1.0 } else { 0.0 };
                            let v1878: f64;
                            if v1852 != 0.0 {
                                let v1866 = v551 / (v71 + ((v1853 - v1847) * (v71 + ((v39 * (v1855 - v1847)) * (v71 + ((v1858 - v1847) * v36))))));
                                v1878 = v1866;
                            } else {
                                let v1867 = v1847 - v491;
                                let v1875 = v494 * (v71 + (v1867 * (v71 + ((v39 * v1867) * (v71 + (v1867 * v36))))));
                                v1878 = v1875;
                            }
                            v1877 = v1878;
                        }
                        let v1881 = v1808 - v1847;
                        let v1885 = (v11 * v1881) + (v403 * (v1877 - v71));
                        let v1890 = (v1881 * v1881) + (v403 * ((v1847 + v71) - v1877));
                        let v1900 = -(v1847 + ((v11 * v1890) / (v1885 + (((v1885 * v1885) - ((v97 * (v71 - ((v39 * v403) * v1877))) * v1890)).sqrt()))));
                        v1902 = v1900;
                    }
                    v1901 = v1902;
                }
                v1903 = v1901;
            } else {
                v1903 = v1454;
            }
            let v1904 = if v1903 < v491 { 1.0 } else { 0.0 };
            let v1923: f64;
            if v1904 != 0.0 {
                let v1906 = v71 / (v1903.exp());
                v1923 = v1906;
            } else {
                let v1908 = if v1903 > (v410 - v491) { 1.0 } else { 0.0 };
                let v1924: f64;
                if v1908 != 0.0 {
                    let v1911 = v439 * ((v410 - v1903).exp());
                    v1924 = v1911;
                } else {
                    let v1912 = v1903 - v491;
                    let v1920 = v551 / (v71 + (v1912 * (v71 + ((v39 * v1912) * (v71 + (v1912 * v36))))));
                    v1924 = v1920;
                }
                v1923 = v1924;
            }
            let v1922 = if v1903 < (-v408) { 1.0 } else { 0.0 };
            let v1942: f64;
            if v1922 != 0.0 {
                let v1928 = -(((v1923 + v1903) - v71).sqrt());
                v1942 = v1928;
            } else {
                let v1930 = if (v1903.abs()) <= v408 { 1.0 } else { 0.0 };
                let v1943: f64;
                if v1930 != 0.0 {
                    let v1938 = (v162 * v1903) * ((v71 - ((v36 * v1903) * (v71 - (v570 * v1903)))).sqrt());
                    v1943 = v1938;
                } else {
                    let v1941 = ((v1903 - v71) + v1923).sqrt();
                    v1943 = v1941;
                }
                v1942 = v1943;
            }
            let v1945 = (v67 * v1942) * v402;
            let v1947 = v71 + v377;
            let v1952 = v71 + (v1950 * v42);
            let v1959 = (((((((v1946 * v1947) * v1947) * v1952) * v1952) * v64) * (v64.sqrt())) * v67) * v67;
            let v1960 = -v1945;
            let v1961 = v1945 - v1960;
            let v1962 = if v1961 > v325 { 1.0 } else { 0.0 };
            let v1984: f64;
            if v1962 != 0.0 {
                let v1968 = v1960 + (v39 * (v1961 + (((v1961 * v1961) + v1959).sqrt())));
                v1984 = v1968;
            } else {
                let v1969 = v1960 - v1945;
                let v1970 = if v1969 > v325 { 1.0 } else { 0.0 };
                let v1983: f64;
                if v1970 != 0.0 {
                    let v1977 = v1960 + ((v39 * v1959) / (v1969 + (((v1969 * v1969) + v1959).sqrt())));
                    v1983 = v1977;
                } else {
                    let v1982 = v1960 + (v39 * (v1961 + ((v341 + v1959).sqrt())));
                    v1983 = v1982;
                }
                v1984 = v1983;
            }
            let v1986 = -v1290;
            let v1987 = v1986 - v1290;
            let v1988 = if v1987 > v325 { 1.0 } else { 0.0 };
            let v2010: f64;
            if v1988 != 0.0 {
                let v1994 = v1290 + (v39 * (v1987 + (((v1987 * v1987) + v1959).sqrt())));
                v2010 = v1994;
            } else {
                let v1995 = v1290 - v1986;
                let v1996 = if v1995 > v325 { 1.0 } else { 0.0 };
                let v2009: f64;
                if v1996 != 0.0 {
                    let v2003 = v1290 + ((v39 * v1959) / (v1995 + (((v1995 * v1995) + v1959).sqrt())));
                    v2009 = v2003;
                } else {
                    let v2008 = v1290 + (v39 * (v1987 + ((v341 + v1959).sqrt())));
                    v2009 = v2008;
                }
                v2010 = v2009;
            }
            let v2012 = v1984 + (v1985 * v2010);
            let v2013 = if v390 > v0 { 1.0 } else { 0.0 };
            let v2050: f64;
            if v2013 != 0.0 {
                let v2020 = v10 / (v71 + (v390 * (((v2012 * v2012) + v70).powf(v2016))));
                v2050 = v2020;
            } else {
                v2050 = v10;
            }
            let v2022 = v451 - v1283;
            let v2023 = if v2022 > v325 { 1.0 } else { 0.0 };
            let v2045: f64;
            if v2023 != 0.0 {
                let v2030 = v451 - (v39 * (v2022 + (((v2022 * v2022) + v2025).sqrt())));
                v2045 = v2030;
            } else {
                let v2031 = v1283 - v451;
                let v2032 = if v2031 > v325 { 1.0 } else { 0.0 };
                let v2044: f64;
                if v2032 != 0.0 {
                    let v2039 = v451 - (v2033 / (v2031 + (((v2031 * v2031) + v2025).sqrt())));
                    v2044 = v2039;
                } else {
                    let v2043 = v451 - (v39 * (v2022 + v2040));
                    v2044 = v2043;
                }
                v2045 = v2044;
            }
            let v2064 = (v247 * ((v401 * v2050) * ((v67 * ((v2021 * v2045).exp())).sqrt()))) / (v71 + (v2061 * (v39 * ((-v430) + (((v430 * v430) + v2055).sqrt())))));
            let v2065 = if v245 == v11 { 1.0 } else { 0.0 };
            let v2766: f64;
            if v2065 != 0.0 {
                let v2066 = v99 * v2064;
                v2766 = v2066;
            } else {
                v2766 = v2767;
            }
            let v2069 = if (v719 * v31) == v2068 { 1.0 } else { 0.0 };
            let v2073: f64;
            if v2069 != 0.0 {
                let v2070 = v719 * v122;
                v2073 = v2070;
            } else {
                v2073 = v0;
            }
            let v2072 = v319 - v2071;
            let v2076 = (v31 * (v2072 - v2073)) * v72;
            let v2079 = if v2078 > v0 { 1.0 } else { 0.0 };
            let v2081 = if v2080 > v0 { 1.0 } else { 0.0 };
            let v2082 = if v2079 != 0.0 || v2081 != 0.0 { 1.0 } else { 0.0 };
            let v2083 = if (if v249 != v0 { 1.0 } else { 0.0 }) != 0.0 && v2082 != 0.0 { 1.0 } else { 0.0 };
            let v2252: f64;
            let v2310: f64;
            if v2083 != 0.0 {
                let v2085 = if (v2076.abs()) <= v173 { 1.0 } else { 0.0 };
                let v2245: f64;
                if v2085 != 0.0 {
                    let v2086 = v2076 / v172;
                    v2245 = v2086;
                } else {
                    let v2087 = if v2076 > v173 { 1.0 } else { 0.0 };
                    let v2246: f64;
                    if v2087 != 0.0 {
                        let v2095 = (v2076 / v172) * (v71 + (((((v172 * v174) / v428) - v71) / v428) * v2076));
                        let v2096 = if v2095 < v175 { 1.0 } else { 0.0 };
                        let v2108: f64;
                        if v2096 != 0.0 {
                            let v2098 = (-v2095).exp();
                            v2108 = v2098;
                        } else {
                            let v2099 = v2095 - v175;
                            let v2107 = v179 / (v71 + (v2099 * (v71 + ((v39 * v2099) * (v71 + (v2099 * v36))))));
                            v2108 = v2107;
                        }
                        let v2110 = v39 * v170;
                        let v2117 = (v2076 + v2110) - (v169 * (((v2076 + (v570 * v170)) - (v71 - v2108)).sqrt()));
                        let v2118 = if v2117 < v175 { 1.0 } else { 0.0 };
                        let v2130: f64;
                        if v2118 != 0.0 {
                            let v2120 = (-v2117).exp();
                            v2130 = v2120;
                        } else {
                            let v2121 = v2117 - v175;
                            let v2129 = v179 / (v71 + (v2121 * (v71 + ((v39 * v2121) * (v71 + (v2121 * v36))))));
                            v2130 = v2129;
                        }
                        let v2133 = v2076 - v2117;
                        let v2137 = (v11 * v2133) + (v170 * (v71 - v2130));
                        let v2142 = (v2133 * v2133) - (v170 * ((v2117 - v71) + v2130));
                        let v2151 = v2117 + ((v11 * v2142) / (v2137 + (((v2137 * v2137) - ((v97 * (v71 - (v2110 * v2130))) * v2142)).sqrt())));
                        v2246 = v2151;
                    } else {
                        let v2152 = -v2076;
                        let v2154 = (v174 * v2152) / v172;
                        let v2156 = v2154 - v155;
                        let v2161 = v39 * ((v2154 + v451) - (((v2156 * v2156) + v455).sqrt()));
                        let v2162 = v2152 - v2161;
                        let v2166 = (v2162 * v2162) + (v170 * (v2161 + v71));
                        let v2168 = (v11 * v2162) - v170;
                        let v2171 = ((v2166 / v170).ln()) - v2161;
                        let v2172 = v2166 + v2168;
                        let v2178 = (v2172 * v2172) + ((((v39 * v2168) * v2168) - v2166) * v2171);
                        let v2191 = v2161 + (((v2166 * v2172) * v2171) / (v2178 + (((((v2172 * v2171) * v2171) / v2178) * v2168) * (((v2168 * v2168) * v36) - v2166))));
                        let v2193 = if (v2191.abs()) < v491 { 1.0 } else { 0.0 };
                        let v2221: f64;
                        if v2193 != 0.0 {
                            let v2194 = v2191.exp();
                            v2221 = v2194;
                        } else {
                            let v2196 = if v2191 < v2195 { 1.0 } else { 0.0 };
                            let v2222: f64;
                            if v2196 != 0.0 {
                                let v2210 = v551 / (v71 + ((v2197 - v2191) * (v71 + ((v39 * (v2199 - v2191)) * (v71 + ((v2202 - v2191) * v36))))));
                                v2222 = v2210;
                            } else {
                                let v2211 = v2191 - v491;
                                let v2219 = v494 * (v71 + (v2211 * (v71 + ((v39 * v2211) * (v71 + (v2211 * v36))))));
                                v2222 = v2219;
                            }
                            v2221 = v2222;
                        }
                        let v2225 = v2152 - v2191;
                        let v2229 = (v11 * v2225) + (v170 * (v2221 - v71));
                        let v2234 = (v2225 * v2225) + (v170 * ((v2191 + v71) - v2221));
                        let v2244 = -(v2191 + ((v11 * v2234) / (v2229 + (((v2229 * v2229) - ((v97 * (v71 - ((v39 * v170) * v2221))) * v2234)).sqrt()))));
                        v2246 = v2244;
                    }
                    v2245 = v2246;
                }
                let v2248 = v67 * (v2076 - v2245);
                v2252 = v2248;
                v2310 = v2245;
            } else {
                v2252 = v0;
                v2310 = v0;
            }
            let v2750: f64;
            let v2753: f64;
            if v249 != 0.0 {
                let v2754: f64;
                if v2082 != 0.0 {
                    let v2249 = v31 * v2072;
                    let v2251 = if (if v719 == v71 { 1.0 } else { 0.0 }) != 0.0 && v2081 != 0.0 { 1.0 } else { 0.0 };
                    let v2491: f64;
                    if v2251 != 0.0 {
                        let v2255 = (v31 * v2252) + v2254;
                        let v2256 = v0 - v2255;
                        let v2257 = if v2256 > v325 { 1.0 } else { 0.0 };
                        let v2277: f64;
                        if v2257 != 0.0 {
                            let v2263 = v2255 + (v39 * (v2256 + (((v2256 * v2256) + v2025).sqrt())));
                            v2277 = v2263;
                        } else {
                            let v2264 = if v2255 > v325 { 1.0 } else { 0.0 };
                            let v2276: f64;
                            if v2264 != 0.0 {
                                let v2271 = v2255 + (v2265 / (v2255 + (((v2255 * v2255) + v2025).sqrt())));
                                v2276 = v2271;
                            } else {
                                let v2275 = v2255 + (v39 * (v2256 + v2272));
                                v2276 = v2275;
                            }
                            v2277 = v2276;
                        }
                        let v2282 = (((v2252 * v2252) + v354).sqrt()) * v2281;
                        let v2283 = if v301 < v0 { 1.0 } else { 0.0 };
                        let v2339: f64;
                        if v2283 != 0.0 {
                            let v2286 = v2284 - v2282;
                            let v2287 = if v2286 > v325 { 1.0 } else { 0.0 };
                            let v2308: f64;
                            if v2287 != 0.0 {
                                let v2293 = v2284 - (v39 * (v2286 + (((v2286 * v2286) + v354).sqrt())));
                                v2308 = v2293;
                            } else {
                                let v2294 = v2282 - v2284;
                                let v2295 = if v2294 > v325 { 1.0 } else { 0.0 };
                                let v2307: f64;
                                if v2295 != 0.0 {
                                    let v2302 = v2284 - (v2296 / (v2294 + (((v2294 * v2294) + v354).sqrt())));
                                    v2307 = v2302;
                                } else {
                                    let v2306 = v2284 - (v39 * (v2286 + v2303));
                                    v2307 = v2306;
                                }
                                v2308 = v2307;
                            }
                            v2339 = v2308;
                        } else {
                            v2339 = v2282;
                        }
                        let v2325: f64;
                        if v2309 != 0.0 {
                            let v2317 = -((v31 * v2310) + (((v122 - v2312) + v2277) * v72));
                            v2325 = v2317;
                        } else {
                            let v2324 = -((v31 * v2310) + (((v122 - v2319) + v2277) * v72));
                            v2325 = v2324;
                        }
                        let v2326 = if v2325 < v491 { 1.0 } else { 0.0 };
                        let v2375: f64;
                        if v2326 != 0.0 {
                            let v2329 = (v71 + (v2325.exp())).ln();
                            v2375 = v2329;
                        } else {
                            v2375 = v2325;
                        }
                        let v2332 = v2325 + ((v31 * v2249) * v72);
                        let v2333 = if v2332 < v491 { 1.0 } else { 0.0 };
                        let v2374: f64;
                        if v2333 != 0.0 {
                            let v2336 = (v71 + (v2332.exp())).ln();
                            v2374 = v2336;
                        } else {
                            v2374 = v2332;
                        }
                        let v2344 = v2337 * (v2338 + (v2339 * (v304 + (v301 * v2339))));
                        let v2345 = if v2344 > v0 { 1.0 } else { 0.0 };
                        let v2370: f64;
                        if v2345 != 0.0 {
                            let v2352 = v71 + (v2344 * (v71 + ((v39 * v2344) * (v71 + (v2344 * v36)))));
                            v2370 = v2352;
                        } else {
                            let v2354 = if v2344 > v2353 { 1.0 } else { 0.0 };
                            let v2371: f64;
                            if v2354 != 0.0 {
                                let v2355 = v2344.exp();
                                v2371 = v2355;
                            } else {
                                let v2369 = v551 / (v71 + ((v2356 - v2344) * (v71 + ((v39 * (v2358 - v2344)) * (v71 + ((v2361 - v2344) * v36))))));
                                v2371 = v2369;
                            }
                            v2370 = v2371;
                        }
                        let v2377 = ((v2080 * v2370) * v31) * (v2374 - v2375);
                        v2491 = v2377;
                    } else {
                        v2491 = v0;
                    }
                    let v2755: f64;
                    if v2079 != 0.0 {
                        let v2380 = (v31 * v2252) + v2379;
                        let v2381 = if v2380 > v325 { 1.0 } else { 0.0 };
                        let v2402: f64;
                        if v2381 != 0.0 {
                            let v2387 = v2380 - (v39 * (v2380 + (((v2380 * v2380) + v2025).sqrt())));
                            v2402 = v2387;
                        } else {
                            let v2388 = v0 - v2380;
                            let v2389 = if v2388 > v325 { 1.0 } else { 0.0 };
                            let v2401: f64;
                            if v2389 != 0.0 {
                                let v2396 = v2380 - (v2390 / (v2388 + (((v2388 * v2388) + v2025).sqrt())));
                                v2401 = v2396;
                            } else {
                                let v2400 = v2380 - (v39 * (v2380 + v2397));
                                v2401 = v2400;
                            }
                            v2402 = v2401;
                        }
                        let v2407 = (((v2252 * v2252) + v354).sqrt()) * v2406;
                        let v2408 = if v295 < v0 { 1.0 } else { 0.0 };
                        let v2457: f64;
                        if v2408 != 0.0 {
                            let v2411 = v2409 - v2407;
                            let v2412 = if v2411 > v325 { 1.0 } else { 0.0 };
                            let v2433: f64;
                            if v2412 != 0.0 {
                                let v2418 = v2409 - (v39 * (v2411 + (((v2411 * v2411) + v354).sqrt())));
                                v2433 = v2418;
                            } else {
                                let v2419 = v2407 - v2409;
                                let v2420 = if v2419 > v325 { 1.0 } else { 0.0 };
                                let v2432: f64;
                                if v2420 != 0.0 {
                                    let v2427 = v2409 - (v2421 / (v2419 + (((v2419 * v2419) + v354).sqrt())));
                                    v2432 = v2427;
                                } else {
                                    let v2431 = v2409 - (v39 * (v2411 + v2428));
                                    v2432 = v2431;
                                }
                                v2433 = v2432;
                            }
                            v2457 = v2433;
                        } else {
                            v2457 = v2407;
                        }
                        let v2443: f64;
                        if v2434 != 0.0 {
                            let v2438 = (v31 * v2310) + ((v2402 - v2312) * v72);
                            v2443 = v2438;
                        } else {
                            let v2442 = (v31 * v2310) + ((v2402 - v2319) * v72);
                            v2443 = v2442;
                        }
                        let v2444 = if v2443 < v491 { 1.0 } else { 0.0 };
                        let v2496: f64;
                        if v2444 != 0.0 {
                            let v2447 = (v71 + (v2443.exp())).ln();
                            v2496 = v2447;
                        } else {
                            v2496 = v2443;
                        }
                        let v2450 = v2443 - ((v31 * v2249) * v72);
                        let v2451 = if v2450 < v491 { 1.0 } else { 0.0 };
                        let v2497: f64;
                        if v2451 != 0.0 {
                            let v2454 = (v71 + (v2450.exp())).ln();
                            v2497 = v2454;
                        } else {
                            v2497 = v2450;
                        }
                        let v2462 = v2455 * (v2456 + (v2457 * (v298 + (v295 * v2457))));
                        let v2464 = if (v2462.abs()) < v491 { 1.0 } else { 0.0 };
                        let v2492: f64;
                        if v2464 != 0.0 {
                            let v2465 = v2462.exp();
                            v2492 = v2465;
                        } else {
                            let v2467 = if v2462 < v2466 { 1.0 } else { 0.0 };
                            let v2493: f64;
                            if v2467 != 0.0 {
                                let v2481 = v551 / (v71 + ((v2468 - v2462) * (v71 + ((v39 * (v2470 - v2462)) * (v71 + ((v2473 - v2462) * v36))))));
                                v2493 = v2481;
                            } else {
                                let v2482 = v2462 - v491;
                                let v2490 = v494 * (v71 + (v2482 * (v71 + ((v39 * v2482) * (v71 + (v2482 * v36))))));
                                v2493 = v2490;
                            }
                            v2492 = v2493;
                        }
                        let v2500 = v2491 + (((v2078 * v2492) * v31) * (v2496 - v2497));
                        v2755 = v2500;
                    } else {
                        v2755 = v2491;
                    }
                    v2754 = v2755;
                } else {
                    v2754 = v0;
                }
                let v2502 = if v2501 > v0 { 1.0 } else { 0.0 };
                let v2504 = if v2503 > v0 { 1.0 } else { 0.0 };
                let v2505 = if v2502 != 0.0 || v2504 != 0.0 { 1.0 } else { 0.0 };
                let v2751: f64;
                if v2505 != 0.0 {
                    let v2506 = v31 * v321;
                    let v2508 = (v1278 - v1903) * v67;
                    let v2510 = if (if v719 == v71 { 1.0 } else { 0.0 }) != 0.0 && v2504 != 0.0 { 1.0 } else { 0.0 };
                    let v2738: f64;
                    if v2510 != 0.0 {
                        let v2512 = (v31 * v2508) + v2254;
                        let v2513 = v0 - v2512;
                        let v2514 = if v2513 > v325 { 1.0 } else { 0.0 };
                        let v2534: f64;
                        if v2514 != 0.0 {
                            let v2520 = v2512 + (v39 * (v2513 + (((v2513 * v2513) + v2025).sqrt())));
                            v2534 = v2520;
                        } else {
                            let v2521 = if v2512 > v325 { 1.0 } else { 0.0 };
                            let v2533: f64;
                            if v2521 != 0.0 {
                                let v2528 = v2512 + (v2522 / (v2512 + (((v2512 * v2512) + v2025).sqrt())));
                                v2533 = v2528;
                            } else {
                                let v2532 = v2512 + (v39 * (v2513 + v2529));
                                v2533 = v2532;
                            }
                            v2534 = v2533;
                        }
                        let v2538 = (((v2508 * v2508) + v354).sqrt()) * v2281;
                        let v2539 = if v301 < v0 { 1.0 } else { 0.0 };
                        let v2590: f64;
                        if v2539 != 0.0 {
                            let v2540 = v2284 - v2538;
                            let v2541 = if v2540 > v325 { 1.0 } else { 0.0 };
                            let v2562: f64;
                            if v2541 != 0.0 {
                                let v2547 = v2284 - (v39 * (v2540 + (((v2540 * v2540) + v354).sqrt())));
                                v2562 = v2547;
                            } else {
                                let v2548 = v2538 - v2284;
                                let v2549 = if v2548 > v325 { 1.0 } else { 0.0 };
                                let v2561: f64;
                                if v2549 != 0.0 {
                                    let v2556 = v2284 - (v2550 / (v2548 + (((v2548 * v2548) + v354).sqrt())));
                                    v2561 = v2556;
                                } else {
                                    let v2560 = v2284 - (v39 * (v2540 + v2557));
                                    v2561 = v2560;
                                }
                                v2562 = v2561;
                            }
                            v2590 = v2562;
                        } else {
                            v2590 = v2538;
                        }
                        let v2576: f64;
                        if v2563 != 0.0 {
                            let v2569 = -((v31 * v1903) + (((v122 - v2312) + v2534) * v72));
                            v2576 = v2569;
                        } else {
                            let v2575 = -((v31 * v1903) + (((v122 - v2319) + v2534) * v72));
                            v2576 = v2575;
                        }
                        let v2577 = if v2576 < v491 { 1.0 } else { 0.0 };
                        let v2626: f64;
                        if v2577 != 0.0 {
                            let v2580 = (v71 + (v2576.exp())).ln();
                            v2626 = v2580;
                        } else {
                            v2626 = v2576;
                        }
                        let v2583 = v2576 + ((v31 * v2506) * v72);
                        let v2584 = if v2583 < v491 { 1.0 } else { 0.0 };
                        let v2625: f64;
                        if v2584 != 0.0 {
                            let v2587 = (v71 + (v2583.exp())).ln();
                            v2625 = v2587;
                        } else {
                            v2625 = v2583;
                        }
                        let v2595 = v2588 * (v2589 + (v2590 * (v304 + (v301 * v2590))));
                        let v2596 = if v2595 > v0 { 1.0 } else { 0.0 };
                        let v2621: f64;
                        if v2596 != 0.0 {
                            let v2603 = v71 + (v2595 * (v71 + ((v39 * v2595) * (v71 + (v2595 * v36)))));
                            v2621 = v2603;
                        } else {
                            let v2605 = if v2595 > v2604 { 1.0 } else { 0.0 };
                            let v2622: f64;
                            if v2605 != 0.0 {
                                let v2606 = v2595.exp();
                                v2622 = v2606;
                            } else {
                                let v2620 = v551 / (v71 + ((v2607 - v2595) * (v71 + ((v39 * (v2609 - v2595)) * (v71 + ((v2612 - v2595) * v36))))));
                                v2622 = v2620;
                            }
                            v2621 = v2622;
                        }
                        let v2628 = ((v2503 * v2621) * v31) * (v2625 - v2626);
                        v2738 = v2628;
                    } else {
                        v2738 = v0;
                    }
                    let v2752: f64;
                    if v2502 != 0.0 {
                        let v2630 = (v31 * v2508) + v2379;
                        let v2631 = if v2630 > v325 { 1.0 } else { 0.0 };
                        let v2652: f64;
                        if v2631 != 0.0 {
                            let v2637 = v2630 - (v39 * (v2630 + (((v2630 * v2630) + v2025).sqrt())));
                            v2652 = v2637;
                        } else {
                            let v2638 = v0 - v2630;
                            let v2639 = if v2638 > v325 { 1.0 } else { 0.0 };
                            let v2651: f64;
                            if v2639 != 0.0 {
                                let v2646 = v2630 - (v2640 / (v2638 + (((v2638 * v2638) + v2025).sqrt())));
                                v2651 = v2646;
                            } else {
                                let v2650 = v2630 - (v39 * (v2630 + v2647));
                                v2651 = v2650;
                            }
                            v2652 = v2651;
                        }
                        let v2656 = (((v2508 * v2508) + v354).sqrt()) * v2406;
                        let v2657 = if v295 < v0 { 1.0 } else { 0.0 };
                        let v2704: f64;
                        if v2657 != 0.0 {
                            let v2658 = v2409 - v2656;
                            let v2659 = if v2658 > v325 { 1.0 } else { 0.0 };
                            let v2680: f64;
                            if v2659 != 0.0 {
                                let v2665 = v2409 - (v39 * (v2658 + (((v2658 * v2658) + v354).sqrt())));
                                v2680 = v2665;
                            } else {
                                let v2666 = v2656 - v2409;
                                let v2667 = if v2666 > v325 { 1.0 } else { 0.0 };
                                let v2679: f64;
                                if v2667 != 0.0 {
                                    let v2674 = v2409 - (v2668 / (v2666 + (((v2666 * v2666) + v354).sqrt())));
                                    v2679 = v2674;
                                } else {
                                    let v2678 = v2409 - (v39 * (v2658 + v2675));
                                    v2679 = v2678;
                                }
                                v2680 = v2679;
                            }
                            v2704 = v2680;
                        } else {
                            v2704 = v2656;
                        }
                        let v2690: f64;
                        if v2681 != 0.0 {
                            let v2685 = (v31 * v1903) + ((v2652 - v2312) * v72);
                            v2690 = v2685;
                        } else {
                            let v2689 = (v31 * v1903) + ((v2652 - v2319) * v72);
                            v2690 = v2689;
                        }
                        let v2691 = if v2690 < v491 { 1.0 } else { 0.0 };
                        let v2743: f64;
                        if v2691 != 0.0 {
                            let v2694 = (v71 + (v2690.exp())).ln();
                            v2743 = v2694;
                        } else {
                            v2743 = v2690;
                        }
                        let v2697 = v2690 - ((v31 * v2506) * v72);
                        let v2698 = if v2697 < v491 { 1.0 } else { 0.0 };
                        let v2744: f64;
                        if v2698 != 0.0 {
                            let v2701 = (v71 + (v2697.exp())).ln();
                            v2744 = v2701;
                        } else {
                            v2744 = v2697;
                        }
                        let v2709 = v2702 * (v2703 + (v2704 * (v298 + (v295 * v2704))));
                        let v2711 = if (v2709.abs()) < v491 { 1.0 } else { 0.0 };
                        let v2739: f64;
                        if v2711 != 0.0 {
                            let v2712 = v2709.exp();
                            v2739 = v2712;
                        } else {
                            let v2714 = if v2709 < v2713 { 1.0 } else { 0.0 };
                            let v2740: f64;
                            if v2714 != 0.0 {
                                let v2728 = v551 / (v71 + ((v2715 - v2709) * (v71 + ((v39 * (v2717 - v2709)) * (v71 + ((v2720 - v2709) * v36))))));
                                v2740 = v2728;
                            } else {
                                let v2729 = v2709 - v491;
                                let v2737 = v494 * (v71 + (v2729 * (v71 + ((v39 * v2729) * (v71 + (v2729 * v36))))));
                                v2740 = v2737;
                            }
                            v2739 = v2740;
                        }
                        let v2747 = v2738 + (((v2501 * v2739) * v31) * (v2743 - v2744));
                        v2752 = v2747;
                    } else {
                        v2752 = v2738;
                    }
                    v2751 = v2752;
                } else {
                    v2751 = v0;
                }
                v2750 = v2751;
                v2753 = v2754;
            } else {
                v2750 = v0;
                v2753 = v0;
            }
            if v189 != 0.0 {
            } else {
            }
            let v2759 = if ((v2750 + v2753).abs()) > v2758 { 1.0 } else { 0.0 };
            if v2759 != 0.0 {
            } else {
            }
            let v2772: f64;
            let v2773: f64;
            let v2774: f64;
            let v2775: f64;
            if v249 != 0.0 {
                let v2762 = v2760 * (v2750.abs());
                let v2765 = v2763 * (v2753.abs());
                v2772 = v71;
                v2773 = v2762;
                v2774 = v71;
                v2775 = v2765;
            } else {
                v2772 = v0;
                v2773 = v0;
                v2774 = v0;
                v2775 = v0;
            }
            let v2776: f64;
            let v2777: f64;
            let v2778: f64;
            let v2779: f64;
            let v2780: f64;
            let v2781: f64;
            let v2782: f64;
            let v2783: f64;
            let v2784: f64;
            let v2785: f64;
            if v189 != 0.0 {
                v2776 = v71;
                v2777 = v238;
                v2778 = v71;
                v2779 = v240;
                v2780 = v71;
                v2781 = v242;
                v2782 = v71;
                v2783 = v244;
                v2784 = v71;
                v2785 = v2766;
            } else {
                v2776 = v0;
                v2777 = v0;
                v2778 = v0;
                v2779 = v0;
                v2780 = v0;
                v2781 = v0;
                v2782 = v0;
                v2783 = v0;
                v2784 = v0;
                v2785 = v0;
            }
            let v2771 = if ((v2748 - v2749).abs()) > v2770 { 1.0 } else { 0.0 };
            if v2771 != 0.0 {
            } else {
            }
            if v189 != 0.0 {
            } else {
            }
        if v2772 == 0.0 {
            if !visitor.visit(0, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v2773;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 0, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 0, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 0, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(0, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v2774 == 0.0 {
            if !visitor.visit(1, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v2775;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 1, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(1, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v2776 == 0.0 {
            if !visitor.visit(2, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v2777;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 2, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 2, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 2, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(2, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v2778 == 0.0 {
            if !visitor.visit(3, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v2779;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 3, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 3, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 3, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(3, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v2780 == 0.0 {
            if !visitor.visit(4, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v2781;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 4, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 4, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 4, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(4, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v2782 == 0.0 {
            if !visitor.visit(5, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v2783;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 5, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 5, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 5, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(5, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v2784 == 0.0 {
            if !visitor.visit(6, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v2785;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 6, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 6, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 6, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(6, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        Ok(())
    }
}
