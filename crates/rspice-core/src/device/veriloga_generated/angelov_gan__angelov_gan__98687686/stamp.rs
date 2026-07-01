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
        let v2284: f64 = ((v525) as f64).sinh();
        let v2287: f64 = (self.scalar_v520 * v2284);
        let v2290: f64 = (if self.scalar_v569 { v2287 } else { v13 });
        let v2293: f64 = (v2290 / v578);
        let v2296: f64 = (if self.scalar_v569 { v2293 } else { v13 });
        let v2303: f64 = (self.scalar_v520 + v2296);
        let v2312: f64 = (v571 * v2303);
        let v2315: f64 = (v2312 / self.scalar_v520);
        let v2318: f64 = (self.scalar_v562 + v2315);
        let v2321: f64 = (v137 * v2318);
        let v2324: f64 = (self.scalar_v551 + v2321);
        let v2327: f64 = (if self.scalar_v569 { v2324 } else { v13 });
        let v614: f64 = v2327;
        let v615: f64 = (if self.scalar_v569 { v614 } else { v560 });
        let v2337: f64 = ((v544) as f64).sinh();
        let v2340: f64 = (self.scalar_v541 * v2337);
        let v2343: f64 = (if self.scalar_v569 { v2340 } else { v13 });
        let v2347: f64 = (v2343 / v599);
        let v2351: f64 = (if self.scalar_v569 { v2347 } else { v13 });
        let v2359: f64 = (self.scalar_v541 + v2351);
        let v2368: f64 = (v540 * v2359);
        let v2372: f64 = (v2368 / self.scalar_v541);
        let v2375: f64 = (self.scalar_v562 + v2372);
        let v2378: f64 = (v138 * v2375);
        let v2382: f64 = (self.scalar_v553 + v2378);
        let v2385: f64 = (if self.scalar_v569 { v2382 } else { v13 });
        let v616: f64 = v2385;
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
        let v2504: f64 = (self.scalar_v654 * v682);
        let v2492: f64 = f64::powf(v678, v44);
        let v2493: f64 = (v222 * v2492);
        let v2495: f64 = (self.scalar_v2394 * v2493);
        let v2497: f64 = f64::powf(v680, self.scalar_v2496);
        let v2498: f64 = (self.scalar_v681 * v2497);
        let v2500: f64 = (v2495 * v2498);
        let v2505: f64 = (v677 * v2500);
        let v2506: f64 = (v2504 + v2505);
        let v2508: f64 = (if self.scalar_v668 { v2506 } else { v13 });
        let v2344: f64 = (if self.scalar_v569 { v13 } else { v2290 });
        let v2482: f64 = (if self.scalar_v668 { v2287 } else { v2344 });
        let v2486: f64 = (v2482 / v672);
        let v2490: f64 = (if self.scalar_v668 { v2486 } else { v2296 });
        let v2515: f64 = (self.scalar_v520 + v2490);
        let v2519: f64 = (v2508 + v2515);
        let v2527: f64 = (v696 * v2519);
        let v2531: f64 = (v2527 / self.scalar_v520);
        let v2533: f64 = (self.scalar_v562 + v2531);
        let v2537: f64 = (v137 * v2533);
        let v2539: f64 = (self.scalar_v551 + v2537);
        let v2543: f64 = (if self.scalar_v668 { v2539 } else { v2327 });
        let v719: f64 = v2543;
        let v720: f64 = (if self.scalar_v668 { v719 } else { v660 });
        let v2481: f64 = (if self.scalar_v668 { v13 } else { v2343 });
        let v2552: f64 = (if self.scalar_v668 { v2340 } else { v2481 });
        let v2556: f64 = (v2552 / v706);
        let v2560: f64 = (if self.scalar_v668 { v2556 } else { v2351 });
        let v2568: f64 = (self.scalar_v541 + v2560);
        let v2577: f64 = (v645 * v2568);
        let v2581: f64 = (v2577 / self.scalar_v541);
        let v2584: f64 = (self.scalar_v562 + v2581);
        let v2587: f64 = (v138 * v2584);
        let v2590: f64 = (self.scalar_v553 + v2587);
        let v2593: f64 = (if self.scalar_v668 { v2590 } else { v2385 });
        let v721: f64 = v2593;
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
        let v772: f64 = (-v424);
        let v774: f64 = ctx.node_voltage(nodes[15]);
        let v775: f64 = (self.scalar_v773 * v774);
        let v777: f64 = ctx.branch_current(branches[0]);
        let v778: f64 = (self.scalar_v776 * v777);
        let v779: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, v718);
        let v780: f64 = (if self.scalar_v723 { v779 } else { v13 });
        let v781: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, v702);
        let v782: f64 = (if self.scalar_v723 { v781 } else { v13 });
        let v784: f64 = (v5 * v722);
        let v785: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 5, v784);
        let v786: f64 = (if self.scalar_v783 { v785 } else { v13 });
        let v787: f64 = (v9 * v720);
        let v788: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 6, v787);
        let v789: f64 = (if self.scalar_v783 { v788 } else { v13 });
        let v791: f64 = ctx.node_voltage(nodes[7]);
        let v792: f64 = (v791 - v4);
        let v793: f64 = (self.scalar_v790 * v792);
        let v795: f64 = (v7 * self.scalar_v794);
        let v796: f64 = ctx.node_voltage(nodes[6]);
        let v797: f64 = (v796 - v10);
        let v798: f64 = (v140 * v797);
        let v799: f64 = (v167 * v797);
        let v800: f64 = ctx.branch_current(branches[1]);
        let v801: f64 = (v443 * v800);
        let v802: f64 = (self.scalar_v724 * v800);
        let v803: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, v802);
        let v804: f64 = (v801 + v803);
        let v805: f64 = (if self.scalar_v725 { v804 } else { v13 });
        let v808: f64 = (v8 - v0);
        let v809: f64 = (v808 / v141);
        let v810: f64 = (if self.scalar_v728 { v809 } else { v13 });
        let v811: f64 = (v2 * v142);
        let v812: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 10, v811);
        let v813: f64 = (if self.scalar_v728 { v812 } else { v13 });
        let v817: f64 = ctx.node_voltage(nodes[14]);
        let v818: f64 = (v8 - v817);
        let v819: f64 = (self.scalar_v816 * v818);
        let v820: f64 = (v817 - v1);
        let v821: f64 = (v820 / self.scalar_v729);
        let v822: f64 = (if self.scalar_v730 { v821 } else { v13 });
        let v825: f64 = ctx.node_voltage(nodes[13]);
        let v826: f64 = (v825 - v3);
        let v827: f64 = (v826 / self.scalar_v731);
        let v828: f64 = (if self.scalar_v732 { v827 } else { v13 });
        let v832: f64 = (v825 - v8);
        let v833: f64 = (v832 / self.scalar_v733);
        let v834: f64 = (if self.scalar_v734 { v833 } else { v13 });
        let v837: f64 = ctx.branch_current(branches[7]);
        let v838: f64 = (self.scalar_v735 * v837);
        let v839: f64 = (if self.scalar_v736 { v838 } else { v13 });
        let v845: f64 = ctx.branch_current(branches[10]);
        let v846: f64 = (self.scalar_v844 * v845);
        let v847: f64 = ctx.branch_current(branches[11]);
        let v848: f64 = (v452 * v847);
        let v849: f64 = (if self.scalar_v737 { v848 } else { v13 });
        let v855: f64 = ctx.branch_current(branches[14]);
        let v856: f64 = (self.scalar_v854 * v855);
        let v857: f64 = ctx.branch_current(branches[15]);
        let v858: f64 = (v453 * v857);
        let v859: f64 = (if self.scalar_v740 { v858 } else { v13 });
        let v865: f64 = ctx.branch_current(branches[18]);
        let v866: f64 = (self.scalar_v864 * v865);
        let v867: f64 = 1e-15;
        let v868: f64 = ctx.node_voltage(nodes[2]);
        let v869: f64 = (v0 - v868);
        let v870: f64 = (v167 * v869);
        let v874: f64 = ctx.node_voltage(nodes[17]);
        let v875: f64 = (if self.scalar_v747 { v874 } else { v13 });
        let v876: f64 = ctx.node_voltage(nodes[18]);
        let v877: f64 = (if self.scalar_v747 { v876 } else { v13 });
        let v878: f64 = (v766 * v874);
        let v879: f64 = (v762 * v876);
        let v880: f64 = (v878 + v879);
        let v881: f64 = (if self.scalar_v747 { v880 } else { v13 });
        let v882: f64 = (-v768);
        let v883: f64 = (v874 * v882);
        let v884: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 15, v883);
        let v885: f64 = (if self.scalar_v747 { v884 } else { v13 });
        let v890: f64 = (v7 * v424);
        let v891: f64 = ((v890) as f64).abs();
        let v892: f64 = (v9 * v510);
        let v893: f64 = ((v892) as f64).abs();
        let v894: f64 = (v891 + v893);
        let v895: f64 = (-v894);
        let v896: f64 = (if self.scalar_v771 { v895 } else { v13 });
        let v897: f64 = (v32 / v49);
        let v898: f64 = (if self.scalar_v771 { v897 } else { v13 });
        let v900: f64 = (v32 * self.scalar_v899);
        let v901: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 16, v900);
        let v902: f64 = (if self.scalar_v771 { v901 } else { v13 });
        let v904: f64 = (v32 * v167);
        let v905: f64 = (if self.scalar_v903 { v904 } else { v13 });
        let v907: f64 = ((v161) as f64).sinh();
        let v908: f64 = (self.scalar_v160 * v907);
        let v909: f64 = (self.scalar_v906 * v907);
        let v911: f64 = (v162 * v908);
        let v912: f64 = (v911 + v911);
        let v913: f64 = (v162 * v909);
        let v914: f64 = (v913 + v913);
        let v915: f64 = (self.scalar_v166 * v912);
        let v916: f64 = (-v915);
        let v917: f64 = (v169 * v169);
        let v918: f64 = (v916 / v917);
        let v919: f64 = (self.scalar_v166 * v914);
        let v920: f64 = (-v919);
        let v921: f64 = (v920 / v917);
        let v922: f64 = (self.scalar_v165 * v918);
        let v923: f64 = (self.scalar_v165 * v921);
        let v924: f64 = (v175 * v922);
        let v925: f64 = (v175 * v923);
        let v927: f64 = (v186 * v186);
        let v928: f64 = (v44 - v927);
        let v929: f64 = (self.scalar_v184 * v928);
        let v930: f64 = (self.scalar_v926 * v928);
        let v931: f64 = (self.scalar_v182 * v929);
        let v932: f64 = (self.scalar_v182 * v930);
        let v933: f64 = (v932 - self.scalar_v910);
        let v935: f64 = (v192 + v192);
        let v936: f64 = (-v192);
        let v937: f64 = (v191 * self.scalar_v934);
        let v938: f64 = (v936 + v937);
        let v939: f64 = (v931 - v935);
        let v940: f64 = (-v938);
        let v941: f64 = (v196 * self.scalar_v910);
        let v942: f64 = (v196 * v939);
        let v943: f64 = (v196 * v933);
        let v944: f64 = (v196 * v940);
        let v945: f64 = (-v941);
        let v946: f64 = (-v942);
        let v947: f64 = (v456 - v943);
        let v948: f64 = (-v944);
        let v949: f64 = (v198 * v945);
        let v950: f64 = (v949 + v949);
        let v951: f64 = (v198 * v946);
        let v952: f64 = (v951 + v951);
        let v953: f64 = (v198 * v947);
        let v954: f64 = (v953 + v953);
        let v955: f64 = (v198 * v948);
        let v956: f64 = (v955 + v955);
        let v957: f64 = (v198 + v198);
        let v958: f64 = (v176 * v945);
        let v959: f64 = (v198 * v924);
        let v960: f64 = (v176 * v946);
        let v961: f64 = (v959 + v960);
        let v962: f64 = (v198 * v925);
        let v963: f64 = (v176 * v947);
        let v964: f64 = (v962 + v963);
        let v965: f64 = (v176 * v948);
        let v966: f64 = (self.scalar_v201 * v950);
        let v967: f64 = (self.scalar_v201 * v952);
        let v968: f64 = (self.scalar_v201 * v954);
        let v969: f64 = (self.scalar_v201 * v956);
        let v970: f64 = (self.scalar_v201 * v957);
        let v971: f64 = (v958 + v966);
        let v972: f64 = (v961 + v967);
        let v973: f64 = (v964 + v968);
        let v974: f64 = (v965 + v969);
        let v975: f64 = (v176 + v970);
        let v976: f64 = (v181 * v945);
        let v977: f64 = (v181 * v946);
        let v978: f64 = (v181 * v947);
        let v979: f64 = (v181 * v948);
        let v980: f64 = (v204 * v950);
        let v981: f64 = (v199 * v976);
        let v982: f64 = (v980 + v981);
        let v983: f64 = (v204 * v952);
        let v984: f64 = (v199 * v977);
        let v985: f64 = (v983 + v984);
        let v986: f64 = (v204 * v954);
        let v987: f64 = (v199 * v978);
        let v988: f64 = (v986 + v987);
        let v989: f64 = (v204 * v956);
        let v990: f64 = (v199 * v979);
        let v991: f64 = (v989 + v990);
        let v992: f64 = (v204 * v957);
        let v993: f64 = (v181 * v199);
        let v994: f64 = (v992 + v993);
        let v995: f64 = (v971 + v982);
        let v996: f64 = (v972 + v985);
        let v997: f64 = (v973 + v988);
        let v998: f64 = (v974 + v991);
        let v999: f64 = (v975 + v994);
        let v1000: f64 = (v207 * v207);
        let v1001: f64 = (v44 - v1000);
        let v1002: f64 = (v995 * v1001);
        let v1003: f64 = (v996 * v1001);
        let v1004: f64 = (v997 * v1001);
        let v1005: f64 = (v998 * v1001);
        let v1006: f64 = (v999 * v1001);
        let v1007: f64 = { let limexp_arg = v206; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v1008: f64 = (v995 * v1007);
        let v1009: f64 = (v996 * v1007);
        let v1010: f64 = (v997 * v1007);
        let v1011: f64 = (v998 * v1007);
        let v1012: f64 = (v999 * v1007);
        let v1013: f64 = (-v995);
        let v1014: f64 = (-v996);
        let v1015: f64 = (-v997);
        let v1016: f64 = (-v998);
        let v1017: f64 = (-v999);
        let v1018: f64 = { let limexp_arg = v210; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v1019: f64 = (v1013 * v1018);
        let v1020: f64 = (v1014 * v1018);
        let v1021: f64 = (v1015 * v1018);
        let v1022: f64 = (v1016 * v1018);
        let v1023: f64 = (v1017 * v1018);
        let v1024: f64 = (v1008 - v1019);
        let v1025: f64 = (v1009 - v1020);
        let v1026: f64 = (v1010 - v1021);
        let v1027: f64 = (v1011 - v1022);
        let v1028: f64 = (v1012 - v1023);
        let v1029: f64 = (v152 * v1024);
        let v1030: f64 = (v152 * v1025);
        let v1031: f64 = (v152 * v1026);
        let v1032: f64 = (v152 * v1027);
        let v1033: f64 = (v152 * v1028);
        let v1034: f64 = (v214 * v214);
        let v1035: f64 = (v44 - v1034);
        let v1036: f64 = (v1029 * v1035);
        let v1037: f64 = (v1030 * v1035);
        let v1038: f64 = (v1031 * v1035);
        let v1039: f64 = (v1032 * v1035);
        let v1040: f64 = (v1033 * v1035);
        let v1041: f64 = (self.scalar_v184 * v1002);
        let v1042: f64 = (self.scalar_v184 * v1003);
        let v1043: f64 = (self.scalar_v184 * v1004);
        let v1044: f64 = (self.scalar_v184 * v1005);
        let v1045: f64 = (self.scalar_v184 * v1006);
        let v1046: f64 = (v7 * v1041);
        let v1047: f64 = (v7 * v1042);
        let v1048: f64 = (v218 + v1047);
        let v1049: f64 = (-v218);
        let v1050: f64 = (v7 * v1043);
        let v1051: f64 = (v1049 + v1050);
        let v1052: f64 = (v7 * v1044);
        let v1053: f64 = (v7 * v1045);
        let v1054: f64 = (v220 * v220);
        let v1055: f64 = (v44 - v1054);
        let v1056: f64 = (v1046 * v1055);
        let v1057: f64 = (v1048 * v1055);
        let v1058: f64 = (v1051 * v1055);
        let v1059: f64 = (v1052 * v1055);
        let v1060: f64 = (v1053 * v1055);
        let v1061: f64 = (v135 * v1002);
        let v1062: f64 = (v135 * v1003);
        let v1063: f64 = (v135 * v1004);
        let v1064: f64 = (v135 * v1005);
        let v1065: f64 = (v135 * v1006);
        let v1066: f64 = (v226 * v1056);
        let v1067: f64 = (v220 * v1061);
        let v1068: f64 = (v1066 + v1067);
        let v1069: f64 = (v226 * v1057);
        let v1070: f64 = (v220 * v1062);
        let v1071: f64 = (v1069 + v1070);
        let v1072: f64 = (v226 * v1058);
        let v1073: f64 = (v220 * v1063);
        let v1074: f64 = (v1072 + v1073);
        let v1075: f64 = (v226 * v1059);
        let v1076: f64 = (v220 * v1064);
        let v1077: f64 = (v1075 + v1076);
        let v1078: f64 = (v226 * v1060);
        let v1079: f64 = (v220 * v1065);
        let v1080: f64 = (v1078 + v1079);
        let v1082: f64 = { let limexp_arg = v191; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v1083: f64 = (-v1082);
        let v1084: f64 = (v136 * v1082);
        let v1085: f64 = (v136 * v1083);
        let v1086: f64 = (self.scalar_v228 + v1084);
        let v1087: f64 = (v233 * v1068);
        let v1088: f64 = (v233 * v1071);
        let v1089: f64 = (v227 * v1086);
        let v1090: f64 = (v1088 + v1089);
        let v1091: f64 = (v233 * v1074);
        let v1092: f64 = (v227 * self.scalar_v1081);
        let v1093: f64 = (v1091 + v1092);
        let v1094: f64 = (v233 * v1077);
        let v1095: f64 = (v227 * v1085);
        let v1096: f64 = (v1094 + v1095);
        let v1097: f64 = (v233 * v1080);
        let v1098: f64 = (if self.scalar_v221 { v1087 } else { v13 });
        let v1099: f64 = (if self.scalar_v221 { v1090 } else { v13 });
        let v1100: f64 = (if self.scalar_v221 { v1093 } else { v13 });
        let v1101: f64 = (if self.scalar_v221 { v1096 } else { v13 });
        let v1102: f64 = (if self.scalar_v221 { v1097 } else { v13 });
        let v1103: f64 = (v456 - v942);
        let v1104: f64 = (-v943);
        let v1105: f64 = (v44 - v944);
        let v1106: f64 = (if self.scalar_v237 { v945 } else { v13 });
        let v1107: f64 = (if self.scalar_v237 { v1103 } else { v908 });
        let v1108: f64 = (if self.scalar_v237 { v1104 } else { v909 });
        let v1109: f64 = (if self.scalar_v237 { v1105 } else { v13 });
        let v1110: f64 = (v239 * v1106);
        let v1111: f64 = (v1110 + v1110);
        let v1112: f64 = (v239 * v1107);
        let v1113: f64 = (v1112 + v1112);
        let v1114: f64 = (v239 * v1108);
        let v1115: f64 = (v1114 + v1114);
        let v1116: f64 = (v239 * v1109);
        let v1117: f64 = (v1116 + v1116);
        let v1118: f64 = (if self.scalar_v237 { v1111 } else { v945 });
        let v1119: f64 = (if self.scalar_v237 { v1113 } else { v946 });
        let v1120: f64 = (if self.scalar_v237 { v1115 } else { v947 });
        let v1121: f64 = (if self.scalar_v237 { v1117 } else { v948 });
        let v1123: f64 = (v241 * v1106);
        let v1124: f64 = (v239 * v1118);
        let v1125: f64 = (v1123 + v1124);
        let v1126: f64 = (v241 * v1107);
        let v1127: f64 = (v239 * v1119);
        let v1128: f64 = (v1126 + v1127);
        let v1129: f64 = (v241 * v1108);
        let v1130: f64 = (v239 * v1120);
        let v1131: f64 = (v1129 + v1130);
        let v1132: f64 = (v241 * v1109);
        let v1133: f64 = (v239 * v1121);
        let v1134: f64 = (v1132 + v1133);
        let v1135: f64 = (v239 * self.scalar_v1122);
        let v1136: f64 = (if self.scalar_v237 { v1125 } else { v950 });
        let v1137: f64 = (if self.scalar_v237 { v1128 } else { v952 });
        let v1138: f64 = (if self.scalar_v237 { v1131 } else { v954 });
        let v1139: f64 = (if self.scalar_v237 { v1134 } else { v956 });
        let v1140: f64 = (if self.scalar_v237 { v1135 } else { v957 });
        let v1141: f64 = (v176 * v1106);
        let v1142: f64 = (v239 * v924);
        let v1143: f64 = (v176 * v1107);
        let v1144: f64 = (v1142 + v1143);
        let v1145: f64 = (v239 * v925);
        let v1146: f64 = (v176 * v1108);
        let v1147: f64 = (v1145 + v1146);
        let v1148: f64 = (v176 * v1109);
        let v1149: f64 = (self.scalar_v201 * v1118);
        let v1150: f64 = (self.scalar_v201 * v1119);
        let v1151: f64 = (self.scalar_v201 * v1120);
        let v1152: f64 = (self.scalar_v201 * v1121);
        let v1154: f64 = (v1141 + v1149);
        let v1155: f64 = (v1144 + v1150);
        let v1156: f64 = (v1147 + v1151);
        let v1157: f64 = (v1148 + v1152);
        let v1158: f64 = (v181 * v1136);
        let v1159: f64 = (v181 * v1137);
        let v1160: f64 = (v181 * v1138);
        let v1161: f64 = (v181 * v1139);
        let v1162: f64 = (v181 * v1140);
        let v1163: f64 = (v1154 + v1158);
        let v1164: f64 = (v1155 + v1159);
        let v1165: f64 = (v1156 + v1160);
        let v1166: f64 = (v1157 + v1161);
        let v1167: f64 = (self.scalar_v1153 + v1162);
        let v1168: f64 = (if self.scalar_v237 { v1163 } else { v13 });
        let v1169: f64 = (if self.scalar_v237 { v1164 } else { v13 });
        let v1170: f64 = (if self.scalar_v237 { v1165 } else { v13 });
        let v1171: f64 = (if self.scalar_v237 { v1166 } else { v13 });
        let v1172: f64 = (if self.scalar_v237 { v1167 } else { v13 });
        let v1173: f64 = (v250 * v250);
        let v1174: f64 = (v44 - v1173);
        let v1175: f64 = (v1168 * v1174);
        let v1176: f64 = (v1169 * v1174);
        let v1177: f64 = (v1170 * v1174);
        let v1178: f64 = (v1171 * v1174);
        let v1179: f64 = (v1172 * v1174);
        let v1180: f64 = (if self.scalar_v237 { v1175 } else { v13 });
        let v1181: f64 = (if self.scalar_v237 { v1176 } else { v13 });
        let v1182: f64 = (if self.scalar_v237 { v1177 } else { v13 });
        let v1183: f64 = (if self.scalar_v237 { v1178 } else { v13 });
        let v1184: f64 = (if self.scalar_v237 { v1179 } else { v13 });
        let v1185: f64 = (self.scalar_v184 * v1180);
        let v1186: f64 = (self.scalar_v184 * v1181);
        let v1187: f64 = (self.scalar_v184 * v1182);
        let v1188: f64 = (self.scalar_v184 * v1183);
        let v1189: f64 = (self.scalar_v184 * v1184);
        let v1190: f64 = (if self.scalar_v237 { v1185 } else { v13 });
        let v1191: f64 = (if self.scalar_v237 { v1186 } else { v13 });
        let v1192: f64 = (if self.scalar_v237 { v1187 } else { v13 });
        let v1193: f64 = (if self.scalar_v237 { v1188 } else { v13 });
        let v1194: f64 = (if self.scalar_v237 { v1189 } else { v13 });
        let v1195: f64 = (self.scalar_v256 * v1002);
        let v1196: f64 = (self.scalar_v256 * v1003);
        let v1197: f64 = (self.scalar_v256 * v1004);
        let v1198: f64 = (self.scalar_v256 * v1005);
        let v1199: f64 = (self.scalar_v256 * v1006);
        let v1200: f64 = (if self.scalar_v237 { v1195 } else { v13 });
        let v1201: f64 = (if self.scalar_v237 { v1196 } else { v13 });
        let v1202: f64 = (if self.scalar_v237 { v1197 } else { v13 });
        let v1203: f64 = (if self.scalar_v237 { v1198 } else { v13 });
        let v1204: f64 = (if self.scalar_v237 { v1199 } else { v13 });
        let v1205: f64 = (v260 * v1061);
        let v1206: f64 = (v1066 + v1205);
        let v1207: f64 = (v260 * v1062);
        let v1208: f64 = (v1069 + v1207);
        let v1209: f64 = (v260 * v1063);
        let v1210: f64 = (v1072 + v1209);
        let v1211: f64 = (v260 * v1064);
        let v1212: f64 = (v1075 + v1211);
        let v1213: f64 = (v260 * v1065);
        let v1214: f64 = (v1078 + v1213);
        let v1215: f64 = (v7 * v1200);
        let v1216: f64 = (v7 * v1201);
        let v1217: f64 = (v259 + v1216);
        let v1218: f64 = (-v259);
        let v1219: f64 = (v7 * v1202);
        let v1220: f64 = (v1218 + v1219);
        let v1221: f64 = (v7 * v1203);
        let v1222: f64 = (v7 * v1204);
        let v1224: f64 = { let limexp_arg = v266; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v1225: f64 = (self.scalar_v264 * v1224);
        let v1226: f64 = (self.scalar_v1223 * v1224);
        let v1227: f64 = (v136 * v1225);
        let v1228: f64 = (v136 * v1226);
        let v1229: f64 = (v1217 + v1227);
        let v1230: f64 = (v1220 + v1228);
        let v1231: f64 = (v269 * v1206);
        let v1232: f64 = (v261 * v1215);
        let v1233: f64 = (v1231 + v1232);
        let v1234: f64 = (v269 * v1208);
        let v1235: f64 = (v261 * v1229);
        let v1236: f64 = (v1234 + v1235);
        let v1237: f64 = (v269 * v1210);
        let v1238: f64 = (v261 * v1230);
        let v1239: f64 = (v1237 + v1238);
        let v1240: f64 = (v269 * v1212);
        let v1241: f64 = (v261 * v1221);
        let v1242: f64 = (v1240 + v1241);
        let v1243: f64 = (v269 * v1214);
        let v1244: f64 = (v261 * v1222);
        let v1245: f64 = (v1243 + v1244);
        let v1246: f64 = (if self.scalar_v237 { v1233 } else { v13 });
        let v1247: f64 = (if self.scalar_v237 { v1236 } else { v13 });
        let v1248: f64 = (if self.scalar_v237 { v1239 } else { v13 });
        let v1249: f64 = (if self.scalar_v237 { v1242 } else { v13 });
        let v1250: f64 = (if self.scalar_v237 { v1245 } else { v13 });
        let v1251: f64 = (self.scalar_v256 * v1180);
        let v1252: f64 = (self.scalar_v256 * v1181);
        let v1253: f64 = (self.scalar_v256 * v1182);
        let v1254: f64 = (self.scalar_v256 * v1183);
        let v1255: f64 = (self.scalar_v256 * v1184);
        let v1256: f64 = (if self.scalar_v237 { v1251 } else { v13 });
        let v1257: f64 = (if self.scalar_v237 { v1252 } else { v13 });
        let v1258: f64 = (if self.scalar_v237 { v1253 } else { v13 });
        let v1259: f64 = (if self.scalar_v237 { v1254 } else { v13 });
        let v1260: f64 = (if self.scalar_v237 { v1255 } else { v13 });
        let v1261: f64 = (v7 * v1190);
        let v1262: f64 = (v7 * v1191);
        let v1263: f64 = (v255 + v1262);
        let v1264: f64 = (-v255);
        let v1265: f64 = (v7 * v1192);
        let v1266: f64 = (v1264 + v1265);
        let v1267: f64 = (v7 * v1193);
        let v1268: f64 = (v7 * v1194);
        let v1269: f64 = (v276 * v276);
        let v1270: f64 = (v44 - v1269);
        let v1271: f64 = (v1261 * v1270);
        let v1272: f64 = (v1263 * v1270);
        let v1273: f64 = (v1266 * v1270);
        let v1274: f64 = (v1267 * v1270);
        let v1275: f64 = (v1268 * v1270);
        let v1276: f64 = (if self.scalar_v237 { v1271 } else { v13 });
        let v1277: f64 = (if self.scalar_v237 { v1272 } else { v13 });
        let v1278: f64 = (if self.scalar_v237 { v1273 } else { v13 });
        let v1279: f64 = (if self.scalar_v237 { v1274 } else { v13 });
        let v1280: f64 = (if self.scalar_v237 { v1275 } else { v13 });
        let v1281: f64 = (v135 * v1180);
        let v1282: f64 = (v135 * v1181);
        let v1283: f64 = (v135 * v1182);
        let v1284: f64 = (v135 * v1183);
        let v1285: f64 = (v135 * v1184);
        let v1286: f64 = (-v1276);
        let v1287: f64 = (-v1277);
        let v1288: f64 = (-v1278);
        let v1289: f64 = (-v1279);
        let v1290: f64 = (-v1280);
        let v1291: f64 = (v279 * v1281);
        let v1292: f64 = (v278 * v1286);
        let v1293: f64 = (v1291 + v1292);
        let v1294: f64 = (v279 * v1282);
        let v1295: f64 = (v278 * v1287);
        let v1296: f64 = (v1294 + v1295);
        let v1297: f64 = (v279 * v1283);
        let v1298: f64 = (v278 * v1288);
        let v1299: f64 = (v1297 + v1298);
        let v1300: f64 = (v279 * v1284);
        let v1301: f64 = (v278 * v1289);
        let v1302: f64 = (v1300 + v1301);
        let v1303: f64 = (v279 * v1285);
        let v1304: f64 = (v278 * v1290);
        let v1305: f64 = (v1303 + v1304);
        let v1306: f64 = (v7 * v1256);
        let v1307: f64 = (v7 * v1257);
        let v1308: f64 = (v274 + v1307);
        let v1309: f64 = (-v274);
        let v1310: f64 = (v7 * v1258);
        let v1311: f64 = (v1309 + v1310);
        let v1312: f64 = (v7 * v1259);
        let v1313: f64 = (v7 * v1260);
        let v1314: f64 = (-v1306);
        let v1315: f64 = (-v1308);
        let v1316: f64 = (-v1311);
        let v1317: f64 = (-v1312);
        let v1318: f64 = (-v1313);
        let v1319: f64 = (v282 * v1293);
        let v1320: f64 = (v280 * v1314);
        let v1321: f64 = (v1319 + v1320);
        let v1322: f64 = (v282 * v1296);
        let v1323: f64 = (v280 * v1315);
        let v1324: f64 = (v1322 + v1323);
        let v1325: f64 = (v282 * v1299);
        let v1326: f64 = (v280 * v1316);
        let v1327: f64 = (v1325 + v1326);
        let v1328: f64 = (v282 * v1302);
        let v1329: f64 = (v280 * v1317);
        let v1330: f64 = (v1328 + v1329);
        let v1331: f64 = (v282 * v1305);
        let v1332: f64 = (v280 * v1318);
        let v1333: f64 = (v1331 + v1332);
        let v1334: f64 = (if self.scalar_v237 { v1321 } else { v13 });
        let v1335: f64 = (if self.scalar_v237 { v1324 } else { v13 });
        let v1336: f64 = (if self.scalar_v237 { v1327 } else { v13 });
        let v1337: f64 = (if self.scalar_v237 { v1330 } else { v13 });
        let v1338: f64 = (if self.scalar_v237 { v1333 } else { v13 });
        let v1339: f64 = (v1246 - v1334);
        let v1340: f64 = (v1247 - v1335);
        let v1341: f64 = (v1248 - v1336);
        let v1342: f64 = (v1249 - v1337);
        let v1343: f64 = (v1250 - v1338);
        let v1344: f64 = (v152 * v1339);
        let v1345: f64 = (v152 * v1340);
        let v1346: f64 = (v152 * v1341);
        let v1347: f64 = (v152 * v1342);
        let v1348: f64 = (v152 * v1343);
        let v1349: f64 = (if self.scalar_v237 { v1344 } else { v1098 });
        let v1350: f64 = (if self.scalar_v237 { v1345 } else { v1099 });
        let v1351: f64 = (if self.scalar_v237 { v1346 } else { v1100 });
        let v1352: f64 = (if self.scalar_v237 { v1347 } else { v1101 });
        let v1353: f64 = (if self.scalar_v237 { v1348 } else { v1102 });
        let v1354: f64 = (if self.scalar_v290 { v945 } else { v1106 });
        let v1355: f64 = (if self.scalar_v290 { v946 } else { v1107 });
        let v1356: f64 = (if self.scalar_v290 { v947 } else { v1108 });
        let v1357: f64 = (if self.scalar_v290 { v948 } else { v1109 });
        let v1359: f64 = (v291 * v1354);
        let v1360: f64 = (v1359 + v1359);
        let v1361: f64 = (v291 * v1355);
        let v1362: f64 = (v1361 + v1361);
        let v1363: f64 = (v291 * v1356);
        let v1364: f64 = (v1363 + v1363);
        let v1365: f64 = (v291 * v1357);
        let v1366: f64 = (v1365 + v1365);
        let v1367: f64 = (v291 * self.scalar_v1358);
        let v1368: f64 = (v1367 + v1367);
        let v1369: f64 = (if self.scalar_v290 { v1360 } else { v1118 });
        let v1370: f64 = (if self.scalar_v290 { v1362 } else { v1119 });
        let v1371: f64 = (if self.scalar_v290 { v1364 } else { v1120 });
        let v1372: f64 = (if self.scalar_v290 { v1366 } else { v1121 });
        let v1373: f64 = (if self.scalar_v290 { v1368 } else { self.scalar_v1122 });
        let v1374: f64 = (self.scalar_v201 * v1369);
        let v1375: f64 = (self.scalar_v201 * v1370);
        let v1376: f64 = (self.scalar_v201 * v1371);
        let v1377: f64 = (self.scalar_v201 * v1372);
        let v1378: f64 = (self.scalar_v201 * v1373);
        let v1379: f64 = (v1354 + v1374);
        let v1380: f64 = (v1355 + v1375);
        let v1381: f64 = (v1356 + v1376);
        let v1382: f64 = (v1357 + v1377);
        let v1383: f64 = (self.scalar_v1358 + v1378);
        let v1384: f64 = (v181 * v1369);
        let v1385: f64 = (v181 * v1370);
        let v1386: f64 = (v181 * v1371);
        let v1387: f64 = (v181 * v1372);
        let v1388: f64 = (v181 * v1373);
        let v1389: f64 = (v296 * v1354);
        let v1390: f64 = (v291 * v1384);
        let v1391: f64 = (v1389 + v1390);
        let v1392: f64 = (v296 * v1355);
        let v1393: f64 = (v291 * v1385);
        let v1394: f64 = (v1392 + v1393);
        let v1395: f64 = (v296 * v1356);
        let v1396: f64 = (v291 * v1386);
        let v1397: f64 = (v1395 + v1396);
        let v1398: f64 = (v296 * v1357);
        let v1399: f64 = (v291 * v1387);
        let v1400: f64 = (v1398 + v1399);
        let v1401: f64 = (v296 * self.scalar_v1358);
        let v1402: f64 = (v291 * v1388);
        let v1403: f64 = (v1401 + v1402);
        let v1404: f64 = (v1379 + v1391);
        let v1405: f64 = (v1380 + v1394);
        let v1406: f64 = (v1381 + v1397);
        let v1407: f64 = (v1382 + v1400);
        let v1408: f64 = (v1383 + v1403);
        let v1409: f64 = (v176 * v1404);
        let v1410: f64 = (v298 * v924);
        let v1411: f64 = (v176 * v1405);
        let v1412: f64 = (v1410 + v1411);
        let v1413: f64 = (v298 * v925);
        let v1414: f64 = (v176 * v1406);
        let v1415: f64 = (v1413 + v1414);
        let v1416: f64 = (v176 * v1407);
        let v1417: f64 = (v176 * v1408);
        let v1418: f64 = (if self.scalar_v290 { v1409 } else { v995 });
        let v1419: f64 = (if self.scalar_v290 { v1412 } else { v996 });
        let v1420: f64 = (if self.scalar_v290 { v1415 } else { v997 });
        let v1421: f64 = (if self.scalar_v290 { v1416 } else { v998 });
        let v1422: f64 = (if self.scalar_v290 { v1417 } else { v999 });
        let v1423: f64 = { let limexp_arg = v300; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v1424: f64 = (v1418 * v1423);
        let v1425: f64 = (v1419 * v1423);
        let v1426: f64 = (v1420 * v1423);
        let v1427: f64 = (v1421 * v1423);
        let v1428: f64 = (v1422 * v1423);
        let v1429: f64 = (-v1418);
        let v1430: f64 = (-v1419);
        let v1431: f64 = (-v1420);
        let v1432: f64 = (-v1421);
        let v1433: f64 = (-v1422);
        let v1434: f64 = { let limexp_arg = v302; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v1435: f64 = (v1429 * v1434);
        let v1436: f64 = (v1430 * v1434);
        let v1437: f64 = (v1431 * v1434);
        let v1438: f64 = (v1432 * v1434);
        let v1439: f64 = (v1433 * v1434);
        let v1440: f64 = (v1424 - v1435);
        let v1441: f64 = (v1425 - v1436);
        let v1442: f64 = (v1426 - v1437);
        let v1443: f64 = (v1427 - v1438);
        let v1444: f64 = (v1428 - v1439);
        let v1445: f64 = (v152 * v1440);
        let v1446: f64 = (v152 * v1441);
        let v1447: f64 = (v152 * v1442);
        let v1448: f64 = (v152 * v1443);
        let v1449: f64 = (v152 * v1444);
        let v1450: f64 = (v306 * v306);
        let v1451: f64 = (v44 - v1450);
        let v1452: f64 = (v1445 * v1451);
        let v1453: f64 = (v1446 * v1451);
        let v1454: f64 = (v1447 * v1451);
        let v1455: f64 = (v1448 * v1451);
        let v1456: f64 = (v1449 * v1451);
        let v1457: f64 = (if self.scalar_v290 { v1452 } else { v1036 });
        let v1458: f64 = (if self.scalar_v290 { v1453 } else { v1037 });
        let v1459: f64 = (if self.scalar_v290 { v1454 } else { v1038 });
        let v1460: f64 = (if self.scalar_v290 { v1455 } else { v1039 });
        let v1461: f64 = (if self.scalar_v290 { v1456 } else { v1040 });
        let v1462: f64 = (self.scalar_v184 * v1457);
        let v1463: f64 = (self.scalar_v184 * v1458);
        let v1464: f64 = (self.scalar_v184 * v1459);
        let v1465: f64 = (self.scalar_v184 * v1460);
        let v1466: f64 = (self.scalar_v184 * v1461);
        let v1467: f64 = (if self.scalar_v290 { v1462 } else { v13 });
        let v1468: f64 = (if self.scalar_v290 { v1463 } else { v13 });
        let v1469: f64 = (if self.scalar_v290 { v1464 } else { v13 });
        let v1470: f64 = (if self.scalar_v290 { v1465 } else { v13 });
        let v1471: f64 = (if self.scalar_v290 { v1466 } else { v13 });
        let v1472: f64 = (v7 * v1467);
        let v1473: f64 = (v7 * v1468);
        let v1474: f64 = (v311 + v1473);
        let v1475: f64 = (-v311);
        let v1476: f64 = (v7 * v1469);
        let v1477: f64 = (v1475 + v1476);
        let v1478: f64 = (v7 * v1470);
        let v1479: f64 = (v7 * v1471);
        let v1480: f64 = (v313 * v313);
        let v1481: f64 = (v44 - v1480);
        let v1482: f64 = (v1472 * v1481);
        let v1483: f64 = (v1474 * v1481);
        let v1484: f64 = (v1477 * v1481);
        let v1485: f64 = (v1478 * v1481);
        let v1486: f64 = (v1479 * v1481);
        let v1487: f64 = (if self.scalar_v290 { v1482 } else { v13 });
        let v1488: f64 = (if self.scalar_v290 { v1483 } else { v13 });
        let v1489: f64 = (if self.scalar_v290 { v1484 } else { v13 });
        let v1490: f64 = (if self.scalar_v290 { v1485 } else { v13 });
        let v1491: f64 = (if self.scalar_v290 { v1486 } else { v13 });
        let v1492: f64 = (self.scalar_v256 * v1457);
        let v1493: f64 = (self.scalar_v256 * v1458);
        let v1494: f64 = (self.scalar_v256 * v1459);
        let v1495: f64 = (self.scalar_v256 * v1460);
        let v1496: f64 = (self.scalar_v256 * v1461);
        let v1497: f64 = (if self.scalar_v290 { v1492 } else { v1200 });
        let v1498: f64 = (if self.scalar_v290 { v1493 } else { v1201 });
        let v1499: f64 = (if self.scalar_v290 { v1494 } else { v1202 });
        let v1500: f64 = (if self.scalar_v290 { v1495 } else { v1203 });
        let v1501: f64 = (if self.scalar_v290 { v1496 } else { v1204 });
        let v1502: f64 = (v135 * v1457);
        let v1503: f64 = (v135 * v1458);
        let v1504: f64 = (v135 * v1459);
        let v1505: f64 = (v135 * v1460);
        let v1506: f64 = (v135 * v1461);
        let v1507: f64 = (v318 * v1487);
        let v1508: f64 = (v314 * v1502);
        let v1509: f64 = (v1507 + v1508);
        let v1510: f64 = (v318 * v1488);
        let v1511: f64 = (v314 * v1503);
        let v1512: f64 = (v1510 + v1511);
        let v1513: f64 = (v318 * v1489);
        let v1514: f64 = (v314 * v1504);
        let v1515: f64 = (v1513 + v1514);
        let v1516: f64 = (v318 * v1490);
        let v1517: f64 = (v314 * v1505);
        let v1518: f64 = (v1516 + v1517);
        let v1519: f64 = (v318 * v1491);
        let v1520: f64 = (v314 * v1506);
        let v1521: f64 = (v1519 + v1520);
        let v1522: f64 = (v7 * v1497);
        let v1523: f64 = (v7 * v1498);
        let v1524: f64 = (v317 + v1523);
        let v1525: f64 = (-v317);
        let v1526: f64 = (v7 * v1499);
        let v1527: f64 = (v1525 + v1526);
        let v1528: f64 = (v7 * v1500);
        let v1529: f64 = (v7 * v1501);
        let v1530: f64 = { let limexp_arg = v322; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v1531: f64 = (self.scalar_v264 * v1530);
        let v1532: f64 = (self.scalar_v1223 * v1530);
        let v1533: f64 = (v136 * v1531);
        let v1534: f64 = (v136 * v1532);
        let v1535: f64 = (v1524 + v1533);
        let v1536: f64 = (v1528 + v1534);
        let v1537: f64 = (v325 * v1509);
        let v1538: f64 = (v319 * v1522);
        let v1539: f64 = (v1537 + v1538);
        let v1540: f64 = (v325 * v1512);
        let v1541: f64 = (v319 * v1535);
        let v1542: f64 = (v1540 + v1541);
        let v1543: f64 = (v325 * v1515);
        let v1544: f64 = (v319 * v1527);
        let v1545: f64 = (v1543 + v1544);
        let v1546: f64 = (v325 * v1518);
        let v1547: f64 = (v319 * v1536);
        let v1548: f64 = (v1546 + v1547);
        let v1549: f64 = (v325 * v1521);
        let v1550: f64 = (v319 * v1529);
        let v1551: f64 = (v1549 + v1550);
        let v1552: f64 = (if self.scalar_v290 { v1539 } else { v1349 });
        let v1553: f64 = (if self.scalar_v290 { v1542 } else { v1350 });
        let v1554: f64 = (if self.scalar_v290 { v1545 } else { v1351 });
        let v1555: f64 = (if self.scalar_v290 { v1548 } else { v1352 });
        let v1556: f64 = (if self.scalar_v290 { v1551 } else { v1353 });
        let v1557: f64 = (if self.scalar_v330 { v945 } else { v1354 });
        let v1558: f64 = (if self.scalar_v330 { v946 } else { v1355 });
        let v1559: f64 = (if self.scalar_v330 { v947 } else { v1356 });
        let v1560: f64 = (if self.scalar_v330 { v948 } else { v1357 });
        let v1562: f64 = (v331 * v1557);
        let v1563: f64 = (v1562 + v1562);
        let v1564: f64 = (v331 * v1558);
        let v1565: f64 = (v1564 + v1564);
        let v1566: f64 = (v331 * v1559);
        let v1567: f64 = (v1566 + v1566);
        let v1568: f64 = (v331 * v1560);
        let v1569: f64 = (v1568 + v1568);
        let v1570: f64 = (v331 * self.scalar_v1561);
        let v1571: f64 = (v1570 + v1570);
        let v1572: f64 = (if self.scalar_v330 { v1563 } else { v1369 });
        let v1573: f64 = (if self.scalar_v330 { v1565 } else { v1370 });
        let v1574: f64 = (if self.scalar_v330 { v1567 } else { v1371 });
        let v1575: f64 = (if self.scalar_v330 { v1569 } else { v1372 });
        let v1576: f64 = (if self.scalar_v330 { v1571 } else { v1373 });
        let v1577: f64 = (self.scalar_v201 * v1572);
        let v1578: f64 = (self.scalar_v201 * v1573);
        let v1579: f64 = (self.scalar_v201 * v1574);
        let v1580: f64 = (self.scalar_v201 * v1575);
        let v1581: f64 = (self.scalar_v201 * v1576);
        let v1582: f64 = (v1557 + v1577);
        let v1583: f64 = (v1558 + v1578);
        let v1584: f64 = (v1559 + v1579);
        let v1585: f64 = (v1560 + v1580);
        let v1586: f64 = (self.scalar_v1561 + v1581);
        let v1587: f64 = (v181 * v1572);
        let v1588: f64 = (v181 * v1573);
        let v1589: f64 = (v181 * v1574);
        let v1590: f64 = (v181 * v1575);
        let v1591: f64 = (v181 * v1576);
        let v1592: f64 = (v336 * v1557);
        let v1593: f64 = (v331 * v1587);
        let v1594: f64 = (v1592 + v1593);
        let v1595: f64 = (v336 * v1558);
        let v1596: f64 = (v331 * v1588);
        let v1597: f64 = (v1595 + v1596);
        let v1598: f64 = (v336 * v1559);
        let v1599: f64 = (v331 * v1589);
        let v1600: f64 = (v1598 + v1599);
        let v1601: f64 = (v336 * v1560);
        let v1602: f64 = (v331 * v1590);
        let v1603: f64 = (v1601 + v1602);
        let v1604: f64 = (v336 * self.scalar_v1561);
        let v1605: f64 = (v331 * v1591);
        let v1606: f64 = (v1604 + v1605);
        let v1607: f64 = (v1582 + v1594);
        let v1608: f64 = (v1583 + v1597);
        let v1609: f64 = (v1584 + v1600);
        let v1610: f64 = (v1585 + v1603);
        let v1611: f64 = (v1586 + v1606);
        let v1612: f64 = (v176 * v1607);
        let v1613: f64 = (v338 * v924);
        let v1614: f64 = (v176 * v1608);
        let v1615: f64 = (v1613 + v1614);
        let v1616: f64 = (v338 * v925);
        let v1617: f64 = (v176 * v1609);
        let v1618: f64 = (v1616 + v1617);
        let v1619: f64 = (v176 * v1610);
        let v1620: f64 = (v176 * v1611);
        let v1621: f64 = (if self.scalar_v330 { v1612 } else { v1418 });
        let v1622: f64 = (if self.scalar_v330 { v1615 } else { v1419 });
        let v1623: f64 = (if self.scalar_v330 { v1618 } else { v1420 });
        let v1624: f64 = (if self.scalar_v330 { v1619 } else { v1421 });
        let v1625: f64 = (if self.scalar_v330 { v1620 } else { v1422 });
        let v1626: f64 = (if self.scalar_v330 { v945 } else { v1136 });
        let v1627: f64 = (if self.scalar_v330 { v1103 } else { v1137 });
        let v1628: f64 = (if self.scalar_v330 { v1104 } else { v1138 });
        let v1629: f64 = (if self.scalar_v330 { v1105 } else { v1139 });
        let v1630: f64 = (if self.scalar_v330 { v13 } else { v1140 });
        let v1631: f64 = (v341 * v1626);
        let v1632: f64 = (v1631 + v1631);
        let v1633: f64 = (v341 * v1627);
        let v1634: f64 = (v1633 + v1633);
        let v1635: f64 = (v341 * v1628);
        let v1636: f64 = (v1635 + v1635);
        let v1637: f64 = (v341 * v1629);
        let v1638: f64 = (v1637 + v1637);
        let v1639: f64 = (v341 * v1630);
        let v1640: f64 = (v1639 + v1639);
        let v1641: f64 = (if self.scalar_v330 { v1632 } else { v13 });
        let v1642: f64 = (if self.scalar_v330 { v1634 } else { v13 });
        let v1643: f64 = (if self.scalar_v330 { v1636 } else { v13 });
        let v1644: f64 = (if self.scalar_v330 { v1638 } else { v13 });
        let v1645: f64 = (if self.scalar_v330 { v1640 } else { v13 });
        let v1646: f64 = (self.scalar_v201 * v1641);
        let v1647: f64 = (self.scalar_v201 * v1642);
        let v1648: f64 = (self.scalar_v201 * v1643);
        let v1649: f64 = (self.scalar_v201 * v1644);
        let v1650: f64 = (self.scalar_v201 * v1645);
        let v1651: f64 = (v1626 + v1646);
        let v1652: f64 = (v1627 + v1647);
        let v1653: f64 = (v1628 + v1648);
        let v1654: f64 = (v1629 + v1649);
        let v1655: f64 = (v1630 + v1650);
        let v1656: f64 = (v181 * v1626);
        let v1657: f64 = (v181 * v1627);
        let v1658: f64 = (v181 * v1628);
        let v1659: f64 = (v181 * v1629);
        let v1660: f64 = (v181 * v1630);
        let v1661: f64 = (v346 * v1641);
        let v1662: f64 = (v343 * v1656);
        let v1663: f64 = (v1661 + v1662);
        let v1664: f64 = (v346 * v1642);
        let v1665: f64 = (v343 * v1657);
        let v1666: f64 = (v1664 + v1665);
        let v1667: f64 = (v346 * v1643);
        let v1668: f64 = (v343 * v1658);
        let v1669: f64 = (v1667 + v1668);
        let v1670: f64 = (v346 * v1644);
        let v1671: f64 = (v343 * v1659);
        let v1672: f64 = (v1670 + v1671);
        let v1673: f64 = (v346 * v1645);
        let v1674: f64 = (v343 * v1660);
        let v1675: f64 = (v1673 + v1674);
        let v1676: f64 = (v1651 + v1663);
        let v1677: f64 = (v1652 + v1666);
        let v1678: f64 = (v1653 + v1669);
        let v1679: f64 = (v1654 + v1672);
        let v1680: f64 = (v1655 + v1675);
        let v1681: f64 = (v176 * v1676);
        let v1682: f64 = (v348 * v924);
        let v1683: f64 = (v176 * v1677);
        let v1684: f64 = (v1682 + v1683);
        let v1685: f64 = (v348 * v925);
        let v1686: f64 = (v176 * v1678);
        let v1687: f64 = (v1685 + v1686);
        let v1688: f64 = (v176 * v1679);
        let v1689: f64 = (v176 * v1680);
        let v1690: f64 = (if self.scalar_v330 { v1681 } else { v1168 });
        let v1691: f64 = (if self.scalar_v330 { v1684 } else { v1169 });
        let v1692: f64 = (if self.scalar_v330 { v1687 } else { v1170 });
        let v1693: f64 = (if self.scalar_v330 { v1688 } else { v1171 });
        let v1694: f64 = (if self.scalar_v330 { v1689 } else { v1172 });
        let v1695: f64 = { let limexp_arg = v340; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v1696: f64 = (v1621 * v1695);
        let v1697: f64 = (v1622 * v1695);
        let v1698: f64 = (v1623 * v1695);
        let v1699: f64 = (v1624 * v1695);
        let v1700: f64 = (v1625 * v1695);
        let v1701: f64 = (-v1621);
        let v1702: f64 = (-v1622);
        let v1703: f64 = (-v1623);
        let v1704: f64 = (-v1624);
        let v1705: f64 = (-v1625);
        let v1706: f64 = { let limexp_arg = v352; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v1707: f64 = (v1701 * v1706);
        let v1708: f64 = (v1702 * v1706);
        let v1709: f64 = (v1703 * v1706);
        let v1710: f64 = (v1704 * v1706);
        let v1711: f64 = (v1705 * v1706);
        let v1712: f64 = (v1696 - v1707);
        let v1713: f64 = (v1697 - v1708);
        let v1714: f64 = (v1698 - v1709);
        let v1715: f64 = (v1699 - v1710);
        let v1716: f64 = (v1700 - v1711);
        let v1717: f64 = (v152 * v1712);
        let v1718: f64 = (v152 * v1713);
        let v1719: f64 = (v152 * v1714);
        let v1720: f64 = (v152 * v1715);
        let v1721: f64 = (v152 * v1716);
        let v1722: f64 = (v356 * v356);
        let v1723: f64 = (v44 - v1722);
        let v1724: f64 = (v1717 * v1723);
        let v1725: f64 = (v1718 * v1723);
        let v1726: f64 = (v1719 * v1723);
        let v1727: f64 = (v1720 * v1723);
        let v1728: f64 = (v1721 * v1723);
        let v1729: f64 = (if self.scalar_v330 { v1724 } else { v1457 });
        let v1730: f64 = (if self.scalar_v330 { v1725 } else { v1458 });
        let v1731: f64 = (if self.scalar_v330 { v1726 } else { v1459 });
        let v1732: f64 = (if self.scalar_v330 { v1727 } else { v1460 });
        let v1733: f64 = (if self.scalar_v330 { v1728 } else { v1461 });
        let v1734: f64 = { let limexp_arg = v350; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v1735: f64 = (v1690 * v1734);
        let v1736: f64 = (v1691 * v1734);
        let v1737: f64 = (v1692 * v1734);
        let v1738: f64 = (v1693 * v1734);
        let v1739: f64 = (v1694 * v1734);
        let v1740: f64 = (-v1690);
        let v1741: f64 = (-v1691);
        let v1742: f64 = (-v1692);
        let v1743: f64 = (-v1693);
        let v1744: f64 = (-v1694);
        let v1745: f64 = { let limexp_arg = v360; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v1746: f64 = (v1740 * v1745);
        let v1747: f64 = (v1741 * v1745);
        let v1748: f64 = (v1742 * v1745);
        let v1749: f64 = (v1743 * v1745);
        let v1750: f64 = (v1744 * v1745);
        let v1751: f64 = (v1735 - v1746);
        let v1752: f64 = (v1736 - v1747);
        let v1753: f64 = (v1737 - v1748);
        let v1754: f64 = (v1738 - v1749);
        let v1755: f64 = (v1739 - v1750);
        let v1756: f64 = (v152 * v1751);
        let v1757: f64 = (v152 * v1752);
        let v1758: f64 = (v152 * v1753);
        let v1759: f64 = (v152 * v1754);
        let v1760: f64 = (v152 * v1755);
        let v1761: f64 = (v364 * v364);
        let v1762: f64 = (v44 - v1761);
        let v1763: f64 = (v1756 * v1762);
        let v1764: f64 = (v1757 * v1762);
        let v1765: f64 = (v1758 * v1762);
        let v1766: f64 = (v1759 * v1762);
        let v1767: f64 = (v1760 * v1762);
        let v1768: f64 = (if self.scalar_v330 { v1763 } else { v13 });
        let v1769: f64 = (if self.scalar_v330 { v1764 } else { v13 });
        let v1770: f64 = (if self.scalar_v330 { v1765 } else { v13 });
        let v1771: f64 = (if self.scalar_v330 { v1766 } else { v13 });
        let v1772: f64 = (if self.scalar_v330 { v1767 } else { v13 });
        let v1773: f64 = (self.scalar_v184 * v1729);
        let v1774: f64 = (self.scalar_v184 * v1730);
        let v1775: f64 = (self.scalar_v184 * v1731);
        let v1776: f64 = (self.scalar_v184 * v1732);
        let v1777: f64 = (self.scalar_v184 * v1733);
        let v1778: f64 = (if self.scalar_v330 { v1773 } else { v1467 });
        let v1779: f64 = (if self.scalar_v330 { v1774 } else { v1468 });
        let v1780: f64 = (if self.scalar_v330 { v1775 } else { v1469 });
        let v1781: f64 = (if self.scalar_v330 { v1776 } else { v1470 });
        let v1782: f64 = (if self.scalar_v330 { v1777 } else { v1471 });
        let v1783: f64 = (self.scalar_v184 * v1768);
        let v1784: f64 = (self.scalar_v184 * v1769);
        let v1785: f64 = (self.scalar_v184 * v1770);
        let v1786: f64 = (self.scalar_v184 * v1771);
        let v1787: f64 = (self.scalar_v184 * v1772);
        let v1788: f64 = (if self.scalar_v330 { v1783 } else { v13 });
        let v1789: f64 = (if self.scalar_v330 { v1784 } else { v13 });
        let v1790: f64 = (if self.scalar_v330 { v1785 } else { v13 });
        let v1791: f64 = (if self.scalar_v330 { v1786 } else { v13 });
        let v1792: f64 = (if self.scalar_v330 { v1787 } else { v13 });
        let v1793: f64 = (v7 * v1778);
        let v1794: f64 = (v7 * v1779);
        let v1795: f64 = (v369 + v1794);
        let v1796: f64 = (-v369);
        let v1797: f64 = (v7 * v1780);
        let v1798: f64 = (v1796 + v1797);
        let v1799: f64 = (v7 * v1781);
        let v1800: f64 = (v7 * v1782);
        let v1801: f64 = (v374 * v374);
        let v1802: f64 = (v44 - v1801);
        let v1803: f64 = (v1793 * v1802);
        let v1804: f64 = (v1795 * v1802);
        let v1805: f64 = (v1798 * v1802);
        let v1806: f64 = (v1799 * v1802);
        let v1807: f64 = (v1800 * v1802);
        let v1808: f64 = (if self.scalar_v330 { v1803 } else { v1487 });
        let v1809: f64 = (if self.scalar_v330 { v1804 } else { v1488 });
        let v1810: f64 = (if self.scalar_v330 { v1805 } else { v1489 });
        let v1811: f64 = (if self.scalar_v330 { v1806 } else { v1490 });
        let v1812: f64 = (if self.scalar_v330 { v1807 } else { v1491 });
        let v1813: f64 = (v7 * v1788);
        let v1814: f64 = (v7 * v1789);
        let v1815: f64 = (v372 + v1814);
        let v1816: f64 = (-v372);
        let v1817: f64 = (v7 * v1790);
        let v1818: f64 = (v1816 + v1817);
        let v1819: f64 = (v7 * v1791);
        let v1820: f64 = (v7 * v1792);
        let v1821: f64 = (v377 * v377);
        let v1822: f64 = (v44 - v1821);
        let v1823: f64 = (v1813 * v1822);
        let v1824: f64 = (v1815 * v1822);
        let v1825: f64 = (v1818 * v1822);
        let v1826: f64 = (v1819 * v1822);
        let v1827: f64 = (v1820 * v1822);
        let v1828: f64 = (if self.scalar_v330 { v1823 } else { v13 });
        let v1829: f64 = (if self.scalar_v330 { v1824 } else { v13 });
        let v1830: f64 = (if self.scalar_v330 { v1825 } else { v13 });
        let v1831: f64 = (if self.scalar_v330 { v1826 } else { v13 });
        let v1832: f64 = (if self.scalar_v330 { v1827 } else { v13 });
        let v1833: f64 = (self.scalar_v256 * v1768);
        let v1834: f64 = (self.scalar_v256 * v1769);
        let v1835: f64 = (self.scalar_v256 * v1770);
        let v1836: f64 = (self.scalar_v256 * v1771);
        let v1837: f64 = (self.scalar_v256 * v1772);
        let v1838: f64 = (if self.scalar_v330 { v1833 } else { v13 });
        let v1839: f64 = (if self.scalar_v330 { v1834 } else { v13 });
        let v1840: f64 = (if self.scalar_v330 { v1835 } else { v13 });
        let v1841: f64 = (if self.scalar_v330 { v1836 } else { v13 });
        let v1842: f64 = (if self.scalar_v330 { v1837 } else { v13 });
        let v1843: f64 = (self.scalar_v256 * v1729);
        let v1844: f64 = (self.scalar_v256 * v1730);
        let v1845: f64 = (self.scalar_v256 * v1731);
        let v1846: f64 = (self.scalar_v256 * v1732);
        let v1847: f64 = (self.scalar_v256 * v1733);
        let v1848: f64 = (if self.scalar_v330 { v1843 } else { v13 });
        let v1849: f64 = (if self.scalar_v330 { v1844 } else { v13 });
        let v1850: f64 = (if self.scalar_v330 { v1845 } else { v13 });
        let v1851: f64 = (if self.scalar_v330 { v1846 } else { v13 });
        let v1852: f64 = (if self.scalar_v330 { v1847 } else { v13 });
        let v1853: f64 = (v135 * v1729);
        let v1854: f64 = (v135 * v1730);
        let v1855: f64 = (v135 * v1731);
        let v1856: f64 = (v135 * v1732);
        let v1857: f64 = (v135 * v1733);
        let v1858: f64 = (v386 * v1853);
        let v1859: f64 = (v385 * v1808);
        let v1860: f64 = (v1858 + v1859);
        let v1861: f64 = (v386 * v1854);
        let v1862: f64 = (v385 * v1809);
        let v1863: f64 = (v1861 + v1862);
        let v1864: f64 = (v386 * v1855);
        let v1865: f64 = (v385 * v1810);
        let v1866: f64 = (v1864 + v1865);
        let v1867: f64 = (v386 * v1856);
        let v1868: f64 = (v385 * v1811);
        let v1869: f64 = (v1867 + v1868);
        let v1870: f64 = (v386 * v1857);
        let v1871: f64 = (v385 * v1812);
        let v1872: f64 = (v1870 + v1871);
        let v1873: f64 = (v7 * v1848);
        let v1874: f64 = (v7 * v1849);
        let v1875: f64 = (v384 + v1874);
        let v1876: f64 = (-v384);
        let v1877: f64 = (v7 * v1850);
        let v1878: f64 = (v1876 + v1877);
        let v1879: f64 = (v7 * v1851);
        let v1880: f64 = (v7 * v1852);
        let v1881: f64 = (v1227 + v1875);
        let v1882: f64 = (v1228 + v1878);
        let v1883: f64 = (v390 * v1860);
        let v1884: f64 = (v387 * v1873);
        let v1885: f64 = (v1883 + v1884);
        let v1886: f64 = (v390 * v1863);
        let v1887: f64 = (v387 * v1881);
        let v1888: f64 = (v1886 + v1887);
        let v1889: f64 = (v390 * v1866);
        let v1890: f64 = (v387 * v1882);
        let v1891: f64 = (v1889 + v1890);
        let v1892: f64 = (v390 * v1869);
        let v1893: f64 = (v387 * v1879);
        let v1894: f64 = (v1892 + v1893);
        let v1895: f64 = (v390 * v1872);
        let v1896: f64 = (v387 * v1880);
        let v1897: f64 = (v1895 + v1896);
        let v1898: f64 = (if self.scalar_v330 { v1885 } else { v1246 });
        let v1899: f64 = (if self.scalar_v330 { v1888 } else { v1247 });
        let v1900: f64 = (if self.scalar_v330 { v1891 } else { v1248 });
        let v1901: f64 = (if self.scalar_v330 { v1894 } else { v1249 });
        let v1902: f64 = (if self.scalar_v330 { v1897 } else { v1250 });
        let v1903: f64 = (v135 * v1768);
        let v1904: f64 = (v135 * v1769);
        let v1905: f64 = (v135 * v1770);
        let v1906: f64 = (v135 * v1771);
        let v1907: f64 = (v135 * v1772);
        let v1908: f64 = (-v1828);
        let v1909: f64 = (-v1829);
        let v1910: f64 = (-v1830);
        let v1911: f64 = (-v1831);
        let v1912: f64 = (-v1832);
        let v1913: f64 = (v394 * v1903);
        let v1914: f64 = (v393 * v1908);
        let v1915: f64 = (v1913 + v1914);
        let v1916: f64 = (v394 * v1904);
        let v1917: f64 = (v393 * v1909);
        let v1918: f64 = (v1916 + v1917);
        let v1919: f64 = (v394 * v1905);
        let v1920: f64 = (v393 * v1910);
        let v1921: f64 = (v1919 + v1920);
        let v1922: f64 = (v394 * v1906);
        let v1923: f64 = (v393 * v1911);
        let v1924: f64 = (v1922 + v1923);
        let v1925: f64 = (v394 * v1907);
        let v1926: f64 = (v393 * v1912);
        let v1927: f64 = (v1925 + v1926);
        let v1928: f64 = (v7 * v1838);
        let v1929: f64 = (v7 * v1839);
        let v1930: f64 = (v381 + v1929);
        let v1931: f64 = (-v381);
        let v1932: f64 = (v7 * v1840);
        let v1933: f64 = (v1931 + v1932);
        let v1934: f64 = (v7 * v1841);
        let v1935: f64 = (v7 * v1842);
        let v1936: f64 = (-v1928);
        let v1937: f64 = (-v1930);
        let v1938: f64 = (-v1933);
        let v1939: f64 = (-v1934);
        let v1940: f64 = (-v1935);
        let v1941: f64 = (v397 * v1915);
        let v1942: f64 = (v395 * v1936);
        let v1943: f64 = (v1941 + v1942);
        let v1944: f64 = (v397 * v1918);
        let v1945: f64 = (v395 * v1937);
        let v1946: f64 = (v1944 + v1945);
        let v1947: f64 = (v397 * v1921);
        let v1948: f64 = (v395 * v1938);
        let v1949: f64 = (v1947 + v1948);
        let v1950: f64 = (v397 * v1924);
        let v1951: f64 = (v395 * v1939);
        let v1952: f64 = (v1950 + v1951);
        let v1953: f64 = (v397 * v1927);
        let v1954: f64 = (v395 * v1940);
        let v1955: f64 = (v1953 + v1954);
        let v1956: f64 = (if self.scalar_v330 { v1943 } else { v1334 });
        let v1957: f64 = (if self.scalar_v330 { v1946 } else { v1335 });
        let v1958: f64 = (if self.scalar_v330 { v1949 } else { v1336 });
        let v1959: f64 = (if self.scalar_v330 { v1952 } else { v1337 });
        let v1960: f64 = (if self.scalar_v330 { v1955 } else { v1338 });
        let v1961: f64 = (v1898 - v1956);
        let v1962: f64 = (v1899 - v1957);
        let v1963: f64 = (v1900 - v1958);
        let v1964: f64 = (v1901 - v1959);
        let v1965: f64 = (v1902 - v1960);
        let v1966: f64 = (v152 * v1961);
        let v1967: f64 = (v152 * v1962);
        let v1968: f64 = (v152 * v1963);
        let v1969: f64 = (v152 * v1964);
        let v1970: f64 = (v152 * v1965);
        let v1971: f64 = (if self.scalar_v330 { v1966 } else { v1552 });
        let v1972: f64 = (if self.scalar_v330 { v1967 } else { v1553 });
        let v1973: f64 = (if self.scalar_v330 { v1968 } else { v1554 });
        let v1974: f64 = (if self.scalar_v330 { v1969 } else { v1555 });
        let v1975: f64 = (if self.scalar_v330 { v1970 } else { v1556 });
        let v1976: f64 = (if self.scalar_v405 { v1195 } else { v1497 });
        let v1977: f64 = (if self.scalar_v405 { v1196 } else { v1498 });
        let v1978: f64 = (if self.scalar_v405 { v1197 } else { v1499 });
        let v1979: f64 = (if self.scalar_v405 { v1198 } else { v1500 });
        let v1980: f64 = (if self.scalar_v405 { v1199 } else { v1501 });
        let v1981: f64 = (if self.scalar_v405 { v1773 } else { v1778 });
        let v1982: f64 = (if self.scalar_v405 { v1774 } else { v1779 });
        let v1983: f64 = (if self.scalar_v405 { v1775 } else { v1780 });
        let v1984: f64 = (if self.scalar_v405 { v1776 } else { v1781 });
        let v1985: f64 = (if self.scalar_v405 { v1777 } else { v1782 });
        let v1986: f64 = (v7 * v1981);
        let v1987: f64 = (v7 * v1982);
        let v1988: f64 = (v407 + v1987);
        let v1989: f64 = (-v407);
        let v1990: f64 = (v7 * v1983);
        let v1991: f64 = (v1989 + v1990);
        let v1992: f64 = (v7 * v1984);
        let v1993: f64 = (v7 * v1985);
        let v1994: f64 = (v409 * v409);
        let v1995: f64 = (v44 - v1994);
        let v1996: f64 = (v1986 * v1995);
        let v1997: f64 = (v1988 * v1995);
        let v1998: f64 = (v1991 * v1995);
        let v1999: f64 = (v1992 * v1995);
        let v2000: f64 = (v1993 * v1995);
        let v2001: f64 = (if self.scalar_v405 { v1996 } else { v1808 });
        let v2002: f64 = (if self.scalar_v405 { v1997 } else { v1809 });
        let v2003: f64 = (if self.scalar_v405 { v1998 } else { v1810 });
        let v2004: f64 = (if self.scalar_v405 { v1999 } else { v1811 });
        let v2005: f64 = (if self.scalar_v405 { v2000 } else { v1812 });
        let v2006: f64 = (v11 * v1981);
        let v2007: f64 = (v407 + v2006);
        let v2008: f64 = (v11 * v1982);
        let v2009: f64 = (v11 * v1983);
        let v2010: f64 = (v1989 + v2009);
        let v2011: f64 = (v11 * v1984);
        let v2012: f64 = (v11 * v1985);
        let v2013: f64 = (v412 * v412);
        let v2014: f64 = (v44 - v2013);
        let v2015: f64 = (v2007 * v2014);
        let v2016: f64 = (v2008 * v2014);
        let v2017: f64 = (v2010 * v2014);
        let v2018: f64 = (v2011 * v2014);
        let v2019: f64 = (v2012 * v2014);
        let v2020: f64 = (if self.scalar_v405 { v2015 } else { v13 });
        let v2021: f64 = (if self.scalar_v405 { v2016 } else { v13 });
        let v2022: f64 = (if self.scalar_v405 { v2017 } else { v13 });
        let v2023: f64 = (if self.scalar_v405 { v2018 } else { v13 });
        let v2024: f64 = (if self.scalar_v405 { v2019 } else { v13 });
        let v2025: f64 = (self.scalar_v414 * v2020);
        let v2026: f64 = (self.scalar_v414 * v2021);
        let v2027: f64 = (self.scalar_v414 * v2022);
        let v2028: f64 = (self.scalar_v414 * v2023);
        let v2029: f64 = (self.scalar_v414 * v2024);
        let v2030: f64 = (v2001 + v2025);
        let v2031: f64 = (v2002 + v2026);
        let v2032: f64 = (v2003 + v2027);
        let v2033: f64 = (v2004 + v2028);
        let v2034: f64 = (v2005 + v2029);
        let v2035: f64 = (v416 * v1061);
        let v2036: f64 = (v226 * v2030);
        let v2037: f64 = (v2035 + v2036);
        let v2038: f64 = (v416 * v1062);
        let v2039: f64 = (v226 * v2031);
        let v2040: f64 = (v2038 + v2039);
        let v2041: f64 = (v416 * v1063);
        let v2042: f64 = (v226 * v2032);
        let v2043: f64 = (v2041 + v2042);
        let v2044: f64 = (v416 * v1064);
        let v2045: f64 = (v226 * v2033);
        let v2046: f64 = (v2044 + v2045);
        let v2047: f64 = (v416 * v1065);
        let v2048: f64 = (v226 * v2034);
        let v2049: f64 = (v2047 + v2048);
        let v2052: f64 = (v419 * v1976);
        let v2053: f64 = (v406 * self.scalar_v414);
        let v2054: f64 = (v2052 + v2053);
        let v2055: f64 = (v419 * v1977);
        let v2056: f64 = (v406 + v2055);
        let v2057: f64 = (v419 * v1978);
        let v2058: f64 = (v406 * self.scalar_v2051);
        let v2059: f64 = (v2057 + v2058);
        let v2060: f64 = (v419 * v1979);
        let v2061: f64 = (v419 * v1980);
        let v2062: f64 = (v1227 + v2056);
        let v2063: f64 = (v1228 + v2059);
        let v2064: f64 = (v422 * v2037);
        let v2065: f64 = (v417 * v2054);
        let v2066: f64 = (v2064 + v2065);
        let v2067: f64 = (v422 * v2040);
        let v2068: f64 = (v417 * v2062);
        let v2069: f64 = (v2067 + v2068);
        let v2070: f64 = (v422 * v2043);
        let v2071: f64 = (v417 * v2063);
        let v2072: f64 = (v2070 + v2071);
        let v2073: f64 = (v422 * v2046);
        let v2074: f64 = (v417 * v2060);
        let v2075: f64 = (v2073 + v2074);
        let v2076: f64 = (v422 * v2049);
        let v2077: f64 = (v417 * v2061);
        let v2078: f64 = (v2076 + v2077);
        let v2079: f64 = (if self.scalar_v405 { v2066 } else { v1971 });
        let v2080: f64 = (if self.scalar_v405 { v2069 } else { v1972 });
        let v2081: f64 = (if self.scalar_v405 { v2072 } else { v1973 });
        let v2082: f64 = (if self.scalar_v405 { v2075 } else { v1974 });
        let v2083: f64 = (if self.scalar_v405 { v2078 } else { v1975 });
        let v2084: f64 = (v139 * v1002);
        let v2085: f64 = (-v2084);
        let v2086: f64 = (v427 * v427);
        let v2087: f64 = (v2085 / v2086);
        let v2088: f64 = (v139 * v1003);
        let v2089: f64 = (-v2088);
        let v2090: f64 = (v2089 / v2086);
        let v2091: f64 = (v139 * v1004);
        let v2092: f64 = (-v2091);
        let v2093: f64 = (v2092 / v2086);
        let v2094: f64 = (v139 * v1005);
        let v2095: f64 = (-v2094);
        let v2096: f64 = (v2095 / v2086);
        let v2097: f64 = (v139 * v1006);
        let v2098: f64 = (-v2097);
        let v2099: f64 = (v2098 / v2086);
        let v2100: f64 = (if self.scalar_v425 { v2087 } else { v13 });
        let v2101: f64 = (if self.scalar_v425 { v2090 } else { v13 });
        let v2102: f64 = (if self.scalar_v425 { v2093 } else { v13 });
        let v2103: f64 = (if self.scalar_v425 { v2096 } else { v13 });
        let v2104: f64 = (if self.scalar_v425 { v2099 } else { v13 });
        let v2105: f64 = (self.scalar_v432 * v1002);
        let v2106: f64 = (self.scalar_v432 * v1003);
        let v2107: f64 = (self.scalar_v432 * v1004);
        let v2108: f64 = (self.scalar_v432 * v1005);
        let v2109: f64 = (self.scalar_v432 * v1006);
        let v2110: f64 = (if self.scalar_v425 { v2105 } else { v13 });
        let v2111: f64 = (if self.scalar_v425 { v2106 } else { v13 });
        let v2112: f64 = (if self.scalar_v425 { v2107 } else { v13 });
        let v2113: f64 = (if self.scalar_v425 { v2108 } else { v13 });
        let v2114: f64 = (if self.scalar_v425 { v2109 } else { v13 });
        let v2115: f64 = (v139 * v1729);
        let v2116: f64 = (-v2115);
        let v2117: f64 = (v440 * v440);
        let v2118: f64 = (v2116 / v2117);
        let v2119: f64 = (v139 * v1730);
        let v2120: f64 = (-v2119);
        let v2121: f64 = (v2120 / v2117);
        let v2122: f64 = (v139 * v1731);
        let v2123: f64 = (-v2122);
        let v2124: f64 = (v2123 / v2117);
        let v2125: f64 = (v139 * v1732);
        let v2126: f64 = (-v2125);
        let v2127: f64 = (v2126 / v2117);
        let v2128: f64 = (v139 * v1733);
        let v2129: f64 = (-v2128);
        let v2130: f64 = (v2129 / v2117);
        let v2131: f64 = (if self.scalar_v439 { v2118 } else { v2100 });
        let v2132: f64 = (if self.scalar_v439 { v2121 } else { v2101 });
        let v2133: f64 = (if self.scalar_v439 { v2124 } else { v2102 });
        let v2134: f64 = (if self.scalar_v439 { v2127 } else { v2103 });
        let v2135: f64 = (if self.scalar_v439 { v2130 } else { v2104 });
        let v2136: f64 = (self.scalar_v432 * v1729);
        let v2137: f64 = (self.scalar_v432 * v1730);
        let v2138: f64 = (self.scalar_v432 * v1731);
        let v2139: f64 = (self.scalar_v432 * v1732);
        let v2140: f64 = (self.scalar_v432 * v1733);
        let v2141: f64 = (if self.scalar_v439 { v2136 } else { v2110 });
        let v2142: f64 = (if self.scalar_v439 { v2137 } else { v2111 });
        let v2143: f64 = (if self.scalar_v439 { v2138 } else { v2112 });
        let v2144: f64 = (if self.scalar_v439 { v2139 } else { v2113 });
        let v2145: f64 = (if self.scalar_v439 { v2140 } else { v2114 });
        let v2146: f64 = (v451 * v2141);
        let v2147: f64 = (v451 * v2142);
        let v2148: f64 = (v451 * v2143);
        let v2149: f64 = (v451 * v2144);
        let v2150: f64 = (v451 * v2145);
        let v2151: f64 = (if self.scalar_v455 { v13 } else { v1557 });
        let v2152: f64 = (if self.scalar_v455 { v13 } else { v1558 });
        let v2153: f64 = (if self.scalar_v455 { v13 } else { v1559 });
        let v2154: f64 = (if self.scalar_v455 { v13 } else { v1560 });
        let v2158: f64 = (if self.scalar_v473 { v13 } else { v2151 });
        let v2159: f64 = (if self.scalar_v473 { v13 } else { v2152 });
        let v2160: f64 = (if self.scalar_v473 { v13 } else { v2153 });
        let v2161: f64 = (if self.scalar_v473 { v13 } else { v2154 });
        let v2163: f64 = (v488 * v488);
        let v2164: f64 = (v44 - v2163);
        let v2165: f64 = (-v2164);
        let v2166: f64 = (if self.scalar_v487 { v2165 } else { self.scalar_v2156 });
        let v2167: f64 = (if self.scalar_v487 { v2164 } else { self.scalar_v2157 });
        let v2168: f64 = (v490 * v490);
        let v2169: f64 = (v44 - v2168);
        let v2170: f64 = (-v2169);
        let v2171: f64 = (if self.scalar_v487 { v2170 } else { self.scalar_v2156 });
        let v2172: f64 = (if self.scalar_v487 { v2169 } else { self.scalar_v2157 });
        let v2173: f64 = (if self.scalar_v493 { v456 } else { v2166 });
        let v2174: f64 = (if self.scalar_v493 { v44 } else { v2167 });
        let v2175: f64 = (if self.scalar_v493 { v456 } else { v2171 });
        let v2176: f64 = (if self.scalar_v493 { v44 } else { v2172 });
        let v2181: f64 = { let limexp_arg = v498; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v2182: f64 = (self.scalar_v2179 * v2181);
        let v2183: f64 = (self.scalar_v2180 * v2181);
        let v2184: f64 = (v159 * v2173);
        let v2185: f64 = (v159 * v2174);
        let v2186: f64 = { let limexp_arg = v502; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v2187: f64 = (v2184 * v2186);
        let v2188: f64 = (v2185 * v2186);
        let v2189: f64 = (self.scalar_v506 * v2182);
        let v2190: f64 = (self.scalar_v506 * v2183);
        let v2191: f64 = (v2187 - v2189);
        let v2192: f64 = (v2188 - v2190);
        let v2193: f64 = (-v2158);
        let v2194: f64 = (-v2159);
        let v2195: f64 = (v2191 - v2160);
        let v2196: f64 = (-v2161);
        let v2198: f64 = (self.scalar_v501 * v2193);
        let v2199: f64 = (self.scalar_v501 * v2194);
        let v2200: f64 = (self.scalar_v501 * v2195);
        let v2201: f64 = (self.scalar_v501 * v2196);
        let v2202: f64 = (self.scalar_v501 * v2192);
        let v2204: f64 = { let limexp_arg = v511; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v2205: f64 = (self.scalar_v2179 * v2204);
        let v2206: f64 = (self.scalar_v2180 * v2204);
        let v2207: f64 = (v159 * v2175);
        let v2208: f64 = (v159 * v2176);
        let v2209: f64 = { let limexp_arg = v514; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v2210: f64 = (v2207 * v2209);
        let v2211: f64 = (v2208 * v2209);
        let v2212: f64 = (self.scalar_v506 * v2205);
        let v2213: f64 = (self.scalar_v506 * v2206);
        let v2214: f64 = (v2210 - v2212);
        let v2215: f64 = (v2211 - v2213);
        let v2216: f64 = (v2214 - v2159);
        let v2217: f64 = (-v2160);
        let v2218: f64 = (v2215 - v2161);
        let v2219: f64 = (self.scalar_v501 * v2216);
        let v2220: f64 = (self.scalar_v501 * v2217);
        let v2221: f64 = (self.scalar_v501 * v2218);
        let v2225: f64 = (v526 * v526);
        let v2226: f64 = (v44 - v2225);
        let v2227: f64 = (self.scalar_v523 * v2226);
        let v2228: f64 = (self.scalar_v2224 * v2226);
        let v2229: f64 = (self.scalar_v520 * v2226);
        let v2231: f64 = (v532 * v532);
        let v2232: f64 = (v44 - v2231);
        let v2233: f64 = (self.scalar_v529 * v2232);
        let v2234: f64 = (self.scalar_v2230 * v2232);
        let v2236: f64 = (v538 * v538);
        let v2237: f64 = (v44 - v2236);
        let v2238: f64 = (self.scalar_v2235 * v2237);
        let v2239: f64 = (self.scalar_v535 * v2237);
        let v2242: f64 = (v545 * v545);
        let v2243: f64 = (v44 - v2242);
        let v2244: f64 = (self.scalar_v2241 * v2243);
        let v2245: f64 = (self.scalar_v523 * v2243);
        let v2246: f64 = (self.scalar_v541 * v2243);
        let v2247: f64 = (v137 * v2227);
        let v2248: f64 = (v137 * v2228);
        let v2249: f64 = (v137 * v2229);
        let v2250: f64 = (v557 * v2233);
        let v2251: f64 = (v533 * v2247);
        let v2252: f64 = (v2250 + v2251);
        let v2253: f64 = (v557 * v2234);
        let v2254: f64 = (v533 * v2248);
        let v2255: f64 = (v2253 + v2254);
        let v2256: f64 = (v533 * v2249);
        let v2257: f64 = (if self.scalar_v556 { v2252 } else { v13 });
        let v2258: f64 = (if self.scalar_v556 { v2255 } else { v13 });
        let v2259: f64 = (if self.scalar_v556 { v2256 } else { v13 });
        let v2260: f64 = (v546 * v2238);
        let v2261: f64 = (v540 * v2244);
        let v2262: f64 = (v2260 + v2261);
        let v2263: f64 = (v546 * v2239);
        let v2264: f64 = (v540 * v2245);
        let v2265: f64 = (v2263 + v2264);
        let v2266: f64 = (v540 * v2246);
        let v2267: f64 = (v138 * v2262);
        let v2268: f64 = (v138 * v2265);
        let v2269: f64 = (v138 * v2266);
        let v2270: f64 = (if self.scalar_v556 { v2267 } else { v13 });
        let v2271: f64 = (if self.scalar_v556 { v2268 } else { v13 });
        let v2272: f64 = (if self.scalar_v556 { v2269 } else { v13 });
        let v2273: f64 = (if self.scalar_v569 { v2233 } else { v2233 });
        let v2274: f64 = (if self.scalar_v569 { v2234 } else { v2234 });
        let v2275: f64 = ((v572) as f64).sinh();
        let v2276: f64 = (self.scalar_v523 * v2275);
        let v2277: f64 = (self.scalar_v2223 * v2275);
        let v2278: f64 = (if self.scalar_v569 { v2276 } else { v13 });
        let v2279: f64 = (if self.scalar_v569 { v2277 } else { v13 });
        let v2280: f64 = (v2278 / v574);
        let v2281: f64 = (v2279 / v574);
        let v2282: f64 = (if self.scalar_v569 { v2280 } else { v13 });
        let v2283: f64 = (if self.scalar_v569 { v2281 } else { v13 });
        let v2285: f64 = (self.scalar_v523 * v2284);
        let v2286: f64 = (self.scalar_v2224 * v2284);
        let v2288: f64 = (if self.scalar_v569 { v2285 } else { v13 });
        let v2289: f64 = (if self.scalar_v569 { v2286 } else { v13 });
        let v2291: f64 = (v2288 / v578);
        let v2292: f64 = (v2289 / v578);
        let v2294: f64 = (if self.scalar_v569 { v2291 } else { v13 });
        let v2295: f64 = (if self.scalar_v569 { v2292 } else { v13 });
        let v2297: f64 = (self.scalar_v523 + v2282);
        let v2298: f64 = (self.scalar_v2223 + v2283);
        let v2299: f64 = (if self.scalar_v569 { v2297 } else { v13 });
        let v2300: f64 = (if self.scalar_v569 { v2298 } else { v13 });
        let v2301: f64 = (self.scalar_v523 + v2294);
        let v2302: f64 = (self.scalar_v2224 + v2295);
        let v2304: f64 = (v2301 - v2299);
        let v2305: f64 = (v2302 - v2300);
        let v2306: f64 = (v584 * v2273);
        let v2307: f64 = (v571 * v2304);
        let v2308: f64 = (v2306 + v2307);
        let v2309: f64 = (v584 * v2274);
        let v2310: f64 = (v571 * v2305);
        let v2311: f64 = (v2309 + v2310);
        let v2313: f64 = (v2308 / self.scalar_v520);
        let v2314: f64 = (v2311 / self.scalar_v520);
        let v2317: f64 = (v2314 + self.scalar_v2316);
        let v2319: f64 = (v137 * v2313);
        let v2320: f64 = (v137 * v2317);
        let v2323: f64 = (v2320 + self.scalar_v2322);
        let v2325: f64 = (if self.scalar_v569 { v2319 } else { v13 });
        let v2326: f64 = (if self.scalar_v569 { v2323 } else { v13 });
        let v2328: f64 = ((v593) as f64).sinh();
        let v2329: f64 = (self.scalar_v2223 * v2328);
        let v2330: f64 = (self.scalar_v523 * v2328);
        let v2331: f64 = (if self.scalar_v569 { v2329 } else { v2278 });
        let v2332: f64 = (if self.scalar_v569 { v2330 } else { v2279 });
        let v2333: f64 = (v2331 / v595);
        let v2334: f64 = (v2332 / v595);
        let v2335: f64 = (if self.scalar_v569 { v2333 } else { v13 });
        let v2336: f64 = (if self.scalar_v569 { v2334 } else { v13 });
        let v2338: f64 = (self.scalar_v2241 * v2337);
        let v2339: f64 = (self.scalar_v523 * v2337);
        let v2341: f64 = (if self.scalar_v569 { v2338 } else { v2288 });
        let v2342: f64 = (if self.scalar_v569 { v2339 } else { v2289 });
        let v2345: f64 = (v2341 / v599);
        let v2346: f64 = (v2342 / v599);
        let v2348: f64 = (v2344 / v599);
        let v2349: f64 = (if self.scalar_v569 { v2345 } else { v13 });
        let v2350: f64 = (if self.scalar_v569 { v2346 } else { v13 });
        let v2352: f64 = (if self.scalar_v569 { v2348 } else { v13 });
        let v2353: f64 = (self.scalar_v2223 + v2335);
        let v2354: f64 = (self.scalar_v523 + v2336);
        let v2355: f64 = (if self.scalar_v569 { v2353 } else { v13 });
        let v2356: f64 = (if self.scalar_v569 { v2354 } else { v13 });
        let v2357: f64 = (self.scalar_v2241 + v2349);
        let v2358: f64 = (self.scalar_v523 + v2350);
        let v2360: f64 = (v2357 - v2355);
        let v2361: f64 = (v2358 - v2356);
        let v2362: f64 = (v605 * v2238);
        let v2363: f64 = (v540 * v2360);
        let v2364: f64 = (v2362 + v2363);
        let v2365: f64 = (v605 * v2239);
        let v2366: f64 = (v540 * v2361);
        let v2367: f64 = (v2365 + v2366);
        let v2369: f64 = (v540 * v2352);
        let v2370: f64 = (v2364 / self.scalar_v541);
        let v2371: f64 = (v2367 / self.scalar_v541);
        let v2373: f64 = (v2369 / self.scalar_v541);
        let v2374: f64 = (self.scalar_v2316 + v2370);
        let v2376: f64 = (v138 * v2374);
        let v2377: f64 = (v138 * v2371);
        let v2379: f64 = (v138 * v2373);
        let v2381: f64 = (v2376 + self.scalar_v2380);
        let v2383: f64 = (if self.scalar_v569 { v2381 } else { v13 });
        let v2384: f64 = (if self.scalar_v569 { v2377 } else { v13 });
        let v2386: f64 = (if self.scalar_v569 { v2379 } else { v13 });
        let v2387: f64 = (if self.scalar_v569 { v13 } else { v2257 });
        let v2388: f64 = (if self.scalar_v569 { v13 } else { v2258 });
        let v2389: f64 = (if self.scalar_v569 { v13 } else { v2259 });
        let v2390: f64 = (if self.scalar_v569 { v13 } else { v2270 });
        let v2391: f64 = (if self.scalar_v569 { v13 } else { v2271 });
        let v2392: f64 = (if self.scalar_v569 { v13 } else { v2272 });
        let v2397: f64 = (v624 * self.scalar_v2395);
        let v2398: f64 = (v2397 + v2397);
        let v2399: f64 = (v624 * self.scalar_v2396);
        let v2400: f64 = (v2399 + v2399);
        let v2402: f64 = f64::powf(v628, self.scalar_v2401);
        let v2403: f64 = (self.scalar_v629 * v2402);
        let v2404: f64 = (v2398 * v2403);
        let v2405: f64 = (v2400 * v2403);
        let v2406: f64 = (self.scalar_v632 * v2398);
        let v2407: f64 = (self.scalar_v632 * v2400);
        let v2408: f64 = (v634 * v2404);
        let v2409: f64 = (v630 * v2406);
        let v2410: f64 = (v2408 + v2409);
        let v2411: f64 = (v634 * v2405);
        let v2412: f64 = (v630 * v2407);
        let v2413: f64 = (v2411 + v2412);
        let v2414: f64 = (if self.scalar_v620 { v2410 } else { v13 });
        let v2415: f64 = (if self.scalar_v620 { v2413 } else { v13 });
        let v2419: f64 = (v640 * v640);
        let v2420: f64 = (v44 - v2419);
        let v2421: f64 = (self.scalar_v2417 * v2420);
        let v2422: f64 = (self.scalar_v2418 * v2420);
        let v2423: f64 = (self.scalar_v520 * v2420);
        let v2424: f64 = (if self.scalar_v620 { v2421 } else { v2227 });
        let v2425: f64 = (if self.scalar_v620 { v2422 } else { v2228 });
        let v2426: f64 = (if self.scalar_v620 { v2423 } else { v2229 });
        let v2427: f64 = (if self.scalar_v620 { v2233 } else { v2273 });
        let v2428: f64 = (if self.scalar_v620 { v2234 } else { v2274 });
        let v2429: f64 = (if self.scalar_v620 { v2238 } else { v2238 });
        let v2430: f64 = (if self.scalar_v620 { v2239 } else { v2239 });
        let v2435: f64 = (v651 * v651);
        let v2436: f64 = (v44 - v2435);
        let v2437: f64 = (self.scalar_v2433 * v2436);
        let v2438: f64 = (self.scalar_v2434 * v2436);
        let v2439: f64 = (self.scalar_v541 * v2436);
        let v2440: f64 = (if self.scalar_v620 { v2437 } else { v2244 });
        let v2441: f64 = (if self.scalar_v620 { v2438 } else { v2245 });
        let v2442: f64 = (if self.scalar_v620 { v2439 } else { v2246 });
        let v2443: f64 = (self.scalar_v654 * v2414);
        let v2444: f64 = (self.scalar_v654 * v2415);
        let v2445: f64 = (v2425 + v2443);
        let v2446: f64 = (v2426 + v2444);
        let v2447: f64 = (v137 * v2424);
        let v2448: f64 = (v137 * v2445);
        let v2449: f64 = (v137 * v2446);
        let v2450: f64 = (v657 * v2427);
        let v2451: f64 = (v643 * v2447);
        let v2452: f64 = (v2450 + v2451);
        let v2453: f64 = (v657 * v2428);
        let v2454: f64 = (v643 * v2448);
        let v2455: f64 = (v2453 + v2454);
        let v2456: f64 = (v643 * v2449);
        let v2457: f64 = (if self.scalar_v620 { v2452 } else { v2387 });
        let v2458: f64 = (if self.scalar_v620 { v2455 } else { v2388 });
        let v2459: f64 = (if self.scalar_v620 { v2456 } else { v2389 });
        let v2460: f64 = (v653 * v2429);
        let v2461: f64 = (v646 * v2440);
        let v2462: f64 = (v2460 + v2461);
        let v2463: f64 = (v653 * v2430);
        let v2464: f64 = (v646 * v2441);
        let v2465: f64 = (v2463 + v2464);
        let v2466: f64 = (v646 * v2442);
        let v2467: f64 = (v138 * v2462);
        let v2468: f64 = (v138 * v2465);
        let v2469: f64 = (v138 * v2466);
        let v2470: f64 = (if self.scalar_v620 { v2467 } else { v2390 });
        let v2471: f64 = (if self.scalar_v620 { v2468 } else { v2391 });
        let v2472: f64 = (if self.scalar_v620 { v2469 } else { v2392 });
        let v2473: f64 = (if self.scalar_v668 { v2276 } else { v2331 });
        let v2474: f64 = (if self.scalar_v668 { v2277 } else { v2332 });
        let v2475: f64 = (v2473 / v669);
        let v2476: f64 = (v2474 / v669);
        let v2477: f64 = (if self.scalar_v668 { v2475 } else { v2282 });
        let v2478: f64 = (if self.scalar_v668 { v2476 } else { v2283 });
        let v2479: f64 = (if self.scalar_v668 { v2285 } else { v2341 });
        let v2480: f64 = (if self.scalar_v668 { v2286 } else { v2342 });
        let v2483: f64 = (v2479 / v672);
        let v2484: f64 = (v2480 / v672);
        let v2485: f64 = (v2481 / v672);
        let v2487: f64 = (if self.scalar_v668 { v2483 } else { v2294 });
        let v2488: f64 = (if self.scalar_v668 { v2484 } else { v2295 });
        let v2489: f64 = (if self.scalar_v668 { v2485 } else { v13 });
        let v2494: f64 = (self.scalar_v2393 * v2493);
        let v2499: f64 = (v2494 * v2498);
        let v2501: f64 = (v682 * self.scalar_v2491);
        let v2502: f64 = (v677 * v2499);
        let v2503: f64 = (v2501 + v2502);
        let v2507: f64 = (if self.scalar_v668 { v2503 } else { v13 });
        let v2509: f64 = (self.scalar_v523 + v2477);
        let v2510: f64 = (self.scalar_v2223 + v2478);
        let v2511: f64 = (if self.scalar_v668 { v2509 } else { v2299 });
        let v2512: f64 = (if self.scalar_v668 { v2510 } else { v2300 });
        let v2513: f64 = (self.scalar_v523 + v2487);
        let v2514: f64 = (self.scalar_v2224 + v2488);
        let v2516: f64 = (v2513 - v2511);
        let v2517: f64 = (v2514 - v2512);
        let v2518: f64 = (v2507 + v2517);
        let v2520: f64 = (v696 * v2516);
        let v2521: f64 = (v695 * v2233);
        let v2522: f64 = (v2520 + v2521);
        let v2523: f64 = (v696 * v2518);
        let v2524: f64 = (v695 * v2234);
        let v2525: f64 = (v2523 + v2524);
        let v2526: f64 = (v696 * v2489);
        let v2528: f64 = (v2522 / self.scalar_v520);
        let v2529: f64 = (v2525 / self.scalar_v520);
        let v2530: f64 = (v2526 / self.scalar_v520);
        let v2532: f64 = (self.scalar_v2316 + v2529);
        let v2534: f64 = (v137 * v2528);
        let v2535: f64 = (v137 * v2532);
        let v2536: f64 = (v137 * v2530);
        let v2538: f64 = (self.scalar_v2322 + v2535);
        let v2540: f64 = (if self.scalar_v668 { v2534 } else { v2325 });
        let v2541: f64 = (if self.scalar_v668 { v2538 } else { v2326 });
        let v2542: f64 = (if self.scalar_v668 { v2536 } else { v13 });
        let v2544: f64 = (if self.scalar_v668 { v2329 } else { v2473 });
        let v2545: f64 = (if self.scalar_v668 { v2330 } else { v2474 });
        let v2546: f64 = (v2544 / v703);
        let v2547: f64 = (v2545 / v703);
        let v2548: f64 = (if self.scalar_v668 { v2546 } else { v2335 });
        let v2549: f64 = (if self.scalar_v668 { v2547 } else { v2336 });
        let v2550: f64 = (if self.scalar_v668 { v2338 } else { v2479 });
        let v2551: f64 = (if self.scalar_v668 { v2339 } else { v2480 });
        let v2553: f64 = (if self.scalar_v668 { v13 } else { v2482 });
        let v2554: f64 = (v2550 / v706);
        let v2555: f64 = (v2551 / v706);
        let v2557: f64 = (v2553 / v706);
        let v2558: f64 = (if self.scalar_v668 { v2554 } else { v2349 });
        let v2559: f64 = (if self.scalar_v668 { v2555 } else { v2350 });
        let v2561: f64 = (if self.scalar_v668 { v2557 } else { v2352 });
        let v2562: f64 = (self.scalar_v2223 + v2548);
        let v2563: f64 = (self.scalar_v523 + v2549);
        let v2564: f64 = (if self.scalar_v668 { v2562 } else { v2355 });
        let v2565: f64 = (if self.scalar_v668 { v2563 } else { v2356 });
        let v2566: f64 = (self.scalar_v2241 + v2558);
        let v2567: f64 = (self.scalar_v523 + v2559);
        let v2569: f64 = (v2566 - v2564);
        let v2570: f64 = (v2567 - v2565);
        let v2571: f64 = (v712 * v2238);
        let v2572: f64 = (v645 * v2569);
        let v2573: f64 = (v2571 + v2572);
        let v2574: f64 = (v712 * v2239);
        let v2575: f64 = (v645 * v2570);
        let v2576: f64 = (v2574 + v2575);
        let v2578: f64 = (v645 * v2561);
        let v2579: f64 = (v2573 / self.scalar_v541);
        let v2580: f64 = (v2576 / self.scalar_v541);
        let v2582: f64 = (v2578 / self.scalar_v541);
        let v2583: f64 = (self.scalar_v2316 + v2579);
        let v2585: f64 = (v138 * v2583);
        let v2586: f64 = (v138 * v2580);
        let v2588: f64 = (v138 * v2582);
        let v2589: f64 = (self.scalar_v2380 + v2585);
        let v2591: f64 = (if self.scalar_v668 { v2589 } else { v2383 });
        let v2592: f64 = (if self.scalar_v668 { v2586 } else { v2384 });
        let v2594: f64 = (if self.scalar_v668 { v2588 } else { v2386 });
        let v2595: f64 = (if self.scalar_v668 { v13 } else { v2457 });
        let v2596: f64 = (if self.scalar_v668 { v13 } else { v2458 });
        let v2597: f64 = (if self.scalar_v668 { v13 } else { v2459 });
        let v2598: f64 = (if self.scalar_v668 { v13 } else { v2470 });
        let v2599: f64 = (if self.scalar_v668 { v13 } else { v2471 });
        let v2600: f64 = (if self.scalar_v668 { v13 } else { v2472 });
        let v2601: f64 = (-v2079);
        let v2602: f64 = (-v2080);
        let v2603: f64 = (-v2081);
        let v2604: f64 = (-v2082);
        let v2605: f64 = (-v2083);
        let v2606: f64 = ddt_scale;
        let v2607: f64 = (v2591 * v2606);
        let v2608: f64 = (v2592 * v2606);
        let v2609: f64 = (v2593 * v2606);
        let v2610: f64 = (v2594 * v2606);
        let v2611: f64 = (if self.scalar_v723 { v2607 } else { v13 });
        let v2612: f64 = (if self.scalar_v723 { v2608 } else { v13 });
        let v2613: f64 = (if self.scalar_v723 { v2609 } else { v13 });
        let v2614: f64 = (if self.scalar_v723 { v2610 } else { v13 });
        let v2615: f64 = (v2540 * v2606);
        let v2616: f64 = (v2541 * v2606);
        let v2617: f64 = (v2542 * v2606);
        let v2618: f64 = (v2543 * v2606);
        let v2619: f64 = (if self.scalar_v723 { v2615 } else { v13 });
        let v2620: f64 = (if self.scalar_v723 { v2616 } else { v13 });
        let v2621: f64 = (if self.scalar_v723 { v2617 } else { v13 });
        let v2622: f64 = (if self.scalar_v723 { v2618 } else { v13 });
        let v2623: f64 = (-v722);
        let v2624: f64 = (v5 * v2598);
        let v2625: f64 = (v2623 + v2624);
        let v2626: f64 = (v5 * v2599);
        let v2627: f64 = (v5 * v2600);
        let v2628: f64 = (v722 + v2627);
        let v2629: f64 = (v2606 * v2625);
        let v2630: f64 = (v2606 * v2626);
        let v2631: f64 = (v2606 * v2628);
        let v2632: f64 = (if self.scalar_v783 { v2629 } else { v13 });
        let v2633: f64 = (if self.scalar_v783 { v2630 } else { v13 });
        let v2634: f64 = (if self.scalar_v783 { v2631 } else { v13 });
        let v2635: f64 = (v9 * v2595);
        let v2636: f64 = (-v720);
        let v2637: f64 = (v9 * v2596);
        let v2638: f64 = (v2636 + v2637);
        let v2639: f64 = (v9 * v2597);
        let v2640: f64 = (v720 + v2639);
        let v2641: f64 = (v2606 * v2635);
        let v2642: f64 = (v2606 * v2638);
        let v2643: f64 = (v2606 * v2640);
        let v2644: f64 = (if self.scalar_v783 { v2641 } else { v13 });
        let v2645: f64 = (if self.scalar_v783 { v2642 } else { v13 });
        let v2646: f64 = (if self.scalar_v783 { v2643 } else { v13 });
        let v2649: f64 = (-v140);
        let v2650: f64 = -1e-12;
        let v2651: f64 = (v800 * v2131);
        let v2652: f64 = (v800 * v2132);
        let v2653: f64 = (v800 * v2133);
        let v2654: f64 = (v800 * v2134);
        let v2655: f64 = (v800 * v2135);
        let v2656: f64 = (self.scalar_v724 * v2606);
        let v2657: f64 = (v443 + v2656);
        let v2658: f64 = (if self.scalar_v725 { v2651 } else { v13 });
        let v2659: f64 = (if self.scalar_v725 { v2652 } else { v13 });
        let v2660: f64 = (if self.scalar_v725 { v2653 } else { v13 });
        let v2661: f64 = (if self.scalar_v725 { v2654 } else { v13 });
        let v2662: f64 = (if self.scalar_v725 { v2655 } else { v13 });
        let v2663: f64 = (if self.scalar_v725 { v2657 } else { v13 });
        let v2664: f64 = (v44 / v141);
        let v2665: f64 = (v456 / v141);
        let v2666: f64 = (if self.scalar_v728 { v2664 } else { v13 });
        let v2667: f64 = (if self.scalar_v728 { v2665 } else { v13 });
        let v2668: f64 = (-v142);
        let v2669: f64 = (v2606 * v2668);
        let v2670: f64 = (v142 * v2606);
        let v2671: f64 = (if self.scalar_v728 { v2669 } else { v13 });
        let v2672: f64 = (if self.scalar_v728 { v2670 } else { v13 });
        let v2687: f64 = (v847 * v2146);
        let v2688: f64 = (v847 * v2147);
        let v2689: f64 = (v847 * v2148);
        let v2690: f64 = (v847 * v2149);
        let v2691: f64 = (v847 * v2150);
        let v2692: f64 = (if self.scalar_v737 { v2687 } else { v13 });
        let v2693: f64 = (if self.scalar_v737 { v2688 } else { v13 });
        let v2694: f64 = (if self.scalar_v737 { v2689 } else { v13 });
        let v2695: f64 = (if self.scalar_v737 { v2690 } else { v13 });
        let v2696: f64 = (if self.scalar_v737 { v2691 } else { v13 });
        let v2697: f64 = (if self.scalar_v737 { v452 } else { v13 });
        let v2698: f64 = (v857 * v2146);
        let v2699: f64 = (v857 * v2147);
        let v2700: f64 = (v857 * v2148);
        let v2701: f64 = (v857 * v2149);
        let v2702: f64 = (v857 * v2150);
        let v2703: f64 = (if self.scalar_v740 { v2698 } else { v13 });
        let v2704: f64 = (if self.scalar_v740 { v2699 } else { v13 });
        let v2705: f64 = (if self.scalar_v740 { v2700 } else { v13 });
        let v2706: f64 = (if self.scalar_v740 { v2701 } else { v13 });
        let v2707: f64 = (if self.scalar_v740 { v2702 } else { v13 });
        let v2708: f64 = (if self.scalar_v740 { v453 } else { v13 });
        let v2710: f64 = (if self.scalar_v747 { v766 } else { v13 });
        let v2711: f64 = (if self.scalar_v747 { v762 } else { v13 });
        let v2712: f64 = (v882 * v2606);
        let v2713: f64 = (if self.scalar_v747 { v2712 } else { v13 });
        let v2714: f64 = (v44 / v49);
        let v2715: f64 = (if self.scalar_v771 { v2714 } else { v13 });
        let v2716: f64 = (self.scalar_v899 * v2606);
        let v2717: f64 = (if self.scalar_v771 { v2716 } else { v13 });

        let d772_dn4: f64 = v2601;
        let d772_dn5: f64 = v2602;
        let d772_dn8: f64 = v2603;
        let d772_dn10: f64 = v2604;
        let d772_dn12: f64 = v2605;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(15),
            None,
            multiplicity * (v772),
            [4, 5, 8, 10, 12],
            [d772_dn4, d772_dn5, d772_dn8, d772_dn10, d772_dn12],
            [],
            [],
            multiplicity,
        );
        let d775_dn15: f64 = self.scalar_v773;
        let v775_ddt: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, v775);
        stamper.stamp_current_node1_local(
            Some(15),
            None,
            multiplicity * (v775_ddt),
            15,
            multiplicity * (((d775_dn15) * ddt_scale)),
        );
        let d12_dn16: f64 = v44;
        stamper.stamp_current_node1_local(
            Some(16),
            None,
            multiplicity * (v12),
            16,
            multiplicity * (d12_dn16),
        );
        let d778_db0: f64 = self.scalar_v776;
        let v778_ddt: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, v778);
        stamper.stamp_potential_branch_local(
            Some(15),
            Some(16),
            0,
            multiplicity,
        );
        stamper.stamp_potential_branch1_local(
            0,
            v778_ddt,
            0,
            ((d778_db0) * ddt_scale),
        );
        let d12_dn16: f64 = v44;
        stamper.stamp_current_node1_local(
            Some(5),
            Some(8),
            multiplicity * (v12),
            16,
            multiplicity * (d12_dn16),
        );
        let d510_dn4: f64 = v2198;
        let d510_dn5: f64 = v2199;
        let d510_dn8: f64 = v2200;
        let d510_dn10: f64 = v2201;
        let d510_dn11: f64 = v2202;
        let d510_dn12: f64 = self.scalar_v2203;
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
        let d519_dn4: f64 = v2198;
        let d519_dn5: f64 = v2219;
        let d519_dn8: f64 = v2220;
        let d519_dn10: f64 = v2221;
        let d519_dn12: f64 = self.scalar_v2203;
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
        let d780_dn5: f64 = v2611;
        let d780_dn8: f64 = v2612;
        let d780_dn10: f64 = v2613;
        let d780_dn11: f64 = v2614;
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(10),
            Some(5),
            multiplicity * (v780),
            [5, 8, 10, 11],
            [d780_dn5, d780_dn8, d780_dn10, d780_dn11],
            [],
            [],
            multiplicity,
        );
        let d782_dn5: f64 = v2619;
        let d782_dn8: f64 = v2620;
        let d782_dn10: f64 = v2621;
        let d782_dn11: f64 = v2622;
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(11),
            Some(8),
            multiplicity * (v782),
            [5, 8, 10, 11],
            [d782_dn5, d782_dn8, d782_dn10, d782_dn11],
            [],
            [],
            multiplicity,
        );
        let d786_dn5: f64 = v2632;
        let d786_dn8: f64 = v2633;
        let d786_dn10: f64 = v2634;
        stamper.stamp_current_node3_local(
            Some(10),
            Some(5),
            multiplicity * (v786),
            5,
            multiplicity * (d786_dn5),
            8,
            multiplicity * (d786_dn8),
            10,
            multiplicity * (d786_dn10),
        );
        let d789_dn5: f64 = v2644;
        let d789_dn8: f64 = v2645;
        let d789_dn11: f64 = v2646;
        stamper.stamp_current_node3_local(
            Some(11),
            Some(8),
            multiplicity * (v789),
            5,
            multiplicity * (d789_dn5),
            8,
            multiplicity * (d789_dn8),
            11,
            multiplicity * (d789_dn11),
        );
        let d793_dn5: f64 = self.scalar_v2647;
        let d793_dn7: f64 = self.scalar_v790;
        let v793_ddt: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 7, v793);
        stamper.stamp_current_node2_local(
            Some(7),
            Some(5),
            multiplicity * (v793_ddt),
            5,
            multiplicity * (((d793_dn5) * ddt_scale)),
            7,
            multiplicity * (((d793_dn7) * ddt_scale)),
        );
        let d795_dn5: f64 = self.scalar_v794;
        let d795_dn8: f64 = self.scalar_v2648;
        let v795_ddt: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 8, v795);
        stamper.stamp_current_node2_local(
            Some(5),
            Some(8),
            multiplicity * (v795_ddt),
            5,
            multiplicity * (((d795_dn5) * ddt_scale)),
            8,
            multiplicity * (((d795_dn8) * ddt_scale)),
        );
        let d798_dn4: f64 = v2649;
        let d798_dn6: f64 = v140;
        let v798_ddt: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 9, v798);
        stamper.stamp_current_node2_local(
            Some(6),
            Some(4),
            multiplicity * (v798_ddt),
            4,
            multiplicity * (((d798_dn4) * ddt_scale)),
            6,
            multiplicity * (((d798_dn6) * ddt_scale)),
        );
        let d799_dn4: f64 = v2650;
        let d799_dn6: f64 = v167;
        stamper.stamp_current_node2_local(
            Some(6),
            Some(4),
            multiplicity * (v799),
            4,
            multiplicity * (d799_dn4),
            6,
            multiplicity * (d799_dn6),
        );
        let d805_dn4: f64 = v2658;
        let d805_dn5: f64 = v2659;
        let d805_dn8: f64 = v2660;
        let d805_dn10: f64 = v2661;
        let d805_dn12: f64 = v2662;
        let d805_db1: f64 = v2663;
        stamper.stamp_potential_branch_local(
            Some(4),
            Some(8),
            1,
            multiplicity,
        );
        stamper.stamp_potential_sparse_local::<5, 1>(
            1,
            v805,
            [4, 5, 8, 10, 12],
            [d805_dn4, d805_dn5, d805_dn8, d805_dn10, d805_dn12],
            [1],
            [d805_db1],
        );
        stamper.stamp_potential_branch_local(
            Some(4),
            Some(8),
            2,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            2,
            self.scalar_v807,
        );
        let d810_dn11: f64 = v2666;
        let d810_dn12: f64 = v2667;
        stamper.stamp_current_node2_local(
            Some(11),
            Some(12),
            multiplicity * (v810),
            11,
            multiplicity * (d810_dn11),
            12,
            multiplicity * (d810_dn12),
        );
        let d813_dn8: f64 = v2671;
        let d813_dn12: f64 = v2672;
        stamper.stamp_current_node2_local(
            Some(12),
            Some(8),
            multiplicity * (v813),
            8,
            multiplicity * (d813_dn8),
            12,
            multiplicity * (d813_dn12),
        );
        stamper.stamp_potential_branch_local(
            Some(11),
            Some(8),
            3,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            3,
            self.scalar_v815,
        );
        let d819_dn11: f64 = self.scalar_v816;
        let d819_dn14: f64 = self.scalar_v2673;
        let v819_ddt: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 11, v819);
        stamper.stamp_current_node2_local(
            Some(11),
            Some(14),
            multiplicity * (v819_ddt),
            11,
            multiplicity * (((d819_dn11) * ddt_scale)),
            14,
            multiplicity * (((d819_dn14) * ddt_scale)),
        );
        let d822_dn8: f64 = self.scalar_v2676;
        let d822_dn14: f64 = self.scalar_v2677;
        stamper.stamp_current_node2_local(
            Some(14),
            Some(8),
            multiplicity * (v822),
            8,
            multiplicity * (d822_dn8),
            14,
            multiplicity * (d822_dn14),
        );
        stamper.stamp_potential_branch_local(
            Some(14),
            Some(8),
            4,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            4,
            self.scalar_v824,
        );
        let d828_dn10: f64 = self.scalar_v2680;
        let d828_dn13: f64 = self.scalar_v2681;
        stamper.stamp_current_node2_local(
            Some(13),
            Some(10),
            multiplicity * (v828),
            10,
            multiplicity * (d828_dn10),
            13,
            multiplicity * (d828_dn13),
        );
        stamper.stamp_potential_branch_local(
            Some(13),
            Some(10),
            5,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            5,
            self.scalar_v830,
        );
        stamper.stamp_current_const_local(
            Some(13),
            Some(10),
            multiplicity * (self.scalar_v831),
        );
        let d834_dn11: f64 = self.scalar_v2684;
        let d834_dn13: f64 = self.scalar_v2685;
        stamper.stamp_current_node2_local(
            Some(13),
            Some(11),
            multiplicity * (v834),
            11,
            multiplicity * (d834_dn11),
            13,
            multiplicity * (d834_dn13),
        );
        stamper.stamp_potential_branch_local(
            Some(13),
            Some(11),
            6,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            6,
            self.scalar_v836,
        );
        let d839_db7: f64 = self.scalar_v2686;
        stamper.stamp_potential_branch_local(
            Some(7),
            Some(13),
            7,
            multiplicity,
        );
        stamper.stamp_potential_branch1_local(
            7,
            v839,
            7,
            d839_db7,
        );
        stamper.stamp_potential_branch_local(
            Some(7),
            Some(13),
            8,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            8,
            self.scalar_v841,
        );
        stamper.stamp_potential_branch_local(
            Some(7),
            Some(13),
            9,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            9,
            self.scalar_v843,
        );
        let d846_db10: f64 = self.scalar_v844;
        let v846_ddt: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 12, v846);
        stamper.stamp_potential_branch_local(
            Some(1),
            Some(7),
            10,
            multiplicity,
        );
        stamper.stamp_potential_branch1_local(
            10,
            v846_ddt,
            10,
            ((d846_db10) * ddt_scale),
        );
        let d849_dn4: f64 = v2692;
        let d849_dn5: f64 = v2693;
        let d849_dn8: f64 = v2694;
        let d849_dn10: f64 = v2695;
        let d849_dn12: f64 = v2696;
        let d849_db11: f64 = v2697;
        stamper.stamp_potential_branch_local(
            Some(8),
            Some(9),
            11,
            multiplicity,
        );
        stamper.stamp_potential_sparse_local::<5, 1>(
            11,
            v849,
            [4, 5, 8, 10, 12],
            [d849_dn4, d849_dn5, d849_dn8, d849_dn10, d849_dn12],
            [11],
            [d849_db11],
        );
        stamper.stamp_potential_branch_local(
            Some(8),
            Some(9),
            12,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            12,
            self.scalar_v851,
        );
        stamper.stamp_potential_branch_local(
            Some(8),
            Some(9),
            13,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            13,
            self.scalar_v853,
        );
        let d856_db14: f64 = self.scalar_v854;
        let v856_ddt: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 13, v856);
        stamper.stamp_potential_branch_local(
            Some(9),
            Some(2),
            14,
            multiplicity,
        );
        stamper.stamp_potential_branch1_local(
            14,
            v856_ddt,
            14,
            ((d856_db14) * ddt_scale),
        );
        let d859_dn4: f64 = v2703;
        let d859_dn5: f64 = v2704;
        let d859_dn8: f64 = v2705;
        let d859_dn10: f64 = v2706;
        let d859_dn12: f64 = v2707;
        let d859_db15: f64 = v2708;
        stamper.stamp_potential_branch_local(
            Some(5),
            Some(6),
            15,
            multiplicity,
        );
        stamper.stamp_potential_sparse_local::<5, 1>(
            15,
            v859,
            [4, 5, 8, 10, 12],
            [d859_dn4, d859_dn5, d859_dn8, d859_dn10, d859_dn12],
            [15],
            [d859_db15],
        );
        stamper.stamp_potential_branch_local(
            Some(5),
            Some(6),
            16,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            16,
            self.scalar_v861,
        );
        stamper.stamp_potential_branch_local(
            Some(5),
            Some(6),
            17,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            17,
            self.scalar_v863,
        );
        let d866_db18: f64 = self.scalar_v864;
        let v866_ddt: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 14, v866);
        stamper.stamp_potential_branch_local(
            Some(6),
            Some(0),
            18,
            multiplicity,
        );
        stamper.stamp_potential_branch1_local(
            18,
            v866_ddt,
            18,
            ((d866_db18) * ddt_scale),
        );
        stamper.stamp_current_const_local(
            Some(0),
            Some(2),
            multiplicity * (v867),
        );
        stamper.stamp_current_const_local(
            Some(14),
            Some(2),
            multiplicity * (v167),
        );
        let d870_dn2: f64 = v2650;
        let d870_dn12: f64 = v167;
        stamper.stamp_current_node2_local(
            Some(12),
            Some(2),
            multiplicity * (v870),
            2,
            multiplicity * (d870_dn2),
            12,
            multiplicity * (d870_dn12),
        );
        stamper.stamp_current_const_local(
            Some(5),
            Some(8),
            multiplicity * (self.scalar_v872),
        );
        stamper.stamp_current_const_local(
            Some(5),
            Some(8),
            multiplicity * (self.scalar_v872),
        );
        stamper.stamp_current_const_local(
            Some(17),
            None,
            multiplicity * (self.scalar_v873),
        );
        let d875_dn17: f64 = self.scalar_v2709;
        stamper.stamp_current_node1_local(
            Some(17),
            None,
            multiplicity * (v875),
            17,
            multiplicity * (d875_dn17),
        );
        stamper.stamp_current_const_local(
            Some(18),
            None,
            multiplicity * (self.scalar_v873),
        );
        let d877_dn18: f64 = self.scalar_v2709;
        stamper.stamp_current_node1_local(
            Some(18),
            None,
            multiplicity * (v877),
            18,
            multiplicity * (d877_dn18),
        );
        let d875_dn17: f64 = self.scalar_v2709;
        stamper.stamp_current_node1_local(
            Some(7),
            Some(8),
            multiplicity * (v875),
            17,
            multiplicity * (d875_dn17),
        );
        let d881_dn17: f64 = v2710;
        let d881_dn18: f64 = v2711;
        stamper.stamp_current_node2_local(
            Some(7),
            Some(5),
            multiplicity * (v881),
            17,
            multiplicity * (d881_dn17),
            18,
            multiplicity * (d881_dn18),
        );
        let d885_dn17: f64 = v2713;
        stamper.stamp_current_node1_local(
            Some(7),
            Some(5),
            multiplicity * (v885),
            17,
            multiplicity * (d885_dn17),
        );
        stamper.stamp_current_const_local(
            Some(5),
            Some(8),
            multiplicity * (self.scalar_v873),
        );
        stamper.stamp_current_const_local(
            Some(7),
            Some(8),
            multiplicity * (self.scalar_v873),
        );
        stamper.stamp_current_const_local(
            Some(5),
            Some(8),
            multiplicity * (self.scalar_v873),
        );
        stamper.stamp_current_const_local(
            Some(5),
            Some(8),
            multiplicity * (self.scalar_v887),
        );
        let d874_dn17: f64 = v44;
        stamper.stamp_current_node1_local(
            Some(17),
            None,
            multiplicity * (v874),
            17,
            multiplicity * (d874_dn17),
        );
        let d876_dn18: f64 = v44;
        stamper.stamp_current_node1_local(
            Some(18),
            None,
            multiplicity * (v876),
            18,
            multiplicity * (d876_dn18),
        );
        stamper.stamp_current_const_local(
            Some(11),
            Some(8),
            multiplicity * (self.scalar_v831),
        );
        stamper.stamp_current_const_local(
            Some(10),
            Some(5),
            multiplicity * (self.scalar_v831),
        );
        stamper.stamp_current_const_local(
            Some(11),
            Some(8),
            multiplicity * (self.scalar_v889),
        );
        stamper.stamp_current_const_local(
            Some(10),
            Some(5),
            multiplicity * (self.scalar_v889),
        );
        stamper.stamp_current_const_local(
            Some(3),
            None,
            multiplicity * (v896),
        );
        let d898_dn3: f64 = v2715;
        stamper.stamp_current_node1_local(
            Some(3),
            None,
            multiplicity * (v898),
            3,
            multiplicity * (d898_dn3),
        );
        let d902_dn3: f64 = v2717;
        stamper.stamp_current_node1_local(
            Some(3),
            None,
            multiplicity * (v902),
            3,
            multiplicity * (d902_dn3),
        );
        let d905_dn3: f64 = self.scalar_v2718;
        stamper.stamp_current_node1_local(
            Some(3),
            None,
            multiplicity * (v905),
            3,
            multiplicity * (d905_dn3),
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
        let v2284: f64 = ((v525) as f64).sinh();
        let v2287: f64 = (self.scalar_v520 * v2284);
        let v2290: f64 = (if self.scalar_v569 { v2287 } else { v13 });
        let v2293: f64 = (v2290 / v578);
        let v2296: f64 = (if self.scalar_v569 { v2293 } else { v13 });
        let v2303: f64 = (self.scalar_v520 + v2296);
        let v2312: f64 = (v571 * v2303);
        let v2315: f64 = (v2312 / self.scalar_v520);
        let v2318: f64 = (self.scalar_v562 + v2315);
        let v2321: f64 = (v137 * v2318);
        let v2324: f64 = (self.scalar_v551 + v2321);
        let v2327: f64 = (if self.scalar_v569 { v2324 } else { v13 });
        let v614: f64 = v2327;
        let v615: f64 = (if self.scalar_v569 { v614 } else { v560 });
        let v2337: f64 = ((v544) as f64).sinh();
        let v2340: f64 = (self.scalar_v541 * v2337);
        let v2343: f64 = (if self.scalar_v569 { v2340 } else { v13 });
        let v2347: f64 = (v2343 / v599);
        let v2351: f64 = (if self.scalar_v569 { v2347 } else { v13 });
        let v2359: f64 = (self.scalar_v541 + v2351);
        let v2368: f64 = (v540 * v2359);
        let v2372: f64 = (v2368 / self.scalar_v541);
        let v2375: f64 = (self.scalar_v562 + v2372);
        let v2378: f64 = (v138 * v2375);
        let v2382: f64 = (self.scalar_v553 + v2378);
        let v2385: f64 = (if self.scalar_v569 { v2382 } else { v13 });
        let v616: f64 = v2385;
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
        let v2504: f64 = (self.scalar_v654 * v682);
        let v2492: f64 = f64::powf(v678, v44);
        let v2493: f64 = (v222 * v2492);
        let v2495: f64 = (self.scalar_v2394 * v2493);
        let v2497: f64 = f64::powf(v680, self.scalar_v2496);
        let v2498: f64 = (self.scalar_v681 * v2497);
        let v2500: f64 = (v2495 * v2498);
        let v2505: f64 = (v677 * v2500);
        let v2506: f64 = (v2504 + v2505);
        let v2508: f64 = (if self.scalar_v668 { v2506 } else { v13 });
        let v2344: f64 = (if self.scalar_v569 { v13 } else { v2290 });
        let v2482: f64 = (if self.scalar_v668 { v2287 } else { v2344 });
        let v2486: f64 = (v2482 / v672);
        let v2490: f64 = (if self.scalar_v668 { v2486 } else { v2296 });
        let v2515: f64 = (self.scalar_v520 + v2490);
        let v2519: f64 = (v2508 + v2515);
        let v2527: f64 = (v696 * v2519);
        let v2531: f64 = (v2527 / self.scalar_v520);
        let v2533: f64 = (self.scalar_v562 + v2531);
        let v2537: f64 = (v137 * v2533);
        let v2539: f64 = (self.scalar_v551 + v2537);
        let v2543: f64 = (if self.scalar_v668 { v2539 } else { v2327 });
        let v719: f64 = v2543;
        let v720: f64 = (if self.scalar_v668 { v719 } else { v660 });
        let v2481: f64 = (if self.scalar_v668 { v13 } else { v2343 });
        let v2552: f64 = (if self.scalar_v668 { v2340 } else { v2481 });
        let v2556: f64 = (v2552 / v706);
        let v2560: f64 = (if self.scalar_v668 { v2556 } else { v2351 });
        let v2568: f64 = (self.scalar_v541 + v2560);
        let v2577: f64 = (v645 * v2568);
        let v2581: f64 = (v2577 / self.scalar_v541);
        let v2584: f64 = (self.scalar_v562 + v2581);
        let v2587: f64 = (v138 * v2584);
        let v2590: f64 = (self.scalar_v553 + v2587);
        let v2593: f64 = (if self.scalar_v668 { v2590 } else { v2385 });
        let v721: f64 = v2593;
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
        let v774: f64 = ctx.node_voltage(nodes[15]);
        let v775: f64 = (self.scalar_v773 * v774);
        let v777: f64 = ctx.branch_current(branches[0]);
        let v778: f64 = (self.scalar_v776 * v777);
        let v779: f64 = 0.0;
        let v780: f64 = (if self.scalar_v723 { v779 } else { v13 });
        let v781: f64 = 0.0;
        let v782: f64 = (if self.scalar_v723 { v781 } else { v13 });
        let v784: f64 = (v5 * v722);
        let v785: f64 = 0.0;
        let v786: f64 = (if self.scalar_v783 { v785 } else { v13 });
        let v787: f64 = (v9 * v720);
        let v788: f64 = 0.0;
        let v789: f64 = (if self.scalar_v783 { v788 } else { v13 });
        let v791: f64 = ctx.node_voltage(nodes[7]);
        let v792: f64 = (v791 - v4);
        let v793: f64 = (self.scalar_v790 * v792);
        let v795: f64 = (v7 * self.scalar_v794);
        let v796: f64 = ctx.node_voltage(nodes[6]);
        let v797: f64 = (v796 - v10);
        let v798: f64 = (v140 * v797);
        let v800: f64 = ctx.branch_current(branches[1]);
        let v801: f64 = (v443 * v800);
        let v802: f64 = (self.scalar_v724 * v800);
        let v803: f64 = 0.0;
        let v804: f64 = (v801 + v803);
        let v805: f64 = (if self.scalar_v725 { v804 } else { v13 });
        let v811: f64 = (v2 * v142);
        let v812: f64 = 0.0;
        let v813: f64 = (if self.scalar_v728 { v812 } else { v13 });
        let v817: f64 = ctx.node_voltage(nodes[14]);
        let v818: f64 = (v8 - v817);
        let v819: f64 = (self.scalar_v816 * v818);
        let v845: f64 = ctx.branch_current(branches[10]);
        let v846: f64 = (self.scalar_v844 * v845);
        let v855: f64 = ctx.branch_current(branches[14]);
        let v856: f64 = (self.scalar_v854 * v855);
        let v865: f64 = ctx.branch_current(branches[18]);
        let v866: f64 = (self.scalar_v864 * v865);
        let v874: f64 = ctx.node_voltage(nodes[17]);
        let v882: f64 = (-v768);
        let v883: f64 = (v874 * v882);
        let v884: f64 = 0.0;
        let v885: f64 = (if self.scalar_v747 { v884 } else { v13 });
        let v900: f64 = (v32 * self.scalar_v899);
        let v901: f64 = 0.0;
        let v902: f64 = (if self.scalar_v771 { v901 } else { v13 });
        let v907: f64 = ((v161) as f64).sinh();
        let v908: f64 = (self.scalar_v160 * v907);
        let v909: f64 = (self.scalar_v906 * v907);
        let v911: f64 = (v162 * v908);
        let v912: f64 = (v911 + v911);
        let v913: f64 = (v162 * v909);
        let v914: f64 = (v913 + v913);
        let v915: f64 = (self.scalar_v166 * v912);
        let v916: f64 = (-v915);
        let v917: f64 = (v169 * v169);
        let v918: f64 = (v916 / v917);
        let v919: f64 = (self.scalar_v166 * v914);
        let v920: f64 = (-v919);
        let v921: f64 = (v920 / v917);
        let v922: f64 = (self.scalar_v165 * v918);
        let v923: f64 = (self.scalar_v165 * v921);
        let v924: f64 = (v175 * v922);
        let v925: f64 = (v175 * v923);
        let v927: f64 = (v186 * v186);
        let v928: f64 = (v44 - v927);
        let v929: f64 = (self.scalar_v184 * v928);
        let v930: f64 = (self.scalar_v926 * v928);
        let v931: f64 = (self.scalar_v182 * v929);
        let v932: f64 = (self.scalar_v182 * v930);
        let v933: f64 = (v932 - self.scalar_v910);
        let v935: f64 = (v192 + v192);
        let v936: f64 = (-v192);
        let v937: f64 = (v191 * self.scalar_v934);
        let v938: f64 = (v936 + v937);
        let v939: f64 = (v931 - v935);
        let v940: f64 = (-v938);
        let v941: f64 = (v196 * self.scalar_v910);
        let v942: f64 = (v196 * v939);
        let v943: f64 = (v196 * v933);
        let v944: f64 = (v196 * v940);
        let v945: f64 = (-v941);
        let v946: f64 = (-v942);
        let v947: f64 = (v456 - v943);
        let v948: f64 = (-v944);
        let v949: f64 = (v198 * v945);
        let v950: f64 = (v949 + v949);
        let v951: f64 = (v198 * v946);
        let v952: f64 = (v951 + v951);
        let v953: f64 = (v198 * v947);
        let v954: f64 = (v953 + v953);
        let v955: f64 = (v198 * v948);
        let v956: f64 = (v955 + v955);
        let v957: f64 = (v198 + v198);
        let v958: f64 = (v176 * v945);
        let v959: f64 = (v198 * v924);
        let v960: f64 = (v176 * v946);
        let v961: f64 = (v959 + v960);
        let v962: f64 = (v198 * v925);
        let v963: f64 = (v176 * v947);
        let v964: f64 = (v962 + v963);
        let v965: f64 = (v176 * v948);
        let v966: f64 = (self.scalar_v201 * v950);
        let v967: f64 = (self.scalar_v201 * v952);
        let v968: f64 = (self.scalar_v201 * v954);
        let v969: f64 = (self.scalar_v201 * v956);
        let v970: f64 = (self.scalar_v201 * v957);
        let v971: f64 = (v958 + v966);
        let v972: f64 = (v961 + v967);
        let v973: f64 = (v964 + v968);
        let v974: f64 = (v965 + v969);
        let v975: f64 = (v176 + v970);
        let v976: f64 = (v181 * v945);
        let v977: f64 = (v181 * v946);
        let v978: f64 = (v181 * v947);
        let v979: f64 = (v181 * v948);
        let v980: f64 = (v204 * v950);
        let v981: f64 = (v199 * v976);
        let v982: f64 = (v980 + v981);
        let v983: f64 = (v204 * v952);
        let v984: f64 = (v199 * v977);
        let v985: f64 = (v983 + v984);
        let v986: f64 = (v204 * v954);
        let v987: f64 = (v199 * v978);
        let v988: f64 = (v986 + v987);
        let v989: f64 = (v204 * v956);
        let v990: f64 = (v199 * v979);
        let v991: f64 = (v989 + v990);
        let v992: f64 = (v204 * v957);
        let v993: f64 = (v181 * v199);
        let v994: f64 = (v992 + v993);
        let v995: f64 = (v971 + v982);
        let v996: f64 = (v972 + v985);
        let v997: f64 = (v973 + v988);
        let v998: f64 = (v974 + v991);
        let v999: f64 = (v975 + v994);
        let v1000: f64 = (v207 * v207);
        let v1001: f64 = (v44 - v1000);
        let v1002: f64 = (v995 * v1001);
        let v1003: f64 = (v996 * v1001);
        let v1004: f64 = (v997 * v1001);
        let v1005: f64 = (v998 * v1001);
        let v1006: f64 = (v999 * v1001);
        let v1007: f64 = { let limexp_arg = v206; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v1008: f64 = (v995 * v1007);
        let v1009: f64 = (v996 * v1007);
        let v1010: f64 = (v997 * v1007);
        let v1011: f64 = (v998 * v1007);
        let v1012: f64 = (v999 * v1007);
        let v1013: f64 = (-v995);
        let v1014: f64 = (-v996);
        let v1015: f64 = (-v997);
        let v1016: f64 = (-v998);
        let v1017: f64 = (-v999);
        let v1018: f64 = { let limexp_arg = v210; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v1019: f64 = (v1013 * v1018);
        let v1020: f64 = (v1014 * v1018);
        let v1021: f64 = (v1015 * v1018);
        let v1022: f64 = (v1016 * v1018);
        let v1023: f64 = (v1017 * v1018);
        let v1024: f64 = (v1008 - v1019);
        let v1025: f64 = (v1009 - v1020);
        let v1026: f64 = (v1010 - v1021);
        let v1027: f64 = (v1011 - v1022);
        let v1028: f64 = (v1012 - v1023);
        let v1029: f64 = (v152 * v1024);
        let v1030: f64 = (v152 * v1025);
        let v1031: f64 = (v152 * v1026);
        let v1032: f64 = (v152 * v1027);
        let v1033: f64 = (v152 * v1028);
        let v1034: f64 = (v214 * v214);
        let v1035: f64 = (v44 - v1034);
        let v1036: f64 = (v1029 * v1035);
        let v1037: f64 = (v1030 * v1035);
        let v1038: f64 = (v1031 * v1035);
        let v1039: f64 = (v1032 * v1035);
        let v1040: f64 = (v1033 * v1035);
        let v1103: f64 = (v456 - v942);
        let v1104: f64 = (-v943);
        let v1105: f64 = (v44 - v944);
        let v1106: f64 = (if self.scalar_v237 { v945 } else { v13 });
        let v1107: f64 = (if self.scalar_v237 { v1103 } else { v908 });
        let v1108: f64 = (if self.scalar_v237 { v1104 } else { v909 });
        let v1109: f64 = (if self.scalar_v237 { v1105 } else { v13 });
        let v1110: f64 = (v239 * v1106);
        let v1111: f64 = (v1110 + v1110);
        let v1112: f64 = (v239 * v1107);
        let v1113: f64 = (v1112 + v1112);
        let v1114: f64 = (v239 * v1108);
        let v1115: f64 = (v1114 + v1114);
        let v1116: f64 = (v239 * v1109);
        let v1117: f64 = (v1116 + v1116);
        let v1118: f64 = (if self.scalar_v237 { v1111 } else { v945 });
        let v1119: f64 = (if self.scalar_v237 { v1113 } else { v946 });
        let v1120: f64 = (if self.scalar_v237 { v1115 } else { v947 });
        let v1121: f64 = (if self.scalar_v237 { v1117 } else { v948 });
        let v1354: f64 = (if self.scalar_v290 { v945 } else { v1106 });
        let v1355: f64 = (if self.scalar_v290 { v946 } else { v1107 });
        let v1356: f64 = (if self.scalar_v290 { v947 } else { v1108 });
        let v1357: f64 = (if self.scalar_v290 { v948 } else { v1109 });
        let v1359: f64 = (v291 * v1354);
        let v1360: f64 = (v1359 + v1359);
        let v1361: f64 = (v291 * v1355);
        let v1362: f64 = (v1361 + v1361);
        let v1363: f64 = (v291 * v1356);
        let v1364: f64 = (v1363 + v1363);
        let v1365: f64 = (v291 * v1357);
        let v1366: f64 = (v1365 + v1365);
        let v1367: f64 = (v291 * self.scalar_v1358);
        let v1368: f64 = (v1367 + v1367);
        let v1369: f64 = (if self.scalar_v290 { v1360 } else { v1118 });
        let v1370: f64 = (if self.scalar_v290 { v1362 } else { v1119 });
        let v1371: f64 = (if self.scalar_v290 { v1364 } else { v1120 });
        let v1372: f64 = (if self.scalar_v290 { v1366 } else { v1121 });
        let v1373: f64 = (if self.scalar_v290 { v1368 } else { self.scalar_v1122 });
        let v1374: f64 = (self.scalar_v201 * v1369);
        let v1375: f64 = (self.scalar_v201 * v1370);
        let v1376: f64 = (self.scalar_v201 * v1371);
        let v1377: f64 = (self.scalar_v201 * v1372);
        let v1378: f64 = (self.scalar_v201 * v1373);
        let v1379: f64 = (v1354 + v1374);
        let v1380: f64 = (v1355 + v1375);
        let v1381: f64 = (v1356 + v1376);
        let v1382: f64 = (v1357 + v1377);
        let v1383: f64 = (self.scalar_v1358 + v1378);
        let v1384: f64 = (v181 * v1369);
        let v1385: f64 = (v181 * v1370);
        let v1386: f64 = (v181 * v1371);
        let v1387: f64 = (v181 * v1372);
        let v1388: f64 = (v181 * v1373);
        let v1389: f64 = (v296 * v1354);
        let v1390: f64 = (v291 * v1384);
        let v1391: f64 = (v1389 + v1390);
        let v1392: f64 = (v296 * v1355);
        let v1393: f64 = (v291 * v1385);
        let v1394: f64 = (v1392 + v1393);
        let v1395: f64 = (v296 * v1356);
        let v1396: f64 = (v291 * v1386);
        let v1397: f64 = (v1395 + v1396);
        let v1398: f64 = (v296 * v1357);
        let v1399: f64 = (v291 * v1387);
        let v1400: f64 = (v1398 + v1399);
        let v1401: f64 = (v296 * self.scalar_v1358);
        let v1402: f64 = (v291 * v1388);
        let v1403: f64 = (v1401 + v1402);
        let v1404: f64 = (v1379 + v1391);
        let v1405: f64 = (v1380 + v1394);
        let v1406: f64 = (v1381 + v1397);
        let v1407: f64 = (v1382 + v1400);
        let v1408: f64 = (v1383 + v1403);
        let v1409: f64 = (v176 * v1404);
        let v1410: f64 = (v298 * v924);
        let v1411: f64 = (v176 * v1405);
        let v1412: f64 = (v1410 + v1411);
        let v1413: f64 = (v298 * v925);
        let v1414: f64 = (v176 * v1406);
        let v1415: f64 = (v1413 + v1414);
        let v1416: f64 = (v176 * v1407);
        let v1417: f64 = (v176 * v1408);
        let v1418: f64 = (if self.scalar_v290 { v1409 } else { v995 });
        let v1419: f64 = (if self.scalar_v290 { v1412 } else { v996 });
        let v1420: f64 = (if self.scalar_v290 { v1415 } else { v997 });
        let v1421: f64 = (if self.scalar_v290 { v1416 } else { v998 });
        let v1422: f64 = (if self.scalar_v290 { v1417 } else { v999 });
        let v1423: f64 = { let limexp_arg = v300; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v1424: f64 = (v1418 * v1423);
        let v1425: f64 = (v1419 * v1423);
        let v1426: f64 = (v1420 * v1423);
        let v1427: f64 = (v1421 * v1423);
        let v1428: f64 = (v1422 * v1423);
        let v1429: f64 = (-v1418);
        let v1430: f64 = (-v1419);
        let v1431: f64 = (-v1420);
        let v1432: f64 = (-v1421);
        let v1433: f64 = (-v1422);
        let v1434: f64 = { let limexp_arg = v302; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v1435: f64 = (v1429 * v1434);
        let v1436: f64 = (v1430 * v1434);
        let v1437: f64 = (v1431 * v1434);
        let v1438: f64 = (v1432 * v1434);
        let v1439: f64 = (v1433 * v1434);
        let v1440: f64 = (v1424 - v1435);
        let v1441: f64 = (v1425 - v1436);
        let v1442: f64 = (v1426 - v1437);
        let v1443: f64 = (v1427 - v1438);
        let v1444: f64 = (v1428 - v1439);
        let v1445: f64 = (v152 * v1440);
        let v1446: f64 = (v152 * v1441);
        let v1447: f64 = (v152 * v1442);
        let v1448: f64 = (v152 * v1443);
        let v1449: f64 = (v152 * v1444);
        let v1450: f64 = (v306 * v306);
        let v1451: f64 = (v44 - v1450);
        let v1452: f64 = (v1445 * v1451);
        let v1453: f64 = (v1446 * v1451);
        let v1454: f64 = (v1447 * v1451);
        let v1455: f64 = (v1448 * v1451);
        let v1456: f64 = (v1449 * v1451);
        let v1457: f64 = (if self.scalar_v290 { v1452 } else { v1036 });
        let v1458: f64 = (if self.scalar_v290 { v1453 } else { v1037 });
        let v1459: f64 = (if self.scalar_v290 { v1454 } else { v1038 });
        let v1460: f64 = (if self.scalar_v290 { v1455 } else { v1039 });
        let v1461: f64 = (if self.scalar_v290 { v1456 } else { v1040 });
        let v1557: f64 = (if self.scalar_v330 { v945 } else { v1354 });
        let v1558: f64 = (if self.scalar_v330 { v946 } else { v1355 });
        let v1559: f64 = (if self.scalar_v330 { v947 } else { v1356 });
        let v1560: f64 = (if self.scalar_v330 { v948 } else { v1357 });
        let v1562: f64 = (v331 * v1557);
        let v1563: f64 = (v1562 + v1562);
        let v1564: f64 = (v331 * v1558);
        let v1565: f64 = (v1564 + v1564);
        let v1566: f64 = (v331 * v1559);
        let v1567: f64 = (v1566 + v1566);
        let v1568: f64 = (v331 * v1560);
        let v1569: f64 = (v1568 + v1568);
        let v1570: f64 = (v331 * self.scalar_v1561);
        let v1571: f64 = (v1570 + v1570);
        let v1572: f64 = (if self.scalar_v330 { v1563 } else { v1369 });
        let v1573: f64 = (if self.scalar_v330 { v1565 } else { v1370 });
        let v1574: f64 = (if self.scalar_v330 { v1567 } else { v1371 });
        let v1575: f64 = (if self.scalar_v330 { v1569 } else { v1372 });
        let v1576: f64 = (if self.scalar_v330 { v1571 } else { v1373 });
        let v1577: f64 = (self.scalar_v201 * v1572);
        let v1578: f64 = (self.scalar_v201 * v1573);
        let v1579: f64 = (self.scalar_v201 * v1574);
        let v1580: f64 = (self.scalar_v201 * v1575);
        let v1581: f64 = (self.scalar_v201 * v1576);
        let v1582: f64 = (v1557 + v1577);
        let v1583: f64 = (v1558 + v1578);
        let v1584: f64 = (v1559 + v1579);
        let v1585: f64 = (v1560 + v1580);
        let v1586: f64 = (self.scalar_v1561 + v1581);
        let v1587: f64 = (v181 * v1572);
        let v1588: f64 = (v181 * v1573);
        let v1589: f64 = (v181 * v1574);
        let v1590: f64 = (v181 * v1575);
        let v1591: f64 = (v181 * v1576);
        let v1592: f64 = (v336 * v1557);
        let v1593: f64 = (v331 * v1587);
        let v1594: f64 = (v1592 + v1593);
        let v1595: f64 = (v336 * v1558);
        let v1596: f64 = (v331 * v1588);
        let v1597: f64 = (v1595 + v1596);
        let v1598: f64 = (v336 * v1559);
        let v1599: f64 = (v331 * v1589);
        let v1600: f64 = (v1598 + v1599);
        let v1601: f64 = (v336 * v1560);
        let v1602: f64 = (v331 * v1590);
        let v1603: f64 = (v1601 + v1602);
        let v1604: f64 = (v336 * self.scalar_v1561);
        let v1605: f64 = (v331 * v1591);
        let v1606: f64 = (v1604 + v1605);
        let v1607: f64 = (v1582 + v1594);
        let v1608: f64 = (v1583 + v1597);
        let v1609: f64 = (v1584 + v1600);
        let v1610: f64 = (v1585 + v1603);
        let v1611: f64 = (v1586 + v1606);
        let v1612: f64 = (v176 * v1607);
        let v1613: f64 = (v338 * v924);
        let v1614: f64 = (v176 * v1608);
        let v1615: f64 = (v1613 + v1614);
        let v1616: f64 = (v338 * v925);
        let v1617: f64 = (v176 * v1609);
        let v1618: f64 = (v1616 + v1617);
        let v1619: f64 = (v176 * v1610);
        let v1620: f64 = (v176 * v1611);
        let v1621: f64 = (if self.scalar_v330 { v1612 } else { v1418 });
        let v1622: f64 = (if self.scalar_v330 { v1615 } else { v1419 });
        let v1623: f64 = (if self.scalar_v330 { v1618 } else { v1420 });
        let v1624: f64 = (if self.scalar_v330 { v1619 } else { v1421 });
        let v1625: f64 = (if self.scalar_v330 { v1620 } else { v1422 });
        let v1695: f64 = { let limexp_arg = v340; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v1696: f64 = (v1621 * v1695);
        let v1697: f64 = (v1622 * v1695);
        let v1698: f64 = (v1623 * v1695);
        let v1699: f64 = (v1624 * v1695);
        let v1700: f64 = (v1625 * v1695);
        let v1701: f64 = (-v1621);
        let v1702: f64 = (-v1622);
        let v1703: f64 = (-v1623);
        let v1704: f64 = (-v1624);
        let v1705: f64 = (-v1625);
        let v1706: f64 = { let limexp_arg = v352; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v1707: f64 = (v1701 * v1706);
        let v1708: f64 = (v1702 * v1706);
        let v1709: f64 = (v1703 * v1706);
        let v1710: f64 = (v1704 * v1706);
        let v1711: f64 = (v1705 * v1706);
        let v1712: f64 = (v1696 - v1707);
        let v1713: f64 = (v1697 - v1708);
        let v1714: f64 = (v1698 - v1709);
        let v1715: f64 = (v1699 - v1710);
        let v1716: f64 = (v1700 - v1711);
        let v1717: f64 = (v152 * v1712);
        let v1718: f64 = (v152 * v1713);
        let v1719: f64 = (v152 * v1714);
        let v1720: f64 = (v152 * v1715);
        let v1721: f64 = (v152 * v1716);
        let v1722: f64 = (v356 * v356);
        let v1723: f64 = (v44 - v1722);
        let v1724: f64 = (v1717 * v1723);
        let v1725: f64 = (v1718 * v1723);
        let v1726: f64 = (v1719 * v1723);
        let v1727: f64 = (v1720 * v1723);
        let v1728: f64 = (v1721 * v1723);
        let v1729: f64 = (if self.scalar_v330 { v1724 } else { v1457 });
        let v1730: f64 = (if self.scalar_v330 { v1725 } else { v1458 });
        let v1731: f64 = (if self.scalar_v330 { v1726 } else { v1459 });
        let v1732: f64 = (if self.scalar_v330 { v1727 } else { v1460 });
        let v1733: f64 = (if self.scalar_v330 { v1728 } else { v1461 });
        let v2084: f64 = (v139 * v1002);
        let v2085: f64 = (-v2084);
        let v2086: f64 = (v427 * v427);
        let v2087: f64 = (v2085 / v2086);
        let v2088: f64 = (v139 * v1003);
        let v2089: f64 = (-v2088);
        let v2090: f64 = (v2089 / v2086);
        let v2091: f64 = (v139 * v1004);
        let v2092: f64 = (-v2091);
        let v2093: f64 = (v2092 / v2086);
        let v2094: f64 = (v139 * v1005);
        let v2095: f64 = (-v2094);
        let v2096: f64 = (v2095 / v2086);
        let v2097: f64 = (v139 * v1006);
        let v2098: f64 = (-v2097);
        let v2099: f64 = (v2098 / v2086);
        let v2100: f64 = (if self.scalar_v425 { v2087 } else { v13 });
        let v2101: f64 = (if self.scalar_v425 { v2090 } else { v13 });
        let v2102: f64 = (if self.scalar_v425 { v2093 } else { v13 });
        let v2103: f64 = (if self.scalar_v425 { v2096 } else { v13 });
        let v2104: f64 = (if self.scalar_v425 { v2099 } else { v13 });
        let v2115: f64 = (v139 * v1729);
        let v2116: f64 = (-v2115);
        let v2117: f64 = (v440 * v440);
        let v2118: f64 = (v2116 / v2117);
        let v2119: f64 = (v139 * v1730);
        let v2120: f64 = (-v2119);
        let v2121: f64 = (v2120 / v2117);
        let v2122: f64 = (v139 * v1731);
        let v2123: f64 = (-v2122);
        let v2124: f64 = (v2123 / v2117);
        let v2125: f64 = (v139 * v1732);
        let v2126: f64 = (-v2125);
        let v2127: f64 = (v2126 / v2117);
        let v2128: f64 = (v139 * v1733);
        let v2129: f64 = (-v2128);
        let v2130: f64 = (v2129 / v2117);
        let v2131: f64 = (if self.scalar_v439 { v2118 } else { v2100 });
        let v2132: f64 = (if self.scalar_v439 { v2121 } else { v2101 });
        let v2133: f64 = (if self.scalar_v439 { v2124 } else { v2102 });
        let v2134: f64 = (if self.scalar_v439 { v2127 } else { v2103 });
        let v2135: f64 = (if self.scalar_v439 { v2130 } else { v2104 });
        let v2225: f64 = (v526 * v526);
        let v2226: f64 = (v44 - v2225);
        let v2227: f64 = (self.scalar_v523 * v2226);
        let v2228: f64 = (self.scalar_v2224 * v2226);
        let v2229: f64 = (self.scalar_v520 * v2226);
        let v2231: f64 = (v532 * v532);
        let v2232: f64 = (v44 - v2231);
        let v2233: f64 = (self.scalar_v529 * v2232);
        let v2234: f64 = (self.scalar_v2230 * v2232);
        let v2236: f64 = (v538 * v538);
        let v2237: f64 = (v44 - v2236);
        let v2238: f64 = (self.scalar_v2235 * v2237);
        let v2239: f64 = (self.scalar_v535 * v2237);
        let v2242: f64 = (v545 * v545);
        let v2243: f64 = (v44 - v2242);
        let v2244: f64 = (self.scalar_v2241 * v2243);
        let v2245: f64 = (self.scalar_v523 * v2243);
        let v2246: f64 = (self.scalar_v541 * v2243);
        let v2247: f64 = (v137 * v2227);
        let v2248: f64 = (v137 * v2228);
        let v2249: f64 = (v137 * v2229);
        let v2250: f64 = (v557 * v2233);
        let v2251: f64 = (v533 * v2247);
        let v2252: f64 = (v2250 + v2251);
        let v2253: f64 = (v557 * v2234);
        let v2254: f64 = (v533 * v2248);
        let v2255: f64 = (v2253 + v2254);
        let v2256: f64 = (v533 * v2249);
        let v2257: f64 = (if self.scalar_v556 { v2252 } else { v13 });
        let v2258: f64 = (if self.scalar_v556 { v2255 } else { v13 });
        let v2259: f64 = (if self.scalar_v556 { v2256 } else { v13 });
        let v2260: f64 = (v546 * v2238);
        let v2261: f64 = (v540 * v2244);
        let v2262: f64 = (v2260 + v2261);
        let v2263: f64 = (v546 * v2239);
        let v2264: f64 = (v540 * v2245);
        let v2265: f64 = (v2263 + v2264);
        let v2266: f64 = (v540 * v2246);
        let v2267: f64 = (v138 * v2262);
        let v2268: f64 = (v138 * v2265);
        let v2269: f64 = (v138 * v2266);
        let v2270: f64 = (if self.scalar_v556 { v2267 } else { v13 });
        let v2271: f64 = (if self.scalar_v556 { v2268 } else { v13 });
        let v2272: f64 = (if self.scalar_v556 { v2269 } else { v13 });
        let v2273: f64 = (if self.scalar_v569 { v2233 } else { v2233 });
        let v2274: f64 = (if self.scalar_v569 { v2234 } else { v2234 });
        let v2275: f64 = ((v572) as f64).sinh();
        let v2276: f64 = (self.scalar_v523 * v2275);
        let v2277: f64 = (self.scalar_v2223 * v2275);
        let v2278: f64 = (if self.scalar_v569 { v2276 } else { v13 });
        let v2279: f64 = (if self.scalar_v569 { v2277 } else { v13 });
        let v2280: f64 = (v2278 / v574);
        let v2281: f64 = (v2279 / v574);
        let v2282: f64 = (if self.scalar_v569 { v2280 } else { v13 });
        let v2283: f64 = (if self.scalar_v569 { v2281 } else { v13 });
        let v2285: f64 = (self.scalar_v523 * v2284);
        let v2286: f64 = (self.scalar_v2224 * v2284);
        let v2288: f64 = (if self.scalar_v569 { v2285 } else { v13 });
        let v2289: f64 = (if self.scalar_v569 { v2286 } else { v13 });
        let v2291: f64 = (v2288 / v578);
        let v2292: f64 = (v2289 / v578);
        let v2294: f64 = (if self.scalar_v569 { v2291 } else { v13 });
        let v2295: f64 = (if self.scalar_v569 { v2292 } else { v13 });
        let v2297: f64 = (self.scalar_v523 + v2282);
        let v2298: f64 = (self.scalar_v2223 + v2283);
        let v2299: f64 = (if self.scalar_v569 { v2297 } else { v13 });
        let v2300: f64 = (if self.scalar_v569 { v2298 } else { v13 });
        let v2301: f64 = (self.scalar_v523 + v2294);
        let v2302: f64 = (self.scalar_v2224 + v2295);
        let v2304: f64 = (v2301 - v2299);
        let v2305: f64 = (v2302 - v2300);
        let v2306: f64 = (v584 * v2273);
        let v2307: f64 = (v571 * v2304);
        let v2308: f64 = (v2306 + v2307);
        let v2309: f64 = (v584 * v2274);
        let v2310: f64 = (v571 * v2305);
        let v2311: f64 = (v2309 + v2310);
        let v2313: f64 = (v2308 / self.scalar_v520);
        let v2314: f64 = (v2311 / self.scalar_v520);
        let v2317: f64 = (v2314 + self.scalar_v2316);
        let v2319: f64 = (v137 * v2313);
        let v2320: f64 = (v137 * v2317);
        let v2323: f64 = (v2320 + self.scalar_v2322);
        let v2325: f64 = (if self.scalar_v569 { v2319 } else { v13 });
        let v2326: f64 = (if self.scalar_v569 { v2323 } else { v13 });
        let v2328: f64 = ((v593) as f64).sinh();
        let v2329: f64 = (self.scalar_v2223 * v2328);
        let v2330: f64 = (self.scalar_v523 * v2328);
        let v2331: f64 = (if self.scalar_v569 { v2329 } else { v2278 });
        let v2332: f64 = (if self.scalar_v569 { v2330 } else { v2279 });
        let v2333: f64 = (v2331 / v595);
        let v2334: f64 = (v2332 / v595);
        let v2335: f64 = (if self.scalar_v569 { v2333 } else { v13 });
        let v2336: f64 = (if self.scalar_v569 { v2334 } else { v13 });
        let v2338: f64 = (self.scalar_v2241 * v2337);
        let v2339: f64 = (self.scalar_v523 * v2337);
        let v2341: f64 = (if self.scalar_v569 { v2338 } else { v2288 });
        let v2342: f64 = (if self.scalar_v569 { v2339 } else { v2289 });
        let v2345: f64 = (v2341 / v599);
        let v2346: f64 = (v2342 / v599);
        let v2348: f64 = (v2344 / v599);
        let v2349: f64 = (if self.scalar_v569 { v2345 } else { v13 });
        let v2350: f64 = (if self.scalar_v569 { v2346 } else { v13 });
        let v2352: f64 = (if self.scalar_v569 { v2348 } else { v13 });
        let v2353: f64 = (self.scalar_v2223 + v2335);
        let v2354: f64 = (self.scalar_v523 + v2336);
        let v2355: f64 = (if self.scalar_v569 { v2353 } else { v13 });
        let v2356: f64 = (if self.scalar_v569 { v2354 } else { v13 });
        let v2357: f64 = (self.scalar_v2241 + v2349);
        let v2358: f64 = (self.scalar_v523 + v2350);
        let v2360: f64 = (v2357 - v2355);
        let v2361: f64 = (v2358 - v2356);
        let v2362: f64 = (v605 * v2238);
        let v2363: f64 = (v540 * v2360);
        let v2364: f64 = (v2362 + v2363);
        let v2365: f64 = (v605 * v2239);
        let v2366: f64 = (v540 * v2361);
        let v2367: f64 = (v2365 + v2366);
        let v2369: f64 = (v540 * v2352);
        let v2370: f64 = (v2364 / self.scalar_v541);
        let v2371: f64 = (v2367 / self.scalar_v541);
        let v2373: f64 = (v2369 / self.scalar_v541);
        let v2374: f64 = (self.scalar_v2316 + v2370);
        let v2376: f64 = (v138 * v2374);
        let v2377: f64 = (v138 * v2371);
        let v2379: f64 = (v138 * v2373);
        let v2381: f64 = (v2376 + self.scalar_v2380);
        let v2383: f64 = (if self.scalar_v569 { v2381 } else { v13 });
        let v2384: f64 = (if self.scalar_v569 { v2377 } else { v13 });
        let v2386: f64 = (if self.scalar_v569 { v2379 } else { v13 });
        let v2387: f64 = (if self.scalar_v569 { v13 } else { v2257 });
        let v2388: f64 = (if self.scalar_v569 { v13 } else { v2258 });
        let v2389: f64 = (if self.scalar_v569 { v13 } else { v2259 });
        let v2390: f64 = (if self.scalar_v569 { v13 } else { v2270 });
        let v2391: f64 = (if self.scalar_v569 { v13 } else { v2271 });
        let v2392: f64 = (if self.scalar_v569 { v13 } else { v2272 });
        let v2397: f64 = (v624 * self.scalar_v2395);
        let v2398: f64 = (v2397 + v2397);
        let v2399: f64 = (v624 * self.scalar_v2396);
        let v2400: f64 = (v2399 + v2399);
        let v2402: f64 = f64::powf(v628, self.scalar_v2401);
        let v2403: f64 = (self.scalar_v629 * v2402);
        let v2404: f64 = (v2398 * v2403);
        let v2405: f64 = (v2400 * v2403);
        let v2406: f64 = (self.scalar_v632 * v2398);
        let v2407: f64 = (self.scalar_v632 * v2400);
        let v2408: f64 = (v634 * v2404);
        let v2409: f64 = (v630 * v2406);
        let v2410: f64 = (v2408 + v2409);
        let v2411: f64 = (v634 * v2405);
        let v2412: f64 = (v630 * v2407);
        let v2413: f64 = (v2411 + v2412);
        let v2414: f64 = (if self.scalar_v620 { v2410 } else { v13 });
        let v2415: f64 = (if self.scalar_v620 { v2413 } else { v13 });
        let v2419: f64 = (v640 * v640);
        let v2420: f64 = (v44 - v2419);
        let v2421: f64 = (self.scalar_v2417 * v2420);
        let v2422: f64 = (self.scalar_v2418 * v2420);
        let v2423: f64 = (self.scalar_v520 * v2420);
        let v2424: f64 = (if self.scalar_v620 { v2421 } else { v2227 });
        let v2425: f64 = (if self.scalar_v620 { v2422 } else { v2228 });
        let v2426: f64 = (if self.scalar_v620 { v2423 } else { v2229 });
        let v2427: f64 = (if self.scalar_v620 { v2233 } else { v2273 });
        let v2428: f64 = (if self.scalar_v620 { v2234 } else { v2274 });
        let v2429: f64 = (if self.scalar_v620 { v2238 } else { v2238 });
        let v2430: f64 = (if self.scalar_v620 { v2239 } else { v2239 });
        let v2435: f64 = (v651 * v651);
        let v2436: f64 = (v44 - v2435);
        let v2437: f64 = (self.scalar_v2433 * v2436);
        let v2438: f64 = (self.scalar_v2434 * v2436);
        let v2439: f64 = (self.scalar_v541 * v2436);
        let v2440: f64 = (if self.scalar_v620 { v2437 } else { v2244 });
        let v2441: f64 = (if self.scalar_v620 { v2438 } else { v2245 });
        let v2442: f64 = (if self.scalar_v620 { v2439 } else { v2246 });
        let v2443: f64 = (self.scalar_v654 * v2414);
        let v2444: f64 = (self.scalar_v654 * v2415);
        let v2445: f64 = (v2425 + v2443);
        let v2446: f64 = (v2426 + v2444);
        let v2447: f64 = (v137 * v2424);
        let v2448: f64 = (v137 * v2445);
        let v2449: f64 = (v137 * v2446);
        let v2450: f64 = (v657 * v2427);
        let v2451: f64 = (v643 * v2447);
        let v2452: f64 = (v2450 + v2451);
        let v2453: f64 = (v657 * v2428);
        let v2454: f64 = (v643 * v2448);
        let v2455: f64 = (v2453 + v2454);
        let v2456: f64 = (v643 * v2449);
        let v2457: f64 = (if self.scalar_v620 { v2452 } else { v2387 });
        let v2458: f64 = (if self.scalar_v620 { v2455 } else { v2388 });
        let v2459: f64 = (if self.scalar_v620 { v2456 } else { v2389 });
        let v2460: f64 = (v653 * v2429);
        let v2461: f64 = (v646 * v2440);
        let v2462: f64 = (v2460 + v2461);
        let v2463: f64 = (v653 * v2430);
        let v2464: f64 = (v646 * v2441);
        let v2465: f64 = (v2463 + v2464);
        let v2466: f64 = (v646 * v2442);
        let v2467: f64 = (v138 * v2462);
        let v2468: f64 = (v138 * v2465);
        let v2469: f64 = (v138 * v2466);
        let v2470: f64 = (if self.scalar_v620 { v2467 } else { v2390 });
        let v2471: f64 = (if self.scalar_v620 { v2468 } else { v2391 });
        let v2472: f64 = (if self.scalar_v620 { v2469 } else { v2392 });
        let v2473: f64 = (if self.scalar_v668 { v2276 } else { v2331 });
        let v2474: f64 = (if self.scalar_v668 { v2277 } else { v2332 });
        let v2475: f64 = (v2473 / v669);
        let v2476: f64 = (v2474 / v669);
        let v2477: f64 = (if self.scalar_v668 { v2475 } else { v2282 });
        let v2478: f64 = (if self.scalar_v668 { v2476 } else { v2283 });
        let v2479: f64 = (if self.scalar_v668 { v2285 } else { v2341 });
        let v2480: f64 = (if self.scalar_v668 { v2286 } else { v2342 });
        let v2483: f64 = (v2479 / v672);
        let v2484: f64 = (v2480 / v672);
        let v2485: f64 = (v2481 / v672);
        let v2487: f64 = (if self.scalar_v668 { v2483 } else { v2294 });
        let v2488: f64 = (if self.scalar_v668 { v2484 } else { v2295 });
        let v2489: f64 = (if self.scalar_v668 { v2485 } else { v13 });
        let v2494: f64 = (self.scalar_v2393 * v2493);
        let v2499: f64 = (v2494 * v2498);
        let v2501: f64 = (v682 * self.scalar_v2491);
        let v2502: f64 = (v677 * v2499);
        let v2503: f64 = (v2501 + v2502);
        let v2507: f64 = (if self.scalar_v668 { v2503 } else { v13 });
        let v2509: f64 = (self.scalar_v523 + v2477);
        let v2510: f64 = (self.scalar_v2223 + v2478);
        let v2511: f64 = (if self.scalar_v668 { v2509 } else { v2299 });
        let v2512: f64 = (if self.scalar_v668 { v2510 } else { v2300 });
        let v2513: f64 = (self.scalar_v523 + v2487);
        let v2514: f64 = (self.scalar_v2224 + v2488);
        let v2516: f64 = (v2513 - v2511);
        let v2517: f64 = (v2514 - v2512);
        let v2518: f64 = (v2507 + v2517);
        let v2520: f64 = (v696 * v2516);
        let v2521: f64 = (v695 * v2233);
        let v2522: f64 = (v2520 + v2521);
        let v2523: f64 = (v696 * v2518);
        let v2524: f64 = (v695 * v2234);
        let v2525: f64 = (v2523 + v2524);
        let v2526: f64 = (v696 * v2489);
        let v2528: f64 = (v2522 / self.scalar_v520);
        let v2529: f64 = (v2525 / self.scalar_v520);
        let v2530: f64 = (v2526 / self.scalar_v520);
        let v2532: f64 = (self.scalar_v2316 + v2529);
        let v2534: f64 = (v137 * v2528);
        let v2535: f64 = (v137 * v2532);
        let v2536: f64 = (v137 * v2530);
        let v2538: f64 = (self.scalar_v2322 + v2535);
        let v2540: f64 = (if self.scalar_v668 { v2534 } else { v2325 });
        let v2541: f64 = (if self.scalar_v668 { v2538 } else { v2326 });
        let v2542: f64 = (if self.scalar_v668 { v2536 } else { v13 });
        let v2544: f64 = (if self.scalar_v668 { v2329 } else { v2473 });
        let v2545: f64 = (if self.scalar_v668 { v2330 } else { v2474 });
        let v2546: f64 = (v2544 / v703);
        let v2547: f64 = (v2545 / v703);
        let v2548: f64 = (if self.scalar_v668 { v2546 } else { v2335 });
        let v2549: f64 = (if self.scalar_v668 { v2547 } else { v2336 });
        let v2550: f64 = (if self.scalar_v668 { v2338 } else { v2479 });
        let v2551: f64 = (if self.scalar_v668 { v2339 } else { v2480 });
        let v2553: f64 = (if self.scalar_v668 { v13 } else { v2482 });
        let v2554: f64 = (v2550 / v706);
        let v2555: f64 = (v2551 / v706);
        let v2557: f64 = (v2553 / v706);
        let v2558: f64 = (if self.scalar_v668 { v2554 } else { v2349 });
        let v2559: f64 = (if self.scalar_v668 { v2555 } else { v2350 });
        let v2561: f64 = (if self.scalar_v668 { v2557 } else { v2352 });
        let v2562: f64 = (self.scalar_v2223 + v2548);
        let v2563: f64 = (self.scalar_v523 + v2549);
        let v2564: f64 = (if self.scalar_v668 { v2562 } else { v2355 });
        let v2565: f64 = (if self.scalar_v668 { v2563 } else { v2356 });
        let v2566: f64 = (self.scalar_v2241 + v2558);
        let v2567: f64 = (self.scalar_v523 + v2559);
        let v2569: f64 = (v2566 - v2564);
        let v2570: f64 = (v2567 - v2565);
        let v2571: f64 = (v712 * v2238);
        let v2572: f64 = (v645 * v2569);
        let v2573: f64 = (v2571 + v2572);
        let v2574: f64 = (v712 * v2239);
        let v2575: f64 = (v645 * v2570);
        let v2576: f64 = (v2574 + v2575);
        let v2578: f64 = (v645 * v2561);
        let v2579: f64 = (v2573 / self.scalar_v541);
        let v2580: f64 = (v2576 / self.scalar_v541);
        let v2582: f64 = (v2578 / self.scalar_v541);
        let v2583: f64 = (self.scalar_v2316 + v2579);
        let v2585: f64 = (v138 * v2583);
        let v2586: f64 = (v138 * v2580);
        let v2588: f64 = (v138 * v2582);
        let v2589: f64 = (self.scalar_v2380 + v2585);
        let v2591: f64 = (if self.scalar_v668 { v2589 } else { v2383 });
        let v2592: f64 = (if self.scalar_v668 { v2586 } else { v2384 });
        let v2594: f64 = (if self.scalar_v668 { v2588 } else { v2386 });
        let v2595: f64 = (if self.scalar_v668 { v13 } else { v2457 });
        let v2596: f64 = (if self.scalar_v668 { v13 } else { v2458 });
        let v2597: f64 = (if self.scalar_v668 { v13 } else { v2459 });
        let v2598: f64 = (if self.scalar_v668 { v13 } else { v2470 });
        let v2599: f64 = (if self.scalar_v668 { v13 } else { v2471 });
        let v2600: f64 = (if self.scalar_v668 { v13 } else { v2472 });
        let v2606: f64 = 1.0;
        let v2607: f64 = (v2591 * v2606);
        let v2608: f64 = (v2592 * v2606);
        let v2609: f64 = (v2593 * v2606);
        let v2610: f64 = (v2594 * v2606);
        let v2611: f64 = (if self.scalar_v723 { v2607 } else { v13 });
        let v2612: f64 = (if self.scalar_v723 { v2608 } else { v13 });
        let v2613: f64 = (if self.scalar_v723 { v2609 } else { v13 });
        let v2614: f64 = (if self.scalar_v723 { v2610 } else { v13 });
        let v2615: f64 = (v2540 * v2606);
        let v2616: f64 = (v2541 * v2606);
        let v2617: f64 = (v2542 * v2606);
        let v2618: f64 = (v2543 * v2606);
        let v2619: f64 = (if self.scalar_v723 { v2615 } else { v13 });
        let v2620: f64 = (if self.scalar_v723 { v2616 } else { v13 });
        let v2621: f64 = (if self.scalar_v723 { v2617 } else { v13 });
        let v2622: f64 = (if self.scalar_v723 { v2618 } else { v13 });
        let v2623: f64 = (-v722);
        let v2624: f64 = (v5 * v2598);
        let v2625: f64 = (v2623 + v2624);
        let v2626: f64 = (v5 * v2599);
        let v2627: f64 = (v5 * v2600);
        let v2628: f64 = (v722 + v2627);
        let v2629: f64 = (v2606 * v2625);
        let v2630: f64 = (v2606 * v2626);
        let v2631: f64 = (v2606 * v2628);
        let v2632: f64 = (if self.scalar_v783 { v2629 } else { v13 });
        let v2633: f64 = (if self.scalar_v783 { v2630 } else { v13 });
        let v2634: f64 = (if self.scalar_v783 { v2631 } else { v13 });
        let v2635: f64 = (v9 * v2595);
        let v2636: f64 = (-v720);
        let v2637: f64 = (v9 * v2596);
        let v2638: f64 = (v2636 + v2637);
        let v2639: f64 = (v9 * v2597);
        let v2640: f64 = (v720 + v2639);
        let v2641: f64 = (v2606 * v2635);
        let v2642: f64 = (v2606 * v2638);
        let v2643: f64 = (v2606 * v2640);
        let v2644: f64 = (if self.scalar_v783 { v2641 } else { v13 });
        let v2645: f64 = (if self.scalar_v783 { v2642 } else { v13 });
        let v2646: f64 = (if self.scalar_v783 { v2643 } else { v13 });
        let v2649: f64 = (-v140);
        let v2651: f64 = (v800 * v2131);
        let v2652: f64 = (v800 * v2132);
        let v2653: f64 = (v800 * v2133);
        let v2654: f64 = (v800 * v2134);
        let v2655: f64 = (v800 * v2135);
        let v2656: f64 = (self.scalar_v724 * v2606);
        let v2657: f64 = (v443 + v2656);
        let v2658: f64 = (if self.scalar_v725 { v2651 } else { v13 });
        let v2659: f64 = (if self.scalar_v725 { v2652 } else { v13 });
        let v2660: f64 = (if self.scalar_v725 { v2653 } else { v13 });
        let v2661: f64 = (if self.scalar_v725 { v2654 } else { v13 });
        let v2662: f64 = (if self.scalar_v725 { v2655 } else { v13 });
        let v2663: f64 = (if self.scalar_v725 { v2657 } else { v13 });
        let v2668: f64 = (-v142);
        let v2669: f64 = (v2606 * v2668);
        let v2670: f64 = (v142 * v2606);
        let v2671: f64 = (if self.scalar_v728 { v2669 } else { v13 });
        let v2672: f64 = (if self.scalar_v728 { v2670 } else { v13 });
        let v2712: f64 = (v882 * v2606);
        let v2713: f64 = (if self.scalar_v747 { v2712 } else { v13 });
        let v2716: f64 = (self.scalar_v899 * v2606);
        let v2717: f64 = (if self.scalar_v771 { v2716 } else { v13 });

        let d775_dn15: f64 = self.scalar_v773;
        stamper.stamp_current_reactive_node1(
            Some(nodes[15]),
            None,
            nodes[15],
            multiplicity * (d775_dn15),
        );
        let d778_db0: f64 = self.scalar_v776;
        stamper.stamp_current_reactive_branch1(
            Some(nodes[15]),
            Some(nodes[16]),
            branches[0],
            multiplicity * (d778_db0),
        );
        let d780_dn5: f64 = v2611;
        let d780_dn8: f64 = v2612;
        let d780_dn10: f64 = v2613;
        let d780_dn11: f64 = v2614;
        let v780_reactive_nodes: [usize; 4] = [nodes[5], nodes[8], nodes[10], nodes[11]];
        let v780_reactive_node_derivatives: [f64; 4] = [d780_dn5, d780_dn8, d780_dn10, d780_dn11];
        stamper.stamp_current_reactive_dense(
            Some(nodes[10]),
            Some(nodes[5]),
            &v780_reactive_nodes,
            &v780_reactive_node_derivatives,
            &[],
            &[],
            multiplicity,
        );
        let d782_dn5: f64 = v2619;
        let d782_dn8: f64 = v2620;
        let d782_dn10: f64 = v2621;
        let d782_dn11: f64 = v2622;
        let v782_reactive_nodes: [usize; 4] = [nodes[5], nodes[8], nodes[10], nodes[11]];
        let v782_reactive_node_derivatives: [f64; 4] = [d782_dn5, d782_dn8, d782_dn10, d782_dn11];
        stamper.stamp_current_reactive_dense(
            Some(nodes[11]),
            Some(nodes[8]),
            &v782_reactive_nodes,
            &v782_reactive_node_derivatives,
            &[],
            &[],
            multiplicity,
        );
        let d786_dn5: f64 = v2632;
        let d786_dn8: f64 = v2633;
        let d786_dn10: f64 = v2634;
        stamper.stamp_current_reactive_node3(
            Some(nodes[10]),
            Some(nodes[5]),
            nodes[5],
            multiplicity * (d786_dn5),
            nodes[8],
            multiplicity * (d786_dn8),
            nodes[10],
            multiplicity * (d786_dn10),
        );
        let d789_dn5: f64 = v2644;
        let d789_dn8: f64 = v2645;
        let d789_dn11: f64 = v2646;
        stamper.stamp_current_reactive_node3(
            Some(nodes[11]),
            Some(nodes[8]),
            nodes[5],
            multiplicity * (d789_dn5),
            nodes[8],
            multiplicity * (d789_dn8),
            nodes[11],
            multiplicity * (d789_dn11),
        );
        let d793_dn5: f64 = self.scalar_v2647;
        let d793_dn7: f64 = self.scalar_v790;
        stamper.stamp_current_reactive_node2(
            Some(nodes[7]),
            Some(nodes[5]),
            nodes[5],
            multiplicity * (d793_dn5),
            nodes[7],
            multiplicity * (d793_dn7),
        );
        let d795_dn5: f64 = self.scalar_v794;
        let d795_dn8: f64 = self.scalar_v2648;
        stamper.stamp_current_reactive_node2(
            Some(nodes[5]),
            Some(nodes[8]),
            nodes[5],
            multiplicity * (d795_dn5),
            nodes[8],
            multiplicity * (d795_dn8),
        );
        let d798_dn4: f64 = v2649;
        let d798_dn6: f64 = v140;
        stamper.stamp_current_reactive_node2(
            Some(nodes[6]),
            Some(nodes[4]),
            nodes[4],
            multiplicity * (d798_dn4),
            nodes[6],
            multiplicity * (d798_dn6),
        );
        let d805_dn4: f64 = v2658;
        let d805_dn5: f64 = v2659;
        let d805_dn8: f64 = v2660;
        let d805_dn10: f64 = v2661;
        let d805_dn12: f64 = v2662;
        let d805_db1: f64 = v2663;
        let v805_reactive_nodes: [usize; 5] = [nodes[4], nodes[5], nodes[8], nodes[10], nodes[12]];
        let v805_reactive_node_derivatives: [f64; 5] = [d805_dn4, d805_dn5, d805_dn8, d805_dn10, d805_dn12];
        let v805_reactive_branches: [usize; 1] = [branches[1]];
        let v805_reactive_branch_derivatives: [f64; 1] = [d805_db1];
        stamper.stamp_current_reactive_dense(
            Some(nodes[4]),
            Some(nodes[8]),
            &v805_reactive_nodes,
            &v805_reactive_node_derivatives,
            &v805_reactive_branches,
            &v805_reactive_branch_derivatives,
            multiplicity,
        );
        let d813_dn8: f64 = v2671;
        let d813_dn12: f64 = v2672;
        stamper.stamp_current_reactive_node2(
            Some(nodes[12]),
            Some(nodes[8]),
            nodes[8],
            multiplicity * (d813_dn8),
            nodes[12],
            multiplicity * (d813_dn12),
        );
        let d819_dn11: f64 = self.scalar_v816;
        let d819_dn14: f64 = self.scalar_v2673;
        stamper.stamp_current_reactive_node2(
            Some(nodes[11]),
            Some(nodes[14]),
            nodes[11],
            multiplicity * (d819_dn11),
            nodes[14],
            multiplicity * (d819_dn14),
        );
        let d846_db10: f64 = self.scalar_v844;
        stamper.stamp_current_reactive_branch1(
            Some(nodes[1]),
            Some(nodes[7]),
            branches[10],
            multiplicity * (d846_db10),
        );
        let d856_db14: f64 = self.scalar_v854;
        stamper.stamp_current_reactive_branch1(
            Some(nodes[9]),
            Some(nodes[2]),
            branches[14],
            multiplicity * (d856_db14),
        );
        let d866_db18: f64 = self.scalar_v864;
        stamper.stamp_current_reactive_branch1(
            Some(nodes[6]),
            Some(nodes[0]),
            branches[18],
            multiplicity * (d866_db18),
        );
        let d885_dn17: f64 = v2713;
        stamper.stamp_current_reactive_node1(
            Some(nodes[7]),
            Some(nodes[5]),
            nodes[17],
            multiplicity * (d885_dn17),
        );
        let d902_dn3: f64 = v2717;
        stamper.stamp_current_reactive_node1(
            Some(nodes[3]),
            None,
            nodes[3],
            multiplicity * (d902_dn3),
        );
    }
}
