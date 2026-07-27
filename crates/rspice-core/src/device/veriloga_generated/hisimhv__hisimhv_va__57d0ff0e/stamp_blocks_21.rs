#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_reactive_block_115(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
    ) {
        if ((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && (!s.b[2462])) {s.store_div_from_scalar(335, 1.0, 339);s.store_scale(338, 335, 1.034943e-10);s.store_scalar(335, (1.0 - s.v[507]));s.store_add_scaled_inputs_product_indices(168, 790, s.v[507], 87, s.v[507], 335, 91, 1.0);}
        s.b[2469] = ((s.v[168] > (((s.v[87] + s.v[790]) - (10.0 * 2.220446049250313e-16)) - (10.0 * 2.220446049250313e-16))) && ((10.0 * 2.220446049250313e-16) >= 0.0));s.store_scalar(2469, if s.b[2469] { 1.0 } else { 0.0 });
        if (((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && (!s.b[2462])) && s.b[2469]) {s.store_offset_add_scaled_inputs3_offset_indices(781, 168, 1.0, 87, -1.0, 790, -1.0, (-(-(10.0 * 2.220446049250313e-16))), (10.0 * 2.220446049250313e-16));s.store_square(722, 781);s.store_scalar(723, ((10.0 * 2.220446049250313e-16) * (10.0 * 2.220446049250313e-16)));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2470] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(2470, if s.b[2470] { 1.0 } else { 0.0 });s.b[2471] = (2.0 == 1.0);s.store_scalar(2471, if s.b[2471] { 1.0 } else { 0.0 });
        if (((((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && (!s.b[2462])) && s.b[2469]) && s.b[2470]) && s.b[2471]) {s.store_scalar(720, 1.0);}
        s.b[2472] = (2.0 == 2.0);s.store_scalar(2472, if s.b[2472] { 1.0 } else { 0.0 });
        if ((((((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && (!s.b[2462])) && s.b[2469]) && s.b[2470]) && (!s.b[2471])) && s.b[2472]) {s.store_scalar(720, 2.0);}
        s.b[2473] = (2.0 == 4.0);s.store_scalar(2473, if s.b[2473] { 1.0 } else { 0.0 });
        if (((((((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && (!s.b[2462])) && s.b[2469]) && s.b[2470]) && (!s.b[2471])) && (!s.b[2472])) && s.b[2473]) {s.store_scalar(720, 3.0);}
        s.b[2474] = (2.0 == 8.0);s.store_scalar(2474, if s.b[2474] { 1.0 } else { 0.0 });
        if ((((((((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && (!s.b[2462])) && s.b[2469]) && s.b[2470]) && (!s.b[2471])) && (!s.b[2472])) && (!s.b[2473])) && s.b[2474]) {s.store_scalar(720, 4.0);}
        if ((((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && (!s.b[2462])) && s.b[2469]) && s.b[2470]) {s.store_scalar(719, 0.0);}
        let mut t1: usize = 0;
        while {
            let t0: f64 = if (((((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && (!s.b[2462])) && s.b[2469]) && s.b[2470]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t0 != 0.0
        } {
            t1 += 1;
            if t1 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t1, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && (!s.b[2462])) && s.b[2469]) && s.b[2470]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
        }
        if ((((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && (!s.b[2462])) && s.b[2469]) && (!s.b[2470])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if (((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && (!s.b[2462])) && s.b[2469]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, (10.0 * 2.220446049250313e-16));s.store_div_scaled_product_indices(334, 725, 726, (10.0 * 2.220446049250313e-16), 770, 1.0);s.store_add_scaled_inputs3_offset_indices(168, 87, 1.0, 790, 1.0, 780, 1.0, (((-(10.0 * 2.220446049250313e-16))) + ((-(10.0 * 2.220446049250313e-16)))));}
        if (((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && (!s.b[2462])) && s.b[2469]) {
        }
        if (((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && (!s.b[2462])) && (!s.b[2469])) {
        }
        if (((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && (!s.b[2462])) && (!s.b[2469])) {s.store_scalar(334, 1.0);}
        if ((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && (!s.b[2462])) {s.store_sub(340, 168, 91);s.store_mul(337, 154, 238);s.store_div_from_scalar(335, 1.0, 337);s.store_mul_ad_product_lhs_mixed_ai(339, A::offset(s.ad_value(94), (10.0 * 2.220446049250313e-16)), 250, 335);s.store_mul(336, 339, 154);s.store_scale(344, 166, 9662367879.197212);s.store_scalar(335, 100000.0);s.store_div_from_scalar(336, 1.0, 162);s.store_mul_mixed_ai(345, A::add_scaled_inputs_product(s.ad_value(339), 2.0, A::mul3_scaled_output(s.ad_value(344), s.ad_value(340), s.ad_value(338), 2.0), 1.0, s.ad_value(335), s.ad_value(338), 1.0), 336);s.store_mul(337, 336, 338);s.store_mul(341, 345, 338);s.store_add_scaled_product_indices(345, 335, 4.0, 344, 340, (2.0 * 4.0));s.store_mul3_affine_lhs(335, 344, 338, 8.0, 0.0, 338);s.store_scaled_mul(336, 345, 338, 2.0);s.store_mul3_lhs(342, 345, 338, 338);s.store_sqrt_square_add(343, 341, 342);s.store_scaled_sub(169, 343, 341, 0.5);s.copy_ad(335, 169);s.store_mul(169, 208, 335);}
        if (((!s.b[1443]) && s.b[2430]) && s.b[2461]) {s.store_scale(169, 169, s.v[619]);s.store_add(335, 85, 155);s.store_add_scaled_product_indices(336, 269, (-1.0), 335, 267, 1.0);s.store_mul_mixed_ia(240, 209, A::add_scaled_products(s.ad_value(209), A::add_scaled_sub_value_product(1.5, A::offset(s.ad_value(99), 1.0), 1.0, s.ad_value(154), s.ad_value(94), (-0.5)), 1.0, s.ad_value(185), s.ad_value(336), 1.0));s.copy_ad(335, 154);s.store_div_scaled_product_indices(131, 335, 240, 1.0, 250, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_116(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
    ) {
        if (((!s.b[1443]) && s.b[2430]) && s.b[2461]) {s.store_scale(335, 212, 2.0);s.store_mul_sub_rhs(241, 335, 267, 100);s.store_scaled_sub(336, 267, 100, 2.0);s.store_add(126, 94, 241);s.store_div_from_scalar(335, 1.0, 127);s.store_mul(336, 126, 335);s.store_sub_from_scalar(337, 1.0, 336);s.store_sub_from_scalar(332, 1.0, 337);s.store_square(722, 332);s.store_scalar(723, 1.0);s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2475] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));s.store_scalar(2475, if s.b[2475] { 1.0 } else { 0.0 });s.b[2476] = (4.0 == 1.0);s.store_scalar(2476, if s.b[2476] { 1.0 } else { 0.0 });
        if (((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && s.b[2475]) && s.b[2476]) {s.store_scalar(720, 1.0);}
        s.b[2477] = (4.0 == 2.0);s.store_scalar(2477, if s.b[2477] { 1.0 } else { 0.0 });
        if ((((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && s.b[2475]) && (!s.b[2476])) && s.b[2477]) {s.store_scalar(720, 2.0);}
        s.b[2478] = (4.0 == 4.0);s.store_scalar(2478, if s.b[2478] { 1.0 } else { 0.0 });
        if (((((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && s.b[2475]) && (!s.b[2476])) && (!s.b[2477])) && s.b[2478]) {s.store_scalar(720, 3.0);}
        s.b[2479] = (4.0 == 8.0);s.store_scalar(2479, if s.b[2479] { 1.0 } else { 0.0 });
        if ((((((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && s.b[2475]) && (!s.b[2476])) && (!s.b[2477])) && (!s.b[2478])) && s.b[2479]) {s.store_scalar(720, 4.0);}
        if ((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && s.b[2475]) {s.store_scalar(719, 0.0);}
        let mut t3: usize = 0;
        while {
            let t2: f64 = if (((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && s.b[2475]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t2 != 0.0
        } {
            t3 += 1;
            if t3 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t3, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && s.b[2475]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
        }
        if ((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && (!s.b[2475])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 4.0)));
            }
        }
        if (((!s.b[1443]) && s.b[2430]) && s.b[2461]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(333, 332, 726, 1.0);s.store_div_scaled_product_indices(338, 725, 726, 1.0, 770, 1.0);s.store_sub_from_scalar(125, 1.0, 333);s.store_offset_mul_offset_rhs(242, 125, 125, 1.0, 1.0);}
        s.b[2480] = (((1.0 + s.v[125]) < ((10.0 * 2.220446049250313e-16) + (10.0 * 2.220446049250313e-16))) && ((10.0 * 2.220446049250313e-16) >= 0.0));s.store_scalar(2480, if s.b[2480] { 1.0 } else { 0.0 });
        if ((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && s.b[2480]) {s.store_sub_from_scalar_ad(781, ((10.0 * 2.220446049250313e-16) + (10.0 * 2.220446049250313e-16)), A::offset(s.ad_value(125), 1.0));s.store_square(722, 781);s.store_scalar(723, ((10.0 * 2.220446049250313e-16) * (10.0 * 2.220446049250313e-16)));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2481] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(2481, if s.b[2481] { 1.0 } else { 0.0 });s.b[2482] = (2.0 == 1.0);s.store_scalar(2482, if s.b[2482] { 1.0 } else { 0.0 });
        if ((((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && s.b[2480]) && s.b[2481]) && s.b[2482]) {s.store_scalar(720, 1.0);}
        s.b[2483] = (2.0 == 2.0);s.store_scalar(2483, if s.b[2483] { 1.0 } else { 0.0 });
        if (((((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && s.b[2480]) && s.b[2481]) && (!s.b[2482])) && s.b[2483]) {s.store_scalar(720, 2.0);}
        s.b[2484] = (2.0 == 4.0);s.store_scalar(2484, if s.b[2484] { 1.0 } else { 0.0 });
        if ((((((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && s.b[2480]) && s.b[2481]) && (!s.b[2482])) && (!s.b[2483])) && s.b[2484]) {s.store_scalar(720, 3.0);}
        s.b[2485] = (2.0 == 8.0);s.store_scalar(2485, if s.b[2485] { 1.0 } else { 0.0 });
        if (((((((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && s.b[2480]) && s.b[2481]) && (!s.b[2482])) && (!s.b[2483])) && (!s.b[2484])) && s.b[2485]) {s.store_scalar(720, 4.0);}
        if (((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && s.b[2480]) && s.b[2481]) {s.store_scalar(719, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_117(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        let mut t5: usize = 0;
        while {
            let t4: f64 = if ((((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && s.b[2480]) && s.b[2481]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t4 != 0.0
        } {
            t5 += 1;
            if t5 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t5, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if (((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && s.b[2480]) && s.b[2481]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
        }
        if (((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && s.b[2480]) && (!s.b[2481])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if ((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && s.b[2480]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, (10.0 * 2.220446049250313e-16));s.store_div_scaled_product_indices(334, 725, 726, (10.0 * 2.220446049250313e-16), 770, 1.0);s.store_sub_from_scalar(243, ((10.0 * 2.220446049250313e-16) + (10.0 * 2.220446049250313e-16)), 780);}
        if ((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && s.b[2480]) {
        }
        if ((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && (!s.b[2480])) {s.store_offset(243, 125, 1.0);s.store_scalar(334, 1.0);}
        if (((!s.b[1443]) && s.b[2430]) && s.b[2461]) {s.store_div_scaled_product_indices(335, 127, 242, 0.6666666666666667, 243, 1.0);s.store_mul(133, 335, 185);s.store_offset(244, 125, 0.5);s.store_mul(245, 243, 242);s.store_div_scaled_inputs_indices(246, 244, 0.4, 245, 1.0);s.store_sub_from_scalar(247, 0.6, 246);}
        s.b[2486] = (s.v[247] > 0.5);s.store_scalar(2486, if s.b[2486] { 1.0 } else { 0.0 });
        if ((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && s.b[2486]) {s.store_scalar(247, 0.5);}
        s.b[2487] = (s.v[347] == 2.0);s.store_scalar(2487, if s.b[2487] { 1.0 } else { 0.0 });
        if ((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && s.b[2487]) {s.copy_ad(335, 131);s.store_add_scaled_product_mixed_aii(131, A::mul_sub_from_scalar_lhs(1.0, s.ad_value(207), s.ad_value(239)), 1.0, 207, 131, 1.0);}
        s.b[2488] = (s.v[131] < 0.0);s.store_scalar(2488, if s.b[2488] { 1.0 } else { 0.0 });
        if (((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && s.b[2487]) && s.b[2488]) {s.store_scalar(131, 0.0);}
        if ((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && s.b[2487]) {s.copy_ad(335, 133);s.store_add_scaled_product_mixed_aii(133, A::mul_sub_from_scalar_lhs(1.0, s.ad_value(207), s.ad_value(238)), 1.0, 207, 133, 1.0);}
        s.b[2489] = (s.v[133] < 0.0);s.store_scalar(2489, if s.b[2489] { 1.0 } else { 0.0 });
        if (((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && s.b[2487]) && s.b[2489]) {s.store_scalar(133, 0.0);}
        if ((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && s.b[2487]) {s.copy_ad(335, 247);s.store_add_scaled_product_mixed_aii(247, A::scale_offset(s.ad_value(207), (-0.5), 0.5), 1.0, 207, 247, 1.0);s.copy_ad(335, 169);s.store_mul(169, 207, 169);}
        if (((!s.b[1443]) && s.b[2430]) && (s.v[948] != 0.0)) {s.store_scalar(948, 0.0);}
        if ((!s.b[1443]) && s.b[2430]) {s.store_sub(170, 162, 169);}
        s.b[2490] = (s.v[170] < 1e-9);s.store_scalar(2490, if s.b[2490] { 1.0 } else { 0.0 });
        if (((!s.b[1443]) && s.b[2430]) && s.b[2490]) {s.store_scalar(170, 1e-9);}
        if ((!s.b[1443]) && s.b[2430]) {s.store_scalar(335, (s.v[625] / 100.0));s.store_scalar(336, (s.v[626] / 100.0));s.copy_ad(334, 682);s.store_offset_mul_ad(338, A::sub(s.ad_value(91), s.ad_value(87)), s.ad_value(334), 1.0);s.store_add_scaled_products_indices(339, 335, 131, 1.0, 336, 133, 1.0);s.store_div(337, 339, 338);s.store_mul_scale_offset_rhs(251, 337, 1438, p[166], 1.0);}
        if ((!s.b[1443]) && s.b[2430]) {
            if (s.v[251] == 0.0) {
                s.store_scalar(339, 0.0);
            } else {
                s.store_powf(339, 251, (p[160] - 1.0));
            }
        }
        if ((!s.b[1443]) && s.b[2430]) {s.store_mul(342, 339, 251);}
        if ((!s.b[1443]) && s.b[2430]) {
            if (s.v[251] == 0.0) {
                s.store_scalar(341, 0.0);
            } else {
                s.store_pow_offset_rhs(341, 251, 624, (-1.0));
            }
        }
        if ((!s.b[1443]) && s.b[2430]) {s.store_mul(340, 341, 251);s.store_scalar(343, (1.6021918e-19 * 10000.0));s.store_div(252, 133, 343);s.store_add_scaled_inputs_mixed_ai(335, A::add_scaled_product(A::div_from_scalar(1.0, A::scale_offset(s.ad_value(252), (s.v[475] * 1e-11), s.v[474])), 1.0, s.ad_value(679), s.ad_value(342), 1.0), 1.0, 340, 1.0 / (s.v[479]));s.store_div_from_scalar(254, 1.0, 335);s.store_scale(254, 254, 0.0001);s.store_mul_ad_product_lhs_mixed_ia(336, 154, A::offset(s.ad_value(238), 1e-25), 170);s.store_div_from_scalar(335, 1.0, 336);s.store_square(337, 335);s.store_mul_scale_offset_indices(338, 337, 154, -1.0, 0.0);s.store_mul(339, 338, 170);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_118(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((!s.b[1443]) && s.b[2430]) {s.store_mul_scale_offset_indices(340, 338, 238, 1.0, 1e-25);s.store_mul_ad_product_lhs_mixed_ai(333, A::offset(s.ad_value(94), (10.0 * 2.220446049250313e-16)), 250, 335);s.store_div_scaled_inputs_indices(336, 257, 0.2, 254, 1.0);s.store_div_scaled_inputs_indices(337, 336, -1.0, 254, 1.0);s.store_sqrt_square_sum(255, 333, 336);s.store_div_from_scalar(338, 1.0, 255);s.store_mul(256, 254, 255);s.store_div(335, 256, 257);}
        s.b[2491] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= p[178]) && (p[178] <= (1.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(2491, if s.b[2491] { 1.0 } else { 0.0 });
        if (((!s.b[1443]) && s.b[2430]) && s.b[2491]) {s.store_scalar(337, 1.0);}
        s.b[2492] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= p[178]) && (p[178] <= (2.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(2492, if s.b[2492] { 1.0 } else { 0.0 });
        if ((((!s.b[1443]) && s.b[2430]) && (!s.b[2491])) && s.b[2492]) {s.copy_ad(337, 335);}
        if ((((!s.b[1443]) && s.b[2430]) && (!s.b[2491])) && (!s.b[2492])) {
            if (s.v[335] == 0.0) {
                s.store_scalar(337, 0.0);
            } else {
                s.store_powf(337, 335, (p[178] - 1.0));
            }
        }
        if ((!s.b[1443]) && s.b[2430]) {s.store_mul(336, 335, 337);s.store_offset(338, 336, 1.0);}
        s.b[2493] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= p[178]) && (p[178] <= (1.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(2493, if s.b[2493] { 1.0 } else { 0.0 });
        if (((!s.b[1443]) && s.b[2430]) && s.b[2493]) {s.store_div_from_scalar(339, 1.0, 338);}
        s.b[2494] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= p[178]) && (p[178] <= (2.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(2494, if s.b[2494] { 1.0 } else { 0.0 });
        if ((((!s.b[1443]) && s.b[2430]) && (!s.b[2493])) && s.b[2494]) {s.store_div_from_scalar_sqrt_ad(339, 1.0, s.ad_value(338));}
        if ((((!s.b[1443]) && s.b[2430]) && (!s.b[2493])) && (!s.b[2494])) {
            if (s.v[338] == 0.0) {
                s.store_scalar(340, 0.0);
            } else {
                s.store_powf(340, 338, (((-1.0) / p[178]) - 1.0));
            }
        }
        if ((((!s.b[1443]) && s.b[2430]) && (!s.b[2493])) && (!s.b[2494])) {s.store_mul(339, 338, 340);}
        if ((!s.b[1443]) && s.b[2430]) {s.store_mul(253, 254, 339);s.store_div_scaled_inputs_indices(115, 155, s.v[632], 170, 1.0);s.store_div_scaled_inputs_indices(335, 115, -1.0, 170, 1.0);s.store_mul3_lhs(135, 115, 248, 253);}
        s.b[2495] = (p[283] != 0.0);s.store_scalar(2495, if s.b[2495] { 1.0 } else { 0.0 });
        if (((!s.b[1443]) && s.b[2430]) && s.b[2495]) {s.store_scaled_sub(335, 790, 94, 0.5);s.store_scale(781, 335, (2.0 * 100.0));s.store_offset_mul_offset_rhs_mixed_ia(782, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 5040.0), (1.0 / 720.0))), (1.0 / 120.0)), (1.0 / 24.0)), (1.0 / 6.0)), (1.0 / 2.0), 1.0);s.store_offset_mul_offset_rhs_mixed_ia(783, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 840.0), (1.0 / 144.0))), (1.0 / 30.0)), (1.0 / 8.0)), (1.0 / 3.0), (1.0 / 2.0));s.store_div_from_scalar(340, 0.01, 782);s.store_div_scaled_inputs_square_rhs(336, 783, (-2.0), 782, 1.0);s.store_scale(336, 336, 0.5);s.store_sub_from_scalar_ad(335, 1.1, A::add(s.ad_value(87), s.ad_value(340)));s.store_sqrt_square_offset(782, 335, ((4.0 * 0.05) * 0.05));s.store_offset_scaled_div(334, 335, 782, 0.5, 0.5);s.store_scaled_add(336, 335, 782, 0.5);}
        s.b[2496] = (s.v[336] < 0.0);s.store_scalar(2496, if s.b[2496] { 1.0 } else { 0.0 });
        if ((((!s.b[1443]) && s.b[2430]) && s.b[2495]) && s.b[2496]) {s.store_scalar(336, 0.0);s.store_scalar(334, 0.0);}
        if (((!s.b[1443]) && s.b[2430]) && s.b[2495]) {s.store_offset(336, 336, 1e-25);s.store_scale(334, 154, s.v[672]);s.store_mul(337, 185, 334);s.store_powf(334, 336, p[284]);s.store_mul(343, 337, 334);s.store_offset_scaled(338, 1439, p[285], 1.0);s.store_scalar(334, s.v[673]);s.store_add_scaled_inputs3_indices(339, 87, 1.0, 340, 1.0, 1438, -1.0);s.store_add_product3_rhs_indices(338, 338, 1439, 334, 339, 1.0);s.store_mul(340, 343, 338);s.copy_ad(343, 340);}
        if (((!s.b[1443]) && s.b[2430]) && (!s.b[2495])) {s.store_scalar(343, 0.0);}
        s.b[2497] = (p[287] != 0.0);s.store_scalar(2497, if s.b[2497] { 1.0 } else { 0.0 });
        if (((!s.b[1443]) && s.b[2430]) && s.b[2497]) {s.store_scale(335, 154, s.v[674]);s.store_mul(336, 185, 335);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_119(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((!s.b[1443]) && s.b[2430]) && s.b[2497]) {s.store_mul(342, 336, 1439);}
        if (((!s.b[1443]) && s.b[2430]) && (!s.b[2497])) {s.store_scalar(342, 0.0);}
        s.b[2498] = ((s.v[343] + s.v[342]) > 0.0);s.store_scalar(2498, if s.b[2498] { 1.0 } else { 0.0 });
        if (((!s.b[1443]) && s.b[2430]) && s.b[2498]) {s.store_mul_add_rhs(249, 94, 343, 342);s.store_mul3_lhs(45, 115, 249, 253);s.store_add(135, 135, 45);}
        if (((!s.b[1443]) && s.b[2430]) && (!s.b[2498])) {s.store_scalar(45, 0.0);}
        s.b[2499] = ((s.v[74] == 2.0) || (s.v[74] == 3.0));s.store_scalar(2499, if s.b[2499] { 1.0 } else { 0.0 });s.b[2500] = (p[296] > 0.0);s.store_scalar(2500, if s.b[2500] { 1.0 } else { 0.0 });
        if ((((!s.b[1443]) && s.b[2430]) && s.b[2499]) && s.b[2500]) {s.copy_ad(338, 647);s.store_scaled_offset(335, 796, (-p[300]), s.v[533]);s.store_offset_sub(781, 335, 338, (-(0.01 * 0.01)));s.store_scale(782, 338, (4.0 * (0.01 * 0.01)));}
        if ((((!s.b[1443]) && s.b[2430]) && s.b[2499]) && s.b[2500]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if ((((!s.b[1443]) && s.b[2430]) && s.b[2499]) && s.b[2500]) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_add_scaled_inputs3_indices(336, 338, 1.0, 781, 0.5, 782, 0.5);s.store_scale(337, 338, (p[296] + 1.0));s.store_offset_sub(781, 337, 336, (-(0.01 * 0.01)));s.store_scale(782, 337, (4.0 * (0.01 * 0.01)));}
        if ((((!s.b[1443]) && s.b[2430]) && s.b[2499]) && s.b[2500]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if ((((!s.b[1443]) && s.b[2430]) && s.b[2499]) && s.b[2500]) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_add_scaled_inputs3_indices(341, 337, 1.0, 781, (-0.5), 782, (-0.5));}
        if ((((!s.b[1443]) && s.b[2430]) && s.b[2499]) && (!s.b[2500])) {s.copy_ad(341, 647);}
        s.b[2501] = (s.v[793] >= 0.0);s.store_scalar(2501, if s.b[2501] { 1.0 } else { 0.0 });
        if ((((!s.b[1443]) && s.b[2430]) && s.b[2499]) && s.b[2501]) {s.copy_ad(369, 793);}
        if ((((!s.b[1443]) && s.b[2430]) && s.b[2499]) && (!s.b[2501])) {s.store_scalar(369, 0.0);}
        s.b[2502] = (s.v[369] < (20.0 * 1e-12));s.store_scalar(2502, if s.b[2502] { 1.0 } else { 0.0 });
        if ((((!s.b[1443]) && s.b[2430]) && s.b[2499]) && s.b[2502]) {s.store_scalar(378, (((((20.0 + 1.0)) as f64).powf((p[297] - 1.0)) * ((20.0 + 1.0) - ((0.5 * p[297]) * 20.0))) * ((1e-12) as f64).powf(p[297])));s.store_scalar(379, ((((0.5 * p[297]) * (((20.0 + 1.0)) as f64).powf((p[297] - 1.0))) / 20.0) * ((1e-12) as f64).powf((p[297] - 2.0))));s.store_add_product3_rhs_indices(335, 378, 379, 369, 369, 1.0);}
        if ((((!s.b[1443]) && s.b[2430]) && s.b[2499]) && (!s.b[2502])) {s.store_powf_offset_input(335, 369, 1e-12, p[297]);}
        if (((!s.b[1443]) && s.b[2430]) && s.b[2499]) {s.store_powf_offset_input(343, 369, 1e-12, p[299]);s.store_add_scaled_products_indices(368, 341, 335, 1.0 / (s.v[632]), 797, 343, (s.v[531] * 1.0 / (s.v[632])));s.store_mul(334, 368, 135);s.store_offset(335, 790, 1e-12);s.store_div_from_scalar(336, 1.0, 335);s.store_offset_mul(337, 334, 336, 1.0);s.store_div_from_scalar(338, 1.0, 337);s.store_mul(134, 135, 338);}
        if (((!s.b[1443]) && s.b[2430]) && (!s.b[2499])) {s.copy_ad(134, 135);s.store_scalar(368, 0.0);}
        s.b[2503] = (p[27] != 0.0);s.store_scalar(2503, if s.b[2503] { 1.0 } else { 0.0 });
        if (((!s.b[1443]) && s.b[2430]) && s.b[2503]) {s.store_scale(335, 186, 1.034943e-10);s.copy_ad(336, 684);s.store_scalar(337, (s.v[628] - p[139]));s.store_div_from_scalar_square_ad(338, 1.0, s.ad_value(337));s.store_mul_ad_product_lhs_mixed_ai(339, A::mul_sub_from_scalar_lhs_scaled_output(p[137], s.ad_value(123), s.ad_value(335), 2.0), 336, 338);s.store_mul(121, 339, 181);s.store_div_scaled_inputs_indices(340, 339, 0.5, 181, 1.0);s.store_mul_ad_product_lhs_mixed_ai(341, A::mul_sub_from_scalar_lhs_scaled_output(p[137], s.ad_value(123), s.ad_value(336), (2.0 * 1.034943e-10)), 338, 181);s.store_mul_product3_indices(342, 181, 335, 336, 338, (-2.0));s.store_scalar(338, s.v[496]);s.store_scalar(340, s.v[497]);s.store_add_scaled_product_indices(335, 338, 1.0, 340, 1439, 1.0);s.store_mul(137, 121, 335);s.store_sub_from_scalar_scaled_input(335, s.v[498], 790, p[213]);s.store_add_scaled_inputs3_offset_indices(138, 1440, 1.0, 335, 1.0, 137, 1.0, (-s.v[160]));s.store_mul3_lhs(141, 694, 186, 186);s.store_scaled_mul(142, 141, 154, 0.5);s.store_scaled_mul(143, 142, 154, 2.0);s.store_scale(345, 154, 0.25);s.store_offset_sub_ad(344, A::offset(A::add_scaled_product(s.ad_value(155), 1.0, s.ad_value(141), s.ad_value(345), (-1.0)), ((s.v[160]) + ((-s.v[498])))), s.ad_value(137), 1e-25);s.store_offset_sub(335, 1440, 344, (-0.005));}
        if (((!s.b[1443]) && s.b[2430]) && s.b[2503]) {s.store_scalar(334, (if (s.v[344] >= 0.0) { 1.0 } else { (-1.0) }));}
        if (((!s.b[1443]) && s.b[2430]) && s.b[2503]) {s.store_sqrt_add_scaled_square_product(336, 335, 1.0, 334, 344, (4.0 * 0.005));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_120(
        s: &mut ReactiveScratch,
    ) {
        if (((!s.b[1443]) && s.b[2430]) && s.b[2503]) {s.store_sub_mixed_ai(337, A::add_scaled_inputs4_offset(s.ad_value(344), 1.0, s.ad_value(335), 0.5, s.ad_value(336), 0.5, s.ad_value(137), 1.0, (((-s.v[160])) + (s.v[498]))), 1438);s.store_offset_mul(338, 154, 337, (-1.0));s.store_div_from_scalar(339, 4.0, 143);s.store_offset_mul(335, 338, 339, 1.0);s.store_mul(340, 154, 339);s.store_mul(341, 338, 339);s.store_sqrt_square_offset(782, 335, ((4.0 * 0.01) * 0.01));s.store_offset_scaled_div(336, 335, 782, 0.5, 0.5);s.store_scaled_add(335, 335, 782, 0.5);}
        s.b[2504] = (s.v[335] < 0.0);s.store_scalar(2504, if s.b[2504] { 1.0 } else { 0.0 });
        if ((((!s.b[1443]) && s.b[2430]) && s.b[2503]) && s.b[2504]) {s.store_scalar(335, 0.0);s.store_scalar(336, 0.0);}
        if (((!s.b[1443]) && s.b[2430]) && s.b[2503]) {s.store_offset(335, 335, 1e-25);s.store_sqrt(144, 335);s.store_mul_scale_offset_indices(334, 142, 144, -1.0, 1.0);s.store_add(146, 138, 334);s.store_div_from_scalar_add_ad(334, 1.0, s.ad_value(154), A::div_scalar_offset_denominator(2.0, s.ad_value(138), 1e-25, 1.0));s.store_mul_ln_mixed_ia(147, 334, A::mul(A::div_scalar_by_product(1.0, s.ad_value(140), s.ad_value(141), 1.0), A::square(s.ad_value(138))));s.store_offset_sub(148, 147, 146, (-0.002));s.store_sqrt_add_scaled_square_input(334, 148, 1.0, 147, (4.0 * 0.002));s.store_add_scaled_inputs3_indices(149, 147, 1.0, 148, (-0.5), 334, (-0.5));s.store_mul_exp_mixed_ia(334, 140, A::mul(s.ad_value(154), s.ad_value(149)));s.store_add_offset_lhs_mixed_ai(335, A::mul(s.ad_value(154), A::sub(s.ad_value(149), s.ad_value(1438))), (-1.0), 334);s.store_sqrt_square_offset(782, 335, ((4.0 * 0.01) * 0.01));s.store_offset_scaled_div(334, 335, 782, 0.5, 0.5);s.store_scaled_add(335, 335, 782, 0.5);}
        s.b[2505] = (s.v[335] < 0.0);s.store_scalar(2505, if s.b[2505] { 1.0 } else { 0.0 });
        if ((((!s.b[1443]) && s.b[2430]) && s.b[2503]) && s.b[2505]) {s.store_scalar(335, 0.0);s.store_scalar(334, 0.0);}
        if (((!s.b[1443]) && s.b[2430]) && s.b[2503]) {s.store_offset(335, 335, 1e-25);s.store_sqrt(150, 335);s.store_offset_mul_ad(335, s.ad_value(154), A::sub(s.ad_value(149), s.ad_value(1438)), (-1.0));s.store_sqrt_square_offset(782, 335, ((4.0 * 0.01) * 0.01));s.store_offset_scaled_div(334, 335, 782, 0.5, 0.5);s.store_scaled_add(335, 335, 782, 0.5);}
        s.b[2506] = (s.v[335] < 0.0);s.store_scalar(2506, if s.b[2506] { 1.0 } else { 0.0 });
        if ((((!s.b[1443]) && s.b[2430]) && s.b[2503]) && s.b[2506]) {s.store_scalar(335, 0.0);s.store_scalar(334, 0.0);}
        if (((!s.b[1443]) && s.b[2430]) && s.b[2503]) {s.store_offset(335, 335, 1e-25);s.store_sqrt(151, 335);s.store_div_from_scalar(336, 0.5, 151);s.store_mul_sub_rhs(152, 139, 150, 151);s.store_sub(335, 146, 149);s.store_sqrt_square_offset(782, 335, ((4.0 * 0.1) * 0.1));s.store_offset_scaled_div(336, 335, 782, 0.5, 0.5);s.store_scaled_add(335, 335, 782, 0.5);}
        s.b[2507] = (s.v[335] < 0.0);s.store_scalar(2507, if s.b[2507] { 1.0 } else { 0.0 });
        if ((((!s.b[1443]) && s.b[2430]) && s.b[2503]) && s.b[2507]) {s.store_scalar(335, 0.0);s.store_scalar(336, 0.0);}
        if (((!s.b[1443]) && s.b[2430]) && s.b[2503]) {s.store_offset(335, 335, 1e-25);s.store_div(332, 790, 335);s.store_div_from_scalar_square_ad(336, 1.0, s.ad_value(335));s.store_square(722, 332);s.store_scalar(723, 1.0);s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_121(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((!s.b[1443]) && s.b[2430]) && s.b[2503]) {s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2508] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));s.store_scalar(2508, if s.b[2508] { 1.0 } else { 0.0 });s.b[2509] = (4.0 == 1.0);s.store_scalar(2509, if s.b[2509] { 1.0 } else { 0.0 });
        if (((((!s.b[1443]) && s.b[2430]) && s.b[2503]) && s.b[2508]) && s.b[2509]) {s.store_scalar(720, 1.0);}
        s.b[2510] = (4.0 == 2.0);s.store_scalar(2510, if s.b[2510] { 1.0 } else { 0.0 });
        if ((((((!s.b[1443]) && s.b[2430]) && s.b[2503]) && s.b[2508]) && (!s.b[2509])) && s.b[2510]) {s.store_scalar(720, 2.0);}
        s.b[2511] = (4.0 == 4.0);s.store_scalar(2511, if s.b[2511] { 1.0 } else { 0.0 });
        if (((((((!s.b[1443]) && s.b[2430]) && s.b[2503]) && s.b[2508]) && (!s.b[2509])) && (!s.b[2510])) && s.b[2511]) {s.store_scalar(720, 3.0);}
        s.b[2512] = (4.0 == 8.0);s.store_scalar(2512, if s.b[2512] { 1.0 } else { 0.0 });
        if ((((((((!s.b[1443]) && s.b[2430]) && s.b[2503]) && s.b[2508]) && (!s.b[2509])) && (!s.b[2510])) && (!s.b[2511])) && s.b[2512]) {s.store_scalar(720, 4.0);}
        if ((((!s.b[1443]) && s.b[2430]) && s.b[2503]) && s.b[2508]) {s.store_scalar(719, 0.0);}
        let mut t7: usize = 0;
        while {
            let t6: f64 = if (((((!s.b[1443]) && s.b[2430]) && s.b[2503]) && s.b[2508]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t6 != 0.0
        } {
            t7 += 1;
            if t7 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t7, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((((!s.b[1443]) && s.b[2430]) && s.b[2503]) && s.b[2508]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
        }
        if ((((!s.b[1443]) && s.b[2430]) && s.b[2503]) && (!s.b[2508])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 4.0)));
            }
        }
        if (((!s.b[1443]) && s.b[2430]) && s.b[2503]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(333, 332, 726, 1.0);s.store_div_scaled_product_indices(336, 725, 726, 1.0, 770, 1.0);s.store_scale(145, 155, ((2.0 * s.v[495]) * p[7]));s.copy_ad(335, 170);s.store_div_scaled_product_mixed_aii(153, A::mul3(s.ad_value(145), s.ad_value(253), s.ad_value(152)), 333, 1.0, 335, 1.0);s.store_add(134, 134, 153);}
        s.b[2513] = (((p[31] != 0.0) && (p[30] != 0.0)) && (s.v[963] == 0.0));s.store_scalar(2513, if s.b[2513] { 1.0 } else { 0.0 });
        if (((!s.b[1443]) && s.b[2430]) && s.b[2513]) {s.store_square(317, 127);s.store_mul3_affine_lhs(318, 155, 186, 2.0, 0.0, 248);s.store_sub(319, 317, 318);s.store_sqrt_square_offset(782, 317, ((4.0 * 0.001) * 0.001));s.store_offset_scaled_div(334, 317, 782, 0.5, 0.5);s.store_scaled_add(317, 317, 782, 0.5);}
        s.b[2514] = (s.v[317] < 0.0);s.store_scalar(2514, if s.b[2514] { 1.0 } else { 0.0 });
        if ((((!s.b[1443]) && s.b[2430]) && s.b[2513]) && s.b[2514]) {s.store_scalar(317, 0.0);s.store_scalar(334, 0.0);}
        if (((!s.b[1443]) && s.b[2430]) && s.b[2513]) {s.store_sqrt_square_offset(782, 319, ((4.0 * 0.001) * 0.001));s.store_offset_scaled_div(334, 319, 782, 0.5, 0.5);s.store_scaled_add(319, 319, 782, 0.5);}
        s.b[2515] = (s.v[319] < 0.0);s.store_scalar(2515, if s.b[2515] { 1.0 } else { 0.0 });
        if ((((!s.b[1443]) && s.b[2430]) && s.b[2513]) && s.b[2515]) {s.store_scalar(319, 0.0);s.store_scalar(334, 0.0);}
        if (((!s.b[1443]) && s.b[2430]) && s.b[2513]) {s.store_sub(320, 317, 319);}
        s.b[2516] = ((s.v[238] < (10.0 * 2.220446049250313e-16)) || (s.v[320] < (10.0 * 2.220446049250313e-16)));s.store_scalar(2516, if s.b[2516] { 1.0 } else { 0.0 });
        if ((((!s.b[1443]) && s.b[2430]) && s.b[2513]) && s.b[2516]) {s.store_scalar(321, 0.0);}
        if ((((!s.b[1443]) && s.b[2430]) && s.b[2513]) && (!s.b[2516])) {s.store_scalar(321, 1.0);}
        if ((!s.b[1443]) && (s.v[946] != 0.0)) {s.store_scalar(946, 0.0);}
        s.b[2517] = ((s.v[78] == 0.0) && (s.v[127] > 1e-12));s.store_scalar(2517, if s.b[2517] { 1.0 } else { 0.0 });
        if ((!s.b[1443]) && s.b[2517]) {s.store_div_scaled_product_indices(130, 212, 154, 1.0, 100, 2.0);s.store_add_mixed_ai(128, A::div_scaled_value_offset_denominator(s.ad_value(127), 1.0, s.ad_value(130), 1.0, 1.0), 87);}
        if ((!s.b[1443]) && (!s.b[2517])) {s.store_scalar(128, 0.0);}
        if (!s.b[1443]) {s.copy_ad(136, 134);s.store_scalar(46, 0.0);}
        s.b[2519] = ((p[450] > 0.0) && (p[454] > 0.0));s.store_scalar(2519, if s.b[2519] { 1.0 } else { 0.0 });
        if ((!s.b[1443]) && s.b[2519]) {s.store_scalar(2524, 1e-5);s.store_offset_add_scaled_inputs3_offset_indices(2525, 791, 1.0, 120, 1.0, 182, -1.0, (-s.v[160]), (-p[455]));s.store_offset(2526, 118, p[455]);s.store_sqrt_offset_ad(781, A::square(A::sub(s.ad_value(960), s.ad_value(1435))), ((4.0 * 0.01) * 0.01));s.store_add_scaled_inputs3_indices(2536, 960, 0.5, 1435, ((-1.0) * 0.5), 781, 0.5);s.store_sqrt_ad(2520, A::div_scaled_product_offset_denominator(s.ad_value(2536), s.ad_value(586), (((2.0 * 1.6021918e-19) * 1.0 / (1.034943e-10)) * s.v[489]), s.ad_value(586), s.v[489], 1.0));s.store_mul(2522, 2520, 162);s.store_div_scaled_product_add_scaled_denominator_indices(993, 2522, 2522, (-0.25), 790, 1.0, 2522, 1.0, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_122(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[2538] = (p[457] > 0.0);s.store_scalar(2538, if s.b[2538] { 1.0 } else { 0.0 });
        if (((!s.b[1443]) && s.b[2519]) && s.b[2538]) {s.store_scalar(2523, p[457]);}
        if (((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) {s.copy_ad(2539, 993);s.copy_ad(2540, 2526);s.store_offset_div_scaled_offset_numerator(332, A::mul(s.ad_value(154), A::sub(s.ad_value(2525), s.ad_value(2539))), 4.0, ((-1.0) * 4.0), A::mul(s.ad_value(213), s.ad_value(156)), 1.0, 1.0);}
        if (((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) {
            if (s.v[332] >= (10.0 * 2.220446049250313e-16)) {
            } else {
                s.store_scalar(332, (10.0 * 2.220446049250313e-16));
            }
        }
        if (((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) {s.store_add_product3_rhs_mixed_iia(89, 2525, 213, 154, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(332))), 0.5);s.store_mul_sub_rhs(116, 154, 89, 2539);}
        s.b[2541] = (s.v[116] < 3.0);s.store_scalar(2541, if s.b[2541] { 1.0 } else { 0.0 });
        if ((((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) && s.b[2541]) {s.store_mul_sub_rhs(333, 154, 2525, 2539);s.store_div_scalar_by_product_indices(335, 1.0, 154, 212, (1.414213562373095 / 108.0));s.store_offset_scaled(336, 335, 3.0, 81.0);s.store_add_scaled_sub_value_product_mixed_aii(337, (-2916.0), A::scale(s.ad_value(335), 81.0), 1.0, 335, 333, 27.0);s.store_add_scaled_sub_value_product_mixed_aii(338, 1458.0, A::scaled_offset(s.ad_value(335), 54.0, 81.0), 1.0, 335, 333, 27.0);s.store_square(338, 338);}
        if ((((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) && s.b[2541]) {
            if ((s.v[337] + ((((((4.0 * s.v[336]) * s.v[336]) * s.v[336]) + s.v[338])) as f64).sqrt()) == 0.0) {
                s.store_scalar(339, 0.0);
            } else {
                s.store_powf_ad(339, A::add(s.ad_value(337), A::sqrt(A::add(A::mul3_scaled_output(s.ad_value(336), s.ad_value(336), s.ad_value(336), 4.0), s.ad_value(338)))), 0.3333333333333333);
            }
        }
        if ((((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) && s.b[2541]) {s.store_add_scaled_inputs_mixed_ai(332, A::sub_from_scalar(3.0, A::div_scaled_inputs(s.ad_value(336), 1.259921049894873, s.ad_value(339), 3.0)), 1.0, 339, (1.0 / (3.0 * 1.259921049894873)));s.store_add_scaled_product_indices(89, 2539, 1.0, 332, 155, 1.0);s.copy_ad(88, 89);}
        s.b[2542] = (s.v[791] <= s.v[2540]);s.store_scalar(2542, if s.b[2542] { 1.0 } else { 0.0 });
        if (((((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) && (!s.b[2541])) && s.b[2542]) {s.copy_ad(88, 89);}
        if (((((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) && (!s.b[2541])) && (!s.b[2542])) {s.store_div_scalar_by_product_indices(335, 1.0, 210, 211, 1.0);s.store_mul3_lhs(336, 335, 2525, 2525);s.store_add_div_from_scalar_rhs(337, 154, 2.0, 2525);s.store_offset_div_ad(90, A::ln(s.ad_value(336)), s.ad_value(337), p[456]);s.store_offset_sub(781, 90, 89, (-0.0008));s.store_scale(782, 90, (4.0 * 0.0008));}
        if (((((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) && (!s.b[2541])) && (!s.b[2542])) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if (((((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) && (!s.b[2541])) && (!s.b[2542])) {s.store_sqrt_square_add(782, 781, 782);s.store_add_scaled_inputs3_indices(88, 90, 1.0, 781, (-0.5), 782, (-0.5));}
        if (((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) {s.store_offset(332, 2539, (1e-12 / 2.0));}
        s.b[2543] = (s.v[88] < s.v[332]);s.store_scalar(2543, if s.b[2543] { 1.0 } else { 0.0 });
        if ((((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) && s.b[2543]) {s.copy_ad(88, 332);}
        if (((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) {s.copy_ad(2523, 88);}
        s.b[2544] = (p[451] == 1.0);s.store_scalar(2544, if s.b[2544] { 1.0 } else { 0.0 });
        if ((((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) && s.b[2544]) {s.copy_ad(88, 2523);s.copy_ad(2545, 993);s.store_offset_add_scaled_inputs3_offset_indices(86, 120, (-1.0), 182, 1.0, 2545, 1.0, s.v[160], p[455]);}
        s.b[2554] = (s.v[791] < s.v[86]);s.store_scalar(2554, if s.b[2554] { 1.0 } else { 0.0 });
        if (((((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) && s.b[2544]) && s.b[2554]) {s.store_scalar(347, (-1.0));s.store_mul_scaled_ln_ad_rhs(271, 155, 2.0, A::div_from_scalar((-s.v[270]), s.ad_value(212)));s.store_mul_sub_rhs(332, 154, 2525, 2545);s.store_div_scalar_by_product_indices(335, 1.0, 154, 209, 1.0);s.store_mul(333, 335, 185);s.store_offset_scaled(279, 333, (3.0 * 1.414213562373095), 2.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_123(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) && s.b[2544]) && s.b[2554]) {s.store_mul3_affine_lhs(277, 279, 279, 8.0, 0.0, 279);s.store_offset(338, 332, (-2.0));s.store_scaled_mul(339, 333, 338, 9.0);s.store_sub_from_scalar(278, (7.0 * 1.414213562373095), 339);s.store_square(276, 278);}
        s.b[2555] = (s.v[277] < (s.v[276] * 1e-8));s.store_scalar(2555, if s.b[2555] { 1.0 } else { 0.0 });
        if ((((((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) && s.b[2544]) && s.b[2554]) && s.b[2555]) {s.store_add_scaled_inputs3_offset_mixed_iai(274, 278, 1.0, A::div_scaled_inputs(s.ad_value(277), 0.5, s.ad_value(278), 1.0), 1.0, 339, 1.0, ((-7.0) * 1.414213562373095));}
        if ((((((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) && s.b[2544]) && s.b[2554]) && (!s.b[2555])) {s.store_sqrt_add(275, 277, 276);s.store_add_offset_lhs(274, 275, ((-7.0) * 1.414213562373095), 339);}
        if (((((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) && s.b[2544]) && s.b[2554]) {
            if (s.v[274] == 0.0) {
                s.store_scalar(273, 0.0);
            } else {
                s.store_powf(273, 274, 0.3333333333333333);
            }
        }
        if (((((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) && s.b[2544]) && s.b[2554]) {s.store_add_scaled_inputs_product_mixed_aiii(272, A::sub_from_scalar(((-4.0) * 1.414213562373095), A::scale(s.ad_value(333), 12.0)), 1.0, 273, 2.0, 273, 273, 1.414213562373095);s.store_div_from_scalar(335, 1.0, 273);s.store_mul(116, 272, 335);s.store_add_scaled_product_indices(167, 2545, 1.0, 116, 155, 1.0);s.store_sub(335, 167, 2545);s.store_div(336, 335, 271);s.store_sqrt_square_offset(337, 336, 1.0);s.store_add_div_lhs_indices(2523, 335, 337, 2545);}
        if (((((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) && s.b[2544]) && (!s.b[2554])) {s.store_exp_ad(230, A::mul_offset_rhs(s.ad_value(154), s.ad_value(2545), (-p[456])));s.store_scalar(79, 0.0);s.copy_ad(2546, 88);s.store_mul3_affine_lhs(2547, 166, 2524, (0.5 * 9662367879.197212), 0.0, 2524);s.store_sqrt_mul_scaled_lhs(334, 154, 2.0, 2547);s.store_scaled_add_ad(335, A::limited_exp(s.ad_value(334)), A::limited_exp_scaled_input(s.ad_value(334), -1.0), 0.5);s.store_div_ln_lhs(2548, 335, 2547);s.store_scalar(97, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_124(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        let mut ta: usize = 0;
        while {
            let t8: f64 = (s.v[421] + 1.0);let t9: f64 = if ((((((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) && s.b[2544]) && (!s.b[2554])) && (s.v[97] <= t8)) { 1.0 } else { 0.0 };
            t9 != 0.0
        } {
            ta += 1;
            if ta > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", ta, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if (((((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) && s.b[2544]) && (!s.b[2554])) {s.store_sub(2549, 2546, 2545);s.store_mul(116, 154, 2549);s.store_mul_sub_rhs(333, 2548, 2549, 2547);}
            s.b[2556] = (s.v[333] < 60.0);s.store_scalar(2556, if s.b[2556] { 1.0 } else { 0.0 });
            if ((((((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) && s.b[2544]) && (!s.b[2554])) && s.b[2556]) {s.store_exp(335, 333);s.store_exp_mul_scaled_lhs_indices(334, 2548, -1.0, 2547);s.store_sub(336, 335, 334);s.store_div_ln_offset_lhs(2551, 336, 1.0, 2548);s.store_div_scaled_value_offset_denominator(2552, s.ad_value(335), 1.0, s.ad_value(336), 1.0, 1.0);}
            if ((((((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) && s.b[2544]) && (!s.b[2554])) && (!s.b[2556])) {s.store_sub(2551, 2549, 2547);s.store_scalar(2552, 1.0);}
            if (((((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) && s.b[2544]) && (!s.b[2554])) {s.store_mul(2550, 154, 2551);}
            s.b[2557] = (((s.v[116]) as f64).abs() < 1e-16);s.store_scalar(2557, if s.b[2557] { 1.0 } else { 0.0 });
            if ((((((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) && s.b[2544]) && (!s.b[2554])) && s.b[2557]) {s.store_sqrt_scaled_input_ad(334, A::sub_from_scalar(1.0, A::square(s.ad_value(2552))), 1.0 / (2.0));s.store_mul(223, 116, 334);s.store_mul(2553, 154, 334);}
            s.b[2558] = (s.v[116] < 0.0);s.store_scalar(2558, if s.b[2558] { 1.0 } else { 0.0 });
            if (((((((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) && s.b[2544]) && (!s.b[2554])) && s.b[2557]) && s.b[2558]) {s.store_neg(223, 223);s.store_neg(2553, 2553);}
            s.b[2559] = (((s.v[116]) as f64).abs() < 0.005);s.store_scalar(2559, if s.b[2559] { 1.0 } else { 0.0 });
            if (((((((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) && s.b[2544]) && (!s.b[2554])) && (!s.b[2557])) && s.b[2559]) {s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(334, 116, 1.0, 116, 1.0, 116, 1.0, 116, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));s.store_mul_scale_offset_mixed_ia(335, 116, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(116), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(116), 1.0, A::scale(s.ad_value(116), 0.25), 1.0 / (3.0)), 1.0 / (2.0)), -1.0, 1.0);s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(336, 2550, 1.0, 2550, 1.0, 2550, 1.0, 2550, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));s.store_mul_scale_offset_mixed_ia(337, 2550, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2550), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2550), 1.0, A::scale(s.ad_value(2550), 0.25), 1.0 / (3.0)), 1.0 / (2.0)), -1.0, 1.0);s.store_sqrt_sub(223, 334, 336);s.store_div_scaled_product_mixed_iai(2553, 154, A::add_scaled_product(s.ad_value(335), 1.0, s.ad_value(2552), s.ad_value(337), (-1.0)), 0.5, 223, 1.0);}
            if (((((((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) && s.b[2544]) && (!s.b[2554])) && (!s.b[2557])) && (!s.b[2559])) {s.store_exp_neg_input(334, 116);s.store_exp_neg_input(335, 2550);s.store_sqrt_ad(223, A::add_scaled_inputs4(s.ad_value(116), 1.0, s.ad_value(2550), (-1.0), s.ad_value(334), 1.0, s.ad_value(335), (-1.0)));s.store_div_scaled_product_mixed_iai(2553, 154, A::sub(A::sub_from_scalar(1.0, s.ad_value(334)), A::mul_sub_from_scalar_rhs(s.ad_value(2552), 1.0, s.ad_value(335))), 0.5, 223, 1.0);}
            s.b[2560] = ((s.v[79] == 1.0) && (s.v[116] < 0.0));s.store_scalar(2560, if s.b[2560] { 1.0 } else { 0.0 });
            if ((((((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) && s.b[2544]) && (!s.b[2554])) && s.b[2560]) {s.store_scalar(347, (-1.0));}
            s.b[2561] = (s.v[116] < 0.0);s.store_scalar(2561, if s.b[2561] { 1.0 } else { 0.0 });
            if ((((((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) && s.b[2544]) && (!s.b[2554])) && s.b[2561]) {s.store_neg(216, 223);s.store_neg(217, 2553);}
            s.b[2562] = (s.v[116] < 1e-7);s.store_scalar(2562, if s.b[2562] { 1.0 } else { 0.0 });
            if (((((((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) && s.b[2544]) && (!s.b[2554])) && (!s.b[2561])) && s.b[2562]) {s.copy_ad(216, 223);s.copy_ad(217, 2553);}
            if (((((((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) && s.b[2544]) && (!s.b[2554])) && (!s.b[2561])) && (!s.b[2562])) {s.store_mul_scale_offset_indices(117, 154, 2546, 1.0, (-p[456]));s.store_exp(228, 117);s.store_mul_mixed_ia(214, 210, A::add_scaled_offset_product_rhs(s.ad_value(228), 1.0, s.ad_value(230), s.ad_value(116), 1.0, (-1.0)));s.store_mul_ad_product_rhs_mixed_ia(215, 210, 154, A::sub(s.ad_value(228), s.ad_value(230)));s.store_sqrt_square_add(216, 223, 214);s.store_div_scaled_add_product_indices(217, 215, 0.5, 2553, 223, (2.0 * 0.5), 216, 1.0);}
            if (((((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) && s.b[2544]) && (!s.b[2554])) {s.store_add_scaled_inputs_product_indices(232, 2546, 1.0, 2525, (-1.0), 212, 216, 1.0);s.store_offset_mul(233, 212, 217, 1.0);}
            s.b[2563] = (s.v[79] == 1.0);s.store_scalar(2563, if s.b[2563] { 1.0 } else { 0.0 });
            if ((((((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) && s.b[2544]) && (!s.b[2554])) && s.b[2563]) {s.store_scalar(97, (s.v[421] + 1.0));}
            if ((((((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) && s.b[2544]) && (!s.b[2554])) && (!s.b[2563])) {s.store_div_scaled_inputs_indices(236, 232, -1.0, 233, 1.0);}
            if ((((((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) && s.b[2544]) && (!s.b[2554])) && (!s.b[2563])) {
                s.store_scaled_offset_ad(93, {
                    if (1.0 >= ((s.v[2546]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(2546))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[2564] = (((s.v[236]) as f64).abs() > s.v[93]);s.store_scalar(2564, if s.b[2564] { 1.0 } else { 0.0 });
            if (((((((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) && s.b[2544]) && (!s.b[2554])) && (!s.b[2563])) && s.b[2564]) {s.store_scale(236, 93, (if (s.v[236] >= 0.0) { 1.0 } else { (-1.0) }));}
            if ((((((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) && s.b[2544]) && (!s.b[2554])) && (!s.b[2563])) {s.store_add(2546, 2546, 236);}
            s.b[2565] = ((((s.v[236]) as f64).abs() <= 1e-12) && (((s.v[232]) as f64).abs() <= 1e-8));s.store_scalar(2565, if s.b[2565] { 1.0 } else { 0.0 });
            if (((((((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) && s.b[2544]) && (!s.b[2554])) && (!s.b[2563])) && s.b[2565]) {s.store_scalar(79, 1.0);}
            if (((((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) && s.b[2544]) && (!s.b[2554])) {s.store_primal_offset(97, 97, 1.0);}
        }
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_125(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) && s.b[2544]) && (!s.b[2554])) {s.copy_ad(2523, 2546);}
        if ((!s.b[1443]) && s.b[2519]) {s.store_mul_sub_scaled_inputs_rhs_indices(339, 154, 2523, -1.0, 993, -1.0);s.store_abs(2535, 339);s.store_exp(340, 339);s.store_sub_offset_lhs(341, 340, (-1.0), 339);}
        s.b[2566] = (s.v[339] > 1e-7);s.store_scalar(2566, if s.b[2566] { 1.0 } else { 0.0 });
        if (((!s.b[1443]) && s.b[2519]) && s.b[2566]) {s.store_mul_scaled_sqrt_rhs(2537, 209, -1.0, 341);}
        s.b[2567] = (s.v[2535] > 1e-7);s.store_scalar(2567, if s.b[2567] { 1.0 } else { 0.0 });
        if ((((!s.b[1443]) && s.b[2519]) && (!s.b[2566])) && s.b[2567]) {s.store_mul_sqrt_rhs(2537, 209, 341);}
        if ((((!s.b[1443]) && s.b[2519]) && (!s.b[2566])) && (!s.b[2567])) {s.store_mul_scaled_sqrt_ad_rhs(2537, 339, (-0.7071067811865475), A::offset(A::mul_scaled_lhs(s.ad_value(2535), 0.3333333333333333, A::scale_offset(s.ad_value(2535), 0.25, 1.0)), 1.0));}
        if ((!s.b[1443]) && s.b[2519]) {s.store_sqrt_square_offset(781, 2537, ((4.0 * 1e-6) * 1e-6));s.store_scaled_add(2532, 2537, 781, 0.5);s.store_div_scaled_inputs_indices(2533, 2532, 1.0, 586, 1.6021918e-19);s.store_offset(335, 2533, (-p[452]));s.store_scale(2534, 2533, 0.01);s.store_sqrt_add_scaled_square_product(781, 335, 1.0, 2534, 2534, 4.0);s.store_scaled_add(336, 335, 781, 0.5);s.store_div_scaled_product_by_product_indices(2531, 336, 336, 1.0, 2533, 2533, 1.0);s.store_add_scaled_product_mixed_iai(994, 993, 1.0, A::sub(s.ad_value(2523), s.ad_value(993)), 2531, 1.0);s.store_mul_scale_offset(333, A::exp(A::mul(s.ad_value(154), A::add_scaled_inputs3(s.ad_value(994), 1.0, s.ad_value(960), -1.0, s.ad_value(1435), 1.0))), A::exp(A::mul_scaled_lhs(s.ad_value(154), -1.0, s.ad_value(790))), -1.0, 1.0);s.store_scalar(2527, (((((2.0 * 1.6021918e-19) * s.v[489]) * 1.034943e-10)) as f64).sqrt());s.store_mul_sqrt_rhs(2528, 2527, 155);s.store_mul_sub_rhs(2521, 154, 994, 993);}
        s.b[2568] = ((s.v[2521] < (0.2 * s.v[154])) && ((0.2 * s.v[154]) >= 0.0));s.store_scalar(2568, if s.b[2568] { 1.0 } else { 0.0 });
        if (((!s.b[1443]) && s.b[2519]) && s.b[2568]) {s.store_sub_scaled_inputs(781, 154, 0.2, 2521, 1.0);s.store_square(722, 781);s.store_scaled_mul(723, 154, 154, (0.2 * 0.2));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2569] = ((((1.0 == 1.0) || (1.0 == 2.0)) || (1.0 == 4.0)) || (1.0 == 8.0));s.store_scalar(2569, if s.b[2569] { 1.0 } else { 0.0 });s.b[2570] = (1.0 == 1.0);s.store_scalar(2570, if s.b[2570] { 1.0 } else { 0.0 });
        if (((((!s.b[1443]) && s.b[2519]) && s.b[2568]) && s.b[2569]) && s.b[2570]) {s.store_scalar(720, 1.0);}
        s.b[2571] = (1.0 == 2.0);s.store_scalar(2571, if s.b[2571] { 1.0 } else { 0.0 });
        if ((((((!s.b[1443]) && s.b[2519]) && s.b[2568]) && s.b[2569]) && (!s.b[2570])) && s.b[2571]) {s.store_scalar(720, 2.0);}
        s.b[2572] = (1.0 == 4.0);s.store_scalar(2572, if s.b[2572] { 1.0 } else { 0.0 });
        if (((((((!s.b[1443]) && s.b[2519]) && s.b[2568]) && s.b[2569]) && (!s.b[2570])) && (!s.b[2571])) && s.b[2572]) {s.store_scalar(720, 3.0);}
        s.b[2573] = (1.0 == 8.0);s.store_scalar(2573, if s.b[2573] { 1.0 } else { 0.0 });
        if ((((((((!s.b[1443]) && s.b[2519]) && s.b[2568]) && s.b[2569]) && (!s.b[2570])) && (!s.b[2571])) && (!s.b[2572])) && s.b[2573]) {s.store_scalar(720, 4.0);}
        if ((((!s.b[1443]) && s.b[2519]) && s.b[2568]) && s.b[2569]) {s.store_scalar(719, 0.0);}
        let mut tc: usize = 0;
        while {
            let tb: f64 = if (((((!s.b[1443]) && s.b[2519]) && s.b[2568]) && s.b[2569]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            tb != 0.0
        } {
            tc += 1;
            if tc > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", tc, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((((!s.b[1443]) && s.b[2519]) && s.b[2568]) && s.b[2569]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
        }
        if ((((!s.b[1443]) && s.b[2519]) && s.b[2568]) && (!s.b[2569])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / 2.0));
            }
        }
        if (((!s.b[1443]) && s.b[2519]) && s.b[2568]) {s.store_div_from_scalar(726, 1.0, 726);s.store_mul3_affine_lhs(780, 781, 154, 0.2, 0.0, 726);s.store_div_scaled_product3_indices(334, 154, 725, 726, 0.2, 770, 1.0);s.store_sub_scaled_inputs(335, 154, 0.2, 780, 1.0);}
        if (((!s.b[1443]) && s.b[2519]) && s.b[2568]) {
        }
        if (((!s.b[1443]) && s.b[2519]) && (!s.b[2568])) {s.copy_ad(335, 2521);s.store_scalar(334, 1.0);}
        if ((!s.b[1443]) && s.b[2519]) {s.store_sqrt_offset_input(2529, 335, (10.0 * 2.220446049250313e-16));s.store_mul(2530, 2528, 2529);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_126(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((!s.b[1443]) && s.b[2519]) {s.store_mul_scale_offset_mixed_ai(995, A::div_scaled_inputs(s.ad_value(155), 2.0, s.ad_value(162), 1.0), 2530, p[454], 0.0);s.store_scaled_mul(46, 995, 333, s.v[632]);s.store_add(134, 136, 46);}
        if (!s.b[1443]) {s.store_add(134, 136, 46);s.copy_ad(978, 133);}
        s.store_scale(335, 162, (-s.v[635]));s.store_mul(20, 335, 131);s.store_mul(132, 335, 133);s.store_mul(19, 132, 247);s.store_mul(979, 335, 978);s.store_scaled_sub(335, 790, 94, 0.5);s.store_scale(781, 335, (2.0 * 1.0 / (p[263])));s.store_offset_mul_offset_rhs_mixed_ia(782, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 5040.0), (1.0 / 720.0))), (1.0 / 120.0)), (1.0 / 24.0)), (1.0 / 6.0)), (1.0 / 2.0), 1.0);s.store_offset_mul_offset_rhs_mixed_ia(783, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 840.0), (1.0 / 144.0))), (1.0 / 30.0)), (1.0 / 8.0)), (1.0 / 3.0), (1.0 / 2.0));s.store_div_from_scalar(110, p[263], 782);s.store_div_scaled_inputs_square_rhs(336, 783, (-2.0), 782, 1.0);s.b[2574] = ((s.v[110] < ((10.0 * 2.220446049250313e-16) + (10.0 * 2.220446049250313e-16))) && ((10.0 * 2.220446049250313e-16) >= 0.0));s.store_scalar(2574, if s.b[2574] { 1.0 } else { 0.0 });
        if s.b[2574] {s.store_sub_from_scalar(781, ((10.0 * 2.220446049250313e-16) + (10.0 * 2.220446049250313e-16)), 110);s.store_square(722, 781);s.store_scalar(723, ((10.0 * 2.220446049250313e-16) * (10.0 * 2.220446049250313e-16)));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2575] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(2575, if s.b[2575] { 1.0 } else { 0.0 });s.b[2576] = (2.0 == 1.0);s.store_scalar(2576, if s.b[2576] { 1.0 } else { 0.0 });
        if ((s.b[2574] && s.b[2575]) && s.b[2576]) {s.store_scalar(720, 1.0);}
        s.b[2577] = (2.0 == 2.0);s.store_scalar(2577, if s.b[2577] { 1.0 } else { 0.0 });
        if (((s.b[2574] && s.b[2575]) && (!s.b[2576])) && s.b[2577]) {s.store_scalar(720, 2.0);}
        s.b[2578] = (2.0 == 4.0);s.store_scalar(2578, if s.b[2578] { 1.0 } else { 0.0 });
        if ((((s.b[2574] && s.b[2575]) && (!s.b[2576])) && (!s.b[2577])) && s.b[2578]) {s.store_scalar(720, 3.0);}
        s.b[2579] = (2.0 == 8.0);s.store_scalar(2579, if s.b[2579] { 1.0 } else { 0.0 });
        if (((((s.b[2574] && s.b[2575]) && (!s.b[2576])) && (!s.b[2577])) && (!s.b[2578])) && s.b[2579]) {s.store_scalar(720, 4.0);}
        if (s.b[2574] && s.b[2575]) {s.store_scalar(719, 0.0);}
        let mut te: usize = 0;
        while {
            let td: f64 = if ((s.b[2574] && s.b[2575]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            td != 0.0
        } {
            te += 1;
            if te > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", te, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if (s.b[2574] && s.b[2575]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
        }
        if (s.b[2574] && (!s.b[2575])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if s.b[2574] {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, (10.0 * 2.220446049250313e-16));s.store_div_scaled_product_indices(334, 725, 726, (10.0 * 2.220446049250313e-16), 770, 1.0);s.store_sub_from_scalar(110, ((10.0 * 2.220446049250313e-16) + (10.0 * 2.220446049250313e-16)), 780);}
        if s.b[2574] {
        }
        if (!s.b[2574]) {
        }
        if (!s.b[2574]) {s.store_scalar(334, 1.0);}
        s.store_add(109, 87, 110);s.store_add_scaled_product_mixed_iai(134, 134, 1.0, A::div_from_scalar(s.v[163], s.ad_value(162)), 790, p[435]);s.b[2580] = (p[23] == 0.0);s.store_scalar(2580, if s.b[2580] { 1.0 } else { 0.0 });
        if s.b[2580] {s.store_scalar(280, 0.0);s.store_scalar(288, 0.0);}
        s.b[2581] = ((s.v[481] > 0.0) && (s.v[454] > 0.0));s.store_scalar(2581, if s.b[2581] { 1.0 } else { 0.0 });
        if ((!s.b[2580]) && s.b[2581]) {s.store_mul(335, 659, 85);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_127(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
    ) {
        if ((!s.b[2580]) && s.b[2581]) {s.store_scale(337, 636, 1.0 / ((s.v[188] * s.v[188])));s.store_scale_ad(338, A::div_from_scalar(2.0, s.ad_value(636)), (s.v[188] * s.v[188]));s.store_add_scaled_inputs_product_indices(339, 335, 1.0, 155, (-1.0), 660, 1438, (-1.0));s.store_offset_mul(340, 338, 339, 1.0);s.store_scaled_offset(341, 338, 1.0, 2.0);}
        s.b[2582] = ((s.v[340] < (1e-6 + s.v[341])) && (s.v[341] >= 0.0));s.store_scalar(2582, if s.b[2582] { 1.0 } else { 0.0 });
        if (((!s.b[2580]) && s.b[2581]) && s.b[2582]) {s.store_sub_offset_lhs(781, 341, 1e-6, 340);s.store_square(722, 781);s.store_square(723, 341);s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2583] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));s.store_scalar(2583, if s.b[2583] { 1.0 } else { 0.0 });s.b[2584] = (4.0 == 1.0);s.store_scalar(2584, if s.b[2584] { 1.0 } else { 0.0 });
        if (((((!s.b[2580]) && s.b[2581]) && s.b[2582]) && s.b[2583]) && s.b[2584]) {s.store_scalar(720, 1.0);}
        s.b[2585] = (4.0 == 2.0);s.store_scalar(2585, if s.b[2585] { 1.0 } else { 0.0 });
        if ((((((!s.b[2580]) && s.b[2581]) && s.b[2582]) && s.b[2583]) && (!s.b[2584])) && s.b[2585]) {s.store_scalar(720, 2.0);}
        s.b[2586] = (4.0 == 4.0);s.store_scalar(2586, if s.b[2586] { 1.0 } else { 0.0 });
        if (((((((!s.b[2580]) && s.b[2581]) && s.b[2582]) && s.b[2583]) && (!s.b[2584])) && (!s.b[2585])) && s.b[2586]) {s.store_scalar(720, 3.0);}
        s.b[2587] = (4.0 == 8.0);s.store_scalar(2587, if s.b[2587] { 1.0 } else { 0.0 });
        if ((((((((!s.b[2580]) && s.b[2581]) && s.b[2582]) && s.b[2583]) && (!s.b[2584])) && (!s.b[2585])) && (!s.b[2586])) && s.b[2587]) {s.store_scalar(720, 4.0);}
        if ((((!s.b[2580]) && s.b[2581]) && s.b[2582]) && s.b[2583]) {s.store_scalar(719, 0.0);}
        let mut t10: usize = 0;
        while {
            let tf: f64 = if (((((!s.b[2580]) && s.b[2581]) && s.b[2582]) && s.b[2583]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            tf != 0.0
        } {
            t10 += 1;
            if t10 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t10, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((((!s.b[2580]) && s.b[2581]) && s.b[2582]) && s.b[2583]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
        }
        if ((((!s.b[2580]) && s.b[2581]) && s.b[2582]) && (!s.b[2583])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 4.0)));
            }
        }
        if (((!s.b[2580]) && s.b[2581]) && s.b[2582]) {s.store_div_from_scalar(726, 1.0, 726);s.store_mul3_lhs(780, 781, 341, 726);s.store_div_scaled_product3_indices(334, 341, 725, 726, 1.0, 770, 1.0);s.store_sub_offset_lhs(340, 341, 1e-6, 780);}
        if (((!s.b[2580]) && s.b[2581]) && s.b[2582]) {
        }
        if (((!s.b[2580]) && s.b[2581]) && (!s.b[2582])) {
        }
        if (((!s.b[2580]) && s.b[2581]) && (!s.b[2582])) {s.store_scalar(334, 1.0);}
        if ((!s.b[2580]) && s.b[2581]) {s.store_sqrt(340, 340);s.store_add_mul_sub_from_scalar_rhs_indices(282, 335, 337, 1.0, 340);s.store_div_from_scalar_offset_input(336, s.v[582], 661, s.v[582]);s.store_add_scaled_inputs_product_indices(283, 1439, s.v[483], 109, 1.0, 336, 282, (-1.0));s.store_sqrt_square_offset(782, 283, ((4.0 * 0.001) * 0.001));s.store_offset_scaled_div(343, 283, 782, 0.5, 0.5);s.store_scaled_add(283, 283, 782, 0.5);}
        s.b[2588] = (s.v[283] < 0.0);s.store_scalar(2588, if s.b[2588] { 1.0 } else { 0.0 });
        if (((!s.b[2580]) && s.b[2581]) && s.b[2588]) {s.store_scalar(283, 0.0);s.store_scalar(343, 0.0);}
        if ((!s.b[2580]) && s.b[2581]) {s.store_offset(283, 283, 1e-25);s.store_offset_mul_offset_rhs(958, 957, 387, (-s.v[764]), 1.0);}
        if ((!s.b[2580]) && s.b[2581]) {
            if (s.v[958] <= 0.001) {
                s.store_scalar(958, 0.001);
            } else {
            }
        }
        if ((!s.b[2580]) && s.b[2581]) {s.store_div(339, 662, 958);s.store_mul(340, 663, 958);s.store_ad_value(336, A::exp_div_scaled_inputs(s.ad_value(340), -1.0, s.ad_value(283), 1.0));s.store_mul_product3_indices(280, 336, 339, 283, 134, 1.0);s.store_mul3_lhs(288, 339, 283, 336);}
        if ((!s.b[2580]) && (!s.b[2581])) {s.store_scalar(280, 0.0);}
        s.b[2589] = (s.v[664] != 0.0);s.store_scalar(2589, if s.b[2589] { 1.0 } else { 0.0 });
        if ((!s.b[2580]) && s.b[2589]) {s.copy_ad(334, 799);s.store_sqrt_square_offset(782, 334, ((4.0 * 1e-6) * 1e-6));s.store_offset_scaled_div(335, 334, 782, 0.5, 0.5);s.store_scaled_add(334, 334, 782, 0.5);}
        s.b[2590] = (s.v[334] < 0.0);s.store_scalar(2590, if s.b[2590] { 1.0 } else { 0.0 });
        if (((!s.b[2580]) && s.b[2589]) && s.b[2590]) {s.store_scalar(334, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_128(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((!s.b[2580]) && s.b[2589]) && s.b[2590]) {s.store_scalar(335, 0.0);}
        if ((!s.b[2580]) && s.b[2589]) {s.store_sqrt_offset_input(335, 127, 1e-25);s.store_div_from_scalar_scaled_input(337, 1.0, 335, 2.0);s.store_sub_mixed_ia(338, 334, A::scale_offset(s.ad_value(791), ((p[106]) * (p[105])), p[105]));s.store_sqrt_square_offset(782, 338, ((4.0 * 0.01) * 0.01));s.store_offset_scaled_div(343, 338, 782, 0.5, 0.5);s.store_scaled_add(338, 338, 782, 0.5);}
        s.b[2591] = (s.v[338] < 0.0);s.store_scalar(2591, if s.b[2591] { 1.0 } else { 0.0 });
        if (((!s.b[2580]) && s.b[2589]) && s.b[2591]) {s.store_scalar(338, 0.0);s.store_scalar(343, 0.0);}
        if ((!s.b[2580]) && s.b[2589]) {s.store_offset(338, 338, 1e-25);s.store_mul_ad_product_rhs_mixed_ia(344, 450, 451, A::exp(A::div_from_scalar((-1.0), s.ad_value(338))));s.store_mul_scale_offset_mixed_ia(345, 344, A::div_from_scalar(1.0, s.ad_value(338)), 1.0, 1.0);s.store_mul(337, 338, 344);s.store_sub(334, 334, 337);s.store_sqrt_square_offset(782, 334, ((4.0 * 0.01) * 0.01));s.store_offset_scaled_div(343, 334, 782, 0.5, 0.5);s.store_scaled_add(334, 334, 782, 0.5);}
        s.b[2592] = (s.v[334] < 0.0);s.store_scalar(2592, if s.b[2592] { 1.0 } else { 0.0 });
        if (((!s.b[2580]) && s.b[2589]) && s.b[2592]) {s.store_scalar(334, 0.0);s.store_scalar(343, 0.0);}
        if ((!s.b[2580]) && s.b[2589]) {s.store_offset(334, 334, 1e-25);s.store_div_scalar_by_product_indices(338, 1.0, 334, 335, 1.0);s.store_scalar(341, (s.v[165] * s.v[554]));s.store_exp_mul_scaled_lhs_indices(336, 341, -1.0, 338);s.store_mul_product3_indices(340, 338, 341, 336, 338, 1.0);s.store_mul_product3_indices(281, 336, 664, 134, 334, 1.0);}
        s.b[2593] = (p[45] == 0.0);s.store_scalar(2593, if s.b[2593] { 1.0 } else { 0.0 });
        if s.b[2593] {s.store_scalar(423, 0.0);}
        s.b[2594] = ((p[45] * (s.v[796] - p[446])) < 0.0);s.store_scalar(2594, if s.b[2594] { 1.0 } else { 0.0 });
        if ((!s.b[2593]) && s.b[2594]) {s.copy_ad(426, 427);}
        if ((!s.b[2593]) && (!s.b[2594])) {s.store_add_scaled_inputs_mixed_ai(426, A::square(A::offset(s.ad_value(796), (-p[446]))), p[445], 427, 1.0);}
        if (!s.b[2593]) {s.store_scaled_limited_exp_ad(423, A::mul(s.ad_value(154), A::sub(s.ad_value(793), s.ad_value(426))), p[449]);}
        s.b[2595] = (s.v[423] > 0.0);s.store_scalar(2595, if s.b[2595] { 1.0 } else { 0.0 });s.b[2596] = ((s.v[423] > (100000.0 - 50000.0)) && (50000.0 >= 0.0));s.store_scalar(2596, if s.b[2596] { 1.0 } else { 0.0 });
        if (s.b[2595] && s.b[2596]) {s.store_offset(781, 423, (((-100000.0)) + (50000.0)));s.store_square(722, 781);s.store_scalar(723, (50000.0 * 50000.0));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2597] = ((((1.0 == 1.0) || (1.0 == 2.0)) || (1.0 == 4.0)) || (1.0 == 8.0));s.store_scalar(2597, if s.b[2597] { 1.0 } else { 0.0 });s.b[2598] = (1.0 == 1.0);s.store_scalar(2598, if s.b[2598] { 1.0 } else { 0.0 });
        if (((s.b[2595] && s.b[2596]) && s.b[2597]) && s.b[2598]) {s.store_scalar(720, 1.0);}
        s.b[2599] = (1.0 == 2.0);s.store_scalar(2599, if s.b[2599] { 1.0 } else { 0.0 });
        if ((((s.b[2595] && s.b[2596]) && s.b[2597]) && (!s.b[2598])) && s.b[2599]) {s.store_scalar(720, 2.0);}
        s.b[2600] = (1.0 == 4.0);s.store_scalar(2600, if s.b[2600] { 1.0 } else { 0.0 });
        if (((((s.b[2595] && s.b[2596]) && s.b[2597]) && (!s.b[2598])) && (!s.b[2599])) && s.b[2600]) {s.store_scalar(720, 3.0);}
        s.b[2601] = (1.0 == 8.0);s.store_scalar(2601, if s.b[2601] { 1.0 } else { 0.0 });
        if ((((((s.b[2595] && s.b[2596]) && s.b[2597]) && (!s.b[2598])) && (!s.b[2599])) && (!s.b[2600])) && s.b[2601]) {s.store_scalar(720, 4.0);}
        if ((s.b[2595] && s.b[2596]) && s.b[2597]) {s.store_scalar(719, 0.0);}
        let mut t12: usize = 0;
        while {
            let t11: f64 = if (((s.b[2595] && s.b[2596]) && s.b[2597]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t11 != 0.0
        } {
            t12 += 1;
            if t12 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t12, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((s.b[2595] && s.b[2596]) && s.b[2597]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
        }
        if ((s.b[2595] && s.b[2596]) && (!s.b[2597])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / 2.0));
            }
        }
        if (s.b[2595] && s.b[2596]) {s.store_div_from_scalar(726, 1.0, 726);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_129(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[2595] && s.b[2596]) {s.store_scaled_mul(780, 781, 726, 50000.0);s.store_div_scaled_product_indices(334, 725, 726, 50000.0, 770, 1.0);s.store_offset(336, 780, (100000.0 - 50000.0));}
        if (s.b[2595] && s.b[2596]) {
        }
        if (s.b[2595] && (!s.b[2596])) {s.copy_ad(336, 423);s.store_scalar(334, 1.0);}
        s.b[2602] = ((((s.v[280] + s.v[281]) > 0.0) && (s.v[523] != 0.0)) && (s.v[963] == 0.0));s.store_scalar(2602, if s.b[2602] { 1.0 } else { 0.0 });
        if s.b[2602] {s.store_offset_scaled(334, 120, s.v[524], 1.0);s.store_add(335, 280, 281);s.store_scaled_mul(111, 334, 335, s.v[523]);s.store_div_from_scalar(344, 1.0, 99);s.store_mul3_lhs(335, 154, 111, 344);s.store_square(345, 344);s.store_div_from_scalar(344, 1.0, 102);s.store_mul3_lhs(336, 154, 111, 344);s.store_square(345, 344);s.store_mul_mixed_ia(112, 209, A::add_scaled_products(s.ad_value(104), s.ad_value(336), 1.0, s.ad_value(101), s.ad_value(335), (-1.0)));s.store_mul_add_scaled_products_indices_rhs(113, 209, 103, 336, ((-1.0) * (0.5)), 100, 335, 0.5);s.store_add(114, 112, 113);s.store_mul3_lhs(400, 115, 114, 253);s.store_mul(287, 288, 400);}
        s.b[2603] = (p[24] != 0.0);s.store_scalar(2603, if s.b[2603] { 1.0 } else { 0.0 });s.b[2604] = (s.v[78] == 0.0);s.store_scalar(2604, if s.b[2604] { 1.0 } else { 0.0 });
        if (s.b[2603] && s.b[2604]) {s.store_offset_add(191, 109, 1439, (-(10.0 * 2.220446049250313e-16)));s.store_sub_scaled_inputs_mixed_ai(335, A::add_scaled_product(A::offset(s.ad_value(1440), (-s.v[160])), 1.0, A::sub(s.ad_value(120), s.ad_value(182)), s.ad_value(162), s.v[560]), 1.0, 191, s.v[515]);s.store_square(335, 335);s.store_scalar(337, (1.0 / s.v[187]));s.store_mul(336, 335, 337);s.store_scalar(337, (1.0 / s.v[561]));s.store_offset_mul(341, 255, 337, 1.0);s.store_mul(195, 336, 341);s.store_sqrt_square_offset(782, 195, ((4.0 * (0.01 / 0.01)) * (0.01 / 0.01)));s.store_offset_scaled_div(339, 195, 782, 0.5, 0.5);s.store_scaled_add(195, 195, 782, 0.5);}
        s.b[2605] = (s.v[195] < 0.0);s.store_scalar(2605, if s.b[2605] { 1.0 } else { 0.0 });
        if ((s.b[2603] && s.b[2604]) && s.b[2605]) {s.store_scalar(195, 0.0);s.store_scalar(339, 0.0);}
        if (s.b[2603] && s.b[2604]) {s.store_sqrt_square_offset(782, 1440, ((4.0 * 0.001) * 0.001));s.store_offset_scaled_div(338, 1440, 782, 0.5, 0.5);s.store_scaled_add(337, 1440, 782, 0.5);}
        s.b[2606] = (s.v[337] < 0.0);s.store_scalar(2606, if s.b[2606] { 1.0 } else { 0.0 });
        if ((s.b[2603] && s.b[2604]) && s.b[2606]) {s.store_scalar(337, 0.0);s.store_scalar(338, 0.0);}
        if (s.b[2603] && s.b[2604]) {s.store_offset(337, 337, (-p[262]));s.store_scale(332, 337, 10.0);s.store_offset_square(336, 332, 1.0);s.store_sub_from_scalar_ad(335, 1.0, A::div_from_scalar(1.0, s.ad_value(336)));s.store_mul(195, 195, 335);s.store_scale(334, 162, s.v[632]);s.store_div_from_scalar_offset_input(341, s.v[562], 334, s.v[562]);s.store_scalar(340, s.v[516]);s.store_div_add_scaled_inputs_rhs_indices(343, 340, 340, 1.0, 1439, 1.0);s.store_div_from_scalar_offset_input(338, 1.0, 195, 1e-25);s.store_scaled_mul(335, 193, 338, (-s.v[514]));s.store_scaled_mul(337, 338, 338, s.v[514]);}
        s.b[2607] = (s.v[335] < (-34.0));s.store_scalar(2607, if s.b[2607] { 1.0 } else { 0.0 });
        if ((s.b[2603] && s.b[2604]) && (!s.b[2607])) {s.store_exp(336, 335);s.store_mul_scale_offset_mixed_ia(337, 334, A::div_from_scalar(s.v[513], s.ad_value(192)), 1.6021918e-19, 0.0);s.store_div_from_scalar(339, 1.0, 209);s.store_sqrt_ad(340, A::mul_offset_lhs(s.ad_value(978), (s.v[188] * 1e-12), s.ad_value(339)));s.store_mul3_lhs(338, 336, 337, 340);s.store_mul(339, 338, 195);s.store_mul(344, 339, 195);}
        if s.b[2603] {s.store_offset_scaled(334, 791, (-s.v[518]), s.v[559]);s.store_exp_scaled_input(336, 334, s.v[187]);s.store_scale(334, 791, (1.0 / (s.v[187]) * 1.0 / (s.v[187])));s.store_mul(337, 791, 334);s.store_scalar(338, ((s.v[517] / 1000000.0) * s.v[632]));s.store_sub(335, 791, 790);s.store_offset_scaled(334, 335, (-s.v[518]), s.v[559]);s.store_exp_scaled_input(336, 334, s.v[187]);s.store_scale(334, 335, (1.0 / (s.v[187]) * 1.0 / (s.v[187])));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_130(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[2603] {s.store_mul(337, 335, 334);s.store_scalar(338, ((s.v[517] / 1000000.0) * s.v[632]));s.store_scaled_offset_ad(195, A::neg(A::sub(s.ad_value(791), s.ad_value(792))), ((s.v[160]) + (p[258])), 1.0 / (s.v[187]));s.store_sqrt_square_offset(782, 195, ((4.0 * (0.01 / 0.01)) * (0.01 / 0.01)));s.store_offset_scaled_div(339, 195, 782, 0.5, 0.5);s.store_scaled_add(195, 195, 782, 0.5);}
        s.b[2610] = (s.v[195] < 0.0);s.store_scalar(2610, if s.b[2610] { 1.0 } else { 0.0 });
        if (s.b[2603] && s.b[2610]) {s.store_scalar(195, 0.0);s.store_scalar(339, 0.0);}
        if s.b[2603] {s.store_offset(195, 195, 1e-25);s.store_div_from_scalar(335, (-s.v[520]), 195);}
        s.b[2611] = (s.v[335] < (-34.0));s.store_scalar(2611, if s.b[2611] { 1.0 } else { 0.0 });
        if (s.b[2603] && (!s.b[2611])) {s.store_exp(336, 335);s.store_mul_div_from_scalar_lhs_ad_mixed_ai(337, s.v[520], A::square(s.ad_value(195)), 336);s.store_scale(337, 162, (s.v[519] * s.v[632]));}
        if s.b[2603] {s.copy_ad(285, 677);s.store_mul(286, 393, 285);s.store_scaled_offset_ad(336, A::add_scaled_inputs4(s.ad_value(1438), s.v[493], s.ad_value(1440), (-1.0), s.ad_value(122), 1.0, s.ad_value(174), 1.0), (-s.v[492]), (-1.0 / (s.v[187])));s.store_square(334, 336);s.store_scale(335, 286, s.v[491]);s.store_div_scaled_inputs_indices(337, 335, -1.0, 336, 1.0);}
        s.b[2612] = (s.v[337] < (-34.0));s.store_scalar(2612, if s.b[2612] { 1.0 } else { 0.0 });
        if (s.b[2603] && s.b[2612]) {s.store_scalar(339, 0.0);}
        if (s.b[2603] && (!s.b[2612])) {s.store_exp(339, 337);}
        if s.b[2603] {s.store_div_from_scalar(338, (((1.6021918e-19 * s.v[490]) * s.v[632]) * s.v[582]), 285);}
        s.b[2614] = (p[25] != 0.0);s.store_scalar(2614, if s.b[2614] { 1.0 } else { 0.0 });
        if s.b[2614] {s.store_offset_ad(335, A::mul_sub_from_scalar_rhs(s.ad_value(790), 1.0, A::scale(s.ad_value(790), 100.0)), (-1e-5));s.store_sqrt_add_scaled_square_input(336, 335, 1.0, 790, (4.0 * 1e-5));s.store_add_scaled_inputs3_indices(196, 790, 1.0, 335, (-0.5), 336, (-0.5));}
        s.b[2615] = (p[25] == 0.0);s.store_scalar(2615, if s.b[2615] { 1.0 } else { 0.0 });
        if (!s.b[2615]) {s.store_add_scaled_inputs4_offset_indices(335, 196, p[242], 791, (-1.0), 122, p[244], 174, p[244], (p[243] * p[242]));s.store_scalar(336, (1.0 / s.v[187]));s.store_mul(194, 335, 336);s.store_sqrt_square_offset(782, 194, ((4.0 * (0.01 / 0.01)) * (0.01 / 0.01)));s.store_offset_scaled_div(339, 194, 782, 0.5, 0.5);s.store_scaled_add(197, 194, 782, 0.5);}
        s.b[2616] = (s.v[197] < 0.0);s.store_scalar(2616, if s.b[2616] { 1.0 } else { 0.0 });
        if ((!s.b[2615]) && s.b[2616]) {s.store_scalar(197, 0.0);s.store_scalar(339, 0.0);}
        if (!s.b[2615]) {s.store_div_from_scalar_offset_input(337, 1.0, 197, 1e-25);s.store_scaled_mul(334, 193, 337, (-s.v[512]));}
        s.b[2617] = (s.v[334] < (-34.0));s.store_scalar(2617, if s.b[2617] { 1.0 } else { 0.0 });
        if ((!s.b[2615]) && (!s.b[2617])) {s.store_exp(335, 334);s.store_scale_ad(336, A::div_from_scalar(s.v[511], s.ad_value(192)), (1.6021918e-19 * s.v[632]));}
        if (!s.b[2615]) {s.store_sub(205, 790, 792);}
        s.b[2618] = (s.v[205] > 0.0);s.store_scalar(2618, if s.b[2618] { 1.0 } else { 0.0 });
        if ((!s.b[2615]) && s.b[2618]) {s.store_square(336, 205);s.store_mul(338, 336, 205);s.store_offset(334, 338, 0.5);s.store_div(339, 338, 334);s.store_div_square_rhs_mixed_ai(341, A::add_scaled_products(s.ad_value(336), s.ad_value(334), 3.0, s.ad_value(338), s.ad_value(336), (-3.0)), 334);}
        s.b[2619] = (p[25] == 0.0);s.store_scalar(2619, if s.b[2619] { 1.0 } else { 0.0 });
        if (!s.b[2619]) {s.store_add_scaled_inputs3_mixed_aii(335, A::add_scaled_inputs3_offset(s.ad_value(196), (-p[242]), s.ad_value(791), -1.0, s.ad_value(196), 1.0, ((p[243]) * (p[242]))), 1.0, 122, p[244], 174, p[244]);s.store_scalar(336, (1.0 / s.v[187]));s.store_mul(194, 335, 336);s.store_sqrt_square_offset(782, 194, ((4.0 * (0.01 / 0.01)) * (0.01 / 0.01)));s.store_offset_scaled_div(339, 194, 782, 0.5, 0.5);s.store_scaled_add(198, 194, 782, 0.5);}
        s.b[2620] = (s.v[198] < 0.0);s.store_scalar(2620, if s.b[2620] { 1.0 } else { 0.0 });
        if ((!s.b[2619]) && s.b[2620]) {s.store_scalar(198, 0.0);}
    }
}
