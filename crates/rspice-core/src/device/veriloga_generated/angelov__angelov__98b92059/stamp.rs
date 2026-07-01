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
        let v31: f64 = ((v30) as f64).abs();
        let v32: f64 = (self.scalar_v21 + v31);
        let v33: f64 = (if (self.scalar_v29 != 0.0) { v32 } else { self.scalar_v21 });
        let v34: f64 = 8.617333262145179e-5;
        let v35: f64 = (v33 * v34);
        let v36: f64 = (v33 - self.scalar_v28);
        let v37: f64 = ((v36) as f64).abs();
        let v38: bool = (v37 > v11);
        let v41: bool = (v38 || self.scalar_v40);
        let v42: f64 = 1.0;
        let v45: f64 = (v37 * self.scalar_v44);
        let v46: f64 = (v42 + v45);
        let v47: f64 = (self.scalar_v43 * v46);
        let v48: f64 = (if v41 { v47 } else { v11 });
        let v51: f64 = (v37 * self.scalar_v50);
        let v52: f64 = (v42 + v51);
        let v53: f64 = (self.scalar_v49 * v52);
        let v54: f64 = (if v41 { v53 } else { v11 });
        let v57: f64 = (v37 * self.scalar_v56);
        let v58: f64 = (v42 + v57);
        let v59: f64 = (self.scalar_v55 * v58);
        let v60: f64 = (if v41 { v59 } else { v11 });
        let v63: f64 = (v37 * self.scalar_v62);
        let v64: f64 = (v42 + v63);
        let v65: f64 = (self.scalar_v61 * v64);
        let v66: f64 = (if v41 { v65 } else { v11 });
        let v69: f64 = (v37 * self.scalar_v68);
        let v70: f64 = (v42 + v69);
        let v71: f64 = (self.scalar_v67 * v70);
        let v72: f64 = (if v41 { v71 } else { v11 });
        let v75: f64 = (v37 * self.scalar_v74);
        let v76: f64 = (v42 + v75);
        let v77: f64 = (self.scalar_v73 * v76);
        let v78: f64 = (if v41 { v77 } else { v11 });
        let v81: f64 = (v37 * self.scalar_v80);
        let v82: f64 = (v42 + v81);
        let v83: f64 = (self.scalar_v79 * v82);
        let v84: f64 = (if v41 { v83 } else { v11 });
        let v87: f64 = (v37 * self.scalar_v86);
        let v88: f64 = (self.scalar_v85 + v87);
        let v89: f64 = (if v41 { v88 } else { v11 });
        let v93: f64 = (v37 * self.scalar_v92);
        let v94: f64 = (self.scalar_v90 + v93);
        let v95: f64 = (if v41 { v94 } else { v11 });
        let v99: f64 = (v37 * self.scalar_v98);
        let v100: f64 = (self.scalar_v96 + v99);
        let v101: f64 = (if v41 { v100 } else { v11 });
        let v104: f64 = (v37 * self.scalar_v103);
        let v105: f64 = (self.scalar_v102 + v104);
        let v106: f64 = (if v41 { v105 } else { v11 });
        let v109: f64 = (v37 * self.scalar_v108);
        let v110: f64 = (self.scalar_v107 + v109);
        let v111: f64 = (if v41 { v110 } else { v11 });
        let v112: bool = (!v41);
        let v113: f64 = (if v112 { self.scalar_v43 } else { v48 });
        let v114: f64 = (if v112 { self.scalar_v49 } else { v54 });
        let v115: f64 = (if v112 { self.scalar_v55 } else { v60 });
        let v116: f64 = (if v112 { self.scalar_v61 } else { v66 });
        let v117: f64 = (if v112 { self.scalar_v67 } else { v72 });
        let v118: f64 = (if v112 { self.scalar_v73 } else { v78 });
        let v119: f64 = (if v112 { self.scalar_v79 } else { v84 });
        let v120: f64 = (if v112 { self.scalar_v85 } else { v89 });
        let v121: f64 = (if v112 { self.scalar_v90 } else { v95 });
        let v122: f64 = (if v112 { self.scalar_v96 } else { v101 });
        let v123: f64 = (if v112 { self.scalar_v102 } else { v106 });
        let v124: f64 = (if v112 { self.scalar_v107 } else { v111 });
        let v129: f64 = 0.5;
        let v132: f64 = (self.scalar_v131 / v35);
        let v133: f64 = (if self.scalar_v128 { v132 } else { v11 });
        let v136: f64 = (if self.scalar_v134 { self.scalar_v135 } else { v133 });
        let v138: f64 = (v7 * self.scalar_v137);
        let v139: f64 = ((v138) as f64).cosh();
        let v141: f64 = (v139 * v139);
        let v142: f64 = (self.scalar_v140 / v141);
        let v143: f64 = (v42 + v142);
        let v144: f64 = (v114 * v143);
        let v146: f64 = (v120 - self.scalar_v145);
        let v148: f64 = (v7 * self.scalar_v147);
        let v149: f64 = ((v148) as f64).tanh();
        let v150: f64 = (self.scalar_v145 * v149);
        let v151: f64 = (v146 + v150);
        let v153: f64 = (v6 - self.scalar_v107);
        let v154: f64 = (self.scalar_v152 * v153);
        let v155: f64 = (v6 - v124);
        let v156: f64 = (v154 * v155);
        let v157: f64 = (v151 - v156);
        let v158: f64 = (v2 - v157);
        let v159: f64 = (v158 * v158);
        let v160: f64 = (v144 * v158);
        let v162: f64 = (v159 * self.scalar_v161);
        let v163: f64 = (v160 + v162);
        let v165: f64 = (v158 * self.scalar_v164);
        let v166: f64 = (v159 * v165);
        let v167: f64 = (v163 + v166);
        let v168: f64 = ((v167) as f64).tanh();
        let v169: f64 = (v42 + v168);
        let v170: f64 = { let limexp_arg = v167; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let v171: f64 = (-v167);
        let v172: f64 = { let limexp_arg = v171; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let v173: f64 = (v170 - v172);
        let v174: f64 = (v129 * v173);
        let v175: f64 = ((v174) as f64).tanh();
        let v176: f64 = (v42 + v175);
        let v178: f64 = (self.scalar_v147 * v169);
        let v179: f64 = (self.scalar_v177 + v178);
        let v180: f64 = (v7 * v179);
        let v181: f64 = ((v180) as f64).tanh();
        let v189: f64 = (v113 * v169);
        let v190: f64 = (v181 * v189);
        let v192: f64 = (v7 * self.scalar_v191);
        let v193: f64 = (v42 + v192);
        let v194: f64 = { let limexp_arg = v155; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let v195: f64 = (v115 * v194);
        let v196: f64 = (v193 + v195);
        let v197: f64 = (v190 * v196);
        let v198: f64 = (if self.scalar_v183 { v197 } else { v11 });
        let v201: f64 = (v5 - v157);
        let v202: f64 = (if self.scalar_v200 { v201 } else { v139 });
        let v203: f64 = (v202 * v202);
        let v204: f64 = (if self.scalar_v200 { v203 } else { v158 });
        let v205: f64 = (v202 * v204);
        let v206: f64 = (if self.scalar_v200 { v205 } else { v159 });
        let v207: f64 = (v144 * v202);
        let v208: f64 = (self.scalar_v161 * v204);
        let v209: f64 = (v207 + v208);
        let v210: f64 = (self.scalar_v164 * v206);
        let v211: f64 = (v209 + v210);
        let v212: f64 = (if self.scalar_v200 { v211 } else { v11 });
        let v213: f64 = ((v212) as f64).tanh();
        let v214: f64 = (v42 + v213);
        let v215: f64 = (if self.scalar_v200 { v214 } else { v11 });
        let v216: f64 = (self.scalar_v147 * v215);
        let v217: f64 = (self.scalar_v177 + v216);
        let v218: f64 = (if self.scalar_v200 { v217 } else { v11 });
        let v220: f64 = (v169 * self.scalar_v219);
        let v221: f64 = (self.scalar_v191 + v220);
        let v222: f64 = (if self.scalar_v200 { v221 } else { v11 });
        let v223: f64 = (v42 + v181);
        let v224: f64 = (v189 * v223);
        let v225: f64 = (v7 * v222);
        let v226: f64 = (v42 + v225);
        let v227: f64 = (v7 - v124);
        let v228: f64 = { let limexp_arg = v227; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let v229: f64 = (v115 * v228);
        let v230: f64 = (v226 + v229);
        let v231: f64 = (v224 * v230);
        let v232: f64 = (if self.scalar_v200 { v231 } else { v11 });
        let v233: f64 = (v215 * self.scalar_v219);
        let v234: f64 = (self.scalar_v191 + v233);
        let v235: f64 = (if self.scalar_v200 { v234 } else { v11 });
        let v236: f64 = (v7 * v218);
        let v237: f64 = ((v236) as f64).tanh();
        let v238: f64 = (if self.scalar_v200 { v237 } else { v11 });
        let v239: f64 = (v113 * v215);
        let v240: f64 = (v42 - v238);
        let v241: f64 = (v239 * v240);
        let v242: f64 = (v7 * v235);
        let v243: f64 = (v42 - v242);
        let v244: f64 = (v241 * v243);
        let v245: f64 = (if self.scalar_v200 { v244 } else { v11 });
        let v246: f64 = (v232 - v245);
        let v247: f64 = (v129 * v246);
        let v248: f64 = (if self.scalar_v200 { v247 } else { v198 });
        let v252: f64 = (if self.scalar_v251 { v158 } else { v202 });
        let v253: f64 = (v252 * v252);
        let v254: f64 = (if self.scalar_v251 { v253 } else { v204 });
        let v255: f64 = (self.scalar_v161 * v254);
        let v256: f64 = (v252 + v255);
        let v257: f64 = (self.scalar_v164 * v254);
        let v258: f64 = (v252 * v257);
        let v259: f64 = (v256 + v258);
        let v260: f64 = (v144 * v259);
        let v261: f64 = (if self.scalar_v251 { v260 } else { v167 });
        let v262: f64 = { let limexp_arg = v261; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let v263: f64 = (-v261);
        let v264: f64 = { let limexp_arg = v263; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let v265: f64 = (v262 - v264);
        let v266: f64 = (v129 * v265);
        let v267: f64 = ((v266) as f64).tanh();
        let v268: f64 = (v42 + v267);
        let v269: f64 = (if self.scalar_v251 { v268 } else { v176 });
        let v270: f64 = (self.scalar_v147 * v269);
        let v271: f64 = (self.scalar_v177 + v270);
        let v272: f64 = (if self.scalar_v251 { v271 } else { v11 });
        let v273: f64 = (v7 * v272);
        let v274: f64 = ((v273) as f64).tanh();
        let v275: f64 = (if self.scalar_v251 { v274 } else { v11 });
        let v276: f64 = (self.scalar_v219 * v269);
        let v277: f64 = (self.scalar_v191 + v276);
        let v278: f64 = (if self.scalar_v251 { v277 } else { v222 });
        let v279: f64 = (v113 * v269);
        let v280: f64 = (v275 * v279);
        let v281: f64 = (v7 * v278);
        let v282: f64 = (v42 + v281);
        let v283: f64 = (v195 + v282);
        let v284: f64 = (v280 * v283);
        let v285: f64 = (if self.scalar_v251 { v284 } else { v248 });
        let v289: f64 = (if self.scalar_v288 { v158 } else { v252 });
        let v290: f64 = (v289 * v289);
        let v291: f64 = (if self.scalar_v288 { v290 } else { v254 });
        let v292: f64 = (self.scalar_v161 * v291);
        let v293: f64 = (v289 + v292);
        let v294: f64 = (self.scalar_v164 * v291);
        let v295: f64 = (v289 * v294);
        let v296: f64 = (v293 + v295);
        let v297: f64 = (v144 * v296);
        let v298: f64 = (if self.scalar_v288 { v297 } else { v261 });
        let v299: f64 = (if self.scalar_v288 { v201 } else { v206 });
        let v300: f64 = (v299 * v299);
        let v301: f64 = (if self.scalar_v288 { v300 } else { v11 });
        let v302: f64 = (self.scalar_v161 * v301);
        let v303: f64 = (v299 + v302);
        let v304: f64 = (self.scalar_v164 * v299);
        let v305: f64 = (v301 * v304);
        let v306: f64 = (v303 + v305);
        let v307: f64 = (v144 * v306);
        let v308: f64 = (if self.scalar_v288 { v307 } else { v212 });
        let v309: f64 = { let limexp_arg = v298; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let v310: f64 = (-v298);
        let v311: f64 = { let limexp_arg = v310; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let v312: f64 = (v309 - v311);
        let v313: f64 = (v129 * v312);
        let v314: f64 = ((v313) as f64).tanh();
        let v315: f64 = (v42 + v314);
        let v316: f64 = (if self.scalar_v288 { v315 } else { v269 });
        let v317: f64 = { let limexp_arg = v308; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let v318: f64 = (-v308);
        let v319: f64 = { let limexp_arg = v318; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let v320: f64 = (v317 - v319);
        let v321: f64 = (v129 * v320);
        let v322: f64 = ((v321) as f64).tanh();
        let v323: f64 = (v42 + v322);
        let v324: f64 = (if self.scalar_v288 { v323 } else { v11 });
        let v325: f64 = (self.scalar_v147 * v316);
        let v326: f64 = (self.scalar_v177 + v325);
        let v327: f64 = (if self.scalar_v288 { v326 } else { v272 });
        let v328: f64 = (self.scalar_v147 * v324);
        let v329: f64 = (self.scalar_v177 + v328);
        let v330: f64 = (if self.scalar_v288 { v329 } else { v11 });
        let v331: f64 = (v7 * v327);
        let v332: f64 = ((v331) as f64).tanh();
        let v333: f64 = (if self.scalar_v288 { v332 } else { v275 });
        let v334: f64 = (v7 * v330);
        let v335: f64 = ((v334) as f64).tanh();
        let v336: f64 = (if self.scalar_v288 { v335 } else { v11 });
        let v337: f64 = (self.scalar_v219 * v324);
        let v338: f64 = (self.scalar_v191 + v337);
        let v339: f64 = (if self.scalar_v288 { v338 } else { v11 });
        let v340: f64 = (self.scalar_v219 * v316);
        let v341: f64 = (self.scalar_v191 + v340);
        let v342: f64 = (if self.scalar_v288 { v341 } else { v11 });
        let v343: f64 = (v113 * v316);
        let v344: f64 = (v42 + v333);
        let v345: f64 = (v343 * v344);
        let v346: f64 = (v7 * v342);
        let v347: f64 = (v42 + v346);
        let v348: f64 = (v229 + v347);
        let v349: f64 = (v345 * v348);
        let v350: f64 = (if self.scalar_v288 { v349 } else { v232 });
        let v351: f64 = (v113 * v324);
        let v352: f64 = (v42 - v336);
        let v353: f64 = (v351 * v352);
        let v354: f64 = (v7 * v339);
        let v355: f64 = (v42 - v354);
        let v356: f64 = (v353 * v355);
        let v357: f64 = (if self.scalar_v288 { v356 } else { v245 });
        let v358: f64 = (v350 - v357);
        let v359: f64 = (v129 * v358);
        let v360: f64 = (if self.scalar_v288 { v359 } else { v285 });
        let v362: f64 = (v42 + v169);
        let v363: f64 = (v118 / v362);
        let v364: f64 = (self.scalar_v361 + v363);
        let v365: f64 = (if self.scalar_v249 { v364 } else { v11 });
        let v368: f64 = (v169 * self.scalar_v367);
        let v369: f64 = (self.scalar_v366 + v368);
        let v370: f64 = (if self.scalar_v249 { v369 } else { v11 });
        let v372: f64 = (v368 + self.scalar_v371);
        let v373: f64 = (if self.scalar_v249 { v372 } else { v11 });
        let v374: f64 = (v42 + v316);
        let v375: f64 = (v118 / v374);
        let v376: f64 = (self.scalar_v361 + v375);
        let v377: f64 = (if self.scalar_v250 { v376 } else { v365 });
        let v378: f64 = (v316 * self.scalar_v367);
        let v379: f64 = (self.scalar_v366 + v378);
        let v380: f64 = (if self.scalar_v250 { v379 } else { v370 });
        let v381: f64 = (self.scalar_v371 + v378);
        let v382: f64 = (if self.scalar_v250 { v381 } else { v373 });
        let v383: bool = ((v37 != 0.0) || self.scalar_v40);
        let v385: f64 = (v37 * self.scalar_v384);
        let v386: f64 = (v42 + v385);
        let v387: f64 = (v382 * v386);
        let v388: f64 = (if v383 { v387 } else { v11 });
        let v389: f64 = (v380 * v386);
        let v390: f64 = (if v383 { v389 } else { v11 });
        let v391: bool = (!v383);
        let v392: f64 = (if v391 { v380 } else { v390 });
        let v393: f64 = (if v391 { v382 } else { v388 });
        let v396: f64 = -1.0;
        let v397: f64 = (-v123);
        let v398: f64 = ((v397) as f64).tanh();
        let v399: f64 = (v136 * v398);
        let v400: f64 = { let limexp_arg = v399; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let v401: f64 = (if self.scalar_v395 { v400 } else { v289 });
        let v402: f64 = (v2 - v123);
        let v403: f64 = (if self.scalar_v395 { v402 } else { v11 });
        let v404: f64 = (v9 - v123);
        let v405: f64 = (if self.scalar_v395 { v404 } else { v11 });
        let v407: f64 = (-v136);
        let v408: f64 = (v123 * v407);
        let v409: f64 = { let limexp_arg = v408; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let v410: f64 = (if self.scalar_v406 { v409 } else { v401 });
        let v413: f64 = ((v402) as f64).tanh();
        let v414: f64 = (if self.scalar_v412 { v413 } else { v403 });
        let v415: f64 = ((v404) as f64).tanh();
        let v416: f64 = (if self.scalar_v412 { v415 } else { v405 });
        let v419: f64 = (if self.scalar_v418 { v402 } else { v414 });
        let v420: f64 = (if self.scalar_v418 { v404 } else { v416 });
        let v422: f64 = (v136 * v419);
        let v423: f64 = { let limexp_arg = v422; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let v424: f64 = (v423 - v410);
        let v425: f64 = (self.scalar_v421 * v424);
        let v426: f64 = (v136 * v420);
        let v427: f64 = { let limexp_arg = v426; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let v428: f64 = (v427 - v410);
        let v429: f64 = (self.scalar_v421 * v428);
        let v430: f64 = (v2 * self.scalar_v91);
        let v431: f64 = (v121 + v430);
        let v433: f64 = (v7 * self.scalar_v432);
        let v434: f64 = (v431 + v433);
        let v435: f64 = ((v434) as f64).tanh();
        let v436: f64 = (v42 + v435);
        let v439: f64 = (v7 * self.scalar_v438);
        let v440: f64 = (self.scalar_v437 + v439);
        let v441: f64 = ((v440) as f64).tanh();
        let v442: f64 = (v42 + v441);
        let v445: f64 = (v7 * self.scalar_v444);
        let v446: f64 = (self.scalar_v443 - v445);
        let v447: f64 = ((v446) as f64).tanh();
        let v448: f64 = (v42 + v447);
        let v449: f64 = (v448 - self.scalar_v432);
        let v450: f64 = (v9 * self.scalar_v97);
        let v451: f64 = (v122 + v450);
        let v452: f64 = (v451 - v433);
        let v453: f64 = ((v452) as f64).tanh();
        let v454: f64 = (v42 + v453);
        let v465: f64 = (v116 * v436);
        let v466: f64 = (v442 * v465);
        let v467: f64 = (self.scalar_v459 + v466);
        let v468: f64 = (if self.scalar_v464 { v467 } else { self.scalar_v460 });
        let v469: f64 = (v449 * v454);
        let v471: f64 = (v469 + self.scalar_v470);
        let v472: f64 = (v117 * v471);
        let v473: f64 = (self.scalar_v461 + v472);
        let v474: f64 = (if self.scalar_v464 { v473 } else { self.scalar_v462 });
        let v478: f64 = (v442 - self.scalar_v432);
        let v479: f64 = (if self.scalar_v477 { v478 } else { v442 });
        let v480: f64 = (v121 + v433);
        let v481: f64 = ((v480) as f64).cosh();
        let v482: f64 = (if self.scalar_v477 { v481 } else { v11 });
        let v483: f64 = ((v482) as f64).ln();
        let v484: f64 = (if self.scalar_v477 { v483 } else { v11 });
        let v485: f64 = ((v434) as f64).cosh();
        let v486: f64 = (if self.scalar_v477 { v485 } else { v11 });
        let v487: f64 = ((v486) as f64).ln();
        let v488: f64 = (if self.scalar_v477 { v487 } else { v11 });
        let v489: f64 = (v480 + v484);
        let v490: f64 = (if self.scalar_v477 { v489 } else { v11 });
        let v491: f64 = (v434 + v488);
        let v492: f64 = (v491 - v490);
        let v493: f64 = (v479 * v492);
        let v494: f64 = (v493 / self.scalar_v91);
        let v495: f64 = (v2 * self.scalar_v470);
        let v496: f64 = (v494 + v495);
        let v497: f64 = (v116 * v496);
        let v498: f64 = (v2 * self.scalar_v459);
        let v499: f64 = (v497 + v498);
        let v500: f64 = (if self.scalar_v477 { v499 } else { v11 });
        let v501: f64 = (v122 - v433);
        let v502: f64 = ((v501) as f64).cosh();
        let v503: f64 = (if self.scalar_v477 { v502 } else { v482 });
        let v504: f64 = ((v503) as f64).ln();
        let v505: f64 = (if self.scalar_v477 { v504 } else { v11 });
        let v506: f64 = ((v452) as f64).cosh();
        let v507: f64 = (if self.scalar_v477 { v506 } else { v486 });
        let v508: f64 = ((v507) as f64).ln();
        let v509: f64 = (if self.scalar_v477 { v508 } else { v11 });
        let v510: f64 = (v501 + v505);
        let v511: f64 = (if self.scalar_v477 { v510 } else { v11 });
        let v512: f64 = (v452 + v509);
        let v513: f64 = (v512 - v511);
        let v514: f64 = (v449 * v513);
        let v515: f64 = (v514 / self.scalar_v97);
        let v516: f64 = (v9 * self.scalar_v470);
        let v517: f64 = (v515 + v516);
        let v518: f64 = (v117 * v517);
        let v519: f64 = (v9 * self.scalar_v461);
        let v520: f64 = (v518 + v519);
        let v521: f64 = (if self.scalar_v477 { v520 } else { v11 });
        let v1716: f64 = ((v434) as f64).sinh();
        let v1719: f64 = (self.scalar_v91 * v1716);
        let v1722: f64 = (if self.scalar_v477 { v1719 } else { v11 });
        let v1725: f64 = (v1722 / v486);
        let v1728: f64 = (if self.scalar_v477 { v1725 } else { v11 });
        let v1735: f64 = (self.scalar_v91 + v1728);
        let v1744: f64 = (v479 * v1735);
        let v1747: f64 = (v1744 / self.scalar_v91);
        let v1750: f64 = (self.scalar_v470 + v1747);
        let v1753: f64 = (v116 * v1750);
        let v1756: f64 = (self.scalar_v459 + v1753);
        let v1759: f64 = (if self.scalar_v477 { v1756 } else { v11 });
        let v522: f64 = v1759;
        let v523: f64 = (if self.scalar_v477 { v522 } else { v468 });
        let v1769: f64 = ((v452) as f64).sinh();
        let v1772: f64 = (self.scalar_v97 * v1769);
        let v1775: f64 = (if self.scalar_v477 { v1772 } else { v11 });
        let v1779: f64 = (v1775 / v507);
        let v1783: f64 = (if self.scalar_v477 { v1779 } else { v11 });
        let v1791: f64 = (self.scalar_v97 + v1783);
        let v1800: f64 = (v449 * v1791);
        let v1804: f64 = (v1800 / self.scalar_v97);
        let v1807: f64 = (self.scalar_v470 + v1804);
        let v1810: f64 = (v117 * v1807);
        let v1814: f64 = (self.scalar_v461 + v1810);
        let v1817: f64 = (if self.scalar_v477 { v1814 } else { v11 });
        let v524: f64 = v1817;
        let v525: f64 = (if self.scalar_v477 { v524 } else { v474 });
        let v550: f64 = 5.5226012e-23;
        let v551: f64 = (v33 * v550);
        let v555: f64 = (v551 * self.scalar_v554);
        let v556: f64 = (v116 * v555);
        let v559: f64 = (v556 * self.scalar_v558);
        let v560: f64 = (if self.scalar_v549 { v559 } else { v11 });
        let v561: f64 = (v560 * v560);
        let v562: f64 = (v42 - v561);
        let v563: f64 = ((v562) as f64).sqrt();
        let v564: f64 = (if self.scalar_v549 { v563 } else { v11 });
        let v565: f64 = (-v560);
        let v566: f64 = 3.141592653589793;
        let v567: f64 = (v565 * v566);
        let v568: f64 = (if self.scalar_v549 { v567 } else { v11 });
        let v569: f64 = (v560 * v566);
        let v570: f64 = (if self.scalar_v549 { v569 } else { v11 });
        let v572: f64 = (-v360);
        let v574: f64 = ctx.node_voltage(nodes[12]);
        let v575: f64 = (self.scalar_v573 * v574);
        let v577: f64 = ctx.branch_current(branches[0]);
        let v578: f64 = (self.scalar_v576 * v577);
        let v579: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, v521);
        let v580: f64 = (if self.scalar_v458 { v579 } else { v11 });
        let v581: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, v500);
        let v582: f64 = (if self.scalar_v458 { v581 } else { v11 });
        let v584: f64 = (v9 * v525);
        let v585: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, v584);
        let v586: f64 = (if self.scalar_v583 { v585 } else { v11 });
        let v587: f64 = (v2 * v523);
        let v588: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 5, v587);
        let v589: f64 = (if self.scalar_v583 { v588 } else { v11 });
        let v591: f64 = ctx.node_voltage(nodes[1]);
        let v592: f64 = (v591 - v4);
        let v593: f64 = (self.scalar_v590 * v592);
        let v595: f64 = (v7 * self.scalar_v594);
        let v596: f64 = ctx.node_voltage(nodes[10]);
        let v597: f64 = (v4 - v596);
        let v598: f64 = (v119 * v597);
        let v599: f64 = (v596 - v1);
        let v600: f64 = (v599 / v377);
        let v601: f64 = (if self.scalar_v526 { v600 } else { v11 });
        let v603: f64 = ctx.node_voltage(nodes[9]);
        let v604: f64 = (v603 - v0);
        let v605: f64 = (self.scalar_v602 * v604);
        let v606: f64 = (v603 - v1);
        let v607: f64 = (v606 / self.scalar_v527);
        let v608: f64 = (if self.scalar_v528 { v607 } else { v11 });
        let v609: f64 = (v3 - v8);
        let v610: f64 = (v609 / self.scalar_v529);
        let v611: f64 = (if self.scalar_v530 { v610 } else { v11 });
        let v612: f64 = (v3 - v0);
        let v613: f64 = (v612 / self.scalar_v531);
        let v614: f64 = (if self.scalar_v532 { v613 } else { v11 });
        let v615: f64 = ctx.branch_current(branches[5]);
        let v616: f64 = (self.scalar_v533 * v615);
        let v617: f64 = (if self.scalar_v534 { v616 } else { v11 });
        let v618: f64 = ctx.branch_current(branches[6]);
        let v619: f64 = (self.scalar_v535 * v618);
        let v620: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 10, v619);
        let v621: f64 = (if self.scalar_v534 { v620 } else { v11 });
        let v624: f64 = ctx.branch_current(branches[8]);
        let v625: f64 = (self.scalar_v535 * v624);
        let v626: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 11, v625);
        let v627: f64 = (if self.scalar_v623 { v626 } else { v11 });
        let v628: f64 = ctx.branch_current(branches[10]);
        let v629: f64 = (v393 * v628);
        let v630: f64 = (if self.scalar_v537 { v629 } else { v11 });
        let v632: f64 = ctx.branch_current(branches[13]);
        let v633: f64 = (self.scalar_v631 * v632);
        let v634: f64 = ctx.branch_current(branches[14]);
        let v635: f64 = (v392 * v634);
        let v636: f64 = (if self.scalar_v540 { v635 } else { v11 });
        let v637: f64 = ctx.branch_current(branches[15]);
        let v638: f64 = (self.scalar_v541 * v637);
        let v639: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 13, v638);
        let v640: f64 = (if self.scalar_v540 { v639 } else { v11 });
        let v643: f64 = ctx.branch_current(branches[17]);
        let v644: f64 = (self.scalar_v541 * v643);
        let v645: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 14, v644);
        let v646: f64 = (if self.scalar_v642 { v645 } else { v11 });
        let v647: f64 = ctx.node_voltage(nodes[14]);
        let v648: f64 = (if self.scalar_v549 { v647 } else { v11 });
        let v649: f64 = ctx.node_voltage(nodes[15]);
        let v650: f64 = (if self.scalar_v549 { v649 } else { v11 });
        let v651: f64 = (v568 * v647);
        let v652: f64 = (v564 * v649);
        let v653: f64 = (v651 + v652);
        let v654: f64 = (if self.scalar_v549 { v653 } else { v11 });
        let v655: f64 = (-v570);
        let v656: f64 = (v647 * v655);
        let v657: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 15, v656);
        let v658: f64 = (if self.scalar_v549 { v657 } else { v11 });
        let v660: f64 = (v30 * self.scalar_v659);
        let v661: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 16, v660);
        let v662: f64 = (if self.scalar_v571 { v661 } else { v11 });
        let v663: f64 = (-v10);
        let v664: f64 = (v7 * v663);
        let v665: f64 = (v2 * v425);
        let v666: f64 = (v664 + v665);
        let v667: f64 = ((v666) as f64).abs();
        let v668: f64 = (-v667);
        let v669: f64 = (if self.scalar_v571 { v668 } else { v11 });
        let v670: f64 = (v30 / self.scalar_v39);
        let v671: f64 = (if self.scalar_v571 { v670 } else { v11 });
        let v673: f64 = 1e-12;
        let v674: f64 = (v30 * v673);
        let v675: f64 = (if self.scalar_v672 { v674 } else { v11 });
        let v677: f64 = ((v138) as f64).sinh();
        let v678: f64 = (self.scalar_v137 * v677);
        let v679: f64 = (self.scalar_v676 * v677);
        let v680: f64 = (v139 * v678);
        let v681: f64 = (v680 + v680);
        let v682: f64 = (v139 * v679);
        let v683: f64 = (v682 + v682);
        let v684: f64 = (self.scalar_v140 * v681);
        let v685: f64 = (-v684);
        let v686: f64 = (v141 * v141);
        let v687: f64 = (v685 / v686);
        let v688: f64 = (self.scalar_v140 * v683);
        let v689: f64 = (-v688);
        let v690: f64 = (v689 / v686);
        let v691: f64 = (v114 * v687);
        let v692: f64 = (v114 * v690);
        let v694: f64 = (v149 * v149);
        let v695: f64 = (v42 - v694);
        let v696: f64 = (self.scalar_v147 * v695);
        let v697: f64 = (self.scalar_v693 * v695);
        let v698: f64 = (self.scalar_v145 * v696);
        let v699: f64 = (self.scalar_v145 * v697);
        let v701: f64 = (self.scalar_v152 * v155);
        let v702: f64 = (v154 + v701);
        let v703: f64 = (v155 * self.scalar_v700);
        let v704: f64 = (-v154);
        let v705: f64 = (v703 + v704);
        let v706: f64 = (v698 - v702);
        let v707: f64 = (-v705);
        let v708: f64 = (-v706);
        let v709: f64 = (v396 - v699);
        let v710: f64 = (v158 * v708);
        let v711: f64 = (v710 + v710);
        let v712: f64 = (v158 * v705);
        let v713: f64 = (v712 + v712);
        let v714: f64 = (v158 * v709);
        let v715: f64 = (v714 + v714);
        let v716: f64 = (v158 + v158);
        let v717: f64 = (v158 * v691);
        let v718: f64 = (v144 * v708);
        let v719: f64 = (v717 + v718);
        let v720: f64 = (v144 * v705);
        let v721: f64 = (v158 * v692);
        let v722: f64 = (v144 * v709);
        let v723: f64 = (v721 + v722);
        let v724: f64 = (self.scalar_v161 * v711);
        let v725: f64 = (self.scalar_v161 * v713);
        let v726: f64 = (self.scalar_v161 * v715);
        let v727: f64 = (self.scalar_v161 * v716);
        let v728: f64 = (v719 + v724);
        let v729: f64 = (v720 + v725);
        let v730: f64 = (v723 + v726);
        let v731: f64 = (v144 + v727);
        let v732: f64 = (self.scalar_v164 * v708);
        let v733: f64 = (self.scalar_v164 * v705);
        let v734: f64 = (self.scalar_v164 * v709);
        let v735: f64 = (v165 * v711);
        let v736: f64 = (v159 * v732);
        let v737: f64 = (v735 + v736);
        let v738: f64 = (v165 * v713);
        let v739: f64 = (v159 * v733);
        let v740: f64 = (v738 + v739);
        let v741: f64 = (v165 * v715);
        let v742: f64 = (v159 * v734);
        let v743: f64 = (v741 + v742);
        let v744: f64 = (v165 * v716);
        let v745: f64 = (v159 * self.scalar_v164);
        let v746: f64 = (v744 + v745);
        let v747: f64 = (v728 + v737);
        let v748: f64 = (v729 + v740);
        let v749: f64 = (v730 + v743);
        let v750: f64 = (v731 + v746);
        let v751: f64 = (v168 * v168);
        let v752: f64 = (v42 - v751);
        let v753: f64 = (v747 * v752);
        let v754: f64 = (v748 * v752);
        let v755: f64 = (v749 * v752);
        let v756: f64 = (v750 * v752);
        let v757: f64 = { let limexp_arg = v167; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v758: f64 = (v747 * v757);
        let v759: f64 = (v748 * v757);
        let v760: f64 = (v749 * v757);
        let v761: f64 = (v750 * v757);
        let v762: f64 = (-v747);
        let v763: f64 = (-v748);
        let v764: f64 = (-v749);
        let v765: f64 = (-v750);
        let v766: f64 = { let limexp_arg = v171; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v767: f64 = (v762 * v766);
        let v768: f64 = (v763 * v766);
        let v769: f64 = (v764 * v766);
        let v770: f64 = (v765 * v766);
        let v771: f64 = (v758 - v767);
        let v772: f64 = (v759 - v768);
        let v773: f64 = (v760 - v769);
        let v774: f64 = (v761 - v770);
        let v775: f64 = (v129 * v771);
        let v776: f64 = (v129 * v772);
        let v777: f64 = (v129 * v773);
        let v778: f64 = (v129 * v774);
        let v779: f64 = (v175 * v175);
        let v780: f64 = (v42 - v779);
        let v781: f64 = (v775 * v780);
        let v782: f64 = (v776 * v780);
        let v783: f64 = (v777 * v780);
        let v784: f64 = (v778 * v780);
        let v785: f64 = (self.scalar_v147 * v753);
        let v786: f64 = (self.scalar_v147 * v754);
        let v787: f64 = (self.scalar_v147 * v755);
        let v788: f64 = (self.scalar_v147 * v756);
        let v789: f64 = (v7 * v785);
        let v790: f64 = (v179 + v789);
        let v791: f64 = (v7 * v786);
        let v792: f64 = (-v179);
        let v793: f64 = (v7 * v787);
        let v794: f64 = (v792 + v793);
        let v795: f64 = (v7 * v788);
        let v796: f64 = (v181 * v181);
        let v797: f64 = (v42 - v796);
        let v798: f64 = (v790 * v797);
        let v799: f64 = (v791 * v797);
        let v800: f64 = (v794 * v797);
        let v801: f64 = (v795 * v797);
        let v802: f64 = (v113 * v753);
        let v803: f64 = (v113 * v754);
        let v804: f64 = (v113 * v755);
        let v805: f64 = (v113 * v756);
        let v806: f64 = (v189 * v798);
        let v807: f64 = (v181 * v802);
        let v808: f64 = (v806 + v807);
        let v809: f64 = (v189 * v799);
        let v810: f64 = (v181 * v803);
        let v811: f64 = (v809 + v810);
        let v812: f64 = (v189 * v800);
        let v813: f64 = (v181 * v804);
        let v814: f64 = (v812 + v813);
        let v815: f64 = (v189 * v801);
        let v816: f64 = (v181 * v805);
        let v817: f64 = (v815 + v816);
        let v819: f64 = { let limexp_arg = v155; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v820: f64 = (-v819);
        let v821: f64 = (v115 * v819);
        let v822: f64 = (v115 * v820);
        let v823: f64 = (self.scalar_v191 + v821);
        let v824: f64 = (v196 * v808);
        let v825: f64 = (v190 * v823);
        let v826: f64 = (v824 + v825);
        let v827: f64 = (v196 * v811);
        let v828: f64 = (v190 * v822);
        let v829: f64 = (v827 + v828);
        let v830: f64 = (v196 * v814);
        let v831: f64 = (v190 * self.scalar_v818);
        let v832: f64 = (v830 + v831);
        let v833: f64 = (v196 * v817);
        let v834: f64 = (if self.scalar_v183 { v826 } else { v11 });
        let v835: f64 = (if self.scalar_v183 { v829 } else { v11 });
        let v836: f64 = (if self.scalar_v183 { v832 } else { v11 });
        let v837: f64 = (if self.scalar_v183 { v833 } else { v11 });
        let v838: f64 = (v396 - v706);
        let v839: f64 = (v42 - v707);
        let v840: f64 = (-v699);
        let v841: f64 = (if self.scalar_v200 { v838 } else { v678 });
        let v842: f64 = (if self.scalar_v200 { v839 } else { v11 });
        let v843: f64 = (if self.scalar_v200 { v840 } else { v679 });
        let v844: f64 = (v202 * v841);
        let v845: f64 = (v844 + v844);
        let v846: f64 = (v202 * v842);
        let v847: f64 = (v846 + v846);
        let v848: f64 = (v202 * v843);
        let v849: f64 = (v848 + v848);
        let v850: f64 = (if self.scalar_v200 { v845 } else { v708 });
        let v851: f64 = (if self.scalar_v200 { v847 } else { v705 });
        let v852: f64 = (if self.scalar_v200 { v849 } else { v709 });
        let v854: f64 = (v204 * v841);
        let v855: f64 = (v202 * v850);
        let v856: f64 = (v854 + v855);
        let v857: f64 = (v204 * v842);
        let v858: f64 = (v202 * v851);
        let v859: f64 = (v857 + v858);
        let v860: f64 = (v204 * v843);
        let v861: f64 = (v202 * v852);
        let v862: f64 = (v860 + v861);
        let v863: f64 = (v202 * self.scalar_v853);
        let v864: f64 = (if self.scalar_v200 { v856 } else { v711 });
        let v865: f64 = (if self.scalar_v200 { v859 } else { v713 });
        let v866: f64 = (if self.scalar_v200 { v862 } else { v715 });
        let v867: f64 = (if self.scalar_v200 { v863 } else { v716 });
        let v868: f64 = (v202 * v691);
        let v869: f64 = (v144 * v841);
        let v870: f64 = (v868 + v869);
        let v871: f64 = (v144 * v842);
        let v872: f64 = (v202 * v692);
        let v873: f64 = (v144 * v843);
        let v874: f64 = (v872 + v873);
        let v875: f64 = (self.scalar_v161 * v850);
        let v876: f64 = (self.scalar_v161 * v851);
        let v877: f64 = (self.scalar_v161 * v852);
        let v879: f64 = (v870 + v875);
        let v880: f64 = (v871 + v876);
        let v881: f64 = (v874 + v877);
        let v882: f64 = (self.scalar_v164 * v864);
        let v883: f64 = (self.scalar_v164 * v865);
        let v884: f64 = (self.scalar_v164 * v866);
        let v885: f64 = (self.scalar_v164 * v867);
        let v886: f64 = (v879 + v882);
        let v887: f64 = (v880 + v883);
        let v888: f64 = (v881 + v884);
        let v889: f64 = (self.scalar_v878 + v885);
        let v890: f64 = (if self.scalar_v200 { v886 } else { v11 });
        let v891: f64 = (if self.scalar_v200 { v887 } else { v11 });
        let v892: f64 = (if self.scalar_v200 { v888 } else { v11 });
        let v893: f64 = (if self.scalar_v200 { v889 } else { v11 });
        let v894: f64 = (v213 * v213);
        let v895: f64 = (v42 - v894);
        let v896: f64 = (v890 * v895);
        let v897: f64 = (v891 * v895);
        let v898: f64 = (v892 * v895);
        let v899: f64 = (v893 * v895);
        let v900: f64 = (if self.scalar_v200 { v896 } else { v11 });
        let v901: f64 = (if self.scalar_v200 { v897 } else { v11 });
        let v902: f64 = (if self.scalar_v200 { v898 } else { v11 });
        let v903: f64 = (if self.scalar_v200 { v899 } else { v11 });
        let v904: f64 = (self.scalar_v147 * v900);
        let v905: f64 = (self.scalar_v147 * v901);
        let v906: f64 = (self.scalar_v147 * v902);
        let v907: f64 = (self.scalar_v147 * v903);
        let v908: f64 = (if self.scalar_v200 { v904 } else { v11 });
        let v909: f64 = (if self.scalar_v200 { v905 } else { v11 });
        let v910: f64 = (if self.scalar_v200 { v906 } else { v11 });
        let v911: f64 = (if self.scalar_v200 { v907 } else { v11 });
        let v912: f64 = (self.scalar_v219 * v753);
        let v913: f64 = (self.scalar_v219 * v754);
        let v914: f64 = (self.scalar_v219 * v755);
        let v915: f64 = (self.scalar_v219 * v756);
        let v916: f64 = (if self.scalar_v200 { v912 } else { v11 });
        let v917: f64 = (if self.scalar_v200 { v913 } else { v11 });
        let v918: f64 = (if self.scalar_v200 { v914 } else { v11 });
        let v919: f64 = (if self.scalar_v200 { v915 } else { v11 });
        let v920: f64 = (v223 * v802);
        let v921: f64 = (v806 + v920);
        let v922: f64 = (v223 * v803);
        let v923: f64 = (v809 + v922);
        let v924: f64 = (v223 * v804);
        let v925: f64 = (v812 + v924);
        let v926: f64 = (v223 * v805);
        let v927: f64 = (v815 + v926);
        let v928: f64 = (v7 * v916);
        let v929: f64 = (v222 + v928);
        let v930: f64 = (v7 * v917);
        let v931: f64 = (-v222);
        let v932: f64 = (v7 * v918);
        let v933: f64 = (v931 + v932);
        let v934: f64 = (v7 * v919);
        let v935: f64 = { let limexp_arg = v227; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v936: f64 = (-v935);
        let v937: f64 = (v115 * v935);
        let v938: f64 = (v115 * v936);
        let v939: f64 = (v929 + v937);
        let v940: f64 = (v933 + v938);
        let v941: f64 = (v230 * v921);
        let v942: f64 = (v224 * v939);
        let v943: f64 = (v941 + v942);
        let v944: f64 = (v230 * v923);
        let v945: f64 = (v224 * v930);
        let v946: f64 = (v944 + v945);
        let v947: f64 = (v230 * v925);
        let v948: f64 = (v224 * v940);
        let v949: f64 = (v947 + v948);
        let v950: f64 = (v230 * v927);
        let v951: f64 = (v224 * v934);
        let v952: f64 = (v950 + v951);
        let v953: f64 = (if self.scalar_v200 { v943 } else { v11 });
        let v954: f64 = (if self.scalar_v200 { v946 } else { v11 });
        let v955: f64 = (if self.scalar_v200 { v949 } else { v11 });
        let v956: f64 = (if self.scalar_v200 { v952 } else { v11 });
        let v957: f64 = (self.scalar_v219 * v900);
        let v958: f64 = (self.scalar_v219 * v901);
        let v959: f64 = (self.scalar_v219 * v902);
        let v960: f64 = (self.scalar_v219 * v903);
        let v961: f64 = (if self.scalar_v200 { v957 } else { v11 });
        let v962: f64 = (if self.scalar_v200 { v958 } else { v11 });
        let v963: f64 = (if self.scalar_v200 { v959 } else { v11 });
        let v964: f64 = (if self.scalar_v200 { v960 } else { v11 });
        let v965: f64 = (v7 * v908);
        let v966: f64 = (v218 + v965);
        let v967: f64 = (v7 * v909);
        let v968: f64 = (-v218);
        let v969: f64 = (v7 * v910);
        let v970: f64 = (v968 + v969);
        let v971: f64 = (v7 * v911);
        let v972: f64 = (v237 * v237);
        let v973: f64 = (v42 - v972);
        let v974: f64 = (v966 * v973);
        let v975: f64 = (v967 * v973);
        let v976: f64 = (v970 * v973);
        let v977: f64 = (v971 * v973);
        let v978: f64 = (if self.scalar_v200 { v974 } else { v11 });
        let v979: f64 = (if self.scalar_v200 { v975 } else { v11 });
        let v980: f64 = (if self.scalar_v200 { v976 } else { v11 });
        let v981: f64 = (if self.scalar_v200 { v977 } else { v11 });
        let v982: f64 = (v113 * v900);
        let v983: f64 = (v113 * v901);
        let v984: f64 = (v113 * v902);
        let v985: f64 = (v113 * v903);
        let v986: f64 = (-v978);
        let v987: f64 = (-v979);
        let v988: f64 = (-v980);
        let v989: f64 = (-v981);
        let v990: f64 = (v240 * v982);
        let v991: f64 = (v239 * v986);
        let v992: f64 = (v990 + v991);
        let v993: f64 = (v240 * v983);
        let v994: f64 = (v239 * v987);
        let v995: f64 = (v993 + v994);
        let v996: f64 = (v240 * v984);
        let v997: f64 = (v239 * v988);
        let v998: f64 = (v996 + v997);
        let v999: f64 = (v240 * v985);
        let v1000: f64 = (v239 * v989);
        let v1001: f64 = (v999 + v1000);
        let v1002: f64 = (v7 * v961);
        let v1003: f64 = (v235 + v1002);
        let v1004: f64 = (v7 * v962);
        let v1005: f64 = (-v235);
        let v1006: f64 = (v7 * v963);
        let v1007: f64 = (v1005 + v1006);
        let v1008: f64 = (v7 * v964);
        let v1009: f64 = (-v1003);
        let v1010: f64 = (-v1004);
        let v1011: f64 = (-v1007);
        let v1012: f64 = (-v1008);
        let v1013: f64 = (v243 * v992);
        let v1014: f64 = (v241 * v1009);
        let v1015: f64 = (v1013 + v1014);
        let v1016: f64 = (v243 * v995);
        let v1017: f64 = (v241 * v1010);
        let v1018: f64 = (v1016 + v1017);
        let v1019: f64 = (v243 * v998);
        let v1020: f64 = (v241 * v1011);
        let v1021: f64 = (v1019 + v1020);
        let v1022: f64 = (v243 * v1001);
        let v1023: f64 = (v241 * v1012);
        let v1024: f64 = (v1022 + v1023);
        let v1025: f64 = (if self.scalar_v200 { v1015 } else { v11 });
        let v1026: f64 = (if self.scalar_v200 { v1018 } else { v11 });
        let v1027: f64 = (if self.scalar_v200 { v1021 } else { v11 });
        let v1028: f64 = (if self.scalar_v200 { v1024 } else { v11 });
        let v1029: f64 = (v953 - v1025);
        let v1030: f64 = (v954 - v1026);
        let v1031: f64 = (v955 - v1027);
        let v1032: f64 = (v956 - v1028);
        let v1033: f64 = (v129 * v1029);
        let v1034: f64 = (v129 * v1030);
        let v1035: f64 = (v129 * v1031);
        let v1036: f64 = (v129 * v1032);
        let v1037: f64 = (if self.scalar_v200 { v1033 } else { v834 });
        let v1038: f64 = (if self.scalar_v200 { v1034 } else { v835 });
        let v1039: f64 = (if self.scalar_v200 { v1035 } else { v836 });
        let v1040: f64 = (if self.scalar_v200 { v1036 } else { v837 });
        let v1041: f64 = (if self.scalar_v251 { v708 } else { v841 });
        let v1042: f64 = (if self.scalar_v251 { v705 } else { v842 });
        let v1043: f64 = (if self.scalar_v251 { v709 } else { v843 });
        let v1045: f64 = (v252 * v1041);
        let v1046: f64 = (v1045 + v1045);
        let v1047: f64 = (v252 * v1042);
        let v1048: f64 = (v1047 + v1047);
        let v1049: f64 = (v252 * v1043);
        let v1050: f64 = (v1049 + v1049);
        let v1051: f64 = (v252 * self.scalar_v1044);
        let v1052: f64 = (v1051 + v1051);
        let v1053: f64 = (if self.scalar_v251 { v1046 } else { v850 });
        let v1054: f64 = (if self.scalar_v251 { v1048 } else { v851 });
        let v1055: f64 = (if self.scalar_v251 { v1050 } else { v852 });
        let v1056: f64 = (if self.scalar_v251 { v1052 } else { self.scalar_v853 });
        let v1057: f64 = (self.scalar_v161 * v1053);
        let v1058: f64 = (self.scalar_v161 * v1054);
        let v1059: f64 = (self.scalar_v161 * v1055);
        let v1060: f64 = (self.scalar_v161 * v1056);
        let v1061: f64 = (v1041 + v1057);
        let v1062: f64 = (v1042 + v1058);
        let v1063: f64 = (v1043 + v1059);
        let v1064: f64 = (self.scalar_v1044 + v1060);
        let v1065: f64 = (self.scalar_v164 * v1053);
        let v1066: f64 = (self.scalar_v164 * v1054);
        let v1067: f64 = (self.scalar_v164 * v1055);
        let v1068: f64 = (self.scalar_v164 * v1056);
        let v1069: f64 = (v257 * v1041);
        let v1070: f64 = (v252 * v1065);
        let v1071: f64 = (v1069 + v1070);
        let v1072: f64 = (v257 * v1042);
        let v1073: f64 = (v252 * v1066);
        let v1074: f64 = (v1072 + v1073);
        let v1075: f64 = (v257 * v1043);
        let v1076: f64 = (v252 * v1067);
        let v1077: f64 = (v1075 + v1076);
        let v1078: f64 = (v257 * self.scalar_v1044);
        let v1079: f64 = (v252 * v1068);
        let v1080: f64 = (v1078 + v1079);
        let v1081: f64 = (v1061 + v1071);
        let v1082: f64 = (v1062 + v1074);
        let v1083: f64 = (v1063 + v1077);
        let v1084: f64 = (v1064 + v1080);
        let v1085: f64 = (v259 * v691);
        let v1086: f64 = (v144 * v1081);
        let v1087: f64 = (v1085 + v1086);
        let v1088: f64 = (v144 * v1082);
        let v1089: f64 = (v259 * v692);
        let v1090: f64 = (v144 * v1083);
        let v1091: f64 = (v1089 + v1090);
        let v1092: f64 = (v144 * v1084);
        let v1093: f64 = (if self.scalar_v251 { v1087 } else { v747 });
        let v1094: f64 = (if self.scalar_v251 { v1088 } else { v748 });
        let v1095: f64 = (if self.scalar_v251 { v1091 } else { v749 });
        let v1096: f64 = (if self.scalar_v251 { v1092 } else { v750 });
        let v1097: f64 = { let limexp_arg = v261; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v1098: f64 = (v1093 * v1097);
        let v1099: f64 = (v1094 * v1097);
        let v1100: f64 = (v1095 * v1097);
        let v1101: f64 = (v1096 * v1097);
        let v1102: f64 = (-v1093);
        let v1103: f64 = (-v1094);
        let v1104: f64 = (-v1095);
        let v1105: f64 = (-v1096);
        let v1106: f64 = { let limexp_arg = v263; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v1107: f64 = (v1102 * v1106);
        let v1108: f64 = (v1103 * v1106);
        let v1109: f64 = (v1104 * v1106);
        let v1110: f64 = (v1105 * v1106);
        let v1111: f64 = (v1098 - v1107);
        let v1112: f64 = (v1099 - v1108);
        let v1113: f64 = (v1100 - v1109);
        let v1114: f64 = (v1101 - v1110);
        let v1115: f64 = (v129 * v1111);
        let v1116: f64 = (v129 * v1112);
        let v1117: f64 = (v129 * v1113);
        let v1118: f64 = (v129 * v1114);
        let v1119: f64 = (v267 * v267);
        let v1120: f64 = (v42 - v1119);
        let v1121: f64 = (v1115 * v1120);
        let v1122: f64 = (v1116 * v1120);
        let v1123: f64 = (v1117 * v1120);
        let v1124: f64 = (v1118 * v1120);
        let v1125: f64 = (if self.scalar_v251 { v1121 } else { v781 });
        let v1126: f64 = (if self.scalar_v251 { v1122 } else { v782 });
        let v1127: f64 = (if self.scalar_v251 { v1123 } else { v783 });
        let v1128: f64 = (if self.scalar_v251 { v1124 } else { v784 });
        let v1129: f64 = (self.scalar_v147 * v1125);
        let v1130: f64 = (self.scalar_v147 * v1126);
        let v1131: f64 = (self.scalar_v147 * v1127);
        let v1132: f64 = (self.scalar_v147 * v1128);
        let v1133: f64 = (if self.scalar_v251 { v1129 } else { v11 });
        let v1134: f64 = (if self.scalar_v251 { v1130 } else { v11 });
        let v1135: f64 = (if self.scalar_v251 { v1131 } else { v11 });
        let v1136: f64 = (if self.scalar_v251 { v1132 } else { v11 });
        let v1137: f64 = (v7 * v1133);
        let v1138: f64 = (v272 + v1137);
        let v1139: f64 = (v7 * v1134);
        let v1140: f64 = (-v272);
        let v1141: f64 = (v7 * v1135);
        let v1142: f64 = (v1140 + v1141);
        let v1143: f64 = (v7 * v1136);
        let v1144: f64 = (v274 * v274);
        let v1145: f64 = (v42 - v1144);
        let v1146: f64 = (v1138 * v1145);
        let v1147: f64 = (v1139 * v1145);
        let v1148: f64 = (v1142 * v1145);
        let v1149: f64 = (v1143 * v1145);
        let v1150: f64 = (if self.scalar_v251 { v1146 } else { v11 });
        let v1151: f64 = (if self.scalar_v251 { v1147 } else { v11 });
        let v1152: f64 = (if self.scalar_v251 { v1148 } else { v11 });
        let v1153: f64 = (if self.scalar_v251 { v1149 } else { v11 });
        let v1154: f64 = (self.scalar_v219 * v1125);
        let v1155: f64 = (self.scalar_v219 * v1126);
        let v1156: f64 = (self.scalar_v219 * v1127);
        let v1157: f64 = (self.scalar_v219 * v1128);
        let v1158: f64 = (if self.scalar_v251 { v1154 } else { v916 });
        let v1159: f64 = (if self.scalar_v251 { v1155 } else { v917 });
        let v1160: f64 = (if self.scalar_v251 { v1156 } else { v918 });
        let v1161: f64 = (if self.scalar_v251 { v1157 } else { v919 });
        let v1162: f64 = (v113 * v1125);
        let v1163: f64 = (v113 * v1126);
        let v1164: f64 = (v113 * v1127);
        let v1165: f64 = (v113 * v1128);
        let v1166: f64 = (v279 * v1150);
        let v1167: f64 = (v275 * v1162);
        let v1168: f64 = (v1166 + v1167);
        let v1169: f64 = (v279 * v1151);
        let v1170: f64 = (v275 * v1163);
        let v1171: f64 = (v1169 + v1170);
        let v1172: f64 = (v279 * v1152);
        let v1173: f64 = (v275 * v1164);
        let v1174: f64 = (v1172 + v1173);
        let v1175: f64 = (v279 * v1153);
        let v1176: f64 = (v275 * v1165);
        let v1177: f64 = (v1175 + v1176);
        let v1178: f64 = (v7 * v1158);
        let v1179: f64 = (v278 + v1178);
        let v1180: f64 = (v7 * v1159);
        let v1181: f64 = (-v278);
        let v1182: f64 = (v7 * v1160);
        let v1183: f64 = (v1181 + v1182);
        let v1184: f64 = (v7 * v1161);
        let v1185: f64 = (v821 + v1179);
        let v1186: f64 = (v822 + v1180);
        let v1187: f64 = (v283 * v1168);
        let v1188: f64 = (v280 * v1185);
        let v1189: f64 = (v1187 + v1188);
        let v1190: f64 = (v283 * v1171);
        let v1191: f64 = (v280 * v1186);
        let v1192: f64 = (v1190 + v1191);
        let v1193: f64 = (v283 * v1174);
        let v1194: f64 = (v280 * v1183);
        let v1195: f64 = (v1193 + v1194);
        let v1196: f64 = (v283 * v1177);
        let v1197: f64 = (v280 * v1184);
        let v1198: f64 = (v1196 + v1197);
        let v1199: f64 = (if self.scalar_v251 { v1189 } else { v1037 });
        let v1200: f64 = (if self.scalar_v251 { v1192 } else { v1038 });
        let v1201: f64 = (if self.scalar_v251 { v1195 } else { v1039 });
        let v1202: f64 = (if self.scalar_v251 { v1198 } else { v1040 });
        let v1203: f64 = (if self.scalar_v288 { v708 } else { v1041 });
        let v1204: f64 = (if self.scalar_v288 { v705 } else { v1042 });
        let v1205: f64 = (if self.scalar_v288 { v709 } else { v1043 });
        let v1207: f64 = (v289 * v1203);
        let v1208: f64 = (v1207 + v1207);
        let v1209: f64 = (v289 * v1204);
        let v1210: f64 = (v1209 + v1209);
        let v1211: f64 = (v289 * v1205);
        let v1212: f64 = (v1211 + v1211);
        let v1213: f64 = (v289 * self.scalar_v1206);
        let v1214: f64 = (v1213 + v1213);
        let v1215: f64 = (if self.scalar_v288 { v1208 } else { v1053 });
        let v1216: f64 = (if self.scalar_v288 { v1210 } else { v1054 });
        let v1217: f64 = (if self.scalar_v288 { v1212 } else { v1055 });
        let v1218: f64 = (if self.scalar_v288 { v1214 } else { v1056 });
        let v1219: f64 = (self.scalar_v161 * v1215);
        let v1220: f64 = (self.scalar_v161 * v1216);
        let v1221: f64 = (self.scalar_v161 * v1217);
        let v1222: f64 = (self.scalar_v161 * v1218);
        let v1223: f64 = (v1203 + v1219);
        let v1224: f64 = (v1204 + v1220);
        let v1225: f64 = (v1205 + v1221);
        let v1226: f64 = (self.scalar_v1206 + v1222);
        let v1227: f64 = (self.scalar_v164 * v1215);
        let v1228: f64 = (self.scalar_v164 * v1216);
        let v1229: f64 = (self.scalar_v164 * v1217);
        let v1230: f64 = (self.scalar_v164 * v1218);
        let v1231: f64 = (v294 * v1203);
        let v1232: f64 = (v289 * v1227);
        let v1233: f64 = (v1231 + v1232);
        let v1234: f64 = (v294 * v1204);
        let v1235: f64 = (v289 * v1228);
        let v1236: f64 = (v1234 + v1235);
        let v1237: f64 = (v294 * v1205);
        let v1238: f64 = (v289 * v1229);
        let v1239: f64 = (v1237 + v1238);
        let v1240: f64 = (v294 * self.scalar_v1206);
        let v1241: f64 = (v289 * v1230);
        let v1242: f64 = (v1240 + v1241);
        let v1243: f64 = (v1223 + v1233);
        let v1244: f64 = (v1224 + v1236);
        let v1245: f64 = (v1225 + v1239);
        let v1246: f64 = (v1226 + v1242);
        let v1247: f64 = (v296 * v691);
        let v1248: f64 = (v144 * v1243);
        let v1249: f64 = (v1247 + v1248);
        let v1250: f64 = (v144 * v1244);
        let v1251: f64 = (v296 * v692);
        let v1252: f64 = (v144 * v1245);
        let v1253: f64 = (v1251 + v1252);
        let v1254: f64 = (v144 * v1246);
        let v1255: f64 = (if self.scalar_v288 { v1249 } else { v1093 });
        let v1256: f64 = (if self.scalar_v288 { v1250 } else { v1094 });
        let v1257: f64 = (if self.scalar_v288 { v1253 } else { v1095 });
        let v1258: f64 = (if self.scalar_v288 { v1254 } else { v1096 });
        let v1259: f64 = (if self.scalar_v288 { v838 } else { v864 });
        let v1260: f64 = (if self.scalar_v288 { v839 } else { v865 });
        let v1261: f64 = (if self.scalar_v288 { v840 } else { v866 });
        let v1262: f64 = (if self.scalar_v288 { v11 } else { v867 });
        let v1263: f64 = (v299 * v1259);
        let v1264: f64 = (v1263 + v1263);
        let v1265: f64 = (v299 * v1260);
        let v1266: f64 = (v1265 + v1265);
        let v1267: f64 = (v299 * v1261);
        let v1268: f64 = (v1267 + v1267);
        let v1269: f64 = (v299 * v1262);
        let v1270: f64 = (v1269 + v1269);
        let v1271: f64 = (if self.scalar_v288 { v1264 } else { v11 });
        let v1272: f64 = (if self.scalar_v288 { v1266 } else { v11 });
        let v1273: f64 = (if self.scalar_v288 { v1268 } else { v11 });
        let v1274: f64 = (if self.scalar_v288 { v1270 } else { v11 });
        let v1275: f64 = (self.scalar_v161 * v1271);
        let v1276: f64 = (self.scalar_v161 * v1272);
        let v1277: f64 = (self.scalar_v161 * v1273);
        let v1278: f64 = (self.scalar_v161 * v1274);
        let v1279: f64 = (v1259 + v1275);
        let v1280: f64 = (v1260 + v1276);
        let v1281: f64 = (v1261 + v1277);
        let v1282: f64 = (v1262 + v1278);
        let v1283: f64 = (self.scalar_v164 * v1259);
        let v1284: f64 = (self.scalar_v164 * v1260);
        let v1285: f64 = (self.scalar_v164 * v1261);
        let v1286: f64 = (self.scalar_v164 * v1262);
        let v1287: f64 = (v304 * v1271);
        let v1288: f64 = (v301 * v1283);
        let v1289: f64 = (v1287 + v1288);
        let v1290: f64 = (v304 * v1272);
        let v1291: f64 = (v301 * v1284);
        let v1292: f64 = (v1290 + v1291);
        let v1293: f64 = (v304 * v1273);
        let v1294: f64 = (v301 * v1285);
        let v1295: f64 = (v1293 + v1294);
        let v1296: f64 = (v304 * v1274);
        let v1297: f64 = (v301 * v1286);
        let v1298: f64 = (v1296 + v1297);
        let v1299: f64 = (v1279 + v1289);
        let v1300: f64 = (v1280 + v1292);
        let v1301: f64 = (v1281 + v1295);
        let v1302: f64 = (v1282 + v1298);
        let v1303: f64 = (v306 * v691);
        let v1304: f64 = (v144 * v1299);
        let v1305: f64 = (v1303 + v1304);
        let v1306: f64 = (v144 * v1300);
        let v1307: f64 = (v306 * v692);
        let v1308: f64 = (v144 * v1301);
        let v1309: f64 = (v1307 + v1308);
        let v1310: f64 = (v144 * v1302);
        let v1311: f64 = (if self.scalar_v288 { v1305 } else { v890 });
        let v1312: f64 = (if self.scalar_v288 { v1306 } else { v891 });
        let v1313: f64 = (if self.scalar_v288 { v1309 } else { v892 });
        let v1314: f64 = (if self.scalar_v288 { v1310 } else { v893 });
        let v1315: f64 = { let limexp_arg = v298; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v1316: f64 = (v1255 * v1315);
        let v1317: f64 = (v1256 * v1315);
        let v1318: f64 = (v1257 * v1315);
        let v1319: f64 = (v1258 * v1315);
        let v1320: f64 = (-v1255);
        let v1321: f64 = (-v1256);
        let v1322: f64 = (-v1257);
        let v1323: f64 = (-v1258);
        let v1324: f64 = { let limexp_arg = v310; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v1325: f64 = (v1320 * v1324);
        let v1326: f64 = (v1321 * v1324);
        let v1327: f64 = (v1322 * v1324);
        let v1328: f64 = (v1323 * v1324);
        let v1329: f64 = (v1316 - v1325);
        let v1330: f64 = (v1317 - v1326);
        let v1331: f64 = (v1318 - v1327);
        let v1332: f64 = (v1319 - v1328);
        let v1333: f64 = (v129 * v1329);
        let v1334: f64 = (v129 * v1330);
        let v1335: f64 = (v129 * v1331);
        let v1336: f64 = (v129 * v1332);
        let v1337: f64 = (v314 * v314);
        let v1338: f64 = (v42 - v1337);
        let v1339: f64 = (v1333 * v1338);
        let v1340: f64 = (v1334 * v1338);
        let v1341: f64 = (v1335 * v1338);
        let v1342: f64 = (v1336 * v1338);
        let v1343: f64 = (if self.scalar_v288 { v1339 } else { v1125 });
        let v1344: f64 = (if self.scalar_v288 { v1340 } else { v1126 });
        let v1345: f64 = (if self.scalar_v288 { v1341 } else { v1127 });
        let v1346: f64 = (if self.scalar_v288 { v1342 } else { v1128 });
        let v1347: f64 = { let limexp_arg = v308; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v1348: f64 = (v1311 * v1347);
        let v1349: f64 = (v1312 * v1347);
        let v1350: f64 = (v1313 * v1347);
        let v1351: f64 = (v1314 * v1347);
        let v1352: f64 = (-v1311);
        let v1353: f64 = (-v1312);
        let v1354: f64 = (-v1313);
        let v1355: f64 = (-v1314);
        let v1356: f64 = { let limexp_arg = v318; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v1357: f64 = (v1352 * v1356);
        let v1358: f64 = (v1353 * v1356);
        let v1359: f64 = (v1354 * v1356);
        let v1360: f64 = (v1355 * v1356);
        let v1361: f64 = (v1348 - v1357);
        let v1362: f64 = (v1349 - v1358);
        let v1363: f64 = (v1350 - v1359);
        let v1364: f64 = (v1351 - v1360);
        let v1365: f64 = (v129 * v1361);
        let v1366: f64 = (v129 * v1362);
        let v1367: f64 = (v129 * v1363);
        let v1368: f64 = (v129 * v1364);
        let v1369: f64 = (v322 * v322);
        let v1370: f64 = (v42 - v1369);
        let v1371: f64 = (v1365 * v1370);
        let v1372: f64 = (v1366 * v1370);
        let v1373: f64 = (v1367 * v1370);
        let v1374: f64 = (v1368 * v1370);
        let v1375: f64 = (if self.scalar_v288 { v1371 } else { v11 });
        let v1376: f64 = (if self.scalar_v288 { v1372 } else { v11 });
        let v1377: f64 = (if self.scalar_v288 { v1373 } else { v11 });
        let v1378: f64 = (if self.scalar_v288 { v1374 } else { v11 });
        let v1379: f64 = (self.scalar_v147 * v1343);
        let v1380: f64 = (self.scalar_v147 * v1344);
        let v1381: f64 = (self.scalar_v147 * v1345);
        let v1382: f64 = (self.scalar_v147 * v1346);
        let v1383: f64 = (if self.scalar_v288 { v1379 } else { v1133 });
        let v1384: f64 = (if self.scalar_v288 { v1380 } else { v1134 });
        let v1385: f64 = (if self.scalar_v288 { v1381 } else { v1135 });
        let v1386: f64 = (if self.scalar_v288 { v1382 } else { v1136 });
        let v1387: f64 = (self.scalar_v147 * v1375);
        let v1388: f64 = (self.scalar_v147 * v1376);
        let v1389: f64 = (self.scalar_v147 * v1377);
        let v1390: f64 = (self.scalar_v147 * v1378);
        let v1391: f64 = (if self.scalar_v288 { v1387 } else { v11 });
        let v1392: f64 = (if self.scalar_v288 { v1388 } else { v11 });
        let v1393: f64 = (if self.scalar_v288 { v1389 } else { v11 });
        let v1394: f64 = (if self.scalar_v288 { v1390 } else { v11 });
        let v1395: f64 = (v7 * v1383);
        let v1396: f64 = (v327 + v1395);
        let v1397: f64 = (v7 * v1384);
        let v1398: f64 = (-v327);
        let v1399: f64 = (v7 * v1385);
        let v1400: f64 = (v1398 + v1399);
        let v1401: f64 = (v7 * v1386);
        let v1402: f64 = (v332 * v332);
        let v1403: f64 = (v42 - v1402);
        let v1404: f64 = (v1396 * v1403);
        let v1405: f64 = (v1397 * v1403);
        let v1406: f64 = (v1400 * v1403);
        let v1407: f64 = (v1401 * v1403);
        let v1408: f64 = (if self.scalar_v288 { v1404 } else { v1150 });
        let v1409: f64 = (if self.scalar_v288 { v1405 } else { v1151 });
        let v1410: f64 = (if self.scalar_v288 { v1406 } else { v1152 });
        let v1411: f64 = (if self.scalar_v288 { v1407 } else { v1153 });
        let v1412: f64 = (v7 * v1391);
        let v1413: f64 = (v330 + v1412);
        let v1414: f64 = (v7 * v1392);
        let v1415: f64 = (-v330);
        let v1416: f64 = (v7 * v1393);
        let v1417: f64 = (v1415 + v1416);
        let v1418: f64 = (v7 * v1394);
        let v1419: f64 = (v335 * v335);
        let v1420: f64 = (v42 - v1419);
        let v1421: f64 = (v1413 * v1420);
        let v1422: f64 = (v1414 * v1420);
        let v1423: f64 = (v1417 * v1420);
        let v1424: f64 = (v1418 * v1420);
        let v1425: f64 = (if self.scalar_v288 { v1421 } else { v11 });
        let v1426: f64 = (if self.scalar_v288 { v1422 } else { v11 });
        let v1427: f64 = (if self.scalar_v288 { v1423 } else { v11 });
        let v1428: f64 = (if self.scalar_v288 { v1424 } else { v11 });
        let v1429: f64 = (self.scalar_v219 * v1375);
        let v1430: f64 = (self.scalar_v219 * v1376);
        let v1431: f64 = (self.scalar_v219 * v1377);
        let v1432: f64 = (self.scalar_v219 * v1378);
        let v1433: f64 = (if self.scalar_v288 { v1429 } else { v11 });
        let v1434: f64 = (if self.scalar_v288 { v1430 } else { v11 });
        let v1435: f64 = (if self.scalar_v288 { v1431 } else { v11 });
        let v1436: f64 = (if self.scalar_v288 { v1432 } else { v11 });
        let v1437: f64 = (self.scalar_v219 * v1343);
        let v1438: f64 = (self.scalar_v219 * v1344);
        let v1439: f64 = (self.scalar_v219 * v1345);
        let v1440: f64 = (self.scalar_v219 * v1346);
        let v1441: f64 = (if self.scalar_v288 { v1437 } else { v11 });
        let v1442: f64 = (if self.scalar_v288 { v1438 } else { v11 });
        let v1443: f64 = (if self.scalar_v288 { v1439 } else { v11 });
        let v1444: f64 = (if self.scalar_v288 { v1440 } else { v11 });
        let v1445: f64 = (v113 * v1343);
        let v1446: f64 = (v113 * v1344);
        let v1447: f64 = (v113 * v1345);
        let v1448: f64 = (v113 * v1346);
        let v1449: f64 = (v344 * v1445);
        let v1450: f64 = (v343 * v1408);
        let v1451: f64 = (v1449 + v1450);
        let v1452: f64 = (v344 * v1446);
        let v1453: f64 = (v343 * v1409);
        let v1454: f64 = (v1452 + v1453);
        let v1455: f64 = (v344 * v1447);
        let v1456: f64 = (v343 * v1410);
        let v1457: f64 = (v1455 + v1456);
        let v1458: f64 = (v344 * v1448);
        let v1459: f64 = (v343 * v1411);
        let v1460: f64 = (v1458 + v1459);
        let v1461: f64 = (v7 * v1441);
        let v1462: f64 = (v342 + v1461);
        let v1463: f64 = (v7 * v1442);
        let v1464: f64 = (-v342);
        let v1465: f64 = (v7 * v1443);
        let v1466: f64 = (v1464 + v1465);
        let v1467: f64 = (v7 * v1444);
        let v1468: f64 = (v937 + v1462);
        let v1469: f64 = (v938 + v1466);
        let v1470: f64 = (v348 * v1451);
        let v1471: f64 = (v345 * v1468);
        let v1472: f64 = (v1470 + v1471);
        let v1473: f64 = (v348 * v1454);
        let v1474: f64 = (v345 * v1463);
        let v1475: f64 = (v1473 + v1474);
        let v1476: f64 = (v348 * v1457);
        let v1477: f64 = (v345 * v1469);
        let v1478: f64 = (v1476 + v1477);
        let v1479: f64 = (v348 * v1460);
        let v1480: f64 = (v345 * v1467);
        let v1481: f64 = (v1479 + v1480);
        let v1482: f64 = (if self.scalar_v288 { v1472 } else { v953 });
        let v1483: f64 = (if self.scalar_v288 { v1475 } else { v954 });
        let v1484: f64 = (if self.scalar_v288 { v1478 } else { v955 });
        let v1485: f64 = (if self.scalar_v288 { v1481 } else { v956 });
        let v1486: f64 = (v113 * v1375);
        let v1487: f64 = (v113 * v1376);
        let v1488: f64 = (v113 * v1377);
        let v1489: f64 = (v113 * v1378);
        let v1490: f64 = (-v1425);
        let v1491: f64 = (-v1426);
        let v1492: f64 = (-v1427);
        let v1493: f64 = (-v1428);
        let v1494: f64 = (v352 * v1486);
        let v1495: f64 = (v351 * v1490);
        let v1496: f64 = (v1494 + v1495);
        let v1497: f64 = (v352 * v1487);
        let v1498: f64 = (v351 * v1491);
        let v1499: f64 = (v1497 + v1498);
        let v1500: f64 = (v352 * v1488);
        let v1501: f64 = (v351 * v1492);
        let v1502: f64 = (v1500 + v1501);
        let v1503: f64 = (v352 * v1489);
        let v1504: f64 = (v351 * v1493);
        let v1505: f64 = (v1503 + v1504);
        let v1506: f64 = (v7 * v1433);
        let v1507: f64 = (v339 + v1506);
        let v1508: f64 = (v7 * v1434);
        let v1509: f64 = (-v339);
        let v1510: f64 = (v7 * v1435);
        let v1511: f64 = (v1509 + v1510);
        let v1512: f64 = (v7 * v1436);
        let v1513: f64 = (-v1507);
        let v1514: f64 = (-v1508);
        let v1515: f64 = (-v1511);
        let v1516: f64 = (-v1512);
        let v1517: f64 = (v355 * v1496);
        let v1518: f64 = (v353 * v1513);
        let v1519: f64 = (v1517 + v1518);
        let v1520: f64 = (v355 * v1499);
        let v1521: f64 = (v353 * v1514);
        let v1522: f64 = (v1520 + v1521);
        let v1523: f64 = (v355 * v1502);
        let v1524: f64 = (v353 * v1515);
        let v1525: f64 = (v1523 + v1524);
        let v1526: f64 = (v355 * v1505);
        let v1527: f64 = (v353 * v1516);
        let v1528: f64 = (v1526 + v1527);
        let v1529: f64 = (if self.scalar_v288 { v1519 } else { v1025 });
        let v1530: f64 = (if self.scalar_v288 { v1522 } else { v1026 });
        let v1531: f64 = (if self.scalar_v288 { v1525 } else { v1027 });
        let v1532: f64 = (if self.scalar_v288 { v1528 } else { v1028 });
        let v1533: f64 = (v1482 - v1529);
        let v1534: f64 = (v1483 - v1530);
        let v1535: f64 = (v1484 - v1531);
        let v1536: f64 = (v1485 - v1532);
        let v1537: f64 = (v129 * v1533);
        let v1538: f64 = (v129 * v1534);
        let v1539: f64 = (v129 * v1535);
        let v1540: f64 = (v129 * v1536);
        let v1541: f64 = (if self.scalar_v288 { v1537 } else { v1199 });
        let v1542: f64 = (if self.scalar_v288 { v1538 } else { v1200 });
        let v1543: f64 = (if self.scalar_v288 { v1539 } else { v1201 });
        let v1544: f64 = (if self.scalar_v288 { v1540 } else { v1202 });
        let v1545: f64 = (v118 * v753);
        let v1546: f64 = (-v1545);
        let v1547: f64 = (v362 * v362);
        let v1548: f64 = (v1546 / v1547);
        let v1549: f64 = (v118 * v754);
        let v1550: f64 = (-v1549);
        let v1551: f64 = (v1550 / v1547);
        let v1552: f64 = (v118 * v755);
        let v1553: f64 = (-v1552);
        let v1554: f64 = (v1553 / v1547);
        let v1555: f64 = (v118 * v756);
        let v1556: f64 = (-v1555);
        let v1557: f64 = (v1556 / v1547);
        let v1558: f64 = (if self.scalar_v249 { v1548 } else { v11 });
        let v1559: f64 = (if self.scalar_v249 { v1551 } else { v11 });
        let v1560: f64 = (if self.scalar_v249 { v1554 } else { v11 });
        let v1561: f64 = (if self.scalar_v249 { v1557 } else { v11 });
        let v1562: f64 = (self.scalar_v367 * v753);
        let v1563: f64 = (self.scalar_v367 * v754);
        let v1564: f64 = (self.scalar_v367 * v755);
        let v1565: f64 = (self.scalar_v367 * v756);
        let v1566: f64 = (if self.scalar_v249 { v1562 } else { v11 });
        let v1567: f64 = (if self.scalar_v249 { v1563 } else { v11 });
        let v1568: f64 = (if self.scalar_v249 { v1564 } else { v11 });
        let v1569: f64 = (if self.scalar_v249 { v1565 } else { v11 });
        let v1570: f64 = (v118 * v1343);
        let v1571: f64 = (-v1570);
        let v1572: f64 = (v374 * v374);
        let v1573: f64 = (v1571 / v1572);
        let v1574: f64 = (v118 * v1344);
        let v1575: f64 = (-v1574);
        let v1576: f64 = (v1575 / v1572);
        let v1577: f64 = (v118 * v1345);
        let v1578: f64 = (-v1577);
        let v1579: f64 = (v1578 / v1572);
        let v1580: f64 = (v118 * v1346);
        let v1581: f64 = (-v1580);
        let v1582: f64 = (v1581 / v1572);
        let v1583: f64 = (if self.scalar_v250 { v1573 } else { v1558 });
        let v1584: f64 = (if self.scalar_v250 { v1576 } else { v1559 });
        let v1585: f64 = (if self.scalar_v250 { v1579 } else { v1560 });
        let v1586: f64 = (if self.scalar_v250 { v1582 } else { v1561 });
        let v1587: f64 = (self.scalar_v367 * v1343);
        let v1588: f64 = (self.scalar_v367 * v1344);
        let v1589: f64 = (self.scalar_v367 * v1345);
        let v1590: f64 = (self.scalar_v367 * v1346);
        let v1591: f64 = (if self.scalar_v250 { v1587 } else { v1566 });
        let v1592: f64 = (if self.scalar_v250 { v1588 } else { v1567 });
        let v1593: f64 = (if self.scalar_v250 { v1589 } else { v1568 });
        let v1594: f64 = (if self.scalar_v250 { v1590 } else { v1569 });
        let v1595: f64 = (v386 * v1591);
        let v1596: f64 = (v386 * v1592);
        let v1597: f64 = (v386 * v1593);
        let v1598: f64 = (v386 * v1594);
        let v1599: f64 = (if v383 { v1595 } else { v11 });
        let v1600: f64 = (if v383 { v1596 } else { v11 });
        let v1601: f64 = (if v383 { v1597 } else { v11 });
        let v1602: f64 = (if v383 { v1598 } else { v11 });
        let v1603: f64 = (if v391 { v1591 } else { v1599 });
        let v1604: f64 = (if v391 { v1592 } else { v1600 });
        let v1605: f64 = (if v391 { v1593 } else { v1601 });
        let v1606: f64 = (if v391 { v1594 } else { v1602 });
        let v1607: f64 = (if self.scalar_v395 { v11 } else { v1203 });
        let v1608: f64 = (if self.scalar_v395 { v11 } else { v1204 });
        let v1609: f64 = (if self.scalar_v395 { v11 } else { v1205 });
        let v1613: f64 = (if self.scalar_v406 { v11 } else { v1607 });
        let v1614: f64 = (if self.scalar_v406 { v11 } else { v1608 });
        let v1615: f64 = (if self.scalar_v406 { v11 } else { v1609 });
        let v1617: f64 = (v413 * v413);
        let v1618: f64 = (v42 - v1617);
        let v1619: f64 = (-v1618);
        let v1620: f64 = (if self.scalar_v412 { v1619 } else { self.scalar_v1611 });
        let v1621: f64 = (if self.scalar_v412 { v1618 } else { self.scalar_v1612 });
        let v1622: f64 = (v415 * v415);
        let v1623: f64 = (v42 - v1622);
        let v1624: f64 = (-v1623);
        let v1625: f64 = (if self.scalar_v412 { v1624 } else { self.scalar_v1611 });
        let v1626: f64 = (if self.scalar_v412 { v1623 } else { self.scalar_v1612 });
        let v1627: f64 = (if self.scalar_v418 { v396 } else { v1620 });
        let v1628: f64 = (if self.scalar_v418 { v42 } else { v1621 });
        let v1629: f64 = (if self.scalar_v418 { v396 } else { v1625 });
        let v1630: f64 = (if self.scalar_v418 { v42 } else { v1626 });
        let v1631: f64 = (v136 * v1627);
        let v1632: f64 = (v136 * v1628);
        let v1633: f64 = { let limexp_arg = v422; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v1634: f64 = (v1631 * v1633);
        let v1635: f64 = (v1632 * v1633);
        let v1636: f64 = (-v1613);
        let v1637: f64 = (-v1614);
        let v1638: f64 = (v1634 - v1615);
        let v1639: f64 = (v1635 - self.scalar_v1616);
        let v1640: f64 = (self.scalar_v421 * v1636);
        let v1641: f64 = (self.scalar_v421 * v1637);
        let v1642: f64 = (self.scalar_v421 * v1638);
        let v1643: f64 = (self.scalar_v421 * v1639);
        let v1644: f64 = (v136 * v1629);
        let v1645: f64 = (v136 * v1630);
        let v1646: f64 = { let limexp_arg = v426; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v1647: f64 = (v1644 * v1646);
        let v1648: f64 = (v1645 * v1646);
        let v1649: f64 = (v1647 - v1613);
        let v1650: f64 = (-v1615);
        let v1652: f64 = (self.scalar_v421 * v1649);
        let v1653: f64 = (self.scalar_v421 * v1650);
        let v1654: f64 = (self.scalar_v421 * v1648);
        let v1659: f64 = (v435 * v435);
        let v1660: f64 = (v42 - v1659);
        let v1661: f64 = (self.scalar_v432 * v1660);
        let v1662: f64 = (self.scalar_v1658 * v1660);
        let v1663: f64 = (self.scalar_v91 * v1660);
        let v1665: f64 = (v441 * v441);
        let v1666: f64 = (v42 - v1665);
        let v1667: f64 = (self.scalar_v438 * v1666);
        let v1668: f64 = (self.scalar_v1664 * v1666);
        let v1670: f64 = (v447 * v447);
        let v1671: f64 = (v42 - v1670);
        let v1672: f64 = (self.scalar_v1669 * v1671);
        let v1673: f64 = (self.scalar_v444 * v1671);
        let v1676: f64 = (v453 * v453);
        let v1677: f64 = (v42 - v1676);
        let v1678: f64 = (self.scalar_v1675 * v1677);
        let v1679: f64 = (self.scalar_v432 * v1677);
        let v1680: f64 = (self.scalar_v97 * v1677);
        let v1681: f64 = (v116 * v1661);
        let v1682: f64 = (v116 * v1662);
        let v1683: f64 = (v116 * v1663);
        let v1684: f64 = (v465 * v1667);
        let v1685: f64 = (v442 * v1681);
        let v1686: f64 = (v1684 + v1685);
        let v1687: f64 = (v465 * v1668);
        let v1688: f64 = (v442 * v1682);
        let v1689: f64 = (v1687 + v1688);
        let v1690: f64 = (v442 * v1683);
        let v1691: f64 = (if self.scalar_v464 { v1686 } else { v11 });
        let v1692: f64 = (if self.scalar_v464 { v1689 } else { v11 });
        let v1693: f64 = (if self.scalar_v464 { v1690 } else { v11 });
        let v1694: f64 = (v454 * v1672);
        let v1695: f64 = (v449 * v1678);
        let v1696: f64 = (v1694 + v1695);
        let v1697: f64 = (v454 * v1673);
        let v1698: f64 = (v449 * v1679);
        let v1699: f64 = (v1697 + v1698);
        let v1700: f64 = (v449 * v1680);
        let v1701: f64 = (v117 * v1696);
        let v1702: f64 = (v117 * v1699);
        let v1703: f64 = (v117 * v1700);
        let v1704: f64 = (if self.scalar_v464 { v1701 } else { v11 });
        let v1705: f64 = (if self.scalar_v464 { v1702 } else { v11 });
        let v1706: f64 = (if self.scalar_v464 { v1703 } else { v11 });
        let v1707: f64 = ((v480) as f64).sinh();
        let v1708: f64 = (self.scalar_v432 * v1707);
        let v1709: f64 = (self.scalar_v1657 * v1707);
        let v1710: f64 = (if self.scalar_v477 { v1708 } else { v11 });
        let v1711: f64 = (if self.scalar_v477 { v1709 } else { v11 });
        let v1712: f64 = (v1710 / v482);
        let v1713: f64 = (v1711 / v482);
        let v1714: f64 = (if self.scalar_v477 { v1712 } else { v11 });
        let v1715: f64 = (if self.scalar_v477 { v1713 } else { v11 });
        let v1717: f64 = (self.scalar_v432 * v1716);
        let v1718: f64 = (self.scalar_v1658 * v1716);
        let v1720: f64 = (if self.scalar_v477 { v1717 } else { v11 });
        let v1721: f64 = (if self.scalar_v477 { v1718 } else { v11 });
        let v1723: f64 = (v1720 / v486);
        let v1724: f64 = (v1721 / v486);
        let v1726: f64 = (if self.scalar_v477 { v1723 } else { v11 });
        let v1727: f64 = (if self.scalar_v477 { v1724 } else { v11 });
        let v1729: f64 = (self.scalar_v432 + v1714);
        let v1730: f64 = (self.scalar_v1657 + v1715);
        let v1731: f64 = (if self.scalar_v477 { v1729 } else { v11 });
        let v1732: f64 = (if self.scalar_v477 { v1730 } else { v11 });
        let v1733: f64 = (self.scalar_v432 + v1726);
        let v1734: f64 = (self.scalar_v1658 + v1727);
        let v1736: f64 = (v1733 - v1731);
        let v1737: f64 = (v1734 - v1732);
        let v1738: f64 = (v492 * v1667);
        let v1739: f64 = (v479 * v1736);
        let v1740: f64 = (v1738 + v1739);
        let v1741: f64 = (v492 * v1668);
        let v1742: f64 = (v479 * v1737);
        let v1743: f64 = (v1741 + v1742);
        let v1745: f64 = (v1740 / self.scalar_v91);
        let v1746: f64 = (v1743 / self.scalar_v91);
        let v1749: f64 = (v1746 + self.scalar_v1748);
        let v1751: f64 = (v116 * v1745);
        let v1752: f64 = (v116 * v1749);
        let v1755: f64 = (v1752 + self.scalar_v1754);
        let v1757: f64 = (if self.scalar_v477 { v1751 } else { v11 });
        let v1758: f64 = (if self.scalar_v477 { v1755 } else { v11 });
        let v1760: f64 = ((v501) as f64).sinh();
        let v1761: f64 = (self.scalar_v1657 * v1760);
        let v1762: f64 = (self.scalar_v432 * v1760);
        let v1763: f64 = (if self.scalar_v477 { v1761 } else { v1710 });
        let v1764: f64 = (if self.scalar_v477 { v1762 } else { v1711 });
        let v1765: f64 = (v1763 / v503);
        let v1766: f64 = (v1764 / v503);
        let v1767: f64 = (if self.scalar_v477 { v1765 } else { v11 });
        let v1768: f64 = (if self.scalar_v477 { v1766 } else { v11 });
        let v1770: f64 = (self.scalar_v1675 * v1769);
        let v1771: f64 = (self.scalar_v432 * v1769);
        let v1773: f64 = (if self.scalar_v477 { v1770 } else { v1720 });
        let v1774: f64 = (if self.scalar_v477 { v1771 } else { v1721 });
        let v1776: f64 = (if self.scalar_v477 { v11 } else { v1722 });
        let v1777: f64 = (v1773 / v507);
        let v1778: f64 = (v1774 / v507);
        let v1780: f64 = (v1776 / v507);
        let v1781: f64 = (if self.scalar_v477 { v1777 } else { v11 });
        let v1782: f64 = (if self.scalar_v477 { v1778 } else { v11 });
        let v1784: f64 = (if self.scalar_v477 { v1780 } else { v11 });
        let v1785: f64 = (self.scalar_v1657 + v1767);
        let v1786: f64 = (self.scalar_v432 + v1768);
        let v1787: f64 = (if self.scalar_v477 { v1785 } else { v11 });
        let v1788: f64 = (if self.scalar_v477 { v1786 } else { v11 });
        let v1789: f64 = (self.scalar_v1675 + v1781);
        let v1790: f64 = (self.scalar_v432 + v1782);
        let v1792: f64 = (v1789 - v1787);
        let v1793: f64 = (v1790 - v1788);
        let v1794: f64 = (v513 * v1672);
        let v1795: f64 = (v449 * v1792);
        let v1796: f64 = (v1794 + v1795);
        let v1797: f64 = (v513 * v1673);
        let v1798: f64 = (v449 * v1793);
        let v1799: f64 = (v1797 + v1798);
        let v1801: f64 = (v449 * v1784);
        let v1802: f64 = (v1796 / self.scalar_v97);
        let v1803: f64 = (v1799 / self.scalar_v97);
        let v1805: f64 = (v1801 / self.scalar_v97);
        let v1806: f64 = (self.scalar_v1748 + v1802);
        let v1808: f64 = (v117 * v1806);
        let v1809: f64 = (v117 * v1803);
        let v1811: f64 = (v117 * v1805);
        let v1813: f64 = (v1808 + self.scalar_v1812);
        let v1815: f64 = (if self.scalar_v477 { v1813 } else { v11 });
        let v1816: f64 = (if self.scalar_v477 { v1809 } else { v11 });
        let v1818: f64 = (if self.scalar_v477 { v1811 } else { v11 });
        let v1819: f64 = (if self.scalar_v477 { v11 } else { v1691 });
        let v1820: f64 = (if self.scalar_v477 { v11 } else { v1692 });
        let v1821: f64 = (if self.scalar_v477 { v11 } else { v1693 });
        let v1822: f64 = (if self.scalar_v477 { v11 } else { v1704 });
        let v1823: f64 = (if self.scalar_v477 { v11 } else { v1705 });
        let v1824: f64 = (if self.scalar_v477 { v11 } else { v1706 });
        let v1825: f64 = (-v1541);
        let v1826: f64 = (-v1542);
        let v1827: f64 = (-v1543);
        let v1828: f64 = (-v1544);
        let v1829: f64 = ddt_scale;
        let v1830: f64 = (v1815 * v1829);
        let v1831: f64 = (v1816 * v1829);
        let v1832: f64 = (v1817 * v1829);
        let v1833: f64 = (v1818 * v1829);
        let v1834: f64 = (if self.scalar_v458 { v1830 } else { v11 });
        let v1835: f64 = (if self.scalar_v458 { v1831 } else { v11 });
        let v1836: f64 = (if self.scalar_v458 { v1832 } else { v11 });
        let v1837: f64 = (if self.scalar_v458 { v1833 } else { v11 });
        let v1838: f64 = (v1757 * v1829);
        let v1839: f64 = (v1758 * v1829);
        let v1840: f64 = (v1759 * v1829);
        let v1841: f64 = (if self.scalar_v458 { v1838 } else { v11 });
        let v1842: f64 = (if self.scalar_v458 { v1839 } else { v11 });
        let v1843: f64 = (if self.scalar_v458 { v1840 } else { v11 });
        let v1844: f64 = (-v525);
        let v1845: f64 = (v9 * v1822);
        let v1846: f64 = (v1844 + v1845);
        let v1847: f64 = (v9 * v1823);
        let v1848: f64 = (v9 * v1824);
        let v1849: f64 = (v525 + v1848);
        let v1850: f64 = (v1829 * v1846);
        let v1851: f64 = (v1829 * v1847);
        let v1852: f64 = (v1829 * v1849);
        let v1853: f64 = (if self.scalar_v583 { v1850 } else { v11 });
        let v1854: f64 = (if self.scalar_v583 { v1851 } else { v11 });
        let v1855: f64 = (if self.scalar_v583 { v1852 } else { v11 });
        let v1856: f64 = (v2 * v1819);
        let v1857: f64 = (-v523);
        let v1858: f64 = (v2 * v1820);
        let v1859: f64 = (v1857 + v1858);
        let v1860: f64 = (v2 * v1821);
        let v1861: f64 = (v523 + v1860);
        let v1862: f64 = (v1829 * v1856);
        let v1863: f64 = (v1829 * v1859);
        let v1864: f64 = (v1829 * v1861);
        let v1865: f64 = (if self.scalar_v583 { v1862 } else { v11 });
        let v1866: f64 = (if self.scalar_v583 { v1863 } else { v11 });
        let v1867: f64 = (if self.scalar_v583 { v1864 } else { v11 });
        let v1870: f64 = (-v119);
        let v1871: f64 = (v599 * v1583);
        let v1872: f64 = (-v1871);
        let v1873: f64 = (v377 * v377);
        let v1874: f64 = (v1872 / v1873);
        let v1875: f64 = (v599 * v1584);
        let v1876: f64 = (-v1875);
        let v1877: f64 = (v1876 / v1873);
        let v1878: f64 = (-v377);
        let v1879: f64 = (v599 * v1585);
        let v1880: f64 = (v1878 - v1879);
        let v1881: f64 = (v1880 / v1873);
        let v1882: f64 = (v599 * v1586);
        let v1883: f64 = (-v1882);
        let v1884: f64 = (v1883 / v1873);
        let v1885: f64 = (v42 / v377);
        let v1886: f64 = (if self.scalar_v526 { v1874 } else { v11 });
        let v1887: f64 = (if self.scalar_v526 { v1877 } else { v11 });
        let v1888: f64 = (if self.scalar_v526 { v1881 } else { v11 });
        let v1889: f64 = (if self.scalar_v526 { v1884 } else { v11 });
        let v1890: f64 = (if self.scalar_v526 { v1885 } else { v11 });
        let v1905: f64 = (self.scalar_v535 * v1829);
        let v1906: f64 = (if self.scalar_v534 { v1905 } else { v11 });
        let v1907: f64 = (if self.scalar_v623 { v1905 } else { v11 });
        let v1908: f64 = (v628 * v1603);
        let v1909: f64 = (v628 * v1604);
        let v1910: f64 = (v628 * v1605);
        let v1911: f64 = (v628 * v1606);
        let v1912: f64 = (if self.scalar_v537 { v1908 } else { v11 });
        let v1913: f64 = (if self.scalar_v537 { v1909 } else { v11 });
        let v1914: f64 = (if self.scalar_v537 { v1910 } else { v11 });
        let v1915: f64 = (if self.scalar_v537 { v1911 } else { v11 });
        let v1916: f64 = (if self.scalar_v537 { v393 } else { v11 });
        let v1917: f64 = (v634 * v1603);
        let v1918: f64 = (v634 * v1604);
        let v1919: f64 = (v634 * v1605);
        let v1920: f64 = (v634 * v1606);
        let v1921: f64 = (if self.scalar_v540 { v1917 } else { v11 });
        let v1922: f64 = (if self.scalar_v540 { v1918 } else { v11 });
        let v1923: f64 = (if self.scalar_v540 { v1919 } else { v11 });
        let v1924: f64 = (if self.scalar_v540 { v1920 } else { v11 });
        let v1925: f64 = (if self.scalar_v540 { v392 } else { v11 });
        let v1926: f64 = (self.scalar_v541 * v1829);
        let v1927: f64 = (if self.scalar_v540 { v1926 } else { v11 });
        let v1928: f64 = (if self.scalar_v642 { v1926 } else { v11 });
        let v1930: f64 = (if self.scalar_v549 { v568 } else { v11 });
        let v1931: f64 = (if self.scalar_v549 { v564 } else { v11 });
        let v1932: f64 = (v655 * v1829);
        let v1933: f64 = (if self.scalar_v549 { v1932 } else { v11 });
        let v1934: f64 = (self.scalar_v659 * v1829);
        let v1935: f64 = (if self.scalar_v571 { v1934 } else { v11 });

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
        let v8: f64 = ctx.node_voltage(nodes[7]);
        let v9: f64 = (v8 - v4);
        let v11: f64 = 0.0;
        let v30: f64 = ctx.node_voltage(nodes[11]);
        let v31: f64 = ((v30) as f64).abs();
        let v32: f64 = (self.scalar_v21 + v31);
        let v33: f64 = (if (self.scalar_v29 != 0.0) { v32 } else { self.scalar_v21 });
        let v36: f64 = (v33 - self.scalar_v28);
        let v37: f64 = ((v36) as f64).abs();
        let v38: bool = (v37 > v11);
        let v41: bool = (v38 || self.scalar_v40);
        let v42: f64 = 1.0;
        let v63: f64 = (v37 * self.scalar_v62);
        let v64: f64 = (v42 + v63);
        let v65: f64 = (self.scalar_v61 * v64);
        let v66: f64 = (if v41 { v65 } else { v11 });
        let v69: f64 = (v37 * self.scalar_v68);
        let v70: f64 = (v42 + v69);
        let v71: f64 = (self.scalar_v67 * v70);
        let v72: f64 = (if v41 { v71 } else { v11 });
        let v81: f64 = (v37 * self.scalar_v80);
        let v82: f64 = (v42 + v81);
        let v83: f64 = (self.scalar_v79 * v82);
        let v84: f64 = (if v41 { v83 } else { v11 });
        let v93: f64 = (v37 * self.scalar_v92);
        let v94: f64 = (self.scalar_v90 + v93);
        let v95: f64 = (if v41 { v94 } else { v11 });
        let v99: f64 = (v37 * self.scalar_v98);
        let v100: f64 = (self.scalar_v96 + v99);
        let v101: f64 = (if v41 { v100 } else { v11 });
        let v112: bool = (!v41);
        let v116: f64 = (if v112 { self.scalar_v61 } else { v66 });
        let v117: f64 = (if v112 { self.scalar_v67 } else { v72 });
        let v119: f64 = (if v112 { self.scalar_v79 } else { v84 });
        let v121: f64 = (if v112 { self.scalar_v90 } else { v95 });
        let v122: f64 = (if v112 { self.scalar_v96 } else { v101 });
        let v430: f64 = (v2 * self.scalar_v91);
        let v431: f64 = (v121 + v430);
        let v433: f64 = (v7 * self.scalar_v432);
        let v434: f64 = (v431 + v433);
        let v435: f64 = ((v434) as f64).tanh();
        let v436: f64 = (v42 + v435);
        let v439: f64 = (v7 * self.scalar_v438);
        let v440: f64 = (self.scalar_v437 + v439);
        let v441: f64 = ((v440) as f64).tanh();
        let v442: f64 = (v42 + v441);
        let v445: f64 = (v7 * self.scalar_v444);
        let v446: f64 = (self.scalar_v443 - v445);
        let v447: f64 = ((v446) as f64).tanh();
        let v448: f64 = (v42 + v447);
        let v449: f64 = (v448 - self.scalar_v432);
        let v450: f64 = (v9 * self.scalar_v97);
        let v451: f64 = (v122 + v450);
        let v452: f64 = (v451 - v433);
        let v453: f64 = ((v452) as f64).tanh();
        let v454: f64 = (v42 + v453);
        let v465: f64 = (v116 * v436);
        let v466: f64 = (v442 * v465);
        let v467: f64 = (self.scalar_v459 + v466);
        let v468: f64 = (if self.scalar_v464 { v467 } else { self.scalar_v460 });
        let v469: f64 = (v449 * v454);
        let v471: f64 = (v469 + self.scalar_v470);
        let v472: f64 = (v117 * v471);
        let v473: f64 = (self.scalar_v461 + v472);
        let v474: f64 = (if self.scalar_v464 { v473 } else { self.scalar_v462 });
        let v478: f64 = (v442 - self.scalar_v432);
        let v479: f64 = (if self.scalar_v477 { v478 } else { v442 });
        let v480: f64 = (v121 + v433);
        let v481: f64 = ((v480) as f64).cosh();
        let v482: f64 = (if self.scalar_v477 { v481 } else { v11 });
        let v483: f64 = ((v482) as f64).ln();
        let v484: f64 = (if self.scalar_v477 { v483 } else { v11 });
        let v485: f64 = ((v434) as f64).cosh();
        let v486: f64 = (if self.scalar_v477 { v485 } else { v11 });
        let v487: f64 = ((v486) as f64).ln();
        let v488: f64 = (if self.scalar_v477 { v487 } else { v11 });
        let v489: f64 = (v480 + v484);
        let v490: f64 = (if self.scalar_v477 { v489 } else { v11 });
        let v491: f64 = (v434 + v488);
        let v492: f64 = (v491 - v490);
        let v493: f64 = (v479 * v492);
        let v494: f64 = (v493 / self.scalar_v91);
        let v495: f64 = (v2 * self.scalar_v470);
        let v496: f64 = (v494 + v495);
        let v497: f64 = (v116 * v496);
        let v498: f64 = (v2 * self.scalar_v459);
        let v499: f64 = (v497 + v498);
        let v500: f64 = (if self.scalar_v477 { v499 } else { v11 });
        let v501: f64 = (v122 - v433);
        let v502: f64 = ((v501) as f64).cosh();
        let v503: f64 = (if self.scalar_v477 { v502 } else { v482 });
        let v504: f64 = ((v503) as f64).ln();
        let v505: f64 = (if self.scalar_v477 { v504 } else { v11 });
        let v506: f64 = ((v452) as f64).cosh();
        let v507: f64 = (if self.scalar_v477 { v506 } else { v486 });
        let v508: f64 = ((v507) as f64).ln();
        let v509: f64 = (if self.scalar_v477 { v508 } else { v11 });
        let v510: f64 = (v501 + v505);
        let v511: f64 = (if self.scalar_v477 { v510 } else { v11 });
        let v512: f64 = (v452 + v509);
        let v513: f64 = (v512 - v511);
        let v514: f64 = (v449 * v513);
        let v515: f64 = (v514 / self.scalar_v97);
        let v516: f64 = (v9 * self.scalar_v470);
        let v517: f64 = (v515 + v516);
        let v518: f64 = (v117 * v517);
        let v519: f64 = (v9 * self.scalar_v461);
        let v520: f64 = (v518 + v519);
        let v521: f64 = (if self.scalar_v477 { v520 } else { v11 });
        let v1716: f64 = ((v434) as f64).sinh();
        let v1719: f64 = (self.scalar_v91 * v1716);
        let v1722: f64 = (if self.scalar_v477 { v1719 } else { v11 });
        let v1725: f64 = (v1722 / v486);
        let v1728: f64 = (if self.scalar_v477 { v1725 } else { v11 });
        let v1735: f64 = (self.scalar_v91 + v1728);
        let v1744: f64 = (v479 * v1735);
        let v1747: f64 = (v1744 / self.scalar_v91);
        let v1750: f64 = (self.scalar_v470 + v1747);
        let v1753: f64 = (v116 * v1750);
        let v1756: f64 = (self.scalar_v459 + v1753);
        let v1759: f64 = (if self.scalar_v477 { v1756 } else { v11 });
        let v522: f64 = v1759;
        let v523: f64 = (if self.scalar_v477 { v522 } else { v468 });
        let v1769: f64 = ((v452) as f64).sinh();
        let v1772: f64 = (self.scalar_v97 * v1769);
        let v1775: f64 = (if self.scalar_v477 { v1772 } else { v11 });
        let v1779: f64 = (v1775 / v507);
        let v1783: f64 = (if self.scalar_v477 { v1779 } else { v11 });
        let v1791: f64 = (self.scalar_v97 + v1783);
        let v1800: f64 = (v449 * v1791);
        let v1804: f64 = (v1800 / self.scalar_v97);
        let v1807: f64 = (self.scalar_v470 + v1804);
        let v1810: f64 = (v117 * v1807);
        let v1814: f64 = (self.scalar_v461 + v1810);
        let v1817: f64 = (if self.scalar_v477 { v1814 } else { v11 });
        let v524: f64 = v1817;
        let v525: f64 = (if self.scalar_v477 { v524 } else { v474 });
        let v550: f64 = 5.5226012e-23;
        let v551: f64 = (v33 * v550);
        let v555: f64 = (v551 * self.scalar_v554);
        let v556: f64 = (v116 * v555);
        let v559: f64 = (v556 * self.scalar_v558);
        let v560: f64 = (if self.scalar_v549 { v559 } else { v11 });
        let v566: f64 = 3.141592653589793;
        let v569: f64 = (v560 * v566);
        let v570: f64 = (if self.scalar_v549 { v569 } else { v11 });
        let v574: f64 = ctx.node_voltage(nodes[12]);
        let v575: f64 = (self.scalar_v573 * v574);
        let v577: f64 = ctx.branch_current(branches[0]);
        let v578: f64 = (self.scalar_v576 * v577);
        let v579: f64 = 0.0;
        let v580: f64 = (if self.scalar_v458 { v579 } else { v11 });
        let v581: f64 = 0.0;
        let v582: f64 = (if self.scalar_v458 { v581 } else { v11 });
        let v584: f64 = (v9 * v525);
        let v585: f64 = 0.0;
        let v586: f64 = (if self.scalar_v583 { v585 } else { v11 });
        let v587: f64 = (v2 * v523);
        let v588: f64 = 0.0;
        let v589: f64 = (if self.scalar_v583 { v588 } else { v11 });
        let v591: f64 = ctx.node_voltage(nodes[1]);
        let v592: f64 = (v591 - v4);
        let v593: f64 = (self.scalar_v590 * v592);
        let v595: f64 = (v7 * self.scalar_v594);
        let v596: f64 = ctx.node_voltage(nodes[10]);
        let v597: f64 = (v4 - v596);
        let v598: f64 = (v119 * v597);
        let v603: f64 = ctx.node_voltage(nodes[9]);
        let v604: f64 = (v603 - v0);
        let v605: f64 = (self.scalar_v602 * v604);
        let v618: f64 = ctx.branch_current(branches[6]);
        let v619: f64 = (self.scalar_v535 * v618);
        let v620: f64 = 0.0;
        let v621: f64 = (if self.scalar_v534 { v620 } else { v11 });
        let v624: f64 = ctx.branch_current(branches[8]);
        let v625: f64 = (self.scalar_v535 * v624);
        let v626: f64 = 0.0;
        let v627: f64 = (if self.scalar_v623 { v626 } else { v11 });
        let v632: f64 = ctx.branch_current(branches[13]);
        let v633: f64 = (self.scalar_v631 * v632);
        let v637: f64 = ctx.branch_current(branches[15]);
        let v638: f64 = (self.scalar_v541 * v637);
        let v639: f64 = 0.0;
        let v640: f64 = (if self.scalar_v540 { v639 } else { v11 });
        let v643: f64 = ctx.branch_current(branches[17]);
        let v644: f64 = (self.scalar_v541 * v643);
        let v645: f64 = 0.0;
        let v646: f64 = (if self.scalar_v642 { v645 } else { v11 });
        let v647: f64 = ctx.node_voltage(nodes[14]);
        let v655: f64 = (-v570);
        let v656: f64 = (v647 * v655);
        let v657: f64 = 0.0;
        let v658: f64 = (if self.scalar_v549 { v657 } else { v11 });
        let v660: f64 = (v30 * self.scalar_v659);
        let v661: f64 = 0.0;
        let v662: f64 = (if self.scalar_v571 { v661 } else { v11 });
        let v1659: f64 = (v435 * v435);
        let v1660: f64 = (v42 - v1659);
        let v1661: f64 = (self.scalar_v432 * v1660);
        let v1662: f64 = (self.scalar_v1658 * v1660);
        let v1663: f64 = (self.scalar_v91 * v1660);
        let v1665: f64 = (v441 * v441);
        let v1666: f64 = (v42 - v1665);
        let v1667: f64 = (self.scalar_v438 * v1666);
        let v1668: f64 = (self.scalar_v1664 * v1666);
        let v1670: f64 = (v447 * v447);
        let v1671: f64 = (v42 - v1670);
        let v1672: f64 = (self.scalar_v1669 * v1671);
        let v1673: f64 = (self.scalar_v444 * v1671);
        let v1676: f64 = (v453 * v453);
        let v1677: f64 = (v42 - v1676);
        let v1678: f64 = (self.scalar_v1675 * v1677);
        let v1679: f64 = (self.scalar_v432 * v1677);
        let v1680: f64 = (self.scalar_v97 * v1677);
        let v1681: f64 = (v116 * v1661);
        let v1682: f64 = (v116 * v1662);
        let v1683: f64 = (v116 * v1663);
        let v1684: f64 = (v465 * v1667);
        let v1685: f64 = (v442 * v1681);
        let v1686: f64 = (v1684 + v1685);
        let v1687: f64 = (v465 * v1668);
        let v1688: f64 = (v442 * v1682);
        let v1689: f64 = (v1687 + v1688);
        let v1690: f64 = (v442 * v1683);
        let v1691: f64 = (if self.scalar_v464 { v1686 } else { v11 });
        let v1692: f64 = (if self.scalar_v464 { v1689 } else { v11 });
        let v1693: f64 = (if self.scalar_v464 { v1690 } else { v11 });
        let v1694: f64 = (v454 * v1672);
        let v1695: f64 = (v449 * v1678);
        let v1696: f64 = (v1694 + v1695);
        let v1697: f64 = (v454 * v1673);
        let v1698: f64 = (v449 * v1679);
        let v1699: f64 = (v1697 + v1698);
        let v1700: f64 = (v449 * v1680);
        let v1701: f64 = (v117 * v1696);
        let v1702: f64 = (v117 * v1699);
        let v1703: f64 = (v117 * v1700);
        let v1704: f64 = (if self.scalar_v464 { v1701 } else { v11 });
        let v1705: f64 = (if self.scalar_v464 { v1702 } else { v11 });
        let v1706: f64 = (if self.scalar_v464 { v1703 } else { v11 });
        let v1707: f64 = ((v480) as f64).sinh();
        let v1708: f64 = (self.scalar_v432 * v1707);
        let v1709: f64 = (self.scalar_v1657 * v1707);
        let v1710: f64 = (if self.scalar_v477 { v1708 } else { v11 });
        let v1711: f64 = (if self.scalar_v477 { v1709 } else { v11 });
        let v1712: f64 = (v1710 / v482);
        let v1713: f64 = (v1711 / v482);
        let v1714: f64 = (if self.scalar_v477 { v1712 } else { v11 });
        let v1715: f64 = (if self.scalar_v477 { v1713 } else { v11 });
        let v1717: f64 = (self.scalar_v432 * v1716);
        let v1718: f64 = (self.scalar_v1658 * v1716);
        let v1720: f64 = (if self.scalar_v477 { v1717 } else { v11 });
        let v1721: f64 = (if self.scalar_v477 { v1718 } else { v11 });
        let v1723: f64 = (v1720 / v486);
        let v1724: f64 = (v1721 / v486);
        let v1726: f64 = (if self.scalar_v477 { v1723 } else { v11 });
        let v1727: f64 = (if self.scalar_v477 { v1724 } else { v11 });
        let v1729: f64 = (self.scalar_v432 + v1714);
        let v1730: f64 = (self.scalar_v1657 + v1715);
        let v1731: f64 = (if self.scalar_v477 { v1729 } else { v11 });
        let v1732: f64 = (if self.scalar_v477 { v1730 } else { v11 });
        let v1733: f64 = (self.scalar_v432 + v1726);
        let v1734: f64 = (self.scalar_v1658 + v1727);
        let v1736: f64 = (v1733 - v1731);
        let v1737: f64 = (v1734 - v1732);
        let v1738: f64 = (v492 * v1667);
        let v1739: f64 = (v479 * v1736);
        let v1740: f64 = (v1738 + v1739);
        let v1741: f64 = (v492 * v1668);
        let v1742: f64 = (v479 * v1737);
        let v1743: f64 = (v1741 + v1742);
        let v1745: f64 = (v1740 / self.scalar_v91);
        let v1746: f64 = (v1743 / self.scalar_v91);
        let v1749: f64 = (v1746 + self.scalar_v1748);
        let v1751: f64 = (v116 * v1745);
        let v1752: f64 = (v116 * v1749);
        let v1755: f64 = (v1752 + self.scalar_v1754);
        let v1757: f64 = (if self.scalar_v477 { v1751 } else { v11 });
        let v1758: f64 = (if self.scalar_v477 { v1755 } else { v11 });
        let v1760: f64 = ((v501) as f64).sinh();
        let v1761: f64 = (self.scalar_v1657 * v1760);
        let v1762: f64 = (self.scalar_v432 * v1760);
        let v1763: f64 = (if self.scalar_v477 { v1761 } else { v1710 });
        let v1764: f64 = (if self.scalar_v477 { v1762 } else { v1711 });
        let v1765: f64 = (v1763 / v503);
        let v1766: f64 = (v1764 / v503);
        let v1767: f64 = (if self.scalar_v477 { v1765 } else { v11 });
        let v1768: f64 = (if self.scalar_v477 { v1766 } else { v11 });
        let v1770: f64 = (self.scalar_v1675 * v1769);
        let v1771: f64 = (self.scalar_v432 * v1769);
        let v1773: f64 = (if self.scalar_v477 { v1770 } else { v1720 });
        let v1774: f64 = (if self.scalar_v477 { v1771 } else { v1721 });
        let v1776: f64 = (if self.scalar_v477 { v11 } else { v1722 });
        let v1777: f64 = (v1773 / v507);
        let v1778: f64 = (v1774 / v507);
        let v1780: f64 = (v1776 / v507);
        let v1781: f64 = (if self.scalar_v477 { v1777 } else { v11 });
        let v1782: f64 = (if self.scalar_v477 { v1778 } else { v11 });
        let v1784: f64 = (if self.scalar_v477 { v1780 } else { v11 });
        let v1785: f64 = (self.scalar_v1657 + v1767);
        let v1786: f64 = (self.scalar_v432 + v1768);
        let v1787: f64 = (if self.scalar_v477 { v1785 } else { v11 });
        let v1788: f64 = (if self.scalar_v477 { v1786 } else { v11 });
        let v1789: f64 = (self.scalar_v1675 + v1781);
        let v1790: f64 = (self.scalar_v432 + v1782);
        let v1792: f64 = (v1789 - v1787);
        let v1793: f64 = (v1790 - v1788);
        let v1794: f64 = (v513 * v1672);
        let v1795: f64 = (v449 * v1792);
        let v1796: f64 = (v1794 + v1795);
        let v1797: f64 = (v513 * v1673);
        let v1798: f64 = (v449 * v1793);
        let v1799: f64 = (v1797 + v1798);
        let v1801: f64 = (v449 * v1784);
        let v1802: f64 = (v1796 / self.scalar_v97);
        let v1803: f64 = (v1799 / self.scalar_v97);
        let v1805: f64 = (v1801 / self.scalar_v97);
        let v1806: f64 = (self.scalar_v1748 + v1802);
        let v1808: f64 = (v117 * v1806);
        let v1809: f64 = (v117 * v1803);
        let v1811: f64 = (v117 * v1805);
        let v1813: f64 = (v1808 + self.scalar_v1812);
        let v1815: f64 = (if self.scalar_v477 { v1813 } else { v11 });
        let v1816: f64 = (if self.scalar_v477 { v1809 } else { v11 });
        let v1818: f64 = (if self.scalar_v477 { v1811 } else { v11 });
        let v1819: f64 = (if self.scalar_v477 { v11 } else { v1691 });
        let v1820: f64 = (if self.scalar_v477 { v11 } else { v1692 });
        let v1821: f64 = (if self.scalar_v477 { v11 } else { v1693 });
        let v1822: f64 = (if self.scalar_v477 { v11 } else { v1704 });
        let v1823: f64 = (if self.scalar_v477 { v11 } else { v1705 });
        let v1824: f64 = (if self.scalar_v477 { v11 } else { v1706 });
        let v1829: f64 = 1.0;
        let v1830: f64 = (v1815 * v1829);
        let v1831: f64 = (v1816 * v1829);
        let v1832: f64 = (v1817 * v1829);
        let v1833: f64 = (v1818 * v1829);
        let v1834: f64 = (if self.scalar_v458 { v1830 } else { v11 });
        let v1835: f64 = (if self.scalar_v458 { v1831 } else { v11 });
        let v1836: f64 = (if self.scalar_v458 { v1832 } else { v11 });
        let v1837: f64 = (if self.scalar_v458 { v1833 } else { v11 });
        let v1838: f64 = (v1757 * v1829);
        let v1839: f64 = (v1758 * v1829);
        let v1840: f64 = (v1759 * v1829);
        let v1841: f64 = (if self.scalar_v458 { v1838 } else { v11 });
        let v1842: f64 = (if self.scalar_v458 { v1839 } else { v11 });
        let v1843: f64 = (if self.scalar_v458 { v1840 } else { v11 });
        let v1844: f64 = (-v525);
        let v1845: f64 = (v9 * v1822);
        let v1846: f64 = (v1844 + v1845);
        let v1847: f64 = (v9 * v1823);
        let v1848: f64 = (v9 * v1824);
        let v1849: f64 = (v525 + v1848);
        let v1850: f64 = (v1829 * v1846);
        let v1851: f64 = (v1829 * v1847);
        let v1852: f64 = (v1829 * v1849);
        let v1853: f64 = (if self.scalar_v583 { v1850 } else { v11 });
        let v1854: f64 = (if self.scalar_v583 { v1851 } else { v11 });
        let v1855: f64 = (if self.scalar_v583 { v1852 } else { v11 });
        let v1856: f64 = (v2 * v1819);
        let v1857: f64 = (-v523);
        let v1858: f64 = (v2 * v1820);
        let v1859: f64 = (v1857 + v1858);
        let v1860: f64 = (v2 * v1821);
        let v1861: f64 = (v523 + v1860);
        let v1862: f64 = (v1829 * v1856);
        let v1863: f64 = (v1829 * v1859);
        let v1864: f64 = (v1829 * v1861);
        let v1865: f64 = (if self.scalar_v583 { v1862 } else { v11 });
        let v1866: f64 = (if self.scalar_v583 { v1863 } else { v11 });
        let v1867: f64 = (if self.scalar_v583 { v1864 } else { v11 });
        let v1870: f64 = (-v119);
        let v1905: f64 = (self.scalar_v535 * v1829);
        let v1906: f64 = (if self.scalar_v534 { v1905 } else { v11 });
        let v1907: f64 = (if self.scalar_v623 { v1905 } else { v11 });
        let v1926: f64 = (self.scalar_v541 * v1829);
        let v1927: f64 = (if self.scalar_v540 { v1926 } else { v11 });
        let v1928: f64 = (if self.scalar_v642 { v1926 } else { v11 });
        let v1932: f64 = (v655 * v1829);
        let v1933: f64 = (if self.scalar_v549 { v1932 } else { v11 });
        let v1934: f64 = (self.scalar_v659 * v1829);
        let v1935: f64 = (if self.scalar_v571 { v1934 } else { v11 });

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
