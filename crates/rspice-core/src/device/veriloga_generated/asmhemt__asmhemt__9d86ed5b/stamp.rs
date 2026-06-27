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
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let param_given = self.param_given.as_ref();
        let multiplicity = (*self).multiplicity;
        let timestep = (*self).timestep;
        let ddt_state_current = self.ddt_state_current.as_mut();
        let ddt_state_previous = self.ddt_state_previous.as_mut();
        let ddt_state_initialized = self.ddt_state_initialized.as_mut();
        let ddt_active = timestep.abs() > Instance::DDT_EPSILON;
        let ddt_scale = if ddt_active { 1.0 / timestep } else { 0.0 };
        let v0: f64 = 0.0;
        let v10: f64 = -1.0;
        let v11: f64 = nv0;
        let v12: f64 = nv2;
        let v13: f64 = (v11 - v12);
        let v27: f64 = nv5;
        let v31: f64 = nv6;
        let v90: f64 = (v27 / self.scalar_v89);
        let v91: f64 = (if self.scalar_v26 { v90 } else { v0 });
        let v94: f64 = (v27 / self.scalar_v93);
        let v95: f64 = (if self.scalar_v30 { v94 } else { v0 });
        let v97: f64 = (v31 / self.scalar_v96);
        let v98: f64 = (if self.scalar_v30 { v97 } else { v0 });
        let v99: f64 = (v10 * v13);
        let v100: f64 = (if self.scalar_v30 { v99 } else { v0 });
        let v103: f64 = (v27 / self.scalar_v102);
        let v104: f64 = (if self.scalar_v34 { v103 } else { v0 });

        let d91_dn5: f64 = self.scalar_v132;
        stamper.stamp_current_node1_local(
            Some(5),
            None,
            multiplicity * (v91),
            5,
            multiplicity * (d91_dn5),
        );
        let d95_dn5: f64 = self.scalar_v134;
        stamper.stamp_current_node1_local(
            Some(5),
            None,
            multiplicity * (v95),
            5,
            multiplicity * (d95_dn5),
        );
        let d98_dn6: f64 = self.scalar_v136;
        stamper.stamp_current_node1_local(
            Some(6),
            None,
            multiplicity * (v98),
            6,
            multiplicity * (d98_dn6),
        );
        let d100_dn0: f64 = self.scalar_v137;
        let d100_dn2: f64 = self.scalar_v138;
        stamper.stamp_current_node2_local(
            Some(6),
            None,
            multiplicity * (v100),
            0,
            multiplicity * (d100_dn0),
            2,
            multiplicity * (d100_dn2),
        );
        let d104_dn5: f64 = self.scalar_v140;
        stamper.stamp_current_node1_local(
            Some(5),
            None,
            multiplicity * (v104),
            5,
            multiplicity * (d104_dn5),
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
        Self::stamp_transient_block_9(s, p);
        Self::stamp_transient_block_10(ctx, s, p, nodes);
        Self::stamp_transient_block_11(ctx, s, p, nodes);
        Self::stamp_transient_block_12(s, p);
        Self::stamp_transient_block_13(ctx, s, p, nodes);
        Self::stamp_transient_block_14(ctx, s, p, nodes);
        Self::stamp_transient_block_15(s, p);
        Self::stamp_transient_block_16(ctx, s, p, nodes);
        Self::stamp_transient_block_17(ctx, s, p, nodes);
        Self::stamp_transient_block_18(s, p);
        Self::stamp_transient_block_19(ctx, s, p, nodes);
        Self::stamp_transient_block_20(ctx, s, p, nodes);
        Self::stamp_transient_block_21(s, p);
        Self::stamp_transient_block_22(ctx, s, p, nodes);
        Self::stamp_transient_block_23(ctx, s, p, nodes);
        Self::stamp_transient_block_24(s, p);
        Self::stamp_transient_block_25(ctx, s, p, nodes);
        Self::stamp_transient_block_26(ctx, s, p, nodes);
        Self::stamp_transient_block_27(s, p);
        Self::stamp_transient_block_28(ctx, s, p, nodes);
        Self::stamp_transient_block_29(s, p);

        stamper.stamp_potential_branch_local(
            Some(4),
            None,
            0,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(5),
            None,
            1,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(6),
            None,
            2,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(12),
            None,
            3,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(14),
            None,
            4,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(11),
            None,
            5,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(13),
            None,
            6,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(6),
            None,
            7,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(12),
            None,
            8,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(14),
            None,
            9,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(11),
            None,
            10,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(13),
            None,
            11,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(12),
            None,
            12,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(14),
            None,
            13,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(11),
            None,
            14,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(13),
            None,
            15,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(6),
            None,
            16,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(12),
            None,
            17,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(14),
            None,
            18,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(11),
            None,
            19,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(13),
            None,
            20,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(5),
            None,
            21,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(6),
            None,
            22,
            multiplicity,
        );
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
        stamper.stamp_potential_branch_local(
            Some(12),
            None,
            25,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(14),
            None,
            26,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(11),
            None,
            27,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(13),
            None,
            28,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(0),
            Some(18),
            29,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(2),
            Some(22),
            30,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(0),
            Some(7),
            31,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(8),
            Some(2),
            32,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(15),
            Some(7),
            33,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(15),
            Some(7),
            34,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(8),
            Some(19),
            35,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(8),
            Some(19),
            36,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(8),
            Some(19),
            37,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(16),
            Some(15),
            38,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(16),
            Some(7),
            39,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(19),
            Some(20),
            40,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(8),
            Some(20),
            41,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(8),
            Some(20),
            42,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(17),
            Some(16),
            43,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(17),
            Some(7),
            44,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(20),
            Some(21),
            45,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(8),
            Some(21),
            46,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(8),
            Some(21),
            47,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(18),
            Some(17),
            48,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(18),
            Some(7),
            49,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(21),
            Some(22),
            50,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(8),
            Some(22),
            51,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(8),
            Some(22),
            52,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(10),
            Some(9),
            53,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(1),
            Some(10),
            54,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(10),
            Some(9),
            55,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(4),
            None,
            56,
            multiplicity,
        );

        Self::stamp_transient_equations_block_0(ctx, stamper, s, p, nodes, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_1(ctx, stamper, s, p, nodes, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_2(ctx, stamper, s, p, nodes, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_3(ctx, stamper, s, p, nodes, multiplicity);
        Self::stamp_transient_equations_block_4(ctx, stamper, s, p, nodes, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_5(ctx, stamper, s, p, nodes, multiplicity);
        Self::stamp_transient_equations_block_6(ctx, stamper, s, p, nodes, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_7(ctx, stamper, s, p, nodes, multiplicity);
        Self::stamp_transient_equations_block_8(stamper, s, p, multiplicity);
        Self::stamp_transient_equations_block_9(ctx, stamper, s, p, nodes, multiplicity);
        Self::stamp_transient_equations_block_10(ctx, stamper, s, p, nodes, multiplicity);
        Self::stamp_transient_equations_block_11(ctx, stamper, s, p, nodes, multiplicity);
        Self::stamp_transient_equations_block_12(ctx, stamper, s, p, nodes, multiplicity);
        Self::stamp_transient_equations_block_13(ctx, stamper, s, p, nodes, multiplicity);
        Self::stamp_transient_equations_block_14(ctx, stamper, s, p, nodes, multiplicity);
        Self::stamp_transient_equations_block_15(ctx, stamper, s, p, nodes, multiplicity);
        Self::stamp_transient_equations_block_16(ctx, stamper, s, p, nodes, multiplicity);
        Self::stamp_transient_equations_block_17(ctx, stamper, s, p, nodes, multiplicity);
        Self::stamp_transient_equations_block_18(ctx, stamper, s, p, nodes, multiplicity);
        Self::stamp_transient_equations_block_19(ctx, stamper, s, p, nodes, multiplicity);
        Self::stamp_transient_equations_block_20(ctx, stamper, s, p, nodes, multiplicity);
        Self::stamp_transient_equations_block_21(ctx, stamper, s, p, nodes, multiplicity);
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
        Self::stamp_transient_equations_block_71(stamper, s, p, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_72(stamper, s, p, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_73(stamper, s, p, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_74(stamper, s, p, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_75(ctx, stamper, s, p, nodes, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_76(ctx, stamper, s, p, nodes, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
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
        Self::stamp_reactive_block_9(ctx, s, p, nodes);
        Self::stamp_reactive_block_10(ctx, s, p, nodes);
        Self::stamp_reactive_block_11(ctx, s, p, nodes);
        Self::stamp_reactive_block_12(ctx, s, p, nodes);
        Self::stamp_reactive_block_13(ctx, s, p, nodes);
        Self::stamp_reactive_block_14(ctx, s, p, nodes);
        Self::stamp_reactive_block_15(ctx, s, p, nodes);
        Self::stamp_reactive_block_16(ctx, s, p, nodes);
        Self::stamp_reactive_block_17(ctx, s, p, nodes);
        Self::stamp_reactive_block_18(ctx, s, p, nodes);
        Self::stamp_reactive_block_19(ctx, s, p, nodes);
        Self::stamp_reactive_block_20(ctx, s, p, nodes);
        Self::stamp_reactive_block_21(ctx, s, p, nodes);
        Self::stamp_reactive_block_22(ctx, s, p, nodes);
        Self::stamp_reactive_block_23(ctx, s, p, nodes);
        Self::stamp_reactive_block_24(ctx, s, p, nodes);
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
