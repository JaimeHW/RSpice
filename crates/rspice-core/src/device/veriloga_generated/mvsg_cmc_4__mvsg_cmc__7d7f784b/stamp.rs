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
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv20 = ctx.node_voltage(nodes[20]);
        let nv21 = ctx.node_voltage(nodes[21]);
        let nv22 = ctx.node_voltage(nodes[22]);
        let nv23 = ctx.node_voltage(nodes[23]);
        let nv24 = ctx.node_voltage(nodes[24]);
        let nv25 = ctx.node_voltage(nodes[25]);
        let nv26 = ctx.node_voltage(nodes[26]);
        let nv27 = ctx.node_voltage(nodes[27]);
        let multiplicity = (*self).multiplicity;
        let timestep = (*self).timestep;
        let ddt_state_current = self.ddt_state_current.as_mut();
        let ddt_state_previous = self.ddt_state_previous.as_mut();
        let ddt_state_initialized = self.ddt_state_initialized.as_mut();
        let ddt_active = timestep.abs() > Instance::DDT_EPSILON;
        let ddt_scale = if ddt_active { 1.0 / timestep } else { 0.0 };
        let v0: f64 = 0.0;
        let v17: f64 = 1.0;
        let v23: f64 = nv8;
        let v24: f64 = nv0;
        let v29: f64 = nv1;
        let v30: f64 = (v24 - v29);
        let v31: f64 = v30.abs();
        let v32: f64 = (self.scalar_v28 * v31);
        let v34: f64 = (v30 - self.scalar_v33);
        let v35: f64 = nv21;
        let v37: f64 = (v35 * self.scalar_v36);
        let v38: f64 = (v34 - v37);
        let v40: f64 = (v38 / self.scalar_v39);
        let v41: f64 = 50.0;
        let v42: bool = (v40 > v41);
        let v43: bool = (!v42);
        let v44: f64 = -50.0;
        let v45: bool = (v40 < v44);
        let v46: bool = (!v45);
        let v47: bool = (v43 && v46);
        let v48: f64 = v40.exp();
        let v49: bool = (v43 && v45);
        let v50: f64 = 1.9287498479639178e-22;
        let v51: f64 = 5.184705528587072e21;
        let v52: f64 = (v40 - v41);
        let v53: f64 = (v17 + v52);
        let v54: f64 = (v51 * v53);
        let v55: f64 = (if v42 { v54 } else { v0 });
        let v56: f64 = (if v49 { v50 } else { v55 });
        let v57: f64 = (if v47 { v48 } else { v56 });
        let v58: f64 = (v32 + v57);
        let v59: f64 = (if self.scalar_v27 { v58 } else { v0 });
        let v60: f64 = nv20;
        let v61: f64 = (if self.scalar_v27 { v60 } else { v0 });
        let v66: f64 = nv22;
        let v67: f64 = nv23;
        let v68: f64 = nv25;
        let v69: f64 = nv26;
        let v72: f64 = nv7;
        let v120: f64 = (v23 - v72);
        let v139: f64 = nv6;
        let v143: f64 = (-v59);
        let v144: f64 = (if self.scalar_v27 { v143 } else { v0 });
        let v146: f64 = (v35 / self.scalar_v145);
        let v147: f64 = (if self.scalar_v27 { v146 } else { v0 });
        let v151: f64 = nv24;
        let v152: f64 = (v66 - v151);
        let v154: f64 = (v152 / self.scalar_v153);
        let v155: f64 = (if self.scalar_v65 { v154 } else { v0 });
        let v156: f64 = (v66 - v67);
        let v158: f64 = (v156 / self.scalar_v157);
        let v159: f64 = (if self.scalar_v65 { v158 } else { v0 });
        let v163: f64 = nv27;
        let v164: f64 = (v68 - v163);
        let v165: f64 = (v164 / self.scalar_v153);
        let v166: f64 = (if self.scalar_v65 { v165 } else { v0 });
        let v167: f64 = (v68 - v69);
        let v168: f64 = (v167 / self.scalar_v157);
        let v169: f64 = (if self.scalar_v65 { v168 } else { v0 });
        let v210: f64 = (v120 / self.scalar_v130);
        let v211: f64 = (if self.scalar_v126 { v210 } else { v0 });
        let v214: f64 = (v29 - v139);
        let v215: f64 = (v214 / self.scalar_v16);
        let v216: f64 = (if self.scalar_v135 { v215 } else { v0 });
        let v219: f64 = (v139 - v72);
        let v220: f64 = (v219 / self.scalar_v21);
        let v221: f64 = (if self.scalar_v138 { v220 } else { v0 });
        let v232: f64 = (v48 * self.scalar_v229);
        let v233: f64 = (v48 * self.scalar_v230);
        let v234: f64 = (v48 * self.scalar_v231);
        let v238: f64 = (if v42 { self.scalar_v235 } else { v0 });
        let v239: f64 = (if v42 { self.scalar_v236 } else { v0 });
        let v240: f64 = (if v42 { self.scalar_v237 } else { v0 });
        let v241: f64 = (if v49 { v0 } else { v238 });
        let v242: f64 = (if v49 { v0 } else { v239 });
        let v243: f64 = (if v49 { v0 } else { v240 });
        let v244: f64 = (if v47 { v232 } else { v241 });
        let v245: f64 = (if v47 { v233 } else { v242 });
        let v246: f64 = (if v47 { v234 } else { v243 });
        let v247: f64 = (if self.scalar_v27 { v244 } else { v0 });
        let v248: f64 = (if self.scalar_v27 { v245 } else { v0 });
        let v249: f64 = (if self.scalar_v27 { v246 } else { v0 });
        let v251: f64 = (-v247);
        let v252: f64 = (-v248);
        let v253: f64 = (-v249);
        let v254: f64 = (if self.scalar_v27 { v251 } else { v0 });
        let v255: f64 = (if self.scalar_v27 { v252 } else { v0 });
        let v256: f64 = (if self.scalar_v27 { v253 } else { v0 });

        let d144_dn0: f64 = v254;
        let d144_dn1: f64 = v255;
        let d144_dn21: f64 = v256;
        stamper.stamp_current_node3_local(
            Some(21),
            None,
            multiplicity * (v144),
            0,
            multiplicity * (d144_dn0),
            1,
            multiplicity * (d144_dn1),
            21,
            multiplicity * (d144_dn21),
        );
        let d147_dn21: f64 = self.scalar_v258;
        stamper.stamp_current_node1_local(
            Some(21),
            None,
            multiplicity * (v147),
            21,
            multiplicity * (d147_dn21),
        );
        let d61_dn20: f64 = self.scalar_v250;
        stamper.stamp_current_node1_local(
            Some(20),
            None,
            multiplicity * (v61),
            20,
            multiplicity * (d61_dn20),
        );
        let d155_dn22: f64 = self.scalar_v263;
        let d155_dn24: f64 = self.scalar_v264;
        stamper.stamp_current_node2_local(
            Some(22),
            Some(24),
            multiplicity * (v155),
            22,
            multiplicity * (d155_dn22),
            24,
            multiplicity * (d155_dn24),
        );
        let d159_dn22: f64 = self.scalar_v267;
        let d159_dn23: f64 = self.scalar_v268;
        stamper.stamp_current_node2_local(
            Some(22),
            Some(23),
            multiplicity * (v159),
            22,
            multiplicity * (d159_dn22),
            23,
            multiplicity * (d159_dn23),
        );
        let d166_dn25: f64 = self.scalar_v263;
        let d166_dn27: f64 = self.scalar_v264;
        stamper.stamp_current_node2_local(
            Some(25),
            Some(27),
            multiplicity * (v166),
            25,
            multiplicity * (d166_dn25),
            27,
            multiplicity * (d166_dn27),
        );
        let d169_dn25: f64 = self.scalar_v267;
        let d169_dn26: f64 = self.scalar_v268;
        stamper.stamp_current_node2_local(
            Some(25),
            Some(26),
            multiplicity * (v169),
            25,
            multiplicity * (d169_dn25),
            26,
            multiplicity * (d169_dn26),
        );
        stamper.stamp_current_const_local(
            Some(2),
            Some(17),
            multiplicity * (self.scalar_v175),
        );
        stamper.stamp_current_const_local(
            Some(7),
            Some(17),
            multiplicity * (self.scalar_v176),
        );
        stamper.stamp_current_const_local(
            Some(7),
            Some(9),
            multiplicity * (self.scalar_v176),
        );
        stamper.stamp_current_const_local(
            Some(2),
            Some(16),
            multiplicity * (self.scalar_v179),
        );
        stamper.stamp_current_const_local(
            Some(7),
            Some(16),
            multiplicity * (self.scalar_v180),
        );
        stamper.stamp_current_const_local(
            Some(7),
            Some(9),
            multiplicity * (self.scalar_v180),
        );
        stamper.stamp_current_const_local(
            Some(2),
            Some(15),
            multiplicity * (self.scalar_v183),
        );
        stamper.stamp_current_const_local(
            Some(7),
            Some(15),
            multiplicity * (self.scalar_v184),
        );
        stamper.stamp_current_const_local(
            Some(7),
            Some(9),
            multiplicity * (self.scalar_v184),
        );
        stamper.stamp_current_const_local(
            Some(2),
            Some(14),
            multiplicity * (self.scalar_v187),
        );
        stamper.stamp_current_const_local(
            Some(7),
            Some(14),
            multiplicity * (self.scalar_v188),
        );
        stamper.stamp_current_const_local(
            Some(7),
            Some(9),
            multiplicity * (self.scalar_v188),
        );
        stamper.stamp_current_const_local(
            Some(2),
            Some(9),
            multiplicity * (self.scalar_v191),
        );
        stamper.stamp_current_const_local(
            Some(7),
            Some(9),
            multiplicity * (self.scalar_v192),
        );
        stamper.stamp_current_const_local(
            Some(7),
            Some(9),
            multiplicity * (self.scalar_v192),
        );
        stamper.stamp_current_const_local(
            Some(2),
            Some(10),
            multiplicity * (self.scalar_v195),
        );
        stamper.stamp_current_const_local(
            Some(7),
            Some(10),
            multiplicity * (self.scalar_v196),
        );
        stamper.stamp_current_const_local(
            Some(7),
            Some(9),
            multiplicity * (self.scalar_v196),
        );
        stamper.stamp_current_const_local(
            Some(2),
            Some(11),
            multiplicity * (self.scalar_v199),
        );
        stamper.stamp_current_const_local(
            Some(7),
            Some(11),
            multiplicity * (self.scalar_v200),
        );
        stamper.stamp_current_const_local(
            Some(7),
            Some(9),
            multiplicity * (self.scalar_v200),
        );
        stamper.stamp_current_const_local(
            Some(2),
            Some(12),
            multiplicity * (self.scalar_v203),
        );
        stamper.stamp_current_const_local(
            Some(7),
            Some(12),
            multiplicity * (self.scalar_v204),
        );
        stamper.stamp_current_const_local(
            Some(7),
            Some(9),
            multiplicity * (self.scalar_v204),
        );
        let d211_dn7: f64 = self.scalar_v271;
        let d211_dn8: f64 = self.scalar_v272;
        stamper.stamp_current_node2_local(
            Some(8),
            Some(7),
            multiplicity * (v211),
            7,
            multiplicity * (d211_dn7),
            8,
            multiplicity * (d211_dn8),
        );
        let d216_dn1: f64 = self.scalar_v275;
        let d216_dn6: f64 = self.scalar_v276;
        stamper.stamp_current_node2_local(
            Some(1),
            Some(6),
            multiplicity * (v216),
            1,
            multiplicity * (d216_dn1),
            6,
            multiplicity * (d216_dn6),
        );
        let d221_dn6: f64 = self.scalar_v279;
        let d221_dn7: f64 = self.scalar_v280;
        stamper.stamp_current_node2_local(
            Some(6),
            Some(7),
            multiplicity * (v221),
            6,
            multiplicity * (d221_dn6),
            7,
            multiplicity * (d221_dn7),
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
        Self::stamp_transient_block_26(s, p);
        Self::stamp_transient_block_27(ctx, s, p, nodes);
        Self::stamp_transient_block_28(ctx, s, p, nodes);
        Self::stamp_transient_block_29(ctx, s, p, nodes);
        Self::stamp_transient_block_30(ctx, s, p, nodes);
        Self::stamp_transient_block_31(ctx, s, p, nodes);
        Self::stamp_transient_block_32(ctx, s, p, nodes);
        Self::stamp_transient_block_33(s, p);
        Self::stamp_transient_block_34(ctx, s, p, nodes);
        Self::stamp_transient_block_35(s, p);
        Self::stamp_transient_block_36(ctx, s, p, nodes);
        Self::stamp_transient_block_37(s, p);
        Self::stamp_transient_block_38(ctx, s, p, nodes);
        Self::stamp_transient_block_39(ctx, s, p, nodes);

        stamper.stamp_potential_branch_local(
            Some(22),
            None,
            0,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(23),
            None,
            1,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(24),
            None,
            2,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(25),
            None,
            3,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(26),
            None,
            4,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(27),
            None,
            5,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(21),
            None,
            6,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(20),
            None,
            7,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(22),
            None,
            8,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(25),
            None,
            9,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(21),
            None,
            10,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(20),
            None,
            11,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(22),
            None,
            12,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(23),
            None,
            13,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(24),
            None,
            14,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(25),
            None,
            15,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(26),
            None,
            16,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(27),
            None,
            17,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(17),
            Some(16),
            18,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(16),
            Some(15),
            19,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(15),
            Some(14),
            20,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(14),
            Some(5),
            21,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(9),
            Some(10),
            22,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(10),
            Some(11),
            23,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(11),
            Some(12),
            24,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(12),
            Some(13),
            25,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(13),
            Some(19),
            26,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(18),
            Some(17),
            27,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(28),
            None,
            28,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(29),
            None,
            29,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(7),
            Some(8),
            30,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(0),
            Some(18),
            31,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(19),
            Some(2),
            32,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(1),
            Some(6),
            33,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(6),
            Some(7),
            34,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(4),
            None,
            35,
            multiplicity,
        );

        Self::stamp_transient_equations_block_0(ctx, stamper, s, p, nodes, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_1(ctx, stamper, s, p, nodes, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_2(ctx, stamper, s, p, nodes, multiplicity);
        Self::stamp_transient_equations_block_3(stamper, s, p, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_4(ctx, stamper, s, p, nodes, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_5(ctx, stamper, s, p, nodes, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_6(ctx, stamper, s, p, nodes, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_7(ctx, stamper, s, p, nodes, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_8(ctx, stamper, s, p, nodes, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_9(ctx, stamper, s, p, nodes, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_10(ctx, stamper, s, p, nodes, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_11(ctx, stamper, s, p, nodes, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_12(ctx, stamper, s, p, nodes, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_13(ctx, stamper, s, p, nodes, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_14(ctx, stamper, s, p, nodes, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_15(ctx, stamper, s, p, nodes, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_16(ctx, stamper, s, p, nodes, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_17(ctx, stamper, s, p, nodes, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_18(ctx, stamper, s, nodes, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_19(stamper, s, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_20(ctx, stamper, s, p, nodes, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
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
        Self::stamp_reactive_block_21(ctx, s, p, nodes);

        Self::stamp_reactive_equations_block_0(ctx, stamper, s, p, nodes, multiplicity);
        Self::stamp_reactive_equations_block_1(stamper, s, p, nodes, branches, multiplicity);
        Self::stamp_reactive_equations_block_2(stamper, s, p, nodes, branches, multiplicity);
        Self::stamp_reactive_equations_block_3(ctx, stamper, s, p, nodes, branches, multiplicity);
        Self::stamp_reactive_equations_block_4(ctx, stamper, s, p, nodes, branches, multiplicity);
        Self::stamp_reactive_equations_block_5(ctx, stamper, s, p, nodes, branches, multiplicity);
        Self::stamp_reactive_equations_block_6(ctx, stamper, s, p, nodes, branches, multiplicity);
    }
}
