#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_transient_block_32(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
    ) {
        let mut t1: usize = 0;
        while {
            let t0: f64 = if (((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && s.b[962]) && (s.v[168] < s.v[58])) { 1.0 } else { 0.0 };
            t0 != 0.0
        } {
            t1 += 1;
            if t1 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", t1, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && s.b[962]) {s.copy_ad(954, 474);s.store_mul(955, 225, 354);s.store_exp_neg_input(956, 955);}
            s.b[963] = (s.v[354] > 1e-9);s.store_scalar(963, if s.b[963] { 1.0 } else { 0.0 });
            if (((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && s.b[962]) && s.b[963]) {s.store_exp_mul(953, 225, 354);s.store_mul_scaled_sqrt_ad_rhs(957, 954, -1.0, A::add_scaled_offset_product_rhs(A::offset(A::add(s.ad_value(956), s.ad_value(955)), (-1.0)), 1.0, s.ad_value(239), s.ad_value(953), (-1.0), 1.0));s.store_mul_div_from_scalar_lhs_ad_mixed_ia(958, s.v[122], 957, A::add_scaled_sub_value_product(1.0, s.ad_value(956), 1.0, s.ad_value(239), s.ad_value(953), 1.0));}
            s.b[964] = (s.v[354] < (-1e-9));s.store_scalar(964, if s.b[964] { 1.0 } else { 0.0 });
            if ((((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && s.b[962]) && (!s.b[963])) && s.b[964]) {s.store_mul_sqrt_mixed_ia(957, 954, A::offset(A::add(s.ad_value(956), s.ad_value(955)), (-1.0)));s.store_mul_scale_offset_mixed_ai(958, A::div_from_scalar(s.v[122], s.ad_value(957)), 956, -1.0, 1.0);}
            if ((((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && s.b[962]) && (!s.b[963])) && (!s.b[964])) {s.store_mul_ad_affine_product_lhs(957, A::sqrt(A::div_from_scalar(s.v[122], s.ad_value(225))), s.ad_value(225), -1.0, 0.0, 354);s.store_scaled_sqrt_scaled_input(958, 225, s.v[122], -1.0);}
            if ((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && s.b[962]) {s.store_sqrt_add_scaled_square_product(45, 957, 1.0, 739, 739, 4.0);s.store_offset_scaled_div(960, 957, 45, 0.5, 0.5);s.store_add_scaled_inputs3_indices(959, 957, 0.5, 45, 0.5, 739, 1e-10);}
            s.b[965] = (s.v[959] < 0.0);s.store_scalar(965, if s.b[965] { 1.0 } else { 0.0 });
            if (((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && s.b[962]) && s.b[965]) {s.store_scalar(959, 0.0);s.store_scalar(960, 0.0);}
            if ((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && s.b[962]) {s.store_add_scaled_inputs3_indices(44, 341, -1.0, 959, (-1.0), 740, -1.0);s.store_scaled_mul(45, 341, 740, (-4.0));}
            if ((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && s.b[962]) {
                if (s.v[45] > 0.0) {
                } else {
                    s.store_neg(45, 45);
                }
            }
            if ((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && s.b[962]) {s.store_sqrt_square_add(45, 44, 45);s.store_offset_scaled_div(335, 44, 45, 0.5, 0.5);s.store_add_scaled_inputs3_indices(959, 341, -1.0, 44, (-0.5), 45, (-0.5));s.store_mul3_lhs(960, 960, 958, 335);s.store_div_scaled_inputs_mixed_ai(388, A::square(s.ad_value(959)), ((0.5 * 9662367879.197212) * 6.241449993689894e18), 544, 1.0);s.store_div_scaled_product_indices(389, 388, 960, 2.0, 959, 1.0);s.store_sub_mixed_ia(959, 354, A::div_scaled_inputs4(s.ad_value(957), 1.0 / (s.v[93]), s.ad_value(354), (-1.0), s.ad_value(475), -1.0, s.ad_value(388), 1.0, A::add(A::scale_offset(s.ad_value(958), 1.0 / (s.v[93]), (-1.0)), s.ad_value(389)), 1.0));}
            s.b[966] = ((((s.v[959] - s.v[354])) as f64).abs() < 5e-12);s.store_scalar(966, if s.b[966] { 1.0 } else { 0.0 });
            if (((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && s.b[962]) && s.b[966]) {s.store_scalar(168, s.v[58]);}
            if ((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && s.b[962]) {s.copy_ad(354, 959);s.copy_ad(360, 957);s.store_primal_offset(168, 168, 1.0);}
        }
        if ((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && s.b[962]) {s.store_add(354, 475, 354);s.store_sub_scaled_inputs(353, 354, 1.0, 360, 1.0 / (s.v[93]));}
        if ((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && (!s.b[962])) {s.store_scalar(168, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_33(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
    ) {
        let mut t3: usize = 0;
        while {
            let t2: f64 = if (((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && (!s.b[962])) && (s.v[168] < s.v[58])) { 1.0 } else { 0.0 };
            t2 != 0.0
        } {
            t3 += 1;
            if t3 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", t3, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && (!s.b[962])) {s.copy_ad(954, 474);s.store_mul(955, 225, 354);s.store_exp_neg_input(956, 955);}
            s.b[967] = (s.v[354] > 1e-9);s.store_scalar(967, if s.b[967] { 1.0 } else { 0.0 });
            if (((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && (!s.b[962])) && s.b[967]) {s.store_exp_mul(953, 225, 354);s.store_mul_scaled_sqrt_ad_rhs(957, 954, -1.0, A::add_scaled_offset_product_rhs(A::offset(A::add(s.ad_value(956), s.ad_value(955)), (-1.0)), 1.0, s.ad_value(239), s.ad_value(953), (-1.0), 1.0));s.store_mul_div_from_scalar_lhs_ad_mixed_ia(958, s.v[122], 957, A::add_scaled_sub_value_product(1.0, s.ad_value(956), 1.0, s.ad_value(239), s.ad_value(953), 1.0));}
            s.b[968] = (s.v[354] < (-1e-9));s.store_scalar(968, if s.b[968] { 1.0 } else { 0.0 });
            if ((((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && (!s.b[962])) && (!s.b[967])) && s.b[968]) {s.store_mul_sqrt_mixed_ia(957, 954, A::offset(A::add(s.ad_value(956), s.ad_value(955)), (-1.0)));s.store_mul_scale_offset_mixed_ai(958, A::div_from_scalar(s.v[122], s.ad_value(957)), 956, -1.0, 1.0);}
            if ((((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && (!s.b[962])) && (!s.b[967])) && (!s.b[968])) {s.store_mul_ad_affine_product_lhs(957, A::sqrt(A::div_from_scalar(s.v[122], s.ad_value(225))), s.ad_value(225), -1.0, 0.0, 354);s.store_scaled_sqrt_scaled_input(958, 225, s.v[122], -1.0);}
            if ((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && (!s.b[962])) {s.store_sqrt_add_scaled_square_product(45, 957, 1.0, 739, 739, 4.0);s.store_offset_scaled_div(960, 957, 45, 0.5, 0.5);s.store_add_scaled_inputs3_indices(959, 957, 0.5, 45, 0.5, 739, 1e-10);}
            s.b[969] = (s.v[959] < 0.0);s.store_scalar(969, if s.b[969] { 1.0 } else { 0.0 });
            if (((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && (!s.b[962])) && s.b[969]) {s.store_scalar(959, 0.0);s.store_scalar(960, 0.0);}
            if ((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && (!s.b[962])) {s.store_add_scaled_inputs3_indices(44, 341, -1.0, 959, (-1.0), 740, -1.0);s.store_scaled_mul(45, 341, 740, (-4.0));}
            if ((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && (!s.b[962])) {
                if (s.v[45] > 0.0) {
                } else {
                    s.store_neg(45, 45);
                }
            }
            if ((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && (!s.b[962])) {s.store_sqrt_square_add(45, 44, 45);s.store_offset_scaled_div(335, 44, 45, 0.5, 0.5);s.store_add_scaled_inputs3_indices(959, 341, -1.0, 44, (-0.5), 45, (-0.5));s.store_mul3_lhs(960, 960, 958, 335);s.store_div_scaled_inputs_mixed_ai(388, A::square(s.ad_value(959)), ((0.5 * 9662367879.197212) * 6.241449993689894e18), 544, 1.0);s.store_div_scaled_product_indices(389, 388, 960, 2.0, 959, 1.0);s.store_sub_mixed_ia(959, 354, A::div_scaled_inputs3(A::add_scaled_product(A::add_scaled_inputs3(s.ad_value(352), 1.0, s.ad_value(354), (-1.0), s.ad_value(957), 1.0 / (s.v[93])), 1.0, A::add_scaled_inputs(s.ad_value(957), 1.0, s.ad_value(341), 0.5), s.ad_value(736), 9662367879.197212), 1.0, s.ad_value(475), (-1.0), s.ad_value(388), 1.0, A::add(A::add_scaled_product(A::scale_offset(s.ad_value(958), 1.0 / (s.v[93]), (-1.0)), 1.0, s.ad_value(958), s.ad_value(736), 9662367879.197212), s.ad_value(389)), 1.0));}
            s.b[970] = ((((s.v[959] - s.v[354])) as f64).abs() < 5e-12);s.store_scalar(970, if s.b[970] { 1.0 } else { 0.0 });
            if (((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && (!s.b[962])) && s.b[970]) {s.store_scalar(168, s.v[58]);}
            if ((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && (!s.b[962])) {s.copy_ad(354, 959);s.copy_ad(360, 957);s.store_primal_offset(168, 168, 1.0);}
        }
        if ((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && (!s.b[962])) {s.store_add(354, 475, 354);s.store_sub_scaled_inputs(353, 354, 1.0, 360, 1.0 / (s.v[93]));}
        s.b[971] = (s.v[353] < 0.0);s.store_scalar(971, if s.b[971] { 1.0 } else { 0.0 });
        if ((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && s.b[971]) {s.store_scalar(353, 0.0);}
        s.b[1007] = (s.v[349] < 0.0);s.store_scalar(1007, if s.b[1007] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_34(
        s: &mut Scratch,
    ) {
        if ((s.b[733] && (!s.b[925])) && s.b[1007]) {s.copy_ad(352, 349);}
        s.b[1008] = (s.v[353] < 0.01);s.store_scalar(1008, if s.b[1008] { 1.0 } else { 0.0 });
        if ((s.b[733] && (!s.b[925])) && s.b[1008]) {s.store_add_scaled_product_mixed_iia(353, 352, 1.0, 735, A::add_scaled_inputs(s.ad_value(341), 0.5, s.ad_value(357), 1.0), 1.0);}
        if (s.b[733] && (!s.b[925])) {s.copy_ad(346, 352);s.copy_ad(347, 353);s.copy_ad(348, 354);}
        let (t4,) = {
    if (s.b[733] && (!s.b[925])) {
        (0.0,)
    } else {
        (s.v[430],)
    }
};
        s.store_scalar(430, t4);
        if (s.b[733] && (!s.b[925])) {s.store_scalar(611, 0.0);s.store_scalar(168, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_35(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
    ) {
        let mut t22: usize = 0;
        while {
            let t21: f64 = if ((s.b[733] && (!s.b[925])) && (s.v[168] <= s.v[58])) { 1.0 } else { 0.0 };
            t21 != 0.0
        } {
            t22 += 1;
            if t22 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", t22, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if (s.b[733] && (!s.b[925])) {s.store_sub(973, 354, 475);s.store_mul(972, 225, 973);s.store_exp_neg_input(327, 972);}
            s.b[1009] = (s.v[973] < (-1e-9));s.store_scalar(1009, if s.b[1009] { 1.0 } else { 0.0 });
            if ((s.b[733] && (!s.b[925])) && s.b[1009]) {s.store_mul_sqrt_mixed_ia(360, 474, A::offset(A::add(s.ad_value(327), s.ad_value(972)), (-1.0)));s.store_div_scaled_offset_numerator_indices(979, 327, (-s.v[122]), s.v[122], 360, 1.0);}
            s.b[1010] = (s.v[973] > 1e-9);s.store_scalar(1010, if s.b[1010] { 1.0 } else { 0.0 });
            if (((s.b[733] && (!s.b[925])) && (!s.b[1009])) && s.b[1010]) {s.store_exp(974, 972);s.store_mul_scaled_sqrt_ad_rhs(360, 474, -1.0, A::add_scaled_offset_product_rhs(A::offset(A::add(s.ad_value(327), s.ad_value(972)), (-1.0)), 1.0, s.ad_value(239), A::add(s.ad_value(974), s.ad_value(972)), (-1.0), 1.0));s.store_div_mixed_ai(979, A::add_scaled_sub_value_product(1.0, s.ad_value(327), s.v[122], s.ad_value(239), A::offset(s.ad_value(974), 1.0), s.v[122]), 360);}
            if (((s.b[733] && (!s.b[925])) && (!s.b[1009])) && (!s.b[1010])) {s.store_mul_scale_offset_indices(360, 972, 474, -1.0, 0.0);s.store_mul_scale_offset_indices(979, 225, 474, -1.0, 0.0);}
            if (s.b[733] && (!s.b[925])) {s.copy_ad(362, 369);s.store_exp_ad(977, A::mul(s.ad_value(225), A::sub(s.ad_value(352), s.ad_value(157))));s.store_scalar(975, 1.0);s.store_sqrt_ad(976, A::add_scaled_product(A::div_scaled_product(s.ad_value(362), s.ad_value(362), 1.0, A::square(s.ad_value(238)), 1.0), 1.0, s.ad_value(379), A::add_scaled_inputs3(s.ad_value(977), 1.0, s.ad_value(972), 1.0, s.ad_value(975), -1.0), 2.0));s.store_div_scaled_product3_mixed_iiai(1006, 225, 379, A::offset(s.ad_value(977), 1.0), 2.0, 976, 2.0);s.store_add_scaled_product_indices(358, 362, (-1.0), 238, 976, -1.0);s.store_mul_scale_offset_indices(978, 1006, 238, -1.0, 0.0);s.store_div_scaled_inputs2_indices(973, 353, 1.0, 352, (-1.0), 738, 1.0);s.store_mul(972, 225, 973);}
            s.b[1011] = ((-s.v[972]) >= 500.0);s.store_scalar(1011, if s.b[1011] { 1.0 } else { 0.0 });
            if ((s.b[733] && (!s.b[925])) && s.b[1011]) {s.store_scaled_offset_ad(327, A::sub_from_scalar(1.0, s.ad_value(972)), (-500.0), 1.403592217853e217);s.store_scalar(333, 1.403592217853e217);}
            if ((s.b[733] && (!s.b[925])) && (!s.b[1011])) {s.store_neg(44, 972);s.store_scalar(327, 1.0);}
            let mut t11: usize = 0;
            while {
                let t10: f64 = if (((s.b[733] && (!s.b[925])) && (!s.b[1011])) && (s.v[44] >= 60.0)) { 1.0 } else { 0.0 };
                t10 != 0.0
            } {
                t11 += 1;
                if t11 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", t11, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
                if ((s.b[733] && (!s.b[925])) && (!s.b[1011])) {s.store_scale(327, 327, 1.14200738981568e26);s.store_offset(44, 44, (-60.0));}
            }
            if ((s.b[733] && (!s.b[925])) && (!s.b[1011])) {s.store_mul_exp_rhs(327, 327, 44);s.copy_ad(333, 327);}
            if (s.b[733] && (!s.b[925])) {s.store_sqrt_offset_ad(974, A::add(s.ad_value(327), s.ad_value(972)), (-1.0));}
            s.b[1012] = (s.v[973] < (-1e-9));s.store_scalar(1012, if s.b[1012] { 1.0 } else { 0.0 });
            if ((s.b[733] && (!s.b[925])) && s.b[1012]) {s.store_mul(366, 238, 974);s.store_div_scaled_product3_by_product_mixed_iiaii(367, 238, 225, A::sub_from_scalar(1.0, s.ad_value(333)), 1.0, 974, 738, 2.0);s.store_neg(368, 367);}
            s.b[1013] = (s.v[973] > 1e-9);s.store_scalar(1013, if s.b[1013] { 1.0 } else { 0.0 });
            if (((s.b[733] && (!s.b[925])) && (!s.b[1012])) && s.b[1013]) {s.store_mul_scale_offset_indices(366, 974, 238, -1.0, 0.0);s.store_div_scaled_product3_by_product_mixed_iiaii(367, 238, 225, A::sub_from_scalar(1.0, s.ad_value(333)), -1.0, 974, 738, 2.0);s.store_neg(368, 367);}
            if (((s.b[733] && (!s.b[925])) && (!s.b[1012])) && (!s.b[1013])) {s.store_scaled_mul(366, 238, 972, (-0.7071067811865476));s.store_scaled_mul(367, 238, 225, (-0.7071067811865476));s.store_neg(368, 367);}
            s.b[1014] = ((s.v[366] > (-(-s.v[406]))) && ((-s.v[406]) >= 0.0));s.store_scalar(1014, if s.b[1014] { 1.0 } else { 0.0 });
            if ((s.b[733] && (!s.b[925])) && s.b[1014]) {s.store_add_scaled_inputs(44, 366, 1.0, 406, -1.0);s.store_square(49, 44);s.store_scaled_mul(50, 406, 406, 1.0);s.store_scalar(51, 1.0);s.store_scalar(52, 1.0);}
            let (t12,) = {
    if ((s.b[733] && (!s.b[925])) && s.b[1014]) {
        (0.0,)
    } else {
        (s.v[54],)
    }
};
            s.store_scalar(54, t12);
            let (t13,) = {
    if ((s.b[733] && (!s.b[925])) && s.b[1014]) {
        (0.0,)
    } else {
        (s.v[55],)
    }
};
            s.store_scalar(55, t13);
            if ((s.b[733] && (!s.b[925])) && s.b[1014]) {s.store_scalar(48, 0.0);s.store_scalar(53, 0.0);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_add(48, 51, 52);s.copy_ad(53, 48);}
            s.b[1015] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1015, if s.b[1015] { 1.0 } else { 0.0 });s.b[1016] = (2.0 == 1.0);s.store_scalar(1016, if s.b[1016] { 1.0 } else { 0.0 });
            let (t14,) = {
    if ((((s.b[733] && (!s.b[925])) && s.b[1014]) && s.b[1015]) && s.b[1016]) {
        (1.0,)
    } else {
        (s.v[55],)
    }
};
            s.store_scalar(55, t14);s.b[1017] = (2.0 == 2.0);s.store_scalar(1017, if s.b[1017] { 1.0 } else { 0.0 });
            let (t15,) = {
    if (((((s.b[733] && (!s.b[925])) && s.b[1014]) && s.b[1015]) && (!s.b[1016])) && s.b[1017]) {
        (2.0,)
    } else {
        (s.v[55],)
    }
};
            s.store_scalar(55, t15);s.b[1018] = (2.0 == 4.0);s.store_scalar(1018, if s.b[1018] { 1.0 } else { 0.0 });
            let (t16,) = {
    if ((((((s.b[733] && (!s.b[925])) && s.b[1014]) && s.b[1015]) && (!s.b[1016])) && (!s.b[1017])) && s.b[1018]) {
        (3.0,)
    } else {
        (s.v[55],)
    }
};
            s.store_scalar(55, t16);s.b[1019] = (2.0 == 8.0);s.store_scalar(1019, if s.b[1019] { 1.0 } else { 0.0 });
            let (t17,) = {
    if (((((((s.b[733] && (!s.b[925])) && s.b[1014]) && s.b[1015]) && (!s.b[1016])) && (!s.b[1017])) && (!s.b[1018])) && s.b[1019]) {
        (4.0,)
    } else {
        (s.v[55],)
    }
};
            s.store_scalar(55, t17);
            let (t18,) = {
    if (((s.b[733] && (!s.b[925])) && s.b[1014]) && s.b[1015]) {
        (0.0,)
    } else {
        (s.v[54],)
    }
};
            s.store_scalar(54, t18);let mut t1c: usize = 0;
            while {
                let t1b: f64 = if ((((s.b[733] && (!s.b[925])) && s.b[1014]) && s.b[1015]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
                t1b != 0.0
            } {
                t1c += 1;
                if t1c > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", t1c, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
                if (((s.b[733] && (!s.b[925])) && s.b[1014]) && s.b[1015]) {s.store_sqrt(53, 53);}
                let (t1a,) = {
    if (((s.b[733] && (!s.b[925])) && s.b[1014]) && s.b[1015]) {
        let t19: f64 = (s.v[54] + 1.0);
        (t19,)
    } else {
        (s.v[54],)
    }
};
                s.store_scalar(54, t1a);
            }
            if (((s.b[733] && (!s.b[925])) && s.b[1014]) && (!s.b[1015])) {s.store_powf(53, 53, (1.0 / (2.0 * 2.0)));}
            if ((s.b[733] && (!s.b[925])) && s.b[1014]) {s.store_div_from_scalar(53, 1.0, 53);s.store_mul3_affine_lhs(1005, 44, 406, -1.0, 0.0, 53);s.store_div_scaled_product3_indices(327, 406, 52, 53, -1.0, 48, 1.0);s.store_add_scaled_inputs_mixed_ai(366, A::neg(s.ad_value(406)), -1.0, 1005, 1.0);}
            if ((s.b[733] && (!s.b[925])) && s.b[1014]) {
            }
            if ((s.b[733] && (!s.b[925])) && (!s.b[1014])) {
            }
            if ((s.b[733] && (!s.b[925])) && (!s.b[1014])) {s.store_scalar(327, 1.0);}
            if (s.b[733] && (!s.b[925])) {s.store_mul(367, 367, 327);s.store_mul(368, 368, 327);}
            s.b[1020] = ((s.v[366] < ((s.v[341] - s.v[362]) + (-(s.v[341] - s.v[362])))) && ((-(s.v[341] - s.v[362])) >= 0.0));s.store_scalar(1020, if s.b[1020] { 1.0 } else { 0.0 });
            if ((s.b[733] && (!s.b[925])) && s.b[1020]) {s.store_sub_add_scaled_inputs4_lhs_indices(44, 341, 1.0, 362, (-1.0), 341, -1.0, 362, 1.0, 366);s.store_square(49, 44);s.store_scaled_mul_ad(50, A::sub(s.ad_value(341), s.ad_value(362)), A::sub(s.ad_value(341), s.ad_value(362)), 1.0);s.store_scalar(51, 1.0);s.store_scalar(52, 1.0);}
            let (t1d,) = {
    if ((s.b[733] && (!s.b[925])) && s.b[1020]) {
        (0.0,)
    } else {
        (s.v[54],)
    }
};
            s.store_scalar(54, t1d);
            let (t1e,) = {
    if ((s.b[733] && (!s.b[925])) && s.b[1020]) {
        (0.0,)
    } else {
        (s.v[55],)
    }
};
            s.store_scalar(55, t1e);
            if ((s.b[733] && (!s.b[925])) && s.b[1020]) {s.store_scalar(48, 0.0);s.store_scalar(53, 0.0);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_add(48, 51, 52);s.copy_ad(53, 48);}
            s.b[1021] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1021, if s.b[1021] { 1.0 } else { 0.0 });s.b[1022] = (2.0 == 1.0);s.store_scalar(1022, if s.b[1022] { 1.0 } else { 0.0 });
            let (t1f,) = {
    if ((((s.b[733] && (!s.b[925])) && s.b[1020]) && s.b[1021]) && s.b[1022]) {
        (1.0,)
    } else {
        (s.v[55],)
    }
};
            s.store_scalar(55, t1f);s.b[1023] = (2.0 == 2.0);s.store_scalar(1023, if s.b[1023] { 1.0 } else { 0.0 });
            let (t20,) = {
    if (((((s.b[733] && (!s.b[925])) && s.b[1020]) && s.b[1021]) && (!s.b[1022])) && s.b[1023]) {
        (2.0,)
    } else {
        (s.v[55],)
    }
};
            s.store_scalar(55, t20);s.b[1024] = (2.0 == 4.0);s.store_scalar(1024, if s.b[1024] { 1.0 } else { 0.0 });
            let (t5,) = {
    if ((((((s.b[733] && (!s.b[925])) && s.b[1020]) && s.b[1021]) && (!s.b[1022])) && (!s.b[1023])) && s.b[1024]) {
        (3.0,)
    } else {
        (s.v[55],)
    }
};
            s.store_scalar(55, t5);s.b[1025] = (2.0 == 8.0);s.store_scalar(1025, if s.b[1025] { 1.0 } else { 0.0 });
            let (t6,) = {
    if (((((((s.b[733] && (!s.b[925])) && s.b[1020]) && s.b[1021]) && (!s.b[1022])) && (!s.b[1023])) && (!s.b[1024])) && s.b[1025]) {
        (4.0,)
    } else {
        (s.v[55],)
    }
};
            s.store_scalar(55, t6);
            let (t7,) = {
    if (((s.b[733] && (!s.b[925])) && s.b[1020]) && s.b[1021]) {
        (0.0,)
    } else {
        (s.v[54],)
    }
};
            s.store_scalar(54, t7);let mut tb: usize = 0;
            while {
                let ta: f64 = if ((((s.b[733] && (!s.b[925])) && s.b[1020]) && s.b[1021]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
                ta != 0.0
            } {
                tb += 1;
                if tb > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", tb, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
                if (((s.b[733] && (!s.b[925])) && s.b[1020]) && s.b[1021]) {s.store_sqrt(53, 53);}
                let (t9,) = {
    if (((s.b[733] && (!s.b[925])) && s.b[1020]) && s.b[1021]) {
        let t8: f64 = (s.v[54] + 1.0);
        (t8,)
    } else {
        (s.v[54],)
    }
};
                s.store_scalar(54, t9);
            }
            if (((s.b[733] && (!s.b[925])) && s.b[1020]) && (!s.b[1021])) {s.store_powf(53, 53, (1.0 / (2.0 * 2.0)));}
            if ((s.b[733] && (!s.b[925])) && s.b[1020]) {s.store_div_from_scalar(53, 1.0, 53);s.store_mul_ad_affine_product_lhs(1005, s.ad_value(44), A::sub(s.ad_value(341), s.ad_value(362)), -1.0, 0.0, 53);s.store_div_scaled_product3_mixed_aiii(327, A::sub(s.ad_value(341), s.ad_value(362)), 52, 53, -1.0, 48, 1.0);s.store_sub_add_scaled_inputs4_lhs_indices(366, 341, 1.0, 362, (-1.0), 341, -1.0, 362, 1.0, 1005);}
            if ((s.b[733] && (!s.b[925])) && s.b[1020]) {
            }
            if ((s.b[733] && (!s.b[925])) && (!s.b[1020])) {
            }
            if ((s.b[733] && (!s.b[925])) && (!s.b[1020])) {s.store_scalar(327, 1.0);}
            if (s.b[733] && (!s.b[925])) {s.store_mul(368, 368, 327);s.store_mul(367, 367, 327);s.store_add(359, 362, 366);}
            s.b[1026] = ((s.v[430] == 1.0) && (s.v[168] > 3.0));s.store_scalar(1026, if s.b[1026] { 1.0 } else { 0.0 });
            if ((s.b[733] && (!s.b[925])) && s.b[1026]) {s.copy_ad(611, 168);s.store_scalar(168, s.v[58]);}
            if ((s.b[733] && (!s.b[925])) && (!s.b[1026])) {s.store_add_scaled_inputs_product_mixed_iiia(983, 352, 1.0, 178, (-1.0), 324, A::add(A::add_scaled_inputs4(s.ad_value(360), 1.0, s.ad_value(362), 1.0, s.ad_value(358), 1.0, s.ad_value(366), 1.0), s.ad_value(393)), (-1.0));s.store_sub_from_scalar_scaled_mul_mixed_ia(984, 1.0, 324, A::add(s.ad_value(978), s.ad_value(368)), 1.0);s.store_mul_scale_offset_indices(985, 367, 324, -1.0, 0.0);s.store_mul_scale_offset_indices(986, 979, 324, -1.0, 0.0);s.store_add_scaled_product_mixed_iia(973, 352, 1.0, 735, A::add_scaled_inputs(s.ad_value(341), 0.5, s.ad_value(360), 1.0), 1.0);s.store_mul(975, 735, 979);s.store_sub(987, 353, 973);s.store_scalar(988, (-1.0));s.store_scalar(989, 1.0);s.store_neg(990, 975);s.store_add_scaled_inputs3_indices(991, 354, 1.0, 353, (-1.0), 360, (-s.v[94]));s.store_scalar(992, (-1.0));s.store_sub_from_scalar_scaled_input(993, 1.0, 979, s.v[94]);s.store_add_scaled_inputs4(994, A::mul3(s.ad_value(984), s.ad_value(989), s.ad_value(993)), 1.0, A::mul3(s.ad_value(984), s.ad_value(990), s.ad_value(992)), (-1.0), A::mul3(s.ad_value(985), s.ad_value(988), s.ad_value(993)), -1.0, A::mul3(s.ad_value(986), s.ad_value(988), s.ad_value(992)), 1.0);s.store_div_from_scalar_offset_input(995, 1.0, 994, 1e-50);s.store_add_scaled_products_indices(996, 989, 993, 1.0, 990, 992, (-1.0));s.store_add_scaled_products_indices(997, 986, 992, 1.0, 985, 993, (-1.0));s.store_add_scaled_products_indices(998, 985, 990, 1.0, 986, 989, (-1.0));s.store_mul_scale_offset_indices(999, 993, 988, -1.0, 0.0);s.store_mul(1000, 984, 993);s.store_add_scaled_products_indices(1001, 986, 988, 1.0, 984, 990, (-1.0));s.store_primal_mul(1002, 988, 992);s.store_mul_scale_offset_indices(1003, 992, 984, -1.0, 0.0);s.store_add_scaled_products_indices(1004, 984, 989, 1.0, 985, 988, (-1.0));s.store_mul_add_scaled_products3_indices_rhs(980, 995, 996, 983, -1.0, 997, 987, -1.0, 998, 991, -1.0);s.store_mul_add_scaled_products3_indices_rhs(981, 995, 999, 983, -1.0, 1000, 987, -1.0, 1001, 991, -1.0);s.store_mul_add_scaled_products3_indices_rhs(982, 995, 1002, 983, -1.0, 1003, 987, -1.0, 1004, 991, -1.0);s.store_abs(973, 980);}
            s.b[1027] = (s.v[973] < ((s.v[981]) as f64).abs());s.store_scalar(1027, if s.b[1027] { 1.0 } else { 0.0 });
            if (((s.b[733] && (!s.b[925])) && (!s.b[1026])) && s.b[1027]) {s.store_abs(973, 981);}
            s.b[1028] = (s.v[973] < ((s.v[982]) as f64).abs());s.store_scalar(1028, if s.b[1028] { 1.0 } else { 0.0 });
            if (((s.b[733] && (!s.b[925])) && (!s.b[1026])) && s.b[1028]) {s.store_abs(973, 982);}
            if ((s.b[733] && (!s.b[925])) && (!s.b[1026])) {s.store_scalar(407, 1.0);}
            s.b[1029] = (s.v[168] > 80.0);s.store_scalar(1029, if s.b[1029] { 1.0 } else { 0.0 });
            if (((s.b[733] && (!s.b[925])) && (!s.b[1026])) && s.b[1029]) {s.store_scalar(407, 125.0);}
            s.b[1030] = (s.v[168] > 40.0);s.store_scalar(1030, if s.b[1030] { 1.0 } else { 0.0 });
            if ((((s.b[733] && (!s.b[925])) && (!s.b[1026])) && (!s.b[1029])) && s.b[1030]) {s.store_scalar(407, 125.0);}
            s.b[1031] = (s.v[168] > 20.0);s.store_scalar(1031, if s.b[1031] { 1.0 } else { 0.0 });
            if (((((s.b[733] && (!s.b[925])) && (!s.b[1026])) && (!s.b[1029])) && (!s.b[1030])) && s.b[1031]) {s.store_scalar(407, 25.0);}
            s.b[1032] = (s.v[168] > 10.0);s.store_scalar(1032, if s.b[1032] { 1.0 } else { 0.0 });
            if ((((((s.b[733] && (!s.b[925])) && (!s.b[1026])) && (!s.b[1029])) && (!s.b[1030])) && (!s.b[1031])) && s.b[1032]) {s.store_scalar(407, 5.0);}
            s.b[1033] = (s.v[973] > (0.1 / s.v[407]));s.store_scalar(1033, if s.b[1033] { 1.0 } else { 0.0 });
            if (((s.b[733] && (!s.b[925])) && (!s.b[1026])) && s.b[1033]) {s.store_mul_mixed_ia(980, 980, A::div_scalar_by_product(0.1, s.ad_value(407), s.ad_value(973), 1.0));s.store_mul_mixed_ia(981, 981, A::div_scalar_by_product(0.1, s.ad_value(407), s.ad_value(973), 1.0));s.store_mul_mixed_ia(982, 982, A::div_scalar_by_product(0.1, s.ad_value(407), s.ad_value(973), 1.0));}
            if ((s.b[733] && (!s.b[925])) && (!s.b[1026])) {s.store_add(352, 352, 980);s.store_add(353, 353, 981);s.store_add(354, 354, 982);}
            let (te,) = {
    if ((s.b[733] && (!s.b[925])) && (!s.b[1026])) {
        let tc: f64 = (5e-12 * s.v[407]);let td: f64 = tc;
        (td,)
    } else {
        (s.v[408],)
    }
};
            s.store_scalar(408, te);s.b[1034] = (s.v[973] < s.v[408]);s.store_scalar(1034, if s.b[1034] { 1.0 } else { 0.0 });
            let (tf,) = {
    if (((s.b[733] && (!s.b[925])) && (!s.b[1026])) && s.b[1034]) {
        (1.0,)
    } else {
        (s.v[430],)
    }
};
            s.store_scalar(430, tf);
            if (s.b[733] && (!s.b[925])) {s.store_primal_offset(168, 168, 1.0);}
        }
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_36(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
    ) {
        if (s.b[733] && (!s.b[925])) {
            if (s.v[611] > 0.0) {
                s.copy_ad(168, 611);
            } else {
            }
        }
        s.b[1035] = (s.v[430] == 0.0);s.store_scalar(1035, if s.b[1035] { 1.0 } else { 0.0 });
        if ((s.b[733] && (!s.b[925])) && s.b[1035]) {s.copy_ad(352, 346);s.copy_ad(353, 347);s.copy_ad(354, 348);}
        if (s.b[733] && (!s.b[925])) {s.copy_ad(162, 352);s.copy_ad(157, 453);}
        s.b[1036] = (s.v[349] < 0.0);s.store_scalar(1036, if s.b[1036] { 1.0 } else { 0.0 });
        let (t23,) = {
    if ((s.b[733] && (!s.b[925])) && s.b[1036]) {
        (1.0,)
    } else {
        (s.v[145],)
    }
};
        s.store_scalar(145, t23);
        if (s.b[733] && (!s.b[925])) {s.copy_ad(374, 349);s.copy_ad(375, 352);s.store_sub(164, 375, 374);s.copy_ad(373, 351);s.store_scale(400, 401, 9662367879.197212);s.store_add_scaled_inputs3_mixed_iia(246, 358, 1.0, 355, (-1.0), A::mul3_scaled_output(s.ad_value(225), A::add(s.ad_value(358), s.ad_value(355)), A::sub(s.ad_value(375), s.ad_value(374)), 0.5), -1.0);}
        s.b[1037] = ((s.v[246] < 0.0) || (s.v[157] == 0.0));s.store_scalar(1037, if s.b[1037] { 1.0 } else { 0.0 });
        if ((s.b[733] && (!s.b[925])) && s.b[1037]) {s.store_scalar(246, 0.0);}
        if (s.b[733] && (!s.b[925])) {s.store_scaled_add(437, 359, 356, (-0.5));s.store_sub(411, 352, 349);s.store_offset(411, 411, 5e-12);s.store_div_from_scalar_offset_scaled_input(410, s.v[93], 400, s.v[93], 1.0);s.store_div_scaled_inputs2_mixed_aai(409, A::square(s.ad_value(360)), 1.0, A::square(s.ad_value(357)), (-1.0), 410, 1.0);}
        s.b[1038] = (((-s.v[409]) < (s.v[341] * 1e-5)) && ((s.v[341] * 1e-5) >= 0.0));s.store_scalar(1038, if s.b[1038] { 1.0 } else { 0.0 });
        if ((s.b[733] && (!s.b[925])) && s.b[1038]) {s.store_sub_scaled_inputs(44, 341, 1e-5, 409, -1.0);s.store_square(49, 44);s.store_scaled_mul(50, 341, 341, (1e-5 * 1e-5));s.store_scalar(51, 1.0);s.store_scalar(52, 1.0);}
        let (t24,) = {
    if ((s.b[733] && (!s.b[925])) && s.b[1038]) {
        (0.0,)
    } else {
        (s.v[54],)
    }
};
        s.store_scalar(54, t24);
        let (t25,) = {
    if ((s.b[733] && (!s.b[925])) && s.b[1038]) {
        (0.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t25);
        if ((s.b[733] && (!s.b[925])) && s.b[1038]) {s.store_scalar(48, 0.0);s.store_scalar(53, 0.0);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_add(48, 51, 52);s.copy_ad(53, 48);}
        s.b[1039] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1039, if s.b[1039] { 1.0 } else { 0.0 });s.b[1040] = (2.0 == 1.0);s.store_scalar(1040, if s.b[1040] { 1.0 } else { 0.0 });
        let (t26,) = {
    if ((((s.b[733] && (!s.b[925])) && s.b[1038]) && s.b[1039]) && s.b[1040]) {
        (1.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t26);s.b[1041] = (2.0 == 2.0);s.store_scalar(1041, if s.b[1041] { 1.0 } else { 0.0 });
        let (t27,) = {
    if (((((s.b[733] && (!s.b[925])) && s.b[1038]) && s.b[1039]) && (!s.b[1040])) && s.b[1041]) {
        (2.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t27);s.b[1042] = (2.0 == 4.0);s.store_scalar(1042, if s.b[1042] { 1.0 } else { 0.0 });
        let (t28,) = {
    if ((((((s.b[733] && (!s.b[925])) && s.b[1038]) && s.b[1039]) && (!s.b[1040])) && (!s.b[1041])) && s.b[1042]) {
        (3.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t28);s.b[1043] = (2.0 == 8.0);s.store_scalar(1043, if s.b[1043] { 1.0 } else { 0.0 });
        let (t29,) = {
    if (((((((s.b[733] && (!s.b[925])) && s.b[1038]) && s.b[1039]) && (!s.b[1040])) && (!s.b[1041])) && (!s.b[1042])) && s.b[1043]) {
        (4.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t29);
        let (t2a,) = {
    if (((s.b[733] && (!s.b[925])) && s.b[1038]) && s.b[1039]) {
        (0.0,)
    } else {
        (s.v[54],)
    }
};
        s.store_scalar(54, t2a);let mut t2e: usize = 0;
        while {
            let t2d: f64 = if ((((s.b[733] && (!s.b[925])) && s.b[1038]) && s.b[1039]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            t2d != 0.0
        } {
            t2e += 1;
            if t2e > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", t2e, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if (((s.b[733] && (!s.b[925])) && s.b[1038]) && s.b[1039]) {s.store_sqrt(53, 53);}
            let (t2c,) = {
    if (((s.b[733] && (!s.b[925])) && s.b[1038]) && s.b[1039]) {
        let t2b: f64 = (s.v[54] + 1.0);
        (t2b,)
    } else {
        (s.v[54],)
    }
};
            s.store_scalar(54, t2c);
        }
        if (((s.b[733] && (!s.b[925])) && s.b[1038]) && (!s.b[1039])) {s.store_powf(53, 53, (1.0 / (2.0 * 2.0)));}
        if ((s.b[733] && (!s.b[925])) && s.b[1038]) {s.store_div_from_scalar(53, 1.0, 53);s.store_mul3_affine_lhs(43, 44, 341, 1e-5, 0.0, 53);s.store_sub_scaled_inputs(328, 341, 1e-5, 43, 1.0);}
        if ((s.b[733] && (!s.b[925])) && (!s.b[1038])) {s.store_neg(328, 409);}
        if (s.b[733] && (!s.b[925])) {s.store_neg(409, 328);}
        s.b[1044] = (((s.v[225] * s.v[373]) - 1.0) > 0.0);s.store_scalar(1044, if s.b[1044] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_37(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
    ) {
        if ((s.b[733] && (!s.b[925])) && s.b[1044]) {s.store_sqrt_offset_ad(328, A::mul(s.ad_value(225), s.ad_value(373)), (-1.0));}
        if (s.b[733] && (!s.b[925])) {s.store_sub(414, 355, 358);}
        s.b[1045] = ((s.v[414] < (s.v[341] * 1e-5)) && ((s.v[341] * 1e-5) >= 0.0));s.store_scalar(1045, if s.b[1045] { 1.0 } else { 0.0 });
        if ((s.b[733] && (!s.b[925])) && s.b[1045]) {s.store_sub_scaled_inputs(44, 341, 1e-5, 414, 1.0);s.store_square(49, 44);s.store_scaled_mul(50, 341, 341, (1e-5 * 1e-5));s.store_scalar(51, 1.0);s.store_scalar(52, 1.0);}
        let (t2f,) = {
    if ((s.b[733] && (!s.b[925])) && s.b[1045]) {
        (0.0,)
    } else {
        (s.v[54],)
    }
};
        s.store_scalar(54, t2f);
        let (t30,) = {
    if ((s.b[733] && (!s.b[925])) && s.b[1045]) {
        (0.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t30);
        if ((s.b[733] && (!s.b[925])) && s.b[1045]) {s.store_scalar(48, 0.0);s.store_scalar(53, 0.0);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_add(48, 51, 52);s.copy_ad(53, 48);}
        s.b[1046] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1046, if s.b[1046] { 1.0 } else { 0.0 });s.b[1047] = (2.0 == 1.0);s.store_scalar(1047, if s.b[1047] { 1.0 } else { 0.0 });
        let (t31,) = {
    if ((((s.b[733] && (!s.b[925])) && s.b[1045]) && s.b[1046]) && s.b[1047]) {
        (1.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t31);s.b[1048] = (2.0 == 2.0);s.store_scalar(1048, if s.b[1048] { 1.0 } else { 0.0 });
        let (t32,) = {
    if (((((s.b[733] && (!s.b[925])) && s.b[1045]) && s.b[1046]) && (!s.b[1047])) && s.b[1048]) {
        (2.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t32);s.b[1049] = (2.0 == 4.0);s.store_scalar(1049, if s.b[1049] { 1.0 } else { 0.0 });
        let (t33,) = {
    if ((((((s.b[733] && (!s.b[925])) && s.b[1045]) && s.b[1046]) && (!s.b[1047])) && (!s.b[1048])) && s.b[1049]) {
        (3.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t33);s.b[1050] = (2.0 == 8.0);s.store_scalar(1050, if s.b[1050] { 1.0 } else { 0.0 });
        let (t34,) = {
    if (((((((s.b[733] && (!s.b[925])) && s.b[1045]) && s.b[1046]) && (!s.b[1047])) && (!s.b[1048])) && (!s.b[1049])) && s.b[1050]) {
        (4.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t34);
        let (t35,) = {
    if (((s.b[733] && (!s.b[925])) && s.b[1045]) && s.b[1046]) {
        (0.0,)
    } else {
        (s.v[54],)
    }
};
        s.store_scalar(54, t35);let mut t39: usize = 0;
        while {
            let t38: f64 = if ((((s.b[733] && (!s.b[925])) && s.b[1045]) && s.b[1046]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            t38 != 0.0
        } {
            t39 += 1;
            if t39 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", t39, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if (((s.b[733] && (!s.b[925])) && s.b[1045]) && s.b[1046]) {s.store_sqrt(53, 53);}
            let (t37,) = {
    if (((s.b[733] && (!s.b[925])) && s.b[1045]) && s.b[1046]) {
        let t36: f64 = (s.v[54] + 1.0);
        (t36,)
    } else {
        (s.v[54],)
    }
};
            s.store_scalar(54, t37);
        }
        if (((s.b[733] && (!s.b[925])) && s.b[1045]) && (!s.b[1046])) {s.store_powf(53, 53, (1.0 / (2.0 * 2.0)));}
        if ((s.b[733] && (!s.b[925])) && s.b[1045]) {s.store_div_from_scalar(53, 1.0, 53);s.store_mul3_affine_lhs(43, 44, 341, 1e-5, 0.0, 53);s.store_sub_scaled_inputs(414, 341, 1e-5, 43, 1.0);}
        if ((s.b[733] && (!s.b[925])) && (!s.b[1045])) {
        }
        if (s.b[733] && (!s.b[925])) {s.store_offset_div_scaled_inputs_mixed_ia(412, 414, (-2.0), A::mul(A::mul3(s.ad_value(225), s.ad_value(323), s.ad_value(411)), s.ad_value(411)), 1.0, 1.0);s.store_mul_ad_product_lhs_mixed_ai(328, A::square(s.ad_value(411)), 411, 411);s.store_mul(415, 412, 411);s.store_sub_from_scalar_div_indices(413, 1.0, 415, 192);}
        s.b[1051] = ((s.v[413] < 1e-5) && (1e-5 >= 0.0));s.store_scalar(1051, if s.b[1051] { 1.0 } else { 0.0 });
        if ((s.b[733] && (!s.b[925])) && s.b[1051]) {s.store_sub_from_scalar(44, 1e-5, 413);s.store_square(49, 44);s.store_scalar(50, (1e-5 * 1e-5));s.store_scalar(51, 1.0);s.store_scalar(52, 1.0);}
        let (t3a,) = {
    if ((s.b[733] && (!s.b[925])) && s.b[1051]) {
        (0.0,)
    } else {
        (s.v[54],)
    }
};
        s.store_scalar(54, t3a);
        let (t3b,) = {
    if ((s.b[733] && (!s.b[925])) && s.b[1051]) {
        (0.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t3b);
        if ((s.b[733] && (!s.b[925])) && s.b[1051]) {s.store_scalar(48, 0.0);s.store_scalar(53, 0.0);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_add(48, 51, 52);s.copy_ad(53, 48);}
        s.b[1052] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1052, if s.b[1052] { 1.0 } else { 0.0 });s.b[1053] = (2.0 == 1.0);s.store_scalar(1053, if s.b[1053] { 1.0 } else { 0.0 });
        let (t3c,) = {
    if ((((s.b[733] && (!s.b[925])) && s.b[1051]) && s.b[1052]) && s.b[1053]) {
        (1.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t3c);s.b[1054] = (2.0 == 2.0);s.store_scalar(1054, if s.b[1054] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_38(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
    ) {
        let (t3d,) = {
    if (((((s.b[733] && (!s.b[925])) && s.b[1051]) && s.b[1052]) && (!s.b[1053])) && s.b[1054]) {
        (2.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t3d);s.b[1055] = (2.0 == 4.0);s.store_scalar(1055, if s.b[1055] { 1.0 } else { 0.0 });
        let (t3e,) = {
    if ((((((s.b[733] && (!s.b[925])) && s.b[1051]) && s.b[1052]) && (!s.b[1053])) && (!s.b[1054])) && s.b[1055]) {
        (3.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t3e);s.b[1056] = (2.0 == 8.0);s.store_scalar(1056, if s.b[1056] { 1.0 } else { 0.0 });
        let (t3f,) = {
    if (((((((s.b[733] && (!s.b[925])) && s.b[1051]) && s.b[1052]) && (!s.b[1053])) && (!s.b[1054])) && (!s.b[1055])) && s.b[1056]) {
        (4.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t3f);
        let (t40,) = {
    if (((s.b[733] && (!s.b[925])) && s.b[1051]) && s.b[1052]) {
        (0.0,)
    } else {
        (s.v[54],)
    }
};
        s.store_scalar(54, t40);let mut t44: usize = 0;
        while {
            let t43: f64 = if ((((s.b[733] && (!s.b[925])) && s.b[1051]) && s.b[1052]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            t43 != 0.0
        } {
            t44 += 1;
            if t44 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", t44, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if (((s.b[733] && (!s.b[925])) && s.b[1051]) && s.b[1052]) {s.store_sqrt(53, 53);}
            let (t42,) = {
    if (((s.b[733] && (!s.b[925])) && s.b[1051]) && s.b[1052]) {
        let t41: f64 = (s.v[54] + 1.0);
        (t41,)
    } else {
        (s.v[54],)
    }
};
            s.store_scalar(54, t42);
        }
        if (((s.b[733] && (!s.b[925])) && s.b[1051]) && (!s.b[1052])) {s.store_powf(53, 53, (1.0 / (2.0 * 2.0)));}
        if ((s.b[733] && (!s.b[925])) && s.b[1051]) {s.store_div_from_scalar(53, 1.0, 53);s.store_scaled_mul(43, 44, 53, 1e-5);s.store_sub_from_scalar(413, 1e-5, 43);}
        if ((s.b[733] && (!s.b[925])) && (!s.b[1051])) {
        }
        if (s.b[733] && (!s.b[925])) {s.copy_ad(190, 413);s.store_offset_mul_offset_rhs(478, 190, 190, 1.0, 1.0);}
        if (s.b[733] && (!s.b[925])) {
            if ((1.0 + s.v[190]) >= (10.0 * 2.220446049250313e-16)) {
                s.store_offset(479, 190, 1.0);
            } else {
                s.store_scalar(479, (10.0 * 2.220446049250313e-16));
            }
        }
        if (s.b[733] && (!s.b[925])) {s.store_scaled_add(436, 355, 358, (-0.5));}
        if (!s.b[733]) {s.copy_ad(515, 154);}
        s.b[1063] = (s.v[416] < p[237]);s.store_scalar(1063, if s.b[1063] { 1.0 } else { 0.0 });
        let (t45,) = {
    if ((!s.b[733]) && s.b[1063]) {
        (1.0,)
    } else {
        (s.v[339],)
    }
};
        s.store_scalar(339, t45);
        let (t46,) = {
    if ((!s.b[733]) && (!s.b[1063])) {
        (2.0,)
    } else {
        (s.v[339],)
    }
};
        s.store_scalar(339, t46);
        let (t4a,) = {
    if (!s.b[733]) {
        let t47: f64 = (s.v[123] - s.v[185]);let t48: f64 = (t47 + s.v[320]);let t49: f64 = (t48 + s.v[515]);
        (t49,)
    } else {
        (s.v[160],)
    }
};
        s.store_scalar(160, t4a);s.b[1064] = (s.v[158] < s.v[160]);s.store_scalar(1064, if s.b[1064] { 1.0 } else { 0.0 });
        let (t4c,) = {
    if ((!s.b[733]) && s.b[1064]) {
        let t4b: f64 = (-1.0);
        (t4b,)
    } else {
        (s.v[338],)
    }
};
        s.store_scalar(338, t4c);
        if ((!s.b[733]) && s.b[1064]) {s.store_mul_scaled_ln_ad_rhs(254, 227, 2.0, A::div_from_scalar((-s.v[139]), s.ad_value(240)));s.store_mul_sub_rhs(336, 225, 159, 515);s.store_div_scalar_by_product_indices(328, 1.0, 225, 238, 1.0);s.store_mul(337, 328, 323);s.store_offset_scaled(262, 337, (3.0 * 1.414213562373095), 2.0);s.store_mul3_affine_lhs(260, 262, 262, 8.0, 0.0, 262);s.store_offset(331, 336, (-2.0));s.store_scaled_mul(332, 337, 331, 9.0);s.store_sub_from_scalar(261, (7.0 * 1.414213562373095), 332);s.store_square(259, 261);}
        s.b[1065] = (s.v[260] < (s.v[259] * 1e-8));s.store_scalar(1065, if s.b[1065] { 1.0 } else { 0.0 });
        if (((!s.b[733]) && s.b[1064]) && s.b[1065]) {s.store_add_scaled_inputs3_offset_mixed_iai(257, 261, 1.0, A::div_scaled_inputs(s.ad_value(260), 0.5, s.ad_value(261), 1.0), 1.0, 332, 1.0, ((-7.0) * 1.414213562373095));}
        if (((!s.b[733]) && s.b[1064]) && (!s.b[1065])) {s.store_sqrt_add(258, 260, 259);s.store_add_offset_lhs(257, 258, ((-7.0) * 1.414213562373095), 332);}
        if ((!s.b[733]) && s.b[1064]) {s.store_powf(256, 257, 0.3333333333333333);s.store_add_scaled_inputs_product_mixed_aiii(255, A::sub_from_scalar(((-4.0) * 1.414213562373095), A::scale(s.ad_value(337), 12.0)), 1.0, 256, 2.0, 256, 256, 1.414213562373095);s.store_div_from_scalar(328, 1.0, 256);s.store_mul(181, 255, 328);s.store_add_scaled_product_indices(313, 515, 1.0, 181, 227, 1.0);s.store_sub(328, 313, 515);s.store_div(329, 328, 254);s.store_sqrt_square_offset(330, 329, 1.0);s.store_add_div_lhs_indices(161, 328, 330, 515);}
        s.b[1066] = (s.v[144] >= 1.0);s.store_scalar(1066, if s.b[1066] { 1.0 } else { 0.0 });
        if (((!s.b[733]) && (!s.b[1064])) && s.b[1066]) {s.store_scalar(349, s.v[619]);s.store_scalar(378, s.v[619]);}
        if (((!s.b[733]) && (!s.b[1064])) && (!s.b[1066])) {s.store_offset_div_scaled_offset_numerator(336, A::mul(s.ad_value(225), A::sub(s.ad_value(159), s.ad_value(515))), 4.0, ((-1.0) * 4.0), A::mul(s.ad_value(241), s.ad_value(226)), 1.0, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_39(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if (((!s.b[733]) && (!s.b[1064])) && (!s.b[1066])) {
            if (s.v[336] >= (10.0 * 2.220446049250313e-16)) {
            } else {
                s.store_scalar(336, (10.0 * 2.220446049250313e-16));
            }
        }
        if (((!s.b[733]) && (!s.b[1064])) && (!s.b[1066])) {s.store_add_product3_rhs_mixed_iia(376, 159, 241, 225, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(336))), 0.5);s.store_mul_sub_rhs(181, 225, 376, 515);}
        s.b[1067] = (s.v[181] < 3.0);s.store_scalar(1067, if s.b[1067] { 1.0 } else { 0.0 });
        if ((((!s.b[733]) && (!s.b[1064])) && (!s.b[1066])) && s.b[1067]) {s.store_mul_sub_rhs(337, 225, 159, 515);s.store_div_scalar_by_product_indices(328, 1.0, 225, 240, (1.414213562373095 / 108.0));s.store_offset_scaled(329, 328, 3.0, 81.0);s.store_add_scaled_sub_value_product_mixed_aii(330, (-2916.0), A::scale(s.ad_value(328), 81.0), 1.0, 328, 337, 27.0);s.store_add_scaled_sub_value_product_mixed_aii(331, 1458.0, A::scaled_offset(s.ad_value(328), 54.0, 81.0), 1.0, 328, 337, 27.0);s.store_square(331, 331);s.store_powf_ad(332, A::add(s.ad_value(330), A::sqrt(A::add(A::mul3_scaled_output(s.ad_value(329), s.ad_value(329), s.ad_value(329), 4.0), s.ad_value(331)))), 0.3333333333333333);s.store_add_scaled_inputs_mixed_ai(336, A::sub_from_scalar(3.0, A::div_scaled_inputs(s.ad_value(329), 1.259921049894873, s.ad_value(332), 3.0)), 1.0, 332, (1.0 / (3.0 * 1.259921049894873)));s.store_add_scaled_product_indices(376, 515, 1.0, 336, 227, 1.0);s.copy_ad(378, 376);}
        s.b[1068] = (s.v[158] <= s.v[182]);s.store_scalar(1068, if s.b[1068] { 1.0 } else { 0.0 });
        if (((((!s.b[733]) && (!s.b[1064])) && (!s.b[1066])) && (!s.b[1067])) && s.b[1068]) {s.copy_ad(378, 376);}
        if (((((!s.b[733]) && (!s.b[1064])) && (!s.b[1066])) && (!s.b[1067])) && (!s.b[1068])) {s.store_div_scalar_by_product_indices(328, 1.0, 379, 434, 1.0);s.store_mul3_lhs(329, 328, 159, 159);s.store_add_div_from_scalar_rhs(330, 225, 2.0, 159);s.store_div_ln_lhs(377, 329, 330);s.store_offset_sub(44, 377, 376, (-0.0008));s.store_scale(45, 377, (4.0 * 0.0008));}
        if (((((!s.b[733]) && (!s.b[1064])) && (!s.b[1066])) && (!s.b[1067])) && (!s.b[1068])) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }
        if (((((!s.b[733]) && (!s.b[1064])) && (!s.b[1066])) && (!s.b[1067])) && (!s.b[1068])) {s.store_sqrt_square_add(45, 44, 45);s.store_add_scaled_inputs3_indices(378, 377, 1.0, 44, (-0.5), 45, (-0.5));}
        if (((!s.b[733]) && (!s.b[1064])) && (!s.b[1066])) {s.store_offset(336, 515, (5e-12 / 2.0));}
        s.b[1069] = (s.v[378] < s.v[336]);s.store_scalar(1069, if s.b[1069] { 1.0 } else { 0.0 });
        if ((((!s.b[733]) && (!s.b[1064])) && (!s.b[1066])) && s.b[1069]) {s.copy_ad(378, 336);}
        if ((!s.b[733]) && (!s.b[1064])) {s.copy_ad(161, 378);s.copy_ad(163, 376);}
        s.b[1070] = ((p[25] == 1.0) && (p[26] == 2.0));s.store_scalar(1070, if s.b[1070] { 1.0 } else { 0.0 });
        if ((!s.b[733]) && s.b[1070]) {s.store_scaled_voltage(393, ctx, nodes, Some(17), None, (1e-9 / 0.0001));}
        if ((!s.b[733]) && (!s.b[1070])) {s.store_scalar(393, 0.0);}
        if (!s.b[733]) {s.store_exp_mul(486, 225, 515);s.store_mul(487, 379, 486);}
        let (t4d,) = {
    if (!s.b[733]) {
        (0.0,)
    } else {
        (s.v[430],)
    }
};
        s.store_scalar(430, t4d);
        if (!s.b[733]) {s.copy_ad(349, 161);s.store_scale(419, 229, ((p[237] * (p[237] * 0.5)) * 9662367879.197212));s.store_sqrt_mul_scaled_lhs(327, 225, 2.0, 419);s.store_scaled_add_ad(328, A::exp(s.ad_value(327)), A::exp_scaled_input(s.ad_value(327), -1.0), 0.5);s.store_div_ln_lhs(420, 328, 419);s.store_scalar(167, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_40(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
    ) {
        let mut t55: usize = 0;
        while {
            let t53: f64 = (s.v[57] + 1.0);let t54: f64 = if ((!s.b[733]) && (s.v[167] <= t53)) { 1.0 } else { 0.0 };
            t54 != 0.0
        } {
            t55 += 1;
            if t55 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", t55, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if (!s.b[733]) {s.store_sub(417, 349, 515);s.store_mul(181, 225, 417);s.store_mul_sub_rhs(337, 420, 417, 419);}
            s.b[1071] = (s.v[337] < 80.0);s.store_scalar(1071, if s.b[1071] { 1.0 } else { 0.0 });
            if ((!s.b[733]) && s.b[1071]) {s.store_exp(328, 337);s.store_exp_mul_scaled_lhs_indices(327, 420, -1.0, 419);s.store_sub(329, 328, 327);s.store_div_ln_offset_lhs(422, 329, 1.0, 420);s.store_div_scaled_value_offset_denominator(423, s.ad_value(328), 1.0, s.ad_value(329), 1.0, 1.0);}
            if ((!s.b[733]) && (!s.b[1071])) {s.store_sub(422, 417, 419);s.store_scalar(423, 1.0);}
            if (!s.b[733]) {s.store_mul(421, 225, 422);}
            s.b[1072] = (((s.v[181]) as f64).abs() < 1e-16);s.store_scalar(1072, if s.b[1072] { 1.0 } else { 0.0 });
            if ((!s.b[733]) && s.b[1072]) {s.store_sqrt_scaled_input_ad(327, A::sub_from_scalar(1.0, A::square(s.ad_value(423))), 1.0 / (2.0));s.store_mul(242, 181, 327);s.store_mul(443, 225, 327);}
            s.b[1073] = (s.v[181] < 0.0);s.store_scalar(1073, if s.b[1073] { 1.0 } else { 0.0 });
            if (((!s.b[733]) && s.b[1072]) && s.b[1073]) {s.store_neg(242, 242);s.store_neg(443, 443);}
            s.b[1074] = (((s.v[181]) as f64).abs() < 0.005);s.store_scalar(1074, if s.b[1074] { 1.0 } else { 0.0 });
            if (((!s.b[733]) && (!s.b[1072])) && s.b[1074]) {s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(327, 181, 1.0, 181, 1.0, 181, 1.0, 181, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));s.store_mul_scale_offset_mixed_ia(328, 181, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(181), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(181), 1.0, A::scale(s.ad_value(181), 0.25), 1.0 / (3.0)), 1.0 / (2.0)), -1.0, 1.0);s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(329, 421, 1.0, 421, 1.0, 421, 1.0, 421, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));s.store_mul_scale_offset_mixed_ia(330, 421, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(421), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(421), 1.0, A::scale(s.ad_value(421), 0.25), 1.0 / (3.0)), 1.0 / (2.0)), -1.0, 1.0);s.store_sqrt_sub(242, 327, 329);s.store_div_scaled_product_mixed_iai(443, 225, A::add_scaled_product(s.ad_value(328), 1.0, s.ad_value(423), s.ad_value(330), (-1.0)), 0.5, 242, 1.0);}
            if (((!s.b[733]) && (!s.b[1072])) && (!s.b[1074])) {s.store_exp_neg_input(327, 181);s.store_exp_neg_input(328, 421);s.store_sqrt_ad(242, A::add_scaled_inputs4(s.ad_value(181), 1.0, s.ad_value(421), (-1.0), s.ad_value(327), 1.0, s.ad_value(328), (-1.0)));s.store_div_scaled_product_mixed_iai(443, 225, A::sub(A::sub_from_scalar(1.0, s.ad_value(327)), A::mul_sub_from_scalar_rhs(s.ad_value(423), 1.0, s.ad_value(328))), 0.5, 242, 1.0);}
            s.b[1075] = ((s.v[430] == 1.0) && (s.v[181] < 0.0));s.store_scalar(1075, if s.b[1075] { 1.0 } else { 0.0 });
            let (t4f,) = {
    if ((!s.b[733]) && s.b[1075]) {
        let t4e: f64 = (-1.0);
        (t4e,)
    } else {
        (s.v[338],)
    }
};
            s.store_scalar(338, t4f);s.b[1076] = (s.v[338] == (-1.0));s.store_scalar(1076, if s.b[1076] { 1.0 } else { 0.0 });
            if ((!s.b[733]) && s.b[1076]) {s.store_scalar(401, 0.0);}
            if ((!s.b[733]) && (!s.b[1076])) {s.store_mul(401, 444, 242);}
            s.b[1077] = (s.v[401] < (p[237] * 1.01));s.store_scalar(1077, if s.b[1077] { 1.0 } else { 0.0 });
            let (t50,) = {
    if ((!s.b[733]) && s.b[1077]) {
        (1.0,)
    } else {
        (s.v[339],)
    }
};
            s.store_scalar(339, t50);
            let (t51,) = {
    if ((!s.b[733]) && (!s.b[1077])) {
        (2.0,)
    } else {
        (s.v[339],)
    }
};
            s.store_scalar(339, t51);
            if (!s.b[733]) {s.store_mul(370, 229, 401);}
            s.b[1078] = (s.v[181] < 0.0);s.store_scalar(1078, if s.b[1078] { 1.0 } else { 0.0 });
            if ((!s.b[733]) && s.b[1078]) {s.store_neg(490, 242);s.store_neg(491, 443);}
            s.b[1079] = (s.v[181] < 1e-7);s.store_scalar(1079, if s.b[1079] { 1.0 } else { 0.0 });
            if (((!s.b[733]) && (!s.b[1078])) && s.b[1079]) {s.copy_ad(490, 242);s.copy_ad(491, 443);}
            s.b[1080] = (s.v[181] < 80.0);s.store_scalar(1080, if s.b[1080] { 1.0 } else { 0.0 });
            if ((((!s.b[733]) && (!s.b[1078])) && (!s.b[1079])) && s.b[1080]) {s.store_exp(243, 181);s.store_mul_sub_mixed_iia(488, 487, 243, A::offset(s.ad_value(181), 1.0));s.store_mul_ad_product_rhs_mixed_ia(489, 487, 225, A::offset(s.ad_value(243), (-1.0)));}
            if ((((!s.b[733]) && (!s.b[1078])) && (!s.b[1079])) && (!s.b[1080])) {s.store_exp_mul(485, 225, 349);s.store_mul_mixed_ia(488, 379, A::add_scaled_offset_product_rhs(s.ad_value(485), 1.0, s.ad_value(486), s.ad_value(181), 1.0, (-1.0)));s.store_mul_ad_product_rhs_mixed_ia(489, 379, 225, A::sub(s.ad_value(485), s.ad_value(486)));}
            if (((!s.b[733]) && (!s.b[1078])) && (!s.b[1079])) {s.store_sqrt_square_add(490, 242, 488);s.store_div_scaled_add_product_indices(491, 489, 0.5, 443, 242, (2.0 * 0.5), 490, 1.0);}
            if (!s.b[733]) {s.store_add_scaled_inputs_products_indices(492, 349, 1.0, 159, (-1.0), 240, 490, 1.0, 324, 393, (-1.0));s.store_offset_mul(493, 240, 491, 1.0);}
            s.b[1081] = (s.v[430] == 1.0);s.store_scalar(1081, if s.b[1081] { 1.0 } else { 0.0 });
            if ((!s.b[733]) && s.b[1081]) {s.store_scalar(167, (s.v[57] + 1.0));}
            if ((!s.b[733]) && (!s.b[1081])) {s.store_div_scaled_inputs_indices(494, 492, -1.0, 493, 1.0);}
            if ((!s.b[733]) && (!s.b[1081])) {
                s.store_scaled_offset_ad(496, {
                    if (1.0 >= ((s.v[349]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(349))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[1082] = (((s.v[494]) as f64).abs() > s.v[496]);s.store_scalar(1082, if s.b[1082] { 1.0 } else { 0.0 });
            if (((!s.b[733]) && (!s.b[1081])) && s.b[1082]) {s.store_scale(494, 496, (if (s.v[494] >= 0.0) { 1.0 } else { (-1.0) }));}
            if ((!s.b[733]) && (!s.b[1081])) {s.store_add(349, 349, 494);}
            s.b[1083] = ((((s.v[494]) as f64).abs() <= 5e-12) && (((s.v[492]) as f64).abs() <= 1e-8));s.store_scalar(1083, if s.b[1083] { 1.0 } else { 0.0 });
            let (t52,) = {
    if (((!s.b[733]) && (!s.b[1081])) && s.b[1083]) {
        (1.0,)
    } else {
        (s.v[430],)
    }
};
            s.store_scalar(430, t52);
            if (!s.b[733]) {s.store_primal_offset(167, 167, 1.0);}
        }
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_41(
        s: &mut Scratch,
    ) {
        if (!s.b[733]) {s.store_primal_offset(167, 167, (-1.0));s.copy_ad(371, 370);s.copy_ad(356, 371);s.copy_ad(161, 349);s.store_div(568, 371, 238);s.store_offset_square(169, 568, (10.0 * 2.220446049250313e-16));s.store_scale(328, 568, 2.0);s.store_offset(170, 568, (10.0 * 2.220446049250313e-16));s.store_mul(245, 238, 170);s.store_div_from_scalar_add_ad(328, 1.0, s.ad_value(490), s.ad_value(170));s.store_mul3_lhs(244, 238, 488, 328);s.store_neg(355, 244);s.store_mul(192, 244, 324);}
        s.b[1084] = ((s.v[338] == (-1.0)) || (s.v[192] <= 1e-12));s.store_scalar(1084, if s.b[1084] { 1.0 } else { 0.0 });
        let (t56,) = {
    if ((!s.b[733]) && s.b[1084]) {
        (4.0,)
    } else {
        (s.v[338],)
    }
};
        s.store_scalar(338, t56);
        let (t57,) = {
    if ((!s.b[733]) && s.b[1084]) {
        (1.0,)
    } else {
        (s.v[145],)
    }
};
        s.store_scalar(145, t57);
        if ((!s.b[733]) && s.b[1084]) {s.store_sub(329, 159, 161);s.store_mul(437, 323, 329);s.store_scale(327, 108, (-s.v[98]));s.store_mul(196, 327, 437);s.store_scalar(197, 0.0);s.store_scalar(198, 0.0);s.store_mul_scale_offset_indices(329, 437, 534, -1.0, 0.0);s.store_scale(468, 329, s.v[438]);s.store_sub(467, 329, 468);s.store_scalar(470, 0.0);s.store_scalar(469, 0.0);s.store_scalar(199, 0.0);s.store_scalar(192, 0.0);}
        let (t58,) = {
    if ((!s.b[733]) && s.b[1084]) {
        (1.0,)
    } else {
        (s.v[145],)
    }
};
        s.store_scalar(145, t58);
        if ((!s.b[733]) && s.b[1084]) {s.copy_ad(352, 349);s.copy_ad(162, 161);s.copy_ad(314, 162);}
        let (t59,) = {
    if ((!s.b[733]) && s.b[1084]) {
        (1.0,)
    } else {
        (s.v[612],)
    }
};
        s.store_scalar(612, t59);s.b[1085] = (s.v[612] == 0.0);s.store_scalar(1085, if s.b[1085] { 1.0 } else { 0.0 });
        if ((!s.b[733]) && s.b[1085]) {s.copy_ad(453, 157);s.store_scalar(1092, 1e-50);s.store_div_square_rhs(1087, 545, 323);s.store_offset_mul_ad(1089, A::div_from_scalar(2.0, s.ad_value(1087)), A::sub(s.ad_value(159), s.ad_value(1092)), 1.0);s.store_offset_div_from_scalar_ad(332, 2.0, s.ad_value(1087), 1.0);}
        s.b[1093] = ((s.v[1089] < s.v[332]) && (s.v[332] >= 0.0));s.store_scalar(1093, if s.b[1093] { 1.0 } else { 0.0 });
        if (((!s.b[733]) && s.b[1085]) && s.b[1093]) {s.store_sub(44, 332, 1089);s.store_square(49, 44);s.store_square(50, 332);s.store_scalar(51, 1.0);s.store_scalar(52, 1.0);}
        let (t5a,) = {
    if (((!s.b[733]) && s.b[1085]) && s.b[1093]) {
        (0.0,)
    } else {
        (s.v[54],)
    }
};
        s.store_scalar(54, t5a);
        let (t5b,) = {
    if (((!s.b[733]) && s.b[1085]) && s.b[1093]) {
        (0.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t5b);
        if (((!s.b[733]) && s.b[1085]) && s.b[1093]) {s.store_scalar(48, 0.0);s.store_scalar(53, 0.0);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_add(48, 51, 52);s.copy_ad(53, 48);}
        s.b[1094] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));s.store_scalar(1094, if s.b[1094] { 1.0 } else { 0.0 });s.b[1095] = (4.0 == 1.0);s.store_scalar(1095, if s.b[1095] { 1.0 } else { 0.0 });
        let (t5c,) = {
    if (((((!s.b[733]) && s.b[1085]) && s.b[1093]) && s.b[1094]) && s.b[1095]) {
        (1.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t5c);s.b[1096] = (4.0 == 2.0);s.store_scalar(1096, if s.b[1096] { 1.0 } else { 0.0 });
        let (t5d,) = {
    if ((((((!s.b[733]) && s.b[1085]) && s.b[1093]) && s.b[1094]) && (!s.b[1095])) && s.b[1096]) {
        (2.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t5d);s.b[1097] = (4.0 == 4.0);s.store_scalar(1097, if s.b[1097] { 1.0 } else { 0.0 });
        let (t5e,) = {
    if (((((((!s.b[733]) && s.b[1085]) && s.b[1093]) && s.b[1094]) && (!s.b[1095])) && (!s.b[1096])) && s.b[1097]) {
        (3.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t5e);s.b[1098] = (4.0 == 8.0);s.store_scalar(1098, if s.b[1098] { 1.0 } else { 0.0 });
        let (t5f,) = {
    if ((((((((!s.b[733]) && s.b[1085]) && s.b[1093]) && s.b[1094]) && (!s.b[1095])) && (!s.b[1096])) && (!s.b[1097])) && s.b[1098]) {
        (4.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t5f);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_42(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
    ) {
        let (t60,) = {
    if ((((!s.b[733]) && s.b[1085]) && s.b[1093]) && s.b[1094]) {
        (0.0,)
    } else {
        (s.v[54],)
    }
};
        s.store_scalar(54, t60);let mut t64: usize = 0;
        while {
            let t63: f64 = if (((((!s.b[733]) && s.b[1085]) && s.b[1093]) && s.b[1094]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            t63 != 0.0
        } {
            t64 += 1;
            if t64 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", t64, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((((!s.b[733]) && s.b[1085]) && s.b[1093]) && s.b[1094]) {s.store_sqrt(53, 53);}
            let (t62,) = {
    if ((((!s.b[733]) && s.b[1085]) && s.b[1093]) && s.b[1094]) {
        let t61: f64 = (s.v[54] + 1.0);
        (t61,)
    } else {
        (s.v[54],)
    }
};
            s.store_scalar(54, t62);
        }
        if ((((!s.b[733]) && s.b[1085]) && s.b[1093]) && (!s.b[1094])) {s.store_powf(53, 53, (1.0 / (2.0 * 4.0)));}
        if (((!s.b[733]) && s.b[1085]) && s.b[1093]) {s.store_div_from_scalar(53, 1.0, 53);s.store_mul3_lhs(43, 44, 332, 53);s.store_sub(1089, 332, 43);}
        if (((!s.b[733]) && s.b[1085]) && (!s.b[1093])) {
        }
        if ((!s.b[733]) && s.b[1085]) {s.store_sqrt(1088, 1089);s.store_add_mul_sub_from_scalar_rhs_indices(1092, 159, 1087, 1.0, 1088);s.store_sqrt_square_offset(44, 1092, ((4.0 * 0.01) * 0.01));s.store_offset_add_scaled_inputs_indices(1092, 1092, 0.5, 44, 0.5, (1e-10 * 0.01));}
        s.b[1099] = (s.v[1092] < 0.0);s.store_scalar(1099, if s.b[1099] { 1.0 } else { 0.0 });
        if (((!s.b[733]) && s.b[1085]) && s.b[1099]) {s.store_scalar(1092, 0.0);}
        if ((!s.b[733]) && s.b[1085]) {s.store_div(1086, 157, 1092);s.store_pow_offset_rhs(1087, 1086, 138, (-1.0));s.store_mul(1091, 1087, 1086);s.store_offset(1088, 1091, 1.0);s.store_pow_ad(1089, s.ad_value(1088), A::offset(A::div_from_scalar(1.0, s.ad_value(138)), (-1.0)));s.store_mul(1090, 1089, 1088);s.store_div(452, 157, 1090);s.copy_ad(157, 452);s.store_exp_ad(484, A::mul(s.ad_value(225), A::sub(s.ad_value(515), s.ad_value(157))));}
        s.b[1100] = (s.v[157] <= 0.0);s.store_scalar(1100, if s.b[1100] { 1.0 } else { 0.0 });
        if (((!s.b[733]) && s.b[1085]) && s.b[1100]) {s.store_scalar(164, 0.0);s.copy_ad(162, 161);}
        let (t65,) = {
    if (((!s.b[733]) && s.b[1085]) && s.b[1100]) {
        (0.0,)
    } else {
        (s.v[430],)
    }
};
        s.store_scalar(430, t65);s.b[1101] = (s.v[144] >= 1.0);s.store_scalar(1101, if s.b[1101] { 1.0 } else { 0.0 });
        if ((((!s.b[733]) && s.b[1085]) && (!s.b[1100])) && s.b[1101]) {s.store_scalar(352, s.v[622]);s.store_sub_from_scalar(165, s.v[622], 161);}
        s.b[1102] = (s.v[144] == 0.0);s.store_scalar(1102, if s.b[1102] { 1.0 } else { 0.0 });
        if ((((!s.b[733]) && s.b[1085]) && (!s.b[1100])) && s.b[1102]) {
            if ((s.v[163] - s.v[161]) >= 0.0) {
                s.store_sub(166, 163, 161);
            } else {
                s.store_scalar(166, 0.0);
            }
        }
        if ((((!s.b[733]) && s.b[1085]) && (!s.b[1100])) && s.b[1102]) {s.store_offset_sub_scaled_inputs_indices(44, 166, (1.0 + 0.3), 157, 1.0, (-0.03));s.store_scale(45, 166, ((1.0 + 0.3) * (4.0 * 0.03)));}
        if ((((!s.b[733]) && s.b[1085]) && (!s.b[1100])) && s.b[1102]) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }
        if ((((!s.b[733]) && s.b[1085]) && (!s.b[1100])) && s.b[1102]) {s.store_sqrt_square_add(45, 44, 45);s.store_add_scaled_inputs3_indices(165, 166, (1.0 + 0.3), 44, (-0.5), 45, (-0.5));}
        if ((((!s.b[733]) && s.b[1085]) && (!s.b[1100])) && s.b[1102]) {
            if (s.v[165] <= s.v[166]) {
            } else {
                s.copy_ad(165, 166);
            }
        }
        s.b[1103] = (s.v[165] < 0.0);s.store_scalar(1103, if s.b[1103] { 1.0 } else { 0.0 });
        if ((((!s.b[733]) && s.b[1085]) && (!s.b[1100])) && s.b[1103]) {s.store_scalar(165, 0.0);}
        s.b[1104] = (s.v[165] > s.v[157]);s.store_scalar(1104, if s.b[1104] { 1.0 } else { 0.0 });
        if (((((!s.b[733]) && s.b[1085]) && (!s.b[1100])) && (!s.b[1103])) && s.b[1104]) {s.copy_ad(165, 157);}
        if (((!s.b[733]) && s.b[1085]) && (!s.b[1100])) {s.copy_ad(164, 165);s.store_add(162, 161, 164);}
        let (t66,) = {
    if (((!s.b[733]) && s.b[1085]) && (!s.b[1100])) {
        (0.0,)
    } else {
        (s.v[430],)
    }
};
        s.store_scalar(430, t66);
        if ((!s.b[733]) && s.b[1085]) {s.copy_ad(352, 162);s.store_scalar(168, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_43(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
    ) {
        let mut t6a: usize = 0;
        while {
            let t68: f64 = (s.v[58] + 1.0);let t69: f64 = if (((!s.b[733]) && s.b[1085]) && (s.v[168] <= t68)) { 1.0 } else { 0.0 };
            t69 != 0.0
        } {
            t6a += 1;
            if t6a > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", t6a, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((!s.b[733]) && s.b[1085]) {s.store_sub(418, 352, 515);s.store_mul(181, 225, 418);s.store_mul_sub_rhs(337, 420, 418, 419);}
            s.b[1105] = (s.v[337] < 80.0);s.store_scalar(1105, if s.b[1105] { 1.0 } else { 0.0 });
            if (((!s.b[733]) && s.b[1085]) && s.b[1105]) {s.store_exp(328, 337);s.store_exp_mul_scaled_lhs_indices(327, 420, -1.0, 419);s.store_sub(329, 328, 327);s.store_div_ln_offset_lhs(422, 329, 1.0, 420);s.store_div_scaled_value_offset_denominator(423, s.ad_value(328), 1.0, s.ad_value(329), 1.0, 1.0);}
            if (((!s.b[733]) && s.b[1085]) && (!s.b[1105])) {s.store_sub(422, 418, 419);s.store_scalar(423, 1.0);}
            if ((!s.b[733]) && s.b[1085]) {s.store_mul(421, 225, 422);}
            s.b[1106] = (((s.v[181]) as f64).abs() < 1e-16);s.store_scalar(1106, if s.b[1106] { 1.0 } else { 0.0 });
            if (((!s.b[733]) && s.b[1085]) && s.b[1106]) {s.store_sqrt_scaled_input_ad(327, A::sub_from_scalar(1.0, A::square(s.ad_value(423))), 1.0 / (2.0));s.store_mul(242, 181, 327);s.store_mul(443, 225, 327);}
            s.b[1107] = (s.v[181] < 0.0);s.store_scalar(1107, if s.b[1107] { 1.0 } else { 0.0 });
            if ((((!s.b[733]) && s.b[1085]) && s.b[1106]) && s.b[1107]) {s.store_neg(242, 242);s.store_neg(443, 443);}
            s.b[1108] = (((s.v[181]) as f64).abs() < 0.005);s.store_scalar(1108, if s.b[1108] { 1.0 } else { 0.0 });
            if ((((!s.b[733]) && s.b[1085]) && (!s.b[1106])) && s.b[1108]) {s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(327, 181, 1.0, 181, 1.0, 181, 1.0, 181, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));s.store_mul_scale_offset_mixed_ia(328, 181, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(181), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(181), 1.0, A::scale(s.ad_value(181), 0.25), 1.0 / (3.0)), 1.0 / (2.0)), -1.0, 1.0);s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(329, 421, 1.0, 421, 1.0, 421, 1.0, 421, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));s.store_mul_scale_offset_mixed_ia(330, 421, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(421), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(421), 1.0, A::scale(s.ad_value(421), 0.25), 1.0 / (3.0)), 1.0 / (2.0)), -1.0, 1.0);s.store_sqrt_sub(242, 327, 329);s.store_div_scaled_product_mixed_iai(443, 225, A::add_scaled_product(s.ad_value(328), 1.0, s.ad_value(423), s.ad_value(330), (-1.0)), 0.5, 242, 1.0);}
            if ((((!s.b[733]) && s.b[1085]) && (!s.b[1106])) && (!s.b[1108])) {s.store_exp_neg_input(327, 181);s.store_exp_neg_input(328, 421);s.store_sqrt_ad(242, A::add_scaled_inputs4(s.ad_value(181), 1.0, s.ad_value(421), (-1.0), s.ad_value(327), 1.0, s.ad_value(328), (-1.0)));s.store_div_scaled_product_mixed_iai(443, 225, A::sub(A::sub_from_scalar(1.0, s.ad_value(327)), A::mul_sub_from_scalar_rhs(s.ad_value(423), 1.0, s.ad_value(328))), 0.5, 242, 1.0);}
            s.b[1109] = (s.v[338] == (-1.0));s.store_scalar(1109, if s.b[1109] { 1.0 } else { 0.0 });
            if (((!s.b[733]) && s.b[1085]) && s.b[1109]) {s.store_scalar(401, 0.0);}
            if (((!s.b[733]) && s.b[1085]) && (!s.b[1109])) {s.store_mul(401, 444, 242);}
            if ((!s.b[733]) && s.b[1085]) {s.store_mul(370, 229, 401);}
            s.b[1110] = (s.v[181] < 0.0);s.store_scalar(1110, if s.b[1110] { 1.0 } else { 0.0 });
            if (((!s.b[733]) && s.b[1085]) && s.b[1110]) {s.store_neg(499, 242);s.store_neg(500, 443);}
            s.b[1111] = (s.v[181] < 1e-7);s.store_scalar(1111, if s.b[1111] { 1.0 } else { 0.0 });
            if ((((!s.b[733]) && s.b[1085]) && (!s.b[1110])) && s.b[1111]) {s.copy_ad(499, 242);s.copy_ad(500, 443);}
            if ((((!s.b[733]) && s.b[1085]) && (!s.b[1110])) && (!s.b[1111])) {s.store_mul_sub_rhs(501, 225, 352, 157);s.store_exp(502, 501);s.store_mul_mixed_ia(497, 379, A::add_scaled_offset_product_rhs(s.ad_value(502), 1.0, s.ad_value(484), s.ad_value(181), 1.0, (-1.0)));s.store_mul_ad_product_rhs_mixed_ia(498, 379, 225, A::sub(s.ad_value(502), s.ad_value(484)));s.store_sqrt_square_add(499, 242, 497);s.store_div_scaled_add_product_indices(500, 498, 0.5, 443, 242, (2.0 * 0.5), 499, 1.0);}
            if ((!s.b[733]) && s.b[1085]) {s.store_add_scaled_inputs_products_indices(503, 352, 1.0, 159, (-1.0), 240, 499, 1.0, 324, 393, (-1.0));s.store_offset_mul(504, 240, 500, 1.0);}
            s.b[1112] = ((s.v[430] == 1.0) && (s.v[168] > 3.0));s.store_scalar(1112, if s.b[1112] { 1.0 } else { 0.0 });
            if (((!s.b[733]) && s.b[1085]) && s.b[1112]) {s.store_scalar(168, (s.v[58] + 1.0));}
            if (((!s.b[733]) && s.b[1085]) && (!s.b[1112])) {s.store_div_scaled_inputs_indices(495, 503, -1.0, 504, 1.0);}
            if (((!s.b[733]) && s.b[1085]) && (!s.b[1112])) {
                s.store_scaled_offset_ad(496, {
                    if (1.0 >= ((s.v[352]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(352))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[1113] = (((s.v[495]) as f64).abs() > s.v[496]);s.store_scalar(1113, if s.b[1113] { 1.0 } else { 0.0 });
            if ((((!s.b[733]) && s.b[1085]) && (!s.b[1112])) && s.b[1113]) {s.store_scale(495, 496, (if (s.v[495] >= 0.0) { 1.0 } else { (-1.0) }));}
            if (((!s.b[733]) && s.b[1085]) && (!s.b[1112])) {s.store_add(352, 352, 495);}
            s.b[1114] = ((((s.v[495]) as f64).abs() <= 5e-12) && (((s.v[503]) as f64).abs() <= 1e-8));s.store_scalar(1114, if s.b[1114] { 1.0 } else { 0.0 });
            let (t67,) = {
    if ((((!s.b[733]) && s.b[1085]) && (!s.b[1112])) && s.b[1114]) {
        (1.0,)
    } else {
        (s.v[430],)
    }
};
            s.store_scalar(430, t67);
            if ((!s.b[733]) && s.b[1085]) {s.store_primal_offset(168, 168, 1.0);}
        }
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_44(
        s: &mut Scratch,
    ) {
        if ((!s.b[733]) && s.b[1085]) {s.store_primal_offset(168, 168, (-1.0));s.copy_ad(372, 370);s.copy_ad(359, 372);s.copy_ad(162, 352);s.store_div(569, 372, 238);s.store_offset(171, 569, (10.0 * 2.220446049250313e-16));s.store_div_from_scalar_add_ad(328, 1.0, s.ad_value(499), s.ad_value(171));s.store_mul3_lhs(358, 238, 497, 328);s.store_neg(358, 358);s.store_sub(164, 162, 161);s.copy_ad(157, 453);s.store_div(328, 225, 169);s.store_mul(505, 328, 164);s.store_offset(506, 505, 1.0);s.store_sqrt(507, 506);s.store_div_from_scalar_offset_input(508, 1.0, 507, 1.0);s.store_div(509, 508, 170);s.store_scaled_add(510, 568, 569, 0.5);s.store_add_scaled_inputs4_indices(328, 159, 1.0, 227, 1.0, 161, (-(2.0 * 0.5)), 164, (-0.5));s.store_sub(329, 509, 510);s.store_mul(330, 225, 323);s.store_mul(331, 225, 238);s.store_add_scaled_products_indices(511, 330, 328, 1.0, 331, 329, 1.0);s.store_scaled_add(424, 359, 356, 0.5);s.store_scaled_add(425, 358, 355, (-0.5));s.store_sub(426, 359, 356);s.store_sub(427, 355, 358);s.store_square(428, 238);}
        s.b[1115] = (s.v[339] <= 1.0);s.store_scalar(1115, if s.b[1115] { 1.0 } else { 0.0 });
        if (((!s.b[733]) && s.b[1085]) && s.b[1115]) {s.store_add_scaled_inputs3_mixed_aia(246, A::mul3(s.ad_value(425), s.ad_value(225), s.ad_value(164)), 1.0, 427, (-1.0), A::div_scaled_product(A::square(s.ad_value(426)), s.ad_value(426), 0.16666666666666666, s.ad_value(428), 1.0), -1.0);}
        if (((!s.b[733]) && s.b[1085]) && (!s.b[1115])) {s.store_mul(246, 164, 511);}
        s.b[1116] = ((s.v[84] >= 1.0) && (s.v[246] < 0.0));s.store_scalar(1116, if s.b[1116] { 1.0 } else { 0.0 });
        if (((!s.b[733]) && s.b[1085]) && s.b[1116]) {s.store_scalar(246, 0.0);}
        s.b[1117] = (s.v[339] <= 1.0);s.store_scalar(1117, if s.b[1117] { 1.0 } else { 0.0 });s.b[1118] = (((s.v[164]) as f64).abs() > 1e-6);s.store_scalar(1118, if s.b[1118] { 1.0 } else { 0.0 });
        if ((((!s.b[733]) && s.b[1085]) && s.b[1117]) && s.b[1118]) {s.store_add_scaled_product_mixed_aia(437, A::div_scaled_product(A::mul3(A::add_scaled_inputs_product(s.ad_value(425), 1.0, s.ad_value(424), (-2.0), A::div(s.ad_value(323), s.ad_value(225)), A::add(A::sub_from_scalar(1.0, A::div_scaled_product(s.ad_value(424), s.ad_value(424), 2.0, s.ad_value(428), 1.0)), A::div_scaled_product(s.ad_value(426), s.ad_value(426), 0.1, s.ad_value(428), 1.0)), 1.0), s.ad_value(426), s.ad_value(426)), s.ad_value(426), 0.16666666666666666, s.ad_value(428), 1.0), 1.0, 424, A::sub(A::mul3(s.ad_value(425), s.ad_value(225), s.ad_value(164)), s.ad_value(427)), 1.0);s.store_div(437, 437, 246);}
        if ((((!s.b[733]) && s.b[1085]) && s.b[1117]) && (!s.b[1118])) {s.copy_ad(437, 424);}
        if (((!s.b[733]) && s.b[1085]) && (!s.b[1117])) {s.store_scaled_add(437, 359, 356, 0.5);}
        if ((!s.b[733]) && s.b[1085]) {s.store_scale(328, 240, 2.0);s.store_mul_sub_rhs(512, 328, 510, 170);s.store_add(191, 164, 512);s.store_div_from_scalar(328, 1.0, 192);s.store_mul(329, 191, 328);s.store_sub_from_scalar(330, 1.0, 329);s.store_sub_from_scalar(336, 1.0, 330);s.store_square(49, 336);s.store_scalar(50, 1.0);s.store_scalar(51, 1.0);s.store_scalar(52, 1.0);}
        let (t6b,) = {
    if ((!s.b[733]) && s.b[1085]) {
        (0.0,)
    } else {
        (s.v[54],)
    }
};
        s.store_scalar(54, t6b);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_45(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
    ) {
        let (t6c,) = {
    if ((!s.b[733]) && s.b[1085]) {
        (0.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t6c);
        if ((!s.b[733]) && s.b[1085]) {s.store_scalar(48, 0.0);s.store_scalar(53, 0.0);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_add(48, 51, 52);s.copy_ad(53, 48);}
        s.b[1119] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));s.store_scalar(1119, if s.b[1119] { 1.0 } else { 0.0 });s.b[1120] = (4.0 == 1.0);s.store_scalar(1120, if s.b[1120] { 1.0 } else { 0.0 });
        let (t6d,) = {
    if ((((!s.b[733]) && s.b[1085]) && s.b[1119]) && s.b[1120]) {
        (1.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t6d);s.b[1121] = (4.0 == 2.0);s.store_scalar(1121, if s.b[1121] { 1.0 } else { 0.0 });
        let (t6e,) = {
    if (((((!s.b[733]) && s.b[1085]) && s.b[1119]) && (!s.b[1120])) && s.b[1121]) {
        (2.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t6e);s.b[1122] = (4.0 == 4.0);s.store_scalar(1122, if s.b[1122] { 1.0 } else { 0.0 });
        let (t6f,) = {
    if ((((((!s.b[733]) && s.b[1085]) && s.b[1119]) && (!s.b[1120])) && (!s.b[1121])) && s.b[1122]) {
        (3.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t6f);s.b[1123] = (4.0 == 8.0);s.store_scalar(1123, if s.b[1123] { 1.0 } else { 0.0 });
        let (t70,) = {
    if (((((((!s.b[733]) && s.b[1085]) && s.b[1119]) && (!s.b[1120])) && (!s.b[1121])) && (!s.b[1122])) && s.b[1123]) {
        (4.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t70);
        let (t71,) = {
    if (((!s.b[733]) && s.b[1085]) && s.b[1119]) {
        (0.0,)
    } else {
        (s.v[54],)
    }
};
        s.store_scalar(54, t71);let mut t75: usize = 0;
        while {
            let t74: f64 = if ((((!s.b[733]) && s.b[1085]) && s.b[1119]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            t74 != 0.0
        } {
            t75 += 1;
            if t75 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", t75, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if (((!s.b[733]) && s.b[1085]) && s.b[1119]) {s.store_sqrt(53, 53);}
            let (t73,) = {
    if (((!s.b[733]) && s.b[1085]) && s.b[1119]) {
        let t72: f64 = (s.v[54] + 1.0);
        (t72,)
    } else {
        (s.v[54],)
    }
};
            s.store_scalar(54, t73);
        }
        if (((!s.b[733]) && s.b[1085]) && (!s.b[1119])) {s.store_powf(53, 53, (1.0 / (2.0 * 4.0)));}
        if ((!s.b[733]) && s.b[1085]) {s.store_div_from_scalar(53, 1.0, 53);s.store_scaled_mul(337, 336, 53, 1.0);s.store_sub_from_scalar(190, 1.0, 337);s.store_offset_mul_offset_rhs(478, 190, 190, 1.0, 1.0);}
        if ((!s.b[733]) && s.b[1085]) {
            if ((1.0 + s.v[190]) >= (10.0 * 2.220446049250313e-16)) {
                s.store_offset(479, 190, 1.0);
            } else {
                s.store_scalar(479, (10.0 * 2.220446049250313e-16));
            }
        }
        if ((!s.b[733]) && s.b[1085]) {s.store_div_scaled_product_indices(328, 192, 478, 0.6666666666666667, 479, 1.0);}
        s.b[1124] = (s.v[339] <= 1.0);s.store_scalar(1124, if s.b[1124] { 1.0 } else { 0.0 });s.b[1125] = (((s.v[164]) as f64).abs() > 1e-6);s.store_scalar(1125, if s.b[1125] { 1.0 } else { 0.0 });
        if ((((!s.b[733]) && s.b[1085]) && s.b[1124]) && s.b[1125]) {s.store_sub_ad(436, A::add_scaled_product(A::mul3(A::add_scaled_inputs(A::square(s.ad_value(425)), 1.0, A::square(s.ad_value(427)), 0.08333333333333333), s.ad_value(225), s.ad_value(164)), 1.0, s.ad_value(425), s.ad_value(427), (-1.0)), A::div_scaled_product(A::mul3(A::add_scaled_inputs(s.ad_value(425), 2.0, A::div_scaled_product3_by_product(s.ad_value(323), s.ad_value(426), s.ad_value(426), 0.2, s.ad_value(225), s.ad_value(428), 1.0), 1.0), s.ad_value(426), s.ad_value(426)), s.ad_value(426), 0.16666666666666666, s.ad_value(428), 1.0));s.store_div(436, 436, 246);}
        if ((((!s.b[733]) && s.b[1085]) && s.b[1124]) && (!s.b[1125])) {s.copy_ad(436, 425);}
        if (((!s.b[733]) && s.b[1085]) && (!s.b[1124])) {s.store_scaled_add(436, 355, 358, (-0.5));}
        s.b[1129] = (s.v[612] == 0.0);s.store_scalar(1129, if s.b[1129] { 1.0 } else { 0.0 });
        if s.b[1129] {s.store_offset(480, 190, 0.5);s.store_mul(481, 479, 478);s.store_div_scaled_inputs_indices(482, 480, 0.4, 481, 1.0);s.store_sub_from_scalar(438, 0.6, 482);}
        s.b[1130] = (s.v[438] > (0.5 + 1e-8));s.store_scalar(1130, if s.b[1130] { 1.0 } else { 0.0 });
        if (s.b[1129] && s.b[1130]) {s.store_scalar(438, 0.5);}
        if s.b[1129] {s.copy_ad(439, 438);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_46(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[1129] {s.store_scalar(438, 0.5);}
        s.b[1132] = (s.v[145] == 0.0);s.store_scalar(1132, if s.b[1132] { 1.0 } else { 0.0 });s.b[1148] = ((p[190] < (10.0 * 2.220446049250313e-16)) && (p[191] < (10.0 * 2.220446049250313e-16)));s.store_scalar(1148, if s.b[1148] { 1.0 } else { 0.0 });
        if ((s.b[1129] && s.b[1132]) && s.b[1148]) {s.store_scalar(316, 0.0);s.copy_ad(314, 162);}
        s.b[1149] = (s.v[314] > ((s.v[161] + s.v[173]) - (10.0 * 2.220446049250313e-16)));s.store_scalar(1149, if s.b[1149] { 1.0 } else { 0.0 });
        if (((s.b[1129] && s.b[1132]) && s.b[1148]) && s.b[1149]) {s.store_offset_add(314, 161, 173, (-(10.0 * 2.220446049250313e-16)));}
        if ((s.b[1129] && s.b[1132]) && (!s.b[1148])) {s.store_scalar(1147, (if (p[43] == 1.0) { p[237] } else { s.v[402] }));}
        if ((s.b[1129] && s.b[1132]) && (!s.b[1148])) {s.store_primal_div_from_scalar(1133, 1.0, 1147);s.store_mul(1134, 244, 1133);s.store_scale(1135, 1134, p[191]);s.store_add_scaled_product_indices(1138, 1135, 1.0, 80, 229, 1.0);s.store_div_from_scalar(1134, 1.0, 1138);s.store_scale(1137, 1134, 1.034943e-10);s.store_scalar(1134, (1.0 - p[189]));s.store_add_scaled_inputs_product_indices(314, 157, p[189], 161, p[189], 1134, 162, 1.0);}
        s.b[1150] = (s.v[314] > ((s.v[161] + s.v[173]) - (10.0 * 2.220446049250313e-16)));s.store_scalar(1150, if s.b[1150] { 1.0 } else { 0.0 });
        if (((s.b[1129] && s.b[1132]) && (!s.b[1148])) && s.b[1150]) {s.store_offset_add(314, 161, 173, (-(10.0 * 2.220446049250313e-16)));}
        if ((s.b[1129] && s.b[1132]) && (!s.b[1148])) {s.store_sub(1140, 314, 162);s.store_sqrt_square_offset(44, 1140, ((4.0 * 0.001) * 0.001));s.store_offset_add_scaled_inputs_indices(1139, 1140, 0.5, 44, 0.5, (1e-10 * 0.001));}
        s.b[1151] = (s.v[1139] < 0.0);s.store_scalar(1151, if s.b[1151] { 1.0 } else { 0.0 });
        if (((s.b[1129] && s.b[1132]) && (!s.b[1148])) && s.b[1151]) {s.store_scalar(1139, 0.0);}
        if ((s.b[1129] && s.b[1132]) && (!s.b[1148])) {s.store_mul(1136, 225, 244);s.store_div_from_scalar(1134, 1.0, 1136);s.store_mul(1138, 246, 1134);}
        s.b[1152] = (s.v[1138] < s.v[227]);s.store_scalar(1152, if s.b[1152] { 1.0 } else { 0.0 });
        if (((s.b[1129] && s.b[1132]) && (!s.b[1148])) && s.b[1152]) {s.copy_ad(1138, 227);}
        if ((s.b[1129] && s.b[1132]) && (!s.b[1148])) {s.store_scale(1144, 229, 9662367879.197212);s.store_scalar(1134, (100000.0 * 10000.0));s.store_scalar(1135, (1.0 / s.v[97]));s.store_mul_mixed_ai(1146, A::add_scaled_inputs_product(s.ad_value(1138), 2.0, A::mul3_scaled_output(s.ad_value(1144), s.ad_value(1139), s.ad_value(1137), 2.0), 1.0, s.ad_value(1134), s.ad_value(1137), 1.0), 1135);s.store_mul(1141, 1146, 1137);s.store_add_scaled_product_indices(1145, 1134, 4.0, 1144, 1139, (2.0 * 4.0));s.store_mul3_lhs(1142, 1145, 1137, 1137);s.store_sqrt_square_add(1143, 1141, 1142);s.store_mul_sub_scaled_inputs_rhs_indices(316, 326, 1143, 0.5, 1141, 0.5);}
        if (s.b[1129] && s.b[1132]) {s.store_scale(316, 316, s.v[127]);}
        if s.b[1129] {s.store_sub_from_scalar(441, s.v[97], 316);}
        s.b[1153] = (s.v[441] < 1e-9);s.store_scalar(1153, if s.b[1153] { 1.0 } else { 0.0 });
        if (s.b[1129] && s.b[1153]) {s.store_scalar(441, 1e-9);}
        if s.b[1129] {s.store_scale(328, 108, (-s.v[98]));s.store_mul(196, 328, 437);s.store_mul(197, 328, 436);s.store_mul(198, 197, 438);}
        s.b[1154] = (p[43] == 0.0);s.store_scalar(1154, if s.b[1154] { 1.0 } else { 0.0 });
        if (s.b[1129] && s.b[1154]) {s.store_scale(477, 196, 0.5);s.store_scale(476, 196, (1.0 - 0.5));s.store_mul_scale_offset_mixed_ia(392, 108, A::add(s.ad_value(357), s.ad_value(360)), (0.5 * s.v[98]), 0.0);}
        if s.b[1129] {s.store_scaled_sub(1155, 157, 164, 0.5);s.store_scale(44, 1155, (2.0 * 1.0 / (p[227])));s.store_offset_mul_offset_rhs_mixed_ia(45, 44, A::mul_offset_rhs(s.ad_value(44), A::mul_offset_rhs(s.ad_value(44), A::mul_offset_rhs(s.ad_value(44), A::mul(s.ad_value(44), A::scale_offset(s.ad_value(44), (1.0 / 5040.0), (1.0 / 720.0))), (1.0 / 120.0)), (1.0 / 24.0)), (1.0 / 6.0)), (1.0 / 2.0), 1.0);s.store_div_from_scalar(177, p[227], 45);}
        s.b[1156] = (s.v[177] < (10.0 * 2.220446049250313e-16));s.store_scalar(1156, if s.b[1156] { 1.0 } else { 0.0 });
        if (s.b[1129] && s.b[1156]) {s.store_scalar(177, (10.0 * 2.220446049250313e-16));}
        if s.b[1129] {s.store_add(176, 161, 177);s.store_scalar(1166, (1.034943e-10 / 100.0));s.store_scale(1167, 437, 0.0001);s.store_scale(1168, 436, 0.0001);s.store_div_from_scalar(1157, p[92], 1166);s.store_primal_div_from_scalar(1158, p[93], 1166);s.store_scalar(1159, p[94]);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_47(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[1129] {s.store_offset_mul_ad(1160, A::sub(s.ad_value(162), s.ad_value(161)), s.ad_value(1159), 1.0);s.store_add_scaled_products_indices(1161, 1157, 1167, 1.0, 1158, 1168, 1.0);s.store_div(1162, 1161, 1160);s.copy_ad(248, 1162);s.store_sqrt_square_offset(44, 248, ((4.0 * 3000.0) * 3000.0));s.store_offset_add_scaled_inputs_indices(1159, 248, 0.5, 44, 0.5, (1e-10 * 3000.0));}
        s.b[1169] = (s.v[1159] < 0.0);s.store_scalar(1169, if s.b[1169] { 1.0 } else { 0.0 });
        if (s.b[1129] && s.b[1169]) {s.store_scalar(1159, 0.0);}
        if s.b[1129] {s.store_powf(1161, 1159, (p[97] - 1.0));s.store_mul(1163, 1161, 1159);s.store_powf(1164, 1159, (s.v[111] - 1.0));s.store_mul(1165, 1164, 1159);s.store_scale(249, 1168, 6.241449993689894e18);s.store_add_scaled_inputs_mixed_ai(1157, A::add_scaled_product(A::div_from_scalar(1.0, A::scale_offset(s.ad_value(249), (p[96] * 1e-11), p[95])), 1.0, s.ad_value(543), s.ad_value(1163), 1.0), 1.0, 1165, 1.0 / (p[106]));s.store_div_from_scalar(251, 1.0, 1157);s.store_scale(251, 251, 0.0001);s.store_mul3_lhs(1170, 225, 244, 441);s.store_sqrt_square_offset(44, 1170, ((4.0 * 1e-50) * 1e-50));s.store_offset_add_scaled_inputs_indices(1170, 1170, 0.5, 44, 0.5, (1e-10 * 1e-50));}
        s.b[1178] = (s.v[1170] < 0.0);s.store_scalar(1178, if s.b[1178] { 1.0 } else { 0.0 });
        if (s.b[1129] && s.b[1178]) {s.store_scalar(1170, 0.0);}
        if s.b[1129] {s.store_div_from_scalar(1171, 1.0, 1170);s.store_mul(1172, 246, 1171);s.store_div_scaled_inputs_indices(1170, 253, 0.2, 251, 1.0);s.store_sqrt_square_sum(252, 1172, 1170);s.store_mul(1173, 251, 252);s.store_div(1171, 1173, 253);}
        s.b[1179] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= p[113]) && (p[113] <= (1.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(1179, if s.b[1179] { 1.0 } else { 0.0 });
        if (s.b[1129] && s.b[1179]) {s.store_scalar(1174, 1.0);}
        s.b[1180] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= p[113]) && (p[113] <= (2.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(1180, if s.b[1180] { 1.0 } else { 0.0 });
        if ((s.b[1129] && (!s.b[1179])) && s.b[1180]) {s.copy_ad(1174, 1171);}
        if ((s.b[1129] && (!s.b[1179])) && (!s.b[1180])) {s.store_powf(1174, 1171, (p[113] - 1.0));}
        if s.b[1129] {s.store_mul(1170, 1171, 1174);s.store_offset(1175, 1170, 1.0);}
        s.b[1181] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= p[113]) && (p[113] <= (1.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(1181, if s.b[1181] { 1.0 } else { 0.0 });
        if (s.b[1129] && s.b[1181]) {s.store_div_from_scalar(1176, 1.0, 1175);}
        s.b[1182] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= p[113]) && (p[113] <= (2.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(1182, if s.b[1182] { 1.0 } else { 0.0 });
        if ((s.b[1129] && (!s.b[1181])) && s.b[1182]) {s.store_div_from_scalar_sqrt_ad(1176, 1.0, s.ad_value(1175));}
        if ((s.b[1129] && (!s.b[1181])) && (!s.b[1182])) {s.store_powf(1177, 1175, (((-1.0) / p[113]) - 1.0));s.store_mul(1176, 1175, 1177);}
        if s.b[1129] {s.store_mul(250, 251, 1176);s.store_div_scaled_product_mixed_iia(264, 107, 227, 1.0, A::sub_from_scalar(s.v[97], s.ad_value(316)), 1.0);s.store_mul3_lhs(200, 264, 246, 250);s.store_scalar(201, 0.0);}
        s.b[1192] = ((p[281] > 0.0) && (p[244] != 0.0));s.store_scalar(1192, if s.b[1192] { 1.0 } else { 0.0 });
        if (s.b[1129] && s.b[1192]) {s.store_scaled_sub(1183, 157, 164, 0.5);s.store_scale(44, 1183, (2.0 * 100.0));s.store_offset_mul_offset_rhs_mixed_ia(45, 44, A::mul_offset_rhs(s.ad_value(44), A::mul_offset_rhs(s.ad_value(44), A::mul_offset_rhs(s.ad_value(44), A::mul(s.ad_value(44), A::scale_offset(s.ad_value(44), (1.0 / 5040.0), (1.0 / 720.0))), (1.0 / 120.0)), (1.0 / 24.0)), (1.0 / 6.0)), (1.0 / 2.0), 1.0);s.store_div_from_scalar(1189, 0.01, 45);s.store_sub_from_scalar_ad(1183, 1.1, A::add(s.ad_value(161), s.ad_value(1189)));s.store_sqrt_square_offset(44, 1183, ((4.0 * 0.05) * 0.05));s.store_offset_add_scaled_inputs_indices(1191, 1183, 0.5, 44, 0.5, (1e-10 * 0.05));}
        s.b[1193] = (s.v[1191] < 0.0);s.store_scalar(1193, if s.b[1193] { 1.0 } else { 0.0 });
        if ((s.b[1129] && s.b[1192]) && s.b[1193]) {s.store_scalar(1191, 0.0);}
        if (s.b[1129] && s.b[1192]) {s.store_scale(1184, 225, s.v[116]);s.store_mul(1185, 323, 1184);s.store_powf(1184, 1191, p[245]);s.store_mul(1186, 1185, 1184);s.store_offset_scaled(1187, 173, p[246], 1.0);s.store_scalar(1184, s.v[117]);}
        s.b[1194] = ((s.v[56] < 3.0) || (p[43] == 1.0));s.store_scalar(1194, if s.b[1194] { 1.0 } else { 0.0 });
    }
}
