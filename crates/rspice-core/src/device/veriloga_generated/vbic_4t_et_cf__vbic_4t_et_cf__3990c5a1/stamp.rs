#![allow(dead_code, unused_imports, unused_parens, unused_variables)]

use super::state::Instance;
use crate::device::veriloga_generated::{GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper};

const LIMEXP_MAX: f64 = 5.54062238439351e34;

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
        self.ensure_temperature_static(ctx.temperature(), ctx.thermal_voltage());
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
        let v6: f64 = ctx.node_voltage(nodes[4]);
        let v7: f64 = (self.scalar_v5 + v6);
        let v11: f64 = ((v7 * 1.3806503e-23) / 1.602176462e-19);
        let v12: f64 = (v7 / self.scalar_v2);
        let v13: f64 = (v7 - self.scalar_v2);
        let v17: f64 = (self.scalar_v14 * f64::powf(v12, self.scalar_v15));
        let v21: f64 = (self.scalar_v18 * f64::powf(v12, self.scalar_v19));
        let v25: f64 = (self.scalar_v22 * f64::powf(v12, self.scalar_v23));
        let v29: f64 = (self.scalar_v26 * f64::powf(v12, self.scalar_v27));
        let v33: f64 = (self.scalar_v30 * f64::powf(v12, self.scalar_v31));
        let v37: f64 = (self.scalar_v34 * f64::powf(v12, self.scalar_v35));
        let v41: f64 = (self.scalar_v38 * f64::powf(v12, self.scalar_v39));
        let v45: f64 = (self.scalar_v42 * f64::powf(v12, self.scalar_v43));
        let v48: f64 = f64::powf(v12, self.scalar_v47);
        let v51: f64 = 1.0;
        let v52: f64 = (v51 - v12);
        let v53: f64 = (self.scalar_v50 * v52);
        let v55: f64 = (((v53 / v11)) as f64).exp();
        let v56: f64 = (v48 * v55);
        let v60: f64 = (self.scalar_v46 * f64::powf(v56, self.scalar_v58));
        let v63: f64 = f64::powf(v12, self.scalar_v62);
        let v66: f64 = (v52 * self.scalar_v65);
        let v68: f64 = (((v66 / v11)) as f64).exp();
        let v69: f64 = (v63 * v68);
        let v73: f64 = (self.scalar_v61 * f64::powf(v69, self.scalar_v71));
        let v77: f64 = (v52 * self.scalar_v76);
        let v79: f64 = (((v77 / v11)) as f64).exp();
        let v80: f64 = (v48 * v79);
        let v84: f64 = (self.scalar_v74 * f64::powf(v80, self.scalar_v82));
        let v87: f64 = f64::powf(v12, self.scalar_v86);
        let v90: f64 = (v52 * self.scalar_v89);
        let v92: f64 = (((v90 / v11)) as f64).exp();
        let v93: f64 = (v87 * v92);
        let v97: f64 = (self.scalar_v85 * f64::powf(v93, self.scalar_v95));
        let v100: f64 = f64::powf(v12, self.scalar_v99);
        let v103: f64 = (v52 * self.scalar_v102);
        let v105: f64 = (((v103 / v11)) as f64).exp();
        let v106: f64 = (v100 * v105);
        let v110: f64 = (self.scalar_v98 * f64::powf(v106, self.scalar_v108));
        let v114: f64 = (v52 * self.scalar_v113);
        let v116: f64 = (((v114 / v11)) as f64).exp();
        let v117: f64 = (v87 * v116);
        let v120: f64 = f64::powf(v117, self.scalar_v119);
        let v121: f64 = (self.scalar_v111 * v120);
        let v125: f64 = (v52 * self.scalar_v124);
        let v127: f64 = (((v125 / v11)) as f64).exp();
        let v128: f64 = (v100 * v127);
        let v131: f64 = f64::powf(v128, self.scalar_v130);
        let v132: f64 = (self.scalar_v122 * v131);
        let v134: f64 = (v120 * self.scalar_v133);
        let v136: f64 = (v131 * self.scalar_v135);
        let v140: f64 = (v52 * self.scalar_v139);
        let v142: f64 = (((v140 / v11)) as f64).exp();
        let v143: f64 = (v87 * v142);
        let v147: f64 = (self.scalar_v137 * f64::powf(v143, self.scalar_v145));
        let v151: f64 = (v52 * self.scalar_v150);
        let v153: f64 = (((v151 / v11)) as f64).exp();
        let v154: f64 = (v100 * v153);
        let v158: f64 = (self.scalar_v148 * f64::powf(v154, self.scalar_v156));
        let v161: f64 = (v51 + (v13 * self.scalar_v159));
        let v162: f64 = (self.scalar_v57 * v161);
        let v163: f64 = (self.scalar_v70 * v161);
        let v172: f64 = (v13 * self.scalar_v171);
        let v173: f64 = (self.scalar_v170 + v172);
        let v181: f64 = (self.scalar_v177 * (v51 + (v13 * self.scalar_v178)));
        let v182: f64 = 2.0;
        let v184: f64 = (v182 * (v11 / v12));
        let v185: f64 = 0.5;
        let v188: f64 = (v12 * self.scalar_v187);
        let v190: f64 = (((v188 / v11)) as f64).exp();
        let v191: f64 = -0.5;
        let v193: f64 = (v12 * self.scalar_v192);
        let v195: f64 = (((v193 / v11)) as f64).exp();
        let v196: f64 = (v190 - v195);
        let v197: f64 = ((v196) as f64).ln();
        let v198: f64 = (v184 * v197);
        let v201: f64 = (v11 * 3.0);
        let v202: f64 = ((v12) as f64).ln();
        let v203: f64 = (v201 * v202);
        let v205: f64 = (v12 - v51);
        let v207: f64 = (((v12 * v198) - v203) - (self.scalar_v88 * v205));
        let v208: f64 = (v11 * v182);
        let v209: f64 = 4.0;
        let v210: f64 = (-v207);
        let v212: f64 = (((v210 / v11)) as f64).exp();
        let v215: f64 = (((v51 + (v209 * v212))) as f64).sqrt();
        let v217: f64 = (v185 * (v51 + v215));
        let v218: f64 = ((v217) as f64).ln();
        let v220: f64 = (v207 + (v208 * v218));
        let v223: f64 = (v12 * self.scalar_v222);
        let v225: f64 = (((v223 / v11)) as f64).exp();
        let v227: f64 = (v12 * self.scalar_v226);
        let v229: f64 = (((v227 / v11)) as f64).exp();
        let v230: f64 = (v225 - v229);
        let v231: f64 = ((v230) as f64).ln();
        let v232: f64 = (v184 * v231);
        let v236: f64 = (((v12 * v232) - v203) - (self.scalar_v112 * v205));
        let v237: f64 = (-v236);
        let v239: f64 = (((v237 / v11)) as f64).exp();
        let v242: f64 = (((v51 + (v209 * v239))) as f64).sqrt();
        let v244: f64 = (v185 * (v51 + v242));
        let v245: f64 = ((v244) as f64).ln();
        let v247: f64 = (v236 + (v208 * v245));
        let v250: f64 = (v12 * self.scalar_v249);
        let v252: f64 = (((v250 / v11)) as f64).exp();
        let v254: f64 = (v12 * self.scalar_v253);
        let v256: f64 = (((v254 / v11)) as f64).exp();
        let v257: f64 = (v252 - v256);
        let v258: f64 = ((v257) as f64).ln();
        let v259: f64 = (v184 * v258);
        let v263: f64 = (((v12 * v259) - v203) - (self.scalar_v138 * v205));
        let v264: f64 = (-v263);
        let v266: f64 = (((v264 / v11)) as f64).exp();
        let v269: f64 = (((v51 + (v209 * v266))) as f64).sqrt();
        let v271: f64 = (v185 * (v51 + v269));
        let v272: f64 = ((v271) as f64).ln();
        let v274: f64 = (v263 + (v208 * v272));
        let v276: f64 = (self.scalar_v186 / v220);
        let v279: f64 = (self.scalar_v275 * f64::powf(v276, self.scalar_v277));
        let v281: f64 = (self.scalar_v221 / v247);
        let v283: f64 = f64::powf(v281, self.scalar_v282);
        let v284: f64 = (self.scalar_v280 * v283);
        let v286: f64 = (v283 * self.scalar_v285);
        let v288: f64 = (self.scalar_v248 / v274);
        let v291: f64 = (self.scalar_v287 * f64::powf(v288, self.scalar_v289));
        let v293: f64 = (v48 * self.scalar_v292);
        let v294: f64 = (v55 * v293);
        let v298: f64 = (self.scalar_v295 * f64::powf(v12, self.scalar_v296));
        let v299: f64 = (-(self.scalar_v169 * (v51 + (v13 * v173))));
        let v300: f64 = (v11 * v181);
        let v302: f64 = (((v299 / v300)) as f64).exp();
        let v304: f64 = 0.0;
        let v314: f64 = (if self.scalar_v312 { (v51 / v17) } else { v304 });
        let v325: f64 = (if self.scalar_v323 { (v51 / v298) } else { v304 });
        let v339: f64 = ctx.node_voltage(nodes[8]);
        let v340: f64 = ctx.node_voltage(nodes[9]);
        let v341: f64 = (v339 - v340);
        let v342: f64 = ctx.node_voltage(nodes[7]);
        let v343: f64 = (v342 - v340);
        let v344: f64 = ctx.node_voltage(nodes[6]);
        let v345: f64 = (v339 - v344);
        let v346: f64 = ctx.node_voltage(nodes[5]);
        let v347: f64 = (v339 - v346);
        let v348: f64 = ctx.node_voltage(nodes[10]);
        let v349: f64 = (v342 - v348);
        let v350: f64 = (-v220);
        let v352: f64 = (v350 * self.scalar_v351);
        let v355: f64 = (v341 + v352);
        let v356: f64 = (if self.scalar_v354 { v355 } else { v304 });
        let v357: bool = (v356 > v304);
        let v358: bool = (self.scalar_v354 && v357);
        let v360: f64 = -1.0;
        let v363: f64 = (if v358 { self.scalar_v362 } else { v304 });
        let v366: f64 = (v51 - (self.scalar_v359 * (self.scalar_v359 * v363)));
        let v372: f64 = (v356 * self.scalar_v371);
        let v374: f64 = (self.scalar_v359 + (v372 / v220));
        let v379: bool = (self.scalar_v354 && (!v357));
        let v381: f64 = (v51 - (v341 / v220));
        let v383: f64 = (v51 - f64::powf(v381, self.scalar_v368));
        let v386: f64 = (if v379 { ((v220 * v383) / self.scalar_v368) } else { (if v358 { ((v220 * v366) / self.scalar_v368) } else { v304 }) });
        let v387: f64 = (if v379 { v304 } else { (if v358 { (v363 * (v356 * v374)) } else { v304 }) });
        let v395: f64 = ((((v352 * v352) + self.scalar_v393)) as f64).sqrt();
        let v396: f64 = (if self.scalar_v390 { v395 } else { v304 });
        let v399: f64 = (if self.scalar_v390 { (v191 * (v352 + v396)) } else { v304 });
        let v401: f64 = (v51 - (v399 / v220));
        let v402: f64 = f64::powf(v401, self.scalar_v368);
        let v405: f64 = (if self.scalar_v390 { ((v350 * v402) / self.scalar_v368) } else { v304 });
        let v406: f64 = (if self.scalar_v390 { v355 } else { v304 });
        let v409: f64 = (((self.scalar_v393 + (v406 * v406))) as f64).sqrt();
        let v410: f64 = (if self.scalar_v390 { v409 } else { v304 });
        let v414: f64 = (if self.scalar_v390 { ((v185 * (v406 - v410)) - v352) } else { v304 });
        let v416: f64 = (v51 - (v414 / v220));
        let v417: f64 = f64::powf(v416, self.scalar_v368);
        let v420: f64 = (if self.scalar_v390 { ((v350 * v417) / self.scalar_v368) } else { v386 });
        let v428: f64 = (if self.scalar_v390 { ((v420 + (self.scalar_v422 * (v399 + (v341 - v414)))) - v405) } else { (if self.scalar_v354 { (v386 + v387) } else { v304 }) });
        let v429: f64 = (v343 + v352);
        let v430: f64 = (if self.scalar_v354 { v429 } else { v356 });
        let v431: bool = (v430 > v304);
        let v432: bool = (self.scalar_v354 && v431);
        let v433: f64 = (if v432 { self.scalar_v362 } else { v363 });
        let v436: f64 = (v51 - (self.scalar_v359 * (self.scalar_v359 * v433)));
        let v440: f64 = (self.scalar_v371 * v430);
        let v442: f64 = (self.scalar_v359 + (v440 / v220));
        let v447: bool = (self.scalar_v354 && (!v431));
        let v449: f64 = (v51 - (v343 / v220));
        let v451: f64 = (v51 - f64::powf(v449, self.scalar_v368));
        let v454: f64 = (if v447 { ((v220 * v451) / self.scalar_v368) } else { (if v432 { ((v220 * v436) / self.scalar_v368) } else { v420 }) });
        let v455: f64 = (if v447 { v304 } else { (if v432 { (v433 * (v430 * v442)) } else { v387 }) });
        let v458: f64 = (if self.scalar_v390 { v395 } else { v396 });
        let v461: f64 = (if self.scalar_v390 { (v191 * (v352 + v458)) } else { v399 });
        let v463: f64 = (v51 - (v461 / v220));
        let v464: f64 = f64::powf(v463, self.scalar_v368);
        let v467: f64 = (if self.scalar_v390 { ((v350 * v464) / self.scalar_v368) } else { v405 });
        let v468: f64 = (if self.scalar_v390 { v429 } else { v406 });
        let v471: f64 = (((self.scalar_v393 + (v468 * v468))) as f64).sqrt();
        let v472: f64 = (if self.scalar_v390 { v471 } else { v410 });
        let v476: f64 = (if self.scalar_v390 { ((v185 * (v468 - v472)) - v352) } else { v414 });
        let v478: f64 = (v51 - (v476 / v220));
        let v479: f64 = f64::powf(v478, self.scalar_v368);
        let v482: f64 = (if self.scalar_v390 { ((v350 * v479) / self.scalar_v368) } else { v454 });
        let v488: f64 = (if self.scalar_v390 { ((v482 + (self.scalar_v422 * (v461 + (v343 - v476)))) - v467) } else { (if self.scalar_v354 { (v454 + v455) } else { v304 }) });
        let v489: f64 = (-v247);
        let v490: f64 = (self.scalar_v351 * v489);
        let v493: f64 = (v345 + v490);
        let v494: f64 = (if self.scalar_v492 { v493 } else { v430 });
        let v495: bool = (v494 > v304);
        let v496: bool = (self.scalar_v492 && v495);
        let v499: f64 = (if v496 { self.scalar_v498 } else { v433 });
        let v502: f64 = (v51 - (self.scalar_v359 * (self.scalar_v359 * v499)));
        let v508: f64 = (v494 * self.scalar_v507);
        let v510: f64 = (self.scalar_v359 + (v508 / v247));
        let v518: bool = (self.scalar_v515 && (v345 < self.scalar_v516));
        let v520: bool = (self.scalar_v492 && (!v495));
        let v521: bool = (v518 && v520);
        let v523: f64 = (v51 + (self.scalar_v514 / v247));
        let v524: f64 = f64::powf(v523, self.scalar_v504);
        let v526: f64 = (self.scalar_v504 * (v345 + self.scalar_v514));
        let v527: f64 = (v247 + self.scalar_v514);
        let v529: f64 = (v51 - (v526 / v527));
        let v531: f64 = (v51 - (v524 * v529));
        let v536: bool = (v520 && (!v518));
        let v538: f64 = (v51 - (v345 / v247));
        let v540: f64 = (v51 - f64::powf(v538, self.scalar_v504));
        let v543: f64 = (if v536 { ((v247 * v540) / self.scalar_v504) } else { (if v521 { ((v247 * v531) / self.scalar_v504) } else { (if v496 { ((v247 * v502) / self.scalar_v504) } else { v482 }) }) });
        let v544: f64 = (if v520 { v304 } else { (if v496 { (v499 * (v494 * v510)) } else { v455 }) });
        let v552: f64 = (v490 + self.scalar_v514);
        let v553: f64 = (self.scalar_v514 - v490);
        let v554: f64 = (v552 / v553);
        let v555: f64 = (if self.scalar_v551 { v554 } else { v304 });
        let v556: f64 = (v182 * v555);
        let v557: f64 = (v555 - v51);
        let v562: f64 = ((((v557 * v557) + self.scalar_v560)) as f64).sqrt();
        let v563: f64 = (v51 + v555);
        let v568: f64 = ((((v563 * v563) + self.scalar_v566)) as f64).sqrt();
        let v569: f64 = (v562 + v568);
        let v571: f64 = (if self.scalar_v551 { (v556 / v569) } else { v304 });
        let v576: f64 = (if self.scalar_v551 { (v185 * (((v553 * v571) - self.scalar_v514) - v490)) } else { v461 });
        let v578: f64 = (v51 - (v576 / v247));
        let v580: f64 = (v51 - f64::powf(v578, self.scalar_v504));
        let v583: f64 = (if self.scalar_v551 { ((v247 * v580) / self.scalar_v504) } else { v304 });
        let v586: f64 = (v490 + (self.scalar_v514 + (v182 * v345)));
        let v588: f64 = (if self.scalar_v551 { (v586 / v553) } else { v304 });
        let v589: f64 = (v182 * v588);
        let v590: f64 = (v588 - v51);
        let v593: f64 = (((self.scalar_v560 + (v590 * v590))) as f64).sqrt();
        let v594: f64 = (v51 + v588);
        let v597: f64 = (((self.scalar_v566 + (v594 * v594))) as f64).sqrt();
        let v598: f64 = (v593 + v597);
        let v600: f64 = (if self.scalar_v551 { (v589 / v598) } else { v304 });
        let v605: f64 = (if self.scalar_v551 { (v185 * (((v553 * v600) - self.scalar_v514) - v490)) } else { v476 });
        let v607: f64 = (v51 - (v605 / v247));
        let v609: f64 = (v51 - f64::powf(v607, self.scalar_v504));
        let v612: f64 = (if self.scalar_v551 { ((v247 * v609) / self.scalar_v504) } else { v543 });
        let v615: f64 = (if self.scalar_v551 { (v185 * (v51 + v600)) } else { v304 });
        let v617: f64 = f64::powf(v523, self.scalar_v616);
        let v618: f64 = (if self.scalar_v551 { v617 } else { v304 });
        let v620: f64 = (v51 + (v490 / v247));
        let v621: f64 = f64::powf(v620, self.scalar_v616);
        let v622: f64 = (if self.scalar_v551 { v621 } else { v304 });
        let v623: f64 = (v51 - v615);
        let v627: f64 = (if self.scalar_v551 { ((v618 * v623) + (v615 * v622)) } else { v304 });
        let v629: f64 = (v576 + (v345 - v605));
        let v631: f64 = (if self.scalar_v551 { (v627 * v629) } else { v304 });
        let v639: f64 = (((self.scalar_v560 + (v490 * v490))) as f64).sqrt();
        let v640: f64 = (if self.scalar_v636 { v639 } else { v458 });
        let v643: f64 = (if self.scalar_v636 { (v191 * (v490 + v640)) } else { v576 });
        let v645: f64 = (v51 - (v643 / v247));
        let v646: f64 = f64::powf(v645, self.scalar_v504);
        let v649: f64 = (if self.scalar_v636 { ((v489 * v646) / self.scalar_v504) } else { v467 });
        let v650: f64 = (if self.scalar_v636 { v493 } else { v468 });
        let v653: f64 = (((self.scalar_v560 + (v650 * v650))) as f64).sqrt();
        let v654: f64 = (if self.scalar_v636 { v653 } else { v472 });
        let v658: f64 = (if self.scalar_v636 { ((v185 * (v650 - v654)) - v490) } else { v605 });
        let v660: f64 = (v51 - (v658 / v247));
        let v661: f64 = f64::powf(v660, self.scalar_v504);
        let v664: f64 = (if self.scalar_v636 { ((v489 * v661) / self.scalar_v504) } else { v612 });
        let v671: f64 = (if self.scalar_v636 { ((v664 + (self.scalar_v665 * (v643 + (v345 - v658)))) - v649) } else { (if self.scalar_v551 { ((v612 + v631) - v583) } else { (if self.scalar_v492 { (v543 + v544) } else { v304 }) }) });
        let v672: f64 = (v349 + v490);
        let v673: f64 = (if self.scalar_v492 { v672 } else { v494 });
        let v674: bool = (v673 > v304);
        let v675: bool = (self.scalar_v492 && v674);
        let v676: f64 = (if v675 { self.scalar_v498 } else { v499 });
        let v679: f64 = (v51 - (self.scalar_v359 * (self.scalar_v359 * v676)));
        let v683: f64 = (self.scalar_v507 * v673);
        let v685: f64 = (self.scalar_v359 + (v683 / v247));
        let v690: bool = (self.scalar_v515 && (v349 < self.scalar_v516));
        let v692: bool = (self.scalar_v492 && (!v674));
        let v693: bool = (v690 && v692);
        let v695: f64 = (self.scalar_v504 * (v349 + self.scalar_v514));
        let v697: f64 = (v51 - (v695 / v527));
        let v699: f64 = (v51 - (v524 * v697));
        let v704: bool = (v692 && (!v690));
        let v706: f64 = (v51 - (v349 / v247));
        let v708: f64 = (v51 - f64::powf(v706, self.scalar_v504));
        let v711: f64 = (if v704 { ((v247 * v708) / self.scalar_v504) } else { (if v693 { ((v247 * v699) / self.scalar_v504) } else { (if v675 { ((v247 * v679) / self.scalar_v504) } else { v664 }) }) });
        let v712: f64 = (if v692 { v304 } else { (if v675 { (v676 * (v673 * v685)) } else { v544 }) });
        let v715: f64 = (if self.scalar_v551 { v554 } else { v555 });
        let v716: f64 = (v182 * v715);
        let v717: f64 = (v715 - v51);
        let v720: f64 = (((self.scalar_v560 + (v717 * v717))) as f64).sqrt();
        let v721: f64 = (v51 + v715);
        let v724: f64 = (((self.scalar_v566 + (v721 * v721))) as f64).sqrt();
        let v725: f64 = (v720 + v724);
        let v727: f64 = (if self.scalar_v551 { (v716 / v725) } else { v571 });
        let v732: f64 = (if self.scalar_v551 { (v185 * (((v553 * v727) - self.scalar_v514) - v490)) } else { v643 });
        let v734: f64 = (v51 - (v732 / v247));
        let v736: f64 = (v51 - f64::powf(v734, self.scalar_v504));
        let v742: f64 = (v490 + (self.scalar_v514 + (v182 * v349)));
        let v744: f64 = (if self.scalar_v551 { (v742 / v553) } else { v588 });
        let v745: f64 = (v182 * v744);
        let v746: f64 = (v744 - v51);
        let v749: f64 = (((self.scalar_v560 + (v746 * v746))) as f64).sqrt();
        let v750: f64 = (v51 + v744);
        let v753: f64 = (((self.scalar_v566 + (v750 * v750))) as f64).sqrt();
        let v754: f64 = (v749 + v753);
        let v756: f64 = (if self.scalar_v551 { (v745 / v754) } else { v600 });
        let v761: f64 = (if self.scalar_v551 { (v185 * (((v553 * v756) - self.scalar_v514) - v490)) } else { v658 });
        let v763: f64 = (v51 - (v761 / v247));
        let v765: f64 = (v51 - f64::powf(v763, self.scalar_v504));
        let v768: f64 = (if self.scalar_v551 { ((v247 * v765) / self.scalar_v504) } else { v711 });
        let v771: f64 = (if self.scalar_v551 { (v185 * (v51 + v756)) } else { v615 });
        let v772: f64 = (if self.scalar_v551 { v617 } else { v618 });
        let v773: f64 = (if self.scalar_v551 { v621 } else { v622 });
        let v774: f64 = (v51 - v771);
        let v778: f64 = (if self.scalar_v551 { ((v772 * v774) + (v771 * v773)) } else { v627 });
        let v780: f64 = (v732 + (v349 - v761));
        let v785: f64 = (if self.scalar_v551 { ((v768 + (if self.scalar_v551 { (v778 * v780) } else { v631 })) - (if self.scalar_v551 { ((v247 * v736) / self.scalar_v504) } else { v583 })) } else { (if self.scalar_v492 { (v711 + v712) } else { v304 }) });
        let v786: f64 = (if self.scalar_v636 { v639 } else { v640 });
        let v789: f64 = (if self.scalar_v636 { (v191 * (v490 + v786)) } else { v732 });
        let v791: f64 = (v51 - (v789 / v247));
        let v792: f64 = f64::powf(v791, self.scalar_v504);
        let v795: f64 = (if self.scalar_v636 { ((v489 * v792) / self.scalar_v504) } else { v649 });
        let v796: f64 = (if self.scalar_v636 { v672 } else { v650 });
        let v799: f64 = (((self.scalar_v560 + (v796 * v796))) as f64).sqrt();
        let v800: f64 = (if self.scalar_v636 { v799 } else { v654 });
        let v804: f64 = (if self.scalar_v636 { ((v185 * (v796 - v800)) - v490) } else { v761 });
        let v806: f64 = (v51 - (v804 / v247));
        let v807: f64 = f64::powf(v806, self.scalar_v504);
        let v810: f64 = (if self.scalar_v636 { ((v489 * v807) / self.scalar_v504) } else { v768 });
        let v816: f64 = (if self.scalar_v636 { ((v810 + (self.scalar_v665 * (v789 + (v349 - v804)))) - v795) } else { v785 });
        let v818: f64 = (-v274);
        let v820: f64 = (if self.scalar_v817 { (self.scalar_v351 * v818) } else { v490 });
        let v824: f64 = ctx.node_voltage(nodes[11]);
        let v825: f64 = (v824 - v348);
        let v826: f64 = (v820 + v825);
        let v827: f64 = (if self.scalar_v823 { v826 } else { v673 });
        let v828: bool = (v827 > v304);
        let v829: bool = (self.scalar_v823 && v828);
        let v832: f64 = (if v829 { self.scalar_v831 } else { v676 });
        let v835: f64 = (v51 - (self.scalar_v359 * (self.scalar_v359 * v832)));
        let v841: f64 = (v827 * self.scalar_v840);
        let v843: f64 = (self.scalar_v359 + (v841 / v274));
        let v848: bool = (self.scalar_v823 && (!v828));
        let v850: f64 = (v51 - (v825 / v274));
        let v852: f64 = (v51 - f64::powf(v850, self.scalar_v837));
        let v855: f64 = (if v848 { ((v274 * v852) / self.scalar_v837) } else { (if v829 { ((v274 * v835) / self.scalar_v837) } else { v810 }) });
        let v865: f64 = ((((v820 * v820) + self.scalar_v863)) as f64).sqrt();
        let v869: f64 = (if self.scalar_v860 { (v191 * (v820 + (if self.scalar_v860 { v865 } else { v786 }))) } else { v789 });
        let v871: f64 = (v51 - (v869 / v274));
        let v872: f64 = f64::powf(v871, self.scalar_v837);
        let v876: f64 = (if self.scalar_v860 { v826 } else { v796 });
        let v879: f64 = (((self.scalar_v863 + (v876 * v876))) as f64).sqrt();
        let v884: f64 = (if self.scalar_v860 { ((v185 * (v876 - (if self.scalar_v860 { v879 } else { v800 }))) - v820) } else { v804 });
        let v886: f64 = (v51 - (v884 / v274));
        let v887: f64 = f64::powf(v886, self.scalar_v837);
        let v897: f64 = (((if self.scalar_v860 { ((v818 * v887) / self.scalar_v837) } else { v855 }) + (self.scalar_v892 * (v869 + (v825 - v884)))) - (if self.scalar_v860 { ((v818 * v872) / self.scalar_v837) } else { v795 }));
        let v900: f64 = (if self.scalar_v899 { v304 } else { (if self.scalar_v860 { v897 } else { (if self.scalar_v823 { (v855 + (if v848 { v304 } else { (if v829 { (v832 * (v827 * v843)) } else { v712 }) })) } else { v304 }) }) });
        let v901: f64 = (v11 * v162);
        let v902: f64 = (v341 / v901);
        let v904: f64 = ({ let limexp_arg = v902; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } } - v51);
        let v905: f64 = (v60 * v904);
        let v906: f64 = (v11 * v163);
        let v907: f64 = (v345 / v906);
        let v908: f64 = { let limexp_arg = v907; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let v909: f64 = (v60 * v73);
        let v910: f64 = (v908 - v51);
        let v911: f64 = (v909 * v910);
        let v915: f64 = ((v51 + (self.scalar_v311 * v428)) + (self.scalar_v307 * v671));
        let v916: f64 = 0.0001;
        let v917: f64 = (v915 - v916);
        let v921: f64 = ((((v917 * v917) + 1e-8)) as f64).sqrt();
        let v925: f64 = (v916 + (v185 * ((v915 + v921) - v916)));
        let v934: f64 = (v209 * ((v314 * v905) + (self.scalar_v318 * v911)));
        let v935: f64 = (f64::powf(v925, self.scalar_v932) + v934);
        let v941: f64 = (v185 * v925);
        let v942: f64 = (v51 + v934);
        let v944: f64 = (v51 + f64::powf(v942, self.scalar_v931));
        let v946: f64 = (if self.scalar_v940 { (v941 * v944) } else { (if self.scalar_v930 { (v185 * (v925 + f64::powf(v935, self.scalar_v931))) } else { v304 }) });
        let v947: f64 = (v911 / v946);
        let v948: f64 = (v905 / v946);
        let v950: f64 = (v11 * self.scalar_v81);
        let v952: f64 = (if self.scalar_v949 { (v349 / v950) } else { v907 });
        let v954: f64 = (if self.scalar_v949 { { let limexp_arg = v952; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } } } else { v908 });
        let v956: f64 = (if self.scalar_v949 { (v345 / v950) } else { v304 });
        let v958: f64 = (if self.scalar_v949 { { let limexp_arg = v956; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } } } else { v304 });
        let v964: f64 = (((v954 * self.scalar_v959) + (v958 * self.scalar_v961)) - v51);
        let v966: f64 = (if self.scalar_v949 { (v84 * v964) } else { v304 });
        let v971: f64 = (((v51 + (v209 * (if self.scalar_v949 { (self.scalar_v322 * v966) } else { v304 })))) as f64).sqrt();
        let v974: f64 = (if self.scalar_v949 { (v185 * (v51 + v971)) } else { v304 });
        let v976: f64 = (if self.scalar_v949 { (v825 / v950) } else { v952 });
        let v978: f64 = (if self.scalar_v949 { { let limexp_arg = v976; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } } } else { v954 });
        let v979: f64 = (v978 - v51);
        let v982: f64 = (v966 - (if self.scalar_v949 { (v84 * v979) } else { v304 }));
        let v987: f64 = (if self.scalar_v985 { v51 } else { v974 });
        let v988: f64 = (if self.scalar_v985 { v304 } else { (if self.scalar_v949 { (v982 / v974) } else { v304 }) });
        let v991: f64 = (v11 * self.scalar_v94);
        let v992: f64 = (v341 / v991);
        let v993: f64 = (if self.scalar_v990 { v992 } else { v976 });
        let v995: f64 = (if self.scalar_v990 { { let limexp_arg = v993; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } } } else { v978 });
        let v996: f64 = (v11 * self.scalar_v107);
        let v997: f64 = (v341 / v996);
        let v998: f64 = (if self.scalar_v990 { v997 } else { v304 });
        let v1000: f64 = (if self.scalar_v990 { { let limexp_arg = v998; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } } } else { v304 });
        let v1003: f64 = (v299 - v341);
        let v1004: f64 = (v1003 / v300);
        let v1005: f64 = (if self.scalar_v1002 { v1004 } else { v956 });
        let v1007: f64 = (if self.scalar_v1002 { { let limexp_arg = v1005; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } } } else { v958 });
        let v1008: f64 = (v995 - v51);
        let v1010: f64 = (v1000 - v51);
        let v1012: f64 = ((v97 * v1008) + (v110 * v1010));
        let v1025: f64 = (v343 / v991);
        let v1026: f64 = (if self.scalar_v1023 { v1025 } else { v993 });
        let v1028: f64 = (if self.scalar_v1023 { { let limexp_arg = v1026; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } } } else { v995 });
        let v1029: f64 = (v343 / v996);
        let v1030: f64 = (if self.scalar_v1023 { v1029 } else { v998 });
        let v1032: f64 = (if self.scalar_v1023 { { let limexp_arg = v1030; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } } } else { v1000 });
        let v1034: f64 = (v299 - v343);
        let v1035: f64 = (v1034 / v300);
        let v1036: f64 = (if self.scalar_v1033 { v1035 } else { v1005 });
        let v1038: f64 = (if self.scalar_v1033 { { let limexp_arg = v1036; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } } } else { v1007 });
        let v1039: f64 = (v1028 - v51);
        let v1041: f64 = (v1032 - v51);
        let v1043: f64 = ((v97 * v1039) + (v110 * v1041));
        let v1052: f64 = (if self.scalar_v1051 { v992 } else { v1026 });
        let v1054: f64 = (if self.scalar_v1051 { { let limexp_arg = v1052; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } } } else { v1028 });
        let v1055: f64 = (if self.scalar_v1051 { v997 } else { v1030 });
        let v1057: f64 = (if self.scalar_v1051 { { let limexp_arg = v1055; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } } } else { v1032 });
        let v1059: f64 = (if self.scalar_v1058 { v1004 } else { v1036 });
        let v1061: f64 = (if self.scalar_v1058 { { let limexp_arg = v1059; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } } } else { v1038 });
        let v1062: f64 = (v1054 - v51);
        let v1064: f64 = (v1057 - v51);
        let v1066: f64 = ((v97 * v1062) + (v110 * v1064));
        let v1071: f64 = (if self.scalar_v1058 { (self.scalar_v989 * (v1066 - (self.scalar_v1013 * (v1061 - v302)))) } else { (if self.scalar_v1023 { v304 } else { (if self.scalar_v1019 { v1012 } else { (if self.scalar_v1002 { (v1012 - (self.scalar_v1013 * (v1007 - v302))) } else { v304 }) }) }) });
        let v1074: f64 = (if self.scalar_v1072 { (self.scalar_v989 * v1066) } else { v1071 });
        let v1075: f64 = (if self.scalar_v1051 { v1025 } else { v1052 });
        let v1078: f64 = (if self.scalar_v1051 { v1029 } else { v1055 });
        let v1081: f64 = (if self.scalar_v1058 { v1035 } else { v1059 });
        let v1085: f64 = ((if self.scalar_v1051 { { let limexp_arg = v1075; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } } } else { v1054 }) - v51);
        let v1087: f64 = ((if self.scalar_v1051 { { let limexp_arg = v1078; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } } } else { v1057 }) - v51);
        let v1089: f64 = ((v97 * v1085) + (v110 * v1087));
        let v1094: f64 = (if self.scalar_v1058 { (self.scalar_v1084 * (v1089 - (self.scalar_v1013 * ((if self.scalar_v1058 { { let limexp_arg = v1081; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } } } else { v1061 }) - v302)))) } else { (if self.scalar_v1048 { v1043 } else { (if self.scalar_v1033 { (v1043 - (self.scalar_v1013 * (v1038 - v302))) } else { v304 }) }) });
        let v1096: f64 = (if self.scalar_v1072 { (self.scalar_v1084 * v1089) } else { v1094 });
        let v1097: f64 = (v11 * self.scalar_v118);
        let v1098: f64 = (v345 / v1097);
        let v1099: f64 = { let limexp_arg = v1098; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let v1100: f64 = (v11 * self.scalar_v129);
        let v1101: f64 = (v345 / v1100);
        let v1102: f64 = { let limexp_arg = v1101; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let v1103: f64 = (v1099 - v51);
        let v1105: f64 = (v1102 - v51);
        let v1107: f64 = ((v121 * v1103) + (v132 * v1105));
        let v1112: f64 = (if self.scalar_v1110 { (v349 / v1097) } else { v1098 });
        let v1116: f64 = (if self.scalar_v1110 { (v349 / v1100) } else { v1101 });
        let v1118: f64 = (if self.scalar_v1110 { { let limexp_arg = v1116; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } } } else { v1102 });
        let v1119: f64 = ((if self.scalar_v1110 { { let limexp_arg = v1112; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } } } else { v1099 }) - v51);
        let v1121: f64 = (v1118 - v51);
        let v1126: f64 = (if self.scalar_v1125 { v304 } else { (if self.scalar_v1110 { ((v134 * v1119) + (v136 * v1121)) } else { v304 }) });
        let v1129: f64 = (v247 - v345);
        let v1131: f64 = 0.01;
        let v1133: f64 = ((((v1129 * v1129) + v1131)) as f64).sqrt();
        let v1136: f64 = (if self.scalar_v1128 { (v185 * (v1129 + v1133)) } else { v884 });
        let v1137: f64 = (self.scalar_v1127 * v1136);
        let v1138: f64 = (-(self.scalar_v164 * (v51 + (v13 * self.scalar_v165))));
        let v1140: f64 = f64::powf(v1136, self.scalar_v1139);
        let v1141: f64 = (v1138 * v1140);
        let v1142: f64 = { let limexp_arg = v1141; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let v1144: f64 = (if self.scalar_v1128 { (v1137 * v1142) } else { v304 });
        let v1145: f64 = (v948 - v947);
        let v1146: f64 = (v1145 - v1107);
        let v1151: f64 = (v1107 - (if self.scalar_v1149 { v304 } else { (if self.scalar_v1128 { (v1144 * v1146) } else { v304 }) }));
        let v1153: f64 = ctx.node_voltage(nodes[0]);
        let v1154: f64 = (v1153 - v346);
        let v1158: f64 = (if self.scalar_v1157 { v304 } else { (if self.scalar_v1152 { (v1154 / v21) } else { v304 }) });
        let v1159: f64 = (v345 / v11);
        let v1160: f64 = { let limexp_arg = v1159; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let v1161: f64 = (v347 / v11);
        let v1162: f64 = { let limexp_arg = v1161; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let v1165: f64 = (((v51 + (v294 * v1160))) as f64).sqrt();
        let v1168: f64 = (((v51 + (v294 * v1162))) as f64).sqrt();
        let v1170: f64 = (v51 + v1165);
        let v1171: f64 = (v51 + v1168);
        let v1173: f64 = (if self.scalar_v1169 { (v1170 / v1171) } else { v304 });
        let v1174: f64 = (v346 - v344);
        let v1177: f64 = ((v1165 - v1168) - ((v1173) as f64).ln());
        let v1179: f64 = (v1174 + (v11 * v1177));
        let v1181: f64 = (if self.scalar_v1169 { (v1179 / v25) } else { v304 });
        let v1182: f64 = (v25 * v325);
        let v1183: f64 = (v1181 * v1182);
        let v1185: f64 = (self.scalar_v329 * (v185 * v325));
        let v1188: f64 = (((v1131 + (v1174 * v1174))) as f64).sqrt();
        let v1190: f64 = (v51 + (v1185 * v1188));
        let v1192: f64 = (if self.scalar_v1169 { (v1183 / v1190) } else { v304 });
        let v1195: f64 = (((v51 + (v1192 * v1192))) as f64).sqrt();
        let v1199: f64 = (if self.scalar_v1198 { v304 } else { (if self.scalar_v1169 { (v1181 / v1195) } else { v304 }) });
        let v1201: f64 = ctx.node_voltage(nodes[1]);
        let v1202: f64 = (v1201 - v342);
        let v1206: f64 = (if self.scalar_v1205 { v304 } else { (if self.scalar_v1200 { (v1202 / v29) } else { v304 }) });
        let v1208: f64 = (v342 - v339);
        let v1209: f64 = (v946 * v1208);
        let v1213: f64 = (if self.scalar_v1212 { v304 } else { (if self.scalar_v1207 { (v1209 / v33) } else { v304 }) });
        let v1215: f64 = ctx.node_voltage(nodes[2]);
        let v1216: f64 = (v1215 - v340);
        let v1220: f64 = (if self.scalar_v1219 { v304 } else { (if self.scalar_v1214 { (v1216 / v37) } else { v304 }) });
        let v1222: f64 = (v348 - v346);
        let v1223: f64 = (v987 * v1222);
        let v1227: f64 = (if self.scalar_v1226 { v304 } else { (if self.scalar_v1221 { (v1223 / v45) } else { v304 }) });
        let v1231: f64 = (v11 * self.scalar_v144);
        let v1233: f64 = (if self.scalar_v1230 { (v825 / v1231) } else { v1159 });
        let v1236: f64 = (v11 * self.scalar_v155);
        let v1238: f64 = (if self.scalar_v1230 { (v825 / v1236) } else { v1116 });
        let v1241: f64 = ((if self.scalar_v1230 { { let limexp_arg = v1233; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } } } else { v1160 }) - v51);
        let v1243: f64 = ((if self.scalar_v1230 { { let limexp_arg = v1238; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } } } else { v1118 }) - v51);
        let v1248: f64 = (if self.scalar_v1247 { v304 } else { (if self.scalar_v1230 { ((v147 * v1241) + (v158 * v1243)) } else { v304 }) });
        let v1251: f64 = (ctx.node_voltage(nodes[3]) - v824);
        let v1255: f64 = (if self.scalar_v1254 { v304 } else { (if self.scalar_v1249 { (v1251 / v41) } else { v304 }) });
        let v1257: f64 = (if (v905 > v304) { v51 } else { v304 });
        let v1259: f64 = (self.scalar_v337 * (v905 * v1257));
        let v1260: f64 = (v51 + v1259);
        let v1261: f64 = (v1259 / v1260);
        let v1266: f64 = (self.scalar_v1262 * (v51 + (v925 * self.scalar_v1263)));
        let v1270: f64 = ((self.scalar_v333 * v345) / 1.44);
        let v1272: f64 = (self.scalar_v1267 * { let limexp_arg = v1270; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } });
        let v1274: f64 = (self.scalar_v338 + (v1261 * v1261));
        let v1277: f64 = (v51 + (v1257 * (v1272 * v1274)));
        let v1278: f64 = (v1266 * v1277);
        let v1281: f64 = (v905 * v1278);
        let v1283: f64 = ((self.scalar_v989 * (v279 * v428)) + (v1281 / v946));
        let v1285: f64 = (self.scalar_v1084 * (v279 * v488));
        let v1292: f64 = (((v284 * v671) + (v911 * self.scalar_v1287)) + (v1165 * self.scalar_v1290));
        let v1293: f64 = (v1168 * self.scalar_v1290);
        let v1296: f64 = ((v286 * v816) + ((if self.scalar_v985 { v304 } else { v966 }) * self.scalar_v1287));
        let v1300: f64 = ((v291 * v900) + (v825 * self.scalar_v1298));
        let v1303: f64 = ((v1201 - v1215) * self.scalar_v1302);
        let v1306: f64 = ((v1201 - v1153) * self.scalar_v1305);
        let v1310: f64 = (v344 - v340);
        let v1320: f64 = (((((((v341 * v1074) + (v345 * v1151)) + (v1145 * v1310)) + (v343 * v1096)) + (v349 * v1126)) + (v1251 * v1255)) + (v825 * v1248));
        let v1321: f64 = (v342 - v824);
        let v1333: f64 = ((((((v1320 + (v988 * v1321)) + (v1154 * v1158)) + (v1174 * v1199)) + (v1202 * v1206)) + (v1208 * v1213)) + (v1216 * v1220));
        let v1336: f64 = (-(v1333 + (v1222 * v1227)));
        let v1342: f64 = (if self.scalar_v1341 { v304 } else { (if self.scalar_v1338 { (v6 / self.scalar_v1337) } else { v304 }) });
        let v1344: f64 = (v6 * self.scalar_v1343);
        let v1345: f64 = 8.617342301212761e-5;
        let v1361: f64 = (self.scalar_v22 * (self.scalar_v1346 * (self.scalar_v23 * f64::powf(v12, self.scalar_v1357))));
        let v1390: f64 = (self.scalar_v1346 * (self.scalar_v47 * f64::powf(v12, self.scalar_v1387)));
        let v1396: f64 = (v11 * v11);
        let v1398: f64 = (v55 * (((v11 * self.scalar_v1392) - (v53 * v1345)) / v1396));
        let v1406: f64 = (self.scalar_v46 * (((v55 * v1390) + (v48 * v1398)) * (self.scalar_v58 * f64::powf(v56, self.scalar_v1402))));
        let v1423: f64 = (((v68 * (self.scalar_v1346 * (self.scalar_v62 * f64::powf(v12, self.scalar_v1407)))) + (v63 * (v68 * (((v11 * self.scalar_v1411) - (v66 * v1345)) / v1396)))) * (self.scalar_v71 * f64::powf(v69, self.scalar_v1420)));
        let v1438: f64 = (self.scalar_v74 * (((v79 * v1390) + (v48 * (v79 * (((v11 * self.scalar_v1425) - (v77 * v1345)) / v1396)))) * (self.scalar_v82 * f64::powf(v80, self.scalar_v1434))));
        let v1442: f64 = (self.scalar_v1346 * (self.scalar_v86 * f64::powf(v12, self.scalar_v1439)));
        let v1456: f64 = (self.scalar_v85 * (((v92 * v1442) + (v87 * (v92 * (((v11 * self.scalar_v1443) - (v90 * v1345)) / v1396)))) * (self.scalar_v95 * f64::powf(v93, self.scalar_v1452))));
        let v1460: f64 = (self.scalar_v1346 * (self.scalar_v99 * f64::powf(v12, self.scalar_v1457)));
        let v1474: f64 = (self.scalar_v98 * (((v105 * v1460) + (v100 * (v105 * (((v11 * self.scalar_v1461) - (v103 * v1345)) / v1396)))) * (self.scalar_v108 * f64::powf(v106, self.scalar_v1470))));
        let v1487: f64 = (((v116 * v1442) + (v87 * (v116 * (((v11 * self.scalar_v1475) - (v114 * v1345)) / v1396)))) * (self.scalar_v119 * f64::powf(v117, self.scalar_v1484)));
        let v1501: f64 = (((v127 * v1460) + (v100 * (v127 * (((v11 * self.scalar_v1489) - (v125 * v1345)) / v1396)))) * (self.scalar_v130 * f64::powf(v128, self.scalar_v1498)));
        let v1518: f64 = (self.scalar_v137 * (((v142 * v1442) + (v87 * (v142 * (((v11 * self.scalar_v1505) - (v140 * v1345)) / v1396)))) * (self.scalar_v145 * f64::powf(v143, self.scalar_v1514))));
        let v1532: f64 = (self.scalar_v148 * (((v153 * v1460) + (v100 * (v153 * (((v11 * self.scalar_v1519) - (v151 * v1345)) / v1396)))) * (self.scalar_v156 * f64::powf(v154, self.scalar_v1528))));
        let v1544: f64 = (v182 * (((v12 * v1345) - (v11 * self.scalar_v1346)) / (v12 * v12)));
        let v1558: f64 = (((v190 * (((v11 * self.scalar_v1545) - (v188 * v1345)) / v1396)) - (v195 * (((v11 * self.scalar_v1551) - (v193 * v1345)) / v1396))) / v196);
        let v1569: f64 = ((v202 * 0.00025852026903638284) + (v201 * (self.scalar_v1346 / v12)));
        let v1572: f64 = ((((v198 * self.scalar_v1346) + (v12 * ((v197 * v1544) + (v184 * v1558)))) - v1569) - self.scalar_v1571);
        let v1573: f64 = 0.00017234684602425522;
        let v1587: f64 = ((v218 * v1573) + (v208 * ((v185 * ((v209 * (v212 * (((v11 * (-v1572)) - (v210 * v1345)) / v1396))) / (v182 * v215))) / v217)));
        let v1588: f64 = (v1572 + v1587);
        let v1602: f64 = (((v225 * (((v11 * self.scalar_v1589) - (v223 * v1345)) / v1396)) - (v229 * (((v11 * self.scalar_v1595) - (v227 * v1345)) / v1396))) / v230);
        let v1611: f64 = ((((v232 * self.scalar_v1346) + (v12 * ((v231 * v1544) + (v184 * v1602)))) - v1569) - self.scalar_v1610);
        let v1625: f64 = ((v245 * v1573) + (v208 * ((v185 * ((v209 * (v239 * (((v11 * (-v1611)) - (v237 * v1345)) / v1396))) / (v182 * v242))) / v244)));
        let v1626: f64 = (v1611 + v1625);
        let v1640: f64 = (((v252 * (((v11 * self.scalar_v1627) - (v250 * v1345)) / v1396)) - (v256 * (((v11 * self.scalar_v1633) - (v254 * v1345)) / v1396))) / v257);
        let v1649: f64 = ((((v259 * self.scalar_v1346) + (v12 * ((v258 * v1544) + (v184 * v1640)))) - v1569) - self.scalar_v1648);
        let v1663: f64 = ((v272 * v1573) + (v208 * ((v185 * ((v209 * (v266 * (((v11 * (-v1649)) - (v264 * v1345)) / v1396))) / (v182 * v269))) / v271)));
        let v1664: f64 = (v1649 + v1663);
        let v1667: f64 = (v220 * v220);
        let v1673: f64 = (self.scalar_v275 * (((-(self.scalar_v186 * v1588)) / v1667) * (self.scalar_v277 * f64::powf(v276, self.scalar_v1669))));
        let v1676: f64 = (v247 * v247);
        let v1680: f64 = (((-(self.scalar_v221 * v1626)) / v1676) * (self.scalar_v282 * f64::powf(v281, self.scalar_v1139)));
        let v1685: f64 = (v274 * v274);
        let v1695: f64 = ((v293 * v1398) + (v55 * (self.scalar_v292 * v1390)));
        let v1704: f64 = ((v181 * v1345) + (v11 * self.scalar_v1538));
        let v1705: f64 = (v300 * (-(self.scalar_v169 * (v172 + v173))));
        let v1708: f64 = (v300 * v300);
        let v1710: f64 = (v302 * ((v1705 - (v299 * v1704)) / v1708));
        let v1718: f64 = (if self.scalar_v323 { ((-(self.scalar_v295 * (self.scalar_v1346 * (self.scalar_v296 * f64::powf(v12, self.scalar_v1696))))) / (v298 * v298)) } else { v304 });
        let v1719: f64 = (-v1588);
        let v1720: f64 = (self.scalar_v351 * v1719);
        let v1721: f64 = (if self.scalar_v354 { v1720 } else { v304 });
        let v1734: f64 = (self.scalar_v1728 / v220);
        let v1757: f64 = (-(v51 / v220));
        let v1758: f64 = (-(v360 / v220));
        let v1761: f64 = (self.scalar_v368 * f64::powf(v381, self.scalar_v1759));
        let v1776: f64 = (if v379 { (((v383 * v1588) + (v220 * (-((-((-(v341 * v1588)) / v1667)) * v1761)))) / self.scalar_v368) } else { (if v358 { ((v366 * v1588) / self.scalar_v368) } else { v304 }) });
        let v1777: f64 = (if v379 { ((v220 * (-(v1757 * v1761))) / self.scalar_v368) } else { v304 });
        let v1778: f64 = (if v379 { ((v220 * (-(v1758 * v1761))) / self.scalar_v368) } else { v304 });
        let v1779: f64 = (if v379 { v304 } else { (if v358 { (v363 * ((v374 * v1721) + (v356 * (((v220 * (self.scalar_v371 * v1721)) - (v372 * v1588)) / v1667)))) } else { v304 }) });
        let v1780: f64 = (if v379 { v304 } else { (if v358 { (v363 * ((v374 * self.scalar_v1722) + (v356 * v1734))) } else { v304 }) });
        let v1781: f64 = (if v379 { v304 } else { (if v358 { (v363 * ((v374 * self.scalar_v1723) + (v356 * (self.scalar_v1729 / v220)))) } else { v304 }) });
        let v1788: f64 = (v352 * v1720);
        let v1791: f64 = ((v1788 + v1788) / (v182 * v395));
        let v1792: f64 = (if self.scalar_v390 { v1791 } else { v304 });
        let v1795: f64 = (if self.scalar_v390 { (v191 * (v1720 + v1792)) } else { v304 });
        let v1808: f64 = (if self.scalar_v390 { (((v402 * v1719) + (v350 * ((-(((v220 * v1795) - (v399 * v1588)) / v1667)) * (self.scalar_v368 * f64::powf(v401, self.scalar_v1759))))) / self.scalar_v368) } else { v304 });
        let v1809: f64 = (if self.scalar_v390 { v1720 } else { v304 });
        let v1812: f64 = (v406 * v1809);
        let v1814: f64 = (v406 * self.scalar_v1810);
        let v1816: f64 = (v406 * self.scalar_v1811);
        let v1818: f64 = (v182 * v409);
        let v1822: f64 = (if self.scalar_v390 { ((v1812 + v1812) / v1818) } else { v304 });
        let v1823: f64 = (if self.scalar_v390 { ((v1814 + v1814) / v1818) } else { v304 });
        let v1824: f64 = (if self.scalar_v390 { ((v1816 + v1816) / v1818) } else { v304 });
        let v1832: f64 = (if self.scalar_v390 { ((v185 * (v1809 - v1822)) - v1720) } else { v304 });
        let v1833: f64 = (if self.scalar_v390 { (v185 * (self.scalar_v1810 - v1823)) } else { v304 });
        let v1834: f64 = (if self.scalar_v390 { (v185 * (self.scalar_v1811 - v1824)) } else { v304 });
        let v1845: f64 = (self.scalar_v368 * f64::powf(v416, self.scalar_v1759));
        let v1857: f64 = (if self.scalar_v390 { (((v417 * v1719) + (v350 * ((-(((v220 * v1832) - (v414 * v1588)) / v1667)) * v1845))) / self.scalar_v368) } else { v1776 });
        let v1858: f64 = (if self.scalar_v390 { ((v350 * ((-(v1833 / v220)) * v1845)) / self.scalar_v368) } else { v1777 });
        let v1859: f64 = (if self.scalar_v390 { ((v350 * ((-(v1834 / v220)) * v1845)) / self.scalar_v368) } else { v1778 });
        let v1871: f64 = (if self.scalar_v390 { ((v1857 + (self.scalar_v422 * (v1795 + (-v1832)))) - v1808) } else { (if self.scalar_v354 { (v1776 + v1779) } else { v304 }) });
        let v1872: f64 = (if self.scalar_v390 { (v1858 + (self.scalar_v422 * (v51 - v1833))) } else { (if self.scalar_v354 { (v1777 + v1780) } else { v304 }) });
        let v1873: f64 = (if self.scalar_v390 { (v1859 + (self.scalar_v422 * (v360 - v1834))) } else { (if self.scalar_v354 { (v1778 + v1781) } else { v304 }) });
        let v1874: f64 = (if self.scalar_v354 { v1720 } else { v1721 });
        let v1916: f64 = (self.scalar_v368 * f64::powf(v449, self.scalar_v1759));
        let v1931: f64 = (if v447 { (((v451 * v1588) + (v220 * (-((-((-(v343 * v1588)) / v1667)) * v1916)))) / self.scalar_v368) } else { (if v432 { ((v436 * v1588) / self.scalar_v368) } else { v1857 }) });
        let v1932: f64 = (if v447 { ((v220 * (-(v1757 * v1916))) / self.scalar_v368) } else { v304 });
        let v1933: f64 = (if v447 { v304 } else { (if v432 { v304 } else { v1858 }) });
        let v1934: f64 = (if v447 { ((v220 * (-(v1758 * v1916))) / self.scalar_v368) } else { (if v432 { v304 } else { v1859 }) });
        let v1935: f64 = (if v447 { v304 } else { (if v432 { (v433 * ((v442 * v1874) + (v430 * (((v220 * (self.scalar_v371 * v1874)) - (v440 * v1588)) / v1667)))) } else { v1779 }) });
        let v1936: f64 = (if v447 { v304 } else { (if v432 { (v433 * ((v442 * self.scalar_v1722) + (v430 * v1734))) } else { v304 }) });
        let v1937: f64 = (if v447 { v304 } else { (if v432 { (v433 * ((v442 * self.scalar_v1875) + (v430 * (self.scalar_v1883 / v220)))) } else { v1780 }) });
        let v1938: f64 = (if v447 { v304 } else { (if v432 { (v433 * ((v442 * self.scalar_v1876) + (v430 * (self.scalar_v1884 / v220)))) } else { v1781 }) });
        let v1947: f64 = (if self.scalar_v390 { v1791 } else { v1792 });
        let v1950: f64 = (if self.scalar_v390 { (v191 * (v1720 + v1947)) } else { v1795 });
        let v1963: f64 = (if self.scalar_v390 { (((v464 * v1719) + (v350 * ((-(((v220 * v1950) - (v461 * v1588)) / v1667)) * (self.scalar_v368 * f64::powf(v463, self.scalar_v1759))))) / self.scalar_v368) } else { v1808 });
        let v1964: f64 = (if self.scalar_v390 { v1720 } else { v1809 });
        let v1967: f64 = (v468 * v1964);
        let v1969: f64 = (v468 * self.scalar_v1810);
        let v1971: f64 = (v468 * self.scalar_v1965);
        let v1973: f64 = (v468 * self.scalar_v1966);
        let v1975: f64 = (v182 * v471);
        let v1980: f64 = (if self.scalar_v390 { ((v1967 + v1967) / v1975) } else { v1822 });
        let v1981: f64 = (if self.scalar_v390 { ((v1969 + v1969) / v1975) } else { v304 });
        let v1982: f64 = (if self.scalar_v390 { ((v1971 + v1971) / v1975) } else { v1823 });
        let v1983: f64 = (if self.scalar_v390 { ((v1973 + v1973) / v1975) } else { v1824 });
        let v1993: f64 = (if self.scalar_v390 { ((v185 * (v1964 - v1980)) - v1720) } else { v1832 });
        let v1994: f64 = (if self.scalar_v390 { (v185 * (self.scalar_v1810 - v1981)) } else { v304 });
        let v1995: f64 = (if self.scalar_v390 { (v185 * (self.scalar_v1965 - v1982)) } else { v1833 });
        let v1996: f64 = (if self.scalar_v390 { (v185 * (self.scalar_v1966 - v1983)) } else { v1834 });
        let v2009: f64 = (self.scalar_v368 * f64::powf(v478, self.scalar_v1759));
        let v2024: f64 = (if self.scalar_v390 { (((v479 * v1719) + (v350 * ((-(((v220 * v1993) - (v476 * v1588)) / v1667)) * v2009))) / self.scalar_v368) } else { v1931 });
        let v2025: f64 = (if self.scalar_v390 { ((v350 * ((-(v1994 / v220)) * v2009)) / self.scalar_v368) } else { v1932 });
        let v2026: f64 = (if self.scalar_v390 { ((v350 * ((-(v1995 / v220)) * v2009)) / self.scalar_v368) } else { v1933 });
        let v2027: f64 = (if self.scalar_v390 { ((v350 * ((-(v1996 / v220)) * v2009)) / self.scalar_v368) } else { v1934 });
        let v2046: f64 = (-v1626);
        let v2047: f64 = (self.scalar_v351 * v2046);
        let v2048: f64 = (if self.scalar_v492 { v2047 } else { v1874 });
        let v2068: f64 = (self.scalar_v2060 / v247);
        let v2099: f64 = ((-(self.scalar_v514 * v1626)) / v1676);
        let v2103: f64 = (v2099 * (self.scalar_v504 * f64::powf(v523, self.scalar_v2100)));
        let v2107: f64 = (v527 * v527);
        let v2128: f64 = ((v247 * (-(v524 * (-(self.scalar_v2104 / v527))))) / self.scalar_v504);
        let v2129: f64 = ((v247 * (-(v524 * (-(self.scalar_v504 / v527))))) / self.scalar_v504);
        let v2130: f64 = (if v521 { (((v531 * v1626) + (v247 * (-((v529 * v2103) + (v524 * (-((-(v526 * v1626)) / v2107))))))) / self.scalar_v504) } else { (if v496 { ((v502 * v1626) / self.scalar_v504) } else { v2024 }) });
        let v2141: f64 = (-(v360 / v247));
        let v2142: f64 = (-(v51 / v247));
        let v2144: f64 = (self.scalar_v504 * f64::powf(v538, self.scalar_v2100));
        let v2159: f64 = (if v536 { (((v540 * v1626) + (v247 * (-((-((-(v345 * v1626)) / v1676)) * v2144)))) / self.scalar_v504) } else { v2130 });
        let v2160: f64 = (if v536 { ((v247 * (-(v2141 * v2144))) / self.scalar_v504) } else { (if v521 { v2128 } else { v304 }) });
        let v2161: f64 = (if v536 { v304 } else { (if v521 { v304 } else { (if v496 { v304 } else { v2025 }) }) });
        let v2162: f64 = (if v536 { ((v247 * (-(v2142 * v2144))) / self.scalar_v504) } else { (if v521 { v2129 } else { (if v496 { v304 } else { v2026 }) }) });
        let v2163: f64 = (if v536 { v304 } else { (if v521 { v304 } else { (if v496 { v304 } else { v2027 }) }) });
        let v2164: f64 = (if v520 { v304 } else { (if v496 { (v499 * ((v510 * v2048) + (v494 * (((v247 * (self.scalar_v507 * v2048)) - (v508 * v1626)) / v1676)))) } else { v1935 }) });
        let v2165: f64 = (if v520 { v304 } else { (if v496 { (v499 * ((v510 * self.scalar_v2049) + (v494 * v2068))) } else { v304 }) });
        let v2166: f64 = (if v520 { v304 } else { (if v496 { (v499 * ((v510 * self.scalar_v2050) + (v494 * (self.scalar_v2061 / v247)))) } else { v1936 }) });
        let v2167: f64 = (if v520 { v304 } else { (if v496 { (v499 * ((v510 * self.scalar_v2051) + (v494 * (self.scalar_v2062 / v247)))) } else { v1937 }) });
        let v2168: f64 = (if v520 { v304 } else { (if v496 { (v499 * ((v510 * self.scalar_v2052) + (v494 * (self.scalar_v2063 / v247)))) } else { v1938 }) });
        let v2179: f64 = (-v2047);
        let v2180: f64 = (v553 * v2047);
        let v2183: f64 = (v553 * v553);
        let v2184: f64 = ((v2180 - (v552 * v2179)) / v2183);
        let v2185: f64 = (if self.scalar_v551 { v2184 } else { v304 });
        let v2187: f64 = (v557 * v2185);
        let v2191: f64 = (v563 * v2185);
        let v2200: f64 = (((v569 * (v182 * v2185)) - (v556 * (((v2187 + v2187) / (v182 * v562)) + ((v2191 + v2191) / (v182 * v568))))) / (v569 * v569));
        let v2201: f64 = (if self.scalar_v551 { v2200 } else { v304 });
        let v2207: f64 = (if self.scalar_v551 { (v185 * (((v571 * v2179) + (v553 * v2201)) - v2047)) } else { v1950 });
        let v2220: f64 = (((v580 * v1626) + (v247 * (-((-(((v247 * v2207) - (v576 * v1626)) / v1676)) * (self.scalar_v504 * f64::powf(v578, self.scalar_v2100)))))) / self.scalar_v504);
        let v2221: f64 = (if self.scalar_v551 { v2220 } else { v304 });
        let v2228: f64 = (if self.scalar_v551 { ((v2180 - (v586 * v2179)) / v2183) } else { v304 });
        let v2229: f64 = (if self.scalar_v551 { (-2.0 / v553) } else { v304 });
        let v2230: f64 = (if self.scalar_v551 { (v182 / v553) } else { v304 });
        let v2232: f64 = (v182 * v2229);
        let v2233: f64 = (v182 * v2230);
        let v2234: f64 = (v590 * v2228);
        let v2236: f64 = (v590 * v2229);
        let v2238: f64 = (v590 * v2230);
        let v2240: f64 = (v182 * v593);
        let v2244: f64 = (v594 * v2228);
        let v2246: f64 = (v594 * v2229);
        let v2248: f64 = (v594 * v2230);
        let v2250: f64 = (v182 * v597);
        let v2260: f64 = (v598 * v598);
        let v2270: f64 = (if self.scalar_v551 { (((v598 * (v182 * v2228)) - (v589 * (((v2234 + v2234) / v2240) + ((v2244 + v2244) / v2250)))) / v2260) } else { v304 });
        let v2271: f64 = (if self.scalar_v551 { (((v598 * v2232) - (v589 * (((v2236 + v2236) / v2240) + ((v2246 + v2246) / v2250)))) / v2260) } else { v304 });
        let v2272: f64 = (if self.scalar_v551 { (((v598 * v2233) - (v589 * (((v2238 + v2238) / v2240) + ((v2248 + v2248) / v2250)))) / v2260) } else { v304 });
        let v2282: f64 = (if self.scalar_v551 { (v185 * (((v600 * v2179) + (v553 * v2270)) - v2047)) } else { v1993 });
        let v2283: f64 = (if self.scalar_v551 { (v185 * (v553 * v2271)) } else { v304 });
        let v2284: f64 = (if self.scalar_v551 { v304 } else { v1994 });
        let v2285: f64 = (if self.scalar_v551 { (v185 * (v553 * v2272)) } else { v1995 });
        let v2286: f64 = (if self.scalar_v551 { v304 } else { v1996 });
        let v2301: f64 = (self.scalar_v504 * f64::powf(v607, self.scalar_v2100));
        let v2324: f64 = (if self.scalar_v551 { (((v609 * v1626) + (v247 * (-((-(((v247 * v2282) - (v605 * v1626)) / v1676)) * v2301)))) / self.scalar_v504) } else { v2159 });
        let v2325: f64 = (if self.scalar_v551 { ((v247 * (-((-(v2283 / v247)) * v2301))) / self.scalar_v504) } else { v2160 });
        let v2326: f64 = (if self.scalar_v551 { ((v247 * (-((-(v2284 / v247)) * v2301))) / self.scalar_v504) } else { v2161 });
        let v2327: f64 = (if self.scalar_v551 { ((v247 * (-((-(v2285 / v247)) * v2301))) / self.scalar_v504) } else { v2162 });
        let v2328: f64 = (if self.scalar_v551 { ((v247 * (-((-(v2286 / v247)) * v2301))) / self.scalar_v504) } else { v2163 });
        let v2332: f64 = (if self.scalar_v551 { (v185 * v2270) } else { v304 });
        let v2333: f64 = (if self.scalar_v551 { (v185 * v2271) } else { v304 });
        let v2334: f64 = (if self.scalar_v551 { (v185 * v2272) } else { v304 });
        let v2338: f64 = (v2099 * (self.scalar_v616 * f64::powf(v523, self.scalar_v2335)));
        let v2339: f64 = (if self.scalar_v551 { v2338 } else { v304 });
        let v2346: f64 = ((((v247 * v2047) - (v490 * v1626)) / v1676) * (self.scalar_v616 * f64::powf(v620, self.scalar_v2335)));
        let v2347: f64 = (if self.scalar_v551 { v2346 } else { v304 });
        let v2364: f64 = (if self.scalar_v551 { (((v623 * v2339) + (v618 * (-v2332))) + ((v622 * v2332) + (v615 * v2347))) } else { v304 });
        let v2365: f64 = (if self.scalar_v551 { ((v618 * (-v2333)) + (v622 * v2333)) } else { v304 });
        let v2366: f64 = (if self.scalar_v551 { ((v618 * (-v2334)) + (v622 * v2334)) } else { v304 });
        let v2384: f64 = (if self.scalar_v551 { ((v629 * v2364) + (v627 * (v2207 + (-v2282)))) } else { v304 });
        let v2385: f64 = (if self.scalar_v551 { ((v629 * v2365) + (v627 * (v360 - v2283))) } else { v304 });
        let v2386: f64 = (if self.scalar_v551 { (v627 * (-v2284)) } else { v304 });
        let v2387: f64 = (if self.scalar_v551 { ((v629 * v2366) + (v627 * (v51 - v2285))) } else { v304 });
        let v2388: f64 = (if self.scalar_v551 { (v627 * (-v2286)) } else { v304 });
        let v2400: f64 = (v490 * v2047);
        let v2403: f64 = ((v2400 + v2400) / (v182 * v639));
        let v2404: f64 = (if self.scalar_v636 { v2403 } else { v1947 });
        let v2407: f64 = (if self.scalar_v636 { (v191 * (v2047 + v2404)) } else { v2207 });
        let v2420: f64 = (if self.scalar_v636 { (((v646 * v2046) + (v489 * ((-(((v247 * v2407) - (v643 * v1626)) / v1676)) * (self.scalar_v504 * f64::powf(v645, self.scalar_v2100))))) / self.scalar_v504) } else { v1963 });
        let v2421: f64 = (if self.scalar_v636 { v2047 } else { v1964 });
        let v2426: f64 = (v650 * v2421);
        let v2428: f64 = (v650 * self.scalar_v2422);
        let v2430: f64 = (v650 * self.scalar_v2423);
        let v2432: f64 = (v650 * self.scalar_v2424);
        let v2434: f64 = (v650 * self.scalar_v2425);
        let v2436: f64 = (v182 * v653);
        let v2442: f64 = (if self.scalar_v636 { ((v2426 + v2426) / v2436) } else { v1980 });
        let v2443: f64 = (if self.scalar_v636 { ((v2428 + v2428) / v2436) } else { v304 });
        let v2444: f64 = (if self.scalar_v636 { ((v2430 + v2430) / v2436) } else { v1981 });
        let v2445: f64 = (if self.scalar_v636 { ((v2432 + v2432) / v2436) } else { v1982 });
        let v2446: f64 = (if self.scalar_v636 { ((v2434 + v2434) / v2436) } else { v1983 });
        let v2458: f64 = (if self.scalar_v636 { ((v185 * (v2421 - v2442)) - v2047) } else { v2282 });
        let v2459: f64 = (if self.scalar_v636 { (v185 * (self.scalar_v2422 - v2443)) } else { v2283 });
        let v2460: f64 = (if self.scalar_v636 { (v185 * (self.scalar_v2423 - v2444)) } else { v2284 });
        let v2461: f64 = (if self.scalar_v636 { (v185 * (self.scalar_v2424 - v2445)) } else { v2285 });
        let v2462: f64 = (if self.scalar_v636 { (v185 * (self.scalar_v2425 - v2446)) } else { v2286 });
        let v2477: f64 = (self.scalar_v504 * f64::powf(v660, self.scalar_v2100));
        let v2495: f64 = (if self.scalar_v636 { (((v661 * v2046) + (v489 * ((-(((v247 * v2458) - (v658 * v1626)) / v1676)) * v2477))) / self.scalar_v504) } else { v2324 });
        let v2496: f64 = (if self.scalar_v636 { ((v489 * ((-(v2459 / v247)) * v2477)) / self.scalar_v504) } else { v2325 });
        let v2497: f64 = (if self.scalar_v636 { ((v489 * ((-(v2460 / v247)) * v2477)) / self.scalar_v504) } else { v2326 });
        let v2498: f64 = (if self.scalar_v636 { ((v489 * ((-(v2461 / v247)) * v2477)) / self.scalar_v504) } else { v2327 });
        let v2499: f64 = (if self.scalar_v636 { ((v489 * ((-(v2462 / v247)) * v2477)) / self.scalar_v504) } else { v2328 });
        let v2517: f64 = (if self.scalar_v636 { ((v2495 + (self.scalar_v665 * (v2407 + (-v2458)))) - v2420) } else { (if self.scalar_v551 { ((v2324 + v2384) - v2221) } else { (if self.scalar_v492 { (v2159 + v2164) } else { v304 }) }) });
        let v2518: f64 = (if self.scalar_v636 { (v2496 + (self.scalar_v665 * (v360 - v2459))) } else { (if self.scalar_v551 { (v2325 + v2385) } else { (if self.scalar_v492 { (v2160 + v2165) } else { v304 }) }) });
        let v2519: f64 = (if self.scalar_v636 { (v2497 + (self.scalar_v665 * (-v2460))) } else { (if self.scalar_v551 { (v2326 + v2386) } else { (if self.scalar_v492 { (v2161 + v2166) } else { v304 }) }) });
        let v2520: f64 = (if self.scalar_v636 { (v2498 + (self.scalar_v665 * (v51 - v2461))) } else { (if self.scalar_v551 { (v2327 + v2387) } else { (if self.scalar_v492 { (v2162 + v2167) } else { v304 }) }) });
        let v2521: f64 = (if self.scalar_v636 { (v2499 + (self.scalar_v665 * (-v2462))) } else { (if self.scalar_v551 { (v2328 + v2388) } else { (if self.scalar_v492 { (v2163 + v2168) } else { v304 }) }) });
        let v2522: f64 = (if self.scalar_v492 { v2047 } else { v2048 });
        let v2589: f64 = (if v693 { (((v699 * v1626) + (v247 * (-((v697 * v2103) + (v524 * (-((-(v695 * v1626)) / v2107))))))) / self.scalar_v504) } else { (if v675 { ((v679 * v1626) / self.scalar_v504) } else { v2495 }) });
        let v2600: f64 = (self.scalar_v504 * f64::powf(v706, self.scalar_v2100));
        let v2615: f64 = (if v704 { (((v708 * v1626) + (v247 * (-((-((-(v349 * v1626)) / v1676)) * v2600)))) / self.scalar_v504) } else { v2589 });
        let v2616: f64 = (if v704 { v304 } else { (if v693 { v304 } else { (if v675 { v304 } else { v2496 }) }) });
        let v2617: f64 = (if v704 { ((v247 * (-(v2142 * v2600))) / self.scalar_v504) } else { (if v693 { v2129 } else { (if v675 { v304 } else { v2497 }) }) });
        let v2618: f64 = (if v704 { v304 } else { (if v693 { v304 } else { (if v675 { v304 } else { v2498 }) }) });
        let v2619: f64 = (if v704 { v304 } else { (if v693 { v304 } else { (if v675 { v304 } else { v2499 }) }) });
        let v2620: f64 = (if v704 { ((v247 * (-(v2141 * v2600))) / self.scalar_v504) } else { (if v693 { v2128 } else { v304 }) });
        let v2621: f64 = (if v692 { v304 } else { (if v675 { (v676 * ((v685 * v2522) + (v673 * (((v247 * (self.scalar_v507 * v2522)) - (v683 * v1626)) / v1676)))) } else { v2164 }) });
        let v2622: f64 = (if v692 { v304 } else { (if v675 { (v676 * ((v685 * self.scalar_v2523) + (v673 * (self.scalar_v2535 / v247)))) } else { v2165 }) });
        let v2623: f64 = (if v692 { v304 } else { (if v675 { (v676 * ((v685 * self.scalar_v2524) + (v673 * (self.scalar_v2536 / v247)))) } else { v2166 }) });
        let v2624: f64 = (if v692 { v304 } else { (if v675 { (v676 * ((v685 * self.scalar_v2525) + (v673 * (self.scalar_v2537 / v247)))) } else { v2167 }) });
        let v2625: f64 = (if v692 { v304 } else { (if v675 { (v676 * ((v685 * self.scalar_v2526) + (v673 * (self.scalar_v2538 / v247)))) } else { v2168 }) });
        let v2626: f64 = (if v692 { v304 } else { (if v675 { (v676 * ((v685 * self.scalar_v2049) + (v673 * v2068))) } else { v304 }) });
        let v2639: f64 = (if self.scalar_v551 { v2184 } else { v2185 });
        let v2641: f64 = (v717 * v2639);
        let v2645: f64 = (v721 * v2639);
        let v2654: f64 = (((v725 * (v182 * v2639)) - (v716 * (((v2641 + v2641) / (v182 * v720)) + ((v2645 + v2645) / (v182 * v724))))) / (v725 * v725));
        let v2661: f64 = (if self.scalar_v551 { (v185 * (((v727 * v2179) + (v553 * (if self.scalar_v551 { v2654 } else { v2201 }))) - v2047)) } else { v2407 });
        let v2674: f64 = (((v736 * v1626) + (v247 * (-((-(((v247 * v2661) - (v732 * v1626)) / v1676)) * (self.scalar_v504 * f64::powf(v734, self.scalar_v2100)))))) / self.scalar_v504);
        let v2679: f64 = (if self.scalar_v551 { ((v2180 - (v742 * v2179)) / v2183) } else { v2228 });
        let v2680: f64 = (if self.scalar_v551 { v304 } else { v2229 });
        let v2681: f64 = (if self.scalar_v551 { v304 } else { v2230 });
        let v2685: f64 = (v746 * v2679);
        let v2687: f64 = (v746 * v2680);
        let v2689: f64 = (v746 * v2230);
        let v2691: f64 = (v746 * v2681);
        let v2693: f64 = (v746 * v2229);
        let v2695: f64 = (v182 * v749);
        let v2701: f64 = (v750 * v2679);
        let v2703: f64 = (v750 * v2680);
        let v2705: f64 = (v750 * v2230);
        let v2707: f64 = (v750 * v2681);
        let v2709: f64 = (v750 * v2229);
        let v2711: f64 = (v182 * v753);
        let v2725: f64 = (v754 * v754);
        let v2743: f64 = (if self.scalar_v551 { (((v754 * (v182 * v2679)) - (v745 * (((v2685 + v2685) / v2695) + ((v2701 + v2701) / v2711)))) / v2725) } else { v2270 });
        let v2744: f64 = (if self.scalar_v551 { (((v754 * (v182 * v2680)) - (v745 * (((v2687 + v2687) / v2695) + ((v2703 + v2703) / v2711)))) / v2725) } else { v2271 });
        let v2745: f64 = (if self.scalar_v551 { (((v754 * v2233) - (v745 * (((v2689 + v2689) / v2695) + ((v2705 + v2705) / v2711)))) / v2725) } else { v304 });
        let v2746: f64 = (if self.scalar_v551 { (((v754 * (v182 * v2681)) - (v745 * (((v2691 + v2691) / v2695) + ((v2707 + v2707) / v2711)))) / v2725) } else { v2272 });
        let v2747: f64 = (if self.scalar_v551 { (((v754 * v2232) - (v745 * (((v2693 + v2693) / v2695) + ((v2709 + v2709) / v2711)))) / v2725) } else { v304 });
        let v2761: f64 = (if self.scalar_v551 { (v185 * (((v756 * v2179) + (v553 * v2743)) - v2047)) } else { v2458 });
        let v2762: f64 = (if self.scalar_v551 { (v185 * (v553 * v2744)) } else { v2459 });
        let v2763: f64 = (if self.scalar_v551 { (v185 * (v553 * v2745)) } else { v2460 });
        let v2764: f64 = (if self.scalar_v551 { (v185 * (v553 * v2746)) } else { v2461 });
        let v2765: f64 = (if self.scalar_v551 { v304 } else { v2462 });
        let v2766: f64 = (if self.scalar_v551 { (v185 * (v553 * v2747)) } else { v304 });
        let v2783: f64 = (self.scalar_v504 * f64::powf(v763, self.scalar_v2100));
        let v2810: f64 = (if self.scalar_v551 { (((v765 * v1626) + (v247 * (-((-(((v247 * v2761) - (v761 * v1626)) / v1676)) * v2783)))) / self.scalar_v504) } else { v2615 });
        let v2811: f64 = (if self.scalar_v551 { ((v247 * (-((-(v2762 / v247)) * v2783))) / self.scalar_v504) } else { v2616 });
        let v2812: f64 = (if self.scalar_v551 { ((v247 * (-((-(v2763 / v247)) * v2783))) / self.scalar_v504) } else { v2617 });
        let v2813: f64 = (if self.scalar_v551 { ((v247 * (-((-(v2764 / v247)) * v2783))) / self.scalar_v504) } else { v2618 });
        let v2814: f64 = (if self.scalar_v551 { ((v247 * (-((-(v2765 / v247)) * v2783))) / self.scalar_v504) } else { v2619 });
        let v2815: f64 = (if self.scalar_v551 { ((v247 * (-((-(v2766 / v247)) * v2783))) / self.scalar_v504) } else { v2620 });
        let v2821: f64 = (if self.scalar_v551 { (v185 * v2743) } else { v2332 });
        let v2822: f64 = (if self.scalar_v551 { (v185 * v2744) } else { v2333 });
        let v2823: f64 = (if self.scalar_v551 { (v185 * v2745) } else { v304 });
        let v2824: f64 = (if self.scalar_v551 { (v185 * v2746) } else { v2334 });
        let v2825: f64 = (if self.scalar_v551 { (v185 * v2747) } else { v304 });
        let v2852: f64 = (if self.scalar_v551 { (((v774 * (if self.scalar_v551 { v2338 } else { v2339 })) + (v772 * (-v2821))) + ((v773 * v2821) + (v771 * (if self.scalar_v551 { v2346 } else { v2347 })))) } else { v2364 });
        let v2893: f64 = (if self.scalar_v551 { ((v2810 + (if self.scalar_v551 { ((v780 * v2852) + (v778 * (v2661 + (-v2761)))) } else { v2384 })) - (if self.scalar_v551 { v2674 } else { v2221 })) } else { (if self.scalar_v492 { (v2615 + v2621) } else { v304 }) });
        let v2894: f64 = (if self.scalar_v551 { (v2811 + (if self.scalar_v551 { ((v780 * (if self.scalar_v551 { ((v772 * (-v2822)) + (v773 * v2822)) } else { v2365 })) + (v778 * (-v2762))) } else { v2385 })) } else { (if self.scalar_v492 { (v2616 + v2622) } else { v304 }) });
        let v2895: f64 = (if self.scalar_v551 { (v2812 + (if self.scalar_v551 { ((v780 * (if self.scalar_v551 { ((v772 * (-v2823)) + (v773 * v2823)) } else { v304 })) + (v778 * (v51 - v2763))) } else { v2386 })) } else { (if self.scalar_v492 { (v2617 + v2623) } else { v304 }) });
        let v2896: f64 = (if self.scalar_v551 { (v2813 + (if self.scalar_v551 { ((v780 * (if self.scalar_v551 { ((v772 * (-v2824)) + (v773 * v2824)) } else { v2366 })) + (v778 * (-v2764))) } else { v2387 })) } else { (if self.scalar_v492 { (v2618 + v2624) } else { v304 }) });
        let v2898: f64 = (if self.scalar_v551 { (v2815 + (if self.scalar_v551 { ((v780 * (if self.scalar_v551 { ((v772 * (-v2825)) + (v773 * v2825)) } else { v304 })) + (v778 * (v360 - v2766))) } else { v304 })) } else { (if self.scalar_v492 { (v2620 + v2626) } else { v304 }) });
        let v2899: f64 = (if self.scalar_v636 { v2403 } else { v2404 });
        let v2902: f64 = (if self.scalar_v636 { (v191 * (v2047 + v2899)) } else { v2661 });
        let v2915: f64 = (if self.scalar_v636 { (((v792 * v2046) + (v489 * ((-(((v247 * v2902) - (v789 * v1626)) / v1676)) * (self.scalar_v504 * f64::powf(v791, self.scalar_v2100))))) / self.scalar_v504) } else { v2420 });
        let v2916: f64 = (if self.scalar_v636 { v2047 } else { v2421 });
        let v2921: f64 = (v796 * v2916);
        let v2923: f64 = (v796 * self.scalar_v2917);
        let v2925: f64 = (v796 * self.scalar_v2918);
        let v2927: f64 = (v796 * self.scalar_v2919);
        let v2929: f64 = (v796 * self.scalar_v2920);
        let v2931: f64 = (v796 * self.scalar_v2422);
        let v2933: f64 = (v182 * v799);
        let v2940: f64 = (if self.scalar_v636 { ((v2921 + v2921) / v2933) } else { v2442 });
        let v2941: f64 = (if self.scalar_v636 { ((v2923 + v2923) / v2933) } else { v2443 });
        let v2942: f64 = (if self.scalar_v636 { ((v2925 + v2925) / v2933) } else { v2444 });
        let v2943: f64 = (if self.scalar_v636 { ((v2927 + v2927) / v2933) } else { v2445 });
        let v2944: f64 = (if self.scalar_v636 { ((v2929 + v2929) / v2933) } else { v2446 });
        let v2945: f64 = (if self.scalar_v636 { ((v2931 + v2931) / v2933) } else { v304 });
        let v2959: f64 = (if self.scalar_v636 { ((v185 * (v2916 - v2940)) - v2047) } else { v2761 });
        let v2960: f64 = (if self.scalar_v636 { (v185 * (self.scalar_v2917 - v2941)) } else { v2762 });
        let v2961: f64 = (if self.scalar_v636 { (v185 * (self.scalar_v2918 - v2942)) } else { v2763 });
        let v2962: f64 = (if self.scalar_v636 { (v185 * (self.scalar_v2919 - v2943)) } else { v2764 });
        let v2963: f64 = (if self.scalar_v636 { (v185 * (self.scalar_v2920 - v2944)) } else { v2765 });
        let v2964: f64 = (if self.scalar_v636 { (v185 * (self.scalar_v2422 - v2945)) } else { v2766 });
        let v2981: f64 = (self.scalar_v504 * f64::powf(v806, self.scalar_v2100));
        let v3002: f64 = (if self.scalar_v636 { (((v807 * v2046) + (v489 * ((-(((v247 * v2959) - (v804 * v1626)) / v1676)) * v2981))) / self.scalar_v504) } else { v2810 });
        let v3003: f64 = (if self.scalar_v636 { ((v489 * ((-(v2960 / v247)) * v2981)) / self.scalar_v504) } else { v2811 });
        let v3004: f64 = (if self.scalar_v636 { ((v489 * ((-(v2961 / v247)) * v2981)) / self.scalar_v504) } else { v2812 });
        let v3005: f64 = (if self.scalar_v636 { ((v489 * ((-(v2962 / v247)) * v2981)) / self.scalar_v504) } else { v2813 });
        let v3006: f64 = (if self.scalar_v636 { ((v489 * ((-(v2963 / v247)) * v2981)) / self.scalar_v504) } else { v2814 });
        let v3007: f64 = (if self.scalar_v636 { ((v489 * ((-(v2964 / v247)) * v2981)) / self.scalar_v504) } else { v2815 });
        let v3032: f64 = (if self.scalar_v636 { (v3006 + (self.scalar_v665 * (-v2963))) } else { (if self.scalar_v551 { (v2814 + (if self.scalar_v551 { (v778 * (-v2765)) } else { v2388 })) } else { (if self.scalar_v492 { (v2619 + v2625) } else { v304 }) }) });
        let v3034: f64 = (-v1664);
        let v3036: f64 = (if self.scalar_v817 { (self.scalar_v351 * v3034) } else { v2047 });
        let v3037: f64 = (if self.scalar_v823 { v3036 } else { v2522 });
        let v3114: f64 = (self.scalar_v837 * f64::powf(v850, self.scalar_v3112));
        let v3129: f64 = (if v848 { (((v852 * v1664) + (v274 * (-((-((-(v825 * v1664)) / v1685)) * v3114)))) / self.scalar_v837) } else { (if v829 { ((v835 * v1664) / self.scalar_v837) } else { v3002 }) });
        let v3130: f64 = (if v848 { v304 } else { (if v829 { v304 } else { v3003 }) });
        let v3131: f64 = (if v848 { v304 } else { (if v829 { v304 } else { v3004 }) });
        let v3132: f64 = (if v848 { v304 } else { (if v829 { v304 } else { v3005 }) });
        let v3133: f64 = (if v848 { v304 } else { (if v829 { v304 } else { v3006 }) });
        let v3134: f64 = (if v848 { ((v274 * (-((-(v360 / v274)) * v3114))) / self.scalar_v837) } else { (if v829 { v304 } else { v3007 }) });
        let v3135: f64 = (if v848 { ((v274 * (-((-(v51 / v274)) * v3114))) / self.scalar_v837) } else { v304 });
        let v3136: f64 = (if v848 { v304 } else { (if v829 { (v832 * ((v843 * v3037) + (v827 * (((v274 * (self.scalar_v840 * v3037)) - (v841 * v1664)) / v1685)))) } else { v2621 }) });
        let v3157: f64 = (v820 * v3036);
        let v3164: f64 = (if self.scalar_v860 { (v191 * (v3036 + (if self.scalar_v860 { ((v3157 + v3157) / (v182 * v865)) } else { v2899 }))) } else { v2902 });
        let v3177: f64 = (if self.scalar_v860 { (((v872 * v3034) + (v818 * ((-(((v274 * v3164) - (v869 * v1664)) / v1685)) * (self.scalar_v837 * f64::powf(v871, self.scalar_v3112))))) / self.scalar_v837) } else { v2915 });
        let v3178: f64 = (if self.scalar_v860 { v3036 } else { v2916 });
        let v3185: f64 = (v876 * v3178);
        let v3187: f64 = (v876 * self.scalar_v3179);
        let v3189: f64 = (v876 * self.scalar_v3180);
        let v3191: f64 = (v876 * self.scalar_v3181);
        let v3193: f64 = (v876 * self.scalar_v3182);
        let v3195: f64 = (v876 * self.scalar_v3183);
        let v3197: f64 = (v876 * self.scalar_v3184);
        let v3199: f64 = (v182 * v879);
        let v3229: f64 = (if self.scalar_v860 { ((v185 * (v3178 - (if self.scalar_v860 { ((v3185 + v3185) / v3199) } else { v2940 }))) - v3036) } else { v2959 });
        let v3230: f64 = (if self.scalar_v860 { (v185 * (self.scalar_v3179 - (if self.scalar_v860 { ((v3187 + v3187) / v3199) } else { v2941 }))) } else { v2960 });
        let v3231: f64 = (if self.scalar_v860 { (v185 * (self.scalar_v3180 - (if self.scalar_v860 { ((v3189 + v3189) / v3199) } else { v2942 }))) } else { v2961 });
        let v3232: f64 = (if self.scalar_v860 { (v185 * (self.scalar_v3181 - (if self.scalar_v860 { ((v3191 + v3191) / v3199) } else { v2943 }))) } else { v2962 });
        let v3233: f64 = (if self.scalar_v860 { (v185 * (self.scalar_v3182 - (if self.scalar_v860 { ((v3193 + v3193) / v3199) } else { v2944 }))) } else { v2963 });
        let v3234: f64 = (if self.scalar_v860 { (v185 * (self.scalar_v3183 - (if self.scalar_v860 { ((v3195 + v3195) / v3199) } else { v2945 }))) } else { v2964 });
        let v3235: f64 = (if self.scalar_v860 { (v185 * (self.scalar_v3184 - (if self.scalar_v860 { ((v3197 + v3197) / v3199) } else { v304 }))) } else { v304 });
        let v3254: f64 = (self.scalar_v837 * f64::powf(v886, self.scalar_v3112));
        let v3300: f64 = ((if self.scalar_v860 { (((v887 * v3034) + (v818 * ((-(((v274 * v3229) - (v884 * v1664)) / v1685)) * v3254))) / self.scalar_v837) } else { v3129 }) + (self.scalar_v892 * (v3164 + (-v3229))));
        let v3309: f64 = (if self.scalar_v860 { ((if self.scalar_v860 { ((v818 * ((-(v3230 / v274)) * v3254)) / self.scalar_v837) } else { v3130 }) + (self.scalar_v892 * (-v3230))) } else { (if self.scalar_v823 { (v3130 + (if v848 { v304 } else { (if v829 { (v832 * ((v843 * self.scalar_v3038) + (v827 * (self.scalar_v3053 / v274)))) } else { v2622 }) })) } else { v304 }) });
        let v3310: f64 = (if self.scalar_v860 { ((if self.scalar_v860 { ((v818 * ((-(v3231 / v274)) * v3254)) / self.scalar_v837) } else { v3131 }) + (self.scalar_v892 * (-v3231))) } else { (if self.scalar_v823 { (v3131 + (if v848 { v304 } else { (if v829 { (v832 * ((v843 * self.scalar_v3039) + (v827 * (self.scalar_v3054 / v274)))) } else { v2623 }) })) } else { v304 }) });
        let v3311: f64 = (if self.scalar_v860 { ((if self.scalar_v860 { ((v818 * ((-(v3232 / v274)) * v3254)) / self.scalar_v837) } else { v3132 }) + (self.scalar_v892 * (-v3232))) } else { (if self.scalar_v823 { (v3132 + (if v848 { v304 } else { (if v829 { (v832 * ((v843 * self.scalar_v3040) + (v827 * (self.scalar_v3055 / v274)))) } else { v2624 }) })) } else { v304 }) });
        let v3312: f64 = (if self.scalar_v860 { ((if self.scalar_v860 { ((v818 * ((-(v3233 / v274)) * v3254)) / self.scalar_v837) } else { v3133 }) + (self.scalar_v892 * (-v3233))) } else { (if self.scalar_v823 { (v3133 + (if v848 { v304 } else { (if v829 { (v832 * ((v843 * self.scalar_v3041) + (v827 * (self.scalar_v3056 / v274)))) } else { v2625 }) })) } else { v304 }) });
        let v3313: f64 = (if self.scalar_v860 { ((if self.scalar_v860 { ((v818 * ((-(v3234 / v274)) * v3254)) / self.scalar_v837) } else { v3134 }) + (self.scalar_v892 * (v360 - v3234))) } else { (if self.scalar_v823 { (v3134 + (if v848 { v304 } else { (if v829 { (v832 * ((v843 * self.scalar_v3042) + (v827 * (self.scalar_v3057 / v274)))) } else { v2626 }) })) } else { v304 }) });
        let v3314: f64 = (if self.scalar_v860 { ((if self.scalar_v860 { ((v818 * ((-(v3235 / v274)) * v3254)) / self.scalar_v837) } else { v3135 }) + (self.scalar_v892 * (v51 - v3235))) } else { (if self.scalar_v823 { (v3135 + (if v848 { v304 } else { (if v829 { (v832 * ((v843 * self.scalar_v3043) + (v827 * (self.scalar_v3058 / v274)))) } else { v304 }) })) } else { v304 }) });
        let v3331: f64 = { let limexp_arg = v902; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v3337: f64 = ((v904 * v1406) + (v60 * (((-(v341 * ((v162 * v1345) + (v11 * self.scalar_v1533)))) / (v901 * v901)) * v3331)));
        let v3338: f64 = (v60 * ((v51 / v901) * v3331));
        let v3339: f64 = (v60 * ((v360 / v901) * v3331));
        let v3346: f64 = ((-(v345 * ((v163 * v1345) + (v11 * self.scalar_v1534)))) / (v906 * v906));
        let v3347: f64 = (v360 / v906);
        let v3348: f64 = (v51 / v906);
        let v3349: f64 = { let limexp_arg = v907; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v3350: f64 = (v3346 * v3349);
        let v3351: f64 = (v3347 * v3349);
        let v3352: f64 = (v3348 * v3349);
        let v3358: f64 = ((v910 * ((v73 * v1406) + (v60 * (self.scalar_v61 * v1423)))) + (v909 * v3350));
        let v3359: f64 = (v909 * v3351);
        let v3360: f64 = (v909 * v3352);
        let v3365: f64 = (self.scalar_v307 * v2518);
        let v3366: f64 = (self.scalar_v307 * v2519);
        let v3369: f64 = ((self.scalar_v311 * v1871) + (self.scalar_v307 * v2517));
        let v3370: f64 = ((self.scalar_v311 * v1872) + (self.scalar_v307 * v2520));
        let v3371: f64 = ((self.scalar_v311 * v1873) + (self.scalar_v307 * v2521));
        let v3372: f64 = (v917 * v3369);
        let v3374: f64 = (v917 * v3365);
        let v3376: f64 = (v917 * v3366);
        let v3378: f64 = (v917 * v3370);
        let v3380: f64 = (v917 * v3371);
        let v3382: f64 = (v182 * v921);
        let v3393: f64 = (v185 * (v3369 + ((v3372 + v3372) / v3382)));
        let v3394: f64 = (v185 * (v3365 + ((v3374 + v3374) / v3382)));
        let v3395: f64 = (v185 * (v3366 + ((v3376 + v3376) / v3382)));
        let v3396: f64 = (v185 * (v3370 + ((v3378 + v3378) / v3382)));
        let v3397: f64 = (v185 * (v3371 + ((v3380 + v3380) / v3382)));
        let v3406: f64 = (((v905 * (if self.scalar_v312 { ((-(self.scalar_v14 * (self.scalar_v1346 * (self.scalar_v15 * f64::powf(v12, self.scalar_v1347))))) / (v17 * v17)) } else { v304 })) + (v314 * v3337)) + (self.scalar_v318 * v3358));
        let v3410: f64 = (self.scalar_v932 * f64::powf(v925, self.scalar_v3408));
        let v3416: f64 = (v209 * v3406);
        let v3417: f64 = (v209 * (self.scalar_v318 * v3359));
        let v3418: f64 = (v209 * ((v314 * v3338) + (self.scalar_v318 * v3360)));
        let v3419: f64 = (v209 * (v314 * v3339));
        let v3426: f64 = (self.scalar_v931 * f64::powf(v935, self.scalar_v3424));
        let v3453: f64 = (self.scalar_v931 * f64::powf(v942, self.scalar_v3424));
        let v3471: f64 = (if self.scalar_v940 { ((v944 * (v185 * v3393)) + (v941 * (v3416 * v3453))) } else { (if self.scalar_v930 { (v185 * (v3393 + (((v3393 * v3410) + v3416) * v3426))) } else { v304 }) });
        let v3472: f64 = (if self.scalar_v940 { ((v944 * (v185 * v3394)) + (v941 * (v3417 * v3453))) } else { (if self.scalar_v930 { (v185 * (v3394 + (((v3394 * v3410) + v3417) * v3426))) } else { v304 }) });
        let v3473: f64 = (if self.scalar_v940 { (v944 * (v185 * v3395)) } else { (if self.scalar_v930 { (v185 * (v3395 + ((v3395 * v3410) * v3426))) } else { v304 }) });
        let v3474: f64 = (if self.scalar_v940 { ((v944 * (v185 * v3396)) + (v941 * (v3418 * v3453))) } else { (if self.scalar_v930 { (v185 * (v3396 + (((v3396 * v3410) + v3418) * v3426))) } else { v304 }) });
        let v3475: f64 = (if self.scalar_v940 { ((v944 * (v185 * v3397)) + (v941 * (v3419 * v3453))) } else { (if self.scalar_v930 { (v185 * (v3397 + (((v3397 * v3410) + v3419) * v3426))) } else { v304 }) });
        let v3479: f64 = (v946 * v946);
        let v3480: f64 = (((v946 * v3358) - (v911 * v3471)) / v3479);
        let v3484: f64 = (((v946 * v3359) - (v911 * v3472)) / v3479);
        let v3487: f64 = ((-(v911 * v3473)) / v3479);
        let v3491: f64 = (((v946 * v3360) - (v911 * v3474)) / v3479);
        let v3494: f64 = ((-(v911 * v3475)) / v3479);
        let v3498: f64 = (((v946 * v3337) - (v905 * v3471)) / v3479);
        let v3501: f64 = ((-(v905 * v3472)) / v3479);
        let v3504: f64 = ((-(v905 * v3473)) / v3479);
        let v3508: f64 = (((v946 * v3338) - (v905 * v3474)) / v3479);
        let v3512: f64 = (((v946 * v3339) - (v905 * v3475)) / v3479);
        let v3516: f64 = (v950 * v950);
        let v3519: f64 = (v360 / v950);
        let v3520: f64 = (if self.scalar_v949 { ((-(v349 * self.scalar_v3513)) / v3516) } else { v3346 });
        let v3521: f64 = (if self.scalar_v949 { v304 } else { v3347 });
        let v3522: f64 = (if self.scalar_v949 { (v51 / v950) } else { v304 });
        let v3523: f64 = (if self.scalar_v949 { v304 } else { v3348 });
        let v3524: f64 = (if self.scalar_v949 { v3519 } else { v304 });
        let v3525: f64 = { let limexp_arg = v952; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v3531: f64 = (if self.scalar_v949 { (v3520 * v3525) } else { v3350 });
        let v3532: f64 = (if self.scalar_v949 { (v3521 * v3525) } else { v3351 });
        let v3533: f64 = (if self.scalar_v949 { (v3522 * v3525) } else { v304 });
        let v3534: f64 = (if self.scalar_v949 { (v3523 * v3525) } else { v3352 });
        let v3535: f64 = (if self.scalar_v949 { (v3524 * v3525) } else { v304 });
        let v3539: f64 = (if self.scalar_v949 { ((-(v345 * self.scalar_v3513)) / v3516) } else { v304 });
        let v3540: f64 = { let limexp_arg = v956; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v3544: f64 = (if self.scalar_v949 { (v3539 * v3540) } else { v304 });
        let v3545: f64 = (if self.scalar_v949 { (v3524 * v3540) } else { v304 });
        let v3546: f64 = (if self.scalar_v949 { (v3522 * v3540) } else { v304 });
        let v3565: f64 = (if self.scalar_v949 { ((v964 * v1438) + (v84 * ((self.scalar_v959 * v3531) + (self.scalar_v961 * v3544)))) } else { v304 });
        let v3566: f64 = (if self.scalar_v949 { (v84 * ((self.scalar_v959 * v3532) + (self.scalar_v961 * v3545))) } else { v304 });
        let v3567: f64 = (if self.scalar_v949 { (v84 * (self.scalar_v959 * v3533)) } else { v304 });
        let v3568: f64 = (if self.scalar_v949 { (v84 * ((self.scalar_v959 * v3534) + (self.scalar_v961 * v3546))) } else { v304 });
        let v3569: f64 = (if self.scalar_v949 { (v84 * (self.scalar_v959 * v3535)) } else { v304 });
        let v3585: f64 = (v182 * v971);
        let v3596: f64 = (if self.scalar_v949 { (v185 * ((v209 * (if self.scalar_v949 { (self.scalar_v322 * v3565) } else { v304 })) / v3585)) } else { v304 });
        let v3597: f64 = (if self.scalar_v949 { (v185 * ((v209 * (if self.scalar_v949 { (self.scalar_v322 * v3566) } else { v304 })) / v3585)) } else { v304 });
        let v3598: f64 = (if self.scalar_v949 { (v185 * ((v209 * (if self.scalar_v949 { (self.scalar_v322 * v3567) } else { v304 })) / v3585)) } else { v304 });
        let v3599: f64 = (if self.scalar_v949 { (v185 * ((v209 * (if self.scalar_v949 { (self.scalar_v322 * v3568) } else { v304 })) / v3585)) } else { v304 });
        let v3600: f64 = (if self.scalar_v949 { (v185 * ((v209 * (if self.scalar_v949 { (self.scalar_v322 * v3569) } else { v304 })) / v3585)) } else { v304 });
        let v3604: f64 = (if self.scalar_v949 { ((-(v825 * self.scalar_v3513)) / v3516) } else { v3520 });
        let v3605: f64 = (if self.scalar_v949 { v304 } else { v3521 });
        let v3606: f64 = (if self.scalar_v949 { v304 } else { v3522 });
        let v3607: f64 = (if self.scalar_v949 { v304 } else { v3523 });
        let v3608: f64 = (if self.scalar_v949 { v3519 } else { v3524 });
        let v3609: f64 = { let limexp_arg = v976; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v3616: f64 = (if self.scalar_v949 { (v3604 * v3609) } else { v3531 });
        let v3617: f64 = (if self.scalar_v949 { (v3605 * v3609) } else { v3532 });
        let v3618: f64 = (if self.scalar_v949 { (v3606 * v3609) } else { v3533 });
        let v3619: f64 = (if self.scalar_v949 { (v3607 * v3609) } else { v3534 });
        let v3620: f64 = (if self.scalar_v949 { (v3608 * v3609) } else { v3535 });
        let v3621: f64 = (if self.scalar_v949 { (v3522 * v3609) } else { v304 });
        let v3645: f64 = (v974 * v974);
        let v3680: f64 = (if self.scalar_v985 { v304 } else { (if self.scalar_v949 { (((v974 * (v3565 - (if self.scalar_v949 { ((v979 * v1438) + (v84 * v3616)) } else { v304 }))) - (v982 * v3596)) / v3645) } else { v304 }) });
        let v3681: f64 = (if self.scalar_v985 { v304 } else { (if self.scalar_v949 { (((v974 * (v3566 - (if self.scalar_v949 { (v84 * v3617) } else { v304 }))) - (v982 * v3597)) / v3645) } else { v304 }) });
        let v3682: f64 = (if self.scalar_v985 { v304 } else { (if self.scalar_v949 { (((v974 * (v3567 - (if self.scalar_v949 { (v84 * v3618) } else { v304 }))) - (v982 * v3598)) / v3645) } else { v304 }) });
        let v3683: f64 = (if self.scalar_v985 { v304 } else { (if self.scalar_v949 { (((v974 * (v3568 - (if self.scalar_v949 { (v84 * v3619) } else { v304 }))) - (v982 * v3599)) / v3645) } else { v304 }) });
        let v3684: f64 = (if self.scalar_v985 { v304 } else { (if self.scalar_v949 { (((v974 * (v3569 - (if self.scalar_v949 { (v84 * v3620) } else { v304 }))) - (v982 * v3600)) / v3645) } else { v304 }) });
        let v3685: f64 = (if self.scalar_v985 { v304 } else { (if self.scalar_v949 { ((-(if self.scalar_v949 { (v84 * v3621) } else { v304 })) / v974) } else { v304 }) });
        let v3689: f64 = (v991 * v991);
        let v3690: f64 = ((-(v341 * self.scalar_v3686)) / v3689);
        let v3691: f64 = (v51 / v991);
        let v3692: f64 = (v360 / v991);
        let v3693: f64 = (if self.scalar_v990 { v3690 } else { v3604 });
        let v3694: f64 = (if self.scalar_v990 { v304 } else { v3605 });
        let v3695: f64 = (if self.scalar_v990 { v304 } else { v3606 });
        let v3696: f64 = (if self.scalar_v990 { v3691 } else { v3607 });
        let v3697: f64 = (if self.scalar_v990 { v3692 } else { v304 });
        let v3698: f64 = (if self.scalar_v990 { v304 } else { v3608 });
        let v3699: f64 = (if self.scalar_v990 { v304 } else { v3522 });
        let v3700: f64 = { let limexp_arg = v993; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v3708: f64 = (if self.scalar_v990 { (v3693 * v3700) } else { v3616 });
        let v3709: f64 = (if self.scalar_v990 { (v3694 * v3700) } else { v3617 });
        let v3710: f64 = (if self.scalar_v990 { (v3695 * v3700) } else { v3618 });
        let v3711: f64 = (if self.scalar_v990 { (v3696 * v3700) } else { v3619 });
        let v3712: f64 = (if self.scalar_v990 { (v3697 * v3700) } else { v304 });
        let v3713: f64 = (if self.scalar_v990 { (v3698 * v3700) } else { v3620 });
        let v3714: f64 = (if self.scalar_v990 { (v3699 * v3700) } else { v3621 });
        let v3718: f64 = (v996 * v996);
        let v3719: f64 = ((-(v341 * self.scalar_v3715)) / v3718);
        let v3720: f64 = (v51 / v996);
        let v3721: f64 = (v360 / v996);
        let v3722: f64 = (if self.scalar_v990 { v3719 } else { v304 });
        let v3723: f64 = (if self.scalar_v990 { v3720 } else { v304 });
        let v3724: f64 = (if self.scalar_v990 { v3721 } else { v304 });
        let v3725: f64 = { let limexp_arg = v998; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v3729: f64 = (if self.scalar_v990 { (v3722 * v3725) } else { v304 });
        let v3730: f64 = (if self.scalar_v990 { (v3723 * v3725) } else { v304 });
        let v3731: f64 = (if self.scalar_v990 { (v3724 * v3725) } else { v304 });
        let v3734: f64 = ((v1705 - (v1003 * v1704)) / v1708);
        let v3735: f64 = (v360 / v300);
        let v3736: f64 = (v51 / v300);
        let v3737: f64 = (if self.scalar_v1002 { v3734 } else { v3539 });
        let v3738: f64 = (if self.scalar_v1002 { v304 } else { v3524 });
        let v3739: f64 = (if self.scalar_v1002 { v3735 } else { v3522 });
        let v3740: f64 = (if self.scalar_v1002 { v3736 } else { v304 });
        let v3741: f64 = { let limexp_arg = v1005; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v3746: f64 = (if self.scalar_v1002 { (v3737 * v3741) } else { v3544 });
        let v3747: f64 = (if self.scalar_v1002 { (v3738 * v3741) } else { v3545 });
        let v3748: f64 = (if self.scalar_v1002 { (v3739 * v3741) } else { v3546 });
        let v3749: f64 = (if self.scalar_v1002 { (v3740 * v3741) } else { v304 });
        let v3753: f64 = (v97 * v3709);
        let v3754: f64 = (v97 * v3710);
        let v3757: f64 = (v97 * v3713);
        let v3758: f64 = (v97 * v3714);
        let v3764: f64 = (((v1008 * v1456) + (v97 * v3708)) + ((v1010 * v1474) + (v110 * v3729)));
        let v3765: f64 = ((v97 * v3711) + (v110 * v3730));
        let v3766: f64 = ((v97 * v3712) + (v110 * v3731));
        let v3799: f64 = ((-(v343 * self.scalar_v3686)) / v3689);
        let v3800: f64 = (if self.scalar_v1023 { v3799 } else { v3693 });
        let v3801: f64 = (if self.scalar_v1023 { v304 } else { v3694 });
        let v3802: f64 = (if self.scalar_v1023 { v3691 } else { v3695 });
        let v3803: f64 = (if self.scalar_v1023 { v304 } else { v3696 });
        let v3804: f64 = (if self.scalar_v1023 { v3692 } else { v3697 });
        let v3805: f64 = (if self.scalar_v1023 { v304 } else { v3698 });
        let v3806: f64 = (if self.scalar_v1023 { v304 } else { v3699 });
        let v3807: f64 = { let limexp_arg = v1026; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v3815: f64 = (if self.scalar_v1023 { (v3800 * v3807) } else { v3708 });
        let v3816: f64 = (if self.scalar_v1023 { (v3801 * v3807) } else { v3709 });
        let v3817: f64 = (if self.scalar_v1023 { (v3802 * v3807) } else { v3710 });
        let v3818: f64 = (if self.scalar_v1023 { (v3803 * v3807) } else { v3711 });
        let v3819: f64 = (if self.scalar_v1023 { (v3804 * v3807) } else { v3712 });
        let v3820: f64 = (if self.scalar_v1023 { (v3805 * v3807) } else { v3713 });
        let v3821: f64 = (if self.scalar_v1023 { (v3806 * v3807) } else { v3714 });
        let v3824: f64 = ((-(v343 * self.scalar_v3715)) / v3718);
        let v3825: f64 = (if self.scalar_v1023 { v3824 } else { v3722 });
        let v3826: f64 = (if self.scalar_v1023 { v3720 } else { v304 });
        let v3827: f64 = (if self.scalar_v1023 { v304 } else { v3723 });
        let v3828: f64 = (if self.scalar_v1023 { v3721 } else { v3724 });
        let v3829: f64 = { let limexp_arg = v1030; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v3834: f64 = (if self.scalar_v1023 { (v3825 * v3829) } else { v3729 });
        let v3835: f64 = (if self.scalar_v1023 { (v3826 * v3829) } else { v304 });
        let v3836: f64 = (if self.scalar_v1023 { (v3827 * v3829) } else { v3730 });
        let v3837: f64 = (if self.scalar_v1023 { (v3828 * v3829) } else { v3731 });
        let v3840: f64 = ((v1705 - (v1034 * v1704)) / v1708);
        let v3841: f64 = (if self.scalar_v1033 { v3840 } else { v3737 });
        let v3842: f64 = (if self.scalar_v1033 { v304 } else { v3738 });
        let v3843: f64 = (if self.scalar_v1033 { v3735 } else { v304 });
        let v3844: f64 = (if self.scalar_v1033 { v304 } else { v3739 });
        let v3845: f64 = (if self.scalar_v1033 { v3736 } else { v3740 });
        let v3846: f64 = { let limexp_arg = v1036; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v3852: f64 = (if self.scalar_v1033 { (v3841 * v3846) } else { v3746 });
        let v3853: f64 = (if self.scalar_v1033 { (v3842 * v3846) } else { v3747 });
        let v3854: f64 = (if self.scalar_v1033 { (v3843 * v3846) } else { v304 });
        let v3855: f64 = (if self.scalar_v1033 { (v3844 * v3846) } else { v3748 });
        let v3856: f64 = (if self.scalar_v1033 { (v3845 * v3846) } else { v3749 });
        let v3860: f64 = (v97 * v3816);
        let v3864: f64 = (v97 * v3820);
        let v3865: f64 = (v97 * v3821);
        let v3872: f64 = (((v1039 * v1456) + (v97 * v3815)) + ((v1041 * v1474) + (v110 * v3834)));
        let v3873: f64 = ((v97 * v3817) + (v110 * v3835));
        let v3874: f64 = ((v97 * v3818) + (v110 * v3836));
        let v3875: f64 = ((v97 * v3819) + (v110 * v3837));
        let v3901: f64 = (if self.scalar_v1051 { v3690 } else { v3800 });
        let v3902: f64 = (if self.scalar_v1051 { v304 } else { v3801 });
        let v3903: f64 = (if self.scalar_v1051 { v304 } else { v3802 });
        let v3904: f64 = (if self.scalar_v1051 { v3691 } else { v3803 });
        let v3905: f64 = (if self.scalar_v1051 { v3692 } else { v3804 });
        let v3906: f64 = (if self.scalar_v1051 { v304 } else { v3805 });
        let v3907: f64 = (if self.scalar_v1051 { v304 } else { v3806 });
        let v3908: f64 = { let limexp_arg = v1052; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v3916: f64 = (if self.scalar_v1051 { (v3901 * v3908) } else { v3815 });
        let v3917: f64 = (if self.scalar_v1051 { (v3902 * v3908) } else { v3816 });
        let v3918: f64 = (if self.scalar_v1051 { (v3903 * v3908) } else { v3817 });
        let v3919: f64 = (if self.scalar_v1051 { (v3904 * v3908) } else { v3818 });
        let v3920: f64 = (if self.scalar_v1051 { (v3905 * v3908) } else { v3819 });
        let v3921: f64 = (if self.scalar_v1051 { (v3906 * v3908) } else { v3820 });
        let v3922: f64 = (if self.scalar_v1051 { (v3907 * v3908) } else { v3821 });
        let v3923: f64 = (if self.scalar_v1051 { v3719 } else { v3825 });
        let v3924: f64 = (if self.scalar_v1051 { v304 } else { v3826 });
        let v3925: f64 = (if self.scalar_v1051 { v3720 } else { v3827 });
        let v3926: f64 = (if self.scalar_v1051 { v3721 } else { v3828 });
        let v3927: f64 = { let limexp_arg = v1055; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v3932: f64 = (if self.scalar_v1051 { (v3923 * v3927) } else { v3834 });
        let v3933: f64 = (if self.scalar_v1051 { (v3924 * v3927) } else { v3835 });
        let v3934: f64 = (if self.scalar_v1051 { (v3925 * v3927) } else { v3836 });
        let v3935: f64 = (if self.scalar_v1051 { (v3926 * v3927) } else { v3837 });
        let v3936: f64 = (if self.scalar_v1058 { v3734 } else { v3841 });
        let v3937: f64 = (if self.scalar_v1058 { v304 } else { v3842 });
        let v3938: f64 = (if self.scalar_v1058 { v304 } else { v3843 });
        let v3939: f64 = (if self.scalar_v1058 { v3735 } else { v3844 });
        let v3940: f64 = (if self.scalar_v1058 { v3736 } else { v3845 });
        let v3941: f64 = { let limexp_arg = v1059; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v3947: f64 = (if self.scalar_v1058 { (v3936 * v3941) } else { v3852 });
        let v3948: f64 = (if self.scalar_v1058 { (v3937 * v3941) } else { v3853 });
        let v3949: f64 = (if self.scalar_v1058 { (v3938 * v3941) } else { v3854 });
        let v3950: f64 = (if self.scalar_v1058 { (v3939 * v3941) } else { v3855 });
        let v3951: f64 = (if self.scalar_v1058 { (v3940 * v3941) } else { v3856 });
        let v3955: f64 = (v97 * v3917);
        let v3967: f64 = (((v1062 * v1456) + (v97 * v3916)) + ((v1064 * v1474) + (v110 * v3932)));
        let v3968: f64 = ((v97 * v3918) + (v110 * v3933));
        let v3969: f64 = ((v97 * v3919) + (v110 * v3934));
        let v3970: f64 = ((v97 * v3920) + (v110 * v3935));
        let v3987: f64 = (self.scalar_v989 * (v97 * v3921));
        let v3988: f64 = (self.scalar_v989 * (v97 * v3922));
        let v3989: f64 = (if self.scalar_v1058 { (self.scalar_v989 * (v3967 - (self.scalar_v1013 * (v3947 - v1710)))) } else { (if self.scalar_v1023 { v304 } else { (if self.scalar_v1019 { v3764 } else { (if self.scalar_v1002 { (v3764 - (self.scalar_v1013 * (v3746 - v1710))) } else { v304 }) }) }) });
        let v4001: f64 = (if self.scalar_v1072 { (self.scalar_v989 * v3967) } else { v3989 });
        let v4002: f64 = (if self.scalar_v1072 { (self.scalar_v989 * v3955) } else { (if self.scalar_v1058 { (self.scalar_v989 * (v3955 - (self.scalar_v1013 * v3948))) } else { (if self.scalar_v1023 { v304 } else { (if self.scalar_v1019 { v3753 } else { (if self.scalar_v1002 { (v3753 - (self.scalar_v1013 * v3747)) } else { v304 }) }) }) }) });
        let v4003: f64 = (if self.scalar_v1072 { (self.scalar_v989 * v3968) } else { (if self.scalar_v1058 { (self.scalar_v989 * (v3968 - (self.scalar_v1013 * v3949))) } else { (if self.scalar_v1023 { v304 } else { (if self.scalar_v1019 { v3754 } else { (if self.scalar_v1002 { v3754 } else { v304 }) }) }) }) });
        let v4004: f64 = (if self.scalar_v1072 { (self.scalar_v989 * v3969) } else { (if self.scalar_v1058 { (self.scalar_v989 * (v3969 - (self.scalar_v1013 * v3950))) } else { (if self.scalar_v1023 { v304 } else { (if self.scalar_v1019 { v3765 } else { (if self.scalar_v1002 { (v3765 - (self.scalar_v1013 * v3748)) } else { v304 }) }) }) }) });
        let v4005: f64 = (if self.scalar_v1072 { (self.scalar_v989 * v3970) } else { (if self.scalar_v1058 { (self.scalar_v989 * (v3970 - (self.scalar_v1013 * v3951))) } else { (if self.scalar_v1023 { v304 } else { (if self.scalar_v1019 { v3766 } else { (if self.scalar_v1002 { (v3766 - (self.scalar_v1013 * v3749)) } else { v304 }) }) }) }) });
        let v4006: f64 = (if self.scalar_v1072 { v3987 } else { (if self.scalar_v1058 { v3987 } else { (if self.scalar_v1023 { v304 } else { (if self.scalar_v1019 { v3757 } else { (if self.scalar_v1002 { v3757 } else { v304 }) }) }) }) });
        let v4007: f64 = (if self.scalar_v1072 { v3988 } else { (if self.scalar_v1058 { v3988 } else { (if self.scalar_v1023 { v304 } else { (if self.scalar_v1019 { v3758 } else { (if self.scalar_v1002 { v3758 } else { v304 }) }) }) }) });
        let v4015: f64 = { let limexp_arg = v1075; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v4034: f64 = { let limexp_arg = v1078; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v4048: f64 = { let limexp_arg = v1081; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v4062: f64 = (v97 * (if self.scalar_v1051 { ((if self.scalar_v1051 { v304 } else { v3902 }) * v4015) } else { v3917 }));
        let v4074: f64 = (((v1085 * v1456) + (v97 * (if self.scalar_v1051 { ((if self.scalar_v1051 { v3799 } else { v3901 }) * v4015) } else { v3916 }))) + ((v1087 * v1474) + (v110 * (if self.scalar_v1051 { ((if self.scalar_v1051 { v3824 } else { v3923 }) * v4034) } else { v3932 }))));
        let v4075: f64 = ((v97 * (if self.scalar_v1051 { ((if self.scalar_v1051 { v3691 } else { v3903 }) * v4015) } else { v3918 })) + (v110 * (if self.scalar_v1051 { ((if self.scalar_v1051 { v3720 } else { v3924 }) * v4034) } else { v3933 })));
        let v4076: f64 = ((v97 * (if self.scalar_v1051 { ((if self.scalar_v1051 { v304 } else { v3904 }) * v4015) } else { v3919 })) + (v110 * (if self.scalar_v1051 { ((if self.scalar_v1051 { v304 } else { v3925 }) * v4034) } else { v3934 })));
        let v4077: f64 = ((v97 * (if self.scalar_v1051 { ((if self.scalar_v1051 { v3692 } else { v3905 }) * v4015) } else { v3920 })) + (v110 * (if self.scalar_v1051 { ((if self.scalar_v1051 { v3721 } else { v3926 }) * v4034) } else { v3935 })));
        let v4094: f64 = (self.scalar_v1084 * (v97 * (if self.scalar_v1051 { ((if self.scalar_v1051 { v304 } else { v3906 }) * v4015) } else { v3921 })));
        let v4095: f64 = (self.scalar_v1084 * (v97 * (if self.scalar_v1051 { ((if self.scalar_v1051 { v304 } else { v3907 }) * v4015) } else { v3922 })));
        let v4096: f64 = (if self.scalar_v1058 { (self.scalar_v1084 * (v4074 - (self.scalar_v1013 * ((if self.scalar_v1058 { ((if self.scalar_v1058 { v3840 } else { v3936 }) * v4048) } else { v3947 }) - v1710)))) } else { (if self.scalar_v1048 { v3872 } else { (if self.scalar_v1033 { (v3872 - (self.scalar_v1013 * (v3852 - v1710))) } else { v304 }) }) });
        let v4097: f64 = (if self.scalar_v1058 { (self.scalar_v1084 * (v4062 - (self.scalar_v1013 * (if self.scalar_v1058 { ((if self.scalar_v1058 { v304 } else { v3937 }) * v4048) } else { v3948 })))) } else { (if self.scalar_v1048 { v3860 } else { (if self.scalar_v1033 { (v3860 - (self.scalar_v1013 * v3853)) } else { v304 }) }) });
        let v4098: f64 = (if self.scalar_v1058 { (self.scalar_v1084 * (v4075 - (self.scalar_v1013 * (if self.scalar_v1058 { ((if self.scalar_v1058 { v3735 } else { v3938 }) * v4048) } else { v3949 })))) } else { (if self.scalar_v1048 { v3873 } else { (if self.scalar_v1033 { (v3873 - (self.scalar_v1013 * v3854)) } else { v304 }) }) });
        let v4099: f64 = (if self.scalar_v1058 { (self.scalar_v1084 * (v4076 - (self.scalar_v1013 * (if self.scalar_v1058 { ((if self.scalar_v1058 { v304 } else { v3939 }) * v4048) } else { v3950 })))) } else { (if self.scalar_v1048 { v3874 } else { (if self.scalar_v1033 { (v3874 - (self.scalar_v1013 * v3855)) } else { v304 }) }) });
        let v4100: f64 = (if self.scalar_v1058 { (self.scalar_v1084 * (v4077 - (self.scalar_v1013 * (if self.scalar_v1058 { ((if self.scalar_v1058 { v3736 } else { v3940 }) * v4048) } else { v3951 })))) } else { (if self.scalar_v1048 { v3875 } else { (if self.scalar_v1033 { (v3875 - (self.scalar_v1013 * v3856)) } else { v304 }) }) });
        let v4108: f64 = (if self.scalar_v1072 { (self.scalar_v1084 * v4074) } else { v4096 });
        let v4109: f64 = (if self.scalar_v1072 { (self.scalar_v1084 * v4062) } else { v4097 });
        let v4110: f64 = (if self.scalar_v1072 { (self.scalar_v1084 * v4075) } else { v4098 });
        let v4111: f64 = (if self.scalar_v1072 { (self.scalar_v1084 * v4076) } else { v4099 });
        let v4112: f64 = (if self.scalar_v1072 { (self.scalar_v1084 * v4077) } else { v4100 });
        let v4113: f64 = (if self.scalar_v1072 { v4094 } else { (if self.scalar_v1058 { v4094 } else { (if self.scalar_v1048 { v3864 } else { (if self.scalar_v1033 { v3864 } else { v304 }) }) }) });
        let v4114: f64 = (if self.scalar_v1072 { v4095 } else { (if self.scalar_v1058 { v4095 } else { (if self.scalar_v1048 { v3865 } else { (if self.scalar_v1033 { v3865 } else { v304 }) }) }) });
        let v4118: f64 = (v1097 * v1097);
        let v4119: f64 = ((-(v345 * self.scalar_v4115)) / v4118);
        let v4120: f64 = (v360 / v1097);
        let v4121: f64 = (v51 / v1097);
        let v4122: f64 = { let limexp_arg = v1098; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v4123: f64 = (v4119 * v4122);
        let v4124: f64 = (v4120 * v4122);
        let v4125: f64 = (v4121 * v4122);
        let v4129: f64 = (v1100 * v1100);
        let v4130: f64 = ((-(v345 * self.scalar_v4126)) / v4129);
        let v4131: f64 = (v360 / v1100);
        let v4132: f64 = (v51 / v1100);
        let v4133: f64 = { let limexp_arg = v1101; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v4134: f64 = (v4130 * v4133);
        let v4135: f64 = (v4131 * v4133);
        let v4136: f64 = (v4132 * v4133);
        let v4147: f64 = (((v1103 * (self.scalar_v111 * v1487)) + (v121 * v4123)) + ((v1105 * (self.scalar_v122 * v1501)) + (v132 * v4134)));
        let v4148: f64 = ((v121 * v4124) + (v132 * v4135));
        let v4149: f64 = ((v121 * v4125) + (v132 * v4136));
        let v4158: f64 = { let limexp_arg = v1112; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v4172: f64 = (if self.scalar_v1110 { ((-(v349 * self.scalar_v4126)) / v4129) } else { v4130 });
        let v4173: f64 = (if self.scalar_v1110 { v304 } else { v4131 });
        let v4174: f64 = (if self.scalar_v1110 { v4132 } else { v304 });
        let v4175: f64 = (if self.scalar_v1110 { v304 } else { v4132 });
        let v4176: f64 = (if self.scalar_v1110 { v4131 } else { v304 });
        let v4177: f64 = { let limexp_arg = v1116; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v4183: f64 = (if self.scalar_v1110 { (v4172 * v4177) } else { v4134 });
        let v4184: f64 = (if self.scalar_v1110 { (v4173 * v4177) } else { v4135 });
        let v4185: f64 = (if self.scalar_v1110 { (v4174 * v4177) } else { v304 });
        let v4186: f64 = (if self.scalar_v1110 { (v4175 * v4177) } else { v4136 });
        let v4187: f64 = (if self.scalar_v1110 { (v4176 * v4177) } else { v304 });
        let v4202: f64 = (((v1119 * (self.scalar_v133 * v1487)) + (v134 * (if self.scalar_v1110 { ((if self.scalar_v1110 { ((-(v349 * self.scalar_v4115)) / v4118) } else { v4119 }) * v4158) } else { v4123 }))) + ((v1121 * (self.scalar_v135 * v1501)) + (v136 * v4183)));
        let v4212: f64 = (if self.scalar_v1125 { v304 } else { (if self.scalar_v1110 { v4202 } else { v304 }) });
        let v4213: f64 = (if self.scalar_v1125 { v304 } else { (if self.scalar_v1110 { ((v134 * (if self.scalar_v1110 { ((if self.scalar_v1110 { v304 } else { v4120 }) * v4158) } else { v4124 })) + (v136 * v4184)) } else { v304 }) });
        let v4214: f64 = (if self.scalar_v1125 { v304 } else { (if self.scalar_v1110 { ((v134 * (if self.scalar_v1110 { ((if self.scalar_v1110 { v4121 } else { v304 }) * v4158) } else { v304 })) + (v136 * v4185)) } else { v304 }) });
        let v4215: f64 = (if self.scalar_v1125 { v304 } else { (if self.scalar_v1110 { ((v134 * (if self.scalar_v1110 { ((if self.scalar_v1110 { v304 } else { v4121 }) * v4158) } else { v4125 })) + (v136 * v4186)) } else { v304 }) });
        let v4216: f64 = (if self.scalar_v1125 { v304 } else { (if self.scalar_v1110 { ((v134 * (if self.scalar_v1110 { ((if self.scalar_v1110 { v4120 } else { v304 }) * v4158) } else { v304 })) + (v136 * v4187)) } else { v304 }) });
        let v4217: f64 = (v1129 * v1626);
        let v4220: f64 = (-v1129);
        let v4222: f64 = (v182 * v1133);
        let v4232: f64 = (if self.scalar_v1128 { (v185 * (v1626 + ((v4217 + v4217) / v4222))) } else { v3229 });
        let v4233: f64 = (if self.scalar_v1128 { (v185 * (v51 + ((v1129 + v1129) / v4222))) } else { v3230 });
        let v4234: f64 = (if self.scalar_v1128 { v304 } else { v3231 });
        let v4235: f64 = (if self.scalar_v1128 { (v185 * (v360 + ((v4220 + v4220) / v4222))) } else { v3232 });
        let v4236: f64 = (if self.scalar_v1128 { v304 } else { v3233 });
        let v4237: f64 = (if self.scalar_v1128 { v304 } else { v3234 });
        let v4238: f64 = (if self.scalar_v1128 { v304 } else { v3235 });
        let v4249: f64 = (self.scalar_v1139 * f64::powf(v1136, self.scalar_v4247));
        let v4266: f64 = { let limexp_arg = v1141; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v4302: f64 = (v3498 - v3480);
        let v4303: f64 = (v3501 - v3484);
        let v4304: f64 = (v3504 - v3487);
        let v4305: f64 = (v3508 - v3491);
        let v4306: f64 = (v3512 - v3494);
        let v4312: f64 = ((v1146 * (if self.scalar_v1128 { ((v1142 * (self.scalar_v1127 * v4232)) + (v1137 * (((v1140 * self.scalar_v4246) + (v1138 * (v4232 * v4249))) * v4266))) } else { v304 })) + (v1144 * (v4302 - v4147)));
        let v4315: f64 = ((v1146 * (if self.scalar_v1128 { ((v1142 * (self.scalar_v1127 * v4233)) + (v1137 * ((v1138 * (v4233 * v4249)) * v4266))) } else { v304 })) + (v1144 * (v4303 - v4148)));
        let v4321: f64 = ((v1146 * (if self.scalar_v1128 { ((v1142 * (self.scalar_v1127 * v4235)) + (v1137 * ((v1138 * (v4235 * v4249)) * v4266))) } else { v304 })) + (v1144 * (v4305 - v4149)));
        let v4329: f64 = (if self.scalar_v1128 { ((v1146 * (if self.scalar_v1128 { ((v1142 * (self.scalar_v1127 * v4234)) + (v1137 * ((v1138 * (v4234 * v4249)) * v4266))) } else { v304 })) + (v1144 * v4304)) } else { v304 });
        let v4331: f64 = (if self.scalar_v1128 { ((v1146 * (if self.scalar_v1128 { ((v1142 * (self.scalar_v1127 * v4236)) + (v1137 * ((v1138 * (v4236 * v4249)) * v4266))) } else { v304 })) + (v1144 * v4306)) } else { v304 });
        let v4339: f64 = (if self.scalar_v1149 { v304 } else { (if self.scalar_v1128 { (v1146 * (if self.scalar_v1128 { ((v1142 * (self.scalar_v1127 * v4237)) + (v1137 * ((v1138 * (v4237 * v4249)) * v4266))) } else { v304 })) } else { v304 }) });
        let v4340: f64 = (if self.scalar_v1149 { v304 } else { (if self.scalar_v1128 { (v1146 * (if self.scalar_v1128 { ((v1142 * (self.scalar_v1127 * v4238)) + (v1137 * ((v1138 * (v4238 * v4249)) * v4266))) } else { v304 })) } else { v304 }) });
        let v4341: f64 = (v4147 - (if self.scalar_v1149 { v304 } else { (if self.scalar_v1128 { v4312 } else { v304 }) }));
        let v4342: f64 = (v4148 - (if self.scalar_v1149 { v304 } else { (if self.scalar_v1128 { v4315 } else { v304 }) }));
        let v4343: f64 = (-(if self.scalar_v1149 { v304 } else { v4329 }));
        let v4344: f64 = (v4149 - (if self.scalar_v1149 { v304 } else { (if self.scalar_v1128 { v4321 } else { v304 }) }));
        let v4345: f64 = (-(if self.scalar_v1149 { v304 } else { v4331 }));
        let v4346: f64 = (-v4339);
        let v4347: f64 = (-v4340);
        let v4357: f64 = (if self.scalar_v1157 { v304 } else { (if self.scalar_v1152 { (v51 / v21) } else { v304 }) });
        let v4358: f64 = (if self.scalar_v1157 { v304 } else { (if self.scalar_v1152 { ((-(v1154 * (self.scalar_v18 * (self.scalar_v1346 * (self.scalar_v19 * f64::powf(v12, self.scalar_v1352)))))) / (v21 * v21)) } else { v304 }) });
        let v4359: f64 = (if self.scalar_v1157 { v304 } else { (if self.scalar_v1152 { (v360 / v21) } else { v304 }) });
        let v4362: f64 = ((-(v345 * v1345)) / v1396);
        let v4363: f64 = (v360 / v11);
        let v4364: f64 = (v51 / v11);
        let v4365: f64 = { let limexp_arg = v1159; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v4366: f64 = (v4362 * v4365);
        let v4367: f64 = (v4363 * v4365);
        let v4368: f64 = (v4364 * v4365);
        let v4372: f64 = { let limexp_arg = v1161; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v4381: f64 = (v182 * v1165);
        let v4382: f64 = (((v1160 * v1695) + (v294 * v4366)) / v4381);
        let v4383: f64 = ((v294 * v4367) / v4381);
        let v4384: f64 = ((v294 * v4368) / v4381);
        let v4390: f64 = (v182 * v1168);
        let v4391: f64 = (((v1162 * v1695) + (v294 * (((-(v347 * v1345)) / v1396) * v4372))) / v4390);
        let v4392: f64 = ((v294 * (v4363 * v4372)) / v4390);
        let v4393: f64 = ((v294 * (v4364 * v4372)) / v4390);
        let v4397: f64 = (v1171 * v1171);
        let v4430: f64 = (v25 * ((v1177 * v1345) + (v11 * ((v4382 - v4391) - ((if self.scalar_v1169 { (((v1171 * v4382) - (v1170 * v4391)) / v4397) } else { v304 }) / v1173)))));
        let v4438: f64 = (if self.scalar_v1169 { ((v4430 - (v1179 * v1361)) / (v25 * v25)) } else { v304 });
        let v4439: f64 = (if self.scalar_v1169 { ((v51 + (v11 * ((-v4392) - ((if self.scalar_v1169 { ((-(v1170 * v4392)) / v4397) } else { v304 }) / v1173)))) / v25) } else { v304 });
        let v4440: f64 = (if self.scalar_v1169 { ((v360 + (v11 * (v4383 - ((if self.scalar_v1169 { (v4383 / v1171) } else { v304 }) / v1173)))) / v25) } else { v304 });
        let v4441: f64 = (if self.scalar_v1169 { ((v11 * ((v4384 - v4393) - ((if self.scalar_v1169 { (((v1171 * v4384) - (v1170 * v4393)) / v4397) } else { v304 }) / v1173))) / v25) } else { v304 });
        let v4454: f64 = (-v1174);
        let v4456: f64 = (v182 * v1188);
        let v4464: f64 = ((v1190 * ((v1182 * v4438) + (v1181 * ((v325 * v1361) + (v25 * v1718))))) - (v1183 * (v1188 * (self.scalar_v329 * (v185 * v1718)))));
        let v4465: f64 = (v1190 * v1190);
        let v4480: f64 = (v1192 * (if self.scalar_v1169 { (v4464 / v4465) } else { v304 }));
        let v4482: f64 = (v1192 * (if self.scalar_v1169 { (((v1190 * (v1182 * v4439)) - (v1183 * (v1185 * ((v1174 + v1174) / v4456)))) / v4465) } else { v304 }));
        let v4484: f64 = (v1192 * (if self.scalar_v1169 { (((v1190 * (v1182 * v4440)) - (v1183 * (v1185 * ((v4454 + v4454) / v4456)))) / v4465) } else { v304 }));
        let v4486: f64 = (v1192 * (if self.scalar_v1169 { ((v1182 * v4441) / v1190) } else { v304 }));
        let v4488: f64 = (v182 * v1195);
        let v4496: f64 = (v1195 * v1195);
        let v4514: f64 = (if self.scalar_v1198 { v304 } else { (if self.scalar_v1169 { (((v1195 * v4438) - (v1181 * ((v4480 + v4480) / v4488))) / v4496) } else { v304 }) });
        let v4515: f64 = (if self.scalar_v1198 { v304 } else { (if self.scalar_v1169 { (((v1195 * v4439) - (v1181 * ((v4482 + v4482) / v4488))) / v4496) } else { v304 }) });
        let v4516: f64 = (if self.scalar_v1198 { v304 } else { (if self.scalar_v1169 { (((v1195 * v4440) - (v1181 * ((v4484 + v4484) / v4488))) / v4496) } else { v304 }) });
        let v4517: f64 = (if self.scalar_v1198 { v304 } else { (if self.scalar_v1169 { (((v1195 * v4441) - (v1181 * ((v4486 + v4486) / v4488))) / v4496) } else { v304 }) });
        let v4527: f64 = (if self.scalar_v1205 { v304 } else { (if self.scalar_v1200 { (v51 / v29) } else { v304 }) });
        let v4528: f64 = (if self.scalar_v1205 { v304 } else { (if self.scalar_v1200 { ((-(v1202 * (self.scalar_v26 * (self.scalar_v1346 * (self.scalar_v27 * f64::powf(v12, self.scalar_v1362)))))) / (v29 * v29)) } else { v304 }) });
        let v4529: f64 = (if self.scalar_v1205 { v304 } else { (if self.scalar_v1200 { (v360 / v29) } else { v304 }) });
        let v4552: f64 = (if self.scalar_v1212 { v304 } else { (if self.scalar_v1207 { (((v33 * (v1208 * v3471)) - (v1209 * (self.scalar_v30 * (self.scalar_v1346 * (self.scalar_v31 * f64::powf(v12, self.scalar_v1367)))))) / (v33 * v33)) } else { v304 }) });
        let v4553: f64 = (if self.scalar_v1212 { v304 } else { (if self.scalar_v1207 { ((v1208 * v3472) / v33) } else { v304 }) });
        let v4554: f64 = (if self.scalar_v1212 { v304 } else { (if self.scalar_v1207 { ((v946 + (v1208 * v3473)) / v33) } else { v304 }) });
        let v4555: f64 = (if self.scalar_v1212 { v304 } else { (if self.scalar_v1207 { (((v1208 * v3474) + (-v946)) / v33) } else { v304 }) });
        let v4556: f64 = (if self.scalar_v1212 { v304 } else { (if self.scalar_v1207 { ((v1208 * v3475) / v33) } else { v304 }) });
        let v4566: f64 = (if self.scalar_v1219 { v304 } else { (if self.scalar_v1214 { (v51 / v37) } else { v304 }) });
        let v4567: f64 = (if self.scalar_v1219 { v304 } else { (if self.scalar_v1214 { ((-(v1216 * (self.scalar_v34 * (self.scalar_v1346 * (self.scalar_v35 * f64::powf(v12, self.scalar_v1372)))))) / (v37 * v37)) } else { v304 }) });
        let v4568: f64 = (if self.scalar_v1219 { v304 } else { (if self.scalar_v1214 { (v360 / v37) } else { v304 }) });
        let v4586: f64 = (if self.scalar_v1221 { (((v45 * (v1222 * (if self.scalar_v985 { v304 } else { v3596 }))) - (v1223 * (self.scalar_v42 * (self.scalar_v1346 * (self.scalar_v43 * f64::powf(v12, self.scalar_v1382)))))) / (v45 * v45)) } else { v304 });
        let v4592: f64 = (if self.scalar_v1226 { v304 } else { v4586 });
        let v4593: f64 = (if self.scalar_v1226 { v304 } else { (if self.scalar_v1221 { ((-v987) / v45) } else { v304 }) });
        let v4594: f64 = (if self.scalar_v1226 { v304 } else { (if self.scalar_v1221 { ((v1222 * (if self.scalar_v985 { v304 } else { v3597 })) / v45) } else { v304 }) });
        let v4595: f64 = (if self.scalar_v1226 { v304 } else { (if self.scalar_v1221 { ((v1222 * (if self.scalar_v985 { v304 } else { v3598 })) / v45) } else { v304 }) });
        let v4596: f64 = (if self.scalar_v1226 { v304 } else { (if self.scalar_v1221 { ((v1222 * (if self.scalar_v985 { v304 } else { v3599 })) / v45) } else { v304 }) });
        let v4597: f64 = (if self.scalar_v1226 { v304 } else { (if self.scalar_v1221 { ((v987 + (v1222 * (if self.scalar_v985 { v304 } else { v3600 }))) / v45) } else { v304 }) });
        let v4610: f64 = { let limexp_arg = v1233; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v4634: f64 = { let limexp_arg = v1238; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v4662: f64 = (((v1241 * v1518) + (v147 * (if self.scalar_v1230 { ((if self.scalar_v1230 { ((-(v825 * self.scalar_v4598)) / (v1231 * v1231)) } else { v4362 }) * v4610) } else { v4366 }))) + ((v1243 * v1532) + (v158 * (if self.scalar_v1230 { ((if self.scalar_v1230 { ((-(v825 * self.scalar_v4621)) / (v1236 * v1236)) } else { v4172 }) * v4634) } else { v4183 }))));
        let v4665: f64 = ((v147 * (if self.scalar_v1230 { ((if self.scalar_v1230 { (v360 / v1231) } else { v304 }) * v4610) } else { v304 })) + (v158 * (if self.scalar_v1230 { ((if self.scalar_v1230 { (v360 / v1236) } else { v4176 }) * v4634) } else { v4187 })));
        let v4666: f64 = ((v147 * (if self.scalar_v1230 { ((if self.scalar_v1230 { (v51 / v1231) } else { v304 }) * v4610) } else { v304 })) + (v158 * (if self.scalar_v1230 { ((if self.scalar_v1230 { (v51 / v1236) } else { v304 }) * v4634) } else { v304 })));
        let v4668: f64 = (if self.scalar_v1230 { ((v147 * (if self.scalar_v1230 { ((if self.scalar_v1230 { v304 } else { v4363 }) * v4610) } else { v4367 })) + (v158 * (if self.scalar_v1230 { ((if self.scalar_v1230 { v304 } else { v4173 }) * v4634) } else { v4184 }))) } else { v304 });
        let v4670: f64 = (if self.scalar_v1230 { ((v147 * (if self.scalar_v1230 { ((if self.scalar_v1230 { v304 } else { v4364 }) * v4610) } else { v4368 })) + (v158 * (if self.scalar_v1230 { ((if self.scalar_v1230 { v304 } else { v4175 }) * v4634) } else { v4186 }))) } else { v304 });
        let v4673: f64 = (if self.scalar_v1247 { v304 } else { (if self.scalar_v1230 { v4662 } else { v304 }) });
        let v4674: f64 = (if self.scalar_v1247 { v304 } else { v4668 });
        let v4675: f64 = (if self.scalar_v1247 { v304 } else { (if self.scalar_v1230 { (v158 * (if self.scalar_v1230 { ((if self.scalar_v1230 { v304 } else { v4174 }) * v4634) } else { v4185 })) } else { v304 }) });
        let v4676: f64 = (if self.scalar_v1247 { v304 } else { v4670 });
        let v4677: f64 = (if self.scalar_v1247 { v304 } else { (if self.scalar_v1230 { v4665 } else { v304 }) });
        let v4678: f64 = (if self.scalar_v1247 { v304 } else { (if self.scalar_v1230 { v4666 } else { v304 }) });
        let v4688: f64 = (if self.scalar_v1254 { v304 } else { (if self.scalar_v1249 { (v51 / v41) } else { v304 }) });
        let v4689: f64 = (if self.scalar_v1254 { v304 } else { (if self.scalar_v1249 { ((-(v1251 * (self.scalar_v38 * (self.scalar_v1346 * (self.scalar_v39 * f64::powf(v12, self.scalar_v1377)))))) / (v41 * v41)) } else { v304 }) });
        let v4690: f64 = (if self.scalar_v1254 { v304 } else { (if self.scalar_v1249 { (v360 / v41) } else { v304 }) });
        let v4694: f64 = (self.scalar_v337 * (v1257 * v3337));
        let v4695: f64 = (self.scalar_v337 * (v1257 * v3338));
        let v4696: f64 = (self.scalar_v337 * (v1257 * v3339));
        let v4700: f64 = (v1260 * v1260);
        let v4723: f64 = { let limexp_arg = v1270; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v4728: f64 = (v1261 * (((v1260 * v4694) - (v1259 * v4694)) / v4700));
        let v4730: f64 = (v1261 * (((v1260 * v4695) - (v1259 * v4695)) / v4700));
        let v4732: f64 = (v1261 * (((v1260 * v4696) - (v1259 * v4696)) / v4700));
        let v4753: f64 = ((v1277 * (self.scalar_v1262 * (self.scalar_v1263 * v3396))) + (v1266 * (v1257 * ((v1274 * (self.scalar_v1267 * (self.scalar_v4722 * v4723))) + (v1272 * (v4730 + v4730))))));
        let v4776: f64 = (v946 * ((v1278 * v3337) + (v905 * ((v1277 * (self.scalar_v1262 * (self.scalar_v1263 * v3393))) + (v1266 * (v1257 * (v1272 * (v4728 + v4728))))))));
        let v4782: f64 = ((v946 * (v905 * ((v1277 * (self.scalar_v1262 * (self.scalar_v1263 * v3394))) + (v1266 * (v1257 * (v1274 * (self.scalar_v1267 * (self.scalar_v4721 * v4723)))))))) - (v1281 * v3472));
        let v4783: f64 = (v4782 / v3479);
        let v4787: f64 = (((v946 * (v905 * (v1277 * (self.scalar_v1262 * (self.scalar_v1263 * v3395))))) - (v1281 * v3473)) / v3479);
        let v4792: f64 = (v946 * ((v1278 * v3339) + (v905 * ((v1277 * (self.scalar_v1262 * (self.scalar_v1263 * v3397))) + (v1266 * (v1257 * (v1272 * (v4732 + v4732))))))));
        let v4796: f64 = ((self.scalar_v989 * ((v428 * v1673) + (v279 * v1871))) + ((v4776 - (v1281 * v3471)) / v3479));
        let v4797: f64 = ((self.scalar_v989 * (v279 * v1872)) + (((v946 * ((v1278 * v3338) + (v905 * v4753))) - (v1281 * v3474)) / v3479));
        let v4798: f64 = ((self.scalar_v989 * (v279 * v1873)) + ((v4792 - (v1281 * v3475)) / v3479));
        let v4805: f64 = (self.scalar_v1084 * ((v488 * v1673) + (v279 * (if self.scalar_v390 { ((v2024 + (self.scalar_v422 * (v1950 + (-v1993)))) - v1963) } else { (if self.scalar_v354 { (v1931 + v1935) } else { v304 }) }))));
        let v4806: f64 = (self.scalar_v1084 * (v279 * (if self.scalar_v390 { (v2025 + (self.scalar_v422 * (v51 - v1994))) } else { (if self.scalar_v354 { (v1932 + v1936) } else { v304 }) })));
        let v4807: f64 = (self.scalar_v1084 * (v279 * (if self.scalar_v390 { (v2026 + (self.scalar_v422 * (-v1995))) } else { (if self.scalar_v354 { (v1933 + v1937) } else { v304 }) })));
        let v4808: f64 = (self.scalar_v1084 * (v279 * (if self.scalar_v390 { (v2027 + (self.scalar_v422 * (v360 - v1996))) } else { (if self.scalar_v354 { (v1934 + v1938) } else { v304 }) })));
        let v4813: f64 = (v284 * v2519);
        let v4815: f64 = (v284 * v2521);
        let v4825: f64 = ((((v671 * (self.scalar_v280 * v1680)) + (v284 * v2517)) + (self.scalar_v1287 * v3358)) + (self.scalar_v1290 * v4382));
        let v4826: f64 = (((v284 * v2518) + (self.scalar_v1287 * v3359)) + (self.scalar_v1290 * v4383));
        let v4827: f64 = (((v284 * v2520) + (self.scalar_v1287 * v3360)) + (self.scalar_v1290 * v4384));
        let v4828: f64 = (self.scalar_v1290 * v4391);
        let v4829: f64 = (self.scalar_v1290 * v4392);
        let v4830: f64 = (self.scalar_v1290 * v4393);
        let v4837: f64 = (v286 * v3032);
        let v4844: f64 = (((v816 * (self.scalar_v285 * v1680)) + (v286 * (if self.scalar_v636 { ((v3002 + (self.scalar_v665 * (v2902 + (-v2959)))) - v2915) } else { v2893 }))) + (self.scalar_v1287 * (if self.scalar_v985 { v304 } else { v3565 })));
        let v4845: f64 = ((v286 * (if self.scalar_v636 { (v3003 + (self.scalar_v665 * (-v2960))) } else { v2894 })) + (self.scalar_v1287 * (if self.scalar_v985 { v304 } else { v3566 })));
        let v4846: f64 = ((v286 * (if self.scalar_v636 { (v3004 + (self.scalar_v665 * (v51 - v2961))) } else { v2895 })) + (self.scalar_v1287 * (if self.scalar_v985 { v304 } else { v3567 })));
        let v4847: f64 = ((v286 * (if self.scalar_v636 { (v3005 + (self.scalar_v665 * (-v2962))) } else { v2896 })) + (self.scalar_v1287 * (if self.scalar_v985 { v304 } else { v3568 })));
        let v4848: f64 = ((v286 * (if self.scalar_v636 { (v3007 + (self.scalar_v665 * (v360 - v2964))) } else { v2898 })) + (self.scalar_v1287 * (if self.scalar_v985 { v304 } else { v3569 })));
        let v4851: f64 = ((v900 * (self.scalar_v287 * (((-(self.scalar_v248 * v1664)) / v1685) * (self.scalar_v289 * f64::powf(v288, self.scalar_v1687))))) + (v291 * (if self.scalar_v899 { v304 } else { (if self.scalar_v860 { (v3300 - v3177) } else { (if self.scalar_v823 { (v3129 + v3136) } else { v304 }) }) })));
        let v4852: f64 = (v291 * (if self.scalar_v899 { v304 } else { v3309 }));
        let v4853: f64 = (v291 * (if self.scalar_v899 { v304 } else { v3310 }));
        let v4854: f64 = (v291 * (if self.scalar_v899 { v304 } else { v3311 }));
        let v4855: f64 = (v291 * (if self.scalar_v899 { v304 } else { v3312 }));
        let v4859: f64 = ((v291 * (if self.scalar_v899 { v304 } else { v3313 })) + self.scalar_v4858);
        let v4860: f64 = (self.scalar_v1298 + (v291 * (if self.scalar_v899 { v304 } else { v3314 })));
        let v4950: f64 = (((((((v341 * v4001) + (v345 * v4341)) + (v1310 * v4302)) + (v343 * v4108)) + (v349 * v4212)) + (v1251 * v4689)) + (v825 * v4673));
        let v4951: f64 = ((((((v341 * v4002) + ((-v1151) + (v345 * v4342))) + (v1145 + (v1310 * v4303))) + (v343 * v4109)) + (v349 * v4213)) + (v825 * v4674));
        let v4952: f64 = ((((((v341 * v4003) + (v345 * v4343)) + (v1310 * v4304)) + (v1096 + (v343 * v4110))) + (v1126 + (v349 * v4214))) + (v825 * v4675));
        let v4953: f64 = ((((((v1074 + (v341 * v4004)) + (v1151 + (v345 * v4344))) + (v1310 * v4305)) + (v343 * v4111)) + (v349 * v4215)) + (v825 * v4676));
        let v4954: f64 = (((((v341 * v4006) + (v345 * v4346)) + (v343 * v4113)) + ((-v1126) + (v349 * v4216))) + ((-v1248) + (v825 * v4677)));
        let v4970: f64 = ((((((v341 * v4007) + (v345 * v4347)) + (v343 * v4114)) + ((-v1255) + (v1251 * v4690))) + (v1248 + (v825 * v4678))) + ((v1321 * v3685) + (-v988)));
        let v5009: f64 = ((((((-v1074) + (v341 * v4005)) + (v345 * v4345)) + ((v1310 * v4306) + (-v1145))) + ((-v1096) + (v343 * v4112))) + (v1208 * v4556));
        let v5016: f64 = ((((((v4950 + (v1321 * v3680)) + (v1154 * v4358)) + (v1174 * v4514)) + (v1202 * v4528)) + (v1208 * v4552)) + (v1216 * v4567));
        let v5033: f64 = (-(v1158 + (v1154 * v4357)));
        let v5034: f64 = (-(v1206 + (v1202 * v4527)));
        let v5035: f64 = (-(v1220 + (v1216 * v4566)));
        let v5036: f64 = (-(v1255 + (v1251 * v4688)));
        let v5037: f64 = (-(v5016 + (v1222 * v4592)));
        let v5038: f64 = (-((((-v1158) + (v1154 * v4359)) + (v1199 + (v1174 * v4515))) + ((-v1227) + (v1222 * v4593))));
        let v5039: f64 = (-((((v4951 + (v1321 * v3681)) + ((-v1199) + (v1174 * v4516))) + (v1208 * v4553)) + (v1222 * v4594)));
        let v5040: f64 = (-((((v4952 + (v988 + (v1321 * v3682))) + ((-v1206) + (v1202 * v4529))) + (v1213 + (v1208 * v4554))) + (v1222 * v4595)));
        let v5041: f64 = (-((((v4953 + (v1321 * v3683)) + (v1174 * v4517)) + ((-v1213) + (v1208 * v4555))) + (v1222 * v4596)));
        let v5042: f64 = (-(v5009 + ((-v1220) + (v1216 * v4568))));
        let v5043: f64 = (-((v4954 + (v1321 * v3684)) + (v1227 + (v1222 * v4597))));
        let v5044: f64 = (-v4970);

        let d1074_dn4: f64 = v4001;
        let d1074_dn6: f64 = v4002;
        let d1074_dn7: f64 = v4003;
        let d1074_dn8: f64 = v4004;
        let d1074_dn9: f64 = v4005;
        let d1074_dn10: f64 = v4006;
        let d1074_dn11: f64 = v4007;
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(8),
            Some(9),
            multiplicity * (v1074),
            [4, 6, 7, 8, 9, 10, 11],
            [d1074_dn4, d1074_dn6, d1074_dn7, d1074_dn8, d1074_dn9, d1074_dn10, d1074_dn11],
            [],
            [],
            multiplicity,
        );
        let d1096_dn4: f64 = v4108;
        let d1096_dn6: f64 = v4109;
        let d1096_dn7: f64 = v4110;
        let d1096_dn8: f64 = v4111;
        let d1096_dn9: f64 = v4112;
        let d1096_dn10: f64 = v4113;
        let d1096_dn11: f64 = v4114;
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(7),
            Some(9),
            multiplicity * (v1096),
            [4, 6, 7, 8, 9, 10, 11],
            [d1096_dn4, d1096_dn6, d1096_dn7, d1096_dn8, d1096_dn9, d1096_dn10, d1096_dn11],
            [],
            [],
            multiplicity,
        );
        let d948_dn4: f64 = v3498;
        let d948_dn6: f64 = v3501;
        let d948_dn7: f64 = v3504;
        let d948_dn8: f64 = v3508;
        let d948_dn9: f64 = v3512;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(6),
            Some(9),
            multiplicity * (v948),
            [4, 6, 7, 8, 9],
            [d948_dn4, d948_dn6, d948_dn7, d948_dn8, d948_dn9],
            [],
            [],
            multiplicity,
        );
        let d947_dn4: f64 = v3480;
        let d947_dn6: f64 = v3484;
        let d947_dn7: f64 = v3487;
        let d947_dn8: f64 = v3491;
        let d947_dn9: f64 = v3494;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(9),
            Some(6),
            multiplicity * (v947),
            [4, 6, 7, 8, 9],
            [d947_dn4, d947_dn6, d947_dn7, d947_dn8, d947_dn9],
            [],
            [],
            multiplicity,
        );
        let d1151_dn4: f64 = v4341;
        let d1151_dn6: f64 = v4342;
        let d1151_dn7: f64 = v4343;
        let d1151_dn8: f64 = v4344;
        let d1151_dn9: f64 = v4345;
        let d1151_dn10: f64 = v4346;
        let d1151_dn11: f64 = v4347;
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(8),
            Some(6),
            multiplicity * (v1151),
            [4, 6, 7, 8, 9, 10, 11],
            [d1151_dn4, d1151_dn6, d1151_dn7, d1151_dn8, d1151_dn9, d1151_dn10, d1151_dn11],
            [],
            [],
            multiplicity,
        );
        let d1126_dn4: f64 = v4212;
        let d1126_dn6: f64 = v4213;
        let d1126_dn7: f64 = v4214;
        let d1126_dn8: f64 = v4215;
        let d1126_dn10: f64 = v4216;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(7),
            Some(10),
            multiplicity * (v1126),
            [4, 6, 7, 8, 10],
            [d1126_dn4, d1126_dn6, d1126_dn7, d1126_dn8, d1126_dn10],
            [],
            [],
            multiplicity,
        );
        let d1158_dn0: f64 = v4357;
        let d1158_dn4: f64 = v4358;
        let d1158_dn5: f64 = v4359;
        stamper.stamp_current_node3_local(
            Some(0),
            Some(5),
            multiplicity * (v1158),
            0,
            multiplicity * (d1158_dn0),
            4,
            multiplicity * (d1158_dn4),
            5,
            multiplicity * (d1158_dn5),
        );
        let d1199_dn4: f64 = v4514;
        let d1199_dn5: f64 = v4515;
        let d1199_dn6: f64 = v4516;
        let d1199_dn8: f64 = v4517;
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(5),
            Some(6),
            multiplicity * (v1199),
            [4, 5, 6, 8],
            [d1199_dn4, d1199_dn5, d1199_dn6, d1199_dn8],
            [],
            [],
            multiplicity,
        );
        let d1206_dn1: f64 = v4527;
        let d1206_dn4: f64 = v4528;
        let d1206_dn7: f64 = v4529;
        stamper.stamp_current_node3_local(
            Some(1),
            Some(7),
            multiplicity * (v1206),
            1,
            multiplicity * (d1206_dn1),
            4,
            multiplicity * (d1206_dn4),
            7,
            multiplicity * (d1206_dn7),
        );
        let d1213_dn4: f64 = v4552;
        let d1213_dn6: f64 = v4553;
        let d1213_dn7: f64 = v4554;
        let d1213_dn8: f64 = v4555;
        let d1213_dn9: f64 = v4556;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(7),
            Some(8),
            multiplicity * (v1213),
            [4, 6, 7, 8, 9],
            [d1213_dn4, d1213_dn6, d1213_dn7, d1213_dn8, d1213_dn9],
            [],
            [],
            multiplicity,
        );
        let d1220_dn2: f64 = v4566;
        let d1220_dn4: f64 = v4567;
        let d1220_dn9: f64 = v4568;
        stamper.stamp_current_node3_local(
            Some(2),
            Some(9),
            multiplicity * (v1220),
            2,
            multiplicity * (d1220_dn2),
            4,
            multiplicity * (d1220_dn4),
            9,
            multiplicity * (d1220_dn9),
        );
        let d1227_dn4: f64 = v4592;
        let d1227_dn5: f64 = v4593;
        let d1227_dn6: f64 = v4594;
        let d1227_dn7: f64 = v4595;
        let d1227_dn8: f64 = v4596;
        let d1227_dn10: f64 = v4597;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(10),
            Some(5),
            multiplicity * (v1227),
            [4, 5, 6, 7, 8, 10],
            [d1227_dn4, d1227_dn5, d1227_dn6, d1227_dn7, d1227_dn8, d1227_dn10],
            [],
            [],
            multiplicity,
        );
        let d1283_dn4: f64 = v4796;
        let d1283_dn6: f64 = v4783;
        let d1283_dn7: f64 = v4787;
        let d1283_dn8: f64 = v4797;
        let d1283_dn9: f64 = v4798;
        let v1283_ddt: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, v1283);
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(8),
            Some(9),
            multiplicity * (v1283_ddt),
            [4, 6, 7, 8, 9],
            [((d1283_dn4) * ddt_scale), ((d1283_dn6) * ddt_scale), ((d1283_dn7) * ddt_scale), ((d1283_dn8) * ddt_scale), ((d1283_dn9) * ddt_scale)],
            [],
            [],
            multiplicity,
        );
        let d1285_dn4: f64 = v4805;
        let d1285_dn7: f64 = v4806;
        let d1285_dn8: f64 = v4807;
        let d1285_dn9: f64 = v4808;
        let v1285_ddt: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, v1285);
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(7),
            Some(9),
            multiplicity * (v1285_ddt),
            [4, 7, 8, 9],
            [((d1285_dn4) * ddt_scale), ((d1285_dn7) * ddt_scale), ((d1285_dn8) * ddt_scale), ((d1285_dn9) * ddt_scale)],
            [],
            [],
            multiplicity,
        );
        let d1292_dn4: f64 = v4825;
        let d1292_dn6: f64 = v4826;
        let d1292_dn7: f64 = v4813;
        let d1292_dn8: f64 = v4827;
        let d1292_dn9: f64 = v4815;
        let v1292_ddt: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, v1292);
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(8),
            Some(6),
            multiplicity * (v1292_ddt),
            [4, 6, 7, 8, 9],
            [((d1292_dn4) * ddt_scale), ((d1292_dn6) * ddt_scale), ((d1292_dn7) * ddt_scale), ((d1292_dn8) * ddt_scale), ((d1292_dn9) * ddt_scale)],
            [],
            [],
            multiplicity,
        );
        let d1293_dn4: f64 = v4828;
        let d1293_dn5: f64 = v4829;
        let d1293_dn8: f64 = v4830;
        let v1293_ddt: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, v1293);
        stamper.stamp_current_node3_local(
            Some(8),
            Some(5),
            multiplicity * (v1293_ddt),
            4,
            multiplicity * (((d1293_dn4) * ddt_scale)),
            5,
            multiplicity * (((d1293_dn5) * ddt_scale)),
            8,
            multiplicity * (((d1293_dn8) * ddt_scale)),
        );
        let d1296_dn4: f64 = v4844;
        let d1296_dn6: f64 = v4845;
        let d1296_dn7: f64 = v4846;
        let d1296_dn8: f64 = v4847;
        let d1296_dn9: f64 = v4837;
        let d1296_dn10: f64 = v4848;
        let v1296_ddt: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, v1296);
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(7),
            Some(10),
            multiplicity * (v1296_ddt),
            [4, 6, 7, 8, 9, 10],
            [((d1296_dn4) * ddt_scale), ((d1296_dn6) * ddt_scale), ((d1296_dn7) * ddt_scale), ((d1296_dn8) * ddt_scale), ((d1296_dn9) * ddt_scale), ((d1296_dn10) * ddt_scale)],
            [],
            [],
            multiplicity,
        );
        let d1303_dn1: f64 = self.scalar_v1302;
        let d1303_dn2: f64 = self.scalar_v4861;
        let v1303_ddt: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 5, v1303);
        stamper.stamp_current_node2_local(
            Some(1),
            Some(2),
            multiplicity * (v1303_ddt),
            1,
            multiplicity * (((d1303_dn1) * ddt_scale)),
            2,
            multiplicity * (((d1303_dn2) * ddt_scale)),
        );
        let d1306_dn0: f64 = self.scalar_v4862;
        let d1306_dn1: f64 = self.scalar_v1305;
        let v1306_ddt: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 6, v1306);
        stamper.stamp_current_node2_local(
            Some(1),
            Some(0),
            multiplicity * (v1306_ddt),
            0,
            multiplicity * (((d1306_dn0) * ddt_scale)),
            1,
            multiplicity * (((d1306_dn1) * ddt_scale)),
        );
        let d1248_dn4: f64 = v4673;
        let d1248_dn6: f64 = v4674;
        let d1248_dn7: f64 = v4675;
        let d1248_dn8: f64 = v4676;
        let d1248_dn10: f64 = v4677;
        let d1248_dn11: f64 = v4678;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(11),
            Some(10),
            multiplicity * (v1248),
            [4, 6, 7, 8, 10, 11],
            [d1248_dn4, d1248_dn6, d1248_dn7, d1248_dn8, d1248_dn10, d1248_dn11],
            [],
            [],
            multiplicity,
        );
        let d988_dn4: f64 = v3680;
        let d988_dn6: f64 = v3681;
        let d988_dn7: f64 = v3682;
        let d988_dn8: f64 = v3683;
        let d988_dn10: f64 = v3684;
        let d988_dn11: f64 = v3685;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(7),
            Some(11),
            multiplicity * (v988),
            [4, 6, 7, 8, 10, 11],
            [d988_dn4, d988_dn6, d988_dn7, d988_dn8, d988_dn10, d988_dn11],
            [],
            [],
            multiplicity,
        );
        let d1255_dn3: f64 = v4688;
        let d1255_dn4: f64 = v4689;
        let d1255_dn11: f64 = v4690;
        stamper.stamp_current_node3_local(
            Some(3),
            Some(11),
            multiplicity * (v1255),
            3,
            multiplicity * (d1255_dn3),
            4,
            multiplicity * (d1255_dn4),
            11,
            multiplicity * (d1255_dn11),
        );
        let d1300_dn4: f64 = v4851;
        let d1300_dn6: f64 = v4852;
        let d1300_dn7: f64 = v4853;
        let d1300_dn8: f64 = v4854;
        let d1300_dn9: f64 = v4855;
        let d1300_dn10: f64 = v4859;
        let d1300_dn11: f64 = v4860;
        let v1300_ddt: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 7, v1300);
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(11),
            Some(10),
            multiplicity * (v1300_ddt),
            [4, 6, 7, 8, 9, 10, 11],
            [((d1300_dn4) * ddt_scale), ((d1300_dn6) * ddt_scale), ((d1300_dn7) * ddt_scale), ((d1300_dn8) * ddt_scale), ((d1300_dn9) * ddt_scale), ((d1300_dn10) * ddt_scale), ((d1300_dn11) * ddt_scale)],
            [],
            [],
            multiplicity,
        );
        let d1342_dn4: f64 = self.scalar_v5047;
        stamper.stamp_current_node1_local(
            Some(4),
            None,
            multiplicity * (v1342),
            4,
            multiplicity * (d1342_dn4),
        );
        let d1336_dn0: f64 = v5033;
        let d1336_dn1: f64 = v5034;
        let d1336_dn2: f64 = v5035;
        let d1336_dn3: f64 = v5036;
        let d1336_dn4: f64 = v5037;
        let d1336_dn5: f64 = v5038;
        let d1336_dn6: f64 = v5039;
        let d1336_dn7: f64 = v5040;
        let d1336_dn8: f64 = v5041;
        let d1336_dn9: f64 = v5042;
        let d1336_dn10: f64 = v5043;
        let d1336_dn11: f64 = v5044;
        let v1336_node_derivatives: [f64; 12] = [d1336_dn0, d1336_dn1, d1336_dn2, d1336_dn3, d1336_dn4, d1336_dn5, d1336_dn6, d1336_dn7, d1336_dn8, d1336_dn9, d1336_dn10, d1336_dn11];
        let v1336_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense_local(
            Some(4),
            None,
            multiplicity * (v1336),
            &v1336_node_derivatives,
            &v1336_branch_derivatives,
            multiplicity,
        );
        let d1344_dn4: f64 = self.scalar_v1343;
        let v1344_ddt: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 8, v1344);
        stamper.stamp_current_node1_local(
            Some(4),
            None,
            multiplicity * (v1344_ddt),
            4,
            multiplicity * (((d1344_dn4) * ddt_scale)),
        );
    }

    pub fn stamp_reactive(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedReactiveStamper<'_>) {
        let nodes = self.nodes;
        let branches = self.branches;
        self.ensure_temperature_static(ctx.temperature(), ctx.thermal_voltage());
        let p = &(*self.params);
        let multiplicity = self.multiplicity;
        let v6: f64 = ctx.node_voltage(nodes[4]);
        let v7: f64 = (self.scalar_v5 + v6);
        let v11: f64 = ((v7 * 1.3806503e-23) / 1.602176462e-19);
        let v12: f64 = (v7 / self.scalar_v2);
        let v17: f64 = (self.scalar_v14 * f64::powf(v12, self.scalar_v15));
        let v48: f64 = f64::powf(v12, self.scalar_v47);
        let v51: f64 = 1.0;
        let v52: f64 = (v51 - v12);
        let v53: f64 = (self.scalar_v50 * v52);
        let v55: f64 = (((v53 / v11)) as f64).exp();
        let v56: f64 = (v48 * v55);
        let v60: f64 = (self.scalar_v46 * f64::powf(v56, self.scalar_v58));
        let v63: f64 = f64::powf(v12, self.scalar_v62);
        let v66: f64 = (v52 * self.scalar_v65);
        let v68: f64 = (((v66 / v11)) as f64).exp();
        let v69: f64 = (v63 * v68);
        let v73: f64 = (self.scalar_v61 * f64::powf(v69, self.scalar_v71));
        let v77: f64 = (v52 * self.scalar_v76);
        let v79: f64 = (((v77 / v11)) as f64).exp();
        let v80: f64 = (v48 * v79);
        let v84: f64 = (self.scalar_v74 * f64::powf(v80, self.scalar_v82));
        let v161: f64 = (v51 + ((v7 - self.scalar_v2) * self.scalar_v159));
        let v162: f64 = (self.scalar_v57 * v161);
        let v163: f64 = (self.scalar_v70 * v161);
        let v182: f64 = 2.0;
        let v184: f64 = (v182 * (v11 / v12));
        let v185: f64 = 0.5;
        let v188: f64 = (v12 * self.scalar_v187);
        let v190: f64 = (((v188 / v11)) as f64).exp();
        let v191: f64 = -0.5;
        let v193: f64 = (v12 * self.scalar_v192);
        let v195: f64 = (((v193 / v11)) as f64).exp();
        let v196: f64 = (v190 - v195);
        let v197: f64 = ((v196) as f64).ln();
        let v198: f64 = (v184 * v197);
        let v201: f64 = (v11 * 3.0);
        let v202: f64 = ((v12) as f64).ln();
        let v203: f64 = (v201 * v202);
        let v205: f64 = (v12 - v51);
        let v207: f64 = (((v12 * v198) - v203) - (self.scalar_v88 * v205));
        let v208: f64 = (v11 * v182);
        let v209: f64 = 4.0;
        let v210: f64 = (-v207);
        let v212: f64 = (((v210 / v11)) as f64).exp();
        let v215: f64 = (((v51 + (v209 * v212))) as f64).sqrt();
        let v217: f64 = (v185 * (v51 + v215));
        let v218: f64 = ((v217) as f64).ln();
        let v220: f64 = (v207 + (v208 * v218));
        let v223: f64 = (v12 * self.scalar_v222);
        let v225: f64 = (((v223 / v11)) as f64).exp();
        let v227: f64 = (v12 * self.scalar_v226);
        let v229: f64 = (((v227 / v11)) as f64).exp();
        let v230: f64 = (v225 - v229);
        let v231: f64 = ((v230) as f64).ln();
        let v232: f64 = (v184 * v231);
        let v236: f64 = (((v12 * v232) - v203) - (self.scalar_v112 * v205));
        let v237: f64 = (-v236);
        let v239: f64 = (((v237 / v11)) as f64).exp();
        let v242: f64 = (((v51 + (v209 * v239))) as f64).sqrt();
        let v244: f64 = (v185 * (v51 + v242));
        let v245: f64 = ((v244) as f64).ln();
        let v247: f64 = (v236 + (v208 * v245));
        let v250: f64 = (v12 * self.scalar_v249);
        let v252: f64 = (((v250 / v11)) as f64).exp();
        let v254: f64 = (v12 * self.scalar_v253);
        let v256: f64 = (((v254 / v11)) as f64).exp();
        let v257: f64 = (v252 - v256);
        let v258: f64 = ((v257) as f64).ln();
        let v259: f64 = (v184 * v258);
        let v263: f64 = (((v12 * v259) - v203) - (self.scalar_v138 * v205));
        let v264: f64 = (-v263);
        let v266: f64 = (((v264 / v11)) as f64).exp();
        let v269: f64 = (((v51 + (v209 * v266))) as f64).sqrt();
        let v271: f64 = (v185 * (v51 + v269));
        let v272: f64 = ((v271) as f64).ln();
        let v274: f64 = (v263 + (v208 * v272));
        let v276: f64 = (self.scalar_v186 / v220);
        let v279: f64 = (self.scalar_v275 * f64::powf(v276, self.scalar_v277));
        let v281: f64 = (self.scalar_v221 / v247);
        let v283: f64 = f64::powf(v281, self.scalar_v282);
        let v284: f64 = (self.scalar_v280 * v283);
        let v286: f64 = (v283 * self.scalar_v285);
        let v288: f64 = (self.scalar_v248 / v274);
        let v291: f64 = (self.scalar_v287 * f64::powf(v288, self.scalar_v289));
        let v293: f64 = (v48 * self.scalar_v292);
        let v294: f64 = (v55 * v293);
        let v304: f64 = 0.0;
        let v314: f64 = (if self.scalar_v312 { (v51 / v17) } else { v304 });
        let v339: f64 = ctx.node_voltage(nodes[8]);
        let v340: f64 = ctx.node_voltage(nodes[9]);
        let v341: f64 = (v339 - v340);
        let v342: f64 = ctx.node_voltage(nodes[7]);
        let v343: f64 = (v342 - v340);
        let v345: f64 = (v339 - ctx.node_voltage(nodes[6]));
        let v347: f64 = (v339 - ctx.node_voltage(nodes[5]));
        let v348: f64 = ctx.node_voltage(nodes[10]);
        let v349: f64 = (v342 - v348);
        let v350: f64 = (-v220);
        let v352: f64 = (v350 * self.scalar_v351);
        let v355: f64 = (v341 + v352);
        let v356: f64 = (if self.scalar_v354 { v355 } else { v304 });
        let v357: bool = (v356 > v304);
        let v358: bool = (self.scalar_v354 && v357);
        let v360: f64 = -1.0;
        let v363: f64 = (if v358 { self.scalar_v362 } else { v304 });
        let v366: f64 = (v51 - (self.scalar_v359 * (self.scalar_v359 * v363)));
        let v372: f64 = (v356 * self.scalar_v371);
        let v374: f64 = (self.scalar_v359 + (v372 / v220));
        let v379: bool = (self.scalar_v354 && (!v357));
        let v381: f64 = (v51 - (v341 / v220));
        let v383: f64 = (v51 - f64::powf(v381, self.scalar_v368));
        let v386: f64 = (if v379 { ((v220 * v383) / self.scalar_v368) } else { (if v358 { ((v220 * v366) / self.scalar_v368) } else { v304 }) });
        let v387: f64 = (if v379 { v304 } else { (if v358 { (v363 * (v356 * v374)) } else { v304 }) });
        let v395: f64 = ((((v352 * v352) + self.scalar_v393)) as f64).sqrt();
        let v396: f64 = (if self.scalar_v390 { v395 } else { v304 });
        let v399: f64 = (if self.scalar_v390 { (v191 * (v352 + v396)) } else { v304 });
        let v401: f64 = (v51 - (v399 / v220));
        let v402: f64 = f64::powf(v401, self.scalar_v368);
        let v405: f64 = (if self.scalar_v390 { ((v350 * v402) / self.scalar_v368) } else { v304 });
        let v406: f64 = (if self.scalar_v390 { v355 } else { v304 });
        let v409: f64 = (((self.scalar_v393 + (v406 * v406))) as f64).sqrt();
        let v410: f64 = (if self.scalar_v390 { v409 } else { v304 });
        let v414: f64 = (if self.scalar_v390 { ((v185 * (v406 - v410)) - v352) } else { v304 });
        let v416: f64 = (v51 - (v414 / v220));
        let v417: f64 = f64::powf(v416, self.scalar_v368);
        let v420: f64 = (if self.scalar_v390 { ((v350 * v417) / self.scalar_v368) } else { v386 });
        let v428: f64 = (if self.scalar_v390 { ((v420 + (self.scalar_v422 * (v399 + (v341 - v414)))) - v405) } else { (if self.scalar_v354 { (v386 + v387) } else { v304 }) });
        let v429: f64 = (v343 + v352);
        let v430: f64 = (if self.scalar_v354 { v429 } else { v356 });
        let v431: bool = (v430 > v304);
        let v432: bool = (self.scalar_v354 && v431);
        let v433: f64 = (if v432 { self.scalar_v362 } else { v363 });
        let v436: f64 = (v51 - (self.scalar_v359 * (self.scalar_v359 * v433)));
        let v440: f64 = (self.scalar_v371 * v430);
        let v442: f64 = (self.scalar_v359 + (v440 / v220));
        let v447: bool = (self.scalar_v354 && (!v431));
        let v449: f64 = (v51 - (v343 / v220));
        let v451: f64 = (v51 - f64::powf(v449, self.scalar_v368));
        let v454: f64 = (if v447 { ((v220 * v451) / self.scalar_v368) } else { (if v432 { ((v220 * v436) / self.scalar_v368) } else { v420 }) });
        let v455: f64 = (if v447 { v304 } else { (if v432 { (v433 * (v430 * v442)) } else { v387 }) });
        let v458: f64 = (if self.scalar_v390 { v395 } else { v396 });
        let v461: f64 = (if self.scalar_v390 { (v191 * (v352 + v458)) } else { v399 });
        let v463: f64 = (v51 - (v461 / v220));
        let v464: f64 = f64::powf(v463, self.scalar_v368);
        let v467: f64 = (if self.scalar_v390 { ((v350 * v464) / self.scalar_v368) } else { v405 });
        let v468: f64 = (if self.scalar_v390 { v429 } else { v406 });
        let v471: f64 = (((self.scalar_v393 + (v468 * v468))) as f64).sqrt();
        let v472: f64 = (if self.scalar_v390 { v471 } else { v410 });
        let v476: f64 = (if self.scalar_v390 { ((v185 * (v468 - v472)) - v352) } else { v414 });
        let v478: f64 = (v51 - (v476 / v220));
        let v479: f64 = f64::powf(v478, self.scalar_v368);
        let v482: f64 = (if self.scalar_v390 { ((v350 * v479) / self.scalar_v368) } else { v454 });
        let v488: f64 = (if self.scalar_v390 { ((v482 + (self.scalar_v422 * (v461 + (v343 - v476)))) - v467) } else { (if self.scalar_v354 { (v454 + v455) } else { v304 }) });
        let v489: f64 = (-v247);
        let v490: f64 = (self.scalar_v351 * v489);
        let v493: f64 = (v345 + v490);
        let v494: f64 = (if self.scalar_v492 { v493 } else { v430 });
        let v495: bool = (v494 > v304);
        let v496: bool = (self.scalar_v492 && v495);
        let v499: f64 = (if v496 { self.scalar_v498 } else { v433 });
        let v502: f64 = (v51 - (self.scalar_v359 * (self.scalar_v359 * v499)));
        let v508: f64 = (v494 * self.scalar_v507);
        let v510: f64 = (self.scalar_v359 + (v508 / v247));
        let v518: bool = (self.scalar_v515 && (v345 < self.scalar_v516));
        let v520: bool = (self.scalar_v492 && (!v495));
        let v521: bool = (v518 && v520);
        let v523: f64 = (v51 + (self.scalar_v514 / v247));
        let v524: f64 = f64::powf(v523, self.scalar_v504);
        let v526: f64 = (self.scalar_v504 * (v345 + self.scalar_v514));
        let v527: f64 = (v247 + self.scalar_v514);
        let v529: f64 = (v51 - (v526 / v527));
        let v531: f64 = (v51 - (v524 * v529));
        let v536: bool = (v520 && (!v518));
        let v538: f64 = (v51 - (v345 / v247));
        let v540: f64 = (v51 - f64::powf(v538, self.scalar_v504));
        let v543: f64 = (if v536 { ((v247 * v540) / self.scalar_v504) } else { (if v521 { ((v247 * v531) / self.scalar_v504) } else { (if v496 { ((v247 * v502) / self.scalar_v504) } else { v482 }) }) });
        let v544: f64 = (if v520 { v304 } else { (if v496 { (v499 * (v494 * v510)) } else { v455 }) });
        let v552: f64 = (v490 + self.scalar_v514);
        let v553: f64 = (self.scalar_v514 - v490);
        let v554: f64 = (v552 / v553);
        let v555: f64 = (if self.scalar_v551 { v554 } else { v304 });
        let v556: f64 = (v182 * v555);
        let v557: f64 = (v555 - v51);
        let v562: f64 = ((((v557 * v557) + self.scalar_v560)) as f64).sqrt();
        let v563: f64 = (v51 + v555);
        let v568: f64 = ((((v563 * v563) + self.scalar_v566)) as f64).sqrt();
        let v569: f64 = (v562 + v568);
        let v571: f64 = (if self.scalar_v551 { (v556 / v569) } else { v304 });
        let v576: f64 = (if self.scalar_v551 { (v185 * (((v553 * v571) - self.scalar_v514) - v490)) } else { v461 });
        let v578: f64 = (v51 - (v576 / v247));
        let v580: f64 = (v51 - f64::powf(v578, self.scalar_v504));
        let v583: f64 = (if self.scalar_v551 { ((v247 * v580) / self.scalar_v504) } else { v304 });
        let v586: f64 = (v490 + (self.scalar_v514 + (v182 * v345)));
        let v588: f64 = (if self.scalar_v551 { (v586 / v553) } else { v304 });
        let v589: f64 = (v182 * v588);
        let v590: f64 = (v588 - v51);
        let v593: f64 = (((self.scalar_v560 + (v590 * v590))) as f64).sqrt();
        let v594: f64 = (v51 + v588);
        let v597: f64 = (((self.scalar_v566 + (v594 * v594))) as f64).sqrt();
        let v598: f64 = (v593 + v597);
        let v600: f64 = (if self.scalar_v551 { (v589 / v598) } else { v304 });
        let v605: f64 = (if self.scalar_v551 { (v185 * (((v553 * v600) - self.scalar_v514) - v490)) } else { v476 });
        let v607: f64 = (v51 - (v605 / v247));
        let v609: f64 = (v51 - f64::powf(v607, self.scalar_v504));
        let v612: f64 = (if self.scalar_v551 { ((v247 * v609) / self.scalar_v504) } else { v543 });
        let v615: f64 = (if self.scalar_v551 { (v185 * (v51 + v600)) } else { v304 });
        let v617: f64 = f64::powf(v523, self.scalar_v616);
        let v618: f64 = (if self.scalar_v551 { v617 } else { v304 });
        let v620: f64 = (v51 + (v490 / v247));
        let v621: f64 = f64::powf(v620, self.scalar_v616);
        let v622: f64 = (if self.scalar_v551 { v621 } else { v304 });
        let v623: f64 = (v51 - v615);
        let v627: f64 = (if self.scalar_v551 { ((v618 * v623) + (v615 * v622)) } else { v304 });
        let v629: f64 = (v576 + (v345 - v605));
        let v631: f64 = (if self.scalar_v551 { (v627 * v629) } else { v304 });
        let v639: f64 = (((self.scalar_v560 + (v490 * v490))) as f64).sqrt();
        let v640: f64 = (if self.scalar_v636 { v639 } else { v458 });
        let v643: f64 = (if self.scalar_v636 { (v191 * (v490 + v640)) } else { v576 });
        let v645: f64 = (v51 - (v643 / v247));
        let v646: f64 = f64::powf(v645, self.scalar_v504);
        let v649: f64 = (if self.scalar_v636 { ((v489 * v646) / self.scalar_v504) } else { v467 });
        let v650: f64 = (if self.scalar_v636 { v493 } else { v468 });
        let v653: f64 = (((self.scalar_v560 + (v650 * v650))) as f64).sqrt();
        let v654: f64 = (if self.scalar_v636 { v653 } else { v472 });
        let v658: f64 = (if self.scalar_v636 { ((v185 * (v650 - v654)) - v490) } else { v605 });
        let v660: f64 = (v51 - (v658 / v247));
        let v661: f64 = f64::powf(v660, self.scalar_v504);
        let v664: f64 = (if self.scalar_v636 { ((v489 * v661) / self.scalar_v504) } else { v612 });
        let v671: f64 = (if self.scalar_v636 { ((v664 + (self.scalar_v665 * (v643 + (v345 - v658)))) - v649) } else { (if self.scalar_v551 { ((v612 + v631) - v583) } else { (if self.scalar_v492 { (v543 + v544) } else { v304 }) }) });
        let v672: f64 = (v349 + v490);
        let v673: f64 = (if self.scalar_v492 { v672 } else { v494 });
        let v674: bool = (v673 > v304);
        let v675: bool = (self.scalar_v492 && v674);
        let v676: f64 = (if v675 { self.scalar_v498 } else { v499 });
        let v679: f64 = (v51 - (self.scalar_v359 * (self.scalar_v359 * v676)));
        let v683: f64 = (self.scalar_v507 * v673);
        let v685: f64 = (self.scalar_v359 + (v683 / v247));
        let v690: bool = (self.scalar_v515 && (v349 < self.scalar_v516));
        let v692: bool = (self.scalar_v492 && (!v674));
        let v693: bool = (v690 && v692);
        let v695: f64 = (self.scalar_v504 * (v349 + self.scalar_v514));
        let v697: f64 = (v51 - (v695 / v527));
        let v699: f64 = (v51 - (v524 * v697));
        let v704: bool = (v692 && (!v690));
        let v706: f64 = (v51 - (v349 / v247));
        let v708: f64 = (v51 - f64::powf(v706, self.scalar_v504));
        let v711: f64 = (if v704 { ((v247 * v708) / self.scalar_v504) } else { (if v693 { ((v247 * v699) / self.scalar_v504) } else { (if v675 { ((v247 * v679) / self.scalar_v504) } else { v664 }) }) });
        let v712: f64 = (if v692 { v304 } else { (if v675 { (v676 * (v673 * v685)) } else { v544 }) });
        let v715: f64 = (if self.scalar_v551 { v554 } else { v555 });
        let v716: f64 = (v182 * v715);
        let v717: f64 = (v715 - v51);
        let v720: f64 = (((self.scalar_v560 + (v717 * v717))) as f64).sqrt();
        let v721: f64 = (v51 + v715);
        let v724: f64 = (((self.scalar_v566 + (v721 * v721))) as f64).sqrt();
        let v725: f64 = (v720 + v724);
        let v727: f64 = (if self.scalar_v551 { (v716 / v725) } else { v571 });
        let v732: f64 = (if self.scalar_v551 { (v185 * (((v553 * v727) - self.scalar_v514) - v490)) } else { v643 });
        let v734: f64 = (v51 - (v732 / v247));
        let v736: f64 = (v51 - f64::powf(v734, self.scalar_v504));
        let v742: f64 = (v490 + (self.scalar_v514 + (v182 * v349)));
        let v744: f64 = (if self.scalar_v551 { (v742 / v553) } else { v588 });
        let v745: f64 = (v182 * v744);
        let v746: f64 = (v744 - v51);
        let v749: f64 = (((self.scalar_v560 + (v746 * v746))) as f64).sqrt();
        let v750: f64 = (v51 + v744);
        let v753: f64 = (((self.scalar_v566 + (v750 * v750))) as f64).sqrt();
        let v754: f64 = (v749 + v753);
        let v756: f64 = (if self.scalar_v551 { (v745 / v754) } else { v600 });
        let v761: f64 = (if self.scalar_v551 { (v185 * (((v553 * v756) - self.scalar_v514) - v490)) } else { v658 });
        let v763: f64 = (v51 - (v761 / v247));
        let v765: f64 = (v51 - f64::powf(v763, self.scalar_v504));
        let v768: f64 = (if self.scalar_v551 { ((v247 * v765) / self.scalar_v504) } else { v711 });
        let v771: f64 = (if self.scalar_v551 { (v185 * (v51 + v756)) } else { v615 });
        let v772: f64 = (if self.scalar_v551 { v617 } else { v618 });
        let v773: f64 = (if self.scalar_v551 { v621 } else { v622 });
        let v774: f64 = (v51 - v771);
        let v778: f64 = (if self.scalar_v551 { ((v772 * v774) + (v771 * v773)) } else { v627 });
        let v780: f64 = (v732 + (v349 - v761));
        let v785: f64 = (if self.scalar_v551 { ((v768 + (if self.scalar_v551 { (v778 * v780) } else { v631 })) - (if self.scalar_v551 { ((v247 * v736) / self.scalar_v504) } else { v583 })) } else { (if self.scalar_v492 { (v711 + v712) } else { v304 }) });
        let v786: f64 = (if self.scalar_v636 { v639 } else { v640 });
        let v789: f64 = (if self.scalar_v636 { (v191 * (v490 + v786)) } else { v732 });
        let v791: f64 = (v51 - (v789 / v247));
        let v792: f64 = f64::powf(v791, self.scalar_v504);
        let v795: f64 = (if self.scalar_v636 { ((v489 * v792) / self.scalar_v504) } else { v649 });
        let v796: f64 = (if self.scalar_v636 { v672 } else { v650 });
        let v799: f64 = (((self.scalar_v560 + (v796 * v796))) as f64).sqrt();
        let v800: f64 = (if self.scalar_v636 { v799 } else { v654 });
        let v804: f64 = (if self.scalar_v636 { ((v185 * (v796 - v800)) - v490) } else { v761 });
        let v806: f64 = (v51 - (v804 / v247));
        let v807: f64 = f64::powf(v806, self.scalar_v504);
        let v810: f64 = (if self.scalar_v636 { ((v489 * v807) / self.scalar_v504) } else { v768 });
        let v816: f64 = (if self.scalar_v636 { ((v810 + (self.scalar_v665 * (v789 + (v349 - v804)))) - v795) } else { v785 });
        let v818: f64 = (-v274);
        let v820: f64 = (if self.scalar_v817 { (self.scalar_v351 * v818) } else { v490 });
        let v825: f64 = (ctx.node_voltage(nodes[11]) - v348);
        let v826: f64 = (v820 + v825);
        let v827: f64 = (if self.scalar_v823 { v826 } else { v673 });
        let v828: bool = (v827 > v304);
        let v829: bool = (self.scalar_v823 && v828);
        let v832: f64 = (if v829 { self.scalar_v831 } else { v676 });
        let v835: f64 = (v51 - (self.scalar_v359 * (self.scalar_v359 * v832)));
        let v841: f64 = (v827 * self.scalar_v840);
        let v843: f64 = (self.scalar_v359 + (v841 / v274));
        let v848: bool = (self.scalar_v823 && (!v828));
        let v850: f64 = (v51 - (v825 / v274));
        let v852: f64 = (v51 - f64::powf(v850, self.scalar_v837));
        let v855: f64 = (if v848 { ((v274 * v852) / self.scalar_v837) } else { (if v829 { ((v274 * v835) / self.scalar_v837) } else { v810 }) });
        let v865: f64 = ((((v820 * v820) + self.scalar_v863)) as f64).sqrt();
        let v869: f64 = (if self.scalar_v860 { (v191 * (v820 + (if self.scalar_v860 { v865 } else { v786 }))) } else { v789 });
        let v871: f64 = (v51 - (v869 / v274));
        let v872: f64 = f64::powf(v871, self.scalar_v837);
        let v876: f64 = (if self.scalar_v860 { v826 } else { v796 });
        let v879: f64 = (((self.scalar_v863 + (v876 * v876))) as f64).sqrt();
        let v884: f64 = (if self.scalar_v860 { ((v185 * (v876 - (if self.scalar_v860 { v879 } else { v800 }))) - v820) } else { v804 });
        let v886: f64 = (v51 - (v884 / v274));
        let v887: f64 = f64::powf(v886, self.scalar_v837);
        let v897: f64 = (((if self.scalar_v860 { ((v818 * v887) / self.scalar_v837) } else { v855 }) + (self.scalar_v892 * (v869 + (v825 - v884)))) - (if self.scalar_v860 { ((v818 * v872) / self.scalar_v837) } else { v795 }));
        let v900: f64 = (if self.scalar_v899 { v304 } else { (if self.scalar_v860 { v897 } else { (if self.scalar_v823 { (v855 + (if v848 { v304 } else { (if v829 { (v832 * (v827 * v843)) } else { v712 }) })) } else { v304 }) }) });
        let v901: f64 = (v11 * v162);
        let v902: f64 = (v341 / v901);
        let v904: f64 = ({ let limexp_arg = v902; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } } - v51);
        let v905: f64 = (v60 * v904);
        let v906: f64 = (v11 * v163);
        let v907: f64 = (v345 / v906);
        let v908: f64 = { let limexp_arg = v907; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let v909: f64 = (v60 * v73);
        let v910: f64 = (v908 - v51);
        let v911: f64 = (v909 * v910);
        let v915: f64 = ((v51 + (self.scalar_v311 * v428)) + (self.scalar_v307 * v671));
        let v916: f64 = 0.0001;
        let v917: f64 = (v915 - v916);
        let v921: f64 = ((((v917 * v917) + 1e-8)) as f64).sqrt();
        let v925: f64 = (v916 + (v185 * ((v915 + v921) - v916)));
        let v934: f64 = (v209 * ((v314 * v905) + (self.scalar_v318 * v911)));
        let v935: f64 = (f64::powf(v925, self.scalar_v932) + v934);
        let v941: f64 = (v185 * v925);
        let v942: f64 = (v51 + v934);
        let v944: f64 = (v51 + f64::powf(v942, self.scalar_v931));
        let v946: f64 = (if self.scalar_v940 { (v941 * v944) } else { (if self.scalar_v930 { (v185 * (v925 + f64::powf(v935, self.scalar_v931))) } else { v304 }) });
        let v950: f64 = (v11 * self.scalar_v81);
        let v952: f64 = (if self.scalar_v949 { (v349 / v950) } else { v907 });
        let v956: f64 = (if self.scalar_v949 { (v345 / v950) } else { v304 });
        let v964: f64 = ((((if self.scalar_v949 { { let limexp_arg = v952; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } } } else { v908 }) * self.scalar_v959) + ((if self.scalar_v949 { { let limexp_arg = v956; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } } } else { v304 }) * self.scalar_v961)) - v51);
        let v1159: f64 = (v345 / v11);
        let v1160: f64 = { let limexp_arg = v1159; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let v1161: f64 = (v347 / v11);
        let v1162: f64 = { let limexp_arg = v1161; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let v1165: f64 = (((v51 + (v294 * v1160))) as f64).sqrt();
        let v1168: f64 = (((v51 + (v294 * v1162))) as f64).sqrt();
        let v1201: f64 = ctx.node_voltage(nodes[1]);
        let v1257: f64 = (if (v905 > v304) { v51 } else { v304 });
        let v1259: f64 = (self.scalar_v337 * (v905 * v1257));
        let v1260: f64 = (v51 + v1259);
        let v1261: f64 = (v1259 / v1260);
        let v1266: f64 = (self.scalar_v1262 * (v51 + (v925 * self.scalar_v1263)));
        let v1270: f64 = ((self.scalar_v333 * v345) / 1.44);
        let v1272: f64 = (self.scalar_v1267 * { let limexp_arg = v1270; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } });
        let v1274: f64 = (self.scalar_v338 + (v1261 * v1261));
        let v1277: f64 = (v51 + (v1257 * (v1272 * v1274)));
        let v1278: f64 = (v1266 * v1277);
        let v1281: f64 = (v905 * v1278);
        let v1283: f64 = ((self.scalar_v989 * (v279 * v428)) + (v1281 / v946));
        let v1285: f64 = (self.scalar_v1084 * (v279 * v488));
        let v1292: f64 = (((v284 * v671) + (v911 * self.scalar_v1287)) + (v1165 * self.scalar_v1290));
        let v1293: f64 = (v1168 * self.scalar_v1290);
        let v1296: f64 = ((v286 * v816) + ((if self.scalar_v985 { v304 } else { (if self.scalar_v949 { (v84 * v964) } else { v304 }) }) * self.scalar_v1287));
        let v1300: f64 = ((v291 * v900) + (v825 * self.scalar_v1298));
        let v1303: f64 = ((v1201 - ctx.node_voltage(nodes[2])) * self.scalar_v1302);
        let v1306: f64 = ((v1201 - ctx.node_voltage(nodes[0])) * self.scalar_v1305);
        let v1344: f64 = (v6 * self.scalar_v1343);
        let v1345: f64 = 8.617342301212761e-5;
        let v1390: f64 = (self.scalar_v1346 * (self.scalar_v47 * f64::powf(v12, self.scalar_v1387)));
        let v1396: f64 = (v11 * v11);
        let v1398: f64 = (v55 * (((v11 * self.scalar_v1392) - (v53 * v1345)) / v1396));
        let v1406: f64 = (self.scalar_v46 * (((v55 * v1390) + (v48 * v1398)) * (self.scalar_v58 * f64::powf(v56, self.scalar_v1402))));
        let v1423: f64 = (((v68 * (self.scalar_v1346 * (self.scalar_v62 * f64::powf(v12, self.scalar_v1407)))) + (v63 * (v68 * (((v11 * self.scalar_v1411) - (v66 * v1345)) / v1396)))) * (self.scalar_v71 * f64::powf(v69, self.scalar_v1420)));
        let v1438: f64 = (self.scalar_v74 * (((v79 * v1390) + (v48 * (v79 * (((v11 * self.scalar_v1425) - (v77 * v1345)) / v1396)))) * (self.scalar_v82 * f64::powf(v80, self.scalar_v1434))));
        let v1544: f64 = (v182 * (((v12 * v1345) - (v11 * self.scalar_v1346)) / (v12 * v12)));
        let v1558: f64 = (((v190 * (((v11 * self.scalar_v1545) - (v188 * v1345)) / v1396)) - (v195 * (((v11 * self.scalar_v1551) - (v193 * v1345)) / v1396))) / v196);
        let v1569: f64 = ((v202 * 0.00025852026903638284) + (v201 * (self.scalar_v1346 / v12)));
        let v1572: f64 = ((((v198 * self.scalar_v1346) + (v12 * ((v197 * v1544) + (v184 * v1558)))) - v1569) - self.scalar_v1571);
        let v1573: f64 = 0.00017234684602425522;
        let v1587: f64 = ((v218 * v1573) + (v208 * ((v185 * ((v209 * (v212 * (((v11 * (-v1572)) - (v210 * v1345)) / v1396))) / (v182 * v215))) / v217)));
        let v1588: f64 = (v1572 + v1587);
        let v1602: f64 = (((v225 * (((v11 * self.scalar_v1589) - (v223 * v1345)) / v1396)) - (v229 * (((v11 * self.scalar_v1595) - (v227 * v1345)) / v1396))) / v230);
        let v1611: f64 = ((((v232 * self.scalar_v1346) + (v12 * ((v231 * v1544) + (v184 * v1602)))) - v1569) - self.scalar_v1610);
        let v1625: f64 = ((v245 * v1573) + (v208 * ((v185 * ((v209 * (v239 * (((v11 * (-v1611)) - (v237 * v1345)) / v1396))) / (v182 * v242))) / v244)));
        let v1626: f64 = (v1611 + v1625);
        let v1640: f64 = (((v252 * (((v11 * self.scalar_v1627) - (v250 * v1345)) / v1396)) - (v256 * (((v11 * self.scalar_v1633) - (v254 * v1345)) / v1396))) / v257);
        let v1649: f64 = ((((v259 * self.scalar_v1346) + (v12 * ((v258 * v1544) + (v184 * v1640)))) - v1569) - self.scalar_v1648);
        let v1663: f64 = ((v272 * v1573) + (v208 * ((v185 * ((v209 * (v266 * (((v11 * (-v1649)) - (v264 * v1345)) / v1396))) / (v182 * v269))) / v271)));
        let v1664: f64 = (v1649 + v1663);
        let v1667: f64 = (v220 * v220);
        let v1673: f64 = (self.scalar_v275 * (((-(self.scalar_v186 * v1588)) / v1667) * (self.scalar_v277 * f64::powf(v276, self.scalar_v1669))));
        let v1676: f64 = (v247 * v247);
        let v1680: f64 = (((-(self.scalar_v221 * v1626)) / v1676) * (self.scalar_v282 * f64::powf(v281, self.scalar_v1139)));
        let v1685: f64 = (v274 * v274);
        let v1695: f64 = ((v293 * v1398) + (v55 * (self.scalar_v292 * v1390)));
        let v1719: f64 = (-v1588);
        let v1720: f64 = (self.scalar_v351 * v1719);
        let v1721: f64 = (if self.scalar_v354 { v1720 } else { v304 });
        let v1734: f64 = (self.scalar_v1728 / v220);
        let v1757: f64 = (-(v51 / v220));
        let v1758: f64 = (-(v360 / v220));
        let v1761: f64 = (self.scalar_v368 * f64::powf(v381, self.scalar_v1759));
        let v1776: f64 = (if v379 { (((v383 * v1588) + (v220 * (-((-((-(v341 * v1588)) / v1667)) * v1761)))) / self.scalar_v368) } else { (if v358 { ((v366 * v1588) / self.scalar_v368) } else { v304 }) });
        let v1777: f64 = (if v379 { ((v220 * (-(v1757 * v1761))) / self.scalar_v368) } else { v304 });
        let v1778: f64 = (if v379 { ((v220 * (-(v1758 * v1761))) / self.scalar_v368) } else { v304 });
        let v1779: f64 = (if v379 { v304 } else { (if v358 { (v363 * ((v374 * v1721) + (v356 * (((v220 * (self.scalar_v371 * v1721)) - (v372 * v1588)) / v1667)))) } else { v304 }) });
        let v1780: f64 = (if v379 { v304 } else { (if v358 { (v363 * ((v374 * self.scalar_v1722) + (v356 * v1734))) } else { v304 }) });
        let v1781: f64 = (if v379 { v304 } else { (if v358 { (v363 * ((v374 * self.scalar_v1723) + (v356 * (self.scalar_v1729 / v220)))) } else { v304 }) });
        let v1788: f64 = (v352 * v1720);
        let v1791: f64 = ((v1788 + v1788) / (v182 * v395));
        let v1792: f64 = (if self.scalar_v390 { v1791 } else { v304 });
        let v1795: f64 = (if self.scalar_v390 { (v191 * (v1720 + v1792)) } else { v304 });
        let v1808: f64 = (if self.scalar_v390 { (((v402 * v1719) + (v350 * ((-(((v220 * v1795) - (v399 * v1588)) / v1667)) * (self.scalar_v368 * f64::powf(v401, self.scalar_v1759))))) / self.scalar_v368) } else { v304 });
        let v1809: f64 = (if self.scalar_v390 { v1720 } else { v304 });
        let v1812: f64 = (v406 * v1809);
        let v1814: f64 = (v406 * self.scalar_v1810);
        let v1816: f64 = (v406 * self.scalar_v1811);
        let v1818: f64 = (v182 * v409);
        let v1822: f64 = (if self.scalar_v390 { ((v1812 + v1812) / v1818) } else { v304 });
        let v1823: f64 = (if self.scalar_v390 { ((v1814 + v1814) / v1818) } else { v304 });
        let v1824: f64 = (if self.scalar_v390 { ((v1816 + v1816) / v1818) } else { v304 });
        let v1832: f64 = (if self.scalar_v390 { ((v185 * (v1809 - v1822)) - v1720) } else { v304 });
        let v1833: f64 = (if self.scalar_v390 { (v185 * (self.scalar_v1810 - v1823)) } else { v304 });
        let v1834: f64 = (if self.scalar_v390 { (v185 * (self.scalar_v1811 - v1824)) } else { v304 });
        let v1845: f64 = (self.scalar_v368 * f64::powf(v416, self.scalar_v1759));
        let v1857: f64 = (if self.scalar_v390 { (((v417 * v1719) + (v350 * ((-(((v220 * v1832) - (v414 * v1588)) / v1667)) * v1845))) / self.scalar_v368) } else { v1776 });
        let v1858: f64 = (if self.scalar_v390 { ((v350 * ((-(v1833 / v220)) * v1845)) / self.scalar_v368) } else { v1777 });
        let v1859: f64 = (if self.scalar_v390 { ((v350 * ((-(v1834 / v220)) * v1845)) / self.scalar_v368) } else { v1778 });
        let v1871: f64 = (if self.scalar_v390 { ((v1857 + (self.scalar_v422 * (v1795 + (-v1832)))) - v1808) } else { (if self.scalar_v354 { (v1776 + v1779) } else { v304 }) });
        let v1872: f64 = (if self.scalar_v390 { (v1858 + (self.scalar_v422 * (v51 - v1833))) } else { (if self.scalar_v354 { (v1777 + v1780) } else { v304 }) });
        let v1873: f64 = (if self.scalar_v390 { (v1859 + (self.scalar_v422 * (v360 - v1834))) } else { (if self.scalar_v354 { (v1778 + v1781) } else { v304 }) });
        let v1874: f64 = (if self.scalar_v354 { v1720 } else { v1721 });
        let v1916: f64 = (self.scalar_v368 * f64::powf(v449, self.scalar_v1759));
        let v1931: f64 = (if v447 { (((v451 * v1588) + (v220 * (-((-((-(v343 * v1588)) / v1667)) * v1916)))) / self.scalar_v368) } else { (if v432 { ((v436 * v1588) / self.scalar_v368) } else { v1857 }) });
        let v1932: f64 = (if v447 { ((v220 * (-(v1757 * v1916))) / self.scalar_v368) } else { v304 });
        let v1933: f64 = (if v447 { v304 } else { (if v432 { v304 } else { v1858 }) });
        let v1934: f64 = (if v447 { ((v220 * (-(v1758 * v1916))) / self.scalar_v368) } else { (if v432 { v304 } else { v1859 }) });
        let v1935: f64 = (if v447 { v304 } else { (if v432 { (v433 * ((v442 * v1874) + (v430 * (((v220 * (self.scalar_v371 * v1874)) - (v440 * v1588)) / v1667)))) } else { v1779 }) });
        let v1936: f64 = (if v447 { v304 } else { (if v432 { (v433 * ((v442 * self.scalar_v1722) + (v430 * v1734))) } else { v304 }) });
        let v1937: f64 = (if v447 { v304 } else { (if v432 { (v433 * ((v442 * self.scalar_v1875) + (v430 * (self.scalar_v1883 / v220)))) } else { v1780 }) });
        let v1938: f64 = (if v447 { v304 } else { (if v432 { (v433 * ((v442 * self.scalar_v1876) + (v430 * (self.scalar_v1884 / v220)))) } else { v1781 }) });
        let v1947: f64 = (if self.scalar_v390 { v1791 } else { v1792 });
        let v1950: f64 = (if self.scalar_v390 { (v191 * (v1720 + v1947)) } else { v1795 });
        let v1963: f64 = (if self.scalar_v390 { (((v464 * v1719) + (v350 * ((-(((v220 * v1950) - (v461 * v1588)) / v1667)) * (self.scalar_v368 * f64::powf(v463, self.scalar_v1759))))) / self.scalar_v368) } else { v1808 });
        let v1964: f64 = (if self.scalar_v390 { v1720 } else { v1809 });
        let v1967: f64 = (v468 * v1964);
        let v1969: f64 = (v468 * self.scalar_v1810);
        let v1971: f64 = (v468 * self.scalar_v1965);
        let v1973: f64 = (v468 * self.scalar_v1966);
        let v1975: f64 = (v182 * v471);
        let v1980: f64 = (if self.scalar_v390 { ((v1967 + v1967) / v1975) } else { v1822 });
        let v1981: f64 = (if self.scalar_v390 { ((v1969 + v1969) / v1975) } else { v304 });
        let v1982: f64 = (if self.scalar_v390 { ((v1971 + v1971) / v1975) } else { v1823 });
        let v1983: f64 = (if self.scalar_v390 { ((v1973 + v1973) / v1975) } else { v1824 });
        let v1993: f64 = (if self.scalar_v390 { ((v185 * (v1964 - v1980)) - v1720) } else { v1832 });
        let v1994: f64 = (if self.scalar_v390 { (v185 * (self.scalar_v1810 - v1981)) } else { v304 });
        let v1995: f64 = (if self.scalar_v390 { (v185 * (self.scalar_v1965 - v1982)) } else { v1833 });
        let v1996: f64 = (if self.scalar_v390 { (v185 * (self.scalar_v1966 - v1983)) } else { v1834 });
        let v2009: f64 = (self.scalar_v368 * f64::powf(v478, self.scalar_v1759));
        let v2024: f64 = (if self.scalar_v390 { (((v479 * v1719) + (v350 * ((-(((v220 * v1993) - (v476 * v1588)) / v1667)) * v2009))) / self.scalar_v368) } else { v1931 });
        let v2025: f64 = (if self.scalar_v390 { ((v350 * ((-(v1994 / v220)) * v2009)) / self.scalar_v368) } else { v1932 });
        let v2026: f64 = (if self.scalar_v390 { ((v350 * ((-(v1995 / v220)) * v2009)) / self.scalar_v368) } else { v1933 });
        let v2027: f64 = (if self.scalar_v390 { ((v350 * ((-(v1996 / v220)) * v2009)) / self.scalar_v368) } else { v1934 });
        let v2046: f64 = (-v1626);
        let v2047: f64 = (self.scalar_v351 * v2046);
        let v2048: f64 = (if self.scalar_v492 { v2047 } else { v1874 });
        let v2068: f64 = (self.scalar_v2060 / v247);
        let v2099: f64 = ((-(self.scalar_v514 * v1626)) / v1676);
        let v2103: f64 = (v2099 * (self.scalar_v504 * f64::powf(v523, self.scalar_v2100)));
        let v2107: f64 = (v527 * v527);
        let v2128: f64 = ((v247 * (-(v524 * (-(self.scalar_v2104 / v527))))) / self.scalar_v504);
        let v2129: f64 = ((v247 * (-(v524 * (-(self.scalar_v504 / v527))))) / self.scalar_v504);
        let v2130: f64 = (if v521 { (((v531 * v1626) + (v247 * (-((v529 * v2103) + (v524 * (-((-(v526 * v1626)) / v2107))))))) / self.scalar_v504) } else { (if v496 { ((v502 * v1626) / self.scalar_v504) } else { v2024 }) });
        let v2141: f64 = (-(v360 / v247));
        let v2142: f64 = (-(v51 / v247));
        let v2144: f64 = (self.scalar_v504 * f64::powf(v538, self.scalar_v2100));
        let v2159: f64 = (if v536 { (((v540 * v1626) + (v247 * (-((-((-(v345 * v1626)) / v1676)) * v2144)))) / self.scalar_v504) } else { v2130 });
        let v2160: f64 = (if v536 { ((v247 * (-(v2141 * v2144))) / self.scalar_v504) } else { (if v521 { v2128 } else { v304 }) });
        let v2161: f64 = (if v536 { v304 } else { (if v521 { v304 } else { (if v496 { v304 } else { v2025 }) }) });
        let v2162: f64 = (if v536 { ((v247 * (-(v2142 * v2144))) / self.scalar_v504) } else { (if v521 { v2129 } else { (if v496 { v304 } else { v2026 }) }) });
        let v2163: f64 = (if v536 { v304 } else { (if v521 { v304 } else { (if v496 { v304 } else { v2027 }) }) });
        let v2164: f64 = (if v520 { v304 } else { (if v496 { (v499 * ((v510 * v2048) + (v494 * (((v247 * (self.scalar_v507 * v2048)) - (v508 * v1626)) / v1676)))) } else { v1935 }) });
        let v2165: f64 = (if v520 { v304 } else { (if v496 { (v499 * ((v510 * self.scalar_v2049) + (v494 * v2068))) } else { v304 }) });
        let v2166: f64 = (if v520 { v304 } else { (if v496 { (v499 * ((v510 * self.scalar_v2050) + (v494 * (self.scalar_v2061 / v247)))) } else { v1936 }) });
        let v2167: f64 = (if v520 { v304 } else { (if v496 { (v499 * ((v510 * self.scalar_v2051) + (v494 * (self.scalar_v2062 / v247)))) } else { v1937 }) });
        let v2168: f64 = (if v520 { v304 } else { (if v496 { (v499 * ((v510 * self.scalar_v2052) + (v494 * (self.scalar_v2063 / v247)))) } else { v1938 }) });
        let v2179: f64 = (-v2047);
        let v2180: f64 = (v553 * v2047);
        let v2183: f64 = (v553 * v553);
        let v2184: f64 = ((v2180 - (v552 * v2179)) / v2183);
        let v2185: f64 = (if self.scalar_v551 { v2184 } else { v304 });
        let v2187: f64 = (v557 * v2185);
        let v2191: f64 = (v563 * v2185);
        let v2200: f64 = (((v569 * (v182 * v2185)) - (v556 * (((v2187 + v2187) / (v182 * v562)) + ((v2191 + v2191) / (v182 * v568))))) / (v569 * v569));
        let v2201: f64 = (if self.scalar_v551 { v2200 } else { v304 });
        let v2207: f64 = (if self.scalar_v551 { (v185 * (((v571 * v2179) + (v553 * v2201)) - v2047)) } else { v1950 });
        let v2220: f64 = (((v580 * v1626) + (v247 * (-((-(((v247 * v2207) - (v576 * v1626)) / v1676)) * (self.scalar_v504 * f64::powf(v578, self.scalar_v2100)))))) / self.scalar_v504);
        let v2221: f64 = (if self.scalar_v551 { v2220 } else { v304 });
        let v2228: f64 = (if self.scalar_v551 { ((v2180 - (v586 * v2179)) / v2183) } else { v304 });
        let v2229: f64 = (if self.scalar_v551 { (-2.0 / v553) } else { v304 });
        let v2230: f64 = (if self.scalar_v551 { (v182 / v553) } else { v304 });
        let v2232: f64 = (v182 * v2229);
        let v2233: f64 = (v182 * v2230);
        let v2234: f64 = (v590 * v2228);
        let v2236: f64 = (v590 * v2229);
        let v2238: f64 = (v590 * v2230);
        let v2240: f64 = (v182 * v593);
        let v2244: f64 = (v594 * v2228);
        let v2246: f64 = (v594 * v2229);
        let v2248: f64 = (v594 * v2230);
        let v2250: f64 = (v182 * v597);
        let v2260: f64 = (v598 * v598);
        let v2270: f64 = (if self.scalar_v551 { (((v598 * (v182 * v2228)) - (v589 * (((v2234 + v2234) / v2240) + ((v2244 + v2244) / v2250)))) / v2260) } else { v304 });
        let v2271: f64 = (if self.scalar_v551 { (((v598 * v2232) - (v589 * (((v2236 + v2236) / v2240) + ((v2246 + v2246) / v2250)))) / v2260) } else { v304 });
        let v2272: f64 = (if self.scalar_v551 { (((v598 * v2233) - (v589 * (((v2238 + v2238) / v2240) + ((v2248 + v2248) / v2250)))) / v2260) } else { v304 });
        let v2282: f64 = (if self.scalar_v551 { (v185 * (((v600 * v2179) + (v553 * v2270)) - v2047)) } else { v1993 });
        let v2283: f64 = (if self.scalar_v551 { (v185 * (v553 * v2271)) } else { v304 });
        let v2284: f64 = (if self.scalar_v551 { v304 } else { v1994 });
        let v2285: f64 = (if self.scalar_v551 { (v185 * (v553 * v2272)) } else { v1995 });
        let v2286: f64 = (if self.scalar_v551 { v304 } else { v1996 });
        let v2301: f64 = (self.scalar_v504 * f64::powf(v607, self.scalar_v2100));
        let v2324: f64 = (if self.scalar_v551 { (((v609 * v1626) + (v247 * (-((-(((v247 * v2282) - (v605 * v1626)) / v1676)) * v2301)))) / self.scalar_v504) } else { v2159 });
        let v2325: f64 = (if self.scalar_v551 { ((v247 * (-((-(v2283 / v247)) * v2301))) / self.scalar_v504) } else { v2160 });
        let v2326: f64 = (if self.scalar_v551 { ((v247 * (-((-(v2284 / v247)) * v2301))) / self.scalar_v504) } else { v2161 });
        let v2327: f64 = (if self.scalar_v551 { ((v247 * (-((-(v2285 / v247)) * v2301))) / self.scalar_v504) } else { v2162 });
        let v2328: f64 = (if self.scalar_v551 { ((v247 * (-((-(v2286 / v247)) * v2301))) / self.scalar_v504) } else { v2163 });
        let v2332: f64 = (if self.scalar_v551 { (v185 * v2270) } else { v304 });
        let v2333: f64 = (if self.scalar_v551 { (v185 * v2271) } else { v304 });
        let v2334: f64 = (if self.scalar_v551 { (v185 * v2272) } else { v304 });
        let v2338: f64 = (v2099 * (self.scalar_v616 * f64::powf(v523, self.scalar_v2335)));
        let v2339: f64 = (if self.scalar_v551 { v2338 } else { v304 });
        let v2346: f64 = ((((v247 * v2047) - (v490 * v1626)) / v1676) * (self.scalar_v616 * f64::powf(v620, self.scalar_v2335)));
        let v2347: f64 = (if self.scalar_v551 { v2346 } else { v304 });
        let v2364: f64 = (if self.scalar_v551 { (((v623 * v2339) + (v618 * (-v2332))) + ((v622 * v2332) + (v615 * v2347))) } else { v304 });
        let v2365: f64 = (if self.scalar_v551 { ((v618 * (-v2333)) + (v622 * v2333)) } else { v304 });
        let v2366: f64 = (if self.scalar_v551 { ((v618 * (-v2334)) + (v622 * v2334)) } else { v304 });
        let v2384: f64 = (if self.scalar_v551 { ((v629 * v2364) + (v627 * (v2207 + (-v2282)))) } else { v304 });
        let v2385: f64 = (if self.scalar_v551 { ((v629 * v2365) + (v627 * (v360 - v2283))) } else { v304 });
        let v2386: f64 = (if self.scalar_v551 { (v627 * (-v2284)) } else { v304 });
        let v2387: f64 = (if self.scalar_v551 { ((v629 * v2366) + (v627 * (v51 - v2285))) } else { v304 });
        let v2388: f64 = (if self.scalar_v551 { (v627 * (-v2286)) } else { v304 });
        let v2400: f64 = (v490 * v2047);
        let v2403: f64 = ((v2400 + v2400) / (v182 * v639));
        let v2404: f64 = (if self.scalar_v636 { v2403 } else { v1947 });
        let v2407: f64 = (if self.scalar_v636 { (v191 * (v2047 + v2404)) } else { v2207 });
        let v2420: f64 = (if self.scalar_v636 { (((v646 * v2046) + (v489 * ((-(((v247 * v2407) - (v643 * v1626)) / v1676)) * (self.scalar_v504 * f64::powf(v645, self.scalar_v2100))))) / self.scalar_v504) } else { v1963 });
        let v2421: f64 = (if self.scalar_v636 { v2047 } else { v1964 });
        let v2426: f64 = (v650 * v2421);
        let v2428: f64 = (v650 * self.scalar_v2422);
        let v2430: f64 = (v650 * self.scalar_v2423);
        let v2432: f64 = (v650 * self.scalar_v2424);
        let v2434: f64 = (v650 * self.scalar_v2425);
        let v2436: f64 = (v182 * v653);
        let v2442: f64 = (if self.scalar_v636 { ((v2426 + v2426) / v2436) } else { v1980 });
        let v2443: f64 = (if self.scalar_v636 { ((v2428 + v2428) / v2436) } else { v304 });
        let v2444: f64 = (if self.scalar_v636 { ((v2430 + v2430) / v2436) } else { v1981 });
        let v2445: f64 = (if self.scalar_v636 { ((v2432 + v2432) / v2436) } else { v1982 });
        let v2446: f64 = (if self.scalar_v636 { ((v2434 + v2434) / v2436) } else { v1983 });
        let v2458: f64 = (if self.scalar_v636 { ((v185 * (v2421 - v2442)) - v2047) } else { v2282 });
        let v2459: f64 = (if self.scalar_v636 { (v185 * (self.scalar_v2422 - v2443)) } else { v2283 });
        let v2460: f64 = (if self.scalar_v636 { (v185 * (self.scalar_v2423 - v2444)) } else { v2284 });
        let v2461: f64 = (if self.scalar_v636 { (v185 * (self.scalar_v2424 - v2445)) } else { v2285 });
        let v2462: f64 = (if self.scalar_v636 { (v185 * (self.scalar_v2425 - v2446)) } else { v2286 });
        let v2477: f64 = (self.scalar_v504 * f64::powf(v660, self.scalar_v2100));
        let v2495: f64 = (if self.scalar_v636 { (((v661 * v2046) + (v489 * ((-(((v247 * v2458) - (v658 * v1626)) / v1676)) * v2477))) / self.scalar_v504) } else { v2324 });
        let v2496: f64 = (if self.scalar_v636 { ((v489 * ((-(v2459 / v247)) * v2477)) / self.scalar_v504) } else { v2325 });
        let v2497: f64 = (if self.scalar_v636 { ((v489 * ((-(v2460 / v247)) * v2477)) / self.scalar_v504) } else { v2326 });
        let v2498: f64 = (if self.scalar_v636 { ((v489 * ((-(v2461 / v247)) * v2477)) / self.scalar_v504) } else { v2327 });
        let v2499: f64 = (if self.scalar_v636 { ((v489 * ((-(v2462 / v247)) * v2477)) / self.scalar_v504) } else { v2328 });
        let v2517: f64 = (if self.scalar_v636 { ((v2495 + (self.scalar_v665 * (v2407 + (-v2458)))) - v2420) } else { (if self.scalar_v551 { ((v2324 + v2384) - v2221) } else { (if self.scalar_v492 { (v2159 + v2164) } else { v304 }) }) });
        let v2518: f64 = (if self.scalar_v636 { (v2496 + (self.scalar_v665 * (v360 - v2459))) } else { (if self.scalar_v551 { (v2325 + v2385) } else { (if self.scalar_v492 { (v2160 + v2165) } else { v304 }) }) });
        let v2519: f64 = (if self.scalar_v636 { (v2497 + (self.scalar_v665 * (-v2460))) } else { (if self.scalar_v551 { (v2326 + v2386) } else { (if self.scalar_v492 { (v2161 + v2166) } else { v304 }) }) });
        let v2520: f64 = (if self.scalar_v636 { (v2498 + (self.scalar_v665 * (v51 - v2461))) } else { (if self.scalar_v551 { (v2327 + v2387) } else { (if self.scalar_v492 { (v2162 + v2167) } else { v304 }) }) });
        let v2521: f64 = (if self.scalar_v636 { (v2499 + (self.scalar_v665 * (-v2462))) } else { (if self.scalar_v551 { (v2328 + v2388) } else { (if self.scalar_v492 { (v2163 + v2168) } else { v304 }) }) });
        let v2522: f64 = (if self.scalar_v492 { v2047 } else { v2048 });
        let v2589: f64 = (if v693 { (((v699 * v1626) + (v247 * (-((v697 * v2103) + (v524 * (-((-(v695 * v1626)) / v2107))))))) / self.scalar_v504) } else { (if v675 { ((v679 * v1626) / self.scalar_v504) } else { v2495 }) });
        let v2600: f64 = (self.scalar_v504 * f64::powf(v706, self.scalar_v2100));
        let v2615: f64 = (if v704 { (((v708 * v1626) + (v247 * (-((-((-(v349 * v1626)) / v1676)) * v2600)))) / self.scalar_v504) } else { v2589 });
        let v2616: f64 = (if v704 { v304 } else { (if v693 { v304 } else { (if v675 { v304 } else { v2496 }) }) });
        let v2617: f64 = (if v704 { ((v247 * (-(v2142 * v2600))) / self.scalar_v504) } else { (if v693 { v2129 } else { (if v675 { v304 } else { v2497 }) }) });
        let v2618: f64 = (if v704 { v304 } else { (if v693 { v304 } else { (if v675 { v304 } else { v2498 }) }) });
        let v2619: f64 = (if v704 { v304 } else { (if v693 { v304 } else { (if v675 { v304 } else { v2499 }) }) });
        let v2620: f64 = (if v704 { ((v247 * (-(v2141 * v2600))) / self.scalar_v504) } else { (if v693 { v2128 } else { v304 }) });
        let v2621: f64 = (if v692 { v304 } else { (if v675 { (v676 * ((v685 * v2522) + (v673 * (((v247 * (self.scalar_v507 * v2522)) - (v683 * v1626)) / v1676)))) } else { v2164 }) });
        let v2622: f64 = (if v692 { v304 } else { (if v675 { (v676 * ((v685 * self.scalar_v2523) + (v673 * (self.scalar_v2535 / v247)))) } else { v2165 }) });
        let v2623: f64 = (if v692 { v304 } else { (if v675 { (v676 * ((v685 * self.scalar_v2524) + (v673 * (self.scalar_v2536 / v247)))) } else { v2166 }) });
        let v2624: f64 = (if v692 { v304 } else { (if v675 { (v676 * ((v685 * self.scalar_v2525) + (v673 * (self.scalar_v2537 / v247)))) } else { v2167 }) });
        let v2625: f64 = (if v692 { v304 } else { (if v675 { (v676 * ((v685 * self.scalar_v2526) + (v673 * (self.scalar_v2538 / v247)))) } else { v2168 }) });
        let v2626: f64 = (if v692 { v304 } else { (if v675 { (v676 * ((v685 * self.scalar_v2049) + (v673 * v2068))) } else { v304 }) });
        let v2639: f64 = (if self.scalar_v551 { v2184 } else { v2185 });
        let v2641: f64 = (v717 * v2639);
        let v2645: f64 = (v721 * v2639);
        let v2654: f64 = (((v725 * (v182 * v2639)) - (v716 * (((v2641 + v2641) / (v182 * v720)) + ((v2645 + v2645) / (v182 * v724))))) / (v725 * v725));
        let v2661: f64 = (if self.scalar_v551 { (v185 * (((v727 * v2179) + (v553 * (if self.scalar_v551 { v2654 } else { v2201 }))) - v2047)) } else { v2407 });
        let v2674: f64 = (((v736 * v1626) + (v247 * (-((-(((v247 * v2661) - (v732 * v1626)) / v1676)) * (self.scalar_v504 * f64::powf(v734, self.scalar_v2100)))))) / self.scalar_v504);
        let v2679: f64 = (if self.scalar_v551 { ((v2180 - (v742 * v2179)) / v2183) } else { v2228 });
        let v2680: f64 = (if self.scalar_v551 { v304 } else { v2229 });
        let v2681: f64 = (if self.scalar_v551 { v304 } else { v2230 });
        let v2685: f64 = (v746 * v2679);
        let v2687: f64 = (v746 * v2680);
        let v2689: f64 = (v746 * v2230);
        let v2691: f64 = (v746 * v2681);
        let v2693: f64 = (v746 * v2229);
        let v2695: f64 = (v182 * v749);
        let v2701: f64 = (v750 * v2679);
        let v2703: f64 = (v750 * v2680);
        let v2705: f64 = (v750 * v2230);
        let v2707: f64 = (v750 * v2681);
        let v2709: f64 = (v750 * v2229);
        let v2711: f64 = (v182 * v753);
        let v2725: f64 = (v754 * v754);
        let v2743: f64 = (if self.scalar_v551 { (((v754 * (v182 * v2679)) - (v745 * (((v2685 + v2685) / v2695) + ((v2701 + v2701) / v2711)))) / v2725) } else { v2270 });
        let v2744: f64 = (if self.scalar_v551 { (((v754 * (v182 * v2680)) - (v745 * (((v2687 + v2687) / v2695) + ((v2703 + v2703) / v2711)))) / v2725) } else { v2271 });
        let v2745: f64 = (if self.scalar_v551 { (((v754 * v2233) - (v745 * (((v2689 + v2689) / v2695) + ((v2705 + v2705) / v2711)))) / v2725) } else { v304 });
        let v2746: f64 = (if self.scalar_v551 { (((v754 * (v182 * v2681)) - (v745 * (((v2691 + v2691) / v2695) + ((v2707 + v2707) / v2711)))) / v2725) } else { v2272 });
        let v2747: f64 = (if self.scalar_v551 { (((v754 * v2232) - (v745 * (((v2693 + v2693) / v2695) + ((v2709 + v2709) / v2711)))) / v2725) } else { v304 });
        let v2761: f64 = (if self.scalar_v551 { (v185 * (((v756 * v2179) + (v553 * v2743)) - v2047)) } else { v2458 });
        let v2762: f64 = (if self.scalar_v551 { (v185 * (v553 * v2744)) } else { v2459 });
        let v2763: f64 = (if self.scalar_v551 { (v185 * (v553 * v2745)) } else { v2460 });
        let v2764: f64 = (if self.scalar_v551 { (v185 * (v553 * v2746)) } else { v2461 });
        let v2765: f64 = (if self.scalar_v551 { v304 } else { v2462 });
        let v2766: f64 = (if self.scalar_v551 { (v185 * (v553 * v2747)) } else { v304 });
        let v2783: f64 = (self.scalar_v504 * f64::powf(v763, self.scalar_v2100));
        let v2810: f64 = (if self.scalar_v551 { (((v765 * v1626) + (v247 * (-((-(((v247 * v2761) - (v761 * v1626)) / v1676)) * v2783)))) / self.scalar_v504) } else { v2615 });
        let v2811: f64 = (if self.scalar_v551 { ((v247 * (-((-(v2762 / v247)) * v2783))) / self.scalar_v504) } else { v2616 });
        let v2812: f64 = (if self.scalar_v551 { ((v247 * (-((-(v2763 / v247)) * v2783))) / self.scalar_v504) } else { v2617 });
        let v2813: f64 = (if self.scalar_v551 { ((v247 * (-((-(v2764 / v247)) * v2783))) / self.scalar_v504) } else { v2618 });
        let v2814: f64 = (if self.scalar_v551 { ((v247 * (-((-(v2765 / v247)) * v2783))) / self.scalar_v504) } else { v2619 });
        let v2815: f64 = (if self.scalar_v551 { ((v247 * (-((-(v2766 / v247)) * v2783))) / self.scalar_v504) } else { v2620 });
        let v2821: f64 = (if self.scalar_v551 { (v185 * v2743) } else { v2332 });
        let v2822: f64 = (if self.scalar_v551 { (v185 * v2744) } else { v2333 });
        let v2823: f64 = (if self.scalar_v551 { (v185 * v2745) } else { v304 });
        let v2824: f64 = (if self.scalar_v551 { (v185 * v2746) } else { v2334 });
        let v2825: f64 = (if self.scalar_v551 { (v185 * v2747) } else { v304 });
        let v2852: f64 = (if self.scalar_v551 { (((v774 * (if self.scalar_v551 { v2338 } else { v2339 })) + (v772 * (-v2821))) + ((v773 * v2821) + (v771 * (if self.scalar_v551 { v2346 } else { v2347 })))) } else { v2364 });
        let v2893: f64 = (if self.scalar_v551 { ((v2810 + (if self.scalar_v551 { ((v780 * v2852) + (v778 * (v2661 + (-v2761)))) } else { v2384 })) - (if self.scalar_v551 { v2674 } else { v2221 })) } else { (if self.scalar_v492 { (v2615 + v2621) } else { v304 }) });
        let v2894: f64 = (if self.scalar_v551 { (v2811 + (if self.scalar_v551 { ((v780 * (if self.scalar_v551 { ((v772 * (-v2822)) + (v773 * v2822)) } else { v2365 })) + (v778 * (-v2762))) } else { v2385 })) } else { (if self.scalar_v492 { (v2616 + v2622) } else { v304 }) });
        let v2895: f64 = (if self.scalar_v551 { (v2812 + (if self.scalar_v551 { ((v780 * (if self.scalar_v551 { ((v772 * (-v2823)) + (v773 * v2823)) } else { v304 })) + (v778 * (v51 - v2763))) } else { v2386 })) } else { (if self.scalar_v492 { (v2617 + v2623) } else { v304 }) });
        let v2896: f64 = (if self.scalar_v551 { (v2813 + (if self.scalar_v551 { ((v780 * (if self.scalar_v551 { ((v772 * (-v2824)) + (v773 * v2824)) } else { v2366 })) + (v778 * (-v2764))) } else { v2387 })) } else { (if self.scalar_v492 { (v2618 + v2624) } else { v304 }) });
        let v2898: f64 = (if self.scalar_v551 { (v2815 + (if self.scalar_v551 { ((v780 * (if self.scalar_v551 { ((v772 * (-v2825)) + (v773 * v2825)) } else { v304 })) + (v778 * (v360 - v2766))) } else { v304 })) } else { (if self.scalar_v492 { (v2620 + v2626) } else { v304 }) });
        let v2899: f64 = (if self.scalar_v636 { v2403 } else { v2404 });
        let v2902: f64 = (if self.scalar_v636 { (v191 * (v2047 + v2899)) } else { v2661 });
        let v2915: f64 = (if self.scalar_v636 { (((v792 * v2046) + (v489 * ((-(((v247 * v2902) - (v789 * v1626)) / v1676)) * (self.scalar_v504 * f64::powf(v791, self.scalar_v2100))))) / self.scalar_v504) } else { v2420 });
        let v2916: f64 = (if self.scalar_v636 { v2047 } else { v2421 });
        let v2921: f64 = (v796 * v2916);
        let v2923: f64 = (v796 * self.scalar_v2917);
        let v2925: f64 = (v796 * self.scalar_v2918);
        let v2927: f64 = (v796 * self.scalar_v2919);
        let v2929: f64 = (v796 * self.scalar_v2920);
        let v2931: f64 = (v796 * self.scalar_v2422);
        let v2933: f64 = (v182 * v799);
        let v2940: f64 = (if self.scalar_v636 { ((v2921 + v2921) / v2933) } else { v2442 });
        let v2941: f64 = (if self.scalar_v636 { ((v2923 + v2923) / v2933) } else { v2443 });
        let v2942: f64 = (if self.scalar_v636 { ((v2925 + v2925) / v2933) } else { v2444 });
        let v2943: f64 = (if self.scalar_v636 { ((v2927 + v2927) / v2933) } else { v2445 });
        let v2944: f64 = (if self.scalar_v636 { ((v2929 + v2929) / v2933) } else { v2446 });
        let v2945: f64 = (if self.scalar_v636 { ((v2931 + v2931) / v2933) } else { v304 });
        let v2959: f64 = (if self.scalar_v636 { ((v185 * (v2916 - v2940)) - v2047) } else { v2761 });
        let v2960: f64 = (if self.scalar_v636 { (v185 * (self.scalar_v2917 - v2941)) } else { v2762 });
        let v2961: f64 = (if self.scalar_v636 { (v185 * (self.scalar_v2918 - v2942)) } else { v2763 });
        let v2962: f64 = (if self.scalar_v636 { (v185 * (self.scalar_v2919 - v2943)) } else { v2764 });
        let v2963: f64 = (if self.scalar_v636 { (v185 * (self.scalar_v2920 - v2944)) } else { v2765 });
        let v2964: f64 = (if self.scalar_v636 { (v185 * (self.scalar_v2422 - v2945)) } else { v2766 });
        let v2981: f64 = (self.scalar_v504 * f64::powf(v806, self.scalar_v2100));
        let v3002: f64 = (if self.scalar_v636 { (((v807 * v2046) + (v489 * ((-(((v247 * v2959) - (v804 * v1626)) / v1676)) * v2981))) / self.scalar_v504) } else { v2810 });
        let v3003: f64 = (if self.scalar_v636 { ((v489 * ((-(v2960 / v247)) * v2981)) / self.scalar_v504) } else { v2811 });
        let v3004: f64 = (if self.scalar_v636 { ((v489 * ((-(v2961 / v247)) * v2981)) / self.scalar_v504) } else { v2812 });
        let v3005: f64 = (if self.scalar_v636 { ((v489 * ((-(v2962 / v247)) * v2981)) / self.scalar_v504) } else { v2813 });
        let v3006: f64 = (if self.scalar_v636 { ((v489 * ((-(v2963 / v247)) * v2981)) / self.scalar_v504) } else { v2814 });
        let v3007: f64 = (if self.scalar_v636 { ((v489 * ((-(v2964 / v247)) * v2981)) / self.scalar_v504) } else { v2815 });
        let v3032: f64 = (if self.scalar_v636 { (v3006 + (self.scalar_v665 * (-v2963))) } else { (if self.scalar_v551 { (v2814 + (if self.scalar_v551 { (v778 * (-v2765)) } else { v2388 })) } else { (if self.scalar_v492 { (v2619 + v2625) } else { v304 }) }) });
        let v3034: f64 = (-v1664);
        let v3036: f64 = (if self.scalar_v817 { (self.scalar_v351 * v3034) } else { v2047 });
        let v3037: f64 = (if self.scalar_v823 { v3036 } else { v2522 });
        let v3114: f64 = (self.scalar_v837 * f64::powf(v850, self.scalar_v3112));
        let v3129: f64 = (if v848 { (((v852 * v1664) + (v274 * (-((-((-(v825 * v1664)) / v1685)) * v3114)))) / self.scalar_v837) } else { (if v829 { ((v835 * v1664) / self.scalar_v837) } else { v3002 }) });
        let v3130: f64 = (if v848 { v304 } else { (if v829 { v304 } else { v3003 }) });
        let v3131: f64 = (if v848 { v304 } else { (if v829 { v304 } else { v3004 }) });
        let v3132: f64 = (if v848 { v304 } else { (if v829 { v304 } else { v3005 }) });
        let v3133: f64 = (if v848 { v304 } else { (if v829 { v304 } else { v3006 }) });
        let v3134: f64 = (if v848 { ((v274 * (-((-(v360 / v274)) * v3114))) / self.scalar_v837) } else { (if v829 { v304 } else { v3007 }) });
        let v3135: f64 = (if v848 { ((v274 * (-((-(v51 / v274)) * v3114))) / self.scalar_v837) } else { v304 });
        let v3136: f64 = (if v848 { v304 } else { (if v829 { (v832 * ((v843 * v3037) + (v827 * (((v274 * (self.scalar_v840 * v3037)) - (v841 * v1664)) / v1685)))) } else { v2621 }) });
        let v3157: f64 = (v820 * v3036);
        let v3164: f64 = (if self.scalar_v860 { (v191 * (v3036 + (if self.scalar_v860 { ((v3157 + v3157) / (v182 * v865)) } else { v2899 }))) } else { v2902 });
        let v3177: f64 = (if self.scalar_v860 { (((v872 * v3034) + (v818 * ((-(((v274 * v3164) - (v869 * v1664)) / v1685)) * (self.scalar_v837 * f64::powf(v871, self.scalar_v3112))))) / self.scalar_v837) } else { v2915 });
        let v3178: f64 = (if self.scalar_v860 { v3036 } else { v2916 });
        let v3185: f64 = (v876 * v3178);
        let v3187: f64 = (v876 * self.scalar_v3179);
        let v3189: f64 = (v876 * self.scalar_v3180);
        let v3191: f64 = (v876 * self.scalar_v3181);
        let v3193: f64 = (v876 * self.scalar_v3182);
        let v3195: f64 = (v876 * self.scalar_v3183);
        let v3197: f64 = (v876 * self.scalar_v3184);
        let v3199: f64 = (v182 * v879);
        let v3229: f64 = (if self.scalar_v860 { ((v185 * (v3178 - (if self.scalar_v860 { ((v3185 + v3185) / v3199) } else { v2940 }))) - v3036) } else { v2959 });
        let v3230: f64 = (if self.scalar_v860 { (v185 * (self.scalar_v3179 - (if self.scalar_v860 { ((v3187 + v3187) / v3199) } else { v2941 }))) } else { v2960 });
        let v3231: f64 = (if self.scalar_v860 { (v185 * (self.scalar_v3180 - (if self.scalar_v860 { ((v3189 + v3189) / v3199) } else { v2942 }))) } else { v2961 });
        let v3232: f64 = (if self.scalar_v860 { (v185 * (self.scalar_v3181 - (if self.scalar_v860 { ((v3191 + v3191) / v3199) } else { v2943 }))) } else { v2962 });
        let v3233: f64 = (if self.scalar_v860 { (v185 * (self.scalar_v3182 - (if self.scalar_v860 { ((v3193 + v3193) / v3199) } else { v2944 }))) } else { v2963 });
        let v3234: f64 = (if self.scalar_v860 { (v185 * (self.scalar_v3183 - (if self.scalar_v860 { ((v3195 + v3195) / v3199) } else { v2945 }))) } else { v2964 });
        let v3235: f64 = (if self.scalar_v860 { (v185 * (self.scalar_v3184 - (if self.scalar_v860 { ((v3197 + v3197) / v3199) } else { v304 }))) } else { v304 });
        let v3254: f64 = (self.scalar_v837 * f64::powf(v886, self.scalar_v3112));
        let v3300: f64 = ((if self.scalar_v860 { (((v887 * v3034) + (v818 * ((-(((v274 * v3229) - (v884 * v1664)) / v1685)) * v3254))) / self.scalar_v837) } else { v3129 }) + (self.scalar_v892 * (v3164 + (-v3229))));
        let v3309: f64 = (if self.scalar_v860 { ((if self.scalar_v860 { ((v818 * ((-(v3230 / v274)) * v3254)) / self.scalar_v837) } else { v3130 }) + (self.scalar_v892 * (-v3230))) } else { (if self.scalar_v823 { (v3130 + (if v848 { v304 } else { (if v829 { (v832 * ((v843 * self.scalar_v3038) + (v827 * (self.scalar_v3053 / v274)))) } else { v2622 }) })) } else { v304 }) });
        let v3310: f64 = (if self.scalar_v860 { ((if self.scalar_v860 { ((v818 * ((-(v3231 / v274)) * v3254)) / self.scalar_v837) } else { v3131 }) + (self.scalar_v892 * (-v3231))) } else { (if self.scalar_v823 { (v3131 + (if v848 { v304 } else { (if v829 { (v832 * ((v843 * self.scalar_v3039) + (v827 * (self.scalar_v3054 / v274)))) } else { v2623 }) })) } else { v304 }) });
        let v3311: f64 = (if self.scalar_v860 { ((if self.scalar_v860 { ((v818 * ((-(v3232 / v274)) * v3254)) / self.scalar_v837) } else { v3132 }) + (self.scalar_v892 * (-v3232))) } else { (if self.scalar_v823 { (v3132 + (if v848 { v304 } else { (if v829 { (v832 * ((v843 * self.scalar_v3040) + (v827 * (self.scalar_v3055 / v274)))) } else { v2624 }) })) } else { v304 }) });
        let v3312: f64 = (if self.scalar_v860 { ((if self.scalar_v860 { ((v818 * ((-(v3233 / v274)) * v3254)) / self.scalar_v837) } else { v3133 }) + (self.scalar_v892 * (-v3233))) } else { (if self.scalar_v823 { (v3133 + (if v848 { v304 } else { (if v829 { (v832 * ((v843 * self.scalar_v3041) + (v827 * (self.scalar_v3056 / v274)))) } else { v2625 }) })) } else { v304 }) });
        let v3313: f64 = (if self.scalar_v860 { ((if self.scalar_v860 { ((v818 * ((-(v3234 / v274)) * v3254)) / self.scalar_v837) } else { v3134 }) + (self.scalar_v892 * (v360 - v3234))) } else { (if self.scalar_v823 { (v3134 + (if v848 { v304 } else { (if v829 { (v832 * ((v843 * self.scalar_v3042) + (v827 * (self.scalar_v3057 / v274)))) } else { v2626 }) })) } else { v304 }) });
        let v3314: f64 = (if self.scalar_v860 { ((if self.scalar_v860 { ((v818 * ((-(v3235 / v274)) * v3254)) / self.scalar_v837) } else { v3135 }) + (self.scalar_v892 * (v51 - v3235))) } else { (if self.scalar_v823 { (v3135 + (if v848 { v304 } else { (if v829 { (v832 * ((v843 * self.scalar_v3043) + (v827 * (self.scalar_v3058 / v274)))) } else { v304 }) })) } else { v304 }) });
        let v3331: f64 = { let limexp_arg = v902; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v3337: f64 = ((v904 * v1406) + (v60 * (((-(v341 * ((v162 * v1345) + (v11 * self.scalar_v1533)))) / (v901 * v901)) * v3331)));
        let v3338: f64 = (v60 * ((v51 / v901) * v3331));
        let v3339: f64 = (v60 * ((v360 / v901) * v3331));
        let v3346: f64 = ((-(v345 * ((v163 * v1345) + (v11 * self.scalar_v1534)))) / (v906 * v906));
        let v3347: f64 = (v360 / v906);
        let v3348: f64 = (v51 / v906);
        let v3349: f64 = { let limexp_arg = v907; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v3350: f64 = (v3346 * v3349);
        let v3351: f64 = (v3347 * v3349);
        let v3352: f64 = (v3348 * v3349);
        let v3358: f64 = ((v910 * ((v73 * v1406) + (v60 * (self.scalar_v61 * v1423)))) + (v909 * v3350));
        let v3359: f64 = (v909 * v3351);
        let v3360: f64 = (v909 * v3352);
        let v3365: f64 = (self.scalar_v307 * v2518);
        let v3366: f64 = (self.scalar_v307 * v2519);
        let v3369: f64 = ((self.scalar_v311 * v1871) + (self.scalar_v307 * v2517));
        let v3370: f64 = ((self.scalar_v311 * v1872) + (self.scalar_v307 * v2520));
        let v3371: f64 = ((self.scalar_v311 * v1873) + (self.scalar_v307 * v2521));
        let v3372: f64 = (v917 * v3369);
        let v3374: f64 = (v917 * v3365);
        let v3376: f64 = (v917 * v3366);
        let v3378: f64 = (v917 * v3370);
        let v3380: f64 = (v917 * v3371);
        let v3382: f64 = (v182 * v921);
        let v3393: f64 = (v185 * (v3369 + ((v3372 + v3372) / v3382)));
        let v3394: f64 = (v185 * (v3365 + ((v3374 + v3374) / v3382)));
        let v3395: f64 = (v185 * (v3366 + ((v3376 + v3376) / v3382)));
        let v3396: f64 = (v185 * (v3370 + ((v3378 + v3378) / v3382)));
        let v3397: f64 = (v185 * (v3371 + ((v3380 + v3380) / v3382)));
        let v3406: f64 = (((v905 * (if self.scalar_v312 { ((-(self.scalar_v14 * (self.scalar_v1346 * (self.scalar_v15 * f64::powf(v12, self.scalar_v1347))))) / (v17 * v17)) } else { v304 })) + (v314 * v3337)) + (self.scalar_v318 * v3358));
        let v3410: f64 = (self.scalar_v932 * f64::powf(v925, self.scalar_v3408));
        let v3416: f64 = (v209 * v3406);
        let v3417: f64 = (v209 * (self.scalar_v318 * v3359));
        let v3418: f64 = (v209 * ((v314 * v3338) + (self.scalar_v318 * v3360)));
        let v3419: f64 = (v209 * (v314 * v3339));
        let v3426: f64 = (self.scalar_v931 * f64::powf(v935, self.scalar_v3424));
        let v3453: f64 = (self.scalar_v931 * f64::powf(v942, self.scalar_v3424));
        let v3471: f64 = (if self.scalar_v940 { ((v944 * (v185 * v3393)) + (v941 * (v3416 * v3453))) } else { (if self.scalar_v930 { (v185 * (v3393 + (((v3393 * v3410) + v3416) * v3426))) } else { v304 }) });
        let v3472: f64 = (if self.scalar_v940 { ((v944 * (v185 * v3394)) + (v941 * (v3417 * v3453))) } else { (if self.scalar_v930 { (v185 * (v3394 + (((v3394 * v3410) + v3417) * v3426))) } else { v304 }) });
        let v3474: f64 = (if self.scalar_v940 { ((v944 * (v185 * v3396)) + (v941 * (v3418 * v3453))) } else { (if self.scalar_v930 { (v185 * (v3396 + (((v3396 * v3410) + v3418) * v3426))) } else { v304 }) });
        let v3475: f64 = (if self.scalar_v940 { ((v944 * (v185 * v3397)) + (v941 * (v3419 * v3453))) } else { (if self.scalar_v930 { (v185 * (v3397 + (((v3397 * v3410) + v3419) * v3426))) } else { v304 }) });
        let v3479: f64 = (v946 * v946);
        let v3516: f64 = (v950 * v950);
        let v3522: f64 = (if self.scalar_v949 { (v51 / v950) } else { v304 });
        let v3524: f64 = (if self.scalar_v949 { (v360 / v950) } else { v304 });
        let v3525: f64 = { let limexp_arg = v952; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v3540: f64 = { let limexp_arg = v956; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v3555: f64 = ((self.scalar_v959 * (if self.scalar_v949 { ((if self.scalar_v949 { ((-(v349 * self.scalar_v3513)) / v3516) } else { v3346 }) * v3525) } else { v3350 })) + (self.scalar_v961 * (if self.scalar_v949 { ((if self.scalar_v949 { ((-(v345 * self.scalar_v3513)) / v3516) } else { v304 }) * v3540) } else { v304 })));
        let v3566: f64 = (if self.scalar_v949 { (v84 * ((self.scalar_v959 * (if self.scalar_v949 { ((if self.scalar_v949 { v304 } else { v3347 }) * v3525) } else { v3351 })) + (self.scalar_v961 * (if self.scalar_v949 { (v3524 * v3540) } else { v304 })))) } else { v304 });
        let v3568: f64 = (if self.scalar_v949 { (v84 * ((self.scalar_v959 * (if self.scalar_v949 { ((if self.scalar_v949 { v304 } else { v3348 }) * v3525) } else { v3352 })) + (self.scalar_v961 * (if self.scalar_v949 { (v3522 * v3540) } else { v304 })))) } else { v304 });
        let v4363: f64 = (v360 / v11);
        let v4364: f64 = (v51 / v11);
        let v4365: f64 = { let limexp_arg = v1159; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v4372: f64 = { let limexp_arg = v1161; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v4381: f64 = (v182 * v1165);
        let v4390: f64 = (v182 * v1168);
        let v4694: f64 = (self.scalar_v337 * (v1257 * v3337));
        let v4695: f64 = (self.scalar_v337 * (v1257 * v3338));
        let v4696: f64 = (self.scalar_v337 * (v1257 * v3339));
        let v4700: f64 = (v1260 * v1260);
        let v4723: f64 = { let limexp_arg = v1270; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v4728: f64 = (v1261 * (((v1260 * v4694) - (v1259 * v4694)) / v4700));
        let v4730: f64 = (v1261 * (((v1260 * v4695) - (v1259 * v4695)) / v4700));
        let v4732: f64 = (v1261 * (((v1260 * v4696) - (v1259 * v4696)) / v4700));
        let v4753: f64 = ((v1277 * (self.scalar_v1262 * (self.scalar_v1263 * v3396))) + (v1266 * (v1257 * ((v1274 * (self.scalar_v1267 * (self.scalar_v4722 * v4723))) + (v1272 * (v4730 + v4730))))));
        let v4776: f64 = (v946 * ((v1278 * v3337) + (v905 * ((v1277 * (self.scalar_v1262 * (self.scalar_v1263 * v3393))) + (v1266 * (v1257 * (v1272 * (v4728 + v4728))))))));
        let v4782: f64 = ((v946 * (v905 * ((v1277 * (self.scalar_v1262 * (self.scalar_v1263 * v3394))) + (v1266 * (v1257 * (v1274 * (self.scalar_v1267 * (self.scalar_v4721 * v4723)))))))) - (v1281 * v3472));
        let v4783: f64 = (v4782 / v3479);
        let v4786: f64 = ((v946 * (v905 * (v1277 * (self.scalar_v1262 * (self.scalar_v1263 * v3395))))) - (v1281 * (if self.scalar_v940 { (v944 * (v185 * v3395)) } else { (if self.scalar_v930 { (v185 * (v3395 + ((v3395 * v3410) * v3426))) } else { v304 }) })));
        let v4787: f64 = (v4786 / v3479);
        let v4792: f64 = (v946 * ((v1278 * v3339) + (v905 * ((v1277 * (self.scalar_v1262 * (self.scalar_v1263 * v3397))) + (v1266 * (v1257 * (v1272 * (v4732 + v4732))))))));
        let v4796: f64 = ((self.scalar_v989 * ((v428 * v1673) + (v279 * v1871))) + ((v4776 - (v1281 * v3471)) / v3479));
        let v4797: f64 = ((self.scalar_v989 * (v279 * v1872)) + (((v946 * ((v1278 * v3338) + (v905 * v4753))) - (v1281 * v3474)) / v3479));
        let v4798: f64 = ((self.scalar_v989 * (v279 * v1873)) + ((v4792 - (v1281 * v3475)) / v3479));
        let v4805: f64 = (self.scalar_v1084 * ((v488 * v1673) + (v279 * (if self.scalar_v390 { ((v2024 + (self.scalar_v422 * (v1950 + (-v1993)))) - v1963) } else { (if self.scalar_v354 { (v1931 + v1935) } else { v304 }) }))));
        let v4806: f64 = (self.scalar_v1084 * (v279 * (if self.scalar_v390 { (v2025 + (self.scalar_v422 * (v51 - v1994))) } else { (if self.scalar_v354 { (v1932 + v1936) } else { v304 }) })));
        let v4807: f64 = (self.scalar_v1084 * (v279 * (if self.scalar_v390 { (v2026 + (self.scalar_v422 * (-v1995))) } else { (if self.scalar_v354 { (v1933 + v1937) } else { v304 }) })));
        let v4808: f64 = (self.scalar_v1084 * (v279 * (if self.scalar_v390 { (v2027 + (self.scalar_v422 * (v360 - v1996))) } else { (if self.scalar_v354 { (v1934 + v1938) } else { v304 }) })));
        let v4813: f64 = (v284 * v2519);
        let v4815: f64 = (v284 * v2521);
        let v4825: f64 = ((((v671 * (self.scalar_v280 * v1680)) + (v284 * v2517)) + (self.scalar_v1287 * v3358)) + (self.scalar_v1290 * (((v1160 * v1695) + (v294 * (((-(v345 * v1345)) / v1396) * v4365))) / v4381)));
        let v4826: f64 = (((v284 * v2518) + (self.scalar_v1287 * v3359)) + (self.scalar_v1290 * ((v294 * (v4363 * v4365)) / v4381)));
        let v4827: f64 = (((v284 * v2520) + (self.scalar_v1287 * v3360)) + (self.scalar_v1290 * ((v294 * (v4364 * v4365)) / v4381)));
        let v4828: f64 = (self.scalar_v1290 * (((v1162 * v1695) + (v294 * (((-(v347 * v1345)) / v1396) * v4372))) / v4390));
        let v4829: f64 = (self.scalar_v1290 * ((v294 * (v4363 * v4372)) / v4390));
        let v4830: f64 = (self.scalar_v1290 * ((v294 * (v4364 * v4372)) / v4390));
        let v4837: f64 = (v286 * v3032);
        let v4844: f64 = (((v816 * (self.scalar_v285 * v1680)) + (v286 * (if self.scalar_v636 { ((v3002 + (self.scalar_v665 * (v2902 + (-v2959)))) - v2915) } else { v2893 }))) + (self.scalar_v1287 * (if self.scalar_v985 { v304 } else { (if self.scalar_v949 { ((v964 * v1438) + (v84 * v3555)) } else { v304 }) })));
        let v4845: f64 = ((v286 * (if self.scalar_v636 { (v3003 + (self.scalar_v665 * (-v2960))) } else { v2894 })) + (self.scalar_v1287 * (if self.scalar_v985 { v304 } else { v3566 })));
        let v4846: f64 = ((v286 * (if self.scalar_v636 { (v3004 + (self.scalar_v665 * (v51 - v2961))) } else { v2895 })) + (self.scalar_v1287 * (if self.scalar_v985 { v304 } else { (if self.scalar_v949 { (v84 * (self.scalar_v959 * (if self.scalar_v949 { (v3522 * v3525) } else { v304 }))) } else { v304 }) })));
        let v4847: f64 = ((v286 * (if self.scalar_v636 { (v3005 + (self.scalar_v665 * (-v2962))) } else { v2896 })) + (self.scalar_v1287 * (if self.scalar_v985 { v304 } else { v3568 })));
        let v4848: f64 = ((v286 * (if self.scalar_v636 { (v3007 + (self.scalar_v665 * (v360 - v2964))) } else { v2898 })) + (self.scalar_v1287 * (if self.scalar_v985 { v304 } else { (if self.scalar_v949 { (v84 * (self.scalar_v959 * (if self.scalar_v949 { (v3524 * v3525) } else { v304 }))) } else { v304 }) })));
        let v4851: f64 = ((v900 * (self.scalar_v287 * (((-(self.scalar_v248 * v1664)) / v1685) * (self.scalar_v289 * f64::powf(v288, self.scalar_v1687))))) + (v291 * (if self.scalar_v899 { v304 } else { (if self.scalar_v860 { (v3300 - v3177) } else { (if self.scalar_v823 { (v3129 + v3136) } else { v304 }) }) })));
        let v4852: f64 = (v291 * (if self.scalar_v899 { v304 } else { v3309 }));
        let v4853: f64 = (v291 * (if self.scalar_v899 { v304 } else { v3310 }));
        let v4854: f64 = (v291 * (if self.scalar_v899 { v304 } else { v3311 }));
        let v4855: f64 = (v291 * (if self.scalar_v899 { v304 } else { v3312 }));
        let v4859: f64 = ((v291 * (if self.scalar_v899 { v304 } else { v3313 })) + self.scalar_v4858);
        let v4860: f64 = (self.scalar_v1298 + (v291 * (if self.scalar_v899 { v304 } else { v3314 })));

        let d1283_dn4: f64 = v4796;
        let d1283_dn6: f64 = v4783;
        let d1283_dn7: f64 = v4787;
        let d1283_dn8: f64 = v4797;
        let d1283_dn9: f64 = v4798;
        let v1283_reactive_nodes: [usize; 5] = [nodes[4], nodes[6], nodes[7], nodes[8], nodes[9]];
        let v1283_reactive_node_derivatives: [f64; 5] = [d1283_dn4, d1283_dn6, d1283_dn7, d1283_dn8, d1283_dn9];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[9]),
            &v1283_reactive_nodes,
            &v1283_reactive_node_derivatives,
            &[],
            &[],
            multiplicity,
        );
        let d1285_dn4: f64 = v4805;
        let d1285_dn7: f64 = v4806;
        let d1285_dn8: f64 = v4807;
        let d1285_dn9: f64 = v4808;
        let v1285_reactive_nodes: [usize; 4] = [nodes[4], nodes[7], nodes[8], nodes[9]];
        let v1285_reactive_node_derivatives: [f64; 4] = [d1285_dn4, d1285_dn7, d1285_dn8, d1285_dn9];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[9]),
            &v1285_reactive_nodes,
            &v1285_reactive_node_derivatives,
            &[],
            &[],
            multiplicity,
        );
        let d1292_dn4: f64 = v4825;
        let d1292_dn6: f64 = v4826;
        let d1292_dn7: f64 = v4813;
        let d1292_dn8: f64 = v4827;
        let d1292_dn9: f64 = v4815;
        let v1292_reactive_nodes: [usize; 5] = [nodes[4], nodes[6], nodes[7], nodes[8], nodes[9]];
        let v1292_reactive_node_derivatives: [f64; 5] = [d1292_dn4, d1292_dn6, d1292_dn7, d1292_dn8, d1292_dn9];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[6]),
            &v1292_reactive_nodes,
            &v1292_reactive_node_derivatives,
            &[],
            &[],
            multiplicity,
        );
        let d1293_dn4: f64 = v4828;
        let d1293_dn5: f64 = v4829;
        let d1293_dn8: f64 = v4830;
        stamper.stamp_current_reactive_node3(
            Some(nodes[8]),
            Some(nodes[5]),
            nodes[4],
            multiplicity * (d1293_dn4),
            nodes[5],
            multiplicity * (d1293_dn5),
            nodes[8],
            multiplicity * (d1293_dn8),
        );
        let d1296_dn4: f64 = v4844;
        let d1296_dn6: f64 = v4845;
        let d1296_dn7: f64 = v4846;
        let d1296_dn8: f64 = v4847;
        let d1296_dn9: f64 = v4837;
        let d1296_dn10: f64 = v4848;
        let v1296_reactive_nodes: [usize; 6] = [nodes[4], nodes[6], nodes[7], nodes[8], nodes[9], nodes[10]];
        let v1296_reactive_node_derivatives: [f64; 6] = [d1296_dn4, d1296_dn6, d1296_dn7, d1296_dn8, d1296_dn9, d1296_dn10];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[10]),
            &v1296_reactive_nodes,
            &v1296_reactive_node_derivatives,
            &[],
            &[],
            multiplicity,
        );
        let d1303_dn1: f64 = self.scalar_v1302;
        let d1303_dn2: f64 = self.scalar_v4861;
        stamper.stamp_current_reactive_node2(
            Some(nodes[1]),
            Some(nodes[2]),
            nodes[1],
            multiplicity * (d1303_dn1),
            nodes[2],
            multiplicity * (d1303_dn2),
        );
        let d1306_dn0: f64 = self.scalar_v4862;
        let d1306_dn1: f64 = self.scalar_v1305;
        stamper.stamp_current_reactive_node2(
            Some(nodes[1]),
            Some(nodes[0]),
            nodes[0],
            multiplicity * (d1306_dn0),
            nodes[1],
            multiplicity * (d1306_dn1),
        );
        let d1300_dn4: f64 = v4851;
        let d1300_dn6: f64 = v4852;
        let d1300_dn7: f64 = v4853;
        let d1300_dn8: f64 = v4854;
        let d1300_dn9: f64 = v4855;
        let d1300_dn10: f64 = v4859;
        let d1300_dn11: f64 = v4860;
        let v1300_reactive_nodes: [usize; 7] = [nodes[4], nodes[6], nodes[7], nodes[8], nodes[9], nodes[10], nodes[11]];
        let v1300_reactive_node_derivatives: [f64; 7] = [d1300_dn4, d1300_dn6, d1300_dn7, d1300_dn8, d1300_dn9, d1300_dn10, d1300_dn11];
        stamper.stamp_current_reactive_dense(
            Some(nodes[11]),
            Some(nodes[10]),
            &v1300_reactive_nodes,
            &v1300_reactive_node_derivatives,
            &[],
            &[],
            multiplicity,
        );
        let d1344_dn4: f64 = self.scalar_v1343;
        stamper.stamp_current_reactive_node1(
            Some(nodes[4]),
            None,
            nodes[4],
            multiplicity * (d1344_dn4),
        );
    }
}
