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
#[path = "stamp_blocks_8.rs"]
mod stamp_blocks_8;
#[path = "stamp_blocks_9.rs"]
mod stamp_blocks_9;
#[path = "stamp_blocks_10.rs"]
mod stamp_blocks_10;
#[path = "stamp_blocks_11.rs"]
mod stamp_blocks_11;

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
        let ddt_state_initialized = self.ddt_state_initialized.as_mut();
        let ddt_active = timestep.abs() > Instance::DDT_EPSILON;
        let ddt_scale = if ddt_active { 1.0 / timestep } else { 0.0 };
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
        let v75: f64 = v74.sqrt();
        let v76: f64 = (v67 + v75);
        let v77: f64 = (v44 * v76);
        let v78: f64 = (if self.scalar_v51 { v77 } else { v0 });
        let v84: f64 = nv11;
        let v85: f64 = nv12;
        let v86: f64 = (v84 - v85);
        let v88: f64 = (v86 / self.scalar_v87);
        let v89: f64 = v88.exp();
        let v90: f64 = (self.scalar_v83 * v89);
        let v91: f64 = (v1 + v90);
        let v92: f64 = (self.scalar_v82 / v91);
        let v93: f64 = (if self.scalar_v81 { v92 } else { v0 });
        let v96: f64 = nv13;
        let v97: f64 = nv14;
        let v98: f64 = (v96 - v97);
        let v100: f64 = (v98 / self.scalar_v99);
        let v101: f64 = v100.exp();
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

        Self::stamp_transient_block_0(ctx, s, p, nodes);
        Self::stamp_transient_block_1(ctx, s, p, nodes);
        Self::stamp_transient_block_2(ctx, s, p, nodes);
        Self::stamp_transient_block_3(ctx, s, p, nodes, param_given);
        Self::stamp_transient_block_4(ctx, s, p, nodes);
        Self::stamp_transient_block_5(ctx, s, p, nodes);
        Self::stamp_transient_block_6(s, p);
        Self::stamp_transient_block_7(ctx, s, p, nodes);
        Self::stamp_transient_block_8(ctx, s, p, nodes);
        Self::stamp_transient_block_9(ctx, s, p, nodes);
        Self::stamp_transient_block_10(ctx, s, p, nodes);
        Self::stamp_transient_block_11(ctx, s, p, nodes);
        Self::stamp_transient_block_12(ctx, s, p, nodes);
        Self::stamp_transient_block_13(ctx, s, p, nodes);
        Self::stamp_transient_block_14(ctx, s, p, nodes);
        Self::stamp_transient_block_15(ctx, s, p, nodes);
        Self::stamp_transient_block_16(ctx, s, p, nodes);
        Self::stamp_transient_block_17(ctx, s, p, nodes);
        Self::stamp_transient_block_18(ctx, s, p, nodes);
        Self::stamp_transient_block_19(ctx, s, p, nodes);
        Self::stamp_transient_block_20(ctx, s, p, nodes);
        Self::stamp_transient_block_21(ctx, s, p, nodes);
        Self::stamp_transient_block_22(ctx, s, p, nodes);
        Self::stamp_transient_block_23(ctx, s, p, nodes);
        Self::stamp_transient_block_24(ctx, s, p, nodes);
        Self::stamp_transient_block_25(ctx, s, p, nodes);
        Self::stamp_transient_block_26(ctx, s, p, nodes);
        Self::stamp_transient_block_27(ctx, s, p, nodes);
        Self::stamp_transient_block_28(ctx, s, p, nodes);

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

        Self::stamp_transient_equations_block_0(ctx, stamper, s, p, nodes, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_1(ctx, stamper, s, p, nodes, multiplicity);
        Self::stamp_transient_equations_block_2(ctx, stamper, s, p, nodes, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_3(ctx, stamper, s, p, nodes, multiplicity);
        Self::stamp_transient_equations_block_4(ctx, stamper, s, p, nodes, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_5(ctx, stamper, s, p, nodes, multiplicity);
        Self::stamp_transient_equations_block_6(stamper, s, p, multiplicity);
        Self::stamp_transient_equations_block_7(ctx, stamper, s, p, nodes, multiplicity);
        Self::stamp_transient_equations_block_8(ctx, stamper, s, p, nodes, multiplicity);
        Self::stamp_transient_equations_block_9(ctx, stamper, s, p, nodes, multiplicity);
        Self::stamp_transient_equations_block_10(ctx, stamper, s, p, nodes, multiplicity);
        Self::stamp_transient_equations_block_11(ctx, stamper, s, p, nodes, multiplicity);
        Self::stamp_transient_equations_block_12(ctx, stamper, s, p, nodes, multiplicity);
        Self::stamp_transient_equations_block_13(ctx, stamper, s, p, nodes, multiplicity);
        Self::stamp_transient_equations_block_14(ctx, stamper, s, p, nodes, multiplicity);
        Self::stamp_transient_equations_block_15(ctx, stamper, s, p, nodes, multiplicity);
        Self::stamp_transient_equations_block_16(ctx, stamper, s, p, nodes, multiplicity);
        Self::stamp_transient_equations_block_17(ctx, stamper, s, p, nodes, multiplicity);
        Self::stamp_transient_equations_block_18(stamper, s, p, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_19(stamper, s, p, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_20(stamper, s, p, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_21(stamper, s, p, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_22(stamper, s, p, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_23(stamper, s, p, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_24(stamper, s, p, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_25(stamper, s, p, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_26(stamper, s, p, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_27(stamper, s, p, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_28(stamper, s, p, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_29(stamper, s, p, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_30(stamper, s, p, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_31(stamper, s, p, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_32(stamper, s, p, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_33(stamper, s, p, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_34(stamper, s, p, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_35(stamper, s, p, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_36(stamper, s, p, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_37(stamper, s, p, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_38(stamper, s, p, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_39(stamper, s, p, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_40(stamper, s, p, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_41(stamper, s, p, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_42(stamper, s, p, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_43(stamper, s, p, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_44(stamper, s, p, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_45(stamper, s, p, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_46(stamper, s, p, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_47(stamper, s, p, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_48(stamper, s, p, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_49(stamper, s, p, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_50(stamper, s, p, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_51(stamper, s, p, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_52(stamper, s, p, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_53(stamper, s, p, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_54(stamper, s, p, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_55(stamper, s, p, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_56(stamper, s, p, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_57(stamper, s, p, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_58(stamper, s, p, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_59(stamper, s, p, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_60(stamper, s, p, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_61(stamper, s, p, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_62(stamper, s, p, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_63(stamper, s, p, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_64(stamper, s, p, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_65(stamper, s, p, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_66(stamper, s, p, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_67(stamper, s, p, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_68(stamper, s, p, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_69(stamper, s, p, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_70(stamper, s, p, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_71(ctx, stamper, s, p, nodes, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_72(ctx, stamper, s, p, nodes, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
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
        Self::stamp_reactive_equations_block_12(stamper, s, p, nodes, branches, multiplicity);
        Self::stamp_reactive_equations_block_13(stamper, s, p, nodes, branches, multiplicity);
        Self::stamp_reactive_equations_block_14(stamper, s, p, nodes, branches, multiplicity);
        Self::stamp_reactive_equations_block_15(stamper, s, p, nodes, branches, multiplicity);
        Self::stamp_reactive_equations_block_16(stamper, s, p, nodes, branches, multiplicity);
        Self::stamp_reactive_equations_block_17(stamper, s, p, nodes, branches, multiplicity);
        Self::stamp_reactive_equations_block_18(stamper, s, p, nodes, branches, multiplicity);
        Self::stamp_reactive_equations_block_19(stamper, s, p, nodes, branches, multiplicity);
        Self::stamp_reactive_equations_block_20(stamper, s, p, nodes, branches, multiplicity);
        Self::stamp_reactive_equations_block_21(stamper, s, p, nodes, branches, multiplicity);
        Self::stamp_reactive_equations_block_22(stamper, s, p, nodes, branches, multiplicity);
        Self::stamp_reactive_equations_block_23(stamper, s, p, nodes, branches, multiplicity);
        Self::stamp_reactive_equations_block_24(stamper, s, p, nodes, branches, multiplicity);
        Self::stamp_reactive_equations_block_25(stamper, s, p, nodes, branches, multiplicity);
        Self::stamp_reactive_equations_block_26(stamper, s, p, nodes, branches, multiplicity);
        Self::stamp_reactive_equations_block_27(stamper, s, p, nodes, branches, multiplicity);
        Self::stamp_reactive_equations_block_28(stamper, s, p, nodes, branches, multiplicity);
        Self::stamp_reactive_equations_block_29(stamper, s, p, nodes, branches, multiplicity);
        Self::stamp_reactive_equations_block_30(stamper, s, p, nodes, branches, multiplicity);
        Self::stamp_reactive_equations_block_31(stamper, s, p, nodes, branches, multiplicity);
        Self::stamp_reactive_equations_block_32(stamper, s, p, nodes, branches, multiplicity);
        Self::stamp_reactive_equations_block_33(stamper, s, p, nodes, branches, multiplicity);
        Self::stamp_reactive_equations_block_34(stamper, s, p, nodes, branches, multiplicity);
        Self::stamp_reactive_equations_block_35(stamper, s, p, nodes, branches, multiplicity);
        Self::stamp_reactive_equations_block_36(stamper, s, p, nodes, branches, multiplicity);
        Self::stamp_reactive_equations_block_37(stamper, s, p, nodes, branches, multiplicity);
        Self::stamp_reactive_equations_block_38(stamper, s, p, nodes, branches, multiplicity);
        Self::stamp_reactive_equations_block_39(stamper, s, p, nodes, branches, multiplicity);
        Self::stamp_reactive_equations_block_40(stamper, s, p, nodes, branches, multiplicity);
        Self::stamp_reactive_equations_block_41(stamper, s, p, nodes, branches, multiplicity);
        Self::stamp_reactive_equations_block_42(stamper, s, p, nodes, branches, multiplicity);
        Self::stamp_reactive_equations_block_43(stamper, s, p, nodes, branches, multiplicity);
        Self::stamp_reactive_equations_block_44(stamper, s, p, nodes, branches, multiplicity);
        Self::stamp_reactive_equations_block_45(stamper, s, p, nodes, branches, multiplicity);
        Self::stamp_reactive_equations_block_46(stamper, s, p, nodes, branches, multiplicity);
        Self::stamp_reactive_equations_block_47(stamper, s, p, nodes, branches, multiplicity);
        Self::stamp_reactive_equations_block_48(stamper, s, p, nodes, branches, multiplicity);
        Self::stamp_reactive_equations_block_49(stamper, s, p, nodes, branches, multiplicity);
        Self::stamp_reactive_equations_block_50(stamper, s, p, nodes, branches, multiplicity);
        Self::stamp_reactive_equations_block_51(stamper, s, p, nodes, branches, multiplicity);
        Self::stamp_reactive_equations_block_52(stamper, s, p, nodes, branches, multiplicity);
        Self::stamp_reactive_equations_block_53(ctx, stamper, s, p, nodes, branches, multiplicity);
        Self::stamp_reactive_equations_block_54(stamper, s, p, nodes, branches, multiplicity);
        Self::stamp_reactive_equations_block_55(ctx, stamper, s, p, nodes, multiplicity);
    }
}
