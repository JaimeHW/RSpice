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
        let branches = self.branches;
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
        let v0: f64 = ctx.node_voltage(nodes[8]);
        let v1: f64 = ctx.node_voltage(nodes[5]);
        let v2: f64 = (v0 - v1);
        let v3: f64 = ctx.node_voltage(nodes[4]);
        let v4: f64 = ctx.node_voltage(nodes[3]);
        let v5: f64 = (v3 - v4);
        let v6: f64 = (-v5);
        let v7: f64 = (v4 - v1);
        let v8: f64 = ctx.node_voltage(nodes[7]);
        let v9: f64 = (v8 - v4);
        let v10: f64 = ctx.node_voltage(nodes[13]);
        let v11: f64 = 0.0;
        let v30: f64 = ctx.node_voltage(nodes[11]);
        let v33: f64 = (if (self.scalar_v29 != 0.0) { (self.scalar_v21 + ((v30) as f64).abs()) } else { self.scalar_v21 });
        let v37: f64 = (((v33 - self.scalar_v28)) as f64).abs();
        let v41: bool = ((v37 > v11) || self.scalar_v40);
        let v42: f64 = 1.0;
        let v112: bool = (!v41);
        let v113: f64 = (if v112 { self.scalar_v43 } else { (if v41 { (self.scalar_v43 * (v42 + (v37 * self.scalar_v44))) } else { v11 }) });
        let v114: f64 = (if v112 { self.scalar_v49 } else { (if v41 { (self.scalar_v49 * (v42 + (v37 * self.scalar_v50))) } else { v11 }) });
        let v115: f64 = (if v112 { self.scalar_v55 } else { (if v41 { (self.scalar_v55 * (v42 + (v37 * self.scalar_v56))) } else { v11 }) });
        let v116: f64 = (if v112 { self.scalar_v61 } else { (if v41 { (self.scalar_v61 * (v42 + (v37 * self.scalar_v62))) } else { v11 }) });
        let v117: f64 = (if v112 { self.scalar_v67 } else { (if v41 { (self.scalar_v67 * (v42 + (v37 * self.scalar_v68))) } else { v11 }) });
        let v118: f64 = (if v112 { self.scalar_v73 } else { (if v41 { (self.scalar_v73 * (v42 + (v37 * self.scalar_v74))) } else { v11 }) });
        let v119: f64 = (if v112 { self.scalar_v79 } else { (if v41 { (self.scalar_v79 * (v42 + (v37 * self.scalar_v80))) } else { v11 }) });
        let v121: f64 = (if v112 { self.scalar_v90 } else { (if v41 { (self.scalar_v90 + (v37 * self.scalar_v92)) } else { v11 }) });
        let v122: f64 = (if v112 { self.scalar_v96 } else { (if v41 { (self.scalar_v96 + (v37 * self.scalar_v98)) } else { v11 }) });
        let v123: f64 = (if v112 { self.scalar_v102 } else { (if v41 { (self.scalar_v102 + (v37 * self.scalar_v103)) } else { v11 }) });
        let v124: f64 = (if v112 { self.scalar_v107 } else { (if v41 { (self.scalar_v107 + (v37 * self.scalar_v108)) } else { v11 }) });
        let v129: f64 = 0.5;
        let v136: f64 = (if self.scalar_v134 { self.scalar_v135 } else { (if self.scalar_v128 { (self.scalar_v131 / (v33 * 8.617333262145179e-5)) } else { v11 }) });
        let v138: f64 = (v7 * self.scalar_v137);
        let v139: f64 = ((v138) as f64).cosh();
        let v141: f64 = (v139 * v139);
        let v144: f64 = (v114 * (v42 + (self.scalar_v140 / v141)));
        let v149: f64 = (((v7 * self.scalar_v147)) as f64).tanh();
        let v154: f64 = (self.scalar_v152 * (v6 - self.scalar_v107));
        let v155: f64 = (v6 - v124);
        let v157: f64 = ((((if v112 { self.scalar_v85 } else { (if v41 { (self.scalar_v85 + (v37 * self.scalar_v86)) } else { v11 }) }) - self.scalar_v145) + (self.scalar_v145 * v149)) - (v154 * v155));
        let v158: f64 = (v2 - v157);
        let v159: f64 = (v158 * v158);
        let v165: f64 = (v158 * self.scalar_v164);
        let v167: f64 = (((v144 * v158) + (v159 * self.scalar_v161)) + (v159 * v165));
        let v168: f64 = ((v167) as f64).tanh();
        let v169: f64 = (v42 + v168);
        let v171: f64 = (-v167);
        let v175: f64 = (((v129 * ({ let limexp_arg = v167; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } } - { let limexp_arg = v171; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } }))) as f64).tanh();
        let v179: f64 = (self.scalar_v177 + (self.scalar_v147 * v169));
        let v181: f64 = (((v7 * v179)) as f64).tanh();
        let v189: f64 = (v113 * v169);
        let v190: f64 = (v181 * v189);
        let v195: f64 = (v115 * { let limexp_arg = v155; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } });
        let v196: f64 = ((v42 + (v7 * self.scalar_v191)) + v195);
        let v201: f64 = (v5 - v157);
        let v202: f64 = (if self.scalar_v200 { v201 } else { v139 });
        let v204: f64 = (if self.scalar_v200 { (v202 * v202) } else { v158 });
        let v206: f64 = (if self.scalar_v200 { (v202 * v204) } else { v159 });
        let v212: f64 = (if self.scalar_v200 { (((v144 * v202) + (self.scalar_v161 * v204)) + (self.scalar_v164 * v206)) } else { v11 });
        let v213: f64 = ((v212) as f64).tanh();
        let v215: f64 = (if self.scalar_v200 { (v42 + v213) } else { v11 });
        let v218: f64 = (if self.scalar_v200 { (self.scalar_v177 + (self.scalar_v147 * v215)) } else { v11 });
        let v222: f64 = (if self.scalar_v200 { (self.scalar_v191 + (v169 * self.scalar_v219)) } else { v11 });
        let v223: f64 = (v42 + v181);
        let v224: f64 = (v189 * v223);
        let v227: f64 = (v7 - v124);
        let v229: f64 = (v115 * { let limexp_arg = v227; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } });
        let v230: f64 = ((v42 + (v7 * v222)) + v229);
        let v232: f64 = (if self.scalar_v200 { (v224 * v230) } else { v11 });
        let v235: f64 = (if self.scalar_v200 { (self.scalar_v191 + (v215 * self.scalar_v219)) } else { v11 });
        let v237: f64 = (((v7 * v218)) as f64).tanh();
        let v239: f64 = (v113 * v215);
        let v240: f64 = (v42 - (if self.scalar_v200 { v237 } else { v11 }));
        let v241: f64 = (v239 * v240);
        let v243: f64 = (v42 - (v7 * v235));
        let v245: f64 = (if self.scalar_v200 { (v241 * v243) } else { v11 });
        let v252: f64 = (if self.scalar_v251 { v158 } else { v202 });
        let v254: f64 = (if self.scalar_v251 { (v252 * v252) } else { v204 });
        let v257: f64 = (self.scalar_v164 * v254);
        let v259: f64 = ((v252 + (self.scalar_v161 * v254)) + (v252 * v257));
        let v261: f64 = (if self.scalar_v251 { (v144 * v259) } else { v167 });
        let v263: f64 = (-v261);
        let v267: f64 = (((v129 * ({ let limexp_arg = v261; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } } - { let limexp_arg = v263; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } }))) as f64).tanh();
        let v269: f64 = (if self.scalar_v251 { (v42 + v267) } else { (v42 + v175) });
        let v272: f64 = (if self.scalar_v251 { (self.scalar_v177 + (self.scalar_v147 * v269)) } else { v11 });
        let v274: f64 = (((v7 * v272)) as f64).tanh();
        let v275: f64 = (if self.scalar_v251 { v274 } else { v11 });
        let v278: f64 = (if self.scalar_v251 { (self.scalar_v191 + (self.scalar_v219 * v269)) } else { v222 });
        let v279: f64 = (v113 * v269);
        let v280: f64 = (v275 * v279);
        let v283: f64 = (v195 + (v42 + (v7 * v278)));
        let v289: f64 = (if self.scalar_v288 { v158 } else { v252 });
        let v291: f64 = (if self.scalar_v288 { (v289 * v289) } else { v254 });
        let v294: f64 = (self.scalar_v164 * v291);
        let v296: f64 = ((v289 + (self.scalar_v161 * v291)) + (v289 * v294));
        let v298: f64 = (if self.scalar_v288 { (v144 * v296) } else { v261 });
        let v299: f64 = (if self.scalar_v288 { v201 } else { v206 });
        let v301: f64 = (if self.scalar_v288 { (v299 * v299) } else { v11 });
        let v304: f64 = (self.scalar_v164 * v299);
        let v306: f64 = ((v299 + (self.scalar_v161 * v301)) + (v301 * v304));
        let v308: f64 = (if self.scalar_v288 { (v144 * v306) } else { v212 });
        let v310: f64 = (-v298);
        let v314: f64 = (((v129 * ({ let limexp_arg = v298; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } } - { let limexp_arg = v310; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } }))) as f64).tanh();
        let v316: f64 = (if self.scalar_v288 { (v42 + v314) } else { v269 });
        let v318: f64 = (-v308);
        let v322: f64 = (((v129 * ({ let limexp_arg = v308; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } } - { let limexp_arg = v318; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } }))) as f64).tanh();
        let v324: f64 = (if self.scalar_v288 { (v42 + v322) } else { v11 });
        let v327: f64 = (if self.scalar_v288 { (self.scalar_v177 + (self.scalar_v147 * v316)) } else { v272 });
        let v330: f64 = (if self.scalar_v288 { (self.scalar_v177 + (self.scalar_v147 * v324)) } else { v11 });
        let v332: f64 = (((v7 * v327)) as f64).tanh();
        let v335: f64 = (((v7 * v330)) as f64).tanh();
        let v339: f64 = (if self.scalar_v288 { (self.scalar_v191 + (self.scalar_v219 * v324)) } else { v11 });
        let v342: f64 = (if self.scalar_v288 { (self.scalar_v191 + (self.scalar_v219 * v316)) } else { v11 });
        let v343: f64 = (v113 * v316);
        let v344: f64 = (v42 + (if self.scalar_v288 { v332 } else { v275 }));
        let v345: f64 = (v343 * v344);
        let v348: f64 = (v229 + (v42 + (v7 * v342)));
        let v351: f64 = (v113 * v324);
        let v352: f64 = (v42 - (if self.scalar_v288 { v335 } else { v11 }));
        let v353: f64 = (v351 * v352);
        let v355: f64 = (v42 - (v7 * v339));
        let v360: f64 = (if self.scalar_v288 { (v129 * ((if self.scalar_v288 { (v345 * v348) } else { v232 }) - (if self.scalar_v288 { (v353 * v355) } else { v245 }))) } else { (if self.scalar_v251 { (v280 * v283) } else { (if self.scalar_v200 { (v129 * (v232 - v245)) } else { (if self.scalar_v183 { (v190 * v196) } else { v11 }) }) }) });
        let v362: f64 = (v42 + v169);
        let v368: f64 = (v169 * self.scalar_v367);
        let v374: f64 = (v42 + v316);
        let v377: f64 = (if self.scalar_v250 { (self.scalar_v361 + (v118 / v374)) } else { (if self.scalar_v249 { (self.scalar_v361 + (v118 / v362)) } else { v11 }) });
        let v378: f64 = (v316 * self.scalar_v367);
        let v380: f64 = (if self.scalar_v250 { (self.scalar_v366 + v378) } else { (if self.scalar_v249 { (self.scalar_v366 + v368) } else { v11 }) });
        let v382: f64 = (if self.scalar_v250 { (self.scalar_v371 + v378) } else { (if self.scalar_v249 { (v368 + self.scalar_v371) } else { v11 }) });
        let v383: bool = ((v37 != 0.0) || self.scalar_v40);
        let v386: f64 = (v42 + (v37 * self.scalar_v384));
        let v391: bool = (!v383);
        let v392: f64 = (if v391 { v380 } else { (if v383 { (v380 * v386) } else { v11 }) });
        let v393: f64 = (if v391 { v382 } else { (if v383 { (v382 * v386) } else { v11 }) });
        let v396: f64 = -1.0;
        let v402: f64 = (v2 - v123);
        let v404: f64 = (v9 - v123);
        let v410: f64 = (if self.scalar_v406 { { let limexp_arg = (v123 * (-v136)); if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } } } else { (if self.scalar_v395 { { let limexp_arg = (v136 * (((-v123)) as f64).tanh()); if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } } } else { v289 }) });
        let v413: f64 = ((v402) as f64).tanh();
        let v415: f64 = ((v404) as f64).tanh();
        let v422: f64 = (v136 * (if self.scalar_v418 { v402 } else { (if self.scalar_v412 { v413 } else { (if self.scalar_v395 { v402 } else { v11 }) }) }));
        let v425: f64 = (self.scalar_v421 * ({ let limexp_arg = v422; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } } - v410));
        let v426: f64 = (v136 * (if self.scalar_v418 { v404 } else { (if self.scalar_v412 { v415 } else { (if self.scalar_v395 { v404 } else { v11 }) }) }));
        let v429: f64 = (self.scalar_v421 * ({ let limexp_arg = v426; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } } - v410));
        let v433: f64 = (v7 * self.scalar_v432);
        let v434: f64 = ((v121 + (v2 * self.scalar_v91)) + v433);
        let v435: f64 = ((v434) as f64).tanh();
        let v441: f64 = (((self.scalar_v437 + (v7 * self.scalar_v438))) as f64).tanh();
        let v442: f64 = (v42 + v441);
        let v447: f64 = (((self.scalar_v443 - (v7 * self.scalar_v444))) as f64).tanh();
        let v449: f64 = ((v42 + v447) - self.scalar_v432);
        let v452: f64 = ((v122 + (v9 * self.scalar_v97)) - v433);
        let v453: f64 = ((v452) as f64).tanh();
        let v454: f64 = (v42 + v453);
        let v465: f64 = (v116 * (v42 + v435));
        let v479: f64 = (if self.scalar_v477 { (v442 - self.scalar_v432) } else { v442 });
        let v480: f64 = (v121 + v433);
        let v482: f64 = (if self.scalar_v477 { ((v480) as f64).cosh() } else { v11 });
        let v486: f64 = (if self.scalar_v477 { ((v434) as f64).cosh() } else { v11 });
        let v492: f64 = ((v434 + (if self.scalar_v477 { ((v486) as f64).ln() } else { v11 })) - (if self.scalar_v477 { (v480 + (if self.scalar_v477 { ((v482) as f64).ln() } else { v11 })) } else { v11 }));
        let v501: f64 = (v122 - v433);
        let v503: f64 = (if self.scalar_v477 { ((v501) as f64).cosh() } else { v482 });
        let v507: f64 = (if self.scalar_v477 { ((v452) as f64).cosh() } else { v486 });
        let v513: f64 = ((v452 + (if self.scalar_v477 { ((v507) as f64).ln() } else { v11 })) - (if self.scalar_v477 { (v501 + (if self.scalar_v477 { ((v503) as f64).ln() } else { v11 })) } else { v11 }));
        let v1716: f64 = ((v434) as f64).sinh();
        let v1722: f64 = (if self.scalar_v477 { (self.scalar_v91 * v1716) } else { v11 });
        let v1759: f64 = (if self.scalar_v477 { (self.scalar_v459 + (v116 * (self.scalar_v470 + ((v479 * (self.scalar_v91 + (if self.scalar_v477 { (v1722 / v486) } else { v11 }))) / self.scalar_v91)))) } else { v11 });
        let v522: f64 = v1759;
        let v523: f64 = (if self.scalar_v477 { v522 } else { (if self.scalar_v464 { (self.scalar_v459 + (v442 * v465)) } else { self.scalar_v460 }) });
        let v1769: f64 = ((v452) as f64).sinh();
        let v1817: f64 = (if self.scalar_v477 { (self.scalar_v461 + (v117 * (self.scalar_v470 + ((v449 * (self.scalar_v97 + (if self.scalar_v477 { ((if self.scalar_v477 { (self.scalar_v97 * v1769) } else { v11 }) / v507) } else { v11 }))) / self.scalar_v97)))) } else { v11 });
        let v524: f64 = v1817;
        let v525: f64 = (if self.scalar_v477 { v524 } else { (if self.scalar_v464 { (self.scalar_v461 + (v117 * ((v449 * v454) + self.scalar_v470))) } else { self.scalar_v462 }) });
        let v560: f64 = (if self.scalar_v549 { ((v116 * ((v33 * 5.5226012e-23) * self.scalar_v554)) * self.scalar_v558) } else { v11 });
        let v564: f64 = (if self.scalar_v549 { (((v42 - (v560 * v560))) as f64).sqrt() } else { v11 });
        let v566: f64 = 3.141592653589793;
        let v568: f64 = (if self.scalar_v549 { ((-v560) * v566) } else { v11 });
        let v572: f64 = (-v360);
        let v575: f64 = (self.scalar_v573 * ctx.node_voltage(nodes[12]));
        let v578: f64 = (self.scalar_v576 * ctx.branch_current(branches[0]));
        let v579: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, (if self.scalar_v477 { ((v117 * (((v449 * v513) / self.scalar_v97) + (v9 * self.scalar_v470))) + (v9 * self.scalar_v461)) } else { v11 }));
        let v580: f64 = (if self.scalar_v458 { v579 } else { v11 });
        let v581: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, (if self.scalar_v477 { ((v116 * (((v479 * v492) / self.scalar_v91) + (v2 * self.scalar_v470))) + (v2 * self.scalar_v459)) } else { v11 }));
        let v582: f64 = (if self.scalar_v458 { v581 } else { v11 });
        let v585: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, (v9 * v525));
        let v586: f64 = (if self.scalar_v583 { v585 } else { v11 });
        let v588: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 5, (v2 * v523));
        let v589: f64 = (if self.scalar_v583 { v588 } else { v11 });
        let v593: f64 = (self.scalar_v590 * (ctx.node_voltage(nodes[1]) - v4));
        let v595: f64 = (v7 * self.scalar_v594);
        let v596: f64 = ctx.node_voltage(nodes[10]);
        let v598: f64 = (v119 * (v4 - v596));
        let v599: f64 = (v596 - v1);
        let v601: f64 = (if self.scalar_v526 { (v599 / v377) } else { v11 });
        let v603: f64 = ctx.node_voltage(nodes[9]);
        let v605: f64 = (self.scalar_v602 * (v603 - v0));
        let v608: f64 = (if self.scalar_v528 { ((v603 - v1) / self.scalar_v527) } else { v11 });
        let v611: f64 = (if self.scalar_v530 { ((v3 - v8) / self.scalar_v529) } else { v11 });
        let v614: f64 = (if self.scalar_v532 { ((v3 - v0) / self.scalar_v531) } else { v11 });
        let v617: f64 = (if self.scalar_v534 { (self.scalar_v533 * ctx.branch_current(branches[5])) } else { v11 });
        let v620: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 10, (self.scalar_v535 * ctx.branch_current(branches[6])));
        let v621: f64 = (if self.scalar_v534 { v620 } else { v11 });
        let v626: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 11, (self.scalar_v535 * ctx.branch_current(branches[8])));
        let v627: f64 = (if self.scalar_v623 { v626 } else { v11 });
        let v628: f64 = ctx.branch_current(branches[10]);
        let v630: f64 = (if self.scalar_v537 { (v393 * v628) } else { v11 });
        let v633: f64 = (self.scalar_v631 * ctx.branch_current(branches[13]));
        let v634: f64 = ctx.branch_current(branches[14]);
        let v636: f64 = (if self.scalar_v540 { (v392 * v634) } else { v11 });
        let v639: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 13, (self.scalar_v541 * ctx.branch_current(branches[15])));
        let v640: f64 = (if self.scalar_v540 { v639 } else { v11 });
        let v645: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 14, (self.scalar_v541 * ctx.branch_current(branches[17])));
        let v646: f64 = (if self.scalar_v642 { v645 } else { v11 });
        let v647: f64 = ctx.node_voltage(nodes[14]);
        let v648: f64 = (if self.scalar_v549 { v647 } else { v11 });
        let v649: f64 = ctx.node_voltage(nodes[15]);
        let v650: f64 = (if self.scalar_v549 { v649 } else { v11 });
        let v654: f64 = (if self.scalar_v549 { ((v568 * v647) + (v564 * v649)) } else { v11 });
        let v655: f64 = (-(if self.scalar_v549 { (v560 * v566) } else { v11 }));
        let v657: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 15, (v647 * v655));
        let v658: f64 = (if self.scalar_v549 { v657 } else { v11 });
        let v661: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 16, (v30 * self.scalar_v659));
        let v662: f64 = (if self.scalar_v571 { v661 } else { v11 });
        let v669: f64 = (if self.scalar_v571 { (-((((v7 * (-v10)) + (v2 * v425))) as f64).abs()) } else { v11 });
        let v671: f64 = (if self.scalar_v571 { (v30 / self.scalar_v39) } else { v11 });
        let v675: f64 = (if self.scalar_v672 { (v30 * 1e-12) } else { v11 });
        let v677: f64 = ((v138) as f64).sinh();
        let v678: f64 = (self.scalar_v137 * v677);
        let v679: f64 = (self.scalar_v676 * v677);
        let v680: f64 = (v139 * v678);
        let v682: f64 = (v139 * v679);
        let v686: f64 = (v141 * v141);
        let v691: f64 = (v114 * ((-(self.scalar_v140 * (v680 + v680))) / v686));
        let v692: f64 = (v114 * ((-(self.scalar_v140 * (v682 + v682))) / v686));
        let v695: f64 = (v42 - (v149 * v149));
        let v699: f64 = (self.scalar_v145 * (self.scalar_v693 * v695));
        let v705: f64 = ((v155 * self.scalar_v700) + (-v154));
        let v706: f64 = ((self.scalar_v145 * (self.scalar_v147 * v695)) - (v154 + (self.scalar_v152 * v155)));
        let v708: f64 = (-v706);
        let v709: f64 = (v396 - v699);
        let v710: f64 = (v158 * v708);
        let v711: f64 = (v710 + v710);
        let v712: f64 = (v158 * v705);
        let v713: f64 = (v712 + v712);
        let v714: f64 = (v158 * v709);
        let v715: f64 = (v714 + v714);
        let v716: f64 = (v158 + v158);
        let v747: f64 = ((((v158 * v691) + (v144 * v708)) + (self.scalar_v161 * v711)) + ((v165 * v711) + (v159 * (self.scalar_v164 * v708))));
        let v748: f64 = (((v144 * v705) + (self.scalar_v161 * v713)) + ((v165 * v713) + (v159 * (self.scalar_v164 * v705))));
        let v749: f64 = ((((v158 * v692) + (v144 * v709)) + (self.scalar_v161 * v715)) + ((v165 * v715) + (v159 * (self.scalar_v164 * v709))));
        let v750: f64 = ((v144 + (self.scalar_v161 * v716)) + ((v165 * v716) + (v159 * self.scalar_v164)));
        let v752: f64 = (v42 - (v168 * v168));
        let v753: f64 = (v747 * v752);
        let v754: f64 = (v748 * v752);
        let v755: f64 = (v749 * v752);
        let v756: f64 = (v750 * v752);
        let v757: f64 = { let limexp_arg = v167; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v766: f64 = { let limexp_arg = v171; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v780: f64 = (v42 - (v175 * v175));
        let v797: f64 = (v42 - (v181 * v181));
        let v802: f64 = (v113 * v753);
        let v803: f64 = (v113 * v754);
        let v804: f64 = (v113 * v755);
        let v805: f64 = (v113 * v756);
        let v806: f64 = (v189 * ((v179 + (v7 * (self.scalar_v147 * v753))) * v797));
        let v809: f64 = (v189 * ((v7 * (self.scalar_v147 * v754)) * v797));
        let v812: f64 = (v189 * (((-v179) + (v7 * (self.scalar_v147 * v755))) * v797));
        let v815: f64 = (v189 * ((v7 * (self.scalar_v147 * v756)) * v797));
        let v819: f64 = { let limexp_arg = v155; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v821: f64 = (v115 * v819);
        let v822: f64 = (v115 * (-v819));
        let v838: f64 = (v396 - v706);
        let v839: f64 = (v42 - (-v705));
        let v840: f64 = (-v699);
        let v841: f64 = (if self.scalar_v200 { v838 } else { v678 });
        let v842: f64 = (if self.scalar_v200 { v839 } else { v11 });
        let v843: f64 = (if self.scalar_v200 { v840 } else { v679 });
        let v844: f64 = (v202 * v841);
        let v846: f64 = (v202 * v842);
        let v848: f64 = (v202 * v843);
        let v850: f64 = (if self.scalar_v200 { (v844 + v844) } else { v708 });
        let v851: f64 = (if self.scalar_v200 { (v846 + v846) } else { v705 });
        let v852: f64 = (if self.scalar_v200 { (v848 + v848) } else { v709 });
        let v864: f64 = (if self.scalar_v200 { ((v204 * v841) + (v202 * v850)) } else { v711 });
        let v865: f64 = (if self.scalar_v200 { ((v204 * v842) + (v202 * v851)) } else { v713 });
        let v866: f64 = (if self.scalar_v200 { ((v204 * v843) + (v202 * v852)) } else { v715 });
        let v867: f64 = (if self.scalar_v200 { (v202 * self.scalar_v853) } else { v716 });
        let v890: f64 = (if self.scalar_v200 { ((((v202 * v691) + (v144 * v841)) + (self.scalar_v161 * v850)) + (self.scalar_v164 * v864)) } else { v11 });
        let v891: f64 = (if self.scalar_v200 { (((v144 * v842) + (self.scalar_v161 * v851)) + (self.scalar_v164 * v865)) } else { v11 });
        let v892: f64 = (if self.scalar_v200 { ((((v202 * v692) + (v144 * v843)) + (self.scalar_v161 * v852)) + (self.scalar_v164 * v866)) } else { v11 });
        let v893: f64 = (if self.scalar_v200 { (self.scalar_v878 + (self.scalar_v164 * v867)) } else { v11 });
        let v895: f64 = (v42 - (v213 * v213));
        let v900: f64 = (if self.scalar_v200 { (v890 * v895) } else { v11 });
        let v901: f64 = (if self.scalar_v200 { (v891 * v895) } else { v11 });
        let v902: f64 = (if self.scalar_v200 { (v892 * v895) } else { v11 });
        let v903: f64 = (if self.scalar_v200 { (v893 * v895) } else { v11 });
        let v916: f64 = (if self.scalar_v200 { (self.scalar_v219 * v753) } else { v11 });
        let v917: f64 = (if self.scalar_v200 { (self.scalar_v219 * v754) } else { v11 });
        let v918: f64 = (if self.scalar_v200 { (self.scalar_v219 * v755) } else { v11 });
        let v919: f64 = (if self.scalar_v200 { (self.scalar_v219 * v756) } else { v11 });
        let v935: f64 = { let limexp_arg = v227; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v937: f64 = (v115 * v935);
        let v938: f64 = (v115 * (-v935));
        let v953: f64 = (if self.scalar_v200 { ((v230 * (v806 + (v223 * v802))) + (v224 * ((v222 + (v7 * v916)) + v937))) } else { v11 });
        let v954: f64 = (if self.scalar_v200 { ((v230 * (v809 + (v223 * v803))) + (v224 * (v7 * v917))) } else { v11 });
        let v955: f64 = (if self.scalar_v200 { ((v230 * (v812 + (v223 * v804))) + (v224 * (((-v222) + (v7 * v918)) + v938))) } else { v11 });
        let v956: f64 = (if self.scalar_v200 { ((v230 * (v815 + (v223 * v805))) + (v224 * (v7 * v919))) } else { v11 });
        let v973: f64 = (v42 - (v237 * v237));
        let v998: f64 = ((v240 * (v113 * v902)) + (v239 * (-(if self.scalar_v200 { (((-v218) + (v7 * (if self.scalar_v200 { (self.scalar_v147 * v902) } else { v11 }))) * v973) } else { v11 }))));
        let v1013: f64 = (v243 * ((v240 * (v113 * v900)) + (v239 * (-(if self.scalar_v200 { ((v218 + (v7 * (if self.scalar_v200 { (self.scalar_v147 * v900) } else { v11 }))) * v973) } else { v11 })))));
        let v1018: f64 = ((v243 * ((v240 * (v113 * v901)) + (v239 * (-(if self.scalar_v200 { ((v7 * (if self.scalar_v200 { (self.scalar_v147 * v901) } else { v11 })) * v973) } else { v11 }))))) + (v241 * (-(v7 * (if self.scalar_v200 { (self.scalar_v219 * v901) } else { v11 })))));
        let v1024: f64 = ((v243 * ((v240 * (v113 * v903)) + (v239 * (-(if self.scalar_v200 { ((v7 * (if self.scalar_v200 { (self.scalar_v147 * v903) } else { v11 })) * v973) } else { v11 }))))) + (v241 * (-(v7 * (if self.scalar_v200 { (self.scalar_v219 * v903) } else { v11 })))));
        let v1025: f64 = (if self.scalar_v200 { (v1013 + (v241 * (-(v235 + (v7 * (if self.scalar_v200 { (self.scalar_v219 * v900) } else { v11 })))))) } else { v11 });
        let v1026: f64 = (if self.scalar_v200 { v1018 } else { v11 });
        let v1027: f64 = (if self.scalar_v200 { ((v243 * v998) + (v241 * (-((-v235) + (v7 * (if self.scalar_v200 { (self.scalar_v219 * v902) } else { v11 })))))) } else { v11 });
        let v1028: f64 = (if self.scalar_v200 { v1024 } else { v11 });
        let v1041: f64 = (if self.scalar_v251 { v708 } else { v841 });
        let v1042: f64 = (if self.scalar_v251 { v705 } else { v842 });
        let v1043: f64 = (if self.scalar_v251 { v709 } else { v843 });
        let v1045: f64 = (v252 * v1041);
        let v1047: f64 = (v252 * v1042);
        let v1049: f64 = (v252 * v1043);
        let v1051: f64 = (v252 * self.scalar_v1044);
        let v1053: f64 = (if self.scalar_v251 { (v1045 + v1045) } else { v850 });
        let v1054: f64 = (if self.scalar_v251 { (v1047 + v1047) } else { v851 });
        let v1055: f64 = (if self.scalar_v251 { (v1049 + v1049) } else { v852 });
        let v1056: f64 = (if self.scalar_v251 { (v1051 + v1051) } else { self.scalar_v853 });
        let v1093: f64 = (if self.scalar_v251 { ((v259 * v691) + (v144 * ((v1041 + (self.scalar_v161 * v1053)) + ((v257 * v1041) + (v252 * (self.scalar_v164 * v1053)))))) } else { v747 });
        let v1094: f64 = (if self.scalar_v251 { (v144 * ((v1042 + (self.scalar_v161 * v1054)) + ((v257 * v1042) + (v252 * (self.scalar_v164 * v1054))))) } else { v748 });
        let v1095: f64 = (if self.scalar_v251 { ((v259 * v692) + (v144 * ((v1043 + (self.scalar_v161 * v1055)) + ((v257 * v1043) + (v252 * (self.scalar_v164 * v1055)))))) } else { v749 });
        let v1096: f64 = (if self.scalar_v251 { (v144 * ((self.scalar_v1044 + (self.scalar_v161 * v1056)) + ((v257 * self.scalar_v1044) + (v252 * (self.scalar_v164 * v1056))))) } else { v750 });
        let v1097: f64 = { let limexp_arg = v261; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v1106: f64 = { let limexp_arg = v263; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v1120: f64 = (v42 - (v267 * v267));
        let v1125: f64 = (if self.scalar_v251 { ((v129 * ((v1093 * v1097) - ((-v1093) * v1106))) * v1120) } else { ((v129 * ((v747 * v757) - ((-v747) * v766))) * v780) });
        let v1126: f64 = (if self.scalar_v251 { ((v129 * ((v1094 * v1097) - ((-v1094) * v1106))) * v1120) } else { ((v129 * ((v748 * v757) - ((-v748) * v766))) * v780) });
        let v1127: f64 = (if self.scalar_v251 { ((v129 * ((v1095 * v1097) - ((-v1095) * v1106))) * v1120) } else { ((v129 * ((v749 * v757) - ((-v749) * v766))) * v780) });
        let v1128: f64 = (if self.scalar_v251 { ((v129 * ((v1096 * v1097) - ((-v1096) * v1106))) * v1120) } else { ((v129 * ((v750 * v757) - ((-v750) * v766))) * v780) });
        let v1133: f64 = (if self.scalar_v251 { (self.scalar_v147 * v1125) } else { v11 });
        let v1134: f64 = (if self.scalar_v251 { (self.scalar_v147 * v1126) } else { v11 });
        let v1135: f64 = (if self.scalar_v251 { (self.scalar_v147 * v1127) } else { v11 });
        let v1136: f64 = (if self.scalar_v251 { (self.scalar_v147 * v1128) } else { v11 });
        let v1145: f64 = (v42 - (v274 * v274));
        let v1150: f64 = (if self.scalar_v251 { ((v272 + (v7 * v1133)) * v1145) } else { v11 });
        let v1151: f64 = (if self.scalar_v251 { ((v7 * v1134) * v1145) } else { v11 });
        let v1152: f64 = (if self.scalar_v251 { (((-v272) + (v7 * v1135)) * v1145) } else { v11 });
        let v1153: f64 = (if self.scalar_v251 { ((v7 * v1136) * v1145) } else { v11 });
        let v1189: f64 = ((v283 * ((v279 * v1150) + (v275 * (v113 * v1125)))) + (v280 * (v821 + (v278 + (v7 * (if self.scalar_v251 { (self.scalar_v219 * v1125) } else { v916 }))))));
        let v1195: f64 = ((v283 * ((v279 * v1152) + (v275 * (v113 * v1127)))) + (v280 * ((-v278) + (v7 * (if self.scalar_v251 { (self.scalar_v219 * v1127) } else { v918 })))));
        let v1199: f64 = (if self.scalar_v251 { v1189 } else { (if self.scalar_v200 { (v129 * (v953 - v1025)) } else { (if self.scalar_v183 { ((v196 * (v806 + (v181 * v802))) + (v190 * (self.scalar_v191 + v821))) } else { v11 }) }) });
        let v1200: f64 = (if self.scalar_v251 { ((v283 * ((v279 * v1151) + (v275 * (v113 * v1126)))) + (v280 * (v822 + (v7 * (if self.scalar_v251 { (self.scalar_v219 * v1126) } else { v917 }))))) } else { (if self.scalar_v200 { (v129 * (v954 - v1026)) } else { (if self.scalar_v183 { ((v196 * (v809 + (v181 * v803))) + (v190 * v822)) } else { v11 }) }) });
        let v1202: f64 = (if self.scalar_v251 { ((v283 * ((v279 * v1153) + (v275 * (v113 * v1128)))) + (v280 * (v7 * (if self.scalar_v251 { (self.scalar_v219 * v1128) } else { v919 })))) } else { (if self.scalar_v200 { (v129 * (v956 - v1028)) } else { (if self.scalar_v183 { (v196 * (v815 + (v181 * v805))) } else { v11 }) }) });
        let v1203: f64 = (if self.scalar_v288 { v708 } else { v1041 });
        let v1204: f64 = (if self.scalar_v288 { v705 } else { v1042 });
        let v1205: f64 = (if self.scalar_v288 { v709 } else { v1043 });
        let v1207: f64 = (v289 * v1203);
        let v1209: f64 = (v289 * v1204);
        let v1211: f64 = (v289 * v1205);
        let v1213: f64 = (v289 * self.scalar_v1206);
        let v1215: f64 = (if self.scalar_v288 { (v1207 + v1207) } else { v1053 });
        let v1216: f64 = (if self.scalar_v288 { (v1209 + v1209) } else { v1054 });
        let v1217: f64 = (if self.scalar_v288 { (v1211 + v1211) } else { v1055 });
        let v1218: f64 = (if self.scalar_v288 { (v1213 + v1213) } else { v1056 });
        let v1255: f64 = (if self.scalar_v288 { ((v296 * v691) + (v144 * ((v1203 + (self.scalar_v161 * v1215)) + ((v294 * v1203) + (v289 * (self.scalar_v164 * v1215)))))) } else { v1093 });
        let v1256: f64 = (if self.scalar_v288 { (v144 * ((v1204 + (self.scalar_v161 * v1216)) + ((v294 * v1204) + (v289 * (self.scalar_v164 * v1216))))) } else { v1094 });
        let v1257: f64 = (if self.scalar_v288 { ((v296 * v692) + (v144 * ((v1205 + (self.scalar_v161 * v1217)) + ((v294 * v1205) + (v289 * (self.scalar_v164 * v1217)))))) } else { v1095 });
        let v1258: f64 = (if self.scalar_v288 { (v144 * ((self.scalar_v1206 + (self.scalar_v161 * v1218)) + ((v294 * self.scalar_v1206) + (v289 * (self.scalar_v164 * v1218))))) } else { v1096 });
        let v1259: f64 = (if self.scalar_v288 { v838 } else { v864 });
        let v1260: f64 = (if self.scalar_v288 { v839 } else { v865 });
        let v1261: f64 = (if self.scalar_v288 { v840 } else { v866 });
        let v1262: f64 = (if self.scalar_v288 { v11 } else { v867 });
        let v1263: f64 = (v299 * v1259);
        let v1265: f64 = (v299 * v1260);
        let v1267: f64 = (v299 * v1261);
        let v1269: f64 = (v299 * v1262);
        let v1271: f64 = (if self.scalar_v288 { (v1263 + v1263) } else { v11 });
        let v1272: f64 = (if self.scalar_v288 { (v1265 + v1265) } else { v11 });
        let v1273: f64 = (if self.scalar_v288 { (v1267 + v1267) } else { v11 });
        let v1274: f64 = (if self.scalar_v288 { (v1269 + v1269) } else { v11 });
        let v1311: f64 = (if self.scalar_v288 { ((v306 * v691) + (v144 * ((v1259 + (self.scalar_v161 * v1271)) + ((v304 * v1271) + (v301 * (self.scalar_v164 * v1259)))))) } else { v890 });
        let v1312: f64 = (if self.scalar_v288 { (v144 * ((v1260 + (self.scalar_v161 * v1272)) + ((v304 * v1272) + (v301 * (self.scalar_v164 * v1260))))) } else { v891 });
        let v1313: f64 = (if self.scalar_v288 { ((v306 * v692) + (v144 * ((v1261 + (self.scalar_v161 * v1273)) + ((v304 * v1273) + (v301 * (self.scalar_v164 * v1261)))))) } else { v892 });
        let v1314: f64 = (if self.scalar_v288 { (v144 * ((v1262 + (self.scalar_v161 * v1274)) + ((v304 * v1274) + (v301 * (self.scalar_v164 * v1262))))) } else { v893 });
        let v1315: f64 = { let limexp_arg = v298; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v1324: f64 = { let limexp_arg = v310; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v1338: f64 = (v42 - (v314 * v314));
        let v1343: f64 = (if self.scalar_v288 { ((v129 * ((v1255 * v1315) - ((-v1255) * v1324))) * v1338) } else { v1125 });
        let v1344: f64 = (if self.scalar_v288 { ((v129 * ((v1256 * v1315) - ((-v1256) * v1324))) * v1338) } else { v1126 });
        let v1345: f64 = (if self.scalar_v288 { ((v129 * ((v1257 * v1315) - ((-v1257) * v1324))) * v1338) } else { v1127 });
        let v1346: f64 = (if self.scalar_v288 { ((v129 * ((v1258 * v1315) - ((-v1258) * v1324))) * v1338) } else { v1128 });
        let v1347: f64 = { let limexp_arg = v308; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v1356: f64 = { let limexp_arg = v318; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v1370: f64 = (v42 - (v322 * v322));
        let v1375: f64 = (if self.scalar_v288 { ((v129 * ((v1311 * v1347) - ((-v1311) * v1356))) * v1370) } else { v11 });
        let v1376: f64 = (if self.scalar_v288 { ((v129 * ((v1312 * v1347) - ((-v1312) * v1356))) * v1370) } else { v11 });
        let v1377: f64 = (if self.scalar_v288 { ((v129 * ((v1313 * v1347) - ((-v1313) * v1356))) * v1370) } else { v11 });
        let v1378: f64 = (if self.scalar_v288 { ((v129 * ((v1314 * v1347) - ((-v1314) * v1356))) * v1370) } else { v11 });
        let v1403: f64 = (v42 - (v332 * v332));
        let v1420: f64 = (v42 - (v335 * v335));
        let v1470: f64 = (v348 * ((v344 * (v113 * v1343)) + (v343 * (if self.scalar_v288 { ((v327 + (v7 * (if self.scalar_v288 { (self.scalar_v147 * v1343) } else { v1133 }))) * v1403) } else { v1150 }))));
        let v1475: f64 = ((v348 * ((v344 * (v113 * v1344)) + (v343 * (if self.scalar_v288 { ((v7 * (if self.scalar_v288 { (self.scalar_v147 * v1344) } else { v1134 })) * v1403) } else { v1151 })))) + (v345 * (v7 * (if self.scalar_v288 { (self.scalar_v219 * v1344) } else { v11 }))));
        let v1476: f64 = (v348 * ((v344 * (v113 * v1345)) + (v343 * (if self.scalar_v288 { (((-v327) + (v7 * (if self.scalar_v288 { (self.scalar_v147 * v1345) } else { v1135 }))) * v1403) } else { v1152 }))));
        let v1481: f64 = ((v348 * ((v344 * (v113 * v1346)) + (v343 * (if self.scalar_v288 { ((v7 * (if self.scalar_v288 { (self.scalar_v147 * v1346) } else { v1136 })) * v1403) } else { v1153 })))) + (v345 * (v7 * (if self.scalar_v288 { (self.scalar_v219 * v1346) } else { v11 }))));
        let v1502: f64 = ((v352 * (v113 * v1377)) + (v351 * (-(if self.scalar_v288 { (((-v330) + (v7 * (if self.scalar_v288 { (self.scalar_v147 * v1377) } else { v11 }))) * v1420) } else { v11 }))));
        let v1517: f64 = (v355 * ((v352 * (v113 * v1375)) + (v351 * (-(if self.scalar_v288 { ((v330 + (v7 * (if self.scalar_v288 { (self.scalar_v147 * v1375) } else { v11 }))) * v1420) } else { v11 })))));
        let v1522: f64 = ((v355 * ((v352 * (v113 * v1376)) + (v351 * (-(if self.scalar_v288 { ((v7 * (if self.scalar_v288 { (self.scalar_v147 * v1376) } else { v11 })) * v1420) } else { v11 }))))) + (v353 * (-(v7 * (if self.scalar_v288 { (self.scalar_v219 * v1376) } else { v11 })))));
        let v1528: f64 = ((v355 * ((v352 * (v113 * v1378)) + (v351 * (-(if self.scalar_v288 { ((v7 * (if self.scalar_v288 { (self.scalar_v147 * v1378) } else { v11 })) * v1420) } else { v11 }))))) + (v353 * (-(v7 * (if self.scalar_v288 { (self.scalar_v219 * v1378) } else { v11 })))));
        let v1533: f64 = ((if self.scalar_v288 { (v1470 + (v345 * (v937 + (v342 + (v7 * (if self.scalar_v288 { (self.scalar_v219 * v1343) } else { v11 })))))) } else { v953 }) - (if self.scalar_v288 { (v1517 + (v353 * (-(v339 + (v7 * (if self.scalar_v288 { (self.scalar_v219 * v1375) } else { v11 })))))) } else { v1025 }));
        let v1535: f64 = ((if self.scalar_v288 { (v1476 + (v345 * (v938 + ((-v342) + (v7 * (if self.scalar_v288 { (self.scalar_v219 * v1345) } else { v11 })))))) } else { v955 }) - (if self.scalar_v288 { ((v355 * v1502) + (v353 * (-((-v339) + (v7 * (if self.scalar_v288 { (self.scalar_v219 * v1377) } else { v11 })))))) } else { v1027 }));
        let v1543: f64 = (if self.scalar_v288 { (v129 * v1535) } else { (if self.scalar_v251 { v1195 } else { (if self.scalar_v200 { (v129 * (v955 - v1027)) } else { (if self.scalar_v183 { ((v196 * (v812 + (v181 * v804))) + (v190 * self.scalar_v818)) } else { v11 }) }) }) });
        let v1547: f64 = (v362 * v362);
        let v1572: f64 = (v374 * v374);
        let v1591: f64 = (if self.scalar_v250 { (self.scalar_v367 * v1343) } else { (if self.scalar_v249 { (self.scalar_v367 * v753) } else { v11 }) });
        let v1592: f64 = (if self.scalar_v250 { (self.scalar_v367 * v1344) } else { (if self.scalar_v249 { (self.scalar_v367 * v754) } else { v11 }) });
        let v1593: f64 = (if self.scalar_v250 { (self.scalar_v367 * v1345) } else { (if self.scalar_v249 { (self.scalar_v367 * v755) } else { v11 }) });
        let v1594: f64 = (if self.scalar_v250 { (self.scalar_v367 * v1346) } else { (if self.scalar_v249 { (self.scalar_v367 * v756) } else { v11 }) });
        let v1603: f64 = (if v391 { v1591 } else { (if v383 { (v386 * v1591) } else { v11 }) });
        let v1604: f64 = (if v391 { v1592 } else { (if v383 { (v386 * v1592) } else { v11 }) });
        let v1605: f64 = (if v391 { v1593 } else { (if v383 { (v386 * v1593) } else { v11 }) });
        let v1606: f64 = (if v391 { v1594 } else { (if v383 { (v386 * v1594) } else { v11 }) });
        let v1613: f64 = (if self.scalar_v406 { v11 } else { (if self.scalar_v395 { v11 } else { v1203 }) });
        let v1615: f64 = (if self.scalar_v406 { v11 } else { (if self.scalar_v395 { v11 } else { v1205 }) });
        let v1618: f64 = (v42 - (v413 * v413));
        let v1623: f64 = (v42 - (v415 * v415));
        let v1633: f64 = { let limexp_arg = v422; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v1640: f64 = (self.scalar_v421 * (-v1613));
        let v1641: f64 = (self.scalar_v421 * (-(if self.scalar_v406 { v11 } else { (if self.scalar_v395 { v11 } else { v1204 }) })));
        let v1642: f64 = (self.scalar_v421 * (((v136 * (if self.scalar_v418 { v396 } else { (if self.scalar_v412 { (-v1618) } else { self.scalar_v1611 }) })) * v1633) - v1615));
        let v1643: f64 = (self.scalar_v421 * (((v136 * (if self.scalar_v418 { v42 } else { (if self.scalar_v412 { v1618 } else { self.scalar_v1612 }) })) * v1633) - self.scalar_v1616));
        let v1646: f64 = { let limexp_arg = v426; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v1652: f64 = (self.scalar_v421 * (((v136 * (if self.scalar_v418 { v396 } else { (if self.scalar_v412 { (-v1623) } else { self.scalar_v1611 }) })) * v1646) - v1613));
        let v1653: f64 = (self.scalar_v421 * (-v1615));
        let v1654: f64 = (self.scalar_v421 * ((v136 * (if self.scalar_v418 { v42 } else { (if self.scalar_v412 { v1623 } else { self.scalar_v1612 }) })) * v1646));
        let v1660: f64 = (v42 - (v435 * v435));
        let v1666: f64 = (v42 - (v441 * v441));
        let v1667: f64 = (self.scalar_v438 * v1666);
        let v1668: f64 = (self.scalar_v1664 * v1666);
        let v1671: f64 = (v42 - (v447 * v447));
        let v1672: f64 = (self.scalar_v1669 * v1671);
        let v1673: f64 = (self.scalar_v444 * v1671);
        let v1677: f64 = (v42 - (v453 * v453));
        let v1707: f64 = ((v480) as f64).sinh();
        let v1710: f64 = (if self.scalar_v477 { (self.scalar_v432 * v1707) } else { v11 });
        let v1711: f64 = (if self.scalar_v477 { (self.scalar_v1657 * v1707) } else { v11 });
        let v1720: f64 = (if self.scalar_v477 { (self.scalar_v432 * v1716) } else { v11 });
        let v1721: f64 = (if self.scalar_v477 { (self.scalar_v1658 * v1716) } else { v11 });
        let v1740: f64 = ((v492 * v1667) + (v479 * ((self.scalar_v432 + (if self.scalar_v477 { (v1720 / v486) } else { v11 })) - (if self.scalar_v477 { (self.scalar_v432 + (if self.scalar_v477 { (v1710 / v482) } else { v11 })) } else { v11 }))));
        let v1743: f64 = ((v492 * v1668) + (v479 * ((self.scalar_v1658 + (if self.scalar_v477 { (v1721 / v486) } else { v11 })) - (if self.scalar_v477 { (self.scalar_v1657 + (if self.scalar_v477 { (v1711 / v482) } else { v11 })) } else { v11 }))));
        let v1760: f64 = ((v501) as f64).sinh();
        let v1792: f64 = ((self.scalar_v1675 + (if self.scalar_v477 { ((if self.scalar_v477 { (self.scalar_v1675 * v1769) } else { v1720 }) / v507) } else { v11 })) - (if self.scalar_v477 { (self.scalar_v1657 + (if self.scalar_v477 { ((if self.scalar_v477 { (self.scalar_v1657 * v1760) } else { v1710 }) / v503) } else { v11 })) } else { v11 }));
        let v1793: f64 = ((self.scalar_v432 + (if self.scalar_v477 { ((if self.scalar_v477 { (self.scalar_v432 * v1769) } else { v1721 }) / v507) } else { v11 })) - (if self.scalar_v477 { (self.scalar_v432 + (if self.scalar_v477 { ((if self.scalar_v477 { (self.scalar_v432 * v1760) } else { v1711 }) / v503) } else { v11 })) } else { v11 }));
        let v1825: f64 = (-(if self.scalar_v288 { (v129 * v1533) } else { v1199 }));
        let v1826: f64 = (-(if self.scalar_v288 { (v129 * ((if self.scalar_v288 { v1475 } else { v954 }) - (if self.scalar_v288 { v1522 } else { v1026 }))) } else { v1200 }));
        let v1827: f64 = (-v1543);
        let v1828: f64 = (-(if self.scalar_v288 { (v129 * ((if self.scalar_v288 { v1481 } else { v956 }) - (if self.scalar_v288 { v1528 } else { v1028 }))) } else { v1202 }));
        let v1829: f64 = ddt_scale;
        let v1834: f64 = (if self.scalar_v458 { ((if self.scalar_v477 { ((v117 * (self.scalar_v1748 + (((v513 * v1672) + (v449 * v1792)) / self.scalar_v97))) + self.scalar_v1812) } else { v11 }) * v1829) } else { v11 });
        let v1835: f64 = (if self.scalar_v458 { ((if self.scalar_v477 { (v117 * (((v513 * v1673) + (v449 * v1793)) / self.scalar_v97)) } else { v11 }) * v1829) } else { v11 });
        let v1836: f64 = (if self.scalar_v458 { (v1817 * v1829) } else { v11 });
        let v1837: f64 = (if self.scalar_v458 { ((if self.scalar_v477 { (v117 * ((v449 * (if self.scalar_v477 { ((if self.scalar_v477 { v11 } else { v1722 }) / v507) } else { v11 })) / self.scalar_v97)) } else { v11 }) * v1829) } else { v11 });
        let v1841: f64 = (if self.scalar_v458 { ((if self.scalar_v477 { (v116 * (v1740 / self.scalar_v91)) } else { v11 }) * v1829) } else { v11 });
        let v1842: f64 = (if self.scalar_v458 { ((if self.scalar_v477 { ((v116 * ((v1743 / self.scalar_v91) + self.scalar_v1748)) + self.scalar_v1754) } else { v11 }) * v1829) } else { v11 });
        let v1843: f64 = (if self.scalar_v458 { (v1759 * v1829) } else { v11 });
        let v1853: f64 = (if self.scalar_v583 { (v1829 * ((-v525) + (v9 * (if self.scalar_v477 { v11 } else { (if self.scalar_v464 { (v117 * ((v454 * v1672) + (v449 * (self.scalar_v1675 * v1677)))) } else { v11 }) })))) } else { v11 });
        let v1854: f64 = (if self.scalar_v583 { (v1829 * (v9 * (if self.scalar_v477 { v11 } else { (if self.scalar_v464 { (v117 * ((v454 * v1673) + (v449 * (self.scalar_v432 * v1677)))) } else { v11 }) }))) } else { v11 });
        let v1855: f64 = (if self.scalar_v583 { (v1829 * (v525 + (v9 * (if self.scalar_v477 { v11 } else { (if self.scalar_v464 { (v117 * (v449 * (self.scalar_v97 * v1677))) } else { v11 }) })))) } else { v11 });
        let v1865: f64 = (if self.scalar_v583 { (v1829 * (v2 * (if self.scalar_v477 { v11 } else { (if self.scalar_v464 { ((v465 * v1667) + (v442 * (v116 * (self.scalar_v432 * v1660)))) } else { v11 }) }))) } else { v11 });
        let v1866: f64 = (if self.scalar_v583 { (v1829 * ((-v523) + (v2 * (if self.scalar_v477 { v11 } else { (if self.scalar_v464 { ((v465 * v1668) + (v442 * (v116 * (self.scalar_v1658 * v1660)))) } else { v11 }) })))) } else { v11 });
        let v1867: f64 = (if self.scalar_v583 { (v1829 * (v523 + (v2 * (if self.scalar_v477 { v11 } else { (if self.scalar_v464 { (v442 * (v116 * (self.scalar_v91 * v1660))) } else { v11 }) })))) } else { v11 });
        let v1870: f64 = (-v119);
        let v1873: f64 = (v377 * v377);
        let v1886: f64 = (if self.scalar_v526 { ((-(v599 * (if self.scalar_v250 { ((-(v118 * v1343)) / v1572) } else { (if self.scalar_v249 { ((-(v118 * v753)) / v1547) } else { v11 }) }))) / v1873) } else { v11 });
        let v1887: f64 = (if self.scalar_v526 { ((-(v599 * (if self.scalar_v250 { ((-(v118 * v1344)) / v1572) } else { (if self.scalar_v249 { ((-(v118 * v754)) / v1547) } else { v11 }) }))) / v1873) } else { v11 });
        let v1888: f64 = (if self.scalar_v526 { (((-v377) - (v599 * (if self.scalar_v250 { ((-(v118 * v1345)) / v1572) } else { (if self.scalar_v249 { ((-(v118 * v755)) / v1547) } else { v11 }) }))) / v1873) } else { v11 });
        let v1889: f64 = (if self.scalar_v526 { ((-(v599 * (if self.scalar_v250 { ((-(v118 * v1346)) / v1572) } else { (if self.scalar_v249 { ((-(v118 * v756)) / v1547) } else { v11 }) }))) / v1873) } else { v11 });
        let v1890: f64 = (if self.scalar_v526 { (v42 / v377) } else { v11 });
        let v1905: f64 = (self.scalar_v535 * v1829);
        let v1906: f64 = (if self.scalar_v534 { v1905 } else { v11 });
        let v1907: f64 = (if self.scalar_v623 { v1905 } else { v11 });
        let v1912: f64 = (if self.scalar_v537 { (v628 * v1603) } else { v11 });
        let v1913: f64 = (if self.scalar_v537 { (v628 * v1604) } else { v11 });
        let v1914: f64 = (if self.scalar_v537 { (v628 * v1605) } else { v11 });
        let v1915: f64 = (if self.scalar_v537 { (v628 * v1606) } else { v11 });
        let v1916: f64 = (if self.scalar_v537 { v393 } else { v11 });
        let v1921: f64 = (if self.scalar_v540 { (v634 * v1603) } else { v11 });
        let v1922: f64 = (if self.scalar_v540 { (v634 * v1604) } else { v11 });
        let v1923: f64 = (if self.scalar_v540 { (v634 * v1605) } else { v11 });
        let v1924: f64 = (if self.scalar_v540 { (v634 * v1606) } else { v11 });
        let v1925: f64 = (if self.scalar_v540 { v392 } else { v11 });
        let v1926: f64 = (self.scalar_v541 * v1829);
        let v1927: f64 = (if self.scalar_v540 { v1926 } else { v11 });
        let v1928: f64 = (if self.scalar_v642 { v1926 } else { v11 });
        let v1930: f64 = (if self.scalar_v549 { v568 } else { v11 });
        let v1931: f64 = (if self.scalar_v549 { v564 } else { v11 });
        let v1933: f64 = (if self.scalar_v549 { (v655 * v1829) } else { v11 });
        let v1935: f64 = (if self.scalar_v571 { (self.scalar_v659 * v1829) } else { v11 });

        let d572_dn3: f64 = v1825;
        let d572_dn4: f64 = v1826;
        let d572_dn5: f64 = v1827;
        let d572_dn8: f64 = v1828;
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(12),
            None,
            multiplicity * (v572),
            [3, 4, 5, 8],
            [d572_dn3, d572_dn4, d572_dn5, d572_dn8],
            [],
            [],
            multiplicity,
        );
        let d575_dn12: f64 = self.scalar_v573;
        let v575_ddt: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, v575);
        stamper.stamp_current_node1_local(
            Some(12),
            None,
            multiplicity * (v575_ddt),
            12,
            multiplicity * (((d575_dn12) * ddt_scale)),
        );
        let d10_dn13: f64 = v42;
        stamper.stamp_current_node1_local(
            Some(13),
            None,
            multiplicity * (v10),
            13,
            multiplicity * (d10_dn13),
        );
        let d578_db0: f64 = self.scalar_v576;
        let v578_ddt: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, v578);
        stamper.stamp_potential_branch_local(
            Some(12),
            Some(13),
            0,
            multiplicity,
        );
        stamper.stamp_potential_branch1_local(
            0,
            v578_ddt,
            0,
            ((d578_db0) * ddt_scale),
        );
        let d10_dn13: f64 = v42;
        stamper.stamp_current_node1_local(
            Some(3),
            Some(5),
            multiplicity * (v10),
            13,
            multiplicity * (d10_dn13),
        );
        let d425_dn3: f64 = v1640;
        let d425_dn4: f64 = v1641;
        let d425_dn5: f64 = v1642;
        let d425_dn8: f64 = v1643;
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(8),
            Some(5),
            multiplicity * (v425),
            [3, 4, 5, 8],
            [d425_dn3, d425_dn4, d425_dn5, d425_dn8],
            [],
            [],
            multiplicity,
        );
        let d429_dn3: f64 = v1652;
        let d429_dn4: f64 = v1641;
        let d429_dn5: f64 = v1653;
        let d429_dn7: f64 = v1654;
        let d429_dn8: f64 = self.scalar_v1655;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(7),
            Some(3),
            multiplicity * (v429),
            [3, 4, 5, 7, 8],
            [d429_dn3, d429_dn4, d429_dn5, d429_dn7, d429_dn8],
            [],
            [],
            multiplicity,
        );
        let d580_dn3: f64 = v1834;
        let d580_dn5: f64 = v1835;
        let d580_dn7: f64 = v1836;
        let d580_dn8: f64 = v1837;
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(7),
            Some(3),
            multiplicity * (v580),
            [3, 5, 7, 8],
            [d580_dn3, d580_dn5, d580_dn7, d580_dn8],
            [],
            [],
            multiplicity,
        );
        let d582_dn3: f64 = v1841;
        let d582_dn5: f64 = v1842;
        let d582_dn8: f64 = v1843;
        stamper.stamp_current_node3_local(
            Some(8),
            Some(5),
            multiplicity * (v582),
            3,
            multiplicity * (d582_dn3),
            5,
            multiplicity * (d582_dn5),
            8,
            multiplicity * (d582_dn8),
        );
        let d586_dn3: f64 = v1853;
        let d586_dn5: f64 = v1854;
        let d586_dn7: f64 = v1855;
        stamper.stamp_current_node3_local(
            Some(7),
            Some(3),
            multiplicity * (v586),
            3,
            multiplicity * (d586_dn3),
            5,
            multiplicity * (d586_dn5),
            7,
            multiplicity * (d586_dn7),
        );
        let d589_dn3: f64 = v1865;
        let d589_dn5: f64 = v1866;
        let d589_dn8: f64 = v1867;
        stamper.stamp_current_node3_local(
            Some(8),
            Some(5),
            multiplicity * (v589),
            3,
            multiplicity * (d589_dn3),
            5,
            multiplicity * (d589_dn5),
            8,
            multiplicity * (d589_dn8),
        );
        let d593_dn1: f64 = self.scalar_v590;
        let d593_dn3: f64 = self.scalar_v1868;
        let v593_ddt: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 6, v593);
        stamper.stamp_current_node2_local(
            Some(4),
            Some(3),
            multiplicity * (v593_ddt),
            1,
            multiplicity * (((d593_dn1) * ddt_scale)),
            3,
            multiplicity * (((d593_dn3) * ddt_scale)),
        );
        let d595_dn3: f64 = self.scalar_v594;
        let d595_dn5: f64 = self.scalar_v1869;
        let v595_ddt: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 7, v595);
        stamper.stamp_current_node2_local(
            Some(3),
            Some(5),
            multiplicity * (v595_ddt),
            3,
            multiplicity * (((d595_dn3) * ddt_scale)),
            5,
            multiplicity * (((d595_dn5) * ddt_scale)),
        );
        let d598_dn3: f64 = v119;
        let d598_dn10: f64 = v1870;
        let v598_ddt: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 8, v598);
        stamper.stamp_current_node2_local(
            Some(3),
            Some(10),
            multiplicity * (v598_ddt),
            3,
            multiplicity * (((d598_dn3) * ddt_scale)),
            10,
            multiplicity * (((d598_dn10) * ddt_scale)),
        );
        let d601_dn3: f64 = v1886;
        let d601_dn4: f64 = v1887;
        let d601_dn5: f64 = v1888;
        let d601_dn8: f64 = v1889;
        let d601_dn10: f64 = v1890;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(10),
            Some(5),
            multiplicity * (v601),
            [3, 4, 5, 8, 10],
            [d601_dn3, d601_dn4, d601_dn5, d601_dn8, d601_dn10],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(10),
            Some(5),
            1,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            1,
            v11,
        );
        let d605_dn8: f64 = self.scalar_v1891;
        let d605_dn9: f64 = self.scalar_v602;
        let v605_ddt: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 9, v605);
        stamper.stamp_current_node2_local(
            Some(9),
            Some(8),
            multiplicity * (v605_ddt),
            8,
            multiplicity * (((d605_dn8) * ddt_scale)),
            9,
            multiplicity * (((d605_dn9) * ddt_scale)),
        );
        let d608_dn5: f64 = self.scalar_v1894;
        let d608_dn9: f64 = self.scalar_v1895;
        stamper.stamp_current_node2_local(
            Some(9),
            Some(5),
            multiplicity * (v608),
            5,
            multiplicity * (d608_dn5),
            9,
            multiplicity * (d608_dn9),
        );
        stamper.stamp_potential_branch_local(
            Some(9),
            Some(5),
            2,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            2,
            v11,
        );
        let d611_dn4: f64 = self.scalar_v1898;
        let d611_dn7: f64 = self.scalar_v1899;
        stamper.stamp_current_node2_local(
            Some(4),
            Some(7),
            multiplicity * (v611),
            4,
            multiplicity * (d611_dn4),
            7,
            multiplicity * (d611_dn7),
        );
        stamper.stamp_current_const_local(
            Some(4),
            Some(7),
            multiplicity * (v11),
        );
        stamper.stamp_potential_branch_local(
            Some(4),
            Some(7),
            3,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            3,
            v11,
        );
        let d614_dn4: f64 = self.scalar_v1902;
        let d614_dn8: f64 = self.scalar_v1903;
        stamper.stamp_current_node2_local(
            Some(4),
            Some(8),
            multiplicity * (v614),
            4,
            multiplicity * (d614_dn4),
            8,
            multiplicity * (d614_dn8),
        );
        stamper.stamp_potential_branch_local(
            Some(4),
            Some(8),
            4,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            4,
            v11,
        );
        let d617_db5: f64 = self.scalar_v1904;
        stamper.stamp_potential_branch_local(
            Some(1),
            Some(4),
            5,
            multiplicity,
        );
        stamper.stamp_potential_branch1_local(
            5,
            v617,
            5,
            d617_db5,
        );
        let d621_db6: f64 = v1906;
        stamper.stamp_potential_branch_local(
            Some(1),
            Some(4),
            6,
            multiplicity,
        );
        stamper.stamp_potential_branch1_local(
            6,
            v621,
            6,
            d621_db6,
        );
        stamper.stamp_potential_branch_local(
            Some(1),
            Some(4),
            7,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            7,
            v11,
        );
        let d627_db8: f64 = v1907;
        stamper.stamp_potential_branch_local(
            Some(1),
            Some(4),
            8,
            multiplicity,
        );
        stamper.stamp_potential_branch1_local(
            8,
            v627,
            8,
            d627_db8,
        );
        stamper.stamp_potential_branch_local(
            Some(1),
            Some(4),
            9,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            9,
            v11,
        );
        let d630_dn3: f64 = v1912;
        let d630_dn4: f64 = v1913;
        let d630_dn5: f64 = v1914;
        let d630_dn8: f64 = v1915;
        let d630_db10: f64 = v1916;
        stamper.stamp_potential_branch_local(
            Some(5),
            Some(6),
            10,
            multiplicity,
        );
        stamper.stamp_potential_sparse_local::<4, 1>(
            10,
            v630,
            [3, 4, 5, 8],
            [d630_dn3, d630_dn4, d630_dn5, d630_dn8],
            [10],
            [d630_db10],
        );
        stamper.stamp_potential_branch_local(
            Some(5),
            Some(6),
            11,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            11,
            v11,
        );
        stamper.stamp_potential_branch_local(
            Some(5),
            Some(6),
            12,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            12,
            v11,
        );
        let d633_db13: f64 = self.scalar_v631;
        let v633_ddt: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 12, v633);
        stamper.stamp_potential_branch_local(
            Some(6),
            Some(2),
            13,
            multiplicity,
        );
        stamper.stamp_potential_branch1_local(
            13,
            v633_ddt,
            13,
            ((d633_db13) * ddt_scale),
        );
        let d636_dn3: f64 = v1921;
        let d636_dn4: f64 = v1922;
        let d636_dn5: f64 = v1923;
        let d636_dn8: f64 = v1924;
        let d636_db14: f64 = v1925;
        stamper.stamp_potential_branch_local(
            Some(3),
            Some(0),
            14,
            multiplicity,
        );
        stamper.stamp_potential_sparse_local::<4, 1>(
            14,
            v636,
            [3, 4, 5, 8],
            [d636_dn3, d636_dn4, d636_dn5, d636_dn8],
            [14],
            [d636_db14],
        );
        let d640_db15: f64 = v1927;
        stamper.stamp_potential_branch_local(
            Some(3),
            Some(0),
            15,
            multiplicity,
        );
        stamper.stamp_potential_branch1_local(
            15,
            v640,
            15,
            d640_db15,
        );
        stamper.stamp_potential_branch_local(
            Some(3),
            Some(0),
            16,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            16,
            v11,
        );
        let d646_db17: f64 = v1928;
        stamper.stamp_potential_branch_local(
            Some(3),
            Some(0),
            17,
            multiplicity,
        );
        stamper.stamp_potential_branch1_local(
            17,
            v646,
            17,
            d646_db17,
        );
        stamper.stamp_potential_branch_local(
            Some(3),
            Some(0),
            18,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            18,
            v11,
        );
        stamper.stamp_current_const_local(
            Some(3),
            Some(5),
            multiplicity * (v11),
        );
        stamper.stamp_current_const_local(
            Some(3),
            Some(5),
            multiplicity * (v11),
        );
        stamper.stamp_current_const_local(
            Some(14),
            None,
            multiplicity * (v11),
        );
        let d648_dn14: f64 = self.scalar_v1929;
        stamper.stamp_current_node1_local(
            Some(14),
            None,
            multiplicity * (v648),
            14,
            multiplicity * (d648_dn14),
        );
        stamper.stamp_current_const_local(
            Some(15),
            None,
            multiplicity * (v11),
        );
        let d650_dn15: f64 = self.scalar_v1929;
        stamper.stamp_current_node1_local(
            Some(15),
            None,
            multiplicity * (v650),
            15,
            multiplicity * (d650_dn15),
        );
        let d648_dn14: f64 = self.scalar_v1929;
        stamper.stamp_current_node1_local(
            Some(4),
            Some(5),
            multiplicity * (v648),
            14,
            multiplicity * (d648_dn14),
        );
        let d654_dn14: f64 = v1930;
        let d654_dn15: f64 = v1931;
        stamper.stamp_current_node2_local(
            Some(4),
            Some(3),
            multiplicity * (v654),
            14,
            multiplicity * (d654_dn14),
            15,
            multiplicity * (d654_dn15),
        );
        let d658_dn14: f64 = v1933;
        stamper.stamp_current_node1_local(
            Some(4),
            Some(3),
            multiplicity * (v658),
            14,
            multiplicity * (d658_dn14),
        );
        stamper.stamp_current_const_local(
            Some(3),
            Some(5),
            multiplicity * (v11),
        );
        stamper.stamp_current_const_local(
            Some(4),
            Some(5),
            multiplicity * (v11),
        );
        stamper.stamp_current_const_local(
            Some(3),
            Some(5),
            multiplicity * (v11),
        );
        stamper.stamp_current_const_local(
            Some(3),
            Some(5),
            multiplicity * (v11),
        );
        let d647_dn14: f64 = v42;
        stamper.stamp_current_node1_local(
            Some(14),
            None,
            multiplicity * (v647),
            14,
            multiplicity * (d647_dn14),
        );
        let d649_dn15: f64 = v42;
        stamper.stamp_current_node1_local(
            Some(15),
            None,
            multiplicity * (v649),
            15,
            multiplicity * (d649_dn15),
        );
        stamper.stamp_current_const_local(
            Some(8),
            Some(5),
            multiplicity * (v11),
        );
        stamper.stamp_current_const_local(
            Some(7),
            Some(3),
            multiplicity * (v11),
        );
        stamper.stamp_current_const_local(
            Some(8),
            Some(5),
            multiplicity * (v11),
        );
        stamper.stamp_current_const_local(
            Some(7),
            Some(3),
            multiplicity * (v11),
        );
        let d662_dn11: f64 = v1935;
        stamper.stamp_current_node1_local(
            Some(11),
            None,
            multiplicity * (v662),
            11,
            multiplicity * (d662_dn11),
        );
        stamper.stamp_current_const_local(
            Some(11),
            None,
            multiplicity * (v669),
        );
        let d671_dn11: f64 = self.scalar_v1937;
        stamper.stamp_current_node1_local(
            Some(11),
            None,
            multiplicity * (v671),
            11,
            multiplicity * (d671_dn11),
        );
        let d675_dn11: f64 = self.scalar_v1938;
        stamper.stamp_current_node1_local(
            Some(11),
            None,
            multiplicity * (v675),
            11,
            multiplicity * (d675_dn11),
        );
    }

    pub fn stamp_reactive(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedReactiveStamper<'_>) {
        let nodes = self.nodes;
        let branches = self.branches;
        self.ensure_temperature_static(ctx.temperature(), ctx.thermal_voltage());
        let p = &(*self.params);
        let multiplicity = self.multiplicity;
        let v0: f64 = ctx.node_voltage(nodes[8]);
        let v1: f64 = ctx.node_voltage(nodes[5]);
        let v2: f64 = (v0 - v1);
        let v4: f64 = ctx.node_voltage(nodes[3]);
        let v7: f64 = (v4 - v1);
        let v9: f64 = (ctx.node_voltage(nodes[7]) - v4);
        let v11: f64 = 0.0;
        let v30: f64 = ctx.node_voltage(nodes[11]);
        let v33: f64 = (if (self.scalar_v29 != 0.0) { (self.scalar_v21 + ((v30) as f64).abs()) } else { self.scalar_v21 });
        let v37: f64 = (((v33 - self.scalar_v28)) as f64).abs();
        let v41: bool = ((v37 > v11) || self.scalar_v40);
        let v42: f64 = 1.0;
        let v112: bool = (!v41);
        let v116: f64 = (if v112 { self.scalar_v61 } else { (if v41 { (self.scalar_v61 * (v42 + (v37 * self.scalar_v62))) } else { v11 }) });
        let v117: f64 = (if v112 { self.scalar_v67 } else { (if v41 { (self.scalar_v67 * (v42 + (v37 * self.scalar_v68))) } else { v11 }) });
        let v119: f64 = (if v112 { self.scalar_v79 } else { (if v41 { (self.scalar_v79 * (v42 + (v37 * self.scalar_v80))) } else { v11 }) });
        let v121: f64 = (if v112 { self.scalar_v90 } else { (if v41 { (self.scalar_v90 + (v37 * self.scalar_v92)) } else { v11 }) });
        let v122: f64 = (if v112 { self.scalar_v96 } else { (if v41 { (self.scalar_v96 + (v37 * self.scalar_v98)) } else { v11 }) });
        let v433: f64 = (v7 * self.scalar_v432);
        let v434: f64 = ((v121 + (v2 * self.scalar_v91)) + v433);
        let v435: f64 = ((v434) as f64).tanh();
        let v441: f64 = (((self.scalar_v437 + (v7 * self.scalar_v438))) as f64).tanh();
        let v442: f64 = (v42 + v441);
        let v447: f64 = (((self.scalar_v443 - (v7 * self.scalar_v444))) as f64).tanh();
        let v449: f64 = ((v42 + v447) - self.scalar_v432);
        let v452: f64 = ((v122 + (v9 * self.scalar_v97)) - v433);
        let v453: f64 = ((v452) as f64).tanh();
        let v454: f64 = (v42 + v453);
        let v465: f64 = (v116 * (v42 + v435));
        let v479: f64 = (if self.scalar_v477 { (v442 - self.scalar_v432) } else { v442 });
        let v480: f64 = (v121 + v433);
        let v482: f64 = (if self.scalar_v477 { ((v480) as f64).cosh() } else { v11 });
        let v486: f64 = (if self.scalar_v477 { ((v434) as f64).cosh() } else { v11 });
        let v492: f64 = ((v434 + (if self.scalar_v477 { ((v486) as f64).ln() } else { v11 })) - (if self.scalar_v477 { (v480 + (if self.scalar_v477 { ((v482) as f64).ln() } else { v11 })) } else { v11 }));
        let v501: f64 = (v122 - v433);
        let v503: f64 = (if self.scalar_v477 { ((v501) as f64).cosh() } else { v482 });
        let v507: f64 = (if self.scalar_v477 { ((v452) as f64).cosh() } else { v486 });
        let v513: f64 = ((v452 + (if self.scalar_v477 { ((v507) as f64).ln() } else { v11 })) - (if self.scalar_v477 { (v501 + (if self.scalar_v477 { ((v503) as f64).ln() } else { v11 })) } else { v11 }));
        let v1716: f64 = ((v434) as f64).sinh();
        let v1722: f64 = (if self.scalar_v477 { (self.scalar_v91 * v1716) } else { v11 });
        let v1759: f64 = (if self.scalar_v477 { (self.scalar_v459 + (v116 * (self.scalar_v470 + ((v479 * (self.scalar_v91 + (if self.scalar_v477 { (v1722 / v486) } else { v11 }))) / self.scalar_v91)))) } else { v11 });
        let v522: f64 = v1759;
        let v523: f64 = (if self.scalar_v477 { v522 } else { (if self.scalar_v464 { (self.scalar_v459 + (v442 * v465)) } else { self.scalar_v460 }) });
        let v1769: f64 = ((v452) as f64).sinh();
        let v1817: f64 = (if self.scalar_v477 { (self.scalar_v461 + (v117 * (self.scalar_v470 + ((v449 * (self.scalar_v97 + (if self.scalar_v477 { ((if self.scalar_v477 { (self.scalar_v97 * v1769) } else { v11 }) / v507) } else { v11 }))) / self.scalar_v97)))) } else { v11 });
        let v524: f64 = v1817;
        let v525: f64 = (if self.scalar_v477 { v524 } else { (if self.scalar_v464 { (self.scalar_v461 + (v117 * ((v449 * v454) + self.scalar_v470))) } else { self.scalar_v462 }) });
        let v575: f64 = (self.scalar_v573 * ctx.node_voltage(nodes[12]));
        let v578: f64 = (self.scalar_v576 * ctx.branch_current(branches[0]));
        let v579: f64 = 0.0;
        let v580: f64 = (if self.scalar_v458 { v579 } else { v11 });
        let v581: f64 = 0.0;
        let v582: f64 = (if self.scalar_v458 { v581 } else { v11 });
        let v585: f64 = 0.0;
        let v586: f64 = (if self.scalar_v583 { v585 } else { v11 });
        let v588: f64 = 0.0;
        let v589: f64 = (if self.scalar_v583 { v588 } else { v11 });
        let v593: f64 = (self.scalar_v590 * (ctx.node_voltage(nodes[1]) - v4));
        let v595: f64 = (v7 * self.scalar_v594);
        let v598: f64 = (v119 * (v4 - ctx.node_voltage(nodes[10])));
        let v605: f64 = (self.scalar_v602 * (ctx.node_voltage(nodes[9]) - v0));
        let v620: f64 = 0.0;
        let v621: f64 = (if self.scalar_v534 { v620 } else { v11 });
        let v626: f64 = 0.0;
        let v627: f64 = (if self.scalar_v623 { v626 } else { v11 });
        let v633: f64 = (self.scalar_v631 * ctx.branch_current(branches[13]));
        let v639: f64 = 0.0;
        let v640: f64 = (if self.scalar_v540 { v639 } else { v11 });
        let v645: f64 = 0.0;
        let v646: f64 = (if self.scalar_v642 { v645 } else { v11 });
        let v655: f64 = (-(if self.scalar_v549 { ((if self.scalar_v549 { ((v116 * ((v33 * 5.5226012e-23) * self.scalar_v554)) * self.scalar_v558) } else { v11 }) * 3.141592653589793) } else { v11 }));
        let v657: f64 = 0.0;
        let v658: f64 = (if self.scalar_v549 { v657 } else { v11 });
        let v661: f64 = 0.0;
        let v662: f64 = (if self.scalar_v571 { v661 } else { v11 });
        let v1660: f64 = (v42 - (v435 * v435));
        let v1666: f64 = (v42 - (v441 * v441));
        let v1667: f64 = (self.scalar_v438 * v1666);
        let v1668: f64 = (self.scalar_v1664 * v1666);
        let v1671: f64 = (v42 - (v447 * v447));
        let v1672: f64 = (self.scalar_v1669 * v1671);
        let v1673: f64 = (self.scalar_v444 * v1671);
        let v1677: f64 = (v42 - (v453 * v453));
        let v1707: f64 = ((v480) as f64).sinh();
        let v1710: f64 = (if self.scalar_v477 { (self.scalar_v432 * v1707) } else { v11 });
        let v1711: f64 = (if self.scalar_v477 { (self.scalar_v1657 * v1707) } else { v11 });
        let v1720: f64 = (if self.scalar_v477 { (self.scalar_v432 * v1716) } else { v11 });
        let v1721: f64 = (if self.scalar_v477 { (self.scalar_v1658 * v1716) } else { v11 });
        let v1740: f64 = ((v492 * v1667) + (v479 * ((self.scalar_v432 + (if self.scalar_v477 { (v1720 / v486) } else { v11 })) - (if self.scalar_v477 { (self.scalar_v432 + (if self.scalar_v477 { (v1710 / v482) } else { v11 })) } else { v11 }))));
        let v1743: f64 = ((v492 * v1668) + (v479 * ((self.scalar_v1658 + (if self.scalar_v477 { (v1721 / v486) } else { v11 })) - (if self.scalar_v477 { (self.scalar_v1657 + (if self.scalar_v477 { (v1711 / v482) } else { v11 })) } else { v11 }))));
        let v1760: f64 = ((v501) as f64).sinh();
        let v1792: f64 = ((self.scalar_v1675 + (if self.scalar_v477 { ((if self.scalar_v477 { (self.scalar_v1675 * v1769) } else { v1720 }) / v507) } else { v11 })) - (if self.scalar_v477 { (self.scalar_v1657 + (if self.scalar_v477 { ((if self.scalar_v477 { (self.scalar_v1657 * v1760) } else { v1710 }) / v503) } else { v11 })) } else { v11 }));
        let v1793: f64 = ((self.scalar_v432 + (if self.scalar_v477 { ((if self.scalar_v477 { (self.scalar_v432 * v1769) } else { v1721 }) / v507) } else { v11 })) - (if self.scalar_v477 { (self.scalar_v432 + (if self.scalar_v477 { ((if self.scalar_v477 { (self.scalar_v432 * v1760) } else { v1711 }) / v503) } else { v11 })) } else { v11 }));
        let v1829: f64 = 1.0;
        let v1834: f64 = (if self.scalar_v458 { ((if self.scalar_v477 { ((v117 * (self.scalar_v1748 + (((v513 * v1672) + (v449 * v1792)) / self.scalar_v97))) + self.scalar_v1812) } else { v11 }) * v1829) } else { v11 });
        let v1835: f64 = (if self.scalar_v458 { ((if self.scalar_v477 { (v117 * (((v513 * v1673) + (v449 * v1793)) / self.scalar_v97)) } else { v11 }) * v1829) } else { v11 });
        let v1836: f64 = (if self.scalar_v458 { (v1817 * v1829) } else { v11 });
        let v1837: f64 = (if self.scalar_v458 { ((if self.scalar_v477 { (v117 * ((v449 * (if self.scalar_v477 { ((if self.scalar_v477 { v11 } else { v1722 }) / v507) } else { v11 })) / self.scalar_v97)) } else { v11 }) * v1829) } else { v11 });
        let v1841: f64 = (if self.scalar_v458 { ((if self.scalar_v477 { (v116 * (v1740 / self.scalar_v91)) } else { v11 }) * v1829) } else { v11 });
        let v1842: f64 = (if self.scalar_v458 { ((if self.scalar_v477 { ((v116 * ((v1743 / self.scalar_v91) + self.scalar_v1748)) + self.scalar_v1754) } else { v11 }) * v1829) } else { v11 });
        let v1843: f64 = (if self.scalar_v458 { (v1759 * v1829) } else { v11 });
        let v1853: f64 = (if self.scalar_v583 { (v1829 * ((-v525) + (v9 * (if self.scalar_v477 { v11 } else { (if self.scalar_v464 { (v117 * ((v454 * v1672) + (v449 * (self.scalar_v1675 * v1677)))) } else { v11 }) })))) } else { v11 });
        let v1854: f64 = (if self.scalar_v583 { (v1829 * (v9 * (if self.scalar_v477 { v11 } else { (if self.scalar_v464 { (v117 * ((v454 * v1673) + (v449 * (self.scalar_v432 * v1677)))) } else { v11 }) }))) } else { v11 });
        let v1855: f64 = (if self.scalar_v583 { (v1829 * (v525 + (v9 * (if self.scalar_v477 { v11 } else { (if self.scalar_v464 { (v117 * (v449 * (self.scalar_v97 * v1677))) } else { v11 }) })))) } else { v11 });
        let v1865: f64 = (if self.scalar_v583 { (v1829 * (v2 * (if self.scalar_v477 { v11 } else { (if self.scalar_v464 { ((v465 * v1667) + (v442 * (v116 * (self.scalar_v432 * v1660)))) } else { v11 }) }))) } else { v11 });
        let v1866: f64 = (if self.scalar_v583 { (v1829 * ((-v523) + (v2 * (if self.scalar_v477 { v11 } else { (if self.scalar_v464 { ((v465 * v1668) + (v442 * (v116 * (self.scalar_v1658 * v1660)))) } else { v11 }) })))) } else { v11 });
        let v1867: f64 = (if self.scalar_v583 { (v1829 * (v523 + (v2 * (if self.scalar_v477 { v11 } else { (if self.scalar_v464 { (v442 * (v116 * (self.scalar_v91 * v1660))) } else { v11 }) })))) } else { v11 });
        let v1870: f64 = (-v119);
        let v1905: f64 = (self.scalar_v535 * v1829);
        let v1906: f64 = (if self.scalar_v534 { v1905 } else { v11 });
        let v1907: f64 = (if self.scalar_v623 { v1905 } else { v11 });
        let v1926: f64 = (self.scalar_v541 * v1829);
        let v1927: f64 = (if self.scalar_v540 { v1926 } else { v11 });
        let v1928: f64 = (if self.scalar_v642 { v1926 } else { v11 });
        let v1933: f64 = (if self.scalar_v549 { (v655 * v1829) } else { v11 });
        let v1935: f64 = (if self.scalar_v571 { (self.scalar_v659 * v1829) } else { v11 });

        let d575_dn12: f64 = self.scalar_v573;
        stamper.stamp_current_reactive_node1(
            Some(nodes[12]),
            None,
            nodes[12],
            multiplicity * (d575_dn12),
        );
        let d578_db0: f64 = self.scalar_v576;
        stamper.stamp_current_reactive_branch1(
            Some(nodes[12]),
            Some(nodes[13]),
            branches[0],
            multiplicity * (d578_db0),
        );
        let d580_dn3: f64 = v1834;
        let d580_dn5: f64 = v1835;
        let d580_dn7: f64 = v1836;
        let d580_dn8: f64 = v1837;
        let v580_reactive_nodes: [usize; 4] = [nodes[3], nodes[5], nodes[7], nodes[8]];
        let v580_reactive_node_derivatives: [f64; 4] = [d580_dn3, d580_dn5, d580_dn7, d580_dn8];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[3]),
            &v580_reactive_nodes,
            &v580_reactive_node_derivatives,
            &[],
            &[],
            multiplicity,
        );
        let d582_dn3: f64 = v1841;
        let d582_dn5: f64 = v1842;
        let d582_dn8: f64 = v1843;
        stamper.stamp_current_reactive_node3(
            Some(nodes[8]),
            Some(nodes[5]),
            nodes[3],
            multiplicity * (d582_dn3),
            nodes[5],
            multiplicity * (d582_dn5),
            nodes[8],
            multiplicity * (d582_dn8),
        );
        let d586_dn3: f64 = v1853;
        let d586_dn5: f64 = v1854;
        let d586_dn7: f64 = v1855;
        stamper.stamp_current_reactive_node3(
            Some(nodes[7]),
            Some(nodes[3]),
            nodes[3],
            multiplicity * (d586_dn3),
            nodes[5],
            multiplicity * (d586_dn5),
            nodes[7],
            multiplicity * (d586_dn7),
        );
        let d589_dn3: f64 = v1865;
        let d589_dn5: f64 = v1866;
        let d589_dn8: f64 = v1867;
        stamper.stamp_current_reactive_node3(
            Some(nodes[8]),
            Some(nodes[5]),
            nodes[3],
            multiplicity * (d589_dn3),
            nodes[5],
            multiplicity * (d589_dn5),
            nodes[8],
            multiplicity * (d589_dn8),
        );
        let d593_dn1: f64 = self.scalar_v590;
        let d593_dn3: f64 = self.scalar_v1868;
        stamper.stamp_current_reactive_node2(
            Some(nodes[4]),
            Some(nodes[3]),
            nodes[1],
            multiplicity * (d593_dn1),
            nodes[3],
            multiplicity * (d593_dn3),
        );
        let d595_dn3: f64 = self.scalar_v594;
        let d595_dn5: f64 = self.scalar_v1869;
        stamper.stamp_current_reactive_node2(
            Some(nodes[3]),
            Some(nodes[5]),
            nodes[3],
            multiplicity * (d595_dn3),
            nodes[5],
            multiplicity * (d595_dn5),
        );
        let d598_dn3: f64 = v119;
        let d598_dn10: f64 = v1870;
        stamper.stamp_current_reactive_node2(
            Some(nodes[3]),
            Some(nodes[10]),
            nodes[3],
            multiplicity * (d598_dn3),
            nodes[10],
            multiplicity * (d598_dn10),
        );
        let d605_dn8: f64 = self.scalar_v1891;
        let d605_dn9: f64 = self.scalar_v602;
        stamper.stamp_current_reactive_node2(
            Some(nodes[9]),
            Some(nodes[8]),
            nodes[8],
            multiplicity * (d605_dn8),
            nodes[9],
            multiplicity * (d605_dn9),
        );
        let d621_db6: f64 = v1906;
        stamper.stamp_current_reactive_branch1(
            Some(nodes[1]),
            Some(nodes[4]),
            branches[6],
            multiplicity * (d621_db6),
        );
        let d627_db8: f64 = v1907;
        stamper.stamp_current_reactive_branch1(
            Some(nodes[1]),
            Some(nodes[4]),
            branches[8],
            multiplicity * (d627_db8),
        );
        let d633_db13: f64 = self.scalar_v631;
        stamper.stamp_current_reactive_branch1(
            Some(nodes[6]),
            Some(nodes[2]),
            branches[13],
            multiplicity * (d633_db13),
        );
        let d640_db15: f64 = v1927;
        stamper.stamp_current_reactive_branch1(
            Some(nodes[3]),
            Some(nodes[0]),
            branches[15],
            multiplicity * (d640_db15),
        );
        let d646_db17: f64 = v1928;
        stamper.stamp_current_reactive_branch1(
            Some(nodes[3]),
            Some(nodes[0]),
            branches[17],
            multiplicity * (d646_db17),
        );
        let d658_dn14: f64 = v1933;
        stamper.stamp_current_reactive_node1(
            Some(nodes[4]),
            Some(nodes[3]),
            nodes[14],
            multiplicity * (d658_dn14),
        );
        let d662_dn11: f64 = v1935;
        stamper.stamp_current_reactive_node1(
            Some(nodes[11]),
            None,
            nodes[11],
            multiplicity * (d662_dn11),
        );
    }
}
