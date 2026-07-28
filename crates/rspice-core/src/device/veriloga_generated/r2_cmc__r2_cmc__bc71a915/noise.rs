#![allow(dead_code, non_snake_case, unused_parens, unused_variables)]

use super::state::Instance;
use crate::device::veriloga_generated::GeneratedEvalContext;
pub use crate::device::veriloga_generated::{GeneratedNoiseDescriptor, GeneratedNoiseEndpoint, GeneratedNoiseEvaluation, GeneratedNoiseEvaluationError, GeneratedNoiseEvaluationRef, GeneratedNoiseKind, GeneratedNoiseVisitor};

pub static NOISE_SOURCES: [GeneratedNoiseDescriptor; 2] = [
    GeneratedNoiseDescriptor { mechanism: "WHITE_N1_N2_THERMAL", label: Some("thermal"), kind: GeneratedNoiseKind::White, equation: 1, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(0), name: "n1", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(1), name: "n2", is_internal: false }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_N1_N2_FLICKER", label: Some("flicker"), kind: GeneratedNoiseKind::Flicker, equation: 2, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(0), name: "n1", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(1), name: "n2", is_internal: false }, table_len: 0, table_log_interp: false },
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
        let node_potentials = [ctx.node_voltage(self.nodes[0]), ctx.node_voltage(self.nodes[1])];
            let v0 = 0e0f64;
            let v1 = parameters[14];
            let v2 = 1.002e3f64;
            let v4 = if parameter_given[9] { 1.0 } else { 0.0 };
            let v5 = parameters[9];
            let v6 = 1e0f64;
            let v8 = if parameter_given[10] { 1.0 } else { 0.0 };
            let v9 = 1e-2f64;
            let v10 = parameters[10];
            let v16 = if parameter_given[13] { 1.0 } else { 0.0 };
            let v17 = parameters[13];
            let v18 = 1e-3f64;
            let v23 = 1e6f64;
            let v25 = 2.7315e2f64;
            let v26 = parameters[15];
            let v28 = temperature;
            let v29 = parameters[5];
            let v32 = parameters[11];
            let v34 = parameters[12];
            let v36 = parameters[34];
            let v43 = parameters[35];
            let v54 = parameters[42];
            let v57 = parameters[29];
            let v60 = parameters[3];
            let v61 = parameters[4];
            let v63 = parameters[22];
            let v65 = 5e-1f64;
            let v67 = if parameter_given[1] { 1.0 } else { 0.0 };
            let v68 = if parameter_given[2] { 1.0 } else { 0.0 };
            let v70 = if parameter_given[0] { 1.0 } else { 0.0 };
            let v73 = parameters[2];
            let v75 = parameters[1];
            let v78 = parameters[0];
            let v80 = parameters[21];
            let v82 = 1e99f64;
            let v89 = parameters[16];
            let v141 = parameters[17];
            let v143 = parameters[18];
            let v153 = parameters[19];
            let v155 = parameters[20];
            let v157 = parameters[24];
            let v166 = parameters[23];
            let v184 = parameters[28];
            let v186 = parameters[26];
            let v190 = parameters[36];
            let v191 = parameters[37];
            let v193 = parameters[38];
            let v196 = parameters[39];
            let v218 = parameters[40];
            let v224 = parameters[41];
            let v228 = multiplicity;
            let v237 = 1e-1f64;
            let v238 = 1.1e-1f64;
            let v240 = 1e1f64;
            let v261 = node_potentials[0];
            let v262 = node_potentials[1];
            let v266 = parameters[27];
            let v271 = parameters[25];
            let v277 = 3.333333333333333e-1f64;
            let v290 = parameters[33];
            let v292 = parameters[6];
            let v296 = 5.522602e-23f64;
            let v300 = parameters[32];
            let v306 = parameters[30];
            let v327 = parameters[31];
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
            let v227: f64;
            if v16 != 0.0 {
                v227 = v17;
            } else {
                let v19 = ctx.simparam_or("rthresh", v18);
                v227 = v19;
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
            let v38 = if v31 < (v36 + v6) { 1.0 } else { 0.0 };
            let v50: f64;
            if v38 != 0.0 {
                let v42 = v36 + (((v31 - v36) - v6).exp());
                v50 = v42;
            } else {
                let v45 = if v31 > (v43 - v6) { 1.0 } else { 0.0 };
                let v51: f64;
                if v45 != 0.0 {
                    let v49 = v43 - (((v43 - v31) - v6).exp());
                    v51 = v49;
                } else {
                    v51 = v31;
                }
                v50 = v51;
            }
            let v52 = v50 + v25;
            let v53 = v52 - v27;
            let v58 = (v6 + (v53 * v54)) * v57;
            let v59 = if v58 < v0 { 1.0 } else { 0.0 };
            let v303: f64;
            if v59 != 0.0 {
                v303 = v0;
            } else {
                v303 = v58;
            }
            let v62 = if v60 != 0.0 && v61 != 0.0 { 1.0 } else { 0.0 };
            let v84: f64;
            if v62 != 0.0 {
                v84 = v63;
            } else {
                let v64 = if v60 != 0.0 || v61 != 0.0 { 1.0 } else { 0.0 };
                let v85: f64;
                if v64 != 0.0 {
                    let v66 = v63 * v65;
                    v85 = v66;
                } else {
                    v85 = v0;
                }
                v84 = v85;
            }
            let v72 = if (if v67 != 0.0 && v68 != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if v70 == 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let v133: f64;
            let v145: f64;
            let v158: f64;
            let v171: f64;
            let v206: f64;
            let v249: f64;
            if v72 != 0.0 {
                let v77 = if (if v73 == v0 { 1.0 } else { 0.0 }) != 0.0 || (if v75 == v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v134: f64;
                let v146: f64;
                let v159: f64;
                let v172: f64;
                let v207: f64;
                let v250: f64;
                if v77 != 0.0 {
                    let v79 = v78 * v24;
                    let v81 = v79 + v80;
                    v134 = v0;
                    v146 = v79;
                    v159 = v0;
                    v172 = v0;
                    v207 = v81;
                    v250 = v82;
                } else {
                    let v83 = v75 * v24;
                    let v86 = v83 + v84;
                    let v87 = if v86 < v0 { 1.0 } else { 0.0 };
                    if v87 != 0.0 {
                    } else {
                    }
                    let v88 = if v86 > v0 { 1.0 } else { 0.0 };
                    let v147: f64;
                    let v173: f64;
                    let v208: f64;
                    let v251: f64;
                    if v88 != 0.0 {
                        let v91 = (v89 / v73) * v86;
                        let v92 = v91 - v80;
                        let v93 = if v92 <= v0 { 1.0 } else { 0.0 };
                        if v93 != 0.0 {
                        } else {
                        }
                        let v94 = v6 / v73;
                        v147 = v92;
                        v173 = v73;
                        v208 = v91;
                        v251 = v94;
                    } else {
                        let v95 = v78 * v24;
                        let v96 = v95 + v80;
                        v147 = v95;
                        v173 = v0;
                        v208 = v96;
                        v251 = v82;
                    }
                    v134 = v83;
                    v146 = v147;
                    v159 = v86;
                    v172 = v173;
                    v207 = v208;
                    v250 = v251;
                }
                v133 = v134;
                v145 = v146;
                v158 = v159;
                v171 = v172;
                v206 = v207;
                v249 = v250;
            } else {
                let v98 = if v68 != 0.0 && (if v67 == 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v135: f64;
                let v148: f64;
                let v160: f64;
                let v174: f64;
                let v209: f64;
                let v252: f64;
                if v98 != 0.0 {
                    let v99 = if v73 == v0 { 1.0 } else { 0.0 };
                    let v136: f64;
                    let v149: f64;
                    let v161: f64;
                    let v175: f64;
                    let v210: f64;
                    let v253: f64;
                    if v99 != 0.0 {
                        let v100 = v78 * v24;
                        let v101 = v100 + v80;
                        v136 = v0;
                        v149 = v100;
                        v161 = v0;
                        v175 = v0;
                        v210 = v101;
                        v253 = v82;
                    } else {
                        let v102 = if v78 == v0 { 1.0 } else { 0.0 };
                        let v137: f64;
                        let v150: f64;
                        let v162: f64;
                        let v176: f64;
                        let v211: f64;
                        let v254: f64;
                        if v102 != 0.0 {
                            let v103 = v75 * v24;
                            let v104 = v103 + v84;
                            v137 = v103;
                            v150 = v0;
                            v162 = v104;
                            v176 = v82;
                            v211 = v0;
                            v254 = v0;
                        } else {
                            let v105 = v78 * v24;
                            let v106 = v105 + v80;
                            let v107 = if v106 < v0 { 1.0 } else { 0.0 };
                            if v107 != 0.0 {
                            } else {
                            }
                            let v108 = if v106 > v0 { 1.0 } else { 0.0 };
                            let v138: f64;
                            let v163: f64;
                            let v177: f64;
                            let v255: f64;
                            if v108 != 0.0 {
                                let v110 = (v73 / v89) * v106;
                                let v111 = v110 - v84;
                                let v112 = if v111 <= v0 { 1.0 } else { 0.0 };
                                if v112 != 0.0 {
                                } else {
                                }
                                let v113 = v6 / v73;
                                v138 = v111;
                                v163 = v110;
                                v177 = v73;
                                v255 = v113;
                            } else {
                                let v114 = v75 * v24;
                                let v115 = v114 + v84;
                                v138 = v114;
                                v163 = v115;
                                v177 = v82;
                                v255 = v0;
                            }
                            v137 = v138;
                            v150 = v105;
                            v162 = v163;
                            v176 = v177;
                            v211 = v106;
                            v254 = v255;
                        }
                        v136 = v137;
                        v149 = v150;
                        v161 = v162;
                        v175 = v176;
                        v210 = v211;
                        v253 = v254;
                    }
                    v135 = v136;
                    v148 = v149;
                    v160 = v161;
                    v174 = v175;
                    v209 = v210;
                    v252 = v253;
                } else {
                    let v116 = if v78 == v0 { 1.0 } else { 0.0 };
                    let v139: f64;
                    let v151: f64;
                    let v164: f64;
                    let v178: f64;
                    let v212: f64;
                    let v256: f64;
                    if v116 != 0.0 {
                        let v117 = v75 * v24;
                        let v118 = v117 + v84;
                        v139 = v117;
                        v151 = v0;
                        v164 = v118;
                        v178 = v82;
                        v212 = v0;
                        v256 = v0;
                    } else {
                        let v119 = if v75 == v0 { 1.0 } else { 0.0 };
                        let v140: f64;
                        let v152: f64;
                        let v165: f64;
                        let v179: f64;
                        let v213: f64;
                        let v257: f64;
                        if v119 != 0.0 {
                            let v120 = v78 * v24;
                            let v121 = v120 + v80;
                            v140 = v0;
                            v152 = v120;
                            v165 = v0;
                            v179 = v0;
                            v213 = v121;
                            v257 = v82;
                        } else {
                            let v122 = v78 * v24;
                            let v123 = v122 + v80;
                            let v124 = if v123 < v0 { 1.0 } else { 0.0 };
                            if v124 != 0.0 {
                            } else {
                            }
                            let v125 = v75 * v24;
                            let v126 = v125 + v84;
                            let v127 = if v123 > v0 { 1.0 } else { 0.0 };
                            let v180: f64;
                            let v258: f64;
                            if v127 != 0.0 {
                                let v128 = if v126 < v0 { 1.0 } else { 0.0 };
                                if v128 != 0.0 {
                                } else {
                                }
                                let v129 = if v126 > v0 { 1.0 } else { 0.0 };
                                let v181: f64;
                                let v259: f64;
                                if v129 != 0.0 {
                                    let v131 = v89 * (v126 / v123);
                                    let v132 = v6 / v131;
                                    v181 = v131;
                                    v259 = v132;
                                } else {
                                    v181 = v0;
                                    v259 = v82;
                                }
                                v180 = v181;
                                v258 = v259;
                            } else {
                                v180 = v82;
                                v258 = v0;
                            }
                            v140 = v125;
                            v152 = v122;
                            v165 = v126;
                            v179 = v180;
                            v213 = v123;
                            v257 = v258;
                        }
                        v139 = v140;
                        v151 = v152;
                        v164 = v165;
                        v178 = v179;
                        v212 = v213;
                        v256 = v257;
                    }
                    v135 = v139;
                    v148 = v151;
                    v160 = v164;
                    v174 = v178;
                    v209 = v212;
                    v252 = v256;
                }
                v133 = v135;
                v145 = v148;
                v158 = v160;
                v171 = v174;
                v206 = v209;
                v249 = v252;
            }
            let v142 = if v133 < v141 { 1.0 } else { 0.0 };
            if v142 != 0.0 {
            } else {
            }
            let v144 = if v133 > v143 { 1.0 } else { 0.0 };
            if v144 != 0.0 {
            } else {
            }
            let v154 = if v145 < v153 { 1.0 } else { 0.0 };
            if v154 != 0.0 {
            } else {
            }
            let v156 = if v145 > v155 { 1.0 } else { 0.0 };
            if v156 != 0.0 {
            } else {
            }
            let v169: f64;
            if v157 != 0.0 {
                let v167 = v158 + v166;
                v169 = v167;
            } else {
                let v168 = v133 + v166;
                v169 = v168;
            }
            let v182 = if v171 > v0 { 1.0 } else { 0.0 };
            let v188 = if (if v184 > v0 { 1.0 } else { 0.0 }) != 0.0 || (if v186 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let v189 = if (if (if v169 <= v0 { 1.0 } else { 0.0 }) != 0.0 && v182 != 0.0 { 1.0 } else { 0.0 }) != 0.0 && v188 != 0.0 { 1.0 } else { 0.0 };
            if v189 != 0.0 {
            } else {
            }
            let v192 = if v158 > v0 { 1.0 } else { 0.0 };
            let v215: f64;
            let v221: f64;
            if v192 != 0.0 {
                let v216: f64;
                let v222: f64;
                if v62 != 0.0 {
                    let v195 = v190 + (v193 / v158);
                    let v198 = v191 + (v196 / v158);
                    v216 = v195;
                    v222 = v198;
                } else {
                    let v199 = if v60 != 0.0 || v61 != 0.0 { 1.0 } else { 0.0 };
                    let v217: f64;
                    let v223: f64;
                    if v199 != 0.0 {
                        let v202 = v190 + ((v65 * v193) / v158);
                        let v205 = v191 + ((v65 * v196) / v158);
                        v217 = v202;
                        v223 = v205;
                    } else {
                        v217 = v190;
                        v223 = v191;
                    }
                    v216 = v217;
                    v222 = v223;
                }
                v215 = v216;
                v221 = v222;
            } else {
                v215 = v190;
                v221 = v191;
            }
            let v214 = if v206 > v0 { 1.0 } else { 0.0 };
            let v231: f64;
            let v232: f64;
            if v214 != 0.0 {
                let v220 = v215 + (v218 / v206);
                let v226 = v221 + (v224 / v206);
                v231 = v220;
                v232 = v226;
            } else {
                v231 = v215;
                v232 = v221;
            }
            let v230 = if v171 > (v227 / v228) { 1.0 } else { 0.0 };
            if v230 != 0.0 {
            } else {
            }
            let v236 = v6 + (v53 * (v231 + (v53 * v232)));
            let v239 = if v236 < v238 { 1.0 } else { 0.0 };
            let v247: f64;
            if v239 != 0.0 {
                let v246 = v9 + (v237 * (((v240 * (v236 - v9)) - v6).exp()));
                v247 = v246;
            } else {
                v247 = v236;
            }
            let v248 = v171 * v247;
            let v260 = v249 / v247;
            let v263 = v261 - v262;
            let v264 = if v182 != 0.0 && v188 != 0.0 { 1.0 } else { 0.0 };
            let v285: f64;
            if v264 != 0.0 {
                let v265 = v263 / v169;
                let v267 = v266 * v265;
                let v273 = v271 * (v265.abs());
                let v284 = (((v6 - v184) - v186) + (v184 * ((v6 + (v267 * v267)).sqrt()))) + (v186 * ((v6 + ((v273 * v273) * v273)).powf(v277)));
                v285 = v284;
            } else {
                v285 = v6;
            }
            let v287 = v263 / (v248 * v285);
            if v214 != 0.0 {
                let v291 = if ((v287 / v206).abs()) > v290 { 1.0 } else { 0.0 };
                if v291 != 0.0 {
                } else {
                }
            } else {
            }
            let v294 = if v249 > v0 { 1.0 } else { 0.0 };
            let v295 = if (if v292 != 0.0 && v182 != 0.0 { 1.0 } else { 0.0 }) != 0.0 && v294 != 0.0 { 1.0 } else { 0.0 };
            let v324: f64;
            let v325: f64;
            if v295 != 0.0 {
                let v299 = ((v296 * v52) * v260) / v285;
                let v302 = if (if v300 != 0.0 && v192 != 0.0 { 1.0 } else { 0.0 }) != 0.0 && v214 != 0.0 { 1.0 } else { 0.0 };
                let v321: f64;
                if v302 != 0.0 {
                    let v310 = ((v303 * (((v287 / v206).abs()).powf(v306))) * v206) / v158;
                    v321 = v310;
                } else {
                    let v313 = if (if v133 > v0 { 1.0 } else { 0.0 }) != 0.0 && (if v145 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let v322: f64;
                    if v313 != 0.0 {
                        let v319 = ((v303 * (((v287 / v145).abs()).powf(v306))) * v145) / v133;
                        v322 = v319;
                    } else {
                        v322 = v0;
                    }
                    v321 = v322;
                }
                let v320 = if v287 < v0 { 1.0 } else { 0.0 };
                let v326: f64;
                if v320 != 0.0 {
                    let v323 = -v321;
                    v326 = v323;
                } else {
                    v326 = v321;
                }
                v324 = v299;
                v325 = v326;
            } else {
                v324 = v0;
                v325 = v0;
            }
            let v328 = if v182 != 0.0 && v294 != 0.0 { 1.0 } else { 0.0 };
            if v328 != 0.0 {
                let v330 = if (0e0f64) != v0 { 1.0 } else { 0.0 };
                if v330 != 0.0 {
                } else {
                }
            } else {
            }
        {
            let psd = v324;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 0, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 0, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(0, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = v325;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = Some(v327);
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(1, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        Ok(())
    }
}
