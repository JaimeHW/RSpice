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
        let v0: f64 = ctx.node_voltage(nodes[12]);
        let v1: f64 = ctx.node_voltage(nodes[8]);
        let v2: f64 = (v0 - v1);
        let v3: f64 = ctx.node_voltage(nodes[10]);
        let v4: f64 = ctx.node_voltage(nodes[5]);
        let v5: f64 = (v3 - v4);
        let v6: f64 = (-v5);
        let v7: f64 = (v4 - v1);
        let v8: f64 = ctx.node_voltage(nodes[11]);
        let v9: f64 = (v8 - v1);
        let v10: f64 = ctx.node_voltage(nodes[4]);
        let v11: f64 = (v10 - v1);
        let v12: f64 = ctx.node_voltage(nodes[16]);
        let v13: f64 = 0.0;
        let v32: f64 = ctx.node_voltage(nodes[3]);
        let v35: f64 = (if (self.scalar_v31 != 0.0) { (self.scalar_v23 + ((v32) as f64).abs()) } else { self.scalar_v23 });
        let v39: f64 = (((v35 - self.scalar_v30)) as f64).abs();
        let v43: bool = ((v39 > v13) || self.scalar_v42);
        let v44: f64 = 1.0;
        let v46: f64 = ((v39) as f64).abs();
        let v49: f64 = (self.scalar_v41 * (v44 + (self.scalar_v45 * v46)));
        let v83: f64 = (v44 + (v46 * self.scalar_v81));
        let v94: f64 = (v44 + (v39 * self.scalar_v92));
        let v118: bool = (v43 && self.scalar_v117);
        let v122: f64 = (v44 + (self.scalar_v81 * (v39 * v39)));
        let v129: bool = (v43 && self.scalar_v128);
        let v134: bool = (!v43);
        let v135: f64 = (if v134 { self.scalar_v50 } else { (if v43 { (self.scalar_v50 * (v44 + (v46 * self.scalar_v51))) } else { v13 }) });
        let v136: f64 = (if v134 { self.scalar_v56 } else { (if v43 { (self.scalar_v56 * (v44 + (v46 * self.scalar_v57))) } else { v13 }) });
        let v137: f64 = (if v134 { self.scalar_v62 } else { (if v43 { (self.scalar_v62 * (v44 + (v46 * self.scalar_v63))) } else { v13 }) });
        let v138: f64 = (if v134 { self.scalar_v68 } else { (if v43 { (self.scalar_v68 * (v44 + (v46 * self.scalar_v69))) } else { v13 }) });
        let v139: f64 = (if v134 { self.scalar_v74 } else { (if v43 { (self.scalar_v74 * (v44 + (v46 * self.scalar_v75))) } else { v13 }) });
        let v140: f64 = (if v134 { self.scalar_v80 } else { (if v43 { (self.scalar_v80 * v83) } else { v13 }) });
        let v141: f64 = (if v134 { self.scalar_v119 } else { (if v129 { (v83 * self.scalar_v119) } else { (if v118 { (self.scalar_v119 * v122) } else { v13 }) }) });
        let v142: f64 = (if v134 { self.scalar_v125 } else { (if v129 { (v83 * self.scalar_v125) } else { (if v118 { (v122 * self.scalar_v125) } else { v13 }) }) });
        let v144: f64 = (if v134 { self.scalar_v91 } else { (if v43 { (self.scalar_v91 * v94) } else { v13 }) });
        let v145: f64 = (if v134 { self.scalar_v97 } else { (if v43 { (v94 * self.scalar_v97) } else { v13 }) });
        let v146: f64 = (if v134 { self.scalar_v100 } else { (if v43 { (self.scalar_v100 + (v39 * self.scalar_v101)) } else { v13 }) });
        let v147: f64 = (if v134 { self.scalar_v105 } else { (if v43 { (self.scalar_v105 + (v39 * self.scalar_v106)) } else { v13 }) });
        let v152: f64 = 0.5;
        let v159: f64 = (if self.scalar_v157 { self.scalar_v158 } else { (if self.scalar_v151 { (self.scalar_v154 / (v35 * 8.617333262145179e-5)) } else { v13 }) });
        let v161: f64 = (v7 * self.scalar_v160);
        let v162: f64 = ((v161) as f64).cosh();
        let v167: f64 = 1e-12;
        let v169: f64 = (v167 + (v162 * v162));
        let v175: f64 = (v44 + (v46 * self.scalar_v173));
        let v176: f64 = ((self.scalar_v165 * (v44 + (self.scalar_v166 / v169))) * v175);
        let v181: f64 = (self.scalar_v177 * (v44 + (v46 * self.scalar_v178)));
        let v186: f64 = (((v7 * self.scalar_v184)) as f64).tanh();
        let v191: f64 = (v6 - v147);
        let v192: f64 = (self.scalar_v190 * v191);
        let v194: f64 = (((((if v134 { self.scalar_v86 } else { (if v43 { (self.scalar_v86 + (v39 * self.scalar_v87)) } else { v13 }) }) - self.scalar_v182) + (self.scalar_v182 * v186)) - (v11 * self.scalar_v163)) - (v191 * v192));
        let v196: f64 = (v44 + (v46 * self.scalar_v87));
        let v197: f64 = (v194 * v196);
        let v198: f64 = (v2 - v197);
        let v199: f64 = (v198 * v198);
        let v204: f64 = (v181 * v198);
        let v206: f64 = (((v176 * v198) + (v199 * self.scalar_v201)) + (v199 * v204));
        let v207: f64 = ((v206) as f64).tanh();
        let v208: f64 = (v44 + v207);
        let v210: f64 = (-v206);
        let v214: f64 = (((v152 * ({ let limexp_arg = v206; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } } - { let limexp_arg = v210; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } }))) as f64).tanh();
        let v218: f64 = (self.scalar_v216 + (self.scalar_v184 * v208));
        let v220: f64 = (((v7 * v218)) as f64).tanh();
        let v222: f64 = 2.0;
        let v226: f64 = (v135 * v208);
        let v227: f64 = (v220 * v226);
        let v233: f64 = ((v44 + (v7 * self.scalar_v228)) + (v136 * { let limexp_arg = v191; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } }));
        let v238: f64 = (v5 - v197);
        let v239: f64 = (if self.scalar_v237 { v238 } else { v162 });
        let v241: f64 = (if self.scalar_v237 { (v239 * v239) } else { v198 });
        let v243: f64 = (if self.scalar_v237 { (v239 * v241) } else { v199 });
        let v249: f64 = (if self.scalar_v237 { (((v176 * v239) + (self.scalar_v201 * v241)) + (v181 * v243)) } else { v13 });
        let v250: f64 = ((v249) as f64).tanh();
        let v252: f64 = (if self.scalar_v237 { (v44 + v250) } else { v13 });
        let v255: f64 = (if self.scalar_v237 { (self.scalar_v216 + (self.scalar_v184 * v252)) } else { v13 });
        let v258: f64 = (self.scalar_v228 + (v208 * self.scalar_v256));
        let v259: f64 = (if self.scalar_v237 { v258 } else { v13 });
        let v260: f64 = (v44 + v220);
        let v261: f64 = (v226 * v260);
        let v266: f64 = (self.scalar_v264 * (v7 - v147));
        let v268: f64 = (v136 * { let limexp_arg = v266; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } });
        let v269: f64 = ((v44 + (v7 * v259)) + v268);
        let v271: f64 = (if self.scalar_v237 { (v261 * v269) } else { v13 });
        let v274: f64 = (if self.scalar_v237 { (self.scalar_v228 + (v252 * self.scalar_v256)) } else { v13 });
        let v276: f64 = (((v7 * v255)) as f64).tanh();
        let v278: f64 = (v135 * v252);
        let v279: f64 = (v44 - (if self.scalar_v237 { v276 } else { v13 }));
        let v280: f64 = (v278 * v279);
        let v282: f64 = (v44 - (v7 * v274));
        let v284: f64 = (if self.scalar_v237 { (v280 * v282) } else { v13 });
        let v291: f64 = (if self.scalar_v290 { v198 } else { v239 });
        let v293: f64 = (if self.scalar_v290 { (v291 * v291) } else { v241 });
        let v296: f64 = (v181 * v293);
        let v298: f64 = ((v291 + (self.scalar_v201 * v293)) + (v291 * v296));
        let v300: f64 = (if self.scalar_v290 { (v176 * v298) } else { v206 });
        let v302: f64 = (-v300);
        let v306: f64 = (((v152 * ({ let limexp_arg = v300; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } } - { let limexp_arg = v302; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } }))) as f64).tanh();
        let v308: f64 = (if self.scalar_v290 { (v44 + v306) } else { (v44 + v214) });
        let v311: f64 = (if self.scalar_v290 { (self.scalar_v216 + (self.scalar_v184 * v308)) } else { v13 });
        let v313: f64 = (((v7 * v311)) as f64).tanh();
        let v314: f64 = (if self.scalar_v290 { v313 } else { v13 });
        let v317: f64 = (if self.scalar_v290 { (self.scalar_v228 + (self.scalar_v256 * v308)) } else { v259 });
        let v318: f64 = (v135 * v308);
        let v319: f64 = (v314 * v318);
        let v322: f64 = (v191 * self.scalar_v264);
        let v325: f64 = ((v44 + (v7 * v317)) + (v136 * { let limexp_arg = v322; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } }));
        let v331: f64 = (if self.scalar_v330 { v198 } else { v291 });
        let v333: f64 = (if self.scalar_v330 { (v331 * v331) } else { v293 });
        let v336: f64 = (v181 * v333);
        let v338: f64 = ((v331 + (self.scalar_v201 * v333)) + (v331 * v336));
        let v340: f64 = (if self.scalar_v330 { (v176 * v338) } else { v300 });
        let v341: f64 = (if self.scalar_v330 { v238 } else { v243 });
        let v343: f64 = (if self.scalar_v330 { (v341 * v341) } else { v13 });
        let v346: f64 = (v181 * v341);
        let v348: f64 = ((v341 + (self.scalar_v201 * v343)) + (v343 * v346));
        let v350: f64 = (if self.scalar_v330 { (v176 * v348) } else { v249 });
        let v352: f64 = (-v340);
        let v356: f64 = (((v152 * ({ let limexp_arg = v340; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } } - { let limexp_arg = v352; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } }))) as f64).tanh();
        let v358: f64 = (if self.scalar_v330 { (v44 + v356) } else { v308 });
        let v360: f64 = (-v350);
        let v364: f64 = (((v152 * ({ let limexp_arg = v350; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } } - { let limexp_arg = v360; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } }))) as f64).tanh();
        let v366: f64 = (if self.scalar_v330 { (v44 + v364) } else { v13 });
        let v368: f64 = (self.scalar_v216 + (self.scalar_v184 * v358));
        let v369: f64 = (if self.scalar_v330 { v368 } else { v311 });
        let v372: f64 = (if self.scalar_v330 { (self.scalar_v216 + (self.scalar_v184 * v366)) } else { v13 });
        let v374: f64 = (((v7 * v369)) as f64).tanh();
        let v375: f64 = (if self.scalar_v330 { v374 } else { v314 });
        let v377: f64 = (((v7 * v372)) as f64).tanh();
        let v381: f64 = (if self.scalar_v330 { (self.scalar_v228 + (self.scalar_v256 * v366)) } else { v13 });
        let v384: f64 = (if self.scalar_v330 { (self.scalar_v228 + (self.scalar_v256 * v358)) } else { v13 });
        let v385: f64 = (v135 * v358);
        let v386: f64 = (v44 + v375);
        let v387: f64 = (v385 * v386);
        let v390: f64 = (v268 + (v44 + (v7 * v384)));
        let v393: f64 = (v135 * v366);
        let v394: f64 = (v44 - (if self.scalar_v330 { v377 } else { v13 }));
        let v395: f64 = (v393 * v394);
        let v397: f64 = (v44 - (v7 * v381));
        let v402: f64 = (if self.scalar_v330 { (v152 * ((if self.scalar_v330 { (v387 * v390) } else { v271 }) - (if self.scalar_v330 { (v395 * v397) } else { v284 }))) } else { (if self.scalar_v290 { (v319 * v325) } else { (if self.scalar_v237 { (v152 * (v271 - v284)) } else { (if self.scalar_v221 { (v227 * v233) } else { v13 }) }) }) });
        let v406: f64 = (if self.scalar_v405 { v258 } else { v317 });
        let v407: f64 = (if self.scalar_v405 { v368 } else { v369 });
        let v409: f64 = (((v7 * v407)) as f64).tanh();
        let v412: f64 = (((v11 * v407)) as f64).tanh();
        let v416: f64 = ((if self.scalar_v405 { v409 } else { v375 }) + ((if self.scalar_v405 { v412 } else { v13 }) * self.scalar_v414));
        let v417: f64 = (v226 * v416);
        let v419: f64 = (v7 + (v11 * self.scalar_v414));
        let v422: f64 = (v268 + (v44 + (v406 * v419)));
        let v424: f64 = (if self.scalar_v405 { (v417 * v422) } else { v402 });
        let v427: f64 = (v44 + v208);
        let v433: f64 = (v208 * self.scalar_v432);
        let v440: f64 = (v44 + v358);
        let v443: f64 = (if self.scalar_v439 { (self.scalar_v426 + (v139 / v440)) } else { (if self.scalar_v425 { (self.scalar_v426 + (v139 / v427)) } else { v13 }) });
        let v444: f64 = (v358 * self.scalar_v432);
        let v451: f64 = (v44 + (v46 * self.scalar_v449));
        let v452: f64 = ((if self.scalar_v439 { (self.scalar_v436 + v444) } else { (if self.scalar_v425 { (v433 + self.scalar_v436) } else { v13 }) }) * v451);
        let v453: f64 = ((if self.scalar_v439 { (self.scalar_v431 + v444) } else { (if self.scalar_v425 { (self.scalar_v431 + v433) } else { v13 }) }) * v451);
        let v456: f64 = -1.0;
        let v462: f64 = (v9 - v146);
        let v466: f64 = ((-v9) - self.scalar_v465);
        let v468: f64 = (v5 - v146);
        let v471: f64 = (v6 - self.scalar_v470);
        let v477: f64 = (if self.scalar_v473 { { let limexp_arg = (v146 * (-v159)); if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } } } else { (if self.scalar_v455 { { let limexp_arg = (v159 * (((-v146)) as f64).tanh()); if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } } } else { v331 }) });
        let v488: f64 = ((v462) as f64).tanh();
        let v490: f64 = ((v468) as f64).tanh();
        let v498: f64 = (self.scalar_v478 * (if self.scalar_v473 { v466 } else { (if self.scalar_v455 { v466 } else { v13 }) }));
        let v502: f64 = (v159 * (if self.scalar_v493 { v462 } else { (if self.scalar_v487 { v488 } else { (if self.scalar_v455 { v462 } else { v13 }) }) }));
        let v510: f64 = (self.scalar_v501 * (({ let limexp_arg = v502; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } } - (({ let limexp_arg = v498; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } } - self.scalar_v482) * self.scalar_v506)) - v477));
        let v511: f64 = (self.scalar_v478 * (if self.scalar_v473 { v471 } else { (if self.scalar_v455 { v471 } else { v13 }) }));
        let v514: f64 = (v159 * (if self.scalar_v493 { v468 } else { (if self.scalar_v487 { v490 } else { (if self.scalar_v455 { v468 } else { v13 }) }) }));
        let v519: f64 = (self.scalar_v501 * (({ let limexp_arg = v514; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } } - (self.scalar_v506 * ({ let limexp_arg = v511; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } } - self.scalar_v485))) - v477));
        let v524: f64 = (v7 * self.scalar_v523);
        let v525: f64 = ((v144 + (v9 * self.scalar_v520)) + v524);
        let v526: f64 = ((v525) as f64).tanh();
        let v527: f64 = (v44 + v526);
        let v532: f64 = (((self.scalar_v528 + (v7 * self.scalar_v529))) as f64).tanh();
        let v533: f64 = (v44 + v532);
        let v538: f64 = (((self.scalar_v534 - (v7 * self.scalar_v535))) as f64).tanh();
        let v540: f64 = ((v44 + v538) - self.scalar_v523);
        let v544: f64 = ((v145 + (v5 * self.scalar_v541)) - v524);
        let v545: f64 = ((v544) as f64).tanh();
        let v546: f64 = (v44 + v545);
        let v557: f64 = (v137 * v527);
        let v571: f64 = (if self.scalar_v569 { (v533 - self.scalar_v523) } else { v533 });
        let v572: f64 = (v144 + v524);
        let v573: f64 = ((v572) as f64).cosh();
        let v574: f64 = (if self.scalar_v569 { v573 } else { v13 });
        let v576: f64 = (if self.scalar_v569 { ((v574) as f64).ln() } else { v13 });
        let v577: f64 = ((v525) as f64).cosh();
        let v578: f64 = (if self.scalar_v569 { v577 } else { v13 });
        let v580: f64 = (if self.scalar_v569 { ((v578) as f64).ln() } else { v13 });
        let v582: f64 = (if self.scalar_v569 { (v572 + v576) } else { v13 });
        let v584: f64 = ((v525 + v580) - v582);
        let v587: f64 = (v9 * self.scalar_v562);
        let v590: f64 = (v9 * self.scalar_v551);
        let v593: f64 = (v145 - v524);
        let v594: f64 = ((v593) as f64).cosh();
        let v595: f64 = (if self.scalar_v569 { v594 } else { v574 });
        let v597: f64 = (if self.scalar_v569 { ((v595) as f64).ln() } else { v13 });
        let v598: f64 = ((v544) as f64).cosh();
        let v599: f64 = (if self.scalar_v569 { v598 } else { v578 });
        let v601: f64 = (if self.scalar_v569 { ((v599) as f64).ln() } else { v13 });
        let v603: f64 = (if self.scalar_v569 { (v593 + v597) } else { v13 });
        let v605: f64 = ((v544 + v601) - v603);
        let v608: f64 = (v5 * self.scalar_v562);
        let v611: f64 = (v5 * self.scalar_v553);
        let v2250: f64 = ((v525) as f64).sinh();
        let v2253: f64 = (self.scalar_v520 * v2250);
        let v2256: f64 = (if self.scalar_v569 { v2253 } else { v13 });
        let v2262: f64 = (if self.scalar_v569 { (v2256 / v578) } else { v13 });
        let v2293: f64 = (if self.scalar_v569 { (self.scalar_v551 + (v137 * (self.scalar_v562 + ((v571 * (self.scalar_v520 + v2262)) / self.scalar_v520)))) } else { v13 });
        let v614: f64 = v2293;
        let v2303: f64 = ((v544) as f64).sinh();
        let v2306: f64 = (self.scalar_v541 * v2303);
        let v2309: f64 = (if self.scalar_v569 { v2306 } else { v13 });
        let v2317: f64 = (if self.scalar_v569 { (v2309 / v599) } else { v13 });
        let v2351: f64 = (if self.scalar_v569 { (self.scalar_v553 + (v138 * (self.scalar_v562 + ((v540 * (self.scalar_v541 + v2317)) / self.scalar_v541)))) } else { v13 });
        let v616: f64 = v2351;
        let v622: f64 = (v9 / self.scalar_v621);
        let v624: f64 = (if self.scalar_v620 { (v622 - v44) } else { v13 });
        let v627: f64 = (v624 * v624);
        let v628: f64 = (self.scalar_v626 + v627);
        let v630: f64 = f64::powf(v628, self.scalar_v629);
        let v634: f64 = (self.scalar_v626 + (v627 * self.scalar_v632));
        let v640: f64 = (((v144 + (self.scalar_v520 * (v9 + v524)))) as f64).tanh();
        let v643: f64 = (if self.scalar_v620 { v533 } else { v571 });
        let v645: f64 = (v538 + self.scalar_v644);
        let v646: f64 = (if self.scalar_v620 { v645 } else { v540 });
        let v651: f64 = (((v145 + (self.scalar_v541 * (v5 + (v7 * self.scalar_v644))))) as f64).tanh();
        let v653: f64 = (if self.scalar_v620 { (v44 + v651) } else { v546 });
        let v657: f64 = (v137 * ((if self.scalar_v620 { (v44 + v640) } else { v527 }) + ((if self.scalar_v620 { (v630 * v634) } else { v13 }) * self.scalar_v654)));
        let v665: f64 = (if self.scalar_v620 { (self.scalar_v553 + (v138 * (self.scalar_v562 + (v646 * v653)))) } else { (if self.scalar_v569 { v616 } else { (if self.scalar_v556 { (self.scalar_v553 + (v138 * ((v540 * v546) + self.scalar_v562))) } else { self.scalar_v554 }) }) });
        let v669: f64 = (if self.scalar_v668 { v573 } else { v595 });
        let v672: f64 = (if self.scalar_v668 { v577 } else { v599 });
        let v677: f64 = (self.scalar_v654 * (v9 + self.scalar_v621));
        let v678: f64 = (v456 + v622);
        let v680: f64 = (self.scalar_v626 + f64::powf(v678, v222));
        let v682: f64 = f64::powf(v680, self.scalar_v681);
        let v694: f64 = ((if self.scalar_v668 { (v677 * v682) } else { v13 }) + ((v525 + (if self.scalar_v668 { ((v672) as f64).ln() } else { v580 })) - (if self.scalar_v668 { (v572 + (if self.scalar_v668 { ((v669) as f64).ln() } else { v576 })) } else { v582 })));
        let v695: f64 = (v694 - self.scalar_v689);
        let v696: f64 = (v532 + self.scalar_v644);
        let v702: f64 = (if self.scalar_v668 { (v590 + (v137 * (v587 + ((v695 * v696) / self.scalar_v520)))) } else { (if self.scalar_v569 { ((v137 * (((v571 * v584) / self.scalar_v520) + v587)) + v590) } else { v13 }) });
        let v703: f64 = (if self.scalar_v668 { v594 } else { v669 });
        let v706: f64 = (if self.scalar_v668 { v598 } else { v672 });
        let v712: f64 = ((v544 + (if self.scalar_v668 { ((v706) as f64).ln() } else { v601 })) - (if self.scalar_v668 { (v593 + (if self.scalar_v668 { ((v703) as f64).ln() } else { v597 })) } else { v603 }));
        let v718: f64 = (if self.scalar_v668 { (v611 + (v138 * (v608 + ((v645 * v712) / self.scalar_v541)))) } else { (if self.scalar_v569 { ((v138 * (((v540 * v605) / self.scalar_v541) + v608)) + v611) } else { v13 }) });
        let v2455: f64 = (v222 * f64::powf(v678, v44));
        let v2460: f64 = (self.scalar_v681 * f64::powf(v680, self.scalar_v2458));
        let v2310: f64 = (if self.scalar_v569 { v13 } else { v2256 });
        let v2444: f64 = (if self.scalar_v668 { v2253 } else { v2310 });
        let v2489: f64 = (v696 * ((if self.scalar_v668 { ((self.scalar_v654 * v682) + (v677 * ((self.scalar_v2360 * v2455) * v2460))) } else { v13 }) + (self.scalar_v520 + (if self.scalar_v668 { (v2444 / v672) } else { v2262 }))));
        let v2505: f64 = (if self.scalar_v668 { (self.scalar_v551 + (v137 * (self.scalar_v562 + (v2489 / self.scalar_v520)))) } else { v2293 });
        let v719: f64 = v2505;
        let v720: f64 = (if self.scalar_v668 { v719 } else { (if self.scalar_v620 { (self.scalar_v551 + (v643 * v657)) } else { (if self.scalar_v569 { v614 } else { (if self.scalar_v556 { (self.scalar_v551 + (v533 * v557)) } else { self.scalar_v552 }) }) }) });
        let v2443: f64 = (if self.scalar_v668 { v13 } else { v2309 });
        let v2555: f64 = (if self.scalar_v668 { (self.scalar_v553 + (v138 * (self.scalar_v562 + ((v645 * (self.scalar_v541 + (if self.scalar_v668 { ((if self.scalar_v668 { v2306 } else { v2443 }) / v706) } else { v2317 }))) / self.scalar_v541)))) } else { v2351 });
        let v721: f64 = v2555;
        let v722: f64 = (if self.scalar_v668 { v721 } else { v665 });
        let v758: f64 = (if self.scalar_v747 { ((v137 * ((v35 * 5.5226012e-23) * self.scalar_v752)) * self.scalar_v756) } else { v13 });
        let v762: f64 = (if self.scalar_v747 { (((v44 - (v758 * v758))) as f64).sqrt() } else { v13 });
        let v764: f64 = 3.141592653589793;
        let v766: f64 = (if self.scalar_v747 { ((-v758) * v764) } else { v13 });
        let v770: f64 = (-v424);
        let v773: f64 = (self.scalar_v771 * ctx.node_voltage(nodes[15]));
        let v776: f64 = (self.scalar_v774 * ctx.branch_current(branches[0]));
        let v777: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, v718);
        let v778: f64 = (if self.scalar_v723 { v777 } else { v13 });
        let v779: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, v702);
        let v780: f64 = (if self.scalar_v723 { v779 } else { v13 });
        let v783: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 5, (v5 * v722));
        let v784: f64 = (if self.scalar_v781 { v783 } else { v13 });
        let v786: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 6, (v9 * v720));
        let v787: f64 = (if self.scalar_v781 { v786 } else { v13 });
        let v791: f64 = (self.scalar_v788 * (ctx.node_voltage(nodes[7]) - v4));
        let v793: f64 = (v7 * self.scalar_v792);
        let v795: f64 = (ctx.node_voltage(nodes[6]) - v10);
        let v796: f64 = (v140 * v795);
        let v797: f64 = (v167 * v795);
        let v798: f64 = ctx.branch_current(branches[1]);
        let v801: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, (self.scalar_v724 * v798));
        let v803: f64 = (if self.scalar_v725 { ((v443 * v798) + v801) } else { v13 });
        let v806: f64 = (if self.scalar_v728 { ((v8 - v0) / v141) } else { v13 });
        let v808: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 10, (v2 * v142));
        let v809: f64 = (if self.scalar_v728 { v808 } else { v13 });
        let v811: f64 = ctx.node_voltage(nodes[14]);
        let v813: f64 = (self.scalar_v810 * (v8 - v811));
        let v816: f64 = (if self.scalar_v730 { ((v811 - v1) / self.scalar_v729) } else { v13 });
        let v817: f64 = ctx.node_voltage(nodes[13]);
        let v820: f64 = (if self.scalar_v732 { ((v817 - v3) / self.scalar_v731) } else { v13 });
        let v823: f64 = (if self.scalar_v734 { ((v817 - v8) / self.scalar_v733) } else { v13 });
        let v826: f64 = (if self.scalar_v736 { (self.scalar_v735 * ctx.branch_current(branches[7])) } else { v13 });
        let v829: f64 = (self.scalar_v827 * ctx.branch_current(branches[10]));
        let v830: f64 = ctx.branch_current(branches[11]);
        let v832: f64 = (if self.scalar_v737 { (v452 * v830) } else { v13 });
        let v835: f64 = (self.scalar_v833 * ctx.branch_current(branches[14]));
        let v836: f64 = ctx.branch_current(branches[15]);
        let v838: f64 = (if self.scalar_v740 { (v453 * v836) } else { v13 });
        let v841: f64 = (self.scalar_v839 * ctx.branch_current(branches[18]));
        let v842: f64 = 1e-15;
        let v845: f64 = (v167 * (v0 - ctx.node_voltage(nodes[2])));
        let v846: f64 = ctx.node_voltage(nodes[17]);
        let v847: f64 = (if self.scalar_v747 { v846 } else { v13 });
        let v848: f64 = ctx.node_voltage(nodes[18]);
        let v849: f64 = (if self.scalar_v747 { v848 } else { v13 });
        let v853: f64 = (if self.scalar_v747 { ((v766 * v846) + (v762 * v848)) } else { v13 });
        let v854: f64 = (-(if self.scalar_v747 { (v758 * v764) } else { v13 }));
        let v856: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 15, (v846 * v854));
        let v857: f64 = (if self.scalar_v747 { v856 } else { v13 });
        let v864: f64 = (if self.scalar_v769 { (-((((v7 * v424)) as f64).abs() + (((v9 * v510)) as f64).abs())) } else { v13 });
        let v866: f64 = (if self.scalar_v769 { (v32 / v49) } else { v13 });
        let v869: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 16, (v32 * self.scalar_v867));
        let v870: f64 = (if self.scalar_v769 { v869 } else { v13 });
        let v873: f64 = (if self.scalar_v871 { (v32 * v167) } else { v13 });
        let v875: f64 = ((v161) as f64).sinh();
        let v876: f64 = (self.scalar_v160 * v875);
        let v877: f64 = (self.scalar_v874 * v875);
        let v879: f64 = (v162 * v876);
        let v881: f64 = (v162 * v877);
        let v885: f64 = (v169 * v169);
        let v892: f64 = (v175 * (self.scalar_v165 * ((-(self.scalar_v166 * (v879 + v879))) / v885)));
        let v893: f64 = (v175 * (self.scalar_v165 * ((-(self.scalar_v166 * (v881 + v881))) / v885)));
        let v896: f64 = (v44 - (v186 * v186));
        let v910: f64 = (v196 * ((self.scalar_v182 * (self.scalar_v184 * v896)) - (v192 + v192)));
        let v911: f64 = (v196 * ((self.scalar_v182 * (self.scalar_v894 * v896)) - self.scalar_v878));
        let v912: f64 = (v196 * (-((-v192) + (v191 * self.scalar_v902))));
        let v913: f64 = (-(v196 * self.scalar_v878));
        let v914: f64 = (-v910);
        let v915: f64 = (v456 - v911);
        let v916: f64 = (-v912);
        let v917: f64 = (v198 * v913);
        let v918: f64 = (v917 + v917);
        let v919: f64 = (v198 * v914);
        let v920: f64 = (v919 + v919);
        let v921: f64 = (v198 * v915);
        let v922: f64 = (v921 + v921);
        let v923: f64 = (v198 * v916);
        let v924: f64 = (v923 + v923);
        let v925: f64 = (v198 + v198);
        let v963: f64 = (((v176 * v913) + (self.scalar_v201 * v918)) + ((v204 * v918) + (v199 * (v181 * v913))));
        let v964: f64 = ((((v198 * v892) + (v176 * v914)) + (self.scalar_v201 * v920)) + ((v204 * v920) + (v199 * (v181 * v914))));
        let v965: f64 = ((((v198 * v893) + (v176 * v915)) + (self.scalar_v201 * v922)) + ((v204 * v922) + (v199 * (v181 * v915))));
        let v966: f64 = (((v176 * v916) + (self.scalar_v201 * v924)) + ((v204 * v924) + (v199 * (v181 * v916))));
        let v967: f64 = ((v176 + (self.scalar_v201 * v925)) + ((v204 * v925) + (v181 * v199)));
        let v969: f64 = (v44 - (v207 * v207));
        let v970: f64 = (v963 * v969);
        let v971: f64 = (v964 * v969);
        let v972: f64 = (v965 * v969);
        let v973: f64 = (v966 * v969);
        let v974: f64 = (v967 * v969);
        let v975: f64 = { let limexp_arg = v206; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v986: f64 = { let limexp_arg = v210; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v1003: f64 = (v44 - (v214 * v214));
        let v1023: f64 = (v44 - (v220 * v220));
        let v1029: f64 = (v135 * v970);
        let v1030: f64 = (v135 * v971);
        let v1031: f64 = (v135 * v972);
        let v1032: f64 = (v135 * v973);
        let v1033: f64 = (v135 * v974);
        let v1034: f64 = (v226 * ((v7 * (self.scalar_v184 * v970)) * v1023));
        let v1037: f64 = (v226 * ((v218 + (v7 * (self.scalar_v184 * v971))) * v1023));
        let v1040: f64 = (v226 * (((-v218) + (v7 * (self.scalar_v184 * v972))) * v1023));
        let v1043: f64 = (v226 * ((v7 * (self.scalar_v184 * v973)) * v1023));
        let v1046: f64 = (v226 * ((v7 * (self.scalar_v184 * v974)) * v1023));
        let v1050: f64 = { let limexp_arg = v191; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v1071: f64 = (v456 - v910);
        let v1072: f64 = (-v911);
        let v1073: f64 = (v44 - v912);
        let v1074: f64 = (if self.scalar_v237 { v913 } else { v13 });
        let v1075: f64 = (if self.scalar_v237 { v1071 } else { v876 });
        let v1076: f64 = (if self.scalar_v237 { v1072 } else { v877 });
        let v1077: f64 = (if self.scalar_v237 { v1073 } else { v13 });
        let v1078: f64 = (v239 * v1074);
        let v1080: f64 = (v239 * v1075);
        let v1082: f64 = (v239 * v1076);
        let v1084: f64 = (v239 * v1077);
        let v1086: f64 = (if self.scalar_v237 { (v1078 + v1078) } else { v913 });
        let v1087: f64 = (if self.scalar_v237 { (v1080 + v1080) } else { v914 });
        let v1088: f64 = (if self.scalar_v237 { (v1082 + v1082) } else { v915 });
        let v1089: f64 = (if self.scalar_v237 { (v1084 + v1084) } else { v916 });
        let v1104: f64 = (if self.scalar_v237 { ((v241 * v1074) + (v239 * v1086)) } else { v918 });
        let v1105: f64 = (if self.scalar_v237 { ((v241 * v1075) + (v239 * v1087)) } else { v920 });
        let v1106: f64 = (if self.scalar_v237 { ((v241 * v1076) + (v239 * v1088)) } else { v922 });
        let v1107: f64 = (if self.scalar_v237 { ((v241 * v1077) + (v239 * v1089)) } else { v924 });
        let v1108: f64 = (if self.scalar_v237 { (v239 * self.scalar_v1090) } else { v925 });
        let v1136: f64 = (if self.scalar_v237 { (((v176 * v1074) + (self.scalar_v201 * v1086)) + (v181 * v1104)) } else { v13 });
        let v1137: f64 = (if self.scalar_v237 { ((((v239 * v892) + (v176 * v1075)) + (self.scalar_v201 * v1087)) + (v181 * v1105)) } else { v13 });
        let v1138: f64 = (if self.scalar_v237 { ((((v239 * v893) + (v176 * v1076)) + (self.scalar_v201 * v1088)) + (v181 * v1106)) } else { v13 });
        let v1139: f64 = (if self.scalar_v237 { (((v176 * v1077) + (self.scalar_v201 * v1089)) + (v181 * v1107)) } else { v13 });
        let v1140: f64 = (if self.scalar_v237 { (self.scalar_v1121 + (v181 * v1108)) } else { v13 });
        let v1142: f64 = (v44 - (v250 * v250));
        let v1148: f64 = (if self.scalar_v237 { (v1136 * v1142) } else { v13 });
        let v1149: f64 = (if self.scalar_v237 { (v1137 * v1142) } else { v13 });
        let v1150: f64 = (if self.scalar_v237 { (v1138 * v1142) } else { v13 });
        let v1151: f64 = (if self.scalar_v237 { (v1139 * v1142) } else { v13 });
        let v1152: f64 = (if self.scalar_v237 { (v1140 * v1142) } else { v13 });
        let v1163: f64 = (self.scalar_v256 * v970);
        let v1164: f64 = (self.scalar_v256 * v971);
        let v1165: f64 = (self.scalar_v256 * v972);
        let v1166: f64 = (self.scalar_v256 * v973);
        let v1167: f64 = (self.scalar_v256 * v974);
        let v1168: f64 = (if self.scalar_v237 { v1163 } else { v13 });
        let v1169: f64 = (if self.scalar_v237 { v1164 } else { v13 });
        let v1170: f64 = (if self.scalar_v237 { v1165 } else { v13 });
        let v1171: f64 = (if self.scalar_v237 { v1166 } else { v13 });
        let v1172: f64 = (if self.scalar_v237 { v1167 } else { v13 });
        let v1192: f64 = { let limexp_arg = v266; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v1195: f64 = (v136 * (self.scalar_v264 * v1192));
        let v1196: f64 = (v136 * (self.scalar_v1191 * v1192));
        let v1214: f64 = (if self.scalar_v237 { ((v269 * (v1034 + (v260 * v1029))) + (v261 * (v7 * v1168))) } else { v13 });
        let v1215: f64 = (if self.scalar_v237 { ((v269 * (v1037 + (v260 * v1030))) + (v261 * ((v259 + (v7 * v1169)) + v1195))) } else { v13 });
        let v1216: f64 = (if self.scalar_v237 { ((v269 * (v1040 + (v260 * v1031))) + (v261 * (((-v259) + (v7 * v1170)) + v1196))) } else { v13 });
        let v1217: f64 = (if self.scalar_v237 { ((v269 * (v1043 + (v260 * v1032))) + (v261 * (v7 * v1171))) } else { v13 });
        let v1218: f64 = (if self.scalar_v237 { ((v269 * (v1046 + (v260 * v1033))) + (v261 * (v7 * v1172))) } else { v13 });
        let v1238: f64 = (v44 - (v276 * v276));
        let v1267: f64 = ((v279 * (v135 * v1150)) + (v278 * (-(if self.scalar_v237 { (((-v255) + (v7 * (if self.scalar_v237 { (self.scalar_v184 * v1150) } else { v13 }))) * v1238) } else { v13 }))));
        let v1289: f64 = ((v282 * ((v279 * (v135 * v1148)) + (v278 * (-(if self.scalar_v237 { ((v7 * (if self.scalar_v237 { (self.scalar_v184 * v1148) } else { v13 })) * v1238) } else { v13 }))))) + (v280 * (-(v7 * (if self.scalar_v237 { (self.scalar_v256 * v1148) } else { v13 })))));
        let v1290: f64 = (v282 * ((v279 * (v135 * v1149)) + (v278 * (-(if self.scalar_v237 { ((v255 + (v7 * (if self.scalar_v237 { (self.scalar_v184 * v1149) } else { v13 }))) * v1238) } else { v13 })))));
        let v1298: f64 = ((v282 * ((v279 * (v135 * v1151)) + (v278 * (-(if self.scalar_v237 { ((v7 * (if self.scalar_v237 { (self.scalar_v184 * v1151) } else { v13 })) * v1238) } else { v13 }))))) + (v280 * (-(v7 * (if self.scalar_v237 { (self.scalar_v256 * v1151) } else { v13 })))));
        let v1301: f64 = ((v282 * ((v279 * (v135 * v1152)) + (v278 * (-(if self.scalar_v237 { ((v7 * (if self.scalar_v237 { (self.scalar_v184 * v1152) } else { v13 })) * v1238) } else { v13 }))))) + (v280 * (-(v7 * (if self.scalar_v237 { (self.scalar_v256 * v1152) } else { v13 })))));
        let v1302: f64 = (if self.scalar_v237 { v1289 } else { v13 });
        let v1303: f64 = (if self.scalar_v237 { (v1290 + (v280 * (-(v274 + (v7 * (if self.scalar_v237 { (self.scalar_v256 * v1149) } else { v13 })))))) } else { v13 });
        let v1304: f64 = (if self.scalar_v237 { ((v282 * v1267) + (v280 * (-((-v274) + (v7 * (if self.scalar_v237 { (self.scalar_v256 * v1150) } else { v13 })))))) } else { v13 });
        let v1305: f64 = (if self.scalar_v237 { v1298 } else { v13 });
        let v1306: f64 = (if self.scalar_v237 { v1301 } else { v13 });
        let v1318: f64 = (if self.scalar_v237 { (v152 * (v1215 - v1303)) } else { (if self.scalar_v221 { ((v233 * (v1037 + (v220 * v1030))) + (v227 * (self.scalar_v228 + (v136 * v1050)))) } else { v13 }) });
        let v1322: f64 = (if self.scalar_v290 { v913 } else { v1074 });
        let v1323: f64 = (if self.scalar_v290 { v914 } else { v1075 });
        let v1324: f64 = (if self.scalar_v290 { v915 } else { v1076 });
        let v1325: f64 = (if self.scalar_v290 { v916 } else { v1077 });
        let v1327: f64 = (v291 * v1322);
        let v1329: f64 = (v291 * v1323);
        let v1331: f64 = (v291 * v1324);
        let v1333: f64 = (v291 * v1325);
        let v1335: f64 = (v291 * self.scalar_v1326);
        let v1337: f64 = (if self.scalar_v290 { (v1327 + v1327) } else { v1086 });
        let v1338: f64 = (if self.scalar_v290 { (v1329 + v1329) } else { v1087 });
        let v1339: f64 = (if self.scalar_v290 { (v1331 + v1331) } else { v1088 });
        let v1340: f64 = (if self.scalar_v290 { (v1333 + v1333) } else { v1089 });
        let v1341: f64 = (if self.scalar_v290 { (v1335 + v1335) } else { self.scalar_v1090 });
        let v1386: f64 = (if self.scalar_v290 { (v176 * ((v1322 + (self.scalar_v201 * v1337)) + ((v296 * v1322) + (v291 * (v181 * v1337))))) } else { v963 });
        let v1387: f64 = (if self.scalar_v290 { ((v298 * v892) + (v176 * ((v1323 + (self.scalar_v201 * v1338)) + ((v296 * v1323) + (v291 * (v181 * v1338)))))) } else { v964 });
        let v1388: f64 = (if self.scalar_v290 { ((v298 * v893) + (v176 * ((v1324 + (self.scalar_v201 * v1339)) + ((v296 * v1324) + (v291 * (v181 * v1339)))))) } else { v965 });
        let v1389: f64 = (if self.scalar_v290 { (v176 * ((v1325 + (self.scalar_v201 * v1340)) + ((v296 * v1325) + (v291 * (v181 * v1340))))) } else { v966 });
        let v1390: f64 = (if self.scalar_v290 { (v176 * ((self.scalar_v1326 + (self.scalar_v201 * v1341)) + ((v296 * self.scalar_v1326) + (v291 * (v181 * v1341))))) } else { v967 });
        let v1391: f64 = { let limexp_arg = v300; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v1402: f64 = { let limexp_arg = v302; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v1419: f64 = (v44 - (v306 * v306));
        let v1425: f64 = (if self.scalar_v290 { ((v152 * ((v1386 * v1391) - ((-v1386) * v1402))) * v1419) } else { ((v152 * ((v963 * v975) - ((-v963) * v986))) * v1003) });
        let v1426: f64 = (if self.scalar_v290 { ((v152 * ((v1387 * v1391) - ((-v1387) * v1402))) * v1419) } else { ((v152 * ((v964 * v975) - ((-v964) * v986))) * v1003) });
        let v1427: f64 = (if self.scalar_v290 { ((v152 * ((v1388 * v1391) - ((-v1388) * v1402))) * v1419) } else { ((v152 * ((v965 * v975) - ((-v965) * v986))) * v1003) });
        let v1428: f64 = (if self.scalar_v290 { ((v152 * ((v1389 * v1391) - ((-v1389) * v1402))) * v1419) } else { ((v152 * ((v966 * v975) - ((-v966) * v986))) * v1003) });
        let v1429: f64 = (if self.scalar_v290 { ((v152 * ((v1390 * v1391) - ((-v1390) * v1402))) * v1419) } else { ((v152 * ((v967 * v975) - ((-v967) * v986))) * v1003) });
        let v1435: f64 = (if self.scalar_v290 { (self.scalar_v184 * v1425) } else { v13 });
        let v1436: f64 = (if self.scalar_v290 { (self.scalar_v184 * v1426) } else { v13 });
        let v1437: f64 = (if self.scalar_v290 { (self.scalar_v184 * v1427) } else { v13 });
        let v1438: f64 = (if self.scalar_v290 { (self.scalar_v184 * v1428) } else { v13 });
        let v1439: f64 = (if self.scalar_v290 { (self.scalar_v184 * v1429) } else { v13 });
        let v1449: f64 = (v44 - (v313 * v313));
        let v1455: f64 = (if self.scalar_v290 { ((v7 * v1435) * v1449) } else { v13 });
        let v1456: f64 = (if self.scalar_v290 { ((v311 + (v7 * v1436)) * v1449) } else { v13 });
        let v1457: f64 = (if self.scalar_v290 { (((-v311) + (v7 * v1437)) * v1449) } else { v13 });
        let v1458: f64 = (if self.scalar_v290 { ((v7 * v1438) * v1449) } else { v13 });
        let v1459: f64 = (if self.scalar_v290 { ((v7 * v1439) * v1449) } else { v13 });
        let v1465: f64 = (if self.scalar_v290 { (self.scalar_v256 * v1425) } else { v1168 });
        let v1466: f64 = (if self.scalar_v290 { (self.scalar_v256 * v1426) } else { v1169 });
        let v1467: f64 = (if self.scalar_v290 { (self.scalar_v256 * v1427) } else { v1170 });
        let v1468: f64 = (if self.scalar_v290 { (self.scalar_v256 * v1428) } else { v1171 });
        let v1469: f64 = (if self.scalar_v290 { (self.scalar_v256 * v1429) } else { v1172 });
        let v1498: f64 = { let limexp_arg = v322; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v1510: f64 = ((v325 * ((v318 * v1456) + (v314 * (v135 * v1426)))) + (v319 * ((v317 + (v7 * v1466)) + (v136 * (self.scalar_v264 * v1498)))));
        let v1520: f64 = (if self.scalar_v290 { ((v325 * ((v318 * v1455) + (v314 * (v135 * v1425)))) + (v319 * (v7 * v1465))) } else { (if self.scalar_v237 { (v152 * (v1214 - v1302)) } else { (if self.scalar_v221 { (v233 * (v1034 + (v220 * v1029))) } else { v13 }) }) });
        let v1522: f64 = (if self.scalar_v290 { ((v325 * ((v318 * v1457) + (v314 * (v135 * v1427)))) + (v319 * ((-v317) + (v7 * v1467)))) } else { (if self.scalar_v237 { (v152 * (v1216 - v1304)) } else { (if self.scalar_v221 { ((v233 * (v1040 + (v220 * v1031))) + (v227 * self.scalar_v1049)) } else { v13 }) }) });
        let v1523: f64 = (if self.scalar_v290 { ((v325 * ((v318 * v1458) + (v314 * (v135 * v1428)))) + (v319 * ((v7 * v1468) + (v136 * (self.scalar_v1191 * v1498))))) } else { (if self.scalar_v237 { (v152 * (v1217 - v1305)) } else { (if self.scalar_v221 { ((v233 * (v1043 + (v220 * v1032))) + (v227 * (v136 * (-v1050)))) } else { v13 }) }) });
        let v1524: f64 = (if self.scalar_v290 { ((v325 * ((v318 * v1459) + (v314 * (v135 * v1429)))) + (v319 * (v7 * v1469))) } else { (if self.scalar_v237 { (v152 * (v1218 - v1306)) } else { (if self.scalar_v221 { (v233 * (v1046 + (v220 * v1033))) } else { v13 }) }) });
        let v1525: f64 = (if self.scalar_v330 { v913 } else { v1322 });
        let v1526: f64 = (if self.scalar_v330 { v914 } else { v1323 });
        let v1527: f64 = (if self.scalar_v330 { v915 } else { v1324 });
        let v1528: f64 = (if self.scalar_v330 { v916 } else { v1325 });
        let v1530: f64 = (v331 * v1525);
        let v1532: f64 = (v331 * v1526);
        let v1534: f64 = (v331 * v1527);
        let v1536: f64 = (v331 * v1528);
        let v1538: f64 = (v331 * self.scalar_v1529);
        let v1540: f64 = (if self.scalar_v330 { (v1530 + v1530) } else { v1337 });
        let v1541: f64 = (if self.scalar_v330 { (v1532 + v1532) } else { v1338 });
        let v1542: f64 = (if self.scalar_v330 { (v1534 + v1534) } else { v1339 });
        let v1543: f64 = (if self.scalar_v330 { (v1536 + v1536) } else { v1340 });
        let v1544: f64 = (if self.scalar_v330 { (v1538 + v1538) } else { v1341 });
        let v1589: f64 = (if self.scalar_v330 { (v176 * ((v1525 + (self.scalar_v201 * v1540)) + ((v336 * v1525) + (v331 * (v181 * v1540))))) } else { v1386 });
        let v1590: f64 = (if self.scalar_v330 { ((v338 * v892) + (v176 * ((v1526 + (self.scalar_v201 * v1541)) + ((v336 * v1526) + (v331 * (v181 * v1541)))))) } else { v1387 });
        let v1591: f64 = (if self.scalar_v330 { ((v338 * v893) + (v176 * ((v1527 + (self.scalar_v201 * v1542)) + ((v336 * v1527) + (v331 * (v181 * v1542)))))) } else { v1388 });
        let v1592: f64 = (if self.scalar_v330 { (v176 * ((v1528 + (self.scalar_v201 * v1543)) + ((v336 * v1528) + (v331 * (v181 * v1543))))) } else { v1389 });
        let v1593: f64 = (if self.scalar_v330 { (v176 * ((self.scalar_v1529 + (self.scalar_v201 * v1544)) + ((v336 * self.scalar_v1529) + (v331 * (v181 * v1544))))) } else { v1390 });
        let v1594: f64 = (if self.scalar_v330 { v913 } else { v1104 });
        let v1595: f64 = (if self.scalar_v330 { v1071 } else { v1105 });
        let v1596: f64 = (if self.scalar_v330 { v1072 } else { v1106 });
        let v1597: f64 = (if self.scalar_v330 { v1073 } else { v1107 });
        let v1598: f64 = (if self.scalar_v330 { v13 } else { v1108 });
        let v1599: f64 = (v341 * v1594);
        let v1601: f64 = (v341 * v1595);
        let v1603: f64 = (v341 * v1596);
        let v1605: f64 = (v341 * v1597);
        let v1607: f64 = (v341 * v1598);
        let v1609: f64 = (if self.scalar_v330 { (v1599 + v1599) } else { v13 });
        let v1610: f64 = (if self.scalar_v330 { (v1601 + v1601) } else { v13 });
        let v1611: f64 = (if self.scalar_v330 { (v1603 + v1603) } else { v13 });
        let v1612: f64 = (if self.scalar_v330 { (v1605 + v1605) } else { v13 });
        let v1613: f64 = (if self.scalar_v330 { (v1607 + v1607) } else { v13 });
        let v1658: f64 = (if self.scalar_v330 { (v176 * ((v1594 + (self.scalar_v201 * v1609)) + ((v346 * v1609) + (v343 * (v181 * v1594))))) } else { v1136 });
        let v1659: f64 = (if self.scalar_v330 { ((v348 * v892) + (v176 * ((v1595 + (self.scalar_v201 * v1610)) + ((v346 * v1610) + (v343 * (v181 * v1595)))))) } else { v1137 });
        let v1660: f64 = (if self.scalar_v330 { ((v348 * v893) + (v176 * ((v1596 + (self.scalar_v201 * v1611)) + ((v346 * v1611) + (v343 * (v181 * v1596)))))) } else { v1138 });
        let v1661: f64 = (if self.scalar_v330 { (v176 * ((v1597 + (self.scalar_v201 * v1612)) + ((v346 * v1612) + (v343 * (v181 * v1597))))) } else { v1139 });
        let v1662: f64 = (if self.scalar_v330 { (v176 * ((v1598 + (self.scalar_v201 * v1613)) + ((v346 * v1613) + (v343 * (v181 * v1598))))) } else { v1140 });
        let v1663: f64 = { let limexp_arg = v340; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v1674: f64 = { let limexp_arg = v352; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v1691: f64 = (v44 - (v356 * v356));
        let v1697: f64 = (if self.scalar_v330 { ((v152 * ((v1589 * v1663) - ((-v1589) * v1674))) * v1691) } else { v1425 });
        let v1698: f64 = (if self.scalar_v330 { ((v152 * ((v1590 * v1663) - ((-v1590) * v1674))) * v1691) } else { v1426 });
        let v1699: f64 = (if self.scalar_v330 { ((v152 * ((v1591 * v1663) - ((-v1591) * v1674))) * v1691) } else { v1427 });
        let v1700: f64 = (if self.scalar_v330 { ((v152 * ((v1592 * v1663) - ((-v1592) * v1674))) * v1691) } else { v1428 });
        let v1701: f64 = (if self.scalar_v330 { ((v152 * ((v1593 * v1663) - ((-v1593) * v1674))) * v1691) } else { v1429 });
        let v1702: f64 = { let limexp_arg = v350; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v1713: f64 = { let limexp_arg = v360; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v1730: f64 = (v44 - (v364 * v364));
        let v1736: f64 = (if self.scalar_v330 { ((v152 * ((v1658 * v1702) - ((-v1658) * v1713))) * v1730) } else { v13 });
        let v1737: f64 = (if self.scalar_v330 { ((v152 * ((v1659 * v1702) - ((-v1659) * v1713))) * v1730) } else { v13 });
        let v1738: f64 = (if self.scalar_v330 { ((v152 * ((v1660 * v1702) - ((-v1660) * v1713))) * v1730) } else { v13 });
        let v1739: f64 = (if self.scalar_v330 { ((v152 * ((v1661 * v1702) - ((-v1661) * v1713))) * v1730) } else { v13 });
        let v1740: f64 = (if self.scalar_v330 { ((v152 * ((v1662 * v1702) - ((-v1662) * v1713))) * v1730) } else { v13 });
        let v1741: f64 = (self.scalar_v184 * v1697);
        let v1742: f64 = (self.scalar_v184 * v1698);
        let v1743: f64 = (self.scalar_v184 * v1699);
        let v1744: f64 = (self.scalar_v184 * v1700);
        let v1745: f64 = (self.scalar_v184 * v1701);
        let v1746: f64 = (if self.scalar_v330 { v1741 } else { v1435 });
        let v1747: f64 = (if self.scalar_v330 { v1742 } else { v1436 });
        let v1748: f64 = (if self.scalar_v330 { v1743 } else { v1437 });
        let v1749: f64 = (if self.scalar_v330 { v1744 } else { v1438 });
        let v1750: f64 = (if self.scalar_v330 { v1745 } else { v1439 });
        let v1770: f64 = (v44 - (v374 * v374));
        let v1776: f64 = (if self.scalar_v330 { ((v7 * v1746) * v1770) } else { v1455 });
        let v1777: f64 = (if self.scalar_v330 { ((v369 + (v7 * v1747)) * v1770) } else { v1456 });
        let v1778: f64 = (if self.scalar_v330 { (((-v369) + (v7 * v1748)) * v1770) } else { v1457 });
        let v1779: f64 = (if self.scalar_v330 { ((v7 * v1749) * v1770) } else { v1458 });
        let v1780: f64 = (if self.scalar_v330 { ((v7 * v1750) * v1770) } else { v1459 });
        let v1790: f64 = (v44 - (v377 * v377));
        let v1856: f64 = ((v390 * ((v386 * (v135 * v1698)) + (v385 * v1777))) + (v387 * (v1195 + (v384 + (v7 * (if self.scalar_v330 { (self.scalar_v256 * v1698) } else { v13 }))))));
        let v1859: f64 = ((v390 * ((v386 * (v135 * v1699)) + (v385 * v1778))) + (v387 * (v1196 + ((-v384) + (v7 * (if self.scalar_v330 { (self.scalar_v256 * v1699) } else { v13 }))))));
        let v1866: f64 = (if self.scalar_v330 { ((v390 * ((v386 * (v135 * v1697)) + (v385 * v1776))) + (v387 * (v7 * (if self.scalar_v330 { (self.scalar_v256 * v1697) } else { v13 })))) } else { v1214 });
        let v1869: f64 = (if self.scalar_v330 { ((v390 * ((v386 * (v135 * v1700)) + (v385 * v1779))) + (v387 * (v7 * (if self.scalar_v330 { (self.scalar_v256 * v1700) } else { v13 })))) } else { v1217 });
        let v1870: f64 = (if self.scalar_v330 { ((v390 * ((v386 * (v135 * v1701)) + (v385 * v1780))) + (v387 * (v7 * (if self.scalar_v330 { (self.scalar_v256 * v1701) } else { v13 })))) } else { v1218 });
        let v1889: f64 = ((v394 * (v135 * v1738)) + (v393 * (-(if self.scalar_v330 { (((-v372) + (v7 * (if self.scalar_v330 { (self.scalar_v184 * v1738) } else { v13 }))) * v1790) } else { v13 }))));
        let v1911: f64 = ((v397 * ((v394 * (v135 * v1736)) + (v393 * (-(if self.scalar_v330 { ((v7 * (if self.scalar_v330 { (self.scalar_v184 * v1736) } else { v13 })) * v1790) } else { v13 }))))) + (v395 * (-(v7 * (if self.scalar_v330 { (self.scalar_v256 * v1736) } else { v13 })))));
        let v1912: f64 = (v397 * ((v394 * (v135 * v1737)) + (v393 * (-(if self.scalar_v330 { ((v372 + (v7 * (if self.scalar_v330 { (self.scalar_v184 * v1737) } else { v13 }))) * v1790) } else { v13 })))));
        let v1920: f64 = ((v397 * ((v394 * (v135 * v1739)) + (v393 * (-(if self.scalar_v330 { ((v7 * (if self.scalar_v330 { (self.scalar_v184 * v1739) } else { v13 })) * v1790) } else { v13 }))))) + (v395 * (-(v7 * (if self.scalar_v330 { (self.scalar_v256 * v1739) } else { v13 })))));
        let v1923: f64 = ((v397 * ((v394 * (v135 * v1740)) + (v393 * (-(if self.scalar_v330 { ((v7 * (if self.scalar_v330 { (self.scalar_v184 * v1740) } else { v13 })) * v1790) } else { v13 }))))) + (v395 * (-(v7 * (if self.scalar_v330 { (self.scalar_v256 * v1740) } else { v13 })))));
        let v1931: f64 = ((if self.scalar_v330 { v1859 } else { v1216 }) - (if self.scalar_v330 { ((v397 * v1889) + (v395 * (-((-v381) + (v7 * (if self.scalar_v330 { (self.scalar_v256 * v1738) } else { v13 })))))) } else { v1304 }));
        let v1935: f64 = (v152 * ((if self.scalar_v330 { v1856 } else { v1215 }) - (if self.scalar_v330 { (v1912 + (v395 * (-(v381 + (v7 * (if self.scalar_v330 { (self.scalar_v256 * v1737) } else { v13 })))))) } else { v1303 })));
        let v1949: f64 = (if self.scalar_v405 { v1741 } else { v1746 });
        let v1950: f64 = (if self.scalar_v405 { v1742 } else { v1747 });
        let v1951: f64 = (if self.scalar_v405 { v1743 } else { v1748 });
        let v1952: f64 = (if self.scalar_v405 { v1744 } else { v1749 });
        let v1953: f64 = (if self.scalar_v405 { v1745 } else { v1750 });
        let v1957: f64 = (-v407);
        let v1963: f64 = (v44 - (v409 * v409));
        let v1982: f64 = (v44 - (v412 * v412));
        let v2005: f64 = ((v416 * v1029) + (v226 * ((if self.scalar_v405 { ((v7 * v1949) * v1963) } else { v1776 }) + (self.scalar_v414 * (if self.scalar_v405 { ((v407 + (v11 * v1949)) * v1982) } else { v13 })))));
        let v2008: f64 = ((v416 * v1030) + (v226 * ((if self.scalar_v405 { ((v407 + (v7 * v1950)) * v1963) } else { v1777 }) + (self.scalar_v414 * (if self.scalar_v405 { ((v11 * v1950) * v1982) } else { v13 })))));
        let v2010: f64 = (v226 * ((if self.scalar_v405 { ((v1957 + (v7 * v1951)) * v1963) } else { v1778 }) + (self.scalar_v414 * (if self.scalar_v405 { ((v1957 + (v11 * v1951)) * v1982) } else { v13 }))));
        let v2014: f64 = ((v416 * v1032) + (v226 * ((if self.scalar_v405 { ((v7 * v1952) * v1963) } else { v1779 }) + (self.scalar_v414 * (if self.scalar_v405 { ((v11 * v1952) * v1982) } else { v13 })))));
        let v2017: f64 = ((v416 * v1033) + (v226 * ((if self.scalar_v405 { ((v7 * v1953) * v1963) } else { v1780 }) + (self.scalar_v414 * (if self.scalar_v405 { ((v11 * v1953) * v1982) } else { v13 })))));
        let v2047: f64 = (if self.scalar_v405 { ((v422 * v2005) + (v417 * ((v419 * (if self.scalar_v405 { v1163 } else { v1465 })) + (v406 * self.scalar_v414)))) } else { (if self.scalar_v330 { (v152 * (v1866 - (if self.scalar_v330 { v1911 } else { v1302 }))) } else { v1520 }) });
        let v2048: f64 = (if self.scalar_v405 { ((v422 * v2008) + (v417 * (v1195 + (v406 + (v419 * (if self.scalar_v405 { v1164 } else { v1466 })))))) } else { (if self.scalar_v330 { v1935 } else { (if self.scalar_v290 { v1510 } else { v1318 }) }) });
        let v2049: f64 = (if self.scalar_v405 { ((v422 * ((v416 * v1031) + v2010)) + (v417 * (v1196 + ((v419 * (if self.scalar_v405 { v1165 } else { v1467 })) + (v406 * self.scalar_v2019))))) } else { (if self.scalar_v330 { (v152 * v1931) } else { v1522 }) });
        let v2050: f64 = (if self.scalar_v405 { ((v422 * v2014) + (v417 * (v419 * (if self.scalar_v405 { v1166 } else { v1468 })))) } else { (if self.scalar_v330 { (v152 * (v1869 - (if self.scalar_v330 { v1920 } else { v1305 }))) } else { v1523 }) });
        let v2051: f64 = (if self.scalar_v405 { ((v422 * v2017) + (v417 * (v419 * (if self.scalar_v405 { v1167 } else { v1469 })))) } else { (if self.scalar_v330 { (v152 * (v1870 - (if self.scalar_v330 { v1923 } else { v1306 }))) } else { v1524 }) });
        let v2054: f64 = (v427 * v427);
        let v2085: f64 = (v440 * v440);
        let v2114: f64 = (v451 * (if self.scalar_v439 { (self.scalar_v432 * v1697) } else { (if self.scalar_v425 { (self.scalar_v432 * v970) } else { v13 }) }));
        let v2115: f64 = (v451 * (if self.scalar_v439 { (self.scalar_v432 * v1698) } else { (if self.scalar_v425 { (self.scalar_v432 * v971) } else { v13 }) }));
        let v2116: f64 = (v451 * (if self.scalar_v439 { (self.scalar_v432 * v1699) } else { (if self.scalar_v425 { (self.scalar_v432 * v972) } else { v13 }) }));
        let v2117: f64 = (v451 * (if self.scalar_v439 { (self.scalar_v432 * v1700) } else { (if self.scalar_v425 { (self.scalar_v432 * v973) } else { v13 }) }));
        let v2118: f64 = (v451 * (if self.scalar_v439 { (self.scalar_v432 * v1701) } else { (if self.scalar_v425 { (self.scalar_v432 * v974) } else { v13 }) }));
        let v2127: f64 = (if self.scalar_v473 { v13 } else { (if self.scalar_v455 { v13 } else { v1526 }) });
        let v2128: f64 = (if self.scalar_v473 { v13 } else { (if self.scalar_v455 { v13 } else { v1527 }) });
        let v2129: f64 = (if self.scalar_v473 { v13 } else { (if self.scalar_v455 { v13 } else { v1528 }) });
        let v2132: f64 = (v44 - (v488 * v488));
        let v2137: f64 = (v44 - (v490 * v490));
        let v2149: f64 = { let limexp_arg = v498; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v2154: f64 = { let limexp_arg = v502; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v2166: f64 = (self.scalar_v501 * (-(if self.scalar_v473 { v13 } else { (if self.scalar_v455 { v13 } else { v1525 }) })));
        let v2167: f64 = (self.scalar_v501 * (-v2127));
        let v2168: f64 = (self.scalar_v501 * ((((v159 * (if self.scalar_v493 { v456 } else { (if self.scalar_v487 { (-v2132) } else { self.scalar_v2124 }) })) * v2154) - (self.scalar_v506 * (self.scalar_v2147 * v2149))) - v2128));
        let v2169: f64 = (self.scalar_v501 * (-v2129));
        let v2170: f64 = (self.scalar_v501 * (((v159 * (if self.scalar_v493 { v44 } else { (if self.scalar_v487 { v2132 } else { self.scalar_v2125 }) })) * v2154) - (self.scalar_v506 * (self.scalar_v2148 * v2149))));
        let v2172: f64 = { let limexp_arg = v511; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v2177: f64 = { let limexp_arg = v514; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v2187: f64 = (self.scalar_v501 * ((((v159 * (if self.scalar_v493 { v456 } else { (if self.scalar_v487 { (-v2137) } else { self.scalar_v2124 }) })) * v2177) - (self.scalar_v506 * (self.scalar_v2147 * v2172))) - v2127));
        let v2188: f64 = (self.scalar_v501 * (-v2128));
        let v2189: f64 = (self.scalar_v501 * ((((v159 * (if self.scalar_v493 { v44 } else { (if self.scalar_v487 { v2137 } else { self.scalar_v2125 }) })) * v2177) - (self.scalar_v506 * (self.scalar_v2148 * v2172))) - v2129));
        let v2194: f64 = (v44 - (v526 * v526));
        let v2195: f64 = (self.scalar_v523 * v2194);
        let v2196: f64 = (self.scalar_v2192 * v2194);
        let v2197: f64 = (self.scalar_v520 * v2194);
        let v2200: f64 = (v44 - (v532 * v532));
        let v2201: f64 = (self.scalar_v529 * v2200);
        let v2202: f64 = (self.scalar_v2198 * v2200);
        let v2205: f64 = (v44 - (v538 * v538));
        let v2206: f64 = (self.scalar_v2203 * v2205);
        let v2207: f64 = (self.scalar_v535 * v2205);
        let v2211: f64 = (v44 - (v545 * v545));
        let v2212: f64 = (self.scalar_v2209 * v2211);
        let v2213: f64 = (self.scalar_v523 * v2211);
        let v2214: f64 = (self.scalar_v541 * v2211);
        let v2241: f64 = ((v572) as f64).sinh();
        let v2242: f64 = (self.scalar_v523 * v2241);
        let v2243: f64 = (self.scalar_v2191 * v2241);
        let v2244: f64 = (if self.scalar_v569 { v2242 } else { v13 });
        let v2245: f64 = (if self.scalar_v569 { v2243 } else { v13 });
        let v2248: f64 = (if self.scalar_v569 { (v2244 / v574) } else { v13 });
        let v2249: f64 = (if self.scalar_v569 { (v2245 / v574) } else { v13 });
        let v2251: f64 = (self.scalar_v523 * v2250);
        let v2252: f64 = (self.scalar_v2192 * v2250);
        let v2254: f64 = (if self.scalar_v569 { v2251 } else { v13 });
        let v2255: f64 = (if self.scalar_v569 { v2252 } else { v13 });
        let v2260: f64 = (if self.scalar_v569 { (v2254 / v578) } else { v13 });
        let v2261: f64 = (if self.scalar_v569 { (v2255 / v578) } else { v13 });
        let v2265: f64 = (if self.scalar_v569 { (self.scalar_v523 + v2248) } else { v13 });
        let v2266: f64 = (if self.scalar_v569 { (self.scalar_v2191 + v2249) } else { v13 });
        let v2294: f64 = ((v593) as f64).sinh();
        let v2295: f64 = (self.scalar_v2191 * v2294);
        let v2296: f64 = (self.scalar_v523 * v2294);
        let v2297: f64 = (if self.scalar_v569 { v2295 } else { v2244 });
        let v2298: f64 = (if self.scalar_v569 { v2296 } else { v2245 });
        let v2301: f64 = (if self.scalar_v569 { (v2297 / v595) } else { v13 });
        let v2302: f64 = (if self.scalar_v569 { (v2298 / v595) } else { v13 });
        let v2304: f64 = (self.scalar_v2209 * v2303);
        let v2305: f64 = (self.scalar_v523 * v2303);
        let v2307: f64 = (if self.scalar_v569 { v2304 } else { v2254 });
        let v2308: f64 = (if self.scalar_v569 { v2305 } else { v2255 });
        let v2315: f64 = (if self.scalar_v569 { (v2307 / v599) } else { v13 });
        let v2316: f64 = (if self.scalar_v569 { (v2308 / v599) } else { v13 });
        let v2318: f64 = (if self.scalar_v569 { (v2310 / v599) } else { v13 });
        let v2321: f64 = (if self.scalar_v569 { (self.scalar_v2191 + v2301) } else { v13 });
        let v2322: f64 = (if self.scalar_v569 { (self.scalar_v523 + v2302) } else { v13 });
        let v2363: f64 = (v624 * self.scalar_v2361);
        let v2364: f64 = (v2363 + v2363);
        let v2365: f64 = (v624 * self.scalar_v2362);
        let v2366: f64 = (v2365 + v2365);
        let v2369: f64 = (self.scalar_v629 * f64::powf(v628, self.scalar_v2367));
        let v2386: f64 = (v44 - (v640 * v640));
        let v2398: f64 = (v44 - (v651 * v651));
        let v2410: f64 = (v137 * ((if self.scalar_v620 { (self.scalar_v2384 * v2386) } else { v2196 }) + (self.scalar_v654 * (if self.scalar_v620 { ((v634 * (v2364 * v2369)) + (v630 * (self.scalar_v632 * v2364))) } else { v13 }))));
        let v2411: f64 = (v137 * ((if self.scalar_v620 { (self.scalar_v520 * v2386) } else { v2197 }) + (self.scalar_v654 * (if self.scalar_v620 { ((v634 * (v2366 * v2369)) + (v630 * (self.scalar_v632 * v2366))) } else { v13 }))));
        let v2419: f64 = (if self.scalar_v620 { ((v657 * v2201) + (v643 * (v137 * (if self.scalar_v620 { (self.scalar_v2383 * v2386) } else { v2195 })))) } else { (if self.scalar_v569 { v13 } else { (if self.scalar_v556 { ((v557 * v2201) + (v533 * (v137 * v2195))) } else { v13 }) }) });
        let v2432: f64 = (if self.scalar_v620 { (v138 * ((v653 * v2206) + (v646 * (if self.scalar_v620 { (self.scalar_v2395 * v2398) } else { v2212 })))) } else { (if self.scalar_v569 { v13 } else { (if self.scalar_v556 { (v138 * ((v546 * v2206) + (v540 * v2212))) } else { v13 }) }) });
        let v2433: f64 = (if self.scalar_v620 { (v138 * ((v653 * v2207) + (v646 * (if self.scalar_v620 { (self.scalar_v2396 * v2398) } else { v2213 })))) } else { (if self.scalar_v569 { v13 } else { (if self.scalar_v556 { (v138 * ((v546 * v2207) + (v540 * v2213))) } else { v13 }) }) });
        let v2435: f64 = (if self.scalar_v668 { v2242 } else { v2297 });
        let v2436: f64 = (if self.scalar_v668 { v2243 } else { v2298 });
        let v2441: f64 = (if self.scalar_v668 { v2251 } else { v2307 });
        let v2442: f64 = (if self.scalar_v668 { v2252 } else { v2308 });
        let v2480: f64 = ((if self.scalar_v668 { ((v682 * self.scalar_v2453) + (v677 * ((self.scalar_v2359 * v2455) * v2460))) } else { v13 }) + ((self.scalar_v2192 + (if self.scalar_v668 { (v2442 / v672) } else { v2261 })) - (if self.scalar_v668 { (self.scalar_v2191 + (if self.scalar_v668 { (v2436 / v669) } else { v2249 })) } else { v2266 })));
        let v2484: f64 = ((v696 * ((self.scalar_v523 + (if self.scalar_v668 { (v2441 / v672) } else { v2260 })) - (if self.scalar_v668 { (self.scalar_v523 + (if self.scalar_v668 { (v2435 / v669) } else { v2248 })) } else { v2265 }))) + (v695 * v2201));
        let v2502: f64 = (if self.scalar_v668 { (v137 * (v2484 / self.scalar_v520)) } else { (if self.scalar_v569 { (v137 * (((v584 * v2201) + (v571 * ((self.scalar_v523 + v2260) - v2265))) / self.scalar_v520)) } else { v13 }) });
        let v2503: f64 = (if self.scalar_v668 { (self.scalar_v2288 + (v137 * (self.scalar_v2282 + (((v696 * v2480) + (v695 * v2202)) / self.scalar_v520)))) } else { (if self.scalar_v569 { ((v137 * ((((v584 * v2202) + (v571 * ((self.scalar_v2192 + v2261) - v2266))) / self.scalar_v520) + self.scalar_v2282)) + self.scalar_v2288) } else { v13 }) });
        let v2531: f64 = ((self.scalar_v2209 + (if self.scalar_v668 { ((if self.scalar_v668 { v2304 } else { v2441 }) / v706) } else { v2315 })) - (if self.scalar_v668 { (self.scalar_v2191 + (if self.scalar_v668 { ((if self.scalar_v668 { v2295 } else { v2435 }) / v703) } else { v2301 })) } else { v2321 }));
        let v2532: f64 = ((self.scalar_v523 + (if self.scalar_v668 { ((if self.scalar_v668 { v2305 } else { v2442 }) / v706) } else { v2316 })) - (if self.scalar_v668 { (self.scalar_v523 + (if self.scalar_v668 { ((if self.scalar_v668 { v2296 } else { v2436 }) / v703) } else { v2302 })) } else { v2322 }));
        let v2553: f64 = (if self.scalar_v668 { (self.scalar_v2346 + (v138 * (self.scalar_v2282 + (((v712 * v2206) + (v645 * v2531)) / self.scalar_v541)))) } else { (if self.scalar_v569 { ((v138 * (self.scalar_v2282 + (((v605 * v2206) + (v540 * ((self.scalar_v2209 + v2315) - v2321))) / self.scalar_v541))) + self.scalar_v2346) } else { v13 }) });
        let v2554: f64 = (if self.scalar_v668 { (v138 * (((v712 * v2207) + (v645 * v2532)) / self.scalar_v541)) } else { (if self.scalar_v569 { (v138 * (((v605 * v2207) + (v540 * ((self.scalar_v523 + v2316) - v2322))) / self.scalar_v541)) } else { v13 }) });
        let v2556: f64 = (if self.scalar_v668 { (v138 * ((v645 * (if self.scalar_v668 { ((if self.scalar_v668 { v13 } else { v2444 }) / v706) } else { v2318 })) / self.scalar_v541)) } else { (if self.scalar_v569 { (v138 * ((v540 * v2318) / self.scalar_v541)) } else { v13 }) });
        let v2558: f64 = (if self.scalar_v668 { v13 } else { (if self.scalar_v620 { ((v657 * v2202) + (v643 * v2410)) } else { (if self.scalar_v569 { v13 } else { (if self.scalar_v556 { ((v557 * v2202) + (v533 * (v137 * v2196))) } else { v13 }) }) }) });
        let v2562: f64 = (if self.scalar_v668 { v13 } else { (if self.scalar_v620 { (v138 * (v646 * (if self.scalar_v620 { (self.scalar_v541 * v2398) } else { v2214 }))) } else { (if self.scalar_v569 { v13 } else { (if self.scalar_v556 { (v138 * (v540 * v2214)) } else { v13 }) }) }) });
        let v2563: f64 = (-v2047);
        let v2564: f64 = (-v2048);
        let v2565: f64 = (-v2049);
        let v2566: f64 = (-v2050);
        let v2567: f64 = (-v2051);
        let v2568: f64 = ddt_scale;
        let v2573: f64 = (if self.scalar_v723 { (v2553 * v2568) } else { v13 });
        let v2574: f64 = (if self.scalar_v723 { (v2554 * v2568) } else { v13 });
        let v2575: f64 = (if self.scalar_v723 { (v2555 * v2568) } else { v13 });
        let v2576: f64 = (if self.scalar_v723 { (v2556 * v2568) } else { v13 });
        let v2581: f64 = (if self.scalar_v723 { (v2502 * v2568) } else { v13 });
        let v2582: f64 = (if self.scalar_v723 { (v2503 * v2568) } else { v13 });
        let v2583: f64 = (if self.scalar_v723 { ((if self.scalar_v668 { (v137 * ((v696 * (if self.scalar_v668 { (v2443 / v672) } else { v13 })) / self.scalar_v520)) } else { v13 }) * v2568) } else { v13 });
        let v2584: f64 = (if self.scalar_v723 { (v2505 * v2568) } else { v13 });
        let v2594: f64 = (if self.scalar_v781 { (v2568 * ((-v722) + (v5 * (if self.scalar_v668 { v13 } else { v2432 })))) } else { v13 });
        let v2595: f64 = (if self.scalar_v781 { (v2568 * (v5 * (if self.scalar_v668 { v13 } else { v2433 }))) } else { v13 });
        let v2596: f64 = (if self.scalar_v781 { (v2568 * (v722 + (v5 * v2562))) } else { v13 });
        let v2605: f64 = (v2568 * (v720 + (v9 * (if self.scalar_v668 { v13 } else { (if self.scalar_v620 { (v643 * v2411) } else { (if self.scalar_v569 { v13 } else { (if self.scalar_v556 { (v533 * (v137 * v2197)) } else { v13 }) }) }) }))));
        let v2606: f64 = (if self.scalar_v781 { (v2568 * (v9 * (if self.scalar_v668 { v13 } else { v2419 }))) } else { v13 });
        let v2607: f64 = (if self.scalar_v781 { (v2568 * ((-v720) + (v9 * v2558))) } else { v13 });
        let v2608: f64 = (if self.scalar_v781 { v2605 } else { v13 });
        let v2611: f64 = (-v140);
        let v2612: f64 = -1e-12;
        let v2620: f64 = (if self.scalar_v725 { (v798 * (if self.scalar_v439 { ((-(v139 * v1697)) / v2085) } else { (if self.scalar_v425 { ((-(v139 * v970)) / v2054) } else { v13 }) })) } else { v13 });
        let v2621: f64 = (if self.scalar_v725 { (v798 * (if self.scalar_v439 { ((-(v139 * v1698)) / v2085) } else { (if self.scalar_v425 { ((-(v139 * v971)) / v2054) } else { v13 }) })) } else { v13 });
        let v2622: f64 = (if self.scalar_v725 { (v798 * (if self.scalar_v439 { ((-(v139 * v1699)) / v2085) } else { (if self.scalar_v425 { ((-(v139 * v972)) / v2054) } else { v13 }) })) } else { v13 });
        let v2623: f64 = (if self.scalar_v725 { (v798 * (if self.scalar_v439 { ((-(v139 * v1700)) / v2085) } else { (if self.scalar_v425 { ((-(v139 * v973)) / v2054) } else { v13 }) })) } else { v13 });
        let v2624: f64 = (if self.scalar_v725 { (v798 * (if self.scalar_v439 { ((-(v139 * v1701)) / v2085) } else { (if self.scalar_v425 { ((-(v139 * v974)) / v2054) } else { v13 }) })) } else { v13 });
        let v2625: f64 = (if self.scalar_v725 { (v443 + (self.scalar_v724 * v2568)) } else { v13 });
        let v2628: f64 = (if self.scalar_v728 { (v44 / v141) } else { v13 });
        let v2629: f64 = (if self.scalar_v728 { (v456 / v141) } else { v13 });
        let v2633: f64 = (if self.scalar_v728 { (v2568 * (-v142)) } else { v13 });
        let v2634: f64 = (if self.scalar_v728 { (v142 * v2568) } else { v13 });
        let v2654: f64 = (if self.scalar_v737 { (v830 * v2114) } else { v13 });
        let v2655: f64 = (if self.scalar_v737 { (v830 * v2115) } else { v13 });
        let v2656: f64 = (if self.scalar_v737 { (v830 * v2116) } else { v13 });
        let v2657: f64 = (if self.scalar_v737 { (v830 * v2117) } else { v13 });
        let v2658: f64 = (if self.scalar_v737 { (v830 * v2118) } else { v13 });
        let v2659: f64 = (if self.scalar_v737 { v452 } else { v13 });
        let v2665: f64 = (if self.scalar_v740 { (v836 * v2114) } else { v13 });
        let v2666: f64 = (if self.scalar_v740 { (v836 * v2115) } else { v13 });
        let v2667: f64 = (if self.scalar_v740 { (v836 * v2116) } else { v13 });
        let v2668: f64 = (if self.scalar_v740 { (v836 * v2117) } else { v13 });
        let v2669: f64 = (if self.scalar_v740 { (v836 * v2118) } else { v13 });
        let v2670: f64 = (if self.scalar_v740 { v453 } else { v13 });
        let v2672: f64 = (if self.scalar_v747 { v766 } else { v13 });
        let v2673: f64 = (if self.scalar_v747 { v762 } else { v13 });
        let v2675: f64 = (if self.scalar_v747 { (v854 * v2568) } else { v13 });
        let v2677: f64 = (if self.scalar_v769 { (v44 / v49) } else { v13 });
        let v2679: f64 = (if self.scalar_v769 { (self.scalar_v867 * v2568) } else { v13 });

        let d770_dn4: f64 = v2563;
        let d770_dn5: f64 = v2564;
        let d770_dn8: f64 = v2565;
        let d770_dn10: f64 = v2566;
        let d770_dn12: f64 = v2567;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(15),
            None,
            multiplicity * (v770),
            [4, 5, 8, 10, 12],
            [d770_dn4, d770_dn5, d770_dn8, d770_dn10, d770_dn12],
            [],
            [],
            multiplicity,
        );
        let d773_dn15: f64 = self.scalar_v771;
        let v773_ddt: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, v773);
        stamper.stamp_current_node1_local(
            Some(15),
            None,
            multiplicity * (v773_ddt),
            15,
            multiplicity * (((d773_dn15) * ddt_scale)),
        );
        let d12_dn16: f64 = v44;
        stamper.stamp_current_node1_local(
            Some(16),
            None,
            multiplicity * (v12),
            16,
            multiplicity * (d12_dn16),
        );
        let d776_db0: f64 = self.scalar_v774;
        let v776_ddt: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, v776);
        stamper.stamp_potential_branch_local(
            Some(15),
            Some(16),
            0,
            multiplicity,
        );
        stamper.stamp_potential_branch1_local(
            0,
            v776_ddt,
            0,
            ((d776_db0) * ddt_scale),
        );
        let d12_dn16: f64 = v44;
        stamper.stamp_current_node1_local(
            Some(5),
            Some(8),
            multiplicity * (v12),
            16,
            multiplicity * (d12_dn16),
        );
        let d510_dn4: f64 = v2166;
        let d510_dn5: f64 = v2167;
        let d510_dn8: f64 = v2168;
        let d510_dn10: f64 = v2169;
        let d510_dn11: f64 = v2170;
        let d510_dn12: f64 = self.scalar_v2171;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(11),
            Some(8),
            multiplicity * (v510),
            [4, 5, 8, 10, 11, 12],
            [d510_dn4, d510_dn5, d510_dn8, d510_dn10, d510_dn11, d510_dn12],
            [],
            [],
            multiplicity,
        );
        let d519_dn4: f64 = v2166;
        let d519_dn5: f64 = v2187;
        let d519_dn8: f64 = v2188;
        let d519_dn10: f64 = v2189;
        let d519_dn12: f64 = self.scalar_v2171;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(10),
            Some(5),
            multiplicity * (v519),
            [4, 5, 8, 10, 12],
            [d519_dn4, d519_dn5, d519_dn8, d519_dn10, d519_dn12],
            [],
            [],
            multiplicity,
        );
        let d778_dn5: f64 = v2573;
        let d778_dn8: f64 = v2574;
        let d778_dn10: f64 = v2575;
        let d778_dn11: f64 = v2576;
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(10),
            Some(5),
            multiplicity * (v778),
            [5, 8, 10, 11],
            [d778_dn5, d778_dn8, d778_dn10, d778_dn11],
            [],
            [],
            multiplicity,
        );
        let d780_dn5: f64 = v2581;
        let d780_dn8: f64 = v2582;
        let d780_dn10: f64 = v2583;
        let d780_dn11: f64 = v2584;
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(11),
            Some(8),
            multiplicity * (v780),
            [5, 8, 10, 11],
            [d780_dn5, d780_dn8, d780_dn10, d780_dn11],
            [],
            [],
            multiplicity,
        );
        let d784_dn5: f64 = v2594;
        let d784_dn8: f64 = v2595;
        let d784_dn10: f64 = v2596;
        stamper.stamp_current_node3_local(
            Some(10),
            Some(5),
            multiplicity * (v784),
            5,
            multiplicity * (d784_dn5),
            8,
            multiplicity * (d784_dn8),
            10,
            multiplicity * (d784_dn10),
        );
        let d787_dn5: f64 = v2606;
        let d787_dn8: f64 = v2607;
        let d787_dn11: f64 = v2608;
        stamper.stamp_current_node3_local(
            Some(11),
            Some(8),
            multiplicity * (v787),
            5,
            multiplicity * (d787_dn5),
            8,
            multiplicity * (d787_dn8),
            11,
            multiplicity * (d787_dn11),
        );
        let d791_dn5: f64 = self.scalar_v2609;
        let d791_dn7: f64 = self.scalar_v788;
        let v791_ddt: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 7, v791);
        stamper.stamp_current_node2_local(
            Some(7),
            Some(5),
            multiplicity * (v791_ddt),
            5,
            multiplicity * (((d791_dn5) * ddt_scale)),
            7,
            multiplicity * (((d791_dn7) * ddt_scale)),
        );
        let d793_dn5: f64 = self.scalar_v792;
        let d793_dn8: f64 = self.scalar_v2610;
        let v793_ddt: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 8, v793);
        stamper.stamp_current_node2_local(
            Some(5),
            Some(8),
            multiplicity * (v793_ddt),
            5,
            multiplicity * (((d793_dn5) * ddt_scale)),
            8,
            multiplicity * (((d793_dn8) * ddt_scale)),
        );
        let d796_dn4: f64 = v2611;
        let d796_dn6: f64 = v140;
        let v796_ddt: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 9, v796);
        stamper.stamp_current_node2_local(
            Some(6),
            Some(4),
            multiplicity * (v796_ddt),
            4,
            multiplicity * (((d796_dn4) * ddt_scale)),
            6,
            multiplicity * (((d796_dn6) * ddt_scale)),
        );
        let d797_dn4: f64 = v2612;
        let d797_dn6: f64 = v167;
        stamper.stamp_current_node2_local(
            Some(6),
            Some(4),
            multiplicity * (v797),
            4,
            multiplicity * (d797_dn4),
            6,
            multiplicity * (d797_dn6),
        );
        let d803_dn4: f64 = v2620;
        let d803_dn5: f64 = v2621;
        let d803_dn8: f64 = v2622;
        let d803_dn10: f64 = v2623;
        let d803_dn12: f64 = v2624;
        let d803_db1: f64 = v2625;
        stamper.stamp_potential_branch_local(
            Some(4),
            Some(8),
            1,
            multiplicity,
        );
        stamper.stamp_potential_sparse_local::<5, 1>(
            1,
            v803,
            [4, 5, 8, 10, 12],
            [d803_dn4, d803_dn5, d803_dn8, d803_dn10, d803_dn12],
            [1],
            [d803_db1],
        );
        stamper.stamp_potential_branch_local(
            Some(4),
            Some(8),
            2,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            2,
            v13,
        );
        let d806_dn11: f64 = v2628;
        let d806_dn12: f64 = v2629;
        stamper.stamp_current_node2_local(
            Some(11),
            Some(12),
            multiplicity * (v806),
            11,
            multiplicity * (d806_dn11),
            12,
            multiplicity * (d806_dn12),
        );
        let d809_dn8: f64 = v2633;
        let d809_dn12: f64 = v2634;
        stamper.stamp_current_node2_local(
            Some(12),
            Some(8),
            multiplicity * (v809),
            8,
            multiplicity * (d809_dn8),
            12,
            multiplicity * (d809_dn12),
        );
        stamper.stamp_potential_branch_local(
            Some(11),
            Some(8),
            3,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            3,
            v13,
        );
        let d813_dn11: f64 = self.scalar_v810;
        let d813_dn14: f64 = self.scalar_v2635;
        let v813_ddt: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 11, v813);
        stamper.stamp_current_node2_local(
            Some(11),
            Some(14),
            multiplicity * (v813_ddt),
            11,
            multiplicity * (((d813_dn11) * ddt_scale)),
            14,
            multiplicity * (((d813_dn14) * ddt_scale)),
        );
        let d816_dn8: f64 = self.scalar_v2638;
        let d816_dn14: f64 = self.scalar_v2639;
        stamper.stamp_current_node2_local(
            Some(14),
            Some(8),
            multiplicity * (v816),
            8,
            multiplicity * (d816_dn8),
            14,
            multiplicity * (d816_dn14),
        );
        stamper.stamp_potential_branch_local(
            Some(14),
            Some(8),
            4,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            4,
            v13,
        );
        let d820_dn10: f64 = self.scalar_v2642;
        let d820_dn13: f64 = self.scalar_v2643;
        stamper.stamp_current_node2_local(
            Some(13),
            Some(10),
            multiplicity * (v820),
            10,
            multiplicity * (d820_dn10),
            13,
            multiplicity * (d820_dn13),
        );
        stamper.stamp_potential_branch_local(
            Some(13),
            Some(10),
            5,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            5,
            v13,
        );
        stamper.stamp_current_const_local(
            Some(13),
            Some(10),
            multiplicity * (v13),
        );
        let d823_dn11: f64 = self.scalar_v2646;
        let d823_dn13: f64 = self.scalar_v2647;
        stamper.stamp_current_node2_local(
            Some(13),
            Some(11),
            multiplicity * (v823),
            11,
            multiplicity * (d823_dn11),
            13,
            multiplicity * (d823_dn13),
        );
        stamper.stamp_potential_branch_local(
            Some(13),
            Some(11),
            6,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            6,
            v13,
        );
        let d826_db7: f64 = self.scalar_v2648;
        stamper.stamp_potential_branch_local(
            Some(7),
            Some(13),
            7,
            multiplicity,
        );
        stamper.stamp_potential_branch1_local(
            7,
            v826,
            7,
            d826_db7,
        );
        stamper.stamp_potential_branch_local(
            Some(7),
            Some(13),
            8,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            8,
            v13,
        );
        stamper.stamp_potential_branch_local(
            Some(7),
            Some(13),
            9,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            9,
            v13,
        );
        let d829_db10: f64 = self.scalar_v827;
        let v829_ddt: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 12, v829);
        stamper.stamp_potential_branch_local(
            Some(1),
            Some(7),
            10,
            multiplicity,
        );
        stamper.stamp_potential_branch1_local(
            10,
            v829_ddt,
            10,
            ((d829_db10) * ddt_scale),
        );
        let d832_dn4: f64 = v2654;
        let d832_dn5: f64 = v2655;
        let d832_dn8: f64 = v2656;
        let d832_dn10: f64 = v2657;
        let d832_dn12: f64 = v2658;
        let d832_db11: f64 = v2659;
        stamper.stamp_potential_branch_local(
            Some(8),
            Some(9),
            11,
            multiplicity,
        );
        stamper.stamp_potential_sparse_local::<5, 1>(
            11,
            v832,
            [4, 5, 8, 10, 12],
            [d832_dn4, d832_dn5, d832_dn8, d832_dn10, d832_dn12],
            [11],
            [d832_db11],
        );
        stamper.stamp_potential_branch_local(
            Some(8),
            Some(9),
            12,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            12,
            v13,
        );
        stamper.stamp_potential_branch_local(
            Some(8),
            Some(9),
            13,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            13,
            v13,
        );
        let d835_db14: f64 = self.scalar_v833;
        let v835_ddt: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 13, v835);
        stamper.stamp_potential_branch_local(
            Some(9),
            Some(2),
            14,
            multiplicity,
        );
        stamper.stamp_potential_branch1_local(
            14,
            v835_ddt,
            14,
            ((d835_db14) * ddt_scale),
        );
        let d838_dn4: f64 = v2665;
        let d838_dn5: f64 = v2666;
        let d838_dn8: f64 = v2667;
        let d838_dn10: f64 = v2668;
        let d838_dn12: f64 = v2669;
        let d838_db15: f64 = v2670;
        stamper.stamp_potential_branch_local(
            Some(5),
            Some(6),
            15,
            multiplicity,
        );
        stamper.stamp_potential_sparse_local::<5, 1>(
            15,
            v838,
            [4, 5, 8, 10, 12],
            [d838_dn4, d838_dn5, d838_dn8, d838_dn10, d838_dn12],
            [15],
            [d838_db15],
        );
        stamper.stamp_potential_branch_local(
            Some(5),
            Some(6),
            16,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            16,
            v13,
        );
        stamper.stamp_potential_branch_local(
            Some(5),
            Some(6),
            17,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            17,
            v13,
        );
        let d841_db18: f64 = self.scalar_v839;
        let v841_ddt: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 14, v841);
        stamper.stamp_potential_branch_local(
            Some(6),
            Some(0),
            18,
            multiplicity,
        );
        stamper.stamp_potential_branch1_local(
            18,
            v841_ddt,
            18,
            ((d841_db18) * ddt_scale),
        );
        stamper.stamp_current_const_local(
            Some(0),
            Some(2),
            multiplicity * (v842),
        );
        stamper.stamp_current_const_local(
            Some(14),
            Some(2),
            multiplicity * (v167),
        );
        let d845_dn2: f64 = v2612;
        let d845_dn12: f64 = v167;
        stamper.stamp_current_node2_local(
            Some(12),
            Some(2),
            multiplicity * (v845),
            2,
            multiplicity * (d845_dn2),
            12,
            multiplicity * (d845_dn12),
        );
        stamper.stamp_current_const_local(
            Some(5),
            Some(8),
            multiplicity * (v13),
        );
        stamper.stamp_current_const_local(
            Some(5),
            Some(8),
            multiplicity * (v13),
        );
        stamper.stamp_current_const_local(
            Some(17),
            None,
            multiplicity * (v13),
        );
        let d847_dn17: f64 = self.scalar_v2671;
        stamper.stamp_current_node1_local(
            Some(17),
            None,
            multiplicity * (v847),
            17,
            multiplicity * (d847_dn17),
        );
        stamper.stamp_current_const_local(
            Some(18),
            None,
            multiplicity * (v13),
        );
        let d849_dn18: f64 = self.scalar_v2671;
        stamper.stamp_current_node1_local(
            Some(18),
            None,
            multiplicity * (v849),
            18,
            multiplicity * (d849_dn18),
        );
        let d847_dn17: f64 = self.scalar_v2671;
        stamper.stamp_current_node1_local(
            Some(7),
            Some(8),
            multiplicity * (v847),
            17,
            multiplicity * (d847_dn17),
        );
        let d853_dn17: f64 = v2672;
        let d853_dn18: f64 = v2673;
        stamper.stamp_current_node2_local(
            Some(7),
            Some(5),
            multiplicity * (v853),
            17,
            multiplicity * (d853_dn17),
            18,
            multiplicity * (d853_dn18),
        );
        let d857_dn17: f64 = v2675;
        stamper.stamp_current_node1_local(
            Some(7),
            Some(5),
            multiplicity * (v857),
            17,
            multiplicity * (d857_dn17),
        );
        stamper.stamp_current_const_local(
            Some(5),
            Some(8),
            multiplicity * (v13),
        );
        stamper.stamp_current_const_local(
            Some(7),
            Some(8),
            multiplicity * (v13),
        );
        stamper.stamp_current_const_local(
            Some(5),
            Some(8),
            multiplicity * (v13),
        );
        stamper.stamp_current_const_local(
            Some(5),
            Some(8),
            multiplicity * (v13),
        );
        let d846_dn17: f64 = v44;
        stamper.stamp_current_node1_local(
            Some(17),
            None,
            multiplicity * (v846),
            17,
            multiplicity * (d846_dn17),
        );
        let d848_dn18: f64 = v44;
        stamper.stamp_current_node1_local(
            Some(18),
            None,
            multiplicity * (v848),
            18,
            multiplicity * (d848_dn18),
        );
        stamper.stamp_current_const_local(
            Some(11),
            Some(8),
            multiplicity * (v13),
        );
        stamper.stamp_current_const_local(
            Some(10),
            Some(5),
            multiplicity * (v13),
        );
        stamper.stamp_current_const_local(
            Some(11),
            Some(8),
            multiplicity * (v13),
        );
        stamper.stamp_current_const_local(
            Some(10),
            Some(5),
            multiplicity * (v13),
        );
        stamper.stamp_current_const_local(
            Some(3),
            None,
            multiplicity * (v864),
        );
        let d866_dn3: f64 = v2677;
        stamper.stamp_current_node1_local(
            Some(3),
            None,
            multiplicity * (v866),
            3,
            multiplicity * (d866_dn3),
        );
        let d870_dn3: f64 = v2679;
        stamper.stamp_current_node1_local(
            Some(3),
            None,
            multiplicity * (v870),
            3,
            multiplicity * (d870_dn3),
        );
        let d873_dn3: f64 = self.scalar_v2680;
        stamper.stamp_current_node1_local(
            Some(3),
            None,
            multiplicity * (v873),
            3,
            multiplicity * (d873_dn3),
        );
    }

    pub fn stamp_reactive(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedReactiveStamper<'_>) {
        let nodes = self.nodes;
        let branches = self.branches;
        self.ensure_temperature_static(ctx.temperature(), ctx.thermal_voltage());
        let p = &(*self.params);
        let multiplicity = self.multiplicity;
        let v1: f64 = ctx.node_voltage(nodes[8]);
        let v2: f64 = (ctx.node_voltage(nodes[12]) - v1);
        let v4: f64 = ctx.node_voltage(nodes[5]);
        let v5: f64 = (ctx.node_voltage(nodes[10]) - v4);
        let v7: f64 = (v4 - v1);
        let v8: f64 = ctx.node_voltage(nodes[11]);
        let v9: f64 = (v8 - v1);
        let v10: f64 = ctx.node_voltage(nodes[4]);
        let v13: f64 = 0.0;
        let v32: f64 = ctx.node_voltage(nodes[3]);
        let v35: f64 = (if (self.scalar_v31 != 0.0) { (self.scalar_v23 + ((v32) as f64).abs()) } else { self.scalar_v23 });
        let v39: f64 = (((v35 - self.scalar_v30)) as f64).abs();
        let v43: bool = ((v39 > v13) || self.scalar_v42);
        let v44: f64 = 1.0;
        let v46: f64 = ((v39) as f64).abs();
        let v83: f64 = (v44 + (v46 * self.scalar_v81));
        let v94: f64 = (v44 + (v39 * self.scalar_v92));
        let v134: bool = (!v43);
        let v137: f64 = (if v134 { self.scalar_v62 } else { (if v43 { (self.scalar_v62 * (v44 + (v46 * self.scalar_v63))) } else { v13 }) });
        let v138: f64 = (if v134 { self.scalar_v68 } else { (if v43 { (self.scalar_v68 * (v44 + (v46 * self.scalar_v69))) } else { v13 }) });
        let v139: f64 = (if v134 { self.scalar_v74 } else { (if v43 { (self.scalar_v74 * (v44 + (v46 * self.scalar_v75))) } else { v13 }) });
        let v140: f64 = (if v134 { self.scalar_v80 } else { (if v43 { (self.scalar_v80 * v83) } else { v13 }) });
        let v142: f64 = (if v134 { self.scalar_v125 } else { (if (v43 && self.scalar_v128) { (v83 * self.scalar_v125) } else { (if (v43 && self.scalar_v117) { ((v44 + (self.scalar_v81 * (v39 * v39))) * self.scalar_v125) } else { v13 }) }) });
        let v144: f64 = (if v134 { self.scalar_v91 } else { (if v43 { (self.scalar_v91 * v94) } else { v13 }) });
        let v145: f64 = (if v134 { self.scalar_v97 } else { (if v43 { (v94 * self.scalar_v97) } else { v13 }) });
        let v152: f64 = 0.5;
        let v161: f64 = (v7 * self.scalar_v160);
        let v162: f64 = ((v161) as f64).cosh();
        let v169: f64 = (1e-12 + (v162 * v162));
        let v175: f64 = (v44 + (v46 * self.scalar_v173));
        let v176: f64 = ((self.scalar_v165 * (v44 + (self.scalar_v166 / v169))) * v175);
        let v181: f64 = (self.scalar_v177 * (v44 + (v46 * self.scalar_v178)));
        let v186: f64 = (((v7 * self.scalar_v184)) as f64).tanh();
        let v191: f64 = ((-v5) - (if v134 { self.scalar_v105 } else { (if v43 { (self.scalar_v105 + (v39 * self.scalar_v106)) } else { v13 }) }));
        let v192: f64 = (self.scalar_v190 * v191);
        let v194: f64 = (((((if v134 { self.scalar_v86 } else { (if v43 { (self.scalar_v86 + (v39 * self.scalar_v87)) } else { v13 }) }) - self.scalar_v182) + (self.scalar_v182 * v186)) - ((v10 - v1) * self.scalar_v163)) - (v191 * v192));
        let v196: f64 = (v44 + (v46 * self.scalar_v87));
        let v197: f64 = (v194 * v196);
        let v198: f64 = (v2 - v197);
        let v199: f64 = (v198 * v198);
        let v204: f64 = (v181 * v198);
        let v206: f64 = (((v176 * v198) + (v199 * self.scalar_v201)) + (v199 * v204));
        let v207: f64 = ((v206) as f64).tanh();
        let v210: f64 = (-v206);
        let v214: f64 = (((v152 * ({ let limexp_arg = v206; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } } - { let limexp_arg = v210; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } }))) as f64).tanh();
        let v222: f64 = 2.0;
        let v239: f64 = (if self.scalar_v237 { (v5 - v197) } else { v162 });
        let v291: f64 = (if self.scalar_v290 { v198 } else { v239 });
        let v293: f64 = (if self.scalar_v290 { (v291 * v291) } else { (if self.scalar_v237 { (v239 * v239) } else { v198 }) });
        let v296: f64 = (v181 * v293);
        let v298: f64 = ((v291 + (self.scalar_v201 * v293)) + (v291 * v296));
        let v300: f64 = (if self.scalar_v290 { (v176 * v298) } else { v206 });
        let v302: f64 = (-v300);
        let v306: f64 = (((v152 * ({ let limexp_arg = v300; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } } - { let limexp_arg = v302; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } }))) as f64).tanh();
        let v331: f64 = (if self.scalar_v330 { v198 } else { v291 });
        let v333: f64 = (if self.scalar_v330 { (v331 * v331) } else { v293 });
        let v336: f64 = (v181 * v333);
        let v338: f64 = ((v331 + (self.scalar_v201 * v333)) + (v331 * v336));
        let v340: f64 = (if self.scalar_v330 { (v176 * v338) } else { v300 });
        let v352: f64 = (-v340);
        let v356: f64 = (((v152 * ({ let limexp_arg = v340; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } } - { let limexp_arg = v352; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } }))) as f64).tanh();
        let v427: f64 = (v44 + (v44 + v207));
        let v440: f64 = (v44 + (if self.scalar_v330 { (v44 + v356) } else { (if self.scalar_v290 { (v44 + v306) } else { (v44 + v214) }) }));
        let v443: f64 = (if self.scalar_v439 { (self.scalar_v426 + (v139 / v440)) } else { (if self.scalar_v425 { (self.scalar_v426 + (v139 / v427)) } else { v13 }) });
        let v456: f64 = -1.0;
        let v524: f64 = (v7 * self.scalar_v523);
        let v525: f64 = ((v144 + (v9 * self.scalar_v520)) + v524);
        let v526: f64 = ((v525) as f64).tanh();
        let v527: f64 = (v44 + v526);
        let v532: f64 = (((self.scalar_v528 + (v7 * self.scalar_v529))) as f64).tanh();
        let v533: f64 = (v44 + v532);
        let v538: f64 = (((self.scalar_v534 - (v7 * self.scalar_v535))) as f64).tanh();
        let v540: f64 = ((v44 + v538) - self.scalar_v523);
        let v544: f64 = ((v145 + (v5 * self.scalar_v541)) - v524);
        let v545: f64 = ((v544) as f64).tanh();
        let v546: f64 = (v44 + v545);
        let v557: f64 = (v137 * v527);
        let v571: f64 = (if self.scalar_v569 { (v533 - self.scalar_v523) } else { v533 });
        let v572: f64 = (v144 + v524);
        let v573: f64 = ((v572) as f64).cosh();
        let v574: f64 = (if self.scalar_v569 { v573 } else { v13 });
        let v576: f64 = (if self.scalar_v569 { ((v574) as f64).ln() } else { v13 });
        let v577: f64 = ((v525) as f64).cosh();
        let v578: f64 = (if self.scalar_v569 { v577 } else { v13 });
        let v580: f64 = (if self.scalar_v569 { ((v578) as f64).ln() } else { v13 });
        let v582: f64 = (if self.scalar_v569 { (v572 + v576) } else { v13 });
        let v584: f64 = ((v525 + v580) - v582);
        let v587: f64 = (v9 * self.scalar_v562);
        let v590: f64 = (v9 * self.scalar_v551);
        let v593: f64 = (v145 - v524);
        let v594: f64 = ((v593) as f64).cosh();
        let v595: f64 = (if self.scalar_v569 { v594 } else { v574 });
        let v597: f64 = (if self.scalar_v569 { ((v595) as f64).ln() } else { v13 });
        let v598: f64 = ((v544) as f64).cosh();
        let v599: f64 = (if self.scalar_v569 { v598 } else { v578 });
        let v601: f64 = (if self.scalar_v569 { ((v599) as f64).ln() } else { v13 });
        let v603: f64 = (if self.scalar_v569 { (v593 + v597) } else { v13 });
        let v605: f64 = ((v544 + v601) - v603);
        let v608: f64 = (v5 * self.scalar_v562);
        let v611: f64 = (v5 * self.scalar_v553);
        let v2250: f64 = ((v525) as f64).sinh();
        let v2253: f64 = (self.scalar_v520 * v2250);
        let v2256: f64 = (if self.scalar_v569 { v2253 } else { v13 });
        let v2262: f64 = (if self.scalar_v569 { (v2256 / v578) } else { v13 });
        let v2293: f64 = (if self.scalar_v569 { (self.scalar_v551 + (v137 * (self.scalar_v562 + ((v571 * (self.scalar_v520 + v2262)) / self.scalar_v520)))) } else { v13 });
        let v614: f64 = v2293;
        let v2303: f64 = ((v544) as f64).sinh();
        let v2306: f64 = (self.scalar_v541 * v2303);
        let v2309: f64 = (if self.scalar_v569 { v2306 } else { v13 });
        let v2317: f64 = (if self.scalar_v569 { (v2309 / v599) } else { v13 });
        let v2351: f64 = (if self.scalar_v569 { (self.scalar_v553 + (v138 * (self.scalar_v562 + ((v540 * (self.scalar_v541 + v2317)) / self.scalar_v541)))) } else { v13 });
        let v616: f64 = v2351;
        let v622: f64 = (v9 / self.scalar_v621);
        let v624: f64 = (if self.scalar_v620 { (v622 - v44) } else { v13 });
        let v627: f64 = (v624 * v624);
        let v628: f64 = (self.scalar_v626 + v627);
        let v630: f64 = f64::powf(v628, self.scalar_v629);
        let v634: f64 = (self.scalar_v626 + (v627 * self.scalar_v632));
        let v640: f64 = (((v144 + (self.scalar_v520 * (v9 + v524)))) as f64).tanh();
        let v643: f64 = (if self.scalar_v620 { v533 } else { v571 });
        let v645: f64 = (v538 + self.scalar_v644);
        let v646: f64 = (if self.scalar_v620 { v645 } else { v540 });
        let v651: f64 = (((v145 + (self.scalar_v541 * (v5 + (v7 * self.scalar_v644))))) as f64).tanh();
        let v653: f64 = (if self.scalar_v620 { (v44 + v651) } else { v546 });
        let v657: f64 = (v137 * ((if self.scalar_v620 { (v44 + v640) } else { v527 }) + ((if self.scalar_v620 { (v630 * v634) } else { v13 }) * self.scalar_v654)));
        let v665: f64 = (if self.scalar_v620 { (self.scalar_v553 + (v138 * (self.scalar_v562 + (v646 * v653)))) } else { (if self.scalar_v569 { v616 } else { (if self.scalar_v556 { (self.scalar_v553 + (v138 * ((v540 * v546) + self.scalar_v562))) } else { self.scalar_v554 }) }) });
        let v669: f64 = (if self.scalar_v668 { v573 } else { v595 });
        let v672: f64 = (if self.scalar_v668 { v577 } else { v599 });
        let v677: f64 = (self.scalar_v654 * (v9 + self.scalar_v621));
        let v678: f64 = (v456 + v622);
        let v680: f64 = (self.scalar_v626 + f64::powf(v678, v222));
        let v682: f64 = f64::powf(v680, self.scalar_v681);
        let v694: f64 = ((if self.scalar_v668 { (v677 * v682) } else { v13 }) + ((v525 + (if self.scalar_v668 { ((v672) as f64).ln() } else { v580 })) - (if self.scalar_v668 { (v572 + (if self.scalar_v668 { ((v669) as f64).ln() } else { v576 })) } else { v582 })));
        let v695: f64 = (v694 - self.scalar_v689);
        let v696: f64 = (v532 + self.scalar_v644);
        let v702: f64 = (if self.scalar_v668 { (v590 + (v137 * (v587 + ((v695 * v696) / self.scalar_v520)))) } else { (if self.scalar_v569 { ((v137 * (((v571 * v584) / self.scalar_v520) + v587)) + v590) } else { v13 }) });
        let v703: f64 = (if self.scalar_v668 { v594 } else { v669 });
        let v706: f64 = (if self.scalar_v668 { v598 } else { v672 });
        let v712: f64 = ((v544 + (if self.scalar_v668 { ((v706) as f64).ln() } else { v601 })) - (if self.scalar_v668 { (v593 + (if self.scalar_v668 { ((v703) as f64).ln() } else { v597 })) } else { v603 }));
        let v718: f64 = (if self.scalar_v668 { (v611 + (v138 * (v608 + ((v645 * v712) / self.scalar_v541)))) } else { (if self.scalar_v569 { ((v138 * (((v540 * v605) / self.scalar_v541) + v608)) + v611) } else { v13 }) });
        let v2455: f64 = (v222 * f64::powf(v678, v44));
        let v2460: f64 = (self.scalar_v681 * f64::powf(v680, self.scalar_v2458));
        let v2310: f64 = (if self.scalar_v569 { v13 } else { v2256 });
        let v2444: f64 = (if self.scalar_v668 { v2253 } else { v2310 });
        let v2489: f64 = (v696 * ((if self.scalar_v668 { ((self.scalar_v654 * v682) + (v677 * ((self.scalar_v2360 * v2455) * v2460))) } else { v13 }) + (self.scalar_v520 + (if self.scalar_v668 { (v2444 / v672) } else { v2262 }))));
        let v2505: f64 = (if self.scalar_v668 { (self.scalar_v551 + (v137 * (self.scalar_v562 + (v2489 / self.scalar_v520)))) } else { v2293 });
        let v719: f64 = v2505;
        let v720: f64 = (if self.scalar_v668 { v719 } else { (if self.scalar_v620 { (self.scalar_v551 + (v643 * v657)) } else { (if self.scalar_v569 { v614 } else { (if self.scalar_v556 { (self.scalar_v551 + (v533 * v557)) } else { self.scalar_v552 }) }) }) });
        let v2443: f64 = (if self.scalar_v668 { v13 } else { v2309 });
        let v2555: f64 = (if self.scalar_v668 { (self.scalar_v553 + (v138 * (self.scalar_v562 + ((v645 * (self.scalar_v541 + (if self.scalar_v668 { ((if self.scalar_v668 { v2306 } else { v2443 }) / v706) } else { v2317 }))) / self.scalar_v541)))) } else { v2351 });
        let v721: f64 = v2555;
        let v722: f64 = (if self.scalar_v668 { v721 } else { v665 });
        let v773: f64 = (self.scalar_v771 * ctx.node_voltage(nodes[15]));
        let v776: f64 = (self.scalar_v774 * ctx.branch_current(branches[0]));
        let v777: f64 = 0.0;
        let v778: f64 = (if self.scalar_v723 { v777 } else { v13 });
        let v779: f64 = 0.0;
        let v780: f64 = (if self.scalar_v723 { v779 } else { v13 });
        let v783: f64 = 0.0;
        let v784: f64 = (if self.scalar_v781 { v783 } else { v13 });
        let v786: f64 = 0.0;
        let v787: f64 = (if self.scalar_v781 { v786 } else { v13 });
        let v791: f64 = (self.scalar_v788 * (ctx.node_voltage(nodes[7]) - v4));
        let v793: f64 = (v7 * self.scalar_v792);
        let v796: f64 = (v140 * (ctx.node_voltage(nodes[6]) - v10));
        let v798: f64 = ctx.branch_current(branches[1]);
        let v801: f64 = 0.0;
        let v803: f64 = (if self.scalar_v725 { ((v443 * v798) + v801) } else { v13 });
        let v808: f64 = 0.0;
        let v809: f64 = (if self.scalar_v728 { v808 } else { v13 });
        let v813: f64 = (self.scalar_v810 * (v8 - ctx.node_voltage(nodes[14])));
        let v829: f64 = (self.scalar_v827 * ctx.branch_current(branches[10]));
        let v835: f64 = (self.scalar_v833 * ctx.branch_current(branches[14]));
        let v841: f64 = (self.scalar_v839 * ctx.branch_current(branches[18]));
        let v854: f64 = (-(if self.scalar_v747 { ((if self.scalar_v747 { ((v137 * ((v35 * 5.5226012e-23) * self.scalar_v752)) * self.scalar_v756) } else { v13 }) * 3.141592653589793) } else { v13 }));
        let v856: f64 = 0.0;
        let v857: f64 = (if self.scalar_v747 { v856 } else { v13 });
        let v869: f64 = 0.0;
        let v870: f64 = (if self.scalar_v769 { v869 } else { v13 });
        let v875: f64 = ((v161) as f64).sinh();
        let v876: f64 = (self.scalar_v160 * v875);
        let v877: f64 = (self.scalar_v874 * v875);
        let v879: f64 = (v162 * v876);
        let v881: f64 = (v162 * v877);
        let v885: f64 = (v169 * v169);
        let v892: f64 = (v175 * (self.scalar_v165 * ((-(self.scalar_v166 * (v879 + v879))) / v885)));
        let v893: f64 = (v175 * (self.scalar_v165 * ((-(self.scalar_v166 * (v881 + v881))) / v885)));
        let v896: f64 = (v44 - (v186 * v186));
        let v910: f64 = (v196 * ((self.scalar_v182 * (self.scalar_v184 * v896)) - (v192 + v192)));
        let v911: f64 = (v196 * ((self.scalar_v182 * (self.scalar_v894 * v896)) - self.scalar_v878));
        let v912: f64 = (v196 * (-((-v192) + (v191 * self.scalar_v902))));
        let v913: f64 = (-(v196 * self.scalar_v878));
        let v914: f64 = (-v910);
        let v915: f64 = (v456 - v911);
        let v916: f64 = (-v912);
        let v917: f64 = (v198 * v913);
        let v918: f64 = (v917 + v917);
        let v919: f64 = (v198 * v914);
        let v920: f64 = (v919 + v919);
        let v921: f64 = (v198 * v915);
        let v922: f64 = (v921 + v921);
        let v923: f64 = (v198 * v916);
        let v924: f64 = (v923 + v923);
        let v925: f64 = (v198 + v198);
        let v963: f64 = (((v176 * v913) + (self.scalar_v201 * v918)) + ((v204 * v918) + (v199 * (v181 * v913))));
        let v964: f64 = ((((v198 * v892) + (v176 * v914)) + (self.scalar_v201 * v920)) + ((v204 * v920) + (v199 * (v181 * v914))));
        let v965: f64 = ((((v198 * v893) + (v176 * v915)) + (self.scalar_v201 * v922)) + ((v204 * v922) + (v199 * (v181 * v915))));
        let v966: f64 = (((v176 * v916) + (self.scalar_v201 * v924)) + ((v204 * v924) + (v199 * (v181 * v916))));
        let v967: f64 = ((v176 + (self.scalar_v201 * v925)) + ((v204 * v925) + (v181 * v199)));
        let v969: f64 = (v44 - (v207 * v207));
        let v975: f64 = { let limexp_arg = v206; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v986: f64 = { let limexp_arg = v210; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v1003: f64 = (v44 - (v214 * v214));
        let v1074: f64 = (if self.scalar_v237 { v913 } else { v13 });
        let v1075: f64 = (if self.scalar_v237 { (v456 - v910) } else { v876 });
        let v1076: f64 = (if self.scalar_v237 { (-v911) } else { v877 });
        let v1077: f64 = (if self.scalar_v237 { (v44 - v912) } else { v13 });
        let v1078: f64 = (v239 * v1074);
        let v1080: f64 = (v239 * v1075);
        let v1082: f64 = (v239 * v1076);
        let v1084: f64 = (v239 * v1077);
        let v1322: f64 = (if self.scalar_v290 { v913 } else { v1074 });
        let v1323: f64 = (if self.scalar_v290 { v914 } else { v1075 });
        let v1324: f64 = (if self.scalar_v290 { v915 } else { v1076 });
        let v1325: f64 = (if self.scalar_v290 { v916 } else { v1077 });
        let v1327: f64 = (v291 * v1322);
        let v1329: f64 = (v291 * v1323);
        let v1331: f64 = (v291 * v1324);
        let v1333: f64 = (v291 * v1325);
        let v1335: f64 = (v291 * self.scalar_v1326);
        let v1337: f64 = (if self.scalar_v290 { (v1327 + v1327) } else { (if self.scalar_v237 { (v1078 + v1078) } else { v913 }) });
        let v1338: f64 = (if self.scalar_v290 { (v1329 + v1329) } else { (if self.scalar_v237 { (v1080 + v1080) } else { v914 }) });
        let v1339: f64 = (if self.scalar_v290 { (v1331 + v1331) } else { (if self.scalar_v237 { (v1082 + v1082) } else { v915 }) });
        let v1340: f64 = (if self.scalar_v290 { (v1333 + v1333) } else { (if self.scalar_v237 { (v1084 + v1084) } else { v916 }) });
        let v1341: f64 = (if self.scalar_v290 { (v1335 + v1335) } else { self.scalar_v1090 });
        let v1386: f64 = (if self.scalar_v290 { (v176 * ((v1322 + (self.scalar_v201 * v1337)) + ((v296 * v1322) + (v291 * (v181 * v1337))))) } else { v963 });
        let v1387: f64 = (if self.scalar_v290 { ((v298 * v892) + (v176 * ((v1323 + (self.scalar_v201 * v1338)) + ((v296 * v1323) + (v291 * (v181 * v1338)))))) } else { v964 });
        let v1388: f64 = (if self.scalar_v290 { ((v298 * v893) + (v176 * ((v1324 + (self.scalar_v201 * v1339)) + ((v296 * v1324) + (v291 * (v181 * v1339)))))) } else { v965 });
        let v1389: f64 = (if self.scalar_v290 { (v176 * ((v1325 + (self.scalar_v201 * v1340)) + ((v296 * v1325) + (v291 * (v181 * v1340))))) } else { v966 });
        let v1390: f64 = (if self.scalar_v290 { (v176 * ((self.scalar_v1326 + (self.scalar_v201 * v1341)) + ((v296 * self.scalar_v1326) + (v291 * (v181 * v1341))))) } else { v967 });
        let v1391: f64 = { let limexp_arg = v300; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v1402: f64 = { let limexp_arg = v302; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v1419: f64 = (v44 - (v306 * v306));
        let v1425: f64 = (if self.scalar_v290 { ((v152 * ((v1386 * v1391) - ((-v1386) * v1402))) * v1419) } else { ((v152 * ((v963 * v975) - ((-v963) * v986))) * v1003) });
        let v1426: f64 = (if self.scalar_v290 { ((v152 * ((v1387 * v1391) - ((-v1387) * v1402))) * v1419) } else { ((v152 * ((v964 * v975) - ((-v964) * v986))) * v1003) });
        let v1427: f64 = (if self.scalar_v290 { ((v152 * ((v1388 * v1391) - ((-v1388) * v1402))) * v1419) } else { ((v152 * ((v965 * v975) - ((-v965) * v986))) * v1003) });
        let v1428: f64 = (if self.scalar_v290 { ((v152 * ((v1389 * v1391) - ((-v1389) * v1402))) * v1419) } else { ((v152 * ((v966 * v975) - ((-v966) * v986))) * v1003) });
        let v1429: f64 = (if self.scalar_v290 { ((v152 * ((v1390 * v1391) - ((-v1390) * v1402))) * v1419) } else { ((v152 * ((v967 * v975) - ((-v967) * v986))) * v1003) });
        let v1525: f64 = (if self.scalar_v330 { v913 } else { v1322 });
        let v1526: f64 = (if self.scalar_v330 { v914 } else { v1323 });
        let v1527: f64 = (if self.scalar_v330 { v915 } else { v1324 });
        let v1528: f64 = (if self.scalar_v330 { v916 } else { v1325 });
        let v1530: f64 = (v331 * v1525);
        let v1532: f64 = (v331 * v1526);
        let v1534: f64 = (v331 * v1527);
        let v1536: f64 = (v331 * v1528);
        let v1538: f64 = (v331 * self.scalar_v1529);
        let v1540: f64 = (if self.scalar_v330 { (v1530 + v1530) } else { v1337 });
        let v1541: f64 = (if self.scalar_v330 { (v1532 + v1532) } else { v1338 });
        let v1542: f64 = (if self.scalar_v330 { (v1534 + v1534) } else { v1339 });
        let v1543: f64 = (if self.scalar_v330 { (v1536 + v1536) } else { v1340 });
        let v1544: f64 = (if self.scalar_v330 { (v1538 + v1538) } else { v1341 });
        let v1589: f64 = (if self.scalar_v330 { (v176 * ((v1525 + (self.scalar_v201 * v1540)) + ((v336 * v1525) + (v331 * (v181 * v1540))))) } else { v1386 });
        let v1590: f64 = (if self.scalar_v330 { ((v338 * v892) + (v176 * ((v1526 + (self.scalar_v201 * v1541)) + ((v336 * v1526) + (v331 * (v181 * v1541)))))) } else { v1387 });
        let v1591: f64 = (if self.scalar_v330 { ((v338 * v893) + (v176 * ((v1527 + (self.scalar_v201 * v1542)) + ((v336 * v1527) + (v331 * (v181 * v1542)))))) } else { v1388 });
        let v1592: f64 = (if self.scalar_v330 { (v176 * ((v1528 + (self.scalar_v201 * v1543)) + ((v336 * v1528) + (v331 * (v181 * v1543))))) } else { v1389 });
        let v1593: f64 = (if self.scalar_v330 { (v176 * ((self.scalar_v1529 + (self.scalar_v201 * v1544)) + ((v336 * self.scalar_v1529) + (v331 * (v181 * v1544))))) } else { v1390 });
        let v1663: f64 = { let limexp_arg = v340; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v1674: f64 = { let limexp_arg = v352; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v1691: f64 = (v44 - (v356 * v356));
        let v2054: f64 = (v427 * v427);
        let v2085: f64 = (v440 * v440);
        let v2099: f64 = (if self.scalar_v439 { ((-(v139 * (if self.scalar_v330 { ((v152 * ((v1589 * v1663) - ((-v1589) * v1674))) * v1691) } else { v1425 }))) / v2085) } else { (if self.scalar_v425 { ((-(v139 * (v963 * v969))) / v2054) } else { v13 }) });
        let v2100: f64 = (if self.scalar_v439 { ((-(v139 * (if self.scalar_v330 { ((v152 * ((v1590 * v1663) - ((-v1590) * v1674))) * v1691) } else { v1426 }))) / v2085) } else { (if self.scalar_v425 { ((-(v139 * (v964 * v969))) / v2054) } else { v13 }) });
        let v2101: f64 = (if self.scalar_v439 { ((-(v139 * (if self.scalar_v330 { ((v152 * ((v1591 * v1663) - ((-v1591) * v1674))) * v1691) } else { v1427 }))) / v2085) } else { (if self.scalar_v425 { ((-(v139 * (v965 * v969))) / v2054) } else { v13 }) });
        let v2102: f64 = (if self.scalar_v439 { ((-(v139 * (if self.scalar_v330 { ((v152 * ((v1592 * v1663) - ((-v1592) * v1674))) * v1691) } else { v1428 }))) / v2085) } else { (if self.scalar_v425 { ((-(v139 * (v966 * v969))) / v2054) } else { v13 }) });
        let v2103: f64 = (if self.scalar_v439 { ((-(v139 * (if self.scalar_v330 { ((v152 * ((v1593 * v1663) - ((-v1593) * v1674))) * v1691) } else { v1429 }))) / v2085) } else { (if self.scalar_v425 { ((-(v139 * (v967 * v969))) / v2054) } else { v13 }) });
        let v2194: f64 = (v44 - (v526 * v526));
        let v2195: f64 = (self.scalar_v523 * v2194);
        let v2196: f64 = (self.scalar_v2192 * v2194);
        let v2197: f64 = (self.scalar_v520 * v2194);
        let v2200: f64 = (v44 - (v532 * v532));
        let v2201: f64 = (self.scalar_v529 * v2200);
        let v2202: f64 = (self.scalar_v2198 * v2200);
        let v2205: f64 = (v44 - (v538 * v538));
        let v2206: f64 = (self.scalar_v2203 * v2205);
        let v2207: f64 = (self.scalar_v535 * v2205);
        let v2211: f64 = (v44 - (v545 * v545));
        let v2212: f64 = (self.scalar_v2209 * v2211);
        let v2213: f64 = (self.scalar_v523 * v2211);
        let v2214: f64 = (self.scalar_v541 * v2211);
        let v2241: f64 = ((v572) as f64).sinh();
        let v2242: f64 = (self.scalar_v523 * v2241);
        let v2243: f64 = (self.scalar_v2191 * v2241);
        let v2244: f64 = (if self.scalar_v569 { v2242 } else { v13 });
        let v2245: f64 = (if self.scalar_v569 { v2243 } else { v13 });
        let v2248: f64 = (if self.scalar_v569 { (v2244 / v574) } else { v13 });
        let v2249: f64 = (if self.scalar_v569 { (v2245 / v574) } else { v13 });
        let v2251: f64 = (self.scalar_v523 * v2250);
        let v2252: f64 = (self.scalar_v2192 * v2250);
        let v2254: f64 = (if self.scalar_v569 { v2251 } else { v13 });
        let v2255: f64 = (if self.scalar_v569 { v2252 } else { v13 });
        let v2260: f64 = (if self.scalar_v569 { (v2254 / v578) } else { v13 });
        let v2261: f64 = (if self.scalar_v569 { (v2255 / v578) } else { v13 });
        let v2265: f64 = (if self.scalar_v569 { (self.scalar_v523 + v2248) } else { v13 });
        let v2266: f64 = (if self.scalar_v569 { (self.scalar_v2191 + v2249) } else { v13 });
        let v2294: f64 = ((v593) as f64).sinh();
        let v2295: f64 = (self.scalar_v2191 * v2294);
        let v2296: f64 = (self.scalar_v523 * v2294);
        let v2297: f64 = (if self.scalar_v569 { v2295 } else { v2244 });
        let v2298: f64 = (if self.scalar_v569 { v2296 } else { v2245 });
        let v2301: f64 = (if self.scalar_v569 { (v2297 / v595) } else { v13 });
        let v2302: f64 = (if self.scalar_v569 { (v2298 / v595) } else { v13 });
        let v2304: f64 = (self.scalar_v2209 * v2303);
        let v2305: f64 = (self.scalar_v523 * v2303);
        let v2307: f64 = (if self.scalar_v569 { v2304 } else { v2254 });
        let v2308: f64 = (if self.scalar_v569 { v2305 } else { v2255 });
        let v2315: f64 = (if self.scalar_v569 { (v2307 / v599) } else { v13 });
        let v2316: f64 = (if self.scalar_v569 { (v2308 / v599) } else { v13 });
        let v2318: f64 = (if self.scalar_v569 { (v2310 / v599) } else { v13 });
        let v2321: f64 = (if self.scalar_v569 { (self.scalar_v2191 + v2301) } else { v13 });
        let v2322: f64 = (if self.scalar_v569 { (self.scalar_v523 + v2302) } else { v13 });
        let v2363: f64 = (v624 * self.scalar_v2361);
        let v2364: f64 = (v2363 + v2363);
        let v2365: f64 = (v624 * self.scalar_v2362);
        let v2366: f64 = (v2365 + v2365);
        let v2369: f64 = (self.scalar_v629 * f64::powf(v628, self.scalar_v2367));
        let v2386: f64 = (v44 - (v640 * v640));
        let v2398: f64 = (v44 - (v651 * v651));
        let v2410: f64 = (v137 * ((if self.scalar_v620 { (self.scalar_v2384 * v2386) } else { v2196 }) + (self.scalar_v654 * (if self.scalar_v620 { ((v634 * (v2364 * v2369)) + (v630 * (self.scalar_v632 * v2364))) } else { v13 }))));
        let v2411: f64 = (v137 * ((if self.scalar_v620 { (self.scalar_v520 * v2386) } else { v2197 }) + (self.scalar_v654 * (if self.scalar_v620 { ((v634 * (v2366 * v2369)) + (v630 * (self.scalar_v632 * v2366))) } else { v13 }))));
        let v2419: f64 = (if self.scalar_v620 { ((v657 * v2201) + (v643 * (v137 * (if self.scalar_v620 { (self.scalar_v2383 * v2386) } else { v2195 })))) } else { (if self.scalar_v569 { v13 } else { (if self.scalar_v556 { ((v557 * v2201) + (v533 * (v137 * v2195))) } else { v13 }) }) });
        let v2432: f64 = (if self.scalar_v620 { (v138 * ((v653 * v2206) + (v646 * (if self.scalar_v620 { (self.scalar_v2395 * v2398) } else { v2212 })))) } else { (if self.scalar_v569 { v13 } else { (if self.scalar_v556 { (v138 * ((v546 * v2206) + (v540 * v2212))) } else { v13 }) }) });
        let v2433: f64 = (if self.scalar_v620 { (v138 * ((v653 * v2207) + (v646 * (if self.scalar_v620 { (self.scalar_v2396 * v2398) } else { v2213 })))) } else { (if self.scalar_v569 { v13 } else { (if self.scalar_v556 { (v138 * ((v546 * v2207) + (v540 * v2213))) } else { v13 }) }) });
        let v2435: f64 = (if self.scalar_v668 { v2242 } else { v2297 });
        let v2436: f64 = (if self.scalar_v668 { v2243 } else { v2298 });
        let v2441: f64 = (if self.scalar_v668 { v2251 } else { v2307 });
        let v2442: f64 = (if self.scalar_v668 { v2252 } else { v2308 });
        let v2480: f64 = ((if self.scalar_v668 { ((v682 * self.scalar_v2453) + (v677 * ((self.scalar_v2359 * v2455) * v2460))) } else { v13 }) + ((self.scalar_v2192 + (if self.scalar_v668 { (v2442 / v672) } else { v2261 })) - (if self.scalar_v668 { (self.scalar_v2191 + (if self.scalar_v668 { (v2436 / v669) } else { v2249 })) } else { v2266 })));
        let v2484: f64 = ((v696 * ((self.scalar_v523 + (if self.scalar_v668 { (v2441 / v672) } else { v2260 })) - (if self.scalar_v668 { (self.scalar_v523 + (if self.scalar_v668 { (v2435 / v669) } else { v2248 })) } else { v2265 }))) + (v695 * v2201));
        let v2502: f64 = (if self.scalar_v668 { (v137 * (v2484 / self.scalar_v520)) } else { (if self.scalar_v569 { (v137 * (((v584 * v2201) + (v571 * ((self.scalar_v523 + v2260) - v2265))) / self.scalar_v520)) } else { v13 }) });
        let v2503: f64 = (if self.scalar_v668 { (self.scalar_v2288 + (v137 * (self.scalar_v2282 + (((v696 * v2480) + (v695 * v2202)) / self.scalar_v520)))) } else { (if self.scalar_v569 { ((v137 * ((((v584 * v2202) + (v571 * ((self.scalar_v2192 + v2261) - v2266))) / self.scalar_v520) + self.scalar_v2282)) + self.scalar_v2288) } else { v13 }) });
        let v2531: f64 = ((self.scalar_v2209 + (if self.scalar_v668 { ((if self.scalar_v668 { v2304 } else { v2441 }) / v706) } else { v2315 })) - (if self.scalar_v668 { (self.scalar_v2191 + (if self.scalar_v668 { ((if self.scalar_v668 { v2295 } else { v2435 }) / v703) } else { v2301 })) } else { v2321 }));
        let v2532: f64 = ((self.scalar_v523 + (if self.scalar_v668 { ((if self.scalar_v668 { v2305 } else { v2442 }) / v706) } else { v2316 })) - (if self.scalar_v668 { (self.scalar_v523 + (if self.scalar_v668 { ((if self.scalar_v668 { v2296 } else { v2436 }) / v703) } else { v2302 })) } else { v2322 }));
        let v2553: f64 = (if self.scalar_v668 { (self.scalar_v2346 + (v138 * (self.scalar_v2282 + (((v712 * v2206) + (v645 * v2531)) / self.scalar_v541)))) } else { (if self.scalar_v569 { ((v138 * (self.scalar_v2282 + (((v605 * v2206) + (v540 * ((self.scalar_v2209 + v2315) - v2321))) / self.scalar_v541))) + self.scalar_v2346) } else { v13 }) });
        let v2554: f64 = (if self.scalar_v668 { (v138 * (((v712 * v2207) + (v645 * v2532)) / self.scalar_v541)) } else { (if self.scalar_v569 { (v138 * (((v605 * v2207) + (v540 * ((self.scalar_v523 + v2316) - v2322))) / self.scalar_v541)) } else { v13 }) });
        let v2556: f64 = (if self.scalar_v668 { (v138 * ((v645 * (if self.scalar_v668 { ((if self.scalar_v668 { v13 } else { v2444 }) / v706) } else { v2318 })) / self.scalar_v541)) } else { (if self.scalar_v569 { (v138 * ((v540 * v2318) / self.scalar_v541)) } else { v13 }) });
        let v2558: f64 = (if self.scalar_v668 { v13 } else { (if self.scalar_v620 { ((v657 * v2202) + (v643 * v2410)) } else { (if self.scalar_v569 { v13 } else { (if self.scalar_v556 { ((v557 * v2202) + (v533 * (v137 * v2196))) } else { v13 }) }) }) });
        let v2562: f64 = (if self.scalar_v668 { v13 } else { (if self.scalar_v620 { (v138 * (v646 * (if self.scalar_v620 { (self.scalar_v541 * v2398) } else { v2214 }))) } else { (if self.scalar_v569 { v13 } else { (if self.scalar_v556 { (v138 * (v540 * v2214)) } else { v13 }) }) }) });
        let v2568: f64 = 1.0;
        let v2573: f64 = (if self.scalar_v723 { (v2553 * v2568) } else { v13 });
        let v2574: f64 = (if self.scalar_v723 { (v2554 * v2568) } else { v13 });
        let v2575: f64 = (if self.scalar_v723 { (v2555 * v2568) } else { v13 });
        let v2576: f64 = (if self.scalar_v723 { (v2556 * v2568) } else { v13 });
        let v2581: f64 = (if self.scalar_v723 { (v2502 * v2568) } else { v13 });
        let v2582: f64 = (if self.scalar_v723 { (v2503 * v2568) } else { v13 });
        let v2583: f64 = (if self.scalar_v723 { ((if self.scalar_v668 { (v137 * ((v696 * (if self.scalar_v668 { (v2443 / v672) } else { v13 })) / self.scalar_v520)) } else { v13 }) * v2568) } else { v13 });
        let v2584: f64 = (if self.scalar_v723 { (v2505 * v2568) } else { v13 });
        let v2594: f64 = (if self.scalar_v781 { (v2568 * ((-v722) + (v5 * (if self.scalar_v668 { v13 } else { v2432 })))) } else { v13 });
        let v2595: f64 = (if self.scalar_v781 { (v2568 * (v5 * (if self.scalar_v668 { v13 } else { v2433 }))) } else { v13 });
        let v2596: f64 = (if self.scalar_v781 { (v2568 * (v722 + (v5 * v2562))) } else { v13 });
        let v2605: f64 = (v2568 * (v720 + (v9 * (if self.scalar_v668 { v13 } else { (if self.scalar_v620 { (v643 * v2411) } else { (if self.scalar_v569 { v13 } else { (if self.scalar_v556 { (v533 * (v137 * v2197)) } else { v13 }) }) }) }))));
        let v2606: f64 = (if self.scalar_v781 { (v2568 * (v9 * (if self.scalar_v668 { v13 } else { v2419 }))) } else { v13 });
        let v2607: f64 = (if self.scalar_v781 { (v2568 * ((-v720) + (v9 * v2558))) } else { v13 });
        let v2608: f64 = (if self.scalar_v781 { v2605 } else { v13 });
        let v2611: f64 = (-v140);
        let v2620: f64 = (if self.scalar_v725 { (v798 * v2099) } else { v13 });
        let v2621: f64 = (if self.scalar_v725 { (v798 * v2100) } else { v13 });
        let v2622: f64 = (if self.scalar_v725 { (v798 * v2101) } else { v13 });
        let v2623: f64 = (if self.scalar_v725 { (v798 * v2102) } else { v13 });
        let v2624: f64 = (if self.scalar_v725 { (v798 * v2103) } else { v13 });
        let v2625: f64 = (if self.scalar_v725 { (v443 + (self.scalar_v724 * v2568)) } else { v13 });
        let v2633: f64 = (if self.scalar_v728 { (v2568 * (-v142)) } else { v13 });
        let v2634: f64 = (if self.scalar_v728 { (v142 * v2568) } else { v13 });
        let v2675: f64 = (if self.scalar_v747 { (v854 * v2568) } else { v13 });
        let v2679: f64 = (if self.scalar_v769 { (self.scalar_v867 * v2568) } else { v13 });

        let d773_dn15: f64 = self.scalar_v771;
        stamper.stamp_current_reactive_node1(
            Some(nodes[15]),
            None,
            nodes[15],
            multiplicity * (d773_dn15),
        );
        let d776_db0: f64 = self.scalar_v774;
        stamper.stamp_current_reactive_branch1(
            Some(nodes[15]),
            Some(nodes[16]),
            branches[0],
            multiplicity * (d776_db0),
        );
        let d778_dn5: f64 = v2573;
        let d778_dn8: f64 = v2574;
        let d778_dn10: f64 = v2575;
        let d778_dn11: f64 = v2576;
        let v778_reactive_nodes: [usize; 4] = [nodes[5], nodes[8], nodes[10], nodes[11]];
        let v778_reactive_node_derivatives: [f64; 4] = [d778_dn5, d778_dn8, d778_dn10, d778_dn11];
        stamper.stamp_current_reactive_dense(
            Some(nodes[10]),
            Some(nodes[5]),
            &v778_reactive_nodes,
            &v778_reactive_node_derivatives,
            &[],
            &[],
            multiplicity,
        );
        let d780_dn5: f64 = v2581;
        let d780_dn8: f64 = v2582;
        let d780_dn10: f64 = v2583;
        let d780_dn11: f64 = v2584;
        let v780_reactive_nodes: [usize; 4] = [nodes[5], nodes[8], nodes[10], nodes[11]];
        let v780_reactive_node_derivatives: [f64; 4] = [d780_dn5, d780_dn8, d780_dn10, d780_dn11];
        stamper.stamp_current_reactive_dense(
            Some(nodes[11]),
            Some(nodes[8]),
            &v780_reactive_nodes,
            &v780_reactive_node_derivatives,
            &[],
            &[],
            multiplicity,
        );
        let d784_dn5: f64 = v2594;
        let d784_dn8: f64 = v2595;
        let d784_dn10: f64 = v2596;
        stamper.stamp_current_reactive_node3(
            Some(nodes[10]),
            Some(nodes[5]),
            nodes[5],
            multiplicity * (d784_dn5),
            nodes[8],
            multiplicity * (d784_dn8),
            nodes[10],
            multiplicity * (d784_dn10),
        );
        let d787_dn5: f64 = v2606;
        let d787_dn8: f64 = v2607;
        let d787_dn11: f64 = v2608;
        stamper.stamp_current_reactive_node3(
            Some(nodes[11]),
            Some(nodes[8]),
            nodes[5],
            multiplicity * (d787_dn5),
            nodes[8],
            multiplicity * (d787_dn8),
            nodes[11],
            multiplicity * (d787_dn11),
        );
        let d791_dn5: f64 = self.scalar_v2609;
        let d791_dn7: f64 = self.scalar_v788;
        stamper.stamp_current_reactive_node2(
            Some(nodes[7]),
            Some(nodes[5]),
            nodes[5],
            multiplicity * (d791_dn5),
            nodes[7],
            multiplicity * (d791_dn7),
        );
        let d793_dn5: f64 = self.scalar_v792;
        let d793_dn8: f64 = self.scalar_v2610;
        stamper.stamp_current_reactive_node2(
            Some(nodes[5]),
            Some(nodes[8]),
            nodes[5],
            multiplicity * (d793_dn5),
            nodes[8],
            multiplicity * (d793_dn8),
        );
        let d796_dn4: f64 = v2611;
        let d796_dn6: f64 = v140;
        stamper.stamp_current_reactive_node2(
            Some(nodes[6]),
            Some(nodes[4]),
            nodes[4],
            multiplicity * (d796_dn4),
            nodes[6],
            multiplicity * (d796_dn6),
        );
        let d803_dn4: f64 = v2620;
        let d803_dn5: f64 = v2621;
        let d803_dn8: f64 = v2622;
        let d803_dn10: f64 = v2623;
        let d803_dn12: f64 = v2624;
        let d803_db1: f64 = v2625;
        let v803_reactive_nodes: [usize; 5] = [nodes[4], nodes[5], nodes[8], nodes[10], nodes[12]];
        let v803_reactive_node_derivatives: [f64; 5] = [d803_dn4, d803_dn5, d803_dn8, d803_dn10, d803_dn12];
        let v803_reactive_branches: [usize; 1] = [branches[1]];
        let v803_reactive_branch_derivatives: [f64; 1] = [d803_db1];
        stamper.stamp_current_reactive_dense(
            Some(nodes[4]),
            Some(nodes[8]),
            &v803_reactive_nodes,
            &v803_reactive_node_derivatives,
            &v803_reactive_branches,
            &v803_reactive_branch_derivatives,
            multiplicity,
        );
        let d809_dn8: f64 = v2633;
        let d809_dn12: f64 = v2634;
        stamper.stamp_current_reactive_node2(
            Some(nodes[12]),
            Some(nodes[8]),
            nodes[8],
            multiplicity * (d809_dn8),
            nodes[12],
            multiplicity * (d809_dn12),
        );
        let d813_dn11: f64 = self.scalar_v810;
        let d813_dn14: f64 = self.scalar_v2635;
        stamper.stamp_current_reactive_node2(
            Some(nodes[11]),
            Some(nodes[14]),
            nodes[11],
            multiplicity * (d813_dn11),
            nodes[14],
            multiplicity * (d813_dn14),
        );
        let d829_db10: f64 = self.scalar_v827;
        stamper.stamp_current_reactive_branch1(
            Some(nodes[1]),
            Some(nodes[7]),
            branches[10],
            multiplicity * (d829_db10),
        );
        let d835_db14: f64 = self.scalar_v833;
        stamper.stamp_current_reactive_branch1(
            Some(nodes[9]),
            Some(nodes[2]),
            branches[14],
            multiplicity * (d835_db14),
        );
        let d841_db18: f64 = self.scalar_v839;
        stamper.stamp_current_reactive_branch1(
            Some(nodes[6]),
            Some(nodes[0]),
            branches[18],
            multiplicity * (d841_db18),
        );
        let d857_dn17: f64 = v2675;
        stamper.stamp_current_reactive_node1(
            Some(nodes[7]),
            Some(nodes[5]),
            nodes[17],
            multiplicity * (d857_dn17),
        );
        let d870_dn3: f64 = v2679;
        stamper.stamp_current_reactive_node1(
            Some(nodes[3]),
            None,
            nodes[3],
            multiplicity * (d870_dn3),
        );
    }
}
