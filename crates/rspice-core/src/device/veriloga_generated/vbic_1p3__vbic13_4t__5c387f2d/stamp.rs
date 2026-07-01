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
        let v0: f64 = if ctx.analysis_static() { 1.0 } else { 0.0 };
        let v1: f64 = 0.0;
        let v2: f64 = 1.0;
        let v3: f64 = 0.01;
        let v11: f64 = (if ((v0 != 0.0) && self.scalar_v8) { 1e-12 } else { (if ((v0 != 0.0) && (self.scalar_v4 != 0.0)) { self.scalar_v6 } else { v1 }) });
        let v18: f64 = (if ((v0 != 0.0) && self.scalar_v16) { v2 } else { (if ((v0 != 0.0) && (self.scalar_v12 != 0.0)) { self.scalar_v14 } else { v1 }) });
        let v24: bool = ((v0 != 0.0) && self.scalar_v23);
        let v26: f64 = -1.0;
        let v30: bool = (v24 && self.scalar_v29);
        let v36: f64 = (if (v30 && self.scalar_v34) { v2 } else { (if ((self.scalar_v28 != 0.0) && v30) { self.scalar_v32 } else { (if ((self.scalar_v22 != 0.0) && v24) { v26 } else { (if ((v0 != 0.0) && (self.scalar_v19 != 0.0)) { v2 } else { v1 }) }) }) });
        let v39: f64 = (if (v0 != 0.0) { self.scalar_v38 } else { v1 });
        let v44: f64 = (if (v0 != 0.0) { self.scalar_v43 } else { v1 });
        let v49: f64 = (if (v0 != 0.0) { self.scalar_v48 } else { v1 });
        let v54: f64 = (if (v0 != 0.0) { self.scalar_v53 } else { v1 });
        let v59: f64 = (if (v0 != 0.0) { self.scalar_v58 } else { v1 });
        let v64: f64 = (if (v0 != 0.0) { self.scalar_v63 } else { v1 });
        let v67: f64 = 273.15;
        let v70: f64 = (if (v0 != 0.0) { self.scalar_v69 } else { v1 });
        let v94: f64 = 1.380662e-23;
        let v96: f64 = 1.602189e-19;
        let v98: f64 = (self.scalar_v93 / v70);
        let v114: f64 = (if self.scalar_v113 { v1 } else { (if self.scalar_v101 { (self.scalar_v103 * (((self.scalar_v107 + (v18 / self.scalar_v100))) as f64).ln()) } else { v1 }) });
        let v122: f64 = (v2 - v98);
        let v127: f64 = ((self.scalar_v99 * f64::powf(v98, self.scalar_v117)) * ((((self.scalar_v121 * v122) / self.scalar_v124)) as f64).exp());
        let v128: bool = (v127 > v1);
        let v132: bool = (self.scalar_v130 && (v18 > self.scalar_v129));
        let v134: f64 = 0.5;
        let v135: f64 = (v18 * v134);
        let v136: f64 = 4.0;
        let v155: f64 = (if (v128 && (!v132)) { (self.scalar_v124 * (((v2 + (v18 / v127))) as f64).ln()) } else { (if (v128 && v132) { (self.scalar_v124 * (((v2 + (f64::powf((v135 * self.scalar_v139), self.scalar_v142) / v127))) as f64).ln()) } else { v1 }) });
        let v157: f64 = (if (!v128) { v1 } else { v155 });
        let v170: f64 = ((self.scalar_v158 * f64::powf(v98, self.scalar_v161)) * ((((v122 * self.scalar_v165) / self.scalar_v167)) as f64).exp());
        let v172: bool = (v128 && (v170 > v1));
        let v174: bool = (self.scalar_v41 && (v18 > self.scalar_v40));
        let v180: f64 = (v127 * v170);
        let v192: f64 = (if (v172 && (!v174)) { (self.scalar_v167 * (((v2 + (v18 / v180))) as f64).ln()) } else { (if (v172 && v174) { (self.scalar_v167 * (((v2 + (f64::powf((v135 * self.scalar_v177), self.scalar_v142) / v180))) as f64).ln()) } else { v1 }) });
        let v194: f64 = (if (!v172) { v1 } else { v192 });
        let v206: f64 = ((self.scalar_v195 * f64::powf(v98, self.scalar_v197)) * ((((v122 * self.scalar_v201) / self.scalar_v203)) as f64).exp());
        let v207: bool = (v206 > v1);
        let v209: bool = (self.scalar_v46 && (v18 > self.scalar_v45));
        let v224: f64 = (if (v207 && (!v209)) { (self.scalar_v203 * (((v2 + (v18 / v206))) as f64).ln()) } else { (if (v207 && v209) { (self.scalar_v203 * (((v2 + ((v49 * (v18 * v18)) / v206))) as f64).ln()) } else { v1 }) });
        let v226: f64 = (if (!v207) { v1 } else { v224 });
        let v239: f64 = ((self.scalar_v227 * f64::powf(v98, self.scalar_v230)) * ((((v122 * self.scalar_v234) / self.scalar_v236)) as f64).exp());
        let v240: bool = (v239 > v1);
        let v247: f64 = (if (!v240) { v1 } else { (if v240 { (self.scalar_v236 * (((v2 + (v18 / v239))) as f64).ln()) } else { v1 }) });
        let v260: f64 = ((self.scalar_v248 * f64::powf(v98, self.scalar_v251)) * ((((v122 * self.scalar_v255) / self.scalar_v257)) as f64).exp());
        let v261: bool = (v260 > v1);
        let v268: f64 = (if (!v261) { v1 } else { (if v261 { (self.scalar_v257 * (((v2 + (v18 / v260))) as f64).ln()) } else { v1 }) });
        let v272: f64 = f64::powf(v98, self.scalar_v271);
        let v279: f64 = ((((v122 * self.scalar_v275) / self.scalar_v277)) as f64).exp();
        let v280: f64 = ((self.scalar_v269 * v272) * v279);
        let v281: bool = (v280 > v1);
        let v288: f64 = (if (!v281) { v1 } else { (if v281 { (self.scalar_v277 * (((v2 + (v18 / v280))) as f64).ln()) } else { v1 }) });
        let v292: f64 = f64::powf(v98, self.scalar_v291);
        let v299: f64 = ((((v122 * self.scalar_v295) / self.scalar_v297)) as f64).exp();
        let v300: f64 = ((self.scalar_v289 * v292) * v299);
        let v301: bool = (v300 > v1);
        let v308: f64 = (if (!v301) { v1 } else { (if v301 { (self.scalar_v297 * (((v2 + (v18 / v300))) as f64).ln()) } else { v1 }) });
        let v311: f64 = (v279 * (v272 * self.scalar_v309));
        let v312: bool = (v311 > v1);
        let v319: f64 = (if (!v312) { v1 } else { (if v312 { (self.scalar_v277 * (((v2 + (v18 / v311))) as f64).ln()) } else { v1 }) });
        let v322: f64 = (v299 * (v292 * self.scalar_v320));
        let v323: bool = (v322 > v1);
        let v330: f64 = (if (!v323) { v1 } else { (if v323 { (self.scalar_v297 * (((v2 + (v18 / v322))) as f64).ln()) } else { v1 }) });
        let v342: f64 = ((self.scalar_v331 * f64::powf(v98, self.scalar_v333)) * ((((v122 * self.scalar_v337) / self.scalar_v339)) as f64).exp());
        let v343: bool = (v342 > v1);
        let v350: f64 = (if (!v343) { v1 } else { (if v343 { (self.scalar_v339 * (((v2 + (v18 / v342))) as f64).ln()) } else { v1 }) });
        let v362: f64 = ((self.scalar_v351 * f64::powf(v98, self.scalar_v353)) * ((((v122 * self.scalar_v357) / self.scalar_v359)) as f64).exp());
        let v363: bool = (v362 > v1);
        let v370: f64 = (if (!v363) { v1 } else { (if v363 { (self.scalar_v359 * (((v2 + (v18 / v362))) as f64).ln()) } else { v1 }) });
        let v371: f64 = ctx.node_voltage(nodes[4]);
        let v373: f64 = ((self.scalar_v73 + v371) - v67);
        let v374: bool = (v373 < self.scalar_v76);
        let v377: f64 = ((((v373 - self.scalar_v75) - v2)) as f64).exp();
        let v379: f64 = (if v374 { (self.scalar_v75 + v377) } else { v373 });
        let v382: bool = ((v379 > self.scalar_v84) && (!v374));
        let v385: f64 = ((((self.scalar_v83 - v379) - v2)) as f64).exp();
        let v388: f64 = (v67 + (if v382 { (self.scalar_v83 - v385) } else { v379 }));
        let v390: f64 = ((v94 * v388) / v96);
        let v391: f64 = (v388 / v70);
        let v392: f64 = (v388 - v70);
        let v395: f64 = (self.scalar_v129 * f64::powf(v391, self.scalar_v393));
        let v404: f64 = f64::powf(v391, self.scalar_v403);
        let v406: f64 = (if self.scalar_v402 { (self.scalar_v397 * v404) } else { (if (self.scalar_v396 != 0.0) { (self.scalar_v397 * f64::powf(v391, self.scalar_v398)) } else { v1 }) });
        let v415: f64 = (if self.scalar_v413 { (v404 * self.scalar_v408) } else { (if (self.scalar_v407 != 0.0) { (self.scalar_v408 * f64::powf(v391, self.scalar_v409)) } else { v1 }) });
        let v424: f64 = f64::powf(v391, self.scalar_v423);
        let v426: f64 = (if self.scalar_v422 { (self.scalar_v417 * v424) } else { (if (self.scalar_v416 != 0.0) { (self.scalar_v417 * f64::powf(v391, self.scalar_v418)) } else { v1 }) });
        let v435: f64 = (if self.scalar_v433 { (v424 * self.scalar_v428) } else { (if (self.scalar_v427 != 0.0) { (self.scalar_v428 * f64::powf(v391, self.scalar_v429)) } else { v1 }) });
        let v439: f64 = (self.scalar_v436 * f64::powf(v391, self.scalar_v437));
        let v443: f64 = (self.scalar_v440 * f64::powf(v391, self.scalar_v441));
        let v452: f64 = (if self.scalar_v450 { (v404 * self.scalar_v445) } else { (if (self.scalar_v444 != 0.0) { (self.scalar_v445 * f64::powf(v391, self.scalar_v446)) } else { v1 }) });
        let v457: f64 = (self.scalar_v453 * (v2 + (v392 * self.scalar_v454)));
        let v459: f64 = (self.scalar_v99 * f64::powf(v391, self.scalar_v117));
        let v460: f64 = (v2 - v391);
        let v461: f64 = (self.scalar_v121 * v460);
        let v462: f64 = (self.scalar_v116 * v390);
        let v464: f64 = (((v461 / v462)) as f64).exp();
        let v465: f64 = (v459 * v464);
        let v467: f64 = (self.scalar_v158 * f64::powf(v391, self.scalar_v161));
        let v468: f64 = (self.scalar_v165 * v460);
        let v469: f64 = (self.scalar_v160 * v390);
        let v471: f64 = (((v468 / v469)) as f64).exp();
        let v472: f64 = (v467 * v471);
        let v474: f64 = (self.scalar_v195 * f64::powf(v391, self.scalar_v197));
        let v475: f64 = (self.scalar_v201 * v460);
        let v476: f64 = (self.scalar_v196 * v390);
        let v478: f64 = (((v475 / v476)) as f64).exp();
        let v479: f64 = (v474 * v478);
        let v481: f64 = (self.scalar_v227 * f64::powf(v391, self.scalar_v230));
        let v482: f64 = (self.scalar_v234 * v460);
        let v483: f64 = (self.scalar_v229 * v390);
        let v485: f64 = (((v482 / v483)) as f64).exp();
        let v486: f64 = (v481 * v485);
        let v488: f64 = (self.scalar_v248 * f64::powf(v391, self.scalar_v251));
        let v489: f64 = (self.scalar_v255 * v460);
        let v490: f64 = (self.scalar_v250 * v390);
        let v492: f64 = (((v489 / v490)) as f64).exp();
        let v493: f64 = (v488 * v492);
        let v494: f64 = f64::powf(v391, self.scalar_v271);
        let v495: f64 = (self.scalar_v269 * v494);
        let v496: f64 = (self.scalar_v275 * v460);
        let v497: f64 = (self.scalar_v270 * v390);
        let v499: f64 = (((v496 / v497)) as f64).exp();
        let v500: f64 = (v495 * v499);
        let v501: f64 = f64::powf(v391, self.scalar_v291);
        let v502: f64 = (self.scalar_v289 * v501);
        let v503: f64 = (self.scalar_v295 * v460);
        let v504: f64 = (self.scalar_v290 * v390);
        let v506: f64 = (((v503 / v504)) as f64).exp();
        let v507: f64 = (v502 * v506);
        let v508: f64 = (self.scalar_v309 * v494);
        let v509: f64 = (v499 * v508);
        let v510: f64 = (self.scalar_v320 * v501);
        let v511: f64 = (v506 * v510);
        let v513: f64 = (self.scalar_v331 * f64::powf(v391, self.scalar_v333));
        let v514: f64 = (self.scalar_v337 * v460);
        let v515: f64 = (self.scalar_v332 * v390);
        let v517: f64 = (((v514 / v515)) as f64).exp();
        let v518: f64 = (v513 * v517);
        let v520: f64 = (self.scalar_v351 * f64::powf(v391, self.scalar_v353));
        let v521: f64 = (self.scalar_v357 * v460);
        let v522: f64 = (self.scalar_v352 * v390);
        let v524: f64 = (((v521 / v522)) as f64).exp();
        let v525: f64 = (v520 * v524);
        let v528: f64 = (v2 + (v392 * self.scalar_v526));
        let v529: f64 = (self.scalar_v116 * v528);
        let v530: f64 = (self.scalar_v160 * v528);
        let v535: f64 = (self.scalar_v531 * (v2 + (v392 * self.scalar_v532)));
        let v540: f64 = (self.scalar_v536 * (v2 + (v392 * self.scalar_v537)));
        let v544: f64 = (self.scalar_v541 + (v392 * self.scalar_v542));
        let v551: f64 = (self.scalar_v102 * (v2 + (v392 * self.scalar_v548)));
        let v552: f64 = 2.0;
        let v554: f64 = (v552 * (v390 / v391));
        let v557: f64 = (v391 * self.scalar_v556);
        let v559: f64 = (((v557 / v390)) as f64).exp();
        let v560: f64 = -0.5;
        let v562: f64 = (v391 * self.scalar_v561);
        let v564: f64 = (((v562 / v390)) as f64).exp();
        let v565: f64 = (v559 - v564);
        let v566: f64 = ((v565) as f64).ln();
        let v567: f64 = (v554 * v566);
        let v569: f64 = 3.0;
        let v570: f64 = (v390 * v569);
        let v571: f64 = ((v391) as f64).ln();
        let v572: f64 = (v570 * v571);
        let v574: f64 = (v391 - v2);
        let v576: f64 = (((v391 * v567) - v572) - (self.scalar_v233 * v574));
        let v577: f64 = (v390 * v552);
        let v578: f64 = (-v576);
        let v580: f64 = (((v578 / v390)) as f64).exp();
        let v583: f64 = (((v2 + (v136 * v580))) as f64).sqrt();
        let v585: f64 = (v134 * (v2 + v583));
        let v586: f64 = ((v585) as f64).ln();
        let v588: f64 = (v576 + (v577 * v586));
        let v591: f64 = (v391 * self.scalar_v590);
        let v593: f64 = (((v591 / v390)) as f64).exp();
        let v595: f64 = (v391 * self.scalar_v594);
        let v597: f64 = (((v595 / v390)) as f64).exp();
        let v598: f64 = (v593 - v597);
        let v599: f64 = ((v598) as f64).ln();
        let v600: f64 = (v554 * v599);
        let v604: f64 = (((v391 * v600) - v572) - (self.scalar_v274 * v574));
        let v605: f64 = (-v604);
        let v607: f64 = (((v605 / v390)) as f64).exp();
        let v610: f64 = (((v2 + (v136 * v607))) as f64).sqrt();
        let v612: f64 = (v134 * (v2 + v610));
        let v613: f64 = ((v612) as f64).ln();
        let v615: f64 = (v604 + (v577 * v613));
        let v618: f64 = (v391 * self.scalar_v617);
        let v620: f64 = (((v618 / v390)) as f64).exp();
        let v622: f64 = (v391 * self.scalar_v621);
        let v624: f64 = (((v622 / v390)) as f64).exp();
        let v625: f64 = (v620 - v624);
        let v626: f64 = ((v625) as f64).ln();
        let v627: f64 = (v554 * v626);
        let v631: f64 = (((v391 * v627) - v572) - (self.scalar_v336 * v574));
        let v632: f64 = (-v631);
        let v634: f64 = (((v632 / v390)) as f64).exp();
        let v637: f64 = (((v2 + (v136 * v634))) as f64).sqrt();
        let v639: f64 = (v134 * (v2 + v637));
        let v640: f64 = ((v639) as f64).ln();
        let v642: f64 = (v631 + (v577 * v640));
        let v644: f64 = (self.scalar_v555 / v588);
        let v647: f64 = (self.scalar_v643 * f64::powf(v644, self.scalar_v645));
        let v649: f64 = (self.scalar_v589 / v615);
        let v651: f64 = f64::powf(v649, self.scalar_v650);
        let v652: f64 = (self.scalar_v648 * v651);
        let v654: f64 = (v651 * self.scalar_v653);
        let v656: f64 = (self.scalar_v616 / v642);
        let v659: f64 = (self.scalar_v655 * f64::powf(v656, self.scalar_v657));
        let v662: f64 = (self.scalar_v660 * f64::powf(v391, self.scalar_v115));
        let v664: f64 = (((v461 / v390)) as f64).exp();
        let v665: f64 = (v662 * v664);
        let v669: f64 = (self.scalar_v666 * f64::powf(v391, self.scalar_v667));
        let v670: f64 = (-(self.scalar_v104 * (v2 + (v392 * v544))));
        let v671: f64 = (v390 * v551);
        let v673: f64 = (((v670 / v671)) as f64).exp();
        let v678: f64 = (self.scalar_v674 * (v2 + (v392 * self.scalar_v675)));
        let v683: f64 = (self.scalar_v679 * (v2 + (v392 * self.scalar_v680)));
        let v684: f64 = 0.001;
        let v685: bool = (v406 > v684);
        let v687: f64 = 1000.0;
        let v688: f64 = (if v685 { (v2 / v406) } else { v687 });
        let v689: bool = (v415 > v684);
        let v691: f64 = (if v689 { (v2 / v415) } else { v687 });
        let v692: bool = (v426 > v684);
        let v694: f64 = (if v692 { (v2 / v426) } else { v687 });
        let v695: bool = (v435 > v684);
        let v697: f64 = (if v695 { (v2 / v435) } else { v687 });
        let v698: bool = (v439 > v684);
        let v700: f64 = (if v698 { (v2 / v439) } else { v687 });
        let v701: bool = (v452 > v684);
        let v703: f64 = (if v701 { (v2 / v452) } else { v687 });
        let v704: bool = (v443 > v684);
        let v706: f64 = (if v704 { (v2 / v443) } else { v687 });
        let v707: bool = (v457 > v684);
        let v709: f64 = (if v707 { (v2 / v457) } else { v687 });
        let v710: bool = (v678 > v1);
        let v712: f64 = (if v710 { (v2 / v678) } else { v1 });
        let v713: bool = (v683 > v1);
        let v715: f64 = (if v713 { (v2 / v683) } else { v1 });
        let v716: bool = (v395 > v1);
        let v718: f64 = (if v716 { (v2 / v395) } else { v1 });
        let v719: bool = (v669 > v1);
        let v721: f64 = (if v719 { (v2 / v669) } else { v1 });
        let v722: f64 = ctx.node_voltage(nodes[8]);
        let v723: f64 = ctx.node_voltage(nodes[9]);
        let v725: f64 = (v36 * (v722 - v723));
        let v726: f64 = ctx.node_voltage(nodes[7]);
        let v728: f64 = (v36 * (v726 - v723));
        let v729: f64 = ctx.node_voltage(nodes[6]);
        let v731: f64 = (v36 * (v722 - v729));
        let v732: f64 = ctx.node_voltage(nodes[5]);
        let v734: f64 = (v36 * (v722 - v732));
        let v736: f64 = (v36 * (v726 - v732));
        let v737: f64 = ctx.node_voltage(nodes[10]);
        let v739: f64 = (v36 * (v726 - v737));
        let v740: f64 = ctx.node_voltage(nodes[1]);
        let v741: f64 = ctx.node_voltage(nodes[2]);
        let v744: f64 = (v36 * (v729 - v723));
        let v745: f64 = ctx.node_voltage(nodes[0]);
        let v747: f64 = (v745 - v732);
        let v749: f64 = (v36 * (v732 - v729));
        let v750: f64 = (v740 - v726);
        let v751: f64 = (v726 - v722);
        let v752: f64 = (v741 - v723);
        let v753: f64 = (v737 - v732);
        let v754: f64 = ctx.node_voltage(nodes[11]);
        let v756: f64 = (v36 * (v754 - v737));
        let v758: f64 = (v36 * (v726 - v754));
        let v760: f64 = (ctx.node_voltage(nodes[3]) - v754);
        let v761: f64 = ctx.node_voltage(nodes[12]);
        let v762: f64 = ctx.node_voltage(nodes[13]);
        let v763: f64 = (-v588);
        let v765: f64 = (v763 * self.scalar_v764);
        let v768: f64 = (v725 + v765);
        let v769: f64 = (if self.scalar_v767 { v768 } else { v1 });
        let v770: bool = (v769 > v1);
        let v771: bool = (self.scalar_v767 && v770);
        let v775: f64 = (if v771 { self.scalar_v774 } else { v1 });
        let v777: f64 = (v2 - (self.scalar_v772 * v775));
        let v783: f64 = (v769 * self.scalar_v782);
        let v784: f64 = (v588 * self.scalar_v772);
        let v786: f64 = (v2 + (v783 / v784));
        let v791: bool = (self.scalar_v767 && (!v770));
        let v793: f64 = (v2 - (v725 / v588));
        let v795: f64 = (v2 - f64::powf(v793, self.scalar_v779));
        let v798: f64 = (if v791 { ((v588 * v795) / self.scalar_v779) } else { (if v771 { ((v588 * v777) / self.scalar_v779) } else { v1 }) });
        let v807: f64 = ((((v765 * v765) + self.scalar_v805)) as f64).sqrt();
        let v811: f64 = (if self.scalar_v802 { (v560 * (v765 + (if self.scalar_v802 { v807 } else { v1 }))) } else { v1 });
        let v813: f64 = (v2 - (v811 / v588));
        let v814: f64 = f64::powf(v813, self.scalar_v779);
        let v817: f64 = (if self.scalar_v802 { ((v763 * v814) / self.scalar_v779) } else { v1 });
        let v818: f64 = (if self.scalar_v802 { v768 } else { v1 });
        let v821: f64 = (((self.scalar_v805 + (v818 * v818))) as f64).sqrt();
        let v826: f64 = (if self.scalar_v802 { ((v134 * (v818 - (if self.scalar_v802 { v821 } else { v1 }))) - v765) } else { v1 });
        let v828: f64 = (v2 - (v826 / v588));
        let v829: f64 = f64::powf(v828, self.scalar_v779);
        let v834: f64 = (v811 + (v725 - v826));
        let v835: f64 = (self.scalar_v774 * v834);
        let v836: f64 = (self.scalar_v782 * v834);
        let v838: f64 = (v2 + (v836 / v784));
        let v842: f64 = (if self.scalar_v802 { (((if self.scalar_v802 { ((v763 * v829) / self.scalar_v779) } else { v798 }) + (v835 * v838)) - v817) } else { (if self.scalar_v767 { (v798 + (if v791 { v1 } else { (if v771 { (v775 * (v769 * v786)) } else { v1 }) })) } else { v1 }) });
        let v843: f64 = (-v615);
        let v844: f64 = (self.scalar_v764 * v843);
        let v847: f64 = (v731 + v844);
        let v848: f64 = (if self.scalar_v846 { v847 } else { v1 });
        let v849: bool = (v848 > v1);
        let v850: bool = (self.scalar_v846 && v849);
        let v853: f64 = (if v850 { self.scalar_v852 } else { v1 });
        let v856: f64 = (v2 - (self.scalar_v772 * (self.scalar_v772 * v853)));
        let v862: f64 = (v848 * self.scalar_v861);
        let v864: f64 = (self.scalar_v772 + (v862 / v615));
        let v872: bool = (self.scalar_v869 && (v731 < self.scalar_v870));
        let v874: bool = (self.scalar_v846 && (!v849));
        let v875: bool = (v872 && v874);
        let v877: f64 = (v2 + (self.scalar_v868 / v615));
        let v878: f64 = f64::powf(v877, self.scalar_v858);
        let v880: f64 = (self.scalar_v858 * (v731 + self.scalar_v868));
        let v881: f64 = (v615 + self.scalar_v868);
        let v883: f64 = (v2 - (v880 / v881));
        let v885: f64 = (v2 - (v878 * v883));
        let v890: bool = (v874 && (!v872));
        let v892: f64 = (v2 - (v731 / v615));
        let v894: f64 = (v2 - f64::powf(v892, self.scalar_v858));
        let v897: f64 = (if v890 { ((v615 * v894) / self.scalar_v858) } else { (if v875 { ((v615 * v885) / self.scalar_v858) } else { (if v850 { ((v615 * v856) / self.scalar_v858) } else { v1 }) }) });
        let v906: f64 = (v844 + self.scalar_v868);
        let v907: f64 = (self.scalar_v868 - v844);
        let v909: f64 = (if self.scalar_v905 { (v906 / v907) } else { v1 });
        let v910: f64 = (v552 * v909);
        let v911: f64 = (v909 - v2);
        let v916: f64 = ((((v911 * v911) + self.scalar_v914)) as f64).sqrt();
        let v917: f64 = (v2 + v909);
        let v922: f64 = ((((v917 * v917) + self.scalar_v920)) as f64).sqrt();
        let v923: f64 = (v916 + v922);
        let v925: f64 = (if self.scalar_v905 { (v910 / v923) } else { v1 });
        let v930: f64 = (if self.scalar_v905 { (v134 * (((v907 * v925) - self.scalar_v868) - v844)) } else { v1 });
        let v932: f64 = (v2 - (v930 / v615));
        let v934: f64 = (v2 - f64::powf(v932, self.scalar_v858));
        let v937: f64 = (if self.scalar_v905 { ((v615 * v934) / self.scalar_v858) } else { v1 });
        let v940: f64 = (v844 + (self.scalar_v868 + (v552 * v731)));
        let v942: f64 = (if self.scalar_v905 { (v940 / v907) } else { v1 });
        let v943: f64 = (v552 * v942);
        let v944: f64 = (v942 - v2);
        let v947: f64 = (((self.scalar_v914 + (v944 * v944))) as f64).sqrt();
        let v948: f64 = (v2 + v942);
        let v951: f64 = (((self.scalar_v920 + (v948 * v948))) as f64).sqrt();
        let v952: f64 = (v947 + v951);
        let v954: f64 = (if self.scalar_v905 { (v943 / v952) } else { v1 });
        let v959: f64 = (if self.scalar_v905 { (v134 * (((v907 * v954) - self.scalar_v868) - v844)) } else { v1 });
        let v961: f64 = (v2 - (v959 / v615));
        let v963: f64 = (v2 - f64::powf(v961, self.scalar_v858));
        let v966: f64 = (if self.scalar_v905 { ((v615 * v963) / self.scalar_v858) } else { v897 });
        let v969: f64 = (if self.scalar_v905 { (v134 * (v2 + v954)) } else { v1 });
        let v972: f64 = (if self.scalar_v905 { f64::powf(v877, self.scalar_v970) } else { v1 });
        let v974: f64 = (v2 + (v844 / v615));
        let v976: f64 = (if self.scalar_v905 { f64::powf(v974, self.scalar_v970) } else { v1 });
        let v977: f64 = (v2 - v969);
        let v981: f64 = (if self.scalar_v905 { ((v972 * v977) + (v969 * v976)) } else { v1 });
        let v983: f64 = (v930 + (v731 - v959));
        let v988: f64 = (if self.scalar_v905 { ((v966 + (if self.scalar_v905 { (v981 * v983) } else { v1 })) - v937) } else { (if self.scalar_v846 { (v897 + (if v874 { v1 } else { (if v850 { (v853 * (v848 * v864)) } else { v1 }) })) } else { v1 }) });
        let v993: f64 = (((self.scalar_v914 + (v844 * v844))) as f64).sqrt();
        let v997: f64 = (if self.scalar_v990 { (v560 * (v844 + (if self.scalar_v990 { v993 } else { v1 }))) } else { v930 });
        let v999: f64 = (v2 - (v997 / v615));
        let v1000: f64 = f64::powf(v999, self.scalar_v858);
        let v1003: f64 = (if self.scalar_v990 { ((v843 * v1000) / self.scalar_v858) } else { v1 });
        let v1004: f64 = (if self.scalar_v990 { v847 } else { v1 });
        let v1007: f64 = (((self.scalar_v914 + (v1004 * v1004))) as f64).sqrt();
        let v1012: f64 = (if self.scalar_v990 { ((v134 * (v1004 - (if self.scalar_v990 { v1007 } else { v1 }))) - v844) } else { v959 });
        let v1014: f64 = (v2 - (v1012 / v615));
        let v1015: f64 = f64::powf(v1014, self.scalar_v858);
        let v1025: f64 = (if self.scalar_v990 { (((if self.scalar_v990 { ((v843 * v1015) / self.scalar_v858) } else { v966 }) + (self.scalar_v1019 * (v997 + (v731 - v1012)))) - v1003) } else { v988 });
        let v1026: f64 = (v390 * v529);
        let v1027: f64 = (v2 / v1026);
        let v1028: bool = (v725 < v157);
        let v1030: f64 = (((v725 * v1027)) as f64).exp();
        let v1032: bool = (!v1028);
        let v1034: f64 = (((v157 * v1027)) as f64).exp();
        let v1035: f64 = (v725 - v157);
        let v1037: f64 = (v2 + (v1027 * v1035));
        let v1039: f64 = (if v1032 { (v1034 * v1037) } else { (if v1028 { v1030 } else { v1 }) });
        let v1040: f64 = (v1039 - v2);
        let v1041: f64 = (v465 * v1040);
        let v1042: f64 = (v390 * v530);
        let v1043: f64 = (v2 / v1042);
        let v1044: bool = (v731 < v194);
        let v1046: f64 = (((v731 * v1043)) as f64).exp();
        let v1048: bool = (!v1044);
        let v1050: f64 = (((v194 * v1043)) as f64).exp();
        let v1051: f64 = (v731 - v194);
        let v1053: f64 = (v2 + (v1043 * v1051));
        let v1055: f64 = (if v1048 { (v1050 * v1053) } else { (if v1044 { v1046 } else { v1039 }) });
        let v1056: f64 = (v465 * v472);
        let v1057: f64 = (v1055 - v2);
        let v1058: f64 = (v1056 * v1057);
        let v1063: f64 = 0.0001;
        let v1064: f64 = (((v2 + (v715 * v842)) + (v712 * v1025)) - v1063);
        let v1066: f64 = 1e-8;
        let v1068: f64 = ((((v1064 * v1064) + v1066)) as f64).sqrt();
        let v1071: f64 = (v1063 + (v134 * (v1064 + v1068)));
        let v1079: f64 = (v136 * ((v718 * v1041) + (v44 * v1058)));
        let v1081: f64 = (if self.scalar_v1076 { (f64::powf(v1071, self.scalar_v1077) + v1079) } else { v1 });
        let v1082: bool = (v1081 > v1066);
        let v1083: bool = (self.scalar_v1076 && v1082);
        let v1089: bool = (self.scalar_v1076 && (!v1082));
        let v1096: f64 = (if self.scalar_v1094 { (v2 + v1079) } else { v1081 });
        let v1097: bool = (v1096 > v1066);
        let v1098: bool = (self.scalar_v1094 && v1097);
        let v1099: f64 = (v134 * v1071);
        let v1101: f64 = (v2 + f64::powf(v1096, self.scalar_v138));
        let v1105: bool = (self.scalar_v1094 && (!v1097));
        let v1108: f64 = (if v1105 { (v1099 * self.scalar_v1106) } else { (if v1098 { (v1099 * v1101) } else { (if v1089 { (v134 * (v1071 + self.scalar_v1090)) } else { (if v1083 { (v134 * (v1071 + f64::powf(v1081, self.scalar_v138))) } else { v1 }) }) }) });
        let v1109: f64 = (v1058 / v1108);
        let v1110: f64 = (v1041 / v1108);
        let v1113: f64 = (if self.scalar_v1111 { (v2 / v476) } else { v1043 });
        let v1114: bool = (v739 < v226);
        let v1115: bool = (self.scalar_v1111 && v1114);
        let v1117: f64 = (((v739 * v1113)) as f64).exp();
        let v1120: bool = (self.scalar_v1111 && (!v1114));
        let v1122: f64 = (((v226 * v1113)) as f64).exp();
        let v1123: f64 = (v739 - v226);
        let v1125: f64 = (v2 + (v1113 * v1123));
        let v1127: f64 = (if v1120 { (v1122 * v1125) } else { (if v1115 { v1117 } else { v1055 }) });
        let v1128: bool = (v731 < v226);
        let v1129: bool = (self.scalar_v1111 && v1128);
        let v1131: f64 = (((v731 * v1113)) as f64).exp();
        let v1134: bool = (self.scalar_v1111 && (!v1128));
        let v1135: f64 = (v731 - v226);
        let v1137: f64 = (v2 + (v1113 * v1135));
        let v1139: f64 = (if v1134 { (v1122 * v1137) } else { (if v1129 { v1131 } else { v1 }) });
        let v1145: f64 = (((v1127 * self.scalar_v1140) + (v1139 * self.scalar_v1142)) - v2);
        let v1147: f64 = (if self.scalar_v1111 { (v479 * v1145) } else { v1 });
        let v1152: f64 = (if self.scalar_v1111 { (v2 + (v136 * (if self.scalar_v1111 { (v49 * v1147) } else { v1 }))) } else { v1096 });
        let v1153: bool = (v1152 > v1066);
        let v1154: bool = (self.scalar_v1111 && v1153);
        let v1155: f64 = ((v1152) as f64).sqrt();
        let v1160: bool = (self.scalar_v1111 && (!v1153));
        let v1162: f64 = (if v1160 { 0.50005 } else { (if v1154 { (v134 * (v2 + v1155)) } else { v1 }) });
        let v1163: bool = (v756 < v226);
        let v1164: bool = (self.scalar_v1111 && v1163);
        let v1166: f64 = (((v756 * v1113)) as f64).exp();
        let v1169: bool = (self.scalar_v1111 && (!v1163));
        let v1170: f64 = (v756 - v226);
        let v1172: f64 = (v2 + (v1113 * v1170));
        let v1174: f64 = (if v1169 { (v1122 * v1172) } else { (if v1164 { v1166 } else { v1127 }) });
        let v1175: f64 = (v1174 - v2);
        let v1178: f64 = (v1147 - (if self.scalar_v1111 { (v479 * v1175) } else { v1 }));
        let v1183: f64 = (if self.scalar_v1181 { v2 } else { v1162 });
        let v1184: f64 = (if self.scalar_v1181 { v1 } else { (if self.scalar_v1111 { (v1178 / v1162) } else { v1 }) });
        let v1187: f64 = (v2 / v483);
        let v1188: f64 = (if self.scalar_v1186 { v1187 } else { v1113 });
        let v1189: bool = (v725 < v247);
        let v1190: bool = (self.scalar_v1186 && v1189);
        let v1192: f64 = (((v725 * v1188)) as f64).exp();
        let v1194: bool = (!v1189);
        let v1195: bool = (self.scalar_v1186 && v1194);
        let v1197: f64 = (((v247 * v1188)) as f64).exp();
        let v1198: f64 = (v725 - v247);
        let v1200: f64 = (v2 + (v1188 * v1198));
        let v1202: f64 = (if v1195 { (v1197 * v1200) } else { (if v1190 { v1192 } else { v1174 }) });
        let v1203: f64 = (v2 / v490);
        let v1204: f64 = (if self.scalar_v1186 { v1203 } else { v1188 });
        let v1205: bool = (v725 < v268);
        let v1206: bool = (self.scalar_v1186 && v1205);
        let v1208: f64 = (((v725 * v1204)) as f64).exp();
        let v1210: bool = (!v1205);
        let v1211: bool = (self.scalar_v1186 && v1210);
        let v1213: f64 = (((v268 * v1204)) as f64).exp();
        let v1214: f64 = (v725 - v268);
        let v1216: f64 = (v2 + (v1204 * v1214));
        let v1218: f64 = (if v1211 { (v1213 * v1216) } else { (if v1206 { v1208 } else { v1 }) });
        let v1224: f64 = (v2 + (self.scalar_v1219 * (v1071 - v2)));
        let v1225: f64 = (v486 * v1224);
        let v1226: f64 = (v1202 - v2);
        let v1228: f64 = (v1218 - v2);
        let v1229: f64 = (v493 * v1228);
        let v1236: f64 = (if self.scalar_v1233 { (v1229 + (v486 * v1226)) } else { (if self.scalar_v1221 { ((v1225 * v1226) + v1229) } else { v1 }) });
        let v1239: f64 = (v670 - v725);
        let v1240: f64 = (if self.scalar_v1238 { v1239 } else { v1 });
        let v1241: f64 = (v2 / v671);
        let v1242: f64 = (if self.scalar_v1238 { v1241 } else { v1204 });
        let v1243: bool = (v1240 < v114);
        let v1244: bool = (self.scalar_v1238 && v1243);
        let v1246: f64 = (((v1240 * v1242)) as f64).exp();
        let v1249: bool = (self.scalar_v1238 && (!v1243));
        let v1251: f64 = (((v114 * v1242)) as f64).exp();
        let v1252: f64 = (v1240 - v114);
        let v1254: f64 = (v2 + (v1242 * v1252));
        let v1256: f64 = (if v1249 { (v1251 * v1254) } else { (if v1244 { v1246 } else { v1139 }) });
        let v1265: f64 = (if self.scalar_v1263 { v1187 } else { v1242 });
        let v1266: bool = (v728 < v247);
        let v1267: bool = (self.scalar_v1263 && v1266);
        let v1269: f64 = (((v728 * v1265)) as f64).exp();
        let v1271: bool = (!v1266);
        let v1272: bool = (self.scalar_v1263 && v1271);
        let v1274: f64 = (((v247 * v1265)) as f64).exp();
        let v1275: f64 = (v728 - v247);
        let v1277: f64 = (v2 + (v1265 * v1275));
        let v1279: f64 = (if v1272 { (v1274 * v1277) } else { (if v1267 { v1269 } else { v1202 }) });
        let v1280: f64 = (if self.scalar_v1263 { v1203 } else { v1265 });
        let v1281: bool = (v728 < v268);
        let v1282: bool = (self.scalar_v1263 && v1281);
        let v1284: f64 = (((v728 * v1280)) as f64).exp();
        let v1286: bool = (!v1281);
        let v1287: bool = (self.scalar_v1263 && v1286);
        let v1289: f64 = (((v268 * v1280)) as f64).exp();
        let v1290: f64 = (v728 - v268);
        let v1292: f64 = (v2 + (v1280 * v1290));
        let v1294: f64 = (if v1287 { (v1289 * v1292) } else { (if v1282 { v1284 } else { v1218 }) });
        let v1295: f64 = (v1279 - v2);
        let v1297: f64 = (v1294 - v2);
        let v1300: f64 = (if self.scalar_v1263 { ((v486 * v1295) + (v493 * v1297)) } else { v1 });
        let v1302: f64 = (if self.scalar_v1301 { v1239 } else { v1240 });
        let v1303: f64 = (if self.scalar_v1301 { v1241 } else { v1280 });
        let v1304: bool = (v1302 < v114);
        let v1305: bool = (self.scalar_v1301 && v1304);
        let v1307: f64 = (((v1302 * v1303)) as f64).exp();
        let v1310: bool = (self.scalar_v1301 && (!v1304));
        let v1312: f64 = (((v114 * v1303)) as f64).exp();
        let v1313: f64 = (v1302 - v114);
        let v1315: f64 = (v2 + (v1303 * v1313));
        let v1317: f64 = (if v1310 { (v1312 * v1315) } else { (if v1305 { v1307 } else { v1256 }) });
        let v1324: f64 = (if self.scalar_v1323 { v1187 } else { v1303 });
        let v1325: bool = (v1189 && self.scalar_v1323);
        let v1327: f64 = (((v725 * v1324)) as f64).exp();
        let v1329: bool = (v1194 && self.scalar_v1323);
        let v1331: f64 = (((v247 * v1324)) as f64).exp();
        let v1333: f64 = (v2 + (v1198 * v1324));
        let v1335: f64 = (if v1329 { (v1331 * v1333) } else { (if v1325 { v1327 } else { v1279 }) });
        let v1336: f64 = (if self.scalar_v1323 { v1203 } else { v1324 });
        let v1337: bool = (v1205 && self.scalar_v1323);
        let v1339: f64 = (((v725 * v1336)) as f64).exp();
        let v1341: bool = (v1210 && self.scalar_v1323);
        let v1343: f64 = (((v268 * v1336)) as f64).exp();
        let v1345: f64 = (v2 + (v1214 * v1336));
        let v1347: f64 = (if v1341 { (v1343 * v1345) } else { (if v1337 { v1339 } else { v1294 }) });
        let v1349: f64 = (v1335 - v2);
        let v1351: f64 = (v1347 - v2);
        let v1352: f64 = (v493 * v1351);
        let v1360: f64 = (if self.scalar_v1356 { (self.scalar_v1185 * (v1352 + (v486 * v1349))) } else { (if self.scalar_v1348 { (self.scalar_v1185 * ((v1225 * v1349) + v1352)) } else { (if self.scalar_v1263 { v1 } else { (if self.scalar_v1238 { (v1236 - (self.scalar_v100 * (v1256 - v673))) } else { v1236 }) }) }) });
        let v1362: f64 = (if self.scalar_v1361 { v1239 } else { v1302 });
        let v1363: f64 = (if self.scalar_v1361 { v1241 } else { v1336 });
        let v1364: bool = (v1362 < v114);
        let v1365: bool = (self.scalar_v1361 && v1364);
        let v1367: f64 = (((v1362 * v1363)) as f64).exp();
        let v1370: bool = (self.scalar_v1361 && (!v1364));
        let v1372: f64 = (((v114 * v1363)) as f64).exp();
        let v1373: f64 = (v1362 - v114);
        let v1375: f64 = (v2 + (v1363 * v1373));
        let v1377: f64 = (if v1370 { (v1372 * v1375) } else { (if v1365 { v1367 } else { v1317 }) });
        let v1382: f64 = (if self.scalar_v1361 { (v1360 - (self.scalar_v1378 * (v1377 - v673))) } else { v1360 });
        let v1383: f64 = (if self.scalar_v1323 { v1187 } else { v1363 });
        let v1384: bool = (v1266 && self.scalar_v1323);
        let v1386: f64 = (((v728 * v1383)) as f64).exp();
        let v1388: bool = (v1271 && self.scalar_v1323);
        let v1390: f64 = (((v247 * v1383)) as f64).exp();
        let v1392: f64 = (v2 + (v1275 * v1383));
        let v1394: f64 = (if v1388 { (v1390 * v1392) } else { (if v1384 { v1386 } else { v1335 }) });
        let v1395: f64 = (if self.scalar_v1323 { v1203 } else { v1383 });
        let v1396: bool = (v1281 && self.scalar_v1323);
        let v1398: f64 = (((v728 * v1395)) as f64).exp();
        let v1400: bool = (v1286 && self.scalar_v1323);
        let v1402: f64 = (((v268 * v1395)) as f64).exp();
        let v1404: f64 = (v2 + (v1290 * v1395));
        let v1406: f64 = (if v1400 { (v1402 * v1404) } else { (if v1396 { v1398 } else { v1347 }) });
        let v1408: f64 = (v1394 - v2);
        let v1410: f64 = (v1406 - v2);
        let v1414: f64 = (if self.scalar_v1323 { (self.scalar_v1407 * ((v486 * v1408) + (v493 * v1410))) } else { (if self.scalar_v1301 { (v1300 - (self.scalar_v100 * (v1317 - v673))) } else { v1300 }) });
        let v1415: f64 = (if self.scalar_v1361 { v1239 } else { v1362 });
        let v1416: f64 = (if self.scalar_v1361 { v1241 } else { v1395 });
        let v1417: bool = (v1415 < v114);
        let v1418: bool = (self.scalar_v1361 && v1417);
        let v1420: f64 = (((v1415 * v1416)) as f64).exp();
        let v1423: bool = (self.scalar_v1361 && (!v1417));
        let v1425: f64 = (((v114 * v1416)) as f64).exp();
        let v1426: f64 = (v1415 - v114);
        let v1428: f64 = (v2 + (v1416 * v1426));
        let v1430: f64 = (if v1423 { (v1425 * v1428) } else { (if v1418 { v1420 } else { v1377 }) });
        let v1435: f64 = (if self.scalar_v1361 { (v1414 - (self.scalar_v1431 * (v1430 - v673))) } else { v1414 });
        let v1436: f64 = (v2 / v497);
        let v1437: bool = (v731 < v288);
        let v1439: f64 = (((v731 * v1436)) as f64).exp();
        let v1441: bool = (!v1437);
        let v1443: f64 = (((v288 * v1436)) as f64).exp();
        let v1444: f64 = (v731 - v288);
        let v1446: f64 = (v2 + (v1436 * v1444));
        let v1448: f64 = (if v1441 { (v1443 * v1446) } else { (if v1437 { v1439 } else { v1394 }) });
        let v1449: f64 = (v2 / v504);
        let v1450: bool = (v731 < v308);
        let v1452: f64 = (((v731 * v1449)) as f64).exp();
        let v1454: bool = (!v1450);
        let v1456: f64 = (((v308 * v1449)) as f64).exp();
        let v1457: f64 = (v731 - v308);
        let v1459: f64 = (v2 + (v1449 * v1457));
        let v1461: f64 = (if v1454 { (v1456 * v1459) } else { (if v1450 { v1452 } else { v1406 }) });
        let v1462: f64 = (v1448 - v2);
        let v1464: f64 = (v1461 - v2);
        let v1466: f64 = ((v500 * v1462) + (v507 * v1464));
        let v1470: f64 = (if self.scalar_v1469 { v1436 } else { v1449 });
        let v1471: bool = (v739 < v319);
        let v1472: bool = (self.scalar_v1469 && v1471);
        let v1474: f64 = (((v739 * v1470)) as f64).exp();
        let v1477: bool = (self.scalar_v1469 && (!v1471));
        let v1479: f64 = (((v319 * v1470)) as f64).exp();
        let v1480: f64 = (v739 - v319);
        let v1482: f64 = (v2 + (v1470 * v1480));
        let v1484: f64 = (if v1477 { (v1479 * v1482) } else { (if v1472 { v1474 } else { v1448 }) });
        let v1485: f64 = (if self.scalar_v1469 { v1449 } else { v1470 });
        let v1486: bool = (v739 < v330);
        let v1487: bool = (self.scalar_v1469 && v1486);
        let v1489: f64 = (((v739 * v1485)) as f64).exp();
        let v1492: bool = (self.scalar_v1469 && (!v1486));
        let v1494: f64 = (((v330 * v1485)) as f64).exp();
        let v1495: f64 = (v739 - v330);
        let v1497: f64 = (v2 + (v1485 * v1495));
        let v1499: f64 = (if v1492 { (v1494 * v1497) } else { (if v1487 { v1489 } else { v1461 }) });
        let v1500: f64 = (v1484 - v2);
        let v1502: f64 = (v1499 - v2);
        let v1507: f64 = (if self.scalar_v1506 { v1 } else { (if self.scalar_v1469 { ((v509 * v1500) + (v511 * v1502)) } else { v1 }) });
        let v1508: f64 = (v731 / v390);
        let v1509: bool = (v1508 < v39);
        let v1510: f64 = ((v1508) as f64).exp();
        let v1512: bool = (!v1509);
        let v1513: f64 = ((v39) as f64).exp();
        let v1517: f64 = (if v1512 { (v1513 * (v2 + (v1508 - v39))) } else { (if v1509 { v1510 } else { v1484 }) });
        let v1518: f64 = (v734 / v390);
        let v1519: bool = (v1518 < v39);
        let v1520: f64 = ((v1518) as f64).exp();
        let v1522: bool = (!v1519);
        let v1526: f64 = (if v1522 { (v1513 * (v2 + (v1518 - v39))) } else { (if v1519 { v1520 } else { v1430 }) });
        let v1529: f64 = (((v2 + (v665 * v1517))) as f64).sqrt();
        let v1532: f64 = (((v2 + (v665 * v1526))) as f64).sqrt();
        let v1533: f64 = (v688 * v747);
        let v1534: f64 = (v2 + v1529);
        let v1535: f64 = (v2 + v1532);
        let v1536: f64 = (v1534 / v1535);
        let v1539: f64 = ((v1529 - v1532) - ((v1536) as f64).ln());
        let v1541: f64 = (v749 + (v390 * v1539));
        let v1542: f64 = (v691 * v1541);
        let v1543: f64 = (v721 * v1542);
        let v1545: f64 = (v54 * (v134 * v721));
        let v1548: f64 = (((v3 + (v749 * v749))) as f64).sqrt();
        let v1550: f64 = (v2 + (v1545 * v1548));
        let v1551: f64 = (v691 * v1550);
        let v1552: f64 = (v1543 / v1551);
        let v1555: f64 = (((v2 + (v1552 * v1552))) as f64).sqrt();
        let v1556: f64 = (v1542 / v1555);
        let v1557: f64 = (v694 * v750);
        let v1558: f64 = (v751 * v1108);
        let v1559: f64 = (v697 * v1558);
        let v1560: f64 = (v700 * v752);
        let v1561: f64 = (v753 * v1183);
        let v1562: f64 = (v703 * v1561);
        let v1563: f64 = (v706 * v760);
        let v1566: f64 = 0.02;
        let v1568: f64 = (v1566 * (v2 + v535));
        let v1573: f64 = (if self.scalar_v1565 { f64::powf(v1568, self.scalar_v1571) } else { v1 });
        let v1575: f64 = ((v615 - v731) - v1573);
        let v1578: f64 = (((v3 + (v1575 * v1575))) as f64).sqrt();
        let v1582: f64 = (if self.scalar_v1565 { (v1573 + (v134 * (v1575 + v1578))) } else { v1 });
        let v1583: f64 = (-v535);
        let v1585: f64 = f64::powf(v1582, self.scalar_v1584);
        let v1587: f64 = (if self.scalar_v1565 { (v1583 * v1585) } else { v1 });
        let v1588: bool = (v1587 < v39);
        let v1589: bool = (self.scalar_v1565 && v1588);
        let v1590: f64 = ((v1587) as f64).exp();
        let v1593: bool = (self.scalar_v1565 && (!v1588));
        let v1594: f64 = (if v1593 { v1513 } else { v1 });
        let v1598: f64 = (if v1593 { (v1594 * (v2 + (v1587 - v39))) } else { (if v1589 { v1590 } else { v1 }) });
        let v1599: f64 = (self.scalar_v1564 * v1582);
        let v1601: f64 = (if self.scalar_v1565 { (v1598 * v1599) } else { v1 });
        let v1602: f64 = (v762 - v1109);
        let v1603: f64 = (v1602 - v1466);
        let v1611: f64 = (v1566 * (v2 + v540));
        let v1616: f64 = (if self.scalar_v1609 { f64::powf(v1611, self.scalar_v1614) } else { v1 });
        let v1618: f64 = ((v1 - v736) - v1616);
        let v1621: f64 = (((v3 + (v1618 * v1618))) as f64).sqrt();
        let v1625: f64 = (if self.scalar_v1609 { (v1616 + (v134 * (v1618 + v1621))) } else { v1 });
        let v1626: f64 = (-v540);
        let v1628: f64 = f64::powf(v1625, self.scalar_v1627);
        let v1630: f64 = (if self.scalar_v1609 { (v1626 * v1628) } else { v1 });
        let v1631: bool = (v1630 < v39);
        let v1632: bool = (self.scalar_v1609 && v1631);
        let v1633: f64 = ((v1630) as f64).exp();
        let v1636: bool = (self.scalar_v1609 && (!v1631));
        let v1637: f64 = (if v1636 { v1513 } else { v1 });
        let v1641: f64 = (if v1636 { (v1637 * (v2 + (v1630 - v39))) } else { (if v1632 { v1633 } else { v1 }) });
        let v1642: f64 = (self.scalar_v1608 * v1625);
        let v1644: f64 = (if self.scalar_v1609 { (v1641 * v1642) } else { v1601 });
        let v1645: f64 = (-v1533);
        let v1660: f64 = 0.1;
        let v1662: f64 = (if self.scalar_v1657 { ((v2 - (v731 / self.scalar_v1655)) - v1660) } else { v1 });
        let v1665: f64 = (((v1063 + (v1662 * v1662))) as f64).sqrt();
        let v1674: f64 = (if self.scalar_v1673 { self.scalar_v1652 } else { (if self.scalar_v1657 { (self.scalar_v1652 * (if self.scalar_v1657 { (v1660 + (v134 * (v1662 + v1665))) } else { v1662 })) } else { v1 }) });
        let v1676: f64 = ((v1110 / v1674) - v2);
        let v1684: f64 = ((v1466 - (if self.scalar_v1606 { v1 } else { (if self.scalar_v1565 { (v1601 * v1603) } else { v1 }) })) - (if self.scalar_v1681 { v1 } else { (if self.scalar_v1654 { (self.scalar_v1650 * f64::powf(v1676, self.scalar_v1677)) } else { v1 }) }));
        let v1689: f64 = (if self.scalar_v1687 { (v2 / v515) } else { v1485 });
        let v1690: bool = (v756 < v350);
        let v1691: bool = (self.scalar_v1687 && v1690);
        let v1693: f64 = (((v756 * v1689)) as f64).exp();
        let v1696: bool = (self.scalar_v1687 && (!v1690));
        let v1698: f64 = (((v350 * v1689)) as f64).exp();
        let v1699: f64 = (v756 - v350);
        let v1701: f64 = (v2 + (v1689 * v1699));
        let v1703: f64 = (if v1696 { (v1698 * v1701) } else { (if v1691 { v1693 } else { v1517 }) });
        let v1705: f64 = (if self.scalar_v1687 { (v2 / v522) } else { v1689 });
        let v1706: bool = (v756 < v370);
        let v1707: bool = (self.scalar_v1687 && v1706);
        let v1709: f64 = (((v756 * v1705)) as f64).exp();
        let v1712: bool = (self.scalar_v1687 && (!v1706));
        let v1714: f64 = (((v370 * v1705)) as f64).exp();
        let v1715: f64 = (v756 - v370);
        let v1717: f64 = (v2 + (v1705 * v1715));
        let v1720: f64 = (v1703 - v2);
        let v1722: f64 = ((if v1712 { (v1714 * v1717) } else { (if v1707 { v1709 } else { v1499 }) }) - v2);
        let v1727: f64 = (if self.scalar_v1726 { v1 } else { (if self.scalar_v1687 { ((v518 * v1720) + (v525 * v1722)) } else { v1 }) });
        let v1740: f64 = (((((((v725 * v1382) + (v731 * v1684)) + (v744 * v1602)) + (v728 * v1435)) + (v739 * v1507)) + (v760 * v1563)) + (v756 * v1727));
        let v1752: f64 = ((((((v1740 + (v758 * v1184)) + (v747 * v1533)) + (v749 * v1556)) + (v750 * v1557)) + (v751 * v1559)) + (v752 * v1560));
        let v1757: f64 = ((v1752 + (v753 * v1562)) * self.scalar_v1756);
        let v1758: f64 = (v371 * v709);
        let v1759: f64 = (v762 - v1110);
        let v1760: f64 = (v762 - v761);
        let v1773: f64 = (v36 * (v1382 + (v11 * v725)));
        let v1774: f64 = (v36 * (v1435 + (v11 * v728)));
        let v1775: f64 = (v36 * v762);
        let v1776: f64 = (v36 * v1109);
        let v1777: f64 = (v36 * (v1684 + (v11 * v731)));
        let v1778: f64 = (v36 * ((if self.scalar_v1648 { v1 } else { (if self.scalar_v1609 { (v1644 * v1645) } else { v1 }) }) + (v11 * v736)));
        let v1779: f64 = (v36 * (v1507 + (v11 * v739)));
        let v1780: f64 = (v36 * v1556);
        let v1781: f64 = (v36 * (v1727 + (v11 * v756)));
        let v1782: f64 = (v36 * v1184);
        let v1784: f64 = (-v642);
        let v1786: f64 = (if self.scalar_v1783 { (self.scalar_v764 * v1784) } else { v1 });
        let v1790: f64 = (v756 + v1786);
        let v1791: f64 = (if self.scalar_v1789 { v1790 } else { v1 });
        let v1792: bool = (v1791 > v1);
        let v1793: bool = (self.scalar_v1789 && v1792);
        let v1796: f64 = (if v1793 { self.scalar_v1795 } else { v1 });
        let v1798: f64 = (v2 - (self.scalar_v772 * v1796));
        let v1804: f64 = (v1791 * self.scalar_v1803);
        let v1805: f64 = (v642 * self.scalar_v772);
        let v1807: f64 = (v2 + (v1804 / v1805));
        let v1812: bool = (self.scalar_v1789 && (!v1792));
        let v1814: f64 = (v2 - (v756 / v642));
        let v1816: f64 = (v2 - f64::powf(v1814, self.scalar_v1800));
        let v1819: f64 = (if v1812 { ((v642 * v1816) / self.scalar_v1800) } else { (if v1793 { ((v642 * v1798) / self.scalar_v1800) } else { v1 }) });
        let v1829: f64 = ((((v1786 * v1786) + self.scalar_v1827)) as f64).sqrt();
        let v1833: f64 = (if self.scalar_v1824 { (v560 * (v1786 + (if self.scalar_v1824 { v1829 } else { v1 }))) } else { v1 });
        let v1835: f64 = (v2 - (v1833 / v642));
        let v1836: f64 = f64::powf(v1835, self.scalar_v1800);
        let v1840: f64 = (if self.scalar_v1824 { v1790 } else { v1 });
        let v1843: f64 = (((self.scalar_v1827 + (v1840 * v1840))) as f64).sqrt();
        let v1848: f64 = (if self.scalar_v1824 { ((v134 * (v1840 - (if self.scalar_v1824 { v1843 } else { v1 }))) - v1786) } else { v1 });
        let v1850: f64 = (v2 - (v1848 / v642));
        let v1851: f64 = f64::powf(v1850, self.scalar_v1800);
        let v1856: f64 = (v1833 + (v756 - v1848));
        let v1857: f64 = (self.scalar_v1795 * v1856);
        let v1858: f64 = (self.scalar_v1803 * v1856);
        let v1860: f64 = (v2 + (v1858 / v1805));
        let v1864: f64 = (if self.scalar_v1824 { (((if self.scalar_v1824 { ((v1784 * v1851) / self.scalar_v1800) } else { v1819 }) + (v1857 * v1860)) - (if self.scalar_v1824 { ((v1784 * v1836) / self.scalar_v1800) } else { v1 })) } else { (if self.scalar_v1789 { (v1819 + (if v1812 { v1 } else { (if v1793 { (v1796 * (v1791 * v1807)) } else { v1 }) })) } else { v1 }) });
        let v1866: f64 = (if self.scalar_v1865 { v1 } else { v1864 });
        let v1867: f64 = (v728 + v765);
        let v1868: f64 = (if self.scalar_v767 { v1867 } else { v1 });
        let v1869: bool = (v1868 > v1);
        let v1870: bool = (self.scalar_v767 && v1869);
        let v1871: f64 = (if v1870 { self.scalar_v774 } else { v1 });
        let v1873: f64 = (v2 - (self.scalar_v772 * v1871));
        let v1877: f64 = (self.scalar_v782 * v1868);
        let v1879: f64 = (v2 + (v1877 / v784));
        let v1884: bool = (self.scalar_v767 && (!v1869));
        let v1886: f64 = (v2 - (v728 / v588));
        let v1888: f64 = (v2 - f64::powf(v1886, self.scalar_v779));
        let v1891: f64 = (if v1884 { ((v588 * v1888) / self.scalar_v779) } else { (if v1870 { ((v588 * v1873) / self.scalar_v779) } else { v1 }) });
        let v1895: f64 = (if self.scalar_v802 { v1867 } else { v1 });
        let v1898: f64 = (((self.scalar_v805 + (v1895 * v1895))) as f64).sqrt();
        let v1903: f64 = (if self.scalar_v802 { ((v134 * (v1895 - (if self.scalar_v802 { v1898 } else { v1 }))) - v765) } else { v1 });
        let v1905: f64 = (v2 - (v1903 / v588));
        let v1906: f64 = f64::powf(v1905, self.scalar_v779);
        let v1911: f64 = (v811 + (v728 - v1903));
        let v1912: f64 = (self.scalar_v774 * v1911);
        let v1913: f64 = (self.scalar_v782 * v1911);
        let v1915: f64 = (v2 + (v1913 / v784));
        let v1919: f64 = (if self.scalar_v802 { (((if self.scalar_v802 { ((v763 * v1906) / self.scalar_v779) } else { v1891 }) + (v1912 * v1915)) - v817) } else { (if self.scalar_v767 { (v1891 + (if v1884 { v1 } else { (if v1870 { (v1871 * (v1868 * v1879)) } else { v1 }) })) } else { v1 }) });
        let v1920: f64 = (v739 + v844);
        let v1921: f64 = (if self.scalar_v846 { v1920 } else { v1 });
        let v1922: bool = (v1921 > v1);
        let v1923: bool = (self.scalar_v846 && v1922);
        let v1924: f64 = (if v1923 { self.scalar_v852 } else { v1 });
        let v1927: f64 = (v2 - (self.scalar_v772 * (self.scalar_v772 * v1924)));
        let v1931: f64 = (self.scalar_v861 * v1921);
        let v1933: f64 = (self.scalar_v772 + (v1931 / v615));
        let v1938: bool = (self.scalar_v869 && (v739 < self.scalar_v870));
        let v1940: bool = (self.scalar_v846 && (!v1922));
        let v1941: bool = (v1938 && v1940);
        let v1943: f64 = (self.scalar_v858 * (v739 + self.scalar_v868));
        let v1945: f64 = (v2 - (v1943 / v881));
        let v1947: f64 = (v2 - (v878 * v1945));
        let v1952: bool = (v1940 && (!v1938));
        let v1954: f64 = (v2 - (v739 / v615));
        let v1956: f64 = (v2 - f64::powf(v1954, self.scalar_v858));
        let v1959: f64 = (if v1952 { ((v615 * v1956) / self.scalar_v858) } else { (if v1941 { ((v615 * v1947) / self.scalar_v858) } else { (if v1923 { ((v615 * v1927) / self.scalar_v858) } else { v1 }) }) });
        let v1965: f64 = (v844 + (self.scalar_v868 + (v552 * v739)));
        let v1967: f64 = (if self.scalar_v905 { (v1965 / v907) } else { v1 });
        let v1968: f64 = (v552 * v1967);
        let v1969: f64 = (v1967 - v2);
        let v1972: f64 = (((self.scalar_v914 + (v1969 * v1969))) as f64).sqrt();
        let v1973: f64 = (v2 + v1967);
        let v1976: f64 = (((self.scalar_v920 + (v1973 * v1973))) as f64).sqrt();
        let v1977: f64 = (v1972 + v1976);
        let v1979: f64 = (if self.scalar_v905 { (v1968 / v1977) } else { v1 });
        let v1984: f64 = (if self.scalar_v905 { (v134 * (((v907 * v1979) - self.scalar_v868) - v844)) } else { v1 });
        let v1986: f64 = (v2 - (v1984 / v615));
        let v1988: f64 = (v2 - f64::powf(v1986, self.scalar_v858));
        let v1991: f64 = (if self.scalar_v905 { ((v615 * v1988) / self.scalar_v858) } else { v1959 });
        let v1994: f64 = (if self.scalar_v905 { (v134 * (v2 + v1979)) } else { v1 });
        let v1995: f64 = (v2 - v1994);
        let v1999: f64 = (if self.scalar_v905 { ((v972 * v1995) + (v976 * v1994)) } else { v1 });
        let v2001: f64 = (v930 + (v739 - v1984));
        let v2006: f64 = (if self.scalar_v905 { ((v1991 + (if self.scalar_v905 { (v1999 * v2001) } else { v1 })) - v937) } else { (if self.scalar_v846 { (v1959 + (if v1940 { v1 } else { (if v1923 { (v1924 * (v1921 * v1933)) } else { v1 }) })) } else { v1 }) });
        let v2007: f64 = (if self.scalar_v990 { v1920 } else { v1 });
        let v2010: f64 = (((self.scalar_v914 + (v2007 * v2007))) as f64).sqrt();
        let v2015: f64 = (if self.scalar_v990 { ((v134 * (v2007 - (if self.scalar_v990 { v2010 } else { v1 }))) - v844) } else { v1984 });
        let v2017: f64 = (v2 - (v2015 / v615));
        let v2018: f64 = f64::powf(v2017, self.scalar_v858);
        let v2027: f64 = (if self.scalar_v990 { (((if self.scalar_v990 { ((v843 * v2018) / self.scalar_v858) } else { v1991 }) + (self.scalar_v1019 * (v997 + (v739 - v2015)))) - v1003) } else { v2006 });
        let v2029: f64 = (if (v1041 > v1) { v2 } else { v1 });
        let v2031: f64 = (v64 * (v1041 * v2029));
        let v2032: f64 = (v2 + v2031);
        let v2033: f64 = (v2031 / v2032);
        let v2035: f64 = 1.44;
        let v2036: f64 = ((v59 * v731) / v2035);
        let v2037: bool = (v2036 < v39);
        let v2038: f64 = ((v2036) as f64).exp();
        let v2040: bool = (!v2037);
        let v2049: f64 = (self.scalar_v2045 * (v2 + (v1071 * self.scalar_v2046)));
        let v2051: f64 = ((if v2040 { (v1513 * (v2 + (v2036 - v39))) } else { (if v2037 { v2038 } else { v1703 }) }) * self.scalar_v2050);
        let v2053: f64 = ((if (v0 != 0.0) { self.scalar_v65 } else { v1 }) + (v2033 * v2033));
        let v2056: f64 = (v2 + (v2029 * (v2051 * v2053)));
        let v2057: f64 = (v2049 * v2056);
        let v2060: f64 = (v1041 * v2057);
        let v2081: f64 = ((v740 - v741) * self.scalar_v2080);
        let v2083: f64 = ((v740 - v745) * self.scalar_v2082);
        let v2085: f64 = (v371 * self.scalar_v2084);
        let v2087: f64 = (v761 * self.scalar_v2086);
        let v2090: f64 = ((v762 * self.scalar_v2086) * 0.3333333333333333);
        let v2091: f64 = (v36 * ((self.scalar_v1185 * (v647 * v842)) + (v2060 / v1108)));
        let v2092: f64 = (v36 * (self.scalar_v1407 * (v647 * v1919)));
        let v2093: f64 = (v36 * (((v652 * v1025) + (v1058 * self.scalar_v2066)) + (v1529 * self.scalar_v2069)));
        let v2094: f64 = (v36 * (v1532 * self.scalar_v2069));
        let v2095: f64 = (v36 * ((v654 * v2027) + ((if self.scalar_v1181 { v1 } else { v1147 }) * self.scalar_v2066)));
        let v2096: f64 = (v36 * ((v659 * v1866) + (v756 * self.scalar_v2077)));
        let v2097: f64 = (if v374 { v377 } else { v2 });
        let v2101: f64 = (if v382 { (-(v385 * (-v2097))) } else { v2097 });
        let v2103: f64 = ((v94 * v2101) / v96);
        let v2104: f64 = (v2101 / v70);
        let v2119: f64 = (v2104 * (self.scalar_v403 * f64::powf(v391, self.scalar_v2116)));
        let v2139: f64 = (v2104 * (self.scalar_v423 * f64::powf(v391, self.scalar_v2136)));
        let v2175: f64 = (-v2104);
        let v2176: f64 = (self.scalar_v121 * v2175);
        let v2186: f64 = ((v464 * (self.scalar_v99 * (v2104 * (self.scalar_v117 * f64::powf(v391, self.scalar_v2170))))) + (v459 * (v464 * (((v462 * v2176) - (v461 * (self.scalar_v116 * v2103))) / (v462 * v462)))));
        let v2202: f64 = ((v471 * (self.scalar_v158 * (v2104 * (self.scalar_v161 * f64::powf(v391, self.scalar_v2187))))) + (v467 * (v471 * (((v469 * (self.scalar_v165 * v2175)) - (v468 * (self.scalar_v160 * v2103))) / (v469 * v469)))));
        let v2209: f64 = (self.scalar_v196 * v2103);
        let v2213: f64 = (v476 * v476);
        let v2218: f64 = ((v478 * (self.scalar_v195 * (v2104 * (self.scalar_v197 * f64::powf(v391, self.scalar_v2203))))) + (v474 * (v478 * (((v476 * (self.scalar_v201 * v2175)) - (v475 * v2209)) / v2213))));
        let v2225: f64 = (self.scalar_v229 * v2103);
        let v2229: f64 = (v483 * v483);
        let v2234: f64 = ((v485 * (self.scalar_v227 * (v2104 * (self.scalar_v230 * f64::powf(v391, self.scalar_v2219))))) + (v481 * (v485 * (((v483 * (self.scalar_v234 * v2175)) - (v482 * v2225)) / v2229))));
        let v2241: f64 = (self.scalar_v250 * v2103);
        let v2245: f64 = (v490 * v490);
        let v2250: f64 = ((v492 * (self.scalar_v248 * (v2104 * (self.scalar_v251 * f64::powf(v391, self.scalar_v2235))))) + (v488 * (v492 * (((v490 * (self.scalar_v255 * v2175)) - (v489 * v2241)) / v2245))));
        let v2254: f64 = (v2104 * (self.scalar_v271 * f64::powf(v391, self.scalar_v2251)));
        let v2257: f64 = (self.scalar_v270 * v2103);
        let v2261: f64 = (v497 * v497);
        let v2263: f64 = (v499 * (((v497 * (self.scalar_v275 * v2175)) - (v496 * v2257)) / v2261));
        let v2270: f64 = (v2104 * (self.scalar_v291 * f64::powf(v391, self.scalar_v2267)));
        let v2273: f64 = (self.scalar_v290 * v2103);
        let v2277: f64 = (v504 * v504);
        let v2279: f64 = (v506 * (((v504 * (self.scalar_v295 * v2175)) - (v503 * v2273)) / v2277));
        let v2297: f64 = (self.scalar_v332 * v2103);
        let v2301: f64 = (v515 * v515);
        let v2306: f64 = ((v517 * (self.scalar_v331 * (v2104 * (self.scalar_v333 * f64::powf(v391, self.scalar_v2291))))) + (v513 * (v517 * (((v515 * (self.scalar_v337 * v2175)) - (v514 * v2297)) / v2301))));
        let v2313: f64 = (self.scalar_v352 * v2103);
        let v2317: f64 = (v522 * v522);
        let v2322: f64 = ((v524 * (self.scalar_v351 * (v2104 * (self.scalar_v353 * f64::powf(v391, self.scalar_v2307))))) + (v520 * (v524 * (((v522 * (self.scalar_v357 * v2175)) - (v521 * v2313)) / v2317))));
        let v2323: f64 = (self.scalar_v526 * v2101);
        let v2327: f64 = (self.scalar_v531 * (self.scalar_v532 * v2101));
        let v2329: f64 = (self.scalar_v536 * (self.scalar_v537 * v2101));
        let v2342: f64 = (v552 * (((v391 * v2103) - (v390 * v2104)) / (v391 * v391)));
        let v2347: f64 = (v390 * v390);
        let v2356: f64 = ((v559 * (((v390 * (self.scalar_v556 * v2104)) - (v557 * v2103)) / v2347)) - (v564 * (((v390 * (self.scalar_v561 * v2104)) - (v562 * v2103)) / v2347)));
        let v2368: f64 = ((v571 * (v569 * v2103)) + (v570 * (v2104 / v391)));
        let v2371: f64 = ((((v567 * v2104) + (v391 * ((v566 * v2342) + (v554 * (v2356 / v565))))) - v2368) - (self.scalar_v233 * v2104));
        let v2372: f64 = (v552 * v2103);
        let v2386: f64 = ((v586 * v2372) + (v577 * ((v134 * ((v136 * (v580 * (((v390 * (-v2371)) - (v578 * v2103)) / v2347))) / (v552 * v583))) / v585)));
        let v2387: f64 = (v2371 + v2386);
        let v2400: f64 = ((v593 * (((v390 * (self.scalar_v590 * v2104)) - (v591 * v2103)) / v2347)) - (v597 * (((v390 * (self.scalar_v594 * v2104)) - (v595 * v2103)) / v2347)));
        let v2410: f64 = ((((v600 * v2104) + (v391 * ((v599 * v2342) + (v554 * (v2400 / v598))))) - v2368) - (self.scalar_v274 * v2104));
        let v2424: f64 = ((v613 * v2372) + (v577 * ((v134 * ((v136 * (v607 * (((v390 * (-v2410)) - (v605 * v2103)) / v2347))) / (v552 * v610))) / v612)));
        let v2425: f64 = (v2410 + v2424);
        let v2438: f64 = ((v620 * (((v390 * (self.scalar_v617 * v2104)) - (v618 * v2103)) / v2347)) - (v624 * (((v390 * (self.scalar_v621 * v2104)) - (v622 * v2103)) / v2347)));
        let v2448: f64 = ((((v627 * v2104) + (v391 * ((v626 * v2342) + (v554 * (v2438 / v625))))) - v2368) - (self.scalar_v336 * v2104));
        let v2462: f64 = ((v640 * v2372) + (v577 * ((v134 * ((v136 * (v634 * (((v390 * (-v2448)) - (v632 * v2103)) / v2347))) / (v552 * v637))) / v639)));
        let v2463: f64 = (v2448 + v2462);
        let v2466: f64 = (v588 * v588);
        let v2472: f64 = (self.scalar_v643 * (((-(self.scalar_v555 * v2387)) / v2466) * (self.scalar_v645 * f64::powf(v644, self.scalar_v2468))));
        let v2475: f64 = (v615 * v615);
        let v2479: f64 = (((-(self.scalar_v589 * v2425)) / v2475) * (self.scalar_v650 * f64::powf(v649, self.scalar_v1584)));
        let v2484: f64 = (v642 * v642);
        let v2503: f64 = ((v664 * (self.scalar_v660 * (v2104 * (self.scalar_v115 * f64::powf(v391, self.scalar_v2491))))) + (v662 * (v664 * (((v390 * v2176) - (v461 * v2103)) / v2347))));
        let v2509: f64 = (-(self.scalar_v104 * ((v544 * v2101) + (v392 * (self.scalar_v542 * v2101)))));
        let v2512: f64 = ((v551 * v2103) + (v390 * (self.scalar_v102 * (self.scalar_v548 * v2101))));
        let v2516: f64 = (v671 * v671);
        let v2518: f64 = (v673 * (((v671 * v2509) - (v670 * v2512)) / v2516));
        let v2526: f64 = (if v685 { ((-(if self.scalar_v402 { (self.scalar_v397 * v2119) } else { (if (self.scalar_v396 != 0.0) { (self.scalar_v397 * (v2104 * (self.scalar_v398 * f64::powf(v391, self.scalar_v2110)))) } else { v1 }) })) / (v406 * v406)) } else { v1 });
        let v2530: f64 = (if v689 { ((-(if self.scalar_v413 { (self.scalar_v408 * v2119) } else { (if (self.scalar_v407 != 0.0) { (self.scalar_v408 * (v2104 * (self.scalar_v409 * f64::powf(v391, self.scalar_v2122)))) } else { v1 }) })) / (v415 * v415)) } else { v1 });
        let v2534: f64 = (if v692 { ((-(if self.scalar_v422 { (self.scalar_v417 * v2139) } else { (if (self.scalar_v416 != 0.0) { (self.scalar_v417 * (v2104 * (self.scalar_v418 * f64::powf(v391, self.scalar_v2130)))) } else { v1 }) })) / (v426 * v426)) } else { v1 });
        let v2538: f64 = (if v695 { ((-(if self.scalar_v433 { (self.scalar_v428 * v2139) } else { (if (self.scalar_v427 != 0.0) { (self.scalar_v428 * (v2104 * (self.scalar_v429 * f64::powf(v391, self.scalar_v2142)))) } else { v1 }) })) / (v435 * v435)) } else { v1 });
        let v2546: f64 = (if v701 { ((-(if self.scalar_v450 { (self.scalar_v445 * v2119) } else { (if (self.scalar_v444 != 0.0) { (self.scalar_v445 * (v2104 * (self.scalar_v446 * f64::powf(v391, self.scalar_v2160)))) } else { v1 }) })) / (v452 * v452)) } else { v1 });
        let v2570: f64 = (if v719 { ((-(self.scalar_v666 * (v2104 * (self.scalar_v667 * f64::powf(v391, self.scalar_v2504))))) / (v669 * v669)) } else { v1 });
        let v2571: f64 = (-v36);
        let v2572: f64 = (-v2387);
        let v2573: f64 = (self.scalar_v764 * v2572);
        let v2574: f64 = (if self.scalar_v767 { v2573 } else { v1 });
        let v2575: f64 = (if self.scalar_v767 { v36 } else { v1 });
        let v2576: f64 = (if self.scalar_v767 { v2571 } else { v1 });
        let v2583: f64 = (self.scalar_v772 * v2387);
        let v2584: f64 = (v784 * (self.scalar_v782 * v2574));
        let v2587: f64 = (v784 * v784);
        let v2589: f64 = ((self.scalar_v782 * v2575) / v784);
        let v2590: f64 = ((self.scalar_v782 * v2576) / v784);
        let v2612: f64 = (-(v36 / v588));
        let v2613: f64 = (-(v2571 / v588));
        let v2616: f64 = (self.scalar_v779 * f64::powf(v793, self.scalar_v2614));
        let v2631: f64 = (if v791 { (((v795 * v2387) + (v588 * (-((-((-(v725 * v2387)) / v2466)) * v2616)))) / self.scalar_v779) } else { (if v771 { ((v777 * v2387) / self.scalar_v779) } else { v1 }) });
        let v2632: f64 = (if v791 { ((v588 * (-(v2612 * v2616))) / self.scalar_v779) } else { v1 });
        let v2633: f64 = (if v791 { ((v588 * (-(v2613 * v2616))) / self.scalar_v779) } else { v1 });
        let v2640: f64 = (if self.scalar_v767 { (v2631 + (if v791 { v1 } else { (if v771 { (v775 * ((v786 * v2574) + (v769 * ((v2584 - (v783 * v2583)) / v2587)))) } else { v1 }) })) } else { v1 });
        let v2643: f64 = (v765 * v2573);
        let v2650: f64 = (if self.scalar_v802 { (v560 * (v2573 + (if self.scalar_v802 { ((v2643 + v2643) / (v552 * v807)) } else { v1 }))) } else { v1 });
        let v2663: f64 = (if self.scalar_v802 { (((v814 * v2572) + (v763 * ((-(((v588 * v2650) - (v811 * v2387)) / v2466)) * (self.scalar_v779 * f64::powf(v813, self.scalar_v2614))))) / self.scalar_v779) } else { v1 });
        let v2664: f64 = (if self.scalar_v802 { v2573 } else { v1 });
        let v2665: f64 = (if self.scalar_v802 { v36 } else { v1 });
        let v2666: f64 = (if self.scalar_v802 { v2571 } else { v1 });
        let v2667: f64 = (v818 * v2664);
        let v2669: f64 = (v818 * v2665);
        let v2671: f64 = (v818 * v2666);
        let v2673: f64 = (v552 * v821);
        let v2687: f64 = (if self.scalar_v802 { ((v134 * (v2664 - (if self.scalar_v802 { ((v2667 + v2667) / v2673) } else { v1 }))) - v2573) } else { v1 });
        let v2688: f64 = (if self.scalar_v802 { (v134 * (v2665 - (if self.scalar_v802 { ((v2669 + v2669) / v2673) } else { v1 }))) } else { v1 });
        let v2689: f64 = (if self.scalar_v802 { (v134 * (v2666 - (if self.scalar_v802 { ((v2671 + v2671) / v2673) } else { v1 }))) } else { v1 });
        let v2700: f64 = (self.scalar_v779 * f64::powf(v828, self.scalar_v2614));
        let v2716: f64 = (v36 - v2688);
        let v2717: f64 = (v2571 - v2689);
        let v2718: f64 = (v2650 + (-v2687));
        let v2740: f64 = ((if self.scalar_v802 { (((v829 * v2572) + (v763 * ((-(((v588 * v2687) - (v826 * v2387)) / v2466)) * v2700))) / self.scalar_v779) } else { v2631 }) + ((v838 * (self.scalar_v774 * v2718)) + (v835 * (((v784 * (self.scalar_v782 * v2718)) - (v836 * v2583)) / v2587))));
        let v2741: f64 = ((if self.scalar_v802 { ((v763 * ((-(v2688 / v588)) * v2700)) / self.scalar_v779) } else { v2632 }) + ((v838 * (self.scalar_v774 * v2716)) + (v835 * ((self.scalar_v782 * v2716) / v784))));
        let v2742: f64 = ((if self.scalar_v802 { ((v763 * ((-(v2689 / v588)) * v2700)) / self.scalar_v779) } else { v2633 }) + ((v838 * (self.scalar_v774 * v2717)) + (v835 * ((self.scalar_v782 * v2717) / v784))));
        let v2744: f64 = (if self.scalar_v802 { (v2740 - v2663) } else { v2640 });
        let v2745: f64 = (if self.scalar_v802 { v2741 } else { (if self.scalar_v767 { (v2632 + (if v791 { v1 } else { (if v771 { (v775 * ((v786 * v2575) + (v769 * v2589))) } else { v1 }) })) } else { v1 }) });
        let v2746: f64 = (if self.scalar_v802 { v2742 } else { (if self.scalar_v767 { (v2633 + (if v791 { v1 } else { (if v771 { (v775 * ((v786 * v2576) + (v769 * v2590))) } else { v1 }) })) } else { v1 }) });
        let v2747: f64 = (-v2425);
        let v2748: f64 = (self.scalar_v764 * v2747);
        let v2749: f64 = (if self.scalar_v846 { v2748 } else { v1 });
        let v2750: f64 = (if self.scalar_v846 { v2571 } else { v1 });
        let v2751: f64 = (if self.scalar_v846 { v36 } else { v1 });
        let v2758: f64 = (v615 * (self.scalar_v861 * v2749));
        let v2762: f64 = ((self.scalar_v861 * v2750) / v615);
        let v2763: f64 = ((self.scalar_v861 * v2751) / v615);
        let v2781: f64 = ((-(self.scalar_v868 * v2425)) / v2475);
        let v2785: f64 = (v2781 * (self.scalar_v858 * f64::powf(v877, self.scalar_v2782)));
        let v2790: f64 = (v881 * v881);
        let v2811: f64 = ((v615 * (-(v878 * (-((self.scalar_v858 * v2571) / v881))))) / self.scalar_v858);
        let v2812: f64 = ((v615 * (-(v878 * (-((v36 * self.scalar_v858) / v881))))) / self.scalar_v858);
        let v2813: f64 = (if v875 { (((v885 * v2425) + (v615 * (-((v883 * v2785) + (v878 * (-((-(v880 * v2425)) / v2790))))))) / self.scalar_v858) } else { (if v850 { ((v856 * v2425) / self.scalar_v858) } else { v1 }) });
        let v2822: f64 = (-(v2571 / v615));
        let v2823: f64 = (-(v36 / v615));
        let v2825: f64 = (self.scalar_v858 * f64::powf(v892, self.scalar_v2782));
        let v2840: f64 = (if v890 { (((v894 * v2425) + (v615 * (-((-((-(v731 * v2425)) / v2475)) * v2825)))) / self.scalar_v858) } else { v2813 });
        let v2841: f64 = (if v890 { ((v615 * (-(v2822 * v2825))) / self.scalar_v858) } else { (if v875 { v2811 } else { v1 }) });
        let v2842: f64 = (if v890 { ((v615 * (-(v2823 * v2825))) / self.scalar_v858) } else { (if v875 { v2812 } else { v1 }) });
        let v2849: f64 = (if self.scalar_v846 { (v2840 + (if v874 { v1 } else { (if v850 { (v853 * ((v864 * v2749) + (v848 * ((v2758 - (v862 * v2425)) / v2475)))) } else { v1 }) })) } else { v1 });
        let v2852: f64 = (-v2748);
        let v2853: f64 = (v907 * v2748);
        let v2856: f64 = (v907 * v907);
        let v2858: f64 = (if self.scalar_v905 { ((v2853 - (v906 * v2852)) / v2856) } else { v1 });
        let v2860: f64 = (v911 * v2858);
        let v2864: f64 = (v917 * v2858);
        let v2873: f64 = (((v923 * (v552 * v2858)) - (v910 * (((v2860 + v2860) / (v552 * v916)) + ((v2864 + v2864) / (v552 * v922))))) / (v923 * v923));
        let v2880: f64 = (if self.scalar_v905 { (v134 * (((v925 * v2852) + (v907 * (if self.scalar_v905 { v2873 } else { v1 }))) - v2748)) } else { v1 });
        let v2893: f64 = (((v934 * v2425) + (v615 * (-((-(((v615 * v2880) - (v930 * v2425)) / v2475)) * (self.scalar_v858 * f64::powf(v932, self.scalar_v2782)))))) / self.scalar_v858);
        let v2894: f64 = (if self.scalar_v905 { v2893 } else { v1 });
        let v2902: f64 = (if self.scalar_v905 { ((v2853 - (v940 * v2852)) / v2856) } else { v1 });
        let v2903: f64 = (if self.scalar_v905 { ((v552 * v2571) / v907) } else { v1 });
        let v2904: f64 = (if self.scalar_v905 { ((v36 * v552) / v907) } else { v1 });
        let v2906: f64 = (v552 * v2903);
        let v2907: f64 = (v552 * v2904);
        let v2908: f64 = (v944 * v2902);
        let v2910: f64 = (v944 * v2903);
        let v2912: f64 = (v944 * v2904);
        let v2914: f64 = (v552 * v947);
        let v2918: f64 = (v948 * v2902);
        let v2920: f64 = (v948 * v2903);
        let v2922: f64 = (v948 * v2904);
        let v2924: f64 = (v552 * v951);
        let v2934: f64 = (v952 * v952);
        let v2944: f64 = (if self.scalar_v905 { (((v952 * (v552 * v2902)) - (v943 * (((v2908 + v2908) / v2914) + ((v2918 + v2918) / v2924)))) / v2934) } else { v1 });
        let v2945: f64 = (if self.scalar_v905 { (((v952 * v2906) - (v943 * (((v2910 + v2910) / v2914) + ((v2920 + v2920) / v2924)))) / v2934) } else { v1 });
        let v2946: f64 = (if self.scalar_v905 { (((v952 * v2907) - (v943 * (((v2912 + v2912) / v2914) + ((v2922 + v2922) / v2924)))) / v2934) } else { v1 });
        let v2956: f64 = (if self.scalar_v905 { (v134 * (((v954 * v2852) + (v907 * v2944)) - v2748)) } else { v1 });
        let v2957: f64 = (if self.scalar_v905 { (v134 * (v907 * v2945)) } else { v1 });
        let v2958: f64 = (if self.scalar_v905 { (v134 * (v907 * v2946)) } else { v1 });
        let v2969: f64 = (self.scalar_v858 * f64::powf(v961, self.scalar_v2782));
        let v2984: f64 = (if self.scalar_v905 { (((v963 * v2425) + (v615 * (-((-(((v615 * v2956) - (v959 * v2425)) / v2475)) * v2969)))) / self.scalar_v858) } else { v2840 });
        let v2985: f64 = (if self.scalar_v905 { ((v615 * (-((-(v2957 / v615)) * v2969))) / self.scalar_v858) } else { v2841 });
        let v2986: f64 = (if self.scalar_v905 { ((v615 * (-((-(v2958 / v615)) * v2969))) / self.scalar_v858) } else { v2842 });
        let v2990: f64 = (if self.scalar_v905 { (v134 * v2944) } else { v1 });
        let v2991: f64 = (if self.scalar_v905 { (v134 * v2945) } else { v1 });
        let v2992: f64 = (if self.scalar_v905 { (v134 * v2946) } else { v1 });
        let v2997: f64 = (if self.scalar_v905 { (v2781 * (self.scalar_v970 * f64::powf(v877, self.scalar_v2993))) } else { v1 });
        let v3005: f64 = (if self.scalar_v905 { ((((v615 * v2748) - (v844 * v2425)) / v2475) * (self.scalar_v970 * f64::powf(v974, self.scalar_v2993))) } else { v1 });
        let v3031: f64 = ((v983 * (if self.scalar_v905 { (((v977 * v2997) + (v972 * (-v2990))) + ((v976 * v2990) + (v969 * v3005))) } else { v1 })) + (v981 * (v2880 + (-v2956))));
        let v3046: f64 = (if self.scalar_v905 { (v2985 + (if self.scalar_v905 { ((v983 * (if self.scalar_v905 { ((v972 * (-v2991)) + (v976 * v2991)) } else { v1 })) + (v981 * (v2571 - v2957))) } else { v1 })) } else { (if self.scalar_v846 { (v2841 + (if v874 { v1 } else { (if v850 { (v853 * ((v864 * v2750) + (v848 * v2762))) } else { v1 }) })) } else { v1 }) });
        let v3047: f64 = (if self.scalar_v905 { (v2986 + (if self.scalar_v905 { ((v983 * (if self.scalar_v905 { ((v972 * (-v2992)) + (v976 * v2992)) } else { v1 })) + (v981 * (v36 - v2958))) } else { v1 })) } else { (if self.scalar_v846 { (v2842 + (if v874 { v1 } else { (if v850 { (v853 * ((v864 * v2751) + (v848 * v2763))) } else { v1 }) })) } else { v1 }) });
        let v3048: f64 = (v844 * v2748);
        let v3055: f64 = (if self.scalar_v990 { (v560 * (v2748 + (if self.scalar_v990 { ((v3048 + v3048) / (v552 * v993)) } else { v1 }))) } else { v2880 });
        let v3068: f64 = (if self.scalar_v990 { (((v1000 * v2747) + (v843 * ((-(((v615 * v3055) - (v997 * v2425)) / v2475)) * (self.scalar_v858 * f64::powf(v999, self.scalar_v2782))))) / self.scalar_v858) } else { v1 });
        let v3069: f64 = (if self.scalar_v990 { v2748 } else { v1 });
        let v3070: f64 = (if self.scalar_v990 { v2571 } else { v1 });
        let v3071: f64 = (if self.scalar_v990 { v36 } else { v1 });
        let v3072: f64 = (v1004 * v3069);
        let v3074: f64 = (v1004 * v3070);
        let v3076: f64 = (v1004 * v3071);
        let v3078: f64 = (v552 * v1007);
        let v3092: f64 = (if self.scalar_v990 { ((v134 * (v3069 - (if self.scalar_v990 { ((v3072 + v3072) / v3078) } else { v1 }))) - v2748) } else { v2956 });
        let v3093: f64 = (if self.scalar_v990 { (v134 * (v3070 - (if self.scalar_v990 { ((v3074 + v3074) / v3078) } else { v1 }))) } else { v2957 });
        let v3094: f64 = (if self.scalar_v990 { (v134 * (v3071 - (if self.scalar_v990 { ((v3076 + v3076) / v3078) } else { v1 }))) } else { v2958 });
        let v3105: f64 = (self.scalar_v858 * f64::powf(v1014, self.scalar_v2782));
        let v3127: f64 = ((if self.scalar_v990 { (((v1015 * v2747) + (v843 * ((-(((v615 * v3092) - (v1012 * v2425)) / v2475)) * v3105))) / self.scalar_v858) } else { v2984 }) + (self.scalar_v1019 * (v3055 + (-v3092))));
        let v3131: f64 = (if self.scalar_v990 { (v3127 - v3068) } else { (if self.scalar_v905 { ((v2984 + (if self.scalar_v905 { v3031 } else { v1 })) - v2894) } else { v2849 }) });
        let v3132: f64 = (if self.scalar_v990 { ((if self.scalar_v990 { ((v843 * ((-(v3093 / v615)) * v3105)) / self.scalar_v858) } else { v2985 }) + (self.scalar_v1019 * (v2571 - v3093))) } else { v3046 });
        let v3133: f64 = (if self.scalar_v990 { ((if self.scalar_v990 { ((v843 * ((-(v3094 / v615)) * v3105)) / self.scalar_v858) } else { v2986 }) + (self.scalar_v1019 * (v36 - v3094))) } else { v3047 });
        let v3139: f64 = ((-((v529 * v2103) + (v390 * (self.scalar_v116 * v2323)))) / (v1026 * v1026));
        let v3141: f64 = (v36 * v1027);
        let v3142: f64 = (v1027 * v2571);
        let v3157: f64 = (if v1032 { ((v1037 * (v1034 * (v157 * v3139))) + (v1034 * (v1035 * v3139))) } else { (if v1028 { (v1030 * (v725 * v3139)) } else { v1 }) });
        let v3158: f64 = (if v1032 { (v1034 * v3141) } else { (if v1028 { (v1030 * v3141) } else { v1 }) });
        let v3159: f64 = (if v1032 { (v1034 * v3142) } else { (if v1028 { (v1030 * v3142) } else { v1 }) });
        let v3162: f64 = ((v1040 * v2186) + (v465 * v3157));
        let v3163: f64 = (v465 * v3158);
        let v3164: f64 = (v465 * v3159);
        let v3170: f64 = ((-((v530 * v2103) + (v390 * (self.scalar_v160 * v2323)))) / (v1042 * v1042));
        let v3172: f64 = (v1043 * v2571);
        let v3173: f64 = (v36 * v1043);
        let v3189: f64 = (if v1048 { ((v1053 * (v1050 * (v194 * v3170))) + (v1050 * (v1051 * v3170))) } else { (if v1044 { (v1046 * (v731 * v3170)) } else { v3157 }) });
        let v3190: f64 = (if v1048 { (v1050 * v3172) } else { (if v1044 { (v1046 * v3172) } else { v1 }) });
        let v3191: f64 = (if v1048 { (v1050 * v3173) } else { (if v1044 { (v1046 * v3173) } else { v3158 }) });
        let v3192: f64 = (if v1048 { v1 } else { (if v1044 { v1 } else { v3159 }) });
        let v3198: f64 = ((v1057 * ((v472 * v2186) + (v465 * v2202))) + (v1056 * v3189));
        let v3199: f64 = (v1056 * v3190);
        let v3200: f64 = (v1056 * v3191);
        let v3201: f64 = (v1056 * v3192);
        let v3206: f64 = (v715 * v2746);
        let v3210: f64 = (v712 * v3132);
        let v3212: f64 = (((v842 * (if v713 { ((-(self.scalar_v679 * (self.scalar_v680 * v2101))) / (v683 * v683)) } else { v1 })) + (v715 * v2744)) + ((v1025 * (if v710 { ((-(self.scalar_v674 * (self.scalar_v675 * v2101))) / (v678 * v678)) } else { v1 })) + (v712 * v3131)));
        let v3213: f64 = ((v715 * v2745) + (v712 * v3133));
        let v3214: f64 = (v1064 * v3212);
        let v3216: f64 = (v1064 * v3210);
        let v3218: f64 = (v1064 * v3213);
        let v3220: f64 = (v1064 * v3206);
        let v3222: f64 = (v552 * v1068);
        let v3231: f64 = (v134 * (v3212 + ((v3214 + v3214) / v3222)));
        let v3232: f64 = (v134 * (v3210 + ((v3216 + v3216) / v3222)));
        let v3233: f64 = (v134 * (v3213 + ((v3218 + v3218) / v3222)));
        let v3234: f64 = (v134 * (v3206 + ((v3220 + v3220) / v3222)));
        let v3244: f64 = (((v1041 * (if v716 { ((-(self.scalar_v129 * (v2104 * (self.scalar_v393 * f64::powf(v391, self.scalar_v2105))))) / (v395 * v395)) } else { v1 })) + (v718 * v3162)) + (v44 * v3198));
        let v3249: f64 = (self.scalar_v1077 * f64::powf(v1071, self.scalar_v3247));
        let v3254: f64 = (v136 * v3244);
        let v3255: f64 = (v136 * (v44 * v3199));
        let v3256: f64 = (v136 * ((v718 * v3163) + (v44 * v3200)));
        let v3257: f64 = (v136 * ((v718 * v3164) + (v44 * v3201)));
        let v3262: f64 = (if self.scalar_v1076 { ((v3231 * v3249) + v3254) } else { v1 });
        let v3263: f64 = (if self.scalar_v1076 { ((v3232 * v3249) + v3255) } else { v1 });
        let v3264: f64 = (if self.scalar_v1076 { ((v3233 * v3249) + v3256) } else { v1 });
        let v3265: f64 = (if self.scalar_v1076 { ((v3234 * v3249) + v3257) } else { v1 });
        let v3268: f64 = (self.scalar_v138 * f64::powf(v1081, self.scalar_v3266));
        let v3285: f64 = (v134 * v3231);
        let v3286: f64 = (v134 * v3232);
        let v3287: f64 = (v134 * v3233);
        let v3288: f64 = (v134 * v3234);
        let v3293: f64 = (if self.scalar_v1094 { v3254 } else { v3262 });
        let v3294: f64 = (if self.scalar_v1094 { v3255 } else { v3263 });
        let v3295: f64 = (if self.scalar_v1094 { v3256 } else { v3264 });
        let v3296: f64 = (if self.scalar_v1094 { v3257 } else { v3265 });
        let v3298: f64 = (self.scalar_v138 * f64::powf(v1096, self.scalar_v3266));
        let v3323: f64 = (if v1105 { (self.scalar_v1106 * v3285) } else { (if v1098 { ((v1101 * v3285) + (v1099 * (v3293 * v3298))) } else { (if v1089 { v3285 } else { (if v1083 { (v134 * (v3231 + (v3262 * v3268))) } else { v1 }) }) }) });
        let v3324: f64 = (if v1105 { (self.scalar_v1106 * v3286) } else { (if v1098 { ((v1101 * v3286) + (v1099 * (v3294 * v3298))) } else { (if v1089 { v3286 } else { (if v1083 { (v134 * (v3232 + (v3263 * v3268))) } else { v1 }) }) }) });
        let v3325: f64 = (if v1105 { (self.scalar_v1106 * v3287) } else { (if v1098 { ((v1101 * v3287) + (v1099 * (v3295 * v3298))) } else { (if v1089 { v3287 } else { (if v1083 { (v134 * (v3233 + (v3264 * v3268))) } else { v1 }) }) }) });
        let v3326: f64 = (if v1105 { (self.scalar_v1106 * v3288) } else { (if v1098 { ((v1101 * v3288) + (v1099 * (v3296 * v3298))) } else { (if v1089 { v3288 } else { (if v1083 { (v134 * (v3234 + (v3265 * v3268))) } else { v1 }) }) }) });
        let v3330: f64 = (v1108 * v1108);
        let v3331: f64 = (((v1108 * v3198) - (v1058 * v3323)) / v3330);
        let v3335: f64 = (((v1108 * v3199) - (v1058 * v3324)) / v3330);
        let v3339: f64 = (((v1108 * v3200) - (v1058 * v3325)) / v3330);
        let v3343: f64 = (((v1108 * v3201) - (v1058 * v3326)) / v3330);
        let v3347: f64 = (((v1108 * v3162) - (v1041 * v3323)) / v3330);
        let v3350: f64 = ((-(v1041 * v3324)) / v3330);
        let v3354: f64 = (((v1108 * v3163) - (v1041 * v3325)) / v3330);
        let v3358: f64 = (((v1108 * v3164) - (v1041 * v3326)) / v3330);
        let v3361: f64 = (if self.scalar_v1111 { ((-v2209) / v2213) } else { v3170 });
        let v3363: f64 = (v36 * v1113);
        let v3364: f64 = (v1113 * v2571);
        let v3375: f64 = (v1122 * (v226 * v3361));
        let v3380: f64 = (v1122 * v3363);
        let v3381: f64 = (v1122 * v3364);
        let v3382: f64 = (if v1120 { ((v1125 * v3375) + (v1122 * (v1123 * v3361))) } else { (if v1115 { (v1117 * (v739 * v3361)) } else { v3189 }) });
        let v3383: f64 = (if v1120 { v1 } else { (if v1115 { v1 } else { v3190 }) });
        let v3384: f64 = (if v1120 { v3380 } else { (if v1115 { (v1117 * v3363) } else { v1 }) });
        let v3385: f64 = (if v1120 { v1 } else { (if v1115 { v1 } else { v3191 }) });
        let v3386: f64 = (if v1120 { v1 } else { (if v1115 { v1 } else { v3192 }) });
        let v3387: f64 = (if v1120 { v3381 } else { (if v1115 { (v1117 * v3364) } else { v1 }) });
        let v3399: f64 = (if v1134 { ((v1137 * v3375) + (v1122 * (v1135 * v3361))) } else { (if v1129 { (v1131 * (v731 * v3361)) } else { v1 }) });
        let v3400: f64 = (if v1134 { v3381 } else { (if v1129 { (v1131 * v3364) } else { v1 }) });
        let v3401: f64 = (if v1134 { v3380 } else { (if v1129 { (v1131 * v3363) } else { v1 }) });
        let v3422: f64 = (if self.scalar_v1111 { ((v1145 * v2218) + (v479 * ((self.scalar_v1140 * v3382) + (self.scalar_v1142 * v3399)))) } else { v1 });
        let v3423: f64 = (if self.scalar_v1111 { (v479 * ((self.scalar_v1140 * v3383) + (self.scalar_v1142 * v3400))) } else { v1 });
        let v3424: f64 = (if self.scalar_v1111 { (v479 * (self.scalar_v1140 * v3384)) } else { v1 });
        let v3425: f64 = (if self.scalar_v1111 { (v479 * ((self.scalar_v1140 * v3385) + (self.scalar_v1142 * v3401))) } else { v1 });
        let v3426: f64 = (if self.scalar_v1111 { (v479 * (self.scalar_v1140 * v3386)) } else { v1 });
        let v3427: f64 = (if self.scalar_v1111 { (v479 * (self.scalar_v1140 * v3387)) } else { v1 });
        let v3452: f64 = (v552 * v1155);
        let v3471: f64 = (if v1160 { v1 } else { (if v1154 { (v134 * ((if self.scalar_v1111 { (v136 * (if self.scalar_v1111 { (v49 * v3422) } else { v1 })) } else { v3293 }) / v3452)) } else { v1 }) });
        let v3472: f64 = (if v1160 { v1 } else { (if v1154 { (v134 * ((if self.scalar_v1111 { (v136 * (if self.scalar_v1111 { (v49 * v3423) } else { v1 })) } else { v3294 }) / v3452)) } else { v1 }) });
        let v3473: f64 = (if v1160 { v1 } else { (if v1154 { (v134 * ((if self.scalar_v1111 { (v136 * (if self.scalar_v1111 { (v49 * v3424) } else { v1 })) } else { v1 }) / v3452)) } else { v1 }) });
        let v3474: f64 = (if v1160 { v1 } else { (if v1154 { (v134 * ((if self.scalar_v1111 { (v136 * (if self.scalar_v1111 { (v49 * v3425) } else { v1 })) } else { v3295 }) / v3452)) } else { v1 }) });
        let v3475: f64 = (if v1160 { v1 } else { (if v1154 { (v134 * ((if self.scalar_v1111 { (v136 * (if self.scalar_v1111 { (v49 * v3426) } else { v1 })) } else { v3296 }) / v3452)) } else { v1 }) });
        let v3476: f64 = (if v1160 { v1 } else { (if v1154 { (v134 * ((if self.scalar_v1111 { (v136 * (if self.scalar_v1111 { (v49 * v3427) } else { v1 })) } else { v1 }) / v3452)) } else { v1 }) });
        let v3492: f64 = (if v1169 { ((v1172 * v3375) + (v1122 * (v1170 * v3361))) } else { (if v1164 { (v1166 * (v756 * v3361)) } else { v3382 }) });
        let v3493: f64 = (if v1169 { v1 } else { (if v1164 { v1 } else { v3383 }) });
        let v3494: f64 = (if v1169 { v1 } else { (if v1164 { v1 } else { v3384 }) });
        let v3495: f64 = (if v1169 { v1 } else { (if v1164 { v1 } else { v3385 }) });
        let v3496: f64 = (if v1169 { v1 } else { (if v1164 { v1 } else { v3386 }) });
        let v3497: f64 = (if v1169 { v3381 } else { (if v1164 { (v1166 * v3364) } else { v3387 }) });
        let v3498: f64 = (if v1169 { v3380 } else { (if v1164 { (v1166 * v3363) } else { v1 }) });
        let v3525: f64 = (v1162 * v1162);
        let v3567: f64 = (if self.scalar_v1181 { v1 } else { (if self.scalar_v1111 { (((v1162 * (v3422 - (if self.scalar_v1111 { ((v1175 * v2218) + (v479 * v3492)) } else { v1 }))) - (v1178 * v3471)) / v3525) } else { v1 }) });
        let v3568: f64 = (if self.scalar_v1181 { v1 } else { (if self.scalar_v1111 { (((v1162 * (v3423 - (if self.scalar_v1111 { (v479 * v3493) } else { v1 }))) - (v1178 * v3472)) / v3525) } else { v1 }) });
        let v3569: f64 = (if self.scalar_v1181 { v1 } else { (if self.scalar_v1111 { (((v1162 * (v3424 - (if self.scalar_v1111 { (v479 * v3494) } else { v1 }))) - (v1178 * v3473)) / v3525) } else { v1 }) });
        let v3570: f64 = (if self.scalar_v1181 { v1 } else { (if self.scalar_v1111 { (((v1162 * (v3425 - (if self.scalar_v1111 { (v479 * v3495) } else { v1 }))) - (v1178 * v3474)) / v3525) } else { v1 }) });
        let v3571: f64 = (if self.scalar_v1181 { v1 } else { (if self.scalar_v1111 { (((v1162 * (v3426 - (if self.scalar_v1111 { (v479 * v3496) } else { v1 }))) - (v1178 * v3475)) / v3525) } else { v1 }) });
        let v3572: f64 = (if self.scalar_v1181 { v1 } else { (if self.scalar_v1111 { (((v1162 * (v3427 - (if self.scalar_v1111 { (v479 * v3497) } else { v1 }))) - (v1178 * v3476)) / v3525) } else { v1 }) });
        let v3573: f64 = (if self.scalar_v1181 { v1 } else { (if self.scalar_v1111 { ((-(if self.scalar_v1111 { (v479 * v3498) } else { v1 })) / v1162) } else { v1 }) });
        let v3575: f64 = ((-v2225) / v2229);
        let v3576: f64 = (if self.scalar_v1186 { v3575 } else { v3361 });
        let v3578: f64 = (v36 * v1188);
        let v3579: f64 = (v1188 * v2571);
        let v3598: f64 = (if v1195 { ((v1200 * (v1197 * (v247 * v3576))) + (v1197 * (v1198 * v3576))) } else { (if v1190 { (v1192 * (v725 * v3576)) } else { v3492 }) });
        let v3599: f64 = (if v1195 { v1 } else { (if v1190 { v1 } else { v3493 }) });
        let v3600: f64 = (if v1195 { v1 } else { (if v1190 { v1 } else { v3494 }) });
        let v3601: f64 = (if v1195 { (v1197 * v3578) } else { (if v1190 { (v1192 * v3578) } else { v3495 }) });
        let v3602: f64 = (if v1195 { (v1197 * v3579) } else { (if v1190 { (v1192 * v3579) } else { v3496 }) });
        let v3603: f64 = (if v1195 { v1 } else { (if v1190 { v1 } else { v3497 }) });
        let v3604: f64 = (if v1195 { v1 } else { (if v1190 { v1 } else { v3498 }) });
        let v3606: f64 = ((-v2241) / v2245);
        let v3607: f64 = (if self.scalar_v1186 { v3606 } else { v3576 });
        let v3609: f64 = (v36 * v1204);
        let v3610: f64 = (v1204 * v2571);
        let v3625: f64 = (if v1211 { ((v1216 * (v1213 * (v268 * v3607))) + (v1213 * (v1214 * v3607))) } else { (if v1206 { (v1208 * (v725 * v3607)) } else { v1 }) });
        let v3626: f64 = (if v1211 { (v1213 * v3609) } else { (if v1206 { (v1208 * v3609) } else { v1 }) });
        let v3627: f64 = (if v1211 { (v1213 * v3610) } else { (if v1206 { (v1208 * v3610) } else { v1 }) });
        let v3634: f64 = ((v1224 * v2234) + (v486 * (self.scalar_v1219 * v3231)));
        let v3635: f64 = (v486 * (self.scalar_v1219 * v3232));
        let v3636: f64 = (v486 * (self.scalar_v1219 * v3233));
        let v3637: f64 = (v486 * (self.scalar_v1219 * v3234));
        let v3655: f64 = ((v1228 * v2250) + (v493 * v3625));
        let v3656: f64 = (v493 * v3626);
        let v3657: f64 = (v493 * v3627);
        let v3680: f64 = (if self.scalar_v1233 { (v3655 + ((v1226 * v2234) + (v486 * v3598))) } else { (if self.scalar_v1221 { (((v1226 * v3634) + (v1225 * v3598)) + v3655) } else { v1 }) });
        let v3681: f64 = (if self.scalar_v1233 { (v486 * v3599) } else { (if self.scalar_v1221 { ((v1226 * v3635) + (v1225 * v3599)) } else { v1 }) });
        let v3683: f64 = (if self.scalar_v1233 { (v3656 + (v486 * v3601)) } else { (if self.scalar_v1221 { (((v1226 * v3636) + (v1225 * v3601)) + v3656) } else { v1 }) });
        let v3684: f64 = (if self.scalar_v1233 { (v3657 + (v486 * v3602)) } else { (if self.scalar_v1221 { (((v1226 * v3637) + (v1225 * v3602)) + v3657) } else { v1 }) });
        let v3687: f64 = (if self.scalar_v1238 { v2509 } else { v1 });
        let v3688: f64 = (if self.scalar_v1238 { v2571 } else { v1 });
        let v3689: f64 = (if self.scalar_v1238 { v36 } else { v1 });
        let v3691: f64 = ((-v2512) / v2516);
        let v3692: f64 = (if self.scalar_v1238 { v3691 } else { v3607 });
        let v3693: f64 = (v1242 * v3687);
        let v3696: f64 = (v1242 * v3688);
        let v3697: f64 = (v1242 * v3689);
        let v3714: f64 = (if v1249 { ((v1254 * (v1251 * (v114 * v3692))) + (v1251 * (v3693 + (v1252 * v3692)))) } else { (if v1244 { (v1246 * (v3693 + (v1240 * v3692))) } else { v3399 }) });
        let v3715: f64 = (if v1249 { v1 } else { (if v1244 { v1 } else { v3400 }) });
        let v3716: f64 = (if v1249 { (v1251 * v3696) } else { (if v1244 { (v1246 * v3696) } else { v3401 }) });
        let v3717: f64 = (if v1249 { (v1251 * v3697) } else { (if v1244 { (v1246 * v3697) } else { v1 }) });
        let v3738: f64 = (if self.scalar_v1263 { v3575 } else { v3692 });
        let v3740: f64 = (v36 * v1265);
        let v3741: f64 = (v1265 * v2571);
        let v3760: f64 = (if v1272 { ((v1277 * (v1274 * (v247 * v3738))) + (v1274 * (v1275 * v3738))) } else { (if v1267 { (v1269 * (v728 * v3738)) } else { v3598 }) });
        let v3761: f64 = (if v1272 { v1 } else { (if v1267 { v1 } else { v3599 }) });
        let v3762: f64 = (if v1272 { (v1274 * v3740) } else { (if v1267 { (v1269 * v3740) } else { v3600 }) });
        let v3763: f64 = (if v1272 { v1 } else { (if v1267 { v1 } else { v3601 }) });
        let v3764: f64 = (if v1272 { (v1274 * v3741) } else { (if v1267 { (v1269 * v3741) } else { v3602 }) });
        let v3765: f64 = (if v1272 { v1 } else { (if v1267 { v1 } else { v3603 }) });
        let v3766: f64 = (if v1272 { v1 } else { (if v1267 { v1 } else { v3604 }) });
        let v3767: f64 = (if self.scalar_v1263 { v3606 } else { v3738 });
        let v3769: f64 = (v36 * v1280);
        let v3770: f64 = (v1280 * v2571);
        let v3786: f64 = (if v1287 { ((v1292 * (v1289 * (v268 * v3767))) + (v1289 * (v1290 * v3767))) } else { (if v1282 { (v1284 * (v728 * v3767)) } else { v3625 }) });
        let v3787: f64 = (if v1287 { (v1289 * v3769) } else { (if v1282 { (v1284 * v3769) } else { v1 }) });
        let v3788: f64 = (if v1287 { v1 } else { (if v1282 { v1 } else { v3626 }) });
        let v3789: f64 = (if v1287 { (v1289 * v3770) } else { (if v1282 { (v1284 * v3770) } else { v3627 }) });
        let v3809: f64 = (if self.scalar_v1263 { (((v1295 * v2234) + (v486 * v3760)) + ((v1297 * v2250) + (v493 * v3786))) } else { v1 });
        let v3810: f64 = (if self.scalar_v1263 { (v486 * v3761) } else { v1 });
        let v3812: f64 = (if self.scalar_v1263 { ((v486 * v3763) + (v493 * v3788)) } else { v1 });
        let v3813: f64 = (if self.scalar_v1263 { ((v486 * v3764) + (v493 * v3789)) } else { v1 });
        let v3816: f64 = (if self.scalar_v1301 { v2509 } else { v3687 });
        let v3817: f64 = (if self.scalar_v1301 { v2571 } else { v3688 });
        let v3818: f64 = (if self.scalar_v1301 { v36 } else { v3689 });
        let v3819: f64 = (if self.scalar_v1301 { v3691 } else { v3767 });
        let v3820: f64 = (v1303 * v3816);
        let v3823: f64 = (v1303 * v3817);
        let v3824: f64 = (v1303 * v3818);
        let v3841: f64 = (if v1310 { ((v1315 * (v1312 * (v114 * v3819))) + (v1312 * (v3820 + (v1313 * v3819)))) } else { (if v1305 { (v1307 * (v3820 + (v1302 * v3819))) } else { v3714 }) });
        let v3842: f64 = (if v1310 { v1 } else { (if v1305 { v1 } else { v3715 }) });
        let v3843: f64 = (if v1310 { (v1312 * v3823) } else { (if v1305 { (v1307 * v3823) } else { v3716 }) });
        let v3844: f64 = (if v1310 { (v1312 * v3824) } else { (if v1305 { (v1307 * v3824) } else { v3717 }) });
        let v3858: f64 = (if self.scalar_v1323 { v3575 } else { v3819 });
        let v3860: f64 = (v36 * v1324);
        let v3861: f64 = (v1324 * v2571);
        let v3880: f64 = (if v1329 { ((v1333 * (v1331 * (v247 * v3858))) + (v1331 * (v1198 * v3858))) } else { (if v1325 { (v1327 * (v725 * v3858)) } else { v3760 }) });
        let v3881: f64 = (if v1329 { v1 } else { (if v1325 { v1 } else { v3761 }) });
        let v3882: f64 = (if v1329 { v1 } else { (if v1325 { v1 } else { v3762 }) });
        let v3883: f64 = (if v1329 { (v1331 * v3860) } else { (if v1325 { (v1327 * v3860) } else { v3763 }) });
        let v3884: f64 = (if v1329 { (v1331 * v3861) } else { (if v1325 { (v1327 * v3861) } else { v3764 }) });
        let v3885: f64 = (if v1329 { v1 } else { (if v1325 { v1 } else { v3765 }) });
        let v3886: f64 = (if v1329 { v1 } else { (if v1325 { v1 } else { v3766 }) });
        let v3887: f64 = (if self.scalar_v1323 { v3606 } else { v3858 });
        let v3889: f64 = (v36 * v1336);
        let v3890: f64 = (v1336 * v2571);
        let v3906: f64 = (if v1341 { ((v1345 * (v1343 * (v268 * v3887))) + (v1343 * (v1214 * v3887))) } else { (if v1337 { (v1339 * (v725 * v3887)) } else { v3786 }) });
        let v3907: f64 = (if v1341 { v1 } else { (if v1337 { v1 } else { v3787 }) });
        let v3908: f64 = (if v1341 { (v1343 * v3889) } else { (if v1337 { (v1339 * v3889) } else { v3788 }) });
        let v3909: f64 = (if v1341 { (v1343 * v3890) } else { (if v1337 { (v1339 * v3890) } else { v3789 }) });
        let v3927: f64 = ((v1351 * v2250) + (v493 * v3906));
        let v3928: f64 = (v493 * v3907);
        let v3929: f64 = (v493 * v3908);
        let v3930: f64 = (v493 * v3909);
        let v3942: f64 = (if self.scalar_v1348 { (self.scalar_v1185 * (((v1349 * v3634) + (v1225 * v3880)) + v3927)) } else { (if self.scalar_v1263 { v1 } else { (if self.scalar_v1238 { (v3680 - (self.scalar_v100 * (v3714 - v2518))) } else { v3680 }) }) });
        let v3969: f64 = (if self.scalar_v1356 { (self.scalar_v1185 * (v3927 + ((v1349 * v2234) + (v486 * v3880)))) } else { v3942 });
        let v3970: f64 = (if self.scalar_v1356 { (self.scalar_v1185 * (v486 * v3881)) } else { (if self.scalar_v1348 { (self.scalar_v1185 * ((v1349 * v3635) + (v1225 * v3881))) } else { (if self.scalar_v1263 { v1 } else { (if self.scalar_v1238 { (v3681 - (self.scalar_v100 * v3715)) } else { v3681 }) }) }) });
        let v3971: f64 = (if self.scalar_v1356 { (self.scalar_v1185 * (v3928 + (v486 * v3882))) } else { (if self.scalar_v1348 { (self.scalar_v1185 * ((v1225 * v3882) + v3928)) } else { (if self.scalar_v1263 { v1 } else { (if self.scalar_v1233 { (v486 * v3600) } else { (if self.scalar_v1221 { (v1225 * v3600) } else { v1 }) }) }) }) });
        let v3972: f64 = (if self.scalar_v1356 { (self.scalar_v1185 * (v3929 + (v486 * v3883))) } else { (if self.scalar_v1348 { (self.scalar_v1185 * (((v1349 * v3636) + (v1225 * v3883)) + v3929)) } else { (if self.scalar_v1263 { v1 } else { (if self.scalar_v1238 { (v3683 - (self.scalar_v100 * v3716)) } else { v3683 }) }) }) });
        let v3973: f64 = (if self.scalar_v1356 { (self.scalar_v1185 * (v3930 + (v486 * v3884))) } else { (if self.scalar_v1348 { (self.scalar_v1185 * (((v1349 * v3637) + (v1225 * v3884)) + v3930)) } else { (if self.scalar_v1263 { v1 } else { (if self.scalar_v1238 { (v3684 - (self.scalar_v100 * v3717)) } else { v3684 }) }) }) });
        let v3974: f64 = (if self.scalar_v1356 { (self.scalar_v1185 * (v486 * v3885)) } else { (if self.scalar_v1348 { (self.scalar_v1185 * (v1225 * v3885)) } else { (if self.scalar_v1263 { v1 } else { (if self.scalar_v1233 { (v486 * v3603) } else { (if self.scalar_v1221 { (v1225 * v3603) } else { v1 }) }) }) }) });
        let v3975: f64 = (if self.scalar_v1356 { (self.scalar_v1185 * (v486 * v3886)) } else { (if self.scalar_v1348 { (self.scalar_v1185 * (v1225 * v3886)) } else { (if self.scalar_v1263 { v1 } else { (if self.scalar_v1233 { (v486 * v3604) } else { (if self.scalar_v1221 { (v1225 * v3604) } else { v1 }) }) }) }) });
        let v3976: f64 = (if self.scalar_v1361 { v2509 } else { v3816 });
        let v3977: f64 = (if self.scalar_v1361 { v2571 } else { v3817 });
        let v3978: f64 = (if self.scalar_v1361 { v36 } else { v3818 });
        let v3979: f64 = (if self.scalar_v1361 { v3691 } else { v3887 });
        let v3980: f64 = (v1363 * v3976);
        let v3983: f64 = (v1363 * v3977);
        let v3984: f64 = (v1363 * v3978);
        let v4001: f64 = (if v1370 { ((v1375 * (v1372 * (v114 * v3979))) + (v1372 * (v3980 + (v1373 * v3979)))) } else { (if v1365 { (v1367 * (v3980 + (v1362 * v3979))) } else { v3841 }) });
        let v4002: f64 = (if v1370 { v1 } else { (if v1365 { v1 } else { v3842 }) });
        let v4003: f64 = (if v1370 { (v1372 * v3983) } else { (if v1365 { (v1367 * v3983) } else { v3843 }) });
        let v4004: f64 = (if v1370 { (v1372 * v3984) } else { (if v1365 { (v1367 * v3984) } else { v3844 }) });
        let v4014: f64 = (if self.scalar_v1361 { (v3969 - (self.scalar_v1378 * (v4001 - v2518))) } else { v3969 });
        let v4015: f64 = (if self.scalar_v1361 { (v3970 - (self.scalar_v1378 * v4002)) } else { v3970 });
        let v4016: f64 = (if self.scalar_v1361 { (v3972 - (self.scalar_v1378 * v4003)) } else { v3972 });
        let v4017: f64 = (if self.scalar_v1361 { (v3973 - (self.scalar_v1378 * v4004)) } else { v3973 });
        let v4018: f64 = (if self.scalar_v1323 { v3575 } else { v3979 });
        let v4020: f64 = (v36 * v1383);
        let v4021: f64 = (v1383 * v2571);
        let v4040: f64 = (if v1388 { ((v1392 * (v1390 * (v247 * v4018))) + (v1390 * (v1275 * v4018))) } else { (if v1384 { (v1386 * (v728 * v4018)) } else { v3880 }) });
        let v4041: f64 = (if v1388 { v1 } else { (if v1384 { v1 } else { v3881 }) });
        let v4042: f64 = (if v1388 { (v1390 * v4020) } else { (if v1384 { (v1386 * v4020) } else { v3882 }) });
        let v4043: f64 = (if v1388 { v1 } else { (if v1384 { v1 } else { v3883 }) });
        let v4044: f64 = (if v1388 { (v1390 * v4021) } else { (if v1384 { (v1386 * v4021) } else { v3884 }) });
        let v4045: f64 = (if v1388 { v1 } else { (if v1384 { v1 } else { v3885 }) });
        let v4046: f64 = (if v1388 { v1 } else { (if v1384 { v1 } else { v3886 }) });
        let v4047: f64 = (if self.scalar_v1323 { v3606 } else { v4018 });
        let v4049: f64 = (v36 * v1395);
        let v4050: f64 = (v1395 * v2571);
        let v4066: f64 = (if v1400 { ((v1404 * (v1402 * (v268 * v4047))) + (v1402 * (v1290 * v4047))) } else { (if v1396 { (v1398 * (v728 * v4047)) } else { v3906 }) });
        let v4067: f64 = (if v1400 { (v1402 * v4049) } else { (if v1396 { (v1398 * v4049) } else { v3907 }) });
        let v4068: f64 = (if v1400 { v1 } else { (if v1396 { v1 } else { v3908 }) });
        let v4069: f64 = (if v1400 { (v1402 * v4050) } else { (if v1396 { (v1398 * v4050) } else { v3909 }) });
        let v4096: f64 = (if self.scalar_v1323 { (self.scalar_v1407 * (((v1408 * v2234) + (v486 * v4040)) + ((v1410 * v2250) + (v493 * v4066)))) } else { (if self.scalar_v1301 { (v3809 - (self.scalar_v100 * (v3841 - v2518))) } else { v3809 }) });
        let v4097: f64 = (if self.scalar_v1323 { (self.scalar_v1407 * (v486 * v4041)) } else { (if self.scalar_v1301 { (v3810 - (self.scalar_v100 * v3842)) } else { v3810 }) });
        let v4098: f64 = (if self.scalar_v1323 { (self.scalar_v1407 * ((v486 * v4042) + (v493 * v4067))) } else { (if self.scalar_v1263 { ((v486 * v3762) + (v493 * v3787)) } else { v1 }) });
        let v4099: f64 = (if self.scalar_v1323 { (self.scalar_v1407 * ((v486 * v4043) + (v493 * v4068))) } else { (if self.scalar_v1301 { (v3812 - (self.scalar_v100 * v3843)) } else { v3812 }) });
        let v4100: f64 = (if self.scalar_v1323 { (self.scalar_v1407 * ((v486 * v4044) + (v493 * v4069))) } else { (if self.scalar_v1301 { (v3813 - (self.scalar_v100 * v3844)) } else { v3813 }) });
        let v4101: f64 = (if self.scalar_v1323 { (self.scalar_v1407 * (v486 * v4045)) } else { (if self.scalar_v1263 { (v486 * v3765) } else { v1 }) });
        let v4102: f64 = (if self.scalar_v1323 { (self.scalar_v1407 * (v486 * v4046)) } else { (if self.scalar_v1263 { (v486 * v3766) } else { v1 }) });
        let v4106: f64 = (if self.scalar_v1361 { v3691 } else { v4047 });
        let v4107: f64 = (v1416 * (if self.scalar_v1361 { v2509 } else { v3976 }));
        let v4110: f64 = (v1416 * (if self.scalar_v1361 { v2571 } else { v3977 }));
        let v4111: f64 = (v1416 * (if self.scalar_v1361 { v36 } else { v3978 }));
        let v4128: f64 = (if v1423 { ((v1428 * (v1425 * (v114 * v4106))) + (v1425 * (v4107 + (v1426 * v4106)))) } else { (if v1418 { (v1420 * (v4107 + (v1415 * v4106))) } else { v4001 }) });
        let v4129: f64 = (if v1423 { v1 } else { (if v1418 { v1 } else { v4002 }) });
        let v4130: f64 = (if v1423 { (v1425 * v4110) } else { (if v1418 { (v1420 * v4110) } else { v4003 }) });
        let v4131: f64 = (if v1423 { (v1425 * v4111) } else { (if v1418 { (v1420 * v4111) } else { v4004 }) });
        let v4141: f64 = (if self.scalar_v1361 { (v4096 - (self.scalar_v1431 * (v4128 - v2518))) } else { v4096 });
        let v4142: f64 = (if self.scalar_v1361 { (v4097 - (self.scalar_v1431 * v4129)) } else { v4097 });
        let v4143: f64 = (if self.scalar_v1361 { (v4099 - (self.scalar_v1431 * v4130)) } else { v4099 });
        let v4144: f64 = (if self.scalar_v1361 { (v4100 - (self.scalar_v1431 * v4131)) } else { v4100 });
        let v4146: f64 = ((-v2257) / v2261);
        let v4148: f64 = (v1436 * v2571);
        let v4149: f64 = (v36 * v1436);
        let v4168: f64 = (if v1441 { ((v1446 * (v1443 * (v288 * v4146))) + (v1443 * (v1444 * v4146))) } else { (if v1437 { (v1439 * (v731 * v4146)) } else { v4040 }) });
        let v4169: f64 = (if v1441 { (v1443 * v4148) } else { (if v1437 { (v1439 * v4148) } else { v4041 }) });
        let v4170: f64 = (if v1441 { v1 } else { (if v1437 { v1 } else { v4042 }) });
        let v4171: f64 = (if v1441 { (v1443 * v4149) } else { (if v1437 { (v1439 * v4149) } else { v4043 }) });
        let v4172: f64 = (if v1441 { v1 } else { (if v1437 { v1 } else { v4044 }) });
        let v4173: f64 = (if v1441 { v1 } else { (if v1437 { v1 } else { v4045 }) });
        let v4174: f64 = (if v1441 { v1 } else { (if v1437 { v1 } else { v4046 }) });
        let v4176: f64 = ((-v2273) / v2277);
        let v4178: f64 = (v1449 * v2571);
        let v4179: f64 = (v36 * v1449);
        let v4196: f64 = (if v1454 { ((v1459 * (v1456 * (v308 * v4176))) + (v1456 * (v1457 * v4176))) } else { (if v1450 { (v1452 * (v731 * v4176)) } else { v4066 }) });
        let v4197: f64 = (if v1454 { (v1456 * v4178) } else { (if v1450 { (v1452 * v4178) } else { v1 }) });
        let v4198: f64 = (if v1454 { v1 } else { (if v1450 { v1 } else { v4067 }) });
        let v4199: f64 = (if v1454 { (v1456 * v4179) } else { (if v1450 { (v1452 * v4179) } else { v4068 }) });
        let v4200: f64 = (if v1454 { v1 } else { (if v1450 { v1 } else { v4069 }) });
        let v4208: f64 = (v500 * v4173);
        let v4209: f64 = (v500 * v4174);
        let v4217: f64 = (((v1462 * ((v499 * (self.scalar_v269 * v2254)) + (v495 * v2263))) + (v500 * v4168)) + ((v1464 * ((v506 * (self.scalar_v289 * v2270)) + (v502 * v2279))) + (v507 * v4196)));
        let v4218: f64 = ((v500 * v4169) + (v507 * v4197));
        let v4219: f64 = ((v500 * v4170) + (v507 * v4198));
        let v4220: f64 = ((v500 * v4171) + (v507 * v4199));
        let v4221: f64 = ((v500 * v4172) + (v507 * v4200));
        let v4222: f64 = (if self.scalar_v1469 { v4146 } else { v4176 });
        let v4224: f64 = (v36 * v1470);
        let v4225: f64 = (v1470 * v2571);
        let v4244: f64 = (if v1477 { ((v1482 * (v1479 * (v319 * v4222))) + (v1479 * (v1480 * v4222))) } else { (if v1472 { (v1474 * (v739 * v4222)) } else { v4168 }) });
        let v4245: f64 = (if v1477 { v1 } else { (if v1472 { v1 } else { v4169 }) });
        let v4246: f64 = (if v1477 { (v1479 * v4224) } else { (if v1472 { (v1474 * v4224) } else { v4170 }) });
        let v4247: f64 = (if v1477 { v1 } else { (if v1472 { v1 } else { v4171 }) });
        let v4248: f64 = (if v1477 { v1 } else { (if v1472 { v1 } else { v4172 }) });
        let v4249: f64 = (if v1477 { (v1479 * v4225) } else { (if v1472 { (v1474 * v4225) } else { v4173 }) });
        let v4250: f64 = (if v1477 { v1 } else { (if v1472 { v1 } else { v4174 }) });
        let v4251: f64 = (if self.scalar_v1469 { v4176 } else { v4222 });
        let v4253: f64 = (v36 * v1485);
        let v4254: f64 = (v1485 * v2571);
        let v4272: f64 = (if v1492 { ((v1497 * (v1494 * (v330 * v4251))) + (v1494 * (v1495 * v4251))) } else { (if v1487 { (v1489 * (v739 * v4251)) } else { v4196 }) });
        let v4273: f64 = (if v1492 { v1 } else { (if v1487 { v1 } else { v4197 }) });
        let v4274: f64 = (if v1492 { (v1494 * v4253) } else { (if v1487 { (v1489 * v4253) } else { v4198 }) });
        let v4275: f64 = (if v1492 { v1 } else { (if v1487 { v1 } else { v4199 }) });
        let v4276: f64 = (if v1492 { v1 } else { (if v1487 { v1 } else { v4200 }) });
        let v4277: f64 = (if v1492 { (v1494 * v4254) } else { (if v1487 { (v1489 * v4254) } else { v1 }) });
        let v4295: f64 = (((v1500 * ((v508 * v2263) + (v499 * (self.scalar_v309 * v2254)))) + (v509 * v4244)) + ((v1502 * ((v510 * v2279) + (v506 * (self.scalar_v320 * v2270)))) + (v511 * v4272)));
        let v4308: f64 = (if self.scalar_v1506 { v1 } else { (if self.scalar_v1469 { v4295 } else { v1 }) });
        let v4309: f64 = (if self.scalar_v1506 { v1 } else { (if self.scalar_v1469 { ((v509 * v4245) + (v511 * v4273)) } else { v1 }) });
        let v4310: f64 = (if self.scalar_v1506 { v1 } else { (if self.scalar_v1469 { ((v509 * v4246) + (v511 * v4274)) } else { v1 }) });
        let v4311: f64 = (if self.scalar_v1506 { v1 } else { (if self.scalar_v1469 { ((v509 * v4247) + (v511 * v4275)) } else { v1 }) });
        let v4312: f64 = (if self.scalar_v1506 { v1 } else { (if self.scalar_v1469 { ((v509 * v4248) + (v511 * v4276)) } else { v1 }) });
        let v4313: f64 = (if self.scalar_v1506 { v1 } else { (if self.scalar_v1469 { ((v509 * v4249) + (v511 * v4277)) } else { v1 }) });
        let v4314: f64 = (if self.scalar_v1506 { v1 } else { (if self.scalar_v1469 { (v509 * v4250) } else { v1 }) });
        let v4317: f64 = ((-(v731 * v2103)) / v2347);
        let v4318: f64 = (v2571 / v390);
        let v4319: f64 = (v36 / v390);
        let v4331: f64 = (v1513 * v4318);
        let v4332: f64 = (v1513 * v4319);
        let v4333: f64 = (if v1512 { (v1513 * v4317) } else { (if v1509 { (v1510 * v4317) } else { v4244 }) });
        let v4334: f64 = (if v1512 { v4331 } else { (if v1509 { (v1510 * v4318) } else { v4245 }) });
        let v4335: f64 = (if v1512 { v1 } else { (if v1509 { v1 } else { v4246 }) });
        let v4336: f64 = (if v1512 { v4332 } else { (if v1509 { (v1510 * v4319) } else { v4247 }) });
        let v4337: f64 = (if v1512 { v1 } else { (if v1509 { v1 } else { v4248 }) });
        let v4338: f64 = (if v1512 { v1 } else { (if v1509 { v1 } else { v4249 }) });
        let v4339: f64 = (if v1512 { v1 } else { (if v1509 { v1 } else { v4250 }) });
        let v4342: f64 = ((-(v734 * v2103)) / v2347);
        let v4366: f64 = (v552 * v1529);
        let v4367: f64 = (((v1517 * v2503) + (v665 * v4333)) / v4366);
        let v4368: f64 = ((v665 * v4334) / v4366);
        let v4369: f64 = ((v665 * v4335) / v4366);
        let v4370: f64 = ((v665 * v4336) / v4366);
        let v4371: f64 = ((v665 * v4337) / v4366);
        let v4372: f64 = ((v665 * v4338) / v4366);
        let v4373: f64 = ((v665 * v4339) / v4366);
        let v4381: f64 = (v552 * v1532);
        let v4382: f64 = (((v1526 * v2503) + (v665 * (if v1522 { (v1513 * v4342) } else { (if v1519 { (v1520 * v4342) } else { v4128 }) }))) / v4381);
        let v4383: f64 = ((v665 * (if v1522 { v4331 } else { (if v1519 { (v1520 * v4318) } else { v1 }) })) / v4381);
        let v4384: f64 = ((v665 * (if v1522 { v1 } else { (if v1519 { v1 } else { v4129 }) })) / v4381);
        let v4385: f64 = ((v665 * (if v1522 { v4332 } else { (if v1519 { (v1520 * v4319) } else { v4130 }) })) / v4381);
        let v4386: f64 = ((v665 * (if v1522 { v1 } else { (if v1519 { v1 } else { v4131 }) })) / v4381);
        let v4387: f64 = (v747 * v2526);
        let v4388: f64 = (-v688);
        let v4392: f64 = (v1535 * v1535);
        let v4447: f64 = ((v1541 * v2530) + (v691 * ((v1539 * v2103) + (v390 * ((v4367 - v4382) - ((((v1535 * v4367) - (v1534 * v4382)) / v4392) / v1536))))));
        let v4448: f64 = (v691 * (v36 + (v390 * ((-v4383) - (((-(v1534 * v4383)) / v4392) / v1536)))));
        let v4449: f64 = (v691 * (v2571 + (v390 * ((v4368 - v4384) - ((((v1535 * v4368) - (v1534 * v4384)) / v4392) / v1536)))));
        let v4450: f64 = (v691 * (v390 * (v4369 - ((v4369 / v1535) / v1536))));
        let v4451: f64 = (v691 * (v390 * ((v4370 - v4385) - ((((v1535 * v4370) - (v1534 * v4385)) / v4392) / v1536))));
        let v4452: f64 = (v691 * (v390 * ((v4371 - v4386) - ((((v1535 * v4371) - (v1534 * v4386)) / v4392) / v1536))));
        let v4453: f64 = (v691 * (v390 * (v4372 - ((v4372 / v1535) / v1536))));
        let v4454: f64 = (v691 * (v390 * (v4373 - ((v4373 / v1535) / v1536))));
        let v4467: f64 = (v36 * v749);
        let v4469: f64 = (v749 * v2571);
        let v4471: f64 = (v552 * v1548);
        let v4484: f64 = ((v1551 * ((v1542 * v2570) + (v721 * v4447))) - (v1543 * ((v1550 * v2530) + (v691 * (v1548 * (v54 * (v134 * v2570)))))));
        let v4485: f64 = (v1551 * v1551);
        let v4500: f64 = (v1552 * (v4484 / v4485));
        let v4502: f64 = (v1552 * (((v1551 * (v721 * v4448)) - (v1543 * (v691 * (v1545 * ((v4467 + v4467) / v4471))))) / v4485));
        let v4504: f64 = (v1552 * (((v1551 * (v721 * v4449)) - (v1543 * (v691 * (v1545 * ((v4469 + v4469) / v4471))))) / v4485));
        let v4506: f64 = (v1552 * ((v721 * v4450) / v1551));
        let v4508: f64 = (v1552 * ((v721 * v4451) / v1551));
        let v4510: f64 = (v1552 * ((v721 * v4452) / v1551));
        let v4512: f64 = (v1552 * ((v721 * v4453) / v1551));
        let v4514: f64 = (v1552 * ((v721 * v4454) / v1551));
        let v4516: f64 = (v552 * v1555);
        let v4528: f64 = (v1555 * v1555);
        let v4529: f64 = (((v1555 * v4447) - (v1542 * ((v4500 + v4500) / v4516))) / v4528);
        let v4533: f64 = (((v1555 * v4448) - (v1542 * ((v4502 + v4502) / v4516))) / v4528);
        let v4537: f64 = (((v1555 * v4449) - (v1542 * ((v4504 + v4504) / v4516))) / v4528);
        let v4541: f64 = (((v1555 * v4450) - (v1542 * ((v4506 + v4506) / v4516))) / v4528);
        let v4545: f64 = (((v1555 * v4451) - (v1542 * ((v4508 + v4508) / v4516))) / v4528);
        let v4549: f64 = (((v1555 * v4452) - (v1542 * ((v4510 + v4510) / v4516))) / v4528);
        let v4553: f64 = (((v1555 * v4453) - (v1542 * ((v4512 + v4512) / v4516))) / v4528);
        let v4557: f64 = (((v1555 * v4454) - (v1542 * ((v4514 + v4514) / v4516))) / v4528);
        let v4558: f64 = (v750 * v2534);
        let v4559: f64 = (-v694);
        let v4568: f64 = ((v1558 * v2538) + (v697 * (v751 * v3323)));
        let v4569: f64 = (v697 * (v751 * v3324));
        let v4570: f64 = (v697 * v1108);
        let v4571: f64 = (v697 * ((-v1108) + (v751 * v3325)));
        let v4572: f64 = (v697 * (v751 * v3326));
        let v4573: f64 = (v752 * (if v698 { ((-(self.scalar_v436 * (v2104 * (self.scalar_v437 * f64::powf(v391, self.scalar_v2150))))) / (v439 * v439)) } else { v1 }));
        let v4574: f64 = (-v700);
        let v4585: f64 = ((v1561 * v2546) + (v703 * (v753 * (if self.scalar_v1181 { v1 } else { v3471 }))));
        let v4586: f64 = (v703 * (-v1183));
        let v4587: f64 = (v703 * (v753 * (if self.scalar_v1181 { v1 } else { v3472 })));
        let v4588: f64 = (v703 * (v753 * (if self.scalar_v1181 { v1 } else { v3473 })));
        let v4589: f64 = (v703 * (v753 * (if self.scalar_v1181 { v1 } else { v3474 })));
        let v4590: f64 = (v703 * (v753 * (if self.scalar_v1181 { v1 } else { v3475 })));
        let v4591: f64 = (v703 * (v1183 + (v753 * (if self.scalar_v1181 { v1 } else { v3476 }))));
        let v4592: f64 = (v760 * (if v704 { ((-(self.scalar_v440 * (v2104 * (self.scalar_v441 * f64::powf(v391, self.scalar_v2155))))) / (v443 * v443)) } else { v1 }));
        let v4593: f64 = (-v706);
        let v4599: f64 = (if self.scalar_v1565 { ((v1566 * v2327) * (self.scalar_v1571 * f64::powf(v1568, self.scalar_v4595))) } else { v1 });
        let v4600: f64 = (v2425 - v4599);
        let v4601: f64 = (v1575 * v4600);
        let v4603: f64 = (v36 * v1575);
        let v4605: f64 = (v1575 * v2571);
        let v4607: f64 = (v552 * v1578);
        let v4618: f64 = (if self.scalar_v1565 { (v4599 + (v134 * (v4600 + ((v4601 + v4601) / v4607)))) } else { v1 });
        let v4619: f64 = (if self.scalar_v1565 { (v134 * (v36 + ((v4603 + v4603) / v4607))) } else { v1 });
        let v4620: f64 = (if self.scalar_v1565 { (v134 * (v2571 + ((v4605 + v4605) / v4607))) } else { v1 });
        let v4624: f64 = (self.scalar_v1584 * f64::powf(v1582, self.scalar_v4622));
        let v4633: f64 = (if self.scalar_v1565 { ((v1585 * (-v2327)) + (v1583 * (v4618 * v4624))) } else { v1 });
        let v4634: f64 = (if self.scalar_v1565 { (v1583 * (v4619 * v4624)) } else { v1 });
        let v4635: f64 = (if self.scalar_v1565 { (v1583 * (v4620 * v4624)) } else { v1 });
        let v4660: f64 = (if self.scalar_v1565 { ((v1599 * (if v1593 { (v1594 * v4633) } else { (if v1589 { (v1590 * v4633) } else { v1 }) })) + (v1598 * (self.scalar_v1564 * v4618))) } else { v1 });
        let v4661: f64 = (if self.scalar_v1565 { ((v1599 * (if v1593 { (v1594 * v4634) } else { (if v1589 { (v1590 * v4634) } else { v1 }) })) + (v1598 * (self.scalar_v1564 * v4619))) } else { v1 });
        let v4662: f64 = (if self.scalar_v1565 { ((v1599 * (if v1593 { (v1594 * v4635) } else { (if v1589 { (v1590 * v4635) } else { v1 }) })) + (v1598 * (self.scalar_v1564 * v4620))) } else { v1 });
        let v4663: f64 = (-v3331);
        let v4664: f64 = (-v3335);
        let v4665: f64 = (-v3339);
        let v4666: f64 = (-v3343);
        let v4708: f64 = (if self.scalar_v1609 { ((v1566 * v2329) * (self.scalar_v1614 * f64::powf(v1611, self.scalar_v4704))) } else { v1 });
        let v4709: f64 = (-v4708);
        let v4710: f64 = (v1618 * v4709);
        let v4712: f64 = (v36 * v1618);
        let v4714: f64 = (v1618 * v2571);
        let v4716: f64 = (v552 * v1621);
        let v4727: f64 = (if self.scalar_v1609 { (v4708 + (v134 * (v4709 + ((v4710 + v4710) / v4716)))) } else { v1 });
        let v4728: f64 = (if self.scalar_v1609 { (v134 * (v36 + ((v4712 + v4712) / v4716))) } else { v1 });
        let v4729: f64 = (if self.scalar_v1609 { (v134 * (v2571 + ((v4714 + v4714) / v4716))) } else { v1 });
        let v4733: f64 = (self.scalar_v1627 * f64::powf(v1625, self.scalar_v4731));
        let v4742: f64 = (if self.scalar_v1609 { ((v1628 * (-v2329)) + (v1626 * (v4727 * v4733))) } else { v1 });
        let v4743: f64 = (if self.scalar_v1609 { (v1626 * (v4728 * v4733)) } else { v1 });
        let v4744: f64 = (if self.scalar_v1609 { (v1626 * (v4729 * v4733)) } else { v1 });
        let v4778: f64 = ((v1645 * (if self.scalar_v1609 { ((v1642 * (if v1636 { (v1637 * v4742) } else { (if v1632 { (v1633 * v4742) } else { v1 }) })) + (v1641 * (self.scalar_v1608 * v4727))) } else { v4660 })) + (v1644 * (-v4387)));
        let v4781: f64 = ((v1645 * (if self.scalar_v1609 { ((v1642 * (if v1636 { (v1637 * v4743) } else { (if v1632 { (v1633 * v4743) } else { v1 }) })) + (v1641 * (self.scalar_v1608 * v4728))) } else { v1 })) + (v688 * v1644));
        let v4789: f64 = (if self.scalar_v1609 { (v1645 * (if self.scalar_v1609 { ((v1642 * (if v1636 { (v1637 * v4744) } else { (if v1632 { (v1633 * v4744) } else { v1 }) })) + (v1641 * (self.scalar_v1608 * v4729))) } else { v1 })) } else { v1 });
        let v4801: f64 = (if self.scalar_v1657 { (-(v2571 / self.scalar_v1655)) } else { v1 });
        let v4802: f64 = (if self.scalar_v1657 { (-(v36 / self.scalar_v1655)) } else { v1 });
        let v4803: f64 = (v1662 * v4801);
        let v4805: f64 = (v1662 * v4802);
        let v4807: f64 = (v552 * v1665);
        let v4825: f64 = ((v1674 * v3350) - (v1110 * (if self.scalar_v1673 { v1 } else { (if self.scalar_v1657 { (self.scalar_v1652 * (if self.scalar_v1657 { (v134 * (v4801 + ((v4803 + v4803) / v4807))) } else { v4801 })) } else { v1 }) })));
        let v4826: f64 = (v1674 * v1674);
        let v4830: f64 = ((v1674 * v3354) - (v1110 * (if self.scalar_v1673 { v1 } else { (if self.scalar_v1657 { (self.scalar_v1652 * (if self.scalar_v1657 { (v134 * (v4802 + ((v4805 + v4805) / v4807))) } else { v4802 })) } else { v1 }) })));
        let v4835: f64 = (self.scalar_v1677 * f64::powf(v1676, self.scalar_v4833));
        let v4854: f64 = (v4219 - (if self.scalar_v1606 { v1 } else { (if self.scalar_v1565 { (v1601 * (-v4219)) } else { v1 }) }));
        let v4857: f64 = (v4208 - (if self.scalar_v1606 { v1 } else { (if self.scalar_v1565 { (v1601 * (-v4208)) } else { v1 }) }));
        let v4858: f64 = (v4209 - (if self.scalar_v1606 { v1 } else { (if self.scalar_v1565 { (v1601 * (-v4209)) } else { v1 }) }));
        let v4859: f64 = (-(if self.scalar_v1606 { v1 } else { (if self.scalar_v1565 { v1601 } else { v1 }) }));
        let v4860: f64 = ((v4217 - (if self.scalar_v1606 { v1 } else { (if self.scalar_v1565 { ((v1603 * v4660) + (v1601 * (v4663 - v4217))) } else { v1 }) })) - (if self.scalar_v1681 { v1 } else { (if self.scalar_v1654 { (self.scalar_v1650 * ((v3347 / v1674) * v4835)) } else { v1 }) }));
        let v4861: f64 = ((v4218 - (if self.scalar_v1606 { v1 } else { (if self.scalar_v1565 { ((v1603 * v4661) + (v1601 * (v4664 - v4218))) } else { v1 }) })) - (if self.scalar_v1681 { v1 } else { (if self.scalar_v1654 { (self.scalar_v1650 * ((v4825 / v4826) * v4835)) } else { v1 }) }));
        let v4862: f64 = ((v4220 - (if self.scalar_v1606 { v1 } else { (if self.scalar_v1565 { ((v1603 * v4662) + (v1601 * (v4665 - v4220))) } else { v1 }) })) - (if self.scalar_v1681 { v1 } else { (if self.scalar_v1654 { (self.scalar_v1650 * ((v4830 / v4826) * v4835)) } else { v1 }) }));
        let v4863: f64 = ((v4221 - (if self.scalar_v1606 { v1 } else { (if self.scalar_v1565 { (v1601 * (v4666 - v4221)) } else { v1 }) })) - (if self.scalar_v1681 { v1 } else { (if self.scalar_v1654 { (self.scalar_v1650 * ((v3358 / v1674) * v4835)) } else { v1 }) }));
        let v4866: f64 = (if self.scalar_v1687 { ((-v2297) / v2301) } else { v4251 });
        let v4868: f64 = (v1689 * v2571);
        let v4869: f64 = (v36 * v1689);
        let v4888: f64 = (if v1696 { ((v1701 * (v1698 * (v350 * v4866))) + (v1698 * (v1699 * v4866))) } else { (if v1691 { (v1693 * (v756 * v4866)) } else { v4333 }) });
        let v4889: f64 = (if v1696 { v1 } else { (if v1691 { v1 } else { v4334 }) });
        let v4890: f64 = (if v1696 { v1 } else { (if v1691 { v1 } else { v4335 }) });
        let v4891: f64 = (if v1696 { v1 } else { (if v1691 { v1 } else { v4336 }) });
        let v4892: f64 = (if v1696 { v1 } else { (if v1691 { v1 } else { v4337 }) });
        let v4893: f64 = (if v1696 { (v1698 * v4868) } else { (if v1691 { (v1693 * v4868) } else { v4338 }) });
        let v4894: f64 = (if v1696 { (v1698 * v4869) } else { (if v1691 { (v1693 * v4869) } else { v4339 }) });
        let v4897: f64 = (if self.scalar_v1687 { ((-v2313) / v2317) } else { v4866 });
        let v4899: f64 = (v1705 * v2571);
        let v4900: f64 = (v36 * v1705);
        let v4936: f64 = (v525 * (if v1712 { ((v1717 * (v1714 * (v370 * v4897))) + (v1714 * (v1715 * v4897))) } else { (if v1707 { (v1709 * (v756 * v4897)) } else { v4272 }) }));
        let v4958: f64 = (if self.scalar_v1726 { v1 } else { (if self.scalar_v1687 { (((v1720 * v2306) + (v518 * v4888)) + ((v1722 * v2322) + v4936)) } else { v1 }) });
        let v4959: f64 = (if self.scalar_v1726 { v1 } else { (if self.scalar_v1687 { ((v518 * v4889) + (v525 * (if v1712 { v1 } else { (if v1707 { v1 } else { v4273 }) }))) } else { v1 }) });
        let v4960: f64 = (if self.scalar_v1726 { v1 } else { (if self.scalar_v1687 { ((v518 * v4890) + (v525 * (if v1712 { v1 } else { (if v1707 { v1 } else { v4274 }) }))) } else { v1 }) });
        let v4961: f64 = (if self.scalar_v1726 { v1 } else { (if self.scalar_v1687 { ((v518 * v4891) + (v525 * (if v1712 { v1 } else { (if v1707 { v1 } else { v4275 }) }))) } else { v1 }) });
        let v4962: f64 = (if self.scalar_v1726 { v1 } else { (if self.scalar_v1687 { ((v518 * v4892) + (v525 * (if v1712 { v1 } else { (if v1707 { v1 } else { v4276 }) }))) } else { v1 }) });
        let v4963: f64 = (if self.scalar_v1726 { v1 } else { (if self.scalar_v1687 { ((v518 * v4893) + (v525 * (if v1712 { (v1714 * v4899) } else { (if v1707 { (v1709 * v4899) } else { v4277 }) }))) } else { v1 }) });
        let v4964: f64 = (if self.scalar_v1726 { v1 } else { (if self.scalar_v1687 { ((v518 * v4894) + (v525 * (if v1712 { (v1714 * v4900) } else { (if v1707 { (v1709 * v4900) } else { v1 }) }))) } else { v1 }) });
        let v5023: f64 = (((((v1382 * v2571) + (v725 * v4017)) + (v731 * v4863)) + ((v1602 * v2571) + (v744 * v4666))) + ((v1435 * v2571) + (v728 * v4144)));
        let v5038: f64 = (((((v725 * v4015) + ((v1684 * v2571) + (v731 * v4861))) + ((v36 * v1602) + (v744 * v4664))) + (v728 * v4142)) + (v739 * v4309));
        let v5040: f64 = ((((((v36 * v1382) + (v725 * v4016)) + ((v36 * v1684) + (v731 * v4862))) + (v744 * v4665)) + (v728 * v4143)) + (v739 * v4311));
        let v5062: f64 = (((((((v725 * v4014) + (v731 * v4860)) + (v744 * v4663)) + (v728 * v4141)) + (v739 * v4308)) + (v760 * v4592)) + (v756 * v4958));
        let v5064: f64 = (((((v725 * v3971) + (v731 * v4854)) + ((v36 * v1435) + (v728 * v4098))) + ((v36 * v1507) + (v739 * v4310))) + (v756 * v4960));
        let v5067: f64 = (((((v725 * v3974) + (v731 * v4857)) + (v728 * v4101)) + ((v1507 * v2571) + (v739 * v4313))) + ((v1727 * v2571) + (v756 * v4963)));
        let v5068: f64 = ((((((v725 * v3975) + (v731 * v4858)) + (v728 * v4102)) + (v739 * v4314)) + ((-v1563) + (v760 * v4593))) + ((v36 * v1727) + (v756 * v4964)));
        let v5135: f64 = ((((((v5062 + (v758 * v3567)) + (v747 * v4387)) + (v749 * v4529)) + (v750 * v4558)) + (v751 * v4568)) + (v752 * v4573));
        let v5136: f64 = ((((((v5023 + (v739 * v4312)) + (v756 * v4962)) + (v758 * v3571)) + (v749 * v4549)) + (v751 * v4572)) + ((-v1560) + (v752 * v4574)));
        let v5149: f64 = (((((v5038 + (v756 * v4959)) + (v758 * v3568)) + ((v1556 * v2571) + (v749 * v4537))) + (v751 * v4569)) + (v753 * v4587));
        let v5150: f64 = (((((v5064 + (v1782 + (v758 * v3569))) + (v749 * v4541)) + ((-v1557) + (v750 * v4559))) + (v1559 + (v751 * v4570))) + (v753 * v4588));
        let v5154: f64 = (self.scalar_v1756 * (v1533 + v1533));
        let v5155: f64 = (self.scalar_v1756 * (v1557 + v1557));
        let v5156: f64 = (self.scalar_v1756 * (v1560 + v1560));
        let v5157: f64 = (self.scalar_v1756 * (v1563 + v1563));
        let v5158: f64 = (self.scalar_v1756 * (v5135 + (v753 * v4585)));
        let v5159: f64 = (self.scalar_v1756 * (((v1645 + (v747 * v4388)) + (v1780 + (v749 * v4533))) + ((-v1562) + (v753 * v4586))));
        let v5160: f64 = (self.scalar_v1756 * v5149);
        let v5161: f64 = (self.scalar_v1756 * v5150);
        let v5162: f64 = (self.scalar_v1756 * (((((v5040 + (v756 * v4961)) + (v758 * v3570)) + (v749 * v4545)) + ((-v1559) + (v751 * v4571))) + (v753 * v4589)));
        let v5163: f64 = (self.scalar_v1756 * (v5136 + (v753 * v4590)));
        let v5164: f64 = (self.scalar_v1756 * (((v5067 + (v758 * v3572)) + (v749 * v4553)) + (v1562 + (v753 * v4591))));
        let v5165: f64 = (self.scalar_v1756 * ((v5068 + ((v1184 * v2571) + (v758 * v3573))) + (v749 * v4557)));
        let v5166: f64 = (self.scalar_v1756 * (v744 + (v731 * v4859)));
        let v5168: f64 = (v709 + (v371 * (if v707 { ((-(self.scalar_v453 * (self.scalar_v454 * v2101))) / (v457 * v457)) } else { v1 })));
        let v5169: f64 = (-v3347);
        let v5170: f64 = (-v3350);
        let v5171: f64 = (-v3354);
        let v5172: f64 = (-v3358);
        let v5173: f64 = (v11 * v36);
        let v5174: f64 = (v11 * v2571);
        let v5187: f64 = (v36 * v4014);
        let v5188: f64 = (v36 * v4015);
        let v5189: f64 = (v36 * v3971);
        let v5190: f64 = (v36 * (v4016 + v5173));
        let v5191: f64 = (v36 * (v4017 + v5174));
        let v5192: f64 = (v36 * v3974);
        let v5193: f64 = (v36 * v3975);
        let v5194: f64 = (v36 * v4141);
        let v5195: f64 = (v36 * v4142);
        let v5196: f64 = (v36 * (v4098 + v5173));
        let v5197: f64 = (v36 * v4143);
        let v5198: f64 = (v36 * (v4144 + v5174));
        let v5199: f64 = (v36 * v4101);
        let v5200: f64 = (v36 * v4102);
        let v5201: f64 = (v36 * v3331);
        let v5202: f64 = (v36 * v3335);
        let v5203: f64 = (v36 * v3339);
        let v5204: f64 = (v36 * v3343);
        let v5205: f64 = (v36 * v4860);
        let v5206: f64 = (v36 * (v4861 + v5174));
        let v5207: f64 = (v36 * v4854);
        let v5208: f64 = (v36 * (v4862 + v5173));
        let v5209: f64 = (v36 * v4863);
        let v5210: f64 = (v36 * v4857);
        let v5211: f64 = (v36 * v4858);
        let v5212: f64 = (v36 * v4859);
        let v5213: f64 = (v36 * (if self.scalar_v1648 { v1 } else { (if self.scalar_v1609 { (v1644 * v4388) } else { v1 }) }));
        let v5214: f64 = (v36 * (if self.scalar_v1648 { v1 } else { (if self.scalar_v1609 { v4778 } else { v1 }) }));
        let v5215: f64 = (v36 * ((if self.scalar_v1648 { v1 } else { (if self.scalar_v1609 { v4781 } else { v1 }) }) + v5174));
        let v5216: f64 = (v36 * (if self.scalar_v1648 { v1 } else { (if self.scalar_v1609 { (v1645 * (if self.scalar_v1609 { v1 } else { v4661 })) } else { v1 }) }));
        let v5217: f64 = (v36 * ((if self.scalar_v1648 { v1 } else { v4789 }) + v5173));
        let v5218: f64 = (v36 * (if self.scalar_v1648 { v1 } else { (if self.scalar_v1609 { (v1645 * (if self.scalar_v1609 { v1 } else { v4662 })) } else { v1 }) }));
        let v5219: f64 = (v36 * v4308);
        let v5220: f64 = (v36 * v4309);
        let v5221: f64 = (v36 * (v4310 + v5173));
        let v5222: f64 = (v36 * v4311);
        let v5223: f64 = (v36 * v4312);
        let v5224: f64 = (v36 * (v4313 + v5174));
        let v5225: f64 = (v36 * v4314);
        let v5226: f64 = (v36 * v4529);
        let v5227: f64 = (v36 * v4533);
        let v5228: f64 = (v36 * v4537);
        let v5229: f64 = (v36 * v4541);
        let v5230: f64 = (v36 * v4545);
        let v5231: f64 = (v36 * v4549);
        let v5232: f64 = (v36 * v4553);
        let v5233: f64 = (v36 * v4557);
        let v5234: f64 = (v36 * v4958);
        let v5235: f64 = (v36 * v4959);
        let v5236: f64 = (v36 * v4960);
        let v5237: f64 = (v36 * v4961);
        let v5238: f64 = (v36 * v4962);
        let v5239: f64 = (v36 * (v4963 + v5174));
        let v5240: f64 = (v36 * (v4964 + v5173));
        let v5241: f64 = (v36 * v3567);
        let v5242: f64 = (v36 * v3568);
        let v5243: f64 = (v36 * v3569);
        let v5244: f64 = (v36 * v3570);
        let v5245: f64 = (v36 * v3571);
        let v5246: f64 = (v36 * v3572);
        let v5247: f64 = (v36 * v3573);
        let v5248: f64 = (-v2463);
        let v5250: f64 = (if self.scalar_v1783 { (self.scalar_v764 * v5248) } else { v1 });
        let v5251: f64 = (if self.scalar_v1789 { v5250 } else { v1 });
        let v5252: f64 = (if self.scalar_v1789 { v2571 } else { v1 });
        let v5253: f64 = (if self.scalar_v1789 { v36 } else { v1 });
        let v5260: f64 = (self.scalar_v772 * v2463);
        let v5264: f64 = (v1805 * v1805);
        let v5293: f64 = (self.scalar_v1800 * f64::powf(v1814, self.scalar_v5291));
        let v5308: f64 = (if v1812 { (((v1816 * v2463) + (v642 * (-((-((-(v756 * v2463)) / v2484)) * v5293)))) / self.scalar_v1800) } else { (if v1793 { ((v1798 * v2463) / self.scalar_v1800) } else { v1 }) });
        let v5309: f64 = (if v1812 { ((v642 * (-((-(v2571 / v642)) * v5293))) / self.scalar_v1800) } else { v1 });
        let v5310: f64 = (if v1812 { ((v642 * (-((-(v36 / v642)) * v5293))) / self.scalar_v1800) } else { v1 });
        let v5311: f64 = (if v1812 { v1 } else { (if v1793 { (v1796 * ((v1807 * v5251) + (v1791 * (((v1805 * (self.scalar_v1803 * v5251)) - (v1804 * v5260)) / v5264)))) } else { v1 }) });
        let v5320: f64 = (v1786 * v5250);
        let v5327: f64 = (if self.scalar_v1824 { (v560 * (v5250 + (if self.scalar_v1824 { ((v5320 + v5320) / (v552 * v1829)) } else { v1 }))) } else { v1 });
        let v5340: f64 = (if self.scalar_v1824 { (((v1836 * v5248) + (v1784 * ((-(((v642 * v5327) - (v1833 * v2463)) / v2484)) * (self.scalar_v1800 * f64::powf(v1835, self.scalar_v5291))))) / self.scalar_v1800) } else { v1 });
        let v5341: f64 = (if self.scalar_v1824 { v5250 } else { v1 });
        let v5342: f64 = (if self.scalar_v1824 { v2571 } else { v1 });
        let v5343: f64 = (if self.scalar_v1824 { v36 } else { v1 });
        let v5344: f64 = (v1840 * v5341);
        let v5346: f64 = (v1840 * v5342);
        let v5348: f64 = (v1840 * v5343);
        let v5350: f64 = (v552 * v1843);
        let v5364: f64 = (if self.scalar_v1824 { ((v134 * (v5341 - (if self.scalar_v1824 { ((v5344 + v5344) / v5350) } else { v1 }))) - v5250) } else { v1 });
        let v5365: f64 = (if self.scalar_v1824 { (v134 * (v5342 - (if self.scalar_v1824 { ((v5346 + v5346) / v5350) } else { v1 }))) } else { v1 });
        let v5366: f64 = (if self.scalar_v1824 { (v134 * (v5343 - (if self.scalar_v1824 { ((v5348 + v5348) / v5350) } else { v1 }))) } else { v1 });
        let v5377: f64 = (self.scalar_v1800 * f64::powf(v1850, self.scalar_v5291));
        let v5393: f64 = (v2571 - v5365);
        let v5394: f64 = (v36 - v5366);
        let v5395: f64 = (v5327 + (-v5364));
        let v5417: f64 = ((if self.scalar_v1824 { (((v1851 * v5248) + (v1784 * ((-(((v642 * v5364) - (v1848 * v2463)) / v2484)) * v5377))) / self.scalar_v1800) } else { v5308 }) + ((v1860 * (self.scalar_v1795 * v5395)) + (v1857 * (((v1805 * (self.scalar_v1803 * v5395)) - (v1858 * v5260)) / v5264))));
        let v5418: f64 = ((if self.scalar_v1824 { ((v1784 * ((-(v5365 / v642)) * v5377)) / self.scalar_v1800) } else { v5309 }) + ((v1860 * (self.scalar_v1795 * v5393)) + (v1857 * ((self.scalar_v1803 * v5393) / v1805))));
        let v5419: f64 = ((if self.scalar_v1824 { ((v1784 * ((-(v5366 / v642)) * v5377)) / self.scalar_v1800) } else { v5310 }) + ((v1860 * (self.scalar_v1795 * v5394)) + (v1857 * ((self.scalar_v1803 * v5394) / v1805))));
        let v5422: f64 = (if self.scalar_v1824 { v5418 } else { (if self.scalar_v1789 { (v5309 + (if v1812 { v1 } else { (if v1793 { (v1796 * ((v1807 * v5252) + (v1791 * ((self.scalar_v1803 * v5252) / v1805)))) } else { v1 }) })) } else { v1 }) });
        let v5423: f64 = (if self.scalar_v1824 { v5419 } else { (if self.scalar_v1789 { (v5310 + (if v1812 { v1 } else { (if v1793 { (v1796 * ((v1807 * v5253) + (v1791 * ((self.scalar_v1803 * v5253) / v1805)))) } else { v1 }) })) } else { v1 }) });
        let v5453: f64 = (self.scalar_v779 * f64::powf(v1886, self.scalar_v2614));
        let v5468: f64 = (if v1884 { (((v1888 * v2387) + (v588 * (-((-((-(v728 * v2387)) / v2466)) * v5453)))) / self.scalar_v779) } else { (if v1870 { ((v1873 * v2387) / self.scalar_v779) } else { v1 }) });
        let v5469: f64 = (if v1884 { ((v588 * (-(v2612 * v5453))) / self.scalar_v779) } else { v1 });
        let v5470: f64 = (if v1884 { ((v588 * (-(v2613 * v5453))) / self.scalar_v779) } else { v1 });
        let v5477: f64 = (if self.scalar_v767 { (v5468 + (if v1884 { v1 } else { (if v1870 { (v1871 * ((v1879 * v2574) + (v1868 * ((v2584 - (v1877 * v2583)) / v2587)))) } else { v1 }) })) } else { v1 });
        let v5480: f64 = (v1895 * v2664);
        let v5482: f64 = (v1895 * v2665);
        let v5484: f64 = (v1895 * v2666);
        let v5486: f64 = (v552 * v1898);
        let v5500: f64 = (if self.scalar_v802 { ((v134 * (v2664 - (if self.scalar_v802 { ((v5480 + v5480) / v5486) } else { v1 }))) - v2573) } else { v1 });
        let v5501: f64 = (if self.scalar_v802 { (v134 * (v2665 - (if self.scalar_v802 { ((v5482 + v5482) / v5486) } else { v1 }))) } else { v1 });
        let v5502: f64 = (if self.scalar_v802 { (v134 * (v2666 - (if self.scalar_v802 { ((v5484 + v5484) / v5486) } else { v1 }))) } else { v1 });
        let v5513: f64 = (self.scalar_v779 * f64::powf(v1905, self.scalar_v2614));
        let v5529: f64 = (v36 - v5501);
        let v5530: f64 = (v2571 - v5502);
        let v5531: f64 = (v2650 + (-v5500));
        let v5553: f64 = ((if self.scalar_v802 { (((v1906 * v2572) + (v763 * ((-(((v588 * v5500) - (v1903 * v2387)) / v2466)) * v5513))) / self.scalar_v779) } else { v5468 }) + ((v1915 * (self.scalar_v774 * v5531)) + (v1912 * (((v784 * (self.scalar_v782 * v5531)) - (v1913 * v2583)) / v2587))));
        let v5554: f64 = ((if self.scalar_v802 { ((v763 * ((-(v5501 / v588)) * v5513)) / self.scalar_v779) } else { v5469 }) + ((v1915 * (self.scalar_v774 * v5529)) + (v1912 * ((self.scalar_v782 * v5529) / v784))));
        let v5555: f64 = ((if self.scalar_v802 { ((v763 * ((-(v5502 / v588)) * v5513)) / self.scalar_v779) } else { v5470 }) + ((v1915 * (self.scalar_v774 * v5530)) + (v1912 * ((self.scalar_v782 * v5530) / v784))));
        let v5593: f64 = (if v1941 { (((v1947 * v2425) + (v615 * (-((v1945 * v2785) + (v878 * (-((-(v1943 * v2425)) / v2790))))))) / self.scalar_v858) } else { (if v1923 { ((v1927 * v2425) / self.scalar_v858) } else { v1 }) });
        let v5601: f64 = (self.scalar_v858 * f64::powf(v1954, self.scalar_v2782));
        let v5616: f64 = (if v1952 { (((v1956 * v2425) + (v615 * (-((-((-(v739 * v2425)) / v2475)) * v5601)))) / self.scalar_v858) } else { v5593 });
        let v5617: f64 = (if v1952 { ((v615 * (-(v2823 * v5601))) / self.scalar_v858) } else { (if v1941 { v2812 } else { v1 }) });
        let v5618: f64 = (if v1952 { ((v615 * (-(v2822 * v5601))) / self.scalar_v858) } else { (if v1941 { v2811 } else { v1 }) });
        let v5625: f64 = (if self.scalar_v846 { (v5616 + (if v1940 { v1 } else { (if v1923 { (v1924 * ((v1933 * v2749) + (v1921 * ((v2758 - (v1931 * v2425)) / v2475)))) } else { v1 }) })) } else { v1 });
        let v5631: f64 = (if self.scalar_v905 { ((v2853 - (v1965 * v2852)) / v2856) } else { v1 });
        let v5633: f64 = (v1969 * v5631);
        let v5635: f64 = (v1969 * v2904);
        let v5637: f64 = (v1969 * v2903);
        let v5639: f64 = (v552 * v1972);
        let v5643: f64 = (v1973 * v5631);
        let v5645: f64 = (v1973 * v2904);
        let v5647: f64 = (v1973 * v2903);
        let v5649: f64 = (v552 * v1976);
        let v5659: f64 = (v1977 * v1977);
        let v5669: f64 = (if self.scalar_v905 { (((v1977 * (v552 * v5631)) - (v1968 * (((v5633 + v5633) / v5639) + ((v5643 + v5643) / v5649)))) / v5659) } else { v1 });
        let v5670: f64 = (if self.scalar_v905 { (((v1977 * v2907) - (v1968 * (((v5635 + v5635) / v5639) + ((v5645 + v5645) / v5649)))) / v5659) } else { v1 });
        let v5671: f64 = (if self.scalar_v905 { (((v1977 * v2906) - (v1968 * (((v5637 + v5637) / v5639) + ((v5647 + v5647) / v5649)))) / v5659) } else { v1 });
        let v5681: f64 = (if self.scalar_v905 { (v134 * (((v1979 * v2852) + (v907 * v5669)) - v2748)) } else { v1 });
        let v5682: f64 = (if self.scalar_v905 { (v134 * (v907 * v5670)) } else { v1 });
        let v5683: f64 = (if self.scalar_v905 { (v134 * (v907 * v5671)) } else { v1 });
        let v5694: f64 = (self.scalar_v858 * f64::powf(v1986, self.scalar_v2782));
        let v5709: f64 = (if self.scalar_v905 { (((v1988 * v2425) + (v615 * (-((-(((v615 * v5681) - (v1984 * v2425)) / v2475)) * v5694)))) / self.scalar_v858) } else { v5616 });
        let v5710: f64 = (if self.scalar_v905 { ((v615 * (-((-(v5682 / v615)) * v5694))) / self.scalar_v858) } else { v5617 });
        let v5711: f64 = (if self.scalar_v905 { ((v615 * (-((-(v5683 / v615)) * v5694))) / self.scalar_v858) } else { v5618 });
        let v5715: f64 = (if self.scalar_v905 { (v134 * v5669) } else { v1 });
        let v5716: f64 = (if self.scalar_v905 { (v134 * v5670) } else { v1 });
        let v5717: f64 = (if self.scalar_v905 { (v134 * v5671) } else { v1 });
        let v5743: f64 = ((v2001 * (if self.scalar_v905 { (((v1995 * v2997) + (v972 * (-v5715))) + ((v1994 * v3005) + (v976 * v5715))) } else { v1 })) + (v1999 * (v2880 + (-v5681))));
        let v5758: f64 = (if self.scalar_v905 { (v5710 + (if self.scalar_v905 { ((v2001 * (if self.scalar_v905 { ((v972 * (-v5716)) + (v976 * v5716)) } else { v1 })) + (v1999 * (v36 - v5682))) } else { v1 })) } else { (if self.scalar_v846 { (v5617 + (if v1940 { v1 } else { (if v1923 { (v1924 * ((v1933 * v2751) + (v1921 * v2763))) } else { v1 }) })) } else { v1 }) });
        let v5759: f64 = (if self.scalar_v905 { (v5711 + (if self.scalar_v905 { ((v2001 * (if self.scalar_v905 { ((v972 * (-v5717)) + (v976 * v5717)) } else { v1 })) + (v1999 * (v2571 - v5683))) } else { v1 })) } else { (if self.scalar_v846 { (v5618 + (if v1940 { v1 } else { (if v1923 { (v1924 * ((v1933 * v2750) + (v1921 * v2762))) } else { v1 }) })) } else { v1 }) });
        let v5760: f64 = (v2007 * v3069);
        let v5762: f64 = (v2007 * v3071);
        let v5764: f64 = (v2007 * v3070);
        let v5766: f64 = (v552 * v2010);
        let v5780: f64 = (if self.scalar_v990 { ((v134 * (v3069 - (if self.scalar_v990 { ((v5760 + v5760) / v5766) } else { v1 }))) - v2748) } else { v5681 });
        let v5781: f64 = (if self.scalar_v990 { (v134 * (v3071 - (if self.scalar_v990 { ((v5762 + v5762) / v5766) } else { v1 }))) } else { v5682 });
        let v5782: f64 = (if self.scalar_v990 { (v134 * (v3070 - (if self.scalar_v990 { ((v5764 + v5764) / v5766) } else { v1 }))) } else { v5683 });
        let v5793: f64 = (self.scalar_v858 * f64::powf(v2017, self.scalar_v2782));
        let v5815: f64 = ((if self.scalar_v990 { (((v2018 * v2747) + (v843 * ((-(((v615 * v5780) - (v2015 * v2425)) / v2475)) * v5793))) / self.scalar_v858) } else { v5709 }) + (self.scalar_v1019 * (v3055 + (-v5780))));
        let v5825: f64 = (v64 * (v2029 * v3162));
        let v5826: f64 = (v64 * (v2029 * v3163));
        let v5827: f64 = (v64 * (v2029 * v3164));
        let v5831: f64 = (v2032 * v2032);
        let v5843: f64 = ((v59 * v2571) / v2035);
        let v5844: f64 = ((v36 * v59) / v2035);
        let v5878: f64 = (v2033 * (((v2032 * v5825) - (v2031 * v5825)) / v5831));
        let v5880: f64 = (v2033 * (((v2032 * v5826) - (v2031 * v5826)) / v5831));
        let v5882: f64 = (v2033 * (((v2032 * v5827) - (v2031 * v5827)) / v5831));
        let v5906: f64 = ((v2056 * (self.scalar_v2045 * (self.scalar_v2046 * v3231))) + (v2049 * (v2029 * ((v2053 * (self.scalar_v2050 * (if v2040 { v1 } else { (if v2037 { v1 } else { v4888 }) }))) + (v2051 * (v5878 + v5878))))));
        let v5909: f64 = ((v2056 * (self.scalar_v2045 * (self.scalar_v2046 * v3232))) + (v2049 * (v2029 * (v2053 * (self.scalar_v2050 * (if v2040 { (v1513 * v5843) } else { (if v2037 { (v2038 * v5843) } else { v4889 }) }))))));
        let v5912: f64 = (v2049 * (v2029 * ((v2053 * (self.scalar_v2050 * (if v2040 { (v1513 * v5844) } else { (if v2037 { (v2038 * v5844) } else { v4891 }) }))) + (v2051 * (v5880 + v5880)))));
        let v5916: f64 = ((v2056 * (self.scalar_v2045 * (self.scalar_v2046 * v3234))) + (v2049 * (v2029 * ((v2053 * (self.scalar_v2050 * (if v2040 { v1 } else { (if v2037 { v1 } else { v4892 }) }))) + (v2051 * (v5882 + v5882))))));
        let v5959: f64 = ((self.scalar_v1185 * ((v842 * v2472) + (v647 * v2744))) + (((v1108 * ((v2057 * v3162) + (v1041 * v5906))) - (v2060 * v3323)) / v3330));
        let v5960: f64 = ((self.scalar_v1185 * (v647 * v2745)) + (((v1108 * ((v2057 * v3163) + (v1041 * ((v2056 * (self.scalar_v2045 * (self.scalar_v2046 * v3233))) + v5912)))) - (v2060 * v3325)) / v3330));
        let v5965: f64 = (v647 * (if self.scalar_v802 { v5554 } else { (if self.scalar_v767 { (v5469 + (if v1884 { v1 } else { (if v1870 { (v1871 * ((v1879 * v2575) + (v1868 * v2589))) } else { v1 }) })) } else { v1 }) }));
        let v5966: f64 = (v647 * (if self.scalar_v802 { v5555 } else { (if self.scalar_v767 { (v5470 + (if v1884 { v1 } else { (if v1870 { (v1871 * ((v1879 * v2576) + (v1868 * v2590))) } else { v1 }) })) } else { v1 }) }));
        let v6009: f64 = (((v2027 * (self.scalar_v653 * v2479)) + (v654 * (if self.scalar_v990 { (v5815 - v3068) } else { (if self.scalar_v905 { ((v5709 + (if self.scalar_v905 { v5743 } else { v1 })) - v2894) } else { v5625 }) }))) + (self.scalar_v2066 * (if self.scalar_v1181 { v1 } else { v3422 })));
        let v6010: f64 = ((v654 * (if self.scalar_v990 { ((if self.scalar_v990 { ((v843 * ((-(v5781 / v615)) * v5793)) / self.scalar_v858) } else { v5710 }) + (self.scalar_v1019 * (v36 - v5781))) } else { v5758 })) + (self.scalar_v2066 * (if self.scalar_v1181 { v1 } else { v3424 })));
        let v6011: f64 = ((v654 * (if self.scalar_v990 { ((if self.scalar_v990 { ((v843 * ((-(v5782 / v615)) * v5793)) / self.scalar_v858) } else { v5711 }) + (self.scalar_v1019 * (v2571 - v5782))) } else { v5759 })) + (self.scalar_v2066 * (if self.scalar_v1181 { v1 } else { v3427 })));
        let v6014: f64 = ((v1866 * (self.scalar_v655 * (((-(self.scalar_v616 * v2463)) / v2484) * (self.scalar_v657 * f64::powf(v656, self.scalar_v2486))))) + (v659 * (if self.scalar_v1865 { v1 } else { (if self.scalar_v1824 { (v5417 - v5340) } else { (if self.scalar_v1789 { (v5308 + v5311) } else { v1 }) }) })));
        let v6024: f64 = (v36 * v5959);
        let v6025: f64 = (v36 * (((v1108 * (v1041 * v5909)) - (v2060 * v3324)) / v3330));
        let v6026: f64 = (v36 * ((v1041 * (v2049 * (v2029 * (v2053 * (self.scalar_v2050 * (if v2040 { v1 } else { (if v2037 { v1 } else { v4890 }) })))))) / v1108));
        let v6027: f64 = (v36 * v5960);
        let v6028: f64 = (v36 * ((self.scalar_v1185 * (v647 * v2746)) + (((v1108 * ((v2057 * v3164) + (v1041 * v5916))) - (v2060 * v3326)) / v3330)));
        let v6029: f64 = (v36 * ((v1041 * (v2049 * (v2029 * (v2053 * (self.scalar_v2050 * (if v2040 { v1 } else { (if v2037 { v1 } else { v4893 }) })))))) / v1108));
        let v6030: f64 = (v36 * ((v1041 * (v2049 * (v2029 * (v2053 * (self.scalar_v2050 * (if v2040 { v1 } else { (if v2037 { v1 } else { v4894 }) })))))) / v1108));
        let v6031: f64 = (v36 * (self.scalar_v1407 * ((v1919 * v2472) + (v647 * (if self.scalar_v802 { (v5553 - v2663) } else { v5477 })))));
        let v6032: f64 = (v36 * (self.scalar_v1407 * v5965));
        let v6033: f64 = (v36 * (self.scalar_v1407 * v5966));
        let v6034: f64 = (v36 * ((((v1025 * (self.scalar_v648 * v2479)) + (v652 * v3131)) + (self.scalar_v2066 * v3198)) + (self.scalar_v2069 * v4367)));
        let v6035: f64 = (v36 * (((v652 * v3132) + (self.scalar_v2066 * v3199)) + (self.scalar_v2069 * v4368)));
        let v6036: f64 = (v36 * (self.scalar_v2069 * v4369));
        let v6037: f64 = (v36 * (((v652 * v3133) + (self.scalar_v2066 * v3200)) + (self.scalar_v2069 * v4370)));
        let v6038: f64 = (v36 * ((self.scalar_v2066 * v3201) + (self.scalar_v2069 * v4371)));
        let v6039: f64 = (v36 * (self.scalar_v2069 * v4372));
        let v6040: f64 = (v36 * (self.scalar_v2069 * v4373));
        let v6041: f64 = (v36 * (self.scalar_v2069 * v4382));
        let v6042: f64 = (v36 * (self.scalar_v2069 * v4383));
        let v6043: f64 = (v36 * (self.scalar_v2069 * v4384));
        let v6044: f64 = (v36 * (self.scalar_v2069 * v4385));
        let v6045: f64 = (v36 * (self.scalar_v2069 * v4386));
        let v6046: f64 = (v36 * v6009);
        let v6047: f64 = (v36 * (self.scalar_v2066 * (if self.scalar_v1181 { v1 } else { v3423 })));
        let v6048: f64 = (v36 * v6010);
        let v6049: f64 = (v36 * (self.scalar_v2066 * (if self.scalar_v1181 { v1 } else { v3425 })));
        let v6050: f64 = (v36 * (self.scalar_v2066 * (if self.scalar_v1181 { v1 } else { v3426 })));
        let v6051: f64 = (v36 * v6011);
        let v6052: f64 = (v36 * v6014);
        let v6053: f64 = (v36 * ((v659 * (if self.scalar_v1865 { v1 } else { v5422 })) + (self.scalar_v2077 * v2571)));
        let v6054: f64 = (v36 * ((v659 * (if self.scalar_v1865 { v1 } else { v5423 })) + (v36 * self.scalar_v2077)));

        let d1773_dn4: f64 = v5187;
        let d1773_dn6: f64 = v5188;
        let d1773_dn7: f64 = v5189;
        let d1773_dn8: f64 = v5190;
        let d1773_dn9: f64 = v5191;
        let d1773_dn10: f64 = v5192;
        let d1773_dn11: f64 = v5193;
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(8),
            Some(9),
            multiplicity * (v1773),
            [4, 6, 7, 8, 9, 10, 11],
            [d1773_dn4, d1773_dn6, d1773_dn7, d1773_dn8, d1773_dn9, d1773_dn10, d1773_dn11],
            [],
            [],
            multiplicity,
        );
        let d1774_dn4: f64 = v5194;
        let d1774_dn6: f64 = v5195;
        let d1774_dn7: f64 = v5196;
        let d1774_dn8: f64 = v5197;
        let d1774_dn9: f64 = v5198;
        let d1774_dn10: f64 = v5199;
        let d1774_dn11: f64 = v5200;
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(7),
            Some(9),
            multiplicity * (v1774),
            [4, 6, 7, 8, 9, 10, 11],
            [d1774_dn4, d1774_dn6, d1774_dn7, d1774_dn8, d1774_dn9, d1774_dn10, d1774_dn11],
            [],
            [],
            multiplicity,
        );
        let d1775_dn13: f64 = v36;
        stamper.stamp_current_node1_local(
            Some(6),
            Some(9),
            multiplicity * (v1775),
            13,
            multiplicity * (d1775_dn13),
        );
        let d1776_dn4: f64 = v5201;
        let d1776_dn6: f64 = v5202;
        let d1776_dn8: f64 = v5203;
        let d1776_dn9: f64 = v5204;
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(9),
            Some(6),
            multiplicity * (v1776),
            [4, 6, 8, 9],
            [d1776_dn4, d1776_dn6, d1776_dn8, d1776_dn9],
            [],
            [],
            multiplicity,
        );
        let d1777_dn4: f64 = v5205;
        let d1777_dn6: f64 = v5206;
        let d1777_dn7: f64 = v5207;
        let d1777_dn8: f64 = v5208;
        let d1777_dn9: f64 = v5209;
        let d1777_dn10: f64 = v5210;
        let d1777_dn11: f64 = v5211;
        let d1777_dn13: f64 = v5212;
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(8),
            Some(6),
            multiplicity * (v1777),
            [4, 6, 7, 8, 9, 10, 11, 13],
            [d1777_dn4, d1777_dn6, d1777_dn7, d1777_dn8, d1777_dn9, d1777_dn10, d1777_dn11, d1777_dn13],
            [],
            [],
            multiplicity,
        );
        let d1778_dn0: f64 = v5213;
        let d1778_dn4: f64 = v5214;
        let d1778_dn5: f64 = v5215;
        let d1778_dn6: f64 = v5216;
        let d1778_dn7: f64 = v5217;
        let d1778_dn8: f64 = v5218;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(7),
            Some(5),
            multiplicity * (v1778),
            [0, 4, 5, 6, 7, 8],
            [d1778_dn0, d1778_dn4, d1778_dn5, d1778_dn6, d1778_dn7, d1778_dn8],
            [],
            [],
            multiplicity,
        );
        let d1779_dn4: f64 = v5219;
        let d1779_dn6: f64 = v5220;
        let d1779_dn7: f64 = v5221;
        let d1779_dn8: f64 = v5222;
        let d1779_dn9: f64 = v5223;
        let d1779_dn10: f64 = v5224;
        let d1779_dn11: f64 = v5225;
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(7),
            Some(10),
            multiplicity * (v1779),
            [4, 6, 7, 8, 9, 10, 11],
            [d1779_dn4, d1779_dn6, d1779_dn7, d1779_dn8, d1779_dn9, d1779_dn10, d1779_dn11],
            [],
            [],
            multiplicity,
        );
        let d1533_dn0: f64 = v688;
        let d1533_dn4: f64 = v4387;
        let d1533_dn5: f64 = v4388;
        stamper.stamp_current_node3_local(
            Some(0),
            Some(5),
            multiplicity * (v1533),
            0,
            multiplicity * (d1533_dn0),
            4,
            multiplicity * (d1533_dn4),
            5,
            multiplicity * (d1533_dn5),
        );
        let d1780_dn4: f64 = v5226;
        let d1780_dn5: f64 = v5227;
        let d1780_dn6: f64 = v5228;
        let d1780_dn7: f64 = v5229;
        let d1780_dn8: f64 = v5230;
        let d1780_dn9: f64 = v5231;
        let d1780_dn10: f64 = v5232;
        let d1780_dn11: f64 = v5233;
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(5),
            Some(6),
            multiplicity * (v1780),
            [4, 5, 6, 7, 8, 9, 10, 11],
            [d1780_dn4, d1780_dn5, d1780_dn6, d1780_dn7, d1780_dn8, d1780_dn9, d1780_dn10, d1780_dn11],
            [],
            [],
            multiplicity,
        );
        let d1557_dn1: f64 = v694;
        let d1557_dn4: f64 = v4558;
        let d1557_dn7: f64 = v4559;
        stamper.stamp_current_node3_local(
            Some(1),
            Some(7),
            multiplicity * (v1557),
            1,
            multiplicity * (d1557_dn1),
            4,
            multiplicity * (d1557_dn4),
            7,
            multiplicity * (d1557_dn7),
        );
        let d1559_dn4: f64 = v4568;
        let d1559_dn6: f64 = v4569;
        let d1559_dn7: f64 = v4570;
        let d1559_dn8: f64 = v4571;
        let d1559_dn9: f64 = v4572;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(7),
            Some(8),
            multiplicity * (v1559),
            [4, 6, 7, 8, 9],
            [d1559_dn4, d1559_dn6, d1559_dn7, d1559_dn8, d1559_dn9],
            [],
            [],
            multiplicity,
        );
        let d1560_dn2: f64 = v700;
        let d1560_dn4: f64 = v4573;
        let d1560_dn9: f64 = v4574;
        stamper.stamp_current_node3_local(
            Some(2),
            Some(9),
            multiplicity * (v1560),
            2,
            multiplicity * (d1560_dn2),
            4,
            multiplicity * (d1560_dn4),
            9,
            multiplicity * (d1560_dn9),
        );
        let d1562_dn4: f64 = v4585;
        let d1562_dn5: f64 = v4586;
        let d1562_dn6: f64 = v4587;
        let d1562_dn7: f64 = v4588;
        let d1562_dn8: f64 = v4589;
        let d1562_dn9: f64 = v4590;
        let d1562_dn10: f64 = v4591;
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(10),
            Some(5),
            multiplicity * (v1562),
            [4, 5, 6, 7, 8, 9, 10],
            [d1562_dn4, d1562_dn5, d1562_dn6, d1562_dn7, d1562_dn8, d1562_dn9, d1562_dn10],
            [],
            [],
            multiplicity,
        );
        let d1781_dn4: f64 = v5234;
        let d1781_dn6: f64 = v5235;
        let d1781_dn7: f64 = v5236;
        let d1781_dn8: f64 = v5237;
        let d1781_dn9: f64 = v5238;
        let d1781_dn10: f64 = v5239;
        let d1781_dn11: f64 = v5240;
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(11),
            Some(10),
            multiplicity * (v1781),
            [4, 6, 7, 8, 9, 10, 11],
            [d1781_dn4, d1781_dn6, d1781_dn7, d1781_dn8, d1781_dn9, d1781_dn10, d1781_dn11],
            [],
            [],
            multiplicity,
        );
        let d1782_dn4: f64 = v5241;
        let d1782_dn6: f64 = v5242;
        let d1782_dn7: f64 = v5243;
        let d1782_dn8: f64 = v5244;
        let d1782_dn9: f64 = v5245;
        let d1782_dn10: f64 = v5246;
        let d1782_dn11: f64 = v5247;
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(7),
            Some(11),
            multiplicity * (v1782),
            [4, 6, 7, 8, 9, 10, 11],
            [d1782_dn4, d1782_dn6, d1782_dn7, d1782_dn8, d1782_dn9, d1782_dn10, d1782_dn11],
            [],
            [],
            multiplicity,
        );
        let d1563_dn3: f64 = v706;
        let d1563_dn4: f64 = v4592;
        let d1563_dn11: f64 = v4593;
        stamper.stamp_current_node3_local(
            Some(3),
            Some(11),
            multiplicity * (v1563),
            3,
            multiplicity * (d1563_dn3),
            4,
            multiplicity * (d1563_dn4),
            11,
            multiplicity * (d1563_dn11),
        );
        let d1759_dn4: f64 = v5169;
        let d1759_dn6: f64 = v5170;
        let d1759_dn8: f64 = v5171;
        let d1759_dn9: f64 = v5172;
        let d1759_dn13: f64 = v2;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(12),
            None,
            multiplicity * (v1759),
            [4, 6, 8, 9, 13],
            [d1759_dn4, d1759_dn6, d1759_dn8, d1759_dn9, d1759_dn13],
            [],
            [],
            multiplicity,
        );
        let d1760_dn12: f64 = v26;
        let d1760_dn13: f64 = v2;
        stamper.stamp_current_node2_local(
            Some(13),
            None,
            multiplicity * (v1760),
            12,
            multiplicity * (d1760_dn12),
            13,
            multiplicity * (d1760_dn13),
        );
        let d1758_dn4: f64 = v5168;
        stamper.stamp_current_node1_local(
            Some(4),
            None,
            multiplicity * (v1758),
            4,
            multiplicity * (d1758_dn4),
        );
        let d1757_dn0: f64 = v5154;
        let d1757_dn1: f64 = v5155;
        let d1757_dn2: f64 = v5156;
        let d1757_dn3: f64 = v5157;
        let d1757_dn4: f64 = v5158;
        let d1757_dn5: f64 = v5159;
        let d1757_dn6: f64 = v5160;
        let d1757_dn7: f64 = v5161;
        let d1757_dn8: f64 = v5162;
        let d1757_dn9: f64 = v5163;
        let d1757_dn10: f64 = v5164;
        let d1757_dn11: f64 = v5165;
        let d1757_dn13: f64 = v5166;
        let v1757_node_derivative_indices: [usize; 13] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13];
        let v1757_node_derivatives: [f64; 13] = [d1757_dn0, d1757_dn1, d1757_dn2, d1757_dn3, d1757_dn4, d1757_dn5, d1757_dn6, d1757_dn7, d1757_dn8, d1757_dn9, d1757_dn10, d1757_dn11, d1757_dn13];
        let v1757_branch_derivative_indices: [usize; 0] = [];
        let v1757_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(4),
            None,
            multiplicity * (v1757),
            &v1757_node_derivative_indices,
            &v1757_node_derivatives,
            &v1757_branch_derivative_indices,
            &v1757_branch_derivatives,
            multiplicity,
        );
        let d2091_dn4: f64 = v6024;
        let d2091_dn6: f64 = v6025;
        let d2091_dn7: f64 = v6026;
        let d2091_dn8: f64 = v6027;
        let d2091_dn9: f64 = v6028;
        let d2091_dn10: f64 = v6029;
        let d2091_dn11: f64 = v6030;
        let v2091_ddt: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, v2091);
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(8),
            Some(9),
            multiplicity * (v2091_ddt),
            [4, 6, 7, 8, 9, 10, 11],
            [((d2091_dn4) * ddt_scale), ((d2091_dn6) * ddt_scale), ((d2091_dn7) * ddt_scale), ((d2091_dn8) * ddt_scale), ((d2091_dn9) * ddt_scale), ((d2091_dn10) * ddt_scale), ((d2091_dn11) * ddt_scale)],
            [],
            [],
            multiplicity,
        );
        let d2092_dn4: f64 = v6031;
        let d2092_dn7: f64 = v6032;
        let d2092_dn9: f64 = v6033;
        let v2092_ddt: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, v2092);
        stamper.stamp_current_node3_local(
            Some(7),
            Some(9),
            multiplicity * (v2092_ddt),
            4,
            multiplicity * (((d2092_dn4) * ddt_scale)),
            7,
            multiplicity * (((d2092_dn7) * ddt_scale)),
            9,
            multiplicity * (((d2092_dn9) * ddt_scale)),
        );
        let d2093_dn4: f64 = v6034;
        let d2093_dn6: f64 = v6035;
        let d2093_dn7: f64 = v6036;
        let d2093_dn8: f64 = v6037;
        let d2093_dn9: f64 = v6038;
        let d2093_dn10: f64 = v6039;
        let d2093_dn11: f64 = v6040;
        let v2093_ddt: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, v2093);
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(8),
            Some(6),
            multiplicity * (v2093_ddt),
            [4, 6, 7, 8, 9, 10, 11],
            [((d2093_dn4) * ddt_scale), ((d2093_dn6) * ddt_scale), ((d2093_dn7) * ddt_scale), ((d2093_dn8) * ddt_scale), ((d2093_dn9) * ddt_scale), ((d2093_dn10) * ddt_scale), ((d2093_dn11) * ddt_scale)],
            [],
            [],
            multiplicity,
        );
        let d2094_dn4: f64 = v6041;
        let d2094_dn5: f64 = v6042;
        let d2094_dn6: f64 = v6043;
        let d2094_dn8: f64 = v6044;
        let d2094_dn9: f64 = v6045;
        let v2094_ddt: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, v2094);
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(8),
            Some(5),
            multiplicity * (v2094_ddt),
            [4, 5, 6, 8, 9],
            [((d2094_dn4) * ddt_scale), ((d2094_dn5) * ddt_scale), ((d2094_dn6) * ddt_scale), ((d2094_dn8) * ddt_scale), ((d2094_dn9) * ddt_scale)],
            [],
            [],
            multiplicity,
        );
        let d2095_dn4: f64 = v6046;
        let d2095_dn6: f64 = v6047;
        let d2095_dn7: f64 = v6048;
        let d2095_dn8: f64 = v6049;
        let d2095_dn9: f64 = v6050;
        let d2095_dn10: f64 = v6051;
        let v2095_ddt: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, v2095);
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(7),
            Some(10),
            multiplicity * (v2095_ddt),
            [4, 6, 7, 8, 9, 10],
            [((d2095_dn4) * ddt_scale), ((d2095_dn6) * ddt_scale), ((d2095_dn7) * ddt_scale), ((d2095_dn8) * ddt_scale), ((d2095_dn9) * ddt_scale), ((d2095_dn10) * ddt_scale)],
            [],
            [],
            multiplicity,
        );
        let d2081_dn1: f64 = self.scalar_v2080;
        let d2081_dn2: f64 = self.scalar_v6021;
        let v2081_ddt: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 5, v2081);
        stamper.stamp_current_node2_local(
            Some(1),
            Some(2),
            multiplicity * (v2081_ddt),
            1,
            multiplicity * (((d2081_dn1) * ddt_scale)),
            2,
            multiplicity * (((d2081_dn2) * ddt_scale)),
        );
        let d2083_dn0: f64 = self.scalar_v6022;
        let d2083_dn1: f64 = self.scalar_v2082;
        let v2083_ddt: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 6, v2083);
        stamper.stamp_current_node2_local(
            Some(1),
            Some(0),
            multiplicity * (v2083_ddt),
            0,
            multiplicity * (((d2083_dn0) * ddt_scale)),
            1,
            multiplicity * (((d2083_dn1) * ddt_scale)),
        );
        let d2096_dn4: f64 = v6052;
        let d2096_dn10: f64 = v6053;
        let d2096_dn11: f64 = v6054;
        let v2096_ddt: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 7, v2096);
        stamper.stamp_current_node3_local(
            Some(11),
            Some(10),
            multiplicity * (v2096_ddt),
            4,
            multiplicity * (((d2096_dn4) * ddt_scale)),
            10,
            multiplicity * (((d2096_dn10) * ddt_scale)),
            11,
            multiplicity * (((d2096_dn11) * ddt_scale)),
        );
        let d2087_dn12: f64 = self.scalar_v2086;
        let v2087_ddt: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 8, v2087);
        stamper.stamp_current_node1_local(
            Some(12),
            None,
            multiplicity * (v2087_ddt),
            12,
            multiplicity * (((d2087_dn12) * ddt_scale)),
        );
        let d2090_dn13: f64 = self.scalar_v6023;
        let v2090_ddt: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 9, v2090);
        stamper.stamp_current_node1_local(
            Some(13),
            None,
            multiplicity * (v2090_ddt),
            13,
            multiplicity * (((d2090_dn13) * ddt_scale)),
        );
        let d2085_dn4: f64 = self.scalar_v2084;
        let v2085_ddt: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 10, v2085);
        stamper.stamp_current_node1_local(
            Some(4),
            None,
            multiplicity * (v2085_ddt),
            4,
            multiplicity * (((d2085_dn4) * ddt_scale)),
        );
        stamper.stamp_current_const_local(
            Some(8),
            Some(9),
            multiplicity * (v1),
        );
        stamper.stamp_current_const_local(
            Some(8),
            Some(9),
            multiplicity * (v1),
        );
        stamper.stamp_current_const_local(
            Some(7),
            Some(9),
            multiplicity * (v1),
        );
        stamper.stamp_current_const_local(
            Some(7),
            Some(9),
            multiplicity * (v1),
        );
        stamper.stamp_current_const_local(
            Some(6),
            Some(9),
            multiplicity * (v1),
        );
        stamper.stamp_current_const_local(
            Some(7),
            Some(10),
            multiplicity * (v1),
        );
        stamper.stamp_current_const_local(
            Some(7),
            Some(10),
            multiplicity * (v1),
        );
        stamper.stamp_current_const_local(
            Some(0),
            Some(5),
            multiplicity * (v1),
        );
        stamper.stamp_current_const_local(
            Some(5),
            Some(6),
            multiplicity * (v1),
        );
        stamper.stamp_current_const_local(
            Some(1),
            Some(7),
            multiplicity * (v1),
        );
        stamper.stamp_current_const_local(
            Some(7),
            Some(8),
            multiplicity * (v1),
        );
        stamper.stamp_current_const_local(
            Some(2),
            Some(9),
            multiplicity * (v1),
        );
        stamper.stamp_current_const_local(
            Some(10),
            Some(5),
            multiplicity * (v1),
        );
        stamper.stamp_current_const_local(
            Some(7),
            Some(11),
            multiplicity * (v1),
        );
        stamper.stamp_current_const_local(
            Some(3),
            Some(11),
            multiplicity * (v1),
        );
    }

    pub fn stamp_reactive(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedReactiveStamper<'_>) {
        let nodes = self.nodes;
        let branches = self.branches;
        self.ensure_temperature_static(ctx.temperature(), ctx.thermal_voltage());
        let p = &(*self.params);
        let multiplicity = self.multiplicity;
        let v0: f64 = if ctx.analysis_static() { 1.0 } else { 0.0 };
        let v1: f64 = 0.0;
        let v2: f64 = 1.0;
        let v18: f64 = (if ((v0 != 0.0) && self.scalar_v16) { v2 } else { (if ((v0 != 0.0) && (self.scalar_v12 != 0.0)) { self.scalar_v14 } else { v1 }) });
        let v24: bool = ((v0 != 0.0) && self.scalar_v23);
        let v30: bool = (v24 && self.scalar_v29);
        let v36: f64 = (if (v30 && self.scalar_v34) { v2 } else { (if ((self.scalar_v28 != 0.0) && v30) { self.scalar_v32 } else { (if ((self.scalar_v22 != 0.0) && v24) { -1.0 } else { (if ((v0 != 0.0) && (self.scalar_v19 != 0.0)) { v2 } else { v1 }) }) }) });
        let v39: f64 = (if (v0 != 0.0) { self.scalar_v38 } else { v1 });
        let v44: f64 = (if (v0 != 0.0) { self.scalar_v43 } else { v1 });
        let v59: f64 = (if (v0 != 0.0) { self.scalar_v58 } else { v1 });
        let v64: f64 = (if (v0 != 0.0) { self.scalar_v63 } else { v1 });
        let v67: f64 = 273.15;
        let v70: f64 = (if (v0 != 0.0) { self.scalar_v69 } else { v1 });
        let v94: f64 = 1.380662e-23;
        let v96: f64 = 1.602189e-19;
        let v98: f64 = (self.scalar_v93 / v70);
        let v114: f64 = (if self.scalar_v113 { v1 } else { (if self.scalar_v101 { (self.scalar_v103 * (((self.scalar_v107 + (v18 / self.scalar_v100))) as f64).ln()) } else { v1 }) });
        let v122: f64 = (v2 - v98);
        let v127: f64 = ((self.scalar_v99 * f64::powf(v98, self.scalar_v117)) * ((((self.scalar_v121 * v122) / self.scalar_v124)) as f64).exp());
        let v128: bool = (v127 > v1);
        let v132: bool = (self.scalar_v130 && (v18 > self.scalar_v129));
        let v134: f64 = 0.5;
        let v135: f64 = (v18 * v134);
        let v136: f64 = 4.0;
        let v155: f64 = (if (v128 && (!v132)) { (self.scalar_v124 * (((v2 + (v18 / v127))) as f64).ln()) } else { (if (v128 && v132) { (self.scalar_v124 * (((v2 + (f64::powf((v135 * self.scalar_v139), self.scalar_v142) / v127))) as f64).ln()) } else { v1 }) });
        let v157: f64 = (if (!v128) { v1 } else { v155 });
        let v170: f64 = ((self.scalar_v158 * f64::powf(v98, self.scalar_v161)) * ((((v122 * self.scalar_v165) / self.scalar_v167)) as f64).exp());
        let v172: bool = (v128 && (v170 > v1));
        let v174: bool = (self.scalar_v41 && (v18 > self.scalar_v40));
        let v180: f64 = (v127 * v170);
        let v192: f64 = (if (v172 && (!v174)) { (self.scalar_v167 * (((v2 + (v18 / v180))) as f64).ln()) } else { (if (v172 && v174) { (self.scalar_v167 * (((v2 + (f64::powf((v135 * self.scalar_v177), self.scalar_v142) / v180))) as f64).ln()) } else { v1 }) });
        let v194: f64 = (if (!v172) { v1 } else { v192 });
        let v206: f64 = ((self.scalar_v195 * f64::powf(v98, self.scalar_v197)) * ((((v122 * self.scalar_v201) / self.scalar_v203)) as f64).exp());
        let v207: bool = (v206 > v1);
        let v209: bool = (self.scalar_v46 && (v18 > self.scalar_v45));
        let v224: f64 = (if (v207 && (!v209)) { (self.scalar_v203 * (((v2 + (v18 / v206))) as f64).ln()) } else { (if (v207 && v209) { (self.scalar_v203 * (((v2 + (((if (v0 != 0.0) { self.scalar_v48 } else { v1 }) * (v18 * v18)) / v206))) as f64).ln()) } else { v1 }) });
        let v226: f64 = (if (!v207) { v1 } else { v224 });
        let v239: f64 = ((self.scalar_v227 * f64::powf(v98, self.scalar_v230)) * ((((v122 * self.scalar_v234) / self.scalar_v236)) as f64).exp());
        let v240: bool = (v239 > v1);
        let v247: f64 = (if (!v240) { v1 } else { (if v240 { (self.scalar_v236 * (((v2 + (v18 / v239))) as f64).ln()) } else { v1 }) });
        let v272: f64 = f64::powf(v98, self.scalar_v271);
        let v279: f64 = ((((v122 * self.scalar_v275) / self.scalar_v277)) as f64).exp();
        let v280: f64 = ((self.scalar_v269 * v272) * v279);
        let v281: bool = (v280 > v1);
        let v288: f64 = (if (!v281) { v1 } else { (if v281 { (self.scalar_v277 * (((v2 + (v18 / v280))) as f64).ln()) } else { v1 }) });
        let v311: f64 = (v279 * (v272 * self.scalar_v309));
        let v312: bool = (v311 > v1);
        let v319: f64 = (if (!v312) { v1 } else { (if v312 { (self.scalar_v277 * (((v2 + (v18 / v311))) as f64).ln()) } else { v1 }) });
        let v342: f64 = ((self.scalar_v331 * f64::powf(v98, self.scalar_v333)) * ((((v122 * self.scalar_v337) / self.scalar_v339)) as f64).exp());
        let v343: bool = (v342 > v1);
        let v350: f64 = (if (!v343) { v1 } else { (if v343 { (self.scalar_v339 * (((v2 + (v18 / v342))) as f64).ln()) } else { v1 }) });
        let v371: f64 = ctx.node_voltage(nodes[4]);
        let v373: f64 = ((self.scalar_v73 + v371) - v67);
        let v374: bool = (v373 < self.scalar_v76);
        let v377: f64 = ((((v373 - self.scalar_v75) - v2)) as f64).exp();
        let v379: f64 = (if v374 { (self.scalar_v75 + v377) } else { v373 });
        let v382: bool = ((v379 > self.scalar_v84) && (!v374));
        let v385: f64 = ((((self.scalar_v83 - v379) - v2)) as f64).exp();
        let v388: f64 = (v67 + (if v382 { (self.scalar_v83 - v385) } else { v379 }));
        let v390: f64 = ((v94 * v388) / v96);
        let v391: f64 = (v388 / v70);
        let v392: f64 = (v388 - v70);
        let v395: f64 = (self.scalar_v129 * f64::powf(v391, self.scalar_v393));
        let v459: f64 = (self.scalar_v99 * f64::powf(v391, self.scalar_v117));
        let v460: f64 = (v2 - v391);
        let v461: f64 = (self.scalar_v121 * v460);
        let v462: f64 = (self.scalar_v116 * v390);
        let v464: f64 = (((v461 / v462)) as f64).exp();
        let v465: f64 = (v459 * v464);
        let v467: f64 = (self.scalar_v158 * f64::powf(v391, self.scalar_v161));
        let v468: f64 = (self.scalar_v165 * v460);
        let v469: f64 = (self.scalar_v160 * v390);
        let v471: f64 = (((v468 / v469)) as f64).exp();
        let v472: f64 = (v467 * v471);
        let v474: f64 = (self.scalar_v195 * f64::powf(v391, self.scalar_v197));
        let v475: f64 = (self.scalar_v201 * v460);
        let v476: f64 = (self.scalar_v196 * v390);
        let v478: f64 = (((v475 / v476)) as f64).exp();
        let v479: f64 = (v474 * v478);
        let v483: f64 = (self.scalar_v229 * v390);
        let v490: f64 = (self.scalar_v250 * v390);
        let v497: f64 = (self.scalar_v270 * v390);
        let v504: f64 = (self.scalar_v290 * v390);
        let v515: f64 = (self.scalar_v332 * v390);
        let v528: f64 = (v2 + (v392 * self.scalar_v526));
        let v529: f64 = (self.scalar_v116 * v528);
        let v530: f64 = (self.scalar_v160 * v528);
        let v544: f64 = (self.scalar_v541 + (v392 * self.scalar_v542));
        let v551: f64 = (self.scalar_v102 * (v2 + (v392 * self.scalar_v548)));
        let v552: f64 = 2.0;
        let v554: f64 = (v552 * (v390 / v391));
        let v557: f64 = (v391 * self.scalar_v556);
        let v559: f64 = (((v557 / v390)) as f64).exp();
        let v560: f64 = -0.5;
        let v562: f64 = (v391 * self.scalar_v561);
        let v564: f64 = (((v562 / v390)) as f64).exp();
        let v565: f64 = (v559 - v564);
        let v566: f64 = ((v565) as f64).ln();
        let v567: f64 = (v554 * v566);
        let v569: f64 = 3.0;
        let v570: f64 = (v390 * v569);
        let v571: f64 = ((v391) as f64).ln();
        let v572: f64 = (v570 * v571);
        let v574: f64 = (v391 - v2);
        let v576: f64 = (((v391 * v567) - v572) - (self.scalar_v233 * v574));
        let v577: f64 = (v390 * v552);
        let v578: f64 = (-v576);
        let v580: f64 = (((v578 / v390)) as f64).exp();
        let v583: f64 = (((v2 + (v136 * v580))) as f64).sqrt();
        let v585: f64 = (v134 * (v2 + v583));
        let v586: f64 = ((v585) as f64).ln();
        let v588: f64 = (v576 + (v577 * v586));
        let v591: f64 = (v391 * self.scalar_v590);
        let v593: f64 = (((v591 / v390)) as f64).exp();
        let v595: f64 = (v391 * self.scalar_v594);
        let v597: f64 = (((v595 / v390)) as f64).exp();
        let v598: f64 = (v593 - v597);
        let v599: f64 = ((v598) as f64).ln();
        let v600: f64 = (v554 * v599);
        let v604: f64 = (((v391 * v600) - v572) - (self.scalar_v274 * v574));
        let v605: f64 = (-v604);
        let v607: f64 = (((v605 / v390)) as f64).exp();
        let v610: f64 = (((v2 + (v136 * v607))) as f64).sqrt();
        let v612: f64 = (v134 * (v2 + v610));
        let v613: f64 = ((v612) as f64).ln();
        let v615: f64 = (v604 + (v577 * v613));
        let v618: f64 = (v391 * self.scalar_v617);
        let v620: f64 = (((v618 / v390)) as f64).exp();
        let v622: f64 = (v391 * self.scalar_v621);
        let v624: f64 = (((v622 / v390)) as f64).exp();
        let v625: f64 = (v620 - v624);
        let v626: f64 = ((v625) as f64).ln();
        let v627: f64 = (v554 * v626);
        let v631: f64 = (((v391 * v627) - v572) - (self.scalar_v336 * v574));
        let v632: f64 = (-v631);
        let v634: f64 = (((v632 / v390)) as f64).exp();
        let v637: f64 = (((v2 + (v136 * v634))) as f64).sqrt();
        let v639: f64 = (v134 * (v2 + v637));
        let v640: f64 = ((v639) as f64).ln();
        let v642: f64 = (v631 + (v577 * v640));
        let v644: f64 = (self.scalar_v555 / v588);
        let v647: f64 = (self.scalar_v643 * f64::powf(v644, self.scalar_v645));
        let v649: f64 = (self.scalar_v589 / v615);
        let v651: f64 = f64::powf(v649, self.scalar_v650);
        let v652: f64 = (self.scalar_v648 * v651);
        let v654: f64 = (v651 * self.scalar_v653);
        let v656: f64 = (self.scalar_v616 / v642);
        let v659: f64 = (self.scalar_v655 * f64::powf(v656, self.scalar_v657));
        let v662: f64 = (self.scalar_v660 * f64::powf(v391, self.scalar_v115));
        let v664: f64 = (((v461 / v390)) as f64).exp();
        let v665: f64 = (v662 * v664);
        let v671: f64 = (v390 * v551);
        let v678: f64 = (self.scalar_v674 * (v2 + (v392 * self.scalar_v675)));
        let v683: f64 = (self.scalar_v679 * (v2 + (v392 * self.scalar_v680)));
        let v710: bool = (v678 > v1);
        let v712: f64 = (if v710 { (v2 / v678) } else { v1 });
        let v713: bool = (v683 > v1);
        let v715: f64 = (if v713 { (v2 / v683) } else { v1 });
        let v716: bool = (v395 > v1);
        let v718: f64 = (if v716 { (v2 / v395) } else { v1 });
        let v722: f64 = ctx.node_voltage(nodes[8]);
        let v723: f64 = ctx.node_voltage(nodes[9]);
        let v725: f64 = (v36 * (v722 - v723));
        let v726: f64 = ctx.node_voltage(nodes[7]);
        let v728: f64 = (v36 * (v726 - v723));
        let v731: f64 = (v36 * (v722 - ctx.node_voltage(nodes[6])));
        let v734: f64 = (v36 * (v722 - ctx.node_voltage(nodes[5])));
        let v737: f64 = ctx.node_voltage(nodes[10]);
        let v739: f64 = (v36 * (v726 - v737));
        let v740: f64 = ctx.node_voltage(nodes[1]);
        let v756: f64 = (v36 * (ctx.node_voltage(nodes[11]) - v737));
        let v763: f64 = (-v588);
        let v765: f64 = (v763 * self.scalar_v764);
        let v768: f64 = (v725 + v765);
        let v769: f64 = (if self.scalar_v767 { v768 } else { v1 });
        let v770: bool = (v769 > v1);
        let v771: bool = (self.scalar_v767 && v770);
        let v775: f64 = (if v771 { self.scalar_v774 } else { v1 });
        let v777: f64 = (v2 - (self.scalar_v772 * v775));
        let v783: f64 = (v769 * self.scalar_v782);
        let v784: f64 = (v588 * self.scalar_v772);
        let v786: f64 = (v2 + (v783 / v784));
        let v791: bool = (self.scalar_v767 && (!v770));
        let v793: f64 = (v2 - (v725 / v588));
        let v795: f64 = (v2 - f64::powf(v793, self.scalar_v779));
        let v798: f64 = (if v791 { ((v588 * v795) / self.scalar_v779) } else { (if v771 { ((v588 * v777) / self.scalar_v779) } else { v1 }) });
        let v807: f64 = ((((v765 * v765) + self.scalar_v805)) as f64).sqrt();
        let v811: f64 = (if self.scalar_v802 { (v560 * (v765 + (if self.scalar_v802 { v807 } else { v1 }))) } else { v1 });
        let v813: f64 = (v2 - (v811 / v588));
        let v814: f64 = f64::powf(v813, self.scalar_v779);
        let v817: f64 = (if self.scalar_v802 { ((v763 * v814) / self.scalar_v779) } else { v1 });
        let v818: f64 = (if self.scalar_v802 { v768 } else { v1 });
        let v821: f64 = (((self.scalar_v805 + (v818 * v818))) as f64).sqrt();
        let v826: f64 = (if self.scalar_v802 { ((v134 * (v818 - (if self.scalar_v802 { v821 } else { v1 }))) - v765) } else { v1 });
        let v828: f64 = (v2 - (v826 / v588));
        let v829: f64 = f64::powf(v828, self.scalar_v779);
        let v834: f64 = (v811 + (v725 - v826));
        let v835: f64 = (self.scalar_v774 * v834);
        let v836: f64 = (self.scalar_v782 * v834);
        let v838: f64 = (v2 + (v836 / v784));
        let v842: f64 = (if self.scalar_v802 { (((if self.scalar_v802 { ((v763 * v829) / self.scalar_v779) } else { v798 }) + (v835 * v838)) - v817) } else { (if self.scalar_v767 { (v798 + (if v791 { v1 } else { (if v771 { (v775 * (v769 * v786)) } else { v1 }) })) } else { v1 }) });
        let v843: f64 = (-v615);
        let v844: f64 = (self.scalar_v764 * v843);
        let v847: f64 = (v731 + v844);
        let v848: f64 = (if self.scalar_v846 { v847 } else { v1 });
        let v849: bool = (v848 > v1);
        let v850: bool = (self.scalar_v846 && v849);
        let v853: f64 = (if v850 { self.scalar_v852 } else { v1 });
        let v856: f64 = (v2 - (self.scalar_v772 * (self.scalar_v772 * v853)));
        let v862: f64 = (v848 * self.scalar_v861);
        let v864: f64 = (self.scalar_v772 + (v862 / v615));
        let v872: bool = (self.scalar_v869 && (v731 < self.scalar_v870));
        let v874: bool = (self.scalar_v846 && (!v849));
        let v875: bool = (v872 && v874);
        let v877: f64 = (v2 + (self.scalar_v868 / v615));
        let v878: f64 = f64::powf(v877, self.scalar_v858);
        let v880: f64 = (self.scalar_v858 * (v731 + self.scalar_v868));
        let v881: f64 = (v615 + self.scalar_v868);
        let v883: f64 = (v2 - (v880 / v881));
        let v885: f64 = (v2 - (v878 * v883));
        let v890: bool = (v874 && (!v872));
        let v892: f64 = (v2 - (v731 / v615));
        let v894: f64 = (v2 - f64::powf(v892, self.scalar_v858));
        let v897: f64 = (if v890 { ((v615 * v894) / self.scalar_v858) } else { (if v875 { ((v615 * v885) / self.scalar_v858) } else { (if v850 { ((v615 * v856) / self.scalar_v858) } else { v1 }) }) });
        let v906: f64 = (v844 + self.scalar_v868);
        let v907: f64 = (self.scalar_v868 - v844);
        let v909: f64 = (if self.scalar_v905 { (v906 / v907) } else { v1 });
        let v910: f64 = (v552 * v909);
        let v911: f64 = (v909 - v2);
        let v916: f64 = ((((v911 * v911) + self.scalar_v914)) as f64).sqrt();
        let v917: f64 = (v2 + v909);
        let v922: f64 = ((((v917 * v917) + self.scalar_v920)) as f64).sqrt();
        let v923: f64 = (v916 + v922);
        let v925: f64 = (if self.scalar_v905 { (v910 / v923) } else { v1 });
        let v930: f64 = (if self.scalar_v905 { (v134 * (((v907 * v925) - self.scalar_v868) - v844)) } else { v1 });
        let v932: f64 = (v2 - (v930 / v615));
        let v934: f64 = (v2 - f64::powf(v932, self.scalar_v858));
        let v937: f64 = (if self.scalar_v905 { ((v615 * v934) / self.scalar_v858) } else { v1 });
        let v940: f64 = (v844 + (self.scalar_v868 + (v552 * v731)));
        let v942: f64 = (if self.scalar_v905 { (v940 / v907) } else { v1 });
        let v943: f64 = (v552 * v942);
        let v944: f64 = (v942 - v2);
        let v947: f64 = (((self.scalar_v914 + (v944 * v944))) as f64).sqrt();
        let v948: f64 = (v2 + v942);
        let v951: f64 = (((self.scalar_v920 + (v948 * v948))) as f64).sqrt();
        let v952: f64 = (v947 + v951);
        let v954: f64 = (if self.scalar_v905 { (v943 / v952) } else { v1 });
        let v959: f64 = (if self.scalar_v905 { (v134 * (((v907 * v954) - self.scalar_v868) - v844)) } else { v1 });
        let v961: f64 = (v2 - (v959 / v615));
        let v963: f64 = (v2 - f64::powf(v961, self.scalar_v858));
        let v966: f64 = (if self.scalar_v905 { ((v615 * v963) / self.scalar_v858) } else { v897 });
        let v969: f64 = (if self.scalar_v905 { (v134 * (v2 + v954)) } else { v1 });
        let v972: f64 = (if self.scalar_v905 { f64::powf(v877, self.scalar_v970) } else { v1 });
        let v974: f64 = (v2 + (v844 / v615));
        let v976: f64 = (if self.scalar_v905 { f64::powf(v974, self.scalar_v970) } else { v1 });
        let v977: f64 = (v2 - v969);
        let v981: f64 = (if self.scalar_v905 { ((v972 * v977) + (v969 * v976)) } else { v1 });
        let v983: f64 = (v930 + (v731 - v959));
        let v988: f64 = (if self.scalar_v905 { ((v966 + (if self.scalar_v905 { (v981 * v983) } else { v1 })) - v937) } else { (if self.scalar_v846 { (v897 + (if v874 { v1 } else { (if v850 { (v853 * (v848 * v864)) } else { v1 }) })) } else { v1 }) });
        let v993: f64 = (((self.scalar_v914 + (v844 * v844))) as f64).sqrt();
        let v997: f64 = (if self.scalar_v990 { (v560 * (v844 + (if self.scalar_v990 { v993 } else { v1 }))) } else { v930 });
        let v999: f64 = (v2 - (v997 / v615));
        let v1000: f64 = f64::powf(v999, self.scalar_v858);
        let v1003: f64 = (if self.scalar_v990 { ((v843 * v1000) / self.scalar_v858) } else { v1 });
        let v1004: f64 = (if self.scalar_v990 { v847 } else { v1 });
        let v1007: f64 = (((self.scalar_v914 + (v1004 * v1004))) as f64).sqrt();
        let v1012: f64 = (if self.scalar_v990 { ((v134 * (v1004 - (if self.scalar_v990 { v1007 } else { v1 }))) - v844) } else { v959 });
        let v1014: f64 = (v2 - (v1012 / v615));
        let v1015: f64 = f64::powf(v1014, self.scalar_v858);
        let v1025: f64 = (if self.scalar_v990 { (((if self.scalar_v990 { ((v843 * v1015) / self.scalar_v858) } else { v966 }) + (self.scalar_v1019 * (v997 + (v731 - v1012)))) - v1003) } else { v988 });
        let v1026: f64 = (v390 * v529);
        let v1027: f64 = (v2 / v1026);
        let v1028: bool = (v725 < v157);
        let v1030: f64 = (((v725 * v1027)) as f64).exp();
        let v1032: bool = (!v1028);
        let v1034: f64 = (((v157 * v1027)) as f64).exp();
        let v1035: f64 = (v725 - v157);
        let v1037: f64 = (v2 + (v1027 * v1035));
        let v1039: f64 = (if v1032 { (v1034 * v1037) } else { (if v1028 { v1030 } else { v1 }) });
        let v1040: f64 = (v1039 - v2);
        let v1041: f64 = (v465 * v1040);
        let v1042: f64 = (v390 * v530);
        let v1043: f64 = (v2 / v1042);
        let v1044: bool = (v731 < v194);
        let v1046: f64 = (((v731 * v1043)) as f64).exp();
        let v1048: bool = (!v1044);
        let v1050: f64 = (((v194 * v1043)) as f64).exp();
        let v1051: f64 = (v731 - v194);
        let v1053: f64 = (v2 + (v1043 * v1051));
        let v1055: f64 = (if v1048 { (v1050 * v1053) } else { (if v1044 { v1046 } else { v1039 }) });
        let v1056: f64 = (v465 * v472);
        let v1057: f64 = (v1055 - v2);
        let v1058: f64 = (v1056 * v1057);
        let v1063: f64 = 0.0001;
        let v1064: f64 = (((v2 + (v715 * v842)) + (v712 * v1025)) - v1063);
        let v1066: f64 = 1e-8;
        let v1068: f64 = ((((v1064 * v1064) + v1066)) as f64).sqrt();
        let v1071: f64 = (v1063 + (v134 * (v1064 + v1068)));
        let v1079: f64 = (v136 * ((v718 * v1041) + (v44 * v1058)));
        let v1081: f64 = (if self.scalar_v1076 { (f64::powf(v1071, self.scalar_v1077) + v1079) } else { v1 });
        let v1082: bool = (v1081 > v1066);
        let v1083: bool = (self.scalar_v1076 && v1082);
        let v1089: bool = (self.scalar_v1076 && (!v1082));
        let v1096: f64 = (if self.scalar_v1094 { (v2 + v1079) } else { v1081 });
        let v1097: bool = (v1096 > v1066);
        let v1098: bool = (self.scalar_v1094 && v1097);
        let v1099: f64 = (v134 * v1071);
        let v1101: f64 = (v2 + f64::powf(v1096, self.scalar_v138));
        let v1105: bool = (self.scalar_v1094 && (!v1097));
        let v1108: f64 = (if v1105 { (v1099 * self.scalar_v1106) } else { (if v1098 { (v1099 * v1101) } else { (if v1089 { (v134 * (v1071 + self.scalar_v1090)) } else { (if v1083 { (v134 * (v1071 + f64::powf(v1081, self.scalar_v138))) } else { v1 }) }) }) });
        let v1113: f64 = (if self.scalar_v1111 { (v2 / v476) } else { v1043 });
        let v1114: bool = (v739 < v226);
        let v1115: bool = (self.scalar_v1111 && v1114);
        let v1117: f64 = (((v739 * v1113)) as f64).exp();
        let v1120: bool = (self.scalar_v1111 && (!v1114));
        let v1122: f64 = (((v226 * v1113)) as f64).exp();
        let v1123: f64 = (v739 - v226);
        let v1125: f64 = (v2 + (v1113 * v1123));
        let v1127: f64 = (if v1120 { (v1122 * v1125) } else { (if v1115 { v1117 } else { v1055 }) });
        let v1128: bool = (v731 < v226);
        let v1129: bool = (self.scalar_v1111 && v1128);
        let v1131: f64 = (((v731 * v1113)) as f64).exp();
        let v1134: bool = (self.scalar_v1111 && (!v1128));
        let v1135: f64 = (v731 - v226);
        let v1137: f64 = (v2 + (v1113 * v1135));
        let v1139: f64 = (if v1134 { (v1122 * v1137) } else { (if v1129 { v1131 } else { v1 }) });
        let v1145: f64 = (((v1127 * self.scalar_v1140) + (v1139 * self.scalar_v1142)) - v2);
        let v1163: bool = (v756 < v226);
        let v1164: bool = (self.scalar_v1111 && v1163);
        let v1166: f64 = (((v756 * v1113)) as f64).exp();
        let v1169: bool = (self.scalar_v1111 && (!v1163));
        let v1170: f64 = (v756 - v226);
        let v1172: f64 = (v2 + (v1113 * v1170));
        let v1187: f64 = (v2 / v483);
        let v1188: f64 = (if self.scalar_v1186 { v1187 } else { v1113 });
        let v1189: bool = (v725 < v247);
        let v1190: bool = (self.scalar_v1186 && v1189);
        let v1192: f64 = (((v725 * v1188)) as f64).exp();
        let v1194: bool = (!v1189);
        let v1195: bool = (self.scalar_v1186 && v1194);
        let v1197: f64 = (((v247 * v1188)) as f64).exp();
        let v1198: f64 = (v725 - v247);
        let v1200: f64 = (v2 + (v1188 * v1198));
        let v1203: f64 = (v2 / v490);
        let v1239: f64 = ((-(self.scalar_v104 * (v2 + (v392 * v544)))) - v725);
        let v1240: f64 = (if self.scalar_v1238 { v1239 } else { v1 });
        let v1241: f64 = (v2 / v671);
        let v1242: f64 = (if self.scalar_v1238 { v1241 } else { (if self.scalar_v1186 { v1203 } else { v1188 }) });
        let v1243: bool = (v1240 < v114);
        let v1244: bool = (self.scalar_v1238 && v1243);
        let v1246: f64 = (((v1240 * v1242)) as f64).exp();
        let v1249: bool = (self.scalar_v1238 && (!v1243));
        let v1251: f64 = (((v114 * v1242)) as f64).exp();
        let v1252: f64 = (v1240 - v114);
        let v1254: f64 = (v2 + (v1242 * v1252));
        let v1265: f64 = (if self.scalar_v1263 { v1187 } else { v1242 });
        let v1266: bool = (v728 < v247);
        let v1267: bool = (self.scalar_v1263 && v1266);
        let v1269: f64 = (((v728 * v1265)) as f64).exp();
        let v1271: bool = (!v1266);
        let v1272: bool = (self.scalar_v1263 && v1271);
        let v1274: f64 = (((v247 * v1265)) as f64).exp();
        let v1275: f64 = (v728 - v247);
        let v1277: f64 = (v2 + (v1265 * v1275));
        let v1279: f64 = (if v1272 { (v1274 * v1277) } else { (if v1267 { v1269 } else { (if v1195 { (v1197 * v1200) } else { (if v1190 { v1192 } else { (if v1169 { (v1122 * v1172) } else { (if v1164 { v1166 } else { v1127 }) }) }) }) }) });
        let v1302: f64 = (if self.scalar_v1301 { v1239 } else { v1240 });
        let v1303: f64 = (if self.scalar_v1301 { v1241 } else { (if self.scalar_v1263 { v1203 } else { v1265 }) });
        let v1304: bool = (v1302 < v114);
        let v1305: bool = (self.scalar_v1301 && v1304);
        let v1307: f64 = (((v1302 * v1303)) as f64).exp();
        let v1310: bool = (self.scalar_v1301 && (!v1304));
        let v1312: f64 = (((v114 * v1303)) as f64).exp();
        let v1313: f64 = (v1302 - v114);
        let v1315: f64 = (v2 + (v1303 * v1313));
        let v1324: f64 = (if self.scalar_v1323 { v1187 } else { v1303 });
        let v1325: bool = (v1189 && self.scalar_v1323);
        let v1327: f64 = (((v725 * v1324)) as f64).exp();
        let v1329: bool = (v1194 && self.scalar_v1323);
        let v1331: f64 = (((v247 * v1324)) as f64).exp();
        let v1333: f64 = (v2 + (v1198 * v1324));
        let v1362: f64 = (if self.scalar_v1361 { v1239 } else { v1302 });
        let v1363: f64 = (if self.scalar_v1361 { v1241 } else { (if self.scalar_v1323 { v1203 } else { v1324 }) });
        let v1364: bool = (v1362 < v114);
        let v1365: bool = (self.scalar_v1361 && v1364);
        let v1367: f64 = (((v1362 * v1363)) as f64).exp();
        let v1370: bool = (self.scalar_v1361 && (!v1364));
        let v1372: f64 = (((v114 * v1363)) as f64).exp();
        let v1373: f64 = (v1362 - v114);
        let v1375: f64 = (v2 + (v1363 * v1373));
        let v1377: f64 = (if v1370 { (v1372 * v1375) } else { (if v1365 { v1367 } else { (if v1310 { (v1312 * v1315) } else { (if v1305 { v1307 } else { (if v1249 { (v1251 * v1254) } else { (if v1244 { v1246 } else { v1139 }) }) }) }) }) });
        let v1383: f64 = (if self.scalar_v1323 { v1187 } else { v1363 });
        let v1384: bool = (v1266 && self.scalar_v1323);
        let v1386: f64 = (((v728 * v1383)) as f64).exp();
        let v1388: bool = (v1271 && self.scalar_v1323);
        let v1390: f64 = (((v247 * v1383)) as f64).exp();
        let v1392: f64 = (v2 + (v1275 * v1383));
        let v1415: f64 = (if self.scalar_v1361 { v1239 } else { v1362 });
        let v1416: f64 = (if self.scalar_v1361 { v1241 } else { (if self.scalar_v1323 { v1203 } else { v1383 }) });
        let v1417: bool = (v1415 < v114);
        let v1418: bool = (self.scalar_v1361 && v1417);
        let v1420: f64 = (((v1415 * v1416)) as f64).exp();
        let v1423: bool = (self.scalar_v1361 && (!v1417));
        let v1425: f64 = (((v114 * v1416)) as f64).exp();
        let v1426: f64 = (v1415 - v114);
        let v1428: f64 = (v2 + (v1416 * v1426));
        let v1436: f64 = (v2 / v497);
        let v1437: bool = (v731 < v288);
        let v1439: f64 = (((v731 * v1436)) as f64).exp();
        let v1441: bool = (!v1437);
        let v1443: f64 = (((v288 * v1436)) as f64).exp();
        let v1444: f64 = (v731 - v288);
        let v1446: f64 = (v2 + (v1436 * v1444));
        let v1448: f64 = (if v1441 { (v1443 * v1446) } else { (if v1437 { v1439 } else { (if v1388 { (v1390 * v1392) } else { (if v1384 { v1386 } else { (if v1329 { (v1331 * v1333) } else { (if v1325 { v1327 } else { v1279 }) }) }) }) }) });
        let v1449: f64 = (v2 / v504);
        let v1470: f64 = (if self.scalar_v1469 { v1436 } else { v1449 });
        let v1471: bool = (v739 < v319);
        let v1472: bool = (self.scalar_v1469 && v1471);
        let v1474: f64 = (((v739 * v1470)) as f64).exp();
        let v1477: bool = (self.scalar_v1469 && (!v1471));
        let v1479: f64 = (((v319 * v1470)) as f64).exp();
        let v1480: f64 = (v739 - v319);
        let v1482: f64 = (v2 + (v1470 * v1480));
        let v1508: f64 = (v731 / v390);
        let v1509: bool = (v1508 < v39);
        let v1510: f64 = ((v1508) as f64).exp();
        let v1512: bool = (!v1509);
        let v1513: f64 = ((v39) as f64).exp();
        let v1517: f64 = (if v1512 { (v1513 * (v2 + (v1508 - v39))) } else { (if v1509 { v1510 } else { (if v1477 { (v1479 * v1482) } else { (if v1472 { v1474 } else { v1448 }) }) }) });
        let v1518: f64 = (v734 / v390);
        let v1519: bool = (v1518 < v39);
        let v1520: f64 = ((v1518) as f64).exp();
        let v1522: bool = (!v1519);
        let v1526: f64 = (if v1522 { (v1513 * (v2 + (v1518 - v39))) } else { (if v1519 { v1520 } else { (if v1423 { (v1425 * v1428) } else { (if v1418 { v1420 } else { v1377 }) }) }) });
        let v1529: f64 = (((v2 + (v665 * v1517))) as f64).sqrt();
        let v1532: f64 = (((v2 + (v665 * v1526))) as f64).sqrt();
        let v1689: f64 = (if self.scalar_v1687 { (v2 / v515) } else { (if self.scalar_v1469 { v1449 } else { v1470 }) });
        let v1690: bool = (v756 < v350);
        let v1691: bool = (self.scalar_v1687 && v1690);
        let v1693: f64 = (((v756 * v1689)) as f64).exp();
        let v1696: bool = (self.scalar_v1687 && (!v1690));
        let v1698: f64 = (((v350 * v1689)) as f64).exp();
        let v1699: f64 = (v756 - v350);
        let v1701: f64 = (v2 + (v1689 * v1699));
        let v1784: f64 = (-v642);
        let v1786: f64 = (if self.scalar_v1783 { (self.scalar_v764 * v1784) } else { v1 });
        let v1790: f64 = (v756 + v1786);
        let v1791: f64 = (if self.scalar_v1789 { v1790 } else { v1 });
        let v1792: bool = (v1791 > v1);
        let v1793: bool = (self.scalar_v1789 && v1792);
        let v1796: f64 = (if v1793 { self.scalar_v1795 } else { v1 });
        let v1798: f64 = (v2 - (self.scalar_v772 * v1796));
        let v1804: f64 = (v1791 * self.scalar_v1803);
        let v1805: f64 = (v642 * self.scalar_v772);
        let v1807: f64 = (v2 + (v1804 / v1805));
        let v1812: bool = (self.scalar_v1789 && (!v1792));
        let v1814: f64 = (v2 - (v756 / v642));
        let v1816: f64 = (v2 - f64::powf(v1814, self.scalar_v1800));
        let v1819: f64 = (if v1812 { ((v642 * v1816) / self.scalar_v1800) } else { (if v1793 { ((v642 * v1798) / self.scalar_v1800) } else { v1 }) });
        let v1829: f64 = ((((v1786 * v1786) + self.scalar_v1827)) as f64).sqrt();
        let v1833: f64 = (if self.scalar_v1824 { (v560 * (v1786 + (if self.scalar_v1824 { v1829 } else { v1 }))) } else { v1 });
        let v1835: f64 = (v2 - (v1833 / v642));
        let v1836: f64 = f64::powf(v1835, self.scalar_v1800);
        let v1840: f64 = (if self.scalar_v1824 { v1790 } else { v1 });
        let v1843: f64 = (((self.scalar_v1827 + (v1840 * v1840))) as f64).sqrt();
        let v1848: f64 = (if self.scalar_v1824 { ((v134 * (v1840 - (if self.scalar_v1824 { v1843 } else { v1 }))) - v1786) } else { v1 });
        let v1850: f64 = (v2 - (v1848 / v642));
        let v1851: f64 = f64::powf(v1850, self.scalar_v1800);
        let v1856: f64 = (v1833 + (v756 - v1848));
        let v1857: f64 = (self.scalar_v1795 * v1856);
        let v1858: f64 = (self.scalar_v1803 * v1856);
        let v1860: f64 = (v2 + (v1858 / v1805));
        let v1864: f64 = (if self.scalar_v1824 { (((if self.scalar_v1824 { ((v1784 * v1851) / self.scalar_v1800) } else { v1819 }) + (v1857 * v1860)) - (if self.scalar_v1824 { ((v1784 * v1836) / self.scalar_v1800) } else { v1 })) } else { (if self.scalar_v1789 { (v1819 + (if v1812 { v1 } else { (if v1793 { (v1796 * (v1791 * v1807)) } else { v1 }) })) } else { v1 }) });
        let v1866: f64 = (if self.scalar_v1865 { v1 } else { v1864 });
        let v1867: f64 = (v728 + v765);
        let v1868: f64 = (if self.scalar_v767 { v1867 } else { v1 });
        let v1869: bool = (v1868 > v1);
        let v1870: bool = (self.scalar_v767 && v1869);
        let v1871: f64 = (if v1870 { self.scalar_v774 } else { v1 });
        let v1873: f64 = (v2 - (self.scalar_v772 * v1871));
        let v1877: f64 = (self.scalar_v782 * v1868);
        let v1879: f64 = (v2 + (v1877 / v784));
        let v1884: bool = (self.scalar_v767 && (!v1869));
        let v1886: f64 = (v2 - (v728 / v588));
        let v1888: f64 = (v2 - f64::powf(v1886, self.scalar_v779));
        let v1891: f64 = (if v1884 { ((v588 * v1888) / self.scalar_v779) } else { (if v1870 { ((v588 * v1873) / self.scalar_v779) } else { v1 }) });
        let v1895: f64 = (if self.scalar_v802 { v1867 } else { v1 });
        let v1898: f64 = (((self.scalar_v805 + (v1895 * v1895))) as f64).sqrt();
        let v1903: f64 = (if self.scalar_v802 { ((v134 * (v1895 - (if self.scalar_v802 { v1898 } else { v1 }))) - v765) } else { v1 });
        let v1905: f64 = (v2 - (v1903 / v588));
        let v1906: f64 = f64::powf(v1905, self.scalar_v779);
        let v1911: f64 = (v811 + (v728 - v1903));
        let v1912: f64 = (self.scalar_v774 * v1911);
        let v1913: f64 = (self.scalar_v782 * v1911);
        let v1915: f64 = (v2 + (v1913 / v784));
        let v1919: f64 = (if self.scalar_v802 { (((if self.scalar_v802 { ((v763 * v1906) / self.scalar_v779) } else { v1891 }) + (v1912 * v1915)) - v817) } else { (if self.scalar_v767 { (v1891 + (if v1884 { v1 } else { (if v1870 { (v1871 * (v1868 * v1879)) } else { v1 }) })) } else { v1 }) });
        let v1920: f64 = (v739 + v844);
        let v1921: f64 = (if self.scalar_v846 { v1920 } else { v1 });
        let v1922: bool = (v1921 > v1);
        let v1923: bool = (self.scalar_v846 && v1922);
        let v1924: f64 = (if v1923 { self.scalar_v852 } else { v1 });
        let v1927: f64 = (v2 - (self.scalar_v772 * (self.scalar_v772 * v1924)));
        let v1931: f64 = (self.scalar_v861 * v1921);
        let v1933: f64 = (self.scalar_v772 + (v1931 / v615));
        let v1938: bool = (self.scalar_v869 && (v739 < self.scalar_v870));
        let v1940: bool = (self.scalar_v846 && (!v1922));
        let v1941: bool = (v1938 && v1940);
        let v1943: f64 = (self.scalar_v858 * (v739 + self.scalar_v868));
        let v1945: f64 = (v2 - (v1943 / v881));
        let v1947: f64 = (v2 - (v878 * v1945));
        let v1952: bool = (v1940 && (!v1938));
        let v1954: f64 = (v2 - (v739 / v615));
        let v1956: f64 = (v2 - f64::powf(v1954, self.scalar_v858));
        let v1959: f64 = (if v1952 { ((v615 * v1956) / self.scalar_v858) } else { (if v1941 { ((v615 * v1947) / self.scalar_v858) } else { (if v1923 { ((v615 * v1927) / self.scalar_v858) } else { v1 }) }) });
        let v1965: f64 = (v844 + (self.scalar_v868 + (v552 * v739)));
        let v1967: f64 = (if self.scalar_v905 { (v1965 / v907) } else { v1 });
        let v1968: f64 = (v552 * v1967);
        let v1969: f64 = (v1967 - v2);
        let v1972: f64 = (((self.scalar_v914 + (v1969 * v1969))) as f64).sqrt();
        let v1973: f64 = (v2 + v1967);
        let v1976: f64 = (((self.scalar_v920 + (v1973 * v1973))) as f64).sqrt();
        let v1977: f64 = (v1972 + v1976);
        let v1979: f64 = (if self.scalar_v905 { (v1968 / v1977) } else { v1 });
        let v1984: f64 = (if self.scalar_v905 { (v134 * (((v907 * v1979) - self.scalar_v868) - v844)) } else { v1 });
        let v1986: f64 = (v2 - (v1984 / v615));
        let v1988: f64 = (v2 - f64::powf(v1986, self.scalar_v858));
        let v1991: f64 = (if self.scalar_v905 { ((v615 * v1988) / self.scalar_v858) } else { v1959 });
        let v1994: f64 = (if self.scalar_v905 { (v134 * (v2 + v1979)) } else { v1 });
        let v1995: f64 = (v2 - v1994);
        let v1999: f64 = (if self.scalar_v905 { ((v972 * v1995) + (v976 * v1994)) } else { v1 });
        let v2001: f64 = (v930 + (v739 - v1984));
        let v2006: f64 = (if self.scalar_v905 { ((v1991 + (if self.scalar_v905 { (v1999 * v2001) } else { v1 })) - v937) } else { (if self.scalar_v846 { (v1959 + (if v1940 { v1 } else { (if v1923 { (v1924 * (v1921 * v1933)) } else { v1 }) })) } else { v1 }) });
        let v2007: f64 = (if self.scalar_v990 { v1920 } else { v1 });
        let v2010: f64 = (((self.scalar_v914 + (v2007 * v2007))) as f64).sqrt();
        let v2015: f64 = (if self.scalar_v990 { ((v134 * (v2007 - (if self.scalar_v990 { v2010 } else { v1 }))) - v844) } else { v1984 });
        let v2017: f64 = (v2 - (v2015 / v615));
        let v2018: f64 = f64::powf(v2017, self.scalar_v858);
        let v2027: f64 = (if self.scalar_v990 { (((if self.scalar_v990 { ((v843 * v2018) / self.scalar_v858) } else { v1991 }) + (self.scalar_v1019 * (v997 + (v739 - v2015)))) - v1003) } else { v2006 });
        let v2029: f64 = (if (v1041 > v1) { v2 } else { v1 });
        let v2031: f64 = (v64 * (v1041 * v2029));
        let v2032: f64 = (v2 + v2031);
        let v2033: f64 = (v2031 / v2032);
        let v2035: f64 = 1.44;
        let v2036: f64 = ((v59 * v731) / v2035);
        let v2037: bool = (v2036 < v39);
        let v2038: f64 = ((v2036) as f64).exp();
        let v2040: bool = (!v2037);
        let v2049: f64 = (self.scalar_v2045 * (v2 + (v1071 * self.scalar_v2046)));
        let v2051: f64 = ((if v2040 { (v1513 * (v2 + (v2036 - v39))) } else { (if v2037 { v2038 } else { (if v1696 { (v1698 * v1701) } else { (if v1691 { v1693 } else { v1517 }) }) }) }) * self.scalar_v2050);
        let v2053: f64 = ((if (v0 != 0.0) { self.scalar_v65 } else { v1 }) + (v2033 * v2033));
        let v2056: f64 = (v2 + (v2029 * (v2051 * v2053)));
        let v2057: f64 = (v2049 * v2056);
        let v2060: f64 = (v1041 * v2057);
        let v2081: f64 = ((v740 - ctx.node_voltage(nodes[2])) * self.scalar_v2080);
        let v2083: f64 = ((v740 - ctx.node_voltage(nodes[0])) * self.scalar_v2082);
        let v2085: f64 = (v371 * self.scalar_v2084);
        let v2087: f64 = (ctx.node_voltage(nodes[12]) * self.scalar_v2086);
        let v2090: f64 = ((ctx.node_voltage(nodes[13]) * self.scalar_v2086) * 0.3333333333333333);
        let v2091: f64 = (v36 * ((self.scalar_v1185 * (v647 * v842)) + (v2060 / v1108)));
        let v2092: f64 = (v36 * (self.scalar_v1407 * (v647 * v1919)));
        let v2093: f64 = (v36 * (((v652 * v1025) + (v1058 * self.scalar_v2066)) + (v1529 * self.scalar_v2069)));
        let v2094: f64 = (v36 * (v1532 * self.scalar_v2069));
        let v2095: f64 = (v36 * ((v654 * v2027) + ((if self.scalar_v1181 { v1 } else { (if self.scalar_v1111 { (v479 * v1145) } else { v1 }) }) * self.scalar_v2066)));
        let v2096: f64 = (v36 * ((v659 * v1866) + (v756 * self.scalar_v2077)));
        let v2097: f64 = (if v374 { v377 } else { v2 });
        let v2101: f64 = (if v382 { (-(v385 * (-v2097))) } else { v2097 });
        let v2103: f64 = ((v94 * v2101) / v96);
        let v2104: f64 = (v2101 / v70);
        let v2175: f64 = (-v2104);
        let v2176: f64 = (self.scalar_v121 * v2175);
        let v2186: f64 = ((v464 * (self.scalar_v99 * (v2104 * (self.scalar_v117 * f64::powf(v391, self.scalar_v2170))))) + (v459 * (v464 * (((v462 * v2176) - (v461 * (self.scalar_v116 * v2103))) / (v462 * v462)))));
        let v2202: f64 = ((v471 * (self.scalar_v158 * (v2104 * (self.scalar_v161 * f64::powf(v391, self.scalar_v2187))))) + (v467 * (v471 * (((v469 * (self.scalar_v165 * v2175)) - (v468 * (self.scalar_v160 * v2103))) / (v469 * v469)))));
        let v2209: f64 = (self.scalar_v196 * v2103);
        let v2213: f64 = (v476 * v476);
        let v2218: f64 = ((v478 * (self.scalar_v195 * (v2104 * (self.scalar_v197 * f64::powf(v391, self.scalar_v2203))))) + (v474 * (v478 * (((v476 * (self.scalar_v201 * v2175)) - (v475 * v2209)) / v2213))));
        let v2323: f64 = (self.scalar_v526 * v2101);
        let v2342: f64 = (v552 * (((v391 * v2103) - (v390 * v2104)) / (v391 * v391)));
        let v2347: f64 = (v390 * v390);
        let v2356: f64 = ((v559 * (((v390 * (self.scalar_v556 * v2104)) - (v557 * v2103)) / v2347)) - (v564 * (((v390 * (self.scalar_v561 * v2104)) - (v562 * v2103)) / v2347)));
        let v2368: f64 = ((v571 * (v569 * v2103)) + (v570 * (v2104 / v391)));
        let v2371: f64 = ((((v567 * v2104) + (v391 * ((v566 * v2342) + (v554 * (v2356 / v565))))) - v2368) - (self.scalar_v233 * v2104));
        let v2372: f64 = (v552 * v2103);
        let v2386: f64 = ((v586 * v2372) + (v577 * ((v134 * ((v136 * (v580 * (((v390 * (-v2371)) - (v578 * v2103)) / v2347))) / (v552 * v583))) / v585)));
        let v2387: f64 = (v2371 + v2386);
        let v2400: f64 = ((v593 * (((v390 * (self.scalar_v590 * v2104)) - (v591 * v2103)) / v2347)) - (v597 * (((v390 * (self.scalar_v594 * v2104)) - (v595 * v2103)) / v2347)));
        let v2410: f64 = ((((v600 * v2104) + (v391 * ((v599 * v2342) + (v554 * (v2400 / v598))))) - v2368) - (self.scalar_v274 * v2104));
        let v2424: f64 = ((v613 * v2372) + (v577 * ((v134 * ((v136 * (v607 * (((v390 * (-v2410)) - (v605 * v2103)) / v2347))) / (v552 * v610))) / v612)));
        let v2425: f64 = (v2410 + v2424);
        let v2438: f64 = ((v620 * (((v390 * (self.scalar_v617 * v2104)) - (v618 * v2103)) / v2347)) - (v624 * (((v390 * (self.scalar_v621 * v2104)) - (v622 * v2103)) / v2347)));
        let v2448: f64 = ((((v627 * v2104) + (v391 * ((v626 * v2342) + (v554 * (v2438 / v625))))) - v2368) - (self.scalar_v336 * v2104));
        let v2462: f64 = ((v640 * v2372) + (v577 * ((v134 * ((v136 * (v634 * (((v390 * (-v2448)) - (v632 * v2103)) / v2347))) / (v552 * v637))) / v639)));
        let v2463: f64 = (v2448 + v2462);
        let v2466: f64 = (v588 * v588);
        let v2472: f64 = (self.scalar_v643 * (((-(self.scalar_v555 * v2387)) / v2466) * (self.scalar_v645 * f64::powf(v644, self.scalar_v2468))));
        let v2475: f64 = (v615 * v615);
        let v2479: f64 = (((-(self.scalar_v589 * v2425)) / v2475) * (self.scalar_v650 * f64::powf(v649, self.scalar_v1584)));
        let v2484: f64 = (v642 * v642);
        let v2503: f64 = ((v664 * (self.scalar_v660 * (v2104 * (self.scalar_v115 * f64::powf(v391, self.scalar_v2491))))) + (v662 * (v664 * (((v390 * v2176) - (v461 * v2103)) / v2347))));
        let v2509: f64 = (-(self.scalar_v104 * ((v544 * v2101) + (v392 * (self.scalar_v542 * v2101)))));
        let v2571: f64 = (-v36);
        let v2572: f64 = (-v2387);
        let v2573: f64 = (self.scalar_v764 * v2572);
        let v2574: f64 = (if self.scalar_v767 { v2573 } else { v1 });
        let v2575: f64 = (if self.scalar_v767 { v36 } else { v1 });
        let v2576: f64 = (if self.scalar_v767 { v2571 } else { v1 });
        let v2583: f64 = (self.scalar_v772 * v2387);
        let v2584: f64 = (v784 * (self.scalar_v782 * v2574));
        let v2587: f64 = (v784 * v784);
        let v2589: f64 = ((self.scalar_v782 * v2575) / v784);
        let v2590: f64 = ((self.scalar_v782 * v2576) / v784);
        let v2612: f64 = (-(v36 / v588));
        let v2613: f64 = (-(v2571 / v588));
        let v2616: f64 = (self.scalar_v779 * f64::powf(v793, self.scalar_v2614));
        let v2631: f64 = (if v791 { (((v795 * v2387) + (v588 * (-((-((-(v725 * v2387)) / v2466)) * v2616)))) / self.scalar_v779) } else { (if v771 { ((v777 * v2387) / self.scalar_v779) } else { v1 }) });
        let v2632: f64 = (if v791 { ((v588 * (-(v2612 * v2616))) / self.scalar_v779) } else { v1 });
        let v2633: f64 = (if v791 { ((v588 * (-(v2613 * v2616))) / self.scalar_v779) } else { v1 });
        let v2640: f64 = (if self.scalar_v767 { (v2631 + (if v791 { v1 } else { (if v771 { (v775 * ((v786 * v2574) + (v769 * ((v2584 - (v783 * v2583)) / v2587)))) } else { v1 }) })) } else { v1 });
        let v2643: f64 = (v765 * v2573);
        let v2650: f64 = (if self.scalar_v802 { (v560 * (v2573 + (if self.scalar_v802 { ((v2643 + v2643) / (v552 * v807)) } else { v1 }))) } else { v1 });
        let v2663: f64 = (if self.scalar_v802 { (((v814 * v2572) + (v763 * ((-(((v588 * v2650) - (v811 * v2387)) / v2466)) * (self.scalar_v779 * f64::powf(v813, self.scalar_v2614))))) / self.scalar_v779) } else { v1 });
        let v2664: f64 = (if self.scalar_v802 { v2573 } else { v1 });
        let v2665: f64 = (if self.scalar_v802 { v36 } else { v1 });
        let v2666: f64 = (if self.scalar_v802 { v2571 } else { v1 });
        let v2667: f64 = (v818 * v2664);
        let v2669: f64 = (v818 * v2665);
        let v2671: f64 = (v818 * v2666);
        let v2673: f64 = (v552 * v821);
        let v2687: f64 = (if self.scalar_v802 { ((v134 * (v2664 - (if self.scalar_v802 { ((v2667 + v2667) / v2673) } else { v1 }))) - v2573) } else { v1 });
        let v2688: f64 = (if self.scalar_v802 { (v134 * (v2665 - (if self.scalar_v802 { ((v2669 + v2669) / v2673) } else { v1 }))) } else { v1 });
        let v2689: f64 = (if self.scalar_v802 { (v134 * (v2666 - (if self.scalar_v802 { ((v2671 + v2671) / v2673) } else { v1 }))) } else { v1 });
        let v2700: f64 = (self.scalar_v779 * f64::powf(v828, self.scalar_v2614));
        let v2716: f64 = (v36 - v2688);
        let v2717: f64 = (v2571 - v2689);
        let v2718: f64 = (v2650 + (-v2687));
        let v2740: f64 = ((if self.scalar_v802 { (((v829 * v2572) + (v763 * ((-(((v588 * v2687) - (v826 * v2387)) / v2466)) * v2700))) / self.scalar_v779) } else { v2631 }) + ((v838 * (self.scalar_v774 * v2718)) + (v835 * (((v784 * (self.scalar_v782 * v2718)) - (v836 * v2583)) / v2587))));
        let v2741: f64 = ((if self.scalar_v802 { ((v763 * ((-(v2688 / v588)) * v2700)) / self.scalar_v779) } else { v2632 }) + ((v838 * (self.scalar_v774 * v2716)) + (v835 * ((self.scalar_v782 * v2716) / v784))));
        let v2742: f64 = ((if self.scalar_v802 { ((v763 * ((-(v2689 / v588)) * v2700)) / self.scalar_v779) } else { v2633 }) + ((v838 * (self.scalar_v774 * v2717)) + (v835 * ((self.scalar_v782 * v2717) / v784))));
        let v2744: f64 = (if self.scalar_v802 { (v2740 - v2663) } else { v2640 });
        let v2745: f64 = (if self.scalar_v802 { v2741 } else { (if self.scalar_v767 { (v2632 + (if v791 { v1 } else { (if v771 { (v775 * ((v786 * v2575) + (v769 * v2589))) } else { v1 }) })) } else { v1 }) });
        let v2746: f64 = (if self.scalar_v802 { v2742 } else { (if self.scalar_v767 { (v2633 + (if v791 { v1 } else { (if v771 { (v775 * ((v786 * v2576) + (v769 * v2590))) } else { v1 }) })) } else { v1 }) });
        let v2747: f64 = (-v2425);
        let v2748: f64 = (self.scalar_v764 * v2747);
        let v2749: f64 = (if self.scalar_v846 { v2748 } else { v1 });
        let v2750: f64 = (if self.scalar_v846 { v2571 } else { v1 });
        let v2751: f64 = (if self.scalar_v846 { v36 } else { v1 });
        let v2758: f64 = (v615 * (self.scalar_v861 * v2749));
        let v2762: f64 = ((self.scalar_v861 * v2750) / v615);
        let v2763: f64 = ((self.scalar_v861 * v2751) / v615);
        let v2781: f64 = ((-(self.scalar_v868 * v2425)) / v2475);
        let v2785: f64 = (v2781 * (self.scalar_v858 * f64::powf(v877, self.scalar_v2782)));
        let v2790: f64 = (v881 * v881);
        let v2811: f64 = ((v615 * (-(v878 * (-((self.scalar_v858 * v2571) / v881))))) / self.scalar_v858);
        let v2812: f64 = ((v615 * (-(v878 * (-((v36 * self.scalar_v858) / v881))))) / self.scalar_v858);
        let v2813: f64 = (if v875 { (((v885 * v2425) + (v615 * (-((v883 * v2785) + (v878 * (-((-(v880 * v2425)) / v2790))))))) / self.scalar_v858) } else { (if v850 { ((v856 * v2425) / self.scalar_v858) } else { v1 }) });
        let v2822: f64 = (-(v2571 / v615));
        let v2823: f64 = (-(v36 / v615));
        let v2825: f64 = (self.scalar_v858 * f64::powf(v892, self.scalar_v2782));
        let v2840: f64 = (if v890 { (((v894 * v2425) + (v615 * (-((-((-(v731 * v2425)) / v2475)) * v2825)))) / self.scalar_v858) } else { v2813 });
        let v2841: f64 = (if v890 { ((v615 * (-(v2822 * v2825))) / self.scalar_v858) } else { (if v875 { v2811 } else { v1 }) });
        let v2842: f64 = (if v890 { ((v615 * (-(v2823 * v2825))) / self.scalar_v858) } else { (if v875 { v2812 } else { v1 }) });
        let v2849: f64 = (if self.scalar_v846 { (v2840 + (if v874 { v1 } else { (if v850 { (v853 * ((v864 * v2749) + (v848 * ((v2758 - (v862 * v2425)) / v2475)))) } else { v1 }) })) } else { v1 });
        let v2852: f64 = (-v2748);
        let v2853: f64 = (v907 * v2748);
        let v2856: f64 = (v907 * v907);
        let v2858: f64 = (if self.scalar_v905 { ((v2853 - (v906 * v2852)) / v2856) } else { v1 });
        let v2860: f64 = (v911 * v2858);
        let v2864: f64 = (v917 * v2858);
        let v2873: f64 = (((v923 * (v552 * v2858)) - (v910 * (((v2860 + v2860) / (v552 * v916)) + ((v2864 + v2864) / (v552 * v922))))) / (v923 * v923));
        let v2880: f64 = (if self.scalar_v905 { (v134 * (((v925 * v2852) + (v907 * (if self.scalar_v905 { v2873 } else { v1 }))) - v2748)) } else { v1 });
        let v2893: f64 = (((v934 * v2425) + (v615 * (-((-(((v615 * v2880) - (v930 * v2425)) / v2475)) * (self.scalar_v858 * f64::powf(v932, self.scalar_v2782)))))) / self.scalar_v858);
        let v2894: f64 = (if self.scalar_v905 { v2893 } else { v1 });
        let v2902: f64 = (if self.scalar_v905 { ((v2853 - (v940 * v2852)) / v2856) } else { v1 });
        let v2903: f64 = (if self.scalar_v905 { ((v552 * v2571) / v907) } else { v1 });
        let v2904: f64 = (if self.scalar_v905 { ((v36 * v552) / v907) } else { v1 });
        let v2906: f64 = (v552 * v2903);
        let v2907: f64 = (v552 * v2904);
        let v2908: f64 = (v944 * v2902);
        let v2910: f64 = (v944 * v2903);
        let v2912: f64 = (v944 * v2904);
        let v2914: f64 = (v552 * v947);
        let v2918: f64 = (v948 * v2902);
        let v2920: f64 = (v948 * v2903);
        let v2922: f64 = (v948 * v2904);
        let v2924: f64 = (v552 * v951);
        let v2934: f64 = (v952 * v952);
        let v2944: f64 = (if self.scalar_v905 { (((v952 * (v552 * v2902)) - (v943 * (((v2908 + v2908) / v2914) + ((v2918 + v2918) / v2924)))) / v2934) } else { v1 });
        let v2945: f64 = (if self.scalar_v905 { (((v952 * v2906) - (v943 * (((v2910 + v2910) / v2914) + ((v2920 + v2920) / v2924)))) / v2934) } else { v1 });
        let v2946: f64 = (if self.scalar_v905 { (((v952 * v2907) - (v943 * (((v2912 + v2912) / v2914) + ((v2922 + v2922) / v2924)))) / v2934) } else { v1 });
        let v2956: f64 = (if self.scalar_v905 { (v134 * (((v954 * v2852) + (v907 * v2944)) - v2748)) } else { v1 });
        let v2957: f64 = (if self.scalar_v905 { (v134 * (v907 * v2945)) } else { v1 });
        let v2958: f64 = (if self.scalar_v905 { (v134 * (v907 * v2946)) } else { v1 });
        let v2969: f64 = (self.scalar_v858 * f64::powf(v961, self.scalar_v2782));
        let v2984: f64 = (if self.scalar_v905 { (((v963 * v2425) + (v615 * (-((-(((v615 * v2956) - (v959 * v2425)) / v2475)) * v2969)))) / self.scalar_v858) } else { v2840 });
        let v2985: f64 = (if self.scalar_v905 { ((v615 * (-((-(v2957 / v615)) * v2969))) / self.scalar_v858) } else { v2841 });
        let v2986: f64 = (if self.scalar_v905 { ((v615 * (-((-(v2958 / v615)) * v2969))) / self.scalar_v858) } else { v2842 });
        let v2990: f64 = (if self.scalar_v905 { (v134 * v2944) } else { v1 });
        let v2991: f64 = (if self.scalar_v905 { (v134 * v2945) } else { v1 });
        let v2992: f64 = (if self.scalar_v905 { (v134 * v2946) } else { v1 });
        let v2997: f64 = (if self.scalar_v905 { (v2781 * (self.scalar_v970 * f64::powf(v877, self.scalar_v2993))) } else { v1 });
        let v3005: f64 = (if self.scalar_v905 { ((((v615 * v2748) - (v844 * v2425)) / v2475) * (self.scalar_v970 * f64::powf(v974, self.scalar_v2993))) } else { v1 });
        let v3031: f64 = ((v983 * (if self.scalar_v905 { (((v977 * v2997) + (v972 * (-v2990))) + ((v976 * v2990) + (v969 * v3005))) } else { v1 })) + (v981 * (v2880 + (-v2956))));
        let v3046: f64 = (if self.scalar_v905 { (v2985 + (if self.scalar_v905 { ((v983 * (if self.scalar_v905 { ((v972 * (-v2991)) + (v976 * v2991)) } else { v1 })) + (v981 * (v2571 - v2957))) } else { v1 })) } else { (if self.scalar_v846 { (v2841 + (if v874 { v1 } else { (if v850 { (v853 * ((v864 * v2750) + (v848 * v2762))) } else { v1 }) })) } else { v1 }) });
        let v3047: f64 = (if self.scalar_v905 { (v2986 + (if self.scalar_v905 { ((v983 * (if self.scalar_v905 { ((v972 * (-v2992)) + (v976 * v2992)) } else { v1 })) + (v981 * (v36 - v2958))) } else { v1 })) } else { (if self.scalar_v846 { (v2842 + (if v874 { v1 } else { (if v850 { (v853 * ((v864 * v2751) + (v848 * v2763))) } else { v1 }) })) } else { v1 }) });
        let v3048: f64 = (v844 * v2748);
        let v3055: f64 = (if self.scalar_v990 { (v560 * (v2748 + (if self.scalar_v990 { ((v3048 + v3048) / (v552 * v993)) } else { v1 }))) } else { v2880 });
        let v3068: f64 = (if self.scalar_v990 { (((v1000 * v2747) + (v843 * ((-(((v615 * v3055) - (v997 * v2425)) / v2475)) * (self.scalar_v858 * f64::powf(v999, self.scalar_v2782))))) / self.scalar_v858) } else { v1 });
        let v3069: f64 = (if self.scalar_v990 { v2748 } else { v1 });
        let v3070: f64 = (if self.scalar_v990 { v2571 } else { v1 });
        let v3071: f64 = (if self.scalar_v990 { v36 } else { v1 });
        let v3072: f64 = (v1004 * v3069);
        let v3074: f64 = (v1004 * v3070);
        let v3076: f64 = (v1004 * v3071);
        let v3078: f64 = (v552 * v1007);
        let v3092: f64 = (if self.scalar_v990 { ((v134 * (v3069 - (if self.scalar_v990 { ((v3072 + v3072) / v3078) } else { v1 }))) - v2748) } else { v2956 });
        let v3093: f64 = (if self.scalar_v990 { (v134 * (v3070 - (if self.scalar_v990 { ((v3074 + v3074) / v3078) } else { v1 }))) } else { v2957 });
        let v3094: f64 = (if self.scalar_v990 { (v134 * (v3071 - (if self.scalar_v990 { ((v3076 + v3076) / v3078) } else { v1 }))) } else { v2958 });
        let v3105: f64 = (self.scalar_v858 * f64::powf(v1014, self.scalar_v2782));
        let v3127: f64 = ((if self.scalar_v990 { (((v1015 * v2747) + (v843 * ((-(((v615 * v3092) - (v1012 * v2425)) / v2475)) * v3105))) / self.scalar_v858) } else { v2984 }) + (self.scalar_v1019 * (v3055 + (-v3092))));
        let v3131: f64 = (if self.scalar_v990 { (v3127 - v3068) } else { (if self.scalar_v905 { ((v2984 + (if self.scalar_v905 { v3031 } else { v1 })) - v2894) } else { v2849 }) });
        let v3132: f64 = (if self.scalar_v990 { ((if self.scalar_v990 { ((v843 * ((-(v3093 / v615)) * v3105)) / self.scalar_v858) } else { v2985 }) + (self.scalar_v1019 * (v2571 - v3093))) } else { v3046 });
        let v3133: f64 = (if self.scalar_v990 { ((if self.scalar_v990 { ((v843 * ((-(v3094 / v615)) * v3105)) / self.scalar_v858) } else { v2986 }) + (self.scalar_v1019 * (v36 - v3094))) } else { v3047 });
        let v3139: f64 = ((-((v529 * v2103) + (v390 * (self.scalar_v116 * v2323)))) / (v1026 * v1026));
        let v3141: f64 = (v36 * v1027);
        let v3142: f64 = (v1027 * v2571);
        let v3157: f64 = (if v1032 { ((v1037 * (v1034 * (v157 * v3139))) + (v1034 * (v1035 * v3139))) } else { (if v1028 { (v1030 * (v725 * v3139)) } else { v1 }) });
        let v3158: f64 = (if v1032 { (v1034 * v3141) } else { (if v1028 { (v1030 * v3141) } else { v1 }) });
        let v3159: f64 = (if v1032 { (v1034 * v3142) } else { (if v1028 { (v1030 * v3142) } else { v1 }) });
        let v3162: f64 = ((v1040 * v2186) + (v465 * v3157));
        let v3163: f64 = (v465 * v3158);
        let v3164: f64 = (v465 * v3159);
        let v3170: f64 = ((-((v530 * v2103) + (v390 * (self.scalar_v160 * v2323)))) / (v1042 * v1042));
        let v3172: f64 = (v1043 * v2571);
        let v3173: f64 = (v36 * v1043);
        let v3189: f64 = (if v1048 { ((v1053 * (v1050 * (v194 * v3170))) + (v1050 * (v1051 * v3170))) } else { (if v1044 { (v1046 * (v731 * v3170)) } else { v3157 }) });
        let v3190: f64 = (if v1048 { (v1050 * v3172) } else { (if v1044 { (v1046 * v3172) } else { v1 }) });
        let v3191: f64 = (if v1048 { (v1050 * v3173) } else { (if v1044 { (v1046 * v3173) } else { v3158 }) });
        let v3192: f64 = (if v1048 { v1 } else { (if v1044 { v1 } else { v3159 }) });
        let v3198: f64 = ((v1057 * ((v472 * v2186) + (v465 * v2202))) + (v1056 * v3189));
        let v3199: f64 = (v1056 * v3190);
        let v3200: f64 = (v1056 * v3191);
        let v3201: f64 = (v1056 * v3192);
        let v3206: f64 = (v715 * v2746);
        let v3210: f64 = (v712 * v3132);
        let v3212: f64 = (((v842 * (if v713 { ((-(self.scalar_v679 * (self.scalar_v680 * v2101))) / (v683 * v683)) } else { v1 })) + (v715 * v2744)) + ((v1025 * (if v710 { ((-(self.scalar_v674 * (self.scalar_v675 * v2101))) / (v678 * v678)) } else { v1 })) + (v712 * v3131)));
        let v3213: f64 = ((v715 * v2745) + (v712 * v3133));
        let v3214: f64 = (v1064 * v3212);
        let v3216: f64 = (v1064 * v3210);
        let v3218: f64 = (v1064 * v3213);
        let v3220: f64 = (v1064 * v3206);
        let v3222: f64 = (v552 * v1068);
        let v3231: f64 = (v134 * (v3212 + ((v3214 + v3214) / v3222)));
        let v3232: f64 = (v134 * (v3210 + ((v3216 + v3216) / v3222)));
        let v3233: f64 = (v134 * (v3213 + ((v3218 + v3218) / v3222)));
        let v3234: f64 = (v134 * (v3206 + ((v3220 + v3220) / v3222)));
        let v3244: f64 = (((v1041 * (if v716 { ((-(self.scalar_v129 * (v2104 * (self.scalar_v393 * f64::powf(v391, self.scalar_v2105))))) / (v395 * v395)) } else { v1 })) + (v718 * v3162)) + (v44 * v3198));
        let v3249: f64 = (self.scalar_v1077 * f64::powf(v1071, self.scalar_v3247));
        let v3254: f64 = (v136 * v3244);
        let v3255: f64 = (v136 * (v44 * v3199));
        let v3256: f64 = (v136 * ((v718 * v3163) + (v44 * v3200)));
        let v3257: f64 = (v136 * ((v718 * v3164) + (v44 * v3201)));
        let v3262: f64 = (if self.scalar_v1076 { ((v3231 * v3249) + v3254) } else { v1 });
        let v3263: f64 = (if self.scalar_v1076 { ((v3232 * v3249) + v3255) } else { v1 });
        let v3264: f64 = (if self.scalar_v1076 { ((v3233 * v3249) + v3256) } else { v1 });
        let v3265: f64 = (if self.scalar_v1076 { ((v3234 * v3249) + v3257) } else { v1 });
        let v3268: f64 = (self.scalar_v138 * f64::powf(v1081, self.scalar_v3266));
        let v3285: f64 = (v134 * v3231);
        let v3286: f64 = (v134 * v3232);
        let v3287: f64 = (v134 * v3233);
        let v3288: f64 = (v134 * v3234);
        let v3298: f64 = (self.scalar_v138 * f64::powf(v1096, self.scalar_v3266));
        let v3315: f64 = (if v1098 { ((v1101 * v3285) + (v1099 * ((if self.scalar_v1094 { v3254 } else { v3262 }) * v3298))) } else { (if v1089 { v3285 } else { (if v1083 { (v134 * (v3231 + (v3262 * v3268))) } else { v1 }) }) });
        let v3316: f64 = (if v1098 { ((v1101 * v3286) + (v1099 * ((if self.scalar_v1094 { v3255 } else { v3263 }) * v3298))) } else { (if v1089 { v3286 } else { (if v1083 { (v134 * (v3232 + (v3263 * v3268))) } else { v1 }) }) });
        let v3317: f64 = (if v1098 { ((v1101 * v3287) + (v1099 * ((if self.scalar_v1094 { v3256 } else { v3264 }) * v3298))) } else { (if v1089 { v3287 } else { (if v1083 { (v134 * (v3233 + (v3264 * v3268))) } else { v1 }) }) });
        let v3318: f64 = (if v1098 { ((v1101 * v3288) + (v1099 * ((if self.scalar_v1094 { v3257 } else { v3265 }) * v3298))) } else { (if v1089 { v3288 } else { (if v1083 { (v134 * (v3234 + (v3265 * v3268))) } else { v1 }) }) });
        let v3330: f64 = (v1108 * v1108);
        let v3361: f64 = (if self.scalar_v1111 { ((-v2209) / v2213) } else { v3170 });
        let v3363: f64 = (v36 * v1113);
        let v3364: f64 = (v1113 * v2571);
        let v3375: f64 = (v1122 * (v226 * v3361));
        let v3380: f64 = (v1122 * v3363);
        let v3381: f64 = (v1122 * v3364);
        let v3382: f64 = (if v1120 { ((v1125 * v3375) + (v1122 * (v1123 * v3361))) } else { (if v1115 { (v1117 * (v739 * v3361)) } else { v3189 }) });
        let v3383: f64 = (if v1120 { v1 } else { (if v1115 { v1 } else { v3190 }) });
        let v3384: f64 = (if v1120 { v3380 } else { (if v1115 { (v1117 * v3363) } else { v1 }) });
        let v3385: f64 = (if v1120 { v1 } else { (if v1115 { v1 } else { v3191 }) });
        let v3386: f64 = (if v1120 { v1 } else { (if v1115 { v1 } else { v3192 }) });
        let v3387: f64 = (if v1120 { v3381 } else { (if v1115 { (v1117 * v3364) } else { v1 }) });
        let v3399: f64 = (if v1134 { ((v1137 * v3375) + (v1122 * (v1135 * v3361))) } else { (if v1129 { (v1131 * (v731 * v3361)) } else { v1 }) });
        let v3400: f64 = (if v1134 { v3381 } else { (if v1129 { (v1131 * v3364) } else { v1 }) });
        let v3401: f64 = (if v1134 { v3380 } else { (if v1129 { (v1131 * v3363) } else { v1 }) });
        let v3575: f64 = ((-(self.scalar_v229 * v2103)) / (v483 * v483));
        let v3576: f64 = (if self.scalar_v1186 { v3575 } else { v3361 });
        let v3578: f64 = (v36 * v1188);
        let v3579: f64 = (v1188 * v2571);
        let v3583: f64 = (if v1190 { (v1192 * (v725 * v3576)) } else { (if v1169 { ((v1172 * v3375) + (v1122 * (v1170 * v3361))) } else { (if v1164 { (v1166 * (v756 * v3361)) } else { v3382 }) }) });
        let v3606: f64 = ((-(self.scalar_v250 * v2103)) / (v490 * v490));
        let v3687: f64 = (if self.scalar_v1238 { v2509 } else { v1 });
        let v3688: f64 = (if self.scalar_v1238 { v2571 } else { v1 });
        let v3689: f64 = (if self.scalar_v1238 { v36 } else { v1 });
        let v3691: f64 = ((-((v551 * v2103) + (v390 * (self.scalar_v102 * (self.scalar_v548 * v2101))))) / (v671 * v671));
        let v3692: f64 = (if self.scalar_v1238 { v3691 } else { (if self.scalar_v1186 { v3606 } else { v3576 }) });
        let v3693: f64 = (v1242 * v3687);
        let v3696: f64 = (v1242 * v3688);
        let v3697: f64 = (v1242 * v3689);
        let v3714: f64 = (if v1249 { ((v1254 * (v1251 * (v114 * v3692))) + (v1251 * (v3693 + (v1252 * v3692)))) } else { (if v1244 { (v1246 * (v3693 + (v1240 * v3692))) } else { v3399 }) });
        let v3738: f64 = (if self.scalar_v1263 { v3575 } else { v3692 });
        let v3740: f64 = (v36 * v1265);
        let v3741: f64 = (v1265 * v2571);
        let v3760: f64 = (if v1272 { ((v1277 * (v1274 * (v247 * v3738))) + (v1274 * (v1275 * v3738))) } else { (if v1267 { (v1269 * (v728 * v3738)) } else { (if v1195 { ((v1200 * (v1197 * (v247 * v3576))) + (v1197 * (v1198 * v3576))) } else { v3583 }) }) });
        let v3764: f64 = (if v1272 { (v1274 * v3741) } else { (if v1267 { (v1269 * v3741) } else { (if v1195 { (v1197 * v3579) } else { (if v1190 { (v1192 * v3579) } else { (if v1169 { v1 } else { (if v1164 { v1 } else { v3386 }) }) }) }) }) });
        let v3816: f64 = (if self.scalar_v1301 { v2509 } else { v3687 });
        let v3817: f64 = (if self.scalar_v1301 { v2571 } else { v3688 });
        let v3818: f64 = (if self.scalar_v1301 { v36 } else { v3689 });
        let v3819: f64 = (if self.scalar_v1301 { v3691 } else { (if self.scalar_v1263 { v3606 } else { v3738 }) });
        let v3820: f64 = (v1303 * v3816);
        let v3823: f64 = (v1303 * v3817);
        let v3824: f64 = (v1303 * v3818);
        let v3841: f64 = (if v1310 { ((v1315 * (v1312 * (v114 * v3819))) + (v1312 * (v3820 + (v1313 * v3819)))) } else { (if v1305 { (v1307 * (v3820 + (v1302 * v3819))) } else { v3714 }) });
        let v3858: f64 = (if self.scalar_v1323 { v3575 } else { v3819 });
        let v3860: f64 = (v36 * v1324);
        let v3861: f64 = (v1324 * v2571);
        let v3867: f64 = (if v1325 { v1 } else { (if v1272 { (v1274 * v3740) } else { (if v1267 { (v1269 * v3740) } else { (if v1195 { v1 } else { (if v1190 { v1 } else { (if v1169 { v1 } else { (if v1164 { v1 } else { v3384 }) }) }) }) }) }) });
        let v3868: f64 = (if v1325 { (v1327 * v3860) } else { (if v1272 { v1 } else { (if v1267 { v1 } else { (if v1195 { (v1197 * v3578) } else { (if v1190 { (v1192 * v3578) } else { (if v1169 { v1 } else { (if v1164 { v1 } else { v3385 }) }) }) }) }) }) });
        let v3881: f64 = (if v1329 { v1 } else { (if v1325 { v1 } else { (if v1272 { v1 } else { (if v1267 { v1 } else { (if v1195 { v1 } else { (if v1190 { v1 } else { (if v1169 { v1 } else { (if v1164 { v1 } else { v3383 }) }) }) }) }) }) }) });
        let v3885: f64 = (if v1329 { v1 } else { (if v1325 { v1 } else { (if v1272 { v1 } else { (if v1267 { v1 } else { (if v1195 { v1 } else { (if v1190 { v1 } else { (if v1169 { v3381 } else { (if v1164 { (v1166 * v3364) } else { v3387 }) }) }) }) }) }) }) });
        let v3886: f64 = (if v1329 { v1 } else { (if v1325 { v1 } else { (if v1272 { v1 } else { (if v1267 { v1 } else { (if v1195 { v1 } else { (if v1190 { v1 } else { (if v1169 { v3380 } else { (if v1164 { (v1166 * v3363) } else { v1 }) }) }) }) }) }) }) });
        let v3976: f64 = (if self.scalar_v1361 { v2509 } else { v3816 });
        let v3977: f64 = (if self.scalar_v1361 { v2571 } else { v3817 });
        let v3978: f64 = (if self.scalar_v1361 { v36 } else { v3818 });
        let v3979: f64 = (if self.scalar_v1361 { v3691 } else { (if self.scalar_v1323 { v3606 } else { v3858 }) });
        let v3980: f64 = (v1363 * v3976);
        let v3983: f64 = (v1363 * v3977);
        let v3984: f64 = (v1363 * v3978);
        let v3990: f64 = (if v1365 { (v1367 * v3983) } else { (if v1310 { (v1312 * v3823) } else { (if v1305 { (v1307 * v3823) } else { (if v1249 { (v1251 * v3696) } else { (if v1244 { (v1246 * v3696) } else { v3401 }) }) }) }) });
        let v3991: f64 = (if v1365 { (v1367 * v3984) } else { (if v1310 { (v1312 * v3824) } else { (if v1305 { (v1307 * v3824) } else { (if v1249 { (v1251 * v3697) } else { (if v1244 { (v1246 * v3697) } else { v1 }) }) }) }) });
        let v4001: f64 = (if v1370 { ((v1375 * (v1372 * (v114 * v3979))) + (v1372 * (v3980 + (v1373 * v3979)))) } else { (if v1365 { (v1367 * (v3980 + (v1362 * v3979))) } else { v3841 }) });
        let v4018: f64 = (if self.scalar_v1323 { v3575 } else { v3979 });
        let v4020: f64 = (v36 * v1383);
        let v4021: f64 = (v1383 * v2571);
        let v4025: f64 = (if v1384 { (v1386 * (v728 * v4018)) } else { (if v1329 { ((v1333 * (v1331 * (v247 * v3858))) + (v1331 * (v1198 * v3858))) } else { (if v1325 { (v1327 * (v725 * v3858)) } else { v3760 }) }) });
        let v4106: f64 = (if self.scalar_v1361 { v3691 } else { (if self.scalar_v1323 { v3606 } else { v4018 }) });
        let v4107: f64 = (v1416 * (if self.scalar_v1361 { v2509 } else { v3976 }));
        let v4110: f64 = (v1416 * (if self.scalar_v1361 { v2571 } else { v3977 }));
        let v4111: f64 = (v1416 * (if self.scalar_v1361 { v36 } else { v3978 }));
        let v4128: f64 = (if v1423 { ((v1428 * (v1425 * (v114 * v4106))) + (v1425 * (v4107 + (v1426 * v4106)))) } else { (if v1418 { (v1420 * (v4107 + (v1415 * v4106))) } else { v4001 }) });
        let v4129: f64 = (if v1423 { v1 } else { (if v1418 { v1 } else { (if v1370 { v1 } else { (if v1365 { v1 } else { (if v1310 { v1 } else { (if v1305 { v1 } else { (if v1249 { v1 } else { (if v1244 { v1 } else { v3400 }) }) }) }) }) }) }) });
        let v4146: f64 = ((-(self.scalar_v270 * v2103)) / (v497 * v497));
        let v4148: f64 = (v1436 * v2571);
        let v4149: f64 = (v36 * v1436);
        let v4168: f64 = (if v1441 { ((v1446 * (v1443 * (v288 * v4146))) + (v1443 * (v1444 * v4146))) } else { (if v1437 { (v1439 * (v731 * v4146)) } else { (if v1388 { ((v1392 * (v1390 * (v247 * v4018))) + (v1390 * (v1275 * v4018))) } else { v4025 }) }) });
        let v4172: f64 = (if v1441 { v1 } else { (if v1437 { v1 } else { (if v1388 { (v1390 * v4021) } else { (if v1384 { (v1386 * v4021) } else { (if v1329 { (v1331 * v3861) } else { (if v1325 { (v1327 * v3861) } else { v3764 }) }) }) }) }) });
        let v4176: f64 = ((-(self.scalar_v290 * v2103)) / (v504 * v504));
        let v4222: f64 = (if self.scalar_v1469 { v4146 } else { v4176 });
        let v4224: f64 = (v36 * v1470);
        let v4225: f64 = (v1470 * v2571);
        let v4231: f64 = (if v1472 { (v1474 * v4224) } else { (if v1441 { v1 } else { (if v1437 { v1 } else { (if v1388 { (v1390 * v4020) } else { (if v1384 { (v1386 * v4020) } else { (if v1329 { v1 } else { v3867 }) }) }) }) }) });
        let v4232: f64 = (if v1472 { v1 } else { (if v1441 { (v1443 * v4149) } else { (if v1437 { (v1439 * v4149) } else { (if v1388 { v1 } else { (if v1384 { v1 } else { (if v1329 { (v1331 * v3860) } else { v3868 }) }) }) }) }) });
        let v4317: f64 = ((-(v731 * v2103)) / v2347);
        let v4318: f64 = (v2571 / v390);
        let v4319: f64 = (v36 / v390);
        let v4323: f64 = (if v1509 { (v1510 * v4317) } else { (if v1477 { ((v1482 * (v1479 * (v319 * v4222))) + (v1479 * (v1480 * v4222))) } else { (if v1472 { (v1474 * (v739 * v4222)) } else { v4168 }) }) });
        let v4324: f64 = (if v1509 { (v1510 * v4318) } else { (if v1477 { v1 } else { (if v1472 { v1 } else { (if v1441 { (v1443 * v4148) } else { (if v1437 { (v1439 * v4148) } else { (if v1388 { v1 } else { (if v1384 { v1 } else { v3881 }) }) }) }) }) }) });
        let v4328: f64 = (if v1509 { v1 } else { (if v1477 { (v1479 * v4225) } else { (if v1472 { (v1474 * v4225) } else { (if v1441 { v1 } else { (if v1437 { v1 } else { (if v1388 { v1 } else { (if v1384 { v1 } else { v3885 }) }) }) }) }) }) });
        let v4331: f64 = (v1513 * v4318);
        let v4332: f64 = (v1513 * v4319);
        let v4333: f64 = (if v1512 { (v1513 * v4317) } else { v4323 });
        let v4334: f64 = (if v1512 { v4331 } else { v4324 });
        let v4335: f64 = (if v1512 { v1 } else { (if v1509 { v1 } else { (if v1477 { (v1479 * v4224) } else { v4231 }) }) });
        let v4336: f64 = (if v1512 { v4332 } else { (if v1509 { (v1510 * v4319) } else { (if v1477 { v1 } else { v4232 }) }) });
        let v4337: f64 = (if v1512 { v1 } else { (if v1509 { v1 } else { (if v1477 { v1 } else { (if v1472 { v1 } else { v4172 }) }) }) });
        let v4338: f64 = (if v1512 { v1 } else { v4328 });
        let v4339: f64 = (if v1512 { v1 } else { (if v1509 { v1 } else { (if v1477 { v1 } else { (if v1472 { v1 } else { (if v1441 { v1 } else { (if v1437 { v1 } else { (if v1388 { v1 } else { (if v1384 { v1 } else { v3886 }) }) }) }) }) }) }) });
        let v4342: f64 = ((-(v734 * v2103)) / v2347);
        let v4366: f64 = (v552 * v1529);
        let v4379: f64 = (v665 * (if v1522 { v4332 } else { (if v1519 { (v1520 * v4319) } else { (if v1423 { (v1425 * v4110) } else { (if v1418 { (v1420 * v4110) } else { (if v1370 { (v1372 * v3983) } else { v3990 }) }) }) }) }));
        let v4381: f64 = (v552 * v1532);
        let v4386: f64 = ((v665 * (if v1522 { v1 } else { (if v1519 { v1 } else { (if v1423 { (v1425 * v4111) } else { (if v1418 { (v1420 * v4111) } else { (if v1370 { (v1372 * v3984) } else { v3991 }) }) }) }) })) / v4381);
        let v4866: f64 = (if self.scalar_v1687 { ((-(self.scalar_v332 * v2103)) / (v515 * v515)) } else { (if self.scalar_v1469 { v4176 } else { v4222 }) });
        let v4868: f64 = (v1689 * v2571);
        let v4869: f64 = (v36 * v1689);
        let v5248: f64 = (-v2463);
        let v5250: f64 = (if self.scalar_v1783 { (self.scalar_v764 * v5248) } else { v1 });
        let v5251: f64 = (if self.scalar_v1789 { v5250 } else { v1 });
        let v5252: f64 = (if self.scalar_v1789 { v2571 } else { v1 });
        let v5253: f64 = (if self.scalar_v1789 { v36 } else { v1 });
        let v5260: f64 = (self.scalar_v772 * v2463);
        let v5264: f64 = (v1805 * v1805);
        let v5293: f64 = (self.scalar_v1800 * f64::powf(v1814, self.scalar_v5291));
        let v5308: f64 = (if v1812 { (((v1816 * v2463) + (v642 * (-((-((-(v756 * v2463)) / v2484)) * v5293)))) / self.scalar_v1800) } else { (if v1793 { ((v1798 * v2463) / self.scalar_v1800) } else { v1 }) });
        let v5309: f64 = (if v1812 { ((v642 * (-((-(v2571 / v642)) * v5293))) / self.scalar_v1800) } else { v1 });
        let v5310: f64 = (if v1812 { ((v642 * (-((-(v36 / v642)) * v5293))) / self.scalar_v1800) } else { v1 });
        let v5311: f64 = (if v1812 { v1 } else { (if v1793 { (v1796 * ((v1807 * v5251) + (v1791 * (((v1805 * (self.scalar_v1803 * v5251)) - (v1804 * v5260)) / v5264)))) } else { v1 }) });
        let v5320: f64 = (v1786 * v5250);
        let v5327: f64 = (if self.scalar_v1824 { (v560 * (v5250 + (if self.scalar_v1824 { ((v5320 + v5320) / (v552 * v1829)) } else { v1 }))) } else { v1 });
        let v5340: f64 = (if self.scalar_v1824 { (((v1836 * v5248) + (v1784 * ((-(((v642 * v5327) - (v1833 * v2463)) / v2484)) * (self.scalar_v1800 * f64::powf(v1835, self.scalar_v5291))))) / self.scalar_v1800) } else { v1 });
        let v5341: f64 = (if self.scalar_v1824 { v5250 } else { v1 });
        let v5342: f64 = (if self.scalar_v1824 { v2571 } else { v1 });
        let v5343: f64 = (if self.scalar_v1824 { v36 } else { v1 });
        let v5344: f64 = (v1840 * v5341);
        let v5346: f64 = (v1840 * v5342);
        let v5348: f64 = (v1840 * v5343);
        let v5350: f64 = (v552 * v1843);
        let v5364: f64 = (if self.scalar_v1824 { ((v134 * (v5341 - (if self.scalar_v1824 { ((v5344 + v5344) / v5350) } else { v1 }))) - v5250) } else { v1 });
        let v5365: f64 = (if self.scalar_v1824 { (v134 * (v5342 - (if self.scalar_v1824 { ((v5346 + v5346) / v5350) } else { v1 }))) } else { v1 });
        let v5366: f64 = (if self.scalar_v1824 { (v134 * (v5343 - (if self.scalar_v1824 { ((v5348 + v5348) / v5350) } else { v1 }))) } else { v1 });
        let v5377: f64 = (self.scalar_v1800 * f64::powf(v1850, self.scalar_v5291));
        let v5393: f64 = (v2571 - v5365);
        let v5394: f64 = (v36 - v5366);
        let v5395: f64 = (v5327 + (-v5364));
        let v5417: f64 = ((if self.scalar_v1824 { (((v1851 * v5248) + (v1784 * ((-(((v642 * v5364) - (v1848 * v2463)) / v2484)) * v5377))) / self.scalar_v1800) } else { v5308 }) + ((v1860 * (self.scalar_v1795 * v5395)) + (v1857 * (((v1805 * (self.scalar_v1803 * v5395)) - (v1858 * v5260)) / v5264))));
        let v5418: f64 = ((if self.scalar_v1824 { ((v1784 * ((-(v5365 / v642)) * v5377)) / self.scalar_v1800) } else { v5309 }) + ((v1860 * (self.scalar_v1795 * v5393)) + (v1857 * ((self.scalar_v1803 * v5393) / v1805))));
        let v5419: f64 = ((if self.scalar_v1824 { ((v1784 * ((-(v5366 / v642)) * v5377)) / self.scalar_v1800) } else { v5310 }) + ((v1860 * (self.scalar_v1795 * v5394)) + (v1857 * ((self.scalar_v1803 * v5394) / v1805))));
        let v5422: f64 = (if self.scalar_v1824 { v5418 } else { (if self.scalar_v1789 { (v5309 + (if v1812 { v1 } else { (if v1793 { (v1796 * ((v1807 * v5252) + (v1791 * ((self.scalar_v1803 * v5252) / v1805)))) } else { v1 }) })) } else { v1 }) });
        let v5423: f64 = (if self.scalar_v1824 { v5419 } else { (if self.scalar_v1789 { (v5310 + (if v1812 { v1 } else { (if v1793 { (v1796 * ((v1807 * v5253) + (v1791 * ((self.scalar_v1803 * v5253) / v1805)))) } else { v1 }) })) } else { v1 }) });
        let v5453: f64 = (self.scalar_v779 * f64::powf(v1886, self.scalar_v2614));
        let v5468: f64 = (if v1884 { (((v1888 * v2387) + (v588 * (-((-((-(v728 * v2387)) / v2466)) * v5453)))) / self.scalar_v779) } else { (if v1870 { ((v1873 * v2387) / self.scalar_v779) } else { v1 }) });
        let v5469: f64 = (if v1884 { ((v588 * (-(v2612 * v5453))) / self.scalar_v779) } else { v1 });
        let v5470: f64 = (if v1884 { ((v588 * (-(v2613 * v5453))) / self.scalar_v779) } else { v1 });
        let v5477: f64 = (if self.scalar_v767 { (v5468 + (if v1884 { v1 } else { (if v1870 { (v1871 * ((v1879 * v2574) + (v1868 * ((v2584 - (v1877 * v2583)) / v2587)))) } else { v1 }) })) } else { v1 });
        let v5480: f64 = (v1895 * v2664);
        let v5482: f64 = (v1895 * v2665);
        let v5484: f64 = (v1895 * v2666);
        let v5486: f64 = (v552 * v1898);
        let v5500: f64 = (if self.scalar_v802 { ((v134 * (v2664 - (if self.scalar_v802 { ((v5480 + v5480) / v5486) } else { v1 }))) - v2573) } else { v1 });
        let v5501: f64 = (if self.scalar_v802 { (v134 * (v2665 - (if self.scalar_v802 { ((v5482 + v5482) / v5486) } else { v1 }))) } else { v1 });
        let v5502: f64 = (if self.scalar_v802 { (v134 * (v2666 - (if self.scalar_v802 { ((v5484 + v5484) / v5486) } else { v1 }))) } else { v1 });
        let v5513: f64 = (self.scalar_v779 * f64::powf(v1905, self.scalar_v2614));
        let v5529: f64 = (v36 - v5501);
        let v5530: f64 = (v2571 - v5502);
        let v5531: f64 = (v2650 + (-v5500));
        let v5553: f64 = ((if self.scalar_v802 { (((v1906 * v2572) + (v763 * ((-(((v588 * v5500) - (v1903 * v2387)) / v2466)) * v5513))) / self.scalar_v779) } else { v5468 }) + ((v1915 * (self.scalar_v774 * v5531)) + (v1912 * (((v784 * (self.scalar_v782 * v5531)) - (v1913 * v2583)) / v2587))));
        let v5554: f64 = ((if self.scalar_v802 { ((v763 * ((-(v5501 / v588)) * v5513)) / self.scalar_v779) } else { v5469 }) + ((v1915 * (self.scalar_v774 * v5529)) + (v1912 * ((self.scalar_v782 * v5529) / v784))));
        let v5555: f64 = ((if self.scalar_v802 { ((v763 * ((-(v5502 / v588)) * v5513)) / self.scalar_v779) } else { v5470 }) + ((v1915 * (self.scalar_v774 * v5530)) + (v1912 * ((self.scalar_v782 * v5530) / v784))));
        let v5593: f64 = (if v1941 { (((v1947 * v2425) + (v615 * (-((v1945 * v2785) + (v878 * (-((-(v1943 * v2425)) / v2790))))))) / self.scalar_v858) } else { (if v1923 { ((v1927 * v2425) / self.scalar_v858) } else { v1 }) });
        let v5601: f64 = (self.scalar_v858 * f64::powf(v1954, self.scalar_v2782));
        let v5616: f64 = (if v1952 { (((v1956 * v2425) + (v615 * (-((-((-(v739 * v2425)) / v2475)) * v5601)))) / self.scalar_v858) } else { v5593 });
        let v5617: f64 = (if v1952 { ((v615 * (-(v2823 * v5601))) / self.scalar_v858) } else { (if v1941 { v2812 } else { v1 }) });
        let v5618: f64 = (if v1952 { ((v615 * (-(v2822 * v5601))) / self.scalar_v858) } else { (if v1941 { v2811 } else { v1 }) });
        let v5625: f64 = (if self.scalar_v846 { (v5616 + (if v1940 { v1 } else { (if v1923 { (v1924 * ((v1933 * v2749) + (v1921 * ((v2758 - (v1931 * v2425)) / v2475)))) } else { v1 }) })) } else { v1 });
        let v5631: f64 = (if self.scalar_v905 { ((v2853 - (v1965 * v2852)) / v2856) } else { v1 });
        let v5633: f64 = (v1969 * v5631);
        let v5635: f64 = (v1969 * v2904);
        let v5637: f64 = (v1969 * v2903);
        let v5639: f64 = (v552 * v1972);
        let v5643: f64 = (v1973 * v5631);
        let v5645: f64 = (v1973 * v2904);
        let v5647: f64 = (v1973 * v2903);
        let v5649: f64 = (v552 * v1976);
        let v5659: f64 = (v1977 * v1977);
        let v5669: f64 = (if self.scalar_v905 { (((v1977 * (v552 * v5631)) - (v1968 * (((v5633 + v5633) / v5639) + ((v5643 + v5643) / v5649)))) / v5659) } else { v1 });
        let v5670: f64 = (if self.scalar_v905 { (((v1977 * v2907) - (v1968 * (((v5635 + v5635) / v5639) + ((v5645 + v5645) / v5649)))) / v5659) } else { v1 });
        let v5671: f64 = (if self.scalar_v905 { (((v1977 * v2906) - (v1968 * (((v5637 + v5637) / v5639) + ((v5647 + v5647) / v5649)))) / v5659) } else { v1 });
        let v5681: f64 = (if self.scalar_v905 { (v134 * (((v1979 * v2852) + (v907 * v5669)) - v2748)) } else { v1 });
        let v5682: f64 = (if self.scalar_v905 { (v134 * (v907 * v5670)) } else { v1 });
        let v5683: f64 = (if self.scalar_v905 { (v134 * (v907 * v5671)) } else { v1 });
        let v5694: f64 = (self.scalar_v858 * f64::powf(v1986, self.scalar_v2782));
        let v5709: f64 = (if self.scalar_v905 { (((v1988 * v2425) + (v615 * (-((-(((v615 * v5681) - (v1984 * v2425)) / v2475)) * v5694)))) / self.scalar_v858) } else { v5616 });
        let v5710: f64 = (if self.scalar_v905 { ((v615 * (-((-(v5682 / v615)) * v5694))) / self.scalar_v858) } else { v5617 });
        let v5711: f64 = (if self.scalar_v905 { ((v615 * (-((-(v5683 / v615)) * v5694))) / self.scalar_v858) } else { v5618 });
        let v5715: f64 = (if self.scalar_v905 { (v134 * v5669) } else { v1 });
        let v5716: f64 = (if self.scalar_v905 { (v134 * v5670) } else { v1 });
        let v5717: f64 = (if self.scalar_v905 { (v134 * v5671) } else { v1 });
        let v5743: f64 = ((v2001 * (if self.scalar_v905 { (((v1995 * v2997) + (v972 * (-v5715))) + ((v1994 * v3005) + (v976 * v5715))) } else { v1 })) + (v1999 * (v2880 + (-v5681))));
        let v5758: f64 = (if self.scalar_v905 { (v5710 + (if self.scalar_v905 { ((v2001 * (if self.scalar_v905 { ((v972 * (-v5716)) + (v976 * v5716)) } else { v1 })) + (v1999 * (v36 - v5682))) } else { v1 })) } else { (if self.scalar_v846 { (v5617 + (if v1940 { v1 } else { (if v1923 { (v1924 * ((v1933 * v2751) + (v1921 * v2763))) } else { v1 }) })) } else { v1 }) });
        let v5759: f64 = (if self.scalar_v905 { (v5711 + (if self.scalar_v905 { ((v2001 * (if self.scalar_v905 { ((v972 * (-v5717)) + (v976 * v5717)) } else { v1 })) + (v1999 * (v2571 - v5683))) } else { v1 })) } else { (if self.scalar_v846 { (v5618 + (if v1940 { v1 } else { (if v1923 { (v1924 * ((v1933 * v2750) + (v1921 * v2762))) } else { v1 }) })) } else { v1 }) });
        let v5760: f64 = (v2007 * v3069);
        let v5762: f64 = (v2007 * v3071);
        let v5764: f64 = (v2007 * v3070);
        let v5766: f64 = (v552 * v2010);
        let v5780: f64 = (if self.scalar_v990 { ((v134 * (v3069 - (if self.scalar_v990 { ((v5760 + v5760) / v5766) } else { v1 }))) - v2748) } else { v5681 });
        let v5781: f64 = (if self.scalar_v990 { (v134 * (v3071 - (if self.scalar_v990 { ((v5762 + v5762) / v5766) } else { v1 }))) } else { v5682 });
        let v5782: f64 = (if self.scalar_v990 { (v134 * (v3070 - (if self.scalar_v990 { ((v5764 + v5764) / v5766) } else { v1 }))) } else { v5683 });
        let v5793: f64 = (self.scalar_v858 * f64::powf(v2017, self.scalar_v2782));
        let v5815: f64 = ((if self.scalar_v990 { (((v2018 * v2747) + (v843 * ((-(((v615 * v5780) - (v2015 * v2425)) / v2475)) * v5793))) / self.scalar_v858) } else { v5709 }) + (self.scalar_v1019 * (v3055 + (-v5780))));
        let v5825: f64 = (v64 * (v2029 * v3162));
        let v5826: f64 = (v64 * (v2029 * v3163));
        let v5827: f64 = (v64 * (v2029 * v3164));
        let v5831: f64 = (v2032 * v2032);
        let v5843: f64 = ((v59 * v2571) / v2035);
        let v5844: f64 = ((v36 * v59) / v2035);
        let v5847: f64 = (if v2037 { v1 } else { (if v1696 { ((v1701 * (v1698 * (v350 * v4866))) + (v1698 * (v1699 * v4866))) } else { (if v1691 { (v1693 * (v756 * v4866)) } else { v4333 }) }) });
        let v5878: f64 = (v2033 * (((v2032 * v5825) - (v2031 * v5825)) / v5831));
        let v5880: f64 = (v2033 * (((v2032 * v5826) - (v2031 * v5826)) / v5831));
        let v5882: f64 = (v2033 * (((v2032 * v5827) - (v2031 * v5827)) / v5831));
        let v5891: f64 = ((v2053 * (self.scalar_v2050 * (if v2040 { (v1513 * v5844) } else { (if v2037 { (v2038 * v5844) } else { (if v1696 { v1 } else { (if v1691 { v1 } else { v4336 }) }) }) }))) + (v2051 * (v5880 + v5880)));
        let v5901: f64 = (v2029 * ((v2053 * (self.scalar_v2050 * (if v2040 { v1 } else { (if v2037 { v1 } else { (if v1696 { v1 } else { (if v1691 { v1 } else { v4337 }) }) }) }))) + (v2051 * (v5882 + v5882))));
        let v5906: f64 = ((v2056 * (self.scalar_v2045 * (self.scalar_v2046 * v3231))) + (v2049 * (v2029 * ((v2053 * (self.scalar_v2050 * (if v2040 { v1 } else { v5847 }))) + (v2051 * (v5878 + v5878))))));
        let v5908: f64 = (v2049 * (v2029 * (v2053 * (self.scalar_v2050 * (if v2040 { (v1513 * v5843) } else { (if v2037 { (v2038 * v5843) } else { (if v1696 { v1 } else { (if v1691 { v1 } else { v4334 }) }) }) })))));
        let v5917: f64 = (v2049 * (v2029 * (v2053 * (self.scalar_v2050 * (if v2040 { v1 } else { (if v2037 { v1 } else { (if v1696 { (v1698 * v4868) } else { (if v1691 { (v1693 * v4868) } else { v4338 }) }) }) })))));
        let v5918: f64 = (v2049 * (v2029 * (v2053 * (self.scalar_v2050 * (if v2040 { v1 } else { (if v2037 { v1 } else { (if v1696 { (v1698 * v4869) } else { (if v1691 { (v1693 * v4869) } else { v4339 }) }) }) })))));
        let v5948: f64 = ((v1041 * (v2049 * (v2029 * (v2053 * (self.scalar_v2050 * (if v2040 { v1 } else { (if v2037 { v1 } else { (if v1696 { v1 } else { (if v1691 { v1 } else { v4335 }) }) }) })))))) / v1108);
        let v5951: f64 = ((v1108 * ((v2057 * v3163) + (v1041 * ((v2056 * (self.scalar_v2045 * (self.scalar_v2046 * v3233))) + (v2049 * (v2029 * v5891)))))) - (v2060 * (if v1105 { (self.scalar_v1106 * v3287) } else { v3317 })));
        let v5955: f64 = ((v1108 * ((v2057 * v3164) + (v1041 * ((v2056 * (self.scalar_v2045 * (self.scalar_v2046 * v3234))) + (v2049 * v5901))))) - (v2060 * (if v1105 { (self.scalar_v1106 * v3288) } else { v3318 })));
        let v5959: f64 = ((self.scalar_v1185 * ((v842 * v2472) + (v647 * v2744))) + (((v1108 * ((v2057 * v3162) + (v1041 * v5906))) - (v2060 * (if v1105 { (self.scalar_v1106 * v3285) } else { v3315 }))) / v3330));
        let v5965: f64 = (v647 * (if self.scalar_v802 { v5554 } else { (if self.scalar_v767 { (v5469 + (if v1884 { v1 } else { (if v1870 { (v1871 * ((v1879 * v2575) + (v1868 * v2589))) } else { v1 }) })) } else { v1 }) }));
        let v5966: f64 = (v647 * (if self.scalar_v802 { v5555 } else { (if self.scalar_v767 { (v5470 + (if v1884 { v1 } else { (if v1870 { (v1871 * ((v1879 * v2576) + (v1868 * v2590))) } else { v1 }) })) } else { v1 }) }));
        let v5989: f64 = ((((v1025 * (self.scalar_v648 * v2479)) + (v652 * v3131)) + (self.scalar_v2066 * v3198)) + (self.scalar_v2069 * (((v1517 * v2503) + (v665 * v4333)) / v4366)));
        let v6009: f64 = (((v2027 * (self.scalar_v653 * v2479)) + (v654 * (if self.scalar_v990 { (v5815 - v3068) } else { (if self.scalar_v905 { ((v5709 + (if self.scalar_v905 { v5743 } else { v1 })) - v2894) } else { v5625 }) }))) + (self.scalar_v2066 * (if self.scalar_v1181 { v1 } else { (if self.scalar_v1111 { ((v1145 * v2218) + (v479 * ((self.scalar_v1140 * v3382) + (self.scalar_v1142 * v3399)))) } else { v1 }) })));
        let v6010: f64 = ((v654 * (if self.scalar_v990 { ((if self.scalar_v990 { ((v843 * ((-(v5781 / v615)) * v5793)) / self.scalar_v858) } else { v5710 }) + (self.scalar_v1019 * (v36 - v5781))) } else { v5758 })) + (self.scalar_v2066 * (if self.scalar_v1181 { v1 } else { (if self.scalar_v1111 { (v479 * (self.scalar_v1140 * v3384)) } else { v1 }) })));
        let v6011: f64 = ((v654 * (if self.scalar_v990 { ((if self.scalar_v990 { ((v843 * ((-(v5782 / v615)) * v5793)) / self.scalar_v858) } else { v5711 }) + (self.scalar_v1019 * (v2571 - v5782))) } else { v5759 })) + (self.scalar_v2066 * (if self.scalar_v1181 { v1 } else { (if self.scalar_v1111 { (v479 * (self.scalar_v1140 * v3387)) } else { v1 }) })));
        let v6014: f64 = ((v1866 * (self.scalar_v655 * (((-(self.scalar_v616 * v2463)) / v2484) * (self.scalar_v657 * f64::powf(v656, self.scalar_v2486))))) + (v659 * (if self.scalar_v1865 { v1 } else { (if self.scalar_v1824 { (v5417 - v5340) } else { (if self.scalar_v1789 { (v5308 + v5311) } else { v1 }) }) })));
        let v6024: f64 = (v36 * v5959);
        let v6025: f64 = (v36 * (((v1108 * (v1041 * ((v2056 * (self.scalar_v2045 * (self.scalar_v2046 * v3232))) + v5908))) - (v2060 * (if v1105 { (self.scalar_v1106 * v3286) } else { v3316 }))) / v3330));
        let v6026: f64 = (v36 * v5948);
        let v6027: f64 = (v36 * ((self.scalar_v1185 * (v647 * v2745)) + (v5951 / v3330)));
        let v6028: f64 = (v36 * ((self.scalar_v1185 * (v647 * v2746)) + (v5955 / v3330)));
        let v6029: f64 = (v36 * ((v1041 * v5917) / v1108));
        let v6030: f64 = (v36 * ((v1041 * v5918) / v1108));
        let v6031: f64 = (v36 * (self.scalar_v1407 * ((v1919 * v2472) + (v647 * (if self.scalar_v802 { (v5553 - v2663) } else { v5477 })))));
        let v6032: f64 = (v36 * (self.scalar_v1407 * v5965));
        let v6033: f64 = (v36 * (self.scalar_v1407 * v5966));
        let v6034: f64 = (v36 * v5989);
        let v6035: f64 = (v36 * (((v652 * v3132) + (self.scalar_v2066 * v3199)) + (self.scalar_v2069 * ((v665 * v4334) / v4366))));
        let v6036: f64 = (v36 * (self.scalar_v2069 * ((v665 * v4335) / v4366)));
        let v6037: f64 = (v36 * (((v652 * v3133) + (self.scalar_v2066 * v3200)) + (self.scalar_v2069 * ((v665 * v4336) / v4366))));
        let v6038: f64 = (v36 * ((self.scalar_v2066 * v3201) + (self.scalar_v2069 * ((v665 * v4337) / v4366))));
        let v6039: f64 = (v36 * (self.scalar_v2069 * ((v665 * v4338) / v4366)));
        let v6040: f64 = (v36 * (self.scalar_v2069 * ((v665 * v4339) / v4366)));
        let v6041: f64 = (v36 * (self.scalar_v2069 * (((v1526 * v2503) + (v665 * (if v1522 { (v1513 * v4342) } else { (if v1519 { (v1520 * v4342) } else { v4128 }) }))) / v4381)));
        let v6042: f64 = (v36 * (self.scalar_v2069 * ((v665 * (if v1522 { v4331 } else { (if v1519 { (v1520 * v4318) } else { v1 }) })) / v4381)));
        let v6043: f64 = (v36 * (self.scalar_v2069 * ((v665 * (if v1522 { v1 } else { (if v1519 { v1 } else { v4129 }) })) / v4381)));
        let v6044: f64 = (v36 * (self.scalar_v2069 * (v4379 / v4381)));
        let v6045: f64 = (v36 * (self.scalar_v2069 * v4386));
        let v6046: f64 = (v36 * v6009);
        let v6047: f64 = (v36 * (self.scalar_v2066 * (if self.scalar_v1181 { v1 } else { (if self.scalar_v1111 { (v479 * ((self.scalar_v1140 * v3383) + (self.scalar_v1142 * v3400))) } else { v1 }) })));
        let v6048: f64 = (v36 * v6010);
        let v6049: f64 = (v36 * (self.scalar_v2066 * (if self.scalar_v1181 { v1 } else { (if self.scalar_v1111 { (v479 * ((self.scalar_v1140 * v3385) + (self.scalar_v1142 * v3401))) } else { v1 }) })));
        let v6050: f64 = (v36 * (self.scalar_v2066 * (if self.scalar_v1181 { v1 } else { (if self.scalar_v1111 { (v479 * (self.scalar_v1140 * v3386)) } else { v1 }) })));
        let v6051: f64 = (v36 * v6011);
        let v6052: f64 = (v36 * v6014);
        let v6053: f64 = (v36 * ((v659 * (if self.scalar_v1865 { v1 } else { v5422 })) + (self.scalar_v2077 * v2571)));
        let v6054: f64 = (v36 * ((v659 * (if self.scalar_v1865 { v1 } else { v5423 })) + (v36 * self.scalar_v2077)));

        let d2091_dn4: f64 = v6024;
        let d2091_dn6: f64 = v6025;
        let d2091_dn7: f64 = v6026;
        let d2091_dn8: f64 = v6027;
        let d2091_dn9: f64 = v6028;
        let d2091_dn10: f64 = v6029;
        let d2091_dn11: f64 = v6030;
        let v2091_reactive_nodes: [usize; 7] = [nodes[4], nodes[6], nodes[7], nodes[8], nodes[9], nodes[10], nodes[11]];
        let v2091_reactive_node_derivatives: [f64; 7] = [d2091_dn4, d2091_dn6, d2091_dn7, d2091_dn8, d2091_dn9, d2091_dn10, d2091_dn11];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[9]),
            &v2091_reactive_nodes,
            &v2091_reactive_node_derivatives,
            &[],
            &[],
            multiplicity,
        );
        let d2092_dn4: f64 = v6031;
        let d2092_dn7: f64 = v6032;
        let d2092_dn9: f64 = v6033;
        stamper.stamp_current_reactive_node3(
            Some(nodes[7]),
            Some(nodes[9]),
            nodes[4],
            multiplicity * (d2092_dn4),
            nodes[7],
            multiplicity * (d2092_dn7),
            nodes[9],
            multiplicity * (d2092_dn9),
        );
        let d2093_dn4: f64 = v6034;
        let d2093_dn6: f64 = v6035;
        let d2093_dn7: f64 = v6036;
        let d2093_dn8: f64 = v6037;
        let d2093_dn9: f64 = v6038;
        let d2093_dn10: f64 = v6039;
        let d2093_dn11: f64 = v6040;
        let v2093_reactive_nodes: [usize; 7] = [nodes[4], nodes[6], nodes[7], nodes[8], nodes[9], nodes[10], nodes[11]];
        let v2093_reactive_node_derivatives: [f64; 7] = [d2093_dn4, d2093_dn6, d2093_dn7, d2093_dn8, d2093_dn9, d2093_dn10, d2093_dn11];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[6]),
            &v2093_reactive_nodes,
            &v2093_reactive_node_derivatives,
            &[],
            &[],
            multiplicity,
        );
        let d2094_dn4: f64 = v6041;
        let d2094_dn5: f64 = v6042;
        let d2094_dn6: f64 = v6043;
        let d2094_dn8: f64 = v6044;
        let d2094_dn9: f64 = v6045;
        let v2094_reactive_nodes: [usize; 5] = [nodes[4], nodes[5], nodes[6], nodes[8], nodes[9]];
        let v2094_reactive_node_derivatives: [f64; 5] = [d2094_dn4, d2094_dn5, d2094_dn6, d2094_dn8, d2094_dn9];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[5]),
            &v2094_reactive_nodes,
            &v2094_reactive_node_derivatives,
            &[],
            &[],
            multiplicity,
        );
        let d2095_dn4: f64 = v6046;
        let d2095_dn6: f64 = v6047;
        let d2095_dn7: f64 = v6048;
        let d2095_dn8: f64 = v6049;
        let d2095_dn9: f64 = v6050;
        let d2095_dn10: f64 = v6051;
        let v2095_reactive_nodes: [usize; 6] = [nodes[4], nodes[6], nodes[7], nodes[8], nodes[9], nodes[10]];
        let v2095_reactive_node_derivatives: [f64; 6] = [d2095_dn4, d2095_dn6, d2095_dn7, d2095_dn8, d2095_dn9, d2095_dn10];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[10]),
            &v2095_reactive_nodes,
            &v2095_reactive_node_derivatives,
            &[],
            &[],
            multiplicity,
        );
        let d2081_dn1: f64 = self.scalar_v2080;
        let d2081_dn2: f64 = self.scalar_v6021;
        stamper.stamp_current_reactive_node2(
            Some(nodes[1]),
            Some(nodes[2]),
            nodes[1],
            multiplicity * (d2081_dn1),
            nodes[2],
            multiplicity * (d2081_dn2),
        );
        let d2083_dn0: f64 = self.scalar_v6022;
        let d2083_dn1: f64 = self.scalar_v2082;
        stamper.stamp_current_reactive_node2(
            Some(nodes[1]),
            Some(nodes[0]),
            nodes[0],
            multiplicity * (d2083_dn0),
            nodes[1],
            multiplicity * (d2083_dn1),
        );
        let d2096_dn4: f64 = v6052;
        let d2096_dn10: f64 = v6053;
        let d2096_dn11: f64 = v6054;
        stamper.stamp_current_reactive_node3(
            Some(nodes[11]),
            Some(nodes[10]),
            nodes[4],
            multiplicity * (d2096_dn4),
            nodes[10],
            multiplicity * (d2096_dn10),
            nodes[11],
            multiplicity * (d2096_dn11),
        );
        let d2087_dn12: f64 = self.scalar_v2086;
        stamper.stamp_current_reactive_node1(
            Some(nodes[12]),
            None,
            nodes[12],
            multiplicity * (d2087_dn12),
        );
        let d2090_dn13: f64 = self.scalar_v6023;
        stamper.stamp_current_reactive_node1(
            Some(nodes[13]),
            None,
            nodes[13],
            multiplicity * (d2090_dn13),
        );
        let d2085_dn4: f64 = self.scalar_v2084;
        stamper.stamp_current_reactive_node1(
            Some(nodes[4]),
            None,
            nodes[4],
            multiplicity * (d2085_dn4),
        );
    }
}
