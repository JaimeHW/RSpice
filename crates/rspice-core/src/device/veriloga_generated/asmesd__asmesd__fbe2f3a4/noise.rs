#![allow(dead_code, non_snake_case, unused_parens, unused_variables)]

use super::state::Instance;
use crate::device::veriloga_generated::GeneratedEvalContext;
pub use crate::device::veriloga_generated::{GeneratedNoiseDescriptor, GeneratedNoiseEndpoint, GeneratedNoiseEvaluation, GeneratedNoiseEvaluationError, GeneratedNoiseEvaluationRef, GeneratedNoiseKind, GeneratedNoiseVisitor};

pub static NOISE_SOURCES: [GeneratedNoiseDescriptor; 6] = [
    GeneratedNoiseDescriptor { mechanism: "WHITE_B_BI_RB", label: Some("Rb"), kind: GeneratedNoiseKind::White, equation: 24, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(1), name: "b", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(5), name: "bi", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_E_EI_RE", label: Some("Re"), kind: GeneratedNoiseKind::White, equation: 27, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(2), name: "e", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(6), name: "ei", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C_CI_RC", label: Some("Rc"), kind: GeneratedNoiseKind::White, equation: 30, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(0), name: "c", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(4), name: "ci", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_BI_EI_FLICKER_IBE", label: Some("flicker_Ibe"), kind: GeneratedNoiseKind::Flicker, equation: 44, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "bi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(6), name: "ei", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_BI_EI_IBE", label: Some("Ibe"), kind: GeneratedNoiseKind::White, equation: 45, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "bi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(6), name: "ei", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_CI_EI_IT", label: Some("It"), kind: GeneratedNoiseKind::White, equation: 46, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(4), name: "ci", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(6), name: "ei", is_internal: true }, table_len: 0, table_log_interp: false },
];

impl Instance {
    pub fn evaluate_noise_sources(&self, ctx: &GeneratedEvalContext<'_>, visitor: &mut dyn GeneratedNoiseVisitor) -> Result<(), GeneratedNoiseEvaluationError> {
        if !self.multiplicity.is_finite() || self.multiplicity <= 0.0 {
            return Err(GeneratedNoiseEvaluationError::InvalidMultiplicity { value: self.multiplicity });
        }
        let parameters = &self.params.values;
        let temperature = ctx.temperature();
        let node_potentials = [ctx.node_voltage(self.nodes[0]), ctx.node_voltage(self.nodes[1]), ctx.node_voltage(self.nodes[2]), ctx.node_voltage(self.nodes[3]), ctx.node_voltage(self.nodes[4]), ctx.node_voltage(self.nodes[5]), ctx.node_voltage(self.nodes[6]), ctx.node_voltage(self.nodes[7]), ctx.node_voltage(self.nodes[8]), ctx.node_voltage(self.nodes[9])];
            let v0 = 0e0f64;
            let v1 = temperature;
            let v2 = node_potentials[3];
            let v4 = parameters[45];
            let v6 = 2.7315e2f64;
            let v7 = 1.3e3f64;
            let v8 = 1.7314999999999998e2f64;
            let v10 = 1.7314999999999998e2f64;
            let v13 = 1.3e3f64;
            let v14 = 1.7314999999999998e2f64;
            let v16 = 1.7314999999999998e2f64;
            let v19 = 1e0f64;
            let v20 = 0.0f64;
            let v21 = parameters[26];
            let v23 = parameters[43];
            let v24 = parameters[42];
            let v26 = parameters[29];
            let v27 = node_potentials[5];
            let v28 = node_potentials[4];
            let v31 = parameters[79];
            let v34 = parameters[80];
            let v38 = parameters[25];
            let v40 = 8.6170869e-5f64;
            let v44 = parameters[77];
            let v47 = parameters[52];
            let v50 = parameters[53];
            let v54 = parameters[62];
            let v58 = parameters[54];
            let v62 = parameters[63];
            let v66 = parameters[22];
            let v68 = parameters[21];
            let v73 = parameters[23];
            let v75 = parameters[0];
            let v78 = parameters[2];
            let v81 = parameters[58];
            let v82 = parameters[59];
            let v87 = parameters[64];
            let v88 = parameters[65];
            let v93 = parameters[47];
            let v94 = parameters[7];
            let v98 = parameters[5];
            let v99 = parameters[6];
            let v103 = parameters[9];
            let v104 = parameters[10];
            let v108 = parameters[56];
            let v109 = parameters[55];
            let v113 = 3.0015e2f64;
            let v116 = 1.16e0f64;
            let v117 = 7.02e-4f64;
            let v120 = 1.108e3f64;
            let v125 = 1.3806226e-23f64;
            let v129 = 1.3454442398941469e20f64;
            let v133 = 1.5e0f64;
            let v136 = 1.6021918e-19f64;
            let v140 = parameters[17];
            let v145 = 1.3454442398941469e20f64;
            let v150 = parameters[70];
            let v155 = node_potentials[2];
            let v158 = node_potentials[6];
            let v161 = node_potentials[1];
            let v169 = parameters[1];
            let v174 = parameters[11];
            let v179 = 8e1f64;
            let v187 = 3.7e1f64;
            let v189 = -3.7e1f64;
            let v196 = -3.7e1f64;
            let v210 = parameters[8];
            let v218 = parameters[4];
            let v220 = 1e-3f64;
            let v222 = -1e0f64;
            let v225 = parameters[3];
            let v243 = parameters[57];
            let v256 = -3.7e1f64;
            let v263 = -3.7e1f64;
            let v283 = parameters[61];
            let v300 = -3.7e1f64;
            let v307 = -3.7e1f64;
            let v338 = -3.7e1f64;
            let v341 = -3.7e1f64;
            let v350 = parameters[81];
            let v361 = 4e0f64;
            let v365 = parameters[82];
            let v368 = 2e0f64;
            let v373 = parameters[48];
            let v376 = parameters[49];
            let v379 = parameters[50];
            let v382 = parameters[51];
            let v385 = parameters[12];
            let v386 = parameters[37];
            let v393 = parameters[66];
            let v394 = parameters[78];
            let v398 = parameters[14];
            let v399 = parameters[38];
            let v406 = parameters[19];
            let v407 = parameters[32];
            let v409 = node_potentials[8];
            let v411 = parameters[20];
            let v413 = parameters[44];
            let v417 = parameters[31];
            let v420 = parameters[13];
            let v422 = parameters[67];
            let v424 = parameters[15];
            let v428 = parameters[24];
            let v438 = parameters[68];
            let v442 = parameters[30];
            let v444 = parameters[33];
            let v449 = parameters[35];
            let v452 = -1e0f64;
            let v454 = 5.5224904e-23f64;
            let v466 = parameters[46];
            let v494 = parameters[28];
            let v496 = parameters[27];
            let v505 = -1e0f64;
            let v508 = 3.2043836e-19f64;
            let v5 = (v1 + v2) + v4;
            let v9 = if v5 > v8 { 1.0 } else { 0.0 };
            let v11: f64;
            if v9 != 0.0 {
                v11 = v5;
            } else {
                v11 = v10;
            }
            let v12 = if v7 < v11 { 1.0 } else { 0.0 };
            let v18: f64;
            if v12 != 0.0 {
                v18 = v13;
            } else {
                let v15 = if v5 > v14 { 1.0 } else { 0.0 };
                let v17: f64;
                if v15 != 0.0 {
                    v17 = v5;
                } else {
                    v17 = v16;
                }
                v18 = v17;
            }
            if v20 != 0.0 {
            } else {
            }
            let v22 = if v18 > v21 { 1.0 } else { 0.0 };
            if v22 != 0.0 {
            } else {
            }
            let v25 = v23 * v24;
            let v30 = v26 * (v27 - v28);
            let v39 = v38 + v6;
            let v41 = v40 * v18;
            let v42 = v18 / v39;
            let v43 = v42.ln();
            let v46 = (v44 * v43).exp();
            let v49 = (v47 * v46) * (v19 + (v31 * ((-(if v30 <= v0 { v30 } else { v0 })).powf(v34))));
            let v51 = if v50 > v0 { 1.0 } else { 0.0 };
            let v53: f64;
            if v51 != 0.0 {
                let v52 = v19 / v50;
                v53 = v52;
            } else {
                v53 = v0;
            }
            let v55 = if v54 > v0 { 1.0 } else { 0.0 };
            let v57: f64;
            if v55 != 0.0 {
                let v56 = v19 / v54;
                v57 = v56;
            } else {
                v57 = v0;
            }
            let v59 = if v58 > v0 { 1.0 } else { 0.0 };
            let v61: f64;
            if v59 != 0.0 {
                let v60 = v19 / v58;
                v61 = v60;
            } else {
                v61 = v0;
            }
            let v63 = if v62 > v0 { 1.0 } else { 0.0 };
            let v65: f64;
            if v63 != 0.0 {
                let v64 = v19 / v62;
                v65 = v64;
            } else {
                v65 = v0;
            }
            let v69 = v42 - v19;
            let v72 = (v66 * v43) + ((v68 * v69) / v41);
            let v77 = v75 * (v72.exp());
            let v80 = v78 * ((v73 * v43).exp());
            let v86 = (v81 * ((v72 / v82).exp())) / v46;
            let v92 = (v87 * ((v72 / v88).exp())) / v46;
            let v97 = v93 * (v19 + (v94 * v69));
            let v102 = v98 * (v19 + (v99 * v69));
            let v107 = v103 * (v19 + (v104 * v69));
            let v112 = v108 * (v19 + (v109 * v69));
            let v114 = v39 / v113;
            let v115 = v18 / v113;
            let v128 = (-(v116 - (((v117 * v18) * v18) / (v120 + v18)))) / (v125 * (v18 + v18));
            let v132 = -(v41 + v41);
            let v135 = v133 * (v115.ln());
            let v139 = v132 * (v135 + (v136 * (v128 + v129)));
            let v144 = (v115 * ((v140 - v139) / v114)) + v139;
            let v149 = v132 * (v135 + (v136 * (v128 + v145)));
            let v154 = (v115 * ((v150 - v149) / v114)) + v149;
            let v157 = v26 * (v155 - v28);
            let v160 = v26 * (v27 - v158);
            let v163 = v26 * (v161 - v28);
            let v165 = v26 * (v161 - v27);
            let v167 = v26 * (v155 - v158);
            let v168 = if v77 > v0 { 1.0 } else { 0.0 };
            let v343: f64;
            if v168 != 0.0 {
                let v171 = v160 / (v169 * v41);
                let v175 = v174 * v41;
                let v176 = ((-v160) - v102) / v175;
                let v178 = (-v102) / v175;
                let v180 = if v171 > v179 { 1.0 } else { 0.0 };
                let v183: f64;
                let v184: f64;
                if v180 != 0.0 {
                    let v182 = v19 + (v171 - v179);
                    v183 = v182;
                    v184 = v179;
                } else {
                    v183 = v19;
                    v184 = v171;
                }
                let v186 = v183 * (v184.exp());
                let v188 = if v176 >= v187 { 1.0 } else { 0.0 };
                let v202: f64;
                if v188 != 0.0 {
                    v202 = v176;
                } else {
                    let v190 = if v176 <= v189 { 1.0 } else { 0.0 };
                    let v203: f64;
                    if v190 != 0.0 {
                        let v191 = v176.exp();
                        v203 = v191;
                    } else {
                        let v194 = ((v176.exp()) + v19).ln();
                        v203 = v194;
                    }
                    v202 = v203;
                }
                let v195 = if v178 >= v187 { 1.0 } else { 0.0 };
                let v204: f64;
                if v195 != 0.0 {
                    v204 = v178;
                } else {
                    let v197 = if v178 <= v196 { 1.0 } else { 0.0 };
                    let v205: f64;
                    if v197 != 0.0 {
                        let v198 = v178.exp();
                        v205 = v198;
                    } else {
                        let v201 = ((v178.exp()) + v19).ln();
                        v205 = v201;
                    }
                    v204 = v205;
                }
                let v216 = (v77 * (v186 - v19)) - ((v97 * (v202 - v204)) / (v19 + (v210 * ((v160.abs()).powf(v107)))));
                v343 = v216;
            } else {
                v343 = v0;
            }
            let v217 = if v80 > v0 { 1.0 } else { 0.0 };
            let v344: f64;
            if v217 != 0.0 {
                let v228 = ((v222 * v160) * v218) / ((v225 * v41) * (if (v218 - v160) >= v220 { (v218 - v160) } else { v220 }));
                let v229 = if v228 > v179 { 1.0 } else { 0.0 };
                let v232: f64;
                let v233: f64;
                if v229 != 0.0 {
                    let v231 = v19 + (v228 - v179);
                    v232 = v231;
                    v233 = v179;
                } else {
                    v232 = v19;
                    v233 = v228;
                }
                let v237 = v80 * ((v232 * (v233.exp())) - v19);
                v344 = v237;
            } else {
                v344 = v0;
            }
            let v238 = if v86 > v0 { 1.0 } else { 0.0 };
            let v347: f64;
            if v238 != 0.0 {
                let v240 = v160 / (v82 * v41);
                let v244 = v243 * v41;
                let v245 = ((-v160) - v102) / v244;
                let v247 = (-v102) / v244;
                let v248 = if v240 > v179 { 1.0 } else { 0.0 };
                let v251: f64;
                let v252: f64;
                if v248 != 0.0 {
                    let v250 = v19 + (v240 - v179);
                    v251 = v250;
                    v252 = v179;
                } else {
                    v251 = v19;
                    v252 = v240;
                }
                let v254 = v251 * (v252.exp());
                let v255 = if v245 >= v187 { 1.0 } else { 0.0 };
                let v269: f64;
                if v255 != 0.0 {
                    v269 = v245;
                } else {
                    let v257 = if v245 <= v256 { 1.0 } else { 0.0 };
                    let v270: f64;
                    if v257 != 0.0 {
                        let v258 = v245.exp();
                        v270 = v258;
                    } else {
                        let v261 = ((v245.exp()) + v19).ln();
                        v270 = v261;
                    }
                    v269 = v270;
                }
                let v262 = if v247 >= v187 { 1.0 } else { 0.0 };
                let v271: f64;
                if v262 != 0.0 {
                    v271 = v247;
                } else {
                    let v264 = if v247 <= v263 { 1.0 } else { 0.0 };
                    let v272: f64;
                    if v264 != 0.0 {
                        let v265 = v247.exp();
                        v272 = v265;
                    } else {
                        let v268 = ((v247.exp()) + v19).ln();
                        v272 = v268;
                    }
                    v271 = v272;
                }
                let v282 = (v86 * (v254 - v19)) - ((v0 * (v269 - v271)) / (v19 + (v210 * ((v160.abs()).powf(v107)))));
                v347 = v282;
            } else {
                v347 = v0;
            }
            let v349: f64;
            if v168 != 0.0 {
                let v285 = v30 / (v283 * v41);
                let v288 = v243 * v41;
                let v289 = ((-v30) - v102) / v288;
                let v291 = (-v102) / v288;
                let v292 = if v285 > v179 { 1.0 } else { 0.0 };
                let v295: f64;
                let v296: f64;
                if v292 != 0.0 {
                    let v294 = v19 + (v285 - v179);
                    v295 = v294;
                    v296 = v179;
                } else {
                    v295 = v19;
                    v296 = v285;
                }
                let v298 = v295 * (v296.exp());
                let v299 = if v289 >= v187 { 1.0 } else { 0.0 };
                let v313: f64;
                if v299 != 0.0 {
                    v313 = v289;
                } else {
                    let v301 = if v289 <= v300 { 1.0 } else { 0.0 };
                    let v314: f64;
                    if v301 != 0.0 {
                        let v302 = v289.exp();
                        v314 = v302;
                    } else {
                        let v305 = ((v289.exp()) + v19).ln();
                        v314 = v305;
                    }
                    v313 = v314;
                }
                let v306 = if v291 >= v187 { 1.0 } else { 0.0 };
                let v315: f64;
                if v306 != 0.0 {
                    v315 = v291;
                } else {
                    let v308 = if v291 <= v307 { 1.0 } else { 0.0 };
                    let v316: f64;
                    if v308 != 0.0 {
                        let v309 = v291.exp();
                        v316 = v309;
                    } else {
                        let v312 = ((v291.exp()) + v19).ln();
                        v316 = v312;
                    }
                    v315 = v316;
                }
                let v326 = (v77 * (v298 - v19)) - ((v112 * (v313 - v315)) / (v19 + (v210 * ((v30.abs()).powf(v107)))));
                v349 = v326;
            } else {
                v349 = v0;
            }
            let v327 = if v92 > v0 { 1.0 } else { 0.0 };
            if v327 != 0.0 {
                let v332 = v243 * v41;
                let v333 = ((-v30) - v102) / v332;
                let v335 = (-v102) / v332;
                let v336 = if (v30 / (v88 * v41)) > v179 { 1.0 } else { 0.0 };
                if v336 != 0.0 {
                } else {
                }
                let v337 = if v333 >= v187 { 1.0 } else { 0.0 };
                if v337 != 0.0 {
                } else {
                    let v339 = if v333 <= v338 { 1.0 } else { 0.0 };
                    if v339 != 0.0 {
                    } else {
                    }
                }
                let v340 = if v335 >= v187 { 1.0 } else { 0.0 };
                if v340 != 0.0 {
                } else {
                    let v342 = if v335 <= v341 { 1.0 } else { 0.0 };
                    if v342 != 0.0 {
                    } else {
                    }
                }
            } else {
            }
            let v348 = ((v343 - v344) / v49) + v347;
            let v370 = (v368 * ((v19 - (v160 * v57)) - (v30 * v53))) / (v19 + (((v19 + (v361 * ((v343 * (v61 * (v19 + (v30 * v350)))) + (v349 * v65)))).abs()).powf(v365)));
            let v371 = v349 * v370;
            let v372 = v343 * v370;
            let v392 = (v385 * ((v43 * v386).exp())) * ((v19 + (((v165 / v373).abs()).powf(v376))).powf((v19 / v376)));
            let v397 = v393 * ((v43 * v394).exp());
            let v405 = (v398 * ((v43 * v399).exp())) * ((v19 + (((v167 / v379).abs()).powf(v382))).powf((v19 / v382)));
            let v408 = if v407 == v19 { 1.0 } else { 0.0 };
            let v419: f64;
            if v408 != 0.0 {
                let v416 = v392 / (v19 + (((v409.abs()) / v411).powf(v413)));
                v419 = v416;
            } else {
                v419 = v392;
            }
            let v418 = if v417 == v19 { 1.0 } else { 0.0 };
            let v469: f64;
            let v478: f64;
            let v487: f64;
            if v418 != 0.0 {
                let v421 = v419 + v420;
                let v423 = v397 + v422;
                let v425 = v405 + v424;
                v469 = v421;
                v478 = v425;
                v487 = v423;
            } else {
                v469 = v419;
                v478 = v405;
                v487 = v397;
            }
            let v426 = if v157 <= v0 { 1.0 } else { 0.0 };
            if v426 != 0.0 {
            } else {
            }
            let v431 = if (v160 + ((-v144) * v428)) > v0 { 1.0 } else { 0.0 };
            if v431 != 0.0 {
            } else {
            }
            let v433 = (-v154) * v428;
            let v435 = if (v163 + v433) > v0 { 1.0 } else { 0.0 };
            if v435 != 0.0 {
            } else {
            }
            let v437 = if (v30 + v433) > v0 { 1.0 } else { 0.0 };
            if v437 != 0.0 {
            } else {
            }
            let v441 = if (if v438 != v0 { 1.0 } else { 0.0 }) != 0.0 && (if v406 != v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            if v441 != 0.0 {
            } else {
            }
            let v445 = if v444 > v0 { 1.0 } else { 0.0 };
            let v446 = if (if v442 == v19 { 1.0 } else { 0.0 }) != 0.0 && v445 != 0.0 { 1.0 } else { 0.0 };
            if v446 != 0.0 {
            } else {
                let v451 = if (if (if v442 == v368 { 1.0 } else { 0.0 }) != 0.0 && v445 != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if v449 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                if v451 != 0.0 {
                } else {
                    let v453 = if v442 == v452 { 1.0 } else { 0.0 };
                    if v453 != 0.0 {
                    } else {
                    }
                }
            }
            let v455 = v454 * v18;
            let v458 = (v385 + (v417 * v420)) / v25;
            let v461 = (v398 + (v417 * v424)) / v25;
            let v464 = (v393 + (v417 * v422)) / v25;
            let v468 = if (if v458 > v0 { 1.0 } else { 0.0 }) != 0.0 && (if v458 >= v466 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let v514: f64;
            let v515: f64;
            if v468 != 0.0 {
                let v470 = v469 / v25;
                let v471 = if v470 > v466 { 1.0 } else { 0.0 };
                if v471 != 0.0 {
                } else {
                }
                let v472 = if v470 >= v466 { 1.0 } else { 0.0 };
                let v474: f64;
                if v472 != 0.0 {
                    let v473 = v455 / v470;
                    v474 = v473;
                } else {
                    v474 = v0;
                }
                v514 = v19;
                v515 = v474;
            } else {
                v514 = v0;
                v515 = v0;
            }
            let v477 = if (if v461 > v0 { 1.0 } else { 0.0 }) != 0.0 && (if v461 >= v466 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let v516: f64;
            let v517: f64;
            if v477 != 0.0 {
                let v479 = v478 / v25;
                let v480 = if v479 > v466 { 1.0 } else { 0.0 };
                if v480 != 0.0 {
                } else {
                }
                let v481 = if v479 >= v466 { 1.0 } else { 0.0 };
                let v483: f64;
                if v481 != 0.0 {
                    let v482 = v455 / v479;
                    v483 = v482;
                } else {
                    v483 = v0;
                }
                v516 = v19;
                v517 = v483;
            } else {
                v516 = v0;
                v517 = v0;
            }
            let v486 = if (if v464 > v0 { 1.0 } else { 0.0 }) != 0.0 && (if v464 >= v466 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let v518: f64;
            let v519: f64;
            if v486 != 0.0 {
                let v488 = v487 / v25;
                let v489 = if v488 > v466 { 1.0 } else { 0.0 };
                if v489 != 0.0 {
                } else {
                }
                let v490 = if v488 >= v466 { 1.0 } else { 0.0 };
                let v492: f64;
                if v490 != 0.0 {
                    let v491 = v455 / v488;
                    v492 = v491;
                } else {
                    v492 = v0;
                }
                v518 = v19;
                v519 = v492;
            } else {
                v518 = v0;
                v519 = v0;
            }
            let v493 = v26 * v348;
            let v499 = if (if (if v494 > v0 { 1.0 } else { 0.0 }) != 0.0 && (if v496 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) > v0 { 1.0 } else { 0.0 };
            let v503: f64;
            if v499 != 0.0 {
                let v502 = v496 * ((v348.abs()).powf(v494));
                v503 = v502;
            } else {
                v503 = v0;
            }
            let v504 = if v493 >= v0 { 1.0 } else { 0.0 };
            let v506: f64;
            if v504 != 0.0 {
                v506 = v19;
            } else {
                v506 = v505;
            }
            let v507 = v506 * v503;
            let v510 = v508 * (v348.abs());
            let v513 = v508 * ((v372 - v371).abs());
        if v514 == 0.0 {
            if !visitor.visit(0, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v515;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 0, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 0, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 0, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(0, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v516 == 0.0 {
            if !visitor.visit(1, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v517;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 1, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(1, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v518 == 0.0 {
            if !visitor.visit(2, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v519;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 2, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 2, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 2, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(2, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = v507;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 3, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 3, value: psd }); }
            let exponent: Option<f64> = Some(v19);
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 3, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 3, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(3, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = v510;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 4, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 4, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 4, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(4, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = v513;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 5, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 5, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 5, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(5, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        Ok(())
    }
}
