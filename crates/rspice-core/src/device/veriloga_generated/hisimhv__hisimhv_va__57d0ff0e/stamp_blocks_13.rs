#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_transient_block_208(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[3322] {s.store_sub_mixed_ai(337, A::div_scaled_inputs(s.ad_value(979), (((-2.0) * 6.241449993689894e18) * 1.0 / (s.v[635])), s.ad_value(170), 1.0), 335);}
        s.b[3324] = ((((s.v[337] - s.v[335])) as f64).abs() > (10.0 * 2.220446049250313e-16));s.store_scalar(3324, if s.b[3324] { 1.0 } else { 0.0 });
        if (s.b[3322] && s.b[3324]) {s.store_add_scaled_value_products_mixed_aaaai(338, A::div_scalar_by_product(1.0, A::add(s.ad_value(335), s.ad_value(336)), A::add(s.ad_value(337), s.ad_value(336)), 1.0), 1.0, A::div_scaled_product3(s.ad_value(309), s.ad_value(255), s.ad_value(253), 2.0, A::sub(s.ad_value(337), s.ad_value(335)), 1.0), A::ln(A::div_scaled_inputs2(s.ad_value(337), 1.0, s.ad_value(336), 1.0, A::add(s.ad_value(335), s.ad_value(336)), 1.0)), 1.0, A::mul3(A::mul3(s.ad_value(309), s.ad_value(255), s.ad_value(253)), s.ad_value(309), s.ad_value(255)), 253, 1.0);}
        if (s.b[3322] && (!s.b[3324])) {s.store_add_scaled_inputs_product_mixed_aaai(338, A::div_scalar_by_product(1.0, A::add(s.ad_value(335), s.ad_value(336)), A::add(s.ad_value(337), s.ad_value(336)), 1.0), 1.0, A::div_scaled_product3(s.ad_value(309), s.ad_value(255), s.ad_value(253), 2.0, A::add(s.ad_value(335), s.ad_value(336)), 1.0), 1.0, A::mul3(A::mul3(s.ad_value(309), s.ad_value(255), s.ad_value(253)), s.ad_value(309), s.ad_value(255)), 253, 1.0);}
        s.b[3325] = (((p.p30 != 0.0) && (s.v[78] == 0.0)) && (s.v[963] == 0.0));s.store_scalar(3325, if s.b[3325] { 1.0 } else { 0.0 });
        if s.b[3325] {s.store_div_scaled_offset_numerator_mixed_ai(313, A::sub(s.ad_value(168), s.ad_value(87)), 1.0, (10.0 * 2.220446049250313e-16), 170, 1.0);}
        if s.b[3325] {
            if (s.v[313] >= 0.0) {
            } else {
                s.store_scalar(313, 0.0);
            }
        }
        if s.b[3325] {s.store_scaled_mul(346, 254, 313, 1e-7);}
        s.b[3326] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= p.p178) && (p.p178 <= (1.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(3326, if s.b[3326] { 1.0 } else { 0.0 });
        if (s.b[3325] && s.b[3326]) {s.store_scalar(341, 1.0);}
        s.b[3327] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= p.p178) && (p.p178 <= (2.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(3327, if s.b[3327] { 1.0 } else { 0.0 });
        if ((s.b[3325] && (!s.b[3326])) && s.b[3327]) {s.copy_ad(341, 346);}
        if ((s.b[3325] && (!s.b[3326])) && (!s.b[3327])) {
            if (s.v[313] == 0.0) {
                s.store_scalar(341, 0.0);
            } else {
                s.store_powf(341, 313, (p.p178 - 1.0));
            }
        }
        if s.b[3325] {s.store_mul(342, 346, 341);s.store_offset(343, 342, 1.0);}
        if s.b[3325] {
            if (s.v[343] == 0.0) {
                s.store_scalar(344, 0.0);
            } else {
                s.store_powf(344, 343, (((-1.0) / p.p178) - 1.0));
            }
        }
        if s.b[3325] {s.store_mul(345, 343, 344);s.store_mul(316, 254, 345);s.store_scaled_add(314, 253, 316, 0.5);s.store_square(334, 125);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_209(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[3325] {s.store_div_scaled_product_by_product_mixed_aaai(315, A::mul3_scaled_output(s.ad_value(185), s.ad_value(127), s.ad_value(253), s.v[632]), A::add_scaled_inputs3(A::mul3(A::add_scaled_inputs(A::scale_offset(s.ad_value(125), 3.0, 1.0), 1.0, s.ad_value(334), 6.0), s.ad_value(316), s.ad_value(316)), 1.0, A::mul3(A::add_scaled_inputs(A::scale_offset(s.ad_value(125), 4.0, 3.0), 1.0, s.ad_value(334), 3.0), s.ad_value(316), s.ad_value(253)), 1.0, A::mul3(A::add(A::scale_offset(s.ad_value(125), 3.0, 6.0), s.ad_value(334)), s.ad_value(253), s.ad_value(253)), 1.0), 1.0, A::mul3_scaled_output(s.ad_value(170), A::offset(s.ad_value(125), 1.0), s.ad_value(314), 15.0), 314, 1.0);}
        if (!s.b[3325]) {s.store_scalar(315, 0.0);}
        s.b[3328] = (((((p.p31 != 0.0) && (p.p30 != 0.0)) && (s.v[321] == 1.0)) && (s.v[78] == 0.0)) && (s.v[963] == 0.0));s.store_scalar(3328, if s.b[3328] { 1.0 } else { 0.0 });
        if s.b[3328] {s.store_sqrt(322, 319);s.store_add(336, 127, 322);s.store_square(337, 317);s.store_square(338, 319);s.store_scaled_mul(339, 317, 319, 42.0);s.store_add_scaled_inputs3_indices(339, 339, 1.0, 337, 4.0, 338, 4.0);s.store_add_product3_rhs_mixed_iia(339, 339, 322, 127, A::add(s.ad_value(317), s.ad_value(319)), 20.0);s.store_square(344, 336);s.store_square(344, 344);s.store_div_scaled_value_by_product_indices(323, 339, 1.0, 344, 336, 1.0);s.store_mul_ad_product_lhs_mixed_ai(324, A::div_from_scalar(s.v[632], s.ad_value(170)), 253, 185);s.store_mul(325, 324, 127);s.store_div(326, 315, 325);s.store_add_mixed_ai(341, A::add_scaled_product(s.ad_value(317), 1.0, s.ad_value(127), s.ad_value(322), 4.0), 319);s.store_div_scaled_product_by_product_mixed_iiia(327, 320, 341, 3.872983346207417, 336, A::sqrt(A::mul(A::mul3(s.ad_value(326), s.ad_value(336), s.ad_value(127)), s.ad_value(339))), 6.0);}
        s.store_scale(0, 134, s.v[365]);s.store_scale(699, 400, s.v[365]);s.copy_ad(430, 429);s.store_scalar(705, 0.0);s.store_scalar(706, 0.0);s.store_scalar(707, 0.0);s.store_scalar(811, 0.0);s.store_scalar(810, 0.0);s.store_scalar(812, 0.0);s.store_scalar(703, 0.0);s.store_scalar(704, 0.0);s.b[3329] = ((s.v[81] != 0.0) || (p.p22 == 2.0));s.store_scalar(3329, if s.b[3329] { 1.0 } else { 0.0 });
        if s.b[3329] {s.store_scalar(700, 0.0);s.store_scalar(701, 0.0);s.store_scalar(702, 0.0);s.copy_ad(708, 247);s.store_scale(754, 20, s.v[365]);s.store_scale(132, 132, s.v[365]);}
        if (!s.b[3329]) {s.store_scaled_add(700, 20, 132, (-s.v[365]));s.store_scale(701, 19, s.v[365]);s.store_scaled_sub(702, 132, 19, s.v[365]);}
        if (p.p29 != 0.0) {s.store_scale(572, 91, s.v[572]);s.store_sqrt_square_offset(782, 572, ((4.0 * 1e-12) * 1e-12));s.store_offset_scaled_div(334, 572, 782, 0.5, 0.5);s.store_scaled_add(572, 572, 782, 0.5);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_210(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        s.b[3330] = (s.v[572] < 0.0);s.store_scalar(3330, if s.b[3330] { 1.0 } else { 0.0 });
        if ((p.p29 != 0.0) && s.b[3330]) {s.store_scalar(572, 0.0);s.store_scalar(334, 0.0);}
        if (p.p29 != 0.0) {s.store_scale(308, 572, s.v[188]);s.store_voltage(817, ctx, nodes, Some(14), None);s.store_div_scaled_inputs2_indices(815, 817, 1.0, 816, (-1.0), 308, 1.0);s.store_add_scaled_inputs3_indices(352, 352, 1.0, 816, -1.0, 817, 1.0);s.copy_ad(355, 817);}
        if (p.p29 == 0.0) {s.copy_ad(817, 816);}
        s.b[3331] = (p.p22 > 0.0);s.store_scalar(3331, if s.b[3331] { 1.0 } else { 0.0 });
        if s.b[3331] {s.store_scaled_add_mixed_ai(811, A::add_scaled_inputs4(s.ad_value(293), 1.0, s.ad_value(352), (-1.0), s.ad_value(353), -1.0, s.ad_value(291), 1.0), 292, s.v[365]);s.store_scaled_sub(810, 355, 292, s.v[365]);s.store_scaled_sub(812, 356, 291, s.v[365]);s.store_add_scaled_inputs4_indices(700, 700, 1.0, 305, s.v[365], 360, ((-1.0) * s.v[365]), 362, (-s.v[365]));s.store_add_scaled_inputs3_indices(701, 701, 1.0, 361, s.v[365], 305, (-s.v[365]));s.store_add_scaled_inputs(702, 702, 1.0, 363, s.v[365]);s.store_sub_scaled_inputs(705, 350, (-s.v[365]), 351, s.v[365]);s.store_scale(706, 358, s.v[365]);s.store_scale(707, 359, s.v[365]);s.store_offset_sub_scaled_inputs_indices(703, 299, (-s.v[365]), 298, s.v[365], s.v[703]);s.store_offset_sub_scaled_inputs_indices(704, 301, (-s.v[365]), 297, s.v[365], s.v[704]);}
        s.store_scaled_add(709, 280, 287, s.v[365]);s.store_scale(710, 281, s.v[365]);s.store_scale(11, 202, (-s.v[365]));s.b[3332] = (s.v[949] == 1.0);s.store_scalar(3332, if s.b[3332] { 1.0 } else { 0.0 });
        if s.b[3332] {s.store_sub_scaled_inputs(9, 199, (p.p252 * s.v[365]), 201, s.v[365]);}
        if (!s.b[3332]) {s.store_sub_scaled_inputs(9, 199, ((1.0 - p.p252) * s.v[365]), 200, s.v[365]);}
        s.b[3333] = (s.v[949] == 1.0);s.store_scalar(3333, if s.b[3333] { 1.0 } else { 0.0 });
        if s.b[3333] {s.store_sub_scaled_inputs(10, 199, ((1.0 - p.p252) * s.v[365]), 200, s.v[365]);}
        if (!s.b[3333]) {s.store_sub_scaled_inputs(10, 199, (p.p252 * s.v[365]), 201, s.v[365]);}
        s.store_scale(7, 203, s.v[365]);s.store_scale(8, 204, s.v[365]);s.store_scale(807, 387, (4.0 * 1.3806226e-23));s.store_scale(712, 315, s.v[365]);s.store_scalar(22, A::ddx_projection(&s.ad_value(700), Some(6), None));s.store_scale(22, 22, p.p87);s.store_scalar(23, A::ddx_projection(&s.ad_value(700), Some(8), None));s.store_scale(23, 23, p.p87);
        if (s.v[949] > 0.0) {
            s.copy_ad(757, 23);
        } else {
            s.copy_ad(757, 22);
        }
        s.store_scalar(713, 0.0);s.store_scalar(714, 0.0);s.b[3334] = (((((p.p31 != 0.0) && (p.p30 != 0.0)) && (s.v[321] == 1.0)) && (s.v[78] == 0.0)) && (s.v[963] == 0.0));s.store_scalar(3334, if s.b[3334] { 1.0 } else { 0.0 });
        if s.b[3334] {s.store_scaled_mul(334, 185, 162, (1e-6 * s.v[635]));s.store_scale(344, 757, 1.0 / (s.v[365]));s.store_div_scaled_product3_indices(328, 155, 344, 344, (0.1185185185185185 * 1.6021918e-19), 324, 1.0);}
        s.b[3335] = ((s.v[320] > (10.0 * 2.220446049250313e-16)) && (s.v[790] > (10.0 * 2.220446049250313e-16)));s.store_scalar(3335, if s.b[3335] { 1.0 } else { 0.0 });
        if (s.b[3334] && s.b[3335]) {s.store_div(329, 254, 253);s.store_div_scaled_inputs2_mixed_aii(330, A::div(s.ad_value(254), s.ad_value(316)), 1.0, 329, (-1.0), 790, 1.0);s.store_add_mixed_ia(331, 329, A::div_scaled_product(s.ad_value(330), A::add(A::add_scaled_product(s.ad_value(317), 1.0, s.ad_value(127), s.ad_value(322), 1.0), s.ad_value(319)), 0.6666666666666667, A::add(s.ad_value(127), s.ad_value(322)), 1.0));}
        if (s.b[3334] && (!s.b[3335])) {s.store_div(331, 254, 316);}
        if s.b[3334] {s.store_mul3_affine_lhs(713, 328, 323, s.v[365], 0.0, 331);s.copy_ad(714, 327);}
        if s.b[3334] {
            if (s.v[713] < 0.0) {
                s.store_scalar(713, 0.0);
            } else {
            }
        }
        if s.b[3334] {
            if ((-s.v[344]) > s.v[334]) {
            } else {
                s.store_scalar(713, 0.0);
            }
        }
        if s.b[3334] {
            if ((-s.v[344]) > s.v[334]) {
            } else {
                s.store_scalar(714, 0.0);
            }
        }
        s.store_mul(952, 807, 712);s.copy_ad(951, 714);
        if ((s.v[952] > 0.0) && (s.v[713] > 0.0)) {
            s.store_sqrt_div(953, 713, 952);
        } else {
            s.store_scalar(953, 0.0);
        }
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_211(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
    ) {
        let ctx_temp = ctx.temperature();
        if (s.v[949] > 0.0) {
            s.store_mul_scale_offset_indices(954, 953, 247, -1.0, 1.0);
        } else {
            s.store_mul(954, 953, 247);
        }
        if (s.v[949] > 0.0) {
            s.store_mul(955, 953, 247);
        } else {
            s.store_mul_scale_offset_indices(955, 953, 247, -1.0, 1.0);
        }
        s.store_scalar(716, 0.0);s.store_scalar(715, 0.0);s.b[3336] = (s.v[449] == 1.0);s.store_scalar(3336, if s.b[3336] { 1.0 } else { 0.0 });s.b[3337] = (s.v[76] == 0.0);s.store_scalar(3337, if s.b[3337] { 1.0 } else { 0.0 });s.b[3338] = ((p.p53 > 0.0) && (s.v[541] != 0.0));s.store_scalar(3338, if s.b[3338] { 1.0 } else { 0.0 });
        if ((s.b[3336] && (!s.b[3337])) && s.b[3338]) {
            if (s.v[676] == 0.0) {
                s.store_scalar(335, 0.0);
            } else {
                s.store_powf(335, 676, p.p416);
            }
        }
        if ((s.b[3336] && (!s.b[3337])) && s.b[3338]) {s.store_div_from_scalar(794, s.v[569], 335);s.store_sub_ad(334, A::add_scaled_product(A::scale_offset(s.ad_value(676), 0.4, 1.8), 1.0, s.ad_value(676), s.ad_value(676), 0.1), A::scale_offset(s.ad_value(676), (-p.p418), p.p418));s.store_div_from_scalar(795, s.v[570], 334);s.store_add_mixed_ia(959, 959, A::scaled_offset(s.ad_value(387), (-s.v[764]), p.p439));}
        if ((s.b[3336] && (!s.b[3337])) && (!s.b[3338])) {s.store_scalar(387, (ctx_temp + p.p11));}
        if (s.b[3336] && (!s.b[3337])) {s.store_scalar(164, (s.v[630] * p.p7));s.store_scalar(604, p.p71);s.store_scalar(605, s.v[460]);s.store_mul(606, 794, 653);s.store_offset_product3(607, s.ad_value(795), s.ad_value(786), s.ad_value(652), 1.0, 1e-25);s.store_div(608, 804, 604);s.store_mul(609, 606, 608);}
        s.b[3339] = (s.v[804] >= 0.0);s.store_scalar(3339, if s.b[3339] { 1.0 } else { 0.0 });
        if ((s.b[3336] && (!s.b[3337])) && s.b[3339]) {s.store_div(335, 609, 607);}
        if ((s.b[3336] && (!s.b[3337])) && (!s.b[3339])) {s.store_div_scaled_inputs_indices(335, 609, -1.0, 607, 1.0);}
        s.b[3340] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= s.v[959]) && (s.v[959] <= (1.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(3340, if s.b[3340] { 1.0 } else { 0.0 });
        if ((s.b[3336] && (!s.b[3337])) && s.b[3340]) {s.store_scalar(337, 1.0);}
        s.b[3341] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= s.v[959]) && (s.v[959] <= (2.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(3341, if s.b[3341] { 1.0 } else { 0.0 });
        if (((s.b[3336] && (!s.b[3337])) && (!s.b[3340])) && s.b[3341]) {s.copy_ad(337, 335);}
        if (((s.b[3336] && (!s.b[3337])) && (!s.b[3340])) && (!s.b[3341])) {s.store_pow_offset_rhs(337, 335, 959, (-1.0));}
        if (s.b[3336] && (!s.b[3337])) {s.store_mul(336, 335, 337);s.store_offset(338, 336, 1.0);}
        s.b[3342] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= s.v[959]) && (s.v[959] <= (1.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(3342, if s.b[3342] { 1.0 } else { 0.0 });
        if ((s.b[3336] && (!s.b[3337])) && s.b[3342]) {s.store_div_from_scalar(339, 1.0, 338);}
        s.b[3343] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= s.v[959]) && (s.v[959] <= (2.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(3343, if s.b[3343] { 1.0 } else { 0.0 });
        if (((s.b[3336] && (!s.b[3337])) && (!s.b[3342])) && s.b[3343]) {s.store_div_from_scalar_sqrt_ad(339, 1.0, s.ad_value(338));}
        if (((s.b[3336] && (!s.b[3337])) && (!s.b[3342])) && (!s.b[3343])) {
            if (s.v[338] == 0.0) {
                s.store_scalar(340, 0.0);
            } else {
                s.store_pow_ad(340, s.ad_value(338), A::offset(A::div_from_scalar((-1.0), s.ad_value(959)), (-1.0)));
            }
        }
        if (((s.b[3336] && (!s.b[3337])) && (!s.b[3342])) && (!s.b[3343])) {s.store_mul(339, 338, 340);}
        if (s.b[3336] && (!s.b[3337])) {s.store_mul(610, 606, 339);s.copy_ad(611, 605);s.copy_ad(612, 614);s.store_div_from_scalar(335, 1.6021918e-19, 604);s.store_mul_product3_indices(613, 611, 335, 612, 610, 1.0);}
        s.b[3344] = ((s.v[613] < 1e-25) && (1e-25 >= 0.0));s.store_scalar(3344, if s.b[3344] { 1.0 } else { 0.0 });
        if ((s.b[3336] && (!s.b[3337])) && s.b[3344]) {s.store_sub_from_scalar(781, 1e-25, 613);s.store_square(722, 781);s.store_scalar(723, (1e-25 * 1e-25));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
        let (t0,) = {
    if ((s.b[3336] && (!s.b[3337])) && s.b[3344]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t0);
        let (t1,) = {
    if ((s.b[3336] && (!s.b[3337])) && s.b[3344]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t1);
        if ((s.b[3336] && (!s.b[3337])) && s.b[3344]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[3345] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(3345, if s.b[3345] { 1.0 } else { 0.0 });s.b[3346] = (2.0 == 1.0);s.store_scalar(3346, if s.b[3346] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_212(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
    ) {
        let ctx_temp = ctx.temperature();
        let (t2,) = {
    if ((((s.b[3336] && (!s.b[3337])) && s.b[3344]) && s.b[3345]) && s.b[3346]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t2);s.b[3347] = (2.0 == 2.0);s.store_scalar(3347, if s.b[3347] { 1.0 } else { 0.0 });
        let (t3,) = {
    if (((((s.b[3336] && (!s.b[3337])) && s.b[3344]) && s.b[3345]) && (!s.b[3346])) && s.b[3347]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t3);s.b[3348] = (2.0 == 4.0);s.store_scalar(3348, if s.b[3348] { 1.0 } else { 0.0 });
        let (t4,) = {
    if ((((((s.b[3336] && (!s.b[3337])) && s.b[3344]) && s.b[3345]) && (!s.b[3346])) && (!s.b[3347])) && s.b[3348]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t4);s.b[3349] = (2.0 == 8.0);s.store_scalar(3349, if s.b[3349] { 1.0 } else { 0.0 });
        let (t5,) = {
    if (((((((s.b[3336] && (!s.b[3337])) && s.b[3344]) && s.b[3345]) && (!s.b[3346])) && (!s.b[3347])) && (!s.b[3348])) && s.b[3349]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t5);
        let (t6,) = {
    if (((s.b[3336] && (!s.b[3337])) && s.b[3344]) && s.b[3345]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t6);let mut ta: usize = 0;
        while {
            let t9: f64 = if ((((s.b[3336] && (!s.b[3337])) && s.b[3344]) && s.b[3345]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t9 != 0.0
        } {
            ta += 1;assert!(ta <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[3336] && (!s.b[3337])) && s.b[3344]) && s.b[3345]) {s.store_sqrt(726, 726);}
            let (t8,) = {
    if (((s.b[3336] && (!s.b[3337])) && s.b[3344]) && s.b[3345]) {
        let t7: f64 = (s.v[719] + 1.0);
        (t7,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t8);
        }
        if (((s.b[3336] && (!s.b[3337])) && s.b[3344]) && (!s.b[3345])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if ((s.b[3336] && (!s.b[3337])) && s.b[3344]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 1e-25);s.store_div_scaled_product_indices(334, 725, 726, 1e-25, 770, 1.0);s.store_sub_from_scalar(613, 1e-25, 780);}
        if ((s.b[3336] && (!s.b[3337])) && s.b[3344]) {
        }
        if ((s.b[3336] && (!s.b[3337])) && (!s.b[3344])) {
        }
        if ((s.b[3336] && (!s.b[3337])) && (!s.b[3344])) {s.store_scalar(334, 1.0);}
        if (s.b[3336] && (!s.b[3337])) {s.store_div_from_scalar(5, 1.0, 613);s.store_div(5, 5, 164);s.store_add(5, 5, 648);}
        s.b[3351] = (s.v[5] < p.p444);s.store_scalar(3351, if s.b[3351] { 1.0 } else { 0.0 });
        if ((s.b[3336] && (!s.b[3337])) && s.b[3351]) {s.store_scalar(5, p.p444);}
        if (s.b[3336] && (!s.b[3337])) {s.store_scale(716, 5, 1.0 / (s.v[365]));}
        s.b[3356] = (s.v[75] == 0.0);s.store_scalar(3356, if s.b[3356] { 1.0 } else { 0.0 });
        if (s.b[3336] && (!s.b[3356])) {s.copy_ad(3352, 729);s.copy_ad(3353, 728);}
        s.b[3357] = ((p.p53 > 0.0) && (s.v[541] != 0.0));s.store_scalar(3357, if s.b[3357] { 1.0 } else { 0.0 });
        if ((s.b[3336] && (!s.b[3356])) && s.b[3357]) {
            if (s.v[676] == 0.0) {
                s.store_scalar(335, 0.0);
            } else {
                s.store_powf(335, 676, p.p415);
            }
        }
        if ((s.b[3336] && (!s.b[3356])) && s.b[3357]) {s.store_div_from_scalar(787, s.v[567], 335);s.store_sub_ad(334, A::add_scaled_product(A::scale_offset(s.ad_value(676), 0.4, 1.8), 1.0, s.ad_value(676), s.ad_value(676), 0.1), A::scale_offset(s.ad_value(676), (-p.p417), p.p417));s.store_div_from_scalar(788, s.v[568], 334);s.store_add_mixed_ia(956, 956, A::scaled_offset(s.ad_value(387), (-s.v[764]), p.p438));}
        s.b[3359] = (s.v[956] < 0.1);s.store_scalar(3359, if s.b[3359] { 1.0 } else { 0.0 });
        if (((s.b[3336] && (!s.b[3356])) && s.b[3357]) && s.b[3359]) {s.store_scalar(956, 0.1);}
        if ((s.b[3336] && (!s.b[3356])) && (!s.b[3357])) {s.store_scalar(387, (ctx_temp + p.p11));}
        if (s.b[3336] && (!s.b[3356])) {s.store_scalar(164, (s.v[630] * p.p7));s.store_scalar(785, (p.p67 + p.p68));s.store_primal_offset(789, 451, 1e-12);s.store_scalar(408, s.v[459]);s.store_offset_ad(335, A::mul_sub_from_scalar_rhs(s.ad_value(3353), p.p410, A::scale(s.ad_value(3353), p.p411)), 1.0);s.store_sqrt_square_offset(782, 335, ((4.0 * 0.1) * 0.1));s.store_offset_scaled_div(336, 335, 782, 0.5, 0.5);s.store_scaled_add(654, 335, 782, 0.5);}
        s.b[3360] = (s.v[654] < 0.0);s.store_scalar(3360, if s.b[3360] { 1.0 } else { 0.0 });
        if ((s.b[3336] && (!s.b[3356])) && s.b[3360]) {s.store_scalar(654, 0.0);s.store_scalar(336, 0.0);}
        if (s.b[3336] && (!s.b[3356])) {s.store_mul3_lhs(593, 787, 653, 654);s.store_offset_product3(3355, s.ad_value(788), s.ad_value(786), s.ad_value(652), 1.0, 1e-25);s.copy_ad(594, 453);s.store_scalar(595, p.p421);s.store_scale(335, 593, 10000.0);s.store_scale(336, 3355, 100.0);}
        s.b[3363] = (s.v[799] < 0.0);s.store_scalar(3363, if s.b[3363] { 1.0 } else { 0.0 });
        if ((s.b[3336] && (!s.b[3356])) && s.b[3363]) {s.store_scale(781, 799, ((-0.5) * (2.0 * 1.0 / (p.p262))));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_213(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.b[3336] && (!s.b[3356])) && s.b[3363]) {s.store_offset_mul_offset_rhs_mixed_ia(782, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 5040.0), (1.0 / 720.0))), (1.0 / 120.0)), (1.0 / 24.0)), (1.0 / 6.0)), (1.0 / 2.0), 1.0);s.store_offset_mul_offset_rhs_mixed_ia(783, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 840.0), (1.0 / 144.0))), (1.0 / 30.0)), (1.0 / 8.0)), (1.0 / 3.0), (1.0 / 2.0));s.store_div_from_scalar(108, p.p262, 782);s.store_div_scaled_inputs_square_rhs(336, 783, (-2.0), 782, 1.0);}
        s.b[3364] = (s.v[108] < 1e-12);s.store_scalar(3364, if s.b[3364] { 1.0 } else { 0.0 });
        if (((s.b[3336] && (!s.b[3356])) && s.b[3363]) && s.b[3364]) {s.store_scalar(108, 1e-12);}
        if ((s.b[3336] && (!s.b[3356])) && s.b[3363]) {s.store_sub_scaled_inputs(598, 799, 1.0, 108, 2.0);}
        if ((s.b[3336] && (!s.b[3356])) && (!s.b[3363])) {s.store_scale(781, 799, (0.5 * (2.0 * 1.0 / (p.p262))));s.store_offset_mul_offset_rhs_mixed_ia(782, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 5040.0), (1.0 / 720.0))), (1.0 / 120.0)), (1.0 / 24.0)), (1.0 / 6.0)), (1.0 / 2.0), 1.0);s.store_offset_mul_offset_rhs_mixed_ia(783, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 840.0), (1.0 / 144.0))), (1.0 / 30.0)), (1.0 / 8.0)), (1.0 / 3.0), (1.0 / 2.0));s.store_div_from_scalar(108, p.p262, 782);s.store_div_scaled_inputs_square_rhs(336, 783, (-2.0), 782, 1.0);}
        s.b[3365] = (s.v[108] < 1e-12);s.store_scalar(3365, if s.b[3365] { 1.0 } else { 0.0 });
        if (((s.b[3336] && (!s.b[3356])) && (!s.b[3363])) && s.b[3365]) {s.store_scalar(108, 1e-12);}
        if ((s.b[3336] && (!s.b[3356])) && (!s.b[3363])) {s.store_add_scaled_inputs(598, 799, 1.0, 108, 2.0);}
        if (s.b[3336] && (!s.b[3356])) {s.store_div(591, 598, 785);s.store_mul(592, 593, 591);}
        s.b[3366] = (s.v[799] >= 0.0);s.store_scalar(3366, if s.b[3366] { 1.0 } else { 0.0 });
        if ((s.b[3336] && (!s.b[3356])) && s.b[3366]) {s.store_div(335, 592, 3355);}
        if ((s.b[3336] && (!s.b[3356])) && (!s.b[3366])) {s.store_div_scaled_inputs_indices(335, 592, -1.0, 3355, 1.0);}
        s.b[3367] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= s.v[956]) && (s.v[956] <= (1.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(3367, if s.b[3367] { 1.0 } else { 0.0 });
        if ((s.b[3336] && (!s.b[3356])) && s.b[3367]) {s.store_scalar(337, 1.0);}
        s.b[3368] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= s.v[956]) && (s.v[956] <= (2.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(3368, if s.b[3368] { 1.0 } else { 0.0 });
        if (((s.b[3336] && (!s.b[3356])) && (!s.b[3367])) && s.b[3368]) {s.copy_ad(337, 335);}
        if (((s.b[3336] && (!s.b[3356])) && (!s.b[3367])) && (!s.b[3368])) {s.store_pow_offset_rhs(337, 335, 956, (-1.0));}
        if (s.b[3336] && (!s.b[3356])) {s.store_mul(336, 335, 337);s.store_offset(338, 336, 1.0);}
        s.b[3369] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= s.v[956]) && (s.v[956] <= (1.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(3369, if s.b[3369] { 1.0 } else { 0.0 });
        if ((s.b[3336] && (!s.b[3356])) && s.b[3369]) {s.store_div_from_scalar(339, 1.0, 338);}
        s.b[3370] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= s.v[956]) && (s.v[956] <= (2.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(3370, if s.b[3370] { 1.0 } else { 0.0 });
        if (((s.b[3336] && (!s.b[3356])) && (!s.b[3369])) && s.b[3370]) {s.store_div_from_scalar_sqrt_ad(339, 1.0, s.ad_value(338));}
        if (((s.b[3336] && (!s.b[3356])) && (!s.b[3369])) && (!s.b[3370])) {
            if (s.v[338] == 0.0) {
                s.store_scalar(340, 0.0);
            } else {
                s.store_pow_ad(340, s.ad_value(338), A::offset(A::div_from_scalar((-1.0), s.ad_value(956)), (-1.0)));
            }
        }
        if (((s.b[3336] && (!s.b[3356])) && (!s.b[3369])) && (!s.b[3370])) {s.store_mul(339, 338, 340);}
        if (s.b[3336] && (!s.b[3356])) {s.store_mul(3354, 593, 339);s.store_offset(338, 335, 1.0);s.store_div_from_scalar(339, 1.0, 338);s.store_offset_ad(338, A::div_scaled_product_offset_denominator(A::mul_sub_from_scalar_rhs(s.ad_value(595), 1.0, s.ad_value(339)), s.ad_value(598), 1.0, s.ad_value(785), (-p.p423), 1.0), 1.0);s.store_offset(781, 338, (-0.001));s.store_scalar(782, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_214(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[3336] && (!s.b[3356])) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if (s.b[3336] && (!s.b[3356])) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_scaled_add(339, 781, 782, 0.5);s.store_mul(717, 408, 339);s.store_scale(718, 698, (6.241449993689894e18 * p.p430));s.store_add_scaled_inputs3_indices(781, 717, 1.0, 718, (-1.0), 717, (-0.001));s.store_scaled_mul(782, 717, 717, (4.0 * 0.001));}
        if (s.b[3336] && (!s.b[3356])) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if (s.b[3336] && (!s.b[3356])) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_add_scaled_inputs3_indices(718, 717, 1.0, 781, (-0.5), 782, (-0.5));s.store_sub(597, 717, 718);}
        s.b[3371] = ((p.p441 > 0.0) && (p.p440 > 1.0));s.store_scalar(3371, if s.b[3371] { 1.0 } else { 0.0 });s.b[3372] = ((s.v[597] > ((s.v[408] * p.p440) - (s.v[408] * p.p441))) && ((s.v[408] * p.p441) >= 0.0));s.store_scalar(3372, if s.b[3372] { 1.0 } else { 0.0 });
        if (((s.b[3336] && (!s.b[3356])) && s.b[3371]) && s.b[3372]) {s.store_add_scaled_inputs3_indices(781, 597, 1.0, 408, (-p.p440), 408, p.p441);s.store_square(722, 781);s.store_scaled_mul(723, 408, 408, (p.p441 * p.p441));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
        let (tb,) = {
    if (((s.b[3336] && (!s.b[3356])) && s.b[3371]) && s.b[3372]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, tb);
        let (tc,) = {
    if (((s.b[3336] && (!s.b[3356])) && s.b[3371]) && s.b[3372]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, tc);
        if (((s.b[3336] && (!s.b[3356])) && s.b[3371]) && s.b[3372]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);}
        let (td,) = {
    if (((s.b[3336] && (!s.b[3356])) && s.b[3371]) && s.b[3372]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, td);let mut t11: usize = 0;
        while {
            let t10: f64 = if ((((s.b[3336] && (!s.b[3356])) && s.b[3371]) && s.b[3372]) && (s.v[719] < p.p442)) { 1.0 } else { 0.0 };
            t10 != 0.0
        } {
            t11 += 1;assert!(t11 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[3336] && (!s.b[3356])) && s.b[3371]) && s.b[3372]) {s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);}
            let (tf,) = {
    if (((s.b[3336] && (!s.b[3356])) && s.b[3371]) && s.b[3372]) {
        let te: f64 = (s.v[719] + 1.0);
        (te,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, tf);
        }
        if (((s.b[3336] && (!s.b[3356])) && s.b[3371]) && s.b[3372]) {s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[3373] = ((((p.p442 == 1.0) || (p.p442 == 2.0)) || (p.p442 == 4.0)) || (p.p442 == 8.0));s.store_scalar(3373, if s.b[3373] { 1.0 } else { 0.0 });s.b[3374] = (p.p442 == 1.0);s.store_scalar(3374, if s.b[3374] { 1.0 } else { 0.0 });
        let (t12,) = {
    if (((((s.b[3336] && (!s.b[3356])) && s.b[3371]) && s.b[3372]) && s.b[3373]) && s.b[3374]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t12);s.b[3375] = (p.p442 == 2.0);s.store_scalar(3375, if s.b[3375] { 1.0 } else { 0.0 });
        let (t13,) = {
    if ((((((s.b[3336] && (!s.b[3356])) && s.b[3371]) && s.b[3372]) && s.b[3373]) && (!s.b[3374])) && s.b[3375]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t13);s.b[3376] = (p.p442 == 4.0);s.store_scalar(3376, if s.b[3376] { 1.0 } else { 0.0 });
        let (t14,) = {
    if (((((((s.b[3336] && (!s.b[3356])) && s.b[3371]) && s.b[3372]) && s.b[3373]) && (!s.b[3374])) && (!s.b[3375])) && s.b[3376]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t14);s.b[3377] = (p.p442 == 8.0);s.store_scalar(3377, if s.b[3377] { 1.0 } else { 0.0 });
        let (t15,) = {
    if ((((((((s.b[3336] && (!s.b[3356])) && s.b[3371]) && s.b[3372]) && s.b[3373]) && (!s.b[3374])) && (!s.b[3375])) && (!s.b[3376])) && s.b[3377]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t15);
        let (t16,) = {
    if ((((s.b[3336] && (!s.b[3356])) && s.b[3371]) && s.b[3372]) && s.b[3373]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t16);let mut t1a: usize = 0;
        while {
            let t19: f64 = if (((((s.b[3336] && (!s.b[3356])) && s.b[3371]) && s.b[3372]) && s.b[3373]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t19 != 0.0
        } {
            t1a += 1;assert!(t1a <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[3336] && (!s.b[3356])) && s.b[3371]) && s.b[3372]) && s.b[3373]) {s.store_sqrt(726, 726);}
            let (t18,) = {
    if ((((s.b[3336] && (!s.b[3356])) && s.b[3371]) && s.b[3372]) && s.b[3373]) {
        let t17: f64 = (s.v[719] + 1.0);
        (t17,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t18);
        }
        if ((((s.b[3336] && (!s.b[3356])) && s.b[3371]) && s.b[3372]) && (!s.b[3373])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * p.p442)));
            }
        }
        if (((s.b[3336] && (!s.b[3356])) && s.b[3371]) && s.b[3372]) {s.store_div_from_scalar(726, 1.0, 726);s.store_mul3_affine_lhs(780, 781, 408, p.p441, 0.0, 726);s.store_div_scaled_product3_indices(334, 408, 725, 726, p.p441, 770, 1.0);s.store_add_scaled_inputs3_indices(336, 408, p.p440, 408, (-p.p441), 780, 1.0);}
        if (((s.b[3336] && (!s.b[3356])) && s.b[3371]) && s.b[3372]) {
        }
        if (((s.b[3336] && (!s.b[3356])) && s.b[3371]) && (!s.b[3372])) {s.copy_ad(336, 597);s.store_scalar(334, 1.0);}
        if ((s.b[3336] && (!s.b[3356])) && s.b[3371]) {s.copy_ad(597, 336);}
        if (s.b[3336] && (!s.b[3356])) {s.store_neg(334, 697);s.store_sqrt_square_offset(782, 334, ((4.0 * 0.01) * 0.01));s.store_offset_scaled_div(343, 334, 782, 0.5, 0.5);s.store_scaled_add(334, 334, 782, 0.5);}
        s.b[3378] = (s.v[334] < 0.0);s.store_scalar(3378, if s.b[3378] { 1.0 } else { 0.0 });
        if ((s.b[3336] && (!s.b[3356])) && s.b[3378]) {s.store_scalar(334, 0.0);s.store_scalar(343, 0.0);}
        if (s.b[3336] && (!s.b[3356])) {s.store_offset(334, 334, (10.0 * 2.220446049250313e-16));s.store_sqrt_mul(599, 650, 334);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_215(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[3336] && (!s.b[3356])) {s.store_offset_sub(336, 3352, 3353, p.p137);s.store_sqrt_square_offset(782, 336, ((4.0 * 0.01) * 0.01));s.store_offset_scaled_div(343, 336, 782, 0.5, 0.5);s.store_scaled_add(336, 336, 782, 0.5);}
        s.b[3379] = (s.v[336] < 0.0);s.store_scalar(3379, if s.b[3379] { 1.0 } else { 0.0 });
        if ((s.b[3336] && (!s.b[3356])) && s.b[3379]) {s.store_scalar(336, 0.0);s.store_scalar(343, 0.0);}
        if (s.b[3336] && (!s.b[3356])) {s.store_offset(336, 336, (10.0 * 2.220446049250313e-16));s.store_sqrt_mul(600, 651, 336);s.store_add_scaled_inputs3_indices(781, 789, 1.0, 600, (-1.0), 789, (-0.01));s.store_scaled_mul(782, 789, 789, (4.0 * 0.01));}
        if (s.b[3336] && (!s.b[3356])) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if (s.b[3336] && (!s.b[3356])) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_add_scaled_inputs3_indices(602, 789, 1.0, 781, (-0.5), 782, (-0.5));s.store_scalar(601, (p.p419 + 1e-25));s.store_mul_scale_offset_mixed_ia(596, 649, A::mul(s.ad_value(594), A::add(A::div(s.ad_value(599), s.ad_value(601)), A::div(s.ad_value(602), s.ad_value(789)))), -1.0, 1.0);s.store_sqrt_ad(782, A::add_scaled_square_product(s.ad_value(596), 1.0, A::mul_sub_from_scalar_lhs(1.0, s.ad_value(453), s.ad_value(649)), A::mul_sub_from_scalar_lhs(1.0, s.ad_value(453), s.ad_value(649)), ((1.0 / (100.0) * 4.0) * 1.0 / (100.0))));s.store_offset_scaled_div(343, 596, 782, 0.5, 0.5);s.store_scaled_add(596, 596, 782, 0.5);}
        s.b[3380] = (s.v[596] < 0.0);s.store_scalar(3380, if s.b[3380] { 1.0 } else { 0.0 });
        if ((s.b[3336] && (!s.b[3356])) && s.b[3380]) {s.store_scalar(596, 0.0);s.store_scalar(343, 0.0);}
        if (s.b[3336] && (!s.b[3356])) {s.store_div_from_scalar_offset_input(335, 1.6021918e-19, 785, p.p422);s.store_mul_product3_indices(739, 597, 335, 596, 3354, 1.0);}
        s.b[3381] = ((s.v[739] < 1e-25) && (1e-25 >= 0.0));s.store_scalar(3381, if s.b[3381] { 1.0 } else { 0.0 });
        if ((s.b[3336] && (!s.b[3356])) && s.b[3381]) {s.store_sub_from_scalar(781, 1e-25, 739);s.store_square(722, 781);s.store_scalar(723, (1e-25 * 1e-25));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
        let (t1b,) = {
    if ((s.b[3336] && (!s.b[3356])) && s.b[3381]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t1b);
        let (t1c,) = {
    if ((s.b[3336] && (!s.b[3356])) && s.b[3381]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t1c);
        if ((s.b[3336] && (!s.b[3356])) && s.b[3381]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[3382] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(3382, if s.b[3382] { 1.0 } else { 0.0 });s.b[3383] = (2.0 == 1.0);s.store_scalar(3383, if s.b[3383] { 1.0 } else { 0.0 });
        let (t1d,) = {
    if ((((s.b[3336] && (!s.b[3356])) && s.b[3381]) && s.b[3382]) && s.b[3383]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t1d);s.b[3384] = (2.0 == 2.0);s.store_scalar(3384, if s.b[3384] { 1.0 } else { 0.0 });
        let (t1e,) = {
    if (((((s.b[3336] && (!s.b[3356])) && s.b[3381]) && s.b[3382]) && (!s.b[3383])) && s.b[3384]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t1e);s.b[3385] = (2.0 == 4.0);s.store_scalar(3385, if s.b[3385] { 1.0 } else { 0.0 });
        let (t1f,) = {
    if ((((((s.b[3336] && (!s.b[3356])) && s.b[3381]) && s.b[3382]) && (!s.b[3383])) && (!s.b[3384])) && s.b[3385]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t1f);s.b[3386] = (2.0 == 8.0);s.store_scalar(3386, if s.b[3386] { 1.0 } else { 0.0 });
        let (t20,) = {
    if (((((((s.b[3336] && (!s.b[3356])) && s.b[3381]) && s.b[3382]) && (!s.b[3383])) && (!s.b[3384])) && (!s.b[3385])) && s.b[3386]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t20);
        let (t21,) = {
    if (((s.b[3336] && (!s.b[3356])) && s.b[3381]) && s.b[3382]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t21);let mut t25: usize = 0;
        while {
            let t24: f64 = if ((((s.b[3336] && (!s.b[3356])) && s.b[3381]) && s.b[3382]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t24 != 0.0
        } {
            t25 += 1;assert!(t25 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[3336] && (!s.b[3356])) && s.b[3381]) && s.b[3382]) {s.store_sqrt(726, 726);}
            let (t23,) = {
    if (((s.b[3336] && (!s.b[3356])) && s.b[3381]) && s.b[3382]) {
        let t22: f64 = (s.v[719] + 1.0);
        (t22,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t23);
        }
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_216(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[3336] && (!s.b[3356])) && s.b[3381]) && (!s.b[3382])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if ((s.b[3336] && (!s.b[3356])) && s.b[3381]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 1e-25);s.store_div_scaled_product_indices(334, 725, 726, 1e-25, 770, 1.0);s.store_sub_from_scalar(739, 1e-25, 780);}
        if ((s.b[3336] && (!s.b[3356])) && s.b[3381]) {
        }
        if ((s.b[3336] && (!s.b[3356])) && (!s.b[3381])) {
        }
        if ((s.b[3336] && (!s.b[3356])) && (!s.b[3381])) {s.store_scalar(334, 1.0);}
        if (s.b[3336] && (!s.b[3356])) {s.store_div_from_scalar(4, 1.0, 739);s.store_div(4, 4, 164);}
        s.b[3387] = ((s.v[4] > (1000000.0 - 1000.0)) && (1000.0 >= 0.0));s.store_scalar(3387, if s.b[3387] { 1.0 } else { 0.0 });
        if ((s.b[3336] && (!s.b[3356])) && s.b[3387]) {s.store_offset(781, 4, (((-1000000.0)) + (1000.0)));s.store_square(722, 781);s.store_scalar(723, (1000.0 * 1000.0));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
        let (t26,) = {
    if ((s.b[3336] && (!s.b[3356])) && s.b[3387]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t26);
        let (t27,) = {
    if ((s.b[3336] && (!s.b[3356])) && s.b[3387]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t27);
        if ((s.b[3336] && (!s.b[3356])) && s.b[3387]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[3388] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(3388, if s.b[3388] { 1.0 } else { 0.0 });s.b[3389] = (2.0 == 1.0);s.store_scalar(3389, if s.b[3389] { 1.0 } else { 0.0 });
        let (t28,) = {
    if ((((s.b[3336] && (!s.b[3356])) && s.b[3387]) && s.b[3388]) && s.b[3389]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t28);s.b[3390] = (2.0 == 2.0);s.store_scalar(3390, if s.b[3390] { 1.0 } else { 0.0 });
        let (t29,) = {
    if (((((s.b[3336] && (!s.b[3356])) && s.b[3387]) && s.b[3388]) && (!s.b[3389])) && s.b[3390]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t29);s.b[3391] = (2.0 == 4.0);s.store_scalar(3391, if s.b[3391] { 1.0 } else { 0.0 });
        let (t2a,) = {
    if ((((((s.b[3336] && (!s.b[3356])) && s.b[3387]) && s.b[3388]) && (!s.b[3389])) && (!s.b[3390])) && s.b[3391]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t2a);s.b[3392] = (2.0 == 8.0);s.store_scalar(3392, if s.b[3392] { 1.0 } else { 0.0 });
        let (t2b,) = {
    if (((((((s.b[3336] && (!s.b[3356])) && s.b[3387]) && s.b[3388]) && (!s.b[3389])) && (!s.b[3390])) && (!s.b[3391])) && s.b[3392]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t2b);
        let (t2c,) = {
    if (((s.b[3336] && (!s.b[3356])) && s.b[3387]) && s.b[3388]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t2c);let mut t30: usize = 0;
        while {
            let t2f: f64 = if ((((s.b[3336] && (!s.b[3356])) && s.b[3387]) && s.b[3388]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t2f != 0.0
        } {
            t30 += 1;assert!(t30 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[3336] && (!s.b[3356])) && s.b[3387]) && s.b[3388]) {s.store_sqrt(726, 726);}
            let (t2e,) = {
    if (((s.b[3336] && (!s.b[3356])) && s.b[3387]) && s.b[3388]) {
        let t2d: f64 = (s.v[719] + 1.0);
        (t2d,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t2e);
        }
        if (((s.b[3336] && (!s.b[3356])) && s.b[3387]) && (!s.b[3388])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if ((s.b[3336] && (!s.b[3356])) && s.b[3387]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 1000.0);s.store_div_scaled_product_indices(334, 725, 726, 1000.0, 770, 1.0);s.store_offset(4, 780, (1000000.0 - 1000.0));}
        if ((s.b[3336] && (!s.b[3356])) && s.b[3387]) {
        }
        if ((s.b[3336] && (!s.b[3356])) && (!s.b[3387])) {
        }
        if ((s.b[3336] && (!s.b[3356])) && (!s.b[3387])) {s.store_scalar(334, 1.0);}
        s.b[3393] = ((p.p54 == 1.0) && ((s.v[459] * (s.v[544] + s.v[459])) > 0.0));s.store_scalar(3393, if s.b[3393] { 1.0 } else { 0.0 });
        if ((s.b[3336] && (!s.b[3356])) && s.b[3393]) {s.store_sub_from_scalar(385, p.p334, 384);s.store_div_scaled_inputs_indices(4, 4, s.v[165], 385, 1.0);}
        if (s.b[3336] && (!s.b[3356])) {s.store_add(4, 4, 644);}
        s.b[3395] = (s.v[4] < p.p444);s.store_scalar(3395, if s.b[3395] { 1.0 } else { 0.0 });
        if ((s.b[3336] && (!s.b[3356])) && s.b[3395]) {s.store_scalar(4, p.p444);}
        if (s.b[3336] && (!s.b[3356])) {s.store_scale(715, 4, 1.0 / (s.v[365]));}
        s.b[3396] = (s.v[4] < p.p444);s.store_scalar(3396, if s.b[3396] { 1.0 } else { 0.0 });
        if ((!s.b[3336]) && s.b[3396]) {s.store_scalar(4, p.p444);}
        s.b[3397] = (s.v[5] < p.p444);s.store_scalar(3397, if s.b[3397] { 1.0 } else { 0.0 });
        if ((!s.b[3336]) && s.b[3397]) {s.store_scalar(5, p.p444);}
        s.b[3398] = (s.v[370] > 0.0);s.store_scalar(3398, if s.b[3398] { 1.0 } else { 0.0 });
        if ((!s.b[3336]) && s.b[3398]) {s.store_scale(715, 4, 1.0 / (s.v[365]));s.store_scale(716, 5, 1.0 / (s.v[365]));}
        if ((!s.b[3336]) && (!s.b[3398])) {s.store_scale(715, 5, 1.0 / (s.v[365]));s.store_scale(716, 4, 1.0 / (s.v[365]));}
        s.copy_ad(4, 715);s.copy_ad(5, 716);s.copy_ad(201, 9);s.copy_ad(200, 10);s.copy_ad(202, 11);s.b[3399] = (s.v[949] > 0.0);s.store_scalar(3399, if s.b[3399] { 1.0 } else { 0.0 });
        if s.b[3399] {s.copy_ad(134, 0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_217(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[3399] {s.copy_ad(19, 701);s.copy_ad(18, 700);s.copy_ad(741, 702);s.store_add_scaled_inputs3_indices(20, 700, (-1.0), 701, (-1.0), 702, (-1.0));s.copy_ad(280, 709);s.store_scalar(736, 0.0);s.copy_ad(281, 710);s.store_scalar(737, 0.0);s.copy_ad(400, 699);s.store_scalar(738, 0.0);s.copy_ad(431, 430);s.store_scalar(432, 0.0);s.copy_ad(424, 422);s.store_scalar(425, 0.0);s.copy_ad(203, 7);s.copy_ad(204, 8);}
        if (s.b[3399] && (s.v[81] != 0.0)) {s.copy_ad(247, 708);}
        if (!s.b[3399]) {s.store_neg(134, 0);s.copy_ad(19, 702);s.copy_ad(18, 700);s.copy_ad(741, 701);s.store_add_scaled_inputs3_indices(20, 700, (-1.0), 701, (-1.0), 702, (-1.0));s.store_scalar(280, 0.0);s.copy_ad(736, 709);s.store_scalar(281, 0.0);s.copy_ad(737, 710);s.store_scalar(400, 0.0);s.copy_ad(738, 699);s.store_scalar(431, 0.0);s.copy_ad(432, 430);s.store_scalar(424, 0.0);s.copy_ad(425, 422);s.copy_ad(203, 8);s.copy_ad(204, 7);}
        if ((!s.b[3399]) && (s.v[81] != 0.0)) {s.store_sub_from_scalar(247, 1.0, 708);}
        s.store_add(18, 18, 811);s.store_add(19, 19, 810);s.store_add(741, 741, 812);s.store_add_scaled_inputs3_indices(20, 18, (-1.0), 19, (-1.0), 741, (-1.0));s.copy_ad(299, 703);s.copy_ad(301, 704);s.copy_ad(742, 706);s.copy_ad(743, 705);s.store_add_scaled_inputs3_indices(744, 705, (-1.0), 706, (-1.0), 707, (-1.0));s.b[3400] = (p.p53 > 0.0);s.store_scalar(3400, if s.b[3400] { 1.0 } else { 0.0 });s.b[3401] = (s.v[766] > 0.0001);s.store_scalar(3401, if s.b[3401] { 1.0 } else { 0.0 });
        if (s.b[3400] && s.b[3401]) {s.store_div_from_scalar(740, 1.0, 766);}
        if (s.b[3400] && (!s.b[3401])) {s.store_scalar(740, (1.0 / 0.0001));}
        s.b[3402] = ((s.v[729] * (s.v[733] - s.v[729])) >= 0.0);s.store_scalar(3402, if s.b[3402] { 1.0 } else { 0.0 });s.b[3403] = (s.v[529] == 1.0);s.store_scalar(3403, if s.b[3403] { 1.0 } else { 0.0 });
        if ((s.b[3400] && s.b[3402]) && s.b[3403]) {s.copy_ad(745, 733);}
        if ((s.b[3400] && s.b[3402]) && (!s.b[3403])) {s.store_add_scaled_product_right_sub(745, 729, 1.0, 683, 733, 729, 1.0);}
        if (s.b[3400] && (!s.b[3402])) {s.copy_ad(745, 729);}
        if s.b[3400] {s.store_mul(746, 134, 745);}
        s.b[3404] = (p.p53 == 1.0);s.store_scalar(3404, if s.b[3404] { 1.0 } else { 0.0 });
        if (s.b[3400] && s.b[3404]) {s.store_scale(335, 740, p.p433);s.store_add_scaled_inputs3_indices(781, 335, 1.0, 746, (-1.0), 740, (-p.p337));s.store_scaled_mul(782, 335, 740, (4.0 * p.p337));}
        if (s.b[3400] && s.b[3404]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if (s.b[3400] && s.b[3404]) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_add_scaled_inputs3_indices(336, 335, 1.0, 781, (-0.5), 782, (-0.5));s.copy_ad(746, 336);}
        if (!s.b[3400]) {s.store_scalar(740, 0.0);s.store_scalar(746, 0.0);}
        s.b[3405] = (s.v[306] < 1e-15);s.store_scalar(3405, if s.b[3405] { 1.0 } else { 0.0 });
        if ((s.v[81] != 0.0) && s.b[3405]) {s.store_scalar(306, 1e-15);}
        s.b[3406] = (s.v[307] < 1e-15);s.store_scalar(3406, if s.b[3406] { 1.0 } else { 0.0 });
        if ((s.v[81] != 0.0) && s.b[3406]) {s.store_scalar(307, 1e-15);}
        if (s.v[81] != 0.0) {s.store_div_scaled_inputs2_indices(749, 747, 1.0, 132, (-1.0), 306, 1.0);s.store_div_scaled_inputs2_indices(750, 748, 1.0, 754, (-1.0), 307, 1.0);s.store_mul(751, 747, 247);s.store_sub_scaled_inputs(753, 747, -1.0, 748, 1.0);s.store_mul_scale_offset_indices(752, 747, 247, -1.0, 1.0);}
        if (s.v[81] == 0.0) {s.store_scalar(749, 0.0);s.store_scalar(750, 0.0);s.store_scalar(751, 0.0);s.store_scalar(753, 0.0);s.store_scalar(752, 0.0);}
        s.store_scaled_mul(0, 949, 134, p.p87);s.store_scalar(22, A::ddx_projection(&s.ad_value(18), Some(6), None));s.store_scale(22, 22, p.p87);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_218(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.store_scalar(23, A::ddx_projection(&s.ad_value(18), Some(8), None));s.store_scale(23, 23, p.p87);s.b[3407] = (s.v[949] == 1.0);s.store_scalar(3407, if s.b[3407] { 1.0 } else { 0.0 });
        if s.b[3407] {s.copy_ad(757, 23);}
        if (!s.b[3407]) {s.copy_ad(757, 22);}
        s.b[3409] = (p.p48 > 0.0);s.store_scalar(3409, if s.b[3409] { 1.0 } else { 0.0 });s.b[3410] = (p.p24 == 1.0);s.store_scalar(3410, if s.b[3410] { 1.0 } else { 0.0 });s.b[3412] = ((p.p51 == 1.0) && (p.p132 > 0.0));s.store_scalar(3412, if s.b[3412] { 1.0 } else { 0.0 });s.b[3413] = (p.p53 > 0.0);s.store_scalar(3413, if s.b[3413] { 1.0 } else { 0.0 });
        if s.b[3413] {s.copy_ad(802, 746);}
        if (!s.b[3413]) {s.store_scalar(767, 0.0);}
        if (p.p28 != 0.0) {s.store_scalar(800, 1.0);s.store_scalar(801, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_0(
        s: &mut ReactiveScratch,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        s.b[623] = param_given[12];s.store_scalar(623, if s.b[623] { 1.0 } else { 0.0 });s.b[769] = param_given[268];s.store_scalar(769, if s.b[769] { 1.0 } else { 0.0 });s.b[768] = param_given[269];s.store_scalar(768, if s.b[768] { 1.0 } else { 0.0 });s.store_scalar(294, 0.0);s.store_scalar(295, 0.0);s.store_scalar(708, 0.0);s.store_scalar(4, 0.0);s.store_scalar(5, 0.0);s.store_scalar(321, 0.0);s.store_scalar(78, 0.0);s.store_scalar(74, 0.0);s.store_scalar(347, 0.0);s.store_scalar(697, 0.0);s.store_scalar(698, 0.0);s.store_scalar(69, 0.8);s.store_scalar(70, 0.4);s.store_scalar(77, 0.0);s.store_scalar(79, 0.0);s.store_scalar(80, 0.0);s.store_scalar(81, 0.0);s.store_scalar(83, 0.0);s.store_scalar(84, 0.0);s.store_scalar(85, 0.0);s.store_scalar(86, 0.0);s.store_scalar(87, 0.0);s.store_scalar(88, 0.0);s.store_scalar(89, 0.0);s.store_scalar(90, 0.0);s.store_scalar(91, 0.0);s.store_scalar(92, 0.0);s.store_scalar(93, 0.0);s.store_scalar(94, 0.0);s.store_scalar(95, 0.0);s.store_scalar(96, 0.0);s.store_scalar(97, 0.0);s.store_scalar(98, 0.0);s.store_scalar(99, 0.0);s.store_scalar(100, 0.0);s.store_scalar(101, 0.0);s.store_scalar(102, 0.0);s.store_scalar(103, 0.0);s.store_scalar(104, 0.0);s.store_scalar(105, 0.0);s.store_scalar(106, 0.0);s.store_scalar(107, 0.0);s.store_scalar(108, 0.0);s.store_scalar(109, 0.0);s.store_scalar(110, 0.0);s.store_scalar(111, 0.0);s.store_scalar(112, 0.0);s.store_scalar(113, 0.0);s.store_scalar(114, 0.0);s.store_scalar(115, 0.0);s.store_scalar(116, 0.0);s.store_scalar(415, 0.0);s.store_scalar(117, 0.0);s.store_scalar(118, 0.0);s.store_scalar(119, 0.0);s.store_scalar(120, 0.0);s.store_scalar(121, 0.0);s.store_scalar(122, 0.0);s.store_scalar(123, 0.0);s.store_scalar(124, 0.0);s.store_scalar(125, 0.0);s.store_scalar(126, 0.0);s.store_scalar(127, 0.0);s.store_scalar(128, 0.0);s.store_scalar(129, 0.0);s.store_scalar(130, 0.0);s.store_scalar(20, 0.0);s.store_scalar(131, 0.0);s.store_scalar(132, 0.0);s.store_scalar(133, 0.0);s.store_scalar(19, 0.0);s.store_scalar(134, 0.0);s.store_scalar(135, 0.0);s.store_scalar(137, 0.0);s.store_scalar(138, 0.0);s.store_scalar(139, 0.0);s.store_scalar(140, 0.0);s.store_scalar(141, 0.0);s.store_scalar(142, 0.0);s.store_scalar(143, 0.0);s.store_scalar(144, 0.0);s.store_scalar(145, 0.0);s.store_scalar(146, 0.0);s.store_scalar(147, 0.0);s.store_scalar(148, 0.0);s.store_scalar(149, 0.0);s.store_scalar(150, 0.0);s.store_scalar(151, 0.0);s.store_scalar(152, 0.0);s.store_scalar(153, 0.0);
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_1(
        s: &mut ReactiveScratch,
    ) {
        s.store_scalar(154, 0.0);s.store_scalar(155, 0.0);s.store_scalar(156, 0.0);s.store_scalar(157, 0.0);s.store_scalar(158, 0.0);s.store_scalar(159, 0.0);s.store_scalar(160, 0.0);s.store_scalar(161, 0.0);s.store_scalar(162, 0.0);s.store_scalar(163, 0.0);s.store_scalar(164, 0.0);s.store_scalar(165, 0.0);s.store_scalar(166, 0.0);s.store_scalar(167, 0.0);s.store_scalar(168, 0.0);s.store_scalar(169, 0.0);s.store_scalar(170, 0.0);s.store_scalar(171, 0.0);s.store_scalar(172, 0.0);s.store_scalar(173, 0.0);s.store_scalar(174, 0.0);s.store_scalar(175, 0.0);s.store_scalar(176, 0.0);s.store_scalar(177, 0.0);s.store_scalar(178, 0.0);s.store_scalar(179, 0.0);s.store_scalar(180, 0.0);s.store_scalar(181, 0.0);s.store_scalar(182, 0.0);s.store_scalar(184, 0.0);s.store_scalar(185, 0.0);s.store_scalar(186, 0.0);s.store_scalar(187, 0.0);s.store_scalar(188, 0.0);s.store_scalar(412, 0.0);s.store_scalar(189, 0.0);s.store_scalar(190, 0.0);s.store_scalar(191, 0.0);s.store_scalar(192, 0.0);s.store_scalar(193, 0.0);s.store_scalar(194, 0.0);s.store_scalar(195, 0.0);s.store_scalar(196, 0.0);s.store_scalar(197, 0.0);s.store_scalar(198, 0.0);s.store_scalar(205, 0.0);s.store_scalar(206, 0.0);s.store_scalar(207, 0.0);s.store_scalar(208, 0.0);s.store_scalar(209, 0.0);s.store_scalar(210, 0.0);s.store_scalar(211, 0.0);s.store_scalar(212, 0.0);s.store_scalar(213, 0.0);s.store_scalar(214, 0.0);s.store_scalar(215, 0.0);s.store_scalar(216, 0.0);s.store_scalar(217, 0.0);s.store_scalar(218, 0.0);s.store_scalar(219, 0.0);s.store_scalar(220, 0.0);s.store_scalar(221, 0.0);s.store_scalar(222, 0.0);s.store_scalar(223, 0.0);s.store_scalar(224, 0.0);s.store_scalar(225, 0.0);s.store_scalar(226, 0.0);s.store_scalar(227, 0.0);s.store_scalar(228, 0.0);s.store_scalar(229, 0.0);s.store_scalar(230, 0.0);s.store_scalar(231, 0.0);s.store_scalar(232, 0.0);s.store_scalar(233, 0.0);s.store_scalar(234, 0.0);s.store_scalar(235, 0.0);s.store_scalar(236, 0.0);s.store_scalar(237, 0.0);s.store_scalar(238, 0.0);s.store_scalar(239, 0.0);s.store_scalar(240, 0.0);s.store_scalar(241, 0.0);s.store_scalar(242, 0.0);s.store_scalar(243, 0.0);s.store_scalar(244, 0.0);s.store_scalar(245, 0.0);s.store_scalar(246, 0.0);s.store_scalar(247, 0.5);s.store_scalar(248, 0.0);s.store_scalar(249, 0.0);s.store_scalar(250, 0.0);s.store_scalar(251, 0.0);s.store_scalar(252, 0.0);s.store_scalar(253, 0.0);s.store_scalar(254, 0.0);s.store_scalar(255, 0.0);
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_2(
        s: &mut ReactiveScratch,
    ) {
        s.store_scalar(256, 0.0);s.store_scalar(258, 0.0);s.store_scalar(259, 0.0);s.store_scalar(260, 0.0);s.store_scalar(261, 0.0);s.store_scalar(262, 0.0);s.store_scalar(263, 0.0);s.store_scalar(264, 0.0);s.store_scalar(265, 0.0);s.store_scalar(266, 0.0);s.store_scalar(267, 0.0);s.store_scalar(268, 0.0);s.store_scalar(269, 0.0);s.store_scalar(270, 0.0);s.store_scalar(271, 0.0);s.store_scalar(272, 0.0);s.store_scalar(273, 0.0);s.store_scalar(274, 0.0);s.store_scalar(275, 0.0);s.store_scalar(276, 0.0);s.store_scalar(277, 0.0);s.store_scalar(278, 0.0);s.store_scalar(279, 0.0);s.store_scalar(280, 0.0);s.store_scalar(281, 0.0);s.store_scalar(282, 0.0);s.store_scalar(283, 0.0);s.store_scalar(285, 0.0);s.store_scalar(286, 0.0);s.store_scalar(289, 0.0);s.store_scalar(290, 0.0);s.store_scalar(291, 0.0);s.store_scalar(292, 0.0);s.store_scalar(293, 0.0);s.store_scalar(296, 0.0);s.store_scalar(297, 0.0);s.store_scalar(298, 0.0);s.store_scalar(299, 0.0);s.store_scalar(300, 0.0);s.store_scalar(301, 0.0);s.store_scalar(302, 0.0);s.store_scalar(303, 0.0);s.store_scalar(304, 0.0);s.store_scalar(305, 0.0);s.store_scalar(313, 0.0);s.store_scalar(314, 0.0);s.store_scalar(315, 0.0);s.store_scalar(316, 0.0);s.store_scalar(317, 0.0);s.store_scalar(318, 0.0);s.store_scalar(319, 0.0);s.store_scalar(320, 0.0);s.store_scalar(322, 0.0);s.store_scalar(323, 0.0);s.store_scalar(324, 0.0);s.store_scalar(328, 0.0);s.store_scalar(329, 0.0);s.store_scalar(330, 0.0);s.store_scalar(331, 0.0);s.store_scalar(332, 0.0);s.store_scalar(333, 0.0);s.store_scalar(334, 0.0);s.store_scalar(335, 0.0);s.store_scalar(336, 0.0);s.store_scalar(337, 0.0);s.store_scalar(338, 0.0);s.store_scalar(339, 0.0);s.store_scalar(340, 0.0);s.store_scalar(341, 0.0);s.store_scalar(342, 0.0);s.store_scalar(343, 0.0);s.store_scalar(344, 0.0);s.store_scalar(345, 0.0);s.store_scalar(346, 0.0);s.store_scalar(348, 0.0);s.store_scalar(349, 0.0);s.store_scalar(350, 0.0);s.store_scalar(351, 0.0);s.store_scalar(352, 0.0);s.store_scalar(353, 0.0);s.store_scalar(354, 0.0);s.store_scalar(355, 0.0);s.store_scalar(356, 0.0);s.store_scalar(357, 0.0);s.store_scalar(358, 0.0);s.store_scalar(359, 0.0);s.store_scalar(364, 0.0);s.store_scalar(366, 0.0);s.store_scalar(367, 0.0);s.store_scalar(368, 0.0);s.store_scalar(369, 0.0);s.store_scalar(370, 0.0);s.store_scalar(371, 0.0);s.store_scalar(372, 0.0);s.store_scalar(373, 0.0);s.store_scalar(374, 0.0);
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_3(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.store_scalar(375, 0.0);s.store_scalar(376, 0.0);s.store_scalar(377, 0.0);s.store_scalar(380, 0.0);s.store_scalar(381, 0.0);s.store_scalar(382, 0.0);s.store_scalar(383, 0.0);s.store_scalar(387, 0.0);s.store_scalar(388, 0.0);s.store_scalar(389, 0.0);s.store_scalar(390, 0.0);s.store_scalar(391, 0.0);s.store_scalar(392, 0.0);s.store_scalar(393, 0.0);s.store_scalar(394, 0.0);s.store_scalar(395, 0.0);s.store_scalar(396, 0.0);s.store_scalar(397, 0.0);s.store_scalar(398, 0.0);s.store_scalar(399, 0.0);s.store_scalar(400, 0.0);s.store_scalar(402, 0.0);s.store_scalar(403, 0.0);s.store_scalar(404, 0.0);s.store_scalar(405, 0.0);s.store_scalar(385, p.p334);s.store_scalar(386, p.p334);s.store_scalar(409, 0.0);s.store_scalar(410, 0.0);s.store_scalar(434, 0.0093868);s.store_scalar(435, (-0.1047839));s.store_scalar(447, 0.0);s.store_scalar(573, 0.0);s.store_scalar(574, 0.0);s.store_scalar(575, 0.0);s.store_scalar(576, 0.0);s.store_scalar(577, 0.0);s.store_scalar(578, 0.0);s.store_scalar(579, 0.0);s.store_scalar(580, 0.0);s.store_scalar(581, 0.0);s.store_scalar(582, 0.0);s.store_scalar(583, 0.0);s.store_scalar(584, 0.0);s.store_scalar(585, 0.0);s.store_scalar(586, 0.0);s.store_scalar(587, 0.0);s.store_scalar(588, 0.0);s.store_scalar(589, 0.0);s.store_scalar(590, 0.0);s.store_scalar(591, 0.0);s.store_scalar(592, 0.0);s.store_scalar(593, 0.0);s.store_scalar(594, 0.0);s.store_scalar(595, 0.0);s.store_scalar(596, 0.0);s.store_scalar(597, 0.0);s.store_scalar(739, 0.0);s.store_scalar(598, 0.0);s.store_scalar(770, 0.0);s.store_scalar(727, 0.0);s.store_scalar(728, 0.0);s.store_scalar(729, 0.0);s.store_scalar(730, 0.0);s.store_scalar(731, 0.0);s.store_scalar(732, 0.0);s.store_scalar(733, 0.0);s.store_scalar(734, 0.0);s.store_scalar(735, 0.0);s.store_scalar(740, 0.0);s.store_scalar(18, 0.0);s.store_scalar(741, 0.0);s.store_scalar(745, 0.0);s.store_scalar(746, 0.0);s.store_scalar(747, 0.0);s.store_scalar(748, 0.0);s.store_scalar(751, 0.0);s.store_scalar(752, 0.0);s.store_scalar(753, 0.0);s.store_scalar(757, 0.0);s.store_scalar(682, 0.0);s.store_scalar(688, 0.0);s.store_scalar(689, 0.0);s.store_scalar(787, 0.0);s.store_scalar(794, 0.0);s.store_scalar(788, 0.0);s.store_scalar(690, 0.0);s.store_scalar(692, 0.0);s.store_scalar(691, 0.0);s.store_scalar(693, 0.0);s.store_scalar(795, 0.0);s.store_scalar(676, 0.0);s.store_scalar(681, 0.0);s.store_scalar(678, 0.0);s.store_scalar(686, 0.0);s.store_scalar(687, 0.0);
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_4(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.store_scalar(694, 0.0);s.store_scalar(679, 0.0);s.store_scalar(683, 0.0);s.store_scalar(680, 0.0);s.store_scalar(677, 0.0);s.store_scalar(684, 0.0);s.store_scalar(685, 0.0);s.store_scalar(956, p.p436);s.store_scalar(959, p.p437);s.store_scalar(986, 0.0);s.store_scalar(987, 0.0);s.store_scalar(988, 0.0);s.store_scalar(961, 0.0);s.store_scalar(960, 0.0);s.store_scalar(427, p.p447);s.store_scalar(957, p.p193);s.store_scalar(977, 0.0);s.store_scalar(978, 0.0);s.store_scalar(421, 40.0);s.store_scalar(828, 0.0);s.store_scalar(829, 0.0);s.store_scalar(830, 0.0);s.store_scalar(831, 0.0);s.store_scalar(66, 0.0);s.store_scalar(65, 0.0);s.store_scalar(68, 0.0);s.store_scalar(67, 0.0);s.store_scalar(832, 0.0);s.store_scalar(833, 0.0);s.store_scalar(834, 0.0);s.store_scalar(835, 0.0);s.store_scalar(838, 0.0);s.store_scalar(839, 0.0);s.store_scalar(841, 0.0);s.store_scalar(842, 0.0);s.store_scalar(843, 0.0);s.store_scalar(844, 0.0);s.store_scalar(845, 0.0);s.store_scalar(846, 0.0);s.store_scalar(840, 0.0);s.store_scalar(857, 0.0);s.store_scalar(858, 0.0);s.store_scalar(859, 0.0);s.store_scalar(860, 0.0);s.store_scalar(865, 0.0);s.store_scalar(866, 0.0);s.store_scalar(867, 0.0);s.store_scalar(868, 0.0);s.store_scalar(849, 0.0);s.store_scalar(854, 0.0);s.store_scalar(847, 0.0);s.store_scalar(852, 0.0);s.store_scalar(851, 0.0);s.store_scalar(856, 0.0);s.store_scalar(848, 0.0);s.store_scalar(853, 0.0);s.store_scalar(850, 0.0);s.store_scalar(855, 0.0);s.store_scalar(946, 0.0);s.store_scalar(944, 0.0);s.store_scalar(947, 0.0);s.store_scalar(945, 0.0);s.store_scalar(948, 0.0);s.store_scalar(816, 0.0);s.store_scalar(873, 0.0);s.store_scalar(874, 0.0);s.store_scalar(875, 0.0);s.store_scalar(876, 0.0);s.store_scalar(877, 0.0);s.store_scalar(878, 0.0);s.store_scalar(879, 0.0);s.store_scalar(880, 0.0);s.store_scalar(881, 0.0);s.store_scalar(882, 0.0);s.store_scalar(883, 0.0);s.store_scalar(884, 0.0);s.store_scalar(360, 0.0);s.store_scalar(362, 0.0);s.store_scalar(361, 0.0);s.store_scalar(363, 0.0);s.store_scalar(603, 0.0);s.store_scalar(45, 0.0);s.store_scalar(46, 0.0);s.store_scalar(413, 0.0);s.store_scalar(932, 0.0);s.store_scalar(926, 0.0);s.store_scalar(927, 0.0);s.store_scalar(287, 0.0);s.store_scalar(407, 0.0);s.store_scalar(924, 0.0);s.store_scalar(925, 0.0);s.store_scalar(931, 0.0);s.store_scalar(990, 0.0);s.store_scalar(411, 0.0);s.store_scalar(288, 0.0);s.store_scalar(448, (if (p.p40 != 0.0) { 0.0 } else { p.p17 }));
    }
}
