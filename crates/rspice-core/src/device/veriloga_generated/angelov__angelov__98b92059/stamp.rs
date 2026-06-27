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
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv4 = ctx.node_voltage(nodes[4]);
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let nv11 = ctx.node_voltage(nodes[11]);
        let nv13 = ctx.node_voltage(nodes[13]);
        let nv14 = ctx.node_voltage(nodes[14]);
        let nv15 = ctx.node_voltage(nodes[15]);
        let param_given = self.param_given.as_ref();
        let multiplicity = (*self).multiplicity;
        let timestep = (*self).timestep;
        let ddt_state_current = self.ddt_state_current.as_mut();
        let ddt_state_previous = self.ddt_state_previous.as_mut();
        let ddt_state_initialized = self.ddt_state_initialized.as_mut();
        let ddt_active = timestep.abs() > Instance::DDT_EPSILON;
        let ddt_scale = if ddt_active { 1.0 / timestep } else { 0.0 };
        let v0: f64 = nv8;
        let v1: f64 = nv5;
        let v2: f64 = (v0 - v1);
        let v3: f64 = nv4;
        let v4: f64 = nv3;
        let v5: f64 = (v3 - v4);
        let v6: f64 = (-v5);
        let v7: f64 = (v4 - v1);
        let v8: f64 = nv7;
        let v9: f64 = (v8 - v4);
        let v10: f64 = nv13;
        let v11: f64 = 0.0;
        let v18: f64 = ctx_temp;
        let v20: f64 = (v18 + self.scalar_v19);
        let v21: f64 = (if self.scalar_v17 { v20 } else { self.scalar_v16 });
        let v30: f64 = nv11;
        let v31: f64 = v30.abs();
        let v32: f64 = (v21 + v31);
        let v33: f64 = (if (self.scalar_v29 != 0.0) { v32 } else { v21 });
        let v34: f64 = 8.617333262145179e-5;
        let v35: f64 = (v33 * v34);
        let v36: f64 = (v33 - self.scalar_v28);
        let v37: f64 = v36.abs();
        let v38: bool = (v37 > v11);
        let v41: bool = (v38 || self.scalar_v40);
        let v42: f64 = 1.0;
        let v45: f64 = (self.scalar_v44 * v37);
        let v46: f64 = (v42 + v45);
        let v47: f64 = (self.scalar_v43 * v46);
        let v48: f64 = (if v41 { v47 } else { v11 });
        let v51: f64 = (self.scalar_v50 * v37);
        let v52: f64 = (v42 + v51);
        let v53: f64 = (self.scalar_v49 * v52);
        let v54: f64 = (if v41 { v53 } else { v11 });
        let v57: f64 = (self.scalar_v56 * v37);
        let v58: f64 = (v42 + v57);
        let v59: f64 = (self.scalar_v55 * v58);
        let v60: f64 = (if v41 { v59 } else { v11 });
        let v63: f64 = (self.scalar_v62 * v37);
        let v64: f64 = (v42 + v63);
        let v65: f64 = (self.scalar_v61 * v64);
        let v66: f64 = (if v41 { v65 } else { v11 });
        let v69: f64 = (self.scalar_v68 * v37);
        let v70: f64 = (v42 + v69);
        let v71: f64 = (self.scalar_v67 * v70);
        let v72: f64 = (if v41 { v71 } else { v11 });
        let v81: f64 = (self.scalar_v80 * v37);
        let v82: f64 = (self.scalar_v79 + v81);
        let v83: f64 = (if v41 { v82 } else { v11 });
        let v86: f64 = (self.scalar_v85 * v37);
        let v87: f64 = (self.scalar_v84 + v86);
        let v88: f64 = (if v41 { v87 } else { v11 });
        let v91: f64 = (self.scalar_v90 * v37);
        let v92: f64 = (self.scalar_v89 + v91);
        let v93: f64 = (if v41 { v92 } else { v11 });
        let v94: bool = (!v41);
        let v95: f64 = (if v94 { self.scalar_v43 } else { v48 });
        let v96: f64 = (if v94 { self.scalar_v49 } else { v54 });
        let v97: f64 = (if v94 { self.scalar_v55 } else { v60 });
        let v98: f64 = (if v94 { self.scalar_v61 } else { v66 });
        let v99: f64 = (if v94 { self.scalar_v67 } else { v72 });
        let v101: f64 = (if v94 { self.scalar_v79 } else { v83 });
        let v102: f64 = (if v94 { self.scalar_v84 } else { v88 });
        let v103: f64 = (if v94 { self.scalar_v89 } else { v93 });
        let v108: f64 = 0.5;
        let v111: f64 = (self.scalar_v110 / v35);
        let v112: f64 = (if self.scalar_v107 { v111 } else { v11 });
        let v115: f64 = (if self.scalar_v113 { self.scalar_v114 } else { v112 });
        let v117: f64 = (self.scalar_v116 * v7);
        let v118: f64 = v117.cosh();
        let v120: f64 = (v118 * v118);
        let v121: f64 = (self.scalar_v119 / v120);
        let v122: f64 = (v42 + v121);
        let v123: f64 = (v96 * v122);
        let v125: f64 = (v101 - self.scalar_v124);
        let v127: f64 = (self.scalar_v126 * v7);
        let v128: f64 = v127.tanh();
        let v129: f64 = (self.scalar_v124 * v128);
        let v130: f64 = (v125 + v129);
        let v132: f64 = (v6 - self.scalar_v89);
        let v133: f64 = (self.scalar_v131 * v132);
        let v134: f64 = (v6 - v103);
        let v135: f64 = (v133 * v134);
        let v136: f64 = (v130 - v135);
        let v137: f64 = (v2 - v136);
        let v138: f64 = (v137 * v137);
        let v139: f64 = (v123 * v137);
        let v141: f64 = (self.scalar_v140 * v138);
        let v142: f64 = (v139 + v141);
        let v144: f64 = (self.scalar_v143 * v137);
        let v145: f64 = (v144 * v138);
        let v146: f64 = (v142 + v145);
        let v147: f64 = v146.tanh();
        let v148: f64 = (v42 + v147);
        let v149: f64 = { let limexp_arg = v146; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let v150: f64 = (-v146);
        let v151: f64 = { let limexp_arg = v150; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let v152: f64 = (v149 - v151);
        let v153: f64 = (v108 * v152);
        let v154: f64 = v153.tanh();
        let v155: f64 = (v42 + v154);
        let v157: f64 = (self.scalar_v126 * v148);
        let v158: f64 = (self.scalar_v156 + v157);
        let v159: f64 = (v158 * v7);
        let v160: f64 = v159.tanh();
        let v168: f64 = (v95 * v148);
        let v169: f64 = (v168 * v160);
        let v171: f64 = (self.scalar_v170 * v7);
        let v172: f64 = (v42 + v171);
        let v173: f64 = { let limexp_arg = v134; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let v174: f64 = (v97 * v173);
        let v175: f64 = (v172 + v174);
        let v176: f64 = (v169 * v175);
        let v177: f64 = (if self.scalar_v162 { v176 } else { v11 });
        let v180: f64 = (v5 - v136);
        let v181: f64 = (if self.scalar_v179 { v180 } else { v118 });
        let v182: f64 = (v181 * v181);
        let v183: f64 = (if self.scalar_v179 { v182 } else { v137 });
        let v184: f64 = (v183 * v181);
        let v185: f64 = (if self.scalar_v179 { v184 } else { v138 });
        let v186: f64 = (v123 * v181);
        let v187: f64 = (self.scalar_v140 * v183);
        let v188: f64 = (v186 + v187);
        let v189: f64 = (self.scalar_v143 * v185);
        let v190: f64 = (v188 + v189);
        let v191: f64 = (if self.scalar_v179 { v190 } else { v11 });
        let v192: f64 = v191.tanh();
        let v193: f64 = (v42 + v192);
        let v194: f64 = (if self.scalar_v179 { v193 } else { v11 });
        let v195: f64 = (self.scalar_v126 * v194);
        let v196: f64 = (self.scalar_v156 + v195);
        let v197: f64 = (if self.scalar_v179 { v196 } else { v11 });
        let v199: f64 = (self.scalar_v198 * v148);
        let v200: f64 = (self.scalar_v170 + v199);
        let v201: f64 = (if self.scalar_v179 { v200 } else { v11 });
        let v202: f64 = (v42 + v160);
        let v203: f64 = (v168 * v202);
        let v204: f64 = (v201 * v7);
        let v205: f64 = (v42 + v204);
        let v206: f64 = (v7 - v103);
        let v207: f64 = { let limexp_arg = v206; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let v208: f64 = (v97 * v207);
        let v209: f64 = (v205 + v208);
        let v210: f64 = (v203 * v209);
        let v211: f64 = (if self.scalar_v179 { v210 } else { v11 });
        let v212: f64 = (self.scalar_v198 * v194);
        let v213: f64 = (self.scalar_v170 + v212);
        let v214: f64 = (if self.scalar_v179 { v213 } else { v11 });
        let v215: f64 = (v197 * v7);
        let v216: f64 = v215.tanh();
        let v217: f64 = (if self.scalar_v179 { v216 } else { v11 });
        let v218: f64 = (v95 * v194);
        let v219: f64 = (v42 - v217);
        let v220: f64 = (v218 * v219);
        let v221: f64 = (v214 * v7);
        let v222: f64 = (v42 - v221);
        let v223: f64 = (v220 * v222);
        let v224: f64 = (if self.scalar_v179 { v223 } else { v11 });
        let v225: f64 = (v211 - v224);
        let v226: f64 = (v108 * v225);
        let v227: f64 = (if self.scalar_v179 { v226 } else { v177 });
        let v231: f64 = (if self.scalar_v230 { v137 } else { v181 });
        let v232: f64 = (v231 * v231);
        let v233: f64 = (if self.scalar_v230 { v232 } else { v183 });
        let v234: f64 = (self.scalar_v140 * v233);
        let v235: f64 = (v231 + v234);
        let v236: f64 = (self.scalar_v143 * v233);
        let v237: f64 = (v236 * v231);
        let v238: f64 = (v235 + v237);
        let v239: f64 = (v123 * v238);
        let v240: f64 = (if self.scalar_v230 { v239 } else { v146 });
        let v241: f64 = { let limexp_arg = v240; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let v242: f64 = (-v240);
        let v243: f64 = { let limexp_arg = v242; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let v244: f64 = (v241 - v243);
        let v245: f64 = (v108 * v244);
        let v246: f64 = v245.tanh();
        let v247: f64 = (v42 + v246);
        let v248: f64 = (if self.scalar_v230 { v247 } else { v155 });
        let v249: f64 = (self.scalar_v126 * v248);
        let v250: f64 = (self.scalar_v156 + v249);
        let v251: f64 = (if self.scalar_v230 { v250 } else { v11 });
        let v252: f64 = (v251 * v7);
        let v253: f64 = v252.tanh();
        let v254: f64 = (if self.scalar_v230 { v253 } else { v11 });
        let v255: f64 = (self.scalar_v198 * v248);
        let v256: f64 = (self.scalar_v170 + v255);
        let v257: f64 = (if self.scalar_v230 { v256 } else { v201 });
        let v258: f64 = (v95 * v248);
        let v259: f64 = (v258 * v254);
        let v260: f64 = (v257 * v7);
        let v261: f64 = (v42 + v260);
        let v262: f64 = (v261 + v174);
        let v263: f64 = (v259 * v262);
        let v264: f64 = (if self.scalar_v230 { v263 } else { v227 });
        let v268: f64 = (if self.scalar_v267 { v137 } else { v231 });
        let v269: f64 = (v268 * v268);
        let v270: f64 = (if self.scalar_v267 { v269 } else { v233 });
        let v271: f64 = (self.scalar_v140 * v270);
        let v272: f64 = (v268 + v271);
        let v273: f64 = (self.scalar_v143 * v270);
        let v274: f64 = (v273 * v268);
        let v275: f64 = (v272 + v274);
        let v276: f64 = (v123 * v275);
        let v277: f64 = (if self.scalar_v267 { v276 } else { v240 });
        let v278: f64 = (if self.scalar_v267 { v180 } else { v185 });
        let v279: f64 = (v278 * v278);
        let v280: f64 = (if self.scalar_v267 { v279 } else { v11 });
        let v281: f64 = (self.scalar_v140 * v280);
        let v282: f64 = (v278 + v281);
        let v283: f64 = (self.scalar_v143 * v278);
        let v284: f64 = (v283 * v280);
        let v285: f64 = (v282 + v284);
        let v286: f64 = (v123 * v285);
        let v287: f64 = (if self.scalar_v267 { v286 } else { v191 });
        let v288: f64 = { let limexp_arg = v277; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let v289: f64 = (-v277);
        let v290: f64 = { let limexp_arg = v289; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let v291: f64 = (v288 - v290);
        let v292: f64 = (v108 * v291);
        let v293: f64 = v292.tanh();
        let v294: f64 = (v42 + v293);
        let v295: f64 = (if self.scalar_v267 { v294 } else { v248 });
        let v296: f64 = { let limexp_arg = v287; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let v297: f64 = (-v287);
        let v298: f64 = { let limexp_arg = v297; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let v299: f64 = (v296 - v298);
        let v300: f64 = (v108 * v299);
        let v301: f64 = v300.tanh();
        let v302: f64 = (v42 + v301);
        let v303: f64 = (if self.scalar_v267 { v302 } else { v11 });
        let v304: f64 = (self.scalar_v126 * v295);
        let v305: f64 = (self.scalar_v156 + v304);
        let v306: f64 = (if self.scalar_v267 { v305 } else { v251 });
        let v307: f64 = (self.scalar_v126 * v303);
        let v308: f64 = (self.scalar_v156 + v307);
        let v309: f64 = (if self.scalar_v267 { v308 } else { v11 });
        let v310: f64 = (v306 * v7);
        let v311: f64 = v310.tanh();
        let v312: f64 = (if self.scalar_v267 { v311 } else { v254 });
        let v313: f64 = (v309 * v7);
        let v314: f64 = v313.tanh();
        let v315: f64 = (if self.scalar_v267 { v314 } else { v11 });
        let v316: f64 = (self.scalar_v198 * v303);
        let v317: f64 = (self.scalar_v170 + v316);
        let v318: f64 = (if self.scalar_v267 { v317 } else { v11 });
        let v319: f64 = (self.scalar_v198 * v295);
        let v320: f64 = (self.scalar_v170 + v319);
        let v321: f64 = (if self.scalar_v267 { v320 } else { v11 });
        let v322: f64 = (v95 * v295);
        let v323: f64 = (v42 + v312);
        let v324: f64 = (v322 * v323);
        let v325: f64 = (v321 * v7);
        let v326: f64 = (v42 + v325);
        let v327: f64 = (v326 + v208);
        let v328: f64 = (v324 * v327);
        let v329: f64 = (if self.scalar_v267 { v328 } else { v211 });
        let v330: f64 = (v95 * v303);
        let v331: f64 = (v42 - v315);
        let v332: f64 = (v330 * v331);
        let v333: f64 = (v318 * v7);
        let v334: f64 = (v42 - v333);
        let v335: f64 = (v332 * v334);
        let v336: f64 = (if self.scalar_v267 { v335 } else { v224 });
        let v337: f64 = (v329 - v336);
        let v338: f64 = (v108 * v337);
        let v339: f64 = (if self.scalar_v267 { v338 } else { v264 });
        let v341: f64 = (v42 + v148);
        let v342: f64 = (v99 / v341);
        let v343: f64 = (self.scalar_v340 + v342);
        let v344: f64 = (if self.scalar_v228 { v343 } else { v11 });
        let v348: f64 = (v42 + v295);
        let v349: f64 = (v99 / v348);
        let v350: f64 = (self.scalar_v340 + v349);
        let v351: f64 = (if self.scalar_v229 { v350 } else { v344 });
        let v354: f64 = -1.0;
        let v355: f64 = (v354 * v102);
        let v356: f64 = v355.tanh();
        let v357: f64 = (v115 * v356);
        let v358: f64 = { let limexp_arg = v357; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let v359: f64 = (if self.scalar_v353 { v358 } else { v268 });
        let v360: f64 = (v2 - v102);
        let v361: f64 = (if self.scalar_v353 { v360 } else { v11 });
        let v362: f64 = (v9 - v102);
        let v363: f64 = (if self.scalar_v353 { v362 } else { v11 });
        let v365: f64 = (-v115);
        let v366: f64 = (v365 * v102);
        let v367: f64 = { let limexp_arg = v366; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let v368: f64 = (if self.scalar_v364 { v367 } else { v359 });
        let v371: f64 = v360.tanh();
        let v372: f64 = (if self.scalar_v370 { v371 } else { v361 });
        let v373: f64 = v362.tanh();
        let v374: f64 = (if self.scalar_v370 { v373 } else { v363 });
        let v377: f64 = (if self.scalar_v376 { v360 } else { v372 });
        let v378: f64 = (if self.scalar_v376 { v362 } else { v374 });
        let v380: f64 = (v115 * v377);
        let v381: f64 = { let limexp_arg = v380; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let v382: f64 = (v381 - v368);
        let v383: f64 = (self.scalar_v379 * v382);
        let v384: f64 = (v115 * v378);
        let v385: f64 = { let limexp_arg = v384; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let v386: f64 = (v385 - v368);
        let v387: f64 = (self.scalar_v379 * v386);
        let v412: f64 = 5.5226012e-23;
        let v413: f64 = (v412 * v33);
        let v415: f64 = (v413 * self.scalar_v414);
        let v416: f64 = (v415 * v98);
        let v421: f64 = (v416 * self.scalar_v420);
        let v422: f64 = (if self.scalar_v411 { v421 } else { v11 });
        let v423: f64 = (v422 * v422);
        let v424: f64 = (v42 - v423);
        let v425: f64 = v424.sqrt();
        let v426: f64 = (if self.scalar_v411 { v425 } else { v11 });
        let v427: f64 = (-v422);
        let v428: f64 = 3.141592653589793;
        let v429: f64 = (v427 * v428);
        let v430: f64 = (if self.scalar_v411 { v429 } else { v11 });
        let v432: f64 = (-v339);
        let v442: f64 = nv10;
        let v445: f64 = (v442 - v1);
        let v446: f64 = (v445 / v351);
        let v447: f64 = (if self.scalar_v388 { v446 } else { v11 });
        let v451: f64 = nv9;
        let v454: f64 = (v451 - v1);
        let v455: f64 = (v454 / self.scalar_v389);
        let v456: f64 = (if self.scalar_v390 { v455 } else { v11 });
        let v459: f64 = (v3 - v8);
        let v460: f64 = (v459 / self.scalar_v391);
        let v461: f64 = (if self.scalar_v392 { v460 } else { v11 });
        let v464: f64 = (v3 - v0);
        let v465: f64 = (v464 / self.scalar_v393);
        let v466: f64 = (if self.scalar_v394 { v465 } else { v11 });
        let v479: f64 = nv14;
        let v480: f64 = (if self.scalar_v411 { v479 } else { v11 });
        let v481: f64 = nv15;
        let v482: f64 = (if self.scalar_v411 { v481 } else { v11 });
        let v483: f64 = (v430 * v479);
        let v484: f64 = (v426 * v481);
        let v485: f64 = (v483 + v484);
        let v486: f64 = (if self.scalar_v411 { v485 } else { v11 });
        let v487: f64 = (-v10);
        let v488: f64 = (v487 * v7);
        let v489: f64 = (v383 * v2);
        let v490: f64 = (v488 + v489);
        let v491: f64 = v490.abs();
        let v492: f64 = (v354 * v491);
        let v493: f64 = (if self.scalar_v431 { v492 } else { v11 });
        let v494: f64 = (v30 / self.scalar_v39);
        let v495: f64 = (if self.scalar_v431 { v494 } else { v11 });
        let v497: f64 = 1e-12;
        let v498: f64 = (v30 * v497);
        let v499: f64 = (if self.scalar_v496 { v498 } else { v11 });
        let v501: f64 = v117.sinh();
        let v502: f64 = (v501 * self.scalar_v116);
        let v503: f64 = (v501 * self.scalar_v500);
        let v504: f64 = (v502 * v118);
        let v505: f64 = (v118 * v502);
        let v506: f64 = (v504 + v505);
        let v507: f64 = (v503 * v118);
        let v508: f64 = (v118 * v503);
        let v509: f64 = (v507 + v508);
        let v510: f64 = (self.scalar_v119 * v506);
        let v511: f64 = (-v510);
        let v512: f64 = (v120 * v120);
        let v513: f64 = (v511 / v512);
        let v514: f64 = (self.scalar_v119 * v509);
        let v515: f64 = (-v514);
        let v516: f64 = (v515 / v512);
        let v517: f64 = (v96 * v513);
        let v518: f64 = (v96 * v516);
        let v520: f64 = (v128 * v128);
        let v521: f64 = (v42 - v520);
        let v522: f64 = (v521 * self.scalar_v126);
        let v523: f64 = (v521 * self.scalar_v519);
        let v524: f64 = (self.scalar_v124 * v522);
        let v525: f64 = (self.scalar_v124 * v523);
        let v527: f64 = (self.scalar_v131 * v134);
        let v528: f64 = (v527 + v133);
        let v529: f64 = (self.scalar_v526 * v134);
        let v530: f64 = (v133 * v354);
        let v531: f64 = (v529 + v530);
        let v532: f64 = (v524 - v528);
        let v533: f64 = (-v531);
        let v534: f64 = (-v532);
        let v535: f64 = (-v533);
        let v536: f64 = (v354 - v525);
        let v537: f64 = (v534 * v137);
        let v538: f64 = (v137 * v534);
        let v539: f64 = (v537 + v538);
        let v540: f64 = (v535 * v137);
        let v541: f64 = (v137 * v535);
        let v542: f64 = (v540 + v541);
        let v543: f64 = (v536 * v137);
        let v544: f64 = (v137 * v536);
        let v545: f64 = (v543 + v544);
        let v546: f64 = (v137 + v137);
        let v547: f64 = (v517 * v137);
        let v548: f64 = (v123 * v534);
        let v549: f64 = (v547 + v548);
        let v550: f64 = (v123 * v535);
        let v551: f64 = (v518 * v137);
        let v552: f64 = (v123 * v536);
        let v553: f64 = (v551 + v552);
        let v554: f64 = (self.scalar_v140 * v539);
        let v555: f64 = (self.scalar_v140 * v542);
        let v556: f64 = (self.scalar_v140 * v545);
        let v557: f64 = (self.scalar_v140 * v546);
        let v558: f64 = (v549 + v554);
        let v559: f64 = (v550 + v555);
        let v560: f64 = (v553 + v556);
        let v561: f64 = (v123 + v557);
        let v562: f64 = (self.scalar_v143 * v534);
        let v563: f64 = (self.scalar_v143 * v535);
        let v564: f64 = (self.scalar_v143 * v536);
        let v565: f64 = (v562 * v138);
        let v566: f64 = (v144 * v539);
        let v567: f64 = (v565 + v566);
        let v568: f64 = (v563 * v138);
        let v569: f64 = (v144 * v542);
        let v570: f64 = (v568 + v569);
        let v571: f64 = (v564 * v138);
        let v572: f64 = (v144 * v545);
        let v573: f64 = (v571 + v572);
        let v574: f64 = (self.scalar_v143 * v138);
        let v575: f64 = (v144 * v546);
        let v576: f64 = (v574 + v575);
        let v577: f64 = (v558 + v567);
        let v578: f64 = (v559 + v570);
        let v579: f64 = (v560 + v573);
        let v580: f64 = (v561 + v576);
        let v581: f64 = (v147 * v147);
        let v582: f64 = (v42 - v581);
        let v583: f64 = (v582 * v577);
        let v584: f64 = (v582 * v578);
        let v585: f64 = (v582 * v579);
        let v586: f64 = (v582 * v580);
        let v587: f64 = { let limexp_arg = v146; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v588: f64 = (v587 * v577);
        let v589: f64 = (v587 * v578);
        let v590: f64 = (v587 * v579);
        let v591: f64 = (v587 * v580);
        let v592: f64 = (-v577);
        let v593: f64 = (-v578);
        let v594: f64 = (-v579);
        let v595: f64 = (-v580);
        let v596: f64 = { let limexp_arg = v150; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v597: f64 = (v596 * v592);
        let v598: f64 = (v596 * v593);
        let v599: f64 = (v596 * v594);
        let v600: f64 = (v596 * v595);
        let v601: f64 = (v588 - v597);
        let v602: f64 = (v589 - v598);
        let v603: f64 = (v590 - v599);
        let v604: f64 = (v591 - v600);
        let v605: f64 = (v108 * v601);
        let v606: f64 = (v108 * v602);
        let v607: f64 = (v108 * v603);
        let v608: f64 = (v108 * v604);
        let v609: f64 = (v154 * v154);
        let v610: f64 = (v42 - v609);
        let v611: f64 = (v610 * v605);
        let v612: f64 = (v610 * v606);
        let v613: f64 = (v610 * v607);
        let v614: f64 = (v610 * v608);
        let v615: f64 = (self.scalar_v126 * v583);
        let v616: f64 = (self.scalar_v126 * v584);
        let v617: f64 = (self.scalar_v126 * v585);
        let v618: f64 = (self.scalar_v126 * v586);
        let v619: f64 = (v615 * v7);
        let v620: f64 = (v619 + v158);
        let v621: f64 = (v616 * v7);
        let v622: f64 = (v617 * v7);
        let v623: f64 = (v158 * v354);
        let v624: f64 = (v622 + v623);
        let v625: f64 = (v618 * v7);
        let v626: f64 = (v160 * v160);
        let v627: f64 = (v42 - v626);
        let v628: f64 = (v627 * v620);
        let v629: f64 = (v627 * v621);
        let v630: f64 = (v627 * v624);
        let v631: f64 = (v627 * v625);
        let v632: f64 = (v95 * v583);
        let v633: f64 = (v95 * v584);
        let v634: f64 = (v95 * v585);
        let v635: f64 = (v95 * v586);
        let v636: f64 = (v632 * v160);
        let v637: f64 = (v168 * v628);
        let v638: f64 = (v636 + v637);
        let v639: f64 = (v633 * v160);
        let v640: f64 = (v168 * v629);
        let v641: f64 = (v639 + v640);
        let v642: f64 = (v634 * v160);
        let v643: f64 = (v168 * v630);
        let v644: f64 = (v642 + v643);
        let v645: f64 = (v635 * v160);
        let v646: f64 = (v168 * v631);
        let v647: f64 = (v645 + v646);
        let v649: f64 = { let limexp_arg = v134; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v650: f64 = (v649 * v354);
        let v651: f64 = (v97 * v649);
        let v652: f64 = (v97 * v650);
        let v653: f64 = (self.scalar_v170 + v651);
        let v654: f64 = (v638 * v175);
        let v655: f64 = (v169 * v653);
        let v656: f64 = (v654 + v655);
        let v657: f64 = (v641 * v175);
        let v658: f64 = (v169 * v652);
        let v659: f64 = (v657 + v658);
        let v660: f64 = (v644 * v175);
        let v661: f64 = (v169 * self.scalar_v648);
        let v662: f64 = (v660 + v661);
        let v663: f64 = (v647 * v175);
        let v664: f64 = (if self.scalar_v162 { v656 } else { v11 });
        let v665: f64 = (if self.scalar_v162 { v659 } else { v11 });
        let v666: f64 = (if self.scalar_v162 { v662 } else { v11 });
        let v667: f64 = (if self.scalar_v162 { v663 } else { v11 });
        let v668: f64 = (v354 - v532);
        let v669: f64 = (v42 - v533);
        let v670: f64 = (-v525);
        let v671: f64 = (if self.scalar_v179 { v668 } else { v502 });
        let v672: f64 = (if self.scalar_v179 { v669 } else { v11 });
        let v673: f64 = (if self.scalar_v179 { v670 } else { v503 });
        let v674: f64 = (v671 * v181);
        let v675: f64 = (v181 * v671);
        let v676: f64 = (v674 + v675);
        let v677: f64 = (v672 * v181);
        let v678: f64 = (v181 * v672);
        let v679: f64 = (v677 + v678);
        let v680: f64 = (v673 * v181);
        let v681: f64 = (v181 * v673);
        let v682: f64 = (v680 + v681);
        let v683: f64 = (if self.scalar_v179 { v676 } else { v534 });
        let v684: f64 = (if self.scalar_v179 { v679 } else { v535 });
        let v685: f64 = (if self.scalar_v179 { v682 } else { v536 });
        let v687: f64 = (v683 * v181);
        let v688: f64 = (v183 * v671);
        let v689: f64 = (v687 + v688);
        let v690: f64 = (v684 * v181);
        let v691: f64 = (v183 * v672);
        let v692: f64 = (v690 + v691);
        let v693: f64 = (v685 * v181);
        let v694: f64 = (v183 * v673);
        let v695: f64 = (v693 + v694);
        let v696: f64 = (self.scalar_v686 * v181);
        let v697: f64 = (if self.scalar_v179 { v689 } else { v539 });
        let v698: f64 = (if self.scalar_v179 { v692 } else { v542 });
        let v699: f64 = (if self.scalar_v179 { v695 } else { v545 });
        let v700: f64 = (if self.scalar_v179 { v696 } else { v546 });
        let v701: f64 = (v517 * v181);
        let v702: f64 = (v123 * v671);
        let v703: f64 = (v701 + v702);
        let v704: f64 = (v123 * v672);
        let v705: f64 = (v518 * v181);
        let v706: f64 = (v123 * v673);
        let v707: f64 = (v705 + v706);
        let v708: f64 = (self.scalar_v140 * v683);
        let v709: f64 = (self.scalar_v140 * v684);
        let v710: f64 = (self.scalar_v140 * v685);
        let v712: f64 = (v703 + v708);
        let v713: f64 = (v704 + v709);
        let v714: f64 = (v707 + v710);
        let v715: f64 = (self.scalar_v143 * v697);
        let v716: f64 = (self.scalar_v143 * v698);
        let v717: f64 = (self.scalar_v143 * v699);
        let v718: f64 = (self.scalar_v143 * v700);
        let v719: f64 = (v712 + v715);
        let v720: f64 = (v713 + v716);
        let v721: f64 = (v714 + v717);
        let v722: f64 = (self.scalar_v711 + v718);
        let v723: f64 = (if self.scalar_v179 { v719 } else { v11 });
        let v724: f64 = (if self.scalar_v179 { v720 } else { v11 });
        let v725: f64 = (if self.scalar_v179 { v721 } else { v11 });
        let v726: f64 = (if self.scalar_v179 { v722 } else { v11 });
        let v727: f64 = (v192 * v192);
        let v728: f64 = (v42 - v727);
        let v729: f64 = (v728 * v723);
        let v730: f64 = (v728 * v724);
        let v731: f64 = (v728 * v725);
        let v732: f64 = (v728 * v726);
        let v733: f64 = (if self.scalar_v179 { v729 } else { v11 });
        let v734: f64 = (if self.scalar_v179 { v730 } else { v11 });
        let v735: f64 = (if self.scalar_v179 { v731 } else { v11 });
        let v736: f64 = (if self.scalar_v179 { v732 } else { v11 });
        let v737: f64 = (self.scalar_v126 * v733);
        let v738: f64 = (self.scalar_v126 * v734);
        let v739: f64 = (self.scalar_v126 * v735);
        let v740: f64 = (self.scalar_v126 * v736);
        let v741: f64 = (if self.scalar_v179 { v737 } else { v11 });
        let v742: f64 = (if self.scalar_v179 { v738 } else { v11 });
        let v743: f64 = (if self.scalar_v179 { v739 } else { v11 });
        let v744: f64 = (if self.scalar_v179 { v740 } else { v11 });
        let v745: f64 = (self.scalar_v198 * v583);
        let v746: f64 = (self.scalar_v198 * v584);
        let v747: f64 = (self.scalar_v198 * v585);
        let v748: f64 = (self.scalar_v198 * v586);
        let v749: f64 = (if self.scalar_v179 { v745 } else { v11 });
        let v750: f64 = (if self.scalar_v179 { v746 } else { v11 });
        let v751: f64 = (if self.scalar_v179 { v747 } else { v11 });
        let v752: f64 = (if self.scalar_v179 { v748 } else { v11 });
        let v753: f64 = (v632 * v202);
        let v754: f64 = (v753 + v637);
        let v755: f64 = (v633 * v202);
        let v756: f64 = (v755 + v640);
        let v757: f64 = (v634 * v202);
        let v758: f64 = (v757 + v643);
        let v759: f64 = (v635 * v202);
        let v760: f64 = (v759 + v646);
        let v761: f64 = (v749 * v7);
        let v762: f64 = (v761 + v201);
        let v763: f64 = (v750 * v7);
        let v764: f64 = (v751 * v7);
        let v765: f64 = (v201 * v354);
        let v766: f64 = (v764 + v765);
        let v767: f64 = (v752 * v7);
        let v768: f64 = { let limexp_arg = v206; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v769: f64 = (v768 * v354);
        let v770: f64 = (v97 * v768);
        let v771: f64 = (v97 * v769);
        let v772: f64 = (v762 + v770);
        let v773: f64 = (v766 + v771);
        let v774: f64 = (v754 * v209);
        let v775: f64 = (v203 * v772);
        let v776: f64 = (v774 + v775);
        let v777: f64 = (v756 * v209);
        let v778: f64 = (v203 * v763);
        let v779: f64 = (v777 + v778);
        let v780: f64 = (v758 * v209);
        let v781: f64 = (v203 * v773);
        let v782: f64 = (v780 + v781);
        let v783: f64 = (v760 * v209);
        let v784: f64 = (v203 * v767);
        let v785: f64 = (v783 + v784);
        let v786: f64 = (if self.scalar_v179 { v776 } else { v11 });
        let v787: f64 = (if self.scalar_v179 { v779 } else { v11 });
        let v788: f64 = (if self.scalar_v179 { v782 } else { v11 });
        let v789: f64 = (if self.scalar_v179 { v785 } else { v11 });
        let v790: f64 = (self.scalar_v198 * v733);
        let v791: f64 = (self.scalar_v198 * v734);
        let v792: f64 = (self.scalar_v198 * v735);
        let v793: f64 = (self.scalar_v198 * v736);
        let v794: f64 = (if self.scalar_v179 { v790 } else { v11 });
        let v795: f64 = (if self.scalar_v179 { v791 } else { v11 });
        let v796: f64 = (if self.scalar_v179 { v792 } else { v11 });
        let v797: f64 = (if self.scalar_v179 { v793 } else { v11 });
        let v798: f64 = (v741 * v7);
        let v799: f64 = (v798 + v197);
        let v800: f64 = (v742 * v7);
        let v801: f64 = (v743 * v7);
        let v802: f64 = (v197 * v354);
        let v803: f64 = (v801 + v802);
        let v804: f64 = (v744 * v7);
        let v805: f64 = (v216 * v216);
        let v806: f64 = (v42 - v805);
        let v807: f64 = (v806 * v799);
        let v808: f64 = (v806 * v800);
        let v809: f64 = (v806 * v803);
        let v810: f64 = (v806 * v804);
        let v811: f64 = (if self.scalar_v179 { v807 } else { v11 });
        let v812: f64 = (if self.scalar_v179 { v808 } else { v11 });
        let v813: f64 = (if self.scalar_v179 { v809 } else { v11 });
        let v814: f64 = (if self.scalar_v179 { v810 } else { v11 });
        let v815: f64 = (v95 * v733);
        let v816: f64 = (v95 * v734);
        let v817: f64 = (v95 * v735);
        let v818: f64 = (v95 * v736);
        let v819: f64 = (-v811);
        let v820: f64 = (-v812);
        let v821: f64 = (-v813);
        let v822: f64 = (-v814);
        let v823: f64 = (v815 * v219);
        let v824: f64 = (v218 * v819);
        let v825: f64 = (v823 + v824);
        let v826: f64 = (v816 * v219);
        let v827: f64 = (v218 * v820);
        let v828: f64 = (v826 + v827);
        let v829: f64 = (v817 * v219);
        let v830: f64 = (v218 * v821);
        let v831: f64 = (v829 + v830);
        let v832: f64 = (v818 * v219);
        let v833: f64 = (v218 * v822);
        let v834: f64 = (v832 + v833);
        let v835: f64 = (v794 * v7);
        let v836: f64 = (v835 + v214);
        let v837: f64 = (v795 * v7);
        let v838: f64 = (v796 * v7);
        let v839: f64 = (v214 * v354);
        let v840: f64 = (v838 + v839);
        let v841: f64 = (v797 * v7);
        let v842: f64 = (-v836);
        let v843: f64 = (-v837);
        let v844: f64 = (-v840);
        let v845: f64 = (-v841);
        let v846: f64 = (v825 * v222);
        let v847: f64 = (v220 * v842);
        let v848: f64 = (v846 + v847);
        let v849: f64 = (v828 * v222);
        let v850: f64 = (v220 * v843);
        let v851: f64 = (v849 + v850);
        let v852: f64 = (v831 * v222);
        let v853: f64 = (v220 * v844);
        let v854: f64 = (v852 + v853);
        let v855: f64 = (v834 * v222);
        let v856: f64 = (v220 * v845);
        let v857: f64 = (v855 + v856);
        let v858: f64 = (if self.scalar_v179 { v848 } else { v11 });
        let v859: f64 = (if self.scalar_v179 { v851 } else { v11 });
        let v860: f64 = (if self.scalar_v179 { v854 } else { v11 });
        let v861: f64 = (if self.scalar_v179 { v857 } else { v11 });
        let v862: f64 = (v786 - v858);
        let v863: f64 = (v787 - v859);
        let v864: f64 = (v788 - v860);
        let v865: f64 = (v789 - v861);
        let v866: f64 = (v108 * v862);
        let v867: f64 = (v108 * v863);
        let v868: f64 = (v108 * v864);
        let v869: f64 = (v108 * v865);
        let v870: f64 = (if self.scalar_v179 { v866 } else { v664 });
        let v871: f64 = (if self.scalar_v179 { v867 } else { v665 });
        let v872: f64 = (if self.scalar_v179 { v868 } else { v666 });
        let v873: f64 = (if self.scalar_v179 { v869 } else { v667 });
        let v874: f64 = (if self.scalar_v230 { v534 } else { v671 });
        let v875: f64 = (if self.scalar_v230 { v535 } else { v672 });
        let v876: f64 = (if self.scalar_v230 { v536 } else { v673 });
        let v878: f64 = (v874 * v231);
        let v879: f64 = (v231 * v874);
        let v880: f64 = (v878 + v879);
        let v881: f64 = (v875 * v231);
        let v882: f64 = (v231 * v875);
        let v883: f64 = (v881 + v882);
        let v884: f64 = (v876 * v231);
        let v885: f64 = (v231 * v876);
        let v886: f64 = (v884 + v885);
        let v887: f64 = (self.scalar_v877 * v231);
        let v888: f64 = (v231 * self.scalar_v877);
        let v889: f64 = (v887 + v888);
        let v890: f64 = (if self.scalar_v230 { v880 } else { v683 });
        let v891: f64 = (if self.scalar_v230 { v883 } else { v684 });
        let v892: f64 = (if self.scalar_v230 { v886 } else { v685 });
        let v893: f64 = (if self.scalar_v230 { v889 } else { self.scalar_v686 });
        let v894: f64 = (self.scalar_v140 * v890);
        let v895: f64 = (self.scalar_v140 * v891);
        let v896: f64 = (self.scalar_v140 * v892);
        let v897: f64 = (self.scalar_v140 * v893);
        let v898: f64 = (v874 + v894);
        let v899: f64 = (v875 + v895);
        let v900: f64 = (v876 + v896);
        let v901: f64 = (self.scalar_v877 + v897);
        let v902: f64 = (self.scalar_v143 * v890);
        let v903: f64 = (self.scalar_v143 * v891);
        let v904: f64 = (self.scalar_v143 * v892);
        let v905: f64 = (self.scalar_v143 * v893);
        let v906: f64 = (v902 * v231);
        let v907: f64 = (v236 * v874);
        let v908: f64 = (v906 + v907);
        let v909: f64 = (v903 * v231);
        let v910: f64 = (v236 * v875);
        let v911: f64 = (v909 + v910);
        let v912: f64 = (v904 * v231);
        let v913: f64 = (v236 * v876);
        let v914: f64 = (v912 + v913);
        let v915: f64 = (v905 * v231);
        let v916: f64 = (v236 * self.scalar_v877);
        let v917: f64 = (v915 + v916);
        let v918: f64 = (v898 + v908);
        let v919: f64 = (v899 + v911);
        let v920: f64 = (v900 + v914);
        let v921: f64 = (v901 + v917);
        let v922: f64 = (v517 * v238);
        let v923: f64 = (v123 * v918);
        let v924: f64 = (v922 + v923);
        let v925: f64 = (v123 * v919);
        let v926: f64 = (v518 * v238);
        let v927: f64 = (v123 * v920);
        let v928: f64 = (v926 + v927);
        let v929: f64 = (v123 * v921);
        let v930: f64 = (if self.scalar_v230 { v924 } else { v577 });
        let v931: f64 = (if self.scalar_v230 { v925 } else { v578 });
        let v932: f64 = (if self.scalar_v230 { v928 } else { v579 });
        let v933: f64 = (if self.scalar_v230 { v929 } else { v580 });
        let v934: f64 = { let limexp_arg = v240; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v935: f64 = (v934 * v930);
        let v936: f64 = (v934 * v931);
        let v937: f64 = (v934 * v932);
        let v938: f64 = (v934 * v933);
        let v939: f64 = (-v930);
        let v940: f64 = (-v931);
        let v941: f64 = (-v932);
        let v942: f64 = (-v933);
        let v943: f64 = { let limexp_arg = v242; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v944: f64 = (v943 * v939);
        let v945: f64 = (v943 * v940);
        let v946: f64 = (v943 * v941);
        let v947: f64 = (v943 * v942);
        let v948: f64 = (v935 - v944);
        let v949: f64 = (v936 - v945);
        let v950: f64 = (v937 - v946);
        let v951: f64 = (v938 - v947);
        let v952: f64 = (v108 * v948);
        let v953: f64 = (v108 * v949);
        let v954: f64 = (v108 * v950);
        let v955: f64 = (v108 * v951);
        let v956: f64 = (v246 * v246);
        let v957: f64 = (v42 - v956);
        let v958: f64 = (v957 * v952);
        let v959: f64 = (v957 * v953);
        let v960: f64 = (v957 * v954);
        let v961: f64 = (v957 * v955);
        let v962: f64 = (if self.scalar_v230 { v958 } else { v611 });
        let v963: f64 = (if self.scalar_v230 { v959 } else { v612 });
        let v964: f64 = (if self.scalar_v230 { v960 } else { v613 });
        let v965: f64 = (if self.scalar_v230 { v961 } else { v614 });
        let v966: f64 = (self.scalar_v126 * v962);
        let v967: f64 = (self.scalar_v126 * v963);
        let v968: f64 = (self.scalar_v126 * v964);
        let v969: f64 = (self.scalar_v126 * v965);
        let v970: f64 = (if self.scalar_v230 { v966 } else { v11 });
        let v971: f64 = (if self.scalar_v230 { v967 } else { v11 });
        let v972: f64 = (if self.scalar_v230 { v968 } else { v11 });
        let v973: f64 = (if self.scalar_v230 { v969 } else { v11 });
        let v974: f64 = (v970 * v7);
        let v975: f64 = (v974 + v251);
        let v976: f64 = (v971 * v7);
        let v977: f64 = (v972 * v7);
        let v978: f64 = (v251 * v354);
        let v979: f64 = (v977 + v978);
        let v980: f64 = (v973 * v7);
        let v981: f64 = (v253 * v253);
        let v982: f64 = (v42 - v981);
        let v983: f64 = (v982 * v975);
        let v984: f64 = (v982 * v976);
        let v985: f64 = (v982 * v979);
        let v986: f64 = (v982 * v980);
        let v987: f64 = (if self.scalar_v230 { v983 } else { v11 });
        let v988: f64 = (if self.scalar_v230 { v984 } else { v11 });
        let v989: f64 = (if self.scalar_v230 { v985 } else { v11 });
        let v990: f64 = (if self.scalar_v230 { v986 } else { v11 });
        let v991: f64 = (self.scalar_v198 * v962);
        let v992: f64 = (self.scalar_v198 * v963);
        let v993: f64 = (self.scalar_v198 * v964);
        let v994: f64 = (self.scalar_v198 * v965);
        let v995: f64 = (if self.scalar_v230 { v991 } else { v749 });
        let v996: f64 = (if self.scalar_v230 { v992 } else { v750 });
        let v997: f64 = (if self.scalar_v230 { v993 } else { v751 });
        let v998: f64 = (if self.scalar_v230 { v994 } else { v752 });
        let v999: f64 = (v95 * v962);
        let v1000: f64 = (v95 * v963);
        let v1001: f64 = (v95 * v964);
        let v1002: f64 = (v95 * v965);
        let v1003: f64 = (v999 * v254);
        let v1004: f64 = (v258 * v987);
        let v1005: f64 = (v1003 + v1004);
        let v1006: f64 = (v1000 * v254);
        let v1007: f64 = (v258 * v988);
        let v1008: f64 = (v1006 + v1007);
        let v1009: f64 = (v1001 * v254);
        let v1010: f64 = (v258 * v989);
        let v1011: f64 = (v1009 + v1010);
        let v1012: f64 = (v1002 * v254);
        let v1013: f64 = (v258 * v990);
        let v1014: f64 = (v1012 + v1013);
        let v1015: f64 = (v995 * v7);
        let v1016: f64 = (v1015 + v257);
        let v1017: f64 = (v996 * v7);
        let v1018: f64 = (v997 * v7);
        let v1019: f64 = (v257 * v354);
        let v1020: f64 = (v1018 + v1019);
        let v1021: f64 = (v998 * v7);
        let v1022: f64 = (v1016 + v651);
        let v1023: f64 = (v1017 + v652);
        let v1024: f64 = (v1005 * v262);
        let v1025: f64 = (v259 * v1022);
        let v1026: f64 = (v1024 + v1025);
        let v1027: f64 = (v1008 * v262);
        let v1028: f64 = (v259 * v1023);
        let v1029: f64 = (v1027 + v1028);
        let v1030: f64 = (v1011 * v262);
        let v1031: f64 = (v259 * v1020);
        let v1032: f64 = (v1030 + v1031);
        let v1033: f64 = (v1014 * v262);
        let v1034: f64 = (v259 * v1021);
        let v1035: f64 = (v1033 + v1034);
        let v1036: f64 = (if self.scalar_v230 { v1026 } else { v870 });
        let v1037: f64 = (if self.scalar_v230 { v1029 } else { v871 });
        let v1038: f64 = (if self.scalar_v230 { v1032 } else { v872 });
        let v1039: f64 = (if self.scalar_v230 { v1035 } else { v873 });
        let v1040: f64 = (if self.scalar_v267 { v534 } else { v874 });
        let v1041: f64 = (if self.scalar_v267 { v535 } else { v875 });
        let v1042: f64 = (if self.scalar_v267 { v536 } else { v876 });
        let v1044: f64 = (v1040 * v268);
        let v1045: f64 = (v268 * v1040);
        let v1046: f64 = (v1044 + v1045);
        let v1047: f64 = (v1041 * v268);
        let v1048: f64 = (v268 * v1041);
        let v1049: f64 = (v1047 + v1048);
        let v1050: f64 = (v1042 * v268);
        let v1051: f64 = (v268 * v1042);
        let v1052: f64 = (v1050 + v1051);
        let v1053: f64 = (self.scalar_v1043 * v268);
        let v1054: f64 = (v268 * self.scalar_v1043);
        let v1055: f64 = (v1053 + v1054);
        let v1056: f64 = (if self.scalar_v267 { v1046 } else { v890 });
        let v1057: f64 = (if self.scalar_v267 { v1049 } else { v891 });
        let v1058: f64 = (if self.scalar_v267 { v1052 } else { v892 });
        let v1059: f64 = (if self.scalar_v267 { v1055 } else { v893 });
        let v1060: f64 = (self.scalar_v140 * v1056);
        let v1061: f64 = (self.scalar_v140 * v1057);
        let v1062: f64 = (self.scalar_v140 * v1058);
        let v1063: f64 = (self.scalar_v140 * v1059);
        let v1064: f64 = (v1040 + v1060);
        let v1065: f64 = (v1041 + v1061);
        let v1066: f64 = (v1042 + v1062);
        let v1067: f64 = (self.scalar_v1043 + v1063);
        let v1068: f64 = (self.scalar_v143 * v1056);
        let v1069: f64 = (self.scalar_v143 * v1057);
        let v1070: f64 = (self.scalar_v143 * v1058);
        let v1071: f64 = (self.scalar_v143 * v1059);
        let v1072: f64 = (v1068 * v268);
        let v1073: f64 = (v273 * v1040);
        let v1074: f64 = (v1072 + v1073);
        let v1075: f64 = (v1069 * v268);
        let v1076: f64 = (v273 * v1041);
        let v1077: f64 = (v1075 + v1076);
        let v1078: f64 = (v1070 * v268);
        let v1079: f64 = (v273 * v1042);
        let v1080: f64 = (v1078 + v1079);
        let v1081: f64 = (v1071 * v268);
        let v1082: f64 = (v273 * self.scalar_v1043);
        let v1083: f64 = (v1081 + v1082);
        let v1084: f64 = (v1064 + v1074);
        let v1085: f64 = (v1065 + v1077);
        let v1086: f64 = (v1066 + v1080);
        let v1087: f64 = (v1067 + v1083);
        let v1088: f64 = (v517 * v275);
        let v1089: f64 = (v123 * v1084);
        let v1090: f64 = (v1088 + v1089);
        let v1091: f64 = (v123 * v1085);
        let v1092: f64 = (v518 * v275);
        let v1093: f64 = (v123 * v1086);
        let v1094: f64 = (v1092 + v1093);
        let v1095: f64 = (v123 * v1087);
        let v1096: f64 = (if self.scalar_v267 { v1090 } else { v930 });
        let v1097: f64 = (if self.scalar_v267 { v1091 } else { v931 });
        let v1098: f64 = (if self.scalar_v267 { v1094 } else { v932 });
        let v1099: f64 = (if self.scalar_v267 { v1095 } else { v933 });
        let v1100: f64 = (if self.scalar_v267 { v668 } else { v697 });
        let v1101: f64 = (if self.scalar_v267 { v669 } else { v698 });
        let v1102: f64 = (if self.scalar_v267 { v670 } else { v699 });
        let v1103: f64 = (if self.scalar_v267 { v11 } else { v700 });
        let v1104: f64 = (v1100 * v278);
        let v1105: f64 = (v278 * v1100);
        let v1106: f64 = (v1104 + v1105);
        let v1107: f64 = (v1101 * v278);
        let v1108: f64 = (v278 * v1101);
        let v1109: f64 = (v1107 + v1108);
        let v1110: f64 = (v1102 * v278);
        let v1111: f64 = (v278 * v1102);
        let v1112: f64 = (v1110 + v1111);
        let v1113: f64 = (v1103 * v278);
        let v1114: f64 = (v278 * v1103);
        let v1115: f64 = (v1113 + v1114);
        let v1116: f64 = (if self.scalar_v267 { v1106 } else { v11 });
        let v1117: f64 = (if self.scalar_v267 { v1109 } else { v11 });
        let v1118: f64 = (if self.scalar_v267 { v1112 } else { v11 });
        let v1119: f64 = (if self.scalar_v267 { v1115 } else { v11 });
        let v1120: f64 = (self.scalar_v140 * v1116);
        let v1121: f64 = (self.scalar_v140 * v1117);
        let v1122: f64 = (self.scalar_v140 * v1118);
        let v1123: f64 = (self.scalar_v140 * v1119);
        let v1124: f64 = (v1100 + v1120);
        let v1125: f64 = (v1101 + v1121);
        let v1126: f64 = (v1102 + v1122);
        let v1127: f64 = (v1103 + v1123);
        let v1128: f64 = (self.scalar_v143 * v1100);
        let v1129: f64 = (self.scalar_v143 * v1101);
        let v1130: f64 = (self.scalar_v143 * v1102);
        let v1131: f64 = (self.scalar_v143 * v1103);
        let v1132: f64 = (v1128 * v280);
        let v1133: f64 = (v283 * v1116);
        let v1134: f64 = (v1132 + v1133);
        let v1135: f64 = (v1129 * v280);
        let v1136: f64 = (v283 * v1117);
        let v1137: f64 = (v1135 + v1136);
        let v1138: f64 = (v1130 * v280);
        let v1139: f64 = (v283 * v1118);
        let v1140: f64 = (v1138 + v1139);
        let v1141: f64 = (v1131 * v280);
        let v1142: f64 = (v283 * v1119);
        let v1143: f64 = (v1141 + v1142);
        let v1144: f64 = (v1124 + v1134);
        let v1145: f64 = (v1125 + v1137);
        let v1146: f64 = (v1126 + v1140);
        let v1147: f64 = (v1127 + v1143);
        let v1148: f64 = (v517 * v285);
        let v1149: f64 = (v123 * v1144);
        let v1150: f64 = (v1148 + v1149);
        let v1151: f64 = (v123 * v1145);
        let v1152: f64 = (v518 * v285);
        let v1153: f64 = (v123 * v1146);
        let v1154: f64 = (v1152 + v1153);
        let v1155: f64 = (v123 * v1147);
        let v1156: f64 = (if self.scalar_v267 { v1150 } else { v723 });
        let v1157: f64 = (if self.scalar_v267 { v1151 } else { v724 });
        let v1158: f64 = (if self.scalar_v267 { v1154 } else { v725 });
        let v1159: f64 = (if self.scalar_v267 { v1155 } else { v726 });
        let v1160: f64 = { let limexp_arg = v277; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v1161: f64 = (v1160 * v1096);
        let v1162: f64 = (v1160 * v1097);
        let v1163: f64 = (v1160 * v1098);
        let v1164: f64 = (v1160 * v1099);
        let v1165: f64 = (-v1096);
        let v1166: f64 = (-v1097);
        let v1167: f64 = (-v1098);
        let v1168: f64 = (-v1099);
        let v1169: f64 = { let limexp_arg = v289; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v1170: f64 = (v1169 * v1165);
        let v1171: f64 = (v1169 * v1166);
        let v1172: f64 = (v1169 * v1167);
        let v1173: f64 = (v1169 * v1168);
        let v1174: f64 = (v1161 - v1170);
        let v1175: f64 = (v1162 - v1171);
        let v1176: f64 = (v1163 - v1172);
        let v1177: f64 = (v1164 - v1173);
        let v1178: f64 = (v108 * v1174);
        let v1179: f64 = (v108 * v1175);
        let v1180: f64 = (v108 * v1176);
        let v1181: f64 = (v108 * v1177);
        let v1182: f64 = (v293 * v293);
        let v1183: f64 = (v42 - v1182);
        let v1184: f64 = (v1183 * v1178);
        let v1185: f64 = (v1183 * v1179);
        let v1186: f64 = (v1183 * v1180);
        let v1187: f64 = (v1183 * v1181);
        let v1188: f64 = (if self.scalar_v267 { v1184 } else { v962 });
        let v1189: f64 = (if self.scalar_v267 { v1185 } else { v963 });
        let v1190: f64 = (if self.scalar_v267 { v1186 } else { v964 });
        let v1191: f64 = (if self.scalar_v267 { v1187 } else { v965 });
        let v1192: f64 = { let limexp_arg = v287; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v1193: f64 = (v1192 * v1156);
        let v1194: f64 = (v1192 * v1157);
        let v1195: f64 = (v1192 * v1158);
        let v1196: f64 = (v1192 * v1159);
        let v1197: f64 = (-v1156);
        let v1198: f64 = (-v1157);
        let v1199: f64 = (-v1158);
        let v1200: f64 = (-v1159);
        let v1201: f64 = { let limexp_arg = v297; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v1202: f64 = (v1201 * v1197);
        let v1203: f64 = (v1201 * v1198);
        let v1204: f64 = (v1201 * v1199);
        let v1205: f64 = (v1201 * v1200);
        let v1206: f64 = (v1193 - v1202);
        let v1207: f64 = (v1194 - v1203);
        let v1208: f64 = (v1195 - v1204);
        let v1209: f64 = (v1196 - v1205);
        let v1210: f64 = (v108 * v1206);
        let v1211: f64 = (v108 * v1207);
        let v1212: f64 = (v108 * v1208);
        let v1213: f64 = (v108 * v1209);
        let v1214: f64 = (v301 * v301);
        let v1215: f64 = (v42 - v1214);
        let v1216: f64 = (v1215 * v1210);
        let v1217: f64 = (v1215 * v1211);
        let v1218: f64 = (v1215 * v1212);
        let v1219: f64 = (v1215 * v1213);
        let v1220: f64 = (if self.scalar_v267 { v1216 } else { v11 });
        let v1221: f64 = (if self.scalar_v267 { v1217 } else { v11 });
        let v1222: f64 = (if self.scalar_v267 { v1218 } else { v11 });
        let v1223: f64 = (if self.scalar_v267 { v1219 } else { v11 });
        let v1224: f64 = (self.scalar_v126 * v1188);
        let v1225: f64 = (self.scalar_v126 * v1189);
        let v1226: f64 = (self.scalar_v126 * v1190);
        let v1227: f64 = (self.scalar_v126 * v1191);
        let v1228: f64 = (if self.scalar_v267 { v1224 } else { v970 });
        let v1229: f64 = (if self.scalar_v267 { v1225 } else { v971 });
        let v1230: f64 = (if self.scalar_v267 { v1226 } else { v972 });
        let v1231: f64 = (if self.scalar_v267 { v1227 } else { v973 });
        let v1232: f64 = (self.scalar_v126 * v1220);
        let v1233: f64 = (self.scalar_v126 * v1221);
        let v1234: f64 = (self.scalar_v126 * v1222);
        let v1235: f64 = (self.scalar_v126 * v1223);
        let v1236: f64 = (if self.scalar_v267 { v1232 } else { v11 });
        let v1237: f64 = (if self.scalar_v267 { v1233 } else { v11 });
        let v1238: f64 = (if self.scalar_v267 { v1234 } else { v11 });
        let v1239: f64 = (if self.scalar_v267 { v1235 } else { v11 });
        let v1240: f64 = (v1228 * v7);
        let v1241: f64 = (v1240 + v306);
        let v1242: f64 = (v1229 * v7);
        let v1243: f64 = (v1230 * v7);
        let v1244: f64 = (v306 * v354);
        let v1245: f64 = (v1243 + v1244);
        let v1246: f64 = (v1231 * v7);
        let v1247: f64 = (v311 * v311);
        let v1248: f64 = (v42 - v1247);
        let v1249: f64 = (v1248 * v1241);
        let v1250: f64 = (v1248 * v1242);
        let v1251: f64 = (v1248 * v1245);
        let v1252: f64 = (v1248 * v1246);
        let v1253: f64 = (if self.scalar_v267 { v1249 } else { v987 });
        let v1254: f64 = (if self.scalar_v267 { v1250 } else { v988 });
        let v1255: f64 = (if self.scalar_v267 { v1251 } else { v989 });
        let v1256: f64 = (if self.scalar_v267 { v1252 } else { v990 });
        let v1257: f64 = (v1236 * v7);
        let v1258: f64 = (v1257 + v309);
        let v1259: f64 = (v1237 * v7);
        let v1260: f64 = (v1238 * v7);
        let v1261: f64 = (v309 * v354);
        let v1262: f64 = (v1260 + v1261);
        let v1263: f64 = (v1239 * v7);
        let v1264: f64 = (v314 * v314);
        let v1265: f64 = (v42 - v1264);
        let v1266: f64 = (v1265 * v1258);
        let v1267: f64 = (v1265 * v1259);
        let v1268: f64 = (v1265 * v1262);
        let v1269: f64 = (v1265 * v1263);
        let v1270: f64 = (if self.scalar_v267 { v1266 } else { v11 });
        let v1271: f64 = (if self.scalar_v267 { v1267 } else { v11 });
        let v1272: f64 = (if self.scalar_v267 { v1268 } else { v11 });
        let v1273: f64 = (if self.scalar_v267 { v1269 } else { v11 });
        let v1274: f64 = (self.scalar_v198 * v1220);
        let v1275: f64 = (self.scalar_v198 * v1221);
        let v1276: f64 = (self.scalar_v198 * v1222);
        let v1277: f64 = (self.scalar_v198 * v1223);
        let v1278: f64 = (if self.scalar_v267 { v1274 } else { v11 });
        let v1279: f64 = (if self.scalar_v267 { v1275 } else { v11 });
        let v1280: f64 = (if self.scalar_v267 { v1276 } else { v11 });
        let v1281: f64 = (if self.scalar_v267 { v1277 } else { v11 });
        let v1282: f64 = (self.scalar_v198 * v1188);
        let v1283: f64 = (self.scalar_v198 * v1189);
        let v1284: f64 = (self.scalar_v198 * v1190);
        let v1285: f64 = (self.scalar_v198 * v1191);
        let v1286: f64 = (if self.scalar_v267 { v1282 } else { v11 });
        let v1287: f64 = (if self.scalar_v267 { v1283 } else { v11 });
        let v1288: f64 = (if self.scalar_v267 { v1284 } else { v11 });
        let v1289: f64 = (if self.scalar_v267 { v1285 } else { v11 });
        let v1290: f64 = (v95 * v1188);
        let v1291: f64 = (v95 * v1189);
        let v1292: f64 = (v95 * v1190);
        let v1293: f64 = (v95 * v1191);
        let v1294: f64 = (v1290 * v323);
        let v1295: f64 = (v322 * v1253);
        let v1296: f64 = (v1294 + v1295);
        let v1297: f64 = (v1291 * v323);
        let v1298: f64 = (v322 * v1254);
        let v1299: f64 = (v1297 + v1298);
        let v1300: f64 = (v1292 * v323);
        let v1301: f64 = (v322 * v1255);
        let v1302: f64 = (v1300 + v1301);
        let v1303: f64 = (v1293 * v323);
        let v1304: f64 = (v322 * v1256);
        let v1305: f64 = (v1303 + v1304);
        let v1306: f64 = (v1286 * v7);
        let v1307: f64 = (v1306 + v321);
        let v1308: f64 = (v1287 * v7);
        let v1309: f64 = (v1288 * v7);
        let v1310: f64 = (v321 * v354);
        let v1311: f64 = (v1309 + v1310);
        let v1312: f64 = (v1289 * v7);
        let v1313: f64 = (v1307 + v770);
        let v1314: f64 = (v1311 + v771);
        let v1315: f64 = (v1296 * v327);
        let v1316: f64 = (v324 * v1313);
        let v1317: f64 = (v1315 + v1316);
        let v1318: f64 = (v1299 * v327);
        let v1319: f64 = (v324 * v1308);
        let v1320: f64 = (v1318 + v1319);
        let v1321: f64 = (v1302 * v327);
        let v1322: f64 = (v324 * v1314);
        let v1323: f64 = (v1321 + v1322);
        let v1324: f64 = (v1305 * v327);
        let v1325: f64 = (v324 * v1312);
        let v1326: f64 = (v1324 + v1325);
        let v1327: f64 = (if self.scalar_v267 { v1317 } else { v786 });
        let v1328: f64 = (if self.scalar_v267 { v1320 } else { v787 });
        let v1329: f64 = (if self.scalar_v267 { v1323 } else { v788 });
        let v1330: f64 = (if self.scalar_v267 { v1326 } else { v789 });
        let v1331: f64 = (v95 * v1220);
        let v1332: f64 = (v95 * v1221);
        let v1333: f64 = (v95 * v1222);
        let v1334: f64 = (v95 * v1223);
        let v1335: f64 = (-v1270);
        let v1336: f64 = (-v1271);
        let v1337: f64 = (-v1272);
        let v1338: f64 = (-v1273);
        let v1339: f64 = (v1331 * v331);
        let v1340: f64 = (v330 * v1335);
        let v1341: f64 = (v1339 + v1340);
        let v1342: f64 = (v1332 * v331);
        let v1343: f64 = (v330 * v1336);
        let v1344: f64 = (v1342 + v1343);
        let v1345: f64 = (v1333 * v331);
        let v1346: f64 = (v330 * v1337);
        let v1347: f64 = (v1345 + v1346);
        let v1348: f64 = (v1334 * v331);
        let v1349: f64 = (v330 * v1338);
        let v1350: f64 = (v1348 + v1349);
        let v1351: f64 = (v1278 * v7);
        let v1352: f64 = (v1351 + v318);
        let v1353: f64 = (v1279 * v7);
        let v1354: f64 = (v1280 * v7);
        let v1355: f64 = (v318 * v354);
        let v1356: f64 = (v1354 + v1355);
        let v1357: f64 = (v1281 * v7);
        let v1358: f64 = (-v1352);
        let v1359: f64 = (-v1353);
        let v1360: f64 = (-v1356);
        let v1361: f64 = (-v1357);
        let v1362: f64 = (v1341 * v334);
        let v1363: f64 = (v332 * v1358);
        let v1364: f64 = (v1362 + v1363);
        let v1365: f64 = (v1344 * v334);
        let v1366: f64 = (v332 * v1359);
        let v1367: f64 = (v1365 + v1366);
        let v1368: f64 = (v1347 * v334);
        let v1369: f64 = (v332 * v1360);
        let v1370: f64 = (v1368 + v1369);
        let v1371: f64 = (v1350 * v334);
        let v1372: f64 = (v332 * v1361);
        let v1373: f64 = (v1371 + v1372);
        let v1374: f64 = (if self.scalar_v267 { v1364 } else { v858 });
        let v1375: f64 = (if self.scalar_v267 { v1367 } else { v859 });
        let v1376: f64 = (if self.scalar_v267 { v1370 } else { v860 });
        let v1377: f64 = (if self.scalar_v267 { v1373 } else { v861 });
        let v1378: f64 = (v1327 - v1374);
        let v1379: f64 = (v1328 - v1375);
        let v1380: f64 = (v1329 - v1376);
        let v1381: f64 = (v1330 - v1377);
        let v1382: f64 = (v108 * v1378);
        let v1383: f64 = (v108 * v1379);
        let v1384: f64 = (v108 * v1380);
        let v1385: f64 = (v108 * v1381);
        let v1386: f64 = (if self.scalar_v267 { v1382 } else { v1036 });
        let v1387: f64 = (if self.scalar_v267 { v1383 } else { v1037 });
        let v1388: f64 = (if self.scalar_v267 { v1384 } else { v1038 });
        let v1389: f64 = (if self.scalar_v267 { v1385 } else { v1039 });
        let v1390: f64 = (v99 * v583);
        let v1391: f64 = (-v1390);
        let v1392: f64 = (v341 * v341);
        let v1393: f64 = (v1391 / v1392);
        let v1394: f64 = (v99 * v584);
        let v1395: f64 = (-v1394);
        let v1396: f64 = (v1395 / v1392);
        let v1397: f64 = (v99 * v585);
        let v1398: f64 = (-v1397);
        let v1399: f64 = (v1398 / v1392);
        let v1400: f64 = (v99 * v586);
        let v1401: f64 = (-v1400);
        let v1402: f64 = (v1401 / v1392);
        let v1403: f64 = (if self.scalar_v228 { v1393 } else { v11 });
        let v1404: f64 = (if self.scalar_v228 { v1396 } else { v11 });
        let v1405: f64 = (if self.scalar_v228 { v1399 } else { v11 });
        let v1406: f64 = (if self.scalar_v228 { v1402 } else { v11 });
        let v1407: f64 = (v99 * v1188);
        let v1408: f64 = (-v1407);
        let v1409: f64 = (v348 * v348);
        let v1410: f64 = (v1408 / v1409);
        let v1411: f64 = (v99 * v1189);
        let v1412: f64 = (-v1411);
        let v1413: f64 = (v1412 / v1409);
        let v1414: f64 = (v99 * v1190);
        let v1415: f64 = (-v1414);
        let v1416: f64 = (v1415 / v1409);
        let v1417: f64 = (v99 * v1191);
        let v1418: f64 = (-v1417);
        let v1419: f64 = (v1418 / v1409);
        let v1420: f64 = (if self.scalar_v229 { v1410 } else { v1403 });
        let v1421: f64 = (if self.scalar_v229 { v1413 } else { v1404 });
        let v1422: f64 = (if self.scalar_v229 { v1416 } else { v1405 });
        let v1423: f64 = (if self.scalar_v229 { v1419 } else { v1406 });
        let v1424: f64 = (if self.scalar_v353 { v11 } else { v1040 });
        let v1425: f64 = (if self.scalar_v353 { v11 } else { v1041 });
        let v1426: f64 = (if self.scalar_v353 { v11 } else { v1042 });
        let v1430: f64 = (if self.scalar_v364 { v11 } else { v1424 });
        let v1431: f64 = (if self.scalar_v364 { v11 } else { v1425 });
        let v1432: f64 = (if self.scalar_v364 { v11 } else { v1426 });
        let v1434: f64 = (v371 * v371);
        let v1435: f64 = (v42 - v1434);
        let v1436: f64 = (v1435 * v354);
        let v1437: f64 = (if self.scalar_v370 { v1436 } else { self.scalar_v1428 });
        let v1438: f64 = (if self.scalar_v370 { v1435 } else { self.scalar_v1429 });
        let v1439: f64 = (v373 * v373);
        let v1440: f64 = (v42 - v1439);
        let v1441: f64 = (v1440 * v354);
        let v1442: f64 = (if self.scalar_v370 { v1441 } else { self.scalar_v1428 });
        let v1443: f64 = (if self.scalar_v370 { v1440 } else { self.scalar_v1429 });
        let v1444: f64 = (if self.scalar_v376 { v354 } else { v1437 });
        let v1445: f64 = (if self.scalar_v376 { v42 } else { v1438 });
        let v1446: f64 = (if self.scalar_v376 { v354 } else { v1442 });
        let v1447: f64 = (if self.scalar_v376 { v42 } else { v1443 });
        let v1448: f64 = (v115 * v1444);
        let v1449: f64 = (v115 * v1445);
        let v1450: f64 = { let limexp_arg = v380; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v1451: f64 = (v1450 * v1448);
        let v1452: f64 = (v1450 * v1449);
        let v1453: f64 = (-v1430);
        let v1454: f64 = (-v1431);
        let v1455: f64 = (v1451 - v1432);
        let v1456: f64 = (v1452 - self.scalar_v1433);
        let v1457: f64 = (self.scalar_v379 * v1453);
        let v1458: f64 = (self.scalar_v379 * v1454);
        let v1459: f64 = (self.scalar_v379 * v1455);
        let v1460: f64 = (self.scalar_v379 * v1456);
        let v1461: f64 = (v115 * v1446);
        let v1462: f64 = (v115 * v1447);
        let v1463: f64 = { let limexp_arg = v384; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } };
        let v1464: f64 = (v1463 * v1461);
        let v1465: f64 = (v1463 * v1462);
        let v1466: f64 = (v1464 - v1430);
        let v1467: f64 = (-v1432);
        let v1469: f64 = (self.scalar_v379 * v1466);
        let v1470: f64 = (self.scalar_v379 * v1467);
        let v1471: f64 = (self.scalar_v379 * v1465);
        let v1473: f64 = (-v1386);
        let v1474: f64 = (-v1387);
        let v1475: f64 = (-v1388);
        let v1476: f64 = (-v1389);
        let v1480: f64 = (v445 * v1420);
        let v1481: f64 = (-v1480);
        let v1482: f64 = (v351 * v351);
        let v1483: f64 = (v1481 / v1482);
        let v1484: f64 = (v445 * v1421);
        let v1485: f64 = (-v1484);
        let v1486: f64 = (v1485 / v1482);
        let v1487: f64 = (v354 * v351);
        let v1488: f64 = (v445 * v1422);
        let v1489: f64 = (v1487 - v1488);
        let v1490: f64 = (v1489 / v1482);
        let v1491: f64 = (v445 * v1423);
        let v1492: f64 = (-v1491);
        let v1493: f64 = (v1492 / v1482);
        let v1494: f64 = (v42 / v351);
        let v1495: f64 = (if self.scalar_v388 { v1483 } else { v11 });
        let v1496: f64 = (if self.scalar_v388 { v1486 } else { v11 });
        let v1497: f64 = (if self.scalar_v388 { v1490 } else { v11 });
        let v1498: f64 = (if self.scalar_v388 { v1493 } else { v11 });
        let v1499: f64 = (if self.scalar_v388 { v1494 } else { v11 });
        let v1514: f64 = (if self.scalar_v411 { v430 } else { v11 });
        let v1515: f64 = (if self.scalar_v411 { v426 } else { v11 });

        let d432_dn3: f64 = v1473;
        let d432_dn4: f64 = v1474;
        let d432_dn5: f64 = v1475;
        let d432_dn8: f64 = v1476;
        let v432_node_derivative_indices: [usize; 4] = [3, 4, 5, 8];
        let v432_node_derivatives: [f64; 4] = [d432_dn3, d432_dn4, d432_dn5, d432_dn8];
        let v432_branch_derivative_indices: [usize; 0] = [];
        let v432_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(12),
            None,
            multiplicity * (v432),
            &v432_node_derivative_indices,
            &v432_node_derivatives,
            &v432_branch_derivative_indices,
            &v432_branch_derivatives,
            multiplicity,
        );
        let d10_dn13: f64 = v42;
        stamper.stamp_current_node1_local(
            Some(13),
            None,
            multiplicity * (v10),
            13,
            multiplicity * (d10_dn13),
        );
        let d10_dn13: f64 = v42;
        stamper.stamp_current_node1_local(
            Some(3),
            Some(5),
            multiplicity * (v10),
            13,
            multiplicity * (d10_dn13),
        );
        let d383_dn3: f64 = v1457;
        let d383_dn4: f64 = v1458;
        let d383_dn5: f64 = v1459;
        let d383_dn8: f64 = v1460;
        let v383_node_derivative_indices: [usize; 4] = [3, 4, 5, 8];
        let v383_node_derivatives: [f64; 4] = [d383_dn3, d383_dn4, d383_dn5, d383_dn8];
        let v383_branch_derivative_indices: [usize; 0] = [];
        let v383_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(8),
            Some(5),
            multiplicity * (v383),
            &v383_node_derivative_indices,
            &v383_node_derivatives,
            &v383_branch_derivative_indices,
            &v383_branch_derivatives,
            multiplicity,
        );
        let d387_dn3: f64 = v1469;
        let d387_dn4: f64 = v1458;
        let d387_dn5: f64 = v1470;
        let d387_dn7: f64 = v1471;
        let d387_dn8: f64 = self.scalar_v1472;
        let v387_node_derivative_indices: [usize; 5] = [3, 4, 5, 7, 8];
        let v387_node_derivatives: [f64; 5] = [d387_dn3, d387_dn4, d387_dn5, d387_dn7, d387_dn8];
        let v387_branch_derivative_indices: [usize; 0] = [];
        let v387_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(7),
            Some(3),
            multiplicity * (v387),
            &v387_node_derivative_indices,
            &v387_node_derivatives,
            &v387_branch_derivative_indices,
            &v387_branch_derivatives,
            multiplicity,
        );
        let d447_dn3: f64 = v1495;
        let d447_dn4: f64 = v1496;
        let d447_dn5: f64 = v1497;
        let d447_dn8: f64 = v1498;
        let d447_dn10: f64 = v1499;
        let v447_node_derivative_indices: [usize; 5] = [3, 4, 5, 8, 10];
        let v447_node_derivatives: [f64; 5] = [d447_dn3, d447_dn4, d447_dn5, d447_dn8, d447_dn10];
        let v447_branch_derivative_indices: [usize; 0] = [];
        let v447_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(10),
            Some(5),
            multiplicity * (v447),
            &v447_node_derivative_indices,
            &v447_node_derivatives,
            &v447_branch_derivative_indices,
            &v447_branch_derivatives,
            multiplicity,
        );
        let d456_dn5: f64 = self.scalar_v1503;
        let d456_dn9: f64 = self.scalar_v1504;
        stamper.stamp_current_node2_local(
            Some(9),
            Some(5),
            multiplicity * (v456),
            5,
            multiplicity * (d456_dn5),
            9,
            multiplicity * (d456_dn9),
        );
        let d461_dn4: f64 = self.scalar_v1507;
        let d461_dn7: f64 = self.scalar_v1508;
        stamper.stamp_current_node2_local(
            Some(4),
            Some(7),
            multiplicity * (v461),
            4,
            multiplicity * (d461_dn4),
            7,
            multiplicity * (d461_dn7),
        );
        let d466_dn4: f64 = self.scalar_v1511;
        let d466_dn8: f64 = self.scalar_v1512;
        stamper.stamp_current_node2_local(
            Some(4),
            Some(8),
            multiplicity * (v466),
            4,
            multiplicity * (d466_dn4),
            8,
            multiplicity * (d466_dn8),
        );
        let d480_dn14: f64 = self.scalar_v1513;
        stamper.stamp_current_node1_local(
            Some(14),
            None,
            multiplicity * (v480),
            14,
            multiplicity * (d480_dn14),
        );
        let d482_dn15: f64 = self.scalar_v1513;
        stamper.stamp_current_node1_local(
            Some(15),
            None,
            multiplicity * (v482),
            15,
            multiplicity * (d482_dn15),
        );
        let d480_dn14: f64 = self.scalar_v1513;
        stamper.stamp_current_node1_local(
            Some(4),
            Some(5),
            multiplicity * (v480),
            14,
            multiplicity * (d480_dn14),
        );
        let d486_dn14: f64 = v1514;
        let d486_dn15: f64 = v1515;
        stamper.stamp_current_node2_local(
            Some(4),
            Some(3),
            multiplicity * (v486),
            14,
            multiplicity * (d486_dn14),
            15,
            multiplicity * (d486_dn15),
        );
        let d479_dn14: f64 = v42;
        stamper.stamp_current_node1_local(
            Some(14),
            None,
            multiplicity * (v479),
            14,
            multiplicity * (d479_dn14),
        );
        let d481_dn15: f64 = v42;
        stamper.stamp_current_node1_local(
            Some(15),
            None,
            multiplicity * (v481),
            15,
            multiplicity * (d481_dn15),
        );
        stamper.stamp_current_const_local(
            Some(11),
            None,
            multiplicity * (v493),
        );
        let d495_dn11: f64 = self.scalar_v1517;
        stamper.stamp_current_node1_local(
            Some(11),
            None,
            multiplicity * (v495),
            11,
            multiplicity * (d495_dn11),
        );
        let d499_dn11: f64 = self.scalar_v1518;
        stamper.stamp_current_node1_local(
            Some(11),
            None,
            multiplicity * (v499),
            11,
            multiplicity * (d499_dn11),
        );
        let s = match &mut self.scratch {
            Some(buf) => buf.as_mut(),
            slot @ None => slot.insert(Scratch::new_box()).as_mut(),
        };

        s.store_voltage(4, ctx, nodes, Some(8), Some(5));

        s.store_voltage(3, ctx, nodes, Some(4), Some(3));

        s.store_neg(6, 3);

        s.store_voltage(5, ctx, nodes, Some(3), Some(5));

        s.copy_ad(79, 4);

        s.store_voltage(80, ctx, nodes, Some(7), Some(3));

        s.v[21] = 0.0;

        s.v[20] = 0.0;

        s.v[19] = 0.0;

        s.v[18] = 0.0;

        s.b[82] = param_given[3];
        s.v[82] = if s.b[82] { 1.0 } else { 0.0 };

        if s.b[82] {
            s.store_scalar(11, (p.p3 + 273.15));
        }

        if (!s.b[82]) {
            s.store_scalar(11, (ctx_temp + p.p2));
        }

        s.b[83] = param_given[85];
        s.v[83] = if s.b[83] { 1.0 } else { 0.0 };

        if s.b[83] {
            s.store_scalar(10, (p.p85 + 273.15));
        }

        if (!s.b[83]) {
            s.store_scalar(10, (27.0 + 273.15));
        }

        if (p.p1 != 0.0) {
            s.store_add_ad_rhs(11, 11, A::abs(A::voltage(ctx, nodes, Some(11), None)));
        }

        s.store_scale(9, 11, THERMAL_VOLTAGE_PER_K);

        s.store_abs_ad(12, A::sub(s.ad_value(11), s.ad_value(10)));

        s.b[84] = ((s.v[12] > 0.0) || (p.p57 > 0.0));
        s.v[84] = if s.b[84] { 1.0 } else { 0.0 };

        if s.b[84] {
            s.store_offset_scaled(45, 12, ((p.p60) * (p.p11)), p.p11);
            s.store_offset_scaled(31, 12, ((p.p61) * (p.p25)), p.p25);
            s.store_offset_scaled(32, 12, ((p.p62) * (p.p28)), p.p28);
            s.store_offset_scaled(34, 12, ((p.p65) * (p.p54)), p.p54);
            s.store_offset_scaled(39, 12, p.p68, p.p9);
            s.store_offset_scaled(40, 12, (p.p30 * p.p68), p.p29);
            s.store_offset_scaled(41, 12, (p.p36 * p.p68), p.p35);
            s.store_offset_scaled(42, 12, p.p69, p.p41);
            s.store_offset_scaled(38, 12, p.p70, p.p21);
        }

        if (!s.b[84]) {
            s.store_scalar(45, p.p11);
            s.store_scalar(31, p.p25);
            s.store_scalar(32, p.p28);
            s.store_scalar(34, p.p54);
            s.store_scalar(39, p.p9);
            s.store_scalar(40, p.p29);
            s.store_scalar(41, p.p35);
            s.store_scalar(42, p.p41);
            s.store_scalar(38, p.p21);
        }

        s.b[85] = ((!param_given[39]) && param_given[40]);
        s.v[85] = if s.b[85] { 1.0 } else { 0.0 };

        if s.b[85] {
            s.store_div_from_scalar(15, (0.5 / p.p40), 9);
        }

        if (!s.b[85]) {
            s.store_scalar(15, p.p39);
        }

        s.store_cosh_ad(47, A::scale(s.ad_value(5), p.p19));

        s.store_mul_offset_ad_rhs(44, 45, A::div_from_scalar(p.p18, A::square(s.ad_value(47))), 1.0);

        s.store_add_scaled_inputs_product(46, A::offset(s.ad_value(39), (-p.p10)), 1.0, A::tanh_scaled_input(s.ad_value(5), p.p15), p.p10, A::offset(s.ad_value(6), (-p.p21)), A::sub(s.ad_value(6), s.ad_value(38)), (-p.p22));

        s.store_sub(48, 4, 46);

        s.store_square(49, 48);

        s.store_add_scaled_value_products(13, s.ad_value(49), p.p12, s.ad_value(44), s.ad_value(48), 1.0, s.ad_value(48), s.ad_value(49), p.p13);

        s.store_offset_tanh_ad(59, s.ad_value(13), 1.0);

        s.store_offset_ad(60, A::tanh_scaled_input(A::sub(A::limexp(s.ad_value(13)), A::limexp_scaled_input(s.ad_value(13), -1.0)), 0.5), 1.0);

        s.b[86] = (p.p4 == 0.0);
        s.v[86] = if s.b[86] { 1.0 } else { 0.0 };

        s.b[87] = (p.p4 == 1.0);
        s.v[87] = if s.b[87] { 1.0 } else { 0.0 };

        s.b[88] = (p.p4 == 2.0);
        s.v[88] = if s.b[88] { 1.0 } else { 0.0 };

        s.b[89] = (p.p4 == 3.0);
        s.v[89] = if s.b[89] { 1.0 } else { 0.0 };

        if (s.b[87] && (!s.b[86])) {
            s.store_sub(47, 3, 46);
            s.store_square(48, 47);
            s.store_mul(49, 48, 47);
        }

        if (s.b[88] && (!(s.b[86] || s.b[87]))) {
            s.store_sub(47, 4, 46);
            s.store_square(48, 47);
            s.store_mul_ad_rhs(13, 44, A::add_scaled_inputs_product(s.ad_value(47), 1.0, s.ad_value(48), p.p12, s.ad_value(48), s.ad_value(47), p.p13));
            s.store_offset_ad(60, A::tanh_scaled_input(A::sub(A::limexp(s.ad_value(13)), A::limexp_scaled_input(s.ad_value(13), -1.0)), 0.5), 1.0);
        }

        if (s.b[89] && (!((s.b[86] || s.b[87]) || s.b[88]))) {
            s.store_sub(47, 4, 46);
            s.store_square(48, 47);
            s.store_mul_ad_rhs(13, 44, A::add_scaled_inputs_product(s.ad_value(47), 1.0, s.ad_value(48), p.p12, s.ad_value(48), s.ad_value(47), p.p13));
            s.store_sub(49, 3, 46);
            s.store_offset_ad(60, A::tanh_scaled_input(A::sub(A::limexp(s.ad_value(13)), A::limexp_scaled_input(s.ad_value(13), -1.0)), 0.5), 1.0);
        }

        s.b[90] = ((p.p4 == 0.0) || (p.p4 == 1.0));
        s.v[90] = if s.b[90] { 1.0 } else { 0.0 };

        if s.b[90] {
            s.store_offset_scaled(28, 59, p.p44, p.p43);
            s.store_offset_scaled(29, 59, p.p44, p.p46);
        }

        if (!s.b[90]) {
            s.store_offset_scaled(28, 60, p.p44, p.p43);
            s.store_offset_scaled(29, 60, p.p44, p.p46);
        }

        s.b[91] = ((s.v[12] != 0.0) || (p.p57 > 0.0));
        s.v[91] = if s.b[91] { 1.0 } else { 0.0 };

        if s.b[91] {
            s.store_mul_scale_offset_rhs(36, 29, 12, p.p66, 1.0);
            s.store_mul_scale_offset_rhs(35, 28, 12, p.p66, 1.0);
        }

        if (!s.b[91]) {
            s.copy_ad(35, 28);
            s.copy_ad(36, 29);
        }

        s.b[92] = (p.p5 == 0.0);
        s.v[92] = if s.b[92] { 1.0 } else { 0.0 };

        if s.b[92] {
            s.store_limexp_ad(47, A::mul(s.ad_value(15), A::tanh_scaled_input(s.ad_value(42), (-1.0))));
        }

        if (!s.b[92]) {
            s.store_limexp_ad(47, A::mul_scaled_lhs(s.ad_value(15), -1.0, s.ad_value(42)));
        }

        s.store_add_scaled_inputs3(22, s.ad_value(40), 1.0, s.ad_value(79), p.p30, s.ad_value(5), p.p37);

        s.store_offset_tanh_ad(67, s.ad_value(22), 1.0);

        s.store_offset_scaled(23, 5, p.p32, p.p31);

        s.store_offset_tanh_ad(68, s.ad_value(23), 1.0);

        s.store_sub_from_scalar_scaled_input(24, p.p33, 5, p.p34);

        s.store_offset_tanh_ad(69, s.ad_value(24), ((1.0) + ((-p.p37))));

        s.store_add_scaled_inputs3(25, s.ad_value(41), 1.0, s.ad_value(80), p.p36, s.ad_value(5), (-p.p37));

        s.store_offset_tanh_ad(70, s.ad_value(25), 1.0);

        s.b[94] = (p.p6 == 0.0);
        s.v[94] = if s.b[94] { 1.0 } else { 0.0 };

        s.b[95] = (p.p6 == 1.0);
        s.v[95] = if s.b[95] { 1.0 } else { 0.0 };

        s.b[96] = (p.p6 == 2.0);
        s.v[96] = if s.b[96] { 1.0 } else { 0.0 };

        if s.b[94] {
            s.store_scalar(18, p.p24);
            s.store_scalar(19, p.p26);
        }

        if (s.b[95] && (!s.b[94])) {
            s.store_offset_product3(18, s.ad_value(31), s.ad_value(67), s.ad_value(68), 1.0, p.p24);
            s.store_offset_mul_offset_rhs_ad_rhs(19, 32, A::mul(s.ad_value(69), s.ad_value(70)), (2.0 * p.p37), p.p26);
        }

        if (s.b[96] && (!(s.b[94] || s.b[95]))) {
            s.store_offset(68, 68, (-p.p37));
            s.store_cosh_ad(71, A::add_scaled_inputs(s.ad_value(40), 1.0, s.ad_value(5), p.p37));
            s.store_ln(74, 71);
            s.store_cosh(72, 22);
            s.store_ln(73, 72);
            s.store_add_scaled_inputs3(77, s.ad_value(40), 1.0, s.ad_value(5), p.p37, s.ad_value(74), 1.0);
            s.store_add_scaled_product_right_ad(20, 79, p.p24, 31, A::add_scaled_product(s.ad_value(79), (2.0 * p.p37), A::add_scaled_inputs3(s.ad_value(22), 1.0, s.ad_value(73), 1.0, s.ad_value(77), -1.0), s.ad_value(68), 1.0 / (p.p30)), 1.0);
            s.store_cosh_ad(71, A::sub_scaled_inputs(s.ad_value(41), 1.0, s.ad_value(5), p.p37));
            s.store_ln(76, 71);
            s.store_cosh(72, 25);
            s.store_ln(75, 72);
            s.store_add_scaled_inputs3(78, s.ad_value(41), 1.0, s.ad_value(5), (-p.p37), s.ad_value(76), 1.0);
            s.store_add_scaled_product_right_ad(21, 80, p.p26, 32, A::add_scaled_product(s.ad_value(80), (2.0 * p.p37), A::add_scaled_inputs3(s.ad_value(25), 1.0, s.ad_value(75), 1.0, s.ad_value(78), -1.0), s.ad_value(69), 1.0 / (p.p36)), 1.0);
            s.store_scalar(18, A::ddx_projection(&s.ad_value(20), Some(8), None));
            s.store_scalar(19, A::ddx_projection(&s.ad_value(21), Some(7), None));
        }

        s.b[97] = (p.p6 == 2.0);
        s.v[97] = if s.b[97] { 1.0 } else { 0.0 };

        s.b[102] = (p.p42 > 0.0);
        s.v[102] = if s.b[102] { 1.0 } else { 0.0 };

        s.b[103] = (p.p50 > 0.0);
        s.v[103] = if s.b[103] { 1.0 } else { 0.0 };

        s.b[104] = (p.p46 > 0.0);
        s.v[104] = if s.b[104] { 1.0 } else { 0.0 };

        s.b[105] = ((p.p43 > 0.0) || (p.p44 > 0.0));
        s.v[105] = if s.b[105] { 1.0 } else { 0.0 };

        s.b[106] = (p.p48 > 0.0);
        s.v[106] = if s.b[106] { 1.0 } else { 0.0 };

        s.b[107] = (p.p7 == 0.0);
        s.v[107] = if s.b[107] { 1.0 } else { 0.0 };

        s.b[108] = (p.p7 == 1.0);
        s.v[108] = if s.b[108] { 1.0 } else { 0.0 };

        if ((s.b[108] && (!s.b[107])) && (p.p0 != 0.0)) {
            s.store_scaled_mul(120, 11, 31, (((4.0 * 1.3806503e-23) * p.p73) * (((p.p72 * p.p71)) as f64).sqrt()));
            s.store_scale(118, 120, 3.141592653589793);
        }

        s.b[124] = ((p.p1 != 0.0) && (p.p57 != 0.0));
        s.v[124] = if s.b[124] { 1.0 } else { 0.0 };

        stamper.stamp_potential_branch_local(
            Some(12),
            Some(13),
            0,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(10),
            Some(5),
            1,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(9),
            Some(5),
            2,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(4),
            Some(7),
            3,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(4),
            Some(8),
            4,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(1),
            Some(4),
            5,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(1),
            Some(4),
            6,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(1),
            Some(4),
            7,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(1),
            Some(4),
            8,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(1),
            Some(4),
            9,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(5),
            Some(6),
            10,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(5),
            Some(6),
            11,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(5),
            Some(6),
            12,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(6),
            Some(2),
            13,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(3),
            Some(0),
            14,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(3),
            Some(0),
            15,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(3),
            Some(0),
            16,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(3),
            Some(0),
            17,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(3),
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
        let param_given = self.param_given.as_ref();
        let multiplicity = (*self).multiplicity;
        let s = match &mut self.reactive_scratch {
            Some(buf) => buf.as_mut(),
            slot @ None => slot.insert(ReactiveScratch::new_box()).as_mut(),
        };

        s.store_voltage(4, ctx, nodes, Some(8), Some(5));

        s.store_voltage(5, ctx, nodes, Some(3), Some(5));

        s.copy_ad(79, 4);

        s.store_voltage(80, ctx, nodes, Some(7), Some(3));

        s.v[21] = 0.0;

        s.v[20] = 0.0;

        s.v[19] = 0.0;

        s.v[18] = 0.0;

        s.b[82] = param_given[3];
        s.v[82] = if s.b[82] { 1.0 } else { 0.0 };

        if s.b[82] {
            s.store_scalar(11, (p.p3 + 273.15));
        }

        if (!s.b[82]) {
            s.store_scalar(11, (ctx_temp + p.p2));
        }

        s.b[83] = param_given[85];
        s.v[83] = if s.b[83] { 1.0 } else { 0.0 };

        if s.b[83] {
            s.store_scalar(10, (p.p85 + 273.15));
        }

        if (!s.b[83]) {
            s.store_scalar(10, (27.0 + 273.15));
        }

        if (p.p1 != 0.0) {
            s.store_add_ad_rhs(11, 11, A::abs(A::voltage(ctx, nodes, Some(11), None)));
        }

        s.store_abs_ad(12, A::sub(s.ad_value(11), s.ad_value(10)));

        s.b[84] = ((s.v[12] > 0.0) || (p.p57 > 0.0));
        s.v[84] = if s.b[84] { 1.0 } else { 0.0 };

        if s.b[84] {
            s.store_offset_scaled(31, 12, ((p.p61) * (p.p25)), p.p25);
            s.store_offset_scaled(32, 12, ((p.p62) * (p.p28)), p.p28);
            s.store_offset_scaled(34, 12, ((p.p65) * (p.p54)), p.p54);
            s.store_offset_scaled(40, 12, (p.p30 * p.p68), p.p29);
            s.store_offset_scaled(41, 12, (p.p36 * p.p68), p.p35);
        }

        if (!s.b[84]) {
            s.store_scalar(31, p.p25);
            s.store_scalar(32, p.p28);
            s.store_scalar(34, p.p54);
            s.store_scalar(40, p.p29);
            s.store_scalar(41, p.p35);
        }

        s.store_add_scaled_inputs3(22, s.ad_value(40), 1.0, s.ad_value(79), p.p30, s.ad_value(5), p.p37);

        s.store_offset_tanh_ad(67, s.ad_value(22), 1.0);

        s.store_offset_scaled(23, 5, p.p32, p.p31);

        s.store_offset_tanh_ad(68, s.ad_value(23), 1.0);

        s.store_sub_from_scalar_scaled_input(24, p.p33, 5, p.p34);

        s.store_offset_tanh_ad(69, s.ad_value(24), ((1.0) + ((-p.p37))));

        s.store_add_scaled_inputs3(25, s.ad_value(41), 1.0, s.ad_value(80), p.p36, s.ad_value(5), (-p.p37));

        s.store_offset_tanh_ad(70, s.ad_value(25), 1.0);

        s.b[94] = (p.p6 == 0.0);
        s.v[94] = if s.b[94] { 1.0 } else { 0.0 };

        s.b[95] = (p.p6 == 1.0);
        s.v[95] = if s.b[95] { 1.0 } else { 0.0 };

        s.b[96] = (p.p6 == 2.0);
        s.v[96] = if s.b[96] { 1.0 } else { 0.0 };

        if s.b[94] {
            s.store_scalar(18, p.p24);
            s.store_scalar(19, p.p26);
        }

        if (s.b[95] && (!s.b[94])) {
            s.store_offset_product3(18, s.ad_value(31), s.ad_value(67), s.ad_value(68), 1.0, p.p24);
            s.store_offset_mul_offset_rhs_ad_rhs(19, 32, A::mul(s.ad_value(69), s.ad_value(70)), (2.0 * p.p37), p.p26);
        }

        if (s.b[96] && (!(s.b[94] || s.b[95]))) {
            s.store_offset(68, 68, (-p.p37));
            s.store_cosh_ad(71, A::add_scaled_inputs(s.ad_value(40), 1.0, s.ad_value(5), p.p37));
            s.store_ln(74, 71);
            s.store_cosh(72, 22);
            s.store_ln(73, 72);
            s.store_add_scaled_inputs3(77, s.ad_value(40), 1.0, s.ad_value(5), p.p37, s.ad_value(74), 1.0);
            s.store_add_scaled_product_right_ad(20, 79, p.p24, 31, A::add_scaled_product(s.ad_value(79), (2.0 * p.p37), A::add_scaled_inputs3(s.ad_value(22), 1.0, s.ad_value(73), 1.0, s.ad_value(77), -1.0), s.ad_value(68), 1.0 / (p.p30)), 1.0);
            s.store_cosh_ad(71, A::sub_scaled_inputs(s.ad_value(41), 1.0, s.ad_value(5), p.p37));
            s.store_ln(76, 71);
            s.store_cosh(72, 25);
            s.store_ln(75, 72);
            s.store_add_scaled_inputs3(78, s.ad_value(41), 1.0, s.ad_value(5), (-p.p37), s.ad_value(76), 1.0);
            s.store_add_scaled_product_right_ad(21, 80, p.p26, 32, A::add_scaled_product(s.ad_value(80), (2.0 * p.p37), A::add_scaled_inputs3(s.ad_value(25), 1.0, s.ad_value(75), 1.0, s.ad_value(78), -1.0), s.ad_value(69), 1.0 / (p.p36)), 1.0);
            s.store_scalar(18, A::ddx_projection(&s.ad_value(20), Some(8), None));
            s.store_scalar(19, A::ddx_projection(&s.ad_value(21), Some(7), None));
        }

        s.b[97] = (p.p6 == 2.0);
        s.v[97] = if s.b[97] { 1.0 } else { 0.0 };

        s.b[102] = (p.p42 > 0.0);
        s.v[102] = if s.b[102] { 1.0 } else { 0.0 };

        s.b[103] = (p.p50 > 0.0);
        s.v[103] = if s.b[103] { 1.0 } else { 0.0 };

        s.b[105] = ((p.p43 > 0.0) || (p.p44 > 0.0));
        s.v[105] = if s.b[105] { 1.0 } else { 0.0 };

        s.b[106] = (p.p48 > 0.0);
        s.v[106] = if s.b[106] { 1.0 } else { 0.0 };

        s.b[107] = (p.p7 == 0.0);
        s.v[107] = if s.b[107] { 1.0 } else { 0.0 };

        s.b[108] = (p.p7 == 1.0);
        s.v[108] = if s.b[108] { 1.0 } else { 0.0 };

        if ((s.b[108] && (!s.b[107])) && (p.p0 != 0.0)) {
            s.store_scaled_mul(120, 11, 31, (((4.0 * 1.3806503e-23) * p.p73) * (((p.p72 * p.p71)) as f64).sqrt()));
            s.store_scale(118, 120, 3.141592653589793);
        }

        s.b[124] = ((p.p1 != 0.0) && (p.p57 != 0.0));
        s.v[124] = if s.b[124] { 1.0 } else { 0.0 };

        Self::stamp_reactive_equations_block_0(ctx, stamper, s, p, nodes, branches, multiplicity);
    }
}
