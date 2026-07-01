#![allow(dead_code, unused_assignments, unused_parens, unused_variables)]

use super::state::Instance;
use crate::device::veriloga_generated::{GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper};

const LIMEXP_MAX: f64 = 5.54062238439351e34;
#[path = "stamp_blocks_0.rs"]
mod stamp_blocks_0;

const THERMAL_VOLTAGE_PER_K: f64 = 1.380649e-23 / 1.602176634e-19;

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

#[inline]
fn ddt_jacobian(ddt_active: bool, ddt_scale: f64, derivative: f64) -> f64 {
    if ddt_active {
        derivative * ddt_scale
    } else {
        0.0
    }
}

#[inline]
fn eval_idt<const STATE_COUNT: usize>(
    current: &mut [f64; STATE_COUNT],
    previous: &mut [f64; STATE_COUNT],
    initialized: &mut [bool; STATE_COUNT],
    ddt_active: bool,
    idt_scale: f64,
    slot: usize,
    value: f64,
    ic: f64,
) -> f64 {
    debug_assert!(slot < STATE_COUNT, "generated idt state slot out of range");
    let previous_value = if initialized[slot] { previous[slot] } else { ic };
    let current_value = if ddt_active {
        previous_value + value * idt_scale
    } else {
        ic
    };
    current[slot] = current_value;
    if !ddt_active {
        previous[slot] = current_value;
        initialized[slot] = true;
    }
    current_value
}

#[inline]
fn idt_jacobian(timestep: f64, derivative: f64) -> f64 {
    if timestep.abs() > Instance::DDT_EPSILON {
        derivative * timestep
    } else {
        0.0
    }
}

#[derive(Default)]
pub(crate) struct StampLocals {
    pub(crate) var_guard10: f64,
    pub(crate) var_guard10_rv: f64,
    pub(crate) var_guard11: f64,
    pub(crate) var_guard11_rv: f64,
    pub(crate) var_guard8: f64,
    pub(crate) var_guard8_rv: f64,
    pub(crate) var_tff: f64,
    pub(crate) var_tff_dn0: f64,
    pub(crate) var_tff_dn1: f64,
    pub(crate) var_tff_rv: f64,
    pub(crate) var_vtff: f64,
    pub(crate) var_vtff1: f64,
    pub(crate) var_vtff1_dn0: f64,
    pub(crate) var_vtff1_dn1: f64,
    pub(crate) var_vtff1_rv: f64,
    pub(crate) var_vtff_dn0: f64,
    pub(crate) var_vtff_dn1: f64,
    pub(crate) var_vtff_rv: f64,
}

impl Instance {
    pub fn stamp(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedStamper<'_>) {
        let p = Box::as_ref(&self.params);
        let nodes = &(*self).nodes;
        let branches = &(*self).branches;
        let ctx_temp = ctx.temperature();
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv1 = ctx.node_voltage(nodes[1]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv4 = ctx.node_voltage(nodes[4]);
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let multiplicity = (*self).multiplicity;
        let timestep = (*self).timestep;
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
        let v0: f64 = ctx_temp;
        let v1: f64 = nv2;
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
        let v105: f64 = nv3;
        let v106: f64 = nv4;
        let v107: f64 = (v105 - v106);
        let v108: f64 = (self.scalar_v104 * v107);
        let v109: f64 = nv0;
        let v110: f64 = (v109 - v105);
        let v111: f64 = (self.scalar_v104 * v110);
        let v112: f64 = nv1;
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
        let v269: f64 = nv6;
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
        let v351: f64 = (v214 * v251);
        let v352: f64 = ((v351) as f64).abs();
        let v353: f64 = (-v352);
        let v354: f64 = (if self.scalar_v323 { v353 } else { v13 });
        let v355: f64 = (v1 / self.scalar_v321);
        let v356: f64 = (if self.scalar_v323 { v355 } else { v13 });
        let v360: f64 = (if self.scalar_v359 { v353 } else { v13 });
        let v361: f64 = nv5;
        let v362: f64 = (v1 - v361);
        let v363: f64 = (v362 / self.scalar_v321);
        let v364: f64 = (if self.scalar_v359 { v363 } else { v13 });
        let v365: f64 = (v361 / self.scalar_v327);
        let v366: f64 = (if self.scalar_v359 { v365 } else { v13 });
        let v370: f64 = (if self.scalar_v369 { v353 } else { v13 });
        let v375: f64 = (v13 * v107);
        let v376: bool = (v341 > self.scalar_v338);
        let v377: f64 = (if v376 { v341 } else { self.scalar_v338 });
        let v378: f64 = (v110 / v377);
        let v379: f64 = (if self.scalar_v340 { v378 } else { v13 });
        let v383: bool = (v345 > self.scalar_v338);
        let v384: f64 = (if v383 { v345 } else { self.scalar_v338 });
        let v385: f64 = (v113 / v384);
        let v386: f64 = (if self.scalar_v344 { v385 } else { v13 });
        let v390: f64 = (self.scalar_v104 * v214);
        let v391: f64 = (self.scalar_v16 * v390);
        let v392: f64 = (self.scalar_v104 * v318);
        let v393: f64 = (self.scalar_v16 * v392);
        let v394: f64 = (self.scalar_v104 * v266);
        let v395: f64 = (self.scalar_v16 * v394);
        let v396: f64 = (if v8 { v12 } else { v13 });
        let v397: f64 = (if v10 { v13 } else { v396 });
        let v398: f64 = (v19 * v397);
        let v399: f64 = (v397 / self.scalar_v18);
        let v400: f64 = (v399 / v21);
        let v401: f64 = (self.scalar_v23 * v400);
        let v402: f64 = (self.scalar_v25 * v399);
        let v403: f64 = (v20 * v402);
        let v404: f64 = (v27 * v398);
        let v405: f64 = (v403 - v404);
        let v406: f64 = (v20 * v20);
        let v407: f64 = (v405 / v406);
        let v408: f64 = (v401 + v407);
        let v409: f64 = (self.scalar_v30 * v400);
        let v410: f64 = (v33 * v408);
        let v411: f64 = (self.scalar_v32 * v410);
        let v412: f64 = (v36 * v409);
        let v413: f64 = (self.scalar_v35 * v412);
        let v414: f64 = (self.scalar_v39 * v399);
        let v415: f64 = (self.scalar_v38 * v414);
        let v416: f64 = (self.scalar_v44 * v399);
        let v417: f64 = (self.scalar_v43 * v416);
        let v418: f64 = (self.scalar_v49 * v399);
        let v419: f64 = (self.scalar_v48 * v418);
        let v420: f64 = (v397 / v54);
        let v421: f64 = (v58 * v397);
        let v422: f64 = (v59 * v397);
        let v423: f64 = (v11 * v421);
        let v424: f64 = (v422 + v423);
        let v425: f64 = (v62 * v424);
        let v426: f64 = (v60 * v397);
        let v427: f64 = (v425 - v426);
        let v428: f64 = (v62 * v62);
        let v429: f64 = (v427 / v428);
        let v430: f64 = (v397 + v397);
        let v431: f64 = (v66 * v430);
        let v432: f64 = (v68 * v429);
        let v433: f64 = (v65 * v431);
        let v434: f64 = (v432 - v433);
        let v435: f64 = (v68 * v68);
        let v436: f64 = (v434 / v435);
        let v437: f64 = (v398 + v398);
        let v438: f64 = (-v437);
        let v439: f64 = (v420 / v56);
        let v440: f64 = (v74 * v439);
        let v441: f64 = (v77 * v436);
        let v442: f64 = (v440 + v441);
        let v443: f64 = (v79 * v438);
        let v444: f64 = (v73 * v442);
        let v445: f64 = (v443 + v444);
        let v446: f64 = (-v445);
        let v447: f64 = (v446 / self.scalar_v55);
        let v448: f64 = (-v447);
        let v449: f64 = (v83 * v448);
        let v450: f64 = (v84 * v447);
        let v451: f64 = (v449 - v450);
        let v452: f64 = (v83 * v83);
        let v453: f64 = (v451 / v452);
        let v454: f64 = (-v453);
        let v455: f64 = (self.scalar_v86 * v454);
        let v456: f64 = (self.scalar_v53 * v455);
        let v457: f64 = (-v456);
        let v458: f64 = (v92 * v92);
        let v459: f64 = (v457 / v458);
        let v460: f64 = (v83 * v420);
        let v461: f64 = (v56 * v447);
        let v462: f64 = (v460 + v461);
        let v463: f64 = (v445 + v462);
        let v464: f64 = (v463 - v447);
        let v465: f64 = (v83 * v464);
        let v466: f64 = (v96 * v447);
        let v467: f64 = (v465 - v466);
        let v468: f64 = (v467 / v452);
        let v469: f64 = (v87 * v397);
        let v470: f64 = (v469 - v468);
        let v471: f64 = (self.scalar_v86 * v470);
        let v472: f64 = (v102 * v459);
        let v473: f64 = (v93 * v471);
        let v474: f64 = (v472 + v473);
        let v476: f64 = (self.scalar_v116 * v398);
        let v477: f64 = (v108 * v476);
        let v478: f64 = (-v477);
        let v479: f64 = (v117 * v117);
        let v480: f64 = (v478 / v479);
        let v481: f64 = (self.scalar_v104 / v117);
        let v482: f64 = (self.scalar_v475 / v117);
        let v483: f64 = (if v115 { v480 } else { v13 });
        let v484: f64 = (if v115 { v481 } else { v13 });
        let v485: f64 = (if v115 { v482 } else { v13 });
        let v486: f64 = (-v417);
        let v487: f64 = (self.scalar_v122 * v398);
        let v488: f64 = (v123 * v486);
        let v489: f64 = (v121 * v487);
        let v490: f64 = (v488 - v489);
        let v491: f64 = (v123 * v123);
        let v492: f64 = (v490 / v491);
        let v493: f64 = (self.scalar_v475 / v123);
        let v494: f64 = (self.scalar_v104 / v123);
        let v495: f64 = (if v115 { v492 } else { v13 });
        let v496: f64 = (if v115 { v493 } else { v13 });
        let v497: f64 = (if v115 { v494 } else { v13 });
        let v498: f64 = (v126 * v487);
        let v499: f64 = (v488 - v498);
        let v500: f64 = (v499 / v491);
        let v501: f64 = (if v115 { v500 } else { v13 });
        let v502: f64 = (if v131 { v483 } else { v13 });
        let v503: f64 = (if v131 { v484 } else { v13 });
        let v504: f64 = (if v131 { v485 } else { v13 });
        let v505: f64 = (if v131 { v13 } else { v483 });
        let v506: f64 = (if v131 { v13 } else { v484 });
        let v507: f64 = (if v131 { v13 } else { v485 });
        let v508: f64 = (if v137 { v13 } else { v502 });
        let v509: f64 = (if v137 { v13 } else { v503 });
        let v510: f64 = (if v137 { v13 } else { v504 });
        let v511: f64 = (v139 * v505);
        let v512: f64 = (v139 * v506);
        let v513: f64 = (v139 * v507);
        let v514: f64 = (v139 * v508);
        let v515: f64 = (v138 * v511);
        let v516: f64 = (v514 + v515);
        let v517: f64 = (v139 * v509);
        let v518: f64 = (v138 * v512);
        let v519: f64 = (v517 + v518);
        let v520: f64 = (v139 * v510);
        let v521: f64 = (v138 * v513);
        let v522: f64 = (v520 + v521);
        let v523: f64 = (if v115 { v516 } else { v508 });
        let v524: f64 = (if v115 { v519 } else { v509 });
        let v525: f64 = (if v115 { v522 } else { v510 });
        let v526: f64 = (v149 * v495);
        let v527: f64 = (v149 * v496);
        let v528: f64 = (v149 * v497);
        let v529: f64 = (v526 / v150);
        let v530: f64 = (v527 / v150);
        let v531: f64 = (v528 / v150);
        let v532: f64 = (if v143 { v495 } else { v13 });
        let v533: f64 = (if v143 { v496 } else { v13 });
        let v534: f64 = (if v143 { v497 } else { v13 });
        let v535: f64 = (if v152 { v526 } else { v532 });
        let v536: f64 = (if v152 { v527 } else { v533 });
        let v537: f64 = (if v152 { v528 } else { v534 });
        let v538: f64 = (if v148 { v529 } else { v535 });
        let v539: f64 = (if v148 { v530 } else { v536 });
        let v540: f64 = (if v148 { v531 } else { v537 });
        let v541: f64 = (v161 * v501);
        let v542: f64 = (v541 / v162);
        let v543: f64 = (if v156 { v501 } else { v13 });
        let v544: f64 = (if v164 { v541 } else { v543 });
        let v545: f64 = (if v160 { v542 } else { v544 });
        let v546: f64 = (v538 - v545);
        let v547: f64 = (if v115 { v546 } else { v13 });
        let v548: f64 = (if v115 { v539 } else { v13 });
        let v549: f64 = (if v115 { v540 } else { v13 });
        let v550: f64 = (v170 * v411);
        let v551: f64 = (v34 * v523);
        let v552: f64 = (v550 + v551);
        let v553: f64 = (v34 * v524);
        let v554: f64 = (v34 * v525);
        let v555: f64 = (v169 * v415);
        let v556: f64 = (v42 * v547);
        let v557: f64 = (v555 + v556);
        let v558: f64 = (v42 * v548);
        let v559: f64 = (v42 * v549);
        let v560: f64 = ((v174) as f64).ln();
        let v561: f64 = (v175 * v560);
        let v562: f64 = (v419 * v561);
        let v563: f64 = (self.scalar_v173 * v562);
        let v564: f64 = (v177 * v557);
        let v565: f64 = (v172 * v563);
        let v566: f64 = (v564 - v565);
        let v567: f64 = (v177 * v177);
        let v568: f64 = (v566 / v567);
        let v569: f64 = (v558 / v177);
        let v570: f64 = (v559 / v177);
        let v571: f64 = (v552 - v568);
        let v572: f64 = (v553 - v569);
        let v573: f64 = (v554 - v570);
        let v574: f64 = (if v115 { v571 } else { v13 });
        let v575: f64 = (if v115 { v572 } else { v13 });
        let v576: f64 = (if v115 { v573 } else { v13 });
        let v577: f64 = (if v181 { v13 } else { v574 });
        let v578: f64 = (if v181 { v13 } else { v575 });
        let v579: f64 = (if v181 { v13 } else { v576 });
        let v580: f64 = (if v187 { self.scalar_v475 } else { v13 });
        let v581: f64 = (if v187 { self.scalar_v104 } else { v13 });
        let v582: f64 = (if v183 { v580 } else { v13 });
        let v583: f64 = (if v183 { v581 } else { v13 });
        let v586: f64 = (self.scalar_v192 * v398);
        let v587: f64 = (v189 * v586);
        let v588: f64 = (v193 * v582);
        let v589: f64 = (v193 * v583);
        let v590: f64 = (v191 * v587);
        let v591: f64 = (-v590);
        let v592: f64 = (v194 * v194);
        let v593: f64 = (v591 / v592);
        let v594: f64 = (v194 * self.scalar_v584);
        let v595: f64 = (v191 * v588);
        let v596: f64 = (v594 - v595);
        let v597: f64 = (v596 / v592);
        let v598: f64 = (v194 * self.scalar_v585);
        let v599: f64 = (v191 * v589);
        let v600: f64 = (v598 - v599);
        let v601: f64 = (v600 / v592);
        let v602: f64 = (if v183 { v593 } else { v505 });
        let v603: f64 = (if v183 { v597 } else { v506 });
        let v604: f64 = (if v183 { v601 } else { v507 });
        let v605: f64 = (if v198 { v602 } else { v523 });
        let v606: f64 = (if v198 { v603 } else { v524 });
        let v607: f64 = (if v198 { v604 } else { v525 });
        let v608: f64 = (if v198 { v13 } else { v602 });
        let v609: f64 = (if v198 { v13 } else { v603 });
        let v610: f64 = (if v198 { v13 } else { v604 });
        let v611: f64 = (if v204 { v13 } else { v605 });
        let v612: f64 = (if v204 { v13 } else { v606 });
        let v613: f64 = (if v204 { v13 } else { v607 });
        let v614: f64 = (v206 * v608);
        let v615: f64 = (v206 * v609);
        let v616: f64 = (v206 * v610);
        let v617: f64 = (v206 * v611);
        let v618: f64 = (v205 * v614);
        let v619: f64 = (v617 + v618);
        let v620: f64 = (v206 * v612);
        let v621: f64 = (v205 * v615);
        let v622: f64 = (v620 + v621);
        let v623: f64 = (v206 * v613);
        let v624: f64 = (v205 * v616);
        let v625: f64 = (v623 + v624);
        let v626: f64 = (if v183 { v619 } else { v611 });
        let v627: f64 = (if v183 { v622 } else { v612 });
        let v628: f64 = (if v183 { v625 } else { v613 });
        let v629: f64 = (v209 * v413);
        let v630: f64 = (v37 * v626);
        let v631: f64 = (v629 + v630);
        let v632: f64 = (v37 * v627);
        let v633: f64 = (v37 * v628);
        let v634: f64 = (if v183 { v631 } else { v13 });
        let v635: f64 = (if v183 { v632 } else { v13 });
        let v636: f64 = (if v183 { v633 } else { v13 });
        let v637: f64 = (if v212 { v13 } else { v634 });
        let v638: f64 = (if v212 { v13 } else { v635 });
        let v639: f64 = (if v212 { v13 } else { v636 });
        let v640: f64 = (v577 - v637);
        let v641: f64 = (v578 - v638);
        let v642: f64 = (v579 - v639);
        let v643: f64 = (self.scalar_v228 * v400);
        let v644: f64 = (v230 * v643);
        let v645: f64 = (self.scalar_v227 * v644);
        let v646: f64 = (v233 * v645);
        let v647: f64 = (self.scalar_v236 * v400);
        let v648: f64 = (v238 * v647);
        let v649: f64 = (self.scalar_v235 * v648);
        let v650: f64 = (v241 * v649);
        let v651: f64 = (if self.scalar_v244 { v646 } else { v646 });
        let v652: f64 = (if self.scalar_v244 { v650 } else { v650 });
        let v653: f64 = (v265 * v577);
        let v654: f64 = (v265 * v578);
        let v655: f64 = (v265 * v579);
        let v656: f64 = (v651 / v275);
        let v657: f64 = (if self.scalar_v268 { v656 } else { v651 });
        let v658: f64 = (if self.scalar_v278 { v657 } else { v657 });
        let v659: f64 = (-v463);
        let v660: f64 = (self.scalar_v281 * v659);
        let v661: f64 = (v293 * v463);
        let v662: f64 = (v661 / self.scalar_v295);
        let v663: f64 = (if v284 { v662 } else { v13 });
        let v664: f64 = (self.scalar_v299 * v660);
        let v667: f64 = (v95 * v664);
        let v668: f64 = (v300 * v463);
        let v669: f64 = (v667 - v668);
        let v670: f64 = (v95 * v95);
        let v671: f64 = (v669 / v670);
        let v672: f64 = (self.scalar_v665 / v95);
        let v673: f64 = (self.scalar_v666 / v95);
        let v674: f64 = (v302 * v660);
        let v675: f64 = (v283 * v671);
        let v676: f64 = (v674 + v675);
        let v677: f64 = (self.scalar_v104 * v302);
        let v678: f64 = (v283 * v672);
        let v679: f64 = (v677 + v678);
        let v680: f64 = (v302 * self.scalar_v475);
        let v681: f64 = (v283 * v673);
        let v682: f64 = (v680 + v681);
        let v683: f64 = (v290 * v676);
        let v684: f64 = (v290 * v679);
        let v685: f64 = (v290 * v682);
        let v686: f64 = (if v284 { v683 } else { v13 });
        let v687: f64 = (if v284 { v684 } else { v13 });
        let v688: f64 = (if v284 { v685 } else { v13 });
        let v689: f64 = (v108 * v463);
        let v690: f64 = (-v689);
        let v691: f64 = (v690 / v670);
        let v692: f64 = (self.scalar_v104 / v95);
        let v693: f64 = (self.scalar_v475 / v95);
        let v694: f64 = (-v691);
        let v695: f64 = (-v692);
        let v696: f64 = (-v693);
        let v697: f64 = (v694 / v308);
        let v698: f64 = (v695 / v308);
        let v699: f64 = (v696 / v308);
        let v700: f64 = (self.scalar_v295 * v697);
        let v701: f64 = (self.scalar_v295 * v698);
        let v702: f64 = (self.scalar_v295 * v699);
        let v703: f64 = (v311 * v700);
        let v704: f64 = (v311 * v701);
        let v705: f64 = (v311 * v702);
        let v706: f64 = (-v703);
        let v707: f64 = (-v704);
        let v708: f64 = (-v705);
        let v709: f64 = (v312 * v463);
        let v710: f64 = (v95 * v706);
        let v711: f64 = (v709 + v710);
        let v712: f64 = (v95 * v707);
        let v713: f64 = (v95 * v708);
        let v714: f64 = (v711 / self.scalar_v295);
        let v715: f64 = (v712 / self.scalar_v295);
        let v716: f64 = (v713 / self.scalar_v295);
        let v717: f64 = (if v306 { v714 } else { v663 });
        let v718: f64 = (if v306 { v715 } else { v13 });
        let v719: f64 = (if v306 { v716 } else { v13 });
        let v720: f64 = (if v306 { v13 } else { v686 });
        let v721: f64 = (if v306 { v13 } else { v687 });
        let v722: f64 = (if v306 { v13 } else { v688 });
        let v723: f64 = (v717 + v720);
        let v724: f64 = (v718 + v721);
        let v725: f64 = (v719 + v722);
        let v726: f64 = (v317 * v474);
        let v727: f64 = (v103 * v723);
        let v728: f64 = (v726 + v727);
        let v729: f64 = (v103 * v724);
        let v730: f64 = (v103 * v725);
        let v731: f64 = (v658 / self.scalar_v16);
        let v732: f64 = (v652 / self.scalar_v16);
        let v733: f64 = (-v577);
        let v734: f64 = (-v578);
        let v735: f64 = (-v579);
        let v736: f64 = (v265 * v733);
        let v737: f64 = (v265 * v734);
        let v738: f64 = (v265 * v735);
        let v739: f64 = (if self.scalar_v268 { v736 } else { v13 });
        let v740: f64 = (if self.scalar_v268 { v737 } else { v13 });
        let v741: f64 = (if self.scalar_v268 { v738 } else { v13 });
        let v750: f64 = -0.0;
        let v751: f64 = (if v376 { v731 } else { v13 });
        let v752: f64 = (v12 / v377);
        let v753: f64 = (v110 * v751);
        let v754: f64 = (-v753);
        let v755: f64 = (v377 * v377);
        let v756: f64 = (v754 / v755);
        let v757: f64 = (v190 / v377);
        let v758: f64 = (if self.scalar_v340 { v752 } else { v13 });
        let v759: f64 = (if self.scalar_v340 { v756 } else { v13 });
        let v760: f64 = (if self.scalar_v340 { v757 } else { v13 });
        let v761: f64 = (if v383 { v732 } else { v13 });
        let v762: f64 = (v12 / v384);
        let v763: f64 = (v113 * v761);
        let v764: f64 = (-v763);
        let v765: f64 = (v384 * v384);
        let v766: f64 = (v764 / v765);
        let v767: f64 = (v190 / v384);
        let v768: f64 = (if self.scalar_v344 { v762 } else { v13 });
        let v769: f64 = (if self.scalar_v344 { v766 } else { v13 });
        let v770: f64 = (if self.scalar_v344 { v767 } else { v13 });
        let v771: f64 = (self.scalar_v104 * v640);
        let v772: f64 = (self.scalar_v104 * v641);
        let v773: f64 = (self.scalar_v104 * v642);
        let v774: f64 = (self.scalar_v16 * v771);
        let v775: f64 = (self.scalar_v16 * v772);
        let v776: f64 = (self.scalar_v16 * v773);
        let v777: f64 = (self.scalar_v104 * v728);
        let v778: f64 = (self.scalar_v104 * v729);
        let v779: f64 = (self.scalar_v104 * v730);
        let v780: f64 = (self.scalar_v16 * v777);
        let v781: f64 = (self.scalar_v16 * v778);
        let v782: f64 = (self.scalar_v16 * v779);
        let v783: f64 = (self.scalar_v104 * v653);
        let v784: f64 = (self.scalar_v104 * v654);
        let v785: f64 = (self.scalar_v104 * v655);
        let v786: f64 = (self.scalar_v16 * v783);
        let v787: f64 = (self.scalar_v16 * v784);
        let v788: f64 = (self.scalar_v16 * v785);

        let d348_dn2: f64 = v739;
        let d348_dn3: f64 = v740;
        let d348_dn4: f64 = v741;
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
        let d349_dn6: f64 = self.scalar_v742;
        stamper.stamp_current_node1_local(
            Some(6),
            None,
            multiplicity * (v349),
            6,
            multiplicity * (d349_dn6),
        );
        stamper.stamp_potential_branch_local(
            Some(6),
            None,
            0,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            0,
            self.scalar_v350,
        );
        stamper.stamp_current_const_local(
            Some(2),
            None,
            multiplicity * (v354),
        );
        let d356_dn2: f64 = self.scalar_v744;
        stamper.stamp_current_node1_local(
            Some(2),
            None,
            multiplicity * (v356),
            2,
            multiplicity * (d356_dn2),
        );
        stamper.stamp_potential_branch_local(
            Some(5),
            None,
            1,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            1,
            self.scalar_v357,
        );
        stamper.stamp_current_const_local(
            Some(2),
            None,
            multiplicity * (v360),
        );
        let d364_dn2: f64 = self.scalar_v746;
        let d364_dn5: f64 = self.scalar_v747;
        stamper.stamp_current_node2_local(
            Some(2),
            Some(5),
            multiplicity * (v364),
            2,
            multiplicity * (d364_dn2),
            5,
            multiplicity * (d364_dn5),
        );
        let d366_dn5: f64 = self.scalar_v749;
        stamper.stamp_current_node1_local(
            Some(5),
            None,
            multiplicity * (v366),
            5,
            multiplicity * (d366_dn5),
        );
        stamper.stamp_current_const_local(
            Some(2),
            None,
            multiplicity * (v370),
        );
        stamper.stamp_potential_branch_local(
            Some(5),
            None,
            2,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            2,
            self.scalar_v371,
        );
        stamper.stamp_potential_branch_local(
            Some(2),
            None,
            3,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            3,
            self.scalar_v374,
        );
        stamper.stamp_potential_branch_local(
            Some(5),
            None,
            4,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            4,
            self.scalar_v374,
        );
        let d375_dn3: f64 = v13;
        let d375_dn4: f64 = v750;
        stamper.stamp_current_node2_local(
            Some(3),
            Some(4),
            multiplicity * (v375),
            3,
            multiplicity * (d375_dn3),
            4,
            multiplicity * (d375_dn4),
        );
        let d379_dn0: f64 = v758;
        let d379_dn2: f64 = v759;
        let d379_dn3: f64 = v760;
        stamper.stamp_current_node3_local(
            Some(0),
            Some(3),
            multiplicity * (v379),
            0,
            multiplicity * (d379_dn0),
            2,
            multiplicity * (d379_dn2),
            3,
            multiplicity * (d379_dn3),
        );
        stamper.stamp_current_const_local(
            Some(0),
            Some(3),
            multiplicity * (self.scalar_v380),
        );
        stamper.stamp_potential_branch_local(
            Some(0),
            Some(3),
            5,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            5,
            self.scalar_v382,
        );
        let d386_dn1: f64 = v768;
        let d386_dn2: f64 = v769;
        let d386_dn4: f64 = v770;
        stamper.stamp_current_node3_local(
            Some(1),
            Some(4),
            multiplicity * (v386),
            1,
            multiplicity * (d386_dn1),
            2,
            multiplicity * (d386_dn2),
            4,
            multiplicity * (d386_dn4),
        );
        stamper.stamp_current_const_local(
            Some(1),
            Some(4),
            multiplicity * (self.scalar_v387),
        );
        stamper.stamp_potential_branch_local(
            Some(1),
            Some(4),
            6,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            6,
            self.scalar_v389,
        );
        let d391_dn2: f64 = v774;
        let d391_dn3: f64 = v775;
        let d391_dn4: f64 = v776;
        stamper.stamp_current_node3_local(
            Some(3),
            Some(4),
            multiplicity * (v391),
            2,
            multiplicity * (d391_dn2),
            3,
            multiplicity * (d391_dn3),
            4,
            multiplicity * (d391_dn4),
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
        let d393_dn2: f64 = v780;
        let d393_dn3: f64 = v781;
        let d393_dn4: f64 = v782;
        let v393_ddt: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, v393);
        stamper.stamp_current_node3_local(
            Some(3),
            Some(4),
            multiplicity * (v393_ddt),
            2,
            multiplicity * (((d393_dn2) * ddt_scale)),
            3,
            multiplicity * (((d393_dn3) * ddt_scale)),
            4,
            multiplicity * (((d393_dn4) * ddt_scale)),
        );
        let d395_dn2: f64 = v786;
        let d395_dn3: f64 = v787;
        let d395_dn4: f64 = v788;
        let v395_ddt: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 5, v395);
        stamper.stamp_current_node3_local(
            Some(3),
            Some(4),
            multiplicity * (v395_ddt),
            2,
            multiplicity * (((d395_dn2) * ddt_scale)),
            3,
            multiplicity * (((d395_dn3) * ddt_scale)),
            4,
            multiplicity * (((d395_dn4) * ddt_scale)),
        );
        let mut locals = StampLocals::default();

        let assign630_e796: f64 = ((nv0 - nv1) / p.p40);
        let assign630_e797: f64 = (assign630_e796).abs();
        let assign630_e799: f64 = (assign630_e797).powf(p.p39);
        locals.var_vtff = assign630_e799;
        locals.var_vtff_dn0 = if 0.0 == 0.0 && ((p.p39) as f64).is_finite() && ((p.p39) as f64).fract() == 0.0 { if p.p39 == 0.0 { 0.0 } else { (p.p39 * ((assign630_e797).powf(p.p39 - 1.0) * if assign630_e796 >= 0.0 { (1.0 / p.p40) } else { (-(1.0 / p.p40)) })) } } else { (assign630_e799 * (p.p39 * (if assign630_e796 >= 0.0 { (1.0 / p.p40) } else { (-(1.0 / p.p40)) } / assign630_e797))) };
        locals.var_vtff_dn1 = if 0.0 == 0.0 && ((p.p39) as f64).is_finite() && ((p.p39) as f64).fract() == 0.0 { if p.p39 == 0.0 { 0.0 } else { (p.p39 * ((assign630_e797).powf(p.p39 - 1.0) * if assign630_e796 >= 0.0 { (-1.0 / p.p40) } else { (-(-1.0 / p.p40)) })) } } else { (assign630_e799 * (p.p39 * (if assign630_e796 >= 0.0 { (-1.0 / p.p40) } else { (-(-1.0 / p.p40)) } / assign630_e797))) };

        let assign640_e802: f64 = (1.0 + locals.var_vtff);
        let assign640_e805: f64 = (1.0 / p.p39);
        let assign640_e806: f64 = (assign640_e802).powf(assign640_e805);
        let assign640_e808: f64 = (assign640_e806 - 1.0);
        locals.var_vtff1 = assign640_e808;
        locals.var_vtff1_dn0 = if 0.0 == 0.0 && ((assign640_e805) as f64).is_finite() && ((assign640_e805) as f64).fract() == 0.0 { if assign640_e805 == 0.0 { 0.0 } else { (assign640_e805 * ((assign640_e802).powf(assign640_e805 - 1.0) * locals.var_vtff_dn0)) } } else { (assign640_e806 * (assign640_e805 * (locals.var_vtff_dn0 / assign640_e802))) };
        locals.var_vtff1_dn1 = if 0.0 == 0.0 && ((assign640_e805) as f64).is_finite() && ((assign640_e805) as f64).fract() == 0.0 { if assign640_e805 == 0.0 { 0.0 } else { (assign640_e805 * ((assign640_e802).powf(assign640_e805 - 1.0) * locals.var_vtff_dn1)) } } else { (assign640_e806 * (assign640_e805 * (locals.var_vtff_dn1 / assign640_e802))) };

        let assign650_e813: f64 = (p.p41 * locals.var_vtff1);
        let assign650_e814: f64 = (1.0 + assign650_e813);
        let assign650_e815: f64 = (p.p19 * assign650_e814);
        locals.var_tff = assign650_e815;
        locals.var_tff_dn0 = (p.p19 * (p.p41 * locals.var_vtff1_dn0));
        locals.var_tff_dn1 = (p.p19 * (p.p41 * locals.var_vtff1_dn1));

        let assign670_e821: f64 = if p.p32 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard8 = assign670_e821;

        let assign790_e940: f64 = if ((p.p30 == 1.0) && (p.p33 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard10 = assign790_e940;

        let assign800_e951: f64 = if (((p.p30 == 2.0) && (p.p33 > 0.0)) && (p.p35 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard11 = assign800_e951;

        Self::stamp_transient_equations_block_0(ctx, stamper, p, nodes, multiplicity, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, &mut locals);
    }

    pub fn stamp_reactive(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedReactiveStamper<'_>) {
        let p = Box::as_ref(&self.params);
        let nodes = &(*self).nodes;
        let branches = &(*self).branches;
        let ctx_temp = ctx.temperature();
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv1 = ctx.node_voltage(nodes[1]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv4 = ctx.node_voltage(nodes[4]);
        let multiplicity = (*self).multiplicity;
        let v0: f64 = ctx_temp;
        let v1: f64 = nv2;
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
        let v105: f64 = nv3;
        let v106: f64 = nv4;
        let v107: f64 = (v105 - v106);
        let v108: f64 = (self.scalar_v104 * v107);
        let v109: f64 = nv0;
        let v112: f64 = nv1;
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
        let v392: f64 = (self.scalar_v104 * v318);
        let v393: f64 = (self.scalar_v16 * v392);
        let v394: f64 = (self.scalar_v104 * v266);
        let v395: f64 = (self.scalar_v16 * v394);
        let v396: f64 = (if v8 { v12 } else { v13 });
        let v397: f64 = (if v10 { v13 } else { v396 });
        let v398: f64 = (v19 * v397);
        let v399: f64 = (v397 / self.scalar_v18);
        let v400: f64 = (v399 / v21);
        let v401: f64 = (self.scalar_v23 * v400);
        let v402: f64 = (self.scalar_v25 * v399);
        let v403: f64 = (v20 * v402);
        let v404: f64 = (v27 * v398);
        let v405: f64 = (v403 - v404);
        let v406: f64 = (v20 * v20);
        let v407: f64 = (v405 / v406);
        let v408: f64 = (v401 + v407);
        let v410: f64 = (v33 * v408);
        let v411: f64 = (self.scalar_v32 * v410);
        let v414: f64 = (self.scalar_v39 * v399);
        let v415: f64 = (self.scalar_v38 * v414);
        let v416: f64 = (self.scalar_v44 * v399);
        let v417: f64 = (self.scalar_v43 * v416);
        let v418: f64 = (self.scalar_v49 * v399);
        let v419: f64 = (self.scalar_v48 * v418);
        let v420: f64 = (v397 / v54);
        let v421: f64 = (v58 * v397);
        let v422: f64 = (v59 * v397);
        let v423: f64 = (v11 * v421);
        let v424: f64 = (v422 + v423);
        let v425: f64 = (v62 * v424);
        let v426: f64 = (v60 * v397);
        let v427: f64 = (v425 - v426);
        let v428: f64 = (v62 * v62);
        let v429: f64 = (v427 / v428);
        let v430: f64 = (v397 + v397);
        let v431: f64 = (v66 * v430);
        let v432: f64 = (v68 * v429);
        let v433: f64 = (v65 * v431);
        let v434: f64 = (v432 - v433);
        let v435: f64 = (v68 * v68);
        let v436: f64 = (v434 / v435);
        let v437: f64 = (v398 + v398);
        let v438: f64 = (-v437);
        let v439: f64 = (v420 / v56);
        let v440: f64 = (v74 * v439);
        let v441: f64 = (v77 * v436);
        let v442: f64 = (v440 + v441);
        let v443: f64 = (v79 * v438);
        let v444: f64 = (v73 * v442);
        let v445: f64 = (v443 + v444);
        let v446: f64 = (-v445);
        let v447: f64 = (v446 / self.scalar_v55);
        let v448: f64 = (-v447);
        let v449: f64 = (v83 * v448);
        let v450: f64 = (v84 * v447);
        let v451: f64 = (v449 - v450);
        let v452: f64 = (v83 * v83);
        let v453: f64 = (v451 / v452);
        let v454: f64 = (-v453);
        let v455: f64 = (self.scalar_v86 * v454);
        let v456: f64 = (self.scalar_v53 * v455);
        let v457: f64 = (-v456);
        let v458: f64 = (v92 * v92);
        let v459: f64 = (v457 / v458);
        let v460: f64 = (v83 * v420);
        let v461: f64 = (v56 * v447);
        let v462: f64 = (v460 + v461);
        let v463: f64 = (v445 + v462);
        let v464: f64 = (v463 - v447);
        let v465: f64 = (v83 * v464);
        let v466: f64 = (v96 * v447);
        let v467: f64 = (v465 - v466);
        let v468: f64 = (v467 / v452);
        let v469: f64 = (v87 * v397);
        let v470: f64 = (v469 - v468);
        let v471: f64 = (self.scalar_v86 * v470);
        let v472: f64 = (v102 * v459);
        let v473: f64 = (v93 * v471);
        let v474: f64 = (v472 + v473);
        let v476: f64 = (self.scalar_v116 * v398);
        let v477: f64 = (v108 * v476);
        let v478: f64 = (-v477);
        let v479: f64 = (v117 * v117);
        let v480: f64 = (v478 / v479);
        let v481: f64 = (self.scalar_v104 / v117);
        let v482: f64 = (self.scalar_v475 / v117);
        let v483: f64 = (if v115 { v480 } else { v13 });
        let v484: f64 = (if v115 { v481 } else { v13 });
        let v485: f64 = (if v115 { v482 } else { v13 });
        let v486: f64 = (-v417);
        let v487: f64 = (self.scalar_v122 * v398);
        let v488: f64 = (v123 * v486);
        let v489: f64 = (v121 * v487);
        let v490: f64 = (v488 - v489);
        let v491: f64 = (v123 * v123);
        let v492: f64 = (v490 / v491);
        let v493: f64 = (self.scalar_v475 / v123);
        let v494: f64 = (self.scalar_v104 / v123);
        let v495: f64 = (if v115 { v492 } else { v13 });
        let v496: f64 = (if v115 { v493 } else { v13 });
        let v497: f64 = (if v115 { v494 } else { v13 });
        let v498: f64 = (v126 * v487);
        let v499: f64 = (v488 - v498);
        let v500: f64 = (v499 / v491);
        let v501: f64 = (if v115 { v500 } else { v13 });
        let v502: f64 = (if v131 { v483 } else { v13 });
        let v503: f64 = (if v131 { v484 } else { v13 });
        let v504: f64 = (if v131 { v485 } else { v13 });
        let v505: f64 = (if v131 { v13 } else { v483 });
        let v506: f64 = (if v131 { v13 } else { v484 });
        let v507: f64 = (if v131 { v13 } else { v485 });
        let v508: f64 = (if v137 { v13 } else { v502 });
        let v509: f64 = (if v137 { v13 } else { v503 });
        let v510: f64 = (if v137 { v13 } else { v504 });
        let v511: f64 = (v139 * v505);
        let v512: f64 = (v139 * v506);
        let v513: f64 = (v139 * v507);
        let v514: f64 = (v139 * v508);
        let v515: f64 = (v138 * v511);
        let v516: f64 = (v514 + v515);
        let v517: f64 = (v139 * v509);
        let v518: f64 = (v138 * v512);
        let v519: f64 = (v517 + v518);
        let v520: f64 = (v139 * v510);
        let v521: f64 = (v138 * v513);
        let v522: f64 = (v520 + v521);
        let v523: f64 = (if v115 { v516 } else { v508 });
        let v524: f64 = (if v115 { v519 } else { v509 });
        let v525: f64 = (if v115 { v522 } else { v510 });
        let v526: f64 = (v149 * v495);
        let v527: f64 = (v149 * v496);
        let v528: f64 = (v149 * v497);
        let v529: f64 = (v526 / v150);
        let v530: f64 = (v527 / v150);
        let v531: f64 = (v528 / v150);
        let v532: f64 = (if v143 { v495 } else { v13 });
        let v533: f64 = (if v143 { v496 } else { v13 });
        let v534: f64 = (if v143 { v497 } else { v13 });
        let v535: f64 = (if v152 { v526 } else { v532 });
        let v536: f64 = (if v152 { v527 } else { v533 });
        let v537: f64 = (if v152 { v528 } else { v534 });
        let v538: f64 = (if v148 { v529 } else { v535 });
        let v539: f64 = (if v148 { v530 } else { v536 });
        let v540: f64 = (if v148 { v531 } else { v537 });
        let v541: f64 = (v161 * v501);
        let v542: f64 = (v541 / v162);
        let v543: f64 = (if v156 { v501 } else { v13 });
        let v544: f64 = (if v164 { v541 } else { v543 });
        let v545: f64 = (if v160 { v542 } else { v544 });
        let v546: f64 = (v538 - v545);
        let v547: f64 = (if v115 { v546 } else { v13 });
        let v548: f64 = (if v115 { v539 } else { v13 });
        let v549: f64 = (if v115 { v540 } else { v13 });
        let v550: f64 = (v170 * v411);
        let v551: f64 = (v34 * v523);
        let v552: f64 = (v550 + v551);
        let v553: f64 = (v34 * v524);
        let v554: f64 = (v34 * v525);
        let v555: f64 = (v169 * v415);
        let v556: f64 = (v42 * v547);
        let v557: f64 = (v555 + v556);
        let v558: f64 = (v42 * v548);
        let v559: f64 = (v42 * v549);
        let v560: f64 = ((v174) as f64).ln();
        let v561: f64 = (v175 * v560);
        let v562: f64 = (v419 * v561);
        let v563: f64 = (self.scalar_v173 * v562);
        let v564: f64 = (v177 * v557);
        let v565: f64 = (v172 * v563);
        let v566: f64 = (v564 - v565);
        let v567: f64 = (v177 * v177);
        let v568: f64 = (v566 / v567);
        let v569: f64 = (v558 / v177);
        let v570: f64 = (v559 / v177);
        let v571: f64 = (v552 - v568);
        let v572: f64 = (v553 - v569);
        let v573: f64 = (v554 - v570);
        let v574: f64 = (if v115 { v571 } else { v13 });
        let v575: f64 = (if v115 { v572 } else { v13 });
        let v576: f64 = (if v115 { v573 } else { v13 });
        let v577: f64 = (if v181 { v13 } else { v574 });
        let v578: f64 = (if v181 { v13 } else { v575 });
        let v579: f64 = (if v181 { v13 } else { v576 });
        let v653: f64 = (v265 * v577);
        let v654: f64 = (v265 * v578);
        let v655: f64 = (v265 * v579);
        let v659: f64 = (-v463);
        let v660: f64 = (self.scalar_v281 * v659);
        let v661: f64 = (v293 * v463);
        let v662: f64 = (v661 / self.scalar_v295);
        let v663: f64 = (if v284 { v662 } else { v13 });
        let v664: f64 = (self.scalar_v299 * v660);
        let v667: f64 = (v95 * v664);
        let v668: f64 = (v300 * v463);
        let v669: f64 = (v667 - v668);
        let v670: f64 = (v95 * v95);
        let v671: f64 = (v669 / v670);
        let v672: f64 = (self.scalar_v665 / v95);
        let v673: f64 = (self.scalar_v666 / v95);
        let v674: f64 = (v302 * v660);
        let v675: f64 = (v283 * v671);
        let v676: f64 = (v674 + v675);
        let v677: f64 = (self.scalar_v104 * v302);
        let v678: f64 = (v283 * v672);
        let v679: f64 = (v677 + v678);
        let v680: f64 = (v302 * self.scalar_v475);
        let v681: f64 = (v283 * v673);
        let v682: f64 = (v680 + v681);
        let v683: f64 = (v290 * v676);
        let v684: f64 = (v290 * v679);
        let v685: f64 = (v290 * v682);
        let v686: f64 = (if v284 { v683 } else { v13 });
        let v687: f64 = (if v284 { v684 } else { v13 });
        let v688: f64 = (if v284 { v685 } else { v13 });
        let v689: f64 = (v108 * v463);
        let v690: f64 = (-v689);
        let v691: f64 = (v690 / v670);
        let v692: f64 = (self.scalar_v104 / v95);
        let v693: f64 = (self.scalar_v475 / v95);
        let v694: f64 = (-v691);
        let v695: f64 = (-v692);
        let v696: f64 = (-v693);
        let v697: f64 = (v694 / v308);
        let v698: f64 = (v695 / v308);
        let v699: f64 = (v696 / v308);
        let v700: f64 = (self.scalar_v295 * v697);
        let v701: f64 = (self.scalar_v295 * v698);
        let v702: f64 = (self.scalar_v295 * v699);
        let v703: f64 = (v311 * v700);
        let v704: f64 = (v311 * v701);
        let v705: f64 = (v311 * v702);
        let v706: f64 = (-v703);
        let v707: f64 = (-v704);
        let v708: f64 = (-v705);
        let v709: f64 = (v312 * v463);
        let v710: f64 = (v95 * v706);
        let v711: f64 = (v709 + v710);
        let v712: f64 = (v95 * v707);
        let v713: f64 = (v95 * v708);
        let v714: f64 = (v711 / self.scalar_v295);
        let v715: f64 = (v712 / self.scalar_v295);
        let v716: f64 = (v713 / self.scalar_v295);
        let v717: f64 = (if v306 { v714 } else { v663 });
        let v718: f64 = (if v306 { v715 } else { v13 });
        let v719: f64 = (if v306 { v716 } else { v13 });
        let v720: f64 = (if v306 { v13 } else { v686 });
        let v721: f64 = (if v306 { v13 } else { v687 });
        let v722: f64 = (if v306 { v13 } else { v688 });
        let v723: f64 = (v717 + v720);
        let v724: f64 = (v718 + v721);
        let v725: f64 = (v719 + v722);
        let v726: f64 = (v317 * v474);
        let v727: f64 = (v103 * v723);
        let v728: f64 = (v726 + v727);
        let v729: f64 = (v103 * v724);
        let v730: f64 = (v103 * v725);
        let v777: f64 = (self.scalar_v104 * v728);
        let v778: f64 = (self.scalar_v104 * v729);
        let v779: f64 = (self.scalar_v104 * v730);
        let v780: f64 = (self.scalar_v16 * v777);
        let v781: f64 = (self.scalar_v16 * v778);
        let v782: f64 = (self.scalar_v16 * v779);
        let v783: f64 = (self.scalar_v104 * v653);
        let v784: f64 = (self.scalar_v104 * v654);
        let v785: f64 = (self.scalar_v104 * v655);
        let v786: f64 = (self.scalar_v16 * v783);
        let v787: f64 = (self.scalar_v16 * v784);
        let v788: f64 = (self.scalar_v16 * v785);

        let d393_dn2: f64 = v780;
        let d393_dn3: f64 = v781;
        let d393_dn4: f64 = v782;
        stamper.stamp_current_reactive_node3(
            Some(nodes[3]),
            Some(nodes[4]),
            nodes[2],
            multiplicity * (d393_dn2),
            nodes[3],
            multiplicity * (d393_dn3),
            nodes[4],
            multiplicity * (d393_dn4),
        );
        let d395_dn2: f64 = v786;
        let d395_dn3: f64 = v787;
        let d395_dn4: f64 = v788;
        stamper.stamp_current_reactive_node3(
            Some(nodes[3]),
            Some(nodes[4]),
            nodes[2],
            multiplicity * (d395_dn2),
            nodes[3],
            multiplicity * (d395_dn3),
            nodes[4],
            multiplicity * (d395_dn4),
        );
        let mut locals = StampLocals::default();

        let assign630_e796: f64 = ((nv0 - nv1) / p.p40);
        let assign630_e797: f64 = (assign630_e796).abs();
        let assign630_e799: f64 = (assign630_e797).powf(p.p39);
        locals.var_vtff = assign630_e799;
        locals.var_vtff_dn0 = if 0.0 == 0.0 && ((p.p39) as f64).is_finite() && ((p.p39) as f64).fract() == 0.0 { if p.p39 == 0.0 { 0.0 } else { (p.p39 * ((assign630_e797).powf(p.p39 - 1.0) * if assign630_e796 >= 0.0 { (1.0 / p.p40) } else { (-(1.0 / p.p40)) })) } } else { (assign630_e799 * (p.p39 * (if assign630_e796 >= 0.0 { (1.0 / p.p40) } else { (-(1.0 / p.p40)) } / assign630_e797))) };
        locals.var_vtff_dn1 = if 0.0 == 0.0 && ((p.p39) as f64).is_finite() && ((p.p39) as f64).fract() == 0.0 { if p.p39 == 0.0 { 0.0 } else { (p.p39 * ((assign630_e797).powf(p.p39 - 1.0) * if assign630_e796 >= 0.0 { (-1.0 / p.p40) } else { (-(-1.0 / p.p40)) })) } } else { (assign630_e799 * (p.p39 * (if assign630_e796 >= 0.0 { (-1.0 / p.p40) } else { (-(-1.0 / p.p40)) } / assign630_e797))) };
        locals.var_vtff_rv = 0.0;

        let assign640_e802: f64 = (1.0 + locals.var_vtff);
        let assign640_e805: f64 = (1.0 / p.p39);
        let assign640_e806: f64 = (assign640_e802).powf(assign640_e805);
        let assign640_e808: f64 = (assign640_e806 - 1.0);
        locals.var_vtff1 = assign640_e808;
        locals.var_vtff1_dn0 = if 0.0 == 0.0 && ((assign640_e805) as f64).is_finite() && ((assign640_e805) as f64).fract() == 0.0 { if assign640_e805 == 0.0 { 0.0 } else { (assign640_e805 * ((assign640_e802).powf(assign640_e805 - 1.0) * locals.var_vtff_dn0)) } } else { (assign640_e806 * (assign640_e805 * (locals.var_vtff_dn0 / assign640_e802))) };
        locals.var_vtff1_dn1 = if 0.0 == 0.0 && ((assign640_e805) as f64).is_finite() && ((assign640_e805) as f64).fract() == 0.0 { if assign640_e805 == 0.0 { 0.0 } else { (assign640_e805 * ((assign640_e802).powf(assign640_e805 - 1.0) * locals.var_vtff_dn1)) } } else { (assign640_e806 * (assign640_e805 * (locals.var_vtff_dn1 / assign640_e802))) };
        locals.var_vtff1_rv = 0.0;

        let assign650_e813: f64 = (p.p41 * locals.var_vtff1);
        let assign650_e814: f64 = (1.0 + assign650_e813);
        let assign650_e815: f64 = (p.p19 * assign650_e814);
        locals.var_tff = assign650_e815;
        locals.var_tff_dn0 = (p.p19 * (p.p41 * locals.var_vtff1_dn0));
        locals.var_tff_dn1 = (p.p19 * (p.p41 * locals.var_vtff1_dn1));
        locals.var_tff_rv = 0.0;

        let assign670_e821: f64 = if p.p32 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard8 = assign670_e821;
        locals.var_guard8_rv = 0.0;

        let assign790_e940: f64 = if ((p.p30 == 1.0) && (p.p33 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard10 = assign790_e940;
        locals.var_guard10_rv = 0.0;

        let assign800_e951: f64 = if (((p.p30 == 2.0) && (p.p33 > 0.0)) && (p.p35 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard11 = assign800_e951;
        locals.var_guard11_rv = 0.0;

        Self::stamp_reactive_equations_block_0(ctx, stamper, p, nodes, multiplicity, &mut locals);
    }
}
