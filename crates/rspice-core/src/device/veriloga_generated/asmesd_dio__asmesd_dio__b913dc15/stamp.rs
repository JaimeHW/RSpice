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
        let v0: f64 = ctx.temperature();
        let v1: f64 = ctx.node_voltage(nodes[2]);
        let v2: f64 = (v0 + v1);
        let v4: f64 = (v2 + self.scalar_v3);
        let v6: f64 = 1300.0;
        let v7: f64 = 173.14999999999998;
        let v8: bool = (v4 > v7);
        let v9: f64 = (if v8 { v4 } else { v7 });
        let v10: bool = (v6 < v9);
        let v11: f64 = (if v10 { v6 } else { v9 });
        let v12: f64 = 1.0;
        let v13: f64 = 0.0;
        let v19: f64 = 8.6170869e-5;
        let v20: f64 = (v11 * v19);
        let v21: f64 = (v11 / self.scalar_v18);
        let v22: f64 = ((v21) as f64).ln();
        let v24: f64 = (v22 * self.scalar_v23);
        let v26: f64 = (v21 - v12);
        let v27: f64 = (self.scalar_v25 * v26);
        let v28: f64 = (v27 / v20);
        let v29: f64 = (v24 + v28);
        let v31: f64 = (v22 * self.scalar_v30);
        let v33: f64 = ((v29) as f64).exp();
        let v34: f64 = (self.scalar_v32 * v33);
        let v36: f64 = ((v31) as f64).exp();
        let v37: f64 = (self.scalar_v35 * v36);
        let v40: f64 = (v26 * self.scalar_v39);
        let v41: f64 = (v12 + v40);
        let v42: f64 = (self.scalar_v38 * v41);
        let v45: f64 = (v26 * self.scalar_v44);
        let v46: f64 = (v12 + v45);
        let v47: f64 = (self.scalar_v43 * v46);
        let v50: f64 = (v26 * self.scalar_v49);
        let v51: f64 = (v12 + v50);
        let v52: f64 = (self.scalar_v48 * v51);
        let v54: f64 = 300.15;
        let v56: f64 = (v11 / v54);
        let v57: f64 = 1.16;
        let v58: f64 = 0.000702;
        let v59: f64 = (v11 * v58);
        let v60: f64 = (v11 * v59);
        let v61: f64 = 1108.0;
        let v62: f64 = (v11 + v61);
        let v63: f64 = (v60 / v62);
        let v64: f64 = (v57 - v63);
        let v65: f64 = (-v64);
        let v66: f64 = 1.3806226e-23;
        let v67: f64 = (v11 + v11);
        let v68: f64 = (v66 * v67);
        let v69: f64 = (v65 / v68);
        let v70: f64 = 1.3454442398941469e20;
        let v71: f64 = (v69 + v70);
        let v72: f64 = (v20 + v20);
        let v73: f64 = (-v72);
        let v74: f64 = 1.5;
        let v75: f64 = ((v56) as f64).ln();
        let v76: f64 = (v74 * v75);
        let v77: f64 = 1.6021918e-19;
        let v78: f64 = (v71 * v77);
        let v79: f64 = (v76 + v78);
        let v80: f64 = (v73 * v79);
        let v82: f64 = (self.scalar_v81 - v80);
        let v83: f64 = (v82 / self.scalar_v55);
        let v84: f64 = (self.scalar_v81 - v83);
        let v85: f64 = (v84 / v83);
        let v87: f64 = 0.0004;
        let v90: f64 = (self.scalar_v89 - v85);
        let v91: f64 = (self.scalar_v86 * v90);
        let v92: f64 = (v12 + v91);
        let v93: f64 = (self.scalar_v53 / v92);
        let v94: f64 = (v56 * v83);
        let v95: f64 = (v80 + v94);
        let v96: f64 = (v95 - v83);
        let v97: f64 = (v96 / v83);
        let v98: f64 = (v11 - v54);
        let v99: f64 = (v87 * v98);
        let v100: f64 = (v99 - v97);
        let v101: f64 = (self.scalar_v86 * v100);
        let v102: f64 = (v12 + v101);
        let v103: f64 = (v93 * v102);
        let v105: f64 = ctx.node_voltage(nodes[3]);
        let v106: f64 = ctx.node_voltage(nodes[4]);
        let v107: f64 = (v105 - v106);
        let v108: f64 = (self.scalar_v104 * v107);
        let v109: f64 = ctx.node_voltage(nodes[0]);
        let v110: f64 = (v109 - v105);
        let v111: f64 = (self.scalar_v104 * v110);
        let v112: f64 = ctx.node_voltage(nodes[1]);
        let v113: f64 = (v112 - v106);
        let v114: f64 = (self.scalar_v104 * v113);
        let v115: bool = (v34 > v13);
        let v117: f64 = (v20 * self.scalar_v116);
        let v118: f64 = (v108 / v117);
        let v119: f64 = (if v115 { v118 } else { v13 });
        let v120: f64 = (-v108);
        let v121: f64 = (v120 - v47);
        let v123: f64 = (v20 * self.scalar_v122);
        let v124: f64 = (v121 / v123);
        let v125: f64 = (if v115 { v124 } else { v13 });
        let v126: f64 = (-v47);
        let v127: f64 = (v126 / v123);
        let v128: f64 = (if v115 { v127 } else { v13 });
        let v129: f64 = 80.0;
        let v130: bool = (v119 > v129);
        let v131: bool = (v115 && v130);
        let v132: f64 = (v119 - v129);
        let v133: f64 = (v12 + v132);
        let v134: f64 = (if v131 { v133 } else { v13 });
        let v135: f64 = (if v131 { v129 } else { v119 });
        let v136: bool = (!v130);
        let v137: bool = (v115 && v136);
        let v138: f64 = (if v137 { v12 } else { v134 });
        let v139: f64 = ((v135) as f64).exp();
        let v140: f64 = (v138 * v139);
        let v141: f64 = (if v115 { v140 } else { v138 });
        let v142: f64 = 37.0;
        let v143: bool = (v125 >= v142);
        let v144: bool = (!v143);
        let v145: f64 = -37.0;
        let v146: bool = (v125 <= v145);
        let v147: bool = (!v146);
        let v148: bool = (v144 && v147);
        let v149: f64 = ((v125) as f64).exp();
        let v150: f64 = (v12 + v149);
        let v151: f64 = ((v150) as f64).ln();
        let v152: bool = (v144 && v146);
        let v153: f64 = (if v143 { v125 } else { v13 });
        let v154: f64 = (if v152 { v149 } else { v153 });
        let v155: f64 = (if v148 { v151 } else { v154 });
        let v156: bool = (v128 >= v142);
        let v157: bool = (!v156);
        let v158: bool = (v128 <= v145);
        let v159: bool = (!v158);
        let v160: bool = (v157 && v159);
        let v161: f64 = ((v128) as f64).exp();
        let v162: f64 = (v12 + v161);
        let v163: f64 = ((v162) as f64).ln();
        let v164: bool = (v157 && v158);
        let v165: f64 = (if v156 { v128 } else { v13 });
        let v166: f64 = (if v164 { v161 } else { v165 });
        let v167: f64 = (if v160 { v163 } else { v166 });
        let v168: f64 = (v155 - v167);
        let v169: f64 = (if v115 { v168 } else { v13 });
        let v170: f64 = (v141 - v12);
        let v171: f64 = (v34 * v170);
        let v172: f64 = (v42 * v169);
        let v174: f64 = ((v108) as f64).abs();
        let v175: f64 = f64::powf(v174, v52);
        let v176: f64 = (self.scalar_v173 * v175);
        let v177: f64 = (v12 + v176);
        let v178: f64 = (v172 / v177);
        let v179: f64 = (v171 - v178);
        let v180: f64 = (if v115 { v179 } else { v13 });
        let v181: bool = (!v115);
        let v182: f64 = (if v181 { v13 } else { v180 });
        let v183: bool = (v37 > v13);
        let v185: f64 = (self.scalar_v184 - v108);
        let v186: f64 = 0.001;
        let v187: bool = (v185 > v186);
        let v188: f64 = (if v187 { v185 } else { v186 });
        let v189: f64 = (if v183 { v188 } else { v13 });
        let v190: f64 = -1.0;
        let v191: f64 = (v120 * self.scalar_v184);
        let v193: f64 = (v20 * self.scalar_v192);
        let v194: f64 = (v189 * v193);
        let v195: f64 = (v191 / v194);
        let v196: f64 = (if v183 { v195 } else { v135 });
        let v197: bool = (v196 > v129);
        let v198: bool = (v183 && v197);
        let v199: f64 = (v196 - v129);
        let v200: f64 = (v12 + v199);
        let v201: f64 = (if v198 { v200 } else { v141 });
        let v202: f64 = (if v198 { v129 } else { v196 });
        let v203: bool = (!v197);
        let v204: bool = (v183 && v203);
        let v205: f64 = (if v204 { v12 } else { v201 });
        let v206: f64 = ((v202) as f64).exp();
        let v207: f64 = (v205 * v206);
        let v208: f64 = (if v183 { v207 } else { v205 });
        let v209: f64 = (v208 - v12);
        let v210: f64 = (v37 * v209);
        let v211: f64 = (if v183 { v210 } else { v13 });
        let v212: bool = (!v183);
        let v213: f64 = (if v212 { v13 } else { v211 });
        let v214: f64 = (v182 - v213);
        let v216: f64 = (v111 / self.scalar_v215);
        let v217: f64 = ((v216) as f64).abs();
        let v219: f64 = f64::powf(v217, self.scalar_v218);
        let v220: f64 = (v12 + v219);
        let v222: f64 = (v114 / self.scalar_v221);
        let v223: f64 = ((v222) as f64).abs();
        let v225: f64 = f64::powf(v223, self.scalar_v224);
        let v226: f64 = (v12 + v225);
        let v229: f64 = (v22 * self.scalar_v228);
        let v230: f64 = ((v229) as f64).exp();
        let v231: f64 = (self.scalar_v227 * v230);
        let v233: f64 = f64::powf(v220, self.scalar_v232);
        let v234: f64 = (v231 * v233);
        let v237: f64 = (v22 * self.scalar_v236);
        let v238: f64 = ((v237) as f64).exp();
        let v239: f64 = (self.scalar_v235 * v238);
        let v241: f64 = f64::powf(v226, self.scalar_v240);
        let v242: f64 = (v239 * v241);
        let v246: f64 = (v234 + self.scalar_v245);
        let v247: f64 = (if self.scalar_v244 { v246 } else { v234 });
        let v249: f64 = (v242 + self.scalar_v248);
        let v250: f64 = (if self.scalar_v244 { v249 } else { v242 });
        let v251: f64 = (v109 - v112);
        let v253: f64 = (v251 / self.scalar_v252);
        let v254: f64 = ((v253) as f64).abs();
        let v256: f64 = f64::powf(v254, self.scalar_v255);
        let v257: f64 = (v12 + v256);
        let v259: f64 = f64::powf(v257, self.scalar_v258);
        let v260: f64 = (v259 - v12);
        let v263: f64 = (v260 * self.scalar_v262);
        let v264: f64 = (v12 + v263);
        let v265: f64 = (self.scalar_v261 * v264);
        let v266: f64 = (v182 * v265);
        let v269: f64 = ctx.node_voltage(nodes[6]);
        let v270: f64 = ((v269) as f64).abs();
        let v272: f64 = (v270 / self.scalar_v271);
        let v274: f64 = f64::powf(v272, self.scalar_v273);
        let v275: f64 = (v12 + v274);
        let v276: f64 = (v247 / v275);
        let v277: f64 = (if self.scalar_v268 { v276 } else { v247 });
        let v279: f64 = (if self.scalar_v278 { v277 } else { v277 });
        let v280: f64 = (-v95);
        let v282: f64 = (v280 * self.scalar_v281);
        let v283: f64 = (v108 + v282);
        let v284: bool = (v283 > v13);
        let v290: f64 = (if v284 { self.scalar_v289 } else { v13 });
        let v291: f64 = (self.scalar_v286 * v290);
        let v292: f64 = (self.scalar_v286 * v291);
        let v293: f64 = (v12 - v292);
        let v294: f64 = (v95 * v293);
        let v296: f64 = (v294 / self.scalar_v295);
        let v297: f64 = (if v284 { v296 } else { v13 });
        let v300: f64 = (v283 * self.scalar_v299);
        let v301: f64 = (v300 / v95);
        let v302: f64 = (self.scalar_v286 + v301);
        let v303: f64 = (v283 * v302);
        let v304: f64 = (v290 * v303);
        let v305: f64 = (if v284 { v304 } else { v13 });
        let v306: bool = (!v284);
        let v307: f64 = (v108 / v95);
        let v308: f64 = (v12 - v307);
        let v309: f64 = ((v308) as f64).ln();
        let v310: f64 = (self.scalar_v295 * v309);
        let v311: f64 = ((v310) as f64).exp();
        let v312: f64 = (v12 - v311);
        let v313: f64 = (v95 * v312);
        let v314: f64 = (v313 / self.scalar_v295);
        let v315: f64 = (if v306 { v314 } else { v297 });
        let v316: f64 = (if v306 { v13 } else { v305 });
        let v317: f64 = (v315 + v316);
        let v318: f64 = (v103 * v317);
        let v341: f64 = (v279 / self.scalar_v16);
        let v345: f64 = (v250 / self.scalar_v16);
        let v346: f64 = (-v182);
        let v347: f64 = (v265 * v346);
        let v348: f64 = (if self.scalar_v268 { v347 } else { v13 });
        let v349: f64 = (if self.scalar_v268 { v269 } else { v13 });
        let v350: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, v269);
        let v351: f64 = (v265 * v350);
        let v352: f64 = (if self.scalar_v268 { v351 } else { v13 });
        let v354: f64 = (v214 * v251);
        let v355: f64 = ((v354) as f64).abs();
        let v356: f64 = (-v355);
        let v357: f64 = (if self.scalar_v323 { v356 } else { v13 });
        let v358: f64 = (v1 / self.scalar_v321);
        let v359: f64 = (if self.scalar_v323 { v358 } else { v13 });
        let v361: f64 = (v1 * self.scalar_v360);
        let v362: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, v361);
        let v363: f64 = (if self.scalar_v323 { v362 } else { v13 });
        let v367: f64 = (if self.scalar_v366 { v356 } else { v13 });
        let v368: f64 = ctx.node_voltage(nodes[5]);
        let v369: f64 = (v1 - v368);
        let v370: f64 = (v369 / self.scalar_v321);
        let v371: f64 = (if self.scalar_v366 { v370 } else { v13 });
        let v372: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, v361);
        let v373: f64 = (if self.scalar_v366 { v372 } else { v13 });
        let v374: f64 = (v368 / self.scalar_v327);
        let v375: f64 = (if self.scalar_v366 { v374 } else { v13 });
        let v377: f64 = (v368 * self.scalar_v376);
        let v378: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, v377);
        let v379: f64 = (if self.scalar_v366 { v378 } else { v13 });
        let v383: f64 = (if self.scalar_v382 { v356 } else { v13 });
        let v388: f64 = (v13 * v107);
        let v389: bool = (v341 > self.scalar_v338);
        let v390: f64 = (if v389 { v341 } else { self.scalar_v338 });
        let v391: f64 = (v110 / v390);
        let v392: f64 = (if self.scalar_v340 { v391 } else { v13 });
        let v396: bool = (v345 > self.scalar_v338);
        let v397: f64 = (if v396 { v345 } else { self.scalar_v338 });
        let v398: f64 = (v113 / v397);
        let v399: f64 = (if self.scalar_v344 { v398 } else { v13 });
        let v403: f64 = (self.scalar_v104 * v214);
        let v404: f64 = (self.scalar_v16 * v403);
        let v405: f64 = (self.scalar_v104 * v318);
        let v406: f64 = (self.scalar_v16 * v405);
        let v407: f64 = (self.scalar_v104 * v266);
        let v408: f64 = (self.scalar_v16 * v407);
        let v409: f64 = (if v8 { v12 } else { v13 });
        let v410: f64 = (if v10 { v13 } else { v409 });
        let v411: f64 = (v19 * v410);
        let v412: f64 = (v410 / self.scalar_v18);
        let v413: f64 = (v412 / v21);
        let v414: f64 = (self.scalar_v23 * v413);
        let v415: f64 = (self.scalar_v25 * v412);
        let v416: f64 = (v20 * v415);
        let v417: f64 = (v27 * v411);
        let v418: f64 = (v416 - v417);
        let v419: f64 = (v20 * v20);
        let v420: f64 = (v418 / v419);
        let v421: f64 = (v414 + v420);
        let v422: f64 = (self.scalar_v30 * v413);
        let v423: f64 = (v33 * v421);
        let v424: f64 = (self.scalar_v32 * v423);
        let v425: f64 = (v36 * v422);
        let v426: f64 = (self.scalar_v35 * v425);
        let v427: f64 = (self.scalar_v39 * v412);
        let v428: f64 = (self.scalar_v38 * v427);
        let v429: f64 = (self.scalar_v44 * v412);
        let v430: f64 = (self.scalar_v43 * v429);
        let v431: f64 = (self.scalar_v49 * v412);
        let v432: f64 = (self.scalar_v48 * v431);
        let v433: f64 = (v410 / v54);
        let v434: f64 = (v58 * v410);
        let v435: f64 = (v59 * v410);
        let v436: f64 = (v11 * v434);
        let v437: f64 = (v435 + v436);
        let v438: f64 = (v62 * v437);
        let v439: f64 = (v60 * v410);
        let v440: f64 = (v438 - v439);
        let v441: f64 = (v62 * v62);
        let v442: f64 = (v440 / v441);
        let v443: f64 = (v410 + v410);
        let v444: f64 = (v66 * v443);
        let v445: f64 = (v68 * v442);
        let v446: f64 = (v65 * v444);
        let v447: f64 = (v445 - v446);
        let v448: f64 = (v68 * v68);
        let v449: f64 = (v447 / v448);
        let v450: f64 = (v411 + v411);
        let v451: f64 = (-v450);
        let v452: f64 = (v433 / v56);
        let v453: f64 = (v74 * v452);
        let v454: f64 = (v77 * v449);
        let v455: f64 = (v453 + v454);
        let v456: f64 = (v79 * v451);
        let v457: f64 = (v73 * v455);
        let v458: f64 = (v456 + v457);
        let v459: f64 = (-v458);
        let v460: f64 = (v459 / self.scalar_v55);
        let v461: f64 = (-v460);
        let v462: f64 = (v83 * v461);
        let v463: f64 = (v84 * v460);
        let v464: f64 = (v462 - v463);
        let v465: f64 = (v83 * v83);
        let v466: f64 = (v464 / v465);
        let v467: f64 = (-v466);
        let v468: f64 = (self.scalar_v86 * v467);
        let v469: f64 = (self.scalar_v53 * v468);
        let v470: f64 = (-v469);
        let v471: f64 = (v92 * v92);
        let v472: f64 = (v470 / v471);
        let v473: f64 = (v83 * v433);
        let v474: f64 = (v56 * v460);
        let v475: f64 = (v473 + v474);
        let v476: f64 = (v458 + v475);
        let v477: f64 = (v476 - v460);
        let v478: f64 = (v83 * v477);
        let v479: f64 = (v96 * v460);
        let v480: f64 = (v478 - v479);
        let v481: f64 = (v480 / v465);
        let v482: f64 = (v87 * v410);
        let v483: f64 = (v482 - v481);
        let v484: f64 = (self.scalar_v86 * v483);
        let v485: f64 = (v102 * v472);
        let v486: f64 = (v93 * v484);
        let v487: f64 = (v485 + v486);
        let v489: f64 = (self.scalar_v116 * v411);
        let v490: f64 = (v108 * v489);
        let v491: f64 = (-v490);
        let v492: f64 = (v117 * v117);
        let v493: f64 = (v491 / v492);
        let v494: f64 = (self.scalar_v104 / v117);
        let v495: f64 = (self.scalar_v488 / v117);
        let v496: f64 = (if v115 { v493 } else { v13 });
        let v497: f64 = (if v115 { v494 } else { v13 });
        let v498: f64 = (if v115 { v495 } else { v13 });
        let v499: f64 = (-v430);
        let v500: f64 = (self.scalar_v122 * v411);
        let v501: f64 = (v123 * v499);
        let v502: f64 = (v121 * v500);
        let v503: f64 = (v501 - v502);
        let v504: f64 = (v123 * v123);
        let v505: f64 = (v503 / v504);
        let v506: f64 = (self.scalar_v488 / v123);
        let v507: f64 = (self.scalar_v104 / v123);
        let v508: f64 = (if v115 { v505 } else { v13 });
        let v509: f64 = (if v115 { v506 } else { v13 });
        let v510: f64 = (if v115 { v507 } else { v13 });
        let v511: f64 = (v126 * v500);
        let v512: f64 = (v501 - v511);
        let v513: f64 = (v512 / v504);
        let v514: f64 = (if v115 { v513 } else { v13 });
        let v515: f64 = (if v131 { v496 } else { v13 });
        let v516: f64 = (if v131 { v497 } else { v13 });
        let v517: f64 = (if v131 { v498 } else { v13 });
        let v518: f64 = (if v131 { v13 } else { v496 });
        let v519: f64 = (if v131 { v13 } else { v497 });
        let v520: f64 = (if v131 { v13 } else { v498 });
        let v521: f64 = (if v137 { v13 } else { v515 });
        let v522: f64 = (if v137 { v13 } else { v516 });
        let v523: f64 = (if v137 { v13 } else { v517 });
        let v524: f64 = (v139 * v518);
        let v525: f64 = (v139 * v519);
        let v526: f64 = (v139 * v520);
        let v527: f64 = (v139 * v521);
        let v528: f64 = (v138 * v524);
        let v529: f64 = (v527 + v528);
        let v530: f64 = (v139 * v522);
        let v531: f64 = (v138 * v525);
        let v532: f64 = (v530 + v531);
        let v533: f64 = (v139 * v523);
        let v534: f64 = (v138 * v526);
        let v535: f64 = (v533 + v534);
        let v536: f64 = (if v115 { v529 } else { v521 });
        let v537: f64 = (if v115 { v532 } else { v522 });
        let v538: f64 = (if v115 { v535 } else { v523 });
        let v539: f64 = (v149 * v508);
        let v540: f64 = (v149 * v509);
        let v541: f64 = (v149 * v510);
        let v542: f64 = (v539 / v150);
        let v543: f64 = (v540 / v150);
        let v544: f64 = (v541 / v150);
        let v545: f64 = (if v143 { v508 } else { v13 });
        let v546: f64 = (if v143 { v509 } else { v13 });
        let v547: f64 = (if v143 { v510 } else { v13 });
        let v548: f64 = (if v152 { v539 } else { v545 });
        let v549: f64 = (if v152 { v540 } else { v546 });
        let v550: f64 = (if v152 { v541 } else { v547 });
        let v551: f64 = (if v148 { v542 } else { v548 });
        let v552: f64 = (if v148 { v543 } else { v549 });
        let v553: f64 = (if v148 { v544 } else { v550 });
        let v554: f64 = (v161 * v514);
        let v555: f64 = (v554 / v162);
        let v556: f64 = (if v156 { v514 } else { v13 });
        let v557: f64 = (if v164 { v554 } else { v556 });
        let v558: f64 = (if v160 { v555 } else { v557 });
        let v559: f64 = (v551 - v558);
        let v560: f64 = (if v115 { v559 } else { v13 });
        let v561: f64 = (if v115 { v552 } else { v13 });
        let v562: f64 = (if v115 { v553 } else { v13 });
        let v563: f64 = (v170 * v424);
        let v564: f64 = (v34 * v536);
        let v565: f64 = (v563 + v564);
        let v566: f64 = (v34 * v537);
        let v567: f64 = (v34 * v538);
        let v568: f64 = (v169 * v428);
        let v569: f64 = (v42 * v560);
        let v570: f64 = (v568 + v569);
        let v571: f64 = (v42 * v561);
        let v572: f64 = (v42 * v562);
        let v573: f64 = ((v174) as f64).ln();
        let v574: f64 = (v175 * v573);
        let v575: f64 = (v432 * v574);
        let v576: f64 = (self.scalar_v173 * v575);
        let v577: f64 = (v177 * v570);
        let v578: f64 = (v172 * v576);
        let v579: f64 = (v577 - v578);
        let v580: f64 = (v177 * v177);
        let v581: f64 = (v579 / v580);
        let v582: f64 = (v571 / v177);
        let v583: f64 = (v572 / v177);
        let v584: f64 = (v565 - v581);
        let v585: f64 = (v566 - v582);
        let v586: f64 = (v567 - v583);
        let v587: f64 = (if v115 { v584 } else { v13 });
        let v588: f64 = (if v115 { v585 } else { v13 });
        let v589: f64 = (if v115 { v586 } else { v13 });
        let v590: f64 = (if v181 { v13 } else { v587 });
        let v591: f64 = (if v181 { v13 } else { v588 });
        let v592: f64 = (if v181 { v13 } else { v589 });
        let v593: f64 = (if v187 { self.scalar_v488 } else { v13 });
        let v594: f64 = (if v187 { self.scalar_v104 } else { v13 });
        let v595: f64 = (if v183 { v593 } else { v13 });
        let v596: f64 = (if v183 { v594 } else { v13 });
        let v599: f64 = (self.scalar_v192 * v411);
        let v600: f64 = (v189 * v599);
        let v601: f64 = (v193 * v595);
        let v602: f64 = (v193 * v596);
        let v603: f64 = (v191 * v600);
        let v604: f64 = (-v603);
        let v605: f64 = (v194 * v194);
        let v606: f64 = (v604 / v605);
        let v607: f64 = (v194 * self.scalar_v597);
        let v608: f64 = (v191 * v601);
        let v609: f64 = (v607 - v608);
        let v610: f64 = (v609 / v605);
        let v611: f64 = (v194 * self.scalar_v598);
        let v612: f64 = (v191 * v602);
        let v613: f64 = (v611 - v612);
        let v614: f64 = (v613 / v605);
        let v615: f64 = (if v183 { v606 } else { v518 });
        let v616: f64 = (if v183 { v610 } else { v519 });
        let v617: f64 = (if v183 { v614 } else { v520 });
        let v618: f64 = (if v198 { v615 } else { v536 });
        let v619: f64 = (if v198 { v616 } else { v537 });
        let v620: f64 = (if v198 { v617 } else { v538 });
        let v621: f64 = (if v198 { v13 } else { v615 });
        let v622: f64 = (if v198 { v13 } else { v616 });
        let v623: f64 = (if v198 { v13 } else { v617 });
        let v624: f64 = (if v204 { v13 } else { v618 });
        let v625: f64 = (if v204 { v13 } else { v619 });
        let v626: f64 = (if v204 { v13 } else { v620 });
        let v627: f64 = (v206 * v621);
        let v628: f64 = (v206 * v622);
        let v629: f64 = (v206 * v623);
        let v630: f64 = (v206 * v624);
        let v631: f64 = (v205 * v627);
        let v632: f64 = (v630 + v631);
        let v633: f64 = (v206 * v625);
        let v634: f64 = (v205 * v628);
        let v635: f64 = (v633 + v634);
        let v636: f64 = (v206 * v626);
        let v637: f64 = (v205 * v629);
        let v638: f64 = (v636 + v637);
        let v639: f64 = (if v183 { v632 } else { v624 });
        let v640: f64 = (if v183 { v635 } else { v625 });
        let v641: f64 = (if v183 { v638 } else { v626 });
        let v642: f64 = (v209 * v426);
        let v643: f64 = (v37 * v639);
        let v644: f64 = (v642 + v643);
        let v645: f64 = (v37 * v640);
        let v646: f64 = (v37 * v641);
        let v647: f64 = (if v183 { v644 } else { v13 });
        let v648: f64 = (if v183 { v645 } else { v13 });
        let v649: f64 = (if v183 { v646 } else { v13 });
        let v650: f64 = (if v212 { v13 } else { v647 });
        let v651: f64 = (if v212 { v13 } else { v648 });
        let v652: f64 = (if v212 { v13 } else { v649 });
        let v653: f64 = (v590 - v650);
        let v654: f64 = (v591 - v651);
        let v655: f64 = (v592 - v652);
        let v656: f64 = (self.scalar_v228 * v413);
        let v657: f64 = (v230 * v656);
        let v658: f64 = (self.scalar_v227 * v657);
        let v659: f64 = (v233 * v658);
        let v660: f64 = (self.scalar_v236 * v413);
        let v661: f64 = (v238 * v660);
        let v662: f64 = (self.scalar_v235 * v661);
        let v663: f64 = (v241 * v662);
        let v664: f64 = (if self.scalar_v244 { v659 } else { v659 });
        let v665: f64 = (if self.scalar_v244 { v663 } else { v663 });
        let v666: f64 = (v265 * v590);
        let v667: f64 = (v265 * v591);
        let v668: f64 = (v265 * v592);
        let v669: f64 = (v664 / v275);
        let v670: f64 = (if self.scalar_v268 { v669 } else { v664 });
        let v671: f64 = (if self.scalar_v278 { v670 } else { v670 });
        let v672: f64 = (-v476);
        let v673: f64 = (self.scalar_v281 * v672);
        let v674: f64 = (v293 * v476);
        let v675: f64 = (v674 / self.scalar_v295);
        let v676: f64 = (if v284 { v675 } else { v13 });
        let v677: f64 = (self.scalar_v299 * v673);
        let v680: f64 = (v95 * v677);
        let v681: f64 = (v300 * v476);
        let v682: f64 = (v680 - v681);
        let v683: f64 = (v95 * v95);
        let v684: f64 = (v682 / v683);
        let v685: f64 = (self.scalar_v678 / v95);
        let v686: f64 = (self.scalar_v679 / v95);
        let v687: f64 = (v302 * v673);
        let v688: f64 = (v283 * v684);
        let v689: f64 = (v687 + v688);
        let v690: f64 = (self.scalar_v104 * v302);
        let v691: f64 = (v283 * v685);
        let v692: f64 = (v690 + v691);
        let v693: f64 = (v302 * self.scalar_v488);
        let v694: f64 = (v283 * v686);
        let v695: f64 = (v693 + v694);
        let v696: f64 = (v290 * v689);
        let v697: f64 = (v290 * v692);
        let v698: f64 = (v290 * v695);
        let v699: f64 = (if v284 { v696 } else { v13 });
        let v700: f64 = (if v284 { v697 } else { v13 });
        let v701: f64 = (if v284 { v698 } else { v13 });
        let v702: f64 = (v108 * v476);
        let v703: f64 = (-v702);
        let v704: f64 = (v703 / v683);
        let v705: f64 = (self.scalar_v104 / v95);
        let v706: f64 = (self.scalar_v488 / v95);
        let v707: f64 = (-v704);
        let v708: f64 = (-v705);
        let v709: f64 = (-v706);
        let v710: f64 = (v707 / v308);
        let v711: f64 = (v708 / v308);
        let v712: f64 = (v709 / v308);
        let v713: f64 = (self.scalar_v295 * v710);
        let v714: f64 = (self.scalar_v295 * v711);
        let v715: f64 = (self.scalar_v295 * v712);
        let v716: f64 = (v311 * v713);
        let v717: f64 = (v311 * v714);
        let v718: f64 = (v311 * v715);
        let v719: f64 = (-v716);
        let v720: f64 = (-v717);
        let v721: f64 = (-v718);
        let v722: f64 = (v312 * v476);
        let v723: f64 = (v95 * v719);
        let v724: f64 = (v722 + v723);
        let v725: f64 = (v95 * v720);
        let v726: f64 = (v95 * v721);
        let v727: f64 = (v724 / self.scalar_v295);
        let v728: f64 = (v725 / self.scalar_v295);
        let v729: f64 = (v726 / self.scalar_v295);
        let v730: f64 = (if v306 { v727 } else { v676 });
        let v731: f64 = (if v306 { v728 } else { v13 });
        let v732: f64 = (if v306 { v729 } else { v13 });
        let v733: f64 = (if v306 { v13 } else { v699 });
        let v734: f64 = (if v306 { v13 } else { v700 });
        let v735: f64 = (if v306 { v13 } else { v701 });
        let v736: f64 = (v730 + v733);
        let v737: f64 = (v731 + v734);
        let v738: f64 = (v732 + v735);
        let v739: f64 = (v317 * v487);
        let v740: f64 = (v103 * v736);
        let v741: f64 = (v739 + v740);
        let v742: f64 = (v103 * v737);
        let v743: f64 = (v103 * v738);
        let v744: f64 = (v671 / self.scalar_v16);
        let v745: f64 = (v665 / self.scalar_v16);
        let v746: f64 = (-v590);
        let v747: f64 = (-v591);
        let v748: f64 = (-v592);
        let v749: f64 = (v265 * v746);
        let v750: f64 = (v265 * v747);
        let v751: f64 = (v265 * v748);
        let v752: f64 = (if self.scalar_v268 { v749 } else { v13 });
        let v753: f64 = (if self.scalar_v268 { v750 } else { v13 });
        let v754: f64 = (if self.scalar_v268 { v751 } else { v13 });
        let v756: f64 = ddt_scale;
        let v757: f64 = (v265 * v756);
        let v758: f64 = (if self.scalar_v268 { v757 } else { v13 });
        let v761: f64 = (self.scalar_v360 * v756);
        let v762: f64 = (if self.scalar_v323 { v761 } else { v13 });
        let v766: f64 = (if self.scalar_v366 { v761 } else { v13 });
        let v769: f64 = (self.scalar_v376 * v756);
        let v770: f64 = (if self.scalar_v366 { v769 } else { v13 });
        let v771: f64 = -0.0;
        let v772: f64 = (if v389 { v744 } else { v13 });
        let v773: f64 = (v12 / v390);
        let v774: f64 = (v110 * v772);
        let v775: f64 = (-v774);
        let v776: f64 = (v390 * v390);
        let v777: f64 = (v775 / v776);
        let v778: f64 = (v190 / v390);
        let v779: f64 = (if self.scalar_v340 { v773 } else { v13 });
        let v780: f64 = (if self.scalar_v340 { v777 } else { v13 });
        let v781: f64 = (if self.scalar_v340 { v778 } else { v13 });
        let v782: f64 = (if v396 { v745 } else { v13 });
        let v783: f64 = (v12 / v397);
        let v784: f64 = (v113 * v782);
        let v785: f64 = (-v784);
        let v786: f64 = (v397 * v397);
        let v787: f64 = (v785 / v786);
        let v788: f64 = (v190 / v397);
        let v789: f64 = (if self.scalar_v344 { v783 } else { v13 });
        let v790: f64 = (if self.scalar_v344 { v787 } else { v13 });
        let v791: f64 = (if self.scalar_v344 { v788 } else { v13 });
        let v792: f64 = (self.scalar_v104 * v653);
        let v793: f64 = (self.scalar_v104 * v654);
        let v794: f64 = (self.scalar_v104 * v655);
        let v795: f64 = (self.scalar_v16 * v792);
        let v796: f64 = (self.scalar_v16 * v793);
        let v797: f64 = (self.scalar_v16 * v794);
        let v798: f64 = (self.scalar_v104 * v741);
        let v799: f64 = (self.scalar_v104 * v742);
        let v800: f64 = (self.scalar_v104 * v743);
        let v801: f64 = (self.scalar_v16 * v798);
        let v802: f64 = (self.scalar_v16 * v799);
        let v803: f64 = (self.scalar_v16 * v800);
        let v804: f64 = (self.scalar_v104 * v666);
        let v805: f64 = (self.scalar_v104 * v667);
        let v806: f64 = (self.scalar_v104 * v668);
        let v807: f64 = (self.scalar_v16 * v804);
        let v808: f64 = (self.scalar_v16 * v805);
        let v809: f64 = (self.scalar_v16 * v806);

        let d348_dn2: f64 = v752;
        let d348_dn3: f64 = v753;
        let d348_dn4: f64 = v754;
        stamper.stamp_current_node3_local(
            Some(6),
            None,
            multiplicity * (v348),
            2,
            multiplicity * (d348_dn2),
            3,
            multiplicity * (d348_dn3),
            4,
            multiplicity * (d348_dn4),
        );
        let d349_dn6: f64 = self.scalar_v755;
        stamper.stamp_current_node1_local(
            Some(6),
            None,
            multiplicity * (v349),
            6,
            multiplicity * (d349_dn6),
        );
        let d352_dn6: f64 = v758;
        stamper.stamp_current_node1_local(
            Some(6),
            None,
            multiplicity * (v352),
            6,
            multiplicity * (d352_dn6),
        );
        stamper.stamp_potential_branch_local(
            Some(6),
            None,
            0,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            0,
            self.scalar_v353,
        );
        stamper.stamp_current_const_local(
            Some(2),
            None,
            multiplicity * (v357),
        );
        let d359_dn2: f64 = self.scalar_v760;
        stamper.stamp_current_node1_local(
            Some(2),
            None,
            multiplicity * (v359),
            2,
            multiplicity * (d359_dn2),
        );
        let d363_dn2: f64 = v762;
        stamper.stamp_current_node1_local(
            Some(2),
            None,
            multiplicity * (v363),
            2,
            multiplicity * (d363_dn2),
        );
        stamper.stamp_potential_branch_local(
            Some(5),
            None,
            1,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            1,
            self.scalar_v364,
        );
        stamper.stamp_current_const_local(
            Some(2),
            None,
            multiplicity * (v367),
        );
        let d371_dn2: f64 = self.scalar_v764;
        let d371_dn5: f64 = self.scalar_v765;
        stamper.stamp_current_node2_local(
            Some(2),
            Some(5),
            multiplicity * (v371),
            2,
            multiplicity * (d371_dn2),
            5,
            multiplicity * (d371_dn5),
        );
        let d373_dn2: f64 = v766;
        stamper.stamp_current_node1_local(
            Some(2),
            None,
            multiplicity * (v373),
            2,
            multiplicity * (d373_dn2),
        );
        let d375_dn5: f64 = self.scalar_v768;
        stamper.stamp_current_node1_local(
            Some(5),
            None,
            multiplicity * (v375),
            5,
            multiplicity * (d375_dn5),
        );
        let d379_dn5: f64 = v770;
        stamper.stamp_current_node1_local(
            Some(5),
            None,
            multiplicity * (v379),
            5,
            multiplicity * (d379_dn5),
        );
        stamper.stamp_current_const_local(
            Some(2),
            None,
            multiplicity * (v383),
        );
        stamper.stamp_potential_branch_local(
            Some(5),
            None,
            2,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            2,
            self.scalar_v384,
        );
        stamper.stamp_potential_branch_local(
            Some(2),
            None,
            3,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            3,
            self.scalar_v387,
        );
        stamper.stamp_potential_branch_local(
            Some(5),
            None,
            4,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            4,
            self.scalar_v387,
        );
        let d388_dn3: f64 = v13;
        let d388_dn4: f64 = v771;
        stamper.stamp_current_node2_local(
            Some(3),
            Some(4),
            multiplicity * (v388),
            3,
            multiplicity * (d388_dn3),
            4,
            multiplicity * (d388_dn4),
        );
        let d392_dn0: f64 = v779;
        let d392_dn2: f64 = v780;
        let d392_dn3: f64 = v781;
        stamper.stamp_current_node3_local(
            Some(0),
            Some(3),
            multiplicity * (v392),
            0,
            multiplicity * (d392_dn0),
            2,
            multiplicity * (d392_dn2),
            3,
            multiplicity * (d392_dn3),
        );
        stamper.stamp_current_const_local(
            Some(0),
            Some(3),
            multiplicity * (self.scalar_v393),
        );
        stamper.stamp_potential_branch_local(
            Some(0),
            Some(3),
            5,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            5,
            self.scalar_v395,
        );
        let d399_dn1: f64 = v789;
        let d399_dn2: f64 = v790;
        let d399_dn4: f64 = v791;
        stamper.stamp_current_node3_local(
            Some(1),
            Some(4),
            multiplicity * (v399),
            1,
            multiplicity * (d399_dn1),
            2,
            multiplicity * (d399_dn2),
            4,
            multiplicity * (d399_dn4),
        );
        stamper.stamp_current_const_local(
            Some(1),
            Some(4),
            multiplicity * (self.scalar_v400),
        );
        stamper.stamp_potential_branch_local(
            Some(1),
            Some(4),
            6,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            6,
            self.scalar_v402,
        );
        let d404_dn2: f64 = v795;
        let d404_dn3: f64 = v796;
        let d404_dn4: f64 = v797;
        stamper.stamp_current_node3_local(
            Some(3),
            Some(4),
            multiplicity * (v404),
            2,
            multiplicity * (d404_dn2),
            3,
            multiplicity * (d404_dn3),
            4,
            multiplicity * (d404_dn4),
        );
        let d406_dn2: f64 = v801;
        let d406_dn3: f64 = v802;
        let d406_dn4: f64 = v803;
        let v406_ddt: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, v406);
        stamper.stamp_current_node3_local(
            Some(3),
            Some(4),
            multiplicity * (v406_ddt),
            2,
            multiplicity * (((d406_dn2) * ddt_scale)),
            3,
            multiplicity * (((d406_dn3) * ddt_scale)),
            4,
            multiplicity * (((d406_dn4) * ddt_scale)),
        );
        let d408_dn2: f64 = v807;
        let d408_dn3: f64 = v808;
        let d408_dn4: f64 = v809;
        let v408_ddt: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 5, v408);
        stamper.stamp_current_node3_local(
            Some(3),
            Some(4),
            multiplicity * (v408_ddt),
            2,
            multiplicity * (((d408_dn2) * ddt_scale)),
            3,
            multiplicity * (((d408_dn3) * ddt_scale)),
            4,
            multiplicity * (((d408_dn4) * ddt_scale)),
        );
        stamper.stamp_current_const_local(
            Some(3),
            Some(4),
            multiplicity * (v13),
        );
        stamper.stamp_current_const_local(
            Some(3),
            Some(4),
            multiplicity * (v13),
        );
    }

    pub fn stamp_reactive(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedReactiveStamper<'_>) {
        let nodes = self.nodes;
        let branches = self.branches;
        let p = &(*self.params);
        let multiplicity = self.multiplicity;
        let v0: f64 = ctx.temperature();
        let v1: f64 = ctx.node_voltage(nodes[2]);
        let v2: f64 = (v0 + v1);
        let v4: f64 = (v2 + self.scalar_v3);
        let v6: f64 = 1300.0;
        let v7: f64 = 173.14999999999998;
        let v8: bool = (v4 > v7);
        let v9: f64 = (if v8 { v4 } else { v7 });
        let v10: bool = (v6 < v9);
        let v11: f64 = (if v10 { v6 } else { v9 });
        let v12: f64 = 1.0;
        let v13: f64 = 0.0;
        let v19: f64 = 8.6170869e-5;
        let v20: f64 = (v11 * v19);
        let v21: f64 = (v11 / self.scalar_v18);
        let v22: f64 = ((v21) as f64).ln();
        let v24: f64 = (v22 * self.scalar_v23);
        let v26: f64 = (v21 - v12);
        let v27: f64 = (self.scalar_v25 * v26);
        let v28: f64 = (v27 / v20);
        let v29: f64 = (v24 + v28);
        let v33: f64 = ((v29) as f64).exp();
        let v34: f64 = (self.scalar_v32 * v33);
        let v40: f64 = (v26 * self.scalar_v39);
        let v41: f64 = (v12 + v40);
        let v42: f64 = (self.scalar_v38 * v41);
        let v45: f64 = (v26 * self.scalar_v44);
        let v46: f64 = (v12 + v45);
        let v47: f64 = (self.scalar_v43 * v46);
        let v50: f64 = (v26 * self.scalar_v49);
        let v51: f64 = (v12 + v50);
        let v52: f64 = (self.scalar_v48 * v51);
        let v54: f64 = 300.15;
        let v56: f64 = (v11 / v54);
        let v57: f64 = 1.16;
        let v58: f64 = 0.000702;
        let v59: f64 = (v11 * v58);
        let v60: f64 = (v11 * v59);
        let v61: f64 = 1108.0;
        let v62: f64 = (v11 + v61);
        let v63: f64 = (v60 / v62);
        let v64: f64 = (v57 - v63);
        let v65: f64 = (-v64);
        let v66: f64 = 1.3806226e-23;
        let v67: f64 = (v11 + v11);
        let v68: f64 = (v66 * v67);
        let v69: f64 = (v65 / v68);
        let v70: f64 = 1.3454442398941469e20;
        let v71: f64 = (v69 + v70);
        let v72: f64 = (v20 + v20);
        let v73: f64 = (-v72);
        let v74: f64 = 1.5;
        let v75: f64 = ((v56) as f64).ln();
        let v76: f64 = (v74 * v75);
        let v77: f64 = 1.6021918e-19;
        let v78: f64 = (v71 * v77);
        let v79: f64 = (v76 + v78);
        let v80: f64 = (v73 * v79);
        let v82: f64 = (self.scalar_v81 - v80);
        let v83: f64 = (v82 / self.scalar_v55);
        let v84: f64 = (self.scalar_v81 - v83);
        let v85: f64 = (v84 / v83);
        let v87: f64 = 0.0004;
        let v90: f64 = (self.scalar_v89 - v85);
        let v91: f64 = (self.scalar_v86 * v90);
        let v92: f64 = (v12 + v91);
        let v93: f64 = (self.scalar_v53 / v92);
        let v94: f64 = (v56 * v83);
        let v95: f64 = (v80 + v94);
        let v96: f64 = (v95 - v83);
        let v97: f64 = (v96 / v83);
        let v98: f64 = (v11 - v54);
        let v99: f64 = (v87 * v98);
        let v100: f64 = (v99 - v97);
        let v101: f64 = (self.scalar_v86 * v100);
        let v102: f64 = (v12 + v101);
        let v103: f64 = (v93 * v102);
        let v105: f64 = ctx.node_voltage(nodes[3]);
        let v106: f64 = ctx.node_voltage(nodes[4]);
        let v107: f64 = (v105 - v106);
        let v108: f64 = (self.scalar_v104 * v107);
        let v109: f64 = ctx.node_voltage(nodes[0]);
        let v112: f64 = ctx.node_voltage(nodes[1]);
        let v115: bool = (v34 > v13);
        let v117: f64 = (v20 * self.scalar_v116);
        let v118: f64 = (v108 / v117);
        let v119: f64 = (if v115 { v118 } else { v13 });
        let v120: f64 = (-v108);
        let v121: f64 = (v120 - v47);
        let v123: f64 = (v20 * self.scalar_v122);
        let v124: f64 = (v121 / v123);
        let v125: f64 = (if v115 { v124 } else { v13 });
        let v126: f64 = (-v47);
        let v127: f64 = (v126 / v123);
        let v128: f64 = (if v115 { v127 } else { v13 });
        let v129: f64 = 80.0;
        let v130: bool = (v119 > v129);
        let v131: bool = (v115 && v130);
        let v132: f64 = (v119 - v129);
        let v133: f64 = (v12 + v132);
        let v134: f64 = (if v131 { v133 } else { v13 });
        let v135: f64 = (if v131 { v129 } else { v119 });
        let v136: bool = (!v130);
        let v137: bool = (v115 && v136);
        let v138: f64 = (if v137 { v12 } else { v134 });
        let v139: f64 = ((v135) as f64).exp();
        let v140: f64 = (v138 * v139);
        let v141: f64 = (if v115 { v140 } else { v138 });
        let v142: f64 = 37.0;
        let v143: bool = (v125 >= v142);
        let v144: bool = (!v143);
        let v145: f64 = -37.0;
        let v146: bool = (v125 <= v145);
        let v147: bool = (!v146);
        let v148: bool = (v144 && v147);
        let v149: f64 = ((v125) as f64).exp();
        let v150: f64 = (v12 + v149);
        let v151: f64 = ((v150) as f64).ln();
        let v152: bool = (v144 && v146);
        let v153: f64 = (if v143 { v125 } else { v13 });
        let v154: f64 = (if v152 { v149 } else { v153 });
        let v155: f64 = (if v148 { v151 } else { v154 });
        let v156: bool = (v128 >= v142);
        let v157: bool = (!v156);
        let v158: bool = (v128 <= v145);
        let v159: bool = (!v158);
        let v160: bool = (v157 && v159);
        let v161: f64 = ((v128) as f64).exp();
        let v162: f64 = (v12 + v161);
        let v163: f64 = ((v162) as f64).ln();
        let v164: bool = (v157 && v158);
        let v165: f64 = (if v156 { v128 } else { v13 });
        let v166: f64 = (if v164 { v161 } else { v165 });
        let v167: f64 = (if v160 { v163 } else { v166 });
        let v168: f64 = (v155 - v167);
        let v169: f64 = (if v115 { v168 } else { v13 });
        let v170: f64 = (v141 - v12);
        let v171: f64 = (v34 * v170);
        let v172: f64 = (v42 * v169);
        let v174: f64 = ((v108) as f64).abs();
        let v175: f64 = f64::powf(v174, v52);
        let v176: f64 = (self.scalar_v173 * v175);
        let v177: f64 = (v12 + v176);
        let v178: f64 = (v172 / v177);
        let v179: f64 = (v171 - v178);
        let v180: f64 = (if v115 { v179 } else { v13 });
        let v181: bool = (!v115);
        let v182: f64 = (if v181 { v13 } else { v180 });
        let v251: f64 = (v109 - v112);
        let v253: f64 = (v251 / self.scalar_v252);
        let v254: f64 = ((v253) as f64).abs();
        let v256: f64 = f64::powf(v254, self.scalar_v255);
        let v257: f64 = (v12 + v256);
        let v259: f64 = f64::powf(v257, self.scalar_v258);
        let v260: f64 = (v259 - v12);
        let v263: f64 = (v260 * self.scalar_v262);
        let v264: f64 = (v12 + v263);
        let v265: f64 = (self.scalar_v261 * v264);
        let v266: f64 = (v182 * v265);
        let v269: f64 = ctx.node_voltage(nodes[6]);
        let v280: f64 = (-v95);
        let v282: f64 = (v280 * self.scalar_v281);
        let v283: f64 = (v108 + v282);
        let v284: bool = (v283 > v13);
        let v290: f64 = (if v284 { self.scalar_v289 } else { v13 });
        let v291: f64 = (self.scalar_v286 * v290);
        let v292: f64 = (self.scalar_v286 * v291);
        let v293: f64 = (v12 - v292);
        let v294: f64 = (v95 * v293);
        let v296: f64 = (v294 / self.scalar_v295);
        let v297: f64 = (if v284 { v296 } else { v13 });
        let v300: f64 = (v283 * self.scalar_v299);
        let v301: f64 = (v300 / v95);
        let v302: f64 = (self.scalar_v286 + v301);
        let v303: f64 = (v283 * v302);
        let v304: f64 = (v290 * v303);
        let v305: f64 = (if v284 { v304 } else { v13 });
        let v306: bool = (!v284);
        let v307: f64 = (v108 / v95);
        let v308: f64 = (v12 - v307);
        let v309: f64 = ((v308) as f64).ln();
        let v310: f64 = (self.scalar_v295 * v309);
        let v311: f64 = ((v310) as f64).exp();
        let v312: f64 = (v12 - v311);
        let v313: f64 = (v95 * v312);
        let v314: f64 = (v313 / self.scalar_v295);
        let v315: f64 = (if v306 { v314 } else { v297 });
        let v316: f64 = (if v306 { v13 } else { v305 });
        let v317: f64 = (v315 + v316);
        let v318: f64 = (v103 * v317);
        let v350: f64 = 0.0;
        let v351: f64 = (v265 * v350);
        let v352: f64 = (if self.scalar_v268 { v351 } else { v13 });
        let v361: f64 = (v1 * self.scalar_v360);
        let v362: f64 = 0.0;
        let v363: f64 = (if self.scalar_v323 { v362 } else { v13 });
        let v368: f64 = ctx.node_voltage(nodes[5]);
        let v372: f64 = 0.0;
        let v373: f64 = (if self.scalar_v366 { v372 } else { v13 });
        let v377: f64 = (v368 * self.scalar_v376);
        let v378: f64 = 0.0;
        let v379: f64 = (if self.scalar_v366 { v378 } else { v13 });
        let v405: f64 = (self.scalar_v104 * v318);
        let v406: f64 = (self.scalar_v16 * v405);
        let v407: f64 = (self.scalar_v104 * v266);
        let v408: f64 = (self.scalar_v16 * v407);
        let v409: f64 = (if v8 { v12 } else { v13 });
        let v410: f64 = (if v10 { v13 } else { v409 });
        let v411: f64 = (v19 * v410);
        let v412: f64 = (v410 / self.scalar_v18);
        let v413: f64 = (v412 / v21);
        let v414: f64 = (self.scalar_v23 * v413);
        let v415: f64 = (self.scalar_v25 * v412);
        let v416: f64 = (v20 * v415);
        let v417: f64 = (v27 * v411);
        let v418: f64 = (v416 - v417);
        let v419: f64 = (v20 * v20);
        let v420: f64 = (v418 / v419);
        let v421: f64 = (v414 + v420);
        let v423: f64 = (v33 * v421);
        let v424: f64 = (self.scalar_v32 * v423);
        let v427: f64 = (self.scalar_v39 * v412);
        let v428: f64 = (self.scalar_v38 * v427);
        let v429: f64 = (self.scalar_v44 * v412);
        let v430: f64 = (self.scalar_v43 * v429);
        let v431: f64 = (self.scalar_v49 * v412);
        let v432: f64 = (self.scalar_v48 * v431);
        let v433: f64 = (v410 / v54);
        let v434: f64 = (v58 * v410);
        let v435: f64 = (v59 * v410);
        let v436: f64 = (v11 * v434);
        let v437: f64 = (v435 + v436);
        let v438: f64 = (v62 * v437);
        let v439: f64 = (v60 * v410);
        let v440: f64 = (v438 - v439);
        let v441: f64 = (v62 * v62);
        let v442: f64 = (v440 / v441);
        let v443: f64 = (v410 + v410);
        let v444: f64 = (v66 * v443);
        let v445: f64 = (v68 * v442);
        let v446: f64 = (v65 * v444);
        let v447: f64 = (v445 - v446);
        let v448: f64 = (v68 * v68);
        let v449: f64 = (v447 / v448);
        let v450: f64 = (v411 + v411);
        let v451: f64 = (-v450);
        let v452: f64 = (v433 / v56);
        let v453: f64 = (v74 * v452);
        let v454: f64 = (v77 * v449);
        let v455: f64 = (v453 + v454);
        let v456: f64 = (v79 * v451);
        let v457: f64 = (v73 * v455);
        let v458: f64 = (v456 + v457);
        let v459: f64 = (-v458);
        let v460: f64 = (v459 / self.scalar_v55);
        let v461: f64 = (-v460);
        let v462: f64 = (v83 * v461);
        let v463: f64 = (v84 * v460);
        let v464: f64 = (v462 - v463);
        let v465: f64 = (v83 * v83);
        let v466: f64 = (v464 / v465);
        let v467: f64 = (-v466);
        let v468: f64 = (self.scalar_v86 * v467);
        let v469: f64 = (self.scalar_v53 * v468);
        let v470: f64 = (-v469);
        let v471: f64 = (v92 * v92);
        let v472: f64 = (v470 / v471);
        let v473: f64 = (v83 * v433);
        let v474: f64 = (v56 * v460);
        let v475: f64 = (v473 + v474);
        let v476: f64 = (v458 + v475);
        let v477: f64 = (v476 - v460);
        let v478: f64 = (v83 * v477);
        let v479: f64 = (v96 * v460);
        let v480: f64 = (v478 - v479);
        let v481: f64 = (v480 / v465);
        let v482: f64 = (v87 * v410);
        let v483: f64 = (v482 - v481);
        let v484: f64 = (self.scalar_v86 * v483);
        let v485: f64 = (v102 * v472);
        let v486: f64 = (v93 * v484);
        let v487: f64 = (v485 + v486);
        let v489: f64 = (self.scalar_v116 * v411);
        let v490: f64 = (v108 * v489);
        let v491: f64 = (-v490);
        let v492: f64 = (v117 * v117);
        let v493: f64 = (v491 / v492);
        let v494: f64 = (self.scalar_v104 / v117);
        let v495: f64 = (self.scalar_v488 / v117);
        let v496: f64 = (if v115 { v493 } else { v13 });
        let v497: f64 = (if v115 { v494 } else { v13 });
        let v498: f64 = (if v115 { v495 } else { v13 });
        let v499: f64 = (-v430);
        let v500: f64 = (self.scalar_v122 * v411);
        let v501: f64 = (v123 * v499);
        let v502: f64 = (v121 * v500);
        let v503: f64 = (v501 - v502);
        let v504: f64 = (v123 * v123);
        let v505: f64 = (v503 / v504);
        let v506: f64 = (self.scalar_v488 / v123);
        let v507: f64 = (self.scalar_v104 / v123);
        let v508: f64 = (if v115 { v505 } else { v13 });
        let v509: f64 = (if v115 { v506 } else { v13 });
        let v510: f64 = (if v115 { v507 } else { v13 });
        let v511: f64 = (v126 * v500);
        let v512: f64 = (v501 - v511);
        let v513: f64 = (v512 / v504);
        let v514: f64 = (if v115 { v513 } else { v13 });
        let v515: f64 = (if v131 { v496 } else { v13 });
        let v516: f64 = (if v131 { v497 } else { v13 });
        let v517: f64 = (if v131 { v498 } else { v13 });
        let v518: f64 = (if v131 { v13 } else { v496 });
        let v519: f64 = (if v131 { v13 } else { v497 });
        let v520: f64 = (if v131 { v13 } else { v498 });
        let v521: f64 = (if v137 { v13 } else { v515 });
        let v522: f64 = (if v137 { v13 } else { v516 });
        let v523: f64 = (if v137 { v13 } else { v517 });
        let v524: f64 = (v139 * v518);
        let v525: f64 = (v139 * v519);
        let v526: f64 = (v139 * v520);
        let v527: f64 = (v139 * v521);
        let v528: f64 = (v138 * v524);
        let v529: f64 = (v527 + v528);
        let v530: f64 = (v139 * v522);
        let v531: f64 = (v138 * v525);
        let v532: f64 = (v530 + v531);
        let v533: f64 = (v139 * v523);
        let v534: f64 = (v138 * v526);
        let v535: f64 = (v533 + v534);
        let v536: f64 = (if v115 { v529 } else { v521 });
        let v537: f64 = (if v115 { v532 } else { v522 });
        let v538: f64 = (if v115 { v535 } else { v523 });
        let v539: f64 = (v149 * v508);
        let v540: f64 = (v149 * v509);
        let v541: f64 = (v149 * v510);
        let v542: f64 = (v539 / v150);
        let v543: f64 = (v540 / v150);
        let v544: f64 = (v541 / v150);
        let v545: f64 = (if v143 { v508 } else { v13 });
        let v546: f64 = (if v143 { v509 } else { v13 });
        let v547: f64 = (if v143 { v510 } else { v13 });
        let v548: f64 = (if v152 { v539 } else { v545 });
        let v549: f64 = (if v152 { v540 } else { v546 });
        let v550: f64 = (if v152 { v541 } else { v547 });
        let v551: f64 = (if v148 { v542 } else { v548 });
        let v552: f64 = (if v148 { v543 } else { v549 });
        let v553: f64 = (if v148 { v544 } else { v550 });
        let v554: f64 = (v161 * v514);
        let v555: f64 = (v554 / v162);
        let v556: f64 = (if v156 { v514 } else { v13 });
        let v557: f64 = (if v164 { v554 } else { v556 });
        let v558: f64 = (if v160 { v555 } else { v557 });
        let v559: f64 = (v551 - v558);
        let v560: f64 = (if v115 { v559 } else { v13 });
        let v561: f64 = (if v115 { v552 } else { v13 });
        let v562: f64 = (if v115 { v553 } else { v13 });
        let v563: f64 = (v170 * v424);
        let v564: f64 = (v34 * v536);
        let v565: f64 = (v563 + v564);
        let v566: f64 = (v34 * v537);
        let v567: f64 = (v34 * v538);
        let v568: f64 = (v169 * v428);
        let v569: f64 = (v42 * v560);
        let v570: f64 = (v568 + v569);
        let v571: f64 = (v42 * v561);
        let v572: f64 = (v42 * v562);
        let v573: f64 = ((v174) as f64).ln();
        let v574: f64 = (v175 * v573);
        let v575: f64 = (v432 * v574);
        let v576: f64 = (self.scalar_v173 * v575);
        let v577: f64 = (v177 * v570);
        let v578: f64 = (v172 * v576);
        let v579: f64 = (v577 - v578);
        let v580: f64 = (v177 * v177);
        let v581: f64 = (v579 / v580);
        let v582: f64 = (v571 / v177);
        let v583: f64 = (v572 / v177);
        let v584: f64 = (v565 - v581);
        let v585: f64 = (v566 - v582);
        let v586: f64 = (v567 - v583);
        let v587: f64 = (if v115 { v584 } else { v13 });
        let v588: f64 = (if v115 { v585 } else { v13 });
        let v589: f64 = (if v115 { v586 } else { v13 });
        let v590: f64 = (if v181 { v13 } else { v587 });
        let v591: f64 = (if v181 { v13 } else { v588 });
        let v592: f64 = (if v181 { v13 } else { v589 });
        let v666: f64 = (v265 * v590);
        let v667: f64 = (v265 * v591);
        let v668: f64 = (v265 * v592);
        let v672: f64 = (-v476);
        let v673: f64 = (self.scalar_v281 * v672);
        let v674: f64 = (v293 * v476);
        let v675: f64 = (v674 / self.scalar_v295);
        let v676: f64 = (if v284 { v675 } else { v13 });
        let v677: f64 = (self.scalar_v299 * v673);
        let v680: f64 = (v95 * v677);
        let v681: f64 = (v300 * v476);
        let v682: f64 = (v680 - v681);
        let v683: f64 = (v95 * v95);
        let v684: f64 = (v682 / v683);
        let v685: f64 = (self.scalar_v678 / v95);
        let v686: f64 = (self.scalar_v679 / v95);
        let v687: f64 = (v302 * v673);
        let v688: f64 = (v283 * v684);
        let v689: f64 = (v687 + v688);
        let v690: f64 = (self.scalar_v104 * v302);
        let v691: f64 = (v283 * v685);
        let v692: f64 = (v690 + v691);
        let v693: f64 = (v302 * self.scalar_v488);
        let v694: f64 = (v283 * v686);
        let v695: f64 = (v693 + v694);
        let v696: f64 = (v290 * v689);
        let v697: f64 = (v290 * v692);
        let v698: f64 = (v290 * v695);
        let v699: f64 = (if v284 { v696 } else { v13 });
        let v700: f64 = (if v284 { v697 } else { v13 });
        let v701: f64 = (if v284 { v698 } else { v13 });
        let v702: f64 = (v108 * v476);
        let v703: f64 = (-v702);
        let v704: f64 = (v703 / v683);
        let v705: f64 = (self.scalar_v104 / v95);
        let v706: f64 = (self.scalar_v488 / v95);
        let v707: f64 = (-v704);
        let v708: f64 = (-v705);
        let v709: f64 = (-v706);
        let v710: f64 = (v707 / v308);
        let v711: f64 = (v708 / v308);
        let v712: f64 = (v709 / v308);
        let v713: f64 = (self.scalar_v295 * v710);
        let v714: f64 = (self.scalar_v295 * v711);
        let v715: f64 = (self.scalar_v295 * v712);
        let v716: f64 = (v311 * v713);
        let v717: f64 = (v311 * v714);
        let v718: f64 = (v311 * v715);
        let v719: f64 = (-v716);
        let v720: f64 = (-v717);
        let v721: f64 = (-v718);
        let v722: f64 = (v312 * v476);
        let v723: f64 = (v95 * v719);
        let v724: f64 = (v722 + v723);
        let v725: f64 = (v95 * v720);
        let v726: f64 = (v95 * v721);
        let v727: f64 = (v724 / self.scalar_v295);
        let v728: f64 = (v725 / self.scalar_v295);
        let v729: f64 = (v726 / self.scalar_v295);
        let v730: f64 = (if v306 { v727 } else { v676 });
        let v731: f64 = (if v306 { v728 } else { v13 });
        let v732: f64 = (if v306 { v729 } else { v13 });
        let v733: f64 = (if v306 { v13 } else { v699 });
        let v734: f64 = (if v306 { v13 } else { v700 });
        let v735: f64 = (if v306 { v13 } else { v701 });
        let v736: f64 = (v730 + v733);
        let v737: f64 = (v731 + v734);
        let v738: f64 = (v732 + v735);
        let v739: f64 = (v317 * v487);
        let v740: f64 = (v103 * v736);
        let v741: f64 = (v739 + v740);
        let v742: f64 = (v103 * v737);
        let v743: f64 = (v103 * v738);
        let v756: f64 = 1.0;
        let v757: f64 = (v265 * v756);
        let v758: f64 = (if self.scalar_v268 { v757 } else { v13 });
        let v761: f64 = (self.scalar_v360 * v756);
        let v762: f64 = (if self.scalar_v323 { v761 } else { v13 });
        let v766: f64 = (if self.scalar_v366 { v761 } else { v13 });
        let v769: f64 = (self.scalar_v376 * v756);
        let v770: f64 = (if self.scalar_v366 { v769 } else { v13 });
        let v798: f64 = (self.scalar_v104 * v741);
        let v799: f64 = (self.scalar_v104 * v742);
        let v800: f64 = (self.scalar_v104 * v743);
        let v801: f64 = (self.scalar_v16 * v798);
        let v802: f64 = (self.scalar_v16 * v799);
        let v803: f64 = (self.scalar_v16 * v800);
        let v804: f64 = (self.scalar_v104 * v666);
        let v805: f64 = (self.scalar_v104 * v667);
        let v806: f64 = (self.scalar_v104 * v668);
        let v807: f64 = (self.scalar_v16 * v804);
        let v808: f64 = (self.scalar_v16 * v805);
        let v809: f64 = (self.scalar_v16 * v806);

        let d352_dn6: f64 = v758;
        stamper.stamp_current_reactive_node1(
            Some(nodes[6]),
            None,
            nodes[6],
            multiplicity * (d352_dn6),
        );
        let d363_dn2: f64 = v762;
        stamper.stamp_current_reactive_node1(
            Some(nodes[2]),
            None,
            nodes[2],
            multiplicity * (d363_dn2),
        );
        let d373_dn2: f64 = v766;
        stamper.stamp_current_reactive_node1(
            Some(nodes[2]),
            None,
            nodes[2],
            multiplicity * (d373_dn2),
        );
        let d379_dn5: f64 = v770;
        stamper.stamp_current_reactive_node1(
            Some(nodes[5]),
            None,
            nodes[5],
            multiplicity * (d379_dn5),
        );
        let d406_dn2: f64 = v801;
        let d406_dn3: f64 = v802;
        let d406_dn4: f64 = v803;
        stamper.stamp_current_reactive_node3(
            Some(nodes[3]),
            Some(nodes[4]),
            nodes[2],
            multiplicity * (d406_dn2),
            nodes[3],
            multiplicity * (d406_dn3),
            nodes[4],
            multiplicity * (d406_dn4),
        );
        let d408_dn2: f64 = v807;
        let d408_dn3: f64 = v808;
        let d408_dn4: f64 = v809;
        stamper.stamp_current_reactive_node3(
            Some(nodes[3]),
            Some(nodes[4]),
            nodes[2],
            multiplicity * (d408_dn2),
            nodes[3],
            multiplicity * (d408_dn3),
            nodes[4],
            multiplicity * (d408_dn4),
        );
    }
}
