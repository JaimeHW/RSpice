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
        let nv1 = ctx.node_voltage(nodes[1]);
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let nv12 = ctx.node_voltage(nodes[12]);
        let nv13 = ctx.node_voltage(nodes[13]);
        let nv14 = ctx.node_voltage(nodes[14]);
        let nv16 = ctx.node_voltage(nodes[16]);
        let param_given = self.param_given.as_ref();
        let multiplicity = (*self).multiplicity;
        let timestep = (*self).timestep;
        let ddt_state_current = self.ddt_state_current.as_mut();
        let ddt_state_previous = self.ddt_state_previous.as_mut();
        let ddt_state_initialized = self.ddt_state_initialized.as_mut();
        let ddt_active = timestep.abs() > Instance::DDT_EPSILON;
        let ddt_scale = if ddt_active { 1.0 / timestep } else { 0.0 };
        let v0: f64 = 0.0;
        let v191: f64 = nv5;
        let v192: f64 = nv10;
        let v193: f64 = nv14;
        let v194: f64 = nv13;
        let v199: f64 = nv7;
        let v206: f64 = (v199 - v191);
        let v207: f64 = 1000.0;
        let v208: f64 = (v206 * v207);
        let v209: f64 = (if self.scalar_v198 { v208 } else { v0 });
        let v212: f64 = nv12;
        let v216: f64 = nv1;
        let v217: f64 = (v216 - v192);
        let v218: f64 = (v217 * self.scalar_v177);
        let v219: f64 = (if self.scalar_v161 { v218 } else { v0 });
        let v220: f64 = (v192 - v212);
        let v221: f64 = (v220 * self.scalar_v184);
        let v222: f64 = (if self.scalar_v179 { v221 } else { v0 });
        let v223: f64 = (v192 - v194);
        let v224: f64 = (v223 * self.scalar_v189);
        let v225: f64 = (if self.scalar_v179 { v224 } else { v0 });
        let v226: f64 = (v192 - v193);
        let v227: f64 = (v226 * self.scalar_v189);
        let v228: f64 = (if self.scalar_v179 { v227 } else { v0 });
        let v234: f64 = nv16;
        let v235: f64 = (if self.scalar_v203 { v234 } else { v0 });

        let d209_dn5: f64 = self.scalar_v238;
        let d209_dn7: f64 = self.scalar_v239;
        stamper.stamp_current_node2_local(
            Some(7),
            Some(5),
            multiplicity * (v209),
            5,
            multiplicity * (d209_dn5),
            7,
            multiplicity * (d209_dn7),
        );
        stamper.stamp_potential_branch_local(
            Some(7),
            Some(5),
            0,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            0,
            self.scalar_v211,
        );
        stamper.stamp_potential_branch_local(
            Some(12),
            Some(11),
            7,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            7,
            self.scalar_v214,
        );
        stamper.stamp_potential_branch_local(
            Some(15),
            None,
            8,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            8,
            self.scalar_v215,
        );
        let d219_dn1: f64 = self.scalar_v241;
        let d219_dn10: f64 = self.scalar_v242;
        stamper.stamp_current_node2_local(
            Some(1),
            Some(10),
            multiplicity * (v219),
            1,
            multiplicity * (d219_dn1),
            10,
            multiplicity * (d219_dn10),
        );
        let d222_dn10: f64 = self.scalar_v244;
        let d222_dn12: f64 = self.scalar_v245;
        stamper.stamp_current_node2_local(
            Some(10),
            Some(12),
            multiplicity * (v222),
            10,
            multiplicity * (d222_dn10),
            12,
            multiplicity * (d222_dn12),
        );
        let d225_dn10: f64 = self.scalar_v247;
        let d225_dn13: f64 = self.scalar_v248;
        stamper.stamp_current_node2_local(
            Some(10),
            Some(13),
            multiplicity * (v225),
            10,
            multiplicity * (d225_dn10),
            13,
            multiplicity * (d225_dn13),
        );
        let d228_dn10: f64 = self.scalar_v247;
        let d228_dn14: f64 = self.scalar_v248;
        stamper.stamp_current_node2_local(
            Some(10),
            Some(14),
            multiplicity * (v228),
            10,
            multiplicity * (d228_dn10),
            14,
            multiplicity * (d228_dn14),
        );
        stamper.stamp_potential_branch_local(
            Some(10),
            Some(12),
            9,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            9,
            self.scalar_v230,
        );
        stamper.stamp_potential_branch_local(
            Some(10),
            Some(13),
            10,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            10,
            self.scalar_v230,
        );
        stamper.stamp_potential_branch_local(
            Some(10),
            Some(14),
            11,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            11,
            self.scalar_v230,
        );
        stamper.stamp_potential_branch_local(
            Some(1),
            Some(10),
            12,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            12,
            self.scalar_v232,
        );
        stamper.stamp_potential_branch_local(
            Some(10),
            Some(12),
            13,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            13,
            self.scalar_v232,
        );
        stamper.stamp_potential_branch_local(
            Some(10),
            Some(13),
            14,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            14,
            self.scalar_v232,
        );
        stamper.stamp_potential_branch_local(
            Some(10),
            Some(14),
            15,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            15,
            self.scalar_v232,
        );
        stamper.stamp_potential_branch_local(
            Some(16),
            None,
            16,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            16,
            self.scalar_v233,
        );
        let d235_dn16: f64 = self.scalar_v249;
        stamper.stamp_current_node1_local(
            Some(16),
            None,
            multiplicity * (v235),
            16,
            multiplicity * (d235_dn16),
        );
        stamper.stamp_potential_branch_local(
            Some(4),
            None,
            17,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            17,
            self.scalar_v236,
        );
        let s = match &mut self.scratch {
            Some(buf) => buf.as_mut(),
            slot @ None => slot.insert(Scratch::new_box()).as_mut(),
        };

        Self::stamp_transient_block_0(s, p);
        Self::stamp_transient_block_1(s, p);
        Self::stamp_transient_block_2(s, p);
        Self::stamp_transient_block_3(s, p);
        Self::stamp_transient_block_4(s, p, param_given);
        Self::stamp_transient_block_5(s, p, param_given);
        Self::stamp_transient_block_6(s, p);
        Self::stamp_transient_block_7(s, p);
        Self::stamp_transient_block_8(s, p, param_given);
        Self::stamp_transient_block_9(ctx, s, p, nodes);
        Self::stamp_transient_block_10(s, p);
        Self::stamp_transient_block_11(s, p);
        Self::stamp_transient_block_12(s, p);
        Self::stamp_transient_block_13(s, p);
        Self::stamp_transient_block_14(s, p, param_given);
        Self::stamp_transient_block_15(s, p, param_given);
        Self::stamp_transient_block_16(ctx, s, p, nodes);
        Self::stamp_transient_block_17(s, p);
        Self::stamp_transient_block_18(s, p);
        Self::stamp_transient_block_19(s, p);
        Self::stamp_transient_block_20(s, p);
        Self::stamp_transient_block_21(ctx, s, p, nodes);
        Self::stamp_transient_block_22(ctx, s, p, nodes);
        Self::stamp_transient_block_23(ctx, s, p, nodes);
        Self::stamp_transient_block_24(s, p);
        Self::stamp_transient_block_25(s, p);
        Self::stamp_transient_block_26(s, p);
        Self::stamp_transient_block_27(s, p);
        Self::stamp_transient_block_28(ctx, s, p, nodes);
        Self::stamp_transient_block_29(ctx, s, p, nodes);

        stamper.stamp_potential_branch_local(
            Some(9),
            Some(7),
            1,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(0),
            Some(9),
            2,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(9),
            Some(7),
            3,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(8),
            Some(6),
            4,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(2),
            Some(8),
            5,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(8),
            Some(6),
            6,
            multiplicity,
        );

        Self::stamp_transient_equations_block_0(ctx, stamper, s, nodes, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_1(stamper, s, multiplicity);
        Self::stamp_transient_equations_block_2(stamper, s, multiplicity);
        Self::stamp_transient_equations_block_3(stamper, s, multiplicity);
        Self::stamp_transient_equations_block_4(stamper, s, multiplicity);
        Self::stamp_transient_equations_block_5(ctx, stamper, s, nodes, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_6(stamper, s, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_7(stamper, s, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_8(ctx, stamper, s, nodes, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_9(ctx, stamper, s, nodes, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_10(ctx, stamper, s, nodes, multiplicity);
        Self::stamp_transient_equations_block_11(ctx, stamper, s, nodes, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
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

        Self::stamp_reactive_block_0(s, p);
        Self::stamp_reactive_block_1(s, p);
        Self::stamp_reactive_block_2(s, p);
        Self::stamp_reactive_block_3(s, p);
        Self::stamp_reactive_block_4(s, p, param_given);
        Self::stamp_reactive_block_5(s, p);
        Self::stamp_reactive_block_6(s, p);
        Self::stamp_reactive_block_7(s, p);
        Self::stamp_reactive_block_8(s, p, param_given);
        Self::stamp_reactive_block_9(ctx, s, p, nodes);
        Self::stamp_reactive_block_10(s, p);
        Self::stamp_reactive_block_11(s, p);
        Self::stamp_reactive_block_12(s, p);
        Self::stamp_reactive_block_13(s, p);
        Self::stamp_reactive_block_14(s, p, param_given);
        Self::stamp_reactive_block_15(ctx, s, p, nodes);
        Self::stamp_reactive_block_16(s, p);
        Self::stamp_reactive_block_17(s, p);
        Self::stamp_reactive_block_18(s, p);
        Self::stamp_reactive_block_19(s, p);
        Self::stamp_reactive_block_20(ctx, s, p, nodes);
        Self::stamp_reactive_block_21(ctx, s, p, nodes);
        Self::stamp_reactive_block_22(ctx, s, p, nodes);
        Self::stamp_reactive_block_23(s, p);
        Self::stamp_reactive_block_24(s, p);
        Self::stamp_reactive_block_25(s, p);
        Self::stamp_reactive_block_26(ctx, s, p, nodes);
        Self::stamp_reactive_block_27(s, p);
        Self::stamp_reactive_block_28(ctx, s, p, nodes);

        Self::stamp_reactive_equations_block_0(stamper, s, nodes, branches, multiplicity);
        Self::stamp_reactive_equations_block_1(stamper, s, nodes, branches, multiplicity);
        Self::stamp_reactive_equations_block_2(ctx, stamper, s, nodes, branches, multiplicity);
        Self::stamp_reactive_equations_block_3(ctx, stamper, s, nodes, branches, multiplicity);
    }
}
