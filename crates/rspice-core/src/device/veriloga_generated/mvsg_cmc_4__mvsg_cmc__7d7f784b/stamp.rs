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
        let nv20 = ctx.node_voltage(nodes[20]);
        let nv21 = ctx.node_voltage(nodes[21]);
        let nv22 = ctx.node_voltage(nodes[22]);
        let nv23 = ctx.node_voltage(nodes[23]);
        let nv24 = ctx.node_voltage(nodes[24]);
        let nv25 = ctx.node_voltage(nodes[25]);
        let nv26 = ctx.node_voltage(nodes[26]);
        let nv27 = ctx.node_voltage(nodes[27]);
        let nv29 = ctx.node_voltage(nodes[29]);
        let multiplicity = (*self).multiplicity;
        let timestep = (*self).timestep;
        let ddt_state_current = self.ddt_state_current.as_mut();
        let ddt_state_previous = self.ddt_state_previous.as_mut();
        let ddt_state_initialized = self.ddt_state_initialized.as_mut();
        let ddt_active = timestep.abs() > Instance::DDT_EPSILON;
        let ddt_scale = if ddt_active { 1.0 / timestep } else { 0.0 };
        let v0: f64 = 0.0;
        let v1: f64 = 1.0;
        let v52: f64 = nv5;
        let v53: f64 = nv9;
        let v54: f64 = (v52 - v53);
        let v55: f64 = nv8;
        let v56: f64 = nv0;
        let v57: f64 = nv2;
        let v61: f64 = nv1;
        let v62: f64 = (v56 - v61);
        let v63: f64 = v62.abs();
        let v64: f64 = (self.scalar_v60 * v63);
        let v66: f64 = (v62 - self.scalar_v65);
        let v67: f64 = nv21;
        let v69: f64 = (v67 * self.scalar_v68);
        let v70: f64 = (v66 - v69);
        let v72: f64 = (v70 / self.scalar_v71);
        let v73: f64 = 50.0;
        let v74: bool = (v72 > v73);
        let v75: bool = (!v74);
        let v76: f64 = -50.0;
        let v77: bool = (v72 < v76);
        let v78: bool = (!v77);
        let v79: bool = (v75 && v78);
        let v80: f64 = v72.exp();
        let v81: bool = (v75 && v77);
        let v82: f64 = 1.9287498479639178e-22;
        let v83: f64 = 5.184705528587072e21;
        let v84: f64 = (v72 - v73);
        let v85: f64 = (v1 + v84);
        let v86: f64 = (v83 * v85);
        let v87: f64 = (if v74 { v86 } else { v0 });
        let v88: f64 = (if v81 { v82 } else { v87 });
        let v89: f64 = (if v79 { v80 } else { v88 });
        let v90: f64 = (v64 + v89);
        let v91: f64 = (if self.scalar_v59 { v90 } else { v0 });
        let v92: f64 = nv20;
        let v93: f64 = (if self.scalar_v59 { v92 } else { v0 });
        let v98: f64 = nv22;
        let v99: f64 = nv23;
        let v100: f64 = nv25;
        let v101: f64 = nv26;
        let v104: f64 = nv7;
        let v148: f64 = nv29;
        let v153: f64 = (v55 - v104);
        let v164: f64 = (v56 - v57);
        let v171: f64 = nv6;
        let v175: f64 = (-v91);
        let v176: f64 = (if self.scalar_v59 { v175 } else { v0 });
        let v178: f64 = (v67 / self.scalar_v177);
        let v179: f64 = (if self.scalar_v59 { v178 } else { v0 });
        let v181: f64 = (self.scalar_v51 * v164);
        let v182: f64 = (if self.scalar_v97 { v181 } else { v0 });
        let v183: f64 = nv24;
        let v184: f64 = (v98 - v183);
        let v186: f64 = (v184 / self.scalar_v185);
        let v187: f64 = (if self.scalar_v97 { v186 } else { v0 });
        let v188: f64 = (v98 - v99);
        let v190: f64 = (v188 / self.scalar_v189);
        let v191: f64 = (if self.scalar_v97 { v190 } else { v0 });
        let v192: f64 = (v61 - v57);
        let v193: f64 = (self.scalar_v51 * v192);
        let v194: f64 = (if self.scalar_v97 { v193 } else { v0 });
        let v195: f64 = nv27;
        let v196: f64 = (v100 - v195);
        let v197: f64 = (v196 / self.scalar_v185);
        let v198: f64 = (if self.scalar_v97 { v197 } else { v0 });
        let v199: f64 = (v100 - v101);
        let v200: f64 = (v199 / self.scalar_v189);
        let v201: f64 = (if self.scalar_v97 { v200 } else { v0 });
        let v243: f64 = (v0 * v54);
        let v244: f64 = (v148 + v243);
        let v245: f64 = (if self.scalar_v242 { v244 } else { v0 });
        let v246: f64 = (v153 / self.scalar_v163);
        let v247: f64 = (if self.scalar_v159 { v246 } else { v0 });
        let v252: f64 = (v61 - v171);
        let v253: f64 = (v252 / self.scalar_v46);
        let v254: f64 = (if self.scalar_v167 { v253 } else { v0 });
        let v257: f64 = (v171 - v104);
        let v258: f64 = (v257 / self.scalar_v50);
        let v259: f64 = (if self.scalar_v170 { v258 } else { v0 });
        let v270: f64 = (v80 * self.scalar_v267);
        let v271: f64 = (v80 * self.scalar_v268);
        let v272: f64 = (v80 * self.scalar_v269);
        let v276: f64 = (if v74 { self.scalar_v273 } else { v0 });
        let v277: f64 = (if v74 { self.scalar_v274 } else { v0 });
        let v278: f64 = (if v74 { self.scalar_v275 } else { v0 });
        let v279: f64 = (if v81 { v0 } else { v276 });
        let v280: f64 = (if v81 { v0 } else { v277 });
        let v281: f64 = (if v81 { v0 } else { v278 });
        let v282: f64 = (if v79 { v270 } else { v279 });
        let v283: f64 = (if v79 { v271 } else { v280 });
        let v284: f64 = (if v79 { v272 } else { v281 });
        let v285: f64 = (if self.scalar_v59 { v282 } else { v0 });
        let v286: f64 = (if self.scalar_v59 { v283 } else { v0 });
        let v287: f64 = (if self.scalar_v59 { v284 } else { v0 });
        let v289: f64 = (-v285);
        let v290: f64 = (-v286);
        let v291: f64 = (-v287);
        let v292: f64 = (if self.scalar_v59 { v289 } else { v0 });
        let v293: f64 = (if self.scalar_v59 { v290 } else { v0 });
        let v294: f64 = (if self.scalar_v59 { v291 } else { v0 });

        stamper.stamp_potential_branch_local(
            Some(22),
            None,
            0,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            0,
            self.scalar_v174,
        );
        stamper.stamp_potential_branch_local(
            Some(23),
            None,
            1,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            1,
            self.scalar_v174,
        );
        stamper.stamp_potential_branch_local(
            Some(24),
            None,
            2,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            2,
            self.scalar_v174,
        );
        stamper.stamp_potential_branch_local(
            Some(25),
            None,
            3,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            3,
            self.scalar_v174,
        );
        stamper.stamp_potential_branch_local(
            Some(26),
            None,
            4,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            4,
            self.scalar_v174,
        );
        stamper.stamp_potential_branch_local(
            Some(27),
            None,
            5,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            5,
            self.scalar_v174,
        );
        let d176_dn0: f64 = v292;
        let d176_dn1: f64 = v293;
        let d176_dn21: f64 = v294;
        stamper.stamp_current_node3_local(
            Some(21),
            None,
            multiplicity * (v176),
            0,
            multiplicity * (d176_dn0),
            1,
            multiplicity * (d176_dn1),
            21,
            multiplicity * (d176_dn21),
        );
        let d179_dn21: f64 = self.scalar_v296;
        stamper.stamp_current_node1_local(
            Some(21),
            None,
            multiplicity * (v179),
            21,
            multiplicity * (d179_dn21),
        );
        let d93_dn20: f64 = self.scalar_v288;
        stamper.stamp_current_node1_local(
            Some(20),
            None,
            multiplicity * (v93),
            20,
            multiplicity * (d93_dn20),
        );
        stamper.stamp_potential_branch_local(
            Some(21),
            None,
            6,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            6,
            self.scalar_v180,
        );
        stamper.stamp_potential_branch_local(
            Some(20),
            None,
            7,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            7,
            self.scalar_v180,
        );
        let d182_dn0: f64 = self.scalar_v297;
        let d182_dn2: f64 = self.scalar_v298;
        stamper.stamp_potential_branch_local(
            Some(22),
            None,
            8,
            multiplicity,
        );
        stamper.stamp_potential_node2_local(
            8,
            v182,
            0,
            d182_dn0,
            2,
            d182_dn2,
        );
        let d187_dn22: f64 = self.scalar_v301;
        let d187_dn24: f64 = self.scalar_v302;
        stamper.stamp_current_node2_local(
            Some(22),
            Some(24),
            multiplicity * (v187),
            22,
            multiplicity * (d187_dn22),
            24,
            multiplicity * (d187_dn24),
        );
        let d191_dn22: f64 = self.scalar_v305;
        let d191_dn23: f64 = self.scalar_v306;
        stamper.stamp_current_node2_local(
            Some(22),
            Some(23),
            multiplicity * (v191),
            22,
            multiplicity * (d191_dn22),
            23,
            multiplicity * (d191_dn23),
        );
        let d194_dn1: f64 = self.scalar_v297;
        let d194_dn2: f64 = self.scalar_v298;
        stamper.stamp_potential_branch_local(
            Some(25),
            None,
            9,
            multiplicity,
        );
        stamper.stamp_potential_node2_local(
            9,
            v194,
            1,
            d194_dn1,
            2,
            d194_dn2,
        );
        let d198_dn25: f64 = self.scalar_v301;
        let d198_dn27: f64 = self.scalar_v302;
        stamper.stamp_current_node2_local(
            Some(25),
            Some(27),
            multiplicity * (v198),
            25,
            multiplicity * (d198_dn25),
            27,
            multiplicity * (d198_dn27),
        );
        let d201_dn25: f64 = self.scalar_v305;
        let d201_dn26: f64 = self.scalar_v306;
        stamper.stamp_current_node2_local(
            Some(25),
            Some(26),
            multiplicity * (v201),
            25,
            multiplicity * (d201_dn25),
            26,
            multiplicity * (d201_dn26),
        );
        stamper.stamp_potential_branch_local(
            Some(21),
            None,
            10,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            10,
            self.scalar_v204,
        );
        stamper.stamp_potential_branch_local(
            Some(20),
            None,
            11,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            11,
            self.scalar_v204,
        );
        stamper.stamp_potential_branch_local(
            Some(22),
            None,
            12,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            12,
            self.scalar_v204,
        );
        stamper.stamp_potential_branch_local(
            Some(23),
            None,
            13,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            13,
            self.scalar_v204,
        );
        stamper.stamp_potential_branch_local(
            Some(24),
            None,
            14,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            14,
            self.scalar_v204,
        );
        stamper.stamp_potential_branch_local(
            Some(25),
            None,
            15,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            15,
            self.scalar_v204,
        );
        stamper.stamp_potential_branch_local(
            Some(26),
            None,
            16,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            16,
            self.scalar_v204,
        );
        stamper.stamp_potential_branch_local(
            Some(27),
            None,
            17,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            17,
            self.scalar_v204,
        );
        stamper.stamp_potential_branch_local(
            Some(17),
            Some(16),
            18,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            18,
            self.scalar_v206,
        );
        stamper.stamp_current_const_local(
            Some(2),
            Some(17),
            multiplicity * (self.scalar_v207),
        );
        stamper.stamp_current_const_local(
            Some(7),
            Some(17),
            multiplicity * (self.scalar_v208),
        );
        stamper.stamp_current_const_local(
            Some(7),
            Some(9),
            multiplicity * (self.scalar_v208),
        );
        stamper.stamp_potential_branch_local(
            Some(16),
            Some(15),
            19,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            19,
            self.scalar_v210,
        );
        stamper.stamp_current_const_local(
            Some(2),
            Some(16),
            multiplicity * (self.scalar_v211),
        );
        stamper.stamp_current_const_local(
            Some(7),
            Some(16),
            multiplicity * (self.scalar_v212),
        );
        stamper.stamp_current_const_local(
            Some(7),
            Some(9),
            multiplicity * (self.scalar_v212),
        );
        stamper.stamp_potential_branch_local(
            Some(15),
            Some(14),
            20,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            20,
            self.scalar_v214,
        );
        stamper.stamp_current_const_local(
            Some(2),
            Some(15),
            multiplicity * (self.scalar_v215),
        );
        stamper.stamp_current_const_local(
            Some(7),
            Some(15),
            multiplicity * (self.scalar_v216),
        );
        stamper.stamp_current_const_local(
            Some(7),
            Some(9),
            multiplicity * (self.scalar_v216),
        );
        stamper.stamp_potential_branch_local(
            Some(14),
            Some(5),
            21,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            21,
            self.scalar_v218,
        );
        stamper.stamp_current_const_local(
            Some(2),
            Some(14),
            multiplicity * (self.scalar_v219),
        );
        stamper.stamp_current_const_local(
            Some(7),
            Some(14),
            multiplicity * (self.scalar_v220),
        );
        stamper.stamp_current_const_local(
            Some(7),
            Some(9),
            multiplicity * (self.scalar_v220),
        );
        stamper.stamp_potential_branch_local(
            Some(9),
            Some(10),
            22,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            22,
            self.scalar_v222,
        );
        stamper.stamp_current_const_local(
            Some(2),
            Some(9),
            multiplicity * (self.scalar_v223),
        );
        stamper.stamp_current_const_local(
            Some(7),
            Some(9),
            multiplicity * (self.scalar_v224),
        );
        stamper.stamp_current_const_local(
            Some(7),
            Some(9),
            multiplicity * (self.scalar_v224),
        );
        stamper.stamp_potential_branch_local(
            Some(10),
            Some(11),
            23,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            23,
            self.scalar_v226,
        );
        stamper.stamp_current_const_local(
            Some(2),
            Some(10),
            multiplicity * (self.scalar_v227),
        );
        stamper.stamp_current_const_local(
            Some(7),
            Some(10),
            multiplicity * (self.scalar_v228),
        );
        stamper.stamp_current_const_local(
            Some(7),
            Some(9),
            multiplicity * (self.scalar_v228),
        );
        stamper.stamp_potential_branch_local(
            Some(11),
            Some(12),
            24,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            24,
            self.scalar_v230,
        );
        stamper.stamp_current_const_local(
            Some(2),
            Some(11),
            multiplicity * (self.scalar_v231),
        );
        stamper.stamp_current_const_local(
            Some(7),
            Some(11),
            multiplicity * (self.scalar_v232),
        );
        stamper.stamp_current_const_local(
            Some(7),
            Some(9),
            multiplicity * (self.scalar_v232),
        );
        stamper.stamp_potential_branch_local(
            Some(12),
            Some(13),
            25,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            25,
            self.scalar_v234,
        );
        stamper.stamp_current_const_local(
            Some(2),
            Some(12),
            multiplicity * (self.scalar_v235),
        );
        stamper.stamp_current_const_local(
            Some(7),
            Some(12),
            multiplicity * (self.scalar_v236),
        );
        stamper.stamp_current_const_local(
            Some(7),
            Some(9),
            multiplicity * (self.scalar_v236),
        );
        stamper.stamp_potential_branch_local(
            Some(13),
            Some(19),
            26,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            26,
            self.scalar_v238,
        );
        stamper.stamp_potential_branch_local(
            Some(18),
            Some(17),
            27,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            27,
            self.scalar_v240,
        );
        stamper.stamp_potential_branch_local(
            Some(28),
            None,
            28,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            28,
            self.scalar_v241,
        );
        stamper.stamp_potential_branch_local(
            Some(29),
            None,
            29,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            29,
            self.scalar_v241,
        );
        let d245_dn5: f64 = self.scalar_v308;
        let d245_dn9: f64 = self.scalar_v309;
        let d245_dn29: f64 = self.scalar_v310;
        stamper.stamp_current_node3_local(
            Some(5),
            Some(9),
            multiplicity * (v245),
            5,
            multiplicity * (d245_dn5),
            9,
            multiplicity * (d245_dn9),
            29,
            multiplicity * (d245_dn29),
        );
        let d247_dn7: f64 = self.scalar_v313;
        let d247_dn8: f64 = self.scalar_v314;
        stamper.stamp_current_node2_local(
            Some(8),
            Some(7),
            multiplicity * (v247),
            7,
            multiplicity * (d247_dn7),
            8,
            multiplicity * (d247_dn8),
        );
        stamper.stamp_potential_branch_local(
            Some(7),
            Some(8),
            30,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            30,
            self.scalar_v249,
        );
        stamper.stamp_potential_branch_local(
            Some(0),
            Some(18),
            31,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            31,
            self.scalar_v250,
        );
        stamper.stamp_potential_branch_local(
            Some(19),
            Some(2),
            32,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            32,
            self.scalar_v251,
        );
        let d254_dn1: f64 = self.scalar_v317;
        let d254_dn6: f64 = self.scalar_v318;
        stamper.stamp_current_node2_local(
            Some(1),
            Some(6),
            multiplicity * (v254),
            1,
            multiplicity * (d254_dn1),
            6,
            multiplicity * (d254_dn6),
        );
        stamper.stamp_potential_branch_local(
            Some(1),
            Some(6),
            33,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            33,
            self.scalar_v256,
        );
        let d259_dn6: f64 = self.scalar_v321;
        let d259_dn7: f64 = self.scalar_v322;
        stamper.stamp_current_node2_local(
            Some(6),
            Some(7),
            multiplicity * (v259),
            6,
            multiplicity * (d259_dn6),
            7,
            multiplicity * (d259_dn7),
        );
        stamper.stamp_potential_branch_local(
            Some(6),
            Some(7),
            34,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            34,
            self.scalar_v261,
        );
        stamper.stamp_potential_branch_local(
            Some(4),
            None,
            35,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            35,
            self.scalar_v263,
        );
        let s = match &mut self.scratch {
            Some(buf) => buf.as_mut(),
            slot @ None => slot.insert(Scratch::new_box()).as_mut(),
        };

        Self::stamp_transient_block_0(ctx, s, p, nodes);
        Self::stamp_transient_block_1(ctx, s, p, nodes);
        Self::stamp_transient_block_2(s, p);
        Self::stamp_transient_block_3(s, p);
        Self::stamp_transient_block_4(s, p);
        Self::stamp_transient_block_5(s, p);
        Self::stamp_transient_block_6(s, p);
        Self::stamp_transient_block_7(s, p);
        Self::stamp_transient_block_8(s, p);
        Self::stamp_transient_block_9(s, p);
        Self::stamp_transient_block_10(s, p);
        Self::stamp_transient_block_11(s, p);
        Self::stamp_transient_block_12(s, p);
        Self::stamp_transient_block_13(s, p);
        Self::stamp_transient_block_14(s, p);
        Self::stamp_transient_block_15(s, p);
        Self::stamp_transient_block_16(s, p);
        Self::stamp_transient_block_17(s, p);
        Self::stamp_transient_block_18(s, p);
        Self::stamp_transient_block_19(s, p);
        Self::stamp_transient_block_20(s, p);
        Self::stamp_transient_block_21(s, p);
        Self::stamp_transient_block_22(s, p);
        Self::stamp_transient_block_23(s, p);
        Self::stamp_transient_block_24(ctx, s, p, nodes);
        Self::stamp_transient_block_25(ctx, s, p, nodes);
        Self::stamp_transient_block_26(ctx, s, p, nodes);
        Self::stamp_transient_block_27(s, p);
        Self::stamp_transient_block_28(ctx, s, p, nodes);
        Self::stamp_transient_block_29(ctx, s, p, nodes);
        Self::stamp_transient_block_30(ctx, s, p, nodes);
        Self::stamp_transient_block_31(ctx, s, p, nodes);
        Self::stamp_transient_block_32(ctx, s, p, nodes);
        Self::stamp_transient_block_33(ctx, s, p, nodes);
        Self::stamp_transient_block_34(s, p);
        Self::stamp_transient_block_35(s, p);
        Self::stamp_transient_block_36(ctx, s, p, nodes);
        Self::stamp_transient_block_37(s, p);
        Self::stamp_transient_block_38(ctx, s, p, nodes);

        Self::stamp_transient_equations_block_0(ctx, stamper, s, p, nodes, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_1(ctx, stamper, s, p, nodes, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_2(ctx, stamper, s, p, nodes, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_3(ctx, stamper, s, p, nodes, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_4(ctx, stamper, s, p, nodes, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_5(ctx, stamper, s, p, nodes, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_6(ctx, stamper, s, p, nodes, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_7(ctx, stamper, s, p, nodes, multiplicity);
    }

    pub fn stamp_reactive(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedReactiveStamper<'_>) {
        let p = Box::as_ref(&self.params);
        let nodes = &(*self).nodes;
        let branches = &(*self).branches;
        let multiplicity = (*self).multiplicity;
        let s = match &mut self.reactive_scratch {
            Some(buf) => buf.as_mut(),
            slot @ None => slot.insert(ReactiveScratch::new_box()).as_mut(),
        };

        Self::stamp_reactive_block_0(ctx, s, p, nodes);
        Self::stamp_reactive_block_1(ctx, s, p, nodes);
        Self::stamp_reactive_block_2(s, p);
        Self::stamp_reactive_block_3(s, p);
        Self::stamp_reactive_block_4(s, p);
        Self::stamp_reactive_block_5(s, p);
        Self::stamp_reactive_block_6(s, p);
        Self::stamp_reactive_block_7(s, p);
        Self::stamp_reactive_block_8(s, p);
        Self::stamp_reactive_block_9(s, p);
        Self::stamp_reactive_block_10(s, p);
        Self::stamp_reactive_block_11(s, p);
        Self::stamp_reactive_block_12(s, p);
        Self::stamp_reactive_block_13(s, p);
        Self::stamp_reactive_block_14(s, p);
        Self::stamp_reactive_block_15(s, p);
        Self::stamp_reactive_block_16(s, p);
        Self::stamp_reactive_block_17(s, p);
        Self::stamp_reactive_block_18(s, p);
        Self::stamp_reactive_block_19(s, p);
        Self::stamp_reactive_block_20(ctx, s, p, nodes);

        Self::stamp_reactive_equations_block_0(ctx, stamper, s, p, nodes, multiplicity);
        Self::stamp_reactive_equations_block_1(stamper, s, p, nodes, branches, multiplicity);
        Self::stamp_reactive_equations_block_2(stamper, s, p, nodes, branches, multiplicity);
        Self::stamp_reactive_equations_block_3(ctx, stamper, s, p, nodes, branches, multiplicity);
        Self::stamp_reactive_equations_block_4(ctx, stamper, s, p, nodes, branches, multiplicity);
        Self::stamp_reactive_equations_block_5(ctx, stamper, s, p, nodes, branches, multiplicity);
        Self::stamp_reactive_equations_block_6(ctx, stamper, s, p, nodes, branches, multiplicity);
    }
}
