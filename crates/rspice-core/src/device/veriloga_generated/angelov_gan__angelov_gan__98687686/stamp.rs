#![allow(dead_code, unused_assignments, unused_parens, unused_variables)]

use super::state::Instance;
use crate::device::veriloga_generated::{GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper};

use crate::device::veriloga_generated::support::{AdValue as GenericAdValue, ReactiveScratch as GenericReactiveScratch, Scratch as GenericScratch};

type A = GenericAdValue<{ Instance::NODE_COUNT }, { Instance::BRANCH_COUNT }>;
type Scratch = GenericScratch<{ Instance::VARIABLE_COUNT }, { Instance::NODE_COUNT }, { Instance::BRANCH_COUNT }>;
type ReactiveScratch = GenericReactiveScratch<{ Instance::VARIABLE_COUNT }, { Instance::NODE_COUNT }, { Instance::BRANCH_COUNT }>;

const LIMEXP_MAX: f64 = 5.54062238439351e34;
#[path = "stamp_blocks_0.rs"]
mod stamp_blocks_0;

const THERMAL_VOLTAGE_PER_K: f64 = 1.380649e-23 / 1.602176634e-19;

#[inline]
fn eval_ddt<const STATE_COUNT: usize>(
    current: &mut [f64; STATE_COUNT],
    previous: &mut [f64; STATE_COUNT],
    initialized: &mut [bool; STATE_COUNT],
    ddt_active: bool,
    ddt_scale: f64,
    slot: usize,
    value: f64,
) -> f64 {
    debug_assert!(slot < STATE_COUNT, "generated ddt state slot out of range");
    let previous_value = if initialized[slot] { previous[slot] } else { value };
    current[slot] = value;
    if ddt_active {
        (value - previous_value) * ddt_scale
    } else {
        previous[slot] = value;
        initialized[slot] = true;
        0.0
    }
}

#[inline]
fn ddt_jacobian(timestep: f64, derivative: f64) -> f64 {
    if timestep.abs() > Instance::DDT_EPSILON {
        derivative / timestep
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

impl Instance {
    pub fn stamp(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedStamper<'_>) {
        let p = Box::as_ref(&self.params);
        let nodes = &(*self).nodes;
        let branches = &(*self).branches;
        let ctx_temp = ctx.temperature();
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv4 = ctx.node_voltage(nodes[4]);
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let nv11 = ctx.node_voltage(nodes[11]);
        let nv12 = ctx.node_voltage(nodes[12]);
        let nv13 = ctx.node_voltage(nodes[13]);
        let nv14 = ctx.node_voltage(nodes[14]);
        let nv15 = ctx.node_voltage(nodes[15]);
        let nv16 = ctx.node_voltage(nodes[16]);
        let nv17 = ctx.node_voltage(nodes[17]);
        let nv18 = ctx.node_voltage(nodes[18]);
        let param_given = self.param_given.as_ref();
        let multiplicity = (*self).multiplicity;
        let timestep = (*self).timestep;
        let ddt_state_current = self.ddt_state_current.as_mut();
        let ddt_state_previous = self.ddt_state_previous.as_mut();
        let ddt_state_initialized = self.ddt_state_initialized.as_mut();
        let ddt_active = timestep.abs() > Instance::DDT_EPSILON;
        let ddt_scale = if ddt_active { 1.0 / timestep } else { 0.0 };
        let v0: f64 = nv12;
        let v1: f64 = nv8;
        let v2: f64 = (v0 - v1);
        let v3: f64 = nv10;
        let v4: f64 = nv5;
        let v5: f64 = (v3 - v4);
        let v6: f64 = (-v5);
        let v7: f64 = (v4 - v1);
        let v8: f64 = nv11;
        let v9: f64 = (v8 - v1);
        let v10: f64 = nv4;
        let v11: f64 = (v10 - v1);
        let v12: f64 = nv16;
        let v13: f64 = 0.0;
        let v20: f64 = ctx_temp;
        let v22: f64 = (v20 + self.scalar_v21);
        let v23: f64 = (if self.scalar_v19 { v22 } else { self.scalar_v18 });
        let v32: f64 = nv3;
        let v33: f64 = v32.abs();
        let v34: f64 = (v23 + v33);
        let v35: f64 = (if (self.scalar_v31 != 0.0) { v34 } else { v23 });
        let v36: f64 = 8.617333262145179e-5;
        let v37: f64 = (v35 * v36);
        let v38: f64 = (v35 - self.scalar_v30);
        let v39: f64 = v38.abs();
        let v40: bool = (v39 > v13);
        let v43: bool = (v40 || self.scalar_v42);
        let v44: f64 = 1.0;
        let v46: f64 = v39.abs();
        let v47: f64 = (self.scalar_v45 * v46);
        let v48: f64 = (v44 + v47);
        let v49: f64 = (self.scalar_v41 * v48);
        let v52: f64 = (self.scalar_v51 * v46);
        let v53: f64 = (v44 + v52);
        let v54: f64 = (self.scalar_v50 * v53);
        let v55: f64 = (if v43 { v54 } else { v13 });
        let v58: f64 = (self.scalar_v57 * v46);
        let v59: f64 = (v44 + v58);
        let v60: f64 = (self.scalar_v56 * v59);
        let v61: f64 = (if v43 { v60 } else { v13 });
        let v64: f64 = (self.scalar_v63 * v46);
        let v65: f64 = (v44 + v64);
        let v66: f64 = (self.scalar_v62 * v65);
        let v67: f64 = (if v43 { v66 } else { v13 });
        let v71: f64 = (self.scalar_v70 * v46);
        let v72: f64 = (v44 + v71);
        let v73: f64 = (self.scalar_v69 * v72);
        let v74: f64 = (if v43 { v73 } else { v13 });
        let v77: f64 = (self.scalar_v76 * v39);
        let v78: f64 = (self.scalar_v75 + v77);
        let v79: f64 = (if v43 { v78 } else { v13 });
        let v82: f64 = (self.scalar_v81 * v39);
        let v83: f64 = (self.scalar_v80 + v82);
        let v84: f64 = (if v43 { v83 } else { v13 });
        let v87: f64 = (self.scalar_v86 * v39);
        let v88: f64 = (self.scalar_v85 + v87);
        let v89: f64 = (if v43 { v88 } else { v13 });
        let v98: bool = (v43 && self.scalar_v97);
        let v100: f64 = (v39 * v39);
        let v101: f64 = (self.scalar_v70 * v100);
        let v102: f64 = (v44 + v101);
        let v103: f64 = (self.scalar_v99 * v102);
        let v104: f64 = (if v98 { v103 } else { v13 });
        let v107: bool = (v43 && self.scalar_v106);
        let v108: f64 = (self.scalar_v99 * v72);
        let v109: f64 = (if v107 { v108 } else { v104 });
        let v110: bool = (!v43);
        let v111: f64 = (if v110 { self.scalar_v50 } else { v55 });
        let v112: f64 = (if v110 { self.scalar_v56 } else { v61 });
        let v113: f64 = (if v110 { self.scalar_v62 } else { v67 });
        let v114: f64 = (if v110 { self.scalar_v69 } else { v74 });
        let v115: f64 = (if v110 { self.scalar_v99 } else { v109 });
        let v116: f64 = (if v110 { self.scalar_v75 } else { v79 });
        let v117: f64 = (if v110 { self.scalar_v80 } else { v84 });
        let v118: f64 = (if v110 { self.scalar_v85 } else { v89 });
        let v123: f64 = 0.5;
        let v126: f64 = (self.scalar_v125 / v37);
        let v127: f64 = (if self.scalar_v122 { v126 } else { v13 });
        let v130: f64 = (if self.scalar_v128 { self.scalar_v129 } else { v127 });
        let v132: f64 = (self.scalar_v131 * v7);
        let v133: f64 = v132.cosh();
        let v135: f64 = (self.scalar_v134 * v11);
        let v138: f64 = 1e-12;
        let v139: f64 = (v133 * v133);
        let v140: f64 = (v138 + v139);
        let v141: f64 = (self.scalar_v137 / v140);
        let v142: f64 = (v44 + v141);
        let v143: f64 = (self.scalar_v136 * v142);
        let v145: f64 = (self.scalar_v144 * v46);
        let v146: f64 = (v44 + v145);
        let v147: f64 = (v143 * v146);
        let v150: f64 = (self.scalar_v149 * v46);
        let v151: f64 = (v44 + v150);
        let v152: f64 = (self.scalar_v148 * v151);
        let v154: f64 = (v116 - self.scalar_v153);
        let v156: f64 = (self.scalar_v155 * v7);
        let v157: f64 = v156.tanh();
        let v158: f64 = (self.scalar_v153 * v157);
        let v159: f64 = (v154 + v158);
        let v160: f64 = (v159 - v135);
        let v162: f64 = (v6 - v118);
        let v163: f64 = (self.scalar_v161 * v162);
        let v164: f64 = (v163 * v162);
        let v165: f64 = (v160 - v164);
        let v166: f64 = (self.scalar_v76 * v46);
        let v167: f64 = (v44 + v166);
        let v168: f64 = (v165 * v167);
        let v169: f64 = (v2 - v168);
        let v170: f64 = (v169 * v169);
        let v171: f64 = (v147 * v169);
        let v173: f64 = (self.scalar_v172 * v170);
        let v174: f64 = (v171 + v173);
        let v175: f64 = (v152 * v169);
        let v176: f64 = (v175 * v170);
        let v177: f64 = (v174 + v176);
        let v178: f64 = v177.tanh();
        let v179: f64 = (v44 + v178);
        let v180: f64 = { let limexp_arg = v177; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let v181: f64 = (-v177);
        let v182: f64 = { let limexp_arg = v181; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let v183: f64 = (v180 - v182);
        let v184: f64 = (v123 * v183);
        let v185: f64 = v184.tanh();
        let v186: f64 = (v44 + v185);
        let v188: f64 = (self.scalar_v155 * v179);
        let v189: f64 = (self.scalar_v187 + v188);
        let v190: f64 = (v189 * v7);
        let v191: f64 = v190.tanh();
        let v197: f64 = (v111 * v179);
        let v198: f64 = (v197 * v191);
        let v200: f64 = (self.scalar_v199 * v7);
        let v201: f64 = (v44 + v200);
        let v202: f64 = { let limexp_arg = v162; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let v203: f64 = (v112 * v202);
        let v204: f64 = (v201 + v203);
        let v205: f64 = (v198 * v204);
        let v206: f64 = (if self.scalar_v192 { v205 } else { v13 });
        let v209: f64 = (v5 - v168);
        let v210: f64 = (if self.scalar_v208 { v209 } else { v133 });
        let v211: f64 = (v210 * v210);
        let v212: f64 = (if self.scalar_v208 { v211 } else { v169 });
        let v213: f64 = (v212 * v210);
        let v214: f64 = (if self.scalar_v208 { v213 } else { v170 });
        let v215: f64 = (v147 * v210);
        let v216: f64 = (self.scalar_v172 * v212);
        let v217: f64 = (v215 + v216);
        let v218: f64 = (v152 * v214);
        let v219: f64 = (v217 + v218);
        let v220: f64 = (if self.scalar_v208 { v219 } else { v13 });
        let v221: f64 = v220.tanh();
        let v222: f64 = (v44 + v221);
        let v223: f64 = (if self.scalar_v208 { v222 } else { v13 });
        let v224: f64 = (self.scalar_v155 * v223);
        let v225: f64 = (self.scalar_v187 + v224);
        let v226: f64 = (if self.scalar_v208 { v225 } else { v13 });
        let v228: f64 = (self.scalar_v227 * v179);
        let v229: f64 = (self.scalar_v199 + v228);
        let v230: f64 = (if self.scalar_v208 { v229 } else { v13 });
        let v231: f64 = (v44 + v191);
        let v232: f64 = (v197 * v231);
        let v233: f64 = (v230 * v7);
        let v234: f64 = (v44 + v233);
        let v236: f64 = (v7 - v118);
        let v237: f64 = (self.scalar_v235 * v236);
        let v238: f64 = { let limexp_arg = v237; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let v239: f64 = (v112 * v238);
        let v240: f64 = (v234 + v239);
        let v241: f64 = (v232 * v240);
        let v242: f64 = (if self.scalar_v208 { v241 } else { v13 });
        let v243: f64 = (self.scalar_v227 * v223);
        let v244: f64 = (self.scalar_v199 + v243);
        let v245: f64 = (if self.scalar_v208 { v244 } else { v13 });
        let v246: f64 = (v226 * v7);
        let v247: f64 = v246.tanh();
        let v248: f64 = (if self.scalar_v208 { v247 } else { v13 });
        let v249: f64 = (v111 * v223);
        let v250: f64 = (v44 - v248);
        let v251: f64 = (v249 * v250);
        let v252: f64 = (v245 * v7);
        let v253: f64 = (v44 - v252);
        let v254: f64 = (v251 * v253);
        let v255: f64 = (if self.scalar_v208 { v254 } else { v13 });
        let v256: f64 = (v242 - v255);
        let v257: f64 = (v123 * v256);
        let v258: f64 = (if self.scalar_v208 { v257 } else { v206 });
        let v262: f64 = (if self.scalar_v261 { v169 } else { v210 });
        let v263: f64 = (v262 * v262);
        let v264: f64 = (if self.scalar_v261 { v263 } else { v212 });
        let v265: f64 = (self.scalar_v172 * v264);
        let v266: f64 = (v262 + v265);
        let v267: f64 = (v152 * v264);
        let v268: f64 = (v267 * v262);
        let v269: f64 = (v266 + v268);
        let v270: f64 = (v147 * v269);
        let v271: f64 = (if self.scalar_v261 { v270 } else { v177 });
        let v272: f64 = { let limexp_arg = v271; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let v273: f64 = (-v271);
        let v274: f64 = { let limexp_arg = v273; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let v275: f64 = (v272 - v274);
        let v276: f64 = (v123 * v275);
        let v277: f64 = v276.tanh();
        let v278: f64 = (v44 + v277);
        let v279: f64 = (if self.scalar_v261 { v278 } else { v186 });
        let v280: f64 = (self.scalar_v155 * v279);
        let v281: f64 = (self.scalar_v187 + v280);
        let v282: f64 = (if self.scalar_v261 { v281 } else { v13 });
        let v283: f64 = (v282 * v7);
        let v284: f64 = v283.tanh();
        let v285: f64 = (if self.scalar_v261 { v284 } else { v13 });
        let v286: f64 = (self.scalar_v227 * v279);
        let v287: f64 = (self.scalar_v199 + v286);
        let v288: f64 = (if self.scalar_v261 { v287 } else { v230 });
        let v289: f64 = (v111 * v279);
        let v290: f64 = (v289 * v285);
        let v291: f64 = (v288 * v7);
        let v292: f64 = (v44 + v291);
        let v293: f64 = (self.scalar_v235 * v162);
        let v294: f64 = { let limexp_arg = v293; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let v295: f64 = (v112 * v294);
        let v296: f64 = (v292 + v295);
        let v297: f64 = (v290 * v296);
        let v298: f64 = (if self.scalar_v261 { v297 } else { v258 });
        let v302: f64 = (if self.scalar_v301 { v169 } else { v262 });
        let v303: f64 = (v302 * v302);
        let v304: f64 = (if self.scalar_v301 { v303 } else { v264 });
        let v305: f64 = (self.scalar_v172 * v304);
        let v306: f64 = (v302 + v305);
        let v307: f64 = (v152 * v304);
        let v308: f64 = (v307 * v302);
        let v309: f64 = (v306 + v308);
        let v310: f64 = (v147 * v309);
        let v311: f64 = (if self.scalar_v301 { v310 } else { v271 });
        let v312: f64 = (if self.scalar_v301 { v209 } else { v214 });
        let v313: f64 = (v312 * v312);
        let v314: f64 = (if self.scalar_v301 { v313 } else { v13 });
        let v315: f64 = (self.scalar_v172 * v314);
        let v316: f64 = (v312 + v315);
        let v317: f64 = (v152 * v312);
        let v318: f64 = (v317 * v314);
        let v319: f64 = (v316 + v318);
        let v320: f64 = (v147 * v319);
        let v321: f64 = (if self.scalar_v301 { v320 } else { v220 });
        let v322: f64 = { let limexp_arg = v311; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let v323: f64 = (-v311);
        let v324: f64 = { let limexp_arg = v323; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let v325: f64 = (v322 - v324);
        let v326: f64 = (v123 * v325);
        let v327: f64 = v326.tanh();
        let v328: f64 = (v44 + v327);
        let v329: f64 = (if self.scalar_v301 { v328 } else { v279 });
        let v330: f64 = { let limexp_arg = v321; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let v331: f64 = (-v321);
        let v332: f64 = { let limexp_arg = v331; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let v333: f64 = (v330 - v332);
        let v334: f64 = (v123 * v333);
        let v335: f64 = v334.tanh();
        let v336: f64 = (v44 + v335);
        let v337: f64 = (if self.scalar_v301 { v336 } else { v13 });
        let v338: f64 = (self.scalar_v155 * v329);
        let v339: f64 = (self.scalar_v187 + v338);
        let v340: f64 = (if self.scalar_v301 { v339 } else { v282 });
        let v341: f64 = (self.scalar_v155 * v337);
        let v342: f64 = (self.scalar_v187 + v341);
        let v343: f64 = (if self.scalar_v301 { v342 } else { v13 });
        let v344: f64 = (v340 * v7);
        let v345: f64 = v344.tanh();
        let v346: f64 = (if self.scalar_v301 { v345 } else { v285 });
        let v347: f64 = (v343 * v7);
        let v348: f64 = v347.tanh();
        let v349: f64 = (if self.scalar_v301 { v348 } else { v13 });
        let v350: f64 = (self.scalar_v227 * v337);
        let v351: f64 = (self.scalar_v199 + v350);
        let v352: f64 = (if self.scalar_v301 { v351 } else { v13 });
        let v353: f64 = (self.scalar_v227 * v329);
        let v354: f64 = (self.scalar_v199 + v353);
        let v355: f64 = (if self.scalar_v301 { v354 } else { v13 });
        let v356: f64 = (v111 * v329);
        let v357: f64 = (v44 + v346);
        let v358: f64 = (v356 * v357);
        let v359: f64 = (v355 * v7);
        let v360: f64 = (v44 + v359);
        let v361: f64 = (v360 + v239);
        let v362: f64 = (v358 * v361);
        let v363: f64 = (if self.scalar_v301 { v362 } else { v242 });
        let v364: f64 = (v111 * v337);
        let v365: f64 = (v44 - v349);
        let v366: f64 = (v364 * v365);
        let v367: f64 = (v352 * v7);
        let v368: f64 = (v44 - v367);
        let v369: f64 = (v366 * v368);
        let v370: f64 = (if self.scalar_v301 { v369 } else { v255 });
        let v371: f64 = (v363 - v370);
        let v372: f64 = (v123 * v371);
        let v373: f64 = (if self.scalar_v301 { v372 } else { v298 });
        let v377: f64 = (if self.scalar_v376 { v229 } else { v288 });
        let v378: f64 = (if self.scalar_v376 { v339 } else { v340 });
        let v379: f64 = (v378 * v7);
        let v380: f64 = v379.tanh();
        let v381: f64 = (if self.scalar_v376 { v380 } else { v346 });
        let v382: f64 = (v378 * v11);
        let v383: f64 = v382.tanh();
        let v384: f64 = (if self.scalar_v376 { v383 } else { v13 });
        let v386: f64 = (self.scalar_v385 * v384);
        let v387: f64 = (v381 + v386);
        let v388: f64 = (v197 * v387);
        let v389: f64 = (self.scalar_v385 * v11);
        let v390: f64 = (v7 + v389);
        let v391: f64 = (v377 * v390);
        let v392: f64 = (v44 + v391);
        let v393: f64 = (v392 + v239);
        let v394: f64 = (v388 * v393);
        let v395: f64 = (if self.scalar_v376 { v394 } else { v373 });
        let v401: f64 = -1.0;
        let v402: f64 = (v401 * v117);
        let v403: f64 = v402.tanh();
        let v404: f64 = (v130 * v403);
        let v405: f64 = { let limexp_arg = v404; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let v406: f64 = (if self.scalar_v400 { v405 } else { v302 });
        let v407: f64 = (v9 - v117);
        let v408: f64 = (if self.scalar_v400 { v407 } else { v13 });
        let v409: f64 = (-v9);
        let v411: f64 = (v409 - self.scalar_v410);
        let v412: f64 = (if self.scalar_v400 { v411 } else { v13 });
        let v413: f64 = (v5 - v117);
        let v414: f64 = (if self.scalar_v400 { v413 } else { v13 });
        let v416: f64 = (v6 - self.scalar_v415);
        let v417: f64 = (if self.scalar_v400 { v416 } else { v13 });
        let v419: f64 = (-v130);
        let v420: f64 = (v419 * v117);
        let v421: f64 = { let limexp_arg = v420; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let v422: f64 = (if self.scalar_v418 { v421 } else { v406 });
        let v433: f64 = v407.tanh();
        let v434: f64 = (if self.scalar_v432 { v433 } else { v408 });
        let v435: f64 = v413.tanh();
        let v436: f64 = (if self.scalar_v432 { v435 } else { v414 });
        let v439: f64 = (if self.scalar_v438 { v407 } else { v434 });
        let v440: f64 = (if self.scalar_v438 { v413 } else { v436 });
        let v441: f64 = (if self.scalar_v418 { v411 } else { v412 });
        let v442: f64 = (if self.scalar_v418 { v416 } else { v417 });
        let v443: f64 = (self.scalar_v423 * v441);
        let v444: f64 = { let limexp_arg = v443; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let v445: f64 = (v444 - self.scalar_v427);
        let v447: f64 = (v130 * v439);
        let v448: f64 = { let limexp_arg = v447; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let v452: f64 = (self.scalar_v451 * v445);
        let v453: f64 = (v448 - v452);
        let v454: f64 = (v453 - v422);
        let v455: f64 = (self.scalar_v446 * v454);
        let v456: f64 = (self.scalar_v423 * v442);
        let v457: f64 = { let limexp_arg = v456; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let v458: f64 = (v457 - self.scalar_v430);
        let v459: f64 = (v130 * v440);
        let v460: f64 = { let limexp_arg = v459; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let v461: f64 = (self.scalar_v451 * v458);
        let v462: f64 = (v460 - v461);
        let v463: f64 = (v462 - v422);
        let v464: f64 = (self.scalar_v446 * v463);
        let v488: f64 = 5.5226012e-23;
        let v489: f64 = (v488 * v35);
        let v491: f64 = (v489 * self.scalar_v490);
        let v492: f64 = (v491 * v113);
        let v497: f64 = (v492 * self.scalar_v496);
        let v498: f64 = (if self.scalar_v487 { v497 } else { v13 });
        let v499: f64 = (v498 * v498);
        let v500: f64 = (v44 - v499);
        let v501: f64 = v500.sqrt();
        let v502: f64 = (if self.scalar_v487 { v501 } else { v13 });
        let v503: f64 = (-v498);
        let v504: f64 = 3.141592653589793;
        let v505: f64 = (v503 * v504);
        let v506: f64 = (if self.scalar_v487 { v505 } else { v13 });
        let v508: f64 = (-v395);
        let v510: f64 = nv15;
        let v511: f64 = (self.scalar_v509 * v510);
        let v513: f64 = nv7;
        let v514: f64 = (v513 - v4);
        let v515: f64 = (self.scalar_v512 * v514);
        let v517: f64 = (self.scalar_v516 * v7);
        let v518: f64 = nv6;
        let v519: f64 = (v518 - v10);
        let v520: f64 = (v114 * v519);
        let v521: f64 = (v138 * v519);
        let v524: f64 = (v8 - v0);
        let v525: f64 = (v524 / v115);
        let v526: f64 = (if self.scalar_v468 { v525 } else { v13 });
        let v530: f64 = nv14;
        let v531: f64 = (v8 - v530);
        let v532: f64 = (self.scalar_v529 * v531);
        let v533: f64 = (v530 - v1);
        let v534: f64 = (v533 / self.scalar_v469);
        let v535: f64 = (if self.scalar_v470 { v534 } else { v13 });
        let v538: f64 = nv13;
        let v539: f64 = (v538 - v3);
        let v540: f64 = (v539 / self.scalar_v471);
        let v541: f64 = (if self.scalar_v472 { v540 } else { v13 });
        let v544: f64 = (v538 - v8);
        let v545: f64 = (v544 / self.scalar_v473);
        let v546: f64 = (if self.scalar_v474 { v545 } else { v13 });
        let v555: f64 = 1e-15;
        let v556: f64 = nv2;
        let v557: f64 = (v0 - v556);
        let v558: f64 = (v557 * v138);
        let v559: f64 = nv17;
        let v560: f64 = (if self.scalar_v487 { v559 } else { v13 });
        let v561: f64 = nv18;
        let v562: f64 = (if self.scalar_v487 { v561 } else { v13 });
        let v563: f64 = (v506 * v559);
        let v564: f64 = (v502 * v561);
        let v565: f64 = (v563 + v564);
        let v566: f64 = (if self.scalar_v487 { v565 } else { v13 });
        let v567: f64 = (v395 * v7);
        let v568: f64 = v567.abs();
        let v569: f64 = (v455 * v9);
        let v570: f64 = v569.abs();
        let v571: f64 = (v568 + v570);
        let v572: f64 = (v401 * v571);
        let v573: f64 = (if self.scalar_v507 { v572 } else { v13 });
        let v574: f64 = (v32 / v49);
        let v575: f64 = (if self.scalar_v507 { v574 } else { v13 });
        let v577: f64 = (v32 * v138);
        let v578: f64 = (if self.scalar_v576 { v577 } else { v13 });
        let v580: f64 = v132.sinh();
        let v581: f64 = (v580 * self.scalar_v131);
        let v582: f64 = (v580 * self.scalar_v579);
        let v584: f64 = (v581 * v133);
        let v585: f64 = (v133 * v581);
        let v586: f64 = (v584 + v585);
        let v587: f64 = (v582 * v133);
        let v588: f64 = (v133 * v582);
        let v589: f64 = (v587 + v588);
        let v590: f64 = (self.scalar_v137 * v586);
        let v591: f64 = (-v590);
        let v592: f64 = (v140 * v140);
        let v593: f64 = (v591 / v592);
        let v594: f64 = (self.scalar_v137 * v589);
        let v595: f64 = (-v594);
        let v596: f64 = (v595 / v592);
        let v597: f64 = (self.scalar_v136 * v593);
        let v598: f64 = (self.scalar_v136 * v596);
        let v599: f64 = (v597 * v146);
        let v600: f64 = (v598 * v146);
        let v602: f64 = (v157 * v157);
        let v603: f64 = (v44 - v602);
        let v604: f64 = (v603 * self.scalar_v155);
        let v605: f64 = (v603 * self.scalar_v601);
        let v606: f64 = (self.scalar_v153 * v604);
        let v607: f64 = (self.scalar_v153 * v605);
        let v609: f64 = (v607 - self.scalar_v583);
        let v611: f64 = (v163 + v163);
        let v612: f64 = (self.scalar_v610 * v162);
        let v613: f64 = (v163 * v401);
        let v614: f64 = (v612 + v613);
        let v615: f64 = (v606 - v611);
        let v616: f64 = (-v614);
        let v617: f64 = (self.scalar_v608 * v167);
        let v618: f64 = (v615 * v167);
        let v619: f64 = (v609 * v167);
        let v620: f64 = (v616 * v167);
        let v621: f64 = (-v617);
        let v622: f64 = (-v618);
        let v623: f64 = (v401 - v619);
        let v624: f64 = (-v620);
        let v625: f64 = (v621 * v169);
        let v626: f64 = (v169 * v621);
        let v627: f64 = (v625 + v626);
        let v628: f64 = (v622 * v169);
        let v629: f64 = (v169 * v622);
        let v630: f64 = (v628 + v629);
        let v631: f64 = (v623 * v169);
        let v632: f64 = (v169 * v623);
        let v633: f64 = (v631 + v632);
        let v634: f64 = (v624 * v169);
        let v635: f64 = (v169 * v624);
        let v636: f64 = (v634 + v635);
        let v637: f64 = (v169 + v169);
        let v638: f64 = (v147 * v621);
        let v639: f64 = (v599 * v169);
        let v640: f64 = (v147 * v622);
        let v641: f64 = (v639 + v640);
        let v642: f64 = (v600 * v169);
        let v643: f64 = (v147 * v623);
        let v644: f64 = (v642 + v643);
        let v645: f64 = (v147 * v624);
        let v646: f64 = (self.scalar_v172 * v627);
        let v647: f64 = (self.scalar_v172 * v630);
        let v648: f64 = (self.scalar_v172 * v633);
        let v649: f64 = (self.scalar_v172 * v636);
        let v650: f64 = (self.scalar_v172 * v637);
        let v651: f64 = (v638 + v646);
        let v652: f64 = (v641 + v647);
        let v653: f64 = (v644 + v648);
        let v654: f64 = (v645 + v649);
        let v655: f64 = (v147 + v650);
        let v656: f64 = (v152 * v621);
        let v657: f64 = (v152 * v622);
        let v658: f64 = (v152 * v623);
        let v659: f64 = (v152 * v624);
        let v660: f64 = (v656 * v170);
        let v661: f64 = (v175 * v627);
        let v662: f64 = (v660 + v661);
        let v663: f64 = (v657 * v170);
        let v664: f64 = (v175 * v630);
        let v665: f64 = (v663 + v664);
        let v666: f64 = (v658 * v170);
        let v667: f64 = (v175 * v633);
        let v668: f64 = (v666 + v667);
        let v669: f64 = (v659 * v170);
        let v670: f64 = (v175 * v636);
        let v671: f64 = (v669 + v670);
        let v672: f64 = (v152 * v170);
        let v673: f64 = (v175 * v637);
        let v674: f64 = (v672 + v673);
        let v675: f64 = (v651 + v662);
        let v676: f64 = (v652 + v665);
        let v677: f64 = (v653 + v668);
        let v678: f64 = (v654 + v671);
        let v679: f64 = (v655 + v674);
        let v680: f64 = (v178 * v178);
        let v681: f64 = (v44 - v680);
        let v682: f64 = (v681 * v675);
        let v683: f64 = (v681 * v676);
        let v684: f64 = (v681 * v677);
        let v685: f64 = (v681 * v678);
        let v686: f64 = (v681 * v679);
        let v687: f64 = { let limexp_arg = v177; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v688: f64 = (v687 * v675);
        let v689: f64 = (v687 * v676);
        let v690: f64 = (v687 * v677);
        let v691: f64 = (v687 * v678);
        let v692: f64 = (v687 * v679);
        let v693: f64 = (-v675);
        let v694: f64 = (-v676);
        let v695: f64 = (-v677);
        let v696: f64 = (-v678);
        let v697: f64 = (-v679);
        let v698: f64 = { let limexp_arg = v181; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v699: f64 = (v698 * v693);
        let v700: f64 = (v698 * v694);
        let v701: f64 = (v698 * v695);
        let v702: f64 = (v698 * v696);
        let v703: f64 = (v698 * v697);
        let v704: f64 = (v688 - v699);
        let v705: f64 = (v689 - v700);
        let v706: f64 = (v690 - v701);
        let v707: f64 = (v691 - v702);
        let v708: f64 = (v692 - v703);
        let v709: f64 = (v123 * v704);
        let v710: f64 = (v123 * v705);
        let v711: f64 = (v123 * v706);
        let v712: f64 = (v123 * v707);
        let v713: f64 = (v123 * v708);
        let v714: f64 = (v185 * v185);
        let v715: f64 = (v44 - v714);
        let v716: f64 = (v715 * v709);
        let v717: f64 = (v715 * v710);
        let v718: f64 = (v715 * v711);
        let v719: f64 = (v715 * v712);
        let v720: f64 = (v715 * v713);
        let v721: f64 = (self.scalar_v155 * v682);
        let v722: f64 = (self.scalar_v155 * v683);
        let v723: f64 = (self.scalar_v155 * v684);
        let v724: f64 = (self.scalar_v155 * v685);
        let v725: f64 = (self.scalar_v155 * v686);
        let v726: f64 = (v721 * v7);
        let v727: f64 = (v722 * v7);
        let v728: f64 = (v727 + v189);
        let v729: f64 = (v723 * v7);
        let v730: f64 = (v189 * v401);
        let v731: f64 = (v729 + v730);
        let v732: f64 = (v724 * v7);
        let v733: f64 = (v725 * v7);
        let v734: f64 = (v191 * v191);
        let v735: f64 = (v44 - v734);
        let v736: f64 = (v735 * v726);
        let v737: f64 = (v735 * v728);
        let v738: f64 = (v735 * v731);
        let v739: f64 = (v735 * v732);
        let v740: f64 = (v735 * v733);
        let v741: f64 = (v111 * v682);
        let v742: f64 = (v111 * v683);
        let v743: f64 = (v111 * v684);
        let v744: f64 = (v111 * v685);
        let v745: f64 = (v111 * v686);
        let v746: f64 = (v741 * v191);
        let v747: f64 = (v197 * v736);
        let v748: f64 = (v746 + v747);
        let v749: f64 = (v742 * v191);
        let v750: f64 = (v197 * v737);
        let v751: f64 = (v749 + v750);
        let v752: f64 = (v743 * v191);
        let v753: f64 = (v197 * v738);
        let v754: f64 = (v752 + v753);
        let v755: f64 = (v744 * v191);
        let v756: f64 = (v197 * v739);
        let v757: f64 = (v755 + v756);
        let v758: f64 = (v745 * v191);
        let v759: f64 = (v197 * v740);
        let v760: f64 = (v758 + v759);
        let v762: f64 = { let limexp_arg = v162; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v763: f64 = (v762 * v401);
        let v764: f64 = (v112 * v762);
        let v765: f64 = (v112 * v763);
        let v766: f64 = (self.scalar_v199 + v764);
        let v767: f64 = (v748 * v204);
        let v768: f64 = (v751 * v204);
        let v769: f64 = (v198 * v766);
        let v770: f64 = (v768 + v769);
        let v771: f64 = (v754 * v204);
        let v772: f64 = (v198 * self.scalar_v761);
        let v773: f64 = (v771 + v772);
        let v774: f64 = (v757 * v204);
        let v775: f64 = (v198 * v765);
        let v776: f64 = (v774 + v775);
        let v777: f64 = (v760 * v204);
        let v778: f64 = (if self.scalar_v192 { v767 } else { v13 });
        let v779: f64 = (if self.scalar_v192 { v770 } else { v13 });
        let v780: f64 = (if self.scalar_v192 { v773 } else { v13 });
        let v781: f64 = (if self.scalar_v192 { v776 } else { v13 });
        let v782: f64 = (if self.scalar_v192 { v777 } else { v13 });
        let v783: f64 = (v401 - v618);
        let v784: f64 = (-v619);
        let v785: f64 = (v44 - v620);
        let v786: f64 = (if self.scalar_v208 { v621 } else { v13 });
        let v787: f64 = (if self.scalar_v208 { v783 } else { v581 });
        let v788: f64 = (if self.scalar_v208 { v784 } else { v582 });
        let v789: f64 = (if self.scalar_v208 { v785 } else { v13 });
        let v790: f64 = (v786 * v210);
        let v791: f64 = (v210 * v786);
        let v792: f64 = (v790 + v791);
        let v793: f64 = (v787 * v210);
        let v794: f64 = (v210 * v787);
        let v795: f64 = (v793 + v794);
        let v796: f64 = (v788 * v210);
        let v797: f64 = (v210 * v788);
        let v798: f64 = (v796 + v797);
        let v799: f64 = (v789 * v210);
        let v800: f64 = (v210 * v789);
        let v801: f64 = (v799 + v800);
        let v802: f64 = (if self.scalar_v208 { v792 } else { v621 });
        let v803: f64 = (if self.scalar_v208 { v795 } else { v622 });
        let v804: f64 = (if self.scalar_v208 { v798 } else { v623 });
        let v805: f64 = (if self.scalar_v208 { v801 } else { v624 });
        let v807: f64 = (v802 * v210);
        let v808: f64 = (v212 * v786);
        let v809: f64 = (v807 + v808);
        let v810: f64 = (v803 * v210);
        let v811: f64 = (v212 * v787);
        let v812: f64 = (v810 + v811);
        let v813: f64 = (v804 * v210);
        let v814: f64 = (v212 * v788);
        let v815: f64 = (v813 + v814);
        let v816: f64 = (v805 * v210);
        let v817: f64 = (v212 * v789);
        let v818: f64 = (v816 + v817);
        let v819: f64 = (self.scalar_v806 * v210);
        let v820: f64 = (if self.scalar_v208 { v809 } else { v627 });
        let v821: f64 = (if self.scalar_v208 { v812 } else { v630 });
        let v822: f64 = (if self.scalar_v208 { v815 } else { v633 });
        let v823: f64 = (if self.scalar_v208 { v818 } else { v636 });
        let v824: f64 = (if self.scalar_v208 { v819 } else { v637 });
        let v825: f64 = (v147 * v786);
        let v826: f64 = (v599 * v210);
        let v827: f64 = (v147 * v787);
        let v828: f64 = (v826 + v827);
        let v829: f64 = (v600 * v210);
        let v830: f64 = (v147 * v788);
        let v831: f64 = (v829 + v830);
        let v832: f64 = (v147 * v789);
        let v833: f64 = (self.scalar_v172 * v802);
        let v834: f64 = (self.scalar_v172 * v803);
        let v835: f64 = (self.scalar_v172 * v804);
        let v836: f64 = (self.scalar_v172 * v805);
        let v838: f64 = (v825 + v833);
        let v839: f64 = (v828 + v834);
        let v840: f64 = (v831 + v835);
        let v841: f64 = (v832 + v836);
        let v842: f64 = (v152 * v820);
        let v843: f64 = (v152 * v821);
        let v844: f64 = (v152 * v822);
        let v845: f64 = (v152 * v823);
        let v846: f64 = (v152 * v824);
        let v847: f64 = (v838 + v842);
        let v848: f64 = (v839 + v843);
        let v849: f64 = (v840 + v844);
        let v850: f64 = (v841 + v845);
        let v851: f64 = (self.scalar_v837 + v846);
        let v852: f64 = (if self.scalar_v208 { v847 } else { v13 });
        let v853: f64 = (if self.scalar_v208 { v848 } else { v13 });
        let v854: f64 = (if self.scalar_v208 { v849 } else { v13 });
        let v855: f64 = (if self.scalar_v208 { v850 } else { v13 });
        let v856: f64 = (if self.scalar_v208 { v851 } else { v13 });
        let v857: f64 = (v221 * v221);
        let v858: f64 = (v44 - v857);
        let v859: f64 = (v858 * v852);
        let v860: f64 = (v858 * v853);
        let v861: f64 = (v858 * v854);
        let v862: f64 = (v858 * v855);
        let v863: f64 = (v858 * v856);
        let v864: f64 = (if self.scalar_v208 { v859 } else { v13 });
        let v865: f64 = (if self.scalar_v208 { v860 } else { v13 });
        let v866: f64 = (if self.scalar_v208 { v861 } else { v13 });
        let v867: f64 = (if self.scalar_v208 { v862 } else { v13 });
        let v868: f64 = (if self.scalar_v208 { v863 } else { v13 });
        let v869: f64 = (self.scalar_v155 * v864);
        let v870: f64 = (self.scalar_v155 * v865);
        let v871: f64 = (self.scalar_v155 * v866);
        let v872: f64 = (self.scalar_v155 * v867);
        let v873: f64 = (self.scalar_v155 * v868);
        let v874: f64 = (if self.scalar_v208 { v869 } else { v13 });
        let v875: f64 = (if self.scalar_v208 { v870 } else { v13 });
        let v876: f64 = (if self.scalar_v208 { v871 } else { v13 });
        let v877: f64 = (if self.scalar_v208 { v872 } else { v13 });
        let v878: f64 = (if self.scalar_v208 { v873 } else { v13 });
        let v879: f64 = (self.scalar_v227 * v682);
        let v880: f64 = (self.scalar_v227 * v683);
        let v881: f64 = (self.scalar_v227 * v684);
        let v882: f64 = (self.scalar_v227 * v685);
        let v883: f64 = (self.scalar_v227 * v686);
        let v884: f64 = (if self.scalar_v208 { v879 } else { v13 });
        let v885: f64 = (if self.scalar_v208 { v880 } else { v13 });
        let v886: f64 = (if self.scalar_v208 { v881 } else { v13 });
        let v887: f64 = (if self.scalar_v208 { v882 } else { v13 });
        let v888: f64 = (if self.scalar_v208 { v883 } else { v13 });
        let v889: f64 = (v741 * v231);
        let v890: f64 = (v889 + v747);
        let v891: f64 = (v742 * v231);
        let v892: f64 = (v891 + v750);
        let v893: f64 = (v743 * v231);
        let v894: f64 = (v893 + v753);
        let v895: f64 = (v744 * v231);
        let v896: f64 = (v895 + v756);
        let v897: f64 = (v745 * v231);
        let v898: f64 = (v897 + v759);
        let v899: f64 = (v884 * v7);
        let v900: f64 = (v885 * v7);
        let v901: f64 = (v900 + v230);
        let v902: f64 = (v886 * v7);
        let v903: f64 = (v230 * v401);
        let v904: f64 = (v902 + v903);
        let v905: f64 = (v887 * v7);
        let v906: f64 = (v888 * v7);
        let v908: f64 = { let limexp_arg = v237; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v909: f64 = (v908 * self.scalar_v235);
        let v910: f64 = (v908 * self.scalar_v907);
        let v911: f64 = (v112 * v909);
        let v912: f64 = (v112 * v910);
        let v913: f64 = (v901 + v911);
        let v914: f64 = (v904 + v912);
        let v915: f64 = (v890 * v240);
        let v916: f64 = (v232 * v899);
        let v917: f64 = (v915 + v916);
        let v918: f64 = (v892 * v240);
        let v919: f64 = (v232 * v913);
        let v920: f64 = (v918 + v919);
        let v921: f64 = (v894 * v240);
        let v922: f64 = (v232 * v914);
        let v923: f64 = (v921 + v922);
        let v924: f64 = (v896 * v240);
        let v925: f64 = (v232 * v905);
        let v926: f64 = (v924 + v925);
        let v927: f64 = (v898 * v240);
        let v928: f64 = (v232 * v906);
        let v929: f64 = (v927 + v928);
        let v930: f64 = (if self.scalar_v208 { v917 } else { v13 });
        let v931: f64 = (if self.scalar_v208 { v920 } else { v13 });
        let v932: f64 = (if self.scalar_v208 { v923 } else { v13 });
        let v933: f64 = (if self.scalar_v208 { v926 } else { v13 });
        let v934: f64 = (if self.scalar_v208 { v929 } else { v13 });
        let v935: f64 = (self.scalar_v227 * v864);
        let v936: f64 = (self.scalar_v227 * v865);
        let v937: f64 = (self.scalar_v227 * v866);
        let v938: f64 = (self.scalar_v227 * v867);
        let v939: f64 = (self.scalar_v227 * v868);
        let v940: f64 = (if self.scalar_v208 { v935 } else { v13 });
        let v941: f64 = (if self.scalar_v208 { v936 } else { v13 });
        let v942: f64 = (if self.scalar_v208 { v937 } else { v13 });
        let v943: f64 = (if self.scalar_v208 { v938 } else { v13 });
        let v944: f64 = (if self.scalar_v208 { v939 } else { v13 });
        let v945: f64 = (v874 * v7);
        let v946: f64 = (v875 * v7);
        let v947: f64 = (v946 + v226);
        let v948: f64 = (v876 * v7);
        let v949: f64 = (v226 * v401);
        let v950: f64 = (v948 + v949);
        let v951: f64 = (v877 * v7);
        let v952: f64 = (v878 * v7);
        let v953: f64 = (v247 * v247);
        let v954: f64 = (v44 - v953);
        let v955: f64 = (v954 * v945);
        let v956: f64 = (v954 * v947);
        let v957: f64 = (v954 * v950);
        let v958: f64 = (v954 * v951);
        let v959: f64 = (v954 * v952);
        let v960: f64 = (if self.scalar_v208 { v955 } else { v13 });
        let v961: f64 = (if self.scalar_v208 { v956 } else { v13 });
        let v962: f64 = (if self.scalar_v208 { v957 } else { v13 });
        let v963: f64 = (if self.scalar_v208 { v958 } else { v13 });
        let v964: f64 = (if self.scalar_v208 { v959 } else { v13 });
        let v965: f64 = (v111 * v864);
        let v966: f64 = (v111 * v865);
        let v967: f64 = (v111 * v866);
        let v968: f64 = (v111 * v867);
        let v969: f64 = (v111 * v868);
        let v970: f64 = (-v960);
        let v971: f64 = (-v961);
        let v972: f64 = (-v962);
        let v973: f64 = (-v963);
        let v974: f64 = (-v964);
        let v975: f64 = (v965 * v250);
        let v976: f64 = (v249 * v970);
        let v977: f64 = (v975 + v976);
        let v978: f64 = (v966 * v250);
        let v979: f64 = (v249 * v971);
        let v980: f64 = (v978 + v979);
        let v981: f64 = (v967 * v250);
        let v982: f64 = (v249 * v972);
        let v983: f64 = (v981 + v982);
        let v984: f64 = (v968 * v250);
        let v985: f64 = (v249 * v973);
        let v986: f64 = (v984 + v985);
        let v987: f64 = (v969 * v250);
        let v988: f64 = (v249 * v974);
        let v989: f64 = (v987 + v988);
        let v990: f64 = (v940 * v7);
        let v991: f64 = (v941 * v7);
        let v992: f64 = (v991 + v245);
        let v993: f64 = (v942 * v7);
        let v994: f64 = (v245 * v401);
        let v995: f64 = (v993 + v994);
        let v996: f64 = (v943 * v7);
        let v997: f64 = (v944 * v7);
        let v998: f64 = (-v990);
        let v999: f64 = (-v992);
        let v1000: f64 = (-v995);
        let v1001: f64 = (-v996);
        let v1002: f64 = (-v997);
        let v1003: f64 = (v977 * v253);
        let v1004: f64 = (v251 * v998);
        let v1005: f64 = (v1003 + v1004);
        let v1006: f64 = (v980 * v253);
        let v1007: f64 = (v251 * v999);
        let v1008: f64 = (v1006 + v1007);
        let v1009: f64 = (v983 * v253);
        let v1010: f64 = (v251 * v1000);
        let v1011: f64 = (v1009 + v1010);
        let v1012: f64 = (v986 * v253);
        let v1013: f64 = (v251 * v1001);
        let v1014: f64 = (v1012 + v1013);
        let v1015: f64 = (v989 * v253);
        let v1016: f64 = (v251 * v1002);
        let v1017: f64 = (v1015 + v1016);
        let v1018: f64 = (if self.scalar_v208 { v1005 } else { v13 });
        let v1019: f64 = (if self.scalar_v208 { v1008 } else { v13 });
        let v1020: f64 = (if self.scalar_v208 { v1011 } else { v13 });
        let v1021: f64 = (if self.scalar_v208 { v1014 } else { v13 });
        let v1022: f64 = (if self.scalar_v208 { v1017 } else { v13 });
        let v1023: f64 = (v930 - v1018);
        let v1024: f64 = (v931 - v1019);
        let v1025: f64 = (v932 - v1020);
        let v1026: f64 = (v933 - v1021);
        let v1027: f64 = (v934 - v1022);
        let v1028: f64 = (v123 * v1023);
        let v1029: f64 = (v123 * v1024);
        let v1030: f64 = (v123 * v1025);
        let v1031: f64 = (v123 * v1026);
        let v1032: f64 = (v123 * v1027);
        let v1033: f64 = (if self.scalar_v208 { v1028 } else { v778 });
        let v1034: f64 = (if self.scalar_v208 { v1029 } else { v779 });
        let v1035: f64 = (if self.scalar_v208 { v1030 } else { v780 });
        let v1036: f64 = (if self.scalar_v208 { v1031 } else { v781 });
        let v1037: f64 = (if self.scalar_v208 { v1032 } else { v782 });
        let v1038: f64 = (if self.scalar_v261 { v621 } else { v786 });
        let v1039: f64 = (if self.scalar_v261 { v622 } else { v787 });
        let v1040: f64 = (if self.scalar_v261 { v623 } else { v788 });
        let v1041: f64 = (if self.scalar_v261 { v624 } else { v789 });
        let v1043: f64 = (v1038 * v262);
        let v1044: f64 = (v262 * v1038);
        let v1045: f64 = (v1043 + v1044);
        let v1046: f64 = (v1039 * v262);
        let v1047: f64 = (v262 * v1039);
        let v1048: f64 = (v1046 + v1047);
        let v1049: f64 = (v1040 * v262);
        let v1050: f64 = (v262 * v1040);
        let v1051: f64 = (v1049 + v1050);
        let v1052: f64 = (v1041 * v262);
        let v1053: f64 = (v262 * v1041);
        let v1054: f64 = (v1052 + v1053);
        let v1055: f64 = (self.scalar_v1042 * v262);
        let v1056: f64 = (v262 * self.scalar_v1042);
        let v1057: f64 = (v1055 + v1056);
        let v1058: f64 = (if self.scalar_v261 { v1045 } else { v802 });
        let v1059: f64 = (if self.scalar_v261 { v1048 } else { v803 });
        let v1060: f64 = (if self.scalar_v261 { v1051 } else { v804 });
        let v1061: f64 = (if self.scalar_v261 { v1054 } else { v805 });
        let v1062: f64 = (if self.scalar_v261 { v1057 } else { self.scalar_v806 });
        let v1063: f64 = (self.scalar_v172 * v1058);
        let v1064: f64 = (self.scalar_v172 * v1059);
        let v1065: f64 = (self.scalar_v172 * v1060);
        let v1066: f64 = (self.scalar_v172 * v1061);
        let v1067: f64 = (self.scalar_v172 * v1062);
        let v1068: f64 = (v1038 + v1063);
        let v1069: f64 = (v1039 + v1064);
        let v1070: f64 = (v1040 + v1065);
        let v1071: f64 = (v1041 + v1066);
        let v1072: f64 = (self.scalar_v1042 + v1067);
        let v1073: f64 = (v152 * v1058);
        let v1074: f64 = (v152 * v1059);
        let v1075: f64 = (v152 * v1060);
        let v1076: f64 = (v152 * v1061);
        let v1077: f64 = (v152 * v1062);
        let v1078: f64 = (v1073 * v262);
        let v1079: f64 = (v267 * v1038);
        let v1080: f64 = (v1078 + v1079);
        let v1081: f64 = (v1074 * v262);
        let v1082: f64 = (v267 * v1039);
        let v1083: f64 = (v1081 + v1082);
        let v1084: f64 = (v1075 * v262);
        let v1085: f64 = (v267 * v1040);
        let v1086: f64 = (v1084 + v1085);
        let v1087: f64 = (v1076 * v262);
        let v1088: f64 = (v267 * v1041);
        let v1089: f64 = (v1087 + v1088);
        let v1090: f64 = (v1077 * v262);
        let v1091: f64 = (v267 * self.scalar_v1042);
        let v1092: f64 = (v1090 + v1091);
        let v1093: f64 = (v1068 + v1080);
        let v1094: f64 = (v1069 + v1083);
        let v1095: f64 = (v1070 + v1086);
        let v1096: f64 = (v1071 + v1089);
        let v1097: f64 = (v1072 + v1092);
        let v1098: f64 = (v147 * v1093);
        let v1099: f64 = (v599 * v269);
        let v1100: f64 = (v147 * v1094);
        let v1101: f64 = (v1099 + v1100);
        let v1102: f64 = (v600 * v269);
        let v1103: f64 = (v147 * v1095);
        let v1104: f64 = (v1102 + v1103);
        let v1105: f64 = (v147 * v1096);
        let v1106: f64 = (v147 * v1097);
        let v1107: f64 = (if self.scalar_v261 { v1098 } else { v675 });
        let v1108: f64 = (if self.scalar_v261 { v1101 } else { v676 });
        let v1109: f64 = (if self.scalar_v261 { v1104 } else { v677 });
        let v1110: f64 = (if self.scalar_v261 { v1105 } else { v678 });
        let v1111: f64 = (if self.scalar_v261 { v1106 } else { v679 });
        let v1112: f64 = { let limexp_arg = v271; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v1113: f64 = (v1112 * v1107);
        let v1114: f64 = (v1112 * v1108);
        let v1115: f64 = (v1112 * v1109);
        let v1116: f64 = (v1112 * v1110);
        let v1117: f64 = (v1112 * v1111);
        let v1118: f64 = (-v1107);
        let v1119: f64 = (-v1108);
        let v1120: f64 = (-v1109);
        let v1121: f64 = (-v1110);
        let v1122: f64 = (-v1111);
        let v1123: f64 = { let limexp_arg = v273; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v1124: f64 = (v1123 * v1118);
        let v1125: f64 = (v1123 * v1119);
        let v1126: f64 = (v1123 * v1120);
        let v1127: f64 = (v1123 * v1121);
        let v1128: f64 = (v1123 * v1122);
        let v1129: f64 = (v1113 - v1124);
        let v1130: f64 = (v1114 - v1125);
        let v1131: f64 = (v1115 - v1126);
        let v1132: f64 = (v1116 - v1127);
        let v1133: f64 = (v1117 - v1128);
        let v1134: f64 = (v123 * v1129);
        let v1135: f64 = (v123 * v1130);
        let v1136: f64 = (v123 * v1131);
        let v1137: f64 = (v123 * v1132);
        let v1138: f64 = (v123 * v1133);
        let v1139: f64 = (v277 * v277);
        let v1140: f64 = (v44 - v1139);
        let v1141: f64 = (v1140 * v1134);
        let v1142: f64 = (v1140 * v1135);
        let v1143: f64 = (v1140 * v1136);
        let v1144: f64 = (v1140 * v1137);
        let v1145: f64 = (v1140 * v1138);
        let v1146: f64 = (if self.scalar_v261 { v1141 } else { v716 });
        let v1147: f64 = (if self.scalar_v261 { v1142 } else { v717 });
        let v1148: f64 = (if self.scalar_v261 { v1143 } else { v718 });
        let v1149: f64 = (if self.scalar_v261 { v1144 } else { v719 });
        let v1150: f64 = (if self.scalar_v261 { v1145 } else { v720 });
        let v1151: f64 = (self.scalar_v155 * v1146);
        let v1152: f64 = (self.scalar_v155 * v1147);
        let v1153: f64 = (self.scalar_v155 * v1148);
        let v1154: f64 = (self.scalar_v155 * v1149);
        let v1155: f64 = (self.scalar_v155 * v1150);
        let v1156: f64 = (if self.scalar_v261 { v1151 } else { v13 });
        let v1157: f64 = (if self.scalar_v261 { v1152 } else { v13 });
        let v1158: f64 = (if self.scalar_v261 { v1153 } else { v13 });
        let v1159: f64 = (if self.scalar_v261 { v1154 } else { v13 });
        let v1160: f64 = (if self.scalar_v261 { v1155 } else { v13 });
        let v1161: f64 = (v1156 * v7);
        let v1162: f64 = (v1157 * v7);
        let v1163: f64 = (v1162 + v282);
        let v1164: f64 = (v1158 * v7);
        let v1165: f64 = (v282 * v401);
        let v1166: f64 = (v1164 + v1165);
        let v1167: f64 = (v1159 * v7);
        let v1168: f64 = (v1160 * v7);
        let v1169: f64 = (v284 * v284);
        let v1170: f64 = (v44 - v1169);
        let v1171: f64 = (v1170 * v1161);
        let v1172: f64 = (v1170 * v1163);
        let v1173: f64 = (v1170 * v1166);
        let v1174: f64 = (v1170 * v1167);
        let v1175: f64 = (v1170 * v1168);
        let v1176: f64 = (if self.scalar_v261 { v1171 } else { v13 });
        let v1177: f64 = (if self.scalar_v261 { v1172 } else { v13 });
        let v1178: f64 = (if self.scalar_v261 { v1173 } else { v13 });
        let v1179: f64 = (if self.scalar_v261 { v1174 } else { v13 });
        let v1180: f64 = (if self.scalar_v261 { v1175 } else { v13 });
        let v1181: f64 = (self.scalar_v227 * v1146);
        let v1182: f64 = (self.scalar_v227 * v1147);
        let v1183: f64 = (self.scalar_v227 * v1148);
        let v1184: f64 = (self.scalar_v227 * v1149);
        let v1185: f64 = (self.scalar_v227 * v1150);
        let v1186: f64 = (if self.scalar_v261 { v1181 } else { v884 });
        let v1187: f64 = (if self.scalar_v261 { v1182 } else { v885 });
        let v1188: f64 = (if self.scalar_v261 { v1183 } else { v886 });
        let v1189: f64 = (if self.scalar_v261 { v1184 } else { v887 });
        let v1190: f64 = (if self.scalar_v261 { v1185 } else { v888 });
        let v1191: f64 = (v111 * v1146);
        let v1192: f64 = (v111 * v1147);
        let v1193: f64 = (v111 * v1148);
        let v1194: f64 = (v111 * v1149);
        let v1195: f64 = (v111 * v1150);
        let v1196: f64 = (v1191 * v285);
        let v1197: f64 = (v289 * v1176);
        let v1198: f64 = (v1196 + v1197);
        let v1199: f64 = (v1192 * v285);
        let v1200: f64 = (v289 * v1177);
        let v1201: f64 = (v1199 + v1200);
        let v1202: f64 = (v1193 * v285);
        let v1203: f64 = (v289 * v1178);
        let v1204: f64 = (v1202 + v1203);
        let v1205: f64 = (v1194 * v285);
        let v1206: f64 = (v289 * v1179);
        let v1207: f64 = (v1205 + v1206);
        let v1208: f64 = (v1195 * v285);
        let v1209: f64 = (v289 * v1180);
        let v1210: f64 = (v1208 + v1209);
        let v1211: f64 = (v1186 * v7);
        let v1212: f64 = (v1187 * v7);
        let v1213: f64 = (v1212 + v288);
        let v1214: f64 = (v1188 * v7);
        let v1215: f64 = (v288 * v401);
        let v1216: f64 = (v1214 + v1215);
        let v1217: f64 = (v1189 * v7);
        let v1218: f64 = (v1190 * v7);
        let v1219: f64 = { let limexp_arg = v293; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v1220: f64 = (v1219 * self.scalar_v235);
        let v1221: f64 = (v1219 * self.scalar_v907);
        let v1222: f64 = (v112 * v1220);
        let v1223: f64 = (v112 * v1221);
        let v1224: f64 = (v1213 + v1222);
        let v1225: f64 = (v1217 + v1223);
        let v1226: f64 = (v1198 * v296);
        let v1227: f64 = (v290 * v1211);
        let v1228: f64 = (v1226 + v1227);
        let v1229: f64 = (v1201 * v296);
        let v1230: f64 = (v290 * v1224);
        let v1231: f64 = (v1229 + v1230);
        let v1232: f64 = (v1204 * v296);
        let v1233: f64 = (v290 * v1216);
        let v1234: f64 = (v1232 + v1233);
        let v1235: f64 = (v1207 * v296);
        let v1236: f64 = (v290 * v1225);
        let v1237: f64 = (v1235 + v1236);
        let v1238: f64 = (v1210 * v296);
        let v1239: f64 = (v290 * v1218);
        let v1240: f64 = (v1238 + v1239);
        let v1241: f64 = (if self.scalar_v261 { v1228 } else { v1033 });
        let v1242: f64 = (if self.scalar_v261 { v1231 } else { v1034 });
        let v1243: f64 = (if self.scalar_v261 { v1234 } else { v1035 });
        let v1244: f64 = (if self.scalar_v261 { v1237 } else { v1036 });
        let v1245: f64 = (if self.scalar_v261 { v1240 } else { v1037 });
        let v1246: f64 = (if self.scalar_v301 { v621 } else { v1038 });
        let v1247: f64 = (if self.scalar_v301 { v622 } else { v1039 });
        let v1248: f64 = (if self.scalar_v301 { v623 } else { v1040 });
        let v1249: f64 = (if self.scalar_v301 { v624 } else { v1041 });
        let v1251: f64 = (v1246 * v302);
        let v1252: f64 = (v302 * v1246);
        let v1253: f64 = (v1251 + v1252);
        let v1254: f64 = (v1247 * v302);
        let v1255: f64 = (v302 * v1247);
        let v1256: f64 = (v1254 + v1255);
        let v1257: f64 = (v1248 * v302);
        let v1258: f64 = (v302 * v1248);
        let v1259: f64 = (v1257 + v1258);
        let v1260: f64 = (v1249 * v302);
        let v1261: f64 = (v302 * v1249);
        let v1262: f64 = (v1260 + v1261);
        let v1263: f64 = (self.scalar_v1250 * v302);
        let v1264: f64 = (v302 * self.scalar_v1250);
        let v1265: f64 = (v1263 + v1264);
        let v1266: f64 = (if self.scalar_v301 { v1253 } else { v1058 });
        let v1267: f64 = (if self.scalar_v301 { v1256 } else { v1059 });
        let v1268: f64 = (if self.scalar_v301 { v1259 } else { v1060 });
        let v1269: f64 = (if self.scalar_v301 { v1262 } else { v1061 });
        let v1270: f64 = (if self.scalar_v301 { v1265 } else { v1062 });
        let v1271: f64 = (self.scalar_v172 * v1266);
        let v1272: f64 = (self.scalar_v172 * v1267);
        let v1273: f64 = (self.scalar_v172 * v1268);
        let v1274: f64 = (self.scalar_v172 * v1269);
        let v1275: f64 = (self.scalar_v172 * v1270);
        let v1276: f64 = (v1246 + v1271);
        let v1277: f64 = (v1247 + v1272);
        let v1278: f64 = (v1248 + v1273);
        let v1279: f64 = (v1249 + v1274);
        let v1280: f64 = (self.scalar_v1250 + v1275);
        let v1281: f64 = (v152 * v1266);
        let v1282: f64 = (v152 * v1267);
        let v1283: f64 = (v152 * v1268);
        let v1284: f64 = (v152 * v1269);
        let v1285: f64 = (v152 * v1270);
        let v1286: f64 = (v1281 * v302);
        let v1287: f64 = (v307 * v1246);
        let v1288: f64 = (v1286 + v1287);
        let v1289: f64 = (v1282 * v302);
        let v1290: f64 = (v307 * v1247);
        let v1291: f64 = (v1289 + v1290);
        let v1292: f64 = (v1283 * v302);
        let v1293: f64 = (v307 * v1248);
        let v1294: f64 = (v1292 + v1293);
        let v1295: f64 = (v1284 * v302);
        let v1296: f64 = (v307 * v1249);
        let v1297: f64 = (v1295 + v1296);
        let v1298: f64 = (v1285 * v302);
        let v1299: f64 = (v307 * self.scalar_v1250);
        let v1300: f64 = (v1298 + v1299);
        let v1301: f64 = (v1276 + v1288);
        let v1302: f64 = (v1277 + v1291);
        let v1303: f64 = (v1278 + v1294);
        let v1304: f64 = (v1279 + v1297);
        let v1305: f64 = (v1280 + v1300);
        let v1306: f64 = (v147 * v1301);
        let v1307: f64 = (v599 * v309);
        let v1308: f64 = (v147 * v1302);
        let v1309: f64 = (v1307 + v1308);
        let v1310: f64 = (v600 * v309);
        let v1311: f64 = (v147 * v1303);
        let v1312: f64 = (v1310 + v1311);
        let v1313: f64 = (v147 * v1304);
        let v1314: f64 = (v147 * v1305);
        let v1315: f64 = (if self.scalar_v301 { v1306 } else { v1107 });
        let v1316: f64 = (if self.scalar_v301 { v1309 } else { v1108 });
        let v1317: f64 = (if self.scalar_v301 { v1312 } else { v1109 });
        let v1318: f64 = (if self.scalar_v301 { v1313 } else { v1110 });
        let v1319: f64 = (if self.scalar_v301 { v1314 } else { v1111 });
        let v1320: f64 = (if self.scalar_v301 { v621 } else { v820 });
        let v1321: f64 = (if self.scalar_v301 { v783 } else { v821 });
        let v1322: f64 = (if self.scalar_v301 { v784 } else { v822 });
        let v1323: f64 = (if self.scalar_v301 { v785 } else { v823 });
        let v1324: f64 = (if self.scalar_v301 { v13 } else { v824 });
        let v1325: f64 = (v1320 * v312);
        let v1326: f64 = (v312 * v1320);
        let v1327: f64 = (v1325 + v1326);
        let v1328: f64 = (v1321 * v312);
        let v1329: f64 = (v312 * v1321);
        let v1330: f64 = (v1328 + v1329);
        let v1331: f64 = (v1322 * v312);
        let v1332: f64 = (v312 * v1322);
        let v1333: f64 = (v1331 + v1332);
        let v1334: f64 = (v1323 * v312);
        let v1335: f64 = (v312 * v1323);
        let v1336: f64 = (v1334 + v1335);
        let v1337: f64 = (v1324 * v312);
        let v1338: f64 = (v312 * v1324);
        let v1339: f64 = (v1337 + v1338);
        let v1340: f64 = (if self.scalar_v301 { v1327 } else { v13 });
        let v1341: f64 = (if self.scalar_v301 { v1330 } else { v13 });
        let v1342: f64 = (if self.scalar_v301 { v1333 } else { v13 });
        let v1343: f64 = (if self.scalar_v301 { v1336 } else { v13 });
        let v1344: f64 = (if self.scalar_v301 { v1339 } else { v13 });
        let v1345: f64 = (self.scalar_v172 * v1340);
        let v1346: f64 = (self.scalar_v172 * v1341);
        let v1347: f64 = (self.scalar_v172 * v1342);
        let v1348: f64 = (self.scalar_v172 * v1343);
        let v1349: f64 = (self.scalar_v172 * v1344);
        let v1350: f64 = (v1320 + v1345);
        let v1351: f64 = (v1321 + v1346);
        let v1352: f64 = (v1322 + v1347);
        let v1353: f64 = (v1323 + v1348);
        let v1354: f64 = (v1324 + v1349);
        let v1355: f64 = (v152 * v1320);
        let v1356: f64 = (v152 * v1321);
        let v1357: f64 = (v152 * v1322);
        let v1358: f64 = (v152 * v1323);
        let v1359: f64 = (v152 * v1324);
        let v1360: f64 = (v1355 * v314);
        let v1361: f64 = (v317 * v1340);
        let v1362: f64 = (v1360 + v1361);
        let v1363: f64 = (v1356 * v314);
        let v1364: f64 = (v317 * v1341);
        let v1365: f64 = (v1363 + v1364);
        let v1366: f64 = (v1357 * v314);
        let v1367: f64 = (v317 * v1342);
        let v1368: f64 = (v1366 + v1367);
        let v1369: f64 = (v1358 * v314);
        let v1370: f64 = (v317 * v1343);
        let v1371: f64 = (v1369 + v1370);
        let v1372: f64 = (v1359 * v314);
        let v1373: f64 = (v317 * v1344);
        let v1374: f64 = (v1372 + v1373);
        let v1375: f64 = (v1350 + v1362);
        let v1376: f64 = (v1351 + v1365);
        let v1377: f64 = (v1352 + v1368);
        let v1378: f64 = (v1353 + v1371);
        let v1379: f64 = (v1354 + v1374);
        let v1380: f64 = (v147 * v1375);
        let v1381: f64 = (v599 * v319);
        let v1382: f64 = (v147 * v1376);
        let v1383: f64 = (v1381 + v1382);
        let v1384: f64 = (v600 * v319);
        let v1385: f64 = (v147 * v1377);
        let v1386: f64 = (v1384 + v1385);
        let v1387: f64 = (v147 * v1378);
        let v1388: f64 = (v147 * v1379);
        let v1389: f64 = (if self.scalar_v301 { v1380 } else { v852 });
        let v1390: f64 = (if self.scalar_v301 { v1383 } else { v853 });
        let v1391: f64 = (if self.scalar_v301 { v1386 } else { v854 });
        let v1392: f64 = (if self.scalar_v301 { v1387 } else { v855 });
        let v1393: f64 = (if self.scalar_v301 { v1388 } else { v856 });
        let v1394: f64 = { let limexp_arg = v311; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v1395: f64 = (v1394 * v1315);
        let v1396: f64 = (v1394 * v1316);
        let v1397: f64 = (v1394 * v1317);
        let v1398: f64 = (v1394 * v1318);
        let v1399: f64 = (v1394 * v1319);
        let v1400: f64 = (-v1315);
        let v1401: f64 = (-v1316);
        let v1402: f64 = (-v1317);
        let v1403: f64 = (-v1318);
        let v1404: f64 = (-v1319);
        let v1405: f64 = { let limexp_arg = v323; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v1406: f64 = (v1405 * v1400);
        let v1407: f64 = (v1405 * v1401);
        let v1408: f64 = (v1405 * v1402);
        let v1409: f64 = (v1405 * v1403);
        let v1410: f64 = (v1405 * v1404);
        let v1411: f64 = (v1395 - v1406);
        let v1412: f64 = (v1396 - v1407);
        let v1413: f64 = (v1397 - v1408);
        let v1414: f64 = (v1398 - v1409);
        let v1415: f64 = (v1399 - v1410);
        let v1416: f64 = (v123 * v1411);
        let v1417: f64 = (v123 * v1412);
        let v1418: f64 = (v123 * v1413);
        let v1419: f64 = (v123 * v1414);
        let v1420: f64 = (v123 * v1415);
        let v1421: f64 = (v327 * v327);
        let v1422: f64 = (v44 - v1421);
        let v1423: f64 = (v1422 * v1416);
        let v1424: f64 = (v1422 * v1417);
        let v1425: f64 = (v1422 * v1418);
        let v1426: f64 = (v1422 * v1419);
        let v1427: f64 = (v1422 * v1420);
        let v1428: f64 = (if self.scalar_v301 { v1423 } else { v1146 });
        let v1429: f64 = (if self.scalar_v301 { v1424 } else { v1147 });
        let v1430: f64 = (if self.scalar_v301 { v1425 } else { v1148 });
        let v1431: f64 = (if self.scalar_v301 { v1426 } else { v1149 });
        let v1432: f64 = (if self.scalar_v301 { v1427 } else { v1150 });
        let v1433: f64 = { let limexp_arg = v321; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v1434: f64 = (v1433 * v1389);
        let v1435: f64 = (v1433 * v1390);
        let v1436: f64 = (v1433 * v1391);
        let v1437: f64 = (v1433 * v1392);
        let v1438: f64 = (v1433 * v1393);
        let v1439: f64 = (-v1389);
        let v1440: f64 = (-v1390);
        let v1441: f64 = (-v1391);
        let v1442: f64 = (-v1392);
        let v1443: f64 = (-v1393);
        let v1444: f64 = { let limexp_arg = v331; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v1445: f64 = (v1444 * v1439);
        let v1446: f64 = (v1444 * v1440);
        let v1447: f64 = (v1444 * v1441);
        let v1448: f64 = (v1444 * v1442);
        let v1449: f64 = (v1444 * v1443);
        let v1450: f64 = (v1434 - v1445);
        let v1451: f64 = (v1435 - v1446);
        let v1452: f64 = (v1436 - v1447);
        let v1453: f64 = (v1437 - v1448);
        let v1454: f64 = (v1438 - v1449);
        let v1455: f64 = (v123 * v1450);
        let v1456: f64 = (v123 * v1451);
        let v1457: f64 = (v123 * v1452);
        let v1458: f64 = (v123 * v1453);
        let v1459: f64 = (v123 * v1454);
        let v1460: f64 = (v335 * v335);
        let v1461: f64 = (v44 - v1460);
        let v1462: f64 = (v1461 * v1455);
        let v1463: f64 = (v1461 * v1456);
        let v1464: f64 = (v1461 * v1457);
        let v1465: f64 = (v1461 * v1458);
        let v1466: f64 = (v1461 * v1459);
        let v1467: f64 = (if self.scalar_v301 { v1462 } else { v13 });
        let v1468: f64 = (if self.scalar_v301 { v1463 } else { v13 });
        let v1469: f64 = (if self.scalar_v301 { v1464 } else { v13 });
        let v1470: f64 = (if self.scalar_v301 { v1465 } else { v13 });
        let v1471: f64 = (if self.scalar_v301 { v1466 } else { v13 });
        let v1472: f64 = (self.scalar_v155 * v1428);
        let v1473: f64 = (self.scalar_v155 * v1429);
        let v1474: f64 = (self.scalar_v155 * v1430);
        let v1475: f64 = (self.scalar_v155 * v1431);
        let v1476: f64 = (self.scalar_v155 * v1432);
        let v1477: f64 = (if self.scalar_v301 { v1472 } else { v1156 });
        let v1478: f64 = (if self.scalar_v301 { v1473 } else { v1157 });
        let v1479: f64 = (if self.scalar_v301 { v1474 } else { v1158 });
        let v1480: f64 = (if self.scalar_v301 { v1475 } else { v1159 });
        let v1481: f64 = (if self.scalar_v301 { v1476 } else { v1160 });
        let v1482: f64 = (self.scalar_v155 * v1467);
        let v1483: f64 = (self.scalar_v155 * v1468);
        let v1484: f64 = (self.scalar_v155 * v1469);
        let v1485: f64 = (self.scalar_v155 * v1470);
        let v1486: f64 = (self.scalar_v155 * v1471);
        let v1487: f64 = (if self.scalar_v301 { v1482 } else { v13 });
        let v1488: f64 = (if self.scalar_v301 { v1483 } else { v13 });
        let v1489: f64 = (if self.scalar_v301 { v1484 } else { v13 });
        let v1490: f64 = (if self.scalar_v301 { v1485 } else { v13 });
        let v1491: f64 = (if self.scalar_v301 { v1486 } else { v13 });
        let v1492: f64 = (v1477 * v7);
        let v1493: f64 = (v1478 * v7);
        let v1494: f64 = (v1493 + v340);
        let v1495: f64 = (v1479 * v7);
        let v1496: f64 = (v340 * v401);
        let v1497: f64 = (v1495 + v1496);
        let v1498: f64 = (v1480 * v7);
        let v1499: f64 = (v1481 * v7);
        let v1500: f64 = (v345 * v345);
        let v1501: f64 = (v44 - v1500);
        let v1502: f64 = (v1501 * v1492);
        let v1503: f64 = (v1501 * v1494);
        let v1504: f64 = (v1501 * v1497);
        let v1505: f64 = (v1501 * v1498);
        let v1506: f64 = (v1501 * v1499);
        let v1507: f64 = (if self.scalar_v301 { v1502 } else { v1176 });
        let v1508: f64 = (if self.scalar_v301 { v1503 } else { v1177 });
        let v1509: f64 = (if self.scalar_v301 { v1504 } else { v1178 });
        let v1510: f64 = (if self.scalar_v301 { v1505 } else { v1179 });
        let v1511: f64 = (if self.scalar_v301 { v1506 } else { v1180 });
        let v1512: f64 = (v1487 * v7);
        let v1513: f64 = (v1488 * v7);
        let v1514: f64 = (v1513 + v343);
        let v1515: f64 = (v1489 * v7);
        let v1516: f64 = (v343 * v401);
        let v1517: f64 = (v1515 + v1516);
        let v1518: f64 = (v1490 * v7);
        let v1519: f64 = (v1491 * v7);
        let v1520: f64 = (v348 * v348);
        let v1521: f64 = (v44 - v1520);
        let v1522: f64 = (v1521 * v1512);
        let v1523: f64 = (v1521 * v1514);
        let v1524: f64 = (v1521 * v1517);
        let v1525: f64 = (v1521 * v1518);
        let v1526: f64 = (v1521 * v1519);
        let v1527: f64 = (if self.scalar_v301 { v1522 } else { v13 });
        let v1528: f64 = (if self.scalar_v301 { v1523 } else { v13 });
        let v1529: f64 = (if self.scalar_v301 { v1524 } else { v13 });
        let v1530: f64 = (if self.scalar_v301 { v1525 } else { v13 });
        let v1531: f64 = (if self.scalar_v301 { v1526 } else { v13 });
        let v1532: f64 = (self.scalar_v227 * v1467);
        let v1533: f64 = (self.scalar_v227 * v1468);
        let v1534: f64 = (self.scalar_v227 * v1469);
        let v1535: f64 = (self.scalar_v227 * v1470);
        let v1536: f64 = (self.scalar_v227 * v1471);
        let v1537: f64 = (if self.scalar_v301 { v1532 } else { v13 });
        let v1538: f64 = (if self.scalar_v301 { v1533 } else { v13 });
        let v1539: f64 = (if self.scalar_v301 { v1534 } else { v13 });
        let v1540: f64 = (if self.scalar_v301 { v1535 } else { v13 });
        let v1541: f64 = (if self.scalar_v301 { v1536 } else { v13 });
        let v1542: f64 = (self.scalar_v227 * v1428);
        let v1543: f64 = (self.scalar_v227 * v1429);
        let v1544: f64 = (self.scalar_v227 * v1430);
        let v1545: f64 = (self.scalar_v227 * v1431);
        let v1546: f64 = (self.scalar_v227 * v1432);
        let v1547: f64 = (if self.scalar_v301 { v1542 } else { v13 });
        let v1548: f64 = (if self.scalar_v301 { v1543 } else { v13 });
        let v1549: f64 = (if self.scalar_v301 { v1544 } else { v13 });
        let v1550: f64 = (if self.scalar_v301 { v1545 } else { v13 });
        let v1551: f64 = (if self.scalar_v301 { v1546 } else { v13 });
        let v1552: f64 = (v111 * v1428);
        let v1553: f64 = (v111 * v1429);
        let v1554: f64 = (v111 * v1430);
        let v1555: f64 = (v111 * v1431);
        let v1556: f64 = (v111 * v1432);
        let v1557: f64 = (v1552 * v357);
        let v1558: f64 = (v356 * v1507);
        let v1559: f64 = (v1557 + v1558);
        let v1560: f64 = (v1553 * v357);
        let v1561: f64 = (v356 * v1508);
        let v1562: f64 = (v1560 + v1561);
        let v1563: f64 = (v1554 * v357);
        let v1564: f64 = (v356 * v1509);
        let v1565: f64 = (v1563 + v1564);
        let v1566: f64 = (v1555 * v357);
        let v1567: f64 = (v356 * v1510);
        let v1568: f64 = (v1566 + v1567);
        let v1569: f64 = (v1556 * v357);
        let v1570: f64 = (v356 * v1511);
        let v1571: f64 = (v1569 + v1570);
        let v1572: f64 = (v1547 * v7);
        let v1573: f64 = (v1548 * v7);
        let v1574: f64 = (v1573 + v355);
        let v1575: f64 = (v1549 * v7);
        let v1576: f64 = (v355 * v401);
        let v1577: f64 = (v1575 + v1576);
        let v1578: f64 = (v1550 * v7);
        let v1579: f64 = (v1551 * v7);
        let v1580: f64 = (v1574 + v911);
        let v1581: f64 = (v1577 + v912);
        let v1582: f64 = (v1559 * v361);
        let v1583: f64 = (v358 * v1572);
        let v1584: f64 = (v1582 + v1583);
        let v1585: f64 = (v1562 * v361);
        let v1586: f64 = (v358 * v1580);
        let v1587: f64 = (v1585 + v1586);
        let v1588: f64 = (v1565 * v361);
        let v1589: f64 = (v358 * v1581);
        let v1590: f64 = (v1588 + v1589);
        let v1591: f64 = (v1568 * v361);
        let v1592: f64 = (v358 * v1578);
        let v1593: f64 = (v1591 + v1592);
        let v1594: f64 = (v1571 * v361);
        let v1595: f64 = (v358 * v1579);
        let v1596: f64 = (v1594 + v1595);
        let v1597: f64 = (if self.scalar_v301 { v1584 } else { v930 });
        let v1598: f64 = (if self.scalar_v301 { v1587 } else { v931 });
        let v1599: f64 = (if self.scalar_v301 { v1590 } else { v932 });
        let v1600: f64 = (if self.scalar_v301 { v1593 } else { v933 });
        let v1601: f64 = (if self.scalar_v301 { v1596 } else { v934 });
        let v1602: f64 = (v111 * v1467);
        let v1603: f64 = (v111 * v1468);
        let v1604: f64 = (v111 * v1469);
        let v1605: f64 = (v111 * v1470);
        let v1606: f64 = (v111 * v1471);
        let v1607: f64 = (-v1527);
        let v1608: f64 = (-v1528);
        let v1609: f64 = (-v1529);
        let v1610: f64 = (-v1530);
        let v1611: f64 = (-v1531);
        let v1612: f64 = (v1602 * v365);
        let v1613: f64 = (v364 * v1607);
        let v1614: f64 = (v1612 + v1613);
        let v1615: f64 = (v1603 * v365);
        let v1616: f64 = (v364 * v1608);
        let v1617: f64 = (v1615 + v1616);
        let v1618: f64 = (v1604 * v365);
        let v1619: f64 = (v364 * v1609);
        let v1620: f64 = (v1618 + v1619);
        let v1621: f64 = (v1605 * v365);
        let v1622: f64 = (v364 * v1610);
        let v1623: f64 = (v1621 + v1622);
        let v1624: f64 = (v1606 * v365);
        let v1625: f64 = (v364 * v1611);
        let v1626: f64 = (v1624 + v1625);
        let v1627: f64 = (v1537 * v7);
        let v1628: f64 = (v1538 * v7);
        let v1629: f64 = (v1628 + v352);
        let v1630: f64 = (v1539 * v7);
        let v1631: f64 = (v352 * v401);
        let v1632: f64 = (v1630 + v1631);
        let v1633: f64 = (v1540 * v7);
        let v1634: f64 = (v1541 * v7);
        let v1635: f64 = (-v1627);
        let v1636: f64 = (-v1629);
        let v1637: f64 = (-v1632);
        let v1638: f64 = (-v1633);
        let v1639: f64 = (-v1634);
        let v1640: f64 = (v1614 * v368);
        let v1641: f64 = (v366 * v1635);
        let v1642: f64 = (v1640 + v1641);
        let v1643: f64 = (v1617 * v368);
        let v1644: f64 = (v366 * v1636);
        let v1645: f64 = (v1643 + v1644);
        let v1646: f64 = (v1620 * v368);
        let v1647: f64 = (v366 * v1637);
        let v1648: f64 = (v1646 + v1647);
        let v1649: f64 = (v1623 * v368);
        let v1650: f64 = (v366 * v1638);
        let v1651: f64 = (v1649 + v1650);
        let v1652: f64 = (v1626 * v368);
        let v1653: f64 = (v366 * v1639);
        let v1654: f64 = (v1652 + v1653);
        let v1655: f64 = (if self.scalar_v301 { v1642 } else { v1018 });
        let v1656: f64 = (if self.scalar_v301 { v1645 } else { v1019 });
        let v1657: f64 = (if self.scalar_v301 { v1648 } else { v1020 });
        let v1658: f64 = (if self.scalar_v301 { v1651 } else { v1021 });
        let v1659: f64 = (if self.scalar_v301 { v1654 } else { v1022 });
        let v1660: f64 = (v1597 - v1655);
        let v1661: f64 = (v1598 - v1656);
        let v1662: f64 = (v1599 - v1657);
        let v1663: f64 = (v1600 - v1658);
        let v1664: f64 = (v1601 - v1659);
        let v1665: f64 = (v123 * v1660);
        let v1666: f64 = (v123 * v1661);
        let v1667: f64 = (v123 * v1662);
        let v1668: f64 = (v123 * v1663);
        let v1669: f64 = (v123 * v1664);
        let v1670: f64 = (if self.scalar_v301 { v1665 } else { v1241 });
        let v1671: f64 = (if self.scalar_v301 { v1666 } else { v1242 });
        let v1672: f64 = (if self.scalar_v301 { v1667 } else { v1243 });
        let v1673: f64 = (if self.scalar_v301 { v1668 } else { v1244 });
        let v1674: f64 = (if self.scalar_v301 { v1669 } else { v1245 });
        let v1675: f64 = (if self.scalar_v376 { v879 } else { v1186 });
        let v1676: f64 = (if self.scalar_v376 { v880 } else { v1187 });
        let v1677: f64 = (if self.scalar_v376 { v881 } else { v1188 });
        let v1678: f64 = (if self.scalar_v376 { v882 } else { v1189 });
        let v1679: f64 = (if self.scalar_v376 { v883 } else { v1190 });
        let v1680: f64 = (if self.scalar_v376 { v1472 } else { v1477 });
        let v1681: f64 = (if self.scalar_v376 { v1473 } else { v1478 });
        let v1682: f64 = (if self.scalar_v376 { v1474 } else { v1479 });
        let v1683: f64 = (if self.scalar_v376 { v1475 } else { v1480 });
        let v1684: f64 = (if self.scalar_v376 { v1476 } else { v1481 });
        let v1685: f64 = (v1680 * v7);
        let v1686: f64 = (v1681 * v7);
        let v1687: f64 = (v1686 + v378);
        let v1688: f64 = (v1682 * v7);
        let v1689: f64 = (v378 * v401);
        let v1690: f64 = (v1688 + v1689);
        let v1691: f64 = (v1683 * v7);
        let v1692: f64 = (v1684 * v7);
        let v1693: f64 = (v380 * v380);
        let v1694: f64 = (v44 - v1693);
        let v1695: f64 = (v1694 * v1685);
        let v1696: f64 = (v1694 * v1687);
        let v1697: f64 = (v1694 * v1690);
        let v1698: f64 = (v1694 * v1691);
        let v1699: f64 = (v1694 * v1692);
        let v1700: f64 = (if self.scalar_v376 { v1695 } else { v1507 });
        let v1701: f64 = (if self.scalar_v376 { v1696 } else { v1508 });
        let v1702: f64 = (if self.scalar_v376 { v1697 } else { v1509 });
        let v1703: f64 = (if self.scalar_v376 { v1698 } else { v1510 });
        let v1704: f64 = (if self.scalar_v376 { v1699 } else { v1511 });
        let v1705: f64 = (v1680 * v11);
        let v1706: f64 = (v1705 + v378);
        let v1707: f64 = (v1681 * v11);
        let v1708: f64 = (v1682 * v11);
        let v1709: f64 = (v1708 + v1689);
        let v1710: f64 = (v1683 * v11);
        let v1711: f64 = (v1684 * v11);
        let v1712: f64 = (v383 * v383);
        let v1713: f64 = (v44 - v1712);
        let v1714: f64 = (v1713 * v1706);
        let v1715: f64 = (v1713 * v1707);
        let v1716: f64 = (v1713 * v1709);
        let v1717: f64 = (v1713 * v1710);
        let v1718: f64 = (v1713 * v1711);
        let v1719: f64 = (if self.scalar_v376 { v1714 } else { v13 });
        let v1720: f64 = (if self.scalar_v376 { v1715 } else { v13 });
        let v1721: f64 = (if self.scalar_v376 { v1716 } else { v13 });
        let v1722: f64 = (if self.scalar_v376 { v1717 } else { v13 });
        let v1723: f64 = (if self.scalar_v376 { v1718 } else { v13 });
        let v1724: f64 = (self.scalar_v385 * v1719);
        let v1725: f64 = (self.scalar_v385 * v1720);
        let v1726: f64 = (self.scalar_v385 * v1721);
        let v1727: f64 = (self.scalar_v385 * v1722);
        let v1728: f64 = (self.scalar_v385 * v1723);
        let v1729: f64 = (v1700 + v1724);
        let v1730: f64 = (v1701 + v1725);
        let v1731: f64 = (v1702 + v1726);
        let v1732: f64 = (v1703 + v1727);
        let v1733: f64 = (v1704 + v1728);
        let v1734: f64 = (v741 * v387);
        let v1735: f64 = (v197 * v1729);
        let v1736: f64 = (v1734 + v1735);
        let v1737: f64 = (v742 * v387);
        let v1738: f64 = (v197 * v1730);
        let v1739: f64 = (v1737 + v1738);
        let v1740: f64 = (v743 * v387);
        let v1741: f64 = (v197 * v1731);
        let v1742: f64 = (v1740 + v1741);
        let v1743: f64 = (v744 * v387);
        let v1744: f64 = (v197 * v1732);
        let v1745: f64 = (v1743 + v1744);
        let v1746: f64 = (v745 * v387);
        let v1747: f64 = (v197 * v1733);
        let v1748: f64 = (v1746 + v1747);
        let v1751: f64 = (v1675 * v390);
        let v1752: f64 = (v377 * self.scalar_v385);
        let v1753: f64 = (v1751 + v1752);
        let v1754: f64 = (v1676 * v390);
        let v1755: f64 = (v1754 + v377);
        let v1756: f64 = (v1677 * v390);
        let v1757: f64 = (v377 * self.scalar_v1750);
        let v1758: f64 = (v1756 + v1757);
        let v1759: f64 = (v1678 * v390);
        let v1760: f64 = (v1679 * v390);
        let v1761: f64 = (v1755 + v911);
        let v1762: f64 = (v1758 + v912);
        let v1763: f64 = (v1736 * v393);
        let v1764: f64 = (v388 * v1753);
        let v1765: f64 = (v1763 + v1764);
        let v1766: f64 = (v1739 * v393);
        let v1767: f64 = (v388 * v1761);
        let v1768: f64 = (v1766 + v1767);
        let v1769: f64 = (v1742 * v393);
        let v1770: f64 = (v388 * v1762);
        let v1771: f64 = (v1769 + v1770);
        let v1772: f64 = (v1745 * v393);
        let v1773: f64 = (v388 * v1759);
        let v1774: f64 = (v1772 + v1773);
        let v1775: f64 = (v1748 * v393);
        let v1776: f64 = (v388 * v1760);
        let v1777: f64 = (v1775 + v1776);
        let v1778: f64 = (if self.scalar_v376 { v1765 } else { v1670 });
        let v1779: f64 = (if self.scalar_v376 { v1768 } else { v1671 });
        let v1780: f64 = (if self.scalar_v376 { v1771 } else { v1672 });
        let v1781: f64 = (if self.scalar_v376 { v1774 } else { v1673 });
        let v1782: f64 = (if self.scalar_v376 { v1777 } else { v1674 });
        let v1783: f64 = (if self.scalar_v400 { v13 } else { v1246 });
        let v1784: f64 = (if self.scalar_v400 { v13 } else { v1247 });
        let v1785: f64 = (if self.scalar_v400 { v13 } else { v1248 });
        let v1786: f64 = (if self.scalar_v400 { v13 } else { v1249 });
        let v1790: f64 = (if self.scalar_v418 { v13 } else { v1783 });
        let v1791: f64 = (if self.scalar_v418 { v13 } else { v1784 });
        let v1792: f64 = (if self.scalar_v418 { v13 } else { v1785 });
        let v1793: f64 = (if self.scalar_v418 { v13 } else { v1786 });
        let v1795: f64 = (v433 * v433);
        let v1796: f64 = (v44 - v1795);
        let v1797: f64 = (v1796 * v401);
        let v1798: f64 = (if self.scalar_v432 { v1797 } else { self.scalar_v1788 });
        let v1799: f64 = (if self.scalar_v432 { v1796 } else { self.scalar_v1789 });
        let v1800: f64 = (v435 * v435);
        let v1801: f64 = (v44 - v1800);
        let v1802: f64 = (v1801 * v401);
        let v1803: f64 = (if self.scalar_v432 { v1802 } else { self.scalar_v1788 });
        let v1804: f64 = (if self.scalar_v432 { v1801 } else { self.scalar_v1789 });
        let v1805: f64 = (if self.scalar_v438 { v401 } else { v1798 });
        let v1806: f64 = (if self.scalar_v438 { v44 } else { v1799 });
        let v1807: f64 = (if self.scalar_v438 { v401 } else { v1803 });
        let v1808: f64 = (if self.scalar_v438 { v44 } else { v1804 });
        let v1813: f64 = { let limexp_arg = v443; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v1814: f64 = (v1813 * self.scalar_v1811);
        let v1815: f64 = (v1813 * self.scalar_v1812);
        let v1816: f64 = (v130 * v1805);
        let v1817: f64 = (v130 * v1806);
        let v1818: f64 = { let limexp_arg = v447; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v1819: f64 = (v1818 * v1816);
        let v1820: f64 = (v1818 * v1817);
        let v1821: f64 = (self.scalar_v451 * v1814);
        let v1822: f64 = (self.scalar_v451 * v1815);
        let v1823: f64 = (v1819 - v1821);
        let v1824: f64 = (v1820 - v1822);
        let v1825: f64 = (-v1790);
        let v1826: f64 = (-v1791);
        let v1827: f64 = (v1823 - v1792);
        let v1828: f64 = (-v1793);
        let v1830: f64 = (self.scalar_v446 * v1825);
        let v1831: f64 = (self.scalar_v446 * v1826);
        let v1832: f64 = (self.scalar_v446 * v1827);
        let v1833: f64 = (self.scalar_v446 * v1828);
        let v1834: f64 = (self.scalar_v446 * v1824);
        let v1836: f64 = { let limexp_arg = v456; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v1837: f64 = (v1836 * self.scalar_v1811);
        let v1838: f64 = (v1836 * self.scalar_v1812);
        let v1839: f64 = (v130 * v1807);
        let v1840: f64 = (v130 * v1808);
        let v1841: f64 = { let limexp_arg = v459; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v1842: f64 = (v1841 * v1839);
        let v1843: f64 = (v1841 * v1840);
        let v1844: f64 = (self.scalar_v451 * v1837);
        let v1845: f64 = (self.scalar_v451 * v1838);
        let v1846: f64 = (v1842 - v1844);
        let v1847: f64 = (v1843 - v1845);
        let v1848: f64 = (v1846 - v1791);
        let v1849: f64 = (-v1792);
        let v1850: f64 = (v1847 - v1793);
        let v1851: f64 = (self.scalar_v446 * v1848);
        let v1852: f64 = (self.scalar_v446 * v1849);
        let v1853: f64 = (self.scalar_v446 * v1850);
        let v1854: f64 = (-v1778);
        let v1855: f64 = (-v1779);
        let v1856: f64 = (-v1780);
        let v1857: f64 = (-v1781);
        let v1858: f64 = (-v1782);
        let v1861: f64 = (v114 * v401);
        let v1862: f64 = -1e-12;
        let v1863: f64 = (v44 / v115);
        let v1864: f64 = (v401 / v115);
        let v1865: f64 = (if self.scalar_v468 { v1863 } else { v13 });
        let v1866: f64 = (if self.scalar_v468 { v1864 } else { v13 });
        let v1881: f64 = (if self.scalar_v487 { v506 } else { v13 });
        let v1882: f64 = (if self.scalar_v487 { v502 } else { v13 });
        let v1883: f64 = (v44 / v49);
        let v1884: f64 = (if self.scalar_v507 { v1883 } else { v13 });

        let d508_dn4: f64 = v1854;
        let d508_dn5: f64 = v1855;
        let d508_dn8: f64 = v1856;
        let d508_dn10: f64 = v1857;
        let d508_dn12: f64 = v1858;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(15),
            None,
            multiplicity * (v508),
            [4, 5, 8, 10, 12],
            [d508_dn4, d508_dn5, d508_dn8, d508_dn10, d508_dn12],
            [],
            [],
            multiplicity,
        );
        let d12_dn16: f64 = v44;
        stamper.stamp_current_node1_local(
            Some(16),
            None,
            multiplicity * (v12),
            16,
            multiplicity * (d12_dn16),
        );
        let d12_dn16: f64 = v44;
        stamper.stamp_current_node1_local(
            Some(5),
            Some(8),
            multiplicity * (v12),
            16,
            multiplicity * (d12_dn16),
        );
        let d455_dn4: f64 = v1830;
        let d455_dn5: f64 = v1831;
        let d455_dn8: f64 = v1832;
        let d455_dn10: f64 = v1833;
        let d455_dn11: f64 = v1834;
        let d455_dn12: f64 = self.scalar_v1835;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(11),
            Some(8),
            multiplicity * (v455),
            [4, 5, 8, 10, 11, 12],
            [d455_dn4, d455_dn5, d455_dn8, d455_dn10, d455_dn11, d455_dn12],
            [],
            [],
            multiplicity,
        );
        let d464_dn4: f64 = v1830;
        let d464_dn5: f64 = v1851;
        let d464_dn8: f64 = v1852;
        let d464_dn10: f64 = v1853;
        let d464_dn12: f64 = self.scalar_v1835;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(10),
            Some(5),
            multiplicity * (v464),
            [4, 5, 8, 10, 12],
            [d464_dn4, d464_dn5, d464_dn8, d464_dn10, d464_dn12],
            [],
            [],
            multiplicity,
        );
        let d521_dn4: f64 = v1862;
        let d521_dn6: f64 = v138;
        stamper.stamp_current_node2_local(
            Some(6),
            Some(4),
            multiplicity * (v521),
            4,
            multiplicity * (d521_dn4),
            6,
            multiplicity * (d521_dn6),
        );
        stamper.stamp_potential_branch_local(
            Some(4),
            Some(8),
            2,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            2,
            self.scalar_v523,
        );
        let d526_dn11: f64 = v1865;
        let d526_dn12: f64 = v1866;
        stamper.stamp_current_node2_local(
            Some(11),
            Some(12),
            multiplicity * (v526),
            11,
            multiplicity * (d526_dn11),
            12,
            multiplicity * (d526_dn12),
        );
        stamper.stamp_potential_branch_local(
            Some(11),
            Some(8),
            3,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            3,
            self.scalar_v528,
        );
        let d535_dn8: f64 = self.scalar_v1870;
        let d535_dn14: f64 = self.scalar_v1871;
        stamper.stamp_current_node2_local(
            Some(14),
            Some(8),
            multiplicity * (v535),
            8,
            multiplicity * (d535_dn8),
            14,
            multiplicity * (d535_dn14),
        );
        stamper.stamp_potential_branch_local(
            Some(14),
            Some(8),
            4,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            4,
            self.scalar_v537,
        );
        let d541_dn10: f64 = self.scalar_v1874;
        let d541_dn13: f64 = self.scalar_v1875;
        stamper.stamp_current_node2_local(
            Some(13),
            Some(10),
            multiplicity * (v541),
            10,
            multiplicity * (d541_dn10),
            13,
            multiplicity * (d541_dn13),
        );
        stamper.stamp_potential_branch_local(
            Some(13),
            Some(10),
            5,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            5,
            self.scalar_v543,
        );
        let d546_dn11: f64 = self.scalar_v1878;
        let d546_dn13: f64 = self.scalar_v1879;
        stamper.stamp_current_node2_local(
            Some(13),
            Some(11),
            multiplicity * (v546),
            11,
            multiplicity * (d546_dn11),
            13,
            multiplicity * (d546_dn13),
        );
        stamper.stamp_potential_branch_local(
            Some(13),
            Some(11),
            6,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            6,
            self.scalar_v548,
        );
        stamper.stamp_potential_branch_local(
            Some(7),
            Some(13),
            9,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            9,
            self.scalar_v550,
        );
        stamper.stamp_potential_branch_local(
            Some(8),
            Some(9),
            13,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            13,
            self.scalar_v552,
        );
        stamper.stamp_potential_branch_local(
            Some(5),
            Some(6),
            17,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            17,
            self.scalar_v554,
        );
        stamper.stamp_current_const_local(
            Some(0),
            Some(2),
            multiplicity * (v555),
        );
        stamper.stamp_current_const_local(
            Some(14),
            Some(2),
            multiplicity * (v138),
        );
        let d558_dn2: f64 = v1862;
        let d558_dn12: f64 = v138;
        stamper.stamp_current_node2_local(
            Some(12),
            Some(2),
            multiplicity * (v558),
            2,
            multiplicity * (d558_dn2),
            12,
            multiplicity * (d558_dn12),
        );
        let d560_dn17: f64 = self.scalar_v1880;
        stamper.stamp_current_node1_local(
            Some(17),
            None,
            multiplicity * (v560),
            17,
            multiplicity * (d560_dn17),
        );
        let d562_dn18: f64 = self.scalar_v1880;
        stamper.stamp_current_node1_local(
            Some(18),
            None,
            multiplicity * (v562),
            18,
            multiplicity * (d562_dn18),
        );
        let d560_dn17: f64 = self.scalar_v1880;
        stamper.stamp_current_node1_local(
            Some(7),
            Some(8),
            multiplicity * (v560),
            17,
            multiplicity * (d560_dn17),
        );
        let d566_dn17: f64 = v1881;
        let d566_dn18: f64 = v1882;
        stamper.stamp_current_node2_local(
            Some(7),
            Some(5),
            multiplicity * (v566),
            17,
            multiplicity * (d566_dn17),
            18,
            multiplicity * (d566_dn18),
        );
        let d559_dn17: f64 = v44;
        stamper.stamp_current_node1_local(
            Some(17),
            None,
            multiplicity * (v559),
            17,
            multiplicity * (d559_dn17),
        );
        let d561_dn18: f64 = v44;
        stamper.stamp_current_node1_local(
            Some(18),
            None,
            multiplicity * (v561),
            18,
            multiplicity * (d561_dn18),
        );
        stamper.stamp_current_const_local(
            Some(3),
            None,
            multiplicity * (v573),
        );
        let d575_dn3: f64 = v1884;
        stamper.stamp_current_node1_local(
            Some(3),
            None,
            multiplicity * (v575),
            3,
            multiplicity * (d575_dn3),
        );
        let d578_dn3: f64 = self.scalar_v1885;
        stamper.stamp_current_node1_local(
            Some(3),
            None,
            multiplicity * (v578),
            3,
            multiplicity * (d578_dn3),
        );
        let d511_dn15: f64 = self.scalar_v509;
        let v511_ddt: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 1, v511);
        stamper.stamp_current_node1_local(
            Some(15),
            None,
            multiplicity * (v511_ddt),
            15,
            multiplicity * (((d511_dn15) * ddt_scale)),
        );
        let d515_dn5: f64 = self.scalar_v1859;
        let d515_dn7: f64 = self.scalar_v512;
        let v515_ddt: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 7, v515);
        stamper.stamp_current_node2_local(
            Some(7),
            Some(5),
            multiplicity * (v515_ddt),
            5,
            multiplicity * (((d515_dn5) * ddt_scale)),
            7,
            multiplicity * (((d515_dn7) * ddt_scale)),
        );
        let d517_dn5: f64 = self.scalar_v516;
        let d517_dn8: f64 = self.scalar_v1860;
        let v517_ddt: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 8, v517);
        stamper.stamp_current_node2_local(
            Some(5),
            Some(8),
            multiplicity * (v517_ddt),
            5,
            multiplicity * (((d517_dn5) * ddt_scale)),
            8,
            multiplicity * (((d517_dn8) * ddt_scale)),
        );
        let d520_dn4: f64 = v1861;
        let d520_dn6: f64 = v114;
        let v520_ddt: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 9, v520);
        stamper.stamp_current_node2_local(
            Some(6),
            Some(4),
            multiplicity * (v520_ddt),
            4,
            multiplicity * (((d520_dn4) * ddt_scale)),
            6,
            multiplicity * (((d520_dn6) * ddt_scale)),
        );
        let d532_dn11: f64 = self.scalar_v529;
        let d532_dn14: f64 = self.scalar_v1867;
        let v532_ddt: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 11, v532);
        stamper.stamp_current_node2_local(
            Some(11),
            Some(14),
            multiplicity * (v532_ddt),
            11,
            multiplicity * (((d532_dn11) * ddt_scale)),
            14,
            multiplicity * (((d532_dn14) * ddt_scale)),
        );
        let s = match &mut self.scratch {
            Some(buf) => buf.as_mut(),
            slot @ None => slot.insert(Scratch::new_box()).as_mut(),
        };

        Self::stamp_transient_block_0(ctx, s, p, nodes, branches, param_given, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_block_1(s, p);

        stamper.stamp_potential_branch_local(
            Some(15),
            Some(16),
            0,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(4),
            Some(8),
            1,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(7),
            Some(13),
            7,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(7),
            Some(13),
            8,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(1),
            Some(7),
            10,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(8),
            Some(9),
            11,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(8),
            Some(9),
            12,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(9),
            Some(2),
            14,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(5),
            Some(6),
            15,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(5),
            Some(6),
            16,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(6),
            Some(0),
            18,
            multiplicity,
        );

        Self::stamp_transient_equations_block_0(ctx, stamper, s, p, nodes, branches, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_1(ctx, stamper, s, p, nodes, branches, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
    }

    pub fn stamp_reactive(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedReactiveStamper<'_>) {
        let p = Box::as_ref(&self.params);
        let nodes = &(*self).nodes;
        let branches = &(*self).branches;
        let ctx_temp = ctx.temperature();
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv4 = ctx.node_voltage(nodes[4]);
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv11 = ctx.node_voltage(nodes[11]);
        let nv14 = ctx.node_voltage(nodes[14]);
        let nv15 = ctx.node_voltage(nodes[15]);
        let param_given = self.param_given.as_ref();
        let multiplicity = (*self).multiplicity;
        let v1: f64 = nv8;
        let v4: f64 = nv5;
        let v7: f64 = (v4 - v1);
        let v8: f64 = nv11;
        let v10: f64 = nv4;
        let v13: f64 = 0.0;
        let v20: f64 = ctx_temp;
        let v22: f64 = (v20 + self.scalar_v21);
        let v23: f64 = (if self.scalar_v19 { v22 } else { self.scalar_v18 });
        let v32: f64 = nv3;
        let v33: f64 = v32.abs();
        let v34: f64 = (v23 + v33);
        let v35: f64 = (if (self.scalar_v31 != 0.0) { v34 } else { v23 });
        let v38: f64 = (v35 - self.scalar_v30);
        let v39: f64 = v38.abs();
        let v40: bool = (v39 > v13);
        let v43: bool = (v40 || self.scalar_v42);
        let v44: f64 = 1.0;
        let v46: f64 = v39.abs();
        let v71: f64 = (self.scalar_v70 * v46);
        let v72: f64 = (v44 + v71);
        let v73: f64 = (self.scalar_v69 * v72);
        let v74: f64 = (if v43 { v73 } else { v13 });
        let v110: bool = (!v43);
        let v114: f64 = (if v110 { self.scalar_v69 } else { v74 });
        let v401: f64 = -1.0;
        let v510: f64 = nv15;
        let v511: f64 = (self.scalar_v509 * v510);
        let v513: f64 = nv7;
        let v514: f64 = (v513 - v4);
        let v515: f64 = (self.scalar_v512 * v514);
        let v517: f64 = (self.scalar_v516 * v7);
        let v518: f64 = nv6;
        let v519: f64 = (v518 - v10);
        let v520: f64 = (v114 * v519);
        let v530: f64 = nv14;
        let v531: f64 = (v8 - v530);
        let v532: f64 = (self.scalar_v529 * v531);
        let v1861: f64 = (v114 * v401);

        let d511_dn15: f64 = self.scalar_v509;
        stamper.stamp_current_reactive_node1(
            Some(nodes[15]),
            None,
            nodes[15],
            multiplicity * (d511_dn15),
        );
        let d515_dn5: f64 = self.scalar_v1859;
        let d515_dn7: f64 = self.scalar_v512;
        stamper.stamp_current_reactive_node2(
            Some(nodes[7]),
            Some(nodes[5]),
            nodes[5],
            multiplicity * (d515_dn5),
            nodes[7],
            multiplicity * (d515_dn7),
        );
        let d517_dn5: f64 = self.scalar_v516;
        let d517_dn8: f64 = self.scalar_v1860;
        stamper.stamp_current_reactive_node2(
            Some(nodes[5]),
            Some(nodes[8]),
            nodes[5],
            multiplicity * (d517_dn5),
            nodes[8],
            multiplicity * (d517_dn8),
        );
        let d520_dn4: f64 = v1861;
        let d520_dn6: f64 = v114;
        stamper.stamp_current_reactive_node2(
            Some(nodes[6]),
            Some(nodes[4]),
            nodes[4],
            multiplicity * (d520_dn4),
            nodes[6],
            multiplicity * (d520_dn6),
        );
        let d532_dn11: f64 = self.scalar_v529;
        let d532_dn14: f64 = self.scalar_v1867;
        stamper.stamp_current_reactive_node2(
            Some(nodes[11]),
            Some(nodes[14]),
            nodes[11],
            multiplicity * (d532_dn11),
            nodes[14],
            multiplicity * (d532_dn14),
        );
        let s = match &mut self.reactive_scratch {
            Some(buf) => buf.as_mut(),
            slot @ None => slot.insert(ReactiveScratch::new_box()).as_mut(),
        };

        Self::stamp_reactive_block_0(ctx, s, p, nodes, param_given);
        Self::stamp_reactive_block_1(ctx, s, p, branches);

        Self::stamp_reactive_equations_block_0(ctx, stamper, s, p, nodes, branches, multiplicity);
    }
}
