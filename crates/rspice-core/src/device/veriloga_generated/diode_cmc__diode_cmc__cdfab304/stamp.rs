#![allow(dead_code, non_snake_case, unused_assignments, unused_parens, unused_variables)]
use super::state::Instance;
use crate::device::veriloga_generated::{GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper};
use crate::device::veriloga_generated::kernel_runtime::{AdValue as KernelAdValue, ReactiveScratch as KernelReactiveScratch, Scratch as KernelScratch};
type A = KernelAdValue<{ Instance::NODE_COUNT }, { Instance::BRANCH_COUNT }>;
type Scratch = KernelScratch<{ Instance::VARIABLE_COUNT }, { Instance::NODE_COUNT }, { Instance::BRANCH_COUNT }>;
type ReactiveScratch = KernelReactiveScratch<{ Instance::VARIABLE_COUNT }, { Instance::NODE_COUNT }, { Instance::BRANCH_COUNT }>;
const TRANSIENT_NODE_DERIVATIVE_ACTIVITY: [u128; Instance::VARIABLE_COUNT] = {
    let mut masks = [0; Instance::VARIABLE_COUNT];
    masks[180]=0x00000000000000000000000000000005;masks[181]=0x00000000000000000000000000000005;masks[182]=0x00000000000000000000000000000005;
    masks[183]=0x00000000000000000000000000000005;masks[184]=0x00000000000000000000000000000005;masks[185]=0x00000000000000000000000000000005;
    masks[186]=0x00000000000000000000000000000005;masks[187]=0x00000000000000000000000000000005;masks[188]=0x00000000000000000000000000000005;
    masks[189]=0x00000000000000000000000000000005;masks[195]=0x00000000000000000000000000000005;masks[196]=0x00000000000000000000000000000005;
    masks[197]=0x00000000000000000000000000000005;masks[199]=0x00000000000000000000000000000005;masks[200]=0x00000000000000000000000000000005;
    masks[201]=0x00000000000000000000000000000005;masks[202]=0x00000000000000000000000000000005;masks[203]=0x00000000000000000000000000000005;
    masks[204]=0x00000000000000000000000000000005;masks[209]=0x00000000000000000000000000000005;masks[210]=0x00000000000000000000000000000005;
    masks[211]=0x00000000000000000000000000000005;masks[212]=0x00000000000000000000000000000005;masks[213]=0x00000000000000000000000000000005;
    masks[214]=0x00000000000000000000000000000005;masks[217]=0x00000000000000000000000000000005;masks[218]=0x00000000000000000000000000000005;
    masks[219]=0x00000000000000000000000000000005;masks[220]=0x00000000000000000000000000000005;masks[221]=0x00000000000000000000000000000005;
    masks[222]=0x00000000000000000000000000000005;masks[223]=0x00000000000000000000000000000005;masks[224]=0x00000000000000000000000000000005;
    masks[225]=0x00000000000000000000000000000005;masks[226]=0x00000000000000000000000000000005;masks[227]=0x00000000000000000000000000000005;
    masks[228]=0x00000000000000000000000000000005;masks[229]=0x00000000000000000000000000000005;masks[230]=0x00000000000000000000000000000005;
    masks[231]=0x00000000000000000000000000000005;masks[232]=0x00000000000000000000000000000005;masks[233]=0x00000000000000000000000000000005;
    masks[234]=0x00000000000000000000000000000005;masks[235]=0x00000000000000000000000000000005;masks[236]=0x00000000000000000000000000000005;
    masks[237]=0x00000000000000000000000000000005;masks[238]=0x00000000000000000000000000000005;masks[239]=0x00000000000000000000000000000005;
    masks[240]=0x00000000000000000000000000000005;masks[241]=0x00000000000000000000000000000005;masks[242]=0x00000000000000000000000000000005;
    masks[243]=0x00000000000000000000000000000005;masks[244]=0x00000000000000000000000000000005;masks[245]=0x00000000000000000000000000000005;
    masks[246]=0x00000000000000000000000000000005;masks[247]=0x00000000000000000000000000000005;masks[248]=0x00000000000000000000000000000005;
    masks[249]=0x00000000000000000000000000000005;masks[250]=0x00000000000000000000000000000005;masks[251]=0x00000000000000000000000000000005;
    masks[252]=0x00000000000000000000000000000005;masks[253]=0x00000000000000000000000000000005;masks[254]=0x00000000000000000000000000000005;
    masks[255]=0x00000000000000000000000000000005;masks[268]=0x00000000000000000000000000000005;masks[269]=0x00000000000000000000000000000005;
    masks[270]=0x00000000000000000000000000000005;masks[271]=0x00000000000000000000000000000005;masks[272]=0x00000000000000000000000000000005;
    masks[273]=0x00000000000000000000000000000005;masks[274]=0x00000000000000000000000000000005;masks[275]=0x0000000000000000000000000000003d;
    masks[277]=0x00000000000000000000000000000005;masks[281]=0x00000000000000000000000000000005;masks[282]=0x00000000000000000000000000000005;
    masks[283]=0x00000000000000000000000000000005;masks[284]=0x00000000000000000000000000000006;masks[290]=0x00000000000000000000000000000005;
    masks[291]=0x00000000000000000000000000000005;masks[292]=0x00000000000000000000000000000005;masks[293]=0x00000000000000000000000000000005;
    masks[296]=0x00000000000000000000000000000005;masks[297]=0x00000000000000000000000000000005;masks[298]=0x00000000000000000000000000000005;
    masks[299]=0x00000000000000000000000000000005;masks[302]=0x00000000000000000000000000000005;masks[303]=0x00000000000000000000000000000005;
    masks[304]=0x00000000000000000000000000000005;masks[305]=0x00000000000000000000000000000005;masks[314]=0x00000000000000000000000000000005;
    masks[315]=0x00000000000000000000000000000005;masks[325]=0x00000000000000000000000000000005;masks[326]=0x00000000000000000000000000000005;
    masks[328]=0x0000000000000000000000000000002d;masks[329]=0x00000000000000000000000000000035;masks[330]=0x0000000000000000000000000000003d;
    masks[331]=0x00000000000000000000000000000005;masks[332]=0x00000000000000000000000000000005;masks[334]=0x00000000000000000000000000000005;
    masks[335]=0x00000000000000000000000000000005;masks[336]=0x00000000000000000000000000000008;masks[337]=0x00000000000000000000000000000010;
    masks[338]=0x0000000000000000000000000000000d;masks[339]=0x00000000000000000000000000000015;masks[340]=0x0000000000000000000000000000000d;
    masks[341]=0x00000000000000000000000000000015;masks[342]=0x00000000000000000000000000000005;masks[343]=0x00000000000000000000000000000005;
    masks[344]=0x00000000000000000000000000000020;masks[345]=0x00000000000000000000000000000025;masks[346]=0x00000000000000000000000000000025;
    masks[349]=0x00000000000000000000000000000005;masks[350]=0x00000000000000000000000000000005;masks[351]=0x00000000000000000000000000000005;
    masks[352]=0x00000000000000000000000000000005;masks[353]=0x00000000000000000000000000000005;masks[354]=0x00000000000000000000000000000005;
    masks[355]=0x00000000000000000000000000000005;masks[356]=0x00000000000000000000000000000005;masks[357]=0x00000000000000000000000000000005;
    masks[358]=0x00000000000000000000000000000005;masks[359]=0x00000000000000000000000000000005;masks[360]=0x00000000000000000000000000000005;
    masks[361]=0x00000000000000000000000000000005;masks[364]=0x00000000000000000000000000000005;masks[365]=0x00000000000000000000000000000005;
    masks[366]=0x00000000000000000000000000000005;masks[367]=0x00000000000000000000000000000005;masks[370]=0x00000000000000000000000000000005;
    masks[371]=0x00000000000000000000000000000005;masks[372]=0x00000000000000000000000000000005;masks[437]=0x00000000000000000000000000000005;
    masks
};
const TRANSIENT_BRANCH_DERIVATIVE_ACTIVITY: [u128; Instance::VARIABLE_COUNT] = [0; Instance::VARIABLE_COUNT];
const REACTIVE_NODE_DERIVATIVE_ACTIVITY: [u128; Instance::VARIABLE_COUNT] = {
    let mut masks = [0; Instance::VARIABLE_COUNT];
    masks[180]=0x00000000000000000000000000000005;masks[181]=0x00000000000000000000000000000005;masks[182]=0x00000000000000000000000000000005;
    masks[183]=0x00000000000000000000000000000005;masks[184]=0x00000000000000000000000000000005;masks[185]=0x00000000000000000000000000000005;
    masks[186]=0x00000000000000000000000000000005;masks[187]=0x00000000000000000000000000000005;masks[188]=0x00000000000000000000000000000005;
    masks[189]=0x00000000000000000000000000000005;masks[195]=0x00000000000000000000000000000005;masks[196]=0x00000000000000000000000000000005;
    masks[197]=0x00000000000000000000000000000005;masks[199]=0x00000000000000000000000000000005;masks[200]=0x00000000000000000000000000000005;
    masks[201]=0x00000000000000000000000000000005;masks[202]=0x00000000000000000000000000000005;masks[203]=0x00000000000000000000000000000005;
    masks[204]=0x00000000000000000000000000000005;masks[209]=0x00000000000000000000000000000005;masks[210]=0x00000000000000000000000000000005;
    masks[211]=0x00000000000000000000000000000005;masks[212]=0x00000000000000000000000000000005;masks[213]=0x00000000000000000000000000000005;
    masks[214]=0x00000000000000000000000000000005;masks[217]=0x00000000000000000000000000000005;masks[218]=0x00000000000000000000000000000005;
    masks[219]=0x00000000000000000000000000000005;masks[220]=0x00000000000000000000000000000005;masks[221]=0x00000000000000000000000000000005;
    masks[222]=0x00000000000000000000000000000005;masks[223]=0x00000000000000000000000000000005;masks[224]=0x00000000000000000000000000000005;
    masks[225]=0x00000000000000000000000000000005;masks[226]=0x00000000000000000000000000000005;masks[227]=0x00000000000000000000000000000005;
    masks[228]=0x00000000000000000000000000000005;masks[229]=0x00000000000000000000000000000005;masks[230]=0x00000000000000000000000000000005;
    masks[231]=0x00000000000000000000000000000005;masks[232]=0x00000000000000000000000000000005;masks[233]=0x00000000000000000000000000000005;
    masks[234]=0x00000000000000000000000000000005;masks[235]=0x00000000000000000000000000000005;masks[236]=0x00000000000000000000000000000005;
    masks[237]=0x00000000000000000000000000000005;masks[238]=0x00000000000000000000000000000005;masks[239]=0x00000000000000000000000000000005;
    masks[240]=0x00000000000000000000000000000005;masks[241]=0x00000000000000000000000000000005;masks[242]=0x00000000000000000000000000000005;
    masks[243]=0x00000000000000000000000000000005;masks[244]=0x00000000000000000000000000000005;masks[245]=0x00000000000000000000000000000005;
    masks[246]=0x00000000000000000000000000000005;masks[247]=0x00000000000000000000000000000005;masks[248]=0x00000000000000000000000000000005;
    masks[249]=0x00000000000000000000000000000005;masks[250]=0x00000000000000000000000000000005;masks[251]=0x00000000000000000000000000000005;
    masks[252]=0x00000000000000000000000000000005;masks[253]=0x00000000000000000000000000000005;masks[254]=0x00000000000000000000000000000005;
    masks[255]=0x00000000000000000000000000000005;masks[268]=0x00000000000000000000000000000005;masks[269]=0x00000000000000000000000000000005;
    masks[270]=0x00000000000000000000000000000005;masks[271]=0x00000000000000000000000000000005;masks[272]=0x00000000000000000000000000000005;
    masks[273]=0x00000000000000000000000000000005;masks[274]=0x00000000000000000000000000000005;masks[275]=0x0000000000000000000000000000003d;
    masks[277]=0x00000000000000000000000000000005;masks[281]=0x00000000000000000000000000000005;masks[282]=0x00000000000000000000000000000005;
    masks[283]=0x00000000000000000000000000000005;masks[290]=0x00000000000000000000000000000005;masks[291]=0x00000000000000000000000000000005;
    masks[292]=0x00000000000000000000000000000005;masks[293]=0x00000000000000000000000000000005;masks[296]=0x00000000000000000000000000000005;
    masks[297]=0x00000000000000000000000000000005;masks[298]=0x00000000000000000000000000000005;masks[299]=0x00000000000000000000000000000005;
    masks[302]=0x00000000000000000000000000000005;masks[303]=0x00000000000000000000000000000005;masks[304]=0x00000000000000000000000000000005;
    masks[305]=0x00000000000000000000000000000005;masks[314]=0x00000000000000000000000000000005;masks[315]=0x00000000000000000000000000000005;
    masks[325]=0x00000000000000000000000000000005;masks[326]=0x00000000000000000000000000000005;masks[328]=0x0000000000000000000000000000002d;
    masks[329]=0x00000000000000000000000000000035;masks[330]=0x0000000000000000000000000000003d;masks[331]=0x00000000000000000000000000000005;
    masks[332]=0x00000000000000000000000000000005;masks[334]=0x00000000000000000000000000000005;masks[335]=0x00000000000000000000000000000005;
    masks[336]=0x00000000000000000000000000000008;masks[337]=0x00000000000000000000000000000010;masks[338]=0x0000000000000000000000000000000d;
    masks[339]=0x00000000000000000000000000000015;masks[340]=0x0000000000000000000000000000000d;masks[341]=0x00000000000000000000000000000015;
    masks[342]=0x00000000000000000000000000000005;masks[343]=0x00000000000000000000000000000005;masks[344]=0x00000000000000000000000000000020;
    masks[345]=0x00000000000000000000000000000025;masks[346]=0x00000000000000000000000000000025;masks[349]=0x00000000000000000000000000000005;
    masks[350]=0x00000000000000000000000000000005;masks[351]=0x00000000000000000000000000000005;masks[352]=0x00000000000000000000000000000005;
    masks[353]=0x00000000000000000000000000000005;masks[354]=0x00000000000000000000000000000005;masks[355]=0x00000000000000000000000000000005;
    masks[356]=0x00000000000000000000000000000005;masks[357]=0x00000000000000000000000000000005;masks[358]=0x00000000000000000000000000000005;
    masks[359]=0x00000000000000000000000000000005;masks[360]=0x00000000000000000000000000000005;masks[361]=0x00000000000000000000000000000005;
    masks[364]=0x00000000000000000000000000000005;masks[365]=0x00000000000000000000000000000005;masks[366]=0x00000000000000000000000000000005;
    masks[367]=0x00000000000000000000000000000005;masks[370]=0x00000000000000000000000000000005;masks[371]=0x00000000000000000000000000000005;
    masks[372]=0x00000000000000000000000000000005;masks[437]=0x00000000000000000000000000000005;
    masks
};
const REACTIVE_BRANCH_DERIVATIVE_ACTIVITY: [u128; Instance::VARIABLE_COUNT] = [0; Instance::VARIABLE_COUNT];
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
        current[slot] = value;previous[slot] = value;older[slot] = value;derivative_current[slot] = 0.0;derivative_previous[slot] = 0.0;initialized[slot] = true;
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
        previous[slot] = current_value;initialized[slot] = true;
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
        let p = Box::as_ref(&self.params);let nodes = &(*self).nodes;let branches = &(*self).branches;let param_given = self.param_given.as_ref();let multiplicity = (*self).multiplicity;let timestep = (*self).timestep;let ddt_state_current = self.ddt_state_current.as_mut();let ddt_state_previous = self.ddt_state_previous.as_mut();let ddt_state_older = self.ddt_state_older.as_mut();let ddt_state_initialized = self.ddt_state_initialized.as_mut();let ddt_derivative_current = self.ddt_derivative_current.as_mut();let ddt_derivative_previous = self.ddt_derivative_previous.as_mut();let ddt_active = self.ddt_coefficients.active;let ddt_scale = self.ddt_coefficients.derivative_scale;let ddt_previous_value_scale = self.ddt_coefficients.previous_value_scale;let ddt_older_value_scale = self.ddt_coefficients.older_value_scale;let ddt_previous_derivative_scale = self.ddt_coefficients.previous_derivative_scale;
        let s = match &mut self.scratch {
            Some(buf) => buf.as_mut(),
            slot @ None => slot.insert(Scratch::new_box_with_activity(&TRANSIENT_NODE_DERIVATIVE_ACTIVITY, &TRANSIENT_BRANCH_DERIVATIVE_ACTIVITY)).as_mut(),
        };Self::stamp_transient_block_0(ctx, s, p, param_given);Self::stamp_transient_block_1(s);Self::stamp_transient_block_2(s, p);Self::stamp_transient_block_3(s);Self::stamp_transient_block_4(s, p);Self::stamp_transient_block_5(s, p);Self::stamp_transient_block_6(s, p);Self::stamp_transient_block_7(s, p);Self::stamp_transient_block_8(s, p);Self::stamp_transient_block_9(s, p);Self::stamp_transient_block_10(s, p);Self::stamp_transient_block_11(s, p);Self::stamp_transient_block_12(s);Self::stamp_transient_block_13(s, p);Self::stamp_transient_block_14(s);Self::stamp_transient_block_15(s, p);Self::stamp_transient_block_16(s);Self::stamp_transient_block_17(s, p);Self::stamp_transient_block_18(s, p);Self::stamp_transient_block_19(s, p);Self::stamp_transient_block_20(s, p);Self::stamp_transient_block_21(s, p);Self::stamp_transient_block_22(s, p);Self::stamp_transient_block_23(s, p);Self::stamp_transient_block_24(s);Self::stamp_transient_block_25(s, p);Self::stamp_transient_block_26(s);Self::stamp_transient_block_27(s, p);Self::stamp_transient_block_28(s);Self::stamp_transient_block_29(s, p);Self::stamp_transient_block_30(s, p);Self::stamp_transient_block_31(s, p);Self::stamp_transient_block_32(s, p);Self::stamp_transient_block_33(s, p);Self::stamp_transient_block_34(s, p);Self::stamp_transient_block_35(s, p);Self::stamp_transient_block_36(s, p);Self::stamp_transient_block_37(s);Self::stamp_transient_block_38(s, p);Self::stamp_transient_block_39(s);Self::stamp_transient_block_40(s, p);Self::stamp_transient_block_41(s);Self::stamp_transient_block_42(s, p);Self::stamp_transient_block_43(s, p);Self::stamp_transient_block_44(s, p);Self::stamp_transient_block_45(s, p);Self::stamp_transient_block_46(s, p);Self::stamp_transient_block_47(s, p);Self::stamp_transient_block_48(s, p);Self::stamp_transient_block_49(s);Self::stamp_transient_block_50(s, p);Self::stamp_transient_block_51(s);Self::stamp_transient_block_52(s, p);Self::stamp_transient_block_53(s);Self::stamp_transient_block_54(s, p);Self::stamp_transient_block_55(s, p);Self::stamp_transient_block_56(s, p);Self::stamp_transient_block_57(s, p);Self::stamp_transient_block_58(s, p);Self::stamp_transient_block_59(s, p);Self::stamp_transient_block_60(s, p);Self::stamp_transient_block_61(s, p);Self::stamp_transient_block_62(s);Self::stamp_transient_block_63(s, p);Self::stamp_transient_block_64(s);Self::stamp_transient_block_65(s, p);Self::stamp_transient_block_66(s);Self::stamp_transient_block_67(s, p);Self::stamp_transient_block_68(s);Self::stamp_transient_block_69(ctx, s, nodes);Self::stamp_transient_block_70(s, p);Self::stamp_transient_block_71(s, p);Self::stamp_transient_block_72(s, p);Self::stamp_transient_block_73(s, p);Self::stamp_transient_block_74(s, p);Self::stamp_transient_block_75(s, p);Self::stamp_transient_block_76(s, p);Self::stamp_transient_block_77(s);Self::stamp_transient_block_78(s, p);Self::stamp_transient_block_79(s);Self::stamp_transient_block_80(s, p);Self::stamp_transient_block_81(s);Self::stamp_transient_block_82(ctx, s, p, nodes);Self::stamp_transient_block_83(s, p);Self::stamp_transient_block_84(ctx, s, p, nodes);
        stamper.stamp_potential_branch_local(
            Some(2),
            Some(1),
            0,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(3),
            None,
            1,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(4),
            None,
            2,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(5),
            None,
            3,
            multiplicity,
        );Self::stamp_transient_equations_block_0(ctx, stamper, s, nodes, multiplicity, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous);Self::stamp_transient_equations_block_1(stamper, s, multiplicity, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous);
    }
    pub fn stamp_reactive(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedReactiveStamper<'_>) {
        let p = Box::as_ref(&self.params);let nodes = &(*self).nodes;let branches = &(*self).branches;let param_given = self.param_given.as_ref();let multiplicity = (*self).multiplicity;
        let s = match &mut self.reactive_scratch {
            Some(buf) => buf.as_mut(),
            slot @ None => slot.insert(ReactiveScratch::new_box_with_activity(&REACTIVE_NODE_DERIVATIVE_ACTIVITY, &REACTIVE_BRANCH_DERIVATIVE_ACTIVITY)).as_mut(),
        };Self::stamp_reactive_block_0(ctx, s, p, param_given);Self::stamp_reactive_block_1(s);Self::stamp_reactive_block_2(s, p);Self::stamp_reactive_block_3(s);Self::stamp_reactive_block_4(s, p);Self::stamp_reactive_block_5(s, p);Self::stamp_reactive_block_6(s, p);Self::stamp_reactive_block_7(s, p);Self::stamp_reactive_block_8(s, p);Self::stamp_reactive_block_9(s, p);Self::stamp_reactive_block_10(s, p);Self::stamp_reactive_block_11(s, p);Self::stamp_reactive_block_12(s);Self::stamp_reactive_block_13(s, p);Self::stamp_reactive_block_14(s);Self::stamp_reactive_block_15(s, p);Self::stamp_reactive_block_16(s, p);Self::stamp_reactive_block_17(s, p);Self::stamp_reactive_block_18(s, p);Self::stamp_reactive_block_19(s, p);Self::stamp_reactive_block_20(s, p);Self::stamp_reactive_block_21(s, p);Self::stamp_reactive_block_22(s, p);Self::stamp_reactive_block_23(s, p);Self::stamp_reactive_block_24(s);Self::stamp_reactive_block_25(s, p);Self::stamp_reactive_block_26(s);Self::stamp_reactive_block_27(s, p);Self::stamp_reactive_block_28(s);Self::stamp_reactive_block_29(s, p);Self::stamp_reactive_block_30(s, p);Self::stamp_reactive_block_31(s, p);Self::stamp_reactive_block_32(s, p);Self::stamp_reactive_block_33(s, p);Self::stamp_reactive_block_34(s, p);Self::stamp_reactive_block_35(s, p);Self::stamp_reactive_block_36(s);Self::stamp_reactive_block_37(s, p);Self::stamp_reactive_block_38(s);Self::stamp_reactive_block_39(s, p);Self::stamp_reactive_block_40(s);Self::stamp_reactive_block_41(s, p);Self::stamp_reactive_block_42(s, p);Self::stamp_reactive_block_43(s, p);Self::stamp_reactive_block_44(s, p);Self::stamp_reactive_block_45(s, p);Self::stamp_reactive_block_46(s, p);Self::stamp_reactive_block_47(s, p);Self::stamp_reactive_block_48(s, p);Self::stamp_reactive_block_49(s);Self::stamp_reactive_block_50(s, p);Self::stamp_reactive_block_51(s);Self::stamp_reactive_block_52(s, p);Self::stamp_reactive_block_53(s);Self::stamp_reactive_block_54(s, p);Self::stamp_reactive_block_55(s, p);Self::stamp_reactive_block_56(s, p);Self::stamp_reactive_block_57(s, p);Self::stamp_reactive_block_58(s, p);Self::stamp_reactive_block_59(s, p);Self::stamp_reactive_block_60(s, p);Self::stamp_reactive_block_61(s);Self::stamp_reactive_block_62(s, p);Self::stamp_reactive_block_63(s);Self::stamp_reactive_block_64(s, p);Self::stamp_reactive_block_65(s);Self::stamp_reactive_block_66(s, p);Self::stamp_reactive_block_67(s);Self::stamp_reactive_block_68(ctx, s, nodes);Self::stamp_reactive_block_69(s);Self::stamp_reactive_block_70(s, p);Self::stamp_reactive_block_71(s, p);Self::stamp_reactive_block_72(s, p);Self::stamp_reactive_block_73(s, p);Self::stamp_reactive_block_74(s, p);Self::stamp_reactive_block_75(s, p);Self::stamp_reactive_block_76(s);Self::stamp_reactive_block_77(s, p);Self::stamp_reactive_block_78(s);Self::stamp_reactive_block_79(s, p);Self::stamp_reactive_block_80(s);Self::stamp_reactive_block_81(s, p);Self::stamp_reactive_block_82(s, p);Self::stamp_reactive_block_83(ctx, s, p, nodes);Self::stamp_reactive_block_84(s, p);Self::stamp_reactive_equations_block_0(stamper, s, multiplicity);Self::stamp_reactive_equations_block_1(stamper, s, multiplicity);
    }
}
