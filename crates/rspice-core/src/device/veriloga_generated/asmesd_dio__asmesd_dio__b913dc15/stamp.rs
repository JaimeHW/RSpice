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
        let v278: f64 = (-v95);
        let v280: f64 = (v278 * self.scalar_v279);
        let v281: f64 = (v108 + v280);
        let v282: bool = (v281 > v13);
        let v288: f64 = (if v282 { self.scalar_v287 } else { v13 });
        let v289: f64 = (self.scalar_v284 * v288);
        let v290: f64 = (self.scalar_v284 * v289);
        let v291: f64 = (v12 - v290);
        let v292: f64 = (v95 * v291);
        let v294: f64 = (v292 / self.scalar_v293);
        let v295: f64 = (if v282 { v294 } else { v13 });
        let v298: f64 = (v281 * self.scalar_v297);
        let v299: f64 = (v298 / v95);
        let v300: f64 = (self.scalar_v284 + v299);
        let v301: f64 = (v281 * v300);
        let v302: f64 = (v288 * v301);
        let v303: f64 = (if v282 { v302 } else { v13 });
        let v304: bool = (!v282);
        let v305: f64 = (v108 / v95);
        let v306: f64 = (v12 - v305);
        let v307: f64 = ((v306) as f64).ln();
        let v308: f64 = (self.scalar_v293 * v307);
        let v309: f64 = ((v308) as f64).exp();
        let v310: f64 = (v12 - v309);
        let v311: f64 = (v95 * v310);
        let v312: f64 = (v311 / self.scalar_v293);
        let v313: f64 = (if v304 { v312 } else { v295 });
        let v314: f64 = (if v304 { v13 } else { v303 });
        let v315: f64 = (v313 + v314);
        let v316: f64 = (v103 * v315);
        let v339: f64 = (v277 / self.scalar_v16);
        let v343: f64 = (v250 / self.scalar_v16);
        let v344: f64 = (-v182);
        let v345: f64 = (v265 * v344);
        let v346: f64 = (if self.scalar_v268 { v345 } else { v13 });
        let v347: f64 = (if self.scalar_v268 { v269 } else { v13 });
        let v348: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, v269);
        let v349: f64 = (v265 * v348);
        let v350: f64 = (if self.scalar_v268 { v349 } else { v13 });
        let v351: f64 = (v214 * v251);
        let v352: f64 = ((v351) as f64).abs();
        let v353: f64 = (-v352);
        let v354: f64 = (if self.scalar_v321 { v353 } else { v13 });
        let v355: f64 = (v1 / self.scalar_v319);
        let v356: f64 = (if self.scalar_v321 { v355 } else { v13 });
        let v358: f64 = (v1 * self.scalar_v357);
        let v359: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, v358);
        let v360: f64 = (if self.scalar_v321 { v359 } else { v13 });
        let v363: f64 = (if self.scalar_v362 { v353 } else { v13 });
        let v364: f64 = ctx.node_voltage(nodes[5]);
        let v365: f64 = (v1 - v364);
        let v366: f64 = (v365 / self.scalar_v319);
        let v367: f64 = (if self.scalar_v362 { v366 } else { v13 });
        let v368: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, v358);
        let v369: f64 = (if self.scalar_v362 { v368 } else { v13 });
        let v370: f64 = (v364 / self.scalar_v325);
        let v371: f64 = (if self.scalar_v362 { v370 } else { v13 });
        let v373: f64 = (v364 * self.scalar_v372);
        let v374: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, v373);
        let v375: f64 = (if self.scalar_v362 { v374 } else { v13 });
        let v379: f64 = (if self.scalar_v378 { v353 } else { v13 });
        let v380: f64 = (v13 * v107);
        let v381: bool = (v339 > self.scalar_v336);
        let v382: f64 = (if v381 { v339 } else { self.scalar_v336 });
        let v383: f64 = (v110 / v382);
        let v384: f64 = (if self.scalar_v338 { v383 } else { v13 });
        let v385: bool = (v343 > self.scalar_v336);
        let v386: f64 = (if v385 { v343 } else { self.scalar_v336 });
        let v387: f64 = (v113 / v386);
        let v388: f64 = (if self.scalar_v342 { v387 } else { v13 });
        let v389: f64 = (self.scalar_v104 * v214);
        let v390: f64 = (self.scalar_v16 * v389);
        let v391: f64 = (self.scalar_v104 * v316);
        let v392: f64 = (self.scalar_v16 * v391);
        let v393: f64 = (self.scalar_v104 * v266);
        let v394: f64 = (self.scalar_v16 * v393);
        let v395: f64 = (if v8 { v12 } else { v13 });
        let v396: f64 = (if v10 { v13 } else { v395 });
        let v397: f64 = (v19 * v396);
        let v398: f64 = (v396 / self.scalar_v18);
        let v399: f64 = (v398 / v21);
        let v400: f64 = (self.scalar_v23 * v399);
        let v401: f64 = (self.scalar_v25 * v398);
        let v402: f64 = (v20 * v401);
        let v403: f64 = (v27 * v397);
        let v404: f64 = (v402 - v403);
        let v405: f64 = (v20 * v20);
        let v406: f64 = (v404 / v405);
        let v407: f64 = (v400 + v406);
        let v408: f64 = (self.scalar_v30 * v399);
        let v409: f64 = (v33 * v407);
        let v410: f64 = (self.scalar_v32 * v409);
        let v411: f64 = (v36 * v408);
        let v412: f64 = (self.scalar_v35 * v411);
        let v413: f64 = (self.scalar_v39 * v398);
        let v414: f64 = (self.scalar_v38 * v413);
        let v415: f64 = (self.scalar_v44 * v398);
        let v416: f64 = (self.scalar_v43 * v415);
        let v417: f64 = (self.scalar_v49 * v398);
        let v418: f64 = (self.scalar_v48 * v417);
        let v419: f64 = (v396 / v54);
        let v420: f64 = (v58 * v396);
        let v421: f64 = (v59 * v396);
        let v422: f64 = (v11 * v420);
        let v423: f64 = (v421 + v422);
        let v424: f64 = (v62 * v423);
        let v425: f64 = (v60 * v396);
        let v426: f64 = (v424 - v425);
        let v427: f64 = (v62 * v62);
        let v428: f64 = (v426 / v427);
        let v429: f64 = (v396 + v396);
        let v430: f64 = (v66 * v429);
        let v431: f64 = (v68 * v428);
        let v432: f64 = (v65 * v430);
        let v433: f64 = (v431 - v432);
        let v434: f64 = (v68 * v68);
        let v435: f64 = (v433 / v434);
        let v436: f64 = (v397 + v397);
        let v437: f64 = (-v436);
        let v438: f64 = (v419 / v56);
        let v439: f64 = (v74 * v438);
        let v440: f64 = (v77 * v435);
        let v441: f64 = (v439 + v440);
        let v442: f64 = (v79 * v437);
        let v443: f64 = (v73 * v441);
        let v444: f64 = (v442 + v443);
        let v445: f64 = (-v444);
        let v446: f64 = (v445 / self.scalar_v55);
        let v447: f64 = (-v446);
        let v448: f64 = (v83 * v447);
        let v449: f64 = (v84 * v446);
        let v450: f64 = (v448 - v449);
        let v451: f64 = (v83 * v83);
        let v452: f64 = (v450 / v451);
        let v453: f64 = (-v452);
        let v454: f64 = (self.scalar_v86 * v453);
        let v455: f64 = (self.scalar_v53 * v454);
        let v456: f64 = (-v455);
        let v457: f64 = (v92 * v92);
        let v458: f64 = (v456 / v457);
        let v459: f64 = (v83 * v419);
        let v460: f64 = (v56 * v446);
        let v461: f64 = (v459 + v460);
        let v462: f64 = (v444 + v461);
        let v463: f64 = (v462 - v446);
        let v464: f64 = (v83 * v463);
        let v465: f64 = (v96 * v446);
        let v466: f64 = (v464 - v465);
        let v467: f64 = (v466 / v451);
        let v468: f64 = (v87 * v396);
        let v469: f64 = (v468 - v467);
        let v470: f64 = (self.scalar_v86 * v469);
        let v471: f64 = (v102 * v458);
        let v472: f64 = (v93 * v470);
        let v473: f64 = (v471 + v472);
        let v475: f64 = (self.scalar_v116 * v397);
        let v476: f64 = (v108 * v475);
        let v477: f64 = (-v476);
        let v478: f64 = (v117 * v117);
        let v479: f64 = (v477 / v478);
        let v480: f64 = (self.scalar_v104 / v117);
        let v481: f64 = (self.scalar_v474 / v117);
        let v482: f64 = (if v115 { v479 } else { v13 });
        let v483: f64 = (if v115 { v480 } else { v13 });
        let v484: f64 = (if v115 { v481 } else { v13 });
        let v485: f64 = (-v416);
        let v486: f64 = (self.scalar_v122 * v397);
        let v487: f64 = (v123 * v485);
        let v488: f64 = (v121 * v486);
        let v489: f64 = (v487 - v488);
        let v490: f64 = (v123 * v123);
        let v491: f64 = (v489 / v490);
        let v492: f64 = (self.scalar_v474 / v123);
        let v493: f64 = (self.scalar_v104 / v123);
        let v494: f64 = (if v115 { v491 } else { v13 });
        let v495: f64 = (if v115 { v492 } else { v13 });
        let v496: f64 = (if v115 { v493 } else { v13 });
        let v497: f64 = (v126 * v486);
        let v498: f64 = (v487 - v497);
        let v499: f64 = (v498 / v490);
        let v500: f64 = (if v115 { v499 } else { v13 });
        let v501: f64 = (if v131 { v482 } else { v13 });
        let v502: f64 = (if v131 { v483 } else { v13 });
        let v503: f64 = (if v131 { v484 } else { v13 });
        let v504: f64 = (if v131 { v13 } else { v482 });
        let v505: f64 = (if v131 { v13 } else { v483 });
        let v506: f64 = (if v131 { v13 } else { v484 });
        let v507: f64 = (if v137 { v13 } else { v501 });
        let v508: f64 = (if v137 { v13 } else { v502 });
        let v509: f64 = (if v137 { v13 } else { v503 });
        let v510: f64 = (v139 * v504);
        let v511: f64 = (v139 * v505);
        let v512: f64 = (v139 * v506);
        let v513: f64 = (v139 * v507);
        let v514: f64 = (v138 * v510);
        let v515: f64 = (v513 + v514);
        let v516: f64 = (v139 * v508);
        let v517: f64 = (v138 * v511);
        let v518: f64 = (v516 + v517);
        let v519: f64 = (v139 * v509);
        let v520: f64 = (v138 * v512);
        let v521: f64 = (v519 + v520);
        let v522: f64 = (if v115 { v515 } else { v507 });
        let v523: f64 = (if v115 { v518 } else { v508 });
        let v524: f64 = (if v115 { v521 } else { v509 });
        let v525: f64 = (v149 * v494);
        let v526: f64 = (v149 * v495);
        let v527: f64 = (v149 * v496);
        let v528: f64 = (v525 / v150);
        let v529: f64 = (v526 / v150);
        let v530: f64 = (v527 / v150);
        let v531: f64 = (if v143 { v494 } else { v13 });
        let v532: f64 = (if v143 { v495 } else { v13 });
        let v533: f64 = (if v143 { v496 } else { v13 });
        let v534: f64 = (if v152 { v525 } else { v531 });
        let v535: f64 = (if v152 { v526 } else { v532 });
        let v536: f64 = (if v152 { v527 } else { v533 });
        let v537: f64 = (if v148 { v528 } else { v534 });
        let v538: f64 = (if v148 { v529 } else { v535 });
        let v539: f64 = (if v148 { v530 } else { v536 });
        let v540: f64 = (v161 * v500);
        let v541: f64 = (v540 / v162);
        let v542: f64 = (if v156 { v500 } else { v13 });
        let v543: f64 = (if v164 { v540 } else { v542 });
        let v544: f64 = (if v160 { v541 } else { v543 });
        let v545: f64 = (v537 - v544);
        let v546: f64 = (if v115 { v545 } else { v13 });
        let v547: f64 = (if v115 { v538 } else { v13 });
        let v548: f64 = (if v115 { v539 } else { v13 });
        let v549: f64 = (v170 * v410);
        let v550: f64 = (v34 * v522);
        let v551: f64 = (v549 + v550);
        let v552: f64 = (v34 * v523);
        let v553: f64 = (v34 * v524);
        let v554: f64 = (v169 * v414);
        let v555: f64 = (v42 * v546);
        let v556: f64 = (v554 + v555);
        let v557: f64 = (v42 * v547);
        let v558: f64 = (v42 * v548);
        let v559: f64 = ((v174) as f64).ln();
        let v560: f64 = (v175 * v559);
        let v561: f64 = (v418 * v560);
        let v562: f64 = (self.scalar_v173 * v561);
        let v563: f64 = (v177 * v556);
        let v564: f64 = (v172 * v562);
        let v565: f64 = (v563 - v564);
        let v566: f64 = (v177 * v177);
        let v567: f64 = (v565 / v566);
        let v568: f64 = (v557 / v177);
        let v569: f64 = (v558 / v177);
        let v570: f64 = (v551 - v567);
        let v571: f64 = (v552 - v568);
        let v572: f64 = (v553 - v569);
        let v573: f64 = (if v115 { v570 } else { v13 });
        let v574: f64 = (if v115 { v571 } else { v13 });
        let v575: f64 = (if v115 { v572 } else { v13 });
        let v576: f64 = (if v181 { v13 } else { v573 });
        let v577: f64 = (if v181 { v13 } else { v574 });
        let v578: f64 = (if v181 { v13 } else { v575 });
        let v579: f64 = (if v187 { self.scalar_v474 } else { v13 });
        let v580: f64 = (if v187 { self.scalar_v104 } else { v13 });
        let v581: f64 = (if v183 { v579 } else { v13 });
        let v582: f64 = (if v183 { v580 } else { v13 });
        let v585: f64 = (self.scalar_v192 * v397);
        let v586: f64 = (v189 * v585);
        let v587: f64 = (v193 * v581);
        let v588: f64 = (v193 * v582);
        let v589: f64 = (v191 * v586);
        let v590: f64 = (-v589);
        let v591: f64 = (v194 * v194);
        let v592: f64 = (v590 / v591);
        let v593: f64 = (v194 * self.scalar_v583);
        let v594: f64 = (v191 * v587);
        let v595: f64 = (v593 - v594);
        let v596: f64 = (v595 / v591);
        let v597: f64 = (v194 * self.scalar_v584);
        let v598: f64 = (v191 * v588);
        let v599: f64 = (v597 - v598);
        let v600: f64 = (v599 / v591);
        let v601: f64 = (if v183 { v592 } else { v504 });
        let v602: f64 = (if v183 { v596 } else { v505 });
        let v603: f64 = (if v183 { v600 } else { v506 });
        let v604: f64 = (if v198 { v601 } else { v522 });
        let v605: f64 = (if v198 { v602 } else { v523 });
        let v606: f64 = (if v198 { v603 } else { v524 });
        let v607: f64 = (if v198 { v13 } else { v601 });
        let v608: f64 = (if v198 { v13 } else { v602 });
        let v609: f64 = (if v198 { v13 } else { v603 });
        let v610: f64 = (if v204 { v13 } else { v604 });
        let v611: f64 = (if v204 { v13 } else { v605 });
        let v612: f64 = (if v204 { v13 } else { v606 });
        let v613: f64 = (v206 * v607);
        let v614: f64 = (v206 * v608);
        let v615: f64 = (v206 * v609);
        let v616: f64 = (v206 * v610);
        let v617: f64 = (v205 * v613);
        let v618: f64 = (v616 + v617);
        let v619: f64 = (v206 * v611);
        let v620: f64 = (v205 * v614);
        let v621: f64 = (v619 + v620);
        let v622: f64 = (v206 * v612);
        let v623: f64 = (v205 * v615);
        let v624: f64 = (v622 + v623);
        let v625: f64 = (if v183 { v618 } else { v610 });
        let v626: f64 = (if v183 { v621 } else { v611 });
        let v627: f64 = (if v183 { v624 } else { v612 });
        let v628: f64 = (v209 * v412);
        let v629: f64 = (v37 * v625);
        let v630: f64 = (v628 + v629);
        let v631: f64 = (v37 * v626);
        let v632: f64 = (v37 * v627);
        let v633: f64 = (if v183 { v630 } else { v13 });
        let v634: f64 = (if v183 { v631 } else { v13 });
        let v635: f64 = (if v183 { v632 } else { v13 });
        let v636: f64 = (if v212 { v13 } else { v633 });
        let v637: f64 = (if v212 { v13 } else { v634 });
        let v638: f64 = (if v212 { v13 } else { v635 });
        let v639: f64 = (v576 - v636);
        let v640: f64 = (v577 - v637);
        let v641: f64 = (v578 - v638);
        let v642: f64 = (self.scalar_v228 * v399);
        let v643: f64 = (v230 * v642);
        let v644: f64 = (self.scalar_v227 * v643);
        let v645: f64 = (v233 * v644);
        let v646: f64 = (self.scalar_v236 * v399);
        let v647: f64 = (v238 * v646);
        let v648: f64 = (self.scalar_v235 * v647);
        let v649: f64 = (v241 * v648);
        let v650: f64 = (v265 * v576);
        let v651: f64 = (v265 * v577);
        let v652: f64 = (v265 * v578);
        let v653: f64 = (v645 / v275);
        let v654: f64 = (if self.scalar_v268 { v653 } else { v645 });
        let v655: f64 = (-v462);
        let v656: f64 = (self.scalar_v279 * v655);
        let v657: f64 = (v291 * v462);
        let v658: f64 = (v657 / self.scalar_v293);
        let v659: f64 = (if v282 { v658 } else { v13 });
        let v660: f64 = (self.scalar_v297 * v656);
        let v663: f64 = (v95 * v660);
        let v664: f64 = (v298 * v462);
        let v665: f64 = (v663 - v664);
        let v666: f64 = (v95 * v95);
        let v667: f64 = (v665 / v666);
        let v668: f64 = (self.scalar_v661 / v95);
        let v669: f64 = (self.scalar_v662 / v95);
        let v670: f64 = (v300 * v656);
        let v671: f64 = (v281 * v667);
        let v672: f64 = (v670 + v671);
        let v673: f64 = (self.scalar_v104 * v300);
        let v674: f64 = (v281 * v668);
        let v675: f64 = (v673 + v674);
        let v676: f64 = (v300 * self.scalar_v474);
        let v677: f64 = (v281 * v669);
        let v678: f64 = (v676 + v677);
        let v679: f64 = (v288 * v672);
        let v680: f64 = (v288 * v675);
        let v681: f64 = (v288 * v678);
        let v682: f64 = (if v282 { v679 } else { v13 });
        let v683: f64 = (if v282 { v680 } else { v13 });
        let v684: f64 = (if v282 { v681 } else { v13 });
        let v685: f64 = (v108 * v462);
        let v686: f64 = (-v685);
        let v687: f64 = (v686 / v666);
        let v688: f64 = (self.scalar_v104 / v95);
        let v689: f64 = (self.scalar_v474 / v95);
        let v690: f64 = (-v687);
        let v691: f64 = (-v688);
        let v692: f64 = (-v689);
        let v693: f64 = (v690 / v306);
        let v694: f64 = (v691 / v306);
        let v695: f64 = (v692 / v306);
        let v696: f64 = (self.scalar_v293 * v693);
        let v697: f64 = (self.scalar_v293 * v694);
        let v698: f64 = (self.scalar_v293 * v695);
        let v699: f64 = (v309 * v696);
        let v700: f64 = (v309 * v697);
        let v701: f64 = (v309 * v698);
        let v702: f64 = (-v699);
        let v703: f64 = (-v700);
        let v704: f64 = (-v701);
        let v705: f64 = (v310 * v462);
        let v706: f64 = (v95 * v702);
        let v707: f64 = (v705 + v706);
        let v708: f64 = (v95 * v703);
        let v709: f64 = (v95 * v704);
        let v710: f64 = (v707 / self.scalar_v293);
        let v711: f64 = (v708 / self.scalar_v293);
        let v712: f64 = (v709 / self.scalar_v293);
        let v713: f64 = (if v304 { v710 } else { v659 });
        let v714: f64 = (if v304 { v711 } else { v13 });
        let v715: f64 = (if v304 { v712 } else { v13 });
        let v716: f64 = (if v304 { v13 } else { v682 });
        let v717: f64 = (if v304 { v13 } else { v683 });
        let v718: f64 = (if v304 { v13 } else { v684 });
        let v719: f64 = (v713 + v716);
        let v720: f64 = (v714 + v717);
        let v721: f64 = (v715 + v718);
        let v722: f64 = (v315 * v473);
        let v723: f64 = (v103 * v719);
        let v724: f64 = (v722 + v723);
        let v725: f64 = (v103 * v720);
        let v726: f64 = (v103 * v721);
        let v727: f64 = (v654 / self.scalar_v16);
        let v728: f64 = (v649 / self.scalar_v16);
        let v729: f64 = (-v576);
        let v730: f64 = (-v577);
        let v731: f64 = (-v578);
        let v732: f64 = (v265 * v729);
        let v733: f64 = (v265 * v730);
        let v734: f64 = (v265 * v731);
        let v735: f64 = (if self.scalar_v268 { v732 } else { v13 });
        let v736: f64 = (if self.scalar_v268 { v733 } else { v13 });
        let v737: f64 = (if self.scalar_v268 { v734 } else { v13 });
        let v739: f64 = ddt_scale;
        let v740: f64 = (v265 * v739);
        let v741: f64 = (if self.scalar_v268 { v740 } else { v13 });
        let v744: f64 = (self.scalar_v357 * v739);
        let v745: f64 = (if self.scalar_v321 { v744 } else { v13 });
        let v749: f64 = (if self.scalar_v362 { v744 } else { v13 });
        let v752: f64 = (self.scalar_v372 * v739);
        let v753: f64 = (if self.scalar_v362 { v752 } else { v13 });
        let v754: f64 = -0.0;
        let v755: f64 = (if v381 { v727 } else { v13 });
        let v756: f64 = (v12 / v382);
        let v757: f64 = (v110 * v755);
        let v758: f64 = (-v757);
        let v759: f64 = (v382 * v382);
        let v760: f64 = (v758 / v759);
        let v761: f64 = (v190 / v382);
        let v762: f64 = (if self.scalar_v338 { v756 } else { v13 });
        let v763: f64 = (if self.scalar_v338 { v760 } else { v13 });
        let v764: f64 = (if self.scalar_v338 { v761 } else { v13 });
        let v765: f64 = (if v385 { v728 } else { v13 });
        let v766: f64 = (v12 / v386);
        let v767: f64 = (v113 * v765);
        let v768: f64 = (-v767);
        let v769: f64 = (v386 * v386);
        let v770: f64 = (v768 / v769);
        let v771: f64 = (v190 / v386);
        let v772: f64 = (if self.scalar_v342 { v766 } else { v13 });
        let v773: f64 = (if self.scalar_v342 { v770 } else { v13 });
        let v774: f64 = (if self.scalar_v342 { v771 } else { v13 });
        let v775: f64 = (self.scalar_v104 * v639);
        let v776: f64 = (self.scalar_v104 * v640);
        let v777: f64 = (self.scalar_v104 * v641);
        let v778: f64 = (self.scalar_v16 * v775);
        let v779: f64 = (self.scalar_v16 * v776);
        let v780: f64 = (self.scalar_v16 * v777);
        let v781: f64 = (self.scalar_v104 * v724);
        let v782: f64 = (self.scalar_v104 * v725);
        let v783: f64 = (self.scalar_v104 * v726);
        let v784: f64 = (self.scalar_v16 * v781);
        let v785: f64 = (self.scalar_v16 * v782);
        let v786: f64 = (self.scalar_v16 * v783);
        let v787: f64 = (self.scalar_v104 * v650);
        let v788: f64 = (self.scalar_v104 * v651);
        let v789: f64 = (self.scalar_v104 * v652);
        let v790: f64 = (self.scalar_v16 * v787);
        let v791: f64 = (self.scalar_v16 * v788);
        let v792: f64 = (self.scalar_v16 * v789);

        let d346_dn2: f64 = v735;
        let d346_dn3: f64 = v736;
        let d346_dn4: f64 = v737;
        stamper.stamp_current_node3_local(
            Some(6),
            None,
            multiplicity * (v346),
            2,
            multiplicity * (d346_dn2),
            3,
            multiplicity * (d346_dn3),
            4,
            multiplicity * (d346_dn4),
        );
        let d347_dn6: f64 = self.scalar_v738;
        stamper.stamp_current_node1_local(
            Some(6),
            None,
            multiplicity * (v347),
            6,
            multiplicity * (d347_dn6),
        );
        let d350_dn6: f64 = v741;
        stamper.stamp_current_node1_local(
            Some(6),
            None,
            multiplicity * (v350),
            6,
            multiplicity * (d350_dn6),
        );
        stamper.stamp_potential_branch_local(
            Some(6),
            None,
            0,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            0,
            v13,
        );
        stamper.stamp_current_const_local(
            Some(2),
            None,
            multiplicity * (v354),
        );
        let d356_dn2: f64 = self.scalar_v743;
        stamper.stamp_current_node1_local(
            Some(2),
            None,
            multiplicity * (v356),
            2,
            multiplicity * (d356_dn2),
        );
        let d360_dn2: f64 = v745;
        stamper.stamp_current_node1_local(
            Some(2),
            None,
            multiplicity * (v360),
            2,
            multiplicity * (d360_dn2),
        );
        stamper.stamp_potential_branch_local(
            Some(5),
            None,
            1,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            1,
            v13,
        );
        stamper.stamp_current_const_local(
            Some(2),
            None,
            multiplicity * (v363),
        );
        let d367_dn2: f64 = self.scalar_v747;
        let d367_dn5: f64 = self.scalar_v748;
        stamper.stamp_current_node2_local(
            Some(2),
            Some(5),
            multiplicity * (v367),
            2,
            multiplicity * (d367_dn2),
            5,
            multiplicity * (d367_dn5),
        );
        let d369_dn2: f64 = v749;
        stamper.stamp_current_node1_local(
            Some(2),
            None,
            multiplicity * (v369),
            2,
            multiplicity * (d369_dn2),
        );
        let d371_dn5: f64 = self.scalar_v751;
        stamper.stamp_current_node1_local(
            Some(5),
            None,
            multiplicity * (v371),
            5,
            multiplicity * (d371_dn5),
        );
        let d375_dn5: f64 = v753;
        stamper.stamp_current_node1_local(
            Some(5),
            None,
            multiplicity * (v375),
            5,
            multiplicity * (d375_dn5),
        );
        stamper.stamp_current_const_local(
            Some(2),
            None,
            multiplicity * (v379),
        );
        stamper.stamp_potential_branch_local(
            Some(5),
            None,
            2,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            2,
            v13,
        );
        stamper.stamp_potential_branch_local(
            Some(2),
            None,
            3,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            3,
            v13,
        );
        stamper.stamp_potential_branch_local(
            Some(5),
            None,
            4,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            4,
            v13,
        );
        let d380_dn3: f64 = v13;
        let d380_dn4: f64 = v754;
        stamper.stamp_current_node2_local(
            Some(3),
            Some(4),
            multiplicity * (v380),
            3,
            multiplicity * (d380_dn3),
            4,
            multiplicity * (d380_dn4),
        );
        let d384_dn0: f64 = v762;
        let d384_dn2: f64 = v763;
        let d384_dn3: f64 = v764;
        stamper.stamp_current_node3_local(
            Some(0),
            Some(3),
            multiplicity * (v384),
            0,
            multiplicity * (d384_dn0),
            2,
            multiplicity * (d384_dn2),
            3,
            multiplicity * (d384_dn3),
        );
        stamper.stamp_current_const_local(
            Some(0),
            Some(3),
            multiplicity * (v13),
        );
        stamper.stamp_potential_branch_local(
            Some(0),
            Some(3),
            5,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            5,
            v13,
        );
        let d388_dn1: f64 = v772;
        let d388_dn2: f64 = v773;
        let d388_dn4: f64 = v774;
        stamper.stamp_current_node3_local(
            Some(1),
            Some(4),
            multiplicity * (v388),
            1,
            multiplicity * (d388_dn1),
            2,
            multiplicity * (d388_dn2),
            4,
            multiplicity * (d388_dn4),
        );
        stamper.stamp_current_const_local(
            Some(1),
            Some(4),
            multiplicity * (v13),
        );
        stamper.stamp_potential_branch_local(
            Some(1),
            Some(4),
            6,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            6,
            v13,
        );
        let d390_dn2: f64 = v778;
        let d390_dn3: f64 = v779;
        let d390_dn4: f64 = v780;
        stamper.stamp_current_node3_local(
            Some(3),
            Some(4),
            multiplicity * (v390),
            2,
            multiplicity * (d390_dn2),
            3,
            multiplicity * (d390_dn3),
            4,
            multiplicity * (d390_dn4),
        );
        let d392_dn2: f64 = v784;
        let d392_dn3: f64 = v785;
        let d392_dn4: f64 = v786;
        let v392_ddt: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, v392);
        stamper.stamp_current_node3_local(
            Some(3),
            Some(4),
            multiplicity * (v392_ddt),
            2,
            multiplicity * (((d392_dn2) * ddt_scale)),
            3,
            multiplicity * (((d392_dn3) * ddt_scale)),
            4,
            multiplicity * (((d392_dn4) * ddt_scale)),
        );
        let d394_dn2: f64 = v790;
        let d394_dn3: f64 = v791;
        let d394_dn4: f64 = v792;
        let v394_ddt: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 5, v394);
        stamper.stamp_current_node3_local(
            Some(3),
            Some(4),
            multiplicity * (v394_ddt),
            2,
            multiplicity * (((d394_dn2) * ddt_scale)),
            3,
            multiplicity * (((d394_dn3) * ddt_scale)),
            4,
            multiplicity * (((d394_dn4) * ddt_scale)),
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
        let v278: f64 = (-v95);
        let v280: f64 = (v278 * self.scalar_v279);
        let v281: f64 = (v108 + v280);
        let v282: bool = (v281 > v13);
        let v288: f64 = (if v282 { self.scalar_v287 } else { v13 });
        let v289: f64 = (self.scalar_v284 * v288);
        let v290: f64 = (self.scalar_v284 * v289);
        let v291: f64 = (v12 - v290);
        let v292: f64 = (v95 * v291);
        let v294: f64 = (v292 / self.scalar_v293);
        let v295: f64 = (if v282 { v294 } else { v13 });
        let v298: f64 = (v281 * self.scalar_v297);
        let v299: f64 = (v298 / v95);
        let v300: f64 = (self.scalar_v284 + v299);
        let v301: f64 = (v281 * v300);
        let v302: f64 = (v288 * v301);
        let v303: f64 = (if v282 { v302 } else { v13 });
        let v304: bool = (!v282);
        let v305: f64 = (v108 / v95);
        let v306: f64 = (v12 - v305);
        let v307: f64 = ((v306) as f64).ln();
        let v308: f64 = (self.scalar_v293 * v307);
        let v309: f64 = ((v308) as f64).exp();
        let v310: f64 = (v12 - v309);
        let v311: f64 = (v95 * v310);
        let v312: f64 = (v311 / self.scalar_v293);
        let v313: f64 = (if v304 { v312 } else { v295 });
        let v314: f64 = (if v304 { v13 } else { v303 });
        let v315: f64 = (v313 + v314);
        let v316: f64 = (v103 * v315);
        let v348: f64 = 0.0;
        let v349: f64 = (v265 * v348);
        let v350: f64 = (if self.scalar_v268 { v349 } else { v13 });
        let v358: f64 = (v1 * self.scalar_v357);
        let v359: f64 = 0.0;
        let v360: f64 = (if self.scalar_v321 { v359 } else { v13 });
        let v364: f64 = ctx.node_voltage(nodes[5]);
        let v368: f64 = 0.0;
        let v369: f64 = (if self.scalar_v362 { v368 } else { v13 });
        let v373: f64 = (v364 * self.scalar_v372);
        let v374: f64 = 0.0;
        let v375: f64 = (if self.scalar_v362 { v374 } else { v13 });
        let v391: f64 = (self.scalar_v104 * v316);
        let v392: f64 = (self.scalar_v16 * v391);
        let v393: f64 = (self.scalar_v104 * v266);
        let v394: f64 = (self.scalar_v16 * v393);
        let v395: f64 = (if v8 { v12 } else { v13 });
        let v396: f64 = (if v10 { v13 } else { v395 });
        let v397: f64 = (v19 * v396);
        let v398: f64 = (v396 / self.scalar_v18);
        let v399: f64 = (v398 / v21);
        let v400: f64 = (self.scalar_v23 * v399);
        let v401: f64 = (self.scalar_v25 * v398);
        let v402: f64 = (v20 * v401);
        let v403: f64 = (v27 * v397);
        let v404: f64 = (v402 - v403);
        let v405: f64 = (v20 * v20);
        let v406: f64 = (v404 / v405);
        let v407: f64 = (v400 + v406);
        let v409: f64 = (v33 * v407);
        let v410: f64 = (self.scalar_v32 * v409);
        let v413: f64 = (self.scalar_v39 * v398);
        let v414: f64 = (self.scalar_v38 * v413);
        let v415: f64 = (self.scalar_v44 * v398);
        let v416: f64 = (self.scalar_v43 * v415);
        let v417: f64 = (self.scalar_v49 * v398);
        let v418: f64 = (self.scalar_v48 * v417);
        let v419: f64 = (v396 / v54);
        let v420: f64 = (v58 * v396);
        let v421: f64 = (v59 * v396);
        let v422: f64 = (v11 * v420);
        let v423: f64 = (v421 + v422);
        let v424: f64 = (v62 * v423);
        let v425: f64 = (v60 * v396);
        let v426: f64 = (v424 - v425);
        let v427: f64 = (v62 * v62);
        let v428: f64 = (v426 / v427);
        let v429: f64 = (v396 + v396);
        let v430: f64 = (v66 * v429);
        let v431: f64 = (v68 * v428);
        let v432: f64 = (v65 * v430);
        let v433: f64 = (v431 - v432);
        let v434: f64 = (v68 * v68);
        let v435: f64 = (v433 / v434);
        let v436: f64 = (v397 + v397);
        let v437: f64 = (-v436);
        let v438: f64 = (v419 / v56);
        let v439: f64 = (v74 * v438);
        let v440: f64 = (v77 * v435);
        let v441: f64 = (v439 + v440);
        let v442: f64 = (v79 * v437);
        let v443: f64 = (v73 * v441);
        let v444: f64 = (v442 + v443);
        let v445: f64 = (-v444);
        let v446: f64 = (v445 / self.scalar_v55);
        let v447: f64 = (-v446);
        let v448: f64 = (v83 * v447);
        let v449: f64 = (v84 * v446);
        let v450: f64 = (v448 - v449);
        let v451: f64 = (v83 * v83);
        let v452: f64 = (v450 / v451);
        let v453: f64 = (-v452);
        let v454: f64 = (self.scalar_v86 * v453);
        let v455: f64 = (self.scalar_v53 * v454);
        let v456: f64 = (-v455);
        let v457: f64 = (v92 * v92);
        let v458: f64 = (v456 / v457);
        let v459: f64 = (v83 * v419);
        let v460: f64 = (v56 * v446);
        let v461: f64 = (v459 + v460);
        let v462: f64 = (v444 + v461);
        let v463: f64 = (v462 - v446);
        let v464: f64 = (v83 * v463);
        let v465: f64 = (v96 * v446);
        let v466: f64 = (v464 - v465);
        let v467: f64 = (v466 / v451);
        let v468: f64 = (v87 * v396);
        let v469: f64 = (v468 - v467);
        let v470: f64 = (self.scalar_v86 * v469);
        let v471: f64 = (v102 * v458);
        let v472: f64 = (v93 * v470);
        let v473: f64 = (v471 + v472);
        let v475: f64 = (self.scalar_v116 * v397);
        let v476: f64 = (v108 * v475);
        let v477: f64 = (-v476);
        let v478: f64 = (v117 * v117);
        let v479: f64 = (v477 / v478);
        let v480: f64 = (self.scalar_v104 / v117);
        let v481: f64 = (self.scalar_v474 / v117);
        let v482: f64 = (if v115 { v479 } else { v13 });
        let v483: f64 = (if v115 { v480 } else { v13 });
        let v484: f64 = (if v115 { v481 } else { v13 });
        let v485: f64 = (-v416);
        let v486: f64 = (self.scalar_v122 * v397);
        let v487: f64 = (v123 * v485);
        let v488: f64 = (v121 * v486);
        let v489: f64 = (v487 - v488);
        let v490: f64 = (v123 * v123);
        let v491: f64 = (v489 / v490);
        let v492: f64 = (self.scalar_v474 / v123);
        let v493: f64 = (self.scalar_v104 / v123);
        let v494: f64 = (if v115 { v491 } else { v13 });
        let v495: f64 = (if v115 { v492 } else { v13 });
        let v496: f64 = (if v115 { v493 } else { v13 });
        let v497: f64 = (v126 * v486);
        let v498: f64 = (v487 - v497);
        let v499: f64 = (v498 / v490);
        let v500: f64 = (if v115 { v499 } else { v13 });
        let v501: f64 = (if v131 { v482 } else { v13 });
        let v502: f64 = (if v131 { v483 } else { v13 });
        let v503: f64 = (if v131 { v484 } else { v13 });
        let v504: f64 = (if v131 { v13 } else { v482 });
        let v505: f64 = (if v131 { v13 } else { v483 });
        let v506: f64 = (if v131 { v13 } else { v484 });
        let v507: f64 = (if v137 { v13 } else { v501 });
        let v508: f64 = (if v137 { v13 } else { v502 });
        let v509: f64 = (if v137 { v13 } else { v503 });
        let v510: f64 = (v139 * v504);
        let v511: f64 = (v139 * v505);
        let v512: f64 = (v139 * v506);
        let v513: f64 = (v139 * v507);
        let v514: f64 = (v138 * v510);
        let v515: f64 = (v513 + v514);
        let v516: f64 = (v139 * v508);
        let v517: f64 = (v138 * v511);
        let v518: f64 = (v516 + v517);
        let v519: f64 = (v139 * v509);
        let v520: f64 = (v138 * v512);
        let v521: f64 = (v519 + v520);
        let v522: f64 = (if v115 { v515 } else { v507 });
        let v523: f64 = (if v115 { v518 } else { v508 });
        let v524: f64 = (if v115 { v521 } else { v509 });
        let v525: f64 = (v149 * v494);
        let v526: f64 = (v149 * v495);
        let v527: f64 = (v149 * v496);
        let v528: f64 = (v525 / v150);
        let v529: f64 = (v526 / v150);
        let v530: f64 = (v527 / v150);
        let v531: f64 = (if v143 { v494 } else { v13 });
        let v532: f64 = (if v143 { v495 } else { v13 });
        let v533: f64 = (if v143 { v496 } else { v13 });
        let v534: f64 = (if v152 { v525 } else { v531 });
        let v535: f64 = (if v152 { v526 } else { v532 });
        let v536: f64 = (if v152 { v527 } else { v533 });
        let v537: f64 = (if v148 { v528 } else { v534 });
        let v538: f64 = (if v148 { v529 } else { v535 });
        let v539: f64 = (if v148 { v530 } else { v536 });
        let v540: f64 = (v161 * v500);
        let v541: f64 = (v540 / v162);
        let v542: f64 = (if v156 { v500 } else { v13 });
        let v543: f64 = (if v164 { v540 } else { v542 });
        let v544: f64 = (if v160 { v541 } else { v543 });
        let v545: f64 = (v537 - v544);
        let v546: f64 = (if v115 { v545 } else { v13 });
        let v547: f64 = (if v115 { v538 } else { v13 });
        let v548: f64 = (if v115 { v539 } else { v13 });
        let v549: f64 = (v170 * v410);
        let v550: f64 = (v34 * v522);
        let v551: f64 = (v549 + v550);
        let v552: f64 = (v34 * v523);
        let v553: f64 = (v34 * v524);
        let v554: f64 = (v169 * v414);
        let v555: f64 = (v42 * v546);
        let v556: f64 = (v554 + v555);
        let v557: f64 = (v42 * v547);
        let v558: f64 = (v42 * v548);
        let v559: f64 = ((v174) as f64).ln();
        let v560: f64 = (v175 * v559);
        let v561: f64 = (v418 * v560);
        let v562: f64 = (self.scalar_v173 * v561);
        let v563: f64 = (v177 * v556);
        let v564: f64 = (v172 * v562);
        let v565: f64 = (v563 - v564);
        let v566: f64 = (v177 * v177);
        let v567: f64 = (v565 / v566);
        let v568: f64 = (v557 / v177);
        let v569: f64 = (v558 / v177);
        let v570: f64 = (v551 - v567);
        let v571: f64 = (v552 - v568);
        let v572: f64 = (v553 - v569);
        let v573: f64 = (if v115 { v570 } else { v13 });
        let v574: f64 = (if v115 { v571 } else { v13 });
        let v575: f64 = (if v115 { v572 } else { v13 });
        let v576: f64 = (if v181 { v13 } else { v573 });
        let v577: f64 = (if v181 { v13 } else { v574 });
        let v578: f64 = (if v181 { v13 } else { v575 });
        let v650: f64 = (v265 * v576);
        let v651: f64 = (v265 * v577);
        let v652: f64 = (v265 * v578);
        let v655: f64 = (-v462);
        let v656: f64 = (self.scalar_v279 * v655);
        let v657: f64 = (v291 * v462);
        let v658: f64 = (v657 / self.scalar_v293);
        let v659: f64 = (if v282 { v658 } else { v13 });
        let v660: f64 = (self.scalar_v297 * v656);
        let v663: f64 = (v95 * v660);
        let v664: f64 = (v298 * v462);
        let v665: f64 = (v663 - v664);
        let v666: f64 = (v95 * v95);
        let v667: f64 = (v665 / v666);
        let v668: f64 = (self.scalar_v661 / v95);
        let v669: f64 = (self.scalar_v662 / v95);
        let v670: f64 = (v300 * v656);
        let v671: f64 = (v281 * v667);
        let v672: f64 = (v670 + v671);
        let v673: f64 = (self.scalar_v104 * v300);
        let v674: f64 = (v281 * v668);
        let v675: f64 = (v673 + v674);
        let v676: f64 = (v300 * self.scalar_v474);
        let v677: f64 = (v281 * v669);
        let v678: f64 = (v676 + v677);
        let v679: f64 = (v288 * v672);
        let v680: f64 = (v288 * v675);
        let v681: f64 = (v288 * v678);
        let v682: f64 = (if v282 { v679 } else { v13 });
        let v683: f64 = (if v282 { v680 } else { v13 });
        let v684: f64 = (if v282 { v681 } else { v13 });
        let v685: f64 = (v108 * v462);
        let v686: f64 = (-v685);
        let v687: f64 = (v686 / v666);
        let v688: f64 = (self.scalar_v104 / v95);
        let v689: f64 = (self.scalar_v474 / v95);
        let v690: f64 = (-v687);
        let v691: f64 = (-v688);
        let v692: f64 = (-v689);
        let v693: f64 = (v690 / v306);
        let v694: f64 = (v691 / v306);
        let v695: f64 = (v692 / v306);
        let v696: f64 = (self.scalar_v293 * v693);
        let v697: f64 = (self.scalar_v293 * v694);
        let v698: f64 = (self.scalar_v293 * v695);
        let v699: f64 = (v309 * v696);
        let v700: f64 = (v309 * v697);
        let v701: f64 = (v309 * v698);
        let v702: f64 = (-v699);
        let v703: f64 = (-v700);
        let v704: f64 = (-v701);
        let v705: f64 = (v310 * v462);
        let v706: f64 = (v95 * v702);
        let v707: f64 = (v705 + v706);
        let v708: f64 = (v95 * v703);
        let v709: f64 = (v95 * v704);
        let v710: f64 = (v707 / self.scalar_v293);
        let v711: f64 = (v708 / self.scalar_v293);
        let v712: f64 = (v709 / self.scalar_v293);
        let v713: f64 = (if v304 { v710 } else { v659 });
        let v714: f64 = (if v304 { v711 } else { v13 });
        let v715: f64 = (if v304 { v712 } else { v13 });
        let v716: f64 = (if v304 { v13 } else { v682 });
        let v717: f64 = (if v304 { v13 } else { v683 });
        let v718: f64 = (if v304 { v13 } else { v684 });
        let v719: f64 = (v713 + v716);
        let v720: f64 = (v714 + v717);
        let v721: f64 = (v715 + v718);
        let v722: f64 = (v315 * v473);
        let v723: f64 = (v103 * v719);
        let v724: f64 = (v722 + v723);
        let v725: f64 = (v103 * v720);
        let v726: f64 = (v103 * v721);
        let v739: f64 = 1.0;
        let v740: f64 = (v265 * v739);
        let v741: f64 = (if self.scalar_v268 { v740 } else { v13 });
        let v744: f64 = (self.scalar_v357 * v739);
        let v745: f64 = (if self.scalar_v321 { v744 } else { v13 });
        let v749: f64 = (if self.scalar_v362 { v744 } else { v13 });
        let v752: f64 = (self.scalar_v372 * v739);
        let v753: f64 = (if self.scalar_v362 { v752 } else { v13 });
        let v781: f64 = (self.scalar_v104 * v724);
        let v782: f64 = (self.scalar_v104 * v725);
        let v783: f64 = (self.scalar_v104 * v726);
        let v784: f64 = (self.scalar_v16 * v781);
        let v785: f64 = (self.scalar_v16 * v782);
        let v786: f64 = (self.scalar_v16 * v783);
        let v787: f64 = (self.scalar_v104 * v650);
        let v788: f64 = (self.scalar_v104 * v651);
        let v789: f64 = (self.scalar_v104 * v652);
        let v790: f64 = (self.scalar_v16 * v787);
        let v791: f64 = (self.scalar_v16 * v788);
        let v792: f64 = (self.scalar_v16 * v789);

        let d350_dn6: f64 = v741;
        stamper.stamp_current_reactive_node1(
            Some(nodes[6]),
            None,
            nodes[6],
            multiplicity * (d350_dn6),
        );
        let d360_dn2: f64 = v745;
        stamper.stamp_current_reactive_node1(
            Some(nodes[2]),
            None,
            nodes[2],
            multiplicity * (d360_dn2),
        );
        let d369_dn2: f64 = v749;
        stamper.stamp_current_reactive_node1(
            Some(nodes[2]),
            None,
            nodes[2],
            multiplicity * (d369_dn2),
        );
        let d375_dn5: f64 = v753;
        stamper.stamp_current_reactive_node1(
            Some(nodes[5]),
            None,
            nodes[5],
            multiplicity * (d375_dn5),
        );
        let d392_dn2: f64 = v784;
        let d392_dn3: f64 = v785;
        let d392_dn4: f64 = v786;
        stamper.stamp_current_reactive_node3(
            Some(nodes[3]),
            Some(nodes[4]),
            nodes[2],
            multiplicity * (d392_dn2),
            nodes[3],
            multiplicity * (d392_dn3),
            nodes[4],
            multiplicity * (d392_dn4),
        );
        let d394_dn2: f64 = v790;
        let d394_dn3: f64 = v791;
        let d394_dn4: f64 = v792;
        stamper.stamp_current_reactive_node3(
            Some(nodes[3]),
            Some(nodes[4]),
            nodes[2],
            multiplicity * (d394_dn2),
            nodes[3],
            multiplicity * (d394_dn3),
            nodes[4],
            multiplicity * (d394_dn4),
        );
    }
}
