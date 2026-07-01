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
        let v31: f64 = 0.001;
        let v32: f64 = 2.0;
        let v45: f64 = 0.05;
        let v47: f64 = 0.1;
        let v102: f64 = ctx.node_voltage(nodes[4]);
        let v103: bool = (v102 < v4);
        let v104: f64 = (v1 - v102);
        let v107: f64 = (if v103 { (-((v104) as f64).ln()) } else { v102 });
        let v109: bool = (v107 < self.scalar_v108);
        let v111: bool = (!v109);
        let v113: f64 = (v1 + (v107 - self.scalar_v108));
        let v117: f64 = (self.scalar_v20 + (if v111 { (self.scalar_v108 + ((v113) as f64).ln()) } else { (if v109 { v107 } else { v4 }) }));
        let v118: f64 = (v117 / self.scalar_v17);
        let v119: f64 = 8.617086918058125e-5;
        let v120: f64 = (v117 * v119);
        let v122: f64 = (v1 / v120);
        let v124: f64 = (v122 - self.scalar_v123);
        let v125: f64 = (v117 - self.scalar_v17);
        let v126: f64 = ((v118) as f64).ln();
        let v127: f64 = (self.scalar_v38 * v117);
        let v128: f64 = (v117 * v127);
        let v129: f64 = (self.scalar_v41 + v117);
        let v131: f64 = (self.scalar_v63 - (v128 / v129));
        let v133: f64 = ((v131 - v45) / v47);
        let v134: bool = (v131 < v45);
        let v135: f64 = ((v133) as f64).exp();
        let v136: f64 = (v1 + v135);
        let v141: bool = (!v134);
        let v143: f64 = (((-v133)) as f64).exp();
        let v144: f64 = (v1 + v143);
        let v148: f64 = (if v141 { (v131 + (v47 * ((v144) as f64).ln())) } else { (if v134 { (v45 + (v47 * ((v136) as f64).ln())) } else { v4 }) });
        let v149: f64 = (self.scalar_v73 * v117);
        let v150: f64 = (v117 * v149);
        let v151: f64 = (self.scalar_v76 + v117);
        let v153: f64 = (self.scalar_v96 - (v150 / v151));
        let v155: f64 = ((v153 - v45) / v47);
        let v156: bool = (v153 < v45);
        let v157: f64 = ((v155) as f64).exp();
        let v158: f64 = (v1 + v157);
        let v163: bool = (!v156);
        let v165: f64 = (((-v155)) as f64).exp();
        let v166: f64 = (v1 + v165);
        let v170: f64 = (if v163 { (v153 + (v47 * ((v166) as f64).ln())) } else { (if v156 { (v45 + (v47 * ((v158) as f64).ln())) } else { v4 }) });
        let v171: f64 = 3.0;
        let v172: f64 = -3.0;
        let v173: f64 = (v120 * v172);
        let v174: f64 = (v126 * v173);
        let v177: f64 = (v1 - v118);
        let v180: f64 = ((v174 + (self.scalar_v65 * v118)) + (v177 * self.scalar_v178));
        let v181: f64 = (v45 - v180);
        let v182: f64 = (v181 / v120);
        let v183: bool = (v45 < v180);
        let v184: f64 = ((v182) as f64).exp();
        let v185: f64 = (v1 + v184);
        let v186: f64 = ((v185) as f64).ln();
        let v190: bool = (!v183);
        let v192: f64 = (((-v182)) as f64).exp();
        let v193: f64 = (v1 + v192);
        let v194: f64 = ((v193) as f64).ln();
        let v197: f64 = (if v190 { (v45 + (v120 * v194)) } else { (if v183 { (v180 + (v120 * v186)) } else { v4 }) });
        let v202: f64 = (v177 * self.scalar_v201);
        let v203: f64 = ((v174 + (v118 * self.scalar_v198)) + v202);
        let v204: f64 = (v45 - v203);
        let v205: f64 = (v204 / v120);
        let v206: bool = (v45 < v203);
        let v207: f64 = ((v205) as f64).exp();
        let v208: f64 = (v1 + v207);
        let v209: f64 = ((v208) as f64).ln();
        let v213: bool = (!v206);
        let v215: f64 = (((-v205)) as f64).exp();
        let v216: f64 = (v1 + v215);
        let v217: f64 = ((v216) as f64).ln();
        let v220: f64 = (if v213 { (v45 + (v120 * v217)) } else { (if v206 { (v203 + (v120 * v209)) } else { v4 }) });
        let v224: f64 = (v202 + (v174 + (v118 * self.scalar_v221)));
        let v225: f64 = (v45 - v224);
        let v226: f64 = (v225 / v120);
        let v227: bool = (v45 < v224);
        let v228: f64 = ((v226) as f64).exp();
        let v229: f64 = (v1 + v228);
        let v230: f64 = ((v229) as f64).ln();
        let v234: bool = (!v227);
        let v236: f64 = (((-v226)) as f64).exp();
        let v237: f64 = (v1 + v236);
        let v238: f64 = ((v237) as f64).ln();
        let v241: f64 = (if v234 { (v45 + (v120 * v238)) } else { (if v227 { (v224 + (v120 * v230)) } else { v4 }) });
        let v244: f64 = (v202 + (v174 + (self.scalar_v67 * v118)));
        let v245: f64 = (v45 - v244);
        let v246: f64 = (v245 / v120);
        let v247: bool = (v45 < v244);
        let v248: f64 = ((v246) as f64).exp();
        let v249: f64 = (v1 + v248);
        let v250: f64 = ((v249) as f64).ln();
        let v254: bool = (!v247);
        let v256: f64 = (((-v246)) as f64).exp();
        let v257: f64 = (v1 + v256);
        let v258: f64 = ((v257) as f64).ln();
        let v261: f64 = (if v254 { (v45 + (v120 * v258)) } else { (if v247 { (v244 + (v120 * v250)) } else { v4 }) });
        let v267: f64 = ((v174 + (v118 * self.scalar_v262)) + (v177 * self.scalar_v265));
        let v268: f64 = (v45 - v267);
        let v269: f64 = (v268 / v120);
        let v270: bool = (v45 < v267);
        let v271: f64 = ((v269) as f64).exp();
        let v272: f64 = (v1 + v271);
        let v273: f64 = ((v272) as f64).ln();
        let v277: bool = (!v270);
        let v279: f64 = (((-v269)) as f64).exp();
        let v280: f64 = (v1 + v279);
        let v281: f64 = ((v280) as f64).ln();
        let v284: f64 = (if v277 { (v45 + (v120 * v281)) } else { (if v270 { (v267 + (v120 * v273)) } else { v4 }) });
        let v290: f64 = ((v174 + (v118 * self.scalar_v285)) + (v177 * self.scalar_v288));
        let v291: f64 = (v45 - v290);
        let v292: f64 = (v291 / v120);
        let v293: bool = (v45 < v290);
        let v294: f64 = ((v292) as f64).exp();
        let v295: f64 = (v1 + v294);
        let v296: f64 = ((v295) as f64).ln();
        let v300: bool = (!v293);
        let v302: f64 = (((-v292)) as f64).exp();
        let v303: f64 = (v1 + v302);
        let v304: f64 = ((v303) as f64).ln();
        let v307: f64 = (if v300 { (v45 + (v120 * v304)) } else { (if v293 { (v290 + (v120 * v296)) } else { v4 }) });
        let v308: f64 = (v1 / v197);
        let v309: f64 = (v1 / v261);
        let v310: f64 = (self.scalar_v65 * v308);
        let v311: f64 = f64::powf(v310, self.scalar_v33);
        let v312: f64 = (self.scalar_v67 * v309);
        let v313: f64 = f64::powf(v312, self.scalar_v68);
        let v315: f64 = (v311 * self.scalar_v314);
        let v317: f64 = (self.scalar_v285 / v307);
        let v320: f64 = (self.scalar_v316 * f64::powf(v317, self.scalar_v318));
        let v323: f64 = (self.scalar_v67 / v261);
        let v326: f64 = (self.scalar_v321 + (self.scalar_v322 * f64::powf(v323, self.scalar_v68)));
        let v327: f64 = (v1 / v326);
        let v329: f64 = (v326 * self.scalar_v328);
        let v330: f64 = (self.scalar_v321 * v327);
        let v334: f64 = (((v126 * self.scalar_v332)) as f64).exp();
        let v335: f64 = (self.scalar_v331 * v334);
        let v336: bool = (v335 < self.scalar_v28);
        let v337: f64 = (if v336 { self.scalar_v28 } else { v335 });
        let v343: f64 = (((v126 * self.scalar_v341)) as f64).exp();
        let v344: f64 = (self.scalar_v338 * v343);
        let v348: f64 = (((v126 * self.scalar_v346)) as f64).exp();
        let v349: f64 = (self.scalar_v345 * v348);
        let v350: bool = (v349 < self.scalar_v28);
        let v351: f64 = (if v350 { self.scalar_v28 } else { v349 });
        let v355: f64 = (((v126 * self.scalar_v353)) as f64).exp();
        let v356: f64 = (self.scalar_v352 * v355);
        let v360: f64 = (((v126 * self.scalar_v358)) as f64).exp();
        let v361: f64 = (self.scalar_v357 * v360);
        let v363: f64 = (v360 * self.scalar_v362);
        let v367: f64 = (((v126 * self.scalar_v365)) as f64).exp();
        let v368: f64 = (self.scalar_v364 * v367);
        let v375: f64 = (if self.scalar_v370 { (self.scalar_v371 * (v1 + (v125 * self.scalar_v369))) } else { v4 });
        let v378: f64 = (if self.scalar_v370 { ((v375 - v1) / v31) } else { v292 });
        let v379: bool = (v375 < v1);
        let v380: bool = (self.scalar_v370 && v379);
        let v381: f64 = ((v378) as f64).exp();
        let v382: f64 = (v1 + v381);
        let v386: f64 = (if v380 { (v1 + (v31 * ((v382) as f64).ln())) } else { v375 });
        let v388: bool = (self.scalar_v370 && (!v379));
        let v390: f64 = (((-v378)) as f64).exp();
        let v391: f64 = (v1 + v390);
        let v396: f64 = 0.0006931471805599453;
        let v400: f64 = (if self.scalar_v399 { self.scalar_v371 } else { (if self.scalar_v370 { ((if v388 { (v386 + (v31 * ((v391) as f64).ln())) } else { v386 }) - v396) } else { v4 }) });
        let v407: f64 = (if self.scalar_v402 { (self.scalar_v403 * (v1 + (v125 * self.scalar_v401))) } else { v4 });
        let v410: f64 = (if self.scalar_v402 { ((v407 - v1) / v31) } else { v378 });
        let v411: bool = (v407 < v1);
        let v412: bool = (self.scalar_v402 && v411);
        let v413: f64 = ((v410) as f64).exp();
        let v414: f64 = (v1 + v413);
        let v418: f64 = (if v412 { (v1 + (v31 * ((v414) as f64).ln())) } else { v407 });
        let v420: bool = (self.scalar_v402 && (!v411));
        let v422: f64 = (((-v410)) as f64).exp();
        let v423: f64 = (v1 + v422);
        let v431: f64 = (if self.scalar_v430 { self.scalar_v403 } else { (if self.scalar_v402 { ((if v420 { (v418 + (v31 * ((v423) as f64).ln())) } else { v418 }) - v396) } else { v4 }) });
        let v436: f64 = (self.scalar_v432 * (v1 + (v125 * self.scalar_v433)));
        let v437: f64 = 1e-6;
        let v438: f64 = (v436 * v436);
        let v439: bool = (v436 < v4);
        let v440: f64 = 0.5;
        let v441: f64 = 5e-7;
        let v443: f64 = (((v437 + v438)) as f64).sqrt();
        let v444: f64 = (v443 - v436);
        let v447: bool = (!v439);
        let v450: f64 = (if v447 { (v440 * (v436 + v443)) } else { (if v439 { (v441 / v444) } else { v4 }) });
        let v452: f64 = 4.0;
        let v457: f64 = (v126 * self.scalar_v456);
        let v459: f64 = (((v457 / v400)) as f64).exp();
        let v460: f64 = (self.scalar_v451 * v459);
        let v462: f64 = (v124 * self.scalar_v461);
        let v464: f64 = (((v462 / v400)) as f64).exp();
        let v465: f64 = (v460 * v464);
        let v469: f64 = (((v126 * self.scalar_v467)) as f64).exp();
        let v470: f64 = (self.scalar_v466 * v469);
        let v475: f64 = (((v126 * self.scalar_v473)) as f64).exp();
        let v476: f64 = (self.scalar_v471 * v475);
        let v478: f64 = 6.0;
        let v483: f64 = (((v126 * self.scalar_v481)) as f64).exp();
        let v484: f64 = (self.scalar_v477 * v483);
        let v487: f64 = (v124 * self.scalar_v486);
        let v489: f64 = (((v487 / self.scalar_v479)) as f64).exp();
        let v490: f64 = (v484 * v489);
        let v496: f64 = (((v126 * self.scalar_v494)) as f64).exp();
        let v497: f64 = (self.scalar_v491 * v496);
        let v501: f64 = ((((v124 * self.scalar_v498) / self.scalar_v492)) as f64).exp();
        let v502: f64 = (v497 * v501);
        let v506: f64 = (v126 * self.scalar_v505);
        let v509: f64 = (((v506 / self.scalar_v507)) as f64).exp();
        let v510: f64 = (self.scalar_v503 * v509);
        let v513: f64 = (v124 * self.scalar_v512);
        let v515: f64 = (((v513 / self.scalar_v507)) as f64).exp();
        let v516: f64 = (v510 * v515);
        let v520: f64 = (((v506 / self.scalar_v518)) as f64).exp();
        let v521: f64 = (self.scalar_v517 * v520);
        let v523: f64 = (((v513 / self.scalar_v518)) as f64).exp();
        let v524: f64 = (v521 * v523);
        let v532: f64 = ((((v124 * self.scalar_v529) / self.scalar_v507)) as f64).exp();
        let v539: f64 = (((v124 * self.scalar_v537)) as f64).exp();
        let v541: f64 = (if self.scalar_v526 { (self.scalar_v535 * v539) } else { v4 });
        let v547: f64 = ((((v124 * self.scalar_v544) / self.scalar_v518)) as f64).exp();
        let v554: f64 = (((v126 * self.scalar_v552)) as f64).exp();
        let v555: f64 = (self.scalar_v550 * v554);
        let v559: f64 = (((v124 * self.scalar_v557)) as f64).exp();
        let v560: f64 = (v555 * v559);
        let v566: f64 = (((v126 * self.scalar_v564)) as f64).exp();
        let v567: f64 = (self.scalar_v561 * v566);
        let v569: f64 = (((v487 / self.scalar_v562)) as f64).exp();
        let v570: f64 = (v567 * v569);
        let v575: f64 = (((v126 * self.scalar_v573)) as f64).exp();
        let v576: f64 = (self.scalar_v571 * v575);
        let v578: f64 = (((v487 / self.scalar_v572)) as f64).exp();
        let v579: f64 = (v576 * v578);
        let v581: f64 = ((v118) as f64).sqrt();
        let v582: f64 = (self.scalar_v580 * v581);
        let v585: f64 = (((v125 * self.scalar_v583)) as f64).exp();
        let v586: f64 = (v582 * v585);
        let v587: f64 = (self.scalar_v64 * v148);
        let v588: f64 = -0.5;
        let v589: f64 = f64::powf(v587, v588);
        let v590: f64 = (v1 / v311);
        let v592: f64 = (v148 * self.scalar_v591);
        let v593: f64 = (v148 * v592);
        let v594: f64 = (v589 * v593);
        let v596: f64 = (self.scalar_v65 * (v590 * v594));
        let v599: f64 = (self.scalar_v64 * (self.scalar_v64 * (v308 * v596)));
        let v601: f64 = (v589 * self.scalar_v600);
        let v602: f64 = (v197 * v601);
        let v605: f64 = (self.scalar_v66 * (self.scalar_v66 * (v197 * v602)));
        let v606: f64 = (v311 * v605);
        let v608: f64 = (((self.scalar_v591 - v599)) as f64).exp();
        let v610: f64 = (self.scalar_v97 * v170);
        let v611: f64 = f64::powf(v610, v588);
        let v612: f64 = (v1 / v313);
        let v614: f64 = (v170 * self.scalar_v613);
        let v615: f64 = (v170 * v614);
        let v616: f64 = (v611 * v615);
        let v618: f64 = (self.scalar_v67 * (v612 * v616));
        let v621: f64 = (self.scalar_v97 * (self.scalar_v97 * (v309 * v618)));
        let v623: f64 = (v611 * self.scalar_v622);
        let v624: f64 = (v261 * v623);
        let v627: f64 = (self.scalar_v98 * (self.scalar_v98 * (v261 * v624)));
        let v628: f64 = (v313 * v627);
        let v630: f64 = (((self.scalar_v613 - v621)) as f64).exp();
        let v633: f64 = (((v126 * self.scalar_v340)) as f64).exp();
        let v635: f64 = (v633 * self.scalar_v634);
        let v636: f64 = (v327 * v635);
        let v638: f64 = (v633 * self.scalar_v637);
        let v639: f64 = (v590 * v638);
        let v644: f64 = (((v126 * self.scalar_v642)) as f64).exp();
        let v645: f64 = (self.scalar_v640 * v644);
        let v648: f64 = (((v124 * self.scalar_v646)) as f64).exp();
        let v649: f64 = (v645 * v648);
        let v655: f64 = (((v126 * self.scalar_v653)) as f64).exp();
        let v656: f64 = (self.scalar_v30 * v655);
        let v657: f64 = (v648 * v656);
        let v661: f64 = (((v126 * self.scalar_v659)) as f64).exp();
        let v662: f64 = (self.scalar_v658 * v661);
        let v666: f64 = (((v126 * self.scalar_v664)) as f64).exp();
        let v667: f64 = (self.scalar_v663 * v666);
        let v671: f64 = (((v126 * self.scalar_v669)) as f64).exp();
        let v672: f64 = (self.scalar_v668 * v671);
        let v676: f64 = (((v124 * self.scalar_v674)) as f64).exp();
        let v677: f64 = (v672 * v676);
        let v682: f64 = (((v126 * self.scalar_v680)) as f64).exp();
        let v683: f64 = (self.scalar_v678 * v682);
        let v687: f64 = (((v126 * self.scalar_v685)) as f64).exp();
        let v688: f64 = (self.scalar_v684 * v687);
        let v690: f64 = (v683 + v688);
        let v693: f64 = ((self.scalar_v689 * v690) / self.scalar_v692);
        let v698: f64 = (((v126 * self.scalar_v696)) as f64).exp();
        let v699: f64 = (self.scalar_v694 * v698);
        let v701: f64 = (v117 - 300.0);
        let v703: bool = (v117 < 525.0);
        let v704: f64 = 0.00072;
        let v707: f64 = 1.6e-6;
        let v708: f64 = (v701 * v707);
        let v713: bool = (!v703);
        let v716: f64 = (if v713 { self.scalar_v715 } else { (if v703 { (self.scalar_v12 * ((v1 + (v701 * v704)) - (v701 * v708))) } else { v4 }) });
        let v718: f64 = (v633 * self.scalar_v717);
        let v726: f64 = (if self.scalar_v724 { (v1 / v356) } else { v4 });
        let v728: bool = (self.scalar_v724 && (v726 > self.scalar_v29));
        let v731: f64 = (if self.scalar_v730 { v4 } else { (if v728 { self.scalar_v29 } else { v726 }) });
        let v734: f64 = (if self.scalar_v732 { (v1 / v361) } else { v4 });
        let v736: bool = (self.scalar_v732 && (v734 > self.scalar_v29));
        let v739: f64 = (if self.scalar_v738 { v4 } else { (if v736 { self.scalar_v29 } else { v734 }) });
        let v742: f64 = (if self.scalar_v740 { (v1 / v363) } else { v4 });
        let v744: bool = (self.scalar_v740 && (v742 > self.scalar_v29));
        let v747: f64 = (if self.scalar_v746 { v4 } else { (if v744 { self.scalar_v29 } else { v742 }) });
        let v748: f64 = ctx.node_voltage(nodes[7]);
        let v749: f64 = ctx.node_voltage(nodes[8]);
        let v751: f64 = (self.scalar_v0 * (v748 - v749));
        let v752: f64 = ctx.node_voltage(nodes[9]);
        let v754: f64 = (self.scalar_v0 * (v748 - v752));
        let v755: f64 = ctx.node_voltage(nodes[5]);
        let v757: f64 = (self.scalar_v0 * (v748 - v755));
        let v758: f64 = ctx.node_voltage(nodes[6]);
        let v760: f64 = (self.scalar_v0 * (v758 - v755));
        let v762: f64 = (self.scalar_v0 * (v758 - v748));
        let v765: f64 = (self.scalar_v0 * (ctx.node_voltage(nodes[3]) - v749));
        let v767: f64 = (self.scalar_v0 * (v749 - v752));
        let v768: f64 = ctx.node_voltage(nodes[2]);
        let v770: f64 = (self.scalar_v0 * (v768 - v755));
        let v771: f64 = ctx.node_voltage(nodes[1]);
        let v773: f64 = (self.scalar_v0 * (v771 - v758));
        let v778: f64 = (self.scalar_v0 * (v771 - ctx.node_voltage(nodes[0])));
        let v779: f64 = ctx.node_voltage(nodes[11]);
        let v781: f64 = (self.scalar_v0 * (v779 - v749));
        let v784: f64 = (self.scalar_v0 * (ctx.node_voltage(nodes[10]) - v779));
        let v787: f64 = (((v754 + v762) - v767) - v781);
        let v791: f64 = ((v787 + (v773 + (-v778))) - v784);
        let v792: f64 = (v778 + v791);
        let v793: f64 = (v765 - v781);
        let v794: f64 = (v793 - v784);
        let v795: f64 = (v122 * v754);
        let v797: bool = (v795 < self.scalar_v796);
        let v798: f64 = ((v795) as f64).exp();
        let v800: bool = (!v797);
        let v802: f64 = (if v800 { self.scalar_v801 } else { v4 });
        let v806: f64 = (if v800 { (v802 * (v1 + (v795 - self.scalar_v796))) } else { (if v797 { v798 } else { v4 }) });
        let v807: f64 = (v122 * v757);
        let v808: f64 = (v807 / v400);
        let v809: bool = (v808 < self.scalar_v796);
        let v810: f64 = ((v808) as f64).exp();
        let v812: bool = (!v809);
        let v813: f64 = (if v812 { self.scalar_v801 } else { v802 });
        let v817: f64 = (if v812 { (v813 * (v1 + (v808 - self.scalar_v796))) } else { (if v809 { v810 } else { v4 }) });
        let v818: f64 = (v122 * v787);
        let v819: bool = (v818 < self.scalar_v796);
        let v820: f64 = ((v818) as f64).exp();
        let v822: bool = (!v819);
        let v823: f64 = (if v822 { self.scalar_v801 } else { v813 });
        let v827: f64 = (if v822 { (v823 * (v1 + (v818 - self.scalar_v796))) } else { (if v819 { v820 } else { v4 }) });
        let v828: f64 = (v122 * v762);
        let v829: bool = (v828 < self.scalar_v796);
        let v830: f64 = ((v828) as f64).exp();
        let v832: bool = (!v829);
        let v833: f64 = (if v832 { self.scalar_v801 } else { v823 });
        let v838: f64 = (v122 * v792);
        let v839: bool = (v838 < self.scalar_v796);
        let v840: f64 = ((v838) as f64).exp();
        let v842: bool = (!v839);
        let v843: f64 = (if v842 { self.scalar_v801 } else { v833 });
        let v847: f64 = (if v842 { (v843 * (v1 + (v838 - self.scalar_v796))) } else { (if v839 { v840 } else { v4 }) });
        let v848: f64 = (v122 * v765);
        let v849: bool = (v848 < self.scalar_v796);
        let v850: f64 = ((v848) as f64).exp();
        let v852: bool = (!v849);
        let v853: f64 = (if v852 { self.scalar_v801 } else { v843 });
        let v857: f64 = (if v852 { (v853 * (v1 + (v848 - self.scalar_v796))) } else { (if v849 { v850 } else { v4 }) });
        let v858: f64 = (v122 * v794);
        let v859: bool = (v858 < self.scalar_v796);
        let v860: f64 = ((v858) as f64).exp();
        let v862: bool = (!v859);
        let v863: f64 = (if v862 { self.scalar_v801 } else { v853 });
        let v867: f64 = (if v862 { (v863 * (v1 + (v858 - self.scalar_v796))) } else { (if v859 { v860 } else { v4 }) });
        let v868: f64 = (v122 * v793);
        let v869: bool = (v868 < self.scalar_v796);
        let v870: f64 = ((v868) as f64).exp();
        let v872: bool = (!v869);
        let v873: f64 = (if v872 { self.scalar_v801 } else { v863 });
        let v877: f64 = (if v872 { (v873 * (v1 + (v868 - self.scalar_v796))) } else { (if v869 { v870 } else { v4 }) });
        let v878: f64 = (v792 - v220);
        let v879: f64 = (v122 * v878);
        let v880: bool = (v879 < self.scalar_v796);
        let v881: f64 = ((v879) as f64).exp();
        let v883: bool = (!v880);
        let v884: f64 = (if v883 { self.scalar_v801 } else { v873 });
        let v889: f64 = (v787 - v220);
        let v890: f64 = (v122 * v889);
        let v891: bool = (v890 < self.scalar_v796);
        let v892: f64 = ((v890) as f64).exp();
        let v894: bool = (!v891);
        let v895: f64 = (if v894 { self.scalar_v801 } else { v884 });
        let v900: f64 = (v754 - v220);
        let v901: f64 = (v122 * v900);
        let v902: bool = (v901 < self.scalar_v796);
        let v903: f64 = ((v901) as f64).exp();
        let v905: bool = (!v902);
        let v906: f64 = (if v905 { self.scalar_v801 } else { v895 });
        let v910: f64 = (if v905 { (v906 * (v1 + (v901 - self.scalar_v796))) } else { (if v902 { v903 } else { v4 }) });
        let v911: f64 = (v751 - v220);
        let v912: f64 = (v122 * v911);
        let v913: bool = (v912 < self.scalar_v796);
        let v914: f64 = ((v912) as f64).exp();
        let v916: bool = (!v913);
        let v917: f64 = (if v916 { self.scalar_v801 } else { v906 });
        let v921: f64 = (if v916 { (v917 * (v1 + (v912 - self.scalar_v796))) } else { (if v913 { v914 } else { v4 }) });
        let v924: f64 = (((v1 + (v452 * v910))) as f64).sqrt();
        let v927: f64 = (((v1 + (v452 * v921))) as f64).sqrt();
        let v928: f64 = (v32 * v921);
        let v929: f64 = (v1 + v927);
        let v930: f64 = (v928 / v929);
        let v932: bool = (v930 < self.scalar_v931);
        let v933: f64 = (if v932 { self.scalar_v931 } else { v930 });
        let v935: f64 = (v1 + v924);
        let v936: f64 = (v935 / v929);
        let v938: f64 = ((v924 - v927) - ((v936) as f64).ln());
        let v939: f64 = (v120 * v938);
        let v940: f64 = (v767 + v939);
        let v941: f64 = (v940 / v368);
        let v942: bool = (v941 > v4);
        let v943: f64 = 100.0;
        let v944: bool = (v751 < v943);
        let v945: bool = (v942 && v944);
        let v948: bool = (v942 && (!v944));
        let v950: f64 = (v1 + (v751 - v943));
        let v954: f64 = (v32 * v120);
        let v955: f64 = (v440 * v941);
        let v956: f64 = (v368 * v955);
        let v958: f64 = (v1 + (v122 * v956));
        let v959: f64 = ((v958) as f64).ln();
        let v963: f64 = (if v942 { ((v220 + (v954 * v959)) - (if v948 { (v943 + ((v950) as f64).ln()) } else { (if v945 { v751 } else { v4 }) })) } else { v4 });
        let v964: f64 = 0.2;
        let v966: f64 = (if v942 { (v220 * v964) } else { v4 });
        let v968: f64 = (if v942 { (v966 * v966) } else { v437 });
        let v971: bool = (v963 < v4);
        let v972: bool = (v942 && v971);
        let v973: f64 = (v440 * v968);
        let v975: f64 = (((v968 + (if v942 { (v963 * v963) } else { v438 }))) as f64).sqrt();
        let v976: f64 = (v975 - v963);
        let v980: bool = (v942 && (!v971));
        let v983: f64 = (if v980 { (v440 * (v963 + v975)) } else { (if v972 { (v973 / v976) } else { v4 }) });
        let v987: f64 = (v983 + self.scalar_v986);
        let v988: f64 = (v983 * v987);
        let v991: f64 = (self.scalar_v985 * (v983 + (v368 * self.scalar_v984)));
        let v993: f64 = (if v942 { (v988 / v991) } else { v4 });
        let v995: f64 = (if v942 { (v941 / v993) } else { v4 });
        let v999: f64 = (if v942 { ((v995 - v1) / self.scalar_v997) } else { v410 });
        let v1000: bool = (v995 < v1);
        let v1001: bool = (v942 && v1000);
        let v1002: f64 = ((v999) as f64).exp();
        let v1003: f64 = (v1 + v1002);
        let v1009: bool = (v942 && (!v1000));
        let v1011: f64 = (((-v999)) as f64).exp();
        let v1012: f64 = (v1 + v1011);
        let v1025: f64 = (if v942 { ((if v1009 { (v995 + (self.scalar_v997 * ((v1012) as f64).ln())) } else { (if v1001 { (v1 + (self.scalar_v997 * ((v1003) as f64).ln())) } else { v4 }) }) / self.scalar_v1023) } else { v4 });
        let v1027: f64 = (if v942 { (v983 / self.scalar_v986) } else { v4 });
        let v1028: f64 = (v452 * v1025);
        let v1029: f64 = (v1027 * v1028);
        let v1030: f64 = (v1 + v1027);
        let v1033: f64 = (((v1 + (v1029 * v1030))) as f64).sqrt();
        let v1034: f64 = (v1 + v1033);
        let v1035: f64 = (v32 * v1025);
        let v1036: f64 = (v1030 * v1035);
        let v1038: f64 = (if v942 { (v1034 / v1036) } else { v4 });
        let v1040: f64 = (v933 * v1038);
        let v1041: f64 = ((v1 - v1038) + v1040);
        let v1042: f64 = (v1 + v1040);
        let v1044: f64 = (if v942 { (v1041 / v1042) } else { v4 });
        let v1045: f64 = (v956 * v1044);
        let v1047: f64 = (if v942 { (v122 * v1045) } else { v4 });
        let v1050: f64 = (v1 + (v933 + v1047));
        let v1053: f64 = (if v942 { ((v32 * v1047) + (v933 * v1050)) } else { v4 });
        let v1056: f64 = (if v942 { (v440 * (v1047 - v1)) } else { v4 });
        let v1059: f64 = (if v942 { (v1053 + (v1056 * v1056)) } else { v4 });
        let v1060: bool = (v1047 >= v1);
        let v1061: bool = (v942 && v1060);
        let v1062: f64 = ((v1059) as f64).sqrt();
        let v1066: bool = (v942 && (!v1060));
        let v1067: f64 = (v1062 - v1056);
        let v1069: f64 = (if v1066 { (v1053 / v1067) } else { (if v1061 { (v1056 + v1062) } else { v4 }) });
        let v1072: bool = (v942 && (v1069 < self.scalar_v1070));
        let v1073: f64 = (if v1072 { self.scalar_v1070 } else { v1069 });
        let v1074: f64 = (v1 + v1073);
        let v1075: f64 = (v1073 * v1074);
        let v1077: f64 = (((v122 * v220)) as f64).exp();
        let v1083: f64 = (if v942 { (self.scalar_v1080 * (v941 - self.scalar_v984)) } else { v4 });
        let v1085: f64 = (self.scalar_v984 * (v368 * self.scalar_v985));
        let v1090: f64 = ((((if v942 { (v941 * v1085) } else { v4 }) + (v1083 * v1083))) as f64).sqrt();
        let v1095: bool = (v942 && self.scalar_v1094);
        let v1096: f64 = (v47 * v261);
        let v1099: bool = (v942 && self.scalar_v1098);
        let v1100: f64 = (v32 * v941);
        let v1101: f64 = (v941 + v993);
        let v1103: f64 = (v47 + (v1100 / v1101));
        let v1106: f64 = (v941 * self.scalar_v984);
        let v1107: f64 = (v941 + self.scalar_v984);
        let v1112: bool = (!v942);
        let v1113: f64 = (v32 * v910);
        let v1116: f64 = (if v1112 { v806 } else { (if v942 { (v1075 * v1077) } else { v4 }) });
        let v1127: bool = ((((v767) as f64).abs() < (v120 * 1e-5)) || (((v939) as f64).abs() < ((v120 * 1e-40) * (v924 + v927))));
        let v1128: bool = (v1112 && v1127);
        let v1129: f64 = (v933 + (if v1112 { (v1113 / v935) } else { v1073 }));
        let v1131: f64 = (if v1128 { (v440 * v1129) } else { v4 });
        let v1132: f64 = (v1 + v1131);
        let v1136: bool = (v1112 && (!v1127));
        let v1138: f64 = ((v754 + v939) - v751);
        let v1140: f64 = (if v1136 { (v939 / v1138) } else { (if v1128 { (v1131 / v1132) } else { v1044 }) });
        let v1142: f64 = (if v1112 { v1096 } else { (if v1099 { (v261 * v1103) } else { (if v1095 { v1096 } else { v4 }) }) });
        let v1143: f64 = (if v1112 { v941 } else { (if v942 { (v1106 / v1107) } else { v4 }) });
        let v1146: f64 = (if v1112 { (v1 - (v1143 / self.scalar_v984)) } else { (if v942 { (self.scalar_v984 / v1107) } else { v4 }) });
        let v1150: f64 = (v197 * self.scalar_v1149);
        let v1151: f64 = (v47 * v197);
        let v1152: f64 = (v757 - v1150);
        let v1153: f64 = (v1152 / v1151);
        let v1154: bool = (v757 < v1150);
        let v1155: f64 = ((v1153) as f64).exp();
        let v1156: f64 = (v1 + v1155);
        let v1157: f64 = ((v1156) as f64).ln();
        let v1161: bool = (!v1154);
        let v1163: f64 = (((-v1153)) as f64).exp();
        let v1164: f64 = (v1 + v1163);
        let v1165: f64 = ((v1164) as f64).ln();
        let v1168: f64 = (if v1161 { (v1150 - (v1151 * v1165)) } else { (if v1154 { (v757 - (v1151 * v1157)) } else { v4 }) });
        let v1170: f64 = (v1 - (v308 * v1168));
        let v1172: f64 = f64::powf(v1170, self.scalar_v1171);
        let v1173: f64 = (v197 / self.scalar_v1171);
        let v1174: f64 = (v1 - v1172);
        let v1178: f64 = ((v1173 * v1174) + (v171 * (v757 - v1168)));
        let v1189: f64 = (if self.scalar_v1188 { v754 } else { (if self.scalar_v1184 { (v751 + (if v1112 { v767 } else { (if v942 { (v1083 + v1090) } else { v4 }) })) } else { (if self.scalar_v1180 { v751 } else { v4 }) }) });
        let v1190: f64 = (v32 - v330);
        let v1191: f64 = (v1 - v330);
        let v1192: f64 = (v1190 / v1191);
        let v1195: f64 = (v1 - f64::powf(v1192, self.scalar_v1193));
        let v1196: f64 = (v261 * v1195);
        let v1197: f64 = (v1189 - v1196);
        let v1198: f64 = (v1197 / v1142);
        let v1199: bool = (v1189 < v1196);
        let v1200: f64 = ((v1198) as f64).exp();
        let v1201: f64 = (v1 + v1200);
        let v1202: f64 = ((v1201) as f64).ln();
        let v1206: bool = (!v1199);
        let v1208: f64 = (((-v1198)) as f64).exp();
        let v1209: f64 = (v1 + v1208);
        let v1210: f64 = ((v1209) as f64).ln();
        let v1213: f64 = (if v1206 { (v1196 - (v1142 * v1210)) } else { (if v1199 { (v1189 - (v1142 * v1202)) } else { v4 }) });
        let v1215: f64 = f64::powf(v1146, self.scalar_v1214);
        let v1217: f64 = (v261 / self.scalar_v1216);
        let v1219: f64 = (v1 - (v1213 / v261));
        let v1220: f64 = f64::powf(v1219, self.scalar_v1216);
        let v1222: f64 = (v1 - (v1215 * v1220));
        let v1224: f64 = (v1192 * v1215);
        let v1225: f64 = (v1189 - v1213);
        let v1227: f64 = ((v1217 * v1222) + (v1224 * v1225));
        let v1230: f64 = ((v1191 * v1227) + (v330 * v751));
        let v1231: f64 = (v452 * v465);
        let v1232: f64 = (v1231 / v470);
        let v1233: f64 = (v817 * v1232);
        let v1235: f64 = (((v1 + v1233)) as f64).sqrt();
        let v1236: f64 = (v1 + v1235);
        let v1237: f64 = (v1233 / v1236);
        let v1238: f64 = (v1 / v431);
        let v1239: f64 = f64::powf(v1116, v1238);
        let v1240: f64 = (v1232 * v1239);
        let v1242: f64 = (((v1 + v1240)) as f64).sqrt();
        let v1243: f64 = (v1 + v1242);
        let v1244: f64 = (v1240 / v1243);
        let v1247: f64 = (v1 + (v1178 / v639));
        let v1248: f64 = (v1230 / v636);
        let v1249: f64 = (v1247 + v1248);
        let v1252: f64 = (v718 * v1247);
        let v1255: f64 = (-v1230);
        let v1256: f64 = (v1255 / v636);
        let v1257: f64 = (v718 * v1256);
        let v1260: f64 = (((if self.scalar_v1251 { (v122 * v1252) } else { v4 })) as f64).exp();
        let v1261: f64 = (((if self.scalar_v1251 { (v122 * v1257) } else { v4 })) as f64).exp();
        let v1262: f64 = (v1260 - v1261);
        let v1264: f64 = (((v122 * v718)) as f64).exp();
        let v1265: f64 = (v1264 - v1);
        let v1267: f64 = (if self.scalar_v1251 { (v1262 / v1265) } else { (if self.scalar_v1245 { v1249 } else { v4 }) });
        let v1268: f64 = 0.010000000000000002;
        let v1269: f64 = (v1267 * v1267);
        let v1270: bool = (v1267 < v4);
        let v1271: f64 = 0.005000000000000001;
        let v1273: f64 = (((v1268 + v1269)) as f64).sqrt();
        let v1274: f64 = (v1273 - v1267);
        let v1277: bool = (!v1270);
        let v1280: f64 = (if v1277 { (v440 * (v1267 + v1273)) } else { (if v1270 { (v1271 / v1274) } else { v4 }) });
        let v1283: f64 = (v1 + (v440 * (v1237 + v1244)));
        let v1284: f64 = (v1280 * v1283);
        let v1286: f64 = (v465 * self.scalar_v1285);
        let v1287: f64 = (v1239 * v1286);
        let v1288: f64 = (v465 * v817);
        let v1289: f64 = (v1288 - v1287);
        let v1290: f64 = (v1289 / v1284);
        let v1291: f64 = 0.0001;
        let v1292: f64 = (v757 / v1291);
        let v1293: bool = (v757 < v4);
        let v1294: f64 = ((v1292) as f64).exp();
        let v1295: f64 = (v1 + v1294);
        let v1299: bool = (!v1293);
        let v1301: f64 = (((-v1292)) as f64).exp();
        let v1302: f64 = (v1 + v1301);
        let v1306: f64 = (if v1299 { (v757 + (v1291 * ((v1302) as f64).ln())) } else { (if v1293 { (v1291 * ((v1295) as f64).ln()) } else { v4 }) });
        let v1308: f64 = (v1306 / self.scalar_v1307);
        let v1309: bool = (v1308 < self.scalar_v796);
        let v1310: f64 = ((v1308) as f64).exp();
        let v1312: bool = (!v1309);
        let v1313: f64 = (if v1312 { self.scalar_v801 } else { v917 });
        let v1317: f64 = (if v1312 { (v1313 * (v1 + (v1308 - self.scalar_v796))) } else { (if v1309 { v1310 } else { v4 }) });
        let v1318: f64 = (v1317 - v1);
        let v1322: f64 = ((v757 - self.scalar_v1320) / v31);
        let v1323: bool = (v757 < self.scalar_v1320);
        let v1324: f64 = ((v1322) as f64).exp();
        let v1325: f64 = (v1 + v1324);
        let v1330: bool = (!v1323);
        let v1332: f64 = (((-v1322)) as f64).exp();
        let v1333: f64 = (v1 + v1332);
        let v1337: f64 = (if v1330 { (self.scalar_v1320 - (v31 * ((v1333) as f64).ln())) } else { (if v1323 { (v757 - (v31 * ((v1325) as f64).ln())) } else { v4 }) });
        let v1339: f64 = (v1337 * self.scalar_v1338);
        let v1340: f64 = (self.scalar_v1320 - v1337);
        let v1341: f64 = f64::powf(v1340, v32);
        let v1343: f64 = (v807 / self.scalar_v507);
        let v1344: bool = (v1343 < self.scalar_v796);
        let v1345: f64 = ((v1343) as f64).exp();
        let v1347: bool = (!v1344);
        let v1348: f64 = (if v1347 { self.scalar_v801 } else { v1313 });
        let v1352: f64 = (if v1347 { (v1348 * (v1 + (v1343 - self.scalar_v796))) } else { (if v1344 { v1345 } else { v1306 }) });
        let v1353: f64 = (v757 - v284);
        let v1354: f64 = (v122 * v1353);
        let v1355: bool = (v1354 < self.scalar_v796);
        let v1356: bool = (self.scalar_v526 && v1355);
        let v1357: f64 = ((v1354) as f64).exp();
        let v1360: bool = (self.scalar_v526 && (!v1355));
        let v1361: f64 = (if v1360 { self.scalar_v801 } else { v1348 });
        let v1365: f64 = (if v1360 { (v1361 * (v1 + (v1354 - self.scalar_v796))) } else { (if v1356 { v1357 } else { v1308 }) });
        let v1368: f64 = ((v1290 / v465) - 1000.0);
        let v1369: f64 = 40.0;
        let v1370: bool = (v1368 < v1369);
        let v1371: bool = (self.scalar_v526 && v1370);
        let v1372: f64 = ((v1368) as f64).exp();
        let v1375: bool = (self.scalar_v526 && (!v1370));
        let v1377: f64 = (if v1375 { 2.3538526683702e17 } else { v1361 });
        let v1381: f64 = (if v1375 { (v1377 * (v1 + (v1368 - v1369))) } else { (if v1371 { v1372 } else { v1317 }) });
        let v1382: f64 = (v1352 - v1);
        let v1383: f64 = (v516 * v1382);
        let v1384: f64 = (v32 * (if self.scalar_v526 { (self.scalar_v527 * v532) } else { v4 }));
        let v1385: f64 = (v1382 * v1384);
        let v1388: f64 = (((v1 + (v452 * v1365))) as f64).sqrt();
        let v1389: f64 = (v1 + v1388);
        let v1390: f64 = (v1385 / v1389);
        let v1391: f64 = (v1 + v1248);
        let v1394: f64 = (v1116 - v1);
        let v1395: f64 = (v541 * v1394);
        let v1396: f64 = (v1381 * v1395);
        let v1397: f64 = (v1 + v1381);
        let v1412: f64 = (self.scalar_v1401 * ((v1116 + v1352) - v32));
        let v1414: f64 = ((v1382 * self.scalar_v1408) + (v1391 * v1412));
        let v1417: f64 = (v122 * v760);
        let v1418: f64 = (v1417 / self.scalar_v518);
        let v1419: bool = (v1418 < self.scalar_v796);
        let v1420: f64 = ((v1418) as f64).exp();
        let v1422: bool = (!v1419);
        let v1423: f64 = (if v1422 { self.scalar_v801 } else { v1377 });
        let v1427: f64 = (if v1422 { (v1423 * (v1 + (v1418 - self.scalar_v796))) } else { (if v1419 { v1420 } else { v1352 }) });
        let v1428: f64 = (v760 - v284);
        let v1429: f64 = (v122 * v1428);
        let v1430: bool = (v1429 < self.scalar_v796);
        let v1431: bool = (self.scalar_v526 && v1430);
        let v1432: f64 = ((v1429) as f64).exp();
        let v1435: bool = (self.scalar_v526 && (!v1430));
        let v1436: f64 = (if v1435 { self.scalar_v801 } else { v1423 });
        let v1441: f64 = (v1427 - v1);
        let v1442: f64 = (v524 * v1441);
        let v1443: f64 = (v32 * (if self.scalar_v526 { (self.scalar_v542 * v547) } else { v4 }));
        let v1444: f64 = (v1441 * v1443);
        let v1447: f64 = (((v1 + (v452 * (if v1435 { (v1436 * (v1 + (v1429 - self.scalar_v796))) } else { (if v1431 { v1432 } else { v1365 }) })))) as f64).sqrt();
        let v1448: f64 = (v1 + v1447);
        let v1453: f64 = (v807 / self.scalar_v479);
        let v1454: bool = (v1453 < self.scalar_v796);
        let v1455: f64 = ((v1453) as f64).exp();
        let v1457: bool = (!v1454);
        let v1458: f64 = (if v1457 { self.scalar_v801 } else { v1436 });
        let v1462: f64 = (if v1457 { (v1458 * (v1 + (v1453 - self.scalar_v796))) } else { (if v1454 { v1455 } else { v1427 }) });
        let v1463: f64 = (v1462 - v1);
        let v1465: f64 = (v1417 / self.scalar_v562);
        let v1466: bool = (v1465 < self.scalar_v796);
        let v1467: f64 = ((v1465) as f64).exp();
        let v1469: bool = (!v1466);
        let v1470: f64 = (if v1469 { self.scalar_v801 } else { v1458 });
        let v1474: f64 = (if v1469 { (v1470 * (v1 + (v1465 - self.scalar_v796))) } else { (if v1466 { v1467 } else { v1462 }) });
        let v1475: f64 = (v1474 - v1);
        let v1477: f64 = (v818 / self.scalar_v492);
        let v1478: bool = (v1477 < self.scalar_v796);
        let v1479: f64 = ((v1477) as f64).exp();
        let v1481: bool = (!v1478);
        let v1482: f64 = (if v1481 { self.scalar_v801 } else { v1470 });
        let v1486: f64 = (if v1481 { (v1482 * (v1 + (v1477 - self.scalar_v796))) } else { (if v1478 { v1479 } else { v1474 }) });
        let v1487: f64 = (v1486 - v1);
        let v1488: f64 = (v502 * v1487);
        let v1489: f64 = (v1417 / self.scalar_v572);
        let v1490: bool = (v1489 < self.scalar_v796);
        let v1491: f64 = ((v1489) as f64).exp();
        let v1493: bool = (!v1490);
        let v1494: f64 = (if v1493 { self.scalar_v801 } else { v1482 });
        let v1498: f64 = (if v1493 { (v1494 * (v1 + (v1489 - self.scalar_v796))) } else { (if v1490 { v1491 } else { v1486 }) });
        let v1499: f64 = (v1498 - v1);
        let v1504: bool = (v1293 && self.scalar_v1503);
        let v1505: f64 = (v32 * v1172);
        let v1507: f64 = (v1 - (self.scalar_v35 / v1505));
        let v1508: f64 = (v599 * v1507);
        let v1509: bool = (v1508 < self.scalar_v796);
        let v1510: bool = (v1504 && v1509);
        let v1511: f64 = ((v1508) as f64).exp();
        let v1514: bool = (v1504 && (!v1509));
        let v1515: f64 = (if v1514 { self.scalar_v801 } else { v1494 });
        let v1519: f64 = (if v1514 { (v1515 * (v1 + (v1508 - self.scalar_v796))) } else { (if v1510 { v1511 } else { v4 }) });
        let v1521: f64 = (if v1504 { (v308 * v757) } else { v633 });
        let v1523: f64 = 1e-30;
        let v1525: f64 = ((((v1521 * v1521) + v1523)) as f64).sqrt();
        let v1528: f64 = f64::powf(v1525, self.scalar_v1527);
        let v1536: f64 = (v478 * v1521);
        let v1537: f64 = (v1521 * v1536);
        let v1538: f64 = (v1521 + self.scalar_v1532);
        let v1540: f64 = ((self.scalar_v33 * (self.scalar_v1530 - ((v171 * v1521) * self.scalar_v1532))) - (v1537 * v1538));
        let v1542: f64 = 0.16666666666666666;
        let v1544: f64 = (if v1504 { ((v1528 * v1540) * v1542) } else { v4 });
        let v1545: f64 = (self.scalar_v35 * v757);
        let v1546: f64 = (v599 * v1545);
        let v1547: f64 = (v148 * v1544);
        let v1549: f64 = (if v1504 { (v1546 / v1547) } else { v1521 });
        let v1550: f64 = -0.001;
        let v1551: bool = (v1549 < v1550);
        let v1552: bool = (v1549 < self.scalar_v796);
        let v1553: bool = (v1504 && v1551);
        let v1554: bool = (v1552 && v1553);
        let v1555: f64 = ((v1549) as f64).exp();
        let v1558: bool = (v1553 && (!v1552));
        let v1559: f64 = (if v1558 { self.scalar_v801 } else { v1515 });
        let v1564: f64 = (-v757);
        let v1565: f64 = (v1 - (if v1558 { (v1559 * (v1 + (v1549 - self.scalar_v796))) } else { (if v1554 { v1555 } else { v4 }) }));
        let v1567: f64 = (v1 + (v1565 / v1549));
        let v1571: bool = (v1504 && (!v1551));
        let v1572: f64 = (v440 * v757);
        let v1573: f64 = (v1549 * v1572);
        let v1574: f64 = 0.3333333333333333;
        let v1575: f64 = (v1549 * v1574);
        let v1576: f64 = 0.25;
        let v1578: f64 = (v1 + (v1549 * v1576));
        let v1580: f64 = (v1 + (v1575 * v1578));
        let v1582: f64 = (if v1571 { (v1573 * v1580) } else { (if v1553 { (v1564 * v1567) } else { v4 }) });
        let v1583: f64 = (v32 * (v606 * v608));
        let v1584: f64 = (v1582 * v1583);
        let v1585: f64 = (v1172 * v1584);
        let v1586: f64 = (v1519 * v1585);
        let v1590: bool = (!v1504);
        let v1596: bool = (self.scalar_v1594 && (v751 < v4));
        let v1597: f64 = (v309 * v751);
        let v1598: f64 = (v1 - v1597);
        let v1600: f64 = (if v1596 { f64::powf(v1598, self.scalar_v1216) } else { v4 });
        let v1601: f64 = (v32 * v1600);
        let v1603: f64 = (v1 - (self.scalar_v70 / v1601));
        let v1604: f64 = (v621 * v1603);
        let v1605: bool = (v1604 < self.scalar_v796);
        let v1606: bool = (v1596 && v1605);
        let v1607: f64 = ((v1604) as f64).exp();
        let v1610: bool = (v1596 && (!v1605));
        let v1611: f64 = (if v1610 { self.scalar_v801 } else { v1559 });
        let v1615: f64 = (if v1610 { (v1611 * (v1 + (v1604 - self.scalar_v796))) } else { (if v1606 { v1607 } else { v4 }) });
        let v1616: f64 = (if v1596 { v1597 } else { v611 });
        let v1619: f64 = (((v1523 + (v1616 * v1616))) as f64).sqrt();
        let v1621: f64 = f64::powf(v1619, self.scalar_v1620);
        let v1629: f64 = (v478 * v1616);
        let v1630: f64 = (v1616 * v1629);
        let v1631: f64 = (v1616 + self.scalar_v1625);
        let v1633: f64 = ((self.scalar_v68 * (self.scalar_v1623 - ((v171 * v1616) * self.scalar_v1625))) - (v1630 * v1631));
        let v1636: f64 = (if v1596 { (v1542 * (v1621 * v1633)) } else { v4 });
        let v1637: f64 = (self.scalar_v70 * v751);
        let v1638: f64 = (v621 * v1637);
        let v1639: f64 = (v170 * v1636);
        let v1641: f64 = (if v1596 { (v1638 / v1639) } else { v1616 });
        let v1642: bool = (v1641 < v1550);
        let v1643: bool = (v1641 < self.scalar_v796);
        let v1644: bool = (v1596 && v1642);
        let v1645: bool = (v1643 && v1644);
        let v1646: f64 = ((v1641) as f64).exp();
        let v1649: bool = (v1644 && (!v1643));
        let v1650: f64 = (if v1649 { self.scalar_v801 } else { v1611 });
        let v1655: f64 = (-v751);
        let v1656: f64 = (v1 - (if v1649 { (v1650 * (v1 + (v1641 - self.scalar_v796))) } else { (if v1645 { v1646 } else { v4 }) }));
        let v1658: f64 = (v1 + (v1656 / v1641));
        let v1662: bool = (v1596 && (!v1642));
        let v1663: f64 = (v440 * v751);
        let v1664: f64 = (v1641 * v1663);
        let v1665: f64 = (v1574 * v1641);
        let v1667: f64 = (v1 + (v1576 * v1641));
        let v1669: f64 = (v1 + (v1665 * v1667));
        let v1671: f64 = (if v1662 { (v1664 * v1669) } else { (if v1644 { (v1655 * v1658) } else { v4 }) });
        let v1672: f64 = (v32 * (v628 * v630));
        let v1673: f64 = (v1671 * v1672);
        let v1674: f64 = (v1600 * v1673);
        let v1675: f64 = (v1615 * v1674);
        let v1679: bool = (!v1596);
        let v1680: f64 = (if v1679 { v4 } else { (if v1596 { (self.scalar_v71 * (v309 * v1675)) } else { v4 }) });
        let v1681: f64 = (v827 * v1232);
        let v1682: f64 = (v452 * (if v894 { (v895 * (v1 + (v890 - self.scalar_v796))) } else { (if v891 { v892 } else { v4 }) }));
        let v1683: f64 = (v1681 - v1232);
        let v1685: f64 = (((v1 + v1681)) as f64).sqrt();
        let v1686: f64 = (v1 + v1685);
        let v1687: f64 = (v1683 / v1686);
        let v1689: f64 = (((v1 + v1682)) as f64).sqrt();
        let v1690: f64 = (v1 + v1689);
        let v1691: f64 = (v1682 / v1690);
        let v1692: f64 = (v32 * v560);
        let v1693: f64 = (v827 - v1);
        let v1694: f64 = (v1692 * v1693);
        let v1695: f64 = (v452 * v560);
        let v1696: f64 = (v1695 / v476);
        let v1699: f64 = (((v1 + (v827 * v1696))) as f64).sqrt();
        let v1700: f64 = (v1 + v1699);
        let v1701: f64 = (v1694 / v1700);
        let v1706: f64 = (v649 * self.scalar_v1705);
        let v1707: f64 = (v806 - v857);
        let v1708: f64 = (v1706 * v1707);
        let v1710: f64 = (v452 * (v649 / v662));
        let v1713: f64 = (v806 + (v857 * self.scalar_v1711));
        let v1716: f64 = (((v1 + (v1710 * v1713))) as f64).sqrt();
        let v1717: f64 = (v1 + v1716);
        let v1722: f64 = (v649 * self.scalar_v1721);
        let v1723: f64 = (v827 - v877);
        let v1724: f64 = (v1722 * v1723);
        let v1726: f64 = (v827 + (v877 * self.scalar_v1711));
        let v1729: f64 = (((v1 + (v1710 * v1726))) as f64).sqrt();
        let v1730: f64 = (v1 + v1729);
        let v1734: f64 = (v806 - v1);
        let v1735: f64 = (v1706 * v1734);
        let v1738: f64 = (((v1 + (v806 * v1710))) as f64).sqrt();
        let v1739: f64 = (v1 + v1738);
        let v1741: f64 = (if self.scalar_v1733 { (v1735 / v1739) } else { (if self.scalar_v1703 { (v1708 / v1717) } else { v4 }) });
        let v1742: f64 = (v1693 * v1722);
        let v1745: f64 = (((v1 + (v827 * v1710))) as f64).sqrt();
        let v1746: f64 = (v1 + v1745);
        let v1748: f64 = (if self.scalar_v1733 { (v1742 / v1746) } else { (if self.scalar_v1703 { (v1724 / v1730) } else { v4 }) });
        let v1749: f64 = (v32 * v657);
        let v1750: f64 = (v857 - v1);
        let v1751: f64 = (v1749 * v1750);
        let v1754: f64 = (self.scalar_v1752 * (v657 / v667));
        let v1757: f64 = (((v1 + (v857 * v1754))) as f64).sqrt();
        let v1758: f64 = (v1 + v1757);
        let v1761: f64 = ((v1751 / v1758) + (v4 * v765));
        let v1767: f64 = (if self.scalar_v1765 { (self.scalar_v14 * v1701) } else { v1701 });
        let v1769: f64 = (if self.scalar_v1765 { (self.scalar_v14 * v1748) } else { v1748 });
        let v1771: f64 = (v560 * self.scalar_v1770);
        let v1772: f64 = (v847 - v1);
        let v1773: f64 = (v1771 * v1772);
        let v1776: f64 = (((v1 + (v847 * v1696))) as f64).sqrt();
        let v1777: f64 = (v1 + v1776);
        let v1779: f64 = (if self.scalar_v1765 { (v1773 / v1777) } else { v4 });
        let v1783: f64 = (v649 * self.scalar_v1782);
        let v1784: f64 = (v847 - v867);
        let v1785: f64 = (v1783 * v1784);
        let v1786: f64 = (v452 * v649);
        let v1787: f64 = (v1786 / v662);
        let v1789: f64 = (v847 + (v867 * self.scalar_v1711));
        let v1792: f64 = (((v1 + (v1787 * v1789))) as f64).sqrt();
        let v1793: f64 = (v1 + v1792);
        let v1797: f64 = (v1772 * v1783);
        let v1800: f64 = (((v1 + (v847 * v1787))) as f64).sqrt();
        let v1801: f64 = (v1 + v1800);
        let v1803: f64 = (if self.scalar_v1796 { (v1797 / v1801) } else { (if self.scalar_v1780 { (v1785 / v1793) } else { v4 }) });
        let v1807: f64 = (self.scalar_v13 * (v560 + v649));
        let v1809: f64 = (if self.scalar_v1805 { (v356 * v1807) } else { v4 });
        let v1810: f64 = (v122 * v1809);
        let v1812: f64 = (v32 - ((v1810) as f64).ln());
        let v1816: f64 = (if self.scalar_v1805 { (v792 - (if self.scalar_v1805 { (v120 * v1812) } else { v4 })) } else { v4 });
        let v1820: f64 = (if self.scalar_v1805 { (v1816 * v1816) } else { v1269 });
        let v1821: bool = (v1816 < v4);
        let v1822: bool = (self.scalar_v1805 && v1821);
        let v1825: f64 = (((self.scalar_v1818 + v1820)) as f64).sqrt();
        let v1826: f64 = (v1825 - v1816);
        let v1830: bool = (self.scalar_v1805 && (!v1821));
        let v1833: f64 = (if v1830 { (v440 * (v1816 + v1825)) } else { (if v1822 { (self.scalar_v1823 / v1826) } else { v4 }) });
        let v1834: f64 = (v1779 + v1803);
        let v1837: f64 = (v1833 + (v1809 + (v356 * v1834)));
        let v1842: f64 = (if self.scalar_v1841 { v1 } else { (if self.scalar_v1805 { (v1833 / v1837) } else { v1 }) });
        let v1844: f64 = (if self.scalar_v1765 { (v1779 * v1842) } else { v4 });
        let v1846: f64 = (if self.scalar_v1765 { (v1803 * v1842) } else { v4 });
        let v1850: f64 = (if self.scalar_v1848 { (v751 + v762) } else { v4 });
        let v1852: f64 = (-v1850);
        let v1855: bool = (v1852 < v4);
        let v1856: bool = (self.scalar_v1848 && v1855);
        let v1859: f64 = (((self.scalar_v1851 + (if self.scalar_v1848 { (v1850 * v1850) } else { v1820 }))) as f64).sqrt();
        let v1860: f64 = (v1859 - v1852);
        let v1864: bool = (self.scalar_v1848 && (!v1855));
        let v1867: f64 = (if v1864 { (v440 * (v1852 + v1859)) } else { (if v1856 { (self.scalar_v1857 / v1860) } else { v4 }) });
        let v1883: bool = (v1867 < self.scalar_v1875);
        let v1884: bool = (self.scalar_v1848 && v1883);
        let v1885: f64 = (v1867 / self.scalar_v1873);
        let v1887: f64 = (v1 - f64::powf(v1885, self.scalar_v1868));
        let v1891: bool = (self.scalar_v1848 && (!v1883));
        let v1897: f64 = (if self.scalar_v1896 { v1 } else { (if v1891 { (self.scalar_v1872 + (self.scalar_v1882 * (v1867 - self.scalar_v1875))) } else { (if v1884 { (v1 / v1887) } else { v4 }) }) });
        let v1898: f64 = (v1680 * v1897);
        let v1899: f64 = (v1767 * v1897);
        let v1900: f64 = (v1488 * v1897);
        let v1901: f64 = (v1844 * v1897);
        let v1903: bool = (v1249 < v4);
        let v1905: f64 = (((v1268 + (v1249 * v1249))) as f64).sqrt();
        let v1906: f64 = (v1905 - v1249);
        let v1909: bool = (!v1903);
        let v1912: f64 = (if v1909 { (v440 * (v1249 + v1905)) } else { (if v1903 { (v1271 / v1906) } else { v4 }) });
        let v1913: f64 = (v1283 * v1912);
        let v1914: f64 = (v344 / v1913);
        let v1915: bool = (v1914 < self.scalar_v28);
        let v1917: f64 = (v171 * (if v1915 { self.scalar_v28 } else { v1914 }));
        let v1918: f64 = ((if v832 { (v833 * (v1 + (v828 - self.scalar_v796))) } else { (if v829 { v830 } else { v4 }) }) - v1);
        let v1920: f64 = (v762 + (v954 * v1918));
        let v1921: f64 = (v1920 / v1917);
        let v1922: bool = (v1290 > v4);
        let v1926: bool = (v751 < self.scalar_v1925);
        let v1929: f64 = ((-v1290) / self.scalar_v1928);
        let v1930: bool = (v1929 < self.scalar_v796);
        let v1932: bool = (v1926 && (v1922 && self.scalar_v1924));
        let v1933: bool = (v1930 && v1932);
        let v1934: f64 = ((v1929) as f64).exp();
        let v1937: bool = (v1932 && (!v1930));
        let v1938: f64 = (if v1937 { self.scalar_v801 } else { v1650 });
        let v1942: f64 = (if v1937 { (v1938 * (v1 + (v1929 - self.scalar_v796))) } else { (if v1933 { v1934 } else { v4 }) });
        let v1943: f64 = (self.scalar_v1925 - v751);
        let v1945: f64 = (if v1932 { (v1942 * v1943) } else { v4 });
        let v1946: f64 = (-v450);
        let v1948: f64 = f64::powf(v1945, self.scalar_v1947);
        let v1949: f64 = (v1946 * v1948);
        let v1950: bool = (v1949 < self.scalar_v796);
        let v1951: bool = (v1932 && v1950);
        let v1952: f64 = ((v1949) as f64).exp();
        let v1955: bool = (v1932 && (!v1950));
        let v1956: f64 = (if v1955 { self.scalar_v801 } else { v1938 });
        let v1960: f64 = (if v1955 { (v1956 * (v1 + (v1949 - self.scalar_v796))) } else { (if v1951 { v1952 } else { v4 }) });
        let v1962: f64 = (self.scalar_v1961 / v450);
        let v1963: f64 = (v1945 * v1962);
        let v1969: bool = (v1922 && self.scalar_v1968);
        let v1971: bool = ((v751 < v220) && (self.scalar_v1966 && v1969));
        let v1977: f64 = (if v1971 { self.scalar_v1976 } else { v4 });
        let v1978: f64 = (v220 - v751);
        let v1980: f64 = (if v1971 { (v1978 / v1146) } else { v1059 });
        let v1983: f64 = ((((v32 * v1980) / v1977)) as f64).sqrt();
        let v1984: f64 = (if v1971 { v1983 } else { v4 });
        let v1987: bool = (v1971 && self.scalar_v1986);
        let v1990: bool = (v1971 && self.scalar_v1989);
        let v1993: f64 = (if v1990 { (v1 - (v440 * v1140)) } else { v4 });
        let v1994: f64 = (self.scalar_v1974 * v1993);
        let v1996: f64 = (if v1990 { (v1993 * v1994) } else { (if v1987 { self.scalar_v1974 } else { v4 }) });
        let v1997: f64 = (v1984 * v1996);
        let v2001: f64 = ((((v1984 * v1984) + (v1996 * v1996))) as f64).sqrt();
        let v2003: f64 = (if v1971 { (v1997 / v2001) } else { v4 });
        let v2005: f64 = (if v1971 { (v1978 / v2003) } else { v4 });
        let v2006: f64 = (v440 * v2003);
        let v2007: f64 = (v1977 * v2006);
        let v2010: f64 = (if v1971 { (v2005 + (v1146 * v2007)) } else { v4 });
        let v2023: f64 = (self.scalar_v984 * (if v1990 { (v1 + (self.scalar_v2013 * (v1 + (v32 * v1140)))) } else { v4 }));
        let v2025: f64 = ((if v1990 { self.scalar_v2021 } else { v4 }) - (v1290 / v2023));
        let v2028: f64 = (if v1990 { (v2005 - (v2007 * v2025)) } else { v4 });
        let v2029: f64 = (v2028 - v2010);
        let v2031: f64 = (v47 * v2005);
        let v2032: f64 = (v2005 * v2031);
        let v2038: f64 = (((if v1990 { ((v2029 * v2029) + ((v1143 * v2032) / self.scalar_v984)) } else { v1980 })) as f64).sqrt();
        let v2041: f64 = (if v1990 { (v440 * ((v2010 + v2028) + v2038)) } else { (if v1987 { v2010 } else { v4 }) });
        let v2042: f64 = (v2041 - v2005);
        let v2044: f64 = (if v1971 { (v2042 / v2041) } else { v4 });
        let v2047: bool = (((v2044) as f64).abs() > 1e-7);
        let v2048: bool = (v1971 && v2047);
        let v2050: f64 = (if v2048 { (v2006 / v2044) } else { v4 });
        let v2051: f64 = (self.scalar_v10 / v716);
        let v2052: f64 = (v2041 * v2051);
        let v2053: f64 = (v2050 * v2052);
        let v2054: f64 = (-v716);
        let v2055: f64 = (v2054 / v2041);
        let v2056: f64 = ((v2055) as f64).exp();
        let v2058: f64 = (v1 + (v1996 / v2050));
        let v2060: f64 = (((v2055 * v2058)) as f64).exp();
        let v2061: f64 = (v2056 - v2060);
        let v2065: bool = (v1971 && (!v2047));
        let v2066: f64 = (self.scalar_v10 * v1996);
        let v2073: bool = (v1926 && (self.scalar_v2069 && (v1969 && self.scalar_v2070)));
        let v2074: f64 = f64::powf(v1943, self.scalar_v1947);
        let v2076: f64 = (v1290 + self.scalar_v2075);
        let v2078: f64 = (v1 - (v1290 / v2076));
        let v2080: f64 = f64::powf(v2078, self.scalar_v2079);
        let v2082: f64 = (if v2073 { (v2074 * v2080) } else { v4 });
        let v2083: bool = (self.scalar_v1986 && v2073);
        let v2085: bool = (self.scalar_v1989 && v2073);
        let v2089: f64 = (if v2085 { ((v1290 - self.scalar_v2086) / self.scalar_v2075) } else { v4 });
        let v2093: f64 = (if v2085 { ((v2089 - v1) / self.scalar_v2091) } else { v1322 });
        let v2094: bool = (v2089 < v1);
        let v2095: bool = (v2085 && v2094);
        let v2096: f64 = ((v2093) as f64).exp();
        let v2097: f64 = (v1 + v2096);
        let v2103: bool = (v2085 && (!v2094));
        let v2105: f64 = (((-v2093)) as f64).exp();
        let v2106: f64 = (v1 + v2105);
        let v2110: f64 = (if v2103 { (v2089 + (self.scalar_v2091 * ((v2106) as f64).ln())) } else { (if v2095 { (v1 + (self.scalar_v2091 * ((v2097) as f64).ln())) } else { v4 }) });
        let v2112: f64 = f64::powf(v2110, self.scalar_v2111);
        let v2114: f64 = (if v2085 { (v2082 * v2112) } else { (if v2083 { v2082 } else { v4 }) });
        let v2115: f64 = (v1946 * v2114);
        let v2116: bool = (v2115 < self.scalar_v796);
        let v2117: bool = (v2073 && v2116);
        let v2118: f64 = ((v2115) as f64).exp();
        let v2121: bool = (v2073 && (!v2116));
        let v2122: f64 = (if v2121 { self.scalar_v801 } else { v1956 });
        let v2126: f64 = (if v2121 { (v2122 * (v1 + (v2115 - self.scalar_v796))) } else { (if v2117 { v2118 } else { v1960 }) });
        let v2127: f64 = (v1943 * v1962);
        let v2129: f64 = (if v2073 { (v2126 * v2127) } else { (if v2065 { (v2056 * v2066) } else { (if v2048 { (v2053 * v2061) } else { (if v1932 { (v1960 * v1963) } else { v4 }) }) }) });
        let v2133: bool = (v1922 && (v2129 > v4));
        let v2134: bool = (self.scalar_v2132 && v2133);
        let v2135: f64 = (v351 + v1917);
        let v2136: f64 = (v1290 * v2135);
        let v2138: f64 = (v1284 / v465);
        let v2143: f64 = (if v2134 { (((v120 / v2136) + (v516 * v2138)) + (v337 / v2135)) } else { v4 });
        let v2144: bool = (self.scalar_v2069 && v2134);
        let v2147: f64 = (if v2144 { ((v2129 - v2143) / v437) } else { v2093 });
        let v2148: bool = (v2129 < v2143);
        let v2149: bool = (v2144 && v2148);
        let v2150: f64 = ((v2147) as f64).exp();
        let v2151: f64 = (v1 + v2150);
        let v2157: bool = (v2144 && (!v2148));
        let v2159: f64 = (((-v2147)) as f64).exp();
        let v2160: f64 = (v1 + v2159);
        let v2164: f64 = (if v2157 { (v2143 - (v437 * ((v2160) as f64).ln())) } else { (if v2149 { (v2129 - (v437 * ((v2151) as f64).ln())) } else { v2129 }) });
        let v2165: f64 = (v1290 * v2164);
        let v2168: bool = (v2134 && self.scalar_v2167);
        let v2169: f64 = (v2143 * v2165);
        let v2170: f64 = (v2143 + v2164);
        let v2174: bool = (v2133 && self.scalar_v2173);
        let v2175: f64 = (if v2174 { v2165 } else { (if v2168 { (v2169 / v2170) } else { (if v2144 { v2165 } else { v4 }) }) });
        let v2176: bool = (v1116 > v4);
        let v2177: f64 = ((v1116) as f64).ln();
        let v2180: bool = (!v2176);
        let v2181: f64 = (if v2180 { v754 } else { (if v2176 { (v120 * v2177) } else { v4 }) });
        let v2183: f64 = (if self.scalar_v1403 { v754 } else { (if self.scalar_v526 { v751 } else { v4 }) });
        let v2184: f64 = (v757 - v2181);
        let v2186: f64 = (v2181 - v751);
        let v2191: f64 = (v770 * v770);
        let v2194: f64 = (v791 * v791);
        let v2197: f64 = (v784 * v784);
        let v2200: f64 = (v781 * v781);
        let v2202: f64 = (((((((v1290 * v2184) + (v941 * v2186)) - (v2175 * v2181)) + (v2191 / v337)) + (v731 * v2194)) + (v739 * v2197)) + (v747 * v2200));
        let v2203: f64 = (v773 * v773);
        let v2210: f64 = (((if self.scalar_v1407 { (v516 * v1414) } else { (if self.scalar_v1404 { v1383 } else { (if self.scalar_v526 { ((v1383 + (v1390 * v1391)) + (v1396 / v1397)) } else { v4 }) }) }) + (v490 * v1463)) + (v4 * v757));
        let v2213: f64 = ((v586 * v1318) + ((v1339 * v1341) + (v2210 - (if v1590 { v4 } else { (if v1504 { (self.scalar_v36 * (v308 * v1586)) } else { v4 }) }))));
        let v2219: f64 = ((v579 * v1499) + ((if self.scalar_v1403 { v1442 } else { (if self.scalar_v526 { (v1442 + (v1444 / v1448)) } else { v4 }) }) + (v570 * v1475)));
        let v2223: f64 = (v4 * v787);
        let v2224: f64 = ((v1899 + v1900) + v2223);
        let v2226: f64 = ((((((v2202 + (v2203 / v351)) + (v762 * v1921)) + (v757 * v2213)) - (v1898 * v2183)) + (v760 * v2219)) + (v787 * v2224));
        let v2229: f64 = (v787 - v793);
        let v2232: f64 = (v751 - v765);
        let v2235: f64 = (v792 - v794);
        let v2242: f64 = (v315 * self.scalar_v2241);
        let v2244: f64 = (v760 - v1150);
        let v2245: f64 = (v2244 / v1151);
        let v2246: bool = (v760 < v1150);
        let v2247: f64 = ((v2245) as f64).exp();
        let v2248: f64 = (v1 + v2247);
        let v2249: f64 = ((v2248) as f64).ln();
        let v2253: bool = (!v2246);
        let v2255: f64 = (((-v2245)) as f64).exp();
        let v2256: f64 = (v1 + v2255);
        let v2257: f64 = ((v2256) as f64).ln();
        let v2260: f64 = (if v2253 { (v1150 - (v1151 * v2257)) } else { (if v2246 { (v760 - (v1151 * v2249)) } else { v4 }) });
        let v2261: f64 = (v315 * self.scalar_v2240);
        let v2263: f64 = (v1 - (v308 * v2260));
        let v2265: f64 = (v1 - f64::powf(v2263, self.scalar_v1171));
        let v2269: f64 = ((v1173 * v2265) + (v171 * (v760 - v2260)));
        let v2272: f64 = (v329 * self.scalar_v2271);
        let v2274: f64 = (v470 * v683);
        let v2275: f64 = (v440 * v2274);
        let v2276: f64 = (v1237 * v2275);
        let v2277: f64 = (v1912 * v2276);
        let v2278: f64 = (v1244 * v2275);
        let v2279: f64 = (v1912 * v2278);
        let v2280: f64 = (v787 - v1196);
        let v2281: f64 = (v2280 / v1096);
        let v2282: bool = (v787 < v1196);
        let v2283: f64 = ((v2281) as f64).exp();
        let v2284: f64 = (v1 + v2283);
        let v2285: f64 = ((v2284) as f64).ln();
        let v2289: bool = (!v2282);
        let v2291: f64 = (((-v2281)) as f64).exp();
        let v2292: f64 = (v1 + v2291);
        let v2293: f64 = ((v2292) as f64).ln();
        let v2296: f64 = (if v2289 { (v1196 - (v1096 * v2293)) } else { (if v2282 { (v787 - (v1096 * v2285)) } else { v4 }) });
        let v2298: f64 = (v1 - (v2296 / v261));
        let v2300: f64 = (v1 - f64::powf(v2298, self.scalar_v1216));
        let v2302: f64 = (v787 - v2296);
        let v2304: f64 = ((v1217 * v2300) + (v1192 * v2302));
        let v2307: f64 = ((v1191 * v2304) + (v330 * v787));
        let v2312: f64 = (v792 - v1196);
        let v2313: f64 = (v2312 / v1096);
        let v2314: bool = (v792 < v1196);
        let v2315: f64 = ((v2313) as f64).exp();
        let v2316: f64 = (v1 + v2315);
        let v2317: f64 = ((v2316) as f64).ln();
        let v2321: bool = (!v2314);
        let v2323: f64 = (((-v2313)) as f64).exp();
        let v2324: f64 = (v1 + v2323);
        let v2325: f64 = ((v2324) as f64).ln();
        let v2328: f64 = (if v2321 { (v1196 - (v1096 * v2325)) } else { (if v2314 { (v792 - (v1096 * v2317)) } else { v4 }) });
        let v2330: f64 = (v1 - (v2328 / v261));
        let v2332: f64 = (v1 - f64::powf(v2330, self.scalar_v1216));
        let v2334: f64 = (v792 - v2328);
        let v2336: f64 = ((v1217 * v2332) + (v1192 * v2334));
        let v2339: f64 = ((v1191 * v2336) + (v330 * v792));
        let v2343: f64 = (v47 * v307);
        let v2347: f64 = (v307 * self.scalar_v2346);
        let v2348: f64 = (v765 - v2347);
        let v2349: f64 = (v2348 / v2343);
        let v2350: bool = (v765 < v2347);
        let v2351: f64 = ((v2349) as f64).exp();
        let v2352: f64 = (v1 + v2351);
        let v2353: f64 = ((v2352) as f64).ln();
        let v2357: bool = (!v2350);
        let v2359: f64 = (((-v2349)) as f64).exp();
        let v2360: f64 = (v1 + v2359);
        let v2361: f64 = ((v2360) as f64).ln();
        let v2364: f64 = (if v2357 { (v2347 - (v2343 * v2361)) } else { (if v2350 { (v765 - (v2343 * v2353)) } else { v4 }) });
        let v2366: f64 = (v307 / self.scalar_v2365);
        let v2368: f64 = (v1 - (v2364 / v307));
        let v2370: f64 = (v1 - f64::powf(v2368, self.scalar_v2365));
        let v2374: f64 = ((v2366 * v2370) + (v32 * (v765 - v2364)));
        let v2376: f64 = (v470 * v677);
        let v2377: f64 = (v465 / v470);
        let v2380: f64 = f64::powf(v2377, self.scalar_v2379);
        let v2381: f64 = (v2376 * v2380);
        let v2382: f64 = (v120 * self.scalar_v2378);
        let v2383: f64 = (v757 / v2382);
        let v2384: bool = (v2383 < self.scalar_v796);
        let v2385: f64 = ((v2383) as f64).exp();
        let v2387: bool = (!v2384);
        let v2388: f64 = (if v2387 { self.scalar_v801 } else { v2122 });
        let v2392: f64 = (if v2387 { (v2388 * (v1 + (v2383 - self.scalar_v796))) } else { (if v2384 { v2385 } else { v1498 }) });
        let v2393: f64 = (v2381 * v2392);
        let v2394: f64 = (v452 * v688);
        let v2395: f64 = (v120 * v2394);
        let v2396: f64 = (v2395 / v368);
        let v2397: f64 = (v440 * v2396);
        let v2398: f64 = (v1140 * v2397);
        let v2399: f64 = (v32 + v1129);
        let v2403: f64 = (v440 * v693);
        let v2406: f64 = ((v1687 * v2274) + (v1691 * v2396));
        let v2407: f64 = (v2403 * v2406);
        let v2412: f64 = ((v787 - v241) / self.scalar_v2411);
        let v2413: f64 = (v122 * v2412);
        let v2414: bool = (v2413 < self.scalar_v796);
        let v2416: bool = (v2414 && self.scalar_v2415);
        let v2417: f64 = ((v2413) as f64).exp();
        let v2420: bool = (self.scalar_v2415 && (!v2414));
        let v2421: f64 = (if v2420 { self.scalar_v801 } else { v2388 });
        let v2426: f64 = (v699 * v1692);
        let v2427: f64 = (v827 * v2426);
        let v2430: f64 = (((v1 + (v452 * (if v2420 { (v2421 * (v1 + (v2413 - self.scalar_v796))) } else { (if v2416 { v2417 } else { v4 }) })))) as f64).sqrt();
        let v2431: f64 = (v1 + v2430);
        let v2433: f64 = (if self.scalar_v2415 { (v2427 / v2431) } else { (if self.scalar_v2402 { (v2407 / v690) } else { v4 }) });
        let v2441: f64 = (if self.scalar_v2439 { (v847 * v1232) } else { v4 });
        let v2442: f64 = (v2441 - v1232);
        let v2444: f64 = (((v1 + v2441)) as f64).sqrt();
        let v2445: f64 = (v1 + v2444);
        let v2447: f64 = (if self.scalar_v2439 { (v2442 / v2445) } else { v4 });
        let v2449: f64 = (if self.scalar_v2439 { (v452 * (if v883 { (v884 * (v1 + (v879 - self.scalar_v796))) } else { (if v880 { v881 } else { v4 }) })) } else { v4 });
        let v2451: f64 = (((v1 + v2449)) as f64).sqrt();
        let v2452: f64 = (v1 + v2451);
        let v2454: f64 = (if self.scalar_v2439 { (v2449 / v2452) } else { v4 });
        let v2456: f64 = (v693 * self.scalar_v2455);
        let v2459: f64 = ((v2274 * v2447) + (v2396 * v2454));
        let v2460: f64 = (v2456 * v2459);
        let v2463: f64 = (v792 - v241);
        let v2464: f64 = (v122 * v2463);
        let v2465: bool = (v2464 < self.scalar_v796);
        let v2467: bool = (v2465 && self.scalar_v2466);
        let v2468: f64 = ((v2464) as f64).exp();
        let v2471: bool = (self.scalar_v2466 && (!v2465));
        let v2472: f64 = (if v2471 { self.scalar_v801 } else { v2421 });
        let v2477: f64 = (v699 * v1771);
        let v2478: f64 = (v847 * v2477);
        let v2481: f64 = (((v1 + (v452 * (if v2471 { (v2472 * (v1 + (v2464 - self.scalar_v796))) } else { (if v2467 { v2468 } else { v4 }) })))) as f64).sqrt();
        let v2482: f64 = (v1 + v2481);
        let v2484: f64 = (if self.scalar_v2466 { (v2478 / v2482) } else { (if self.scalar_v2439 { (v2460 / v690) } else { v4 }) });
        let v2492: f64 = (if self.scalar_v2488 { (f64::powf(v1170, self.scalar_v2489) - v171) } else { v4 });
        let v2493: f64 = (if self.scalar_v2488 { v1153 } else { v4 });
        let v2494: bool = (v2493 < v4);
        let v2495: bool = (self.scalar_v2488 && v2494);
        let v2496: f64 = ((v2493) as f64).exp();
        let v2497: f64 = (v1 + v2496);
        let v2501: bool = (self.scalar_v2488 && (!v2494));
        let v2503: f64 = (((-v2493)) as f64).exp();
        let v2504: f64 = (v1 + v2503);
        let v2506: f64 = (if v2501 { (v2503 / v2504) } else { (if v2495 { (v1 / v2497) } else { v4 }) });
        let v2509: f64 = (if self.scalar_v2488 { (v171 + (v2492 * v2506)) } else { v4 });
        let v2512: f64 = (v122 * v1233);
        let v2513: f64 = (v2512 / v400);
        let v2514: f64 = (v440 / v1235);
        let v2516: f64 = (if self.scalar_v2488 { (v2513 * v2514) } else { v4 });
        let v2517: f64 = (v1912 * v2275);
        let v2522: f64 = (v762 * v964);
        let v2524: f64 = ((if self.scalar_v2488 { (v2393 / v2382) } else { v4 }) + ((if self.scalar_v2488 { (v2242 * v2509) } else { v4 }) + (if self.scalar_v2488 { (v2516 * v2517) } else { v4 })));
        let v2533: f64 = (if self.scalar_v2488 { (v2277 + (v2393 * self.scalar_v2527)) } else { v4 });
        let v2542: f64 = (if self.scalar_v2541 { v2277 } else { (if self.scalar_v2488 { (v2533 * self.scalar_v2538) } else { v4 }) });
        let v2543: f64 = (if self.scalar_v2541 { v2279 } else { (if self.scalar_v2488 { (v2279 + (v2533 * self.scalar_v2534)) } else { v4 }) });
        let v2547: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, (v102 * self.scalar_v2545));
        let v2548: f64 = (self.scalar_v27 * v2547);
        let v2565: f64 = (v1 + (v102 / self.scalar_v20));
        let v2580: f64 = (if self.scalar_v2578 { (v102 / self.scalar_v26) } else { (if self.scalar_v2570 { (self.scalar_v2573 * (f64::powf(v2565, self.scalar_v2549) - v1)) } else { (if self.scalar_v2561 { (self.scalar_v2563 * ((v2565) as f64).ln()) } else { (if self.scalar_v2553 { (self.scalar_v27 * (v102 / self.scalar_v723)) } else { v4 }) }) }) });
        let v2581: f64 = (v1287 + v1288);
        let v2582: f64 = (v2581 / v1284);
        let v2589: f64 = (if self.scalar_v2588 { v4 } else { (if self.scalar_v2584 { (((v2175 / v2582)) as f64).abs() } else { v4 }) });
        let v2590: bool = (v2582 > v4);
        let v2591: f64 = (v2542 + v2543);
        let v2594: bool = (!v2590);
        let v2595: f64 = (v683 * v1912);
        let v2597: f64 = (if v2594 { (v1284 * v2595) } else { (if v2590 { (v2591 / v2582) } else { v4 }) });
        let v2610: f64 = (if self.scalar_v2609 { v4 } else { (if self.scalar_v2604 { (v2597 * self.scalar_v2605) } else { (if self.scalar_v2599 { (self.scalar_v2534 * v2597) } else { v4 }) }) });
        let v2622: f64 = (self.scalar_v27 * (self.scalar_v0 * v941));
        let v2624: f64 = (self.scalar_v27 * (self.scalar_v0 * v1290));
        let v2625: f64 = (self.scalar_v0 * v2219);
        let v2626: f64 = (self.scalar_v27 * v2625);
        let v2627: f64 = (self.scalar_v0 * v2213);
        let v2628: f64 = (self.scalar_v27 * v2627);
        let v2631: f64 = (self.scalar_v27 * (self.scalar_v0 * (-v1898)));
        let v2632: f64 = (if self.scalar_v526 { v2631 } else { v4 });
        let v2633: f64 = (if self.scalar_v1403 { v2631 } else { v4 });
        let v2634: f64 = (self.scalar_v0 * v1769);
        let v2635: f64 = (self.scalar_v27 * v2634);
        let v2636: f64 = (self.scalar_v0 * v1741);
        let v2637: f64 = (self.scalar_v27 * v2636);
        let v2639: f64 = (self.scalar_v27 * (self.scalar_v0 * v1846));
        let v2640: f64 = (self.scalar_v0 * v1761);
        let v2641: f64 = (self.scalar_v27 * v2640);
        let v2642: f64 = (self.scalar_v0 * v1921);
        let v2643: f64 = (self.scalar_v27 * v2642);
        let v2645: f64 = (self.scalar_v27 * (self.scalar_v0 * (-v2175)));
        let v2646: f64 = (self.scalar_v0 * v770);
        let v2648: f64 = (self.scalar_v27 * (v2646 / v337));
        let v2649: f64 = (self.scalar_v0 * v773);
        let v2651: f64 = (self.scalar_v27 * (v2649 / v351));
        let v2653: f64 = (self.scalar_v27 * (-(((((v2226 + (v792 * v1901)) + (v1769 * v2229)) + (v1741 * v2232)) + (v1846 * v2235)) + (v765 * v1761))));
        let v2655: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, (self.scalar_v0 * ((if self.scalar_v2541 { v2393 } else { (if self.scalar_v2488 { (v2393 * self.scalar_v2528) } else { v4 }) }) + ((v1178 * v2242) + v2542))));
        let v2656: f64 = (self.scalar_v27 * v2655);
        let v2658: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, (self.scalar_v0 * (v2261 * v2269)));
        let v2659: f64 = (self.scalar_v27 * v2658);
        let v2661: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, (self.scalar_v0 * ((v2398 * v2399) + ((v1230 * v2272) + v2543))));
        let v2662: f64 = (self.scalar_v27 * v2661);
        let v2664: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, (self.scalar_v0 * (v320 * v2374)));
        let v2665: f64 = (self.scalar_v27 * v2664);
        let v2667: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 5, (self.scalar_v0 * (if self.scalar_v2488 { (v2522 * v2524) } else { v4 })));
        let v2668: f64 = (self.scalar_v27 * v2667);
        let v2671: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 6, ((self.scalar_v0 * (v771 - v768)) * self.scalar_v2669));
        let v2672: f64 = (self.scalar_v27 * v2671);
        let v2675: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 7, (v778 * self.scalar_v2673));
        let v2676: f64 = (self.scalar_v27 * v2675);
        let v2678: f64 = (self.scalar_v27 * (self.scalar_v0 * v1901));
        let v2679: f64 = (self.scalar_v0 * v791);
        let v2681: f64 = (self.scalar_v27 * (v731 * v2679));
        let v2683: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 8, (self.scalar_v0 * ((self.scalar_v13 * (self.scalar_v2309 * (v329 * v2339))) + (if self.scalar_v2436 { (v1842 * v2484) } else { v4 }))));
        let v2684: f64 = (self.scalar_v27 * v2683);
        let v2687: f64 = (self.scalar_v27 * (self.scalar_v0 * (v1899 + (v1900 + v2223))));
        let v2689: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 9, (self.scalar_v0 * ((self.scalar_v14 * ((v329 * v2307) * self.scalar_v2309)) + (if self.scalar_v2436 { (self.scalar_v14 * v2433) } else { v2433 }))));
        let v2690: f64 = (self.scalar_v27 * v2689);
        let v2691: f64 = (self.scalar_v0 * v784);
        let v2694: f64 = (if self.scalar_v732 { (self.scalar_v27 * (v739 * v2691)) } else { v4 });
        let v2695: f64 = (self.scalar_v0 * v781);
        let v2698: f64 = (if self.scalar_v740 { (self.scalar_v27 * (v747 * v2695)) } else { v4 });
        let v2699: f64 = ctx.node_voltage(nodes[12]);
        let v2700: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 10, v2699);
        let v2701: f64 = (v2610 * v2700);
        let v2702: f64 = (v2589 * v2699);
        let v2705: f64 = (if v103 { (-(-1.0 / v104)) } else { v1 });
        let v2708: f64 = (if v111 { (v2705 / v113) } else { (if v109 { v2705 } else { v4 }) });
        let v2709: f64 = (v2708 / self.scalar_v17);
        let v2710: f64 = (v119 * v2708);
        let v2712: f64 = (v120 * v120);
        let v2713: f64 = ((-v2710) / v2712);
        let v2714: f64 = (v2709 / v118);
        let v2724: f64 = (-(((v129 * ((v127 * v2708) + (v117 * (self.scalar_v38 * v2708)))) - (v128 * v2708)) / (v129 * v129)));
        let v2725: f64 = (v2724 / v47);
        let v2735: f64 = (if v141 { (v2724 + (v47 * ((v143 * (-v2725)) / v144))) } else { (if v134 { (v47 * ((v135 * v2725) / v136)) } else { v4 }) });
        let v2745: f64 = (-(((v151 * ((v149 * v2708) + (v117 * (self.scalar_v73 * v2708)))) - (v150 * v2708)) / (v151 * v151)));
        let v2746: f64 = (v2745 / v47);
        let v2756: f64 = (if v163 { (v2745 + (v47 * ((v165 * (-v2746)) / v166))) } else { (if v156 { (v47 * ((v157 * v2746) / v158)) } else { v4 }) });
        let v2760: f64 = ((v173 * v2714) + (v126 * (v172 * v2710)));
        let v2763: f64 = (-v2709);
        let v2765: f64 = ((v2760 + (self.scalar_v65 * v2709)) + (self.scalar_v178 * v2763));
        let v2770: f64 = (((v120 * (-v2765)) - (v181 * v2710)) / v2712);
        let v2784: f64 = (if v190 { ((v194 * v2710) + (v120 * ((v192 * (-v2770)) / v193))) } else { (if v183 { (v2765 + ((v186 * v2710) + (v120 * ((v184 * v2770) / v185)))) } else { v4 }) });
        let v2787: f64 = (self.scalar_v201 * v2763);
        let v2788: f64 = ((v2760 + (self.scalar_v198 * v2709)) + v2787);
        let v2793: f64 = (((v120 * (-v2788)) - (v204 * v2710)) / v2712);
        let v2807: f64 = (if v213 { ((v217 * v2710) + (v120 * ((v215 * (-v2793)) / v216))) } else { (if v206 { (v2788 + ((v209 * v2710) + (v120 * ((v207 * v2793) / v208)))) } else { v4 }) });
        let v2810: f64 = (v2787 + (v2760 + (self.scalar_v221 * v2709)));
        let v2815: f64 = (((v120 * (-v2810)) - (v225 * v2710)) / v2712);
        let v2829: f64 = (if v234 { ((v238 * v2710) + (v120 * ((v236 * (-v2815)) / v237))) } else { (if v227 { (v2810 + ((v230 * v2710) + (v120 * ((v228 * v2815) / v229)))) } else { v4 }) });
        let v2832: f64 = (v2787 + (v2760 + (self.scalar_v67 * v2709)));
        let v2837: f64 = (((v120 * (-v2832)) - (v245 * v2710)) / v2712);
        let v2851: f64 = (if v254 { ((v258 * v2710) + (v120 * ((v256 * (-v2837)) / v257))) } else { (if v247 { (v2832 + ((v250 * v2710) + (v120 * ((v248 * v2837) / v249)))) } else { v4 }) });
        let v2855: f64 = ((v2760 + (self.scalar_v262 * v2709)) + (self.scalar_v265 * v2763));
        let v2860: f64 = (((v120 * (-v2855)) - (v268 * v2710)) / v2712);
        let v2874: f64 = (if v277 { ((v281 * v2710) + (v120 * ((v279 * (-v2860)) / v280))) } else { (if v270 { (v2855 + ((v273 * v2710) + (v120 * ((v271 * v2860) / v272)))) } else { v4 }) });
        let v2878: f64 = ((v2760 + (self.scalar_v285 * v2709)) + (self.scalar_v288 * v2763));
        let v2883: f64 = (((v120 * (-v2878)) - (v291 * v2710)) / v2712);
        let v2897: f64 = (if v300 { ((v304 * v2710) + (v120 * ((v302 * (-v2883)) / v303))) } else { (if v293 { (v2878 + ((v296 * v2710) + (v120 * ((v294 * v2883) / v295)))) } else { v4 }) });
        let v2900: f64 = ((-v2784) / (v197 * v197));
        let v2902: f64 = (v261 * v261);
        let v2903: f64 = ((-v2851) / v2902);
        let v2907: f64 = ((self.scalar_v65 * v2900) * (self.scalar_v33 * f64::powf(v310, self.scalar_v1532)));
        let v2911: f64 = ((self.scalar_v67 * v2903) * (self.scalar_v68 * f64::powf(v312, self.scalar_v1625)));
        let v2912: f64 = (self.scalar_v314 * v2907);
        let v2915: f64 = (v307 * v307);
        let v2928: f64 = (self.scalar_v322 * (((-(self.scalar_v67 * v2851)) / v2902) * (self.scalar_v68 * f64::powf(v323, self.scalar_v1625))));
        let v2931: f64 = ((-v2928) / (v326 * v326));
        let v2932: f64 = (self.scalar_v328 * v2928);
        let v2933: f64 = (self.scalar_v321 * v2931);
        let v2937: f64 = (if v336 { v4 } else { (self.scalar_v331 * (v334 * (self.scalar_v332 * v2714))) });
        let v2944: f64 = (if v350 { v4 } else { (self.scalar_v345 * (v348 * (self.scalar_v346 * v2714))) });
        let v2947: f64 = (self.scalar_v352 * (v355 * (self.scalar_v353 * v2714)));
        let v2949: f64 = (v360 * (self.scalar_v358 * v2714));
        let v2954: f64 = (self.scalar_v364 * (v367 * (self.scalar_v365 * v2714)));
        let v2957: f64 = (if self.scalar_v370 { (self.scalar_v371 * (self.scalar_v369 * v2708)) } else { v4 });
        let v2959: f64 = (if self.scalar_v370 { (v2957 / v31) } else { v2883 });
        let v2963: f64 = (if v380 { (v31 * ((v381 * v2959) / v382)) } else { v2957 });
        let v2971: f64 = (if self.scalar_v399 { v4 } else { (if self.scalar_v370 { (if v388 { (v2963 + (v31 * ((v390 * (-v2959)) / v391))) } else { v2963 }) } else { v4 }) });
        let v2974: f64 = (if self.scalar_v402 { (self.scalar_v403 * (self.scalar_v401 * v2708)) } else { v4 });
        let v2976: f64 = (if self.scalar_v402 { (v2974 / v31) } else { v2959 });
        let v2980: f64 = (if v412 { (v31 * ((v413 * v2976) / v414)) } else { v2974 });
        let v2990: f64 = (self.scalar_v432 * (self.scalar_v433 * v2708));
        let v2991: f64 = (v436 * v2990);
        let v2992: f64 = (v2991 + v2991);
        let v2994: f64 = (v2992 / (v32 * v443));
        let v3003: f64 = (if v447 { (v440 * (v2990 + v2994)) } else { (if v439 { ((-(v441 * (v2994 - v2990))) / (v444 * v444)) } else { v4 }) });
        let v3008: f64 = (v400 * v400);
        let v3020: f64 = ((v464 * (self.scalar_v451 * (v459 * (((v400 * (self.scalar_v456 * v2714)) - (v457 * v2971)) / v3008)))) + (v460 * (v464 * (((v400 * (self.scalar_v461 * v2713)) - (v462 * v2971)) / v3008))));
        let v3023: f64 = (self.scalar_v466 * (v469 * (self.scalar_v467 * v2714)));
        let v3030: f64 = (self.scalar_v486 * v2713);
        let v3045: f64 = (self.scalar_v505 * v2714);
        let v3049: f64 = (self.scalar_v512 * v2713);
        let v3054: f64 = ((v515 * (self.scalar_v503 * (v509 * (v3045 / self.scalar_v507)))) + (v510 * (v515 * (v3049 / self.scalar_v507))));
        let v3084: f64 = ((v559 * (self.scalar_v550 * (v554 * (self.scalar_v552 * v2714)))) + (v555 * (v559 * (self.scalar_v557 * v2713))));
        let v3110: f64 = -1.5;
        let v3113: f64 = ((self.scalar_v64 * v2735) * (v588 * f64::powf(v587, v3110)));
        let v3116: f64 = ((-v2907) / (v311 * v311));
        let v3129: f64 = (v308 * (self.scalar_v65 * ((v594 * v3116) + (v590 * ((v593 * v3113) + (v589 * ((v592 * v2735) + (v148 * (self.scalar_v591 * v2735)))))))));
        let v3132: f64 = (self.scalar_v64 * (self.scalar_v64 * ((v596 * v2900) + v3129)));
        let v3144: f64 = ((v605 * v2907) + (v311 * (self.scalar_v66 * (self.scalar_v66 * ((v602 * v2784) + (v197 * ((v601 * v2784) + (v197 * (self.scalar_v600 * v3113)))))))));
        let v3153: f64 = ((self.scalar_v97 * v2756) * (v588 * f64::powf(v610, v3110)));
        let v3166: f64 = ((v616 * ((-v2911) / (v313 * v313))) + (v612 * ((v615 * v3153) + (v611 * ((v614 * v2756) + (v170 * (self.scalar_v613 * v2756)))))));
        let v3172: f64 = (self.scalar_v97 * (self.scalar_v97 * ((v618 * v2903) + (v309 * (self.scalar_v67 * v3166)))));
        let v3184: f64 = ((v627 * v2911) + (v313 * (self.scalar_v98 * (self.scalar_v98 * ((v624 * v2851) + (v261 * ((v623 * v2851) + (v261 * (self.scalar_v622 * v3153)))))))));
        let v3191: f64 = (v633 * (self.scalar_v340 * v2714));
        let v3195: f64 = ((v635 * v2931) + (v327 * (self.scalar_v634 * v3191)));
        let v3204: f64 = (v648 * (self.scalar_v646 * v2713));
        let v3207: f64 = ((v648 * (self.scalar_v640 * (v644 * (self.scalar_v642 * v2714)))) + (v645 * v3204));
        let v3213: f64 = ((v656 * v3204) + (v648 * (self.scalar_v30 * (v655 * (self.scalar_v653 * v2714)))));
        let v3216: f64 = (self.scalar_v658 * (v661 * (self.scalar_v659 * v2714)));
        let v3230: f64 = (self.scalar_v678 * (v682 * (self.scalar_v680 * v2714)));
        let v3233: f64 = (self.scalar_v684 * (v687 * (self.scalar_v685 * v2714)));
        let v3234: f64 = (v3230 + v3233);
        let v3236: f64 = ((self.scalar_v689 * v3234) / self.scalar_v692);
        let v3239: f64 = (self.scalar_v694 * (v698 * (self.scalar_v696 * v2714)));
        let v3248: f64 = (if v713 { v4 } else { (if v703 { (self.scalar_v12 * ((v704 * v2708) - ((v708 * v2708) + (v701 * (v707 * v2708))))) } else { v4 }) });
        let v3249: f64 = (self.scalar_v717 * v3191);
        let v3255: f64 = (if self.scalar_v730 { v4 } else { (if v728 { v4 } else { (if self.scalar_v724 { ((-v2947) / (v356 * v356)) } else { v4 }) }) });
        let v3261: f64 = (if self.scalar_v738 { v4 } else { (if v736 { v4 } else { (if self.scalar_v732 { ((-(self.scalar_v357 * v2949)) / (v361 * v361)) } else { v4 }) }) });
        let v3267: f64 = (if self.scalar_v746 { v4 } else { (if v744 { v4 } else { (if self.scalar_v740 { ((-(self.scalar_v362 * v2949)) / (v363 * v363)) } else { v4 }) }) });
        let v3272: f64 = (v754 * v2713);
        let v3273: f64 = (self.scalar_v0 * v122);
        let v3274: f64 = (v122 * self.scalar_v3268);
        let v3284: f64 = (if v800 { (v802 * v3272) } else { (if v797 { (v798 * v3272) } else { v4 }) });
        let v3285: f64 = (if v800 { (v802 * v3273) } else { (if v797 { (v798 * v3273) } else { v4 }) });
        let v3286: f64 = (if v800 { (v802 * v3274) } else { (if v797 { (v798 * v3274) } else { v4 }) });
        let v3287: f64 = (v757 * v2713);
        let v3291: f64 = (((v400 * v3287) - (v807 * v2971)) / v3008);
        let v3292: f64 = (v3274 / v400);
        let v3293: f64 = (v3273 / v400);
        let v3303: f64 = (if v812 { (v813 * v3291) } else { (if v809 { (v810 * v3291) } else { v4 }) });
        let v3304: f64 = (if v812 { (v813 * v3292) } else { (if v809 { (v810 * v3292) } else { v4 }) });
        let v3305: f64 = (if v812 { (v813 * v3293) } else { (if v809 { (v810 * v3293) } else { v4 }) });
        let v3306: f64 = (v787 * v2713);
        let v3307: f64 = (v122 * self.scalar_v3269);
        let v3308: f64 = (v122 * self.scalar_v3270);
        let v3324: f64 = (if v822 { (v823 * v3306) } else { (if v819 { (v820 * v3306) } else { v4 }) });
        let v3325: f64 = (if v822 { (v823 * v3273) } else { (if v819 { (v820 * v3273) } else { v4 }) });
        let v3326: f64 = (if v822 { (v823 * v3307) } else { (if v819 { (v820 * v3307) } else { v4 }) });
        let v3327: f64 = (if v822 { (v823 * v3308) } else { (if v819 { (v820 * v3308) } else { v4 }) });
        let v3328: f64 = (if v822 { (v823 * v3274) } else { (if v819 { (v820 * v3274) } else { v4 }) });
        let v3329: f64 = (v762 * v2713);
        let v3342: f64 = (v122 * self.scalar_v3271);
        let v3343: f64 = (v792 * v2713);
        let v3359: f64 = (if v842 { (v843 * v3307) } else { (if v839 { (v840 * v3307) } else { v4 }) });
        let v3360: f64 = (if v842 { (v843 * v3342) } else { (if v839 { (v840 * v3342) } else { v4 }) });
        let v3361: f64 = (if v842 { (v843 * v3343) } else { (if v839 { (v840 * v3343) } else { v4 }) });
        let v3362: f64 = (if v842 { (v843 * v3308) } else { (if v839 { (v840 * v3308) } else { v4 }) });
        let v3363: f64 = (if v842 { (v843 * v3274) } else { (if v839 { (v840 * v3274) } else { v4 }) });
        let v3364: f64 = (v765 * v2713);
        let v3374: f64 = (if v852 { (v853 * v3273) } else { (if v849 { (v850 * v3273) } else { v4 }) });
        let v3375: f64 = (if v852 { (v853 * v3364) } else { (if v849 { (v850 * v3364) } else { v4 }) });
        let v3376: f64 = (if v852 { (v853 * v3274) } else { (if v849 { (v850 * v3274) } else { v4 }) });
        let v3377: f64 = (v794 * v2713);
        let v3390: f64 = (if v862 { (v863 * v3273) } else { (if v859 { (v860 * v3273) } else { v4 }) });
        let v3391: f64 = (if v862 { (v863 * v3377) } else { (if v859 { (v860 * v3377) } else { v4 }) });
        let v3392: f64 = (if v862 { (v863 * v3308) } else { (if v859 { (v860 * v3308) } else { v4 }) });
        let v3393: f64 = (if v862 { (v863 * v3274) } else { (if v859 { (v860 * v3274) } else { v4 }) });
        let v3394: f64 = (v793 * v2713);
        let v3407: f64 = (if v872 { (v873 * v3273) } else { (if v869 { (v870 * v3273) } else { v4 }) });
        let v3408: f64 = (if v872 { (v873 * v3394) } else { (if v869 { (v870 * v3394) } else { v4 }) });
        let v3409: f64 = (if v872 { (v873 * v3308) } else { (if v869 { (v870 * v3308) } else { v4 }) });
        let v3410: f64 = (if v872 { (v873 * v3274) } else { (if v869 { (v870 * v3274) } else { v4 }) });
        let v3413: f64 = (v122 * (-v2807));
        let v3414: f64 = ((v878 * v2713) + v3413);
        let v3436: f64 = (v3413 + (v889 * v2713));
        let v3458: f64 = (v3413 + (v900 * v2713));
        let v3468: f64 = (if v905 { (v906 * v3458) } else { (if v902 { (v903 * v3458) } else { v4 }) });
        let v3469: f64 = (if v905 { (v906 * v3273) } else { (if v902 { (v903 * v3273) } else { v4 }) });
        let v3470: f64 = (if v905 { (v906 * v3274) } else { (if v902 { (v903 * v3274) } else { v4 }) });
        let v3472: f64 = (v3413 + (v911 * v2713));
        let v3482: f64 = (if v916 { (v917 * v3472) } else { (if v913 { (v914 * v3472) } else { v4 }) });
        let v3483: f64 = (if v916 { (v917 * v3273) } else { (if v913 { (v914 * v3273) } else { v4 }) });
        let v3484: f64 = (if v916 { (v917 * v3274) } else { (if v913 { (v914 * v3274) } else { v4 }) });
        let v3488: f64 = (v32 * v924);
        let v3489: f64 = ((v452 * v3468) / v3488);
        let v3490: f64 = ((v452 * v3469) / v3488);
        let v3491: f64 = ((v452 * v3470) / v3488);
        let v3495: f64 = (v32 * v927);
        let v3496: f64 = ((v452 * v3482) / v3495);
        let v3497: f64 = ((v452 * v3483) / v3495);
        let v3498: f64 = ((v452 * v3484) / v3495);
        let v3505: f64 = (v929 * v929);
        let v3515: f64 = (if v932 { v4 } else { (((v929 * (v32 * v3482)) - (v928 * v3496)) / v3505) });
        let v3516: f64 = (if v932 { v4 } else { (((v929 * (v32 * v3483)) - (v928 * v3497)) / v3505) });
        let v3517: f64 = (if v932 { v4 } else { (((v929 * (v32 * v3484)) - (v928 * v3498)) / v3505) });
        let v3543: f64 = ((v938 * v2710) + (v120 * ((v3489 - v3496) - ((((v929 * v3489) - (v935 * v3496)) / v3505) / v936))));
        let v3544: f64 = (v120 * ((v3490 - v3497) - ((((v929 * v3490) - (v935 * v3497)) / v3505) / v936)));
        let v3545: f64 = (v120 * ((-v3498) - (((-(v935 * v3498)) / v3505) / v936)));
        let v3546: f64 = (v120 * (v3491 - ((v3491 / v929) / v936)));
        let v3548: f64 = (self.scalar_v3268 + v3546);
        let v3552: f64 = (v368 * v368);
        let v3553: f64 = (((v368 * v3543) - (v940 * v2954)) / v3552);
        let v3554: f64 = (v3544 / v368);
        let v3555: f64 = ((self.scalar_v0 + v3545) / v368);
        let v3556: f64 = (v3548 / v368);
        let v3563: f64 = (v32 * v2710);
        let v3570: f64 = ((v955 * v2954) + (v368 * (v440 * v3553)));
        let v3571: f64 = (v368 * (v440 * v3554));
        let v3572: f64 = (v368 * (v440 * v3555));
        let v3573: f64 = (v368 * (v440 * v3556));
        let v3593: f64 = (if v942 { (v2807 + ((v959 * v3563) + (v954 * (((v956 * v2713) + (v122 * v3570)) / v958)))) } else { v4 });
        let v3594: f64 = (if v942 { ((v954 * ((v122 * v3571) / v958)) - (if v948 { (self.scalar_v0 / v950) } else { (if v945 { self.scalar_v0 } else { v4 }) })) } else { v4 });
        let v3595: f64 = (if v942 { ((v954 * ((v122 * v3572) / v958)) - (if v948 { (self.scalar_v3268 / v950) } else { (if v945 { self.scalar_v3268 } else { v4 }) })) } else { v4 });
        let v3596: f64 = (if v942 { (v954 * ((v122 * v3573) / v958)) } else { v4 });
        let v3599: f64 = (v966 * (if v942 { (v964 * v2807) } else { v4 }));
        let v3601: f64 = (if v942 { (v3599 + v3599) } else { v4 });
        let v3602: f64 = (v963 * v3593);
        let v3604: f64 = (v963 * v3594);
        let v3606: f64 = (v963 * v3595);
        let v3608: f64 = (v963 * v3596);
        let v3616: f64 = (v32 * v975);
        let v3617: f64 = ((v3601 + (if v942 { (v3602 + v3602) } else { v2992 })) / v3616);
        let v3618: f64 = ((if v942 { (v3604 + v3604) } else { v4 }) / v3616);
        let v3619: f64 = ((if v942 { (v3606 + v3606) } else { v4 }) / v3616);
        let v3620: f64 = ((if v942 { (v3608 + v3608) } else { v4 }) / v3616);
        let v3628: f64 = (v976 * v976);
        let v3651: f64 = (if v980 { (v440 * (v3593 + v3617)) } else { (if v972 { (((v976 * (v440 * v3601)) - (v973 * (v3617 - v3593))) / v3628) } else { v4 }) });
        let v3652: f64 = (if v980 { (v440 * (v3594 + v3618)) } else { (if v972 { ((-(v973 * (v3618 - v3594))) / v3628) } else { v4 }) });
        let v3653: f64 = (if v980 { (v440 * (v3595 + v3619)) } else { (if v972 { ((-(v973 * (v3619 - v3595))) / v3628) } else { v4 }) });
        let v3654: f64 = (if v980 { (v440 * (v3596 + v3620)) } else { (if v972 { ((-(v973 * (v3620 - v3596))) / v3628) } else { v4 }) });
        let v3676: f64 = (v991 * v991);
        let v3690: f64 = (if v942 { (((v991 * ((v987 * v3651) + (v983 * v3651))) - (v988 * (self.scalar_v985 * (v3651 + (self.scalar_v984 * v2954))))) / v3676) } else { v4 });
        let v3691: f64 = (if v942 { (((v991 * ((v987 * v3652) + (v983 * v3652))) - (v988 * (self.scalar_v985 * v3652))) / v3676) } else { v4 });
        let v3692: f64 = (if v942 { (((v991 * ((v987 * v3653) + (v983 * v3653))) - (v988 * (self.scalar_v985 * v3653))) / v3676) } else { v4 });
        let v3693: f64 = (if v942 { (((v991 * ((v987 * v3654) + (v983 * v3654))) - (v988 * (self.scalar_v985 * v3654))) / v3676) } else { v4 });
        let v3697: f64 = (v993 * v993);
        let v3711: f64 = (if v942 { (((v993 * v3553) - (v941 * v3690)) / v3697) } else { v4 });
        let v3712: f64 = (if v942 { (((v993 * v3554) - (v941 * v3691)) / v3697) } else { v4 });
        let v3713: f64 = (if v942 { (((v993 * v3555) - (v941 * v3692)) / v3697) } else { v4 });
        let v3714: f64 = (if v942 { (((v993 * v3556) - (v941 * v3693)) / v3697) } else { v4 });
        let v3719: f64 = (if v942 { (v3711 / self.scalar_v997) } else { v2976 });
        let v3720: f64 = (if v942 { (v3712 / self.scalar_v997) } else { v4 });
        let v3721: f64 = (if v942 { (v3713 / self.scalar_v997) } else { v4 });
        let v3722: f64 = (if v942 { (v3714 / self.scalar_v997) } else { v4 });
        let v3767: f64 = (if v942 { ((if v1009 { (v3711 + (self.scalar_v997 * ((v1011 * (-v3719)) / v1012))) } else { (if v1001 { (self.scalar_v997 * ((v1002 * v3719) / v1003)) } else { v4 }) }) / self.scalar_v1023) } else { v4 });
        let v3768: f64 = (if v942 { ((if v1009 { (v3712 + (self.scalar_v997 * ((v1011 * (-v3720)) / v1012))) } else { (if v1001 { (self.scalar_v997 * ((v1002 * v3720) / v1003)) } else { v4 }) }) / self.scalar_v1023) } else { v4 });
        let v3769: f64 = (if v942 { ((if v1009 { (v3713 + (self.scalar_v997 * ((v1011 * (-v3721)) / v1012))) } else { (if v1001 { (self.scalar_v997 * ((v1002 * v3721) / v1003)) } else { v4 }) }) / self.scalar_v1023) } else { v4 });
        let v3770: f64 = (if v942 { ((if v1009 { (v3714 + (self.scalar_v997 * ((v1011 * (-v3722)) / v1012))) } else { (if v1001 { (self.scalar_v997 * ((v1002 * v3722) / v1003)) } else { v4 }) }) / self.scalar_v1023) } else { v4 });
        let v3775: f64 = (if v942 { (v3651 / self.scalar_v986) } else { v4 });
        let v3776: f64 = (if v942 { (v3652 / self.scalar_v986) } else { v4 });
        let v3777: f64 = (if v942 { (v3653 / self.scalar_v986) } else { v4 });
        let v3778: f64 = (if v942 { (v3654 / self.scalar_v986) } else { v4 });
        let v3807: f64 = (v32 * v1033);
        let v3830: f64 = ((v1036 * (((v1030 * ((v1028 * v3775) + (v1027 * (v452 * v3767)))) + (v1029 * v3775)) / v3807)) - (v1034 * ((v1035 * v3775) + (v1030 * (v32 * v3767)))));
        let v3831: f64 = (v1036 * v1036);
        let v3835: f64 = ((v1036 * (((v1030 * ((v1028 * v3776) + (v1027 * (v452 * v3768)))) + (v1029 * v3776)) / v3807)) - (v1034 * ((v1035 * v3776) + (v1030 * (v32 * v3768)))));
        let v3839: f64 = ((v1036 * (((v1030 * ((v1028 * v3777) + (v1027 * (v452 * v3769)))) + (v1029 * v3777)) / v3807)) - (v1034 * ((v1035 * v3777) + (v1030 * (v32 * v3769)))));
        let v3843: f64 = ((v1036 * (((v1030 * ((v1028 * v3778) + (v1027 * (v452 * v3770)))) + (v1029 * v3778)) / v3807)) - (v1034 * ((v1035 * v3778) + (v1030 * (v32 * v3770)))));
        let v3845: f64 = (if v942 { (v3830 / v3831) } else { v4 });
        let v3846: f64 = (if v942 { (v3835 / v3831) } else { v4 });
        let v3847: f64 = (if v942 { (v3839 / v3831) } else { v4 });
        let v3848: f64 = (if v942 { (v3843 / v3831) } else { v4 });
        let v3855: f64 = ((v1038 * v3515) + (v933 * v3845));
        let v3858: f64 = ((v1038 * v3516) + (v933 * v3846));
        let v3861: f64 = ((v1038 * v3517) + (v933 * v3847));
        let v3862: f64 = (v933 * v3848);
        let v3870: f64 = (v1042 * v1042);
        let v3884: f64 = (if v942 { (((v1042 * ((-v3845) + v3855)) - (v1041 * v3855)) / v3870) } else { v4 });
        let v3885: f64 = (if v942 { (((v1042 * ((-v3846) + v3858)) - (v1041 * v3858)) / v3870) } else { v4 });
        let v3886: f64 = (if v942 { (((v1042 * ((-v3847) + v3861)) - (v1041 * v3861)) / v3870) } else { v4 });
        let v3887: f64 = (if v942 { (((v1042 * ((-v3848) + v3862)) - (v1041 * v3862)) / v3870) } else { v4 });
        let v3906: f64 = (if v942 { ((v1045 * v2713) + (v122 * ((v1044 * v3570) + (v956 * v3884)))) } else { v4 });
        let v3907: f64 = (if v942 { (v122 * ((v1044 * v3571) + (v956 * v3885))) } else { v4 });
        let v3908: f64 = (if v942 { (v122 * ((v1044 * v3572) + (v956 * v3886))) } else { v4 });
        let v3909: f64 = (if v942 { (v122 * ((v1044 * v3573) + (v956 * v3887))) } else { v4 });
        let v3931: f64 = (if v942 { ((v32 * v3906) + ((v1050 * v3515) + (v933 * (v3515 + v3906)))) } else { v4 });
        let v3932: f64 = (if v942 { ((v32 * v3907) + ((v1050 * v3516) + (v933 * (v3516 + v3907)))) } else { v4 });
        let v3933: f64 = (if v942 { ((v32 * v3908) + ((v1050 * v3517) + (v933 * (v3517 + v3908)))) } else { v4 });
        let v3934: f64 = (if v942 { ((v32 * v3909) + (v933 * v3909)) } else { v4 });
        let v3939: f64 = (if v942 { (v440 * v3906) } else { v4 });
        let v3940: f64 = (if v942 { (v440 * v3907) } else { v4 });
        let v3941: f64 = (if v942 { (v440 * v3908) } else { v4 });
        let v3942: f64 = (if v942 { (v440 * v3909) } else { v4 });
        let v3943: f64 = (v1056 * v3939);
        let v3945: f64 = (v1056 * v3940);
        let v3947: f64 = (v1056 * v3941);
        let v3949: f64 = (v1056 * v3942);
        let v3955: f64 = (if v942 { (v3931 + (v3943 + v3943)) } else { v4 });
        let v3956: f64 = (if v942 { (v3932 + (v3945 + v3945)) } else { v4 });
        let v3957: f64 = (if v942 { (v3933 + (v3947 + v3947)) } else { v4 });
        let v3958: f64 = (if v942 { (v3934 + (v3949 + v3949)) } else { v4 });
        let v3959: f64 = (v32 * v1062);
        let v3960: f64 = (v3955 / v3959);
        let v3961: f64 = (v3956 / v3959);
        let v3962: f64 = (v3957 / v3959);
        let v3963: f64 = (v3958 / v3959);
        let v3979: f64 = (v1067 * v1067);
        let v3997: f64 = (if v1072 { v4 } else { (if v1066 { (((v1067 * v3931) - (v1053 * (v3960 - v3939))) / v3979) } else { (if v1061 { (v3939 + v3960) } else { v4 }) }) });
        let v3998: f64 = (if v1072 { v4 } else { (if v1066 { (((v1067 * v3932) - (v1053 * (v3961 - v3940))) / v3979) } else { (if v1061 { (v3940 + v3961) } else { v4 }) }) });
        let v3999: f64 = (if v1072 { v4 } else { (if v1066 { (((v1067 * v3933) - (v1053 * (v3962 - v3941))) / v3979) } else { (if v1061 { (v3941 + v3962) } else { v4 }) }) });
        let v4000: f64 = (if v1072 { v4 } else { (if v1066 { (((v1067 * v3934) - (v1053 * (v3963 - v3942))) / v3979) } else { (if v1061 { (v3942 + v3963) } else { v4 }) }) });
        let v4031: f64 = (if v942 { (self.scalar_v1080 * v3553) } else { v4 });
        let v4032: f64 = (if v942 { (self.scalar_v1080 * v3554) } else { v4 });
        let v4033: f64 = (if v942 { (self.scalar_v1080 * v3555) } else { v4 });
        let v4034: f64 = (if v942 { (self.scalar_v1080 * v3556) } else { v4 });
        let v4047: f64 = (v1083 * v4031);
        let v4049: f64 = (v1083 * v4032);
        let v4051: f64 = (v1083 * v4033);
        let v4053: f64 = (v1083 * v4034);
        let v4059: f64 = (v32 * v1090);
        let v4068: f64 = (if v942 { (v4031 + (((if v942 { ((v1085 * v3553) + (v941 * (self.scalar_v984 * (self.scalar_v985 * v2954)))) } else { v4 }) + (v4047 + v4047)) / v4059)) } else { v4 });
        let v4072: f64 = (v47 * v2851);
        let v4085: f64 = (v1101 * v1101);
        let v4105: f64 = (if v1099 { ((v1103 * v2851) + (v261 * (((v1101 * (v32 * v3553)) - (v1100 * (v3553 + v3690))) / v4085))) } else { (if v1095 { v4072 } else { v4 }) });
        let v4109: f64 = (self.scalar_v984 * v3553);
        let v4110: f64 = (self.scalar_v984 * v3554);
        let v4111: f64 = (self.scalar_v984 * v3555);
        let v4112: f64 = (self.scalar_v984 * v3556);
        let v4116: f64 = (v1107 * v1107);
        let v4152: f64 = (v935 * v935);
        let v4165: f64 = (if v1112 { (((v935 * (v32 * v3470)) - (v1113 * v3491)) / v4152) } else { v4000 });
        let v4166: f64 = (if v1112 { v3284 } else { (if v942 { ((v1077 * ((v1074 * v3997) + (v1073 * v3997))) + (v1075 * (v1077 * ((v220 * v2713) + (v122 * v2807))))) } else { v4 }) });
        let v4167: f64 = (if v1112 { v3285 } else { (if v942 { (v1077 * ((v1074 * v3998) + (v1073 * v3998))) } else { v4 }) });
        let v4168: f64 = (if v1112 { v4 } else { (if v942 { (v1077 * ((v1074 * v3999) + (v1073 * v3999))) } else { v4 }) });
        let v4169: f64 = (if v1112 { v3286 } else { (if v942 { (v1077 * ((v1074 * v4000) + (v1073 * v4000))) } else { v4 }) });
        let v4170: f64 = (v3515 + (if v1112 { (((v935 * (v32 * v3468)) - (v1113 * v3489)) / v4152) } else { v3997 }));
        let v4171: f64 = (v3516 + (if v1112 { (((v935 * (v32 * v3469)) - (v1113 * v3490)) / v4152) } else { v3998 }));
        let v4172: f64 = (v3517 + (if v1112 { v4 } else { v3999 }));
        let v4177: f64 = (if v1128 { (v440 * v4170) } else { v4 });
        let v4178: f64 = (if v1128 { (v440 * v4171) } else { v4 });
        let v4179: f64 = (if v1128 { (v440 * v4172) } else { v4 });
        let v4180: f64 = (if v1128 { (v440 * v4165) } else { v4 });
        let v4184: f64 = (v1132 * v1132);
        let v4208: f64 = (v1138 * v1138);
        let v4222: f64 = (if v1136 { (((v1138 * v3543) - (v939 * v3543)) / v4208) } else { (if v1128 { (((v1132 * v4177) - (v1131 * v4177)) / v4184) } else { v3884 }) });
        let v4223: f64 = (if v1136 { (((v1138 * v3544) - (v939 * ((self.scalar_v0 + v3544) - self.scalar_v0))) / v4208) } else { (if v1128 { (((v1132 * v4178) - (v1131 * v4178)) / v4184) } else { v3885 }) });
        let v4224: f64 = (if v1136 { (((v1138 * v3545) - (v939 * (v3545 - self.scalar_v3268))) / v4208) } else { (if v1128 { (((v1132 * v4179) - (v1131 * v4179)) / v4184) } else { v3886 }) });
        let v4225: f64 = (if v1136 { (((v1138 * v3546) - (v939 * v3548)) / v4208) } else { (if v1128 { (((v1132 * v4180) - (v1131 * v4180)) / v4184) } else { v3887 }) });
        let v4230: f64 = (if v1112 { v4072 } else { v4105 });
        let v4231: f64 = (if v1112 { v4 } else { (if v1099 { (v261 * (((v1101 * (v32 * v3554)) - (v1100 * (v3554 + v3691))) / v4085)) } else { v4 }) });
        let v4232: f64 = (if v1112 { v4 } else { (if v1099 { (v261 * (((v1101 * (v32 * v3555)) - (v1100 * (v3555 + v3692))) / v4085)) } else { v4 }) });
        let v4233: f64 = (if v1112 { v4 } else { (if v1099 { (v261 * (((v1101 * (v32 * v3556)) - (v1100 * (v3556 + v3693))) / v4085)) } else { v4 }) });
        let v4234: f64 = (if v1112 { v3553 } else { (if v942 { (((v1107 * v4109) - (v1106 * v3553)) / v4116) } else { v4 }) });
        let v4235: f64 = (if v1112 { v3554 } else { (if v942 { (((v1107 * v4110) - (v1106 * v3554)) / v4116) } else { v4 }) });
        let v4236: f64 = (if v1112 { v3555 } else { (if v942 { (((v1107 * v4111) - (v1106 * v3555)) / v4116) } else { v4 }) });
        let v4237: f64 = (if v1112 { v3556 } else { (if v942 { (((v1107 * v4112) - (v1106 * v3556)) / v4116) } else { v4 }) });
        let v4246: f64 = (if v1112 { (-(v4234 / self.scalar_v984)) } else { (if v942 { ((-v4109) / v4116) } else { v4 }) });
        let v4247: f64 = (if v1112 { (-(v4235 / self.scalar_v984)) } else { (if v942 { ((-v4110) / v4116) } else { v4 }) });
        let v4248: f64 = (if v1112 { (-(v4236 / self.scalar_v984)) } else { (if v942 { ((-v4111) / v4116) } else { v4 }) });
        let v4249: f64 = (if v1112 { (-(v4237 / self.scalar_v984)) } else { (if v942 { ((-v4112) / v4116) } else { v4 }) });
        let v4250: f64 = (self.scalar_v1149 * v2784);
        let v4251: f64 = (v47 * v2784);
        let v4253: f64 = (v1151 * (-v4250));
        let v4256: f64 = (v1151 * v1151);
        let v4257: f64 = ((v4253 - (v1152 * v4251)) / v4256);
        let v4258: f64 = (self.scalar_v3268 / v1151);
        let v4259: f64 = (self.scalar_v0 / v1151);
        let v4278: f64 = (-v4258);
        let v4279: f64 = (-v4259);
        let v4294: f64 = (if v1161 { (v4250 - ((v1165 * v4251) + (v1151 * ((v1163 * (-v4257)) / v1164)))) } else { (if v1154 { (-((v1157 * v4251) + (v1151 * ((v1155 * v4257) / v1156)))) } else { v4 }) });
        let v4295: f64 = (if v1161 { (-(v1151 * ((v1163 * v4278) / v1164))) } else { (if v1154 { (self.scalar_v3268 - (v1151 * ((v1155 * v4258) / v1156))) } else { v4 }) });
        let v4296: f64 = (if v1161 { (-(v1151 * ((v1163 * v4279) / v1164))) } else { (if v1154 { (self.scalar_v0 - (v1151 * ((v1155 * v4259) / v1156))) } else { v4 }) });
        let v4302: f64 = (-((v1168 * v2900) + (v308 * v4294)));
        let v4303: f64 = (-(v308 * v4295));
        let v4304: f64 = (-(v308 * v4296));
        let v4307: f64 = (self.scalar_v1171 * f64::powf(v1170, self.scalar_v4305));
        let v4308: f64 = (v4302 * v4307);
        let v4309: f64 = (v4303 * v4307);
        let v4310: f64 = (v4304 * v4307);
        let v4311: f64 = (v2784 / self.scalar_v1171);
        let v4326: f64 = (((v1174 * v4311) + (v1173 * (-v4308))) + (v171 * (-v4294)));
        let v4327: f64 = ((v1173 * (-v4309)) + (v171 * (self.scalar_v3268 - v4295)));
        let v4328: f64 = ((v1173 * (-v4310)) + (v171 * (self.scalar_v0 - v4296)));
        let v4334: f64 = (if self.scalar_v1184 { (self.scalar_v0 + (if v1112 { v4 } else { (if v942 { (v4032 + (((if v942 { (v1085 * v3554) } else { v4 }) + (v4049 + v4049)) / v4059)) } else { v4 }) })) } else { self.scalar_v4329 });
        let v4335: f64 = (if self.scalar_v1184 { (self.scalar_v3268 + (if v1112 { self.scalar_v0 } else { (if v942 { (v4033 + (((if v942 { (v1085 * v3555) } else { v4 }) + (v4051 + v4051)) / v4059)) } else { v4 }) })) } else { self.scalar_v4330 });
        let v4337: f64 = (if self.scalar_v1188 { v4 } else { (if self.scalar_v1184 { (if v1112 { v4 } else { v4068 }) } else { v4 }) });
        let v4338: f64 = (if self.scalar_v1188 { self.scalar_v0 } else { v4334 });
        let v4339: f64 = (if self.scalar_v1188 { v4 } else { v4335 });
        let v4340: f64 = (if self.scalar_v1188 { self.scalar_v3268 } else { (if self.scalar_v1184 { (if v1112 { self.scalar_v3268 } else { (if v942 { (v4034 + (((if v942 { (v1085 * v3556) } else { v4 }) + (v4053 + v4053)) / v4059)) } else { v4 }) }) } else { v4 }) });
        let v4341: f64 = (-v2933);
        let v4346: f64 = (((v1191 * v4341) - (v1190 * v4341)) / (v1191 * v1191));
        let v4354: f64 = ((v1195 * v2851) + (v261 * (-(v4346 * (self.scalar_v1193 * f64::powf(v1192, self.scalar_v4347))))));
        let v4359: f64 = (v1142 * v1142);
        let v4360: f64 = (((v1142 * (v4337 - v4354)) - (v1197 * v4230)) / v4359);
        let v4364: f64 = (((v1142 * v4338) - (v1197 * v4231)) / v4359);
        let v4368: f64 = (((v1142 * v4339) - (v1197 * v4232)) / v4359);
        let v4372: f64 = (((v1142 * v4340) - (v1197 * v4233)) / v4359);
        let v4429: f64 = (if v1206 { (v4354 - ((v1210 * v4230) + (v1142 * ((v1208 * (-v4360)) / v1209)))) } else { (if v1199 { (v4337 - ((v1202 * v4230) + (v1142 * ((v1200 * v4360) / v1201)))) } else { v4 }) });
        let v4430: f64 = (if v1206 { (-((v1210 * v4231) + (v1142 * ((v1208 * (-v4364)) / v1209)))) } else { (if v1199 { (v4338 - ((v1202 * v4231) + (v1142 * ((v1200 * v4364) / v1201)))) } else { v4 }) });
        let v4431: f64 = (if v1206 { (-((v1210 * v4232) + (v1142 * ((v1208 * (-v4368)) / v1209)))) } else { (if v1199 { (v4339 - ((v1202 * v4232) + (v1142 * ((v1200 * v4368) / v1201)))) } else { v4 }) });
        let v4432: f64 = (if v1206 { (-((v1210 * v4233) + (v1142 * ((v1208 * (-v4372)) / v1209)))) } else { (if v1199 { (v4340 - ((v1202 * v4233) + (v1142 * ((v1200 * v4372) / v1201)))) } else { v4 }) });
        let v4435: f64 = (self.scalar_v1214 * f64::powf(v1146, self.scalar_v4433));
        let v4436: f64 = (v4246 * v4435);
        let v4437: f64 = (v4247 * v4435);
        let v4438: f64 = (v4248 * v4435);
        let v4439: f64 = (v4249 * v4435);
        let v4440: f64 = (v2851 / self.scalar_v1216);
        let v4454: f64 = (self.scalar_v1216 * f64::powf(v1219, self.scalar_v4452));
        let v4477: f64 = ((v1222 * v4440) + (v1217 * (-((v1220 * v4436) + (v1215 * ((-(((v261 * v4429) - (v1213 * v2851)) / v2902)) * v4454))))));
        let v4504: f64 = ((v1217 * (-((v1220 * v4437) + (v1215 * ((-(v4430 / v261)) * v4454))))) + ((v1225 * (v1192 * v4437)) + (v1224 * (v4338 - v4430))));
        let v4505: f64 = ((v1217 * (-((v1220 * v4438) + (v1215 * ((-(v4431 / v261)) * v4454))))) + ((v1225 * (v1192 * v4438)) + (v1224 * (v4339 - v4431))));
        let v4506: f64 = ((v1217 * (-((v1220 * v4439) + (v1215 * ((-(v4432 / v261)) * v4454))))) + ((v1225 * (v1192 * v4439)) + (v1224 * (v4340 - v4432))));
        let v4512: f64 = (v1191 * v4506);
        let v4514: f64 = (self.scalar_v0 * v330);
        let v4515: f64 = (v330 * self.scalar_v3268);
        let v4516: f64 = (((v1227 * v4341) + (v1191 * (v4477 + ((v1225 * ((v1215 * v4346) + (v1192 * v4436))) + (v1224 * (v4337 - v4429)))))) + (v751 * v2933));
        let v4517: f64 = ((v1191 * v4504) + v4514);
        let v4518: f64 = ((v1191 * v4505) + v4515);
        let v4523: f64 = (v470 * v470);
        let v4524: f64 = (((v470 * (v452 * v3020)) - (v1231 * v3023)) / v4523);
        let v4527: f64 = ((v1232 * v3303) + (v817 * v4524));
        let v4528: f64 = (v1232 * v3304);
        let v4529: f64 = (v1232 * v3305);
        let v4530: f64 = (v32 * v1235);
        let v4531: f64 = (v4527 / v4530);
        let v4532: f64 = (v4528 / v4530);
        let v4533: f64 = (v4529 / v4530);
        let v4537: f64 = (v1236 * v1236);
        let v4538: f64 = (((v1236 * v4527) - (v1233 * v4531)) / v4537);
        let v4542: f64 = (((v1236 * v4528) - (v1233 * v4532)) / v4537);
        let v4546: f64 = (((v1236 * v4529) - (v1233 * v4533)) / v4537);
        let v4552: f64 = (v1238 * f64::powf(v1116, (v1238 - v1)));
        let v4555: f64 = (((-(if self.scalar_v430 { v4 } else { (if self.scalar_v402 { (if v420 { (v2980 + (v31 * ((v422 * (-v2976)) / v423))) } else { v2980 }) } else { v4 }) })) / (v431 * v431)) * (v1239 * v2177));
        let v4556: f64 = ((v4166 * v4552) + v4555);
        let v4557: f64 = (v4167 * v4552);
        let v4558: f64 = (v4168 * v4552);
        let v4559: f64 = (v4169 * v4552);
        let v4562: f64 = ((v1239 * v4524) + (v1232 * v4556));
        let v4563: f64 = (v1232 * v4557);
        let v4564: f64 = (v1232 * v4558);
        let v4565: f64 = (v1232 * v4559);
        let v4566: f64 = (v32 * v1242);
        let v4574: f64 = (v1243 * v1243);
        let v4575: f64 = (((v1243 * v4562) - (v1240 * (v4562 / v4566))) / v4574);
        let v4579: f64 = (((v1243 * v4563) - (v1240 * (v4563 / v4566))) / v4574);
        let v4583: f64 = (((v1243 * v4564) - (v1240 * (v4564 / v4566))) / v4574);
        let v4587: f64 = (((v1243 * v4565) - (v1240 * (v4565 / v4566))) / v4574);
        let v4592: f64 = (((v639 * v4326) - (v1178 * ((v638 * v3116) + (v590 * (self.scalar_v637 * v3191))))) / (v639 * v639));
        let v4593: f64 = (v4327 / v639);
        let v4594: f64 = (v4328 / v639);
        let v4598: f64 = (v636 * v636);
        let v4599: f64 = (((v636 * v4516) - (v1230 * v3195)) / v4598);
        let v4600: f64 = (v4517 / v636);
        let v4601: f64 = (v4518 / v636);
        let v4602: f64 = (v4512 / v636);
        let v4603: f64 = (v4592 + v4599);
        let v4604: f64 = (v4594 + v4600);
        let v4646: f64 = (if self.scalar_v1251 { ((v1257 * v2713) + (v122 * ((v1256 * v3249) + (v718 * (((v636 * (-v4516)) - (v1255 * v3195)) / v4598))))) } else { v4 });
        let v4667: f64 = ((v1265 * ((v1260 * (if self.scalar_v1251 { ((v1252 * v2713) + (v122 * ((v1247 * v3249) + (v718 * v4592)))) } else { v4 })) - (v1261 * v4646))) - (v1262 * (v1264 * ((v718 * v2713) + (v122 * v3249)))));
        let v4671: f64 = (((v1260 * (if self.scalar_v1251 { (v122 * (v718 * v4594)) } else { v4 })) - (v1261 * (if self.scalar_v1251 { (v122 * (v718 * ((-v4517) / v636))) } else { v4 }))) / v1265);
        let v4674: f64 = (if self.scalar_v1251 { (v4667 / (v1265 * v1265)) } else { (if self.scalar_v1245 { v4603 } else { v4 }) });
        let v4675: f64 = (if self.scalar_v1251 { ((v1260 * (if self.scalar_v1251 { (v122 * (v718 * v4593)) } else { v4 })) / v1265) } else { (if self.scalar_v1245 { v4593 } else { v4 }) });
        let v4676: f64 = (if self.scalar_v1251 { v4671 } else { (if self.scalar_v1245 { v4604 } else { v4 }) });
        let v4677: f64 = (if self.scalar_v1251 { ((-(v1261 * (if self.scalar_v1251 { (v122 * (v718 * ((-v4518) / v636))) } else { v4 }))) / v1265) } else { (if self.scalar_v1245 { v4601 } else { v4 }) });
        let v4678: f64 = (if self.scalar_v1251 { ((-(v1261 * (if self.scalar_v1251 { (v122 * (v718 * ((-v4512) / v636))) } else { v4 }))) / v1265) } else { (if self.scalar_v1245 { v4602 } else { v4 }) });
        let v4679: f64 = (v1267 * v4674);
        let v4680: f64 = (v4679 + v4679);
        let v4681: f64 = (v1267 * v4675);
        let v4682: f64 = (v4681 + v4681);
        let v4683: f64 = (v1267 * v4676);
        let v4684: f64 = (v4683 + v4683);
        let v4685: f64 = (v1267 * v4677);
        let v4686: f64 = (v4685 + v4685);
        let v4687: f64 = (v1267 * v4678);
        let v4688: f64 = (v4687 + v4687);
        let v4689: f64 = (v32 * v1273);
        let v4690: f64 = (v4680 / v4689);
        let v4691: f64 = (v4682 / v4689);
        let v4692: f64 = (v4684 / v4689);
        let v4693: f64 = (v4686 / v4689);
        let v4694: f64 = (v4688 / v4689);
        let v4702: f64 = (v1274 * v1274);
        let v4738: f64 = (v440 * (v4538 + v4575));
        let v4739: f64 = (v440 * v4542);
        let v4740: f64 = (v440 * (v4546 + v4579));
        let v4741: f64 = (v440 * v4583);
        let v4742: f64 = (v440 * v4587);
        let v4745: f64 = ((v1283 * (if v1277 { (v440 * (v4674 + v4690)) } else { (if v1270 { ((-(v1271 * (v4690 - v4674))) / v4702) } else { v4 }) })) + (v1280 * v4738));
        let v4748: f64 = ((v1283 * (if v1277 { (v440 * (v4675 + v4691)) } else { (if v1270 { ((-(v1271 * (v4691 - v4675))) / v4702) } else { v4 }) })) + (v1280 * v4739));
        let v4751: f64 = ((v1283 * (if v1277 { (v440 * (v4676 + v4692)) } else { (if v1270 { ((-(v1271 * (v4692 - v4676))) / v4702) } else { v4 }) })) + (v1280 * v4740));
        let v4754: f64 = ((v1283 * (if v1277 { (v440 * (v4677 + v4693)) } else { (if v1270 { ((-(v1271 * (v4693 - v4677))) / v4702) } else { v4 }) })) + (v1280 * v4741));
        let v4757: f64 = ((v1283 * (if v1277 { (v440 * (v4678 + v4694)) } else { (if v1270 { ((-(v1271 * (v4694 - v4678))) / v4702) } else { v4 }) })) + (v1280 * v4742));
        let v4761: f64 = ((v1286 * v4556) + (v1239 * (self.scalar_v1285 * v3020)));
        let v4762: f64 = (v1286 * v4557);
        let v4763: f64 = (v1286 * v4558);
        let v4764: f64 = (v1286 * v4559);
        let v4767: f64 = ((v817 * v3020) + (v465 * v3303));
        let v4769: f64 = (v465 * v3305);
        let v4777: f64 = (v1284 * v1284);
        let v4778: f64 = (((v1284 * (v4767 - v4761)) - (v1289 * v4745)) / v4777);
        let v4779: f64 = (v1284 * (v465 * v3304));
        let v4782: f64 = ((v4779 - (v1289 * v4748)) / v4777);
        let v4786: f64 = (((v1284 * (v4769 - v4762)) - (v1289 * v4751)) / v4777);
        let v4790: f64 = (((v1284 * (-v4763)) - (v1289 * v4754)) / v4777);
        let v4794: f64 = (((v1284 * (-v4764)) - (v1289 * v4757)) / v4777);
        let v4815: f64 = (if v1299 { (self.scalar_v3268 + (v1291 * ((v1301 * self.scalar_v4805) / v1302))) } else { (if v1293 { (v1291 * ((v1294 * self.scalar_v4795) / v1295)) } else { v4 }) });
        let v4816: f64 = (if v1299 { (self.scalar_v0 + (v1291 * ((v1301 * self.scalar_v4806) / v1302))) } else { (if v1293 { (v1291 * ((v1294 * self.scalar_v4796) / v1295)) } else { v4 }) });
        let v4817: f64 = (v4815 / self.scalar_v1307);
        let v4818: f64 = (v4816 / self.scalar_v1307);
        let v4825: f64 = (if v1312 { (v1313 * v4817) } else { (if v1309 { (v1310 * v4817) } else { v4 }) });
        let v4826: f64 = (if v1312 { (v1313 * v4818) } else { (if v1309 { (v1310 * v4818) } else { v4 }) });
        let v4852: f64 = (if v1330 { (-(v31 * ((v1332 * self.scalar_v4842) / v1333))) } else { (if v1323 { (self.scalar_v3268 - (v31 * ((v1324 * self.scalar_v4830) / v1325))) } else { v4 }) });
        let v4853: f64 = (if v1330 { (-(v31 * ((v1332 * self.scalar_v4843) / v1333))) } else { (if v1323 { (self.scalar_v0 - (v31 * ((v1324 * self.scalar_v4831) / v1325))) } else { v4 }) });
        let v4859: f64 = (v32 * f64::powf(v1340, v1));
        let v4868: f64 = (v3287 / self.scalar_v507);
        let v4869: f64 = (v3274 / self.scalar_v507);
        let v4870: f64 = (v3273 / self.scalar_v507);
        let v4880: f64 = (if v1347 { (v1348 * v4868) } else { (if v1344 { (v1345 * v4868) } else { v4 }) });
        let v4881: f64 = (if v1347 { (v1348 * v4869) } else { (if v1344 { (v1345 * v4869) } else { v4815 }) });
        let v4882: f64 = (if v1347 { (v1348 * v4870) } else { (if v1344 { (v1345 * v4870) } else { v4816 }) });
        let v4885: f64 = (v122 * (-v2874));
        let v4886: f64 = ((v1353 * v2713) + v4885);
        let v4896: f64 = (if v1360 { (v1361 * v4886) } else { (if v1356 { (v1357 * v4886) } else { v4 }) });
        let v4897: f64 = (if v1360 { (v1361 * v3274) } else { (if v1356 { (v1357 * v3274) } else { v4817 }) });
        let v4898: f64 = (if v1360 { (v1361 * v3273) } else { (if v1356 { (v1357 * v3273) } else { v4818 }) });
        let v4902: f64 = (v465 * v465);
        let v4903: f64 = (((v465 * v4778) - (v1290 * v3020)) / v4902);
        let v4904: f64 = (v4782 / v465);
        let v4905: f64 = (v4786 / v465);
        let v4906: f64 = (v4790 / v465);
        let v4907: f64 = (v4794 / v465);
        let v4923: f64 = (if v1375 { (v1377 * v4903) } else { (if v1371 { (v1372 * v4903) } else { v4 }) });
        let v4924: f64 = (if v1375 { (v1377 * v4904) } else { (if v1371 { (v1372 * v4904) } else { v4825 }) });
        let v4925: f64 = (if v1375 { (v1377 * v4905) } else { (if v1371 { (v1372 * v4905) } else { v4826 }) });
        let v4926: f64 = (if v1375 { (v1377 * v4906) } else { (if v1371 { (v1372 * v4906) } else { v4 }) });
        let v4927: f64 = (if v1375 { (v1377 * v4907) } else { (if v1371 { (v1372 * v4907) } else { v4 }) });
        let v4930: f64 = ((v1382 * v3054) + (v516 * v4880));
        let v4931: f64 = (v516 * v4881);
        let v4932: f64 = (v516 * v4882);
        let v4942: f64 = (v32 * v1388);
        let v4948: f64 = ((v1389 * ((v1384 * v4880) + (v1382 * (v32 * (if self.scalar_v526 { (self.scalar_v527 * (v532 * ((self.scalar_v529 * v2713) / self.scalar_v507))) } else { v4 }))))) - (v1385 * ((v452 * v4896) / v4942)));
        let v4949: f64 = (v1389 * v1389);
        let v4992: f64 = ((v1397 * ((v1395 * v4923) + (v1381 * ((v1394 * (if self.scalar_v526 { (self.scalar_v535 * (v539 * (self.scalar_v537 * v2713))) } else { v4 })) + (v541 * v4166))))) - (v1396 * v4923));
        let v4993: f64 = (v1397 * v1397);
        let v5012: f64 = ((v4931 + (v1391 * (((v1389 * (v1384 * v4881)) - (v1385 * ((v452 * v4897) / v4942))) / v4949))) + (((v1397 * (v1395 * v4924)) - (v1396 * v4924)) / v4993));
        let v5013: f64 = ((v4932 + ((v1391 * (((v1389 * (v1384 * v4882)) - (v1385 * ((v452 * v4898) / v4942))) / v4949)) + (v1390 * v4600))) + (((v1397 * ((v1395 * v4925) + (v1381 * (v541 * v4167)))) - (v1396 * v4925)) / v4993));
        let v5024: f64 = (if self.scalar_v1404 { v4 } else { (if self.scalar_v526 { ((v1390 * v4601) + (((v1397 * ((v1395 * v4926) + (v1381 * (v541 * v4168)))) - (v1396 * v4926)) / v4993)) } else { v4 }) });
        let v5025: f64 = (if self.scalar_v1404 { v4 } else { (if self.scalar_v526 { ((v1390 * v4602) + (((v1397 * ((v1395 * v4927) + (v1381 * (v541 * v4169)))) - (v1396 * v4927)) / v4993)) } else { v4 }) });
        let v5059: f64 = (if self.scalar_v1407 { ((v1414 * v3054) + (v516 * ((self.scalar_v1408 * v4880) + ((v1412 * v4599) + (v1391 * (self.scalar_v1401 * (v4166 + v4880))))))) } else { (if self.scalar_v1404 { v4930 } else { (if self.scalar_v526 { ((v4930 + ((v1391 * (v4948 / v4949)) + (v1390 * v4599))) + (v4992 / v4993)) } else { v4 }) }) });
        let v5061: f64 = (if self.scalar_v1407 { (v516 * ((self.scalar_v1408 * v4882) + ((v1412 * v4600) + (v1391 * (self.scalar_v1401 * (v4167 + v4882)))))) } else { (if self.scalar_v1404 { v4932 } else { (if self.scalar_v526 { v5013 } else { v4 }) }) });
        let v5062: f64 = (if self.scalar_v1407 { (v516 * ((v1412 * v4601) + (v1391 * (self.scalar_v1401 * v4168)))) } else { v5024 });
        let v5063: f64 = (if self.scalar_v1407 { (v516 * ((v1412 * v4602) + (v1391 * (self.scalar_v1401 * v4169)))) } else { v5025 });
        let v5064: f64 = (v760 * v2713);
        let v5065: f64 = (v5064 / self.scalar_v518);
        let v5066: f64 = (v3274 / self.scalar_v518);
        let v5067: f64 = (v3273 / self.scalar_v518);
        let v5078: f64 = (if v1422 { (v1423 * v5065) } else { (if v1419 { (v1420 * v5065) } else { v4880 }) });
        let v5079: f64 = (if v1422 { (v1423 * v5066) } else { (if v1419 { (v1420 * v5066) } else { v4881 }) });
        let v5080: f64 = (if v1422 { (v1423 * v5067) } else { (if v1419 { (v1420 * v5067) } else { v4 }) });
        let v5081: f64 = (if v1422 { v4 } else { (if v1419 { v4 } else { v4882 }) });
        let v5083: f64 = (v4885 + (v1428 * v2713));
        let v5100: f64 = ((v1441 * ((v523 * (self.scalar_v517 * (v520 * (v3045 / self.scalar_v518)))) + (v521 * (v523 * (v3049 / self.scalar_v518))))) + (v524 * v5078));
        let v5101: f64 = (v524 * v5079);
        let v5102: f64 = (v524 * v5080);
        let v5103: f64 = (v524 * v5081);
        let v5115: f64 = (v32 * v1447);
        let v5122: f64 = ((v1448 * ((v1443 * v5078) + (v1441 * (v32 * (if self.scalar_v526 { (self.scalar_v542 * (v547 * ((self.scalar_v544 * v2713) / self.scalar_v518))) } else { v4 }))))) - (v1444 * ((v452 * (if v1435 { (v1436 * v5083) } else { (if v1431 { (v1432 * v5083) } else { v4896 }) })) / v5115)));
        let v5123: f64 = (v1448 * v1448);
        let v5128: f64 = (((v1448 * (v1443 * v5079)) - (v1444 * ((v452 * (if v1435 { (v1436 * v3274) } else { (if v1431 { (v1432 * v3274) } else { v4897 }) })) / v5115))) / v5123);
        let v5132: f64 = (((v1448 * (v1443 * v5080)) - (v1444 * ((v452 * (if v1435 { (v1436 * v3273) } else { (if v1431 { (v1432 * v3273) } else { v4 }) })) / v5115))) / v5123);
        let v5144: f64 = (if self.scalar_v526 { (v5103 + (((v1448 * (v1443 * v5081)) - (v1444 * ((v452 * (if v1435 { v4 } else { (if v1431 { v4 } else { v4898 }) })) / v5115))) / v5123)) } else { v4 });
        let v5149: f64 = (v3287 / self.scalar_v479);
        let v5150: f64 = (v3274 / self.scalar_v479);
        let v5151: f64 = (v3273 / self.scalar_v479);
        let v5162: f64 = (if v1457 { (v1458 * v5149) } else { (if v1454 { (v1455 * v5149) } else { v5078 }) });
        let v5163: f64 = (if v1457 { (v1458 * v5150) } else { (if v1454 { (v1455 * v5150) } else { v5079 }) });
        let v5164: f64 = (if v1457 { v4 } else { (if v1454 { v4 } else { v5080 }) });
        let v5165: f64 = (if v1457 { (v1458 * v5151) } else { (if v1454 { (v1455 * v5151) } else { v5081 }) });
        let v5170: f64 = (v490 * v5164);
        let v5172: f64 = (v5064 / self.scalar_v562);
        let v5173: f64 = (v3274 / self.scalar_v562);
        let v5174: f64 = (v3273 / self.scalar_v562);
        let v5185: f64 = (if v1469 { (v1470 * v5172) } else { (if v1466 { (v1467 * v5172) } else { v5162 }) });
        let v5186: f64 = (if v1469 { (v1470 * v5173) } else { (if v1466 { (v1467 * v5173) } else { v5163 }) });
        let v5187: f64 = (if v1469 { (v1470 * v5174) } else { (if v1466 { (v1467 * v5174) } else { v5164 }) });
        let v5188: f64 = (if v1469 { v4 } else { (if v1466 { v4 } else { v5165 }) });
        let v5195: f64 = (v3306 / self.scalar_v492);
        let v5196: f64 = (v3273 / self.scalar_v492);
        let v5197: f64 = (v3307 / self.scalar_v492);
        let v5198: f64 = (v3308 / self.scalar_v492);
        let v5199: f64 = (v3274 / self.scalar_v492);
        let v5216: f64 = (if v1481 { (v1482 * v5195) } else { (if v1478 { (v1479 * v5195) } else { v5185 }) });
        let v5217: f64 = (if v1481 { v4 } else { (if v1478 { v4 } else { v5186 }) });
        let v5218: f64 = (if v1481 { (v1482 * v5196) } else { (if v1478 { (v1479 * v5196) } else { v5187 }) });
        let v5219: f64 = (if v1481 { (v1482 * v5197) } else { (if v1478 { (v1479 * v5197) } else { v5188 }) });
        let v5220: f64 = (if v1481 { (v1482 * v5198) } else { (if v1478 { (v1479 * v5198) } else { v4 }) });
        let v5221: f64 = (if v1481 { (v1482 * v5199) } else { (if v1478 { (v1479 * v5199) } else { v4 }) });
        let v5224: f64 = ((v1487 * ((v501 * (self.scalar_v491 * (v496 * (self.scalar_v494 * v2714)))) + (v497 * (v501 * ((self.scalar_v498 * v2713) / self.scalar_v492))))) + (v502 * v5216));
        let v5230: f64 = (v5064 / self.scalar_v572);
        let v5231: f64 = (v3274 / self.scalar_v572);
        let v5232: f64 = (v3273 / self.scalar_v572);
        let v5245: f64 = (if v1493 { (v1494 * v5230) } else { (if v1490 { (v1491 * v5230) } else { v5216 }) });
        let v5246: f64 = (if v1493 { (v1494 * v5231) } else { (if v1490 { (v1491 * v5231) } else { v5217 }) });
        let v5247: f64 = (if v1493 { (v1494 * v5232) } else { (if v1490 { (v1491 * v5232) } else { v5218 }) });
        let v5248: f64 = (if v1493 { v4 } else { (if v1490 { v4 } else { v5219 }) });
        let v5249: f64 = (if v1493 { v4 } else { (if v1490 { v4 } else { v5220 }) });
        let v5250: f64 = (if v1493 { v4 } else { (if v1490 { v4 } else { v5221 }) });
        let v5257: f64 = (v579 * v5249);
        let v5258: f64 = (v579 * v5250);
        let v5264: f64 = (v1505 * v1505);
        let v5277: f64 = ((v1507 * v3132) + (v599 * (-((-(self.scalar_v35 * (v32 * v4308))) / v5264))));
        let v5278: f64 = (v599 * (-((-(self.scalar_v35 * (v32 * v4309))) / v5264)));
        let v5279: f64 = (v599 * (-((-(self.scalar_v35 * (v32 * v4310))) / v5264)));
        let v5295: f64 = (if v1504 { (v757 * v2900) } else { v3191 });
        let v5296: f64 = (if v1504 { (v308 * self.scalar_v3268) } else { v4 });
        let v5297: f64 = (if v1504 { (self.scalar_v0 * v308) } else { v4 });
        let v5298: f64 = (v1521 * v5295);
        let v5300: f64 = (v1521 * v5296);
        let v5302: f64 = (v1521 * v5297);
        let v5304: f64 = (v32 * v1525);
        let v5310: f64 = (self.scalar_v1527 * f64::powf(v1525, self.scalar_v5308));
        let v5351: f64 = (v1528 * ((self.scalar_v33 * (-(self.scalar_v1532 * (v171 * v5295)))) - ((v1538 * ((v1536 * v5295) + (v1521 * (v478 * v5295)))) + (v1537 * v5295))));
        let v5354: f64 = (v1528 * ((self.scalar_v33 * (-(self.scalar_v1532 * (v171 * v5296)))) - ((v1538 * ((v1536 * v5296) + (v1521 * (v478 * v5296)))) + (v1537 * v5296))));
        let v5357: f64 = (v1528 * ((self.scalar_v33 * (-(self.scalar_v1532 * (v171 * v5297)))) - ((v1538 * ((v1536 * v5297) + (v1521 * (v478 * v5297)))) + (v1537 * v5297))));
        let v5377: f64 = ((v1547 * (v1545 * v3132)) - (v1546 * ((v1544 * v2735) + (v148 * (if v1504 { (v1542 * ((v1540 * (((v5298 + v5298) / v5304) * v5310)) + v5351)) } else { v4 })))));
        let v5378: f64 = (v1547 * v1547);
        let v5382: f64 = ((v1547 * (v599 * self.scalar_v5365)) - (v1546 * (v148 * (if v1504 { (v1542 * ((v1540 * (((v5300 + v5300) / v5304) * v5310)) + v5354)) } else { v4 }))));
        let v5386: f64 = ((v1547 * (v599 * self.scalar_v5366)) - (v1546 * (v148 * (if v1504 { (v1542 * ((v1540 * (((v5302 + v5302) / v5304) * v5310)) + v5357)) } else { v4 }))));
        let v5388: f64 = (if v1504 { (v5377 / v5378) } else { v5295 });
        let v5389: f64 = (if v1504 { (v5382 / v5378) } else { v5296 });
        let v5390: f64 = (if v1504 { (v5386 / v5378) } else { v5297 });
        let v5409: f64 = (v1549 * v1549);
        let v5422: f64 = ((self.scalar_v0 * v1567) + (v1564 * (((v1549 * (-(if v1558 { (v1559 * v5389) } else { (if v1554 { (v1555 * v5389) } else { v4 }) }))) - (v1565 * v5389)) / v5409)));
        let v5425: f64 = ((v1567 * self.scalar_v3268) + (v1564 * (((v1549 * (-(if v1558 { (v1559 * v5390) } else { (if v1554 { (v1555 * v5390) } else { v4 }) }))) - (v1565 * v5390)) / v5409)));
        let v5426: f64 = (if v1553 { (v1564 * (((v1549 * (-(if v1558 { (v1559 * v5388) } else { (if v1554 { (v1555 * v5388) } else { v4 }) }))) - (v1565 * v5388)) / v5409)) } else { v4 });
        let v5463: f64 = (if v1571 { ((v1580 * ((v1572 * v5389) + (v1549 * self.scalar_v5429))) + (v1573 * ((v1578 * (v1574 * v5389)) + (v1575 * (v1576 * v5389))))) } else { (if v1553 { v5422 } else { v4 }) });
        let v5464: f64 = (if v1571 { ((v1580 * ((v1572 * v5390) + (v1549 * self.scalar_v5430))) + (v1573 * ((v1578 * (v1574 * v5390)) + (v1575 * (v1576 * v5390))))) } else { (if v1553 { v5425 } else { v4 }) });
        let v5468: f64 = ((v1583 * (if v1571 { ((v1580 * (v1572 * v5388)) + (v1573 * ((v1578 * (v1574 * v5388)) + (v1575 * (v1576 * v5388))))) } else { v5426 })) + (v1582 * (v32 * ((v608 * v3144) + (v606 * (v608 * (-v3132)))))));
        let v5485: f64 = ((v1585 * (if v1514 { (v1515 * v5278) } else { (if v1510 { (v1511 * v5278) } else { v4 }) })) + (v1519 * ((v1584 * v4309) + (v1172 * (v1583 * v5463)))));
        let v5488: f64 = ((v1585 * (if v1514 { (v1515 * v5279) } else { (if v1510 { (v1511 * v5279) } else { v4 }) })) + (v1519 * ((v1584 * v4310) + (v1172 * (v1583 * v5464)))));
        let v5490: f64 = (v308 * ((v1585 * (if v1514 { (v1515 * v5277) } else { (if v1510 { (v1511 * v5277) } else { v4 }) })) + (v1519 * ((v1584 * v4308) + (v1172 * v5468)))));
        let v5503: f64 = (v751 * v2903);
        let v5504: f64 = (self.scalar_v0 * v309);
        let v5505: f64 = (v309 * self.scalar_v3268);
        let v5510: f64 = (self.scalar_v1216 * f64::powf(v1598, self.scalar_v4452));
        let v5514: f64 = (if v1596 { ((-v5503) * v5510) } else { v4 });
        let v5515: f64 = (if v1596 { ((-v5504) * v5510) } else { v4 });
        let v5516: f64 = (if v1596 { ((-v5505) * v5510) } else { v4 });
        let v5522: f64 = (v1601 * v1601);
        let v5535: f64 = ((v1603 * v3172) + (v621 * (-((-(self.scalar_v70 * (v32 * v5514))) / v5522))));
        let v5536: f64 = (v621 * (-((-(self.scalar_v70 * (v32 * v5515))) / v5522)));
        let v5537: f64 = (v621 * (-((-(self.scalar_v70 * (v32 * v5516))) / v5522)));
        let v5550: f64 = (if v1596 { v5503 } else { v3153 });
        let v5551: f64 = (if v1596 { v5504 } else { v4 });
        let v5552: f64 = (if v1596 { v5505 } else { v4 });
        let v5553: f64 = (v1616 * v5550);
        let v5555: f64 = (v1616 * v5551);
        let v5557: f64 = (v1616 * v5552);
        let v5559: f64 = (v32 * v1619);
        let v5565: f64 = (self.scalar_v1620 * f64::powf(v1619, self.scalar_v5563));
        let v5606: f64 = (v1621 * ((self.scalar_v68 * (-(self.scalar_v1625 * (v171 * v5550)))) - ((v1631 * ((v1629 * v5550) + (v1616 * (v478 * v5550)))) + (v1630 * v5550))));
        let v5609: f64 = (v1621 * ((self.scalar_v68 * (-(self.scalar_v1625 * (v171 * v5551)))) - ((v1631 * ((v1629 * v5551) + (v1616 * (v478 * v5551)))) + (v1630 * v5551))));
        let v5612: f64 = (v1621 * ((self.scalar_v68 * (-(self.scalar_v1625 * (v171 * v5552)))) - ((v1631 * ((v1629 * v5552) + (v1616 * (v478 * v5552)))) + (v1630 * v5552))));
        let v5632: f64 = ((v1639 * (v1637 * v3172)) - (v1638 * ((v1636 * v2756) + (v170 * (if v1596 { (v1542 * ((v1633 * (((v5553 + v5553) / v5559) * v5565)) + v5606)) } else { v4 })))));
        let v5633: f64 = (v1639 * v1639);
        let v5637: f64 = ((v1639 * (v621 * self.scalar_v5620)) - (v1638 * (v170 * (if v1596 { (v1542 * ((v1633 * (((v5555 + v5555) / v5559) * v5565)) + v5609)) } else { v4 }))));
        let v5641: f64 = ((v1639 * (v621 * self.scalar_v5621)) - (v1638 * (v170 * (if v1596 { (v1542 * ((v1633 * (((v5557 + v5557) / v5559) * v5565)) + v5612)) } else { v4 }))));
        let v5643: f64 = (if v1596 { (v5632 / v5633) } else { v5550 });
        let v5644: f64 = (if v1596 { (v5637 / v5633) } else { v5551 });
        let v5645: f64 = (if v1596 { (v5641 / v5633) } else { v5552 });
        let v5664: f64 = (v1641 * v1641);
        let v5677: f64 = ((v1658 * self.scalar_v3268) + (v1655 * (((v1641 * (-(if v1649 { (v1650 * v5644) } else { (if v1645 { (v1646 * v5644) } else { v4 }) }))) - (v1656 * v5644)) / v5664)));
        let v5680: f64 = ((self.scalar_v0 * v1658) + (v1655 * (((v1641 * (-(if v1649 { (v1650 * v5645) } else { (if v1645 { (v1646 * v5645) } else { v4 }) }))) - (v1656 * v5645)) / v5664)));
        let v5681: f64 = (if v1644 { (v1655 * (((v1641 * (-(if v1649 { (v1650 * v5643) } else { (if v1645 { (v1646 * v5643) } else { v4 }) }))) - (v1656 * v5643)) / v5664)) } else { v4 });
        let v5716: f64 = (if v1662 { ((v1669 * ((v1663 * v5644) + (v1641 * self.scalar_v5430))) + (v1664 * ((v1667 * (v1574 * v5644)) + (v1665 * (v1576 * v5644))))) } else { (if v1644 { v5677 } else { v4 }) });
        let v5717: f64 = (if v1662 { ((v1669 * ((v1663 * v5645) + (v1641 * self.scalar_v5429))) + (v1664 * ((v1667 * (v1574 * v5645)) + (v1665 * (v1576 * v5645))))) } else { (if v1644 { v5680 } else { v4 }) });
        let v5721: f64 = ((v1672 * (if v1662 { ((v1669 * (v1663 * v5643)) + (v1664 * ((v1667 * (v1574 * v5643)) + (v1665 * (v1576 * v5643))))) } else { v5681 })) + (v1671 * (v32 * ((v630 * v3184) + (v628 * (v630 * (-v3172)))))));
        let v5738: f64 = ((v1674 * (if v1610 { (v1611 * v5536) } else { (if v1606 { (v1607 * v5536) } else { v4 }) })) + (v1615 * ((v1673 * v5515) + (v1600 * (v1672 * v5716)))));
        let v5741: f64 = ((v1674 * (if v1610 { (v1611 * v5537) } else { (if v1606 { (v1607 * v5537) } else { v4 }) })) + (v1615 * ((v1673 * v5516) + (v1600 * (v1672 * v5717)))));
        let v5743: f64 = (v309 * ((v1674 * (if v1610 { (v1611 * v5535) } else { (if v1606 { (v1607 * v5535) } else { v4 }) })) + (v1615 * ((v1673 * v5514) + (v1600 * v5721)))));
        let v5758: f64 = ((v1232 * v3324) + (v827 * v4524));
        let v5759: f64 = (v1232 * v3325);
        let v5760: f64 = (v1232 * v3326);
        let v5761: f64 = (v1232 * v3327);
        let v5762: f64 = (v1232 * v3328);
        let v5763: f64 = (v452 * (if v894 { (v895 * v3436) } else { (if v891 { (v892 * v3436) } else { v4 }) }));
        let v5764: f64 = (v452 * (if v894 { (v895 * v3273) } else { (if v891 { (v892 * v3273) } else { v4 }) }));
        let v5765: f64 = (v452 * (if v894 { (v895 * v3307) } else { (if v891 { (v892 * v3307) } else { v4 }) }));
        let v5766: f64 = (v452 * (if v894 { (v895 * v3308) } else { (if v891 { (v892 * v3308) } else { v4 }) }));
        let v5767: f64 = (v452 * (if v894 { (v895 * v3274) } else { (if v891 { (v892 * v3274) } else { v4 }) }));
        let v5769: f64 = (v32 * v1685);
        let v5778: f64 = (v1686 * v1686);
        let v5796: f64 = (v32 * v1689);
        let v5805: f64 = (v1690 * v1690);
        let v5823: f64 = (v32 * v3084);
        let v5836: f64 = (((v476 * (v452 * v3084)) - (v1695 * (self.scalar_v471 * (v475 * (self.scalar_v473 * v2714))))) / (v476 * v476));
        let v5844: f64 = (v32 * v1699);
        let v5853: f64 = (v1700 * v1700);
        let v5854: f64 = (((v1700 * ((v1693 * v5823) + (v1692 * v3324))) - (v1694 * (((v1696 * v3324) + (v827 * v5836)) / v5844))) / v5853);
        let v5858: f64 = (((v1700 * (v1692 * v3325)) - (v1694 * ((v1696 * v3325) / v5844))) / v5853);
        let v5862: f64 = (((v1700 * (v1692 * v3326)) - (v1694 * ((v1696 * v3326) / v5844))) / v5853);
        let v5866: f64 = (((v1700 * (v1692 * v3327)) - (v1694 * ((v1696 * v3327) / v5844))) / v5853);
        let v5870: f64 = (((v1700 * (v1692 * v3328)) - (v1694 * ((v1696 * v3328) / v5844))) / v5853);
        let v5871: f64 = (self.scalar_v1705 * v3207);
        let v5879: f64 = (v1706 * v3285);
        let v5881: f64 = (v1706 * v3286);
        let v5885: f64 = (v662 * v662);
        let v5887: f64 = (v452 * (((v662 * v3207) - (v649 * v3216)) / v5885));
        let v5896: f64 = (v1710 * v3285);
        let v5898: f64 = (v1710 * v3286);
        let v5899: f64 = (v32 * v1716);
        let v5908: f64 = (v1717 * v1717);
        let v5912: f64 = ((v1717 * ((v1707 * v5871) + (v1706 * (v3284 - v3375)))) - (v1708 * (((v1713 * v5887) + (v1710 * (v3284 + (self.scalar_v1711 * v3375)))) / v5899)));
        let v5931: f64 = (self.scalar_v1721 * v3207);
        let v5940: f64 = (v1722 * v3325);
        let v5941: f64 = (v1722 * v3326);
        let v5943: f64 = (v1722 * v3327);
        let v5956: f64 = (v1710 * v3325);
        let v5957: f64 = (v1710 * v3326);
        let v5959: f64 = (v1710 * v3327);
        let v5961: f64 = (v32 * v1729);
        let v5972: f64 = (v1730 * v1730);
        let v5976: f64 = ((v1730 * ((v1723 * v5931) + (v1722 * (v3324 - v3408)))) - (v1724 * (((v1726 * v5887) + (v1710 * (v3324 + (self.scalar_v1711 * v3408)))) / v5961)));
        let v6011: f64 = (v32 * v1738);
        let v6018: f64 = (v1739 * v1739);
        let v6028: f64 = (if self.scalar_v1733 { v4 } else { (if self.scalar_v1703 { (((v1717 * (v1706 * (-v3374))) - (v1708 * ((v1710 * (self.scalar_v1711 * v3374)) / v5899))) / v5908) } else { v4 }) });
        let v6029: f64 = (if self.scalar_v1733 { (((v1739 * ((v1734 * v5871) + (v1706 * v3284))) - (v1735 * (((v1710 * v3284) + (v806 * v5887)) / v6011))) / v6018) } else { (if self.scalar_v1703 { (v5912 / v5908) } else { v4 }) });
        let v6030: f64 = (if self.scalar_v1733 { (((v1739 * v5879) - (v1735 * (v5896 / v6011))) / v6018) } else { (if self.scalar_v1703 { (((v1717 * v5879) - (v1708 * (v5896 / v5899))) / v5908) } else { v4 }) });
        let v6031: f64 = (if self.scalar_v1733 { v4 } else { (if self.scalar_v1703 { (((v1717 * (v1706 * (-v3376))) - (v1708 * ((v1710 * (self.scalar_v1711 * v3376)) / v5899))) / v5908) } else { v4 }) });
        let v6032: f64 = (if self.scalar_v1733 { (((v1739 * v5881) - (v1735 * (v5898 / v6011))) / v6018) } else { (if self.scalar_v1703 { (((v1717 * v5881) - (v1708 * (v5898 / v5899))) / v5908) } else { v4 }) });
        let v6041: f64 = (v32 * v1745);
        let v6050: f64 = (v1746 * v1746);
        let v6063: f64 = (((v1746 * v5943) - (v1742 * (v5959 / v6041))) / v6050);
        let v6068: f64 = (if self.scalar_v1733 { v4 } else { (if self.scalar_v1703 { (((v1730 * (v1722 * (-v3407))) - (v1724 * ((v1710 * (self.scalar_v1711 * v3407)) / v5961))) / v5972) } else { v4 }) });
        let v6069: f64 = (if self.scalar_v1733 { (((v1746 * ((v1722 * v3324) + (v1693 * v5931))) - (v1742 * (((v1710 * v3324) + (v827 * v5887)) / v6041))) / v6050) } else { (if self.scalar_v1703 { (v5976 / v5972) } else { v4 }) });
        let v6070: f64 = (if self.scalar_v1733 { (((v1746 * v5940) - (v1742 * (v5956 / v6041))) / v6050) } else { (if self.scalar_v1703 { (((v1730 * v5940) - (v1724 * (v5956 / v5961))) / v5972) } else { v4 }) });
        let v6071: f64 = (if self.scalar_v1733 { (((v1746 * v5941) - (v1742 * (v5957 / v6041))) / v6050) } else { (if self.scalar_v1703 { (((v1730 * v5941) - (v1724 * (v5957 / v5961))) / v5972) } else { v4 }) });
        let v6072: f64 = (if self.scalar_v1733 { v6063 } else { (if self.scalar_v1703 { (((v1730 * (v1722 * (v3327 - v3409))) - (v1724 * ((v1710 * (v3327 + (self.scalar_v1711 * v3409))) / v5961))) / v5972) } else { v4 }) });
        let v6073: f64 = (if self.scalar_v1733 { v6063 } else { (if self.scalar_v1703 { (((v1730 * v5943) - (v1724 * (v5959 / v5961))) / v5972) } else { v4 }) });
        let v6074: f64 = (if self.scalar_v1733 { (((v1746 * (v1722 * v3328)) - (v1742 * ((v1710 * v3328) / v6041))) / v6050) } else { (if self.scalar_v1703 { (((v1730 * (v1722 * (v3328 - v3410))) - (v1724 * ((v1710 * (v3328 + (self.scalar_v1711 * v3410))) / v5961))) / v5972) } else { v4 }) });
        let v6090: f64 = ((v1754 * v3375) + (v857 * (self.scalar_v1752 * (((v667 * v3213) - (v657 * (self.scalar_v663 * (v666 * (self.scalar_v664 * v2714))))) / (v667 * v667)))));
        let v6092: f64 = (v32 * v1757);
        let v6099: f64 = (v1758 * v1758);
        let v6104: f64 = (((v1758 * ((v1750 * (v32 * v3213)) + (v1749 * v3375))) - (v1751 * (v6090 / v6092))) / v6099);
        let v6111: f64 = ((((v1758 * (v1749 * v3374)) - (v1751 * ((v1754 * v3374) / v6092))) / v6099) + self.scalar_v6109);
        let v6112: f64 = ((((v1758 * (v1749 * v3376)) - (v1751 * ((v1754 * v3376) / v6092))) / v6099) + self.scalar_v6110);
        let v6130: f64 = (if self.scalar_v1765 { (self.scalar_v14 * v6068) } else { v6068 });
        let v6131: f64 = (if self.scalar_v1765 { (self.scalar_v14 * v6069) } else { v6069 });
        let v6132: f64 = (if self.scalar_v1765 { (self.scalar_v14 * v6070) } else { v6070 });
        let v6133: f64 = (if self.scalar_v1765 { (self.scalar_v14 * v6071) } else { v6071 });
        let v6134: f64 = (if self.scalar_v1765 { (self.scalar_v14 * v6072) } else { v6072 });
        let v6135: f64 = (if self.scalar_v1765 { (self.scalar_v14 * v6073) } else { v6073 });
        let v6136: f64 = (if self.scalar_v1765 { (self.scalar_v14 * v6074) } else { v6074 });
        let v6137: f64 = (self.scalar_v1770 * v3084);
        let v6152: f64 = (v32 * v1776);
        let v6161: f64 = (v1777 * v1777);
        let v6179: f64 = (if self.scalar_v1765 { (((v1777 * (v1771 * v3359)) - (v1773 * ((v1696 * v3359) / v6152))) / v6161) } else { v4 });
        let v6180: f64 = (if self.scalar_v1765 { (((v1777 * (v1771 * v3360)) - (v1773 * ((v1696 * v3360) / v6152))) / v6161) } else { v4 });
        let v6181: f64 = (if self.scalar_v1765 { (((v1777 * ((v1772 * v6137) + (v1771 * v3361))) - (v1773 * (((v1696 * v3361) + (v847 * v5836)) / v6152))) / v6161) } else { v4 });
        let v6182: f64 = (if self.scalar_v1765 { (((v1777 * (v1771 * v3362)) - (v1773 * ((v1696 * v3362) / v6152))) / v6161) } else { v4 });
        let v6183: f64 = (if self.scalar_v1765 { (((v1777 * (v1771 * v3363)) - (v1773 * ((v1696 * v3363) / v6152))) / v6161) } else { v4 });
        let v6184: f64 = (self.scalar_v1782 * v3207);
        let v6189: f64 = (v1783 * v3359);
        let v6190: f64 = (v1783 * v3360);
        let v6196: f64 = (v1783 * v3362);
        let v6202: f64 = (((v662 * (v452 * v3207)) - (v1786 * v3216)) / v5885);
        let v6210: f64 = (v1787 * v3359);
        let v6211: f64 = (v1787 * v3360);
        let v6217: f64 = (v1787 * v3362);
        let v6219: f64 = (v32 * v1792);
        let v6230: f64 = (v1793 * v1793);
        let v6242: f64 = ((v1793 * ((v1784 * v6184) + (v1783 * (v3361 - v3391)))) - (v1785 * (((v1789 * v6202) + (v1787 * (v3361 + (self.scalar_v1711 * v3391)))) / v6219)));
        let v6271: f64 = (v32 * v1800);
        let v6280: f64 = (v1801 * v1801);
        let v6293: f64 = (((v1801 * v6196) - (v1797 * (v6217 / v6271))) / v6280);
        let v6298: f64 = (if self.scalar_v1796 { (((v1801 * v6189) - (v1797 * (v6210 / v6271))) / v6280) } else { (if self.scalar_v1780 { (((v1793 * v6189) - (v1785 * (v6210 / v6219))) / v6230) } else { v4 }) });
        let v6299: f64 = (if self.scalar_v1796 { (((v1801 * v6190) - (v1797 * (v6211 / v6271))) / v6280) } else { (if self.scalar_v1780 { (((v1793 * v6190) - (v1785 * (v6211 / v6219))) / v6230) } else { v4 }) });
        let v6300: f64 = (if self.scalar_v1796 { v4 } else { (if self.scalar_v1780 { (((v1793 * (v1783 * (-v3390))) - (v1785 * ((v1787 * (self.scalar_v1711 * v3390)) / v6219))) / v6230) } else { v4 }) });
        let v6301: f64 = (if self.scalar_v1796 { (((v1801 * ((v1783 * v3361) + (v1772 * v6184))) - (v1797 * (((v1787 * v3361) + (v847 * v6202)) / v6271))) / v6280) } else { (if self.scalar_v1780 { (v6242 / v6230) } else { v4 }) });
        let v6302: f64 = (if self.scalar_v1796 { v6293 } else { (if self.scalar_v1780 { (((v1793 * (v1783 * (v3362 - v3392))) - (v1785 * ((v1787 * (v3362 + (self.scalar_v1711 * v3392))) / v6219))) / v6230) } else { v4 }) });
        let v6303: f64 = (if self.scalar_v1796 { v6293 } else { (if self.scalar_v1780 { (((v1793 * v6196) - (v1785 * (v6217 / v6219))) / v6230) } else { v4 }) });
        let v6304: f64 = (if self.scalar_v1796 { (((v1801 * (v1783 * v3363)) - (v1797 * ((v1787 * v3363) / v6271))) / v6280) } else { (if self.scalar_v1780 { (((v1793 * (v1783 * (v3363 - v3393))) - (v1785 * ((v1787 * (v3363 + (self.scalar_v1711 * v3393))) / v6219))) / v6230) } else { v4 }) });
        let v6310: f64 = (if self.scalar_v1805 { ((v1807 * v2947) + (v356 * (self.scalar_v13 * (v3084 + v3207)))) } else { v4 });
        let v6323: f64 = (if self.scalar_v1805 { (-(if self.scalar_v1805 { ((v1812 * v2710) + (v120 * (-(((v1809 * v2713) + (v122 * v6310)) / v1810)))) } else { v4 })) } else { v4 });
        let v6326: f64 = (v1816 * self.scalar_v6321);
        let v6327: f64 = (v6326 + v6326);
        let v6328: f64 = (v1816 * self.scalar_v6322);
        let v6330: f64 = (v1816 * v6323);
        let v6332: f64 = (v1816 * self.scalar_v6324);
        let v6333: f64 = (v6332 + v6332);
        let v6334: f64 = (v1816 * self.scalar_v6325);
        let v6336: f64 = (if self.scalar_v1805 { v6327 } else { v4 });
        let v6337: f64 = (if self.scalar_v1805 { (v6328 + v6328) } else { v4 });
        let v6338: f64 = (if self.scalar_v1805 { (v6330 + v6330) } else { v4680 });
        let v6339: f64 = (if self.scalar_v1805 { v4 } else { v4682 });
        let v6340: f64 = (if self.scalar_v1805 { v6327 } else { v4684 });
        let v6341: f64 = (if self.scalar_v1805 { v6333 } else { v4686 });
        let v6342: f64 = (if self.scalar_v1805 { v6333 } else { v4688 });
        let v6343: f64 = (if self.scalar_v1805 { (v6334 + v6334) } else { v4 });
        let v6344: f64 = (if self.scalar_v1805 { v6333 } else { v4 });
        let v6345: f64 = (v32 * v1825);
        let v6346: f64 = (v6336 / v6345);
        let v6347: f64 = (v6337 / v6345);
        let v6348: f64 = (v6338 / v6345);
        let v6349: f64 = (v6339 / v6345);
        let v6350: f64 = (v6340 / v6345);
        let v6351: f64 = (v6341 / v6345);
        let v6352: f64 = (v6342 / v6345);
        let v6353: f64 = (v6343 / v6345);
        let v6354: f64 = (v6344 / v6345);
        let v6365: f64 = (v1826 * v1826);
        let v6417: f64 = (if v1830 { (v440 * (self.scalar_v6321 + v6346)) } else { (if v1822 { ((-(self.scalar_v1823 * (v6346 - self.scalar_v6321))) / v6365) } else { v4 }) });
        let v6418: f64 = (if v1830 { (v440 * (self.scalar_v6322 + v6347)) } else { (if v1822 { ((-(self.scalar_v1823 * (v6347 - self.scalar_v6322))) / v6365) } else { v4 }) });
        let v6419: f64 = (if v1830 { (v440 * (v6323 + v6348)) } else { (if v1822 { ((-(self.scalar_v1823 * (v6348 - v6323))) / v6365) } else { v4 }) });
        let v6420: f64 = (if v1830 { (v440 * v6349) } else { (if v1822 { ((-(self.scalar_v1823 * v6349)) / v6365) } else { v4 }) });
        let v6421: f64 = (if v1830 { (v440 * (self.scalar_v6321 + v6350)) } else { (if v1822 { ((-(self.scalar_v1823 * (v6350 - self.scalar_v6321))) / v6365) } else { v4 }) });
        let v6422: f64 = (if v1830 { (v440 * (self.scalar_v6324 + v6351)) } else { (if v1822 { ((-(self.scalar_v1823 * (v6351 - self.scalar_v6324))) / v6365) } else { v4 }) });
        let v6423: f64 = (if v1830 { (v440 * (self.scalar_v6324 + v6352)) } else { (if v1822 { ((-(self.scalar_v1823 * (v6352 - self.scalar_v6324))) / v6365) } else { v4 }) });
        let v6424: f64 = (if v1830 { (v440 * (self.scalar_v6325 + v6353)) } else { (if v1822 { ((-(self.scalar_v1823 * (v6353 - self.scalar_v6325))) / v6365) } else { v4 }) });
        let v6425: f64 = (if v1830 { (v440 * (self.scalar_v6324 + v6354)) } else { (if v1822 { ((-(self.scalar_v1823 * (v6354 - self.scalar_v6324))) / v6365) } else { v4 }) });
        let v6432: f64 = (v356 * (v6179 + v6298));
        let v6438: f64 = (v356 * (v6182 + v6302));
        let v6453: f64 = (v1837 * v1837);
        let v6500: f64 = (if self.scalar_v1841 { v4 } else { (if self.scalar_v1805 { (((v1837 * v6417) - (v1833 * (v6417 + v6432))) / v6453) } else { v4 }) });
        let v6501: f64 = (if self.scalar_v1841 { v4 } else { (if self.scalar_v1805 { (((v1837 * v6418) - (v1833 * (v6418 + (v356 * (v6180 + v6299))))) / v6453) } else { v4 }) });
        let v6502: f64 = (if self.scalar_v1841 { v4 } else { (if self.scalar_v1805 { ((-(v1833 * (v356 * v6300))) / v6453) } else { v4 }) });
        let v6503: f64 = (if self.scalar_v1841 { v4 } else { (if self.scalar_v1805 { (((v1837 * v6419) - (v1833 * (v6419 + (v6310 + ((v1834 * v2947) + (v356 * (v6181 + v6301))))))) / v6453) } else { v4 }) });
        let v6504: f64 = (if self.scalar_v1841 { v4 } else { (if self.scalar_v1805 { (((v1837 * v6420) - (v1833 * v6420)) / v6453) } else { v4 }) });
        let v6505: f64 = (if self.scalar_v1841 { v4 } else { (if self.scalar_v1805 { (((v1837 * v6421) - (v1833 * (v6421 + v6432))) / v6453) } else { v4 }) });
        let v6506: f64 = (if self.scalar_v1841 { v4 } else { (if self.scalar_v1805 { (((v1837 * v6422) - (v1833 * (v6422 + v6438))) / v6453) } else { v4 }) });
        let v6507: f64 = (if self.scalar_v1841 { v4 } else { (if self.scalar_v1805 { (((v1837 * v6423) - (v1833 * (v6423 + (v356 * (v6182 + v6303))))) / v6453) } else { v4 }) });
        let v6508: f64 = (if self.scalar_v1841 { v4 } else { (if self.scalar_v1805 { (((v1837 * v6424) - (v1833 * (v6424 + (v356 * (v6183 + v6304))))) / v6453) } else { v4 }) });
        let v6509: f64 = (if self.scalar_v1841 { v4 } else { (if self.scalar_v1805 { (((v1837 * v6425) - (v1833 * (v6425 + v6438))) / v6453) } else { v4 }) });
        let v6510: f64 = (v1842 * v6179);
        let v6523: f64 = (v1842 * v6182);
        let v6543: f64 = (v1842 * v6298);
        let v6558: f64 = (v1842 * v6302);
        let v6569: f64 = (if self.scalar_v1765 { (v6543 + (v1803 * v6500)) } else { v4 });
        let v6570: f64 = (if self.scalar_v1765 { ((v1842 * v6299) + (v1803 * v6501)) } else { v4 });
        let v6571: f64 = (if self.scalar_v1765 { ((v1842 * v6300) + (v1803 * v6502)) } else { v4 });
        let v6572: f64 = (if self.scalar_v1765 { ((v1842 * v6301) + (v1803 * v6503)) } else { v4 });
        let v6573: f64 = (if self.scalar_v1765 { (v1803 * v6504) } else { v4 });
        let v6574: f64 = (if self.scalar_v1765 { (v6543 + (v1803 * v6505)) } else { v4 });
        let v6575: f64 = (if self.scalar_v1765 { (v6558 + (v1803 * v6506)) } else { v4 });
        let v6576: f64 = (if self.scalar_v1765 { ((v1842 * v6303) + (v1803 * v6507)) } else { v4 });
        let v6577: f64 = (if self.scalar_v1765 { ((v1842 * v6304) + (v1803 * v6508)) } else { v4 });
        let v6578: f64 = (if self.scalar_v1765 { (v6558 + (v1803 * v6509)) } else { v4 });
        let v6585: f64 = (v1850 * self.scalar_v6579);
        let v6587: f64 = (v1850 * self.scalar_v6580);
        let v6589: f64 = (v1850 * self.scalar_v6581);
        let v6601: f64 = (v32 * v1859);
        let v6602: f64 = ((if self.scalar_v1848 { v4 } else { v6336 }) / v6601);
        let v6603: f64 = ((if self.scalar_v1848 { v4 } else { v6337 }) / v6601);
        let v6604: f64 = ((if self.scalar_v1848 { v4 } else { v6338 }) / v6601);
        let v6605: f64 = ((if self.scalar_v1848 { v4 } else { v6339 }) / v6601);
        let v6606: f64 = ((if self.scalar_v1848 { (v6585 + v6585) } else { v6336 }) / v6601);
        let v6607: f64 = ((if self.scalar_v1848 { (v6587 + v6587) } else { v6340 }) / v6601);
        let v6608: f64 = ((if self.scalar_v1848 { (v6589 + v6589) } else { v6341 }) / v6601);
        let v6609: f64 = ((if self.scalar_v1848 { v4 } else { v6342 }) / v6601);
        let v6610: f64 = ((if self.scalar_v1848 { v4 } else { v6343 }) / v6601);
        let v6611: f64 = ((if self.scalar_v1848 { v4 } else { v6344 }) / v6601);
        let v6617: f64 = (v1860 * v1860);
        let v6669: f64 = (if v1864 { (v440 * v6602) } else { (if v1856 { ((-(self.scalar_v1857 * v6602)) / v6617) } else { v4 }) });
        let v6670: f64 = (if v1864 { (v440 * v6603) } else { (if v1856 { ((-(self.scalar_v1857 * v6603)) / v6617) } else { v4 }) });
        let v6671: f64 = (if v1864 { (v440 * v6604) } else { (if v1856 { ((-(self.scalar_v1857 * v6604)) / v6617) } else { v4 }) });
        let v6672: f64 = (if v1864 { (v440 * v6605) } else { (if v1856 { ((-(self.scalar_v1857 * v6605)) / v6617) } else { v4 }) });
        let v6673: f64 = (if v1864 { (v440 * (self.scalar_v6582 + v6606)) } else { (if v1856 { ((-(self.scalar_v1857 * (v6606 - self.scalar_v6582))) / v6617) } else { v4 }) });
        let v6674: f64 = (if v1864 { (v440 * (self.scalar_v6583 + v6607)) } else { (if v1856 { ((-(self.scalar_v1857 * (v6607 - self.scalar_v6583))) / v6617) } else { v4 }) });
        let v6675: f64 = (if v1864 { (v440 * (self.scalar_v6584 + v6608)) } else { (if v1856 { ((-(self.scalar_v1857 * (v6608 - self.scalar_v6584))) / v6617) } else { v4 }) });
        let v6676: f64 = (if v1864 { (v440 * v6609) } else { (if v1856 { ((-(self.scalar_v1857 * v6609)) / v6617) } else { v4 }) });
        let v6677: f64 = (if v1864 { (v440 * v6610) } else { (if v1856 { ((-(self.scalar_v1857 * v6610)) / v6617) } else { v4 }) });
        let v6678: f64 = (if v1864 { (v440 * v6611) } else { (if v1856 { ((-(self.scalar_v1857 * v6611)) / v6617) } else { v4 }) });
        let v6690: f64 = (self.scalar_v1868 * f64::powf(v1885, self.scalar_v1877));
        let v6701: f64 = (v1887 * v1887);
        let v6742: f64 = (if self.scalar_v1896 { v4 } else { (if v1891 { (self.scalar_v1882 * v6669) } else { (if v1884 { (((v6669 / self.scalar_v1873) * v6690) / v6701) } else { v4 }) }) });
        let v6743: f64 = (if self.scalar_v1896 { v4 } else { (if v1891 { (self.scalar_v1882 * v6670) } else { (if v1884 { (((v6670 / self.scalar_v1873) * v6690) / v6701) } else { v4 }) }) });
        let v6744: f64 = (if self.scalar_v1896 { v4 } else { (if v1891 { (self.scalar_v1882 * v6671) } else { (if v1884 { (((v6671 / self.scalar_v1873) * v6690) / v6701) } else { v4 }) }) });
        let v6745: f64 = (if self.scalar_v1896 { v4 } else { (if v1891 { (self.scalar_v1882 * v6672) } else { (if v1884 { (((v6672 / self.scalar_v1873) * v6690) / v6701) } else { v4 }) }) });
        let v6746: f64 = (if self.scalar_v1896 { v4 } else { (if v1891 { (self.scalar_v1882 * v6673) } else { (if v1884 { (((v6673 / self.scalar_v1873) * v6690) / v6701) } else { v4 }) }) });
        let v6747: f64 = (if self.scalar_v1896 { v4 } else { (if v1891 { (self.scalar_v1882 * v6674) } else { (if v1884 { (((v6674 / self.scalar_v1873) * v6690) / v6701) } else { v4 }) }) });
        let v6748: f64 = (if self.scalar_v1896 { v4 } else { (if v1891 { (self.scalar_v1882 * v6675) } else { (if v1884 { (((v6675 / self.scalar_v1873) * v6690) / v6701) } else { v4 }) }) });
        let v6749: f64 = (if self.scalar_v1896 { v4 } else { (if v1891 { (self.scalar_v1882 * v6676) } else { (if v1884 { (((v6676 / self.scalar_v1873) * v6690) / v6701) } else { v4 }) }) });
        let v6750: f64 = (if self.scalar_v1896 { v4 } else { (if v1891 { (self.scalar_v1882 * v6677) } else { (if v1884 { (((v6677 / self.scalar_v1873) * v6690) / v6701) } else { v4 }) }) });
        let v6751: f64 = (if self.scalar_v1896 { v4 } else { (if v1891 { (self.scalar_v1882 * v6678) } else { (if v1884 { (((v6678 / self.scalar_v1873) * v6690) / v6701) } else { v4 }) }) });
        let v6752: f64 = (v1680 * v6742);
        let v6753: f64 = (v1680 * v6743);
        let v6756: f64 = ((v1897 * (if v1679 { v4 } else { (if v1596 { (self.scalar_v71 * ((v1675 * v2903) + v5743)) } else { v4 }) })) + (v1680 * v6744));
        let v6757: f64 = (v1680 * v6745);
        let v6758: f64 = (v1680 * v6746);
        let v6761: f64 = ((v1897 * (if v1679 { v4 } else { (if v1596 { (self.scalar_v71 * (v309 * v5738)) } else { v4 }) })) + (v1680 * v6747));
        let v6764: f64 = ((v1897 * (if v1679 { v4 } else { (if v1596 { (self.scalar_v71 * (v309 * v5741)) } else { v4 }) })) + (v1680 * v6748));
        let v6765: f64 = (v1680 * v6749);
        let v6766: f64 = (v1680 * v6750);
        let v6767: f64 = (v1680 * v6751);
        let v6776: f64 = ((v1897 * (if self.scalar_v1765 { (self.scalar_v14 * v5858) } else { v5858 })) + (v1767 * v6746));
        let v6779: f64 = ((v1897 * (if self.scalar_v1765 { (self.scalar_v14 * v5862) } else { v5862 })) + (v1767 * v6747));
        let v6780: f64 = (v1897 * (if self.scalar_v1765 { (self.scalar_v14 * v5866) } else { v5866 }));
        let v6782: f64 = (v6780 + (v1767 * v6748));
        let v6784: f64 = (v6780 + (v1767 * v6749));
        let v6788: f64 = ((v1897 * (if self.scalar_v1765 { (self.scalar_v14 * v5870) } else { v5870 })) + (v1767 * v6751));
        let v6799: f64 = ((v1897 * (v502 * v5218)) + (v1488 * v6746));
        let v6802: f64 = ((v1897 * (v502 * v5219)) + (v1488 * v6747));
        let v6803: f64 = (v1897 * (v502 * v5220));
        let v6805: f64 = (v6803 + (v1488 * v6748));
        let v6807: f64 = (v6803 + (v1488 * v6749));
        let v6811: f64 = ((v1897 * (v502 * v5221)) + (v1488 * v6751));
        let v6812: f64 = (v1897 * (if self.scalar_v1765 { (v6510 + (v1779 * v6500)) } else { v4 }));
        let v6814: f64 = (v6812 + (v1844 * v6742));
        let v6817: f64 = ((v1897 * (if self.scalar_v1765 { ((v1842 * v6180) + (v1779 * v6501)) } else { v4 })) + (v1844 * v6743));
        let v6818: f64 = (v1897 * (if self.scalar_v1765 { (v1779 * v6502) } else { v4 }));
        let v6821: f64 = ((v1897 * (if self.scalar_v1765 { ((v1842 * v6181) + (v1779 * v6503)) } else { v4 })) + (v1844 * v6744));
        let v6824: f64 = ((v1897 * (if self.scalar_v1765 { (v1779 * v6504) } else { v4 })) + (v1844 * v6745));
        let v6826: f64 = (v6812 + (v1844 * v6746));
        let v6829: f64 = ((v1897 * (if self.scalar_v1765 { (v6510 + (v1779 * v6505)) } else { v4 })) + (v1844 * v6747));
        let v6832: f64 = ((v1897 * (if self.scalar_v1765 { (v6523 + (v1779 * v6506)) } else { v4 })) + (v1844 * v6748));
        let v6835: f64 = ((v1897 * (if self.scalar_v1765 { (v6523 + (v1779 * v6507)) } else { v4 })) + (v1844 * v6749));
        let v6838: f64 = ((v1897 * (if self.scalar_v1765 { ((v1842 * v6183) + (v1779 * v6508)) } else { v4 })) + (v1844 * v6750));
        let v6841: f64 = ((v1897 * (if self.scalar_v1765 { (v6523 + (v1779 * v6509)) } else { v4 })) + (v1844 * v6751));
        let v6842: f64 = (v1249 * v4603);
        let v6844: f64 = (v1249 * v4593);
        let v6846: f64 = (v1249 * v4604);
        let v6848: f64 = (v1249 * v4601);
        let v6850: f64 = (v1249 * v4602);
        let v6852: f64 = (v32 * v1905);
        let v6853: f64 = ((v6842 + v6842) / v6852);
        let v6854: f64 = ((v6844 + v6844) / v6852);
        let v6855: f64 = ((v6846 + v6846) / v6852);
        let v6856: f64 = ((v6848 + v6848) / v6852);
        let v6857: f64 = ((v6850 + v6850) / v6852);
        let v6865: f64 = (v1906 * v1906);
        let v6894: f64 = (if v1909 { (v440 * (v4603 + v6853)) } else { (if v1903 { ((-(v1271 * (v6853 - v4603))) / v6865) } else { v4 }) });
        let v6895: f64 = (if v1909 { (v440 * (v4593 + v6854)) } else { (if v1903 { ((-(v1271 * (v6854 - v4593))) / v6865) } else { v4 }) });
        let v6896: f64 = (if v1909 { (v440 * (v4604 + v6855)) } else { (if v1903 { ((-(v1271 * (v6855 - v4604))) / v6865) } else { v4 }) });
        let v6897: f64 = (if v1909 { (v440 * (v4601 + v6856)) } else { (if v1903 { ((-(v1271 * (v6856 - v4601))) / v6865) } else { v4 }) });
        let v6898: f64 = (if v1909 { (v440 * (v4602 + v6857)) } else { (if v1903 { ((-(v1271 * (v6857 - v4602))) / v6865) } else { v4 }) });
        let v6917: f64 = (v1913 * v1913);
        let v6936: f64 = (v171 * (if v1915 { v4 } else { (((v1913 * (self.scalar_v338 * (v343 * (self.scalar_v341 * v2714)))) - (v344 * ((v1912 * v4738) + (v1283 * v6894)))) / v6917) }));
        let v6937: f64 = (v171 * (if v1915 { v4 } else { ((-(v344 * ((v1912 * v4739) + (v1283 * v6895)))) / v6917) }));
        let v6938: f64 = (v171 * (if v1915 { v4 } else { ((-(v344 * ((v1912 * v4740) + (v1283 * v6896)))) / v6917) }));
        let v6939: f64 = (v171 * (if v1915 { v4 } else { ((-(v344 * ((v1912 * v4741) + (v1283 * v6897)))) / v6917) }));
        let v6940: f64 = (v171 * (if v1915 { v4 } else { ((-(v344 * ((v1912 * v4742) + (v1283 * v6898)))) / v6917) }));
        let v6951: f64 = (v1917 * v1917);
        let v6952: f64 = (((v1917 * ((v1918 * v3563) + (v954 * (if v832 { (v833 * v3329) } else { (if v829 { (v830 * v3329) } else { v4 }) })))) - (v1920 * v6936)) / v6951);
        let v6955: f64 = ((-(v1920 * v6937)) / v6951);
        let v6956: f64 = ((self.scalar_v0 + (v954 * (if v832 { (v833 * v3273) } else { (if v829 { (v830 * v3273) } else { v4 }) }))) / v1917);
        let v6960: f64 = (((v1917 * (self.scalar_v3268 + (v954 * (if v832 { (v833 * v3274) } else { (if v829 { (v830 * v3274) } else { v4 }) })))) - (v1920 * v6938)) / v6951);
        let v6963: f64 = ((-(v1920 * v6939)) / v6951);
        let v6966: f64 = ((-(v1920 * v6940)) / v6951);
        let v6972: f64 = ((-v4778) / self.scalar_v1928);
        let v6973: f64 = ((-v4782) / self.scalar_v1928);
        let v6974: f64 = ((-v4786) / self.scalar_v1928);
        let v6975: f64 = ((-v4790) / self.scalar_v1928);
        let v6976: f64 = ((-v4794) / self.scalar_v1928);
        let v7006: f64 = (if v1932 { (v1943 * (if v1937 { (v1938 * v6972) } else { (if v1933 { (v1934 * v6972) } else { v4 }) })) } else { v4 });
        let v7007: f64 = (if v1932 { (v1943 * (if v1937 { (v1938 * v6973) } else { (if v1933 { (v1934 * v6973) } else { v4 }) })) } else { v4 });
        let v7008: f64 = (if v1932 { ((v1943 * (if v1937 { (v1938 * v6974) } else { (if v1933 { (v1934 * v6974) } else { v4 }) })) + (v1942 * self.scalar_v3268)) } else { v4 });
        let v7009: f64 = (if v1932 { ((v1943 * (if v1937 { (v1938 * v6975) } else { (if v1933 { (v1934 * v6975) } else { v4 }) })) + (self.scalar_v0 * v1942)) } else { v4 });
        let v7010: f64 = (if v1932 { (v1943 * (if v1937 { (v1938 * v6976) } else { (if v1933 { (v1934 * v6976) } else { v4 }) })) } else { v4 });
        let v7011: f64 = (-v3003);
        let v7014: f64 = (self.scalar_v1947 * f64::powf(v1945, self.scalar_v7012));
        let v7022: f64 = ((v1948 * v7011) + (v1946 * (v7006 * v7014)));
        let v7023: f64 = (v1946 * (v7007 * v7014));
        let v7024: f64 = (v1946 * (v7008 * v7014));
        let v7025: f64 = (v1946 * (v7009 * v7014));
        let v7026: f64 = (v1946 * (v7010 * v7014));
        let v7042: f64 = (if v1955 { (v1956 * v7022) } else { (if v1951 { (v1952 * v7022) } else { v4 }) });
        let v7043: f64 = (if v1955 { (v1956 * v7023) } else { (if v1951 { (v1952 * v7023) } else { v4 }) });
        let v7044: f64 = (if v1955 { (v1956 * v7024) } else { (if v1951 { (v1952 * v7024) } else { v4 }) });
        let v7045: f64 = (if v1955 { (v1956 * v7025) } else { (if v1951 { (v1952 * v7025) } else { v4 }) });
        let v7046: f64 = (if v1955 { (v1956 * v7026) } else { (if v1951 { (v1952 * v7026) } else { v4 }) });
        let v7050: f64 = ((-(self.scalar_v1961 * v3003)) / (v450 * v450));
        let v7081: f64 = (v1146 * v1146);
        let v7094: f64 = (if v1971 { (((v1146 * v2807) - (v1978 * v4246)) / v7081) } else { v3955 });
        let v7095: f64 = (if v1971 { (((v1146 * self.scalar_v3268) - (v1978 * v4247)) / v7081) } else { v3956 });
        let v7096: f64 = (if v1971 { (((self.scalar_v0 * v1146) - (v1978 * v4248)) / v7081) } else { v3957 });
        let v7097: f64 = (if v1971 { ((-(v1978 * v4249)) / v7081) } else { v3958 });
        let v7106: f64 = (v32 * v1983);
        let v7111: f64 = (if v1971 { (((v32 * v7094) / v1977) / v7106) } else { v4 });
        let v7112: f64 = (if v1971 { (((v32 * v7095) / v1977) / v7106) } else { v4 });
        let v7113: f64 = (if v1971 { (((v32 * v7096) / v1977) / v7106) } else { v4 });
        let v7114: f64 = (if v1971 { (((v32 * v7097) / v1977) / v7106) } else { v4 });
        let v7123: f64 = (if v1990 { (-(v440 * v4222)) } else { v4 });
        let v7124: f64 = (if v1990 { (-(v440 * v4223)) } else { v4 });
        let v7125: f64 = (if v1990 { (-(v440 * v4224)) } else { v4 });
        let v7126: f64 = (if v1990 { (-(v440 * v4225)) } else { v4 });
        let v7143: f64 = (if v1990 { ((v1994 * v7123) + (v1993 * (self.scalar_v1974 * v7123))) } else { v4 });
        let v7144: f64 = (if v1990 { ((v1994 * v7124) + (v1993 * (self.scalar_v1974 * v7124))) } else { v4 });
        let v7145: f64 = (if v1990 { ((v1994 * v7125) + (v1993 * (self.scalar_v1974 * v7125))) } else { v4 });
        let v7146: f64 = (if v1990 { ((v1994 * v7126) + (v1993 * (self.scalar_v1974 * v7126))) } else { v4 });
        let v7159: f64 = (v1984 * v7111);
        let v7161: f64 = (v1984 * v7112);
        let v7163: f64 = (v1984 * v7113);
        let v7165: f64 = (v1984 * v7114);
        let v7167: f64 = (v1996 * v7143);
        let v7169: f64 = (v1996 * v7144);
        let v7171: f64 = (v1996 * v7145);
        let v7173: f64 = (v1996 * v7146);
        let v7179: f64 = (v32 * v2001);
        let v7187: f64 = (v2001 * v2001);
        let v7201: f64 = (if v1971 { (((v2001 * ((v1996 * v7111) + (v1984 * v7143))) - (v1997 * (((v7159 + v7159) + (v7167 + v7167)) / v7179))) / v7187) } else { v4 });
        let v7202: f64 = (if v1971 { (((v2001 * ((v1996 * v7112) + (v1984 * v7144))) - (v1997 * (((v7161 + v7161) + (v7169 + v7169)) / v7179))) / v7187) } else { v4 });
        let v7203: f64 = (if v1971 { (((v2001 * ((v1996 * v7113) + (v1984 * v7145))) - (v1997 * (((v7163 + v7163) + (v7171 + v7171)) / v7179))) / v7187) } else { v4 });
        let v7204: f64 = (if v1971 { (((v2001 * ((v1996 * v7114) + (v1984 * v7146))) - (v1997 * (((v7165 + v7165) + (v7173 + v7173)) / v7179))) / v7187) } else { v4 });
        let v7208: f64 = (v2003 * v2003);
        let v7221: f64 = (if v1971 { (((v2003 * v2807) - (v1978 * v7201)) / v7208) } else { v4 });
        let v7222: f64 = (if v1971 { (((v2003 * self.scalar_v3268) - (v1978 * v7202)) / v7208) } else { v4 });
        let v7223: f64 = (if v1971 { (((self.scalar_v0 * v2003) - (v1978 * v7203)) / v7208) } else { v4 });
        let v7224: f64 = (if v1971 { ((-(v1978 * v7204)) / v7208) } else { v4 });
        let v7225: f64 = (v440 * v7201);
        let v7226: f64 = (v440 * v7202);
        let v7227: f64 = (v440 * v7203);
        let v7228: f64 = (v440 * v7204);
        let v7229: f64 = (v1977 * v7225);
        let v7230: f64 = (v1977 * v7226);
        let v7231: f64 = (v1977 * v7227);
        let v7232: f64 = (v1977 * v7228);
        let v7249: f64 = (if v1971 { (v7221 + ((v2007 * v4246) + (v1146 * v7229))) } else { v4 });
        let v7250: f64 = (if v1971 { (v7222 + ((v2007 * v4247) + (v1146 * v7230))) } else { v4 });
        let v7251: f64 = (if v1971 { (v7223 + ((v2007 * v4248) + (v1146 * v7231))) } else { v4 });
        let v7252: f64 = (if v1971 { (v7224 + ((v2007 * v4249) + (v1146 * v7232))) } else { v4 });
        let v7276: f64 = (v2023 * v2023);
        let v7298: f64 = ((v2025 * v7229) + (v2007 * (-(((v2023 * v4778) - (v1290 * (self.scalar_v984 * (if v1990 { (self.scalar_v2013 * (v32 * v4222)) } else { v4 })))) / v7276))));
        let v7302: f64 = ((v2025 * v7230) + (v2007 * (-(((v2023 * v4786) - (v1290 * (self.scalar_v984 * (if v1990 { (self.scalar_v2013 * (v32 * v4223)) } else { v4 })))) / v7276))));
        let v7305: f64 = ((v2025 * v7231) + (v2007 * (-(((v2023 * v4790) - (v1290 * (self.scalar_v984 * (if v1990 { (self.scalar_v2013 * (v32 * v4224)) } else { v4 })))) / v7276))));
        let v7308: f64 = ((v2025 * v7232) + (v2007 * (-(((v2023 * v4794) - (v1290 * (self.scalar_v984 * (if v1990 { (self.scalar_v2013 * (v32 * v4225)) } else { v4 })))) / v7276))));
        let v7314: f64 = (if v1990 { (v7221 - v7298) } else { v4 });
        let v7315: f64 = (if v1990 { (-(v2007 * (-(v4782 / v2023)))) } else { v4 });
        let v7316: f64 = (if v1990 { (v7222 - v7302) } else { v4 });
        let v7317: f64 = (if v1990 { (v7223 - v7305) } else { v4 });
        let v7318: f64 = (if v1990 { (v7224 - v7308) } else { v4 });
        let v7323: f64 = (v2029 * (v7314 - v7249));
        let v7325: f64 = (v2029 * v7315);
        let v7327: f64 = (v2029 * (v7316 - v7250));
        let v7329: f64 = (v2029 * (v7317 - v7251));
        let v7331: f64 = (v2029 * (v7318 - v7252));
        let v7378: f64 = (v32 * v2038);
        let v7379: f64 = ((if v1990 { ((v7323 + v7323) + (((v2032 * v4234) + (v1143 * ((v2031 * v7221) + (v2005 * (v47 * v7221))))) / self.scalar_v984)) } else { v7094 }) / v7378);
        let v7381: f64 = ((if v1990 { ((v7327 + v7327) + (((v2032 * v4235) + (v1143 * ((v2031 * v7222) + (v2005 * (v47 * v7222))))) / self.scalar_v984)) } else { v7095 }) / v7378);
        let v7382: f64 = ((if v1990 { ((v7329 + v7329) + (((v2032 * v4236) + (v1143 * ((v2031 * v7223) + (v2005 * (v47 * v7223))))) / self.scalar_v984)) } else { v7096 }) / v7378);
        let v7383: f64 = ((if v1990 { ((v7331 + v7331) + (((v2032 * v4237) + (v1143 * ((v2031 * v7224) + (v2005 * (v47 * v7224))))) / self.scalar_v984)) } else { v7097 }) / v7378);
        let v7394: f64 = (if v1990 { (v440 * ((v7249 + v7314) + v7379)) } else { (if v1987 { v7249 } else { v4 }) });
        let v7395: f64 = (if v1990 { (v440 * (v7315 + ((if v1990 { (v7325 + v7325) } else { v4 }) / v7378))) } else { v4 });
        let v7396: f64 = (if v1990 { (v440 * ((v7250 + v7316) + v7381)) } else { (if v1987 { v7250 } else { v4 }) });
        let v7397: f64 = (if v1990 { (v440 * ((v7251 + v7317) + v7382)) } else { (if v1987 { v7251 } else { v4 }) });
        let v7398: f64 = (if v1990 { (v440 * ((v7252 + v7318) + v7383)) } else { (if v1987 { v7252 } else { v4 }) });
        let v7406: f64 = (v2041 * v2041);
        let v7432: f64 = (v2044 * v2044);
        let v7449: f64 = (if v2048 { (((v2044 * v7225) - (v2006 * (if v1971 { (((v2041 * (v7394 - v7221)) - (v2042 * v7394)) / v7406) } else { v4 }))) / v7432) } else { v4 });
        let v7450: f64 = (if v2048 { ((-(v2006 * (if v1971 { (((v2041 * v7395) - (v2042 * v7395)) / v7406) } else { v4 }))) / v7432) } else { v4 });
        let v7451: f64 = (if v2048 { (((v2044 * v7226) - (v2006 * (if v1971 { (((v2041 * (v7396 - v7222)) - (v2042 * v7396)) / v7406) } else { v4 }))) / v7432) } else { v4 });
        let v7452: f64 = (if v2048 { (((v2044 * v7227) - (v2006 * (if v1971 { (((v2041 * (v7397 - v7223)) - (v2042 * v7397)) / v7406) } else { v4 }))) / v7432) } else { v4 });
        let v7453: f64 = (if v2048 { (((v2044 * v7228) - (v2006 * (if v1971 { (((v2041 * (v7398 - v7224)) - (v2042 * v7398)) / v7406) } else { v4 }))) / v7432) } else { v4 });
        let v7484: f64 = (((v2041 * (-v3248)) - (v2054 * v7394)) / v7406);
        let v7487: f64 = ((-(v2054 * v7395)) / v7406);
        let v7490: f64 = ((-(v2054 * v7396)) / v7406);
        let v7493: f64 = ((-(v2054 * v7397)) / v7406);
        let v7496: f64 = ((-(v2054 * v7398)) / v7406);
        let v7497: f64 = (v2056 * v7484);
        let v7498: f64 = (v2056 * v7487);
        let v7499: f64 = (v2056 * v7490);
        let v7500: f64 = (v2056 * v7493);
        let v7501: f64 = (v2056 * v7496);
        let v7505: f64 = (v2050 * v2050);
        let v7549: f64 = ((v2061 * ((v2052 * v7449) + (v2050 * ((v2051 * v7394) + (v2041 * ((-(self.scalar_v10 * v3248)) / (v716 * v716))))))) + (v2053 * (v7497 - (v2060 * ((v2058 * v7484) + (v2055 * (((v2050 * v7143) - (v1996 * v7449)) / v7505)))))));
        let v7552: f64 = ((v2061 * ((v2052 * v7450) + (v2050 * (v2051 * v7395)))) + (v2053 * (v7498 - (v2060 * ((v2058 * v7487) + (v2055 * ((-(v1996 * v7450)) / v7505)))))));
        let v7555: f64 = ((v2061 * ((v2052 * v7451) + (v2050 * (v2051 * v7396)))) + (v2053 * (v7499 - (v2060 * ((v2058 * v7490) + (v2055 * (((v2050 * v7144) - (v1996 * v7451)) / v7505)))))));
        let v7558: f64 = ((v2061 * ((v2052 * v7452) + (v2050 * (v2051 * v7397)))) + (v2053 * (v7500 - (v2060 * ((v2058 * v7493) + (v2055 * (((v2050 * v7145) - (v1996 * v7452)) / v7505)))))));
        let v7561: f64 = ((v2061 * ((v2052 * v7453) + (v2050 * (v2051 * v7398)))) + (v2053 * (v7501 - (v2060 * ((v2058 * v7496) + (v2055 * (((v2050 * v7146) - (v1996 * v7453)) / v7505)))))));
        let v7584: f64 = (if v2065 { ((v2066 * v7497) + (v2056 * (self.scalar_v10 * v7143))) } else { (if v2048 { v7549 } else { (if v1932 { ((v1963 * v7042) + (v1960 * ((v1962 * v7006) + (v1945 * v7050)))) } else { v4 }) }) });
        let v7586: f64 = (if v2065 { ((v2066 * v7499) + (v2056 * (self.scalar_v10 * v7144))) } else { (if v2048 { v7555 } else { (if v1932 { ((v1963 * v7044) + (v1960 * (v1962 * v7008))) } else { v4 }) }) });
        let v7587: f64 = (if v2065 { ((v2066 * v7500) + (v2056 * (self.scalar_v10 * v7145))) } else { (if v2048 { v7558 } else { (if v1932 { ((v1963 * v7045) + (v1960 * (v1962 * v7009))) } else { v4 }) }) });
        let v7588: f64 = (if v2065 { ((v2066 * v7501) + (v2056 * (self.scalar_v10 * v7146))) } else { (if v2048 { v7561 } else { (if v1932 { ((v1963 * v7046) + (v1960 * (v1962 * v7010))) } else { v4 }) }) });
        let v7590: f64 = (self.scalar_v1947 * f64::powf(v1943, self.scalar_v7012));
        let v7596: f64 = (v2076 * v2076);
        let v7621: f64 = (self.scalar_v2079 * f64::powf(v2078, self.scalar_v7619));
        let v7636: f64 = (if v2073 { (v2074 * ((-(((v2076 * v4778) - (v1290 * v4778)) / v7596)) * v7621)) } else { v4 });
        let v7637: f64 = (if v2073 { (v2074 * ((-(((v2076 * v4782) - (v1290 * v4782)) / v7596)) * v7621)) } else { v4 });
        let v7638: f64 = (if v2073 { ((v2080 * (self.scalar_v3268 * v7590)) + (v2074 * ((-(((v2076 * v4786) - (v1290 * v4786)) / v7596)) * v7621))) } else { v4 });
        let v7639: f64 = (if v2073 { ((v2080 * (self.scalar_v0 * v7590)) + (v2074 * ((-(((v2076 * v4790) - (v1290 * v4790)) / v7596)) * v7621))) } else { v4 });
        let v7640: f64 = (if v2073 { (v2074 * ((-(((v2076 * v4794) - (v1290 * v4794)) / v7596)) * v7621)) } else { v4 });
        let v7651: f64 = (if v2085 { (v4778 / self.scalar_v2075) } else { v4 });
        let v7652: f64 = (if v2085 { (v4782 / self.scalar_v2075) } else { v4 });
        let v7653: f64 = (if v2085 { (v4786 / self.scalar_v2075) } else { v4 });
        let v7654: f64 = (if v2085 { (v4790 / self.scalar_v2075) } else { v4 });
        let v7655: f64 = (if v2085 { (v4794 / self.scalar_v2075) } else { v4 });
        let v7661: f64 = (if v2085 { (v7651 / self.scalar_v2091) } else { v4 });
        let v7662: f64 = (if v2085 { (v7652 / self.scalar_v2091) } else { self.scalar_v4830 });
        let v7663: f64 = (if v2085 { (v7653 / self.scalar_v2091) } else { self.scalar_v4831 });
        let v7664: f64 = (if v2085 { (v7654 / self.scalar_v2091) } else { v4 });
        let v7665: f64 = (if v2085 { (v7655 / self.scalar_v2091) } else { v4 });
        let v7718: f64 = (self.scalar_v2111 * f64::powf(v2110, self.scalar_v7716));
        let v7725: f64 = (v2082 * ((if v2103 { (v7651 + (self.scalar_v2091 * ((v2105 * (-v7661)) / v2106))) } else { (if v2095 { (self.scalar_v2091 * ((v2096 * v7661) / v2097)) } else { v4 }) }) * v7718));
        let v7728: f64 = (v2082 * ((if v2103 { (v7652 + (self.scalar_v2091 * ((v2105 * (-v7662)) / v2106))) } else { (if v2095 { (self.scalar_v2091 * ((v2096 * v7662) / v2097)) } else { v4 }) }) * v7718));
        let v7731: f64 = (v2082 * ((if v2103 { (v7653 + (self.scalar_v2091 * ((v2105 * (-v7663)) / v2106))) } else { (if v2095 { (self.scalar_v2091 * ((v2096 * v7663) / v2097)) } else { v4 }) }) * v7718));
        let v7734: f64 = (v2082 * ((if v2103 { (v7654 + (self.scalar_v2091 * ((v2105 * (-v7664)) / v2106))) } else { (if v2095 { (self.scalar_v2091 * ((v2096 * v7664) / v2097)) } else { v4 }) }) * v7718));
        let v7737: f64 = (v2082 * ((if v2103 { (v7655 + (self.scalar_v2091 * ((v2105 * (-v7665)) / v2106))) } else { (if v2095 { (self.scalar_v2091 * ((v2096 * v7665) / v2097)) } else { v4 }) }) * v7718));
        let v7746: f64 = ((v2114 * v7011) + (v1946 * (if v2085 { ((v2112 * v7636) + v7725) } else { (if v2083 { v7636 } else { v4 }) })));
        let v7747: f64 = (v1946 * (if v2085 { ((v2112 * v7637) + v7728) } else { (if v2083 { v7637 } else { v4 }) }));
        let v7748: f64 = (v1946 * (if v2085 { ((v2112 * v7638) + v7731) } else { (if v2083 { v7638 } else { v4 }) }));
        let v7749: f64 = (v1946 * (if v2085 { ((v2112 * v7639) + v7734) } else { (if v2083 { v7639 } else { v4 }) }));
        let v7750: f64 = (v1946 * (if v2085 { ((v2112 * v7640) + v7737) } else { (if v2083 { v7640 } else { v4 }) }));
        let v7785: f64 = (if v2073 { ((v2127 * (if v2121 { (v2122 * v7746) } else { (if v2117 { (v2118 * v7746) } else { v7042 }) })) + (v2126 * (v1943 * v7050))) } else { v7584 });
        let v7786: f64 = (if v2073 { (v2127 * (if v2121 { (v2122 * v7747) } else { (if v2117 { (v2118 * v7747) } else { v7043 }) })) } else { (if v2065 { (v2066 * v7498) } else { (if v2048 { v7552 } else { (if v1932 { ((v1963 * v7043) + (v1960 * (v1962 * v7007))) } else { v4 }) }) }) });
        let v7787: f64 = (if v2073 { ((v2127 * (if v2121 { (v2122 * v7748) } else { (if v2117 { (v2118 * v7748) } else { v7044 }) })) + (v2126 * (v1962 * self.scalar_v3268))) } else { v7586 });
        let v7788: f64 = (if v2073 { ((v2127 * (if v2121 { (v2122 * v7749) } else { (if v2117 { (v2118 * v7749) } else { v7045 }) })) + (v2126 * (self.scalar_v0 * v1962))) } else { v7587 });
        let v7789: f64 = (if v2073 { (v2127 * (if v2121 { (v2122 * v7750) } else { (if v2117 { (v2118 * v7750) } else { v7046 }) })) } else { v7588 });
        let v7790: f64 = (v2944 + v6936);
        let v7809: f64 = (v2136 * v2136);
        let v7838: f64 = ((((v2136 * v2710) - (v120 * ((v2135 * v4778) + (v1290 * v7790)))) / v7809) + ((v2138 * v3054) + (v516 * (((v465 * v4745) - (v1284 * v3020)) / v4902))));
        let v7846: f64 = (v2135 * v2135);
        let v7861: f64 = ((((-(v120 * ((v2135 * v4782) + (v1290 * v6937)))) / v7809) + (v516 * (v4748 / v465))) + ((-(v337 * v6937)) / v7846));
        let v7862: f64 = ((((-(v120 * ((v2135 * v4786) + (v1290 * v6938)))) / v7809) + (v516 * (v4751 / v465))) + ((-(v337 * v6938)) / v7846));
        let v7863: f64 = ((((-(v120 * ((v2135 * v4790) + (v1290 * v6939)))) / v7809) + (v516 * (v4754 / v465))) + ((-(v337 * v6939)) / v7846));
        let v7864: f64 = ((((-(v120 * ((v2135 * v4794) + (v1290 * v6940)))) / v7809) + (v516 * (v4757 / v465))) + ((-(v337 * v6940)) / v7846));
        let v7865: f64 = (if v2134 { (v7838 + (((v2135 * v2937) - (v337 * v7790)) / v7846)) } else { v4 });
        let v7866: f64 = (if v2134 { v7861 } else { v4 });
        let v7867: f64 = (if v2134 { v7862 } else { v4 });
        let v7868: f64 = (if v2134 { v7863 } else { v4 });
        let v7869: f64 = (if v2134 { v7864 } else { v4 });
        let v7880: f64 = (if v2144 { ((v7785 - v7865) / v437) } else { v7661 });
        let v7881: f64 = (if v2144 { ((v7786 - v7866) / v437) } else { v7662 });
        let v7882: f64 = (if v2144 { ((v7787 - v7867) / v437) } else { v7663 });
        let v7883: f64 = (if v2144 { ((v7788 - v7868) / v437) } else { v7664 });
        let v7884: f64 = (if v2144 { ((v7789 - v7869) / v437) } else { v7665 });
        let v7935: f64 = (if v2157 { (v7865 - (v437 * ((v2159 * (-v7880)) / v2160))) } else { (if v2149 { (v7785 - (v437 * ((v2150 * v7880) / v2151))) } else { v7785 }) });
        let v7936: f64 = (if v2157 { (v7866 - (v437 * ((v2159 * (-v7881)) / v2160))) } else { (if v2149 { (v7786 - (v437 * ((v2150 * v7881) / v2151))) } else { v7786 }) });
        let v7937: f64 = (if v2157 { (v7867 - (v437 * ((v2159 * (-v7882)) / v2160))) } else { (if v2149 { (v7787 - (v437 * ((v2150 * v7882) / v2151))) } else { v7787 }) });
        let v7938: f64 = (if v2157 { (v7868 - (v437 * ((v2159 * (-v7883)) / v2160))) } else { (if v2149 { (v7788 - (v437 * ((v2150 * v7883) / v2151))) } else { v7788 }) });
        let v7939: f64 = (if v2157 { (v7869 - (v437 * ((v2159 * (-v7884)) / v2160))) } else { (if v2149 { (v7789 - (v437 * ((v2150 * v7884) / v2151))) } else { v7789 }) });
        let v7942: f64 = ((v2164 * v4778) + (v1290 * v7935));
        let v7945: f64 = ((v2164 * v4782) + (v1290 * v7936));
        let v7948: f64 = ((v2164 * v4786) + (v1290 * v7937));
        let v7951: f64 = ((v2164 * v4790) + (v1290 * v7938));
        let v7954: f64 = ((v2164 * v4794) + (v1290 * v7939));
        let v7983: f64 = (v2170 * v2170);
        let v8006: f64 = (if v2174 { v7942 } else { (if v2168 { (((v2170 * ((v2165 * v7865) + (v2143 * v7942))) - (v2169 * (v7865 + v7935))) / v7983) } else { (if v2144 { v7942 } else { v4 }) }) });
        let v8007: f64 = (if v2174 { v7945 } else { (if v2168 { (((v2170 * ((v2165 * v7866) + (v2143 * v7945))) - (v2169 * (v7866 + v7936))) / v7983) } else { (if v2144 { v7945 } else { v4 }) }) });
        let v8008: f64 = (if v2174 { v7948 } else { (if v2168 { (((v2170 * ((v2165 * v7867) + (v2143 * v7948))) - (v2169 * (v7867 + v7937))) / v7983) } else { (if v2144 { v7948 } else { v4 }) }) });
        let v8009: f64 = (if v2174 { v7951 } else { (if v2168 { (((v2170 * ((v2165 * v7868) + (v2143 * v7951))) - (v2169 * (v7868 + v7938))) / v7983) } else { (if v2144 { v7951 } else { v4 }) }) });
        let v8010: f64 = (if v2174 { v7954 } else { (if v2168 { (((v2170 * ((v2165 * v7869) + (v2143 * v7954))) - (v2169 * (v7869 + v7939))) / v7983) } else { (if v2144 { v7954 } else { v4 }) }) });
        let v8025: f64 = (if v2180 { v4 } else { (if v2176 { ((v2177 * v2710) + (v120 * (v4166 / v1116))) } else { v4 }) });
        let v8026: f64 = (if v2180 { self.scalar_v0 } else { (if v2176 { (v120 * (v4167 / v1116)) } else { v4 }) });
        let v8027: f64 = (if v2180 { v4 } else { (if v2176 { (v120 * (v4168 / v1116)) } else { v4 }) });
        let v8028: f64 = (if v2180 { self.scalar_v3268 } else { (if v2176 { (v120 * (v4169 / v1116)) } else { v4 }) });
        let v8086: f64 = ((((v2184 * v4786) + (v1290 * (self.scalar_v0 - v8026))) + ((v2186 * v3554) + (v941 * (v8026 - self.scalar_v0)))) - ((v2181 * v8008) + (v2175 * v8026)));
        let v8087: f64 = ((((v2184 * v4790) + (v1290 * (-v8027))) + ((v2186 * v3555) + (v941 * (v8027 - self.scalar_v3268)))) - ((v2181 * v8009) + (v2175 * v8027)));
        let v8090: f64 = (v770 * self.scalar_v3268);
        let v8095: f64 = (v337 * v337);
        let v8098: f64 = (((((v2184 * v4778) + (v1290 * (-v8025))) + ((v2186 * v3553) + (v941 * v8025))) - ((v2181 * v8006) + (v2175 * v8025))) + ((-(v2191 * v2937)) / v8095));
        let v8101: f64 = (v791 * self.scalar_v3269);
        let v8103: f64 = (v791 * self.scalar_v3270);
        let v8105: f64 = (v791 * self.scalar_v3268);
        let v8108: f64 = (v731 * (v8101 + v8101));
        let v8110: f64 = (v731 * (v8103 + v8103));
        let v8115: f64 = (((((v2184 * v4794) + (v1290 * (-v8028))) + ((v2186 * v3556) + (v941 * v8028))) - ((v2181 * v8010) + (v2175 * v8028))) + v8110);
        let v8117: f64 = (v784 * self.scalar_v3268);
        let v8125: f64 = (v781 * self.scalar_v3268);
        let v8135: f64 = (v773 * self.scalar_v3268);
        let v8140: f64 = (v351 * v351);
        let v8161: f64 = (v5059 + ((v1463 * ((v489 * (self.scalar_v477 * (v483 * (self.scalar_v481 * v2714)))) + (v484 * (v489 * (v3030 / self.scalar_v479))))) + (v490 * v5162)));
        let v8164: f64 = (self.scalar_v6110 + ((if self.scalar_v1407 { (v516 * ((self.scalar_v1408 * v4881) + (v1391 * (self.scalar_v1401 * v4881)))) } else { (if self.scalar_v1404 { v4931 } else { (if self.scalar_v526 { v5012 } else { v4 }) }) }) + (v490 * v5163)));
        let v8169: f64 = (((v1341 * (self.scalar_v1338 * v4852)) + (v1339 * ((-v4852) * v4859))) + (v8164 - (if v1590 { v4 } else { (if v1504 { (self.scalar_v36 * (v308 * v5485)) } else { v4 }) })));
        let v8170: f64 = (((v1341 * (self.scalar_v1338 * v4853)) + (v1339 * ((-v4853) * v4859))) + ((self.scalar_v6109 + (v5061 + (v490 * v5165))) - (if v1590 { v4 } else { (if v1504 { (self.scalar_v36 * (v308 * v5488)) } else { v4 }) })));
        let v8171: f64 = ((v1318 * ((v585 * (self.scalar_v580 * (v2709 / (v32 * v581)))) + (v582 * (v585 * (self.scalar_v583 * v2708))))) + (v8161 - (if v1590 { v4 } else { (if v1504 { (self.scalar_v36 * ((v1586 * v2900) + v5490)) } else { v4 }) })));
        let v8172: f64 = ((v586 * v4825) + v8169);
        let v8173: f64 = ((v586 * v4826) + v8170);
        let v8183: f64 = ((((((v8098 + (v2194 * v3255)) + (v2197 * v3261)) + (v2200 * v3267)) + ((-(v2203 * v2944)) / v8140)) + (v762 * v6952)) + (v757 * v8171));
        let v8184: f64 = ((((((v2184 * v4782) + (v1290 * self.scalar_v3268)) - (v2181 * v8007)) + ((v8090 + v8090) / v337)) + (v762 * v6955)) + ((v2213 * self.scalar_v3268) + (v757 * v8172)));
        let v8210: f64 = ((((v8086 + v8108) + ((v1921 * self.scalar_v3268) + (v762 * v6960))) + (v2627 + (v757 * v8173))) - ((v2183 * v6761) + (v1898 * self.scalar_v8031)));
        let v8211: f64 = (((((v8087 + v8110) + (v747 * (v8125 + v8125))) + (v762 * v6963)) + (v757 * v5062)) - ((v2183 * v6764) + (v1898 * self.scalar_v8032)));
        let v8215: f64 = ((if self.scalar_v1403 { v5100 } else { (if self.scalar_v526 { (v5100 + (v5122 / v5123)) } else { v4 }) }) + ((v1475 * ((v569 * (self.scalar_v561 * (v566 * (self.scalar_v564 * v2714)))) + (v567 * (v569 * (v3030 / self.scalar_v562))))) + (v570 * v5185)));
        let v8219: f64 = (((v1499 * ((v578 * (self.scalar_v571 * (v575 * (self.scalar_v573 * v2714)))) + (v576 * (v578 * (v3030 / self.scalar_v572))))) + (v579 * v5245)) + v8215);
        let v8220: f64 = ((v579 * v5246) + ((if self.scalar_v1403 { v5101 } else { (if self.scalar_v526 { (v5101 + v5128) } else { v4 }) }) + (v570 * v5186)));
        let v8221: f64 = ((v579 * v5247) + ((if self.scalar_v1403 { v5102 } else { (if self.scalar_v526 { (v5102 + v5132) } else { v4 }) }) + (v570 * v5187)));
        let v8222: f64 = ((v579 * v5248) + ((if self.scalar_v1403 { v5103 } else { v5144 }) + (v570 * v5188)));
        let v8230: f64 = (v760 * v5257);
        let v8234: f64 = (((((v8108 + ((v8135 + v8135) / v351)) + (v2642 + (v762 * v6956))) + (v757 * v5170)) - (v2183 * v6758)) + (v2625 + (v760 * v8221)));
        let v8239: f64 = ((v1767 * v6742) + (v1488 * v6742));
        let v8240: f64 = ((v1767 * v6743) + (v1488 * v6743));
        let v8241: f64 = (((v1897 * (if self.scalar_v1765 { (self.scalar_v14 * v5854) } else { v5854 })) + (v1767 * v6744)) + ((v1897 * v5224) + (v1488 * v6744)));
        let v8242: f64 = ((v1767 * v6745) + ((v1897 * (v502 * v5217)) + (v1488 * v6745)));
        let v8247: f64 = ((v1767 * v6750) + (v1488 * v6750));
        let v8266: f64 = (v2224 * self.scalar_v3270);
        let v8282: f64 = (((((v8115 + (v762 * v6966)) + (v757 * v5063)) - ((v2183 * v6765) + (v1898 * self.scalar_v8033))) + v8230) + (v8266 + (v787 * ((v6784 + v6807) + self.scalar_v8250))));
        let v8284: f64 = (((((v8110 + (v739 * (v8117 + v8117))) + (v747 * (v2695 + v2695))) - (v2183 * v6767)) + (v760 * v5258)) + ((v2224 * self.scalar_v3268) + (v787 * (self.scalar_v6110 + (v6788 + v6811)))));
        let v8285: f64 = (v1901 * self.scalar_v3269);
        let v8298: f64 = (v1901 * self.scalar_v3270);
        let v8316: f64 = (((((v731 * (v8105 + v8105)) + (v739 * (v2691 + v2691))) - (v2183 * v6766)) + (v787 * v8247)) + ((v1901 * self.scalar_v3268) + (v792 * v6838)));
        let v8332: f64 = (v1769 * self.scalar_v3270);
        let v8338: f64 = (((v8234 + ((self.scalar_v0 * v2224) + (v787 * (self.scalar_v6109 + (v6776 + v6799))))) + (v8285 + (v792 * v6826))) + (v2634 + (v2229 * v6132)));
        let v8339: f64 = ((((v8210 + (v760 * v8222)) + ((v2224 * self.scalar_v3269) + (v787 * ((v6779 + v6802) + self.scalar_v8249)))) + (v8285 + (v792 * v6829))) + ((v2229 * v6133) + (v1769 * self.scalar_v3269)));
        let v8340: f64 = ((((v8211 + v8230) + (v8266 + (v787 * ((v6782 + v6805) + self.scalar_v8250)))) + (v8298 + (v792 * v6832))) + ((v2229 * v6134) + (v1769 * self.scalar_v8318)));
        let v8354: f64 = ((((((v8183 - (v2183 * v6756)) + (v760 * v8219)) + (v787 * v8241)) + (v792 * v6821)) + (v2229 * v6131)) + (v2232 * v6029));
        let v8359: f64 = (v1846 * self.scalar_v3269);
        let v8360: f64 = ((v2235 * v6569) + v8359);
        let v8372: f64 = (v1846 * self.scalar_v8318);
        let v8375: f64 = (v1846 * self.scalar_v3270);
        let v8382: f64 = (((((v8108 + ((v2649 + v2649) / v351)) - (v2183 * v6753)) + (v787 * v8240)) + ((v1901 * self.scalar_v3271) + (v792 * v6817))) + ((v2235 * v6570) + (v1846 * self.scalar_v3271)));
        let v8383: f64 = ((((v792 * v6818) + ((v2229 * v6130) + (v1769 * self.scalar_v3268))) + ((v2232 * v6028) + (v1741 * self.scalar_v3268))) + ((v2235 * v6571) + (v1846 * self.scalar_v3268)));
        let v8385: f64 = (((((v8184 - (v2183 * v6757)) + ((v2219 * self.scalar_v3268) + (v760 * v8220))) + (v787 * v8242)) + (v792 * v6824)) + (v2235 * v6573));
        let v8401: f64 = (self.scalar_v2241 * v2912);
        let v8409: f64 = ((v4253 - (v2244 * v4251)) / v4256);
        let v8442: f64 = (if v2253 { (v4250 - ((v2257 * v4251) + (v1151 * ((v2255 * (-v8409)) / v2256)))) } else { (if v2246 { (-((v2249 * v4251) + (v1151 * ((v2247 * v8409) / v2248)))) } else { v4 }) });
        let v8443: f64 = (if v2253 { (-(v1151 * ((v2255 * v4278) / v2256))) } else { (if v2246 { (self.scalar_v3268 - (v1151 * ((v2247 * v4258) / v2248))) } else { v4 }) });
        let v8444: f64 = (if v2253 { (-(v1151 * ((v2255 * v4279) / v2256))) } else { (if v2246 { (self.scalar_v0 - (v1151 * ((v2247 * v4259) / v2248))) } else { v4 }) });
        let v8455: f64 = (self.scalar_v1171 * f64::powf(v2263, self.scalar_v4305));
        let v8478: f64 = ((v2269 * (self.scalar_v2240 * v2912)) + (v2261 * (((v2265 * v4311) + (v1173 * (-((-((v2260 * v2900) + (v308 * v8442))) * v8455)))) + (v171 * (-v8442)))));
        let v8490: f64 = ((v683 * v3023) + (v470 * v3230));
        let v8491: f64 = (v440 * v8490);
        let v8499: f64 = ((v2276 * v6894) + (v1912 * ((v2275 * v4538) + (v1237 * v8491))));
        let v8502: f64 = ((v2276 * v6895) + (v1912 * (v2275 * v4542)));
        let v8505: f64 = ((v2276 * v6896) + (v1912 * (v2275 * v4546)));
        let v8506: f64 = (v2276 * v6897);
        let v8507: f64 = (v2276 * v6898);
        let v8516: f64 = ((v2278 * v6894) + (v1912 * ((v2275 * v4575) + (v1244 * v8491))));
        let v8517: f64 = (v2278 * v6895);
        let v8520: f64 = ((v2278 * v6896) + (v1912 * (v2275 * v4579)));
        let v8523: f64 = ((v2278 * v6897) + (v1912 * (v2275 * v4583)));
        let v8526: f64 = ((v2278 * v6898) + (v1912 * (v2275 * v4587)));
        let v8528: f64 = (v1096 * (-v4354));
        let v8531: f64 = (v1096 * v1096);
        let v8532: f64 = ((v8528 - (v2280 * v4072)) / v8531);
        let v8533: f64 = (self.scalar_v0 / v1096);
        let v8534: f64 = (self.scalar_v3269 / v1096);
        let v8535: f64 = (self.scalar_v3270 / v1096);
        let v8536: f64 = (self.scalar_v3268 / v1096);
        let v8566: f64 = (-v8534);
        let v8567: f64 = (-v8535);
        let v8568: f64 = (-v8536);
        let v8591: f64 = (if v2289 { (v4354 - ((v2293 * v4072) + (v1096 * ((v2291 * (-v8532)) / v2292)))) } else { (if v2282 { (-((v2285 * v4072) + (v1096 * ((v2283 * v8532) / v2284)))) } else { v4 }) });
        let v8592: f64 = (if v2289 { (-(v1096 * ((v2291 * (-v8533)) / v2292))) } else { (if v2282 { (self.scalar_v0 - (v1096 * ((v2283 * v8533) / v2284))) } else { v4 }) });
        let v8593: f64 = (if v2289 { (-(v1096 * ((v2291 * v8566) / v2292))) } else { (if v2282 { (self.scalar_v3269 - (v1096 * ((v2283 * v8534) / v2284))) } else { v4 }) });
        let v8594: f64 = (if v2289 { (-(v1096 * ((v2291 * v8567) / v2292))) } else { (if v2282 { (self.scalar_v3270 - (v1096 * ((v2283 * v8535) / v2284))) } else { v4 }) });
        let v8595: f64 = (if v2289 { (-(v1096 * ((v2291 * v8568) / v2292))) } else { (if v2282 { (self.scalar_v3268 - (v1096 * ((v2283 * v8536) / v2284))) } else { v4 }) });
        let v8610: f64 = (self.scalar_v1216 * f64::powf(v2298, self.scalar_v4452));
        let v8640: f64 = (((v2300 * v4440) + (v1217 * (-((-(((v261 * v8591) - (v2296 * v2851)) / v2902)) * v8610)))) + ((v2302 * v4346) + (v1192 * (-v8591))));
        let v8653: f64 = (v330 * self.scalar_v3269);
        let v8654: f64 = (v330 * self.scalar_v3270);
        let v8673: f64 = (self.scalar_v14 * (self.scalar_v2309 * (v329 * (v4514 + (v1191 * ((v1217 * (-((-(v8592 / v261)) * v8610))) + (v1192 * (self.scalar_v0 - v8592))))))));
        let v8674: f64 = (self.scalar_v14 * (self.scalar_v2309 * (v329 * ((v1191 * ((v1217 * (-((-(v8593 / v261)) * v8610))) + (v1192 * (self.scalar_v3269 - v8593)))) + v8653))));
        let v8675: f64 = (self.scalar_v14 * (self.scalar_v2309 * (v329 * ((v1191 * ((v1217 * (-((-(v8594 / v261)) * v8610))) + (v1192 * (self.scalar_v3270 - v8594)))) + v8654))));
        let v8676: f64 = (self.scalar_v14 * (self.scalar_v2309 * (v329 * (v4515 + (v1191 * ((v1217 * (-((-(v8595 / v261)) * v8610))) + (v1192 * (self.scalar_v3268 - v8595))))))));
        let v8677: f64 = (self.scalar_v3271 / v1096);
        let v8680: f64 = ((v8528 - (v2312 * v4072)) / v8531);
        let v8732: f64 = (if v2321 { (-(v1096 * ((v2323 * v8566) / v2324))) } else { (if v2314 { (self.scalar_v3269 - (v1096 * ((v2315 * v8534) / v2316))) } else { v4 }) });
        let v8733: f64 = (if v2321 { (-(v1096 * ((v2323 * (-v8677)) / v2324))) } else { (if v2314 { (self.scalar_v3271 - (v1096 * ((v2315 * v8677) / v2316))) } else { v4 }) });
        let v8734: f64 = (if v2321 { (v4354 - ((v2325 * v4072) + (v1096 * ((v2323 * (-v8680)) / v2324)))) } else { (if v2314 { (-((v2317 * v4072) + (v1096 * ((v2315 * v8680) / v2316)))) } else { v4 }) });
        let v8735: f64 = (if v2321 { (-(v1096 * ((v2323 * v8567) / v2324))) } else { (if v2314 { (self.scalar_v3270 - (v1096 * ((v2315 * v8535) / v2316))) } else { v4 }) });
        let v8736: f64 = (if v2321 { (-(v1096 * ((v2323 * v8568) / v2324))) } else { (if v2314 { (self.scalar_v3268 - (v1096 * ((v2315 * v8536) / v2316))) } else { v4 }) });
        let v8751: f64 = (self.scalar_v1216 * f64::powf(v2330, self.scalar_v4452));
        let v8783: f64 = (((v2332 * v4440) + (v1217 * (-((-(((v261 * v8734) - (v2328 * v2851)) / v2902)) * v8751)))) + ((v2334 * v4346) + (v1192 * (-v8734))));
        let v8808: f64 = (self.scalar_v2309 * (v329 * ((v1191 * ((v1217 * (-((-(v8733 / v261)) * v8751))) + (v1192 * (self.scalar_v3271 - v8733)))) + (v330 * self.scalar_v3271))));
        let v8812: f64 = (self.scalar_v13 * (self.scalar_v2309 * (v329 * (v8653 + (v1191 * ((v1217 * (-((-(v8732 / v261)) * v8751))) + (v1192 * (self.scalar_v3269 - v8732))))))));
        let v8815: f64 = (self.scalar_v13 * (self.scalar_v2309 * (v329 * (v8654 + (v1191 * ((v1217 * (-((-(v8735 / v261)) * v8751))) + (v1192 * (self.scalar_v3270 - v8735))))))));
        let v8816: f64 = (self.scalar_v13 * (self.scalar_v2309 * (v329 * (v4515 + (v1191 * ((v1217 * (-((-(v8736 / v261)) * v8751))) + (v1192 * (self.scalar_v3268 - v8736))))))));
        let v8817: f64 = (v47 * v2897);
        let v8818: f64 = (self.scalar_v2346 * v2897);
        let v8820: f64 = (self.scalar_v0 / v2343);
        let v8825: f64 = (((v2343 * (-v8818)) - (v2348 * v8817)) / (v2343 * v2343));
        let v8826: f64 = (self.scalar_v3268 / v2343);
        let v8861: f64 = (if v2357 { (-(v2343 * ((v2359 * (-v8820)) / v2360))) } else { (if v2350 { (self.scalar_v0 - (v2343 * ((v2351 * v8820) / v2352))) } else { v4 }) });
        let v8862: f64 = (if v2357 { (v8818 - ((v2361 * v8817) + (v2343 * ((v2359 * (-v8825)) / v2360)))) } else { (if v2350 { (-((v2353 * v8817) + (v2343 * ((v2351 * v8825) / v2352)))) } else { v4 }) });
        let v8863: f64 = (if v2357 { (-(v2343 * ((v2359 * (-v8826)) / v2360))) } else { (if v2350 { (self.scalar_v3268 - (v2343 * ((v2351 * v8826) / v2352))) } else { v4 }) });
        let v8876: f64 = (self.scalar_v2365 * f64::powf(v2368, self.scalar_v8874));
        let v8895: f64 = (((v2370 * (v2897 / self.scalar_v2365)) + (v2366 * (-((-(((v307 * v8862) - (v2364 * v2897)) / v2915)) * v8876)))) + (v32 * (-v8862)));
        let v8913: f64 = (v2380 * ((v677 * v3023) + (v470 * ((v676 * (self.scalar_v668 * (v671 * (self.scalar_v669 * v2714)))) + (v672 * (v676 * (self.scalar_v674 * v2713)))))));
        let v8916: f64 = (self.scalar_v2378 * v2710);
        let v8919: f64 = (v2382 * v2382);
        let v8920: f64 = ((-(v757 * v8916)) / v8919);
        let v8921: f64 = (self.scalar_v3268 / v2382);
        let v8922: f64 = (self.scalar_v0 / v2382);
        let v8943: f64 = ((v2392 * (v8913 + (v2376 * ((((v470 * v3020) - (v465 * v3023)) / v4523) * (self.scalar_v2379 * f64::powf(v2377, self.scalar_v8909)))))) + (v2381 * (if v2387 { (v2388 * v8920) } else { (if v2384 { (v2385 * v8920) } else { v5245 }) })));
        let v8944: f64 = (v2381 * (if v2387 { (v2388 * v8921) } else { (if v2384 { (v2385 * v8921) } else { v5246 }) }));
        let v8945: f64 = (v2381 * (if v2387 { v4 } else { (if v2384 { v4 } else { v5247 }) }));
        let v8946: f64 = (v2381 * (if v2387 { (v2388 * v8922) } else { (if v2384 { (v2385 * v8922) } else { v5248 }) }));
        let v8947: f64 = (v2381 * (if v2387 { v4 } else { (if v2384 { v4 } else { v5249 }) }));
        let v8948: f64 = (v2381 * (if v2387 { v4 } else { (if v2384 { v4 } else { v5250 }) }));
        let v8956: f64 = (((v368 * ((v2394 * v2710) + (v120 * (v452 * v3233)))) - (v2395 * v2954)) / v3552);
        let v8991: f64 = (((v2274 * (((v1686 * (v5758 - v4524)) - (v1683 * (v5758 / v5769))) / v5778)) + (v1687 * v8490)) + ((v2396 * (((v1690 * v5763) - (v1682 * (v5763 / v5796))) / v5805)) + (v1691 * v8956)));
        let v8992: f64 = ((v2274 * (((v1686 * v5759) - (v1683 * (v5759 / v5769))) / v5778)) + (v2396 * (((v1690 * v5764) - (v1682 * (v5764 / v5796))) / v5805)));
        let v8993: f64 = ((v2274 * (((v1686 * v5760) - (v1683 * (v5760 / v5769))) / v5778)) + (v2396 * (((v1690 * v5765) - (v1682 * (v5765 / v5796))) / v5805)));
        let v8994: f64 = ((v2274 * (((v1686 * v5761) - (v1683 * (v5761 / v5769))) / v5778)) + (v2396 * (((v1690 * v5766) - (v1682 * (v5766 / v5796))) / v5805)));
        let v8995: f64 = ((v2274 * (((v1686 * v5762) - (v1683 * (v5762 / v5769))) / v5778)) + (v2396 * (((v1690 * v5767) - (v1682 * (v5767 / v5796))) / v5805)));
        let v9006: f64 = (v690 * v690);
        let v9017: f64 = (-v2829);
        let v9025: f64 = ((v2412 * v2713) + (v122 * (v9017 / self.scalar_v2411)));
        let v9026: f64 = (v122 * self.scalar_v9019);
        let v9027: f64 = (v122 * self.scalar_v9020);
        let v9028: f64 = (v122 * self.scalar_v9021);
        let v9029: f64 = (v122 * self.scalar_v9022);
        let v9065: f64 = (v32 * v2430);
        let v9073: f64 = ((v2431 * ((v2426 * v3324) + (v827 * ((v1692 * v3239) + (v699 * v5823))))) - (v2427 * ((v452 * (if v2420 { (v2421 * v9025) } else { (if v2416 { (v2417 * v9025) } else { v4 }) })) / v9065)));
        let v9074: f64 = (v2431 * v2431);
        let v9079: f64 = (((v2431 * (v2426 * v3325)) - (v2427 * ((v452 * (if v2420 { (v2421 * v9026) } else { (if v2416 { (v2417 * v9026) } else { v4 }) })) / v9065))) / v9074);
        let v9083: f64 = (((v2431 * (v2426 * v3326)) - (v2427 * ((v452 * (if v2420 { (v2421 * v9027) } else { (if v2416 { (v2417 * v9027) } else { v4 }) })) / v9065))) / v9074);
        let v9087: f64 = (((v2431 * (v2426 * v3327)) - (v2427 * ((v452 * (if v2420 { (v2421 * v9028) } else { (if v2416 { (v2417 * v9028) } else { v4 }) })) / v9065))) / v9074);
        let v9091: f64 = (((v2431 * (v2426 * v3328)) - (v2427 * ((v452 * (if v2420 { (v2421 * v9029) } else { (if v2416 { (v2417 * v9029) } else { v4 }) })) / v9065))) / v9074);
        let v9092: f64 = (if self.scalar_v2415 { (v9073 / v9074) } else { (if self.scalar_v2402 { (((v690 * ((v2406 * (v440 * v3236)) + (v2403 * v8991))) - (v2407 * v3234)) / v9006) } else { v4 }) });
        let v9093: f64 = (if self.scalar_v2415 { v9079 } else { (if self.scalar_v2402 { ((v2403 * v8992) / v690) } else { v4 }) });
        let v9094: f64 = (if self.scalar_v2415 { v9083 } else { (if self.scalar_v2402 { ((v2403 * v8993) / v690) } else { v4 }) });
        let v9095: f64 = (if self.scalar_v2415 { v9087 } else { (if self.scalar_v2402 { ((v2403 * v8994) / v690) } else { v4 }) });
        let v9096: f64 = (if self.scalar_v2415 { v9091 } else { (if self.scalar_v2402 { ((v2403 * v8995) / v690) } else { v4 }) });
        let v9114: f64 = (if self.scalar_v2439 { (v1232 * v3359) } else { v4 });
        let v9115: f64 = (if self.scalar_v2439 { (v1232 * v3360) } else { v4 });
        let v9116: f64 = (if self.scalar_v2439 { ((v1232 * v3361) + (v847 * v4524)) } else { v4 });
        let v9117: f64 = (if self.scalar_v2439 { (v1232 * v3362) } else { v4 });
        let v9118: f64 = (if self.scalar_v2439 { (v1232 * v3363) } else { v4 });
        let v9120: f64 = (v32 * v2444);
        let v9129: f64 = (v2445 * v2445);
        let v9157: f64 = (if self.scalar_v2439 { (v452 * (if v883 { (v884 * v3307) } else { (if v880 { (v881 * v3307) } else { v4 }) })) } else { v4 });
        let v9158: f64 = (if self.scalar_v2439 { (v452 * (if v883 { (v884 * v3342) } else { (if v880 { (v881 * v3342) } else { v4 }) })) } else { v4 });
        let v9159: f64 = (if self.scalar_v2439 { (v452 * (if v883 { (v884 * v3414) } else { (if v880 { (v881 * v3414) } else { v4 }) })) } else { v4 });
        let v9160: f64 = (if self.scalar_v2439 { (v452 * (if v883 { (v884 * v3308) } else { (if v880 { (v881 * v3308) } else { v4 }) })) } else { v4 });
        let v9161: f64 = (if self.scalar_v2439 { (v452 * (if v883 { (v884 * v3274) } else { (if v880 { (v881 * v3274) } else { v4 }) })) } else { v4 });
        let v9162: f64 = (v32 * v2451);
        let v9171: f64 = (v2452 * v2452);
        let v9209: f64 = ((v2274 * (if self.scalar_v2439 { (((v2445 * v9114) - (v2442 * (v9114 / v9120))) / v9129) } else { v4 })) + (v2396 * (if self.scalar_v2439 { (((v2452 * v9157) - (v2449 * (v9157 / v9162))) / v9171) } else { v4 })));
        let v9210: f64 = ((v2274 * (if self.scalar_v2439 { (((v2445 * v9115) - (v2442 * (v9115 / v9120))) / v9129) } else { v4 })) + (v2396 * (if self.scalar_v2439 { (((v2452 * v9158) - (v2449 * (v9158 / v9162))) / v9171) } else { v4 })));
        let v9211: f64 = (((v2447 * v8490) + (v2274 * (if self.scalar_v2439 { (((v2445 * (v9116 - v4524)) - (v2442 * (v9116 / v9120))) / v9129) } else { v4 }))) + ((v2454 * v8956) + (v2396 * (if self.scalar_v2439 { (((v2452 * v9159) - (v2449 * (v9159 / v9162))) / v9171) } else { v4 }))));
        let v9212: f64 = ((v2274 * (if self.scalar_v2439 { (((v2445 * v9117) - (v2442 * (v9117 / v9120))) / v9129) } else { v4 })) + (v2396 * (if self.scalar_v2439 { (((v2452 * v9160) - (v2449 * (v9160 / v9162))) / v9171) } else { v4 })));
        let v9213: f64 = ((v2274 * (if self.scalar_v2439 { (((v2445 * v9118) - (v2442 * (v9118 / v9120))) / v9129) } else { v4 })) + (v2396 * (if self.scalar_v2439 { (((v2452 * v9161) - (v2449 * (v9161 / v9162))) / v9171) } else { v4 })));
        let v9236: f64 = ((v2463 * v2713) + (v122 * v9017));
        let v9272: f64 = (v32 * v2481);
        let v9281: f64 = (v2482 * v2482);
        let v9282: f64 = (((v2482 * (v2477 * v3359)) - (v2478 * ((v452 * (if v2471 { (v2472 * v3307) } else { (if v2467 { (v2468 * v3307) } else { v4 }) })) / v9272))) / v9281);
        let v9286: f64 = (((v2482 * (v2477 * v3360)) - (v2478 * ((v452 * (if v2471 { (v2472 * v3342) } else { (if v2467 { (v2468 * v3342) } else { v4 }) })) / v9272))) / v9281);
        let v9289: f64 = ((v2482 * ((v2477 * v3361) + (v847 * ((v1771 * v3239) + (v699 * v6137))))) - (v2478 * ((v452 * (if v2471 { (v2472 * v9236) } else { (if v2467 { (v2468 * v9236) } else { v4 }) })) / v9272)));
        let v9294: f64 = (((v2482 * (v2477 * v3362)) - (v2478 * ((v452 * (if v2471 { (v2472 * v3308) } else { (if v2467 { (v2468 * v3308) } else { v4 }) })) / v9272))) / v9281);
        let v9298: f64 = (((v2482 * (v2477 * v3363)) - (v2478 * ((v452 * (if v2471 { (v2472 * v3274) } else { (if v2467 { (v2468 * v3274) } else { v4 }) })) / v9272))) / v9281);
        let v9301: f64 = (if self.scalar_v2466 { (v9289 / v9281) } else { (if self.scalar_v2439 { (((v690 * ((v2459 * (self.scalar_v2455 * v3236)) + (v2456 * v9211))) - (v2460 * v3234)) / v9006) } else { v4 }) });
        let v9305: f64 = (v1842 * (if self.scalar_v2466 { v9282 } else { (if self.scalar_v2439 { ((v2456 * v9209) / v690) } else { v4 }) }));
        let v9318: f64 = (v1842 * (if self.scalar_v2466 { v9294 } else { (if self.scalar_v2439 { ((v2456 * v9212) / v690) } else { v4 }) }));
        let v9339: f64 = (self.scalar_v2489 * f64::powf(v1170, self.scalar_v9337));
        let v9346: f64 = (if self.scalar_v2488 { v4257 } else { v4 });
        let v9347: f64 = (if self.scalar_v2488 { v4258 } else { v4 });
        let v9348: f64 = (if self.scalar_v2488 { v4259 } else { v4 });
        let v9353: f64 = (v2497 * v2497);
        let v9365: f64 = (v2503 * (-v9346));
        let v9366: f64 = (v2503 * (-v9347));
        let v9367: f64 = (v2503 * (-v9348));
        let v9371: f64 = (v2504 * v2504);
        let v9386: f64 = ((v2506 * (if self.scalar_v2488 { (v4302 * v9339) } else { v4 })) + (v2492 * (if v2501 { (((v2504 * v9365) - (v2503 * v9365)) / v9371) } else { (if v2495 { ((-(v2496 * v9346)) / v9353) } else { v4 }) })));
        let v9389: f64 = ((v2506 * (if self.scalar_v2488 { (v4303 * v9339) } else { v4 })) + (v2492 * (if v2501 { (((v2504 * v9366) - (v2503 * v9366)) / v9371) } else { (if v2495 { ((-(v2496 * v9347)) / v9353) } else { v4 }) })));
        let v9392: f64 = ((v2506 * (if self.scalar_v2488 { (v4304 * v9339) } else { v4 })) + (v2492 * (if v2501 { (((v2504 * v9367) - (v2503 * v9367)) / v9371) } else { (if v2495 { ((-(v2496 * v9348)) / v9353) } else { v4 }) })));
        let v9417: f64 = (v1235 * v1235);
        let v9427: f64 = ((v2514 * (((v400 * ((v1233 * v2713) + (v122 * v4527))) - (v2512 * v2971)) / v3008)) + (v2513 * ((-(v440 * v4531)) / v9417)));
        let v9449: f64 = ((v2517 * (if self.scalar_v2488 { ((v2514 * ((v122 * v4528) / v400)) + (v2513 * ((-(v440 * v4532)) / v9417))) } else { v4 })) + (v2516 * (v2275 * v6895)));
        let v9452: f64 = ((v2517 * (if self.scalar_v2488 { ((v2514 * ((v122 * v4529) / v400)) + (v2513 * ((-(v440 * v4533)) / v9417))) } else { v4 })) + (v2516 * (v2275 * v6896)));
        let v9473: f64 = (if self.scalar_v2488 { (v8947 / v2382) } else { v4 });
        let v9477: f64 = ((if self.scalar_v2488 { ((v2509 * v8401) + (v2242 * (if self.scalar_v2488 { v9386 } else { v4 }))) } else { v4 }) + (if self.scalar_v2488 { ((v2517 * (if self.scalar_v2488 { v9427 } else { v4 })) + (v2516 * ((v2275 * v6894) + (v1912 * v8491)))) } else { v4 }));
        let v9492: f64 = ((v2524 * self.scalar_v9476) + (v2522 * ((if self.scalar_v2488 { (v8946 / v2382) } else { v4 }) + ((if self.scalar_v2488 { (v2242 * (if self.scalar_v2488 { v9392 } else { v4 })) } else { v4 }) + (if self.scalar_v2488 { v9452 } else { v4 })))));
        let v9497: f64 = (if self.scalar_v2488 { (v2522 * ((if self.scalar_v2488 { (v8944 / v2382) } else { v4 }) + ((if self.scalar_v2488 { (v2242 * (if self.scalar_v2488 { v9389 } else { v4 })) } else { v4 }) + (if self.scalar_v2488 { v9449 } else { v4 })))) } else { v4 });
        let v9519: f64 = (self.scalar_v2527 * v8947);
        let v9526: f64 = (if self.scalar_v2488 { (v8499 + (self.scalar_v2527 * v8943)) } else { v4 });
        let v9527: f64 = (if self.scalar_v2488 { (v8502 + (self.scalar_v2527 * v8944)) } else { v4 });
        let v9528: f64 = (if self.scalar_v2488 { (self.scalar_v2527 * v8945) } else { v4 });
        let v9529: f64 = (if self.scalar_v2488 { (v8505 + (self.scalar_v2527 * v8946)) } else { v4 });
        let v9530: f64 = (if self.scalar_v2488 { (v8506 + v9519) } else { v4 });
        let v9531: f64 = (if self.scalar_v2488 { (v8507 + v9519) } else { v4 });
        let v9532: f64 = (if self.scalar_v2488 { (self.scalar_v2527 * v8948) } else { v4 });
        let v9566: f64 = (if self.scalar_v2541 { v8499 } else { (if self.scalar_v2488 { (self.scalar_v2538 * v9526) } else { v4 }) });
        let v9567: f64 = (if self.scalar_v2541 { v8502 } else { (if self.scalar_v2488 { (self.scalar_v2538 * v9527) } else { v4 }) });
        let v9568: f64 = (if self.scalar_v2541 { v4 } else { (if self.scalar_v2488 { (self.scalar_v2538 * v9528) } else { v4 }) });
        let v9569: f64 = (if self.scalar_v2541 { v8505 } else { (if self.scalar_v2488 { (self.scalar_v2538 * v9529) } else { v4 }) });
        let v9570: f64 = (if self.scalar_v2541 { v8506 } else { (if self.scalar_v2488 { (self.scalar_v2538 * v9530) } else { v4 }) });
        let v9571: f64 = (if self.scalar_v2541 { v8507 } else { (if self.scalar_v2488 { (self.scalar_v2538 * v9531) } else { v4 }) });
        let v9572: f64 = (if self.scalar_v2541 { v4 } else { (if self.scalar_v2488 { (self.scalar_v2538 * v9532) } else { v4 }) });
        let v9573: f64 = (if self.scalar_v2541 { v8516 } else { (if self.scalar_v2488 { (v8516 + (self.scalar_v2534 * v9526)) } else { v4 }) });
        let v9574: f64 = (if self.scalar_v2541 { v8517 } else { (if self.scalar_v2488 { (v8517 + (self.scalar_v2534 * v9527)) } else { v4 }) });
        let v9575: f64 = (if self.scalar_v2541 { v4 } else { (if self.scalar_v2488 { (self.scalar_v2534 * v9528) } else { v4 }) });
        let v9576: f64 = (if self.scalar_v2541 { v8520 } else { (if self.scalar_v2488 { (v8520 + (self.scalar_v2534 * v9529)) } else { v4 }) });
        let v9577: f64 = (if self.scalar_v2541 { v8523 } else { (if self.scalar_v2488 { (v8523 + (self.scalar_v2534 * v9530)) } else { v4 }) });
        let v9578: f64 = (if self.scalar_v2541 { v8526 } else { (if self.scalar_v2488 { (v8526 + (self.scalar_v2534 * v9531)) } else { v4 }) });
        let v9579: f64 = (if self.scalar_v2541 { v4 } else { (if self.scalar_v2488 { (self.scalar_v2534 * v9532) } else { v4 }) });
        let v9584: f64 = (if self.scalar_v2541 { v8947 } else { (if self.scalar_v2488 { (self.scalar_v2528 * v8947) } else { v4 }) });
        let v9586: f64 = ddt_scale;
        let v9588: f64 = (self.scalar_v27 * (self.scalar_v2545 * v9586));
        let v9603: f64 = (if self.scalar_v2578 { self.scalar_v9602 } else { (if self.scalar_v2570 { (self.scalar_v2573 * (self.scalar_v9592 * (self.scalar_v2549 * f64::powf(v2565, self.scalar_v9596)))) } else { (if self.scalar_v2561 { (self.scalar_v2563 * (self.scalar_v9592 / v2565)) } else { self.scalar_v9591 }) }) });
        let v9625: f64 = (v2582 * v2582);
        let v9684: f64 = (if v2594 { ((v2595 * v4745) + (v1284 * ((v1912 * v3230) + (v683 * v6894)))) } else { (if v2590 { (((v2582 * (v9566 + v9573)) - (v2591 * (((v1284 * (v4761 + v4767)) - (v2581 * v4745)) / v4777))) / v9625) } else { v4 }) });
        let v9685: f64 = (if v2594 { ((v2595 * v4748) + (v1284 * (v683 * v6895))) } else { (if v2590 { (((v2582 * (v9567 + v9574)) - (v2591 * ((v4779 - (v2581 * v4748)) / v4777))) / v9625) } else { v4 }) });
        let v9686: f64 = (if v2594 { v4 } else { (if v2590 { ((v9568 + v9575) / v2582) } else { v4 }) });
        let v9687: f64 = (if v2594 { ((v2595 * v4751) + (v1284 * (v683 * v6896))) } else { (if v2590 { (((v2582 * (v9569 + v9576)) - (v2591 * (((v1284 * (v4762 + v4769)) - (v2581 * v4751)) / v4777))) / v9625) } else { v4 }) });
        let v9688: f64 = (if v2594 { ((v2595 * v4754) + (v1284 * (v683 * v6897))) } else { (if v2590 { (((v2582 * (v9570 + v9577)) - (v2591 * (((v1284 * v4763) - (v2581 * v4754)) / v4777))) / v9625) } else { v4 }) });
        let v9689: f64 = (if v2594 { ((v2595 * v4757) + (v1284 * (v683 * v6898))) } else { (if v2590 { (((v2582 * (v9571 + v9578)) - (v2591 * (((v1284 * v4764) - (v2581 * v4757)) / v4777))) / v9625) } else { v4 }) });
        let v9690: f64 = (if v2594 { v4 } else { (if v2590 { ((v9572 + v9579) / v2582) } else { v4 }) });
        let v9750: f64 = (((v2399 * ((v2397 * v4222) + (v1140 * (v440 * v8956)))) + (v2398 * v4170)) + (((v2272 * v4516) + (v1230 * (self.scalar_v2271 * v2932))) + v9573));
        let v9754: f64 = ((self.scalar_v14 * (self.scalar_v2309 * ((v2307 * v2932) + (v329 * (((v2304 * v4341) + (v1191 * v8640)) + (v787 * v2933)))))) + (if self.scalar_v2436 { (self.scalar_v14 * v9092) } else { v9092 }));
        let v9761: f64 = ((self.scalar_v13 * (self.scalar_v2309 * ((v2339 * v2932) + (v329 * (((v2336 * v4341) + (v1191 * v8783)) + (v792 * v2933)))))) + (if self.scalar_v2436 { ((v2484 * v6503) + (v1842 * v9301)) } else { v4 }));
        let v9771: f64 = (self.scalar_v27 * (self.scalar_v0 * v3553));
        let v9772: f64 = (self.scalar_v27 * (self.scalar_v0 * v3554));
        let v9773: f64 = (self.scalar_v27 * (self.scalar_v0 * v3555));
        let v9774: f64 = (self.scalar_v27 * (self.scalar_v0 * v3556));
        let v9780: f64 = (self.scalar_v27 * (self.scalar_v0 * v4778));
        let v9781: f64 = (self.scalar_v27 * (self.scalar_v0 * v4782));
        let v9782: f64 = (self.scalar_v27 * (self.scalar_v0 * v4786));
        let v9783: f64 = (self.scalar_v27 * (self.scalar_v0 * v4790));
        let v9784: f64 = (self.scalar_v27 * (self.scalar_v0 * v4794));
        let v9791: f64 = (self.scalar_v27 * (self.scalar_v0 * v8219));
        let v9792: f64 = (self.scalar_v27 * (self.scalar_v0 * v8220));
        let v9793: f64 = (self.scalar_v27 * (self.scalar_v0 * v8221));
        let v9794: f64 = (self.scalar_v27 * (self.scalar_v0 * v8222));
        let v9795: f64 = (self.scalar_v27 * (self.scalar_v0 * v5257));
        let v9796: f64 = (self.scalar_v27 * (self.scalar_v0 * v5258));
        let v9803: f64 = (self.scalar_v27 * (self.scalar_v0 * v8171));
        let v9804: f64 = (self.scalar_v27 * (self.scalar_v0 * v8172));
        let v9805: f64 = (self.scalar_v27 * (self.scalar_v0 * v5170));
        let v9806: f64 = (self.scalar_v27 * (self.scalar_v0 * v8173));
        let v9807: f64 = (self.scalar_v27 * (self.scalar_v0 * v5062));
        let v9808: f64 = (self.scalar_v27 * (self.scalar_v0 * v5063));
        let v9829: f64 = (self.scalar_v27 * (self.scalar_v0 * (-v6752)));
        let v9830: f64 = (self.scalar_v27 * (self.scalar_v0 * (-v6753)));
        let v9831: f64 = (self.scalar_v27 * (self.scalar_v0 * (-v6756)));
        let v9832: f64 = (self.scalar_v27 * (self.scalar_v0 * (-v6757)));
        let v9833: f64 = (self.scalar_v27 * (self.scalar_v0 * (-v6758)));
        let v9834: f64 = (self.scalar_v27 * (self.scalar_v0 * (-v6761)));
        let v9835: f64 = (self.scalar_v27 * (self.scalar_v0 * (-v6764)));
        let v9836: f64 = (self.scalar_v27 * (self.scalar_v0 * (-v6765)));
        let v9837: f64 = (self.scalar_v27 * (self.scalar_v0 * (-v6766)));
        let v9838: f64 = (self.scalar_v27 * (self.scalar_v0 * (-v6767)));
        let v9839: f64 = (if self.scalar_v526 { v9829 } else { v4 });
        let v9840: f64 = (if self.scalar_v526 { v9830 } else { v4 });
        let v9841: f64 = (if self.scalar_v526 { v9831 } else { v4 });
        let v9842: f64 = (if self.scalar_v526 { v9832 } else { v4 });
        let v9843: f64 = (if self.scalar_v526 { v9833 } else { v4 });
        let v9844: f64 = (if self.scalar_v526 { v9834 } else { v4 });
        let v9845: f64 = (if self.scalar_v526 { v9835 } else { v4 });
        let v9846: f64 = (if self.scalar_v526 { v9836 } else { v4 });
        let v9847: f64 = (if self.scalar_v526 { v9837 } else { v4 });
        let v9848: f64 = (if self.scalar_v526 { v9838 } else { v4 });
        let v9849: f64 = (if self.scalar_v1403 { v9829 } else { v4 });
        let v9850: f64 = (if self.scalar_v1403 { v9830 } else { v4 });
        let v9851: f64 = (if self.scalar_v1403 { v9831 } else { v4 });
        let v9852: f64 = (if self.scalar_v1403 { v9832 } else { v4 });
        let v9853: f64 = (if self.scalar_v1403 { v9833 } else { v4 });
        let v9854: f64 = (if self.scalar_v1403 { v9834 } else { v4 });
        let v9855: f64 = (if self.scalar_v1403 { v9835 } else { v4 });
        let v9856: f64 = (if self.scalar_v1403 { v9836 } else { v4 });
        let v9857: f64 = (if self.scalar_v1403 { v9837 } else { v4 });
        let v9858: f64 = (if self.scalar_v1403 { v9838 } else { v4 });
        let v9866: f64 = (self.scalar_v27 * (self.scalar_v0 * v6130));
        let v9867: f64 = (self.scalar_v27 * (self.scalar_v0 * v6131));
        let v9868: f64 = (self.scalar_v27 * (self.scalar_v0 * v6132));
        let v9869: f64 = (self.scalar_v27 * (self.scalar_v0 * v6133));
        let v9870: f64 = (self.scalar_v27 * (self.scalar_v0 * v6134));
        let v9871: f64 = (self.scalar_v27 * (self.scalar_v0 * v6135));
        let v9872: f64 = (self.scalar_v27 * (self.scalar_v0 * v6136));
        let v9878: f64 = (self.scalar_v27 * (self.scalar_v0 * v6028));
        let v9879: f64 = (self.scalar_v27 * (self.scalar_v0 * v6029));
        let v9880: f64 = (self.scalar_v27 * (self.scalar_v0 * v6030));
        let v9881: f64 = (self.scalar_v27 * (self.scalar_v0 * v6031));
        let v9882: f64 = (self.scalar_v27 * (self.scalar_v0 * v6032));
        let v9893: f64 = (self.scalar_v27 * (self.scalar_v0 * v6569));
        let v9894: f64 = (self.scalar_v27 * (self.scalar_v0 * v6570));
        let v9895: f64 = (self.scalar_v27 * (self.scalar_v0 * v6571));
        let v9896: f64 = (self.scalar_v27 * (self.scalar_v0 * v6572));
        let v9897: f64 = (self.scalar_v27 * (self.scalar_v0 * v6573));
        let v9898: f64 = (self.scalar_v27 * (self.scalar_v0 * v6574));
        let v9899: f64 = (self.scalar_v27 * (self.scalar_v0 * v6575));
        let v9900: f64 = (self.scalar_v27 * (self.scalar_v0 * v6576));
        let v9901: f64 = (self.scalar_v27 * (self.scalar_v0 * v6577));
        let v9902: f64 = (self.scalar_v27 * (self.scalar_v0 * v6578));
        let v9906: f64 = (self.scalar_v27 * (self.scalar_v0 * v6111));
        let v9907: f64 = (self.scalar_v27 * (self.scalar_v0 * v6104));
        let v9908: f64 = (self.scalar_v27 * (self.scalar_v0 * v6112));
        let v9915: f64 = (self.scalar_v27 * (self.scalar_v0 * v6952));
        let v9916: f64 = (self.scalar_v27 * (self.scalar_v0 * v6955));
        let v9917: f64 = (self.scalar_v27 * (self.scalar_v0 * v6956));
        let v9918: f64 = (self.scalar_v27 * (self.scalar_v0 * v6960));
        let v9919: f64 = (self.scalar_v27 * (self.scalar_v0 * v6963));
        let v9920: f64 = (self.scalar_v27 * (self.scalar_v0 * v6966));
        let v9926: f64 = (self.scalar_v27 * (self.scalar_v0 * (-v8006)));
        let v9927: f64 = (self.scalar_v27 * (self.scalar_v0 * (-v8007)));
        let v9928: f64 = (self.scalar_v27 * (self.scalar_v0 * (-v8008)));
        let v9929: f64 = (self.scalar_v27 * (self.scalar_v0 * (-v8009)));
        let v9930: f64 = (self.scalar_v27 * (self.scalar_v0 * (-v8010)));
        let v9938: f64 = (self.scalar_v27 * (self.scalar_v9931 / v337));
        let v9939: f64 = (self.scalar_v27 * ((-(v2646 * v2937)) / v8095));
        let v9940: f64 = (self.scalar_v27 * (self.scalar_v9932 / v337));
        let v9946: f64 = (self.scalar_v27 * (self.scalar_v9931 / v351));
        let v9947: f64 = (self.scalar_v27 * ((-(v2649 * v2944)) / v8140));
        let v9948: f64 = (self.scalar_v27 * (self.scalar_v9932 / v351));
        let v9961: f64 = (self.scalar_v27 * (-(((((v731 * (v2679 + v2679)) - (v2183 * v6752)) + (v787 * v8239)) + (v8285 + (v792 * v6814))) + v8360)));
        let v9962: f64 = (self.scalar_v27 * (-v8382));
        let v9963: f64 = (self.scalar_v27 * (-((v2646 + v2646) / v337)));
        let v9964: f64 = (self.scalar_v27 * (-(v8383 + (v2640 + (v765 * v6111)))));
        let v9965: f64 = (self.scalar_v27 * (-((v8354 + (v2235 * v6572)) + (v765 * v6104))));
        let v9966: f64 = (self.scalar_v27 * (-v8385));
        let v9967: f64 = (self.scalar_v27 * (-(v8338 + v8360)));
        let v9968: f64 = (self.scalar_v27 * (-((v8339 + (v2636 + (v2232 * v6030))) + (v8359 + (v2235 * v6574)))));
        let v9969: f64 = (self.scalar_v27 * (-(((v8340 + ((v2232 * v6031) + (v1741 * self.scalar_v3270))) + ((v2235 * v6575) + v8372)) + ((v1761 * self.scalar_v3268) + (v765 * v6112)))));
        let v9970: f64 = (self.scalar_v27 * (-((((v8282 + (v8298 + (v792 * v6835))) + ((v2229 * v6135) + v8332)) + (v2232 * v6032)) + ((v2235 * v6576) + v8375))));
        let v9971: f64 = (self.scalar_v27 * (-(v8316 + (v8375 + (v2235 * v6577)))));
        let v9972: f64 = (self.scalar_v27 * (-(((v8284 + (v8298 + (v792 * v6841))) + (v8332 + (v2229 * v6136))) + (v8372 + (v2235 * v6578)))));
        let v9987: f64 = (self.scalar_v27 * (v9586 * (self.scalar_v0 * ((if self.scalar_v2541 { v8943 } else { (if self.scalar_v2488 { (self.scalar_v2528 * v8943) } else { v4 }) }) + (((v2242 * v4326) + (v1178 * v8401)) + v9566)))));
        let v9988: f64 = (self.scalar_v27 * (v9586 * (self.scalar_v0 * ((if self.scalar_v2541 { v8944 } else { (if self.scalar_v2488 { (self.scalar_v2528 * v8944) } else { v4 }) }) + ((v2242 * v4327) + v9567)))));
        let v9989: f64 = (self.scalar_v27 * (v9586 * (self.scalar_v0 * (v9568 + (if self.scalar_v2541 { v8945 } else { (if self.scalar_v2488 { (self.scalar_v2528 * v8945) } else { v4 }) })))));
        let v9990: f64 = (self.scalar_v27 * (v9586 * (self.scalar_v0 * ((if self.scalar_v2541 { v8946 } else { (if self.scalar_v2488 { (self.scalar_v2528 * v8946) } else { v4 }) }) + ((v2242 * v4328) + v9569)))));
        let v9991: f64 = (self.scalar_v27 * (v9586 * (self.scalar_v0 * (v9570 + v9584))));
        let v9992: f64 = (self.scalar_v27 * (v9586 * (self.scalar_v0 * (v9571 + v9584))));
        let v9993: f64 = (self.scalar_v27 * (v9586 * (self.scalar_v0 * (v9572 + (if self.scalar_v2541 { v8948 } else { (if self.scalar_v2488 { (self.scalar_v2528 * v8948) } else { v4 }) })))));
        let v10000: f64 = (self.scalar_v27 * (v9586 * (self.scalar_v0 * v8478)));
        let v10001: f64 = (self.scalar_v27 * (v9586 * (self.scalar_v0 * (v2261 * ((v1173 * (-((-(v308 * v8443)) * v8455))) + (v171 * (self.scalar_v3268 - v8443)))))));
        let v10002: f64 = (self.scalar_v27 * (v9586 * (self.scalar_v0 * (v2261 * ((v1173 * (-((-(v308 * v8444)) * v8455))) + (v171 * (self.scalar_v0 - v8444)))))));
        let v10017: f64 = (self.scalar_v27 * (v9586 * (self.scalar_v0 * v9750)));
        let v10018: f64 = (self.scalar_v27 * (v9586 * (self.scalar_v0 * v9574)));
        let v10019: f64 = (self.scalar_v27 * (v9586 * (self.scalar_v0 * v9575)));
        let v10020: f64 = (self.scalar_v27 * (v9586 * (self.scalar_v0 * (((v2399 * (v2397 * v4223)) + (v2398 * v4171)) + ((v2272 * v4517) + v9576)))));
        let v10021: f64 = (self.scalar_v27 * (v9586 * (self.scalar_v0 * (((v2399 * (v2397 * v4224)) + (v2398 * v4172)) + ((v2272 * v4518) + v9577)))));
        let v10022: f64 = (self.scalar_v27 * (v9586 * (self.scalar_v0 * (((v2399 * (v2397 * v4225)) + (v2398 * v4165)) + ((v2272 * v4512) + v9578)))));
        let v10023: f64 = (self.scalar_v27 * (v9586 * (self.scalar_v0 * v9579)));
        let v10030: f64 = (self.scalar_v27 * (v9586 * (self.scalar_v0 * (v320 * ((v2366 * (-((-(v8861 / v307)) * v8876))) + (v32 * (self.scalar_v0 - v8861)))))));
        let v10031: f64 = (self.scalar_v27 * (v9586 * (self.scalar_v0 * ((v2374 * (self.scalar_v316 * (((-(self.scalar_v285 * v2897)) / v2915) * (self.scalar_v318 * f64::powf(v317, self.scalar_v2917))))) + (v320 * v8895)))));
        let v10032: f64 = (self.scalar_v27 * (v9586 * (self.scalar_v0 * (v320 * ((v2366 * (-((-(v8863 / v307)) * v8876))) + (v32 * (self.scalar_v3268 - v8863)))))));
        let v10047: f64 = (self.scalar_v27 * (v9586 * (self.scalar_v0 * (if self.scalar_v2488 { (v2522 * ((if self.scalar_v2488 { (((v2382 * v8943) - (v2393 * v8916)) / v8919) } else { v4 }) + v9477)) } else { v4 }))));
        let v10048: f64 = (self.scalar_v27 * (v9586 * (self.scalar_v0 * v9497)));
        let v10049: f64 = (self.scalar_v27 * (v9586 * (self.scalar_v0 * (if self.scalar_v2488 { ((v2524 * self.scalar_v9475) + (v2522 * (if self.scalar_v2488 { (v8945 / v2382) } else { v4 }))) } else { v4 }))));
        let v10050: f64 = (self.scalar_v27 * (v9586 * (self.scalar_v0 * (if self.scalar_v2488 { v9492 } else { v4 }))));
        let v10051: f64 = (self.scalar_v27 * (v9586 * (self.scalar_v0 * (if self.scalar_v2488 { (v2522 * ((if self.scalar_v2488 { (v2516 * (v2275 * v6897)) } else { v4 }) + v9473)) } else { v4 }))));
        let v10052: f64 = (self.scalar_v27 * (v9586 * (self.scalar_v0 * (if self.scalar_v2488 { (v2522 * ((if self.scalar_v2488 { (v2516 * (v2275 * v6898)) } else { v4 }) + v9473)) } else { v4 }))));
        let v10053: f64 = (self.scalar_v27 * (v9586 * (self.scalar_v0 * (if self.scalar_v2488 { (v2522 * (if self.scalar_v2488 { (v8948 / v2382) } else { v4 })) } else { v4 }))));
        let v10058: f64 = (self.scalar_v27 * (v9586 * self.scalar_v10054));
        let v10059: f64 = (self.scalar_v27 * (v9586 * self.scalar_v10055));
        let v10064: f64 = (self.scalar_v27 * (v9586 * self.scalar_v10060));
        let v10065: f64 = (self.scalar_v27 * (v9586 * self.scalar_v10061));
        let v10077: f64 = (self.scalar_v27 * (self.scalar_v0 * v6814));
        let v10078: f64 = (self.scalar_v27 * (self.scalar_v0 * v6817));
        let v10079: f64 = (self.scalar_v27 * (self.scalar_v0 * v6818));
        let v10080: f64 = (self.scalar_v27 * (self.scalar_v0 * v6821));
        let v10081: f64 = (self.scalar_v27 * (self.scalar_v0 * v6824));
        let v10082: f64 = (self.scalar_v27 * (self.scalar_v0 * v6826));
        let v10083: f64 = (self.scalar_v27 * (self.scalar_v0 * v6829));
        let v10084: f64 = (self.scalar_v27 * (self.scalar_v0 * v6832));
        let v10085: f64 = (self.scalar_v27 * (self.scalar_v0 * v6835));
        let v10086: f64 = (self.scalar_v27 * (self.scalar_v0 * v6838));
        let v10087: f64 = (self.scalar_v27 * (self.scalar_v0 * v6841));
        let v10095: f64 = (self.scalar_v27 * (v731 * self.scalar_v9931));
        let v10096: f64 = (self.scalar_v27 * (v731 * self.scalar_v10088));
        let v10097: f64 = (self.scalar_v27 * (v2679 * v3255));
        let v10098: f64 = (self.scalar_v27 * (v731 * self.scalar_v10089));
        let v10099: f64 = (self.scalar_v27 * (v731 * self.scalar_v9932));
        let v10101: f64 = (self.scalar_v0 * ((self.scalar_v13 * v8808) + (if self.scalar_v2436 { ((v2484 * v6501) + (v1842 * (if self.scalar_v2466 { v9286 } else { (if self.scalar_v2439 { ((v2456 * v9210) / v690) } else { v4 }) }))) } else { v4 })));
        let v10118: f64 = (v9586 * (self.scalar_v0 * (v8816 + (if self.scalar_v2436 { ((v2484 * v6508) + (v1842 * (if self.scalar_v2466 { v9298 } else { (if self.scalar_v2439 { ((v2456 * v9213) / v690) } else { v4 }) }))) } else { v4 }))));
        let v10120: f64 = (self.scalar_v27 * (v9586 * (self.scalar_v0 * (v8812 + (if self.scalar_v2436 { ((v2484 * v6500) + v9305) } else { v4 })))));
        let v10121: f64 = (self.scalar_v27 * (v9586 * v10101));
        let v10122: f64 = (self.scalar_v27 * (v9586 * (self.scalar_v0 * (if self.scalar_v2436 { (v2484 * v6502) } else { v4 }))));
        let v10123: f64 = (self.scalar_v27 * (v9586 * (self.scalar_v0 * v9761)));
        let v10124: f64 = (self.scalar_v27 * (v9586 * (self.scalar_v0 * (if self.scalar_v2436 { (v2484 * v6504) } else { v4 }))));
        let v10125: f64 = (self.scalar_v27 * (v9586 * (self.scalar_v0 * (v8812 + (if self.scalar_v2436 { (v9305 + (v2484 * v6505)) } else { v4 })))));
        let v10126: f64 = (self.scalar_v27 * (v9586 * (self.scalar_v0 * (v8815 + (if self.scalar_v2436 { ((v2484 * v6506) + v9318) } else { v4 })))));
        let v10127: f64 = (self.scalar_v27 * (v9586 * (self.scalar_v0 * (v8815 + (if self.scalar_v2436 { (v9318 + (v2484 * v6507)) } else { v4 })))));
        let v10128: f64 = (self.scalar_v27 * v10118);
        let v10129: f64 = (self.scalar_v27 * (v9586 * (self.scalar_v0 * (v8815 + (if self.scalar_v2436 { (v9318 + (v2484 * v6509)) } else { v4 })))));
        let v10145: f64 = (self.scalar_v27 * (self.scalar_v0 * v8239));
        let v10146: f64 = (self.scalar_v27 * (self.scalar_v0 * v8240));
        let v10147: f64 = (self.scalar_v27 * (self.scalar_v0 * v8241));
        let v10148: f64 = (self.scalar_v27 * (self.scalar_v0 * v8242));
        let v10149: f64 = (self.scalar_v27 * (self.scalar_v0 * (v6776 + (self.scalar_v6109 + v6799))));
        let v10150: f64 = (self.scalar_v27 * (self.scalar_v0 * (v6779 + (v6802 + self.scalar_v8249))));
        let v10151: f64 = (self.scalar_v27 * (self.scalar_v0 * (v6782 + (v6805 + self.scalar_v8250))));
        let v10152: f64 = (self.scalar_v27 * (self.scalar_v0 * (v6784 + (v6807 + self.scalar_v8250))));
        let v10153: f64 = (self.scalar_v27 * (self.scalar_v0 * v8247));
        let v10154: f64 = (self.scalar_v27 * (self.scalar_v0 * (v6788 + (self.scalar_v6110 + v6811))));
        let v10165: f64 = (self.scalar_v27 * (v9586 * (self.scalar_v0 * v9754)));
        let v10166: f64 = (self.scalar_v27 * (v9586 * (self.scalar_v0 * (v8673 + (if self.scalar_v2436 { (self.scalar_v14 * v9093) } else { v9093 })))));
        let v10167: f64 = (self.scalar_v27 * (v9586 * (self.scalar_v0 * (v8674 + (if self.scalar_v2436 { (self.scalar_v14 * v9094) } else { v9094 })))));
        let v10168: f64 = (self.scalar_v27 * (v9586 * (self.scalar_v0 * (v8675 + (if self.scalar_v2436 { (self.scalar_v14 * v9095) } else { v9095 })))));
        let v10169: f64 = (self.scalar_v27 * (v9586 * (self.scalar_v0 * (v8676 + (if self.scalar_v2436 { (self.scalar_v14 * v9096) } else { v9096 })))));
        let v10176: f64 = (if self.scalar_v732 { (self.scalar_v27 * (v2691 * v3261)) } else { v4 });
        let v10177: f64 = (if self.scalar_v732 { (self.scalar_v27 * (v739 * self.scalar_v9931)) } else { v4 });
        let v10178: f64 = (if self.scalar_v732 { (self.scalar_v27 * (v739 * self.scalar_v9932)) } else { v4 });
        let v10185: f64 = (if self.scalar_v740 { (self.scalar_v27 * (v2695 * v3267)) } else { v4 });
        let v10186: f64 = (if self.scalar_v740 { (self.scalar_v27 * (v747 * self.scalar_v9932)) } else { v4 });
        let v10187: f64 = (if self.scalar_v740 { (self.scalar_v27 * (v747 * self.scalar_v9931)) } else { v4 });
        let v10188: f64 = (v2700 * (if self.scalar_v2609 { v4 } else { (if self.scalar_v2604 { (self.scalar_v2605 * v9684) } else { (if self.scalar_v2599 { (self.scalar_v2534 * v9684) } else { v4 }) }) }));
        let v10189: f64 = (v2700 * (if self.scalar_v2609 { v4 } else { (if self.scalar_v2604 { (self.scalar_v2605 * v9685) } else { (if self.scalar_v2599 { (self.scalar_v2534 * v9685) } else { v4 }) }) }));
        let v10190: f64 = (v2700 * (if self.scalar_v2609 { v4 } else { (if self.scalar_v2604 { (self.scalar_v2605 * v9686) } else { (if self.scalar_v2599 { (self.scalar_v2534 * v9686) } else { v4 }) }) }));
        let v10191: f64 = (v2700 * (if self.scalar_v2609 { v4 } else { (if self.scalar_v2604 { (self.scalar_v2605 * v9687) } else { (if self.scalar_v2599 { (self.scalar_v2534 * v9687) } else { v4 }) }) }));
        let v10192: f64 = (v2700 * (if self.scalar_v2609 { v4 } else { (if self.scalar_v2604 { (self.scalar_v2605 * v9688) } else { (if self.scalar_v2599 { (self.scalar_v2534 * v9688) } else { v4 }) }) }));
        let v10193: f64 = (v2700 * (if self.scalar_v2609 { v4 } else { (if self.scalar_v2604 { (self.scalar_v2605 * v9689) } else { (if self.scalar_v2599 { (self.scalar_v2534 * v9689) } else { v4 }) }) }));
        let v10194: f64 = (v2700 * (if self.scalar_v2609 { v4 } else { (if self.scalar_v2604 { (self.scalar_v2605 * v9690) } else { (if self.scalar_v2599 { (self.scalar_v2534 * v9690) } else { v4 }) }) }));
        let v10195: f64 = (v2610 * v9586);

        let d2622_dn4: f64 = v9771;
        let d2622_dn7: f64 = v9772;
        let d2622_dn8: f64 = v9773;
        let d2622_dn9: f64 = v9774;
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(8),
            Some(9),
            multiplicity * (v2622),
            [4, 7, 8, 9],
            [d2622_dn4, d2622_dn7, d2622_dn8, d2622_dn9],
            [],
            [],
            multiplicity,
        );
        let d2624_dn4: f64 = v9780;
        let d2624_dn5: f64 = v9781;
        let d2624_dn7: f64 = v9782;
        let d2624_dn8: f64 = v9783;
        let d2624_dn9: f64 = v9784;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(9),
            Some(5),
            multiplicity * (v2624),
            [4, 5, 7, 8, 9],
            [d2624_dn4, d2624_dn5, d2624_dn7, d2624_dn8, d2624_dn9],
            [],
            [],
            multiplicity,
        );
        let d2626_dn4: f64 = v9791;
        let d2626_dn5: f64 = v9792;
        let d2626_dn6: f64 = v9793;
        let d2626_dn7: f64 = v9794;
        let d2626_dn8: f64 = v9795;
        let d2626_dn9: f64 = v9795;
        let d2626_dn11: f64 = v9796;
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(6),
            Some(5),
            multiplicity * (v2626),
            [4, 5, 6, 7, 8, 9, 11],
            [d2626_dn4, d2626_dn5, d2626_dn6, d2626_dn7, d2626_dn8, d2626_dn9, d2626_dn11],
            [],
            [],
            multiplicity,
        );
        let d2628_dn4: f64 = v9803;
        let d2628_dn5: f64 = v9804;
        let d2628_dn6: f64 = v9805;
        let d2628_dn7: f64 = v9806;
        let d2628_dn8: f64 = v9807;
        let d2628_dn9: f64 = v9808;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(7),
            Some(5),
            multiplicity * (v2628),
            [4, 5, 6, 7, 8, 9],
            [d2628_dn4, d2628_dn5, d2628_dn6, d2628_dn7, d2628_dn8, d2628_dn9],
            [],
            [],
            multiplicity,
        );
        let d2632_dn0: f64 = v9839;
        let d2632_dn1: f64 = v9840;
        let d2632_dn4: f64 = v9841;
        let d2632_dn5: f64 = v9842;
        let d2632_dn6: f64 = v9843;
        let d2632_dn7: f64 = v9844;
        let d2632_dn8: f64 = v9845;
        let d2632_dn9: f64 = v9846;
        let d2632_dn10: f64 = v9847;
        let d2632_dn11: f64 = v9848;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(7),
            Some(8),
            multiplicity * (v2632),
            [0, 1, 4, 5, 6, 7, 8, 9, 10, 11],
            [d2632_dn0, d2632_dn1, d2632_dn4, d2632_dn5, d2632_dn6, d2632_dn7, d2632_dn8, d2632_dn9, d2632_dn10, d2632_dn11],
            [],
            [],
            multiplicity,
        );
        let d2633_dn0: f64 = v9849;
        let d2633_dn1: f64 = v9850;
        let d2633_dn4: f64 = v9851;
        let d2633_dn5: f64 = v9852;
        let d2633_dn6: f64 = v9853;
        let d2633_dn7: f64 = v9854;
        let d2633_dn8: f64 = v9855;
        let d2633_dn9: f64 = v9856;
        let d2633_dn10: f64 = v9857;
        let d2633_dn11: f64 = v9858;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(7),
            Some(9),
            multiplicity * (v2633),
            [0, 1, 4, 5, 6, 7, 8, 9, 10, 11],
            [d2633_dn0, d2633_dn1, d2633_dn4, d2633_dn5, d2633_dn6, d2633_dn7, d2633_dn8, d2633_dn9, d2633_dn10, d2633_dn11],
            [],
            [],
            multiplicity,
        );
        let d2635_dn3: f64 = v9866;
        let d2635_dn4: f64 = v9867;
        let d2635_dn6: f64 = v9868;
        let d2635_dn7: f64 = v9869;
        let d2635_dn8: f64 = v9870;
        let d2635_dn9: f64 = v9871;
        let d2635_dn11: f64 = v9872;
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(6),
            Some(3),
            multiplicity * (v2635),
            [3, 4, 6, 7, 8, 9, 11],
            [d2635_dn3, d2635_dn4, d2635_dn6, d2635_dn7, d2635_dn8, d2635_dn9, d2635_dn11],
            [],
            [],
            multiplicity,
        );
        let d2637_dn3: f64 = v9878;
        let d2637_dn4: f64 = v9879;
        let d2637_dn7: f64 = v9880;
        let d2637_dn8: f64 = v9881;
        let d2637_dn9: f64 = v9882;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(7),
            Some(3),
            multiplicity * (v2637),
            [3, 4, 7, 8, 9],
            [d2637_dn3, d2637_dn4, d2637_dn7, d2637_dn8, d2637_dn9],
            [],
            [],
            multiplicity,
        );
        let d2639_dn0: f64 = v9893;
        let d2639_dn1: f64 = v9894;
        let d2639_dn3: f64 = v9895;
        let d2639_dn4: f64 = v9896;
        let d2639_dn5: f64 = v9897;
        let d2639_dn6: f64 = v9893;
        let d2639_dn7: f64 = v9898;
        let d2639_dn8: f64 = v9899;
        let d2639_dn9: f64 = v9900;
        let d2639_dn10: f64 = v9901;
        let d2639_dn11: f64 = v9902;
        let v2639_node_derivative_indices: [usize; 11] = [0, 1, 3, 4, 5, 6, 7, 8, 9, 10, 11];
        let v2639_node_derivatives: [f64; 11] = [d2639_dn0, d2639_dn1, d2639_dn3, d2639_dn4, d2639_dn5, d2639_dn6, d2639_dn7, d2639_dn8, d2639_dn9, d2639_dn10, d2639_dn11];
        let v2639_branch_derivative_indices: [usize; 0] = [];
        let v2639_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(1),
            Some(3),
            multiplicity * (v2639),
            &v2639_node_derivative_indices,
            &v2639_node_derivatives,
            &v2639_branch_derivative_indices,
            &v2639_branch_derivatives,
            multiplicity,
        );
        let d2641_dn3: f64 = v9906;
        let d2641_dn4: f64 = v9907;
        let d2641_dn8: f64 = v9908;
        stamper.stamp_current_node3_local(
            Some(3),
            Some(8),
            multiplicity * (v2641),
            3,
            multiplicity * (d2641_dn3),
            4,
            multiplicity * (d2641_dn4),
            8,
            multiplicity * (d2641_dn8),
        );
        let d2643_dn4: f64 = v9915;
        let d2643_dn5: f64 = v9916;
        let d2643_dn6: f64 = v9917;
        let d2643_dn7: f64 = v9918;
        let d2643_dn8: f64 = v9919;
        let d2643_dn9: f64 = v9920;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(6),
            Some(7),
            multiplicity * (v2643),
            [4, 5, 6, 7, 8, 9],
            [d2643_dn4, d2643_dn5, d2643_dn6, d2643_dn7, d2643_dn8, d2643_dn9],
            [],
            [],
            multiplicity,
        );
        let d2645_dn4: f64 = v9926;
        let d2645_dn5: f64 = v9927;
        let d2645_dn7: f64 = v9928;
        let d2645_dn8: f64 = v9929;
        let d2645_dn9: f64 = v9930;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(7),
            Some(9),
            multiplicity * (v2645),
            [4, 5, 7, 8, 9],
            [d2645_dn4, d2645_dn5, d2645_dn7, d2645_dn8, d2645_dn9],
            [],
            [],
            multiplicity,
        );
        let d2648_dn2: f64 = v9938;
        let d2648_dn4: f64 = v9939;
        let d2648_dn5: f64 = v9940;
        stamper.stamp_current_node3_local(
            Some(2),
            Some(5),
            multiplicity * (v2648),
            2,
            multiplicity * (d2648_dn2),
            4,
            multiplicity * (d2648_dn4),
            5,
            multiplicity * (d2648_dn5),
        );
        let d2651_dn1: f64 = v9946;
        let d2651_dn4: f64 = v9947;
        let d2651_dn6: f64 = v9948;
        stamper.stamp_current_node3_local(
            Some(1),
            Some(6),
            multiplicity * (v2651),
            1,
            multiplicity * (d2651_dn1),
            4,
            multiplicity * (d2651_dn4),
            6,
            multiplicity * (d2651_dn6),
        );
        let d2580_dn4: f64 = v9603;
        stamper.stamp_current_node1_local(
            Some(4),
            None,
            multiplicity * (v2580),
            4,
            multiplicity * (d2580_dn4),
        );
        let d2548_dn4: f64 = v9588;
        stamper.stamp_current_node1_local(
            Some(4),
            None,
            multiplicity * (v2548),
            4,
            multiplicity * (d2548_dn4),
        );
        let d2653_dn0: f64 = v9961;
        let d2653_dn1: f64 = v9962;
        let d2653_dn2: f64 = v9963;
        let d2653_dn3: f64 = v9964;
        let d2653_dn4: f64 = v9965;
        let d2653_dn5: f64 = v9966;
        let d2653_dn6: f64 = v9967;
        let d2653_dn7: f64 = v9968;
        let d2653_dn8: f64 = v9969;
        let d2653_dn9: f64 = v9970;
        let d2653_dn10: f64 = v9971;
        let d2653_dn11: f64 = v9972;
        let v2653_node_derivative_indices: [usize; 12] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
        let v2653_node_derivatives: [f64; 12] = [d2653_dn0, d2653_dn1, d2653_dn2, d2653_dn3, d2653_dn4, d2653_dn5, d2653_dn6, d2653_dn7, d2653_dn8, d2653_dn9, d2653_dn10, d2653_dn11];
        let v2653_branch_derivative_indices: [usize; 0] = [];
        let v2653_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(4),
            None,
            multiplicity * (v2653),
            &v2653_node_derivative_indices,
            &v2653_node_derivatives,
            &v2653_branch_derivative_indices,
            &v2653_branch_derivatives,
            multiplicity,
        );
        let d2656_dn4: f64 = v9987;
        let d2656_dn5: f64 = v9988;
        let d2656_dn6: f64 = v9989;
        let d2656_dn7: f64 = v9990;
        let d2656_dn8: f64 = v9991;
        let d2656_dn9: f64 = v9992;
        let d2656_dn11: f64 = v9993;
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(7),
            Some(5),
            multiplicity * (v2656),
            [4, 5, 6, 7, 8, 9, 11],
            [d2656_dn4, d2656_dn5, d2656_dn6, d2656_dn7, d2656_dn8, d2656_dn9, d2656_dn11],
            [],
            [],
            multiplicity,
        );
        let d2659_dn4: f64 = v10000;
        let d2659_dn5: f64 = v10001;
        let d2659_dn6: f64 = v10002;
        stamper.stamp_current_node3_local(
            Some(6),
            Some(5),
            multiplicity * (v2659),
            4,
            multiplicity * (d2659_dn4),
            5,
            multiplicity * (d2659_dn5),
            6,
            multiplicity * (d2659_dn6),
        );
        let d2662_dn4: f64 = v10017;
        let d2662_dn5: f64 = v10018;
        let d2662_dn6: f64 = v10019;
        let d2662_dn7: f64 = v10020;
        let d2662_dn8: f64 = v10021;
        let d2662_dn9: f64 = v10022;
        let d2662_dn11: f64 = v10023;
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(7),
            Some(9),
            multiplicity * (v2662),
            [4, 5, 6, 7, 8, 9, 11],
            [d2662_dn4, d2662_dn5, d2662_dn6, d2662_dn7, d2662_dn8, d2662_dn9, d2662_dn11],
            [],
            [],
            multiplicity,
        );
        let d2665_dn3: f64 = v10030;
        let d2665_dn4: f64 = v10031;
        let d2665_dn8: f64 = v10032;
        stamper.stamp_current_node3_local(
            Some(3),
            Some(8),
            multiplicity * (v2665),
            3,
            multiplicity * (d2665_dn3),
            4,
            multiplicity * (d2665_dn4),
            8,
            multiplicity * (d2665_dn8),
        );
        let d2668_dn4: f64 = v10047;
        let d2668_dn5: f64 = v10048;
        let d2668_dn6: f64 = v10049;
        let d2668_dn7: f64 = v10050;
        let d2668_dn8: f64 = v10051;
        let d2668_dn9: f64 = v10052;
        let d2668_dn11: f64 = v10053;
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(6),
            Some(7),
            multiplicity * (v2668),
            [4, 5, 6, 7, 8, 9, 11],
            [d2668_dn4, d2668_dn5, d2668_dn6, d2668_dn7, d2668_dn8, d2668_dn9, d2668_dn11],
            [],
            [],
            multiplicity,
        );
        let d2672_dn1: f64 = v10058;
        let d2672_dn2: f64 = v10059;
        stamper.stamp_current_node2_local(
            Some(1),
            Some(2),
            multiplicity * (v2672),
            1,
            multiplicity * (d2672_dn1),
            2,
            multiplicity * (d2672_dn2),
        );
        let d2676_dn0: f64 = v10064;
        let d2676_dn1: f64 = v10065;
        stamper.stamp_current_node2_local(
            Some(1),
            Some(0),
            multiplicity * (v2676),
            0,
            multiplicity * (d2676_dn0),
            1,
            multiplicity * (d2676_dn1),
        );
        let d2678_dn0: f64 = v10077;
        let d2678_dn1: f64 = v10078;
        let d2678_dn3: f64 = v10079;
        let d2678_dn4: f64 = v10080;
        let d2678_dn5: f64 = v10081;
        let d2678_dn6: f64 = v10082;
        let d2678_dn7: f64 = v10083;
        let d2678_dn8: f64 = v10084;
        let d2678_dn9: f64 = v10085;
        let d2678_dn10: f64 = v10086;
        let d2678_dn11: f64 = v10087;
        let v2678_node_derivative_indices: [usize; 11] = [0, 1, 3, 4, 5, 6, 7, 8, 9, 10, 11];
        let v2678_node_derivatives: [f64; 11] = [d2678_dn0, d2678_dn1, d2678_dn3, d2678_dn4, d2678_dn5, d2678_dn6, d2678_dn7, d2678_dn8, d2678_dn9, d2678_dn10, d2678_dn11];
        let v2678_branch_derivative_indices: [usize; 0] = [];
        let v2678_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(1),
            Some(10),
            multiplicity * (v2678),
            &v2678_node_derivative_indices,
            &v2678_node_derivatives,
            &v2678_branch_derivative_indices,
            &v2678_branch_derivatives,
            multiplicity,
        );
        let d2681_dn0: f64 = v10095;
        let d2681_dn1: f64 = v10096;
        let d2681_dn4: f64 = v10097;
        let d2681_dn6: f64 = v10096;
        let d2681_dn7: f64 = v10096;
        let d2681_dn8: f64 = v10098;
        let d2681_dn9: f64 = v10098;
        let d2681_dn10: f64 = v10099;
        let d2681_dn11: f64 = v10098;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(0),
            Some(10),
            multiplicity * (v2681),
            [0, 1, 4, 6, 7, 8, 9, 10, 11],
            [d2681_dn0, d2681_dn1, d2681_dn4, d2681_dn6, d2681_dn7, d2681_dn8, d2681_dn9, d2681_dn10, d2681_dn11],
            [],
            [],
            multiplicity,
        );
        let d2684_dn0: f64 = v10120;
        let d2684_dn1: f64 = v10121;
        let d2684_dn3: f64 = v10122;
        let d2684_dn4: f64 = v10123;
        let d2684_dn5: f64 = v10124;
        let d2684_dn6: f64 = v10120;
        let d2684_dn7: f64 = v10125;
        let d2684_dn8: f64 = v10126;
        let d2684_dn9: f64 = v10127;
        let d2684_dn10: f64 = v10128;
        let d2684_dn11: f64 = v10129;
        let v2684_node_derivative_indices: [usize; 11] = [0, 1, 3, 4, 5, 6, 7, 8, 9, 10, 11];
        let v2684_node_derivatives: [f64; 11] = [d2684_dn0, d2684_dn1, d2684_dn3, d2684_dn4, d2684_dn5, d2684_dn6, d2684_dn7, d2684_dn8, d2684_dn9, d2684_dn10, d2684_dn11];
        let v2684_branch_derivative_indices: [usize; 0] = [];
        let v2684_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(1),
            Some(10),
            multiplicity * (v2684),
            &v2684_node_derivative_indices,
            &v2684_node_derivatives,
            &v2684_branch_derivative_indices,
            &v2684_branch_derivatives,
            multiplicity,
        );
        let d2687_dn0: f64 = v10145;
        let d2687_dn1: f64 = v10146;
        let d2687_dn4: f64 = v10147;
        let d2687_dn5: f64 = v10148;
        let d2687_dn6: f64 = v10149;
        let d2687_dn7: f64 = v10150;
        let d2687_dn8: f64 = v10151;
        let d2687_dn9: f64 = v10152;
        let d2687_dn10: f64 = v10153;
        let d2687_dn11: f64 = v10154;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(6),
            Some(11),
            multiplicity * (v2687),
            [0, 1, 4, 5, 6, 7, 8, 9, 10, 11],
            [d2687_dn0, d2687_dn1, d2687_dn4, d2687_dn5, d2687_dn6, d2687_dn7, d2687_dn8, d2687_dn9, d2687_dn10, d2687_dn11],
            [],
            [],
            multiplicity,
        );
        let d2690_dn4: f64 = v10165;
        let d2690_dn6: f64 = v10166;
        let d2690_dn7: f64 = v10167;
        let d2690_dn8: f64 = v10168;
        let d2690_dn9: f64 = v10168;
        let d2690_dn11: f64 = v10169;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(6),
            Some(11),
            multiplicity * (v2690),
            [4, 6, 7, 8, 9, 11],
            [d2690_dn4, d2690_dn6, d2690_dn7, d2690_dn8, d2690_dn9, d2690_dn11],
            [],
            [],
            multiplicity,
        );
        let d2694_dn4: f64 = v10176;
        let d2694_dn10: f64 = v10177;
        let d2694_dn11: f64 = v10178;
        stamper.stamp_current_node3_local(
            Some(10),
            Some(11),
            multiplicity * (v2694),
            4,
            multiplicity * (d2694_dn4),
            10,
            multiplicity * (d2694_dn10),
            11,
            multiplicity * (d2694_dn11),
        );
        stamper.stamp_potential_branch_local(
            Some(10),
            Some(11),
            0,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            0,
            v4,
        );
        let d2698_dn4: f64 = v10185;
        let d2698_dn8: f64 = v10186;
        let d2698_dn11: f64 = v10187;
        stamper.stamp_current_node3_local(
            Some(11),
            Some(8),
            multiplicity * (v2698),
            4,
            multiplicity * (d2698_dn4),
            8,
            multiplicity * (d2698_dn8),
            11,
            multiplicity * (d2698_dn11),
        );
        stamper.stamp_potential_branch_local(
            Some(11),
            Some(8),
            1,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            1,
            v4,
        );
        stamper.stamp_current_const_local(
            Some(12),
            None,
            multiplicity * (v4),
        );
        let d2699_dn12: f64 = v1;
        stamper.stamp_current_node1_local(
            Some(12),
            None,
            multiplicity * (v2699),
            12,
            multiplicity * (d2699_dn12),
        );
        let d2701_dn4: f64 = v10188;
        let d2701_dn5: f64 = v10189;
        let d2701_dn6: f64 = v10190;
        let d2701_dn7: f64 = v10191;
        let d2701_dn8: f64 = v10192;
        let d2701_dn9: f64 = v10193;
        let d2701_dn11: f64 = v10194;
        let d2701_dn12: f64 = v10195;
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(7),
            Some(5),
            multiplicity * (v2701),
            [4, 5, 6, 7, 8, 9, 11, 12],
            [d2701_dn4, d2701_dn5, d2701_dn6, d2701_dn7, d2701_dn8, d2701_dn9, d2701_dn11, d2701_dn12],
            [],
            [],
            multiplicity,
        );
        let d2702_dn12: f64 = v2589;
        stamper.stamp_current_node1_local(
            Some(9),
            Some(7),
            multiplicity * (v2702),
            12,
            multiplicity * (d2702_dn12),
        );
        let d2699_dn12: f64 = v1;
        stamper.stamp_current_node1_local(
            Some(9),
            Some(5),
            multiplicity * (v2699),
            12,
            multiplicity * (d2699_dn12),
        );
        stamper.stamp_current_const_local(
            Some(9),
            Some(7),
            multiplicity * (v4),
        );
        stamper.stamp_current_const_local(
            Some(7),
            Some(5),
            multiplicity * (v4),
        );
        stamper.stamp_current_const_local(
            Some(2),
            Some(5),
            multiplicity * (v4),
        );
        stamper.stamp_current_const_local(
            Some(1),
            Some(6),
            multiplicity * (v4),
        );
        stamper.stamp_current_const_local(
            Some(6),
            Some(7),
            multiplicity * (v4),
        );
        stamper.stamp_current_const_local(
            Some(7),
            Some(5),
            multiplicity * (v4),
        );
        stamper.stamp_current_const_local(
            Some(6),
            Some(5),
            multiplicity * (v4),
        );
        stamper.stamp_current_const_local(
            Some(6),
            Some(5),
            multiplicity * (v4),
        );
        stamper.stamp_current_const_local(
            Some(6),
            Some(11),
            multiplicity * (v4),
        );
        stamper.stamp_current_const_local(
            Some(6),
            Some(11),
            multiplicity * (v4),
        );
        stamper.stamp_current_const_local(
            Some(6),
            Some(11),
            multiplicity * (v4),
        );
        stamper.stamp_current_const_local(
            Some(6),
            Some(11),
            multiplicity * (v4),
        );
        stamper.stamp_current_const_local(
            Some(1),
            Some(10),
            multiplicity * (v4),
        );
        stamper.stamp_current_const_local(
            Some(1),
            Some(10),
            multiplicity * (v4),
        );
        stamper.stamp_current_const_local(
            Some(8),
            Some(7),
            multiplicity * (v4),
        );
        stamper.stamp_current_const_local(
            Some(9),
            Some(7),
            multiplicity * (v4),
        );
        stamper.stamp_current_const_local(
            Some(7),
            Some(3),
            multiplicity * (v4),
        );
        stamper.stamp_current_const_local(
            Some(6),
            Some(3),
            multiplicity * (v4),
        );
        stamper.stamp_current_const_local(
            Some(1),
            Some(3),
            multiplicity * (v4),
        );
        stamper.stamp_current_const_local(
            Some(0),
            Some(10),
            multiplicity * (v4),
        );
        stamper.stamp_current_const_local(
            Some(10),
            Some(11),
            multiplicity * (v4),
        );
        stamper.stamp_current_const_local(
            Some(11),
            Some(8),
            multiplicity * (v4),
        );
        stamper.stamp_current_const_local(
            Some(0),
            Some(10),
            multiplicity * (v4),
        );
        stamper.stamp_current_const_local(
            Some(10),
            Some(8),
            multiplicity * (v4),
        );
        stamper.stamp_current_const_local(
            Some(0),
            Some(11),
            multiplicity * (v4),
        );
        stamper.stamp_current_const_local(
            Some(11),
            Some(8),
            multiplicity * (v4),
        );
        stamper.stamp_current_const_local(
            Some(0),
            Some(8),
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
        let v31: f64 = 0.001;
        let v32: f64 = 2.0;
        let v45: f64 = 0.05;
        let v47: f64 = 0.1;
        let v102: f64 = ctx.node_voltage(nodes[4]);
        let v103: bool = (v102 < v4);
        let v104: f64 = (v1 - v102);
        let v107: f64 = (if v103 { (-((v104) as f64).ln()) } else { v102 });
        let v109: bool = (v107 < self.scalar_v108);
        let v111: bool = (!v109);
        let v113: f64 = (v1 + (v107 - self.scalar_v108));
        let v117: f64 = (self.scalar_v20 + (if v111 { (self.scalar_v108 + ((v113) as f64).ln()) } else { (if v109 { v107 } else { v4 }) }));
        let v118: f64 = (v117 / self.scalar_v17);
        let v119: f64 = 8.617086918058125e-5;
        let v120: f64 = (v117 * v119);
        let v122: f64 = (v1 / v120);
        let v124: f64 = (v122 - self.scalar_v123);
        let v125: f64 = (v117 - self.scalar_v17);
        let v126: f64 = ((v118) as f64).ln();
        let v131: f64 = (self.scalar_v63 - ((v117 * (self.scalar_v38 * v117)) / (self.scalar_v41 + v117)));
        let v133: f64 = ((v131 - v45) / v47);
        let v134: bool = (v131 < v45);
        let v148: f64 = (if (!v134) { (v131 + (v47 * (((v1 + (((-v133)) as f64).exp())) as f64).ln())) } else { (if v134 { (v45 + (v47 * (((v1 + ((v133) as f64).exp())) as f64).ln())) } else { v4 }) });
        let v153: f64 = (self.scalar_v96 - ((v117 * (self.scalar_v73 * v117)) / (self.scalar_v76 + v117)));
        let v155: f64 = ((v153 - v45) / v47);
        let v156: bool = (v153 < v45);
        let v170: f64 = (if (!v156) { (v153 + (v47 * (((v1 + (((-v155)) as f64).exp())) as f64).ln())) } else { (if v156 { (v45 + (v47 * (((v1 + ((v155) as f64).exp())) as f64).ln())) } else { v4 }) });
        let v171: f64 = 3.0;
        let v172: f64 = -3.0;
        let v173: f64 = (v120 * v172);
        let v174: f64 = (v126 * v173);
        let v177: f64 = (v1 - v118);
        let v180: f64 = ((v174 + (self.scalar_v65 * v118)) + (v177 * self.scalar_v178));
        let v181: f64 = (v45 - v180);
        let v182: f64 = (v181 / v120);
        let v183: bool = (v45 < v180);
        let v184: f64 = ((v182) as f64).exp();
        let v185: f64 = (v1 + v184);
        let v186: f64 = ((v185) as f64).ln();
        let v190: bool = (!v183);
        let v192: f64 = (((-v182)) as f64).exp();
        let v193: f64 = (v1 + v192);
        let v194: f64 = ((v193) as f64).ln();
        let v197: f64 = (if v190 { (v45 + (v120 * v194)) } else { (if v183 { (v180 + (v120 * v186)) } else { v4 }) });
        let v202: f64 = (v177 * self.scalar_v201);
        let v203: f64 = ((v174 + (v118 * self.scalar_v198)) + v202);
        let v204: f64 = (v45 - v203);
        let v205: f64 = (v204 / v120);
        let v206: bool = (v45 < v203);
        let v207: f64 = ((v205) as f64).exp();
        let v208: f64 = (v1 + v207);
        let v209: f64 = ((v208) as f64).ln();
        let v213: bool = (!v206);
        let v215: f64 = (((-v205)) as f64).exp();
        let v216: f64 = (v1 + v215);
        let v217: f64 = ((v216) as f64).ln();
        let v220: f64 = (if v213 { (v45 + (v120 * v217)) } else { (if v206 { (v203 + (v120 * v209)) } else { v4 }) });
        let v224: f64 = (v202 + (v174 + (v118 * self.scalar_v221)));
        let v225: f64 = (v45 - v224);
        let v226: f64 = (v225 / v120);
        let v227: bool = (v45 < v224);
        let v228: f64 = ((v226) as f64).exp();
        let v229: f64 = (v1 + v228);
        let v230: f64 = ((v229) as f64).ln();
        let v234: bool = (!v227);
        let v236: f64 = (((-v226)) as f64).exp();
        let v237: f64 = (v1 + v236);
        let v238: f64 = ((v237) as f64).ln();
        let v241: f64 = (if v234 { (v45 + (v120 * v238)) } else { (if v227 { (v224 + (v120 * v230)) } else { v4 }) });
        let v244: f64 = (v202 + (v174 + (self.scalar_v67 * v118)));
        let v245: f64 = (v45 - v244);
        let v246: f64 = (v245 / v120);
        let v247: bool = (v45 < v244);
        let v248: f64 = ((v246) as f64).exp();
        let v249: f64 = (v1 + v248);
        let v250: f64 = ((v249) as f64).ln();
        let v254: bool = (!v247);
        let v256: f64 = (((-v246)) as f64).exp();
        let v257: f64 = (v1 + v256);
        let v258: f64 = ((v257) as f64).ln();
        let v261: f64 = (if v254 { (v45 + (v120 * v258)) } else { (if v247 { (v244 + (v120 * v250)) } else { v4 }) });
        let v267: f64 = ((v174 + (v118 * self.scalar_v262)) + (v177 * self.scalar_v265));
        let v269: f64 = ((v45 - v267) / v120);
        let v270: bool = (v45 < v267);
        let v284: f64 = (if (!v270) { (v45 + (v120 * (((v1 + (((-v269)) as f64).exp())) as f64).ln())) } else { (if v270 { (v267 + (v120 * (((v1 + ((v269) as f64).exp())) as f64).ln())) } else { v4 }) });
        let v290: f64 = ((v174 + (v118 * self.scalar_v285)) + (v177 * self.scalar_v288));
        let v291: f64 = (v45 - v290);
        let v292: f64 = (v291 / v120);
        let v293: bool = (v45 < v290);
        let v294: f64 = ((v292) as f64).exp();
        let v295: f64 = (v1 + v294);
        let v296: f64 = ((v295) as f64).ln();
        let v300: bool = (!v293);
        let v302: f64 = (((-v292)) as f64).exp();
        let v303: f64 = (v1 + v302);
        let v304: f64 = ((v303) as f64).ln();
        let v307: f64 = (if v300 { (v45 + (v120 * v304)) } else { (if v293 { (v290 + (v120 * v296)) } else { v4 }) });
        let v308: f64 = (v1 / v197);
        let v309: f64 = (v1 / v261);
        let v310: f64 = (self.scalar_v65 * v308);
        let v311: f64 = f64::powf(v310, self.scalar_v33);
        let v315: f64 = (v311 * self.scalar_v314);
        let v317: f64 = (self.scalar_v285 / v307);
        let v320: f64 = (self.scalar_v316 * f64::powf(v317, self.scalar_v318));
        let v323: f64 = (self.scalar_v67 / v261);
        let v326: f64 = (self.scalar_v321 + (self.scalar_v322 * f64::powf(v323, self.scalar_v68)));
        let v327: f64 = (v1 / v326);
        let v329: f64 = (v326 * self.scalar_v328);
        let v330: f64 = (self.scalar_v321 * v327);
        let v355: f64 = (((v126 * self.scalar_v353)) as f64).exp();
        let v356: f64 = (self.scalar_v352 * v355);
        let v367: f64 = (((v126 * self.scalar_v365)) as f64).exp();
        let v368: f64 = (self.scalar_v364 * v367);
        let v375: f64 = (if self.scalar_v370 { (self.scalar_v371 * (v1 + (v125 * self.scalar_v369))) } else { v4 });
        let v378: f64 = (if self.scalar_v370 { ((v375 - v1) / v31) } else { v292 });
        let v379: bool = (v375 < v1);
        let v380: bool = (self.scalar_v370 && v379);
        let v381: f64 = ((v378) as f64).exp();
        let v382: f64 = (v1 + v381);
        let v386: f64 = (if v380 { (v1 + (v31 * ((v382) as f64).ln())) } else { v375 });
        let v388: bool = (self.scalar_v370 && (!v379));
        let v390: f64 = (((-v378)) as f64).exp();
        let v391: f64 = (v1 + v390);
        let v396: f64 = 0.0006931471805599453;
        let v400: f64 = (if self.scalar_v399 { self.scalar_v371 } else { (if self.scalar_v370 { ((if v388 { (v386 + (v31 * ((v391) as f64).ln())) } else { v386 }) - v396) } else { v4 }) });
        let v407: f64 = (if self.scalar_v402 { (self.scalar_v403 * (v1 + (v125 * self.scalar_v401))) } else { v4 });
        let v410: f64 = (if self.scalar_v402 { ((v407 - v1) / v31) } else { v378 });
        let v411: bool = (v407 < v1);
        let v412: bool = (self.scalar_v402 && v411);
        let v413: f64 = ((v410) as f64).exp();
        let v414: f64 = (v1 + v413);
        let v418: f64 = (if v412 { (v1 + (v31 * ((v414) as f64).ln())) } else { v407 });
        let v420: bool = (self.scalar_v402 && (!v411));
        let v422: f64 = (((-v410)) as f64).exp();
        let v423: f64 = (v1 + v422);
        let v431: f64 = (if self.scalar_v430 { self.scalar_v403 } else { (if self.scalar_v402 { ((if v420 { (v418 + (v31 * ((v423) as f64).ln())) } else { v418 }) - v396) } else { v4 }) });
        let v436: f64 = (self.scalar_v432 * (v1 + (v125 * self.scalar_v433)));
        let v437: f64 = 1e-6;
        let v438: f64 = (v436 * v436);
        let v439: bool = (v436 < v4);
        let v440: f64 = 0.5;
        let v443: f64 = (((v437 + v438)) as f64).sqrt();
        let v452: f64 = 4.0;
        let v457: f64 = (v126 * self.scalar_v456);
        let v459: f64 = (((v457 / v400)) as f64).exp();
        let v460: f64 = (self.scalar_v451 * v459);
        let v462: f64 = (v124 * self.scalar_v461);
        let v464: f64 = (((v462 / v400)) as f64).exp();
        let v465: f64 = (v460 * v464);
        let v469: f64 = (((v126 * self.scalar_v467)) as f64).exp();
        let v470: f64 = (self.scalar_v466 * v469);
        let v475: f64 = (((v126 * self.scalar_v473)) as f64).exp();
        let v476: f64 = (self.scalar_v471 * v475);
        let v478: f64 = 6.0;
        let v554: f64 = (((v126 * self.scalar_v552)) as f64).exp();
        let v555: f64 = (self.scalar_v550 * v554);
        let v559: f64 = (((v124 * self.scalar_v557)) as f64).exp();
        let v560: f64 = (v555 * v559);
        let v588: f64 = -0.5;
        let v590: f64 = (v1 / v311);
        let v599: f64 = (self.scalar_v64 * (self.scalar_v64 * (v308 * (self.scalar_v65 * (v590 * (f64::powf((self.scalar_v64 * v148), v588) * (v148 * (v148 * self.scalar_v591))))))));
        let v611: f64 = f64::powf((self.scalar_v97 * v170), v588);
        let v621: f64 = (self.scalar_v97 * (self.scalar_v97 * (v309 * (self.scalar_v67 * ((v1 / f64::powf((self.scalar_v67 * v309), self.scalar_v68)) * (v611 * (v170 * (v170 * self.scalar_v613))))))));
        let v633: f64 = (((v126 * self.scalar_v340)) as f64).exp();
        let v635: f64 = (v633 * self.scalar_v634);
        let v636: f64 = (v327 * v635);
        let v638: f64 = (v633 * self.scalar_v637);
        let v639: f64 = (v590 * v638);
        let v644: f64 = (((v126 * self.scalar_v642)) as f64).exp();
        let v645: f64 = (self.scalar_v640 * v644);
        let v648: f64 = (((v124 * self.scalar_v646)) as f64).exp();
        let v649: f64 = (v645 * v648);
        let v661: f64 = (((v126 * self.scalar_v659)) as f64).exp();
        let v662: f64 = (self.scalar_v658 * v661);
        let v671: f64 = (((v126 * self.scalar_v669)) as f64).exp();
        let v672: f64 = (self.scalar_v668 * v671);
        let v676: f64 = (((v124 * self.scalar_v674)) as f64).exp();
        let v677: f64 = (v672 * v676);
        let v682: f64 = (((v126 * self.scalar_v680)) as f64).exp();
        let v683: f64 = (self.scalar_v678 * v682);
        let v687: f64 = (((v126 * self.scalar_v685)) as f64).exp();
        let v688: f64 = (self.scalar_v684 * v687);
        let v690: f64 = (v683 + v688);
        let v693: f64 = ((self.scalar_v689 * v690) / self.scalar_v692);
        let v698: f64 = (((v126 * self.scalar_v696)) as f64).exp();
        let v699: f64 = (self.scalar_v694 * v698);
        let v718: f64 = (v633 * self.scalar_v717);
        let v748: f64 = ctx.node_voltage(nodes[7]);
        let v749: f64 = ctx.node_voltage(nodes[8]);
        let v751: f64 = (self.scalar_v0 * (v748 - v749));
        let v752: f64 = ctx.node_voltage(nodes[9]);
        let v754: f64 = (self.scalar_v0 * (v748 - v752));
        let v755: f64 = ctx.node_voltage(nodes[5]);
        let v757: f64 = (self.scalar_v0 * (v748 - v755));
        let v758: f64 = ctx.node_voltage(nodes[6]);
        let v760: f64 = (self.scalar_v0 * (v758 - v755));
        let v762: f64 = (self.scalar_v0 * (v758 - v748));
        let v765: f64 = (self.scalar_v0 * (ctx.node_voltage(nodes[3]) - v749));
        let v767: f64 = (self.scalar_v0 * (v749 - v752));
        let v771: f64 = ctx.node_voltage(nodes[1]);
        let v778: f64 = (self.scalar_v0 * (v771 - ctx.node_voltage(nodes[0])));
        let v779: f64 = ctx.node_voltage(nodes[11]);
        let v781: f64 = (self.scalar_v0 * (v779 - v749));
        let v784: f64 = (self.scalar_v0 * (ctx.node_voltage(nodes[10]) - v779));
        let v787: f64 = (((v754 + v762) - v767) - v781);
        let v792: f64 = (v778 + ((v787 + ((self.scalar_v0 * (v771 - v758)) + (-v778))) - v784));
        let v793: f64 = (v765 - v781);
        let v794: f64 = (v793 - v784);
        let v795: f64 = (v122 * v754);
        let v797: bool = (v795 < self.scalar_v796);
        let v798: f64 = ((v795) as f64).exp();
        let v800: bool = (!v797);
        let v802: f64 = (if v800 { self.scalar_v801 } else { v4 });
        let v807: f64 = (v122 * v757);
        let v808: f64 = (v807 / v400);
        let v809: bool = (v808 < self.scalar_v796);
        let v810: f64 = ((v808) as f64).exp();
        let v812: bool = (!v809);
        let v813: f64 = (if v812 { self.scalar_v801 } else { v802 });
        let v817: f64 = (if v812 { (v813 * (v1 + (v808 - self.scalar_v796))) } else { (if v809 { v810 } else { v4 }) });
        let v818: f64 = (v122 * v787);
        let v819: bool = (v818 < self.scalar_v796);
        let v820: f64 = ((v818) as f64).exp();
        let v822: bool = (!v819);
        let v823: f64 = (if v822 { self.scalar_v801 } else { v813 });
        let v827: f64 = (if v822 { (v823 * (v1 + (v818 - self.scalar_v796))) } else { (if v819 { v820 } else { v4 }) });
        let v838: f64 = (v122 * v792);
        let v839: bool = (v838 < self.scalar_v796);
        let v840: f64 = ((v838) as f64).exp();
        let v842: bool = (!v839);
        let v843: f64 = (if v842 { self.scalar_v801 } else { (if (!((v122 * v762) < self.scalar_v796)) { self.scalar_v801 } else { v823 }) });
        let v847: f64 = (if v842 { (v843 * (v1 + (v838 - self.scalar_v796))) } else { (if v839 { v840 } else { v4 }) });
        let v858: f64 = (v122 * v794);
        let v859: bool = (v858 < self.scalar_v796);
        let v860: f64 = ((v858) as f64).exp();
        let v862: bool = (!v859);
        let v863: f64 = (if v862 { self.scalar_v801 } else { (if (!((v122 * v765) < self.scalar_v796)) { self.scalar_v801 } else { v843 }) });
        let v867: f64 = (if v862 { (v863 * (v1 + (v858 - self.scalar_v796))) } else { (if v859 { v860 } else { v4 }) });
        let v878: f64 = (v792 - v220);
        let v879: f64 = (v122 * v878);
        let v880: bool = (v879 < self.scalar_v796);
        let v881: f64 = ((v879) as f64).exp();
        let v883: bool = (!v880);
        let v884: f64 = (if v883 { self.scalar_v801 } else { (if (!((v122 * v793) < self.scalar_v796)) { self.scalar_v801 } else { v863 }) });
        let v889: f64 = (v787 - v220);
        let v890: f64 = (v122 * v889);
        let v891: bool = (v890 < self.scalar_v796);
        let v892: f64 = ((v890) as f64).exp();
        let v894: bool = (!v891);
        let v895: f64 = (if v894 { self.scalar_v801 } else { v884 });
        let v900: f64 = (v754 - v220);
        let v901: f64 = (v122 * v900);
        let v902: bool = (v901 < self.scalar_v796);
        let v903: f64 = ((v901) as f64).exp();
        let v905: bool = (!v902);
        let v906: f64 = (if v905 { self.scalar_v801 } else { v895 });
        let v910: f64 = (if v905 { (v906 * (v1 + (v901 - self.scalar_v796))) } else { (if v902 { v903 } else { v4 }) });
        let v911: f64 = (v751 - v220);
        let v912: f64 = (v122 * v911);
        let v913: bool = (v912 < self.scalar_v796);
        let v914: f64 = ((v912) as f64).exp();
        let v916: bool = (!v913);
        let v917: f64 = (if v916 { self.scalar_v801 } else { v906 });
        let v921: f64 = (if v916 { (v917 * (v1 + (v912 - self.scalar_v796))) } else { (if v913 { v914 } else { v4 }) });
        let v924: f64 = (((v1 + (v452 * v910))) as f64).sqrt();
        let v927: f64 = (((v1 + (v452 * v921))) as f64).sqrt();
        let v928: f64 = (v32 * v921);
        let v929: f64 = (v1 + v927);
        let v930: f64 = (v928 / v929);
        let v932: bool = (v930 < self.scalar_v931);
        let v933: f64 = (if v932 { self.scalar_v931 } else { v930 });
        let v935: f64 = (v1 + v924);
        let v936: f64 = (v935 / v929);
        let v938: f64 = ((v924 - v927) - ((v936) as f64).ln());
        let v939: f64 = (v120 * v938);
        let v940: f64 = (v767 + v939);
        let v941: f64 = (v940 / v368);
        let v942: bool = (v941 > v4);
        let v943: f64 = 100.0;
        let v944: bool = (v751 < v943);
        let v945: bool = (v942 && v944);
        let v948: bool = (v942 && (!v944));
        let v950: f64 = (v1 + (v751 - v943));
        let v954: f64 = (v32 * v120);
        let v955: f64 = (v440 * v941);
        let v956: f64 = (v368 * v955);
        let v958: f64 = (v1 + (v122 * v956));
        let v959: f64 = ((v958) as f64).ln();
        let v963: f64 = (if v942 { ((v220 + (v954 * v959)) - (if v948 { (v943 + ((v950) as f64).ln()) } else { (if v945 { v751 } else { v4 }) })) } else { v4 });
        let v964: f64 = 0.2;
        let v966: f64 = (if v942 { (v220 * v964) } else { v4 });
        let v968: f64 = (if v942 { (v966 * v966) } else { v437 });
        let v971: bool = (v963 < v4);
        let v972: bool = (v942 && v971);
        let v973: f64 = (v440 * v968);
        let v975: f64 = (((v968 + (if v942 { (v963 * v963) } else { v438 }))) as f64).sqrt();
        let v976: f64 = (v975 - v963);
        let v980: bool = (v942 && (!v971));
        let v983: f64 = (if v980 { (v440 * (v963 + v975)) } else { (if v972 { (v973 / v976) } else { v4 }) });
        let v987: f64 = (v983 + self.scalar_v986);
        let v988: f64 = (v983 * v987);
        let v991: f64 = (self.scalar_v985 * (v983 + (v368 * self.scalar_v984)));
        let v993: f64 = (if v942 { (v988 / v991) } else { v4 });
        let v995: f64 = (if v942 { (v941 / v993) } else { v4 });
        let v999: f64 = (if v942 { ((v995 - v1) / self.scalar_v997) } else { v410 });
        let v1000: bool = (v995 < v1);
        let v1001: bool = (v942 && v1000);
        let v1002: f64 = ((v999) as f64).exp();
        let v1003: f64 = (v1 + v1002);
        let v1009: bool = (v942 && (!v1000));
        let v1011: f64 = (((-v999)) as f64).exp();
        let v1012: f64 = (v1 + v1011);
        let v1025: f64 = (if v942 { ((if v1009 { (v995 + (self.scalar_v997 * ((v1012) as f64).ln())) } else { (if v1001 { (v1 + (self.scalar_v997 * ((v1003) as f64).ln())) } else { v4 }) }) / self.scalar_v1023) } else { v4 });
        let v1027: f64 = (if v942 { (v983 / self.scalar_v986) } else { v4 });
        let v1028: f64 = (v452 * v1025);
        let v1029: f64 = (v1027 * v1028);
        let v1030: f64 = (v1 + v1027);
        let v1033: f64 = (((v1 + (v1029 * v1030))) as f64).sqrt();
        let v1034: f64 = (v1 + v1033);
        let v1035: f64 = (v32 * v1025);
        let v1036: f64 = (v1030 * v1035);
        let v1038: f64 = (if v942 { (v1034 / v1036) } else { v4 });
        let v1040: f64 = (v933 * v1038);
        let v1041: f64 = ((v1 - v1038) + v1040);
        let v1042: f64 = (v1 + v1040);
        let v1044: f64 = (if v942 { (v1041 / v1042) } else { v4 });
        let v1045: f64 = (v956 * v1044);
        let v1047: f64 = (if v942 { (v122 * v1045) } else { v4 });
        let v1050: f64 = (v1 + (v933 + v1047));
        let v1053: f64 = (if v942 { ((v32 * v1047) + (v933 * v1050)) } else { v4 });
        let v1056: f64 = (if v942 { (v440 * (v1047 - v1)) } else { v4 });
        let v1060: bool = (v1047 >= v1);
        let v1061: bool = (v942 && v1060);
        let v1062: f64 = (((if v942 { (v1053 + (v1056 * v1056)) } else { v4 })) as f64).sqrt();
        let v1066: bool = (v942 && (!v1060));
        let v1067: f64 = (v1062 - v1056);
        let v1069: f64 = (if v1066 { (v1053 / v1067) } else { (if v1061 { (v1056 + v1062) } else { v4 }) });
        let v1072: bool = (v942 && (v1069 < self.scalar_v1070));
        let v1073: f64 = (if v1072 { self.scalar_v1070 } else { v1069 });
        let v1074: f64 = (v1 + v1073);
        let v1075: f64 = (v1073 * v1074);
        let v1077: f64 = (((v122 * v220)) as f64).exp();
        let v1083: f64 = (if v942 { (self.scalar_v1080 * (v941 - self.scalar_v984)) } else { v4 });
        let v1085: f64 = (self.scalar_v984 * (v368 * self.scalar_v985));
        let v1090: f64 = ((((if v942 { (v941 * v1085) } else { v4 }) + (v1083 * v1083))) as f64).sqrt();
        let v1095: bool = (v942 && self.scalar_v1094);
        let v1096: f64 = (v47 * v261);
        let v1099: bool = (v942 && self.scalar_v1098);
        let v1100: f64 = (v32 * v941);
        let v1101: f64 = (v941 + v993);
        let v1103: f64 = (v47 + (v1100 / v1101));
        let v1106: f64 = (v941 * self.scalar_v984);
        let v1107: f64 = (v941 + self.scalar_v984);
        let v1112: bool = (!v942);
        let v1113: f64 = (v32 * v910);
        let v1116: f64 = (if v1112 { (if v800 { (v802 * (v1 + (v795 - self.scalar_v796))) } else { (if v797 { v798 } else { v4 }) }) } else { (if v942 { (v1075 * v1077) } else { v4 }) });
        let v1127: bool = ((((v767) as f64).abs() < (v120 * 1e-5)) || (((v939) as f64).abs() < ((v120 * 1e-40) * (v924 + v927))));
        let v1128: bool = (v1112 && v1127);
        let v1129: f64 = (v933 + (if v1112 { (v1113 / v935) } else { v1073 }));
        let v1131: f64 = (if v1128 { (v440 * v1129) } else { v4 });
        let v1132: f64 = (v1 + v1131);
        let v1136: bool = (v1112 && (!v1127));
        let v1138: f64 = ((v754 + v939) - v751);
        let v1140: f64 = (if v1136 { (v939 / v1138) } else { (if v1128 { (v1131 / v1132) } else { v1044 }) });
        let v1142: f64 = (if v1112 { v1096 } else { (if v1099 { (v261 * v1103) } else { (if v1095 { v1096 } else { v4 }) }) });
        let v1146: f64 = (if v1112 { (v1 - ((if v1112 { v941 } else { (if v942 { (v1106 / v1107) } else { v4 }) }) / self.scalar_v984)) } else { (if v942 { (self.scalar_v984 / v1107) } else { v4 }) });
        let v1150: f64 = (v197 * self.scalar_v1149);
        let v1151: f64 = (v47 * v197);
        let v1152: f64 = (v757 - v1150);
        let v1153: f64 = (v1152 / v1151);
        let v1154: bool = (v757 < v1150);
        let v1155: f64 = ((v1153) as f64).exp();
        let v1156: f64 = (v1 + v1155);
        let v1157: f64 = ((v1156) as f64).ln();
        let v1161: bool = (!v1154);
        let v1163: f64 = (((-v1153)) as f64).exp();
        let v1164: f64 = (v1 + v1163);
        let v1165: f64 = ((v1164) as f64).ln();
        let v1168: f64 = (if v1161 { (v1150 - (v1151 * v1165)) } else { (if v1154 { (v757 - (v1151 * v1157)) } else { v4 }) });
        let v1170: f64 = (v1 - (v308 * v1168));
        let v1172: f64 = f64::powf(v1170, self.scalar_v1171);
        let v1173: f64 = (v197 / self.scalar_v1171);
        let v1174: f64 = (v1 - v1172);
        let v1178: f64 = ((v1173 * v1174) + (v171 * (v757 - v1168)));
        let v1189: f64 = (if self.scalar_v1188 { v754 } else { (if self.scalar_v1184 { (v751 + (if v1112 { v767 } else { (if v942 { (v1083 + v1090) } else { v4 }) })) } else { (if self.scalar_v1180 { v751 } else { v4 }) }) });
        let v1190: f64 = (v32 - v330);
        let v1191: f64 = (v1 - v330);
        let v1192: f64 = (v1190 / v1191);
        let v1195: f64 = (v1 - f64::powf(v1192, self.scalar_v1193));
        let v1196: f64 = (v261 * v1195);
        let v1197: f64 = (v1189 - v1196);
        let v1198: f64 = (v1197 / v1142);
        let v1199: bool = (v1189 < v1196);
        let v1200: f64 = ((v1198) as f64).exp();
        let v1201: f64 = (v1 + v1200);
        let v1202: f64 = ((v1201) as f64).ln();
        let v1206: bool = (!v1199);
        let v1208: f64 = (((-v1198)) as f64).exp();
        let v1209: f64 = (v1 + v1208);
        let v1210: f64 = ((v1209) as f64).ln();
        let v1213: f64 = (if v1206 { (v1196 - (v1142 * v1210)) } else { (if v1199 { (v1189 - (v1142 * v1202)) } else { v4 }) });
        let v1215: f64 = f64::powf(v1146, self.scalar_v1214);
        let v1217: f64 = (v261 / self.scalar_v1216);
        let v1219: f64 = (v1 - (v1213 / v261));
        let v1220: f64 = f64::powf(v1219, self.scalar_v1216);
        let v1222: f64 = (v1 - (v1215 * v1220));
        let v1224: f64 = (v1192 * v1215);
        let v1225: f64 = (v1189 - v1213);
        let v1227: f64 = ((v1217 * v1222) + (v1224 * v1225));
        let v1230: f64 = ((v1191 * v1227) + (v330 * v751));
        let v1231: f64 = (v452 * v465);
        let v1232: f64 = (v1231 / v470);
        let v1233: f64 = (v817 * v1232);
        let v1235: f64 = (((v1 + v1233)) as f64).sqrt();
        let v1236: f64 = (v1 + v1235);
        let v1237: f64 = (v1233 / v1236);
        let v1238: f64 = (v1 / v431);
        let v1239: f64 = f64::powf(v1116, v1238);
        let v1240: f64 = (v1232 * v1239);
        let v1242: f64 = (((v1 + v1240)) as f64).sqrt();
        let v1243: f64 = (v1 + v1242);
        let v1244: f64 = (v1240 / v1243);
        let v1247: f64 = (v1 + (v1178 / v639));
        let v1249: f64 = (v1247 + (v1230 / v636));
        let v1252: f64 = (v718 * v1247);
        let v1255: f64 = (-v1230);
        let v1256: f64 = (v1255 / v636);
        let v1257: f64 = (v718 * v1256);
        let v1260: f64 = (((if self.scalar_v1251 { (v122 * v1252) } else { v4 })) as f64).exp();
        let v1261: f64 = (((if self.scalar_v1251 { (v122 * v1257) } else { v4 })) as f64).exp();
        let v1262: f64 = (v1260 - v1261);
        let v1264: f64 = (((v122 * v718)) as f64).exp();
        let v1265: f64 = (v1264 - v1);
        let v1267: f64 = (if self.scalar_v1251 { (v1262 / v1265) } else { (if self.scalar_v1245 { v1249 } else { v4 }) });
        let v1268: f64 = 0.010000000000000002;
        let v1269: f64 = (v1267 * v1267);
        let v1270: bool = (v1267 < v4);
        let v1271: f64 = 0.005000000000000001;
        let v1273: f64 = (((v1268 + v1269)) as f64).sqrt();
        let v1274: f64 = (v1273 - v1267);
        let v1277: bool = (!v1270);
        let v1280: f64 = (if v1277 { (v440 * (v1267 + v1273)) } else { (if v1270 { (v1271 / v1274) } else { v4 }) });
        let v1283: f64 = (v1 + (v440 * (v1237 + v1244)));
        let v1284: f64 = (v1280 * v1283);
        let v1286: f64 = (v465 * self.scalar_v1285);
        let v1287: f64 = (v1239 * v1286);
        let v1288: f64 = (v465 * v817);
        let v1290: f64 = ((v1288 - v1287) / v1284);
        let v1291: f64 = 0.0001;
        let v1292: f64 = (v757 / v1291);
        let v1293: bool = (v757 < v4);
        let v1294: f64 = ((v1292) as f64).exp();
        let v1295: f64 = (v1 + v1294);
        let v1299: bool = (!v1293);
        let v1301: f64 = (((-v1292)) as f64).exp();
        let v1302: f64 = (v1 + v1301);
        let v1306: f64 = (if v1299 { (v757 + (v1291 * ((v1302) as f64).ln())) } else { (if v1293 { (v1291 * ((v1295) as f64).ln()) } else { v4 }) });
        let v1343: f64 = (v807 / self.scalar_v507);
        let v1344: bool = (v1343 < self.scalar_v796);
        let v1345: f64 = ((v1343) as f64).exp();
        let v1347: bool = (!v1344);
        let v1348: f64 = (if v1347 { self.scalar_v801 } else { (if (!((v1306 / self.scalar_v1307) < self.scalar_v796)) { self.scalar_v801 } else { v917 }) });
        let v1377: f64 = (if (self.scalar_v526 && (!(((v1290 / v465) - 1000.0) < 40.0))) { 2.3538526683702e17 } else { (if (self.scalar_v526 && (!((v122 * (v757 - v284)) < self.scalar_v796))) { self.scalar_v801 } else { v1348 }) });
        let v1417: f64 = (v122 * v760);
        let v1418: f64 = (v1417 / self.scalar_v518);
        let v1419: bool = (v1418 < self.scalar_v796);
        let v1420: f64 = ((v1418) as f64).exp();
        let v1422: bool = (!v1419);
        let v1423: f64 = (if v1422 { self.scalar_v801 } else { v1377 });
        let v1427: f64 = (if v1422 { (v1423 * (v1 + (v1418 - self.scalar_v796))) } else { (if v1419 { v1420 } else { (if v1347 { (v1348 * (v1 + (v1343 - self.scalar_v796))) } else { (if v1344 { v1345 } else { v1306 }) }) }) });
        let v1453: f64 = (v807 / self.scalar_v479);
        let v1454: bool = (v1453 < self.scalar_v796);
        let v1455: f64 = ((v1453) as f64).exp();
        let v1457: bool = (!v1454);
        let v1458: f64 = (if v1457 { self.scalar_v801 } else { (if (self.scalar_v526 && (!((v122 * (v760 - v284)) < self.scalar_v796))) { self.scalar_v801 } else { v1423 }) });
        let v1465: f64 = (v1417 / self.scalar_v562);
        let v1466: bool = (v1465 < self.scalar_v796);
        let v1467: f64 = ((v1465) as f64).exp();
        let v1469: bool = (!v1466);
        let v1470: f64 = (if v1469 { self.scalar_v801 } else { v1458 });
        let v1474: f64 = (if v1469 { (v1470 * (v1 + (v1465 - self.scalar_v796))) } else { (if v1466 { v1467 } else { (if v1457 { (v1458 * (v1 + (v1453 - self.scalar_v796))) } else { (if v1454 { v1455 } else { v1427 }) }) }) });
        let v1477: f64 = (v818 / self.scalar_v492);
        let v1478: bool = (v1477 < self.scalar_v796);
        let v1479: f64 = ((v1477) as f64).exp();
        let v1481: bool = (!v1478);
        let v1482: f64 = (if v1481 { self.scalar_v801 } else { v1470 });
        let v1489: f64 = (v1417 / self.scalar_v572);
        let v1490: bool = (v1489 < self.scalar_v796);
        let v1491: f64 = ((v1489) as f64).exp();
        let v1493: bool = (!v1490);
        let v1494: f64 = (if v1493 { self.scalar_v801 } else { v1482 });
        let v1498: f64 = (if v1493 { (v1494 * (v1 + (v1489 - self.scalar_v796))) } else { (if v1490 { v1491 } else { (if v1481 { (v1482 * (v1 + (v1477 - self.scalar_v796))) } else { (if v1478 { v1479 } else { v1474 }) }) }) });
        let v1504: bool = (v1293 && self.scalar_v1503);
        let v1521: f64 = (if v1504 { (v308 * v757) } else { v633 });
        let v1523: f64 = 1e-30;
        let v1541: f64 = (f64::powf(((((v1521 * v1521) + v1523)) as f64).sqrt(), self.scalar_v1527) * ((self.scalar_v33 * (self.scalar_v1530 - ((v171 * v1521) * self.scalar_v1532))) - ((v1521 * (v478 * v1521)) * (v1521 + self.scalar_v1532))));
        let v1542: f64 = 0.16666666666666666;
        let v1549: f64 = (if v1504 { ((v599 * (self.scalar_v35 * v757)) / (v148 * (if v1504 { (v1541 * v1542) } else { v4 }))) } else { v1521 });
        let v1550: f64 = -0.001;
        let v1559: f64 = (if ((v1504 && (v1549 < v1550)) && (!(v1549 < self.scalar_v796))) { self.scalar_v801 } else { (if (v1504 && (!((v599 * (v1 - (self.scalar_v35 / (v32 * v1172)))) < self.scalar_v796))) { self.scalar_v801 } else { v1494 }) });
        let v1596: bool = (self.scalar_v1594 && (v751 < v4));
        let v1597: f64 = (v309 * v751);
        let v1616: f64 = (if v1596 { v1597 } else { v611 });
        let v1634: f64 = (f64::powf((((v1523 + (v1616 * v1616))) as f64).sqrt(), self.scalar_v1620) * ((self.scalar_v68 * (self.scalar_v1623 - ((v171 * v1616) * self.scalar_v1625))) - ((v1616 * (v478 * v1616)) * (v1616 + self.scalar_v1625))));
        let v1641: f64 = (if v1596 { ((v621 * (self.scalar_v70 * v751)) / (v170 * (if v1596 { (v1542 * v1634) } else { v4 }))) } else { v1616 });
        let v1650: f64 = (if ((v1596 && (v1641 < v1550)) && (!(v1641 < self.scalar_v796))) { self.scalar_v801 } else { (if (v1596 && (!((v621 * (v1 - (self.scalar_v70 / (v32 * (if v1596 { f64::powf((v1 - v1597), self.scalar_v1216) } else { v4 }))))) < self.scalar_v796))) { self.scalar_v801 } else { v1559 }) });
        let v1681: f64 = (v827 * v1232);
        let v1682: f64 = (v452 * (if v894 { (v895 * (v1 + (v890 - self.scalar_v796))) } else { (if v891 { v892 } else { v4 }) }));
        let v1683: f64 = (v1681 - v1232);
        let v1685: f64 = (((v1 + v1681)) as f64).sqrt();
        let v1686: f64 = (v1 + v1685);
        let v1687: f64 = (v1683 / v1686);
        let v1689: f64 = (((v1 + v1682)) as f64).sqrt();
        let v1690: f64 = (v1 + v1689);
        let v1691: f64 = (v1682 / v1690);
        let v1692: f64 = (v32 * v560);
        let v1695: f64 = (v452 * v560);
        let v1696: f64 = (v1695 / v476);
        let v1771: f64 = (v560 * self.scalar_v1770);
        let v1772: f64 = (v847 - v1);
        let v1773: f64 = (v1771 * v1772);
        let v1776: f64 = (((v1 + (v847 * v1696))) as f64).sqrt();
        let v1777: f64 = (v1 + v1776);
        let v1783: f64 = (v649 * self.scalar_v1782);
        let v1784: f64 = (v847 - v867);
        let v1785: f64 = (v1783 * v1784);
        let v1786: f64 = (v452 * v649);
        let v1787: f64 = (v1786 / v662);
        let v1789: f64 = (v847 + (v867 * self.scalar_v1711));
        let v1792: f64 = (((v1 + (v1787 * v1789))) as f64).sqrt();
        let v1793: f64 = (v1 + v1792);
        let v1797: f64 = (v1772 * v1783);
        let v1800: f64 = (((v1 + (v847 * v1787))) as f64).sqrt();
        let v1801: f64 = (v1 + v1800);
        let v1807: f64 = (self.scalar_v13 * (v560 + v649));
        let v1809: f64 = (if self.scalar_v1805 { (v356 * v1807) } else { v4 });
        let v1810: f64 = (v122 * v1809);
        let v1812: f64 = (v32 - ((v1810) as f64).ln());
        let v1816: f64 = (if self.scalar_v1805 { (v792 - (if self.scalar_v1805 { (v120 * v1812) } else { v4 })) } else { v4 });
        let v1821: bool = (v1816 < v4);
        let v1822: bool = (self.scalar_v1805 && v1821);
        let v1825: f64 = (((self.scalar_v1818 + (if self.scalar_v1805 { (v1816 * v1816) } else { v1269 }))) as f64).sqrt();
        let v1826: f64 = (v1825 - v1816);
        let v1830: bool = (self.scalar_v1805 && (!v1821));
        let v1833: f64 = (if v1830 { (v440 * (v1816 + v1825)) } else { (if v1822 { (self.scalar_v1823 / v1826) } else { v4 }) });
        let v1834: f64 = ((if self.scalar_v1765 { (v1773 / v1777) } else { v4 }) + (if self.scalar_v1796 { (v1797 / v1801) } else { (if self.scalar_v1780 { (v1785 / v1793) } else { v4 }) }));
        let v1837: f64 = (v1833 + (v1809 + (v356 * v1834)));
        let v1842: f64 = (if self.scalar_v1841 { v1 } else { (if self.scalar_v1805 { (v1833 / v1837) } else { v1 }) });
        let v1903: bool = (v1249 < v4);
        let v1905: f64 = (((v1268 + (v1249 * v1249))) as f64).sqrt();
        let v1906: f64 = (v1905 - v1249);
        let v1909: bool = (!v1903);
        let v1912: f64 = (if v1909 { (v440 * (v1249 + v1905)) } else { (if v1903 { (v1271 / v1906) } else { v4 }) });
        let v1922: bool = (v1290 > v4);
        let v1926: bool = (v751 < self.scalar_v1925);
        let v1929: f64 = ((-v1290) / self.scalar_v1928);
        let v1930: bool = (v1929 < self.scalar_v796);
        let v1932: bool = (v1926 && (v1922 && self.scalar_v1924));
        let v1937: bool = (v1932 && (!v1930));
        let v1938: f64 = (if v1937 { self.scalar_v801 } else { v1650 });
        let v1943: f64 = (self.scalar_v1925 - v751);
        let v1946: f64 = (-(if (!v439) { (v440 * (v436 + v443)) } else { (if v439 { (5e-7 / (v443 - v436)) } else { v4 }) }));
        let v1949: f64 = (v1946 * f64::powf((if v1932 { ((if v1937 { (v1938 * (v1 + (v1929 - self.scalar_v796))) } else { (if (v1930 && v1932) { ((v1929) as f64).exp() } else { v4 }) }) * v1943) } else { v4 }), self.scalar_v1947));
        let v2073: bool = (v1926 && (self.scalar_v2069 && ((v1922 && self.scalar_v1968) && self.scalar_v2070)));
        let v2082: f64 = (if v2073 { (f64::powf(v1943, self.scalar_v1947) * f64::powf((v1 - (v1290 / (v1290 + self.scalar_v2075))), self.scalar_v2079)) } else { v4 });
        let v2085: bool = (self.scalar_v1989 && v2073);
        let v2089: f64 = (if v2085 { ((v1290 - self.scalar_v2086) / self.scalar_v2075) } else { v4 });
        let v2093: f64 = (if v2085 { ((v2089 - v1) / self.scalar_v2091) } else { ((v757 - self.scalar_v1320) / v31) });
        let v2094: bool = (v2089 < v1);
        let v2110: f64 = (if (v2085 && (!v2094)) { (v2089 + (self.scalar_v2091 * (((v1 + (((-v2093)) as f64).exp())) as f64).ln())) } else { (if (v2085 && v2094) { (v1 + (self.scalar_v2091 * (((v1 + ((v2093) as f64).exp())) as f64).ln())) } else { v4 }) });
        let v2122: f64 = (if (v2073 && (!((v1946 * (if v2085 { (v2082 * f64::powf(v2110, self.scalar_v2111)) } else { (if (self.scalar_v1986 && v2073) { v2082 } else { v4 }) })) < self.scalar_v796))) { self.scalar_v801 } else { (if (v1932 && (!(v1949 < self.scalar_v796))) { self.scalar_v801 } else { v1938 }) });
        let v2242: f64 = (v315 * self.scalar_v2241);
        let v2244: f64 = (v760 - v1150);
        let v2245: f64 = (v2244 / v1151);
        let v2246: bool = (v760 < v1150);
        let v2247: f64 = ((v2245) as f64).exp();
        let v2248: f64 = (v1 + v2247);
        let v2249: f64 = ((v2248) as f64).ln();
        let v2253: bool = (!v2246);
        let v2255: f64 = (((-v2245)) as f64).exp();
        let v2256: f64 = (v1 + v2255);
        let v2257: f64 = ((v2256) as f64).ln();
        let v2260: f64 = (if v2253 { (v1150 - (v1151 * v2257)) } else { (if v2246 { (v760 - (v1151 * v2249)) } else { v4 }) });
        let v2261: f64 = (v315 * self.scalar_v2240);
        let v2263: f64 = (v1 - (v308 * v2260));
        let v2265: f64 = (v1 - f64::powf(v2263, self.scalar_v1171));
        let v2269: f64 = ((v1173 * v2265) + (v171 * (v760 - v2260)));
        let v2272: f64 = (v329 * self.scalar_v2271);
        let v2274: f64 = (v470 * v683);
        let v2275: f64 = (v440 * v2274);
        let v2276: f64 = (v1237 * v2275);
        let v2277: f64 = (v1912 * v2276);
        let v2278: f64 = (v1244 * v2275);
        let v2279: f64 = (v1912 * v2278);
        let v2280: f64 = (v787 - v1196);
        let v2281: f64 = (v2280 / v1096);
        let v2282: bool = (v787 < v1196);
        let v2283: f64 = ((v2281) as f64).exp();
        let v2284: f64 = (v1 + v2283);
        let v2285: f64 = ((v2284) as f64).ln();
        let v2289: bool = (!v2282);
        let v2291: f64 = (((-v2281)) as f64).exp();
        let v2292: f64 = (v1 + v2291);
        let v2293: f64 = ((v2292) as f64).ln();
        let v2296: f64 = (if v2289 { (v1196 - (v1096 * v2293)) } else { (if v2282 { (v787 - (v1096 * v2285)) } else { v4 }) });
        let v2298: f64 = (v1 - (v2296 / v261));
        let v2300: f64 = (v1 - f64::powf(v2298, self.scalar_v1216));
        let v2302: f64 = (v787 - v2296);
        let v2304: f64 = ((v1217 * v2300) + (v1192 * v2302));
        let v2307: f64 = ((v1191 * v2304) + (v330 * v787));
        let v2312: f64 = (v792 - v1196);
        let v2313: f64 = (v2312 / v1096);
        let v2314: bool = (v792 < v1196);
        let v2315: f64 = ((v2313) as f64).exp();
        let v2316: f64 = (v1 + v2315);
        let v2317: f64 = ((v2316) as f64).ln();
        let v2321: bool = (!v2314);
        let v2323: f64 = (((-v2313)) as f64).exp();
        let v2324: f64 = (v1 + v2323);
        let v2325: f64 = ((v2324) as f64).ln();
        let v2328: f64 = (if v2321 { (v1196 - (v1096 * v2325)) } else { (if v2314 { (v792 - (v1096 * v2317)) } else { v4 }) });
        let v2330: f64 = (v1 - (v2328 / v261));
        let v2332: f64 = (v1 - f64::powf(v2330, self.scalar_v1216));
        let v2334: f64 = (v792 - v2328);
        let v2336: f64 = ((v1217 * v2332) + (v1192 * v2334));
        let v2339: f64 = ((v1191 * v2336) + (v330 * v792));
        let v2343: f64 = (v47 * v307);
        let v2347: f64 = (v307 * self.scalar_v2346);
        let v2348: f64 = (v765 - v2347);
        let v2349: f64 = (v2348 / v2343);
        let v2350: bool = (v765 < v2347);
        let v2351: f64 = ((v2349) as f64).exp();
        let v2352: f64 = (v1 + v2351);
        let v2353: f64 = ((v2352) as f64).ln();
        let v2357: bool = (!v2350);
        let v2359: f64 = (((-v2349)) as f64).exp();
        let v2360: f64 = (v1 + v2359);
        let v2361: f64 = ((v2360) as f64).ln();
        let v2364: f64 = (if v2357 { (v2347 - (v2343 * v2361)) } else { (if v2350 { (v765 - (v2343 * v2353)) } else { v4 }) });
        let v2366: f64 = (v307 / self.scalar_v2365);
        let v2368: f64 = (v1 - (v2364 / v307));
        let v2370: f64 = (v1 - f64::powf(v2368, self.scalar_v2365));
        let v2374: f64 = ((v2366 * v2370) + (v32 * (v765 - v2364)));
        let v2376: f64 = (v470 * v677);
        let v2377: f64 = (v465 / v470);
        let v2380: f64 = f64::powf(v2377, self.scalar_v2379);
        let v2381: f64 = (v2376 * v2380);
        let v2382: f64 = (v120 * self.scalar_v2378);
        let v2383: f64 = (v757 / v2382);
        let v2384: bool = (v2383 < self.scalar_v796);
        let v2385: f64 = ((v2383) as f64).exp();
        let v2387: bool = (!v2384);
        let v2388: f64 = (if v2387 { self.scalar_v801 } else { v2122 });
        let v2392: f64 = (if v2387 { (v2388 * (v1 + (v2383 - self.scalar_v796))) } else { (if v2384 { v2385 } else { v1498 }) });
        let v2393: f64 = (v2381 * v2392);
        let v2394: f64 = (v452 * v688);
        let v2395: f64 = (v120 * v2394);
        let v2396: f64 = (v2395 / v368);
        let v2397: f64 = (v440 * v2396);
        let v2398: f64 = (v1140 * v2397);
        let v2399: f64 = (v32 + v1129);
        let v2403: f64 = (v440 * v693);
        let v2406: f64 = ((v1687 * v2274) + (v1691 * v2396));
        let v2407: f64 = (v2403 * v2406);
        let v2412: f64 = ((v787 - v241) / self.scalar_v2411);
        let v2413: f64 = (v122 * v2412);
        let v2414: bool = (v2413 < self.scalar_v796);
        let v2416: bool = (v2414 && self.scalar_v2415);
        let v2417: f64 = ((v2413) as f64).exp();
        let v2420: bool = (self.scalar_v2415 && (!v2414));
        let v2421: f64 = (if v2420 { self.scalar_v801 } else { v2388 });
        let v2426: f64 = (v699 * v1692);
        let v2427: f64 = (v827 * v2426);
        let v2430: f64 = (((v1 + (v452 * (if v2420 { (v2421 * (v1 + (v2413 - self.scalar_v796))) } else { (if v2416 { v2417 } else { v4 }) })))) as f64).sqrt();
        let v2431: f64 = (v1 + v2430);
        let v2433: f64 = (if self.scalar_v2415 { (v2427 / v2431) } else { (if self.scalar_v2402 { (v2407 / v690) } else { v4 }) });
        let v2441: f64 = (if self.scalar_v2439 { (v847 * v1232) } else { v4 });
        let v2442: f64 = (v2441 - v1232);
        let v2444: f64 = (((v1 + v2441)) as f64).sqrt();
        let v2445: f64 = (v1 + v2444);
        let v2447: f64 = (if self.scalar_v2439 { (v2442 / v2445) } else { v4 });
        let v2449: f64 = (if self.scalar_v2439 { (v452 * (if v883 { (v884 * (v1 + (v879 - self.scalar_v796))) } else { (if v880 { v881 } else { v4 }) })) } else { v4 });
        let v2451: f64 = (((v1 + v2449)) as f64).sqrt();
        let v2452: f64 = (v1 + v2451);
        let v2454: f64 = (if self.scalar_v2439 { (v2449 / v2452) } else { v4 });
        let v2456: f64 = (v693 * self.scalar_v2455);
        let v2459: f64 = ((v2274 * v2447) + (v2396 * v2454));
        let v2460: f64 = (v2456 * v2459);
        let v2463: f64 = (v792 - v241);
        let v2464: f64 = (v122 * v2463);
        let v2465: bool = (v2464 < self.scalar_v796);
        let v2467: bool = (v2465 && self.scalar_v2466);
        let v2468: f64 = ((v2464) as f64).exp();
        let v2471: bool = (self.scalar_v2466 && (!v2465));
        let v2472: f64 = (if v2471 { self.scalar_v801 } else { v2421 });
        let v2477: f64 = (v699 * v1771);
        let v2478: f64 = (v847 * v2477);
        let v2481: f64 = (((v1 + (v452 * (if v2471 { (v2472 * (v1 + (v2464 - self.scalar_v796))) } else { (if v2467 { v2468 } else { v4 }) })))) as f64).sqrt();
        let v2482: f64 = (v1 + v2481);
        let v2484: f64 = (if self.scalar_v2466 { (v2478 / v2482) } else { (if self.scalar_v2439 { (v2460 / v690) } else { v4 }) });
        let v2492: f64 = (if self.scalar_v2488 { (f64::powf(v1170, self.scalar_v2489) - v171) } else { v4 });
        let v2493: f64 = (if self.scalar_v2488 { v1153 } else { v4 });
        let v2494: bool = (v2493 < v4);
        let v2495: bool = (self.scalar_v2488 && v2494);
        let v2496: f64 = ((v2493) as f64).exp();
        let v2497: f64 = (v1 + v2496);
        let v2501: bool = (self.scalar_v2488 && (!v2494));
        let v2503: f64 = (((-v2493)) as f64).exp();
        let v2504: f64 = (v1 + v2503);
        let v2506: f64 = (if v2501 { (v2503 / v2504) } else { (if v2495 { (v1 / v2497) } else { v4 }) });
        let v2509: f64 = (if self.scalar_v2488 { (v171 + (v2492 * v2506)) } else { v4 });
        let v2512: f64 = (v122 * v1233);
        let v2513: f64 = (v2512 / v400);
        let v2514: f64 = (v440 / v1235);
        let v2516: f64 = (if self.scalar_v2488 { (v2513 * v2514) } else { v4 });
        let v2517: f64 = (v1912 * v2275);
        let v2522: f64 = (v762 * v964);
        let v2524: f64 = ((if self.scalar_v2488 { (v2393 / v2382) } else { v4 }) + ((if self.scalar_v2488 { (v2242 * v2509) } else { v4 }) + (if self.scalar_v2488 { (v2516 * v2517) } else { v4 })));
        let v2533: f64 = (if self.scalar_v2488 { (v2277 + (v2393 * self.scalar_v2527)) } else { v4 });
        let v2542: f64 = (if self.scalar_v2541 { v2277 } else { (if self.scalar_v2488 { (v2533 * self.scalar_v2538) } else { v4 }) });
        let v2543: f64 = (if self.scalar_v2541 { v2279 } else { (if self.scalar_v2488 { (v2279 + (v2533 * self.scalar_v2534)) } else { v4 }) });
        let v2547: f64 = 0.0;
        let v2548: f64 = (self.scalar_v27 * v2547);
        let v2581: f64 = (v1287 + v1288);
        let v2582: f64 = (v2581 / v1284);
        let v2590: bool = (v2582 > v4);
        let v2591: f64 = (v2542 + v2543);
        let v2594: bool = (!v2590);
        let v2595: f64 = (v683 * v1912);
        let v2597: f64 = (if v2594 { (v1284 * v2595) } else { (if v2590 { (v2591 / v2582) } else { v4 }) });
        let v2610: f64 = (if self.scalar_v2609 { v4 } else { (if self.scalar_v2604 { (v2597 * self.scalar_v2605) } else { (if self.scalar_v2599 { (self.scalar_v2534 * v2597) } else { v4 }) }) });
        let v2655: f64 = 0.0;
        let v2656: f64 = (self.scalar_v27 * v2655);
        let v2658: f64 = 0.0;
        let v2659: f64 = (self.scalar_v27 * v2658);
        let v2661: f64 = 0.0;
        let v2662: f64 = (self.scalar_v27 * v2661);
        let v2664: f64 = 0.0;
        let v2665: f64 = (self.scalar_v27 * v2664);
        let v2667: f64 = 0.0;
        let v2668: f64 = (self.scalar_v27 * v2667);
        let v2671: f64 = 0.0;
        let v2672: f64 = (self.scalar_v27 * v2671);
        let v2675: f64 = 0.0;
        let v2676: f64 = (self.scalar_v27 * v2675);
        let v2683: f64 = 0.0;
        let v2684: f64 = (self.scalar_v27 * v2683);
        let v2689: f64 = 0.0;
        let v2690: f64 = (self.scalar_v27 * v2689);
        let v2700: f64 = 0.0;
        let v2701: f64 = (v2610 * v2700);
        let v2705: f64 = (if v103 { (-(-1.0 / v104)) } else { v1 });
        let v2708: f64 = (if v111 { (v2705 / v113) } else { (if v109 { v2705 } else { v4 }) });
        let v2709: f64 = (v2708 / self.scalar_v17);
        let v2710: f64 = (v119 * v2708);
        let v2712: f64 = (v120 * v120);
        let v2713: f64 = ((-v2710) / v2712);
        let v2714: f64 = (v2709 / v118);
        let v2760: f64 = ((v173 * v2714) + (v126 * (v172 * v2710)));
        let v2763: f64 = (-v2709);
        let v2765: f64 = ((v2760 + (self.scalar_v65 * v2709)) + (self.scalar_v178 * v2763));
        let v2770: f64 = (((v120 * (-v2765)) - (v181 * v2710)) / v2712);
        let v2784: f64 = (if v190 { ((v194 * v2710) + (v120 * ((v192 * (-v2770)) / v193))) } else { (if v183 { (v2765 + ((v186 * v2710) + (v120 * ((v184 * v2770) / v185)))) } else { v4 }) });
        let v2787: f64 = (self.scalar_v201 * v2763);
        let v2788: f64 = ((v2760 + (self.scalar_v198 * v2709)) + v2787);
        let v2793: f64 = (((v120 * (-v2788)) - (v204 * v2710)) / v2712);
        let v2807: f64 = (if v213 { ((v217 * v2710) + (v120 * ((v215 * (-v2793)) / v216))) } else { (if v206 { (v2788 + ((v209 * v2710) + (v120 * ((v207 * v2793) / v208)))) } else { v4 }) });
        let v2810: f64 = (v2787 + (v2760 + (self.scalar_v221 * v2709)));
        let v2815: f64 = (((v120 * (-v2810)) - (v225 * v2710)) / v2712);
        let v2829: f64 = (if v234 { ((v238 * v2710) + (v120 * ((v236 * (-v2815)) / v237))) } else { (if v227 { (v2810 + ((v230 * v2710) + (v120 * ((v228 * v2815) / v229)))) } else { v4 }) });
        let v2832: f64 = (v2787 + (v2760 + (self.scalar_v67 * v2709)));
        let v2837: f64 = (((v120 * (-v2832)) - (v245 * v2710)) / v2712);
        let v2851: f64 = (if v254 { ((v258 * v2710) + (v120 * ((v256 * (-v2837)) / v257))) } else { (if v247 { (v2832 + ((v250 * v2710) + (v120 * ((v248 * v2837) / v249)))) } else { v4 }) });
        let v2878: f64 = ((v2760 + (self.scalar_v285 * v2709)) + (self.scalar_v288 * v2763));
        let v2883: f64 = (((v120 * (-v2878)) - (v291 * v2710)) / v2712);
        let v2897: f64 = (if v300 { ((v304 * v2710) + (v120 * ((v302 * (-v2883)) / v303))) } else { (if v293 { (v2878 + ((v296 * v2710) + (v120 * ((v294 * v2883) / v295)))) } else { v4 }) });
        let v2900: f64 = ((-v2784) / (v197 * v197));
        let v2902: f64 = (v261 * v261);
        let v2907: f64 = ((self.scalar_v65 * v2900) * (self.scalar_v33 * f64::powf(v310, self.scalar_v1532)));
        let v2912: f64 = (self.scalar_v314 * v2907);
        let v2915: f64 = (v307 * v307);
        let v2928: f64 = (self.scalar_v322 * (((-(self.scalar_v67 * v2851)) / v2902) * (self.scalar_v68 * f64::powf(v323, self.scalar_v1625))));
        let v2931: f64 = ((-v2928) / (v326 * v326));
        let v2932: f64 = (self.scalar_v328 * v2928);
        let v2933: f64 = (self.scalar_v321 * v2931);
        let v2947: f64 = (self.scalar_v352 * (v355 * (self.scalar_v353 * v2714)));
        let v2954: f64 = (self.scalar_v364 * (v367 * (self.scalar_v365 * v2714)));
        let v2957: f64 = (if self.scalar_v370 { (self.scalar_v371 * (self.scalar_v369 * v2708)) } else { v4 });
        let v2959: f64 = (if self.scalar_v370 { (v2957 / v31) } else { v2883 });
        let v2963: f64 = (if v380 { (v31 * ((v381 * v2959) / v382)) } else { v2957 });
        let v2971: f64 = (if self.scalar_v399 { v4 } else { (if self.scalar_v370 { (if v388 { (v2963 + (v31 * ((v390 * (-v2959)) / v391))) } else { v2963 }) } else { v4 }) });
        let v2974: f64 = (if self.scalar_v402 { (self.scalar_v403 * (self.scalar_v401 * v2708)) } else { v4 });
        let v2976: f64 = (if self.scalar_v402 { (v2974 / v31) } else { v2959 });
        let v2980: f64 = (if v412 { (v31 * ((v413 * v2976) / v414)) } else { v2974 });
        let v2991: f64 = (v436 * (self.scalar_v432 * (self.scalar_v433 * v2708)));
        let v3008: f64 = (v400 * v400);
        let v3020: f64 = ((v464 * (self.scalar_v451 * (v459 * (((v400 * (self.scalar_v456 * v2714)) - (v457 * v2971)) / v3008)))) + (v460 * (v464 * (((v400 * (self.scalar_v461 * v2713)) - (v462 * v2971)) / v3008))));
        let v3023: f64 = (self.scalar_v466 * (v469 * (self.scalar_v467 * v2714)));
        let v3084: f64 = ((v559 * (self.scalar_v550 * (v554 * (self.scalar_v552 * v2714)))) + (v555 * (v559 * (self.scalar_v557 * v2713))));
        let v3191: f64 = (v633 * (self.scalar_v340 * v2714));
        let v3195: f64 = ((v635 * v2931) + (v327 * (self.scalar_v634 * v3191)));
        let v3207: f64 = ((v648 * (self.scalar_v640 * (v644 * (self.scalar_v642 * v2714)))) + (v645 * (v648 * (self.scalar_v646 * v2713))));
        let v3230: f64 = (self.scalar_v678 * (v682 * (self.scalar_v680 * v2714)));
        let v3233: f64 = (self.scalar_v684 * (v687 * (self.scalar_v685 * v2714)));
        let v3234: f64 = (v3230 + v3233);
        let v3236: f64 = ((self.scalar_v689 * v3234) / self.scalar_v692);
        let v3239: f64 = (self.scalar_v694 * (v698 * (self.scalar_v696 * v2714)));
        let v3249: f64 = (self.scalar_v717 * v3191);
        let v3272: f64 = (v754 * v2713);
        let v3273: f64 = (self.scalar_v0 * v122);
        let v3274: f64 = (v122 * self.scalar_v3268);
        let v3287: f64 = (v757 * v2713);
        let v3291: f64 = (((v400 * v3287) - (v807 * v2971)) / v3008);
        let v3292: f64 = (v3274 / v400);
        let v3293: f64 = (v3273 / v400);
        let v3303: f64 = (if v812 { (v813 * v3291) } else { (if v809 { (v810 * v3291) } else { v4 }) });
        let v3304: f64 = (if v812 { (v813 * v3292) } else { (if v809 { (v810 * v3292) } else { v4 }) });
        let v3305: f64 = (if v812 { (v813 * v3293) } else { (if v809 { (v810 * v3293) } else { v4 }) });
        let v3306: f64 = (v787 * v2713);
        let v3307: f64 = (v122 * self.scalar_v3269);
        let v3308: f64 = (v122 * self.scalar_v3270);
        let v3324: f64 = (if v822 { (v823 * v3306) } else { (if v819 { (v820 * v3306) } else { v4 }) });
        let v3325: f64 = (if v822 { (v823 * v3273) } else { (if v819 { (v820 * v3273) } else { v4 }) });
        let v3326: f64 = (if v822 { (v823 * v3307) } else { (if v819 { (v820 * v3307) } else { v4 }) });
        let v3327: f64 = (if v822 { (v823 * v3308) } else { (if v819 { (v820 * v3308) } else { v4 }) });
        let v3328: f64 = (if v822 { (v823 * v3274) } else { (if v819 { (v820 * v3274) } else { v4 }) });
        let v3342: f64 = (v122 * self.scalar_v3271);
        let v3343: f64 = (v792 * v2713);
        let v3359: f64 = (if v842 { (v843 * v3307) } else { (if v839 { (v840 * v3307) } else { v4 }) });
        let v3360: f64 = (if v842 { (v843 * v3342) } else { (if v839 { (v840 * v3342) } else { v4 }) });
        let v3361: f64 = (if v842 { (v843 * v3343) } else { (if v839 { (v840 * v3343) } else { v4 }) });
        let v3362: f64 = (if v842 { (v843 * v3308) } else { (if v839 { (v840 * v3308) } else { v4 }) });
        let v3363: f64 = (if v842 { (v843 * v3274) } else { (if v839 { (v840 * v3274) } else { v4 }) });
        let v3377: f64 = (v794 * v2713);
        let v3390: f64 = (if v862 { (v863 * v3273) } else { (if v859 { (v860 * v3273) } else { v4 }) });
        let v3391: f64 = (if v862 { (v863 * v3377) } else { (if v859 { (v860 * v3377) } else { v4 }) });
        let v3392: f64 = (if v862 { (v863 * v3308) } else { (if v859 { (v860 * v3308) } else { v4 }) });
        let v3393: f64 = (if v862 { (v863 * v3274) } else { (if v859 { (v860 * v3274) } else { v4 }) });
        let v3413: f64 = (v122 * (-v2807));
        let v3414: f64 = ((v878 * v2713) + v3413);
        let v3436: f64 = (v3413 + (v889 * v2713));
        let v3458: f64 = (v3413 + (v900 * v2713));
        let v3468: f64 = (if v905 { (v906 * v3458) } else { (if v902 { (v903 * v3458) } else { v4 }) });
        let v3469: f64 = (if v905 { (v906 * v3273) } else { (if v902 { (v903 * v3273) } else { v4 }) });
        let v3470: f64 = (if v905 { (v906 * v3274) } else { (if v902 { (v903 * v3274) } else { v4 }) });
        let v3472: f64 = (v3413 + (v911 * v2713));
        let v3482: f64 = (if v916 { (v917 * v3472) } else { (if v913 { (v914 * v3472) } else { v4 }) });
        let v3483: f64 = (if v916 { (v917 * v3273) } else { (if v913 { (v914 * v3273) } else { v4 }) });
        let v3484: f64 = (if v916 { (v917 * v3274) } else { (if v913 { (v914 * v3274) } else { v4 }) });
        let v3488: f64 = (v32 * v924);
        let v3489: f64 = ((v452 * v3468) / v3488);
        let v3490: f64 = ((v452 * v3469) / v3488);
        let v3491: f64 = ((v452 * v3470) / v3488);
        let v3495: f64 = (v32 * v927);
        let v3496: f64 = ((v452 * v3482) / v3495);
        let v3497: f64 = ((v452 * v3483) / v3495);
        let v3498: f64 = ((v452 * v3484) / v3495);
        let v3505: f64 = (v929 * v929);
        let v3515: f64 = (if v932 { v4 } else { (((v929 * (v32 * v3482)) - (v928 * v3496)) / v3505) });
        let v3516: f64 = (if v932 { v4 } else { (((v929 * (v32 * v3483)) - (v928 * v3497)) / v3505) });
        let v3517: f64 = (if v932 { v4 } else { (((v929 * (v32 * v3484)) - (v928 * v3498)) / v3505) });
        let v3543: f64 = ((v938 * v2710) + (v120 * ((v3489 - v3496) - ((((v929 * v3489) - (v935 * v3496)) / v3505) / v936))));
        let v3544: f64 = (v120 * ((v3490 - v3497) - ((((v929 * v3490) - (v935 * v3497)) / v3505) / v936)));
        let v3545: f64 = (v120 * ((-v3498) - (((-(v935 * v3498)) / v3505) / v936)));
        let v3546: f64 = (v120 * (v3491 - ((v3491 / v929) / v936)));
        let v3548: f64 = (self.scalar_v3268 + v3546);
        let v3552: f64 = (v368 * v368);
        let v3553: f64 = (((v368 * v3543) - (v940 * v2954)) / v3552);
        let v3554: f64 = (v3544 / v368);
        let v3555: f64 = ((self.scalar_v0 + v3545) / v368);
        let v3556: f64 = (v3548 / v368);
        let v3570: f64 = ((v955 * v2954) + (v368 * (v440 * v3553)));
        let v3571: f64 = (v368 * (v440 * v3554));
        let v3572: f64 = (v368 * (v440 * v3555));
        let v3573: f64 = (v368 * (v440 * v3556));
        let v3593: f64 = (if v942 { (v2807 + ((v959 * (v32 * v2710)) + (v954 * (((v956 * v2713) + (v122 * v3570)) / v958)))) } else { v4 });
        let v3594: f64 = (if v942 { ((v954 * ((v122 * v3571) / v958)) - (if v948 { (self.scalar_v0 / v950) } else { (if v945 { self.scalar_v0 } else { v4 }) })) } else { v4 });
        let v3595: f64 = (if v942 { ((v954 * ((v122 * v3572) / v958)) - (if v948 { (self.scalar_v3268 / v950) } else { (if v945 { self.scalar_v3268 } else { v4 }) })) } else { v4 });
        let v3596: f64 = (if v942 { (v954 * ((v122 * v3573) / v958)) } else { v4 });
        let v3599: f64 = (v966 * (if v942 { (v964 * v2807) } else { v4 }));
        let v3601: f64 = (if v942 { (v3599 + v3599) } else { v4 });
        let v3602: f64 = (v963 * v3593);
        let v3604: f64 = (v963 * v3594);
        let v3606: f64 = (v963 * v3595);
        let v3608: f64 = (v963 * v3596);
        let v3616: f64 = (v32 * v975);
        let v3617: f64 = ((v3601 + (if v942 { (v3602 + v3602) } else { (v2991 + v2991) })) / v3616);
        let v3618: f64 = ((if v942 { (v3604 + v3604) } else { v4 }) / v3616);
        let v3619: f64 = ((if v942 { (v3606 + v3606) } else { v4 }) / v3616);
        let v3620: f64 = ((if v942 { (v3608 + v3608) } else { v4 }) / v3616);
        let v3628: f64 = (v976 * v976);
        let v3651: f64 = (if v980 { (v440 * (v3593 + v3617)) } else { (if v972 { (((v976 * (v440 * v3601)) - (v973 * (v3617 - v3593))) / v3628) } else { v4 }) });
        let v3652: f64 = (if v980 { (v440 * (v3594 + v3618)) } else { (if v972 { ((-(v973 * (v3618 - v3594))) / v3628) } else { v4 }) });
        let v3653: f64 = (if v980 { (v440 * (v3595 + v3619)) } else { (if v972 { ((-(v973 * (v3619 - v3595))) / v3628) } else { v4 }) });
        let v3654: f64 = (if v980 { (v440 * (v3596 + v3620)) } else { (if v972 { ((-(v973 * (v3620 - v3596))) / v3628) } else { v4 }) });
        let v3676: f64 = (v991 * v991);
        let v3690: f64 = (if v942 { (((v991 * ((v987 * v3651) + (v983 * v3651))) - (v988 * (self.scalar_v985 * (v3651 + (self.scalar_v984 * v2954))))) / v3676) } else { v4 });
        let v3691: f64 = (if v942 { (((v991 * ((v987 * v3652) + (v983 * v3652))) - (v988 * (self.scalar_v985 * v3652))) / v3676) } else { v4 });
        let v3692: f64 = (if v942 { (((v991 * ((v987 * v3653) + (v983 * v3653))) - (v988 * (self.scalar_v985 * v3653))) / v3676) } else { v4 });
        let v3693: f64 = (if v942 { (((v991 * ((v987 * v3654) + (v983 * v3654))) - (v988 * (self.scalar_v985 * v3654))) / v3676) } else { v4 });
        let v3697: f64 = (v993 * v993);
        let v3711: f64 = (if v942 { (((v993 * v3553) - (v941 * v3690)) / v3697) } else { v4 });
        let v3712: f64 = (if v942 { (((v993 * v3554) - (v941 * v3691)) / v3697) } else { v4 });
        let v3713: f64 = (if v942 { (((v993 * v3555) - (v941 * v3692)) / v3697) } else { v4 });
        let v3714: f64 = (if v942 { (((v993 * v3556) - (v941 * v3693)) / v3697) } else { v4 });
        let v3719: f64 = (if v942 { (v3711 / self.scalar_v997) } else { v2976 });
        let v3720: f64 = (if v942 { (v3712 / self.scalar_v997) } else { v4 });
        let v3721: f64 = (if v942 { (v3713 / self.scalar_v997) } else { v4 });
        let v3722: f64 = (if v942 { (v3714 / self.scalar_v997) } else { v4 });
        let v3767: f64 = (if v942 { ((if v1009 { (v3711 + (self.scalar_v997 * ((v1011 * (-v3719)) / v1012))) } else { (if v1001 { (self.scalar_v997 * ((v1002 * v3719) / v1003)) } else { v4 }) }) / self.scalar_v1023) } else { v4 });
        let v3768: f64 = (if v942 { ((if v1009 { (v3712 + (self.scalar_v997 * ((v1011 * (-v3720)) / v1012))) } else { (if v1001 { (self.scalar_v997 * ((v1002 * v3720) / v1003)) } else { v4 }) }) / self.scalar_v1023) } else { v4 });
        let v3769: f64 = (if v942 { ((if v1009 { (v3713 + (self.scalar_v997 * ((v1011 * (-v3721)) / v1012))) } else { (if v1001 { (self.scalar_v997 * ((v1002 * v3721) / v1003)) } else { v4 }) }) / self.scalar_v1023) } else { v4 });
        let v3770: f64 = (if v942 { ((if v1009 { (v3714 + (self.scalar_v997 * ((v1011 * (-v3722)) / v1012))) } else { (if v1001 { (self.scalar_v997 * ((v1002 * v3722) / v1003)) } else { v4 }) }) / self.scalar_v1023) } else { v4 });
        let v3775: f64 = (if v942 { (v3651 / self.scalar_v986) } else { v4 });
        let v3776: f64 = (if v942 { (v3652 / self.scalar_v986) } else { v4 });
        let v3777: f64 = (if v942 { (v3653 / self.scalar_v986) } else { v4 });
        let v3778: f64 = (if v942 { (v3654 / self.scalar_v986) } else { v4 });
        let v3807: f64 = (v32 * v1033);
        let v3830: f64 = ((v1036 * (((v1030 * ((v1028 * v3775) + (v1027 * (v452 * v3767)))) + (v1029 * v3775)) / v3807)) - (v1034 * ((v1035 * v3775) + (v1030 * (v32 * v3767)))));
        let v3831: f64 = (v1036 * v1036);
        let v3835: f64 = ((v1036 * (((v1030 * ((v1028 * v3776) + (v1027 * (v452 * v3768)))) + (v1029 * v3776)) / v3807)) - (v1034 * ((v1035 * v3776) + (v1030 * (v32 * v3768)))));
        let v3839: f64 = ((v1036 * (((v1030 * ((v1028 * v3777) + (v1027 * (v452 * v3769)))) + (v1029 * v3777)) / v3807)) - (v1034 * ((v1035 * v3777) + (v1030 * (v32 * v3769)))));
        let v3843: f64 = ((v1036 * (((v1030 * ((v1028 * v3778) + (v1027 * (v452 * v3770)))) + (v1029 * v3778)) / v3807)) - (v1034 * ((v1035 * v3778) + (v1030 * (v32 * v3770)))));
        let v3845: f64 = (if v942 { (v3830 / v3831) } else { v4 });
        let v3846: f64 = (if v942 { (v3835 / v3831) } else { v4 });
        let v3847: f64 = (if v942 { (v3839 / v3831) } else { v4 });
        let v3848: f64 = (if v942 { (v3843 / v3831) } else { v4 });
        let v3855: f64 = ((v1038 * v3515) + (v933 * v3845));
        let v3858: f64 = ((v1038 * v3516) + (v933 * v3846));
        let v3861: f64 = ((v1038 * v3517) + (v933 * v3847));
        let v3862: f64 = (v933 * v3848);
        let v3870: f64 = (v1042 * v1042);
        let v3884: f64 = (if v942 { (((v1042 * ((-v3845) + v3855)) - (v1041 * v3855)) / v3870) } else { v4 });
        let v3885: f64 = (if v942 { (((v1042 * ((-v3846) + v3858)) - (v1041 * v3858)) / v3870) } else { v4 });
        let v3886: f64 = (if v942 { (((v1042 * ((-v3847) + v3861)) - (v1041 * v3861)) / v3870) } else { v4 });
        let v3887: f64 = (if v942 { (((v1042 * ((-v3848) + v3862)) - (v1041 * v3862)) / v3870) } else { v4 });
        let v3906: f64 = (if v942 { ((v1045 * v2713) + (v122 * ((v1044 * v3570) + (v956 * v3884)))) } else { v4 });
        let v3907: f64 = (if v942 { (v122 * ((v1044 * v3571) + (v956 * v3885))) } else { v4 });
        let v3908: f64 = (if v942 { (v122 * ((v1044 * v3572) + (v956 * v3886))) } else { v4 });
        let v3909: f64 = (if v942 { (v122 * ((v1044 * v3573) + (v956 * v3887))) } else { v4 });
        let v3931: f64 = (if v942 { ((v32 * v3906) + ((v1050 * v3515) + (v933 * (v3515 + v3906)))) } else { v4 });
        let v3932: f64 = (if v942 { ((v32 * v3907) + ((v1050 * v3516) + (v933 * (v3516 + v3907)))) } else { v4 });
        let v3933: f64 = (if v942 { ((v32 * v3908) + ((v1050 * v3517) + (v933 * (v3517 + v3908)))) } else { v4 });
        let v3934: f64 = (if v942 { ((v32 * v3909) + (v933 * v3909)) } else { v4 });
        let v3939: f64 = (if v942 { (v440 * v3906) } else { v4 });
        let v3940: f64 = (if v942 { (v440 * v3907) } else { v4 });
        let v3941: f64 = (if v942 { (v440 * v3908) } else { v4 });
        let v3942: f64 = (if v942 { (v440 * v3909) } else { v4 });
        let v3943: f64 = (v1056 * v3939);
        let v3945: f64 = (v1056 * v3940);
        let v3947: f64 = (v1056 * v3941);
        let v3949: f64 = (v1056 * v3942);
        let v3959: f64 = (v32 * v1062);
        let v3960: f64 = ((if v942 { (v3931 + (v3943 + v3943)) } else { v4 }) / v3959);
        let v3961: f64 = ((if v942 { (v3932 + (v3945 + v3945)) } else { v4 }) / v3959);
        let v3962: f64 = ((if v942 { (v3933 + (v3947 + v3947)) } else { v4 }) / v3959);
        let v3963: f64 = ((if v942 { (v3934 + (v3949 + v3949)) } else { v4 }) / v3959);
        let v3979: f64 = (v1067 * v1067);
        let v3997: f64 = (if v1072 { v4 } else { (if v1066 { (((v1067 * v3931) - (v1053 * (v3960 - v3939))) / v3979) } else { (if v1061 { (v3939 + v3960) } else { v4 }) }) });
        let v3998: f64 = (if v1072 { v4 } else { (if v1066 { (((v1067 * v3932) - (v1053 * (v3961 - v3940))) / v3979) } else { (if v1061 { (v3940 + v3961) } else { v4 }) }) });
        let v3999: f64 = (if v1072 { v4 } else { (if v1066 { (((v1067 * v3933) - (v1053 * (v3962 - v3941))) / v3979) } else { (if v1061 { (v3941 + v3962) } else { v4 }) }) });
        let v4000: f64 = (if v1072 { v4 } else { (if v1066 { (((v1067 * v3934) - (v1053 * (v3963 - v3942))) / v3979) } else { (if v1061 { (v3942 + v3963) } else { v4 }) }) });
        let v4031: f64 = (if v942 { (self.scalar_v1080 * v3553) } else { v4 });
        let v4032: f64 = (if v942 { (self.scalar_v1080 * v3554) } else { v4 });
        let v4033: f64 = (if v942 { (self.scalar_v1080 * v3555) } else { v4 });
        let v4034: f64 = (if v942 { (self.scalar_v1080 * v3556) } else { v4 });
        let v4047: f64 = (v1083 * v4031);
        let v4049: f64 = (v1083 * v4032);
        let v4051: f64 = (v1083 * v4033);
        let v4053: f64 = (v1083 * v4034);
        let v4059: f64 = (v32 * v1090);
        let v4068: f64 = (if v942 { (v4031 + (((if v942 { ((v1085 * v3553) + (v941 * (self.scalar_v984 * (self.scalar_v985 * v2954)))) } else { v4 }) + (v4047 + v4047)) / v4059)) } else { v4 });
        let v4072: f64 = (v47 * v2851);
        let v4085: f64 = (v1101 * v1101);
        let v4105: f64 = (if v1099 { ((v1103 * v2851) + (v261 * (((v1101 * (v32 * v3553)) - (v1100 * (v3553 + v3690))) / v4085))) } else { (if v1095 { v4072 } else { v4 }) });
        let v4109: f64 = (self.scalar_v984 * v3553);
        let v4110: f64 = (self.scalar_v984 * v3554);
        let v4111: f64 = (self.scalar_v984 * v3555);
        let v4112: f64 = (self.scalar_v984 * v3556);
        let v4116: f64 = (v1107 * v1107);
        let v4152: f64 = (v935 * v935);
        let v4165: f64 = (if v1112 { (((v935 * (v32 * v3470)) - (v1113 * v3491)) / v4152) } else { v4000 });
        let v4166: f64 = (if v1112 { (if v800 { (v802 * v3272) } else { (if v797 { (v798 * v3272) } else { v4 }) }) } else { (if v942 { ((v1077 * ((v1074 * v3997) + (v1073 * v3997))) + (v1075 * (v1077 * ((v220 * v2713) + (v122 * v2807))))) } else { v4 }) });
        let v4167: f64 = (if v1112 { (if v800 { (v802 * v3273) } else { (if v797 { (v798 * v3273) } else { v4 }) }) } else { (if v942 { (v1077 * ((v1074 * v3998) + (v1073 * v3998))) } else { v4 }) });
        let v4169: f64 = (if v1112 { (if v800 { (v802 * v3274) } else { (if v797 { (v798 * v3274) } else { v4 }) }) } else { (if v942 { (v1077 * ((v1074 * v4000) + (v1073 * v4000))) } else { v4 }) });
        let v4170: f64 = (v3515 + (if v1112 { (((v935 * (v32 * v3468)) - (v1113 * v3489)) / v4152) } else { v3997 }));
        let v4171: f64 = (v3516 + (if v1112 { (((v935 * (v32 * v3469)) - (v1113 * v3490)) / v4152) } else { v3998 }));
        let v4172: f64 = (v3517 + (if v1112 { v4 } else { v3999 }));
        let v4177: f64 = (if v1128 { (v440 * v4170) } else { v4 });
        let v4178: f64 = (if v1128 { (v440 * v4171) } else { v4 });
        let v4179: f64 = (if v1128 { (v440 * v4172) } else { v4 });
        let v4180: f64 = (if v1128 { (v440 * v4165) } else { v4 });
        let v4184: f64 = (v1132 * v1132);
        let v4208: f64 = (v1138 * v1138);
        let v4223: f64 = (if v1136 { (((v1138 * v3544) - (v939 * ((self.scalar_v0 + v3544) - self.scalar_v0))) / v4208) } else { (if v1128 { (((v1132 * v4178) - (v1131 * v4178)) / v4184) } else { v3885 }) });
        let v4224: f64 = (if v1136 { (((v1138 * v3545) - (v939 * (v3545 - self.scalar_v3268))) / v4208) } else { (if v1128 { (((v1132 * v4179) - (v1131 * v4179)) / v4184) } else { v3886 }) });
        let v4230: f64 = (if v1112 { v4072 } else { v4105 });
        let v4231: f64 = (if v1112 { v4 } else { (if v1099 { (v261 * (((v1101 * (v32 * v3554)) - (v1100 * (v3554 + v3691))) / v4085)) } else { v4 }) });
        let v4232: f64 = (if v1112 { v4 } else { (if v1099 { (v261 * (((v1101 * (v32 * v3555)) - (v1100 * (v3555 + v3692))) / v4085)) } else { v4 }) });
        let v4233: f64 = (if v1112 { v4 } else { (if v1099 { (v261 * (((v1101 * (v32 * v3556)) - (v1100 * (v3556 + v3693))) / v4085)) } else { v4 }) });
        let v4246: f64 = (if v1112 { (-((if v1112 { v3553 } else { (if v942 { (((v1107 * v4109) - (v1106 * v3553)) / v4116) } else { v4 }) }) / self.scalar_v984)) } else { (if v942 { ((-v4109) / v4116) } else { v4 }) });
        let v4247: f64 = (if v1112 { (-((if v1112 { v3554 } else { (if v942 { (((v1107 * v4110) - (v1106 * v3554)) / v4116) } else { v4 }) }) / self.scalar_v984)) } else { (if v942 { ((-v4110) / v4116) } else { v4 }) });
        let v4248: f64 = (if v1112 { (-((if v1112 { v3555 } else { (if v942 { (((v1107 * v4111) - (v1106 * v3555)) / v4116) } else { v4 }) }) / self.scalar_v984)) } else { (if v942 { ((-v4111) / v4116) } else { v4 }) });
        let v4249: f64 = (if v1112 { (-((if v1112 { v3556 } else { (if v942 { (((v1107 * v4112) - (v1106 * v3556)) / v4116) } else { v4 }) }) / self.scalar_v984)) } else { (if v942 { ((-v4112) / v4116) } else { v4 }) });
        let v4250: f64 = (self.scalar_v1149 * v2784);
        let v4251: f64 = (v47 * v2784);
        let v4253: f64 = (v1151 * (-v4250));
        let v4256: f64 = (v1151 * v1151);
        let v4257: f64 = ((v4253 - (v1152 * v4251)) / v4256);
        let v4258: f64 = (self.scalar_v3268 / v1151);
        let v4259: f64 = (self.scalar_v0 / v1151);
        let v4278: f64 = (-v4258);
        let v4279: f64 = (-v4259);
        let v4294: f64 = (if v1161 { (v4250 - ((v1165 * v4251) + (v1151 * ((v1163 * (-v4257)) / v1164)))) } else { (if v1154 { (-((v1157 * v4251) + (v1151 * ((v1155 * v4257) / v1156)))) } else { v4 }) });
        let v4295: f64 = (if v1161 { (-(v1151 * ((v1163 * v4278) / v1164))) } else { (if v1154 { (self.scalar_v3268 - (v1151 * ((v1155 * v4258) / v1156))) } else { v4 }) });
        let v4296: f64 = (if v1161 { (-(v1151 * ((v1163 * v4279) / v1164))) } else { (if v1154 { (self.scalar_v0 - (v1151 * ((v1155 * v4259) / v1156))) } else { v4 }) });
        let v4302: f64 = (-((v1168 * v2900) + (v308 * v4294)));
        let v4303: f64 = (-(v308 * v4295));
        let v4304: f64 = (-(v308 * v4296));
        let v4307: f64 = (self.scalar_v1171 * f64::powf(v1170, self.scalar_v4305));
        let v4311: f64 = (v2784 / self.scalar_v1171);
        let v4326: f64 = (((v1174 * v4311) + (v1173 * (-(v4302 * v4307)))) + (v171 * (-v4294)));
        let v4327: f64 = ((v1173 * (-(v4303 * v4307))) + (v171 * (self.scalar_v3268 - v4295)));
        let v4328: f64 = ((v1173 * (-(v4304 * v4307))) + (v171 * (self.scalar_v0 - v4296)));
        let v4334: f64 = (if self.scalar_v1184 { (self.scalar_v0 + (if v1112 { v4 } else { (if v942 { (v4032 + (((if v942 { (v1085 * v3554) } else { v4 }) + (v4049 + v4049)) / v4059)) } else { v4 }) })) } else { self.scalar_v4329 });
        let v4335: f64 = (if self.scalar_v1184 { (self.scalar_v3268 + (if v1112 { self.scalar_v0 } else { (if v942 { (v4033 + (((if v942 { (v1085 * v3555) } else { v4 }) + (v4051 + v4051)) / v4059)) } else { v4 }) })) } else { self.scalar_v4330 });
        let v4337: f64 = (if self.scalar_v1188 { v4 } else { (if self.scalar_v1184 { (if v1112 { v4 } else { v4068 }) } else { v4 }) });
        let v4338: f64 = (if self.scalar_v1188 { self.scalar_v0 } else { v4334 });
        let v4339: f64 = (if self.scalar_v1188 { v4 } else { v4335 });
        let v4340: f64 = (if self.scalar_v1188 { self.scalar_v3268 } else { (if self.scalar_v1184 { (if v1112 { self.scalar_v3268 } else { (if v942 { (v4034 + (((if v942 { (v1085 * v3556) } else { v4 }) + (v4053 + v4053)) / v4059)) } else { v4 }) }) } else { v4 }) });
        let v4341: f64 = (-v2933);
        let v4346: f64 = (((v1191 * v4341) - (v1190 * v4341)) / (v1191 * v1191));
        let v4354: f64 = ((v1195 * v2851) + (v261 * (-(v4346 * (self.scalar_v1193 * f64::powf(v1192, self.scalar_v4347))))));
        let v4359: f64 = (v1142 * v1142);
        let v4360: f64 = (((v1142 * (v4337 - v4354)) - (v1197 * v4230)) / v4359);
        let v4364: f64 = (((v1142 * v4338) - (v1197 * v4231)) / v4359);
        let v4368: f64 = (((v1142 * v4339) - (v1197 * v4232)) / v4359);
        let v4372: f64 = (((v1142 * v4340) - (v1197 * v4233)) / v4359);
        let v4429: f64 = (if v1206 { (v4354 - ((v1210 * v4230) + (v1142 * ((v1208 * (-v4360)) / v1209)))) } else { (if v1199 { (v4337 - ((v1202 * v4230) + (v1142 * ((v1200 * v4360) / v1201)))) } else { v4 }) });
        let v4430: f64 = (if v1206 { (-((v1210 * v4231) + (v1142 * ((v1208 * (-v4364)) / v1209)))) } else { (if v1199 { (v4338 - ((v1202 * v4231) + (v1142 * ((v1200 * v4364) / v1201)))) } else { v4 }) });
        let v4431: f64 = (if v1206 { (-((v1210 * v4232) + (v1142 * ((v1208 * (-v4368)) / v1209)))) } else { (if v1199 { (v4339 - ((v1202 * v4232) + (v1142 * ((v1200 * v4368) / v1201)))) } else { v4 }) });
        let v4432: f64 = (if v1206 { (-((v1210 * v4233) + (v1142 * ((v1208 * (-v4372)) / v1209)))) } else { (if v1199 { (v4340 - ((v1202 * v4233) + (v1142 * ((v1200 * v4372) / v1201)))) } else { v4 }) });
        let v4435: f64 = (self.scalar_v1214 * f64::powf(v1146, self.scalar_v4433));
        let v4436: f64 = (v4246 * v4435);
        let v4437: f64 = (v4247 * v4435);
        let v4438: f64 = (v4248 * v4435);
        let v4439: f64 = (v4249 * v4435);
        let v4440: f64 = (v2851 / self.scalar_v1216);
        let v4454: f64 = (self.scalar_v1216 * f64::powf(v1219, self.scalar_v4452));
        let v4477: f64 = ((v1222 * v4440) + (v1217 * (-((v1220 * v4436) + (v1215 * ((-(((v261 * v4429) - (v1213 * v2851)) / v2902)) * v4454))))));
        let v4504: f64 = ((v1217 * (-((v1220 * v4437) + (v1215 * ((-(v4430 / v261)) * v4454))))) + ((v1225 * (v1192 * v4437)) + (v1224 * (v4338 - v4430))));
        let v4505: f64 = ((v1217 * (-((v1220 * v4438) + (v1215 * ((-(v4431 / v261)) * v4454))))) + ((v1225 * (v1192 * v4438)) + (v1224 * (v4339 - v4431))));
        let v4506: f64 = ((v1217 * (-((v1220 * v4439) + (v1215 * ((-(v4432 / v261)) * v4454))))) + ((v1225 * (v1192 * v4439)) + (v1224 * (v4340 - v4432))));
        let v4512: f64 = (v1191 * v4506);
        let v4514: f64 = (self.scalar_v0 * v330);
        let v4515: f64 = (v330 * self.scalar_v3268);
        let v4516: f64 = (((v1227 * v4341) + (v1191 * (v4477 + ((v1225 * ((v1215 * v4346) + (v1192 * v4436))) + (v1224 * (v4337 - v4429)))))) + (v751 * v2933));
        let v4517: f64 = ((v1191 * v4504) + v4514);
        let v4518: f64 = ((v1191 * v4505) + v4515);
        let v4523: f64 = (v470 * v470);
        let v4524: f64 = (((v470 * (v452 * v3020)) - (v1231 * v3023)) / v4523);
        let v4527: f64 = ((v1232 * v3303) + (v817 * v4524));
        let v4528: f64 = (v1232 * v3304);
        let v4529: f64 = (v1232 * v3305);
        let v4530: f64 = (v32 * v1235);
        let v4531: f64 = (v4527 / v4530);
        let v4532: f64 = (v4528 / v4530);
        let v4533: f64 = (v4529 / v4530);
        let v4537: f64 = (v1236 * v1236);
        let v4538: f64 = (((v1236 * v4527) - (v1233 * v4531)) / v4537);
        let v4542: f64 = (((v1236 * v4528) - (v1233 * v4532)) / v4537);
        let v4546: f64 = (((v1236 * v4529) - (v1233 * v4533)) / v4537);
        let v4552: f64 = (v1238 * f64::powf(v1116, (v1238 - v1)));
        let v4555: f64 = (((-(if self.scalar_v430 { v4 } else { (if self.scalar_v402 { (if v420 { (v2980 + (v31 * ((v422 * (-v2976)) / v423))) } else { v2980 }) } else { v4 }) })) / (v431 * v431)) * (v1239 * ((v1116) as f64).ln()));
        let v4556: f64 = ((v4166 * v4552) + v4555);
        let v4557: f64 = (v4167 * v4552);
        let v4558: f64 = ((if v1112 { v4 } else { (if v942 { (v1077 * ((v1074 * v3999) + (v1073 * v3999))) } else { v4 }) }) * v4552);
        let v4559: f64 = (v4169 * v4552);
        let v4562: f64 = ((v1239 * v4524) + (v1232 * v4556));
        let v4563: f64 = (v1232 * v4557);
        let v4564: f64 = (v1232 * v4558);
        let v4565: f64 = (v1232 * v4559);
        let v4566: f64 = (v32 * v1242);
        let v4574: f64 = (v1243 * v1243);
        let v4575: f64 = (((v1243 * v4562) - (v1240 * (v4562 / v4566))) / v4574);
        let v4579: f64 = (((v1243 * v4563) - (v1240 * (v4563 / v4566))) / v4574);
        let v4583: f64 = (((v1243 * v4564) - (v1240 * (v4564 / v4566))) / v4574);
        let v4587: f64 = (((v1243 * v4565) - (v1240 * (v4565 / v4566))) / v4574);
        let v4592: f64 = (((v639 * v4326) - (v1178 * ((v638 * ((-v2907) / (v311 * v311))) + (v590 * (self.scalar_v637 * v3191))))) / (v639 * v639));
        let v4593: f64 = (v4327 / v639);
        let v4594: f64 = (v4328 / v639);
        let v4598: f64 = (v636 * v636);
        let v4601: f64 = (v4518 / v636);
        let v4602: f64 = (v4512 / v636);
        let v4603: f64 = (v4592 + (((v636 * v4516) - (v1230 * v3195)) / v4598));
        let v4604: f64 = (v4594 + (v4517 / v636));
        let v4646: f64 = (if self.scalar_v1251 { ((v1257 * v2713) + (v122 * ((v1256 * v3249) + (v718 * (((v636 * (-v4516)) - (v1255 * v3195)) / v4598))))) } else { v4 });
        let v4667: f64 = ((v1265 * ((v1260 * (if self.scalar_v1251 { ((v1252 * v2713) + (v122 * ((v1247 * v3249) + (v718 * v4592)))) } else { v4 })) - (v1261 * v4646))) - (v1262 * (v1264 * ((v718 * v2713) + (v122 * v3249)))));
        let v4671: f64 = (((v1260 * (if self.scalar_v1251 { (v122 * (v718 * v4594)) } else { v4 })) - (v1261 * (if self.scalar_v1251 { (v122 * (v718 * ((-v4517) / v636))) } else { v4 }))) / v1265);
        let v4674: f64 = (if self.scalar_v1251 { (v4667 / (v1265 * v1265)) } else { (if self.scalar_v1245 { v4603 } else { v4 }) });
        let v4675: f64 = (if self.scalar_v1251 { ((v1260 * (if self.scalar_v1251 { (v122 * (v718 * v4593)) } else { v4 })) / v1265) } else { (if self.scalar_v1245 { v4593 } else { v4 }) });
        let v4676: f64 = (if self.scalar_v1251 { v4671 } else { (if self.scalar_v1245 { v4604 } else { v4 }) });
        let v4677: f64 = (if self.scalar_v1251 { ((-(v1261 * (if self.scalar_v1251 { (v122 * (v718 * ((-v4518) / v636))) } else { v4 }))) / v1265) } else { (if self.scalar_v1245 { v4601 } else { v4 }) });
        let v4678: f64 = (if self.scalar_v1251 { ((-(v1261 * (if self.scalar_v1251 { (v122 * (v718 * ((-v4512) / v636))) } else { v4 }))) / v1265) } else { (if self.scalar_v1245 { v4602 } else { v4 }) });
        let v4679: f64 = (v1267 * v4674);
        let v4680: f64 = (v4679 + v4679);
        let v4681: f64 = (v1267 * v4675);
        let v4682: f64 = (v4681 + v4681);
        let v4683: f64 = (v1267 * v4676);
        let v4684: f64 = (v4683 + v4683);
        let v4685: f64 = (v1267 * v4677);
        let v4686: f64 = (v4685 + v4685);
        let v4687: f64 = (v1267 * v4678);
        let v4688: f64 = (v4687 + v4687);
        let v4689: f64 = (v32 * v1273);
        let v4690: f64 = (v4680 / v4689);
        let v4691: f64 = (v4682 / v4689);
        let v4692: f64 = (v4684 / v4689);
        let v4693: f64 = (v4686 / v4689);
        let v4694: f64 = (v4688 / v4689);
        let v4702: f64 = (v1274 * v1274);
        let v4745: f64 = ((v1283 * (if v1277 { (v440 * (v4674 + v4690)) } else { (if v1270 { ((-(v1271 * (v4690 - v4674))) / v4702) } else { v4 }) })) + (v1280 * (v440 * (v4538 + v4575))));
        let v4748: f64 = ((v1283 * (if v1277 { (v440 * (v4675 + v4691)) } else { (if v1270 { ((-(v1271 * (v4691 - v4675))) / v4702) } else { v4 }) })) + (v1280 * (v440 * v4542)));
        let v4751: f64 = ((v1283 * (if v1277 { (v440 * (v4676 + v4692)) } else { (if v1270 { ((-(v1271 * (v4692 - v4676))) / v4702) } else { v4 }) })) + (v1280 * (v440 * (v4546 + v4579))));
        let v4754: f64 = ((v1283 * (if v1277 { (v440 * (v4677 + v4693)) } else { (if v1270 { ((-(v1271 * (v4693 - v4677))) / v4702) } else { v4 }) })) + (v1280 * (v440 * v4583)));
        let v4757: f64 = ((v1283 * (if v1277 { (v440 * (v4678 + v4694)) } else { (if v1270 { ((-(v1271 * (v4694 - v4678))) / v4702) } else { v4 }) })) + (v1280 * (v440 * v4587)));
        let v4777: f64 = (v1284 * v1284);
        let v4868: f64 = (v3287 / self.scalar_v507);
        let v4869: f64 = (v3274 / self.scalar_v507);
        let v4870: f64 = (v3273 / self.scalar_v507);
        let v4875: f64 = (if v1344 { (v1345 * v4869) } else { (if v1299 { (self.scalar_v3268 + (v1291 * ((v1301 * self.scalar_v4805) / v1302))) } else { (if v1293 { (v1291 * ((v1294 * self.scalar_v4795) / v1295)) } else { v4 }) }) });
        let v4876: f64 = (if v1344 { (v1345 * v4870) } else { (if v1299 { (self.scalar_v0 + (v1291 * ((v1301 * self.scalar_v4806) / v1302))) } else { (if v1293 { (v1291 * ((v1294 * self.scalar_v4796) / v1295)) } else { v4 }) }) });
        let v5064: f64 = (v760 * v2713);
        let v5065: f64 = (v5064 / self.scalar_v518);
        let v5066: f64 = (v3274 / self.scalar_v518);
        let v5067: f64 = (v3273 / self.scalar_v518);
        let v5149: f64 = (v3287 / self.scalar_v479);
        let v5150: f64 = (v3274 / self.scalar_v479);
        let v5151: f64 = (v3273 / self.scalar_v479);
        let v5155: f64 = (if v1454 { (v1455 * v5149) } else { (if v1422 { (v1423 * v5065) } else { (if v1419 { (v1420 * v5065) } else { (if v1347 { (v1348 * v4868) } else { (if v1344 { (v1345 * v4868) } else { v4 }) }) }) }) });
        let v5163: f64 = (if v1457 { (v1458 * v5150) } else { (if v1454 { (v1455 * v5150) } else { (if v1422 { (v1423 * v5066) } else { (if v1419 { (v1420 * v5066) } else { (if v1347 { (v1348 * v4869) } else { v4875 }) }) }) }) });
        let v5172: f64 = (v5064 / self.scalar_v562);
        let v5173: f64 = (v3274 / self.scalar_v562);
        let v5174: f64 = (v3273 / self.scalar_v562);
        let v5181: f64 = (if v1466 { v4 } else { (if v1457 { (v1458 * v5151) } else { (if v1454 { (v1455 * v5151) } else { (if v1422 { v4 } else { (if v1419 { v4 } else { (if v1347 { (v1348 * v4870) } else { v4876 }) }) }) }) }) });
        let v5187: f64 = (if v1469 { (v1470 * v5174) } else { (if v1466 { (v1467 * v5174) } else { (if v1457 { v4 } else { (if v1454 { v4 } else { (if v1422 { (v1423 * v5067) } else { (if v1419 { (v1420 * v5067) } else { v4 }) }) }) }) }) });
        let v5195: f64 = (v3306 / self.scalar_v492);
        let v5196: f64 = (v3273 / self.scalar_v492);
        let v5197: f64 = (v3307 / self.scalar_v492);
        let v5198: f64 = (v3308 / self.scalar_v492);
        let v5199: f64 = (v3274 / self.scalar_v492);
        let v5216: f64 = (if v1481 { (v1482 * v5195) } else { (if v1478 { (v1479 * v5195) } else { (if v1469 { (v1470 * v5172) } else { (if v1466 { (v1467 * v5172) } else { (if v1457 { (v1458 * v5149) } else { v5155 }) }) }) }) });
        let v5230: f64 = (v5064 / self.scalar_v572);
        let v5231: f64 = (v3274 / self.scalar_v572);
        let v5232: f64 = (v3273 / self.scalar_v572);
        let v5246: f64 = (if v1493 { (v1494 * v5231) } else { (if v1490 { (v1491 * v5231) } else { (if v1481 { v4 } else { (if v1478 { v4 } else { (if v1469 { (v1470 * v5173) } else { (if v1466 { (v1467 * v5173) } else { v5163 }) }) }) }) }) });
        let v5758: f64 = ((v1232 * v3324) + (v827 * v4524));
        let v5759: f64 = (v1232 * v3325);
        let v5760: f64 = (v1232 * v3326);
        let v5761: f64 = (v1232 * v3327);
        let v5762: f64 = (v1232 * v3328);
        let v5763: f64 = (v452 * (if v894 { (v895 * v3436) } else { (if v891 { (v892 * v3436) } else { v4 }) }));
        let v5764: f64 = (v452 * (if v894 { (v895 * v3273) } else { (if v891 { (v892 * v3273) } else { v4 }) }));
        let v5765: f64 = (v452 * (if v894 { (v895 * v3307) } else { (if v891 { (v892 * v3307) } else { v4 }) }));
        let v5766: f64 = (v452 * (if v894 { (v895 * v3308) } else { (if v891 { (v892 * v3308) } else { v4 }) }));
        let v5767: f64 = (v452 * (if v894 { (v895 * v3274) } else { (if v891 { (v892 * v3274) } else { v4 }) }));
        let v5769: f64 = (v32 * v1685);
        let v5778: f64 = (v1686 * v1686);
        let v5796: f64 = (v32 * v1689);
        let v5805: f64 = (v1690 * v1690);
        let v6137: f64 = (self.scalar_v1770 * v3084);
        let v6149: f64 = ((v1696 * v3361) + (v847 * (((v476 * (v452 * v3084)) - (v1695 * (self.scalar_v471 * (v475 * (self.scalar_v473 * v2714))))) / (v476 * v476))));
        let v6152: f64 = (v32 * v1776);
        let v6161: f64 = (v1777 * v1777);
        let v6182: f64 = (if self.scalar_v1765 { (((v1777 * (v1771 * v3362)) - (v1773 * ((v1696 * v3362) / v6152))) / v6161) } else { v4 });
        let v6184: f64 = (self.scalar_v1782 * v3207);
        let v6189: f64 = (v1783 * v3359);
        let v6190: f64 = (v1783 * v3360);
        let v6196: f64 = (v1783 * v3362);
        let v6202: f64 = (((v662 * (v452 * v3207)) - (v1786 * (self.scalar_v658 * (v661 * (self.scalar_v659 * v2714))))) / (v662 * v662));
        let v6210: f64 = (v1787 * v3359);
        let v6211: f64 = (v1787 * v3360);
        let v6217: f64 = (v1787 * v3362);
        let v6219: f64 = (v32 * v1792);
        let v6230: f64 = (v1793 * v1793);
        let v6242: f64 = ((v1793 * ((v1784 * v6184) + (v1783 * (v3361 - v3391)))) - (v1785 * (((v1789 * v6202) + (v1787 * (v3361 + (self.scalar_v1711 * v3391)))) / v6219)));
        let v6271: f64 = (v32 * v1800);
        let v6280: f64 = (v1801 * v1801);
        let v6293: f64 = (((v1801 * v6196) - (v1797 * (v6217 / v6271))) / v6280);
        let v6298: f64 = (if self.scalar_v1796 { (((v1801 * v6189) - (v1797 * (v6210 / v6271))) / v6280) } else { (if self.scalar_v1780 { (((v1793 * v6189) - (v1785 * (v6210 / v6219))) / v6230) } else { v4 }) });
        let v6299: f64 = (if self.scalar_v1796 { (((v1801 * v6190) - (v1797 * (v6211 / v6271))) / v6280) } else { (if self.scalar_v1780 { (((v1793 * v6190) - (v1785 * (v6211 / v6219))) / v6230) } else { v4 }) });
        let v6301: f64 = (if self.scalar_v1796 { (((v1801 * ((v1783 * v3361) + (v1772 * v6184))) - (v1797 * (((v1787 * v3361) + (v847 * v6202)) / v6271))) / v6280) } else { (if self.scalar_v1780 { (v6242 / v6230) } else { v4 }) });
        let v6302: f64 = (if self.scalar_v1796 { v6293 } else { (if self.scalar_v1780 { (((v1793 * (v1783 * (v3362 - v3392))) - (v1785 * ((v1787 * (v3362 + (self.scalar_v1711 * v3392))) / v6219))) / v6230) } else { v4 }) });
        let v6304: f64 = (if self.scalar_v1796 { (((v1801 * (v1783 * v3363)) - (v1797 * ((v1787 * v3363) / v6271))) / v6280) } else { (if self.scalar_v1780 { (((v1793 * (v1783 * (v3363 - v3393))) - (v1785 * ((v1787 * (v3363 + (self.scalar_v1711 * v3393))) / v6219))) / v6230) } else { v4 }) });
        let v6310: f64 = (if self.scalar_v1805 { ((v1807 * v2947) + (v356 * (self.scalar_v13 * (v3084 + v3207)))) } else { v4 });
        let v6323: f64 = (if self.scalar_v1805 { (-(if self.scalar_v1805 { ((v1812 * v2710) + (v120 * (-(((v1809 * v2713) + (v122 * v6310)) / v1810)))) } else { v4 })) } else { v4 });
        let v6326: f64 = (v1816 * self.scalar_v6321);
        let v6327: f64 = (v6326 + v6326);
        let v6328: f64 = (v1816 * self.scalar_v6322);
        let v6330: f64 = (v1816 * v6323);
        let v6332: f64 = (v1816 * self.scalar_v6324);
        let v6333: f64 = (v6332 + v6332);
        let v6334: f64 = (v1816 * self.scalar_v6325);
        let v6345: f64 = (v32 * v1825);
        let v6346: f64 = ((if self.scalar_v1805 { v6327 } else { v4 }) / v6345);
        let v6347: f64 = ((if self.scalar_v1805 { (v6328 + v6328) } else { v4 }) / v6345);
        let v6348: f64 = ((if self.scalar_v1805 { (v6330 + v6330) } else { v4680 }) / v6345);
        let v6349: f64 = ((if self.scalar_v1805 { v4 } else { v4682 }) / v6345);
        let v6350: f64 = ((if self.scalar_v1805 { v6327 } else { v4684 }) / v6345);
        let v6351: f64 = ((if self.scalar_v1805 { v6333 } else { v4686 }) / v6345);
        let v6352: f64 = ((if self.scalar_v1805 { v6333 } else { v4688 }) / v6345);
        let v6353: f64 = ((if self.scalar_v1805 { (v6334 + v6334) } else { v4 }) / v6345);
        let v6354: f64 = ((if self.scalar_v1805 { v6333 } else { v4 }) / v6345);
        let v6365: f64 = (v1826 * v1826);
        let v6417: f64 = (if v1830 { (v440 * (self.scalar_v6321 + v6346)) } else { (if v1822 { ((-(self.scalar_v1823 * (v6346 - self.scalar_v6321))) / v6365) } else { v4 }) });
        let v6418: f64 = (if v1830 { (v440 * (self.scalar_v6322 + v6347)) } else { (if v1822 { ((-(self.scalar_v1823 * (v6347 - self.scalar_v6322))) / v6365) } else { v4 }) });
        let v6419: f64 = (if v1830 { (v440 * (v6323 + v6348)) } else { (if v1822 { ((-(self.scalar_v1823 * (v6348 - v6323))) / v6365) } else { v4 }) });
        let v6420: f64 = (if v1830 { (v440 * v6349) } else { (if v1822 { ((-(self.scalar_v1823 * v6349)) / v6365) } else { v4 }) });
        let v6421: f64 = (if v1830 { (v440 * (self.scalar_v6321 + v6350)) } else { (if v1822 { ((-(self.scalar_v1823 * (v6350 - self.scalar_v6321))) / v6365) } else { v4 }) });
        let v6422: f64 = (if v1830 { (v440 * (self.scalar_v6324 + v6351)) } else { (if v1822 { ((-(self.scalar_v1823 * (v6351 - self.scalar_v6324))) / v6365) } else { v4 }) });
        let v6423: f64 = (if v1830 { (v440 * (self.scalar_v6324 + v6352)) } else { (if v1822 { ((-(self.scalar_v1823 * (v6352 - self.scalar_v6324))) / v6365) } else { v4 }) });
        let v6424: f64 = (if v1830 { (v440 * (self.scalar_v6325 + v6353)) } else { (if v1822 { ((-(self.scalar_v1823 * (v6353 - self.scalar_v6325))) / v6365) } else { v4 }) });
        let v6425: f64 = (if v1830 { (v440 * (self.scalar_v6324 + v6354)) } else { (if v1822 { ((-(self.scalar_v1823 * (v6354 - self.scalar_v6324))) / v6365) } else { v4 }) });
        let v6432: f64 = (v356 * ((if self.scalar_v1765 { (((v1777 * (v1771 * v3359)) - (v1773 * ((v1696 * v3359) / v6152))) / v6161) } else { v4 }) + v6298));
        let v6434: f64 = (v356 * (if self.scalar_v1796 { v4 } else { (if self.scalar_v1780 { (((v1793 * (v1783 * (-v3390))) - (v1785 * ((v1787 * (self.scalar_v1711 * v3390)) / v6219))) / v6230) } else { v4 }) }));
        let v6437: f64 = ((v1834 * v2947) + (v356 * ((if self.scalar_v1765 { (((v1777 * ((v1772 * v6137) + (v1771 * v3361))) - (v1773 * (v6149 / v6152))) / v6161) } else { v4 }) + v6301)));
        let v6438: f64 = (v356 * (v6182 + v6302));
        let v6453: f64 = (v1837 * v1837);
        let v6456: f64 = (v1833 * (v6418 + (v356 * ((if self.scalar_v1765 { (((v1777 * (v1771 * v3360)) - (v1773 * ((v1696 * v3360) / v6152))) / v6161) } else { v4 }) + v6299))));
        let v6479: f64 = (v1833 * (v6423 + (v356 * (v6182 + (if self.scalar_v1796 { v6293 } else { (if self.scalar_v1780 { (((v1793 * v6196) - (v1785 * (v6217 / v6219))) / v6230) } else { v4 }) })))));
        let v6483: f64 = (v1833 * (v6424 + (v356 * ((if self.scalar_v1765 { (((v1777 * (v1771 * v3363)) - (v1773 * ((v1696 * v3363) / v6152))) / v6161) } else { v4 }) + v6304))));
        let v6842: f64 = (v1249 * v4603);
        let v6844: f64 = (v1249 * v4593);
        let v6846: f64 = (v1249 * v4604);
        let v6848: f64 = (v1249 * v4601);
        let v6850: f64 = (v1249 * v4602);
        let v6852: f64 = (v32 * v1905);
        let v6853: f64 = ((v6842 + v6842) / v6852);
        let v6854: f64 = ((v6844 + v6844) / v6852);
        let v6855: f64 = ((v6846 + v6846) / v6852);
        let v6856: f64 = ((v6848 + v6848) / v6852);
        let v6857: f64 = ((v6850 + v6850) / v6852);
        let v6865: f64 = (v1906 * v1906);
        let v6894: f64 = (if v1909 { (v440 * (v4603 + v6853)) } else { (if v1903 { ((-(v1271 * (v6853 - v4603))) / v6865) } else { v4 }) });
        let v6895: f64 = (if v1909 { (v440 * (v4593 + v6854)) } else { (if v1903 { ((-(v1271 * (v6854 - v4593))) / v6865) } else { v4 }) });
        let v6896: f64 = (if v1909 { (v440 * (v4604 + v6855)) } else { (if v1903 { ((-(v1271 * (v6855 - v4604))) / v6865) } else { v4 }) });
        let v6897: f64 = (if v1909 { (v440 * (v4601 + v6856)) } else { (if v1903 { ((-(v1271 * (v6856 - v4601))) / v6865) } else { v4 }) });
        let v6898: f64 = (if v1909 { (v440 * (v4602 + v6857)) } else { (if v1903 { ((-(v1271 * (v6857 - v4602))) / v6865) } else { v4 }) });
        let v8401: f64 = (self.scalar_v2241 * v2912);
        let v8409: f64 = ((v4253 - (v2244 * v4251)) / v4256);
        let v8442: f64 = (if v2253 { (v4250 - ((v2257 * v4251) + (v1151 * ((v2255 * (-v8409)) / v2256)))) } else { (if v2246 { (-((v2249 * v4251) + (v1151 * ((v2247 * v8409) / v2248)))) } else { v4 }) });
        let v8443: f64 = (if v2253 { (-(v1151 * ((v2255 * v4278) / v2256))) } else { (if v2246 { (self.scalar_v3268 - (v1151 * ((v2247 * v4258) / v2248))) } else { v4 }) });
        let v8444: f64 = (if v2253 { (-(v1151 * ((v2255 * v4279) / v2256))) } else { (if v2246 { (self.scalar_v0 - (v1151 * ((v2247 * v4259) / v2248))) } else { v4 }) });
        let v8455: f64 = (self.scalar_v1171 * f64::powf(v2263, self.scalar_v4305));
        let v8478: f64 = ((v2269 * (self.scalar_v2240 * v2912)) + (v2261 * (((v2265 * v4311) + (v1173 * (-((-((v2260 * v2900) + (v308 * v8442))) * v8455)))) + (v171 * (-v8442)))));
        let v8490: f64 = ((v683 * v3023) + (v470 * v3230));
        let v8491: f64 = (v440 * v8490);
        let v8499: f64 = ((v2276 * v6894) + (v1912 * ((v2275 * v4538) + (v1237 * v8491))));
        let v8502: f64 = ((v2276 * v6895) + (v1912 * (v2275 * v4542)));
        let v8505: f64 = ((v2276 * v6896) + (v1912 * (v2275 * v4546)));
        let v8506: f64 = (v2276 * v6897);
        let v8507: f64 = (v2276 * v6898);
        let v8516: f64 = ((v2278 * v6894) + (v1912 * ((v2275 * v4575) + (v1244 * v8491))));
        let v8517: f64 = (v2278 * v6895);
        let v8520: f64 = ((v2278 * v6896) + (v1912 * (v2275 * v4579)));
        let v8523: f64 = ((v2278 * v6897) + (v1912 * (v2275 * v4583)));
        let v8526: f64 = ((v2278 * v6898) + (v1912 * (v2275 * v4587)));
        let v8528: f64 = (v1096 * (-v4354));
        let v8531: f64 = (v1096 * v1096);
        let v8532: f64 = ((v8528 - (v2280 * v4072)) / v8531);
        let v8533: f64 = (self.scalar_v0 / v1096);
        let v8534: f64 = (self.scalar_v3269 / v1096);
        let v8535: f64 = (self.scalar_v3270 / v1096);
        let v8536: f64 = (self.scalar_v3268 / v1096);
        let v8566: f64 = (-v8534);
        let v8567: f64 = (-v8535);
        let v8568: f64 = (-v8536);
        let v8591: f64 = (if v2289 { (v4354 - ((v2293 * v4072) + (v1096 * ((v2291 * (-v8532)) / v2292)))) } else { (if v2282 { (-((v2285 * v4072) + (v1096 * ((v2283 * v8532) / v2284)))) } else { v4 }) });
        let v8592: f64 = (if v2289 { (-(v1096 * ((v2291 * (-v8533)) / v2292))) } else { (if v2282 { (self.scalar_v0 - (v1096 * ((v2283 * v8533) / v2284))) } else { v4 }) });
        let v8593: f64 = (if v2289 { (-(v1096 * ((v2291 * v8566) / v2292))) } else { (if v2282 { (self.scalar_v3269 - (v1096 * ((v2283 * v8534) / v2284))) } else { v4 }) });
        let v8594: f64 = (if v2289 { (-(v1096 * ((v2291 * v8567) / v2292))) } else { (if v2282 { (self.scalar_v3270 - (v1096 * ((v2283 * v8535) / v2284))) } else { v4 }) });
        let v8595: f64 = (if v2289 { (-(v1096 * ((v2291 * v8568) / v2292))) } else { (if v2282 { (self.scalar_v3268 - (v1096 * ((v2283 * v8536) / v2284))) } else { v4 }) });
        let v8610: f64 = (self.scalar_v1216 * f64::powf(v2298, self.scalar_v4452));
        let v8640: f64 = (((v2300 * v4440) + (v1217 * (-((-(((v261 * v8591) - (v2296 * v2851)) / v2902)) * v8610)))) + ((v2302 * v4346) + (v1192 * (-v8591))));
        let v8653: f64 = (v330 * self.scalar_v3269);
        let v8654: f64 = (v330 * self.scalar_v3270);
        let v8673: f64 = (self.scalar_v14 * (self.scalar_v2309 * (v329 * (v4514 + (v1191 * ((v1217 * (-((-(v8592 / v261)) * v8610))) + (v1192 * (self.scalar_v0 - v8592))))))));
        let v8674: f64 = (self.scalar_v14 * (self.scalar_v2309 * (v329 * ((v1191 * ((v1217 * (-((-(v8593 / v261)) * v8610))) + (v1192 * (self.scalar_v3269 - v8593)))) + v8653))));
        let v8675: f64 = (self.scalar_v14 * (self.scalar_v2309 * (v329 * ((v1191 * ((v1217 * (-((-(v8594 / v261)) * v8610))) + (v1192 * (self.scalar_v3270 - v8594)))) + v8654))));
        let v8676: f64 = (self.scalar_v14 * (self.scalar_v2309 * (v329 * (v4515 + (v1191 * ((v1217 * (-((-(v8595 / v261)) * v8610))) + (v1192 * (self.scalar_v3268 - v8595))))))));
        let v8677: f64 = (self.scalar_v3271 / v1096);
        let v8680: f64 = ((v8528 - (v2312 * v4072)) / v8531);
        let v8732: f64 = (if v2321 { (-(v1096 * ((v2323 * v8566) / v2324))) } else { (if v2314 { (self.scalar_v3269 - (v1096 * ((v2315 * v8534) / v2316))) } else { v4 }) });
        let v8733: f64 = (if v2321 { (-(v1096 * ((v2323 * (-v8677)) / v2324))) } else { (if v2314 { (self.scalar_v3271 - (v1096 * ((v2315 * v8677) / v2316))) } else { v4 }) });
        let v8734: f64 = (if v2321 { (v4354 - ((v2325 * v4072) + (v1096 * ((v2323 * (-v8680)) / v2324)))) } else { (if v2314 { (-((v2317 * v4072) + (v1096 * ((v2315 * v8680) / v2316)))) } else { v4 }) });
        let v8735: f64 = (if v2321 { (-(v1096 * ((v2323 * v8567) / v2324))) } else { (if v2314 { (self.scalar_v3270 - (v1096 * ((v2315 * v8535) / v2316))) } else { v4 }) });
        let v8736: f64 = (if v2321 { (-(v1096 * ((v2323 * v8568) / v2324))) } else { (if v2314 { (self.scalar_v3268 - (v1096 * ((v2315 * v8536) / v2316))) } else { v4 }) });
        let v8751: f64 = (self.scalar_v1216 * f64::powf(v2330, self.scalar_v4452));
        let v8783: f64 = (((v2332 * v4440) + (v1217 * (-((-(((v261 * v8734) - (v2328 * v2851)) / v2902)) * v8751)))) + ((v2334 * v4346) + (v1192 * (-v8734))));
        let v8808: f64 = (self.scalar_v2309 * (v329 * ((v1191 * ((v1217 * (-((-(v8733 / v261)) * v8751))) + (v1192 * (self.scalar_v3271 - v8733)))) + (v330 * self.scalar_v3271))));
        let v8812: f64 = (self.scalar_v13 * (self.scalar_v2309 * (v329 * (v8653 + (v1191 * ((v1217 * (-((-(v8732 / v261)) * v8751))) + (v1192 * (self.scalar_v3269 - v8732))))))));
        let v8815: f64 = (self.scalar_v13 * (self.scalar_v2309 * (v329 * (v8654 + (v1191 * ((v1217 * (-((-(v8735 / v261)) * v8751))) + (v1192 * (self.scalar_v3270 - v8735))))))));
        let v8816: f64 = (self.scalar_v13 * (self.scalar_v2309 * (v329 * (v4515 + (v1191 * ((v1217 * (-((-(v8736 / v261)) * v8751))) + (v1192 * (self.scalar_v3268 - v8736))))))));
        let v8817: f64 = (v47 * v2897);
        let v8818: f64 = (self.scalar_v2346 * v2897);
        let v8820: f64 = (self.scalar_v0 / v2343);
        let v8825: f64 = (((v2343 * (-v8818)) - (v2348 * v8817)) / (v2343 * v2343));
        let v8826: f64 = (self.scalar_v3268 / v2343);
        let v8861: f64 = (if v2357 { (-(v2343 * ((v2359 * (-v8820)) / v2360))) } else { (if v2350 { (self.scalar_v0 - (v2343 * ((v2351 * v8820) / v2352))) } else { v4 }) });
        let v8862: f64 = (if v2357 { (v8818 - ((v2361 * v8817) + (v2343 * ((v2359 * (-v8825)) / v2360)))) } else { (if v2350 { (-((v2353 * v8817) + (v2343 * ((v2351 * v8825) / v2352)))) } else { v4 }) });
        let v8863: f64 = (if v2357 { (-(v2343 * ((v2359 * (-v8826)) / v2360))) } else { (if v2350 { (self.scalar_v3268 - (v2343 * ((v2351 * v8826) / v2352))) } else { v4 }) });
        let v8876: f64 = (self.scalar_v2365 * f64::powf(v2368, self.scalar_v8874));
        let v8895: f64 = (((v2370 * (v2897 / self.scalar_v2365)) + (v2366 * (-((-(((v307 * v8862) - (v2364 * v2897)) / v2915)) * v8876)))) + (v32 * (-v8862)));
        let v8913: f64 = (v2380 * ((v677 * v3023) + (v470 * ((v676 * (self.scalar_v668 * (v671 * (self.scalar_v669 * v2714)))) + (v672 * (v676 * (self.scalar_v674 * v2713)))))));
        let v8916: f64 = (self.scalar_v2378 * v2710);
        let v8919: f64 = (v2382 * v2382);
        let v8920: f64 = ((-(v757 * v8916)) / v8919);
        let v8921: f64 = (self.scalar_v3268 / v2382);
        let v8922: f64 = (self.scalar_v0 / v2382);
        let v8929: f64 = (if v2384 { (v2385 * v8922) } else { (if v1493 { v4 } else { (if v1490 { v4 } else { (if v1481 { (v1482 * v5197) } else { (if v1478 { (v1479 * v5197) } else { (if v1469 { v4 } else { v5181 }) }) }) }) }) });
        let v8937: f64 = (if v2387 { v4 } else { (if v2384 { v4 } else { (if v1493 { (v1494 * v5232) } else { (if v1490 { (v1491 * v5232) } else { (if v1481 { (v1482 * v5196) } else { (if v1478 { (v1479 * v5196) } else { v5187 }) }) }) }) }) });
        let v8943: f64 = ((v2392 * (v8913 + (v2376 * ((((v470 * v3020) - (v465 * v3023)) / v4523) * (self.scalar_v2379 * f64::powf(v2377, self.scalar_v8909)))))) + (v2381 * (if v2387 { (v2388 * v8920) } else { (if v2384 { (v2385 * v8920) } else { (if v1493 { (v1494 * v5230) } else { (if v1490 { (v1491 * v5230) } else { v5216 }) }) }) })));
        let v8944: f64 = (v2381 * (if v2387 { (v2388 * v8921) } else { (if v2384 { (v2385 * v8921) } else { v5246 }) }));
        let v8945: f64 = (v2381 * v8937);
        let v8946: f64 = (v2381 * (if v2387 { (v2388 * v8922) } else { v8929 }));
        let v8947: f64 = (v2381 * (if v2387 { v4 } else { (if v2384 { v4 } else { (if v1493 { v4 } else { (if v1490 { v4 } else { (if v1481 { (v1482 * v5198) } else { (if v1478 { (v1479 * v5198) } else { v4 }) }) }) }) }) }));
        let v8948: f64 = (v2381 * (if v2387 { v4 } else { (if v2384 { v4 } else { (if v1493 { v4 } else { (if v1490 { v4 } else { (if v1481 { (v1482 * v5199) } else { (if v1478 { (v1479 * v5199) } else { v4 }) }) }) }) }) }));
        let v8956: f64 = (((v368 * ((v2394 * v2710) + (v120 * (v452 * v3233)))) - (v2395 * v2954)) / v3552);
        let v8958: f64 = (v2397 * (if v1136 { (((v1138 * v3543) - (v939 * v3543)) / v4208) } else { (if v1128 { (((v1132 * v4177) - (v1131 * v4177)) / v4184) } else { v3884 }) }));
        let v8963: f64 = (v2397 * (if v1136 { (((v1138 * v3546) - (v939 * v3548)) / v4208) } else { (if v1128 { (((v1132 * v4180) - (v1131 * v4180)) / v4184) } else { v3887 }) }));
        let v8991: f64 = (((v2274 * (((v1686 * (v5758 - v4524)) - (v1683 * (v5758 / v5769))) / v5778)) + (v1687 * v8490)) + ((v2396 * (((v1690 * v5763) - (v1682 * (v5763 / v5796))) / v5805)) + (v1691 * v8956)));
        let v8992: f64 = ((v2274 * (((v1686 * v5759) - (v1683 * (v5759 / v5769))) / v5778)) + (v2396 * (((v1690 * v5764) - (v1682 * (v5764 / v5796))) / v5805)));
        let v8993: f64 = ((v2274 * (((v1686 * v5760) - (v1683 * (v5760 / v5769))) / v5778)) + (v2396 * (((v1690 * v5765) - (v1682 * (v5765 / v5796))) / v5805)));
        let v8994: f64 = ((v2274 * (((v1686 * v5761) - (v1683 * (v5761 / v5769))) / v5778)) + (v2396 * (((v1690 * v5766) - (v1682 * (v5766 / v5796))) / v5805)));
        let v8995: f64 = ((v2274 * (((v1686 * v5762) - (v1683 * (v5762 / v5769))) / v5778)) + (v2396 * (((v1690 * v5767) - (v1682 * (v5767 / v5796))) / v5805)));
        let v9006: f64 = (v690 * v690);
        let v9017: f64 = (-v2829);
        let v9025: f64 = ((v2412 * v2713) + (v122 * (v9017 / self.scalar_v2411)));
        let v9026: f64 = (v122 * self.scalar_v9019);
        let v9027: f64 = (v122 * self.scalar_v9020);
        let v9028: f64 = (v122 * self.scalar_v9021);
        let v9029: f64 = (v122 * self.scalar_v9022);
        let v9065: f64 = (v32 * v2430);
        let v9073: f64 = ((v2431 * ((v2426 * v3324) + (v827 * ((v1692 * v3239) + (v699 * (v32 * v3084)))))) - (v2427 * ((v452 * (if v2420 { (v2421 * v9025) } else { (if v2416 { (v2417 * v9025) } else { v4 }) })) / v9065)));
        let v9074: f64 = (v2431 * v2431);
        let v9079: f64 = (((v2431 * (v2426 * v3325)) - (v2427 * ((v452 * (if v2420 { (v2421 * v9026) } else { (if v2416 { (v2417 * v9026) } else { v4 }) })) / v9065))) / v9074);
        let v9083: f64 = (((v2431 * (v2426 * v3326)) - (v2427 * ((v452 * (if v2420 { (v2421 * v9027) } else { (if v2416 { (v2417 * v9027) } else { v4 }) })) / v9065))) / v9074);
        let v9087: f64 = (((v2431 * (v2426 * v3327)) - (v2427 * ((v452 * (if v2420 { (v2421 * v9028) } else { (if v2416 { (v2417 * v9028) } else { v4 }) })) / v9065))) / v9074);
        let v9091: f64 = (((v2431 * (v2426 * v3328)) - (v2427 * ((v452 * (if v2420 { (v2421 * v9029) } else { (if v2416 { (v2417 * v9029) } else { v4 }) })) / v9065))) / v9074);
        let v9092: f64 = (if self.scalar_v2415 { (v9073 / v9074) } else { (if self.scalar_v2402 { (((v690 * ((v2406 * (v440 * v3236)) + (v2403 * v8991))) - (v2407 * v3234)) / v9006) } else { v4 }) });
        let v9093: f64 = (if self.scalar_v2415 { v9079 } else { (if self.scalar_v2402 { ((v2403 * v8992) / v690) } else { v4 }) });
        let v9094: f64 = (if self.scalar_v2415 { v9083 } else { (if self.scalar_v2402 { ((v2403 * v8993) / v690) } else { v4 }) });
        let v9095: f64 = (if self.scalar_v2415 { v9087 } else { (if self.scalar_v2402 { ((v2403 * v8994) / v690) } else { v4 }) });
        let v9096: f64 = (if self.scalar_v2415 { v9091 } else { (if self.scalar_v2402 { ((v2403 * v8995) / v690) } else { v4 }) });
        let v9114: f64 = (if self.scalar_v2439 { (v1232 * v3359) } else { v4 });
        let v9115: f64 = (if self.scalar_v2439 { (v1232 * v3360) } else { v4 });
        let v9116: f64 = (if self.scalar_v2439 { ((v1232 * v3361) + (v847 * v4524)) } else { v4 });
        let v9117: f64 = (if self.scalar_v2439 { (v1232 * v3362) } else { v4 });
        let v9118: f64 = (if self.scalar_v2439 { (v1232 * v3363) } else { v4 });
        let v9120: f64 = (v32 * v2444);
        let v9129: f64 = (v2445 * v2445);
        let v9157: f64 = (if self.scalar_v2439 { (v452 * (if v883 { (v884 * v3307) } else { (if v880 { (v881 * v3307) } else { v4 }) })) } else { v4 });
        let v9158: f64 = (if self.scalar_v2439 { (v452 * (if v883 { (v884 * v3342) } else { (if v880 { (v881 * v3342) } else { v4 }) })) } else { v4 });
        let v9159: f64 = (if self.scalar_v2439 { (v452 * (if v883 { (v884 * v3414) } else { (if v880 { (v881 * v3414) } else { v4 }) })) } else { v4 });
        let v9160: f64 = (if self.scalar_v2439 { (v452 * (if v883 { (v884 * v3308) } else { (if v880 { (v881 * v3308) } else { v4 }) })) } else { v4 });
        let v9161: f64 = (if self.scalar_v2439 { (v452 * (if v883 { (v884 * v3274) } else { (if v880 { (v881 * v3274) } else { v4 }) })) } else { v4 });
        let v9162: f64 = (v32 * v2451);
        let v9171: f64 = (v2452 * v2452);
        let v9209: f64 = ((v2274 * (if self.scalar_v2439 { (((v2445 * v9114) - (v2442 * (v9114 / v9120))) / v9129) } else { v4 })) + (v2396 * (if self.scalar_v2439 { (((v2452 * v9157) - (v2449 * (v9157 / v9162))) / v9171) } else { v4 })));
        let v9210: f64 = ((v2274 * (if self.scalar_v2439 { (((v2445 * v9115) - (v2442 * (v9115 / v9120))) / v9129) } else { v4 })) + (v2396 * (if self.scalar_v2439 { (((v2452 * v9158) - (v2449 * (v9158 / v9162))) / v9171) } else { v4 })));
        let v9211: f64 = (((v2447 * v8490) + (v2274 * (if self.scalar_v2439 { (((v2445 * (v9116 - v4524)) - (v2442 * (v9116 / v9120))) / v9129) } else { v4 }))) + ((v2454 * v8956) + (v2396 * (if self.scalar_v2439 { (((v2452 * v9159) - (v2449 * (v9159 / v9162))) / v9171) } else { v4 }))));
        let v9212: f64 = ((v2274 * (if self.scalar_v2439 { (((v2445 * v9117) - (v2442 * (v9117 / v9120))) / v9129) } else { v4 })) + (v2396 * (if self.scalar_v2439 { (((v2452 * v9160) - (v2449 * (v9160 / v9162))) / v9171) } else { v4 })));
        let v9213: f64 = ((v2274 * (if self.scalar_v2439 { (((v2445 * v9118) - (v2442 * (v9118 / v9120))) / v9129) } else { v4 })) + (v2396 * (if self.scalar_v2439 { (((v2452 * v9161) - (v2449 * (v9161 / v9162))) / v9171) } else { v4 })));
        let v9236: f64 = ((v2463 * v2713) + (v122 * v9017));
        let v9272: f64 = (v32 * v2481);
        let v9281: f64 = (v2482 * v2482);
        let v9282: f64 = (((v2482 * (v2477 * v3359)) - (v2478 * ((v452 * (if v2471 { (v2472 * v3307) } else { (if v2467 { (v2468 * v3307) } else { v4 }) })) / v9272))) / v9281);
        let v9286: f64 = (((v2482 * (v2477 * v3360)) - (v2478 * ((v452 * (if v2471 { (v2472 * v3342) } else { (if v2467 { (v2468 * v3342) } else { v4 }) })) / v9272))) / v9281);
        let v9289: f64 = ((v2482 * ((v2477 * v3361) + (v847 * ((v1771 * v3239) + (v699 * v6137))))) - (v2478 * ((v452 * (if v2471 { (v2472 * v9236) } else { (if v2467 { (v2468 * v9236) } else { v4 }) })) / v9272)));
        let v9294: f64 = (((v2482 * (v2477 * v3362)) - (v2478 * ((v452 * (if v2471 { (v2472 * v3308) } else { (if v2467 { (v2468 * v3308) } else { v4 }) })) / v9272))) / v9281);
        let v9298: f64 = (((v2482 * (v2477 * v3363)) - (v2478 * ((v452 * (if v2471 { (v2472 * v3274) } else { (if v2467 { (v2468 * v3274) } else { v4 }) })) / v9272))) / v9281);
        let v9301: f64 = (if self.scalar_v2466 { (v9289 / v9281) } else { (if self.scalar_v2439 { (((v690 * ((v2459 * (self.scalar_v2455 * v3236)) + (v2456 * v9211))) - (v2460 * v3234)) / v9006) } else { v4 }) });
        let v9305: f64 = (v1842 * (if self.scalar_v2466 { v9282 } else { (if self.scalar_v2439 { ((v2456 * v9209) / v690) } else { v4 }) }));
        let v9309: f64 = ((v2484 * (if self.scalar_v1841 { v4 } else { (if self.scalar_v1805 { (((v1837 * v6418) - v6456) / v6453) } else { v4 }) })) + (v1842 * (if self.scalar_v2466 { v9286 } else { (if self.scalar_v2439 { ((v2456 * v9210) / v690) } else { v4 }) })));
        let v9313: f64 = ((v2484 * (if self.scalar_v1841 { v4 } else { (if self.scalar_v1805 { (((v1837 * v6419) - (v1833 * (v6419 + (v6310 + v6437)))) / v6453) } else { v4 }) })) + (v1842 * v9301));
        let v9318: f64 = (v1842 * (if self.scalar_v2466 { v9294 } else { (if self.scalar_v2439 { ((v2456 * v9212) / v690) } else { v4 }) }));
        let v9324: f64 = ((v2484 * (if self.scalar_v1841 { v4 } else { (if self.scalar_v1805 { (((v1837 * v6424) - v6483) / v6453) } else { v4 }) })) + (v1842 * (if self.scalar_v2466 { v9298 } else { (if self.scalar_v2439 { ((v2456 * v9213) / v690) } else { v4 }) })));
        let v9339: f64 = (self.scalar_v2489 * f64::powf(v1170, self.scalar_v9337));
        let v9346: f64 = (if self.scalar_v2488 { v4257 } else { v4 });
        let v9347: f64 = (if self.scalar_v2488 { v4258 } else { v4 });
        let v9348: f64 = (if self.scalar_v2488 { v4259 } else { v4 });
        let v9353: f64 = (v2497 * v2497);
        let v9365: f64 = (v2503 * (-v9346));
        let v9366: f64 = (v2503 * (-v9347));
        let v9367: f64 = (v2503 * (-v9348));
        let v9371: f64 = (v2504 * v2504);
        let v9386: f64 = ((v2506 * (if self.scalar_v2488 { (v4302 * v9339) } else { v4 })) + (v2492 * (if v2501 { (((v2504 * v9365) - (v2503 * v9365)) / v9371) } else { (if v2495 { ((-(v2496 * v9346)) / v9353) } else { v4 }) })));
        let v9389: f64 = ((v2506 * (if self.scalar_v2488 { (v4303 * v9339) } else { v4 })) + (v2492 * (if v2501 { (((v2504 * v9366) - (v2503 * v9366)) / v9371) } else { (if v2495 { ((-(v2496 * v9347)) / v9353) } else { v4 }) })));
        let v9392: f64 = ((v2506 * (if self.scalar_v2488 { (v4304 * v9339) } else { v4 })) + (v2492 * (if v2501 { (((v2504 * v9367) - (v2503 * v9367)) / v9371) } else { (if v2495 { ((-(v2496 * v9348)) / v9353) } else { v4 }) })));
        let v9417: f64 = (v1235 * v1235);
        let v9427: f64 = ((v2514 * (((v400 * ((v1233 * v2713) + (v122 * v4527))) - (v2512 * v2971)) / v3008)) + (v2513 * ((-(v440 * v4531)) / v9417)));
        let v9449: f64 = ((v2517 * (if self.scalar_v2488 { ((v2514 * ((v122 * v4528) / v400)) + (v2513 * ((-(v440 * v4532)) / v9417))) } else { v4 })) + (v2516 * (v2275 * v6895)));
        let v9452: f64 = ((v2517 * (if self.scalar_v2488 { ((v2514 * ((v122 * v4529) / v400)) + (v2513 * ((-(v440 * v4533)) / v9417))) } else { v4 })) + (v2516 * (v2275 * v6896)));
        let v9473: f64 = (if self.scalar_v2488 { (v8947 / v2382) } else { v4 });
        let v9477: f64 = ((if self.scalar_v2488 { ((v2509 * v8401) + (v2242 * (if self.scalar_v2488 { v9386 } else { v4 }))) } else { v4 }) + (if self.scalar_v2488 { ((v2517 * (if self.scalar_v2488 { v9427 } else { v4 })) + (v2516 * ((v2275 * v6894) + (v1912 * v8491)))) } else { v4 }));
        let v9492: f64 = ((v2524 * self.scalar_v9476) + (v2522 * ((if self.scalar_v2488 { (v8946 / v2382) } else { v4 }) + ((if self.scalar_v2488 { (v2242 * (if self.scalar_v2488 { v9392 } else { v4 })) } else { v4 }) + (if self.scalar_v2488 { v9452 } else { v4 })))));
        let v9497: f64 = (if self.scalar_v2488 { (v2522 * ((if self.scalar_v2488 { (v8944 / v2382) } else { v4 }) + ((if self.scalar_v2488 { (v2242 * (if self.scalar_v2488 { v9389 } else { v4 })) } else { v4 }) + (if self.scalar_v2488 { v9449 } else { v4 })))) } else { v4 });
        let v9519: f64 = (self.scalar_v2527 * v8947);
        let v9526: f64 = (if self.scalar_v2488 { (v8499 + (self.scalar_v2527 * v8943)) } else { v4 });
        let v9527: f64 = (if self.scalar_v2488 { (v8502 + (self.scalar_v2527 * v8944)) } else { v4 });
        let v9528: f64 = (if self.scalar_v2488 { (self.scalar_v2527 * v8945) } else { v4 });
        let v9529: f64 = (if self.scalar_v2488 { (v8505 + (self.scalar_v2527 * v8946)) } else { v4 });
        let v9530: f64 = (if self.scalar_v2488 { (v8506 + v9519) } else { v4 });
        let v9531: f64 = (if self.scalar_v2488 { (v8507 + v9519) } else { v4 });
        let v9532: f64 = (if self.scalar_v2488 { (self.scalar_v2527 * v8948) } else { v4 });
        let v9566: f64 = (if self.scalar_v2541 { v8499 } else { (if self.scalar_v2488 { (self.scalar_v2538 * v9526) } else { v4 }) });
        let v9567: f64 = (if self.scalar_v2541 { v8502 } else { (if self.scalar_v2488 { (self.scalar_v2538 * v9527) } else { v4 }) });
        let v9568: f64 = (if self.scalar_v2541 { v4 } else { (if self.scalar_v2488 { (self.scalar_v2538 * v9528) } else { v4 }) });
        let v9569: f64 = (if self.scalar_v2541 { v8505 } else { (if self.scalar_v2488 { (self.scalar_v2538 * v9529) } else { v4 }) });
        let v9570: f64 = (if self.scalar_v2541 { v8506 } else { (if self.scalar_v2488 { (self.scalar_v2538 * v9530) } else { v4 }) });
        let v9571: f64 = (if self.scalar_v2541 { v8507 } else { (if self.scalar_v2488 { (self.scalar_v2538 * v9531) } else { v4 }) });
        let v9572: f64 = (if self.scalar_v2541 { v4 } else { (if self.scalar_v2488 { (self.scalar_v2538 * v9532) } else { v4 }) });
        let v9573: f64 = (if self.scalar_v2541 { v8516 } else { (if self.scalar_v2488 { (v8516 + (self.scalar_v2534 * v9526)) } else { v4 }) });
        let v9574: f64 = (if self.scalar_v2541 { v8517 } else { (if self.scalar_v2488 { (v8517 + (self.scalar_v2534 * v9527)) } else { v4 }) });
        let v9575: f64 = (if self.scalar_v2541 { v4 } else { (if self.scalar_v2488 { (self.scalar_v2534 * v9528) } else { v4 }) });
        let v9576: f64 = (if self.scalar_v2541 { v8520 } else { (if self.scalar_v2488 { (v8520 + (self.scalar_v2534 * v9529)) } else { v4 }) });
        let v9577: f64 = (if self.scalar_v2541 { v8523 } else { (if self.scalar_v2488 { (v8523 + (self.scalar_v2534 * v9530)) } else { v4 }) });
        let v9578: f64 = (if self.scalar_v2541 { v8526 } else { (if self.scalar_v2488 { (v8526 + (self.scalar_v2534 * v9531)) } else { v4 }) });
        let v9579: f64 = (if self.scalar_v2541 { v4 } else { (if self.scalar_v2488 { (self.scalar_v2534 * v9532) } else { v4 }) });
        let v9584: f64 = (if self.scalar_v2541 { v8947 } else { (if self.scalar_v2488 { (self.scalar_v2528 * v8947) } else { v4 }) });
        let v9586: f64 = 1.0;
        let v9588: f64 = (self.scalar_v27 * (self.scalar_v2545 * v9586));
        let v9609: f64 = (((v1284 * (((v1286 * v4556) + (v1239 * (self.scalar_v1285 * v3020))) + ((v817 * v3020) + (v465 * v3303)))) - (v2581 * v4745)) / v4777);
        let v9625: f64 = (v2582 * v2582);
        let v9645: f64 = (((v2582 * (v9569 + v9576)) - (v2591 * (((v1284 * ((v1286 * v4557) + (v465 * v3305))) - (v2581 * v4751)) / v4777))) / v9625);
        let v9684: f64 = (if v2594 { ((v2595 * v4745) + (v1284 * ((v1912 * v3230) + (v683 * v6894)))) } else { (if v2590 { (((v2582 * (v9566 + v9573)) - (v2591 * v9609)) / v9625) } else { v4 }) });
        let v9685: f64 = (if v2594 { ((v2595 * v4748) + (v1284 * (v683 * v6895))) } else { (if v2590 { (((v2582 * (v9567 + v9574)) - (v2591 * (((v1284 * (v465 * v3304)) - (v2581 * v4748)) / v4777))) / v9625) } else { v4 }) });
        let v9686: f64 = (if v2594 { v4 } else { (if v2590 { ((v9568 + v9575) / v2582) } else { v4 }) });
        let v9687: f64 = (if v2594 { ((v2595 * v4751) + (v1284 * (v683 * v6896))) } else { (if v2590 { v9645 } else { v4 }) });
        let v9688: f64 = (if v2594 { ((v2595 * v4754) + (v1284 * (v683 * v6897))) } else { (if v2590 { (((v2582 * (v9570 + v9577)) - (v2591 * (((v1284 * (v1286 * v4558)) - (v2581 * v4754)) / v4777))) / v9625) } else { v4 }) });
        let v9689: f64 = (if v2594 { ((v2595 * v4757) + (v1284 * (v683 * v6898))) } else { (if v2590 { (((v2582 * (v9571 + v9578)) - (v2591 * (((v1284 * (v1286 * v4559)) - (v2581 * v4757)) / v4777))) / v9625) } else { v4 }) });
        let v9690: f64 = (if v2594 { v4 } else { (if v2590 { ((v9572 + v9579) / v2582) } else { v4 }) });
        let v9750: f64 = (((v2399 * (v8958 + (v1140 * (v440 * v8956)))) + (v2398 * v4170)) + (((v2272 * v4516) + (v1230 * (self.scalar_v2271 * v2932))) + v9573));
        let v9754: f64 = ((self.scalar_v14 * (self.scalar_v2309 * ((v2307 * v2932) + (v329 * (((v2304 * v4341) + (v1191 * v8640)) + (v787 * v2933)))))) + (if self.scalar_v2436 { (self.scalar_v14 * v9092) } else { v9092 }));
        let v9759: f64 = (v8812 + (if self.scalar_v2436 { ((v2484 * (if self.scalar_v1841 { v4 } else { (if self.scalar_v1805 { (((v1837 * v6417) - (v1833 * (v6417 + v6432))) / v6453) } else { v4 }) })) + v9305) } else { v4 }));
        let v9761: f64 = ((self.scalar_v13 * (self.scalar_v2309 * ((v2339 * v2932) + (v329 * (((v2336 * v4341) + (v1191 * v8783)) + (v792 * v2933)))))) + (if self.scalar_v2436 { v9313 } else { v4 }));
        let v9762: f64 = (v8812 + (if self.scalar_v2436 { (v9305 + (v2484 * (if self.scalar_v1841 { v4 } else { (if self.scalar_v1805 { (((v1837 * v6421) - (v1833 * (v6421 + v6432))) / v6453) } else { v4 }) }))) } else { v4 }));
        let v9763: f64 = (v8815 + (if self.scalar_v2436 { ((v2484 * (if self.scalar_v1841 { v4 } else { (if self.scalar_v1805 { (((v1837 * v6422) - (v1833 * (v6422 + v6438))) / v6453) } else { v4 }) })) + v9318) } else { v4 }));
        let v9766: f64 = (v8815 + (if self.scalar_v2436 { (v9318 + (v2484 * (if self.scalar_v1841 { v4 } else { (if self.scalar_v1805 { (((v1837 * v6425) - (v1833 * (v6425 + v6438))) / v6453) } else { v4 }) }))) } else { v4 }));
        let v9987: f64 = (self.scalar_v27 * (v9586 * (self.scalar_v0 * ((if self.scalar_v2541 { v8943 } else { (if self.scalar_v2488 { (self.scalar_v2528 * v8943) } else { v4 }) }) + (((v2242 * v4326) + (v1178 * v8401)) + v9566)))));
        let v9988: f64 = (self.scalar_v27 * (v9586 * (self.scalar_v0 * ((if self.scalar_v2541 { v8944 } else { (if self.scalar_v2488 { (self.scalar_v2528 * v8944) } else { v4 }) }) + ((v2242 * v4327) + v9567)))));
        let v9989: f64 = (self.scalar_v27 * (v9586 * (self.scalar_v0 * (v9568 + (if self.scalar_v2541 { v8945 } else { (if self.scalar_v2488 { (self.scalar_v2528 * v8945) } else { v4 }) })))));
        let v9990: f64 = (self.scalar_v27 * (v9586 * (self.scalar_v0 * ((if self.scalar_v2541 { v8946 } else { (if self.scalar_v2488 { (self.scalar_v2528 * v8946) } else { v4 }) }) + ((v2242 * v4328) + v9569)))));
        let v9991: f64 = (self.scalar_v27 * (v9586 * (self.scalar_v0 * (v9570 + v9584))));
        let v9992: f64 = (self.scalar_v27 * (v9586 * (self.scalar_v0 * (v9571 + v9584))));
        let v9993: f64 = (self.scalar_v27 * (v9586 * (self.scalar_v0 * (v9572 + (if self.scalar_v2541 { v8948 } else { (if self.scalar_v2488 { (self.scalar_v2528 * v8948) } else { v4 }) })))));
        let v10000: f64 = (self.scalar_v27 * (v9586 * (self.scalar_v0 * v8478)));
        let v10001: f64 = (self.scalar_v27 * (v9586 * (self.scalar_v0 * (v2261 * ((v1173 * (-((-(v308 * v8443)) * v8455))) + (v171 * (self.scalar_v3268 - v8443)))))));
        let v10002: f64 = (self.scalar_v27 * (v9586 * (self.scalar_v0 * (v2261 * ((v1173 * (-((-(v308 * v8444)) * v8455))) + (v171 * (self.scalar_v0 - v8444)))))));
        let v10017: f64 = (self.scalar_v27 * (v9586 * (self.scalar_v0 * v9750)));
        let v10018: f64 = (self.scalar_v27 * (v9586 * (self.scalar_v0 * v9574)));
        let v10019: f64 = (self.scalar_v27 * (v9586 * (self.scalar_v0 * v9575)));
        let v10020: f64 = (self.scalar_v27 * (v9586 * (self.scalar_v0 * (((v2399 * (v2397 * v4223)) + (v2398 * v4171)) + ((v2272 * v4517) + v9576)))));
        let v10021: f64 = (self.scalar_v27 * (v9586 * (self.scalar_v0 * (((v2399 * (v2397 * v4224)) + (v2398 * v4172)) + ((v2272 * v4518) + v9577)))));
        let v10022: f64 = (self.scalar_v27 * (v9586 * (self.scalar_v0 * (((v2399 * v8963) + (v2398 * v4165)) + ((v2272 * v4512) + v9578)))));
        let v10023: f64 = (self.scalar_v27 * (v9586 * (self.scalar_v0 * v9579)));
        let v10030: f64 = (self.scalar_v27 * (v9586 * (self.scalar_v0 * (v320 * ((v2366 * (-((-(v8861 / v307)) * v8876))) + (v32 * (self.scalar_v0 - v8861)))))));
        let v10031: f64 = (self.scalar_v27 * (v9586 * (self.scalar_v0 * ((v2374 * (self.scalar_v316 * (((-(self.scalar_v285 * v2897)) / v2915) * (self.scalar_v318 * f64::powf(v317, self.scalar_v2917))))) + (v320 * v8895)))));
        let v10032: f64 = (self.scalar_v27 * (v9586 * (self.scalar_v0 * (v320 * ((v2366 * (-((-(v8863 / v307)) * v8876))) + (v32 * (self.scalar_v3268 - v8863)))))));
        let v10047: f64 = (self.scalar_v27 * (v9586 * (self.scalar_v0 * (if self.scalar_v2488 { (v2522 * ((if self.scalar_v2488 { (((v2382 * v8943) - (v2393 * v8916)) / v8919) } else { v4 }) + v9477)) } else { v4 }))));
        let v10048: f64 = (self.scalar_v27 * (v9586 * (self.scalar_v0 * v9497)));
        let v10049: f64 = (self.scalar_v27 * (v9586 * (self.scalar_v0 * (if self.scalar_v2488 { ((v2524 * self.scalar_v9475) + (v2522 * (if self.scalar_v2488 { (v8945 / v2382) } else { v4 }))) } else { v4 }))));
        let v10050: f64 = (self.scalar_v27 * (v9586 * (self.scalar_v0 * (if self.scalar_v2488 { v9492 } else { v4 }))));
        let v10051: f64 = (self.scalar_v27 * (v9586 * (self.scalar_v0 * (if self.scalar_v2488 { (v2522 * ((if self.scalar_v2488 { (v2516 * (v2275 * v6897)) } else { v4 }) + v9473)) } else { v4 }))));
        let v10052: f64 = (self.scalar_v27 * (v9586 * (self.scalar_v0 * (if self.scalar_v2488 { (v2522 * ((if self.scalar_v2488 { (v2516 * (v2275 * v6898)) } else { v4 }) + v9473)) } else { v4 }))));
        let v10053: f64 = (self.scalar_v27 * (v9586 * (self.scalar_v0 * (if self.scalar_v2488 { (v2522 * (if self.scalar_v2488 { (v8948 / v2382) } else { v4 })) } else { v4 }))));
        let v10058: f64 = (self.scalar_v27 * (v9586 * self.scalar_v10054));
        let v10059: f64 = (self.scalar_v27 * (v9586 * self.scalar_v10055));
        let v10064: f64 = (self.scalar_v27 * (v9586 * self.scalar_v10060));
        let v10065: f64 = (self.scalar_v27 * (v9586 * self.scalar_v10061));
        let v10117: f64 = (v9586 * (self.scalar_v0 * (v8815 + (if self.scalar_v2436 { (v9318 + (v2484 * (if self.scalar_v1841 { v4 } else { (if self.scalar_v1805 { (((v1837 * v6423) - v6479) / v6453) } else { v4 }) }))) } else { v4 }))));
        let v10120: f64 = (self.scalar_v27 * (v9586 * (self.scalar_v0 * v9759)));
        let v10121: f64 = (self.scalar_v27 * (v9586 * (self.scalar_v0 * ((self.scalar_v13 * v8808) + (if self.scalar_v2436 { v9309 } else { v4 })))));
        let v10122: f64 = (self.scalar_v27 * (v9586 * (self.scalar_v0 * (if self.scalar_v2436 { (v2484 * (if self.scalar_v1841 { v4 } else { (if self.scalar_v1805 { ((-(v1833 * v6434)) / v6453) } else { v4 }) })) } else { v4 }))));
        let v10123: f64 = (self.scalar_v27 * (v9586 * (self.scalar_v0 * v9761)));
        let v10124: f64 = (self.scalar_v27 * (v9586 * (self.scalar_v0 * (if self.scalar_v2436 { (v2484 * (if self.scalar_v1841 { v4 } else { (if self.scalar_v1805 { (((v1837 * v6420) - (v1833 * v6420)) / v6453) } else { v4 }) })) } else { v4 }))));
        let v10125: f64 = (self.scalar_v27 * (v9586 * (self.scalar_v0 * v9762)));
        let v10126: f64 = (self.scalar_v27 * (v9586 * (self.scalar_v0 * v9763)));
        let v10127: f64 = (self.scalar_v27 * v10117);
        let v10128: f64 = (self.scalar_v27 * (v9586 * (self.scalar_v0 * (v8816 + (if self.scalar_v2436 { v9324 } else { v4 })))));
        let v10129: f64 = (self.scalar_v27 * (v9586 * (self.scalar_v0 * v9766)));
        let v10165: f64 = (self.scalar_v27 * (v9586 * (self.scalar_v0 * v9754)));
        let v10166: f64 = (self.scalar_v27 * (v9586 * (self.scalar_v0 * (v8673 + (if self.scalar_v2436 { (self.scalar_v14 * v9093) } else { v9093 })))));
        let v10167: f64 = (self.scalar_v27 * (v9586 * (self.scalar_v0 * (v8674 + (if self.scalar_v2436 { (self.scalar_v14 * v9094) } else { v9094 })))));
        let v10168: f64 = (self.scalar_v27 * (v9586 * (self.scalar_v0 * (v8675 + (if self.scalar_v2436 { (self.scalar_v14 * v9095) } else { v9095 })))));
        let v10169: f64 = (self.scalar_v27 * (v9586 * (self.scalar_v0 * (v8676 + (if self.scalar_v2436 { (self.scalar_v14 * v9096) } else { v9096 })))));
        let v10188: f64 = (v2700 * (if self.scalar_v2609 { v4 } else { (if self.scalar_v2604 { (self.scalar_v2605 * v9684) } else { (if self.scalar_v2599 { (self.scalar_v2534 * v9684) } else { v4 }) }) }));
        let v10189: f64 = (v2700 * (if self.scalar_v2609 { v4 } else { (if self.scalar_v2604 { (self.scalar_v2605 * v9685) } else { (if self.scalar_v2599 { (self.scalar_v2534 * v9685) } else { v4 }) }) }));
        let v10190: f64 = (v2700 * (if self.scalar_v2609 { v4 } else { (if self.scalar_v2604 { (self.scalar_v2605 * v9686) } else { (if self.scalar_v2599 { (self.scalar_v2534 * v9686) } else { v4 }) }) }));
        let v10191: f64 = (v2700 * (if self.scalar_v2609 { v4 } else { (if self.scalar_v2604 { (self.scalar_v2605 * v9687) } else { (if self.scalar_v2599 { (self.scalar_v2534 * v9687) } else { v4 }) }) }));
        let v10192: f64 = (v2700 * (if self.scalar_v2609 { v4 } else { (if self.scalar_v2604 { (self.scalar_v2605 * v9688) } else { (if self.scalar_v2599 { (self.scalar_v2534 * v9688) } else { v4 }) }) }));
        let v10193: f64 = (v2700 * (if self.scalar_v2609 { v4 } else { (if self.scalar_v2604 { (self.scalar_v2605 * v9689) } else { (if self.scalar_v2599 { (self.scalar_v2534 * v9689) } else { v4 }) }) }));
        let v10194: f64 = (v2700 * (if self.scalar_v2609 { v4 } else { (if self.scalar_v2604 { (self.scalar_v2605 * v9690) } else { (if self.scalar_v2599 { (self.scalar_v2534 * v9690) } else { v4 }) }) }));
        let v10195: f64 = (v2610 * v9586);

        let d2548_dn4: f64 = v9588;
        stamper.stamp_current_reactive_node1(
            Some(nodes[4]),
            None,
            nodes[4],
            multiplicity * (d2548_dn4),
        );
        let d2656_dn4: f64 = v9987;
        let d2656_dn5: f64 = v9988;
        let d2656_dn6: f64 = v9989;
        let d2656_dn7: f64 = v9990;
        let d2656_dn8: f64 = v9991;
        let d2656_dn9: f64 = v9992;
        let d2656_dn11: f64 = v9993;
        let v2656_reactive_nodes: [usize; 7] = [nodes[4], nodes[5], nodes[6], nodes[7], nodes[8], nodes[9], nodes[11]];
        let v2656_reactive_node_derivatives: [f64; 7] = [d2656_dn4, d2656_dn5, d2656_dn6, d2656_dn7, d2656_dn8, d2656_dn9, d2656_dn11];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[5]),
            &v2656_reactive_nodes,
            &v2656_reactive_node_derivatives,
            &[],
            &[],
            multiplicity,
        );
        let d2659_dn4: f64 = v10000;
        let d2659_dn5: f64 = v10001;
        let d2659_dn6: f64 = v10002;
        stamper.stamp_current_reactive_node3(
            Some(nodes[6]),
            Some(nodes[5]),
            nodes[4],
            multiplicity * (d2659_dn4),
            nodes[5],
            multiplicity * (d2659_dn5),
            nodes[6],
            multiplicity * (d2659_dn6),
        );
        let d2662_dn4: f64 = v10017;
        let d2662_dn5: f64 = v10018;
        let d2662_dn6: f64 = v10019;
        let d2662_dn7: f64 = v10020;
        let d2662_dn8: f64 = v10021;
        let d2662_dn9: f64 = v10022;
        let d2662_dn11: f64 = v10023;
        let v2662_reactive_nodes: [usize; 7] = [nodes[4], nodes[5], nodes[6], nodes[7], nodes[8], nodes[9], nodes[11]];
        let v2662_reactive_node_derivatives: [f64; 7] = [d2662_dn4, d2662_dn5, d2662_dn6, d2662_dn7, d2662_dn8, d2662_dn9, d2662_dn11];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[9]),
            &v2662_reactive_nodes,
            &v2662_reactive_node_derivatives,
            &[],
            &[],
            multiplicity,
        );
        let d2665_dn3: f64 = v10030;
        let d2665_dn4: f64 = v10031;
        let d2665_dn8: f64 = v10032;
        stamper.stamp_current_reactive_node3(
            Some(nodes[3]),
            Some(nodes[8]),
            nodes[3],
            multiplicity * (d2665_dn3),
            nodes[4],
            multiplicity * (d2665_dn4),
            nodes[8],
            multiplicity * (d2665_dn8),
        );
        let d2668_dn4: f64 = v10047;
        let d2668_dn5: f64 = v10048;
        let d2668_dn6: f64 = v10049;
        let d2668_dn7: f64 = v10050;
        let d2668_dn8: f64 = v10051;
        let d2668_dn9: f64 = v10052;
        let d2668_dn11: f64 = v10053;
        let v2668_reactive_nodes: [usize; 7] = [nodes[4], nodes[5], nodes[6], nodes[7], nodes[8], nodes[9], nodes[11]];
        let v2668_reactive_node_derivatives: [f64; 7] = [d2668_dn4, d2668_dn5, d2668_dn6, d2668_dn7, d2668_dn8, d2668_dn9, d2668_dn11];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[7]),
            &v2668_reactive_nodes,
            &v2668_reactive_node_derivatives,
            &[],
            &[],
            multiplicity,
        );
        let d2672_dn1: f64 = v10058;
        let d2672_dn2: f64 = v10059;
        stamper.stamp_current_reactive_node2(
            Some(nodes[1]),
            Some(nodes[2]),
            nodes[1],
            multiplicity * (d2672_dn1),
            nodes[2],
            multiplicity * (d2672_dn2),
        );
        let d2676_dn0: f64 = v10064;
        let d2676_dn1: f64 = v10065;
        stamper.stamp_current_reactive_node2(
            Some(nodes[1]),
            Some(nodes[0]),
            nodes[0],
            multiplicity * (d2676_dn0),
            nodes[1],
            multiplicity * (d2676_dn1),
        );
        let d2684_dn0: f64 = v10120;
        let d2684_dn1: f64 = v10121;
        let d2684_dn3: f64 = v10122;
        let d2684_dn4: f64 = v10123;
        let d2684_dn5: f64 = v10124;
        let d2684_dn6: f64 = v10120;
        let d2684_dn7: f64 = v10125;
        let d2684_dn8: f64 = v10126;
        let d2684_dn9: f64 = v10127;
        let d2684_dn10: f64 = v10128;
        let d2684_dn11: f64 = v10129;
        let v2684_reactive_nodes: [usize; 11] = [nodes[0], nodes[1], nodes[3], nodes[4], nodes[5], nodes[6], nodes[7], nodes[8], nodes[9], nodes[10], nodes[11]];
        let v2684_reactive_node_derivatives: [f64; 11] = [d2684_dn0, d2684_dn1, d2684_dn3, d2684_dn4, d2684_dn5, d2684_dn6, d2684_dn7, d2684_dn8, d2684_dn9, d2684_dn10, d2684_dn11];
        stamper.stamp_current_reactive_dense(
            Some(nodes[1]),
            Some(nodes[10]),
            &v2684_reactive_nodes,
            &v2684_reactive_node_derivatives,
            &[],
            &[],
            multiplicity,
        );
        let d2690_dn4: f64 = v10165;
        let d2690_dn6: f64 = v10166;
        let d2690_dn7: f64 = v10167;
        let d2690_dn8: f64 = v10168;
        let d2690_dn9: f64 = v10168;
        let d2690_dn11: f64 = v10169;
        let v2690_reactive_nodes: [usize; 6] = [nodes[4], nodes[6], nodes[7], nodes[8], nodes[9], nodes[11]];
        let v2690_reactive_node_derivatives: [f64; 6] = [d2690_dn4, d2690_dn6, d2690_dn7, d2690_dn8, d2690_dn9, d2690_dn11];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[11]),
            &v2690_reactive_nodes,
            &v2690_reactive_node_derivatives,
            &[],
            &[],
            multiplicity,
        );
        let d2701_dn4: f64 = v10188;
        let d2701_dn5: f64 = v10189;
        let d2701_dn6: f64 = v10190;
        let d2701_dn7: f64 = v10191;
        let d2701_dn8: f64 = v10192;
        let d2701_dn9: f64 = v10193;
        let d2701_dn11: f64 = v10194;
        let d2701_dn12: f64 = v10195;
        let v2701_reactive_nodes: [usize; 8] = [nodes[4], nodes[5], nodes[6], nodes[7], nodes[8], nodes[9], nodes[11], nodes[12]];
        let v2701_reactive_node_derivatives: [f64; 8] = [d2701_dn4, d2701_dn5, d2701_dn6, d2701_dn7, d2701_dn8, d2701_dn9, d2701_dn11, d2701_dn12];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[5]),
            &v2701_reactive_nodes,
            &v2701_reactive_node_derivatives,
            &[],
            &[],
            multiplicity,
        );
    }
}
