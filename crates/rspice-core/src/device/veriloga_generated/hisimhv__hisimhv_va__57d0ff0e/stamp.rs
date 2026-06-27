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
        let p = Box::as_ref(&self.params);
        let nodes = &(*self).nodes;
        let branches = &(*self).branches;
        let ctx_temp = ctx.temperature();
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
        let v38: f64 = 10000.0;
        let v309: f64 = ctx_temp;
        let v311: f64 = (v309 + self.scalar_v310);
        let v312: f64 = (if self.scalar_v308 { v311 } else { v2 });
        let v313: f64 = (if self.scalar_v308 { v312 } else { v2 });
        let v314: f64 = (v312 + v2);
        let v315: f64 = (if self.scalar_v308 { v314 } else { v312 });
        let v316: f64 = (v313 - self.scalar_v42);
        let v317: f64 = (if self.scalar_v308 { v316 } else { v2 });
        let v318: f64 = (v313 * v313);
        let v320: f64 = (v318 - self.scalar_v319);
        let v321: f64 = (if self.scalar_v308 { v320 } else { v2 });
        let v323: f64 = (self.scalar_v19 * v317);
        let v324: f64 = (self.scalar_v203 + v323);
        let v325: f64 = (self.scalar_v21 * v321);
        let v326: f64 = (v324 + v325);
        let v327: f64 = (v326 * self.scalar_v304);
        let v328: f64 = (if self.scalar_v322 { v327 } else { self.scalar_v301 });
        let v329: f64 = 0.0001;
        let v330: bool = (v328 < v329);
        let v331: bool = (self.scalar_v322 && v330);
        let v332: f64 = (if v331 { v329 } else { v328 });
        let v333: f64 = nv7;
        let v334: f64 = nv9;
        let v335: f64 = nv11;
        let v336: f64 = nv10;
        let v341: f64 = nv5;
        let v342: f64 = (if self.scalar_v340 { v311 } else { v315 });
        let v343: f64 = (if self.scalar_v340 { v342 } else { v313 });
        let v344: f64 = (v343 - self.scalar_v42);
        let v345: f64 = (if self.scalar_v340 { v344 } else { v317 });
        let v346: f64 = (v343 * v343);
        let v347: f64 = (v346 - self.scalar_v319);
        let v348: f64 = (if self.scalar_v340 { v347 } else { v321 });
        let v350: f64 = (self.scalar_v19 * v345);
        let v351: f64 = (self.scalar_v203 + v350);
        let v352: f64 = (self.scalar_v21 * v348);
        let v353: f64 = (v351 + v352);
        let v354: f64 = (v353 * self.scalar_v304);
        let v355: f64 = (if self.scalar_v349 { v354 } else { v332 });
        let v356: bool = (v355 < v329);
        let v357: bool = (self.scalar_v349 && v356);
        let v358: f64 = (if v357 { v329 } else { v355 });
        let v369: bool = (v358 > v329);
        let v370: bool = (self.scalar_v338 && v369);
        let v371: f64 = (v0 / v358);
        let v372: f64 = (if v370 { v371 } else { v2 });
        let v373: bool = (!v369);
        let v374: bool = (self.scalar_v338 && v373);
        let v375: f64 = (if v374 { v38 } else { v372 });
        let v377: f64 = (if self.scalar_v376 { v2 } else { v375 });
        let v390: f64 = nv15;
        let v391: f64 = nv1;
        let v392: f64 = (v391 - v333);
        let v393: f64 = (self.scalar_v254 * v392);
        let v394: f64 = (if self.scalar_v379 { v393 } else { v2 });
        let v397: f64 = (v335 - v334);
        let v398: f64 = (self.scalar_v287 * v397);
        let v399: f64 = (if (self.scalar_v255 != 0.0) { v398 } else { v2 });
        let v400: f64 = (v336 - v334);
        let v401: f64 = (self.scalar_v288 * v400);
        let v402: f64 = (if (self.scalar_v255 != 0.0) { v401 } else { v2 });
        let v403: f64 = nv3;
        let v404: f64 = (v403 - v334);
        let v405: f64 = (self.scalar_v286 * v404);
        let v406: f64 = (if (self.scalar_v255 != 0.0) { v405 } else { v2 });
        let v409: f64 = (v341 * v377);
        let v410: f64 = (if self.scalar_v338 { v409 } else { v2 });
        let v411: f64 = (v341 * v38);
        let v412: f64 = (if self.scalar_v376 { v411 } else { v2 });
        let v428: f64 = (if self.scalar_v338 { v377 } else { v2 });

        let d390_dn15: f64 = v0;
        stamper.stamp_current_node1_local(
            Some(15),
            None,
            multiplicity * (v390),
            15,
            multiplicity * (d390_dn15),
        );
        let d394_dn1: f64 = self.scalar_v417;
        let d394_dn7: f64 = self.scalar_v418;
        stamper.stamp_current_node2_local(
            Some(1),
            Some(7),
            multiplicity * (v394),
            1,
            multiplicity * (d394_dn1),
            7,
            multiplicity * (d394_dn7),
        );
        let d399_dn9: f64 = self.scalar_v420;
        let d399_dn11: f64 = self.scalar_v421;
        stamper.stamp_current_node2_local(
            Some(11),
            Some(9),
            multiplicity * (v399),
            9,
            multiplicity * (d399_dn9),
            11,
            multiplicity * (d399_dn11),
        );
        let d402_dn9: f64 = self.scalar_v423;
        let d402_dn10: f64 = self.scalar_v424;
        stamper.stamp_current_node2_local(
            Some(10),
            Some(9),
            multiplicity * (v402),
            9,
            multiplicity * (d402_dn9),
            10,
            multiplicity * (d402_dn10),
        );
        let d406_dn3: f64 = self.scalar_v426;
        let d406_dn9: f64 = self.scalar_v427;
        stamper.stamp_current_node2_local(
            Some(3),
            Some(9),
            multiplicity * (v406),
            3,
            multiplicity * (d406_dn3),
            9,
            multiplicity * (d406_dn9),
        );
        let d410_dn5: f64 = v428;
        stamper.stamp_current_node1_local(
            Some(5),
            None,
            multiplicity * (v410),
            5,
            multiplicity * (d410_dn5),
        );
        let d412_dn5: f64 = self.scalar_v429;
        stamper.stamp_current_node1_local(
            Some(5),
            None,
            multiplicity * (v412),
            5,
            multiplicity * (d412_dn5),
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

        stamper.stamp_potential_branch_local(
            Some(4),
            Some(5),
            0,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(16),
            None,
            1,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(17),
            None,
            2,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(18),
            None,
            3,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(0),
            Some(6),
            4,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(8),
            Some(2),
            5,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(1),
            Some(7),
            6,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(3),
            Some(9),
            7,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(11),
            Some(9),
            8,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(10),
            Some(9),
            9,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(12),
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
            Some(14),
            None,
            12,
            multiplicity,
        );

        Self::stamp_transient_equations_block_0(stamper, s, p, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_1(stamper, s, p, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_2(stamper, s, p, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_3(ctx, stamper, s, p, nodes, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_4(stamper, s, p, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_5(ctx, stamper, s, p, nodes, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_6(ctx, stamper, s, p, nodes, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
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
        Self::stamp_reactive_equations_block_1(stamper, s, p, nodes, branches, multiplicity);
        Self::stamp_reactive_equations_block_2(ctx, stamper, s, p, nodes, branches, multiplicity);
        Self::stamp_reactive_equations_block_3(ctx, stamper, s, p, nodes, branches, multiplicity);
    }
}
