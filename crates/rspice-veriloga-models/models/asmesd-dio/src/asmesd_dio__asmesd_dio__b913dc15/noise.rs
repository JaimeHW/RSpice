#![allow(dead_code, non_snake_case, unused_parens, unused_variables)]

use super::state::Instance;
use rspice_veriloga_runtime::GeneratedEvalContext;
pub use rspice_veriloga_runtime::{GeneratedNoiseDescriptor, GeneratedNoiseEndpoint, GeneratedNoiseEvaluation, GeneratedNoiseEvaluationError, GeneratedNoiseEvaluationRef, GeneratedNoiseKind, GeneratedNoiseVisitor};

pub static NOISE_SOURCES: [GeneratedNoiseDescriptor; 4] = [
    GeneratedNoiseDescriptor { mechanism: "WHITE_B_BI_RB", label: Some("Rb"), kind: GeneratedNoiseKind::White, equation: 19, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(0), name: "b", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(3), name: "bi", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_E_EI_RE", label: Some("Re"), kind: GeneratedNoiseKind::White, equation: 22, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(1), name: "e", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(4), name: "ei", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_BI_EI_FLICKER_IBE", label: Some("flicker_Ibe"), kind: GeneratedNoiseKind::Flicker, equation: 27, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(3), name: "bi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(4), name: "ei", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_BI_EI_IBE", label: Some("Ibe"), kind: GeneratedNoiseKind::White, equation: 28, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(3), name: "bi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(4), name: "ei", is_internal: true }, table_len: 0, table_log_interp: false },
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
            let v1 = temperature;
            let v2 = node_potentials[2];
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
            let v26 = parameters[25];
            let v28 = 8.6170869e-5f64;
            let v32 = parameters[22];
            let v34 = parameters[21];
            let v39 = parameters[23];
            let v41 = parameters[0];
            let v44 = parameters[2];
            let v47 = parameters[47];
            let v48 = parameters[7];
            let v52 = parameters[5];
            let v53 = parameters[6];
            let v57 = parameters[9];
            let v58 = parameters[10];
            let v62 = 3.0015e2f64;
            let v65 = 1.16e0f64;
            let v66 = 7.02e-4f64;
            let v69 = 1.108e3f64;
            let v74 = 1.3806226e-23f64;
            let v78 = 1.3454442398941469e20f64;
            let v82 = 1.5e0f64;
            let v85 = 1.6021918e-19f64;
            let v89 = parameters[17];
            let v94 = parameters[29];
            let v95 = node_potentials[3];
            let v96 = node_potentials[4];
            let v99 = node_potentials[0];
            let v102 = node_potentials[1];
            let v106 = parameters[1];
            let v111 = parameters[11];
            let v116 = 8e1f64;
            let v124 = 3.7e1f64;
            let v126 = -3.7e1f64;
            let v133 = -3.7e1f64;
            let v147 = parameters[8];
            let v155 = parameters[4];
            let v157 = 1e-3f64;
            let v159 = -1e0f64;
            let v162 = parameters[3];
            let v178 = parameters[48];
            let v181 = parameters[49];
            let v184 = parameters[50];
            let v187 = parameters[51];
            let v190 = parameters[12];
            let v191 = parameters[37];
            let v198 = parameters[14];
            let v199 = parameters[38];
            let v206 = parameters[31];
            let v208 = parameters[13];
            let v210 = parameters[15];
            let v212 = parameters[32];
            let v214 = node_potentials[6];
            let v217 = parameters[20];
            let v219 = parameters[44];
            let v224 = parameters[24];
            let v228 = parameters[30];
            let v230 = parameters[33];
            let v233 = 2e0f64;
            let v236 = parameters[35];
            let v239 = -1e0f64;
            let v241 = 5.5224904e-23f64;
            let v250 = parameters[46];
            let v269 = parameters[28];
            let v271 = parameters[27];
            let v280 = -1e0f64;
            let v283 = 3.2043836e-19f64;
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
            let v27 = v26 + v6;
            let v29 = v28 * v18;
            let v30 = v18 / v27;
            let v31 = v30.ln();
            let v35 = v30 - v19;
            let v43 = v41 * (((v32 * v31) + ((v34 * v35) / v29)).exp());
            let v46 = v44 * ((v39 * v31).exp());
            let v51 = v47 * (v19 + (v48 * v35));
            let v56 = v52 * (v19 + (v53 * v35));
            let v61 = v57 * (v19 + (v58 * v35));
            let v64 = v18 / v62;
            let v88 = (-(v29 + v29)) * ((v82 * (v64.ln())) + (v85 * (((-(v65 - (((v66 * v18) * v18) / (v69 + v18)))) / (v74 * (v18 + v18))) + v78)));
            let v93 = (v64 * ((v89 - v88) / (v27 / v62))) + v88;
            let v98 = v94 * (v95 - v96);
            let v101 = v94 * (v99 - v95);
            let v104 = v94 * (v102 - v96);
            let v105 = if v43 > v0 { 1.0 } else { 0.0 };
            let v175: f64;
            if v105 != 0.0 {
                let v108 = v98 / (v106 * v29);
                let v112 = v111 * v29;
                let v113 = ((-v98) - v56) / v112;
                let v115 = (-v56) / v112;
                let v117 = if v108 > v116 { 1.0 } else { 0.0 };
                let v120: f64;
                let v121: f64;
                if v117 != 0.0 {
                    let v119 = v19 + (v108 - v116);
                    v120 = v119;
                    v121 = v116;
                } else {
                    v120 = v19;
                    v121 = v108;
                }
                let v123 = v120 * (v121.exp());
                let v125 = if v113 >= v124 { 1.0 } else { 0.0 };
                let v139: f64;
                if v125 != 0.0 {
                    v139 = v113;
                } else {
                    let v127 = if v113 <= v126 { 1.0 } else { 0.0 };
                    let v140: f64;
                    if v127 != 0.0 {
                        let v128 = v113.exp();
                        v140 = v128;
                    } else {
                        let v131 = ((v113.exp()) + v19).ln();
                        v140 = v131;
                    }
                    v139 = v140;
                }
                let v132 = if v115 >= v124 { 1.0 } else { 0.0 };
                let v141: f64;
                if v132 != 0.0 {
                    v141 = v115;
                } else {
                    let v134 = if v115 <= v133 { 1.0 } else { 0.0 };
                    let v142: f64;
                    if v134 != 0.0 {
                        let v135 = v115.exp();
                        v142 = v135;
                    } else {
                        let v138 = ((v115.exp()) + v19).ln();
                        v142 = v138;
                    }
                    v141 = v142;
                }
                let v153 = (v43 * (v123 - v19)) - ((v51 * (v139 - v141)) / (v19 + (v147 * ((v98.abs()).powf(v61)))));
                v175 = v153;
            } else {
                v175 = v0;
            }
            let v154 = if v46 > v0 { 1.0 } else { 0.0 };
            let v176: f64;
            if v154 != 0.0 {
                let v165 = ((v159 * v98) * v155) / ((v162 * v29) * (if (v155 - v98) >= v157 { (v155 - v98) } else { v157 }));
                let v166 = if v165 > v116 { 1.0 } else { 0.0 };
                let v169: f64;
                let v170: f64;
                if v166 != 0.0 {
                    let v168 = v19 + (v165 - v116);
                    v169 = v168;
                    v170 = v116;
                } else {
                    v169 = v19;
                    v170 = v165;
                }
                let v174 = v46 * ((v169 * (v170.exp())) - v19);
                v176 = v174;
            } else {
                v176 = v0;
            }
            let v177 = v175 - v176;
            let v197 = (v190 * ((v31 * v191).exp())) * ((v19 + (((v101 / v178).abs()).powf(v181))).powf((v19 / v181)));
            let v205 = (v198 * ((v31 * v199).exp())) * ((v19 + (((v104 / v184).abs()).powf(v187))).powf((v19 / v187)));
            let v207 = if v206 == v19 { 1.0 } else { 0.0 };
            let v215: f64;
            let v262: f64;
            if v207 != 0.0 {
                let v209 = v197 + v208;
                let v211 = v205 + v210;
                v215 = v209;
                v262 = v211;
            } else {
                v215 = v197;
                v262 = v205;
            }
            let v213 = if v212 == v19 { 1.0 } else { 0.0 };
            let v253: f64;
            if v213 != 0.0 {
                let v222 = v215 / (v19 + (((v214.abs()) / v217).powf(v219)));
                v253 = v222;
            } else {
                v253 = v215;
            }
            let v227 = if (v98 + ((-v93) * v224)) > v0 { 1.0 } else { 0.0 };
            if v227 != 0.0 {
            } else {
            }
            let v231 = if v230 > v0 { 1.0 } else { 0.0 };
            let v232 = if (if v228 == v19 { 1.0 } else { 0.0 }) != 0.0 && v231 != 0.0 { 1.0 } else { 0.0 };
            if v232 != 0.0 {
            } else {
                let v238 = if (if (if v228 == v233 { 1.0 } else { 0.0 }) != 0.0 && v231 != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if v236 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                if v238 != 0.0 {
                } else {
                    let v240 = if v228 == v239 { 1.0 } else { 0.0 };
                    if v240 != 0.0 {
                    } else {
                    }
                }
            }
            let v242 = v241 * v18;
            let v245 = (v190 + (v206 * v208)) / v25;
            let v248 = (v198 + (v206 * v210)) / v25;
            let v252 = if (if v245 > v0 { 1.0 } else { 0.0 }) != 0.0 && (if v245 >= v250 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let v286: f64;
            let v287: f64;
            if v252 != 0.0 {
                let v254 = v253 / v25;
                let v255 = if v254 > v250 { 1.0 } else { 0.0 };
                if v255 != 0.0 {
                } else {
                }
                let v256 = if v254 >= v250 { 1.0 } else { 0.0 };
                let v258: f64;
                if v256 != 0.0 {
                    let v257 = v242 / v254;
                    v258 = v257;
                } else {
                    v258 = v0;
                }
                v286 = v19;
                v287 = v258;
            } else {
                v286 = v0;
                v287 = v0;
            }
            let v261 = if (if v248 > v0 { 1.0 } else { 0.0 }) != 0.0 && (if v248 >= v250 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let v288: f64;
            let v289: f64;
            if v261 != 0.0 {
                let v263 = v262 / v25;
                let v264 = if v263 > v250 { 1.0 } else { 0.0 };
                if v264 != 0.0 {
                } else {
                }
                let v265 = if v263 >= v250 { 1.0 } else { 0.0 };
                let v267: f64;
                if v265 != 0.0 {
                    let v266 = v242 / v263;
                    v267 = v266;
                } else {
                    v267 = v0;
                }
                v288 = v19;
                v289 = v267;
            } else {
                v288 = v0;
                v289 = v0;
            }
            let v268 = v94 * v177;
            let v274 = if (if (if v269 > v0 { 1.0 } else { 0.0 }) != 0.0 && (if v271 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) > v0 { 1.0 } else { 0.0 };
            let v278: f64;
            if v274 != 0.0 {
                let v277 = v271 * ((v177.abs()).powf(v269));
                v278 = v277;
            } else {
                v278 = v0;
            }
            let v279 = if v268 >= v0 { 1.0 } else { 0.0 };
            let v281: f64;
            if v279 != 0.0 {
                v281 = v19;
            } else {
                v281 = v280;
            }
            let v282 = v281 * v278;
            let v285 = v283 * (v177.abs());
        if v286 == 0.0 {
            if !visitor.visit(0, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v287;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 0, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 0, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(0, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v288 == 0.0 {
            if !visitor.visit(1, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v289;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(1, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = v282;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 2, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = Some(v19);
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 2, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 2, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(2, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = v285;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 3, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 3, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(3, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        Ok(())
    }
}
