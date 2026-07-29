#![allow(dead_code, non_snake_case, unused_parens, unused_variables)]

use super::state::Instance;
use rspice_veriloga_runtime::GeneratedEvalContext;
pub use rspice_veriloga_runtime::{GeneratedNoiseDescriptor, GeneratedNoiseEndpoint, GeneratedNoiseEvaluation, GeneratedNoiseEvaluationError, GeneratedNoiseEvaluationRef, GeneratedNoiseKind, GeneratedNoiseVisitor};

pub static NOISE_SOURCES: [GeneratedNoiseDescriptor; 6] = [
    GeneratedNoiseDescriptor { mechanism: "WHITE_I2_I1_BODY_THERMAL_NOISE", label: Some("body thermal noise"), kind: GeneratedNoiseKind::White, equation: 12, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "i2", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(4), name: "i1", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_I2_I1_BODY_1_F_NOISE", label: Some("body 1/f noise"), kind: GeneratedNoiseKind::Flicker, equation: 13, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "i2", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(4), name: "i1", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_N1_I1_END_1_RESISTANCE_THERMAL_NOISE", label: Some("end 1 resistance thermal noise"), kind: GeneratedNoiseKind::White, equation: 14, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(0), name: "n1", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(4), name: "i1", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_N2_I2_END_2_RESISTANCE_THERMAL_NOISE", label: Some("end 2 resistance thermal noise"), kind: GeneratedNoiseKind::White, equation: 15, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(2), name: "n2", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(5), name: "i2", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_NC_I1_END_1_PARASITIC_SHOT_NOISE", label: Some("end 1 parasitic shot noise"), kind: GeneratedNoiseKind::White, equation: 16, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(1), name: "nc", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(4), name: "i1", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_NC_I2_END_2_PARASITIC_SHOT_NOISE", label: Some("end 2 parasitic shot noise"), kind: GeneratedNoiseKind::White, equation: 17, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(1), name: "nc", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(5), name: "i2", is_internal: true }, table_len: 0, table_log_interp: false },
];

impl Instance {
    pub fn evaluate_noise_sources(&self, ctx: &GeneratedEvalContext<'_>, visitor: &mut dyn GeneratedNoiseVisitor) -> Result<(), GeneratedNoiseEvaluationError> {
        if !self.multiplicity.is_finite() || self.multiplicity <= 0.0 {
            return Err(GeneratedNoiseEvaluationError::InvalidMultiplicity { value: self.multiplicity });
        }
        let parameters = &self.params.values;
        let multiplicity = self.multiplicity;
        let temperature = ctx.temperature();
        let node_potentials = [ctx.node_voltage(self.nodes[0]), ctx.node_voltage(self.nodes[1]), ctx.node_voltage(self.nodes[2]), ctx.node_voltage(self.nodes[3]), ctx.node_voltage(self.nodes[4]), ctx.node_voltage(self.nodes[5])];
            let v0 = 0e0f64;
            let v1 = 1.003e3f64;
            let v2 = parameters[20];
            let v4 = 1e0f64;
            let v5 = parameters[17];
            let v7 = parameters[18];
            let v9 = multiplicity;
            let v11 = 1e-2f64;
            let v12 = parameters[23];
            let v15 = parameters[22];
            let v17 = 1e6f64;
            let v20 = 2.7315e2f64;
            let v21 = parameters[28];
            let v23 = temperature;
            let v24 = parameters[9];
            let v27 = parameters[24];
            let v29 = parameters[25];
            let v31 = parameters[35];
            let v38 = parameters[36];
            let v48 = 1.3806505e-23f64;
            let v50 = 1.60217653e-19f64;
            let v54 = parameters[0];
            let v56 = parameters[1];
            let v58 = parameters[31];
            let v60 = parameters[32];
            let v62 = parameters[29];
            let v64 = parameters[30];
            let v66 = parameters[2];
            let v68 = 0e0f64;
            let v70 = parameters[4];
            let v72 = 0e0f64;
            let v74 = parameters[7];
            let v77 = 2e0f64;
            let v79 = parameters[5];
            let v81 = parameters[8];
            let v86 = 5e-1f64;
            let v88 = parameters[43];
            let v89 = parameters[44];
            let v93 = parameters[38];
            let v95 = parameters[39];
            let v98 = parameters[42];
            let v100 = parameters[41];
            let v106 = parameters[40];
            let v112 = parameters[127];
            let v113 = parameters[16];
            let v114 = parameters[119];
            let v115 = parameters[122];
            let v118 = parameters[11];
            let v119 = parameters[125];
            let v126 = parameters[120];
            let v127 = parameters[123];
            let v130 = parameters[12];
            let v131 = parameters[126];
            let v138 = parameters[118];
            let v139 = parameters[121];
            let v141 = parameters[10];
            let v142 = parameters[124];
            let v200 = parameters[45];
            let v203 = parameters[53];
            let v205 = parameters[56];
            let v209 = parameters[58];
            let v212 = parameters[54];
            let v213 = parameters[55];
            let v217 = parameters[57];
            let v221 = parameters[59];
            let v226 = parameters[103];
            let v227 = parameters[104];
            let v233 = 1e-1f64;
            let v237 = 1e4f64;
            let v240 = parameters[15];
            let v241 = parameters[49];
            let v242 = parameters[50];
            let v244 = parameters[51];
            let v247 = parameters[52];
            let v262 = parameters[63];
            let v264 = parameters[64];
            let v268 = 1.666666666666667e-1f64;
            let v276 = parameters[47];
            let v277 = parameters[48];
            let v281 = parameters[46];
            let v284 = 5.5e-1f64;
            let v291 = 1.1e0f64;
            let v297 = 4e0f64;
            let v303 = parameters[37];
            let v313 = 1e-99f64;
            let v315 = parameters[66];
            let v318 = parameters[67];
            let v326 = parameters[110];
            let v327 = parameters[111];
            let v330 = parameters[112];
            let v333 = parameters[113];
            let v337 = parameters[109];
            let v340 = parameters[93];
            let v341 = parameters[97];
            let v344 = parameters[95];
            let v345 = parameters[99];
            let v351 = parameters[94];
            let v352 = parameters[98];
            let v355 = parameters[96];
            let v356 = parameters[100];
            let v362 = parameters[72];
            let v364 = parameters[79];
            let v370 = node_potentials[3];
            let v371 = parameters[21];
            let v373 = node_potentials[5];
            let v374 = node_potentials[4];
            let v377 = node_potentials[1];
            let v406 = 1.1e-1f64;
            let v408 = 1e1f64;
            let v421 = parameters[101];
            let v422 = parameters[102];
            let v427 = 1.1e-1f64;
            let v435 = parameters[92];
            let v437 = parameters[69];
            let v439 = parameters[90];
            let v444 = parameters[91];
            let v448 = parameters[70];
            let v453 = parameters[27];
            let v458 = parameters[76];
            let v467 = parameters[77];
            let v487 = parameters[73];
            let v492 = -5e-1f64;
            let v501 = 3e0f64;
            let v522 = parameters[74];
            let v528 = parameters[80];
            let v533 = -5e-1f64;
            let v562 = parameters[81];
            let v565 = parameters[108];
            let v568 = parameters[86];
            let v572 = parameters[83];
            let v574 = parameters[105];
            let v575 = parameters[106];
            let v583 = parameters[85];
            let v584 = parameters[107];
            let v592 = parameters[84];
            let v597 = parameters[60];
            let v601 = parameters[62];
            let v602 = parameters[61];
            let v609 = parameters[65];
            let v630 = 1e3f64;
            let v633 = 1e5f64;
            let v636 = -1e0f64;
            let v661 = -4e-1f64;
            let v669 = -4e-1f64;
            let v673 = -4e-1f64;
            let v676 = -4e-1f64;
            let v688 = -1e0f64;
            let v692 = 9e0f64;
            let v693 = 2.25e0f64;
            let v698 = 1.5e0f64;
            let v720 = 3.333333333333333e-1f64;
            let v730 = 2.7e1f64;
            let v732 = 2.5e-1f64;
            let v738 = -5e-1f64;
            let v743 = -5e-1f64;
            let v749 = 1e-6f64;
            let v752 = -1e-6f64;
            let v761 = -1e-6f64;
            let v778 = 7.5e-1f64;
            let v793 = -2.5e-1f64;
            let v800 = 1e-4f64;
            let v804 = -2.5e-1f64;
            let v830 = 4.5e0f64;
            let v833 = 1e-9f64;
            let v1109 = parameters[14];
            let v1123 = parameters[33];
            let v1129 = parameters[34];
            let v1140 = 4e-2f64;
            let v1152 = parameters[68];
            let v1154 = parameters[75];
            let v1163 = parameters[82];
            let v1192 = parameters[26];
            let v1198 = parameters[13];
            let v1199 = parameters[89];
            let v1200 = 5.522602e-23f64;
            let v1206 = parameters[87];
            let v1215 = parameters[88];
            let v1219 = 5.522602e-23f64;
            let v1226 = 5.522602e-23f64;
            let v1230 = 3.20435306e-19f64;
            let v1239 = 3.20435306e-19f64;
            let v3 = if v1 != v2 { 1.0 } else { 0.0 };
            if v3 != 0.0 {
            } else {
            }
            let v6 = if v4 != v5 { 1.0 } else { 0.0 };
            if v6 != 0.0 {
            } else {
            }
            let v8 = if v4 < v7 { 1.0 } else { 0.0 };
            if v8 != 0.0 {
            } else {
            }
            let v10 = ctx.simparam_or("gmin", v0);
            let v18 = ((v4 - (v11 * v12)) * v15) * v17;
            let v19 = v18 * v18;
            let v22 = v20 + v21;
            if v4 != 0.0 {
            } else {
            }
            let v25 = v23 + v24;
            let v26 = v25 - v20;
            let v28 = if v26 < v27 { 1.0 } else { 0.0 };
            if v28 != 0.0 {
            } else {
            }
            let v30 = if v26 > v29 { 1.0 } else { 0.0 };
            if v30 != 0.0 {
            } else {
            }
            let v32 = v31 + v4;
            let v33 = if v26 < v32 { 1.0 } else { 0.0 };
            let v45: f64;
            if v33 != 0.0 {
                let v37 = v31 + (((v26 - v31) - v4).exp());
                v45 = v37;
            } else {
                let v40 = if v26 > (v38 - v4) { 1.0 } else { 0.0 };
                let v46: f64;
                if v40 != 0.0 {
                    let v44 = v38 - (((v38 - v26) - v4).exp());
                    v46 = v44;
                } else {
                    v46 = v26;
                }
                v45 = v46;
            }
            let v47 = v45 + v20;
            let v51 = (v48 * v47) / v50;
            let v52 = v47 / v22;
            let v53 = v47 - v22;
            let v55 = v54 * v18;
            let v57 = v56 * v18;
            let v59 = if v55 < v58 { 1.0 } else { 0.0 };
            if v59 != 0.0 {
            } else {
            }
            let v61 = if v55 > v60 { 1.0 } else { 0.0 };
            if v61 != 0.0 {
            } else {
            }
            let v63 = if v57 < v62 { 1.0 } else { 0.0 };
            if v63 != 0.0 {
            } else {
            }
            let v65 = if v57 > v64 { 1.0 } else { 0.0 };
            if v65 != 0.0 {
            } else {
            }
            let v69 = v68 * v19;
            let v71 = v70 * v18;
            let v73 = v72 * v19;
            let v75 = v74 * v18;
            let v76 = v57 * v55;
            let v80 = if v79 > v0 { 1.0 } else { 0.0 };
            let v82 = if v81 > v0 { 1.0 } else { 0.0 };
            let v83 = v80 + v82;
            let v85 = (v77 * v57) + (v83 * v55);
            let v87 = v86 * v83;
            let v110 = (((v55 + v93) + (v95 / v55)) + (v98 * (v4 - (((-v55) / v100).exp())))) / (v4 - ((v106 * (v66 * v18)) / v76));
            let v111 = v57 + (v87 * (v88 + (v89 / v55)));
            let v121: f64;
            let v133: f64;
            if v112 != 0.0 {
                v121 = v111;
                v133 = v110;
            } else {
                v121 = v57;
                v133 = v55;
            }
            let v194: f64;
            let v197: f64;
            let v304: f64;
            if v113 != 0.0 {
                let v122 = v9 * v121;
                let v125 = (v110 + (v114 * v115)) + ((v118 * v119) / (v122.sqrt()));
                let v137 = (v111 + (v126 * v127)) + ((v130 * v131) / ((v9 * v133).sqrt()));
                let v149 = (v11 * ((v138 * v139) + ((v141 * v142) / ((v122 * v133).sqrt())))).exp();
                v194 = v125;
                v197 = v137;
                v304 = v149;
            } else {
                let v154 = if (if v114 != v0 { 1.0 } else { 0.0 }) != 0.0 && (if (if v119 > v0 { 1.0 } else { 0.0 }) != 0.0 || (if v115 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v195: f64;
                if v154 != 0.0 {
                    let v157 = v119 / ((v9 * v121).sqrt());
                    let v163 = v110 + (v114 * (((v115 * v115) + (v157 * v157)).sqrt()));
                    v195 = v163;
                } else {
                    v195 = v110;
                }
                let v168 = if (if v126 != v0 { 1.0 } else { 0.0 }) != 0.0 && (if (if v131 > v0 { 1.0 } else { 0.0 }) != 0.0 || (if v127 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v198: f64;
                if v168 != 0.0 {
                    let v171 = v131 / ((v9 * v133).sqrt());
                    let v177 = v111 + (v126 * (((v127 * v127) + (v171 * v171)).sqrt()));
                    v198 = v177;
                } else {
                    v198 = v111;
                }
                let v182 = if (if v138 != v0 { 1.0 } else { 0.0 }) != 0.0 && (if (if v142 > v0 { 1.0 } else { 0.0 }) != 0.0 || (if v139 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v305: f64;
                if v182 != 0.0 {
                    let v186 = v142 / (((v9 * v121) * v133).sqrt());
                    let v193 = ((v11 * v138) * (((v139 * v139) + (v186 * v186)).sqrt())).exp();
                    v305 = v193;
                } else {
                    v305 = v4;
                }
                v194 = v195;
                v197 = v198;
                v304 = v305;
            }
            let v196 = if v194 <= v0 { 1.0 } else { 0.0 };
            if v196 != 0.0 {
            } else {
            }
            let v199 = if v197 <= v0 { 1.0 } else { 0.0 };
            if v199 != 0.0 {
            } else {
            }
            let v201 = v197 + v200;
            let v202 = if v201 <= v0 { 1.0 } else { 0.0 };
            if v202 != 0.0 {
            } else {
            }
            let v204: f64;
            let v208: f64;
            if v203 != 0.0 {
                v204 = v194;
                v208 = v197;
            } else {
                v204 = v55;
                v208 = v57;
            }
            let v207 = v4 / (v204.powf(v205));
            let v211 = v4 / (v208.powf(v209));
            let v232 = (((v212 * (v4 + (v213 * v207))) * (v4 + (v217 * v211))) * (v4 + ((v221 * v207) * v211))) * (v4 + (v53 * (v226 + (v53 * v227))));
            let v234 = if v232 > v233 { 1.0 } else { 0.0 };
            let v235: f64;
            if v234 != 0.0 {
                v235 = v232;
            } else {
                v235 = v233;
            }
            let v236 = v235.sqrt();
            let v239 = v236 / (v235 + v237);
            let v252: f64;
            if v240 != 0.0 {
                v252 = v0;
            } else {
                let v251 = v241 + ((((v242 * v208) + (v244 * v204)) + v247) / (v208 * v204));
                v252 = v251;
            }
            let v253 = if v252 < v239 { 1.0 } else { 0.0 };
            let v258: f64;
            let v309: f64;
            if v253 != 0.0 {
                let v254 = if v252 > v0 { 1.0 } else { 0.0 };
                let v255: f64;
                if v254 != 0.0 {
                    v255 = v252;
                } else {
                    v255 = v0;
                }
                let v256 = v239 * v239;
                v258 = v256;
                v309 = v255;
            } else {
                let v257 = v252 * v252;
                v258 = v257;
                v309 = v252;
            }
            let v260 = v235 * v86;
            let v261 = (v86 / v258) - v260;
            let v263 = if v262 > v4 { 1.0 } else { 0.0 };
            let v641: f64;
            let v809: f64;
            if v263 != 0.0 {
                let v267 = v261 - ((v77 * v264) / v258);
                let v270 = (v268 / v258) - v260;
                v641 = v267;
                v809 = v270;
            } else {
                let v271 = if v262 > v0 { 1.0 } else { 0.0 };
                let v642: f64;
                if v271 != 0.0 {
                    let v275 = v261 - (((v77 * v264) / v258).sqrt());
                    v642 = v275;
                } else {
                    v642 = v261;
                }
                v641 = v642;
                v809 = v0;
            }
            let v280 = v276 / (v4 + (v277 / v197));
            let v644: f64;
            let v887: f64;
            if v263 != 0.0 {
                let v282 = v281 * v51;
                let v283 = if v262 > v77 { 1.0 } else { 0.0 };
                let v293: f64;
                if v283 != 0.0 {
                    let v290 = (v284 * v51) * (v4 + (((-v280) / v51).exp()));
                    v293 = v290;
                } else {
                    let v292 = v291 * v51;
                    v293 = v292;
                }
                v644 = v282;
                v887 = v293;
            } else {
                let v294 = if v262 > v0 { 1.0 } else { 0.0 };
                let v645: f64;
                let v888: f64;
                if v294 != 0.0 {
                    let v296 = (v77 * v281) * v51;
                    let v299 = (v297 * v280) * v280;
                    v645 = v296;
                    v888 = v299;
                } else {
                    let v300 = v281 * v51;
                    let v302 = (v297 * v280) * v280;
                    v645 = v300;
                    v888 = v302;
                }
                v644 = v645;
                v887 = v888;
            }
            let v311 = v4 - (v309 * v236);
            let v312 = ((v303 * v304) * (v197 / v194)) * v311;
            let v314 = if v312 <= v313 { 1.0 } else { 0.0 };
            if v314 != 0.0 {
            } else {
            }
            let v316 = if v315 > v0 { 1.0 } else { 0.0 };
            let v317 = if v316 != 0.0 && v80 != 0.0 { 1.0 } else { 0.0 };
            let v1190: f64;
            if v317 != 0.0 {
                let v321 = (v315 + (v318 / v55)) / v79;
                v1190 = v321;
            } else {
                v1190 = v0;
            }
            let v322 = if v316 != 0.0 && v82 != 0.0 { 1.0 } else { 0.0 };
            let v1195: f64;
            if v322 != 0.0 {
                let v325 = (v315 + (v318 / v55)) / v81;
                v1195 = v325;
            } else {
                v1195 = v0;
            }
            let v1107: f64;
            if v240 != 0.0 {
                v1107 = v0;
            } else {
                let v339 = (((v326 + (v327 * v85)) + (v330 * v76)) + (v333 * (v79 + v81))) * (v52.powf(v337));
                v1107 = v339;
            }
            let v350 = (v340 + (v341 / v194)) + ((v87 * (v344 + (v345 / v194))) / v197);
            let v361 = (v351 + (v352 / v194)) + ((v87 * (v355 + (v356 / v194))) / v197);
            let v366 = (v362 * v69) + (v364 * v71);
            let v369 = (v362 * v73) + (v364 * v75);
            let v372 = -v371;
            let v376 = v372 * (v373 - v374);
            let v379 = v372 * (v377 - v374);
            let v381 = v372 * (v377 - v373);
            let v383 = (v25 + v370) - v20;
            let v384 = if v383 < v32 { 1.0 } else { 0.0 };
            let v395: f64;
            if v384 != 0.0 {
                let v388 = v31 + (((v383 - v31) - v4).exp());
                v395 = v388;
            } else {
                let v390 = if v383 > (v38 - v4) { 1.0 } else { 0.0 };
                let v396: f64;
                if v390 != 0.0 {
                    let v394 = v38 - (((v38 - v383) - v4).exp());
                    v396 = v394;
                } else {
                    v396 = v383;
                }
                v395 = v396;
            }
            let v397 = v395 + v20;
            let v399 = (v48 * v397) / v50;
            let v400 = v397 / v22;
            let v401 = v397 - v22;
            let v405 = v4 + (v401 * (v350 + (v401 * v361)));
            let v407 = if v405 < v406 { 1.0 } else { 0.0 };
            let v416: f64;
            if v407 != 0.0 {
                let v414 = v11 + (v233 * (((v408 * (v405 - v11)) - v4).exp()));
                v416 = v414;
            } else {
                v416 = v405;
            }
            let v972: f64;
            if v262 != 0.0 {
                let v418 = v4 / ((v312 * v311) * v416);
                v972 = v418;
            } else {
                let v420 = v4 / (v312 * v416);
                v972 = v420;
            }
            let v426 = v4 + (v401 * (v421 + (v401 * v422)));
            let v428 = if v426 < v427 { 1.0 } else { 0.0 };
            let v1194: f64;
            if v428 != 0.0 {
                let v434 = v11 + (v233 * (((v408 * (v426 - v11)) - v4).exp()));
                v1194 = v434;
            } else {
                v1194 = v426;
            }
            let v436 = v400.powf(v435);
            let v438 = if v437 > v0 { 1.0 } else { 0.0 };
            let v476: f64;
            let v988: f64;
            if v438 != 0.0 {
                let v451 = v437 * ((((((-v439) * (v4 - v400)) / v399) + (v444 * (v400.ln()))) / v448).exp());
                let v457 = (v448 * v399) * ((v4 + (v453 / v451)).ln());
                v476 = v451;
                v988 = v457;
            } else {
                v476 = v0;
                v988 = v0;
            }
            let v459 = if v458 > v0 { 1.0 } else { 0.0 };
            let v478: f64;
            let v1004: f64;
            if v459 != 0.0 {
                let v470 = v458 * ((((((-v439) * (v4 - v400)) / v399) + (v444 * (v400.ln()))) / v467).exp());
                let v475 = (v467 * v399) * ((v4 + (v453 / v470)).ln());
                v478 = v470;
                v1004 = v475;
            } else {
                v478 = v0;
                v1004 = v0;
            }
            let v477 = v69 * v476;
            let v479 = v71 * v478;
            let v480 = v477 + v479;
            let v481 = v73 * v476;
            let v482 = v75 * v478;
            let v483 = v481 + v482;
            let v484 = if v362 > v0 { 1.0 } else { 0.0 };
            let v1145: f64;
            let v1150: f64;
            if v484 != 0.0 {
                let v508 = ((((v77 * (v399 / v400)) * ((((((v86 * v487) * v400) / v399).exp()) - ((((v492 * v487) * v400) / v399).exp())).ln())) * v400) - ((v501 * v399) * (v400.ln()))) - (v439 * (v400 - v4));
                let v520 = v508 + ((v77 * v399) * ((v86 * (v4 + ((v4 + (v297 * (((-v508) / v399).exp()))).sqrt()))).ln()));
                let v524 = v362 * ((v487 / v520).powf(v522));
                v1145 = v524;
                v1150 = v520;
            } else {
                v1145 = v0;
                v1150 = v487;
            }
            let v525 = if v364 > v0 { 1.0 } else { 0.0 };
            let v1147: f64;
            let v1160: f64;
            if v525 != 0.0 {
                let v548 = ((((v77 * (v399 / v400)) * ((((((v86 * v528) * v400) / v399).exp()) - ((((v533 * v528) * v400) / v399).exp())).ln())) * v400) - ((v501 * v399) * (v400.ln()))) - (v439 * (v400 - v4));
                let v560 = v548 + ((v77 * v399) * ((v86 * (v4 + ((v4 + (v297 * (((-v548) / v399).exp()))).sqrt()))).ln()));
                let v564 = v364 * ((v528 / v560).powf(v562));
                v1147 = v564;
                v1160 = v560;
            } else {
                v1147 = v0;
                v1160 = v528;
            }
            let v569 = (v4 + (v401 * v565)) * v568;
            let v570 = if v569 > v0 { 1.0 } else { 0.0 };
            let v571: f64;
            if v570 != 0.0 {
                v571 = v569;
            } else {
                v571 = v0;
            }
            let v573 = if v572 > v0 { 1.0 } else { 0.0 };
            let v1020: f64;
            let v1024: f64;
            let v1027: f64;
            if v573 != 0.0 {
                let v580 = v572 * (v4 + (v401 * (v574 + (v401 * v575))));
                let v581 = if v580 > v0 { 1.0 } else { 0.0 };
                let v582: f64;
                if v581 != 0.0 {
                    v582 = v580;
                } else {
                    v582 = v0;
                }
                let v587 = v583 * (v4 + (v584 * v401));
                let v588 = v587 * v399;
                let v596 = v588 * (((((-v582) / v588).exp()) + (v453 / v592)).ln());
                v1020 = v582;
                v1024 = v587;
                v1027 = v596;
            } else {
                v1020 = v572;
                v1024 = v583;
                v1027 = v4;
            }
            let v599 = if v240 == 0.0 { 1.0 } else { 0.0 };
            let v600 = if (if v597 > v0 { 1.0 } else { 0.0 }) != 0.0 && v599 != 0.0 { 1.0 } else { 0.0 };
            let v631: f64;
            let v683: f64;
            let v844: f64;
            let v852: f64;
            let v859: f64;
            if v600 != 0.0 {
                let v607: f64;
                let v612: f64;
                if v601 != 0.0 {
                    let v604 = (v602 * v436) * v416;
                    let v606 = (v597 * v436) * v416;
                    v607 = v604;
                    v612 = v606;
                } else {
                    v607 = v602;
                    v612 = v597;
                }
                let v619 = (((v607 * v607) + ((((v297 * v609) * v609) * v612) * v612)).sqrt()) - ((v77 * v609) * v612);
                let v621 = (v609 * v619) / v612;
                let v627 = (((v619 * v619) / (v612 * v612)) + (v297 * v621)).sqrt();
                let v628 = v612 - v607;
                let v629 = v4 / v612;
                v631 = v628;
                v683 = v629;
                v844 = v619;
                v852 = v621;
                v859 = v627;
            } else {
                v631 = v630;
                v683 = v0;
                v844 = v0;
                v852 = v0;
                v859 = v0;
            }
            let v632 = v201 * v631;
            let v634 = if v632 > v633 { 1.0 } else { 0.0 };
            let v694: f64;
            if v634 != 0.0 {
                v694 = v633;
            } else {
                v694 = v632;
            }
            let v635 = if v376 < v0 { 1.0 } else { 0.0 };
            let v640: f64;
            let v662: f64;
            let v980: f64;
            if v635 != 0.0 {
                let v637 = -v381;
                let v638 = -v376;
                v640 = v637;
                v662 = v638;
                v980 = v636;
            } else {
                let v639 = -v379;
                v640 = v639;
                v662 = v376;
                v980 = v4;
            }
            let v643 = if v640 > v641 { 1.0 } else { 0.0 };
            let v660: f64;
            if v643 != 0.0 {
                let v652 = v641 - (v644 * ((v4 + (((v641 - v640) / v644).exp())).ln()));
                v660 = v652;
            } else {
                let v659 = v640 - (v644 * ((v4 + (((v640 - v641) / v644).exp())).ln()));
                v660 = v659;
            }
            let v678: f64;
            if v262 != 0.0 {
                let v663 = v641 - v660;
                let v664 = if v662 < v663 { 1.0 } else { 0.0 };
                let v665: f64;
                if v664 != 0.0 {
                    v665 = v662;
                } else {
                    v665 = v663;
                }
                let v668 = if v660 < (v661 * (v235 + v665)) { 1.0 } else { 0.0 };
                let v679: f64;
                if v668 != 0.0 {
                    let v670: f64;
                    if v664 != 0.0 {
                        v670 = v662;
                    } else {
                        v670 = v663;
                    }
                    let v672 = v669 * (v235 + v670);
                    v679 = v672;
                } else {
                    v679 = v660;
                }
                v678 = v679;
            } else {
                let v675 = if v660 < (v673 * v235) { 1.0 } else { 0.0 };
                let v680: f64;
                if v675 != 0.0 {
                    let v677 = v676 * v235;
                    v680 = v677;
                } else {
                    v680 = v660;
                }
                v678 = v680;
            }
            let v682 = v235 + (v77 * v678);
            let v684 = if v683 > v0 { 1.0 } else { 0.0 };
            let v836: f64;
            if v684 != 0.0 {
                let v703 = ((v297 * v694) * v694) / v258;
                let v704 = (((v258 * v682) * v682) - v682) * v703;
                let v705 = (v688 + ((v501 * v258) * v682)) * v703;
                let v706 = (v258 * (v693 + (v682 / v694))) * v703;
                let v707 = ((v698 * v258) / v694) * v703;
                let v708 = v707 * v707;
                let v709 = -v706;
                let v712 = (v707 * v705) - (v297 * v704);
                let v722 = v712 - ((v709 * v709) * v720);
                let v727 = ((((v297 * v706) * v704) - (v705 * v705)) - (v704 * v708)) - ((v709 * (v712 + (v77 * v722))) / v692);
                let v731 = ((v722 * v722) * v722) / v730;
                let v736 = (((v732 * v727) * v727) + v731).sqrt();
                let v737 = if v727 < v0 { 1.0 } else { 0.0 };
                let v748: f64;
                let v758: f64;
                if v737 != 0.0 {
                    let v740 = (v738 * v727) + v736;
                    let v742 = (-v731) / v740;
                    v748 = v740;
                    v758 = v742;
                } else {
                    let v745 = (v743 * v727) - v736;
                    let v747 = (-v731) / v745;
                    v748 = v747;
                    v758 = v745;
                }
                let v750 = if v748 > v749 { 1.0 } else { 0.0 };
                let v767: f64;
                if v750 != 0.0 {
                    let v751 = v748.powf(v720);
                    v767 = v751;
                } else {
                    let v753 = if v748 < v752 { 1.0 } else { 0.0 };
                    let v768: f64;
                    if v753 != 0.0 {
                        let v756 = -((-v748).powf(v720));
                        v768 = v756;
                    } else {
                        let v757 = v237 * v748;
                        v768 = v757;
                    }
                    v767 = v768;
                }
                let v759 = if v758 > v749 { 1.0 } else { 0.0 };
                let v769: f64;
                if v759 != 0.0 {
                    let v760 = v758.powf(v720);
                    v769 = v760;
                } else {
                    let v762 = if v758 < v761 { 1.0 } else { 0.0 };
                    let v770: f64;
                    if v762 != 0.0 {
                        let v765 = -((-v758).powf(v720));
                        v770 = v765;
                    } else {
                        let v766 = v237 * v758;
                        v770 = v766;
                    }
                    v769 = v770;
                }
                let v774 = v732 * v708;
                let v777 = ((v774 - v706) + ((v767 + v769) - (v709 * v720))).sqrt();
                let v783 = ((v778 * v708) - (v777 * v777)) - (v77 * v706);
                let v789 = (((v707 * v706) - (v77 * v705)) - (v774 * v707)) / v777;
                let v790 = v783 + v789;
                let v791 = if v790 > v0 { 1.0 } else { 0.0 };
                let v837: f64;
                if v791 != 0.0 {
                    let v797 = (v793 * v707) + (v86 * ((v790.sqrt()) + v777));
                    v837 = v797;
                } else {
                    let v798 = v783 - v789;
                    let v808 = (v804 * v707) + (v86 * (((((v798 * v798) + v800).sqrt()).sqrt()) - v777));
                    v837 = v808;
                }
                v836 = v837;
            } else {
                let v810 = if v678 > v809 { 1.0 } else { 0.0 };
                let v838: f64;
                if v810 != 0.0 {
                    let v811 = v261 - v678;
                    let v812 = v258 * v811;
                    let v823 = ((v77 * (v4 - (v77 * v812))) * v811) / ((v4 - (v501 * v812)) + ((v4 - (v698 * v812)).sqrt()));
                    v838 = v823;
                } else {
                    let v825 = (v501 * v258) * v682;
                    let v832 = ((v4 - v825) + ((v4 + v825).sqrt())) / (v830 * v258);
                    v838 = v832;
                }
                v836 = v838;
            }
            let v835 = if v263 != 0.0 && (if v309 > v833 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let v970: f64;
            let v975: f64;
            let v982: f64;
            if v835 != 0.0 {
                let v839 = v836 + v51;
                let v840 = v682 + v836;
                let v842 = v309 * (v840.sqrt());
                let v883: f64;
                if v684 != 0.0 {
                    let v843 = v839 / v201;
                    let v847 = (v86 * (v843 - v844)) * v683;
                    let v850 = (v86 * (v843 + v844)) * v683;
                    let v854 = ((v847 * v847) + v852).sqrt();
                    let v857 = ((v850 * v850) + v852).sqrt();
                    let v876 = ((((v77 * v842) * (v4 - v842)) * (v4 - (((((v86 * ((v847 / v854) + (v850 / v857))) * v683) / v201) * v839) / (v4 + ((v854 + v857) - v859))))) / v839).sqrt();
                    v883 = v876;
                } else {
                    let v881 = (((v77 * v842) * (v4 - v842)) / v839).sqrt();
                    v883 = v881;
                }
                let v886 = ((v258 * v840) / (v883 * v883)) - v839;
                let v890 = v276 + v839;
                let v892 = v887 + ((v276 * v836) / v890);
                let v894 = (v297 * v892) * v892;
                let v896 = (v77 * v662) * v839;
                let v897 = v662 - v839;
                let v898 = v897 * v897;
                let v901 = v662 + v839;
                let v902 = v901 * v901;
                let v906 = v896 / (((v898 + v894).sqrt()) + ((v902 + v894).sqrt()));
                let v907 = if v262 > v77 { 1.0 } else { 0.0 };
                let v919: f64;
                if v907 != 0.0 {
                    let v910 = v887 + ((v276 * v906) / v890);
                    let v912 = (v297 * v910) * v910;
                    let v918 = v896 / (((v898 + v912).sqrt()) + ((v902 + v912).sqrt()));
                    v919 = v918;
                } else {
                    v919 = v906;
                }
                let v923 = v4 - (v883 * ((v886 + v919).sqrt()));
                let v976: f64;
                if v684 != 0.0 {
                    let v924 = v919 / v201;
                    let v927 = (v86 * (v924 - v844)) * v683;
                    let v930 = (v86 * (v924 + v844)) * v683;
                    let v938 = ((((v927 * v927) + v852).sqrt()) + (((v930 * v930) + v852).sqrt())) - v859;
                    v976 = v938;
                } else {
                    v976 = v0;
                }
                v970 = v923;
                v975 = v976;
                v982 = v919;
            } else {
                let v941 = v662 - v836;
                let v945 = v662 + v836;
                let v950 = ((v77 * v662) * v836) / ((((v941 * v941) + v887).sqrt()) + (((v945 * v945) + v887).sqrt()));
                let v977: f64;
                if v684 != 0.0 {
                    let v951 = v950 / v201;
                    let v954 = (v86 * (v951 - v844)) * v683;
                    let v957 = (v86 * (v951 + v844)) * v683;
                    let v965 = ((((v954 * v954) + v852).sqrt()) + (((v957 * v957) + v852).sqrt())) - v859;
                    v977 = v965;
                } else {
                    v977 = v0;
                }
                let v969 = v4 - (v309 * ((v682 + v950).sqrt()));
                v970 = v969;
                v975 = v977;
                v982 = v950;
            }
            let v971 = if v970 < v264 { 1.0 } else { 0.0 };
            let v973: f64;
            if v971 != 0.0 {
                v973 = v264;
            } else {
                v973 = v970;
            }
            let v979 = (v972 * v973) / (v4 + v975);
            let v983 = (v980 * v979) * v982;
            let v984 = if v480 > v0 { 1.0 } else { 0.0 };
            let v1105: f64;
            let v1231: f64;
            let v1235: f64;
            if v984 != 0.0 {
                let v985 = if v477 > v0 { 1.0 } else { 0.0 };
                let v1017: f64;
                if v985 != 0.0 {
                    let v987 = v4 / (v448 * v399);
                    let v989 = if v379 < v988 { 1.0 } else { 0.0 };
                    let v998: f64;
                    if v989 != 0.0 {
                        let v991 = (v379 * v987).exp();
                        v998 = v991;
                    } else {
                        let v997 = ((v988 * v987).exp()) * (v4 + ((v379 - v988) * v987));
                        v998 = v997;
                    }
                    let v1000 = v477 * (v998 - v4);
                    v1017 = v1000;
                } else {
                    v1017 = v0;
                }
                let v1001 = if v479 > v0 { 1.0 } else { 0.0 };
                let v1018: f64;
                if v1001 != 0.0 {
                    let v1003 = v4 / (v467 * v399);
                    let v1005 = if v379 < v1004 { 1.0 } else { 0.0 };
                    let v1014: f64;
                    if v1005 != 0.0 {
                        let v1007 = (v379 * v1003).exp();
                        v1014 = v1007;
                    } else {
                        let v1013 = ((v1004 * v1003).exp()) * (v4 + ((v379 - v1004) * v1003));
                        v1014 = v1013;
                    }
                    let v1016 = v479 * (v1014 - v4);
                    v1018 = v1016;
                } else {
                    v1018 = v0;
                }
                let v1019 = v1017 + v1018;
                let v1021 = if v1020 > v0 { 1.0 } else { 0.0 };
                let v1043: f64;
                if v1021 != 0.0 {
                    let v1022 = -v1020;
                    let v1023 = v1022 - v379;
                    let v1026 = v4 / (v1024 * v399);
                    let v1028 = if v1023 < v1027 { 1.0 } else { 0.0 };
                    let v1038: f64;
                    if v1028 != 0.0 {
                        let v1030 = (v1023 * v1026).exp();
                        v1038 = v1030;
                    } else {
                        let v1036 = ((v1027 * v1026).exp()) * (v4 + ((v1023 - v1027) * v1026));
                        v1038 = v1036;
                    }
                    let v1042 = (-v592) * (v1038 - ((v1022 * v1026).exp()));
                    v1043 = v1042;
                } else {
                    v1043 = v0;
                }
                let v1046 = (v1019 + v1043) + (v10 * v379);
                v1105 = v1046;
                v1231 = v1019;
                v1235 = v1043;
            } else {
                v1105 = v0;
                v1231 = v0;
                v1235 = v0;
            }
            let v1047 = if v483 > v0 { 1.0 } else { 0.0 };
            let v1106: f64;
            let v1240: f64;
            let v1244: f64;
            if v1047 != 0.0 {
                let v1048 = if v481 > v0 { 1.0 } else { 0.0 };
                let v1078: f64;
                if v1048 != 0.0 {
                    let v1050 = v4 / (v448 * v399);
                    let v1051 = if v381 < v988 { 1.0 } else { 0.0 };
                    let v1060: f64;
                    if v1051 != 0.0 {
                        let v1053 = (v381 * v1050).exp();
                        v1060 = v1053;
                    } else {
                        let v1059 = ((v988 * v1050).exp()) * (v4 + ((v381 - v988) * v1050));
                        v1060 = v1059;
                    }
                    let v1062 = v481 * (v1060 - v4);
                    v1078 = v1062;
                } else {
                    v1078 = v0;
                }
                let v1063 = if v482 > v0 { 1.0 } else { 0.0 };
                let v1079: f64;
                if v1063 != 0.0 {
                    let v1065 = v4 / (v467 * v399);
                    let v1066 = if v381 < v1004 { 1.0 } else { 0.0 };
                    let v1075: f64;
                    if v1066 != 0.0 {
                        let v1068 = (v381 * v1065).exp();
                        v1075 = v1068;
                    } else {
                        let v1074 = ((v1004 * v1065).exp()) * (v4 + ((v381 - v1004) * v1065));
                        v1075 = v1074;
                    }
                    let v1077 = v482 * (v1075 - v4);
                    v1079 = v1077;
                } else {
                    v1079 = v0;
                }
                let v1080 = v1078 + v1079;
                let v1081 = if v1020 > v0 { 1.0 } else { 0.0 };
                let v1101: f64;
                if v1081 != 0.0 {
                    let v1082 = -v1020;
                    let v1083 = v1082 - v381;
                    let v1085 = v4 / (v1024 * v399);
                    let v1086 = if v1083 < v1027 { 1.0 } else { 0.0 };
                    let v1096: f64;
                    if v1086 != 0.0 {
                        let v1088 = (v1083 * v1085).exp();
                        v1096 = v1088;
                    } else {
                        let v1094 = ((v1027 * v1085).exp()) * (v4 + ((v1083 - v1027) * v1085));
                        v1096 = v1094;
                    }
                    let v1100 = (-v592) * (v1096 - ((v1082 * v1085).exp()));
                    v1101 = v1100;
                } else {
                    v1101 = v0;
                }
                let v1104 = (v1080 + v1101) + (v10 * v381);
                v1106 = v1104;
                v1240 = v1080;
                v1244 = v1101;
            } else {
                v1106 = v0;
                v1240 = v0;
                v1244 = v0;
            }
            let v1108 = if v1107 > v0 { 1.0 } else { 0.0 };
            let v1111 = if (if v1108 != 0.0 && v1109 != 0.0 { 1.0 } else { 0.0 }) != 0.0 && v599 != 0.0 { 1.0 } else { 0.0 };
            if v1111 != 0.0 {
                let v1112 = if v337 == v0 { 1.0 } else { 0.0 };
                if v1112 != 0.0 {
                } else {
                    if v33 != 0.0 {
                    } else {
                        let v1114 = if v26 > (v38 - v4) { 1.0 } else { 0.0 };
                        if v1114 != 0.0 {
                        } else {
                        }
                    }
                    let v1117 = if ((v337 + v4).abs()) > v233 { 1.0 } else { 0.0 };
                    if v1117 != 0.0 {
                    } else {
                    }
                }
            } else {
            }
            let v1118 = v372 * v983;
            let v1119 = v372 * v1105;
            let v1120 = v372 * v1106;
            let v1124 = if ((v1118 / v194).abs()) > v1123 { 1.0 } else { 0.0 };
            if v1124 != 0.0 {
            } else {
            }
            let v1127 = if ((v1119 / v194).abs()) > v1123 { 1.0 } else { 0.0 };
            if v1127 != 0.0 {
            } else {
            }
            let v1130 = if (v379.abs()) > v1129 { 1.0 } else { 0.0 };
            if v1130 != 0.0 {
            } else {
            }
            let v1133 = if ((v1120 / v194).abs()) > v1123 { 1.0 } else { 0.0 };
            if v1133 != 0.0 {
            } else {
            }
            let v1135 = if (v381.abs()) > v1129 { 1.0 } else { 0.0 };
            if v1135 != 0.0 {
            } else {
            }
            let v1136 = if v366 > v0 { 1.0 } else { 0.0 };
            if v1136 != 0.0 {
                let v1156: f64;
                if v262 != 0.0 {
                    let v1138 = v379 + v261;
                    let v1144 = v86 * ((v379 - v261) + (((v1138 * v1138) + v1140).sqrt()));
                    v1156 = v1144;
                } else {
                    v1156 = v379;
                }
                let v1148 = v71 * v1147;
                let v1149 = if (v69 * v1145) > v0 { 1.0 } else { 0.0 };
                if v1149 != 0.0 {
                    let v1153 = (-v1150) * v1152;
                    let v1155 = if v1154 <= v0 { 1.0 } else { 0.0 };
                    if v1155 != 0.0 {
                        let v1158 = if (v1156 + v1153) > v0 { 1.0 } else { 0.0 };
                        if v1158 != 0.0 {
                        } else {
                        }
                    } else {
                    }
                } else {
                }
                let v1159 = if v1148 > v0 { 1.0 } else { 0.0 };
                if v1159 != 0.0 {
                    let v1162 = (-v1160) * v1152;
                    let v1164 = if v1163 <= v0 { 1.0 } else { 0.0 };
                    if v1164 != 0.0 {
                        let v1166 = if (v1156 + v1162) > v0 { 1.0 } else { 0.0 };
                        if v1166 != 0.0 {
                        } else {
                        }
                    } else {
                    }
                } else {
                }
            } else {
            }
            let v1167 = if v369 > v0 { 1.0 } else { 0.0 };
            if v1167 != 0.0 {
                let v1181: f64;
                if v262 != 0.0 {
                    let v1169 = v381 + v261;
                    let v1174 = v86 * ((v381 - v261) + (((v1169 * v1169) + v1140).sqrt()));
                    v1181 = v1174;
                } else {
                    v1181 = v381;
                }
                let v1176 = v75 * v1147;
                let v1177 = if (v73 * v1145) > v0 { 1.0 } else { 0.0 };
                if v1177 != 0.0 {
                    let v1179 = (-v1150) * v1152;
                    let v1180 = if v1154 <= v0 { 1.0 } else { 0.0 };
                    if v1180 != 0.0 {
                        let v1183 = if (v1181 + v1179) > v0 { 1.0 } else { 0.0 };
                        if v1183 != 0.0 {
                        } else {
                        }
                    } else {
                    }
                } else {
                }
                let v1184 = if v1176 > v0 { 1.0 } else { 0.0 };
                if v1184 != 0.0 {
                    let v1186 = (-v1160) * v1152;
                    let v1187 = if v1163 <= v0 { 1.0 } else { 0.0 };
                    if v1187 != 0.0 {
                        let v1189 = if (v1181 + v1186) > v0 { 1.0 } else { 0.0 };
                        if v1189 != 0.0 {
                        } else {
                        }
                    } else {
                    }
                } else {
                }
            } else {
            }
            let v1193 = if (v1190 / v9) <= v1192 { 1.0 } else { 0.0 };
            if v1193 != 0.0 {
            } else {
            }
            let v1197 = if (v1195 / v9) <= v1192 { 1.0 } else { 0.0 };
            if v1197 != 0.0 {
            } else {
            }
            let v1259: f64;
            let v1260: f64;
            let v1261: f64;
            let v1262: f64;
            let v1263: f64;
            let v1264: f64;
            let v1265: f64;
            let v1266: f64;
            let v1267: f64;
            let v1268: f64;
            let v1270: f64;
            let v1272: f64;
            let v1274: f64;
            if v1198 != 0.0 {
                let v1203: f64;
                let v1210: f64;
                if v1199 != 0.0 {
                    v1203 = v194;
                    v1210 = v197;
                } else {
                    v1203 = v55;
                    v1210 = v57;
                }
                let v1202 = (v1200 * v397) * v979;
                let v1211 = ((v571 * (((v1118 / v1203).abs()).powf(v1206))) * v1203) / v1210;
                let v1212 = if v1118 < v0 { 1.0 } else { 0.0 };
                let v1214: f64;
                if v1212 != 0.0 {
                    let v1213 = -v1211;
                    v1214 = v1213;
                } else {
                    v1214 = v1211;
                }
                let v1216 = if v1190 > v0 { 1.0 } else { 0.0 };
                let v1221: f64;
                if v1216 != 0.0 {
                    let v1218 = v4 / (v1190 * v1194);
                    v1221 = v1218;
                } else {
                    v1221 = v0;
                }
                let v1222 = (v1219 * v397) * v1221;
                let v1223 = if v1195 > v0 { 1.0 } else { 0.0 };
                let v1228: f64;
                if v1223 != 0.0 {
                    let v1225 = v4 / (v1195 * v1194);
                    v1228 = v1225;
                } else {
                    v1228 = v0;
                }
                let v1229 = (v1226 * v397) * v1228;
                let v1269: f64;
                let v1271: f64;
                if v984 != 0.0 {
                    let v1238 = v1230 * (((v1231 + (v77 * v480)).abs()) + (v1235.abs()));
                    v1269 = v4;
                    v1271 = v1238;
                } else {
                    v1269 = v0;
                    v1271 = v0;
                }
                let v1273: f64;
                let v1275: f64;
                if v1047 != 0.0 {
                    let v1247 = v1239 * (((v1240 + (v77 * v483)).abs()) + (v1244.abs()));
                    v1273 = v4;
                    v1275 = v1247;
                } else {
                    v1273 = v0;
                    v1275 = v0;
                }
                v1259 = v4;
                v1260 = v1202;
                v1261 = v4;
                v1262 = v1214;
                v1263 = v1215;
                v1264 = v4;
                v1265 = v1222;
                v1266 = v4;
                v1267 = v1229;
                v1268 = v1269;
                v1270 = v1271;
                v1272 = v1273;
                v1274 = v1275;
            } else {
                v1259 = v0;
                v1260 = v0;
                v1261 = v0;
                v1262 = v0;
                v1263 = v0;
                v1264 = v0;
                v1265 = v0;
                v1266 = v0;
                v1267 = v0;
                v1268 = v0;
                v1270 = v0;
                v1272 = v0;
                v1274 = v0;
            }
            let v1249: f64;
            if v1108 != 0.0 {
                let v1248 = v4 / v1107;
                v1249 = v1248;
            } else {
                v1249 = v0;
            }
            let v1250 = 0e0f64;
            let v1258 = if ((v1250 + (((0e0f64) * v1249) * (v1118 + (v376 * v1250)))).abs()) > v313 { 1.0 } else { 0.0 };
            if v1258 != 0.0 {
            } else {
            }
        if v1259 == 0.0 {
            if !visitor.visit(0, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v1260;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 0, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 0, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(0, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v1261 == 0.0 {
            if !visitor.visit(1, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v1262;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = Some(v1263);
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(1, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v1264 == 0.0 {
            if !visitor.visit(2, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v1265;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 2, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 2, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(2, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v1266 == 0.0 {
            if !visitor.visit(3, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v1267;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 3, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 3, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(3, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v1268 == 0.0 {
            if !visitor.visit(4, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v1270;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 4, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 4, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(4, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v1272 == 0.0 {
            if !visitor.visit(5, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v1274;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 5, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 5, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(5, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        Ok(())
    }
}
