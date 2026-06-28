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
#[path = "stamp_blocks_12.rs"]
mod stamp_blocks_12;
#[path = "stamp_blocks_13.rs"]
mod stamp_blocks_13;

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
        let scalar_temperature_static_temperature = (ctx).temperature();
        let scalar_temperature_static_thermal_voltage = (ctx).thermal_voltage();
        self.ensure_temperature_static(scalar_temperature_static_temperature, scalar_temperature_static_thermal_voltage);
        let p = Box::as_ref(&self.params);
        let nodes = &(*self).nodes;
        let branches = &(*self).branches;
        let nv1 = ctx.node_voltage(nodes[1]);
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let nv11 = ctx.node_voltage(nodes[11]);
        let nv15 = ctx.node_voltage(nodes[15]);
        let param_given = self.param_given.as_ref();
        let multiplicity = (*self).multiplicity;
        let timestep = (*self).timestep;
        let ddt_state_current = self.ddt_state_current.as_mut();
        let ddt_state_previous = self.ddt_state_previous.as_mut();
        let ddt_state_initialized = self.ddt_state_initialized.as_mut();
        let ddt_active = timestep.abs() > Instance::DDT_EPSILON;
        let ddt_scale = if ddt_active { 1.0 / timestep } else { 0.0 };
        let v0: f64 = 1.0;
        let v2: f64 = 0.0;
        let v37: f64 = 10000.0;
        let v331: f64 = nv7;
        let v332: f64 = nv9;
        let v333: f64 = nv11;
        let v334: f64 = nv10;
        let v339: f64 = nv5;
        let v388: f64 = nv15;
        let v389: f64 = nv1;
        let v390: f64 = (v389 - v331);
        let v391: f64 = (self.scalar_v253 * v390);
        let v392: f64 = (if self.scalar_v377 { v391 } else { v2 });
        let v395: f64 = (v333 - v332);
        let v396: f64 = (self.scalar_v286 * v395);
        let v397: f64 = (if (self.scalar_v254 != 0.0) { v396 } else { v2 });
        let v398: f64 = (v334 - v332);
        let v399: f64 = (self.scalar_v287 * v398);
        let v400: f64 = (if (self.scalar_v254 != 0.0) { v399 } else { v2 });
        let v401: f64 = nv3;
        let v402: f64 = (v401 - v332);
        let v403: f64 = (self.scalar_v285 * v402);
        let v404: f64 = (if (self.scalar_v254 != 0.0) { v403 } else { v2 });
        let v407: f64 = (v339 * self.scalar_v375);
        let v408: f64 = (if self.scalar_v336 { v407 } else { v2 });
        let v409: f64 = (v37 * v339);
        let v410: f64 = (if self.scalar_v374 { v409 } else { v2 });
        let v411: f64 = (v339 * self.scalar_v378);

        stamper.stamp_potential_branch_local(
            Some(4),
            Some(5),
            0,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            0,
            self.scalar_v379,
        );
        stamper.stamp_potential_branch_local(
            Some(16),
            None,
            1,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            1,
            self.scalar_v381,
        );
        stamper.stamp_potential_branch_local(
            Some(17),
            None,
            2,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            2,
            self.scalar_v381,
        );
        stamper.stamp_potential_branch_local(
            Some(18),
            None,
            3,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            3,
            self.scalar_v383,
        );
        stamper.stamp_potential_branch_local(
            Some(0),
            Some(6),
            4,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            4,
            self.scalar_v385,
        );
        stamper.stamp_potential_branch_local(
            Some(8),
            Some(2),
            5,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            5,
            self.scalar_v387,
        );
        let d388_dn15: f64 = v0;
        stamper.stamp_current_node1_local(
            Some(15),
            None,
            multiplicity * (v388),
            15,
            multiplicity * (d388_dn15),
        );
        let d392_dn1: f64 = self.scalar_v415;
        let d392_dn7: f64 = self.scalar_v416;
        stamper.stamp_current_node2_local(
            Some(1),
            Some(7),
            multiplicity * (v392),
            1,
            multiplicity * (d392_dn1),
            7,
            multiplicity * (d392_dn7),
        );
        stamper.stamp_potential_branch_local(
            Some(1),
            Some(7),
            6,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            6,
            self.scalar_v394,
        );
        let d397_dn9: f64 = self.scalar_v418;
        let d397_dn11: f64 = self.scalar_v419;
        stamper.stamp_current_node2_local(
            Some(11),
            Some(9),
            multiplicity * (v397),
            9,
            multiplicity * (d397_dn9),
            11,
            multiplicity * (d397_dn11),
        );
        let d400_dn9: f64 = self.scalar_v421;
        let d400_dn10: f64 = self.scalar_v422;
        stamper.stamp_current_node2_local(
            Some(10),
            Some(9),
            multiplicity * (v400),
            9,
            multiplicity * (d400_dn9),
            10,
            multiplicity * (d400_dn10),
        );
        let d404_dn3: f64 = self.scalar_v424;
        let d404_dn9: f64 = self.scalar_v425;
        stamper.stamp_current_node2_local(
            Some(3),
            Some(9),
            multiplicity * (v404),
            3,
            multiplicity * (d404_dn3),
            9,
            multiplicity * (d404_dn9),
        );
        stamper.stamp_potential_branch_local(
            Some(3),
            Some(9),
            7,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            7,
            self.scalar_v406,
        );
        stamper.stamp_potential_branch_local(
            Some(11),
            Some(9),
            8,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            8,
            self.scalar_v406,
        );
        stamper.stamp_potential_branch_local(
            Some(10),
            Some(9),
            9,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            9,
            self.scalar_v406,
        );
        let d408_dn5: f64 = self.scalar_v426;
        stamper.stamp_current_node1_local(
            Some(5),
            None,
            multiplicity * (v408),
            5,
            multiplicity * (d408_dn5),
        );
        let d410_dn5: f64 = self.scalar_v427;
        stamper.stamp_current_node1_local(
            Some(5),
            None,
            multiplicity * (v410),
            5,
            multiplicity * (d410_dn5),
        );
        stamper.stamp_potential_branch_local(
            Some(12),
            None,
            10,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            10,
            self.scalar_v412,
        );
        stamper.stamp_potential_branch_local(
            Some(13),
            None,
            11,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            11,
            self.scalar_v412,
        );
        stamper.stamp_potential_branch_local(
            Some(14),
            None,
            12,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            12,
            self.scalar_v413,
        );
        let d411_dn5: f64 = self.scalar_v378;
        let v411_ddt: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 17, v411);
        stamper.stamp_current_node1_local(
            Some(5),
            None,
            multiplicity * (v411_ddt),
            5,
            multiplicity * (((d411_dn5) * ddt_scale)),
        );
        let s = match &mut self.scratch {
            Some(buf) => buf.as_mut(),
            slot @ None => slot.insert(Scratch::new_box()).as_mut(),
        };

        Self::stamp_transient_block_0(s, param_given);
        Self::stamp_transient_block_1(s, p);
        Self::stamp_transient_block_2(s, p);
        Self::stamp_transient_block_3(s, p);
        Self::stamp_transient_block_4(s, p);
        Self::stamp_transient_block_5(ctx, s, p);
        Self::stamp_transient_block_6(s, p);
        Self::stamp_transient_block_7(ctx, s, p, nodes);
        Self::stamp_transient_block_8(ctx, s, p, nodes);
        Self::stamp_transient_block_9(s, p);
        Self::stamp_transient_block_10(ctx, s, p, param_given);
        Self::stamp_transient_block_11(s, p);
        Self::stamp_transient_block_12(s, p);
        Self::stamp_transient_block_13(s, p);
        Self::stamp_transient_block_14(s);
        Self::stamp_transient_block_15(s);
        Self::stamp_transient_block_16(s);
        Self::stamp_transient_block_17(s);
        Self::stamp_transient_block_18(s);
        Self::stamp_transient_block_19(s);
        Self::stamp_transient_block_20(s);
        Self::stamp_transient_block_21(s);
        Self::stamp_transient_block_22(s);
        Self::stamp_transient_block_23(s);
        Self::stamp_transient_block_24(s);
        Self::stamp_transient_block_25(s);
        Self::stamp_transient_block_26(s);
        Self::stamp_transient_block_27(s);
        Self::stamp_transient_block_28(s);
        Self::stamp_transient_block_29(s, p);
        Self::stamp_transient_block_30(s, p);
        Self::stamp_transient_block_31(s, p);
        Self::stamp_transient_block_32(s, p);
        Self::stamp_transient_block_33(s, p);
        Self::stamp_transient_block_34(s);
        Self::stamp_transient_block_35(s);
        Self::stamp_transient_block_36(s, p);
        Self::stamp_transient_block_37(s, p);
        Self::stamp_transient_block_38(s, p);
        Self::stamp_transient_block_39(s, p);
        Self::stamp_transient_block_40(s, p);
        Self::stamp_transient_block_41(s, p);
        Self::stamp_transient_block_42(s, p, param_given);
        Self::stamp_transient_block_43(s, p);
        Self::stamp_transient_block_44(s, p);
        Self::stamp_transient_block_45(s, p);
        Self::stamp_transient_block_46(s, p);
        Self::stamp_transient_block_47(s, p);
        Self::stamp_transient_block_48(s, p);
        Self::stamp_transient_block_49(s, p);
        Self::stamp_transient_block_50(s, p);
        Self::stamp_transient_block_51(s, p);
        Self::stamp_transient_block_52(s, p);
        Self::stamp_transient_block_53(s);
        Self::stamp_transient_block_54(s, p);
        Self::stamp_transient_block_55(s, p);
        Self::stamp_transient_block_56(s, p);
        Self::stamp_transient_block_57(s, p, param_given);
        Self::stamp_transient_block_58(s, p);
        Self::stamp_transient_block_59(s);
        Self::stamp_transient_block_60(s);
        Self::stamp_transient_block_61(s);
        Self::stamp_transient_block_62(s, p);
        Self::stamp_transient_block_63(s, p);
        Self::stamp_transient_block_64(s, p);
        Self::stamp_transient_block_65(s, p);
        Self::stamp_transient_block_66(s, p);
        Self::stamp_transient_block_67(s, p);
        Self::stamp_transient_block_68(s, p);
        Self::stamp_transient_block_69(s, p);
        Self::stamp_transient_block_70(s, p, param_given);
        Self::stamp_transient_block_71(s, p);
        Self::stamp_transient_block_72(s, p);
        Self::stamp_transient_block_73(s, p);
        Self::stamp_transient_block_74(s, p);
        Self::stamp_transient_block_75(s, p, param_given);
        Self::stamp_transient_block_76(s, p);
        Self::stamp_transient_block_77(s, p);
        Self::stamp_transient_block_78(s, p);
        Self::stamp_transient_block_79(s, p);
        Self::stamp_transient_block_80(s, p);
        Self::stamp_transient_block_81(s, p, param_given);
        Self::stamp_transient_block_82(s, p);
        Self::stamp_transient_block_83(s, p);
        Self::stamp_transient_block_84(s, p);
        Self::stamp_transient_block_85(s, p);
        Self::stamp_transient_block_86(s, p);
        Self::stamp_transient_block_87(s, p, param_given);
        Self::stamp_transient_block_88(s, p);
        Self::stamp_transient_block_89(s, p);
        Self::stamp_transient_block_90(s, p);
        Self::stamp_transient_block_91(s, p);
        Self::stamp_transient_block_92(s, p);
        Self::stamp_transient_block_93(s, p, param_given);
        Self::stamp_transient_block_94(s, p);
        Self::stamp_transient_block_95(s, p);
        Self::stamp_transient_block_96(s, p);
        Self::stamp_transient_block_97(s, p);
        Self::stamp_transient_block_98(s, p);
        Self::stamp_transient_block_99(ctx, s, p);
        Self::stamp_transient_block_100(s, p);
        Self::stamp_transient_block_101(ctx, s, p, nodes);
        Self::stamp_transient_block_102(ctx, s, p, nodes);
        Self::stamp_transient_block_103(ctx, s, p);
        Self::stamp_transient_block_104(ctx, s, p);
        Self::stamp_transient_block_105(s, p);
        Self::stamp_transient_block_106(s, p);

        Self::stamp_transient_equations_block_0(stamper, s, p, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_1(stamper, s, p, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_2(stamper, s, p, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_3(ctx, stamper, s, p, nodes, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_4(ctx, stamper, s, p, nodes, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_5(ctx, stamper, s, p, nodes, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
    }

    pub fn stamp_reactive(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedReactiveStamper<'_>) {
        let scalar_temperature_static_temperature = (ctx).temperature();
        let scalar_temperature_static_thermal_voltage = (ctx).thermal_voltage();
        self.ensure_temperature_static(scalar_temperature_static_temperature, scalar_temperature_static_thermal_voltage);
        let p = Box::as_ref(&self.params);
        let nodes = &(*self).nodes;
        let branches = &(*self).branches;
        let nv5 = ctx.node_voltage(nodes[5]);
        let param_given = self.param_given.as_ref();
        let multiplicity = (*self).multiplicity;
        let v339: f64 = nv5;
        let v411: f64 = (v339 * self.scalar_v378);

        let d411_dn5: f64 = self.scalar_v378;
        stamper.stamp_current_reactive_node1(
            Some(nodes[5]),
            None,
            nodes[5],
            multiplicity * (d411_dn5),
        );
        let s = match &mut self.reactive_scratch {
            Some(buf) => buf.as_mut(),
            slot @ None => slot.insert(ReactiveScratch::new_box()).as_mut(),
        };

        Self::stamp_reactive_block_0(s, param_given);
        Self::stamp_reactive_block_1(s, p);
        Self::stamp_reactive_block_2(s, p);
        Self::stamp_reactive_block_3(s, p);
        Self::stamp_reactive_block_4(ctx, s, p);
        Self::stamp_reactive_block_5(s, p);
        Self::stamp_reactive_block_6(s, p);
        Self::stamp_reactive_block_7(ctx, s, p, nodes);
        Self::stamp_reactive_block_8(s, p);
        Self::stamp_reactive_block_9(ctx, s, p);
        Self::stamp_reactive_block_10(s, p, param_given);
        Self::stamp_reactive_block_11(s, p);
        Self::stamp_reactive_block_12(s, p);
        Self::stamp_reactive_block_13(s, p);
        Self::stamp_reactive_block_14(s);
        Self::stamp_reactive_block_15(s);
        Self::stamp_reactive_block_16(s);
        Self::stamp_reactive_block_17(s);
        Self::stamp_reactive_block_18(s);
        Self::stamp_reactive_block_19(s);
        Self::stamp_reactive_block_20(s);
        Self::stamp_reactive_block_21(s);
        Self::stamp_reactive_block_22(s);
        Self::stamp_reactive_block_23(s);
        Self::stamp_reactive_block_24(s);
        Self::stamp_reactive_block_25(s);
        Self::stamp_reactive_block_26(s);
        Self::stamp_reactive_block_27(s, p);
        Self::stamp_reactive_block_28(s, p);
        Self::stamp_reactive_block_29(s, p);
        Self::stamp_reactive_block_30(s);
        Self::stamp_reactive_block_31(s, p);
        Self::stamp_reactive_block_32(s);
        Self::stamp_reactive_block_33(s);
        Self::stamp_reactive_block_34(s, p);
        Self::stamp_reactive_block_35(s, p);
        Self::stamp_reactive_block_36(s, p);
        Self::stamp_reactive_block_37(s, p);
        Self::stamp_reactive_block_38(s, p);
        Self::stamp_reactive_block_39(s, p, param_given);
        Self::stamp_reactive_block_40(s, p);
        Self::stamp_reactive_block_41(s, p);
        Self::stamp_reactive_block_42(s, p);
        Self::stamp_reactive_block_43(s, p);
        Self::stamp_reactive_block_44(s, p);
        Self::stamp_reactive_block_45(s, p);
        Self::stamp_reactive_block_46(s, p);
        Self::stamp_reactive_block_47(s, p);
        Self::stamp_reactive_block_48(s, p);
        Self::stamp_reactive_block_49(s, p);
        Self::stamp_reactive_block_50(s, p);
        Self::stamp_reactive_block_51(s, p);
        Self::stamp_reactive_block_52(s, p);
        Self::stamp_reactive_block_53(s, p, param_given);
        Self::stamp_reactive_block_54(s);
        Self::stamp_reactive_block_55(s);
        Self::stamp_reactive_block_56(s);
        Self::stamp_reactive_block_57(s, p);
        Self::stamp_reactive_block_58(s, p);
        Self::stamp_reactive_block_59(s, p);
        Self::stamp_reactive_block_60(s, p);
        Self::stamp_reactive_block_61(s, p);
        Self::stamp_reactive_block_62(s, p);
        Self::stamp_reactive_block_63(s, p);
        Self::stamp_reactive_block_64(s, p, param_given);
        Self::stamp_reactive_block_65(s, p);
        Self::stamp_reactive_block_66(s, p);
        Self::stamp_reactive_block_67(s, p);
        Self::stamp_reactive_block_68(s, p);
        Self::stamp_reactive_block_69(s, p, param_given);
        Self::stamp_reactive_block_70(s, p);
        Self::stamp_reactive_block_71(s, p);
        Self::stamp_reactive_block_72(s, p);
        Self::stamp_reactive_block_73(s, p);
        Self::stamp_reactive_block_74(s, p, param_given);
        Self::stamp_reactive_block_75(s, p);
        Self::stamp_reactive_block_76(s, p);
        Self::stamp_reactive_block_77(s, p);
        Self::stamp_reactive_block_78(s, p);
        Self::stamp_reactive_block_79(s, p, param_given);
        Self::stamp_reactive_block_80(s, p);
        Self::stamp_reactive_block_81(s, p);
        Self::stamp_reactive_block_82(s, p);
        Self::stamp_reactive_block_83(s, p);
        Self::stamp_reactive_block_84(s, p, param_given);
        Self::stamp_reactive_block_85(s, p);
        Self::stamp_reactive_block_86(s, p);
        Self::stamp_reactive_block_87(s, p);
        Self::stamp_reactive_block_88(s, p);
        Self::stamp_reactive_block_89(s, p);
        Self::stamp_reactive_block_90(ctx, s, p);
        Self::stamp_reactive_block_91(ctx, s, p, nodes);
        Self::stamp_reactive_block_92(ctx, s, p, nodes);
        Self::stamp_reactive_block_93(ctx, s, p, nodes);
        Self::stamp_reactive_block_94(ctx, s, p);
        Self::stamp_reactive_block_95(s, p);
        Self::stamp_reactive_block_96(s, p);

        Self::stamp_reactive_equations_block_0(stamper, s, p, nodes, branches, multiplicity);
        Self::stamp_reactive_equations_block_1(ctx, stamper, s, p, nodes, branches, multiplicity);
        Self::stamp_reactive_equations_block_2(ctx, stamper, s, p, nodes, branches, multiplicity);
    }
}
