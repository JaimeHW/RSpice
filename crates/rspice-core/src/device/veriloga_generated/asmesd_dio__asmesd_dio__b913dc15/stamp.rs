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
        let multiplicity = (*self).multiplicity;
        let timestep = (*self).timestep;
        let ddt_state_current = self.ddt_state_current.as_mut();
        let ddt_state_previous = self.ddt_state_previous.as_mut();
        let ddt_state_initialized = self.ddt_state_initialized.as_mut();
        let ddt_active = timestep.abs() > Instance::DDT_EPSILON;
        let ddt_scale = if ddt_active { 1.0 / timestep } else { 0.0 };
        let s = match &mut self.scratch {
            Some(buf) => buf.as_mut(),
            slot @ None => slot.insert(Scratch::new_box()).as_mut(),
        };

        s.store_offset_voltage(12, ctx, nodes, Some(2), None, ((ctx_temp) + (p.p45)));

        if ((1026.85 + 273.15) < (if (s.v[12] > ((-100.0) + 273.15)) { s.v[12] } else { ((-100.0) + 273.15) })) {
            s.store_scalar(10, (1026.85 + 273.15));
        } else {
            s.store_ad_value(10, {
                if (s.v[12] > ((-100.0) + 273.15)) {
                    s.ad_value(12)
                } else {
                    A::constant(((-100.0) + 273.15))
                }
            });
        }

        s.v[3] = (p.p43 * p.p42);

        s.v[11] = (p.p25 + 273.15);

        s.store_scale(15, 10, 8.6170869e-5);

        s.store_scale(13, 10, 1.0 / (s.v[11]));

        s.store_ln(14, 13);

        s.store_add_scaled_ad_rhs(34, 14, p.p22, A::div_scaled_offset_numerator(s.ad_value(13), p.p21, ((-1.0) * p.p21), s.ad_value(15), 1.0));

        s.store_scale(54, 14, p.p23);

        s.store_scaled_exp(16, 34, p.p0);

        s.store_scaled_exp(55, 54, p.p2);

        s.store_offset_scaled(19, 13, ((p.p7) * (p.p47)), (((((((-1.0)) * (p.p7))) + (1.0))) * (p.p47)));

        s.store_offset_scaled(20, 13, ((p.p6) * (p.p5)), (((((((-1.0)) * (p.p6))) + (1.0))) * (p.p5)));

        s.store_offset_scaled(21, 13, ((p.p10) * (p.p9)), (((((((-1.0)) * (p.p10))) + (1.0))) * (p.p9)));

        s.v[22] = p.p16;

        s.v[43] = (s.v[11] / 300.15);

        s.store_scale(44, 10, 0.003331667499583542);

        s.store_sub_from_scalar_ad(45, 1.16, A::div_scaled_product_offset_denominator(s.ad_value(10), s.ad_value(10), 0.000702, s.ad_value(10), 1108.0, 1.0));

        s.store_offset_ad(46, A::div_scaled_inputs(s.ad_value(45), -1.0, s.ad_value(10), (2.0 * 1.3806226e-23)), (1.1150877 / (1.3806226e-23 * (300.15 + 300.15))));

        s.store_scaled_mul_ad(47, A::scale(s.ad_value(15), 2.0), A::add_scaled_inputs(A::ln(s.ad_value(44)), 1.5, s.ad_value(46), 1.6021918e-19), -1.0);

        s.store_offset_scaled(48, 47, (-1.0 / (s.v[43])), ((p.p17) * (1.0 / (s.v[43]))));

        s.store_div_ad_lhs(49, A::sub_from_scalar(p.p17, s.ad_value(48)), 48);

        s.store_div_from_scalar_offset_scaled_input(51, s.v[22], 49, (-p.p18), (((((0.0004 * (s.v[11] - 300.15))) * (p.p18))) + (1.0)));

        s.store_add_scaled_product(18, s.ad_value(47), 1.0, s.ad_value(44), s.ad_value(48), 1.0);

        s.store_ad_value(50, A::div_scaled_inputs2(s.ad_value(18), 1.0, s.ad_value(48), (-1.0), s.ad_value(48), 1.0));

        s.store_mul_offset_ad_rhs(17, 51, A::sub_scaled_inputs(A::scaled_offset(s.ad_value(10), (-300.15), 0.0004), p.p18, s.ad_value(50), p.p18), 1.0);

        s.v[9] = p.p29;

        s.store_scaled_voltage(40, ctx, nodes, Some(3), Some(4), s.v[9]);

        s.store_scaled_voltage(41, ctx, nodes, Some(0), Some(3), s.v[9]);

        s.store_scaled_voltage(42, ctx, nodes, Some(1), Some(4), s.v[9]);

        s.b[63] = (s.v[16] > 0.0);
        s.v[63] = if s.b[63] { 1.0 } else { 0.0 };

        if s.b[63] {
            s.store_scaled_div(0, 40, 15, (1.0 / (p.p1)));
            s.store_ad_value(52, A::div_scaled_inputs2(s.ad_value(40), -1.0, s.ad_value(20), (-1.0), s.ad_value(15), p.p11));
            s.store_scaled_div(53, 20, 15, ((-1.0) * 1.0 / (p.p11)));
        }

        s.b[64] = (s.v[0] > 80.0);
        s.v[64] = if s.b[64] { 1.0 } else { 0.0 };

        if (s.b[63] && s.b[64]) {
            s.store_offset(1, 0, (((-80.0)) + (1.0)));
            s.store_scalar(0, 80.0);
        }

        if (s.b[63] && (!s.b[64])) {
            s.store_scalar(1, 1.0);
        }

        if s.b[63] {
            s.store_mul_exp_rhs(1, 1, 0);
        }

        if s.b[63] {
            let assign410_ad_e599: A = {
                if ((!(s.v[52] >= 37.0)) && (!(s.v[52] <= (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(52))
                } else {
                    {
                        if ((!(s.v[52] >= 37.0)) && (s.v[52] <= (-37.0))) {
                            A::exp(s.ad_value(52))
                        } else {
                            {
                                if (s.v[52] >= 37.0) {
                                    s.ad_value(52)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            let assign410_ad_e633: A = {
                if ((!(s.v[53] >= 37.0)) && (!(s.v[53] <= (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(53))
                } else {
                    {
                        if ((!(s.v[53] >= 37.0)) && (s.v[53] <= (-37.0))) {
                            A::exp(s.ad_value(53))
                        } else {
                            {
                                if (s.v[53] >= 37.0) {
                                    s.ad_value(53)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            s.store_sub_ad(2, assign410_ad_e599, assign410_ad_e633);
        }

        if s.b[63] {
            s.store_ad_value(23, A::add_scaled_offset_product_rhs(A::div_scaled_product(s.ad_value(19), s.ad_value(2), 1.0, A::scale_offset(A::pow(A::abs(s.ad_value(40)), s.ad_value(21)), p.p8, 1.0), 1.0), (-1.0), s.ad_value(16), s.ad_value(1), (-1.0), 1.0));
        }

        if (!s.b[63]) {
            s.store_scalar(23, 0.0);
        }

        s.b[65] = (s.v[55] > 0.0);
        s.v[65] = if s.b[65] { 1.0 } else { 0.0 };

        if s.b[65] {
            s.store_max_with_scalar_ad(60, A::sub_from_scalar(p.p4, s.ad_value(40)), 0.001);
            s.store_ad_value(0, A::div_scaled_inputs(s.ad_value(40), ((-1.0) * p.p4), A::mul_scaled_lhs(s.ad_value(15), p.p3, s.ad_value(60)), 1.0));
        }

        s.b[66] = (s.v[0] > 80.0);
        s.v[66] = if s.b[66] { 1.0 } else { 0.0 };

        if (s.b[65] && s.b[66]) {
            s.store_offset(1, 0, (((-80.0)) + (1.0)));
            s.store_scalar(0, 80.0);
        }

        if (s.b[65] && (!s.b[66])) {
            s.store_scalar(1, 1.0);
        }

        if s.b[65] {
            s.store_mul_exp_rhs(1, 1, 0);
            s.store_mul_offset_rhs(26, 55, 1, (-1.0));
        }

        if (!s.b[65]) {
            s.store_scalar(26, 0.0);
        }

        s.store_sub(24, 23, 26);

        s.store_offset_powf_ad(58, A::abs_scaled_input(s.ad_value(41), 1.0 / (p.p48)), p.p49, 1.0);

        s.store_offset_powf_ad(59, A::abs_scaled_input(s.ad_value(42), 1.0 / (p.p50)), p.p51, 1.0);

        s.store_scaled_mul_ad(29, A::exp_scaled_input(s.ad_value(14), p.p37), A::powf(s.ad_value(58), (1.0 / p.p49)), p.p12);

        s.store_scaled_mul_ad(30, A::exp_scaled_input(s.ad_value(14), p.p38), A::powf(s.ad_value(59), (1.0 / p.p51)), p.p14);

        s.b[67] = (p.p31 == 1.0);
        s.v[67] = if s.b[67] { 1.0 } else { 0.0 };

        if s.b[67] {
            s.store_offset(29, 29, p.p13);
            s.store_offset(30, 30, p.p15);
        }

        s.copy_ad(25, 23);

        s.store_powf_ad(56, A::abs_scaled_input(A::voltage(ctx, nodes, Some(0), Some(1)), 1.0 / (p.p40)), p.p39);

        s.store_offset_powf_ad(57, A::offset(s.ad_value(56), 1.0), (1.0 / p.p39), (-1.0));

        s.store_offset_scaled(31, 57, ((p.p41) * (p.p19)), p.p19);

        s.store_mul(32, 31, 25);

        s.b[68] = (p.p32 == 1.0);
        s.v[68] = if s.b[68] { 1.0 } else { 0.0 };

        if s.b[68] {
            s.store_ad_value(29, A::div_scaled_value_offset_denominator(s.ad_value(29), 1.0, A::powf(A::scale(A::abs(A::voltage(ctx, nodes, Some(6), None)), 1.0 / (p.p20)), p.p44), 1.0, 1.0));
        }

        if (!s.b[68]) {
        }

        s.store_scale(4, 18, (-p.p24));

        s.store_add(5, 40, 4);

        s.b[69] = (s.v[5] > 0.0);
        s.v[69] = if s.b[69] { 1.0 } else { 0.0 };

        if s.b[69] {
            s.store_scalar(6, (((((-1.0) - p.p18) * (((1.0 - p.p24)) as f64).ln())) as f64).exp());
            s.store_ad_value(7, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(18), 1.0, A::scale(s.ad_value(6), ((1.0 - p.p24) * (1.0 - p.p24))), 1.0 / ((1.0 - p.p18))));
            s.store_mul_ad_product_lhs(8, s.ad_value(5), A::offset(A::div_scaled_inputs(s.ad_value(5), (0.5 * p.p18), s.ad_value(18), 1.0), (1.0 - p.p24)), 6);
        }

        if (!s.b[69]) {
            s.store_ad_value(7, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(18), 1.0, A::exp_scaled_input(A::ln(A::sub_from_scalar(1.0, A::div(s.ad_value(40), s.ad_value(18)))), (1.0 - p.p18)), 1.0 / ((1.0 - p.p18))));
            s.store_scalar(8, 0.0);
        }

        s.store_mul_add_rhs(33, 17, 7, 8);

        s.b[70] = ((p.p30 == 1.0) && (p.p33 > 0.0));
        s.v[70] = if s.b[70] { 1.0 } else { 0.0 };

        s.b[71] = (((p.p30 == 2.0) && (p.p33 > 0.0)) && (p.p35 > 0.0));
        s.v[71] = if s.b[71] { 1.0 } else { 0.0 };

        s.b[72] = (p.p30 == (-1.0));
        s.v[72] = if s.b[72] { 1.0 } else { 0.0 };

        s.store_scale(35, 10, (4.0 * 1.3806226e-23));

        s.v[28] = ((p.p12 + (p.p31 * p.p13)) / s.v[3]);

        s.v[27] = ((p.p14 + (p.p31 * p.p15)) / s.v[3]);

        s.b[73] = ((s.v[28] > 0.0) && (s.v[28] >= p.p46));
        s.v[73] = if s.b[73] { 1.0 } else { 0.0 };

        if s.b[73] {
            s.store_ad_value(38, {
                if ((s.v[29] / s.v[3]) >= p.p46) {
                    A::div_scaled_inputs(s.ad_value(35), 1.0, s.ad_value(29), 1.0 / (s.v[3]))
                } else {
                    A::constant(0.0)
                }
            });
        }

        s.b[74] = ((s.v[27] > 0.0) && (s.v[27] >= p.p46));
        s.v[74] = if s.b[74] { 1.0 } else { 0.0 };

        if s.b[74] {
            s.store_ad_value(39, {
                if ((s.v[30] / s.v[3]) >= p.p46) {
                    A::div_scaled_inputs(s.ad_value(35), 1.0, s.ad_value(30), 1.0 / (s.v[3]))
                } else {
                    A::constant(0.0)
                }
            });
        }

        if (if ((p.p28 > 0.0) && (p.p27 > 0.0)) { 1.0 } else { 0.0 } > 0.0) {
            s.store_scaled_powf_ad(37, A::abs(s.ad_value(24)), p.p28, p.p27);
        } else {
            s.store_scalar(37, 0.0);
        }

        s.v[36] = (2.0 * 1.6021918e-19);

        stamper.stamp_potential_branch_local(
            Some(6),
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
            Some(5),
            None,
            2,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(2),
            None,
            3,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(5),
            None,
            4,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(0),
            Some(3),
            5,
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(1),
            Some(4),
            6,
            multiplicity,
        );

        Self::stamp_transient_equations_block_0(ctx, stamper, s, p, nodes, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
        Self::stamp_transient_equations_block_1(ctx, stamper, s, p, nodes, multiplicity, ddt_active, ddt_scale, ddt_state_current, ddt_state_previous, ddt_state_initialized);
    }

    pub fn stamp_reactive(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedReactiveStamper<'_>) {
        let p = Box::as_ref(&self.params);
        let nodes = &(*self).nodes;
        let branches = &(*self).branches;
        let ctx_temp = ctx.temperature();
        let multiplicity = (*self).multiplicity;
        let s = match &mut self.reactive_scratch {
            Some(buf) => buf.as_mut(),
            slot @ None => slot.insert(ReactiveScratch::new_box()).as_mut(),
        };

        s.store_offset_voltage(12, ctx, nodes, Some(2), None, ((ctx_temp) + (p.p45)));

        if ((1026.85 + 273.15) < (if (s.v[12] > ((-100.0) + 273.15)) { s.v[12] } else { ((-100.0) + 273.15) })) {
            s.store_scalar(10, (1026.85 + 273.15));
        } else {
            s.store_ad_value(10, {
                if (s.v[12] > ((-100.0) + 273.15)) {
                    s.ad_value(12)
                } else {
                    A::constant(((-100.0) + 273.15))
                }
            });
        }

        s.v[3] = (p.p43 * p.p42);

        s.v[11] = (p.p25 + 273.15);

        s.store_scale(15, 10, 8.6170869e-5);

        s.store_scale(13, 10, 1.0 / (s.v[11]));

        s.store_ln(14, 13);

        s.store_add_scaled_ad_rhs(34, 14, p.p22, A::div_scaled_offset_numerator(s.ad_value(13), p.p21, ((-1.0) * p.p21), s.ad_value(15), 1.0));

        s.store_scale(54, 14, p.p23);

        s.store_scaled_exp(16, 34, p.p0);

        s.store_scaled_exp(55, 54, p.p2);

        s.store_offset_scaled(19, 13, ((p.p7) * (p.p47)), (((((((-1.0)) * (p.p7))) + (1.0))) * (p.p47)));

        s.store_offset_scaled(20, 13, ((p.p6) * (p.p5)), (((((((-1.0)) * (p.p6))) + (1.0))) * (p.p5)));

        s.store_offset_scaled(21, 13, ((p.p10) * (p.p9)), (((((((-1.0)) * (p.p10))) + (1.0))) * (p.p9)));

        s.v[22] = p.p16;

        s.v[43] = (s.v[11] / 300.15);

        s.store_scale(44, 10, 0.003331667499583542);

        s.store_sub_from_scalar_ad(45, 1.16, A::div_scaled_product_offset_denominator(s.ad_value(10), s.ad_value(10), 0.000702, s.ad_value(10), 1108.0, 1.0));

        s.store_offset_ad(46, A::div_scaled_inputs(s.ad_value(45), -1.0, s.ad_value(10), (2.0 * 1.3806226e-23)), (1.1150877 / (1.3806226e-23 * (300.15 + 300.15))));

        s.store_scaled_mul_ad(47, A::scale(s.ad_value(15), 2.0), A::add_scaled_inputs(A::ln(s.ad_value(44)), 1.5, s.ad_value(46), 1.6021918e-19), -1.0);

        s.store_offset_scaled(48, 47, (-1.0 / (s.v[43])), ((p.p17) * (1.0 / (s.v[43]))));

        s.store_div_ad_lhs(49, A::sub_from_scalar(p.p17, s.ad_value(48)), 48);

        s.store_div_from_scalar_offset_scaled_input(51, s.v[22], 49, (-p.p18), (((((0.0004 * (s.v[11] - 300.15))) * (p.p18))) + (1.0)));

        s.store_add_scaled_product(18, s.ad_value(47), 1.0, s.ad_value(44), s.ad_value(48), 1.0);

        s.store_ad_value(50, A::div_scaled_inputs2(s.ad_value(18), 1.0, s.ad_value(48), (-1.0), s.ad_value(48), 1.0));

        s.store_mul_offset_ad_rhs(17, 51, A::sub_scaled_inputs(A::scaled_offset(s.ad_value(10), (-300.15), 0.0004), p.p18, s.ad_value(50), p.p18), 1.0);

        s.v[9] = p.p29;

        s.store_scaled_voltage(40, ctx, nodes, Some(3), Some(4), s.v[9]);

        s.b[63] = (s.v[16] > 0.0);
        s.v[63] = if s.b[63] { 1.0 } else { 0.0 };

        if s.b[63] {
            s.store_scaled_div(0, 40, 15, (1.0 / (p.p1)));
            s.store_ad_value(52, A::div_scaled_inputs2(s.ad_value(40), -1.0, s.ad_value(20), (-1.0), s.ad_value(15), p.p11));
            s.store_scaled_div(53, 20, 15, ((-1.0) * 1.0 / (p.p11)));
        }

        s.b[64] = (s.v[0] > 80.0);
        s.v[64] = if s.b[64] { 1.0 } else { 0.0 };

        if (s.b[63] && s.b[64]) {
            s.store_offset(1, 0, (((-80.0)) + (1.0)));
            s.store_scalar(0, 80.0);
        }

        if (s.b[63] && (!s.b[64])) {
            s.store_scalar(1, 1.0);
        }

        if s.b[63] {
            s.store_mul_exp_rhs(1, 1, 0);
        }

        if s.b[63] {
            let assign410_ad_e599: A = {
                if ((!(s.v[52] >= 37.0)) && (!(s.v[52] <= (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(52))
                } else {
                    {
                        if ((!(s.v[52] >= 37.0)) && (s.v[52] <= (-37.0))) {
                            A::exp(s.ad_value(52))
                        } else {
                            {
                                if (s.v[52] >= 37.0) {
                                    s.ad_value(52)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            let assign410_ad_e633: A = {
                if ((!(s.v[53] >= 37.0)) && (!(s.v[53] <= (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(53))
                } else {
                    {
                        if ((!(s.v[53] >= 37.0)) && (s.v[53] <= (-37.0))) {
                            A::exp(s.ad_value(53))
                        } else {
                            {
                                if (s.v[53] >= 37.0) {
                                    s.ad_value(53)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            s.store_sub_ad(2, assign410_ad_e599, assign410_ad_e633);
        }

        if s.b[63] {
            s.store_ad_value(23, A::add_scaled_offset_product_rhs(A::div_scaled_product(s.ad_value(19), s.ad_value(2), 1.0, A::scale_offset(A::pow(A::abs(s.ad_value(40)), s.ad_value(21)), p.p8, 1.0), 1.0), (-1.0), s.ad_value(16), s.ad_value(1), (-1.0), 1.0));
        }

        if (!s.b[63]) {
            s.store_scalar(23, 0.0);
        }

        s.b[65] = (s.v[55] > 0.0);
        s.v[65] = if s.b[65] { 1.0 } else { 0.0 };

        if s.b[65] {
            s.store_max_with_scalar_ad(60, A::sub_from_scalar(p.p4, s.ad_value(40)), 0.001);
            s.store_ad_value(0, A::div_scaled_inputs(s.ad_value(40), ((-1.0) * p.p4), A::mul_scaled_lhs(s.ad_value(15), p.p3, s.ad_value(60)), 1.0));
        }

        s.b[66] = (s.v[0] > 80.0);
        s.v[66] = if s.b[66] { 1.0 } else { 0.0 };

        if (s.b[65] && s.b[66]) {
            s.store_offset(1, 0, (((-80.0)) + (1.0)));
            s.store_scalar(0, 80.0);
        }

        if (s.b[65] && (!s.b[66])) {
            s.store_scalar(1, 1.0);
        }

        if s.b[65] {
            s.store_mul_exp_rhs(1, 1, 0);
        }

        s.copy_ad(25, 23);

        s.store_powf_ad(56, A::abs_scaled_input(A::voltage(ctx, nodes, Some(0), Some(1)), 1.0 / (p.p40)), p.p39);

        s.store_offset_powf_ad(57, A::offset(s.ad_value(56), 1.0), (1.0 / p.p39), (-1.0));

        s.store_offset_scaled(31, 57, ((p.p41) * (p.p19)), p.p19);

        s.store_mul(32, 31, 25);

        s.b[68] = (p.p32 == 1.0);
        s.v[68] = if s.b[68] { 1.0 } else { 0.0 };

        s.store_scale(4, 18, (-p.p24));

        s.store_add(5, 40, 4);

        s.b[69] = (s.v[5] > 0.0);
        s.v[69] = if s.b[69] { 1.0 } else { 0.0 };

        if s.b[69] {
            s.store_scalar(6, (((((-1.0) - p.p18) * (((1.0 - p.p24)) as f64).ln())) as f64).exp());
            s.store_ad_value(7, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(18), 1.0, A::scale(s.ad_value(6), ((1.0 - p.p24) * (1.0 - p.p24))), 1.0 / ((1.0 - p.p18))));
            s.store_mul_ad_product_lhs(8, s.ad_value(5), A::offset(A::div_scaled_inputs(s.ad_value(5), (0.5 * p.p18), s.ad_value(18), 1.0), (1.0 - p.p24)), 6);
        }

        if (!s.b[69]) {
            s.store_ad_value(7, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(18), 1.0, A::exp_scaled_input(A::ln(A::sub_from_scalar(1.0, A::div(s.ad_value(40), s.ad_value(18)))), (1.0 - p.p18)), 1.0 / ((1.0 - p.p18))));
            s.store_scalar(8, 0.0);
        }

        s.store_mul_add_rhs(33, 17, 7, 8);

        s.b[70] = ((p.p30 == 1.0) && (p.p33 > 0.0));
        s.v[70] = if s.b[70] { 1.0 } else { 0.0 };

        s.b[71] = (((p.p30 == 2.0) && (p.p33 > 0.0)) && (p.p35 > 0.0));
        s.v[71] = if s.b[71] { 1.0 } else { 0.0 };

        Self::stamp_reactive_equations_block_0(ctx, stamper, s, p, nodes, branches, multiplicity);
    }
}
