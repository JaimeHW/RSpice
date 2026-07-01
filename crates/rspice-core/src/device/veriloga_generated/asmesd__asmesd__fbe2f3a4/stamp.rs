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
    pub(crate) var_guard13: f64,
    pub(crate) var_guard13_rv: f64,
    pub(crate) var_guard20: f64,
    pub(crate) var_guard20_rv: f64,
    pub(crate) var_guard21: f64,
    pub(crate) var_guard21_rv: f64,
    pub(crate) var_tff: f64,
    pub(crate) var_tff_dn1: f64,
    pub(crate) var_tff_dn2: f64,
    pub(crate) var_tff_rv: f64,
    pub(crate) var_vtff: f64,
    pub(crate) var_vtff1: f64,
    pub(crate) var_vtff1_dn1: f64,
    pub(crate) var_vtff1_dn2: f64,
    pub(crate) var_vtff1_rv: f64,
    pub(crate) var_vtff_dn1: f64,
    pub(crate) var_vtff_dn2: f64,
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
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv9 = ctx.node_voltage(nodes[9]);
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
        let v1: f64 = nv3;
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
        let v18: f64 = nv5;
        let v19: f64 = nv4;
        let v20: f64 = (v18 - v19);
        let v21: f64 = (self.scalar_v17 * v20);
        let v23: bool = (v21 < v13);
        let v24: f64 = (if v23 { v21 } else { v13 });
        let v25: f64 = (-v24);
        let v27: f64 = f64::powf(v25, self.scalar_v26);
        let v28: f64 = (self.scalar_v22 * v27);
        let v29: f64 = (v12 + v28);
        let v32: f64 = 8.6170869e-5;
        let v33: f64 = (v11 * v32);
        let v34: f64 = (v11 / self.scalar_v31);
        let v35: f64 = ((v34) as f64).ln();
        let v37: f64 = (v35 * self.scalar_v36);
        let v38: f64 = ((v37) as f64).exp();
        let v40: f64 = (v38 * self.scalar_v39);
        let v41: f64 = (v29 * v40);
        let v43: f64 = (v38 * self.scalar_v42);
        let v61: f64 = (v35 * self.scalar_v60);
        let v63: f64 = (v34 - v12);
        let v64: f64 = (self.scalar_v62 * v63);
        let v65: f64 = (v64 / v33);
        let v66: f64 = (v61 + v65);
        let v68: f64 = (v35 * self.scalar_v67);
        let v70: f64 = ((v66) as f64).exp();
        let v71: f64 = (self.scalar_v69 * v70);
        let v73: f64 = ((v68) as f64).exp();
        let v74: f64 = (self.scalar_v72 * v73);
        let v77: f64 = (v66 / self.scalar_v76);
        let v78: f64 = ((v77) as f64).exp();
        let v79: f64 = (self.scalar_v75 * v78);
        let v80: f64 = (v79 / v38);
        let v83: f64 = (v66 / self.scalar_v82);
        let v84: f64 = ((v83) as f64).exp();
        let v85: f64 = (self.scalar_v81 * v84);
        let v86: f64 = (v85 / v38);
        let v89: f64 = (v63 * self.scalar_v88);
        let v90: f64 = (v12 + v89);
        let v91: f64 = (self.scalar_v87 * v90);
        let v94: f64 = (v63 * self.scalar_v93);
        let v95: f64 = (v12 + v94);
        let v96: f64 = (self.scalar_v92 * v95);
        let v99: f64 = (v63 * self.scalar_v98);
        let v100: f64 = (v12 + v99);
        let v101: f64 = (self.scalar_v97 * v100);
        let v104: f64 = (v63 * self.scalar_v103);
        let v105: f64 = (v12 + v104);
        let v106: f64 = (self.scalar_v102 * v105);
        let v110: f64 = 300.15;
        let v112: f64 = (v11 / v110);
        let v113: f64 = 1.16;
        let v114: f64 = 0.000702;
        let v115: f64 = (v11 * v114);
        let v116: f64 = (v11 * v115);
        let v117: f64 = 1108.0;
        let v118: f64 = (v11 + v117);
        let v119: f64 = (v116 / v118);
        let v120: f64 = (v113 - v119);
        let v121: f64 = (-v120);
        let v122: f64 = 1.3806226e-23;
        let v123: f64 = (v11 + v11);
        let v124: f64 = (v122 * v123);
        let v125: f64 = (v121 / v124);
        let v126: f64 = 1.3454442398941469e20;
        let v127: f64 = (v125 + v126);
        let v128: f64 = (v33 + v33);
        let v129: f64 = (-v128);
        let v130: f64 = 1.5;
        let v131: f64 = ((v112) as f64).ln();
        let v132: f64 = (v130 * v131);
        let v133: f64 = 1.6021918e-19;
        let v134: f64 = (v127 * v133);
        let v135: f64 = (v132 + v134);
        let v136: f64 = (v129 * v135);
        let v138: f64 = (self.scalar_v137 - v136);
        let v139: f64 = (v138 / self.scalar_v111);
        let v140: f64 = (self.scalar_v137 - v139);
        let v141: f64 = (v140 / v139);
        let v143: f64 = 0.0004;
        let v146: f64 = (self.scalar_v145 - v141);
        let v147: f64 = (self.scalar_v142 * v146);
        let v148: f64 = (v12 + v147);
        let v149: f64 = (self.scalar_v107 / v148);
        let v150: f64 = (v112 * v139);
        let v151: f64 = (v136 + v150);
        let v152: f64 = (v151 - v139);
        let v153: f64 = (v152 / v139);
        let v154: f64 = (v11 - v110);
        let v155: f64 = (v143 * v154);
        let v156: f64 = (v155 - v153);
        let v157: f64 = (self.scalar_v142 * v156);
        let v158: f64 = (v12 + v157);
        let v159: f64 = (v149 * v158);
        let v161: f64 = (self.scalar_v160 - v136);
        let v162: f64 = (v161 / self.scalar_v111);
        let v163: f64 = (self.scalar_v160 - v162);
        let v164: f64 = (v163 / v162);
        let v166: f64 = (self.scalar_v145 - v164);
        let v167: f64 = (self.scalar_v165 * v166);
        let v168: f64 = (v12 + v167);
        let v169: f64 = (self.scalar_v108 / v168);
        let v170: f64 = (v112 * v162);
        let v171: f64 = (v136 + v170);
        let v172: f64 = (v171 - v162);
        let v173: f64 = (v172 / v162);
        let v174: f64 = (v155 - v173);
        let v175: f64 = (self.scalar_v165 * v174);
        let v176: f64 = (v12 + v175);
        let v177: f64 = (v169 * v176);
        let v179: f64 = (self.scalar_v178 - v136);
        let v180: f64 = (v179 / self.scalar_v111);
        let v181: f64 = (self.scalar_v178 - v180);
        let v182: f64 = (v181 / v180);
        let v184: f64 = (self.scalar_v145 - v182);
        let v185: f64 = (self.scalar_v183 * v184);
        let v186: f64 = (v12 + v185);
        let v187: f64 = (self.scalar_v109 / v186);
        let v188: f64 = (v112 * v180);
        let v189: f64 = (v136 + v188);
        let v190: f64 = (v189 - v180);
        let v191: f64 = (v190 / v180);
        let v192: f64 = (v155 - v191);
        let v193: f64 = (self.scalar_v183 * v192);
        let v194: f64 = (v12 + v193);
        let v195: f64 = (v187 * v194);
        let v196: f64 = nv2;
        let v197: f64 = (v196 - v19);
        let v198: f64 = (self.scalar_v17 * v197);
        let v199: f64 = nv6;
        let v200: f64 = (v18 - v199);
        let v201: f64 = (self.scalar_v17 * v200);
        let v202: f64 = nv1;
        let v203: f64 = (v202 - v19);
        let v204: f64 = (self.scalar_v17 * v203);
        let v205: f64 = (v202 - v18);
        let v206: f64 = (self.scalar_v17 * v205);
        let v207: f64 = (v196 - v199);
        let v208: f64 = (self.scalar_v17 * v207);
        let v209: bool = (v71 > v13);
        let v211: f64 = (v33 * self.scalar_v210);
        let v212: f64 = (v201 / v211);
        let v213: f64 = (if v209 { v212 } else { v13 });
        let v214: f64 = (-v201);
        let v215: f64 = (v214 - v96);
        let v217: f64 = (v33 * self.scalar_v216);
        let v218: f64 = (v215 / v217);
        let v219: f64 = (if v209 { v218 } else { v13 });
        let v220: f64 = (-v96);
        let v221: f64 = (v220 / v217);
        let v222: f64 = (if v209 { v221 } else { v13 });
        let v223: f64 = 80.0;
        let v224: bool = (v213 > v223);
        let v225: bool = (v209 && v224);
        let v226: f64 = (v213 - v223);
        let v227: f64 = (v12 + v226);
        let v228: f64 = (if v225 { v227 } else { v13 });
        let v229: f64 = (if v225 { v223 } else { v213 });
        let v230: bool = (!v224);
        let v231: bool = (v209 && v230);
        let v232: f64 = (if v231 { v12 } else { v228 });
        let v233: f64 = ((v229) as f64).exp();
        let v234: f64 = (v232 * v233);
        let v235: f64 = (if v209 { v234 } else { v232 });
        let v236: f64 = 37.0;
        let v237: bool = (v219 >= v236);
        let v238: bool = (!v237);
        let v239: f64 = -37.0;
        let v240: bool = (v219 <= v239);
        let v241: bool = (!v240);
        let v242: bool = (v238 && v241);
        let v243: f64 = ((v219) as f64).exp();
        let v244: f64 = (v12 + v243);
        let v245: f64 = ((v244) as f64).ln();
        let v246: bool = (v238 && v240);
        let v247: f64 = (if v237 { v219 } else { v13 });
        let v248: f64 = (if v246 { v243 } else { v247 });
        let v249: f64 = (if v242 { v245 } else { v248 });
        let v250: bool = (v222 >= v236);
        let v251: bool = (!v250);
        let v252: bool = (v222 <= v239);
        let v253: bool = (!v252);
        let v254: bool = (v251 && v253);
        let v255: f64 = ((v222) as f64).exp();
        let v256: f64 = (v12 + v255);
        let v257: f64 = ((v256) as f64).ln();
        let v258: bool = (v251 && v252);
        let v259: f64 = (if v250 { v222 } else { v13 });
        let v260: f64 = (if v258 { v255 } else { v259 });
        let v261: f64 = (if v254 { v257 } else { v260 });
        let v262: f64 = (v249 - v261);
        let v263: f64 = (if v209 { v262 } else { v13 });
        let v264: f64 = (v235 - v12);
        let v265: f64 = (v71 * v264);
        let v266: f64 = (v91 * v263);
        let v268: f64 = ((v201) as f64).abs();
        let v269: f64 = f64::powf(v268, v101);
        let v270: f64 = (self.scalar_v267 * v269);
        let v271: f64 = (v12 + v270);
        let v272: f64 = (v266 / v271);
        let v273: f64 = (v265 - v272);
        let v274: f64 = (if v209 { v273 } else { v13 });
        let v275: bool = (!v209);
        let v276: f64 = (if v275 { v13 } else { v274 });
        let v277: bool = (v74 > v13);
        let v279: f64 = (self.scalar_v278 - v201);
        let v280: f64 = 0.001;
        let v281: bool = (v279 > v280);
        let v282: f64 = (if v281 { v279 } else { v280 });
        let v283: f64 = (if v277 { v282 } else { v13 });
        let v284: f64 = -1.0;
        let v285: f64 = (v214 * self.scalar_v278);
        let v287: f64 = (v33 * self.scalar_v286);
        let v288: f64 = (v283 * v287);
        let v289: f64 = (v285 / v288);
        let v290: f64 = (if v277 { v289 } else { v229 });
        let v291: bool = (v290 > v223);
        let v292: bool = (v277 && v291);
        let v293: f64 = (v290 - v223);
        let v294: f64 = (v12 + v293);
        let v295: f64 = (if v292 { v294 } else { v235 });
        let v296: f64 = (if v292 { v223 } else { v290 });
        let v297: bool = (!v291);
        let v298: bool = (v277 && v297);
        let v299: f64 = (if v298 { v12 } else { v295 });
        let v300: f64 = ((v296) as f64).exp();
        let v301: f64 = (v299 * v300);
        let v302: f64 = (if v277 { v301 } else { v299 });
        let v303: f64 = (v302 - v12);
        let v304: f64 = (v74 * v303);
        let v305: f64 = (if v277 { v304 } else { v13 });
        let v306: bool = (!v277);
        let v307: f64 = (if v306 { v13 } else { v305 });
        let v308: bool = (v80 > v13);
        let v309: f64 = (v33 * self.scalar_v76);
        let v310: f64 = (v201 / v309);
        let v311: f64 = (if v308 { v310 } else { v296 });
        let v313: f64 = (v33 * self.scalar_v312);
        let v314: f64 = (v215 / v313);
        let v315: f64 = (if v308 { v314 } else { v219 });
        let v316: f64 = (v220 / v313);
        let v317: f64 = (if v308 { v316 } else { v222 });
        let v318: bool = (v311 > v223);
        let v319: bool = (v308 && v318);
        let v320: f64 = (v311 - v223);
        let v321: f64 = (v12 + v320);
        let v322: f64 = (if v319 { v321 } else { v302 });
        let v323: f64 = (if v319 { v223 } else { v311 });
        let v324: bool = (!v318);
        let v325: bool = (v308 && v324);
        let v326: f64 = (if v325 { v12 } else { v322 });
        let v327: f64 = ((v323) as f64).exp();
        let v328: f64 = (v326 * v327);
        let v329: f64 = (if v308 { v328 } else { v326 });
        let v330: bool = (v315 >= v236);
        let v331: bool = (!v330);
        let v332: bool = (v315 <= v239);
        let v333: bool = (!v332);
        let v334: bool = (v331 && v333);
        let v335: f64 = ((v315) as f64).exp();
        let v336: f64 = (v12 + v335);
        let v337: f64 = ((v336) as f64).ln();
        let v338: bool = (v331 && v332);
        let v339: f64 = (if v330 { v315 } else { v13 });
        let v340: f64 = (if v338 { v335 } else { v339 });
        let v341: f64 = (if v334 { v337 } else { v340 });
        let v342: bool = (v317 >= v236);
        let v343: bool = (!v342);
        let v344: bool = (v317 <= v239);
        let v345: bool = (!v344);
        let v346: bool = (v343 && v345);
        let v347: f64 = ((v317) as f64).exp();
        let v348: f64 = (v12 + v347);
        let v349: f64 = ((v348) as f64).ln();
        let v350: bool = (v343 && v344);
        let v351: f64 = (if v342 { v317 } else { v13 });
        let v352: f64 = (if v350 { v347 } else { v351 });
        let v353: f64 = (if v346 { v349 } else { v352 });
        let v354: f64 = (v341 - v353);
        let v355: f64 = (if v308 { v354 } else { v263 });
        let v356: f64 = (v329 - v12);
        let v357: f64 = (v80 * v356);
        let v358: f64 = (v13 * v355);
        let v359: f64 = (v358 / v271);
        let v360: f64 = (v357 - v359);
        let v361: f64 = (if v308 { v360 } else { v13 });
        let v362: bool = (!v308);
        let v363: f64 = (if v362 { v13 } else { v361 });
        let v365: f64 = (v33 * self.scalar_v364);
        let v366: f64 = (v21 / v365);
        let v367: f64 = (if v209 { v366 } else { v323 });
        let v368: f64 = (-v21);
        let v369: f64 = (v368 - v96);
        let v370: f64 = (v369 / v313);
        let v371: f64 = (if v209 { v370 } else { v315 });
        let v372: f64 = (if v209 { v316 } else { v317 });
        let v373: bool = (v367 > v223);
        let v374: bool = (v209 && v373);
        let v375: f64 = (v367 - v223);
        let v376: f64 = (v12 + v375);
        let v377: f64 = (if v374 { v376 } else { v329 });
        let v378: f64 = (if v374 { v223 } else { v367 });
        let v379: bool = (!v373);
        let v380: bool = (v209 && v379);
        let v381: f64 = (if v380 { v12 } else { v377 });
        let v382: f64 = ((v378) as f64).exp();
        let v383: f64 = (v381 * v382);
        let v384: f64 = (if v209 { v383 } else { v381 });
        let v385: bool = (v371 >= v236);
        let v386: bool = (!v385);
        let v387: bool = (v371 <= v239);
        let v388: bool = (!v387);
        let v389: bool = (v386 && v388);
        let v390: f64 = ((v371) as f64).exp();
        let v391: f64 = (v12 + v390);
        let v392: f64 = ((v391) as f64).ln();
        let v393: bool = (v386 && v387);
        let v394: f64 = (if v385 { v371 } else { v13 });
        let v395: f64 = (if v393 { v390 } else { v394 });
        let v396: f64 = (if v389 { v392 } else { v395 });
        let v397: bool = (v372 >= v236);
        let v398: bool = (!v397);
        let v399: bool = (v372 <= v239);
        let v400: bool = (!v399);
        let v401: bool = (v398 && v400);
        let v402: f64 = ((v372) as f64).exp();
        let v403: f64 = (v12 + v402);
        let v404: f64 = ((v403) as f64).ln();
        let v405: bool = (v398 && v399);
        let v406: f64 = (if v397 { v372 } else { v13 });
        let v407: f64 = (if v405 { v402 } else { v406 });
        let v408: f64 = (if v401 { v404 } else { v407 });
        let v409: f64 = (v396 - v408);
        let v410: f64 = (if v209 { v409 } else { v355 });
        let v411: f64 = (v384 - v12);
        let v412: f64 = (v71 * v411);
        let v413: f64 = (v106 * v410);
        let v414: f64 = ((v21) as f64).abs();
        let v415: f64 = f64::powf(v414, v101);
        let v416: f64 = (self.scalar_v267 * v415);
        let v417: f64 = (v12 + v416);
        let v418: f64 = (v413 / v417);
        let v419: f64 = (v412 - v418);
        let v420: f64 = (if v209 { v419 } else { v13 });
        let v421: f64 = (if v275 { v13 } else { v420 });
        let v422: bool = (v86 > v13);
        let v423: f64 = (v33 * self.scalar_v82);
        let v424: f64 = (v21 / v423);
        let v425: f64 = (if v422 { v424 } else { v378 });
        let v426: f64 = (if v422 { v370 } else { v371 });
        let v427: f64 = (if v422 { v316 } else { v372 });
        let v428: bool = (v425 > v223);
        let v429: bool = (v422 && v428);
        let v430: f64 = (v425 - v223);
        let v431: f64 = (v12 + v430);
        let v432: f64 = (if v429 { v431 } else { v384 });
        let v433: f64 = (if v429 { v223 } else { v425 });
        let v434: bool = (!v428);
        let v435: bool = (v422 && v434);
        let v436: f64 = (if v435 { v12 } else { v432 });
        let v437: f64 = ((v433) as f64).exp();
        let v438: f64 = (v436 * v437);
        let v439: f64 = (if v422 { v438 } else { v436 });
        let v440: bool = (v426 >= v236);
        let v441: bool = (!v440);
        let v442: bool = (v426 <= v239);
        let v443: bool = (!v442);
        let v444: bool = (v441 && v443);
        let v445: f64 = ((v426) as f64).exp();
        let v446: f64 = (v12 + v445);
        let v447: f64 = ((v446) as f64).ln();
        let v448: bool = (v441 && v442);
        let v449: f64 = (if v440 { v426 } else { v13 });
        let v450: f64 = (if v448 { v445 } else { v449 });
        let v451: f64 = (if v444 { v447 } else { v450 });
        let v452: bool = (v427 >= v236);
        let v453: bool = (!v452);
        let v454: bool = (v427 <= v239);
        let v455: bool = (!v454);
        let v456: bool = (v453 && v455);
        let v457: f64 = ((v427) as f64).exp();
        let v458: f64 = (v12 + v457);
        let v459: f64 = ((v458) as f64).ln();
        let v460: bool = (v453 && v454);
        let v461: f64 = (if v452 { v427 } else { v13 });
        let v462: f64 = (if v460 { v457 } else { v461 });
        let v463: f64 = (if v456 { v459 } else { v462 });
        let v464: f64 = (v451 - v463);
        let v465: f64 = (if v422 { v464 } else { v410 });
        let v466: f64 = (v439 - v12);
        let v467: f64 = (v86 * v466);
        let v468: f64 = (v13 * v465);
        let v469: f64 = f64::powf(v414, self.scalar_v97);
        let v470: f64 = (self.scalar_v267 * v469);
        let v471: f64 = (v12 + v470);
        let v472: f64 = (v468 / v471);
        let v473: f64 = (v467 - v472);
        let v474: f64 = (if v422 { v473 } else { v13 });
        let v475: bool = (!v422);
        let v476: f64 = (if v475 { v13 } else { v474 });
        let v477: f64 = nv9;
        let v478: bool = (v477 < v201);
        let v479: f64 = (if v478 { v477 } else { v201 });
        let v480: f64 = 1e-9;
        let v481: bool = (v268 > v480);
        let v482: f64 = (if v481 { v268 } else { v480 });
        let v483: f64 = (v479 / v482);
        let v484: f64 = ((v483) as f64).abs();
        let v485: f64 = (v276 - v307);
        let v486: f64 = (v485 / v41);
        let v487: f64 = (v363 + v486);
        let v488: f64 = (v421 / v43);
        let v489: f64 = (v476 + v488);
        let v491: f64 = (v21 * self.scalar_v490);
        let v492: f64 = (v12 + v491);
        let v493: f64 = (self.scalar_v55 * v492);
        let v494: f64 = (v276 * v493);
        let v495: f64 = (self.scalar_v59 * v421);
        let v496: f64 = (v494 + v495);
        let v497: f64 = (self.scalar_v51 * v201);
        let v498: f64 = (v12 - v497);
        let v499: f64 = (v21 * self.scalar_v47);
        let v500: f64 = (v498 - v499);
        let v501: f64 = 4.0;
        let v502: f64 = (v496 * v501);
        let v503: f64 = (v12 + v502);
        let v504: f64 = ((v503) as f64).abs();
        let v506: f64 = f64::powf(v504, self.scalar_v505);
        let v507: f64 = (v12 + v506);
        let v508: f64 = 2.0;
        let v509: f64 = (v500 * v508);
        let v510: f64 = (v509 / v507);
        let v511: f64 = (v421 * v510);
        let v512: f64 = (v276 * v510);
        let v513: f64 = (v484 * v512);
        let v515: f64 = (v513 * self.scalar_v514);
        let v517: f64 = (v276 * self.scalar_v516);
        let v518: f64 = (v510 * v517);
        let v519: f64 = (v515 + v518);
        let v521: f64 = (v206 / self.scalar_v520);
        let v522: f64 = ((v521) as f64).abs();
        let v524: f64 = f64::powf(v522, self.scalar_v523);
        let v525: f64 = (v12 + v524);
        let v527: f64 = (v208 / self.scalar_v526);
        let v528: f64 = ((v527) as f64).abs();
        let v530: f64 = f64::powf(v528, self.scalar_v529);
        let v531: f64 = (v12 + v530);
        let v534: f64 = (v35 * self.scalar_v533);
        let v535: f64 = ((v534) as f64).exp();
        let v536: f64 = (self.scalar_v532 * v535);
        let v538: f64 = f64::powf(v525, self.scalar_v537);
        let v539: f64 = (v536 * v538);
        let v542: f64 = (v35 * self.scalar_v541);
        let v543: f64 = ((v542) as f64).exp();
        let v544: f64 = (self.scalar_v540 * v543);
        let v547: f64 = (v35 * self.scalar_v546);
        let v548: f64 = ((v547) as f64).exp();
        let v549: f64 = (self.scalar_v545 * v548);
        let v551: f64 = f64::powf(v531, self.scalar_v550);
        let v552: f64 = (v549 * v551);
        let v553: f64 = (v202 - v196);
        let v555: f64 = (v553 / self.scalar_v554);
        let v556: f64 = ((v555) as f64).abs();
        let v558: f64 = f64::powf(v556, self.scalar_v557);
        let v559: f64 = (v12 + v558);
        let v561: f64 = f64::powf(v559, self.scalar_v560);
        let v562: f64 = (v561 - v12);
        let v565: f64 = (v562 * self.scalar_v564);
        let v566: f64 = (v12 + v565);
        let v567: f64 = (self.scalar_v563 * v566);
        let v568: f64 = (v276 * v567);
        let v570: f64 = (v511 * self.scalar_v569);
        let v573: f64 = nv8;
        let v574: f64 = ((v573) as f64).abs();
        let v576: f64 = (v574 / self.scalar_v575);
        let v578: f64 = f64::powf(v576, self.scalar_v577);
        let v579: f64 = (v12 + v578);
        let v580: f64 = (v539 / v579);
        let v581: f64 = (if self.scalar_v572 { v580 } else { v539 });
        let v583: f64 = (if self.scalar_v582 { v581 } else { v581 });
        let v587: f64 = (v583 + self.scalar_v586);
        let v588: f64 = (if self.scalar_v585 { v587 } else { v583 });
        let v590: f64 = (v544 + self.scalar_v589);
        let v591: f64 = (if self.scalar_v585 { v590 } else { v544 });
        let v593: f64 = (v552 + self.scalar_v592);
        let v594: f64 = (if self.scalar_v585 { v593 } else { v552 });
        let v595: bool = (v198 <= v13);
        let v596: f64 = (v189 * v195);
        let v598: f64 = (v198 / v189);
        let v599: f64 = (v12 - v598);
        let v600: f64 = ((v599) as f64).ln();
        let v601: f64 = (self.scalar_v597 * v600);
        let v602: f64 = ((v601) as f64).exp();
        let v603: f64 = (v12 - v602);
        let v604: f64 = (v596 * v603);
        let v605: f64 = (v604 / self.scalar_v597);
        let v606: f64 = (if v595 { v605 } else { v13 });
        let v607: bool = (!v595);
        let v608: f64 = (v195 * v198);
        let v611: f64 = (v198 * self.scalar_v610);
        let v612: f64 = (v611 / v189);
        let v613: f64 = (v12 + v612);
        let v614: f64 = (v608 * v613);
        let v615: f64 = (if v607 { v614 } else { v606 });
        let v616: f64 = (-v151);
        let v618: f64 = (v616 * self.scalar_v617);
        let v619: f64 = (v201 + v618);
        let v620: bool = (v619 > v13);
        let v626: f64 = (if v620 { self.scalar_v625 } else { v13 });
        let v627: f64 = (self.scalar_v622 * v626);
        let v628: f64 = (self.scalar_v622 * v627);
        let v629: f64 = (v12 - v628);
        let v630: f64 = (v151 * v629);
        let v632: f64 = (v630 / self.scalar_v631);
        let v633: f64 = (if v620 { v632 } else { v13 });
        let v635: f64 = (v619 * self.scalar_v634);
        let v636: f64 = (v635 / v151);
        let v637: f64 = (self.scalar_v622 + v636);
        let v638: f64 = (v619 * v637);
        let v639: f64 = (v626 * v638);
        let v640: f64 = (if v620 { v639 } else { v13 });
        let v641: bool = (!v620);
        let v642: f64 = (v201 / v151);
        let v643: f64 = (v12 - v642);
        let v644: f64 = ((v643) as f64).ln();
        let v645: f64 = (self.scalar_v631 * v644);
        let v646: f64 = ((v645) as f64).exp();
        let v647: f64 = (v12 - v646);
        let v648: f64 = (v151 * v647);
        let v649: f64 = (v648 / self.scalar_v631);
        let v650: f64 = (if v641 { v649 } else { v633 });
        let v651: f64 = (if v641 { v13 } else { v640 });
        let v652: f64 = (v650 + v651);
        let v653: f64 = (v159 * v652);
        let v654: f64 = (-v171);
        let v655: f64 = (self.scalar_v617 * v654);
        let v656: f64 = (v204 + v655);
        let v657: bool = (v656 > v13);
        let v661: f64 = (if v657 { self.scalar_v660 } else { v626 });
        let v662: f64 = (self.scalar_v622 * v661);
        let v663: f64 = (self.scalar_v622 * v662);
        let v664: f64 = (v12 - v663);
        let v665: f64 = (v171 * v664);
        let v667: f64 = (v665 / self.scalar_v666);
        let v668: f64 = (if v657 { v667 } else { v650 });
        let v670: f64 = (v656 * self.scalar_v669);
        let v671: f64 = (v670 / v171);
        let v672: f64 = (self.scalar_v622 + v671);
        let v673: f64 = (v656 * v672);
        let v674: f64 = (v661 * v673);
        let v675: f64 = (if v657 { v674 } else { v651 });
        let v676: bool = (!v657);
        let v677: f64 = (v204 / v171);
        let v678: f64 = (v12 - v677);
        let v679: f64 = ((v678) as f64).ln();
        let v680: f64 = (self.scalar_v666 * v679);
        let v681: f64 = ((v680) as f64).exp();
        let v682: f64 = (v12 - v681);
        let v683: f64 = (v171 * v682);
        let v684: f64 = (v683 / self.scalar_v666);
        let v685: f64 = (if v676 { v684 } else { v668 });
        let v686: f64 = (if v676 { v13 } else { v675 });
        let v687: f64 = (v685 + v686);
        let v688: f64 = (v177 * v687);
        let v691: f64 = (v688 * self.scalar_v690);
        let v692: f64 = (v21 + v655);
        let v693: bool = (v692 > v13);
        let v694: f64 = (if v693 { self.scalar_v660 } else { v661 });
        let v695: f64 = (self.scalar_v622 * v694);
        let v696: f64 = (self.scalar_v622 * v695);
        let v697: f64 = (v12 - v696);
        let v698: f64 = (v171 * v697);
        let v699: f64 = (v698 / self.scalar_v666);
        let v700: f64 = (if v693 { v699 } else { v685 });
        let v701: f64 = (self.scalar_v669 * v692);
        let v702: f64 = (v701 / v171);
        let v703: f64 = (self.scalar_v622 + v702);
        let v704: f64 = (v692 * v703);
        let v705: f64 = (v694 * v704);
        let v706: f64 = (if v693 { v705 } else { v686 });
        let v707: bool = (!v693);
        let v708: f64 = (v21 / v171);
        let v709: f64 = (v12 - v708);
        let v710: f64 = ((v709) as f64).ln();
        let v711: f64 = (self.scalar_v666 * v710);
        let v712: f64 = ((v711) as f64).exp();
        let v713: f64 = (v12 - v712);
        let v714: f64 = (v171 * v713);
        let v715: f64 = (v714 / self.scalar_v666);
        let v716: f64 = (if v707 { v715 } else { v700 });
        let v717: f64 = (if v707 { v13 } else { v706 });
        let v718: f64 = (v716 + v717);
        let v719: f64 = (v177 * v718);
        let v720: f64 = (self.scalar_v689 * v719);
        let v731: f64 = (v512 * self.scalar_v730);
        let v732: f64 = (if self.scalar_v724 { v731 } else { v13 });
        let v734: f64 = (if self.scalar_v733 { v13 } else { v732 });
        let v759: f64 = (v588 / self.scalar_v16);
        let v763: f64 = (v594 / self.scalar_v16);
        let v767: f64 = (v591 / self.scalar_v16);
        let v768: f64 = (v201 - v477);
        let v769: f64 = (-v768);
        let v770: f64 = 1e-6;
        let v771: f64 = (v477 * v770);
        let v772: f64 = (v276 / v41);
        let v773: f64 = (-v772);
        let v774: f64 = (v567 * v773);
        let v775: f64 = (if self.scalar_v572 { v774 } else { v13 });
        let v776: f64 = (if self.scalar_v572 { v573 } else { v13 });
        let v778: f64 = (v487 * v553);
        let v779: f64 = ((v778) as f64).abs();
        let v780: f64 = (-v779);
        let v781: f64 = nv0;
        let v782: f64 = (v202 - v781);
        let v783: f64 = (v489 * v782);
        let v784: f64 = ((v783) as f64).abs();
        let v785: f64 = (v780 - v784);
        let v786: f64 = (if self.scalar_v739 { v785 } else { v13 });
        let v787: f64 = (v1 / self.scalar_v737);
        let v788: f64 = (if self.scalar_v739 { v787 } else { v13 });
        let v792: f64 = (if self.scalar_v791 { v785 } else { v13 });
        let v793: f64 = nv7;
        let v794: f64 = (v1 - v793);
        let v795: f64 = (v794 / self.scalar_v737);
        let v796: f64 = (if self.scalar_v791 { v795 } else { v13 });
        let v797: f64 = (v793 / self.scalar_v742);
        let v798: f64 = (if self.scalar_v791 { v797 } else { v13 });
        let v802: f64 = (if self.scalar_v801 { v785 } else { v13 });
        let v807: f64 = (v13 * v200);
        let v808: f64 = (v13 * v20);
        let v809: f64 = (v19 - v199);
        let v810: f64 = (v13 * v809);
        let v811: bool = (v759 > self.scalar_v756);
        let v812: f64 = (if v811 { v759 } else { self.scalar_v756 });
        let v813: f64 = (v205 / v812);
        let v814: f64 = (if self.scalar_v758 { v813 } else { v13 });
        let v818: bool = (v763 > self.scalar_v756);
        let v819: f64 = (if v818 { v763 } else { self.scalar_v756 });
        let v820: f64 = (v207 / v819);
        let v821: f64 = (if self.scalar_v762 { v820 } else { v13 });
        let v825: f64 = (v781 - v19);
        let v826: bool = (v767 > self.scalar_v756);
        let v827: f64 = (if v826 { v767 } else { self.scalar_v756 });
        let v828: f64 = (v825 / v827);
        let v829: f64 = (if self.scalar_v766 { v828 } else { v13 });
        let v833: f64 = (self.scalar_v17 * v487);
        let v834: f64 = (self.scalar_v16 * v833);
        let v835: f64 = (self.scalar_v17 * v489);
        let v836: f64 = (self.scalar_v16 * v835);
        let v837: f64 = (-v511);
        let v838: f64 = (self.scalar_v16 * v837);
        let v839: f64 = (self.scalar_v17 * v838);
        let v840: f64 = (self.scalar_v17 * v519);
        let v841: f64 = (self.scalar_v16 * v840);
        let v842: f64 = (self.scalar_v17 * v653);
        let v843: f64 = (self.scalar_v16 * v842);
        let v844: f64 = (self.scalar_v17 * v568);
        let v845: f64 = (self.scalar_v16 * v844);
        let v846: f64 = (self.scalar_v17 * v691);
        let v847: f64 = (self.scalar_v16 * v846);
        let v848: f64 = (self.scalar_v17 * v720);
        let v849: f64 = (self.scalar_v16 * v848);
        let v850: f64 = (self.scalar_v17 * v570);
        let v851: f64 = (self.scalar_v16 * v850);
        let v852: f64 = (self.scalar_v17 * v615);
        let v853: f64 = (self.scalar_v16 * v852);
        let v854: f64 = (-v734);
        let v855: f64 = (self.scalar_v16 * v854);
        let v856: f64 = (self.scalar_v16 * v734);
        let v857: f64 = (if v8 { v12 } else { v13 });
        let v858: f64 = (if v10 { v13 } else { v857 });
        let v860: f64 = (if v23 { self.scalar_v859 } else { v13 });
        let v861: f64 = (if v23 { self.scalar_v17 } else { v13 });
        let v862: f64 = (-v860);
        let v863: f64 = (-v861);
        let v865: f64 = f64::powf(v25, self.scalar_v864);
        let v866: f64 = (self.scalar_v26 * v865);
        let v867: f64 = (v862 * v866);
        let v868: f64 = (v863 * v866);
        let v869: f64 = (self.scalar_v22 * v867);
        let v870: f64 = (self.scalar_v22 * v868);
        let v871: f64 = (v32 * v858);
        let v872: f64 = (v858 / self.scalar_v31);
        let v873: f64 = (v872 / v34);
        let v874: f64 = (self.scalar_v36 * v873);
        let v875: f64 = (v38 * v874);
        let v876: f64 = (self.scalar_v39 * v875);
        let v877: f64 = (v29 * v876);
        let v878: f64 = (v40 * v869);
        let v879: f64 = (v40 * v870);
        let v880: f64 = (self.scalar_v42 * v875);
        let v881: f64 = (self.scalar_v60 * v873);
        let v882: f64 = (self.scalar_v62 * v872);
        let v883: f64 = (v33 * v882);
        let v884: f64 = (v64 * v871);
        let v885: f64 = (v883 - v884);
        let v886: f64 = (v33 * v33);
        let v887: f64 = (v885 / v886);
        let v888: f64 = (v881 + v887);
        let v889: f64 = (self.scalar_v67 * v873);
        let v890: f64 = (v70 * v888);
        let v891: f64 = (self.scalar_v69 * v890);
        let v892: f64 = (v73 * v889);
        let v893: f64 = (self.scalar_v72 * v892);
        let v894: f64 = (v888 / self.scalar_v76);
        let v895: f64 = (v78 * v894);
        let v896: f64 = (self.scalar_v75 * v895);
        let v897: f64 = (v38 * v896);
        let v898: f64 = (v79 * v875);
        let v899: f64 = (v897 - v898);
        let v900: f64 = (v38 * v38);
        let v901: f64 = (v899 / v900);
        let v902: f64 = (v888 / self.scalar_v82);
        let v903: f64 = (v84 * v902);
        let v904: f64 = (self.scalar_v81 * v903);
        let v905: f64 = (v38 * v904);
        let v906: f64 = (v85 * v875);
        let v907: f64 = (v905 - v906);
        let v908: f64 = (v907 / v900);
        let v909: f64 = (self.scalar_v88 * v872);
        let v910: f64 = (self.scalar_v87 * v909);
        let v911: f64 = (self.scalar_v93 * v872);
        let v912: f64 = (self.scalar_v92 * v911);
        let v913: f64 = (self.scalar_v98 * v872);
        let v914: f64 = (self.scalar_v97 * v913);
        let v915: f64 = (self.scalar_v103 * v872);
        let v916: f64 = (self.scalar_v102 * v915);
        let v917: f64 = (v858 / v110);
        let v918: f64 = (v114 * v858);
        let v919: f64 = (v115 * v858);
        let v920: f64 = (v11 * v918);
        let v921: f64 = (v919 + v920);
        let v922: f64 = (v118 * v921);
        let v923: f64 = (v116 * v858);
        let v924: f64 = (v922 - v923);
        let v925: f64 = (v118 * v118);
        let v926: f64 = (v924 / v925);
        let v927: f64 = (v858 + v858);
        let v928: f64 = (v122 * v927);
        let v929: f64 = (v124 * v926);
        let v930: f64 = (v121 * v928);
        let v931: f64 = (v929 - v930);
        let v932: f64 = (v124 * v124);
        let v933: f64 = (v931 / v932);
        let v934: f64 = (v871 + v871);
        let v935: f64 = (-v934);
        let v936: f64 = (v917 / v112);
        let v937: f64 = (v130 * v936);
        let v938: f64 = (v133 * v933);
        let v939: f64 = (v937 + v938);
        let v940: f64 = (v135 * v935);
        let v941: f64 = (v129 * v939);
        let v942: f64 = (v940 + v941);
        let v943: f64 = (-v942);
        let v944: f64 = (v943 / self.scalar_v111);
        let v945: f64 = (-v944);
        let v946: f64 = (v139 * v945);
        let v947: f64 = (v140 * v944);
        let v948: f64 = (v946 - v947);
        let v949: f64 = (v139 * v139);
        let v950: f64 = (v948 / v949);
        let v951: f64 = (-v950);
        let v952: f64 = (self.scalar_v142 * v951);
        let v953: f64 = (self.scalar_v107 * v952);
        let v954: f64 = (-v953);
        let v955: f64 = (v148 * v148);
        let v956: f64 = (v954 / v955);
        let v957: f64 = (v139 * v917);
        let v958: f64 = (v112 * v944);
        let v959: f64 = (v957 + v958);
        let v960: f64 = (v942 + v959);
        let v961: f64 = (v960 - v944);
        let v962: f64 = (v139 * v961);
        let v963: f64 = (v152 * v944);
        let v964: f64 = (v962 - v963);
        let v965: f64 = (v964 / v949);
        let v966: f64 = (v143 * v858);
        let v967: f64 = (v966 - v965);
        let v968: f64 = (self.scalar_v142 * v967);
        let v969: f64 = (v158 * v956);
        let v970: f64 = (v149 * v968);
        let v971: f64 = (v969 + v970);
        let v972: f64 = (v162 * v945);
        let v973: f64 = (v163 * v944);
        let v974: f64 = (v972 - v973);
        let v975: f64 = (v162 * v162);
        let v976: f64 = (v974 / v975);
        let v977: f64 = (-v976);
        let v978: f64 = (self.scalar_v165 * v977);
        let v979: f64 = (self.scalar_v108 * v978);
        let v980: f64 = (-v979);
        let v981: f64 = (v168 * v168);
        let v982: f64 = (v980 / v981);
        let v983: f64 = (v162 * v917);
        let v984: f64 = (v958 + v983);
        let v985: f64 = (v942 + v984);
        let v986: f64 = (v985 - v944);
        let v987: f64 = (v162 * v986);
        let v988: f64 = (v172 * v944);
        let v989: f64 = (v987 - v988);
        let v990: f64 = (v989 / v975);
        let v991: f64 = (v966 - v990);
        let v992: f64 = (self.scalar_v165 * v991);
        let v993: f64 = (v176 * v982);
        let v994: f64 = (v169 * v992);
        let v995: f64 = (v993 + v994);
        let v996: f64 = (v180 * v945);
        let v997: f64 = (v181 * v944);
        let v998: f64 = (v996 - v997);
        let v999: f64 = (v180 * v180);
        let v1000: f64 = (v998 / v999);
        let v1001: f64 = (-v1000);
        let v1002: f64 = (self.scalar_v183 * v1001);
        let v1003: f64 = (self.scalar_v109 * v1002);
        let v1004: f64 = (-v1003);
        let v1005: f64 = (v186 * v186);
        let v1006: f64 = (v1004 / v1005);
        let v1007: f64 = (v180 * v917);
        let v1008: f64 = (v958 + v1007);
        let v1009: f64 = (v942 + v1008);
        let v1010: f64 = (v1009 - v944);
        let v1011: f64 = (v180 * v1010);
        let v1012: f64 = (v190 * v944);
        let v1013: f64 = (v1011 - v1012);
        let v1014: f64 = (v1013 / v999);
        let v1015: f64 = (v966 - v1014);
        let v1016: f64 = (self.scalar_v183 * v1015);
        let v1017: f64 = (v194 * v1006);
        let v1018: f64 = (v187 * v1016);
        let v1019: f64 = (v1017 + v1018);
        let v1020: f64 = (self.scalar_v210 * v871);
        let v1021: f64 = (v201 * v1020);
        let v1022: f64 = (-v1021);
        let v1023: f64 = (v211 * v211);
        let v1024: f64 = (v1022 / v1023);
        let v1025: f64 = (self.scalar_v17 / v211);
        let v1026: f64 = (self.scalar_v859 / v211);
        let v1027: f64 = (if v209 { v1024 } else { v13 });
        let v1028: f64 = (if v209 { v1025 } else { v13 });
        let v1029: f64 = (if v209 { v1026 } else { v13 });
        let v1030: f64 = (-v912);
        let v1031: f64 = (self.scalar_v216 * v871);
        let v1032: f64 = (v217 * v1030);
        let v1033: f64 = (v215 * v1031);
        let v1034: f64 = (v1032 - v1033);
        let v1035: f64 = (v217 * v217);
        let v1036: f64 = (v1034 / v1035);
        let v1037: f64 = (self.scalar_v859 / v217);
        let v1038: f64 = (self.scalar_v17 / v217);
        let v1039: f64 = (if v209 { v1036 } else { v13 });
        let v1040: f64 = (if v209 { v1037 } else { v13 });
        let v1041: f64 = (if v209 { v1038 } else { v13 });
        let v1042: f64 = (v220 * v1031);
        let v1043: f64 = (v1032 - v1042);
        let v1044: f64 = (v1043 / v1035);
        let v1045: f64 = (if v209 { v1044 } else { v13 });
        let v1046: f64 = (if v225 { v1027 } else { v13 });
        let v1047: f64 = (if v225 { v1028 } else { v13 });
        let v1048: f64 = (if v225 { v1029 } else { v13 });
        let v1049: f64 = (if v225 { v13 } else { v1027 });
        let v1050: f64 = (if v225 { v13 } else { v1028 });
        let v1051: f64 = (if v225 { v13 } else { v1029 });
        let v1052: f64 = (if v231 { v13 } else { v1046 });
        let v1053: f64 = (if v231 { v13 } else { v1047 });
        let v1054: f64 = (if v231 { v13 } else { v1048 });
        let v1055: f64 = (v233 * v1049);
        let v1056: f64 = (v233 * v1050);
        let v1057: f64 = (v233 * v1051);
        let v1058: f64 = (v233 * v1052);
        let v1059: f64 = (v232 * v1055);
        let v1060: f64 = (v1058 + v1059);
        let v1061: f64 = (v233 * v1053);
        let v1062: f64 = (v232 * v1056);
        let v1063: f64 = (v1061 + v1062);
        let v1064: f64 = (v233 * v1054);
        let v1065: f64 = (v232 * v1057);
        let v1066: f64 = (v1064 + v1065);
        let v1067: f64 = (if v209 { v1060 } else { v1052 });
        let v1068: f64 = (if v209 { v1063 } else { v1053 });
        let v1069: f64 = (if v209 { v1066 } else { v1054 });
        let v1070: f64 = (v243 * v1039);
        let v1071: f64 = (v243 * v1040);
        let v1072: f64 = (v243 * v1041);
        let v1073: f64 = (v1070 / v244);
        let v1074: f64 = (v1071 / v244);
        let v1075: f64 = (v1072 / v244);
        let v1076: f64 = (if v237 { v1039 } else { v13 });
        let v1077: f64 = (if v237 { v1040 } else { v13 });
        let v1078: f64 = (if v237 { v1041 } else { v13 });
        let v1079: f64 = (if v246 { v1070 } else { v1076 });
        let v1080: f64 = (if v246 { v1071 } else { v1077 });
        let v1081: f64 = (if v246 { v1072 } else { v1078 });
        let v1082: f64 = (if v242 { v1073 } else { v1079 });
        let v1083: f64 = (if v242 { v1074 } else { v1080 });
        let v1084: f64 = (if v242 { v1075 } else { v1081 });
        let v1085: f64 = (v255 * v1045);
        let v1086: f64 = (v1085 / v256);
        let v1087: f64 = (if v250 { v1045 } else { v13 });
        let v1088: f64 = (if v258 { v1085 } else { v1087 });
        let v1089: f64 = (if v254 { v1086 } else { v1088 });
        let v1090: f64 = (v1082 - v1089);
        let v1091: f64 = (if v209 { v1090 } else { v13 });
        let v1092: f64 = (if v209 { v1083 } else { v13 });
        let v1093: f64 = (if v209 { v1084 } else { v13 });
        let v1094: f64 = (v264 * v891);
        let v1095: f64 = (v71 * v1067);
        let v1096: f64 = (v1094 + v1095);
        let v1097: f64 = (v71 * v1068);
        let v1098: f64 = (v71 * v1069);
        let v1099: f64 = (v263 * v910);
        let v1100: f64 = (v91 * v1091);
        let v1101: f64 = (v1099 + v1100);
        let v1102: f64 = (v91 * v1092);
        let v1103: f64 = (v91 * v1093);
        let v1104: f64 = ((v268) as f64).ln();
        let v1105: f64 = (v269 * v1104);
        let v1106: f64 = (v914 * v1105);
        let v1107: f64 = (self.scalar_v267 * v1106);
        let v1108: f64 = (v271 * v1101);
        let v1109: f64 = (v266 * v1107);
        let v1110: f64 = (v1108 - v1109);
        let v1111: f64 = (v271 * v271);
        let v1112: f64 = (v1110 / v1111);
        let v1113: f64 = (v1102 / v271);
        let v1114: f64 = (v1103 / v271);
        let v1115: f64 = (v1096 - v1112);
        let v1116: f64 = (v1097 - v1113);
        let v1117: f64 = (v1098 - v1114);
        let v1118: f64 = (if v209 { v1115 } else { v13 });
        let v1119: f64 = (if v209 { v1116 } else { v13 });
        let v1120: f64 = (if v209 { v1117 } else { v13 });
        let v1121: f64 = (if v275 { v13 } else { v1118 });
        let v1122: f64 = (if v275 { v13 } else { v1119 });
        let v1123: f64 = (if v275 { v13 } else { v1120 });
        let v1124: f64 = (if v281 { self.scalar_v859 } else { v13 });
        let v1125: f64 = (if v281 { self.scalar_v17 } else { v13 });
        let v1126: f64 = (if v277 { v1124 } else { v13 });
        let v1127: f64 = (if v277 { v1125 } else { v13 });
        let v1130: f64 = (self.scalar_v286 * v871);
        let v1131: f64 = (v283 * v1130);
        let v1132: f64 = (v287 * v1126);
        let v1133: f64 = (v287 * v1127);
        let v1134: f64 = (v285 * v1131);
        let v1135: f64 = (-v1134);
        let v1136: f64 = (v288 * v288);
        let v1137: f64 = (v1135 / v1136);
        let v1138: f64 = (v288 * self.scalar_v1128);
        let v1139: f64 = (v285 * v1132);
        let v1140: f64 = (v1138 - v1139);
        let v1141: f64 = (v1140 / v1136);
        let v1142: f64 = (v288 * self.scalar_v1129);
        let v1143: f64 = (v285 * v1133);
        let v1144: f64 = (v1142 - v1143);
        let v1145: f64 = (v1144 / v1136);
        let v1146: f64 = (if v277 { v1137 } else { v1049 });
        let v1147: f64 = (if v277 { v1141 } else { v1050 });
        let v1148: f64 = (if v277 { v1145 } else { v1051 });
        let v1149: f64 = (if v292 { v1146 } else { v1067 });
        let v1150: f64 = (if v292 { v1147 } else { v1068 });
        let v1151: f64 = (if v292 { v1148 } else { v1069 });
        let v1152: f64 = (if v292 { v13 } else { v1146 });
        let v1153: f64 = (if v292 { v13 } else { v1147 });
        let v1154: f64 = (if v292 { v13 } else { v1148 });
        let v1155: f64 = (if v298 { v13 } else { v1149 });
        let v1156: f64 = (if v298 { v13 } else { v1150 });
        let v1157: f64 = (if v298 { v13 } else { v1151 });
        let v1158: f64 = (v300 * v1152);
        let v1159: f64 = (v300 * v1153);
        let v1160: f64 = (v300 * v1154);
        let v1161: f64 = (v300 * v1155);
        let v1162: f64 = (v299 * v1158);
        let v1163: f64 = (v1161 + v1162);
        let v1164: f64 = (v300 * v1156);
        let v1165: f64 = (v299 * v1159);
        let v1166: f64 = (v1164 + v1165);
        let v1167: f64 = (v300 * v1157);
        let v1168: f64 = (v299 * v1160);
        let v1169: f64 = (v1167 + v1168);
        let v1170: f64 = (if v277 { v1163 } else { v1155 });
        let v1171: f64 = (if v277 { v1166 } else { v1156 });
        let v1172: f64 = (if v277 { v1169 } else { v1157 });
        let v1173: f64 = (v303 * v893);
        let v1174: f64 = (v74 * v1170);
        let v1175: f64 = (v1173 + v1174);
        let v1176: f64 = (v74 * v1171);
        let v1177: f64 = (v74 * v1172);
        let v1178: f64 = (if v277 { v1175 } else { v13 });
        let v1179: f64 = (if v277 { v1176 } else { v13 });
        let v1180: f64 = (if v277 { v1177 } else { v13 });
        let v1181: f64 = (if v306 { v13 } else { v1178 });
        let v1182: f64 = (if v306 { v13 } else { v1179 });
        let v1183: f64 = (if v306 { v13 } else { v1180 });
        let v1184: f64 = (self.scalar_v76 * v871);
        let v1185: f64 = (v201 * v1184);
        let v1186: f64 = (-v1185);
        let v1187: f64 = (v309 * v309);
        let v1188: f64 = (v1186 / v1187);
        let v1189: f64 = (self.scalar_v17 / v309);
        let v1190: f64 = (self.scalar_v859 / v309);
        let v1191: f64 = (if v308 { v1188 } else { v1152 });
        let v1192: f64 = (if v308 { v1189 } else { v1153 });
        let v1193: f64 = (if v308 { v1190 } else { v1154 });
        let v1194: f64 = (self.scalar_v312 * v871);
        let v1195: f64 = (v313 * v1030);
        let v1196: f64 = (v215 * v1194);
        let v1197: f64 = (v1195 - v1196);
        let v1198: f64 = (v313 * v313);
        let v1199: f64 = (v1197 / v1198);
        let v1200: f64 = (self.scalar_v859 / v313);
        let v1201: f64 = (self.scalar_v17 / v313);
        let v1202: f64 = (if v308 { v1199 } else { v1039 });
        let v1203: f64 = (if v308 { v1200 } else { v1040 });
        let v1204: f64 = (if v308 { v1201 } else { v1041 });
        let v1205: f64 = (v220 * v1194);
        let v1206: f64 = (v1195 - v1205);
        let v1207: f64 = (v1206 / v1198);
        let v1208: f64 = (if v308 { v1207 } else { v1045 });
        let v1209: f64 = (if v319 { v1191 } else { v1170 });
        let v1210: f64 = (if v319 { v1192 } else { v1171 });
        let v1211: f64 = (if v319 { v1193 } else { v1172 });
        let v1212: f64 = (if v319 { v13 } else { v1191 });
        let v1213: f64 = (if v319 { v13 } else { v1192 });
        let v1214: f64 = (if v319 { v13 } else { v1193 });
        let v1215: f64 = (if v325 { v13 } else { v1209 });
        let v1216: f64 = (if v325 { v13 } else { v1210 });
        let v1217: f64 = (if v325 { v13 } else { v1211 });
        let v1218: f64 = (v327 * v1212);
        let v1219: f64 = (v327 * v1213);
        let v1220: f64 = (v327 * v1214);
        let v1221: f64 = (v327 * v1215);
        let v1222: f64 = (v326 * v1218);
        let v1223: f64 = (v1221 + v1222);
        let v1224: f64 = (v327 * v1216);
        let v1225: f64 = (v326 * v1219);
        let v1226: f64 = (v1224 + v1225);
        let v1227: f64 = (v327 * v1217);
        let v1228: f64 = (v326 * v1220);
        let v1229: f64 = (v1227 + v1228);
        let v1230: f64 = (if v308 { v1223 } else { v1215 });
        let v1231: f64 = (if v308 { v1226 } else { v1216 });
        let v1232: f64 = (if v308 { v1229 } else { v1217 });
        let v1233: f64 = (v335 * v1202);
        let v1234: f64 = (v335 * v1203);
        let v1235: f64 = (v335 * v1204);
        let v1236: f64 = (v1233 / v336);
        let v1237: f64 = (v1234 / v336);
        let v1238: f64 = (v1235 / v336);
        let v1239: f64 = (if v330 { v1202 } else { v13 });
        let v1240: f64 = (if v330 { v1203 } else { v13 });
        let v1241: f64 = (if v330 { v1204 } else { v13 });
        let v1242: f64 = (if v338 { v1233 } else { v1239 });
        let v1243: f64 = (if v338 { v1234 } else { v1240 });
        let v1244: f64 = (if v338 { v1235 } else { v1241 });
        let v1245: f64 = (if v334 { v1236 } else { v1242 });
        let v1246: f64 = (if v334 { v1237 } else { v1243 });
        let v1247: f64 = (if v334 { v1238 } else { v1244 });
        let v1248: f64 = (v347 * v1208);
        let v1249: f64 = (v1248 / v348);
        let v1250: f64 = (if v342 { v1208 } else { v13 });
        let v1251: f64 = (if v350 { v1248 } else { v1250 });
        let v1252: f64 = (if v346 { v1249 } else { v1251 });
        let v1253: f64 = (v1245 - v1252);
        let v1254: f64 = (if v308 { v1253 } else { v1091 });
        let v1255: f64 = (if v308 { v1246 } else { v1092 });
        let v1256: f64 = (if v308 { v1247 } else { v1093 });
        let v1257: f64 = (v356 * v901);
        let v1258: f64 = (v80 * v1230);
        let v1259: f64 = (v1257 + v1258);
        let v1260: f64 = (v80 * v1231);
        let v1261: f64 = (v80 * v1232);
        let v1262: f64 = (v13 * v1254);
        let v1263: f64 = (v13 * v1255);
        let v1264: f64 = (v13 * v1256);
        let v1265: f64 = (v271 * v1262);
        let v1266: f64 = (v358 * v1107);
        let v1267: f64 = (v1265 - v1266);
        let v1268: f64 = (v1267 / v1111);
        let v1269: f64 = (v1263 / v271);
        let v1270: f64 = (v1264 / v271);
        let v1271: f64 = (v1259 - v1268);
        let v1272: f64 = (v1260 - v1269);
        let v1273: f64 = (v1261 - v1270);
        let v1274: f64 = (if v308 { v1271 } else { v13 });
        let v1275: f64 = (if v308 { v1272 } else { v13 });
        let v1276: f64 = (if v308 { v1273 } else { v13 });
        let v1277: f64 = (if v362 { v13 } else { v1274 });
        let v1278: f64 = (if v362 { v13 } else { v1275 });
        let v1279: f64 = (if v362 { v13 } else { v1276 });
        let v1280: f64 = (self.scalar_v364 * v871);
        let v1281: f64 = (v21 * v1280);
        let v1282: f64 = (-v1281);
        let v1283: f64 = (v365 * v365);
        let v1284: f64 = (v1282 / v1283);
        let v1285: f64 = (self.scalar_v859 / v365);
        let v1286: f64 = (self.scalar_v17 / v365);
        let v1287: f64 = (if v209 { v1284 } else { v1212 });
        let v1288: f64 = (if v209 { v1285 } else { v13 });
        let v1289: f64 = (if v209 { v1286 } else { v1213 });
        let v1290: f64 = (if v209 { v13 } else { v1214 });
        let v1291: f64 = (v369 * v1194);
        let v1292: f64 = (v1195 - v1291);
        let v1293: f64 = (v1292 / v1198);
        let v1294: f64 = (if v209 { v1293 } else { v1202 });
        let v1295: f64 = (if v209 { v1201 } else { v13 });
        let v1296: f64 = (if v209 { v1200 } else { v1203 });
        let v1297: f64 = (if v209 { v13 } else { v1204 });
        let v1298: f64 = (if v209 { v1207 } else { v1208 });
        let v1299: f64 = (if v374 { v1287 } else { v1230 });
        let v1300: f64 = (if v374 { v1288 } else { v13 });
        let v1301: f64 = (if v374 { v1289 } else { v1231 });
        let v1302: f64 = (if v374 { v1290 } else { v1232 });
        let v1303: f64 = (if v374 { v13 } else { v1287 });
        let v1304: f64 = (if v374 { v13 } else { v1288 });
        let v1305: f64 = (if v374 { v13 } else { v1289 });
        let v1306: f64 = (if v374 { v13 } else { v1290 });
        let v1307: f64 = (if v380 { v13 } else { v1299 });
        let v1308: f64 = (if v380 { v13 } else { v1300 });
        let v1309: f64 = (if v380 { v13 } else { v1301 });
        let v1310: f64 = (if v380 { v13 } else { v1302 });
        let v1311: f64 = (v382 * v1303);
        let v1312: f64 = (v382 * v1304);
        let v1313: f64 = (v382 * v1305);
        let v1314: f64 = (v382 * v1306);
        let v1315: f64 = (v382 * v1307);
        let v1316: f64 = (v381 * v1311);
        let v1317: f64 = (v1315 + v1316);
        let v1318: f64 = (v382 * v1308);
        let v1319: f64 = (v381 * v1312);
        let v1320: f64 = (v1318 + v1319);
        let v1321: f64 = (v382 * v1309);
        let v1322: f64 = (v381 * v1313);
        let v1323: f64 = (v1321 + v1322);
        let v1324: f64 = (v382 * v1310);
        let v1325: f64 = (v381 * v1314);
        let v1326: f64 = (v1324 + v1325);
        let v1327: f64 = (if v209 { v1317 } else { v1307 });
        let v1328: f64 = (if v209 { v1320 } else { v1308 });
        let v1329: f64 = (if v209 { v1323 } else { v1309 });
        let v1330: f64 = (if v209 { v1326 } else { v1310 });
        let v1331: f64 = (v390 * v1294);
        let v1332: f64 = (v390 * v1295);
        let v1333: f64 = (v390 * v1296);
        let v1334: f64 = (v390 * v1297);
        let v1335: f64 = (v1331 / v391);
        let v1336: f64 = (v1332 / v391);
        let v1337: f64 = (v1333 / v391);
        let v1338: f64 = (v1334 / v391);
        let v1339: f64 = (if v385 { v1294 } else { v13 });
        let v1340: f64 = (if v385 { v1295 } else { v13 });
        let v1341: f64 = (if v385 { v1296 } else { v13 });
        let v1342: f64 = (if v385 { v1297 } else { v13 });
        let v1343: f64 = (if v393 { v1331 } else { v1339 });
        let v1344: f64 = (if v393 { v1332 } else { v1340 });
        let v1345: f64 = (if v393 { v1333 } else { v1341 });
        let v1346: f64 = (if v393 { v1334 } else { v1342 });
        let v1347: f64 = (if v389 { v1335 } else { v1343 });
        let v1348: f64 = (if v389 { v1336 } else { v1344 });
        let v1349: f64 = (if v389 { v1337 } else { v1345 });
        let v1350: f64 = (if v389 { v1338 } else { v1346 });
        let v1351: f64 = (v402 * v1298);
        let v1352: f64 = (v1351 / v403);
        let v1353: f64 = (if v397 { v1298 } else { v13 });
        let v1354: f64 = (if v405 { v1351 } else { v1353 });
        let v1355: f64 = (if v401 { v1352 } else { v1354 });
        let v1356: f64 = (v1347 - v1355);
        let v1357: f64 = (if v209 { v1356 } else { v1254 });
        let v1358: f64 = (if v209 { v1348 } else { v13 });
        let v1359: f64 = (if v209 { v1349 } else { v1255 });
        let v1360: f64 = (if v209 { v1350 } else { v1256 });
        let v1361: f64 = (v411 * v891);
        let v1362: f64 = (v71 * v1327);
        let v1363: f64 = (v1361 + v1362);
        let v1364: f64 = (v71 * v1328);
        let v1365: f64 = (v71 * v1329);
        let v1366: f64 = (v71 * v1330);
        let v1367: f64 = (v410 * v916);
        let v1368: f64 = (v106 * v1357);
        let v1369: f64 = (v1367 + v1368);
        let v1370: f64 = (v106 * v1358);
        let v1371: f64 = (v106 * v1359);
        let v1372: f64 = (v106 * v1360);
        let v1373: f64 = ((v414) as f64).ln();
        let v1374: f64 = (v415 * v1373);
        let v1375: f64 = (v914 * v1374);
        let v1376: f64 = (self.scalar_v267 * v1375);
        let v1377: f64 = (v417 * v1369);
        let v1378: f64 = (v413 * v1376);
        let v1379: f64 = (v1377 - v1378);
        let v1380: f64 = (v417 * v417);
        let v1381: f64 = (v1379 / v1380);
        let v1382: f64 = (v1370 / v417);
        let v1383: f64 = (v1371 / v417);
        let v1384: f64 = (v1372 / v417);
        let v1385: f64 = (v1363 - v1381);
        let v1386: f64 = (v1364 - v1382);
        let v1387: f64 = (v1365 - v1383);
        let v1388: f64 = (v1366 - v1384);
        let v1389: f64 = (if v209 { v1385 } else { v13 });
        let v1390: f64 = (if v209 { v1386 } else { v13 });
        let v1391: f64 = (if v209 { v1387 } else { v13 });
        let v1392: f64 = (if v209 { v1388 } else { v13 });
        let v1393: f64 = (if v275 { v13 } else { v1389 });
        let v1394: f64 = (if v275 { v13 } else { v1390 });
        let v1395: f64 = (if v275 { v13 } else { v1391 });
        let v1396: f64 = (if v275 { v13 } else { v1392 });
        let v1397: f64 = (self.scalar_v82 * v871);
        let v1398: f64 = (v21 * v1397);
        let v1399: f64 = (-v1398);
        let v1400: f64 = (v423 * v423);
        let v1401: f64 = (v1399 / v1400);
        let v1402: f64 = (self.scalar_v859 / v423);
        let v1403: f64 = (self.scalar_v17 / v423);
        let v1404: f64 = (if v422 { v1401 } else { v1303 });
        let v1405: f64 = (if v422 { v1402 } else { v1304 });
        let v1406: f64 = (if v422 { v1403 } else { v1305 });
        let v1407: f64 = (if v422 { v13 } else { v1306 });
        let v1408: f64 = (if v422 { v1293 } else { v1294 });
        let v1409: f64 = (if v422 { v1201 } else { v1295 });
        let v1410: f64 = (if v422 { v1200 } else { v1296 });
        let v1411: f64 = (if v422 { v13 } else { v1297 });
        let v1412: f64 = (if v422 { v1207 } else { v1298 });
        let v1413: f64 = (if v429 { v1404 } else { v1327 });
        let v1414: f64 = (if v429 { v1405 } else { v1328 });
        let v1415: f64 = (if v429 { v1406 } else { v1329 });
        let v1416: f64 = (if v429 { v1407 } else { v1330 });
        let v1417: f64 = (if v429 { v13 } else { v1404 });
        let v1418: f64 = (if v429 { v13 } else { v1405 });
        let v1419: f64 = (if v429 { v13 } else { v1406 });
        let v1420: f64 = (if v429 { v13 } else { v1407 });
        let v1421: f64 = (if v435 { v13 } else { v1413 });
        let v1422: f64 = (if v435 { v13 } else { v1414 });
        let v1423: f64 = (if v435 { v13 } else { v1415 });
        let v1424: f64 = (if v435 { v13 } else { v1416 });
        let v1425: f64 = (v437 * v1417);
        let v1426: f64 = (v437 * v1418);
        let v1427: f64 = (v437 * v1419);
        let v1428: f64 = (v437 * v1420);
        let v1429: f64 = (v437 * v1421);
        let v1430: f64 = (v436 * v1425);
        let v1431: f64 = (v1429 + v1430);
        let v1432: f64 = (v437 * v1422);
        let v1433: f64 = (v436 * v1426);
        let v1434: f64 = (v1432 + v1433);
        let v1435: f64 = (v437 * v1423);
        let v1436: f64 = (v436 * v1427);
        let v1437: f64 = (v1435 + v1436);
        let v1438: f64 = (v437 * v1424);
        let v1439: f64 = (v436 * v1428);
        let v1440: f64 = (v1438 + v1439);
        let v1441: f64 = (if v422 { v1431 } else { v1421 });
        let v1442: f64 = (if v422 { v1434 } else { v1422 });
        let v1443: f64 = (if v422 { v1437 } else { v1423 });
        let v1444: f64 = (if v422 { v1440 } else { v1424 });
        let v1445: f64 = (v445 * v1408);
        let v1446: f64 = (v445 * v1409);
        let v1447: f64 = (v445 * v1410);
        let v1448: f64 = (v445 * v1411);
        let v1449: f64 = (v1445 / v446);
        let v1450: f64 = (v1446 / v446);
        let v1451: f64 = (v1447 / v446);
        let v1452: f64 = (v1448 / v446);
        let v1453: f64 = (if v440 { v1408 } else { v13 });
        let v1454: f64 = (if v440 { v1409 } else { v13 });
        let v1455: f64 = (if v440 { v1410 } else { v13 });
        let v1456: f64 = (if v440 { v1411 } else { v13 });
        let v1457: f64 = (if v448 { v1445 } else { v1453 });
        let v1458: f64 = (if v448 { v1446 } else { v1454 });
        let v1459: f64 = (if v448 { v1447 } else { v1455 });
        let v1460: f64 = (if v448 { v1448 } else { v1456 });
        let v1461: f64 = (if v444 { v1449 } else { v1457 });
        let v1462: f64 = (if v444 { v1450 } else { v1458 });
        let v1463: f64 = (if v444 { v1451 } else { v1459 });
        let v1464: f64 = (if v444 { v1452 } else { v1460 });
        let v1465: f64 = (v457 * v1412);
        let v1466: f64 = (v1465 / v458);
        let v1467: f64 = (if v452 { v1412 } else { v13 });
        let v1468: f64 = (if v460 { v1465 } else { v1467 });
        let v1469: f64 = (if v456 { v1466 } else { v1468 });
        let v1470: f64 = (v1461 - v1469);
        let v1471: f64 = (if v422 { v1470 } else { v1357 });
        let v1472: f64 = (if v422 { v1462 } else { v1358 });
        let v1473: f64 = (if v422 { v1463 } else { v1359 });
        let v1474: f64 = (if v422 { v1464 } else { v1360 });
        let v1475: f64 = (v466 * v908);
        let v1476: f64 = (v86 * v1441);
        let v1477: f64 = (v1475 + v1476);
        let v1478: f64 = (v86 * v1442);
        let v1479: f64 = (v86 * v1443);
        let v1480: f64 = (v86 * v1444);
        let v1481: f64 = (v13 * v1471);
        let v1482: f64 = (v13 * v1472);
        let v1483: f64 = (v13 * v1473);
        let v1484: f64 = (v13 * v1474);
        let v1485: f64 = (v1481 / v471);
        let v1486: f64 = (v1482 / v471);
        let v1487: f64 = (v1483 / v471);
        let v1488: f64 = (v1484 / v471);
        let v1489: f64 = (v1477 - v1485);
        let v1490: f64 = (v1478 - v1486);
        let v1491: f64 = (v1479 - v1487);
        let v1492: f64 = (v1480 - v1488);
        let v1493: f64 = (if v422 { v1489 } else { v13 });
        let v1494: f64 = (if v422 { v1490 } else { v13 });
        let v1495: f64 = (if v422 { v1491 } else { v13 });
        let v1496: f64 = (if v422 { v1492 } else { v13 });
        let v1497: f64 = (if v475 { v13 } else { v1493 });
        let v1498: f64 = (if v475 { v13 } else { v1494 });
        let v1499: f64 = (if v475 { v13 } else { v1495 });
        let v1500: f64 = (if v475 { v13 } else { v1496 });
        let v1501: f64 = (v1121 - v1181);
        let v1502: f64 = (v1122 - v1182);
        let v1503: f64 = (v1123 - v1183);
        let v1504: f64 = (v41 * v1501);
        let v1505: f64 = (v485 * v877);
        let v1506: f64 = (v1504 - v1505);
        let v1507: f64 = (v41 * v41);
        let v1508: f64 = (v1506 / v1507);
        let v1509: f64 = (v485 * v878);
        let v1510: f64 = (-v1509);
        let v1511: f64 = (v1510 / v1507);
        let v1512: f64 = (v41 * v1502);
        let v1513: f64 = (v485 * v879);
        let v1514: f64 = (v1512 - v1513);
        let v1515: f64 = (v1514 / v1507);
        let v1516: f64 = (v1503 / v41);
        let v1517: f64 = (v1277 + v1508);
        let v1518: f64 = (v1278 + v1515);
        let v1519: f64 = (v1279 + v1516);
        let v1520: f64 = (v43 * v1393);
        let v1521: f64 = (v421 * v880);
        let v1522: f64 = (v1520 - v1521);
        let v1523: f64 = (v43 * v43);
        let v1524: f64 = (v1522 / v1523);
        let v1525: f64 = (v1394 / v43);
        let v1526: f64 = (v1395 / v43);
        let v1527: f64 = (v1396 / v43);
        let v1528: f64 = (v1497 + v1524);
        let v1529: f64 = (v1498 + v1525);
        let v1530: f64 = (v1499 + v1526);
        let v1531: f64 = (v1500 + v1527);
        let v1543: f64 = (self.scalar_v1540 / v507);
        let v1544: f64 = (self.scalar_v1541 / v507);
        let v1545: f64 = (self.scalar_v1542 / v507);
        let v1546: f64 = (v510 * v1393);
        let v1547: f64 = (v510 * v1394);
        let v1548: f64 = (v421 * v1543);
        let v1549: f64 = (v1547 + v1548);
        let v1550: f64 = (v510 * v1395);
        let v1551: f64 = (v421 * v1544);
        let v1552: f64 = (v1550 + v1551);
        let v1553: f64 = (v510 * v1396);
        let v1554: f64 = (v421 * v1545);
        let v1555: f64 = (v1553 + v1554);
        let v1556: f64 = (v510 * v1121);
        let v1557: f64 = (v276 * v1543);
        let v1558: f64 = (v510 * v1122);
        let v1559: f64 = (v276 * v1544);
        let v1560: f64 = (v1558 + v1559);
        let v1561: f64 = (v510 * v1123);
        let v1562: f64 = (v276 * v1545);
        let v1563: f64 = (v1561 + v1562);
        let v1564: f64 = (v484 * v1556);
        let v1565: f64 = (v484 * v1557);
        let v1566: f64 = (v484 * v1560);
        let v1567: f64 = (v484 * v1563);
        let v1568: f64 = (self.scalar_v514 * v1564);
        let v1569: f64 = (self.scalar_v514 * v1565);
        let v1570: f64 = (self.scalar_v514 * v1566);
        let v1571: f64 = (self.scalar_v514 * v1567);
        let v1572: f64 = (self.scalar_v516 * v1121);
        let v1573: f64 = (self.scalar_v516 * v1122);
        let v1574: f64 = (self.scalar_v516 * v1123);
        let v1575: f64 = (v510 * v1572);
        let v1576: f64 = (v517 * v1543);
        let v1577: f64 = (v517 * v1544);
        let v1578: f64 = (v510 * v1573);
        let v1579: f64 = (v1577 + v1578);
        let v1580: f64 = (v517 * v1545);
        let v1581: f64 = (v510 * v1574);
        let v1582: f64 = (v1580 + v1581);
        let v1583: f64 = (v1568 + v1575);
        let v1584: f64 = (v1569 + v1576);
        let v1585: f64 = (v1570 + v1579);
        let v1586: f64 = (v1571 + v1582);
        let v1587: f64 = (self.scalar_v533 * v873);
        let v1588: f64 = (v535 * v1587);
        let v1589: f64 = (self.scalar_v532 * v1588);
        let v1590: f64 = (v538 * v1589);
        let v1591: f64 = (self.scalar_v541 * v873);
        let v1592: f64 = (v543 * v1591);
        let v1593: f64 = (self.scalar_v540 * v1592);
        let v1594: f64 = (self.scalar_v546 * v873);
        let v1595: f64 = (v548 * v1594);
        let v1596: f64 = (self.scalar_v545 * v1595);
        let v1597: f64 = (v551 * v1596);
        let v1598: f64 = (v567 * v1121);
        let v1599: f64 = (v567 * v1122);
        let v1600: f64 = (v567 * v1123);
        let v1601: f64 = (self.scalar_v569 * v1546);
        let v1602: f64 = (self.scalar_v569 * v1549);
        let v1603: f64 = (self.scalar_v569 * v1552);
        let v1604: f64 = (self.scalar_v569 * v1555);
        let v1605: f64 = (v1590 / v579);
        let v1606: f64 = (if self.scalar_v572 { v1605 } else { v1590 });
        let v1607: f64 = (if self.scalar_v582 { v1606 } else { v1606 });
        let v1608: f64 = (if self.scalar_v585 { v1607 } else { v1607 });
        let v1609: f64 = (if self.scalar_v585 { v1593 } else { v1593 });
        let v1610: f64 = (if self.scalar_v585 { v1597 } else { v1597 });
        let v1611: f64 = (v195 * v1009);
        let v1612: f64 = (v189 * v1019);
        let v1613: f64 = (v1611 + v1612);
        let v1614: f64 = (self.scalar_v17 / v189);
        let v1615: f64 = (v198 * v1009);
        let v1616: f64 = (-v1615);
        let v1617: f64 = (v189 * v189);
        let v1618: f64 = (v1616 / v1617);
        let v1619: f64 = (self.scalar_v859 / v189);
        let v1620: f64 = (-v1614);
        let v1621: f64 = (-v1618);
        let v1622: f64 = (-v1619);
        let v1623: f64 = (v1620 / v599);
        let v1624: f64 = (v1621 / v599);
        let v1625: f64 = (v1622 / v599);
        let v1626: f64 = (self.scalar_v597 * v1623);
        let v1627: f64 = (self.scalar_v597 * v1624);
        let v1628: f64 = (self.scalar_v597 * v1625);
        let v1629: f64 = (v602 * v1626);
        let v1630: f64 = (v602 * v1627);
        let v1631: f64 = (v602 * v1628);
        let v1632: f64 = (-v1629);
        let v1633: f64 = (-v1630);
        let v1634: f64 = (-v1631);
        let v1635: f64 = (v596 * v1632);
        let v1636: f64 = (v603 * v1613);
        let v1637: f64 = (v596 * v1633);
        let v1638: f64 = (v1636 + v1637);
        let v1639: f64 = (v596 * v1634);
        let v1640: f64 = (v1635 / self.scalar_v597);
        let v1641: f64 = (v1638 / self.scalar_v597);
        let v1642: f64 = (v1639 / self.scalar_v597);
        let v1643: f64 = (if v595 { v1640 } else { v13 });
        let v1644: f64 = (if v595 { v1641 } else { v13 });
        let v1645: f64 = (if v595 { v1642 } else { v13 });
        let v1646: f64 = (self.scalar_v17 * v195);
        let v1647: f64 = (v198 * v1019);
        let v1648: f64 = (v195 * self.scalar_v859);
        let v1651: f64 = (self.scalar_v1649 / v189);
        let v1652: f64 = (v611 * v1009);
        let v1653: f64 = (-v1652);
        let v1654: f64 = (v1653 / v1617);
        let v1655: f64 = (self.scalar_v1650 / v189);
        let v1656: f64 = (v613 * v1646);
        let v1657: f64 = (v608 * v1651);
        let v1658: f64 = (v1656 + v1657);
        let v1659: f64 = (v613 * v1647);
        let v1660: f64 = (v608 * v1654);
        let v1661: f64 = (v1659 + v1660);
        let v1662: f64 = (v613 * v1648);
        let v1663: f64 = (v608 * v1655);
        let v1664: f64 = (v1662 + v1663);
        let v1665: f64 = (if v607 { v1658 } else { v1643 });
        let v1666: f64 = (if v607 { v1661 } else { v1644 });
        let v1667: f64 = (if v607 { v1664 } else { v1645 });
        let v1668: f64 = (-v960);
        let v1669: f64 = (self.scalar_v617 * v1668);
        let v1670: f64 = (v629 * v960);
        let v1671: f64 = (v1670 / self.scalar_v631);
        let v1672: f64 = (if v620 { v1671 } else { v13 });
        let v1673: f64 = (self.scalar_v634 * v1669);
        let v1676: f64 = (v151 * v1673);
        let v1677: f64 = (v635 * v960);
        let v1678: f64 = (v1676 - v1677);
        let v1679: f64 = (v151 * v151);
        let v1680: f64 = (v1678 / v1679);
        let v1681: f64 = (self.scalar_v1674 / v151);
        let v1682: f64 = (self.scalar_v1675 / v151);
        let v1683: f64 = (v637 * v1669);
        let v1684: f64 = (v619 * v1680);
        let v1685: f64 = (v1683 + v1684);
        let v1686: f64 = (self.scalar_v17 * v637);
        let v1687: f64 = (v619 * v1681);
        let v1688: f64 = (v1686 + v1687);
        let v1689: f64 = (v637 * self.scalar_v859);
        let v1690: f64 = (v619 * v1682);
        let v1691: f64 = (v1689 + v1690);
        let v1692: f64 = (v626 * v1685);
        let v1693: f64 = (v626 * v1688);
        let v1694: f64 = (v626 * v1691);
        let v1695: f64 = (if v620 { v1692 } else { v13 });
        let v1696: f64 = (if v620 { v1693 } else { v13 });
        let v1697: f64 = (if v620 { v1694 } else { v13 });
        let v1698: f64 = (v201 * v960);
        let v1699: f64 = (-v1698);
        let v1700: f64 = (v1699 / v1679);
        let v1701: f64 = (self.scalar_v17 / v151);
        let v1702: f64 = (self.scalar_v859 / v151);
        let v1703: f64 = (-v1700);
        let v1704: f64 = (-v1701);
        let v1705: f64 = (-v1702);
        let v1706: f64 = (v1703 / v643);
        let v1707: f64 = (v1704 / v643);
        let v1708: f64 = (v1705 / v643);
        let v1709: f64 = (self.scalar_v631 * v1706);
        let v1710: f64 = (self.scalar_v631 * v1707);
        let v1711: f64 = (self.scalar_v631 * v1708);
        let v1712: f64 = (v646 * v1709);
        let v1713: f64 = (v646 * v1710);
        let v1714: f64 = (v646 * v1711);
        let v1715: f64 = (-v1712);
        let v1716: f64 = (-v1713);
        let v1717: f64 = (-v1714);
        let v1718: f64 = (v647 * v960);
        let v1719: f64 = (v151 * v1715);
        let v1720: f64 = (v1718 + v1719);
        let v1721: f64 = (v151 * v1716);
        let v1722: f64 = (v151 * v1717);
        let v1723: f64 = (v1720 / self.scalar_v631);
        let v1724: f64 = (v1721 / self.scalar_v631);
        let v1725: f64 = (v1722 / self.scalar_v631);
        let v1726: f64 = (if v641 { v1723 } else { v1672 });
        let v1727: f64 = (if v641 { v1724 } else { v13 });
        let v1728: f64 = (if v641 { v1725 } else { v13 });
        let v1729: f64 = (if v641 { v13 } else { v1695 });
        let v1730: f64 = (if v641 { v13 } else { v1696 });
        let v1731: f64 = (if v641 { v13 } else { v1697 });
        let v1732: f64 = (v1726 + v1729);
        let v1733: f64 = (v1727 + v1730);
        let v1734: f64 = (v1728 + v1731);
        let v1735: f64 = (v652 * v971);
        let v1736: f64 = (v159 * v1732);
        let v1737: f64 = (v1735 + v1736);
        let v1738: f64 = (v159 * v1733);
        let v1739: f64 = (v159 * v1734);
        let v1740: f64 = (-v985);
        let v1741: f64 = (self.scalar_v617 * v1740);
        let v1742: f64 = (v664 * v985);
        let v1743: f64 = (v1742 / self.scalar_v666);
        let v1744: f64 = (if v657 { v1743 } else { v1726 });
        let v1745: f64 = (if v657 { v13 } else { v1727 });
        let v1746: f64 = (if v657 { v13 } else { v1728 });
        let v1748: f64 = (self.scalar_v669 * v1741);
        let v1750: f64 = (self.scalar_v1747 / v171);
        let v1751: f64 = (v171 * v1748);
        let v1752: f64 = (v670 * v985);
        let v1753: f64 = (v1751 - v1752);
        let v1754: f64 = (v171 * v171);
        let v1755: f64 = (v1753 / v1754);
        let v1756: f64 = (self.scalar_v1749 / v171);
        let v1757: f64 = (self.scalar_v17 * v672);
        let v1758: f64 = (v656 * v1750);
        let v1759: f64 = (v1757 + v1758);
        let v1760: f64 = (v672 * v1741);
        let v1761: f64 = (v656 * v1755);
        let v1762: f64 = (v1760 + v1761);
        let v1763: f64 = (v672 * self.scalar_v859);
        let v1764: f64 = (v656 * v1756);
        let v1765: f64 = (v1763 + v1764);
        let v1766: f64 = (v661 * v1759);
        let v1767: f64 = (v661 * v1762);
        let v1768: f64 = (v661 * v1765);
        let v1769: f64 = (if v657 { v1766 } else { v13 });
        let v1770: f64 = (if v657 { v1767 } else { v1729 });
        let v1771: f64 = (if v657 { v1768 } else { v13 });
        let v1772: f64 = (if v657 { v13 } else { v1730 });
        let v1773: f64 = (if v657 { v13 } else { v1731 });
        let v1774: f64 = (self.scalar_v17 / v171);
        let v1775: f64 = (v204 * v985);
        let v1776: f64 = (-v1775);
        let v1777: f64 = (v1776 / v1754);
        let v1778: f64 = (self.scalar_v859 / v171);
        let v1779: f64 = (-v1774);
        let v1780: f64 = (-v1777);
        let v1781: f64 = (-v1778);
        let v1782: f64 = (v1779 / v678);
        let v1783: f64 = (v1780 / v678);
        let v1784: f64 = (v1781 / v678);
        let v1785: f64 = (self.scalar_v666 * v1782);
        let v1786: f64 = (self.scalar_v666 * v1783);
        let v1787: f64 = (self.scalar_v666 * v1784);
        let v1788: f64 = (v681 * v1785);
        let v1789: f64 = (v681 * v1786);
        let v1790: f64 = (v681 * v1787);
        let v1791: f64 = (-v1788);
        let v1792: f64 = (-v1789);
        let v1793: f64 = (-v1790);
        let v1794: f64 = (v171 * v1791);
        let v1795: f64 = (v682 * v985);
        let v1796: f64 = (v171 * v1792);
        let v1797: f64 = (v1795 + v1796);
        let v1798: f64 = (v171 * v1793);
        let v1799: f64 = (v1794 / self.scalar_v666);
        let v1800: f64 = (v1797 / self.scalar_v666);
        let v1801: f64 = (v1798 / self.scalar_v666);
        let v1802: f64 = (if v676 { v1799 } else { v13 });
        let v1803: f64 = (if v676 { v1800 } else { v1744 });
        let v1804: f64 = (if v676 { v1801 } else { v13 });
        let v1805: f64 = (if v676 { v13 } else { v1745 });
        let v1806: f64 = (if v676 { v13 } else { v1746 });
        let v1807: f64 = (if v676 { v13 } else { v1769 });
        let v1808: f64 = (if v676 { v13 } else { v1770 });
        let v1809: f64 = (if v676 { v13 } else { v1771 });
        let v1810: f64 = (if v676 { v13 } else { v1772 });
        let v1811: f64 = (if v676 { v13 } else { v1773 });
        let v1812: f64 = (v1802 + v1807);
        let v1813: f64 = (v1803 + v1808);
        let v1814: f64 = (v1804 + v1809);
        let v1815: f64 = (v1805 + v1810);
        let v1816: f64 = (v1806 + v1811);
        let v1817: f64 = (v177 * v1812);
        let v1818: f64 = (v687 * v995);
        let v1819: f64 = (v177 * v1813);
        let v1820: f64 = (v1818 + v1819);
        let v1821: f64 = (v177 * v1814);
        let v1822: f64 = (v177 * v1815);
        let v1823: f64 = (v177 * v1816);
        let v1824: f64 = (self.scalar_v690 * v1817);
        let v1825: f64 = (self.scalar_v690 * v1820);
        let v1826: f64 = (self.scalar_v690 * v1821);
        let v1827: f64 = (self.scalar_v690 * v1822);
        let v1828: f64 = (self.scalar_v690 * v1823);
        let v1829: f64 = (v697 * v985);
        let v1830: f64 = (v1829 / self.scalar_v666);
        let v1831: f64 = (if v693 { v13 } else { v1802 });
        let v1832: f64 = (if v693 { v1830 } else { v1803 });
        let v1833: f64 = (if v693 { v13 } else { v1804 });
        let v1834: f64 = (if v693 { v13 } else { v1805 });
        let v1835: f64 = (if v693 { v13 } else { v1806 });
        let v1836: f64 = (v701 * v985);
        let v1837: f64 = (v1751 - v1836);
        let v1838: f64 = (v1837 / v1754);
        let v1839: f64 = (v703 * v1741);
        let v1840: f64 = (v692 * v1838);
        let v1841: f64 = (v1839 + v1840);
        let v1842: f64 = (v703 * self.scalar_v859);
        let v1843: f64 = (v692 * v1756);
        let v1844: f64 = (v1842 + v1843);
        let v1845: f64 = (self.scalar_v17 * v703);
        let v1846: f64 = (v692 * v1750);
        let v1847: f64 = (v1845 + v1846);
        let v1848: f64 = (v694 * v1841);
        let v1849: f64 = (v694 * v1844);
        let v1850: f64 = (v694 * v1847);
        let v1851: f64 = (if v693 { v13 } else { v1807 });
        let v1852: f64 = (if v693 { v1848 } else { v1808 });
        let v1853: f64 = (if v693 { v1849 } else { v1809 });
        let v1854: f64 = (if v693 { v1850 } else { v1810 });
        let v1855: f64 = (if v693 { v13 } else { v1811 });
        let v1856: f64 = (v21 * v985);
        let v1857: f64 = (-v1856);
        let v1858: f64 = (v1857 / v1754);
        let v1859: f64 = (-v1858);
        let v1860: f64 = (v1859 / v709);
        let v1861: f64 = (v1781 / v709);
        let v1862: f64 = (v1779 / v709);
        let v1863: f64 = (self.scalar_v666 * v1860);
        let v1864: f64 = (self.scalar_v666 * v1861);
        let v1865: f64 = (self.scalar_v666 * v1862);
        let v1866: f64 = (v712 * v1863);
        let v1867: f64 = (v712 * v1864);
        let v1868: f64 = (v712 * v1865);
        let v1869: f64 = (-v1866);
        let v1870: f64 = (-v1867);
        let v1871: f64 = (-v1868);
        let v1872: f64 = (v713 * v985);
        let v1873: f64 = (v171 * v1869);
        let v1874: f64 = (v1872 + v1873);
        let v1875: f64 = (v171 * v1870);
        let v1876: f64 = (v171 * v1871);
        let v1877: f64 = (v1874 / self.scalar_v666);
        let v1878: f64 = (v1875 / self.scalar_v666);
        let v1879: f64 = (v1876 / self.scalar_v666);
        let v1880: f64 = (if v707 { v13 } else { v1831 });
        let v1881: f64 = (if v707 { v1877 } else { v1832 });
        let v1882: f64 = (if v707 { v1878 } else { v1833 });
        let v1883: f64 = (if v707 { v1879 } else { v1834 });
        let v1884: f64 = (if v707 { v13 } else { v1835 });
        let v1885: f64 = (if v707 { v13 } else { v1851 });
        let v1886: f64 = (if v707 { v13 } else { v1852 });
        let v1887: f64 = (if v707 { v13 } else { v1853 });
        let v1888: f64 = (if v707 { v13 } else { v1854 });
        let v1889: f64 = (if v707 { v13 } else { v1855 });
        let v1890: f64 = (v1880 + v1885);
        let v1891: f64 = (v1881 + v1886);
        let v1892: f64 = (v1882 + v1887);
        let v1893: f64 = (v1883 + v1888);
        let v1894: f64 = (v1884 + v1889);
        let v1895: f64 = (v177 * v1890);
        let v1896: f64 = (v718 * v995);
        let v1897: f64 = (v177 * v1891);
        let v1898: f64 = (v1896 + v1897);
        let v1899: f64 = (v177 * v1892);
        let v1900: f64 = (v177 * v1893);
        let v1901: f64 = (v177 * v1894);
        let v1902: f64 = (self.scalar_v689 * v1895);
        let v1903: f64 = (self.scalar_v689 * v1898);
        let v1904: f64 = (self.scalar_v689 * v1899);
        let v1905: f64 = (self.scalar_v689 * v1900);
        let v1906: f64 = (self.scalar_v689 * v1901);
        let v1907: f64 = (self.scalar_v730 * v1556);
        let v1908: f64 = (self.scalar_v730 * v1557);
        let v1909: f64 = (self.scalar_v730 * v1560);
        let v1910: f64 = (self.scalar_v730 * v1563);
        let v1911: f64 = (if self.scalar_v724 { v1907 } else { v13 });
        let v1912: f64 = (if self.scalar_v724 { v1908 } else { v13 });
        let v1913: f64 = (if self.scalar_v724 { v1909 } else { v13 });
        let v1914: f64 = (if self.scalar_v724 { v1910 } else { v13 });
        let v1915: f64 = (if self.scalar_v733 { v13 } else { v1911 });
        let v1916: f64 = (if self.scalar_v733 { v13 } else { v1912 });
        let v1917: f64 = (if self.scalar_v733 { v13 } else { v1913 });
        let v1918: f64 = (if self.scalar_v733 { v13 } else { v1914 });
        let v1919: f64 = (v1608 / self.scalar_v16);
        let v1920: f64 = (v1610 / self.scalar_v16);
        let v1921: f64 = (v1609 / self.scalar_v16);
        let v1922: f64 = (v41 * v1121);
        let v1923: f64 = (v276 * v877);
        let v1924: f64 = (v1922 - v1923);
        let v1925: f64 = (v1924 / v1507);
        let v1926: f64 = (v276 * v878);
        let v1927: f64 = (-v1926);
        let v1928: f64 = (v1927 / v1507);
        let v1929: f64 = (v41 * v1122);
        let v1930: f64 = (v276 * v879);
        let v1931: f64 = (v1929 - v1930);
        let v1932: f64 = (v1931 / v1507);
        let v1933: f64 = (v1123 / v41);
        let v1934: f64 = (-v1925);
        let v1935: f64 = (-v1928);
        let v1936: f64 = (-v1932);
        let v1937: f64 = (-v1933);
        let v1938: f64 = (v567 * v1934);
        let v1939: f64 = (v567 * v1935);
        let v1940: f64 = (v567 * v1936);
        let v1941: f64 = (v567 * v1937);
        let v1942: f64 = (if self.scalar_v572 { v1938 } else { v13 });
        let v1943: f64 = (if self.scalar_v572 { v1939 } else { v13 });
        let v1944: f64 = (if self.scalar_v572 { v1940 } else { v13 });
        let v1945: f64 = (if self.scalar_v572 { v1941 } else { v13 });
        let v1954: f64 = -0.0;
        let v1955: f64 = (if v811 { v1919 } else { v13 });
        let v1956: f64 = (v12 / v812);
        let v1957: f64 = (v205 * v1955);
        let v1958: f64 = (-v1957);
        let v1959: f64 = (v812 * v812);
        let v1960: f64 = (v1958 / v1959);
        let v1961: f64 = (v284 / v812);
        let v1962: f64 = (if self.scalar_v758 { v1956 } else { v13 });
        let v1963: f64 = (if self.scalar_v758 { v1960 } else { v13 });
        let v1964: f64 = (if self.scalar_v758 { v1961 } else { v13 });
        let v1965: f64 = (if v818 { v1920 } else { v13 });
        let v1966: f64 = (v12 / v819);
        let v1967: f64 = (v207 * v1965);
        let v1968: f64 = (-v1967);
        let v1969: f64 = (v819 * v819);
        let v1970: f64 = (v1968 / v1969);
        let v1971: f64 = (v284 / v819);
        let v1972: f64 = (if self.scalar_v762 { v1966 } else { v13 });
        let v1973: f64 = (if self.scalar_v762 { v1970 } else { v13 });
        let v1974: f64 = (if self.scalar_v762 { v1971 } else { v13 });
        let v1975: f64 = (if v826 { v1921 } else { v13 });
        let v1976: f64 = (v12 / v827);
        let v1977: f64 = (v825 * v1975);
        let v1978: f64 = (-v1977);
        let v1979: f64 = (v827 * v827);
        let v1980: f64 = (v1978 / v1979);
        let v1981: f64 = (v284 / v827);
        let v1982: f64 = (if self.scalar_v766 { v1976 } else { v13 });
        let v1983: f64 = (if self.scalar_v766 { v1980 } else { v13 });
        let v1984: f64 = (if self.scalar_v766 { v1981 } else { v13 });
        let v1985: f64 = (self.scalar_v17 * v1517);
        let v1986: f64 = (self.scalar_v17 * v1511);
        let v1987: f64 = (self.scalar_v17 * v1518);
        let v1988: f64 = (self.scalar_v17 * v1519);
        let v1989: f64 = (self.scalar_v16 * v1985);
        let v1990: f64 = (self.scalar_v16 * v1986);
        let v1991: f64 = (self.scalar_v16 * v1987);
        let v1992: f64 = (self.scalar_v16 * v1988);
        let v1993: f64 = (self.scalar_v17 * v1528);
        let v1994: f64 = (self.scalar_v17 * v1529);
        let v1995: f64 = (self.scalar_v17 * v1530);
        let v1996: f64 = (self.scalar_v17 * v1531);
        let v1997: f64 = (self.scalar_v16 * v1993);
        let v1998: f64 = (self.scalar_v16 * v1994);
        let v1999: f64 = (self.scalar_v16 * v1995);
        let v2000: f64 = (self.scalar_v16 * v1996);
        let v2001: f64 = (-v1546);
        let v2002: f64 = (-v1549);
        let v2003: f64 = (-v1552);
        let v2004: f64 = (-v1555);
        let v2005: f64 = (self.scalar_v16 * v2001);
        let v2006: f64 = (self.scalar_v16 * v2002);
        let v2007: f64 = (self.scalar_v16 * v2003);
        let v2008: f64 = (self.scalar_v16 * v2004);
        let v2009: f64 = (self.scalar_v17 * v2005);
        let v2010: f64 = (self.scalar_v17 * v2006);
        let v2011: f64 = (self.scalar_v17 * v2007);
        let v2012: f64 = (self.scalar_v17 * v2008);
        let v2013: f64 = (self.scalar_v17 * v1583);
        let v2014: f64 = (self.scalar_v17 * v1584);
        let v2015: f64 = (self.scalar_v17 * v1585);
        let v2016: f64 = (self.scalar_v17 * v1586);
        let v2017: f64 = (self.scalar_v16 * v2013);
        let v2018: f64 = (self.scalar_v16 * v2014);
        let v2019: f64 = (self.scalar_v16 * v2015);
        let v2020: f64 = (self.scalar_v16 * v2016);
        let v2021: f64 = (self.scalar_v17 * v1737);
        let v2022: f64 = (self.scalar_v17 * v1738);
        let v2023: f64 = (self.scalar_v17 * v1739);
        let v2024: f64 = (self.scalar_v16 * v2021);
        let v2025: f64 = (self.scalar_v16 * v2022);
        let v2026: f64 = (self.scalar_v16 * v2023);
        let v2027: f64 = (self.scalar_v17 * v1598);
        let v2028: f64 = (self.scalar_v17 * v1599);
        let v2029: f64 = (self.scalar_v17 * v1600);
        let v2030: f64 = (self.scalar_v16 * v2027);
        let v2031: f64 = (self.scalar_v16 * v2028);
        let v2032: f64 = (self.scalar_v16 * v2029);
        let v2033: f64 = (self.scalar_v17 * v1824);
        let v2034: f64 = (self.scalar_v17 * v1825);
        let v2035: f64 = (self.scalar_v17 * v1826);
        let v2036: f64 = (self.scalar_v17 * v1827);
        let v2037: f64 = (self.scalar_v17 * v1828);
        let v2038: f64 = (self.scalar_v16 * v2033);
        let v2039: f64 = (self.scalar_v16 * v2034);
        let v2040: f64 = (self.scalar_v16 * v2035);
        let v2041: f64 = (self.scalar_v16 * v2036);
        let v2042: f64 = (self.scalar_v16 * v2037);
        let v2043: f64 = (self.scalar_v17 * v1902);
        let v2044: f64 = (self.scalar_v17 * v1903);
        let v2045: f64 = (self.scalar_v17 * v1904);
        let v2046: f64 = (self.scalar_v17 * v1905);
        let v2047: f64 = (self.scalar_v17 * v1906);
        let v2048: f64 = (self.scalar_v16 * v2043);
        let v2049: f64 = (self.scalar_v16 * v2044);
        let v2050: f64 = (self.scalar_v16 * v2045);
        let v2051: f64 = (self.scalar_v16 * v2046);
        let v2052: f64 = (self.scalar_v16 * v2047);
        let v2053: f64 = (self.scalar_v17 * v1601);
        let v2054: f64 = (self.scalar_v17 * v1602);
        let v2055: f64 = (self.scalar_v17 * v1603);
        let v2056: f64 = (self.scalar_v17 * v1604);
        let v2057: f64 = (self.scalar_v16 * v2053);
        let v2058: f64 = (self.scalar_v16 * v2054);
        let v2059: f64 = (self.scalar_v16 * v2055);
        let v2060: f64 = (self.scalar_v16 * v2056);
        let v2061: f64 = (self.scalar_v17 * v1665);
        let v2062: f64 = (self.scalar_v17 * v1666);
        let v2063: f64 = (self.scalar_v17 * v1667);
        let v2064: f64 = (self.scalar_v16 * v2061);
        let v2065: f64 = (self.scalar_v16 * v2062);
        let v2066: f64 = (self.scalar_v16 * v2063);
        let v2067: f64 = (-v1915);
        let v2068: f64 = (-v1916);
        let v2069: f64 = (-v1917);
        let v2070: f64 = (-v1918);
        let v2071: f64 = (self.scalar_v16 * v2067);
        let v2072: f64 = (self.scalar_v16 * v2068);
        let v2073: f64 = (self.scalar_v16 * v2069);
        let v2074: f64 = (self.scalar_v16 * v2070);
        let v2075: f64 = (self.scalar_v16 * v1915);
        let v2076: f64 = (self.scalar_v16 * v1916);
        let v2077: f64 = (self.scalar_v16 * v1917);
        let v2078: f64 = (self.scalar_v16 * v1918);

        let d769_dn5: f64 = self.scalar_v859;
        let d769_dn6: f64 = self.scalar_v17;
        let d769_dn9: f64 = v12;
        stamper.stamp_current_node3_local(
            Some(9),
            None,
            multiplicity * (v769),
            5,
            multiplicity * (d769_dn5),
            6,
            multiplicity * (d769_dn6),
            9,
            multiplicity * (d769_dn9),
        );
        let d771_dn9: f64 = v770;
        stamper.stamp_current_node1_local(
            Some(9),
            None,
            multiplicity * (v771),
            9,
            multiplicity * (d771_dn9),
        );
        let d775_dn3: f64 = v1942;
        let d775_dn4: f64 = v1943;
        let d775_dn5: f64 = v1944;
        let d775_dn6: f64 = v1945;
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(8),
            None,
            multiplicity * (v775),
            [3, 4, 5, 6],
            [d775_dn3, d775_dn4, d775_dn5, d775_dn6],
            [],
            [],
            multiplicity,
        );
        let d776_dn8: f64 = self.scalar_v1946;
        stamper.stamp_current_node1_local(
            Some(8),
            None,
            multiplicity * (v776),
            8,
            multiplicity * (d776_dn8),
        );
        stamper.stamp_potential_branch_local(
            Some(8),
            None,
            0,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            0,
            self.scalar_v777,
        );
        stamper.stamp_current_const_local(
            Some(3),
            None,
            multiplicity * (v786),
        );
        let d788_dn3: f64 = self.scalar_v1948;
        stamper.stamp_current_node1_local(
            Some(3),
            None,
            multiplicity * (v788),
            3,
            multiplicity * (d788_dn3),
        );
        stamper.stamp_potential_branch_local(
            Some(7),
            None,
            1,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            1,
            self.scalar_v789,
        );
        stamper.stamp_current_const_local(
            Some(3),
            None,
            multiplicity * (v792),
        );
        let d796_dn3: f64 = self.scalar_v1950;
        let d796_dn7: f64 = self.scalar_v1951;
        stamper.stamp_current_node2_local(
            Some(3),
            Some(7),
            multiplicity * (v796),
            3,
            multiplicity * (d796_dn3),
            7,
            multiplicity * (d796_dn7),
        );
        let d798_dn7: f64 = self.scalar_v1953;
        stamper.stamp_current_node1_local(
            Some(7),
            None,
            multiplicity * (v798),
            7,
            multiplicity * (d798_dn7),
        );
        stamper.stamp_current_const_local(
            Some(3),
            None,
            multiplicity * (v802),
        );
        stamper.stamp_potential_branch_local(
            Some(7),
            None,
            2,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            2,
            self.scalar_v803,
        );
        stamper.stamp_potential_branch_local(
            Some(3),
            None,
            3,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            3,
            self.scalar_v806,
        );
        stamper.stamp_potential_branch_local(
            Some(7),
            None,
            4,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            4,
            self.scalar_v806,
        );
        let d807_dn5: f64 = v13;
        let d807_dn6: f64 = v1954;
        stamper.stamp_current_node2_local(
            Some(5),
            Some(6),
            multiplicity * (v807),
            5,
            multiplicity * (d807_dn5),
            6,
            multiplicity * (d807_dn6),
        );
        let d808_dn4: f64 = v1954;
        let d808_dn5: f64 = v13;
        stamper.stamp_current_node2_local(
            Some(5),
            Some(4),
            multiplicity * (v808),
            4,
            multiplicity * (d808_dn4),
            5,
            multiplicity * (d808_dn5),
        );
        let d810_dn4: f64 = v13;
        let d810_dn6: f64 = v1954;
        stamper.stamp_current_node2_local(
            Some(4),
            Some(6),
            multiplicity * (v810),
            4,
            multiplicity * (d810_dn4),
            6,
            multiplicity * (d810_dn6),
        );
        let d814_dn1: f64 = v1962;
        let d814_dn3: f64 = v1963;
        let d814_dn5: f64 = v1964;
        stamper.stamp_current_node3_local(
            Some(1),
            Some(5),
            multiplicity * (v814),
            1,
            multiplicity * (d814_dn1),
            3,
            multiplicity * (d814_dn3),
            5,
            multiplicity * (d814_dn5),
        );
        stamper.stamp_current_const_local(
            Some(1),
            Some(5),
            multiplicity * (self.scalar_v815),
        );
        stamper.stamp_potential_branch_local(
            Some(1),
            Some(5),
            5,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            5,
            self.scalar_v817,
        );
        let d821_dn2: f64 = v1972;
        let d821_dn3: f64 = v1973;
        let d821_dn6: f64 = v1974;
        stamper.stamp_current_node3_local(
            Some(2),
            Some(6),
            multiplicity * (v821),
            2,
            multiplicity * (d821_dn2),
            3,
            multiplicity * (d821_dn3),
            6,
            multiplicity * (d821_dn6),
        );
        stamper.stamp_current_const_local(
            Some(2),
            Some(6),
            multiplicity * (self.scalar_v822),
        );
        stamper.stamp_potential_branch_local(
            Some(2),
            Some(6),
            6,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            6,
            self.scalar_v824,
        );
        let d829_dn0: f64 = v1982;
        let d829_dn3: f64 = v1983;
        let d829_dn4: f64 = v1984;
        stamper.stamp_current_node3_local(
            Some(0),
            Some(4),
            multiplicity * (v829),
            0,
            multiplicity * (d829_dn0),
            3,
            multiplicity * (d829_dn3),
            4,
            multiplicity * (d829_dn4),
        );
        stamper.stamp_current_const_local(
            Some(0),
            Some(4),
            multiplicity * (self.scalar_v830),
        );
        stamper.stamp_potential_branch_local(
            Some(0),
            Some(4),
            7,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            7,
            self.scalar_v832,
        );
        let d834_dn3: f64 = v1989;
        let d834_dn4: f64 = v1990;
        let d834_dn5: f64 = v1991;
        let d834_dn6: f64 = v1992;
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(5),
            Some(6),
            multiplicity * (v834),
            [3, 4, 5, 6],
            [d834_dn3, d834_dn4, d834_dn5, d834_dn6],
            [],
            [],
            multiplicity,
        );
        let d836_dn3: f64 = v1997;
        let d836_dn4: f64 = v1998;
        let d836_dn5: f64 = v1999;
        let d836_dn6: f64 = v2000;
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(5),
            Some(4),
            multiplicity * (v836),
            [3, 4, 5, 6],
            [d836_dn3, d836_dn4, d836_dn5, d836_dn6],
            [],
            [],
            multiplicity,
        );
        let d839_dn3: f64 = v2009;
        let d839_dn4: f64 = v2010;
        let d839_dn5: f64 = v2011;
        let d839_dn6: f64 = v2012;
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(4),
            Some(6),
            multiplicity * (v839),
            [3, 4, 5, 6],
            [d839_dn3, d839_dn4, d839_dn5, d839_dn6],
            [],
            [],
            multiplicity,
        );
        let d841_dn3: f64 = v2017;
        let d841_dn4: f64 = v2018;
        let d841_dn5: f64 = v2019;
        let d841_dn6: f64 = v2020;
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(4),
            Some(6),
            multiplicity * (v841),
            [3, 4, 5, 6],
            [d841_dn3, d841_dn4, d841_dn5, d841_dn6],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_const_local(
            Some(5),
            Some(6),
            multiplicity * (v13),
        );
        stamper.stamp_current_const_local(
            Some(5),
            Some(6),
            multiplicity * (v13),
        );
        stamper.stamp_current_const_local(
            Some(4),
            Some(6),
            multiplicity * (v13),
        );
        let d843_dn3: f64 = v2024;
        let d843_dn5: f64 = v2025;
        let d843_dn6: f64 = v2026;
        let v843_ddt: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 5, v843);
        stamper.stamp_current_node3_local(
            Some(5),
            Some(6),
            multiplicity * (v843_ddt),
            3,
            multiplicity * (((d843_dn3) * ddt_scale)),
            5,
            multiplicity * (((d843_dn5) * ddt_scale)),
            6,
            multiplicity * (((d843_dn6) * ddt_scale)),
        );
        let d845_dn3: f64 = v2030;
        let d845_dn5: f64 = v2031;
        let d845_dn6: f64 = v2032;
        let v845_ddt: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 6, v845);
        stamper.stamp_current_node3_local(
            Some(5),
            Some(6),
            multiplicity * (v845_ddt),
            3,
            multiplicity * (((d845_dn3) * ddt_scale)),
            5,
            multiplicity * (((d845_dn5) * ddt_scale)),
            6,
            multiplicity * (((d845_dn6) * ddt_scale)),
        );
        let d847_dn1: f64 = v2038;
        let d847_dn3: f64 = v2039;
        let d847_dn4: f64 = v2040;
        let d847_dn5: f64 = v2041;
        let d847_dn6: f64 = v2042;
        let v847_ddt: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 7, v847);
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(1),
            Some(4),
            multiplicity * (v847_ddt),
            [1, 3, 4, 5, 6],
            [((d847_dn1) * ddt_scale), ((d847_dn3) * ddt_scale), ((d847_dn4) * ddt_scale), ((d847_dn5) * ddt_scale), ((d847_dn6) * ddt_scale)],
            [],
            [],
            multiplicity,
        );
        let d849_dn1: f64 = v2048;
        let d849_dn3: f64 = v2049;
        let d849_dn4: f64 = v2050;
        let d849_dn5: f64 = v2051;
        let d849_dn6: f64 = v2052;
        let v849_ddt: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 8, v849);
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(5),
            Some(4),
            multiplicity * (v849_ddt),
            [1, 3, 4, 5, 6],
            [((d849_dn1) * ddt_scale), ((d849_dn3) * ddt_scale), ((d849_dn4) * ddt_scale), ((d849_dn5) * ddt_scale), ((d849_dn6) * ddt_scale)],
            [],
            [],
            multiplicity,
        );
        let d851_dn3: f64 = v2057;
        let d851_dn4: f64 = v2058;
        let d851_dn5: f64 = v2059;
        let d851_dn6: f64 = v2060;
        let v851_ddt: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 9, v851);
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(5),
            Some(4),
            multiplicity * (v851_ddt),
            [3, 4, 5, 6],
            [((d851_dn3) * ddt_scale), ((d851_dn4) * ddt_scale), ((d851_dn5) * ddt_scale), ((d851_dn6) * ddt_scale)],
            [],
            [],
            multiplicity,
        );
        let d853_dn2: f64 = v2064;
        let d853_dn3: f64 = v2065;
        let d853_dn4: f64 = v2066;
        let v853_ddt: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 10, v853);
        stamper.stamp_current_node3_local(
            Some(2),
            Some(4),
            multiplicity * (v853_ddt),
            2,
            multiplicity * (((d853_dn2) * ddt_scale)),
            3,
            multiplicity * (((d853_dn3) * ddt_scale)),
            4,
            multiplicity * (((d853_dn4) * ddt_scale)),
        );
        let d855_dn3: f64 = v2071;
        let d855_dn4: f64 = v2072;
        let d855_dn5: f64 = v2073;
        let d855_dn6: f64 = v2074;
        let v855_ddt: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 11, v855);
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(5),
            Some(6),
            multiplicity * (v855_ddt),
            [3, 4, 5, 6],
            [((d855_dn3) * ddt_scale), ((d855_dn4) * ddt_scale), ((d855_dn5) * ddt_scale), ((d855_dn6) * ddt_scale)],
            [],
            [],
            multiplicity,
        );
        let d856_dn3: f64 = v2075;
        let d856_dn4: f64 = v2076;
        let d856_dn5: f64 = v2077;
        let d856_dn6: f64 = v2078;
        let v856_ddt: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 12, v856);
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(5),
            Some(4),
            multiplicity * (v856_ddt),
            [3, 4, 5, 6],
            [((d856_dn3) * ddt_scale), ((d856_dn4) * ddt_scale), ((d856_dn5) * ddt_scale), ((d856_dn6) * ddt_scale)],
            [],
            [],
            multiplicity,
        );
        let mut locals = StampLocals::default();

        let assign1450_e1782: f64 = ((nv1 - nv2) / p.p40);
        let assign1450_e1783: f64 = (assign1450_e1782).abs();
        let assign1450_e1785: f64 = (assign1450_e1783).powf(p.p39);
        locals.var_vtff = assign1450_e1785;
        locals.var_vtff_dn1 = if 0.0 == 0.0 && ((p.p39) as f64).is_finite() && ((p.p39) as f64).fract() == 0.0 { if p.p39 == 0.0 { 0.0 } else { (p.p39 * ((assign1450_e1783).powf(p.p39 - 1.0) * if assign1450_e1782 >= 0.0 { (1.0 / p.p40) } else { (-(1.0 / p.p40)) })) } } else { (assign1450_e1785 * (p.p39 * (if assign1450_e1782 >= 0.0 { (1.0 / p.p40) } else { (-(1.0 / p.p40)) } / assign1450_e1783))) };
        locals.var_vtff_dn2 = if 0.0 == 0.0 && ((p.p39) as f64).is_finite() && ((p.p39) as f64).fract() == 0.0 { if p.p39 == 0.0 { 0.0 } else { (p.p39 * ((assign1450_e1783).powf(p.p39 - 1.0) * if assign1450_e1782 >= 0.0 { (-1.0 / p.p40) } else { (-(-1.0 / p.p40)) })) } } else { (assign1450_e1785 * (p.p39 * (if assign1450_e1782 >= 0.0 { (-1.0 / p.p40) } else { (-(-1.0 / p.p40)) } / assign1450_e1783))) };

        let assign1460_e1788: f64 = (1.0 + locals.var_vtff);
        let assign1460_e1791: f64 = (1.0 / p.p39);
        let assign1460_e1792: f64 = (assign1460_e1788).powf(assign1460_e1791);
        let assign1460_e1794: f64 = (assign1460_e1792 - 1.0);
        locals.var_vtff1 = assign1460_e1794;
        locals.var_vtff1_dn1 = if 0.0 == 0.0 && ((assign1460_e1791) as f64).is_finite() && ((assign1460_e1791) as f64).fract() == 0.0 { if assign1460_e1791 == 0.0 { 0.0 } else { (assign1460_e1791 * ((assign1460_e1788).powf(assign1460_e1791 - 1.0) * locals.var_vtff_dn1)) } } else { (assign1460_e1792 * (assign1460_e1791 * (locals.var_vtff_dn1 / assign1460_e1788))) };
        locals.var_vtff1_dn2 = if 0.0 == 0.0 && ((assign1460_e1791) as f64).is_finite() && ((assign1460_e1791) as f64).fract() == 0.0 { if assign1460_e1791 == 0.0 { 0.0 } else { (assign1460_e1791 * ((assign1460_e1788).powf(assign1460_e1791 - 1.0) * locals.var_vtff_dn2)) } } else { (assign1460_e1792 * (assign1460_e1791 * (locals.var_vtff_dn2 / assign1460_e1788))) };

        let assign1470_e1799: f64 = (p.p41 * locals.var_vtff1);
        let assign1470_e1800: f64 = (1.0 + assign1470_e1799);
        let assign1470_e1801: f64 = (p.p19 * assign1470_e1800);
        locals.var_tff = assign1470_e1801;
        locals.var_tff_dn1 = (p.p19 * (p.p41 * locals.var_vtff1_dn1));
        locals.var_tff_dn2 = (p.p19 * (p.p41 * locals.var_vtff1_dn2));

        let assign1500_e1810: f64 = if p.p32 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard13 = assign1500_e1810;

        let assign1920_e2216: f64 = if ((p.p30 == 1.0) && (p.p33 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard20 = assign1920_e2216;

        let assign1930_e2227: f64 = if (((p.p30 == 2.0) && (p.p33 > 0.0)) && (p.p35 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard21 = assign1930_e2227;

        Self::stamp_transient_equations_block_0(ctx, stamper, p, nodes, multiplicity, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, &mut locals);
    }

    pub fn stamp_reactive(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedReactiveStamper<'_>) {
        let p = Box::as_ref(&self.params);
        let nodes = &(*self).nodes;
        let branches = &(*self).branches;
        let ctx_temp = ctx.temperature();
        let nv1 = ctx.node_voltage(nodes[1]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv4 = ctx.node_voltage(nodes[4]);
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let multiplicity = (*self).multiplicity;
        let v0: f64 = ctx_temp;
        let v1: f64 = nv3;
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
        let v18: f64 = nv5;
        let v19: f64 = nv4;
        let v20: f64 = (v18 - v19);
        let v21: f64 = (self.scalar_v17 * v20);
        let v32: f64 = 8.6170869e-5;
        let v33: f64 = (v11 * v32);
        let v34: f64 = (v11 / self.scalar_v31);
        let v35: f64 = ((v34) as f64).ln();
        let v37: f64 = (v35 * self.scalar_v36);
        let v38: f64 = ((v37) as f64).exp();
        let v61: f64 = (v35 * self.scalar_v60);
        let v63: f64 = (v34 - v12);
        let v64: f64 = (self.scalar_v62 * v63);
        let v65: f64 = (v64 / v33);
        let v66: f64 = (v61 + v65);
        let v68: f64 = (v35 * self.scalar_v67);
        let v70: f64 = ((v66) as f64).exp();
        let v71: f64 = (self.scalar_v69 * v70);
        let v73: f64 = ((v68) as f64).exp();
        let v74: f64 = (self.scalar_v72 * v73);
        let v77: f64 = (v66 / self.scalar_v76);
        let v78: f64 = ((v77) as f64).exp();
        let v79: f64 = (self.scalar_v75 * v78);
        let v80: f64 = (v79 / v38);
        let v89: f64 = (v63 * self.scalar_v88);
        let v90: f64 = (v12 + v89);
        let v91: f64 = (self.scalar_v87 * v90);
        let v94: f64 = (v63 * self.scalar_v93);
        let v95: f64 = (v12 + v94);
        let v96: f64 = (self.scalar_v92 * v95);
        let v99: f64 = (v63 * self.scalar_v98);
        let v100: f64 = (v12 + v99);
        let v101: f64 = (self.scalar_v97 * v100);
        let v104: f64 = (v63 * self.scalar_v103);
        let v105: f64 = (v12 + v104);
        let v106: f64 = (self.scalar_v102 * v105);
        let v110: f64 = 300.15;
        let v112: f64 = (v11 / v110);
        let v113: f64 = 1.16;
        let v114: f64 = 0.000702;
        let v115: f64 = (v11 * v114);
        let v116: f64 = (v11 * v115);
        let v117: f64 = 1108.0;
        let v118: f64 = (v11 + v117);
        let v119: f64 = (v116 / v118);
        let v120: f64 = (v113 - v119);
        let v121: f64 = (-v120);
        let v122: f64 = 1.3806226e-23;
        let v123: f64 = (v11 + v11);
        let v124: f64 = (v122 * v123);
        let v125: f64 = (v121 / v124);
        let v126: f64 = 1.3454442398941469e20;
        let v127: f64 = (v125 + v126);
        let v128: f64 = (v33 + v33);
        let v129: f64 = (-v128);
        let v130: f64 = 1.5;
        let v131: f64 = ((v112) as f64).ln();
        let v132: f64 = (v130 * v131);
        let v133: f64 = 1.6021918e-19;
        let v134: f64 = (v127 * v133);
        let v135: f64 = (v132 + v134);
        let v136: f64 = (v129 * v135);
        let v138: f64 = (self.scalar_v137 - v136);
        let v139: f64 = (v138 / self.scalar_v111);
        let v140: f64 = (self.scalar_v137 - v139);
        let v141: f64 = (v140 / v139);
        let v143: f64 = 0.0004;
        let v146: f64 = (self.scalar_v145 - v141);
        let v147: f64 = (self.scalar_v142 * v146);
        let v148: f64 = (v12 + v147);
        let v149: f64 = (self.scalar_v107 / v148);
        let v150: f64 = (v112 * v139);
        let v151: f64 = (v136 + v150);
        let v152: f64 = (v151 - v139);
        let v153: f64 = (v152 / v139);
        let v154: f64 = (v11 - v110);
        let v155: f64 = (v143 * v154);
        let v156: f64 = (v155 - v153);
        let v157: f64 = (self.scalar_v142 * v156);
        let v158: f64 = (v12 + v157);
        let v159: f64 = (v149 * v158);
        let v161: f64 = (self.scalar_v160 - v136);
        let v162: f64 = (v161 / self.scalar_v111);
        let v163: f64 = (self.scalar_v160 - v162);
        let v164: f64 = (v163 / v162);
        let v166: f64 = (self.scalar_v145 - v164);
        let v167: f64 = (self.scalar_v165 * v166);
        let v168: f64 = (v12 + v167);
        let v169: f64 = (self.scalar_v108 / v168);
        let v170: f64 = (v112 * v162);
        let v171: f64 = (v136 + v170);
        let v172: f64 = (v171 - v162);
        let v173: f64 = (v172 / v162);
        let v174: f64 = (v155 - v173);
        let v175: f64 = (self.scalar_v165 * v174);
        let v176: f64 = (v12 + v175);
        let v177: f64 = (v169 * v176);
        let v179: f64 = (self.scalar_v178 - v136);
        let v180: f64 = (v179 / self.scalar_v111);
        let v181: f64 = (self.scalar_v178 - v180);
        let v182: f64 = (v181 / v180);
        let v184: f64 = (self.scalar_v145 - v182);
        let v185: f64 = (self.scalar_v183 * v184);
        let v186: f64 = (v12 + v185);
        let v187: f64 = (self.scalar_v109 / v186);
        let v188: f64 = (v112 * v180);
        let v189: f64 = (v136 + v188);
        let v190: f64 = (v189 - v180);
        let v191: f64 = (v190 / v180);
        let v192: f64 = (v155 - v191);
        let v193: f64 = (self.scalar_v183 * v192);
        let v194: f64 = (v12 + v193);
        let v195: f64 = (v187 * v194);
        let v196: f64 = nv2;
        let v197: f64 = (v196 - v19);
        let v198: f64 = (self.scalar_v17 * v197);
        let v199: f64 = nv6;
        let v200: f64 = (v18 - v199);
        let v201: f64 = (self.scalar_v17 * v200);
        let v202: f64 = nv1;
        let v203: f64 = (v202 - v19);
        let v204: f64 = (self.scalar_v17 * v203);
        let v209: bool = (v71 > v13);
        let v211: f64 = (v33 * self.scalar_v210);
        let v212: f64 = (v201 / v211);
        let v213: f64 = (if v209 { v212 } else { v13 });
        let v214: f64 = (-v201);
        let v215: f64 = (v214 - v96);
        let v217: f64 = (v33 * self.scalar_v216);
        let v218: f64 = (v215 / v217);
        let v219: f64 = (if v209 { v218 } else { v13 });
        let v220: f64 = (-v96);
        let v221: f64 = (v220 / v217);
        let v222: f64 = (if v209 { v221 } else { v13 });
        let v223: f64 = 80.0;
        let v224: bool = (v213 > v223);
        let v225: bool = (v209 && v224);
        let v226: f64 = (v213 - v223);
        let v227: f64 = (v12 + v226);
        let v228: f64 = (if v225 { v227 } else { v13 });
        let v229: f64 = (if v225 { v223 } else { v213 });
        let v230: bool = (!v224);
        let v231: bool = (v209 && v230);
        let v232: f64 = (if v231 { v12 } else { v228 });
        let v233: f64 = ((v229) as f64).exp();
        let v234: f64 = (v232 * v233);
        let v235: f64 = (if v209 { v234 } else { v232 });
        let v236: f64 = 37.0;
        let v237: bool = (v219 >= v236);
        let v238: bool = (!v237);
        let v239: f64 = -37.0;
        let v240: bool = (v219 <= v239);
        let v241: bool = (!v240);
        let v242: bool = (v238 && v241);
        let v243: f64 = ((v219) as f64).exp();
        let v244: f64 = (v12 + v243);
        let v245: f64 = ((v244) as f64).ln();
        let v246: bool = (v238 && v240);
        let v247: f64 = (if v237 { v219 } else { v13 });
        let v248: f64 = (if v246 { v243 } else { v247 });
        let v249: f64 = (if v242 { v245 } else { v248 });
        let v250: bool = (v222 >= v236);
        let v251: bool = (!v250);
        let v252: bool = (v222 <= v239);
        let v253: bool = (!v252);
        let v254: bool = (v251 && v253);
        let v255: f64 = ((v222) as f64).exp();
        let v256: f64 = (v12 + v255);
        let v257: f64 = ((v256) as f64).ln();
        let v258: bool = (v251 && v252);
        let v259: f64 = (if v250 { v222 } else { v13 });
        let v260: f64 = (if v258 { v255 } else { v259 });
        let v261: f64 = (if v254 { v257 } else { v260 });
        let v262: f64 = (v249 - v261);
        let v263: f64 = (if v209 { v262 } else { v13 });
        let v264: f64 = (v235 - v12);
        let v265: f64 = (v71 * v264);
        let v266: f64 = (v91 * v263);
        let v268: f64 = ((v201) as f64).abs();
        let v269: f64 = f64::powf(v268, v101);
        let v270: f64 = (self.scalar_v267 * v269);
        let v271: f64 = (v12 + v270);
        let v272: f64 = (v266 / v271);
        let v273: f64 = (v265 - v272);
        let v274: f64 = (if v209 { v273 } else { v13 });
        let v275: bool = (!v209);
        let v276: f64 = (if v275 { v13 } else { v274 });
        let v277: bool = (v74 > v13);
        let v279: f64 = (self.scalar_v278 - v201);
        let v280: f64 = 0.001;
        let v281: bool = (v279 > v280);
        let v282: f64 = (if v281 { v279 } else { v280 });
        let v283: f64 = (if v277 { v282 } else { v13 });
        let v285: f64 = (v214 * self.scalar_v278);
        let v287: f64 = (v33 * self.scalar_v286);
        let v288: f64 = (v283 * v287);
        let v289: f64 = (v285 / v288);
        let v290: f64 = (if v277 { v289 } else { v229 });
        let v291: bool = (v290 > v223);
        let v292: bool = (v277 && v291);
        let v293: f64 = (v290 - v223);
        let v294: f64 = (v12 + v293);
        let v295: f64 = (if v292 { v294 } else { v235 });
        let v296: f64 = (if v292 { v223 } else { v290 });
        let v297: bool = (!v291);
        let v298: bool = (v277 && v297);
        let v299: f64 = (if v298 { v12 } else { v295 });
        let v300: f64 = ((v296) as f64).exp();
        let v301: f64 = (v299 * v300);
        let v302: f64 = (if v277 { v301 } else { v299 });
        let v308: bool = (v80 > v13);
        let v309: f64 = (v33 * self.scalar_v76);
        let v310: f64 = (v201 / v309);
        let v311: f64 = (if v308 { v310 } else { v296 });
        let v313: f64 = (v33 * self.scalar_v312);
        let v314: f64 = (v215 / v313);
        let v315: f64 = (if v308 { v314 } else { v219 });
        let v316: f64 = (v220 / v313);
        let v317: f64 = (if v308 { v316 } else { v222 });
        let v318: bool = (v311 > v223);
        let v319: bool = (v308 && v318);
        let v320: f64 = (v311 - v223);
        let v321: f64 = (v12 + v320);
        let v322: f64 = (if v319 { v321 } else { v302 });
        let v323: f64 = (if v319 { v223 } else { v311 });
        let v324: bool = (!v318);
        let v325: bool = (v308 && v324);
        let v326: f64 = (if v325 { v12 } else { v322 });
        let v327: f64 = ((v323) as f64).exp();
        let v328: f64 = (v326 * v327);
        let v329: f64 = (if v308 { v328 } else { v326 });
        let v330: bool = (v315 >= v236);
        let v331: bool = (!v330);
        let v332: bool = (v315 <= v239);
        let v333: bool = (!v332);
        let v334: bool = (v331 && v333);
        let v335: f64 = ((v315) as f64).exp();
        let v336: f64 = (v12 + v335);
        let v337: f64 = ((v336) as f64).ln();
        let v338: bool = (v331 && v332);
        let v339: f64 = (if v330 { v315 } else { v13 });
        let v340: f64 = (if v338 { v335 } else { v339 });
        let v341: f64 = (if v334 { v337 } else { v340 });
        let v342: bool = (v317 >= v236);
        let v343: bool = (!v342);
        let v344: bool = (v317 <= v239);
        let v345: bool = (!v344);
        let v346: bool = (v343 && v345);
        let v347: f64 = ((v317) as f64).exp();
        let v348: f64 = (v12 + v347);
        let v349: f64 = ((v348) as f64).ln();
        let v350: bool = (v343 && v344);
        let v351: f64 = (if v342 { v317 } else { v13 });
        let v352: f64 = (if v350 { v347 } else { v351 });
        let v353: f64 = (if v346 { v349 } else { v352 });
        let v354: f64 = (v341 - v353);
        let v355: f64 = (if v308 { v354 } else { v263 });
        let v365: f64 = (v33 * self.scalar_v364);
        let v366: f64 = (v21 / v365);
        let v367: f64 = (if v209 { v366 } else { v323 });
        let v368: f64 = (-v21);
        let v369: f64 = (v368 - v96);
        let v370: f64 = (v369 / v313);
        let v371: f64 = (if v209 { v370 } else { v315 });
        let v372: f64 = (if v209 { v316 } else { v317 });
        let v373: bool = (v367 > v223);
        let v374: bool = (v209 && v373);
        let v375: f64 = (v367 - v223);
        let v376: f64 = (v12 + v375);
        let v377: f64 = (if v374 { v376 } else { v329 });
        let v378: f64 = (if v374 { v223 } else { v367 });
        let v379: bool = (!v373);
        let v380: bool = (v209 && v379);
        let v381: f64 = (if v380 { v12 } else { v377 });
        let v382: f64 = ((v378) as f64).exp();
        let v383: f64 = (v381 * v382);
        let v384: f64 = (if v209 { v383 } else { v381 });
        let v385: bool = (v371 >= v236);
        let v386: bool = (!v385);
        let v387: bool = (v371 <= v239);
        let v388: bool = (!v387);
        let v389: bool = (v386 && v388);
        let v390: f64 = ((v371) as f64).exp();
        let v391: f64 = (v12 + v390);
        let v392: f64 = ((v391) as f64).ln();
        let v393: bool = (v386 && v387);
        let v394: f64 = (if v385 { v371 } else { v13 });
        let v395: f64 = (if v393 { v390 } else { v394 });
        let v396: f64 = (if v389 { v392 } else { v395 });
        let v397: bool = (v372 >= v236);
        let v398: bool = (!v397);
        let v399: bool = (v372 <= v239);
        let v400: bool = (!v399);
        let v401: bool = (v398 && v400);
        let v402: f64 = ((v372) as f64).exp();
        let v403: f64 = (v12 + v402);
        let v404: f64 = ((v403) as f64).ln();
        let v405: bool = (v398 && v399);
        let v406: f64 = (if v397 { v372 } else { v13 });
        let v407: f64 = (if v405 { v402 } else { v406 });
        let v408: f64 = (if v401 { v404 } else { v407 });
        let v409: f64 = (v396 - v408);
        let v410: f64 = (if v209 { v409 } else { v355 });
        let v411: f64 = (v384 - v12);
        let v412: f64 = (v71 * v411);
        let v413: f64 = (v106 * v410);
        let v414: f64 = ((v21) as f64).abs();
        let v415: f64 = f64::powf(v414, v101);
        let v416: f64 = (self.scalar_v267 * v415);
        let v417: f64 = (v12 + v416);
        let v418: f64 = (v413 / v417);
        let v419: f64 = (v412 - v418);
        let v420: f64 = (if v209 { v419 } else { v13 });
        let v421: f64 = (if v275 { v13 } else { v420 });
        let v491: f64 = (v21 * self.scalar_v490);
        let v492: f64 = (v12 + v491);
        let v493: f64 = (self.scalar_v55 * v492);
        let v494: f64 = (v276 * v493);
        let v495: f64 = (self.scalar_v59 * v421);
        let v496: f64 = (v494 + v495);
        let v497: f64 = (self.scalar_v51 * v201);
        let v498: f64 = (v12 - v497);
        let v499: f64 = (v21 * self.scalar_v47);
        let v500: f64 = (v498 - v499);
        let v501: f64 = 4.0;
        let v502: f64 = (v496 * v501);
        let v503: f64 = (v12 + v502);
        let v504: f64 = ((v503) as f64).abs();
        let v506: f64 = f64::powf(v504, self.scalar_v505);
        let v507: f64 = (v12 + v506);
        let v508: f64 = 2.0;
        let v509: f64 = (v500 * v508);
        let v510: f64 = (v509 / v507);
        let v511: f64 = (v421 * v510);
        let v512: f64 = (v276 * v510);
        let v553: f64 = (v202 - v196);
        let v555: f64 = (v553 / self.scalar_v554);
        let v556: f64 = ((v555) as f64).abs();
        let v558: f64 = f64::powf(v556, self.scalar_v557);
        let v559: f64 = (v12 + v558);
        let v561: f64 = f64::powf(v559, self.scalar_v560);
        let v562: f64 = (v561 - v12);
        let v565: f64 = (v562 * self.scalar_v564);
        let v566: f64 = (v12 + v565);
        let v567: f64 = (self.scalar_v563 * v566);
        let v568: f64 = (v276 * v567);
        let v570: f64 = (v511 * self.scalar_v569);
        let v595: bool = (v198 <= v13);
        let v596: f64 = (v189 * v195);
        let v598: f64 = (v198 / v189);
        let v599: f64 = (v12 - v598);
        let v600: f64 = ((v599) as f64).ln();
        let v601: f64 = (self.scalar_v597 * v600);
        let v602: f64 = ((v601) as f64).exp();
        let v603: f64 = (v12 - v602);
        let v604: f64 = (v596 * v603);
        let v605: f64 = (v604 / self.scalar_v597);
        let v606: f64 = (if v595 { v605 } else { v13 });
        let v607: bool = (!v595);
        let v608: f64 = (v195 * v198);
        let v611: f64 = (v198 * self.scalar_v610);
        let v612: f64 = (v611 / v189);
        let v613: f64 = (v12 + v612);
        let v614: f64 = (v608 * v613);
        let v615: f64 = (if v607 { v614 } else { v606 });
        let v616: f64 = (-v151);
        let v618: f64 = (v616 * self.scalar_v617);
        let v619: f64 = (v201 + v618);
        let v620: bool = (v619 > v13);
        let v626: f64 = (if v620 { self.scalar_v625 } else { v13 });
        let v627: f64 = (self.scalar_v622 * v626);
        let v628: f64 = (self.scalar_v622 * v627);
        let v629: f64 = (v12 - v628);
        let v630: f64 = (v151 * v629);
        let v632: f64 = (v630 / self.scalar_v631);
        let v633: f64 = (if v620 { v632 } else { v13 });
        let v635: f64 = (v619 * self.scalar_v634);
        let v636: f64 = (v635 / v151);
        let v637: f64 = (self.scalar_v622 + v636);
        let v638: f64 = (v619 * v637);
        let v639: f64 = (v626 * v638);
        let v640: f64 = (if v620 { v639 } else { v13 });
        let v641: bool = (!v620);
        let v642: f64 = (v201 / v151);
        let v643: f64 = (v12 - v642);
        let v644: f64 = ((v643) as f64).ln();
        let v645: f64 = (self.scalar_v631 * v644);
        let v646: f64 = ((v645) as f64).exp();
        let v647: f64 = (v12 - v646);
        let v648: f64 = (v151 * v647);
        let v649: f64 = (v648 / self.scalar_v631);
        let v650: f64 = (if v641 { v649 } else { v633 });
        let v651: f64 = (if v641 { v13 } else { v640 });
        let v652: f64 = (v650 + v651);
        let v653: f64 = (v159 * v652);
        let v654: f64 = (-v171);
        let v655: f64 = (self.scalar_v617 * v654);
        let v656: f64 = (v204 + v655);
        let v657: bool = (v656 > v13);
        let v661: f64 = (if v657 { self.scalar_v660 } else { v626 });
        let v662: f64 = (self.scalar_v622 * v661);
        let v663: f64 = (self.scalar_v622 * v662);
        let v664: f64 = (v12 - v663);
        let v665: f64 = (v171 * v664);
        let v667: f64 = (v665 / self.scalar_v666);
        let v668: f64 = (if v657 { v667 } else { v650 });
        let v670: f64 = (v656 * self.scalar_v669);
        let v671: f64 = (v670 / v171);
        let v672: f64 = (self.scalar_v622 + v671);
        let v673: f64 = (v656 * v672);
        let v674: f64 = (v661 * v673);
        let v675: f64 = (if v657 { v674 } else { v651 });
        let v676: bool = (!v657);
        let v677: f64 = (v204 / v171);
        let v678: f64 = (v12 - v677);
        let v679: f64 = ((v678) as f64).ln();
        let v680: f64 = (self.scalar_v666 * v679);
        let v681: f64 = ((v680) as f64).exp();
        let v682: f64 = (v12 - v681);
        let v683: f64 = (v171 * v682);
        let v684: f64 = (v683 / self.scalar_v666);
        let v685: f64 = (if v676 { v684 } else { v668 });
        let v686: f64 = (if v676 { v13 } else { v675 });
        let v687: f64 = (v685 + v686);
        let v688: f64 = (v177 * v687);
        let v691: f64 = (v688 * self.scalar_v690);
        let v692: f64 = (v21 + v655);
        let v693: bool = (v692 > v13);
        let v694: f64 = (if v693 { self.scalar_v660 } else { v661 });
        let v695: f64 = (self.scalar_v622 * v694);
        let v696: f64 = (self.scalar_v622 * v695);
        let v697: f64 = (v12 - v696);
        let v698: f64 = (v171 * v697);
        let v699: f64 = (v698 / self.scalar_v666);
        let v700: f64 = (if v693 { v699 } else { v685 });
        let v701: f64 = (self.scalar_v669 * v692);
        let v702: f64 = (v701 / v171);
        let v703: f64 = (self.scalar_v622 + v702);
        let v704: f64 = (v692 * v703);
        let v705: f64 = (v694 * v704);
        let v706: f64 = (if v693 { v705 } else { v686 });
        let v707: bool = (!v693);
        let v708: f64 = (v21 / v171);
        let v709: f64 = (v12 - v708);
        let v710: f64 = ((v709) as f64).ln();
        let v711: f64 = (self.scalar_v666 * v710);
        let v712: f64 = ((v711) as f64).exp();
        let v713: f64 = (v12 - v712);
        let v714: f64 = (v171 * v713);
        let v715: f64 = (v714 / self.scalar_v666);
        let v716: f64 = (if v707 { v715 } else { v700 });
        let v717: f64 = (if v707 { v13 } else { v706 });
        let v718: f64 = (v716 + v717);
        let v719: f64 = (v177 * v718);
        let v720: f64 = (self.scalar_v689 * v719);
        let v731: f64 = (v512 * self.scalar_v730);
        let v732: f64 = (if self.scalar_v724 { v731 } else { v13 });
        let v734: f64 = (if self.scalar_v733 { v13 } else { v732 });
        let v842: f64 = (self.scalar_v17 * v653);
        let v843: f64 = (self.scalar_v16 * v842);
        let v844: f64 = (self.scalar_v17 * v568);
        let v845: f64 = (self.scalar_v16 * v844);
        let v846: f64 = (self.scalar_v17 * v691);
        let v847: f64 = (self.scalar_v16 * v846);
        let v848: f64 = (self.scalar_v17 * v720);
        let v849: f64 = (self.scalar_v16 * v848);
        let v850: f64 = (self.scalar_v17 * v570);
        let v851: f64 = (self.scalar_v16 * v850);
        let v852: f64 = (self.scalar_v17 * v615);
        let v853: f64 = (self.scalar_v16 * v852);
        let v854: f64 = (-v734);
        let v855: f64 = (self.scalar_v16 * v854);
        let v856: f64 = (self.scalar_v16 * v734);
        let v857: f64 = (if v8 { v12 } else { v13 });
        let v858: f64 = (if v10 { v13 } else { v857 });
        let v871: f64 = (v32 * v858);
        let v872: f64 = (v858 / self.scalar_v31);
        let v873: f64 = (v872 / v34);
        let v881: f64 = (self.scalar_v60 * v873);
        let v882: f64 = (self.scalar_v62 * v872);
        let v883: f64 = (v33 * v882);
        let v884: f64 = (v64 * v871);
        let v885: f64 = (v883 - v884);
        let v886: f64 = (v33 * v33);
        let v887: f64 = (v885 / v886);
        let v888: f64 = (v881 + v887);
        let v890: f64 = (v70 * v888);
        let v891: f64 = (self.scalar_v69 * v890);
        let v909: f64 = (self.scalar_v88 * v872);
        let v910: f64 = (self.scalar_v87 * v909);
        let v911: f64 = (self.scalar_v93 * v872);
        let v912: f64 = (self.scalar_v92 * v911);
        let v913: f64 = (self.scalar_v98 * v872);
        let v914: f64 = (self.scalar_v97 * v913);
        let v915: f64 = (self.scalar_v103 * v872);
        let v916: f64 = (self.scalar_v102 * v915);
        let v917: f64 = (v858 / v110);
        let v918: f64 = (v114 * v858);
        let v919: f64 = (v115 * v858);
        let v920: f64 = (v11 * v918);
        let v921: f64 = (v919 + v920);
        let v922: f64 = (v118 * v921);
        let v923: f64 = (v116 * v858);
        let v924: f64 = (v922 - v923);
        let v925: f64 = (v118 * v118);
        let v926: f64 = (v924 / v925);
        let v927: f64 = (v858 + v858);
        let v928: f64 = (v122 * v927);
        let v929: f64 = (v124 * v926);
        let v930: f64 = (v121 * v928);
        let v931: f64 = (v929 - v930);
        let v932: f64 = (v124 * v124);
        let v933: f64 = (v931 / v932);
        let v934: f64 = (v871 + v871);
        let v935: f64 = (-v934);
        let v936: f64 = (v917 / v112);
        let v937: f64 = (v130 * v936);
        let v938: f64 = (v133 * v933);
        let v939: f64 = (v937 + v938);
        let v940: f64 = (v135 * v935);
        let v941: f64 = (v129 * v939);
        let v942: f64 = (v940 + v941);
        let v943: f64 = (-v942);
        let v944: f64 = (v943 / self.scalar_v111);
        let v945: f64 = (-v944);
        let v946: f64 = (v139 * v945);
        let v947: f64 = (v140 * v944);
        let v948: f64 = (v946 - v947);
        let v949: f64 = (v139 * v139);
        let v950: f64 = (v948 / v949);
        let v951: f64 = (-v950);
        let v952: f64 = (self.scalar_v142 * v951);
        let v953: f64 = (self.scalar_v107 * v952);
        let v954: f64 = (-v953);
        let v955: f64 = (v148 * v148);
        let v956: f64 = (v954 / v955);
        let v957: f64 = (v139 * v917);
        let v958: f64 = (v112 * v944);
        let v959: f64 = (v957 + v958);
        let v960: f64 = (v942 + v959);
        let v961: f64 = (v960 - v944);
        let v962: f64 = (v139 * v961);
        let v963: f64 = (v152 * v944);
        let v964: f64 = (v962 - v963);
        let v965: f64 = (v964 / v949);
        let v966: f64 = (v143 * v858);
        let v967: f64 = (v966 - v965);
        let v968: f64 = (self.scalar_v142 * v967);
        let v969: f64 = (v158 * v956);
        let v970: f64 = (v149 * v968);
        let v971: f64 = (v969 + v970);
        let v972: f64 = (v162 * v945);
        let v973: f64 = (v163 * v944);
        let v974: f64 = (v972 - v973);
        let v975: f64 = (v162 * v162);
        let v976: f64 = (v974 / v975);
        let v977: f64 = (-v976);
        let v978: f64 = (self.scalar_v165 * v977);
        let v979: f64 = (self.scalar_v108 * v978);
        let v980: f64 = (-v979);
        let v981: f64 = (v168 * v168);
        let v982: f64 = (v980 / v981);
        let v983: f64 = (v162 * v917);
        let v984: f64 = (v958 + v983);
        let v985: f64 = (v942 + v984);
        let v986: f64 = (v985 - v944);
        let v987: f64 = (v162 * v986);
        let v988: f64 = (v172 * v944);
        let v989: f64 = (v987 - v988);
        let v990: f64 = (v989 / v975);
        let v991: f64 = (v966 - v990);
        let v992: f64 = (self.scalar_v165 * v991);
        let v993: f64 = (v176 * v982);
        let v994: f64 = (v169 * v992);
        let v995: f64 = (v993 + v994);
        let v996: f64 = (v180 * v945);
        let v997: f64 = (v181 * v944);
        let v998: f64 = (v996 - v997);
        let v999: f64 = (v180 * v180);
        let v1000: f64 = (v998 / v999);
        let v1001: f64 = (-v1000);
        let v1002: f64 = (self.scalar_v183 * v1001);
        let v1003: f64 = (self.scalar_v109 * v1002);
        let v1004: f64 = (-v1003);
        let v1005: f64 = (v186 * v186);
        let v1006: f64 = (v1004 / v1005);
        let v1007: f64 = (v180 * v917);
        let v1008: f64 = (v958 + v1007);
        let v1009: f64 = (v942 + v1008);
        let v1010: f64 = (v1009 - v944);
        let v1011: f64 = (v180 * v1010);
        let v1012: f64 = (v190 * v944);
        let v1013: f64 = (v1011 - v1012);
        let v1014: f64 = (v1013 / v999);
        let v1015: f64 = (v966 - v1014);
        let v1016: f64 = (self.scalar_v183 * v1015);
        let v1017: f64 = (v194 * v1006);
        let v1018: f64 = (v187 * v1016);
        let v1019: f64 = (v1017 + v1018);
        let v1020: f64 = (self.scalar_v210 * v871);
        let v1021: f64 = (v201 * v1020);
        let v1022: f64 = (-v1021);
        let v1023: f64 = (v211 * v211);
        let v1024: f64 = (v1022 / v1023);
        let v1025: f64 = (self.scalar_v17 / v211);
        let v1026: f64 = (self.scalar_v859 / v211);
        let v1027: f64 = (if v209 { v1024 } else { v13 });
        let v1028: f64 = (if v209 { v1025 } else { v13 });
        let v1029: f64 = (if v209 { v1026 } else { v13 });
        let v1030: f64 = (-v912);
        let v1031: f64 = (self.scalar_v216 * v871);
        let v1032: f64 = (v217 * v1030);
        let v1033: f64 = (v215 * v1031);
        let v1034: f64 = (v1032 - v1033);
        let v1035: f64 = (v217 * v217);
        let v1036: f64 = (v1034 / v1035);
        let v1037: f64 = (self.scalar_v859 / v217);
        let v1038: f64 = (self.scalar_v17 / v217);
        let v1039: f64 = (if v209 { v1036 } else { v13 });
        let v1040: f64 = (if v209 { v1037 } else { v13 });
        let v1041: f64 = (if v209 { v1038 } else { v13 });
        let v1042: f64 = (v220 * v1031);
        let v1043: f64 = (v1032 - v1042);
        let v1044: f64 = (v1043 / v1035);
        let v1045: f64 = (if v209 { v1044 } else { v13 });
        let v1046: f64 = (if v225 { v1027 } else { v13 });
        let v1047: f64 = (if v225 { v1028 } else { v13 });
        let v1048: f64 = (if v225 { v1029 } else { v13 });
        let v1049: f64 = (if v225 { v13 } else { v1027 });
        let v1050: f64 = (if v225 { v13 } else { v1028 });
        let v1051: f64 = (if v225 { v13 } else { v1029 });
        let v1052: f64 = (if v231 { v13 } else { v1046 });
        let v1053: f64 = (if v231 { v13 } else { v1047 });
        let v1054: f64 = (if v231 { v13 } else { v1048 });
        let v1055: f64 = (v233 * v1049);
        let v1056: f64 = (v233 * v1050);
        let v1057: f64 = (v233 * v1051);
        let v1058: f64 = (v233 * v1052);
        let v1059: f64 = (v232 * v1055);
        let v1060: f64 = (v1058 + v1059);
        let v1061: f64 = (v233 * v1053);
        let v1062: f64 = (v232 * v1056);
        let v1063: f64 = (v1061 + v1062);
        let v1064: f64 = (v233 * v1054);
        let v1065: f64 = (v232 * v1057);
        let v1066: f64 = (v1064 + v1065);
        let v1067: f64 = (if v209 { v1060 } else { v1052 });
        let v1068: f64 = (if v209 { v1063 } else { v1053 });
        let v1069: f64 = (if v209 { v1066 } else { v1054 });
        let v1070: f64 = (v243 * v1039);
        let v1071: f64 = (v243 * v1040);
        let v1072: f64 = (v243 * v1041);
        let v1073: f64 = (v1070 / v244);
        let v1074: f64 = (v1071 / v244);
        let v1075: f64 = (v1072 / v244);
        let v1076: f64 = (if v237 { v1039 } else { v13 });
        let v1077: f64 = (if v237 { v1040 } else { v13 });
        let v1078: f64 = (if v237 { v1041 } else { v13 });
        let v1079: f64 = (if v246 { v1070 } else { v1076 });
        let v1080: f64 = (if v246 { v1071 } else { v1077 });
        let v1081: f64 = (if v246 { v1072 } else { v1078 });
        let v1082: f64 = (if v242 { v1073 } else { v1079 });
        let v1083: f64 = (if v242 { v1074 } else { v1080 });
        let v1084: f64 = (if v242 { v1075 } else { v1081 });
        let v1085: f64 = (v255 * v1045);
        let v1086: f64 = (v1085 / v256);
        let v1087: f64 = (if v250 { v1045 } else { v13 });
        let v1088: f64 = (if v258 { v1085 } else { v1087 });
        let v1089: f64 = (if v254 { v1086 } else { v1088 });
        let v1090: f64 = (v1082 - v1089);
        let v1091: f64 = (if v209 { v1090 } else { v13 });
        let v1092: f64 = (if v209 { v1083 } else { v13 });
        let v1093: f64 = (if v209 { v1084 } else { v13 });
        let v1094: f64 = (v264 * v891);
        let v1095: f64 = (v71 * v1067);
        let v1096: f64 = (v1094 + v1095);
        let v1097: f64 = (v71 * v1068);
        let v1098: f64 = (v71 * v1069);
        let v1099: f64 = (v263 * v910);
        let v1100: f64 = (v91 * v1091);
        let v1101: f64 = (v1099 + v1100);
        let v1102: f64 = (v91 * v1092);
        let v1103: f64 = (v91 * v1093);
        let v1104: f64 = ((v268) as f64).ln();
        let v1105: f64 = (v269 * v1104);
        let v1106: f64 = (v914 * v1105);
        let v1107: f64 = (self.scalar_v267 * v1106);
        let v1108: f64 = (v271 * v1101);
        let v1109: f64 = (v266 * v1107);
        let v1110: f64 = (v1108 - v1109);
        let v1111: f64 = (v271 * v271);
        let v1112: f64 = (v1110 / v1111);
        let v1113: f64 = (v1102 / v271);
        let v1114: f64 = (v1103 / v271);
        let v1115: f64 = (v1096 - v1112);
        let v1116: f64 = (v1097 - v1113);
        let v1117: f64 = (v1098 - v1114);
        let v1118: f64 = (if v209 { v1115 } else { v13 });
        let v1119: f64 = (if v209 { v1116 } else { v13 });
        let v1120: f64 = (if v209 { v1117 } else { v13 });
        let v1121: f64 = (if v275 { v13 } else { v1118 });
        let v1122: f64 = (if v275 { v13 } else { v1119 });
        let v1123: f64 = (if v275 { v13 } else { v1120 });
        let v1124: f64 = (if v281 { self.scalar_v859 } else { v13 });
        let v1125: f64 = (if v281 { self.scalar_v17 } else { v13 });
        let v1126: f64 = (if v277 { v1124 } else { v13 });
        let v1127: f64 = (if v277 { v1125 } else { v13 });
        let v1130: f64 = (self.scalar_v286 * v871);
        let v1131: f64 = (v283 * v1130);
        let v1132: f64 = (v287 * v1126);
        let v1133: f64 = (v287 * v1127);
        let v1134: f64 = (v285 * v1131);
        let v1135: f64 = (-v1134);
        let v1136: f64 = (v288 * v288);
        let v1137: f64 = (v1135 / v1136);
        let v1138: f64 = (v288 * self.scalar_v1128);
        let v1139: f64 = (v285 * v1132);
        let v1140: f64 = (v1138 - v1139);
        let v1141: f64 = (v1140 / v1136);
        let v1142: f64 = (v288 * self.scalar_v1129);
        let v1143: f64 = (v285 * v1133);
        let v1144: f64 = (v1142 - v1143);
        let v1145: f64 = (v1144 / v1136);
        let v1146: f64 = (if v277 { v1137 } else { v1049 });
        let v1147: f64 = (if v277 { v1141 } else { v1050 });
        let v1148: f64 = (if v277 { v1145 } else { v1051 });
        let v1149: f64 = (if v292 { v1146 } else { v1067 });
        let v1150: f64 = (if v292 { v1147 } else { v1068 });
        let v1151: f64 = (if v292 { v1148 } else { v1069 });
        let v1152: f64 = (if v292 { v13 } else { v1146 });
        let v1153: f64 = (if v292 { v13 } else { v1147 });
        let v1154: f64 = (if v292 { v13 } else { v1148 });
        let v1155: f64 = (if v298 { v13 } else { v1149 });
        let v1156: f64 = (if v298 { v13 } else { v1150 });
        let v1157: f64 = (if v298 { v13 } else { v1151 });
        let v1158: f64 = (v300 * v1152);
        let v1159: f64 = (v300 * v1153);
        let v1160: f64 = (v300 * v1154);
        let v1161: f64 = (v300 * v1155);
        let v1162: f64 = (v299 * v1158);
        let v1163: f64 = (v1161 + v1162);
        let v1164: f64 = (v300 * v1156);
        let v1165: f64 = (v299 * v1159);
        let v1166: f64 = (v1164 + v1165);
        let v1167: f64 = (v300 * v1157);
        let v1168: f64 = (v299 * v1160);
        let v1169: f64 = (v1167 + v1168);
        let v1170: f64 = (if v277 { v1163 } else { v1155 });
        let v1171: f64 = (if v277 { v1166 } else { v1156 });
        let v1172: f64 = (if v277 { v1169 } else { v1157 });
        let v1184: f64 = (self.scalar_v76 * v871);
        let v1185: f64 = (v201 * v1184);
        let v1186: f64 = (-v1185);
        let v1187: f64 = (v309 * v309);
        let v1188: f64 = (v1186 / v1187);
        let v1189: f64 = (self.scalar_v17 / v309);
        let v1190: f64 = (self.scalar_v859 / v309);
        let v1191: f64 = (if v308 { v1188 } else { v1152 });
        let v1192: f64 = (if v308 { v1189 } else { v1153 });
        let v1193: f64 = (if v308 { v1190 } else { v1154 });
        let v1194: f64 = (self.scalar_v312 * v871);
        let v1195: f64 = (v313 * v1030);
        let v1196: f64 = (v215 * v1194);
        let v1197: f64 = (v1195 - v1196);
        let v1198: f64 = (v313 * v313);
        let v1199: f64 = (v1197 / v1198);
        let v1200: f64 = (self.scalar_v859 / v313);
        let v1201: f64 = (self.scalar_v17 / v313);
        let v1202: f64 = (if v308 { v1199 } else { v1039 });
        let v1203: f64 = (if v308 { v1200 } else { v1040 });
        let v1204: f64 = (if v308 { v1201 } else { v1041 });
        let v1205: f64 = (v220 * v1194);
        let v1206: f64 = (v1195 - v1205);
        let v1207: f64 = (v1206 / v1198);
        let v1208: f64 = (if v308 { v1207 } else { v1045 });
        let v1209: f64 = (if v319 { v1191 } else { v1170 });
        let v1210: f64 = (if v319 { v1192 } else { v1171 });
        let v1211: f64 = (if v319 { v1193 } else { v1172 });
        let v1212: f64 = (if v319 { v13 } else { v1191 });
        let v1213: f64 = (if v319 { v13 } else { v1192 });
        let v1214: f64 = (if v319 { v13 } else { v1193 });
        let v1215: f64 = (if v325 { v13 } else { v1209 });
        let v1216: f64 = (if v325 { v13 } else { v1210 });
        let v1217: f64 = (if v325 { v13 } else { v1211 });
        let v1218: f64 = (v327 * v1212);
        let v1219: f64 = (v327 * v1213);
        let v1220: f64 = (v327 * v1214);
        let v1221: f64 = (v327 * v1215);
        let v1222: f64 = (v326 * v1218);
        let v1223: f64 = (v1221 + v1222);
        let v1224: f64 = (v327 * v1216);
        let v1225: f64 = (v326 * v1219);
        let v1226: f64 = (v1224 + v1225);
        let v1227: f64 = (v327 * v1217);
        let v1228: f64 = (v326 * v1220);
        let v1229: f64 = (v1227 + v1228);
        let v1230: f64 = (if v308 { v1223 } else { v1215 });
        let v1231: f64 = (if v308 { v1226 } else { v1216 });
        let v1232: f64 = (if v308 { v1229 } else { v1217 });
        let v1233: f64 = (v335 * v1202);
        let v1234: f64 = (v335 * v1203);
        let v1235: f64 = (v335 * v1204);
        let v1236: f64 = (v1233 / v336);
        let v1237: f64 = (v1234 / v336);
        let v1238: f64 = (v1235 / v336);
        let v1239: f64 = (if v330 { v1202 } else { v13 });
        let v1240: f64 = (if v330 { v1203 } else { v13 });
        let v1241: f64 = (if v330 { v1204 } else { v13 });
        let v1242: f64 = (if v338 { v1233 } else { v1239 });
        let v1243: f64 = (if v338 { v1234 } else { v1240 });
        let v1244: f64 = (if v338 { v1235 } else { v1241 });
        let v1245: f64 = (if v334 { v1236 } else { v1242 });
        let v1246: f64 = (if v334 { v1237 } else { v1243 });
        let v1247: f64 = (if v334 { v1238 } else { v1244 });
        let v1248: f64 = (v347 * v1208);
        let v1249: f64 = (v1248 / v348);
        let v1250: f64 = (if v342 { v1208 } else { v13 });
        let v1251: f64 = (if v350 { v1248 } else { v1250 });
        let v1252: f64 = (if v346 { v1249 } else { v1251 });
        let v1253: f64 = (v1245 - v1252);
        let v1254: f64 = (if v308 { v1253 } else { v1091 });
        let v1255: f64 = (if v308 { v1246 } else { v1092 });
        let v1256: f64 = (if v308 { v1247 } else { v1093 });
        let v1280: f64 = (self.scalar_v364 * v871);
        let v1281: f64 = (v21 * v1280);
        let v1282: f64 = (-v1281);
        let v1283: f64 = (v365 * v365);
        let v1284: f64 = (v1282 / v1283);
        let v1285: f64 = (self.scalar_v859 / v365);
        let v1286: f64 = (self.scalar_v17 / v365);
        let v1287: f64 = (if v209 { v1284 } else { v1212 });
        let v1288: f64 = (if v209 { v1285 } else { v13 });
        let v1289: f64 = (if v209 { v1286 } else { v1213 });
        let v1290: f64 = (if v209 { v13 } else { v1214 });
        let v1291: f64 = (v369 * v1194);
        let v1292: f64 = (v1195 - v1291);
        let v1293: f64 = (v1292 / v1198);
        let v1294: f64 = (if v209 { v1293 } else { v1202 });
        let v1295: f64 = (if v209 { v1201 } else { v13 });
        let v1296: f64 = (if v209 { v1200 } else { v1203 });
        let v1297: f64 = (if v209 { v13 } else { v1204 });
        let v1298: f64 = (if v209 { v1207 } else { v1208 });
        let v1299: f64 = (if v374 { v1287 } else { v1230 });
        let v1300: f64 = (if v374 { v1288 } else { v13 });
        let v1301: f64 = (if v374 { v1289 } else { v1231 });
        let v1302: f64 = (if v374 { v1290 } else { v1232 });
        let v1303: f64 = (if v374 { v13 } else { v1287 });
        let v1304: f64 = (if v374 { v13 } else { v1288 });
        let v1305: f64 = (if v374 { v13 } else { v1289 });
        let v1306: f64 = (if v374 { v13 } else { v1290 });
        let v1307: f64 = (if v380 { v13 } else { v1299 });
        let v1308: f64 = (if v380 { v13 } else { v1300 });
        let v1309: f64 = (if v380 { v13 } else { v1301 });
        let v1310: f64 = (if v380 { v13 } else { v1302 });
        let v1311: f64 = (v382 * v1303);
        let v1312: f64 = (v382 * v1304);
        let v1313: f64 = (v382 * v1305);
        let v1314: f64 = (v382 * v1306);
        let v1315: f64 = (v382 * v1307);
        let v1316: f64 = (v381 * v1311);
        let v1317: f64 = (v1315 + v1316);
        let v1318: f64 = (v382 * v1308);
        let v1319: f64 = (v381 * v1312);
        let v1320: f64 = (v1318 + v1319);
        let v1321: f64 = (v382 * v1309);
        let v1322: f64 = (v381 * v1313);
        let v1323: f64 = (v1321 + v1322);
        let v1324: f64 = (v382 * v1310);
        let v1325: f64 = (v381 * v1314);
        let v1326: f64 = (v1324 + v1325);
        let v1327: f64 = (if v209 { v1317 } else { v1307 });
        let v1328: f64 = (if v209 { v1320 } else { v1308 });
        let v1329: f64 = (if v209 { v1323 } else { v1309 });
        let v1330: f64 = (if v209 { v1326 } else { v1310 });
        let v1331: f64 = (v390 * v1294);
        let v1332: f64 = (v390 * v1295);
        let v1333: f64 = (v390 * v1296);
        let v1334: f64 = (v390 * v1297);
        let v1335: f64 = (v1331 / v391);
        let v1336: f64 = (v1332 / v391);
        let v1337: f64 = (v1333 / v391);
        let v1338: f64 = (v1334 / v391);
        let v1339: f64 = (if v385 { v1294 } else { v13 });
        let v1340: f64 = (if v385 { v1295 } else { v13 });
        let v1341: f64 = (if v385 { v1296 } else { v13 });
        let v1342: f64 = (if v385 { v1297 } else { v13 });
        let v1343: f64 = (if v393 { v1331 } else { v1339 });
        let v1344: f64 = (if v393 { v1332 } else { v1340 });
        let v1345: f64 = (if v393 { v1333 } else { v1341 });
        let v1346: f64 = (if v393 { v1334 } else { v1342 });
        let v1347: f64 = (if v389 { v1335 } else { v1343 });
        let v1348: f64 = (if v389 { v1336 } else { v1344 });
        let v1349: f64 = (if v389 { v1337 } else { v1345 });
        let v1350: f64 = (if v389 { v1338 } else { v1346 });
        let v1351: f64 = (v402 * v1298);
        let v1352: f64 = (v1351 / v403);
        let v1353: f64 = (if v397 { v1298 } else { v13 });
        let v1354: f64 = (if v405 { v1351 } else { v1353 });
        let v1355: f64 = (if v401 { v1352 } else { v1354 });
        let v1356: f64 = (v1347 - v1355);
        let v1357: f64 = (if v209 { v1356 } else { v1254 });
        let v1358: f64 = (if v209 { v1348 } else { v13 });
        let v1359: f64 = (if v209 { v1349 } else { v1255 });
        let v1360: f64 = (if v209 { v1350 } else { v1256 });
        let v1361: f64 = (v411 * v891);
        let v1362: f64 = (v71 * v1327);
        let v1363: f64 = (v1361 + v1362);
        let v1364: f64 = (v71 * v1328);
        let v1365: f64 = (v71 * v1329);
        let v1366: f64 = (v71 * v1330);
        let v1367: f64 = (v410 * v916);
        let v1368: f64 = (v106 * v1357);
        let v1369: f64 = (v1367 + v1368);
        let v1370: f64 = (v106 * v1358);
        let v1371: f64 = (v106 * v1359);
        let v1372: f64 = (v106 * v1360);
        let v1373: f64 = ((v414) as f64).ln();
        let v1374: f64 = (v415 * v1373);
        let v1375: f64 = (v914 * v1374);
        let v1376: f64 = (self.scalar_v267 * v1375);
        let v1377: f64 = (v417 * v1369);
        let v1378: f64 = (v413 * v1376);
        let v1379: f64 = (v1377 - v1378);
        let v1380: f64 = (v417 * v417);
        let v1381: f64 = (v1379 / v1380);
        let v1382: f64 = (v1370 / v417);
        let v1383: f64 = (v1371 / v417);
        let v1384: f64 = (v1372 / v417);
        let v1385: f64 = (v1363 - v1381);
        let v1386: f64 = (v1364 - v1382);
        let v1387: f64 = (v1365 - v1383);
        let v1388: f64 = (v1366 - v1384);
        let v1389: f64 = (if v209 { v1385 } else { v13 });
        let v1390: f64 = (if v209 { v1386 } else { v13 });
        let v1391: f64 = (if v209 { v1387 } else { v13 });
        let v1392: f64 = (if v209 { v1388 } else { v13 });
        let v1393: f64 = (if v275 { v13 } else { v1389 });
        let v1394: f64 = (if v275 { v13 } else { v1390 });
        let v1395: f64 = (if v275 { v13 } else { v1391 });
        let v1396: f64 = (if v275 { v13 } else { v1392 });
        let v1543: f64 = (self.scalar_v1540 / v507);
        let v1544: f64 = (self.scalar_v1541 / v507);
        let v1545: f64 = (self.scalar_v1542 / v507);
        let v1546: f64 = (v510 * v1393);
        let v1547: f64 = (v510 * v1394);
        let v1548: f64 = (v421 * v1543);
        let v1549: f64 = (v1547 + v1548);
        let v1550: f64 = (v510 * v1395);
        let v1551: f64 = (v421 * v1544);
        let v1552: f64 = (v1550 + v1551);
        let v1553: f64 = (v510 * v1396);
        let v1554: f64 = (v421 * v1545);
        let v1555: f64 = (v1553 + v1554);
        let v1556: f64 = (v510 * v1121);
        let v1557: f64 = (v276 * v1543);
        let v1558: f64 = (v510 * v1122);
        let v1559: f64 = (v276 * v1544);
        let v1560: f64 = (v1558 + v1559);
        let v1561: f64 = (v510 * v1123);
        let v1562: f64 = (v276 * v1545);
        let v1563: f64 = (v1561 + v1562);
        let v1598: f64 = (v567 * v1121);
        let v1599: f64 = (v567 * v1122);
        let v1600: f64 = (v567 * v1123);
        let v1601: f64 = (self.scalar_v569 * v1546);
        let v1602: f64 = (self.scalar_v569 * v1549);
        let v1603: f64 = (self.scalar_v569 * v1552);
        let v1604: f64 = (self.scalar_v569 * v1555);
        let v1611: f64 = (v195 * v1009);
        let v1612: f64 = (v189 * v1019);
        let v1613: f64 = (v1611 + v1612);
        let v1614: f64 = (self.scalar_v17 / v189);
        let v1615: f64 = (v198 * v1009);
        let v1616: f64 = (-v1615);
        let v1617: f64 = (v189 * v189);
        let v1618: f64 = (v1616 / v1617);
        let v1619: f64 = (self.scalar_v859 / v189);
        let v1620: f64 = (-v1614);
        let v1621: f64 = (-v1618);
        let v1622: f64 = (-v1619);
        let v1623: f64 = (v1620 / v599);
        let v1624: f64 = (v1621 / v599);
        let v1625: f64 = (v1622 / v599);
        let v1626: f64 = (self.scalar_v597 * v1623);
        let v1627: f64 = (self.scalar_v597 * v1624);
        let v1628: f64 = (self.scalar_v597 * v1625);
        let v1629: f64 = (v602 * v1626);
        let v1630: f64 = (v602 * v1627);
        let v1631: f64 = (v602 * v1628);
        let v1632: f64 = (-v1629);
        let v1633: f64 = (-v1630);
        let v1634: f64 = (-v1631);
        let v1635: f64 = (v596 * v1632);
        let v1636: f64 = (v603 * v1613);
        let v1637: f64 = (v596 * v1633);
        let v1638: f64 = (v1636 + v1637);
        let v1639: f64 = (v596 * v1634);
        let v1640: f64 = (v1635 / self.scalar_v597);
        let v1641: f64 = (v1638 / self.scalar_v597);
        let v1642: f64 = (v1639 / self.scalar_v597);
        let v1643: f64 = (if v595 { v1640 } else { v13 });
        let v1644: f64 = (if v595 { v1641 } else { v13 });
        let v1645: f64 = (if v595 { v1642 } else { v13 });
        let v1646: f64 = (self.scalar_v17 * v195);
        let v1647: f64 = (v198 * v1019);
        let v1648: f64 = (v195 * self.scalar_v859);
        let v1651: f64 = (self.scalar_v1649 / v189);
        let v1652: f64 = (v611 * v1009);
        let v1653: f64 = (-v1652);
        let v1654: f64 = (v1653 / v1617);
        let v1655: f64 = (self.scalar_v1650 / v189);
        let v1656: f64 = (v613 * v1646);
        let v1657: f64 = (v608 * v1651);
        let v1658: f64 = (v1656 + v1657);
        let v1659: f64 = (v613 * v1647);
        let v1660: f64 = (v608 * v1654);
        let v1661: f64 = (v1659 + v1660);
        let v1662: f64 = (v613 * v1648);
        let v1663: f64 = (v608 * v1655);
        let v1664: f64 = (v1662 + v1663);
        let v1665: f64 = (if v607 { v1658 } else { v1643 });
        let v1666: f64 = (if v607 { v1661 } else { v1644 });
        let v1667: f64 = (if v607 { v1664 } else { v1645 });
        let v1668: f64 = (-v960);
        let v1669: f64 = (self.scalar_v617 * v1668);
        let v1670: f64 = (v629 * v960);
        let v1671: f64 = (v1670 / self.scalar_v631);
        let v1672: f64 = (if v620 { v1671 } else { v13 });
        let v1673: f64 = (self.scalar_v634 * v1669);
        let v1676: f64 = (v151 * v1673);
        let v1677: f64 = (v635 * v960);
        let v1678: f64 = (v1676 - v1677);
        let v1679: f64 = (v151 * v151);
        let v1680: f64 = (v1678 / v1679);
        let v1681: f64 = (self.scalar_v1674 / v151);
        let v1682: f64 = (self.scalar_v1675 / v151);
        let v1683: f64 = (v637 * v1669);
        let v1684: f64 = (v619 * v1680);
        let v1685: f64 = (v1683 + v1684);
        let v1686: f64 = (self.scalar_v17 * v637);
        let v1687: f64 = (v619 * v1681);
        let v1688: f64 = (v1686 + v1687);
        let v1689: f64 = (v637 * self.scalar_v859);
        let v1690: f64 = (v619 * v1682);
        let v1691: f64 = (v1689 + v1690);
        let v1692: f64 = (v626 * v1685);
        let v1693: f64 = (v626 * v1688);
        let v1694: f64 = (v626 * v1691);
        let v1695: f64 = (if v620 { v1692 } else { v13 });
        let v1696: f64 = (if v620 { v1693 } else { v13 });
        let v1697: f64 = (if v620 { v1694 } else { v13 });
        let v1698: f64 = (v201 * v960);
        let v1699: f64 = (-v1698);
        let v1700: f64 = (v1699 / v1679);
        let v1701: f64 = (self.scalar_v17 / v151);
        let v1702: f64 = (self.scalar_v859 / v151);
        let v1703: f64 = (-v1700);
        let v1704: f64 = (-v1701);
        let v1705: f64 = (-v1702);
        let v1706: f64 = (v1703 / v643);
        let v1707: f64 = (v1704 / v643);
        let v1708: f64 = (v1705 / v643);
        let v1709: f64 = (self.scalar_v631 * v1706);
        let v1710: f64 = (self.scalar_v631 * v1707);
        let v1711: f64 = (self.scalar_v631 * v1708);
        let v1712: f64 = (v646 * v1709);
        let v1713: f64 = (v646 * v1710);
        let v1714: f64 = (v646 * v1711);
        let v1715: f64 = (-v1712);
        let v1716: f64 = (-v1713);
        let v1717: f64 = (-v1714);
        let v1718: f64 = (v647 * v960);
        let v1719: f64 = (v151 * v1715);
        let v1720: f64 = (v1718 + v1719);
        let v1721: f64 = (v151 * v1716);
        let v1722: f64 = (v151 * v1717);
        let v1723: f64 = (v1720 / self.scalar_v631);
        let v1724: f64 = (v1721 / self.scalar_v631);
        let v1725: f64 = (v1722 / self.scalar_v631);
        let v1726: f64 = (if v641 { v1723 } else { v1672 });
        let v1727: f64 = (if v641 { v1724 } else { v13 });
        let v1728: f64 = (if v641 { v1725 } else { v13 });
        let v1729: f64 = (if v641 { v13 } else { v1695 });
        let v1730: f64 = (if v641 { v13 } else { v1696 });
        let v1731: f64 = (if v641 { v13 } else { v1697 });
        let v1732: f64 = (v1726 + v1729);
        let v1733: f64 = (v1727 + v1730);
        let v1734: f64 = (v1728 + v1731);
        let v1735: f64 = (v652 * v971);
        let v1736: f64 = (v159 * v1732);
        let v1737: f64 = (v1735 + v1736);
        let v1738: f64 = (v159 * v1733);
        let v1739: f64 = (v159 * v1734);
        let v1740: f64 = (-v985);
        let v1741: f64 = (self.scalar_v617 * v1740);
        let v1742: f64 = (v664 * v985);
        let v1743: f64 = (v1742 / self.scalar_v666);
        let v1744: f64 = (if v657 { v1743 } else { v1726 });
        let v1745: f64 = (if v657 { v13 } else { v1727 });
        let v1746: f64 = (if v657 { v13 } else { v1728 });
        let v1748: f64 = (self.scalar_v669 * v1741);
        let v1750: f64 = (self.scalar_v1747 / v171);
        let v1751: f64 = (v171 * v1748);
        let v1752: f64 = (v670 * v985);
        let v1753: f64 = (v1751 - v1752);
        let v1754: f64 = (v171 * v171);
        let v1755: f64 = (v1753 / v1754);
        let v1756: f64 = (self.scalar_v1749 / v171);
        let v1757: f64 = (self.scalar_v17 * v672);
        let v1758: f64 = (v656 * v1750);
        let v1759: f64 = (v1757 + v1758);
        let v1760: f64 = (v672 * v1741);
        let v1761: f64 = (v656 * v1755);
        let v1762: f64 = (v1760 + v1761);
        let v1763: f64 = (v672 * self.scalar_v859);
        let v1764: f64 = (v656 * v1756);
        let v1765: f64 = (v1763 + v1764);
        let v1766: f64 = (v661 * v1759);
        let v1767: f64 = (v661 * v1762);
        let v1768: f64 = (v661 * v1765);
        let v1769: f64 = (if v657 { v1766 } else { v13 });
        let v1770: f64 = (if v657 { v1767 } else { v1729 });
        let v1771: f64 = (if v657 { v1768 } else { v13 });
        let v1772: f64 = (if v657 { v13 } else { v1730 });
        let v1773: f64 = (if v657 { v13 } else { v1731 });
        let v1774: f64 = (self.scalar_v17 / v171);
        let v1775: f64 = (v204 * v985);
        let v1776: f64 = (-v1775);
        let v1777: f64 = (v1776 / v1754);
        let v1778: f64 = (self.scalar_v859 / v171);
        let v1779: f64 = (-v1774);
        let v1780: f64 = (-v1777);
        let v1781: f64 = (-v1778);
        let v1782: f64 = (v1779 / v678);
        let v1783: f64 = (v1780 / v678);
        let v1784: f64 = (v1781 / v678);
        let v1785: f64 = (self.scalar_v666 * v1782);
        let v1786: f64 = (self.scalar_v666 * v1783);
        let v1787: f64 = (self.scalar_v666 * v1784);
        let v1788: f64 = (v681 * v1785);
        let v1789: f64 = (v681 * v1786);
        let v1790: f64 = (v681 * v1787);
        let v1791: f64 = (-v1788);
        let v1792: f64 = (-v1789);
        let v1793: f64 = (-v1790);
        let v1794: f64 = (v171 * v1791);
        let v1795: f64 = (v682 * v985);
        let v1796: f64 = (v171 * v1792);
        let v1797: f64 = (v1795 + v1796);
        let v1798: f64 = (v171 * v1793);
        let v1799: f64 = (v1794 / self.scalar_v666);
        let v1800: f64 = (v1797 / self.scalar_v666);
        let v1801: f64 = (v1798 / self.scalar_v666);
        let v1802: f64 = (if v676 { v1799 } else { v13 });
        let v1803: f64 = (if v676 { v1800 } else { v1744 });
        let v1804: f64 = (if v676 { v1801 } else { v13 });
        let v1805: f64 = (if v676 { v13 } else { v1745 });
        let v1806: f64 = (if v676 { v13 } else { v1746 });
        let v1807: f64 = (if v676 { v13 } else { v1769 });
        let v1808: f64 = (if v676 { v13 } else { v1770 });
        let v1809: f64 = (if v676 { v13 } else { v1771 });
        let v1810: f64 = (if v676 { v13 } else { v1772 });
        let v1811: f64 = (if v676 { v13 } else { v1773 });
        let v1812: f64 = (v1802 + v1807);
        let v1813: f64 = (v1803 + v1808);
        let v1814: f64 = (v1804 + v1809);
        let v1815: f64 = (v1805 + v1810);
        let v1816: f64 = (v1806 + v1811);
        let v1817: f64 = (v177 * v1812);
        let v1818: f64 = (v687 * v995);
        let v1819: f64 = (v177 * v1813);
        let v1820: f64 = (v1818 + v1819);
        let v1821: f64 = (v177 * v1814);
        let v1822: f64 = (v177 * v1815);
        let v1823: f64 = (v177 * v1816);
        let v1824: f64 = (self.scalar_v690 * v1817);
        let v1825: f64 = (self.scalar_v690 * v1820);
        let v1826: f64 = (self.scalar_v690 * v1821);
        let v1827: f64 = (self.scalar_v690 * v1822);
        let v1828: f64 = (self.scalar_v690 * v1823);
        let v1829: f64 = (v697 * v985);
        let v1830: f64 = (v1829 / self.scalar_v666);
        let v1831: f64 = (if v693 { v13 } else { v1802 });
        let v1832: f64 = (if v693 { v1830 } else { v1803 });
        let v1833: f64 = (if v693 { v13 } else { v1804 });
        let v1834: f64 = (if v693 { v13 } else { v1805 });
        let v1835: f64 = (if v693 { v13 } else { v1806 });
        let v1836: f64 = (v701 * v985);
        let v1837: f64 = (v1751 - v1836);
        let v1838: f64 = (v1837 / v1754);
        let v1839: f64 = (v703 * v1741);
        let v1840: f64 = (v692 * v1838);
        let v1841: f64 = (v1839 + v1840);
        let v1842: f64 = (v703 * self.scalar_v859);
        let v1843: f64 = (v692 * v1756);
        let v1844: f64 = (v1842 + v1843);
        let v1845: f64 = (self.scalar_v17 * v703);
        let v1846: f64 = (v692 * v1750);
        let v1847: f64 = (v1845 + v1846);
        let v1848: f64 = (v694 * v1841);
        let v1849: f64 = (v694 * v1844);
        let v1850: f64 = (v694 * v1847);
        let v1851: f64 = (if v693 { v13 } else { v1807 });
        let v1852: f64 = (if v693 { v1848 } else { v1808 });
        let v1853: f64 = (if v693 { v1849 } else { v1809 });
        let v1854: f64 = (if v693 { v1850 } else { v1810 });
        let v1855: f64 = (if v693 { v13 } else { v1811 });
        let v1856: f64 = (v21 * v985);
        let v1857: f64 = (-v1856);
        let v1858: f64 = (v1857 / v1754);
        let v1859: f64 = (-v1858);
        let v1860: f64 = (v1859 / v709);
        let v1861: f64 = (v1781 / v709);
        let v1862: f64 = (v1779 / v709);
        let v1863: f64 = (self.scalar_v666 * v1860);
        let v1864: f64 = (self.scalar_v666 * v1861);
        let v1865: f64 = (self.scalar_v666 * v1862);
        let v1866: f64 = (v712 * v1863);
        let v1867: f64 = (v712 * v1864);
        let v1868: f64 = (v712 * v1865);
        let v1869: f64 = (-v1866);
        let v1870: f64 = (-v1867);
        let v1871: f64 = (-v1868);
        let v1872: f64 = (v713 * v985);
        let v1873: f64 = (v171 * v1869);
        let v1874: f64 = (v1872 + v1873);
        let v1875: f64 = (v171 * v1870);
        let v1876: f64 = (v171 * v1871);
        let v1877: f64 = (v1874 / self.scalar_v666);
        let v1878: f64 = (v1875 / self.scalar_v666);
        let v1879: f64 = (v1876 / self.scalar_v666);
        let v1880: f64 = (if v707 { v13 } else { v1831 });
        let v1881: f64 = (if v707 { v1877 } else { v1832 });
        let v1882: f64 = (if v707 { v1878 } else { v1833 });
        let v1883: f64 = (if v707 { v1879 } else { v1834 });
        let v1884: f64 = (if v707 { v13 } else { v1835 });
        let v1885: f64 = (if v707 { v13 } else { v1851 });
        let v1886: f64 = (if v707 { v13 } else { v1852 });
        let v1887: f64 = (if v707 { v13 } else { v1853 });
        let v1888: f64 = (if v707 { v13 } else { v1854 });
        let v1889: f64 = (if v707 { v13 } else { v1855 });
        let v1890: f64 = (v1880 + v1885);
        let v1891: f64 = (v1881 + v1886);
        let v1892: f64 = (v1882 + v1887);
        let v1893: f64 = (v1883 + v1888);
        let v1894: f64 = (v1884 + v1889);
        let v1895: f64 = (v177 * v1890);
        let v1896: f64 = (v718 * v995);
        let v1897: f64 = (v177 * v1891);
        let v1898: f64 = (v1896 + v1897);
        let v1899: f64 = (v177 * v1892);
        let v1900: f64 = (v177 * v1893);
        let v1901: f64 = (v177 * v1894);
        let v1902: f64 = (self.scalar_v689 * v1895);
        let v1903: f64 = (self.scalar_v689 * v1898);
        let v1904: f64 = (self.scalar_v689 * v1899);
        let v1905: f64 = (self.scalar_v689 * v1900);
        let v1906: f64 = (self.scalar_v689 * v1901);
        let v1907: f64 = (self.scalar_v730 * v1556);
        let v1908: f64 = (self.scalar_v730 * v1557);
        let v1909: f64 = (self.scalar_v730 * v1560);
        let v1910: f64 = (self.scalar_v730 * v1563);
        let v1911: f64 = (if self.scalar_v724 { v1907 } else { v13 });
        let v1912: f64 = (if self.scalar_v724 { v1908 } else { v13 });
        let v1913: f64 = (if self.scalar_v724 { v1909 } else { v13 });
        let v1914: f64 = (if self.scalar_v724 { v1910 } else { v13 });
        let v1915: f64 = (if self.scalar_v733 { v13 } else { v1911 });
        let v1916: f64 = (if self.scalar_v733 { v13 } else { v1912 });
        let v1917: f64 = (if self.scalar_v733 { v13 } else { v1913 });
        let v1918: f64 = (if self.scalar_v733 { v13 } else { v1914 });
        let v2021: f64 = (self.scalar_v17 * v1737);
        let v2022: f64 = (self.scalar_v17 * v1738);
        let v2023: f64 = (self.scalar_v17 * v1739);
        let v2024: f64 = (self.scalar_v16 * v2021);
        let v2025: f64 = (self.scalar_v16 * v2022);
        let v2026: f64 = (self.scalar_v16 * v2023);
        let v2027: f64 = (self.scalar_v17 * v1598);
        let v2028: f64 = (self.scalar_v17 * v1599);
        let v2029: f64 = (self.scalar_v17 * v1600);
        let v2030: f64 = (self.scalar_v16 * v2027);
        let v2031: f64 = (self.scalar_v16 * v2028);
        let v2032: f64 = (self.scalar_v16 * v2029);
        let v2033: f64 = (self.scalar_v17 * v1824);
        let v2034: f64 = (self.scalar_v17 * v1825);
        let v2035: f64 = (self.scalar_v17 * v1826);
        let v2036: f64 = (self.scalar_v17 * v1827);
        let v2037: f64 = (self.scalar_v17 * v1828);
        let v2038: f64 = (self.scalar_v16 * v2033);
        let v2039: f64 = (self.scalar_v16 * v2034);
        let v2040: f64 = (self.scalar_v16 * v2035);
        let v2041: f64 = (self.scalar_v16 * v2036);
        let v2042: f64 = (self.scalar_v16 * v2037);
        let v2043: f64 = (self.scalar_v17 * v1902);
        let v2044: f64 = (self.scalar_v17 * v1903);
        let v2045: f64 = (self.scalar_v17 * v1904);
        let v2046: f64 = (self.scalar_v17 * v1905);
        let v2047: f64 = (self.scalar_v17 * v1906);
        let v2048: f64 = (self.scalar_v16 * v2043);
        let v2049: f64 = (self.scalar_v16 * v2044);
        let v2050: f64 = (self.scalar_v16 * v2045);
        let v2051: f64 = (self.scalar_v16 * v2046);
        let v2052: f64 = (self.scalar_v16 * v2047);
        let v2053: f64 = (self.scalar_v17 * v1601);
        let v2054: f64 = (self.scalar_v17 * v1602);
        let v2055: f64 = (self.scalar_v17 * v1603);
        let v2056: f64 = (self.scalar_v17 * v1604);
        let v2057: f64 = (self.scalar_v16 * v2053);
        let v2058: f64 = (self.scalar_v16 * v2054);
        let v2059: f64 = (self.scalar_v16 * v2055);
        let v2060: f64 = (self.scalar_v16 * v2056);
        let v2061: f64 = (self.scalar_v17 * v1665);
        let v2062: f64 = (self.scalar_v17 * v1666);
        let v2063: f64 = (self.scalar_v17 * v1667);
        let v2064: f64 = (self.scalar_v16 * v2061);
        let v2065: f64 = (self.scalar_v16 * v2062);
        let v2066: f64 = (self.scalar_v16 * v2063);
        let v2067: f64 = (-v1915);
        let v2068: f64 = (-v1916);
        let v2069: f64 = (-v1917);
        let v2070: f64 = (-v1918);
        let v2071: f64 = (self.scalar_v16 * v2067);
        let v2072: f64 = (self.scalar_v16 * v2068);
        let v2073: f64 = (self.scalar_v16 * v2069);
        let v2074: f64 = (self.scalar_v16 * v2070);
        let v2075: f64 = (self.scalar_v16 * v1915);
        let v2076: f64 = (self.scalar_v16 * v1916);
        let v2077: f64 = (self.scalar_v16 * v1917);
        let v2078: f64 = (self.scalar_v16 * v1918);

        let d843_dn3: f64 = v2024;
        let d843_dn5: f64 = v2025;
        let d843_dn6: f64 = v2026;
        stamper.stamp_current_reactive_node3(
            Some(nodes[5]),
            Some(nodes[6]),
            nodes[3],
            multiplicity * (d843_dn3),
            nodes[5],
            multiplicity * (d843_dn5),
            nodes[6],
            multiplicity * (d843_dn6),
        );
        let d845_dn3: f64 = v2030;
        let d845_dn5: f64 = v2031;
        let d845_dn6: f64 = v2032;
        stamper.stamp_current_reactive_node3(
            Some(nodes[5]),
            Some(nodes[6]),
            nodes[3],
            multiplicity * (d845_dn3),
            nodes[5],
            multiplicity * (d845_dn5),
            nodes[6],
            multiplicity * (d845_dn6),
        );
        let d847_dn1: f64 = v2038;
        let d847_dn3: f64 = v2039;
        let d847_dn4: f64 = v2040;
        let d847_dn5: f64 = v2041;
        let d847_dn6: f64 = v2042;
        let v847_reactive_nodes: [usize; 5] = [nodes[1], nodes[3], nodes[4], nodes[5], nodes[6]];
        let v847_reactive_node_derivatives: [f64; 5] = [d847_dn1, d847_dn3, d847_dn4, d847_dn5, d847_dn6];
        stamper.stamp_current_reactive_dense(
            Some(nodes[1]),
            Some(nodes[4]),
            &v847_reactive_nodes,
            &v847_reactive_node_derivatives,
            &[],
            &[],
            multiplicity,
        );
        let d849_dn1: f64 = v2048;
        let d849_dn3: f64 = v2049;
        let d849_dn4: f64 = v2050;
        let d849_dn5: f64 = v2051;
        let d849_dn6: f64 = v2052;
        let v849_reactive_nodes: [usize; 5] = [nodes[1], nodes[3], nodes[4], nodes[5], nodes[6]];
        let v849_reactive_node_derivatives: [f64; 5] = [d849_dn1, d849_dn3, d849_dn4, d849_dn5, d849_dn6];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[4]),
            &v849_reactive_nodes,
            &v849_reactive_node_derivatives,
            &[],
            &[],
            multiplicity,
        );
        let d851_dn3: f64 = v2057;
        let d851_dn4: f64 = v2058;
        let d851_dn5: f64 = v2059;
        let d851_dn6: f64 = v2060;
        let v851_reactive_nodes: [usize; 4] = [nodes[3], nodes[4], nodes[5], nodes[6]];
        let v851_reactive_node_derivatives: [f64; 4] = [d851_dn3, d851_dn4, d851_dn5, d851_dn6];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[4]),
            &v851_reactive_nodes,
            &v851_reactive_node_derivatives,
            &[],
            &[],
            multiplicity,
        );
        let d853_dn2: f64 = v2064;
        let d853_dn3: f64 = v2065;
        let d853_dn4: f64 = v2066;
        stamper.stamp_current_reactive_node3(
            Some(nodes[2]),
            Some(nodes[4]),
            nodes[2],
            multiplicity * (d853_dn2),
            nodes[3],
            multiplicity * (d853_dn3),
            nodes[4],
            multiplicity * (d853_dn4),
        );
        let d855_dn3: f64 = v2071;
        let d855_dn4: f64 = v2072;
        let d855_dn5: f64 = v2073;
        let d855_dn6: f64 = v2074;
        let v855_reactive_nodes: [usize; 4] = [nodes[3], nodes[4], nodes[5], nodes[6]];
        let v855_reactive_node_derivatives: [f64; 4] = [d855_dn3, d855_dn4, d855_dn5, d855_dn6];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[6]),
            &v855_reactive_nodes,
            &v855_reactive_node_derivatives,
            &[],
            &[],
            multiplicity,
        );
        let d856_dn3: f64 = v2075;
        let d856_dn4: f64 = v2076;
        let d856_dn5: f64 = v2077;
        let d856_dn6: f64 = v2078;
        let v856_reactive_nodes: [usize; 4] = [nodes[3], nodes[4], nodes[5], nodes[6]];
        let v856_reactive_node_derivatives: [f64; 4] = [d856_dn3, d856_dn4, d856_dn5, d856_dn6];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[4]),
            &v856_reactive_nodes,
            &v856_reactive_node_derivatives,
            &[],
            &[],
            multiplicity,
        );
        let mut locals = StampLocals::default();

        let assign1450_e1782: f64 = ((nv1 - nv2) / p.p40);
        let assign1450_e1783: f64 = (assign1450_e1782).abs();
        let assign1450_e1785: f64 = (assign1450_e1783).powf(p.p39);
        locals.var_vtff = assign1450_e1785;
        locals.var_vtff_dn1 = if 0.0 == 0.0 && ((p.p39) as f64).is_finite() && ((p.p39) as f64).fract() == 0.0 { if p.p39 == 0.0 { 0.0 } else { (p.p39 * ((assign1450_e1783).powf(p.p39 - 1.0) * if assign1450_e1782 >= 0.0 { (1.0 / p.p40) } else { (-(1.0 / p.p40)) })) } } else { (assign1450_e1785 * (p.p39 * (if assign1450_e1782 >= 0.0 { (1.0 / p.p40) } else { (-(1.0 / p.p40)) } / assign1450_e1783))) };
        locals.var_vtff_dn2 = if 0.0 == 0.0 && ((p.p39) as f64).is_finite() && ((p.p39) as f64).fract() == 0.0 { if p.p39 == 0.0 { 0.0 } else { (p.p39 * ((assign1450_e1783).powf(p.p39 - 1.0) * if assign1450_e1782 >= 0.0 { (-1.0 / p.p40) } else { (-(-1.0 / p.p40)) })) } } else { (assign1450_e1785 * (p.p39 * (if assign1450_e1782 >= 0.0 { (-1.0 / p.p40) } else { (-(-1.0 / p.p40)) } / assign1450_e1783))) };
        locals.var_vtff_rv = 0.0;

        let assign1460_e1788: f64 = (1.0 + locals.var_vtff);
        let assign1460_e1791: f64 = (1.0 / p.p39);
        let assign1460_e1792: f64 = (assign1460_e1788).powf(assign1460_e1791);
        let assign1460_e1794: f64 = (assign1460_e1792 - 1.0);
        locals.var_vtff1 = assign1460_e1794;
        locals.var_vtff1_dn1 = if 0.0 == 0.0 && ((assign1460_e1791) as f64).is_finite() && ((assign1460_e1791) as f64).fract() == 0.0 { if assign1460_e1791 == 0.0 { 0.0 } else { (assign1460_e1791 * ((assign1460_e1788).powf(assign1460_e1791 - 1.0) * locals.var_vtff_dn1)) } } else { (assign1460_e1792 * (assign1460_e1791 * (locals.var_vtff_dn1 / assign1460_e1788))) };
        locals.var_vtff1_dn2 = if 0.0 == 0.0 && ((assign1460_e1791) as f64).is_finite() && ((assign1460_e1791) as f64).fract() == 0.0 { if assign1460_e1791 == 0.0 { 0.0 } else { (assign1460_e1791 * ((assign1460_e1788).powf(assign1460_e1791 - 1.0) * locals.var_vtff_dn2)) } } else { (assign1460_e1792 * (assign1460_e1791 * (locals.var_vtff_dn2 / assign1460_e1788))) };
        locals.var_vtff1_rv = 0.0;

        let assign1470_e1799: f64 = (p.p41 * locals.var_vtff1);
        let assign1470_e1800: f64 = (1.0 + assign1470_e1799);
        let assign1470_e1801: f64 = (p.p19 * assign1470_e1800);
        locals.var_tff = assign1470_e1801;
        locals.var_tff_dn1 = (p.p19 * (p.p41 * locals.var_vtff1_dn1));
        locals.var_tff_dn2 = (p.p19 * (p.p41 * locals.var_vtff1_dn2));
        locals.var_tff_rv = 0.0;

        let assign1500_e1810: f64 = if p.p32 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard13 = assign1500_e1810;
        locals.var_guard13_rv = 0.0;

        let assign1920_e2216: f64 = if ((p.p30 == 1.0) && (p.p33 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard20 = assign1920_e2216;
        locals.var_guard20_rv = 0.0;

        let assign1930_e2227: f64 = if (((p.p30 == 2.0) && (p.p33 > 0.0)) && (p.p35 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard21 = assign1930_e2227;
        locals.var_guard21_rv = 0.0;

        Self::stamp_reactive_equations_block_0(ctx, stamper, p, nodes, multiplicity, &mut locals);
    }
}
