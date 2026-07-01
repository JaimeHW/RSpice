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
        let v33: f64 = ((v32) as f64).abs();
        let v34: f64 = (self.scalar_v23 + v33);
        let v35: f64 = (if (self.scalar_v31 != 0.0) { v34 } else { self.scalar_v23 });
        let v36: f64 = 8.617333262145179e-5;
        let v37: f64 = (v35 * v36);
        let v38: f64 = (v35 - self.scalar_v30);
        let v39: f64 = ((v38) as f64).abs();
        let v40: bool = (v39 > v13);
        let v43: bool = (v40 || self.scalar_v42);
        let v44: f64 = 1.0;
        let v46: f64 = ((v39) as f64).abs();
        let v47: f64 = (self.scalar_v45 * v46);
        let v48: f64 = (v44 + v47);
        let v49: f64 = (self.scalar_v41 * v48);
        let v52: f64 = (v46 * self.scalar_v51);
        let v53: f64 = (v44 + v52);
        let v54: f64 = (self.scalar_v50 * v53);
        let v55: f64 = (if v43 { v54 } else { v13 });
        let v58: f64 = (v46 * self.scalar_v57);
        let v59: f64 = (v44 + v58);
        let v60: f64 = (self.scalar_v56 * v59);
        let v61: f64 = (if v43 { v60 } else { v13 });
        let v64: f64 = (v46 * self.scalar_v63);
        let v65: f64 = (v44 + v64);
        let v66: f64 = (self.scalar_v62 * v65);
        let v67: f64 = (if v43 { v66 } else { v13 });
        let v70: f64 = (v46 * self.scalar_v69);
        let v71: f64 = (v44 + v70);
        let v72: f64 = (self.scalar_v68 * v71);
        let v73: f64 = (if v43 { v72 } else { v13 });
        let v76: f64 = (v46 * self.scalar_v75);
        let v77: f64 = (v44 + v76);
        let v78: f64 = (self.scalar_v74 * v77);
        let v79: f64 = (if v43 { v78 } else { v13 });
        let v82: f64 = (v46 * self.scalar_v81);
        let v83: f64 = (v44 + v82);
        let v84: f64 = (self.scalar_v80 * v83);
        let v85: f64 = (if v43 { v84 } else { v13 });
        let v88: f64 = (v39 * self.scalar_v87);
        let v89: f64 = (self.scalar_v86 + v88);
        let v90: f64 = (if v43 { v89 } else { v13 });
        let v93: f64 = (v39 * self.scalar_v92);
        let v94: f64 = (v44 + v93);
        let v95: f64 = (self.scalar_v91 * v94);
        let v96: f64 = (if v43 { v95 } else { v13 });
        let v98: f64 = (v94 * self.scalar_v97);
        let v99: f64 = (if v43 { v98 } else { v13 });
        let v102: f64 = (v39 * self.scalar_v101);
        let v103: f64 = (self.scalar_v100 + v102);
        let v104: f64 = (if v43 { v103 } else { v13 });
        let v107: f64 = (v39 * self.scalar_v106);
        let v108: f64 = (self.scalar_v105 + v107);
        let v109: f64 = (if v43 { v108 } else { v13 });
        let v118: bool = (v43 && self.scalar_v117);
        let v120: f64 = (v39 * v39);
        let v121: f64 = (self.scalar_v81 * v120);
        let v122: f64 = (v44 + v121);
        let v123: f64 = (self.scalar_v119 * v122);
        let v124: f64 = (if v118 { v123 } else { v13 });
        let v126: f64 = (v122 * self.scalar_v125);
        let v127: f64 = (if v118 { v126 } else { v13 });
        let v129: bool = (v43 && self.scalar_v128);
        let v130: f64 = (v83 * self.scalar_v119);
        let v131: f64 = (if v129 { v130 } else { v124 });
        let v132: f64 = (v83 * self.scalar_v125);
        let v133: f64 = (if v129 { v132 } else { v127 });
        let v134: bool = (!v43);
        let v135: f64 = (if v134 { self.scalar_v50 } else { v55 });
        let v136: f64 = (if v134 { self.scalar_v56 } else { v61 });
        let v137: f64 = (if v134 { self.scalar_v62 } else { v67 });
        let v138: f64 = (if v134 { self.scalar_v68 } else { v73 });
        let v139: f64 = (if v134 { self.scalar_v74 } else { v79 });
        let v140: f64 = (if v134 { self.scalar_v80 } else { v85 });
        let v141: f64 = (if v134 { self.scalar_v119 } else { v131 });
        let v142: f64 = (if v134 { self.scalar_v125 } else { v133 });
        let v143: f64 = (if v134 { self.scalar_v86 } else { v90 });
        let v144: f64 = (if v134 { self.scalar_v91 } else { v96 });
        let v145: f64 = (if v134 { self.scalar_v97 } else { v99 });
        let v146: f64 = (if v134 { self.scalar_v100 } else { v104 });
        let v147: f64 = (if v134 { self.scalar_v105 } else { v109 });
        let v152: f64 = 0.5;
        let v155: f64 = (self.scalar_v154 / v37);
        let v156: f64 = (if self.scalar_v151 { v155 } else { v13 });
        let v159: f64 = (if self.scalar_v157 { self.scalar_v158 } else { v156 });
        let v161: f64 = (v7 * self.scalar_v160);
        let v162: f64 = ((v161) as f64).cosh();
        let v164: f64 = (v11 * self.scalar_v163);
        let v167: f64 = 1e-12;
        let v168: f64 = (v162 * v162);
        let v169: f64 = (v167 + v168);
        let v170: f64 = (self.scalar_v166 / v169);
        let v171: f64 = (v44 + v170);
        let v172: f64 = (self.scalar_v165 * v171);
        let v174: f64 = (v46 * self.scalar_v173);
        let v175: f64 = (v44 + v174);
        let v176: f64 = (v172 * v175);
        let v179: f64 = (v46 * self.scalar_v178);
        let v180: f64 = (v44 + v179);
        let v181: f64 = (self.scalar_v177 * v180);
        let v183: f64 = (v143 - self.scalar_v182);
        let v185: f64 = (v7 * self.scalar_v184);
        let v186: f64 = ((v185) as f64).tanh();
        let v187: f64 = (self.scalar_v182 * v186);
        let v188: f64 = (v183 + v187);
        let v189: f64 = (v188 - v164);
        let v191: f64 = (v6 - v147);
        let v192: f64 = (self.scalar_v190 * v191);
        let v193: f64 = (v191 * v192);
        let v194: f64 = (v189 - v193);
        let v195: f64 = (v46 * self.scalar_v87);
        let v196: f64 = (v44 + v195);
        let v197: f64 = (v194 * v196);
        let v198: f64 = (v2 - v197);
        let v199: f64 = (v198 * v198);
        let v200: f64 = (v176 * v198);
        let v202: f64 = (v199 * self.scalar_v201);
        let v203: f64 = (v200 + v202);
        let v204: f64 = (v181 * v198);
        let v205: f64 = (v199 * v204);
        let v206: f64 = (v203 + v205);
        let v207: f64 = ((v206) as f64).tanh();
        let v208: f64 = (v44 + v207);
        let v209: f64 = { let limexp_arg = v206; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let v210: f64 = (-v206);
        let v211: f64 = { let limexp_arg = v210; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let v212: f64 = (v209 - v211);
        let v213: f64 = (v152 * v212);
        let v214: f64 = ((v213) as f64).tanh();
        let v215: f64 = (v44 + v214);
        let v217: f64 = (self.scalar_v184 * v208);
        let v218: f64 = (self.scalar_v216 + v217);
        let v219: f64 = (v7 * v218);
        let v220: f64 = ((v219) as f64).tanh();
        let v222: f64 = 2.0;
        let v226: f64 = (v135 * v208);
        let v227: f64 = (v220 * v226);
        let v229: f64 = (v7 * self.scalar_v228);
        let v230: f64 = (v44 + v229);
        let v231: f64 = { let limexp_arg = v191; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let v232: f64 = (v136 * v231);
        let v233: f64 = (v230 + v232);
        let v234: f64 = (v227 * v233);
        let v235: f64 = (if self.scalar_v221 { v234 } else { v13 });
        let v238: f64 = (v5 - v197);
        let v239: f64 = (if self.scalar_v237 { v238 } else { v162 });
        let v240: f64 = (v239 * v239);
        let v241: f64 = (if self.scalar_v237 { v240 } else { v198 });
        let v242: f64 = (v239 * v241);
        let v243: f64 = (if self.scalar_v237 { v242 } else { v199 });
        let v244: f64 = (v176 * v239);
        let v245: f64 = (self.scalar_v201 * v241);
        let v246: f64 = (v244 + v245);
        let v247: f64 = (v181 * v243);
        let v248: f64 = (v246 + v247);
        let v249: f64 = (if self.scalar_v237 { v248 } else { v13 });
        let v250: f64 = ((v249) as f64).tanh();
        let v251: f64 = (v44 + v250);
        let v252: f64 = (if self.scalar_v237 { v251 } else { v13 });
        let v253: f64 = (self.scalar_v184 * v252);
        let v254: f64 = (self.scalar_v216 + v253);
        let v255: f64 = (if self.scalar_v237 { v254 } else { v13 });
        let v257: f64 = (v208 * self.scalar_v256);
        let v258: f64 = (self.scalar_v228 + v257);
        let v259: f64 = (if self.scalar_v237 { v258 } else { v13 });
        let v260: f64 = (v44 + v220);
        let v261: f64 = (v226 * v260);
        let v262: f64 = (v7 * v259);
        let v263: f64 = (v44 + v262);
        let v265: f64 = (v7 - v147);
        let v266: f64 = (self.scalar_v264 * v265);
        let v267: f64 = { let limexp_arg = v266; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let v268: f64 = (v136 * v267);
        let v269: f64 = (v263 + v268);
        let v270: f64 = (v261 * v269);
        let v271: f64 = (if self.scalar_v237 { v270 } else { v13 });
        let v272: f64 = (v252 * self.scalar_v256);
        let v273: f64 = (self.scalar_v228 + v272);
        let v274: f64 = (if self.scalar_v237 { v273 } else { v13 });
        let v275: f64 = (v7 * v255);
        let v276: f64 = ((v275) as f64).tanh();
        let v277: f64 = (if self.scalar_v237 { v276 } else { v13 });
        let v278: f64 = (v135 * v252);
        let v279: f64 = (v44 - v277);
        let v280: f64 = (v278 * v279);
        let v281: f64 = (v7 * v274);
        let v282: f64 = (v44 - v281);
        let v283: f64 = (v280 * v282);
        let v284: f64 = (if self.scalar_v237 { v283 } else { v13 });
        let v285: f64 = (v271 - v284);
        let v286: f64 = (v152 * v285);
        let v287: f64 = (if self.scalar_v237 { v286 } else { v235 });
        let v291: f64 = (if self.scalar_v290 { v198 } else { v239 });
        let v292: f64 = (v291 * v291);
        let v293: f64 = (if self.scalar_v290 { v292 } else { v241 });
        let v294: f64 = (self.scalar_v201 * v293);
        let v295: f64 = (v291 + v294);
        let v296: f64 = (v181 * v293);
        let v297: f64 = (v291 * v296);
        let v298: f64 = (v295 + v297);
        let v299: f64 = (v176 * v298);
        let v300: f64 = (if self.scalar_v290 { v299 } else { v206 });
        let v301: f64 = { let limexp_arg = v300; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let v302: f64 = (-v300);
        let v303: f64 = { let limexp_arg = v302; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let v304: f64 = (v301 - v303);
        let v305: f64 = (v152 * v304);
        let v306: f64 = ((v305) as f64).tanh();
        let v307: f64 = (v44 + v306);
        let v308: f64 = (if self.scalar_v290 { v307 } else { v215 });
        let v309: f64 = (self.scalar_v184 * v308);
        let v310: f64 = (self.scalar_v216 + v309);
        let v311: f64 = (if self.scalar_v290 { v310 } else { v13 });
        let v312: f64 = (v7 * v311);
        let v313: f64 = ((v312) as f64).tanh();
        let v314: f64 = (if self.scalar_v290 { v313 } else { v13 });
        let v315: f64 = (self.scalar_v256 * v308);
        let v316: f64 = (self.scalar_v228 + v315);
        let v317: f64 = (if self.scalar_v290 { v316 } else { v259 });
        let v318: f64 = (v135 * v308);
        let v319: f64 = (v314 * v318);
        let v320: f64 = (v7 * v317);
        let v321: f64 = (v44 + v320);
        let v322: f64 = (v191 * self.scalar_v264);
        let v323: f64 = { let limexp_arg = v322; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let v324: f64 = (v136 * v323);
        let v325: f64 = (v321 + v324);
        let v326: f64 = (v319 * v325);
        let v327: f64 = (if self.scalar_v290 { v326 } else { v287 });
        let v331: f64 = (if self.scalar_v330 { v198 } else { v291 });
        let v332: f64 = (v331 * v331);
        let v333: f64 = (if self.scalar_v330 { v332 } else { v293 });
        let v334: f64 = (self.scalar_v201 * v333);
        let v335: f64 = (v331 + v334);
        let v336: f64 = (v181 * v333);
        let v337: f64 = (v331 * v336);
        let v338: f64 = (v335 + v337);
        let v339: f64 = (v176 * v338);
        let v340: f64 = (if self.scalar_v330 { v339 } else { v300 });
        let v341: f64 = (if self.scalar_v330 { v238 } else { v243 });
        let v342: f64 = (v341 * v341);
        let v343: f64 = (if self.scalar_v330 { v342 } else { v13 });
        let v344: f64 = (self.scalar_v201 * v343);
        let v345: f64 = (v341 + v344);
        let v346: f64 = (v181 * v341);
        let v347: f64 = (v343 * v346);
        let v348: f64 = (v345 + v347);
        let v349: f64 = (v176 * v348);
        let v350: f64 = (if self.scalar_v330 { v349 } else { v249 });
        let v351: f64 = { let limexp_arg = v340; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let v352: f64 = (-v340);
        let v353: f64 = { let limexp_arg = v352; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let v354: f64 = (v351 - v353);
        let v355: f64 = (v152 * v354);
        let v356: f64 = ((v355) as f64).tanh();
        let v357: f64 = (v44 + v356);
        let v358: f64 = (if self.scalar_v330 { v357 } else { v308 });
        let v359: f64 = { let limexp_arg = v350; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let v360: f64 = (-v350);
        let v361: f64 = { let limexp_arg = v360; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let v362: f64 = (v359 - v361);
        let v363: f64 = (v152 * v362);
        let v364: f64 = ((v363) as f64).tanh();
        let v365: f64 = (v44 + v364);
        let v366: f64 = (if self.scalar_v330 { v365 } else { v13 });
        let v367: f64 = (self.scalar_v184 * v358);
        let v368: f64 = (self.scalar_v216 + v367);
        let v369: f64 = (if self.scalar_v330 { v368 } else { v311 });
        let v370: f64 = (self.scalar_v184 * v366);
        let v371: f64 = (self.scalar_v216 + v370);
        let v372: f64 = (if self.scalar_v330 { v371 } else { v13 });
        let v373: f64 = (v7 * v369);
        let v374: f64 = ((v373) as f64).tanh();
        let v375: f64 = (if self.scalar_v330 { v374 } else { v314 });
        let v376: f64 = (v7 * v372);
        let v377: f64 = ((v376) as f64).tanh();
        let v378: f64 = (if self.scalar_v330 { v377 } else { v13 });
        let v379: f64 = (self.scalar_v256 * v366);
        let v380: f64 = (self.scalar_v228 + v379);
        let v381: f64 = (if self.scalar_v330 { v380 } else { v13 });
        let v382: f64 = (self.scalar_v256 * v358);
        let v383: f64 = (self.scalar_v228 + v382);
        let v384: f64 = (if self.scalar_v330 { v383 } else { v13 });
        let v385: f64 = (v135 * v358);
        let v386: f64 = (v44 + v375);
        let v387: f64 = (v385 * v386);
        let v388: f64 = (v7 * v384);
        let v389: f64 = (v44 + v388);
        let v390: f64 = (v268 + v389);
        let v391: f64 = (v387 * v390);
        let v392: f64 = (if self.scalar_v330 { v391 } else { v271 });
        let v393: f64 = (v135 * v366);
        let v394: f64 = (v44 - v378);
        let v395: f64 = (v393 * v394);
        let v396: f64 = (v7 * v381);
        let v397: f64 = (v44 - v396);
        let v398: f64 = (v395 * v397);
        let v399: f64 = (if self.scalar_v330 { v398 } else { v284 });
        let v400: f64 = (v392 - v399);
        let v401: f64 = (v152 * v400);
        let v402: f64 = (if self.scalar_v330 { v401 } else { v327 });
        let v406: f64 = (if self.scalar_v405 { v258 } else { v317 });
        let v407: f64 = (if self.scalar_v405 { v368 } else { v369 });
        let v408: f64 = (v7 * v407);
        let v409: f64 = ((v408) as f64).tanh();
        let v410: f64 = (if self.scalar_v405 { v409 } else { v375 });
        let v411: f64 = (v11 * v407);
        let v412: f64 = ((v411) as f64).tanh();
        let v413: f64 = (if self.scalar_v405 { v412 } else { v13 });
        let v415: f64 = (v413 * self.scalar_v414);
        let v416: f64 = (v410 + v415);
        let v417: f64 = (v226 * v416);
        let v418: f64 = (v11 * self.scalar_v414);
        let v419: f64 = (v7 + v418);
        let v420: f64 = (v406 * v419);
        let v421: f64 = (v44 + v420);
        let v422: f64 = (v268 + v421);
        let v423: f64 = (v417 * v422);
        let v424: f64 = (if self.scalar_v405 { v423 } else { v402 });
        let v427: f64 = (v44 + v208);
        let v428: f64 = (v139 / v427);
        let v429: f64 = (self.scalar_v426 + v428);
        let v430: f64 = (if self.scalar_v425 { v429 } else { v13 });
        let v433: f64 = (v208 * self.scalar_v432);
        let v434: f64 = (self.scalar_v431 + v433);
        let v435: f64 = (if self.scalar_v425 { v434 } else { v13 });
        let v437: f64 = (v433 + self.scalar_v436);
        let v438: f64 = (if self.scalar_v425 { v437 } else { v13 });
        let v440: f64 = (v44 + v358);
        let v441: f64 = (v139 / v440);
        let v442: f64 = (self.scalar_v426 + v441);
        let v443: f64 = (if self.scalar_v439 { v442 } else { v430 });
        let v444: f64 = (v358 * self.scalar_v432);
        let v445: f64 = (self.scalar_v431 + v444);
        let v446: f64 = (if self.scalar_v439 { v445 } else { v435 });
        let v447: f64 = (self.scalar_v436 + v444);
        let v448: f64 = (if self.scalar_v439 { v447 } else { v438 });
        let v450: f64 = (v46 * self.scalar_v449);
        let v451: f64 = (v44 + v450);
        let v452: f64 = (v448 * v451);
        let v453: f64 = (v446 * v451);
        let v456: f64 = -1.0;
        let v457: f64 = (-v146);
        let v458: f64 = ((v457) as f64).tanh();
        let v459: f64 = (v159 * v458);
        let v460: f64 = { let limexp_arg = v459; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let v461: f64 = (if self.scalar_v455 { v460 } else { v331 });
        let v462: f64 = (v9 - v146);
        let v463: f64 = (if self.scalar_v455 { v462 } else { v13 });
        let v464: f64 = (-v9);
        let v466: f64 = (v464 - self.scalar_v465);
        let v467: f64 = (if self.scalar_v455 { v466 } else { v13 });
        let v468: f64 = (v5 - v146);
        let v469: f64 = (if self.scalar_v455 { v468 } else { v13 });
        let v471: f64 = (v6 - self.scalar_v470);
        let v472: f64 = (if self.scalar_v455 { v471 } else { v13 });
        let v474: f64 = (-v159);
        let v475: f64 = (v146 * v474);
        let v476: f64 = { let limexp_arg = v475; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let v477: f64 = (if self.scalar_v473 { v476 } else { v461 });
        let v488: f64 = ((v462) as f64).tanh();
        let v489: f64 = (if self.scalar_v487 { v488 } else { v463 });
        let v490: f64 = ((v468) as f64).tanh();
        let v491: f64 = (if self.scalar_v487 { v490 } else { v469 });
        let v494: f64 = (if self.scalar_v493 { v462 } else { v489 });
        let v495: f64 = (if self.scalar_v493 { v468 } else { v491 });
        let v496: f64 = (if self.scalar_v473 { v466 } else { v467 });
        let v497: f64 = (if self.scalar_v473 { v471 } else { v472 });
        let v498: f64 = (self.scalar_v478 * v496);
        let v499: f64 = { let limexp_arg = v498; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let v500: f64 = (v499 - self.scalar_v482);
        let v502: f64 = (v159 * v494);
        let v503: f64 = { let limexp_arg = v502; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let v507: f64 = (v500 * self.scalar_v506);
        let v508: f64 = (v503 - v507);
        let v509: f64 = (v508 - v477);
        let v510: f64 = (self.scalar_v501 * v509);
        let v511: f64 = (self.scalar_v478 * v497);
        let v512: f64 = { let limexp_arg = v511; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let v513: f64 = (v512 - self.scalar_v485);
        let v514: f64 = (v159 * v495);
        let v515: f64 = { let limexp_arg = v514; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let v516: f64 = (self.scalar_v506 * v513);
        let v517: f64 = (v515 - v516);
        let v518: f64 = (v517 - v477);
        let v519: f64 = (self.scalar_v501 * v518);
        let v521: f64 = (v9 * self.scalar_v520);
        let v522: f64 = (v144 + v521);
        let v524: f64 = (v7 * self.scalar_v523);
        let v525: f64 = (v522 + v524);
        let v526: f64 = ((v525) as f64).tanh();
        let v527: f64 = (v44 + v526);
        let v530: f64 = (v7 * self.scalar_v529);
        let v531: f64 = (self.scalar_v528 + v530);
        let v532: f64 = ((v531) as f64).tanh();
        let v533: f64 = (v44 + v532);
        let v536: f64 = (v7 * self.scalar_v535);
        let v537: f64 = (self.scalar_v534 - v536);
        let v538: f64 = ((v537) as f64).tanh();
        let v539: f64 = (v44 + v538);
        let v540: f64 = (v539 - self.scalar_v523);
        let v542: f64 = (v5 * self.scalar_v541);
        let v543: f64 = (v145 + v542);
        let v544: f64 = (v543 - v524);
        let v545: f64 = ((v544) as f64).tanh();
        let v546: f64 = (v44 + v545);
        let v557: f64 = (v137 * v527);
        let v558: f64 = (v533 * v557);
        let v559: f64 = (self.scalar_v551 + v558);
        let v560: f64 = (if self.scalar_v556 { v559 } else { self.scalar_v552 });
        let v561: f64 = (v540 * v546);
        let v563: f64 = (v561 + self.scalar_v562);
        let v564: f64 = (v138 * v563);
        let v565: f64 = (self.scalar_v553 + v564);
        let v566: f64 = (if self.scalar_v556 { v565 } else { self.scalar_v554 });
        let v570: f64 = (v533 - self.scalar_v523);
        let v571: f64 = (if self.scalar_v569 { v570 } else { v533 });
        let v572: f64 = (v144 + v524);
        let v573: f64 = ((v572) as f64).cosh();
        let v574: f64 = (if self.scalar_v569 { v573 } else { v13 });
        let v575: f64 = ((v574) as f64).ln();
        let v576: f64 = (if self.scalar_v569 { v575 } else { v13 });
        let v577: f64 = ((v525) as f64).cosh();
        let v578: f64 = (if self.scalar_v569 { v577 } else { v13 });
        let v579: f64 = ((v578) as f64).ln();
        let v580: f64 = (if self.scalar_v569 { v579 } else { v13 });
        let v581: f64 = (v572 + v576);
        let v582: f64 = (if self.scalar_v569 { v581 } else { v13 });
        let v583: f64 = (v525 + v580);
        let v584: f64 = (v583 - v582);
        let v585: f64 = (v571 * v584);
        let v586: f64 = (v585 / self.scalar_v520);
        let v587: f64 = (v9 * self.scalar_v562);
        let v588: f64 = (v586 + v587);
        let v589: f64 = (v137 * v588);
        let v590: f64 = (v9 * self.scalar_v551);
        let v591: f64 = (v589 + v590);
        let v592: f64 = (if self.scalar_v569 { v591 } else { v13 });
        let v593: f64 = (v145 - v524);
        let v594: f64 = ((v593) as f64).cosh();
        let v595: f64 = (if self.scalar_v569 { v594 } else { v574 });
        let v596: f64 = ((v595) as f64).ln();
        let v597: f64 = (if self.scalar_v569 { v596 } else { v13 });
        let v598: f64 = ((v544) as f64).cosh();
        let v599: f64 = (if self.scalar_v569 { v598 } else { v578 });
        let v600: f64 = ((v599) as f64).ln();
        let v601: f64 = (if self.scalar_v569 { v600 } else { v13 });
        let v602: f64 = (v593 + v597);
        let v603: f64 = (if self.scalar_v569 { v602 } else { v13 });
        let v604: f64 = (v544 + v601);
        let v605: f64 = (v604 - v603);
        let v606: f64 = (v540 * v605);
        let v607: f64 = (v606 / self.scalar_v541);
        let v608: f64 = (v5 * self.scalar_v562);
        let v609: f64 = (v607 + v608);
        let v610: f64 = (v138 * v609);
        let v611: f64 = (v5 * self.scalar_v553);
        let v612: f64 = (v610 + v611);
        let v613: f64 = (if self.scalar_v569 { v612 } else { v13 });
        let v2250: f64 = ((v525) as f64).sinh();
        let v2253: f64 = (self.scalar_v520 * v2250);
        let v2256: f64 = (if self.scalar_v569 { v2253 } else { v13 });
        let v2259: f64 = (v2256 / v578);
        let v2262: f64 = (if self.scalar_v569 { v2259 } else { v13 });
        let v2269: f64 = (self.scalar_v520 + v2262);
        let v2278: f64 = (v571 * v2269);
        let v2281: f64 = (v2278 / self.scalar_v520);
        let v2284: f64 = (self.scalar_v562 + v2281);
        let v2287: f64 = (v137 * v2284);
        let v2290: f64 = (self.scalar_v551 + v2287);
        let v2293: f64 = (if self.scalar_v569 { v2290 } else { v13 });
        let v614: f64 = v2293;
        let v615: f64 = (if self.scalar_v569 { v614 } else { v560 });
        let v2303: f64 = ((v544) as f64).sinh();
        let v2306: f64 = (self.scalar_v541 * v2303);
        let v2309: f64 = (if self.scalar_v569 { v2306 } else { v13 });
        let v2313: f64 = (v2309 / v599);
        let v2317: f64 = (if self.scalar_v569 { v2313 } else { v13 });
        let v2325: f64 = (self.scalar_v541 + v2317);
        let v2334: f64 = (v540 * v2325);
        let v2338: f64 = (v2334 / self.scalar_v541);
        let v2341: f64 = (self.scalar_v562 + v2338);
        let v2344: f64 = (v138 * v2341);
        let v2348: f64 = (self.scalar_v553 + v2344);
        let v2351: f64 = (if self.scalar_v569 { v2348 } else { v13 });
        let v616: f64 = v2351;
        let v617: f64 = (if self.scalar_v569 { v616 } else { v566 });
        let v622: f64 = (v9 / self.scalar_v621);
        let v623: f64 = (v622 - v44);
        let v624: f64 = (if self.scalar_v620 { v623 } else { v13 });
        let v627: f64 = (v624 * v624);
        let v628: f64 = (self.scalar_v626 + v627);
        let v630: f64 = f64::powf(v628, self.scalar_v629);
        let v633: f64 = (v627 * self.scalar_v632);
        let v634: f64 = (self.scalar_v626 + v633);
        let v635: f64 = (v630 * v634);
        let v636: f64 = (if self.scalar_v620 { v635 } else { v13 });
        let v637: f64 = (v9 + v524);
        let v638: f64 = (self.scalar_v520 * v637);
        let v639: f64 = (v144 + v638);
        let v640: f64 = ((v639) as f64).tanh();
        let v641: f64 = (v44 + v640);
        let v642: f64 = (if self.scalar_v620 { v641 } else { v527 });
        let v643: f64 = (if self.scalar_v620 { v533 } else { v571 });
        let v645: f64 = (v538 + self.scalar_v644);
        let v646: f64 = (if self.scalar_v620 { v645 } else { v540 });
        let v647: f64 = (v7 * self.scalar_v644);
        let v648: f64 = (v5 + v647);
        let v649: f64 = (self.scalar_v541 * v648);
        let v650: f64 = (v145 + v649);
        let v651: f64 = ((v650) as f64).tanh();
        let v652: f64 = (v44 + v651);
        let v653: f64 = (if self.scalar_v620 { v652 } else { v546 });
        let v655: f64 = (v636 * self.scalar_v654);
        let v656: f64 = (v642 + v655);
        let v657: f64 = (v137 * v656);
        let v658: f64 = (v643 * v657);
        let v659: f64 = (self.scalar_v551 + v658);
        let v660: f64 = (if self.scalar_v620 { v659 } else { v615 });
        let v661: f64 = (v646 * v653);
        let v662: f64 = (self.scalar_v562 + v661);
        let v663: f64 = (v138 * v662);
        let v664: f64 = (self.scalar_v553 + v663);
        let v665: f64 = (if self.scalar_v620 { v664 } else { v617 });
        let v669: f64 = (if self.scalar_v668 { v573 } else { v595 });
        let v670: f64 = ((v669) as f64).ln();
        let v671: f64 = (if self.scalar_v668 { v670 } else { v576 });
        let v672: f64 = (if self.scalar_v668 { v577 } else { v599 });
        let v673: f64 = ((v672) as f64).ln();
        let v674: f64 = (if self.scalar_v668 { v673 } else { v580 });
        let v676: f64 = (v9 + self.scalar_v621);
        let v677: f64 = (self.scalar_v654 * v676);
        let v678: f64 = (v456 + v622);
        let v679: f64 = f64::powf(v678, v222);
        let v680: f64 = (self.scalar_v626 + v679);
        let v682: f64 = f64::powf(v680, self.scalar_v681);
        let v683: f64 = (v677 * v682);
        let v684: f64 = (if self.scalar_v668 { v683 } else { v13 });
        let v690: f64 = (v572 + v671);
        let v691: f64 = (if self.scalar_v668 { v690 } else { v582 });
        let v692: f64 = (v525 + v674);
        let v693: f64 = (v692 - v691);
        let v694: f64 = (v684 + v693);
        let v695: f64 = (v694 - self.scalar_v689);
        let v696: f64 = (v532 + self.scalar_v644);
        let v697: f64 = (v695 * v696);
        let v698: f64 = (v697 / self.scalar_v520);
        let v699: f64 = (v587 + v698);
        let v700: f64 = (v137 * v699);
        let v701: f64 = (v590 + v700);
        let v702: f64 = (if self.scalar_v668 { v701 } else { v592 });
        let v703: f64 = (if self.scalar_v668 { v594 } else { v669 });
        let v704: f64 = ((v703) as f64).ln();
        let v705: f64 = (if self.scalar_v668 { v704 } else { v597 });
        let v706: f64 = (if self.scalar_v668 { v598 } else { v672 });
        let v707: f64 = ((v706) as f64).ln();
        let v708: f64 = (if self.scalar_v668 { v707 } else { v601 });
        let v709: f64 = (v593 + v705);
        let v710: f64 = (if self.scalar_v668 { v709 } else { v603 });
        let v711: f64 = (v544 + v708);
        let v712: f64 = (v711 - v710);
        let v713: f64 = (v645 * v712);
        let v714: f64 = (v713 / self.scalar_v541);
        let v715: f64 = (v608 + v714);
        let v716: f64 = (v138 * v715);
        let v717: f64 = (v611 + v716);
        let v718: f64 = (if self.scalar_v668 { v717 } else { v613 });
        let v2466: f64 = (self.scalar_v654 * v682);
        let v2454: f64 = f64::powf(v678, v44);
        let v2455: f64 = (v222 * v2454);
        let v2457: f64 = (self.scalar_v2360 * v2455);
        let v2459: f64 = f64::powf(v680, self.scalar_v2458);
        let v2460: f64 = (self.scalar_v681 * v2459);
        let v2462: f64 = (v2457 * v2460);
        let v2467: f64 = (v677 * v2462);
        let v2468: f64 = (v2466 + v2467);
        let v2470: f64 = (if self.scalar_v668 { v2468 } else { v13 });
        let v2310: f64 = (if self.scalar_v569 { v13 } else { v2256 });
        let v2444: f64 = (if self.scalar_v668 { v2253 } else { v2310 });
        let v2448: f64 = (v2444 / v672);
        let v2452: f64 = (if self.scalar_v668 { v2448 } else { v2262 });
        let v2477: f64 = (self.scalar_v520 + v2452);
        let v2481: f64 = (v2470 + v2477);
        let v2489: f64 = (v696 * v2481);
        let v2493: f64 = (v2489 / self.scalar_v520);
        let v2495: f64 = (self.scalar_v562 + v2493);
        let v2499: f64 = (v137 * v2495);
        let v2501: f64 = (self.scalar_v551 + v2499);
        let v2505: f64 = (if self.scalar_v668 { v2501 } else { v2293 });
        let v719: f64 = v2505;
        let v720: f64 = (if self.scalar_v668 { v719 } else { v660 });
        let v2443: f64 = (if self.scalar_v668 { v13 } else { v2309 });
        let v2514: f64 = (if self.scalar_v668 { v2306 } else { v2443 });
        let v2518: f64 = (v2514 / v706);
        let v2522: f64 = (if self.scalar_v668 { v2518 } else { v2317 });
        let v2530: f64 = (self.scalar_v541 + v2522);
        let v2539: f64 = (v645 * v2530);
        let v2543: f64 = (v2539 / self.scalar_v541);
        let v2546: f64 = (self.scalar_v562 + v2543);
        let v2549: f64 = (v138 * v2546);
        let v2552: f64 = (self.scalar_v553 + v2549);
        let v2555: f64 = (if self.scalar_v668 { v2552 } else { v2351 });
        let v721: f64 = v2555;
        let v722: f64 = (if self.scalar_v668 { v721 } else { v665 });
        let v748: f64 = 5.5226012e-23;
        let v749: f64 = (v35 * v748);
        let v753: f64 = (v749 * self.scalar_v752);
        let v754: f64 = (v137 * v753);
        let v757: f64 = (v754 * self.scalar_v756);
        let v758: f64 = (if self.scalar_v747 { v757 } else { v13 });
        let v759: f64 = (v758 * v758);
        let v760: f64 = (v44 - v759);
        let v761: f64 = ((v760) as f64).sqrt();
        let v762: f64 = (if self.scalar_v747 { v761 } else { v13 });
        let v763: f64 = (-v758);
        let v764: f64 = 3.141592653589793;
        let v765: f64 = (v763 * v764);
        let v766: f64 = (if self.scalar_v747 { v765 } else { v13 });
        let v767: f64 = (v758 * v764);
        let v768: f64 = (if self.scalar_v747 { v767 } else { v13 });
        let v770: f64 = (-v424);
        let v772: f64 = ctx.node_voltage(nodes[15]);
        let v773: f64 = (self.scalar_v771 * v772);
        let v775: f64 = ctx.branch_current(branches[0]);
        let v776: f64 = (self.scalar_v774 * v775);
        let v777: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, v718);
        let v778: f64 = (if self.scalar_v723 { v777 } else { v13 });
        let v779: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, v702);
        let v780: f64 = (if self.scalar_v723 { v779 } else { v13 });
        let v782: f64 = (v5 * v722);
        let v783: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 5, v782);
        let v784: f64 = (if self.scalar_v781 { v783 } else { v13 });
        let v785: f64 = (v9 * v720);
        let v786: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 6, v785);
        let v787: f64 = (if self.scalar_v781 { v786 } else { v13 });
        let v789: f64 = ctx.node_voltage(nodes[7]);
        let v790: f64 = (v789 - v4);
        let v791: f64 = (self.scalar_v788 * v790);
        let v793: f64 = (v7 * self.scalar_v792);
        let v794: f64 = ctx.node_voltage(nodes[6]);
        let v795: f64 = (v794 - v10);
        let v796: f64 = (v140 * v795);
        let v797: f64 = (v167 * v795);
        let v798: f64 = ctx.branch_current(branches[1]);
        let v799: f64 = (v443 * v798);
        let v800: f64 = (self.scalar_v724 * v798);
        let v801: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, v800);
        let v802: f64 = (v799 + v801);
        let v803: f64 = (if self.scalar_v725 { v802 } else { v13 });
        let v804: f64 = (v8 - v0);
        let v805: f64 = (v804 / v141);
        let v806: f64 = (if self.scalar_v728 { v805 } else { v13 });
        let v807: f64 = (v2 * v142);
        let v808: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 10, v807);
        let v809: f64 = (if self.scalar_v728 { v808 } else { v13 });
        let v811: f64 = ctx.node_voltage(nodes[14]);
        let v812: f64 = (v8 - v811);
        let v813: f64 = (self.scalar_v810 * v812);
        let v814: f64 = (v811 - v1);
        let v815: f64 = (v814 / self.scalar_v729);
        let v816: f64 = (if self.scalar_v730 { v815 } else { v13 });
        let v817: f64 = ctx.node_voltage(nodes[13]);
        let v818: f64 = (v817 - v3);
        let v819: f64 = (v818 / self.scalar_v731);
        let v820: f64 = (if self.scalar_v732 { v819 } else { v13 });
        let v821: f64 = (v817 - v8);
        let v822: f64 = (v821 / self.scalar_v733);
        let v823: f64 = (if self.scalar_v734 { v822 } else { v13 });
        let v824: f64 = ctx.branch_current(branches[7]);
        let v825: f64 = (self.scalar_v735 * v824);
        let v826: f64 = (if self.scalar_v736 { v825 } else { v13 });
        let v828: f64 = ctx.branch_current(branches[10]);
        let v829: f64 = (self.scalar_v827 * v828);
        let v830: f64 = ctx.branch_current(branches[11]);
        let v831: f64 = (v452 * v830);
        let v832: f64 = (if self.scalar_v737 { v831 } else { v13 });
        let v834: f64 = ctx.branch_current(branches[14]);
        let v835: f64 = (self.scalar_v833 * v834);
        let v836: f64 = ctx.branch_current(branches[15]);
        let v837: f64 = (v453 * v836);
        let v838: f64 = (if self.scalar_v740 { v837 } else { v13 });
        let v840: f64 = ctx.branch_current(branches[18]);
        let v841: f64 = (self.scalar_v839 * v840);
        let v842: f64 = 1e-15;
        let v843: f64 = ctx.node_voltage(nodes[2]);
        let v844: f64 = (v0 - v843);
        let v845: f64 = (v167 * v844);
        let v846: f64 = ctx.node_voltage(nodes[17]);
        let v847: f64 = (if self.scalar_v747 { v846 } else { v13 });
        let v848: f64 = ctx.node_voltage(nodes[18]);
        let v849: f64 = (if self.scalar_v747 { v848 } else { v13 });
        let v850: f64 = (v766 * v846);
        let v851: f64 = (v762 * v848);
        let v852: f64 = (v850 + v851);
        let v853: f64 = (if self.scalar_v747 { v852 } else { v13 });
        let v854: f64 = (-v768);
        let v855: f64 = (v846 * v854);
        let v856: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 15, v855);
        let v857: f64 = (if self.scalar_v747 { v856 } else { v13 });
        let v858: f64 = (v7 * v424);
        let v859: f64 = ((v858) as f64).abs();
        let v860: f64 = (v9 * v510);
        let v861: f64 = ((v860) as f64).abs();
        let v862: f64 = (v859 + v861);
        let v863: f64 = (-v862);
        let v864: f64 = (if self.scalar_v769 { v863 } else { v13 });
        let v865: f64 = (v32 / v49);
        let v866: f64 = (if self.scalar_v769 { v865 } else { v13 });
        let v868: f64 = (v32 * self.scalar_v867);
        let v869: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 16, v868);
        let v870: f64 = (if self.scalar_v769 { v869 } else { v13 });
        let v872: f64 = (v32 * v167);
        let v873: f64 = (if self.scalar_v871 { v872 } else { v13 });
        let v875: f64 = ((v161) as f64).sinh();
        let v876: f64 = (self.scalar_v160 * v875);
        let v877: f64 = (self.scalar_v874 * v875);
        let v879: f64 = (v162 * v876);
        let v880: f64 = (v879 + v879);
        let v881: f64 = (v162 * v877);
        let v882: f64 = (v881 + v881);
        let v883: f64 = (self.scalar_v166 * v880);
        let v884: f64 = (-v883);
        let v885: f64 = (v169 * v169);
        let v886: f64 = (v884 / v885);
        let v887: f64 = (self.scalar_v166 * v882);
        let v888: f64 = (-v887);
        let v889: f64 = (v888 / v885);
        let v890: f64 = (self.scalar_v165 * v886);
        let v891: f64 = (self.scalar_v165 * v889);
        let v892: f64 = (v175 * v890);
        let v893: f64 = (v175 * v891);
        let v895: f64 = (v186 * v186);
        let v896: f64 = (v44 - v895);
        let v897: f64 = (self.scalar_v184 * v896);
        let v898: f64 = (self.scalar_v894 * v896);
        let v899: f64 = (self.scalar_v182 * v897);
        let v900: f64 = (self.scalar_v182 * v898);
        let v901: f64 = (v900 - self.scalar_v878);
        let v903: f64 = (v192 + v192);
        let v904: f64 = (-v192);
        let v905: f64 = (v191 * self.scalar_v902);
        let v906: f64 = (v904 + v905);
        let v907: f64 = (v899 - v903);
        let v908: f64 = (-v906);
        let v909: f64 = (v196 * self.scalar_v878);
        let v910: f64 = (v196 * v907);
        let v911: f64 = (v196 * v901);
        let v912: f64 = (v196 * v908);
        let v913: f64 = (-v909);
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
        let v926: f64 = (v176 * v913);
        let v927: f64 = (v198 * v892);
        let v928: f64 = (v176 * v914);
        let v929: f64 = (v927 + v928);
        let v930: f64 = (v198 * v893);
        let v931: f64 = (v176 * v915);
        let v932: f64 = (v930 + v931);
        let v933: f64 = (v176 * v916);
        let v934: f64 = (self.scalar_v201 * v918);
        let v935: f64 = (self.scalar_v201 * v920);
        let v936: f64 = (self.scalar_v201 * v922);
        let v937: f64 = (self.scalar_v201 * v924);
        let v938: f64 = (self.scalar_v201 * v925);
        let v939: f64 = (v926 + v934);
        let v940: f64 = (v929 + v935);
        let v941: f64 = (v932 + v936);
        let v942: f64 = (v933 + v937);
        let v943: f64 = (v176 + v938);
        let v944: f64 = (v181 * v913);
        let v945: f64 = (v181 * v914);
        let v946: f64 = (v181 * v915);
        let v947: f64 = (v181 * v916);
        let v948: f64 = (v204 * v918);
        let v949: f64 = (v199 * v944);
        let v950: f64 = (v948 + v949);
        let v951: f64 = (v204 * v920);
        let v952: f64 = (v199 * v945);
        let v953: f64 = (v951 + v952);
        let v954: f64 = (v204 * v922);
        let v955: f64 = (v199 * v946);
        let v956: f64 = (v954 + v955);
        let v957: f64 = (v204 * v924);
        let v958: f64 = (v199 * v947);
        let v959: f64 = (v957 + v958);
        let v960: f64 = (v204 * v925);
        let v961: f64 = (v181 * v199);
        let v962: f64 = (v960 + v961);
        let v963: f64 = (v939 + v950);
        let v964: f64 = (v940 + v953);
        let v965: f64 = (v941 + v956);
        let v966: f64 = (v942 + v959);
        let v967: f64 = (v943 + v962);
        let v968: f64 = (v207 * v207);
        let v969: f64 = (v44 - v968);
        let v970: f64 = (v963 * v969);
        let v971: f64 = (v964 * v969);
        let v972: f64 = (v965 * v969);
        let v973: f64 = (v966 * v969);
        let v974: f64 = (v967 * v969);
        let v975: f64 = { let limexp_arg = v206; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v976: f64 = (v963 * v975);
        let v977: f64 = (v964 * v975);
        let v978: f64 = (v965 * v975);
        let v979: f64 = (v966 * v975);
        let v980: f64 = (v967 * v975);
        let v981: f64 = (-v963);
        let v982: f64 = (-v964);
        let v983: f64 = (-v965);
        let v984: f64 = (-v966);
        let v985: f64 = (-v967);
        let v986: f64 = { let limexp_arg = v210; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v987: f64 = (v981 * v986);
        let v988: f64 = (v982 * v986);
        let v989: f64 = (v983 * v986);
        let v990: f64 = (v984 * v986);
        let v991: f64 = (v985 * v986);
        let v992: f64 = (v976 - v987);
        let v993: f64 = (v977 - v988);
        let v994: f64 = (v978 - v989);
        let v995: f64 = (v979 - v990);
        let v996: f64 = (v980 - v991);
        let v997: f64 = (v152 * v992);
        let v998: f64 = (v152 * v993);
        let v999: f64 = (v152 * v994);
        let v1000: f64 = (v152 * v995);
        let v1001: f64 = (v152 * v996);
        let v1002: f64 = (v214 * v214);
        let v1003: f64 = (v44 - v1002);
        let v1004: f64 = (v997 * v1003);
        let v1005: f64 = (v998 * v1003);
        let v1006: f64 = (v999 * v1003);
        let v1007: f64 = (v1000 * v1003);
        let v1008: f64 = (v1001 * v1003);
        let v1009: f64 = (self.scalar_v184 * v970);
        let v1010: f64 = (self.scalar_v184 * v971);
        let v1011: f64 = (self.scalar_v184 * v972);
        let v1012: f64 = (self.scalar_v184 * v973);
        let v1013: f64 = (self.scalar_v184 * v974);
        let v1014: f64 = (v7 * v1009);
        let v1015: f64 = (v7 * v1010);
        let v1016: f64 = (v218 + v1015);
        let v1017: f64 = (-v218);
        let v1018: f64 = (v7 * v1011);
        let v1019: f64 = (v1017 + v1018);
        let v1020: f64 = (v7 * v1012);
        let v1021: f64 = (v7 * v1013);
        let v1022: f64 = (v220 * v220);
        let v1023: f64 = (v44 - v1022);
        let v1024: f64 = (v1014 * v1023);
        let v1025: f64 = (v1016 * v1023);
        let v1026: f64 = (v1019 * v1023);
        let v1027: f64 = (v1020 * v1023);
        let v1028: f64 = (v1021 * v1023);
        let v1029: f64 = (v135 * v970);
        let v1030: f64 = (v135 * v971);
        let v1031: f64 = (v135 * v972);
        let v1032: f64 = (v135 * v973);
        let v1033: f64 = (v135 * v974);
        let v1034: f64 = (v226 * v1024);
        let v1035: f64 = (v220 * v1029);
        let v1036: f64 = (v1034 + v1035);
        let v1037: f64 = (v226 * v1025);
        let v1038: f64 = (v220 * v1030);
        let v1039: f64 = (v1037 + v1038);
        let v1040: f64 = (v226 * v1026);
        let v1041: f64 = (v220 * v1031);
        let v1042: f64 = (v1040 + v1041);
        let v1043: f64 = (v226 * v1027);
        let v1044: f64 = (v220 * v1032);
        let v1045: f64 = (v1043 + v1044);
        let v1046: f64 = (v226 * v1028);
        let v1047: f64 = (v220 * v1033);
        let v1048: f64 = (v1046 + v1047);
        let v1050: f64 = { let limexp_arg = v191; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v1051: f64 = (-v1050);
        let v1052: f64 = (v136 * v1050);
        let v1053: f64 = (v136 * v1051);
        let v1054: f64 = (self.scalar_v228 + v1052);
        let v1055: f64 = (v233 * v1036);
        let v1056: f64 = (v233 * v1039);
        let v1057: f64 = (v227 * v1054);
        let v1058: f64 = (v1056 + v1057);
        let v1059: f64 = (v233 * v1042);
        let v1060: f64 = (v227 * self.scalar_v1049);
        let v1061: f64 = (v1059 + v1060);
        let v1062: f64 = (v233 * v1045);
        let v1063: f64 = (v227 * v1053);
        let v1064: f64 = (v1062 + v1063);
        let v1065: f64 = (v233 * v1048);
        let v1066: f64 = (if self.scalar_v221 { v1055 } else { v13 });
        let v1067: f64 = (if self.scalar_v221 { v1058 } else { v13 });
        let v1068: f64 = (if self.scalar_v221 { v1061 } else { v13 });
        let v1069: f64 = (if self.scalar_v221 { v1064 } else { v13 });
        let v1070: f64 = (if self.scalar_v221 { v1065 } else { v13 });
        let v1071: f64 = (v456 - v910);
        let v1072: f64 = (-v911);
        let v1073: f64 = (v44 - v912);
        let v1074: f64 = (if self.scalar_v237 { v913 } else { v13 });
        let v1075: f64 = (if self.scalar_v237 { v1071 } else { v876 });
        let v1076: f64 = (if self.scalar_v237 { v1072 } else { v877 });
        let v1077: f64 = (if self.scalar_v237 { v1073 } else { v13 });
        let v1078: f64 = (v239 * v1074);
        let v1079: f64 = (v1078 + v1078);
        let v1080: f64 = (v239 * v1075);
        let v1081: f64 = (v1080 + v1080);
        let v1082: f64 = (v239 * v1076);
        let v1083: f64 = (v1082 + v1082);
        let v1084: f64 = (v239 * v1077);
        let v1085: f64 = (v1084 + v1084);
        let v1086: f64 = (if self.scalar_v237 { v1079 } else { v913 });
        let v1087: f64 = (if self.scalar_v237 { v1081 } else { v914 });
        let v1088: f64 = (if self.scalar_v237 { v1083 } else { v915 });
        let v1089: f64 = (if self.scalar_v237 { v1085 } else { v916 });
        let v1091: f64 = (v241 * v1074);
        let v1092: f64 = (v239 * v1086);
        let v1093: f64 = (v1091 + v1092);
        let v1094: f64 = (v241 * v1075);
        let v1095: f64 = (v239 * v1087);
        let v1096: f64 = (v1094 + v1095);
        let v1097: f64 = (v241 * v1076);
        let v1098: f64 = (v239 * v1088);
        let v1099: f64 = (v1097 + v1098);
        let v1100: f64 = (v241 * v1077);
        let v1101: f64 = (v239 * v1089);
        let v1102: f64 = (v1100 + v1101);
        let v1103: f64 = (v239 * self.scalar_v1090);
        let v1104: f64 = (if self.scalar_v237 { v1093 } else { v918 });
        let v1105: f64 = (if self.scalar_v237 { v1096 } else { v920 });
        let v1106: f64 = (if self.scalar_v237 { v1099 } else { v922 });
        let v1107: f64 = (if self.scalar_v237 { v1102 } else { v924 });
        let v1108: f64 = (if self.scalar_v237 { v1103 } else { v925 });
        let v1109: f64 = (v176 * v1074);
        let v1110: f64 = (v239 * v892);
        let v1111: f64 = (v176 * v1075);
        let v1112: f64 = (v1110 + v1111);
        let v1113: f64 = (v239 * v893);
        let v1114: f64 = (v176 * v1076);
        let v1115: f64 = (v1113 + v1114);
        let v1116: f64 = (v176 * v1077);
        let v1117: f64 = (self.scalar_v201 * v1086);
        let v1118: f64 = (self.scalar_v201 * v1087);
        let v1119: f64 = (self.scalar_v201 * v1088);
        let v1120: f64 = (self.scalar_v201 * v1089);
        let v1122: f64 = (v1109 + v1117);
        let v1123: f64 = (v1112 + v1118);
        let v1124: f64 = (v1115 + v1119);
        let v1125: f64 = (v1116 + v1120);
        let v1126: f64 = (v181 * v1104);
        let v1127: f64 = (v181 * v1105);
        let v1128: f64 = (v181 * v1106);
        let v1129: f64 = (v181 * v1107);
        let v1130: f64 = (v181 * v1108);
        let v1131: f64 = (v1122 + v1126);
        let v1132: f64 = (v1123 + v1127);
        let v1133: f64 = (v1124 + v1128);
        let v1134: f64 = (v1125 + v1129);
        let v1135: f64 = (self.scalar_v1121 + v1130);
        let v1136: f64 = (if self.scalar_v237 { v1131 } else { v13 });
        let v1137: f64 = (if self.scalar_v237 { v1132 } else { v13 });
        let v1138: f64 = (if self.scalar_v237 { v1133 } else { v13 });
        let v1139: f64 = (if self.scalar_v237 { v1134 } else { v13 });
        let v1140: f64 = (if self.scalar_v237 { v1135 } else { v13 });
        let v1141: f64 = (v250 * v250);
        let v1142: f64 = (v44 - v1141);
        let v1143: f64 = (v1136 * v1142);
        let v1144: f64 = (v1137 * v1142);
        let v1145: f64 = (v1138 * v1142);
        let v1146: f64 = (v1139 * v1142);
        let v1147: f64 = (v1140 * v1142);
        let v1148: f64 = (if self.scalar_v237 { v1143 } else { v13 });
        let v1149: f64 = (if self.scalar_v237 { v1144 } else { v13 });
        let v1150: f64 = (if self.scalar_v237 { v1145 } else { v13 });
        let v1151: f64 = (if self.scalar_v237 { v1146 } else { v13 });
        let v1152: f64 = (if self.scalar_v237 { v1147 } else { v13 });
        let v1153: f64 = (self.scalar_v184 * v1148);
        let v1154: f64 = (self.scalar_v184 * v1149);
        let v1155: f64 = (self.scalar_v184 * v1150);
        let v1156: f64 = (self.scalar_v184 * v1151);
        let v1157: f64 = (self.scalar_v184 * v1152);
        let v1158: f64 = (if self.scalar_v237 { v1153 } else { v13 });
        let v1159: f64 = (if self.scalar_v237 { v1154 } else { v13 });
        let v1160: f64 = (if self.scalar_v237 { v1155 } else { v13 });
        let v1161: f64 = (if self.scalar_v237 { v1156 } else { v13 });
        let v1162: f64 = (if self.scalar_v237 { v1157 } else { v13 });
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
        let v1173: f64 = (v260 * v1029);
        let v1174: f64 = (v1034 + v1173);
        let v1175: f64 = (v260 * v1030);
        let v1176: f64 = (v1037 + v1175);
        let v1177: f64 = (v260 * v1031);
        let v1178: f64 = (v1040 + v1177);
        let v1179: f64 = (v260 * v1032);
        let v1180: f64 = (v1043 + v1179);
        let v1181: f64 = (v260 * v1033);
        let v1182: f64 = (v1046 + v1181);
        let v1183: f64 = (v7 * v1168);
        let v1184: f64 = (v7 * v1169);
        let v1185: f64 = (v259 + v1184);
        let v1186: f64 = (-v259);
        let v1187: f64 = (v7 * v1170);
        let v1188: f64 = (v1186 + v1187);
        let v1189: f64 = (v7 * v1171);
        let v1190: f64 = (v7 * v1172);
        let v1192: f64 = { let limexp_arg = v266; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v1193: f64 = (self.scalar_v264 * v1192);
        let v1194: f64 = (self.scalar_v1191 * v1192);
        let v1195: f64 = (v136 * v1193);
        let v1196: f64 = (v136 * v1194);
        let v1197: f64 = (v1185 + v1195);
        let v1198: f64 = (v1188 + v1196);
        let v1199: f64 = (v269 * v1174);
        let v1200: f64 = (v261 * v1183);
        let v1201: f64 = (v1199 + v1200);
        let v1202: f64 = (v269 * v1176);
        let v1203: f64 = (v261 * v1197);
        let v1204: f64 = (v1202 + v1203);
        let v1205: f64 = (v269 * v1178);
        let v1206: f64 = (v261 * v1198);
        let v1207: f64 = (v1205 + v1206);
        let v1208: f64 = (v269 * v1180);
        let v1209: f64 = (v261 * v1189);
        let v1210: f64 = (v1208 + v1209);
        let v1211: f64 = (v269 * v1182);
        let v1212: f64 = (v261 * v1190);
        let v1213: f64 = (v1211 + v1212);
        let v1214: f64 = (if self.scalar_v237 { v1201 } else { v13 });
        let v1215: f64 = (if self.scalar_v237 { v1204 } else { v13 });
        let v1216: f64 = (if self.scalar_v237 { v1207 } else { v13 });
        let v1217: f64 = (if self.scalar_v237 { v1210 } else { v13 });
        let v1218: f64 = (if self.scalar_v237 { v1213 } else { v13 });
        let v1219: f64 = (self.scalar_v256 * v1148);
        let v1220: f64 = (self.scalar_v256 * v1149);
        let v1221: f64 = (self.scalar_v256 * v1150);
        let v1222: f64 = (self.scalar_v256 * v1151);
        let v1223: f64 = (self.scalar_v256 * v1152);
        let v1224: f64 = (if self.scalar_v237 { v1219 } else { v13 });
        let v1225: f64 = (if self.scalar_v237 { v1220 } else { v13 });
        let v1226: f64 = (if self.scalar_v237 { v1221 } else { v13 });
        let v1227: f64 = (if self.scalar_v237 { v1222 } else { v13 });
        let v1228: f64 = (if self.scalar_v237 { v1223 } else { v13 });
        let v1229: f64 = (v7 * v1158);
        let v1230: f64 = (v7 * v1159);
        let v1231: f64 = (v255 + v1230);
        let v1232: f64 = (-v255);
        let v1233: f64 = (v7 * v1160);
        let v1234: f64 = (v1232 + v1233);
        let v1235: f64 = (v7 * v1161);
        let v1236: f64 = (v7 * v1162);
        let v1237: f64 = (v276 * v276);
        let v1238: f64 = (v44 - v1237);
        let v1239: f64 = (v1229 * v1238);
        let v1240: f64 = (v1231 * v1238);
        let v1241: f64 = (v1234 * v1238);
        let v1242: f64 = (v1235 * v1238);
        let v1243: f64 = (v1236 * v1238);
        let v1244: f64 = (if self.scalar_v237 { v1239 } else { v13 });
        let v1245: f64 = (if self.scalar_v237 { v1240 } else { v13 });
        let v1246: f64 = (if self.scalar_v237 { v1241 } else { v13 });
        let v1247: f64 = (if self.scalar_v237 { v1242 } else { v13 });
        let v1248: f64 = (if self.scalar_v237 { v1243 } else { v13 });
        let v1249: f64 = (v135 * v1148);
        let v1250: f64 = (v135 * v1149);
        let v1251: f64 = (v135 * v1150);
        let v1252: f64 = (v135 * v1151);
        let v1253: f64 = (v135 * v1152);
        let v1254: f64 = (-v1244);
        let v1255: f64 = (-v1245);
        let v1256: f64 = (-v1246);
        let v1257: f64 = (-v1247);
        let v1258: f64 = (-v1248);
        let v1259: f64 = (v279 * v1249);
        let v1260: f64 = (v278 * v1254);
        let v1261: f64 = (v1259 + v1260);
        let v1262: f64 = (v279 * v1250);
        let v1263: f64 = (v278 * v1255);
        let v1264: f64 = (v1262 + v1263);
        let v1265: f64 = (v279 * v1251);
        let v1266: f64 = (v278 * v1256);
        let v1267: f64 = (v1265 + v1266);
        let v1268: f64 = (v279 * v1252);
        let v1269: f64 = (v278 * v1257);
        let v1270: f64 = (v1268 + v1269);
        let v1271: f64 = (v279 * v1253);
        let v1272: f64 = (v278 * v1258);
        let v1273: f64 = (v1271 + v1272);
        let v1274: f64 = (v7 * v1224);
        let v1275: f64 = (v7 * v1225);
        let v1276: f64 = (v274 + v1275);
        let v1277: f64 = (-v274);
        let v1278: f64 = (v7 * v1226);
        let v1279: f64 = (v1277 + v1278);
        let v1280: f64 = (v7 * v1227);
        let v1281: f64 = (v7 * v1228);
        let v1282: f64 = (-v1274);
        let v1283: f64 = (-v1276);
        let v1284: f64 = (-v1279);
        let v1285: f64 = (-v1280);
        let v1286: f64 = (-v1281);
        let v1287: f64 = (v282 * v1261);
        let v1288: f64 = (v280 * v1282);
        let v1289: f64 = (v1287 + v1288);
        let v1290: f64 = (v282 * v1264);
        let v1291: f64 = (v280 * v1283);
        let v1292: f64 = (v1290 + v1291);
        let v1293: f64 = (v282 * v1267);
        let v1294: f64 = (v280 * v1284);
        let v1295: f64 = (v1293 + v1294);
        let v1296: f64 = (v282 * v1270);
        let v1297: f64 = (v280 * v1285);
        let v1298: f64 = (v1296 + v1297);
        let v1299: f64 = (v282 * v1273);
        let v1300: f64 = (v280 * v1286);
        let v1301: f64 = (v1299 + v1300);
        let v1302: f64 = (if self.scalar_v237 { v1289 } else { v13 });
        let v1303: f64 = (if self.scalar_v237 { v1292 } else { v13 });
        let v1304: f64 = (if self.scalar_v237 { v1295 } else { v13 });
        let v1305: f64 = (if self.scalar_v237 { v1298 } else { v13 });
        let v1306: f64 = (if self.scalar_v237 { v1301 } else { v13 });
        let v1307: f64 = (v1214 - v1302);
        let v1308: f64 = (v1215 - v1303);
        let v1309: f64 = (v1216 - v1304);
        let v1310: f64 = (v1217 - v1305);
        let v1311: f64 = (v1218 - v1306);
        let v1312: f64 = (v152 * v1307);
        let v1313: f64 = (v152 * v1308);
        let v1314: f64 = (v152 * v1309);
        let v1315: f64 = (v152 * v1310);
        let v1316: f64 = (v152 * v1311);
        let v1317: f64 = (if self.scalar_v237 { v1312 } else { v1066 });
        let v1318: f64 = (if self.scalar_v237 { v1313 } else { v1067 });
        let v1319: f64 = (if self.scalar_v237 { v1314 } else { v1068 });
        let v1320: f64 = (if self.scalar_v237 { v1315 } else { v1069 });
        let v1321: f64 = (if self.scalar_v237 { v1316 } else { v1070 });
        let v1322: f64 = (if self.scalar_v290 { v913 } else { v1074 });
        let v1323: f64 = (if self.scalar_v290 { v914 } else { v1075 });
        let v1324: f64 = (if self.scalar_v290 { v915 } else { v1076 });
        let v1325: f64 = (if self.scalar_v290 { v916 } else { v1077 });
        let v1327: f64 = (v291 * v1322);
        let v1328: f64 = (v1327 + v1327);
        let v1329: f64 = (v291 * v1323);
        let v1330: f64 = (v1329 + v1329);
        let v1331: f64 = (v291 * v1324);
        let v1332: f64 = (v1331 + v1331);
        let v1333: f64 = (v291 * v1325);
        let v1334: f64 = (v1333 + v1333);
        let v1335: f64 = (v291 * self.scalar_v1326);
        let v1336: f64 = (v1335 + v1335);
        let v1337: f64 = (if self.scalar_v290 { v1328 } else { v1086 });
        let v1338: f64 = (if self.scalar_v290 { v1330 } else { v1087 });
        let v1339: f64 = (if self.scalar_v290 { v1332 } else { v1088 });
        let v1340: f64 = (if self.scalar_v290 { v1334 } else { v1089 });
        let v1341: f64 = (if self.scalar_v290 { v1336 } else { self.scalar_v1090 });
        let v1342: f64 = (self.scalar_v201 * v1337);
        let v1343: f64 = (self.scalar_v201 * v1338);
        let v1344: f64 = (self.scalar_v201 * v1339);
        let v1345: f64 = (self.scalar_v201 * v1340);
        let v1346: f64 = (self.scalar_v201 * v1341);
        let v1347: f64 = (v1322 + v1342);
        let v1348: f64 = (v1323 + v1343);
        let v1349: f64 = (v1324 + v1344);
        let v1350: f64 = (v1325 + v1345);
        let v1351: f64 = (self.scalar_v1326 + v1346);
        let v1352: f64 = (v181 * v1337);
        let v1353: f64 = (v181 * v1338);
        let v1354: f64 = (v181 * v1339);
        let v1355: f64 = (v181 * v1340);
        let v1356: f64 = (v181 * v1341);
        let v1357: f64 = (v296 * v1322);
        let v1358: f64 = (v291 * v1352);
        let v1359: f64 = (v1357 + v1358);
        let v1360: f64 = (v296 * v1323);
        let v1361: f64 = (v291 * v1353);
        let v1362: f64 = (v1360 + v1361);
        let v1363: f64 = (v296 * v1324);
        let v1364: f64 = (v291 * v1354);
        let v1365: f64 = (v1363 + v1364);
        let v1366: f64 = (v296 * v1325);
        let v1367: f64 = (v291 * v1355);
        let v1368: f64 = (v1366 + v1367);
        let v1369: f64 = (v296 * self.scalar_v1326);
        let v1370: f64 = (v291 * v1356);
        let v1371: f64 = (v1369 + v1370);
        let v1372: f64 = (v1347 + v1359);
        let v1373: f64 = (v1348 + v1362);
        let v1374: f64 = (v1349 + v1365);
        let v1375: f64 = (v1350 + v1368);
        let v1376: f64 = (v1351 + v1371);
        let v1377: f64 = (v176 * v1372);
        let v1378: f64 = (v298 * v892);
        let v1379: f64 = (v176 * v1373);
        let v1380: f64 = (v1378 + v1379);
        let v1381: f64 = (v298 * v893);
        let v1382: f64 = (v176 * v1374);
        let v1383: f64 = (v1381 + v1382);
        let v1384: f64 = (v176 * v1375);
        let v1385: f64 = (v176 * v1376);
        let v1386: f64 = (if self.scalar_v290 { v1377 } else { v963 });
        let v1387: f64 = (if self.scalar_v290 { v1380 } else { v964 });
        let v1388: f64 = (if self.scalar_v290 { v1383 } else { v965 });
        let v1389: f64 = (if self.scalar_v290 { v1384 } else { v966 });
        let v1390: f64 = (if self.scalar_v290 { v1385 } else { v967 });
        let v1391: f64 = { let limexp_arg = v300; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v1392: f64 = (v1386 * v1391);
        let v1393: f64 = (v1387 * v1391);
        let v1394: f64 = (v1388 * v1391);
        let v1395: f64 = (v1389 * v1391);
        let v1396: f64 = (v1390 * v1391);
        let v1397: f64 = (-v1386);
        let v1398: f64 = (-v1387);
        let v1399: f64 = (-v1388);
        let v1400: f64 = (-v1389);
        let v1401: f64 = (-v1390);
        let v1402: f64 = { let limexp_arg = v302; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v1403: f64 = (v1397 * v1402);
        let v1404: f64 = (v1398 * v1402);
        let v1405: f64 = (v1399 * v1402);
        let v1406: f64 = (v1400 * v1402);
        let v1407: f64 = (v1401 * v1402);
        let v1408: f64 = (v1392 - v1403);
        let v1409: f64 = (v1393 - v1404);
        let v1410: f64 = (v1394 - v1405);
        let v1411: f64 = (v1395 - v1406);
        let v1412: f64 = (v1396 - v1407);
        let v1413: f64 = (v152 * v1408);
        let v1414: f64 = (v152 * v1409);
        let v1415: f64 = (v152 * v1410);
        let v1416: f64 = (v152 * v1411);
        let v1417: f64 = (v152 * v1412);
        let v1418: f64 = (v306 * v306);
        let v1419: f64 = (v44 - v1418);
        let v1420: f64 = (v1413 * v1419);
        let v1421: f64 = (v1414 * v1419);
        let v1422: f64 = (v1415 * v1419);
        let v1423: f64 = (v1416 * v1419);
        let v1424: f64 = (v1417 * v1419);
        let v1425: f64 = (if self.scalar_v290 { v1420 } else { v1004 });
        let v1426: f64 = (if self.scalar_v290 { v1421 } else { v1005 });
        let v1427: f64 = (if self.scalar_v290 { v1422 } else { v1006 });
        let v1428: f64 = (if self.scalar_v290 { v1423 } else { v1007 });
        let v1429: f64 = (if self.scalar_v290 { v1424 } else { v1008 });
        let v1430: f64 = (self.scalar_v184 * v1425);
        let v1431: f64 = (self.scalar_v184 * v1426);
        let v1432: f64 = (self.scalar_v184 * v1427);
        let v1433: f64 = (self.scalar_v184 * v1428);
        let v1434: f64 = (self.scalar_v184 * v1429);
        let v1435: f64 = (if self.scalar_v290 { v1430 } else { v13 });
        let v1436: f64 = (if self.scalar_v290 { v1431 } else { v13 });
        let v1437: f64 = (if self.scalar_v290 { v1432 } else { v13 });
        let v1438: f64 = (if self.scalar_v290 { v1433 } else { v13 });
        let v1439: f64 = (if self.scalar_v290 { v1434 } else { v13 });
        let v1440: f64 = (v7 * v1435);
        let v1441: f64 = (v7 * v1436);
        let v1442: f64 = (v311 + v1441);
        let v1443: f64 = (-v311);
        let v1444: f64 = (v7 * v1437);
        let v1445: f64 = (v1443 + v1444);
        let v1446: f64 = (v7 * v1438);
        let v1447: f64 = (v7 * v1439);
        let v1448: f64 = (v313 * v313);
        let v1449: f64 = (v44 - v1448);
        let v1450: f64 = (v1440 * v1449);
        let v1451: f64 = (v1442 * v1449);
        let v1452: f64 = (v1445 * v1449);
        let v1453: f64 = (v1446 * v1449);
        let v1454: f64 = (v1447 * v1449);
        let v1455: f64 = (if self.scalar_v290 { v1450 } else { v13 });
        let v1456: f64 = (if self.scalar_v290 { v1451 } else { v13 });
        let v1457: f64 = (if self.scalar_v290 { v1452 } else { v13 });
        let v1458: f64 = (if self.scalar_v290 { v1453 } else { v13 });
        let v1459: f64 = (if self.scalar_v290 { v1454 } else { v13 });
        let v1460: f64 = (self.scalar_v256 * v1425);
        let v1461: f64 = (self.scalar_v256 * v1426);
        let v1462: f64 = (self.scalar_v256 * v1427);
        let v1463: f64 = (self.scalar_v256 * v1428);
        let v1464: f64 = (self.scalar_v256 * v1429);
        let v1465: f64 = (if self.scalar_v290 { v1460 } else { v1168 });
        let v1466: f64 = (if self.scalar_v290 { v1461 } else { v1169 });
        let v1467: f64 = (if self.scalar_v290 { v1462 } else { v1170 });
        let v1468: f64 = (if self.scalar_v290 { v1463 } else { v1171 });
        let v1469: f64 = (if self.scalar_v290 { v1464 } else { v1172 });
        let v1470: f64 = (v135 * v1425);
        let v1471: f64 = (v135 * v1426);
        let v1472: f64 = (v135 * v1427);
        let v1473: f64 = (v135 * v1428);
        let v1474: f64 = (v135 * v1429);
        let v1475: f64 = (v318 * v1455);
        let v1476: f64 = (v314 * v1470);
        let v1477: f64 = (v1475 + v1476);
        let v1478: f64 = (v318 * v1456);
        let v1479: f64 = (v314 * v1471);
        let v1480: f64 = (v1478 + v1479);
        let v1481: f64 = (v318 * v1457);
        let v1482: f64 = (v314 * v1472);
        let v1483: f64 = (v1481 + v1482);
        let v1484: f64 = (v318 * v1458);
        let v1485: f64 = (v314 * v1473);
        let v1486: f64 = (v1484 + v1485);
        let v1487: f64 = (v318 * v1459);
        let v1488: f64 = (v314 * v1474);
        let v1489: f64 = (v1487 + v1488);
        let v1490: f64 = (v7 * v1465);
        let v1491: f64 = (v7 * v1466);
        let v1492: f64 = (v317 + v1491);
        let v1493: f64 = (-v317);
        let v1494: f64 = (v7 * v1467);
        let v1495: f64 = (v1493 + v1494);
        let v1496: f64 = (v7 * v1468);
        let v1497: f64 = (v7 * v1469);
        let v1498: f64 = { let limexp_arg = v322; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v1499: f64 = (self.scalar_v264 * v1498);
        let v1500: f64 = (self.scalar_v1191 * v1498);
        let v1501: f64 = (v136 * v1499);
        let v1502: f64 = (v136 * v1500);
        let v1503: f64 = (v1492 + v1501);
        let v1504: f64 = (v1496 + v1502);
        let v1505: f64 = (v325 * v1477);
        let v1506: f64 = (v319 * v1490);
        let v1507: f64 = (v1505 + v1506);
        let v1508: f64 = (v325 * v1480);
        let v1509: f64 = (v319 * v1503);
        let v1510: f64 = (v1508 + v1509);
        let v1511: f64 = (v325 * v1483);
        let v1512: f64 = (v319 * v1495);
        let v1513: f64 = (v1511 + v1512);
        let v1514: f64 = (v325 * v1486);
        let v1515: f64 = (v319 * v1504);
        let v1516: f64 = (v1514 + v1515);
        let v1517: f64 = (v325 * v1489);
        let v1518: f64 = (v319 * v1497);
        let v1519: f64 = (v1517 + v1518);
        let v1520: f64 = (if self.scalar_v290 { v1507 } else { v1317 });
        let v1521: f64 = (if self.scalar_v290 { v1510 } else { v1318 });
        let v1522: f64 = (if self.scalar_v290 { v1513 } else { v1319 });
        let v1523: f64 = (if self.scalar_v290 { v1516 } else { v1320 });
        let v1524: f64 = (if self.scalar_v290 { v1519 } else { v1321 });
        let v1525: f64 = (if self.scalar_v330 { v913 } else { v1322 });
        let v1526: f64 = (if self.scalar_v330 { v914 } else { v1323 });
        let v1527: f64 = (if self.scalar_v330 { v915 } else { v1324 });
        let v1528: f64 = (if self.scalar_v330 { v916 } else { v1325 });
        let v1530: f64 = (v331 * v1525);
        let v1531: f64 = (v1530 + v1530);
        let v1532: f64 = (v331 * v1526);
        let v1533: f64 = (v1532 + v1532);
        let v1534: f64 = (v331 * v1527);
        let v1535: f64 = (v1534 + v1534);
        let v1536: f64 = (v331 * v1528);
        let v1537: f64 = (v1536 + v1536);
        let v1538: f64 = (v331 * self.scalar_v1529);
        let v1539: f64 = (v1538 + v1538);
        let v1540: f64 = (if self.scalar_v330 { v1531 } else { v1337 });
        let v1541: f64 = (if self.scalar_v330 { v1533 } else { v1338 });
        let v1542: f64 = (if self.scalar_v330 { v1535 } else { v1339 });
        let v1543: f64 = (if self.scalar_v330 { v1537 } else { v1340 });
        let v1544: f64 = (if self.scalar_v330 { v1539 } else { v1341 });
        let v1545: f64 = (self.scalar_v201 * v1540);
        let v1546: f64 = (self.scalar_v201 * v1541);
        let v1547: f64 = (self.scalar_v201 * v1542);
        let v1548: f64 = (self.scalar_v201 * v1543);
        let v1549: f64 = (self.scalar_v201 * v1544);
        let v1550: f64 = (v1525 + v1545);
        let v1551: f64 = (v1526 + v1546);
        let v1552: f64 = (v1527 + v1547);
        let v1553: f64 = (v1528 + v1548);
        let v1554: f64 = (self.scalar_v1529 + v1549);
        let v1555: f64 = (v181 * v1540);
        let v1556: f64 = (v181 * v1541);
        let v1557: f64 = (v181 * v1542);
        let v1558: f64 = (v181 * v1543);
        let v1559: f64 = (v181 * v1544);
        let v1560: f64 = (v336 * v1525);
        let v1561: f64 = (v331 * v1555);
        let v1562: f64 = (v1560 + v1561);
        let v1563: f64 = (v336 * v1526);
        let v1564: f64 = (v331 * v1556);
        let v1565: f64 = (v1563 + v1564);
        let v1566: f64 = (v336 * v1527);
        let v1567: f64 = (v331 * v1557);
        let v1568: f64 = (v1566 + v1567);
        let v1569: f64 = (v336 * v1528);
        let v1570: f64 = (v331 * v1558);
        let v1571: f64 = (v1569 + v1570);
        let v1572: f64 = (v336 * self.scalar_v1529);
        let v1573: f64 = (v331 * v1559);
        let v1574: f64 = (v1572 + v1573);
        let v1575: f64 = (v1550 + v1562);
        let v1576: f64 = (v1551 + v1565);
        let v1577: f64 = (v1552 + v1568);
        let v1578: f64 = (v1553 + v1571);
        let v1579: f64 = (v1554 + v1574);
        let v1580: f64 = (v176 * v1575);
        let v1581: f64 = (v338 * v892);
        let v1582: f64 = (v176 * v1576);
        let v1583: f64 = (v1581 + v1582);
        let v1584: f64 = (v338 * v893);
        let v1585: f64 = (v176 * v1577);
        let v1586: f64 = (v1584 + v1585);
        let v1587: f64 = (v176 * v1578);
        let v1588: f64 = (v176 * v1579);
        let v1589: f64 = (if self.scalar_v330 { v1580 } else { v1386 });
        let v1590: f64 = (if self.scalar_v330 { v1583 } else { v1387 });
        let v1591: f64 = (if self.scalar_v330 { v1586 } else { v1388 });
        let v1592: f64 = (if self.scalar_v330 { v1587 } else { v1389 });
        let v1593: f64 = (if self.scalar_v330 { v1588 } else { v1390 });
        let v1594: f64 = (if self.scalar_v330 { v913 } else { v1104 });
        let v1595: f64 = (if self.scalar_v330 { v1071 } else { v1105 });
        let v1596: f64 = (if self.scalar_v330 { v1072 } else { v1106 });
        let v1597: f64 = (if self.scalar_v330 { v1073 } else { v1107 });
        let v1598: f64 = (if self.scalar_v330 { v13 } else { v1108 });
        let v1599: f64 = (v341 * v1594);
        let v1600: f64 = (v1599 + v1599);
        let v1601: f64 = (v341 * v1595);
        let v1602: f64 = (v1601 + v1601);
        let v1603: f64 = (v341 * v1596);
        let v1604: f64 = (v1603 + v1603);
        let v1605: f64 = (v341 * v1597);
        let v1606: f64 = (v1605 + v1605);
        let v1607: f64 = (v341 * v1598);
        let v1608: f64 = (v1607 + v1607);
        let v1609: f64 = (if self.scalar_v330 { v1600 } else { v13 });
        let v1610: f64 = (if self.scalar_v330 { v1602 } else { v13 });
        let v1611: f64 = (if self.scalar_v330 { v1604 } else { v13 });
        let v1612: f64 = (if self.scalar_v330 { v1606 } else { v13 });
        let v1613: f64 = (if self.scalar_v330 { v1608 } else { v13 });
        let v1614: f64 = (self.scalar_v201 * v1609);
        let v1615: f64 = (self.scalar_v201 * v1610);
        let v1616: f64 = (self.scalar_v201 * v1611);
        let v1617: f64 = (self.scalar_v201 * v1612);
        let v1618: f64 = (self.scalar_v201 * v1613);
        let v1619: f64 = (v1594 + v1614);
        let v1620: f64 = (v1595 + v1615);
        let v1621: f64 = (v1596 + v1616);
        let v1622: f64 = (v1597 + v1617);
        let v1623: f64 = (v1598 + v1618);
        let v1624: f64 = (v181 * v1594);
        let v1625: f64 = (v181 * v1595);
        let v1626: f64 = (v181 * v1596);
        let v1627: f64 = (v181 * v1597);
        let v1628: f64 = (v181 * v1598);
        let v1629: f64 = (v346 * v1609);
        let v1630: f64 = (v343 * v1624);
        let v1631: f64 = (v1629 + v1630);
        let v1632: f64 = (v346 * v1610);
        let v1633: f64 = (v343 * v1625);
        let v1634: f64 = (v1632 + v1633);
        let v1635: f64 = (v346 * v1611);
        let v1636: f64 = (v343 * v1626);
        let v1637: f64 = (v1635 + v1636);
        let v1638: f64 = (v346 * v1612);
        let v1639: f64 = (v343 * v1627);
        let v1640: f64 = (v1638 + v1639);
        let v1641: f64 = (v346 * v1613);
        let v1642: f64 = (v343 * v1628);
        let v1643: f64 = (v1641 + v1642);
        let v1644: f64 = (v1619 + v1631);
        let v1645: f64 = (v1620 + v1634);
        let v1646: f64 = (v1621 + v1637);
        let v1647: f64 = (v1622 + v1640);
        let v1648: f64 = (v1623 + v1643);
        let v1649: f64 = (v176 * v1644);
        let v1650: f64 = (v348 * v892);
        let v1651: f64 = (v176 * v1645);
        let v1652: f64 = (v1650 + v1651);
        let v1653: f64 = (v348 * v893);
        let v1654: f64 = (v176 * v1646);
        let v1655: f64 = (v1653 + v1654);
        let v1656: f64 = (v176 * v1647);
        let v1657: f64 = (v176 * v1648);
        let v1658: f64 = (if self.scalar_v330 { v1649 } else { v1136 });
        let v1659: f64 = (if self.scalar_v330 { v1652 } else { v1137 });
        let v1660: f64 = (if self.scalar_v330 { v1655 } else { v1138 });
        let v1661: f64 = (if self.scalar_v330 { v1656 } else { v1139 });
        let v1662: f64 = (if self.scalar_v330 { v1657 } else { v1140 });
        let v1663: f64 = { let limexp_arg = v340; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v1664: f64 = (v1589 * v1663);
        let v1665: f64 = (v1590 * v1663);
        let v1666: f64 = (v1591 * v1663);
        let v1667: f64 = (v1592 * v1663);
        let v1668: f64 = (v1593 * v1663);
        let v1669: f64 = (-v1589);
        let v1670: f64 = (-v1590);
        let v1671: f64 = (-v1591);
        let v1672: f64 = (-v1592);
        let v1673: f64 = (-v1593);
        let v1674: f64 = { let limexp_arg = v352; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v1675: f64 = (v1669 * v1674);
        let v1676: f64 = (v1670 * v1674);
        let v1677: f64 = (v1671 * v1674);
        let v1678: f64 = (v1672 * v1674);
        let v1679: f64 = (v1673 * v1674);
        let v1680: f64 = (v1664 - v1675);
        let v1681: f64 = (v1665 - v1676);
        let v1682: f64 = (v1666 - v1677);
        let v1683: f64 = (v1667 - v1678);
        let v1684: f64 = (v1668 - v1679);
        let v1685: f64 = (v152 * v1680);
        let v1686: f64 = (v152 * v1681);
        let v1687: f64 = (v152 * v1682);
        let v1688: f64 = (v152 * v1683);
        let v1689: f64 = (v152 * v1684);
        let v1690: f64 = (v356 * v356);
        let v1691: f64 = (v44 - v1690);
        let v1692: f64 = (v1685 * v1691);
        let v1693: f64 = (v1686 * v1691);
        let v1694: f64 = (v1687 * v1691);
        let v1695: f64 = (v1688 * v1691);
        let v1696: f64 = (v1689 * v1691);
        let v1697: f64 = (if self.scalar_v330 { v1692 } else { v1425 });
        let v1698: f64 = (if self.scalar_v330 { v1693 } else { v1426 });
        let v1699: f64 = (if self.scalar_v330 { v1694 } else { v1427 });
        let v1700: f64 = (if self.scalar_v330 { v1695 } else { v1428 });
        let v1701: f64 = (if self.scalar_v330 { v1696 } else { v1429 });
        let v1702: f64 = { let limexp_arg = v350; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v1703: f64 = (v1658 * v1702);
        let v1704: f64 = (v1659 * v1702);
        let v1705: f64 = (v1660 * v1702);
        let v1706: f64 = (v1661 * v1702);
        let v1707: f64 = (v1662 * v1702);
        let v1708: f64 = (-v1658);
        let v1709: f64 = (-v1659);
        let v1710: f64 = (-v1660);
        let v1711: f64 = (-v1661);
        let v1712: f64 = (-v1662);
        let v1713: f64 = { let limexp_arg = v360; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v1714: f64 = (v1708 * v1713);
        let v1715: f64 = (v1709 * v1713);
        let v1716: f64 = (v1710 * v1713);
        let v1717: f64 = (v1711 * v1713);
        let v1718: f64 = (v1712 * v1713);
        let v1719: f64 = (v1703 - v1714);
        let v1720: f64 = (v1704 - v1715);
        let v1721: f64 = (v1705 - v1716);
        let v1722: f64 = (v1706 - v1717);
        let v1723: f64 = (v1707 - v1718);
        let v1724: f64 = (v152 * v1719);
        let v1725: f64 = (v152 * v1720);
        let v1726: f64 = (v152 * v1721);
        let v1727: f64 = (v152 * v1722);
        let v1728: f64 = (v152 * v1723);
        let v1729: f64 = (v364 * v364);
        let v1730: f64 = (v44 - v1729);
        let v1731: f64 = (v1724 * v1730);
        let v1732: f64 = (v1725 * v1730);
        let v1733: f64 = (v1726 * v1730);
        let v1734: f64 = (v1727 * v1730);
        let v1735: f64 = (v1728 * v1730);
        let v1736: f64 = (if self.scalar_v330 { v1731 } else { v13 });
        let v1737: f64 = (if self.scalar_v330 { v1732 } else { v13 });
        let v1738: f64 = (if self.scalar_v330 { v1733 } else { v13 });
        let v1739: f64 = (if self.scalar_v330 { v1734 } else { v13 });
        let v1740: f64 = (if self.scalar_v330 { v1735 } else { v13 });
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
        let v1751: f64 = (self.scalar_v184 * v1736);
        let v1752: f64 = (self.scalar_v184 * v1737);
        let v1753: f64 = (self.scalar_v184 * v1738);
        let v1754: f64 = (self.scalar_v184 * v1739);
        let v1755: f64 = (self.scalar_v184 * v1740);
        let v1756: f64 = (if self.scalar_v330 { v1751 } else { v13 });
        let v1757: f64 = (if self.scalar_v330 { v1752 } else { v13 });
        let v1758: f64 = (if self.scalar_v330 { v1753 } else { v13 });
        let v1759: f64 = (if self.scalar_v330 { v1754 } else { v13 });
        let v1760: f64 = (if self.scalar_v330 { v1755 } else { v13 });
        let v1761: f64 = (v7 * v1746);
        let v1762: f64 = (v7 * v1747);
        let v1763: f64 = (v369 + v1762);
        let v1764: f64 = (-v369);
        let v1765: f64 = (v7 * v1748);
        let v1766: f64 = (v1764 + v1765);
        let v1767: f64 = (v7 * v1749);
        let v1768: f64 = (v7 * v1750);
        let v1769: f64 = (v374 * v374);
        let v1770: f64 = (v44 - v1769);
        let v1771: f64 = (v1761 * v1770);
        let v1772: f64 = (v1763 * v1770);
        let v1773: f64 = (v1766 * v1770);
        let v1774: f64 = (v1767 * v1770);
        let v1775: f64 = (v1768 * v1770);
        let v1776: f64 = (if self.scalar_v330 { v1771 } else { v1455 });
        let v1777: f64 = (if self.scalar_v330 { v1772 } else { v1456 });
        let v1778: f64 = (if self.scalar_v330 { v1773 } else { v1457 });
        let v1779: f64 = (if self.scalar_v330 { v1774 } else { v1458 });
        let v1780: f64 = (if self.scalar_v330 { v1775 } else { v1459 });
        let v1781: f64 = (v7 * v1756);
        let v1782: f64 = (v7 * v1757);
        let v1783: f64 = (v372 + v1782);
        let v1784: f64 = (-v372);
        let v1785: f64 = (v7 * v1758);
        let v1786: f64 = (v1784 + v1785);
        let v1787: f64 = (v7 * v1759);
        let v1788: f64 = (v7 * v1760);
        let v1789: f64 = (v377 * v377);
        let v1790: f64 = (v44 - v1789);
        let v1791: f64 = (v1781 * v1790);
        let v1792: f64 = (v1783 * v1790);
        let v1793: f64 = (v1786 * v1790);
        let v1794: f64 = (v1787 * v1790);
        let v1795: f64 = (v1788 * v1790);
        let v1796: f64 = (if self.scalar_v330 { v1791 } else { v13 });
        let v1797: f64 = (if self.scalar_v330 { v1792 } else { v13 });
        let v1798: f64 = (if self.scalar_v330 { v1793 } else { v13 });
        let v1799: f64 = (if self.scalar_v330 { v1794 } else { v13 });
        let v1800: f64 = (if self.scalar_v330 { v1795 } else { v13 });
        let v1801: f64 = (self.scalar_v256 * v1736);
        let v1802: f64 = (self.scalar_v256 * v1737);
        let v1803: f64 = (self.scalar_v256 * v1738);
        let v1804: f64 = (self.scalar_v256 * v1739);
        let v1805: f64 = (self.scalar_v256 * v1740);
        let v1806: f64 = (if self.scalar_v330 { v1801 } else { v13 });
        let v1807: f64 = (if self.scalar_v330 { v1802 } else { v13 });
        let v1808: f64 = (if self.scalar_v330 { v1803 } else { v13 });
        let v1809: f64 = (if self.scalar_v330 { v1804 } else { v13 });
        let v1810: f64 = (if self.scalar_v330 { v1805 } else { v13 });
        let v1811: f64 = (self.scalar_v256 * v1697);
        let v1812: f64 = (self.scalar_v256 * v1698);
        let v1813: f64 = (self.scalar_v256 * v1699);
        let v1814: f64 = (self.scalar_v256 * v1700);
        let v1815: f64 = (self.scalar_v256 * v1701);
        let v1816: f64 = (if self.scalar_v330 { v1811 } else { v13 });
        let v1817: f64 = (if self.scalar_v330 { v1812 } else { v13 });
        let v1818: f64 = (if self.scalar_v330 { v1813 } else { v13 });
        let v1819: f64 = (if self.scalar_v330 { v1814 } else { v13 });
        let v1820: f64 = (if self.scalar_v330 { v1815 } else { v13 });
        let v1821: f64 = (v135 * v1697);
        let v1822: f64 = (v135 * v1698);
        let v1823: f64 = (v135 * v1699);
        let v1824: f64 = (v135 * v1700);
        let v1825: f64 = (v135 * v1701);
        let v1826: f64 = (v386 * v1821);
        let v1827: f64 = (v385 * v1776);
        let v1828: f64 = (v1826 + v1827);
        let v1829: f64 = (v386 * v1822);
        let v1830: f64 = (v385 * v1777);
        let v1831: f64 = (v1829 + v1830);
        let v1832: f64 = (v386 * v1823);
        let v1833: f64 = (v385 * v1778);
        let v1834: f64 = (v1832 + v1833);
        let v1835: f64 = (v386 * v1824);
        let v1836: f64 = (v385 * v1779);
        let v1837: f64 = (v1835 + v1836);
        let v1838: f64 = (v386 * v1825);
        let v1839: f64 = (v385 * v1780);
        let v1840: f64 = (v1838 + v1839);
        let v1841: f64 = (v7 * v1816);
        let v1842: f64 = (v7 * v1817);
        let v1843: f64 = (v384 + v1842);
        let v1844: f64 = (-v384);
        let v1845: f64 = (v7 * v1818);
        let v1846: f64 = (v1844 + v1845);
        let v1847: f64 = (v7 * v1819);
        let v1848: f64 = (v7 * v1820);
        let v1849: f64 = (v1195 + v1843);
        let v1850: f64 = (v1196 + v1846);
        let v1851: f64 = (v390 * v1828);
        let v1852: f64 = (v387 * v1841);
        let v1853: f64 = (v1851 + v1852);
        let v1854: f64 = (v390 * v1831);
        let v1855: f64 = (v387 * v1849);
        let v1856: f64 = (v1854 + v1855);
        let v1857: f64 = (v390 * v1834);
        let v1858: f64 = (v387 * v1850);
        let v1859: f64 = (v1857 + v1858);
        let v1860: f64 = (v390 * v1837);
        let v1861: f64 = (v387 * v1847);
        let v1862: f64 = (v1860 + v1861);
        let v1863: f64 = (v390 * v1840);
        let v1864: f64 = (v387 * v1848);
        let v1865: f64 = (v1863 + v1864);
        let v1866: f64 = (if self.scalar_v330 { v1853 } else { v1214 });
        let v1867: f64 = (if self.scalar_v330 { v1856 } else { v1215 });
        let v1868: f64 = (if self.scalar_v330 { v1859 } else { v1216 });
        let v1869: f64 = (if self.scalar_v330 { v1862 } else { v1217 });
        let v1870: f64 = (if self.scalar_v330 { v1865 } else { v1218 });
        let v1871: f64 = (v135 * v1736);
        let v1872: f64 = (v135 * v1737);
        let v1873: f64 = (v135 * v1738);
        let v1874: f64 = (v135 * v1739);
        let v1875: f64 = (v135 * v1740);
        let v1876: f64 = (-v1796);
        let v1877: f64 = (-v1797);
        let v1878: f64 = (-v1798);
        let v1879: f64 = (-v1799);
        let v1880: f64 = (-v1800);
        let v1881: f64 = (v394 * v1871);
        let v1882: f64 = (v393 * v1876);
        let v1883: f64 = (v1881 + v1882);
        let v1884: f64 = (v394 * v1872);
        let v1885: f64 = (v393 * v1877);
        let v1886: f64 = (v1884 + v1885);
        let v1887: f64 = (v394 * v1873);
        let v1888: f64 = (v393 * v1878);
        let v1889: f64 = (v1887 + v1888);
        let v1890: f64 = (v394 * v1874);
        let v1891: f64 = (v393 * v1879);
        let v1892: f64 = (v1890 + v1891);
        let v1893: f64 = (v394 * v1875);
        let v1894: f64 = (v393 * v1880);
        let v1895: f64 = (v1893 + v1894);
        let v1896: f64 = (v7 * v1806);
        let v1897: f64 = (v7 * v1807);
        let v1898: f64 = (v381 + v1897);
        let v1899: f64 = (-v381);
        let v1900: f64 = (v7 * v1808);
        let v1901: f64 = (v1899 + v1900);
        let v1902: f64 = (v7 * v1809);
        let v1903: f64 = (v7 * v1810);
        let v1904: f64 = (-v1896);
        let v1905: f64 = (-v1898);
        let v1906: f64 = (-v1901);
        let v1907: f64 = (-v1902);
        let v1908: f64 = (-v1903);
        let v1909: f64 = (v397 * v1883);
        let v1910: f64 = (v395 * v1904);
        let v1911: f64 = (v1909 + v1910);
        let v1912: f64 = (v397 * v1886);
        let v1913: f64 = (v395 * v1905);
        let v1914: f64 = (v1912 + v1913);
        let v1915: f64 = (v397 * v1889);
        let v1916: f64 = (v395 * v1906);
        let v1917: f64 = (v1915 + v1916);
        let v1918: f64 = (v397 * v1892);
        let v1919: f64 = (v395 * v1907);
        let v1920: f64 = (v1918 + v1919);
        let v1921: f64 = (v397 * v1895);
        let v1922: f64 = (v395 * v1908);
        let v1923: f64 = (v1921 + v1922);
        let v1924: f64 = (if self.scalar_v330 { v1911 } else { v1302 });
        let v1925: f64 = (if self.scalar_v330 { v1914 } else { v1303 });
        let v1926: f64 = (if self.scalar_v330 { v1917 } else { v1304 });
        let v1927: f64 = (if self.scalar_v330 { v1920 } else { v1305 });
        let v1928: f64 = (if self.scalar_v330 { v1923 } else { v1306 });
        let v1929: f64 = (v1866 - v1924);
        let v1930: f64 = (v1867 - v1925);
        let v1931: f64 = (v1868 - v1926);
        let v1932: f64 = (v1869 - v1927);
        let v1933: f64 = (v1870 - v1928);
        let v1934: f64 = (v152 * v1929);
        let v1935: f64 = (v152 * v1930);
        let v1936: f64 = (v152 * v1931);
        let v1937: f64 = (v152 * v1932);
        let v1938: f64 = (v152 * v1933);
        let v1939: f64 = (if self.scalar_v330 { v1934 } else { v1520 });
        let v1940: f64 = (if self.scalar_v330 { v1935 } else { v1521 });
        let v1941: f64 = (if self.scalar_v330 { v1936 } else { v1522 });
        let v1942: f64 = (if self.scalar_v330 { v1937 } else { v1523 });
        let v1943: f64 = (if self.scalar_v330 { v1938 } else { v1524 });
        let v1944: f64 = (if self.scalar_v405 { v1163 } else { v1465 });
        let v1945: f64 = (if self.scalar_v405 { v1164 } else { v1466 });
        let v1946: f64 = (if self.scalar_v405 { v1165 } else { v1467 });
        let v1947: f64 = (if self.scalar_v405 { v1166 } else { v1468 });
        let v1948: f64 = (if self.scalar_v405 { v1167 } else { v1469 });
        let v1949: f64 = (if self.scalar_v405 { v1741 } else { v1746 });
        let v1950: f64 = (if self.scalar_v405 { v1742 } else { v1747 });
        let v1951: f64 = (if self.scalar_v405 { v1743 } else { v1748 });
        let v1952: f64 = (if self.scalar_v405 { v1744 } else { v1749 });
        let v1953: f64 = (if self.scalar_v405 { v1745 } else { v1750 });
        let v1954: f64 = (v7 * v1949);
        let v1955: f64 = (v7 * v1950);
        let v1956: f64 = (v407 + v1955);
        let v1957: f64 = (-v407);
        let v1958: f64 = (v7 * v1951);
        let v1959: f64 = (v1957 + v1958);
        let v1960: f64 = (v7 * v1952);
        let v1961: f64 = (v7 * v1953);
        let v1962: f64 = (v409 * v409);
        let v1963: f64 = (v44 - v1962);
        let v1964: f64 = (v1954 * v1963);
        let v1965: f64 = (v1956 * v1963);
        let v1966: f64 = (v1959 * v1963);
        let v1967: f64 = (v1960 * v1963);
        let v1968: f64 = (v1961 * v1963);
        let v1969: f64 = (if self.scalar_v405 { v1964 } else { v1776 });
        let v1970: f64 = (if self.scalar_v405 { v1965 } else { v1777 });
        let v1971: f64 = (if self.scalar_v405 { v1966 } else { v1778 });
        let v1972: f64 = (if self.scalar_v405 { v1967 } else { v1779 });
        let v1973: f64 = (if self.scalar_v405 { v1968 } else { v1780 });
        let v1974: f64 = (v11 * v1949);
        let v1975: f64 = (v407 + v1974);
        let v1976: f64 = (v11 * v1950);
        let v1977: f64 = (v11 * v1951);
        let v1978: f64 = (v1957 + v1977);
        let v1979: f64 = (v11 * v1952);
        let v1980: f64 = (v11 * v1953);
        let v1981: f64 = (v412 * v412);
        let v1982: f64 = (v44 - v1981);
        let v1983: f64 = (v1975 * v1982);
        let v1984: f64 = (v1976 * v1982);
        let v1985: f64 = (v1978 * v1982);
        let v1986: f64 = (v1979 * v1982);
        let v1987: f64 = (v1980 * v1982);
        let v1988: f64 = (if self.scalar_v405 { v1983 } else { v13 });
        let v1989: f64 = (if self.scalar_v405 { v1984 } else { v13 });
        let v1990: f64 = (if self.scalar_v405 { v1985 } else { v13 });
        let v1991: f64 = (if self.scalar_v405 { v1986 } else { v13 });
        let v1992: f64 = (if self.scalar_v405 { v1987 } else { v13 });
        let v1993: f64 = (self.scalar_v414 * v1988);
        let v1994: f64 = (self.scalar_v414 * v1989);
        let v1995: f64 = (self.scalar_v414 * v1990);
        let v1996: f64 = (self.scalar_v414 * v1991);
        let v1997: f64 = (self.scalar_v414 * v1992);
        let v1998: f64 = (v1969 + v1993);
        let v1999: f64 = (v1970 + v1994);
        let v2000: f64 = (v1971 + v1995);
        let v2001: f64 = (v1972 + v1996);
        let v2002: f64 = (v1973 + v1997);
        let v2003: f64 = (v416 * v1029);
        let v2004: f64 = (v226 * v1998);
        let v2005: f64 = (v2003 + v2004);
        let v2006: f64 = (v416 * v1030);
        let v2007: f64 = (v226 * v1999);
        let v2008: f64 = (v2006 + v2007);
        let v2009: f64 = (v416 * v1031);
        let v2010: f64 = (v226 * v2000);
        let v2011: f64 = (v2009 + v2010);
        let v2012: f64 = (v416 * v1032);
        let v2013: f64 = (v226 * v2001);
        let v2014: f64 = (v2012 + v2013);
        let v2015: f64 = (v416 * v1033);
        let v2016: f64 = (v226 * v2002);
        let v2017: f64 = (v2015 + v2016);
        let v2020: f64 = (v419 * v1944);
        let v2021: f64 = (v406 * self.scalar_v414);
        let v2022: f64 = (v2020 + v2021);
        let v2023: f64 = (v419 * v1945);
        let v2024: f64 = (v406 + v2023);
        let v2025: f64 = (v419 * v1946);
        let v2026: f64 = (v406 * self.scalar_v2019);
        let v2027: f64 = (v2025 + v2026);
        let v2028: f64 = (v419 * v1947);
        let v2029: f64 = (v419 * v1948);
        let v2030: f64 = (v1195 + v2024);
        let v2031: f64 = (v1196 + v2027);
        let v2032: f64 = (v422 * v2005);
        let v2033: f64 = (v417 * v2022);
        let v2034: f64 = (v2032 + v2033);
        let v2035: f64 = (v422 * v2008);
        let v2036: f64 = (v417 * v2030);
        let v2037: f64 = (v2035 + v2036);
        let v2038: f64 = (v422 * v2011);
        let v2039: f64 = (v417 * v2031);
        let v2040: f64 = (v2038 + v2039);
        let v2041: f64 = (v422 * v2014);
        let v2042: f64 = (v417 * v2028);
        let v2043: f64 = (v2041 + v2042);
        let v2044: f64 = (v422 * v2017);
        let v2045: f64 = (v417 * v2029);
        let v2046: f64 = (v2044 + v2045);
        let v2047: f64 = (if self.scalar_v405 { v2034 } else { v1939 });
        let v2048: f64 = (if self.scalar_v405 { v2037 } else { v1940 });
        let v2049: f64 = (if self.scalar_v405 { v2040 } else { v1941 });
        let v2050: f64 = (if self.scalar_v405 { v2043 } else { v1942 });
        let v2051: f64 = (if self.scalar_v405 { v2046 } else { v1943 });
        let v2052: f64 = (v139 * v970);
        let v2053: f64 = (-v2052);
        let v2054: f64 = (v427 * v427);
        let v2055: f64 = (v2053 / v2054);
        let v2056: f64 = (v139 * v971);
        let v2057: f64 = (-v2056);
        let v2058: f64 = (v2057 / v2054);
        let v2059: f64 = (v139 * v972);
        let v2060: f64 = (-v2059);
        let v2061: f64 = (v2060 / v2054);
        let v2062: f64 = (v139 * v973);
        let v2063: f64 = (-v2062);
        let v2064: f64 = (v2063 / v2054);
        let v2065: f64 = (v139 * v974);
        let v2066: f64 = (-v2065);
        let v2067: f64 = (v2066 / v2054);
        let v2068: f64 = (if self.scalar_v425 { v2055 } else { v13 });
        let v2069: f64 = (if self.scalar_v425 { v2058 } else { v13 });
        let v2070: f64 = (if self.scalar_v425 { v2061 } else { v13 });
        let v2071: f64 = (if self.scalar_v425 { v2064 } else { v13 });
        let v2072: f64 = (if self.scalar_v425 { v2067 } else { v13 });
        let v2073: f64 = (self.scalar_v432 * v970);
        let v2074: f64 = (self.scalar_v432 * v971);
        let v2075: f64 = (self.scalar_v432 * v972);
        let v2076: f64 = (self.scalar_v432 * v973);
        let v2077: f64 = (self.scalar_v432 * v974);
        let v2078: f64 = (if self.scalar_v425 { v2073 } else { v13 });
        let v2079: f64 = (if self.scalar_v425 { v2074 } else { v13 });
        let v2080: f64 = (if self.scalar_v425 { v2075 } else { v13 });
        let v2081: f64 = (if self.scalar_v425 { v2076 } else { v13 });
        let v2082: f64 = (if self.scalar_v425 { v2077 } else { v13 });
        let v2083: f64 = (v139 * v1697);
        let v2084: f64 = (-v2083);
        let v2085: f64 = (v440 * v440);
        let v2086: f64 = (v2084 / v2085);
        let v2087: f64 = (v139 * v1698);
        let v2088: f64 = (-v2087);
        let v2089: f64 = (v2088 / v2085);
        let v2090: f64 = (v139 * v1699);
        let v2091: f64 = (-v2090);
        let v2092: f64 = (v2091 / v2085);
        let v2093: f64 = (v139 * v1700);
        let v2094: f64 = (-v2093);
        let v2095: f64 = (v2094 / v2085);
        let v2096: f64 = (v139 * v1701);
        let v2097: f64 = (-v2096);
        let v2098: f64 = (v2097 / v2085);
        let v2099: f64 = (if self.scalar_v439 { v2086 } else { v2068 });
        let v2100: f64 = (if self.scalar_v439 { v2089 } else { v2069 });
        let v2101: f64 = (if self.scalar_v439 { v2092 } else { v2070 });
        let v2102: f64 = (if self.scalar_v439 { v2095 } else { v2071 });
        let v2103: f64 = (if self.scalar_v439 { v2098 } else { v2072 });
        let v2104: f64 = (self.scalar_v432 * v1697);
        let v2105: f64 = (self.scalar_v432 * v1698);
        let v2106: f64 = (self.scalar_v432 * v1699);
        let v2107: f64 = (self.scalar_v432 * v1700);
        let v2108: f64 = (self.scalar_v432 * v1701);
        let v2109: f64 = (if self.scalar_v439 { v2104 } else { v2078 });
        let v2110: f64 = (if self.scalar_v439 { v2105 } else { v2079 });
        let v2111: f64 = (if self.scalar_v439 { v2106 } else { v2080 });
        let v2112: f64 = (if self.scalar_v439 { v2107 } else { v2081 });
        let v2113: f64 = (if self.scalar_v439 { v2108 } else { v2082 });
        let v2114: f64 = (v451 * v2109);
        let v2115: f64 = (v451 * v2110);
        let v2116: f64 = (v451 * v2111);
        let v2117: f64 = (v451 * v2112);
        let v2118: f64 = (v451 * v2113);
        let v2119: f64 = (if self.scalar_v455 { v13 } else { v1525 });
        let v2120: f64 = (if self.scalar_v455 { v13 } else { v1526 });
        let v2121: f64 = (if self.scalar_v455 { v13 } else { v1527 });
        let v2122: f64 = (if self.scalar_v455 { v13 } else { v1528 });
        let v2126: f64 = (if self.scalar_v473 { v13 } else { v2119 });
        let v2127: f64 = (if self.scalar_v473 { v13 } else { v2120 });
        let v2128: f64 = (if self.scalar_v473 { v13 } else { v2121 });
        let v2129: f64 = (if self.scalar_v473 { v13 } else { v2122 });
        let v2131: f64 = (v488 * v488);
        let v2132: f64 = (v44 - v2131);
        let v2133: f64 = (-v2132);
        let v2134: f64 = (if self.scalar_v487 { v2133 } else { self.scalar_v2124 });
        let v2135: f64 = (if self.scalar_v487 { v2132 } else { self.scalar_v2125 });
        let v2136: f64 = (v490 * v490);
        let v2137: f64 = (v44 - v2136);
        let v2138: f64 = (-v2137);
        let v2139: f64 = (if self.scalar_v487 { v2138 } else { self.scalar_v2124 });
        let v2140: f64 = (if self.scalar_v487 { v2137 } else { self.scalar_v2125 });
        let v2141: f64 = (if self.scalar_v493 { v456 } else { v2134 });
        let v2142: f64 = (if self.scalar_v493 { v44 } else { v2135 });
        let v2143: f64 = (if self.scalar_v493 { v456 } else { v2139 });
        let v2144: f64 = (if self.scalar_v493 { v44 } else { v2140 });
        let v2149: f64 = { let limexp_arg = v498; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v2150: f64 = (self.scalar_v2147 * v2149);
        let v2151: f64 = (self.scalar_v2148 * v2149);
        let v2152: f64 = (v159 * v2141);
        let v2153: f64 = (v159 * v2142);
        let v2154: f64 = { let limexp_arg = v502; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v2155: f64 = (v2152 * v2154);
        let v2156: f64 = (v2153 * v2154);
        let v2157: f64 = (self.scalar_v506 * v2150);
        let v2158: f64 = (self.scalar_v506 * v2151);
        let v2159: f64 = (v2155 - v2157);
        let v2160: f64 = (v2156 - v2158);
        let v2161: f64 = (-v2126);
        let v2162: f64 = (-v2127);
        let v2163: f64 = (v2159 - v2128);
        let v2164: f64 = (-v2129);
        let v2166: f64 = (self.scalar_v501 * v2161);
        let v2167: f64 = (self.scalar_v501 * v2162);
        let v2168: f64 = (self.scalar_v501 * v2163);
        let v2169: f64 = (self.scalar_v501 * v2164);
        let v2170: f64 = (self.scalar_v501 * v2160);
        let v2172: f64 = { let limexp_arg = v511; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v2173: f64 = (self.scalar_v2147 * v2172);
        let v2174: f64 = (self.scalar_v2148 * v2172);
        let v2175: f64 = (v159 * v2143);
        let v2176: f64 = (v159 * v2144);
        let v2177: f64 = { let limexp_arg = v514; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v2178: f64 = (v2175 * v2177);
        let v2179: f64 = (v2176 * v2177);
        let v2180: f64 = (self.scalar_v506 * v2173);
        let v2181: f64 = (self.scalar_v506 * v2174);
        let v2182: f64 = (v2178 - v2180);
        let v2183: f64 = (v2179 - v2181);
        let v2184: f64 = (v2182 - v2127);
        let v2185: f64 = (-v2128);
        let v2186: f64 = (v2183 - v2129);
        let v2187: f64 = (self.scalar_v501 * v2184);
        let v2188: f64 = (self.scalar_v501 * v2185);
        let v2189: f64 = (self.scalar_v501 * v2186);
        let v2193: f64 = (v526 * v526);
        let v2194: f64 = (v44 - v2193);
        let v2195: f64 = (self.scalar_v523 * v2194);
        let v2196: f64 = (self.scalar_v2192 * v2194);
        let v2197: f64 = (self.scalar_v520 * v2194);
        let v2199: f64 = (v532 * v532);
        let v2200: f64 = (v44 - v2199);
        let v2201: f64 = (self.scalar_v529 * v2200);
        let v2202: f64 = (self.scalar_v2198 * v2200);
        let v2204: f64 = (v538 * v538);
        let v2205: f64 = (v44 - v2204);
        let v2206: f64 = (self.scalar_v2203 * v2205);
        let v2207: f64 = (self.scalar_v535 * v2205);
        let v2210: f64 = (v545 * v545);
        let v2211: f64 = (v44 - v2210);
        let v2212: f64 = (self.scalar_v2209 * v2211);
        let v2213: f64 = (self.scalar_v523 * v2211);
        let v2214: f64 = (self.scalar_v541 * v2211);
        let v2215: f64 = (v137 * v2195);
        let v2216: f64 = (v137 * v2196);
        let v2217: f64 = (v137 * v2197);
        let v2218: f64 = (v557 * v2201);
        let v2219: f64 = (v533 * v2215);
        let v2220: f64 = (v2218 + v2219);
        let v2221: f64 = (v557 * v2202);
        let v2222: f64 = (v533 * v2216);
        let v2223: f64 = (v2221 + v2222);
        let v2224: f64 = (v533 * v2217);
        let v2225: f64 = (if self.scalar_v556 { v2220 } else { v13 });
        let v2226: f64 = (if self.scalar_v556 { v2223 } else { v13 });
        let v2227: f64 = (if self.scalar_v556 { v2224 } else { v13 });
        let v2228: f64 = (v546 * v2206);
        let v2229: f64 = (v540 * v2212);
        let v2230: f64 = (v2228 + v2229);
        let v2231: f64 = (v546 * v2207);
        let v2232: f64 = (v540 * v2213);
        let v2233: f64 = (v2231 + v2232);
        let v2234: f64 = (v540 * v2214);
        let v2235: f64 = (v138 * v2230);
        let v2236: f64 = (v138 * v2233);
        let v2237: f64 = (v138 * v2234);
        let v2238: f64 = (if self.scalar_v556 { v2235 } else { v13 });
        let v2239: f64 = (if self.scalar_v556 { v2236 } else { v13 });
        let v2240: f64 = (if self.scalar_v556 { v2237 } else { v13 });
        let v2241: f64 = ((v572) as f64).sinh();
        let v2242: f64 = (self.scalar_v523 * v2241);
        let v2243: f64 = (self.scalar_v2191 * v2241);
        let v2244: f64 = (if self.scalar_v569 { v2242 } else { v13 });
        let v2245: f64 = (if self.scalar_v569 { v2243 } else { v13 });
        let v2246: f64 = (v2244 / v574);
        let v2247: f64 = (v2245 / v574);
        let v2248: f64 = (if self.scalar_v569 { v2246 } else { v13 });
        let v2249: f64 = (if self.scalar_v569 { v2247 } else { v13 });
        let v2251: f64 = (self.scalar_v523 * v2250);
        let v2252: f64 = (self.scalar_v2192 * v2250);
        let v2254: f64 = (if self.scalar_v569 { v2251 } else { v13 });
        let v2255: f64 = (if self.scalar_v569 { v2252 } else { v13 });
        let v2257: f64 = (v2254 / v578);
        let v2258: f64 = (v2255 / v578);
        let v2260: f64 = (if self.scalar_v569 { v2257 } else { v13 });
        let v2261: f64 = (if self.scalar_v569 { v2258 } else { v13 });
        let v2263: f64 = (self.scalar_v523 + v2248);
        let v2264: f64 = (self.scalar_v2191 + v2249);
        let v2265: f64 = (if self.scalar_v569 { v2263 } else { v13 });
        let v2266: f64 = (if self.scalar_v569 { v2264 } else { v13 });
        let v2267: f64 = (self.scalar_v523 + v2260);
        let v2268: f64 = (self.scalar_v2192 + v2261);
        let v2270: f64 = (v2267 - v2265);
        let v2271: f64 = (v2268 - v2266);
        let v2272: f64 = (v584 * v2201);
        let v2273: f64 = (v571 * v2270);
        let v2274: f64 = (v2272 + v2273);
        let v2275: f64 = (v584 * v2202);
        let v2276: f64 = (v571 * v2271);
        let v2277: f64 = (v2275 + v2276);
        let v2279: f64 = (v2274 / self.scalar_v520);
        let v2280: f64 = (v2277 / self.scalar_v520);
        let v2283: f64 = (v2280 + self.scalar_v2282);
        let v2285: f64 = (v137 * v2279);
        let v2286: f64 = (v137 * v2283);
        let v2289: f64 = (v2286 + self.scalar_v2288);
        let v2291: f64 = (if self.scalar_v569 { v2285 } else { v13 });
        let v2292: f64 = (if self.scalar_v569 { v2289 } else { v13 });
        let v2294: f64 = ((v593) as f64).sinh();
        let v2295: f64 = (self.scalar_v2191 * v2294);
        let v2296: f64 = (self.scalar_v523 * v2294);
        let v2297: f64 = (if self.scalar_v569 { v2295 } else { v2244 });
        let v2298: f64 = (if self.scalar_v569 { v2296 } else { v2245 });
        let v2299: f64 = (v2297 / v595);
        let v2300: f64 = (v2298 / v595);
        let v2301: f64 = (if self.scalar_v569 { v2299 } else { v13 });
        let v2302: f64 = (if self.scalar_v569 { v2300 } else { v13 });
        let v2304: f64 = (self.scalar_v2209 * v2303);
        let v2305: f64 = (self.scalar_v523 * v2303);
        let v2307: f64 = (if self.scalar_v569 { v2304 } else { v2254 });
        let v2308: f64 = (if self.scalar_v569 { v2305 } else { v2255 });
        let v2311: f64 = (v2307 / v599);
        let v2312: f64 = (v2308 / v599);
        let v2314: f64 = (v2310 / v599);
        let v2315: f64 = (if self.scalar_v569 { v2311 } else { v13 });
        let v2316: f64 = (if self.scalar_v569 { v2312 } else { v13 });
        let v2318: f64 = (if self.scalar_v569 { v2314 } else { v13 });
        let v2319: f64 = (self.scalar_v2191 + v2301);
        let v2320: f64 = (self.scalar_v523 + v2302);
        let v2321: f64 = (if self.scalar_v569 { v2319 } else { v13 });
        let v2322: f64 = (if self.scalar_v569 { v2320 } else { v13 });
        let v2323: f64 = (self.scalar_v2209 + v2315);
        let v2324: f64 = (self.scalar_v523 + v2316);
        let v2326: f64 = (v2323 - v2321);
        let v2327: f64 = (v2324 - v2322);
        let v2328: f64 = (v605 * v2206);
        let v2329: f64 = (v540 * v2326);
        let v2330: f64 = (v2328 + v2329);
        let v2331: f64 = (v605 * v2207);
        let v2332: f64 = (v540 * v2327);
        let v2333: f64 = (v2331 + v2332);
        let v2335: f64 = (v540 * v2318);
        let v2336: f64 = (v2330 / self.scalar_v541);
        let v2337: f64 = (v2333 / self.scalar_v541);
        let v2339: f64 = (v2335 / self.scalar_v541);
        let v2340: f64 = (self.scalar_v2282 + v2336);
        let v2342: f64 = (v138 * v2340);
        let v2343: f64 = (v138 * v2337);
        let v2345: f64 = (v138 * v2339);
        let v2347: f64 = (v2342 + self.scalar_v2346);
        let v2349: f64 = (if self.scalar_v569 { v2347 } else { v13 });
        let v2350: f64 = (if self.scalar_v569 { v2343 } else { v13 });
        let v2352: f64 = (if self.scalar_v569 { v2345 } else { v13 });
        let v2353: f64 = (if self.scalar_v569 { v13 } else { v2225 });
        let v2354: f64 = (if self.scalar_v569 { v13 } else { v2226 });
        let v2355: f64 = (if self.scalar_v569 { v13 } else { v2227 });
        let v2356: f64 = (if self.scalar_v569 { v13 } else { v2238 });
        let v2357: f64 = (if self.scalar_v569 { v13 } else { v2239 });
        let v2358: f64 = (if self.scalar_v569 { v13 } else { v2240 });
        let v2363: f64 = (v624 * self.scalar_v2361);
        let v2364: f64 = (v2363 + v2363);
        let v2365: f64 = (v624 * self.scalar_v2362);
        let v2366: f64 = (v2365 + v2365);
        let v2368: f64 = f64::powf(v628, self.scalar_v2367);
        let v2369: f64 = (self.scalar_v629 * v2368);
        let v2370: f64 = (v2364 * v2369);
        let v2371: f64 = (v2366 * v2369);
        let v2372: f64 = (self.scalar_v632 * v2364);
        let v2373: f64 = (self.scalar_v632 * v2366);
        let v2374: f64 = (v634 * v2370);
        let v2375: f64 = (v630 * v2372);
        let v2376: f64 = (v2374 + v2375);
        let v2377: f64 = (v634 * v2371);
        let v2378: f64 = (v630 * v2373);
        let v2379: f64 = (v2377 + v2378);
        let v2380: f64 = (if self.scalar_v620 { v2376 } else { v13 });
        let v2381: f64 = (if self.scalar_v620 { v2379 } else { v13 });
        let v2385: f64 = (v640 * v640);
        let v2386: f64 = (v44 - v2385);
        let v2387: f64 = (self.scalar_v2383 * v2386);
        let v2388: f64 = (self.scalar_v2384 * v2386);
        let v2389: f64 = (self.scalar_v520 * v2386);
        let v2390: f64 = (if self.scalar_v620 { v2387 } else { v2195 });
        let v2391: f64 = (if self.scalar_v620 { v2388 } else { v2196 });
        let v2392: f64 = (if self.scalar_v620 { v2389 } else { v2197 });
        let v2397: f64 = (v651 * v651);
        let v2398: f64 = (v44 - v2397);
        let v2399: f64 = (self.scalar_v2395 * v2398);
        let v2400: f64 = (self.scalar_v2396 * v2398);
        let v2401: f64 = (self.scalar_v541 * v2398);
        let v2402: f64 = (if self.scalar_v620 { v2399 } else { v2212 });
        let v2403: f64 = (if self.scalar_v620 { v2400 } else { v2213 });
        let v2404: f64 = (if self.scalar_v620 { v2401 } else { v2214 });
        let v2405: f64 = (self.scalar_v654 * v2380);
        let v2406: f64 = (self.scalar_v654 * v2381);
        let v2407: f64 = (v2391 + v2405);
        let v2408: f64 = (v2392 + v2406);
        let v2409: f64 = (v137 * v2390);
        let v2410: f64 = (v137 * v2407);
        let v2411: f64 = (v137 * v2408);
        let v2412: f64 = (v657 * v2201);
        let v2413: f64 = (v643 * v2409);
        let v2414: f64 = (v2412 + v2413);
        let v2415: f64 = (v657 * v2202);
        let v2416: f64 = (v643 * v2410);
        let v2417: f64 = (v2415 + v2416);
        let v2418: f64 = (v643 * v2411);
        let v2419: f64 = (if self.scalar_v620 { v2414 } else { v2353 });
        let v2420: f64 = (if self.scalar_v620 { v2417 } else { v2354 });
        let v2421: f64 = (if self.scalar_v620 { v2418 } else { v2355 });
        let v2422: f64 = (v653 * v2206);
        let v2423: f64 = (v646 * v2402);
        let v2424: f64 = (v2422 + v2423);
        let v2425: f64 = (v653 * v2207);
        let v2426: f64 = (v646 * v2403);
        let v2427: f64 = (v2425 + v2426);
        let v2428: f64 = (v646 * v2404);
        let v2429: f64 = (v138 * v2424);
        let v2430: f64 = (v138 * v2427);
        let v2431: f64 = (v138 * v2428);
        let v2432: f64 = (if self.scalar_v620 { v2429 } else { v2356 });
        let v2433: f64 = (if self.scalar_v620 { v2430 } else { v2357 });
        let v2434: f64 = (if self.scalar_v620 { v2431 } else { v2358 });
        let v2435: f64 = (if self.scalar_v668 { v2242 } else { v2297 });
        let v2436: f64 = (if self.scalar_v668 { v2243 } else { v2298 });
        let v2437: f64 = (v2435 / v669);
        let v2438: f64 = (v2436 / v669);
        let v2439: f64 = (if self.scalar_v668 { v2437 } else { v2248 });
        let v2440: f64 = (if self.scalar_v668 { v2438 } else { v2249 });
        let v2441: f64 = (if self.scalar_v668 { v2251 } else { v2307 });
        let v2442: f64 = (if self.scalar_v668 { v2252 } else { v2308 });
        let v2445: f64 = (v2441 / v672);
        let v2446: f64 = (v2442 / v672);
        let v2447: f64 = (v2443 / v672);
        let v2449: f64 = (if self.scalar_v668 { v2445 } else { v2260 });
        let v2450: f64 = (if self.scalar_v668 { v2446 } else { v2261 });
        let v2451: f64 = (if self.scalar_v668 { v2447 } else { v13 });
        let v2456: f64 = (self.scalar_v2359 * v2455);
        let v2461: f64 = (v2456 * v2460);
        let v2463: f64 = (v682 * self.scalar_v2453);
        let v2464: f64 = (v677 * v2461);
        let v2465: f64 = (v2463 + v2464);
        let v2469: f64 = (if self.scalar_v668 { v2465 } else { v13 });
        let v2471: f64 = (self.scalar_v523 + v2439);
        let v2472: f64 = (self.scalar_v2191 + v2440);
        let v2473: f64 = (if self.scalar_v668 { v2471 } else { v2265 });
        let v2474: f64 = (if self.scalar_v668 { v2472 } else { v2266 });
        let v2475: f64 = (self.scalar_v523 + v2449);
        let v2476: f64 = (self.scalar_v2192 + v2450);
        let v2478: f64 = (v2475 - v2473);
        let v2479: f64 = (v2476 - v2474);
        let v2480: f64 = (v2469 + v2479);
        let v2482: f64 = (v696 * v2478);
        let v2483: f64 = (v695 * v2201);
        let v2484: f64 = (v2482 + v2483);
        let v2485: f64 = (v696 * v2480);
        let v2486: f64 = (v695 * v2202);
        let v2487: f64 = (v2485 + v2486);
        let v2488: f64 = (v696 * v2451);
        let v2490: f64 = (v2484 / self.scalar_v520);
        let v2491: f64 = (v2487 / self.scalar_v520);
        let v2492: f64 = (v2488 / self.scalar_v520);
        let v2494: f64 = (self.scalar_v2282 + v2491);
        let v2496: f64 = (v137 * v2490);
        let v2497: f64 = (v137 * v2494);
        let v2498: f64 = (v137 * v2492);
        let v2500: f64 = (self.scalar_v2288 + v2497);
        let v2502: f64 = (if self.scalar_v668 { v2496 } else { v2291 });
        let v2503: f64 = (if self.scalar_v668 { v2500 } else { v2292 });
        let v2504: f64 = (if self.scalar_v668 { v2498 } else { v13 });
        let v2506: f64 = (if self.scalar_v668 { v2295 } else { v2435 });
        let v2507: f64 = (if self.scalar_v668 { v2296 } else { v2436 });
        let v2508: f64 = (v2506 / v703);
        let v2509: f64 = (v2507 / v703);
        let v2510: f64 = (if self.scalar_v668 { v2508 } else { v2301 });
        let v2511: f64 = (if self.scalar_v668 { v2509 } else { v2302 });
        let v2512: f64 = (if self.scalar_v668 { v2304 } else { v2441 });
        let v2513: f64 = (if self.scalar_v668 { v2305 } else { v2442 });
        let v2515: f64 = (if self.scalar_v668 { v13 } else { v2444 });
        let v2516: f64 = (v2512 / v706);
        let v2517: f64 = (v2513 / v706);
        let v2519: f64 = (v2515 / v706);
        let v2520: f64 = (if self.scalar_v668 { v2516 } else { v2315 });
        let v2521: f64 = (if self.scalar_v668 { v2517 } else { v2316 });
        let v2523: f64 = (if self.scalar_v668 { v2519 } else { v2318 });
        let v2524: f64 = (self.scalar_v2191 + v2510);
        let v2525: f64 = (self.scalar_v523 + v2511);
        let v2526: f64 = (if self.scalar_v668 { v2524 } else { v2321 });
        let v2527: f64 = (if self.scalar_v668 { v2525 } else { v2322 });
        let v2528: f64 = (self.scalar_v2209 + v2520);
        let v2529: f64 = (self.scalar_v523 + v2521);
        let v2531: f64 = (v2528 - v2526);
        let v2532: f64 = (v2529 - v2527);
        let v2533: f64 = (v712 * v2206);
        let v2534: f64 = (v645 * v2531);
        let v2535: f64 = (v2533 + v2534);
        let v2536: f64 = (v712 * v2207);
        let v2537: f64 = (v645 * v2532);
        let v2538: f64 = (v2536 + v2537);
        let v2540: f64 = (v645 * v2523);
        let v2541: f64 = (v2535 / self.scalar_v541);
        let v2542: f64 = (v2538 / self.scalar_v541);
        let v2544: f64 = (v2540 / self.scalar_v541);
        let v2545: f64 = (self.scalar_v2282 + v2541);
        let v2547: f64 = (v138 * v2545);
        let v2548: f64 = (v138 * v2542);
        let v2550: f64 = (v138 * v2544);
        let v2551: f64 = (self.scalar_v2346 + v2547);
        let v2553: f64 = (if self.scalar_v668 { v2551 } else { v2349 });
        let v2554: f64 = (if self.scalar_v668 { v2548 } else { v2350 });
        let v2556: f64 = (if self.scalar_v668 { v2550 } else { v2352 });
        let v2557: f64 = (if self.scalar_v668 { v13 } else { v2419 });
        let v2558: f64 = (if self.scalar_v668 { v13 } else { v2420 });
        let v2559: f64 = (if self.scalar_v668 { v13 } else { v2421 });
        let v2560: f64 = (if self.scalar_v668 { v13 } else { v2432 });
        let v2561: f64 = (if self.scalar_v668 { v13 } else { v2433 });
        let v2562: f64 = (if self.scalar_v668 { v13 } else { v2434 });
        let v2563: f64 = (-v2047);
        let v2564: f64 = (-v2048);
        let v2565: f64 = (-v2049);
        let v2566: f64 = (-v2050);
        let v2567: f64 = (-v2051);
        let v2568: f64 = ddt_scale;
        let v2569: f64 = (v2553 * v2568);
        let v2570: f64 = (v2554 * v2568);
        let v2571: f64 = (v2555 * v2568);
        let v2572: f64 = (v2556 * v2568);
        let v2573: f64 = (if self.scalar_v723 { v2569 } else { v13 });
        let v2574: f64 = (if self.scalar_v723 { v2570 } else { v13 });
        let v2575: f64 = (if self.scalar_v723 { v2571 } else { v13 });
        let v2576: f64 = (if self.scalar_v723 { v2572 } else { v13 });
        let v2577: f64 = (v2502 * v2568);
        let v2578: f64 = (v2503 * v2568);
        let v2579: f64 = (v2504 * v2568);
        let v2580: f64 = (v2505 * v2568);
        let v2581: f64 = (if self.scalar_v723 { v2577 } else { v13 });
        let v2582: f64 = (if self.scalar_v723 { v2578 } else { v13 });
        let v2583: f64 = (if self.scalar_v723 { v2579 } else { v13 });
        let v2584: f64 = (if self.scalar_v723 { v2580 } else { v13 });
        let v2585: f64 = (-v722);
        let v2586: f64 = (v5 * v2560);
        let v2587: f64 = (v2585 + v2586);
        let v2588: f64 = (v5 * v2561);
        let v2589: f64 = (v5 * v2562);
        let v2590: f64 = (v722 + v2589);
        let v2591: f64 = (v2568 * v2587);
        let v2592: f64 = (v2568 * v2588);
        let v2593: f64 = (v2568 * v2590);
        let v2594: f64 = (if self.scalar_v781 { v2591 } else { v13 });
        let v2595: f64 = (if self.scalar_v781 { v2592 } else { v13 });
        let v2596: f64 = (if self.scalar_v781 { v2593 } else { v13 });
        let v2597: f64 = (v9 * v2557);
        let v2598: f64 = (-v720);
        let v2599: f64 = (v9 * v2558);
        let v2600: f64 = (v2598 + v2599);
        let v2601: f64 = (v9 * v2559);
        let v2602: f64 = (v720 + v2601);
        let v2603: f64 = (v2568 * v2597);
        let v2604: f64 = (v2568 * v2600);
        let v2605: f64 = (v2568 * v2602);
        let v2606: f64 = (if self.scalar_v781 { v2603 } else { v13 });
        let v2607: f64 = (if self.scalar_v781 { v2604 } else { v13 });
        let v2608: f64 = (if self.scalar_v781 { v2605 } else { v13 });
        let v2611: f64 = (-v140);
        let v2612: f64 = -1e-12;
        let v2613: f64 = (v798 * v2099);
        let v2614: f64 = (v798 * v2100);
        let v2615: f64 = (v798 * v2101);
        let v2616: f64 = (v798 * v2102);
        let v2617: f64 = (v798 * v2103);
        let v2618: f64 = (self.scalar_v724 * v2568);
        let v2619: f64 = (v443 + v2618);
        let v2620: f64 = (if self.scalar_v725 { v2613 } else { v13 });
        let v2621: f64 = (if self.scalar_v725 { v2614 } else { v13 });
        let v2622: f64 = (if self.scalar_v725 { v2615 } else { v13 });
        let v2623: f64 = (if self.scalar_v725 { v2616 } else { v13 });
        let v2624: f64 = (if self.scalar_v725 { v2617 } else { v13 });
        let v2625: f64 = (if self.scalar_v725 { v2619 } else { v13 });
        let v2626: f64 = (v44 / v141);
        let v2627: f64 = (v456 / v141);
        let v2628: f64 = (if self.scalar_v728 { v2626 } else { v13 });
        let v2629: f64 = (if self.scalar_v728 { v2627 } else { v13 });
        let v2630: f64 = (-v142);
        let v2631: f64 = (v2568 * v2630);
        let v2632: f64 = (v142 * v2568);
        let v2633: f64 = (if self.scalar_v728 { v2631 } else { v13 });
        let v2634: f64 = (if self.scalar_v728 { v2632 } else { v13 });
        let v2649: f64 = (v830 * v2114);
        let v2650: f64 = (v830 * v2115);
        let v2651: f64 = (v830 * v2116);
        let v2652: f64 = (v830 * v2117);
        let v2653: f64 = (v830 * v2118);
        let v2654: f64 = (if self.scalar_v737 { v2649 } else { v13 });
        let v2655: f64 = (if self.scalar_v737 { v2650 } else { v13 });
        let v2656: f64 = (if self.scalar_v737 { v2651 } else { v13 });
        let v2657: f64 = (if self.scalar_v737 { v2652 } else { v13 });
        let v2658: f64 = (if self.scalar_v737 { v2653 } else { v13 });
        let v2659: f64 = (if self.scalar_v737 { v452 } else { v13 });
        let v2660: f64 = (v836 * v2114);
        let v2661: f64 = (v836 * v2115);
        let v2662: f64 = (v836 * v2116);
        let v2663: f64 = (v836 * v2117);
        let v2664: f64 = (v836 * v2118);
        let v2665: f64 = (if self.scalar_v740 { v2660 } else { v13 });
        let v2666: f64 = (if self.scalar_v740 { v2661 } else { v13 });
        let v2667: f64 = (if self.scalar_v740 { v2662 } else { v13 });
        let v2668: f64 = (if self.scalar_v740 { v2663 } else { v13 });
        let v2669: f64 = (if self.scalar_v740 { v2664 } else { v13 });
        let v2670: f64 = (if self.scalar_v740 { v453 } else { v13 });
        let v2672: f64 = (if self.scalar_v747 { v766 } else { v13 });
        let v2673: f64 = (if self.scalar_v747 { v762 } else { v13 });
        let v2674: f64 = (v854 * v2568);
        let v2675: f64 = (if self.scalar_v747 { v2674 } else { v13 });
        let v2676: f64 = (v44 / v49);
        let v2677: f64 = (if self.scalar_v769 { v2676 } else { v13 });
        let v2678: f64 = (self.scalar_v867 * v2568);
        let v2679: f64 = (if self.scalar_v769 { v2678 } else { v13 });

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
        let v13: f64 = 0.0;
        let v32: f64 = ctx.node_voltage(nodes[3]);
        let v33: f64 = ((v32) as f64).abs();
        let v34: f64 = (self.scalar_v23 + v33);
        let v35: f64 = (if (self.scalar_v31 != 0.0) { v34 } else { self.scalar_v23 });
        let v38: f64 = (v35 - self.scalar_v30);
        let v39: f64 = ((v38) as f64).abs();
        let v40: bool = (v39 > v13);
        let v43: bool = (v40 || self.scalar_v42);
        let v44: f64 = 1.0;
        let v46: f64 = ((v39) as f64).abs();
        let v64: f64 = (v46 * self.scalar_v63);
        let v65: f64 = (v44 + v64);
        let v66: f64 = (self.scalar_v62 * v65);
        let v67: f64 = (if v43 { v66 } else { v13 });
        let v70: f64 = (v46 * self.scalar_v69);
        let v71: f64 = (v44 + v70);
        let v72: f64 = (self.scalar_v68 * v71);
        let v73: f64 = (if v43 { v72 } else { v13 });
        let v76: f64 = (v46 * self.scalar_v75);
        let v77: f64 = (v44 + v76);
        let v78: f64 = (self.scalar_v74 * v77);
        let v79: f64 = (if v43 { v78 } else { v13 });
        let v82: f64 = (v46 * self.scalar_v81);
        let v83: f64 = (v44 + v82);
        let v84: f64 = (self.scalar_v80 * v83);
        let v85: f64 = (if v43 { v84 } else { v13 });
        let v88: f64 = (v39 * self.scalar_v87);
        let v89: f64 = (self.scalar_v86 + v88);
        let v90: f64 = (if v43 { v89 } else { v13 });
        let v93: f64 = (v39 * self.scalar_v92);
        let v94: f64 = (v44 + v93);
        let v95: f64 = (self.scalar_v91 * v94);
        let v96: f64 = (if v43 { v95 } else { v13 });
        let v98: f64 = (v94 * self.scalar_v97);
        let v99: f64 = (if v43 { v98 } else { v13 });
        let v107: f64 = (v39 * self.scalar_v106);
        let v108: f64 = (self.scalar_v105 + v107);
        let v109: f64 = (if v43 { v108 } else { v13 });
        let v118: bool = (v43 && self.scalar_v117);
        let v120: f64 = (v39 * v39);
        let v121: f64 = (self.scalar_v81 * v120);
        let v122: f64 = (v44 + v121);
        let v126: f64 = (v122 * self.scalar_v125);
        let v127: f64 = (if v118 { v126 } else { v13 });
        let v129: bool = (v43 && self.scalar_v128);
        let v132: f64 = (v83 * self.scalar_v125);
        let v133: f64 = (if v129 { v132 } else { v127 });
        let v134: bool = (!v43);
        let v137: f64 = (if v134 { self.scalar_v62 } else { v67 });
        let v138: f64 = (if v134 { self.scalar_v68 } else { v73 });
        let v139: f64 = (if v134 { self.scalar_v74 } else { v79 });
        let v140: f64 = (if v134 { self.scalar_v80 } else { v85 });
        let v142: f64 = (if v134 { self.scalar_v125 } else { v133 });
        let v143: f64 = (if v134 { self.scalar_v86 } else { v90 });
        let v144: f64 = (if v134 { self.scalar_v91 } else { v96 });
        let v145: f64 = (if v134 { self.scalar_v97 } else { v99 });
        let v147: f64 = (if v134 { self.scalar_v105 } else { v109 });
        let v152: f64 = 0.5;
        let v161: f64 = (v7 * self.scalar_v160);
        let v162: f64 = ((v161) as f64).cosh();
        let v164: f64 = (v11 * self.scalar_v163);
        let v167: f64 = 1e-12;
        let v168: f64 = (v162 * v162);
        let v169: f64 = (v167 + v168);
        let v170: f64 = (self.scalar_v166 / v169);
        let v171: f64 = (v44 + v170);
        let v172: f64 = (self.scalar_v165 * v171);
        let v174: f64 = (v46 * self.scalar_v173);
        let v175: f64 = (v44 + v174);
        let v176: f64 = (v172 * v175);
        let v179: f64 = (v46 * self.scalar_v178);
        let v180: f64 = (v44 + v179);
        let v181: f64 = (self.scalar_v177 * v180);
        let v183: f64 = (v143 - self.scalar_v182);
        let v185: f64 = (v7 * self.scalar_v184);
        let v186: f64 = ((v185) as f64).tanh();
        let v187: f64 = (self.scalar_v182 * v186);
        let v188: f64 = (v183 + v187);
        let v189: f64 = (v188 - v164);
        let v191: f64 = (v6 - v147);
        let v192: f64 = (self.scalar_v190 * v191);
        let v193: f64 = (v191 * v192);
        let v194: f64 = (v189 - v193);
        let v195: f64 = (v46 * self.scalar_v87);
        let v196: f64 = (v44 + v195);
        let v197: f64 = (v194 * v196);
        let v198: f64 = (v2 - v197);
        let v199: f64 = (v198 * v198);
        let v200: f64 = (v176 * v198);
        let v202: f64 = (v199 * self.scalar_v201);
        let v203: f64 = (v200 + v202);
        let v204: f64 = (v181 * v198);
        let v205: f64 = (v199 * v204);
        let v206: f64 = (v203 + v205);
        let v207: f64 = ((v206) as f64).tanh();
        let v208: f64 = (v44 + v207);
        let v209: f64 = { let limexp_arg = v206; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let v210: f64 = (-v206);
        let v211: f64 = { let limexp_arg = v210; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let v212: f64 = (v209 - v211);
        let v213: f64 = (v152 * v212);
        let v214: f64 = ((v213) as f64).tanh();
        let v215: f64 = (v44 + v214);
        let v222: f64 = 2.0;
        let v238: f64 = (v5 - v197);
        let v239: f64 = (if self.scalar_v237 { v238 } else { v162 });
        let v240: f64 = (v239 * v239);
        let v241: f64 = (if self.scalar_v237 { v240 } else { v198 });
        let v291: f64 = (if self.scalar_v290 { v198 } else { v239 });
        let v292: f64 = (v291 * v291);
        let v293: f64 = (if self.scalar_v290 { v292 } else { v241 });
        let v294: f64 = (self.scalar_v201 * v293);
        let v295: f64 = (v291 + v294);
        let v296: f64 = (v181 * v293);
        let v297: f64 = (v291 * v296);
        let v298: f64 = (v295 + v297);
        let v299: f64 = (v176 * v298);
        let v300: f64 = (if self.scalar_v290 { v299 } else { v206 });
        let v301: f64 = { let limexp_arg = v300; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let v302: f64 = (-v300);
        let v303: f64 = { let limexp_arg = v302; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let v304: f64 = (v301 - v303);
        let v305: f64 = (v152 * v304);
        let v306: f64 = ((v305) as f64).tanh();
        let v307: f64 = (v44 + v306);
        let v308: f64 = (if self.scalar_v290 { v307 } else { v215 });
        let v331: f64 = (if self.scalar_v330 { v198 } else { v291 });
        let v332: f64 = (v331 * v331);
        let v333: f64 = (if self.scalar_v330 { v332 } else { v293 });
        let v334: f64 = (self.scalar_v201 * v333);
        let v335: f64 = (v331 + v334);
        let v336: f64 = (v181 * v333);
        let v337: f64 = (v331 * v336);
        let v338: f64 = (v335 + v337);
        let v339: f64 = (v176 * v338);
        let v340: f64 = (if self.scalar_v330 { v339 } else { v300 });
        let v351: f64 = { let limexp_arg = v340; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let v352: f64 = (-v340);
        let v353: f64 = { let limexp_arg = v352; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let v354: f64 = (v351 - v353);
        let v355: f64 = (v152 * v354);
        let v356: f64 = ((v355) as f64).tanh();
        let v357: f64 = (v44 + v356);
        let v358: f64 = (if self.scalar_v330 { v357 } else { v308 });
        let v427: f64 = (v44 + v208);
        let v428: f64 = (v139 / v427);
        let v429: f64 = (self.scalar_v426 + v428);
        let v430: f64 = (if self.scalar_v425 { v429 } else { v13 });
        let v440: f64 = (v44 + v358);
        let v441: f64 = (v139 / v440);
        let v442: f64 = (self.scalar_v426 + v441);
        let v443: f64 = (if self.scalar_v439 { v442 } else { v430 });
        let v456: f64 = -1.0;
        let v521: f64 = (v9 * self.scalar_v520);
        let v522: f64 = (v144 + v521);
        let v524: f64 = (v7 * self.scalar_v523);
        let v525: f64 = (v522 + v524);
        let v526: f64 = ((v525) as f64).tanh();
        let v527: f64 = (v44 + v526);
        let v530: f64 = (v7 * self.scalar_v529);
        let v531: f64 = (self.scalar_v528 + v530);
        let v532: f64 = ((v531) as f64).tanh();
        let v533: f64 = (v44 + v532);
        let v536: f64 = (v7 * self.scalar_v535);
        let v537: f64 = (self.scalar_v534 - v536);
        let v538: f64 = ((v537) as f64).tanh();
        let v539: f64 = (v44 + v538);
        let v540: f64 = (v539 - self.scalar_v523);
        let v542: f64 = (v5 * self.scalar_v541);
        let v543: f64 = (v145 + v542);
        let v544: f64 = (v543 - v524);
        let v545: f64 = ((v544) as f64).tanh();
        let v546: f64 = (v44 + v545);
        let v557: f64 = (v137 * v527);
        let v558: f64 = (v533 * v557);
        let v559: f64 = (self.scalar_v551 + v558);
        let v560: f64 = (if self.scalar_v556 { v559 } else { self.scalar_v552 });
        let v561: f64 = (v540 * v546);
        let v563: f64 = (v561 + self.scalar_v562);
        let v564: f64 = (v138 * v563);
        let v565: f64 = (self.scalar_v553 + v564);
        let v566: f64 = (if self.scalar_v556 { v565 } else { self.scalar_v554 });
        let v570: f64 = (v533 - self.scalar_v523);
        let v571: f64 = (if self.scalar_v569 { v570 } else { v533 });
        let v572: f64 = (v144 + v524);
        let v573: f64 = ((v572) as f64).cosh();
        let v574: f64 = (if self.scalar_v569 { v573 } else { v13 });
        let v575: f64 = ((v574) as f64).ln();
        let v576: f64 = (if self.scalar_v569 { v575 } else { v13 });
        let v577: f64 = ((v525) as f64).cosh();
        let v578: f64 = (if self.scalar_v569 { v577 } else { v13 });
        let v579: f64 = ((v578) as f64).ln();
        let v580: f64 = (if self.scalar_v569 { v579 } else { v13 });
        let v581: f64 = (v572 + v576);
        let v582: f64 = (if self.scalar_v569 { v581 } else { v13 });
        let v583: f64 = (v525 + v580);
        let v584: f64 = (v583 - v582);
        let v585: f64 = (v571 * v584);
        let v586: f64 = (v585 / self.scalar_v520);
        let v587: f64 = (v9 * self.scalar_v562);
        let v588: f64 = (v586 + v587);
        let v589: f64 = (v137 * v588);
        let v590: f64 = (v9 * self.scalar_v551);
        let v591: f64 = (v589 + v590);
        let v592: f64 = (if self.scalar_v569 { v591 } else { v13 });
        let v593: f64 = (v145 - v524);
        let v594: f64 = ((v593) as f64).cosh();
        let v595: f64 = (if self.scalar_v569 { v594 } else { v574 });
        let v596: f64 = ((v595) as f64).ln();
        let v597: f64 = (if self.scalar_v569 { v596 } else { v13 });
        let v598: f64 = ((v544) as f64).cosh();
        let v599: f64 = (if self.scalar_v569 { v598 } else { v578 });
        let v600: f64 = ((v599) as f64).ln();
        let v601: f64 = (if self.scalar_v569 { v600 } else { v13 });
        let v602: f64 = (v593 + v597);
        let v603: f64 = (if self.scalar_v569 { v602 } else { v13 });
        let v604: f64 = (v544 + v601);
        let v605: f64 = (v604 - v603);
        let v606: f64 = (v540 * v605);
        let v607: f64 = (v606 / self.scalar_v541);
        let v608: f64 = (v5 * self.scalar_v562);
        let v609: f64 = (v607 + v608);
        let v610: f64 = (v138 * v609);
        let v611: f64 = (v5 * self.scalar_v553);
        let v612: f64 = (v610 + v611);
        let v613: f64 = (if self.scalar_v569 { v612 } else { v13 });
        let v2250: f64 = ((v525) as f64).sinh();
        let v2253: f64 = (self.scalar_v520 * v2250);
        let v2256: f64 = (if self.scalar_v569 { v2253 } else { v13 });
        let v2259: f64 = (v2256 / v578);
        let v2262: f64 = (if self.scalar_v569 { v2259 } else { v13 });
        let v2269: f64 = (self.scalar_v520 + v2262);
        let v2278: f64 = (v571 * v2269);
        let v2281: f64 = (v2278 / self.scalar_v520);
        let v2284: f64 = (self.scalar_v562 + v2281);
        let v2287: f64 = (v137 * v2284);
        let v2290: f64 = (self.scalar_v551 + v2287);
        let v2293: f64 = (if self.scalar_v569 { v2290 } else { v13 });
        let v614: f64 = v2293;
        let v615: f64 = (if self.scalar_v569 { v614 } else { v560 });
        let v2303: f64 = ((v544) as f64).sinh();
        let v2306: f64 = (self.scalar_v541 * v2303);
        let v2309: f64 = (if self.scalar_v569 { v2306 } else { v13 });
        let v2313: f64 = (v2309 / v599);
        let v2317: f64 = (if self.scalar_v569 { v2313 } else { v13 });
        let v2325: f64 = (self.scalar_v541 + v2317);
        let v2334: f64 = (v540 * v2325);
        let v2338: f64 = (v2334 / self.scalar_v541);
        let v2341: f64 = (self.scalar_v562 + v2338);
        let v2344: f64 = (v138 * v2341);
        let v2348: f64 = (self.scalar_v553 + v2344);
        let v2351: f64 = (if self.scalar_v569 { v2348 } else { v13 });
        let v616: f64 = v2351;
        let v617: f64 = (if self.scalar_v569 { v616 } else { v566 });
        let v622: f64 = (v9 / self.scalar_v621);
        let v623: f64 = (v622 - v44);
        let v624: f64 = (if self.scalar_v620 { v623 } else { v13 });
        let v627: f64 = (v624 * v624);
        let v628: f64 = (self.scalar_v626 + v627);
        let v630: f64 = f64::powf(v628, self.scalar_v629);
        let v633: f64 = (v627 * self.scalar_v632);
        let v634: f64 = (self.scalar_v626 + v633);
        let v635: f64 = (v630 * v634);
        let v636: f64 = (if self.scalar_v620 { v635 } else { v13 });
        let v637: f64 = (v9 + v524);
        let v638: f64 = (self.scalar_v520 * v637);
        let v639: f64 = (v144 + v638);
        let v640: f64 = ((v639) as f64).tanh();
        let v641: f64 = (v44 + v640);
        let v642: f64 = (if self.scalar_v620 { v641 } else { v527 });
        let v643: f64 = (if self.scalar_v620 { v533 } else { v571 });
        let v645: f64 = (v538 + self.scalar_v644);
        let v646: f64 = (if self.scalar_v620 { v645 } else { v540 });
        let v647: f64 = (v7 * self.scalar_v644);
        let v648: f64 = (v5 + v647);
        let v649: f64 = (self.scalar_v541 * v648);
        let v650: f64 = (v145 + v649);
        let v651: f64 = ((v650) as f64).tanh();
        let v652: f64 = (v44 + v651);
        let v653: f64 = (if self.scalar_v620 { v652 } else { v546 });
        let v655: f64 = (v636 * self.scalar_v654);
        let v656: f64 = (v642 + v655);
        let v657: f64 = (v137 * v656);
        let v658: f64 = (v643 * v657);
        let v659: f64 = (self.scalar_v551 + v658);
        let v660: f64 = (if self.scalar_v620 { v659 } else { v615 });
        let v661: f64 = (v646 * v653);
        let v662: f64 = (self.scalar_v562 + v661);
        let v663: f64 = (v138 * v662);
        let v664: f64 = (self.scalar_v553 + v663);
        let v665: f64 = (if self.scalar_v620 { v664 } else { v617 });
        let v669: f64 = (if self.scalar_v668 { v573 } else { v595 });
        let v670: f64 = ((v669) as f64).ln();
        let v671: f64 = (if self.scalar_v668 { v670 } else { v576 });
        let v672: f64 = (if self.scalar_v668 { v577 } else { v599 });
        let v673: f64 = ((v672) as f64).ln();
        let v674: f64 = (if self.scalar_v668 { v673 } else { v580 });
        let v676: f64 = (v9 + self.scalar_v621);
        let v677: f64 = (self.scalar_v654 * v676);
        let v678: f64 = (v456 + v622);
        let v679: f64 = f64::powf(v678, v222);
        let v680: f64 = (self.scalar_v626 + v679);
        let v682: f64 = f64::powf(v680, self.scalar_v681);
        let v683: f64 = (v677 * v682);
        let v684: f64 = (if self.scalar_v668 { v683 } else { v13 });
        let v690: f64 = (v572 + v671);
        let v691: f64 = (if self.scalar_v668 { v690 } else { v582 });
        let v692: f64 = (v525 + v674);
        let v693: f64 = (v692 - v691);
        let v694: f64 = (v684 + v693);
        let v695: f64 = (v694 - self.scalar_v689);
        let v696: f64 = (v532 + self.scalar_v644);
        let v697: f64 = (v695 * v696);
        let v698: f64 = (v697 / self.scalar_v520);
        let v699: f64 = (v587 + v698);
        let v700: f64 = (v137 * v699);
        let v701: f64 = (v590 + v700);
        let v702: f64 = (if self.scalar_v668 { v701 } else { v592 });
        let v703: f64 = (if self.scalar_v668 { v594 } else { v669 });
        let v704: f64 = ((v703) as f64).ln();
        let v705: f64 = (if self.scalar_v668 { v704 } else { v597 });
        let v706: f64 = (if self.scalar_v668 { v598 } else { v672 });
        let v707: f64 = ((v706) as f64).ln();
        let v708: f64 = (if self.scalar_v668 { v707 } else { v601 });
        let v709: f64 = (v593 + v705);
        let v710: f64 = (if self.scalar_v668 { v709 } else { v603 });
        let v711: f64 = (v544 + v708);
        let v712: f64 = (v711 - v710);
        let v713: f64 = (v645 * v712);
        let v714: f64 = (v713 / self.scalar_v541);
        let v715: f64 = (v608 + v714);
        let v716: f64 = (v138 * v715);
        let v717: f64 = (v611 + v716);
        let v718: f64 = (if self.scalar_v668 { v717 } else { v613 });
        let v2466: f64 = (self.scalar_v654 * v682);
        let v2454: f64 = f64::powf(v678, v44);
        let v2455: f64 = (v222 * v2454);
        let v2457: f64 = (self.scalar_v2360 * v2455);
        let v2459: f64 = f64::powf(v680, self.scalar_v2458);
        let v2460: f64 = (self.scalar_v681 * v2459);
        let v2462: f64 = (v2457 * v2460);
        let v2467: f64 = (v677 * v2462);
        let v2468: f64 = (v2466 + v2467);
        let v2470: f64 = (if self.scalar_v668 { v2468 } else { v13 });
        let v2310: f64 = (if self.scalar_v569 { v13 } else { v2256 });
        let v2444: f64 = (if self.scalar_v668 { v2253 } else { v2310 });
        let v2448: f64 = (v2444 / v672);
        let v2452: f64 = (if self.scalar_v668 { v2448 } else { v2262 });
        let v2477: f64 = (self.scalar_v520 + v2452);
        let v2481: f64 = (v2470 + v2477);
        let v2489: f64 = (v696 * v2481);
        let v2493: f64 = (v2489 / self.scalar_v520);
        let v2495: f64 = (self.scalar_v562 + v2493);
        let v2499: f64 = (v137 * v2495);
        let v2501: f64 = (self.scalar_v551 + v2499);
        let v2505: f64 = (if self.scalar_v668 { v2501 } else { v2293 });
        let v719: f64 = v2505;
        let v720: f64 = (if self.scalar_v668 { v719 } else { v660 });
        let v2443: f64 = (if self.scalar_v668 { v13 } else { v2309 });
        let v2514: f64 = (if self.scalar_v668 { v2306 } else { v2443 });
        let v2518: f64 = (v2514 / v706);
        let v2522: f64 = (if self.scalar_v668 { v2518 } else { v2317 });
        let v2530: f64 = (self.scalar_v541 + v2522);
        let v2539: f64 = (v645 * v2530);
        let v2543: f64 = (v2539 / self.scalar_v541);
        let v2546: f64 = (self.scalar_v562 + v2543);
        let v2549: f64 = (v138 * v2546);
        let v2552: f64 = (self.scalar_v553 + v2549);
        let v2555: f64 = (if self.scalar_v668 { v2552 } else { v2351 });
        let v721: f64 = v2555;
        let v722: f64 = (if self.scalar_v668 { v721 } else { v665 });
        let v748: f64 = 5.5226012e-23;
        let v749: f64 = (v35 * v748);
        let v753: f64 = (v749 * self.scalar_v752);
        let v754: f64 = (v137 * v753);
        let v757: f64 = (v754 * self.scalar_v756);
        let v758: f64 = (if self.scalar_v747 { v757 } else { v13 });
        let v764: f64 = 3.141592653589793;
        let v767: f64 = (v758 * v764);
        let v768: f64 = (if self.scalar_v747 { v767 } else { v13 });
        let v772: f64 = ctx.node_voltage(nodes[15]);
        let v773: f64 = (self.scalar_v771 * v772);
        let v775: f64 = ctx.branch_current(branches[0]);
        let v776: f64 = (self.scalar_v774 * v775);
        let v777: f64 = 0.0;
        let v778: f64 = (if self.scalar_v723 { v777 } else { v13 });
        let v779: f64 = 0.0;
        let v780: f64 = (if self.scalar_v723 { v779 } else { v13 });
        let v782: f64 = (v5 * v722);
        let v783: f64 = 0.0;
        let v784: f64 = (if self.scalar_v781 { v783 } else { v13 });
        let v785: f64 = (v9 * v720);
        let v786: f64 = 0.0;
        let v787: f64 = (if self.scalar_v781 { v786 } else { v13 });
        let v789: f64 = ctx.node_voltage(nodes[7]);
        let v790: f64 = (v789 - v4);
        let v791: f64 = (self.scalar_v788 * v790);
        let v793: f64 = (v7 * self.scalar_v792);
        let v794: f64 = ctx.node_voltage(nodes[6]);
        let v795: f64 = (v794 - v10);
        let v796: f64 = (v140 * v795);
        let v798: f64 = ctx.branch_current(branches[1]);
        let v799: f64 = (v443 * v798);
        let v800: f64 = (self.scalar_v724 * v798);
        let v801: f64 = 0.0;
        let v802: f64 = (v799 + v801);
        let v803: f64 = (if self.scalar_v725 { v802 } else { v13 });
        let v807: f64 = (v2 * v142);
        let v808: f64 = 0.0;
        let v809: f64 = (if self.scalar_v728 { v808 } else { v13 });
        let v811: f64 = ctx.node_voltage(nodes[14]);
        let v812: f64 = (v8 - v811);
        let v813: f64 = (self.scalar_v810 * v812);
        let v828: f64 = ctx.branch_current(branches[10]);
        let v829: f64 = (self.scalar_v827 * v828);
        let v834: f64 = ctx.branch_current(branches[14]);
        let v835: f64 = (self.scalar_v833 * v834);
        let v840: f64 = ctx.branch_current(branches[18]);
        let v841: f64 = (self.scalar_v839 * v840);
        let v846: f64 = ctx.node_voltage(nodes[17]);
        let v854: f64 = (-v768);
        let v855: f64 = (v846 * v854);
        let v856: f64 = 0.0;
        let v857: f64 = (if self.scalar_v747 { v856 } else { v13 });
        let v868: f64 = (v32 * self.scalar_v867);
        let v869: f64 = 0.0;
        let v870: f64 = (if self.scalar_v769 { v869 } else { v13 });
        let v875: f64 = ((v161) as f64).sinh();
        let v876: f64 = (self.scalar_v160 * v875);
        let v877: f64 = (self.scalar_v874 * v875);
        let v879: f64 = (v162 * v876);
        let v880: f64 = (v879 + v879);
        let v881: f64 = (v162 * v877);
        let v882: f64 = (v881 + v881);
        let v883: f64 = (self.scalar_v166 * v880);
        let v884: f64 = (-v883);
        let v885: f64 = (v169 * v169);
        let v886: f64 = (v884 / v885);
        let v887: f64 = (self.scalar_v166 * v882);
        let v888: f64 = (-v887);
        let v889: f64 = (v888 / v885);
        let v890: f64 = (self.scalar_v165 * v886);
        let v891: f64 = (self.scalar_v165 * v889);
        let v892: f64 = (v175 * v890);
        let v893: f64 = (v175 * v891);
        let v895: f64 = (v186 * v186);
        let v896: f64 = (v44 - v895);
        let v897: f64 = (self.scalar_v184 * v896);
        let v898: f64 = (self.scalar_v894 * v896);
        let v899: f64 = (self.scalar_v182 * v897);
        let v900: f64 = (self.scalar_v182 * v898);
        let v901: f64 = (v900 - self.scalar_v878);
        let v903: f64 = (v192 + v192);
        let v904: f64 = (-v192);
        let v905: f64 = (v191 * self.scalar_v902);
        let v906: f64 = (v904 + v905);
        let v907: f64 = (v899 - v903);
        let v908: f64 = (-v906);
        let v909: f64 = (v196 * self.scalar_v878);
        let v910: f64 = (v196 * v907);
        let v911: f64 = (v196 * v901);
        let v912: f64 = (v196 * v908);
        let v913: f64 = (-v909);
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
        let v926: f64 = (v176 * v913);
        let v927: f64 = (v198 * v892);
        let v928: f64 = (v176 * v914);
        let v929: f64 = (v927 + v928);
        let v930: f64 = (v198 * v893);
        let v931: f64 = (v176 * v915);
        let v932: f64 = (v930 + v931);
        let v933: f64 = (v176 * v916);
        let v934: f64 = (self.scalar_v201 * v918);
        let v935: f64 = (self.scalar_v201 * v920);
        let v936: f64 = (self.scalar_v201 * v922);
        let v937: f64 = (self.scalar_v201 * v924);
        let v938: f64 = (self.scalar_v201 * v925);
        let v939: f64 = (v926 + v934);
        let v940: f64 = (v929 + v935);
        let v941: f64 = (v932 + v936);
        let v942: f64 = (v933 + v937);
        let v943: f64 = (v176 + v938);
        let v944: f64 = (v181 * v913);
        let v945: f64 = (v181 * v914);
        let v946: f64 = (v181 * v915);
        let v947: f64 = (v181 * v916);
        let v948: f64 = (v204 * v918);
        let v949: f64 = (v199 * v944);
        let v950: f64 = (v948 + v949);
        let v951: f64 = (v204 * v920);
        let v952: f64 = (v199 * v945);
        let v953: f64 = (v951 + v952);
        let v954: f64 = (v204 * v922);
        let v955: f64 = (v199 * v946);
        let v956: f64 = (v954 + v955);
        let v957: f64 = (v204 * v924);
        let v958: f64 = (v199 * v947);
        let v959: f64 = (v957 + v958);
        let v960: f64 = (v204 * v925);
        let v961: f64 = (v181 * v199);
        let v962: f64 = (v960 + v961);
        let v963: f64 = (v939 + v950);
        let v964: f64 = (v940 + v953);
        let v965: f64 = (v941 + v956);
        let v966: f64 = (v942 + v959);
        let v967: f64 = (v943 + v962);
        let v968: f64 = (v207 * v207);
        let v969: f64 = (v44 - v968);
        let v970: f64 = (v963 * v969);
        let v971: f64 = (v964 * v969);
        let v972: f64 = (v965 * v969);
        let v973: f64 = (v966 * v969);
        let v974: f64 = (v967 * v969);
        let v975: f64 = { let limexp_arg = v206; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v976: f64 = (v963 * v975);
        let v977: f64 = (v964 * v975);
        let v978: f64 = (v965 * v975);
        let v979: f64 = (v966 * v975);
        let v980: f64 = (v967 * v975);
        let v981: f64 = (-v963);
        let v982: f64 = (-v964);
        let v983: f64 = (-v965);
        let v984: f64 = (-v966);
        let v985: f64 = (-v967);
        let v986: f64 = { let limexp_arg = v210; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v987: f64 = (v981 * v986);
        let v988: f64 = (v982 * v986);
        let v989: f64 = (v983 * v986);
        let v990: f64 = (v984 * v986);
        let v991: f64 = (v985 * v986);
        let v992: f64 = (v976 - v987);
        let v993: f64 = (v977 - v988);
        let v994: f64 = (v978 - v989);
        let v995: f64 = (v979 - v990);
        let v996: f64 = (v980 - v991);
        let v997: f64 = (v152 * v992);
        let v998: f64 = (v152 * v993);
        let v999: f64 = (v152 * v994);
        let v1000: f64 = (v152 * v995);
        let v1001: f64 = (v152 * v996);
        let v1002: f64 = (v214 * v214);
        let v1003: f64 = (v44 - v1002);
        let v1004: f64 = (v997 * v1003);
        let v1005: f64 = (v998 * v1003);
        let v1006: f64 = (v999 * v1003);
        let v1007: f64 = (v1000 * v1003);
        let v1008: f64 = (v1001 * v1003);
        let v1071: f64 = (v456 - v910);
        let v1072: f64 = (-v911);
        let v1073: f64 = (v44 - v912);
        let v1074: f64 = (if self.scalar_v237 { v913 } else { v13 });
        let v1075: f64 = (if self.scalar_v237 { v1071 } else { v876 });
        let v1076: f64 = (if self.scalar_v237 { v1072 } else { v877 });
        let v1077: f64 = (if self.scalar_v237 { v1073 } else { v13 });
        let v1078: f64 = (v239 * v1074);
        let v1079: f64 = (v1078 + v1078);
        let v1080: f64 = (v239 * v1075);
        let v1081: f64 = (v1080 + v1080);
        let v1082: f64 = (v239 * v1076);
        let v1083: f64 = (v1082 + v1082);
        let v1084: f64 = (v239 * v1077);
        let v1085: f64 = (v1084 + v1084);
        let v1086: f64 = (if self.scalar_v237 { v1079 } else { v913 });
        let v1087: f64 = (if self.scalar_v237 { v1081 } else { v914 });
        let v1088: f64 = (if self.scalar_v237 { v1083 } else { v915 });
        let v1089: f64 = (if self.scalar_v237 { v1085 } else { v916 });
        let v1322: f64 = (if self.scalar_v290 { v913 } else { v1074 });
        let v1323: f64 = (if self.scalar_v290 { v914 } else { v1075 });
        let v1324: f64 = (if self.scalar_v290 { v915 } else { v1076 });
        let v1325: f64 = (if self.scalar_v290 { v916 } else { v1077 });
        let v1327: f64 = (v291 * v1322);
        let v1328: f64 = (v1327 + v1327);
        let v1329: f64 = (v291 * v1323);
        let v1330: f64 = (v1329 + v1329);
        let v1331: f64 = (v291 * v1324);
        let v1332: f64 = (v1331 + v1331);
        let v1333: f64 = (v291 * v1325);
        let v1334: f64 = (v1333 + v1333);
        let v1335: f64 = (v291 * self.scalar_v1326);
        let v1336: f64 = (v1335 + v1335);
        let v1337: f64 = (if self.scalar_v290 { v1328 } else { v1086 });
        let v1338: f64 = (if self.scalar_v290 { v1330 } else { v1087 });
        let v1339: f64 = (if self.scalar_v290 { v1332 } else { v1088 });
        let v1340: f64 = (if self.scalar_v290 { v1334 } else { v1089 });
        let v1341: f64 = (if self.scalar_v290 { v1336 } else { self.scalar_v1090 });
        let v1342: f64 = (self.scalar_v201 * v1337);
        let v1343: f64 = (self.scalar_v201 * v1338);
        let v1344: f64 = (self.scalar_v201 * v1339);
        let v1345: f64 = (self.scalar_v201 * v1340);
        let v1346: f64 = (self.scalar_v201 * v1341);
        let v1347: f64 = (v1322 + v1342);
        let v1348: f64 = (v1323 + v1343);
        let v1349: f64 = (v1324 + v1344);
        let v1350: f64 = (v1325 + v1345);
        let v1351: f64 = (self.scalar_v1326 + v1346);
        let v1352: f64 = (v181 * v1337);
        let v1353: f64 = (v181 * v1338);
        let v1354: f64 = (v181 * v1339);
        let v1355: f64 = (v181 * v1340);
        let v1356: f64 = (v181 * v1341);
        let v1357: f64 = (v296 * v1322);
        let v1358: f64 = (v291 * v1352);
        let v1359: f64 = (v1357 + v1358);
        let v1360: f64 = (v296 * v1323);
        let v1361: f64 = (v291 * v1353);
        let v1362: f64 = (v1360 + v1361);
        let v1363: f64 = (v296 * v1324);
        let v1364: f64 = (v291 * v1354);
        let v1365: f64 = (v1363 + v1364);
        let v1366: f64 = (v296 * v1325);
        let v1367: f64 = (v291 * v1355);
        let v1368: f64 = (v1366 + v1367);
        let v1369: f64 = (v296 * self.scalar_v1326);
        let v1370: f64 = (v291 * v1356);
        let v1371: f64 = (v1369 + v1370);
        let v1372: f64 = (v1347 + v1359);
        let v1373: f64 = (v1348 + v1362);
        let v1374: f64 = (v1349 + v1365);
        let v1375: f64 = (v1350 + v1368);
        let v1376: f64 = (v1351 + v1371);
        let v1377: f64 = (v176 * v1372);
        let v1378: f64 = (v298 * v892);
        let v1379: f64 = (v176 * v1373);
        let v1380: f64 = (v1378 + v1379);
        let v1381: f64 = (v298 * v893);
        let v1382: f64 = (v176 * v1374);
        let v1383: f64 = (v1381 + v1382);
        let v1384: f64 = (v176 * v1375);
        let v1385: f64 = (v176 * v1376);
        let v1386: f64 = (if self.scalar_v290 { v1377 } else { v963 });
        let v1387: f64 = (if self.scalar_v290 { v1380 } else { v964 });
        let v1388: f64 = (if self.scalar_v290 { v1383 } else { v965 });
        let v1389: f64 = (if self.scalar_v290 { v1384 } else { v966 });
        let v1390: f64 = (if self.scalar_v290 { v1385 } else { v967 });
        let v1391: f64 = { let limexp_arg = v300; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v1392: f64 = (v1386 * v1391);
        let v1393: f64 = (v1387 * v1391);
        let v1394: f64 = (v1388 * v1391);
        let v1395: f64 = (v1389 * v1391);
        let v1396: f64 = (v1390 * v1391);
        let v1397: f64 = (-v1386);
        let v1398: f64 = (-v1387);
        let v1399: f64 = (-v1388);
        let v1400: f64 = (-v1389);
        let v1401: f64 = (-v1390);
        let v1402: f64 = { let limexp_arg = v302; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v1403: f64 = (v1397 * v1402);
        let v1404: f64 = (v1398 * v1402);
        let v1405: f64 = (v1399 * v1402);
        let v1406: f64 = (v1400 * v1402);
        let v1407: f64 = (v1401 * v1402);
        let v1408: f64 = (v1392 - v1403);
        let v1409: f64 = (v1393 - v1404);
        let v1410: f64 = (v1394 - v1405);
        let v1411: f64 = (v1395 - v1406);
        let v1412: f64 = (v1396 - v1407);
        let v1413: f64 = (v152 * v1408);
        let v1414: f64 = (v152 * v1409);
        let v1415: f64 = (v152 * v1410);
        let v1416: f64 = (v152 * v1411);
        let v1417: f64 = (v152 * v1412);
        let v1418: f64 = (v306 * v306);
        let v1419: f64 = (v44 - v1418);
        let v1420: f64 = (v1413 * v1419);
        let v1421: f64 = (v1414 * v1419);
        let v1422: f64 = (v1415 * v1419);
        let v1423: f64 = (v1416 * v1419);
        let v1424: f64 = (v1417 * v1419);
        let v1425: f64 = (if self.scalar_v290 { v1420 } else { v1004 });
        let v1426: f64 = (if self.scalar_v290 { v1421 } else { v1005 });
        let v1427: f64 = (if self.scalar_v290 { v1422 } else { v1006 });
        let v1428: f64 = (if self.scalar_v290 { v1423 } else { v1007 });
        let v1429: f64 = (if self.scalar_v290 { v1424 } else { v1008 });
        let v1525: f64 = (if self.scalar_v330 { v913 } else { v1322 });
        let v1526: f64 = (if self.scalar_v330 { v914 } else { v1323 });
        let v1527: f64 = (if self.scalar_v330 { v915 } else { v1324 });
        let v1528: f64 = (if self.scalar_v330 { v916 } else { v1325 });
        let v1530: f64 = (v331 * v1525);
        let v1531: f64 = (v1530 + v1530);
        let v1532: f64 = (v331 * v1526);
        let v1533: f64 = (v1532 + v1532);
        let v1534: f64 = (v331 * v1527);
        let v1535: f64 = (v1534 + v1534);
        let v1536: f64 = (v331 * v1528);
        let v1537: f64 = (v1536 + v1536);
        let v1538: f64 = (v331 * self.scalar_v1529);
        let v1539: f64 = (v1538 + v1538);
        let v1540: f64 = (if self.scalar_v330 { v1531 } else { v1337 });
        let v1541: f64 = (if self.scalar_v330 { v1533 } else { v1338 });
        let v1542: f64 = (if self.scalar_v330 { v1535 } else { v1339 });
        let v1543: f64 = (if self.scalar_v330 { v1537 } else { v1340 });
        let v1544: f64 = (if self.scalar_v330 { v1539 } else { v1341 });
        let v1545: f64 = (self.scalar_v201 * v1540);
        let v1546: f64 = (self.scalar_v201 * v1541);
        let v1547: f64 = (self.scalar_v201 * v1542);
        let v1548: f64 = (self.scalar_v201 * v1543);
        let v1549: f64 = (self.scalar_v201 * v1544);
        let v1550: f64 = (v1525 + v1545);
        let v1551: f64 = (v1526 + v1546);
        let v1552: f64 = (v1527 + v1547);
        let v1553: f64 = (v1528 + v1548);
        let v1554: f64 = (self.scalar_v1529 + v1549);
        let v1555: f64 = (v181 * v1540);
        let v1556: f64 = (v181 * v1541);
        let v1557: f64 = (v181 * v1542);
        let v1558: f64 = (v181 * v1543);
        let v1559: f64 = (v181 * v1544);
        let v1560: f64 = (v336 * v1525);
        let v1561: f64 = (v331 * v1555);
        let v1562: f64 = (v1560 + v1561);
        let v1563: f64 = (v336 * v1526);
        let v1564: f64 = (v331 * v1556);
        let v1565: f64 = (v1563 + v1564);
        let v1566: f64 = (v336 * v1527);
        let v1567: f64 = (v331 * v1557);
        let v1568: f64 = (v1566 + v1567);
        let v1569: f64 = (v336 * v1528);
        let v1570: f64 = (v331 * v1558);
        let v1571: f64 = (v1569 + v1570);
        let v1572: f64 = (v336 * self.scalar_v1529);
        let v1573: f64 = (v331 * v1559);
        let v1574: f64 = (v1572 + v1573);
        let v1575: f64 = (v1550 + v1562);
        let v1576: f64 = (v1551 + v1565);
        let v1577: f64 = (v1552 + v1568);
        let v1578: f64 = (v1553 + v1571);
        let v1579: f64 = (v1554 + v1574);
        let v1580: f64 = (v176 * v1575);
        let v1581: f64 = (v338 * v892);
        let v1582: f64 = (v176 * v1576);
        let v1583: f64 = (v1581 + v1582);
        let v1584: f64 = (v338 * v893);
        let v1585: f64 = (v176 * v1577);
        let v1586: f64 = (v1584 + v1585);
        let v1587: f64 = (v176 * v1578);
        let v1588: f64 = (v176 * v1579);
        let v1589: f64 = (if self.scalar_v330 { v1580 } else { v1386 });
        let v1590: f64 = (if self.scalar_v330 { v1583 } else { v1387 });
        let v1591: f64 = (if self.scalar_v330 { v1586 } else { v1388 });
        let v1592: f64 = (if self.scalar_v330 { v1587 } else { v1389 });
        let v1593: f64 = (if self.scalar_v330 { v1588 } else { v1390 });
        let v1663: f64 = { let limexp_arg = v340; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v1664: f64 = (v1589 * v1663);
        let v1665: f64 = (v1590 * v1663);
        let v1666: f64 = (v1591 * v1663);
        let v1667: f64 = (v1592 * v1663);
        let v1668: f64 = (v1593 * v1663);
        let v1669: f64 = (-v1589);
        let v1670: f64 = (-v1590);
        let v1671: f64 = (-v1591);
        let v1672: f64 = (-v1592);
        let v1673: f64 = (-v1593);
        let v1674: f64 = { let limexp_arg = v352; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v1675: f64 = (v1669 * v1674);
        let v1676: f64 = (v1670 * v1674);
        let v1677: f64 = (v1671 * v1674);
        let v1678: f64 = (v1672 * v1674);
        let v1679: f64 = (v1673 * v1674);
        let v1680: f64 = (v1664 - v1675);
        let v1681: f64 = (v1665 - v1676);
        let v1682: f64 = (v1666 - v1677);
        let v1683: f64 = (v1667 - v1678);
        let v1684: f64 = (v1668 - v1679);
        let v1685: f64 = (v152 * v1680);
        let v1686: f64 = (v152 * v1681);
        let v1687: f64 = (v152 * v1682);
        let v1688: f64 = (v152 * v1683);
        let v1689: f64 = (v152 * v1684);
        let v1690: f64 = (v356 * v356);
        let v1691: f64 = (v44 - v1690);
        let v1692: f64 = (v1685 * v1691);
        let v1693: f64 = (v1686 * v1691);
        let v1694: f64 = (v1687 * v1691);
        let v1695: f64 = (v1688 * v1691);
        let v1696: f64 = (v1689 * v1691);
        let v1697: f64 = (if self.scalar_v330 { v1692 } else { v1425 });
        let v1698: f64 = (if self.scalar_v330 { v1693 } else { v1426 });
        let v1699: f64 = (if self.scalar_v330 { v1694 } else { v1427 });
        let v1700: f64 = (if self.scalar_v330 { v1695 } else { v1428 });
        let v1701: f64 = (if self.scalar_v330 { v1696 } else { v1429 });
        let v2052: f64 = (v139 * v970);
        let v2053: f64 = (-v2052);
        let v2054: f64 = (v427 * v427);
        let v2055: f64 = (v2053 / v2054);
        let v2056: f64 = (v139 * v971);
        let v2057: f64 = (-v2056);
        let v2058: f64 = (v2057 / v2054);
        let v2059: f64 = (v139 * v972);
        let v2060: f64 = (-v2059);
        let v2061: f64 = (v2060 / v2054);
        let v2062: f64 = (v139 * v973);
        let v2063: f64 = (-v2062);
        let v2064: f64 = (v2063 / v2054);
        let v2065: f64 = (v139 * v974);
        let v2066: f64 = (-v2065);
        let v2067: f64 = (v2066 / v2054);
        let v2068: f64 = (if self.scalar_v425 { v2055 } else { v13 });
        let v2069: f64 = (if self.scalar_v425 { v2058 } else { v13 });
        let v2070: f64 = (if self.scalar_v425 { v2061 } else { v13 });
        let v2071: f64 = (if self.scalar_v425 { v2064 } else { v13 });
        let v2072: f64 = (if self.scalar_v425 { v2067 } else { v13 });
        let v2083: f64 = (v139 * v1697);
        let v2084: f64 = (-v2083);
        let v2085: f64 = (v440 * v440);
        let v2086: f64 = (v2084 / v2085);
        let v2087: f64 = (v139 * v1698);
        let v2088: f64 = (-v2087);
        let v2089: f64 = (v2088 / v2085);
        let v2090: f64 = (v139 * v1699);
        let v2091: f64 = (-v2090);
        let v2092: f64 = (v2091 / v2085);
        let v2093: f64 = (v139 * v1700);
        let v2094: f64 = (-v2093);
        let v2095: f64 = (v2094 / v2085);
        let v2096: f64 = (v139 * v1701);
        let v2097: f64 = (-v2096);
        let v2098: f64 = (v2097 / v2085);
        let v2099: f64 = (if self.scalar_v439 { v2086 } else { v2068 });
        let v2100: f64 = (if self.scalar_v439 { v2089 } else { v2069 });
        let v2101: f64 = (if self.scalar_v439 { v2092 } else { v2070 });
        let v2102: f64 = (if self.scalar_v439 { v2095 } else { v2071 });
        let v2103: f64 = (if self.scalar_v439 { v2098 } else { v2072 });
        let v2193: f64 = (v526 * v526);
        let v2194: f64 = (v44 - v2193);
        let v2195: f64 = (self.scalar_v523 * v2194);
        let v2196: f64 = (self.scalar_v2192 * v2194);
        let v2197: f64 = (self.scalar_v520 * v2194);
        let v2199: f64 = (v532 * v532);
        let v2200: f64 = (v44 - v2199);
        let v2201: f64 = (self.scalar_v529 * v2200);
        let v2202: f64 = (self.scalar_v2198 * v2200);
        let v2204: f64 = (v538 * v538);
        let v2205: f64 = (v44 - v2204);
        let v2206: f64 = (self.scalar_v2203 * v2205);
        let v2207: f64 = (self.scalar_v535 * v2205);
        let v2210: f64 = (v545 * v545);
        let v2211: f64 = (v44 - v2210);
        let v2212: f64 = (self.scalar_v2209 * v2211);
        let v2213: f64 = (self.scalar_v523 * v2211);
        let v2214: f64 = (self.scalar_v541 * v2211);
        let v2215: f64 = (v137 * v2195);
        let v2216: f64 = (v137 * v2196);
        let v2217: f64 = (v137 * v2197);
        let v2218: f64 = (v557 * v2201);
        let v2219: f64 = (v533 * v2215);
        let v2220: f64 = (v2218 + v2219);
        let v2221: f64 = (v557 * v2202);
        let v2222: f64 = (v533 * v2216);
        let v2223: f64 = (v2221 + v2222);
        let v2224: f64 = (v533 * v2217);
        let v2225: f64 = (if self.scalar_v556 { v2220 } else { v13 });
        let v2226: f64 = (if self.scalar_v556 { v2223 } else { v13 });
        let v2227: f64 = (if self.scalar_v556 { v2224 } else { v13 });
        let v2228: f64 = (v546 * v2206);
        let v2229: f64 = (v540 * v2212);
        let v2230: f64 = (v2228 + v2229);
        let v2231: f64 = (v546 * v2207);
        let v2232: f64 = (v540 * v2213);
        let v2233: f64 = (v2231 + v2232);
        let v2234: f64 = (v540 * v2214);
        let v2235: f64 = (v138 * v2230);
        let v2236: f64 = (v138 * v2233);
        let v2237: f64 = (v138 * v2234);
        let v2238: f64 = (if self.scalar_v556 { v2235 } else { v13 });
        let v2239: f64 = (if self.scalar_v556 { v2236 } else { v13 });
        let v2240: f64 = (if self.scalar_v556 { v2237 } else { v13 });
        let v2241: f64 = ((v572) as f64).sinh();
        let v2242: f64 = (self.scalar_v523 * v2241);
        let v2243: f64 = (self.scalar_v2191 * v2241);
        let v2244: f64 = (if self.scalar_v569 { v2242 } else { v13 });
        let v2245: f64 = (if self.scalar_v569 { v2243 } else { v13 });
        let v2246: f64 = (v2244 / v574);
        let v2247: f64 = (v2245 / v574);
        let v2248: f64 = (if self.scalar_v569 { v2246 } else { v13 });
        let v2249: f64 = (if self.scalar_v569 { v2247 } else { v13 });
        let v2251: f64 = (self.scalar_v523 * v2250);
        let v2252: f64 = (self.scalar_v2192 * v2250);
        let v2254: f64 = (if self.scalar_v569 { v2251 } else { v13 });
        let v2255: f64 = (if self.scalar_v569 { v2252 } else { v13 });
        let v2257: f64 = (v2254 / v578);
        let v2258: f64 = (v2255 / v578);
        let v2260: f64 = (if self.scalar_v569 { v2257 } else { v13 });
        let v2261: f64 = (if self.scalar_v569 { v2258 } else { v13 });
        let v2263: f64 = (self.scalar_v523 + v2248);
        let v2264: f64 = (self.scalar_v2191 + v2249);
        let v2265: f64 = (if self.scalar_v569 { v2263 } else { v13 });
        let v2266: f64 = (if self.scalar_v569 { v2264 } else { v13 });
        let v2267: f64 = (self.scalar_v523 + v2260);
        let v2268: f64 = (self.scalar_v2192 + v2261);
        let v2270: f64 = (v2267 - v2265);
        let v2271: f64 = (v2268 - v2266);
        let v2272: f64 = (v584 * v2201);
        let v2273: f64 = (v571 * v2270);
        let v2274: f64 = (v2272 + v2273);
        let v2275: f64 = (v584 * v2202);
        let v2276: f64 = (v571 * v2271);
        let v2277: f64 = (v2275 + v2276);
        let v2279: f64 = (v2274 / self.scalar_v520);
        let v2280: f64 = (v2277 / self.scalar_v520);
        let v2283: f64 = (v2280 + self.scalar_v2282);
        let v2285: f64 = (v137 * v2279);
        let v2286: f64 = (v137 * v2283);
        let v2289: f64 = (v2286 + self.scalar_v2288);
        let v2291: f64 = (if self.scalar_v569 { v2285 } else { v13 });
        let v2292: f64 = (if self.scalar_v569 { v2289 } else { v13 });
        let v2294: f64 = ((v593) as f64).sinh();
        let v2295: f64 = (self.scalar_v2191 * v2294);
        let v2296: f64 = (self.scalar_v523 * v2294);
        let v2297: f64 = (if self.scalar_v569 { v2295 } else { v2244 });
        let v2298: f64 = (if self.scalar_v569 { v2296 } else { v2245 });
        let v2299: f64 = (v2297 / v595);
        let v2300: f64 = (v2298 / v595);
        let v2301: f64 = (if self.scalar_v569 { v2299 } else { v13 });
        let v2302: f64 = (if self.scalar_v569 { v2300 } else { v13 });
        let v2304: f64 = (self.scalar_v2209 * v2303);
        let v2305: f64 = (self.scalar_v523 * v2303);
        let v2307: f64 = (if self.scalar_v569 { v2304 } else { v2254 });
        let v2308: f64 = (if self.scalar_v569 { v2305 } else { v2255 });
        let v2311: f64 = (v2307 / v599);
        let v2312: f64 = (v2308 / v599);
        let v2314: f64 = (v2310 / v599);
        let v2315: f64 = (if self.scalar_v569 { v2311 } else { v13 });
        let v2316: f64 = (if self.scalar_v569 { v2312 } else { v13 });
        let v2318: f64 = (if self.scalar_v569 { v2314 } else { v13 });
        let v2319: f64 = (self.scalar_v2191 + v2301);
        let v2320: f64 = (self.scalar_v523 + v2302);
        let v2321: f64 = (if self.scalar_v569 { v2319 } else { v13 });
        let v2322: f64 = (if self.scalar_v569 { v2320 } else { v13 });
        let v2323: f64 = (self.scalar_v2209 + v2315);
        let v2324: f64 = (self.scalar_v523 + v2316);
        let v2326: f64 = (v2323 - v2321);
        let v2327: f64 = (v2324 - v2322);
        let v2328: f64 = (v605 * v2206);
        let v2329: f64 = (v540 * v2326);
        let v2330: f64 = (v2328 + v2329);
        let v2331: f64 = (v605 * v2207);
        let v2332: f64 = (v540 * v2327);
        let v2333: f64 = (v2331 + v2332);
        let v2335: f64 = (v540 * v2318);
        let v2336: f64 = (v2330 / self.scalar_v541);
        let v2337: f64 = (v2333 / self.scalar_v541);
        let v2339: f64 = (v2335 / self.scalar_v541);
        let v2340: f64 = (self.scalar_v2282 + v2336);
        let v2342: f64 = (v138 * v2340);
        let v2343: f64 = (v138 * v2337);
        let v2345: f64 = (v138 * v2339);
        let v2347: f64 = (v2342 + self.scalar_v2346);
        let v2349: f64 = (if self.scalar_v569 { v2347 } else { v13 });
        let v2350: f64 = (if self.scalar_v569 { v2343 } else { v13 });
        let v2352: f64 = (if self.scalar_v569 { v2345 } else { v13 });
        let v2353: f64 = (if self.scalar_v569 { v13 } else { v2225 });
        let v2354: f64 = (if self.scalar_v569 { v13 } else { v2226 });
        let v2355: f64 = (if self.scalar_v569 { v13 } else { v2227 });
        let v2356: f64 = (if self.scalar_v569 { v13 } else { v2238 });
        let v2357: f64 = (if self.scalar_v569 { v13 } else { v2239 });
        let v2358: f64 = (if self.scalar_v569 { v13 } else { v2240 });
        let v2363: f64 = (v624 * self.scalar_v2361);
        let v2364: f64 = (v2363 + v2363);
        let v2365: f64 = (v624 * self.scalar_v2362);
        let v2366: f64 = (v2365 + v2365);
        let v2368: f64 = f64::powf(v628, self.scalar_v2367);
        let v2369: f64 = (self.scalar_v629 * v2368);
        let v2370: f64 = (v2364 * v2369);
        let v2371: f64 = (v2366 * v2369);
        let v2372: f64 = (self.scalar_v632 * v2364);
        let v2373: f64 = (self.scalar_v632 * v2366);
        let v2374: f64 = (v634 * v2370);
        let v2375: f64 = (v630 * v2372);
        let v2376: f64 = (v2374 + v2375);
        let v2377: f64 = (v634 * v2371);
        let v2378: f64 = (v630 * v2373);
        let v2379: f64 = (v2377 + v2378);
        let v2380: f64 = (if self.scalar_v620 { v2376 } else { v13 });
        let v2381: f64 = (if self.scalar_v620 { v2379 } else { v13 });
        let v2385: f64 = (v640 * v640);
        let v2386: f64 = (v44 - v2385);
        let v2387: f64 = (self.scalar_v2383 * v2386);
        let v2388: f64 = (self.scalar_v2384 * v2386);
        let v2389: f64 = (self.scalar_v520 * v2386);
        let v2390: f64 = (if self.scalar_v620 { v2387 } else { v2195 });
        let v2391: f64 = (if self.scalar_v620 { v2388 } else { v2196 });
        let v2392: f64 = (if self.scalar_v620 { v2389 } else { v2197 });
        let v2397: f64 = (v651 * v651);
        let v2398: f64 = (v44 - v2397);
        let v2399: f64 = (self.scalar_v2395 * v2398);
        let v2400: f64 = (self.scalar_v2396 * v2398);
        let v2401: f64 = (self.scalar_v541 * v2398);
        let v2402: f64 = (if self.scalar_v620 { v2399 } else { v2212 });
        let v2403: f64 = (if self.scalar_v620 { v2400 } else { v2213 });
        let v2404: f64 = (if self.scalar_v620 { v2401 } else { v2214 });
        let v2405: f64 = (self.scalar_v654 * v2380);
        let v2406: f64 = (self.scalar_v654 * v2381);
        let v2407: f64 = (v2391 + v2405);
        let v2408: f64 = (v2392 + v2406);
        let v2409: f64 = (v137 * v2390);
        let v2410: f64 = (v137 * v2407);
        let v2411: f64 = (v137 * v2408);
        let v2412: f64 = (v657 * v2201);
        let v2413: f64 = (v643 * v2409);
        let v2414: f64 = (v2412 + v2413);
        let v2415: f64 = (v657 * v2202);
        let v2416: f64 = (v643 * v2410);
        let v2417: f64 = (v2415 + v2416);
        let v2418: f64 = (v643 * v2411);
        let v2419: f64 = (if self.scalar_v620 { v2414 } else { v2353 });
        let v2420: f64 = (if self.scalar_v620 { v2417 } else { v2354 });
        let v2421: f64 = (if self.scalar_v620 { v2418 } else { v2355 });
        let v2422: f64 = (v653 * v2206);
        let v2423: f64 = (v646 * v2402);
        let v2424: f64 = (v2422 + v2423);
        let v2425: f64 = (v653 * v2207);
        let v2426: f64 = (v646 * v2403);
        let v2427: f64 = (v2425 + v2426);
        let v2428: f64 = (v646 * v2404);
        let v2429: f64 = (v138 * v2424);
        let v2430: f64 = (v138 * v2427);
        let v2431: f64 = (v138 * v2428);
        let v2432: f64 = (if self.scalar_v620 { v2429 } else { v2356 });
        let v2433: f64 = (if self.scalar_v620 { v2430 } else { v2357 });
        let v2434: f64 = (if self.scalar_v620 { v2431 } else { v2358 });
        let v2435: f64 = (if self.scalar_v668 { v2242 } else { v2297 });
        let v2436: f64 = (if self.scalar_v668 { v2243 } else { v2298 });
        let v2437: f64 = (v2435 / v669);
        let v2438: f64 = (v2436 / v669);
        let v2439: f64 = (if self.scalar_v668 { v2437 } else { v2248 });
        let v2440: f64 = (if self.scalar_v668 { v2438 } else { v2249 });
        let v2441: f64 = (if self.scalar_v668 { v2251 } else { v2307 });
        let v2442: f64 = (if self.scalar_v668 { v2252 } else { v2308 });
        let v2445: f64 = (v2441 / v672);
        let v2446: f64 = (v2442 / v672);
        let v2447: f64 = (v2443 / v672);
        let v2449: f64 = (if self.scalar_v668 { v2445 } else { v2260 });
        let v2450: f64 = (if self.scalar_v668 { v2446 } else { v2261 });
        let v2451: f64 = (if self.scalar_v668 { v2447 } else { v13 });
        let v2456: f64 = (self.scalar_v2359 * v2455);
        let v2461: f64 = (v2456 * v2460);
        let v2463: f64 = (v682 * self.scalar_v2453);
        let v2464: f64 = (v677 * v2461);
        let v2465: f64 = (v2463 + v2464);
        let v2469: f64 = (if self.scalar_v668 { v2465 } else { v13 });
        let v2471: f64 = (self.scalar_v523 + v2439);
        let v2472: f64 = (self.scalar_v2191 + v2440);
        let v2473: f64 = (if self.scalar_v668 { v2471 } else { v2265 });
        let v2474: f64 = (if self.scalar_v668 { v2472 } else { v2266 });
        let v2475: f64 = (self.scalar_v523 + v2449);
        let v2476: f64 = (self.scalar_v2192 + v2450);
        let v2478: f64 = (v2475 - v2473);
        let v2479: f64 = (v2476 - v2474);
        let v2480: f64 = (v2469 + v2479);
        let v2482: f64 = (v696 * v2478);
        let v2483: f64 = (v695 * v2201);
        let v2484: f64 = (v2482 + v2483);
        let v2485: f64 = (v696 * v2480);
        let v2486: f64 = (v695 * v2202);
        let v2487: f64 = (v2485 + v2486);
        let v2488: f64 = (v696 * v2451);
        let v2490: f64 = (v2484 / self.scalar_v520);
        let v2491: f64 = (v2487 / self.scalar_v520);
        let v2492: f64 = (v2488 / self.scalar_v520);
        let v2494: f64 = (self.scalar_v2282 + v2491);
        let v2496: f64 = (v137 * v2490);
        let v2497: f64 = (v137 * v2494);
        let v2498: f64 = (v137 * v2492);
        let v2500: f64 = (self.scalar_v2288 + v2497);
        let v2502: f64 = (if self.scalar_v668 { v2496 } else { v2291 });
        let v2503: f64 = (if self.scalar_v668 { v2500 } else { v2292 });
        let v2504: f64 = (if self.scalar_v668 { v2498 } else { v13 });
        let v2506: f64 = (if self.scalar_v668 { v2295 } else { v2435 });
        let v2507: f64 = (if self.scalar_v668 { v2296 } else { v2436 });
        let v2508: f64 = (v2506 / v703);
        let v2509: f64 = (v2507 / v703);
        let v2510: f64 = (if self.scalar_v668 { v2508 } else { v2301 });
        let v2511: f64 = (if self.scalar_v668 { v2509 } else { v2302 });
        let v2512: f64 = (if self.scalar_v668 { v2304 } else { v2441 });
        let v2513: f64 = (if self.scalar_v668 { v2305 } else { v2442 });
        let v2515: f64 = (if self.scalar_v668 { v13 } else { v2444 });
        let v2516: f64 = (v2512 / v706);
        let v2517: f64 = (v2513 / v706);
        let v2519: f64 = (v2515 / v706);
        let v2520: f64 = (if self.scalar_v668 { v2516 } else { v2315 });
        let v2521: f64 = (if self.scalar_v668 { v2517 } else { v2316 });
        let v2523: f64 = (if self.scalar_v668 { v2519 } else { v2318 });
        let v2524: f64 = (self.scalar_v2191 + v2510);
        let v2525: f64 = (self.scalar_v523 + v2511);
        let v2526: f64 = (if self.scalar_v668 { v2524 } else { v2321 });
        let v2527: f64 = (if self.scalar_v668 { v2525 } else { v2322 });
        let v2528: f64 = (self.scalar_v2209 + v2520);
        let v2529: f64 = (self.scalar_v523 + v2521);
        let v2531: f64 = (v2528 - v2526);
        let v2532: f64 = (v2529 - v2527);
        let v2533: f64 = (v712 * v2206);
        let v2534: f64 = (v645 * v2531);
        let v2535: f64 = (v2533 + v2534);
        let v2536: f64 = (v712 * v2207);
        let v2537: f64 = (v645 * v2532);
        let v2538: f64 = (v2536 + v2537);
        let v2540: f64 = (v645 * v2523);
        let v2541: f64 = (v2535 / self.scalar_v541);
        let v2542: f64 = (v2538 / self.scalar_v541);
        let v2544: f64 = (v2540 / self.scalar_v541);
        let v2545: f64 = (self.scalar_v2282 + v2541);
        let v2547: f64 = (v138 * v2545);
        let v2548: f64 = (v138 * v2542);
        let v2550: f64 = (v138 * v2544);
        let v2551: f64 = (self.scalar_v2346 + v2547);
        let v2553: f64 = (if self.scalar_v668 { v2551 } else { v2349 });
        let v2554: f64 = (if self.scalar_v668 { v2548 } else { v2350 });
        let v2556: f64 = (if self.scalar_v668 { v2550 } else { v2352 });
        let v2557: f64 = (if self.scalar_v668 { v13 } else { v2419 });
        let v2558: f64 = (if self.scalar_v668 { v13 } else { v2420 });
        let v2559: f64 = (if self.scalar_v668 { v13 } else { v2421 });
        let v2560: f64 = (if self.scalar_v668 { v13 } else { v2432 });
        let v2561: f64 = (if self.scalar_v668 { v13 } else { v2433 });
        let v2562: f64 = (if self.scalar_v668 { v13 } else { v2434 });
        let v2568: f64 = 1.0;
        let v2569: f64 = (v2553 * v2568);
        let v2570: f64 = (v2554 * v2568);
        let v2571: f64 = (v2555 * v2568);
        let v2572: f64 = (v2556 * v2568);
        let v2573: f64 = (if self.scalar_v723 { v2569 } else { v13 });
        let v2574: f64 = (if self.scalar_v723 { v2570 } else { v13 });
        let v2575: f64 = (if self.scalar_v723 { v2571 } else { v13 });
        let v2576: f64 = (if self.scalar_v723 { v2572 } else { v13 });
        let v2577: f64 = (v2502 * v2568);
        let v2578: f64 = (v2503 * v2568);
        let v2579: f64 = (v2504 * v2568);
        let v2580: f64 = (v2505 * v2568);
        let v2581: f64 = (if self.scalar_v723 { v2577 } else { v13 });
        let v2582: f64 = (if self.scalar_v723 { v2578 } else { v13 });
        let v2583: f64 = (if self.scalar_v723 { v2579 } else { v13 });
        let v2584: f64 = (if self.scalar_v723 { v2580 } else { v13 });
        let v2585: f64 = (-v722);
        let v2586: f64 = (v5 * v2560);
        let v2587: f64 = (v2585 + v2586);
        let v2588: f64 = (v5 * v2561);
        let v2589: f64 = (v5 * v2562);
        let v2590: f64 = (v722 + v2589);
        let v2591: f64 = (v2568 * v2587);
        let v2592: f64 = (v2568 * v2588);
        let v2593: f64 = (v2568 * v2590);
        let v2594: f64 = (if self.scalar_v781 { v2591 } else { v13 });
        let v2595: f64 = (if self.scalar_v781 { v2592 } else { v13 });
        let v2596: f64 = (if self.scalar_v781 { v2593 } else { v13 });
        let v2597: f64 = (v9 * v2557);
        let v2598: f64 = (-v720);
        let v2599: f64 = (v9 * v2558);
        let v2600: f64 = (v2598 + v2599);
        let v2601: f64 = (v9 * v2559);
        let v2602: f64 = (v720 + v2601);
        let v2603: f64 = (v2568 * v2597);
        let v2604: f64 = (v2568 * v2600);
        let v2605: f64 = (v2568 * v2602);
        let v2606: f64 = (if self.scalar_v781 { v2603 } else { v13 });
        let v2607: f64 = (if self.scalar_v781 { v2604 } else { v13 });
        let v2608: f64 = (if self.scalar_v781 { v2605 } else { v13 });
        let v2611: f64 = (-v140);
        let v2613: f64 = (v798 * v2099);
        let v2614: f64 = (v798 * v2100);
        let v2615: f64 = (v798 * v2101);
        let v2616: f64 = (v798 * v2102);
        let v2617: f64 = (v798 * v2103);
        let v2618: f64 = (self.scalar_v724 * v2568);
        let v2619: f64 = (v443 + v2618);
        let v2620: f64 = (if self.scalar_v725 { v2613 } else { v13 });
        let v2621: f64 = (if self.scalar_v725 { v2614 } else { v13 });
        let v2622: f64 = (if self.scalar_v725 { v2615 } else { v13 });
        let v2623: f64 = (if self.scalar_v725 { v2616 } else { v13 });
        let v2624: f64 = (if self.scalar_v725 { v2617 } else { v13 });
        let v2625: f64 = (if self.scalar_v725 { v2619 } else { v13 });
        let v2630: f64 = (-v142);
        let v2631: f64 = (v2568 * v2630);
        let v2632: f64 = (v142 * v2568);
        let v2633: f64 = (if self.scalar_v728 { v2631 } else { v13 });
        let v2634: f64 = (if self.scalar_v728 { v2632 } else { v13 });
        let v2674: f64 = (v854 * v2568);
        let v2675: f64 = (if self.scalar_v747 { v2674 } else { v13 });
        let v2678: f64 = (self.scalar_v867 * v2568);
        let v2679: f64 = (if self.scalar_v769 { v2678 } else { v13 });

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
