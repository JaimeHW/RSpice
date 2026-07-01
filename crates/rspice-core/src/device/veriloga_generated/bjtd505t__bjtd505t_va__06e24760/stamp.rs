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
        let v1: f64 = 1.0;
        let v4: f64 = 0.0;
        let v30: f64 = 0.001;
        let v31: f64 = 2.0;
        let v44: f64 = 0.05;
        let v46: f64 = 0.1;
        let v101: f64 = ctx.node_voltage(nodes[3]);
        let v102: bool = (v101 < v4);
        let v103: f64 = (v1 - v101);
        let v106: f64 = (if v102 { (-((v103) as f64).ln()) } else { v101 });
        let v108: bool = (v106 < self.scalar_v107);
        let v110: bool = (!v108);
        let v112: f64 = (v1 + (v106 - self.scalar_v107));
        let v116: f64 = (self.scalar_v20 + (if v110 { (self.scalar_v107 + ((v112) as f64).ln()) } else { (if v108 { v106 } else { v4 }) }));
        let v117: f64 = (v116 / self.scalar_v17);
        let v118: f64 = 8.617086918058125e-5;
        let v119: f64 = (v116 * v118);
        let v121: f64 = (v1 / v119);
        let v123: f64 = (v121 - self.scalar_v122);
        let v124: f64 = (v116 - self.scalar_v17);
        let v125: f64 = ((v117) as f64).ln();
        let v126: f64 = (self.scalar_v37 * v116);
        let v127: f64 = (v116 * v126);
        let v128: f64 = (self.scalar_v40 + v116);
        let v130: f64 = (self.scalar_v62 - (v127 / v128));
        let v132: f64 = ((v130 - v44) / v46);
        let v133: bool = (v130 < v44);
        let v134: f64 = ((v132) as f64).exp();
        let v135: f64 = (v1 + v134);
        let v140: bool = (!v133);
        let v142: f64 = (((-v132)) as f64).exp();
        let v143: f64 = (v1 + v142);
        let v147: f64 = (if v140 { (v130 + (v46 * ((v143) as f64).ln())) } else { (if v133 { (v44 + (v46 * ((v135) as f64).ln())) } else { v4 }) });
        let v148: f64 = (self.scalar_v72 * v116);
        let v149: f64 = (v116 * v148);
        let v150: f64 = (self.scalar_v75 + v116);
        let v152: f64 = (self.scalar_v95 - (v149 / v150));
        let v154: f64 = ((v152 - v44) / v46);
        let v155: bool = (v152 < v44);
        let v156: f64 = ((v154) as f64).exp();
        let v157: f64 = (v1 + v156);
        let v162: bool = (!v155);
        let v164: f64 = (((-v154)) as f64).exp();
        let v165: f64 = (v1 + v164);
        let v169: f64 = (if v162 { (v152 + (v46 * ((v165) as f64).ln())) } else { (if v155 { (v44 + (v46 * ((v157) as f64).ln())) } else { v4 }) });
        let v170: f64 = 3.0;
        let v171: f64 = -3.0;
        let v172: f64 = (v119 * v171);
        let v173: f64 = (v125 * v172);
        let v176: f64 = (v1 - v117);
        let v179: f64 = ((v173 + (self.scalar_v64 * v117)) + (v176 * self.scalar_v177));
        let v180: f64 = (v44 - v179);
        let v181: f64 = (v180 / v119);
        let v182: bool = (v44 < v179);
        let v183: f64 = ((v181) as f64).exp();
        let v184: f64 = (v1 + v183);
        let v185: f64 = ((v184) as f64).ln();
        let v189: bool = (!v182);
        let v191: f64 = (((-v181)) as f64).exp();
        let v192: f64 = (v1 + v191);
        let v193: f64 = ((v192) as f64).ln();
        let v196: f64 = (if v189 { (v44 + (v119 * v193)) } else { (if v182 { (v179 + (v119 * v185)) } else { v4 }) });
        let v201: f64 = (v176 * self.scalar_v200);
        let v202: f64 = ((v173 + (v117 * self.scalar_v197)) + v201);
        let v203: f64 = (v44 - v202);
        let v204: f64 = (v203 / v119);
        let v205: bool = (v44 < v202);
        let v206: f64 = ((v204) as f64).exp();
        let v207: f64 = (v1 + v206);
        let v208: f64 = ((v207) as f64).ln();
        let v212: bool = (!v205);
        let v214: f64 = (((-v204)) as f64).exp();
        let v215: f64 = (v1 + v214);
        let v216: f64 = ((v215) as f64).ln();
        let v219: f64 = (if v212 { (v44 + (v119 * v216)) } else { (if v205 { (v202 + (v119 * v208)) } else { v4 }) });
        let v223: f64 = (v201 + (v173 + (v117 * self.scalar_v220)));
        let v224: f64 = (v44 - v223);
        let v225: f64 = (v224 / v119);
        let v226: bool = (v44 < v223);
        let v227: f64 = ((v225) as f64).exp();
        let v228: f64 = (v1 + v227);
        let v229: f64 = ((v228) as f64).ln();
        let v233: bool = (!v226);
        let v235: f64 = (((-v225)) as f64).exp();
        let v236: f64 = (v1 + v235);
        let v237: f64 = ((v236) as f64).ln();
        let v240: f64 = (if v233 { (v44 + (v119 * v237)) } else { (if v226 { (v223 + (v119 * v229)) } else { v4 }) });
        let v243: f64 = (v201 + (v173 + (self.scalar_v66 * v117)));
        let v244: f64 = (v44 - v243);
        let v245: f64 = (v244 / v119);
        let v246: bool = (v44 < v243);
        let v247: f64 = ((v245) as f64).exp();
        let v248: f64 = (v1 + v247);
        let v249: f64 = ((v248) as f64).ln();
        let v253: bool = (!v246);
        let v255: f64 = (((-v245)) as f64).exp();
        let v256: f64 = (v1 + v255);
        let v257: f64 = ((v256) as f64).ln();
        let v260: f64 = (if v253 { (v44 + (v119 * v257)) } else { (if v246 { (v243 + (v119 * v249)) } else { v4 }) });
        let v266: f64 = ((v173 + (v117 * self.scalar_v261)) + (v176 * self.scalar_v264));
        let v267: f64 = (v44 - v266);
        let v268: f64 = (v267 / v119);
        let v269: bool = (v44 < v266);
        let v270: f64 = ((v268) as f64).exp();
        let v271: f64 = (v1 + v270);
        let v272: f64 = ((v271) as f64).ln();
        let v276: bool = (!v269);
        let v278: f64 = (((-v268)) as f64).exp();
        let v279: f64 = (v1 + v278);
        let v280: f64 = ((v279) as f64).ln();
        let v283: f64 = (if v276 { (v44 + (v119 * v280)) } else { (if v269 { (v266 + (v119 * v272)) } else { v4 }) });
        let v284: f64 = (v1 / v196);
        let v285: f64 = (v1 / v260);
        let v286: f64 = (self.scalar_v64 * v284);
        let v287: f64 = f64::powf(v286, self.scalar_v32);
        let v288: f64 = (self.scalar_v66 * v285);
        let v289: f64 = f64::powf(v288, self.scalar_v67);
        let v291: f64 = (v287 * self.scalar_v290);
        let v294: f64 = (self.scalar_v66 / v260);
        let v297: f64 = (self.scalar_v292 + (self.scalar_v293 * f64::powf(v294, self.scalar_v67)));
        let v298: f64 = (v1 / v297);
        let v300: f64 = (v297 * self.scalar_v299);
        let v301: f64 = (self.scalar_v292 * v298);
        let v305: f64 = (((v125 * self.scalar_v303)) as f64).exp();
        let v306: f64 = (self.scalar_v302 * v305);
        let v307: bool = (v306 < self.scalar_v28);
        let v308: f64 = (if v307 { self.scalar_v28 } else { v306 });
        let v314: f64 = (((v125 * self.scalar_v312)) as f64).exp();
        let v315: f64 = (self.scalar_v309 * v314);
        let v319: f64 = (((v125 * self.scalar_v317)) as f64).exp();
        let v320: f64 = (self.scalar_v316 * v319);
        let v321: bool = (v320 < self.scalar_v28);
        let v322: f64 = (if v321 { self.scalar_v28 } else { v320 });
        let v326: f64 = (((v125 * self.scalar_v324)) as f64).exp();
        let v327: f64 = (self.scalar_v323 * v326);
        let v331: f64 = (((v125 * self.scalar_v329)) as f64).exp();
        let v332: f64 = (self.scalar_v328 * v331);
        let v334: f64 = (v331 * self.scalar_v333);
        let v338: f64 = (((v125 * self.scalar_v336)) as f64).exp();
        let v339: f64 = (self.scalar_v335 * v338);
        let v346: f64 = (if self.scalar_v341 { (self.scalar_v342 * (v1 + (v124 * self.scalar_v340))) } else { v4 });
        let v349: f64 = (if self.scalar_v341 { ((v346 - v1) / v30) } else { v268 });
        let v350: bool = (v346 < v1);
        let v351: bool = (self.scalar_v341 && v350);
        let v352: f64 = ((v349) as f64).exp();
        let v353: f64 = (v1 + v352);
        let v357: f64 = (if v351 { (v1 + (v30 * ((v353) as f64).ln())) } else { v346 });
        let v359: bool = (self.scalar_v341 && (!v350));
        let v361: f64 = (((-v349)) as f64).exp();
        let v362: f64 = (v1 + v361);
        let v367: f64 = 0.0006931471805599453;
        let v371: f64 = (if self.scalar_v370 { self.scalar_v342 } else { (if self.scalar_v341 { ((if v359 { (v357 + (v30 * ((v362) as f64).ln())) } else { v357 }) - v367) } else { v4 }) });
        let v378: f64 = (if self.scalar_v373 { (self.scalar_v374 * (v1 + (v124 * self.scalar_v372))) } else { v4 });
        let v381: f64 = (if self.scalar_v373 { ((v378 - v1) / v30) } else { v349 });
        let v382: bool = (v378 < v1);
        let v383: bool = (self.scalar_v373 && v382);
        let v384: f64 = ((v381) as f64).exp();
        let v385: f64 = (v1 + v384);
        let v389: f64 = (if v383 { (v1 + (v30 * ((v385) as f64).ln())) } else { v378 });
        let v391: bool = (self.scalar_v373 && (!v382));
        let v393: f64 = (((-v381)) as f64).exp();
        let v394: f64 = (v1 + v393);
        let v402: f64 = (if self.scalar_v401 { self.scalar_v374 } else { (if self.scalar_v373 { ((if v391 { (v389 + (v30 * ((v394) as f64).ln())) } else { v389 }) - v367) } else { v4 }) });
        let v407: f64 = (self.scalar_v403 * (v1 + (v124 * self.scalar_v404)));
        let v408: f64 = 1e-6;
        let v409: f64 = (v407 * v407);
        let v410: bool = (v407 < v4);
        let v411: f64 = 0.5;
        let v412: f64 = 5e-7;
        let v414: f64 = (((v408 + v409)) as f64).sqrt();
        let v415: f64 = (v414 - v407);
        let v418: bool = (!v410);
        let v421: f64 = (if v418 { (v411 * (v407 + v414)) } else { (if v410 { (v412 / v415) } else { v4 }) });
        let v423: f64 = 4.0;
        let v428: f64 = (v125 * self.scalar_v427);
        let v430: f64 = (((v428 / v371)) as f64).exp();
        let v431: f64 = (self.scalar_v422 * v430);
        let v433: f64 = (v123 * self.scalar_v432);
        let v435: f64 = (((v433 / v371)) as f64).exp();
        let v436: f64 = (v431 * v435);
        let v440: f64 = (((v125 * self.scalar_v438)) as f64).exp();
        let v441: f64 = (self.scalar_v437 * v440);
        let v446: f64 = (((v125 * self.scalar_v444)) as f64).exp();
        let v447: f64 = (self.scalar_v442 * v446);
        let v449: f64 = 6.0;
        let v454: f64 = (((v125 * self.scalar_v452)) as f64).exp();
        let v455: f64 = (self.scalar_v448 * v454);
        let v458: f64 = (v123 * self.scalar_v457);
        let v460: f64 = (((v458 / self.scalar_v450)) as f64).exp();
        let v461: f64 = (v455 * v460);
        let v467: f64 = (((v125 * self.scalar_v465)) as f64).exp();
        let v468: f64 = (self.scalar_v462 * v467);
        let v472: f64 = ((((v123 * self.scalar_v469) / self.scalar_v463)) as f64).exp();
        let v473: f64 = (v468 * v472);
        let v477: f64 = (v125 * self.scalar_v476);
        let v480: f64 = (((v477 / self.scalar_v478)) as f64).exp();
        let v481: f64 = (self.scalar_v474 * v480);
        let v484: f64 = (v123 * self.scalar_v483);
        let v486: f64 = (((v484 / self.scalar_v478)) as f64).exp();
        let v487: f64 = (v481 * v486);
        let v491: f64 = (((v477 / self.scalar_v489)) as f64).exp();
        let v492: f64 = (self.scalar_v488 * v491);
        let v494: f64 = (((v484 / self.scalar_v489)) as f64).exp();
        let v495: f64 = (v492 * v494);
        let v503: f64 = ((((v123 * self.scalar_v500) / self.scalar_v478)) as f64).exp();
        let v510: f64 = (((v123 * self.scalar_v508)) as f64).exp();
        let v512: f64 = (if self.scalar_v497 { (self.scalar_v506 * v510) } else { v4 });
        let v518: f64 = ((((v123 * self.scalar_v515) / self.scalar_v489)) as f64).exp();
        let v525: f64 = (((v125 * self.scalar_v523)) as f64).exp();
        let v526: f64 = (self.scalar_v521 * v525);
        let v530: f64 = (((v123 * self.scalar_v528)) as f64).exp();
        let v531: f64 = (v526 * v530);
        let v537: f64 = (((v125 * self.scalar_v535)) as f64).exp();
        let v538: f64 = (self.scalar_v532 * v537);
        let v540: f64 = (((v458 / self.scalar_v533)) as f64).exp();
        let v541: f64 = (v538 * v540);
        let v546: f64 = (((v125 * self.scalar_v544)) as f64).exp();
        let v547: f64 = (self.scalar_v542 * v546);
        let v549: f64 = (((v458 / self.scalar_v543)) as f64).exp();
        let v550: f64 = (v547 * v549);
        let v552: f64 = ((v117) as f64).sqrt();
        let v553: f64 = (self.scalar_v551 * v552);
        let v556: f64 = (((v124 * self.scalar_v554)) as f64).exp();
        let v557: f64 = (v553 * v556);
        let v558: f64 = (self.scalar_v63 * v147);
        let v559: f64 = -0.5;
        let v560: f64 = f64::powf(v558, v559);
        let v561: f64 = (v1 / v287);
        let v563: f64 = (v147 * self.scalar_v562);
        let v564: f64 = (v147 * v563);
        let v565: f64 = (v560 * v564);
        let v567: f64 = (self.scalar_v64 * (v561 * v565));
        let v570: f64 = (self.scalar_v63 * (self.scalar_v63 * (v284 * v567)));
        let v572: f64 = (v560 * self.scalar_v571);
        let v573: f64 = (v196 * v572);
        let v576: f64 = (self.scalar_v65 * (self.scalar_v65 * (v196 * v573)));
        let v577: f64 = (v287 * v576);
        let v579: f64 = (((self.scalar_v562 - v570)) as f64).exp();
        let v581: f64 = (self.scalar_v96 * v169);
        let v582: f64 = f64::powf(v581, v559);
        let v583: f64 = (v1 / v289);
        let v585: f64 = (v169 * self.scalar_v584);
        let v586: f64 = (v169 * v585);
        let v587: f64 = (v582 * v586);
        let v589: f64 = (self.scalar_v66 * (v583 * v587));
        let v592: f64 = (self.scalar_v96 * (self.scalar_v96 * (v285 * v589)));
        let v594: f64 = (v582 * self.scalar_v593);
        let v595: f64 = (v260 * v594);
        let v598: f64 = (self.scalar_v97 * (self.scalar_v97 * (v260 * v595)));
        let v599: f64 = (v289 * v598);
        let v601: f64 = (((self.scalar_v584 - v592)) as f64).exp();
        let v604: f64 = (((v125 * self.scalar_v311)) as f64).exp();
        let v606: f64 = (v604 * self.scalar_v605);
        let v607: f64 = (v298 * v606);
        let v609: f64 = (v604 * self.scalar_v608);
        let v610: f64 = (v561 * v609);
        let v614: f64 = (((v125 * self.scalar_v612)) as f64).exp();
        let v615: f64 = (self.scalar_v611 * v614);
        let v619: f64 = (((v123 * self.scalar_v617)) as f64).exp();
        let v620: f64 = (v615 * v619);
        let v625: f64 = (((v125 * self.scalar_v623)) as f64).exp();
        let v626: f64 = (self.scalar_v621 * v625);
        let v630: f64 = (((v125 * self.scalar_v628)) as f64).exp();
        let v631: f64 = (self.scalar_v627 * v630);
        let v633: f64 = (v626 + v631);
        let v636: f64 = ((self.scalar_v632 * v633) / self.scalar_v635);
        let v641: f64 = (((v125 * self.scalar_v639)) as f64).exp();
        let v642: f64 = (self.scalar_v637 * v641);
        let v644: f64 = (v116 - 300.0);
        let v646: bool = (v116 < 525.0);
        let v647: f64 = 0.00072;
        let v650: f64 = 1.6e-6;
        let v651: f64 = (v644 * v650);
        let v656: bool = (!v646);
        let v659: f64 = (if v656 { self.scalar_v658 } else { (if v646 { (self.scalar_v12 * ((v1 + (v644 * v647)) - (v644 * v651))) } else { v4 }) });
        let v661: f64 = (v604 * self.scalar_v660);
        let v669: f64 = (if self.scalar_v667 { (v1 / v327) } else { v4 });
        let v671: bool = (self.scalar_v667 && (v669 > self.scalar_v29));
        let v674: f64 = (if self.scalar_v673 { v4 } else { (if v671 { self.scalar_v29 } else { v669 }) });
        let v677: f64 = (if self.scalar_v675 { (v1 / v332) } else { v4 });
        let v679: bool = (self.scalar_v675 && (v677 > self.scalar_v29));
        let v682: f64 = (if self.scalar_v681 { v4 } else { (if v679 { self.scalar_v29 } else { v677 }) });
        let v685: f64 = (if self.scalar_v683 { (v1 / v334) } else { v4 });
        let v687: bool = (self.scalar_v683 && (v685 > self.scalar_v29));
        let v690: f64 = (if self.scalar_v689 { v4 } else { (if v687 { self.scalar_v29 } else { v685 }) });
        let v691: f64 = ctx.node_voltage(nodes[6]);
        let v692: f64 = ctx.node_voltage(nodes[7]);
        let v694: f64 = (self.scalar_v0 * (v691 - v692));
        let v695: f64 = ctx.node_voltage(nodes[8]);
        let v697: f64 = (self.scalar_v0 * (v691 - v695));
        let v698: f64 = ctx.node_voltage(nodes[4]);
        let v700: f64 = (self.scalar_v0 * (v691 - v698));
        let v701: f64 = ctx.node_voltage(nodes[5]);
        let v703: f64 = (self.scalar_v0 * (v701 - v698));
        let v705: f64 = (self.scalar_v0 * (v701 - v691));
        let v707: f64 = (self.scalar_v0 * (v692 - v695));
        let v708: f64 = ctx.node_voltage(nodes[2]);
        let v710: f64 = (self.scalar_v0 * (v708 - v698));
        let v711: f64 = ctx.node_voltage(nodes[1]);
        let v713: f64 = (self.scalar_v0 * (v711 - v701));
        let v718: f64 = (self.scalar_v0 * (v711 - ctx.node_voltage(nodes[0])));
        let v719: f64 = ctx.node_voltage(nodes[10]);
        let v721: f64 = (self.scalar_v0 * (v719 - v692));
        let v724: f64 = (self.scalar_v0 * (ctx.node_voltage(nodes[9]) - v719));
        let v727: f64 = (((v697 + v705) - v707) - v721);
        let v731: f64 = ((v727 + (v713 + (-v718))) - v724);
        let v732: f64 = (v718 + v731);
        let v733: f64 = (v121 * v697);
        let v735: bool = (v733 < self.scalar_v734);
        let v736: f64 = ((v733) as f64).exp();
        let v738: bool = (!v735);
        let v740: f64 = (if v738 { self.scalar_v739 } else { v4 });
        let v745: f64 = (v121 * v700);
        let v746: f64 = (v745 / v371);
        let v747: bool = (v746 < self.scalar_v734);
        let v748: f64 = ((v746) as f64).exp();
        let v750: bool = (!v747);
        let v751: f64 = (if v750 { self.scalar_v739 } else { v740 });
        let v755: f64 = (if v750 { (v751 * (v1 + (v746 - self.scalar_v734))) } else { (if v747 { v748 } else { v4 }) });
        let v756: f64 = (v121 * v727);
        let v757: bool = (v756 < self.scalar_v734);
        let v758: f64 = ((v756) as f64).exp();
        let v760: bool = (!v757);
        let v761: f64 = (if v760 { self.scalar_v739 } else { v751 });
        let v765: f64 = (if v760 { (v761 * (v1 + (v756 - self.scalar_v734))) } else { (if v757 { v758 } else { v4 }) });
        let v766: f64 = (v121 * v705);
        let v767: bool = (v766 < self.scalar_v734);
        let v768: f64 = ((v766) as f64).exp();
        let v770: bool = (!v767);
        let v771: f64 = (if v770 { self.scalar_v739 } else { v761 });
        let v776: f64 = (v121 * v732);
        let v777: bool = (v776 < self.scalar_v734);
        let v778: f64 = ((v776) as f64).exp();
        let v780: bool = (!v777);
        let v781: f64 = (if v780 { self.scalar_v739 } else { v771 });
        let v785: f64 = (if v780 { (v781 * (v1 + (v776 - self.scalar_v734))) } else { (if v777 { v778 } else { v4 }) });
        let v786: f64 = (v732 - v219);
        let v787: f64 = (v121 * v786);
        let v788: bool = (v787 < self.scalar_v734);
        let v789: f64 = ((v787) as f64).exp();
        let v791: bool = (!v788);
        let v792: f64 = (if v791 { self.scalar_v739 } else { v781 });
        let v797: f64 = (v727 - v219);
        let v798: f64 = (v121 * v797);
        let v799: bool = (v798 < self.scalar_v734);
        let v800: f64 = ((v798) as f64).exp();
        let v802: bool = (!v799);
        let v803: f64 = (if v802 { self.scalar_v739 } else { v792 });
        let v808: f64 = (v697 - v219);
        let v809: f64 = (v121 * v808);
        let v810: bool = (v809 < self.scalar_v734);
        let v811: f64 = ((v809) as f64).exp();
        let v813: bool = (!v810);
        let v814: f64 = (if v813 { self.scalar_v739 } else { v803 });
        let v818: f64 = (if v813 { (v814 * (v1 + (v809 - self.scalar_v734))) } else { (if v810 { v811 } else { v4 }) });
        let v819: f64 = (v694 - v219);
        let v820: f64 = (v121 * v819);
        let v821: bool = (v820 < self.scalar_v734);
        let v822: f64 = ((v820) as f64).exp();
        let v824: bool = (!v821);
        let v825: f64 = (if v824 { self.scalar_v739 } else { v814 });
        let v829: f64 = (if v824 { (v825 * (v1 + (v820 - self.scalar_v734))) } else { (if v821 { v822 } else { v4 }) });
        let v832: f64 = (((v1 + (v423 * v818))) as f64).sqrt();
        let v835: f64 = (((v1 + (v423 * v829))) as f64).sqrt();
        let v836: f64 = (v31 * v829);
        let v837: f64 = (v1 + v835);
        let v838: f64 = (v836 / v837);
        let v840: bool = (v838 < self.scalar_v839);
        let v841: f64 = (if v840 { self.scalar_v839 } else { v838 });
        let v843: f64 = (v1 + v832);
        let v844: f64 = (v843 / v837);
        let v846: f64 = ((v832 - v835) - ((v844) as f64).ln());
        let v847: f64 = (v119 * v846);
        let v848: f64 = (v707 + v847);
        let v849: f64 = (v848 / v339);
        let v850: bool = (v849 > v4);
        let v851: f64 = 100.0;
        let v852: bool = (v694 < v851);
        let v853: bool = (v850 && v852);
        let v856: bool = (v850 && (!v852));
        let v858: f64 = (v1 + (v694 - v851));
        let v862: f64 = (v31 * v119);
        let v863: f64 = (v411 * v849);
        let v864: f64 = (v339 * v863);
        let v866: f64 = (v1 + (v121 * v864));
        let v867: f64 = ((v866) as f64).ln();
        let v871: f64 = (if v850 { ((v219 + (v862 * v867)) - (if v856 { (v851 + ((v858) as f64).ln()) } else { (if v853 { v694 } else { v4 }) })) } else { v4 });
        let v872: f64 = 0.2;
        let v874: f64 = (if v850 { (v219 * v872) } else { v4 });
        let v876: f64 = (if v850 { (v874 * v874) } else { v408 });
        let v879: bool = (v871 < v4);
        let v880: bool = (v850 && v879);
        let v881: f64 = (v411 * v876);
        let v883: f64 = (((v876 + (if v850 { (v871 * v871) } else { v409 }))) as f64).sqrt();
        let v884: f64 = (v883 - v871);
        let v888: bool = (v850 && (!v879));
        let v891: f64 = (if v888 { (v411 * (v871 + v883)) } else { (if v880 { (v881 / v884) } else { v4 }) });
        let v895: f64 = (v891 + self.scalar_v894);
        let v896: f64 = (v891 * v895);
        let v899: f64 = (self.scalar_v893 * (v891 + (v339 * self.scalar_v892)));
        let v901: f64 = (if v850 { (v896 / v899) } else { v4 });
        let v903: f64 = (if v850 { (v849 / v901) } else { v4 });
        let v907: f64 = (if v850 { ((v903 - v1) / self.scalar_v905) } else { v381 });
        let v908: bool = (v903 < v1);
        let v909: bool = (v850 && v908);
        let v910: f64 = ((v907) as f64).exp();
        let v911: f64 = (v1 + v910);
        let v917: bool = (v850 && (!v908));
        let v919: f64 = (((-v907)) as f64).exp();
        let v920: f64 = (v1 + v919);
        let v933: f64 = (if v850 { ((if v917 { (v903 + (self.scalar_v905 * ((v920) as f64).ln())) } else { (if v909 { (v1 + (self.scalar_v905 * ((v911) as f64).ln())) } else { v4 }) }) / self.scalar_v931) } else { v4 });
        let v935: f64 = (if v850 { (v891 / self.scalar_v894) } else { v4 });
        let v936: f64 = (v423 * v933);
        let v937: f64 = (v935 * v936);
        let v938: f64 = (v1 + v935);
        let v941: f64 = (((v1 + (v937 * v938))) as f64).sqrt();
        let v942: f64 = (v1 + v941);
        let v943: f64 = (v31 * v933);
        let v944: f64 = (v938 * v943);
        let v946: f64 = (if v850 { (v942 / v944) } else { v4 });
        let v948: f64 = (v841 * v946);
        let v949: f64 = ((v1 - v946) + v948);
        let v950: f64 = (v1 + v948);
        let v952: f64 = (if v850 { (v949 / v950) } else { v4 });
        let v953: f64 = (v864 * v952);
        let v955: f64 = (if v850 { (v121 * v953) } else { v4 });
        let v958: f64 = (v1 + (v841 + v955));
        let v961: f64 = (if v850 { ((v31 * v955) + (v841 * v958)) } else { v4 });
        let v964: f64 = (if v850 { (v411 * (v955 - v1)) } else { v4 });
        let v967: f64 = (if v850 { (v961 + (v964 * v964)) } else { v4 });
        let v968: bool = (v955 >= v1);
        let v969: bool = (v850 && v968);
        let v970: f64 = ((v967) as f64).sqrt();
        let v974: bool = (v850 && (!v968));
        let v975: f64 = (v970 - v964);
        let v977: f64 = (if v974 { (v961 / v975) } else { (if v969 { (v964 + v970) } else { v4 }) });
        let v980: bool = (v850 && (v977 < self.scalar_v978));
        let v981: f64 = (if v980 { self.scalar_v978 } else { v977 });
        let v982: f64 = (v1 + v981);
        let v983: f64 = (v981 * v982);
        let v985: f64 = (((v121 * v219)) as f64).exp();
        let v991: f64 = (if v850 { (self.scalar_v988 * (v849 - self.scalar_v892)) } else { v4 });
        let v993: f64 = (self.scalar_v892 * (v339 * self.scalar_v893));
        let v998: f64 = ((((if v850 { (v849 * v993) } else { v4 }) + (v991 * v991))) as f64).sqrt();
        let v1003: bool = (v850 && self.scalar_v1002);
        let v1004: f64 = (v46 * v260);
        let v1007: bool = (v850 && self.scalar_v1006);
        let v1008: f64 = (v31 * v849);
        let v1009: f64 = (v849 + v901);
        let v1011: f64 = (v46 + (v1008 / v1009));
        let v1014: f64 = (v849 * self.scalar_v892);
        let v1015: f64 = (v849 + self.scalar_v892);
        let v1020: bool = (!v850);
        let v1021: f64 = (v31 * v818);
        let v1024: f64 = (if v1020 { (if v738 { (v740 * (v1 + (v733 - self.scalar_v734))) } else { (if v735 { v736 } else { v4 }) }) } else { (if v850 { (v983 * v985) } else { v4 }) });
        let v1035: bool = ((((v707) as f64).abs() < (v119 * 1e-5)) || (((v847) as f64).abs() < ((v119 * 1e-40) * (v832 + v835))));
        let v1036: bool = (v1020 && v1035);
        let v1037: f64 = (v841 + (if v1020 { (v1021 / v843) } else { v981 }));
        let v1039: f64 = (if v1036 { (v411 * v1037) } else { v4 });
        let v1040: f64 = (v1 + v1039);
        let v1044: bool = (v1020 && (!v1035));
        let v1046: f64 = ((v697 + v847) - v694);
        let v1048: f64 = (if v1044 { (v847 / v1046) } else { (if v1036 { (v1039 / v1040) } else { v952 }) });
        let v1050: f64 = (if v1020 { v1004 } else { (if v1007 { (v260 * v1011) } else { (if v1003 { v1004 } else { v4 }) }) });
        let v1051: f64 = (if v1020 { v849 } else { (if v850 { (v1014 / v1015) } else { v4 }) });
        let v1054: f64 = (if v1020 { (v1 - (v1051 / self.scalar_v892)) } else { (if v850 { (self.scalar_v892 / v1015) } else { v4 }) });
        let v1058: f64 = (v196 * self.scalar_v1057);
        let v1059: f64 = (v46 * v196);
        let v1060: f64 = (v700 - v1058);
        let v1061: f64 = (v1060 / v1059);
        let v1062: bool = (v700 < v1058);
        let v1063: f64 = ((v1061) as f64).exp();
        let v1064: f64 = (v1 + v1063);
        let v1065: f64 = ((v1064) as f64).ln();
        let v1069: bool = (!v1062);
        let v1071: f64 = (((-v1061)) as f64).exp();
        let v1072: f64 = (v1 + v1071);
        let v1073: f64 = ((v1072) as f64).ln();
        let v1076: f64 = (if v1069 { (v1058 - (v1059 * v1073)) } else { (if v1062 { (v700 - (v1059 * v1065)) } else { v4 }) });
        let v1078: f64 = (v1 - (v284 * v1076));
        let v1080: f64 = f64::powf(v1078, self.scalar_v1079);
        let v1081: f64 = (v196 / self.scalar_v1079);
        let v1082: f64 = (v1 - v1080);
        let v1086: f64 = ((v1081 * v1082) + (v170 * (v700 - v1076)));
        let v1097: f64 = (if self.scalar_v1096 { v697 } else { (if self.scalar_v1092 { (v694 + (if v1020 { v707 } else { (if v850 { (v991 + v998) } else { v4 }) })) } else { (if self.scalar_v1088 { v694 } else { v4 }) }) });
        let v1098: f64 = (v31 - v301);
        let v1099: f64 = (v1 - v301);
        let v1100: f64 = (v1098 / v1099);
        let v1103: f64 = (v1 - f64::powf(v1100, self.scalar_v1101));
        let v1104: f64 = (v260 * v1103);
        let v1105: f64 = (v1097 - v1104);
        let v1106: f64 = (v1105 / v1050);
        let v1107: bool = (v1097 < v1104);
        let v1108: f64 = ((v1106) as f64).exp();
        let v1109: f64 = (v1 + v1108);
        let v1110: f64 = ((v1109) as f64).ln();
        let v1114: bool = (!v1107);
        let v1116: f64 = (((-v1106)) as f64).exp();
        let v1117: f64 = (v1 + v1116);
        let v1118: f64 = ((v1117) as f64).ln();
        let v1121: f64 = (if v1114 { (v1104 - (v1050 * v1118)) } else { (if v1107 { (v1097 - (v1050 * v1110)) } else { v4 }) });
        let v1123: f64 = f64::powf(v1054, self.scalar_v1122);
        let v1125: f64 = (v260 / self.scalar_v1124);
        let v1127: f64 = (v1 - (v1121 / v260));
        let v1128: f64 = f64::powf(v1127, self.scalar_v1124);
        let v1130: f64 = (v1 - (v1123 * v1128));
        let v1132: f64 = (v1100 * v1123);
        let v1133: f64 = (v1097 - v1121);
        let v1135: f64 = ((v1125 * v1130) + (v1132 * v1133));
        let v1138: f64 = ((v1099 * v1135) + (v301 * v694));
        let v1139: f64 = (v423 * v436);
        let v1140: f64 = (v1139 / v441);
        let v1141: f64 = (v755 * v1140);
        let v1143: f64 = (((v1 + v1141)) as f64).sqrt();
        let v1144: f64 = (v1 + v1143);
        let v1145: f64 = (v1141 / v1144);
        let v1146: f64 = (v1 / v402);
        let v1147: f64 = f64::powf(v1024, v1146);
        let v1148: f64 = (v1140 * v1147);
        let v1150: f64 = (((v1 + v1148)) as f64).sqrt();
        let v1151: f64 = (v1 + v1150);
        let v1152: f64 = (v1148 / v1151);
        let v1155: f64 = (v1 + (v1086 / v610));
        let v1156: f64 = (v1138 / v607);
        let v1157: f64 = (v1155 + v1156);
        let v1160: f64 = (v661 * v1155);
        let v1163: f64 = (-v1138);
        let v1164: f64 = (v1163 / v607);
        let v1165: f64 = (v661 * v1164);
        let v1168: f64 = (((if self.scalar_v1159 { (v121 * v1160) } else { v4 })) as f64).exp();
        let v1169: f64 = (((if self.scalar_v1159 { (v121 * v1165) } else { v4 })) as f64).exp();
        let v1170: f64 = (v1168 - v1169);
        let v1172: f64 = (((v121 * v661)) as f64).exp();
        let v1173: f64 = (v1172 - v1);
        let v1175: f64 = (if self.scalar_v1159 { (v1170 / v1173) } else { (if self.scalar_v1153 { v1157 } else { v4 }) });
        let v1176: f64 = 0.010000000000000002;
        let v1177: f64 = (v1175 * v1175);
        let v1178: bool = (v1175 < v4);
        let v1179: f64 = 0.005000000000000001;
        let v1181: f64 = (((v1176 + v1177)) as f64).sqrt();
        let v1182: f64 = (v1181 - v1175);
        let v1185: bool = (!v1178);
        let v1188: f64 = (if v1185 { (v411 * (v1175 + v1181)) } else { (if v1178 { (v1179 / v1182) } else { v4 }) });
        let v1191: f64 = (v1 + (v411 * (v1145 + v1152)));
        let v1192: f64 = (v1188 * v1191);
        let v1194: f64 = (v436 * self.scalar_v1193);
        let v1195: f64 = (v1147 * v1194);
        let v1196: f64 = (v436 * v755);
        let v1197: f64 = (v1196 - v1195);
        let v1198: f64 = (v1197 / v1192);
        let v1199: f64 = 0.0001;
        let v1200: f64 = (v700 / v1199);
        let v1201: bool = (v700 < v4);
        let v1202: f64 = ((v1200) as f64).exp();
        let v1203: f64 = (v1 + v1202);
        let v1207: bool = (!v1201);
        let v1209: f64 = (((-v1200)) as f64).exp();
        let v1210: f64 = (v1 + v1209);
        let v1214: f64 = (if v1207 { (v700 + (v1199 * ((v1210) as f64).ln())) } else { (if v1201 { (v1199 * ((v1203) as f64).ln()) } else { v4 }) });
        let v1216: f64 = (v1214 / self.scalar_v1215);
        let v1217: bool = (v1216 < self.scalar_v734);
        let v1218: f64 = ((v1216) as f64).exp();
        let v1220: bool = (!v1217);
        let v1221: f64 = (if v1220 { self.scalar_v739 } else { v825 });
        let v1225: f64 = (if v1220 { (v1221 * (v1 + (v1216 - self.scalar_v734))) } else { (if v1217 { v1218 } else { v4 }) });
        let v1226: f64 = (v1225 - v1);
        let v1230: f64 = ((v700 - self.scalar_v1228) / v30);
        let v1231: bool = (v700 < self.scalar_v1228);
        let v1232: f64 = ((v1230) as f64).exp();
        let v1233: f64 = (v1 + v1232);
        let v1238: bool = (!v1231);
        let v1240: f64 = (((-v1230)) as f64).exp();
        let v1241: f64 = (v1 + v1240);
        let v1245: f64 = (if v1238 { (self.scalar_v1228 - (v30 * ((v1241) as f64).ln())) } else { (if v1231 { (v700 - (v30 * ((v1233) as f64).ln())) } else { v4 }) });
        let v1247: f64 = (v1245 * self.scalar_v1246);
        let v1248: f64 = (self.scalar_v1228 - v1245);
        let v1249: f64 = f64::powf(v1248, v31);
        let v1251: f64 = (v745 / self.scalar_v478);
        let v1252: bool = (v1251 < self.scalar_v734);
        let v1253: f64 = ((v1251) as f64).exp();
        let v1255: bool = (!v1252);
        let v1256: f64 = (if v1255 { self.scalar_v739 } else { v1221 });
        let v1260: f64 = (if v1255 { (v1256 * (v1 + (v1251 - self.scalar_v734))) } else { (if v1252 { v1253 } else { v1214 }) });
        let v1261: f64 = (v700 - v283);
        let v1262: f64 = (v121 * v1261);
        let v1263: bool = (v1262 < self.scalar_v734);
        let v1264: bool = (self.scalar_v497 && v1263);
        let v1265: f64 = ((v1262) as f64).exp();
        let v1268: bool = (self.scalar_v497 && (!v1263));
        let v1269: f64 = (if v1268 { self.scalar_v739 } else { v1256 });
        let v1273: f64 = (if v1268 { (v1269 * (v1 + (v1262 - self.scalar_v734))) } else { (if v1264 { v1265 } else { v1216 }) });
        let v1276: f64 = ((v1198 / v436) - 1000.0);
        let v1277: f64 = 40.0;
        let v1278: bool = (v1276 < v1277);
        let v1279: bool = (self.scalar_v497 && v1278);
        let v1280: f64 = ((v1276) as f64).exp();
        let v1283: bool = (self.scalar_v497 && (!v1278));
        let v1285: f64 = (if v1283 { 2.3538526683702e17 } else { v1269 });
        let v1289: f64 = (if v1283 { (v1285 * (v1 + (v1276 - v1277))) } else { (if v1279 { v1280 } else { v1225 }) });
        let v1290: f64 = (v1260 - v1);
        let v1291: f64 = (v487 * v1290);
        let v1292: f64 = (v31 * (if self.scalar_v497 { (self.scalar_v498 * v503) } else { v4 }));
        let v1293: f64 = (v1290 * v1292);
        let v1296: f64 = (((v1 + (v423 * v1273))) as f64).sqrt();
        let v1297: f64 = (v1 + v1296);
        let v1298: f64 = (v1293 / v1297);
        let v1299: f64 = (v1 + v1156);
        let v1302: f64 = (v1024 - v1);
        let v1303: f64 = (v512 * v1302);
        let v1304: f64 = (v1289 * v1303);
        let v1305: f64 = (v1 + v1289);
        let v1320: f64 = (self.scalar_v1309 * ((v1024 + v1260) - v31));
        let v1322: f64 = ((v1290 * self.scalar_v1316) + (v1299 * v1320));
        let v1325: f64 = (v121 * v703);
        let v1326: f64 = (v1325 / self.scalar_v489);
        let v1327: bool = (v1326 < self.scalar_v734);
        let v1328: f64 = ((v1326) as f64).exp();
        let v1330: bool = (!v1327);
        let v1331: f64 = (if v1330 { self.scalar_v739 } else { v1285 });
        let v1335: f64 = (if v1330 { (v1331 * (v1 + (v1326 - self.scalar_v734))) } else { (if v1327 { v1328 } else { v1260 }) });
        let v1336: f64 = (v703 - v283);
        let v1337: f64 = (v121 * v1336);
        let v1338: bool = (v1337 < self.scalar_v734);
        let v1339: bool = (self.scalar_v497 && v1338);
        let v1340: f64 = ((v1337) as f64).exp();
        let v1343: bool = (self.scalar_v497 && (!v1338));
        let v1344: f64 = (if v1343 { self.scalar_v739 } else { v1331 });
        let v1349: f64 = (v1335 - v1);
        let v1350: f64 = (v495 * v1349);
        let v1351: f64 = (v31 * (if self.scalar_v497 { (self.scalar_v513 * v518) } else { v4 }));
        let v1352: f64 = (v1349 * v1351);
        let v1355: f64 = (((v1 + (v423 * (if v1343 { (v1344 * (v1 + (v1337 - self.scalar_v734))) } else { (if v1339 { v1340 } else { v1273 }) })))) as f64).sqrt();
        let v1356: f64 = (v1 + v1355);
        let v1361: f64 = (v745 / self.scalar_v450);
        let v1362: bool = (v1361 < self.scalar_v734);
        let v1363: f64 = ((v1361) as f64).exp();
        let v1365: bool = (!v1362);
        let v1366: f64 = (if v1365 { self.scalar_v739 } else { v1344 });
        let v1370: f64 = (if v1365 { (v1366 * (v1 + (v1361 - self.scalar_v734))) } else { (if v1362 { v1363 } else { v1335 }) });
        let v1371: f64 = (v1370 - v1);
        let v1373: f64 = (v1325 / self.scalar_v533);
        let v1374: bool = (v1373 < self.scalar_v734);
        let v1375: f64 = ((v1373) as f64).exp();
        let v1377: bool = (!v1374);
        let v1378: f64 = (if v1377 { self.scalar_v739 } else { v1366 });
        let v1382: f64 = (if v1377 { (v1378 * (v1 + (v1373 - self.scalar_v734))) } else { (if v1374 { v1375 } else { v1370 }) });
        let v1383: f64 = (v1382 - v1);
        let v1385: f64 = (v756 / self.scalar_v463);
        let v1386: bool = (v1385 < self.scalar_v734);
        let v1387: f64 = ((v1385) as f64).exp();
        let v1389: bool = (!v1386);
        let v1390: f64 = (if v1389 { self.scalar_v739 } else { v1378 });
        let v1394: f64 = (if v1389 { (v1390 * (v1 + (v1385 - self.scalar_v734))) } else { (if v1386 { v1387 } else { v1382 }) });
        let v1395: f64 = (v1394 - v1);
        let v1396: f64 = (v473 * v1395);
        let v1397: f64 = (v1325 / self.scalar_v543);
        let v1398: bool = (v1397 < self.scalar_v734);
        let v1399: f64 = ((v1397) as f64).exp();
        let v1401: bool = (!v1398);
        let v1402: f64 = (if v1401 { self.scalar_v739 } else { v1390 });
        let v1406: f64 = (if v1401 { (v1402 * (v1 + (v1397 - self.scalar_v734))) } else { (if v1398 { v1399 } else { v1394 }) });
        let v1407: f64 = (v1406 - v1);
        let v1412: bool = (v1201 && self.scalar_v1411);
        let v1413: f64 = (v31 * v1080);
        let v1415: f64 = (v1 - (self.scalar_v34 / v1413));
        let v1416: f64 = (v570 * v1415);
        let v1417: bool = (v1416 < self.scalar_v734);
        let v1418: bool = (v1412 && v1417);
        let v1419: f64 = ((v1416) as f64).exp();
        let v1422: bool = (v1412 && (!v1417));
        let v1423: f64 = (if v1422 { self.scalar_v739 } else { v1402 });
        let v1427: f64 = (if v1422 { (v1423 * (v1 + (v1416 - self.scalar_v734))) } else { (if v1418 { v1419 } else { v4 }) });
        let v1429: f64 = (if v1412 { (v284 * v700) } else { v604 });
        let v1431: f64 = 1e-30;
        let v1433: f64 = ((((v1429 * v1429) + v1431)) as f64).sqrt();
        let v1436: f64 = f64::powf(v1433, self.scalar_v1435);
        let v1444: f64 = (v449 * v1429);
        let v1445: f64 = (v1429 * v1444);
        let v1446: f64 = (v1429 + self.scalar_v1440);
        let v1448: f64 = ((self.scalar_v32 * (self.scalar_v1438 - ((v170 * v1429) * self.scalar_v1440))) - (v1445 * v1446));
        let v1450: f64 = 0.16666666666666666;
        let v1452: f64 = (if v1412 { ((v1436 * v1448) * v1450) } else { v4 });
        let v1453: f64 = (self.scalar_v34 * v700);
        let v1454: f64 = (v570 * v1453);
        let v1455: f64 = (v147 * v1452);
        let v1457: f64 = (if v1412 { (v1454 / v1455) } else { v1429 });
        let v1458: f64 = -0.001;
        let v1459: bool = (v1457 < v1458);
        let v1460: bool = (v1457 < self.scalar_v734);
        let v1461: bool = (v1412 && v1459);
        let v1462: bool = (v1460 && v1461);
        let v1463: f64 = ((v1457) as f64).exp();
        let v1466: bool = (v1461 && (!v1460));
        let v1467: f64 = (if v1466 { self.scalar_v739 } else { v1423 });
        let v1472: f64 = (-v700);
        let v1473: f64 = (v1 - (if v1466 { (v1467 * (v1 + (v1457 - self.scalar_v734))) } else { (if v1462 { v1463 } else { v4 }) }));
        let v1475: f64 = (v1 + (v1473 / v1457));
        let v1479: bool = (v1412 && (!v1459));
        let v1480: f64 = (v411 * v700);
        let v1481: f64 = (v1457 * v1480);
        let v1482: f64 = 0.3333333333333333;
        let v1483: f64 = (v1457 * v1482);
        let v1484: f64 = 0.25;
        let v1486: f64 = (v1 + (v1457 * v1484));
        let v1488: f64 = (v1 + (v1483 * v1486));
        let v1490: f64 = (if v1479 { (v1481 * v1488) } else { (if v1461 { (v1472 * v1475) } else { v4 }) });
        let v1491: f64 = (v31 * (v577 * v579));
        let v1492: f64 = (v1490 * v1491);
        let v1493: f64 = (v1080 * v1492);
        let v1494: f64 = (v1427 * v1493);
        let v1498: bool = (!v1412);
        let v1504: bool = (self.scalar_v1502 && (v694 < v4));
        let v1505: f64 = (v285 * v694);
        let v1506: f64 = (v1 - v1505);
        let v1508: f64 = (if v1504 { f64::powf(v1506, self.scalar_v1124) } else { v4 });
        let v1509: f64 = (v31 * v1508);
        let v1511: f64 = (v1 - (self.scalar_v69 / v1509));
        let v1512: f64 = (v592 * v1511);
        let v1513: bool = (v1512 < self.scalar_v734);
        let v1514: bool = (v1504 && v1513);
        let v1515: f64 = ((v1512) as f64).exp();
        let v1518: bool = (v1504 && (!v1513));
        let v1519: f64 = (if v1518 { self.scalar_v739 } else { v1467 });
        let v1523: f64 = (if v1518 { (v1519 * (v1 + (v1512 - self.scalar_v734))) } else { (if v1514 { v1515 } else { v4 }) });
        let v1524: f64 = (if v1504 { v1505 } else { v582 });
        let v1527: f64 = (((v1431 + (v1524 * v1524))) as f64).sqrt();
        let v1529: f64 = f64::powf(v1527, self.scalar_v1528);
        let v1537: f64 = (v449 * v1524);
        let v1538: f64 = (v1524 * v1537);
        let v1539: f64 = (v1524 + self.scalar_v1533);
        let v1541: f64 = ((self.scalar_v67 * (self.scalar_v1531 - ((v170 * v1524) * self.scalar_v1533))) - (v1538 * v1539));
        let v1544: f64 = (if v1504 { (v1450 * (v1529 * v1541)) } else { v4 });
        let v1545: f64 = (self.scalar_v69 * v694);
        let v1546: f64 = (v592 * v1545);
        let v1547: f64 = (v169 * v1544);
        let v1549: f64 = (if v1504 { (v1546 / v1547) } else { v1524 });
        let v1550: bool = (v1549 < v1458);
        let v1551: bool = (v1549 < self.scalar_v734);
        let v1552: bool = (v1504 && v1550);
        let v1553: bool = (v1551 && v1552);
        let v1554: f64 = ((v1549) as f64).exp();
        let v1557: bool = (v1552 && (!v1551));
        let v1558: f64 = (if v1557 { self.scalar_v739 } else { v1519 });
        let v1563: f64 = (-v694);
        let v1564: f64 = (v1 - (if v1557 { (v1558 * (v1 + (v1549 - self.scalar_v734))) } else { (if v1553 { v1554 } else { v4 }) }));
        let v1566: f64 = (v1 + (v1564 / v1549));
        let v1570: bool = (v1504 && (!v1550));
        let v1571: f64 = (v411 * v694);
        let v1572: f64 = (v1549 * v1571);
        let v1573: f64 = (v1482 * v1549);
        let v1575: f64 = (v1 + (v1484 * v1549));
        let v1577: f64 = (v1 + (v1573 * v1575));
        let v1579: f64 = (if v1570 { (v1572 * v1577) } else { (if v1552 { (v1563 * v1566) } else { v4 }) });
        let v1580: f64 = (v31 * (v599 * v601));
        let v1581: f64 = (v1579 * v1580);
        let v1582: f64 = (v1508 * v1581);
        let v1583: f64 = (v1523 * v1582);
        let v1587: bool = (!v1504);
        let v1588: f64 = (if v1587 { v4 } else { (if v1504 { (self.scalar_v70 * (v285 * v1583)) } else { v4 }) });
        let v1589: f64 = (v765 * v1140);
        let v1590: f64 = (v423 * (if v802 { (v803 * (v1 + (v798 - self.scalar_v734))) } else { (if v799 { v800 } else { v4 }) }));
        let v1591: f64 = (v1589 - v1140);
        let v1593: f64 = (((v1 + v1589)) as f64).sqrt();
        let v1594: f64 = (v1 + v1593);
        let v1595: f64 = (v1591 / v1594);
        let v1597: f64 = (((v1 + v1590)) as f64).sqrt();
        let v1598: f64 = (v1 + v1597);
        let v1599: f64 = (v1590 / v1598);
        let v1600: f64 = (v31 * v531);
        let v1601: f64 = (v765 - v1);
        let v1602: f64 = (v1600 * v1601);
        let v1603: f64 = (v423 * v531);
        let v1604: f64 = (v1603 / v447);
        let v1607: f64 = (((v1 + (v765 * v1604))) as f64).sqrt();
        let v1608: f64 = (v1 + v1607);
        let v1609: f64 = (v1602 / v1608);
        let v1615: f64 = (if self.scalar_v1613 { (self.scalar_v14 * v1609) } else { v1609 });
        let v1617: f64 = (v531 * self.scalar_v1616);
        let v1618: f64 = (v785 - v1);
        let v1619: f64 = (v1617 * v1618);
        let v1622: f64 = (((v1 + (v785 * v1604))) as f64).sqrt();
        let v1623: f64 = (v1 + v1622);
        let v1625: f64 = (if self.scalar_v1613 { (v1619 / v1623) } else { v4 });
        let v1628: f64 = (self.scalar_v13 * v531);
        let v1630: f64 = (if self.scalar_v1627 { (v327 * v1628) } else { v4 });
        let v1631: f64 = (v121 * v1630);
        let v1633: f64 = (v31 - ((v1631) as f64).ln());
        let v1637: f64 = (if self.scalar_v1627 { (v732 - (if self.scalar_v1627 { (v119 * v1633) } else { v4 })) } else { v4 });
        let v1641: f64 = (if self.scalar_v1627 { (v1637 * v1637) } else { v1177 });
        let v1642: bool = (v1637 < v4);
        let v1643: bool = (self.scalar_v1627 && v1642);
        let v1646: f64 = (((self.scalar_v1639 + v1641)) as f64).sqrt();
        let v1647: f64 = (v1646 - v1637);
        let v1651: bool = (self.scalar_v1627 && (!v1642));
        let v1654: f64 = (if v1651 { (v411 * (v1637 + v1646)) } else { (if v1643 { (self.scalar_v1644 / v1647) } else { v4 }) });
        let v1657: f64 = (v1654 + (v1630 + (v327 * v1625)));
        let v1662: f64 = (if self.scalar_v1661 { v1 } else { (if self.scalar_v1627 { (v1654 / v1657) } else { v1 }) });
        let v1664: f64 = (if self.scalar_v1613 { (v1625 * v1662) } else { v4 });
        let v1668: f64 = (if self.scalar_v1666 { (v694 + v705) } else { v4 });
        let v1670: f64 = (-v1668);
        let v1673: bool = (v1670 < v4);
        let v1674: bool = (self.scalar_v1666 && v1673);
        let v1677: f64 = (((self.scalar_v1669 + (if self.scalar_v1666 { (v1668 * v1668) } else { v1641 }))) as f64).sqrt();
        let v1678: f64 = (v1677 - v1670);
        let v1682: bool = (self.scalar_v1666 && (!v1673));
        let v1685: f64 = (if v1682 { (v411 * (v1670 + v1677)) } else { (if v1674 { (self.scalar_v1675 / v1678) } else { v4 }) });
        let v1701: bool = (v1685 < self.scalar_v1693);
        let v1702: bool = (self.scalar_v1666 && v1701);
        let v1703: f64 = (v1685 / self.scalar_v1691);
        let v1705: f64 = (v1 - f64::powf(v1703, self.scalar_v1686));
        let v1709: bool = (self.scalar_v1666 && (!v1701));
        let v1715: f64 = (if self.scalar_v1714 { v1 } else { (if v1709 { (self.scalar_v1690 + (self.scalar_v1700 * (v1685 - self.scalar_v1693))) } else { (if v1702 { (v1 / v1705) } else { v4 }) }) });
        let v1716: f64 = (v1588 * v1715);
        let v1717: f64 = (v1615 * v1715);
        let v1718: f64 = (v1396 * v1715);
        let v1719: f64 = (v1664 * v1715);
        let v1721: bool = (v1157 < v4);
        let v1723: f64 = (((v1176 + (v1157 * v1157))) as f64).sqrt();
        let v1724: f64 = (v1723 - v1157);
        let v1727: bool = (!v1721);
        let v1730: f64 = (if v1727 { (v411 * (v1157 + v1723)) } else { (if v1721 { (v1179 / v1724) } else { v4 }) });
        let v1731: f64 = (v1191 * v1730);
        let v1732: f64 = (v315 / v1731);
        let v1733: bool = (v1732 < self.scalar_v28);
        let v1735: f64 = (v170 * (if v1733 { self.scalar_v28 } else { v1732 }));
        let v1736: f64 = ((if v770 { (v771 * (v1 + (v766 - self.scalar_v734))) } else { (if v767 { v768 } else { v4 }) }) - v1);
        let v1738: f64 = (v705 + (v862 * v1736));
        let v1739: f64 = (v1738 / v1735);
        let v1740: bool = (v1198 > v4);
        let v1744: bool = (v694 < self.scalar_v1743);
        let v1747: f64 = ((-v1198) / self.scalar_v1746);
        let v1748: bool = (v1747 < self.scalar_v734);
        let v1750: bool = (v1744 && (v1740 && self.scalar_v1742));
        let v1751: bool = (v1748 && v1750);
        let v1752: f64 = ((v1747) as f64).exp();
        let v1755: bool = (v1750 && (!v1748));
        let v1756: f64 = (if v1755 { self.scalar_v739 } else { v1558 });
        let v1760: f64 = (if v1755 { (v1756 * (v1 + (v1747 - self.scalar_v734))) } else { (if v1751 { v1752 } else { v4 }) });
        let v1761: f64 = (self.scalar_v1743 - v694);
        let v1763: f64 = (if v1750 { (v1760 * v1761) } else { v4 });
        let v1764: f64 = (-v421);
        let v1766: f64 = f64::powf(v1763, self.scalar_v1765);
        let v1767: f64 = (v1764 * v1766);
        let v1768: bool = (v1767 < self.scalar_v734);
        let v1769: bool = (v1750 && v1768);
        let v1770: f64 = ((v1767) as f64).exp();
        let v1773: bool = (v1750 && (!v1768));
        let v1774: f64 = (if v1773 { self.scalar_v739 } else { v1756 });
        let v1778: f64 = (if v1773 { (v1774 * (v1 + (v1767 - self.scalar_v734))) } else { (if v1769 { v1770 } else { v4 }) });
        let v1780: f64 = (self.scalar_v1779 / v421);
        let v1781: f64 = (v1763 * v1780);
        let v1787: bool = (v1740 && self.scalar_v1786);
        let v1789: bool = ((v694 < v219) && (self.scalar_v1784 && v1787));
        let v1795: f64 = (if v1789 { self.scalar_v1794 } else { v4 });
        let v1796: f64 = (v219 - v694);
        let v1798: f64 = (if v1789 { (v1796 / v1054) } else { v967 });
        let v1801: f64 = ((((v31 * v1798) / v1795)) as f64).sqrt();
        let v1802: f64 = (if v1789 { v1801 } else { v4 });
        let v1805: bool = (v1789 && self.scalar_v1804);
        let v1808: bool = (v1789 && self.scalar_v1807);
        let v1811: f64 = (if v1808 { (v1 - (v411 * v1048)) } else { v4 });
        let v1812: f64 = (self.scalar_v1792 * v1811);
        let v1814: f64 = (if v1808 { (v1811 * v1812) } else { (if v1805 { self.scalar_v1792 } else { v4 }) });
        let v1815: f64 = (v1802 * v1814);
        let v1819: f64 = ((((v1802 * v1802) + (v1814 * v1814))) as f64).sqrt();
        let v1821: f64 = (if v1789 { (v1815 / v1819) } else { v4 });
        let v1823: f64 = (if v1789 { (v1796 / v1821) } else { v4 });
        let v1824: f64 = (v411 * v1821);
        let v1825: f64 = (v1795 * v1824);
        let v1828: f64 = (if v1789 { (v1823 + (v1054 * v1825)) } else { v4 });
        let v1841: f64 = (self.scalar_v892 * (if v1808 { (v1 + (self.scalar_v1831 * (v1 + (v31 * v1048)))) } else { v4 }));
        let v1843: f64 = ((if v1808 { self.scalar_v1839 } else { v4 }) - (v1198 / v1841));
        let v1846: f64 = (if v1808 { (v1823 - (v1825 * v1843)) } else { v4 });
        let v1847: f64 = (v1846 - v1828);
        let v1849: f64 = (v46 * v1823);
        let v1850: f64 = (v1823 * v1849);
        let v1856: f64 = (((if v1808 { ((v1847 * v1847) + ((v1051 * v1850) / self.scalar_v892)) } else { v1798 })) as f64).sqrt();
        let v1859: f64 = (if v1808 { (v411 * ((v1828 + v1846) + v1856)) } else { (if v1805 { v1828 } else { v4 }) });
        let v1860: f64 = (v1859 - v1823);
        let v1862: f64 = (if v1789 { (v1860 / v1859) } else { v4 });
        let v1865: bool = (((v1862) as f64).abs() > 1e-7);
        let v1866: bool = (v1789 && v1865);
        let v1868: f64 = (if v1866 { (v1824 / v1862) } else { v4 });
        let v1869: f64 = (self.scalar_v10 / v659);
        let v1870: f64 = (v1859 * v1869);
        let v1871: f64 = (v1868 * v1870);
        let v1872: f64 = (-v659);
        let v1873: f64 = (v1872 / v1859);
        let v1874: f64 = ((v1873) as f64).exp();
        let v1876: f64 = (v1 + (v1814 / v1868));
        let v1878: f64 = (((v1873 * v1876)) as f64).exp();
        let v1879: f64 = (v1874 - v1878);
        let v1883: bool = (v1789 && (!v1865));
        let v1884: f64 = (self.scalar_v10 * v1814);
        let v1891: bool = (v1744 && (self.scalar_v1887 && (v1787 && self.scalar_v1888)));
        let v1892: f64 = f64::powf(v1761, self.scalar_v1765);
        let v1894: f64 = (v1198 + self.scalar_v1893);
        let v1896: f64 = (v1 - (v1198 / v1894));
        let v1898: f64 = f64::powf(v1896, self.scalar_v1897);
        let v1900: f64 = (if v1891 { (v1892 * v1898) } else { v4 });
        let v1901: bool = (self.scalar_v1804 && v1891);
        let v1903: bool = (self.scalar_v1807 && v1891);
        let v1907: f64 = (if v1903 { ((v1198 - self.scalar_v1904) / self.scalar_v1893) } else { v4 });
        let v1911: f64 = (if v1903 { ((v1907 - v1) / self.scalar_v1909) } else { v1230 });
        let v1912: bool = (v1907 < v1);
        let v1913: bool = (v1903 && v1912);
        let v1914: f64 = ((v1911) as f64).exp();
        let v1915: f64 = (v1 + v1914);
        let v1921: bool = (v1903 && (!v1912));
        let v1923: f64 = (((-v1911)) as f64).exp();
        let v1924: f64 = (v1 + v1923);
        let v1928: f64 = (if v1921 { (v1907 + (self.scalar_v1909 * ((v1924) as f64).ln())) } else { (if v1913 { (v1 + (self.scalar_v1909 * ((v1915) as f64).ln())) } else { v4 }) });
        let v1930: f64 = f64::powf(v1928, self.scalar_v1929);
        let v1932: f64 = (if v1903 { (v1900 * v1930) } else { (if v1901 { v1900 } else { v4 }) });
        let v1933: f64 = (v1764 * v1932);
        let v1934: bool = (v1933 < self.scalar_v734);
        let v1935: bool = (v1891 && v1934);
        let v1936: f64 = ((v1933) as f64).exp();
        let v1939: bool = (v1891 && (!v1934));
        let v1940: f64 = (if v1939 { self.scalar_v739 } else { v1774 });
        let v1944: f64 = (if v1939 { (v1940 * (v1 + (v1933 - self.scalar_v734))) } else { (if v1935 { v1936 } else { v1778 }) });
        let v1945: f64 = (v1761 * v1780);
        let v1947: f64 = (if v1891 { (v1944 * v1945) } else { (if v1883 { (v1874 * v1884) } else { (if v1866 { (v1871 * v1879) } else { (if v1750 { (v1778 * v1781) } else { v4 }) }) }) });
        let v1951: bool = (v1740 && (v1947 > v4));
        let v1952: bool = (self.scalar_v1950 && v1951);
        let v1953: f64 = (v322 + v1735);
        let v1954: f64 = (v1198 * v1953);
        let v1956: f64 = (v1192 / v436);
        let v1961: f64 = (if v1952 { (((v119 / v1954) + (v487 * v1956)) + (v308 / v1953)) } else { v4 });
        let v1962: bool = (self.scalar_v1887 && v1952);
        let v1965: f64 = (if v1962 { ((v1947 - v1961) / v408) } else { v1911 });
        let v1966: bool = (v1947 < v1961);
        let v1967: bool = (v1962 && v1966);
        let v1968: f64 = ((v1965) as f64).exp();
        let v1969: f64 = (v1 + v1968);
        let v1975: bool = (v1962 && (!v1966));
        let v1977: f64 = (((-v1965)) as f64).exp();
        let v1978: f64 = (v1 + v1977);
        let v1982: f64 = (if v1975 { (v1961 - (v408 * ((v1978) as f64).ln())) } else { (if v1967 { (v1947 - (v408 * ((v1969) as f64).ln())) } else { v1947 }) });
        let v1983: f64 = (v1198 * v1982);
        let v1986: bool = (v1952 && self.scalar_v1985);
        let v1987: f64 = (v1961 * v1983);
        let v1988: f64 = (v1961 + v1982);
        let v1992: bool = (v1951 && self.scalar_v1991);
        let v1993: f64 = (if v1992 { v1983 } else { (if v1986 { (v1987 / v1988) } else { (if v1962 { v1983 } else { v4 }) }) });
        let v1994: bool = (v1024 > v4);
        let v1995: f64 = ((v1024) as f64).ln();
        let v1998: bool = (!v1994);
        let v1999: f64 = (if v1998 { v697 } else { (if v1994 { (v119 * v1995) } else { v4 }) });
        let v2001: f64 = (if self.scalar_v1311 { v697 } else { (if self.scalar_v497 { v694 } else { v4 }) });
        let v2002: f64 = (v700 - v1999);
        let v2004: f64 = (v1999 - v694);
        let v2009: f64 = (v710 * v710);
        let v2012: f64 = (v731 * v731);
        let v2015: f64 = (v724 * v724);
        let v2018: f64 = (v721 * v721);
        let v2020: f64 = (((((((v1198 * v2002) + (v849 * v2004)) - (v1993 * v1999)) + (v2009 / v308)) + (v674 * v2012)) + (v682 * v2015)) + (v690 * v2018));
        let v2021: f64 = (v713 * v713);
        let v2028: f64 = (((if self.scalar_v1315 { (v487 * v1322) } else { (if self.scalar_v1312 { v1291 } else { (if self.scalar_v497 { ((v1291 + (v1298 * v1299)) + (v1304 / v1305)) } else { v4 }) }) }) + (v461 * v1371)) + (v4 * v700));
        let v2031: f64 = ((v557 * v1226) + ((v1247 * v1249) + (v2028 - (if v1498 { v4 } else { (if v1412 { (self.scalar_v35 * (v284 * v1494)) } else { v4 }) }))));
        let v2037: f64 = ((v550 * v1407) + ((if self.scalar_v1311 { v1350 } else { (if self.scalar_v497 { (v1350 + (v1352 / v1356)) } else { v4 }) }) + (v541 * v1383)));
        let v2041: f64 = (v4 * v727);
        let v2042: f64 = ((v1717 + v1718) + v2041);
        let v2044: f64 = ((((((v2020 + (v2021 / v322)) + (v705 * v1739)) + (v700 * v2031)) - (v1716 * v2001)) + (v703 * v2037)) + (v727 * v2042));
        let v2049: f64 = (v291 * self.scalar_v2048);
        let v2051: f64 = (v703 - v1058);
        let v2052: f64 = (v2051 / v1059);
        let v2053: bool = (v703 < v1058);
        let v2054: f64 = ((v2052) as f64).exp();
        let v2055: f64 = (v1 + v2054);
        let v2056: f64 = ((v2055) as f64).ln();
        let v2060: bool = (!v2053);
        let v2062: f64 = (((-v2052)) as f64).exp();
        let v2063: f64 = (v1 + v2062);
        let v2064: f64 = ((v2063) as f64).ln();
        let v2067: f64 = (if v2060 { (v1058 - (v1059 * v2064)) } else { (if v2053 { (v703 - (v1059 * v2056)) } else { v4 }) });
        let v2068: f64 = (v291 * self.scalar_v2047);
        let v2070: f64 = (v1 - (v284 * v2067));
        let v2072: f64 = (v1 - f64::powf(v2070, self.scalar_v1079));
        let v2076: f64 = ((v1081 * v2072) + (v170 * (v703 - v2067)));
        let v2079: f64 = (v300 * self.scalar_v2078);
        let v2081: f64 = (v441 * v626);
        let v2082: f64 = (v411 * v2081);
        let v2083: f64 = (v1145 * v2082);
        let v2084: f64 = (v1730 * v2083);
        let v2085: f64 = (v1152 * v2082);
        let v2086: f64 = (v1730 * v2085);
        let v2087: f64 = (v727 - v1104);
        let v2088: f64 = (v2087 / v1004);
        let v2089: bool = (v727 < v1104);
        let v2090: f64 = ((v2088) as f64).exp();
        let v2091: f64 = (v1 + v2090);
        let v2092: f64 = ((v2091) as f64).ln();
        let v2096: bool = (!v2089);
        let v2098: f64 = (((-v2088)) as f64).exp();
        let v2099: f64 = (v1 + v2098);
        let v2100: f64 = ((v2099) as f64).ln();
        let v2103: f64 = (if v2096 { (v1104 - (v1004 * v2100)) } else { (if v2089 { (v727 - (v1004 * v2092)) } else { v4 }) });
        let v2105: f64 = (v1 - (v2103 / v260));
        let v2107: f64 = (v1 - f64::powf(v2105, self.scalar_v1124));
        let v2109: f64 = (v727 - v2103);
        let v2111: f64 = ((v1125 * v2107) + (v1100 * v2109));
        let v2114: f64 = ((v1099 * v2111) + (v301 * v727));
        let v2119: f64 = (v732 - v1104);
        let v2120: f64 = (v2119 / v1004);
        let v2121: bool = (v732 < v1104);
        let v2122: f64 = ((v2120) as f64).exp();
        let v2123: f64 = (v1 + v2122);
        let v2124: f64 = ((v2123) as f64).ln();
        let v2128: bool = (!v2121);
        let v2130: f64 = (((-v2120)) as f64).exp();
        let v2131: f64 = (v1 + v2130);
        let v2132: f64 = ((v2131) as f64).ln();
        let v2135: f64 = (if v2128 { (v1104 - (v1004 * v2132)) } else { (if v2121 { (v732 - (v1004 * v2124)) } else { v4 }) });
        let v2137: f64 = (v1 - (v2135 / v260));
        let v2139: f64 = (v1 - f64::powf(v2137, self.scalar_v1124));
        let v2141: f64 = (v732 - v2135);
        let v2143: f64 = ((v1125 * v2139) + (v1100 * v2141));
        let v2146: f64 = ((v1099 * v2143) + (v301 * v732));
        let v2150: f64 = (v441 * v620);
        let v2151: f64 = (v436 / v441);
        let v2154: f64 = f64::powf(v2151, self.scalar_v2153);
        let v2155: f64 = (v2150 * v2154);
        let v2156: f64 = (v119 * self.scalar_v2152);
        let v2157: f64 = (v700 / v2156);
        let v2158: bool = (v2157 < self.scalar_v734);
        let v2159: f64 = ((v2157) as f64).exp();
        let v2161: bool = (!v2158);
        let v2162: f64 = (if v2161 { self.scalar_v739 } else { v1940 });
        let v2166: f64 = (if v2161 { (v2162 * (v1 + (v2157 - self.scalar_v734))) } else { (if v2158 { v2159 } else { v1406 }) });
        let v2167: f64 = (v2155 * v2166);
        let v2168: f64 = (v423 * v631);
        let v2169: f64 = (v119 * v2168);
        let v2170: f64 = (v2169 / v339);
        let v2171: f64 = (v411 * v2170);
        let v2172: f64 = (v1048 * v2171);
        let v2173: f64 = (v31 + v1037);
        let v2177: f64 = (v411 * v636);
        let v2180: f64 = ((v1595 * v2081) + (v1599 * v2170));
        let v2181: f64 = (v2177 * v2180);
        let v2186: f64 = ((v727 - v240) / self.scalar_v2185);
        let v2187: f64 = (v121 * v2186);
        let v2188: bool = (v2187 < self.scalar_v734);
        let v2190: bool = (v2188 && self.scalar_v2189);
        let v2191: f64 = ((v2187) as f64).exp();
        let v2194: bool = (self.scalar_v2189 && (!v2188));
        let v2195: f64 = (if v2194 { self.scalar_v739 } else { v2162 });
        let v2200: f64 = (v642 * v1600);
        let v2201: f64 = (v765 * v2200);
        let v2204: f64 = (((v1 + (v423 * (if v2194 { (v2195 * (v1 + (v2187 - self.scalar_v734))) } else { (if v2190 { v2191 } else { v4 }) })))) as f64).sqrt();
        let v2205: f64 = (v1 + v2204);
        let v2207: f64 = (if self.scalar_v2189 { (v2201 / v2205) } else { (if self.scalar_v2176 { (v2181 / v633) } else { v4 }) });
        let v2215: f64 = (if self.scalar_v2213 { (v785 * v1140) } else { v4 });
        let v2216: f64 = (v2215 - v1140);
        let v2218: f64 = (((v1 + v2215)) as f64).sqrt();
        let v2219: f64 = (v1 + v2218);
        let v2221: f64 = (if self.scalar_v2213 { (v2216 / v2219) } else { v4 });
        let v2223: f64 = (if self.scalar_v2213 { (v423 * (if v791 { (v792 * (v1 + (v787 - self.scalar_v734))) } else { (if v788 { v789 } else { v4 }) })) } else { v4 });
        let v2225: f64 = (((v1 + v2223)) as f64).sqrt();
        let v2226: f64 = (v1 + v2225);
        let v2228: f64 = (if self.scalar_v2213 { (v2223 / v2226) } else { v4 });
        let v2230: f64 = (v636 * self.scalar_v2229);
        let v2233: f64 = ((v2081 * v2221) + (v2170 * v2228));
        let v2234: f64 = (v2230 * v2233);
        let v2237: f64 = (v732 - v240);
        let v2238: f64 = (v121 * v2237);
        let v2239: bool = (v2238 < self.scalar_v734);
        let v2241: bool = (v2239 && self.scalar_v2240);
        let v2242: f64 = ((v2238) as f64).exp();
        let v2245: bool = (self.scalar_v2240 && (!v2239));
        let v2246: f64 = (if v2245 { self.scalar_v739 } else { v2195 });
        let v2251: f64 = (v642 * v1617);
        let v2252: f64 = (v785 * v2251);
        let v2255: f64 = (((v1 + (v423 * (if v2245 { (v2246 * (v1 + (v2238 - self.scalar_v734))) } else { (if v2241 { v2242 } else { v4 }) })))) as f64).sqrt();
        let v2256: f64 = (v1 + v2255);
        let v2258: f64 = (if self.scalar_v2240 { (v2252 / v2256) } else { (if self.scalar_v2213 { (v2234 / v633) } else { v4 }) });
        let v2266: f64 = (if self.scalar_v2262 { (f64::powf(v1078, self.scalar_v2263) - v170) } else { v4 });
        let v2267: f64 = (if self.scalar_v2262 { v1061 } else { v4 });
        let v2268: bool = (v2267 < v4);
        let v2269: bool = (self.scalar_v2262 && v2268);
        let v2270: f64 = ((v2267) as f64).exp();
        let v2271: f64 = (v1 + v2270);
        let v2275: bool = (self.scalar_v2262 && (!v2268));
        let v2277: f64 = (((-v2267)) as f64).exp();
        let v2278: f64 = (v1 + v2277);
        let v2280: f64 = (if v2275 { (v2277 / v2278) } else { (if v2269 { (v1 / v2271) } else { v4 }) });
        let v2283: f64 = (if self.scalar_v2262 { (v170 + (v2266 * v2280)) } else { v4 });
        let v2286: f64 = (v121 * v1141);
        let v2287: f64 = (v2286 / v371);
        let v2288: f64 = (v411 / v1143);
        let v2290: f64 = (if self.scalar_v2262 { (v2287 * v2288) } else { v4 });
        let v2291: f64 = (v1730 * v2082);
        let v2296: f64 = (v705 * v872);
        let v2298: f64 = ((if self.scalar_v2262 { (v2167 / v2156) } else { v4 }) + ((if self.scalar_v2262 { (v2049 * v2283) } else { v4 }) + (if self.scalar_v2262 { (v2290 * v2291) } else { v4 })));
        let v2307: f64 = (if self.scalar_v2262 { (v2084 + (v2167 * self.scalar_v2301)) } else { v4 });
        let v2316: f64 = (if self.scalar_v2315 { v2084 } else { (if self.scalar_v2262 { (v2307 * self.scalar_v2312) } else { v4 }) });
        let v2317: f64 = (if self.scalar_v2315 { v2086 } else { (if self.scalar_v2262 { (v2086 + (v2307 * self.scalar_v2308)) } else { v4 }) });
        let v2321: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, (v101 * self.scalar_v2319));
        let v2322: f64 = (self.scalar_v27 * v2321);
        let v2339: f64 = (v1 + (v101 / self.scalar_v20));
        let v2354: f64 = (if self.scalar_v2352 { (v101 / self.scalar_v26) } else { (if self.scalar_v2344 { (self.scalar_v2347 * (f64::powf(v2339, self.scalar_v2323) - v1)) } else { (if self.scalar_v2335 { (self.scalar_v2337 * ((v2339) as f64).ln()) } else { (if self.scalar_v2327 { (self.scalar_v27 * (v101 / self.scalar_v666)) } else { v4 }) }) }) });
        let v2355: f64 = (v1195 + v1196);
        let v2356: f64 = (v2355 / v1192);
        let v2363: f64 = (if self.scalar_v2362 { v4 } else { (if self.scalar_v2358 { (((v1993 / v2356)) as f64).abs() } else { v4 }) });
        let v2364: bool = (v2356 > v4);
        let v2365: f64 = (v2316 + v2317);
        let v2368: bool = (!v2364);
        let v2369: f64 = (v626 * v1730);
        let v2371: f64 = (if v2368 { (v1192 * v2369) } else { (if v2364 { (v2365 / v2356) } else { v4 }) });
        let v2384: f64 = (if self.scalar_v2383 { v4 } else { (if self.scalar_v2378 { (v2371 * self.scalar_v2379) } else { (if self.scalar_v2373 { (self.scalar_v2308 * v2371) } else { v4 }) }) });
        let v2396: f64 = (self.scalar_v27 * (self.scalar_v0 * v849));
        let v2398: f64 = (self.scalar_v27 * (self.scalar_v0 * v1198));
        let v2399: f64 = (self.scalar_v0 * v2037);
        let v2400: f64 = (self.scalar_v27 * v2399);
        let v2401: f64 = (self.scalar_v0 * v2031);
        let v2402: f64 = (self.scalar_v27 * v2401);
        let v2405: f64 = (self.scalar_v27 * (self.scalar_v0 * (-v1716)));
        let v2406: f64 = (if self.scalar_v497 { v2405 } else { v4 });
        let v2407: f64 = (if self.scalar_v1311 { v2405 } else { v4 });
        let v2408: f64 = (self.scalar_v0 * v1739);
        let v2409: f64 = (self.scalar_v27 * v2408);
        let v2411: f64 = (self.scalar_v27 * (self.scalar_v0 * (-v1993)));
        let v2412: f64 = (self.scalar_v0 * v710);
        let v2414: f64 = (self.scalar_v27 * (v2412 / v308));
        let v2415: f64 = (self.scalar_v0 * v713);
        let v2417: f64 = (self.scalar_v27 * (v2415 / v322));
        let v2419: f64 = (self.scalar_v27 * (-(v2044 + (v732 * v1719))));
        let v2421: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, (self.scalar_v0 * ((if self.scalar_v2315 { v2167 } else { (if self.scalar_v2262 { (v2167 * self.scalar_v2302) } else { v4 }) }) + ((v1086 * v2049) + v2316))));
        let v2422: f64 = (self.scalar_v27 * v2421);
        let v2424: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, (self.scalar_v0 * (v2068 * v2076)));
        let v2425: f64 = (self.scalar_v27 * v2424);
        let v2427: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, (self.scalar_v0 * ((v2172 * v2173) + ((v1138 * v2079) + v2317))));
        let v2428: f64 = (self.scalar_v27 * v2427);
        let v2430: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, (self.scalar_v0 * (if self.scalar_v2262 { (v2296 * v2298) } else { v4 })));
        let v2431: f64 = (self.scalar_v27 * v2430);
        let v2434: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 5, ((self.scalar_v0 * (v711 - v708)) * self.scalar_v2432));
        let v2435: f64 = (self.scalar_v27 * v2434);
        let v2438: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 6, (v718 * self.scalar_v2436));
        let v2439: f64 = (self.scalar_v27 * v2438);
        let v2441: f64 = (self.scalar_v27 * (self.scalar_v0 * v1719));
        let v2442: f64 = (self.scalar_v0 * v731);
        let v2444: f64 = (self.scalar_v27 * (v674 * v2442));
        let v2446: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 7, (self.scalar_v0 * ((self.scalar_v13 * (self.scalar_v2116 * (v300 * v2146))) + (if self.scalar_v2210 { (v1662 * v2258) } else { v4 }))));
        let v2447: f64 = (self.scalar_v27 * v2446);
        let v2450: f64 = (self.scalar_v27 * (self.scalar_v0 * (v1717 + (v1718 + v2041))));
        let v2452: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 8, (self.scalar_v0 * ((self.scalar_v14 * ((v300 * v2114) * self.scalar_v2116)) + (if self.scalar_v2210 { (self.scalar_v14 * v2207) } else { v2207 }))));
        let v2453: f64 = (self.scalar_v27 * v2452);
        let v2454: f64 = (self.scalar_v0 * v724);
        let v2457: f64 = (if self.scalar_v675 { (self.scalar_v27 * (v682 * v2454)) } else { v4 });
        let v2458: f64 = (self.scalar_v0 * v721);
        let v2461: f64 = (if self.scalar_v683 { (self.scalar_v27 * (v690 * v2458)) } else { v4 });
        let v2462: f64 = ctx.node_voltage(nodes[11]);
        let v2463: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 9, v2462);
        let v2464: f64 = (v2384 * v2463);
        let v2465: f64 = (v2363 * v2462);
        let v2468: f64 = (if v102 { (-(-1.0 / v103)) } else { v1 });
        let v2471: f64 = (if v110 { (v2468 / v112) } else { (if v108 { v2468 } else { v4 }) });
        let v2472: f64 = (v2471 / self.scalar_v17);
        let v2473: f64 = (v118 * v2471);
        let v2475: f64 = (v119 * v119);
        let v2476: f64 = ((-v2473) / v2475);
        let v2477: f64 = (v2472 / v117);
        let v2487: f64 = (-(((v128 * ((v126 * v2471) + (v116 * (self.scalar_v37 * v2471)))) - (v127 * v2471)) / (v128 * v128)));
        let v2488: f64 = (v2487 / v46);
        let v2498: f64 = (if v140 { (v2487 + (v46 * ((v142 * (-v2488)) / v143))) } else { (if v133 { (v46 * ((v134 * v2488) / v135)) } else { v4 }) });
        let v2508: f64 = (-(((v150 * ((v148 * v2471) + (v116 * (self.scalar_v72 * v2471)))) - (v149 * v2471)) / (v150 * v150)));
        let v2509: f64 = (v2508 / v46);
        let v2519: f64 = (if v162 { (v2508 + (v46 * ((v164 * (-v2509)) / v165))) } else { (if v155 { (v46 * ((v156 * v2509) / v157)) } else { v4 }) });
        let v2523: f64 = ((v172 * v2477) + (v125 * (v171 * v2473)));
        let v2526: f64 = (-v2472);
        let v2528: f64 = ((v2523 + (self.scalar_v64 * v2472)) + (self.scalar_v177 * v2526));
        let v2533: f64 = (((v119 * (-v2528)) - (v180 * v2473)) / v2475);
        let v2547: f64 = (if v189 { ((v193 * v2473) + (v119 * ((v191 * (-v2533)) / v192))) } else { (if v182 { (v2528 + ((v185 * v2473) + (v119 * ((v183 * v2533) / v184)))) } else { v4 }) });
        let v2550: f64 = (self.scalar_v200 * v2526);
        let v2551: f64 = ((v2523 + (self.scalar_v197 * v2472)) + v2550);
        let v2556: f64 = (((v119 * (-v2551)) - (v203 * v2473)) / v2475);
        let v2570: f64 = (if v212 { ((v216 * v2473) + (v119 * ((v214 * (-v2556)) / v215))) } else { (if v205 { (v2551 + ((v208 * v2473) + (v119 * ((v206 * v2556) / v207)))) } else { v4 }) });
        let v2573: f64 = (v2550 + (v2523 + (self.scalar_v220 * v2472)));
        let v2578: f64 = (((v119 * (-v2573)) - (v224 * v2473)) / v2475);
        let v2592: f64 = (if v233 { ((v237 * v2473) + (v119 * ((v235 * (-v2578)) / v236))) } else { (if v226 { (v2573 + ((v229 * v2473) + (v119 * ((v227 * v2578) / v228)))) } else { v4 }) });
        let v2595: f64 = (v2550 + (v2523 + (self.scalar_v66 * v2472)));
        let v2600: f64 = (((v119 * (-v2595)) - (v244 * v2473)) / v2475);
        let v2614: f64 = (if v253 { ((v257 * v2473) + (v119 * ((v255 * (-v2600)) / v256))) } else { (if v246 { (v2595 + ((v249 * v2473) + (v119 * ((v247 * v2600) / v248)))) } else { v4 }) });
        let v2618: f64 = ((v2523 + (self.scalar_v261 * v2472)) + (self.scalar_v264 * v2526));
        let v2623: f64 = (((v119 * (-v2618)) - (v267 * v2473)) / v2475);
        let v2637: f64 = (if v276 { ((v280 * v2473) + (v119 * ((v278 * (-v2623)) / v279))) } else { (if v269 { (v2618 + ((v272 * v2473) + (v119 * ((v270 * v2623) / v271)))) } else { v4 }) });
        let v2640: f64 = ((-v2547) / (v196 * v196));
        let v2642: f64 = (v260 * v260);
        let v2643: f64 = ((-v2614) / v2642);
        let v2647: f64 = ((self.scalar_v64 * v2640) * (self.scalar_v32 * f64::powf(v286, self.scalar_v1440)));
        let v2651: f64 = ((self.scalar_v66 * v2643) * (self.scalar_v67 * f64::powf(v288, self.scalar_v1533)));
        let v2652: f64 = (self.scalar_v290 * v2647);
        let v2659: f64 = (self.scalar_v293 * (((-(self.scalar_v66 * v2614)) / v2642) * (self.scalar_v67 * f64::powf(v294, self.scalar_v1533))));
        let v2662: f64 = ((-v2659) / (v297 * v297));
        let v2663: f64 = (self.scalar_v299 * v2659);
        let v2664: f64 = (self.scalar_v292 * v2662);
        let v2668: f64 = (if v307 { v4 } else { (self.scalar_v302 * (v305 * (self.scalar_v303 * v2477))) });
        let v2675: f64 = (if v321 { v4 } else { (self.scalar_v316 * (v319 * (self.scalar_v317 * v2477))) });
        let v2678: f64 = (self.scalar_v323 * (v326 * (self.scalar_v324 * v2477)));
        let v2680: f64 = (v331 * (self.scalar_v329 * v2477));
        let v2685: f64 = (self.scalar_v335 * (v338 * (self.scalar_v336 * v2477)));
        let v2688: f64 = (if self.scalar_v341 { (self.scalar_v342 * (self.scalar_v340 * v2471)) } else { v4 });
        let v2690: f64 = (if self.scalar_v341 { (v2688 / v30) } else { v2623 });
        let v2694: f64 = (if v351 { (v30 * ((v352 * v2690) / v353)) } else { v2688 });
        let v2702: f64 = (if self.scalar_v370 { v4 } else { (if self.scalar_v341 { (if v359 { (v2694 + (v30 * ((v361 * (-v2690)) / v362))) } else { v2694 }) } else { v4 }) });
        let v2705: f64 = (if self.scalar_v373 { (self.scalar_v374 * (self.scalar_v372 * v2471)) } else { v4 });
        let v2707: f64 = (if self.scalar_v373 { (v2705 / v30) } else { v2690 });
        let v2711: f64 = (if v383 { (v30 * ((v384 * v2707) / v385)) } else { v2705 });
        let v2721: f64 = (self.scalar_v403 * (self.scalar_v404 * v2471));
        let v2722: f64 = (v407 * v2721);
        let v2723: f64 = (v2722 + v2722);
        let v2725: f64 = (v2723 / (v31 * v414));
        let v2734: f64 = (if v418 { (v411 * (v2721 + v2725)) } else { (if v410 { ((-(v412 * (v2725 - v2721))) / (v415 * v415)) } else { v4 }) });
        let v2739: f64 = (v371 * v371);
        let v2751: f64 = ((v435 * (self.scalar_v422 * (v430 * (((v371 * (self.scalar_v427 * v2477)) - (v428 * v2702)) / v2739)))) + (v431 * (v435 * (((v371 * (self.scalar_v432 * v2476)) - (v433 * v2702)) / v2739))));
        let v2754: f64 = (self.scalar_v437 * (v440 * (self.scalar_v438 * v2477)));
        let v2761: f64 = (self.scalar_v457 * v2476);
        let v2776: f64 = (self.scalar_v476 * v2477);
        let v2780: f64 = (self.scalar_v483 * v2476);
        let v2785: f64 = ((v486 * (self.scalar_v474 * (v480 * (v2776 / self.scalar_v478)))) + (v481 * (v486 * (v2780 / self.scalar_v478))));
        let v2815: f64 = ((v530 * (self.scalar_v521 * (v525 * (self.scalar_v523 * v2477)))) + (v526 * (v530 * (self.scalar_v528 * v2476))));
        let v2841: f64 = -1.5;
        let v2844: f64 = ((self.scalar_v63 * v2498) * (v559 * f64::powf(v558, v2841)));
        let v2847: f64 = ((-v2647) / (v287 * v287));
        let v2860: f64 = (v284 * (self.scalar_v64 * ((v565 * v2847) + (v561 * ((v564 * v2844) + (v560 * ((v563 * v2498) + (v147 * (self.scalar_v562 * v2498)))))))));
        let v2863: f64 = (self.scalar_v63 * (self.scalar_v63 * ((v567 * v2640) + v2860)));
        let v2875: f64 = ((v576 * v2647) + (v287 * (self.scalar_v65 * (self.scalar_v65 * ((v573 * v2547) + (v196 * ((v572 * v2547) + (v196 * (self.scalar_v571 * v2844)))))))));
        let v2884: f64 = ((self.scalar_v96 * v2519) * (v559 * f64::powf(v581, v2841)));
        let v2897: f64 = ((v587 * ((-v2651) / (v289 * v289))) + (v583 * ((v586 * v2884) + (v582 * ((v585 * v2519) + (v169 * (self.scalar_v584 * v2519)))))));
        let v2903: f64 = (self.scalar_v96 * (self.scalar_v96 * ((v589 * v2643) + (v285 * (self.scalar_v66 * v2897)))));
        let v2915: f64 = ((v598 * v2651) + (v289 * (self.scalar_v97 * (self.scalar_v97 * ((v595 * v2614) + (v260 * ((v594 * v2614) + (v260 * (self.scalar_v593 * v2884)))))))));
        let v2922: f64 = (v604 * (self.scalar_v311 * v2477));
        let v2926: f64 = ((v606 * v2662) + (v298 * (self.scalar_v605 * v2922)));
        let v2941: f64 = (self.scalar_v621 * (v625 * (self.scalar_v623 * v2477)));
        let v2944: f64 = (self.scalar_v627 * (v630 * (self.scalar_v628 * v2477)));
        let v2945: f64 = (v2941 + v2944);
        let v2947: f64 = ((self.scalar_v632 * v2945) / self.scalar_v635);
        let v2950: f64 = (self.scalar_v637 * (v641 * (self.scalar_v639 * v2477)));
        let v2959: f64 = (if v656 { v4 } else { (if v646 { (self.scalar_v12 * ((v647 * v2471) - ((v651 * v2471) + (v644 * (v650 * v2471))))) } else { v4 }) });
        let v2960: f64 = (self.scalar_v660 * v2922);
        let v2966: f64 = (if self.scalar_v673 { v4 } else { (if v671 { v4 } else { (if self.scalar_v667 { ((-v2678) / (v327 * v327)) } else { v4 }) }) });
        let v2972: f64 = (if self.scalar_v681 { v4 } else { (if v679 { v4 } else { (if self.scalar_v675 { ((-(self.scalar_v328 * v2680)) / (v332 * v332)) } else { v4 }) }) });
        let v2978: f64 = (if self.scalar_v689 { v4 } else { (if v687 { v4 } else { (if self.scalar_v683 { ((-(self.scalar_v333 * v2680)) / (v334 * v334)) } else { v4 }) }) });
        let v2983: f64 = (v697 * v2476);
        let v2984: f64 = (self.scalar_v0 * v121);
        let v2985: f64 = (v121 * self.scalar_v2979);
        let v2998: f64 = (v700 * v2476);
        let v3002: f64 = (((v371 * v2998) - (v745 * v2702)) / v2739);
        let v3003: f64 = (v2985 / v371);
        let v3004: f64 = (v2984 / v371);
        let v3014: f64 = (if v750 { (v751 * v3002) } else { (if v747 { (v748 * v3002) } else { v4 }) });
        let v3015: f64 = (if v750 { (v751 * v3003) } else { (if v747 { (v748 * v3003) } else { v4 }) });
        let v3016: f64 = (if v750 { (v751 * v3004) } else { (if v747 { (v748 * v3004) } else { v4 }) });
        let v3017: f64 = (v727 * v2476);
        let v3018: f64 = (v121 * self.scalar_v2980);
        let v3019: f64 = (v121 * self.scalar_v2981);
        let v3035: f64 = (if v760 { (v761 * v3017) } else { (if v757 { (v758 * v3017) } else { v4 }) });
        let v3036: f64 = (if v760 { (v761 * v2984) } else { (if v757 { (v758 * v2984) } else { v4 }) });
        let v3037: f64 = (if v760 { (v761 * v3018) } else { (if v757 { (v758 * v3018) } else { v4 }) });
        let v3038: f64 = (if v760 { (v761 * v3019) } else { (if v757 { (v758 * v3019) } else { v4 }) });
        let v3039: f64 = (if v760 { (v761 * v2985) } else { (if v757 { (v758 * v2985) } else { v4 }) });
        let v3040: f64 = (v705 * v2476);
        let v3053: f64 = (v121 * self.scalar_v2982);
        let v3054: f64 = (v732 * v2476);
        let v3070: f64 = (if v780 { (v781 * v3018) } else { (if v777 { (v778 * v3018) } else { v4 }) });
        let v3071: f64 = (if v780 { (v781 * v3053) } else { (if v777 { (v778 * v3053) } else { v4 }) });
        let v3072: f64 = (if v780 { (v781 * v3054) } else { (if v777 { (v778 * v3054) } else { v4 }) });
        let v3073: f64 = (if v780 { (v781 * v3019) } else { (if v777 { (v778 * v3019) } else { v4 }) });
        let v3074: f64 = (if v780 { (v781 * v2985) } else { (if v777 { (v778 * v2985) } else { v4 }) });
        let v3077: f64 = (v121 * (-v2570));
        let v3078: f64 = ((v786 * v2476) + v3077);
        let v3100: f64 = (v3077 + (v797 * v2476));
        let v3122: f64 = (v3077 + (v808 * v2476));
        let v3132: f64 = (if v813 { (v814 * v3122) } else { (if v810 { (v811 * v3122) } else { v4 }) });
        let v3133: f64 = (if v813 { (v814 * v2984) } else { (if v810 { (v811 * v2984) } else { v4 }) });
        let v3134: f64 = (if v813 { (v814 * v2985) } else { (if v810 { (v811 * v2985) } else { v4 }) });
        let v3136: f64 = (v3077 + (v819 * v2476));
        let v3146: f64 = (if v824 { (v825 * v3136) } else { (if v821 { (v822 * v3136) } else { v4 }) });
        let v3147: f64 = (if v824 { (v825 * v2984) } else { (if v821 { (v822 * v2984) } else { v4 }) });
        let v3148: f64 = (if v824 { (v825 * v2985) } else { (if v821 { (v822 * v2985) } else { v4 }) });
        let v3152: f64 = (v31 * v832);
        let v3153: f64 = ((v423 * v3132) / v3152);
        let v3154: f64 = ((v423 * v3133) / v3152);
        let v3155: f64 = ((v423 * v3134) / v3152);
        let v3159: f64 = (v31 * v835);
        let v3160: f64 = ((v423 * v3146) / v3159);
        let v3161: f64 = ((v423 * v3147) / v3159);
        let v3162: f64 = ((v423 * v3148) / v3159);
        let v3169: f64 = (v837 * v837);
        let v3179: f64 = (if v840 { v4 } else { (((v837 * (v31 * v3146)) - (v836 * v3160)) / v3169) });
        let v3180: f64 = (if v840 { v4 } else { (((v837 * (v31 * v3147)) - (v836 * v3161)) / v3169) });
        let v3181: f64 = (if v840 { v4 } else { (((v837 * (v31 * v3148)) - (v836 * v3162)) / v3169) });
        let v3207: f64 = ((v846 * v2473) + (v119 * ((v3153 - v3160) - ((((v837 * v3153) - (v843 * v3160)) / v3169) / v844))));
        let v3208: f64 = (v119 * ((v3154 - v3161) - ((((v837 * v3154) - (v843 * v3161)) / v3169) / v844)));
        let v3209: f64 = (v119 * ((-v3162) - (((-(v843 * v3162)) / v3169) / v844)));
        let v3210: f64 = (v119 * (v3155 - ((v3155 / v837) / v844)));
        let v3212: f64 = (self.scalar_v2979 + v3210);
        let v3216: f64 = (v339 * v339);
        let v3217: f64 = (((v339 * v3207) - (v848 * v2685)) / v3216);
        let v3218: f64 = (v3208 / v339);
        let v3219: f64 = ((self.scalar_v0 + v3209) / v339);
        let v3220: f64 = (v3212 / v339);
        let v3227: f64 = (v31 * v2473);
        let v3234: f64 = ((v863 * v2685) + (v339 * (v411 * v3217)));
        let v3235: f64 = (v339 * (v411 * v3218));
        let v3236: f64 = (v339 * (v411 * v3219));
        let v3237: f64 = (v339 * (v411 * v3220));
        let v3257: f64 = (if v850 { (v2570 + ((v867 * v3227) + (v862 * (((v864 * v2476) + (v121 * v3234)) / v866)))) } else { v4 });
        let v3258: f64 = (if v850 { ((v862 * ((v121 * v3235) / v866)) - (if v856 { (self.scalar_v0 / v858) } else { (if v853 { self.scalar_v0 } else { v4 }) })) } else { v4 });
        let v3259: f64 = (if v850 { ((v862 * ((v121 * v3236) / v866)) - (if v856 { (self.scalar_v2979 / v858) } else { (if v853 { self.scalar_v2979 } else { v4 }) })) } else { v4 });
        let v3260: f64 = (if v850 { (v862 * ((v121 * v3237) / v866)) } else { v4 });
        let v3263: f64 = (v874 * (if v850 { (v872 * v2570) } else { v4 }));
        let v3265: f64 = (if v850 { (v3263 + v3263) } else { v4 });
        let v3266: f64 = (v871 * v3257);
        let v3268: f64 = (v871 * v3258);
        let v3270: f64 = (v871 * v3259);
        let v3272: f64 = (v871 * v3260);
        let v3280: f64 = (v31 * v883);
        let v3281: f64 = ((v3265 + (if v850 { (v3266 + v3266) } else { v2723 })) / v3280);
        let v3282: f64 = ((if v850 { (v3268 + v3268) } else { v4 }) / v3280);
        let v3283: f64 = ((if v850 { (v3270 + v3270) } else { v4 }) / v3280);
        let v3284: f64 = ((if v850 { (v3272 + v3272) } else { v4 }) / v3280);
        let v3292: f64 = (v884 * v884);
        let v3315: f64 = (if v888 { (v411 * (v3257 + v3281)) } else { (if v880 { (((v884 * (v411 * v3265)) - (v881 * (v3281 - v3257))) / v3292) } else { v4 }) });
        let v3316: f64 = (if v888 { (v411 * (v3258 + v3282)) } else { (if v880 { ((-(v881 * (v3282 - v3258))) / v3292) } else { v4 }) });
        let v3317: f64 = (if v888 { (v411 * (v3259 + v3283)) } else { (if v880 { ((-(v881 * (v3283 - v3259))) / v3292) } else { v4 }) });
        let v3318: f64 = (if v888 { (v411 * (v3260 + v3284)) } else { (if v880 { ((-(v881 * (v3284 - v3260))) / v3292) } else { v4 }) });
        let v3340: f64 = (v899 * v899);
        let v3354: f64 = (if v850 { (((v899 * ((v895 * v3315) + (v891 * v3315))) - (v896 * (self.scalar_v893 * (v3315 + (self.scalar_v892 * v2685))))) / v3340) } else { v4 });
        let v3355: f64 = (if v850 { (((v899 * ((v895 * v3316) + (v891 * v3316))) - (v896 * (self.scalar_v893 * v3316))) / v3340) } else { v4 });
        let v3356: f64 = (if v850 { (((v899 * ((v895 * v3317) + (v891 * v3317))) - (v896 * (self.scalar_v893 * v3317))) / v3340) } else { v4 });
        let v3357: f64 = (if v850 { (((v899 * ((v895 * v3318) + (v891 * v3318))) - (v896 * (self.scalar_v893 * v3318))) / v3340) } else { v4 });
        let v3361: f64 = (v901 * v901);
        let v3375: f64 = (if v850 { (((v901 * v3217) - (v849 * v3354)) / v3361) } else { v4 });
        let v3376: f64 = (if v850 { (((v901 * v3218) - (v849 * v3355)) / v3361) } else { v4 });
        let v3377: f64 = (if v850 { (((v901 * v3219) - (v849 * v3356)) / v3361) } else { v4 });
        let v3378: f64 = (if v850 { (((v901 * v3220) - (v849 * v3357)) / v3361) } else { v4 });
        let v3383: f64 = (if v850 { (v3375 / self.scalar_v905) } else { v2707 });
        let v3384: f64 = (if v850 { (v3376 / self.scalar_v905) } else { v4 });
        let v3385: f64 = (if v850 { (v3377 / self.scalar_v905) } else { v4 });
        let v3386: f64 = (if v850 { (v3378 / self.scalar_v905) } else { v4 });
        let v3431: f64 = (if v850 { ((if v917 { (v3375 + (self.scalar_v905 * ((v919 * (-v3383)) / v920))) } else { (if v909 { (self.scalar_v905 * ((v910 * v3383) / v911)) } else { v4 }) }) / self.scalar_v931) } else { v4 });
        let v3432: f64 = (if v850 { ((if v917 { (v3376 + (self.scalar_v905 * ((v919 * (-v3384)) / v920))) } else { (if v909 { (self.scalar_v905 * ((v910 * v3384) / v911)) } else { v4 }) }) / self.scalar_v931) } else { v4 });
        let v3433: f64 = (if v850 { ((if v917 { (v3377 + (self.scalar_v905 * ((v919 * (-v3385)) / v920))) } else { (if v909 { (self.scalar_v905 * ((v910 * v3385) / v911)) } else { v4 }) }) / self.scalar_v931) } else { v4 });
        let v3434: f64 = (if v850 { ((if v917 { (v3378 + (self.scalar_v905 * ((v919 * (-v3386)) / v920))) } else { (if v909 { (self.scalar_v905 * ((v910 * v3386) / v911)) } else { v4 }) }) / self.scalar_v931) } else { v4 });
        let v3439: f64 = (if v850 { (v3315 / self.scalar_v894) } else { v4 });
        let v3440: f64 = (if v850 { (v3316 / self.scalar_v894) } else { v4 });
        let v3441: f64 = (if v850 { (v3317 / self.scalar_v894) } else { v4 });
        let v3442: f64 = (if v850 { (v3318 / self.scalar_v894) } else { v4 });
        let v3471: f64 = (v31 * v941);
        let v3494: f64 = ((v944 * (((v938 * ((v936 * v3439) + (v935 * (v423 * v3431)))) + (v937 * v3439)) / v3471)) - (v942 * ((v943 * v3439) + (v938 * (v31 * v3431)))));
        let v3495: f64 = (v944 * v944);
        let v3499: f64 = ((v944 * (((v938 * ((v936 * v3440) + (v935 * (v423 * v3432)))) + (v937 * v3440)) / v3471)) - (v942 * ((v943 * v3440) + (v938 * (v31 * v3432)))));
        let v3503: f64 = ((v944 * (((v938 * ((v936 * v3441) + (v935 * (v423 * v3433)))) + (v937 * v3441)) / v3471)) - (v942 * ((v943 * v3441) + (v938 * (v31 * v3433)))));
        let v3507: f64 = ((v944 * (((v938 * ((v936 * v3442) + (v935 * (v423 * v3434)))) + (v937 * v3442)) / v3471)) - (v942 * ((v943 * v3442) + (v938 * (v31 * v3434)))));
        let v3509: f64 = (if v850 { (v3494 / v3495) } else { v4 });
        let v3510: f64 = (if v850 { (v3499 / v3495) } else { v4 });
        let v3511: f64 = (if v850 { (v3503 / v3495) } else { v4 });
        let v3512: f64 = (if v850 { (v3507 / v3495) } else { v4 });
        let v3519: f64 = ((v946 * v3179) + (v841 * v3509));
        let v3522: f64 = ((v946 * v3180) + (v841 * v3510));
        let v3525: f64 = ((v946 * v3181) + (v841 * v3511));
        let v3526: f64 = (v841 * v3512);
        let v3534: f64 = (v950 * v950);
        let v3548: f64 = (if v850 { (((v950 * ((-v3509) + v3519)) - (v949 * v3519)) / v3534) } else { v4 });
        let v3549: f64 = (if v850 { (((v950 * ((-v3510) + v3522)) - (v949 * v3522)) / v3534) } else { v4 });
        let v3550: f64 = (if v850 { (((v950 * ((-v3511) + v3525)) - (v949 * v3525)) / v3534) } else { v4 });
        let v3551: f64 = (if v850 { (((v950 * ((-v3512) + v3526)) - (v949 * v3526)) / v3534) } else { v4 });
        let v3570: f64 = (if v850 { ((v953 * v2476) + (v121 * ((v952 * v3234) + (v864 * v3548)))) } else { v4 });
        let v3571: f64 = (if v850 { (v121 * ((v952 * v3235) + (v864 * v3549))) } else { v4 });
        let v3572: f64 = (if v850 { (v121 * ((v952 * v3236) + (v864 * v3550))) } else { v4 });
        let v3573: f64 = (if v850 { (v121 * ((v952 * v3237) + (v864 * v3551))) } else { v4 });
        let v3595: f64 = (if v850 { ((v31 * v3570) + ((v958 * v3179) + (v841 * (v3179 + v3570)))) } else { v4 });
        let v3596: f64 = (if v850 { ((v31 * v3571) + ((v958 * v3180) + (v841 * (v3180 + v3571)))) } else { v4 });
        let v3597: f64 = (if v850 { ((v31 * v3572) + ((v958 * v3181) + (v841 * (v3181 + v3572)))) } else { v4 });
        let v3598: f64 = (if v850 { ((v31 * v3573) + (v841 * v3573)) } else { v4 });
        let v3603: f64 = (if v850 { (v411 * v3570) } else { v4 });
        let v3604: f64 = (if v850 { (v411 * v3571) } else { v4 });
        let v3605: f64 = (if v850 { (v411 * v3572) } else { v4 });
        let v3606: f64 = (if v850 { (v411 * v3573) } else { v4 });
        let v3607: f64 = (v964 * v3603);
        let v3609: f64 = (v964 * v3604);
        let v3611: f64 = (v964 * v3605);
        let v3613: f64 = (v964 * v3606);
        let v3619: f64 = (if v850 { (v3595 + (v3607 + v3607)) } else { v4 });
        let v3620: f64 = (if v850 { (v3596 + (v3609 + v3609)) } else { v4 });
        let v3621: f64 = (if v850 { (v3597 + (v3611 + v3611)) } else { v4 });
        let v3622: f64 = (if v850 { (v3598 + (v3613 + v3613)) } else { v4 });
        let v3623: f64 = (v31 * v970);
        let v3624: f64 = (v3619 / v3623);
        let v3625: f64 = (v3620 / v3623);
        let v3626: f64 = (v3621 / v3623);
        let v3627: f64 = (v3622 / v3623);
        let v3643: f64 = (v975 * v975);
        let v3661: f64 = (if v980 { v4 } else { (if v974 { (((v975 * v3595) - (v961 * (v3624 - v3603))) / v3643) } else { (if v969 { (v3603 + v3624) } else { v4 }) }) });
        let v3662: f64 = (if v980 { v4 } else { (if v974 { (((v975 * v3596) - (v961 * (v3625 - v3604))) / v3643) } else { (if v969 { (v3604 + v3625) } else { v4 }) }) });
        let v3663: f64 = (if v980 { v4 } else { (if v974 { (((v975 * v3597) - (v961 * (v3626 - v3605))) / v3643) } else { (if v969 { (v3605 + v3626) } else { v4 }) }) });
        let v3664: f64 = (if v980 { v4 } else { (if v974 { (((v975 * v3598) - (v961 * (v3627 - v3606))) / v3643) } else { (if v969 { (v3606 + v3627) } else { v4 }) }) });
        let v3695: f64 = (if v850 { (self.scalar_v988 * v3217) } else { v4 });
        let v3696: f64 = (if v850 { (self.scalar_v988 * v3218) } else { v4 });
        let v3697: f64 = (if v850 { (self.scalar_v988 * v3219) } else { v4 });
        let v3698: f64 = (if v850 { (self.scalar_v988 * v3220) } else { v4 });
        let v3711: f64 = (v991 * v3695);
        let v3713: f64 = (v991 * v3696);
        let v3715: f64 = (v991 * v3697);
        let v3717: f64 = (v991 * v3698);
        let v3723: f64 = (v31 * v998);
        let v3732: f64 = (if v850 { (v3695 + (((if v850 { ((v993 * v3217) + (v849 * (self.scalar_v892 * (self.scalar_v893 * v2685)))) } else { v4 }) + (v3711 + v3711)) / v3723)) } else { v4 });
        let v3736: f64 = (v46 * v2614);
        let v3749: f64 = (v1009 * v1009);
        let v3769: f64 = (if v1007 { ((v1011 * v2614) + (v260 * (((v1009 * (v31 * v3217)) - (v1008 * (v3217 + v3354))) / v3749))) } else { (if v1003 { v3736 } else { v4 }) });
        let v3773: f64 = (self.scalar_v892 * v3217);
        let v3774: f64 = (self.scalar_v892 * v3218);
        let v3775: f64 = (self.scalar_v892 * v3219);
        let v3776: f64 = (self.scalar_v892 * v3220);
        let v3780: f64 = (v1015 * v1015);
        let v3816: f64 = (v843 * v843);
        let v3829: f64 = (if v1020 { (((v843 * (v31 * v3134)) - (v1021 * v3155)) / v3816) } else { v3664 });
        let v3830: f64 = (if v1020 { (if v738 { (v740 * v2983) } else { (if v735 { (v736 * v2983) } else { v4 }) }) } else { (if v850 { ((v985 * ((v982 * v3661) + (v981 * v3661))) + (v983 * (v985 * ((v219 * v2476) + (v121 * v2570))))) } else { v4 }) });
        let v3831: f64 = (if v1020 { (if v738 { (v740 * v2984) } else { (if v735 { (v736 * v2984) } else { v4 }) }) } else { (if v850 { (v985 * ((v982 * v3662) + (v981 * v3662))) } else { v4 }) });
        let v3832: f64 = (if v1020 { v4 } else { (if v850 { (v985 * ((v982 * v3663) + (v981 * v3663))) } else { v4 }) });
        let v3833: f64 = (if v1020 { (if v738 { (v740 * v2985) } else { (if v735 { (v736 * v2985) } else { v4 }) }) } else { (if v850 { (v985 * ((v982 * v3664) + (v981 * v3664))) } else { v4 }) });
        let v3834: f64 = (v3179 + (if v1020 { (((v843 * (v31 * v3132)) - (v1021 * v3153)) / v3816) } else { v3661 }));
        let v3835: f64 = (v3180 + (if v1020 { (((v843 * (v31 * v3133)) - (v1021 * v3154)) / v3816) } else { v3662 }));
        let v3836: f64 = (v3181 + (if v1020 { v4 } else { v3663 }));
        let v3841: f64 = (if v1036 { (v411 * v3834) } else { v4 });
        let v3842: f64 = (if v1036 { (v411 * v3835) } else { v4 });
        let v3843: f64 = (if v1036 { (v411 * v3836) } else { v4 });
        let v3844: f64 = (if v1036 { (v411 * v3829) } else { v4 });
        let v3848: f64 = (v1040 * v1040);
        let v3872: f64 = (v1046 * v1046);
        let v3886: f64 = (if v1044 { (((v1046 * v3207) - (v847 * v3207)) / v3872) } else { (if v1036 { (((v1040 * v3841) - (v1039 * v3841)) / v3848) } else { v3548 }) });
        let v3887: f64 = (if v1044 { (((v1046 * v3208) - (v847 * ((self.scalar_v0 + v3208) - self.scalar_v0))) / v3872) } else { (if v1036 { (((v1040 * v3842) - (v1039 * v3842)) / v3848) } else { v3549 }) });
        let v3888: f64 = (if v1044 { (((v1046 * v3209) - (v847 * (v3209 - self.scalar_v2979))) / v3872) } else { (if v1036 { (((v1040 * v3843) - (v1039 * v3843)) / v3848) } else { v3550 }) });
        let v3889: f64 = (if v1044 { (((v1046 * v3210) - (v847 * v3212)) / v3872) } else { (if v1036 { (((v1040 * v3844) - (v1039 * v3844)) / v3848) } else { v3551 }) });
        let v3894: f64 = (if v1020 { v3736 } else { v3769 });
        let v3895: f64 = (if v1020 { v4 } else { (if v1007 { (v260 * (((v1009 * (v31 * v3218)) - (v1008 * (v3218 + v3355))) / v3749)) } else { v4 }) });
        let v3896: f64 = (if v1020 { v4 } else { (if v1007 { (v260 * (((v1009 * (v31 * v3219)) - (v1008 * (v3219 + v3356))) / v3749)) } else { v4 }) });
        let v3897: f64 = (if v1020 { v4 } else { (if v1007 { (v260 * (((v1009 * (v31 * v3220)) - (v1008 * (v3220 + v3357))) / v3749)) } else { v4 }) });
        let v3898: f64 = (if v1020 { v3217 } else { (if v850 { (((v1015 * v3773) - (v1014 * v3217)) / v3780) } else { v4 }) });
        let v3899: f64 = (if v1020 { v3218 } else { (if v850 { (((v1015 * v3774) - (v1014 * v3218)) / v3780) } else { v4 }) });
        let v3900: f64 = (if v1020 { v3219 } else { (if v850 { (((v1015 * v3775) - (v1014 * v3219)) / v3780) } else { v4 }) });
        let v3901: f64 = (if v1020 { v3220 } else { (if v850 { (((v1015 * v3776) - (v1014 * v3220)) / v3780) } else { v4 }) });
        let v3910: f64 = (if v1020 { (-(v3898 / self.scalar_v892)) } else { (if v850 { ((-v3773) / v3780) } else { v4 }) });
        let v3911: f64 = (if v1020 { (-(v3899 / self.scalar_v892)) } else { (if v850 { ((-v3774) / v3780) } else { v4 }) });
        let v3912: f64 = (if v1020 { (-(v3900 / self.scalar_v892)) } else { (if v850 { ((-v3775) / v3780) } else { v4 }) });
        let v3913: f64 = (if v1020 { (-(v3901 / self.scalar_v892)) } else { (if v850 { ((-v3776) / v3780) } else { v4 }) });
        let v3914: f64 = (self.scalar_v1057 * v2547);
        let v3915: f64 = (v46 * v2547);
        let v3917: f64 = (v1059 * (-v3914));
        let v3920: f64 = (v1059 * v1059);
        let v3921: f64 = ((v3917 - (v1060 * v3915)) / v3920);
        let v3922: f64 = (self.scalar_v2979 / v1059);
        let v3923: f64 = (self.scalar_v0 / v1059);
        let v3942: f64 = (-v3922);
        let v3943: f64 = (-v3923);
        let v3958: f64 = (if v1069 { (v3914 - ((v1073 * v3915) + (v1059 * ((v1071 * (-v3921)) / v1072)))) } else { (if v1062 { (-((v1065 * v3915) + (v1059 * ((v1063 * v3921) / v1064)))) } else { v4 }) });
        let v3959: f64 = (if v1069 { (-(v1059 * ((v1071 * v3942) / v1072))) } else { (if v1062 { (self.scalar_v2979 - (v1059 * ((v1063 * v3922) / v1064))) } else { v4 }) });
        let v3960: f64 = (if v1069 { (-(v1059 * ((v1071 * v3943) / v1072))) } else { (if v1062 { (self.scalar_v0 - (v1059 * ((v1063 * v3923) / v1064))) } else { v4 }) });
        let v3966: f64 = (-((v1076 * v2640) + (v284 * v3958)));
        let v3967: f64 = (-(v284 * v3959));
        let v3968: f64 = (-(v284 * v3960));
        let v3971: f64 = (self.scalar_v1079 * f64::powf(v1078, self.scalar_v3969));
        let v3972: f64 = (v3966 * v3971);
        let v3973: f64 = (v3967 * v3971);
        let v3974: f64 = (v3968 * v3971);
        let v3975: f64 = (v2547 / self.scalar_v1079);
        let v3990: f64 = (((v1082 * v3975) + (v1081 * (-v3972))) + (v170 * (-v3958)));
        let v3991: f64 = ((v1081 * (-v3973)) + (v170 * (self.scalar_v2979 - v3959)));
        let v3992: f64 = ((v1081 * (-v3974)) + (v170 * (self.scalar_v0 - v3960)));
        let v3998: f64 = (if self.scalar_v1092 { (self.scalar_v0 + (if v1020 { v4 } else { (if v850 { (v3696 + (((if v850 { (v993 * v3218) } else { v4 }) + (v3713 + v3713)) / v3723)) } else { v4 }) })) } else { self.scalar_v3993 });
        let v3999: f64 = (if self.scalar_v1092 { (self.scalar_v2979 + (if v1020 { self.scalar_v0 } else { (if v850 { (v3697 + (((if v850 { (v993 * v3219) } else { v4 }) + (v3715 + v3715)) / v3723)) } else { v4 }) })) } else { self.scalar_v3994 });
        let v4001: f64 = (if self.scalar_v1096 { v4 } else { (if self.scalar_v1092 { (if v1020 { v4 } else { v3732 }) } else { v4 }) });
        let v4002: f64 = (if self.scalar_v1096 { self.scalar_v0 } else { v3998 });
        let v4003: f64 = (if self.scalar_v1096 { v4 } else { v3999 });
        let v4004: f64 = (if self.scalar_v1096 { self.scalar_v2979 } else { (if self.scalar_v1092 { (if v1020 { self.scalar_v2979 } else { (if v850 { (v3698 + (((if v850 { (v993 * v3220) } else { v4 }) + (v3717 + v3717)) / v3723)) } else { v4 }) }) } else { v4 }) });
        let v4005: f64 = (-v2664);
        let v4010: f64 = (((v1099 * v4005) - (v1098 * v4005)) / (v1099 * v1099));
        let v4018: f64 = ((v1103 * v2614) + (v260 * (-(v4010 * (self.scalar_v1101 * f64::powf(v1100, self.scalar_v4011))))));
        let v4023: f64 = (v1050 * v1050);
        let v4024: f64 = (((v1050 * (v4001 - v4018)) - (v1105 * v3894)) / v4023);
        let v4028: f64 = (((v1050 * v4002) - (v1105 * v3895)) / v4023);
        let v4032: f64 = (((v1050 * v4003) - (v1105 * v3896)) / v4023);
        let v4036: f64 = (((v1050 * v4004) - (v1105 * v3897)) / v4023);
        let v4093: f64 = (if v1114 { (v4018 - ((v1118 * v3894) + (v1050 * ((v1116 * (-v4024)) / v1117)))) } else { (if v1107 { (v4001 - ((v1110 * v3894) + (v1050 * ((v1108 * v4024) / v1109)))) } else { v4 }) });
        let v4094: f64 = (if v1114 { (-((v1118 * v3895) + (v1050 * ((v1116 * (-v4028)) / v1117)))) } else { (if v1107 { (v4002 - ((v1110 * v3895) + (v1050 * ((v1108 * v4028) / v1109)))) } else { v4 }) });
        let v4095: f64 = (if v1114 { (-((v1118 * v3896) + (v1050 * ((v1116 * (-v4032)) / v1117)))) } else { (if v1107 { (v4003 - ((v1110 * v3896) + (v1050 * ((v1108 * v4032) / v1109)))) } else { v4 }) });
        let v4096: f64 = (if v1114 { (-((v1118 * v3897) + (v1050 * ((v1116 * (-v4036)) / v1117)))) } else { (if v1107 { (v4004 - ((v1110 * v3897) + (v1050 * ((v1108 * v4036) / v1109)))) } else { v4 }) });
        let v4099: f64 = (self.scalar_v1122 * f64::powf(v1054, self.scalar_v4097));
        let v4100: f64 = (v3910 * v4099);
        let v4101: f64 = (v3911 * v4099);
        let v4102: f64 = (v3912 * v4099);
        let v4103: f64 = (v3913 * v4099);
        let v4104: f64 = (v2614 / self.scalar_v1124);
        let v4118: f64 = (self.scalar_v1124 * f64::powf(v1127, self.scalar_v4116));
        let v4141: f64 = ((v1130 * v4104) + (v1125 * (-((v1128 * v4100) + (v1123 * ((-(((v260 * v4093) - (v1121 * v2614)) / v2642)) * v4118))))));
        let v4168: f64 = ((v1125 * (-((v1128 * v4101) + (v1123 * ((-(v4094 / v260)) * v4118))))) + ((v1133 * (v1100 * v4101)) + (v1132 * (v4002 - v4094))));
        let v4169: f64 = ((v1125 * (-((v1128 * v4102) + (v1123 * ((-(v4095 / v260)) * v4118))))) + ((v1133 * (v1100 * v4102)) + (v1132 * (v4003 - v4095))));
        let v4170: f64 = ((v1125 * (-((v1128 * v4103) + (v1123 * ((-(v4096 / v260)) * v4118))))) + ((v1133 * (v1100 * v4103)) + (v1132 * (v4004 - v4096))));
        let v4176: f64 = (v1099 * v4170);
        let v4178: f64 = (self.scalar_v0 * v301);
        let v4179: f64 = (v301 * self.scalar_v2979);
        let v4180: f64 = (((v1135 * v4005) + (v1099 * (v4141 + ((v1133 * ((v1123 * v4010) + (v1100 * v4100))) + (v1132 * (v4001 - v4093)))))) + (v694 * v2664));
        let v4181: f64 = ((v1099 * v4168) + v4178);
        let v4182: f64 = ((v1099 * v4169) + v4179);
        let v4187: f64 = (v441 * v441);
        let v4188: f64 = (((v441 * (v423 * v2751)) - (v1139 * v2754)) / v4187);
        let v4191: f64 = ((v1140 * v3014) + (v755 * v4188));
        let v4192: f64 = (v1140 * v3015);
        let v4193: f64 = (v1140 * v3016);
        let v4194: f64 = (v31 * v1143);
        let v4195: f64 = (v4191 / v4194);
        let v4196: f64 = (v4192 / v4194);
        let v4197: f64 = (v4193 / v4194);
        let v4201: f64 = (v1144 * v1144);
        let v4202: f64 = (((v1144 * v4191) - (v1141 * v4195)) / v4201);
        let v4206: f64 = (((v1144 * v4192) - (v1141 * v4196)) / v4201);
        let v4210: f64 = (((v1144 * v4193) - (v1141 * v4197)) / v4201);
        let v4216: f64 = (v1146 * f64::powf(v1024, (v1146 - v1)));
        let v4219: f64 = (((-(if self.scalar_v401 { v4 } else { (if self.scalar_v373 { (if v391 { (v2711 + (v30 * ((v393 * (-v2707)) / v394))) } else { v2711 }) } else { v4 }) })) / (v402 * v402)) * (v1147 * v1995));
        let v4220: f64 = ((v3830 * v4216) + v4219);
        let v4221: f64 = (v3831 * v4216);
        let v4222: f64 = (v3832 * v4216);
        let v4223: f64 = (v3833 * v4216);
        let v4226: f64 = ((v1147 * v4188) + (v1140 * v4220));
        let v4227: f64 = (v1140 * v4221);
        let v4228: f64 = (v1140 * v4222);
        let v4229: f64 = (v1140 * v4223);
        let v4230: f64 = (v31 * v1150);
        let v4238: f64 = (v1151 * v1151);
        let v4239: f64 = (((v1151 * v4226) - (v1148 * (v4226 / v4230))) / v4238);
        let v4243: f64 = (((v1151 * v4227) - (v1148 * (v4227 / v4230))) / v4238);
        let v4247: f64 = (((v1151 * v4228) - (v1148 * (v4228 / v4230))) / v4238);
        let v4251: f64 = (((v1151 * v4229) - (v1148 * (v4229 / v4230))) / v4238);
        let v4256: f64 = (((v610 * v3990) - (v1086 * ((v609 * v2847) + (v561 * (self.scalar_v608 * v2922))))) / (v610 * v610));
        let v4257: f64 = (v3991 / v610);
        let v4258: f64 = (v3992 / v610);
        let v4262: f64 = (v607 * v607);
        let v4263: f64 = (((v607 * v4180) - (v1138 * v2926)) / v4262);
        let v4264: f64 = (v4181 / v607);
        let v4265: f64 = (v4182 / v607);
        let v4266: f64 = (v4176 / v607);
        let v4267: f64 = (v4256 + v4263);
        let v4268: f64 = (v4258 + v4264);
        let v4310: f64 = (if self.scalar_v1159 { ((v1165 * v2476) + (v121 * ((v1164 * v2960) + (v661 * (((v607 * (-v4180)) - (v1163 * v2926)) / v4262))))) } else { v4 });
        let v4331: f64 = ((v1173 * ((v1168 * (if self.scalar_v1159 { ((v1160 * v2476) + (v121 * ((v1155 * v2960) + (v661 * v4256)))) } else { v4 })) - (v1169 * v4310))) - (v1170 * (v1172 * ((v661 * v2476) + (v121 * v2960)))));
        let v4335: f64 = (((v1168 * (if self.scalar_v1159 { (v121 * (v661 * v4258)) } else { v4 })) - (v1169 * (if self.scalar_v1159 { (v121 * (v661 * ((-v4181) / v607))) } else { v4 }))) / v1173);
        let v4338: f64 = (if self.scalar_v1159 { (v4331 / (v1173 * v1173)) } else { (if self.scalar_v1153 { v4267 } else { v4 }) });
        let v4339: f64 = (if self.scalar_v1159 { ((v1168 * (if self.scalar_v1159 { (v121 * (v661 * v4257)) } else { v4 })) / v1173) } else { (if self.scalar_v1153 { v4257 } else { v4 }) });
        let v4340: f64 = (if self.scalar_v1159 { v4335 } else { (if self.scalar_v1153 { v4268 } else { v4 }) });
        let v4341: f64 = (if self.scalar_v1159 { ((-(v1169 * (if self.scalar_v1159 { (v121 * (v661 * ((-v4182) / v607))) } else { v4 }))) / v1173) } else { (if self.scalar_v1153 { v4265 } else { v4 }) });
        let v4342: f64 = (if self.scalar_v1159 { ((-(v1169 * (if self.scalar_v1159 { (v121 * (v661 * ((-v4176) / v607))) } else { v4 }))) / v1173) } else { (if self.scalar_v1153 { v4266 } else { v4 }) });
        let v4343: f64 = (v1175 * v4338);
        let v4344: f64 = (v4343 + v4343);
        let v4345: f64 = (v1175 * v4339);
        let v4346: f64 = (v4345 + v4345);
        let v4347: f64 = (v1175 * v4340);
        let v4348: f64 = (v4347 + v4347);
        let v4349: f64 = (v1175 * v4341);
        let v4350: f64 = (v4349 + v4349);
        let v4351: f64 = (v1175 * v4342);
        let v4352: f64 = (v4351 + v4351);
        let v4353: f64 = (v31 * v1181);
        let v4354: f64 = (v4344 / v4353);
        let v4355: f64 = (v4346 / v4353);
        let v4356: f64 = (v4348 / v4353);
        let v4357: f64 = (v4350 / v4353);
        let v4358: f64 = (v4352 / v4353);
        let v4366: f64 = (v1182 * v1182);
        let v4402: f64 = (v411 * (v4202 + v4239));
        let v4403: f64 = (v411 * v4206);
        let v4404: f64 = (v411 * (v4210 + v4243));
        let v4405: f64 = (v411 * v4247);
        let v4406: f64 = (v411 * v4251);
        let v4409: f64 = ((v1191 * (if v1185 { (v411 * (v4338 + v4354)) } else { (if v1178 { ((-(v1179 * (v4354 - v4338))) / v4366) } else { v4 }) })) + (v1188 * v4402));
        let v4412: f64 = ((v1191 * (if v1185 { (v411 * (v4339 + v4355)) } else { (if v1178 { ((-(v1179 * (v4355 - v4339))) / v4366) } else { v4 }) })) + (v1188 * v4403));
        let v4415: f64 = ((v1191 * (if v1185 { (v411 * (v4340 + v4356)) } else { (if v1178 { ((-(v1179 * (v4356 - v4340))) / v4366) } else { v4 }) })) + (v1188 * v4404));
        let v4418: f64 = ((v1191 * (if v1185 { (v411 * (v4341 + v4357)) } else { (if v1178 { ((-(v1179 * (v4357 - v4341))) / v4366) } else { v4 }) })) + (v1188 * v4405));
        let v4421: f64 = ((v1191 * (if v1185 { (v411 * (v4342 + v4358)) } else { (if v1178 { ((-(v1179 * (v4358 - v4342))) / v4366) } else { v4 }) })) + (v1188 * v4406));
        let v4425: f64 = ((v1194 * v4220) + (v1147 * (self.scalar_v1193 * v2751)));
        let v4426: f64 = (v1194 * v4221);
        let v4427: f64 = (v1194 * v4222);
        let v4428: f64 = (v1194 * v4223);
        let v4431: f64 = ((v755 * v2751) + (v436 * v3014));
        let v4433: f64 = (v436 * v3016);
        let v4441: f64 = (v1192 * v1192);
        let v4442: f64 = (((v1192 * (v4431 - v4425)) - (v1197 * v4409)) / v4441);
        let v4443: f64 = (v1192 * (v436 * v3015));
        let v4446: f64 = ((v4443 - (v1197 * v4412)) / v4441);
        let v4450: f64 = (((v1192 * (v4433 - v4426)) - (v1197 * v4415)) / v4441);
        let v4454: f64 = (((v1192 * (-v4427)) - (v1197 * v4418)) / v4441);
        let v4458: f64 = (((v1192 * (-v4428)) - (v1197 * v4421)) / v4441);
        let v4479: f64 = (if v1207 { (self.scalar_v2979 + (v1199 * ((v1209 * self.scalar_v4469) / v1210))) } else { (if v1201 { (v1199 * ((v1202 * self.scalar_v4459) / v1203)) } else { v4 }) });
        let v4480: f64 = (if v1207 { (self.scalar_v0 + (v1199 * ((v1209 * self.scalar_v4470) / v1210))) } else { (if v1201 { (v1199 * ((v1202 * self.scalar_v4460) / v1203)) } else { v4 }) });
        let v4481: f64 = (v4479 / self.scalar_v1215);
        let v4482: f64 = (v4480 / self.scalar_v1215);
        let v4489: f64 = (if v1220 { (v1221 * v4481) } else { (if v1217 { (v1218 * v4481) } else { v4 }) });
        let v4490: f64 = (if v1220 { (v1221 * v4482) } else { (if v1217 { (v1218 * v4482) } else { v4 }) });
        let v4516: f64 = (if v1238 { (-(v30 * ((v1240 * self.scalar_v4506) / v1241))) } else { (if v1231 { (self.scalar_v2979 - (v30 * ((v1232 * self.scalar_v4494) / v1233))) } else { v4 }) });
        let v4517: f64 = (if v1238 { (-(v30 * ((v1240 * self.scalar_v4507) / v1241))) } else { (if v1231 { (self.scalar_v0 - (v30 * ((v1232 * self.scalar_v4495) / v1233))) } else { v4 }) });
        let v4523: f64 = (v31 * f64::powf(v1248, v1));
        let v4532: f64 = (v2998 / self.scalar_v478);
        let v4533: f64 = (v2985 / self.scalar_v478);
        let v4534: f64 = (v2984 / self.scalar_v478);
        let v4544: f64 = (if v1255 { (v1256 * v4532) } else { (if v1252 { (v1253 * v4532) } else { v4 }) });
        let v4545: f64 = (if v1255 { (v1256 * v4533) } else { (if v1252 { (v1253 * v4533) } else { v4479 }) });
        let v4546: f64 = (if v1255 { (v1256 * v4534) } else { (if v1252 { (v1253 * v4534) } else { v4480 }) });
        let v4549: f64 = (v121 * (-v2637));
        let v4550: f64 = ((v1261 * v2476) + v4549);
        let v4560: f64 = (if v1268 { (v1269 * v4550) } else { (if v1264 { (v1265 * v4550) } else { v4 }) });
        let v4561: f64 = (if v1268 { (v1269 * v2985) } else { (if v1264 { (v1265 * v2985) } else { v4481 }) });
        let v4562: f64 = (if v1268 { (v1269 * v2984) } else { (if v1264 { (v1265 * v2984) } else { v4482 }) });
        let v4566: f64 = (v436 * v436);
        let v4567: f64 = (((v436 * v4442) - (v1198 * v2751)) / v4566);
        let v4568: f64 = (v4446 / v436);
        let v4569: f64 = (v4450 / v436);
        let v4570: f64 = (v4454 / v436);
        let v4571: f64 = (v4458 / v436);
        let v4587: f64 = (if v1283 { (v1285 * v4567) } else { (if v1279 { (v1280 * v4567) } else { v4 }) });
        let v4588: f64 = (if v1283 { (v1285 * v4568) } else { (if v1279 { (v1280 * v4568) } else { v4489 }) });
        let v4589: f64 = (if v1283 { (v1285 * v4569) } else { (if v1279 { (v1280 * v4569) } else { v4490 }) });
        let v4590: f64 = (if v1283 { (v1285 * v4570) } else { (if v1279 { (v1280 * v4570) } else { v4 }) });
        let v4591: f64 = (if v1283 { (v1285 * v4571) } else { (if v1279 { (v1280 * v4571) } else { v4 }) });
        let v4594: f64 = ((v1290 * v2785) + (v487 * v4544));
        let v4595: f64 = (v487 * v4545);
        let v4596: f64 = (v487 * v4546);
        let v4606: f64 = (v31 * v1296);
        let v4612: f64 = ((v1297 * ((v1292 * v4544) + (v1290 * (v31 * (if self.scalar_v497 { (self.scalar_v498 * (v503 * ((self.scalar_v500 * v2476) / self.scalar_v478))) } else { v4 }))))) - (v1293 * ((v423 * v4560) / v4606)));
        let v4613: f64 = (v1297 * v1297);
        let v4656: f64 = ((v1305 * ((v1303 * v4587) + (v1289 * ((v1302 * (if self.scalar_v497 { (self.scalar_v506 * (v510 * (self.scalar_v508 * v2476))) } else { v4 })) + (v512 * v3830))))) - (v1304 * v4587));
        let v4657: f64 = (v1305 * v1305);
        let v4676: f64 = ((v4595 + (v1299 * (((v1297 * (v1292 * v4545)) - (v1293 * ((v423 * v4561) / v4606))) / v4613))) + (((v1305 * (v1303 * v4588)) - (v1304 * v4588)) / v4657));
        let v4677: f64 = ((v4596 + ((v1299 * (((v1297 * (v1292 * v4546)) - (v1293 * ((v423 * v4562) / v4606))) / v4613)) + (v1298 * v4264))) + (((v1305 * ((v1303 * v4589) + (v1289 * (v512 * v3831)))) - (v1304 * v4589)) / v4657));
        let v4688: f64 = (if self.scalar_v1312 { v4 } else { (if self.scalar_v497 { ((v1298 * v4265) + (((v1305 * ((v1303 * v4590) + (v1289 * (v512 * v3832)))) - (v1304 * v4590)) / v4657)) } else { v4 }) });
        let v4689: f64 = (if self.scalar_v1312 { v4 } else { (if self.scalar_v497 { ((v1298 * v4266) + (((v1305 * ((v1303 * v4591) + (v1289 * (v512 * v3833)))) - (v1304 * v4591)) / v4657)) } else { v4 }) });
        let v4723: f64 = (if self.scalar_v1315 { ((v1322 * v2785) + (v487 * ((self.scalar_v1316 * v4544) + ((v1320 * v4263) + (v1299 * (self.scalar_v1309 * (v3830 + v4544))))))) } else { (if self.scalar_v1312 { v4594 } else { (if self.scalar_v497 { ((v4594 + ((v1299 * (v4612 / v4613)) + (v1298 * v4263))) + (v4656 / v4657)) } else { v4 }) }) });
        let v4725: f64 = (if self.scalar_v1315 { (v487 * ((self.scalar_v1316 * v4546) + ((v1320 * v4264) + (v1299 * (self.scalar_v1309 * (v3831 + v4546)))))) } else { (if self.scalar_v1312 { v4596 } else { (if self.scalar_v497 { v4677 } else { v4 }) }) });
        let v4726: f64 = (if self.scalar_v1315 { (v487 * ((v1320 * v4265) + (v1299 * (self.scalar_v1309 * v3832)))) } else { v4688 });
        let v4727: f64 = (if self.scalar_v1315 { (v487 * ((v1320 * v4266) + (v1299 * (self.scalar_v1309 * v3833)))) } else { v4689 });
        let v4728: f64 = (v703 * v2476);
        let v4729: f64 = (v4728 / self.scalar_v489);
        let v4730: f64 = (v2985 / self.scalar_v489);
        let v4731: f64 = (v2984 / self.scalar_v489);
        let v4742: f64 = (if v1330 { (v1331 * v4729) } else { (if v1327 { (v1328 * v4729) } else { v4544 }) });
        let v4743: f64 = (if v1330 { (v1331 * v4730) } else { (if v1327 { (v1328 * v4730) } else { v4545 }) });
        let v4744: f64 = (if v1330 { (v1331 * v4731) } else { (if v1327 { (v1328 * v4731) } else { v4 }) });
        let v4745: f64 = (if v1330 { v4 } else { (if v1327 { v4 } else { v4546 }) });
        let v4747: f64 = (v4549 + (v1336 * v2476));
        let v4764: f64 = ((v1349 * ((v494 * (self.scalar_v488 * (v491 * (v2776 / self.scalar_v489)))) + (v492 * (v494 * (v2780 / self.scalar_v489))))) + (v495 * v4742));
        let v4765: f64 = (v495 * v4743);
        let v4766: f64 = (v495 * v4744);
        let v4767: f64 = (v495 * v4745);
        let v4779: f64 = (v31 * v1355);
        let v4786: f64 = ((v1356 * ((v1351 * v4742) + (v1349 * (v31 * (if self.scalar_v497 { (self.scalar_v513 * (v518 * ((self.scalar_v515 * v2476) / self.scalar_v489))) } else { v4 }))))) - (v1352 * ((v423 * (if v1343 { (v1344 * v4747) } else { (if v1339 { (v1340 * v4747) } else { v4560 }) })) / v4779)));
        let v4787: f64 = (v1356 * v1356);
        let v4792: f64 = (((v1356 * (v1351 * v4743)) - (v1352 * ((v423 * (if v1343 { (v1344 * v2985) } else { (if v1339 { (v1340 * v2985) } else { v4561 }) })) / v4779))) / v4787);
        let v4796: f64 = (((v1356 * (v1351 * v4744)) - (v1352 * ((v423 * (if v1343 { (v1344 * v2984) } else { (if v1339 { (v1340 * v2984) } else { v4 }) })) / v4779))) / v4787);
        let v4808: f64 = (if self.scalar_v497 { (v4767 + (((v1356 * (v1351 * v4745)) - (v1352 * ((v423 * (if v1343 { v4 } else { (if v1339 { v4 } else { v4562 }) })) / v4779))) / v4787)) } else { v4 });
        let v4813: f64 = (v2998 / self.scalar_v450);
        let v4814: f64 = (v2985 / self.scalar_v450);
        let v4815: f64 = (v2984 / self.scalar_v450);
        let v4826: f64 = (if v1365 { (v1366 * v4813) } else { (if v1362 { (v1363 * v4813) } else { v4742 }) });
        let v4827: f64 = (if v1365 { (v1366 * v4814) } else { (if v1362 { (v1363 * v4814) } else { v4743 }) });
        let v4828: f64 = (if v1365 { v4 } else { (if v1362 { v4 } else { v4744 }) });
        let v4829: f64 = (if v1365 { (v1366 * v4815) } else { (if v1362 { (v1363 * v4815) } else { v4745 }) });
        let v4834: f64 = (v461 * v4828);
        let v4836: f64 = (v4728 / self.scalar_v533);
        let v4837: f64 = (v2985 / self.scalar_v533);
        let v4838: f64 = (v2984 / self.scalar_v533);
        let v4849: f64 = (if v1377 { (v1378 * v4836) } else { (if v1374 { (v1375 * v4836) } else { v4826 }) });
        let v4850: f64 = (if v1377 { (v1378 * v4837) } else { (if v1374 { (v1375 * v4837) } else { v4827 }) });
        let v4851: f64 = (if v1377 { (v1378 * v4838) } else { (if v1374 { (v1375 * v4838) } else { v4828 }) });
        let v4852: f64 = (if v1377 { v4 } else { (if v1374 { v4 } else { v4829 }) });
        let v4859: f64 = (v3017 / self.scalar_v463);
        let v4860: f64 = (v2984 / self.scalar_v463);
        let v4861: f64 = (v3018 / self.scalar_v463);
        let v4862: f64 = (v3019 / self.scalar_v463);
        let v4863: f64 = (v2985 / self.scalar_v463);
        let v4880: f64 = (if v1389 { (v1390 * v4859) } else { (if v1386 { (v1387 * v4859) } else { v4849 }) });
        let v4881: f64 = (if v1389 { v4 } else { (if v1386 { v4 } else { v4850 }) });
        let v4882: f64 = (if v1389 { (v1390 * v4860) } else { (if v1386 { (v1387 * v4860) } else { v4851 }) });
        let v4883: f64 = (if v1389 { (v1390 * v4861) } else { (if v1386 { (v1387 * v4861) } else { v4852 }) });
        let v4884: f64 = (if v1389 { (v1390 * v4862) } else { (if v1386 { (v1387 * v4862) } else { v4 }) });
        let v4885: f64 = (if v1389 { (v1390 * v4863) } else { (if v1386 { (v1387 * v4863) } else { v4 }) });
        let v4888: f64 = ((v1395 * ((v472 * (self.scalar_v462 * (v467 * (self.scalar_v465 * v2477)))) + (v468 * (v472 * ((self.scalar_v469 * v2476) / self.scalar_v463))))) + (v473 * v4880));
        let v4894: f64 = (v4728 / self.scalar_v543);
        let v4895: f64 = (v2985 / self.scalar_v543);
        let v4896: f64 = (v2984 / self.scalar_v543);
        let v4909: f64 = (if v1401 { (v1402 * v4894) } else { (if v1398 { (v1399 * v4894) } else { v4880 }) });
        let v4910: f64 = (if v1401 { (v1402 * v4895) } else { (if v1398 { (v1399 * v4895) } else { v4881 }) });
        let v4911: f64 = (if v1401 { (v1402 * v4896) } else { (if v1398 { (v1399 * v4896) } else { v4882 }) });
        let v4912: f64 = (if v1401 { v4 } else { (if v1398 { v4 } else { v4883 }) });
        let v4913: f64 = (if v1401 { v4 } else { (if v1398 { v4 } else { v4884 }) });
        let v4914: f64 = (if v1401 { v4 } else { (if v1398 { v4 } else { v4885 }) });
        let v4921: f64 = (v550 * v4913);
        let v4922: f64 = (v550 * v4914);
        let v4928: f64 = (v1413 * v1413);
        let v4941: f64 = ((v1415 * v2863) + (v570 * (-((-(self.scalar_v34 * (v31 * v3972))) / v4928))));
        let v4942: f64 = (v570 * (-((-(self.scalar_v34 * (v31 * v3973))) / v4928)));
        let v4943: f64 = (v570 * (-((-(self.scalar_v34 * (v31 * v3974))) / v4928)));
        let v4959: f64 = (if v1412 { (v700 * v2640) } else { v2922 });
        let v4960: f64 = (if v1412 { (v284 * self.scalar_v2979) } else { v4 });
        let v4961: f64 = (if v1412 { (self.scalar_v0 * v284) } else { v4 });
        let v4962: f64 = (v1429 * v4959);
        let v4964: f64 = (v1429 * v4960);
        let v4966: f64 = (v1429 * v4961);
        let v4968: f64 = (v31 * v1433);
        let v4974: f64 = (self.scalar_v1435 * f64::powf(v1433, self.scalar_v4972));
        let v5015: f64 = (v1436 * ((self.scalar_v32 * (-(self.scalar_v1440 * (v170 * v4959)))) - ((v1446 * ((v1444 * v4959) + (v1429 * (v449 * v4959)))) + (v1445 * v4959))));
        let v5018: f64 = (v1436 * ((self.scalar_v32 * (-(self.scalar_v1440 * (v170 * v4960)))) - ((v1446 * ((v1444 * v4960) + (v1429 * (v449 * v4960)))) + (v1445 * v4960))));
        let v5021: f64 = (v1436 * ((self.scalar_v32 * (-(self.scalar_v1440 * (v170 * v4961)))) - ((v1446 * ((v1444 * v4961) + (v1429 * (v449 * v4961)))) + (v1445 * v4961))));
        let v5041: f64 = ((v1455 * (v1453 * v2863)) - (v1454 * ((v1452 * v2498) + (v147 * (if v1412 { (v1450 * ((v1448 * (((v4962 + v4962) / v4968) * v4974)) + v5015)) } else { v4 })))));
        let v5042: f64 = (v1455 * v1455);
        let v5046: f64 = ((v1455 * (v570 * self.scalar_v5029)) - (v1454 * (v147 * (if v1412 { (v1450 * ((v1448 * (((v4964 + v4964) / v4968) * v4974)) + v5018)) } else { v4 }))));
        let v5050: f64 = ((v1455 * (v570 * self.scalar_v5030)) - (v1454 * (v147 * (if v1412 { (v1450 * ((v1448 * (((v4966 + v4966) / v4968) * v4974)) + v5021)) } else { v4 }))));
        let v5052: f64 = (if v1412 { (v5041 / v5042) } else { v4959 });
        let v5053: f64 = (if v1412 { (v5046 / v5042) } else { v4960 });
        let v5054: f64 = (if v1412 { (v5050 / v5042) } else { v4961 });
        let v5073: f64 = (v1457 * v1457);
        let v5086: f64 = ((self.scalar_v0 * v1475) + (v1472 * (((v1457 * (-(if v1466 { (v1467 * v5053) } else { (if v1462 { (v1463 * v5053) } else { v4 }) }))) - (v1473 * v5053)) / v5073)));
        let v5089: f64 = ((v1475 * self.scalar_v2979) + (v1472 * (((v1457 * (-(if v1466 { (v1467 * v5054) } else { (if v1462 { (v1463 * v5054) } else { v4 }) }))) - (v1473 * v5054)) / v5073)));
        let v5090: f64 = (if v1461 { (v1472 * (((v1457 * (-(if v1466 { (v1467 * v5052) } else { (if v1462 { (v1463 * v5052) } else { v4 }) }))) - (v1473 * v5052)) / v5073)) } else { v4 });
        let v5127: f64 = (if v1479 { ((v1488 * ((v1480 * v5053) + (v1457 * self.scalar_v5093))) + (v1481 * ((v1486 * (v1482 * v5053)) + (v1483 * (v1484 * v5053))))) } else { (if v1461 { v5086 } else { v4 }) });
        let v5128: f64 = (if v1479 { ((v1488 * ((v1480 * v5054) + (v1457 * self.scalar_v5094))) + (v1481 * ((v1486 * (v1482 * v5054)) + (v1483 * (v1484 * v5054))))) } else { (if v1461 { v5089 } else { v4 }) });
        let v5132: f64 = ((v1491 * (if v1479 { ((v1488 * (v1480 * v5052)) + (v1481 * ((v1486 * (v1482 * v5052)) + (v1483 * (v1484 * v5052))))) } else { v5090 })) + (v1490 * (v31 * ((v579 * v2875) + (v577 * (v579 * (-v2863)))))));
        let v5149: f64 = ((v1493 * (if v1422 { (v1423 * v4942) } else { (if v1418 { (v1419 * v4942) } else { v4 }) })) + (v1427 * ((v1492 * v3973) + (v1080 * (v1491 * v5127)))));
        let v5152: f64 = ((v1493 * (if v1422 { (v1423 * v4943) } else { (if v1418 { (v1419 * v4943) } else { v4 }) })) + (v1427 * ((v1492 * v3974) + (v1080 * (v1491 * v5128)))));
        let v5154: f64 = (v284 * ((v1493 * (if v1422 { (v1423 * v4941) } else { (if v1418 { (v1419 * v4941) } else { v4 }) })) + (v1427 * ((v1492 * v3972) + (v1080 * v5132)))));
        let v5167: f64 = (v694 * v2643);
        let v5168: f64 = (self.scalar_v0 * v285);
        let v5169: f64 = (v285 * self.scalar_v2979);
        let v5174: f64 = (self.scalar_v1124 * f64::powf(v1506, self.scalar_v4116));
        let v5178: f64 = (if v1504 { ((-v5167) * v5174) } else { v4 });
        let v5179: f64 = (if v1504 { ((-v5168) * v5174) } else { v4 });
        let v5180: f64 = (if v1504 { ((-v5169) * v5174) } else { v4 });
        let v5186: f64 = (v1509 * v1509);
        let v5199: f64 = ((v1511 * v2903) + (v592 * (-((-(self.scalar_v69 * (v31 * v5178))) / v5186))));
        let v5200: f64 = (v592 * (-((-(self.scalar_v69 * (v31 * v5179))) / v5186)));
        let v5201: f64 = (v592 * (-((-(self.scalar_v69 * (v31 * v5180))) / v5186)));
        let v5214: f64 = (if v1504 { v5167 } else { v2884 });
        let v5215: f64 = (if v1504 { v5168 } else { v4 });
        let v5216: f64 = (if v1504 { v5169 } else { v4 });
        let v5217: f64 = (v1524 * v5214);
        let v5219: f64 = (v1524 * v5215);
        let v5221: f64 = (v1524 * v5216);
        let v5223: f64 = (v31 * v1527);
        let v5229: f64 = (self.scalar_v1528 * f64::powf(v1527, self.scalar_v5227));
        let v5270: f64 = (v1529 * ((self.scalar_v67 * (-(self.scalar_v1533 * (v170 * v5214)))) - ((v1539 * ((v1537 * v5214) + (v1524 * (v449 * v5214)))) + (v1538 * v5214))));
        let v5273: f64 = (v1529 * ((self.scalar_v67 * (-(self.scalar_v1533 * (v170 * v5215)))) - ((v1539 * ((v1537 * v5215) + (v1524 * (v449 * v5215)))) + (v1538 * v5215))));
        let v5276: f64 = (v1529 * ((self.scalar_v67 * (-(self.scalar_v1533 * (v170 * v5216)))) - ((v1539 * ((v1537 * v5216) + (v1524 * (v449 * v5216)))) + (v1538 * v5216))));
        let v5296: f64 = ((v1547 * (v1545 * v2903)) - (v1546 * ((v1544 * v2519) + (v169 * (if v1504 { (v1450 * ((v1541 * (((v5217 + v5217) / v5223) * v5229)) + v5270)) } else { v4 })))));
        let v5297: f64 = (v1547 * v1547);
        let v5301: f64 = ((v1547 * (v592 * self.scalar_v5284)) - (v1546 * (v169 * (if v1504 { (v1450 * ((v1541 * (((v5219 + v5219) / v5223) * v5229)) + v5273)) } else { v4 }))));
        let v5305: f64 = ((v1547 * (v592 * self.scalar_v5285)) - (v1546 * (v169 * (if v1504 { (v1450 * ((v1541 * (((v5221 + v5221) / v5223) * v5229)) + v5276)) } else { v4 }))));
        let v5307: f64 = (if v1504 { (v5296 / v5297) } else { v5214 });
        let v5308: f64 = (if v1504 { (v5301 / v5297) } else { v5215 });
        let v5309: f64 = (if v1504 { (v5305 / v5297) } else { v5216 });
        let v5328: f64 = (v1549 * v1549);
        let v5341: f64 = ((v1566 * self.scalar_v2979) + (v1563 * (((v1549 * (-(if v1557 { (v1558 * v5308) } else { (if v1553 { (v1554 * v5308) } else { v4 }) }))) - (v1564 * v5308)) / v5328)));
        let v5344: f64 = ((self.scalar_v0 * v1566) + (v1563 * (((v1549 * (-(if v1557 { (v1558 * v5309) } else { (if v1553 { (v1554 * v5309) } else { v4 }) }))) - (v1564 * v5309)) / v5328)));
        let v5345: f64 = (if v1552 { (v1563 * (((v1549 * (-(if v1557 { (v1558 * v5307) } else { (if v1553 { (v1554 * v5307) } else { v4 }) }))) - (v1564 * v5307)) / v5328)) } else { v4 });
        let v5380: f64 = (if v1570 { ((v1577 * ((v1571 * v5308) + (v1549 * self.scalar_v5094))) + (v1572 * ((v1575 * (v1482 * v5308)) + (v1573 * (v1484 * v5308))))) } else { (if v1552 { v5341 } else { v4 }) });
        let v5381: f64 = (if v1570 { ((v1577 * ((v1571 * v5309) + (v1549 * self.scalar_v5093))) + (v1572 * ((v1575 * (v1482 * v5309)) + (v1573 * (v1484 * v5309))))) } else { (if v1552 { v5344 } else { v4 }) });
        let v5385: f64 = ((v1580 * (if v1570 { ((v1577 * (v1571 * v5307)) + (v1572 * ((v1575 * (v1482 * v5307)) + (v1573 * (v1484 * v5307))))) } else { v5345 })) + (v1579 * (v31 * ((v601 * v2915) + (v599 * (v601 * (-v2903)))))));
        let v5402: f64 = ((v1582 * (if v1518 { (v1519 * v5200) } else { (if v1514 { (v1515 * v5200) } else { v4 }) })) + (v1523 * ((v1581 * v5179) + (v1508 * (v1580 * v5380)))));
        let v5405: f64 = ((v1582 * (if v1518 { (v1519 * v5201) } else { (if v1514 { (v1515 * v5201) } else { v4 }) })) + (v1523 * ((v1581 * v5180) + (v1508 * (v1580 * v5381)))));
        let v5407: f64 = (v285 * ((v1582 * (if v1518 { (v1519 * v5199) } else { (if v1514 { (v1515 * v5199) } else { v4 }) })) + (v1523 * ((v1581 * v5178) + (v1508 * v5385)))));
        let v5422: f64 = ((v1140 * v3035) + (v765 * v4188));
        let v5423: f64 = (v1140 * v3036);
        let v5424: f64 = (v1140 * v3037);
        let v5425: f64 = (v1140 * v3038);
        let v5426: f64 = (v1140 * v3039);
        let v5427: f64 = (v423 * (if v802 { (v803 * v3100) } else { (if v799 { (v800 * v3100) } else { v4 }) }));
        let v5428: f64 = (v423 * (if v802 { (v803 * v2984) } else { (if v799 { (v800 * v2984) } else { v4 }) }));
        let v5429: f64 = (v423 * (if v802 { (v803 * v3018) } else { (if v799 { (v800 * v3018) } else { v4 }) }));
        let v5430: f64 = (v423 * (if v802 { (v803 * v3019) } else { (if v799 { (v800 * v3019) } else { v4 }) }));
        let v5431: f64 = (v423 * (if v802 { (v803 * v2985) } else { (if v799 { (v800 * v2985) } else { v4 }) }));
        let v5433: f64 = (v31 * v1593);
        let v5442: f64 = (v1594 * v1594);
        let v5460: f64 = (v31 * v1597);
        let v5469: f64 = (v1598 * v1598);
        let v5487: f64 = (v31 * v2815);
        let v5500: f64 = (((v447 * (v423 * v2815)) - (v1603 * (self.scalar_v442 * (v446 * (self.scalar_v444 * v2477))))) / (v447 * v447));
        let v5508: f64 = (v31 * v1607);
        let v5517: f64 = (v1608 * v1608);
        let v5518: f64 = (((v1608 * ((v1601 * v5487) + (v1600 * v3035))) - (v1602 * (((v1604 * v3035) + (v765 * v5500)) / v5508))) / v5517);
        let v5522: f64 = (((v1608 * (v1600 * v3036)) - (v1602 * ((v1604 * v3036) / v5508))) / v5517);
        let v5526: f64 = (((v1608 * (v1600 * v3037)) - (v1602 * ((v1604 * v3037) / v5508))) / v5517);
        let v5530: f64 = (((v1608 * (v1600 * v3038)) - (v1602 * ((v1604 * v3038) / v5508))) / v5517);
        let v5534: f64 = (((v1608 * (v1600 * v3039)) - (v1602 * ((v1604 * v3039) / v5508))) / v5517);
        let v5545: f64 = (self.scalar_v1616 * v2815);
        let v5560: f64 = (v31 * v1622);
        let v5569: f64 = (v1623 * v1623);
        let v5587: f64 = (if self.scalar_v1613 { (((v1623 * (v1617 * v3070)) - (v1619 * ((v1604 * v3070) / v5560))) / v5569) } else { v4 });
        let v5588: f64 = (if self.scalar_v1613 { (((v1623 * (v1617 * v3071)) - (v1619 * ((v1604 * v3071) / v5560))) / v5569) } else { v4 });
        let v5589: f64 = (if self.scalar_v1613 { (((v1623 * ((v1618 * v5545) + (v1617 * v3072))) - (v1619 * (((v1604 * v3072) + (v785 * v5500)) / v5560))) / v5569) } else { v4 });
        let v5590: f64 = (if self.scalar_v1613 { (((v1623 * (v1617 * v3073)) - (v1619 * ((v1604 * v3073) / v5560))) / v5569) } else { v4 });
        let v5591: f64 = (if self.scalar_v1613 { (((v1623 * (v1617 * v3074)) - (v1619 * ((v1604 * v3074) / v5560))) / v5569) } else { v4 });
        let v5596: f64 = (if self.scalar_v1627 { ((v1628 * v2678) + (v327 * (self.scalar_v13 * v2815))) } else { v4 });
        let v5609: f64 = (if self.scalar_v1627 { (-(if self.scalar_v1627 { ((v1633 * v2473) + (v119 * (-(((v1630 * v2476) + (v121 * v5596)) / v1631)))) } else { v4 })) } else { v4 });
        let v5612: f64 = (v1637 * self.scalar_v5607);
        let v5613: f64 = (v5612 + v5612);
        let v5614: f64 = (v1637 * self.scalar_v5608);
        let v5616: f64 = (v1637 * v5609);
        let v5618: f64 = (v1637 * self.scalar_v5610);
        let v5619: f64 = (v5618 + v5618);
        let v5620: f64 = (v1637 * self.scalar_v5611);
        let v5622: f64 = (if self.scalar_v1627 { v5613 } else { v4 });
        let v5623: f64 = (if self.scalar_v1627 { (v5614 + v5614) } else { v4 });
        let v5624: f64 = (if self.scalar_v1627 { (v5616 + v5616) } else { v4344 });
        let v5625: f64 = (if self.scalar_v1627 { v4 } else { v4346 });
        let v5626: f64 = (if self.scalar_v1627 { v5613 } else { v4348 });
        let v5627: f64 = (if self.scalar_v1627 { v5619 } else { v4350 });
        let v5628: f64 = (if self.scalar_v1627 { v5619 } else { v4352 });
        let v5629: f64 = (if self.scalar_v1627 { (v5620 + v5620) } else { v4 });
        let v5630: f64 = (if self.scalar_v1627 { v5619 } else { v4 });
        let v5631: f64 = (v31 * v1646);
        let v5632: f64 = (v5622 / v5631);
        let v5633: f64 = (v5623 / v5631);
        let v5634: f64 = (v5624 / v5631);
        let v5635: f64 = (v5625 / v5631);
        let v5636: f64 = (v5626 / v5631);
        let v5637: f64 = (v5627 / v5631);
        let v5638: f64 = (v5628 / v5631);
        let v5639: f64 = (v5629 / v5631);
        let v5640: f64 = (v5630 / v5631);
        let v5651: f64 = (v1647 * v1647);
        let v5703: f64 = (if v1651 { (v411 * (self.scalar_v5607 + v5632)) } else { (if v1643 { ((-(self.scalar_v1644 * (v5632 - self.scalar_v5607))) / v5651) } else { v4 }) });
        let v5704: f64 = (if v1651 { (v411 * (self.scalar_v5608 + v5633)) } else { (if v1643 { ((-(self.scalar_v1644 * (v5633 - self.scalar_v5608))) / v5651) } else { v4 }) });
        let v5705: f64 = (if v1651 { (v411 * (v5609 + v5634)) } else { (if v1643 { ((-(self.scalar_v1644 * (v5634 - v5609))) / v5651) } else { v4 }) });
        let v5706: f64 = (if v1651 { (v411 * v5635) } else { (if v1643 { ((-(self.scalar_v1644 * v5635)) / v5651) } else { v4 }) });
        let v5707: f64 = (if v1651 { (v411 * (self.scalar_v5607 + v5636)) } else { (if v1643 { ((-(self.scalar_v1644 * (v5636 - self.scalar_v5607))) / v5651) } else { v4 }) });
        let v5708: f64 = (if v1651 { (v411 * (self.scalar_v5610 + v5637)) } else { (if v1643 { ((-(self.scalar_v1644 * (v5637 - self.scalar_v5610))) / v5651) } else { v4 }) });
        let v5709: f64 = (if v1651 { (v411 * (self.scalar_v5610 + v5638)) } else { (if v1643 { ((-(self.scalar_v1644 * (v5638 - self.scalar_v5610))) / v5651) } else { v4 }) });
        let v5710: f64 = (if v1651 { (v411 * (self.scalar_v5611 + v5639)) } else { (if v1643 { ((-(self.scalar_v1644 * (v5639 - self.scalar_v5611))) / v5651) } else { v4 }) });
        let v5711: f64 = (if v1651 { (v411 * (self.scalar_v5610 + v5640)) } else { (if v1643 { ((-(self.scalar_v1644 * (v5640 - self.scalar_v5610))) / v5651) } else { v4 }) });
        let v5712: f64 = (v327 * v5587);
        let v5717: f64 = (v327 * v5590);
        let v5731: f64 = (v1657 * v1657);
        let v5774: f64 = (if self.scalar_v1661 { v4 } else { (if self.scalar_v1627 { (((v1657 * v5703) - (v1654 * (v5703 + v5712))) / v5731) } else { v4 }) });
        let v5775: f64 = (if self.scalar_v1661 { v4 } else { (if self.scalar_v1627 { (((v1657 * v5704) - (v1654 * (v5704 + (v327 * v5588)))) / v5731) } else { v4 }) });
        let v5776: f64 = (if self.scalar_v1661 { v4 } else { (if self.scalar_v1627 { (((v1657 * v5705) - (v1654 * (v5705 + (v5596 + ((v1625 * v2678) + (v327 * v5589)))))) / v5731) } else { v4 }) });
        let v5777: f64 = (if self.scalar_v1661 { v4 } else { (if self.scalar_v1627 { (((v1657 * v5706) - (v1654 * v5706)) / v5731) } else { v4 }) });
        let v5778: f64 = (if self.scalar_v1661 { v4 } else { (if self.scalar_v1627 { (((v1657 * v5707) - (v1654 * (v5707 + v5712))) / v5731) } else { v4 }) });
        let v5779: f64 = (if self.scalar_v1661 { v4 } else { (if self.scalar_v1627 { (((v1657 * v5708) - (v1654 * (v5708 + v5717))) / v5731) } else { v4 }) });
        let v5780: f64 = (if self.scalar_v1661 { v4 } else { (if self.scalar_v1627 { (((v1657 * v5709) - (v1654 * (v5709 + v5717))) / v5731) } else { v4 }) });
        let v5781: f64 = (if self.scalar_v1661 { v4 } else { (if self.scalar_v1627 { (((v1657 * v5710) - (v1654 * (v5710 + (v327 * v5591)))) / v5731) } else { v4 }) });
        let v5782: f64 = (if self.scalar_v1661 { v4 } else { (if self.scalar_v1627 { (((v1657 * v5711) - (v1654 * (v5711 + v5717))) / v5731) } else { v4 }) });
        let v5783: f64 = (v1662 * v5587);
        let v5795: f64 = (v1662 * v5590);
        let v5820: f64 = (v1668 * self.scalar_v5814);
        let v5822: f64 = (v1668 * self.scalar_v5815);
        let v5824: f64 = (v1668 * self.scalar_v5816);
        let v5836: f64 = (v31 * v1677);
        let v5837: f64 = ((if self.scalar_v1666 { v4 } else { v5622 }) / v5836);
        let v5838: f64 = ((if self.scalar_v1666 { v4 } else { v5623 }) / v5836);
        let v5839: f64 = ((if self.scalar_v1666 { v4 } else { v5624 }) / v5836);
        let v5840: f64 = ((if self.scalar_v1666 { v4 } else { v5625 }) / v5836);
        let v5841: f64 = ((if self.scalar_v1666 { (v5820 + v5820) } else { v5622 }) / v5836);
        let v5842: f64 = ((if self.scalar_v1666 { (v5822 + v5822) } else { v5626 }) / v5836);
        let v5843: f64 = ((if self.scalar_v1666 { (v5824 + v5824) } else { v5627 }) / v5836);
        let v5844: f64 = ((if self.scalar_v1666 { v4 } else { v5628 }) / v5836);
        let v5845: f64 = ((if self.scalar_v1666 { v4 } else { v5629 }) / v5836);
        let v5846: f64 = ((if self.scalar_v1666 { v4 } else { v5630 }) / v5836);
        let v5852: f64 = (v1678 * v1678);
        let v5904: f64 = (if v1682 { (v411 * v5837) } else { (if v1674 { ((-(self.scalar_v1675 * v5837)) / v5852) } else { v4 }) });
        let v5905: f64 = (if v1682 { (v411 * v5838) } else { (if v1674 { ((-(self.scalar_v1675 * v5838)) / v5852) } else { v4 }) });
        let v5906: f64 = (if v1682 { (v411 * v5839) } else { (if v1674 { ((-(self.scalar_v1675 * v5839)) / v5852) } else { v4 }) });
        let v5907: f64 = (if v1682 { (v411 * v5840) } else { (if v1674 { ((-(self.scalar_v1675 * v5840)) / v5852) } else { v4 }) });
        let v5908: f64 = (if v1682 { (v411 * (self.scalar_v5817 + v5841)) } else { (if v1674 { ((-(self.scalar_v1675 * (v5841 - self.scalar_v5817))) / v5852) } else { v4 }) });
        let v5909: f64 = (if v1682 { (v411 * (self.scalar_v5818 + v5842)) } else { (if v1674 { ((-(self.scalar_v1675 * (v5842 - self.scalar_v5818))) / v5852) } else { v4 }) });
        let v5910: f64 = (if v1682 { (v411 * (self.scalar_v5819 + v5843)) } else { (if v1674 { ((-(self.scalar_v1675 * (v5843 - self.scalar_v5819))) / v5852) } else { v4 }) });
        let v5911: f64 = (if v1682 { (v411 * v5844) } else { (if v1674 { ((-(self.scalar_v1675 * v5844)) / v5852) } else { v4 }) });
        let v5912: f64 = (if v1682 { (v411 * v5845) } else { (if v1674 { ((-(self.scalar_v1675 * v5845)) / v5852) } else { v4 }) });
        let v5913: f64 = (if v1682 { (v411 * v5846) } else { (if v1674 { ((-(self.scalar_v1675 * v5846)) / v5852) } else { v4 }) });
        let v5925: f64 = (self.scalar_v1686 * f64::powf(v1703, self.scalar_v1695));
        let v5936: f64 = (v1705 * v1705);
        let v5977: f64 = (if self.scalar_v1714 { v4 } else { (if v1709 { (self.scalar_v1700 * v5904) } else { (if v1702 { (((v5904 / self.scalar_v1691) * v5925) / v5936) } else { v4 }) }) });
        let v5978: f64 = (if self.scalar_v1714 { v4 } else { (if v1709 { (self.scalar_v1700 * v5905) } else { (if v1702 { (((v5905 / self.scalar_v1691) * v5925) / v5936) } else { v4 }) }) });
        let v5979: f64 = (if self.scalar_v1714 { v4 } else { (if v1709 { (self.scalar_v1700 * v5906) } else { (if v1702 { (((v5906 / self.scalar_v1691) * v5925) / v5936) } else { v4 }) }) });
        let v5980: f64 = (if self.scalar_v1714 { v4 } else { (if v1709 { (self.scalar_v1700 * v5907) } else { (if v1702 { (((v5907 / self.scalar_v1691) * v5925) / v5936) } else { v4 }) }) });
        let v5981: f64 = (if self.scalar_v1714 { v4 } else { (if v1709 { (self.scalar_v1700 * v5908) } else { (if v1702 { (((v5908 / self.scalar_v1691) * v5925) / v5936) } else { v4 }) }) });
        let v5982: f64 = (if self.scalar_v1714 { v4 } else { (if v1709 { (self.scalar_v1700 * v5909) } else { (if v1702 { (((v5909 / self.scalar_v1691) * v5925) / v5936) } else { v4 }) }) });
        let v5983: f64 = (if self.scalar_v1714 { v4 } else { (if v1709 { (self.scalar_v1700 * v5910) } else { (if v1702 { (((v5910 / self.scalar_v1691) * v5925) / v5936) } else { v4 }) }) });
        let v5984: f64 = (if self.scalar_v1714 { v4 } else { (if v1709 { (self.scalar_v1700 * v5911) } else { (if v1702 { (((v5911 / self.scalar_v1691) * v5925) / v5936) } else { v4 }) }) });
        let v5985: f64 = (if self.scalar_v1714 { v4 } else { (if v1709 { (self.scalar_v1700 * v5912) } else { (if v1702 { (((v5912 / self.scalar_v1691) * v5925) / v5936) } else { v4 }) }) });
        let v5986: f64 = (if self.scalar_v1714 { v4 } else { (if v1709 { (self.scalar_v1700 * v5913) } else { (if v1702 { (((v5913 / self.scalar_v1691) * v5925) / v5936) } else { v4 }) }) });
        let v5987: f64 = (v1588 * v5977);
        let v5988: f64 = (v1588 * v5978);
        let v5991: f64 = ((v1715 * (if v1587 { v4 } else { (if v1504 { (self.scalar_v70 * ((v1583 * v2643) + v5407)) } else { v4 }) })) + (v1588 * v5979));
        let v5992: f64 = (v1588 * v5980);
        let v5993: f64 = (v1588 * v5981);
        let v5996: f64 = ((v1715 * (if v1587 { v4 } else { (if v1504 { (self.scalar_v70 * (v285 * v5402)) } else { v4 }) })) + (v1588 * v5982));
        let v5999: f64 = ((v1715 * (if v1587 { v4 } else { (if v1504 { (self.scalar_v70 * (v285 * v5405)) } else { v4 }) })) + (v1588 * v5983));
        let v6000: f64 = (v1588 * v5984);
        let v6001: f64 = (v1588 * v5985);
        let v6002: f64 = (v1588 * v5986);
        let v6011: f64 = ((v1715 * (if self.scalar_v1613 { (self.scalar_v14 * v5522) } else { v5522 })) + (v1615 * v5981));
        let v6014: f64 = ((v1715 * (if self.scalar_v1613 { (self.scalar_v14 * v5526) } else { v5526 })) + (v1615 * v5982));
        let v6015: f64 = (v1715 * (if self.scalar_v1613 { (self.scalar_v14 * v5530) } else { v5530 }));
        let v6017: f64 = (v6015 + (v1615 * v5983));
        let v6019: f64 = (v6015 + (v1615 * v5984));
        let v6023: f64 = ((v1715 * (if self.scalar_v1613 { (self.scalar_v14 * v5534) } else { v5534 })) + (v1615 * v5986));
        let v6034: f64 = ((v1715 * (v473 * v4882)) + (v1396 * v5981));
        let v6037: f64 = ((v1715 * (v473 * v4883)) + (v1396 * v5982));
        let v6038: f64 = (v1715 * (v473 * v4884));
        let v6040: f64 = (v6038 + (v1396 * v5983));
        let v6042: f64 = (v6038 + (v1396 * v5984));
        let v6046: f64 = ((v1715 * (v473 * v4885)) + (v1396 * v5986));
        let v6047: f64 = (v1715 * (if self.scalar_v1613 { (v5783 + (v1625 * v5774)) } else { v4 }));
        let v6049: f64 = (v6047 + (v1664 * v5977));
        let v6052: f64 = ((v1715 * (if self.scalar_v1613 { ((v1662 * v5588) + (v1625 * v5775)) } else { v4 })) + (v1664 * v5978));
        let v6055: f64 = ((v1715 * (if self.scalar_v1613 { ((v1662 * v5589) + (v1625 * v5776)) } else { v4 })) + (v1664 * v5979));
        let v6058: f64 = ((v1715 * (if self.scalar_v1613 { (v1625 * v5777) } else { v4 })) + (v1664 * v5980));
        let v6060: f64 = (v6047 + (v1664 * v5981));
        let v6063: f64 = ((v1715 * (if self.scalar_v1613 { (v5783 + (v1625 * v5778)) } else { v4 })) + (v1664 * v5982));
        let v6066: f64 = ((v1715 * (if self.scalar_v1613 { (v5795 + (v1625 * v5779)) } else { v4 })) + (v1664 * v5983));
        let v6069: f64 = ((v1715 * (if self.scalar_v1613 { (v5795 + (v1625 * v5780)) } else { v4 })) + (v1664 * v5984));
        let v6072: f64 = ((v1715 * (if self.scalar_v1613 { ((v1662 * v5591) + (v1625 * v5781)) } else { v4 })) + (v1664 * v5985));
        let v6075: f64 = ((v1715 * (if self.scalar_v1613 { (v5795 + (v1625 * v5782)) } else { v4 })) + (v1664 * v5986));
        let v6076: f64 = (v1157 * v4267);
        let v6078: f64 = (v1157 * v4257);
        let v6080: f64 = (v1157 * v4268);
        let v6082: f64 = (v1157 * v4265);
        let v6084: f64 = (v1157 * v4266);
        let v6086: f64 = (v31 * v1723);
        let v6087: f64 = ((v6076 + v6076) / v6086);
        let v6088: f64 = ((v6078 + v6078) / v6086);
        let v6089: f64 = ((v6080 + v6080) / v6086);
        let v6090: f64 = ((v6082 + v6082) / v6086);
        let v6091: f64 = ((v6084 + v6084) / v6086);
        let v6099: f64 = (v1724 * v1724);
        let v6128: f64 = (if v1727 { (v411 * (v4267 + v6087)) } else { (if v1721 { ((-(v1179 * (v6087 - v4267))) / v6099) } else { v4 }) });
        let v6129: f64 = (if v1727 { (v411 * (v4257 + v6088)) } else { (if v1721 { ((-(v1179 * (v6088 - v4257))) / v6099) } else { v4 }) });
        let v6130: f64 = (if v1727 { (v411 * (v4268 + v6089)) } else { (if v1721 { ((-(v1179 * (v6089 - v4268))) / v6099) } else { v4 }) });
        let v6131: f64 = (if v1727 { (v411 * (v4265 + v6090)) } else { (if v1721 { ((-(v1179 * (v6090 - v4265))) / v6099) } else { v4 }) });
        let v6132: f64 = (if v1727 { (v411 * (v4266 + v6091)) } else { (if v1721 { ((-(v1179 * (v6091 - v4266))) / v6099) } else { v4 }) });
        let v6151: f64 = (v1731 * v1731);
        let v6170: f64 = (v170 * (if v1733 { v4 } else { (((v1731 * (self.scalar_v309 * (v314 * (self.scalar_v312 * v2477)))) - (v315 * ((v1730 * v4402) + (v1191 * v6128)))) / v6151) }));
        let v6171: f64 = (v170 * (if v1733 { v4 } else { ((-(v315 * ((v1730 * v4403) + (v1191 * v6129)))) / v6151) }));
        let v6172: f64 = (v170 * (if v1733 { v4 } else { ((-(v315 * ((v1730 * v4404) + (v1191 * v6130)))) / v6151) }));
        let v6173: f64 = (v170 * (if v1733 { v4 } else { ((-(v315 * ((v1730 * v4405) + (v1191 * v6131)))) / v6151) }));
        let v6174: f64 = (v170 * (if v1733 { v4 } else { ((-(v315 * ((v1730 * v4406) + (v1191 * v6132)))) / v6151) }));
        let v6185: f64 = (v1735 * v1735);
        let v6186: f64 = (((v1735 * ((v1736 * v3227) + (v862 * (if v770 { (v771 * v3040) } else { (if v767 { (v768 * v3040) } else { v4 }) })))) - (v1738 * v6170)) / v6185);
        let v6189: f64 = ((-(v1738 * v6171)) / v6185);
        let v6190: f64 = ((self.scalar_v0 + (v862 * (if v770 { (v771 * v2984) } else { (if v767 { (v768 * v2984) } else { v4 }) }))) / v1735);
        let v6194: f64 = (((v1735 * (self.scalar_v2979 + (v862 * (if v770 { (v771 * v2985) } else { (if v767 { (v768 * v2985) } else { v4 }) })))) - (v1738 * v6172)) / v6185);
        let v6197: f64 = ((-(v1738 * v6173)) / v6185);
        let v6200: f64 = ((-(v1738 * v6174)) / v6185);
        let v6206: f64 = ((-v4442) / self.scalar_v1746);
        let v6207: f64 = ((-v4446) / self.scalar_v1746);
        let v6208: f64 = ((-v4450) / self.scalar_v1746);
        let v6209: f64 = ((-v4454) / self.scalar_v1746);
        let v6210: f64 = ((-v4458) / self.scalar_v1746);
        let v6240: f64 = (if v1750 { (v1761 * (if v1755 { (v1756 * v6206) } else { (if v1751 { (v1752 * v6206) } else { v4 }) })) } else { v4 });
        let v6241: f64 = (if v1750 { (v1761 * (if v1755 { (v1756 * v6207) } else { (if v1751 { (v1752 * v6207) } else { v4 }) })) } else { v4 });
        let v6242: f64 = (if v1750 { ((v1761 * (if v1755 { (v1756 * v6208) } else { (if v1751 { (v1752 * v6208) } else { v4 }) })) + (v1760 * self.scalar_v2979)) } else { v4 });
        let v6243: f64 = (if v1750 { ((v1761 * (if v1755 { (v1756 * v6209) } else { (if v1751 { (v1752 * v6209) } else { v4 }) })) + (self.scalar_v0 * v1760)) } else { v4 });
        let v6244: f64 = (if v1750 { (v1761 * (if v1755 { (v1756 * v6210) } else { (if v1751 { (v1752 * v6210) } else { v4 }) })) } else { v4 });
        let v6245: f64 = (-v2734);
        let v6248: f64 = (self.scalar_v1765 * f64::powf(v1763, self.scalar_v6246));
        let v6256: f64 = ((v1766 * v6245) + (v1764 * (v6240 * v6248)));
        let v6257: f64 = (v1764 * (v6241 * v6248));
        let v6258: f64 = (v1764 * (v6242 * v6248));
        let v6259: f64 = (v1764 * (v6243 * v6248));
        let v6260: f64 = (v1764 * (v6244 * v6248));
        let v6276: f64 = (if v1773 { (v1774 * v6256) } else { (if v1769 { (v1770 * v6256) } else { v4 }) });
        let v6277: f64 = (if v1773 { (v1774 * v6257) } else { (if v1769 { (v1770 * v6257) } else { v4 }) });
        let v6278: f64 = (if v1773 { (v1774 * v6258) } else { (if v1769 { (v1770 * v6258) } else { v4 }) });
        let v6279: f64 = (if v1773 { (v1774 * v6259) } else { (if v1769 { (v1770 * v6259) } else { v4 }) });
        let v6280: f64 = (if v1773 { (v1774 * v6260) } else { (if v1769 { (v1770 * v6260) } else { v4 }) });
        let v6284: f64 = ((-(self.scalar_v1779 * v2734)) / (v421 * v421));
        let v6315: f64 = (v1054 * v1054);
        let v6328: f64 = (if v1789 { (((v1054 * v2570) - (v1796 * v3910)) / v6315) } else { v3619 });
        let v6329: f64 = (if v1789 { (((v1054 * self.scalar_v2979) - (v1796 * v3911)) / v6315) } else { v3620 });
        let v6330: f64 = (if v1789 { (((self.scalar_v0 * v1054) - (v1796 * v3912)) / v6315) } else { v3621 });
        let v6331: f64 = (if v1789 { ((-(v1796 * v3913)) / v6315) } else { v3622 });
        let v6340: f64 = (v31 * v1801);
        let v6345: f64 = (if v1789 { (((v31 * v6328) / v1795) / v6340) } else { v4 });
        let v6346: f64 = (if v1789 { (((v31 * v6329) / v1795) / v6340) } else { v4 });
        let v6347: f64 = (if v1789 { (((v31 * v6330) / v1795) / v6340) } else { v4 });
        let v6348: f64 = (if v1789 { (((v31 * v6331) / v1795) / v6340) } else { v4 });
        let v6357: f64 = (if v1808 { (-(v411 * v3886)) } else { v4 });
        let v6358: f64 = (if v1808 { (-(v411 * v3887)) } else { v4 });
        let v6359: f64 = (if v1808 { (-(v411 * v3888)) } else { v4 });
        let v6360: f64 = (if v1808 { (-(v411 * v3889)) } else { v4 });
        let v6377: f64 = (if v1808 { ((v1812 * v6357) + (v1811 * (self.scalar_v1792 * v6357))) } else { v4 });
        let v6378: f64 = (if v1808 { ((v1812 * v6358) + (v1811 * (self.scalar_v1792 * v6358))) } else { v4 });
        let v6379: f64 = (if v1808 { ((v1812 * v6359) + (v1811 * (self.scalar_v1792 * v6359))) } else { v4 });
        let v6380: f64 = (if v1808 { ((v1812 * v6360) + (v1811 * (self.scalar_v1792 * v6360))) } else { v4 });
        let v6393: f64 = (v1802 * v6345);
        let v6395: f64 = (v1802 * v6346);
        let v6397: f64 = (v1802 * v6347);
        let v6399: f64 = (v1802 * v6348);
        let v6401: f64 = (v1814 * v6377);
        let v6403: f64 = (v1814 * v6378);
        let v6405: f64 = (v1814 * v6379);
        let v6407: f64 = (v1814 * v6380);
        let v6413: f64 = (v31 * v1819);
        let v6421: f64 = (v1819 * v1819);
        let v6435: f64 = (if v1789 { (((v1819 * ((v1814 * v6345) + (v1802 * v6377))) - (v1815 * (((v6393 + v6393) + (v6401 + v6401)) / v6413))) / v6421) } else { v4 });
        let v6436: f64 = (if v1789 { (((v1819 * ((v1814 * v6346) + (v1802 * v6378))) - (v1815 * (((v6395 + v6395) + (v6403 + v6403)) / v6413))) / v6421) } else { v4 });
        let v6437: f64 = (if v1789 { (((v1819 * ((v1814 * v6347) + (v1802 * v6379))) - (v1815 * (((v6397 + v6397) + (v6405 + v6405)) / v6413))) / v6421) } else { v4 });
        let v6438: f64 = (if v1789 { (((v1819 * ((v1814 * v6348) + (v1802 * v6380))) - (v1815 * (((v6399 + v6399) + (v6407 + v6407)) / v6413))) / v6421) } else { v4 });
        let v6442: f64 = (v1821 * v1821);
        let v6455: f64 = (if v1789 { (((v1821 * v2570) - (v1796 * v6435)) / v6442) } else { v4 });
        let v6456: f64 = (if v1789 { (((v1821 * self.scalar_v2979) - (v1796 * v6436)) / v6442) } else { v4 });
        let v6457: f64 = (if v1789 { (((self.scalar_v0 * v1821) - (v1796 * v6437)) / v6442) } else { v4 });
        let v6458: f64 = (if v1789 { ((-(v1796 * v6438)) / v6442) } else { v4 });
        let v6459: f64 = (v411 * v6435);
        let v6460: f64 = (v411 * v6436);
        let v6461: f64 = (v411 * v6437);
        let v6462: f64 = (v411 * v6438);
        let v6463: f64 = (v1795 * v6459);
        let v6464: f64 = (v1795 * v6460);
        let v6465: f64 = (v1795 * v6461);
        let v6466: f64 = (v1795 * v6462);
        let v6483: f64 = (if v1789 { (v6455 + ((v1825 * v3910) + (v1054 * v6463))) } else { v4 });
        let v6484: f64 = (if v1789 { (v6456 + ((v1825 * v3911) + (v1054 * v6464))) } else { v4 });
        let v6485: f64 = (if v1789 { (v6457 + ((v1825 * v3912) + (v1054 * v6465))) } else { v4 });
        let v6486: f64 = (if v1789 { (v6458 + ((v1825 * v3913) + (v1054 * v6466))) } else { v4 });
        let v6510: f64 = (v1841 * v1841);
        let v6532: f64 = ((v1843 * v6463) + (v1825 * (-(((v1841 * v4442) - (v1198 * (self.scalar_v892 * (if v1808 { (self.scalar_v1831 * (v31 * v3886)) } else { v4 })))) / v6510))));
        let v6536: f64 = ((v1843 * v6464) + (v1825 * (-(((v1841 * v4450) - (v1198 * (self.scalar_v892 * (if v1808 { (self.scalar_v1831 * (v31 * v3887)) } else { v4 })))) / v6510))));
        let v6539: f64 = ((v1843 * v6465) + (v1825 * (-(((v1841 * v4454) - (v1198 * (self.scalar_v892 * (if v1808 { (self.scalar_v1831 * (v31 * v3888)) } else { v4 })))) / v6510))));
        let v6542: f64 = ((v1843 * v6466) + (v1825 * (-(((v1841 * v4458) - (v1198 * (self.scalar_v892 * (if v1808 { (self.scalar_v1831 * (v31 * v3889)) } else { v4 })))) / v6510))));
        let v6548: f64 = (if v1808 { (v6455 - v6532) } else { v4 });
        let v6549: f64 = (if v1808 { (-(v1825 * (-(v4446 / v1841)))) } else { v4 });
        let v6550: f64 = (if v1808 { (v6456 - v6536) } else { v4 });
        let v6551: f64 = (if v1808 { (v6457 - v6539) } else { v4 });
        let v6552: f64 = (if v1808 { (v6458 - v6542) } else { v4 });
        let v6557: f64 = (v1847 * (v6548 - v6483));
        let v6559: f64 = (v1847 * v6549);
        let v6561: f64 = (v1847 * (v6550 - v6484));
        let v6563: f64 = (v1847 * (v6551 - v6485));
        let v6565: f64 = (v1847 * (v6552 - v6486));
        let v6612: f64 = (v31 * v1856);
        let v6613: f64 = ((if v1808 { ((v6557 + v6557) + (((v1850 * v3898) + (v1051 * ((v1849 * v6455) + (v1823 * (v46 * v6455))))) / self.scalar_v892)) } else { v6328 }) / v6612);
        let v6615: f64 = ((if v1808 { ((v6561 + v6561) + (((v1850 * v3899) + (v1051 * ((v1849 * v6456) + (v1823 * (v46 * v6456))))) / self.scalar_v892)) } else { v6329 }) / v6612);
        let v6616: f64 = ((if v1808 { ((v6563 + v6563) + (((v1850 * v3900) + (v1051 * ((v1849 * v6457) + (v1823 * (v46 * v6457))))) / self.scalar_v892)) } else { v6330 }) / v6612);
        let v6617: f64 = ((if v1808 { ((v6565 + v6565) + (((v1850 * v3901) + (v1051 * ((v1849 * v6458) + (v1823 * (v46 * v6458))))) / self.scalar_v892)) } else { v6331 }) / v6612);
        let v6628: f64 = (if v1808 { (v411 * ((v6483 + v6548) + v6613)) } else { (if v1805 { v6483 } else { v4 }) });
        let v6629: f64 = (if v1808 { (v411 * (v6549 + ((if v1808 { (v6559 + v6559) } else { v4 }) / v6612))) } else { v4 });
        let v6630: f64 = (if v1808 { (v411 * ((v6484 + v6550) + v6615)) } else { (if v1805 { v6484 } else { v4 }) });
        let v6631: f64 = (if v1808 { (v411 * ((v6485 + v6551) + v6616)) } else { (if v1805 { v6485 } else { v4 }) });
        let v6632: f64 = (if v1808 { (v411 * ((v6486 + v6552) + v6617)) } else { (if v1805 { v6486 } else { v4 }) });
        let v6640: f64 = (v1859 * v1859);
        let v6666: f64 = (v1862 * v1862);
        let v6683: f64 = (if v1866 { (((v1862 * v6459) - (v1824 * (if v1789 { (((v1859 * (v6628 - v6455)) - (v1860 * v6628)) / v6640) } else { v4 }))) / v6666) } else { v4 });
        let v6684: f64 = (if v1866 { ((-(v1824 * (if v1789 { (((v1859 * v6629) - (v1860 * v6629)) / v6640) } else { v4 }))) / v6666) } else { v4 });
        let v6685: f64 = (if v1866 { (((v1862 * v6460) - (v1824 * (if v1789 { (((v1859 * (v6630 - v6456)) - (v1860 * v6630)) / v6640) } else { v4 }))) / v6666) } else { v4 });
        let v6686: f64 = (if v1866 { (((v1862 * v6461) - (v1824 * (if v1789 { (((v1859 * (v6631 - v6457)) - (v1860 * v6631)) / v6640) } else { v4 }))) / v6666) } else { v4 });
        let v6687: f64 = (if v1866 { (((v1862 * v6462) - (v1824 * (if v1789 { (((v1859 * (v6632 - v6458)) - (v1860 * v6632)) / v6640) } else { v4 }))) / v6666) } else { v4 });
        let v6718: f64 = (((v1859 * (-v2959)) - (v1872 * v6628)) / v6640);
        let v6721: f64 = ((-(v1872 * v6629)) / v6640);
        let v6724: f64 = ((-(v1872 * v6630)) / v6640);
        let v6727: f64 = ((-(v1872 * v6631)) / v6640);
        let v6730: f64 = ((-(v1872 * v6632)) / v6640);
        let v6731: f64 = (v1874 * v6718);
        let v6732: f64 = (v1874 * v6721);
        let v6733: f64 = (v1874 * v6724);
        let v6734: f64 = (v1874 * v6727);
        let v6735: f64 = (v1874 * v6730);
        let v6739: f64 = (v1868 * v1868);
        let v6783: f64 = ((v1879 * ((v1870 * v6683) + (v1868 * ((v1869 * v6628) + (v1859 * ((-(self.scalar_v10 * v2959)) / (v659 * v659))))))) + (v1871 * (v6731 - (v1878 * ((v1876 * v6718) + (v1873 * (((v1868 * v6377) - (v1814 * v6683)) / v6739)))))));
        let v6786: f64 = ((v1879 * ((v1870 * v6684) + (v1868 * (v1869 * v6629)))) + (v1871 * (v6732 - (v1878 * ((v1876 * v6721) + (v1873 * ((-(v1814 * v6684)) / v6739)))))));
        let v6789: f64 = ((v1879 * ((v1870 * v6685) + (v1868 * (v1869 * v6630)))) + (v1871 * (v6733 - (v1878 * ((v1876 * v6724) + (v1873 * (((v1868 * v6378) - (v1814 * v6685)) / v6739)))))));
        let v6792: f64 = ((v1879 * ((v1870 * v6686) + (v1868 * (v1869 * v6631)))) + (v1871 * (v6734 - (v1878 * ((v1876 * v6727) + (v1873 * (((v1868 * v6379) - (v1814 * v6686)) / v6739)))))));
        let v6795: f64 = ((v1879 * ((v1870 * v6687) + (v1868 * (v1869 * v6632)))) + (v1871 * (v6735 - (v1878 * ((v1876 * v6730) + (v1873 * (((v1868 * v6380) - (v1814 * v6687)) / v6739)))))));
        let v6818: f64 = (if v1883 { ((v1884 * v6731) + (v1874 * (self.scalar_v10 * v6377))) } else { (if v1866 { v6783 } else { (if v1750 { ((v1781 * v6276) + (v1778 * ((v1780 * v6240) + (v1763 * v6284)))) } else { v4 }) }) });
        let v6820: f64 = (if v1883 { ((v1884 * v6733) + (v1874 * (self.scalar_v10 * v6378))) } else { (if v1866 { v6789 } else { (if v1750 { ((v1781 * v6278) + (v1778 * (v1780 * v6242))) } else { v4 }) }) });
        let v6821: f64 = (if v1883 { ((v1884 * v6734) + (v1874 * (self.scalar_v10 * v6379))) } else { (if v1866 { v6792 } else { (if v1750 { ((v1781 * v6279) + (v1778 * (v1780 * v6243))) } else { v4 }) }) });
        let v6822: f64 = (if v1883 { ((v1884 * v6735) + (v1874 * (self.scalar_v10 * v6380))) } else { (if v1866 { v6795 } else { (if v1750 { ((v1781 * v6280) + (v1778 * (v1780 * v6244))) } else { v4 }) }) });
        let v6824: f64 = (self.scalar_v1765 * f64::powf(v1761, self.scalar_v6246));
        let v6830: f64 = (v1894 * v1894);
        let v6855: f64 = (self.scalar_v1897 * f64::powf(v1896, self.scalar_v6853));
        let v6870: f64 = (if v1891 { (v1892 * ((-(((v1894 * v4442) - (v1198 * v4442)) / v6830)) * v6855)) } else { v4 });
        let v6871: f64 = (if v1891 { (v1892 * ((-(((v1894 * v4446) - (v1198 * v4446)) / v6830)) * v6855)) } else { v4 });
        let v6872: f64 = (if v1891 { ((v1898 * (self.scalar_v2979 * v6824)) + (v1892 * ((-(((v1894 * v4450) - (v1198 * v4450)) / v6830)) * v6855))) } else { v4 });
        let v6873: f64 = (if v1891 { ((v1898 * (self.scalar_v0 * v6824)) + (v1892 * ((-(((v1894 * v4454) - (v1198 * v4454)) / v6830)) * v6855))) } else { v4 });
        let v6874: f64 = (if v1891 { (v1892 * ((-(((v1894 * v4458) - (v1198 * v4458)) / v6830)) * v6855)) } else { v4 });
        let v6885: f64 = (if v1903 { (v4442 / self.scalar_v1893) } else { v4 });
        let v6886: f64 = (if v1903 { (v4446 / self.scalar_v1893) } else { v4 });
        let v6887: f64 = (if v1903 { (v4450 / self.scalar_v1893) } else { v4 });
        let v6888: f64 = (if v1903 { (v4454 / self.scalar_v1893) } else { v4 });
        let v6889: f64 = (if v1903 { (v4458 / self.scalar_v1893) } else { v4 });
        let v6895: f64 = (if v1903 { (v6885 / self.scalar_v1909) } else { v4 });
        let v6896: f64 = (if v1903 { (v6886 / self.scalar_v1909) } else { self.scalar_v4494 });
        let v6897: f64 = (if v1903 { (v6887 / self.scalar_v1909) } else { self.scalar_v4495 });
        let v6898: f64 = (if v1903 { (v6888 / self.scalar_v1909) } else { v4 });
        let v6899: f64 = (if v1903 { (v6889 / self.scalar_v1909) } else { v4 });
        let v6952: f64 = (self.scalar_v1929 * f64::powf(v1928, self.scalar_v6950));
        let v6959: f64 = (v1900 * ((if v1921 { (v6885 + (self.scalar_v1909 * ((v1923 * (-v6895)) / v1924))) } else { (if v1913 { (self.scalar_v1909 * ((v1914 * v6895) / v1915)) } else { v4 }) }) * v6952));
        let v6962: f64 = (v1900 * ((if v1921 { (v6886 + (self.scalar_v1909 * ((v1923 * (-v6896)) / v1924))) } else { (if v1913 { (self.scalar_v1909 * ((v1914 * v6896) / v1915)) } else { v4 }) }) * v6952));
        let v6965: f64 = (v1900 * ((if v1921 { (v6887 + (self.scalar_v1909 * ((v1923 * (-v6897)) / v1924))) } else { (if v1913 { (self.scalar_v1909 * ((v1914 * v6897) / v1915)) } else { v4 }) }) * v6952));
        let v6968: f64 = (v1900 * ((if v1921 { (v6888 + (self.scalar_v1909 * ((v1923 * (-v6898)) / v1924))) } else { (if v1913 { (self.scalar_v1909 * ((v1914 * v6898) / v1915)) } else { v4 }) }) * v6952));
        let v6971: f64 = (v1900 * ((if v1921 { (v6889 + (self.scalar_v1909 * ((v1923 * (-v6899)) / v1924))) } else { (if v1913 { (self.scalar_v1909 * ((v1914 * v6899) / v1915)) } else { v4 }) }) * v6952));
        let v6980: f64 = ((v1932 * v6245) + (v1764 * (if v1903 { ((v1930 * v6870) + v6959) } else { (if v1901 { v6870 } else { v4 }) })));
        let v6981: f64 = (v1764 * (if v1903 { ((v1930 * v6871) + v6962) } else { (if v1901 { v6871 } else { v4 }) }));
        let v6982: f64 = (v1764 * (if v1903 { ((v1930 * v6872) + v6965) } else { (if v1901 { v6872 } else { v4 }) }));
        let v6983: f64 = (v1764 * (if v1903 { ((v1930 * v6873) + v6968) } else { (if v1901 { v6873 } else { v4 }) }));
        let v6984: f64 = (v1764 * (if v1903 { ((v1930 * v6874) + v6971) } else { (if v1901 { v6874 } else { v4 }) }));
        let v7019: f64 = (if v1891 { ((v1945 * (if v1939 { (v1940 * v6980) } else { (if v1935 { (v1936 * v6980) } else { v6276 }) })) + (v1944 * (v1761 * v6284))) } else { v6818 });
        let v7020: f64 = (if v1891 { (v1945 * (if v1939 { (v1940 * v6981) } else { (if v1935 { (v1936 * v6981) } else { v6277 }) })) } else { (if v1883 { (v1884 * v6732) } else { (if v1866 { v6786 } else { (if v1750 { ((v1781 * v6277) + (v1778 * (v1780 * v6241))) } else { v4 }) }) }) });
        let v7021: f64 = (if v1891 { ((v1945 * (if v1939 { (v1940 * v6982) } else { (if v1935 { (v1936 * v6982) } else { v6278 }) })) + (v1944 * (v1780 * self.scalar_v2979))) } else { v6820 });
        let v7022: f64 = (if v1891 { ((v1945 * (if v1939 { (v1940 * v6983) } else { (if v1935 { (v1936 * v6983) } else { v6279 }) })) + (v1944 * (self.scalar_v0 * v1780))) } else { v6821 });
        let v7023: f64 = (if v1891 { (v1945 * (if v1939 { (v1940 * v6984) } else { (if v1935 { (v1936 * v6984) } else { v6280 }) })) } else { v6822 });
        let v7024: f64 = (v2675 + v6170);
        let v7043: f64 = (v1954 * v1954);
        let v7072: f64 = ((((v1954 * v2473) - (v119 * ((v1953 * v4442) + (v1198 * v7024)))) / v7043) + ((v1956 * v2785) + (v487 * (((v436 * v4409) - (v1192 * v2751)) / v4566))));
        let v7080: f64 = (v1953 * v1953);
        let v7095: f64 = ((((-(v119 * ((v1953 * v4446) + (v1198 * v6171)))) / v7043) + (v487 * (v4412 / v436))) + ((-(v308 * v6171)) / v7080));
        let v7096: f64 = ((((-(v119 * ((v1953 * v4450) + (v1198 * v6172)))) / v7043) + (v487 * (v4415 / v436))) + ((-(v308 * v6172)) / v7080));
        let v7097: f64 = ((((-(v119 * ((v1953 * v4454) + (v1198 * v6173)))) / v7043) + (v487 * (v4418 / v436))) + ((-(v308 * v6173)) / v7080));
        let v7098: f64 = ((((-(v119 * ((v1953 * v4458) + (v1198 * v6174)))) / v7043) + (v487 * (v4421 / v436))) + ((-(v308 * v6174)) / v7080));
        let v7099: f64 = (if v1952 { (v7072 + (((v1953 * v2668) - (v308 * v7024)) / v7080)) } else { v4 });
        let v7100: f64 = (if v1952 { v7095 } else { v4 });
        let v7101: f64 = (if v1952 { v7096 } else { v4 });
        let v7102: f64 = (if v1952 { v7097 } else { v4 });
        let v7103: f64 = (if v1952 { v7098 } else { v4 });
        let v7114: f64 = (if v1962 { ((v7019 - v7099) / v408) } else { v6895 });
        let v7115: f64 = (if v1962 { ((v7020 - v7100) / v408) } else { v6896 });
        let v7116: f64 = (if v1962 { ((v7021 - v7101) / v408) } else { v6897 });
        let v7117: f64 = (if v1962 { ((v7022 - v7102) / v408) } else { v6898 });
        let v7118: f64 = (if v1962 { ((v7023 - v7103) / v408) } else { v6899 });
        let v7169: f64 = (if v1975 { (v7099 - (v408 * ((v1977 * (-v7114)) / v1978))) } else { (if v1967 { (v7019 - (v408 * ((v1968 * v7114) / v1969))) } else { v7019 }) });
        let v7170: f64 = (if v1975 { (v7100 - (v408 * ((v1977 * (-v7115)) / v1978))) } else { (if v1967 { (v7020 - (v408 * ((v1968 * v7115) / v1969))) } else { v7020 }) });
        let v7171: f64 = (if v1975 { (v7101 - (v408 * ((v1977 * (-v7116)) / v1978))) } else { (if v1967 { (v7021 - (v408 * ((v1968 * v7116) / v1969))) } else { v7021 }) });
        let v7172: f64 = (if v1975 { (v7102 - (v408 * ((v1977 * (-v7117)) / v1978))) } else { (if v1967 { (v7022 - (v408 * ((v1968 * v7117) / v1969))) } else { v7022 }) });
        let v7173: f64 = (if v1975 { (v7103 - (v408 * ((v1977 * (-v7118)) / v1978))) } else { (if v1967 { (v7023 - (v408 * ((v1968 * v7118) / v1969))) } else { v7023 }) });
        let v7176: f64 = ((v1982 * v4442) + (v1198 * v7169));
        let v7179: f64 = ((v1982 * v4446) + (v1198 * v7170));
        let v7182: f64 = ((v1982 * v4450) + (v1198 * v7171));
        let v7185: f64 = ((v1982 * v4454) + (v1198 * v7172));
        let v7188: f64 = ((v1982 * v4458) + (v1198 * v7173));
        let v7217: f64 = (v1988 * v1988);
        let v7240: f64 = (if v1992 { v7176 } else { (if v1986 { (((v1988 * ((v1983 * v7099) + (v1961 * v7176))) - (v1987 * (v7099 + v7169))) / v7217) } else { (if v1962 { v7176 } else { v4 }) }) });
        let v7241: f64 = (if v1992 { v7179 } else { (if v1986 { (((v1988 * ((v1983 * v7100) + (v1961 * v7179))) - (v1987 * (v7100 + v7170))) / v7217) } else { (if v1962 { v7179 } else { v4 }) }) });
        let v7242: f64 = (if v1992 { v7182 } else { (if v1986 { (((v1988 * ((v1983 * v7101) + (v1961 * v7182))) - (v1987 * (v7101 + v7171))) / v7217) } else { (if v1962 { v7182 } else { v4 }) }) });
        let v7243: f64 = (if v1992 { v7185 } else { (if v1986 { (((v1988 * ((v1983 * v7102) + (v1961 * v7185))) - (v1987 * (v7102 + v7172))) / v7217) } else { (if v1962 { v7185 } else { v4 }) }) });
        let v7244: f64 = (if v1992 { v7188 } else { (if v1986 { (((v1988 * ((v1983 * v7103) + (v1961 * v7188))) - (v1987 * (v7103 + v7173))) / v7217) } else { (if v1962 { v7188 } else { v4 }) }) });
        let v7259: f64 = (if v1998 { v4 } else { (if v1994 { ((v1995 * v2473) + (v119 * (v3830 / v1024))) } else { v4 }) });
        let v7260: f64 = (if v1998 { self.scalar_v0 } else { (if v1994 { (v119 * (v3831 / v1024)) } else { v4 }) });
        let v7261: f64 = (if v1998 { v4 } else { (if v1994 { (v119 * (v3832 / v1024)) } else { v4 }) });
        let v7262: f64 = (if v1998 { self.scalar_v2979 } else { (if v1994 { (v119 * (v3833 / v1024)) } else { v4 }) });
        let v7320: f64 = ((((v2002 * v4450) + (v1198 * (self.scalar_v0 - v7260))) + ((v2004 * v3218) + (v849 * (v7260 - self.scalar_v0)))) - ((v1999 * v7242) + (v1993 * v7260)));
        let v7321: f64 = ((((v2002 * v4454) + (v1198 * (-v7261))) + ((v2004 * v3219) + (v849 * (v7261 - self.scalar_v2979)))) - ((v1999 * v7243) + (v1993 * v7261)));
        let v7324: f64 = (v710 * self.scalar_v2979);
        let v7329: f64 = (v308 * v308);
        let v7332: f64 = (((((v2002 * v4442) + (v1198 * (-v7259))) + ((v2004 * v3217) + (v849 * v7259))) - ((v1999 * v7240) + (v1993 * v7259))) + ((-(v2009 * v2668)) / v7329));
        let v7335: f64 = (v731 * self.scalar_v2980);
        let v7337: f64 = (v731 * self.scalar_v2981);
        let v7339: f64 = (v731 * self.scalar_v2979);
        let v7342: f64 = (v674 * (v7335 + v7335));
        let v7344: f64 = (v674 * (v7337 + v7337));
        let v7349: f64 = (((((v2002 * v4458) + (v1198 * (-v7262))) + ((v2004 * v3220) + (v849 * v7262))) - ((v1999 * v7244) + (v1993 * v7262))) + v7344);
        let v7351: f64 = (v724 * self.scalar_v2979);
        let v7359: f64 = (v721 * self.scalar_v2979);
        let v7369: f64 = (v713 * self.scalar_v2979);
        let v7374: f64 = (v322 * v322);
        let v7395: f64 = (v4723 + ((v1371 * ((v460 * (self.scalar_v448 * (v454 * (self.scalar_v452 * v2477)))) + (v455 * (v460 * (v2761 / self.scalar_v450))))) + (v461 * v4826)));
        let v7400: f64 = (((if self.scalar_v1315 { (v487 * ((self.scalar_v1316 * v4545) + (v1299 * (self.scalar_v1309 * v4545)))) } else { (if self.scalar_v1312 { v4595 } else { (if self.scalar_v497 { v4676 } else { v4 }) }) }) + (v461 * v4827)) + self.scalar_v7398);
        let v7405: f64 = (((v1249 * (self.scalar_v1246 * v4516)) + (v1247 * ((-v4516) * v4523))) + (v7400 - (if v1498 { v4 } else { (if v1412 { (self.scalar_v35 * (v284 * v5149)) } else { v4 }) })));
        let v7406: f64 = (((v1249 * (self.scalar_v1246 * v4517)) + (v1247 * ((-v4517) * v4523))) + (((v4725 + (v461 * v4829)) + self.scalar_v7399) - (if v1498 { v4 } else { (if v1412 { (self.scalar_v35 * (v284 * v5152)) } else { v4 }) })));
        let v7407: f64 = ((v1226 * ((v556 * (self.scalar_v551 * (v2472 / (v31 * v552)))) + (v553 * (v556 * (self.scalar_v554 * v2471))))) + (v7395 - (if v1498 { v4 } else { (if v1412 { (self.scalar_v35 * ((v1494 * v2640) + v5154)) } else { v4 }) })));
        let v7408: f64 = ((v557 * v4489) + v7405);
        let v7409: f64 = ((v557 * v4490) + v7406);
        let v7419: f64 = ((((((v7332 + (v2012 * v2966)) + (v2015 * v2972)) + (v2018 * v2978)) + ((-(v2021 * v2675)) / v7374)) + (v705 * v6186)) + (v700 * v7407));
        let v7420: f64 = ((((((v2002 * v4446) + (v1198 * self.scalar_v2979)) - (v1999 * v7241)) + ((v7324 + v7324) / v308)) + (v705 * v6189)) + ((v2031 * self.scalar_v2979) + (v700 * v7408)));
        let v7446: f64 = ((((v7320 + v7342) + ((v1739 * self.scalar_v2979) + (v705 * v6194))) + (v2401 + (v700 * v7409))) - ((v2001 * v5996) + (v1716 * self.scalar_v7265)));
        let v7447: f64 = (((((v7321 + v7344) + (v690 * (v7359 + v7359))) + (v705 * v6197)) + (v700 * v4726)) - ((v2001 * v5999) + (v1716 * self.scalar_v7266)));
        let v7451: f64 = ((if self.scalar_v1311 { v4764 } else { (if self.scalar_v497 { (v4764 + (v4786 / v4787)) } else { v4 }) }) + ((v1383 * ((v540 * (self.scalar_v532 * (v537 * (self.scalar_v535 * v2477)))) + (v538 * (v540 * (v2761 / self.scalar_v533))))) + (v541 * v4849)));
        let v7455: f64 = (((v1407 * ((v549 * (self.scalar_v542 * (v546 * (self.scalar_v544 * v2477)))) + (v547 * (v549 * (v2761 / self.scalar_v543))))) + (v550 * v4909)) + v7451);
        let v7456: f64 = ((v550 * v4910) + ((if self.scalar_v1311 { v4765 } else { (if self.scalar_v497 { (v4765 + v4792) } else { v4 }) }) + (v541 * v4850)));
        let v7457: f64 = ((v550 * v4911) + ((if self.scalar_v1311 { v4766 } else { (if self.scalar_v497 { (v4766 + v4796) } else { v4 }) }) + (v541 * v4851)));
        let v7458: f64 = ((v550 * v4912) + ((if self.scalar_v1311 { v4767 } else { v4808 }) + (v541 * v4852)));
        let v7466: f64 = (v703 * v4921);
        let v7470: f64 = (((((v7342 + ((v7369 + v7369) / v322)) + (v2408 + (v705 * v6190))) + (v700 * v4834)) - (v2001 * v5993)) + (v2399 + (v703 * v7457)));
        let v7475: f64 = ((v1615 * v5977) + (v1396 * v5977));
        let v7476: f64 = ((v1615 * v5978) + (v1396 * v5978));
        let v7477: f64 = (((v1715 * (if self.scalar_v1613 { (self.scalar_v14 * v5518) } else { v5518 })) + (v1615 * v5979)) + ((v1715 * v4888) + (v1396 * v5979)));
        let v7478: f64 = ((v1615 * v5980) + ((v1715 * (v473 * v4881)) + (v1396 * v5980)));
        let v7483: f64 = ((v1615 * v5985) + (v1396 * v5985));
        let v7502: f64 = (v2042 * self.scalar_v2981);
        let v7518: f64 = (((((v7349 + (v705 * v6200)) + (v700 * v4727)) - ((v2001 * v6000) + (v1716 * self.scalar_v7267))) + v7466) + (v7502 + (v727 * ((v6019 + v6042) + self.scalar_v7486))));
        let v7520: f64 = (((((v7344 + (v682 * (v7351 + v7351))) + (v690 * (v2458 + v2458))) - (v2001 * v6002)) + (v703 * v4922)) + ((v2042 * self.scalar_v2979) + (v727 * (self.scalar_v7398 + (v6023 + v6046)))));
        let v7521: f64 = (v1719 * self.scalar_v2980);
        let v7533: f64 = (v1719 * self.scalar_v2981);
        let v7551: f64 = (((((v674 * (v7339 + v7339)) + (v682 * (v2454 + v2454))) - (v2001 * v6001)) + (v727 * v7483)) + ((v1719 * self.scalar_v2979) + (v732 * v6072)));
        let v7553: f64 = (self.scalar_v2048 * v2652);
        let v7561: f64 = ((v3917 - (v2051 * v3915)) / v3920);
        let v7594: f64 = (if v2060 { (v3914 - ((v2064 * v3915) + (v1059 * ((v2062 * (-v7561)) / v2063)))) } else { (if v2053 { (-((v2056 * v3915) + (v1059 * ((v2054 * v7561) / v2055)))) } else { v4 }) });
        let v7595: f64 = (if v2060 { (-(v1059 * ((v2062 * v3942) / v2063))) } else { (if v2053 { (self.scalar_v2979 - (v1059 * ((v2054 * v3922) / v2055))) } else { v4 }) });
        let v7596: f64 = (if v2060 { (-(v1059 * ((v2062 * v3943) / v2063))) } else { (if v2053 { (self.scalar_v0 - (v1059 * ((v2054 * v3923) / v2055))) } else { v4 }) });
        let v7607: f64 = (self.scalar_v1079 * f64::powf(v2070, self.scalar_v3969));
        let v7630: f64 = ((v2076 * (self.scalar_v2047 * v2652)) + (v2068 * (((v2072 * v3975) + (v1081 * (-((-((v2067 * v2640) + (v284 * v7594))) * v7607)))) + (v170 * (-v7594)))));
        let v7642: f64 = ((v626 * v2754) + (v441 * v2941));
        let v7643: f64 = (v411 * v7642);
        let v7651: f64 = ((v2083 * v6128) + (v1730 * ((v2082 * v4202) + (v1145 * v7643))));
        let v7654: f64 = ((v2083 * v6129) + (v1730 * (v2082 * v4206)));
        let v7657: f64 = ((v2083 * v6130) + (v1730 * (v2082 * v4210)));
        let v7658: f64 = (v2083 * v6131);
        let v7659: f64 = (v2083 * v6132);
        let v7668: f64 = ((v2085 * v6128) + (v1730 * ((v2082 * v4239) + (v1152 * v7643))));
        let v7669: f64 = (v2085 * v6129);
        let v7672: f64 = ((v2085 * v6130) + (v1730 * (v2082 * v4243)));
        let v7675: f64 = ((v2085 * v6131) + (v1730 * (v2082 * v4247)));
        let v7678: f64 = ((v2085 * v6132) + (v1730 * (v2082 * v4251)));
        let v7680: f64 = (v1004 * (-v4018));
        let v7683: f64 = (v1004 * v1004);
        let v7684: f64 = ((v7680 - (v2087 * v3736)) / v7683);
        let v7685: f64 = (self.scalar_v0 / v1004);
        let v7686: f64 = (self.scalar_v2980 / v1004);
        let v7687: f64 = (self.scalar_v2981 / v1004);
        let v7688: f64 = (self.scalar_v2979 / v1004);
        let v7718: f64 = (-v7686);
        let v7719: f64 = (-v7687);
        let v7720: f64 = (-v7688);
        let v7743: f64 = (if v2096 { (v4018 - ((v2100 * v3736) + (v1004 * ((v2098 * (-v7684)) / v2099)))) } else { (if v2089 { (-((v2092 * v3736) + (v1004 * ((v2090 * v7684) / v2091)))) } else { v4 }) });
        let v7744: f64 = (if v2096 { (-(v1004 * ((v2098 * (-v7685)) / v2099))) } else { (if v2089 { (self.scalar_v0 - (v1004 * ((v2090 * v7685) / v2091))) } else { v4 }) });
        let v7745: f64 = (if v2096 { (-(v1004 * ((v2098 * v7718) / v2099))) } else { (if v2089 { (self.scalar_v2980 - (v1004 * ((v2090 * v7686) / v2091))) } else { v4 }) });
        let v7746: f64 = (if v2096 { (-(v1004 * ((v2098 * v7719) / v2099))) } else { (if v2089 { (self.scalar_v2981 - (v1004 * ((v2090 * v7687) / v2091))) } else { v4 }) });
        let v7747: f64 = (if v2096 { (-(v1004 * ((v2098 * v7720) / v2099))) } else { (if v2089 { (self.scalar_v2979 - (v1004 * ((v2090 * v7688) / v2091))) } else { v4 }) });
        let v7762: f64 = (self.scalar_v1124 * f64::powf(v2105, self.scalar_v4116));
        let v7792: f64 = (((v2107 * v4104) + (v1125 * (-((-(((v260 * v7743) - (v2103 * v2614)) / v2642)) * v7762)))) + ((v2109 * v4010) + (v1100 * (-v7743))));
        let v7805: f64 = (v301 * self.scalar_v2980);
        let v7806: f64 = (v301 * self.scalar_v2981);
        let v7825: f64 = (self.scalar_v14 * (self.scalar_v2116 * (v300 * (v4178 + (v1099 * ((v1125 * (-((-(v7744 / v260)) * v7762))) + (v1100 * (self.scalar_v0 - v7744))))))));
        let v7826: f64 = (self.scalar_v14 * (self.scalar_v2116 * (v300 * ((v1099 * ((v1125 * (-((-(v7745 / v260)) * v7762))) + (v1100 * (self.scalar_v2980 - v7745)))) + v7805))));
        let v7827: f64 = (self.scalar_v14 * (self.scalar_v2116 * (v300 * ((v1099 * ((v1125 * (-((-(v7746 / v260)) * v7762))) + (v1100 * (self.scalar_v2981 - v7746)))) + v7806))));
        let v7828: f64 = (self.scalar_v14 * (self.scalar_v2116 * (v300 * (v4179 + (v1099 * ((v1125 * (-((-(v7747 / v260)) * v7762))) + (v1100 * (self.scalar_v2979 - v7747))))))));
        let v7829: f64 = (self.scalar_v2982 / v1004);
        let v7832: f64 = ((v7680 - (v2119 * v3736)) / v7683);
        let v7884: f64 = (if v2128 { (-(v1004 * ((v2130 * v7718) / v2131))) } else { (if v2121 { (self.scalar_v2980 - (v1004 * ((v2122 * v7686) / v2123))) } else { v4 }) });
        let v7885: f64 = (if v2128 { (-(v1004 * ((v2130 * (-v7829)) / v2131))) } else { (if v2121 { (self.scalar_v2982 - (v1004 * ((v2122 * v7829) / v2123))) } else { v4 }) });
        let v7886: f64 = (if v2128 { (v4018 - ((v2132 * v3736) + (v1004 * ((v2130 * (-v7832)) / v2131)))) } else { (if v2121 { (-((v2124 * v3736) + (v1004 * ((v2122 * v7832) / v2123)))) } else { v4 }) });
        let v7887: f64 = (if v2128 { (-(v1004 * ((v2130 * v7719) / v2131))) } else { (if v2121 { (self.scalar_v2981 - (v1004 * ((v2122 * v7687) / v2123))) } else { v4 }) });
        let v7888: f64 = (if v2128 { (-(v1004 * ((v2130 * v7720) / v2131))) } else { (if v2121 { (self.scalar_v2979 - (v1004 * ((v2122 * v7688) / v2123))) } else { v4 }) });
        let v7903: f64 = (self.scalar_v1124 * f64::powf(v2137, self.scalar_v4116));
        let v7935: f64 = (((v2139 * v4104) + (v1125 * (-((-(((v260 * v7886) - (v2135 * v2614)) / v2642)) * v7903)))) + ((v2141 * v4010) + (v1100 * (-v7886))));
        let v7960: f64 = (self.scalar_v2116 * (v300 * ((v1099 * ((v1125 * (-((-(v7885 / v260)) * v7903))) + (v1100 * (self.scalar_v2982 - v7885)))) + (v301 * self.scalar_v2982))));
        let v7964: f64 = (self.scalar_v13 * (self.scalar_v2116 * (v300 * (v7805 + (v1099 * ((v1125 * (-((-(v7884 / v260)) * v7903))) + (v1100 * (self.scalar_v2980 - v7884))))))));
        let v7967: f64 = (self.scalar_v13 * (self.scalar_v2116 * (v300 * (v7806 + (v1099 * ((v1125 * (-((-(v7887 / v260)) * v7903))) + (v1100 * (self.scalar_v2981 - v7887))))))));
        let v7968: f64 = (self.scalar_v13 * (self.scalar_v2116 * (v300 * (v4179 + (v1099 * ((v1125 * (-((-(v7888 / v260)) * v7903))) + (v1100 * (self.scalar_v2979 - v7888))))))));
        let v7980: f64 = (v2154 * ((v620 * v2754) + (v441 * ((v619 * (self.scalar_v611 * (v614 * (self.scalar_v612 * v2477)))) + (v615 * (v619 * (self.scalar_v617 * v2476)))))));
        let v7983: f64 = (self.scalar_v2152 * v2473);
        let v7986: f64 = (v2156 * v2156);
        let v7987: f64 = ((-(v700 * v7983)) / v7986);
        let v7988: f64 = (self.scalar_v2979 / v2156);
        let v7989: f64 = (self.scalar_v0 / v2156);
        let v8010: f64 = ((v2166 * (v7980 + (v2150 * ((((v441 * v2751) - (v436 * v2754)) / v4187) * (self.scalar_v2153 * f64::powf(v2151, self.scalar_v7976)))))) + (v2155 * (if v2161 { (v2162 * v7987) } else { (if v2158 { (v2159 * v7987) } else { v4909 }) })));
        let v8011: f64 = (v2155 * (if v2161 { (v2162 * v7988) } else { (if v2158 { (v2159 * v7988) } else { v4910 }) }));
        let v8012: f64 = (v2155 * (if v2161 { v4 } else { (if v2158 { v4 } else { v4911 }) }));
        let v8013: f64 = (v2155 * (if v2161 { (v2162 * v7989) } else { (if v2158 { (v2159 * v7989) } else { v4912 }) }));
        let v8014: f64 = (v2155 * (if v2161 { v4 } else { (if v2158 { v4 } else { v4913 }) }));
        let v8015: f64 = (v2155 * (if v2161 { v4 } else { (if v2158 { v4 } else { v4914 }) }));
        let v8023: f64 = (((v339 * ((v2168 * v2473) + (v119 * (v423 * v2944)))) - (v2169 * v2685)) / v3216);
        let v8058: f64 = (((v2081 * (((v1594 * (v5422 - v4188)) - (v1591 * (v5422 / v5433))) / v5442)) + (v1595 * v7642)) + ((v2170 * (((v1598 * v5427) - (v1590 * (v5427 / v5460))) / v5469)) + (v1599 * v8023)));
        let v8059: f64 = ((v2081 * (((v1594 * v5423) - (v1591 * (v5423 / v5433))) / v5442)) + (v2170 * (((v1598 * v5428) - (v1590 * (v5428 / v5460))) / v5469)));
        let v8060: f64 = ((v2081 * (((v1594 * v5424) - (v1591 * (v5424 / v5433))) / v5442)) + (v2170 * (((v1598 * v5429) - (v1590 * (v5429 / v5460))) / v5469)));
        let v8061: f64 = ((v2081 * (((v1594 * v5425) - (v1591 * (v5425 / v5433))) / v5442)) + (v2170 * (((v1598 * v5430) - (v1590 * (v5430 / v5460))) / v5469)));
        let v8062: f64 = ((v2081 * (((v1594 * v5426) - (v1591 * (v5426 / v5433))) / v5442)) + (v2170 * (((v1598 * v5431) - (v1590 * (v5431 / v5460))) / v5469)));
        let v8073: f64 = (v633 * v633);
        let v8084: f64 = (-v2592);
        let v8092: f64 = ((v2186 * v2476) + (v121 * (v8084 / self.scalar_v2185)));
        let v8093: f64 = (v121 * self.scalar_v8086);
        let v8094: f64 = (v121 * self.scalar_v8087);
        let v8095: f64 = (v121 * self.scalar_v8088);
        let v8096: f64 = (v121 * self.scalar_v8089);
        let v8132: f64 = (v31 * v2204);
        let v8140: f64 = ((v2205 * ((v2200 * v3035) + (v765 * ((v1600 * v2950) + (v642 * v5487))))) - (v2201 * ((v423 * (if v2194 { (v2195 * v8092) } else { (if v2190 { (v2191 * v8092) } else { v4 }) })) / v8132)));
        let v8141: f64 = (v2205 * v2205);
        let v8146: f64 = (((v2205 * (v2200 * v3036)) - (v2201 * ((v423 * (if v2194 { (v2195 * v8093) } else { (if v2190 { (v2191 * v8093) } else { v4 }) })) / v8132))) / v8141);
        let v8150: f64 = (((v2205 * (v2200 * v3037)) - (v2201 * ((v423 * (if v2194 { (v2195 * v8094) } else { (if v2190 { (v2191 * v8094) } else { v4 }) })) / v8132))) / v8141);
        let v8154: f64 = (((v2205 * (v2200 * v3038)) - (v2201 * ((v423 * (if v2194 { (v2195 * v8095) } else { (if v2190 { (v2191 * v8095) } else { v4 }) })) / v8132))) / v8141);
        let v8158: f64 = (((v2205 * (v2200 * v3039)) - (v2201 * ((v423 * (if v2194 { (v2195 * v8096) } else { (if v2190 { (v2191 * v8096) } else { v4 }) })) / v8132))) / v8141);
        let v8159: f64 = (if self.scalar_v2189 { (v8140 / v8141) } else { (if self.scalar_v2176 { (((v633 * ((v2180 * (v411 * v2947)) + (v2177 * v8058))) - (v2181 * v2945)) / v8073) } else { v4 }) });
        let v8160: f64 = (if self.scalar_v2189 { v8146 } else { (if self.scalar_v2176 { ((v2177 * v8059) / v633) } else { v4 }) });
        let v8161: f64 = (if self.scalar_v2189 { v8150 } else { (if self.scalar_v2176 { ((v2177 * v8060) / v633) } else { v4 }) });
        let v8162: f64 = (if self.scalar_v2189 { v8154 } else { (if self.scalar_v2176 { ((v2177 * v8061) / v633) } else { v4 }) });
        let v8163: f64 = (if self.scalar_v2189 { v8158 } else { (if self.scalar_v2176 { ((v2177 * v8062) / v633) } else { v4 }) });
        let v8181: f64 = (if self.scalar_v2213 { (v1140 * v3070) } else { v4 });
        let v8182: f64 = (if self.scalar_v2213 { (v1140 * v3071) } else { v4 });
        let v8183: f64 = (if self.scalar_v2213 { ((v1140 * v3072) + (v785 * v4188)) } else { v4 });
        let v8184: f64 = (if self.scalar_v2213 { (v1140 * v3073) } else { v4 });
        let v8185: f64 = (if self.scalar_v2213 { (v1140 * v3074) } else { v4 });
        let v8187: f64 = (v31 * v2218);
        let v8196: f64 = (v2219 * v2219);
        let v8224: f64 = (if self.scalar_v2213 { (v423 * (if v791 { (v792 * v3018) } else { (if v788 { (v789 * v3018) } else { v4 }) })) } else { v4 });
        let v8225: f64 = (if self.scalar_v2213 { (v423 * (if v791 { (v792 * v3053) } else { (if v788 { (v789 * v3053) } else { v4 }) })) } else { v4 });
        let v8226: f64 = (if self.scalar_v2213 { (v423 * (if v791 { (v792 * v3078) } else { (if v788 { (v789 * v3078) } else { v4 }) })) } else { v4 });
        let v8227: f64 = (if self.scalar_v2213 { (v423 * (if v791 { (v792 * v3019) } else { (if v788 { (v789 * v3019) } else { v4 }) })) } else { v4 });
        let v8228: f64 = (if self.scalar_v2213 { (v423 * (if v791 { (v792 * v2985) } else { (if v788 { (v789 * v2985) } else { v4 }) })) } else { v4 });
        let v8229: f64 = (v31 * v2225);
        let v8238: f64 = (v2226 * v2226);
        let v8276: f64 = ((v2081 * (if self.scalar_v2213 { (((v2219 * v8181) - (v2216 * (v8181 / v8187))) / v8196) } else { v4 })) + (v2170 * (if self.scalar_v2213 { (((v2226 * v8224) - (v2223 * (v8224 / v8229))) / v8238) } else { v4 })));
        let v8277: f64 = ((v2081 * (if self.scalar_v2213 { (((v2219 * v8182) - (v2216 * (v8182 / v8187))) / v8196) } else { v4 })) + (v2170 * (if self.scalar_v2213 { (((v2226 * v8225) - (v2223 * (v8225 / v8229))) / v8238) } else { v4 })));
        let v8278: f64 = (((v2221 * v7642) + (v2081 * (if self.scalar_v2213 { (((v2219 * (v8183 - v4188)) - (v2216 * (v8183 / v8187))) / v8196) } else { v4 }))) + ((v2228 * v8023) + (v2170 * (if self.scalar_v2213 { (((v2226 * v8226) - (v2223 * (v8226 / v8229))) / v8238) } else { v4 }))));
        let v8279: f64 = ((v2081 * (if self.scalar_v2213 { (((v2219 * v8184) - (v2216 * (v8184 / v8187))) / v8196) } else { v4 })) + (v2170 * (if self.scalar_v2213 { (((v2226 * v8227) - (v2223 * (v8227 / v8229))) / v8238) } else { v4 })));
        let v8280: f64 = ((v2081 * (if self.scalar_v2213 { (((v2219 * v8185) - (v2216 * (v8185 / v8187))) / v8196) } else { v4 })) + (v2170 * (if self.scalar_v2213 { (((v2226 * v8228) - (v2223 * (v8228 / v8229))) / v8238) } else { v4 })));
        let v8303: f64 = ((v2237 * v2476) + (v121 * v8084));
        let v8339: f64 = (v31 * v2255);
        let v8348: f64 = (v2256 * v2256);
        let v8349: f64 = (((v2256 * (v2251 * v3070)) - (v2252 * ((v423 * (if v2245 { (v2246 * v3018) } else { (if v2241 { (v2242 * v3018) } else { v4 }) })) / v8339))) / v8348);
        let v8353: f64 = (((v2256 * (v2251 * v3071)) - (v2252 * ((v423 * (if v2245 { (v2246 * v3053) } else { (if v2241 { (v2242 * v3053) } else { v4 }) })) / v8339))) / v8348);
        let v8356: f64 = ((v2256 * ((v2251 * v3072) + (v785 * ((v1617 * v2950) + (v642 * v5545))))) - (v2252 * ((v423 * (if v2245 { (v2246 * v8303) } else { (if v2241 { (v2242 * v8303) } else { v4 }) })) / v8339)));
        let v8361: f64 = (((v2256 * (v2251 * v3073)) - (v2252 * ((v423 * (if v2245 { (v2246 * v3019) } else { (if v2241 { (v2242 * v3019) } else { v4 }) })) / v8339))) / v8348);
        let v8365: f64 = (((v2256 * (v2251 * v3074)) - (v2252 * ((v423 * (if v2245 { (v2246 * v2985) } else { (if v2241 { (v2242 * v2985) } else { v4 }) })) / v8339))) / v8348);
        let v8368: f64 = (if self.scalar_v2240 { (v8356 / v8348) } else { (if self.scalar_v2213 { (((v633 * ((v2233 * (self.scalar_v2229 * v2947)) + (v2230 * v8278))) - (v2234 * v2945)) / v8073) } else { v4 }) });
        let v8372: f64 = (v1662 * (if self.scalar_v2240 { v8349 } else { (if self.scalar_v2213 { ((v2230 * v8276) / v633) } else { v4 }) }));
        let v8384: f64 = (v1662 * (if self.scalar_v2240 { v8361 } else { (if self.scalar_v2213 { ((v2230 * v8279) / v633) } else { v4 }) }));
        let v8404: f64 = (self.scalar_v2263 * f64::powf(v1078, self.scalar_v8402));
        let v8411: f64 = (if self.scalar_v2262 { v3921 } else { v4 });
        let v8412: f64 = (if self.scalar_v2262 { v3922 } else { v4 });
        let v8413: f64 = (if self.scalar_v2262 { v3923 } else { v4 });
        let v8418: f64 = (v2271 * v2271);
        let v8430: f64 = (v2277 * (-v8411));
        let v8431: f64 = (v2277 * (-v8412));
        let v8432: f64 = (v2277 * (-v8413));
        let v8436: f64 = (v2278 * v2278);
        let v8451: f64 = ((v2280 * (if self.scalar_v2262 { (v3966 * v8404) } else { v4 })) + (v2266 * (if v2275 { (((v2278 * v8430) - (v2277 * v8430)) / v8436) } else { (if v2269 { ((-(v2270 * v8411)) / v8418) } else { v4 }) })));
        let v8454: f64 = ((v2280 * (if self.scalar_v2262 { (v3967 * v8404) } else { v4 })) + (v2266 * (if v2275 { (((v2278 * v8431) - (v2277 * v8431)) / v8436) } else { (if v2269 { ((-(v2270 * v8412)) / v8418) } else { v4 }) })));
        let v8457: f64 = ((v2280 * (if self.scalar_v2262 { (v3968 * v8404) } else { v4 })) + (v2266 * (if v2275 { (((v2278 * v8432) - (v2277 * v8432)) / v8436) } else { (if v2269 { ((-(v2270 * v8413)) / v8418) } else { v4 }) })));
        let v8482: f64 = (v1143 * v1143);
        let v8492: f64 = ((v2288 * (((v371 * ((v1141 * v2476) + (v121 * v4191))) - (v2286 * v2702)) / v2739)) + (v2287 * ((-(v411 * v4195)) / v8482)));
        let v8514: f64 = ((v2291 * (if self.scalar_v2262 { ((v2288 * ((v121 * v4192) / v371)) + (v2287 * ((-(v411 * v4196)) / v8482))) } else { v4 })) + (v2290 * (v2082 * v6129)));
        let v8517: f64 = ((v2291 * (if self.scalar_v2262 { ((v2288 * ((v121 * v4193) / v371)) + (v2287 * ((-(v411 * v4197)) / v8482))) } else { v4 })) + (v2290 * (v2082 * v6130)));
        let v8538: f64 = (if self.scalar_v2262 { (v8014 / v2156) } else { v4 });
        let v8542: f64 = ((if self.scalar_v2262 { ((v2283 * v7553) + (v2049 * (if self.scalar_v2262 { v8451 } else { v4 }))) } else { v4 }) + (if self.scalar_v2262 { ((v2291 * (if self.scalar_v2262 { v8492 } else { v4 })) + (v2290 * ((v2082 * v6128) + (v1730 * v7643)))) } else { v4 }));
        let v8557: f64 = ((v2298 * self.scalar_v8541) + (v2296 * ((if self.scalar_v2262 { (v8013 / v2156) } else { v4 }) + ((if self.scalar_v2262 { (v2049 * (if self.scalar_v2262 { v8457 } else { v4 })) } else { v4 }) + (if self.scalar_v2262 { v8517 } else { v4 })))));
        let v8562: f64 = (if self.scalar_v2262 { (v2296 * ((if self.scalar_v2262 { (v8011 / v2156) } else { v4 }) + ((if self.scalar_v2262 { (v2049 * (if self.scalar_v2262 { v8454 } else { v4 })) } else { v4 }) + (if self.scalar_v2262 { v8514 } else { v4 })))) } else { v4 });
        let v8584: f64 = (self.scalar_v2301 * v8014);
        let v8591: f64 = (if self.scalar_v2262 { (v7651 + (self.scalar_v2301 * v8010)) } else { v4 });
        let v8592: f64 = (if self.scalar_v2262 { (v7654 + (self.scalar_v2301 * v8011)) } else { v4 });
        let v8593: f64 = (if self.scalar_v2262 { (self.scalar_v2301 * v8012) } else { v4 });
        let v8594: f64 = (if self.scalar_v2262 { (v7657 + (self.scalar_v2301 * v8013)) } else { v4 });
        let v8595: f64 = (if self.scalar_v2262 { (v7658 + v8584) } else { v4 });
        let v8596: f64 = (if self.scalar_v2262 { (v7659 + v8584) } else { v4 });
        let v8597: f64 = (if self.scalar_v2262 { (self.scalar_v2301 * v8015) } else { v4 });
        let v8631: f64 = (if self.scalar_v2315 { v7651 } else { (if self.scalar_v2262 { (self.scalar_v2312 * v8591) } else { v4 }) });
        let v8632: f64 = (if self.scalar_v2315 { v7654 } else { (if self.scalar_v2262 { (self.scalar_v2312 * v8592) } else { v4 }) });
        let v8633: f64 = (if self.scalar_v2315 { v4 } else { (if self.scalar_v2262 { (self.scalar_v2312 * v8593) } else { v4 }) });
        let v8634: f64 = (if self.scalar_v2315 { v7657 } else { (if self.scalar_v2262 { (self.scalar_v2312 * v8594) } else { v4 }) });
        let v8635: f64 = (if self.scalar_v2315 { v7658 } else { (if self.scalar_v2262 { (self.scalar_v2312 * v8595) } else { v4 }) });
        let v8636: f64 = (if self.scalar_v2315 { v7659 } else { (if self.scalar_v2262 { (self.scalar_v2312 * v8596) } else { v4 }) });
        let v8637: f64 = (if self.scalar_v2315 { v4 } else { (if self.scalar_v2262 { (self.scalar_v2312 * v8597) } else { v4 }) });
        let v8638: f64 = (if self.scalar_v2315 { v7668 } else { (if self.scalar_v2262 { (v7668 + (self.scalar_v2308 * v8591)) } else { v4 }) });
        let v8639: f64 = (if self.scalar_v2315 { v7669 } else { (if self.scalar_v2262 { (v7669 + (self.scalar_v2308 * v8592)) } else { v4 }) });
        let v8640: f64 = (if self.scalar_v2315 { v4 } else { (if self.scalar_v2262 { (self.scalar_v2308 * v8593) } else { v4 }) });
        let v8641: f64 = (if self.scalar_v2315 { v7672 } else { (if self.scalar_v2262 { (v7672 + (self.scalar_v2308 * v8594)) } else { v4 }) });
        let v8642: f64 = (if self.scalar_v2315 { v7675 } else { (if self.scalar_v2262 { (v7675 + (self.scalar_v2308 * v8595)) } else { v4 }) });
        let v8643: f64 = (if self.scalar_v2315 { v7678 } else { (if self.scalar_v2262 { (v7678 + (self.scalar_v2308 * v8596)) } else { v4 }) });
        let v8644: f64 = (if self.scalar_v2315 { v4 } else { (if self.scalar_v2262 { (self.scalar_v2308 * v8597) } else { v4 }) });
        let v8649: f64 = (if self.scalar_v2315 { v8014 } else { (if self.scalar_v2262 { (self.scalar_v2302 * v8014) } else { v4 }) });
        let v8651: f64 = ddt_scale;
        let v8653: f64 = (self.scalar_v27 * (self.scalar_v2319 * v8651));
        let v8668: f64 = (if self.scalar_v2352 { self.scalar_v8667 } else { (if self.scalar_v2344 { (self.scalar_v2347 * (self.scalar_v8657 * (self.scalar_v2323 * f64::powf(v2339, self.scalar_v8661)))) } else { (if self.scalar_v2335 { (self.scalar_v2337 * (self.scalar_v8657 / v2339)) } else { self.scalar_v8656 }) }) });
        let v8690: f64 = (v2356 * v2356);
        let v8749: f64 = (if v2368 { ((v2369 * v4409) + (v1192 * ((v1730 * v2941) + (v626 * v6128)))) } else { (if v2364 { (((v2356 * (v8631 + v8638)) - (v2365 * (((v1192 * (v4425 + v4431)) - (v2355 * v4409)) / v4441))) / v8690) } else { v4 }) });
        let v8750: f64 = (if v2368 { ((v2369 * v4412) + (v1192 * (v626 * v6129))) } else { (if v2364 { (((v2356 * (v8632 + v8639)) - (v2365 * ((v4443 - (v2355 * v4412)) / v4441))) / v8690) } else { v4 }) });
        let v8751: f64 = (if v2368 { v4 } else { (if v2364 { ((v8633 + v8640) / v2356) } else { v4 }) });
        let v8752: f64 = (if v2368 { ((v2369 * v4415) + (v1192 * (v626 * v6130))) } else { (if v2364 { (((v2356 * (v8634 + v8641)) - (v2365 * (((v1192 * (v4426 + v4433)) - (v2355 * v4415)) / v4441))) / v8690) } else { v4 }) });
        let v8753: f64 = (if v2368 { ((v2369 * v4418) + (v1192 * (v626 * v6131))) } else { (if v2364 { (((v2356 * (v8635 + v8642)) - (v2365 * (((v1192 * v4427) - (v2355 * v4418)) / v4441))) / v8690) } else { v4 }) });
        let v8754: f64 = (if v2368 { ((v2369 * v4421) + (v1192 * (v626 * v6132))) } else { (if v2364 { (((v2356 * (v8636 + v8643)) - (v2365 * (((v1192 * v4428) - (v2355 * v4421)) / v4441))) / v8690) } else { v4 }) });
        let v8755: f64 = (if v2368 { v4 } else { (if v2364 { ((v8637 + v8644) / v2356) } else { v4 }) });
        let v8815: f64 = (((v2173 * ((v2171 * v3886) + (v1048 * (v411 * v8023)))) + (v2172 * v3834)) + (((v2079 * v4180) + (v1138 * (self.scalar_v2078 * v2663))) + v8638));
        let v8819: f64 = ((self.scalar_v14 * (self.scalar_v2116 * ((v2114 * v2663) + (v300 * (((v2111 * v4005) + (v1099 * v7792)) + (v727 * v2664)))))) + (if self.scalar_v2210 { (self.scalar_v14 * v8159) } else { v8159 }));
        let v8826: f64 = ((self.scalar_v13 * (self.scalar_v2116 * ((v2146 * v2663) + (v300 * (((v2143 * v4005) + (v1099 * v7935)) + (v732 * v2664)))))) + (if self.scalar_v2210 { ((v2258 * v5776) + (v1662 * v8368)) } else { v4 }));
        let v8836: f64 = (self.scalar_v27 * (self.scalar_v0 * v3217));
        let v8837: f64 = (self.scalar_v27 * (self.scalar_v0 * v3218));
        let v8838: f64 = (self.scalar_v27 * (self.scalar_v0 * v3219));
        let v8839: f64 = (self.scalar_v27 * (self.scalar_v0 * v3220));
        let v8845: f64 = (self.scalar_v27 * (self.scalar_v0 * v4442));
        let v8846: f64 = (self.scalar_v27 * (self.scalar_v0 * v4446));
        let v8847: f64 = (self.scalar_v27 * (self.scalar_v0 * v4450));
        let v8848: f64 = (self.scalar_v27 * (self.scalar_v0 * v4454));
        let v8849: f64 = (self.scalar_v27 * (self.scalar_v0 * v4458));
        let v8856: f64 = (self.scalar_v27 * (self.scalar_v0 * v7455));
        let v8857: f64 = (self.scalar_v27 * (self.scalar_v0 * v7456));
        let v8858: f64 = (self.scalar_v27 * (self.scalar_v0 * v7457));
        let v8859: f64 = (self.scalar_v27 * (self.scalar_v0 * v7458));
        let v8860: f64 = (self.scalar_v27 * (self.scalar_v0 * v4921));
        let v8861: f64 = (self.scalar_v27 * (self.scalar_v0 * v4922));
        let v8868: f64 = (self.scalar_v27 * (self.scalar_v0 * v7407));
        let v8869: f64 = (self.scalar_v27 * (self.scalar_v0 * v7408));
        let v8870: f64 = (self.scalar_v27 * (self.scalar_v0 * v4834));
        let v8871: f64 = (self.scalar_v27 * (self.scalar_v0 * v7409));
        let v8872: f64 = (self.scalar_v27 * (self.scalar_v0 * v4726));
        let v8873: f64 = (self.scalar_v27 * (self.scalar_v0 * v4727));
        let v8894: f64 = (self.scalar_v27 * (self.scalar_v0 * (-v5987)));
        let v8895: f64 = (self.scalar_v27 * (self.scalar_v0 * (-v5988)));
        let v8896: f64 = (self.scalar_v27 * (self.scalar_v0 * (-v5991)));
        let v8897: f64 = (self.scalar_v27 * (self.scalar_v0 * (-v5992)));
        let v8898: f64 = (self.scalar_v27 * (self.scalar_v0 * (-v5993)));
        let v8899: f64 = (self.scalar_v27 * (self.scalar_v0 * (-v5996)));
        let v8900: f64 = (self.scalar_v27 * (self.scalar_v0 * (-v5999)));
        let v8901: f64 = (self.scalar_v27 * (self.scalar_v0 * (-v6000)));
        let v8902: f64 = (self.scalar_v27 * (self.scalar_v0 * (-v6001)));
        let v8903: f64 = (self.scalar_v27 * (self.scalar_v0 * (-v6002)));
        let v8904: f64 = (if self.scalar_v497 { v8894 } else { v4 });
        let v8905: f64 = (if self.scalar_v497 { v8895 } else { v4 });
        let v8906: f64 = (if self.scalar_v497 { v8896 } else { v4 });
        let v8907: f64 = (if self.scalar_v497 { v8897 } else { v4 });
        let v8908: f64 = (if self.scalar_v497 { v8898 } else { v4 });
        let v8909: f64 = (if self.scalar_v497 { v8899 } else { v4 });
        let v8910: f64 = (if self.scalar_v497 { v8900 } else { v4 });
        let v8911: f64 = (if self.scalar_v497 { v8901 } else { v4 });
        let v8912: f64 = (if self.scalar_v497 { v8902 } else { v4 });
        let v8913: f64 = (if self.scalar_v497 { v8903 } else { v4 });
        let v8914: f64 = (if self.scalar_v1311 { v8894 } else { v4 });
        let v8915: f64 = (if self.scalar_v1311 { v8895 } else { v4 });
        let v8916: f64 = (if self.scalar_v1311 { v8896 } else { v4 });
        let v8917: f64 = (if self.scalar_v1311 { v8897 } else { v4 });
        let v8918: f64 = (if self.scalar_v1311 { v8898 } else { v4 });
        let v8919: f64 = (if self.scalar_v1311 { v8899 } else { v4 });
        let v8920: f64 = (if self.scalar_v1311 { v8900 } else { v4 });
        let v8921: f64 = (if self.scalar_v1311 { v8901 } else { v4 });
        let v8922: f64 = (if self.scalar_v1311 { v8902 } else { v4 });
        let v8923: f64 = (if self.scalar_v1311 { v8903 } else { v4 });
        let v8930: f64 = (self.scalar_v27 * (self.scalar_v0 * v6186));
        let v8931: f64 = (self.scalar_v27 * (self.scalar_v0 * v6189));
        let v8932: f64 = (self.scalar_v27 * (self.scalar_v0 * v6190));
        let v8933: f64 = (self.scalar_v27 * (self.scalar_v0 * v6194));
        let v8934: f64 = (self.scalar_v27 * (self.scalar_v0 * v6197));
        let v8935: f64 = (self.scalar_v27 * (self.scalar_v0 * v6200));
        let v8941: f64 = (self.scalar_v27 * (self.scalar_v0 * (-v7240)));
        let v8942: f64 = (self.scalar_v27 * (self.scalar_v0 * (-v7241)));
        let v8943: f64 = (self.scalar_v27 * (self.scalar_v0 * (-v7242)));
        let v8944: f64 = (self.scalar_v27 * (self.scalar_v0 * (-v7243)));
        let v8945: f64 = (self.scalar_v27 * (self.scalar_v0 * (-v7244)));
        let v8953: f64 = (self.scalar_v27 * (self.scalar_v8946 / v308));
        let v8954: f64 = (self.scalar_v27 * ((-(v2412 * v2668)) / v7329));
        let v8955: f64 = (self.scalar_v27 * (self.scalar_v8947 / v308));
        let v8961: f64 = (self.scalar_v27 * (self.scalar_v8946 / v322));
        let v8962: f64 = (self.scalar_v27 * ((-(v2415 * v2675)) / v7374));
        let v8963: f64 = (self.scalar_v27 * (self.scalar_v8947 / v322));
        let v8975: f64 = (self.scalar_v27 * (-((((v674 * (v2442 + v2442)) - (v2001 * v5987)) + (v727 * v7475)) + (v7521 + (v732 * v6049)))));
        let v8976: f64 = (self.scalar_v27 * (-((((v7342 + ((v2415 + v2415) / v322)) - (v2001 * v5988)) + (v727 * v7476)) + ((v1719 * self.scalar_v2982) + (v732 * v6052)))));
        let v8977: f64 = (self.scalar_v27 * (-((v2412 + v2412) / v308)));
        let v8978: f64 = (self.scalar_v27 * (-((((v7419 - (v2001 * v5991)) + (v703 * v7455)) + (v727 * v7477)) + (v732 * v6055))));
        let v8979: f64 = (self.scalar_v27 * (-((((v7420 - (v2001 * v5992)) + ((v2037 * self.scalar_v2979) + (v703 * v7456))) + (v727 * v7478)) + (v732 * v6058))));
        let v8980: f64 = (self.scalar_v27 * (-((v7470 + ((self.scalar_v0 * v2042) + (v727 * (self.scalar_v7399 + (v6011 + v6034))))) + (v7521 + (v732 * v6060)))));
        let v8981: f64 = (self.scalar_v27 * (-(((v7446 + (v703 * v7458)) + ((v2042 * self.scalar_v2980) + (v727 * ((v6014 + v6037) + self.scalar_v7485)))) + (v7521 + (v732 * v6063)))));
        let v8982: f64 = (self.scalar_v27 * (-(((v7447 + v7466) + (v7502 + (v727 * ((v6017 + v6040) + self.scalar_v7486)))) + (v7533 + (v732 * v6066)))));
        let v8983: f64 = (self.scalar_v27 * (-(v7518 + (v7533 + (v732 * v6069)))));
        let v8984: f64 = (self.scalar_v27 * (-v7551));
        let v8985: f64 = (self.scalar_v27 * (-(v7520 + (v7533 + (v732 * v6075)))));
        let v9000: f64 = (self.scalar_v27 * (v8651 * (self.scalar_v0 * ((if self.scalar_v2315 { v8010 } else { (if self.scalar_v2262 { (self.scalar_v2302 * v8010) } else { v4 }) }) + (((v2049 * v3990) + (v1086 * v7553)) + v8631)))));
        let v9001: f64 = (self.scalar_v27 * (v8651 * (self.scalar_v0 * ((if self.scalar_v2315 { v8011 } else { (if self.scalar_v2262 { (self.scalar_v2302 * v8011) } else { v4 }) }) + ((v2049 * v3991) + v8632)))));
        let v9002: f64 = (self.scalar_v27 * (v8651 * (self.scalar_v0 * (v8633 + (if self.scalar_v2315 { v8012 } else { (if self.scalar_v2262 { (self.scalar_v2302 * v8012) } else { v4 }) })))));
        let v9003: f64 = (self.scalar_v27 * (v8651 * (self.scalar_v0 * ((if self.scalar_v2315 { v8013 } else { (if self.scalar_v2262 { (self.scalar_v2302 * v8013) } else { v4 }) }) + ((v2049 * v3992) + v8634)))));
        let v9004: f64 = (self.scalar_v27 * (v8651 * (self.scalar_v0 * (v8635 + v8649))));
        let v9005: f64 = (self.scalar_v27 * (v8651 * (self.scalar_v0 * (v8636 + v8649))));
        let v9006: f64 = (self.scalar_v27 * (v8651 * (self.scalar_v0 * (v8637 + (if self.scalar_v2315 { v8015 } else { (if self.scalar_v2262 { (self.scalar_v2302 * v8015) } else { v4 }) })))));
        let v9013: f64 = (self.scalar_v27 * (v8651 * (self.scalar_v0 * v7630)));
        let v9014: f64 = (self.scalar_v27 * (v8651 * (self.scalar_v0 * (v2068 * ((v1081 * (-((-(v284 * v7595)) * v7607))) + (v170 * (self.scalar_v2979 - v7595)))))));
        let v9015: f64 = (self.scalar_v27 * (v8651 * (self.scalar_v0 * (v2068 * ((v1081 * (-((-(v284 * v7596)) * v7607))) + (v170 * (self.scalar_v0 - v7596)))))));
        let v9030: f64 = (self.scalar_v27 * (v8651 * (self.scalar_v0 * v8815)));
        let v9031: f64 = (self.scalar_v27 * (v8651 * (self.scalar_v0 * v8639)));
        let v9032: f64 = (self.scalar_v27 * (v8651 * (self.scalar_v0 * v8640)));
        let v9033: f64 = (self.scalar_v27 * (v8651 * (self.scalar_v0 * (((v2173 * (v2171 * v3887)) + (v2172 * v3835)) + ((v2079 * v4181) + v8641)))));
        let v9034: f64 = (self.scalar_v27 * (v8651 * (self.scalar_v0 * (((v2173 * (v2171 * v3888)) + (v2172 * v3836)) + ((v2079 * v4182) + v8642)))));
        let v9035: f64 = (self.scalar_v27 * (v8651 * (self.scalar_v0 * (((v2173 * (v2171 * v3889)) + (v2172 * v3829)) + ((v2079 * v4176) + v8643)))));
        let v9036: f64 = (self.scalar_v27 * (v8651 * (self.scalar_v0 * v8644)));
        let v9051: f64 = (self.scalar_v27 * (v8651 * (self.scalar_v0 * (if self.scalar_v2262 { (v2296 * ((if self.scalar_v2262 { (((v2156 * v8010) - (v2167 * v7983)) / v7986) } else { v4 }) + v8542)) } else { v4 }))));
        let v9052: f64 = (self.scalar_v27 * (v8651 * (self.scalar_v0 * v8562)));
        let v9053: f64 = (self.scalar_v27 * (v8651 * (self.scalar_v0 * (if self.scalar_v2262 { ((v2298 * self.scalar_v8540) + (v2296 * (if self.scalar_v2262 { (v8012 / v2156) } else { v4 }))) } else { v4 }))));
        let v9054: f64 = (self.scalar_v27 * (v8651 * (self.scalar_v0 * (if self.scalar_v2262 { v8557 } else { v4 }))));
        let v9055: f64 = (self.scalar_v27 * (v8651 * (self.scalar_v0 * (if self.scalar_v2262 { (v2296 * ((if self.scalar_v2262 { (v2290 * (v2082 * v6131)) } else { v4 }) + v8538)) } else { v4 }))));
        let v9056: f64 = (self.scalar_v27 * (v8651 * (self.scalar_v0 * (if self.scalar_v2262 { (v2296 * ((if self.scalar_v2262 { (v2290 * (v2082 * v6132)) } else { v4 }) + v8538)) } else { v4 }))));
        let v9057: f64 = (self.scalar_v27 * (v8651 * (self.scalar_v0 * (if self.scalar_v2262 { (v2296 * (if self.scalar_v2262 { (v8015 / v2156) } else { v4 })) } else { v4 }))));
        let v9062: f64 = (self.scalar_v27 * (v8651 * self.scalar_v9058));
        let v9063: f64 = (self.scalar_v27 * (v8651 * self.scalar_v9059));
        let v9068: f64 = (self.scalar_v27 * (v8651 * self.scalar_v9064));
        let v9069: f64 = (self.scalar_v27 * (v8651 * self.scalar_v9065));
        let v9080: f64 = (self.scalar_v27 * (self.scalar_v0 * v6049));
        let v9081: f64 = (self.scalar_v27 * (self.scalar_v0 * v6052));
        let v9082: f64 = (self.scalar_v27 * (self.scalar_v0 * v6055));
        let v9083: f64 = (self.scalar_v27 * (self.scalar_v0 * v6058));
        let v9084: f64 = (self.scalar_v27 * (self.scalar_v0 * v6060));
        let v9085: f64 = (self.scalar_v27 * (self.scalar_v0 * v6063));
        let v9086: f64 = (self.scalar_v27 * (self.scalar_v0 * v6066));
        let v9087: f64 = (self.scalar_v27 * (self.scalar_v0 * v6069));
        let v9088: f64 = (self.scalar_v27 * (self.scalar_v0 * v6072));
        let v9089: f64 = (self.scalar_v27 * (self.scalar_v0 * v6075));
        let v9097: f64 = (self.scalar_v27 * (v674 * self.scalar_v8946));
        let v9098: f64 = (self.scalar_v27 * (v674 * self.scalar_v9090));
        let v9099: f64 = (self.scalar_v27 * (v2442 * v2966));
        let v9100: f64 = (self.scalar_v27 * (v674 * self.scalar_v9091));
        let v9101: f64 = (self.scalar_v27 * (v674 * self.scalar_v8947));
        let v9103: f64 = (self.scalar_v0 * ((self.scalar_v13 * v7960) + (if self.scalar_v2210 { ((v2258 * v5775) + (v1662 * (if self.scalar_v2240 { v8353 } else { (if self.scalar_v2213 { ((v2230 * v8277) / v633) } else { v4 }) }))) } else { v4 })));
        let v9118: f64 = (v8651 * (self.scalar_v0 * (v7968 + (if self.scalar_v2210 { ((v2258 * v5781) + (v1662 * (if self.scalar_v2240 { v8365 } else { (if self.scalar_v2213 { ((v2230 * v8280) / v633) } else { v4 }) }))) } else { v4 }))));
        let v9120: f64 = (self.scalar_v27 * (v8651 * (self.scalar_v0 * (v7964 + (if self.scalar_v2210 { ((v2258 * v5774) + v8372) } else { v4 })))));
        let v9121: f64 = (self.scalar_v27 * (v8651 * v9103));
        let v9122: f64 = (self.scalar_v27 * (v8651 * (self.scalar_v0 * v8826)));
        let v9123: f64 = (self.scalar_v27 * (v8651 * (self.scalar_v0 * (if self.scalar_v2210 { (v2258 * v5777) } else { v4 }))));
        let v9124: f64 = (self.scalar_v27 * (v8651 * (self.scalar_v0 * (v7964 + (if self.scalar_v2210 { (v8372 + (v2258 * v5778)) } else { v4 })))));
        let v9125: f64 = (self.scalar_v27 * (v8651 * (self.scalar_v0 * (v7967 + (if self.scalar_v2210 { ((v2258 * v5779) + v8384) } else { v4 })))));
        let v9126: f64 = (self.scalar_v27 * (v8651 * (self.scalar_v0 * (v7967 + (if self.scalar_v2210 { (v8384 + (v2258 * v5780)) } else { v4 })))));
        let v9127: f64 = (self.scalar_v27 * v9118);
        let v9128: f64 = (self.scalar_v27 * (v8651 * (self.scalar_v0 * (v7967 + (if self.scalar_v2210 { (v8384 + (v2258 * v5782)) } else { v4 })))));
        let v9144: f64 = (self.scalar_v27 * (self.scalar_v0 * v7475));
        let v9145: f64 = (self.scalar_v27 * (self.scalar_v0 * v7476));
        let v9146: f64 = (self.scalar_v27 * (self.scalar_v0 * v7477));
        let v9147: f64 = (self.scalar_v27 * (self.scalar_v0 * v7478));
        let v9148: f64 = (self.scalar_v27 * (self.scalar_v0 * (v6011 + (v6034 + self.scalar_v7399))));
        let v9149: f64 = (self.scalar_v27 * (self.scalar_v0 * (v6014 + (v6037 + self.scalar_v7485))));
        let v9150: f64 = (self.scalar_v27 * (self.scalar_v0 * (v6017 + (v6040 + self.scalar_v7486))));
        let v9151: f64 = (self.scalar_v27 * (self.scalar_v0 * (v6019 + (v6042 + self.scalar_v7486))));
        let v9152: f64 = (self.scalar_v27 * (self.scalar_v0 * v7483));
        let v9153: f64 = (self.scalar_v27 * (self.scalar_v0 * (v6023 + (v6046 + self.scalar_v7398))));
        let v9164: f64 = (self.scalar_v27 * (v8651 * (self.scalar_v0 * v8819)));
        let v9165: f64 = (self.scalar_v27 * (v8651 * (self.scalar_v0 * (v7825 + (if self.scalar_v2210 { (self.scalar_v14 * v8160) } else { v8160 })))));
        let v9166: f64 = (self.scalar_v27 * (v8651 * (self.scalar_v0 * (v7826 + (if self.scalar_v2210 { (self.scalar_v14 * v8161) } else { v8161 })))));
        let v9167: f64 = (self.scalar_v27 * (v8651 * (self.scalar_v0 * (v7827 + (if self.scalar_v2210 { (self.scalar_v14 * v8162) } else { v8162 })))));
        let v9168: f64 = (self.scalar_v27 * (v8651 * (self.scalar_v0 * (v7828 + (if self.scalar_v2210 { (self.scalar_v14 * v8163) } else { v8163 })))));
        let v9175: f64 = (if self.scalar_v675 { (self.scalar_v27 * (v2454 * v2972)) } else { v4 });
        let v9176: f64 = (if self.scalar_v675 { (self.scalar_v27 * (v682 * self.scalar_v8946)) } else { v4 });
        let v9177: f64 = (if self.scalar_v675 { (self.scalar_v27 * (v682 * self.scalar_v8947)) } else { v4 });
        let v9184: f64 = (if self.scalar_v683 { (self.scalar_v27 * (v2458 * v2978)) } else { v4 });
        let v9185: f64 = (if self.scalar_v683 { (self.scalar_v27 * (v690 * self.scalar_v8947)) } else { v4 });
        let v9186: f64 = (if self.scalar_v683 { (self.scalar_v27 * (v690 * self.scalar_v8946)) } else { v4 });
        let v9187: f64 = (v2463 * (if self.scalar_v2383 { v4 } else { (if self.scalar_v2378 { (self.scalar_v2379 * v8749) } else { (if self.scalar_v2373 { (self.scalar_v2308 * v8749) } else { v4 }) }) }));
        let v9188: f64 = (v2463 * (if self.scalar_v2383 { v4 } else { (if self.scalar_v2378 { (self.scalar_v2379 * v8750) } else { (if self.scalar_v2373 { (self.scalar_v2308 * v8750) } else { v4 }) }) }));
        let v9189: f64 = (v2463 * (if self.scalar_v2383 { v4 } else { (if self.scalar_v2378 { (self.scalar_v2379 * v8751) } else { (if self.scalar_v2373 { (self.scalar_v2308 * v8751) } else { v4 }) }) }));
        let v9190: f64 = (v2463 * (if self.scalar_v2383 { v4 } else { (if self.scalar_v2378 { (self.scalar_v2379 * v8752) } else { (if self.scalar_v2373 { (self.scalar_v2308 * v8752) } else { v4 }) }) }));
        let v9191: f64 = (v2463 * (if self.scalar_v2383 { v4 } else { (if self.scalar_v2378 { (self.scalar_v2379 * v8753) } else { (if self.scalar_v2373 { (self.scalar_v2308 * v8753) } else { v4 }) }) }));
        let v9192: f64 = (v2463 * (if self.scalar_v2383 { v4 } else { (if self.scalar_v2378 { (self.scalar_v2379 * v8754) } else { (if self.scalar_v2373 { (self.scalar_v2308 * v8754) } else { v4 }) }) }));
        let v9193: f64 = (v2463 * (if self.scalar_v2383 { v4 } else { (if self.scalar_v2378 { (self.scalar_v2379 * v8755) } else { (if self.scalar_v2373 { (self.scalar_v2308 * v8755) } else { v4 }) }) }));
        let v9194: f64 = (v2384 * v8651);

        let d2396_dn3: f64 = v8836;
        let d2396_dn6: f64 = v8837;
        let d2396_dn7: f64 = v8838;
        let d2396_dn8: f64 = v8839;
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(7),
            Some(8),
            multiplicity * (v2396),
            [3, 6, 7, 8],
            [d2396_dn3, d2396_dn6, d2396_dn7, d2396_dn8],
            [],
            [],
            multiplicity,
        );
        let d2398_dn3: f64 = v8845;
        let d2398_dn4: f64 = v8846;
        let d2398_dn6: f64 = v8847;
        let d2398_dn7: f64 = v8848;
        let d2398_dn8: f64 = v8849;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(8),
            Some(4),
            multiplicity * (v2398),
            [3, 4, 6, 7, 8],
            [d2398_dn3, d2398_dn4, d2398_dn6, d2398_dn7, d2398_dn8],
            [],
            [],
            multiplicity,
        );
        let d2400_dn3: f64 = v8856;
        let d2400_dn4: f64 = v8857;
        let d2400_dn5: f64 = v8858;
        let d2400_dn6: f64 = v8859;
        let d2400_dn7: f64 = v8860;
        let d2400_dn8: f64 = v8860;
        let d2400_dn10: f64 = v8861;
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(5),
            Some(4),
            multiplicity * (v2400),
            [3, 4, 5, 6, 7, 8, 10],
            [d2400_dn3, d2400_dn4, d2400_dn5, d2400_dn6, d2400_dn7, d2400_dn8, d2400_dn10],
            [],
            [],
            multiplicity,
        );
        let d2402_dn3: f64 = v8868;
        let d2402_dn4: f64 = v8869;
        let d2402_dn5: f64 = v8870;
        let d2402_dn6: f64 = v8871;
        let d2402_dn7: f64 = v8872;
        let d2402_dn8: f64 = v8873;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(6),
            Some(4),
            multiplicity * (v2402),
            [3, 4, 5, 6, 7, 8],
            [d2402_dn3, d2402_dn4, d2402_dn5, d2402_dn6, d2402_dn7, d2402_dn8],
            [],
            [],
            multiplicity,
        );
        let d2406_dn0: f64 = v8904;
        let d2406_dn1: f64 = v8905;
        let d2406_dn3: f64 = v8906;
        let d2406_dn4: f64 = v8907;
        let d2406_dn5: f64 = v8908;
        let d2406_dn6: f64 = v8909;
        let d2406_dn7: f64 = v8910;
        let d2406_dn8: f64 = v8911;
        let d2406_dn9: f64 = v8912;
        let d2406_dn10: f64 = v8913;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(6),
            Some(7),
            multiplicity * (v2406),
            [0, 1, 3, 4, 5, 6, 7, 8, 9, 10],
            [d2406_dn0, d2406_dn1, d2406_dn3, d2406_dn4, d2406_dn5, d2406_dn6, d2406_dn7, d2406_dn8, d2406_dn9, d2406_dn10],
            [],
            [],
            multiplicity,
        );
        let d2407_dn0: f64 = v8914;
        let d2407_dn1: f64 = v8915;
        let d2407_dn3: f64 = v8916;
        let d2407_dn4: f64 = v8917;
        let d2407_dn5: f64 = v8918;
        let d2407_dn6: f64 = v8919;
        let d2407_dn7: f64 = v8920;
        let d2407_dn8: f64 = v8921;
        let d2407_dn9: f64 = v8922;
        let d2407_dn10: f64 = v8923;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(6),
            Some(8),
            multiplicity * (v2407),
            [0, 1, 3, 4, 5, 6, 7, 8, 9, 10],
            [d2407_dn0, d2407_dn1, d2407_dn3, d2407_dn4, d2407_dn5, d2407_dn6, d2407_dn7, d2407_dn8, d2407_dn9, d2407_dn10],
            [],
            [],
            multiplicity,
        );
        let d2409_dn3: f64 = v8930;
        let d2409_dn4: f64 = v8931;
        let d2409_dn5: f64 = v8932;
        let d2409_dn6: f64 = v8933;
        let d2409_dn7: f64 = v8934;
        let d2409_dn8: f64 = v8935;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(5),
            Some(6),
            multiplicity * (v2409),
            [3, 4, 5, 6, 7, 8],
            [d2409_dn3, d2409_dn4, d2409_dn5, d2409_dn6, d2409_dn7, d2409_dn8],
            [],
            [],
            multiplicity,
        );
        let d2411_dn3: f64 = v8941;
        let d2411_dn4: f64 = v8942;
        let d2411_dn6: f64 = v8943;
        let d2411_dn7: f64 = v8944;
        let d2411_dn8: f64 = v8945;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(6),
            Some(8),
            multiplicity * (v2411),
            [3, 4, 6, 7, 8],
            [d2411_dn3, d2411_dn4, d2411_dn6, d2411_dn7, d2411_dn8],
            [],
            [],
            multiplicity,
        );
        let d2414_dn2: f64 = v8953;
        let d2414_dn3: f64 = v8954;
        let d2414_dn4: f64 = v8955;
        stamper.stamp_current_node3_local(
            Some(2),
            Some(4),
            multiplicity * (v2414),
            2,
            multiplicity * (d2414_dn2),
            3,
            multiplicity * (d2414_dn3),
            4,
            multiplicity * (d2414_dn4),
        );
        let d2417_dn1: f64 = v8961;
        let d2417_dn3: f64 = v8962;
        let d2417_dn5: f64 = v8963;
        stamper.stamp_current_node3_local(
            Some(1),
            Some(5),
            multiplicity * (v2417),
            1,
            multiplicity * (d2417_dn1),
            3,
            multiplicity * (d2417_dn3),
            5,
            multiplicity * (d2417_dn5),
        );
        let d2354_dn3: f64 = v8668;
        stamper.stamp_current_node1_local(
            Some(3),
            None,
            multiplicity * (v2354),
            3,
            multiplicity * (d2354_dn3),
        );
        let d2322_dn3: f64 = v8653;
        stamper.stamp_current_node1_local(
            Some(3),
            None,
            multiplicity * (v2322),
            3,
            multiplicity * (d2322_dn3),
        );
        let d2419_dn0: f64 = v8975;
        let d2419_dn1: f64 = v8976;
        let d2419_dn2: f64 = v8977;
        let d2419_dn3: f64 = v8978;
        let d2419_dn4: f64 = v8979;
        let d2419_dn5: f64 = v8980;
        let d2419_dn6: f64 = v8981;
        let d2419_dn7: f64 = v8982;
        let d2419_dn8: f64 = v8983;
        let d2419_dn9: f64 = v8984;
        let d2419_dn10: f64 = v8985;
        let v2419_node_derivative_indices: [usize; 11] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let v2419_node_derivatives: [f64; 11] = [d2419_dn0, d2419_dn1, d2419_dn2, d2419_dn3, d2419_dn4, d2419_dn5, d2419_dn6, d2419_dn7, d2419_dn8, d2419_dn9, d2419_dn10];
        let v2419_branch_derivative_indices: [usize; 0] = [];
        let v2419_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(3),
            None,
            multiplicity * (v2419),
            &v2419_node_derivative_indices,
            &v2419_node_derivatives,
            &v2419_branch_derivative_indices,
            &v2419_branch_derivatives,
            multiplicity,
        );
        let d2422_dn3: f64 = v9000;
        let d2422_dn4: f64 = v9001;
        let d2422_dn5: f64 = v9002;
        let d2422_dn6: f64 = v9003;
        let d2422_dn7: f64 = v9004;
        let d2422_dn8: f64 = v9005;
        let d2422_dn10: f64 = v9006;
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(6),
            Some(4),
            multiplicity * (v2422),
            [3, 4, 5, 6, 7, 8, 10],
            [d2422_dn3, d2422_dn4, d2422_dn5, d2422_dn6, d2422_dn7, d2422_dn8, d2422_dn10],
            [],
            [],
            multiplicity,
        );
        let d2425_dn3: f64 = v9013;
        let d2425_dn4: f64 = v9014;
        let d2425_dn5: f64 = v9015;
        stamper.stamp_current_node3_local(
            Some(5),
            Some(4),
            multiplicity * (v2425),
            3,
            multiplicity * (d2425_dn3),
            4,
            multiplicity * (d2425_dn4),
            5,
            multiplicity * (d2425_dn5),
        );
        let d2428_dn3: f64 = v9030;
        let d2428_dn4: f64 = v9031;
        let d2428_dn5: f64 = v9032;
        let d2428_dn6: f64 = v9033;
        let d2428_dn7: f64 = v9034;
        let d2428_dn8: f64 = v9035;
        let d2428_dn10: f64 = v9036;
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(6),
            Some(8),
            multiplicity * (v2428),
            [3, 4, 5, 6, 7, 8, 10],
            [d2428_dn3, d2428_dn4, d2428_dn5, d2428_dn6, d2428_dn7, d2428_dn8, d2428_dn10],
            [],
            [],
            multiplicity,
        );
        let d2431_dn3: f64 = v9051;
        let d2431_dn4: f64 = v9052;
        let d2431_dn5: f64 = v9053;
        let d2431_dn6: f64 = v9054;
        let d2431_dn7: f64 = v9055;
        let d2431_dn8: f64 = v9056;
        let d2431_dn10: f64 = v9057;
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(5),
            Some(6),
            multiplicity * (v2431),
            [3, 4, 5, 6, 7, 8, 10],
            [d2431_dn3, d2431_dn4, d2431_dn5, d2431_dn6, d2431_dn7, d2431_dn8, d2431_dn10],
            [],
            [],
            multiplicity,
        );
        let d2435_dn1: f64 = v9062;
        let d2435_dn2: f64 = v9063;
        stamper.stamp_current_node2_local(
            Some(1),
            Some(2),
            multiplicity * (v2435),
            1,
            multiplicity * (d2435_dn1),
            2,
            multiplicity * (d2435_dn2),
        );
        let d2439_dn0: f64 = v9068;
        let d2439_dn1: f64 = v9069;
        stamper.stamp_current_node2_local(
            Some(1),
            Some(0),
            multiplicity * (v2439),
            0,
            multiplicity * (d2439_dn0),
            1,
            multiplicity * (d2439_dn1),
        );
        let d2441_dn0: f64 = v9080;
        let d2441_dn1: f64 = v9081;
        let d2441_dn3: f64 = v9082;
        let d2441_dn4: f64 = v9083;
        let d2441_dn5: f64 = v9084;
        let d2441_dn6: f64 = v9085;
        let d2441_dn7: f64 = v9086;
        let d2441_dn8: f64 = v9087;
        let d2441_dn9: f64 = v9088;
        let d2441_dn10: f64 = v9089;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(1),
            Some(9),
            multiplicity * (v2441),
            [0, 1, 3, 4, 5, 6, 7, 8, 9, 10],
            [d2441_dn0, d2441_dn1, d2441_dn3, d2441_dn4, d2441_dn5, d2441_dn6, d2441_dn7, d2441_dn8, d2441_dn9, d2441_dn10],
            [],
            [],
            multiplicity,
        );
        let d2444_dn0: f64 = v9097;
        let d2444_dn1: f64 = v9098;
        let d2444_dn3: f64 = v9099;
        let d2444_dn5: f64 = v9098;
        let d2444_dn6: f64 = v9098;
        let d2444_dn7: f64 = v9100;
        let d2444_dn8: f64 = v9100;
        let d2444_dn9: f64 = v9101;
        let d2444_dn10: f64 = v9100;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(0),
            Some(9),
            multiplicity * (v2444),
            [0, 1, 3, 5, 6, 7, 8, 9, 10],
            [d2444_dn0, d2444_dn1, d2444_dn3, d2444_dn5, d2444_dn6, d2444_dn7, d2444_dn8, d2444_dn9, d2444_dn10],
            [],
            [],
            multiplicity,
        );
        let d2447_dn0: f64 = v9120;
        let d2447_dn1: f64 = v9121;
        let d2447_dn3: f64 = v9122;
        let d2447_dn4: f64 = v9123;
        let d2447_dn5: f64 = v9120;
        let d2447_dn6: f64 = v9124;
        let d2447_dn7: f64 = v9125;
        let d2447_dn8: f64 = v9126;
        let d2447_dn9: f64 = v9127;
        let d2447_dn10: f64 = v9128;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(1),
            Some(9),
            multiplicity * (v2447),
            [0, 1, 3, 4, 5, 6, 7, 8, 9, 10],
            [d2447_dn0, d2447_dn1, d2447_dn3, d2447_dn4, d2447_dn5, d2447_dn6, d2447_dn7, d2447_dn8, d2447_dn9, d2447_dn10],
            [],
            [],
            multiplicity,
        );
        let d2450_dn0: f64 = v9144;
        let d2450_dn1: f64 = v9145;
        let d2450_dn3: f64 = v9146;
        let d2450_dn4: f64 = v9147;
        let d2450_dn5: f64 = v9148;
        let d2450_dn6: f64 = v9149;
        let d2450_dn7: f64 = v9150;
        let d2450_dn8: f64 = v9151;
        let d2450_dn9: f64 = v9152;
        let d2450_dn10: f64 = v9153;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(5),
            Some(10),
            multiplicity * (v2450),
            [0, 1, 3, 4, 5, 6, 7, 8, 9, 10],
            [d2450_dn0, d2450_dn1, d2450_dn3, d2450_dn4, d2450_dn5, d2450_dn6, d2450_dn7, d2450_dn8, d2450_dn9, d2450_dn10],
            [],
            [],
            multiplicity,
        );
        let d2453_dn3: f64 = v9164;
        let d2453_dn5: f64 = v9165;
        let d2453_dn6: f64 = v9166;
        let d2453_dn7: f64 = v9167;
        let d2453_dn8: f64 = v9167;
        let d2453_dn10: f64 = v9168;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(5),
            Some(10),
            multiplicity * (v2453),
            [3, 5, 6, 7, 8, 10],
            [d2453_dn3, d2453_dn5, d2453_dn6, d2453_dn7, d2453_dn8, d2453_dn10],
            [],
            [],
            multiplicity,
        );
        let d2457_dn3: f64 = v9175;
        let d2457_dn9: f64 = v9176;
        let d2457_dn10: f64 = v9177;
        stamper.stamp_current_node3_local(
            Some(9),
            Some(10),
            multiplicity * (v2457),
            3,
            multiplicity * (d2457_dn3),
            9,
            multiplicity * (d2457_dn9),
            10,
            multiplicity * (d2457_dn10),
        );
        stamper.stamp_potential_branch_local(
            Some(9),
            Some(10),
            0,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            0,
            v4,
        );
        let d2461_dn3: f64 = v9184;
        let d2461_dn7: f64 = v9185;
        let d2461_dn10: f64 = v9186;
        stamper.stamp_current_node3_local(
            Some(10),
            Some(7),
            multiplicity * (v2461),
            3,
            multiplicity * (d2461_dn3),
            7,
            multiplicity * (d2461_dn7),
            10,
            multiplicity * (d2461_dn10),
        );
        stamper.stamp_potential_branch_local(
            Some(10),
            Some(7),
            1,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            1,
            v4,
        );
        stamper.stamp_current_const_local(
            Some(11),
            None,
            multiplicity * (v4),
        );
        let d2462_dn11: f64 = v1;
        stamper.stamp_current_node1_local(
            Some(11),
            None,
            multiplicity * (v2462),
            11,
            multiplicity * (d2462_dn11),
        );
        let d2464_dn3: f64 = v9187;
        let d2464_dn4: f64 = v9188;
        let d2464_dn5: f64 = v9189;
        let d2464_dn6: f64 = v9190;
        let d2464_dn7: f64 = v9191;
        let d2464_dn8: f64 = v9192;
        let d2464_dn10: f64 = v9193;
        let d2464_dn11: f64 = v9194;
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(6),
            Some(4),
            multiplicity * (v2464),
            [3, 4, 5, 6, 7, 8, 10, 11],
            [d2464_dn3, d2464_dn4, d2464_dn5, d2464_dn6, d2464_dn7, d2464_dn8, d2464_dn10, d2464_dn11],
            [],
            [],
            multiplicity,
        );
        let d2465_dn11: f64 = v2363;
        stamper.stamp_current_node1_local(
            Some(8),
            Some(6),
            multiplicity * (v2465),
            11,
            multiplicity * (d2465_dn11),
        );
        let d2462_dn11: f64 = v1;
        stamper.stamp_current_node1_local(
            Some(8),
            Some(4),
            multiplicity * (v2462),
            11,
            multiplicity * (d2462_dn11),
        );
        stamper.stamp_current_const_local(
            Some(8),
            Some(6),
            multiplicity * (v4),
        );
        stamper.stamp_current_const_local(
            Some(6),
            Some(4),
            multiplicity * (v4),
        );
        stamper.stamp_current_const_local(
            Some(2),
            Some(4),
            multiplicity * (v4),
        );
        stamper.stamp_current_const_local(
            Some(1),
            Some(5),
            multiplicity * (v4),
        );
        stamper.stamp_current_const_local(
            Some(5),
            Some(6),
            multiplicity * (v4),
        );
        stamper.stamp_current_const_local(
            Some(6),
            Some(4),
            multiplicity * (v4),
        );
        stamper.stamp_current_const_local(
            Some(5),
            Some(4),
            multiplicity * (v4),
        );
        stamper.stamp_current_const_local(
            Some(5),
            Some(4),
            multiplicity * (v4),
        );
        stamper.stamp_current_const_local(
            Some(5),
            Some(10),
            multiplicity * (v4),
        );
        stamper.stamp_current_const_local(
            Some(5),
            Some(10),
            multiplicity * (v4),
        );
        stamper.stamp_current_const_local(
            Some(5),
            Some(10),
            multiplicity * (v4),
        );
        stamper.stamp_current_const_local(
            Some(5),
            Some(10),
            multiplicity * (v4),
        );
        stamper.stamp_current_const_local(
            Some(1),
            Some(9),
            multiplicity * (v4),
        );
        stamper.stamp_current_const_local(
            Some(1),
            Some(9),
            multiplicity * (v4),
        );
        stamper.stamp_current_const_local(
            Some(7),
            Some(6),
            multiplicity * (v4),
        );
        stamper.stamp_current_const_local(
            Some(8),
            Some(6),
            multiplicity * (v4),
        );
        stamper.stamp_current_const_local(
            Some(0),
            Some(9),
            multiplicity * (v4),
        );
        stamper.stamp_current_const_local(
            Some(9),
            Some(10),
            multiplicity * (v4),
        );
        stamper.stamp_current_const_local(
            Some(10),
            Some(7),
            multiplicity * (v4),
        );
        stamper.stamp_current_const_local(
            Some(0),
            Some(9),
            multiplicity * (v4),
        );
        stamper.stamp_current_const_local(
            Some(9),
            Some(7),
            multiplicity * (v4),
        );
        stamper.stamp_current_const_local(
            Some(0),
            Some(10),
            multiplicity * (v4),
        );
        stamper.stamp_current_const_local(
            Some(10),
            Some(7),
            multiplicity * (v4),
        );
        stamper.stamp_current_const_local(
            Some(0),
            Some(7),
            multiplicity * (v4),
        );
    }

    pub fn stamp_reactive(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedReactiveStamper<'_>) {
        let nodes = self.nodes;
        let branches = self.branches;
        self.ensure_temperature_static(ctx.temperature(), ctx.thermal_voltage());
        let p = &(*self.params);
        let multiplicity = self.multiplicity;
        let v1: f64 = 1.0;
        let v4: f64 = 0.0;
        let v30: f64 = 0.001;
        let v31: f64 = 2.0;
        let v44: f64 = 0.05;
        let v46: f64 = 0.1;
        let v101: f64 = ctx.node_voltage(nodes[3]);
        let v102: bool = (v101 < v4);
        let v103: f64 = (v1 - v101);
        let v106: f64 = (if v102 { (-((v103) as f64).ln()) } else { v101 });
        let v108: bool = (v106 < self.scalar_v107);
        let v110: bool = (!v108);
        let v112: f64 = (v1 + (v106 - self.scalar_v107));
        let v116: f64 = (self.scalar_v20 + (if v110 { (self.scalar_v107 + ((v112) as f64).ln()) } else { (if v108 { v106 } else { v4 }) }));
        let v117: f64 = (v116 / self.scalar_v17);
        let v118: f64 = 8.617086918058125e-5;
        let v119: f64 = (v116 * v118);
        let v121: f64 = (v1 / v119);
        let v123: f64 = (v121 - self.scalar_v122);
        let v124: f64 = (v116 - self.scalar_v17);
        let v125: f64 = ((v117) as f64).ln();
        let v130: f64 = (self.scalar_v62 - ((v116 * (self.scalar_v37 * v116)) / (self.scalar_v40 + v116)));
        let v132: f64 = ((v130 - v44) / v46);
        let v133: bool = (v130 < v44);
        let v147: f64 = (if (!v133) { (v130 + (v46 * (((v1 + (((-v132)) as f64).exp())) as f64).ln())) } else { (if v133 { (v44 + (v46 * (((v1 + ((v132) as f64).exp())) as f64).ln())) } else { v4 }) });
        let v152: f64 = (self.scalar_v95 - ((v116 * (self.scalar_v72 * v116)) / (self.scalar_v75 + v116)));
        let v154: f64 = ((v152 - v44) / v46);
        let v155: bool = (v152 < v44);
        let v169: f64 = (if (!v155) { (v152 + (v46 * (((v1 + (((-v154)) as f64).exp())) as f64).ln())) } else { (if v155 { (v44 + (v46 * (((v1 + ((v154) as f64).exp())) as f64).ln())) } else { v4 }) });
        let v170: f64 = 3.0;
        let v171: f64 = -3.0;
        let v172: f64 = (v119 * v171);
        let v173: f64 = (v125 * v172);
        let v176: f64 = (v1 - v117);
        let v179: f64 = ((v173 + (self.scalar_v64 * v117)) + (v176 * self.scalar_v177));
        let v180: f64 = (v44 - v179);
        let v181: f64 = (v180 / v119);
        let v182: bool = (v44 < v179);
        let v183: f64 = ((v181) as f64).exp();
        let v184: f64 = (v1 + v183);
        let v185: f64 = ((v184) as f64).ln();
        let v189: bool = (!v182);
        let v191: f64 = (((-v181)) as f64).exp();
        let v192: f64 = (v1 + v191);
        let v193: f64 = ((v192) as f64).ln();
        let v196: f64 = (if v189 { (v44 + (v119 * v193)) } else { (if v182 { (v179 + (v119 * v185)) } else { v4 }) });
        let v201: f64 = (v176 * self.scalar_v200);
        let v202: f64 = ((v173 + (v117 * self.scalar_v197)) + v201);
        let v203: f64 = (v44 - v202);
        let v204: f64 = (v203 / v119);
        let v205: bool = (v44 < v202);
        let v206: f64 = ((v204) as f64).exp();
        let v207: f64 = (v1 + v206);
        let v208: f64 = ((v207) as f64).ln();
        let v212: bool = (!v205);
        let v214: f64 = (((-v204)) as f64).exp();
        let v215: f64 = (v1 + v214);
        let v216: f64 = ((v215) as f64).ln();
        let v219: f64 = (if v212 { (v44 + (v119 * v216)) } else { (if v205 { (v202 + (v119 * v208)) } else { v4 }) });
        let v223: f64 = (v201 + (v173 + (v117 * self.scalar_v220)));
        let v224: f64 = (v44 - v223);
        let v225: f64 = (v224 / v119);
        let v226: bool = (v44 < v223);
        let v227: f64 = ((v225) as f64).exp();
        let v228: f64 = (v1 + v227);
        let v229: f64 = ((v228) as f64).ln();
        let v233: bool = (!v226);
        let v235: f64 = (((-v225)) as f64).exp();
        let v236: f64 = (v1 + v235);
        let v237: f64 = ((v236) as f64).ln();
        let v240: f64 = (if v233 { (v44 + (v119 * v237)) } else { (if v226 { (v223 + (v119 * v229)) } else { v4 }) });
        let v243: f64 = (v201 + (v173 + (self.scalar_v66 * v117)));
        let v244: f64 = (v44 - v243);
        let v245: f64 = (v244 / v119);
        let v246: bool = (v44 < v243);
        let v247: f64 = ((v245) as f64).exp();
        let v248: f64 = (v1 + v247);
        let v249: f64 = ((v248) as f64).ln();
        let v253: bool = (!v246);
        let v255: f64 = (((-v245)) as f64).exp();
        let v256: f64 = (v1 + v255);
        let v257: f64 = ((v256) as f64).ln();
        let v260: f64 = (if v253 { (v44 + (v119 * v257)) } else { (if v246 { (v243 + (v119 * v249)) } else { v4 }) });
        let v266: f64 = ((v173 + (v117 * self.scalar_v261)) + (v176 * self.scalar_v264));
        let v267: f64 = (v44 - v266);
        let v268: f64 = (v267 / v119);
        let v269: bool = (v44 < v266);
        let v283: f64 = (if (!v269) { (v44 + (v119 * (((v1 + (((-v268)) as f64).exp())) as f64).ln())) } else { (if v269 { (v266 + (v119 * (((v1 + ((v268) as f64).exp())) as f64).ln())) } else { v4 }) });
        let v284: f64 = (v1 / v196);
        let v285: f64 = (v1 / v260);
        let v286: f64 = (self.scalar_v64 * v284);
        let v287: f64 = f64::powf(v286, self.scalar_v32);
        let v291: f64 = (v287 * self.scalar_v290);
        let v294: f64 = (self.scalar_v66 / v260);
        let v297: f64 = (self.scalar_v292 + (self.scalar_v293 * f64::powf(v294, self.scalar_v67)));
        let v298: f64 = (v1 / v297);
        let v300: f64 = (v297 * self.scalar_v299);
        let v301: f64 = (self.scalar_v292 * v298);
        let v326: f64 = (((v125 * self.scalar_v324)) as f64).exp();
        let v327: f64 = (self.scalar_v323 * v326);
        let v338: f64 = (((v125 * self.scalar_v336)) as f64).exp();
        let v339: f64 = (self.scalar_v335 * v338);
        let v346: f64 = (if self.scalar_v341 { (self.scalar_v342 * (v1 + (v124 * self.scalar_v340))) } else { v4 });
        let v349: f64 = (if self.scalar_v341 { ((v346 - v1) / v30) } else { v268 });
        let v350: bool = (v346 < v1);
        let v351: bool = (self.scalar_v341 && v350);
        let v352: f64 = ((v349) as f64).exp();
        let v353: f64 = (v1 + v352);
        let v357: f64 = (if v351 { (v1 + (v30 * ((v353) as f64).ln())) } else { v346 });
        let v359: bool = (self.scalar_v341 && (!v350));
        let v361: f64 = (((-v349)) as f64).exp();
        let v362: f64 = (v1 + v361);
        let v367: f64 = 0.0006931471805599453;
        let v371: f64 = (if self.scalar_v370 { self.scalar_v342 } else { (if self.scalar_v341 { ((if v359 { (v357 + (v30 * ((v362) as f64).ln())) } else { v357 }) - v367) } else { v4 }) });
        let v378: f64 = (if self.scalar_v373 { (self.scalar_v374 * (v1 + (v124 * self.scalar_v372))) } else { v4 });
        let v381: f64 = (if self.scalar_v373 { ((v378 - v1) / v30) } else { v349 });
        let v382: bool = (v378 < v1);
        let v383: bool = (self.scalar_v373 && v382);
        let v384: f64 = ((v381) as f64).exp();
        let v385: f64 = (v1 + v384);
        let v389: f64 = (if v383 { (v1 + (v30 * ((v385) as f64).ln())) } else { v378 });
        let v391: bool = (self.scalar_v373 && (!v382));
        let v393: f64 = (((-v381)) as f64).exp();
        let v394: f64 = (v1 + v393);
        let v402: f64 = (if self.scalar_v401 { self.scalar_v374 } else { (if self.scalar_v373 { ((if v391 { (v389 + (v30 * ((v394) as f64).ln())) } else { v389 }) - v367) } else { v4 }) });
        let v407: f64 = (self.scalar_v403 * (v1 + (v124 * self.scalar_v404)));
        let v408: f64 = 1e-6;
        let v409: f64 = (v407 * v407);
        let v410: bool = (v407 < v4);
        let v411: f64 = 0.5;
        let v414: f64 = (((v408 + v409)) as f64).sqrt();
        let v423: f64 = 4.0;
        let v428: f64 = (v125 * self.scalar_v427);
        let v430: f64 = (((v428 / v371)) as f64).exp();
        let v431: f64 = (self.scalar_v422 * v430);
        let v433: f64 = (v123 * self.scalar_v432);
        let v435: f64 = (((v433 / v371)) as f64).exp();
        let v436: f64 = (v431 * v435);
        let v440: f64 = (((v125 * self.scalar_v438)) as f64).exp();
        let v441: f64 = (self.scalar_v437 * v440);
        let v446: f64 = (((v125 * self.scalar_v444)) as f64).exp();
        let v447: f64 = (self.scalar_v442 * v446);
        let v449: f64 = 6.0;
        let v525: f64 = (((v125 * self.scalar_v523)) as f64).exp();
        let v526: f64 = (self.scalar_v521 * v525);
        let v530: f64 = (((v123 * self.scalar_v528)) as f64).exp();
        let v531: f64 = (v526 * v530);
        let v559: f64 = -0.5;
        let v561: f64 = (v1 / v287);
        let v570: f64 = (self.scalar_v63 * (self.scalar_v63 * (v284 * (self.scalar_v64 * (v561 * (f64::powf((self.scalar_v63 * v147), v559) * (v147 * (v147 * self.scalar_v562))))))));
        let v582: f64 = f64::powf((self.scalar_v96 * v169), v559);
        let v592: f64 = (self.scalar_v96 * (self.scalar_v96 * (v285 * (self.scalar_v66 * ((v1 / f64::powf((self.scalar_v66 * v285), self.scalar_v67)) * (v582 * (v169 * (v169 * self.scalar_v584))))))));
        let v604: f64 = (((v125 * self.scalar_v311)) as f64).exp();
        let v606: f64 = (v604 * self.scalar_v605);
        let v607: f64 = (v298 * v606);
        let v609: f64 = (v604 * self.scalar_v608);
        let v610: f64 = (v561 * v609);
        let v614: f64 = (((v125 * self.scalar_v612)) as f64).exp();
        let v615: f64 = (self.scalar_v611 * v614);
        let v619: f64 = (((v123 * self.scalar_v617)) as f64).exp();
        let v620: f64 = (v615 * v619);
        let v625: f64 = (((v125 * self.scalar_v623)) as f64).exp();
        let v626: f64 = (self.scalar_v621 * v625);
        let v630: f64 = (((v125 * self.scalar_v628)) as f64).exp();
        let v631: f64 = (self.scalar_v627 * v630);
        let v633: f64 = (v626 + v631);
        let v636: f64 = ((self.scalar_v632 * v633) / self.scalar_v635);
        let v641: f64 = (((v125 * self.scalar_v639)) as f64).exp();
        let v642: f64 = (self.scalar_v637 * v641);
        let v661: f64 = (v604 * self.scalar_v660);
        let v691: f64 = ctx.node_voltage(nodes[6]);
        let v692: f64 = ctx.node_voltage(nodes[7]);
        let v694: f64 = (self.scalar_v0 * (v691 - v692));
        let v695: f64 = ctx.node_voltage(nodes[8]);
        let v697: f64 = (self.scalar_v0 * (v691 - v695));
        let v698: f64 = ctx.node_voltage(nodes[4]);
        let v700: f64 = (self.scalar_v0 * (v691 - v698));
        let v701: f64 = ctx.node_voltage(nodes[5]);
        let v703: f64 = (self.scalar_v0 * (v701 - v698));
        let v705: f64 = (self.scalar_v0 * (v701 - v691));
        let v707: f64 = (self.scalar_v0 * (v692 - v695));
        let v711: f64 = ctx.node_voltage(nodes[1]);
        let v718: f64 = (self.scalar_v0 * (v711 - ctx.node_voltage(nodes[0])));
        let v719: f64 = ctx.node_voltage(nodes[10]);
        let v727: f64 = (((v697 + v705) - v707) - (self.scalar_v0 * (v719 - v692)));
        let v732: f64 = (v718 + ((v727 + ((self.scalar_v0 * (v711 - v701)) + (-v718))) - (self.scalar_v0 * (ctx.node_voltage(nodes[9]) - v719))));
        let v733: f64 = (v121 * v697);
        let v735: bool = (v733 < self.scalar_v734);
        let v736: f64 = ((v733) as f64).exp();
        let v738: bool = (!v735);
        let v740: f64 = (if v738 { self.scalar_v739 } else { v4 });
        let v745: f64 = (v121 * v700);
        let v746: f64 = (v745 / v371);
        let v747: bool = (v746 < self.scalar_v734);
        let v748: f64 = ((v746) as f64).exp();
        let v750: bool = (!v747);
        let v751: f64 = (if v750 { self.scalar_v739 } else { v740 });
        let v755: f64 = (if v750 { (v751 * (v1 + (v746 - self.scalar_v734))) } else { (if v747 { v748 } else { v4 }) });
        let v756: f64 = (v121 * v727);
        let v757: bool = (v756 < self.scalar_v734);
        let v758: f64 = ((v756) as f64).exp();
        let v760: bool = (!v757);
        let v761: f64 = (if v760 { self.scalar_v739 } else { v751 });
        let v765: f64 = (if v760 { (v761 * (v1 + (v756 - self.scalar_v734))) } else { (if v757 { v758 } else { v4 }) });
        let v776: f64 = (v121 * v732);
        let v777: bool = (v776 < self.scalar_v734);
        let v778: f64 = ((v776) as f64).exp();
        let v780: bool = (!v777);
        let v781: f64 = (if v780 { self.scalar_v739 } else { (if (!((v121 * v705) < self.scalar_v734)) { self.scalar_v739 } else { v761 }) });
        let v785: f64 = (if v780 { (v781 * (v1 + (v776 - self.scalar_v734))) } else { (if v777 { v778 } else { v4 }) });
        let v786: f64 = (v732 - v219);
        let v787: f64 = (v121 * v786);
        let v788: bool = (v787 < self.scalar_v734);
        let v789: f64 = ((v787) as f64).exp();
        let v791: bool = (!v788);
        let v792: f64 = (if v791 { self.scalar_v739 } else { v781 });
        let v797: f64 = (v727 - v219);
        let v798: f64 = (v121 * v797);
        let v799: bool = (v798 < self.scalar_v734);
        let v800: f64 = ((v798) as f64).exp();
        let v802: bool = (!v799);
        let v803: f64 = (if v802 { self.scalar_v739 } else { v792 });
        let v808: f64 = (v697 - v219);
        let v809: f64 = (v121 * v808);
        let v810: bool = (v809 < self.scalar_v734);
        let v811: f64 = ((v809) as f64).exp();
        let v813: bool = (!v810);
        let v814: f64 = (if v813 { self.scalar_v739 } else { v803 });
        let v818: f64 = (if v813 { (v814 * (v1 + (v809 - self.scalar_v734))) } else { (if v810 { v811 } else { v4 }) });
        let v819: f64 = (v694 - v219);
        let v820: f64 = (v121 * v819);
        let v821: bool = (v820 < self.scalar_v734);
        let v822: f64 = ((v820) as f64).exp();
        let v824: bool = (!v821);
        let v825: f64 = (if v824 { self.scalar_v739 } else { v814 });
        let v829: f64 = (if v824 { (v825 * (v1 + (v820 - self.scalar_v734))) } else { (if v821 { v822 } else { v4 }) });
        let v832: f64 = (((v1 + (v423 * v818))) as f64).sqrt();
        let v835: f64 = (((v1 + (v423 * v829))) as f64).sqrt();
        let v836: f64 = (v31 * v829);
        let v837: f64 = (v1 + v835);
        let v838: f64 = (v836 / v837);
        let v840: bool = (v838 < self.scalar_v839);
        let v841: f64 = (if v840 { self.scalar_v839 } else { v838 });
        let v843: f64 = (v1 + v832);
        let v844: f64 = (v843 / v837);
        let v846: f64 = ((v832 - v835) - ((v844) as f64).ln());
        let v847: f64 = (v119 * v846);
        let v848: f64 = (v707 + v847);
        let v849: f64 = (v848 / v339);
        let v850: bool = (v849 > v4);
        let v851: f64 = 100.0;
        let v852: bool = (v694 < v851);
        let v853: bool = (v850 && v852);
        let v856: bool = (v850 && (!v852));
        let v858: f64 = (v1 + (v694 - v851));
        let v862: f64 = (v31 * v119);
        let v863: f64 = (v411 * v849);
        let v864: f64 = (v339 * v863);
        let v866: f64 = (v1 + (v121 * v864));
        let v867: f64 = ((v866) as f64).ln();
        let v871: f64 = (if v850 { ((v219 + (v862 * v867)) - (if v856 { (v851 + ((v858) as f64).ln()) } else { (if v853 { v694 } else { v4 }) })) } else { v4 });
        let v872: f64 = 0.2;
        let v874: f64 = (if v850 { (v219 * v872) } else { v4 });
        let v876: f64 = (if v850 { (v874 * v874) } else { v408 });
        let v879: bool = (v871 < v4);
        let v880: bool = (v850 && v879);
        let v881: f64 = (v411 * v876);
        let v883: f64 = (((v876 + (if v850 { (v871 * v871) } else { v409 }))) as f64).sqrt();
        let v884: f64 = (v883 - v871);
        let v888: bool = (v850 && (!v879));
        let v891: f64 = (if v888 { (v411 * (v871 + v883)) } else { (if v880 { (v881 / v884) } else { v4 }) });
        let v895: f64 = (v891 + self.scalar_v894);
        let v896: f64 = (v891 * v895);
        let v899: f64 = (self.scalar_v893 * (v891 + (v339 * self.scalar_v892)));
        let v901: f64 = (if v850 { (v896 / v899) } else { v4 });
        let v903: f64 = (if v850 { (v849 / v901) } else { v4 });
        let v907: f64 = (if v850 { ((v903 - v1) / self.scalar_v905) } else { v381 });
        let v908: bool = (v903 < v1);
        let v909: bool = (v850 && v908);
        let v910: f64 = ((v907) as f64).exp();
        let v911: f64 = (v1 + v910);
        let v917: bool = (v850 && (!v908));
        let v919: f64 = (((-v907)) as f64).exp();
        let v920: f64 = (v1 + v919);
        let v933: f64 = (if v850 { ((if v917 { (v903 + (self.scalar_v905 * ((v920) as f64).ln())) } else { (if v909 { (v1 + (self.scalar_v905 * ((v911) as f64).ln())) } else { v4 }) }) / self.scalar_v931) } else { v4 });
        let v935: f64 = (if v850 { (v891 / self.scalar_v894) } else { v4 });
        let v936: f64 = (v423 * v933);
        let v937: f64 = (v935 * v936);
        let v938: f64 = (v1 + v935);
        let v941: f64 = (((v1 + (v937 * v938))) as f64).sqrt();
        let v942: f64 = (v1 + v941);
        let v943: f64 = (v31 * v933);
        let v944: f64 = (v938 * v943);
        let v946: f64 = (if v850 { (v942 / v944) } else { v4 });
        let v948: f64 = (v841 * v946);
        let v949: f64 = ((v1 - v946) + v948);
        let v950: f64 = (v1 + v948);
        let v952: f64 = (if v850 { (v949 / v950) } else { v4 });
        let v953: f64 = (v864 * v952);
        let v955: f64 = (if v850 { (v121 * v953) } else { v4 });
        let v958: f64 = (v1 + (v841 + v955));
        let v961: f64 = (if v850 { ((v31 * v955) + (v841 * v958)) } else { v4 });
        let v964: f64 = (if v850 { (v411 * (v955 - v1)) } else { v4 });
        let v968: bool = (v955 >= v1);
        let v969: bool = (v850 && v968);
        let v970: f64 = (((if v850 { (v961 + (v964 * v964)) } else { v4 })) as f64).sqrt();
        let v974: bool = (v850 && (!v968));
        let v975: f64 = (v970 - v964);
        let v977: f64 = (if v974 { (v961 / v975) } else { (if v969 { (v964 + v970) } else { v4 }) });
        let v980: bool = (v850 && (v977 < self.scalar_v978));
        let v981: f64 = (if v980 { self.scalar_v978 } else { v977 });
        let v982: f64 = (v1 + v981);
        let v983: f64 = (v981 * v982);
        let v985: f64 = (((v121 * v219)) as f64).exp();
        let v991: f64 = (if v850 { (self.scalar_v988 * (v849 - self.scalar_v892)) } else { v4 });
        let v993: f64 = (self.scalar_v892 * (v339 * self.scalar_v893));
        let v998: f64 = ((((if v850 { (v849 * v993) } else { v4 }) + (v991 * v991))) as f64).sqrt();
        let v1003: bool = (v850 && self.scalar_v1002);
        let v1004: f64 = (v46 * v260);
        let v1007: bool = (v850 && self.scalar_v1006);
        let v1008: f64 = (v31 * v849);
        let v1009: f64 = (v849 + v901);
        let v1011: f64 = (v46 + (v1008 / v1009));
        let v1014: f64 = (v849 * self.scalar_v892);
        let v1015: f64 = (v849 + self.scalar_v892);
        let v1020: bool = (!v850);
        let v1021: f64 = (v31 * v818);
        let v1024: f64 = (if v1020 { (if v738 { (v740 * (v1 + (v733 - self.scalar_v734))) } else { (if v735 { v736 } else { v4 }) }) } else { (if v850 { (v983 * v985) } else { v4 }) });
        let v1035: bool = ((((v707) as f64).abs() < (v119 * 1e-5)) || (((v847) as f64).abs() < ((v119 * 1e-40) * (v832 + v835))));
        let v1036: bool = (v1020 && v1035);
        let v1037: f64 = (v841 + (if v1020 { (v1021 / v843) } else { v981 }));
        let v1039: f64 = (if v1036 { (v411 * v1037) } else { v4 });
        let v1040: f64 = (v1 + v1039);
        let v1044: bool = (v1020 && (!v1035));
        let v1046: f64 = ((v697 + v847) - v694);
        let v1048: f64 = (if v1044 { (v847 / v1046) } else { (if v1036 { (v1039 / v1040) } else { v952 }) });
        let v1050: f64 = (if v1020 { v1004 } else { (if v1007 { (v260 * v1011) } else { (if v1003 { v1004 } else { v4 }) }) });
        let v1054: f64 = (if v1020 { (v1 - ((if v1020 { v849 } else { (if v850 { (v1014 / v1015) } else { v4 }) }) / self.scalar_v892)) } else { (if v850 { (self.scalar_v892 / v1015) } else { v4 }) });
        let v1058: f64 = (v196 * self.scalar_v1057);
        let v1059: f64 = (v46 * v196);
        let v1060: f64 = (v700 - v1058);
        let v1061: f64 = (v1060 / v1059);
        let v1062: bool = (v700 < v1058);
        let v1063: f64 = ((v1061) as f64).exp();
        let v1064: f64 = (v1 + v1063);
        let v1065: f64 = ((v1064) as f64).ln();
        let v1069: bool = (!v1062);
        let v1071: f64 = (((-v1061)) as f64).exp();
        let v1072: f64 = (v1 + v1071);
        let v1073: f64 = ((v1072) as f64).ln();
        let v1076: f64 = (if v1069 { (v1058 - (v1059 * v1073)) } else { (if v1062 { (v700 - (v1059 * v1065)) } else { v4 }) });
        let v1078: f64 = (v1 - (v284 * v1076));
        let v1080: f64 = f64::powf(v1078, self.scalar_v1079);
        let v1081: f64 = (v196 / self.scalar_v1079);
        let v1082: f64 = (v1 - v1080);
        let v1086: f64 = ((v1081 * v1082) + (v170 * (v700 - v1076)));
        let v1097: f64 = (if self.scalar_v1096 { v697 } else { (if self.scalar_v1092 { (v694 + (if v1020 { v707 } else { (if v850 { (v991 + v998) } else { v4 }) })) } else { (if self.scalar_v1088 { v694 } else { v4 }) }) });
        let v1098: f64 = (v31 - v301);
        let v1099: f64 = (v1 - v301);
        let v1100: f64 = (v1098 / v1099);
        let v1103: f64 = (v1 - f64::powf(v1100, self.scalar_v1101));
        let v1104: f64 = (v260 * v1103);
        let v1105: f64 = (v1097 - v1104);
        let v1106: f64 = (v1105 / v1050);
        let v1107: bool = (v1097 < v1104);
        let v1108: f64 = ((v1106) as f64).exp();
        let v1109: f64 = (v1 + v1108);
        let v1110: f64 = ((v1109) as f64).ln();
        let v1114: bool = (!v1107);
        let v1116: f64 = (((-v1106)) as f64).exp();
        let v1117: f64 = (v1 + v1116);
        let v1118: f64 = ((v1117) as f64).ln();
        let v1121: f64 = (if v1114 { (v1104 - (v1050 * v1118)) } else { (if v1107 { (v1097 - (v1050 * v1110)) } else { v4 }) });
        let v1123: f64 = f64::powf(v1054, self.scalar_v1122);
        let v1125: f64 = (v260 / self.scalar_v1124);
        let v1127: f64 = (v1 - (v1121 / v260));
        let v1128: f64 = f64::powf(v1127, self.scalar_v1124);
        let v1130: f64 = (v1 - (v1123 * v1128));
        let v1132: f64 = (v1100 * v1123);
        let v1133: f64 = (v1097 - v1121);
        let v1135: f64 = ((v1125 * v1130) + (v1132 * v1133));
        let v1138: f64 = ((v1099 * v1135) + (v301 * v694));
        let v1139: f64 = (v423 * v436);
        let v1140: f64 = (v1139 / v441);
        let v1141: f64 = (v755 * v1140);
        let v1143: f64 = (((v1 + v1141)) as f64).sqrt();
        let v1144: f64 = (v1 + v1143);
        let v1145: f64 = (v1141 / v1144);
        let v1146: f64 = (v1 / v402);
        let v1147: f64 = f64::powf(v1024, v1146);
        let v1148: f64 = (v1140 * v1147);
        let v1150: f64 = (((v1 + v1148)) as f64).sqrt();
        let v1151: f64 = (v1 + v1150);
        let v1152: f64 = (v1148 / v1151);
        let v1155: f64 = (v1 + (v1086 / v610));
        let v1157: f64 = (v1155 + (v1138 / v607));
        let v1160: f64 = (v661 * v1155);
        let v1163: f64 = (-v1138);
        let v1164: f64 = (v1163 / v607);
        let v1165: f64 = (v661 * v1164);
        let v1168: f64 = (((if self.scalar_v1159 { (v121 * v1160) } else { v4 })) as f64).exp();
        let v1169: f64 = (((if self.scalar_v1159 { (v121 * v1165) } else { v4 })) as f64).exp();
        let v1170: f64 = (v1168 - v1169);
        let v1172: f64 = (((v121 * v661)) as f64).exp();
        let v1173: f64 = (v1172 - v1);
        let v1175: f64 = (if self.scalar_v1159 { (v1170 / v1173) } else { (if self.scalar_v1153 { v1157 } else { v4 }) });
        let v1176: f64 = 0.010000000000000002;
        let v1177: f64 = (v1175 * v1175);
        let v1178: bool = (v1175 < v4);
        let v1179: f64 = 0.005000000000000001;
        let v1181: f64 = (((v1176 + v1177)) as f64).sqrt();
        let v1182: f64 = (v1181 - v1175);
        let v1185: bool = (!v1178);
        let v1188: f64 = (if v1185 { (v411 * (v1175 + v1181)) } else { (if v1178 { (v1179 / v1182) } else { v4 }) });
        let v1191: f64 = (v1 + (v411 * (v1145 + v1152)));
        let v1192: f64 = (v1188 * v1191);
        let v1194: f64 = (v436 * self.scalar_v1193);
        let v1195: f64 = (v1147 * v1194);
        let v1196: f64 = (v436 * v755);
        let v1198: f64 = ((v1196 - v1195) / v1192);
        let v1199: f64 = 0.0001;
        let v1200: f64 = (v700 / v1199);
        let v1201: bool = (v700 < v4);
        let v1202: f64 = ((v1200) as f64).exp();
        let v1203: f64 = (v1 + v1202);
        let v1207: bool = (!v1201);
        let v1209: f64 = (((-v1200)) as f64).exp();
        let v1210: f64 = (v1 + v1209);
        let v1214: f64 = (if v1207 { (v700 + (v1199 * ((v1210) as f64).ln())) } else { (if v1201 { (v1199 * ((v1203) as f64).ln()) } else { v4 }) });
        let v1251: f64 = (v745 / self.scalar_v478);
        let v1252: bool = (v1251 < self.scalar_v734);
        let v1253: f64 = ((v1251) as f64).exp();
        let v1255: bool = (!v1252);
        let v1256: f64 = (if v1255 { self.scalar_v739 } else { (if (!((v1214 / self.scalar_v1215) < self.scalar_v734)) { self.scalar_v739 } else { v825 }) });
        let v1285: f64 = (if (self.scalar_v497 && (!(((v1198 / v436) - 1000.0) < 40.0))) { 2.3538526683702e17 } else { (if (self.scalar_v497 && (!((v121 * (v700 - v283)) < self.scalar_v734))) { self.scalar_v739 } else { v1256 }) });
        let v1325: f64 = (v121 * v703);
        let v1326: f64 = (v1325 / self.scalar_v489);
        let v1327: bool = (v1326 < self.scalar_v734);
        let v1328: f64 = ((v1326) as f64).exp();
        let v1330: bool = (!v1327);
        let v1331: f64 = (if v1330 { self.scalar_v739 } else { v1285 });
        let v1335: f64 = (if v1330 { (v1331 * (v1 + (v1326 - self.scalar_v734))) } else { (if v1327 { v1328 } else { (if v1255 { (v1256 * (v1 + (v1251 - self.scalar_v734))) } else { (if v1252 { v1253 } else { v1214 }) }) }) });
        let v1361: f64 = (v745 / self.scalar_v450);
        let v1362: bool = (v1361 < self.scalar_v734);
        let v1363: f64 = ((v1361) as f64).exp();
        let v1365: bool = (!v1362);
        let v1366: f64 = (if v1365 { self.scalar_v739 } else { (if (self.scalar_v497 && (!((v121 * (v703 - v283)) < self.scalar_v734))) { self.scalar_v739 } else { v1331 }) });
        let v1373: f64 = (v1325 / self.scalar_v533);
        let v1374: bool = (v1373 < self.scalar_v734);
        let v1375: f64 = ((v1373) as f64).exp();
        let v1377: bool = (!v1374);
        let v1378: f64 = (if v1377 { self.scalar_v739 } else { v1366 });
        let v1382: f64 = (if v1377 { (v1378 * (v1 + (v1373 - self.scalar_v734))) } else { (if v1374 { v1375 } else { (if v1365 { (v1366 * (v1 + (v1361 - self.scalar_v734))) } else { (if v1362 { v1363 } else { v1335 }) }) }) });
        let v1385: f64 = (v756 / self.scalar_v463);
        let v1386: bool = (v1385 < self.scalar_v734);
        let v1387: f64 = ((v1385) as f64).exp();
        let v1389: bool = (!v1386);
        let v1390: f64 = (if v1389 { self.scalar_v739 } else { v1378 });
        let v1397: f64 = (v1325 / self.scalar_v543);
        let v1398: bool = (v1397 < self.scalar_v734);
        let v1399: f64 = ((v1397) as f64).exp();
        let v1401: bool = (!v1398);
        let v1402: f64 = (if v1401 { self.scalar_v739 } else { v1390 });
        let v1406: f64 = (if v1401 { (v1402 * (v1 + (v1397 - self.scalar_v734))) } else { (if v1398 { v1399 } else { (if v1389 { (v1390 * (v1 + (v1385 - self.scalar_v734))) } else { (if v1386 { v1387 } else { v1382 }) }) }) });
        let v1412: bool = (v1201 && self.scalar_v1411);
        let v1429: f64 = (if v1412 { (v284 * v700) } else { v604 });
        let v1431: f64 = 1e-30;
        let v1449: f64 = (f64::powf(((((v1429 * v1429) + v1431)) as f64).sqrt(), self.scalar_v1435) * ((self.scalar_v32 * (self.scalar_v1438 - ((v170 * v1429) * self.scalar_v1440))) - ((v1429 * (v449 * v1429)) * (v1429 + self.scalar_v1440))));
        let v1450: f64 = 0.16666666666666666;
        let v1457: f64 = (if v1412 { ((v570 * (self.scalar_v34 * v700)) / (v147 * (if v1412 { (v1449 * v1450) } else { v4 }))) } else { v1429 });
        let v1458: f64 = -0.001;
        let v1467: f64 = (if ((v1412 && (v1457 < v1458)) && (!(v1457 < self.scalar_v734))) { self.scalar_v739 } else { (if (v1412 && (!((v570 * (v1 - (self.scalar_v34 / (v31 * v1080)))) < self.scalar_v734))) { self.scalar_v739 } else { v1402 }) });
        let v1504: bool = (self.scalar_v1502 && (v694 < v4));
        let v1505: f64 = (v285 * v694);
        let v1524: f64 = (if v1504 { v1505 } else { v582 });
        let v1542: f64 = (f64::powf((((v1431 + (v1524 * v1524))) as f64).sqrt(), self.scalar_v1528) * ((self.scalar_v67 * (self.scalar_v1531 - ((v170 * v1524) * self.scalar_v1533))) - ((v1524 * (v449 * v1524)) * (v1524 + self.scalar_v1533))));
        let v1549: f64 = (if v1504 { ((v592 * (self.scalar_v69 * v694)) / (v169 * (if v1504 { (v1450 * v1542) } else { v4 }))) } else { v1524 });
        let v1558: f64 = (if ((v1504 && (v1549 < v1458)) && (!(v1549 < self.scalar_v734))) { self.scalar_v739 } else { (if (v1504 && (!((v592 * (v1 - (self.scalar_v69 / (v31 * (if v1504 { f64::powf((v1 - v1505), self.scalar_v1124) } else { v4 }))))) < self.scalar_v734))) { self.scalar_v739 } else { v1467 }) });
        let v1589: f64 = (v765 * v1140);
        let v1590: f64 = (v423 * (if v802 { (v803 * (v1 + (v798 - self.scalar_v734))) } else { (if v799 { v800 } else { v4 }) }));
        let v1591: f64 = (v1589 - v1140);
        let v1593: f64 = (((v1 + v1589)) as f64).sqrt();
        let v1594: f64 = (v1 + v1593);
        let v1595: f64 = (v1591 / v1594);
        let v1597: f64 = (((v1 + v1590)) as f64).sqrt();
        let v1598: f64 = (v1 + v1597);
        let v1599: f64 = (v1590 / v1598);
        let v1600: f64 = (v31 * v531);
        let v1603: f64 = (v423 * v531);
        let v1604: f64 = (v1603 / v447);
        let v1617: f64 = (v531 * self.scalar_v1616);
        let v1618: f64 = (v785 - v1);
        let v1619: f64 = (v1617 * v1618);
        let v1622: f64 = (((v1 + (v785 * v1604))) as f64).sqrt();
        let v1623: f64 = (v1 + v1622);
        let v1625: f64 = (if self.scalar_v1613 { (v1619 / v1623) } else { v4 });
        let v1628: f64 = (self.scalar_v13 * v531);
        let v1630: f64 = (if self.scalar_v1627 { (v327 * v1628) } else { v4 });
        let v1631: f64 = (v121 * v1630);
        let v1633: f64 = (v31 - ((v1631) as f64).ln());
        let v1637: f64 = (if self.scalar_v1627 { (v732 - (if self.scalar_v1627 { (v119 * v1633) } else { v4 })) } else { v4 });
        let v1642: bool = (v1637 < v4);
        let v1643: bool = (self.scalar_v1627 && v1642);
        let v1646: f64 = (((self.scalar_v1639 + (if self.scalar_v1627 { (v1637 * v1637) } else { v1177 }))) as f64).sqrt();
        let v1647: f64 = (v1646 - v1637);
        let v1651: bool = (self.scalar_v1627 && (!v1642));
        let v1654: f64 = (if v1651 { (v411 * (v1637 + v1646)) } else { (if v1643 { (self.scalar_v1644 / v1647) } else { v4 }) });
        let v1657: f64 = (v1654 + (v1630 + (v327 * v1625)));
        let v1662: f64 = (if self.scalar_v1661 { v1 } else { (if self.scalar_v1627 { (v1654 / v1657) } else { v1 }) });
        let v1721: bool = (v1157 < v4);
        let v1723: f64 = (((v1176 + (v1157 * v1157))) as f64).sqrt();
        let v1724: f64 = (v1723 - v1157);
        let v1727: bool = (!v1721);
        let v1730: f64 = (if v1727 { (v411 * (v1157 + v1723)) } else { (if v1721 { (v1179 / v1724) } else { v4 }) });
        let v1740: bool = (v1198 > v4);
        let v1744: bool = (v694 < self.scalar_v1743);
        let v1747: f64 = ((-v1198) / self.scalar_v1746);
        let v1748: bool = (v1747 < self.scalar_v734);
        let v1750: bool = (v1744 && (v1740 && self.scalar_v1742));
        let v1755: bool = (v1750 && (!v1748));
        let v1756: f64 = (if v1755 { self.scalar_v739 } else { v1558 });
        let v1761: f64 = (self.scalar_v1743 - v694);
        let v1764: f64 = (-(if (!v410) { (v411 * (v407 + v414)) } else { (if v410 { (5e-7 / (v414 - v407)) } else { v4 }) }));
        let v1767: f64 = (v1764 * f64::powf((if v1750 { ((if v1755 { (v1756 * (v1 + (v1747 - self.scalar_v734))) } else { (if (v1748 && v1750) { ((v1747) as f64).exp() } else { v4 }) }) * v1761) } else { v4 }), self.scalar_v1765));
        let v1891: bool = (v1744 && (self.scalar_v1887 && ((v1740 && self.scalar_v1786) && self.scalar_v1888)));
        let v1900: f64 = (if v1891 { (f64::powf(v1761, self.scalar_v1765) * f64::powf((v1 - (v1198 / (v1198 + self.scalar_v1893))), self.scalar_v1897)) } else { v4 });
        let v1903: bool = (self.scalar_v1807 && v1891);
        let v1907: f64 = (if v1903 { ((v1198 - self.scalar_v1904) / self.scalar_v1893) } else { v4 });
        let v1911: f64 = (if v1903 { ((v1907 - v1) / self.scalar_v1909) } else { ((v700 - self.scalar_v1228) / v30) });
        let v1912: bool = (v1907 < v1);
        let v1928: f64 = (if (v1903 && (!v1912)) { (v1907 + (self.scalar_v1909 * (((v1 + (((-v1911)) as f64).exp())) as f64).ln())) } else { (if (v1903 && v1912) { (v1 + (self.scalar_v1909 * (((v1 + ((v1911) as f64).exp())) as f64).ln())) } else { v4 }) });
        let v1940: f64 = (if (v1891 && (!((v1764 * (if v1903 { (v1900 * f64::powf(v1928, self.scalar_v1929)) } else { (if (self.scalar_v1804 && v1891) { v1900 } else { v4 }) })) < self.scalar_v734))) { self.scalar_v739 } else { (if (v1750 && (!(v1767 < self.scalar_v734))) { self.scalar_v739 } else { v1756 }) });
        let v2049: f64 = (v291 * self.scalar_v2048);
        let v2051: f64 = (v703 - v1058);
        let v2052: f64 = (v2051 / v1059);
        let v2053: bool = (v703 < v1058);
        let v2054: f64 = ((v2052) as f64).exp();
        let v2055: f64 = (v1 + v2054);
        let v2056: f64 = ((v2055) as f64).ln();
        let v2060: bool = (!v2053);
        let v2062: f64 = (((-v2052)) as f64).exp();
        let v2063: f64 = (v1 + v2062);
        let v2064: f64 = ((v2063) as f64).ln();
        let v2067: f64 = (if v2060 { (v1058 - (v1059 * v2064)) } else { (if v2053 { (v703 - (v1059 * v2056)) } else { v4 }) });
        let v2068: f64 = (v291 * self.scalar_v2047);
        let v2070: f64 = (v1 - (v284 * v2067));
        let v2072: f64 = (v1 - f64::powf(v2070, self.scalar_v1079));
        let v2076: f64 = ((v1081 * v2072) + (v170 * (v703 - v2067)));
        let v2079: f64 = (v300 * self.scalar_v2078);
        let v2081: f64 = (v441 * v626);
        let v2082: f64 = (v411 * v2081);
        let v2083: f64 = (v1145 * v2082);
        let v2084: f64 = (v1730 * v2083);
        let v2085: f64 = (v1152 * v2082);
        let v2086: f64 = (v1730 * v2085);
        let v2087: f64 = (v727 - v1104);
        let v2088: f64 = (v2087 / v1004);
        let v2089: bool = (v727 < v1104);
        let v2090: f64 = ((v2088) as f64).exp();
        let v2091: f64 = (v1 + v2090);
        let v2092: f64 = ((v2091) as f64).ln();
        let v2096: bool = (!v2089);
        let v2098: f64 = (((-v2088)) as f64).exp();
        let v2099: f64 = (v1 + v2098);
        let v2100: f64 = ((v2099) as f64).ln();
        let v2103: f64 = (if v2096 { (v1104 - (v1004 * v2100)) } else { (if v2089 { (v727 - (v1004 * v2092)) } else { v4 }) });
        let v2105: f64 = (v1 - (v2103 / v260));
        let v2107: f64 = (v1 - f64::powf(v2105, self.scalar_v1124));
        let v2109: f64 = (v727 - v2103);
        let v2111: f64 = ((v1125 * v2107) + (v1100 * v2109));
        let v2114: f64 = ((v1099 * v2111) + (v301 * v727));
        let v2119: f64 = (v732 - v1104);
        let v2120: f64 = (v2119 / v1004);
        let v2121: bool = (v732 < v1104);
        let v2122: f64 = ((v2120) as f64).exp();
        let v2123: f64 = (v1 + v2122);
        let v2124: f64 = ((v2123) as f64).ln();
        let v2128: bool = (!v2121);
        let v2130: f64 = (((-v2120)) as f64).exp();
        let v2131: f64 = (v1 + v2130);
        let v2132: f64 = ((v2131) as f64).ln();
        let v2135: f64 = (if v2128 { (v1104 - (v1004 * v2132)) } else { (if v2121 { (v732 - (v1004 * v2124)) } else { v4 }) });
        let v2137: f64 = (v1 - (v2135 / v260));
        let v2139: f64 = (v1 - f64::powf(v2137, self.scalar_v1124));
        let v2141: f64 = (v732 - v2135);
        let v2143: f64 = ((v1125 * v2139) + (v1100 * v2141));
        let v2146: f64 = ((v1099 * v2143) + (v301 * v732));
        let v2150: f64 = (v441 * v620);
        let v2151: f64 = (v436 / v441);
        let v2154: f64 = f64::powf(v2151, self.scalar_v2153);
        let v2155: f64 = (v2150 * v2154);
        let v2156: f64 = (v119 * self.scalar_v2152);
        let v2157: f64 = (v700 / v2156);
        let v2158: bool = (v2157 < self.scalar_v734);
        let v2159: f64 = ((v2157) as f64).exp();
        let v2161: bool = (!v2158);
        let v2162: f64 = (if v2161 { self.scalar_v739 } else { v1940 });
        let v2166: f64 = (if v2161 { (v2162 * (v1 + (v2157 - self.scalar_v734))) } else { (if v2158 { v2159 } else { v1406 }) });
        let v2167: f64 = (v2155 * v2166);
        let v2168: f64 = (v423 * v631);
        let v2169: f64 = (v119 * v2168);
        let v2170: f64 = (v2169 / v339);
        let v2171: f64 = (v411 * v2170);
        let v2172: f64 = (v1048 * v2171);
        let v2173: f64 = (v31 + v1037);
        let v2177: f64 = (v411 * v636);
        let v2180: f64 = ((v1595 * v2081) + (v1599 * v2170));
        let v2181: f64 = (v2177 * v2180);
        let v2186: f64 = ((v727 - v240) / self.scalar_v2185);
        let v2187: f64 = (v121 * v2186);
        let v2188: bool = (v2187 < self.scalar_v734);
        let v2190: bool = (v2188 && self.scalar_v2189);
        let v2191: f64 = ((v2187) as f64).exp();
        let v2194: bool = (self.scalar_v2189 && (!v2188));
        let v2195: f64 = (if v2194 { self.scalar_v739 } else { v2162 });
        let v2200: f64 = (v642 * v1600);
        let v2201: f64 = (v765 * v2200);
        let v2204: f64 = (((v1 + (v423 * (if v2194 { (v2195 * (v1 + (v2187 - self.scalar_v734))) } else { (if v2190 { v2191 } else { v4 }) })))) as f64).sqrt();
        let v2205: f64 = (v1 + v2204);
        let v2207: f64 = (if self.scalar_v2189 { (v2201 / v2205) } else { (if self.scalar_v2176 { (v2181 / v633) } else { v4 }) });
        let v2215: f64 = (if self.scalar_v2213 { (v785 * v1140) } else { v4 });
        let v2216: f64 = (v2215 - v1140);
        let v2218: f64 = (((v1 + v2215)) as f64).sqrt();
        let v2219: f64 = (v1 + v2218);
        let v2221: f64 = (if self.scalar_v2213 { (v2216 / v2219) } else { v4 });
        let v2223: f64 = (if self.scalar_v2213 { (v423 * (if v791 { (v792 * (v1 + (v787 - self.scalar_v734))) } else { (if v788 { v789 } else { v4 }) })) } else { v4 });
        let v2225: f64 = (((v1 + v2223)) as f64).sqrt();
        let v2226: f64 = (v1 + v2225);
        let v2228: f64 = (if self.scalar_v2213 { (v2223 / v2226) } else { v4 });
        let v2230: f64 = (v636 * self.scalar_v2229);
        let v2233: f64 = ((v2081 * v2221) + (v2170 * v2228));
        let v2234: f64 = (v2230 * v2233);
        let v2237: f64 = (v732 - v240);
        let v2238: f64 = (v121 * v2237);
        let v2239: bool = (v2238 < self.scalar_v734);
        let v2241: bool = (v2239 && self.scalar_v2240);
        let v2242: f64 = ((v2238) as f64).exp();
        let v2245: bool = (self.scalar_v2240 && (!v2239));
        let v2246: f64 = (if v2245 { self.scalar_v739 } else { v2195 });
        let v2251: f64 = (v642 * v1617);
        let v2252: f64 = (v785 * v2251);
        let v2255: f64 = (((v1 + (v423 * (if v2245 { (v2246 * (v1 + (v2238 - self.scalar_v734))) } else { (if v2241 { v2242 } else { v4 }) })))) as f64).sqrt();
        let v2256: f64 = (v1 + v2255);
        let v2258: f64 = (if self.scalar_v2240 { (v2252 / v2256) } else { (if self.scalar_v2213 { (v2234 / v633) } else { v4 }) });
        let v2266: f64 = (if self.scalar_v2262 { (f64::powf(v1078, self.scalar_v2263) - v170) } else { v4 });
        let v2267: f64 = (if self.scalar_v2262 { v1061 } else { v4 });
        let v2268: bool = (v2267 < v4);
        let v2269: bool = (self.scalar_v2262 && v2268);
        let v2270: f64 = ((v2267) as f64).exp();
        let v2271: f64 = (v1 + v2270);
        let v2275: bool = (self.scalar_v2262 && (!v2268));
        let v2277: f64 = (((-v2267)) as f64).exp();
        let v2278: f64 = (v1 + v2277);
        let v2280: f64 = (if v2275 { (v2277 / v2278) } else { (if v2269 { (v1 / v2271) } else { v4 }) });
        let v2283: f64 = (if self.scalar_v2262 { (v170 + (v2266 * v2280)) } else { v4 });
        let v2286: f64 = (v121 * v1141);
        let v2287: f64 = (v2286 / v371);
        let v2288: f64 = (v411 / v1143);
        let v2290: f64 = (if self.scalar_v2262 { (v2287 * v2288) } else { v4 });
        let v2291: f64 = (v1730 * v2082);
        let v2296: f64 = (v705 * v872);
        let v2298: f64 = ((if self.scalar_v2262 { (v2167 / v2156) } else { v4 }) + ((if self.scalar_v2262 { (v2049 * v2283) } else { v4 }) + (if self.scalar_v2262 { (v2290 * v2291) } else { v4 })));
        let v2307: f64 = (if self.scalar_v2262 { (v2084 + (v2167 * self.scalar_v2301)) } else { v4 });
        let v2316: f64 = (if self.scalar_v2315 { v2084 } else { (if self.scalar_v2262 { (v2307 * self.scalar_v2312) } else { v4 }) });
        let v2317: f64 = (if self.scalar_v2315 { v2086 } else { (if self.scalar_v2262 { (v2086 + (v2307 * self.scalar_v2308)) } else { v4 }) });
        let v2321: f64 = 0.0;
        let v2322: f64 = (self.scalar_v27 * v2321);
        let v2355: f64 = (v1195 + v1196);
        let v2356: f64 = (v2355 / v1192);
        let v2364: bool = (v2356 > v4);
        let v2365: f64 = (v2316 + v2317);
        let v2368: bool = (!v2364);
        let v2369: f64 = (v626 * v1730);
        let v2371: f64 = (if v2368 { (v1192 * v2369) } else { (if v2364 { (v2365 / v2356) } else { v4 }) });
        let v2384: f64 = (if self.scalar_v2383 { v4 } else { (if self.scalar_v2378 { (v2371 * self.scalar_v2379) } else { (if self.scalar_v2373 { (self.scalar_v2308 * v2371) } else { v4 }) }) });
        let v2421: f64 = 0.0;
        let v2422: f64 = (self.scalar_v27 * v2421);
        let v2424: f64 = 0.0;
        let v2425: f64 = (self.scalar_v27 * v2424);
        let v2427: f64 = 0.0;
        let v2428: f64 = (self.scalar_v27 * v2427);
        let v2430: f64 = 0.0;
        let v2431: f64 = (self.scalar_v27 * v2430);
        let v2434: f64 = 0.0;
        let v2435: f64 = (self.scalar_v27 * v2434);
        let v2438: f64 = 0.0;
        let v2439: f64 = (self.scalar_v27 * v2438);
        let v2446: f64 = 0.0;
        let v2447: f64 = (self.scalar_v27 * v2446);
        let v2452: f64 = 0.0;
        let v2453: f64 = (self.scalar_v27 * v2452);
        let v2463: f64 = 0.0;
        let v2464: f64 = (v2384 * v2463);
        let v2468: f64 = (if v102 { (-(-1.0 / v103)) } else { v1 });
        let v2471: f64 = (if v110 { (v2468 / v112) } else { (if v108 { v2468 } else { v4 }) });
        let v2472: f64 = (v2471 / self.scalar_v17);
        let v2473: f64 = (v118 * v2471);
        let v2475: f64 = (v119 * v119);
        let v2476: f64 = ((-v2473) / v2475);
        let v2477: f64 = (v2472 / v117);
        let v2523: f64 = ((v172 * v2477) + (v125 * (v171 * v2473)));
        let v2526: f64 = (-v2472);
        let v2528: f64 = ((v2523 + (self.scalar_v64 * v2472)) + (self.scalar_v177 * v2526));
        let v2533: f64 = (((v119 * (-v2528)) - (v180 * v2473)) / v2475);
        let v2547: f64 = (if v189 { ((v193 * v2473) + (v119 * ((v191 * (-v2533)) / v192))) } else { (if v182 { (v2528 + ((v185 * v2473) + (v119 * ((v183 * v2533) / v184)))) } else { v4 }) });
        let v2550: f64 = (self.scalar_v200 * v2526);
        let v2551: f64 = ((v2523 + (self.scalar_v197 * v2472)) + v2550);
        let v2556: f64 = (((v119 * (-v2551)) - (v203 * v2473)) / v2475);
        let v2570: f64 = (if v212 { ((v216 * v2473) + (v119 * ((v214 * (-v2556)) / v215))) } else { (if v205 { (v2551 + ((v208 * v2473) + (v119 * ((v206 * v2556) / v207)))) } else { v4 }) });
        let v2573: f64 = (v2550 + (v2523 + (self.scalar_v220 * v2472)));
        let v2578: f64 = (((v119 * (-v2573)) - (v224 * v2473)) / v2475);
        let v2592: f64 = (if v233 { ((v237 * v2473) + (v119 * ((v235 * (-v2578)) / v236))) } else { (if v226 { (v2573 + ((v229 * v2473) + (v119 * ((v227 * v2578) / v228)))) } else { v4 }) });
        let v2595: f64 = (v2550 + (v2523 + (self.scalar_v66 * v2472)));
        let v2600: f64 = (((v119 * (-v2595)) - (v244 * v2473)) / v2475);
        let v2614: f64 = (if v253 { ((v257 * v2473) + (v119 * ((v255 * (-v2600)) / v256))) } else { (if v246 { (v2595 + ((v249 * v2473) + (v119 * ((v247 * v2600) / v248)))) } else { v4 }) });
        let v2640: f64 = ((-v2547) / (v196 * v196));
        let v2642: f64 = (v260 * v260);
        let v2647: f64 = ((self.scalar_v64 * v2640) * (self.scalar_v32 * f64::powf(v286, self.scalar_v1440)));
        let v2652: f64 = (self.scalar_v290 * v2647);
        let v2659: f64 = (self.scalar_v293 * (((-(self.scalar_v66 * v2614)) / v2642) * (self.scalar_v67 * f64::powf(v294, self.scalar_v1533))));
        let v2662: f64 = ((-v2659) / (v297 * v297));
        let v2663: f64 = (self.scalar_v299 * v2659);
        let v2664: f64 = (self.scalar_v292 * v2662);
        let v2678: f64 = (self.scalar_v323 * (v326 * (self.scalar_v324 * v2477)));
        let v2685: f64 = (self.scalar_v335 * (v338 * (self.scalar_v336 * v2477)));
        let v2688: f64 = (if self.scalar_v341 { (self.scalar_v342 * (self.scalar_v340 * v2471)) } else { v4 });
        let v2690: f64 = (if self.scalar_v341 { (v2688 / v30) } else { (((v119 * (-((v2523 + (self.scalar_v261 * v2472)) + (self.scalar_v264 * v2526)))) - (v267 * v2473)) / v2475) });
        let v2694: f64 = (if v351 { (v30 * ((v352 * v2690) / v353)) } else { v2688 });
        let v2702: f64 = (if self.scalar_v370 { v4 } else { (if self.scalar_v341 { (if v359 { (v2694 + (v30 * ((v361 * (-v2690)) / v362))) } else { v2694 }) } else { v4 }) });
        let v2705: f64 = (if self.scalar_v373 { (self.scalar_v374 * (self.scalar_v372 * v2471)) } else { v4 });
        let v2707: f64 = (if self.scalar_v373 { (v2705 / v30) } else { v2690 });
        let v2711: f64 = (if v383 { (v30 * ((v384 * v2707) / v385)) } else { v2705 });
        let v2722: f64 = (v407 * (self.scalar_v403 * (self.scalar_v404 * v2471)));
        let v2739: f64 = (v371 * v371);
        let v2751: f64 = ((v435 * (self.scalar_v422 * (v430 * (((v371 * (self.scalar_v427 * v2477)) - (v428 * v2702)) / v2739)))) + (v431 * (v435 * (((v371 * (self.scalar_v432 * v2476)) - (v433 * v2702)) / v2739))));
        let v2754: f64 = (self.scalar_v437 * (v440 * (self.scalar_v438 * v2477)));
        let v2815: f64 = ((v530 * (self.scalar_v521 * (v525 * (self.scalar_v523 * v2477)))) + (v526 * (v530 * (self.scalar_v528 * v2476))));
        let v2922: f64 = (v604 * (self.scalar_v311 * v2477));
        let v2926: f64 = ((v606 * v2662) + (v298 * (self.scalar_v605 * v2922)));
        let v2941: f64 = (self.scalar_v621 * (v625 * (self.scalar_v623 * v2477)));
        let v2944: f64 = (self.scalar_v627 * (v630 * (self.scalar_v628 * v2477)));
        let v2945: f64 = (v2941 + v2944);
        let v2947: f64 = ((self.scalar_v632 * v2945) / self.scalar_v635);
        let v2950: f64 = (self.scalar_v637 * (v641 * (self.scalar_v639 * v2477)));
        let v2960: f64 = (self.scalar_v660 * v2922);
        let v2983: f64 = (v697 * v2476);
        let v2984: f64 = (self.scalar_v0 * v121);
        let v2985: f64 = (v121 * self.scalar_v2979);
        let v2998: f64 = (v700 * v2476);
        let v3002: f64 = (((v371 * v2998) - (v745 * v2702)) / v2739);
        let v3003: f64 = (v2985 / v371);
        let v3004: f64 = (v2984 / v371);
        let v3014: f64 = (if v750 { (v751 * v3002) } else { (if v747 { (v748 * v3002) } else { v4 }) });
        let v3015: f64 = (if v750 { (v751 * v3003) } else { (if v747 { (v748 * v3003) } else { v4 }) });
        let v3016: f64 = (if v750 { (v751 * v3004) } else { (if v747 { (v748 * v3004) } else { v4 }) });
        let v3017: f64 = (v727 * v2476);
        let v3018: f64 = (v121 * self.scalar_v2980);
        let v3019: f64 = (v121 * self.scalar_v2981);
        let v3035: f64 = (if v760 { (v761 * v3017) } else { (if v757 { (v758 * v3017) } else { v4 }) });
        let v3036: f64 = (if v760 { (v761 * v2984) } else { (if v757 { (v758 * v2984) } else { v4 }) });
        let v3037: f64 = (if v760 { (v761 * v3018) } else { (if v757 { (v758 * v3018) } else { v4 }) });
        let v3038: f64 = (if v760 { (v761 * v3019) } else { (if v757 { (v758 * v3019) } else { v4 }) });
        let v3039: f64 = (if v760 { (v761 * v2985) } else { (if v757 { (v758 * v2985) } else { v4 }) });
        let v3053: f64 = (v121 * self.scalar_v2982);
        let v3054: f64 = (v732 * v2476);
        let v3070: f64 = (if v780 { (v781 * v3018) } else { (if v777 { (v778 * v3018) } else { v4 }) });
        let v3071: f64 = (if v780 { (v781 * v3053) } else { (if v777 { (v778 * v3053) } else { v4 }) });
        let v3072: f64 = (if v780 { (v781 * v3054) } else { (if v777 { (v778 * v3054) } else { v4 }) });
        let v3073: f64 = (if v780 { (v781 * v3019) } else { (if v777 { (v778 * v3019) } else { v4 }) });
        let v3074: f64 = (if v780 { (v781 * v2985) } else { (if v777 { (v778 * v2985) } else { v4 }) });
        let v3077: f64 = (v121 * (-v2570));
        let v3078: f64 = ((v786 * v2476) + v3077);
        let v3100: f64 = (v3077 + (v797 * v2476));
        let v3122: f64 = (v3077 + (v808 * v2476));
        let v3132: f64 = (if v813 { (v814 * v3122) } else { (if v810 { (v811 * v3122) } else { v4 }) });
        let v3133: f64 = (if v813 { (v814 * v2984) } else { (if v810 { (v811 * v2984) } else { v4 }) });
        let v3134: f64 = (if v813 { (v814 * v2985) } else { (if v810 { (v811 * v2985) } else { v4 }) });
        let v3136: f64 = (v3077 + (v819 * v2476));
        let v3146: f64 = (if v824 { (v825 * v3136) } else { (if v821 { (v822 * v3136) } else { v4 }) });
        let v3147: f64 = (if v824 { (v825 * v2984) } else { (if v821 { (v822 * v2984) } else { v4 }) });
        let v3148: f64 = (if v824 { (v825 * v2985) } else { (if v821 { (v822 * v2985) } else { v4 }) });
        let v3152: f64 = (v31 * v832);
        let v3153: f64 = ((v423 * v3132) / v3152);
        let v3154: f64 = ((v423 * v3133) / v3152);
        let v3155: f64 = ((v423 * v3134) / v3152);
        let v3159: f64 = (v31 * v835);
        let v3160: f64 = ((v423 * v3146) / v3159);
        let v3161: f64 = ((v423 * v3147) / v3159);
        let v3162: f64 = ((v423 * v3148) / v3159);
        let v3169: f64 = (v837 * v837);
        let v3179: f64 = (if v840 { v4 } else { (((v837 * (v31 * v3146)) - (v836 * v3160)) / v3169) });
        let v3180: f64 = (if v840 { v4 } else { (((v837 * (v31 * v3147)) - (v836 * v3161)) / v3169) });
        let v3181: f64 = (if v840 { v4 } else { (((v837 * (v31 * v3148)) - (v836 * v3162)) / v3169) });
        let v3207: f64 = ((v846 * v2473) + (v119 * ((v3153 - v3160) - ((((v837 * v3153) - (v843 * v3160)) / v3169) / v844))));
        let v3208: f64 = (v119 * ((v3154 - v3161) - ((((v837 * v3154) - (v843 * v3161)) / v3169) / v844)));
        let v3209: f64 = (v119 * ((-v3162) - (((-(v843 * v3162)) / v3169) / v844)));
        let v3210: f64 = (v119 * (v3155 - ((v3155 / v837) / v844)));
        let v3212: f64 = (self.scalar_v2979 + v3210);
        let v3216: f64 = (v339 * v339);
        let v3217: f64 = (((v339 * v3207) - (v848 * v2685)) / v3216);
        let v3218: f64 = (v3208 / v339);
        let v3219: f64 = ((self.scalar_v0 + v3209) / v339);
        let v3220: f64 = (v3212 / v339);
        let v3234: f64 = ((v863 * v2685) + (v339 * (v411 * v3217)));
        let v3235: f64 = (v339 * (v411 * v3218));
        let v3236: f64 = (v339 * (v411 * v3219));
        let v3237: f64 = (v339 * (v411 * v3220));
        let v3257: f64 = (if v850 { (v2570 + ((v867 * (v31 * v2473)) + (v862 * (((v864 * v2476) + (v121 * v3234)) / v866)))) } else { v4 });
        let v3258: f64 = (if v850 { ((v862 * ((v121 * v3235) / v866)) - (if v856 { (self.scalar_v0 / v858) } else { (if v853 { self.scalar_v0 } else { v4 }) })) } else { v4 });
        let v3259: f64 = (if v850 { ((v862 * ((v121 * v3236) / v866)) - (if v856 { (self.scalar_v2979 / v858) } else { (if v853 { self.scalar_v2979 } else { v4 }) })) } else { v4 });
        let v3260: f64 = (if v850 { (v862 * ((v121 * v3237) / v866)) } else { v4 });
        let v3263: f64 = (v874 * (if v850 { (v872 * v2570) } else { v4 }));
        let v3265: f64 = (if v850 { (v3263 + v3263) } else { v4 });
        let v3266: f64 = (v871 * v3257);
        let v3268: f64 = (v871 * v3258);
        let v3270: f64 = (v871 * v3259);
        let v3272: f64 = (v871 * v3260);
        let v3280: f64 = (v31 * v883);
        let v3281: f64 = ((v3265 + (if v850 { (v3266 + v3266) } else { (v2722 + v2722) })) / v3280);
        let v3282: f64 = ((if v850 { (v3268 + v3268) } else { v4 }) / v3280);
        let v3283: f64 = ((if v850 { (v3270 + v3270) } else { v4 }) / v3280);
        let v3284: f64 = ((if v850 { (v3272 + v3272) } else { v4 }) / v3280);
        let v3292: f64 = (v884 * v884);
        let v3315: f64 = (if v888 { (v411 * (v3257 + v3281)) } else { (if v880 { (((v884 * (v411 * v3265)) - (v881 * (v3281 - v3257))) / v3292) } else { v4 }) });
        let v3316: f64 = (if v888 { (v411 * (v3258 + v3282)) } else { (if v880 { ((-(v881 * (v3282 - v3258))) / v3292) } else { v4 }) });
        let v3317: f64 = (if v888 { (v411 * (v3259 + v3283)) } else { (if v880 { ((-(v881 * (v3283 - v3259))) / v3292) } else { v4 }) });
        let v3318: f64 = (if v888 { (v411 * (v3260 + v3284)) } else { (if v880 { ((-(v881 * (v3284 - v3260))) / v3292) } else { v4 }) });
        let v3340: f64 = (v899 * v899);
        let v3354: f64 = (if v850 { (((v899 * ((v895 * v3315) + (v891 * v3315))) - (v896 * (self.scalar_v893 * (v3315 + (self.scalar_v892 * v2685))))) / v3340) } else { v4 });
        let v3355: f64 = (if v850 { (((v899 * ((v895 * v3316) + (v891 * v3316))) - (v896 * (self.scalar_v893 * v3316))) / v3340) } else { v4 });
        let v3356: f64 = (if v850 { (((v899 * ((v895 * v3317) + (v891 * v3317))) - (v896 * (self.scalar_v893 * v3317))) / v3340) } else { v4 });
        let v3357: f64 = (if v850 { (((v899 * ((v895 * v3318) + (v891 * v3318))) - (v896 * (self.scalar_v893 * v3318))) / v3340) } else { v4 });
        let v3361: f64 = (v901 * v901);
        let v3375: f64 = (if v850 { (((v901 * v3217) - (v849 * v3354)) / v3361) } else { v4 });
        let v3376: f64 = (if v850 { (((v901 * v3218) - (v849 * v3355)) / v3361) } else { v4 });
        let v3377: f64 = (if v850 { (((v901 * v3219) - (v849 * v3356)) / v3361) } else { v4 });
        let v3378: f64 = (if v850 { (((v901 * v3220) - (v849 * v3357)) / v3361) } else { v4 });
        let v3383: f64 = (if v850 { (v3375 / self.scalar_v905) } else { v2707 });
        let v3384: f64 = (if v850 { (v3376 / self.scalar_v905) } else { v4 });
        let v3385: f64 = (if v850 { (v3377 / self.scalar_v905) } else { v4 });
        let v3386: f64 = (if v850 { (v3378 / self.scalar_v905) } else { v4 });
        let v3431: f64 = (if v850 { ((if v917 { (v3375 + (self.scalar_v905 * ((v919 * (-v3383)) / v920))) } else { (if v909 { (self.scalar_v905 * ((v910 * v3383) / v911)) } else { v4 }) }) / self.scalar_v931) } else { v4 });
        let v3432: f64 = (if v850 { ((if v917 { (v3376 + (self.scalar_v905 * ((v919 * (-v3384)) / v920))) } else { (if v909 { (self.scalar_v905 * ((v910 * v3384) / v911)) } else { v4 }) }) / self.scalar_v931) } else { v4 });
        let v3433: f64 = (if v850 { ((if v917 { (v3377 + (self.scalar_v905 * ((v919 * (-v3385)) / v920))) } else { (if v909 { (self.scalar_v905 * ((v910 * v3385) / v911)) } else { v4 }) }) / self.scalar_v931) } else { v4 });
        let v3434: f64 = (if v850 { ((if v917 { (v3378 + (self.scalar_v905 * ((v919 * (-v3386)) / v920))) } else { (if v909 { (self.scalar_v905 * ((v910 * v3386) / v911)) } else { v4 }) }) / self.scalar_v931) } else { v4 });
        let v3439: f64 = (if v850 { (v3315 / self.scalar_v894) } else { v4 });
        let v3440: f64 = (if v850 { (v3316 / self.scalar_v894) } else { v4 });
        let v3441: f64 = (if v850 { (v3317 / self.scalar_v894) } else { v4 });
        let v3442: f64 = (if v850 { (v3318 / self.scalar_v894) } else { v4 });
        let v3471: f64 = (v31 * v941);
        let v3494: f64 = ((v944 * (((v938 * ((v936 * v3439) + (v935 * (v423 * v3431)))) + (v937 * v3439)) / v3471)) - (v942 * ((v943 * v3439) + (v938 * (v31 * v3431)))));
        let v3495: f64 = (v944 * v944);
        let v3499: f64 = ((v944 * (((v938 * ((v936 * v3440) + (v935 * (v423 * v3432)))) + (v937 * v3440)) / v3471)) - (v942 * ((v943 * v3440) + (v938 * (v31 * v3432)))));
        let v3503: f64 = ((v944 * (((v938 * ((v936 * v3441) + (v935 * (v423 * v3433)))) + (v937 * v3441)) / v3471)) - (v942 * ((v943 * v3441) + (v938 * (v31 * v3433)))));
        let v3507: f64 = ((v944 * (((v938 * ((v936 * v3442) + (v935 * (v423 * v3434)))) + (v937 * v3442)) / v3471)) - (v942 * ((v943 * v3442) + (v938 * (v31 * v3434)))));
        let v3509: f64 = (if v850 { (v3494 / v3495) } else { v4 });
        let v3510: f64 = (if v850 { (v3499 / v3495) } else { v4 });
        let v3511: f64 = (if v850 { (v3503 / v3495) } else { v4 });
        let v3512: f64 = (if v850 { (v3507 / v3495) } else { v4 });
        let v3519: f64 = ((v946 * v3179) + (v841 * v3509));
        let v3522: f64 = ((v946 * v3180) + (v841 * v3510));
        let v3525: f64 = ((v946 * v3181) + (v841 * v3511));
        let v3526: f64 = (v841 * v3512);
        let v3534: f64 = (v950 * v950);
        let v3548: f64 = (if v850 { (((v950 * ((-v3509) + v3519)) - (v949 * v3519)) / v3534) } else { v4 });
        let v3549: f64 = (if v850 { (((v950 * ((-v3510) + v3522)) - (v949 * v3522)) / v3534) } else { v4 });
        let v3550: f64 = (if v850 { (((v950 * ((-v3511) + v3525)) - (v949 * v3525)) / v3534) } else { v4 });
        let v3551: f64 = (if v850 { (((v950 * ((-v3512) + v3526)) - (v949 * v3526)) / v3534) } else { v4 });
        let v3570: f64 = (if v850 { ((v953 * v2476) + (v121 * ((v952 * v3234) + (v864 * v3548)))) } else { v4 });
        let v3571: f64 = (if v850 { (v121 * ((v952 * v3235) + (v864 * v3549))) } else { v4 });
        let v3572: f64 = (if v850 { (v121 * ((v952 * v3236) + (v864 * v3550))) } else { v4 });
        let v3573: f64 = (if v850 { (v121 * ((v952 * v3237) + (v864 * v3551))) } else { v4 });
        let v3595: f64 = (if v850 { ((v31 * v3570) + ((v958 * v3179) + (v841 * (v3179 + v3570)))) } else { v4 });
        let v3596: f64 = (if v850 { ((v31 * v3571) + ((v958 * v3180) + (v841 * (v3180 + v3571)))) } else { v4 });
        let v3597: f64 = (if v850 { ((v31 * v3572) + ((v958 * v3181) + (v841 * (v3181 + v3572)))) } else { v4 });
        let v3598: f64 = (if v850 { ((v31 * v3573) + (v841 * v3573)) } else { v4 });
        let v3603: f64 = (if v850 { (v411 * v3570) } else { v4 });
        let v3604: f64 = (if v850 { (v411 * v3571) } else { v4 });
        let v3605: f64 = (if v850 { (v411 * v3572) } else { v4 });
        let v3606: f64 = (if v850 { (v411 * v3573) } else { v4 });
        let v3607: f64 = (v964 * v3603);
        let v3609: f64 = (v964 * v3604);
        let v3611: f64 = (v964 * v3605);
        let v3613: f64 = (v964 * v3606);
        let v3623: f64 = (v31 * v970);
        let v3624: f64 = ((if v850 { (v3595 + (v3607 + v3607)) } else { v4 }) / v3623);
        let v3625: f64 = ((if v850 { (v3596 + (v3609 + v3609)) } else { v4 }) / v3623);
        let v3626: f64 = ((if v850 { (v3597 + (v3611 + v3611)) } else { v4 }) / v3623);
        let v3627: f64 = ((if v850 { (v3598 + (v3613 + v3613)) } else { v4 }) / v3623);
        let v3643: f64 = (v975 * v975);
        let v3661: f64 = (if v980 { v4 } else { (if v974 { (((v975 * v3595) - (v961 * (v3624 - v3603))) / v3643) } else { (if v969 { (v3603 + v3624) } else { v4 }) }) });
        let v3662: f64 = (if v980 { v4 } else { (if v974 { (((v975 * v3596) - (v961 * (v3625 - v3604))) / v3643) } else { (if v969 { (v3604 + v3625) } else { v4 }) }) });
        let v3663: f64 = (if v980 { v4 } else { (if v974 { (((v975 * v3597) - (v961 * (v3626 - v3605))) / v3643) } else { (if v969 { (v3605 + v3626) } else { v4 }) }) });
        let v3664: f64 = (if v980 { v4 } else { (if v974 { (((v975 * v3598) - (v961 * (v3627 - v3606))) / v3643) } else { (if v969 { (v3606 + v3627) } else { v4 }) }) });
        let v3695: f64 = (if v850 { (self.scalar_v988 * v3217) } else { v4 });
        let v3696: f64 = (if v850 { (self.scalar_v988 * v3218) } else { v4 });
        let v3697: f64 = (if v850 { (self.scalar_v988 * v3219) } else { v4 });
        let v3698: f64 = (if v850 { (self.scalar_v988 * v3220) } else { v4 });
        let v3711: f64 = (v991 * v3695);
        let v3713: f64 = (v991 * v3696);
        let v3715: f64 = (v991 * v3697);
        let v3717: f64 = (v991 * v3698);
        let v3723: f64 = (v31 * v998);
        let v3732: f64 = (if v850 { (v3695 + (((if v850 { ((v993 * v3217) + (v849 * (self.scalar_v892 * (self.scalar_v893 * v2685)))) } else { v4 }) + (v3711 + v3711)) / v3723)) } else { v4 });
        let v3736: f64 = (v46 * v2614);
        let v3749: f64 = (v1009 * v1009);
        let v3769: f64 = (if v1007 { ((v1011 * v2614) + (v260 * (((v1009 * (v31 * v3217)) - (v1008 * (v3217 + v3354))) / v3749))) } else { (if v1003 { v3736 } else { v4 }) });
        let v3773: f64 = (self.scalar_v892 * v3217);
        let v3774: f64 = (self.scalar_v892 * v3218);
        let v3775: f64 = (self.scalar_v892 * v3219);
        let v3776: f64 = (self.scalar_v892 * v3220);
        let v3780: f64 = (v1015 * v1015);
        let v3816: f64 = (v843 * v843);
        let v3829: f64 = (if v1020 { (((v843 * (v31 * v3134)) - (v1021 * v3155)) / v3816) } else { v3664 });
        let v3830: f64 = (if v1020 { (if v738 { (v740 * v2983) } else { (if v735 { (v736 * v2983) } else { v4 }) }) } else { (if v850 { ((v985 * ((v982 * v3661) + (v981 * v3661))) + (v983 * (v985 * ((v219 * v2476) + (v121 * v2570))))) } else { v4 }) });
        let v3831: f64 = (if v1020 { (if v738 { (v740 * v2984) } else { (if v735 { (v736 * v2984) } else { v4 }) }) } else { (if v850 { (v985 * ((v982 * v3662) + (v981 * v3662))) } else { v4 }) });
        let v3833: f64 = (if v1020 { (if v738 { (v740 * v2985) } else { (if v735 { (v736 * v2985) } else { v4 }) }) } else { (if v850 { (v985 * ((v982 * v3664) + (v981 * v3664))) } else { v4 }) });
        let v3834: f64 = (v3179 + (if v1020 { (((v843 * (v31 * v3132)) - (v1021 * v3153)) / v3816) } else { v3661 }));
        let v3835: f64 = (v3180 + (if v1020 { (((v843 * (v31 * v3133)) - (v1021 * v3154)) / v3816) } else { v3662 }));
        let v3836: f64 = (v3181 + (if v1020 { v4 } else { v3663 }));
        let v3841: f64 = (if v1036 { (v411 * v3834) } else { v4 });
        let v3842: f64 = (if v1036 { (v411 * v3835) } else { v4 });
        let v3843: f64 = (if v1036 { (v411 * v3836) } else { v4 });
        let v3844: f64 = (if v1036 { (v411 * v3829) } else { v4 });
        let v3848: f64 = (v1040 * v1040);
        let v3872: f64 = (v1046 * v1046);
        let v3887: f64 = (if v1044 { (((v1046 * v3208) - (v847 * ((self.scalar_v0 + v3208) - self.scalar_v0))) / v3872) } else { (if v1036 { (((v1040 * v3842) - (v1039 * v3842)) / v3848) } else { v3549 }) });
        let v3888: f64 = (if v1044 { (((v1046 * v3209) - (v847 * (v3209 - self.scalar_v2979))) / v3872) } else { (if v1036 { (((v1040 * v3843) - (v1039 * v3843)) / v3848) } else { v3550 }) });
        let v3894: f64 = (if v1020 { v3736 } else { v3769 });
        let v3895: f64 = (if v1020 { v4 } else { (if v1007 { (v260 * (((v1009 * (v31 * v3218)) - (v1008 * (v3218 + v3355))) / v3749)) } else { v4 }) });
        let v3896: f64 = (if v1020 { v4 } else { (if v1007 { (v260 * (((v1009 * (v31 * v3219)) - (v1008 * (v3219 + v3356))) / v3749)) } else { v4 }) });
        let v3897: f64 = (if v1020 { v4 } else { (if v1007 { (v260 * (((v1009 * (v31 * v3220)) - (v1008 * (v3220 + v3357))) / v3749)) } else { v4 }) });
        let v3910: f64 = (if v1020 { (-((if v1020 { v3217 } else { (if v850 { (((v1015 * v3773) - (v1014 * v3217)) / v3780) } else { v4 }) }) / self.scalar_v892)) } else { (if v850 { ((-v3773) / v3780) } else { v4 }) });
        let v3911: f64 = (if v1020 { (-((if v1020 { v3218 } else { (if v850 { (((v1015 * v3774) - (v1014 * v3218)) / v3780) } else { v4 }) }) / self.scalar_v892)) } else { (if v850 { ((-v3774) / v3780) } else { v4 }) });
        let v3912: f64 = (if v1020 { (-((if v1020 { v3219 } else { (if v850 { (((v1015 * v3775) - (v1014 * v3219)) / v3780) } else { v4 }) }) / self.scalar_v892)) } else { (if v850 { ((-v3775) / v3780) } else { v4 }) });
        let v3913: f64 = (if v1020 { (-((if v1020 { v3220 } else { (if v850 { (((v1015 * v3776) - (v1014 * v3220)) / v3780) } else { v4 }) }) / self.scalar_v892)) } else { (if v850 { ((-v3776) / v3780) } else { v4 }) });
        let v3914: f64 = (self.scalar_v1057 * v2547);
        let v3915: f64 = (v46 * v2547);
        let v3917: f64 = (v1059 * (-v3914));
        let v3920: f64 = (v1059 * v1059);
        let v3921: f64 = ((v3917 - (v1060 * v3915)) / v3920);
        let v3922: f64 = (self.scalar_v2979 / v1059);
        let v3923: f64 = (self.scalar_v0 / v1059);
        let v3942: f64 = (-v3922);
        let v3943: f64 = (-v3923);
        let v3958: f64 = (if v1069 { (v3914 - ((v1073 * v3915) + (v1059 * ((v1071 * (-v3921)) / v1072)))) } else { (if v1062 { (-((v1065 * v3915) + (v1059 * ((v1063 * v3921) / v1064)))) } else { v4 }) });
        let v3959: f64 = (if v1069 { (-(v1059 * ((v1071 * v3942) / v1072))) } else { (if v1062 { (self.scalar_v2979 - (v1059 * ((v1063 * v3922) / v1064))) } else { v4 }) });
        let v3960: f64 = (if v1069 { (-(v1059 * ((v1071 * v3943) / v1072))) } else { (if v1062 { (self.scalar_v0 - (v1059 * ((v1063 * v3923) / v1064))) } else { v4 }) });
        let v3966: f64 = (-((v1076 * v2640) + (v284 * v3958)));
        let v3967: f64 = (-(v284 * v3959));
        let v3968: f64 = (-(v284 * v3960));
        let v3971: f64 = (self.scalar_v1079 * f64::powf(v1078, self.scalar_v3969));
        let v3975: f64 = (v2547 / self.scalar_v1079);
        let v3990: f64 = (((v1082 * v3975) + (v1081 * (-(v3966 * v3971)))) + (v170 * (-v3958)));
        let v3991: f64 = ((v1081 * (-(v3967 * v3971))) + (v170 * (self.scalar_v2979 - v3959)));
        let v3992: f64 = ((v1081 * (-(v3968 * v3971))) + (v170 * (self.scalar_v0 - v3960)));
        let v3998: f64 = (if self.scalar_v1092 { (self.scalar_v0 + (if v1020 { v4 } else { (if v850 { (v3696 + (((if v850 { (v993 * v3218) } else { v4 }) + (v3713 + v3713)) / v3723)) } else { v4 }) })) } else { self.scalar_v3993 });
        let v3999: f64 = (if self.scalar_v1092 { (self.scalar_v2979 + (if v1020 { self.scalar_v0 } else { (if v850 { (v3697 + (((if v850 { (v993 * v3219) } else { v4 }) + (v3715 + v3715)) / v3723)) } else { v4 }) })) } else { self.scalar_v3994 });
        let v4001: f64 = (if self.scalar_v1096 { v4 } else { (if self.scalar_v1092 { (if v1020 { v4 } else { v3732 }) } else { v4 }) });
        let v4002: f64 = (if self.scalar_v1096 { self.scalar_v0 } else { v3998 });
        let v4003: f64 = (if self.scalar_v1096 { v4 } else { v3999 });
        let v4004: f64 = (if self.scalar_v1096 { self.scalar_v2979 } else { (if self.scalar_v1092 { (if v1020 { self.scalar_v2979 } else { (if v850 { (v3698 + (((if v850 { (v993 * v3220) } else { v4 }) + (v3717 + v3717)) / v3723)) } else { v4 }) }) } else { v4 }) });
        let v4005: f64 = (-v2664);
        let v4010: f64 = (((v1099 * v4005) - (v1098 * v4005)) / (v1099 * v1099));
        let v4018: f64 = ((v1103 * v2614) + (v260 * (-(v4010 * (self.scalar_v1101 * f64::powf(v1100, self.scalar_v4011))))));
        let v4023: f64 = (v1050 * v1050);
        let v4024: f64 = (((v1050 * (v4001 - v4018)) - (v1105 * v3894)) / v4023);
        let v4028: f64 = (((v1050 * v4002) - (v1105 * v3895)) / v4023);
        let v4032: f64 = (((v1050 * v4003) - (v1105 * v3896)) / v4023);
        let v4036: f64 = (((v1050 * v4004) - (v1105 * v3897)) / v4023);
        let v4093: f64 = (if v1114 { (v4018 - ((v1118 * v3894) + (v1050 * ((v1116 * (-v4024)) / v1117)))) } else { (if v1107 { (v4001 - ((v1110 * v3894) + (v1050 * ((v1108 * v4024) / v1109)))) } else { v4 }) });
        let v4094: f64 = (if v1114 { (-((v1118 * v3895) + (v1050 * ((v1116 * (-v4028)) / v1117)))) } else { (if v1107 { (v4002 - ((v1110 * v3895) + (v1050 * ((v1108 * v4028) / v1109)))) } else { v4 }) });
        let v4095: f64 = (if v1114 { (-((v1118 * v3896) + (v1050 * ((v1116 * (-v4032)) / v1117)))) } else { (if v1107 { (v4003 - ((v1110 * v3896) + (v1050 * ((v1108 * v4032) / v1109)))) } else { v4 }) });
        let v4096: f64 = (if v1114 { (-((v1118 * v3897) + (v1050 * ((v1116 * (-v4036)) / v1117)))) } else { (if v1107 { (v4004 - ((v1110 * v3897) + (v1050 * ((v1108 * v4036) / v1109)))) } else { v4 }) });
        let v4099: f64 = (self.scalar_v1122 * f64::powf(v1054, self.scalar_v4097));
        let v4100: f64 = (v3910 * v4099);
        let v4101: f64 = (v3911 * v4099);
        let v4102: f64 = (v3912 * v4099);
        let v4103: f64 = (v3913 * v4099);
        let v4104: f64 = (v2614 / self.scalar_v1124);
        let v4118: f64 = (self.scalar_v1124 * f64::powf(v1127, self.scalar_v4116));
        let v4141: f64 = ((v1130 * v4104) + (v1125 * (-((v1128 * v4100) + (v1123 * ((-(((v260 * v4093) - (v1121 * v2614)) / v2642)) * v4118))))));
        let v4168: f64 = ((v1125 * (-((v1128 * v4101) + (v1123 * ((-(v4094 / v260)) * v4118))))) + ((v1133 * (v1100 * v4101)) + (v1132 * (v4002 - v4094))));
        let v4169: f64 = ((v1125 * (-((v1128 * v4102) + (v1123 * ((-(v4095 / v260)) * v4118))))) + ((v1133 * (v1100 * v4102)) + (v1132 * (v4003 - v4095))));
        let v4170: f64 = ((v1125 * (-((v1128 * v4103) + (v1123 * ((-(v4096 / v260)) * v4118))))) + ((v1133 * (v1100 * v4103)) + (v1132 * (v4004 - v4096))));
        let v4176: f64 = (v1099 * v4170);
        let v4178: f64 = (self.scalar_v0 * v301);
        let v4179: f64 = (v301 * self.scalar_v2979);
        let v4180: f64 = (((v1135 * v4005) + (v1099 * (v4141 + ((v1133 * ((v1123 * v4010) + (v1100 * v4100))) + (v1132 * (v4001 - v4093)))))) + (v694 * v2664));
        let v4181: f64 = ((v1099 * v4168) + v4178);
        let v4182: f64 = ((v1099 * v4169) + v4179);
        let v4187: f64 = (v441 * v441);
        let v4188: f64 = (((v441 * (v423 * v2751)) - (v1139 * v2754)) / v4187);
        let v4191: f64 = ((v1140 * v3014) + (v755 * v4188));
        let v4192: f64 = (v1140 * v3015);
        let v4193: f64 = (v1140 * v3016);
        let v4194: f64 = (v31 * v1143);
        let v4195: f64 = (v4191 / v4194);
        let v4196: f64 = (v4192 / v4194);
        let v4197: f64 = (v4193 / v4194);
        let v4201: f64 = (v1144 * v1144);
        let v4202: f64 = (((v1144 * v4191) - (v1141 * v4195)) / v4201);
        let v4206: f64 = (((v1144 * v4192) - (v1141 * v4196)) / v4201);
        let v4210: f64 = (((v1144 * v4193) - (v1141 * v4197)) / v4201);
        let v4216: f64 = (v1146 * f64::powf(v1024, (v1146 - v1)));
        let v4219: f64 = (((-(if self.scalar_v401 { v4 } else { (if self.scalar_v373 { (if v391 { (v2711 + (v30 * ((v393 * (-v2707)) / v394))) } else { v2711 }) } else { v4 }) })) / (v402 * v402)) * (v1147 * ((v1024) as f64).ln()));
        let v4220: f64 = ((v3830 * v4216) + v4219);
        let v4221: f64 = (v3831 * v4216);
        let v4222: f64 = ((if v1020 { v4 } else { (if v850 { (v985 * ((v982 * v3663) + (v981 * v3663))) } else { v4 }) }) * v4216);
        let v4223: f64 = (v3833 * v4216);
        let v4226: f64 = ((v1147 * v4188) + (v1140 * v4220));
        let v4227: f64 = (v1140 * v4221);
        let v4228: f64 = (v1140 * v4222);
        let v4229: f64 = (v1140 * v4223);
        let v4230: f64 = (v31 * v1150);
        let v4238: f64 = (v1151 * v1151);
        let v4239: f64 = (((v1151 * v4226) - (v1148 * (v4226 / v4230))) / v4238);
        let v4243: f64 = (((v1151 * v4227) - (v1148 * (v4227 / v4230))) / v4238);
        let v4247: f64 = (((v1151 * v4228) - (v1148 * (v4228 / v4230))) / v4238);
        let v4251: f64 = (((v1151 * v4229) - (v1148 * (v4229 / v4230))) / v4238);
        let v4256: f64 = (((v610 * v3990) - (v1086 * ((v609 * ((-v2647) / (v287 * v287))) + (v561 * (self.scalar_v608 * v2922))))) / (v610 * v610));
        let v4257: f64 = (v3991 / v610);
        let v4258: f64 = (v3992 / v610);
        let v4262: f64 = (v607 * v607);
        let v4265: f64 = (v4182 / v607);
        let v4266: f64 = (v4176 / v607);
        let v4267: f64 = (v4256 + (((v607 * v4180) - (v1138 * v2926)) / v4262));
        let v4268: f64 = (v4258 + (v4181 / v607));
        let v4310: f64 = (if self.scalar_v1159 { ((v1165 * v2476) + (v121 * ((v1164 * v2960) + (v661 * (((v607 * (-v4180)) - (v1163 * v2926)) / v4262))))) } else { v4 });
        let v4331: f64 = ((v1173 * ((v1168 * (if self.scalar_v1159 { ((v1160 * v2476) + (v121 * ((v1155 * v2960) + (v661 * v4256)))) } else { v4 })) - (v1169 * v4310))) - (v1170 * (v1172 * ((v661 * v2476) + (v121 * v2960)))));
        let v4335: f64 = (((v1168 * (if self.scalar_v1159 { (v121 * (v661 * v4258)) } else { v4 })) - (v1169 * (if self.scalar_v1159 { (v121 * (v661 * ((-v4181) / v607))) } else { v4 }))) / v1173);
        let v4338: f64 = (if self.scalar_v1159 { (v4331 / (v1173 * v1173)) } else { (if self.scalar_v1153 { v4267 } else { v4 }) });
        let v4339: f64 = (if self.scalar_v1159 { ((v1168 * (if self.scalar_v1159 { (v121 * (v661 * v4257)) } else { v4 })) / v1173) } else { (if self.scalar_v1153 { v4257 } else { v4 }) });
        let v4340: f64 = (if self.scalar_v1159 { v4335 } else { (if self.scalar_v1153 { v4268 } else { v4 }) });
        let v4341: f64 = (if self.scalar_v1159 { ((-(v1169 * (if self.scalar_v1159 { (v121 * (v661 * ((-v4182) / v607))) } else { v4 }))) / v1173) } else { (if self.scalar_v1153 { v4265 } else { v4 }) });
        let v4342: f64 = (if self.scalar_v1159 { ((-(v1169 * (if self.scalar_v1159 { (v121 * (v661 * ((-v4176) / v607))) } else { v4 }))) / v1173) } else { (if self.scalar_v1153 { v4266 } else { v4 }) });
        let v4343: f64 = (v1175 * v4338);
        let v4344: f64 = (v4343 + v4343);
        let v4345: f64 = (v1175 * v4339);
        let v4346: f64 = (v4345 + v4345);
        let v4347: f64 = (v1175 * v4340);
        let v4348: f64 = (v4347 + v4347);
        let v4349: f64 = (v1175 * v4341);
        let v4350: f64 = (v4349 + v4349);
        let v4351: f64 = (v1175 * v4342);
        let v4352: f64 = (v4351 + v4351);
        let v4353: f64 = (v31 * v1181);
        let v4354: f64 = (v4344 / v4353);
        let v4355: f64 = (v4346 / v4353);
        let v4356: f64 = (v4348 / v4353);
        let v4357: f64 = (v4350 / v4353);
        let v4358: f64 = (v4352 / v4353);
        let v4366: f64 = (v1182 * v1182);
        let v4409: f64 = ((v1191 * (if v1185 { (v411 * (v4338 + v4354)) } else { (if v1178 { ((-(v1179 * (v4354 - v4338))) / v4366) } else { v4 }) })) + (v1188 * (v411 * (v4202 + v4239))));
        let v4412: f64 = ((v1191 * (if v1185 { (v411 * (v4339 + v4355)) } else { (if v1178 { ((-(v1179 * (v4355 - v4339))) / v4366) } else { v4 }) })) + (v1188 * (v411 * v4206)));
        let v4415: f64 = ((v1191 * (if v1185 { (v411 * (v4340 + v4356)) } else { (if v1178 { ((-(v1179 * (v4356 - v4340))) / v4366) } else { v4 }) })) + (v1188 * (v411 * (v4210 + v4243))));
        let v4418: f64 = ((v1191 * (if v1185 { (v411 * (v4341 + v4357)) } else { (if v1178 { ((-(v1179 * (v4357 - v4341))) / v4366) } else { v4 }) })) + (v1188 * (v411 * v4247)));
        let v4421: f64 = ((v1191 * (if v1185 { (v411 * (v4342 + v4358)) } else { (if v1178 { ((-(v1179 * (v4358 - v4342))) / v4366) } else { v4 }) })) + (v1188 * (v411 * v4251)));
        let v4441: f64 = (v1192 * v1192);
        let v4532: f64 = (v2998 / self.scalar_v478);
        let v4533: f64 = (v2985 / self.scalar_v478);
        let v4534: f64 = (v2984 / self.scalar_v478);
        let v4539: f64 = (if v1252 { (v1253 * v4533) } else { (if v1207 { (self.scalar_v2979 + (v1199 * ((v1209 * self.scalar_v4469) / v1210))) } else { (if v1201 { (v1199 * ((v1202 * self.scalar_v4459) / v1203)) } else { v4 }) }) });
        let v4540: f64 = (if v1252 { (v1253 * v4534) } else { (if v1207 { (self.scalar_v0 + (v1199 * ((v1209 * self.scalar_v4470) / v1210))) } else { (if v1201 { (v1199 * ((v1202 * self.scalar_v4460) / v1203)) } else { v4 }) }) });
        let v4728: f64 = (v703 * v2476);
        let v4729: f64 = (v4728 / self.scalar_v489);
        let v4730: f64 = (v2985 / self.scalar_v489);
        let v4731: f64 = (v2984 / self.scalar_v489);
        let v4813: f64 = (v2998 / self.scalar_v450);
        let v4814: f64 = (v2985 / self.scalar_v450);
        let v4815: f64 = (v2984 / self.scalar_v450);
        let v4819: f64 = (if v1362 { (v1363 * v4813) } else { (if v1330 { (v1331 * v4729) } else { (if v1327 { (v1328 * v4729) } else { (if v1255 { (v1256 * v4532) } else { (if v1252 { (v1253 * v4532) } else { v4 }) }) }) }) });
        let v4827: f64 = (if v1365 { (v1366 * v4814) } else { (if v1362 { (v1363 * v4814) } else { (if v1330 { (v1331 * v4730) } else { (if v1327 { (v1328 * v4730) } else { (if v1255 { (v1256 * v4533) } else { v4539 }) }) }) }) });
        let v4836: f64 = (v4728 / self.scalar_v533);
        let v4837: f64 = (v2985 / self.scalar_v533);
        let v4838: f64 = (v2984 / self.scalar_v533);
        let v4845: f64 = (if v1374 { v4 } else { (if v1365 { (v1366 * v4815) } else { (if v1362 { (v1363 * v4815) } else { (if v1330 { v4 } else { (if v1327 { v4 } else { (if v1255 { (v1256 * v4534) } else { v4540 }) }) }) }) }) });
        let v4851: f64 = (if v1377 { (v1378 * v4838) } else { (if v1374 { (v1375 * v4838) } else { (if v1365 { v4 } else { (if v1362 { v4 } else { (if v1330 { (v1331 * v4731) } else { (if v1327 { (v1328 * v4731) } else { v4 }) }) }) }) }) });
        let v4859: f64 = (v3017 / self.scalar_v463);
        let v4860: f64 = (v2984 / self.scalar_v463);
        let v4861: f64 = (v3018 / self.scalar_v463);
        let v4862: f64 = (v3019 / self.scalar_v463);
        let v4863: f64 = (v2985 / self.scalar_v463);
        let v4880: f64 = (if v1389 { (v1390 * v4859) } else { (if v1386 { (v1387 * v4859) } else { (if v1377 { (v1378 * v4836) } else { (if v1374 { (v1375 * v4836) } else { (if v1365 { (v1366 * v4813) } else { v4819 }) }) }) }) });
        let v4894: f64 = (v4728 / self.scalar_v543);
        let v4895: f64 = (v2985 / self.scalar_v543);
        let v4896: f64 = (v2984 / self.scalar_v543);
        let v4910: f64 = (if v1401 { (v1402 * v4895) } else { (if v1398 { (v1399 * v4895) } else { (if v1389 { v4 } else { (if v1386 { v4 } else { (if v1377 { (v1378 * v4837) } else { (if v1374 { (v1375 * v4837) } else { v4827 }) }) }) }) }) });
        let v5422: f64 = ((v1140 * v3035) + (v765 * v4188));
        let v5423: f64 = (v1140 * v3036);
        let v5424: f64 = (v1140 * v3037);
        let v5425: f64 = (v1140 * v3038);
        let v5426: f64 = (v1140 * v3039);
        let v5427: f64 = (v423 * (if v802 { (v803 * v3100) } else { (if v799 { (v800 * v3100) } else { v4 }) }));
        let v5428: f64 = (v423 * (if v802 { (v803 * v2984) } else { (if v799 { (v800 * v2984) } else { v4 }) }));
        let v5429: f64 = (v423 * (if v802 { (v803 * v3018) } else { (if v799 { (v800 * v3018) } else { v4 }) }));
        let v5430: f64 = (v423 * (if v802 { (v803 * v3019) } else { (if v799 { (v800 * v3019) } else { v4 }) }));
        let v5431: f64 = (v423 * (if v802 { (v803 * v2985) } else { (if v799 { (v800 * v2985) } else { v4 }) }));
        let v5433: f64 = (v31 * v1593);
        let v5442: f64 = (v1594 * v1594);
        let v5460: f64 = (v31 * v1597);
        let v5469: f64 = (v1598 * v1598);
        let v5545: f64 = (self.scalar_v1616 * v2815);
        let v5557: f64 = ((v1604 * v3072) + (v785 * (((v447 * (v423 * v2815)) - (v1603 * (self.scalar_v442 * (v446 * (self.scalar_v444 * v2477))))) / (v447 * v447))));
        let v5560: f64 = (v31 * v1622);
        let v5569: f64 = (v1623 * v1623);
        let v5596: f64 = (if self.scalar_v1627 { ((v1628 * v2678) + (v327 * (self.scalar_v13 * v2815))) } else { v4 });
        let v5609: f64 = (if self.scalar_v1627 { (-(if self.scalar_v1627 { ((v1633 * v2473) + (v119 * (-(((v1630 * v2476) + (v121 * v5596)) / v1631)))) } else { v4 })) } else { v4 });
        let v5612: f64 = (v1637 * self.scalar_v5607);
        let v5613: f64 = (v5612 + v5612);
        let v5614: f64 = (v1637 * self.scalar_v5608);
        let v5616: f64 = (v1637 * v5609);
        let v5618: f64 = (v1637 * self.scalar_v5610);
        let v5619: f64 = (v5618 + v5618);
        let v5620: f64 = (v1637 * self.scalar_v5611);
        let v5631: f64 = (v31 * v1646);
        let v5632: f64 = ((if self.scalar_v1627 { v5613 } else { v4 }) / v5631);
        let v5633: f64 = ((if self.scalar_v1627 { (v5614 + v5614) } else { v4 }) / v5631);
        let v5634: f64 = ((if self.scalar_v1627 { (v5616 + v5616) } else { v4344 }) / v5631);
        let v5635: f64 = ((if self.scalar_v1627 { v4 } else { v4346 }) / v5631);
        let v5636: f64 = ((if self.scalar_v1627 { v5613 } else { v4348 }) / v5631);
        let v5637: f64 = ((if self.scalar_v1627 { v5619 } else { v4350 }) / v5631);
        let v5638: f64 = ((if self.scalar_v1627 { v5619 } else { v4352 }) / v5631);
        let v5639: f64 = ((if self.scalar_v1627 { (v5620 + v5620) } else { v4 }) / v5631);
        let v5640: f64 = ((if self.scalar_v1627 { v5619 } else { v4 }) / v5631);
        let v5651: f64 = (v1647 * v1647);
        let v5703: f64 = (if v1651 { (v411 * (self.scalar_v5607 + v5632)) } else { (if v1643 { ((-(self.scalar_v1644 * (v5632 - self.scalar_v5607))) / v5651) } else { v4 }) });
        let v5704: f64 = (if v1651 { (v411 * (self.scalar_v5608 + v5633)) } else { (if v1643 { ((-(self.scalar_v1644 * (v5633 - self.scalar_v5608))) / v5651) } else { v4 }) });
        let v5705: f64 = (if v1651 { (v411 * (v5609 + v5634)) } else { (if v1643 { ((-(self.scalar_v1644 * (v5634 - v5609))) / v5651) } else { v4 }) });
        let v5706: f64 = (if v1651 { (v411 * v5635) } else { (if v1643 { ((-(self.scalar_v1644 * v5635)) / v5651) } else { v4 }) });
        let v5707: f64 = (if v1651 { (v411 * (self.scalar_v5607 + v5636)) } else { (if v1643 { ((-(self.scalar_v1644 * (v5636 - self.scalar_v5607))) / v5651) } else { v4 }) });
        let v5708: f64 = (if v1651 { (v411 * (self.scalar_v5610 + v5637)) } else { (if v1643 { ((-(self.scalar_v1644 * (v5637 - self.scalar_v5610))) / v5651) } else { v4 }) });
        let v5709: f64 = (if v1651 { (v411 * (self.scalar_v5610 + v5638)) } else { (if v1643 { ((-(self.scalar_v1644 * (v5638 - self.scalar_v5610))) / v5651) } else { v4 }) });
        let v5710: f64 = (if v1651 { (v411 * (self.scalar_v5611 + v5639)) } else { (if v1643 { ((-(self.scalar_v1644 * (v5639 - self.scalar_v5611))) / v5651) } else { v4 }) });
        let v5711: f64 = (if v1651 { (v411 * (self.scalar_v5610 + v5640)) } else { (if v1643 { ((-(self.scalar_v1644 * (v5640 - self.scalar_v5610))) / v5651) } else { v4 }) });
        let v5712: f64 = (v327 * (if self.scalar_v1613 { (((v1623 * (v1617 * v3070)) - (v1619 * ((v1604 * v3070) / v5560))) / v5569) } else { v4 }));
        let v5716: f64 = ((v1625 * v2678) + (v327 * (if self.scalar_v1613 { (((v1623 * ((v1618 * v5545) + (v1617 * v3072))) - (v1619 * (v5557 / v5560))) / v5569) } else { v4 })));
        let v5717: f64 = (v327 * (if self.scalar_v1613 { (((v1623 * (v1617 * v3073)) - (v1619 * ((v1604 * v3073) / v5560))) / v5569) } else { v4 }));
        let v5731: f64 = (v1657 * v1657);
        let v5735: f64 = ((v1657 * v5704) - (v1654 * (v5704 + (v327 * (if self.scalar_v1613 { (((v1623 * (v1617 * v3071)) - (v1619 * ((v1604 * v3071) / v5560))) / v5569) } else { v4 })))));
        let v5759: f64 = ((v1657 * v5710) - (v1654 * (v5710 + (v327 * (if self.scalar_v1613 { (((v1623 * (v1617 * v3074)) - (v1619 * ((v1604 * v3074) / v5560))) / v5569) } else { v4 })))));
        let v6076: f64 = (v1157 * v4267);
        let v6078: f64 = (v1157 * v4257);
        let v6080: f64 = (v1157 * v4268);
        let v6082: f64 = (v1157 * v4265);
        let v6084: f64 = (v1157 * v4266);
        let v6086: f64 = (v31 * v1723);
        let v6087: f64 = ((v6076 + v6076) / v6086);
        let v6088: f64 = ((v6078 + v6078) / v6086);
        let v6089: f64 = ((v6080 + v6080) / v6086);
        let v6090: f64 = ((v6082 + v6082) / v6086);
        let v6091: f64 = ((v6084 + v6084) / v6086);
        let v6099: f64 = (v1724 * v1724);
        let v6128: f64 = (if v1727 { (v411 * (v4267 + v6087)) } else { (if v1721 { ((-(v1179 * (v6087 - v4267))) / v6099) } else { v4 }) });
        let v6129: f64 = (if v1727 { (v411 * (v4257 + v6088)) } else { (if v1721 { ((-(v1179 * (v6088 - v4257))) / v6099) } else { v4 }) });
        let v6130: f64 = (if v1727 { (v411 * (v4268 + v6089)) } else { (if v1721 { ((-(v1179 * (v6089 - v4268))) / v6099) } else { v4 }) });
        let v6131: f64 = (if v1727 { (v411 * (v4265 + v6090)) } else { (if v1721 { ((-(v1179 * (v6090 - v4265))) / v6099) } else { v4 }) });
        let v6132: f64 = (if v1727 { (v411 * (v4266 + v6091)) } else { (if v1721 { ((-(v1179 * (v6091 - v4266))) / v6099) } else { v4 }) });
        let v7553: f64 = (self.scalar_v2048 * v2652);
        let v7561: f64 = ((v3917 - (v2051 * v3915)) / v3920);
        let v7594: f64 = (if v2060 { (v3914 - ((v2064 * v3915) + (v1059 * ((v2062 * (-v7561)) / v2063)))) } else { (if v2053 { (-((v2056 * v3915) + (v1059 * ((v2054 * v7561) / v2055)))) } else { v4 }) });
        let v7595: f64 = (if v2060 { (-(v1059 * ((v2062 * v3942) / v2063))) } else { (if v2053 { (self.scalar_v2979 - (v1059 * ((v2054 * v3922) / v2055))) } else { v4 }) });
        let v7596: f64 = (if v2060 { (-(v1059 * ((v2062 * v3943) / v2063))) } else { (if v2053 { (self.scalar_v0 - (v1059 * ((v2054 * v3923) / v2055))) } else { v4 }) });
        let v7607: f64 = (self.scalar_v1079 * f64::powf(v2070, self.scalar_v3969));
        let v7630: f64 = ((v2076 * (self.scalar_v2047 * v2652)) + (v2068 * (((v2072 * v3975) + (v1081 * (-((-((v2067 * v2640) + (v284 * v7594))) * v7607)))) + (v170 * (-v7594)))));
        let v7642: f64 = ((v626 * v2754) + (v441 * v2941));
        let v7643: f64 = (v411 * v7642);
        let v7651: f64 = ((v2083 * v6128) + (v1730 * ((v2082 * v4202) + (v1145 * v7643))));
        let v7654: f64 = ((v2083 * v6129) + (v1730 * (v2082 * v4206)));
        let v7657: f64 = ((v2083 * v6130) + (v1730 * (v2082 * v4210)));
        let v7658: f64 = (v2083 * v6131);
        let v7659: f64 = (v2083 * v6132);
        let v7668: f64 = ((v2085 * v6128) + (v1730 * ((v2082 * v4239) + (v1152 * v7643))));
        let v7669: f64 = (v2085 * v6129);
        let v7672: f64 = ((v2085 * v6130) + (v1730 * (v2082 * v4243)));
        let v7675: f64 = ((v2085 * v6131) + (v1730 * (v2082 * v4247)));
        let v7678: f64 = ((v2085 * v6132) + (v1730 * (v2082 * v4251)));
        let v7680: f64 = (v1004 * (-v4018));
        let v7683: f64 = (v1004 * v1004);
        let v7684: f64 = ((v7680 - (v2087 * v3736)) / v7683);
        let v7685: f64 = (self.scalar_v0 / v1004);
        let v7686: f64 = (self.scalar_v2980 / v1004);
        let v7687: f64 = (self.scalar_v2981 / v1004);
        let v7688: f64 = (self.scalar_v2979 / v1004);
        let v7718: f64 = (-v7686);
        let v7719: f64 = (-v7687);
        let v7720: f64 = (-v7688);
        let v7743: f64 = (if v2096 { (v4018 - ((v2100 * v3736) + (v1004 * ((v2098 * (-v7684)) / v2099)))) } else { (if v2089 { (-((v2092 * v3736) + (v1004 * ((v2090 * v7684) / v2091)))) } else { v4 }) });
        let v7744: f64 = (if v2096 { (-(v1004 * ((v2098 * (-v7685)) / v2099))) } else { (if v2089 { (self.scalar_v0 - (v1004 * ((v2090 * v7685) / v2091))) } else { v4 }) });
        let v7745: f64 = (if v2096 { (-(v1004 * ((v2098 * v7718) / v2099))) } else { (if v2089 { (self.scalar_v2980 - (v1004 * ((v2090 * v7686) / v2091))) } else { v4 }) });
        let v7746: f64 = (if v2096 { (-(v1004 * ((v2098 * v7719) / v2099))) } else { (if v2089 { (self.scalar_v2981 - (v1004 * ((v2090 * v7687) / v2091))) } else { v4 }) });
        let v7747: f64 = (if v2096 { (-(v1004 * ((v2098 * v7720) / v2099))) } else { (if v2089 { (self.scalar_v2979 - (v1004 * ((v2090 * v7688) / v2091))) } else { v4 }) });
        let v7762: f64 = (self.scalar_v1124 * f64::powf(v2105, self.scalar_v4116));
        let v7792: f64 = (((v2107 * v4104) + (v1125 * (-((-(((v260 * v7743) - (v2103 * v2614)) / v2642)) * v7762)))) + ((v2109 * v4010) + (v1100 * (-v7743))));
        let v7805: f64 = (v301 * self.scalar_v2980);
        let v7806: f64 = (v301 * self.scalar_v2981);
        let v7825: f64 = (self.scalar_v14 * (self.scalar_v2116 * (v300 * (v4178 + (v1099 * ((v1125 * (-((-(v7744 / v260)) * v7762))) + (v1100 * (self.scalar_v0 - v7744))))))));
        let v7826: f64 = (self.scalar_v14 * (self.scalar_v2116 * (v300 * ((v1099 * ((v1125 * (-((-(v7745 / v260)) * v7762))) + (v1100 * (self.scalar_v2980 - v7745)))) + v7805))));
        let v7827: f64 = (self.scalar_v14 * (self.scalar_v2116 * (v300 * ((v1099 * ((v1125 * (-((-(v7746 / v260)) * v7762))) + (v1100 * (self.scalar_v2981 - v7746)))) + v7806))));
        let v7828: f64 = (self.scalar_v14 * (self.scalar_v2116 * (v300 * (v4179 + (v1099 * ((v1125 * (-((-(v7747 / v260)) * v7762))) + (v1100 * (self.scalar_v2979 - v7747))))))));
        let v7829: f64 = (self.scalar_v2982 / v1004);
        let v7832: f64 = ((v7680 - (v2119 * v3736)) / v7683);
        let v7884: f64 = (if v2128 { (-(v1004 * ((v2130 * v7718) / v2131))) } else { (if v2121 { (self.scalar_v2980 - (v1004 * ((v2122 * v7686) / v2123))) } else { v4 }) });
        let v7885: f64 = (if v2128 { (-(v1004 * ((v2130 * (-v7829)) / v2131))) } else { (if v2121 { (self.scalar_v2982 - (v1004 * ((v2122 * v7829) / v2123))) } else { v4 }) });
        let v7886: f64 = (if v2128 { (v4018 - ((v2132 * v3736) + (v1004 * ((v2130 * (-v7832)) / v2131)))) } else { (if v2121 { (-((v2124 * v3736) + (v1004 * ((v2122 * v7832) / v2123)))) } else { v4 }) });
        let v7887: f64 = (if v2128 { (-(v1004 * ((v2130 * v7719) / v2131))) } else { (if v2121 { (self.scalar_v2981 - (v1004 * ((v2122 * v7687) / v2123))) } else { v4 }) });
        let v7888: f64 = (if v2128 { (-(v1004 * ((v2130 * v7720) / v2131))) } else { (if v2121 { (self.scalar_v2979 - (v1004 * ((v2122 * v7688) / v2123))) } else { v4 }) });
        let v7903: f64 = (self.scalar_v1124 * f64::powf(v2137, self.scalar_v4116));
        let v7935: f64 = (((v2139 * v4104) + (v1125 * (-((-(((v260 * v7886) - (v2135 * v2614)) / v2642)) * v7903)))) + ((v2141 * v4010) + (v1100 * (-v7886))));
        let v7960: f64 = (self.scalar_v2116 * (v300 * ((v1099 * ((v1125 * (-((-(v7885 / v260)) * v7903))) + (v1100 * (self.scalar_v2982 - v7885)))) + (v301 * self.scalar_v2982))));
        let v7964: f64 = (self.scalar_v13 * (self.scalar_v2116 * (v300 * (v7805 + (v1099 * ((v1125 * (-((-(v7884 / v260)) * v7903))) + (v1100 * (self.scalar_v2980 - v7884))))))));
        let v7967: f64 = (self.scalar_v13 * (self.scalar_v2116 * (v300 * (v7806 + (v1099 * ((v1125 * (-((-(v7887 / v260)) * v7903))) + (v1100 * (self.scalar_v2981 - v7887))))))));
        let v7968: f64 = (self.scalar_v13 * (self.scalar_v2116 * (v300 * (v4179 + (v1099 * ((v1125 * (-((-(v7888 / v260)) * v7903))) + (v1100 * (self.scalar_v2979 - v7888))))))));
        let v7980: f64 = (v2154 * ((v620 * v2754) + (v441 * ((v619 * (self.scalar_v611 * (v614 * (self.scalar_v612 * v2477)))) + (v615 * (v619 * (self.scalar_v617 * v2476)))))));
        let v7983: f64 = (self.scalar_v2152 * v2473);
        let v7986: f64 = (v2156 * v2156);
        let v7987: f64 = ((-(v700 * v7983)) / v7986);
        let v7988: f64 = (self.scalar_v2979 / v2156);
        let v7989: f64 = (self.scalar_v0 / v2156);
        let v7996: f64 = (if v2158 { (v2159 * v7989) } else { (if v1401 { v4 } else { (if v1398 { v4 } else { (if v1389 { (v1390 * v4861) } else { (if v1386 { (v1387 * v4861) } else { (if v1377 { v4 } else { v4845 }) }) }) }) }) });
        let v8004: f64 = (if v2161 { v4 } else { (if v2158 { v4 } else { (if v1401 { (v1402 * v4896) } else { (if v1398 { (v1399 * v4896) } else { (if v1389 { (v1390 * v4860) } else { (if v1386 { (v1387 * v4860) } else { v4851 }) }) }) }) }) });
        let v8010: f64 = ((v2166 * (v7980 + (v2150 * ((((v441 * v2751) - (v436 * v2754)) / v4187) * (self.scalar_v2153 * f64::powf(v2151, self.scalar_v7976)))))) + (v2155 * (if v2161 { (v2162 * v7987) } else { (if v2158 { (v2159 * v7987) } else { (if v1401 { (v1402 * v4894) } else { (if v1398 { (v1399 * v4894) } else { v4880 }) }) }) })));
        let v8011: f64 = (v2155 * (if v2161 { (v2162 * v7988) } else { (if v2158 { (v2159 * v7988) } else { v4910 }) }));
        let v8012: f64 = (v2155 * v8004);
        let v8013: f64 = (v2155 * (if v2161 { (v2162 * v7989) } else { v7996 }));
        let v8014: f64 = (v2155 * (if v2161 { v4 } else { (if v2158 { v4 } else { (if v1401 { v4 } else { (if v1398 { v4 } else { (if v1389 { (v1390 * v4862) } else { (if v1386 { (v1387 * v4862) } else { v4 }) }) }) }) }) }));
        let v8015: f64 = (v2155 * (if v2161 { v4 } else { (if v2158 { v4 } else { (if v1401 { v4 } else { (if v1398 { v4 } else { (if v1389 { (v1390 * v4863) } else { (if v1386 { (v1387 * v4863) } else { v4 }) }) }) }) }) }));
        let v8023: f64 = (((v339 * ((v2168 * v2473) + (v119 * (v423 * v2944)))) - (v2169 * v2685)) / v3216);
        let v8025: f64 = (v2171 * (if v1044 { (((v1046 * v3207) - (v847 * v3207)) / v3872) } else { (if v1036 { (((v1040 * v3841) - (v1039 * v3841)) / v3848) } else { v3548 }) }));
        let v8030: f64 = (v2171 * (if v1044 { (((v1046 * v3210) - (v847 * v3212)) / v3872) } else { (if v1036 { (((v1040 * v3844) - (v1039 * v3844)) / v3848) } else { v3551 }) }));
        let v8058: f64 = (((v2081 * (((v1594 * (v5422 - v4188)) - (v1591 * (v5422 / v5433))) / v5442)) + (v1595 * v7642)) + ((v2170 * (((v1598 * v5427) - (v1590 * (v5427 / v5460))) / v5469)) + (v1599 * v8023)));
        let v8059: f64 = ((v2081 * (((v1594 * v5423) - (v1591 * (v5423 / v5433))) / v5442)) + (v2170 * (((v1598 * v5428) - (v1590 * (v5428 / v5460))) / v5469)));
        let v8060: f64 = ((v2081 * (((v1594 * v5424) - (v1591 * (v5424 / v5433))) / v5442)) + (v2170 * (((v1598 * v5429) - (v1590 * (v5429 / v5460))) / v5469)));
        let v8061: f64 = ((v2081 * (((v1594 * v5425) - (v1591 * (v5425 / v5433))) / v5442)) + (v2170 * (((v1598 * v5430) - (v1590 * (v5430 / v5460))) / v5469)));
        let v8062: f64 = ((v2081 * (((v1594 * v5426) - (v1591 * (v5426 / v5433))) / v5442)) + (v2170 * (((v1598 * v5431) - (v1590 * (v5431 / v5460))) / v5469)));
        let v8073: f64 = (v633 * v633);
        let v8084: f64 = (-v2592);
        let v8092: f64 = ((v2186 * v2476) + (v121 * (v8084 / self.scalar_v2185)));
        let v8093: f64 = (v121 * self.scalar_v8086);
        let v8094: f64 = (v121 * self.scalar_v8087);
        let v8095: f64 = (v121 * self.scalar_v8088);
        let v8096: f64 = (v121 * self.scalar_v8089);
        let v8132: f64 = (v31 * v2204);
        let v8140: f64 = ((v2205 * ((v2200 * v3035) + (v765 * ((v1600 * v2950) + (v642 * (v31 * v2815)))))) - (v2201 * ((v423 * (if v2194 { (v2195 * v8092) } else { (if v2190 { (v2191 * v8092) } else { v4 }) })) / v8132)));
        let v8141: f64 = (v2205 * v2205);
        let v8146: f64 = (((v2205 * (v2200 * v3036)) - (v2201 * ((v423 * (if v2194 { (v2195 * v8093) } else { (if v2190 { (v2191 * v8093) } else { v4 }) })) / v8132))) / v8141);
        let v8150: f64 = (((v2205 * (v2200 * v3037)) - (v2201 * ((v423 * (if v2194 { (v2195 * v8094) } else { (if v2190 { (v2191 * v8094) } else { v4 }) })) / v8132))) / v8141);
        let v8154: f64 = (((v2205 * (v2200 * v3038)) - (v2201 * ((v423 * (if v2194 { (v2195 * v8095) } else { (if v2190 { (v2191 * v8095) } else { v4 }) })) / v8132))) / v8141);
        let v8158: f64 = (((v2205 * (v2200 * v3039)) - (v2201 * ((v423 * (if v2194 { (v2195 * v8096) } else { (if v2190 { (v2191 * v8096) } else { v4 }) })) / v8132))) / v8141);
        let v8159: f64 = (if self.scalar_v2189 { (v8140 / v8141) } else { (if self.scalar_v2176 { (((v633 * ((v2180 * (v411 * v2947)) + (v2177 * v8058))) - (v2181 * v2945)) / v8073) } else { v4 }) });
        let v8160: f64 = (if self.scalar_v2189 { v8146 } else { (if self.scalar_v2176 { ((v2177 * v8059) / v633) } else { v4 }) });
        let v8161: f64 = (if self.scalar_v2189 { v8150 } else { (if self.scalar_v2176 { ((v2177 * v8060) / v633) } else { v4 }) });
        let v8162: f64 = (if self.scalar_v2189 { v8154 } else { (if self.scalar_v2176 { ((v2177 * v8061) / v633) } else { v4 }) });
        let v8163: f64 = (if self.scalar_v2189 { v8158 } else { (if self.scalar_v2176 { ((v2177 * v8062) / v633) } else { v4 }) });
        let v8181: f64 = (if self.scalar_v2213 { (v1140 * v3070) } else { v4 });
        let v8182: f64 = (if self.scalar_v2213 { (v1140 * v3071) } else { v4 });
        let v8183: f64 = (if self.scalar_v2213 { ((v1140 * v3072) + (v785 * v4188)) } else { v4 });
        let v8184: f64 = (if self.scalar_v2213 { (v1140 * v3073) } else { v4 });
        let v8185: f64 = (if self.scalar_v2213 { (v1140 * v3074) } else { v4 });
        let v8187: f64 = (v31 * v2218);
        let v8196: f64 = (v2219 * v2219);
        let v8224: f64 = (if self.scalar_v2213 { (v423 * (if v791 { (v792 * v3018) } else { (if v788 { (v789 * v3018) } else { v4 }) })) } else { v4 });
        let v8225: f64 = (if self.scalar_v2213 { (v423 * (if v791 { (v792 * v3053) } else { (if v788 { (v789 * v3053) } else { v4 }) })) } else { v4 });
        let v8226: f64 = (if self.scalar_v2213 { (v423 * (if v791 { (v792 * v3078) } else { (if v788 { (v789 * v3078) } else { v4 }) })) } else { v4 });
        let v8227: f64 = (if self.scalar_v2213 { (v423 * (if v791 { (v792 * v3019) } else { (if v788 { (v789 * v3019) } else { v4 }) })) } else { v4 });
        let v8228: f64 = (if self.scalar_v2213 { (v423 * (if v791 { (v792 * v2985) } else { (if v788 { (v789 * v2985) } else { v4 }) })) } else { v4 });
        let v8229: f64 = (v31 * v2225);
        let v8238: f64 = (v2226 * v2226);
        let v8276: f64 = ((v2081 * (if self.scalar_v2213 { (((v2219 * v8181) - (v2216 * (v8181 / v8187))) / v8196) } else { v4 })) + (v2170 * (if self.scalar_v2213 { (((v2226 * v8224) - (v2223 * (v8224 / v8229))) / v8238) } else { v4 })));
        let v8277: f64 = ((v2081 * (if self.scalar_v2213 { (((v2219 * v8182) - (v2216 * (v8182 / v8187))) / v8196) } else { v4 })) + (v2170 * (if self.scalar_v2213 { (((v2226 * v8225) - (v2223 * (v8225 / v8229))) / v8238) } else { v4 })));
        let v8278: f64 = (((v2221 * v7642) + (v2081 * (if self.scalar_v2213 { (((v2219 * (v8183 - v4188)) - (v2216 * (v8183 / v8187))) / v8196) } else { v4 }))) + ((v2228 * v8023) + (v2170 * (if self.scalar_v2213 { (((v2226 * v8226) - (v2223 * (v8226 / v8229))) / v8238) } else { v4 }))));
        let v8279: f64 = ((v2081 * (if self.scalar_v2213 { (((v2219 * v8184) - (v2216 * (v8184 / v8187))) / v8196) } else { v4 })) + (v2170 * (if self.scalar_v2213 { (((v2226 * v8227) - (v2223 * (v8227 / v8229))) / v8238) } else { v4 })));
        let v8280: f64 = ((v2081 * (if self.scalar_v2213 { (((v2219 * v8185) - (v2216 * (v8185 / v8187))) / v8196) } else { v4 })) + (v2170 * (if self.scalar_v2213 { (((v2226 * v8228) - (v2223 * (v8228 / v8229))) / v8238) } else { v4 })));
        let v8303: f64 = ((v2237 * v2476) + (v121 * v8084));
        let v8339: f64 = (v31 * v2255);
        let v8348: f64 = (v2256 * v2256);
        let v8349: f64 = (((v2256 * (v2251 * v3070)) - (v2252 * ((v423 * (if v2245 { (v2246 * v3018) } else { (if v2241 { (v2242 * v3018) } else { v4 }) })) / v8339))) / v8348);
        let v8353: f64 = (((v2256 * (v2251 * v3071)) - (v2252 * ((v423 * (if v2245 { (v2246 * v3053) } else { (if v2241 { (v2242 * v3053) } else { v4 }) })) / v8339))) / v8348);
        let v8356: f64 = ((v2256 * ((v2251 * v3072) + (v785 * ((v1617 * v2950) + (v642 * v5545))))) - (v2252 * ((v423 * (if v2245 { (v2246 * v8303) } else { (if v2241 { (v2242 * v8303) } else { v4 }) })) / v8339)));
        let v8361: f64 = (((v2256 * (v2251 * v3073)) - (v2252 * ((v423 * (if v2245 { (v2246 * v3019) } else { (if v2241 { (v2242 * v3019) } else { v4 }) })) / v8339))) / v8348);
        let v8365: f64 = (((v2256 * (v2251 * v3074)) - (v2252 * ((v423 * (if v2245 { (v2246 * v2985) } else { (if v2241 { (v2242 * v2985) } else { v4 }) })) / v8339))) / v8348);
        let v8368: f64 = (if self.scalar_v2240 { (v8356 / v8348) } else { (if self.scalar_v2213 { (((v633 * ((v2233 * (self.scalar_v2229 * v2947)) + (v2230 * v8278))) - (v2234 * v2945)) / v8073) } else { v4 }) });
        let v8372: f64 = (v1662 * (if self.scalar_v2240 { v8349 } else { (if self.scalar_v2213 { ((v2230 * v8276) / v633) } else { v4 }) }));
        let v8376: f64 = ((v2258 * (if self.scalar_v1661 { v4 } else { (if self.scalar_v1627 { (v5735 / v5731) } else { v4 }) })) + (v1662 * (if self.scalar_v2240 { v8353 } else { (if self.scalar_v2213 { ((v2230 * v8277) / v633) } else { v4 }) })));
        let v8379: f64 = ((v2258 * (if self.scalar_v1661 { v4 } else { (if self.scalar_v1627 { (((v1657 * v5705) - (v1654 * (v5705 + (v5596 + v5716)))) / v5731) } else { v4 }) })) + (v1662 * v8368));
        let v8384: f64 = (v1662 * (if self.scalar_v2240 { v8361 } else { (if self.scalar_v2213 { ((v2230 * v8279) / v633) } else { v4 }) }));
        let v8390: f64 = ((v2258 * (if self.scalar_v1661 { v4 } else { (if self.scalar_v1627 { (v5759 / v5731) } else { v4 }) })) + (v1662 * (if self.scalar_v2240 { v8365 } else { (if self.scalar_v2213 { ((v2230 * v8280) / v633) } else { v4 }) })));
        let v8404: f64 = (self.scalar_v2263 * f64::powf(v1078, self.scalar_v8402));
        let v8411: f64 = (if self.scalar_v2262 { v3921 } else { v4 });
        let v8412: f64 = (if self.scalar_v2262 { v3922 } else { v4 });
        let v8413: f64 = (if self.scalar_v2262 { v3923 } else { v4 });
        let v8418: f64 = (v2271 * v2271);
        let v8430: f64 = (v2277 * (-v8411));
        let v8431: f64 = (v2277 * (-v8412));
        let v8432: f64 = (v2277 * (-v8413));
        let v8436: f64 = (v2278 * v2278);
        let v8451: f64 = ((v2280 * (if self.scalar_v2262 { (v3966 * v8404) } else { v4 })) + (v2266 * (if v2275 { (((v2278 * v8430) - (v2277 * v8430)) / v8436) } else { (if v2269 { ((-(v2270 * v8411)) / v8418) } else { v4 }) })));
        let v8454: f64 = ((v2280 * (if self.scalar_v2262 { (v3967 * v8404) } else { v4 })) + (v2266 * (if v2275 { (((v2278 * v8431) - (v2277 * v8431)) / v8436) } else { (if v2269 { ((-(v2270 * v8412)) / v8418) } else { v4 }) })));
        let v8457: f64 = ((v2280 * (if self.scalar_v2262 { (v3968 * v8404) } else { v4 })) + (v2266 * (if v2275 { (((v2278 * v8432) - (v2277 * v8432)) / v8436) } else { (if v2269 { ((-(v2270 * v8413)) / v8418) } else { v4 }) })));
        let v8482: f64 = (v1143 * v1143);
        let v8492: f64 = ((v2288 * (((v371 * ((v1141 * v2476) + (v121 * v4191))) - (v2286 * v2702)) / v2739)) + (v2287 * ((-(v411 * v4195)) / v8482)));
        let v8514: f64 = ((v2291 * (if self.scalar_v2262 { ((v2288 * ((v121 * v4192) / v371)) + (v2287 * ((-(v411 * v4196)) / v8482))) } else { v4 })) + (v2290 * (v2082 * v6129)));
        let v8517: f64 = ((v2291 * (if self.scalar_v2262 { ((v2288 * ((v121 * v4193) / v371)) + (v2287 * ((-(v411 * v4197)) / v8482))) } else { v4 })) + (v2290 * (v2082 * v6130)));
        let v8538: f64 = (if self.scalar_v2262 { (v8014 / v2156) } else { v4 });
        let v8542: f64 = ((if self.scalar_v2262 { ((v2283 * v7553) + (v2049 * (if self.scalar_v2262 { v8451 } else { v4 }))) } else { v4 }) + (if self.scalar_v2262 { ((v2291 * (if self.scalar_v2262 { v8492 } else { v4 })) + (v2290 * ((v2082 * v6128) + (v1730 * v7643)))) } else { v4 }));
        let v8557: f64 = ((v2298 * self.scalar_v8541) + (v2296 * ((if self.scalar_v2262 { (v8013 / v2156) } else { v4 }) + ((if self.scalar_v2262 { (v2049 * (if self.scalar_v2262 { v8457 } else { v4 })) } else { v4 }) + (if self.scalar_v2262 { v8517 } else { v4 })))));
        let v8562: f64 = (if self.scalar_v2262 { (v2296 * ((if self.scalar_v2262 { (v8011 / v2156) } else { v4 }) + ((if self.scalar_v2262 { (v2049 * (if self.scalar_v2262 { v8454 } else { v4 })) } else { v4 }) + (if self.scalar_v2262 { v8514 } else { v4 })))) } else { v4 });
        let v8584: f64 = (self.scalar_v2301 * v8014);
        let v8591: f64 = (if self.scalar_v2262 { (v7651 + (self.scalar_v2301 * v8010)) } else { v4 });
        let v8592: f64 = (if self.scalar_v2262 { (v7654 + (self.scalar_v2301 * v8011)) } else { v4 });
        let v8593: f64 = (if self.scalar_v2262 { (self.scalar_v2301 * v8012) } else { v4 });
        let v8594: f64 = (if self.scalar_v2262 { (v7657 + (self.scalar_v2301 * v8013)) } else { v4 });
        let v8595: f64 = (if self.scalar_v2262 { (v7658 + v8584) } else { v4 });
        let v8596: f64 = (if self.scalar_v2262 { (v7659 + v8584) } else { v4 });
        let v8597: f64 = (if self.scalar_v2262 { (self.scalar_v2301 * v8015) } else { v4 });
        let v8631: f64 = (if self.scalar_v2315 { v7651 } else { (if self.scalar_v2262 { (self.scalar_v2312 * v8591) } else { v4 }) });
        let v8632: f64 = (if self.scalar_v2315 { v7654 } else { (if self.scalar_v2262 { (self.scalar_v2312 * v8592) } else { v4 }) });
        let v8633: f64 = (if self.scalar_v2315 { v4 } else { (if self.scalar_v2262 { (self.scalar_v2312 * v8593) } else { v4 }) });
        let v8634: f64 = (if self.scalar_v2315 { v7657 } else { (if self.scalar_v2262 { (self.scalar_v2312 * v8594) } else { v4 }) });
        let v8635: f64 = (if self.scalar_v2315 { v7658 } else { (if self.scalar_v2262 { (self.scalar_v2312 * v8595) } else { v4 }) });
        let v8636: f64 = (if self.scalar_v2315 { v7659 } else { (if self.scalar_v2262 { (self.scalar_v2312 * v8596) } else { v4 }) });
        let v8637: f64 = (if self.scalar_v2315 { v4 } else { (if self.scalar_v2262 { (self.scalar_v2312 * v8597) } else { v4 }) });
        let v8638: f64 = (if self.scalar_v2315 { v7668 } else { (if self.scalar_v2262 { (v7668 + (self.scalar_v2308 * v8591)) } else { v4 }) });
        let v8639: f64 = (if self.scalar_v2315 { v7669 } else { (if self.scalar_v2262 { (v7669 + (self.scalar_v2308 * v8592)) } else { v4 }) });
        let v8640: f64 = (if self.scalar_v2315 { v4 } else { (if self.scalar_v2262 { (self.scalar_v2308 * v8593) } else { v4 }) });
        let v8641: f64 = (if self.scalar_v2315 { v7672 } else { (if self.scalar_v2262 { (v7672 + (self.scalar_v2308 * v8594)) } else { v4 }) });
        let v8642: f64 = (if self.scalar_v2315 { v7675 } else { (if self.scalar_v2262 { (v7675 + (self.scalar_v2308 * v8595)) } else { v4 }) });
        let v8643: f64 = (if self.scalar_v2315 { v7678 } else { (if self.scalar_v2262 { (v7678 + (self.scalar_v2308 * v8596)) } else { v4 }) });
        let v8644: f64 = (if self.scalar_v2315 { v4 } else { (if self.scalar_v2262 { (self.scalar_v2308 * v8597) } else { v4 }) });
        let v8649: f64 = (if self.scalar_v2315 { v8014 } else { (if self.scalar_v2262 { (self.scalar_v2302 * v8014) } else { v4 }) });
        let v8651: f64 = 1.0;
        let v8653: f64 = (self.scalar_v27 * (self.scalar_v2319 * v8651));
        let v8674: f64 = (((v1192 * (((v1194 * v4220) + (v1147 * (self.scalar_v1193 * v2751))) + ((v755 * v2751) + (v436 * v3014)))) - (v2355 * v4409)) / v4441);
        let v8690: f64 = (v2356 * v2356);
        let v8710: f64 = (((v2356 * (v8634 + v8641)) - (v2365 * (((v1192 * ((v1194 * v4221) + (v436 * v3016))) - (v2355 * v4415)) / v4441))) / v8690);
        let v8749: f64 = (if v2368 { ((v2369 * v4409) + (v1192 * ((v1730 * v2941) + (v626 * v6128)))) } else { (if v2364 { (((v2356 * (v8631 + v8638)) - (v2365 * v8674)) / v8690) } else { v4 }) });
        let v8750: f64 = (if v2368 { ((v2369 * v4412) + (v1192 * (v626 * v6129))) } else { (if v2364 { (((v2356 * (v8632 + v8639)) - (v2365 * (((v1192 * (v436 * v3015)) - (v2355 * v4412)) / v4441))) / v8690) } else { v4 }) });
        let v8751: f64 = (if v2368 { v4 } else { (if v2364 { ((v8633 + v8640) / v2356) } else { v4 }) });
        let v8752: f64 = (if v2368 { ((v2369 * v4415) + (v1192 * (v626 * v6130))) } else { (if v2364 { v8710 } else { v4 }) });
        let v8753: f64 = (if v2368 { ((v2369 * v4418) + (v1192 * (v626 * v6131))) } else { (if v2364 { (((v2356 * (v8635 + v8642)) - (v2365 * (((v1192 * (v1194 * v4222)) - (v2355 * v4418)) / v4441))) / v8690) } else { v4 }) });
        let v8754: f64 = (if v2368 { ((v2369 * v4421) + (v1192 * (v626 * v6132))) } else { (if v2364 { (((v2356 * (v8636 + v8643)) - (v2365 * (((v1192 * (v1194 * v4223)) - (v2355 * v4421)) / v4441))) / v8690) } else { v4 }) });
        let v8755: f64 = (if v2368 { v4 } else { (if v2364 { ((v8637 + v8644) / v2356) } else { v4 }) });
        let v8815: f64 = (((v2173 * (v8025 + (v1048 * (v411 * v8023)))) + (v2172 * v3834)) + (((v2079 * v4180) + (v1138 * (self.scalar_v2078 * v2663))) + v8638));
        let v8819: f64 = ((self.scalar_v14 * (self.scalar_v2116 * ((v2114 * v2663) + (v300 * (((v2111 * v4005) + (v1099 * v7792)) + (v727 * v2664)))))) + (if self.scalar_v2210 { (self.scalar_v14 * v8159) } else { v8159 }));
        let v8824: f64 = (v7964 + (if self.scalar_v2210 { ((v2258 * (if self.scalar_v1661 { v4 } else { (if self.scalar_v1627 { (((v1657 * v5703) - (v1654 * (v5703 + v5712))) / v5731) } else { v4 }) })) + v8372) } else { v4 }));
        let v8826: f64 = ((self.scalar_v13 * (self.scalar_v2116 * ((v2146 * v2663) + (v300 * (((v2143 * v4005) + (v1099 * v7935)) + (v732 * v2664)))))) + (if self.scalar_v2210 { v8379 } else { v4 }));
        let v8827: f64 = (v7964 + (if self.scalar_v2210 { (v8372 + (v2258 * (if self.scalar_v1661 { v4 } else { (if self.scalar_v1627 { (((v1657 * v5707) - (v1654 * (v5707 + v5712))) / v5731) } else { v4 }) }))) } else { v4 }));
        let v8828: f64 = (v7967 + (if self.scalar_v2210 { ((v2258 * (if self.scalar_v1661 { v4 } else { (if self.scalar_v1627 { (((v1657 * v5708) - (v1654 * (v5708 + v5717))) / v5731) } else { v4 }) })) + v8384) } else { v4 }));
        let v8829: f64 = (v7967 + (if self.scalar_v2210 { (v8384 + (v2258 * (if self.scalar_v1661 { v4 } else { (if self.scalar_v1627 { (((v1657 * v5709) - (v1654 * (v5709 + v5717))) / v5731) } else { v4 }) }))) } else { v4 }));
        let v8831: f64 = (v7967 + (if self.scalar_v2210 { (v8384 + (v2258 * (if self.scalar_v1661 { v4 } else { (if self.scalar_v1627 { (((v1657 * v5711) - (v1654 * (v5711 + v5717))) / v5731) } else { v4 }) }))) } else { v4 }));
        let v9000: f64 = (self.scalar_v27 * (v8651 * (self.scalar_v0 * ((if self.scalar_v2315 { v8010 } else { (if self.scalar_v2262 { (self.scalar_v2302 * v8010) } else { v4 }) }) + (((v2049 * v3990) + (v1086 * v7553)) + v8631)))));
        let v9001: f64 = (self.scalar_v27 * (v8651 * (self.scalar_v0 * ((if self.scalar_v2315 { v8011 } else { (if self.scalar_v2262 { (self.scalar_v2302 * v8011) } else { v4 }) }) + ((v2049 * v3991) + v8632)))));
        let v9002: f64 = (self.scalar_v27 * (v8651 * (self.scalar_v0 * (v8633 + (if self.scalar_v2315 { v8012 } else { (if self.scalar_v2262 { (self.scalar_v2302 * v8012) } else { v4 }) })))));
        let v9003: f64 = (self.scalar_v27 * (v8651 * (self.scalar_v0 * ((if self.scalar_v2315 { v8013 } else { (if self.scalar_v2262 { (self.scalar_v2302 * v8013) } else { v4 }) }) + ((v2049 * v3992) + v8634)))));
        let v9004: f64 = (self.scalar_v27 * (v8651 * (self.scalar_v0 * (v8635 + v8649))));
        let v9005: f64 = (self.scalar_v27 * (v8651 * (self.scalar_v0 * (v8636 + v8649))));
        let v9006: f64 = (self.scalar_v27 * (v8651 * (self.scalar_v0 * (v8637 + (if self.scalar_v2315 { v8015 } else { (if self.scalar_v2262 { (self.scalar_v2302 * v8015) } else { v4 }) })))));
        let v9013: f64 = (self.scalar_v27 * (v8651 * (self.scalar_v0 * v7630)));
        let v9014: f64 = (self.scalar_v27 * (v8651 * (self.scalar_v0 * (v2068 * ((v1081 * (-((-(v284 * v7595)) * v7607))) + (v170 * (self.scalar_v2979 - v7595)))))));
        let v9015: f64 = (self.scalar_v27 * (v8651 * (self.scalar_v0 * (v2068 * ((v1081 * (-((-(v284 * v7596)) * v7607))) + (v170 * (self.scalar_v0 - v7596)))))));
        let v9030: f64 = (self.scalar_v27 * (v8651 * (self.scalar_v0 * v8815)));
        let v9031: f64 = (self.scalar_v27 * (v8651 * (self.scalar_v0 * v8639)));
        let v9032: f64 = (self.scalar_v27 * (v8651 * (self.scalar_v0 * v8640)));
        let v9033: f64 = (self.scalar_v27 * (v8651 * (self.scalar_v0 * (((v2173 * (v2171 * v3887)) + (v2172 * v3835)) + ((v2079 * v4181) + v8641)))));
        let v9034: f64 = (self.scalar_v27 * (v8651 * (self.scalar_v0 * (((v2173 * (v2171 * v3888)) + (v2172 * v3836)) + ((v2079 * v4182) + v8642)))));
        let v9035: f64 = (self.scalar_v27 * (v8651 * (self.scalar_v0 * (((v2173 * v8030) + (v2172 * v3829)) + ((v2079 * v4176) + v8643)))));
        let v9036: f64 = (self.scalar_v27 * (v8651 * (self.scalar_v0 * v8644)));
        let v9051: f64 = (self.scalar_v27 * (v8651 * (self.scalar_v0 * (if self.scalar_v2262 { (v2296 * ((if self.scalar_v2262 { (((v2156 * v8010) - (v2167 * v7983)) / v7986) } else { v4 }) + v8542)) } else { v4 }))));
        let v9052: f64 = (self.scalar_v27 * (v8651 * (self.scalar_v0 * v8562)));
        let v9053: f64 = (self.scalar_v27 * (v8651 * (self.scalar_v0 * (if self.scalar_v2262 { ((v2298 * self.scalar_v8540) + (v2296 * (if self.scalar_v2262 { (v8012 / v2156) } else { v4 }))) } else { v4 }))));
        let v9054: f64 = (self.scalar_v27 * (v8651 * (self.scalar_v0 * (if self.scalar_v2262 { v8557 } else { v4 }))));
        let v9055: f64 = (self.scalar_v27 * (v8651 * (self.scalar_v0 * (if self.scalar_v2262 { (v2296 * ((if self.scalar_v2262 { (v2290 * (v2082 * v6131)) } else { v4 }) + v8538)) } else { v4 }))));
        let v9056: f64 = (self.scalar_v27 * (v8651 * (self.scalar_v0 * (if self.scalar_v2262 { (v2296 * ((if self.scalar_v2262 { (v2290 * (v2082 * v6132)) } else { v4 }) + v8538)) } else { v4 }))));
        let v9057: f64 = (self.scalar_v27 * (v8651 * (self.scalar_v0 * (if self.scalar_v2262 { (v2296 * (if self.scalar_v2262 { (v8015 / v2156) } else { v4 })) } else { v4 }))));
        let v9062: f64 = (self.scalar_v27 * (v8651 * self.scalar_v9058));
        let v9063: f64 = (self.scalar_v27 * (v8651 * self.scalar_v9059));
        let v9068: f64 = (self.scalar_v27 * (v8651 * self.scalar_v9064));
        let v9069: f64 = (self.scalar_v27 * (v8651 * self.scalar_v9065));
        let v9120: f64 = (self.scalar_v27 * (v8651 * (self.scalar_v0 * v8824)));
        let v9121: f64 = (self.scalar_v27 * (v8651 * (self.scalar_v0 * ((self.scalar_v13 * v7960) + (if self.scalar_v2210 { v8376 } else { v4 })))));
        let v9122: f64 = (self.scalar_v27 * (v8651 * (self.scalar_v0 * v8826)));
        let v9123: f64 = (self.scalar_v27 * (v8651 * (self.scalar_v0 * (if self.scalar_v2210 { (v2258 * (if self.scalar_v1661 { v4 } else { (if self.scalar_v1627 { (((v1657 * v5706) - (v1654 * v5706)) / v5731) } else { v4 }) })) } else { v4 }))));
        let v9124: f64 = (self.scalar_v27 * (v8651 * (self.scalar_v0 * v8827)));
        let v9125: f64 = (self.scalar_v27 * (v8651 * (self.scalar_v0 * v8828)));
        let v9126: f64 = (self.scalar_v27 * (v8651 * (self.scalar_v0 * v8829)));
        let v9127: f64 = (self.scalar_v27 * (v8651 * (self.scalar_v0 * (v7968 + (if self.scalar_v2210 { v8390 } else { v4 })))));
        let v9128: f64 = (self.scalar_v27 * (v8651 * (self.scalar_v0 * v8831)));
        let v9164: f64 = (self.scalar_v27 * (v8651 * (self.scalar_v0 * v8819)));
        let v9165: f64 = (self.scalar_v27 * (v8651 * (self.scalar_v0 * (v7825 + (if self.scalar_v2210 { (self.scalar_v14 * v8160) } else { v8160 })))));
        let v9166: f64 = (self.scalar_v27 * (v8651 * (self.scalar_v0 * (v7826 + (if self.scalar_v2210 { (self.scalar_v14 * v8161) } else { v8161 })))));
        let v9167: f64 = (self.scalar_v27 * (v8651 * (self.scalar_v0 * (v7827 + (if self.scalar_v2210 { (self.scalar_v14 * v8162) } else { v8162 })))));
        let v9168: f64 = (self.scalar_v27 * (v8651 * (self.scalar_v0 * (v7828 + (if self.scalar_v2210 { (self.scalar_v14 * v8163) } else { v8163 })))));
        let v9187: f64 = (v2463 * (if self.scalar_v2383 { v4 } else { (if self.scalar_v2378 { (self.scalar_v2379 * v8749) } else { (if self.scalar_v2373 { (self.scalar_v2308 * v8749) } else { v4 }) }) }));
        let v9188: f64 = (v2463 * (if self.scalar_v2383 { v4 } else { (if self.scalar_v2378 { (self.scalar_v2379 * v8750) } else { (if self.scalar_v2373 { (self.scalar_v2308 * v8750) } else { v4 }) }) }));
        let v9189: f64 = (v2463 * (if self.scalar_v2383 { v4 } else { (if self.scalar_v2378 { (self.scalar_v2379 * v8751) } else { (if self.scalar_v2373 { (self.scalar_v2308 * v8751) } else { v4 }) }) }));
        let v9190: f64 = (v2463 * (if self.scalar_v2383 { v4 } else { (if self.scalar_v2378 { (self.scalar_v2379 * v8752) } else { (if self.scalar_v2373 { (self.scalar_v2308 * v8752) } else { v4 }) }) }));
        let v9191: f64 = (v2463 * (if self.scalar_v2383 { v4 } else { (if self.scalar_v2378 { (self.scalar_v2379 * v8753) } else { (if self.scalar_v2373 { (self.scalar_v2308 * v8753) } else { v4 }) }) }));
        let v9192: f64 = (v2463 * (if self.scalar_v2383 { v4 } else { (if self.scalar_v2378 { (self.scalar_v2379 * v8754) } else { (if self.scalar_v2373 { (self.scalar_v2308 * v8754) } else { v4 }) }) }));
        let v9193: f64 = (v2463 * (if self.scalar_v2383 { v4 } else { (if self.scalar_v2378 { (self.scalar_v2379 * v8755) } else { (if self.scalar_v2373 { (self.scalar_v2308 * v8755) } else { v4 }) }) }));
        let v9194: f64 = (v2384 * v8651);

        let d2322_dn3: f64 = v8653;
        stamper.stamp_current_reactive_node1(
            Some(nodes[3]),
            None,
            nodes[3],
            multiplicity * (d2322_dn3),
        );
        let d2422_dn3: f64 = v9000;
        let d2422_dn4: f64 = v9001;
        let d2422_dn5: f64 = v9002;
        let d2422_dn6: f64 = v9003;
        let d2422_dn7: f64 = v9004;
        let d2422_dn8: f64 = v9005;
        let d2422_dn10: f64 = v9006;
        let v2422_reactive_nodes: [usize; 7] = [nodes[3], nodes[4], nodes[5], nodes[6], nodes[7], nodes[8], nodes[10]];
        let v2422_reactive_node_derivatives: [f64; 7] = [d2422_dn3, d2422_dn4, d2422_dn5, d2422_dn6, d2422_dn7, d2422_dn8, d2422_dn10];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[4]),
            &v2422_reactive_nodes,
            &v2422_reactive_node_derivatives,
            &[],
            &[],
            multiplicity,
        );
        let d2425_dn3: f64 = v9013;
        let d2425_dn4: f64 = v9014;
        let d2425_dn5: f64 = v9015;
        stamper.stamp_current_reactive_node3(
            Some(nodes[5]),
            Some(nodes[4]),
            nodes[3],
            multiplicity * (d2425_dn3),
            nodes[4],
            multiplicity * (d2425_dn4),
            nodes[5],
            multiplicity * (d2425_dn5),
        );
        let d2428_dn3: f64 = v9030;
        let d2428_dn4: f64 = v9031;
        let d2428_dn5: f64 = v9032;
        let d2428_dn6: f64 = v9033;
        let d2428_dn7: f64 = v9034;
        let d2428_dn8: f64 = v9035;
        let d2428_dn10: f64 = v9036;
        let v2428_reactive_nodes: [usize; 7] = [nodes[3], nodes[4], nodes[5], nodes[6], nodes[7], nodes[8], nodes[10]];
        let v2428_reactive_node_derivatives: [f64; 7] = [d2428_dn3, d2428_dn4, d2428_dn5, d2428_dn6, d2428_dn7, d2428_dn8, d2428_dn10];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[8]),
            &v2428_reactive_nodes,
            &v2428_reactive_node_derivatives,
            &[],
            &[],
            multiplicity,
        );
        let d2431_dn3: f64 = v9051;
        let d2431_dn4: f64 = v9052;
        let d2431_dn5: f64 = v9053;
        let d2431_dn6: f64 = v9054;
        let d2431_dn7: f64 = v9055;
        let d2431_dn8: f64 = v9056;
        let d2431_dn10: f64 = v9057;
        let v2431_reactive_nodes: [usize; 7] = [nodes[3], nodes[4], nodes[5], nodes[6], nodes[7], nodes[8], nodes[10]];
        let v2431_reactive_node_derivatives: [f64; 7] = [d2431_dn3, d2431_dn4, d2431_dn5, d2431_dn6, d2431_dn7, d2431_dn8, d2431_dn10];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[6]),
            &v2431_reactive_nodes,
            &v2431_reactive_node_derivatives,
            &[],
            &[],
            multiplicity,
        );
        let d2435_dn1: f64 = v9062;
        let d2435_dn2: f64 = v9063;
        stamper.stamp_current_reactive_node2(
            Some(nodes[1]),
            Some(nodes[2]),
            nodes[1],
            multiplicity * (d2435_dn1),
            nodes[2],
            multiplicity * (d2435_dn2),
        );
        let d2439_dn0: f64 = v9068;
        let d2439_dn1: f64 = v9069;
        stamper.stamp_current_reactive_node2(
            Some(nodes[1]),
            Some(nodes[0]),
            nodes[0],
            multiplicity * (d2439_dn0),
            nodes[1],
            multiplicity * (d2439_dn1),
        );
        let d2447_dn0: f64 = v9120;
        let d2447_dn1: f64 = v9121;
        let d2447_dn3: f64 = v9122;
        let d2447_dn4: f64 = v9123;
        let d2447_dn5: f64 = v9120;
        let d2447_dn6: f64 = v9124;
        let d2447_dn7: f64 = v9125;
        let d2447_dn8: f64 = v9126;
        let d2447_dn9: f64 = v9127;
        let d2447_dn10: f64 = v9128;
        let v2447_reactive_nodes: [usize; 10] = [nodes[0], nodes[1], nodes[3], nodes[4], nodes[5], nodes[6], nodes[7], nodes[8], nodes[9], nodes[10]];
        let v2447_reactive_node_derivatives: [f64; 10] = [d2447_dn0, d2447_dn1, d2447_dn3, d2447_dn4, d2447_dn5, d2447_dn6, d2447_dn7, d2447_dn8, d2447_dn9, d2447_dn10];
        stamper.stamp_current_reactive_dense(
            Some(nodes[1]),
            Some(nodes[9]),
            &v2447_reactive_nodes,
            &v2447_reactive_node_derivatives,
            &[],
            &[],
            multiplicity,
        );
        let d2453_dn3: f64 = v9164;
        let d2453_dn5: f64 = v9165;
        let d2453_dn6: f64 = v9166;
        let d2453_dn7: f64 = v9167;
        let d2453_dn8: f64 = v9167;
        let d2453_dn10: f64 = v9168;
        let v2453_reactive_nodes: [usize; 6] = [nodes[3], nodes[5], nodes[6], nodes[7], nodes[8], nodes[10]];
        let v2453_reactive_node_derivatives: [f64; 6] = [d2453_dn3, d2453_dn5, d2453_dn6, d2453_dn7, d2453_dn8, d2453_dn10];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[10]),
            &v2453_reactive_nodes,
            &v2453_reactive_node_derivatives,
            &[],
            &[],
            multiplicity,
        );
        let d2464_dn3: f64 = v9187;
        let d2464_dn4: f64 = v9188;
        let d2464_dn5: f64 = v9189;
        let d2464_dn6: f64 = v9190;
        let d2464_dn7: f64 = v9191;
        let d2464_dn8: f64 = v9192;
        let d2464_dn10: f64 = v9193;
        let d2464_dn11: f64 = v9194;
        let v2464_reactive_nodes: [usize; 8] = [nodes[3], nodes[4], nodes[5], nodes[6], nodes[7], nodes[8], nodes[10], nodes[11]];
        let v2464_reactive_node_derivatives: [f64; 8] = [d2464_dn3, d2464_dn4, d2464_dn5, d2464_dn6, d2464_dn7, d2464_dn8, d2464_dn10, d2464_dn11];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[4]),
            &v2464_reactive_nodes,
            &v2464_reactive_node_derivatives,
            &[],
            &[],
            multiplicity,
        );
    }
}
