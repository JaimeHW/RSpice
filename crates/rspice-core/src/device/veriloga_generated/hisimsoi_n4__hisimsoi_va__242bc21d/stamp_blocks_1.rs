#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_transient_block_16(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (!s.b[729]) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }
        if (!s.b[729]) {s.store_sqrt_square_add(45, 44, 45);s.store_offset_add_scaled_inputs_indices(320, 44, (-0.5), 45, (-0.5), 1.0);}
        s.store_add_scaled_inputs3_offset_indices(159, 158, 1.0, 185, 1.0, 320, -1.0, (-s.v[123]));s.copy_ad(178, 159);s.store_ln_scaled_input(328, 544, 1.0 / (s.v[66]));s.store_mul(342, 227, 328);let t3a: f64 = (s.v[123] - s.v[185]);let t3b: f64 = (t3a + s.v[320]);s.store_scalar(160, t3b);s.store_mul(240, 238, 324);s.store_square(241, 240);s.b[733] = (p[43] == 0.0);s.store_scalar(733, if s.b[733] { 1.0 } else { 0.0 });
        if s.b[733] {s.store_scalar(738, 7.0);s.store_offset(399, 231, 1.0);s.store_div_scalar_by_product_indices(328, 1.0, 379, 434, 1.0);s.store_mul_ad_product_rhs(329, 328, A::offset(s.ad_value(399), (-s.v[383])), A::offset(s.ad_value(399), (-s.v[383])));s.store_add_mixed_ia(330, 225, A::div_scalar_offset_denominator(2.0, s.ad_value(399), (-s.v[383]), 1.0));s.store_div_ln_lhs(180, 329, 330);s.store_sqrt_mul(403, 547, 180);}
        if s.b[733] {
            if (s.v[403] > p[237]) {
                s.store_scalar(403, p[237]);
            } else {
            }
        }
        if s.b[733] {s.store_scaled_mul(406, 544, 403, (-1.6021918e-19));s.store_scalar(736, p[237]);s.store_scaled_mul(341, 544, 736, (-1.6021918e-19));s.store_scalar(737, 1.5);s.store_primal_div_from_scalar(734, 1.034943e-10, 736);s.store_primal_div_from_scalar(735, 1.0, 734);s.store_scale(739, 341, (-0.001));s.store_scale(740, 341, (-1e-5));}
        if (s.b[733] && (p[39] != 0.0)) {s.store_add(475, 172, 342);}
        if (s.b[733] && (p[39] == 0.0)) {s.store_add(475, 156, 342);}
        let (t40,) = {
    if s.b[733] {
        let t3c: f64 = (2.0 / s.v[225]);let t3d: f64 = (s.v[66] / s.v[230]);let t3e: f64 = (t3d).ln();let t3f: f64 = (t3c * t3e);
        (t3f,)
    } else {
        (s.v[382],)
    }
};
        s.store_scalar(382, t40);
        if s.b[733] {s.store_scaled_square(741, 474, (s.v[95] * s.v[95]));s.store_neg(742, 475);s.store_add_scaled_inputs3_mixed_aai(743, A::square(A::add_scaled_product(s.ad_value(742), 2.0, s.ad_value(741), s.ad_value(225), 1.0)), 1.0, A::square(s.ad_value(742)), (-4.0), 741, (-4.0));}
        if s.b[733] {
            if (s.v[743] >= (10.0 * 2.220446049250313e-16)) {
            } else {
                s.store_scalar(743, (10.0 * 2.220446049250313e-16));
            }
        }
        if s.b[733] {s.store_sqrt(743, 743);s.store_add_scaled_product_indices(744, 742, 2.0, 741, 225, 1.0);s.store_scaled_sub(745, 744, 743, 0.5);s.store_div_ad(746, A::ln(A::div_scaled_product_by_product(s.ad_value(742), s.ad_value(742), 1.0, s.ad_value(741), s.ad_value(239), 1.0)), A::add(s.ad_value(225), A::div_from_scalar(2.0, s.ad_value(742))));}
        s.b[747] = (s.v[745] < s.v[382]);s.store_scalar(747, if s.b[747] { 1.0 } else { 0.0 });
        if (s.b[733] && s.b[747]) {s.copy_ad(387, 745);}
        if (s.b[733] && (!s.b[747])) {s.store_offset_sub(44, 746, 745, (-0.0008));s.store_scale(45, 746, (4.0 * 0.0008));}
        if (s.b[733] && (!s.b[747])) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }
        if (s.b[733] && (!s.b[747])) {s.store_sqrt_square_add(45, 44, 45);s.store_add_scaled_inputs3_indices(387, 746, 1.0, 44, (-0.5), 45, (-0.5));}
        if s.b[733] {s.store_scalar(167, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_17(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
    ) {
        let mut t43: usize = 0;
        while {
            let t42: f64 = if (s.b[733] && (s.v[167] < s.v[57])) { 1.0 } else { 0.0 };
            t42 != 0.0
        } {
            t43 += 1;
            if t43 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", t43, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if s.b[733] {s.copy_ad(748, 474);s.store_mul(749, 225, 387);s.store_exp_neg_input(750, 749);}
            s.b[756] = (s.v[387] > 1e-9);s.store_scalar(756, if s.b[756] { 1.0 } else { 0.0 });
            if (s.b[733] && s.b[756]) {s.store_exp_mul(751, 225, 387);s.store_mul_scaled_sqrt_ad_rhs(752, 748, -1.0, A::add_scaled_offset_product_rhs(A::offset(A::add(s.ad_value(750), s.ad_value(749)), (-1.0)), 1.0, s.ad_value(239), s.ad_value(751), (-1.0), 1.0));s.store_mul_div_from_scalar_lhs_ad_mixed_ia(753, s.v[122], 752, A::add_scaled_sub_value_product(1.0, s.ad_value(750), 1.0, s.ad_value(239), s.ad_value(751), 1.0));}
            s.b[757] = (s.v[387] < (-1e-9));s.store_scalar(757, if s.b[757] { 1.0 } else { 0.0 });
            if ((s.b[733] && (!s.b[756])) && s.b[757]) {s.store_mul_sqrt_mixed_ia(752, 748, A::offset(A::add(s.ad_value(750), s.ad_value(749)), (-1.0)));s.store_mul_scale_offset_mixed_ai(753, A::div_from_scalar(s.v[122], s.ad_value(752)), 750, -1.0, 1.0);}
            if ((s.b[733] && (!s.b[756])) && (!s.b[757])) {s.store_mul_ad_affine_product_lhs(752, A::sqrt(A::div_from_scalar(s.v[122], s.ad_value(225))), s.ad_value(225), -1.0, 0.0, 387);s.store_scaled_sqrt_scaled_input(753, 225, s.v[122], -1.0);}
            if s.b[733] {s.store_sqrt_add_scaled_square_product(45, 752, 1.0, 739, 739, 4.0);s.store_offset_scaled_div(755, 752, 45, 0.5, 0.5);s.store_add_scaled_inputs3_indices(754, 752, 0.5, 45, 0.5, 739, 1e-10);}
            s.b[758] = (s.v[754] < 0.0);s.store_scalar(758, if s.b[758] { 1.0 } else { 0.0 });
            if (s.b[733] && s.b[758]) {s.store_scalar(754, 0.0);s.store_scalar(755, 0.0);}
            if s.b[733] {s.store_add_scaled_inputs3_indices(44, 341, -1.0, 754, (-1.0), 740, -1.0);s.store_scaled_mul(45, 341, 740, (-4.0));}
            if s.b[733] {
                if (s.v[45] > 0.0) {
                } else {
                    s.store_neg(45, 45);
                }
            }
            if s.b[733] {s.store_sqrt_square_add(45, 44, 45);s.store_offset_scaled_div(335, 44, 45, 0.5, 0.5);s.store_add_scaled_inputs3_indices(754, 341, -1.0, 44, (-0.5), 45, (-0.5));s.store_mul3_lhs(755, 755, 753, 335);s.store_div_scaled_inputs_mixed_ai(390, A::square(s.ad_value(754)), ((0.5 * 9662367879.197212) * 6.241449993689894e18), 544, 1.0);s.store_div_scaled_product_indices(391, 390, 755, 2.0, 754, 1.0);s.store_sub_mixed_ia(754, 387, A::div_scaled_inputs4(s.ad_value(752), 1.0 / (s.v[93]), s.ad_value(387), (-1.0), s.ad_value(475), -1.0, s.ad_value(390), 1.0, A::add(A::scale_offset(s.ad_value(753), 1.0 / (s.v[93]), (-1.0)), s.ad_value(391)), 1.0));}
            s.b[759] = ((((s.v[754] - s.v[387])) as f64).abs() < 5e-12);s.store_scalar(759, if s.b[759] { 1.0 } else { 0.0 });
            if (s.b[733] && s.b[759]) {s.store_scalar(167, s.v[57]);}
            if s.b[733] {s.copy_ad(387, 754);}
            let (t41,) = {
    if s.b[733] {
        (s.v[752],)
    } else {
        (s.v[386],)
    }
};
            s.store_scalar(386, t41);
            if s.b[733] {s.store_primal_offset(167, 167, 1.0);}
        }
        if s.b[733] {s.copy_ad(388, 390);s.store_sqrt_div_scaled_inputs(761, 388, ((2.0 * 1.034943e-10) / 1.6021918e-19), 544, 1.0);}
        s.b[766] = (s.v[761] > (0.99 * s.v[736]));s.store_scalar(766, if s.b[766] { 1.0 } else { 0.0 });
        if (s.b[733] && s.b[766]) {s.store_div_from_scalar(760, 1.0, 323);s.store_scale(761, 736, 9662367879.197212);s.store_scalar(762, (1.0 / s.v[93]));s.store_div_from_scalar_ad(763, 1.0, A::add_scaled_inputs3(s.ad_value(760), 1.0, s.ad_value(761), 1.0, s.ad_value(762), 1.0));s.store_sub_from_scalar_scaled_mul(764, 1.0, 763, 760, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_18(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
    ) {
        if (s.b[733] && s.b[766]) {s.store_mul_ad_product_rhs_mixed_ia(765, 760, 763, A::sub(A::mul_scaled_rhs(A::add_scaled_inputs(s.ad_value(762), 1.0, s.ad_value(761), 0.5), s.ad_value(341), -1.0), s.ad_value(475)));s.store_div(383, 765, 764);}
        let (t45,) = {
    if (s.b[733] && s.b[766]) {
        let t44: f64 = (s.v[160] + s.v[383]);
        (t44,)
    } else {
        (s.v[160],)
    }
};
        s.store_scalar(160, t45);
        if s.b[733] {s.store_scaled_mul(767, 155, 157, 0.5);s.store_scale(44, 767, (2.0 * 10.0));s.store_offset_mul_offset_rhs_mixed_ia(45, 44, A::mul_offset_rhs(s.ad_value(44), A::mul_offset_rhs(s.ad_value(44), A::mul_offset_rhs(s.ad_value(44), A::mul(s.ad_value(44), A::scale_offset(s.ad_value(44), (1.0 / 5040.0), (1.0 / 720.0))), (1.0 / 120.0)), (1.0 / 24.0)), (1.0 / 6.0)), (1.0 / 2.0), 1.0);s.store_div_from_scalar(768, 0.1, 45);}
        s.b[769] = (s.v[768] < 5e-12);s.store_scalar(769, if s.b[769] { 1.0 } else { 0.0 });
        if (s.b[733] && s.b[769]) {s.store_scalar(768, 5e-12);}
        if s.b[733] {s.copy_ad(330, 768);s.store_add_scaled_inputs4_offset_indices(179, 158, 1.0, 330, 1.0, 185, 1.0, 320, -1.0, (-s.v[123]));s.store_mul_div_mixed_iia(404, 179, 403, A::mul(s.ad_value(737), s.ad_value(231)));}
        s.b[770] = ((s.v[404] < (s.v[736] * 7.0)) && ((s.v[736] * 7.0) >= 0.0));s.store_scalar(770, if s.b[770] { 1.0 } else { 0.0 });
        if (s.b[733] && s.b[770]) {s.store_sub_scaled_inputs(44, 736, 7.0, 404, 1.0);s.store_square(49, 44);s.store_scaled_mul(50, 736, 736, (7.0 * 7.0));s.store_scalar(51, 1.0);s.store_scalar(52, 1.0);}
        let (t46,) = {
    if (s.b[733] && s.b[770]) {
        (0.0,)
    } else {
        (s.v[54],)
    }
};
        s.store_scalar(54, t46);
        let (t47,) = {
    if (s.b[733] && s.b[770]) {
        (0.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t47);
        if (s.b[733] && s.b[770]) {s.store_scalar(48, 0.0);s.store_scalar(53, 0.0);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_add(48, 51, 52);s.copy_ad(53, 48);}
        s.b[771] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(771, if s.b[771] { 1.0 } else { 0.0 });s.b[772] = (2.0 == 1.0);s.store_scalar(772, if s.b[772] { 1.0 } else { 0.0 });
        let (t48,) = {
    if (((s.b[733] && s.b[770]) && s.b[771]) && s.b[772]) {
        (1.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t48);s.b[773] = (2.0 == 2.0);s.store_scalar(773, if s.b[773] { 1.0 } else { 0.0 });
        let (t49,) = {
    if ((((s.b[733] && s.b[770]) && s.b[771]) && (!s.b[772])) && s.b[773]) {
        (2.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t49);s.b[774] = (2.0 == 4.0);s.store_scalar(774, if s.b[774] { 1.0 } else { 0.0 });
        let (t4a,) = {
    if (((((s.b[733] && s.b[770]) && s.b[771]) && (!s.b[772])) && (!s.b[773])) && s.b[774]) {
        (3.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t4a);s.b[775] = (2.0 == 8.0);s.store_scalar(775, if s.b[775] { 1.0 } else { 0.0 });
        let (t4b,) = {
    if ((((((s.b[733] && s.b[770]) && s.b[771]) && (!s.b[772])) && (!s.b[773])) && (!s.b[774])) && s.b[775]) {
        (4.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t4b);
        let (t4c,) = {
    if ((s.b[733] && s.b[770]) && s.b[771]) {
        (0.0,)
    } else {
        (s.v[54],)
    }
};
        s.store_scalar(54, t4c);let mut t50: usize = 0;
        while {
            let t4f: f64 = if (((s.b[733] && s.b[770]) && s.b[771]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            t4f != 0.0
        } {
            t50 += 1;
            if t50 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", t50, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((s.b[733] && s.b[770]) && s.b[771]) {s.store_sqrt(53, 53);}
            let (t4e,) = {
    if ((s.b[733] && s.b[770]) && s.b[771]) {
        let t4d: f64 = (s.v[54] + 1.0);
        (t4d,)
    } else {
        (s.v[54],)
    }
};
            s.store_scalar(54, t4e);
        }
        if ((s.b[733] && s.b[770]) && (!s.b[771])) {s.store_powf(53, 53, (1.0 / (2.0 * 2.0)));}
        if (s.b[733] && s.b[770]) {s.store_div_from_scalar(53, 1.0, 53);s.store_mul3_affine_lhs(43, 44, 736, 7.0, 0.0, 53);s.store_sub_scaled_inputs(405, 736, 7.0, 43, 1.0);}
        if (s.b[733] && (!s.b[770])) {s.copy_ad(405, 404);}
        s.b[776] = ((s.v[405] > (s.v[403] - s.v[736])) && (s.v[736] >= 0.0));s.store_scalar(776, if s.b[776] { 1.0 } else { 0.0 });
        if (s.b[733] && s.b[776]) {s.store_add_scaled_inputs3_indices(44, 405, 1.0, 403, (-1.0), 736, 1.0);s.store_square(49, 44);s.store_scaled_mul(50, 736, 736, 1.0);s.store_scalar(51, 1.0);s.store_scalar(52, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_19(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
    ) {
        let (t51,) = {
    if (s.b[733] && s.b[776]) {
        (0.0,)
    } else {
        (s.v[54],)
    }
};
        s.store_scalar(54, t51);
        let (t52,) = {
    if (s.b[733] && s.b[776]) {
        (0.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t52);
        if (s.b[733] && s.b[776]) {s.store_scalar(48, 0.0);s.store_scalar(53, 0.0);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_add(48, 51, 52);s.copy_ad(53, 48);}
        s.b[777] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(777, if s.b[777] { 1.0 } else { 0.0 });s.b[778] = (2.0 == 1.0);s.store_scalar(778, if s.b[778] { 1.0 } else { 0.0 });
        let (t53,) = {
    if (((s.b[733] && s.b[776]) && s.b[777]) && s.b[778]) {
        (1.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t53);s.b[779] = (2.0 == 2.0);s.store_scalar(779, if s.b[779] { 1.0 } else { 0.0 });
        let (t54,) = {
    if ((((s.b[733] && s.b[776]) && s.b[777]) && (!s.b[778])) && s.b[779]) {
        (2.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t54);s.b[780] = (2.0 == 4.0);s.store_scalar(780, if s.b[780] { 1.0 } else { 0.0 });
        let (t55,) = {
    if (((((s.b[733] && s.b[776]) && s.b[777]) && (!s.b[778])) && (!s.b[779])) && s.b[780]) {
        (3.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t55);s.b[781] = (2.0 == 8.0);s.store_scalar(781, if s.b[781] { 1.0 } else { 0.0 });
        let (t56,) = {
    if ((((((s.b[733] && s.b[776]) && s.b[777]) && (!s.b[778])) && (!s.b[779])) && (!s.b[780])) && s.b[781]) {
        (4.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t56);
        let (t57,) = {
    if ((s.b[733] && s.b[776]) && s.b[777]) {
        (0.0,)
    } else {
        (s.v[54],)
    }
};
        s.store_scalar(54, t57);let mut t5b: usize = 0;
        while {
            let t5a: f64 = if (((s.b[733] && s.b[776]) && s.b[777]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            t5a != 0.0
        } {
            t5b += 1;
            if t5b > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", t5b, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((s.b[733] && s.b[776]) && s.b[777]) {s.store_sqrt(53, 53);}
            let (t59,) = {
    if ((s.b[733] && s.b[776]) && s.b[777]) {
        let t58: f64 = (s.v[54] + 1.0);
        (t58,)
    } else {
        (s.v[54],)
    }
};
            s.store_scalar(54, t59);
        }
        if ((s.b[733] && s.b[776]) && (!s.b[777])) {s.store_powf(53, 53, (1.0 / (2.0 * 2.0)));}
        if (s.b[733] && s.b[776]) {s.store_div_from_scalar(53, 1.0, 53);s.store_mul3_lhs(43, 44, 736, 53);s.store_add_scaled_inputs3_indices(405, 403, 1.0, 736, (-1.0), 43, 1.0);}
        if (s.b[733] && (!s.b[776])) {
        }
        if s.b[733] {s.store_mul_scale_offset_indices(369, 229, 405, -1.0, 0.0);}
        let (t61,) = {
    if s.b[733] {
        let t5c: f64 = (-s.v[341]);let t5d: f64 = (t5c * s.v[736]);let t5e: f64 = (t5d / 2.0);let t5f: f64 = (t5e / 1.034943e-10);let t60: f64 = (t5f + s.v[227]);
        (t60,)
    } else {
        (s.v[384],)
    }
};
        s.store_scalar(384, t61);
        let (t65,) = {
    if s.b[733] {
        let t62: f64 = (s.v[386] * s.v[736]);let t63: f64 = (t62 / 1.034943e-10);let t64: f64 = (s.v[384] - t63);
        (t64,)
    } else {
        (s.v[385],)
    }
};
        s.store_scalar(385, t65);s.b[782] = (s.v[144] >= 1.0);s.store_scalar(782, if s.b[782] { 1.0 } else { 0.0 });
        if (s.b[733] && s.b[782]) {s.store_scalar(349, s.v[619]);s.store_scalar(350, s.v[620]);s.store_scalar(351, s.v[621]);}
        let (t67,) = {
    if (s.b[733] && s.b[782]) {
        let (t66,) = {
            if (s.v[349] < s.v[385]) {
                (1.0,)
            } else {
                (2.0,)
            }
        };
        (t66,)
    } else {
        (s.v[339],)
    }
};
        s.store_scalar(339, t67);
        if (s.b[733] && (!s.b[782])) {s.store_offset_div_scaled_offset_numerator(336, A::mul(s.ad_value(225), s.ad_value(178)), 4.0, ((-1.0) * 4.0), A::mul(s.ad_value(241), s.ad_value(226)), 1.0, 1.0);}
        if (s.b[733] && (!s.b[782])) {
            if (s.v[336] >= (10.0 * 2.220446049250313e-16)) {
            } else {
                s.store_scalar(336, (10.0 * 2.220446049250313e-16));
            }
        }
        if (s.b[733] && (!s.b[782])) {s.store_add_product3_rhs_mixed_iia(376, 178, 241, 225, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(336))), 0.5);s.store_mul(181, 225, 376);}
        s.b[783] = (s.v[181] < 3.0);s.store_scalar(783, if s.b[783] { 1.0 } else { 0.0 });
        if ((s.b[733] && (!s.b[782])) && s.b[783]) {s.store_mul_sub_rhs(337, 225, 178, 156);s.store_div_scalar_by_product_indices(328, 1.0, 225, 240, (1.414213562373095 / 108.0));s.store_offset_scaled(329, 328, 3.0, 81.0);s.store_add_scaled_sub_value_product_mixed_aii(330, (-2916.0), A::scale(s.ad_value(328), 81.0), 1.0, 328, 337, 27.0);s.store_add_scaled_sub_value_product_mixed_aii(331, 1458.0, A::scaled_offset(s.ad_value(328), 54.0, 81.0), 1.0, 328, 337, 27.0);s.store_square(331, 331);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_20(
        s: &mut Scratch,
    ) {
        if ((s.b[733] && (!s.b[782])) && s.b[783]) {s.store_powf_ad(332, A::add(s.ad_value(330), A::sqrt(A::add(A::mul3_scaled_output(s.ad_value(329), s.ad_value(329), s.ad_value(329), 4.0), s.ad_value(331)))), 0.3333333333333333);s.store_add_scaled_inputs_mixed_ai(336, A::sub_from_scalar(3.0, A::div_scaled_inputs(s.ad_value(329), 1.259921049894873, s.ad_value(332), 3.0)), 1.0, 332, (1.0 / (3.0 * 1.259921049894873)));s.store_add_scaled_product_indices(376, 156, 1.0, 336, 227, 1.0);s.copy_ad(378, 376);}
        s.b[784] = ((s.v[158] - s.v[383]) <= s.v[182]);s.store_scalar(784, if s.b[784] { 1.0 } else { 0.0 });
        if (((s.b[733] && (!s.b[782])) && (!s.b[783])) && s.b[784]) {s.store_div_from_scalar(327, 1.0, 323);s.store_scale(328, 736, 9662367879.197212);s.store_scalar(329, (1.0 / s.v[93]));s.store_div_from_scalar_ad(330, 1.0, A::add_scaled_inputs3(s.ad_value(327), 1.0, s.ad_value(328), 1.0, s.ad_value(329), 1.0));s.store_mul_mixed_ia(331, 330, A::add_scaled_inputs_product(s.ad_value(178), 1.0, s.ad_value(475), (-1.0), A::add_scaled_inputs(s.ad_value(329), 1.0, s.ad_value(328), 0.5), s.ad_value(369), -1.0));s.store_sub_div_rhs_indices(376, 178, 331, 323);s.copy_ad(378, 376);}
        if (((s.b[733] && (!s.b[782])) && (!s.b[783])) && (!s.b[784])) {s.store_div_scalar_by_product_indices(328, 1.0, 379, 434, 1.0);s.store_mul_ad_product_rhs(329, 328, A::sub(s.ad_value(178), s.ad_value(383)), A::sub(s.ad_value(178), s.ad_value(383)));s.store_add_mixed_ia(330, 225, A::div_from_scalar(2.0, A::sub(s.ad_value(178), s.ad_value(383))));s.store_div_ln_lhs(377, 329, 330);s.store_offset_sub(44, 377, 376, (-0.0008));s.store_scale(45, 377, (4.0 * 0.0008));}
        if (((s.b[733] && (!s.b[782])) && (!s.b[783])) && (!s.b[784])) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }
        if (((s.b[733] && (!s.b[782])) && (!s.b[783])) && (!s.b[784])) {s.store_sqrt_square_add(45, 44, 45);s.store_add_scaled_inputs3_indices(378, 377, 1.0, 44, (-0.5), 45, (-0.5));}
        if (s.b[733] && (!s.b[782])) {
            if (s.v[378] > 0.0) {
                s.store_sqrt_div_scaled_inputs(401, 378, ((2.0 * 1.034943e-10) / 1.6021918e-19), 544, 1.0);
            } else {
                s.store_scalar(401, 0.0);
            }
        }
        s.b[785] = (s.v[401] < s.v[736]);s.store_scalar(785, if s.b[785] { 1.0 } else { 0.0 });
        let (t68,) = {
    if ((s.b[733] && (!s.b[782])) && s.b[785]) {
        (1.0,)
    } else {
        (s.v[339],)
    }
};
        s.store_scalar(339, t68);
        let (t69,) = {
    if ((s.b[733] && (!s.b[782])) && (!s.b[785])) {
        (2.0,)
    } else {
        (s.v[339],)
    }
};
        s.store_scalar(339, t69);s.b[786] = ((s.v[158] - s.v[383]) <= s.v[182]);s.store_scalar(786, if s.b[786] { 1.0 } else { 0.0 });
        if ((s.b[733] && (!s.b[782])) && s.b[786]) {s.store_div_from_scalar(327, 1.0, 323);s.store_scale(328, 736, 9662367879.197212);s.store_scalar(329, (1.0 / s.v[93]));s.store_div_from_scalar_ad(330, 1.0, A::add_scaled_inputs3(s.ad_value(327), 1.0, s.ad_value(328), 1.0, s.ad_value(329), 1.0));s.store_mul_mixed_ia(331, 330, A::add_scaled_inputs_product(s.ad_value(178), 1.0, s.ad_value(475), (-1.0), A::add_scaled_inputs(s.ad_value(329), 1.0, s.ad_value(328), 0.5), s.ad_value(369), -1.0));s.store_sub_div_rhs_indices(376, 178, 331, 323);s.copy_ad(378, 376);}
        if ((s.b[733] && (!s.b[782])) && (!s.b[786])) {s.store_div_from_scalar(327, 1.0, 323);s.store_scale(328, 736, 9662367879.197212);s.store_scalar(329, (1.0 / s.v[93]));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_21(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
    ) {
        if ((s.b[733] && (!s.b[782])) && (!s.b[786])) {s.store_div_from_scalar_ad(330, 1.0, A::add_scaled_inputs3(s.ad_value(327), 1.0, s.ad_value(328), 1.0, s.ad_value(329), 1.0));s.store_mul_mixed_ia(331, 330, A::add_scaled_inputs_product(s.ad_value(178), 1.0, s.ad_value(475), (-1.0), A::add_scaled_inputs(s.ad_value(329), 1.0, s.ad_value(328), 0.5), s.ad_value(369), -1.0));s.store_sub_div_rhs_indices(376, 178, 331, 323);s.copy_ad(378, 376);}
        s.b[787] = ((s.v[178] - s.v[383]) > 0.0);s.store_scalar(787, if s.b[787] { 1.0 } else { 0.0 });
        if (((s.b[733] && (!s.b[782])) && (!s.b[786])) && s.b[787]) {s.store_div_scalar_by_product_indices(328, 1.0, 379, 434, 1.0);s.store_mul_ad_product_rhs(329, 328, A::sub(s.ad_value(178), s.ad_value(383)), A::sub(s.ad_value(178), s.ad_value(383)));s.store_add_mixed_ia(330, 225, A::div_from_scalar(2.0, A::sub(s.ad_value(178), s.ad_value(383))));s.store_div_ln_lhs(377, 329, 330);}
        s.b[788] = ((s.v[376] > ((s.v[377] * 0.98) - 0.4)) && (0.4 >= 0.0));s.store_scalar(788, if s.b[788] { 1.0 } else { 0.0 });
        if ((((s.b[733] && (!s.b[782])) && (!s.b[786])) && s.b[787]) && s.b[788]) {s.store_offset_sub_scaled_inputs_indices(44, 376, 1.0, 377, 0.98, 0.4);s.store_square(49, 44);s.store_scalar(50, (0.4 * 0.4));s.store_scalar(51, 1.0);s.store_scalar(52, 1.0);}
        let (t6a,) = {
    if ((((s.b[733] && (!s.b[782])) && (!s.b[786])) && s.b[787]) && s.b[788]) {
        (0.0,)
    } else {
        (s.v[54],)
    }
};
        s.store_scalar(54, t6a);
        let (t6b,) = {
    if ((((s.b[733] && (!s.b[782])) && (!s.b[786])) && s.b[787]) && s.b[788]) {
        (0.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t6b);
        if ((((s.b[733] && (!s.b[782])) && (!s.b[786])) && s.b[787]) && s.b[788]) {s.store_scalar(48, 0.0);s.store_scalar(53, 0.0);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_add(48, 51, 52);s.copy_ad(53, 48);}
        s.b[789] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(789, if s.b[789] { 1.0 } else { 0.0 });s.b[790] = (2.0 == 1.0);s.store_scalar(790, if s.b[790] { 1.0 } else { 0.0 });
        let (t6c,) = {
    if ((((((s.b[733] && (!s.b[782])) && (!s.b[786])) && s.b[787]) && s.b[788]) && s.b[789]) && s.b[790]) {
        (1.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t6c);s.b[791] = (2.0 == 2.0);s.store_scalar(791, if s.b[791] { 1.0 } else { 0.0 });
        let (t6d,) = {
    if (((((((s.b[733] && (!s.b[782])) && (!s.b[786])) && s.b[787]) && s.b[788]) && s.b[789]) && (!s.b[790])) && s.b[791]) {
        (2.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t6d);s.b[792] = (2.0 == 4.0);s.store_scalar(792, if s.b[792] { 1.0 } else { 0.0 });
        let (t6e,) = {
    if ((((((((s.b[733] && (!s.b[782])) && (!s.b[786])) && s.b[787]) && s.b[788]) && s.b[789]) && (!s.b[790])) && (!s.b[791])) && s.b[792]) {
        (3.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t6e);s.b[793] = (2.0 == 8.0);s.store_scalar(793, if s.b[793] { 1.0 } else { 0.0 });
        let (t6f,) = {
    if (((((((((s.b[733] && (!s.b[782])) && (!s.b[786])) && s.b[787]) && s.b[788]) && s.b[789]) && (!s.b[790])) && (!s.b[791])) && (!s.b[792])) && s.b[793]) {
        (4.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t6f);
        let (t70,) = {
    if (((((s.b[733] && (!s.b[782])) && (!s.b[786])) && s.b[787]) && s.b[788]) && s.b[789]) {
        (0.0,)
    } else {
        (s.v[54],)
    }
};
        s.store_scalar(54, t70);let mut t74: usize = 0;
        while {
            let t73: f64 = if ((((((s.b[733] && (!s.b[782])) && (!s.b[786])) && s.b[787]) && s.b[788]) && s.b[789]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            t73 != 0.0
        } {
            t74 += 1;
            if t74 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", t74, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if (((((s.b[733] && (!s.b[782])) && (!s.b[786])) && s.b[787]) && s.b[788]) && s.b[789]) {s.store_sqrt(53, 53);}
            let (t72,) = {
    if (((((s.b[733] && (!s.b[782])) && (!s.b[786])) && s.b[787]) && s.b[788]) && s.b[789]) {
        let t71: f64 = (s.v[54] + 1.0);
        (t71,)
    } else {
        (s.v[54],)
    }
};
            s.store_scalar(54, t72);
        }
        if (((((s.b[733] && (!s.b[782])) && (!s.b[786])) && s.b[787]) && s.b[788]) && (!s.b[789])) {s.store_powf(53, 53, (1.0 / (2.0 * 2.0)));}
        if ((((s.b[733] && (!s.b[782])) && (!s.b[786])) && s.b[787]) && s.b[788]) {s.store_div_from_scalar(53, 1.0, 53);s.store_scaled_mul(43, 44, 53, 0.4);s.store_add_mixed_ai(378, A::scale_offset(s.ad_value(377), 0.98, (-0.4)), 43);}
        if ((((s.b[733] && (!s.b[782])) && (!s.b[786])) && s.b[787]) && (!s.b[788])) {s.copy_ad(378, 376);}
        if (s.b[733] && (!s.b[782])) {s.copy_ad(349, 378);s.copy_ad(163, 376);s.store_sub_mixed_ai(328, A::add_scaled_product(s.ad_value(349), 1.0, s.ad_value(341), s.ad_value(735), 0.5), 475);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_22(
        s: &mut Scratch,
    ) {
        s.b[794] = (s.v[328] < 0.0);s.store_scalar(794, if s.b[794] { 1.0 } else { 0.0 });
        if ((s.b[733] && (!s.b[782])) && s.b[794]) {s.store_mul_scale_offset_indices(329, 474, 735, 1.0, s.v[94]);s.store_square(329, 329);s.store_offset_scaled(332, 328, (-1.6), 0.6);s.store_scalar(331, 0.5);s.store_add_scaled_inputs3_indices(44, 332, 1.0, 331, (-1.0), 332, (-0.001));s.store_scaled_mul(45, 332, 332, (4.0 * 0.001));}
        if ((s.b[733] && (!s.b[782])) && s.b[794]) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }
        if ((s.b[733] && (!s.b[782])) && s.b[794]) {s.store_sqrt_square_add(45, 44, 45);s.store_add_scaled_inputs3_indices(331, 332, 1.0, 44, (-0.5), 45, (-0.5));s.store_mul3_lhs(330, 329, 331, 226);s.store_div_ad(351, A::mul_sub_from_scalar_rhs(s.ad_value(328), 1.0, A::sqrt(s.ad_value(330))), A::sub_from_scalar(1.0, s.ad_value(330)));}
        if ((s.b[733] && (!s.b[782])) && (!s.b[794])) {s.store_scaled_square(327, 474, (s.v[95] * s.v[95]));s.store_neg_ad(328, A::add_scaled_inputs_product(s.ad_value(475), 1.0, s.ad_value(349), (-1.0), s.ad_value(341), s.ad_value(736), (-(1.0 / (2.0) * 9662367879.197212))));s.store_add_scaled_inputs3_mixed_aai(329, A::square(A::add_scaled_product(s.ad_value(328), 2.0, s.ad_value(327), s.ad_value(225), 1.0)), 1.0, A::square(s.ad_value(328)), (-4.0), 327, (-4.0));}
        if ((s.b[733] && (!s.b[782])) && (!s.b[794])) {
            if (s.v[329] >= (10.0 * 2.220446049250313e-16)) {
            } else {
                s.store_scalar(329, (10.0 * 2.220446049250313e-16));
            }
        }
        if ((s.b[733] && (!s.b[782])) && (!s.b[794])) {s.store_sqrt(329, 329);s.store_add_scaled_product_indices(330, 328, 2.0, 327, 225, 1.0);s.store_scaled_sub(380, 330, 329, 0.5);s.store_div_ad(381, A::ln(A::div_scaled_product_by_product(s.ad_value(328), s.ad_value(328), 1.0, s.ad_value(327), s.ad_value(239), 1.0)), A::add(s.ad_value(225), A::div_from_scalar(2.0, s.ad_value(328))));}
        s.b[795] = (s.v[380] < s.v[382]);s.store_scalar(795, if s.b[795] { 1.0 } else { 0.0 });
        if (((s.b[733] && (!s.b[782])) && (!s.b[794])) && s.b[795]) {s.copy_ad(351, 380);}
        if (((s.b[733] && (!s.b[782])) && (!s.b[794])) && (!s.b[795])) {s.store_offset_sub(44, 381, 380, (-0.0008));s.store_scale(45, 381, (4.0 * 0.0008));}
        if (((s.b[733] && (!s.b[782])) && (!s.b[794])) && (!s.b[795])) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }
        if (((s.b[733] && (!s.b[782])) && (!s.b[794])) && (!s.b[795])) {s.store_sqrt_square_add(45, 44, 45);s.store_add_scaled_inputs3_indices(351, 381, 1.0, 44, (-0.5), 45, (-0.5));}
        if (s.b[733] && (!s.b[782])) {s.store_scalar(167, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_23(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
    ) {
        let mut t76: usize = 0;
        while {
            let t75: f64 = if ((s.b[733] && (!s.b[782])) && (s.v[167] < s.v[57])) { 1.0 } else { 0.0 };
            t75 != 0.0
        } {
            t76 += 1;
            if t76 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", t76, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if (s.b[733] && (!s.b[782])) {s.copy_ad(328, 474);s.store_mul(329, 225, 351);s.store_exp_neg_input(330, 329);}
            s.b[796] = (s.v[351] > 1e-9);s.store_scalar(796, if s.b[796] { 1.0 } else { 0.0 });
            if ((s.b[733] && (!s.b[782])) && s.b[796]) {s.store_exp_mul(327, 225, 351);s.store_mul_scaled_sqrt_ad_rhs(331, 328, -1.0, A::add_scaled_offset_product_rhs(A::offset(A::add(s.ad_value(330), s.ad_value(329)), (-1.0)), 1.0, s.ad_value(239), s.ad_value(327), (-1.0), 1.0));s.store_mul_div_from_scalar_lhs_ad_mixed_ia(332, s.v[122], 331, A::add_scaled_sub_value_product(1.0, s.ad_value(330), 1.0, s.ad_value(239), s.ad_value(327), 1.0));}
            s.b[797] = (s.v[351] < (-1e-9));s.store_scalar(797, if s.b[797] { 1.0 } else { 0.0 });
            if (((s.b[733] && (!s.b[782])) && (!s.b[796])) && s.b[797]) {s.store_mul_sqrt_mixed_ia(331, 328, A::offset(A::add(s.ad_value(330), s.ad_value(329)), (-1.0)));s.store_mul_scale_offset_mixed_ai(332, A::div_from_scalar(s.v[122], s.ad_value(331)), 330, -1.0, 1.0);}
            if (((s.b[733] && (!s.b[782])) && (!s.b[796])) && (!s.b[797])) {s.store_mul_ad_affine_product_lhs(331, A::sqrt(A::div_from_scalar(s.v[122], s.ad_value(225))), s.ad_value(225), -1.0, 0.0, 351);s.store_scaled_sqrt_scaled_input(332, 225, s.v[122], -1.0);}
            if (s.b[733] && (!s.b[782])) {s.store_sqrt_add_scaled_square_product(45, 331, 1.0, 739, 739, 4.0);s.store_offset_scaled_div(334, 331, 45, 0.5, 0.5);s.store_add_scaled_inputs3_indices(333, 331, 0.5, 45, 0.5, 739, 1e-10);}
            s.b[798] = (s.v[333] < 0.0);s.store_scalar(798, if s.b[798] { 1.0 } else { 0.0 });
            if ((s.b[733] && (!s.b[782])) && s.b[798]) {s.store_scalar(333, 0.0);s.store_scalar(334, 0.0);}
            if (s.b[733] && (!s.b[782])) {s.store_add_scaled_inputs3_indices(44, 341, -1.0, 333, (-1.0), 740, -1.0);s.store_scaled_mul(45, 341, 740, (-4.0));}
            if (s.b[733] && (!s.b[782])) {
                if (s.v[45] > 0.0) {
                } else {
                    s.store_neg(45, 45);
                }
            }
            if (s.b[733] && (!s.b[782])) {s.store_sqrt_square_add(45, 44, 45);s.store_offset_scaled_div(335, 44, 45, 0.5, 0.5);s.store_add_scaled_inputs3_indices(333, 341, -1.0, 44, (-0.5), 45, (-0.5));s.store_mul3_lhs(334, 334, 332, 335);s.store_div_scaled_inputs_mixed_ai(388, A::square(s.ad_value(333)), ((0.5 * 9662367879.197212) * 6.241449993689894e18), 544, 1.0);s.store_div_scaled_product_indices(389, 388, 334, 2.0, 333, 1.0);s.store_sub_mixed_ia(333, 351, A::div_scaled_inputs3(A::add_scaled_product(A::add_scaled_inputs3(s.ad_value(349), 1.0, s.ad_value(351), (-1.0), s.ad_value(331), 1.0 / (s.v[93])), 1.0, A::add_scaled_inputs(s.ad_value(331), 1.0, s.ad_value(341), 0.5), s.ad_value(736), 9662367879.197212), 1.0, s.ad_value(475), (-1.0), s.ad_value(388), 1.0, A::add(A::add_scaled_product(A::scale_offset(s.ad_value(332), 1.0 / (s.v[93]), (-1.0)), 1.0, s.ad_value(332), s.ad_value(736), 9662367879.197212), s.ad_value(389)), 1.0));s.copy_ad(334, 167);}
            s.b[799] = ((((s.v[333] - s.v[351])) as f64).abs() < 0.001);s.store_scalar(799, if s.b[799] { 1.0 } else { 0.0 });
            if ((s.b[733] && (!s.b[782])) && s.b[799]) {s.store_scalar(167, s.v[57]);}
            if (s.b[733] && (!s.b[782])) {s.copy_ad(351, 333);s.copy_ad(357, 331);s.store_primal_offset(167, 167, 1.0);}
        }
        if (s.b[733] && (!s.b[782])) {s.store_add(351, 475, 351);s.store_add_scaled_product_mixed_iia(350, 349, 1.0, 735, A::add_scaled_inputs(s.ad_value(341), 0.5, s.ad_value(357), 1.0), 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_24(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[800] = ((p[25] == 1.0) && (s.v[158] > (s.v[160] + 0.2)));s.store_scalar(800, if s.b[800] { 1.0 } else { 0.0 });
        if (s.b[733] && s.b[800]) {s.store_scalar(446, s.v[136]);s.store_add_scaled_inputs4_indices(445, 174, 1.0, 446, (-1.0), 185, 1.0, 320, -1.0);s.store_scalar(143, p[137]);s.copy_ad(207, 445);s.store_sqrt_div_scaled_inputs(208, 544, ((2.0 * 1.6021918e-19) * 1.034943e-10), 225, 1.0);s.store_div_scaled_product_by_product_indices(209, 230, 230, 1.0, 544, 544, 1.0);s.store_div_scaled_product_by_product_indices(210, 208, 208, 1.0, 323, 323, 1.0);s.store_scaled_mul(211, 210, 225, 0.5);s.store_scaled_mul(212, 211, 225, 2.0);s.store_sqrt_offset_ad(213, A::div_scaled_offset_numerator(A::mul(s.ad_value(225), s.ad_value(207)), 4.0, ((-1.0) * 4.0), s.ad_value(212), 1.0), 1.0);s.store_add_mul_sub_from_scalar_rhs_indices(215, 207, 211, 1.0, 213);s.store_div_scalar_by_product_indices(223, 1.0, 209, 210, 1.0);s.store_div_ad(216, A::ln(A::mul(s.ad_value(223), A::square(s.ad_value(207)))), A::add(s.ad_value(225), A::div_from_scalar(2.0, s.ad_value(207))));s.store_add_scaled_inputs3_indices(217, 216, 1.0, 215, (-1.0), 143, -1.0);s.store_add_scaled_inputs3_sqrt_third_mixed_iia(218, 216, 1.0, 217, (-0.5), A::add_scaled_square_product(s.ad_value(217), 1.0, s.ad_value(143), s.ad_value(216), 4.0), (-0.5));s.store_exp_mul(224, 225, 218);s.store_add_scaled_product_mixed_aii(219, A::offset(A::mul(s.ad_value(225), s.ad_value(218)), (-1.0)), 1.0, 209, 224, 1.0);s.store_offset_mul(220, 225, 218, (-1.0));}
        s.b[801] = ((s.v[219] > 0.0) && (s.v[220] > 0.0));s.store_scalar(801, if s.b[801] { 1.0 } else { 0.0 });
        if ((s.b[733] && s.b[800]) && s.b[801]) {s.store_sqrt_ad(219, A::add_scaled_product(A::offset(A::mul(s.ad_value(225), s.ad_value(218)), (-1.0)), 1.0, s.ad_value(209), s.ad_value(224), 1.0));s.store_sqrt_offset_ad(220, A::mul(s.ad_value(225), s.ad_value(218)), (-1.0));s.store_mul_sub_rhs(221, 208, 219, 220);s.store_div_scaled_inputs_indices(214, 105, 2.0, 225, 1.0);s.store_scalar(250, (300.0 * 0.0001));s.store_scalar(316, 0.0);s.store_neg_ad(328, A::offset(A::exp(A::mul_scaled_lhs(s.ad_value(225), -1.0, s.ad_value(173))), (-1.0)));s.store_div_from_scalar_sub_from_scalar_ad(329, 1.0, s.v[97], s.ad_value(316));s.store_mul_ad_product_lhs_mixed_ai(222, A::mul3(s.ad_value(214), s.ad_value(250), s.ad_value(221)), 328, 329);s.copy_ad(394, 222);s.copy_ad(395, 218);s.store_offset_div_scaled_offset_numerator(336, A::mul(s.ad_value(225), s.ad_value(178)), 4.0, ((-1.0) * 4.0), A::mul(s.ad_value(241), s.ad_value(226)), 1.0, 1.0);}
        s.b[802] = (s.v[336] < (10.0 * 2.220446049250313e-16));s.store_scalar(802, if s.b[802] { 1.0 } else { 0.0 });
        if (((s.b[733] && s.b[800]) && s.b[801]) && s.b[802]) {s.store_scalar(336, (10.0 * 2.220446049250313e-16));}
        if ((s.b[733] && s.b[800]) && s.b[801]) {s.store_add_product3_rhs_mixed_iia(376, 178, 241, 225, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(336))), 0.5);s.copy_ad(163, 376);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_25(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.b[733] && s.b[800]) && s.b[801]) {s.store_sub(166, 376, 395);}
        s.b[803] = (s.v[166] < 0.0);s.store_scalar(803, if s.b[803] { 1.0 } else { 0.0 });
        if (((s.b[733] && s.b[800]) && s.b[801]) && s.b[803]) {s.store_scalar(166, 0.0);}
        if ((s.b[733] && s.b[800]) && s.b[801]) {s.store_scale(332, 166, (1.0 + 0.3));s.store_offset_sub(333, 332, 173, (-0.03));s.store_sqrt_add_scaled_square_input(334, 333, 1.0, 332, (4.0 * 0.03));s.store_add_scaled_inputs3_indices(165, 332, 1.0, 333, (-0.5), 334, (-0.5));}
        s.b[804] = (s.v[165] > s.v[166]);s.store_scalar(804, if s.b[804] { 1.0 } else { 0.0 });
        if (((s.b[733] && s.b[800]) && s.b[801]) && s.b[804]) {s.copy_ad(165, 166);}
        if ((s.b[733] && s.b[800]) && s.b[801]) {s.copy_ad(449, 165);s.store_scalar(822, (s.v[88] * 100.0));s.store_primal_scale(823, 107, 100.0);s.store_scalar(824, (s.v[97] * 100.0));}
        s.b[825] = (p[36] == 0.0);s.store_scalar(825, if s.b[825] { 1.0 } else { 0.0 });
        if (((s.b[733] && s.b[800]) && s.b[801]) && s.b[825]) {s.store_scalar(447, 0.0);}
        if (((s.b[733] && s.b[800]) && s.b[801]) && (!s.b[825])) {s.store_scalar(448, 4.12);s.store_primal_scaled_mul(805, 823, 824, (p[142] * 1.6021918e-19));s.store_div(806, 805, 302);s.store_div_scaled_inputs_mixed_ai(807, A::offset(A::add_scaled_inputs4(s.ad_value(514), p[145], s.ad_value(187), 1.0, s.ad_value(319), 1.0, s.ad_value(237), 1.0), p[144]), -1.0, 822, 1.0);s.store_scalar(562, 0.0);}
        let mut t2: usize = 0;
        while {
            let t0: f64 = (100.0 - 1.0);let t1: f64 = if ((((s.b[733] && s.b[800]) && s.b[801]) && (!s.b[825])) && (s.v[562] <= t0)) { 1.0 } else { 0.0 };
            t1 != 0.0
        } {
            t2 += 1;
            if t2 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", t2, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if (((s.b[733] && s.b[800]) && s.b[801]) && (!s.b[825])) {s.copy_ad(808, 562);s.store_scalar(809, 100.0);s.store_primal_div(810, 808, 809);s.store_add_scaled_inputs3_mixed_iia(811, 159, 1.0, 175, 1.0, A::add_scaled_product(s.ad_value(395), 1.0, s.ad_value(449), s.ad_value(810), 1.0), -1.0);s.store_sub_from_scalar_div_indices(812, 1.0, 811, 448);s.store_add_div_rhs_indices(815, 807, 811, 822);s.store_square(813, 815);s.store_sqrt_square_offset(44, 812, ((4.0 * 0.001) * 0.001));s.store_offset_add_scaled_inputs_indices(812, 812, 0.5, 44, 0.5, (1e-10 * 0.001));}
            s.b[826] = (s.v[812] < 0.0);s.store_scalar(826, if s.b[826] { 1.0 } else { 0.0 });
            if ((((s.b[733] && s.b[800]) && s.b[801]) && (!s.b[825])) && s.b[826]) {s.store_scalar(812, 0.0);}
            if (((s.b[733] && s.b[800]) && s.b[801]) && (!s.b[825])) {s.store_offset_scaled_ad(814, A::mul(A::sqrt(s.ad_value(812)), s.ad_value(812)), (-p[143]), p[143]);s.store_div_scaled_inputs_indices(816, 814, -1.0, 815, 1.0);}
            s.b[827] = (s.v[816] < (-34.0));s.store_scalar(827, if s.b[827] { 1.0 } else { 0.0 });
            if ((((s.b[733] && s.b[800]) && s.b[801]) && (!s.b[825])) && s.b[827]) {s.store_scalar(818, 0.0);}
            if ((((s.b[733] && s.b[800]) && s.b[801]) && (!s.b[825])) && (!s.b[827])) {s.store_exp(818, 816);}
            if (((s.b[733] && s.b[800]) && s.b[801]) && (!s.b[825])) {s.copy_ad(819, 806);s.store_mul3_affine_lhs(820, 819, 814, (0.25 * 7.38905609893065), 0.0, 814);}
            s.b[828] = (((2.0 * s.v[815]) + s.v[814]) < 0.0);s.store_scalar(828, if s.b[828] { 1.0 } else { 0.0 });
            if ((((s.b[733] && s.b[800]) && s.b[801]) && (!s.b[825])) && s.b[828]) {s.copy_ad(450, 820);}
            if ((((s.b[733] && s.b[800]) && s.b[801]) && (!s.b[825])) && (!s.b[828])) {s.copy_ad(817, 805);s.store_mul3_lhs(821, 817, 813, 818);}
            s.b[829] = ((s.v[821] < s.v[820]) || (s.v[815] < 0.0));s.store_scalar(829, if s.b[829] { 1.0 } else { 0.0 });
            if (((((s.b[733] && s.b[800]) && s.b[801]) && (!s.b[825])) && (!s.b[828])) && s.b[829]) {s.copy_ad(450, 820);}
            if (((((s.b[733] && s.b[800]) && s.b[801]) && (!s.b[825])) && (!s.b[828])) && (!s.b[829])) {s.copy_ad(450, 821);}
            if (((s.b[733] && s.b[800]) && s.b[801]) && (!s.b[825])) {s.store_add(447, 447, 450);}
            s.b[830] = (s.v[450] < 1e-9);s.store_scalar(830, if s.b[830] { 1.0 } else { 0.0 });
            if ((((s.b[733] && s.b[800]) && s.b[801]) && (!s.b[825])) && s.b[830]) {s.store_scalar(562, 100.0);s.store_scalar(167, s.v[57]);}
            if (((s.b[733] && s.b[800]) && s.b[801]) && (!s.b[825])) {s.store_primal_offset(562, 562, 1.0);}
        }
        s.b[843] = ((p[117] <= 0.0) || (s.v[73] <= 0.0));s.store_scalar(843, if s.b[843] { 1.0 } else { 0.0 });
        if (((s.b[733] && s.b[800]) && s.b[801]) && s.b[843]) {s.store_scalar(263, 0.0);}
        s.b[844] = (p[44] <= 0.0);s.store_scalar(844, if s.b[844] { 1.0 } else { 0.0 });
        if ((((s.b[733] && s.b[800]) && s.b[801]) && (!s.b[843])) && s.b[844]) {s.copy_ad(831, 445);s.store_square(838, 323);s.copy_ad(839, 545);s.store_div(833, 839, 838);s.store_div_from_scalar(840, 2.0, 839);s.store_mul(834, 840, 838);s.store_add_scaled_inputs_product_indices(835, 831, 1.0, 227, (-1.0), 130, 514, (-1.0));s.store_offset_mul(837, 834, 835, 1.0);s.store_sqrt_square_offset(44, 837, ((4.0 * 0.001) * 0.001));s.store_offset_add_scaled_inputs_indices(836, 837, 0.5, 44, 0.5, (1e-10 * 0.001));}
        s.b[845] = (s.v[836] < 0.0);s.store_scalar(845, if s.b[845] { 1.0 } else { 0.0 });
        if (((((s.b[733] && s.b[800]) && s.b[801]) && (!s.b[843])) && s.b[844]) && s.b[845]) {s.store_scalar(836, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_26(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((s.b[733] && s.b[800]) && s.b[801]) && (!s.b[843])) && s.b[844]) {s.store_offset(836, 836, 1e-50);s.store_sqrt(836, 836);s.store_add_scaled_product_mixed_aii(841, A::mul_sub_from_scalar_rhs(s.ad_value(833), 1.0, s.ad_value(836)), 1.0, 831, 137, 1.0);s.store_add_scaled_inputs3_mixed_iia(842, 173, p[122], 395, 1.0, A::mul3(s.ad_value(131), s.ad_value(129), s.ad_value(841)), -1.0);s.store_sqrt_square_offset(44, 842, ((4.0 * 0.01) * 0.01));s.store_offset_add_scaled_inputs_indices(842, 842, 0.5, 44, 0.5, (1e-10 * 0.01));}
        s.b[846] = (s.v[842] < 0.0);s.store_scalar(846, if s.b[846] { 1.0 } else { 0.0 });
        if (((((s.b[733] && s.b[800]) && s.b[801]) && (!s.b[843])) && s.b[844]) && s.b[846]) {s.store_scalar(842, 0.0);}
        if ((((s.b[733] && s.b[800]) && s.b[801]) && (!s.b[843])) && (!s.b[844])) {s.store_mul(831, 134, 445);s.store_div_square_rhs(833, 545, 323);s.store_mul_div_from_scalar_lhs_ad_mixed_ia(834, 2.0, 545, A::square(s.ad_value(323)));s.store_add_scaled_inputs_product_indices(835, 831, 1.0, 227, (-1.0), 130, 514, (-1.0));s.store_offset_mul(836, 834, 835, 1.0);s.store_scaled_offset(838, 834, 1.0, 2.0);}
        s.b[847] = ((s.v[836] < (1e-50 + s.v[838])) && (s.v[838] >= 0.0));s.store_scalar(847, if s.b[847] { 1.0 } else { 0.0 });
        if (((((s.b[733] && s.b[800]) && s.b[801]) && (!s.b[843])) && (!s.b[844])) && s.b[847]) {s.store_sub_offset_lhs(44, 838, 1e-50, 836);s.store_square(49, 44);s.store_square(50, 838);s.store_scalar(51, 1.0);s.store_scalar(52, 1.0);}
        let (t3,) = {
    if (((((s.b[733] && s.b[800]) && s.b[801]) && (!s.b[843])) && (!s.b[844])) && s.b[847]) {
        (0.0,)
    } else {
        (s.v[54],)
    }
};
        s.store_scalar(54, t3);
        let (t4,) = {
    if (((((s.b[733] && s.b[800]) && s.b[801]) && (!s.b[843])) && (!s.b[844])) && s.b[847]) {
        (0.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t4);
        if (((((s.b[733] && s.b[800]) && s.b[801]) && (!s.b[843])) && (!s.b[844])) && s.b[847]) {s.store_scalar(48, 0.0);s.store_scalar(53, 0.0);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_add(48, 51, 52);s.copy_ad(53, 48);}
        s.b[848] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));s.store_scalar(848, if s.b[848] { 1.0 } else { 0.0 });s.b[849] = (4.0 == 1.0);s.store_scalar(849, if s.b[849] { 1.0 } else { 0.0 });
        let (t5,) = {
    if (((((((s.b[733] && s.b[800]) && s.b[801]) && (!s.b[843])) && (!s.b[844])) && s.b[847]) && s.b[848]) && s.b[849]) {
        (1.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t5);s.b[850] = (4.0 == 2.0);s.store_scalar(850, if s.b[850] { 1.0 } else { 0.0 });
        let (t6,) = {
    if ((((((((s.b[733] && s.b[800]) && s.b[801]) && (!s.b[843])) && (!s.b[844])) && s.b[847]) && s.b[848]) && (!s.b[849])) && s.b[850]) {
        (2.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t6);s.b[851] = (4.0 == 4.0);s.store_scalar(851, if s.b[851] { 1.0 } else { 0.0 });
        let (t7,) = {
    if (((((((((s.b[733] && s.b[800]) && s.b[801]) && (!s.b[843])) && (!s.b[844])) && s.b[847]) && s.b[848]) && (!s.b[849])) && (!s.b[850])) && s.b[851]) {
        (3.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t7);s.b[852] = (4.0 == 8.0);s.store_scalar(852, if s.b[852] { 1.0 } else { 0.0 });
        let (t8,) = {
    if ((((((((((s.b[733] && s.b[800]) && s.b[801]) && (!s.b[843])) && (!s.b[844])) && s.b[847]) && s.b[848]) && (!s.b[849])) && (!s.b[850])) && (!s.b[851])) && s.b[852]) {
        (4.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t8);
        let (t9,) = {
    if ((((((s.b[733] && s.b[800]) && s.b[801]) && (!s.b[843])) && (!s.b[844])) && s.b[847]) && s.b[848]) {
        (0.0,)
    } else {
        (s.v[54],)
    }
};
        s.store_scalar(54, t9);let mut td: usize = 0;
        while {
            let tc: f64 = if (((((((s.b[733] && s.b[800]) && s.b[801]) && (!s.b[843])) && (!s.b[844])) && s.b[847]) && s.b[848]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            tc != 0.0
        } {
            td += 1;
            if td > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", td, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((((((s.b[733] && s.b[800]) && s.b[801]) && (!s.b[843])) && (!s.b[844])) && s.b[847]) && s.b[848]) {s.store_sqrt(53, 53);}
            let (tb,) = {
    if ((((((s.b[733] && s.b[800]) && s.b[801]) && (!s.b[843])) && (!s.b[844])) && s.b[847]) && s.b[848]) {
        let ta: f64 = (s.v[54] + 1.0);
        (ta,)
    } else {
        (s.v[54],)
    }
};
            s.store_scalar(54, tb);
        }
        if ((((((s.b[733] && s.b[800]) && s.b[801]) && (!s.b[843])) && (!s.b[844])) && s.b[847]) && (!s.b[848])) {s.store_powf(53, 53, (1.0 / (2.0 * 4.0)));}
        if (((((s.b[733] && s.b[800]) && s.b[801]) && (!s.b[843])) && (!s.b[844])) && s.b[847]) {s.store_div_from_scalar(53, 1.0, 53);s.store_mul3_lhs(43, 44, 838, 53);s.store_sub_offset_lhs(836, 838, 1e-50, 43);}
        if (((((s.b[733] && s.b[800]) && s.b[801]) && (!s.b[843])) && (!s.b[844])) && (!s.b[847])) {
        }
        if ((((s.b[733] && s.b[800]) && s.b[801]) && (!s.b[843])) && (!s.b[844])) {
            if (s.v[836] <= 0.0) {
                s.store_scalar(836, 0.0);
            } else {
                s.store_sqrt(836, 836);
            }
        }
        if ((((s.b[733] && s.b[800]) && s.b[801]) && (!s.b[843])) && (!s.b[844])) {s.store_add_mul_sub_from_scalar_rhs_indices(841, 831, 833, 1.0, 836);s.store_div_from_scalar_offset_input(832, s.v[100], 131, s.v[100]);s.store_add_scaled_product_mixed_aii(842, A::scale_offset(s.ad_value(173), p[122], s.v[176]), 1.0, 832, 841, (-1.0));s.store_sqrt_square_offset(44, 842, ((4.0 * 0.001) * 0.001));s.store_offset_add_scaled_inputs_indices(842, 842, 0.5, 44, 0.5, (1e-10 * 0.001));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_27(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        s.b[853] = (s.v[842] < 0.0);s.store_scalar(853, if s.b[853] { 1.0 } else { 0.0 });
        if (((((s.b[733] && s.b[800]) && s.b[801]) && (!s.b[843])) && (!s.b[844])) && s.b[853]) {s.store_scalar(842, 0.0);}
        if (((s.b[733] && s.b[800]) && s.b[801]) && (!s.b[843])) {s.store_offset(842, 842, 1e-50);s.store_ad_value(832, A::exp_div_scaled_inputs(s.ad_value(133), -1.0, s.ad_value(842), 1.0));s.store_mul_product3_indices(263, 832, 132, 842, 394, 1.0);}
        s.b[861] = (p[26] == 1.0);s.store_scalar(861, if s.b[861] { 1.0 } else { 0.0 });
        if (((s.b[733] && s.b[800]) && s.b[801]) && s.b[861]) {s.store_mul_ad_affine_product_rhs(854, 736, s.ad_value(107), A::exp_scaled_input(s.ad_value(225), (-p[141])), 1.6021918e-19, 0.0);s.store_offset_scaled(855, 544, (((((36.0 * 1e-7) / 0.0001)) as f64).sqrt() * 13.0), ((((((13.0 * 1e-7) / 0.0001)) as f64).sqrt() * 36.0) * (1e20 / 1e-6)));s.store_div_scalar_by_product_indices(856, (((((13.0 * 1e-7) / 0.0001)) as f64).sqrt() * ((((36.0 * 1e-7) / 0.0001)) as f64).sqrt()), 854, 855, 1.0);s.store_mul_add_lhs(567, 263, 447, 856);s.store_mul_scaled_ln_offset_rhs(857, 227, p[140], 567, 1.0);s.store_sqrt_mul_scaled_lhs(858, 544, ((2.0 * 1.034943e-10) * 1.6021918e-19), 227);s.store_sqrt_ad(859, A::add_scaled_product(A::offset(A::exp(A::mul_scaled_lhs(s.ad_value(225), -1.0, A::sub(s.ad_value(395), s.ad_value(857)))), (-1.0)), 1.0, s.ad_value(225), A::sub(s.ad_value(395), s.ad_value(857)), 1.0));s.store_sqrt_ad(860, A::add_scaled_product(A::offset(A::exp(A::mul_scaled_lhs(s.ad_value(225), -1.0, s.ad_value(395))), (-1.0)), 1.0, s.ad_value(225), s.ad_value(395), 1.0));s.store_mul_sub_scaled_inputs_rhs_indices(393, 858, 859, -1.0, 860, -1.0);}
        if ((((s.b[733] && s.b[800]) && s.b[801]) && s.b[861]) && (p[37] != 0.0)) {s.store_div_from_scalar_offset_input(398, p[138], 263, p[139]);s.store_mul(397, 398, 323);s.copy_ad(396, 393);s.store_scaled_voltage(596, ctx, nodes, Some(17), None, (1e-9 / 0.0001));s.copy_ad(393, 596);s.store_div_scaled_inputs2_indices(592, 596, 1.0, 396, (-1.0), 397, 1.0);}
        if (((s.b[733] && s.b[800]) && s.b[801]) && (!s.b[861])) {s.store_scalar(393, 0.0);}
        if ((s.b[733] && s.b[800]) && (!s.b[801])) {s.store_scalar(263, 0.0);s.store_scalar(393, 0.0);}
        if (s.b[733] && (!s.b[800])) {s.store_scalar(263, 0.0);s.store_scalar(393, 0.0);}
        if s.b[733] {s.copy_ad(343, 349);s.copy_ad(344, 350);s.copy_ad(345, 351);}
        let (te,) = {
    if s.b[733] {
        (0.0,)
    } else {
        (s.v[430],)
    }
};
        s.store_scalar(430, te);
        if s.b[733] {s.store_scalar(611, 0.0);s.store_scalar(167, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_28(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
    ) {
        let mut t2c: usize = 0;
        while {
            let t2b: f64 = if (s.b[733] && (s.v[167] <= s.v[57])) { 1.0 } else { 0.0 };
            t2b != 0.0
        } {
            t2c += 1;
            if t2c > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", t2c, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if s.b[733] {s.store_sub(863, 351, 475);s.store_mul(862, 225, 863);s.store_exp_neg_input(327, 862);}
            s.b[897] = (s.v[863] < (-1e-9));s.store_scalar(897, if s.b[897] { 1.0 } else { 0.0 });
            if (s.b[733] && s.b[897]) {s.store_mul_sqrt_mixed_ia(357, 474, A::offset(A::add(s.ad_value(327), s.ad_value(862)), (-1.0)));s.store_div_scaled_offset_numerator_indices(869, 327, (-s.v[122]), s.v[122], 357, 1.0);}
            s.b[898] = (s.v[863] > 1e-9);s.store_scalar(898, if s.b[898] { 1.0 } else { 0.0 });
            if ((s.b[733] && (!s.b[897])) && s.b[898]) {s.store_exp(864, 862);s.store_mul_scaled_sqrt_ad_rhs(357, 474, -1.0, A::add_scaled_offset_product_rhs(A::offset(A::add(s.ad_value(327), s.ad_value(862)), (-1.0)), 1.0, s.ad_value(239), A::add(s.ad_value(864), s.ad_value(862)), (-1.0), 1.0));s.store_div_mixed_ai(869, A::add_scaled_sub_value_product(1.0, s.ad_value(327), s.v[122], s.ad_value(239), A::offset(s.ad_value(864), 1.0), s.v[122]), 357);}
            if ((s.b[733] && (!s.b[897])) && (!s.b[898])) {s.store_mul_scale_offset_indices(357, 862, 474, -1.0, 0.0);s.store_mul_scale_offset_indices(869, 225, 474, -1.0, 0.0);}
            if s.b[733] {s.copy_ad(361, 369);s.store_mul(862, 225, 349);s.store_exp_mul(867, 225, 349);s.store_scalar(865, 1.0);s.store_sqrt_ad(866, A::add_scaled_product(A::div_scaled_product(s.ad_value(361), s.ad_value(361), 1.0, A::square(s.ad_value(238)), 1.0), 1.0, s.ad_value(379), A::add_scaled_inputs3(s.ad_value(867), 1.0, s.ad_value(862), 1.0, s.ad_value(865), -1.0), 2.0));s.store_div_scaled_product3_mixed_iiai(896, 225, 379, A::offset(s.ad_value(867), 1.0), 2.0, 866, 2.0);s.store_add_scaled_product_indices(355, 361, (-1.0), 238, 866, -1.0);s.store_mul_scale_offset_indices(868, 896, 238, -1.0, 0.0);s.store_div_scaled_inputs2_indices(863, 350, 1.0, 349, (-1.0), 738, 1.0);s.store_mul(862, 225, 863);}
            s.b[899] = ((-s.v[862]) >= 500.0);s.store_scalar(899, if s.b[899] { 1.0 } else { 0.0 });
            if (s.b[733] && s.b[899]) {s.store_scaled_offset_ad(327, A::sub_from_scalar(1.0, s.ad_value(862)), (-500.0), 1.403592217853e217);s.store_scalar(333, 1.403592217853e217);}
            if (s.b[733] && (!s.b[899])) {s.store_neg(44, 862);s.store_scalar(327, 1.0);}
            let mut t1c: usize = 0;
            while {
                let t1b: f64 = if ((s.b[733] && (!s.b[899])) && (s.v[44] >= 60.0)) { 1.0 } else { 0.0 };
                t1b != 0.0
            } {
                t1c += 1;
                if t1c > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", t1c, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
                if (s.b[733] && (!s.b[899])) {s.store_scale(327, 327, 1.14200738981568e26);s.store_offset(44, 44, (-60.0));}
            }
            if (s.b[733] && (!s.b[899])) {s.store_mul_exp_rhs(327, 327, 44);s.copy_ad(333, 327);}
            if s.b[733] {s.store_exp_neg_input(327, 862);s.store_sqrt_offset_ad(864, A::add(s.ad_value(327), s.ad_value(862)), (-1.0));}
            s.b[900] = (s.v[863] < (-1e-9));s.store_scalar(900, if s.b[900] { 1.0 } else { 0.0 });
            if (s.b[733] && s.b[900]) {s.store_mul(363, 238, 864);s.store_div_scaled_product3_by_product_mixed_iiaii(364, 238, 225, A::sub_from_scalar(1.0, s.ad_value(333)), 1.0, 864, 738, 2.0);s.store_neg(365, 364);}
            s.b[901] = (s.v[863] > 1e-9);s.store_scalar(901, if s.b[901] { 1.0 } else { 0.0 });
            if ((s.b[733] && (!s.b[900])) && s.b[901]) {s.store_mul_scale_offset_indices(363, 864, 238, -1.0, 0.0);s.store_div_scaled_product3_by_product_mixed_iiaii(364, 238, 225, A::sub_from_scalar(1.0, s.ad_value(333)), -1.0, 864, 738, 2.0);s.store_neg(365, 364);}
            if ((s.b[733] && (!s.b[900])) && (!s.b[901])) {s.store_scaled_mul(363, 238, 862, (-0.7071067811865476));s.store_scaled_mul(364, 238, 225, (-0.7071067811865476));s.store_neg(365, 364);}
            s.b[902] = ((s.v[363] > (-(-s.v[406]))) && ((-s.v[406]) >= 0.0));s.store_scalar(902, if s.b[902] { 1.0 } else { 0.0 });
            if (s.b[733] && s.b[902]) {s.store_add_scaled_inputs(44, 363, 1.0, 406, -1.0);s.store_square(49, 44);s.store_scaled_mul(50, 406, 406, 1.0);s.store_scalar(51, 1.0);s.store_scalar(52, 1.0);}
            let (t1d,) = {
    if (s.b[733] && s.b[902]) {
        (0.0,)
    } else {
        (s.v[54],)
    }
};
            s.store_scalar(54, t1d);
            let (t1e,) = {
    if (s.b[733] && s.b[902]) {
        (0.0,)
    } else {
        (s.v[55],)
    }
};
            s.store_scalar(55, t1e);
            if (s.b[733] && s.b[902]) {s.store_scalar(48, 0.0);s.store_scalar(53, 0.0);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_add(48, 51, 52);s.copy_ad(53, 48);}
            s.b[903] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(903, if s.b[903] { 1.0 } else { 0.0 });s.b[904] = (2.0 == 1.0);s.store_scalar(904, if s.b[904] { 1.0 } else { 0.0 });
            let (t1f,) = {
    if (((s.b[733] && s.b[902]) && s.b[903]) && s.b[904]) {
        (1.0,)
    } else {
        (s.v[55],)
    }
};
            s.store_scalar(55, t1f);s.b[905] = (2.0 == 2.0);s.store_scalar(905, if s.b[905] { 1.0 } else { 0.0 });
            let (t20,) = {
    if ((((s.b[733] && s.b[902]) && s.b[903]) && (!s.b[904])) && s.b[905]) {
        (2.0,)
    } else {
        (s.v[55],)
    }
};
            s.store_scalar(55, t20);s.b[906] = (2.0 == 4.0);s.store_scalar(906, if s.b[906] { 1.0 } else { 0.0 });
            let (t21,) = {
    if (((((s.b[733] && s.b[902]) && s.b[903]) && (!s.b[904])) && (!s.b[905])) && s.b[906]) {
        (3.0,)
    } else {
        (s.v[55],)
    }
};
            s.store_scalar(55, t21);s.b[907] = (2.0 == 8.0);s.store_scalar(907, if s.b[907] { 1.0 } else { 0.0 });
            let (t22,) = {
    if ((((((s.b[733] && s.b[902]) && s.b[903]) && (!s.b[904])) && (!s.b[905])) && (!s.b[906])) && s.b[907]) {
        (4.0,)
    } else {
        (s.v[55],)
    }
};
            s.store_scalar(55, t22);
            let (t23,) = {
    if ((s.b[733] && s.b[902]) && s.b[903]) {
        (0.0,)
    } else {
        (s.v[54],)
    }
};
            s.store_scalar(54, t23);let mut t27: usize = 0;
            while {
                let t26: f64 = if (((s.b[733] && s.b[902]) && s.b[903]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
                t26 != 0.0
            } {
                t27 += 1;
                if t27 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", t27, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
                if ((s.b[733] && s.b[902]) && s.b[903]) {s.store_sqrt(53, 53);}
                let (t25,) = {
    if ((s.b[733] && s.b[902]) && s.b[903]) {
        let t24: f64 = (s.v[54] + 1.0);
        (t24,)
    } else {
        (s.v[54],)
    }
};
                s.store_scalar(54, t25);
            }
            if ((s.b[733] && s.b[902]) && (!s.b[903])) {s.store_powf(53, 53, (1.0 / (2.0 * 2.0)));}
            if (s.b[733] && s.b[902]) {s.store_div_from_scalar(53, 1.0, 53);s.store_mul3_affine_lhs(895, 44, 406, -1.0, 0.0, 53);s.store_div_scaled_product3_indices(327, 406, 52, 53, -1.0, 48, 1.0);s.store_add_scaled_inputs_mixed_ai(363, A::neg(s.ad_value(406)), -1.0, 895, 1.0);}
            if (s.b[733] && s.b[902]) {
            }
            if (s.b[733] && (!s.b[902])) {
            }
            if (s.b[733] && (!s.b[902])) {s.store_scalar(327, 1.0);}
            if s.b[733] {s.store_mul(364, 364, 327);s.store_mul(365, 365, 327);}
            s.b[908] = ((s.v[363] < ((s.v[341] - s.v[361]) + (-(s.v[341] - s.v[361])))) && ((-(s.v[341] - s.v[361])) >= 0.0));s.store_scalar(908, if s.b[908] { 1.0 } else { 0.0 });
            if (s.b[733] && s.b[908]) {s.store_sub_add_scaled_inputs4_lhs_indices(44, 341, 1.0, 361, (-1.0), 341, -1.0, 361, 1.0, 363);s.store_square(49, 44);s.store_scaled_mul_ad(50, A::sub(s.ad_value(341), s.ad_value(361)), A::sub(s.ad_value(341), s.ad_value(361)), 1.0);s.store_scalar(51, 1.0);s.store_scalar(52, 1.0);}
            let (t28,) = {
    if (s.b[733] && s.b[908]) {
        (0.0,)
    } else {
        (s.v[54],)
    }
};
            s.store_scalar(54, t28);
            let (t29,) = {
    if (s.b[733] && s.b[908]) {
        (0.0,)
    } else {
        (s.v[55],)
    }
};
            s.store_scalar(55, t29);
            if (s.b[733] && s.b[908]) {s.store_scalar(48, 0.0);s.store_scalar(53, 0.0);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_add(48, 51, 52);s.copy_ad(53, 48);}
            s.b[909] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(909, if s.b[909] { 1.0 } else { 0.0 });s.b[910] = (2.0 == 1.0);s.store_scalar(910, if s.b[910] { 1.0 } else { 0.0 });
            let (t2a,) = {
    if (((s.b[733] && s.b[908]) && s.b[909]) && s.b[910]) {
        (1.0,)
    } else {
        (s.v[55],)
    }
};
            s.store_scalar(55, t2a);s.b[911] = (2.0 == 2.0);s.store_scalar(911, if s.b[911] { 1.0 } else { 0.0 });
            let (tf,) = {
    if ((((s.b[733] && s.b[908]) && s.b[909]) && (!s.b[910])) && s.b[911]) {
        (2.0,)
    } else {
        (s.v[55],)
    }
};
            s.store_scalar(55, tf);s.b[912] = (2.0 == 4.0);s.store_scalar(912, if s.b[912] { 1.0 } else { 0.0 });
            let (t10,) = {
    if (((((s.b[733] && s.b[908]) && s.b[909]) && (!s.b[910])) && (!s.b[911])) && s.b[912]) {
        (3.0,)
    } else {
        (s.v[55],)
    }
};
            s.store_scalar(55, t10);s.b[913] = (2.0 == 8.0);s.store_scalar(913, if s.b[913] { 1.0 } else { 0.0 });
            let (t11,) = {
    if ((((((s.b[733] && s.b[908]) && s.b[909]) && (!s.b[910])) && (!s.b[911])) && (!s.b[912])) && s.b[913]) {
        (4.0,)
    } else {
        (s.v[55],)
    }
};
            s.store_scalar(55, t11);
            let (t12,) = {
    if ((s.b[733] && s.b[908]) && s.b[909]) {
        (0.0,)
    } else {
        (s.v[54],)
    }
};
            s.store_scalar(54, t12);let mut t16: usize = 0;
            while {
                let t15: f64 = if (((s.b[733] && s.b[908]) && s.b[909]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
                t15 != 0.0
            } {
                t16 += 1;
                if t16 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", t16, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
                if ((s.b[733] && s.b[908]) && s.b[909]) {s.store_sqrt(53, 53);}
                let (t14,) = {
    if ((s.b[733] && s.b[908]) && s.b[909]) {
        let t13: f64 = (s.v[54] + 1.0);
        (t13,)
    } else {
        (s.v[54],)
    }
};
                s.store_scalar(54, t14);
            }
            if ((s.b[733] && s.b[908]) && (!s.b[909])) {s.store_powf(53, 53, (1.0 / (2.0 * 2.0)));}
            if (s.b[733] && s.b[908]) {s.store_div_from_scalar(53, 1.0, 53);s.store_mul_ad_affine_product_lhs(895, s.ad_value(44), A::sub(s.ad_value(341), s.ad_value(361)), -1.0, 0.0, 53);s.store_div_scaled_product3_mixed_aiii(327, A::sub(s.ad_value(341), s.ad_value(361)), 52, 53, -1.0, 48, 1.0);s.store_sub_add_scaled_inputs4_lhs_indices(363, 341, 1.0, 361, (-1.0), 341, -1.0, 361, 1.0, 895);}
            if (s.b[733] && s.b[908]) {
            }
            if (s.b[733] && (!s.b[908])) {
            }
            if (s.b[733] && (!s.b[908])) {s.store_scalar(327, 1.0);}
            if s.b[733] {s.store_mul(365, 365, 327);s.store_mul(364, 364, 327);s.store_add(356, 361, 363);}
            s.b[914] = (s.v[430] == 1.0);s.store_scalar(914, if s.b[914] { 1.0 } else { 0.0 });
            if (s.b[733] && s.b[914]) {s.copy_ad(611, 167);s.store_scalar(167, s.v[57]);}
            if (s.b[733] && (!s.b[914])) {s.store_add_scaled_inputs_product_mixed_iiia(873, 349, 1.0, 178, (-1.0), 324, A::add(A::add_scaled_inputs4(s.ad_value(357), 1.0, s.ad_value(361), 1.0, s.ad_value(355), 1.0, s.ad_value(363), 1.0), s.ad_value(393)), (-1.0));s.store_sub_from_scalar_scaled_mul_mixed_ia(874, 1.0, 324, A::add(s.ad_value(868), s.ad_value(365)), 1.0);s.store_mul_scale_offset_indices(875, 364, 324, -1.0, 0.0);s.store_mul_scale_offset_indices(876, 869, 324, -1.0, 0.0);s.store_add_scaled_product_mixed_iia(863, 349, 1.0, 735, A::add_scaled_inputs(s.ad_value(341), 0.5, s.ad_value(357), 1.0), 1.0);s.store_mul(865, 735, 869);s.store_sub(877, 350, 863);s.store_scalar(878, (-1.0));s.store_scalar(879, 1.0);s.store_neg(880, 865);s.store_add_scaled_inputs3_indices(881, 351, 1.0, 350, (-1.0), 357, (-s.v[94]));s.store_scalar(882, (-1.0));s.store_sub_from_scalar_scaled_input(883, 1.0, 869, s.v[94]);s.store_add_scaled_inputs4(884, A::mul3(s.ad_value(874), s.ad_value(879), s.ad_value(883)), 1.0, A::mul3(s.ad_value(874), s.ad_value(880), s.ad_value(882)), (-1.0), A::mul3(s.ad_value(875), s.ad_value(878), s.ad_value(883)), -1.0, A::mul3(s.ad_value(876), s.ad_value(878), s.ad_value(882)), 1.0);s.store_div_from_scalar_offset_input(885, 1.0, 884, 1e-50);s.store_add_scaled_products_indices(886, 879, 883, 1.0, 880, 882, (-1.0));s.store_add_scaled_products_indices(887, 876, 882, 1.0, 875, 883, (-1.0));s.store_add_scaled_products_indices(888, 875, 880, 1.0, 876, 879, (-1.0));s.store_mul_scale_offset_indices(889, 883, 878, -1.0, 0.0);s.store_mul(890, 874, 883);s.store_add_scaled_products_indices(891, 876, 878, 1.0, 874, 880, (-1.0));s.store_primal_mul(892, 878, 882);s.store_mul_scale_offset_indices(893, 882, 874, -1.0, 0.0);s.store_add_scaled_products_indices(894, 874, 879, 1.0, 875, 878, (-1.0));s.store_mul_add_scaled_products3_indices_rhs(870, 885, 886, 873, -1.0, 887, 877, -1.0, 888, 881, -1.0);s.store_mul_add_scaled_products3_indices_rhs(871, 885, 889, 873, -1.0, 890, 877, -1.0, 891, 881, -1.0);s.store_mul_add_scaled_products3_indices_rhs(872, 885, 892, 873, -1.0, 893, 877, -1.0, 894, 881, -1.0);s.store_abs(863, 870);}
            s.b[915] = (s.v[863] < ((s.v[871]) as f64).abs());s.store_scalar(915, if s.b[915] { 1.0 } else { 0.0 });
            if ((s.b[733] && (!s.b[914])) && s.b[915]) {s.store_abs(863, 871);}
            s.b[916] = (s.v[863] < ((s.v[872]) as f64).abs());s.store_scalar(916, if s.b[916] { 1.0 } else { 0.0 });
            if ((s.b[733] && (!s.b[914])) && s.b[916]) {s.store_abs(863, 872);}
            if (s.b[733] && (!s.b[914])) {s.store_scalar(407, 1.0);}
            s.b[917] = (s.v[167] > 80.0);s.store_scalar(917, if s.b[917] { 1.0 } else { 0.0 });
            if ((s.b[733] && (!s.b[914])) && s.b[917]) {s.store_scalar(407, 125.0);}
            s.b[918] = (s.v[167] > 40.0);s.store_scalar(918, if s.b[918] { 1.0 } else { 0.0 });
            if (((s.b[733] && (!s.b[914])) && (!s.b[917])) && s.b[918]) {s.store_scalar(407, 125.0);}
            s.b[919] = (s.v[167] > 20.0);s.store_scalar(919, if s.b[919] { 1.0 } else { 0.0 });
            if ((((s.b[733] && (!s.b[914])) && (!s.b[917])) && (!s.b[918])) && s.b[919]) {s.store_scalar(407, 25.0);}
            s.b[920] = (s.v[167] > 10.0);s.store_scalar(920, if s.b[920] { 1.0 } else { 0.0 });
            if (((((s.b[733] && (!s.b[914])) && (!s.b[917])) && (!s.b[918])) && (!s.b[919])) && s.b[920]) {s.store_scalar(407, 5.0);}
            s.b[921] = (s.v[863] > (0.1 / s.v[407]));s.store_scalar(921, if s.b[921] { 1.0 } else { 0.0 });
            if ((s.b[733] && (!s.b[914])) && s.b[921]) {s.store_mul_mixed_ia(870, 870, A::div_scalar_by_product(0.1, s.ad_value(407), s.ad_value(863), 1.0));s.store_mul_mixed_ia(871, 871, A::div_scalar_by_product(0.1, s.ad_value(407), s.ad_value(863), 1.0));s.store_mul_mixed_ia(872, 872, A::div_scalar_by_product(0.1, s.ad_value(407), s.ad_value(863), 1.0));}
            if (s.b[733] && (!s.b[914])) {s.store_add(349, 349, 870);s.store_add(350, 350, 871);s.store_add(351, 351, 872);}
            let (t19,) = {
    if (s.b[733] && (!s.b[914])) {
        let t17: f64 = (5e-12 * s.v[407]);let t18: f64 = t17;
        (t18,)
    } else {
        (s.v[408],)
    }
};
            s.store_scalar(408, t19);s.b[922] = (s.v[863] < s.v[408]);s.store_scalar(922, if s.b[922] { 1.0 } else { 0.0 });
            let (t1a,) = {
    if ((s.b[733] && (!s.b[914])) && s.b[922]) {
        (1.0,)
    } else {
        (s.v[430],)
    }
};
            s.store_scalar(430, t1a);
            if s.b[733] {s.store_primal_offset(167, 167, 1.0);}
        }
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_29(
        s: &mut Scratch,
    ) {
        if s.b[733] {
            if (s.v[611] > 0.0) {
                s.copy_ad(167, 611);
            } else {
            }
        }
        s.b[923] = (s.v[430] == 0.0);s.store_scalar(923, if s.b[923] { 1.0 } else { 0.0 });
        if (s.b[733] && s.b[923]) {s.copy_ad(349, 343);s.copy_ad(350, 344);s.copy_ad(351, 345);}
        if s.b[733] {s.copy_ad(161, 349);s.store_neg(244, 355);}
        s.b[924] = (s.v[244] <= 1e-50);s.store_scalar(924, if s.b[924] { 1.0 } else { 0.0 });
        if (s.b[733] && s.b[924]) {s.store_scalar(244, 1e-50);}
        if s.b[733] {s.store_mul(192, 244, 324);}
        s.b[925] = ((s.v[349] <= 0.0) && (s.v[86] != 0.0));s.store_scalar(925, if s.b[925] { 1.0 } else { 0.0 });
        if (s.b[733] && s.b[925]) {s.store_scale(327, 108, (-s.v[98]));s.copy_ad(362, 369);s.copy_ad(366, 363);s.store_add(359, 362, 366);s.store_scaled_add(437, 359, 356, (-0.5));s.store_mul(196, 327, 437);s.store_scale(477, 196, 0.5);s.store_scale(476, 196, (1.0 - 0.5));s.store_scalar(197, 0.0);s.store_scaled_mul(392, 357, 108, s.v[98]);s.store_scalar(198, 0.0);s.store_scalar(199, 0.0);s.store_scalar(192, 0.0);}
        let (t2d,) = {
    if (s.b[733] && s.b[925]) {
        (1.0,)
    } else {
        (s.v[145],)
    }
};
        s.store_scalar(145, t2d);
        if (s.b[733] && s.b[925]) {s.copy_ad(352, 349);s.copy_ad(353, 350);s.copy_ad(354, 351);s.copy_ad(360, 357);s.copy_ad(162, 161);s.copy_ad(314, 162);}
        if (s.b[733] && (!s.b[925])) {s.copy_ad(453, 157);s.store_scalar(932, 1e-50);s.store_div_square_rhs(927, 545, 323);s.store_offset_mul_ad(929, A::div_from_scalar(2.0, s.ad_value(927)), A::sub(s.ad_value(159), s.ad_value(932)), 1.0);s.store_offset_div_from_scalar_ad(332, 2.0, s.ad_value(927), 1.0);}
        s.b[933] = ((s.v[929] < s.v[332]) && (s.v[332] >= 0.0));s.store_scalar(933, if s.b[933] { 1.0 } else { 0.0 });
        if ((s.b[733] && (!s.b[925])) && s.b[933]) {s.store_sub(44, 332, 929);s.store_square(49, 44);s.store_square(50, 332);s.store_scalar(51, 1.0);s.store_scalar(52, 1.0);}
        let (t2e,) = {
    if ((s.b[733] && (!s.b[925])) && s.b[933]) {
        (0.0,)
    } else {
        (s.v[54],)
    }
};
        s.store_scalar(54, t2e);
        let (t2f,) = {
    if ((s.b[733] && (!s.b[925])) && s.b[933]) {
        (0.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t2f);
        if ((s.b[733] && (!s.b[925])) && s.b[933]) {s.store_scalar(48, 0.0);s.store_scalar(53, 0.0);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_add(48, 51, 52);s.copy_ad(53, 48);}
        s.b[934] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));s.store_scalar(934, if s.b[934] { 1.0 } else { 0.0 });s.b[935] = (4.0 == 1.0);s.store_scalar(935, if s.b[935] { 1.0 } else { 0.0 });
        let (t30,) = {
    if ((((s.b[733] && (!s.b[925])) && s.b[933]) && s.b[934]) && s.b[935]) {
        (1.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t30);s.b[936] = (4.0 == 2.0);s.store_scalar(936, if s.b[936] { 1.0 } else { 0.0 });
        let (t31,) = {
    if (((((s.b[733] && (!s.b[925])) && s.b[933]) && s.b[934]) && (!s.b[935])) && s.b[936]) {
        (2.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t31);s.b[937] = (4.0 == 4.0);s.store_scalar(937, if s.b[937] { 1.0 } else { 0.0 });
        let (t32,) = {
    if ((((((s.b[733] && (!s.b[925])) && s.b[933]) && s.b[934]) && (!s.b[935])) && (!s.b[936])) && s.b[937]) {
        (3.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t32);s.b[938] = (4.0 == 8.0);s.store_scalar(938, if s.b[938] { 1.0 } else { 0.0 });
        let (t33,) = {
    if (((((((s.b[733] && (!s.b[925])) && s.b[933]) && s.b[934]) && (!s.b[935])) && (!s.b[936])) && (!s.b[937])) && s.b[938]) {
        (4.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t33);
        let (t34,) = {
    if (((s.b[733] && (!s.b[925])) && s.b[933]) && s.b[934]) {
        (0.0,)
    } else {
        (s.v[54],)
    }
};
        s.store_scalar(54, t34);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_30(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
    ) {
        let mut t38: usize = 0;
        while {
            let t37: f64 = if ((((s.b[733] && (!s.b[925])) && s.b[933]) && s.b[934]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            t37 != 0.0
        } {
            t38 += 1;
            if t38 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", t38, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if (((s.b[733] && (!s.b[925])) && s.b[933]) && s.b[934]) {s.store_sqrt(53, 53);}
            let (t36,) = {
    if (((s.b[733] && (!s.b[925])) && s.b[933]) && s.b[934]) {
        let t35: f64 = (s.v[54] + 1.0);
        (t35,)
    } else {
        (s.v[54],)
    }
};
            s.store_scalar(54, t36);
        }
        if (((s.b[733] && (!s.b[925])) && s.b[933]) && (!s.b[934])) {s.store_powf(53, 53, (1.0 / (2.0 * 4.0)));}
        if ((s.b[733] && (!s.b[925])) && s.b[933]) {s.store_div_from_scalar(53, 1.0, 53);s.store_mul3_lhs(43, 44, 332, 53);s.store_sub(929, 332, 43);}
        if ((s.b[733] && (!s.b[925])) && (!s.b[933])) {
        }
        if (s.b[733] && (!s.b[925])) {s.store_sqrt(928, 929);s.store_add_mul_sub_from_scalar_rhs_indices(932, 159, 927, 1.0, 928);s.store_sqrt_square_offset(44, 932, ((4.0 * 0.01) * 0.01));s.store_offset_add_scaled_inputs_indices(932, 932, 0.5, 44, 0.5, (1e-10 * 0.01));}
        s.b[939] = (s.v[932] < 0.0);s.store_scalar(939, if s.b[939] { 1.0 } else { 0.0 });
        if ((s.b[733] && (!s.b[925])) && s.b[939]) {s.store_scalar(932, 0.0);}
        if (s.b[733] && (!s.b[925])) {s.store_div(926, 157, 932);s.store_pow_offset_rhs(927, 926, 138, (-1.0));s.store_mul(931, 927, 926);s.store_offset(928, 931, 1.0);s.store_pow_ad(929, s.ad_value(928), A::offset(A::div_from_scalar(1.0, s.ad_value(138)), (-1.0)));s.store_mul(930, 929, 928);s.store_div(452, 157, 930);s.copy_ad(157, 452);}
        s.b[940] = (s.v[157] < 0.0);s.store_scalar(940, if s.b[940] { 1.0 } else { 0.0 });
        if ((s.b[733] && (!s.b[925])) && s.b[940]) {s.copy_ad(162, 161);s.store_sub(164, 162, 161);s.copy_ad(352, 162);s.copy_ad(353, 350);s.copy_ad(354, 351);}
        let (t39,) = {
    if ((s.b[733] && (!s.b[925])) && s.b[940]) {
        (1.0,)
    } else {
        (s.v[430],)
    }
};
        s.store_scalar(430, t39);s.b[941] = (s.v[144] >= 1.0);s.store_scalar(941, if s.b[941] { 1.0 } else { 0.0 });
        if (((s.b[733] && (!s.b[925])) && (!s.b[940])) && s.b[941]) {s.store_scalar(352, s.v[622]);s.store_scalar(353, s.v[623]);s.store_scalar(354, s.v[624]);}
        if (((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) {
            if ((s.v[163] - s.v[349]) >= 0.0) {
                s.store_sub(166, 163, 349);
            } else {
                s.store_scalar(166, 0.0);
            }
        }
        if (((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) {s.store_offset_sub_scaled_inputs_indices(44, 166, (1.0 + 0.3), 157, 1.0, (-0.03));s.store_scale(45, 166, ((1.0 + 0.3) * (4.0 * 0.03)));}
        if (((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }
        if (((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) {s.store_sqrt_square_add(45, 44, 45);s.store_add_scaled_inputs3_indices(165, 166, (1.0 + 0.3), 44, (-0.5), 45, (-0.5));}
        if (((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) {
            if (s.v[165] <= s.v[166]) {
            } else {
                s.copy_ad(165, 166);
            }
        }
        s.b[942] = (s.v[165] < 0.0);s.store_scalar(942, if s.b[942] { 1.0 } else { 0.0 });
        if ((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && s.b[942]) {s.store_scalar(165, 0.0);}
        s.b[943] = (s.v[165] > s.v[157]);s.store_scalar(943, if s.b[943] { 1.0 } else { 0.0 });
        if (((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && (!s.b[942])) && s.b[943]) {s.copy_ad(165, 157);}
        if (((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) {s.copy_ad(164, 165);s.store_add(162, 349, 164);s.copy_ad(352, 162);s.copy_ad(388, 390);s.store_scaled_square(944, 474, (s.v[95] * s.v[95]));}
        s.b[950] = (s.v[352] < s.v[385]);s.store_scalar(950, if s.b[950] { 1.0 } else { 0.0 });
        if ((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && s.b[950]) {s.store_neg(945, 475);s.store_add_scaled_inputs3_mixed_aai(946, A::square(A::add_scaled_product(s.ad_value(945), 2.0, s.ad_value(944), s.ad_value(225), 1.0)), 1.0, A::square(s.ad_value(945)), (-4.0), 944, (-4.0));}
        if ((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && s.b[950]) {
            if (s.v[946] >= (10.0 * 2.220446049250313e-16)) {
            } else {
                s.store_scalar(946, (10.0 * 2.220446049250313e-16));
            }
        }
        if ((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && s.b[950]) {s.store_sqrt(946, 946);s.store_add_scaled_product_indices(947, 945, 2.0, 944, 225, 1.0);s.store_scaled_sub(948, 947, 946, 0.5);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_31(
        s: &mut Scratch,
    ) {
        if ((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && s.b[950]) {s.store_div_ad(949, A::ln(A::div_scaled_product_by_product(s.ad_value(945), s.ad_value(945), 1.0, s.ad_value(944), s.ad_value(239), 1.0)), A::add(s.ad_value(225), A::div_from_scalar(2.0, s.ad_value(945))));}
        s.b[951] = (s.v[948] < s.v[382]);s.store_scalar(951, if s.b[951] { 1.0 } else { 0.0 });
        if (((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && s.b[950]) && s.b[951]) {s.copy_ad(354, 948);}
        if (((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && s.b[950]) && (!s.b[951])) {s.store_offset_sub(44, 949, 948, (-0.0008));s.store_scale(45, 949, (4.0 * 0.0008));}
        if (((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && s.b[950]) && (!s.b[951])) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }
        if (((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && s.b[950]) && (!s.b[951])) {s.store_sqrt_square_add(45, 44, 45);s.store_add_scaled_inputs3_indices(354, 949, 1.0, 44, (-0.5), 45, (-0.5));}
        if ((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && (!s.b[950])) {s.store_neg_ad(945, A::add_scaled_inputs_product(s.ad_value(475), 1.0, s.ad_value(352), (-1.0), s.ad_value(341), s.ad_value(736), (-(1.0 / (2.0) * 9662367879.197212))));s.store_add_scaled_inputs3_mixed_aai(946, A::square(A::add_scaled_product(s.ad_value(945), 2.0, s.ad_value(944), s.ad_value(225), 1.0)), 1.0, A::square(s.ad_value(945)), (-4.0), 944, (-4.0));}
        if ((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && (!s.b[950])) {
            if (s.v[946] >= (10.0 * 2.220446049250313e-16)) {
            } else {
                s.store_scalar(946, (10.0 * 2.220446049250313e-16));
            }
        }
        if ((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && (!s.b[950])) {s.store_sqrt(946, 946);s.store_add_scaled_product_indices(947, 945, 2.0, 944, 225, 1.0);s.store_scaled_sub(948, 947, 946, 0.5);s.store_div_ad(949, A::ln(A::div_scaled_product_by_product(s.ad_value(945), s.ad_value(945), 1.0, s.ad_value(944), s.ad_value(239), 1.0)), A::add(s.ad_value(225), A::div_from_scalar(2.0, s.ad_value(945))));}
        s.b[952] = (s.v[948] < s.v[382]);s.store_scalar(952, if s.b[952] { 1.0 } else { 0.0 });
        if (((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && (!s.b[950])) && s.b[952]) {s.copy_ad(354, 948);}
        if (((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && (!s.b[950])) && (!s.b[952])) {s.store_offset_sub(44, 949, 948, (-0.0008));s.store_scale(45, 949, (4.0 * 0.0008));}
        if (((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && (!s.b[950])) && (!s.b[952])) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }
        if (((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && (!s.b[950])) && (!s.b[952])) {s.store_sqrt_square_add(45, 44, 45);s.store_add_scaled_inputs3_indices(354, 949, 1.0, 44, (-0.5), 45, (-0.5));}
        if (((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) {s.store_div_scaled_inputs_indices(953, 352, ((2.0 * 1.034943e-10) / 1.6021918e-19), 544, 1.0);}
        s.b[961] = (s.v[953] > 0.0);s.store_scalar(961, if s.b[961] { 1.0 } else { 0.0 });
        if ((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && s.b[961]) {s.store_sqrt_div_scaled_inputs(401, 352, ((2.0 * 1.034943e-10) / 1.6021918e-19), 544, 1.0);}
        if ((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && (!s.b[961])) {s.store_scalar(401, 0.0);}
        s.b[962] = ((s.v[352] < s.v[385]) && (0.0 != 0.0));s.store_scalar(962, if s.b[962] { 1.0 } else { 0.0 });
        if ((((s.b[733] && (!s.b[925])) && (!s.b[940])) && (!s.b[941])) && s.b[962]) {s.store_scalar(168, 0.0);}
    }
}
