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
        let nv4 = ctx.node_voltage(nodes[4]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let nv14 = ctx.node_voltage(nodes[14]);
        let param_given = self.param_given.as_ref();
        let multiplicity = (*self).multiplicity;
        let timestep = (*self).timestep;
        let ddt_state_current = self.ddt_state_current.as_mut();
        let ddt_state_previous = self.ddt_state_previous.as_mut();
        let ddt_state_initialized = self.ddt_state_initialized.as_mut();
        let ddt_active = timestep.abs() > Instance::DDT_EPSILON;
        let ddt_scale = if ddt_active { 1.0 / timestep } else { 0.0 };
        let v0: f64 = 1.0;
        let v1: f64 = 0.0;
        let v36: f64 = 10000.0;
        let v307: f64 = ctx_temp;
        let v309: f64 = (v307 + self.scalar_v308);
        let v310: f64 = (if self.scalar_v306 { v309 } else { v1 });
        let v311: f64 = (if self.scalar_v306 { v310 } else { v1 });
        let v312: f64 = (v310 + v1);
        let v313: f64 = (if self.scalar_v306 { v312 } else { v310 });
        let v314: f64 = (v311 - self.scalar_v40);
        let v315: f64 = (if self.scalar_v306 { v314 } else { v1 });
        let v316: f64 = (v311 * v311);
        let v318: f64 = (v316 - self.scalar_v317);
        let v319: f64 = (if self.scalar_v306 { v318 } else { v1 });
        let v321: f64 = (self.scalar_v17 * v315);
        let v322: f64 = (self.scalar_v201 + v321);
        let v323: f64 = (self.scalar_v19 * v319);
        let v324: f64 = (v322 + v323);
        let v325: f64 = (v324 * self.scalar_v302);
        let v326: f64 = (if self.scalar_v320 { v325 } else { self.scalar_v299 });
        let v327: f64 = 0.0001;
        let v328: bool = (v326 < v327);
        let v329: bool = (self.scalar_v320 && v328);
        let v330: f64 = (if v329 { v327 } else { v326 });
        let v331: f64 = nv6;
        let v332: f64 = nv8;
        let v333: f64 = nv10;
        let v334: f64 = nv9;
        let v339: f64 = nv4;
        let v340: f64 = (if self.scalar_v338 { v309 } else { v313 });
        let v341: f64 = (if self.scalar_v338 { v340 } else { v311 });
        let v342: f64 = (v341 - self.scalar_v40);
        let v343: f64 = (if self.scalar_v338 { v342 } else { v315 });
        let v344: f64 = (v341 * v341);
        let v345: f64 = (v344 - self.scalar_v317);
        let v346: f64 = (if self.scalar_v338 { v345 } else { v319 });
        let v348: f64 = (self.scalar_v17 * v343);
        let v349: f64 = (self.scalar_v201 + v348);
        let v350: f64 = (self.scalar_v19 * v346);
        let v351: f64 = (v349 + v350);
        let v352: f64 = (v351 * self.scalar_v302);
        let v353: f64 = (if self.scalar_v347 { v352 } else { v330 });
        let v354: bool = (v353 < v327);
        let v355: bool = (self.scalar_v347 && v354);
        let v356: f64 = (if v355 { v327 } else { v353 });
        let v367: bool = (v356 > v327);
        let v368: bool = (self.scalar_v336 && v367);
        let v369: f64 = (v0 / v356);
        let v370: f64 = (if v368 { v369 } else { v1 });
        let v371: bool = (!v367);
        let v372: bool = (self.scalar_v336 && v371);
        let v373: f64 = (if v372 { v36 } else { v370 });
        let v375: f64 = (if self.scalar_v374 { v1 } else { v373 });
        let v387: f64 = nv14;
        let v388: f64 = nv1;
        let v389: f64 = (v388 - v331);
        let v390: f64 = (self.scalar_v252 * v389);
        let v391: f64 = (if self.scalar_v377 { v390 } else { v1 });
        let v394: f64 = (v333 - v332);
        let v395: f64 = (self.scalar_v285 * v394);
        let v396: f64 = (if (self.scalar_v253 != 0.0) { v395 } else { v1 });
        let v397: f64 = (v334 - v332);
        let v398: f64 = (self.scalar_v286 * v397);
        let v399: f64 = (if (self.scalar_v253 != 0.0) { v398 } else { v1 });
        let v400: f64 = nv3;
        let v401: f64 = (v400 - v332);
        let v402: f64 = (self.scalar_v284 * v401);
        let v403: f64 = (if (self.scalar_v253 != 0.0) { v402 } else { v1 });
        let v406: f64 = (v339 * v375);
        let v407: f64 = (if self.scalar_v336 { v406 } else { v1 });
        let v408: f64 = (v339 * v36);
        let v409: f64 = (if self.scalar_v374 { v408 } else { v1 });
        let v425: f64 = (if self.scalar_v336 { v375 } else { v1 });

        let d387_dn14: f64 = v0;
        stamper.stamp_current_node1_local(
            Some(14),
            None,
            multiplicity * (v387),
            14,
            multiplicity * (d387_dn14),
        );
        let d391_dn1: f64 = self.scalar_v414;
        let d391_dn6: f64 = self.scalar_v415;
        stamper.stamp_current_node2_local(
            Some(1),
            Some(6),
            multiplicity * (v391),
            1,
            multiplicity * (d391_dn1),
            6,
            multiplicity * (d391_dn6),
        );
        let d396_dn8: f64 = self.scalar_v417;
        let d396_dn10: f64 = self.scalar_v418;
        stamper.stamp_current_node2_local(
            Some(10),
            Some(8),
            multiplicity * (v396),
            8,
            multiplicity * (d396_dn8),
            10,
            multiplicity * (d396_dn10),
        );
        let d399_dn8: f64 = self.scalar_v420;
        let d399_dn9: f64 = self.scalar_v421;
        stamper.stamp_current_node2_local(
            Some(9),
            Some(8),
            multiplicity * (v399),
            8,
            multiplicity * (d399_dn8),
            9,
            multiplicity * (d399_dn9),
        );
        let d403_dn3: f64 = self.scalar_v423;
        let d403_dn8: f64 = self.scalar_v424;
        stamper.stamp_current_node2_local(
            Some(3),
            Some(8),
            multiplicity * (v403),
            3,
            multiplicity * (d403_dn3),
            8,
            multiplicity * (d403_dn8),
        );
        let d407_dn4: f64 = v425;
        stamper.stamp_current_node1_local(
            Some(4),
            None,
            multiplicity * (v407),
            4,
            multiplicity * (d407_dn4),
        );
        let d409_dn4: f64 = self.scalar_v426;
        stamper.stamp_current_node1_local(
            Some(4),
            None,
            multiplicity * (v409),
            4,
            multiplicity * (d409_dn4),
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
            Some(15),
            None,
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
            Some(0),
            Some(5),
            3,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(7),
            Some(2),
            4,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(1),
            Some(6),
            5,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(3),
            Some(8),
            6,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(10),
            Some(8),
            7,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(9),
            Some(8),
            8,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(11),
            None,
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

        Self::stamp_transient_equations_block_0(stamper, s, p, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_1(stamper, s, p, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_2(stamper, s, p, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_3(ctx, stamper, s, p, nodes, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_4(ctx, stamper, s, p, nodes, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_5(ctx, stamper, s, p, nodes, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
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
