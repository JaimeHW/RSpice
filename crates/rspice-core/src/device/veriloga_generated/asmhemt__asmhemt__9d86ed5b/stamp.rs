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
#[path = "stamp_blocks_1.rs"]
mod stamp_blocks_1;
#[path = "stamp_blocks_2.rs"]
mod stamp_blocks_2;
#[path = "stamp_blocks_3.rs"]
mod stamp_blocks_3;
#[path = "stamp_blocks_4.rs"]
mod stamp_blocks_4;
#[path = "stamp_blocks_5.rs"]
mod stamp_blocks_5;
#[path = "stamp_blocks_6.rs"]
mod stamp_blocks_6;
#[path = "stamp_blocks_7.rs"]
mod stamp_blocks_7;

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

impl Instance {
    pub fn stamp(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedStamper<'_>) {
        let p = Box::as_ref(&self.params);
        let nodes = &(*self).nodes;
        let branches = &(*self).branches;
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv1 = ctx.node_voltage(nodes[1]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let nv11 = ctx.node_voltage(nodes[11]);
        let nv12 = ctx.node_voltage(nodes[12]);
        let nv13 = ctx.node_voltage(nodes[13]);
        let nv14 = ctx.node_voltage(nodes[14]);
        let param_given = self.param_given.as_ref();
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
        let v0: f64 = 0.0;
        let v1: f64 = 1.0;
        let v15: f64 = nv7;
        let v16: f64 = nv8;
        let v17: f64 = (v15 - v16);
        let v18: f64 = nv9;
        let v19: bool = (v17 < v0);
        let v20: f64 = -1.0;
        let v21: f64 = (if v19 { v20 } else { v1 });
        let v22: f64 = (v17 * v21);
        let v23: f64 = (if v19 { v22 } else { v0 });
        let v24: bool = (!v19);
        let v25: f64 = (if v24 { v17 } else { v23 });
        let v26: f64 = (v25 * v25);
        let v27: f64 = nv0;
        let v28: f64 = nv2;
        let v29: f64 = (v27 - v28);
        let v33: f64 = 2.0;
        let v43: f64 = nv5;
        let v44: f64 = 0.5;
        let v48: f64 = nv6;
        let v52: f64 = nv1;
        let v53: f64 = (v27 - v52);
        let v54: f64 = (if self.scalar_v51 { v53 } else { v0 });
        let v57: f64 = (v54 * self.scalar_v56);
        let v58: f64 = (v1 + v57);
        let v59: f64 = (self.scalar_v55 / v58);
        let v60: f64 = (v54 * v59);
        let v61: f64 = (if self.scalar_v51 { v60 } else { v0 });
        let v64: f64 = (v54 - self.scalar_v63);
        let v65: f64 = (self.scalar_v62 * v64);
        let v66: f64 = (if self.scalar_v51 { v65 } else { v0 });
        let v67: f64 = (v61 + v66);
        let v68: f64 = (v61 - v66);
        let v69: f64 = (v68 * v68);
        let v74: f64 = (v69 + self.scalar_v73);
        let v75: f64 = ((v74) as f64).sqrt();
        let v76: f64 = (v67 + v75);
        let v77: f64 = (v44 * v76);
        let v78: f64 = (if self.scalar_v51 { v77 } else { v0 });
        let v84: f64 = nv11;
        let v85: f64 = nv12;
        let v86: f64 = (v84 - v85);
        let v88: f64 = (v86 / self.scalar_v87);
        let v89: f64 = ((v88) as f64).exp();
        let v90: f64 = (self.scalar_v83 * v89);
        let v91: f64 = (v1 + v90);
        let v92: f64 = (self.scalar_v82 / v91);
        let v93: f64 = (if self.scalar_v81 { v92 } else { v0 });
        let v96: f64 = nv13;
        let v97: f64 = nv14;
        let v98: f64 = (v96 - v97);
        let v100: f64 = (v98 / self.scalar_v99);
        let v101: f64 = ((v100) as f64).exp();
        let v102: f64 = (self.scalar_v95 * v101);
        let v103: f64 = (v1 + v102);
        let v104: f64 = (self.scalar_v94 / v103);
        let v105: f64 = (if self.scalar_v81 { v104 } else { v0 });
        let v195: f64 = nv10;
        let v204: f64 = (if self.scalar_v42 { v26 } else { v0 });
        let v206: f64 = (v43 / self.scalar_v205);
        let v207: f64 = (if self.scalar_v42 { v206 } else { v0 });
        let v210: f64 = (v43 / self.scalar_v209);
        let v211: f64 = (if self.scalar_v47 { v210 } else { v0 });
        let v213: f64 = (v48 / self.scalar_v212);
        let v214: f64 = (if self.scalar_v47 { v213 } else { v0 });
        let v215: f64 = (-v29);
        let v216: f64 = (if self.scalar_v47 { v215 } else { v0 });
        let v219: f64 = (v43 / self.scalar_v218);
        let v220: f64 = (if self.scalar_v51 { v219 } else { v0 });
        let v221: f64 = (-v78);
        let v222: f64 = (if self.scalar_v51 { v221 } else { v0 });
        let v225: f64 = (v86 / v93);
        let v226: f64 = (if self.scalar_v81 { v225 } else { v0 });
        let v227: f64 = (v98 / v105);
        let v228: f64 = (if self.scalar_v81 { v227 } else { v0 });
        let v249: f64 = (v52 - v18);
        let v250: f64 = (self.scalar_v248 * v249);
        let v251: f64 = (if self.scalar_v153 { v250 } else { v0 });
        let v254: f64 = (v52 - v195);
        let v255: f64 = (self.scalar_v253 * v254);
        let v256: f64 = (if self.scalar_v174 { v255 } else { v0 });
        let v258: f64 = (v195 - v18);
        let v259: f64 = (self.scalar_v257 * v258);
        let v260: f64 = (if self.scalar_v174 { v259 } else { v0 });
        let v265: f64 = (-v21);
        let v266: f64 = (if v19 { v21 } else { v0 });
        let v267: f64 = (if v19 { v265 } else { v0 });
        let v268: f64 = (if v24 { v1 } else { v266 });
        let v269: f64 = (if v24 { v20 } else { v267 });
        let v270: f64 = (v25 * v268);
        let v271: f64 = (v270 + v270);
        let v272: f64 = (v25 * v269);
        let v273: f64 = (v272 + v272);
        let v280: f64 = (v58 * v58);
        let v281: f64 = (self.scalar_v279 / v280);
        let v284: f64 = (self.scalar_v283 / v280);
        let v285: f64 = (v59 * self.scalar_v274);
        let v286: f64 = (v54 * v281);
        let v287: f64 = (v285 + v286);
        let v288: f64 = (v59 * self.scalar_v275);
        let v289: f64 = (v54 * v284);
        let v290: f64 = (v288 + v289);
        let v291: f64 = (if self.scalar_v51 { v287 } else { v0 });
        let v292: f64 = (if self.scalar_v51 { v290 } else { v0 });
        let v297: f64 = (v291 + self.scalar_v295);
        let v298: f64 = (v292 + self.scalar_v296);
        let v299: f64 = (v291 - self.scalar_v295);
        let v300: f64 = (v292 - self.scalar_v296);
        let v301: f64 = (v68 * v299);
        let v302: f64 = (v301 + v301);
        let v303: f64 = (v68 * v300);
        let v304: f64 = (v303 + v303);
        let v305: f64 = (v33 * v75);
        let v306: f64 = (v302 / v305);
        let v307: f64 = (v304 / v305);
        let v308: f64 = (v297 + v306);
        let v309: f64 = (v298 + v307);
        let v310: f64 = (v44 * v308);
        let v311: f64 = (v44 * v309);
        let v312: f64 = (if self.scalar_v51 { v310 } else { v0 });
        let v313: f64 = (if self.scalar_v51 { v311 } else { v0 });
        let v316: f64 = (v89 * self.scalar_v314);
        let v317: f64 = (v89 * self.scalar_v315);
        let v318: f64 = (self.scalar_v83 * v316);
        let v319: f64 = (self.scalar_v83 * v317);
        let v320: f64 = (self.scalar_v82 * v318);
        let v321: f64 = (-v320);
        let v322: f64 = (v91 * v91);
        let v323: f64 = (v321 / v322);
        let v324: f64 = (self.scalar_v82 * v319);
        let v325: f64 = (-v324);
        let v326: f64 = (v325 / v322);
        let v327: f64 = (if self.scalar_v81 { v323 } else { v0 });
        let v328: f64 = (if self.scalar_v81 { v326 } else { v0 });
        let v331: f64 = (v101 * self.scalar_v329);
        let v332: f64 = (v101 * self.scalar_v330);
        let v333: f64 = (self.scalar_v95 * v331);
        let v334: f64 = (self.scalar_v95 * v332);
        let v335: f64 = (self.scalar_v94 * v333);
        let v336: f64 = (-v335);
        let v337: f64 = (v103 * v103);
        let v338: f64 = (v336 / v337);
        let v339: f64 = (self.scalar_v94 * v334);
        let v340: f64 = (-v339);
        let v341: f64 = (v340 / v337);
        let v342: f64 = (if self.scalar_v81 { v338 } else { v0 });
        let v343: f64 = (if self.scalar_v81 { v341 } else { v0 });
        let v344: f64 = (if self.scalar_v42 { v271 } else { v0 });
        let v345: f64 = (if self.scalar_v42 { v273 } else { v0 });
        let v356: f64 = (-v312);
        let v357: f64 = (-v313);
        let v358: f64 = (if self.scalar_v51 { v356 } else { v0 });
        let v359: f64 = (if self.scalar_v51 { v357 } else { v0 });
        let v360: f64 = (v86 * v327);
        let v361: f64 = (v93 - v360);
        let v362: f64 = (v93 * v93);
        let v363: f64 = (v361 / v362);
        let v364: f64 = (-v93);
        let v365: f64 = (v86 * v328);
        let v366: f64 = (v364 - v365);
        let v367: f64 = (v366 / v362);
        let v368: f64 = (if self.scalar_v81 { v363 } else { v0 });
        let v369: f64 = (if self.scalar_v81 { v367 } else { v0 });
        let v370: f64 = (v98 * v342);
        let v371: f64 = (v105 - v370);
        let v372: f64 = (v105 * v105);
        let v373: f64 = (v371 / v372);
        let v374: f64 = (-v105);
        let v375: f64 = (v98 * v343);
        let v376: f64 = (v374 - v375);
        let v377: f64 = (v376 / v372);
        let v378: f64 = (if self.scalar_v81 { v373 } else { v0 });
        let v379: f64 = (if self.scalar_v81 { v377 } else { v0 });

        stamper.stamp_potential_branch_local(
            Some(4),
            None,
            0,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            0,
            self.scalar_v202,
        );
        stamper.stamp_potential_branch_local(
            Some(5),
            None,
            1,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            1,
            self.scalar_v203,
        );
        stamper.stamp_potential_branch_local(
            Some(6),
            None,
            2,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            2,
            self.scalar_v203,
        );
        stamper.stamp_potential_branch_local(
            Some(12),
            None,
            3,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            3,
            self.scalar_v203,
        );
        stamper.stamp_potential_branch_local(
            Some(14),
            None,
            4,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            4,
            self.scalar_v203,
        );
        stamper.stamp_potential_branch_local(
            Some(11),
            None,
            5,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            5,
            self.scalar_v203,
        );
        stamper.stamp_potential_branch_local(
            Some(13),
            None,
            6,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            6,
            self.scalar_v203,
        );
        let d204_dn7: f64 = v344;
        let d204_dn8: f64 = v345;
        stamper.stamp_potential_branch_local(
            Some(6),
            None,
            7,
            multiplicity,
        );
        stamper.stamp_potential_node2_local(
            7,
            v204,
            7,
            d204_dn7,
            8,
            d204_dn8,
        );
        let d207_dn5: f64 = self.scalar_v347;
        stamper.stamp_current_node1_local(
            Some(5),
            None,
            multiplicity * (v207),
            5,
            multiplicity * (d207_dn5),
        );
        stamper.stamp_potential_branch_local(
            Some(12),
            None,
            8,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            8,
            self.scalar_v208,
        );
        stamper.stamp_potential_branch_local(
            Some(14),
            None,
            9,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            9,
            self.scalar_v208,
        );
        stamper.stamp_potential_branch_local(
            Some(11),
            None,
            10,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            10,
            self.scalar_v208,
        );
        stamper.stamp_potential_branch_local(
            Some(13),
            None,
            11,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            11,
            self.scalar_v208,
        );
        let d211_dn5: f64 = self.scalar_v349;
        stamper.stamp_current_node1_local(
            Some(5),
            None,
            multiplicity * (v211),
            5,
            multiplicity * (d211_dn5),
        );
        let d214_dn6: f64 = self.scalar_v351;
        stamper.stamp_current_node1_local(
            Some(6),
            None,
            multiplicity * (v214),
            6,
            multiplicity * (d214_dn6),
        );
        let d216_dn0: f64 = self.scalar_v352;
        let d216_dn2: f64 = self.scalar_v353;
        stamper.stamp_current_node2_local(
            Some(6),
            None,
            multiplicity * (v216),
            0,
            multiplicity * (d216_dn0),
            2,
            multiplicity * (d216_dn2),
        );
        stamper.stamp_potential_branch_local(
            Some(12),
            None,
            12,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            12,
            self.scalar_v217,
        );
        stamper.stamp_potential_branch_local(
            Some(14),
            None,
            13,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            13,
            self.scalar_v217,
        );
        stamper.stamp_potential_branch_local(
            Some(11),
            None,
            14,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            14,
            self.scalar_v217,
        );
        stamper.stamp_potential_branch_local(
            Some(13),
            None,
            15,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            15,
            self.scalar_v217,
        );
        let d220_dn5: f64 = self.scalar_v355;
        stamper.stamp_current_node1_local(
            Some(5),
            None,
            multiplicity * (v220),
            5,
            multiplicity * (d220_dn5),
        );
        let d222_dn0: f64 = v358;
        let d222_dn1: f64 = v359;
        stamper.stamp_current_node2_local(
            Some(5),
            None,
            multiplicity * (v222),
            0,
            multiplicity * (d222_dn0),
            1,
            multiplicity * (d222_dn1),
        );
        stamper.stamp_potential_branch_local(
            Some(6),
            None,
            16,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            16,
            self.scalar_v223,
        );
        stamper.stamp_potential_branch_local(
            Some(12),
            None,
            17,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            17,
            self.scalar_v223,
        );
        stamper.stamp_potential_branch_local(
            Some(14),
            None,
            18,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            18,
            self.scalar_v223,
        );
        stamper.stamp_potential_branch_local(
            Some(11),
            None,
            19,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            19,
            self.scalar_v223,
        );
        stamper.stamp_potential_branch_local(
            Some(13),
            None,
            20,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            20,
            self.scalar_v223,
        );
        stamper.stamp_potential_branch_local(
            Some(5),
            None,
            21,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            21,
            self.scalar_v224,
        );
        stamper.stamp_potential_branch_local(
            Some(6),
            None,
            22,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            22,
            self.scalar_v224,
        );
        let d226_dn11: f64 = v368;
        let d226_dn12: f64 = v369;
        stamper.stamp_current_node2_local(
            Some(11),
            Some(12),
            multiplicity * (v226),
            11,
            multiplicity * (d226_dn11),
            12,
            multiplicity * (d226_dn12),
        );
        let d228_dn13: f64 = v378;
        let d228_dn14: f64 = v379;
        stamper.stamp_current_node2_local(
            Some(13),
            Some(14),
            multiplicity * (v228),
            13,
            multiplicity * (d228_dn13),
            14,
            multiplicity * (d228_dn14),
        );
        stamper.stamp_potential_branch_local(
            Some(12),
            None,
            25,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            25,
            self.scalar_v229,
        );
        stamper.stamp_potential_branch_local(
            Some(14),
            None,
            26,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            26,
            self.scalar_v229,
        );
        stamper.stamp_potential_branch_local(
            Some(11),
            None,
            27,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            27,
            self.scalar_v229,
        );
        stamper.stamp_potential_branch_local(
            Some(13),
            None,
            28,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            28,
            self.scalar_v229,
        );
        stamper.stamp_potential_branch_local(
            Some(0),
            Some(18),
            29,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            29,
            self.scalar_v232,
        );
        stamper.stamp_potential_branch_local(
            Some(2),
            Some(22),
            30,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            30,
            self.scalar_v232,
        );
        stamper.stamp_potential_branch_local(
            Some(0),
            Some(7),
            31,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            31,
            self.scalar_v234,
        );
        stamper.stamp_potential_branch_local(
            Some(8),
            Some(2),
            32,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            32,
            self.scalar_v234,
        );
        stamper.stamp_potential_branch_local(
            Some(15),
            Some(7),
            33,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            33,
            self.scalar_v235,
        );
        stamper.stamp_potential_branch_local(
            Some(15),
            Some(7),
            34,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            34,
            self.scalar_v236,
        );
        stamper.stamp_potential_branch_local(
            Some(8),
            Some(19),
            35,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            35,
            self.scalar_v237,
        );
        stamper.stamp_potential_branch_local(
            Some(8),
            Some(19),
            36,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            36,
            self.scalar_v236,
        );
        stamper.stamp_potential_branch_local(
            Some(8),
            Some(19),
            37,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            37,
            self.scalar_v238,
        );
        stamper.stamp_potential_branch_local(
            Some(16),
            Some(15),
            38,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            38,
            self.scalar_v239,
        );
        stamper.stamp_potential_branch_local(
            Some(16),
            Some(7),
            39,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            39,
            self.scalar_v236,
        );
        stamper.stamp_potential_branch_local(
            Some(19),
            Some(20),
            40,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            40,
            self.scalar_v240,
        );
        stamper.stamp_potential_branch_local(
            Some(8),
            Some(20),
            41,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            41,
            self.scalar_v236,
        );
        stamper.stamp_potential_branch_local(
            Some(8),
            Some(20),
            42,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            42,
            self.scalar_v241,
        );
        stamper.stamp_potential_branch_local(
            Some(17),
            Some(16),
            43,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            43,
            self.scalar_v242,
        );
        stamper.stamp_potential_branch_local(
            Some(17),
            Some(7),
            44,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            44,
            self.scalar_v236,
        );
        stamper.stamp_potential_branch_local(
            Some(20),
            Some(21),
            45,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            45,
            self.scalar_v243,
        );
        stamper.stamp_potential_branch_local(
            Some(8),
            Some(21),
            46,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            46,
            self.scalar_v236,
        );
        stamper.stamp_potential_branch_local(
            Some(8),
            Some(21),
            47,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            47,
            self.scalar_v244,
        );
        stamper.stamp_potential_branch_local(
            Some(18),
            Some(17),
            48,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            48,
            self.scalar_v245,
        );
        stamper.stamp_potential_branch_local(
            Some(18),
            Some(7),
            49,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            49,
            self.scalar_v236,
        );
        stamper.stamp_potential_branch_local(
            Some(21),
            Some(22),
            50,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            50,
            self.scalar_v246,
        );
        stamper.stamp_potential_branch_local(
            Some(8),
            Some(22),
            51,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            51,
            self.scalar_v236,
        );
        stamper.stamp_potential_branch_local(
            Some(8),
            Some(22),
            52,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            52,
            self.scalar_v247,
        );
        let d251_dn1: f64 = self.scalar_v381;
        let d251_dn9: f64 = self.scalar_v382;
        stamper.stamp_current_node2_local(
            Some(1),
            Some(9),
            multiplicity * (v251),
            1,
            multiplicity * (d251_dn1),
            9,
            multiplicity * (d251_dn9),
        );
        stamper.stamp_potential_branch_local(
            Some(10),
            Some(9),
            53,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            53,
            self.scalar_v252,
        );
        let d256_dn1: f64 = self.scalar_v384;
        let d256_dn10: f64 = self.scalar_v385;
        stamper.stamp_current_node2_local(
            Some(1),
            Some(10),
            multiplicity * (v256),
            1,
            multiplicity * (d256_dn1),
            10,
            multiplicity * (d256_dn10),
        );
        let d260_dn9: f64 = self.scalar_v387;
        let d260_dn10: f64 = self.scalar_v388;
        stamper.stamp_current_node2_local(
            Some(10),
            Some(9),
            multiplicity * (v260),
            9,
            multiplicity * (d260_dn9),
            10,
            multiplicity * (d260_dn10),
        );
        stamper.stamp_potential_branch_local(
            Some(1),
            Some(10),
            54,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            54,
            self.scalar_v262,
        );
        stamper.stamp_potential_branch_local(
            Some(10),
            Some(9),
            55,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            55,
            self.scalar_v262,
        );
        stamper.stamp_potential_branch_local(
            Some(4),
            None,
            56,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            56,
            self.scalar_v264,
        );
        let s = match &mut self.scratch {
            Some(buf) => buf.as_mut(),
            slot @ None => slot.insert(Scratch::new_box()).as_mut(),
        };

        let mut var_tnom: f64 = 0.0;
        let mut var_tdev: f64 = 0.0;
        let mut var_tdev_dn0: f64 = 0.0;
        let mut var_tdev_dn1: f64 = 0.0;
        let mut var_tdev_dn2: f64 = 0.0;
        let mut var_tdev_dn3: f64 = 0.0;
        let mut var_tdev_dn4: f64 = 0.0;
        let mut var_tdev_dn5: f64 = 0.0;
        let mut var_tdev_dn6: f64 = 0.0;
        let mut var_tdev_dn7: f64 = 0.0;
        let mut var_tdev_dn8: f64 = 0.0;
        let mut var_tdev_dn9: f64 = 0.0;
        let mut var_tdev_dn10: f64 = 0.0;
        let mut var_tdev_dn11: f64 = 0.0;
        let mut var_tdev_dn12: f64 = 0.0;
        let mut var_tdev_dn13: f64 = 0.0;
        let mut var_tdev_dn14: f64 = 0.0;
        let mut var_tdev_dn15: f64 = 0.0;
        let mut var_tdev_dn16: f64 = 0.0;
        let mut var_tdev_dn17: f64 = 0.0;
        let mut var_tdev_dn18: f64 = 0.0;
        let mut var_tdev_dn19: f64 = 0.0;
        let mut var_tdev_dn20: f64 = 0.0;
        let mut var_tdev_dn21: f64 = 0.0;
        let mut var_tdev_dn22: f64 = 0.0;
        let mut var_tdev_db0: f64 = 0.0;
        let mut var_tdev_db1: f64 = 0.0;
        let mut var_tdev_db2: f64 = 0.0;
        let mut var_tdev_db3: f64 = 0.0;
        let mut var_tdev_db4: f64 = 0.0;
        let mut var_tdev_db5: f64 = 0.0;
        let mut var_tdev_db6: f64 = 0.0;
        let mut var_tdev_db7: f64 = 0.0;
        let mut var_tdev_db8: f64 = 0.0;
        let mut var_tdev_db9: f64 = 0.0;
        let mut var_tdev_db10: f64 = 0.0;
        let mut var_tdev_db11: f64 = 0.0;
        let mut var_tdev_db12: f64 = 0.0;
        let mut var_tdev_db13: f64 = 0.0;
        let mut var_tdev_db14: f64 = 0.0;
        let mut var_tdev_db15: f64 = 0.0;
        let mut var_tdev_db16: f64 = 0.0;
        let mut var_tdev_db17: f64 = 0.0;
        let mut var_tdev_db18: f64 = 0.0;
        let mut var_tdev_db19: f64 = 0.0;
        let mut var_tdev_db20: f64 = 0.0;
        let mut var_tdev_db21: f64 = 0.0;
        let mut var_tdev_db22: f64 = 0.0;
        let mut var_tdev_db23: f64 = 0.0;
        let mut var_tdev_db24: f64 = 0.0;
        let mut var_tdev_db25: f64 = 0.0;
        let mut var_tdev_db26: f64 = 0.0;
        let mut var_tdev_db27: f64 = 0.0;
        let mut var_tdev_db28: f64 = 0.0;
        let mut var_tdev_db29: f64 = 0.0;
        let mut var_tdev_db30: f64 = 0.0;
        let mut var_tdev_db31: f64 = 0.0;
        let mut var_tdev_db32: f64 = 0.0;
        let mut var_tdev_db33: f64 = 0.0;
        let mut var_tdev_db34: f64 = 0.0;
        let mut var_tdev_db35: f64 = 0.0;
        let mut var_tdev_db36: f64 = 0.0;
        let mut var_tdev_db37: f64 = 0.0;
        let mut var_tdev_db38: f64 = 0.0;
        let mut var_tdev_db39: f64 = 0.0;
        let mut var_tdev_db40: f64 = 0.0;
        let mut var_tdev_db41: f64 = 0.0;
        let mut var_tdev_db42: f64 = 0.0;
        let mut var_tdev_db43: f64 = 0.0;
        let mut var_tdev_db44: f64 = 0.0;
        let mut var_tdev_db45: f64 = 0.0;
        let mut var_tdev_db46: f64 = 0.0;
        let mut var_tdev_db47: f64 = 0.0;
        let mut var_tdev_db48: f64 = 0.0;
        let mut var_tdev_db49: f64 = 0.0;
        let mut var_tdev_db50: f64 = 0.0;
        let mut var_tdev_db51: f64 = 0.0;
        let mut var_tdev_db52: f64 = 0.0;
        let mut var_tdev_db53: f64 = 0.0;
        let mut var_tdev_db54: f64 = 0.0;
        let mut var_qsov: f64 = 0.0;
        let mut var_qsov_dn0: f64 = 0.0;
        let mut var_qsov_dn1: f64 = 0.0;
        let mut var_qsov_dn2: f64 = 0.0;
        let mut var_qsov_dn3: f64 = 0.0;
        let mut var_qsov_dn4: f64 = 0.0;
        let mut var_qsov_dn5: f64 = 0.0;
        let mut var_qsov_dn6: f64 = 0.0;
        let mut var_qsov_dn7: f64 = 0.0;
        let mut var_qsov_dn8: f64 = 0.0;
        let mut var_qsov_dn9: f64 = 0.0;
        let mut var_qsov_dn10: f64 = 0.0;
        let mut var_qsov_dn11: f64 = 0.0;
        let mut var_qsov_dn12: f64 = 0.0;
        let mut var_qsov_dn13: f64 = 0.0;
        let mut var_qsov_dn14: f64 = 0.0;
        let mut var_qsov_dn15: f64 = 0.0;
        let mut var_qsov_dn16: f64 = 0.0;
        let mut var_qsov_dn17: f64 = 0.0;
        let mut var_qsov_dn18: f64 = 0.0;
        let mut var_qsov_dn19: f64 = 0.0;
        let mut var_qsov_dn20: f64 = 0.0;
        let mut var_qsov_dn21: f64 = 0.0;
        let mut var_qsov_dn22: f64 = 0.0;
        let mut var_qsov_db0: f64 = 0.0;
        let mut var_qsov_db1: f64 = 0.0;
        let mut var_qsov_db2: f64 = 0.0;
        let mut var_qsov_db3: f64 = 0.0;
        let mut var_qsov_db4: f64 = 0.0;
        let mut var_qsov_db5: f64 = 0.0;
        let mut var_qsov_db6: f64 = 0.0;
        let mut var_qsov_db7: f64 = 0.0;
        let mut var_qsov_db8: f64 = 0.0;
        let mut var_qsov_db9: f64 = 0.0;
        let mut var_qsov_db10: f64 = 0.0;
        let mut var_qsov_db11: f64 = 0.0;
        let mut var_qsov_db12: f64 = 0.0;
        let mut var_qsov_db13: f64 = 0.0;
        let mut var_qsov_db14: f64 = 0.0;
        let mut var_qsov_db15: f64 = 0.0;
        let mut var_qsov_db16: f64 = 0.0;
        let mut var_qsov_db17: f64 = 0.0;
        let mut var_qsov_db18: f64 = 0.0;
        let mut var_qsov_db19: f64 = 0.0;
        let mut var_qsov_db20: f64 = 0.0;
        let mut var_qsov_db21: f64 = 0.0;
        let mut var_qsov_db22: f64 = 0.0;
        let mut var_qsov_db23: f64 = 0.0;
        let mut var_qsov_db24: f64 = 0.0;
        let mut var_qsov_db25: f64 = 0.0;
        let mut var_qsov_db26: f64 = 0.0;
        let mut var_qsov_db27: f64 = 0.0;
        let mut var_qsov_db28: f64 = 0.0;
        let mut var_qsov_db29: f64 = 0.0;
        let mut var_qsov_db30: f64 = 0.0;
        let mut var_qsov_db31: f64 = 0.0;
        let mut var_qsov_db32: f64 = 0.0;
        let mut var_qsov_db33: f64 = 0.0;
        let mut var_qsov_db34: f64 = 0.0;
        let mut var_qsov_db35: f64 = 0.0;
        let mut var_qsov_db36: f64 = 0.0;
        let mut var_qsov_db37: f64 = 0.0;
        let mut var_qsov_db38: f64 = 0.0;
        let mut var_qsov_db39: f64 = 0.0;
        let mut var_qsov_db40: f64 = 0.0;
        let mut var_qsov_db41: f64 = 0.0;
        let mut var_qsov_db42: f64 = 0.0;
        let mut var_qsov_db43: f64 = 0.0;
        let mut var_qsov_db44: f64 = 0.0;
        let mut var_qsov_db45: f64 = 0.0;
        let mut var_qsov_db46: f64 = 0.0;
        let mut var_qsov_db47: f64 = 0.0;
        let mut var_qsov_db48: f64 = 0.0;
        let mut var_qsov_db49: f64 = 0.0;
        let mut var_qsov_db50: f64 = 0.0;
        let mut var_qsov_db51: f64 = 0.0;
        let mut var_qsov_db52: f64 = 0.0;
        let mut var_qsov_db53: f64 = 0.0;
        let mut var_qsov_db54: f64 = 0.0;
        let mut var_qdov: f64 = 0.0;
        let mut var_qdov_dn0: f64 = 0.0;
        let mut var_qdov_dn1: f64 = 0.0;
        let mut var_qdov_dn2: f64 = 0.0;
        let mut var_qdov_dn3: f64 = 0.0;
        let mut var_qdov_dn4: f64 = 0.0;
        let mut var_qdov_dn5: f64 = 0.0;
        let mut var_qdov_dn6: f64 = 0.0;
        let mut var_qdov_dn7: f64 = 0.0;
        let mut var_qdov_dn8: f64 = 0.0;
        let mut var_qdov_dn9: f64 = 0.0;
        let mut var_qdov_dn10: f64 = 0.0;
        let mut var_qdov_dn11: f64 = 0.0;
        let mut var_qdov_dn12: f64 = 0.0;
        let mut var_qdov_dn13: f64 = 0.0;
        let mut var_qdov_dn14: f64 = 0.0;
        let mut var_qdov_dn15: f64 = 0.0;
        let mut var_qdov_dn16: f64 = 0.0;
        let mut var_qdov_dn17: f64 = 0.0;
        let mut var_qdov_dn18: f64 = 0.0;
        let mut var_qdov_dn19: f64 = 0.0;
        let mut var_qdov_dn20: f64 = 0.0;
        let mut var_qdov_dn21: f64 = 0.0;
        let mut var_qdov_dn22: f64 = 0.0;
        let mut var_qdov_db0: f64 = 0.0;
        let mut var_qdov_db1: f64 = 0.0;
        let mut var_qdov_db2: f64 = 0.0;
        let mut var_qdov_db3: f64 = 0.0;
        let mut var_qdov_db4: f64 = 0.0;
        let mut var_qdov_db5: f64 = 0.0;
        let mut var_qdov_db6: f64 = 0.0;
        let mut var_qdov_db7: f64 = 0.0;
        let mut var_qdov_db8: f64 = 0.0;
        let mut var_qdov_db9: f64 = 0.0;
        let mut var_qdov_db10: f64 = 0.0;
        let mut var_qdov_db11: f64 = 0.0;
        let mut var_qdov_db12: f64 = 0.0;
        let mut var_qdov_db13: f64 = 0.0;
        let mut var_qdov_db14: f64 = 0.0;
        let mut var_qdov_db15: f64 = 0.0;
        let mut var_qdov_db16: f64 = 0.0;
        let mut var_qdov_db17: f64 = 0.0;
        let mut var_qdov_db18: f64 = 0.0;
        let mut var_qdov_db19: f64 = 0.0;
        let mut var_qdov_db20: f64 = 0.0;
        let mut var_qdov_db21: f64 = 0.0;
        let mut var_qdov_db22: f64 = 0.0;
        let mut var_qdov_db23: f64 = 0.0;
        let mut var_qdov_db24: f64 = 0.0;
        let mut var_qdov_db25: f64 = 0.0;
        let mut var_qdov_db26: f64 = 0.0;
        let mut var_qdov_db27: f64 = 0.0;
        let mut var_qdov_db28: f64 = 0.0;
        let mut var_qdov_db29: f64 = 0.0;
        let mut var_qdov_db30: f64 = 0.0;
        let mut var_qdov_db31: f64 = 0.0;
        let mut var_qdov_db32: f64 = 0.0;
        let mut var_qdov_db33: f64 = 0.0;
        let mut var_qdov_db34: f64 = 0.0;
        let mut var_qdov_db35: f64 = 0.0;
        let mut var_qdov_db36: f64 = 0.0;
        let mut var_qdov_db37: f64 = 0.0;
        let mut var_qdov_db38: f64 = 0.0;
        let mut var_qdov_db39: f64 = 0.0;
        let mut var_qdov_db40: f64 = 0.0;
        let mut var_qdov_db41: f64 = 0.0;
        let mut var_qdov_db42: f64 = 0.0;
        let mut var_qdov_db43: f64 = 0.0;
        let mut var_qdov_db44: f64 = 0.0;
        let mut var_qdov_db45: f64 = 0.0;
        let mut var_qdov_db46: f64 = 0.0;
        let mut var_qdov_db47: f64 = 0.0;
        let mut var_qdov_db48: f64 = 0.0;
        let mut var_qdov_db49: f64 = 0.0;
        let mut var_qdov_db50: f64 = 0.0;
        let mut var_qdov_db51: f64 = 0.0;
        let mut var_qdov_db52: f64 = 0.0;
        let mut var_qdov_db53: f64 = 0.0;
        let mut var_qdov_db54: f64 = 0.0;
        let mut var_qdsov: f64 = 0.0;
        let mut var_qdsov_dn0: f64 = 0.0;
        let mut var_qdsov_dn1: f64 = 0.0;
        let mut var_qdsov_dn2: f64 = 0.0;
        let mut var_qdsov_dn3: f64 = 0.0;
        let mut var_qdsov_dn4: f64 = 0.0;
        let mut var_qdsov_dn5: f64 = 0.0;
        let mut var_qdsov_dn6: f64 = 0.0;
        let mut var_qdsov_dn7: f64 = 0.0;
        let mut var_qdsov_dn8: f64 = 0.0;
        let mut var_qdsov_dn9: f64 = 0.0;
        let mut var_qdsov_dn10: f64 = 0.0;
        let mut var_qdsov_dn11: f64 = 0.0;
        let mut var_qdsov_dn12: f64 = 0.0;
        let mut var_qdsov_dn13: f64 = 0.0;
        let mut var_qdsov_dn14: f64 = 0.0;
        let mut var_qdsov_dn15: f64 = 0.0;
        let mut var_qdsov_dn16: f64 = 0.0;
        let mut var_qdsov_dn17: f64 = 0.0;
        let mut var_qdsov_dn18: f64 = 0.0;
        let mut var_qdsov_dn19: f64 = 0.0;
        let mut var_qdsov_dn20: f64 = 0.0;
        let mut var_qdsov_dn21: f64 = 0.0;
        let mut var_qdsov_dn22: f64 = 0.0;
        let mut var_qdsov_db0: f64 = 0.0;
        let mut var_qdsov_db1: f64 = 0.0;
        let mut var_qdsov_db2: f64 = 0.0;
        let mut var_qdsov_db3: f64 = 0.0;
        let mut var_qdsov_db4: f64 = 0.0;
        let mut var_qdsov_db5: f64 = 0.0;
        let mut var_qdsov_db6: f64 = 0.0;
        let mut var_qdsov_db7: f64 = 0.0;
        let mut var_qdsov_db8: f64 = 0.0;
        let mut var_qdsov_db9: f64 = 0.0;
        let mut var_qdsov_db10: f64 = 0.0;
        let mut var_qdsov_db11: f64 = 0.0;
        let mut var_qdsov_db12: f64 = 0.0;
        let mut var_qdsov_db13: f64 = 0.0;
        let mut var_qdsov_db14: f64 = 0.0;
        let mut var_qdsov_db15: f64 = 0.0;
        let mut var_qdsov_db16: f64 = 0.0;
        let mut var_qdsov_db17: f64 = 0.0;
        let mut var_qdsov_db18: f64 = 0.0;
        let mut var_qdsov_db19: f64 = 0.0;
        let mut var_qdsov_db20: f64 = 0.0;
        let mut var_qdsov_db21: f64 = 0.0;
        let mut var_qdsov_db22: f64 = 0.0;
        let mut var_qdsov_db23: f64 = 0.0;
        let mut var_qdsov_db24: f64 = 0.0;
        let mut var_qdsov_db25: f64 = 0.0;
        let mut var_qdsov_db26: f64 = 0.0;
        let mut var_qdsov_db27: f64 = 0.0;
        let mut var_qdsov_db28: f64 = 0.0;
        let mut var_qdsov_db29: f64 = 0.0;
        let mut var_qdsov_db30: f64 = 0.0;
        let mut var_qdsov_db31: f64 = 0.0;
        let mut var_qdsov_db32: f64 = 0.0;
        let mut var_qdsov_db33: f64 = 0.0;
        let mut var_qdsov_db34: f64 = 0.0;
        let mut var_qdsov_db35: f64 = 0.0;
        let mut var_qdsov_db36: f64 = 0.0;
        let mut var_qdsov_db37: f64 = 0.0;
        let mut var_qdsov_db38: f64 = 0.0;
        let mut var_qdsov_db39: f64 = 0.0;
        let mut var_qdsov_db40: f64 = 0.0;
        let mut var_qdsov_db41: f64 = 0.0;
        let mut var_qdsov_db42: f64 = 0.0;
        let mut var_qdsov_db43: f64 = 0.0;
        let mut var_qdsov_db44: f64 = 0.0;
        let mut var_qdsov_db45: f64 = 0.0;
        let mut var_qdsov_db46: f64 = 0.0;
        let mut var_qdsov_db47: f64 = 0.0;
        let mut var_qdsov_db48: f64 = 0.0;
        let mut var_qdsov_db49: f64 = 0.0;
        let mut var_qdsov_db50: f64 = 0.0;
        let mut var_qdsov_db51: f64 = 0.0;
        let mut var_qdsov_db52: f64 = 0.0;
        let mut var_qdsov_db53: f64 = 0.0;
        let mut var_qdsov_db54: f64 = 0.0;
        let mut var_cgdvar: f64 = 0.0;
        let mut var_cgdvar_dn0: f64 = 0.0;
        let mut var_cgdvar_dn1: f64 = 0.0;
        let mut var_cgdvar_dn2: f64 = 0.0;
        let mut var_cgdvar_dn3: f64 = 0.0;
        let mut var_cgdvar_dn4: f64 = 0.0;
        let mut var_cgdvar_dn5: f64 = 0.0;
        let mut var_cgdvar_dn6: f64 = 0.0;
        let mut var_cgdvar_dn7: f64 = 0.0;
        let mut var_cgdvar_dn8: f64 = 0.0;
        let mut var_cgdvar_dn9: f64 = 0.0;
        let mut var_cgdvar_dn10: f64 = 0.0;
        let mut var_cgdvar_dn11: f64 = 0.0;
        let mut var_cgdvar_dn12: f64 = 0.0;
        let mut var_cgdvar_dn13: f64 = 0.0;
        let mut var_cgdvar_dn14: f64 = 0.0;
        let mut var_cgdvar_dn15: f64 = 0.0;
        let mut var_cgdvar_dn16: f64 = 0.0;
        let mut var_cgdvar_dn17: f64 = 0.0;
        let mut var_cgdvar_dn18: f64 = 0.0;
        let mut var_cgdvar_dn19: f64 = 0.0;
        let mut var_cgdvar_dn20: f64 = 0.0;
        let mut var_cgdvar_dn21: f64 = 0.0;
        let mut var_cgdvar_dn22: f64 = 0.0;
        let mut var_cgdvar_db0: f64 = 0.0;
        let mut var_cgdvar_db1: f64 = 0.0;
        let mut var_cgdvar_db2: f64 = 0.0;
        let mut var_cgdvar_db3: f64 = 0.0;
        let mut var_cgdvar_db4: f64 = 0.0;
        let mut var_cgdvar_db5: f64 = 0.0;
        let mut var_cgdvar_db6: f64 = 0.0;
        let mut var_cgdvar_db7: f64 = 0.0;
        let mut var_cgdvar_db8: f64 = 0.0;
        let mut var_cgdvar_db9: f64 = 0.0;
        let mut var_cgdvar_db10: f64 = 0.0;
        let mut var_cgdvar_db11: f64 = 0.0;
        let mut var_cgdvar_db12: f64 = 0.0;
        let mut var_cgdvar_db13: f64 = 0.0;
        let mut var_cgdvar_db14: f64 = 0.0;
        let mut var_cgdvar_db15: f64 = 0.0;
        let mut var_cgdvar_db16: f64 = 0.0;
        let mut var_cgdvar_db17: f64 = 0.0;
        let mut var_cgdvar_db18: f64 = 0.0;
        let mut var_cgdvar_db19: f64 = 0.0;
        let mut var_cgdvar_db20: f64 = 0.0;
        let mut var_cgdvar_db21: f64 = 0.0;
        let mut var_cgdvar_db22: f64 = 0.0;
        let mut var_cgdvar_db23: f64 = 0.0;
        let mut var_cgdvar_db24: f64 = 0.0;
        let mut var_cgdvar_db25: f64 = 0.0;
        let mut var_cgdvar_db26: f64 = 0.0;
        let mut var_cgdvar_db27: f64 = 0.0;
        let mut var_cgdvar_db28: f64 = 0.0;
        let mut var_cgdvar_db29: f64 = 0.0;
        let mut var_cgdvar_db30: f64 = 0.0;
        let mut var_cgdvar_db31: f64 = 0.0;
        let mut var_cgdvar_db32: f64 = 0.0;
        let mut var_cgdvar_db33: f64 = 0.0;
        let mut var_cgdvar_db34: f64 = 0.0;
        let mut var_cgdvar_db35: f64 = 0.0;
        let mut var_cgdvar_db36: f64 = 0.0;
        let mut var_cgdvar_db37: f64 = 0.0;
        let mut var_cgdvar_db38: f64 = 0.0;
        let mut var_cgdvar_db39: f64 = 0.0;
        let mut var_cgdvar_db40: f64 = 0.0;
        let mut var_cgdvar_db41: f64 = 0.0;
        let mut var_cgdvar_db42: f64 = 0.0;
        let mut var_cgdvar_db43: f64 = 0.0;
        let mut var_cgdvar_db44: f64 = 0.0;
        let mut var_cgdvar_db45: f64 = 0.0;
        let mut var_cgdvar_db46: f64 = 0.0;
        let mut var_cgdvar_db47: f64 = 0.0;
        let mut var_cgdvar_db48: f64 = 0.0;
        let mut var_cgdvar_db49: f64 = 0.0;
        let mut var_cgdvar_db50: f64 = 0.0;
        let mut var_cgdvar_db51: f64 = 0.0;
        let mut var_cgdvar_db52: f64 = 0.0;
        let mut var_cgdvar_db53: f64 = 0.0;
        let mut var_cgdvar_db54: f64 = 0.0;
        let mut var_vdseffcv: f64 = 0.0;
        let mut var_vdseffcv_dn0: f64 = 0.0;
        let mut var_vdseffcv_dn1: f64 = 0.0;
        let mut var_vdseffcv_dn2: f64 = 0.0;
        let mut var_vdseffcv_dn3: f64 = 0.0;
        let mut var_vdseffcv_dn4: f64 = 0.0;
        let mut var_vdseffcv_dn5: f64 = 0.0;
        let mut var_vdseffcv_dn6: f64 = 0.0;
        let mut var_vdseffcv_dn7: f64 = 0.0;
        let mut var_vdseffcv_dn8: f64 = 0.0;
        let mut var_vdseffcv_dn9: f64 = 0.0;
        let mut var_vdseffcv_dn10: f64 = 0.0;
        let mut var_vdseffcv_dn11: f64 = 0.0;
        let mut var_vdseffcv_dn12: f64 = 0.0;
        let mut var_vdseffcv_dn13: f64 = 0.0;
        let mut var_vdseffcv_dn14: f64 = 0.0;
        let mut var_vdseffcv_dn15: f64 = 0.0;
        let mut var_vdseffcv_dn16: f64 = 0.0;
        let mut var_vdseffcv_dn17: f64 = 0.0;
        let mut var_vdseffcv_dn18: f64 = 0.0;
        let mut var_vdseffcv_dn19: f64 = 0.0;
        let mut var_vdseffcv_dn20: f64 = 0.0;
        let mut var_vdseffcv_dn21: f64 = 0.0;
        let mut var_vdseffcv_dn22: f64 = 0.0;
        let mut var_vdseffcv_db0: f64 = 0.0;
        let mut var_vdseffcv_db1: f64 = 0.0;
        let mut var_vdseffcv_db2: f64 = 0.0;
        let mut var_vdseffcv_db3: f64 = 0.0;
        let mut var_vdseffcv_db4: f64 = 0.0;
        let mut var_vdseffcv_db5: f64 = 0.0;
        let mut var_vdseffcv_db6: f64 = 0.0;
        let mut var_vdseffcv_db7: f64 = 0.0;
        let mut var_vdseffcv_db8: f64 = 0.0;
        let mut var_vdseffcv_db9: f64 = 0.0;
        let mut var_vdseffcv_db10: f64 = 0.0;
        let mut var_vdseffcv_db11: f64 = 0.0;
        let mut var_vdseffcv_db12: f64 = 0.0;
        let mut var_vdseffcv_db13: f64 = 0.0;
        let mut var_vdseffcv_db14: f64 = 0.0;
        let mut var_vdseffcv_db15: f64 = 0.0;
        let mut var_vdseffcv_db16: f64 = 0.0;
        let mut var_vdseffcv_db17: f64 = 0.0;
        let mut var_vdseffcv_db18: f64 = 0.0;
        let mut var_vdseffcv_db19: f64 = 0.0;
        let mut var_vdseffcv_db20: f64 = 0.0;
        let mut var_vdseffcv_db21: f64 = 0.0;
        let mut var_vdseffcv_db22: f64 = 0.0;
        let mut var_vdseffcv_db23: f64 = 0.0;
        let mut var_vdseffcv_db24: f64 = 0.0;
        let mut var_vdseffcv_db25: f64 = 0.0;
        let mut var_vdseffcv_db26: f64 = 0.0;
        let mut var_vdseffcv_db27: f64 = 0.0;
        let mut var_vdseffcv_db28: f64 = 0.0;
        let mut var_vdseffcv_db29: f64 = 0.0;
        let mut var_vdseffcv_db30: f64 = 0.0;
        let mut var_vdseffcv_db31: f64 = 0.0;
        let mut var_vdseffcv_db32: f64 = 0.0;
        let mut var_vdseffcv_db33: f64 = 0.0;
        let mut var_vdseffcv_db34: f64 = 0.0;
        let mut var_vdseffcv_db35: f64 = 0.0;
        let mut var_vdseffcv_db36: f64 = 0.0;
        let mut var_vdseffcv_db37: f64 = 0.0;
        let mut var_vdseffcv_db38: f64 = 0.0;
        let mut var_vdseffcv_db39: f64 = 0.0;
        let mut var_vdseffcv_db40: f64 = 0.0;
        let mut var_vdseffcv_db41: f64 = 0.0;
        let mut var_vdseffcv_db42: f64 = 0.0;
        let mut var_vdseffcv_db43: f64 = 0.0;
        let mut var_vdseffcv_db44: f64 = 0.0;
        let mut var_vdseffcv_db45: f64 = 0.0;
        let mut var_vdseffcv_db46: f64 = 0.0;
        let mut var_vdseffcv_db47: f64 = 0.0;
        let mut var_vdseffcv_db48: f64 = 0.0;
        let mut var_vdseffcv_db49: f64 = 0.0;
        let mut var_vdseffcv_db50: f64 = 0.0;
        let mut var_vdseffcv_db51: f64 = 0.0;
        let mut var_vdseffcv_db52: f64 = 0.0;
        let mut var_vdseffcv_db53: f64 = 0.0;
        let mut var_vdseffcv_db54: f64 = 0.0;
        let mut var_cgdl_l: f64 = 0.0;
        let mut var_qfr: f64 = 0.0;
        let mut var_qfr_dn0: f64 = 0.0;
        let mut var_qfr_dn1: f64 = 0.0;
        let mut var_qfr_dn2: f64 = 0.0;
        let mut var_qfr_dn3: f64 = 0.0;
        let mut var_qfr_dn4: f64 = 0.0;
        let mut var_qfr_dn5: f64 = 0.0;
        let mut var_qfr_dn6: f64 = 0.0;
        let mut var_qfr_dn7: f64 = 0.0;
        let mut var_qfr_dn8: f64 = 0.0;
        let mut var_qfr_dn9: f64 = 0.0;
        let mut var_qfr_dn10: f64 = 0.0;
        let mut var_qfr_dn11: f64 = 0.0;
        let mut var_qfr_dn12: f64 = 0.0;
        let mut var_qfr_dn13: f64 = 0.0;
        let mut var_qfr_dn14: f64 = 0.0;
        let mut var_qfr_dn15: f64 = 0.0;
        let mut var_qfr_dn16: f64 = 0.0;
        let mut var_qfr_dn17: f64 = 0.0;
        let mut var_qfr_dn18: f64 = 0.0;
        let mut var_qfr_dn19: f64 = 0.0;
        let mut var_qfr_dn20: f64 = 0.0;
        let mut var_qfr_dn21: f64 = 0.0;
        let mut var_qfr_dn22: f64 = 0.0;
        let mut var_qfr_db0: f64 = 0.0;
        let mut var_qfr_db1: f64 = 0.0;
        let mut var_qfr_db2: f64 = 0.0;
        let mut var_qfr_db3: f64 = 0.0;
        let mut var_qfr_db4: f64 = 0.0;
        let mut var_qfr_db5: f64 = 0.0;
        let mut var_qfr_db6: f64 = 0.0;
        let mut var_qfr_db7: f64 = 0.0;
        let mut var_qfr_db8: f64 = 0.0;
        let mut var_qfr_db9: f64 = 0.0;
        let mut var_qfr_db10: f64 = 0.0;
        let mut var_qfr_db11: f64 = 0.0;
        let mut var_qfr_db12: f64 = 0.0;
        let mut var_qfr_db13: f64 = 0.0;
        let mut var_qfr_db14: f64 = 0.0;
        let mut var_qfr_db15: f64 = 0.0;
        let mut var_qfr_db16: f64 = 0.0;
        let mut var_qfr_db17: f64 = 0.0;
        let mut var_qfr_db18: f64 = 0.0;
        let mut var_qfr_db19: f64 = 0.0;
        let mut var_qfr_db20: f64 = 0.0;
        let mut var_qfr_db21: f64 = 0.0;
        let mut var_qfr_db22: f64 = 0.0;
        let mut var_qfr_db23: f64 = 0.0;
        let mut var_qfr_db24: f64 = 0.0;
        let mut var_qfr_db25: f64 = 0.0;
        let mut var_qfr_db26: f64 = 0.0;
        let mut var_qfr_db27: f64 = 0.0;
        let mut var_qfr_db28: f64 = 0.0;
        let mut var_qfr_db29: f64 = 0.0;
        let mut var_qfr_db30: f64 = 0.0;
        let mut var_qfr_db31: f64 = 0.0;
        let mut var_qfr_db32: f64 = 0.0;
        let mut var_qfr_db33: f64 = 0.0;
        let mut var_qfr_db34: f64 = 0.0;
        let mut var_qfr_db35: f64 = 0.0;
        let mut var_qfr_db36: f64 = 0.0;
        let mut var_qfr_db37: f64 = 0.0;
        let mut var_qfr_db38: f64 = 0.0;
        let mut var_qfr_db39: f64 = 0.0;
        let mut var_qfr_db40: f64 = 0.0;
        let mut var_qfr_db41: f64 = 0.0;
        let mut var_qfr_db42: f64 = 0.0;
        let mut var_qfr_db43: f64 = 0.0;
        let mut var_qfr_db44: f64 = 0.0;
        let mut var_qfr_db45: f64 = 0.0;
        let mut var_qfr_db46: f64 = 0.0;
        let mut var_qfr_db47: f64 = 0.0;
        let mut var_qfr_db48: f64 = 0.0;
        let mut var_qfr_db49: f64 = 0.0;
        let mut var_qfr_db50: f64 = 0.0;
        let mut var_qfr_db51: f64 = 0.0;
        let mut var_qfr_db52: f64 = 0.0;
        let mut var_qfr_db53: f64 = 0.0;
        let mut var_qfr_db54: f64 = 0.0;
        let mut var_qfr3: f64 = 0.0;
        let mut var_qfr3_dn0: f64 = 0.0;
        let mut var_qfr3_dn1: f64 = 0.0;
        let mut var_qfr3_dn2: f64 = 0.0;
        let mut var_qfr3_dn3: f64 = 0.0;
        let mut var_qfr3_dn4: f64 = 0.0;
        let mut var_qfr3_dn5: f64 = 0.0;
        let mut var_qfr3_dn6: f64 = 0.0;
        let mut var_qfr3_dn7: f64 = 0.0;
        let mut var_qfr3_dn8: f64 = 0.0;
        let mut var_qfr3_dn9: f64 = 0.0;
        let mut var_qfr3_dn10: f64 = 0.0;
        let mut var_qfr3_dn11: f64 = 0.0;
        let mut var_qfr3_dn12: f64 = 0.0;
        let mut var_qfr3_dn13: f64 = 0.0;
        let mut var_qfr3_dn14: f64 = 0.0;
        let mut var_qfr3_dn15: f64 = 0.0;
        let mut var_qfr3_dn16: f64 = 0.0;
        let mut var_qfr3_dn17: f64 = 0.0;
        let mut var_qfr3_dn18: f64 = 0.0;
        let mut var_qfr3_dn19: f64 = 0.0;
        let mut var_qfr3_dn20: f64 = 0.0;
        let mut var_qfr3_dn21: f64 = 0.0;
        let mut var_qfr3_dn22: f64 = 0.0;
        let mut var_qfr3_db0: f64 = 0.0;
        let mut var_qfr3_db1: f64 = 0.0;
        let mut var_qfr3_db2: f64 = 0.0;
        let mut var_qfr3_db3: f64 = 0.0;
        let mut var_qfr3_db4: f64 = 0.0;
        let mut var_qfr3_db5: f64 = 0.0;
        let mut var_qfr3_db6: f64 = 0.0;
        let mut var_qfr3_db7: f64 = 0.0;
        let mut var_qfr3_db8: f64 = 0.0;
        let mut var_qfr3_db9: f64 = 0.0;
        let mut var_qfr3_db10: f64 = 0.0;
        let mut var_qfr3_db11: f64 = 0.0;
        let mut var_qfr3_db12: f64 = 0.0;
        let mut var_qfr3_db13: f64 = 0.0;
        let mut var_qfr3_db14: f64 = 0.0;
        let mut var_qfr3_db15: f64 = 0.0;
        let mut var_qfr3_db16: f64 = 0.0;
        let mut var_qfr3_db17: f64 = 0.0;
        let mut var_qfr3_db18: f64 = 0.0;
        let mut var_qfr3_db19: f64 = 0.0;
        let mut var_qfr3_db20: f64 = 0.0;
        let mut var_qfr3_db21: f64 = 0.0;
        let mut var_qfr3_db22: f64 = 0.0;
        let mut var_qfr3_db23: f64 = 0.0;
        let mut var_qfr3_db24: f64 = 0.0;
        let mut var_qfr3_db25: f64 = 0.0;
        let mut var_qfr3_db26: f64 = 0.0;
        let mut var_qfr3_db27: f64 = 0.0;
        let mut var_qfr3_db28: f64 = 0.0;
        let mut var_qfr3_db29: f64 = 0.0;
        let mut var_qfr3_db30: f64 = 0.0;
        let mut var_qfr3_db31: f64 = 0.0;
        let mut var_qfr3_db32: f64 = 0.0;
        let mut var_qfr3_db33: f64 = 0.0;
        let mut var_qfr3_db34: f64 = 0.0;
        let mut var_qfr3_db35: f64 = 0.0;
        let mut var_qfr3_db36: f64 = 0.0;
        let mut var_qfr3_db37: f64 = 0.0;
        let mut var_qfr3_db38: f64 = 0.0;
        let mut var_qfr3_db39: f64 = 0.0;
        let mut var_qfr3_db40: f64 = 0.0;
        let mut var_qfr3_db41: f64 = 0.0;
        let mut var_qfr3_db42: f64 = 0.0;
        let mut var_qfr3_db43: f64 = 0.0;
        let mut var_qfr3_db44: f64 = 0.0;
        let mut var_qfr3_db45: f64 = 0.0;
        let mut var_qfr3_db46: f64 = 0.0;
        let mut var_qfr3_db47: f64 = 0.0;
        let mut var_qfr3_db48: f64 = 0.0;
        let mut var_qfr3_db49: f64 = 0.0;
        let mut var_qfr3_db50: f64 = 0.0;
        let mut var_qfr3_db51: f64 = 0.0;
        let mut var_qfr3_db52: f64 = 0.0;
        let mut var_qfr3_db53: f64 = 0.0;
        let mut var_qfr3_db54: f64 = 0.0;
        let mut var_vdgeff1: f64 = 0.0;
        let mut var_vdgeff1_dn0: f64 = 0.0;
        let mut var_vdgeff1_dn1: f64 = 0.0;
        let mut var_vdgeff1_dn2: f64 = 0.0;
        let mut var_vdgeff1_dn3: f64 = 0.0;
        let mut var_vdgeff1_dn4: f64 = 0.0;
        let mut var_vdgeff1_dn5: f64 = 0.0;
        let mut var_vdgeff1_dn6: f64 = 0.0;
        let mut var_vdgeff1_dn7: f64 = 0.0;
        let mut var_vdgeff1_dn8: f64 = 0.0;
        let mut var_vdgeff1_dn9: f64 = 0.0;
        let mut var_vdgeff1_dn10: f64 = 0.0;
        let mut var_vdgeff1_dn11: f64 = 0.0;
        let mut var_vdgeff1_dn12: f64 = 0.0;
        let mut var_vdgeff1_dn13: f64 = 0.0;
        let mut var_vdgeff1_dn14: f64 = 0.0;
        let mut var_vdgeff1_dn15: f64 = 0.0;
        let mut var_vdgeff1_dn16: f64 = 0.0;
        let mut var_vdgeff1_dn17: f64 = 0.0;
        let mut var_vdgeff1_dn18: f64 = 0.0;
        let mut var_vdgeff1_dn19: f64 = 0.0;
        let mut var_vdgeff1_dn20: f64 = 0.0;
        let mut var_vdgeff1_dn21: f64 = 0.0;
        let mut var_vdgeff1_dn22: f64 = 0.0;
        let mut var_vdgeff1_db0: f64 = 0.0;
        let mut var_vdgeff1_db1: f64 = 0.0;
        let mut var_vdgeff1_db2: f64 = 0.0;
        let mut var_vdgeff1_db3: f64 = 0.0;
        let mut var_vdgeff1_db4: f64 = 0.0;
        let mut var_vdgeff1_db5: f64 = 0.0;
        let mut var_vdgeff1_db6: f64 = 0.0;
        let mut var_vdgeff1_db7: f64 = 0.0;
        let mut var_vdgeff1_db8: f64 = 0.0;
        let mut var_vdgeff1_db9: f64 = 0.0;
        let mut var_vdgeff1_db10: f64 = 0.0;
        let mut var_vdgeff1_db11: f64 = 0.0;
        let mut var_vdgeff1_db12: f64 = 0.0;
        let mut var_vdgeff1_db13: f64 = 0.0;
        let mut var_vdgeff1_db14: f64 = 0.0;
        let mut var_vdgeff1_db15: f64 = 0.0;
        let mut var_vdgeff1_db16: f64 = 0.0;
        let mut var_vdgeff1_db17: f64 = 0.0;
        let mut var_vdgeff1_db18: f64 = 0.0;
        let mut var_vdgeff1_db19: f64 = 0.0;
        let mut var_vdgeff1_db20: f64 = 0.0;
        let mut var_vdgeff1_db21: f64 = 0.0;
        let mut var_vdgeff1_db22: f64 = 0.0;
        let mut var_vdgeff1_db23: f64 = 0.0;
        let mut var_vdgeff1_db24: f64 = 0.0;
        let mut var_vdgeff1_db25: f64 = 0.0;
        let mut var_vdgeff1_db26: f64 = 0.0;
        let mut var_vdgeff1_db27: f64 = 0.0;
        let mut var_vdgeff1_db28: f64 = 0.0;
        let mut var_vdgeff1_db29: f64 = 0.0;
        let mut var_vdgeff1_db30: f64 = 0.0;
        let mut var_vdgeff1_db31: f64 = 0.0;
        let mut var_vdgeff1_db32: f64 = 0.0;
        let mut var_vdgeff1_db33: f64 = 0.0;
        let mut var_vdgeff1_db34: f64 = 0.0;
        let mut var_vdgeff1_db35: f64 = 0.0;
        let mut var_vdgeff1_db36: f64 = 0.0;
        let mut var_vdgeff1_db37: f64 = 0.0;
        let mut var_vdgeff1_db38: f64 = 0.0;
        let mut var_vdgeff1_db39: f64 = 0.0;
        let mut var_vdgeff1_db40: f64 = 0.0;
        let mut var_vdgeff1_db41: f64 = 0.0;
        let mut var_vdgeff1_db42: f64 = 0.0;
        let mut var_vdgeff1_db43: f64 = 0.0;
        let mut var_vdgeff1_db44: f64 = 0.0;
        let mut var_vdgeff1_db45: f64 = 0.0;
        let mut var_vdgeff1_db46: f64 = 0.0;
        let mut var_vdgeff1_db47: f64 = 0.0;
        let mut var_vdgeff1_db48: f64 = 0.0;
        let mut var_vdgeff1_db49: f64 = 0.0;
        let mut var_vdgeff1_db50: f64 = 0.0;
        let mut var_vdgeff1_db51: f64 = 0.0;
        let mut var_vdgeff1_db52: f64 = 0.0;
        let mut var_vdgeff1_db53: f64 = 0.0;
        let mut var_vdgeff1_db54: f64 = 0.0;
        let mut var_qbdov: f64 = 0.0;
        let mut var_qbdov_dn0: f64 = 0.0;
        let mut var_qbdov_dn1: f64 = 0.0;
        let mut var_qbdov_dn2: f64 = 0.0;
        let mut var_qbdov_dn3: f64 = 0.0;
        let mut var_qbdov_dn4: f64 = 0.0;
        let mut var_qbdov_dn5: f64 = 0.0;
        let mut var_qbdov_dn6: f64 = 0.0;
        let mut var_qbdov_dn7: f64 = 0.0;
        let mut var_qbdov_dn8: f64 = 0.0;
        let mut var_qbdov_dn9: f64 = 0.0;
        let mut var_qbdov_dn10: f64 = 0.0;
        let mut var_qbdov_dn11: f64 = 0.0;
        let mut var_qbdov_dn12: f64 = 0.0;
        let mut var_qbdov_dn13: f64 = 0.0;
        let mut var_qbdov_dn14: f64 = 0.0;
        let mut var_qbdov_dn15: f64 = 0.0;
        let mut var_qbdov_dn16: f64 = 0.0;
        let mut var_qbdov_dn17: f64 = 0.0;
        let mut var_qbdov_dn18: f64 = 0.0;
        let mut var_qbdov_dn19: f64 = 0.0;
        let mut var_qbdov_dn20: f64 = 0.0;
        let mut var_qbdov_dn21: f64 = 0.0;
        let mut var_qbdov_dn22: f64 = 0.0;
        let mut var_qbdov_db0: f64 = 0.0;
        let mut var_qbdov_db1: f64 = 0.0;
        let mut var_qbdov_db2: f64 = 0.0;
        let mut var_qbdov_db3: f64 = 0.0;
        let mut var_qbdov_db4: f64 = 0.0;
        let mut var_qbdov_db5: f64 = 0.0;
        let mut var_qbdov_db6: f64 = 0.0;
        let mut var_qbdov_db7: f64 = 0.0;
        let mut var_qbdov_db8: f64 = 0.0;
        let mut var_qbdov_db9: f64 = 0.0;
        let mut var_qbdov_db10: f64 = 0.0;
        let mut var_qbdov_db11: f64 = 0.0;
        let mut var_qbdov_db12: f64 = 0.0;
        let mut var_qbdov_db13: f64 = 0.0;
        let mut var_qbdov_db14: f64 = 0.0;
        let mut var_qbdov_db15: f64 = 0.0;
        let mut var_qbdov_db16: f64 = 0.0;
        let mut var_qbdov_db17: f64 = 0.0;
        let mut var_qbdov_db18: f64 = 0.0;
        let mut var_qbdov_db19: f64 = 0.0;
        let mut var_qbdov_db20: f64 = 0.0;
        let mut var_qbdov_db21: f64 = 0.0;
        let mut var_qbdov_db22: f64 = 0.0;
        let mut var_qbdov_db23: f64 = 0.0;
        let mut var_qbdov_db24: f64 = 0.0;
        let mut var_qbdov_db25: f64 = 0.0;
        let mut var_qbdov_db26: f64 = 0.0;
        let mut var_qbdov_db27: f64 = 0.0;
        let mut var_qbdov_db28: f64 = 0.0;
        let mut var_qbdov_db29: f64 = 0.0;
        let mut var_qbdov_db30: f64 = 0.0;
        let mut var_qbdov_db31: f64 = 0.0;
        let mut var_qbdov_db32: f64 = 0.0;
        let mut var_qbdov_db33: f64 = 0.0;
        let mut var_qbdov_db34: f64 = 0.0;
        let mut var_qbdov_db35: f64 = 0.0;
        let mut var_qbdov_db36: f64 = 0.0;
        let mut var_qbdov_db37: f64 = 0.0;
        let mut var_qbdov_db38: f64 = 0.0;
        let mut var_qbdov_db39: f64 = 0.0;
        let mut var_qbdov_db40: f64 = 0.0;
        let mut var_qbdov_db41: f64 = 0.0;
        let mut var_qbdov_db42: f64 = 0.0;
        let mut var_qbdov_db43: f64 = 0.0;
        let mut var_qbdov_db44: f64 = 0.0;
        let mut var_qbdov_db45: f64 = 0.0;
        let mut var_qbdov_db46: f64 = 0.0;
        let mut var_qbdov_db47: f64 = 0.0;
        let mut var_qbdov_db48: f64 = 0.0;
        let mut var_qbdov_db49: f64 = 0.0;
        let mut var_qbdov_db50: f64 = 0.0;
        let mut var_qbdov_db51: f64 = 0.0;
        let mut var_qbdov_db52: f64 = 0.0;
        let mut var_qbdov_db53: f64 = 0.0;
        let mut var_qbdov_db54: f64 = 0.0;
        let mut var_qbsov: f64 = 0.0;
        let mut var_qbsov_dn0: f64 = 0.0;
        let mut var_qbsov_dn1: f64 = 0.0;
        let mut var_qbsov_dn2: f64 = 0.0;
        let mut var_qbsov_dn3: f64 = 0.0;
        let mut var_qbsov_dn4: f64 = 0.0;
        let mut var_qbsov_dn5: f64 = 0.0;
        let mut var_qbsov_dn6: f64 = 0.0;
        let mut var_qbsov_dn7: f64 = 0.0;
        let mut var_qbsov_dn8: f64 = 0.0;
        let mut var_qbsov_dn9: f64 = 0.0;
        let mut var_qbsov_dn10: f64 = 0.0;
        let mut var_qbsov_dn11: f64 = 0.0;
        let mut var_qbsov_dn12: f64 = 0.0;
        let mut var_qbsov_dn13: f64 = 0.0;
        let mut var_qbsov_dn14: f64 = 0.0;
        let mut var_qbsov_dn15: f64 = 0.0;
        let mut var_qbsov_dn16: f64 = 0.0;
        let mut var_qbsov_dn17: f64 = 0.0;
        let mut var_qbsov_dn18: f64 = 0.0;
        let mut var_qbsov_dn19: f64 = 0.0;
        let mut var_qbsov_dn20: f64 = 0.0;
        let mut var_qbsov_dn21: f64 = 0.0;
        let mut var_qbsov_dn22: f64 = 0.0;
        let mut var_qbsov_db0: f64 = 0.0;
        let mut var_qbsov_db1: f64 = 0.0;
        let mut var_qbsov_db2: f64 = 0.0;
        let mut var_qbsov_db3: f64 = 0.0;
        let mut var_qbsov_db4: f64 = 0.0;
        let mut var_qbsov_db5: f64 = 0.0;
        let mut var_qbsov_db6: f64 = 0.0;
        let mut var_qbsov_db7: f64 = 0.0;
        let mut var_qbsov_db8: f64 = 0.0;
        let mut var_qbsov_db9: f64 = 0.0;
        let mut var_qbsov_db10: f64 = 0.0;
        let mut var_qbsov_db11: f64 = 0.0;
        let mut var_qbsov_db12: f64 = 0.0;
        let mut var_qbsov_db13: f64 = 0.0;
        let mut var_qbsov_db14: f64 = 0.0;
        let mut var_qbsov_db15: f64 = 0.0;
        let mut var_qbsov_db16: f64 = 0.0;
        let mut var_qbsov_db17: f64 = 0.0;
        let mut var_qbsov_db18: f64 = 0.0;
        let mut var_qbsov_db19: f64 = 0.0;
        let mut var_qbsov_db20: f64 = 0.0;
        let mut var_qbsov_db21: f64 = 0.0;
        let mut var_qbsov_db22: f64 = 0.0;
        let mut var_qbsov_db23: f64 = 0.0;
        let mut var_qbsov_db24: f64 = 0.0;
        let mut var_qbsov_db25: f64 = 0.0;
        let mut var_qbsov_db26: f64 = 0.0;
        let mut var_qbsov_db27: f64 = 0.0;
        let mut var_qbsov_db28: f64 = 0.0;
        let mut var_qbsov_db29: f64 = 0.0;
        let mut var_qbsov_db30: f64 = 0.0;
        let mut var_qbsov_db31: f64 = 0.0;
        let mut var_qbsov_db32: f64 = 0.0;
        let mut var_qbsov_db33: f64 = 0.0;
        let mut var_qbsov_db34: f64 = 0.0;
        let mut var_qbsov_db35: f64 = 0.0;
        let mut var_qbsov_db36: f64 = 0.0;
        let mut var_qbsov_db37: f64 = 0.0;
        let mut var_qbsov_db38: f64 = 0.0;
        let mut var_qbsov_db39: f64 = 0.0;
        let mut var_qbsov_db40: f64 = 0.0;
        let mut var_qbsov_db41: f64 = 0.0;
        let mut var_qbsov_db42: f64 = 0.0;
        let mut var_qbsov_db43: f64 = 0.0;
        let mut var_qbsov_db44: f64 = 0.0;
        let mut var_qbsov_db45: f64 = 0.0;
        let mut var_qbsov_db46: f64 = 0.0;
        let mut var_qbsov_db47: f64 = 0.0;
        let mut var_qbsov_db48: f64 = 0.0;
        let mut var_qbsov_db49: f64 = 0.0;
        let mut var_qbsov_db50: f64 = 0.0;
        let mut var_qbsov_db51: f64 = 0.0;
        let mut var_qbsov_db52: f64 = 0.0;
        let mut var_qbsov_db53: f64 = 0.0;
        let mut var_qbsov_db54: f64 = 0.0;
        let mut var_qbgov: f64 = 0.0;
        let mut var_qbgov_dn0: f64 = 0.0;
        let mut var_qbgov_dn1: f64 = 0.0;
        let mut var_qbgov_dn2: f64 = 0.0;
        let mut var_qbgov_dn3: f64 = 0.0;
        let mut var_qbgov_dn4: f64 = 0.0;
        let mut var_qbgov_dn5: f64 = 0.0;
        let mut var_qbgov_dn6: f64 = 0.0;
        let mut var_qbgov_dn7: f64 = 0.0;
        let mut var_qbgov_dn8: f64 = 0.0;
        let mut var_qbgov_dn9: f64 = 0.0;
        let mut var_qbgov_dn10: f64 = 0.0;
        let mut var_qbgov_dn11: f64 = 0.0;
        let mut var_qbgov_dn12: f64 = 0.0;
        let mut var_qbgov_dn13: f64 = 0.0;
        let mut var_qbgov_dn14: f64 = 0.0;
        let mut var_qbgov_dn15: f64 = 0.0;
        let mut var_qbgov_dn16: f64 = 0.0;
        let mut var_qbgov_dn17: f64 = 0.0;
        let mut var_qbgov_dn18: f64 = 0.0;
        let mut var_qbgov_dn19: f64 = 0.0;
        let mut var_qbgov_dn20: f64 = 0.0;
        let mut var_qbgov_dn21: f64 = 0.0;
        let mut var_qbgov_dn22: f64 = 0.0;
        let mut var_qbgov_db0: f64 = 0.0;
        let mut var_qbgov_db1: f64 = 0.0;
        let mut var_qbgov_db2: f64 = 0.0;
        let mut var_qbgov_db3: f64 = 0.0;
        let mut var_qbgov_db4: f64 = 0.0;
        let mut var_qbgov_db5: f64 = 0.0;
        let mut var_qbgov_db6: f64 = 0.0;
        let mut var_qbgov_db7: f64 = 0.0;
        let mut var_qbgov_db8: f64 = 0.0;
        let mut var_qbgov_db9: f64 = 0.0;
        let mut var_qbgov_db10: f64 = 0.0;
        let mut var_qbgov_db11: f64 = 0.0;
        let mut var_qbgov_db12: f64 = 0.0;
        let mut var_qbgov_db13: f64 = 0.0;
        let mut var_qbgov_db14: f64 = 0.0;
        let mut var_qbgov_db15: f64 = 0.0;
        let mut var_qbgov_db16: f64 = 0.0;
        let mut var_qbgov_db17: f64 = 0.0;
        let mut var_qbgov_db18: f64 = 0.0;
        let mut var_qbgov_db19: f64 = 0.0;
        let mut var_qbgov_db20: f64 = 0.0;
        let mut var_qbgov_db21: f64 = 0.0;
        let mut var_qbgov_db22: f64 = 0.0;
        let mut var_qbgov_db23: f64 = 0.0;
        let mut var_qbgov_db24: f64 = 0.0;
        let mut var_qbgov_db25: f64 = 0.0;
        let mut var_qbgov_db26: f64 = 0.0;
        let mut var_qbgov_db27: f64 = 0.0;
        let mut var_qbgov_db28: f64 = 0.0;
        let mut var_qbgov_db29: f64 = 0.0;
        let mut var_qbgov_db30: f64 = 0.0;
        let mut var_qbgov_db31: f64 = 0.0;
        let mut var_qbgov_db32: f64 = 0.0;
        let mut var_qbgov_db33: f64 = 0.0;
        let mut var_qbgov_db34: f64 = 0.0;
        let mut var_qbgov_db35: f64 = 0.0;
        let mut var_qbgov_db36: f64 = 0.0;
        let mut var_qbgov_db37: f64 = 0.0;
        let mut var_qbgov_db38: f64 = 0.0;
        let mut var_qbgov_db39: f64 = 0.0;
        let mut var_qbgov_db40: f64 = 0.0;
        let mut var_qbgov_db41: f64 = 0.0;
        let mut var_qbgov_db42: f64 = 0.0;
        let mut var_qbgov_db43: f64 = 0.0;
        let mut var_qbgov_db44: f64 = 0.0;
        let mut var_qbgov_db45: f64 = 0.0;
        let mut var_qbgov_db46: f64 = 0.0;
        let mut var_qbgov_db47: f64 = 0.0;
        let mut var_qbgov_db48: f64 = 0.0;
        let mut var_qbgov_db49: f64 = 0.0;
        let mut var_qbgov_db50: f64 = 0.0;
        let mut var_qbgov_db51: f64 = 0.0;
        let mut var_qbgov_db52: f64 = 0.0;
        let mut var_qbgov_db53: f64 = 0.0;
        let mut var_qbgov_db54: f64 = 0.0;
        let mut var_phixn: f64 = 0.0;
        let mut var_phixn_dn0: f64 = 0.0;
        let mut var_phixn_dn1: f64 = 0.0;
        let mut var_phixn_dn2: f64 = 0.0;
        let mut var_phixn_dn3: f64 = 0.0;
        let mut var_phixn_dn4: f64 = 0.0;
        let mut var_phixn_dn5: f64 = 0.0;
        let mut var_phixn_dn6: f64 = 0.0;
        let mut var_phixn_dn7: f64 = 0.0;
        let mut var_phixn_dn8: f64 = 0.0;
        let mut var_phixn_dn9: f64 = 0.0;
        let mut var_phixn_dn10: f64 = 0.0;
        let mut var_phixn_dn11: f64 = 0.0;
        let mut var_phixn_dn12: f64 = 0.0;
        let mut var_phixn_dn13: f64 = 0.0;
        let mut var_phixn_dn14: f64 = 0.0;
        let mut var_phixn_dn15: f64 = 0.0;
        let mut var_phixn_dn16: f64 = 0.0;
        let mut var_phixn_dn17: f64 = 0.0;
        let mut var_phixn_dn18: f64 = 0.0;
        let mut var_phixn_dn19: f64 = 0.0;
        let mut var_phixn_dn20: f64 = 0.0;
        let mut var_phixn_dn21: f64 = 0.0;
        let mut var_phixn_dn22: f64 = 0.0;
        let mut var_phixn_db0: f64 = 0.0;
        let mut var_phixn_db1: f64 = 0.0;
        let mut var_phixn_db2: f64 = 0.0;
        let mut var_phixn_db3: f64 = 0.0;
        let mut var_phixn_db4: f64 = 0.0;
        let mut var_phixn_db5: f64 = 0.0;
        let mut var_phixn_db6: f64 = 0.0;
        let mut var_phixn_db7: f64 = 0.0;
        let mut var_phixn_db8: f64 = 0.0;
        let mut var_phixn_db9: f64 = 0.0;
        let mut var_phixn_db10: f64 = 0.0;
        let mut var_phixn_db11: f64 = 0.0;
        let mut var_phixn_db12: f64 = 0.0;
        let mut var_phixn_db13: f64 = 0.0;
        let mut var_phixn_db14: f64 = 0.0;
        let mut var_phixn_db15: f64 = 0.0;
        let mut var_phixn_db16: f64 = 0.0;
        let mut var_phixn_db17: f64 = 0.0;
        let mut var_phixn_db18: f64 = 0.0;
        let mut var_phixn_db19: f64 = 0.0;
        let mut var_phixn_db20: f64 = 0.0;
        let mut var_phixn_db21: f64 = 0.0;
        let mut var_phixn_db22: f64 = 0.0;
        let mut var_phixn_db23: f64 = 0.0;
        let mut var_phixn_db24: f64 = 0.0;
        let mut var_phixn_db25: f64 = 0.0;
        let mut var_phixn_db26: f64 = 0.0;
        let mut var_phixn_db27: f64 = 0.0;
        let mut var_phixn_db28: f64 = 0.0;
        let mut var_phixn_db29: f64 = 0.0;
        let mut var_phixn_db30: f64 = 0.0;
        let mut var_phixn_db31: f64 = 0.0;
        let mut var_phixn_db32: f64 = 0.0;
        let mut var_phixn_db33: f64 = 0.0;
        let mut var_phixn_db34: f64 = 0.0;
        let mut var_phixn_db35: f64 = 0.0;
        let mut var_phixn_db36: f64 = 0.0;
        let mut var_phixn_db37: f64 = 0.0;
        let mut var_phixn_db38: f64 = 0.0;
        let mut var_phixn_db39: f64 = 0.0;
        let mut var_phixn_db40: f64 = 0.0;
        let mut var_phixn_db41: f64 = 0.0;
        let mut var_phixn_db42: f64 = 0.0;
        let mut var_phixn_db43: f64 = 0.0;
        let mut var_phixn_db44: f64 = 0.0;
        let mut var_phixn_db45: f64 = 0.0;
        let mut var_phixn_db46: f64 = 0.0;
        let mut var_phixn_db47: f64 = 0.0;
        let mut var_phixn_db48: f64 = 0.0;
        let mut var_phixn_db49: f64 = 0.0;
        let mut var_phixn_db50: f64 = 0.0;
        let mut var_phixn_db51: f64 = 0.0;
        let mut var_phixn_db52: f64 = 0.0;
        let mut var_phixn_db53: f64 = 0.0;
        let mut var_phixn_db54: f64 = 0.0;
        let mut var_en: f64 = 0.0;
        let mut var_en_dn0: f64 = 0.0;
        let mut var_en_dn1: f64 = 0.0;
        let mut var_en_dn2: f64 = 0.0;
        let mut var_en_dn3: f64 = 0.0;
        let mut var_en_dn4: f64 = 0.0;
        let mut var_en_dn5: f64 = 0.0;
        let mut var_en_dn6: f64 = 0.0;
        let mut var_en_dn7: f64 = 0.0;
        let mut var_en_dn8: f64 = 0.0;
        let mut var_en_dn9: f64 = 0.0;
        let mut var_en_dn10: f64 = 0.0;
        let mut var_en_dn11: f64 = 0.0;
        let mut var_en_dn12: f64 = 0.0;
        let mut var_en_dn13: f64 = 0.0;
        let mut var_en_dn14: f64 = 0.0;
        let mut var_en_dn15: f64 = 0.0;
        let mut var_en_dn16: f64 = 0.0;
        let mut var_en_dn17: f64 = 0.0;
        let mut var_en_dn18: f64 = 0.0;
        let mut var_en_dn19: f64 = 0.0;
        let mut var_en_dn20: f64 = 0.0;
        let mut var_en_dn21: f64 = 0.0;
        let mut var_en_dn22: f64 = 0.0;
        let mut var_en_db0: f64 = 0.0;
        let mut var_en_db1: f64 = 0.0;
        let mut var_en_db2: f64 = 0.0;
        let mut var_en_db3: f64 = 0.0;
        let mut var_en_db4: f64 = 0.0;
        let mut var_en_db5: f64 = 0.0;
        let mut var_en_db6: f64 = 0.0;
        let mut var_en_db7: f64 = 0.0;
        let mut var_en_db8: f64 = 0.0;
        let mut var_en_db9: f64 = 0.0;
        let mut var_en_db10: f64 = 0.0;
        let mut var_en_db11: f64 = 0.0;
        let mut var_en_db12: f64 = 0.0;
        let mut var_en_db13: f64 = 0.0;
        let mut var_en_db14: f64 = 0.0;
        let mut var_en_db15: f64 = 0.0;
        let mut var_en_db16: f64 = 0.0;
        let mut var_en_db17: f64 = 0.0;
        let mut var_en_db18: f64 = 0.0;
        let mut var_en_db19: f64 = 0.0;
        let mut var_en_db20: f64 = 0.0;
        let mut var_en_db21: f64 = 0.0;
        let mut var_en_db22: f64 = 0.0;
        let mut var_en_db23: f64 = 0.0;
        let mut var_en_db24: f64 = 0.0;
        let mut var_en_db25: f64 = 0.0;
        let mut var_en_db26: f64 = 0.0;
        let mut var_en_db27: f64 = 0.0;
        let mut var_en_db28: f64 = 0.0;
        let mut var_en_db29: f64 = 0.0;
        let mut var_en_db30: f64 = 0.0;
        let mut var_en_db31: f64 = 0.0;
        let mut var_en_db32: f64 = 0.0;
        let mut var_en_db33: f64 = 0.0;
        let mut var_en_db34: f64 = 0.0;
        let mut var_en_db35: f64 = 0.0;
        let mut var_en_db36: f64 = 0.0;
        let mut var_en_db37: f64 = 0.0;
        let mut var_en_db38: f64 = 0.0;
        let mut var_en_db39: f64 = 0.0;
        let mut var_en_db40: f64 = 0.0;
        let mut var_en_db41: f64 = 0.0;
        let mut var_en_db42: f64 = 0.0;
        let mut var_en_db43: f64 = 0.0;
        let mut var_en_db44: f64 = 0.0;
        let mut var_en_db45: f64 = 0.0;
        let mut var_en_db46: f64 = 0.0;
        let mut var_en_db47: f64 = 0.0;
        let mut var_en_db48: f64 = 0.0;
        let mut var_en_db49: f64 = 0.0;
        let mut var_en_db50: f64 = 0.0;
        let mut var_en_db51: f64 = 0.0;
        let mut var_en_db52: f64 = 0.0;
        let mut var_en_db53: f64 = 0.0;
        let mut var_en_db54: f64 = 0.0;
        let mut var_en1: f64 = 0.0;
        let mut var_en1_dn0: f64 = 0.0;
        let mut var_en1_dn1: f64 = 0.0;
        let mut var_en1_dn2: f64 = 0.0;
        let mut var_en1_dn3: f64 = 0.0;
        let mut var_en1_dn4: f64 = 0.0;
        let mut var_en1_dn5: f64 = 0.0;
        let mut var_en1_dn6: f64 = 0.0;
        let mut var_en1_dn7: f64 = 0.0;
        let mut var_en1_dn8: f64 = 0.0;
        let mut var_en1_dn9: f64 = 0.0;
        let mut var_en1_dn10: f64 = 0.0;
        let mut var_en1_dn11: f64 = 0.0;
        let mut var_en1_dn12: f64 = 0.0;
        let mut var_en1_dn13: f64 = 0.0;
        let mut var_en1_dn14: f64 = 0.0;
        let mut var_en1_dn15: f64 = 0.0;
        let mut var_en1_dn16: f64 = 0.0;
        let mut var_en1_dn17: f64 = 0.0;
        let mut var_en1_dn18: f64 = 0.0;
        let mut var_en1_dn19: f64 = 0.0;
        let mut var_en1_dn20: f64 = 0.0;
        let mut var_en1_dn21: f64 = 0.0;
        let mut var_en1_dn22: f64 = 0.0;
        let mut var_en1_db0: f64 = 0.0;
        let mut var_en1_db1: f64 = 0.0;
        let mut var_en1_db2: f64 = 0.0;
        let mut var_en1_db3: f64 = 0.0;
        let mut var_en1_db4: f64 = 0.0;
        let mut var_en1_db5: f64 = 0.0;
        let mut var_en1_db6: f64 = 0.0;
        let mut var_en1_db7: f64 = 0.0;
        let mut var_en1_db8: f64 = 0.0;
        let mut var_en1_db9: f64 = 0.0;
        let mut var_en1_db10: f64 = 0.0;
        let mut var_en1_db11: f64 = 0.0;
        let mut var_en1_db12: f64 = 0.0;
        let mut var_en1_db13: f64 = 0.0;
        let mut var_en1_db14: f64 = 0.0;
        let mut var_en1_db15: f64 = 0.0;
        let mut var_en1_db16: f64 = 0.0;
        let mut var_en1_db17: f64 = 0.0;
        let mut var_en1_db18: f64 = 0.0;
        let mut var_en1_db19: f64 = 0.0;
        let mut var_en1_db20: f64 = 0.0;
        let mut var_en1_db21: f64 = 0.0;
        let mut var_en1_db22: f64 = 0.0;
        let mut var_en1_db23: f64 = 0.0;
        let mut var_en1_db24: f64 = 0.0;
        let mut var_en1_db25: f64 = 0.0;
        let mut var_en1_db26: f64 = 0.0;
        let mut var_en1_db27: f64 = 0.0;
        let mut var_en1_db28: f64 = 0.0;
        let mut var_en1_db29: f64 = 0.0;
        let mut var_en1_db30: f64 = 0.0;
        let mut var_en1_db31: f64 = 0.0;
        let mut var_en1_db32: f64 = 0.0;
        let mut var_en1_db33: f64 = 0.0;
        let mut var_en1_db34: f64 = 0.0;
        let mut var_en1_db35: f64 = 0.0;
        let mut var_en1_db36: f64 = 0.0;
        let mut var_en1_db37: f64 = 0.0;
        let mut var_en1_db38: f64 = 0.0;
        let mut var_en1_db39: f64 = 0.0;
        let mut var_en1_db40: f64 = 0.0;
        let mut var_en1_db41: f64 = 0.0;
        let mut var_en1_db42: f64 = 0.0;
        let mut var_en1_db43: f64 = 0.0;
        let mut var_en1_db44: f64 = 0.0;
        let mut var_en1_db45: f64 = 0.0;
        let mut var_en1_db46: f64 = 0.0;
        let mut var_en1_db47: f64 = 0.0;
        let mut var_en1_db48: f64 = 0.0;
        let mut var_en1_db49: f64 = 0.0;
        let mut var_en1_db50: f64 = 0.0;
        let mut var_en1_db51: f64 = 0.0;
        let mut var_en1_db52: f64 = 0.0;
        let mut var_en1_db53: f64 = 0.0;
        let mut var_en1_db54: f64 = 0.0;
        let mut var_phiyn: f64 = 0.0;
        let mut var_phiyn_dn0: f64 = 0.0;
        let mut var_phiyn_dn1: f64 = 0.0;
        let mut var_phiyn_dn2: f64 = 0.0;
        let mut var_phiyn_dn3: f64 = 0.0;
        let mut var_phiyn_dn4: f64 = 0.0;
        let mut var_phiyn_dn5: f64 = 0.0;
        let mut var_phiyn_dn6: f64 = 0.0;
        let mut var_phiyn_dn7: f64 = 0.0;
        let mut var_phiyn_dn8: f64 = 0.0;
        let mut var_phiyn_dn9: f64 = 0.0;
        let mut var_phiyn_dn10: f64 = 0.0;
        let mut var_phiyn_dn11: f64 = 0.0;
        let mut var_phiyn_dn12: f64 = 0.0;
        let mut var_phiyn_dn13: f64 = 0.0;
        let mut var_phiyn_dn14: f64 = 0.0;
        let mut var_phiyn_dn15: f64 = 0.0;
        let mut var_phiyn_dn16: f64 = 0.0;
        let mut var_phiyn_dn17: f64 = 0.0;
        let mut var_phiyn_dn18: f64 = 0.0;
        let mut var_phiyn_dn19: f64 = 0.0;
        let mut var_phiyn_dn20: f64 = 0.0;
        let mut var_phiyn_dn21: f64 = 0.0;
        let mut var_phiyn_dn22: f64 = 0.0;
        let mut var_phiyn_db0: f64 = 0.0;
        let mut var_phiyn_db1: f64 = 0.0;
        let mut var_phiyn_db2: f64 = 0.0;
        let mut var_phiyn_db3: f64 = 0.0;
        let mut var_phiyn_db4: f64 = 0.0;
        let mut var_phiyn_db5: f64 = 0.0;
        let mut var_phiyn_db6: f64 = 0.0;
        let mut var_phiyn_db7: f64 = 0.0;
        let mut var_phiyn_db8: f64 = 0.0;
        let mut var_phiyn_db9: f64 = 0.0;
        let mut var_phiyn_db10: f64 = 0.0;
        let mut var_phiyn_db11: f64 = 0.0;
        let mut var_phiyn_db12: f64 = 0.0;
        let mut var_phiyn_db13: f64 = 0.0;
        let mut var_phiyn_db14: f64 = 0.0;
        let mut var_phiyn_db15: f64 = 0.0;
        let mut var_phiyn_db16: f64 = 0.0;
        let mut var_phiyn_db17: f64 = 0.0;
        let mut var_phiyn_db18: f64 = 0.0;
        let mut var_phiyn_db19: f64 = 0.0;
        let mut var_phiyn_db20: f64 = 0.0;
        let mut var_phiyn_db21: f64 = 0.0;
        let mut var_phiyn_db22: f64 = 0.0;
        let mut var_phiyn_db23: f64 = 0.0;
        let mut var_phiyn_db24: f64 = 0.0;
        let mut var_phiyn_db25: f64 = 0.0;
        let mut var_phiyn_db26: f64 = 0.0;
        let mut var_phiyn_db27: f64 = 0.0;
        let mut var_phiyn_db28: f64 = 0.0;
        let mut var_phiyn_db29: f64 = 0.0;
        let mut var_phiyn_db30: f64 = 0.0;
        let mut var_phiyn_db31: f64 = 0.0;
        let mut var_phiyn_db32: f64 = 0.0;
        let mut var_phiyn_db33: f64 = 0.0;
        let mut var_phiyn_db34: f64 = 0.0;
        let mut var_phiyn_db35: f64 = 0.0;
        let mut var_phiyn_db36: f64 = 0.0;
        let mut var_phiyn_db37: f64 = 0.0;
        let mut var_phiyn_db38: f64 = 0.0;
        let mut var_phiyn_db39: f64 = 0.0;
        let mut var_phiyn_db40: f64 = 0.0;
        let mut var_phiyn_db41: f64 = 0.0;
        let mut var_phiyn_db42: f64 = 0.0;
        let mut var_phiyn_db43: f64 = 0.0;
        let mut var_phiyn_db44: f64 = 0.0;
        let mut var_phiyn_db45: f64 = 0.0;
        let mut var_phiyn_db46: f64 = 0.0;
        let mut var_phiyn_db47: f64 = 0.0;
        let mut var_phiyn_db48: f64 = 0.0;
        let mut var_phiyn_db49: f64 = 0.0;
        let mut var_phiyn_db50: f64 = 0.0;
        let mut var_phiyn_db51: f64 = 0.0;
        let mut var_phiyn_db52: f64 = 0.0;
        let mut var_phiyn_db53: f64 = 0.0;
        let mut var_phiyn_db54: f64 = 0.0;
        let mut var_guard353: f64 = 0.0;
        let mut var_guard354: f64 = 0.0;
        let mut var_guard355: f64 = 0.0;
        let mut var_guard356: f64 = 0.0;
        let mut var_guard357: f64 = 0.0;
        let mut var_guard358: f64 = 0.0;
        let mut var_guard524: f64 = 0.0;
        let mut var_guard535: f64 = 0.0;
        let mut var_guard576: f64 = 0.0;

        Self::stamp_transient_block_0(ctx, s, p, nodes, &mut var_qfr, &mut var_qfr3, &mut var_qfr3_db0, &mut var_qfr3_db1, &mut var_qfr3_db10, &mut var_qfr3_db11, &mut var_qfr3_db12, &mut var_qfr3_db13, &mut var_qfr3_db14, &mut var_qfr3_db15, &mut var_qfr3_db16, &mut var_qfr3_db17, &mut var_qfr3_db18, &mut var_qfr3_db19, &mut var_qfr3_db2, &mut var_qfr3_db20, &mut var_qfr3_db21, &mut var_qfr3_db22, &mut var_qfr3_db23, &mut var_qfr3_db24, &mut var_qfr3_db25, &mut var_qfr3_db26, &mut var_qfr3_db27, &mut var_qfr3_db28, &mut var_qfr3_db29, &mut var_qfr3_db3, &mut var_qfr3_db30, &mut var_qfr3_db31, &mut var_qfr3_db32, &mut var_qfr3_db33, &mut var_qfr3_db34, &mut var_qfr3_db35, &mut var_qfr3_db36, &mut var_qfr3_db37, &mut var_qfr3_db38, &mut var_qfr3_db39, &mut var_qfr3_db4, &mut var_qfr3_db40, &mut var_qfr3_db41, &mut var_qfr3_db42, &mut var_qfr3_db43, &mut var_qfr3_db44, &mut var_qfr3_db45, &mut var_qfr3_db46, &mut var_qfr3_db47, &mut var_qfr3_db48, &mut var_qfr3_db49, &mut var_qfr3_db5, &mut var_qfr3_db50, &mut var_qfr3_db51, &mut var_qfr3_db52, &mut var_qfr3_db53, &mut var_qfr3_db54, &mut var_qfr3_db6, &mut var_qfr3_db7, &mut var_qfr3_db8, &mut var_qfr3_db9, &mut var_qfr3_dn0, &mut var_qfr3_dn1, &mut var_qfr3_dn10, &mut var_qfr3_dn11, &mut var_qfr3_dn12, &mut var_qfr3_dn13, &mut var_qfr3_dn14, &mut var_qfr3_dn15, &mut var_qfr3_dn16, &mut var_qfr3_dn17, &mut var_qfr3_dn18, &mut var_qfr3_dn19, &mut var_qfr3_dn2, &mut var_qfr3_dn20, &mut var_qfr3_dn21, &mut var_qfr3_dn22, &mut var_qfr3_dn3, &mut var_qfr3_dn4, &mut var_qfr3_dn5, &mut var_qfr3_dn6, &mut var_qfr3_dn7, &mut var_qfr3_dn8, &mut var_qfr3_dn9, &mut var_qfr_db0, &mut var_qfr_db1, &mut var_qfr_db10, &mut var_qfr_db11, &mut var_qfr_db12, &mut var_qfr_db13, &mut var_qfr_db14, &mut var_qfr_db15, &mut var_qfr_db16, &mut var_qfr_db17, &mut var_qfr_db18, &mut var_qfr_db19, &mut var_qfr_db2, &mut var_qfr_db20, &mut var_qfr_db21, &mut var_qfr_db22, &mut var_qfr_db23, &mut var_qfr_db24, &mut var_qfr_db25, &mut var_qfr_db26, &mut var_qfr_db27, &mut var_qfr_db28, &mut var_qfr_db29, &mut var_qfr_db3, &mut var_qfr_db30, &mut var_qfr_db31, &mut var_qfr_db32, &mut var_qfr_db33, &mut var_qfr_db34, &mut var_qfr_db35, &mut var_qfr_db36, &mut var_qfr_db37, &mut var_qfr_db38, &mut var_qfr_db39, &mut var_qfr_db4, &mut var_qfr_db40, &mut var_qfr_db41, &mut var_qfr_db42, &mut var_qfr_db43, &mut var_qfr_db44, &mut var_qfr_db45, &mut var_qfr_db46, &mut var_qfr_db47, &mut var_qfr_db48, &mut var_qfr_db49, &mut var_qfr_db5, &mut var_qfr_db50, &mut var_qfr_db51, &mut var_qfr_db52, &mut var_qfr_db53, &mut var_qfr_db54, &mut var_qfr_db6, &mut var_qfr_db7, &mut var_qfr_db8, &mut var_qfr_db9, &mut var_qfr_dn0, &mut var_qfr_dn1, &mut var_qfr_dn10, &mut var_qfr_dn11, &mut var_qfr_dn12, &mut var_qfr_dn13, &mut var_qfr_dn14, &mut var_qfr_dn15, &mut var_qfr_dn16, &mut var_qfr_dn17, &mut var_qfr_dn18, &mut var_qfr_dn19, &mut var_qfr_dn2, &mut var_qfr_dn20, &mut var_qfr_dn21, &mut var_qfr_dn22, &mut var_qfr_dn3, &mut var_qfr_dn4, &mut var_qfr_dn5, &mut var_qfr_dn6, &mut var_qfr_dn7, &mut var_qfr_dn8, &mut var_qfr_dn9, &mut var_tnom);
        Self::stamp_transient_block_1(ctx, s, p, nodes, var_tnom, &mut var_guard353, &mut var_guard354, &mut var_guard355, &mut var_guard356, &mut var_guard357, &mut var_guard358, &mut var_phixn, &mut var_phixn_db0, &mut var_phixn_db1, &mut var_phixn_db10, &mut var_phixn_db11, &mut var_phixn_db12, &mut var_phixn_db13, &mut var_phixn_db14, &mut var_phixn_db15, &mut var_phixn_db16, &mut var_phixn_db17, &mut var_phixn_db18, &mut var_phixn_db19, &mut var_phixn_db2, &mut var_phixn_db20, &mut var_phixn_db21, &mut var_phixn_db22, &mut var_phixn_db23, &mut var_phixn_db24, &mut var_phixn_db25, &mut var_phixn_db26, &mut var_phixn_db27, &mut var_phixn_db28, &mut var_phixn_db29, &mut var_phixn_db3, &mut var_phixn_db30, &mut var_phixn_db31, &mut var_phixn_db32, &mut var_phixn_db33, &mut var_phixn_db34, &mut var_phixn_db35, &mut var_phixn_db36, &mut var_phixn_db37, &mut var_phixn_db38, &mut var_phixn_db39, &mut var_phixn_db4, &mut var_phixn_db40, &mut var_phixn_db41, &mut var_phixn_db42, &mut var_phixn_db43, &mut var_phixn_db44, &mut var_phixn_db45, &mut var_phixn_db46, &mut var_phixn_db47, &mut var_phixn_db48, &mut var_phixn_db49, &mut var_phixn_db5, &mut var_phixn_db50, &mut var_phixn_db51, &mut var_phixn_db52, &mut var_phixn_db53, &mut var_phixn_db54, &mut var_phixn_db6, &mut var_phixn_db7, &mut var_phixn_db8, &mut var_phixn_db9, &mut var_phixn_dn0, &mut var_phixn_dn1, &mut var_phixn_dn10, &mut var_phixn_dn11, &mut var_phixn_dn12, &mut var_phixn_dn13, &mut var_phixn_dn14, &mut var_phixn_dn15, &mut var_phixn_dn16, &mut var_phixn_dn17, &mut var_phixn_dn18, &mut var_phixn_dn19, &mut var_phixn_dn2, &mut var_phixn_dn20, &mut var_phixn_dn21, &mut var_phixn_dn22, &mut var_phixn_dn3, &mut var_phixn_dn4, &mut var_phixn_dn5, &mut var_phixn_dn6, &mut var_phixn_dn7, &mut var_phixn_dn8, &mut var_phixn_dn9, &mut var_tdev, &mut var_tdev_db0, &mut var_tdev_db1, &mut var_tdev_db10, &mut var_tdev_db11, &mut var_tdev_db12, &mut var_tdev_db13, &mut var_tdev_db14, &mut var_tdev_db15, &mut var_tdev_db16, &mut var_tdev_db17, &mut var_tdev_db18, &mut var_tdev_db19, &mut var_tdev_db2, &mut var_tdev_db20, &mut var_tdev_db21, &mut var_tdev_db22, &mut var_tdev_db23, &mut var_tdev_db24, &mut var_tdev_db25, &mut var_tdev_db26, &mut var_tdev_db27, &mut var_tdev_db28, &mut var_tdev_db29, &mut var_tdev_db3, &mut var_tdev_db30, &mut var_tdev_db31, &mut var_tdev_db32, &mut var_tdev_db33, &mut var_tdev_db34, &mut var_tdev_db35, &mut var_tdev_db36, &mut var_tdev_db37, &mut var_tdev_db38, &mut var_tdev_db39, &mut var_tdev_db4, &mut var_tdev_db40, &mut var_tdev_db41, &mut var_tdev_db42, &mut var_tdev_db43, &mut var_tdev_db44, &mut var_tdev_db45, &mut var_tdev_db46, &mut var_tdev_db47, &mut var_tdev_db48, &mut var_tdev_db49, &mut var_tdev_db5, &mut var_tdev_db50, &mut var_tdev_db51, &mut var_tdev_db52, &mut var_tdev_db53, &mut var_tdev_db54, &mut var_tdev_db6, &mut var_tdev_db7, &mut var_tdev_db8, &mut var_tdev_db9, &mut var_tdev_dn0, &mut var_tdev_dn1, &mut var_tdev_dn10, &mut var_tdev_dn11, &mut var_tdev_dn12, &mut var_tdev_dn13, &mut var_tdev_dn14, &mut var_tdev_dn15, &mut var_tdev_dn16, &mut var_tdev_dn17, &mut var_tdev_dn18, &mut var_tdev_dn19, &mut var_tdev_dn2, &mut var_tdev_dn20, &mut var_tdev_dn21, &mut var_tdev_dn22, &mut var_tdev_dn3, &mut var_tdev_dn4, &mut var_tdev_dn5, &mut var_tdev_dn6, &mut var_tdev_dn7, &mut var_tdev_dn8, &mut var_tdev_dn9, &mut var_vdgeff1, &mut var_vdgeff1_db0, &mut var_vdgeff1_db1, &mut var_vdgeff1_db10, &mut var_vdgeff1_db11, &mut var_vdgeff1_db12, &mut var_vdgeff1_db13, &mut var_vdgeff1_db14, &mut var_vdgeff1_db15, &mut var_vdgeff1_db16, &mut var_vdgeff1_db17, &mut var_vdgeff1_db18, &mut var_vdgeff1_db19, &mut var_vdgeff1_db2, &mut var_vdgeff1_db20, &mut var_vdgeff1_db21, &mut var_vdgeff1_db22, &mut var_vdgeff1_db23, &mut var_vdgeff1_db24, &mut var_vdgeff1_db25, &mut var_vdgeff1_db26, &mut var_vdgeff1_db27, &mut var_vdgeff1_db28, &mut var_vdgeff1_db29, &mut var_vdgeff1_db3, &mut var_vdgeff1_db30, &mut var_vdgeff1_db31, &mut var_vdgeff1_db32, &mut var_vdgeff1_db33, &mut var_vdgeff1_db34, &mut var_vdgeff1_db35, &mut var_vdgeff1_db36, &mut var_vdgeff1_db37, &mut var_vdgeff1_db38, &mut var_vdgeff1_db39, &mut var_vdgeff1_db4, &mut var_vdgeff1_db40, &mut var_vdgeff1_db41, &mut var_vdgeff1_db42, &mut var_vdgeff1_db43, &mut var_vdgeff1_db44, &mut var_vdgeff1_db45, &mut var_vdgeff1_db46, &mut var_vdgeff1_db47, &mut var_vdgeff1_db48, &mut var_vdgeff1_db49, &mut var_vdgeff1_db5, &mut var_vdgeff1_db50, &mut var_vdgeff1_db51, &mut var_vdgeff1_db52, &mut var_vdgeff1_db53, &mut var_vdgeff1_db54, &mut var_vdgeff1_db6, &mut var_vdgeff1_db7, &mut var_vdgeff1_db8, &mut var_vdgeff1_db9, &mut var_vdgeff1_dn0, &mut var_vdgeff1_dn1, &mut var_vdgeff1_dn10, &mut var_vdgeff1_dn11, &mut var_vdgeff1_dn12, &mut var_vdgeff1_dn13, &mut var_vdgeff1_dn14, &mut var_vdgeff1_dn15, &mut var_vdgeff1_dn16, &mut var_vdgeff1_dn17, &mut var_vdgeff1_dn18, &mut var_vdgeff1_dn19, &mut var_vdgeff1_dn2, &mut var_vdgeff1_dn20, &mut var_vdgeff1_dn21, &mut var_vdgeff1_dn22, &mut var_vdgeff1_dn3, &mut var_vdgeff1_dn4, &mut var_vdgeff1_dn5, &mut var_vdgeff1_dn6, &mut var_vdgeff1_dn7, &mut var_vdgeff1_dn8, &mut var_vdgeff1_dn9);
        Self::stamp_transient_block_2(ctx, s, p, nodes, var_guard353, var_guard354, var_guard355, var_guard356, var_guard357, var_guard358, var_tdev, var_tdev_db0, var_tdev_db1, var_tdev_db10, var_tdev_db11, var_tdev_db12, var_tdev_db13, var_tdev_db14, var_tdev_db15, var_tdev_db16, var_tdev_db17, var_tdev_db18, var_tdev_db19, var_tdev_db2, var_tdev_db20, var_tdev_db21, var_tdev_db22, var_tdev_db23, var_tdev_db24, var_tdev_db25, var_tdev_db26, var_tdev_db27, var_tdev_db28, var_tdev_db29, var_tdev_db3, var_tdev_db30, var_tdev_db31, var_tdev_db32, var_tdev_db33, var_tdev_db34, var_tdev_db35, var_tdev_db36, var_tdev_db37, var_tdev_db38, var_tdev_db39, var_tdev_db4, var_tdev_db40, var_tdev_db41, var_tdev_db42, var_tdev_db43, var_tdev_db44, var_tdev_db45, var_tdev_db46, var_tdev_db47, var_tdev_db48, var_tdev_db49, var_tdev_db5, var_tdev_db50, var_tdev_db51, var_tdev_db52, var_tdev_db53, var_tdev_db54, var_tdev_db6, var_tdev_db7, var_tdev_db8, var_tdev_db9, var_tdev_dn0, var_tdev_dn1, var_tdev_dn10, var_tdev_dn11, var_tdev_dn12, var_tdev_dn13, var_tdev_dn14, var_tdev_dn15, var_tdev_dn16, var_tdev_dn17, var_tdev_dn18, var_tdev_dn19, var_tdev_dn2, var_tdev_dn20, var_tdev_dn21, var_tdev_dn22, var_tdev_dn3, var_tdev_dn4, var_tdev_dn5, var_tdev_dn6, var_tdev_dn7, var_tdev_dn8, var_tdev_dn9, var_tnom, &mut var_en, &mut var_en1, &mut var_en1_db0, &mut var_en1_db1, &mut var_en1_db10, &mut var_en1_db11, &mut var_en1_db12, &mut var_en1_db13, &mut var_en1_db14, &mut var_en1_db15, &mut var_en1_db16, &mut var_en1_db17, &mut var_en1_db18, &mut var_en1_db19, &mut var_en1_db2, &mut var_en1_db20, &mut var_en1_db21, &mut var_en1_db22, &mut var_en1_db23, &mut var_en1_db24, &mut var_en1_db25, &mut var_en1_db26, &mut var_en1_db27, &mut var_en1_db28, &mut var_en1_db29, &mut var_en1_db3, &mut var_en1_db30, &mut var_en1_db31, &mut var_en1_db32, &mut var_en1_db33, &mut var_en1_db34, &mut var_en1_db35, &mut var_en1_db36, &mut var_en1_db37, &mut var_en1_db38, &mut var_en1_db39, &mut var_en1_db4, &mut var_en1_db40, &mut var_en1_db41, &mut var_en1_db42, &mut var_en1_db43, &mut var_en1_db44, &mut var_en1_db45, &mut var_en1_db46, &mut var_en1_db47, &mut var_en1_db48, &mut var_en1_db49, &mut var_en1_db5, &mut var_en1_db50, &mut var_en1_db51, &mut var_en1_db52, &mut var_en1_db53, &mut var_en1_db54, &mut var_en1_db6, &mut var_en1_db7, &mut var_en1_db8, &mut var_en1_db9, &mut var_en1_dn0, &mut var_en1_dn1, &mut var_en1_dn10, &mut var_en1_dn11, &mut var_en1_dn12, &mut var_en1_dn13, &mut var_en1_dn14, &mut var_en1_dn15, &mut var_en1_dn16, &mut var_en1_dn17, &mut var_en1_dn18, &mut var_en1_dn19, &mut var_en1_dn2, &mut var_en1_dn20, &mut var_en1_dn21, &mut var_en1_dn22, &mut var_en1_dn3, &mut var_en1_dn4, &mut var_en1_dn5, &mut var_en1_dn6, &mut var_en1_dn7, &mut var_en1_dn8, &mut var_en1_dn9, &mut var_en_db0, &mut var_en_db1, &mut var_en_db10, &mut var_en_db11, &mut var_en_db12, &mut var_en_db13, &mut var_en_db14, &mut var_en_db15, &mut var_en_db16, &mut var_en_db17, &mut var_en_db18, &mut var_en_db19, &mut var_en_db2, &mut var_en_db20, &mut var_en_db21, &mut var_en_db22, &mut var_en_db23, &mut var_en_db24, &mut var_en_db25, &mut var_en_db26, &mut var_en_db27, &mut var_en_db28, &mut var_en_db29, &mut var_en_db3, &mut var_en_db30, &mut var_en_db31, &mut var_en_db32, &mut var_en_db33, &mut var_en_db34, &mut var_en_db35, &mut var_en_db36, &mut var_en_db37, &mut var_en_db38, &mut var_en_db39, &mut var_en_db4, &mut var_en_db40, &mut var_en_db41, &mut var_en_db42, &mut var_en_db43, &mut var_en_db44, &mut var_en_db45, &mut var_en_db46, &mut var_en_db47, &mut var_en_db48, &mut var_en_db49, &mut var_en_db5, &mut var_en_db50, &mut var_en_db51, &mut var_en_db52, &mut var_en_db53, &mut var_en_db54, &mut var_en_db6, &mut var_en_db7, &mut var_en_db8, &mut var_en_db9, &mut var_en_dn0, &mut var_en_dn1, &mut var_en_dn10, &mut var_en_dn11, &mut var_en_dn12, &mut var_en_dn13, &mut var_en_dn14, &mut var_en_dn15, &mut var_en_dn16, &mut var_en_dn17, &mut var_en_dn18, &mut var_en_dn19, &mut var_en_dn2, &mut var_en_dn20, &mut var_en_dn21, &mut var_en_dn22, &mut var_en_dn3, &mut var_en_dn4, &mut var_en_dn5, &mut var_en_dn6, &mut var_en_dn7, &mut var_en_dn8, &mut var_en_dn9, &mut var_phiyn, &mut var_phiyn_db0, &mut var_phiyn_db1, &mut var_phiyn_db10, &mut var_phiyn_db11, &mut var_phiyn_db12, &mut var_phiyn_db13, &mut var_phiyn_db14, &mut var_phiyn_db15, &mut var_phiyn_db16, &mut var_phiyn_db17, &mut var_phiyn_db18, &mut var_phiyn_db19, &mut var_phiyn_db2, &mut var_phiyn_db20, &mut var_phiyn_db21, &mut var_phiyn_db22, &mut var_phiyn_db23, &mut var_phiyn_db24, &mut var_phiyn_db25, &mut var_phiyn_db26, &mut var_phiyn_db27, &mut var_phiyn_db28, &mut var_phiyn_db29, &mut var_phiyn_db3, &mut var_phiyn_db30, &mut var_phiyn_db31, &mut var_phiyn_db32, &mut var_phiyn_db33, &mut var_phiyn_db34, &mut var_phiyn_db35, &mut var_phiyn_db36, &mut var_phiyn_db37, &mut var_phiyn_db38, &mut var_phiyn_db39, &mut var_phiyn_db4, &mut var_phiyn_db40, &mut var_phiyn_db41, &mut var_phiyn_db42, &mut var_phiyn_db43, &mut var_phiyn_db44, &mut var_phiyn_db45, &mut var_phiyn_db46, &mut var_phiyn_db47, &mut var_phiyn_db48, &mut var_phiyn_db49, &mut var_phiyn_db5, &mut var_phiyn_db50, &mut var_phiyn_db51, &mut var_phiyn_db52, &mut var_phiyn_db53, &mut var_phiyn_db54, &mut var_phiyn_db6, &mut var_phiyn_db7, &mut var_phiyn_db8, &mut var_phiyn_db9, &mut var_phiyn_dn0, &mut var_phiyn_dn1, &mut var_phiyn_dn10, &mut var_phiyn_dn11, &mut var_phiyn_dn12, &mut var_phiyn_dn13, &mut var_phiyn_dn14, &mut var_phiyn_dn15, &mut var_phiyn_dn16, &mut var_phiyn_dn17, &mut var_phiyn_dn18, &mut var_phiyn_dn19, &mut var_phiyn_dn2, &mut var_phiyn_dn20, &mut var_phiyn_dn21, &mut var_phiyn_dn22, &mut var_phiyn_dn3, &mut var_phiyn_dn4, &mut var_phiyn_dn5, &mut var_phiyn_dn6, &mut var_phiyn_dn7, &mut var_phiyn_dn8, &mut var_phiyn_dn9);
        Self::stamp_transient_block_3(s, p, var_tnom);
        Self::stamp_transient_block_4(ctx, s, p, nodes, var_tnom);
        Self::stamp_transient_block_5(ctx, s, p, nodes, param_given, var_tnom);
        Self::stamp_transient_block_6(s, p, var_tnom);
        Self::stamp_transient_block_7(ctx, s, p, nodes, var_tnom);
        Self::stamp_transient_block_8(ctx, s, p, nodes, var_tnom);
        Self::stamp_transient_block_9(s, p, var_tnom);
        Self::stamp_transient_block_10(ctx, s, p, nodes, var_tnom);
        Self::stamp_transient_block_11(ctx, s, p, nodes, var_tnom);
        Self::stamp_transient_block_12(s, p, var_tnom);
        Self::stamp_transient_block_13(ctx, s, p, nodes, var_tnom);
        Self::stamp_transient_block_14(ctx, s, p, nodes, var_tnom);
        Self::stamp_transient_block_15(s, p, var_tnom);
        Self::stamp_transient_block_16(ctx, s, p, nodes, var_tnom);
        Self::stamp_transient_block_17(ctx, s, p, nodes, var_tnom);
        Self::stamp_transient_block_18(s, p, var_tnom);
        Self::stamp_transient_block_19(ctx, s, p, nodes, var_tnom);
        Self::stamp_transient_block_20(ctx, s, p, nodes, var_tnom);
        Self::stamp_transient_block_21(s, p, var_tnom);
        Self::stamp_transient_block_22(ctx, s, p, nodes, var_tnom);
        Self::stamp_transient_block_23(ctx, s, p, nodes, var_tnom);
        Self::stamp_transient_block_24(s, p, var_tnom);
        Self::stamp_transient_block_25(ctx, s, p, nodes, var_tnom);
        Self::stamp_transient_block_26(ctx, s, p, nodes, var_tnom);
        Self::stamp_transient_block_27(s, p, var_tnom);
        Self::stamp_transient_block_28(ctx, s, p, nodes, var_tnom);
        Self::stamp_transient_block_29(ctx, s, p, nodes, &mut var_cgdl_l, &mut var_guard524, &mut var_qsov, &mut var_qsov_db0, &mut var_qsov_db1, &mut var_qsov_db10, &mut var_qsov_db11, &mut var_qsov_db12, &mut var_qsov_db13, &mut var_qsov_db14, &mut var_qsov_db15, &mut var_qsov_db16, &mut var_qsov_db17, &mut var_qsov_db18, &mut var_qsov_db19, &mut var_qsov_db2, &mut var_qsov_db20, &mut var_qsov_db21, &mut var_qsov_db22, &mut var_qsov_db23, &mut var_qsov_db24, &mut var_qsov_db25, &mut var_qsov_db26, &mut var_qsov_db27, &mut var_qsov_db28, &mut var_qsov_db29, &mut var_qsov_db3, &mut var_qsov_db30, &mut var_qsov_db31, &mut var_qsov_db32, &mut var_qsov_db33, &mut var_qsov_db34, &mut var_qsov_db35, &mut var_qsov_db36, &mut var_qsov_db37, &mut var_qsov_db38, &mut var_qsov_db39, &mut var_qsov_db4, &mut var_qsov_db40, &mut var_qsov_db41, &mut var_qsov_db42, &mut var_qsov_db43, &mut var_qsov_db44, &mut var_qsov_db45, &mut var_qsov_db46, &mut var_qsov_db47, &mut var_qsov_db48, &mut var_qsov_db49, &mut var_qsov_db5, &mut var_qsov_db50, &mut var_qsov_db51, &mut var_qsov_db52, &mut var_qsov_db53, &mut var_qsov_db54, &mut var_qsov_db6, &mut var_qsov_db7, &mut var_qsov_db8, &mut var_qsov_db9, &mut var_qsov_dn0, &mut var_qsov_dn1, &mut var_qsov_dn10, &mut var_qsov_dn11, &mut var_qsov_dn12, &mut var_qsov_dn13, &mut var_qsov_dn14, &mut var_qsov_dn15, &mut var_qsov_dn16, &mut var_qsov_dn17, &mut var_qsov_dn18, &mut var_qsov_dn19, &mut var_qsov_dn2, &mut var_qsov_dn20, &mut var_qsov_dn21, &mut var_qsov_dn22, &mut var_qsov_dn3, &mut var_qsov_dn4, &mut var_qsov_dn5, &mut var_qsov_dn6, &mut var_qsov_dn7, &mut var_qsov_dn8, &mut var_qsov_dn9, &mut var_vdseffcv, &mut var_vdseffcv_db0, &mut var_vdseffcv_db1, &mut var_vdseffcv_db10, &mut var_vdseffcv_db11, &mut var_vdseffcv_db12, &mut var_vdseffcv_db13, &mut var_vdseffcv_db14, &mut var_vdseffcv_db15, &mut var_vdseffcv_db16, &mut var_vdseffcv_db17, &mut var_vdseffcv_db18, &mut var_vdseffcv_db19, &mut var_vdseffcv_db2, &mut var_vdseffcv_db20, &mut var_vdseffcv_db21, &mut var_vdseffcv_db22, &mut var_vdseffcv_db23, &mut var_vdseffcv_db24, &mut var_vdseffcv_db25, &mut var_vdseffcv_db26, &mut var_vdseffcv_db27, &mut var_vdseffcv_db28, &mut var_vdseffcv_db29, &mut var_vdseffcv_db3, &mut var_vdseffcv_db30, &mut var_vdseffcv_db31, &mut var_vdseffcv_db32, &mut var_vdseffcv_db33, &mut var_vdseffcv_db34, &mut var_vdseffcv_db35, &mut var_vdseffcv_db36, &mut var_vdseffcv_db37, &mut var_vdseffcv_db38, &mut var_vdseffcv_db39, &mut var_vdseffcv_db4, &mut var_vdseffcv_db40, &mut var_vdseffcv_db41, &mut var_vdseffcv_db42, &mut var_vdseffcv_db43, &mut var_vdseffcv_db44, &mut var_vdseffcv_db45, &mut var_vdseffcv_db46, &mut var_vdseffcv_db47, &mut var_vdseffcv_db48, &mut var_vdseffcv_db49, &mut var_vdseffcv_db5, &mut var_vdseffcv_db50, &mut var_vdseffcv_db51, &mut var_vdseffcv_db52, &mut var_vdseffcv_db53, &mut var_vdseffcv_db54, &mut var_vdseffcv_db6, &mut var_vdseffcv_db7, &mut var_vdseffcv_db8, &mut var_vdseffcv_db9, &mut var_vdseffcv_dn0, &mut var_vdseffcv_dn1, &mut var_vdseffcv_dn10, &mut var_vdseffcv_dn11, &mut var_vdseffcv_dn12, &mut var_vdseffcv_dn13, &mut var_vdseffcv_dn14, &mut var_vdseffcv_dn15, &mut var_vdseffcv_dn16, &mut var_vdseffcv_dn17, &mut var_vdseffcv_dn18, &mut var_vdseffcv_dn19, &mut var_vdseffcv_dn2, &mut var_vdseffcv_dn20, &mut var_vdseffcv_dn21, &mut var_vdseffcv_dn22, &mut var_vdseffcv_dn3, &mut var_vdseffcv_dn4, &mut var_vdseffcv_dn5, &mut var_vdseffcv_dn6, &mut var_vdseffcv_dn7, &mut var_vdseffcv_dn8, &mut var_vdseffcv_dn9);
        Self::stamp_transient_block_30(ctx, p, nodes, var_guard524, &mut var_cgdl_l, &mut var_cgdvar, &mut var_cgdvar_db0, &mut var_cgdvar_db1, &mut var_cgdvar_db10, &mut var_cgdvar_db11, &mut var_cgdvar_db12, &mut var_cgdvar_db13, &mut var_cgdvar_db14, &mut var_cgdvar_db15, &mut var_cgdvar_db16, &mut var_cgdvar_db17, &mut var_cgdvar_db18, &mut var_cgdvar_db19, &mut var_cgdvar_db2, &mut var_cgdvar_db20, &mut var_cgdvar_db21, &mut var_cgdvar_db22, &mut var_cgdvar_db23, &mut var_cgdvar_db24, &mut var_cgdvar_db25, &mut var_cgdvar_db26, &mut var_cgdvar_db27, &mut var_cgdvar_db28, &mut var_cgdvar_db29, &mut var_cgdvar_db3, &mut var_cgdvar_db30, &mut var_cgdvar_db31, &mut var_cgdvar_db32, &mut var_cgdvar_db33, &mut var_cgdvar_db34, &mut var_cgdvar_db35, &mut var_cgdvar_db36, &mut var_cgdvar_db37, &mut var_cgdvar_db38, &mut var_cgdvar_db39, &mut var_cgdvar_db4, &mut var_cgdvar_db40, &mut var_cgdvar_db41, &mut var_cgdvar_db42, &mut var_cgdvar_db43, &mut var_cgdvar_db44, &mut var_cgdvar_db45, &mut var_cgdvar_db46, &mut var_cgdvar_db47, &mut var_cgdvar_db48, &mut var_cgdvar_db49, &mut var_cgdvar_db5, &mut var_cgdvar_db50, &mut var_cgdvar_db51, &mut var_cgdvar_db52, &mut var_cgdvar_db53, &mut var_cgdvar_db54, &mut var_cgdvar_db6, &mut var_cgdvar_db7, &mut var_cgdvar_db8, &mut var_cgdvar_db9, &mut var_cgdvar_dn0, &mut var_cgdvar_dn1, &mut var_cgdvar_dn10, &mut var_cgdvar_dn11, &mut var_cgdvar_dn12, &mut var_cgdvar_dn13, &mut var_cgdvar_dn14, &mut var_cgdvar_dn15, &mut var_cgdvar_dn16, &mut var_cgdvar_dn17, &mut var_cgdvar_dn18, &mut var_cgdvar_dn19, &mut var_cgdvar_dn2, &mut var_cgdvar_dn20, &mut var_cgdvar_dn21, &mut var_cgdvar_dn22, &mut var_cgdvar_dn3, &mut var_cgdvar_dn4, &mut var_cgdvar_dn5, &mut var_cgdvar_dn6, &mut var_cgdvar_dn7, &mut var_cgdvar_dn8, &mut var_cgdvar_dn9, &mut var_qdov, &mut var_qdov_db0, &mut var_qdov_db1, &mut var_qdov_db10, &mut var_qdov_db11, &mut var_qdov_db12, &mut var_qdov_db13, &mut var_qdov_db14, &mut var_qdov_db15, &mut var_qdov_db16, &mut var_qdov_db17, &mut var_qdov_db18, &mut var_qdov_db19, &mut var_qdov_db2, &mut var_qdov_db20, &mut var_qdov_db21, &mut var_qdov_db22, &mut var_qdov_db23, &mut var_qdov_db24, &mut var_qdov_db25, &mut var_qdov_db26, &mut var_qdov_db27, &mut var_qdov_db28, &mut var_qdov_db29, &mut var_qdov_db3, &mut var_qdov_db30, &mut var_qdov_db31, &mut var_qdov_db32, &mut var_qdov_db33, &mut var_qdov_db34, &mut var_qdov_db35, &mut var_qdov_db36, &mut var_qdov_db37, &mut var_qdov_db38, &mut var_qdov_db39, &mut var_qdov_db4, &mut var_qdov_db40, &mut var_qdov_db41, &mut var_qdov_db42, &mut var_qdov_db43, &mut var_qdov_db44, &mut var_qdov_db45, &mut var_qdov_db46, &mut var_qdov_db47, &mut var_qdov_db48, &mut var_qdov_db49, &mut var_qdov_db5, &mut var_qdov_db50, &mut var_qdov_db51, &mut var_qdov_db52, &mut var_qdov_db53, &mut var_qdov_db54, &mut var_qdov_db6, &mut var_qdov_db7, &mut var_qdov_db8, &mut var_qdov_db9, &mut var_qdov_dn0, &mut var_qdov_dn1, &mut var_qdov_dn10, &mut var_qdov_dn11, &mut var_qdov_dn12, &mut var_qdov_dn13, &mut var_qdov_dn14, &mut var_qdov_dn15, &mut var_qdov_dn16, &mut var_qdov_dn17, &mut var_qdov_dn18, &mut var_qdov_dn19, &mut var_qdov_dn2, &mut var_qdov_dn20, &mut var_qdov_dn21, &mut var_qdov_dn22, &mut var_qdov_dn3, &mut var_qdov_dn4, &mut var_qdov_dn5, &mut var_qdov_dn6, &mut var_qdov_dn7, &mut var_qdov_dn8, &mut var_qdov_dn9, &mut var_qsov, &mut var_qsov_db0, &mut var_qsov_db1, &mut var_qsov_db10, &mut var_qsov_db11, &mut var_qsov_db12, &mut var_qsov_db13, &mut var_qsov_db14, &mut var_qsov_db15, &mut var_qsov_db16, &mut var_qsov_db17, &mut var_qsov_db18, &mut var_qsov_db19, &mut var_qsov_db2, &mut var_qsov_db20, &mut var_qsov_db21, &mut var_qsov_db22, &mut var_qsov_db23, &mut var_qsov_db24, &mut var_qsov_db25, &mut var_qsov_db26, &mut var_qsov_db27, &mut var_qsov_db28, &mut var_qsov_db29, &mut var_qsov_db3, &mut var_qsov_db30, &mut var_qsov_db31, &mut var_qsov_db32, &mut var_qsov_db33, &mut var_qsov_db34, &mut var_qsov_db35, &mut var_qsov_db36, &mut var_qsov_db37, &mut var_qsov_db38, &mut var_qsov_db39, &mut var_qsov_db4, &mut var_qsov_db40, &mut var_qsov_db41, &mut var_qsov_db42, &mut var_qsov_db43, &mut var_qsov_db44, &mut var_qsov_db45, &mut var_qsov_db46, &mut var_qsov_db47, &mut var_qsov_db48, &mut var_qsov_db49, &mut var_qsov_db5, &mut var_qsov_db50, &mut var_qsov_db51, &mut var_qsov_db52, &mut var_qsov_db53, &mut var_qsov_db54, &mut var_qsov_db6, &mut var_qsov_db7, &mut var_qsov_db8, &mut var_qsov_db9, &mut var_qsov_dn0, &mut var_qsov_dn1, &mut var_qsov_dn10, &mut var_qsov_dn11, &mut var_qsov_dn12, &mut var_qsov_dn13, &mut var_qsov_dn14, &mut var_qsov_dn15, &mut var_qsov_dn16, &mut var_qsov_dn17, &mut var_qsov_dn18, &mut var_qsov_dn19, &mut var_qsov_dn2, &mut var_qsov_dn20, &mut var_qsov_dn21, &mut var_qsov_dn22, &mut var_qsov_dn3, &mut var_qsov_dn4, &mut var_qsov_dn5, &mut var_qsov_dn6, &mut var_qsov_dn7, &mut var_qsov_dn8, &mut var_qsov_dn9, &mut var_vdseffcv, &mut var_vdseffcv_db0, &mut var_vdseffcv_db1, &mut var_vdseffcv_db10, &mut var_vdseffcv_db11, &mut var_vdseffcv_db12, &mut var_vdseffcv_db13, &mut var_vdseffcv_db14, &mut var_vdseffcv_db15, &mut var_vdseffcv_db16, &mut var_vdseffcv_db17, &mut var_vdseffcv_db18, &mut var_vdseffcv_db19, &mut var_vdseffcv_db2, &mut var_vdseffcv_db20, &mut var_vdseffcv_db21, &mut var_vdseffcv_db22, &mut var_vdseffcv_db23, &mut var_vdseffcv_db24, &mut var_vdseffcv_db25, &mut var_vdseffcv_db26, &mut var_vdseffcv_db27, &mut var_vdseffcv_db28, &mut var_vdseffcv_db29, &mut var_vdseffcv_db3, &mut var_vdseffcv_db30, &mut var_vdseffcv_db31, &mut var_vdseffcv_db32, &mut var_vdseffcv_db33, &mut var_vdseffcv_db34, &mut var_vdseffcv_db35, &mut var_vdseffcv_db36, &mut var_vdseffcv_db37, &mut var_vdseffcv_db38, &mut var_vdseffcv_db39, &mut var_vdseffcv_db4, &mut var_vdseffcv_db40, &mut var_vdseffcv_db41, &mut var_vdseffcv_db42, &mut var_vdseffcv_db43, &mut var_vdseffcv_db44, &mut var_vdseffcv_db45, &mut var_vdseffcv_db46, &mut var_vdseffcv_db47, &mut var_vdseffcv_db48, &mut var_vdseffcv_db49, &mut var_vdseffcv_db5, &mut var_vdseffcv_db50, &mut var_vdseffcv_db51, &mut var_vdseffcv_db52, &mut var_vdseffcv_db53, &mut var_vdseffcv_db54, &mut var_vdseffcv_db6, &mut var_vdseffcv_db7, &mut var_vdseffcv_db8, &mut var_vdseffcv_db9, &mut var_vdseffcv_dn0, &mut var_vdseffcv_dn1, &mut var_vdseffcv_dn10, &mut var_vdseffcv_dn11, &mut var_vdseffcv_dn12, &mut var_vdseffcv_dn13, &mut var_vdseffcv_dn14, &mut var_vdseffcv_dn15, &mut var_vdseffcv_dn16, &mut var_vdseffcv_dn17, &mut var_vdseffcv_dn18, &mut var_vdseffcv_dn19, &mut var_vdseffcv_dn2, &mut var_vdseffcv_dn20, &mut var_vdseffcv_dn21, &mut var_vdseffcv_dn22, &mut var_vdseffcv_dn3, &mut var_vdseffcv_dn4, &mut var_vdseffcv_dn5, &mut var_vdseffcv_dn6, &mut var_vdseffcv_dn7, &mut var_vdseffcv_dn8, &mut var_vdseffcv_dn9);
        Self::stamp_transient_block_31(ctx, s, p, nodes, var_cgdvar, var_cgdvar_db0, var_cgdvar_db1, var_cgdvar_db10, var_cgdvar_db11, var_cgdvar_db12, var_cgdvar_db13, var_cgdvar_db14, var_cgdvar_db15, var_cgdvar_db16, var_cgdvar_db17, var_cgdvar_db18, var_cgdvar_db19, var_cgdvar_db2, var_cgdvar_db20, var_cgdvar_db21, var_cgdvar_db22, var_cgdvar_db23, var_cgdvar_db24, var_cgdvar_db25, var_cgdvar_db26, var_cgdvar_db27, var_cgdvar_db28, var_cgdvar_db29, var_cgdvar_db3, var_cgdvar_db30, var_cgdvar_db31, var_cgdvar_db32, var_cgdvar_db33, var_cgdvar_db34, var_cgdvar_db35, var_cgdvar_db36, var_cgdvar_db37, var_cgdvar_db38, var_cgdvar_db39, var_cgdvar_db4, var_cgdvar_db40, var_cgdvar_db41, var_cgdvar_db42, var_cgdvar_db43, var_cgdvar_db44, var_cgdvar_db45, var_cgdvar_db46, var_cgdvar_db47, var_cgdvar_db48, var_cgdvar_db49, var_cgdvar_db5, var_cgdvar_db50, var_cgdvar_db51, var_cgdvar_db52, var_cgdvar_db53, var_cgdvar_db54, var_cgdvar_db6, var_cgdvar_db7, var_cgdvar_db8, var_cgdvar_db9, var_cgdvar_dn0, var_cgdvar_dn1, var_cgdvar_dn10, var_cgdvar_dn11, var_cgdvar_dn12, var_cgdvar_dn13, var_cgdvar_dn14, var_cgdvar_dn15, var_cgdvar_dn16, var_cgdvar_dn17, var_cgdvar_dn18, var_cgdvar_dn19, var_cgdvar_dn2, var_cgdvar_dn20, var_cgdvar_dn21, var_cgdvar_dn22, var_cgdvar_dn3, var_cgdvar_dn4, var_cgdvar_dn5, var_cgdvar_dn6, var_cgdvar_dn7, var_cgdvar_dn8, var_cgdvar_dn9, var_guard524, var_tnom, &mut var_qbdov, &mut var_qbdov_db0, &mut var_qbdov_db1, &mut var_qbdov_db10, &mut var_qbdov_db11, &mut var_qbdov_db12, &mut var_qbdov_db13, &mut var_qbdov_db14, &mut var_qbdov_db15, &mut var_qbdov_db16, &mut var_qbdov_db17, &mut var_qbdov_db18, &mut var_qbdov_db19, &mut var_qbdov_db2, &mut var_qbdov_db20, &mut var_qbdov_db21, &mut var_qbdov_db22, &mut var_qbdov_db23, &mut var_qbdov_db24, &mut var_qbdov_db25, &mut var_qbdov_db26, &mut var_qbdov_db27, &mut var_qbdov_db28, &mut var_qbdov_db29, &mut var_qbdov_db3, &mut var_qbdov_db30, &mut var_qbdov_db31, &mut var_qbdov_db32, &mut var_qbdov_db33, &mut var_qbdov_db34, &mut var_qbdov_db35, &mut var_qbdov_db36, &mut var_qbdov_db37, &mut var_qbdov_db38, &mut var_qbdov_db39, &mut var_qbdov_db4, &mut var_qbdov_db40, &mut var_qbdov_db41, &mut var_qbdov_db42, &mut var_qbdov_db43, &mut var_qbdov_db44, &mut var_qbdov_db45, &mut var_qbdov_db46, &mut var_qbdov_db47, &mut var_qbdov_db48, &mut var_qbdov_db49, &mut var_qbdov_db5, &mut var_qbdov_db50, &mut var_qbdov_db51, &mut var_qbdov_db52, &mut var_qbdov_db53, &mut var_qbdov_db54, &mut var_qbdov_db6, &mut var_qbdov_db7, &mut var_qbdov_db8, &mut var_qbdov_db9, &mut var_qbdov_dn0, &mut var_qbdov_dn1, &mut var_qbdov_dn10, &mut var_qbdov_dn11, &mut var_qbdov_dn12, &mut var_qbdov_dn13, &mut var_qbdov_dn14, &mut var_qbdov_dn15, &mut var_qbdov_dn16, &mut var_qbdov_dn17, &mut var_qbdov_dn18, &mut var_qbdov_dn19, &mut var_qbdov_dn2, &mut var_qbdov_dn20, &mut var_qbdov_dn21, &mut var_qbdov_dn22, &mut var_qbdov_dn3, &mut var_qbdov_dn4, &mut var_qbdov_dn5, &mut var_qbdov_dn6, &mut var_qbdov_dn7, &mut var_qbdov_dn8, &mut var_qbdov_dn9, &mut var_qbgov, &mut var_qbgov_db0, &mut var_qbgov_db1, &mut var_qbgov_db10, &mut var_qbgov_db11, &mut var_qbgov_db12, &mut var_qbgov_db13, &mut var_qbgov_db14, &mut var_qbgov_db15, &mut var_qbgov_db16, &mut var_qbgov_db17, &mut var_qbgov_db18, &mut var_qbgov_db19, &mut var_qbgov_db2, &mut var_qbgov_db20, &mut var_qbgov_db21, &mut var_qbgov_db22, &mut var_qbgov_db23, &mut var_qbgov_db24, &mut var_qbgov_db25, &mut var_qbgov_db26, &mut var_qbgov_db27, &mut var_qbgov_db28, &mut var_qbgov_db29, &mut var_qbgov_db3, &mut var_qbgov_db30, &mut var_qbgov_db31, &mut var_qbgov_db32, &mut var_qbgov_db33, &mut var_qbgov_db34, &mut var_qbgov_db35, &mut var_qbgov_db36, &mut var_qbgov_db37, &mut var_qbgov_db38, &mut var_qbgov_db39, &mut var_qbgov_db4, &mut var_qbgov_db40, &mut var_qbgov_db41, &mut var_qbgov_db42, &mut var_qbgov_db43, &mut var_qbgov_db44, &mut var_qbgov_db45, &mut var_qbgov_db46, &mut var_qbgov_db47, &mut var_qbgov_db48, &mut var_qbgov_db49, &mut var_qbgov_db5, &mut var_qbgov_db50, &mut var_qbgov_db51, &mut var_qbgov_db52, &mut var_qbgov_db53, &mut var_qbgov_db54, &mut var_qbgov_db6, &mut var_qbgov_db7, &mut var_qbgov_db8, &mut var_qbgov_db9, &mut var_qbgov_dn0, &mut var_qbgov_dn1, &mut var_qbgov_dn10, &mut var_qbgov_dn11, &mut var_qbgov_dn12, &mut var_qbgov_dn13, &mut var_qbgov_dn14, &mut var_qbgov_dn15, &mut var_qbgov_dn16, &mut var_qbgov_dn17, &mut var_qbgov_dn18, &mut var_qbgov_dn19, &mut var_qbgov_dn2, &mut var_qbgov_dn20, &mut var_qbgov_dn21, &mut var_qbgov_dn22, &mut var_qbgov_dn3, &mut var_qbgov_dn4, &mut var_qbgov_dn5, &mut var_qbgov_dn6, &mut var_qbgov_dn7, &mut var_qbgov_dn8, &mut var_qbgov_dn9, &mut var_qbsov, &mut var_qbsov_db0, &mut var_qbsov_db1, &mut var_qbsov_db10, &mut var_qbsov_db11, &mut var_qbsov_db12, &mut var_qbsov_db13, &mut var_qbsov_db14, &mut var_qbsov_db15, &mut var_qbsov_db16, &mut var_qbsov_db17, &mut var_qbsov_db18, &mut var_qbsov_db19, &mut var_qbsov_db2, &mut var_qbsov_db20, &mut var_qbsov_db21, &mut var_qbsov_db22, &mut var_qbsov_db23, &mut var_qbsov_db24, &mut var_qbsov_db25, &mut var_qbsov_db26, &mut var_qbsov_db27, &mut var_qbsov_db28, &mut var_qbsov_db29, &mut var_qbsov_db3, &mut var_qbsov_db30, &mut var_qbsov_db31, &mut var_qbsov_db32, &mut var_qbsov_db33, &mut var_qbsov_db34, &mut var_qbsov_db35, &mut var_qbsov_db36, &mut var_qbsov_db37, &mut var_qbsov_db38, &mut var_qbsov_db39, &mut var_qbsov_db4, &mut var_qbsov_db40, &mut var_qbsov_db41, &mut var_qbsov_db42, &mut var_qbsov_db43, &mut var_qbsov_db44, &mut var_qbsov_db45, &mut var_qbsov_db46, &mut var_qbsov_db47, &mut var_qbsov_db48, &mut var_qbsov_db49, &mut var_qbsov_db5, &mut var_qbsov_db50, &mut var_qbsov_db51, &mut var_qbsov_db52, &mut var_qbsov_db53, &mut var_qbsov_db54, &mut var_qbsov_db6, &mut var_qbsov_db7, &mut var_qbsov_db8, &mut var_qbsov_db9, &mut var_qbsov_dn0, &mut var_qbsov_dn1, &mut var_qbsov_dn10, &mut var_qbsov_dn11, &mut var_qbsov_dn12, &mut var_qbsov_dn13, &mut var_qbsov_dn14, &mut var_qbsov_dn15, &mut var_qbsov_dn16, &mut var_qbsov_dn17, &mut var_qbsov_dn18, &mut var_qbsov_dn19, &mut var_qbsov_dn2, &mut var_qbsov_dn20, &mut var_qbsov_dn21, &mut var_qbsov_dn22, &mut var_qbsov_dn3, &mut var_qbsov_dn4, &mut var_qbsov_dn5, &mut var_qbsov_dn6, &mut var_qbsov_dn7, &mut var_qbsov_dn8, &mut var_qbsov_dn9, &mut var_qdov, &mut var_qdov_db0, &mut var_qdov_db1, &mut var_qdov_db10, &mut var_qdov_db11, &mut var_qdov_db12, &mut var_qdov_db13, &mut var_qdov_db14, &mut var_qdov_db15, &mut var_qdov_db16, &mut var_qdov_db17, &mut var_qdov_db18, &mut var_qdov_db19, &mut var_qdov_db2, &mut var_qdov_db20, &mut var_qdov_db21, &mut var_qdov_db22, &mut var_qdov_db23, &mut var_qdov_db24, &mut var_qdov_db25, &mut var_qdov_db26, &mut var_qdov_db27, &mut var_qdov_db28, &mut var_qdov_db29, &mut var_qdov_db3, &mut var_qdov_db30, &mut var_qdov_db31, &mut var_qdov_db32, &mut var_qdov_db33, &mut var_qdov_db34, &mut var_qdov_db35, &mut var_qdov_db36, &mut var_qdov_db37, &mut var_qdov_db38, &mut var_qdov_db39, &mut var_qdov_db4, &mut var_qdov_db40, &mut var_qdov_db41, &mut var_qdov_db42, &mut var_qdov_db43, &mut var_qdov_db44, &mut var_qdov_db45, &mut var_qdov_db46, &mut var_qdov_db47, &mut var_qdov_db48, &mut var_qdov_db49, &mut var_qdov_db5, &mut var_qdov_db50, &mut var_qdov_db51, &mut var_qdov_db52, &mut var_qdov_db53, &mut var_qdov_db54, &mut var_qdov_db6, &mut var_qdov_db7, &mut var_qdov_db8, &mut var_qdov_db9, &mut var_qdov_dn0, &mut var_qdov_dn1, &mut var_qdov_dn10, &mut var_qdov_dn11, &mut var_qdov_dn12, &mut var_qdov_dn13, &mut var_qdov_dn14, &mut var_qdov_dn15, &mut var_qdov_dn16, &mut var_qdov_dn17, &mut var_qdov_dn18, &mut var_qdov_dn19, &mut var_qdov_dn2, &mut var_qdov_dn20, &mut var_qdov_dn21, &mut var_qdov_dn22, &mut var_qdov_dn3, &mut var_qdov_dn4, &mut var_qdov_dn5, &mut var_qdov_dn6, &mut var_qdov_dn7, &mut var_qdov_dn8, &mut var_qdov_dn9, &mut var_qdsov, &mut var_qdsov_db0, &mut var_qdsov_db1, &mut var_qdsov_db10, &mut var_qdsov_db11, &mut var_qdsov_db12, &mut var_qdsov_db13, &mut var_qdsov_db14, &mut var_qdsov_db15, &mut var_qdsov_db16, &mut var_qdsov_db17, &mut var_qdsov_db18, &mut var_qdsov_db19, &mut var_qdsov_db2, &mut var_qdsov_db20, &mut var_qdsov_db21, &mut var_qdsov_db22, &mut var_qdsov_db23, &mut var_qdsov_db24, &mut var_qdsov_db25, &mut var_qdsov_db26, &mut var_qdsov_db27, &mut var_qdsov_db28, &mut var_qdsov_db29, &mut var_qdsov_db3, &mut var_qdsov_db30, &mut var_qdsov_db31, &mut var_qdsov_db32, &mut var_qdsov_db33, &mut var_qdsov_db34, &mut var_qdsov_db35, &mut var_qdsov_db36, &mut var_qdsov_db37, &mut var_qdsov_db38, &mut var_qdsov_db39, &mut var_qdsov_db4, &mut var_qdsov_db40, &mut var_qdsov_db41, &mut var_qdsov_db42, &mut var_qdsov_db43, &mut var_qdsov_db44, &mut var_qdsov_db45, &mut var_qdsov_db46, &mut var_qdsov_db47, &mut var_qdsov_db48, &mut var_qdsov_db49, &mut var_qdsov_db5, &mut var_qdsov_db50, &mut var_qdsov_db51, &mut var_qdsov_db52, &mut var_qdsov_db53, &mut var_qdsov_db54, &mut var_qdsov_db6, &mut var_qdsov_db7, &mut var_qdsov_db8, &mut var_qdsov_db9, &mut var_qdsov_dn0, &mut var_qdsov_dn1, &mut var_qdsov_dn10, &mut var_qdsov_dn11, &mut var_qdsov_dn12, &mut var_qdsov_dn13, &mut var_qdsov_dn14, &mut var_qdsov_dn15, &mut var_qdsov_dn16, &mut var_qdsov_dn17, &mut var_qdsov_dn18, &mut var_qdsov_dn19, &mut var_qdsov_dn2, &mut var_qdsov_dn20, &mut var_qdsov_dn21, &mut var_qdsov_dn22, &mut var_qdsov_dn3, &mut var_qdsov_dn4, &mut var_qdsov_dn5, &mut var_qdsov_dn6, &mut var_qdsov_dn7, &mut var_qdsov_dn8, &mut var_qdsov_dn9);
        Self::stamp_transient_block_32(ctx, s, p, nodes, var_tdev, var_tdev_db0, var_tdev_db1, var_tdev_db10, var_tdev_db11, var_tdev_db12, var_tdev_db13, var_tdev_db14, var_tdev_db15, var_tdev_db16, var_tdev_db17, var_tdev_db18, var_tdev_db19, var_tdev_db2, var_tdev_db20, var_tdev_db21, var_tdev_db22, var_tdev_db23, var_tdev_db24, var_tdev_db25, var_tdev_db26, var_tdev_db27, var_tdev_db28, var_tdev_db29, var_tdev_db3, var_tdev_db30, var_tdev_db31, var_tdev_db32, var_tdev_db33, var_tdev_db34, var_tdev_db35, var_tdev_db36, var_tdev_db37, var_tdev_db38, var_tdev_db39, var_tdev_db4, var_tdev_db40, var_tdev_db41, var_tdev_db42, var_tdev_db43, var_tdev_db44, var_tdev_db45, var_tdev_db46, var_tdev_db47, var_tdev_db48, var_tdev_db49, var_tdev_db5, var_tdev_db50, var_tdev_db51, var_tdev_db52, var_tdev_db53, var_tdev_db54, var_tdev_db6, var_tdev_db7, var_tdev_db8, var_tdev_db9, var_tdev_dn0, var_tdev_dn1, var_tdev_dn10, var_tdev_dn11, var_tdev_dn12, var_tdev_dn13, var_tdev_dn14, var_tdev_dn15, var_tdev_dn16, var_tdev_dn17, var_tdev_dn18, var_tdev_dn19, var_tdev_dn2, var_tdev_dn20, var_tdev_dn21, var_tdev_dn22, var_tdev_dn3, var_tdev_dn4, var_tdev_dn5, var_tdev_dn6, var_tdev_dn7, var_tdev_dn8, var_tdev_dn9, var_tnom, &mut var_guard535, &mut var_guard576, &mut var_qfr, &mut var_qfr3, &mut var_qfr3_db0, &mut var_qfr3_db1, &mut var_qfr3_db10, &mut var_qfr3_db11, &mut var_qfr3_db12, &mut var_qfr3_db13, &mut var_qfr3_db14, &mut var_qfr3_db15, &mut var_qfr3_db16, &mut var_qfr3_db17, &mut var_qfr3_db18, &mut var_qfr3_db19, &mut var_qfr3_db2, &mut var_qfr3_db20, &mut var_qfr3_db21, &mut var_qfr3_db22, &mut var_qfr3_db23, &mut var_qfr3_db24, &mut var_qfr3_db25, &mut var_qfr3_db26, &mut var_qfr3_db27, &mut var_qfr3_db28, &mut var_qfr3_db29, &mut var_qfr3_db3, &mut var_qfr3_db30, &mut var_qfr3_db31, &mut var_qfr3_db32, &mut var_qfr3_db33, &mut var_qfr3_db34, &mut var_qfr3_db35, &mut var_qfr3_db36, &mut var_qfr3_db37, &mut var_qfr3_db38, &mut var_qfr3_db39, &mut var_qfr3_db4, &mut var_qfr3_db40, &mut var_qfr3_db41, &mut var_qfr3_db42, &mut var_qfr3_db43, &mut var_qfr3_db44, &mut var_qfr3_db45, &mut var_qfr3_db46, &mut var_qfr3_db47, &mut var_qfr3_db48, &mut var_qfr3_db49, &mut var_qfr3_db5, &mut var_qfr3_db50, &mut var_qfr3_db51, &mut var_qfr3_db52, &mut var_qfr3_db53, &mut var_qfr3_db54, &mut var_qfr3_db6, &mut var_qfr3_db7, &mut var_qfr3_db8, &mut var_qfr3_db9, &mut var_qfr3_dn0, &mut var_qfr3_dn1, &mut var_qfr3_dn10, &mut var_qfr3_dn11, &mut var_qfr3_dn12, &mut var_qfr3_dn13, &mut var_qfr3_dn14, &mut var_qfr3_dn15, &mut var_qfr3_dn16, &mut var_qfr3_dn17, &mut var_qfr3_dn18, &mut var_qfr3_dn19, &mut var_qfr3_dn2, &mut var_qfr3_dn20, &mut var_qfr3_dn21, &mut var_qfr3_dn22, &mut var_qfr3_dn3, &mut var_qfr3_dn4, &mut var_qfr3_dn5, &mut var_qfr3_dn6, &mut var_qfr3_dn7, &mut var_qfr3_dn8, &mut var_qfr3_dn9, &mut var_qfr_db0, &mut var_qfr_db1, &mut var_qfr_db10, &mut var_qfr_db11, &mut var_qfr_db12, &mut var_qfr_db13, &mut var_qfr_db14, &mut var_qfr_db15, &mut var_qfr_db16, &mut var_qfr_db17, &mut var_qfr_db18, &mut var_qfr_db19, &mut var_qfr_db2, &mut var_qfr_db20, &mut var_qfr_db21, &mut var_qfr_db22, &mut var_qfr_db23, &mut var_qfr_db24, &mut var_qfr_db25, &mut var_qfr_db26, &mut var_qfr_db27, &mut var_qfr_db28, &mut var_qfr_db29, &mut var_qfr_db3, &mut var_qfr_db30, &mut var_qfr_db31, &mut var_qfr_db32, &mut var_qfr_db33, &mut var_qfr_db34, &mut var_qfr_db35, &mut var_qfr_db36, &mut var_qfr_db37, &mut var_qfr_db38, &mut var_qfr_db39, &mut var_qfr_db4, &mut var_qfr_db40, &mut var_qfr_db41, &mut var_qfr_db42, &mut var_qfr_db43, &mut var_qfr_db44, &mut var_qfr_db45, &mut var_qfr_db46, &mut var_qfr_db47, &mut var_qfr_db48, &mut var_qfr_db49, &mut var_qfr_db5, &mut var_qfr_db50, &mut var_qfr_db51, &mut var_qfr_db52, &mut var_qfr_db53, &mut var_qfr_db54, &mut var_qfr_db6, &mut var_qfr_db7, &mut var_qfr_db8, &mut var_qfr_db9, &mut var_qfr_dn0, &mut var_qfr_dn1, &mut var_qfr_dn10, &mut var_qfr_dn11, &mut var_qfr_dn12, &mut var_qfr_dn13, &mut var_qfr_dn14, &mut var_qfr_dn15, &mut var_qfr_dn16, &mut var_qfr_dn17, &mut var_qfr_dn18, &mut var_qfr_dn19, &mut var_qfr_dn2, &mut var_qfr_dn20, &mut var_qfr_dn21, &mut var_qfr_dn22, &mut var_qfr_dn3, &mut var_qfr_dn4, &mut var_qfr_dn5, &mut var_qfr_dn6, &mut var_qfr_dn7, &mut var_qfr_dn8, &mut var_qfr_dn9);

        stamper.stamp_potential_branch_local(
            Some(11),
            None,
            23,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(13),
            None,
            24,
            multiplicity,
        );

        Self::stamp_transient_equations_block_0(ctx, stamper, s, p, nodes, multiplicity, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, var_guard353, var_guard354, var_guard355, var_guard356, var_guard357, var_vdgeff1, var_vdgeff1_db0, var_vdgeff1_db1, var_vdgeff1_db10, var_vdgeff1_db11, var_vdgeff1_db12, var_vdgeff1_db13, var_vdgeff1_db14, var_vdgeff1_db15, var_vdgeff1_db16, var_vdgeff1_db17, var_vdgeff1_db18, var_vdgeff1_db19, var_vdgeff1_db2, var_vdgeff1_db20, var_vdgeff1_db21, var_vdgeff1_db22, var_vdgeff1_db23, var_vdgeff1_db24, var_vdgeff1_db25, var_vdgeff1_db26, var_vdgeff1_db27, var_vdgeff1_db28, var_vdgeff1_db29, var_vdgeff1_db3, var_vdgeff1_db30, var_vdgeff1_db31, var_vdgeff1_db32, var_vdgeff1_db33, var_vdgeff1_db34, var_vdgeff1_db35, var_vdgeff1_db36, var_vdgeff1_db37, var_vdgeff1_db38, var_vdgeff1_db39, var_vdgeff1_db4, var_vdgeff1_db40, var_vdgeff1_db41, var_vdgeff1_db42, var_vdgeff1_db43, var_vdgeff1_db44, var_vdgeff1_db45, var_vdgeff1_db46, var_vdgeff1_db47, var_vdgeff1_db48, var_vdgeff1_db49, var_vdgeff1_db5, var_vdgeff1_db50, var_vdgeff1_db51, var_vdgeff1_db52, var_vdgeff1_db53, var_vdgeff1_db54, var_vdgeff1_db6, var_vdgeff1_db7, var_vdgeff1_db8, var_vdgeff1_db9, var_vdgeff1_dn0, var_vdgeff1_dn1, var_vdgeff1_dn10, var_vdgeff1_dn11, var_vdgeff1_dn12, var_vdgeff1_dn13, var_vdgeff1_dn14, var_vdgeff1_dn15, var_vdgeff1_dn16, var_vdgeff1_dn17, var_vdgeff1_dn18, var_vdgeff1_dn19, var_vdgeff1_dn2, var_vdgeff1_dn20, var_vdgeff1_dn21, var_vdgeff1_dn22, var_vdgeff1_dn3, var_vdgeff1_dn4, var_vdgeff1_dn5, var_vdgeff1_dn6, var_vdgeff1_dn7, var_vdgeff1_dn8, var_vdgeff1_dn9);
        Self::stamp_transient_equations_block_1(ctx, stamper, p, nodes, multiplicity, var_en, var_en_db0, var_en_db1, var_en_db10, var_en_db11, var_en_db12, var_en_db13, var_en_db14, var_en_db15, var_en_db16, var_en_db17, var_en_db18, var_en_db19, var_en_db2, var_en_db20, var_en_db21, var_en_db22, var_en_db23, var_en_db24, var_en_db25, var_en_db26, var_en_db27, var_en_db28, var_en_db29, var_en_db3, var_en_db30, var_en_db31, var_en_db32, var_en_db33, var_en_db34, var_en_db35, var_en_db36, var_en_db37, var_en_db38, var_en_db39, var_en_db4, var_en_db40, var_en_db41, var_en_db42, var_en_db43, var_en_db44, var_en_db45, var_en_db46, var_en_db47, var_en_db48, var_en_db49, var_en_db5, var_en_db50, var_en_db51, var_en_db52, var_en_db53, var_en_db54, var_en_db6, var_en_db7, var_en_db8, var_en_db9, var_en_dn0, var_en_dn1, var_en_dn10, var_en_dn11, var_en_dn12, var_en_dn13, var_en_dn14, var_en_dn15, var_en_dn16, var_en_dn17, var_en_dn18, var_en_dn19, var_en_dn2, var_en_dn20, var_en_dn21, var_en_dn22, var_en_dn3, var_en_dn4, var_en_dn5, var_en_dn6, var_en_dn7, var_en_dn8, var_en_dn9, var_guard353, var_guard354, var_guard355, var_guard356, var_guard357, var_guard358, var_phixn, var_phixn_db0, var_phixn_db1, var_phixn_db10, var_phixn_db11, var_phixn_db12, var_phixn_db13, var_phixn_db14, var_phixn_db15, var_phixn_db16, var_phixn_db17, var_phixn_db18, var_phixn_db19, var_phixn_db2, var_phixn_db20, var_phixn_db21, var_phixn_db22, var_phixn_db23, var_phixn_db24, var_phixn_db25, var_phixn_db26, var_phixn_db27, var_phixn_db28, var_phixn_db29, var_phixn_db3, var_phixn_db30, var_phixn_db31, var_phixn_db32, var_phixn_db33, var_phixn_db34, var_phixn_db35, var_phixn_db36, var_phixn_db37, var_phixn_db38, var_phixn_db39, var_phixn_db4, var_phixn_db40, var_phixn_db41, var_phixn_db42, var_phixn_db43, var_phixn_db44, var_phixn_db45, var_phixn_db46, var_phixn_db47, var_phixn_db48, var_phixn_db49, var_phixn_db5, var_phixn_db50, var_phixn_db51, var_phixn_db52, var_phixn_db53, var_phixn_db54, var_phixn_db6, var_phixn_db7, var_phixn_db8, var_phixn_db9, var_phixn_dn0, var_phixn_dn1, var_phixn_dn10, var_phixn_dn11, var_phixn_dn12, var_phixn_dn13, var_phixn_dn14, var_phixn_dn15, var_phixn_dn16, var_phixn_dn17, var_phixn_dn18, var_phixn_dn19, var_phixn_dn2, var_phixn_dn20, var_phixn_dn21, var_phixn_dn22, var_phixn_dn3, var_phixn_dn4, var_phixn_dn5, var_phixn_dn6, var_phixn_dn7, var_phixn_dn8, var_phixn_dn9);
        Self::stamp_transient_equations_block_2(ctx, stamper, p, nodes, multiplicity, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, var_en, var_en_db0, var_en_db1, var_en_db10, var_en_db11, var_en_db12, var_en_db13, var_en_db14, var_en_db15, var_en_db16, var_en_db17, var_en_db18, var_en_db19, var_en_db2, var_en_db20, var_en_db21, var_en_db22, var_en_db23, var_en_db24, var_en_db25, var_en_db26, var_en_db27, var_en_db28, var_en_db29, var_en_db3, var_en_db30, var_en_db31, var_en_db32, var_en_db33, var_en_db34, var_en_db35, var_en_db36, var_en_db37, var_en_db38, var_en_db39, var_en_db4, var_en_db40, var_en_db41, var_en_db42, var_en_db43, var_en_db44, var_en_db45, var_en_db46, var_en_db47, var_en_db48, var_en_db49, var_en_db5, var_en_db50, var_en_db51, var_en_db52, var_en_db53, var_en_db54, var_en_db6, var_en_db7, var_en_db8, var_en_db9, var_en_dn0, var_en_dn1, var_en_dn10, var_en_dn11, var_en_dn12, var_en_dn13, var_en_dn14, var_en_dn15, var_en_dn16, var_en_dn17, var_en_dn18, var_en_dn19, var_en_dn2, var_en_dn20, var_en_dn21, var_en_dn22, var_en_dn3, var_en_dn4, var_en_dn5, var_en_dn6, var_en_dn7, var_en_dn8, var_en_dn9, var_guard353, var_guard354, var_guard355, var_guard356, var_guard357, var_guard358);
        Self::stamp_transient_equations_block_3(ctx, stamper, p, nodes, multiplicity, var_en1, var_en1_db0, var_en1_db1, var_en1_db10, var_en1_db11, var_en1_db12, var_en1_db13, var_en1_db14, var_en1_db15, var_en1_db16, var_en1_db17, var_en1_db18, var_en1_db19, var_en1_db2, var_en1_db20, var_en1_db21, var_en1_db22, var_en1_db23, var_en1_db24, var_en1_db25, var_en1_db26, var_en1_db27, var_en1_db28, var_en1_db29, var_en1_db3, var_en1_db30, var_en1_db31, var_en1_db32, var_en1_db33, var_en1_db34, var_en1_db35, var_en1_db36, var_en1_db37, var_en1_db38, var_en1_db39, var_en1_db4, var_en1_db40, var_en1_db41, var_en1_db42, var_en1_db43, var_en1_db44, var_en1_db45, var_en1_db46, var_en1_db47, var_en1_db48, var_en1_db49, var_en1_db5, var_en1_db50, var_en1_db51, var_en1_db52, var_en1_db53, var_en1_db54, var_en1_db6, var_en1_db7, var_en1_db8, var_en1_db9, var_en1_dn0, var_en1_dn1, var_en1_dn10, var_en1_dn11, var_en1_dn12, var_en1_dn13, var_en1_dn14, var_en1_dn15, var_en1_dn16, var_en1_dn17, var_en1_dn18, var_en1_dn19, var_en1_dn2, var_en1_dn20, var_en1_dn21, var_en1_dn22, var_en1_dn3, var_en1_dn4, var_en1_dn5, var_en1_dn6, var_en1_dn7, var_en1_dn8, var_en1_dn9, var_guard353, var_guard354, var_guard355, var_guard356, var_guard357, var_guard358, var_phiyn, var_phiyn_db0, var_phiyn_db1, var_phiyn_db10, var_phiyn_db11, var_phiyn_db12, var_phiyn_db13, var_phiyn_db14, var_phiyn_db15, var_phiyn_db16, var_phiyn_db17, var_phiyn_db18, var_phiyn_db19, var_phiyn_db2, var_phiyn_db20, var_phiyn_db21, var_phiyn_db22, var_phiyn_db23, var_phiyn_db24, var_phiyn_db25, var_phiyn_db26, var_phiyn_db27, var_phiyn_db28, var_phiyn_db29, var_phiyn_db3, var_phiyn_db30, var_phiyn_db31, var_phiyn_db32, var_phiyn_db33, var_phiyn_db34, var_phiyn_db35, var_phiyn_db36, var_phiyn_db37, var_phiyn_db38, var_phiyn_db39, var_phiyn_db4, var_phiyn_db40, var_phiyn_db41, var_phiyn_db42, var_phiyn_db43, var_phiyn_db44, var_phiyn_db45, var_phiyn_db46, var_phiyn_db47, var_phiyn_db48, var_phiyn_db49, var_phiyn_db5, var_phiyn_db50, var_phiyn_db51, var_phiyn_db52, var_phiyn_db53, var_phiyn_db54, var_phiyn_db6, var_phiyn_db7, var_phiyn_db8, var_phiyn_db9, var_phiyn_dn0, var_phiyn_dn1, var_phiyn_dn10, var_phiyn_dn11, var_phiyn_dn12, var_phiyn_dn13, var_phiyn_dn14, var_phiyn_dn15, var_phiyn_dn16, var_phiyn_dn17, var_phiyn_dn18, var_phiyn_dn19, var_phiyn_dn2, var_phiyn_dn20, var_phiyn_dn21, var_phiyn_dn22, var_phiyn_dn3, var_phiyn_dn4, var_phiyn_dn5, var_phiyn_dn6, var_phiyn_dn7, var_phiyn_dn8, var_phiyn_dn9);
        Self::stamp_transient_equations_block_4(ctx, stamper, s, p, nodes, multiplicity, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, var_en1, var_en1_db0, var_en1_db1, var_en1_db10, var_en1_db11, var_en1_db12, var_en1_db13, var_en1_db14, var_en1_db15, var_en1_db16, var_en1_db17, var_en1_db18, var_en1_db19, var_en1_db2, var_en1_db20, var_en1_db21, var_en1_db22, var_en1_db23, var_en1_db24, var_en1_db25, var_en1_db26, var_en1_db27, var_en1_db28, var_en1_db29, var_en1_db3, var_en1_db30, var_en1_db31, var_en1_db32, var_en1_db33, var_en1_db34, var_en1_db35, var_en1_db36, var_en1_db37, var_en1_db38, var_en1_db39, var_en1_db4, var_en1_db40, var_en1_db41, var_en1_db42, var_en1_db43, var_en1_db44, var_en1_db45, var_en1_db46, var_en1_db47, var_en1_db48, var_en1_db49, var_en1_db5, var_en1_db50, var_en1_db51, var_en1_db52, var_en1_db53, var_en1_db54, var_en1_db6, var_en1_db7, var_en1_db8, var_en1_db9, var_en1_dn0, var_en1_dn1, var_en1_dn10, var_en1_dn11, var_en1_dn12, var_en1_dn13, var_en1_dn14, var_en1_dn15, var_en1_dn16, var_en1_dn17, var_en1_dn18, var_en1_dn19, var_en1_dn2, var_en1_dn20, var_en1_dn21, var_en1_dn22, var_en1_dn3, var_en1_dn4, var_en1_dn5, var_en1_dn6, var_en1_dn7, var_en1_dn8, var_en1_dn9, var_guard353, var_guard354, var_guard355, var_guard356, var_guard357, var_guard358);
        Self::stamp_transient_equations_block_5(ctx, stamper, s, p, nodes, multiplicity);
        Self::stamp_transient_equations_block_6(ctx, stamper, s, p, nodes, multiplicity);
        Self::stamp_transient_equations_block_7(ctx, stamper, s, p, nodes, multiplicity);
        Self::stamp_transient_equations_block_8(ctx, stamper, s, p, nodes, multiplicity);
        Self::stamp_transient_equations_block_9(ctx, stamper, s, p, nodes, multiplicity);
        Self::stamp_transient_equations_block_10(ctx, stamper, s, p, nodes, multiplicity);
        Self::stamp_transient_equations_block_11(ctx, stamper, s, p, nodes, multiplicity);
        Self::stamp_transient_equations_block_12(ctx, stamper, s, p, nodes, multiplicity);
        Self::stamp_transient_equations_block_13(ctx, stamper, s, p, nodes, multiplicity, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous);
        Self::stamp_transient_equations_block_14(stamper, p, multiplicity, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, var_guard535, var_qdov, var_qdov_db0, var_qdov_db1, var_qdov_db10, var_qdov_db11, var_qdov_db12, var_qdov_db13, var_qdov_db14, var_qdov_db15, var_qdov_db16, var_qdov_db17, var_qdov_db18, var_qdov_db19, var_qdov_db2, var_qdov_db20, var_qdov_db21, var_qdov_db22, var_qdov_db23, var_qdov_db24, var_qdov_db25, var_qdov_db26, var_qdov_db27, var_qdov_db28, var_qdov_db29, var_qdov_db3, var_qdov_db30, var_qdov_db31, var_qdov_db32, var_qdov_db33, var_qdov_db34, var_qdov_db35, var_qdov_db36, var_qdov_db37, var_qdov_db38, var_qdov_db39, var_qdov_db4, var_qdov_db40, var_qdov_db41, var_qdov_db42, var_qdov_db43, var_qdov_db44, var_qdov_db45, var_qdov_db46, var_qdov_db47, var_qdov_db48, var_qdov_db49, var_qdov_db5, var_qdov_db50, var_qdov_db51, var_qdov_db52, var_qdov_db53, var_qdov_db54, var_qdov_db6, var_qdov_db7, var_qdov_db8, var_qdov_db9, var_qdov_dn0, var_qdov_dn1, var_qdov_dn10, var_qdov_dn11, var_qdov_dn12, var_qdov_dn13, var_qdov_dn14, var_qdov_dn15, var_qdov_dn16, var_qdov_dn17, var_qdov_dn18, var_qdov_dn19, var_qdov_dn2, var_qdov_dn20, var_qdov_dn21, var_qdov_dn22, var_qdov_dn3, var_qdov_dn4, var_qdov_dn5, var_qdov_dn6, var_qdov_dn7, var_qdov_dn8, var_qdov_dn9, var_qdsov, var_qdsov_db0, var_qdsov_db1, var_qdsov_db10, var_qdsov_db11, var_qdsov_db12, var_qdsov_db13, var_qdsov_db14, var_qdsov_db15, var_qdsov_db16, var_qdsov_db17, var_qdsov_db18, var_qdsov_db19, var_qdsov_db2, var_qdsov_db20, var_qdsov_db21, var_qdsov_db22, var_qdsov_db23, var_qdsov_db24, var_qdsov_db25, var_qdsov_db26, var_qdsov_db27, var_qdsov_db28, var_qdsov_db29, var_qdsov_db3, var_qdsov_db30, var_qdsov_db31, var_qdsov_db32, var_qdsov_db33, var_qdsov_db34, var_qdsov_db35, var_qdsov_db36, var_qdsov_db37, var_qdsov_db38, var_qdsov_db39, var_qdsov_db4, var_qdsov_db40, var_qdsov_db41, var_qdsov_db42, var_qdsov_db43, var_qdsov_db44, var_qdsov_db45, var_qdsov_db46, var_qdsov_db47, var_qdsov_db48, var_qdsov_db49, var_qdsov_db5, var_qdsov_db50, var_qdsov_db51, var_qdsov_db52, var_qdsov_db53, var_qdsov_db54, var_qdsov_db6, var_qdsov_db7, var_qdsov_db8, var_qdsov_db9, var_qdsov_dn0, var_qdsov_dn1, var_qdsov_dn10, var_qdsov_dn11, var_qdsov_dn12, var_qdsov_dn13, var_qdsov_dn14, var_qdsov_dn15, var_qdsov_dn16, var_qdsov_dn17, var_qdsov_dn18, var_qdsov_dn19, var_qdsov_dn2, var_qdsov_dn20, var_qdsov_dn21, var_qdsov_dn22, var_qdsov_dn3, var_qdsov_dn4, var_qdsov_dn5, var_qdsov_dn6, var_qdsov_dn7, var_qdsov_dn8, var_qdsov_dn9, var_qsov, var_qsov_db0, var_qsov_db1, var_qsov_db10, var_qsov_db11, var_qsov_db12, var_qsov_db13, var_qsov_db14, var_qsov_db15, var_qsov_db16, var_qsov_db17, var_qsov_db18, var_qsov_db19, var_qsov_db2, var_qsov_db20, var_qsov_db21, var_qsov_db22, var_qsov_db23, var_qsov_db24, var_qsov_db25, var_qsov_db26, var_qsov_db27, var_qsov_db28, var_qsov_db29, var_qsov_db3, var_qsov_db30, var_qsov_db31, var_qsov_db32, var_qsov_db33, var_qsov_db34, var_qsov_db35, var_qsov_db36, var_qsov_db37, var_qsov_db38, var_qsov_db39, var_qsov_db4, var_qsov_db40, var_qsov_db41, var_qsov_db42, var_qsov_db43, var_qsov_db44, var_qsov_db45, var_qsov_db46, var_qsov_db47, var_qsov_db48, var_qsov_db49, var_qsov_db5, var_qsov_db50, var_qsov_db51, var_qsov_db52, var_qsov_db53, var_qsov_db54, var_qsov_db6, var_qsov_db7, var_qsov_db8, var_qsov_db9, var_qsov_dn0, var_qsov_dn1, var_qsov_dn10, var_qsov_dn11, var_qsov_dn12, var_qsov_dn13, var_qsov_dn14, var_qsov_dn15, var_qsov_dn16, var_qsov_dn17, var_qsov_dn18, var_qsov_dn19, var_qsov_dn2, var_qsov_dn20, var_qsov_dn21, var_qsov_dn22, var_qsov_dn3, var_qsov_dn4, var_qsov_dn5, var_qsov_dn6, var_qsov_dn7, var_qsov_dn8, var_qsov_dn9);
        Self::stamp_transient_equations_block_15(stamper, s, p, multiplicity, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, var_qbdov, var_qbdov_db0, var_qbdov_db1, var_qbdov_db10, var_qbdov_db11, var_qbdov_db12, var_qbdov_db13, var_qbdov_db14, var_qbdov_db15, var_qbdov_db16, var_qbdov_db17, var_qbdov_db18, var_qbdov_db19, var_qbdov_db2, var_qbdov_db20, var_qbdov_db21, var_qbdov_db22, var_qbdov_db23, var_qbdov_db24, var_qbdov_db25, var_qbdov_db26, var_qbdov_db27, var_qbdov_db28, var_qbdov_db29, var_qbdov_db3, var_qbdov_db30, var_qbdov_db31, var_qbdov_db32, var_qbdov_db33, var_qbdov_db34, var_qbdov_db35, var_qbdov_db36, var_qbdov_db37, var_qbdov_db38, var_qbdov_db39, var_qbdov_db4, var_qbdov_db40, var_qbdov_db41, var_qbdov_db42, var_qbdov_db43, var_qbdov_db44, var_qbdov_db45, var_qbdov_db46, var_qbdov_db47, var_qbdov_db48, var_qbdov_db49, var_qbdov_db5, var_qbdov_db50, var_qbdov_db51, var_qbdov_db52, var_qbdov_db53, var_qbdov_db54, var_qbdov_db6, var_qbdov_db7, var_qbdov_db8, var_qbdov_db9, var_qbdov_dn0, var_qbdov_dn1, var_qbdov_dn10, var_qbdov_dn11, var_qbdov_dn12, var_qbdov_dn13, var_qbdov_dn14, var_qbdov_dn15, var_qbdov_dn16, var_qbdov_dn17, var_qbdov_dn18, var_qbdov_dn19, var_qbdov_dn2, var_qbdov_dn20, var_qbdov_dn21, var_qbdov_dn22, var_qbdov_dn3, var_qbdov_dn4, var_qbdov_dn5, var_qbdov_dn6, var_qbdov_dn7, var_qbdov_dn8, var_qbdov_dn9, var_qbgov, var_qbgov_db0, var_qbgov_db1, var_qbgov_db10, var_qbgov_db11, var_qbgov_db12, var_qbgov_db13, var_qbgov_db14, var_qbgov_db15, var_qbgov_db16, var_qbgov_db17, var_qbgov_db18, var_qbgov_db19, var_qbgov_db2, var_qbgov_db20, var_qbgov_db21, var_qbgov_db22, var_qbgov_db23, var_qbgov_db24, var_qbgov_db25, var_qbgov_db26, var_qbgov_db27, var_qbgov_db28, var_qbgov_db29, var_qbgov_db3, var_qbgov_db30, var_qbgov_db31, var_qbgov_db32, var_qbgov_db33, var_qbgov_db34, var_qbgov_db35, var_qbgov_db36, var_qbgov_db37, var_qbgov_db38, var_qbgov_db39, var_qbgov_db4, var_qbgov_db40, var_qbgov_db41, var_qbgov_db42, var_qbgov_db43, var_qbgov_db44, var_qbgov_db45, var_qbgov_db46, var_qbgov_db47, var_qbgov_db48, var_qbgov_db49, var_qbgov_db5, var_qbgov_db50, var_qbgov_db51, var_qbgov_db52, var_qbgov_db53, var_qbgov_db54, var_qbgov_db6, var_qbgov_db7, var_qbgov_db8, var_qbgov_db9, var_qbgov_dn0, var_qbgov_dn1, var_qbgov_dn10, var_qbgov_dn11, var_qbgov_dn12, var_qbgov_dn13, var_qbgov_dn14, var_qbgov_dn15, var_qbgov_dn16, var_qbgov_dn17, var_qbgov_dn18, var_qbgov_dn19, var_qbgov_dn2, var_qbgov_dn20, var_qbgov_dn21, var_qbgov_dn22, var_qbgov_dn3, var_qbgov_dn4, var_qbgov_dn5, var_qbgov_dn6, var_qbgov_dn7, var_qbgov_dn8, var_qbgov_dn9, var_qbsov, var_qbsov_db0, var_qbsov_db1, var_qbsov_db10, var_qbsov_db11, var_qbsov_db12, var_qbsov_db13, var_qbsov_db14, var_qbsov_db15, var_qbsov_db16, var_qbsov_db17, var_qbsov_db18, var_qbsov_db19, var_qbsov_db2, var_qbsov_db20, var_qbsov_db21, var_qbsov_db22, var_qbsov_db23, var_qbsov_db24, var_qbsov_db25, var_qbsov_db26, var_qbsov_db27, var_qbsov_db28, var_qbsov_db29, var_qbsov_db3, var_qbsov_db30, var_qbsov_db31, var_qbsov_db32, var_qbsov_db33, var_qbsov_db34, var_qbsov_db35, var_qbsov_db36, var_qbsov_db37, var_qbsov_db38, var_qbsov_db39, var_qbsov_db4, var_qbsov_db40, var_qbsov_db41, var_qbsov_db42, var_qbsov_db43, var_qbsov_db44, var_qbsov_db45, var_qbsov_db46, var_qbsov_db47, var_qbsov_db48, var_qbsov_db49, var_qbsov_db5, var_qbsov_db50, var_qbsov_db51, var_qbsov_db52, var_qbsov_db53, var_qbsov_db54, var_qbsov_db6, var_qbsov_db7, var_qbsov_db8, var_qbsov_db9, var_qbsov_dn0, var_qbsov_dn1, var_qbsov_dn10, var_qbsov_dn11, var_qbsov_dn12, var_qbsov_dn13, var_qbsov_dn14, var_qbsov_dn15, var_qbsov_dn16, var_qbsov_dn17, var_qbsov_dn18, var_qbsov_dn19, var_qbsov_dn2, var_qbsov_dn20, var_qbsov_dn21, var_qbsov_dn22, var_qbsov_dn3, var_qbsov_dn4, var_qbsov_dn5, var_qbsov_dn6, var_qbsov_dn7, var_qbsov_dn8, var_qbsov_dn9);
        Self::stamp_transient_equations_block_16(stamper, s, p, multiplicity, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous);
        Self::stamp_transient_equations_block_17(stamper, s, p, multiplicity, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous);
        Self::stamp_transient_equations_block_18(stamper, s, p, multiplicity, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous);
        Self::stamp_transient_equations_block_19(stamper, s, p, multiplicity, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous);
        Self::stamp_transient_equations_block_20(stamper, s, p, multiplicity, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous);
        Self::stamp_transient_equations_block_21(stamper, s, p, multiplicity, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous);
        Self::stamp_transient_equations_block_22(stamper, s, p, multiplicity, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous);
        Self::stamp_transient_equations_block_23(stamper, s, p, multiplicity, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous);
        Self::stamp_transient_equations_block_24(stamper, s, p, multiplicity, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous);
        Self::stamp_transient_equations_block_25(stamper, s, p, multiplicity, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous);
        Self::stamp_transient_equations_block_26(stamper, s, p, multiplicity, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous);
        Self::stamp_transient_equations_block_27(stamper, s, p, multiplicity, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous);
        Self::stamp_transient_equations_block_28(stamper, s, p, multiplicity, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous);
        Self::stamp_transient_equations_block_29(stamper, s, p, multiplicity, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous);
        Self::stamp_transient_equations_block_30(stamper, s, p, multiplicity, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous);
        Self::stamp_transient_equations_block_31(stamper, s, p, multiplicity, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous);
        Self::stamp_transient_equations_block_32(stamper, s, p, multiplicity, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous);
        Self::stamp_transient_equations_block_33(stamper, s, p, multiplicity, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous);
        Self::stamp_transient_equations_block_34(stamper, s, p, multiplicity, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous);
        Self::stamp_transient_equations_block_35(stamper, s, p, multiplicity, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous);
        Self::stamp_transient_equations_block_36(stamper, s, p, multiplicity, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous);
        Self::stamp_transient_equations_block_37(stamper, s, p, multiplicity, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous);
        Self::stamp_transient_equations_block_38(stamper, s, p, multiplicity, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous);
        Self::stamp_transient_equations_block_39(stamper, s, p, multiplicity, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous);
        Self::stamp_transient_equations_block_40(stamper, s, p, multiplicity, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous);
        Self::stamp_transient_equations_block_41(stamper, s, p, multiplicity, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous);
        Self::stamp_transient_equations_block_42(stamper, s, p, multiplicity, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous);
        Self::stamp_transient_equations_block_43(stamper, s, p, multiplicity, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous);
        Self::stamp_transient_equations_block_44(stamper, s, p, multiplicity, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous);
        Self::stamp_transient_equations_block_45(stamper, s, p, multiplicity, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous);
        Self::stamp_transient_equations_block_46(ctx, stamper, s, p, nodes, multiplicity, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, var_qfr, var_qfr_db0, var_qfr_db1, var_qfr_db10, var_qfr_db11, var_qfr_db12, var_qfr_db13, var_qfr_db14, var_qfr_db15, var_qfr_db16, var_qfr_db17, var_qfr_db18, var_qfr_db19, var_qfr_db2, var_qfr_db20, var_qfr_db21, var_qfr_db22, var_qfr_db23, var_qfr_db24, var_qfr_db25, var_qfr_db26, var_qfr_db27, var_qfr_db28, var_qfr_db29, var_qfr_db3, var_qfr_db30, var_qfr_db31, var_qfr_db32, var_qfr_db33, var_qfr_db34, var_qfr_db35, var_qfr_db36, var_qfr_db37, var_qfr_db38, var_qfr_db39, var_qfr_db4, var_qfr_db40, var_qfr_db41, var_qfr_db42, var_qfr_db43, var_qfr_db44, var_qfr_db45, var_qfr_db46, var_qfr_db47, var_qfr_db48, var_qfr_db49, var_qfr_db5, var_qfr_db50, var_qfr_db51, var_qfr_db52, var_qfr_db53, var_qfr_db54, var_qfr_db6, var_qfr_db7, var_qfr_db8, var_qfr_db9, var_qfr_dn0, var_qfr_dn1, var_qfr_dn10, var_qfr_dn11, var_qfr_dn12, var_qfr_dn13, var_qfr_dn14, var_qfr_dn15, var_qfr_dn16, var_qfr_dn17, var_qfr_dn18, var_qfr_dn19, var_qfr_dn2, var_qfr_dn20, var_qfr_dn21, var_qfr_dn22, var_qfr_dn3, var_qfr_dn4, var_qfr_dn5, var_qfr_dn6, var_qfr_dn7, var_qfr_dn8, var_qfr_dn9);
        Self::stamp_transient_equations_block_47(ctx, stamper, s, p, nodes, multiplicity, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, var_guard576, var_qfr3, var_qfr3_db0, var_qfr3_db1, var_qfr3_db10, var_qfr3_db11, var_qfr3_db12, var_qfr3_db13, var_qfr3_db14, var_qfr3_db15, var_qfr3_db16, var_qfr3_db17, var_qfr3_db18, var_qfr3_db19, var_qfr3_db2, var_qfr3_db20, var_qfr3_db21, var_qfr3_db22, var_qfr3_db23, var_qfr3_db24, var_qfr3_db25, var_qfr3_db26, var_qfr3_db27, var_qfr3_db28, var_qfr3_db29, var_qfr3_db3, var_qfr3_db30, var_qfr3_db31, var_qfr3_db32, var_qfr3_db33, var_qfr3_db34, var_qfr3_db35, var_qfr3_db36, var_qfr3_db37, var_qfr3_db38, var_qfr3_db39, var_qfr3_db4, var_qfr3_db40, var_qfr3_db41, var_qfr3_db42, var_qfr3_db43, var_qfr3_db44, var_qfr3_db45, var_qfr3_db46, var_qfr3_db47, var_qfr3_db48, var_qfr3_db49, var_qfr3_db5, var_qfr3_db50, var_qfr3_db51, var_qfr3_db52, var_qfr3_db53, var_qfr3_db54, var_qfr3_db6, var_qfr3_db7, var_qfr3_db8, var_qfr3_db9, var_qfr3_dn0, var_qfr3_dn1, var_qfr3_dn10, var_qfr3_dn11, var_qfr3_dn12, var_qfr3_dn13, var_qfr3_dn14, var_qfr3_dn15, var_qfr3_dn16, var_qfr3_dn17, var_qfr3_dn18, var_qfr3_dn19, var_qfr3_dn2, var_qfr3_dn20, var_qfr3_dn21, var_qfr3_dn22, var_qfr3_dn3, var_qfr3_dn4, var_qfr3_dn5, var_qfr3_dn6, var_qfr3_dn7, var_qfr3_dn8, var_qfr3_dn9);
    }

    pub fn stamp_reactive(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedReactiveStamper<'_>) {
        let p = Box::as_ref(&self.params);
        let nodes = &(*self).nodes;
        let branches = &(*self).branches;
        let param_given = self.param_given.as_ref();
        let multiplicity = (*self).multiplicity;
        let s = match &mut self.reactive_scratch {
            Some(buf) => buf.as_mut(),
            slot @ None => slot.insert(ReactiveScratch::new_box()).as_mut(),
        };

        Self::stamp_reactive_block_0(ctx, s, p, nodes);
        Self::stamp_reactive_block_1(s, p);
        Self::stamp_reactive_block_2(ctx, s, p, nodes, param_given);
        Self::stamp_reactive_block_3(s, p);
        Self::stamp_reactive_block_4(ctx, s, p, nodes);
        Self::stamp_reactive_block_5(ctx, s, p, nodes);
        Self::stamp_reactive_block_6(s, p);
        Self::stamp_reactive_block_7(ctx, s, p, nodes);
        Self::stamp_reactive_block_8(ctx, s, p, nodes);
        Self::stamp_reactive_block_9(s, p);
        Self::stamp_reactive_block_10(ctx, s, p, nodes);
        Self::stamp_reactive_block_11(ctx, s, p, nodes);
        Self::stamp_reactive_block_12(s, p);
        Self::stamp_reactive_block_13(ctx, s, p, nodes);
        Self::stamp_reactive_block_14(ctx, s, p, nodes);
        Self::stamp_reactive_block_15(s, p);
        Self::stamp_reactive_block_16(ctx, s, p, nodes);
        Self::stamp_reactive_block_17(ctx, s, p, nodes);
        Self::stamp_reactive_block_18(s, p);
        Self::stamp_reactive_block_19(ctx, s, p, nodes);
        Self::stamp_reactive_block_20(ctx, s, p, nodes);
        Self::stamp_reactive_block_21(s, p);
        Self::stamp_reactive_block_22(ctx, s, p, nodes);
        Self::stamp_reactive_block_23(ctx, s, p, nodes);
        Self::stamp_reactive_block_24(s, p);
        Self::stamp_reactive_block_25(ctx, s, p, nodes);
        Self::stamp_reactive_block_26(ctx, s, p, nodes);
        Self::stamp_reactive_block_27(ctx, s, p, nodes);

        Self::stamp_reactive_equations_block_0(ctx, stamper, s, p, nodes, branches, multiplicity);
        Self::stamp_reactive_equations_block_1(stamper, s, p, nodes, branches, multiplicity);
        Self::stamp_reactive_equations_block_2(stamper, s, p, nodes, branches, multiplicity);
        Self::stamp_reactive_equations_block_3(stamper, s, p, nodes, branches, multiplicity);
        Self::stamp_reactive_equations_block_4(stamper, s, p, nodes, branches, multiplicity);
        Self::stamp_reactive_equations_block_5(stamper, s, p, nodes, branches, multiplicity);
        Self::stamp_reactive_equations_block_6(stamper, s, p, nodes, branches, multiplicity);
        Self::stamp_reactive_equations_block_7(stamper, s, p, nodes, branches, multiplicity);
        Self::stamp_reactive_equations_block_8(stamper, s, p, nodes, branches, multiplicity);
        Self::stamp_reactive_equations_block_9(stamper, s, p, nodes, branches, multiplicity);
        Self::stamp_reactive_equations_block_10(stamper, s, p, nodes, branches, multiplicity);
        Self::stamp_reactive_equations_block_11(stamper, s, p, nodes, branches, multiplicity);
        Self::stamp_reactive_equations_block_12(ctx, stamper, s, p, nodes, branches, multiplicity);
    }
}
