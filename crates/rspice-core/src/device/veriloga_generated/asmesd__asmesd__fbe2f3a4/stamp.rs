#![allow(dead_code, unused_imports, unused_parens, unused_variables)]

use super::state::Instance;
use crate::device::veriloga_generated::{GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper};

#[inline]
fn eval_ddt<const STATE_COUNT: usize>(
    current: &mut [f64; STATE_COUNT],
    previous: &mut [f64; STATE_COUNT],
    older: &mut [f64; STATE_COUNT],
    initialized: &mut [bool; STATE_COUNT],
    derivative_current: &mut [f64; STATE_COUNT],
    derivative_previous: &mut [f64; STATE_COUNT],
    ddt_active: bool,
    ddt_scale: f64,
    ddt_previous_value_scale: f64,
    ddt_older_value_scale: f64,
    ddt_previous_derivative_scale: f64,
    slot: usize,
    value: f64,
) -> f64 {
    debug_assert!(slot < STATE_COUNT, "generated ddt state slot out of range");
    let previous_value = if initialized[slot] { previous[slot] } else { value };
    let older_value = if initialized[slot] { older[slot] } else { value };
    current[slot] = value;
    if ddt_active {
        let result = value * ddt_scale
            - previous_value * ddt_previous_value_scale
            - older_value * ddt_older_value_scale
            - derivative_previous[slot] * ddt_previous_derivative_scale;
        derivative_current[slot] = result;
        result
    } else {
        current[slot] = value;
        previous[slot] = value;
        older[slot] = value;
        derivative_current[slot] = 0.0;
        derivative_previous[slot] = 0.0;
        initialized[slot] = true;
        0.0
    }
}

impl Instance {
    pub fn stamp(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedStamper<'_>) {
        let nodes = self.nodes;
        let multiplicity = self.multiplicity;
        let timestep = self.timestep;
        let ddt_state_current = self.ddt_state_current.as_mut();
        let ddt_state_previous = self.ddt_state_previous.as_mut();
        let ddt_state_older = self.ddt_state_older.as_mut();
        let ddt_state_initialized = self.ddt_state_initialized.as_mut();
        let ddt_derivative_current = self.ddt_derivative_current.as_mut();
        let ddt_derivative_previous = self.ddt_derivative_previous.as_mut();
        let ddt_active = self.ddt_coefficients.active;
        let ddt_scale = self.ddt_coefficients.derivative_scale;
        let ddt_previous_value_scale = self.ddt_coefficients.previous_value_scale;
        let ddt_older_value_scale = self.ddt_coefficients.older_value_scale;
        let ddt_previous_derivative_scale = self.ddt_coefficients.previous_derivative_scale;
        let v1: f64 = ctx.node_voltage(nodes[3]);
        let v4: f64 = ((ctx.temperature() + v1) + self.scalar_v3);
        let v6: f64 = 1300.0;
        let v7: f64 = 173.14999999999998;
        let v8: bool = (v4 > v7);
        let v9: f64 = (if v8 { v4 } else { v7 });
        let v10: bool = (v6 < v9);
        let v11: f64 = (if v10 { v6 } else { v9 });
        let v12: f64 = 1.0;
        let v13: f64 = 0.0;
        let v18: f64 = ctx.node_voltage(nodes[5]);
        let v19: f64 = ctx.node_voltage(nodes[4]);
        let v20: f64 = (v18 - v19);
        let v21: f64 = (self.scalar_v17 * v20);
        let v23: bool = (v21 < v13);
        let v25: f64 = (-(if v23 { v21 } else { v13 }));
        let v29: f64 = (v12 + (self.scalar_v22 * f64::powf(v25, self.scalar_v26)));
        let v32: f64 = 8.6170869e-5;
        let v33: f64 = (v11 * v32);
        let v34: f64 = (v11 / self.scalar_v31);
        let v35: f64 = ((v34) as f64).ln();
        let v38: f64 = (((v35 * self.scalar_v36)) as f64).exp();
        let v40: f64 = (v38 * self.scalar_v39);
        let v41: f64 = (v29 * v40);
        let v43: f64 = (v38 * self.scalar_v42);
        let v63: f64 = (v34 - v12);
        let v64: f64 = (self.scalar_v62 * v63);
        let v66: f64 = ((v35 * self.scalar_v60) + (v64 / v33));
        let v70: f64 = ((v66) as f64).exp();
        let v71: f64 = (self.scalar_v69 * v70);
        let v73: f64 = (((v35 * self.scalar_v67)) as f64).exp();
        let v74: f64 = (self.scalar_v72 * v73);
        let v78: f64 = (((v66 / self.scalar_v76)) as f64).exp();
        let v79: f64 = (self.scalar_v75 * v78);
        let v80: f64 = (v79 / v38);
        let v84: f64 = (((v66 / self.scalar_v82)) as f64).exp();
        let v85: f64 = (self.scalar_v81 * v84);
        let v86: f64 = (v85 / v38);
        let v91: f64 = (self.scalar_v87 * (v12 + (v63 * self.scalar_v88)));
        let v96: f64 = (self.scalar_v92 * (v12 + (v63 * self.scalar_v93)));
        let v101: f64 = (self.scalar_v97 * (v12 + (v63 * self.scalar_v98)));
        let v106: f64 = (self.scalar_v102 * (v12 + (v63 * self.scalar_v103)));
        let v110: f64 = 300.15;
        let v112: f64 = (v11 / v110);
        let v114: f64 = 0.000702;
        let v115: f64 = (v11 * v114);
        let v116: f64 = (v11 * v115);
        let v118: f64 = (v11 + 1108.0);
        let v121: f64 = (-(1.16 - (v116 / v118)));
        let v122: f64 = 1.3806226e-23;
        let v124: f64 = (v122 * (v11 + v11));
        let v129: f64 = (-(v33 + v33));
        let v130: f64 = 1.5;
        let v133: f64 = 1.6021918e-19;
        let v135: f64 = ((v130 * ((v112) as f64).ln()) + (((v121 / v124) + 1.3454442398941469e20) * v133));
        let v136: f64 = (v129 * v135);
        let v139: f64 = ((self.scalar_v137 - v136) / self.scalar_v111);
        let v140: f64 = (self.scalar_v137 - v139);
        let v143: f64 = 0.0004;
        let v148: f64 = (v12 + (self.scalar_v142 * (self.scalar_v145 - (v140 / v139))));
        let v149: f64 = (self.scalar_v107 / v148);
        let v151: f64 = (v136 + (v112 * v139));
        let v152: f64 = (v151 - v139);
        let v155: f64 = (v143 * (v11 - v110));
        let v158: f64 = (v12 + (self.scalar_v142 * (v155 - (v152 / v139))));
        let v159: f64 = (v149 * v158);
        let v162: f64 = ((self.scalar_v160 - v136) / self.scalar_v111);
        let v163: f64 = (self.scalar_v160 - v162);
        let v168: f64 = (v12 + (self.scalar_v165 * (self.scalar_v145 - (v163 / v162))));
        let v169: f64 = (self.scalar_v108 / v168);
        let v171: f64 = (v136 + (v112 * v162));
        let v172: f64 = (v171 - v162);
        let v176: f64 = (v12 + (self.scalar_v165 * (v155 - (v172 / v162))));
        let v177: f64 = (v169 * v176);
        let v180: f64 = ((self.scalar_v178 - v136) / self.scalar_v111);
        let v181: f64 = (self.scalar_v178 - v180);
        let v186: f64 = (v12 + (self.scalar_v183 * (self.scalar_v145 - (v181 / v180))));
        let v187: f64 = (self.scalar_v109 / v186);
        let v189: f64 = (v136 + (v112 * v180));
        let v190: f64 = (v189 - v180);
        let v194: f64 = (v12 + (self.scalar_v183 * (v155 - (v190 / v180))));
        let v195: f64 = (v187 * v194);
        let v196: f64 = ctx.node_voltage(nodes[2]);
        let v198: f64 = (self.scalar_v17 * (v196 - v19));
        let v199: f64 = ctx.node_voltage(nodes[6]);
        let v200: f64 = (v18 - v199);
        let v201: f64 = (self.scalar_v17 * v200);
        let v202: f64 = ctx.node_voltage(nodes[1]);
        let v204: f64 = (self.scalar_v17 * (v202 - v19));
        let v205: f64 = (v202 - v18);
        let v207: f64 = (v196 - v199);
        let v209: bool = (v71 > v13);
        let v211: f64 = (v33 * self.scalar_v210);
        let v213: f64 = (if v209 { (v201 / v211) } else { v13 });
        let v214: f64 = (-v201);
        let v215: f64 = (v214 - v96);
        let v217: f64 = (v33 * self.scalar_v216);
        let v219: f64 = (if v209 { (v215 / v217) } else { v13 });
        let v220: f64 = (-v96);
        let v222: f64 = (if v209 { (v220 / v217) } else { v13 });
        let v223: f64 = 80.0;
        let v224: bool = (v213 > v223);
        let v225: bool = (v209 && v224);
        let v229: f64 = (if v225 { v223 } else { v213 });
        let v231: bool = (v209 && (!v224));
        let v232: f64 = (if v231 { v12 } else { (if v225 { (v12 + (v213 - v223)) } else { v13 }) });
        let v233: f64 = ((v229) as f64).exp();
        let v235: f64 = (if v209 { (v232 * v233) } else { v232 });
        let v236: f64 = 37.0;
        let v237: bool = (v219 >= v236);
        let v238: bool = (!v237);
        let v239: f64 = -37.0;
        let v240: bool = (v219 <= v239);
        let v242: bool = (v238 && (!v240));
        let v243: f64 = ((v219) as f64).exp();
        let v244: f64 = (v12 + v243);
        let v246: bool = (v238 && v240);
        let v250: bool = (v222 >= v236);
        let v251: bool = (!v250);
        let v252: bool = (v222 <= v239);
        let v254: bool = (v251 && (!v252));
        let v255: f64 = ((v222) as f64).exp();
        let v256: f64 = (v12 + v255);
        let v258: bool = (v251 && v252);
        let v263: f64 = (if v209 { ((if v242 { ((v244) as f64).ln() } else { (if v246 { v243 } else { (if v237 { v219 } else { v13 }) }) }) - (if v254 { ((v256) as f64).ln() } else { (if v258 { v255 } else { (if v250 { v222 } else { v13 }) }) })) } else { v13 });
        let v264: f64 = (v235 - v12);
        let v266: f64 = (v91 * v263);
        let v268: f64 = ((v201) as f64).abs();
        let v269: f64 = f64::powf(v268, v101);
        let v271: f64 = (v12 + (self.scalar_v267 * v269));
        let v275: bool = (!v209);
        let v276: f64 = (if v275 { v13 } else { (if v209 { ((v71 * v264) - (v266 / v271)) } else { v13 }) });
        let v277: bool = (v74 > v13);
        let v279: f64 = (self.scalar_v278 - v201);
        let v280: f64 = 0.001;
        let v281: bool = (v279 > v280);
        let v283: f64 = (if v277 { (if v281 { v279 } else { v280 }) } else { v13 });
        let v284: f64 = -1.0;
        let v285: f64 = (v214 * self.scalar_v278);
        let v287: f64 = (v33 * self.scalar_v286);
        let v288: f64 = (v283 * v287);
        let v290: f64 = (if v277 { (v285 / v288) } else { v229 });
        let v291: bool = (v290 > v223);
        let v292: bool = (v277 && v291);
        let v296: f64 = (if v292 { v223 } else { v290 });
        let v298: bool = (v277 && (!v291));
        let v299: f64 = (if v298 { v12 } else { (if v292 { (v12 + (v290 - v223)) } else { v235 }) });
        let v300: f64 = ((v296) as f64).exp();
        let v302: f64 = (if v277 { (v299 * v300) } else { v299 });
        let v303: f64 = (v302 - v12);
        let v306: bool = (!v277);
        let v308: bool = (v80 > v13);
        let v309: f64 = (v33 * self.scalar_v76);
        let v311: f64 = (if v308 { (v201 / v309) } else { v296 });
        let v313: f64 = (v33 * self.scalar_v312);
        let v315: f64 = (if v308 { (v215 / v313) } else { v219 });
        let v316: f64 = (v220 / v313);
        let v317: f64 = (if v308 { v316 } else { v222 });
        let v318: bool = (v311 > v223);
        let v319: bool = (v308 && v318);
        let v323: f64 = (if v319 { v223 } else { v311 });
        let v325: bool = (v308 && (!v318));
        let v326: f64 = (if v325 { v12 } else { (if v319 { (v12 + (v311 - v223)) } else { v302 }) });
        let v327: f64 = ((v323) as f64).exp();
        let v329: f64 = (if v308 { (v326 * v327) } else { v326 });
        let v330: bool = (v315 >= v236);
        let v331: bool = (!v330);
        let v332: bool = (v315 <= v239);
        let v334: bool = (v331 && (!v332));
        let v335: f64 = ((v315) as f64).exp();
        let v336: f64 = (v12 + v335);
        let v338: bool = (v331 && v332);
        let v342: bool = (v317 >= v236);
        let v343: bool = (!v342);
        let v344: bool = (v317 <= v239);
        let v346: bool = (v343 && (!v344));
        let v347: f64 = ((v317) as f64).exp();
        let v348: f64 = (v12 + v347);
        let v350: bool = (v343 && v344);
        let v355: f64 = (if v308 { ((if v334 { ((v336) as f64).ln() } else { (if v338 { v335 } else { (if v330 { v315 } else { v13 }) }) }) - (if v346 { ((v348) as f64).ln() } else { (if v350 { v347 } else { (if v342 { v317 } else { v13 }) }) })) } else { v263 });
        let v356: f64 = (v329 - v12);
        let v358: f64 = (v13 * v355);
        let v362: bool = (!v308);
        let v365: f64 = (v33 * self.scalar_v364);
        let v367: f64 = (if v209 { (v21 / v365) } else { v323 });
        let v369: f64 = ((-v21) - v96);
        let v370: f64 = (v369 / v313);
        let v371: f64 = (if v209 { v370 } else { v315 });
        let v372: f64 = (if v209 { v316 } else { v317 });
        let v373: bool = (v367 > v223);
        let v374: bool = (v209 && v373);
        let v378: f64 = (if v374 { v223 } else { v367 });
        let v380: bool = (v209 && (!v373));
        let v381: f64 = (if v380 { v12 } else { (if v374 { (v12 + (v367 - v223)) } else { v329 }) });
        let v382: f64 = ((v378) as f64).exp();
        let v384: f64 = (if v209 { (v381 * v382) } else { v381 });
        let v385: bool = (v371 >= v236);
        let v386: bool = (!v385);
        let v387: bool = (v371 <= v239);
        let v389: bool = (v386 && (!v387));
        let v390: f64 = ((v371) as f64).exp();
        let v391: f64 = (v12 + v390);
        let v393: bool = (v386 && v387);
        let v397: bool = (v372 >= v236);
        let v398: bool = (!v397);
        let v399: bool = (v372 <= v239);
        let v401: bool = (v398 && (!v399));
        let v402: f64 = ((v372) as f64).exp();
        let v403: f64 = (v12 + v402);
        let v405: bool = (v398 && v399);
        let v410: f64 = (if v209 { ((if v389 { ((v391) as f64).ln() } else { (if v393 { v390 } else { (if v385 { v371 } else { v13 }) }) }) - (if v401 { ((v403) as f64).ln() } else { (if v405 { v402 } else { (if v397 { v372 } else { v13 }) }) })) } else { v355 });
        let v411: f64 = (v384 - v12);
        let v413: f64 = (v106 * v410);
        let v414: f64 = ((v21) as f64).abs();
        let v415: f64 = f64::powf(v414, v101);
        let v417: f64 = (v12 + (self.scalar_v267 * v415));
        let v421: f64 = (if v275 { v13 } else { (if v209 { ((v71 * v411) - (v413 / v417)) } else { v13 }) });
        let v422: bool = (v86 > v13);
        let v423: f64 = (v33 * self.scalar_v82);
        let v425: f64 = (if v422 { (v21 / v423) } else { v378 });
        let v426: f64 = (if v422 { v370 } else { v371 });
        let v427: f64 = (if v422 { v316 } else { v372 });
        let v428: bool = (v425 > v223);
        let v429: bool = (v422 && v428);
        let v435: bool = (v422 && (!v428));
        let v436: f64 = (if v435 { v12 } else { (if v429 { (v12 + (v425 - v223)) } else { v384 }) });
        let v437: f64 = (((if v429 { v223 } else { v425 })) as f64).exp();
        let v440: bool = (v426 >= v236);
        let v441: bool = (!v440);
        let v442: bool = (v426 <= v239);
        let v444: bool = (v441 && (!v442));
        let v445: f64 = ((v426) as f64).exp();
        let v446: f64 = (v12 + v445);
        let v448: bool = (v441 && v442);
        let v452: bool = (v427 >= v236);
        let v453: bool = (!v452);
        let v454: bool = (v427 <= v239);
        let v456: bool = (v453 && (!v454));
        let v457: f64 = ((v427) as f64).exp();
        let v458: f64 = (v12 + v457);
        let v460: bool = (v453 && v454);
        let v465: f64 = (if v422 { ((if v444 { ((v446) as f64).ln() } else { (if v448 { v445 } else { (if v440 { v426 } else { v13 }) }) }) - (if v456 { ((v458) as f64).ln() } else { (if v460 { v457 } else { (if v452 { v427 } else { v13 }) }) })) } else { v410 });
        let v466: f64 = ((if v422 { (v436 * v437) } else { v436 }) - v12);
        let v471: f64 = (v12 + (self.scalar_v267 * f64::powf(v414, self.scalar_v97)));
        let v475: bool = (!v422);
        let v477: f64 = ctx.node_voltage(nodes[9]);
        let v480: f64 = 1e-9;
        let v484: f64 = ((((if (v477 < v201) { v477 } else { v201 }) / (if (v268 > v480) { v268 } else { v480 }))) as f64).abs();
        let v485: f64 = (v276 - (if v306 { v13 } else { (if v277 { (v74 * v303) } else { v13 }) }));
        let v487: f64 = ((if v362 { v13 } else { (if v308 { ((v80 * v356) - (v358 / v271)) } else { v13 }) }) + (v485 / v41));
        let v489: f64 = ((if v475 { v13 } else { (if v422 { ((v86 * v466) - ((v13 * v465) / v471)) } else { v13 }) }) + (v421 / v43));
        let v507: f64 = (v12 + f64::powf((((v12 + (((v276 * (self.scalar_v55 * (v12 + (v21 * self.scalar_v490)))) + (self.scalar_v59 * v421)) * 4.0))) as f64).abs(), self.scalar_v505));
        let v510: f64 = ((((v12 - (self.scalar_v51 * v201)) - (v21 * self.scalar_v47)) * 2.0) / v507);
        let v511: f64 = (v421 * v510);
        let v512: f64 = (v276 * v510);
        let v517: f64 = (v276 * self.scalar_v516);
        let v535: f64 = (((v35 * self.scalar_v533)) as f64).exp();
        let v538: f64 = f64::powf((v12 + f64::powf(((((self.scalar_v17 * v205) / self.scalar_v520)) as f64).abs(), self.scalar_v523)), self.scalar_v537);
        let v539: f64 = ((self.scalar_v532 * v535) * v538);
        let v543: f64 = (((v35 * self.scalar_v541)) as f64).exp();
        let v544: f64 = (self.scalar_v540 * v543);
        let v548: f64 = (((v35 * self.scalar_v546)) as f64).exp();
        let v551: f64 = f64::powf((v12 + f64::powf(((((self.scalar_v17 * v207) / self.scalar_v526)) as f64).abs(), self.scalar_v529)), self.scalar_v550);
        let v552: f64 = ((self.scalar_v545 * v548) * v551);
        let v553: f64 = (v202 - v196);
        let v567: f64 = (self.scalar_v563 * (v12 + ((f64::powf((v12 + f64::powf((((v553 / self.scalar_v554)) as f64).abs(), self.scalar_v557)), self.scalar_v560) - v12) * self.scalar_v564)));
        let v573: f64 = ctx.node_voltage(nodes[8]);
        let v579: f64 = (v12 + f64::powf((((v573) as f64).abs() / self.scalar_v575), self.scalar_v577));
        let v581: f64 = (if self.scalar_v572 { (v539 / v579) } else { v539 });
        let v593: bool = (v198 <= v13);
        let v594: f64 = (v189 * v195);
        let v597: f64 = (v12 - (v198 / v189));
        let v600: f64 = (((self.scalar_v595 * ((v597) as f64).ln())) as f64).exp();
        let v601: f64 = (v12 - v600);
        let v605: bool = (!v593);
        let v606: f64 = (v195 * v198);
        let v609: f64 = (v198 * self.scalar_v608);
        let v611: f64 = (v12 + (v609 / v189));
        let v617: f64 = (v201 + ((-v151) * self.scalar_v615));
        let v618: bool = (v617 > v13);
        let v624: f64 = (if v618 { self.scalar_v623 } else { v13 });
        let v627: f64 = (v12 - (self.scalar_v620 * (self.scalar_v620 * v624)));
        let v633: f64 = (v617 * self.scalar_v632);
        let v635: f64 = (self.scalar_v620 + (v633 / v151));
        let v639: bool = (!v618);
        let v641: f64 = (v12 - (v201 / v151));
        let v644: f64 = (((self.scalar_v629 * ((v641) as f64).ln())) as f64).exp();
        let v645: f64 = (v12 - v644);
        let v648: f64 = (if v639 { ((v151 * v645) / self.scalar_v629) } else { (if v618 { ((v151 * v627) / self.scalar_v629) } else { v13 }) });
        let v649: f64 = (if v639 { v13 } else { (if v618 { (v624 * (v617 * v635)) } else { v13 }) });
        let v650: f64 = (v648 + v649);
        let v653: f64 = (self.scalar_v615 * (-v171));
        let v654: f64 = (v204 + v653);
        let v655: bool = (v654 > v13);
        let v659: f64 = (if v655 { self.scalar_v658 } else { v624 });
        let v662: f64 = (v12 - (self.scalar_v620 * (self.scalar_v620 * v659)));
        let v668: f64 = (v654 * self.scalar_v667);
        let v670: f64 = (self.scalar_v620 + (v668 / v171));
        let v674: bool = (!v655);
        let v676: f64 = (v12 - (v204 / v171));
        let v679: f64 = (((self.scalar_v664 * ((v676) as f64).ln())) as f64).exp();
        let v680: f64 = (v12 - v679);
        let v683: f64 = (if v674 { ((v171 * v680) / self.scalar_v664) } else { (if v655 { ((v171 * v662) / self.scalar_v664) } else { v648 }) });
        let v684: f64 = (if v674 { v13 } else { (if v655 { (v659 * (v654 * v670)) } else { v649 }) });
        let v685: f64 = (v683 + v684);
        let v690: f64 = (v21 + v653);
        let v691: bool = (v690 > v13);
        let v692: f64 = (if v691 { self.scalar_v658 } else { v659 });
        let v695: f64 = (v12 - (self.scalar_v620 * (self.scalar_v620 * v692)));
        let v699: f64 = (self.scalar_v667 * v690);
        let v701: f64 = (self.scalar_v620 + (v699 / v171));
        let v705: bool = (!v691);
        let v707: f64 = (v12 - (v21 / v171));
        let v710: f64 = (((self.scalar_v664 * ((v707) as f64).ln())) as f64).exp();
        let v711: f64 = (v12 - v710);
        let v716: f64 = ((if v705 { ((v171 * v711) / self.scalar_v664) } else { (if v691 { ((v171 * v695) / self.scalar_v664) } else { v683 }) }) + (if v705 { v13 } else { (if v691 { (v692 * (v690 * v701)) } else { v684 }) }));
        let v732: f64 = (if self.scalar_v731 { v13 } else { (if self.scalar_v722 { (v512 * self.scalar_v728) } else { v13 }) });
        let v757: f64 = ((if self.scalar_v583 { (v581 + self.scalar_v584) } else { v581 }) / self.scalar_v16);
        let v761: f64 = ((if self.scalar_v583 { (v552 + self.scalar_v590) } else { v552 }) / self.scalar_v16);
        let v765: f64 = ((if self.scalar_v583 { (v544 + self.scalar_v587) } else { v544 }) / self.scalar_v16);
        let v767: f64 = (-(v201 - v477));
        let v768: f64 = 1e-6;
        let v769: f64 = (v477 * v768);
        let v771: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, v477);
        let v772: f64 = (self.scalar_v770 * v771);
        let v776: f64 = (if self.scalar_v572 { (v567 * (-(v276 / v41))) } else { v13 });
        let v777: f64 = (if self.scalar_v572 { v573 } else { v13 });
        let v778: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, v573);
        let v780: f64 = (if self.scalar_v572 { (v567 * v778) } else { v13 });
        let v784: f64 = ctx.node_voltage(nodes[0]);
        let v788: f64 = ((-(((v487 * v553)) as f64).abs()) - (((v489 * (v202 - v784))) as f64).abs());
        let v789: f64 = (if self.scalar_v737 { v788 } else { v13 });
        let v791: f64 = (if self.scalar_v737 { (v1 / self.scalar_v735) } else { v13 });
        let v793: f64 = (v1 * self.scalar_v792);
        let v794: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, v793);
        let v795: f64 = (if self.scalar_v737 { v794 } else { v13 });
        let v798: f64 = (if self.scalar_v797 { v788 } else { v13 });
        let v799: f64 = ctx.node_voltage(nodes[7]);
        let v802: f64 = (if self.scalar_v797 { ((v1 - v799) / self.scalar_v735) } else { v13 });
        let v803: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, v793);
        let v804: f64 = (if self.scalar_v797 { v803 } else { v13 });
        let v806: f64 = (if self.scalar_v797 { (v799 / self.scalar_v740) } else { v13 });
        let v809: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, (v799 * self.scalar_v807));
        let v810: f64 = (if self.scalar_v797 { v809 } else { v13 });
        let v814: f64 = (if self.scalar_v813 { v788 } else { v13 });
        let v815: f64 = (v13 * v200);
        let v816: f64 = (v13 * v20);
        let v818: f64 = (v13 * (v19 - v199));
        let v819: bool = (v757 > self.scalar_v754);
        let v820: f64 = (if v819 { v757 } else { self.scalar_v754 });
        let v822: f64 = (if self.scalar_v756 { (v205 / v820) } else { v13 });
        let v823: bool = (v761 > self.scalar_v754);
        let v824: f64 = (if v823 { v761 } else { self.scalar_v754 });
        let v826: f64 = (if self.scalar_v760 { (v207 / v824) } else { v13 });
        let v827: f64 = (v784 - v19);
        let v828: bool = (v765 > self.scalar_v754);
        let v829: f64 = (if v828 { v765 } else { self.scalar_v754 });
        let v831: f64 = (if self.scalar_v764 { (v827 / v829) } else { v13 });
        let v833: f64 = (self.scalar_v16 * (self.scalar_v17 * v487));
        let v835: f64 = (self.scalar_v16 * (self.scalar_v17 * v489));
        let v838: f64 = (self.scalar_v17 * (self.scalar_v16 * (-v511)));
        let v840: f64 = (self.scalar_v16 * (self.scalar_v17 * (((v484 * v512) * self.scalar_v514) + (v510 * v517))));
        let v842: f64 = (self.scalar_v16 * (self.scalar_v17 * (v159 * v650)));
        let v844: f64 = (self.scalar_v16 * (self.scalar_v17 * (v276 * v567)));
        let v846: f64 = (self.scalar_v16 * (self.scalar_v17 * ((v177 * v685) * self.scalar_v688)));
        let v848: f64 = (self.scalar_v16 * (self.scalar_v17 * (self.scalar_v687 * (v177 * v716))));
        let v850: f64 = (self.scalar_v16 * (self.scalar_v17 * (v511 * self.scalar_v569)));
        let v852: f64 = (self.scalar_v16 * (self.scalar_v17 * (if v605 { (v606 * v611) } else { (if v593 { ((v594 * v601) / self.scalar_v595) } else { v13 }) })));
        let v854: f64 = (self.scalar_v16 * (-v732));
        let v855: f64 = (self.scalar_v16 * v732);
        let v857: f64 = (if v10 { v13 } else { (if v8 { v12 } else { v13 }) });
        let v865: f64 = (self.scalar_v26 * f64::powf(v25, self.scalar_v863));
        let v870: f64 = (v32 * v857);
        let v871: f64 = (v857 / self.scalar_v31);
        let v872: f64 = (v871 / v34);
        let v874: f64 = (v38 * (self.scalar_v36 * v872));
        let v876: f64 = (v29 * (self.scalar_v39 * v874));
        let v877: f64 = (v40 * (self.scalar_v22 * ((-(if v23 { self.scalar_v858 } else { v13 })) * v865)));
        let v878: f64 = (v40 * (self.scalar_v22 * ((-(if v23 { self.scalar_v17 } else { v13 })) * v865)));
        let v887: f64 = ((self.scalar_v60 * v872) + (((v33 * (self.scalar_v62 * v871)) - (v64 * v870)) / (v33 * v33)));
        let v890: f64 = (self.scalar_v69 * (v70 * v887));
        let v899: f64 = (v38 * v38);
        let v913: f64 = (self.scalar_v97 * (self.scalar_v98 * v871));
        let v916: f64 = (v857 / v110);
        let v930: f64 = ((v124 * (((v118 * ((v115 * v857) + (v11 * (v114 * v857)))) - (v116 * v857)) / (v118 * v118))) - (v121 * (v122 * (v857 + v857))));
        let v941: f64 = ((v135 * (-(v870 + v870))) + (v129 * ((v130 * (v916 / v112)) + (v133 * (v930 / (v124 * v124))))));
        let v943: f64 = ((-v941) / self.scalar_v111);
        let v944: f64 = (-v943);
        let v948: f64 = (v139 * v139);
        let v957: f64 = (v112 * v943);
        let v959: f64 = (v941 + ((v139 * v916) + v957));
        let v965: f64 = (v143 * v857);
        let v970: f64 = ((v158 * ((-(self.scalar_v107 * (self.scalar_v142 * (-(((v139 * v944) - (v140 * v943)) / v948))))) / (v148 * v148))) + (v149 * (self.scalar_v142 * (v965 - (((v139 * (v959 - v943)) - (v152 * v943)) / v948)))));
        let v974: f64 = (v162 * v162);
        let v984: f64 = (v941 + (v957 + (v162 * v916)));
        let v994: f64 = ((v176 * ((-(self.scalar_v108 * (self.scalar_v165 * (-(((v162 * v944) - (v163 * v943)) / v974))))) / (v168 * v168))) + (v169 * (self.scalar_v165 * (v965 - (((v162 * (v984 - v943)) - (v172 * v943)) / v974)))));
        let v998: f64 = (v180 * v180);
        let v1008: f64 = (v941 + (v957 + (v180 * v916)));
        let v1018: f64 = ((v194 * ((-(self.scalar_v109 * (self.scalar_v183 * (-(((v180 * v944) - (v181 * v943)) / v998))))) / (v186 * v186))) + (v187 * (self.scalar_v183 * (v965 - (((v180 * (v1008 - v943)) - (v190 * v943)) / v998)))));
        let v1026: f64 = (if v209 { ((-(v201 * (self.scalar_v210 * v870))) / (v211 * v211)) } else { v13 });
        let v1027: f64 = (if v209 { (self.scalar_v17 / v211) } else { v13 });
        let v1028: f64 = (if v209 { (self.scalar_v858 / v211) } else { v13 });
        let v1029: f64 = (-(self.scalar_v92 * (self.scalar_v93 * v871)));
        let v1030: f64 = (self.scalar_v216 * v870);
        let v1031: f64 = (v217 * v1029);
        let v1034: f64 = (v217 * v217);
        let v1038: f64 = (if v209 { ((v1031 - (v215 * v1030)) / v1034) } else { v13 });
        let v1039: f64 = (if v209 { (self.scalar_v858 / v217) } else { v13 });
        let v1040: f64 = (if v209 { (self.scalar_v17 / v217) } else { v13 });
        let v1044: f64 = (if v209 { ((v1031 - (v220 * v1030)) / v1034) } else { v13 });
        let v1048: f64 = (if v225 { v13 } else { v1026 });
        let v1049: f64 = (if v225 { v13 } else { v1027 });
        let v1050: f64 = (if v225 { v13 } else { v1028 });
        let v1051: f64 = (if v231 { v13 } else { (if v225 { v1026 } else { v13 }) });
        let v1052: f64 = (if v231 { v13 } else { (if v225 { v1027 } else { v13 }) });
        let v1053: f64 = (if v231 { v13 } else { (if v225 { v1028 } else { v13 }) });
        let v1066: f64 = (if v209 { ((v233 * v1051) + (v232 * (v233 * v1048))) } else { v1051 });
        let v1067: f64 = (if v209 { ((v233 * v1052) + (v232 * (v233 * v1049))) } else { v1052 });
        let v1068: f64 = (if v209 { ((v233 * v1053) + (v232 * (v233 * v1050))) } else { v1053 });
        let v1069: f64 = (v243 * v1038);
        let v1070: f64 = (v243 * v1039);
        let v1071: f64 = (v243 * v1040);
        let v1084: f64 = (v255 * v1044);
        let v1089: f64 = ((if v242 { (v1069 / v244) } else { (if v246 { v1069 } else { (if v237 { v1038 } else { v13 }) }) }) - (if v254 { (v1084 / v256) } else { (if v258 { v1084 } else { (if v250 { v1044 } else { v13 }) }) }));
        let v1090: f64 = (if v209 { v1089 } else { v13 });
        let v1091: f64 = (if v209 { (if v242 { (v1070 / v244) } else { (if v246 { v1070 } else { (if v237 { v1039 } else { v13 }) }) }) } else { v13 });
        let v1092: f64 = (if v209 { (if v242 { (v1071 / v244) } else { (if v246 { v1071 } else { (if v237 { v1040 } else { v13 }) }) }) } else { v13 });
        let v1106: f64 = (self.scalar_v267 * (v913 * (v269 * ((v268) as f64).ln())));
        let v1110: f64 = (v271 * v271);
        let v1114: f64 = (((v264 * v890) + (v71 * v1066)) - (((v271 * ((v263 * (self.scalar_v87 * (self.scalar_v88 * v871))) + (v91 * v1090))) - (v266 * v1106)) / v1110));
        let v1120: f64 = (if v275 { v13 } else { (if v209 { v1114 } else { v13 }) });
        let v1121: f64 = (if v275 { v13 } else { (if v209 { ((v71 * v1067) - ((v91 * v1091) / v271)) } else { v13 }) });
        let v1122: f64 = (if v275 { v13 } else { (if v209 { ((v71 * v1068) - ((v91 * v1092) / v271)) } else { v13 }) });
        let v1135: f64 = (v288 * v288);
        let v1145: f64 = (if v277 { ((-(v285 * (v283 * (self.scalar_v286 * v870)))) / v1135) } else { v1048 });
        let v1146: f64 = (if v277 { (((v288 * self.scalar_v1127) - (v285 * (v287 * (if v277 { (if v281 { self.scalar_v858 } else { v13 }) } else { v13 })))) / v1135) } else { v1049 });
        let v1147: f64 = (if v277 { (((v288 * self.scalar_v1128) - (v285 * (v287 * (if v277 { (if v281 { self.scalar_v17 } else { v13 }) } else { v13 })))) / v1135) } else { v1050 });
        let v1151: f64 = (if v292 { v13 } else { v1145 });
        let v1152: f64 = (if v292 { v13 } else { v1146 });
        let v1153: f64 = (if v292 { v13 } else { v1147 });
        let v1154: f64 = (if v298 { v13 } else { (if v292 { v1145 } else { v1066 }) });
        let v1155: f64 = (if v298 { v13 } else { (if v292 { v1146 } else { v1067 }) });
        let v1156: f64 = (if v298 { v13 } else { (if v292 { v1147 } else { v1068 }) });
        let v1169: f64 = (if v277 { ((v300 * v1154) + (v299 * (v300 * v1151))) } else { v1154 });
        let v1170: f64 = (if v277 { ((v300 * v1155) + (v299 * (v300 * v1152))) } else { v1155 });
        let v1171: f64 = (if v277 { ((v300 * v1156) + (v299 * (v300 * v1153))) } else { v1156 });
        let v1190: f64 = (if v308 { ((-(v201 * (self.scalar_v76 * v870))) / (v309 * v309)) } else { v1151 });
        let v1191: f64 = (if v308 { (self.scalar_v17 / v309) } else { v1152 });
        let v1192: f64 = (if v308 { (self.scalar_v858 / v309) } else { v1153 });
        let v1193: f64 = (self.scalar_v312 * v870);
        let v1194: f64 = (v313 * v1029);
        let v1197: f64 = (v313 * v313);
        let v1199: f64 = (self.scalar_v858 / v313);
        let v1200: f64 = (self.scalar_v17 / v313);
        let v1201: f64 = (if v308 { ((v1194 - (v215 * v1193)) / v1197) } else { v1038 });
        let v1202: f64 = (if v308 { v1199 } else { v1039 });
        let v1203: f64 = (if v308 { v1200 } else { v1040 });
        let v1206: f64 = ((v1194 - (v220 * v1193)) / v1197);
        let v1207: f64 = (if v308 { v1206 } else { v1044 });
        let v1211: f64 = (if v319 { v13 } else { v1190 });
        let v1212: f64 = (if v319 { v13 } else { v1191 });
        let v1213: f64 = (if v319 { v13 } else { v1192 });
        let v1214: f64 = (if v325 { v13 } else { (if v319 { v1190 } else { v1169 }) });
        let v1215: f64 = (if v325 { v13 } else { (if v319 { v1191 } else { v1170 }) });
        let v1216: f64 = (if v325 { v13 } else { (if v319 { v1192 } else { v1171 }) });
        let v1229: f64 = (if v308 { ((v327 * v1214) + (v326 * (v327 * v1211))) } else { v1214 });
        let v1230: f64 = (if v308 { ((v327 * v1215) + (v326 * (v327 * v1212))) } else { v1215 });
        let v1231: f64 = (if v308 { ((v327 * v1216) + (v326 * (v327 * v1213))) } else { v1216 });
        let v1232: f64 = (v335 * v1201);
        let v1233: f64 = (v335 * v1202);
        let v1234: f64 = (v335 * v1203);
        let v1247: f64 = (v347 * v1207);
        let v1252: f64 = ((if v334 { (v1232 / v336) } else { (if v338 { v1232 } else { (if v330 { v1201 } else { v13 }) }) }) - (if v346 { (v1247 / v348) } else { (if v350 { v1247 } else { (if v342 { v1207 } else { v13 }) }) }));
        let v1253: f64 = (if v308 { v1252 } else { v1090 });
        let v1254: f64 = (if v308 { (if v334 { (v1233 / v336) } else { (if v338 { v1233 } else { (if v330 { v1202 } else { v13 }) }) }) } else { v1091 });
        let v1255: f64 = (if v308 { (if v334 { (v1234 / v336) } else { (if v338 { v1234 } else { (if v330 { v1203 } else { v13 }) }) }) } else { v1092 });
        let v1270: f64 = (((v356 * (((v38 * (self.scalar_v75 * (v78 * (v887 / self.scalar_v76)))) - (v79 * v874)) / v899)) + (v80 * v1229)) - (((v271 * (v13 * v1253)) - (v358 * v1106)) / v1110));
        let v1286: f64 = (if v209 { ((-(v21 * (self.scalar_v364 * v870))) / (v365 * v365)) } else { v1211 });
        let v1287: f64 = (if v209 { (self.scalar_v858 / v365) } else { v13 });
        let v1288: f64 = (if v209 { (self.scalar_v17 / v365) } else { v1212 });
        let v1289: f64 = (if v209 { v13 } else { v1213 });
        let v1292: f64 = ((v1194 - (v369 * v1193)) / v1197);
        let v1293: f64 = (if v209 { v1292 } else { v1201 });
        let v1294: f64 = (if v209 { v1200 } else { v13 });
        let v1295: f64 = (if v209 { v1199 } else { v1202 });
        let v1296: f64 = (if v209 { v13 } else { v1203 });
        let v1297: f64 = (if v209 { v1206 } else { v1207 });
        let v1302: f64 = (if v374 { v13 } else { v1286 });
        let v1303: f64 = (if v374 { v13 } else { v1287 });
        let v1304: f64 = (if v374 { v13 } else { v1288 });
        let v1305: f64 = (if v374 { v13 } else { v1289 });
        let v1306: f64 = (if v380 { v13 } else { (if v374 { v1286 } else { v1229 }) });
        let v1307: f64 = (if v380 { v13 } else { (if v374 { v1287 } else { v13 }) });
        let v1308: f64 = (if v380 { v13 } else { (if v374 { v1288 } else { v1230 }) });
        let v1309: f64 = (if v380 { v13 } else { (if v374 { v1289 } else { v1231 }) });
        let v1326: f64 = (if v209 { ((v382 * v1306) + (v381 * (v382 * v1302))) } else { v1306 });
        let v1327: f64 = (if v209 { ((v382 * v1307) + (v381 * (v382 * v1303))) } else { v1307 });
        let v1328: f64 = (if v209 { ((v382 * v1308) + (v381 * (v382 * v1304))) } else { v1308 });
        let v1329: f64 = (if v209 { ((v382 * v1309) + (v381 * (v382 * v1305))) } else { v1309 });
        let v1330: f64 = (v390 * v1293);
        let v1331: f64 = (v390 * v1294);
        let v1332: f64 = (v390 * v1295);
        let v1333: f64 = (v390 * v1296);
        let v1350: f64 = (v402 * v1297);
        let v1355: f64 = ((if v389 { (v1330 / v391) } else { (if v393 { v1330 } else { (if v385 { v1293 } else { v13 }) }) }) - (if v401 { (v1350 / v403) } else { (if v405 { v1350 } else { (if v397 { v1297 } else { v13 }) }) }));
        let v1356: f64 = (if v209 { v1355 } else { v1253 });
        let v1357: f64 = (if v209 { (if v389 { (v1331 / v391) } else { (if v393 { v1331 } else { (if v385 { v1294 } else { v13 }) }) }) } else { v13 });
        let v1358: f64 = (if v209 { (if v389 { (v1332 / v391) } else { (if v393 { v1332 } else { (if v385 { v1295 } else { v13 }) }) }) } else { v1254 });
        let v1359: f64 = (if v209 { (if v389 { (v1333 / v391) } else { (if v393 { v1333 } else { (if v385 { v1296 } else { v13 }) }) }) } else { v1255 });
        let v1380: f64 = (((v417 * ((v410 * (self.scalar_v102 * (self.scalar_v103 * v871))) + (v106 * v1356))) - (v413 * (self.scalar_v267 * (v913 * (v415 * ((v414) as f64).ln()))))) / (v417 * v417));
        let v1392: f64 = (if v275 { v13 } else { (if v209 { (((v411 * v890) + (v71 * v1326)) - v1380) } else { v13 }) });
        let v1393: f64 = (if v275 { v13 } else { (if v209 { ((v71 * v1327) - ((v106 * v1357) / v417)) } else { v13 }) });
        let v1394: f64 = (if v275 { v13 } else { (if v209 { ((v71 * v1328) - ((v106 * v1358) / v417)) } else { v13 }) });
        let v1395: f64 = (if v275 { v13 } else { (if v209 { ((v71 * v1329) - ((v106 * v1359) / v417)) } else { v13 }) });
        let v1403: f64 = (if v422 { ((-(v21 * (self.scalar_v82 * v870))) / (v423 * v423)) } else { v1302 });
        let v1404: f64 = (if v422 { (self.scalar_v858 / v423) } else { v1303 });
        let v1405: f64 = (if v422 { (self.scalar_v17 / v423) } else { v1304 });
        let v1406: f64 = (if v422 { v13 } else { v1305 });
        let v1407: f64 = (if v422 { v1292 } else { v1293 });
        let v1408: f64 = (if v422 { v1200 } else { v1294 });
        let v1409: f64 = (if v422 { v1199 } else { v1295 });
        let v1410: f64 = (if v422 { v13 } else { v1296 });
        let v1411: f64 = (if v422 { v1206 } else { v1297 });
        let v1420: f64 = (if v435 { v13 } else { (if v429 { v1403 } else { v1326 }) });
        let v1421: f64 = (if v435 { v13 } else { (if v429 { v1404 } else { v1327 }) });
        let v1422: f64 = (if v435 { v13 } else { (if v429 { v1405 } else { v1328 }) });
        let v1423: f64 = (if v435 { v13 } else { (if v429 { v1406 } else { v1329 }) });
        let v1444: f64 = (v445 * v1407);
        let v1445: f64 = (v445 * v1408);
        let v1446: f64 = (v445 * v1409);
        let v1447: f64 = (v445 * v1410);
        let v1464: f64 = (v457 * v1411);
        let v1469: f64 = ((if v444 { (v1444 / v446) } else { (if v448 { v1444 } else { (if v440 { v1407 } else { v13 }) }) }) - (if v456 { (v1464 / v458) } else { (if v460 { v1464 } else { (if v452 { v1411 } else { v13 }) }) }));
        let v1476: f64 = ((v466 * (((v38 * (self.scalar_v81 * (v84 * (v887 / self.scalar_v82)))) - (v85 * v874)) / v899)) + (v86 * (if v422 { ((v437 * v1420) + (v436 * (v437 * (if v429 { v13 } else { v1403 })))) } else { v1420 })));
        let v1489: f64 = ((v86 * (if v422 { ((v437 * v1421) + (v436 * (v437 * (if v429 { v13 } else { v1404 })))) } else { v1421 })) - ((v13 * (if v422 { (if v444 { (v1445 / v446) } else { (if v448 { v1445 } else { (if v440 { v1408 } else { v13 }) }) }) } else { v1357 })) / v471));
        let v1490: f64 = ((v86 * (if v422 { ((v437 * v1422) + (v436 * (v437 * (if v429 { v13 } else { v1405 })))) } else { v1422 })) - ((v13 * (if v422 { (if v444 { (v1446 / v446) } else { (if v448 { v1446 } else { (if v440 { v1409 } else { v13 }) }) }) } else { v1358 })) / v471));
        let v1491: f64 = ((v86 * (if v422 { ((v437 * v1423) + (v436 * (v437 * (if v429 { v13 } else { v1406 })))) } else { v1423 })) - ((v13 * (if v422 { (if v444 { (v1447 / v446) } else { (if v448 { v1447 } else { (if v440 { v1410 } else { v13 }) }) }) } else { v1359 })) / v471));
        let v1505: f64 = ((v41 * (v1120 - (if v306 { v13 } else { (if v277 { ((v303 * (self.scalar_v72 * (v73 * (self.scalar_v67 * v872)))) + (v74 * v1169)) } else { v13 }) }))) - (v485 * v876));
        let v1506: f64 = (v41 * v41);
        let v1517: f64 = ((if v362 { v13 } else { (if v308 { ((v80 * v1230) - ((v13 * v1254) / v271)) } else { v13 }) }) + (((v41 * (v1121 - (if v306 { v13 } else { (if v277 { (v74 * v1170) } else { v13 }) }))) - (v485 * v878)) / v1506));
        let v1518: f64 = ((if v362 { v13 } else { (if v308 { ((v80 * v1231) - ((v13 * v1255) / v271)) } else { v13 }) }) + ((v1122 - (if v306 { v13 } else { (if v277 { (v74 * v1171) } else { v13 }) })) / v41));
        let v1527: f64 = ((if v475 { v13 } else { (if v422 { (v1476 - ((v13 * (if v422 { v1469 } else { v1356 })) / v471)) } else { v13 }) }) + (((v43 * v1392) - (v421 * (self.scalar_v42 * v874))) / (v43 * v43)));
        let v1542: f64 = (self.scalar_v1539 / v507);
        let v1543: f64 = (self.scalar_v1540 / v507);
        let v1544: f64 = (self.scalar_v1541 / v507);
        let v1545: f64 = (v510 * v1392);
        let v1548: f64 = ((v510 * v1393) + (v421 * v1542));
        let v1551: f64 = ((v510 * v1394) + (v421 * v1543));
        let v1554: f64 = ((v510 * v1395) + (v421 * v1544));
        let v1555: f64 = (v510 * v1120);
        let v1556: f64 = (v276 * v1542);
        let v1559: f64 = ((v510 * v1121) + (v276 * v1543));
        let v1562: f64 = ((v510 * v1122) + (v276 * v1544));
        let v1589: f64 = (v538 * (self.scalar_v532 * (v535 * (self.scalar_v533 * v872))));
        let v1612: f64 = (v189 * v189);
        let v1633: f64 = ((v601 * ((v195 * v1008) + (v189 * v1018))) + (v594 * (-(v600 * (self.scalar_v595 * ((-((-(v198 * v1008)) / v1612)) / v597))))));
        let v1660: f64 = (if v605 { ((v611 * (self.scalar_v17 * v195)) + (v606 * (self.scalar_v1644 / v189))) } else { (if v593 { ((v594 * (-(v600 * (self.scalar_v595 * ((-(self.scalar_v17 / v189)) / v597))))) / self.scalar_v595) } else { v13 }) });
        let v1662: f64 = (if v605 { ((v611 * (v195 * self.scalar_v858)) + (v606 * (self.scalar_v1645 / v189))) } else { (if v593 { ((v594 * (-(v600 * (self.scalar_v595 * ((-(self.scalar_v858 / v189)) / v597))))) / self.scalar_v595) } else { v13 }) });
        let v1664: f64 = (self.scalar_v615 * (-v959));
        let v1674: f64 = (v151 * v151);
        let v1721: f64 = (if v639 { (((v645 * v959) + (v151 * (-(v644 * (self.scalar_v629 * ((-((-(v201 * v959)) / v1674)) / v641)))))) / self.scalar_v629) } else { (if v618 { ((v627 * v959) / self.scalar_v629) } else { v13 }) });
        let v1722: f64 = (if v639 { ((v151 * (-(v644 * (self.scalar_v629 * ((-(self.scalar_v17 / v151)) / v641))))) / self.scalar_v629) } else { v13 });
        let v1723: f64 = (if v639 { ((v151 * (-(v644 * (self.scalar_v629 * ((-(self.scalar_v858 / v151)) / v641))))) / self.scalar_v629) } else { v13 });
        let v1724: f64 = (if v639 { v13 } else { (if v618 { (v624 * ((v635 * v1664) + (v617 * (((v151 * (self.scalar_v632 * v1664)) - (v633 * v959)) / v1674)))) } else { v13 }) });
        let v1725: f64 = (if v639 { v13 } else { (if v618 { (v624 * ((self.scalar_v17 * v635) + (v617 * (self.scalar_v1669 / v151)))) } else { v13 }) });
        let v1726: f64 = (if v639 { v13 } else { (if v618 { (v624 * ((v635 * self.scalar_v858) + (v617 * (self.scalar_v1670 / v151)))) } else { v13 }) });
        let v1736: f64 = (self.scalar_v615 * (-v984));
        let v1745: f64 = (self.scalar_v1742 / v171);
        let v1746: f64 = (v171 * (self.scalar_v667 * v1736));
        let v1749: f64 = (v171 * v171);
        let v1751: f64 = (self.scalar_v1744 / v171);
        let v1774: f64 = (-(self.scalar_v17 / v171));
        let v1776: f64 = (-(self.scalar_v858 / v171));
        let v1797: f64 = (if v674 { ((v171 * (-(v679 * (self.scalar_v664 * (v1774 / v676))))) / self.scalar_v664) } else { v13 });
        let v1798: f64 = (if v674 { (((v680 * v984) + (v171 * (-(v679 * (self.scalar_v664 * ((-((-(v204 * v984)) / v1749)) / v676)))))) / self.scalar_v664) } else { (if v655 { ((v662 * v984) / self.scalar_v664) } else { v1721 }) });
        let v1799: f64 = (if v674 { ((v171 * (-(v679 * (self.scalar_v664 * (v1776 / v676))))) / self.scalar_v664) } else { v13 });
        let v1800: f64 = (if v674 { v13 } else { (if v655 { v13 } else { v1722 }) });
        let v1801: f64 = (if v674 { v13 } else { (if v655 { v13 } else { v1723 }) });
        let v1802: f64 = (if v674 { v13 } else { (if v655 { (v659 * ((self.scalar_v17 * v670) + (v654 * v1745))) } else { v13 }) });
        let v1803: f64 = (if v674 { v13 } else { (if v655 { (v659 * ((v670 * v1736) + (v654 * ((v1746 - (v668 * v984)) / v1749)))) } else { v1724 }) });
        let v1804: f64 = (if v674 { v13 } else { (if v655 { (v659 * ((v670 * self.scalar_v858) + (v654 * v1751))) } else { v13 }) });
        let v1805: f64 = (if v674 { v13 } else { (if v655 { v13 } else { v1725 }) });
        let v1806: f64 = (if v674 { v13 } else { (if v655 { v13 } else { v1726 }) });
        let v1876: f64 = (if v705 { (((v711 * v984) + (v171 * (-(v710 * (self.scalar_v664 * ((-((-(v21 * v984)) / v1749)) / v707)))))) / self.scalar_v664) } else { (if v691 { ((v695 * v984) / self.scalar_v664) } else { v1798 }) });
        let v1887: f64 = ((if v705 { ((v171 * (-(v710 * (self.scalar_v664 * (v1776 / v707))))) / self.scalar_v664) } else { (if v691 { v13 } else { v1799 }) }) + (if v705 { v13 } else { (if v691 { (v692 * ((v701 * self.scalar_v858) + (v690 * v1751))) } else { v1804 }) }));
        let v1888: f64 = ((if v705 { ((v171 * (-(v710 * (self.scalar_v664 * (v1774 / v707))))) / self.scalar_v664) } else { (if v691 { v13 } else { v1800 }) }) + (if v705 { v13 } else { (if v691 { (v692 * ((self.scalar_v17 * v701) + (v690 * v1745))) } else { v1805 }) }));
        let v1892: f64 = (v177 * (v1876 + (if v705 { v13 } else { (if v691 { (v692 * ((v701 * v1736) + (v690 * ((v1746 - (v699 * v984)) / v1749)))) } else { v1803 }) })));
        let v1910: f64 = (if self.scalar_v731 { v13 } else { (if self.scalar_v722 { (self.scalar_v728 * v1555) } else { v13 }) });
        let v1911: f64 = (if self.scalar_v731 { v13 } else { (if self.scalar_v722 { (self.scalar_v728 * v1556) } else { v13 }) });
        let v1912: f64 = (if self.scalar_v731 { v13 } else { (if self.scalar_v722 { (self.scalar_v728 * v1559) } else { v13 }) });
        let v1913: f64 = (if self.scalar_v731 { v13 } else { (if self.scalar_v722 { (self.scalar_v728 * v1562) } else { v13 }) });
        let v1917: f64 = ddt_scale;
        let v1918: f64 = (self.scalar_v770 * v1917);
        let v1939: f64 = (if self.scalar_v572 { (v567 * (-(((v41 * v1120) - (v276 * v876)) / v1506))) } else { v13 });
        let v1940: f64 = (if self.scalar_v572 { (v567 * (-((-(v276 * v877)) / v1506))) } else { v13 });
        let v1941: f64 = (if self.scalar_v572 { (v567 * (-(((v41 * v1121) - (v276 * v878)) / v1506))) } else { v13 });
        let v1942: f64 = (if self.scalar_v572 { (v567 * (-(v1122 / v41))) } else { v13 });
        let v1945: f64 = (if self.scalar_v572 { (v567 * v1917) } else { v13 });
        let v1948: f64 = (self.scalar_v792 * v1917);
        let v1949: f64 = (if self.scalar_v737 { v1948 } else { v13 });
        let v1953: f64 = (if self.scalar_v797 { v1948 } else { v13 });
        let v1957: f64 = (if self.scalar_v797 { (self.scalar_v807 * v1917) } else { v13 });
        let v1958: f64 = -0.0;
        let v1966: f64 = (if self.scalar_v756 { (v12 / v820) } else { v13 });
        let v1967: f64 = (if self.scalar_v756 { ((-(v205 * (if v819 { ((if self.scalar_v572 { (v1589 / v579) } else { v1589 }) / self.scalar_v16) } else { v13 }))) / (v820 * v820)) } else { v13 });
        let v1968: f64 = (if self.scalar_v756 { (v284 / v820) } else { v13 });
        let v1976: f64 = (if self.scalar_v760 { (v12 / v824) } else { v13 });
        let v1977: f64 = (if self.scalar_v760 { ((-(v207 * (if v823 { ((v551 * (self.scalar_v545 * (v548 * (self.scalar_v546 * v872)))) / self.scalar_v16) } else { v13 }))) / (v824 * v824)) } else { v13 });
        let v1978: f64 = (if self.scalar_v760 { (v284 / v824) } else { v13 });
        let v1986: f64 = (if self.scalar_v764 { (v12 / v829) } else { v13 });
        let v1987: f64 = (if self.scalar_v764 { ((-(v827 * (if v828 { ((self.scalar_v540 * (v543 * (self.scalar_v541 * v872))) / self.scalar_v16) } else { v13 }))) / (v829 * v829)) } else { v13 });
        let v1988: f64 = (if self.scalar_v764 { (v284 / v829) } else { v13 });
        let v1993: f64 = (self.scalar_v16 * (self.scalar_v17 * ((if v362 { v13 } else { (if v308 { v1270 } else { v13 }) }) + (v1505 / v1506))));
        let v1994: f64 = (self.scalar_v16 * (self.scalar_v17 * ((-(v485 * v877)) / v1506)));
        let v1995: f64 = (self.scalar_v16 * (self.scalar_v17 * v1517));
        let v1996: f64 = (self.scalar_v16 * (self.scalar_v17 * v1518));
        let v2001: f64 = (self.scalar_v16 * (self.scalar_v17 * v1527));
        let v2002: f64 = (self.scalar_v16 * (self.scalar_v17 * ((if v475 { v13 } else { (if v422 { v1489 } else { v13 }) }) + (v1393 / v43))));
        let v2003: f64 = (self.scalar_v16 * (self.scalar_v17 * ((if v475 { v13 } else { (if v422 { v1490 } else { v13 }) }) + (v1394 / v43))));
        let v2004: f64 = (self.scalar_v16 * (self.scalar_v17 * ((if v475 { v13 } else { (if v422 { v1491 } else { v13 }) }) + (v1395 / v43))));
        let v2013: f64 = (self.scalar_v17 * (self.scalar_v16 * (-v1545)));
        let v2014: f64 = (self.scalar_v17 * (self.scalar_v16 * (-v1548)));
        let v2015: f64 = (self.scalar_v17 * (self.scalar_v16 * (-v1551)));
        let v2016: f64 = (self.scalar_v17 * (self.scalar_v16 * (-v1554)));
        let v2021: f64 = (self.scalar_v16 * (self.scalar_v17 * ((self.scalar_v514 * (v484 * v1555)) + (v510 * (self.scalar_v516 * v1120)))));
        let v2022: f64 = (self.scalar_v16 * (self.scalar_v17 * ((self.scalar_v514 * (v484 * v1556)) + (v517 * v1542))));
        let v2023: f64 = (self.scalar_v16 * (self.scalar_v17 * ((self.scalar_v514 * (v484 * v1559)) + ((v517 * v1543) + (v510 * (self.scalar_v516 * v1121))))));
        let v2024: f64 = (self.scalar_v16 * (self.scalar_v17 * ((self.scalar_v514 * (v484 * v1562)) + ((v517 * v1544) + (v510 * (self.scalar_v516 * v1122))))));
        let v2028: f64 = (self.scalar_v16 * (self.scalar_v17 * ((v650 * v970) + (v159 * (v1721 + v1724)))));
        let v2029: f64 = (self.scalar_v16 * (self.scalar_v17 * (v159 * (v1722 + v1725))));
        let v2030: f64 = (self.scalar_v16 * (self.scalar_v17 * (v159 * (v1723 + v1726))));
        let v2034: f64 = (self.scalar_v16 * (self.scalar_v17 * (v567 * v1120)));
        let v2035: f64 = (self.scalar_v16 * (self.scalar_v17 * (v567 * v1121)));
        let v2036: f64 = (self.scalar_v16 * (self.scalar_v17 * (v567 * v1122)));
        let v2042: f64 = (self.scalar_v16 * (self.scalar_v17 * (self.scalar_v688 * (v177 * (v1797 + v1802)))));
        let v2043: f64 = (self.scalar_v16 * (self.scalar_v17 * (self.scalar_v688 * ((v685 * v994) + (v177 * (v1798 + v1803))))));
        let v2044: f64 = (self.scalar_v16 * (self.scalar_v17 * (self.scalar_v688 * (v177 * (v1799 + v1804)))));
        let v2045: f64 = (self.scalar_v16 * (self.scalar_v17 * (self.scalar_v688 * (v177 * (v1800 + v1805)))));
        let v2046: f64 = (self.scalar_v16 * (self.scalar_v17 * (self.scalar_v688 * (v177 * (v1801 + v1806)))));
        let v2052: f64 = (self.scalar_v16 * (self.scalar_v17 * (self.scalar_v687 * (v177 * ((if v705 { v13 } else { (if v691 { v13 } else { v1797 }) }) + (if v705 { v13 } else { (if v691 { v13 } else { v1802 }) }))))));
        let v2053: f64 = (self.scalar_v16 * (self.scalar_v17 * (self.scalar_v687 * ((v716 * v994) + v1892))));
        let v2054: f64 = (self.scalar_v16 * (self.scalar_v17 * (self.scalar_v687 * (v177 * v1887))));
        let v2055: f64 = (self.scalar_v16 * (self.scalar_v17 * (self.scalar_v687 * (v177 * v1888))));
        let v2056: f64 = (self.scalar_v16 * (self.scalar_v17 * (self.scalar_v687 * (v177 * ((if v705 { v13 } else { (if v691 { v13 } else { v1801 }) }) + (if v705 { v13 } else { (if v691 { v13 } else { v1806 }) }))))));
        let v2061: f64 = (self.scalar_v16 * (self.scalar_v17 * (self.scalar_v569 * v1545)));
        let v2062: f64 = (self.scalar_v16 * (self.scalar_v17 * (self.scalar_v569 * v1548)));
        let v2063: f64 = (self.scalar_v16 * (self.scalar_v17 * (self.scalar_v569 * v1551)));
        let v2064: f64 = (self.scalar_v16 * (self.scalar_v17 * (self.scalar_v569 * v1554)));
        let v2068: f64 = (self.scalar_v16 * (self.scalar_v17 * v1660));
        let v2069: f64 = (self.scalar_v16 * (self.scalar_v17 * (if v605 { ((v611 * (v198 * v1018)) + (v606 * ((-(v609 * v1008)) / v1612))) } else { (if v593 { (v1633 / self.scalar_v595) } else { v13 }) })));
        let v2070: f64 = (self.scalar_v16 * (self.scalar_v17 * v1662));
        let v2075: f64 = (self.scalar_v16 * (-v1910));
        let v2076: f64 = (self.scalar_v16 * (-v1911));
        let v2077: f64 = (self.scalar_v16 * (-v1912));
        let v2078: f64 = (self.scalar_v16 * (-v1913));
        let v2079: f64 = (self.scalar_v16 * v1910);
        let v2080: f64 = (self.scalar_v16 * v1911);
        let v2081: f64 = (self.scalar_v16 * v1912);
        let v2082: f64 = (self.scalar_v16 * v1913);

        let d767_dn5: f64 = self.scalar_v858;
        let d767_dn6: f64 = self.scalar_v17;
        let d767_dn9: f64 = v12;
        stamper.stamp_current_node3_local(
            Some(9),
            None,
            multiplicity * (v767),
            5,
            multiplicity * (d767_dn5),
            6,
            multiplicity * (d767_dn6),
            9,
            multiplicity * (d767_dn9),
        );
        let d769_dn9: f64 = v768;
        stamper.stamp_current_node1_local(
            Some(9),
            None,
            multiplicity * (v769),
            9,
            multiplicity * (d769_dn9),
        );
        let d772_dn9: f64 = v1918;
        stamper.stamp_current_node1_local(
            Some(9),
            None,
            multiplicity * (v772),
            9,
            multiplicity * (d772_dn9),
        );
        let d776_dn3: f64 = v1939;
        let d776_dn4: f64 = v1940;
        let d776_dn5: f64 = v1941;
        let d776_dn6: f64 = v1942;
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(8),
            None,
            multiplicity * (v776),
            [3, 4, 5, 6],
            [d776_dn3, d776_dn4, d776_dn5, d776_dn6],
            [],
            [],
            multiplicity,
        );
        let d777_dn8: f64 = self.scalar_v1943;
        stamper.stamp_current_node1_local(
            Some(8),
            None,
            multiplicity * (v777),
            8,
            multiplicity * (d777_dn8),
        );
        let d780_dn8: f64 = v1945;
        stamper.stamp_current_node1_local(
            Some(8),
            None,
            multiplicity * (v780),
            8,
            multiplicity * (d780_dn8),
        );
        stamper.stamp_potential_branch_local(
            Some(8),
            None,
            0,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            0,
            v13,
        );
        stamper.stamp_current_const_local(
            Some(3),
            None,
            multiplicity * (v789),
        );
        let d791_dn3: f64 = self.scalar_v1947;
        stamper.stamp_current_node1_local(
            Some(3),
            None,
            multiplicity * (v791),
            3,
            multiplicity * (d791_dn3),
        );
        let d795_dn3: f64 = v1949;
        stamper.stamp_current_node1_local(
            Some(3),
            None,
            multiplicity * (v795),
            3,
            multiplicity * (d795_dn3),
        );
        stamper.stamp_potential_branch_local(
            Some(7),
            None,
            1,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            1,
            v13,
        );
        stamper.stamp_current_const_local(
            Some(3),
            None,
            multiplicity * (v798),
        );
        let d802_dn3: f64 = self.scalar_v1951;
        let d802_dn7: f64 = self.scalar_v1952;
        stamper.stamp_current_node2_local(
            Some(3),
            Some(7),
            multiplicity * (v802),
            3,
            multiplicity * (d802_dn3),
            7,
            multiplicity * (d802_dn7),
        );
        let d804_dn3: f64 = v1953;
        stamper.stamp_current_node1_local(
            Some(3),
            None,
            multiplicity * (v804),
            3,
            multiplicity * (d804_dn3),
        );
        let d806_dn7: f64 = self.scalar_v1955;
        stamper.stamp_current_node1_local(
            Some(7),
            None,
            multiplicity * (v806),
            7,
            multiplicity * (d806_dn7),
        );
        let d810_dn7: f64 = v1957;
        stamper.stamp_current_node1_local(
            Some(7),
            None,
            multiplicity * (v810),
            7,
            multiplicity * (d810_dn7),
        );
        stamper.stamp_current_const_local(
            Some(3),
            None,
            multiplicity * (v814),
        );
        stamper.stamp_potential_branch_local(
            Some(7),
            None,
            2,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            2,
            v13,
        );
        stamper.stamp_potential_branch_local(
            Some(3),
            None,
            3,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            3,
            v13,
        );
        stamper.stamp_potential_branch_local(
            Some(7),
            None,
            4,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            4,
            v13,
        );
        let d815_dn5: f64 = v13;
        let d815_dn6: f64 = v1958;
        stamper.stamp_current_node2_local(
            Some(5),
            Some(6),
            multiplicity * (v815),
            5,
            multiplicity * (d815_dn5),
            6,
            multiplicity * (d815_dn6),
        );
        let d816_dn4: f64 = v1958;
        let d816_dn5: f64 = v13;
        stamper.stamp_current_node2_local(
            Some(5),
            Some(4),
            multiplicity * (v816),
            4,
            multiplicity * (d816_dn4),
            5,
            multiplicity * (d816_dn5),
        );
        let d818_dn4: f64 = v13;
        let d818_dn6: f64 = v1958;
        stamper.stamp_current_node2_local(
            Some(4),
            Some(6),
            multiplicity * (v818),
            4,
            multiplicity * (d818_dn4),
            6,
            multiplicity * (d818_dn6),
        );
        let d822_dn1: f64 = v1966;
        let d822_dn3: f64 = v1967;
        let d822_dn5: f64 = v1968;
        stamper.stamp_current_node3_local(
            Some(1),
            Some(5),
            multiplicity * (v822),
            1,
            multiplicity * (d822_dn1),
            3,
            multiplicity * (d822_dn3),
            5,
            multiplicity * (d822_dn5),
        );
        stamper.stamp_current_const_local(
            Some(1),
            Some(5),
            multiplicity * (v13),
        );
        stamper.stamp_potential_branch_local(
            Some(1),
            Some(5),
            5,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            5,
            v13,
        );
        let d826_dn2: f64 = v1976;
        let d826_dn3: f64 = v1977;
        let d826_dn6: f64 = v1978;
        stamper.stamp_current_node3_local(
            Some(2),
            Some(6),
            multiplicity * (v826),
            2,
            multiplicity * (d826_dn2),
            3,
            multiplicity * (d826_dn3),
            6,
            multiplicity * (d826_dn6),
        );
        stamper.stamp_current_const_local(
            Some(2),
            Some(6),
            multiplicity * (v13),
        );
        stamper.stamp_potential_branch_local(
            Some(2),
            Some(6),
            6,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            6,
            v13,
        );
        let d831_dn0: f64 = v1986;
        let d831_dn3: f64 = v1987;
        let d831_dn4: f64 = v1988;
        stamper.stamp_current_node3_local(
            Some(0),
            Some(4),
            multiplicity * (v831),
            0,
            multiplicity * (d831_dn0),
            3,
            multiplicity * (d831_dn3),
            4,
            multiplicity * (d831_dn4),
        );
        stamper.stamp_current_const_local(
            Some(0),
            Some(4),
            multiplicity * (v13),
        );
        stamper.stamp_potential_branch_local(
            Some(0),
            Some(4),
            7,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            7,
            v13,
        );
        let d833_dn3: f64 = v1993;
        let d833_dn4: f64 = v1994;
        let d833_dn5: f64 = v1995;
        let d833_dn6: f64 = v1996;
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(5),
            Some(6),
            multiplicity * (v833),
            [3, 4, 5, 6],
            [d833_dn3, d833_dn4, d833_dn5, d833_dn6],
            [],
            [],
            multiplicity,
        );
        let d835_dn3: f64 = v2001;
        let d835_dn4: f64 = v2002;
        let d835_dn5: f64 = v2003;
        let d835_dn6: f64 = v2004;
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(5),
            Some(4),
            multiplicity * (v835),
            [3, 4, 5, 6],
            [d835_dn3, d835_dn4, d835_dn5, d835_dn6],
            [],
            [],
            multiplicity,
        );
        let d838_dn3: f64 = v2013;
        let d838_dn4: f64 = v2014;
        let d838_dn5: f64 = v2015;
        let d838_dn6: f64 = v2016;
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(4),
            Some(6),
            multiplicity * (v838),
            [3, 4, 5, 6],
            [d838_dn3, d838_dn4, d838_dn5, d838_dn6],
            [],
            [],
            multiplicity,
        );
        let d840_dn3: f64 = v2021;
        let d840_dn4: f64 = v2022;
        let d840_dn5: f64 = v2023;
        let d840_dn6: f64 = v2024;
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(4),
            Some(6),
            multiplicity * (v840),
            [3, 4, 5, 6],
            [d840_dn3, d840_dn4, d840_dn5, d840_dn6],
            [],
            [],
            multiplicity,
        );
        let d842_dn3: f64 = v2028;
        let d842_dn5: f64 = v2029;
        let d842_dn6: f64 = v2030;
        let v842_ddt: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 5, v842);
        stamper.stamp_current_node3_local(
            Some(5),
            Some(6),
            multiplicity * (v842_ddt),
            3,
            multiplicity * (((d842_dn3) * ddt_scale)),
            5,
            multiplicity * (((d842_dn5) * ddt_scale)),
            6,
            multiplicity * (((d842_dn6) * ddt_scale)),
        );
        let d844_dn3: f64 = v2034;
        let d844_dn5: f64 = v2035;
        let d844_dn6: f64 = v2036;
        let v844_ddt: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 6, v844);
        stamper.stamp_current_node3_local(
            Some(5),
            Some(6),
            multiplicity * (v844_ddt),
            3,
            multiplicity * (((d844_dn3) * ddt_scale)),
            5,
            multiplicity * (((d844_dn5) * ddt_scale)),
            6,
            multiplicity * (((d844_dn6) * ddt_scale)),
        );
        let d846_dn1: f64 = v2042;
        let d846_dn3: f64 = v2043;
        let d846_dn4: f64 = v2044;
        let d846_dn5: f64 = v2045;
        let d846_dn6: f64 = v2046;
        let v846_ddt: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 7, v846);
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(1),
            Some(4),
            multiplicity * (v846_ddt),
            [1, 3, 4, 5, 6],
            [((d846_dn1) * ddt_scale), ((d846_dn3) * ddt_scale), ((d846_dn4) * ddt_scale), ((d846_dn5) * ddt_scale), ((d846_dn6) * ddt_scale)],
            [],
            [],
            multiplicity,
        );
        let d848_dn1: f64 = v2052;
        let d848_dn3: f64 = v2053;
        let d848_dn4: f64 = v2054;
        let d848_dn5: f64 = v2055;
        let d848_dn6: f64 = v2056;
        let v848_ddt: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 8, v848);
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(5),
            Some(4),
            multiplicity * (v848_ddt),
            [1, 3, 4, 5, 6],
            [((d848_dn1) * ddt_scale), ((d848_dn3) * ddt_scale), ((d848_dn4) * ddt_scale), ((d848_dn5) * ddt_scale), ((d848_dn6) * ddt_scale)],
            [],
            [],
            multiplicity,
        );
        let d850_dn3: f64 = v2061;
        let d850_dn4: f64 = v2062;
        let d850_dn5: f64 = v2063;
        let d850_dn6: f64 = v2064;
        let v850_ddt: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 9, v850);
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(5),
            Some(4),
            multiplicity * (v850_ddt),
            [3, 4, 5, 6],
            [((d850_dn3) * ddt_scale), ((d850_dn4) * ddt_scale), ((d850_dn5) * ddt_scale), ((d850_dn6) * ddt_scale)],
            [],
            [],
            multiplicity,
        );
        let d852_dn2: f64 = v2068;
        let d852_dn3: f64 = v2069;
        let d852_dn4: f64 = v2070;
        let v852_ddt: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 10, v852);
        stamper.stamp_current_node3_local(
            Some(2),
            Some(4),
            multiplicity * (v852_ddt),
            2,
            multiplicity * (((d852_dn2) * ddt_scale)),
            3,
            multiplicity * (((d852_dn3) * ddt_scale)),
            4,
            multiplicity * (((d852_dn4) * ddt_scale)),
        );
        let d854_dn3: f64 = v2075;
        let d854_dn4: f64 = v2076;
        let d854_dn5: f64 = v2077;
        let d854_dn6: f64 = v2078;
        let v854_ddt: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 11, v854);
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(5),
            Some(6),
            multiplicity * (v854_ddt),
            [3, 4, 5, 6],
            [((d854_dn3) * ddt_scale), ((d854_dn4) * ddt_scale), ((d854_dn5) * ddt_scale), ((d854_dn6) * ddt_scale)],
            [],
            [],
            multiplicity,
        );
        let d855_dn3: f64 = v2079;
        let d855_dn4: f64 = v2080;
        let d855_dn5: f64 = v2081;
        let d855_dn6: f64 = v2082;
        let v855_ddt: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 12, v855);
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(5),
            Some(4),
            multiplicity * (v855_ddt),
            [3, 4, 5, 6],
            [((d855_dn3) * ddt_scale), ((d855_dn4) * ddt_scale), ((d855_dn5) * ddt_scale), ((d855_dn6) * ddt_scale)],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_const_local(
            Some(5),
            Some(6),
            multiplicity * (v13),
        );
        stamper.stamp_current_const_local(
            Some(5),
            Some(6),
            multiplicity * (v13),
        );
        stamper.stamp_current_const_local(
            Some(4),
            Some(6),
            multiplicity * (v13),
        );
    }

    pub fn stamp_reactive(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedReactiveStamper<'_>) {
        let nodes = self.nodes;
        let branches = self.branches;
        let p = &(*self.params);
        let multiplicity = self.multiplicity;
        let v1: f64 = ctx.node_voltage(nodes[3]);
        let v4: f64 = ((ctx.temperature() + v1) + self.scalar_v3);
        let v6: f64 = 1300.0;
        let v7: f64 = 173.14999999999998;
        let v8: bool = (v4 > v7);
        let v9: f64 = (if v8 { v4 } else { v7 });
        let v10: bool = (v6 < v9);
        let v11: f64 = (if v10 { v6 } else { v9 });
        let v12: f64 = 1.0;
        let v13: f64 = 0.0;
        let v18: f64 = ctx.node_voltage(nodes[5]);
        let v19: f64 = ctx.node_voltage(nodes[4]);
        let v21: f64 = (self.scalar_v17 * (v18 - v19));
        let v32: f64 = 8.6170869e-5;
        let v33: f64 = (v11 * v32);
        let v34: f64 = (v11 / self.scalar_v31);
        let v35: f64 = ((v34) as f64).ln();
        let v63: f64 = (v34 - v12);
        let v64: f64 = (self.scalar_v62 * v63);
        let v66: f64 = ((v35 * self.scalar_v60) + (v64 / v33));
        let v70: f64 = ((v66) as f64).exp();
        let v71: f64 = (self.scalar_v69 * v70);
        let v91: f64 = (self.scalar_v87 * (v12 + (v63 * self.scalar_v88)));
        let v96: f64 = (self.scalar_v92 * (v12 + (v63 * self.scalar_v93)));
        let v101: f64 = (self.scalar_v97 * (v12 + (v63 * self.scalar_v98)));
        let v106: f64 = (self.scalar_v102 * (v12 + (v63 * self.scalar_v103)));
        let v110: f64 = 300.15;
        let v112: f64 = (v11 / v110);
        let v114: f64 = 0.000702;
        let v115: f64 = (v11 * v114);
        let v116: f64 = (v11 * v115);
        let v118: f64 = (v11 + 1108.0);
        let v121: f64 = (-(1.16 - (v116 / v118)));
        let v122: f64 = 1.3806226e-23;
        let v124: f64 = (v122 * (v11 + v11));
        let v129: f64 = (-(v33 + v33));
        let v130: f64 = 1.5;
        let v133: f64 = 1.6021918e-19;
        let v135: f64 = ((v130 * ((v112) as f64).ln()) + (((v121 / v124) + 1.3454442398941469e20) * v133));
        let v136: f64 = (v129 * v135);
        let v139: f64 = ((self.scalar_v137 - v136) / self.scalar_v111);
        let v140: f64 = (self.scalar_v137 - v139);
        let v143: f64 = 0.0004;
        let v148: f64 = (v12 + (self.scalar_v142 * (self.scalar_v145 - (v140 / v139))));
        let v149: f64 = (self.scalar_v107 / v148);
        let v151: f64 = (v136 + (v112 * v139));
        let v152: f64 = (v151 - v139);
        let v155: f64 = (v143 * (v11 - v110));
        let v158: f64 = (v12 + (self.scalar_v142 * (v155 - (v152 / v139))));
        let v159: f64 = (v149 * v158);
        let v162: f64 = ((self.scalar_v160 - v136) / self.scalar_v111);
        let v163: f64 = (self.scalar_v160 - v162);
        let v168: f64 = (v12 + (self.scalar_v165 * (self.scalar_v145 - (v163 / v162))));
        let v169: f64 = (self.scalar_v108 / v168);
        let v171: f64 = (v136 + (v112 * v162));
        let v172: f64 = (v171 - v162);
        let v176: f64 = (v12 + (self.scalar_v165 * (v155 - (v172 / v162))));
        let v177: f64 = (v169 * v176);
        let v180: f64 = ((self.scalar_v178 - v136) / self.scalar_v111);
        let v181: f64 = (self.scalar_v178 - v180);
        let v186: f64 = (v12 + (self.scalar_v183 * (self.scalar_v145 - (v181 / v180))));
        let v187: f64 = (self.scalar_v109 / v186);
        let v189: f64 = (v136 + (v112 * v180));
        let v190: f64 = (v189 - v180);
        let v194: f64 = (v12 + (self.scalar_v183 * (v155 - (v190 / v180))));
        let v195: f64 = (v187 * v194);
        let v196: f64 = ctx.node_voltage(nodes[2]);
        let v198: f64 = (self.scalar_v17 * (v196 - v19));
        let v201: f64 = (self.scalar_v17 * (v18 - ctx.node_voltage(nodes[6])));
        let v202: f64 = ctx.node_voltage(nodes[1]);
        let v204: f64 = (self.scalar_v17 * (v202 - v19));
        let v209: bool = (v71 > v13);
        let v211: f64 = (v33 * self.scalar_v210);
        let v213: f64 = (if v209 { (v201 / v211) } else { v13 });
        let v214: f64 = (-v201);
        let v215: f64 = (v214 - v96);
        let v217: f64 = (v33 * self.scalar_v216);
        let v219: f64 = (if v209 { (v215 / v217) } else { v13 });
        let v220: f64 = (-v96);
        let v222: f64 = (if v209 { (v220 / v217) } else { v13 });
        let v223: f64 = 80.0;
        let v224: bool = (v213 > v223);
        let v225: bool = (v209 && v224);
        let v229: f64 = (if v225 { v223 } else { v213 });
        let v231: bool = (v209 && (!v224));
        let v232: f64 = (if v231 { v12 } else { (if v225 { (v12 + (v213 - v223)) } else { v13 }) });
        let v233: f64 = ((v229) as f64).exp();
        let v235: f64 = (if v209 { (v232 * v233) } else { v232 });
        let v236: f64 = 37.0;
        let v237: bool = (v219 >= v236);
        let v238: bool = (!v237);
        let v239: f64 = -37.0;
        let v240: bool = (v219 <= v239);
        let v242: bool = (v238 && (!v240));
        let v243: f64 = ((v219) as f64).exp();
        let v244: f64 = (v12 + v243);
        let v246: bool = (v238 && v240);
        let v250: bool = (v222 >= v236);
        let v251: bool = (!v250);
        let v252: bool = (v222 <= v239);
        let v254: bool = (v251 && (!v252));
        let v255: f64 = ((v222) as f64).exp();
        let v256: f64 = (v12 + v255);
        let v258: bool = (v251 && v252);
        let v263: f64 = (if v209 { ((if v242 { ((v244) as f64).ln() } else { (if v246 { v243 } else { (if v237 { v219 } else { v13 }) }) }) - (if v254 { ((v256) as f64).ln() } else { (if v258 { v255 } else { (if v250 { v222 } else { v13 }) }) })) } else { v13 });
        let v264: f64 = (v235 - v12);
        let v266: f64 = (v91 * v263);
        let v268: f64 = ((v201) as f64).abs();
        let v269: f64 = f64::powf(v268, v101);
        let v271: f64 = (v12 + (self.scalar_v267 * v269));
        let v275: bool = (!v209);
        let v276: f64 = (if v275 { v13 } else { (if v209 { ((v71 * v264) - (v266 / v271)) } else { v13 }) });
        let v277: bool = ((self.scalar_v72 * (((v35 * self.scalar_v67)) as f64).exp()) > v13);
        let v279: f64 = (self.scalar_v278 - v201);
        let v280: f64 = 0.001;
        let v281: bool = (v279 > v280);
        let v283: f64 = (if v277 { (if v281 { v279 } else { v280 }) } else { v13 });
        let v285: f64 = (v214 * self.scalar_v278);
        let v287: f64 = (v33 * self.scalar_v286);
        let v288: f64 = (v283 * v287);
        let v290: f64 = (if v277 { (v285 / v288) } else { v229 });
        let v291: bool = (v290 > v223);
        let v292: bool = (v277 && v291);
        let v296: f64 = (if v292 { v223 } else { v290 });
        let v298: bool = (v277 && (!v291));
        let v299: f64 = (if v298 { v12 } else { (if v292 { (v12 + (v290 - v223)) } else { v235 }) });
        let v300: f64 = ((v296) as f64).exp();
        let v308: bool = (((self.scalar_v75 * (((v66 / self.scalar_v76)) as f64).exp()) / (((v35 * self.scalar_v36)) as f64).exp()) > v13);
        let v309: f64 = (v33 * self.scalar_v76);
        let v311: f64 = (if v308 { (v201 / v309) } else { v296 });
        let v313: f64 = (v33 * self.scalar_v312);
        let v315: f64 = (if v308 { (v215 / v313) } else { v219 });
        let v316: f64 = (v220 / v313);
        let v317: f64 = (if v308 { v316 } else { v222 });
        let v318: bool = (v311 > v223);
        let v319: bool = (v308 && v318);
        let v323: f64 = (if v319 { v223 } else { v311 });
        let v325: bool = (v308 && (!v318));
        let v326: f64 = (if v325 { v12 } else { (if v319 { (v12 + (v311 - v223)) } else { (if v277 { (v299 * v300) } else { v299 }) }) });
        let v327: f64 = ((v323) as f64).exp();
        let v330: bool = (v315 >= v236);
        let v331: bool = (!v330);
        let v332: bool = (v315 <= v239);
        let v334: bool = (v331 && (!v332));
        let v335: f64 = ((v315) as f64).exp();
        let v336: f64 = (v12 + v335);
        let v338: bool = (v331 && v332);
        let v342: bool = (v317 >= v236);
        let v343: bool = (!v342);
        let v344: bool = (v317 <= v239);
        let v346: bool = (v343 && (!v344));
        let v347: f64 = ((v317) as f64).exp();
        let v348: f64 = (v12 + v347);
        let v350: bool = (v343 && v344);
        let v355: f64 = (if v308 { ((if v334 { ((v336) as f64).ln() } else { (if v338 { v335 } else { (if v330 { v315 } else { v13 }) }) }) - (if v346 { ((v348) as f64).ln() } else { (if v350 { v347 } else { (if v342 { v317 } else { v13 }) }) })) } else { v263 });
        let v365: f64 = (v33 * self.scalar_v364);
        let v367: f64 = (if v209 { (v21 / v365) } else { v323 });
        let v369: f64 = ((-v21) - v96);
        let v371: f64 = (if v209 { (v369 / v313) } else { v315 });
        let v372: f64 = (if v209 { v316 } else { v317 });
        let v373: bool = (v367 > v223);
        let v374: bool = (v209 && v373);
        let v380: bool = (v209 && (!v373));
        let v381: f64 = (if v380 { v12 } else { (if v374 { (v12 + (v367 - v223)) } else { (if v308 { (v326 * v327) } else { v326 }) }) });
        let v382: f64 = (((if v374 { v223 } else { v367 })) as f64).exp();
        let v385: bool = (v371 >= v236);
        let v386: bool = (!v385);
        let v387: bool = (v371 <= v239);
        let v389: bool = (v386 && (!v387));
        let v390: f64 = ((v371) as f64).exp();
        let v391: f64 = (v12 + v390);
        let v393: bool = (v386 && v387);
        let v397: bool = (v372 >= v236);
        let v398: bool = (!v397);
        let v399: bool = (v372 <= v239);
        let v401: bool = (v398 && (!v399));
        let v402: f64 = ((v372) as f64).exp();
        let v403: f64 = (v12 + v402);
        let v405: bool = (v398 && v399);
        let v410: f64 = (if v209 { ((if v389 { ((v391) as f64).ln() } else { (if v393 { v390 } else { (if v385 { v371 } else { v13 }) }) }) - (if v401 { ((v403) as f64).ln() } else { (if v405 { v402 } else { (if v397 { v372 } else { v13 }) }) })) } else { v355 });
        let v411: f64 = ((if v209 { (v381 * v382) } else { v381 }) - v12);
        let v413: f64 = (v106 * v410);
        let v414: f64 = ((v21) as f64).abs();
        let v415: f64 = f64::powf(v414, v101);
        let v417: f64 = (v12 + (self.scalar_v267 * v415));
        let v421: f64 = (if v275 { v13 } else { (if v209 { ((v71 * v411) - (v413 / v417)) } else { v13 }) });
        let v507: f64 = (v12 + f64::powf((((v12 + (((v276 * (self.scalar_v55 * (v12 + (v21 * self.scalar_v490)))) + (self.scalar_v59 * v421)) * 4.0))) as f64).abs(), self.scalar_v505));
        let v510: f64 = ((((v12 - (self.scalar_v51 * v201)) - (v21 * self.scalar_v47)) * 2.0) / v507);
        let v567: f64 = (self.scalar_v563 * (v12 + ((f64::powf((v12 + f64::powf(((((v202 - v196) / self.scalar_v554)) as f64).abs(), self.scalar_v557)), self.scalar_v560) - v12) * self.scalar_v564)));
        let v593: bool = (v198 <= v13);
        let v594: f64 = (v189 * v195);
        let v597: f64 = (v12 - (v198 / v189));
        let v600: f64 = (((self.scalar_v595 * ((v597) as f64).ln())) as f64).exp();
        let v601: f64 = (v12 - v600);
        let v605: bool = (!v593);
        let v606: f64 = (v195 * v198);
        let v609: f64 = (v198 * self.scalar_v608);
        let v611: f64 = (v12 + (v609 / v189));
        let v617: f64 = (v201 + ((-v151) * self.scalar_v615));
        let v618: bool = (v617 > v13);
        let v624: f64 = (if v618 { self.scalar_v623 } else { v13 });
        let v627: f64 = (v12 - (self.scalar_v620 * (self.scalar_v620 * v624)));
        let v633: f64 = (v617 * self.scalar_v632);
        let v635: f64 = (self.scalar_v620 + (v633 / v151));
        let v639: bool = (!v618);
        let v641: f64 = (v12 - (v201 / v151));
        let v644: f64 = (((self.scalar_v629 * ((v641) as f64).ln())) as f64).exp();
        let v645: f64 = (v12 - v644);
        let v648: f64 = (if v639 { ((v151 * v645) / self.scalar_v629) } else { (if v618 { ((v151 * v627) / self.scalar_v629) } else { v13 }) });
        let v649: f64 = (if v639 { v13 } else { (if v618 { (v624 * (v617 * v635)) } else { v13 }) });
        let v650: f64 = (v648 + v649);
        let v653: f64 = (self.scalar_v615 * (-v171));
        let v654: f64 = (v204 + v653);
        let v655: bool = (v654 > v13);
        let v659: f64 = (if v655 { self.scalar_v658 } else { v624 });
        let v662: f64 = (v12 - (self.scalar_v620 * (self.scalar_v620 * v659)));
        let v668: f64 = (v654 * self.scalar_v667);
        let v670: f64 = (self.scalar_v620 + (v668 / v171));
        let v674: bool = (!v655);
        let v676: f64 = (v12 - (v204 / v171));
        let v679: f64 = (((self.scalar_v664 * ((v676) as f64).ln())) as f64).exp();
        let v680: f64 = (v12 - v679);
        let v683: f64 = (if v674 { ((v171 * v680) / self.scalar_v664) } else { (if v655 { ((v171 * v662) / self.scalar_v664) } else { v648 }) });
        let v684: f64 = (if v674 { v13 } else { (if v655 { (v659 * (v654 * v670)) } else { v649 }) });
        let v685: f64 = (v683 + v684);
        let v690: f64 = (v21 + v653);
        let v691: bool = (v690 > v13);
        let v692: f64 = (if v691 { self.scalar_v658 } else { v659 });
        let v695: f64 = (v12 - (self.scalar_v620 * (self.scalar_v620 * v692)));
        let v699: f64 = (self.scalar_v667 * v690);
        let v701: f64 = (self.scalar_v620 + (v699 / v171));
        let v705: bool = (!v691);
        let v707: f64 = (v12 - (v21 / v171));
        let v710: f64 = (((self.scalar_v664 * ((v707) as f64).ln())) as f64).exp();
        let v711: f64 = (v12 - v710);
        let v716: f64 = ((if v705 { ((v171 * v711) / self.scalar_v664) } else { (if v691 { ((v171 * v695) / self.scalar_v664) } else { v683 }) }) + (if v705 { v13 } else { (if v691 { (v692 * (v690 * v701)) } else { v684 }) }));
        let v732: f64 = (if self.scalar_v731 { v13 } else { (if self.scalar_v722 { ((v276 * v510) * self.scalar_v728) } else { v13 }) });
        let v771: f64 = 0.0;
        let v772: f64 = (self.scalar_v770 * v771);
        let v778: f64 = 0.0;
        let v780: f64 = (if self.scalar_v572 { (v567 * v778) } else { v13 });
        let v793: f64 = (v1 * self.scalar_v792);
        let v794: f64 = 0.0;
        let v795: f64 = (if self.scalar_v737 { v794 } else { v13 });
        let v803: f64 = 0.0;
        let v804: f64 = (if self.scalar_v797 { v803 } else { v13 });
        let v809: f64 = 0.0;
        let v810: f64 = (if self.scalar_v797 { v809 } else { v13 });
        let v842: f64 = (self.scalar_v16 * (self.scalar_v17 * (v159 * v650)));
        let v844: f64 = (self.scalar_v16 * (self.scalar_v17 * (v276 * v567)));
        let v846: f64 = (self.scalar_v16 * (self.scalar_v17 * ((v177 * v685) * self.scalar_v688)));
        let v848: f64 = (self.scalar_v16 * (self.scalar_v17 * (self.scalar_v687 * (v177 * v716))));
        let v850: f64 = (self.scalar_v16 * (self.scalar_v17 * ((v421 * v510) * self.scalar_v569)));
        let v852: f64 = (self.scalar_v16 * (self.scalar_v17 * (if v605 { (v606 * v611) } else { (if v593 { ((v594 * v601) / self.scalar_v595) } else { v13 }) })));
        let v854: f64 = (self.scalar_v16 * (-v732));
        let v855: f64 = (self.scalar_v16 * v732);
        let v857: f64 = (if v10 { v13 } else { (if v8 { v12 } else { v13 }) });
        let v870: f64 = (v32 * v857);
        let v871: f64 = (v857 / self.scalar_v31);
        let v890: f64 = (self.scalar_v69 * (v70 * ((self.scalar_v60 * (v871 / v34)) + (((v33 * (self.scalar_v62 * v871)) - (v64 * v870)) / (v33 * v33)))));
        let v913: f64 = (self.scalar_v97 * (self.scalar_v98 * v871));
        let v916: f64 = (v857 / v110);
        let v930: f64 = ((v124 * (((v118 * ((v115 * v857) + (v11 * (v114 * v857)))) - (v116 * v857)) / (v118 * v118))) - (v121 * (v122 * (v857 + v857))));
        let v941: f64 = ((v135 * (-(v870 + v870))) + (v129 * ((v130 * (v916 / v112)) + (v133 * (v930 / (v124 * v124))))));
        let v943: f64 = ((-v941) / self.scalar_v111);
        let v944: f64 = (-v943);
        let v948: f64 = (v139 * v139);
        let v957: f64 = (v112 * v943);
        let v959: f64 = (v941 + ((v139 * v916) + v957));
        let v965: f64 = (v143 * v857);
        let v970: f64 = ((v158 * ((-(self.scalar_v107 * (self.scalar_v142 * (-(((v139 * v944) - (v140 * v943)) / v948))))) / (v148 * v148))) + (v149 * (self.scalar_v142 * (v965 - (((v139 * (v959 - v943)) - (v152 * v943)) / v948)))));
        let v974: f64 = (v162 * v162);
        let v984: f64 = (v941 + (v957 + (v162 * v916)));
        let v994: f64 = ((v176 * ((-(self.scalar_v108 * (self.scalar_v165 * (-(((v162 * v944) - (v163 * v943)) / v974))))) / (v168 * v168))) + (v169 * (self.scalar_v165 * (v965 - (((v162 * (v984 - v943)) - (v172 * v943)) / v974)))));
        let v998: f64 = (v180 * v180);
        let v1008: f64 = (v941 + (v957 + (v180 * v916)));
        let v1018: f64 = ((v194 * ((-(self.scalar_v109 * (self.scalar_v183 * (-(((v180 * v944) - (v181 * v943)) / v998))))) / (v186 * v186))) + (v187 * (self.scalar_v183 * (v965 - (((v180 * (v1008 - v943)) - (v190 * v943)) / v998)))));
        let v1026: f64 = (if v209 { ((-(v201 * (self.scalar_v210 * v870))) / (v211 * v211)) } else { v13 });
        let v1027: f64 = (if v209 { (self.scalar_v17 / v211) } else { v13 });
        let v1028: f64 = (if v209 { (self.scalar_v858 / v211) } else { v13 });
        let v1029: f64 = (-(self.scalar_v92 * (self.scalar_v93 * v871)));
        let v1030: f64 = (self.scalar_v216 * v870);
        let v1031: f64 = (v217 * v1029);
        let v1034: f64 = (v217 * v217);
        let v1038: f64 = (if v209 { ((v1031 - (v215 * v1030)) / v1034) } else { v13 });
        let v1039: f64 = (if v209 { (self.scalar_v858 / v217) } else { v13 });
        let v1040: f64 = (if v209 { (self.scalar_v17 / v217) } else { v13 });
        let v1044: f64 = (if v209 { ((v1031 - (v220 * v1030)) / v1034) } else { v13 });
        let v1048: f64 = (if v225 { v13 } else { v1026 });
        let v1049: f64 = (if v225 { v13 } else { v1027 });
        let v1050: f64 = (if v225 { v13 } else { v1028 });
        let v1051: f64 = (if v231 { v13 } else { (if v225 { v1026 } else { v13 }) });
        let v1052: f64 = (if v231 { v13 } else { (if v225 { v1027 } else { v13 }) });
        let v1053: f64 = (if v231 { v13 } else { (if v225 { v1028 } else { v13 }) });
        let v1066: f64 = (if v209 { ((v233 * v1051) + (v232 * (v233 * v1048))) } else { v1051 });
        let v1067: f64 = (if v209 { ((v233 * v1052) + (v232 * (v233 * v1049))) } else { v1052 });
        let v1068: f64 = (if v209 { ((v233 * v1053) + (v232 * (v233 * v1050))) } else { v1053 });
        let v1069: f64 = (v243 * v1038);
        let v1070: f64 = (v243 * v1039);
        let v1071: f64 = (v243 * v1040);
        let v1084: f64 = (v255 * v1044);
        let v1089: f64 = ((if v242 { (v1069 / v244) } else { (if v246 { v1069 } else { (if v237 { v1038 } else { v13 }) }) }) - (if v254 { (v1084 / v256) } else { (if v258 { v1084 } else { (if v250 { v1044 } else { v13 }) }) }));
        let v1090: f64 = (if v209 { v1089 } else { v13 });
        let v1091: f64 = (if v209 { (if v242 { (v1070 / v244) } else { (if v246 { v1070 } else { (if v237 { v1039 } else { v13 }) }) }) } else { v13 });
        let v1092: f64 = (if v209 { (if v242 { (v1071 / v244) } else { (if v246 { v1071 } else { (if v237 { v1040 } else { v13 }) }) }) } else { v13 });
        let v1111: f64 = (((v271 * ((v263 * (self.scalar_v87 * (self.scalar_v88 * v871))) + (v91 * v1090))) - (v266 * (self.scalar_v267 * (v913 * (v269 * ((v268) as f64).ln()))))) / (v271 * v271));
        let v1120: f64 = (if v275 { v13 } else { (if v209 { (((v264 * v890) + (v71 * v1066)) - v1111) } else { v13 }) });
        let v1121: f64 = (if v275 { v13 } else { (if v209 { ((v71 * v1067) - ((v91 * v1091) / v271)) } else { v13 }) });
        let v1122: f64 = (if v275 { v13 } else { (if v209 { ((v71 * v1068) - ((v91 * v1092) / v271)) } else { v13 }) });
        let v1135: f64 = (v288 * v288);
        let v1145: f64 = (if v277 { ((-(v285 * (v283 * (self.scalar_v286 * v870)))) / v1135) } else { v1048 });
        let v1146: f64 = (if v277 { (((v288 * self.scalar_v1127) - (v285 * (v287 * (if v277 { (if v281 { self.scalar_v858 } else { v13 }) } else { v13 })))) / v1135) } else { v1049 });
        let v1147: f64 = (if v277 { (((v288 * self.scalar_v1128) - (v285 * (v287 * (if v277 { (if v281 { self.scalar_v17 } else { v13 }) } else { v13 })))) / v1135) } else { v1050 });
        let v1151: f64 = (if v292 { v13 } else { v1145 });
        let v1152: f64 = (if v292 { v13 } else { v1146 });
        let v1153: f64 = (if v292 { v13 } else { v1147 });
        let v1154: f64 = (if v298 { v13 } else { (if v292 { v1145 } else { v1066 }) });
        let v1155: f64 = (if v298 { v13 } else { (if v292 { v1146 } else { v1067 }) });
        let v1156: f64 = (if v298 { v13 } else { (if v292 { v1147 } else { v1068 }) });
        let v1190: f64 = (if v308 { ((-(v201 * (self.scalar_v76 * v870))) / (v309 * v309)) } else { v1151 });
        let v1191: f64 = (if v308 { (self.scalar_v17 / v309) } else { v1152 });
        let v1192: f64 = (if v308 { (self.scalar_v858 / v309) } else { v1153 });
        let v1193: f64 = (self.scalar_v312 * v870);
        let v1194: f64 = (v313 * v1029);
        let v1197: f64 = (v313 * v313);
        let v1199: f64 = (self.scalar_v858 / v313);
        let v1200: f64 = (self.scalar_v17 / v313);
        let v1201: f64 = (if v308 { ((v1194 - (v215 * v1193)) / v1197) } else { v1038 });
        let v1202: f64 = (if v308 { v1199 } else { v1039 });
        let v1203: f64 = (if v308 { v1200 } else { v1040 });
        let v1206: f64 = ((v1194 - (v220 * v1193)) / v1197);
        let v1207: f64 = (if v308 { v1206 } else { v1044 });
        let v1211: f64 = (if v319 { v13 } else { v1190 });
        let v1212: f64 = (if v319 { v13 } else { v1191 });
        let v1213: f64 = (if v319 { v13 } else { v1192 });
        let v1214: f64 = (if v325 { v13 } else { (if v319 { v1190 } else { (if v277 { ((v300 * v1154) + (v299 * (v300 * v1151))) } else { v1154 }) }) });
        let v1215: f64 = (if v325 { v13 } else { (if v319 { v1191 } else { (if v277 { ((v300 * v1155) + (v299 * (v300 * v1152))) } else { v1155 }) }) });
        let v1216: f64 = (if v325 { v13 } else { (if v319 { v1192 } else { (if v277 { ((v300 * v1156) + (v299 * (v300 * v1153))) } else { v1156 }) }) });
        let v1232: f64 = (v335 * v1201);
        let v1233: f64 = (v335 * v1202);
        let v1234: f64 = (v335 * v1203);
        let v1247: f64 = (v347 * v1207);
        let v1252: f64 = ((if v334 { (v1232 / v336) } else { (if v338 { v1232 } else { (if v330 { v1201 } else { v13 }) }) }) - (if v346 { (v1247 / v348) } else { (if v350 { v1247 } else { (if v342 { v1207 } else { v13 }) }) }));
        let v1286: f64 = (if v209 { ((-(v21 * (self.scalar_v364 * v870))) / (v365 * v365)) } else { v1211 });
        let v1287: f64 = (if v209 { (self.scalar_v858 / v365) } else { v13 });
        let v1288: f64 = (if v209 { (self.scalar_v17 / v365) } else { v1212 });
        let v1289: f64 = (if v209 { v13 } else { v1213 });
        let v1293: f64 = (if v209 { ((v1194 - (v369 * v1193)) / v1197) } else { v1201 });
        let v1294: f64 = (if v209 { v1200 } else { v13 });
        let v1295: f64 = (if v209 { v1199 } else { v1202 });
        let v1296: f64 = (if v209 { v13 } else { v1203 });
        let v1297: f64 = (if v209 { v1206 } else { v1207 });
        let v1306: f64 = (if v380 { v13 } else { (if v374 { v1286 } else { (if v308 { ((v327 * v1214) + (v326 * (v327 * v1211))) } else { v1214 }) }) });
        let v1307: f64 = (if v380 { v13 } else { (if v374 { v1287 } else { v13 }) });
        let v1308: f64 = (if v380 { v13 } else { (if v374 { v1288 } else { (if v308 { ((v327 * v1215) + (v326 * (v327 * v1212))) } else { v1215 }) }) });
        let v1309: f64 = (if v380 { v13 } else { (if v374 { v1289 } else { (if v308 { ((v327 * v1216) + (v326 * (v327 * v1213))) } else { v1216 }) }) });
        let v1330: f64 = (v390 * v1293);
        let v1331: f64 = (v390 * v1294);
        let v1332: f64 = (v390 * v1295);
        let v1333: f64 = (v390 * v1296);
        let v1350: f64 = (v402 * v1297);
        let v1355: f64 = ((if v389 { (v1330 / v391) } else { (if v393 { v1330 } else { (if v385 { v1293 } else { v13 }) }) }) - (if v401 { (v1350 / v403) } else { (if v405 { v1350 } else { (if v397 { v1297 } else { v13 }) }) }));
        let v1358: f64 = (if v209 { (if v389 { (v1332 / v391) } else { (if v393 { v1332 } else { (if v385 { v1295 } else { v13 }) }) }) } else { (if v308 { (if v334 { (v1233 / v336) } else { (if v338 { v1233 } else { (if v330 { v1202 } else { v13 }) }) }) } else { v1091 }) });
        let v1359: f64 = (if v209 { (if v389 { (v1333 / v391) } else { (if v393 { v1333 } else { (if v385 { v1296 } else { v13 }) }) }) } else { (if v308 { (if v334 { (v1234 / v336) } else { (if v338 { v1234 } else { (if v330 { v1203 } else { v13 }) }) }) } else { v1092 }) });
        let v1378: f64 = ((v417 * ((v410 * (self.scalar_v102 * (self.scalar_v103 * v871))) + (v106 * (if v209 { v1355 } else { (if v308 { v1252 } else { v1090 }) })))) - (v413 * (self.scalar_v267 * (v913 * (v415 * ((v414) as f64).ln())))));
        let v1384: f64 = (((v411 * v890) + (v71 * (if v209 { ((v382 * v1306) + (v381 * (v382 * (if v374 { v13 } else { v1286 })))) } else { v1306 }))) - (v1378 / (v417 * v417)));
        let v1385: f64 = ((v71 * (if v209 { ((v382 * v1307) + (v381 * (v382 * (if v374 { v13 } else { v1287 })))) } else { v1307 })) - ((v106 * (if v209 { (if v389 { (v1331 / v391) } else { (if v393 { v1331 } else { (if v385 { v1294 } else { v13 }) }) }) } else { v13 })) / v417));
        let v1390: f64 = (if v209 { ((v71 * (if v209 { ((v382 * v1308) + (v381 * (v382 * (if v374 { v13 } else { v1288 })))) } else { v1308 })) - ((v106 * v1358) / v417)) } else { v13 });
        let v1391: f64 = (if v209 { ((v71 * (if v209 { ((v382 * v1309) + (v381 * (v382 * (if v374 { v13 } else { v1289 })))) } else { v1309 })) - ((v106 * v1359) / v417)) } else { v13 });
        let v1542: f64 = (self.scalar_v1539 / v507);
        let v1543: f64 = (self.scalar_v1540 / v507);
        let v1544: f64 = (self.scalar_v1541 / v507);
        let v1612: f64 = (v189 * v189);
        let v1633: f64 = ((v601 * ((v195 * v1008) + (v189 * v1018))) + (v594 * (-(v600 * (self.scalar_v595 * ((-((-(v198 * v1008)) / v1612)) / v597))))));
        let v1660: f64 = (if v605 { ((v611 * (self.scalar_v17 * v195)) + (v606 * (self.scalar_v1644 / v189))) } else { (if v593 { ((v594 * (-(v600 * (self.scalar_v595 * ((-(self.scalar_v17 / v189)) / v597))))) / self.scalar_v595) } else { v13 }) });
        let v1662: f64 = (if v605 { ((v611 * (v195 * self.scalar_v858)) + (v606 * (self.scalar_v1645 / v189))) } else { (if v593 { ((v594 * (-(v600 * (self.scalar_v595 * ((-(self.scalar_v858 / v189)) / v597))))) / self.scalar_v595) } else { v13 }) });
        let v1664: f64 = (self.scalar_v615 * (-v959));
        let v1674: f64 = (v151 * v151);
        let v1721: f64 = (if v639 { (((v645 * v959) + (v151 * (-(v644 * (self.scalar_v629 * ((-((-(v201 * v959)) / v1674)) / v641)))))) / self.scalar_v629) } else { (if v618 { ((v627 * v959) / self.scalar_v629) } else { v13 }) });
        let v1722: f64 = (if v639 { ((v151 * (-(v644 * (self.scalar_v629 * ((-(self.scalar_v17 / v151)) / v641))))) / self.scalar_v629) } else { v13 });
        let v1723: f64 = (if v639 { ((v151 * (-(v644 * (self.scalar_v629 * ((-(self.scalar_v858 / v151)) / v641))))) / self.scalar_v629) } else { v13 });
        let v1724: f64 = (if v639 { v13 } else { (if v618 { (v624 * ((v635 * v1664) + (v617 * (((v151 * (self.scalar_v632 * v1664)) - (v633 * v959)) / v1674)))) } else { v13 }) });
        let v1725: f64 = (if v639 { v13 } else { (if v618 { (v624 * ((self.scalar_v17 * v635) + (v617 * (self.scalar_v1669 / v151)))) } else { v13 }) });
        let v1726: f64 = (if v639 { v13 } else { (if v618 { (v624 * ((v635 * self.scalar_v858) + (v617 * (self.scalar_v1670 / v151)))) } else { v13 }) });
        let v1736: f64 = (self.scalar_v615 * (-v984));
        let v1745: f64 = (self.scalar_v1742 / v171);
        let v1746: f64 = (v171 * (self.scalar_v667 * v1736));
        let v1749: f64 = (v171 * v171);
        let v1751: f64 = (self.scalar_v1744 / v171);
        let v1774: f64 = (-(self.scalar_v17 / v171));
        let v1776: f64 = (-(self.scalar_v858 / v171));
        let v1797: f64 = (if v674 { ((v171 * (-(v679 * (self.scalar_v664 * (v1774 / v676))))) / self.scalar_v664) } else { v13 });
        let v1798: f64 = (if v674 { (((v680 * v984) + (v171 * (-(v679 * (self.scalar_v664 * ((-((-(v204 * v984)) / v1749)) / v676)))))) / self.scalar_v664) } else { (if v655 { ((v662 * v984) / self.scalar_v664) } else { v1721 }) });
        let v1799: f64 = (if v674 { ((v171 * (-(v679 * (self.scalar_v664 * (v1776 / v676))))) / self.scalar_v664) } else { v13 });
        let v1800: f64 = (if v674 { v13 } else { (if v655 { v13 } else { v1722 }) });
        let v1801: f64 = (if v674 { v13 } else { (if v655 { v13 } else { v1723 }) });
        let v1802: f64 = (if v674 { v13 } else { (if v655 { (v659 * ((self.scalar_v17 * v670) + (v654 * v1745))) } else { v13 }) });
        let v1803: f64 = (if v674 { v13 } else { (if v655 { (v659 * ((v670 * v1736) + (v654 * ((v1746 - (v668 * v984)) / v1749)))) } else { v1724 }) });
        let v1804: f64 = (if v674 { v13 } else { (if v655 { (v659 * ((v670 * self.scalar_v858) + (v654 * v1751))) } else { v13 }) });
        let v1805: f64 = (if v674 { v13 } else { (if v655 { v13 } else { v1725 }) });
        let v1806: f64 = (if v674 { v13 } else { (if v655 { v13 } else { v1726 }) });
        let v1876: f64 = (if v705 { (((v711 * v984) + (v171 * (-(v710 * (self.scalar_v664 * ((-((-(v21 * v984)) / v1749)) / v707)))))) / self.scalar_v664) } else { (if v691 { ((v695 * v984) / self.scalar_v664) } else { v1798 }) });
        let v1887: f64 = ((if v705 { ((v171 * (-(v710 * (self.scalar_v664 * (v1776 / v707))))) / self.scalar_v664) } else { (if v691 { v13 } else { v1799 }) }) + (if v705 { v13 } else { (if v691 { (v692 * ((v701 * self.scalar_v858) + (v690 * v1751))) } else { v1804 }) }));
        let v1888: f64 = ((if v705 { ((v171 * (-(v710 * (self.scalar_v664 * (v1774 / v707))))) / self.scalar_v664) } else { (if v691 { v13 } else { v1800 }) }) + (if v705 { v13 } else { (if v691 { (v692 * ((self.scalar_v17 * v701) + (v690 * v1745))) } else { v1805 }) }));
        let v1892: f64 = (v177 * (v1876 + (if v705 { v13 } else { (if v691 { (v692 * ((v701 * v1736) + (v690 * ((v1746 - (v699 * v984)) / v1749)))) } else { v1803 }) })));
        let v1910: f64 = (if self.scalar_v731 { v13 } else { (if self.scalar_v722 { (self.scalar_v728 * (v510 * v1120)) } else { v13 }) });
        let v1911: f64 = (if self.scalar_v731 { v13 } else { (if self.scalar_v722 { (self.scalar_v728 * (v276 * v1542)) } else { v13 }) });
        let v1912: f64 = (if self.scalar_v731 { v13 } else { (if self.scalar_v722 { (self.scalar_v728 * ((v510 * v1121) + (v276 * v1543))) } else { v13 }) });
        let v1913: f64 = (if self.scalar_v731 { v13 } else { (if self.scalar_v722 { (self.scalar_v728 * ((v510 * v1122) + (v276 * v1544))) } else { v13 }) });
        let v1917: f64 = 1.0;
        let v1918: f64 = (self.scalar_v770 * v1917);
        let v1945: f64 = (if self.scalar_v572 { (v567 * v1917) } else { v13 });
        let v1948: f64 = (self.scalar_v792 * v1917);
        let v1949: f64 = (if self.scalar_v737 { v1948 } else { v13 });
        let v1953: f64 = (if self.scalar_v797 { v1948 } else { v13 });
        let v1957: f64 = (if self.scalar_v797 { (self.scalar_v807 * v1917) } else { v13 });
        let v2028: f64 = (self.scalar_v16 * (self.scalar_v17 * ((v650 * v970) + (v159 * (v1721 + v1724)))));
        let v2029: f64 = (self.scalar_v16 * (self.scalar_v17 * (v159 * (v1722 + v1725))));
        let v2030: f64 = (self.scalar_v16 * (self.scalar_v17 * (v159 * (v1723 + v1726))));
        let v2034: f64 = (self.scalar_v16 * (self.scalar_v17 * (v567 * v1120)));
        let v2035: f64 = (self.scalar_v16 * (self.scalar_v17 * (v567 * v1121)));
        let v2036: f64 = (self.scalar_v16 * (self.scalar_v17 * (v567 * v1122)));
        let v2042: f64 = (self.scalar_v16 * (self.scalar_v17 * (self.scalar_v688 * (v177 * (v1797 + v1802)))));
        let v2043: f64 = (self.scalar_v16 * (self.scalar_v17 * (self.scalar_v688 * ((v685 * v994) + (v177 * (v1798 + v1803))))));
        let v2044: f64 = (self.scalar_v16 * (self.scalar_v17 * (self.scalar_v688 * (v177 * (v1799 + v1804)))));
        let v2045: f64 = (self.scalar_v16 * (self.scalar_v17 * (self.scalar_v688 * (v177 * (v1800 + v1805)))));
        let v2046: f64 = (self.scalar_v16 * (self.scalar_v17 * (self.scalar_v688 * (v177 * (v1801 + v1806)))));
        let v2052: f64 = (self.scalar_v16 * (self.scalar_v17 * (self.scalar_v687 * (v177 * ((if v705 { v13 } else { (if v691 { v13 } else { v1797 }) }) + (if v705 { v13 } else { (if v691 { v13 } else { v1802 }) }))))));
        let v2053: f64 = (self.scalar_v16 * (self.scalar_v17 * (self.scalar_v687 * ((v716 * v994) + v1892))));
        let v2054: f64 = (self.scalar_v16 * (self.scalar_v17 * (self.scalar_v687 * (v177 * v1887))));
        let v2055: f64 = (self.scalar_v16 * (self.scalar_v17 * (self.scalar_v687 * (v177 * v1888))));
        let v2056: f64 = (self.scalar_v16 * (self.scalar_v17 * (self.scalar_v687 * (v177 * ((if v705 { v13 } else { (if v691 { v13 } else { v1801 }) }) + (if v705 { v13 } else { (if v691 { v13 } else { v1806 }) }))))));
        let v2061: f64 = (self.scalar_v16 * (self.scalar_v17 * (self.scalar_v569 * (v510 * (if v275 { v13 } else { (if v209 { v1384 } else { v13 }) })))));
        let v2062: f64 = (self.scalar_v16 * (self.scalar_v17 * (self.scalar_v569 * ((v510 * (if v275 { v13 } else { (if v209 { v1385 } else { v13 }) })) + (v421 * v1542)))));
        let v2063: f64 = (self.scalar_v16 * (self.scalar_v17 * (self.scalar_v569 * ((v510 * (if v275 { v13 } else { v1390 })) + (v421 * v1543)))));
        let v2064: f64 = (self.scalar_v16 * (self.scalar_v17 * (self.scalar_v569 * ((v510 * (if v275 { v13 } else { v1391 })) + (v421 * v1544)))));
        let v2068: f64 = (self.scalar_v16 * (self.scalar_v17 * v1660));
        let v2069: f64 = (self.scalar_v16 * (self.scalar_v17 * (if v605 { ((v611 * (v198 * v1018)) + (v606 * ((-(v609 * v1008)) / v1612))) } else { (if v593 { (v1633 / self.scalar_v595) } else { v13 }) })));
        let v2070: f64 = (self.scalar_v16 * (self.scalar_v17 * v1662));
        let v2075: f64 = (self.scalar_v16 * (-v1910));
        let v2076: f64 = (self.scalar_v16 * (-v1911));
        let v2077: f64 = (self.scalar_v16 * (-v1912));
        let v2078: f64 = (self.scalar_v16 * (-v1913));
        let v2079: f64 = (self.scalar_v16 * v1910);
        let v2080: f64 = (self.scalar_v16 * v1911);
        let v2081: f64 = (self.scalar_v16 * v1912);
        let v2082: f64 = (self.scalar_v16 * v1913);

        let d772_dn9: f64 = v1918;
        stamper.stamp_current_reactive_node1(
            Some(nodes[9]),
            None,
            nodes[9],
            multiplicity * (d772_dn9),
        );
        let d780_dn8: f64 = v1945;
        stamper.stamp_current_reactive_node1(
            Some(nodes[8]),
            None,
            nodes[8],
            multiplicity * (d780_dn8),
        );
        let d795_dn3: f64 = v1949;
        stamper.stamp_current_reactive_node1(
            Some(nodes[3]),
            None,
            nodes[3],
            multiplicity * (d795_dn3),
        );
        let d804_dn3: f64 = v1953;
        stamper.stamp_current_reactive_node1(
            Some(nodes[3]),
            None,
            nodes[3],
            multiplicity * (d804_dn3),
        );
        let d810_dn7: f64 = v1957;
        stamper.stamp_current_reactive_node1(
            Some(nodes[7]),
            None,
            nodes[7],
            multiplicity * (d810_dn7),
        );
        let d842_dn3: f64 = v2028;
        let d842_dn5: f64 = v2029;
        let d842_dn6: f64 = v2030;
        stamper.stamp_current_reactive_node3(
            Some(nodes[5]),
            Some(nodes[6]),
            nodes[3],
            multiplicity * (d842_dn3),
            nodes[5],
            multiplicity * (d842_dn5),
            nodes[6],
            multiplicity * (d842_dn6),
        );
        let d844_dn3: f64 = v2034;
        let d844_dn5: f64 = v2035;
        let d844_dn6: f64 = v2036;
        stamper.stamp_current_reactive_node3(
            Some(nodes[5]),
            Some(nodes[6]),
            nodes[3],
            multiplicity * (d844_dn3),
            nodes[5],
            multiplicity * (d844_dn5),
            nodes[6],
            multiplicity * (d844_dn6),
        );
        let d846_dn1: f64 = v2042;
        let d846_dn3: f64 = v2043;
        let d846_dn4: f64 = v2044;
        let d846_dn5: f64 = v2045;
        let d846_dn6: f64 = v2046;
        let v846_reactive_nodes: [usize; 5] = [nodes[1], nodes[3], nodes[4], nodes[5], nodes[6]];
        let v846_reactive_node_derivatives: [f64; 5] = [d846_dn1, d846_dn3, d846_dn4, d846_dn5, d846_dn6];
        stamper.stamp_current_reactive_dense(
            Some(nodes[1]),
            Some(nodes[4]),
            &v846_reactive_nodes,
            &v846_reactive_node_derivatives,
            &[],
            &[],
            multiplicity,
        );
        let d848_dn1: f64 = v2052;
        let d848_dn3: f64 = v2053;
        let d848_dn4: f64 = v2054;
        let d848_dn5: f64 = v2055;
        let d848_dn6: f64 = v2056;
        let v848_reactive_nodes: [usize; 5] = [nodes[1], nodes[3], nodes[4], nodes[5], nodes[6]];
        let v848_reactive_node_derivatives: [f64; 5] = [d848_dn1, d848_dn3, d848_dn4, d848_dn5, d848_dn6];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[4]),
            &v848_reactive_nodes,
            &v848_reactive_node_derivatives,
            &[],
            &[],
            multiplicity,
        );
        let d850_dn3: f64 = v2061;
        let d850_dn4: f64 = v2062;
        let d850_dn5: f64 = v2063;
        let d850_dn6: f64 = v2064;
        let v850_reactive_nodes: [usize; 4] = [nodes[3], nodes[4], nodes[5], nodes[6]];
        let v850_reactive_node_derivatives: [f64; 4] = [d850_dn3, d850_dn4, d850_dn5, d850_dn6];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[4]),
            &v850_reactive_nodes,
            &v850_reactive_node_derivatives,
            &[],
            &[],
            multiplicity,
        );
        let d852_dn2: f64 = v2068;
        let d852_dn3: f64 = v2069;
        let d852_dn4: f64 = v2070;
        stamper.stamp_current_reactive_node3(
            Some(nodes[2]),
            Some(nodes[4]),
            nodes[2],
            multiplicity * (d852_dn2),
            nodes[3],
            multiplicity * (d852_dn3),
            nodes[4],
            multiplicity * (d852_dn4),
        );
        let d854_dn3: f64 = v2075;
        let d854_dn4: f64 = v2076;
        let d854_dn5: f64 = v2077;
        let d854_dn6: f64 = v2078;
        let v854_reactive_nodes: [usize; 4] = [nodes[3], nodes[4], nodes[5], nodes[6]];
        let v854_reactive_node_derivatives: [f64; 4] = [d854_dn3, d854_dn4, d854_dn5, d854_dn6];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[6]),
            &v854_reactive_nodes,
            &v854_reactive_node_derivatives,
            &[],
            &[],
            multiplicity,
        );
        let d855_dn3: f64 = v2079;
        let d855_dn4: f64 = v2080;
        let d855_dn5: f64 = v2081;
        let d855_dn6: f64 = v2082;
        let v855_reactive_nodes: [usize; 4] = [nodes[3], nodes[4], nodes[5], nodes[6]];
        let v855_reactive_node_derivatives: [f64; 4] = [d855_dn3, d855_dn4, d855_dn5, d855_dn6];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[4]),
            &v855_reactive_nodes,
            &v855_reactive_node_derivatives,
            &[],
            &[],
            multiplicity,
        );
    }
}
