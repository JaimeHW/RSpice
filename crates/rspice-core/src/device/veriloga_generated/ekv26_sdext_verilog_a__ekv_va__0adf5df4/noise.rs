#![allow(dead_code, non_snake_case, unused_parens, unused_variables)]

use super::state::Instance;
use crate::device::veriloga_generated::GeneratedEvalContext;
pub use crate::device::veriloga_generated::{GeneratedNoiseDescriptor, GeneratedNoiseEndpoint, GeneratedNoiseEvaluation, GeneratedNoiseEvaluationError, GeneratedNoiseEvaluationRef, GeneratedNoiseKind, GeneratedNoiseVisitor};

pub static NOISE_SOURCES: [GeneratedNoiseDescriptor; 2] = [
    GeneratedNoiseDescriptor { mechanism: "WHITE_D_S_THERMAL", label: Some("thermal"), kind: GeneratedNoiseKind::White, equation: 8, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(0), name: "d", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(2), name: "s", is_internal: false }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_D_S_FLICKER", label: Some("flicker"), kind: GeneratedNoiseKind::Flicker, equation: 8, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(0), name: "d", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(2), name: "s", is_internal: false }, table_len: 0, table_log_interp: false },
];

impl Instance {
    pub fn evaluate_noise_sources(&self, ctx: &GeneratedEvalContext<'_>, visitor: &mut dyn GeneratedNoiseVisitor) -> Result<(), GeneratedNoiseEvaluationError> {
        if !self.multiplicity.is_finite() || self.multiplicity <= 0.0 {
            return Err(GeneratedNoiseEvaluationError::InvalidMultiplicity { value: self.multiplicity });
        }
        let parameters = &self.params.values;
        let temperature = ctx.temperature();
        let node_potentials = [ctx.node_voltage(self.nodes[0]), ctx.node_voltage(self.nodes[1]), ctx.node_voltage(self.nodes[2]), ctx.node_voltage(self.nodes[3])];
            let v0 = 0e0f64;
            let v1 = 1.0359399871014713e-10f64;
            let v2 = parameters[13];
            let v4 = parameters[14];
            let v7 = parameters[25];
            let v9 = 3e0f64;
            let v11 = parameters[28];
            let v13 = parameters[29];
            let v15 = parameters[35];
            let v17 = parameters[22];
            let v20 = parameters[30];
            let v23 = parameters[0];
            let v25 = 5e-1f64;
            let v26 = 3.333333333333e-1f64;
            let v28 = parameters[3];
            let v29 = 1e21f64;
            let v31 = temperature;
            let v32 = parameters[2];
            let v34 = 2.7315e2f64;
            let v36 = parameters[4];
            let v37 = 1e21f64;
            let v39 = 2.9815e2f64;
            let v42 = 8.617333262e-5f64;
            let v44 = 1e-1f64;
            let v46 = 1e0f64;
            let v52 = 1.6e1f64;
            let v54 = 1.16e0f64;
            let v55 = 7.02e-4f64;
            let v58 = 1.108e3f64;
            let v70 = parameters[15];
            let v71 = parameters[16];
            let v74 = parameters[19];
            let v75 = parameters[20];
            let v78 = parameters[23];
            let v79 = parameters[24];
            let v82 = parameters[33];
            let v83 = parameters[34];
            let v87 = parameters[18];
            let v96 = 2e-1f64;
            let v108 = parameters[32];
            let v110 = parameters[5];
            let v111 = parameters[26];
            let v113 = parameters[6];
            let v114 = parameters[27];
            let v120 = 6e-1f64;
            let v126 = parameters[38];
            let v127 = 1e-6f64;
            let v139 = parameters[39];
            let v147 = parameters[40];
            let v149 = parameters[17];
            let v156 = 2.8e-1f64;
            let v157 = parameters[31];
            let v158 = parameters[8];
            let v164 = 1.936e-3f64;
            let v173 = node_potentials[1];
            let v174 = node_potentials[3];
            let v177 = node_potentials[2];
            let v180 = node_potentials[0];
            let v185 = -1e0f64;
            let v193 = 2e0f64;
            let v215 = parameters[7];
            let v220 = 2.5e-1f64;
            let v253 = -3.5e-1f64;
            let v255 = 1.3e0f64;
            let v257 = 1.6e0f64;
            let v271 = -1.5e1f64;
            let v273 = 1.55e0f64;
            let v286 = -2.3e1f64;
            let v293 = 1e-64f64;
            let v312 = 1.5625e-2f64;
            let v323 = 7.5e-1f64;
            let v345 = -3.5e-1f64;
            let v361 = -1.5e1f64;
            let v375 = -2.3e1f64;
            let v406 = -3.5e-1f64;
            let v422 = -1.5e1f64;
            let v436 = -2.3e1f64;
            let v465 = 6.6666666e-1f64;
            let v466 = 1.33333332e0f64;
            let v474 = -5e-1f64;
            let v485 = parameters[21];
            let v512 = 4e0f64;
            let v554 = 1.5e0f64;
            let v623 = 0e0f64;
            let v642 = parameters[37];
            let v651 = -3.5e1f64;
            let v655 = parameters[1];
            let v656 = 5.5224904e-23f64;
            let v659 = parameters[42];
            let v666 = parameters[41];
            let v667 = parameters[9];
            let v671 = parameters[11];
            let v674 = parameters[10];
            let v677 = parameters[12];
            let v680 = parameters[43];
            let v685 = -4e1f64;
            let v687 = parameters[58];
            let v691 = 7e1f64;
            let v696 = -4e1f64;
            let v3 = v1 / v2;
            let v6 = (v3 * v4).sqrt();
            let v8 = v6 * v7;
            let v12 = (v9 * v3) * v11;
            let v14 = v3 * v13;
            let v16 = v15 + v15;
            let v19 = v2 / (v1 * v17);
            let v22 = (v20 + v20) / v2;
            let v24 = if v23 > v0 { 1.0 } else { 0.0 };
            let v27: f64;
            if v24 != 0.0 {
                v27 = v25;
            } else {
                v27 = v26;
            }
            let v30 = if v28 == v29 { 1.0 } else { 0.0 };
            let v41: f64;
            if v30 != 0.0 {
                let v33 = v31 + v32;
                v41 = v33;
            } else {
                let v35 = v28 + v34;
                v41 = v35;
            }
            let v38 = if v36 == v37 { 1.0 } else { 0.0 };
            let v62: f64;
            if v38 != 0.0 {
                v62 = v39;
            } else {
                let v40 = v36 + v34;
                v62 = v40;
            }
            let v43 = v41 * v42;
            let v45 = v44 * v43;
            let v47 = v46 / v43;
            let v48 = v43 + v43;
            let v49 = v48 + v48;
            let v50 = v43 * v43;
            let v51 = v50 + v50;
            let v53 = v52 * v50;
            let v68 = v41 - v62;
            let v69 = v41 / v62;
            let v73 = v70 - (v71 * v68);
            let v77 = v74 * (v69.powf(v75));
            let v81 = v78 * (v69.powf(v79));
            let v86 = v82 * (v46 + (v83 * v68));
            let v97 = ((((v87 * v69) - ((v9 * v43) * (v69.ln()))) - ((v54 - (((v55 * v62) * v62) / (v62 + v58))) * v69)) + (v54 - (((v55 * v41) * v41) / (v41 + v58)))) - v96;
            let v103 = (v25 * (v97 + (((v97 * v97) + v50).sqrt()))) + v96;
            let v104 = v103.sqrt();
            let v105 = v46 / v81;
            let v106 = v6 * v81;
            let v107 = v6 * v86;
            let v109 = v108 / v86;
            let v112 = v110 + v111;
            let v115 = v113 + v114;
            let v116 = v81 * v112;
            let v122 = v43 * ((((v25 * v116) * v47).ln()) - v120);
            let v125 = v46 / ((v115 * v112).sqrt());
            let v186: f64;
            if v24 != 0.0 {
                let v128 = if v126 != v127 { 1.0 } else { 0.0 };
                let v132: f64;
                if v128 != 0.0 {
                    let v131 = (v125 * (v126 - v127)) + v73;
                    v132 = v131;
                } else {
                    v132 = v73;
                }
                v186 = v132;
            } else {
                let v133 = if v126 != v127 { 1.0 } else { 0.0 };
                let v138: f64;
                if v133 != 0.0 {
                    let v136 = (v125 * (v127 - v126)) - v73;
                    v138 = v136;
                } else {
                    let v137 = -v73;
                    v138 = v137;
                }
                v186 = v138;
            }
            let v140 = if v139 != v127 { 1.0 } else { 0.0 };
            let v145: f64;
            if v140 != 0.0 {
                let v144 = v77 * (v46 + ((v139 - v127) * v125));
                v145 = v144;
            } else {
                v145 = v77;
            }
            let v146 = v115 * v145;
            let v148 = if v147 != v127 { 1.0 } else { 0.0 };
            let v153: f64;
            if v148 != 0.0 {
                let v152 = v149 + ((v147 - v127) * v125);
                v153 = v152;
            } else {
                v153 = v149;
            }
            let v154 = v153 * v104;
            let v155 = if v22 == v0 { 1.0 } else { 0.0 };
            let v188: f64;
            if v155 != 0.0 {
                v188 = v0;
            } else {
                let v162 = v156 * ((v112 / (v157 * v158)) - v44);
                let v170 = v46 / (v46 + (v25 * (v162 + (((v162 * v162) + v164).sqrt()))));
                let v172 = (v22 * v170) * v170;
                v188 = v172;
            }
            let v176 = v23 * (v173 - v174);
            let v179 = v23 * (v177 - v174);
            let v182 = v23 * (v180 - v174);
            let v184 = if (v182 - v179) < v0 { 1.0 } else { 0.0 };
            let v199: f64;
            let v207: f64;
            let v653: f64;
            if v184 != 0.0 {
                v199 = v182;
                v207 = v179;
                v653 = v185;
            } else {
                v199 = v179;
                v207 = v182;
                v653 = v46;
            }
            let v191 = (((v176 - v186) - v188) + v103) + v154;
            let v196 = ((v191 * v191) + (v193 * v53)).sqrt();
            let v198 = v25 * (v191 + v196);
            let v200 = v103 + v199;
            let v208 = v103 + v207;
            let v217 = (v12 * v215) / v115;
            let v224 = (v198 + ((v220 * v153) * v153)).sqrt();
            let v225 = v198 - v103;
            let v227 = v224 - (v25 * v153);
            let v232 = (((v225 - (v153 * v227)) + v103) + v45).sqrt();
            let v237 = (v153 - (((v14 * v158) / v112) * (((v25 * (v200 + (((v200 * v200) + v53).sqrt()))).sqrt()) + ((v25 * (v208 + (((v208 * v208) + v53).sqrt()))).sqrt())))) + (v217 * v232);
            let v240 = ((v237 * v237) + v45).sqrt();
            let v242 = v25 * (v237 + v240);
            let v246 = (v198 + ((v220 * v242) * v242)).sqrt();
            let v250 = v225 - (v242 * (v246 - (v25 * v242)));
            let v252 = (v250 - v199) * v47;
            let v254 = if v252 > v253 { 1.0 } else { 0.0 };
            let v295: f64;
            if v254 != 0.0 {
                let v261 = v193 / ((v255 + v252) - ((v252 + v257).ln()));
                let v263 = v46 + v252;
                let v266 = (v193 + v261) / (v263 + (v261.ln()));
                let v270 = (v263 + (v266.ln())) / (v193 + v266);
                v295 = v270;
            } else {
                let v272 = if v252 > v271 { 1.0 } else { 0.0 };
                let v296: f64;
                if v272 != 0.0 {
                    let v276 = v273 + ((-v252).exp());
                    let v278 = v46 + v252;
                    let v281 = (v193 + v276) / (v278 + (v276.ln()));
                    let v285 = (v278 + (v281.ln())) / (v193 + v281);
                    v296 = v285;
                } else {
                    let v287 = if v252 > v286 { 1.0 } else { 0.0 };
                    let v297: f64;
                    if v287 != 0.0 {
                        let v291 = v46 / (v193 + ((-v252).exp()));
                        v297 = v291;
                    } else {
                        let v294 = (v252.exp()) + v293;
                        v297 = v294;
                    }
                    v296 = v297;
                }
                v295 = v296;
            }
            let v299 = v295 * (v46 + v295);
            let v300 = v299.sqrt();
            let v301 = v43 / v116;
            let v304 = (v220 + (v300 * v301)).sqrt();
            let v306 = v116 * (v304 - v25);
            let v307 = v207 - v199;
            let v308 = v25 * v307;
            let v314 = v53 * ((v7 * (v300 - (v306 * v47))) + v312);
            let v317 = ((v306 * v306) + v314).sqrt();
            let v318 = v308 - v306;
            let v321 = ((v318 * v318) + v314).sqrt();
            let v322 = v317 - v321;
            let v329 = (v220 + ((v300 - (v323 * (v299.ln()))) * v301)).sqrt();
            let v332 = (v116 * (v329 - v25)) + v122;
            let v333 = v308 - v332;
            let v336 = ((v332 * v332) + v314).sqrt();
            let v339 = ((v333 * v333) + v314).sqrt();
            let v344 = ((((v250 - v308) - v199) - v336) + v339) * v47;
            let v346 = if v344 > v345 { 1.0 } else { 0.0 };
            let v383: f64;
            if v346 != 0.0 {
                let v351 = v193 / ((v255 + v344) - ((v344 + v257).ln()));
                let v353 = v46 + v344;
                let v356 = (v193 + v351) / (v353 + (v351.ln()));
                let v360 = (v353 + (v356.ln())) / (v193 + v356);
                v383 = v360;
            } else {
                let v362 = if v344 > v361 { 1.0 } else { 0.0 };
                let v384: f64;
                if v362 != 0.0 {
                    let v365 = v273 + ((-v344).exp());
                    let v367 = v46 + v344;
                    let v370 = (v193 + v365) / (v367 + (v365.ln()));
                    let v374 = (v367 + (v370.ln())) / (v193 + v370);
                    v384 = v374;
                } else {
                    let v376 = if v344 > v375 { 1.0 } else { 0.0 };
                    let v385: f64;
                    if v376 != 0.0 {
                        let v380 = v46 / (v193 + ((-v344).exp()));
                        v385 = v380;
                    } else {
                        let v382 = (v344.exp()) + v293;
                        v385 = v382;
                    }
                    v384 = v385;
                }
                v383 = v384;
            }
            let v387 = v383 * (v46 + v383);
            let v396 = (v112 - (v8 * ((v46 + ((v308 - v322) / v106)).ln()))) + ((v308 + v322) * v105);
            let v397 = v44 * v112;
            let v401 = ((v396 * v396) + (v397 * v397)).sqrt();
            let v403 = v25 * (v396 + v401);
            let v405 = (v250 - v207) * v47;
            let v407 = if v405 > v406 { 1.0 } else { 0.0 };
            let v444: f64;
            if v407 != 0.0 {
                let v412 = v193 / ((v255 + v405) - ((v405 + v257).ln()));
                let v414 = v46 + v405;
                let v417 = (v193 + v412) / (v414 + (v412.ln()));
                let v421 = (v414 + (v417.ln())) / (v193 + v417);
                v444 = v421;
            } else {
                let v423 = if v405 > v422 { 1.0 } else { 0.0 };
                let v445: f64;
                if v423 != 0.0 {
                    let v426 = v273 + ((-v405).exp());
                    let v428 = v46 + v405;
                    let v431 = (v193 + v426) / (v428 + (v426.ln()));
                    let v435 = (v428 + (v431.ln())) / (v193 + v431);
                    v445 = v435;
                } else {
                    let v437 = if v405 > v436 { 1.0 } else { 0.0 };
                    let v446: f64;
                    if v437 != 0.0 {
                        let v441 = v46 / (v193 + ((-v405).exp()));
                        v446 = v441;
                    } else {
                        let v443 = (v405.exp()) + v293;
                        v446 = v443;
                    }
                    v445 = v446;
                }
                v444 = v445;
            }
            let v449 = v220 + v299;
            let v450 = v220 + (v444 * (v46 + v444));
            let v451 = v449.sqrt();
            let v452 = v450.sqrt();
            let v453 = v451 + v452;
            let v454 = v453 * v453;
            let v455 = v250 + v103;
            let v456 = v455 + v127;
            let v458 = v193 * (v456.sqrt());
            let v459 = v153 / v458;
            let v461 = v153 / (v458 + v153);
            let v462 = v46 + v459;
            let v464 = (-v462) * v43;
            let v473 = v464 * (((v466 * ((v450 + (v452 * v451)) + v449)) / v453) - v46);
            let v478 = ((v474 * v153) * v458) - (v461 * v473);
            let v479 = if v17 == v0 { 1.0 } else { 0.0 };
            let v510: f64;
            let v613: f64;
            let v615: f64;
            let v616: f64;
            let v622: f64;
            if v479 != 0.0 {
                let v482 = ((v250 * v250) + v51).sqrt();
                let v484 = v25 * (v250 + v482);
                let v487 = v46 + (v485 * v484);
                let v489 = v146 / (v403 * v487);
                v510 = v489;
                v613 = v484;
                v615 = v487;
                v616 = v482;
                v622 = v623;
            } else {
                let v491 = v478 + (v27 * v473);
                let v492 = if v491 > v0 { 1.0 } else { 0.0 };
                let v500: f64;
                if v492 != 0.0 {
                    let v494 = v46 + (v19 * v491);
                    v500 = v494;
                } else {
                    let v496 = v46 - (v19 * v491);
                    v500 = v496;
                }
                let v502 = (v146 * (v46 + (v19 * v154))) / (v403 * v500);
                v510 = v502;
                v613 = v0;
                v615 = v0;
                v616 = v0;
                v622 = v500;
            }
            let v503 = v455 + v49;
            let v504 = v503.sqrt();
            let v507 = v46 + (v153 / (v193 * v504));
            let v508 = v299 - v387;
            let v511 = (v51 * v507) * v510;
            let v514 = v510 * (v473.abs());
            let v517 = v198 / v196;
            let v530 = ((-(v455 / v246)) * ((((v217 * (v242 / (v240 + v240))) * v227) / (v224 * v232)) * v517)) + ((v46 - (v242 / (v246 + v246))) * v517);
            let v532 = (v295 * v47) * v530;
            let v536 = (v43 / ((v512 * v304) * v300)) * v532;
            let v543 = ((v49 + v49) * v7) * ((v532 * (v43 / (v300 + v300))) - v536);
            let v553 = (((v306 * v536) + v543) * (v46 / v317)) - (((v318 * (-v536)) + v543) * (v46 / v321));
            let v560 = ((v43 * (v300 - v554)) / ((v512 * v329) * v299)) * v532;
            let v573 = (v383 * v47) * ((v530 - (((v332 * v560) + v543) * (v46 / v336))) + (((v333 * (-v560)) + v543) * (v46 / v339)));
            let v583 = (v46 / v401) * ((-((-(v8 / ((v106 + v308) - v322))) * v553)) + (v553 * v105));
            let v587 = (v464 * v465) / v454;
            let v604 = (((((-v459) * v473) / (((v193 + v459) + v459) * v456)) * v530) + ((v587 * (v451 + (v193 * v452))) * v532)) + ((v587 * (v452 + (v193 * v451))) * ((v444 * v47) * v530));
            let v612 = (-v461) * (((v462 - (v473 / ((v193 * v462) * v456))) * v530) + v604);
            let v636: f64;
            if v479 != 0.0 {
                let v621 = (-v583) - (((v485 * v613) / (v615 * v616)) * v530);
                v636 = v621;
            } else {
                let v629 = (-v583) + ((v19 / v622) * (v612 + (v27 * v604)));
                v636 = v629;
            }
            let v641 = v511 * (((((((-v153) / (((v512 * v507) * v504) * v503)) * v530) + v636) * v508) + v532) - v573);
            let v644 = v307 - (v16 * v306);
            let v647 = if (if v644 > v0 { 1.0 } else { 0.0 }) != 0.0 && (if v109 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            if v647 != 0.0 {
                let v652 = if ((-v107) * (v46 / v644)) < v651 { 1.0 } else { 0.0 };
                if v652 != 0.0 {
                } else {
                }
            } else {
            }
            let v654 = if v653 == v46 { 1.0 } else { 0.0 };
            if v654 != 0.0 {
            } else {
            }
            let v704: f64;
            let v705: f64;
            let v706: f64;
            let v707: f64;
            let v708: f64;
            if v655 != 0.0 {
                let v658 = (v656 * v41) * v514;
                let v665 = ((v659 * v641) * v641) / (((v115 * v158) * v112) * v2);
                v704 = v46;
                v705 = v658;
                v706 = v46;
                v707 = v665;
                v708 = v666;
            } else {
                v704 = v0;
                v705 = v0;
                v706 = v0;
                v707 = v0;
                v708 = v0;
            }
            let v669 = if v642 > v0 { 1.0 } else { 0.0 };
            let v670 = if (if v667 == v0 { 1.0 } else { 0.0 }) != 0.0 && v669 != 0.0 { 1.0 } else { 0.0 };
            if v670 != 0.0 {
            } else {
            }
            let v673 = if (if v671 == v0 { 1.0 } else { 0.0 }) != 0.0 && v669 != 0.0 { 1.0 } else { 0.0 };
            if v673 != 0.0 {
            } else {
            }
            let v676 = if (if v674 == v0 { 1.0 } else { 0.0 }) != 0.0 && v669 != 0.0 { 1.0 } else { 0.0 };
            if v676 != 0.0 {
            } else {
            }
            let v679 = if (if v677 == v0 { 1.0 } else { 0.0 }) != 0.0 && v669 != 0.0 { 1.0 } else { 0.0 };
            if v679 != 0.0 {
            } else {
            }
            let v681 = -v182;
            let v683 = v43 * v680;
            let v686 = if ((v681 * v69) / v683) < v685 { 1.0 } else { 0.0 };
            if v686 != 0.0 {
            } else {
            }
            let v692 = if (((v681 + v687) * v69) / v683) > v691 { 1.0 } else { 0.0 };
            if v692 != 0.0 {
            } else {
            }
            let v693 = -v179;
            let v697 = if ((v693 * v69) / v683) < v696 { 1.0 } else { 0.0 };
            if v697 != 0.0 {
            } else {
            }
            let v701 = if (((v693 + v687) * v69) / v683) > v691 { 1.0 } else { 0.0 };
            if v701 != 0.0 {
            } else {
            }
            let v702 = if v182 > v0 { 1.0 } else { 0.0 };
            if v702 != 0.0 {
            } else {
            }
            let v703 = if v179 > v0 { 1.0 } else { 0.0 };
            if v703 != 0.0 {
            } else {
            }
        if v704 == 0.0 {
            if !visitor.visit(0, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v705;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 0, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 0, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 0, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(0, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v706 == 0.0 {
            if !visitor.visit(1, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v707;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 1, value: psd }); }
            let exponent: Option<f64> = Some(v708);
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(1, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        Ok(())
    }
}
