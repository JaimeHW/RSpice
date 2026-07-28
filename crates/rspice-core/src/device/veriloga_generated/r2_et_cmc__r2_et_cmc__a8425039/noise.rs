#![allow(dead_code, non_snake_case, unused_parens, unused_variables)]

use super::state::Instance;
use crate::device::veriloga_generated::GeneratedEvalContext;
pub use crate::device::veriloga_generated::{GeneratedNoiseDescriptor, GeneratedNoiseEndpoint, GeneratedNoiseEvaluation, GeneratedNoiseEvaluationError, GeneratedNoiseEvaluationRef, GeneratedNoiseKind, GeneratedNoiseVisitor};

pub static NOISE_SOURCES: [GeneratedNoiseDescriptor; 2] = [
    GeneratedNoiseDescriptor { mechanism: "WHITE_N1_N2_THERMAL", label: Some("thermal"), kind: GeneratedNoiseKind::White, equation: 5, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(0), name: "n1", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(1), name: "n2", is_internal: false }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_N1_N2_FLICKER", label: Some("flicker"), kind: GeneratedNoiseKind::Flicker, equation: 6, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(0), name: "n1", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(1), name: "n2", is_internal: false }, table_len: 0, table_log_interp: false },
];

impl Instance {
    pub fn evaluate_noise_sources(&self, ctx: &GeneratedEvalContext<'_>, visitor: &mut dyn GeneratedNoiseVisitor) -> Result<(), GeneratedNoiseEvaluationError> {
        if !self.multiplicity.is_finite() || self.multiplicity <= 0.0 {
            return Err(GeneratedNoiseEvaluationError::InvalidMultiplicity { value: self.multiplicity });
        }
        let parameters = &self.params.values;
        let parameter_given = &*self.param_given;
        let multiplicity = self.multiplicity;
        let temperature = ctx.temperature();
        let node_potentials = [ctx.node_voltage(self.nodes[0]), ctx.node_voltage(self.nodes[1]), ctx.node_voltage(self.nodes[2])];
            let v0 = 0e0f64;
            let v1 = parameters[15];
            let v2 = 1.002e3f64;
            let v4 = if parameter_given[10] { 1.0 } else { 0.0 };
            let v5 = parameters[10];
            let v6 = 1e0f64;
            let v8 = if parameter_given[11] { 1.0 } else { 0.0 };
            let v9 = 1e-2f64;
            let v10 = parameters[11];
            let v16 = if parameter_given[14] { 1.0 } else { 0.0 };
            let v17 = parameters[14];
            let v18 = 1e-3f64;
            let v23 = 1e6f64;
            let v25 = 2.7315e2f64;
            let v26 = parameters[16];
            let v28 = temperature;
            let v29 = parameters[5];
            let v32 = parameters[12];
            let v34 = parameters[13];
            let v36 = parameters[3];
            let v37 = parameters[4];
            let v39 = parameters[23];
            let v41 = 5e-1f64;
            let v43 = if parameter_given[1] { 1.0 } else { 0.0 };
            let v44 = if parameter_given[2] { 1.0 } else { 0.0 };
            let v46 = if parameter_given[0] { 1.0 } else { 0.0 };
            let v49 = parameters[2];
            let v51 = parameters[1];
            let v54 = parameters[0];
            let v56 = parameters[22];
            let v58 = 1e99f64;
            let v65 = parameters[17];
            let v117 = parameters[18];
            let v119 = parameters[19];
            let v129 = parameters[20];
            let v131 = parameters[21];
            let v133 = parameters[25];
            let v142 = parameters[24];
            let v160 = parameters[29];
            let v162 = parameters[27];
            let v166 = parameters[37];
            let v167 = parameters[38];
            let v169 = parameters[39];
            let v172 = parameters[40];
            let v194 = parameters[41];
            let v200 = parameters[42];
            let v204 = multiplicity;
            let v207 = 2e0f64;
            let v215 = parameters[44];
            let v216 = parameters[45];
            let v221 = parameters[46];
            let v224 = node_potentials[2];
            let v225 = parameters[7];
            let v228 = parameters[35];
            let v235 = parameters[36];
            let v252 = 1e-1f64;
            let v253 = 1.1e-1f64;
            let v255 = 1e1f64;
            let v276 = parameters[43];
            let v279 = parameters[30];
            let v282 = node_potentials[0];
            let v283 = node_potentials[1];
            let v287 = parameters[28];
            let v292 = parameters[26];
            let v298 = 3.333333333333333e-1f64;
            let v311 = parameters[34];
            let v313 = parameters[6];
            let v317 = 5.522602e-23f64;
            let v321 = parameters[33];
            let v327 = parameters[31];
            let v348 = parameters[32];
            let v3 = if v1 != v2 { 1.0 } else { 0.0 };
            if v3 != 0.0 {
            } else {
            }
            let v21: f64;
            if v4 != 0.0 {
                v21 = v5;
            } else {
                let v7 = ctx.simparam_or("scale", v6);
                v21 = v7;
            }
            let v20: f64;
            if v8 != 0.0 {
                let v12 = v6 - (v9 * v10);
                v20 = v12;
            } else {
                let v15 = v6 - (v9 * (ctx.simparam_or("shrink", v0)));
                v20 = v15;
            }
            let v203: f64;
            if v16 != 0.0 {
                v203 = v17;
            } else {
                let v19 = ctx.simparam_or("rthresh", v18);
                v203 = v19;
            }
            let v24 = (v20 * v21) * v23;
            let v27 = v25 + v26;
            let v31 = (v28 + v29) - v25;
            let v33 = if v31 < v32 { 1.0 } else { 0.0 };
            if v33 != 0.0 {
            } else {
            }
            let v35 = if v31 > v34 { 1.0 } else { 0.0 };
            if v35 != 0.0 {
            } else {
            }
            let v38 = if v36 != 0.0 && v37 != 0.0 { 1.0 } else { 0.0 };
            let v60: f64;
            if v38 != 0.0 {
                v60 = v39;
            } else {
                let v40 = if v36 != 0.0 || v37 != 0.0 { 1.0 } else { 0.0 };
                let v61: f64;
                if v40 != 0.0 {
                    let v42 = v39 * v41;
                    v61 = v42;
                } else {
                    v61 = v0;
                }
                v60 = v61;
            }
            let v48 = if (if v43 != 0.0 && v44 != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if v46 == 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let v109: f64;
            let v121: f64;
            let v134: f64;
            let v147: f64;
            let v182: f64;
            let v264: f64;
            if v48 != 0.0 {
                let v53 = if (if v49 == v0 { 1.0 } else { 0.0 }) != 0.0 || (if v51 == v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v110: f64;
                let v122: f64;
                let v135: f64;
                let v148: f64;
                let v183: f64;
                let v265: f64;
                if v53 != 0.0 {
                    let v55 = v54 * v24;
                    let v57 = v55 + v56;
                    v110 = v0;
                    v122 = v55;
                    v135 = v0;
                    v148 = v0;
                    v183 = v57;
                    v265 = v58;
                } else {
                    let v59 = v51 * v24;
                    let v62 = v59 + v60;
                    let v63 = if v62 < v0 { 1.0 } else { 0.0 };
                    if v63 != 0.0 {
                    } else {
                    }
                    let v64 = if v62 > v0 { 1.0 } else { 0.0 };
                    let v123: f64;
                    let v149: f64;
                    let v184: f64;
                    let v266: f64;
                    if v64 != 0.0 {
                        let v67 = (v65 / v49) * v62;
                        let v68 = v67 - v56;
                        let v69 = if v68 <= v0 { 1.0 } else { 0.0 };
                        if v69 != 0.0 {
                        } else {
                        }
                        let v70 = v6 / v49;
                        v123 = v68;
                        v149 = v49;
                        v184 = v67;
                        v266 = v70;
                    } else {
                        let v71 = v54 * v24;
                        let v72 = v71 + v56;
                        v123 = v71;
                        v149 = v0;
                        v184 = v72;
                        v266 = v58;
                    }
                    v110 = v59;
                    v122 = v123;
                    v135 = v62;
                    v148 = v149;
                    v183 = v184;
                    v265 = v266;
                }
                v109 = v110;
                v121 = v122;
                v134 = v135;
                v147 = v148;
                v182 = v183;
                v264 = v265;
            } else {
                let v74 = if v44 != 0.0 && (if v43 == 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v111: f64;
                let v124: f64;
                let v136: f64;
                let v150: f64;
                let v185: f64;
                let v267: f64;
                if v74 != 0.0 {
                    let v75 = if v49 == v0 { 1.0 } else { 0.0 };
                    let v112: f64;
                    let v125: f64;
                    let v137: f64;
                    let v151: f64;
                    let v186: f64;
                    let v268: f64;
                    if v75 != 0.0 {
                        let v76 = v54 * v24;
                        let v77 = v76 + v56;
                        v112 = v0;
                        v125 = v76;
                        v137 = v0;
                        v151 = v0;
                        v186 = v77;
                        v268 = v58;
                    } else {
                        let v78 = if v54 == v0 { 1.0 } else { 0.0 };
                        let v113: f64;
                        let v126: f64;
                        let v138: f64;
                        let v152: f64;
                        let v187: f64;
                        let v269: f64;
                        if v78 != 0.0 {
                            let v79 = v51 * v24;
                            let v80 = v79 + v60;
                            v113 = v79;
                            v126 = v0;
                            v138 = v80;
                            v152 = v58;
                            v187 = v0;
                            v269 = v0;
                        } else {
                            let v81 = v54 * v24;
                            let v82 = v81 + v56;
                            let v83 = if v82 < v0 { 1.0 } else { 0.0 };
                            if v83 != 0.0 {
                            } else {
                            }
                            let v84 = if v82 > v0 { 1.0 } else { 0.0 };
                            let v114: f64;
                            let v139: f64;
                            let v153: f64;
                            let v270: f64;
                            if v84 != 0.0 {
                                let v86 = (v49 / v65) * v82;
                                let v87 = v86 - v60;
                                let v88 = if v87 <= v0 { 1.0 } else { 0.0 };
                                if v88 != 0.0 {
                                } else {
                                }
                                let v89 = v6 / v49;
                                v114 = v87;
                                v139 = v86;
                                v153 = v49;
                                v270 = v89;
                            } else {
                                let v90 = v51 * v24;
                                let v91 = v90 + v60;
                                v114 = v90;
                                v139 = v91;
                                v153 = v58;
                                v270 = v0;
                            }
                            v113 = v114;
                            v126 = v81;
                            v138 = v139;
                            v152 = v153;
                            v187 = v82;
                            v269 = v270;
                        }
                        v112 = v113;
                        v125 = v126;
                        v137 = v138;
                        v151 = v152;
                        v186 = v187;
                        v268 = v269;
                    }
                    v111 = v112;
                    v124 = v125;
                    v136 = v137;
                    v150 = v151;
                    v185 = v186;
                    v267 = v268;
                } else {
                    let v92 = if v54 == v0 { 1.0 } else { 0.0 };
                    let v115: f64;
                    let v127: f64;
                    let v140: f64;
                    let v154: f64;
                    let v188: f64;
                    let v271: f64;
                    if v92 != 0.0 {
                        let v93 = v51 * v24;
                        let v94 = v93 + v60;
                        v115 = v93;
                        v127 = v0;
                        v140 = v94;
                        v154 = v58;
                        v188 = v0;
                        v271 = v0;
                    } else {
                        let v95 = if v51 == v0 { 1.0 } else { 0.0 };
                        let v116: f64;
                        let v128: f64;
                        let v141: f64;
                        let v155: f64;
                        let v189: f64;
                        let v272: f64;
                        if v95 != 0.0 {
                            let v96 = v54 * v24;
                            let v97 = v96 + v56;
                            v116 = v0;
                            v128 = v96;
                            v141 = v0;
                            v155 = v0;
                            v189 = v97;
                            v272 = v58;
                        } else {
                            let v98 = v54 * v24;
                            let v99 = v98 + v56;
                            let v100 = if v99 < v0 { 1.0 } else { 0.0 };
                            if v100 != 0.0 {
                            } else {
                            }
                            let v101 = v51 * v24;
                            let v102 = v101 + v60;
                            let v103 = if v99 > v0 { 1.0 } else { 0.0 };
                            let v156: f64;
                            let v273: f64;
                            if v103 != 0.0 {
                                let v104 = if v102 < v0 { 1.0 } else { 0.0 };
                                if v104 != 0.0 {
                                } else {
                                }
                                let v105 = if v102 > v0 { 1.0 } else { 0.0 };
                                let v157: f64;
                                let v274: f64;
                                if v105 != 0.0 {
                                    let v107 = v65 * (v102 / v99);
                                    let v108 = v6 / v107;
                                    v157 = v107;
                                    v274 = v108;
                                } else {
                                    v157 = v0;
                                    v274 = v58;
                                }
                                v156 = v157;
                                v273 = v274;
                            } else {
                                v156 = v58;
                                v273 = v0;
                            }
                            v116 = v101;
                            v128 = v98;
                            v141 = v102;
                            v155 = v156;
                            v189 = v99;
                            v272 = v273;
                        }
                        v115 = v116;
                        v127 = v128;
                        v140 = v141;
                        v154 = v155;
                        v188 = v189;
                        v271 = v272;
                    }
                    v111 = v115;
                    v124 = v127;
                    v136 = v140;
                    v150 = v154;
                    v185 = v188;
                    v267 = v271;
                }
                v109 = v111;
                v121 = v124;
                v134 = v136;
                v147 = v150;
                v182 = v185;
                v264 = v267;
            }
            let v118 = if v109 < v117 { 1.0 } else { 0.0 };
            if v118 != 0.0 {
            } else {
            }
            let v120 = if v109 > v119 { 1.0 } else { 0.0 };
            if v120 != 0.0 {
            } else {
            }
            let v130 = if v121 < v129 { 1.0 } else { 0.0 };
            if v130 != 0.0 {
            } else {
            }
            let v132 = if v121 > v131 { 1.0 } else { 0.0 };
            if v132 != 0.0 {
            } else {
            }
            let v145: f64;
            if v133 != 0.0 {
                let v143 = v134 + v142;
                v145 = v143;
            } else {
                let v144 = v109 + v142;
                v145 = v144;
            }
            let v158 = if v147 > v0 { 1.0 } else { 0.0 };
            let v164 = if (if v160 > v0 { 1.0 } else { 0.0 }) != 0.0 || (if v162 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let v165 = if (if (if v145 <= v0 { 1.0 } else { 0.0 }) != 0.0 && v158 != 0.0 { 1.0 } else { 0.0 }) != 0.0 && v164 != 0.0 { 1.0 } else { 0.0 };
            if v165 != 0.0 {
            } else {
            }
            let v168 = if v134 > v0 { 1.0 } else { 0.0 };
            let v191: f64;
            let v197: f64;
            if v168 != 0.0 {
                let v192: f64;
                let v198: f64;
                if v38 != 0.0 {
                    let v171 = v166 + (v169 / v134);
                    let v174 = v167 + (v172 / v134);
                    v192 = v171;
                    v198 = v174;
                } else {
                    let v175 = if v36 != 0.0 || v37 != 0.0 { 1.0 } else { 0.0 };
                    let v193: f64;
                    let v199: f64;
                    if v175 != 0.0 {
                        let v178 = v166 + ((v41 * v169) / v134);
                        let v181 = v167 + ((v41 * v172) / v134);
                        v193 = v178;
                        v199 = v181;
                    } else {
                        v193 = v166;
                        v199 = v167;
                    }
                    v192 = v193;
                    v198 = v199;
                }
                v191 = v192;
                v197 = v198;
            } else {
                v191 = v166;
                v197 = v167;
            }
            let v190 = if v182 > v0 { 1.0 } else { 0.0 };
            let v246: f64;
            let v247: f64;
            if v190 != 0.0 {
                let v196 = v191 + (v194 / v182);
                let v202 = v197 + (v200 / v182);
                v246 = v196;
                v247 = v202;
            } else {
                v246 = v191;
                v247 = v197;
            }
            let v206 = if v147 > (v203 / v204) { 1.0 } else { 0.0 };
            if v206 != 0.0 {
            } else {
            }
            let v217: f64;
            if v38 != 0.0 {
                let v209 = v207 * (v109 + v121);
                v217 = v209;
            } else {
                let v210 = if v36 != 0.0 || v37 != 0.0 { 1.0 } else { 0.0 };
                let v218: f64;
                if v210 != 0.0 {
                    let v212 = (v207 * v109) + v121;
                    v218 = v212;
                } else {
                    let v213 = v207 * v109;
                    v218 = v213;
                }
                v217 = v218;
            }
            let v223 = (v215 + (v216 * v217)) + (v221 * (v109 * v121));
            if v6 != 0.0 {
            } else {
            }
            let v227 = v31 + (v225 * v224);
            let v230 = if v227 < (v228 + v6) { 1.0 } else { 0.0 };
            let v242: f64;
            if v230 != 0.0 {
                let v234 = v228 + (((v227 - v228) - v6).exp());
                v242 = v234;
            } else {
                let v237 = if v227 > (v235 - v6) { 1.0 } else { 0.0 };
                let v243: f64;
                if v237 != 0.0 {
                    let v241 = v235 - (((v235 - v227) - v6).exp());
                    v243 = v241;
                } else {
                    v243 = v227;
                }
                v242 = v243;
            }
            let v244 = v242 + v25;
            let v245 = v244 - v27;
            let v251 = v6 + (v245 * (v246 + (v245 * v247)));
            let v254 = if v251 < v253 { 1.0 } else { 0.0 };
            let v262: f64;
            if v254 != 0.0 {
                let v261 = v9 + (v252 * (((v255 * (v251 - v9)) - v6).exp()));
                v262 = v261;
            } else {
                v262 = v251;
            }
            let v263 = v147 * v262;
            let v275 = v264 / v262;
            let v280 = (v6 + (v245 * v276)) * v279;
            let v281 = if v280 < v0 { 1.0 } else { 0.0 };
            let v324: f64;
            if v281 != 0.0 {
                v324 = v0;
            } else {
                v324 = v280;
            }
            let v284 = v282 - v283;
            let v285 = if v158 != 0.0 && v164 != 0.0 { 1.0 } else { 0.0 };
            let v306: f64;
            if v285 != 0.0 {
                let v286 = v284 / v145;
                let v288 = v287 * v286;
                let v294 = v292 * (v286.abs());
                let v305 = (((v6 - v160) - v162) + (v160 * ((v6 + (v288 * v288)).sqrt()))) + (v162 * ((v6 + ((v294 * v294) * v294)).powf(v298)));
                v306 = v305;
            } else {
                v306 = v6;
            }
            let v308 = v284 / (v263 * v306);
            if v190 != 0.0 {
                let v312 = if ((v308 / v182).abs()) > v311 { 1.0 } else { 0.0 };
                if v312 != 0.0 {
                } else {
                }
            } else {
            }
            if v225 != 0.0 {
            } else {
            }
            if v225 != 0.0 {
            } else {
            }
            let v315 = if v264 > v0 { 1.0 } else { 0.0 };
            let v316 = if (if v313 != 0.0 && v158 != 0.0 { 1.0 } else { 0.0 }) != 0.0 && v315 != 0.0 { 1.0 } else { 0.0 };
            let v345: f64;
            let v346: f64;
            if v316 != 0.0 {
                let v320 = ((v317 * v244) * v275) / v306;
                let v323 = if (if v321 != 0.0 && v168 != 0.0 { 1.0 } else { 0.0 }) != 0.0 && v190 != 0.0 { 1.0 } else { 0.0 };
                let v342: f64;
                if v323 != 0.0 {
                    let v331 = ((v324 * (((v308 / v182).abs()).powf(v327))) * v182) / v134;
                    v342 = v331;
                } else {
                    let v334 = if (if v109 > v0 { 1.0 } else { 0.0 }) != 0.0 && (if v121 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let v343: f64;
                    if v334 != 0.0 {
                        let v340 = ((v324 * (((v308 / v121).abs()).powf(v327))) * v121) / v109;
                        v343 = v340;
                    } else {
                        v343 = v0;
                    }
                    v342 = v343;
                }
                let v341 = if v308 < v0 { 1.0 } else { 0.0 };
                let v347: f64;
                if v341 != 0.0 {
                    let v344 = -v342;
                    v347 = v344;
                } else {
                    v347 = v342;
                }
                v345 = v320;
                v346 = v347;
            } else {
                v345 = v0;
                v346 = v0;
            }
            let v349 = if v158 != 0.0 && v315 != 0.0 { 1.0 } else { 0.0 };
            if v349 != 0.0 {
                let v350 = 0e0f64;
                let v354 = (v284 * (0e0f64)) / (v223 * v306);
                let v356 = v6 - (v284 * v354);
                let v357 = if v356 != v0 { 1.0 } else { 0.0 };
                let v361: f64;
                if v357 != 0.0 {
                    let v360 = (v350 + (v308 * v354)) / v356;
                    v361 = v360;
                } else {
                    v361 = v58;
                }
                let v362 = if v361 != v0 { 1.0 } else { 0.0 };
                if v362 != 0.0 {
                } else {
                }
            } else {
            }
        {
            let psd = v345;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 0, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 0, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 0, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(0, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = v346;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 1, value: psd }); }
            let exponent: Option<f64> = Some(v348);
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(1, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        Ok(())
    }
}
