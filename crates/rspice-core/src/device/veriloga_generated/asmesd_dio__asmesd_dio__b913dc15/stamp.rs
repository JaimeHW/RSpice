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
        let v1: f64 = ctx.node_voltage(nodes[2]);
        let v4: f64 = ((ctx.temperature() + v1) + self.scalar_v3);
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
        let v26: f64 = (v21 - v12);
        let v27: f64 = (self.scalar_v25 * v26);
        let v33: f64 = ((((v22 * self.scalar_v23) + (v27 / v20))) as f64).exp();
        let v34: f64 = (self.scalar_v32 * v33);
        let v36: f64 = (((v22 * self.scalar_v30)) as f64).exp();
        let v37: f64 = (self.scalar_v35 * v36);
        let v42: f64 = (self.scalar_v38 * (v12 + (v26 * self.scalar_v39)));
        let v47: f64 = (self.scalar_v43 * (v12 + (v26 * self.scalar_v44)));
        let v54: f64 = 300.15;
        let v56: f64 = (v11 / v54);
        let v58: f64 = 0.000702;
        let v59: f64 = (v11 * v58);
        let v60: f64 = (v11 * v59);
        let v62: f64 = (v11 + 1108.0);
        let v65: f64 = (-(1.16 - (v60 / v62)));
        let v66: f64 = 1.3806226e-23;
        let v68: f64 = (v66 * (v11 + v11));
        let v73: f64 = (-(v20 + v20));
        let v74: f64 = 1.5;
        let v77: f64 = 1.6021918e-19;
        let v79: f64 = ((v74 * ((v56) as f64).ln()) + (((v65 / v68) + 1.3454442398941469e20) * v77));
        let v80: f64 = (v73 * v79);
        let v83: f64 = ((self.scalar_v81 - v80) / self.scalar_v55);
        let v84: f64 = (self.scalar_v81 - v83);
        let v87: f64 = 0.0004;
        let v92: f64 = (v12 + (self.scalar_v86 * (self.scalar_v89 - (v84 / v83))));
        let v93: f64 = (self.scalar_v53 / v92);
        let v95: f64 = (v80 + (v56 * v83));
        let v96: f64 = (v95 - v83);
        let v102: f64 = (v12 + (self.scalar_v86 * ((v87 * (v11 - v54)) - (v96 / v83))));
        let v103: f64 = (v93 * v102);
        let v105: f64 = ctx.node_voltage(nodes[3]);
        let v106: f64 = ctx.node_voltage(nodes[4]);
        let v107: f64 = (v105 - v106);
        let v108: f64 = (self.scalar_v104 * v107);
        let v109: f64 = ctx.node_voltage(nodes[0]);
        let v110: f64 = (v109 - v105);
        let v112: f64 = ctx.node_voltage(nodes[1]);
        let v113: f64 = (v112 - v106);
        let v115: bool = (v34 > v13);
        let v117: f64 = (v20 * self.scalar_v116);
        let v119: f64 = (if v115 { (v108 / v117) } else { v13 });
        let v120: f64 = (-v108);
        let v121: f64 = (v120 - v47);
        let v123: f64 = (v20 * self.scalar_v122);
        let v125: f64 = (if v115 { (v121 / v123) } else { v13 });
        let v126: f64 = (-v47);
        let v128: f64 = (if v115 { (v126 / v123) } else { v13 });
        let v129: f64 = 80.0;
        let v130: bool = (v119 > v129);
        let v131: bool = (v115 && v130);
        let v135: f64 = (if v131 { v129 } else { v119 });
        let v137: bool = (v115 && (!v130));
        let v138: f64 = (if v137 { v12 } else { (if v131 { (v12 + (v119 - v129)) } else { v13 }) });
        let v139: f64 = ((v135) as f64).exp();
        let v141: f64 = (if v115 { (v138 * v139) } else { v138 });
        let v142: f64 = 37.0;
        let v143: bool = (v125 >= v142);
        let v144: bool = (!v143);
        let v145: f64 = -37.0;
        let v146: bool = (v125 <= v145);
        let v148: bool = (v144 && (!v146));
        let v149: f64 = ((v125) as f64).exp();
        let v150: f64 = (v12 + v149);
        let v152: bool = (v144 && v146);
        let v156: bool = (v128 >= v142);
        let v157: bool = (!v156);
        let v158: bool = (v128 <= v145);
        let v160: bool = (v157 && (!v158));
        let v161: f64 = ((v128) as f64).exp();
        let v162: f64 = (v12 + v161);
        let v164: bool = (v157 && v158);
        let v169: f64 = (if v115 { ((if v148 { ((v150) as f64).ln() } else { (if v152 { v149 } else { (if v143 { v125 } else { v13 }) }) }) - (if v160 { ((v162) as f64).ln() } else { (if v164 { v161 } else { (if v156 { v128 } else { v13 }) }) })) } else { v13 });
        let v170: f64 = (v141 - v12);
        let v172: f64 = (v42 * v169);
        let v174: f64 = ((v108) as f64).abs();
        let v175: f64 = f64::powf(v174, (self.scalar_v48 * (v12 + (v26 * self.scalar_v49))));
        let v177: f64 = (v12 + (self.scalar_v173 * v175));
        let v181: bool = (!v115);
        let v182: f64 = (if v181 { v13 } else { (if v115 { ((v34 * v170) - (v172 / v177)) } else { v13 }) });
        let v183: bool = (v37 > v13);
        let v185: f64 = (self.scalar_v184 - v108);
        let v186: f64 = 0.001;
        let v187: bool = (v185 > v186);
        let v189: f64 = (if v183 { (if v187 { v185 } else { v186 }) } else { v13 });
        let v190: f64 = -1.0;
        let v191: f64 = (v120 * self.scalar_v184);
        let v193: f64 = (v20 * self.scalar_v192);
        let v194: f64 = (v189 * v193);
        let v196: f64 = (if v183 { (v191 / v194) } else { v135 });
        let v197: bool = (v196 > v129);
        let v198: bool = (v183 && v197);
        let v204: bool = (v183 && (!v197));
        let v205: f64 = (if v204 { v12 } else { (if v198 { (v12 + (v196 - v129)) } else { v141 }) });
        let v206: f64 = (((if v198 { v129 } else { v196 })) as f64).exp();
        let v209: f64 = ((if v183 { (v205 * v206) } else { v205 }) - v12);
        let v212: bool = (!v183);
        let v214: f64 = (v182 - (if v212 { v13 } else { (if v183 { (v37 * v209) } else { v13 }) }));
        let v230: f64 = (((v22 * self.scalar_v228)) as f64).exp();
        let v233: f64 = f64::powf((v12 + f64::powf(((((self.scalar_v104 * v110) / self.scalar_v215)) as f64).abs(), self.scalar_v218)), self.scalar_v232);
        let v234: f64 = ((self.scalar_v227 * v230) * v233);
        let v238: f64 = (((v22 * self.scalar_v236)) as f64).exp();
        let v241: f64 = f64::powf((v12 + f64::powf(((((self.scalar_v104 * v113) / self.scalar_v221)) as f64).abs(), self.scalar_v224)), self.scalar_v240);
        let v242: f64 = ((self.scalar_v235 * v238) * v241);
        let v247: f64 = (if self.scalar_v244 { (v234 + self.scalar_v245) } else { v234 });
        let v251: f64 = (v109 - v112);
        let v265: f64 = (self.scalar_v261 * (v12 + ((f64::powf((v12 + f64::powf((((v251 / self.scalar_v252)) as f64).abs(), self.scalar_v255)), self.scalar_v258) - v12) * self.scalar_v262)));
        let v269: f64 = ctx.node_voltage(nodes[6]);
        let v275: f64 = (v12 + f64::powf((((v269) as f64).abs() / self.scalar_v271), self.scalar_v273));
        let v281: f64 = (v108 + ((-v95) * self.scalar_v279));
        let v282: bool = (v281 > v13);
        let v288: f64 = (if v282 { self.scalar_v287 } else { v13 });
        let v291: f64 = (v12 - (self.scalar_v284 * (self.scalar_v284 * v288)));
        let v298: f64 = (v281 * self.scalar_v297);
        let v300: f64 = (self.scalar_v284 + (v298 / v95));
        let v304: bool = (!v282);
        let v306: f64 = (v12 - (v108 / v95));
        let v309: f64 = (((self.scalar_v293 * ((v306) as f64).ln())) as f64).exp();
        let v310: f64 = (v12 - v309);
        let v315: f64 = ((if v304 { ((v95 * v310) / self.scalar_v293) } else { (if v282 { ((v95 * v291) / self.scalar_v293) } else { v13 }) }) + (if v304 { v13 } else { (if v282 { (v288 * (v281 * v300)) } else { v13 }) }));
        let v339: f64 = ((if self.scalar_v268 { (v247 / v275) } else { v247 }) / self.scalar_v16);
        let v343: f64 = ((if self.scalar_v244 { (v242 + self.scalar_v248) } else { v242 }) / self.scalar_v16);
        let v346: f64 = (if self.scalar_v268 { (v265 * (-v182)) } else { v13 });
        let v347: f64 = (if self.scalar_v268 { v269 } else { v13 });
        let v348: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, v269);
        let v350: f64 = (if self.scalar_v268 { (v265 * v348) } else { v13 });
        let v353: f64 = (-(((v214 * v251)) as f64).abs());
        let v354: f64 = (if self.scalar_v321 { v353 } else { v13 });
        let v356: f64 = (if self.scalar_v321 { (v1 / self.scalar_v319) } else { v13 });
        let v358: f64 = (v1 * self.scalar_v357);
        let v359: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, v358);
        let v360: f64 = (if self.scalar_v321 { v359 } else { v13 });
        let v363: f64 = (if self.scalar_v362 { v353 } else { v13 });
        let v364: f64 = ctx.node_voltage(nodes[5]);
        let v367: f64 = (if self.scalar_v362 { ((v1 - v364) / self.scalar_v319) } else { v13 });
        let v368: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, v358);
        let v369: f64 = (if self.scalar_v362 { v368 } else { v13 });
        let v371: f64 = (if self.scalar_v362 { (v364 / self.scalar_v325) } else { v13 });
        let v374: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, (v364 * self.scalar_v372));
        let v375: f64 = (if self.scalar_v362 { v374 } else { v13 });
        let v379: f64 = (if self.scalar_v378 { v353 } else { v13 });
        let v380: f64 = (v13 * v107);
        let v381: bool = (v339 > self.scalar_v336);
        let v382: f64 = (if v381 { v339 } else { self.scalar_v336 });
        let v384: f64 = (if self.scalar_v338 { (v110 / v382) } else { v13 });
        let v385: bool = (v343 > self.scalar_v336);
        let v386: f64 = (if v385 { v343 } else { self.scalar_v336 });
        let v388: f64 = (if self.scalar_v342 { (v113 / v386) } else { v13 });
        let v390: f64 = (self.scalar_v16 * (self.scalar_v104 * v214));
        let v392: f64 = (self.scalar_v16 * (self.scalar_v104 * (v103 * v315)));
        let v394: f64 = (self.scalar_v16 * (self.scalar_v104 * (v182 * v265)));
        let v396: f64 = (if v10 { v13 } else { (if v8 { v12 } else { v13 }) });
        let v397: f64 = (v19 * v396);
        let v398: f64 = (v396 / self.scalar_v18);
        let v399: f64 = (v398 / v21);
        let v419: f64 = (v396 / v54);
        let v433: f64 = ((v68 * (((v62 * ((v59 * v396) + (v11 * (v58 * v396)))) - (v60 * v396)) / (v62 * v62))) - (v65 * (v66 * (v396 + v396))));
        let v444: f64 = ((v79 * (-(v397 + v397))) + (v73 * ((v74 * (v419 / v56)) + (v77 * (v433 / (v68 * v68))))));
        let v446: f64 = ((-v444) / self.scalar_v55);
        let v451: f64 = (v83 * v83);
        let v462: f64 = (v444 + ((v83 * v419) + (v56 * v446)));
        let v473: f64 = ((v102 * ((-(self.scalar_v53 * (self.scalar_v86 * (-(((v83 * (-v446)) - (v84 * v446)) / v451))))) / (v92 * v92))) + (v93 * (self.scalar_v86 * ((v87 * v396) - (((v83 * (v462 - v446)) - (v96 * v446)) / v451)))));
        let v482: f64 = (if v115 { ((-(v108 * (self.scalar_v116 * v397))) / (v117 * v117)) } else { v13 });
        let v483: f64 = (if v115 { (self.scalar_v104 / v117) } else { v13 });
        let v484: f64 = (if v115 { (self.scalar_v474 / v117) } else { v13 });
        let v486: f64 = (self.scalar_v122 * v397);
        let v487: f64 = (v123 * (-(self.scalar_v43 * (self.scalar_v44 * v398))));
        let v490: f64 = (v123 * v123);
        let v494: f64 = (if v115 { ((v487 - (v121 * v486)) / v490) } else { v13 });
        let v495: f64 = (if v115 { (self.scalar_v474 / v123) } else { v13 });
        let v496: f64 = (if v115 { (self.scalar_v104 / v123) } else { v13 });
        let v500: f64 = (if v115 { ((v487 - (v126 * v486)) / v490) } else { v13 });
        let v504: f64 = (if v131 { v13 } else { v482 });
        let v505: f64 = (if v131 { v13 } else { v483 });
        let v506: f64 = (if v131 { v13 } else { v484 });
        let v507: f64 = (if v137 { v13 } else { (if v131 { v482 } else { v13 }) });
        let v508: f64 = (if v137 { v13 } else { (if v131 { v483 } else { v13 }) });
        let v509: f64 = (if v137 { v13 } else { (if v131 { v484 } else { v13 }) });
        let v522: f64 = (if v115 { ((v139 * v507) + (v138 * (v139 * v504))) } else { v507 });
        let v523: f64 = (if v115 { ((v139 * v508) + (v138 * (v139 * v505))) } else { v508 });
        let v524: f64 = (if v115 { ((v139 * v509) + (v138 * (v139 * v506))) } else { v509 });
        let v525: f64 = (v149 * v494);
        let v526: f64 = (v149 * v495);
        let v527: f64 = (v149 * v496);
        let v540: f64 = (v161 * v500);
        let v545: f64 = ((if v148 { (v525 / v150) } else { (if v152 { v525 } else { (if v143 { v494 } else { v13 }) }) }) - (if v160 { (v540 / v162) } else { (if v164 { v540 } else { (if v156 { v500 } else { v13 }) }) }));
        let v551: f64 = ((v170 * (self.scalar_v32 * (v33 * ((self.scalar_v23 * v399) + (((v20 * (self.scalar_v25 * v398)) - (v27 * v397)) / (v20 * v20)))))) + (v34 * v522));
        let v565: f64 = ((v177 * ((v169 * (self.scalar_v38 * (self.scalar_v39 * v398))) + (v42 * (if v115 { v545 } else { v13 })))) - (v172 * (self.scalar_v173 * ((self.scalar_v48 * (self.scalar_v49 * v398)) * (v175 * ((v174) as f64).ln())))));
        let v574: f64 = (if v115 { ((v34 * v523) - ((v42 * (if v115 { (if v148 { (v526 / v150) } else { (if v152 { v526 } else { (if v143 { v495 } else { v13 }) }) }) } else { v13 })) / v177)) } else { v13 });
        let v575: f64 = (if v115 { ((v34 * v524) - ((v42 * (if v115 { (if v148 { (v527 / v150) } else { (if v152 { v527 } else { (if v143 { v496 } else { v13 }) }) }) } else { v13 })) / v177)) } else { v13 });
        let v576: f64 = (if v181 { v13 } else { (if v115 { (v551 - (v565 / (v177 * v177))) } else { v13 }) });
        let v577: f64 = (if v181 { v13 } else { v574 });
        let v578: f64 = (if v181 { v13 } else { v575 });
        let v591: f64 = (v194 * v194);
        let v601: f64 = (if v183 { ((-(v191 * (v189 * (self.scalar_v192 * v397)))) / v591) } else { v504 });
        let v602: f64 = (if v183 { (((v194 * self.scalar_v583) - (v191 * (v193 * (if v183 { (if v187 { self.scalar_v474 } else { v13 }) } else { v13 })))) / v591) } else { v505 });
        let v603: f64 = (if v183 { (((v194 * self.scalar_v584) - (v191 * (v193 * (if v183 { (if v187 { self.scalar_v104 } else { v13 }) } else { v13 })))) / v591) } else { v506 });
        let v610: f64 = (if v204 { v13 } else { (if v198 { v601 } else { v522 }) });
        let v611: f64 = (if v204 { v13 } else { (if v198 { v602 } else { v523 }) });
        let v612: f64 = (if v204 { v13 } else { (if v198 { v603 } else { v524 }) });
        let v630: f64 = ((v209 * (self.scalar_v35 * (v36 * (self.scalar_v30 * v399)))) + (v37 * (if v183 { ((v206 * v610) + (v205 * (v206 * (if v198 { v13 } else { v601 })))) } else { v610 })));
        let v640: f64 = (v577 - (if v212 { v13 } else { (if v183 { (v37 * (if v183 { ((v206 * v611) + (v205 * (v206 * (if v198 { v13 } else { v602 })))) } else { v611 })) } else { v13 }) }));
        let v641: f64 = (v578 - (if v212 { v13 } else { (if v183 { (v37 * (if v183 { ((v206 * v612) + (v205 * (v206 * (if v198 { v13 } else { v603 })))) } else { v612 })) } else { v13 }) }));
        let v645: f64 = (v233 * (self.scalar_v227 * (v230 * (self.scalar_v228 * v399))));
        let v656: f64 = (self.scalar_v279 * (-v462));
        let v666: f64 = (v95 * v95);
        let v713: f64 = (if v304 { (((v310 * v462) + (v95 * (-(v309 * (self.scalar_v293 * ((-((-(v108 * v462)) / v666)) / v306)))))) / self.scalar_v293) } else { (if v282 { ((v291 * v462) / self.scalar_v293) } else { v13 }) });
        let v716: f64 = (if v304 { v13 } else { (if v282 { (v288 * ((v300 * v656) + (v281 * (((v95 * (self.scalar_v297 * v656)) - (v298 * v462)) / v666)))) } else { v13 }) });
        let v720: f64 = ((if v304 { ((v95 * (-(v309 * (self.scalar_v293 * ((-(self.scalar_v104 / v95)) / v306))))) / self.scalar_v293) } else { v13 }) + (if v304 { v13 } else { (if v282 { (v288 * ((self.scalar_v104 * v300) + (v281 * (self.scalar_v661 / v95)))) } else { v13 }) }));
        let v721: f64 = ((if v304 { ((v95 * (-(v309 * (self.scalar_v293 * ((-(self.scalar_v474 / v95)) / v306))))) / self.scalar_v293) } else { v13 }) + (if v304 { v13 } else { (if v282 { (v288 * ((v300 * self.scalar_v474) + (v281 * (self.scalar_v662 / v95)))) } else { v13 }) }));
        let v735: f64 = (if self.scalar_v268 { (v265 * (-v576)) } else { v13 });
        let v736: f64 = (if self.scalar_v268 { (v265 * (-v577)) } else { v13 });
        let v737: f64 = (if self.scalar_v268 { (v265 * (-v578)) } else { v13 });
        let v739: f64 = ddt_scale;
        let v741: f64 = (if self.scalar_v268 { (v265 * v739) } else { v13 });
        let v744: f64 = (self.scalar_v357 * v739);
        let v745: f64 = (if self.scalar_v321 { v744 } else { v13 });
        let v749: f64 = (if self.scalar_v362 { v744 } else { v13 });
        let v753: f64 = (if self.scalar_v362 { (self.scalar_v372 * v739) } else { v13 });
        let v754: f64 = -0.0;
        let v762: f64 = (if self.scalar_v338 { (v12 / v382) } else { v13 });
        let v763: f64 = (if self.scalar_v338 { ((-(v110 * (if v381 { ((if self.scalar_v268 { (v645 / v275) } else { v645 }) / self.scalar_v16) } else { v13 }))) / (v382 * v382)) } else { v13 });
        let v764: f64 = (if self.scalar_v338 { (v190 / v382) } else { v13 });
        let v772: f64 = (if self.scalar_v342 { (v12 / v386) } else { v13 });
        let v773: f64 = (if self.scalar_v342 { ((-(v113 * (if v385 { ((v241 * (self.scalar_v235 * (v238 * (self.scalar_v236 * v399)))) / self.scalar_v16) } else { v13 }))) / (v386 * v386)) } else { v13 });
        let v774: f64 = (if self.scalar_v342 { (v190 / v386) } else { v13 });
        let v778: f64 = (self.scalar_v16 * (self.scalar_v104 * (v576 - (if v212 { v13 } else { (if v183 { v630 } else { v13 }) }))));
        let v779: f64 = (self.scalar_v16 * (self.scalar_v104 * v640));
        let v780: f64 = (self.scalar_v16 * (self.scalar_v104 * v641));
        let v784: f64 = (self.scalar_v16 * (self.scalar_v104 * ((v315 * v473) + (v103 * (v713 + v716)))));
        let v785: f64 = (self.scalar_v16 * (self.scalar_v104 * (v103 * v720)));
        let v786: f64 = (self.scalar_v16 * (self.scalar_v104 * (v103 * v721)));
        let v790: f64 = (self.scalar_v16 * (self.scalar_v104 * (v265 * v576)));
        let v791: f64 = (self.scalar_v16 * (self.scalar_v104 * (v265 * v577)));
        let v792: f64 = (self.scalar_v16 * (self.scalar_v104 * (v265 * v578)));

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
        let v1: f64 = ctx.node_voltage(nodes[2]);
        let v4: f64 = ((ctx.temperature() + v1) + self.scalar_v3);
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
        let v26: f64 = (v21 - v12);
        let v27: f64 = (self.scalar_v25 * v26);
        let v33: f64 = ((((((v21) as f64).ln() * self.scalar_v23) + (v27 / v20))) as f64).exp();
        let v34: f64 = (self.scalar_v32 * v33);
        let v42: f64 = (self.scalar_v38 * (v12 + (v26 * self.scalar_v39)));
        let v47: f64 = (self.scalar_v43 * (v12 + (v26 * self.scalar_v44)));
        let v54: f64 = 300.15;
        let v56: f64 = (v11 / v54);
        let v58: f64 = 0.000702;
        let v59: f64 = (v11 * v58);
        let v60: f64 = (v11 * v59);
        let v62: f64 = (v11 + 1108.0);
        let v65: f64 = (-(1.16 - (v60 / v62)));
        let v66: f64 = 1.3806226e-23;
        let v68: f64 = (v66 * (v11 + v11));
        let v73: f64 = (-(v20 + v20));
        let v74: f64 = 1.5;
        let v77: f64 = 1.6021918e-19;
        let v79: f64 = ((v74 * ((v56) as f64).ln()) + (((v65 / v68) + 1.3454442398941469e20) * v77));
        let v80: f64 = (v73 * v79);
        let v83: f64 = ((self.scalar_v81 - v80) / self.scalar_v55);
        let v84: f64 = (self.scalar_v81 - v83);
        let v87: f64 = 0.0004;
        let v92: f64 = (v12 + (self.scalar_v86 * (self.scalar_v89 - (v84 / v83))));
        let v93: f64 = (self.scalar_v53 / v92);
        let v95: f64 = (v80 + (v56 * v83));
        let v96: f64 = (v95 - v83);
        let v102: f64 = (v12 + (self.scalar_v86 * ((v87 * (v11 - v54)) - (v96 / v83))));
        let v103: f64 = (v93 * v102);
        let v108: f64 = (self.scalar_v104 * (ctx.node_voltage(nodes[3]) - ctx.node_voltage(nodes[4])));
        let v115: bool = (v34 > v13);
        let v117: f64 = (v20 * self.scalar_v116);
        let v119: f64 = (if v115 { (v108 / v117) } else { v13 });
        let v121: f64 = ((-v108) - v47);
        let v123: f64 = (v20 * self.scalar_v122);
        let v125: f64 = (if v115 { (v121 / v123) } else { v13 });
        let v126: f64 = (-v47);
        let v128: f64 = (if v115 { (v126 / v123) } else { v13 });
        let v129: f64 = 80.0;
        let v130: bool = (v119 > v129);
        let v131: bool = (v115 && v130);
        let v137: bool = (v115 && (!v130));
        let v138: f64 = (if v137 { v12 } else { (if v131 { (v12 + (v119 - v129)) } else { v13 }) });
        let v139: f64 = (((if v131 { v129 } else { v119 })) as f64).exp();
        let v142: f64 = 37.0;
        let v143: bool = (v125 >= v142);
        let v144: bool = (!v143);
        let v145: f64 = -37.0;
        let v146: bool = (v125 <= v145);
        let v148: bool = (v144 && (!v146));
        let v149: f64 = ((v125) as f64).exp();
        let v150: f64 = (v12 + v149);
        let v152: bool = (v144 && v146);
        let v156: bool = (v128 >= v142);
        let v157: bool = (!v156);
        let v158: bool = (v128 <= v145);
        let v160: bool = (v157 && (!v158));
        let v161: f64 = ((v128) as f64).exp();
        let v162: f64 = (v12 + v161);
        let v164: bool = (v157 && v158);
        let v169: f64 = (if v115 { ((if v148 { ((v150) as f64).ln() } else { (if v152 { v149 } else { (if v143 { v125 } else { v13 }) }) }) - (if v160 { ((v162) as f64).ln() } else { (if v164 { v161 } else { (if v156 { v128 } else { v13 }) }) })) } else { v13 });
        let v170: f64 = ((if v115 { (v138 * v139) } else { v138 }) - v12);
        let v172: f64 = (v42 * v169);
        let v174: f64 = ((v108) as f64).abs();
        let v175: f64 = f64::powf(v174, (self.scalar_v48 * (v12 + (v26 * self.scalar_v49))));
        let v177: f64 = (v12 + (self.scalar_v173 * v175));
        let v181: bool = (!v115);
        let v265: f64 = (self.scalar_v261 * (v12 + ((f64::powf((v12 + f64::powf(((((ctx.node_voltage(nodes[0]) - ctx.node_voltage(nodes[1])) / self.scalar_v252)) as f64).abs(), self.scalar_v255)), self.scalar_v258) - v12) * self.scalar_v262)));
        let v281: f64 = (v108 + ((-v95) * self.scalar_v279));
        let v282: bool = (v281 > v13);
        let v288: f64 = (if v282 { self.scalar_v287 } else { v13 });
        let v291: f64 = (v12 - (self.scalar_v284 * (self.scalar_v284 * v288)));
        let v298: f64 = (v281 * self.scalar_v297);
        let v300: f64 = (self.scalar_v284 + (v298 / v95));
        let v304: bool = (!v282);
        let v306: f64 = (v12 - (v108 / v95));
        let v309: f64 = (((self.scalar_v293 * ((v306) as f64).ln())) as f64).exp();
        let v310: f64 = (v12 - v309);
        let v315: f64 = ((if v304 { ((v95 * v310) / self.scalar_v293) } else { (if v282 { ((v95 * v291) / self.scalar_v293) } else { v13 }) }) + (if v304 { v13 } else { (if v282 { (v288 * (v281 * v300)) } else { v13 }) }));
        let v348: f64 = 0.0;
        let v350: f64 = (if self.scalar_v268 { (v265 * v348) } else { v13 });
        let v358: f64 = (v1 * self.scalar_v357);
        let v359: f64 = 0.0;
        let v360: f64 = (if self.scalar_v321 { v359 } else { v13 });
        let v368: f64 = 0.0;
        let v369: f64 = (if self.scalar_v362 { v368 } else { v13 });
        let v374: f64 = 0.0;
        let v375: f64 = (if self.scalar_v362 { v374 } else { v13 });
        let v392: f64 = (self.scalar_v16 * (self.scalar_v104 * (v103 * v315)));
        let v394: f64 = (self.scalar_v16 * (self.scalar_v104 * ((if v181 { v13 } else { (if v115 { ((v34 * v170) - (v172 / v177)) } else { v13 }) }) * v265)));
        let v396: f64 = (if v10 { v13 } else { (if v8 { v12 } else { v13 }) });
        let v397: f64 = (v19 * v396);
        let v398: f64 = (v396 / self.scalar_v18);
        let v419: f64 = (v396 / v54);
        let v433: f64 = ((v68 * (((v62 * ((v59 * v396) + (v11 * (v58 * v396)))) - (v60 * v396)) / (v62 * v62))) - (v65 * (v66 * (v396 + v396))));
        let v444: f64 = ((v79 * (-(v397 + v397))) + (v73 * ((v74 * (v419 / v56)) + (v77 * (v433 / (v68 * v68))))));
        let v446: f64 = ((-v444) / self.scalar_v55);
        let v451: f64 = (v83 * v83);
        let v462: f64 = (v444 + ((v83 * v419) + (v56 * v446)));
        let v473: f64 = ((v102 * ((-(self.scalar_v53 * (self.scalar_v86 * (-(((v83 * (-v446)) - (v84 * v446)) / v451))))) / (v92 * v92))) + (v93 * (self.scalar_v86 * ((v87 * v396) - (((v83 * (v462 - v446)) - (v96 * v446)) / v451)))));
        let v482: f64 = (if v115 { ((-(v108 * (self.scalar_v116 * v397))) / (v117 * v117)) } else { v13 });
        let v483: f64 = (if v115 { (self.scalar_v104 / v117) } else { v13 });
        let v484: f64 = (if v115 { (self.scalar_v474 / v117) } else { v13 });
        let v486: f64 = (self.scalar_v122 * v397);
        let v487: f64 = (v123 * (-(self.scalar_v43 * (self.scalar_v44 * v398))));
        let v490: f64 = (v123 * v123);
        let v494: f64 = (if v115 { ((v487 - (v121 * v486)) / v490) } else { v13 });
        let v495: f64 = (if v115 { (self.scalar_v474 / v123) } else { v13 });
        let v496: f64 = (if v115 { (self.scalar_v104 / v123) } else { v13 });
        let v500: f64 = (if v115 { ((v487 - (v126 * v486)) / v490) } else { v13 });
        let v507: f64 = (if v137 { v13 } else { (if v131 { v482 } else { v13 }) });
        let v508: f64 = (if v137 { v13 } else { (if v131 { v483 } else { v13 }) });
        let v509: f64 = (if v137 { v13 } else { (if v131 { v484 } else { v13 }) });
        let v525: f64 = (v149 * v494);
        let v526: f64 = (v149 * v495);
        let v527: f64 = (v149 * v496);
        let v540: f64 = (v161 * v500);
        let v545: f64 = ((if v148 { (v525 / v150) } else { (if v152 { v525 } else { (if v143 { v494 } else { v13 }) }) }) - (if v160 { (v540 / v162) } else { (if v164 { v540 } else { (if v156 { v500 } else { v13 }) }) }));
        let v549: f64 = (v170 * (self.scalar_v32 * (v33 * ((self.scalar_v23 * (v398 / v21)) + (((v20 * (self.scalar_v25 * v398)) - (v27 * v397)) / (v20 * v20))))));
        let v565: f64 = ((v177 * ((v169 * (self.scalar_v38 * (self.scalar_v39 * v398))) + (v42 * (if v115 { v545 } else { v13 })))) - (v172 * (self.scalar_v173 * ((self.scalar_v48 * (self.scalar_v49 * v398)) * (v175 * ((v174) as f64).ln())))));
        let v570: f64 = ((v549 + (v34 * (if v115 { ((v139 * v507) + (v138 * (v139 * (if v131 { v13 } else { v482 })))) } else { v507 }))) - (v565 / (v177 * v177)));
        let v571: f64 = ((v34 * (if v115 { ((v139 * v508) + (v138 * (v139 * (if v131 { v13 } else { v483 })))) } else { v508 })) - ((v42 * (if v115 { (if v148 { (v526 / v150) } else { (if v152 { v526 } else { (if v143 { v495 } else { v13 }) }) }) } else { v13 })) / v177));
        let v572: f64 = ((v34 * (if v115 { ((v139 * v509) + (v138 * (v139 * (if v131 { v13 } else { v484 })))) } else { v509 })) - ((v42 * (if v115 { (if v148 { (v527 / v150) } else { (if v152 { v527 } else { (if v143 { v496 } else { v13 }) }) }) } else { v13 })) / v177));
        let v656: f64 = (self.scalar_v279 * (-v462));
        let v666: f64 = (v95 * v95);
        let v713: f64 = (if v304 { (((v310 * v462) + (v95 * (-(v309 * (self.scalar_v293 * ((-((-(v108 * v462)) / v666)) / v306)))))) / self.scalar_v293) } else { (if v282 { ((v291 * v462) / self.scalar_v293) } else { v13 }) });
        let v716: f64 = (if v304 { v13 } else { (if v282 { (v288 * ((v300 * v656) + (v281 * (((v95 * (self.scalar_v297 * v656)) - (v298 * v462)) / v666)))) } else { v13 }) });
        let v720: f64 = ((if v304 { ((v95 * (-(v309 * (self.scalar_v293 * ((-(self.scalar_v104 / v95)) / v306))))) / self.scalar_v293) } else { v13 }) + (if v304 { v13 } else { (if v282 { (v288 * ((self.scalar_v104 * v300) + (v281 * (self.scalar_v661 / v95)))) } else { v13 }) }));
        let v721: f64 = ((if v304 { ((v95 * (-(v309 * (self.scalar_v293 * ((-(self.scalar_v474 / v95)) / v306))))) / self.scalar_v293) } else { v13 }) + (if v304 { v13 } else { (if v282 { (v288 * ((v300 * self.scalar_v474) + (v281 * (self.scalar_v662 / v95)))) } else { v13 }) }));
        let v739: f64 = 1.0;
        let v741: f64 = (if self.scalar_v268 { (v265 * v739) } else { v13 });
        let v744: f64 = (self.scalar_v357 * v739);
        let v745: f64 = (if self.scalar_v321 { v744 } else { v13 });
        let v749: f64 = (if self.scalar_v362 { v744 } else { v13 });
        let v753: f64 = (if self.scalar_v362 { (self.scalar_v372 * v739) } else { v13 });
        let v784: f64 = (self.scalar_v16 * (self.scalar_v104 * ((v315 * v473) + (v103 * (v713 + v716)))));
        let v785: f64 = (self.scalar_v16 * (self.scalar_v104 * (v103 * v720)));
        let v786: f64 = (self.scalar_v16 * (self.scalar_v104 * (v103 * v721)));
        let v790: f64 = (self.scalar_v16 * (self.scalar_v104 * (v265 * (if v181 { v13 } else { (if v115 { v570 } else { v13 }) }))));
        let v791: f64 = (self.scalar_v16 * (self.scalar_v104 * (v265 * (if v181 { v13 } else { (if v115 { v571 } else { v13 }) }))));
        let v792: f64 = (self.scalar_v16 * (self.scalar_v104 * (v265 * (if v181 { v13 } else { (if v115 { v572 } else { v13 }) }))));

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
