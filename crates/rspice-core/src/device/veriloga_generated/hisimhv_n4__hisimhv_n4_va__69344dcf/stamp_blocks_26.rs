#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_reactive_block_197(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if s.b[3321] {s.store_div_scaled_product_by_product_mixed_aaai(315, A::mul3_scaled_output(s.ad_value(185), s.ad_value(127), s.ad_value(253), s.v[632]), A::add_scaled_inputs3(A::mul3(A::add_scaled_inputs(A::scale_offset(s.ad_value(125), 3.0, 1.0), 1.0, s.ad_value(334), 6.0), s.ad_value(316), s.ad_value(316)), 1.0, A::mul3(A::add_scaled_inputs(A::scale_offset(s.ad_value(125), 4.0, 3.0), 1.0, s.ad_value(334), 3.0), s.ad_value(316), s.ad_value(253)), 1.0, A::mul3(A::add(A::scale_offset(s.ad_value(125), 3.0, 6.0), s.ad_value(334)), s.ad_value(253), s.ad_value(253)), 1.0), 1.0, A::mul3_scaled_output(s.ad_value(170), A::offset(s.ad_value(125), 1.0), s.ad_value(314), 15.0), 314, 1.0);}
        if (!s.b[3321]) {s.store_scalar(315, 0.0);}
        s.b[3324] = (((((p.p31 != 0.0) && (p.p30 != 0.0)) && (s.v[321] == 1.0)) && (s.v[78] == 0.0)) && (s.v[963] == 0.0));s.store_scalar(3324, if s.b[3324] { 1.0 } else { 0.0 });
        if s.b[3324] {s.store_sqrt(322, 319);s.store_add(336, 127, 322);s.store_square(337, 317);s.store_square(338, 319);s.store_scaled_mul(339, 317, 319, 42.0);s.store_add_scaled_inputs3_indices(339, 339, 1.0, 337, 4.0, 338, 4.0);s.store_add_product3_rhs_mixed_iia(339, 339, 322, 127, A::add(s.ad_value(317), s.ad_value(319)), 20.0);s.store_square(344, 336);s.store_square(344, 344);s.store_div_scaled_value_by_product_indices(323, 339, 1.0, 344, 336, 1.0);s.store_mul_ad_product_lhs_mixed_ai(324, A::div_from_scalar(s.v[632], s.ad_value(170)), 253, 185);s.store_add_mixed_ai(341, A::add_scaled_product(s.ad_value(317), 1.0, s.ad_value(127), s.ad_value(322), 4.0), 319);}
        s.store_scale(0, 134, s.v[365]);s.store_scale(699, 400, s.v[365]);s.store_scalar(705, 0.0);s.store_scalar(706, 0.0);s.store_scalar(707, 0.0);s.store_scalar(811, 0.0);s.store_scalar(810, 0.0);s.store_scalar(812, 0.0);s.store_scalar(703, 0.0);s.store_scalar(704, 0.0);s.b[3325] = ((s.v[81] != 0.0) || (p.p22 == 2.0));s.store_scalar(3325, if s.b[3325] { 1.0 } else { 0.0 });
        if s.b[3325] {s.store_scalar(700, 0.0);s.store_scalar(701, 0.0);s.store_scalar(702, 0.0);s.copy_ad(708, 247);s.store_scale(132, 132, s.v[365]);}
        if (!s.b[3325]) {s.store_scaled_add(700, 20, 132, (-s.v[365]));s.store_scale(701, 19, s.v[365]);s.store_scaled_sub(702, 132, 19, s.v[365]);}
        if (p.p29 != 0.0) {s.store_scale(572, 91, s.v[572]);s.store_sqrt_square_offset(782, 572, ((4.0 * 1e-12) * 1e-12));s.store_offset_scaled_div(334, 572, 782, 0.5, 0.5);s.store_scaled_add(572, 572, 782, 0.5);}
        s.b[3326] = (s.v[572] < 0.0);s.store_scalar(3326, if s.b[3326] { 1.0 } else { 0.0 });
        if ((p.p29 != 0.0) && s.b[3326]) {s.store_scalar(572, 0.0);s.store_scalar(334, 0.0);}
        if (p.p29 != 0.0) {s.store_voltage(817, ctx, nodes, Some(13), None);s.store_add_scaled_inputs3_indices(352, 352, 1.0, 816, -1.0, 817, 1.0);s.copy_ad(355, 817);}
        if (p.p29 == 0.0) {s.copy_ad(817, 816);}
        s.b[3327] = (p.p22 > 0.0);s.store_scalar(3327, if s.b[3327] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_198(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        let ctx_temp = ctx.temperature();
        if s.b[3327] {s.store_scaled_add_mixed_ai(811, A::add_scaled_inputs4(s.ad_value(293), 1.0, s.ad_value(352), (-1.0), s.ad_value(353), -1.0, s.ad_value(291), 1.0), 292, s.v[365]);s.store_scaled_sub(810, 355, 292, s.v[365]);s.store_scaled_sub(812, 356, 291, s.v[365]);s.store_add_scaled_inputs4_indices(700, 700, 1.0, 305, s.v[365], 360, ((-1.0) * s.v[365]), 362, (-s.v[365]));s.store_add_scaled_inputs3_indices(701, 701, 1.0, 361, s.v[365], 305, (-s.v[365]));s.store_add_scaled_inputs(702, 702, 1.0, 363, s.v[365]);s.store_sub_scaled_inputs(705, 350, (-s.v[365]), 351, s.v[365]);s.store_scale(706, 358, s.v[365]);s.store_scale(707, 359, s.v[365]);s.store_offset_sub_scaled_inputs_indices(703, 299, (-s.v[365]), 298, s.v[365], s.v[703]);s.store_offset_sub_scaled_inputs_indices(704, 301, (-s.v[365]), 297, s.v[365], s.v[704]);}
        s.store_scaled_add(709, 280, 287, s.v[365]);s.store_scale(710, 281, s.v[365]);s.store_scale(807, 387, (4.0 * 1.3806226e-23));s.store_scale(712, 315, s.v[365]);s.store_scalar(22, A::ddx_projection(&s.ad_value(700), Some(5), None));s.store_scale(22, 22, p.p87);s.store_scalar(23, A::ddx_projection(&s.ad_value(700), Some(7), None));s.store_scale(23, 23, p.p87);
        if (s.v[949] > 0.0) {
            s.copy_ad(757, 23);
        } else {
            s.copy_ad(757, 22);
        }
        s.store_scalar(713, 0.0);s.b[3330] = (((((p.p31 != 0.0) && (p.p30 != 0.0)) && (s.v[321] == 1.0)) && (s.v[78] == 0.0)) && (s.v[963] == 0.0));s.store_scalar(3330, if s.b[3330] { 1.0 } else { 0.0 });
        if s.b[3330] {s.store_scaled_mul(334, 185, 162, (1e-6 * s.v[635]));s.store_scale(344, 757, 1.0 / (s.v[365]));s.store_div_scaled_product3_indices(328, 155, 344, 344, (0.1185185185185185 * 1.6021918e-19), 324, 1.0);}
        s.b[3331] = ((s.v[320] > (10.0 * 2.220446049250313e-16)) && (s.v[790] > (10.0 * 2.220446049250313e-16)));s.store_scalar(3331, if s.b[3331] { 1.0 } else { 0.0 });
        if (s.b[3330] && s.b[3331]) {s.store_div(329, 254, 253);s.store_div_scaled_inputs2_mixed_aii(330, A::div(s.ad_value(254), s.ad_value(316)), 1.0, 329, (-1.0), 790, 1.0);s.store_add_mixed_ia(331, 329, A::div_scaled_product(s.ad_value(330), A::add(A::add_scaled_product(s.ad_value(317), 1.0, s.ad_value(127), s.ad_value(322), 1.0), s.ad_value(319)), 0.6666666666666667, A::add(s.ad_value(127), s.ad_value(322)), 1.0));}
        if (s.b[3330] && (!s.b[3331])) {s.store_div(331, 254, 316);}
        if s.b[3330] {s.store_mul3_affine_lhs(713, 328, 323, s.v[365], 0.0, 331);}
        if s.b[3330] {
            if (s.v[713] < 0.0) {
                s.store_scalar(713, 0.0);
            } else {
            }
        }
        if s.b[3330] {
            if ((-s.v[344]) > s.v[334]) {
            } else {
                s.store_scalar(713, 0.0);
            }
        }
        s.store_mul(952, 807, 712);
        if ((s.v[952] > 0.0) && (s.v[713] > 0.0)) {
            s.store_sqrt_div(953, 713, 952);
        } else {
            s.store_scalar(953, 0.0);
        }
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
        s.store_scalar(716, 0.0);s.store_scalar(715, 0.0);s.b[3332] = (s.v[449] == 1.0);s.store_scalar(3332, if s.b[3332] { 1.0 } else { 0.0 });s.b[3333] = (s.v[76] == 0.0);s.store_scalar(3333, if s.b[3333] { 1.0 } else { 0.0 });s.b[3334] = ((p.p53 > 0.0) && (s.v[541] != 0.0));s.store_scalar(3334, if s.b[3334] { 1.0 } else { 0.0 });
        if ((s.b[3332] && (!s.b[3333])) && s.b[3334]) {
            if (s.v[676] == 0.0) {
                s.store_scalar(335, 0.0);
            } else {
                s.store_powf(335, 676, p.p416);
            }
        }
        if ((s.b[3332] && (!s.b[3333])) && s.b[3334]) {s.store_div_from_scalar(794, s.v[569], 335);s.store_sub_ad(334, A::add_scaled_product(A::scale_offset(s.ad_value(676), 0.4, 1.8), 1.0, s.ad_value(676), s.ad_value(676), 0.1), A::scale_offset(s.ad_value(676), (-p.p418), p.p418));s.store_div_from_scalar(795, s.v[570], 334);s.store_add_mixed_ia(959, 959, A::scaled_offset(s.ad_value(387), (-s.v[764]), p.p439));}
        if ((s.b[3332] && (!s.b[3333])) && (!s.b[3334])) {s.store_scalar(387, (ctx_temp + p.p11));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_199(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[3332] && (!s.b[3333])) {s.store_scalar(164, (s.v[630] * p.p7));s.store_scalar(604, p.p71);s.store_scalar(605, s.v[460]);s.store_mul(606, 794, 653);s.store_offset_product3(607, s.ad_value(795), s.ad_value(786), s.ad_value(652), 1.0, 1e-25);s.store_div(608, 804, 604);s.store_mul(609, 606, 608);}
        s.b[3335] = (s.v[804] >= 0.0);s.store_scalar(3335, if s.b[3335] { 1.0 } else { 0.0 });
        if ((s.b[3332] && (!s.b[3333])) && s.b[3335]) {s.store_div(335, 609, 607);}
        if ((s.b[3332] && (!s.b[3333])) && (!s.b[3335])) {s.store_div_scaled_inputs_indices(335, 609, -1.0, 607, 1.0);}
        s.b[3336] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= s.v[959]) && (s.v[959] <= (1.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(3336, if s.b[3336] { 1.0 } else { 0.0 });
        if ((s.b[3332] && (!s.b[3333])) && s.b[3336]) {s.store_scalar(337, 1.0);}
        s.b[3337] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= s.v[959]) && (s.v[959] <= (2.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(3337, if s.b[3337] { 1.0 } else { 0.0 });
        if (((s.b[3332] && (!s.b[3333])) && (!s.b[3336])) && s.b[3337]) {s.copy_ad(337, 335);}
        if (((s.b[3332] && (!s.b[3333])) && (!s.b[3336])) && (!s.b[3337])) {s.store_pow_offset_rhs(337, 335, 959, (-1.0));}
        if (s.b[3332] && (!s.b[3333])) {s.store_mul(336, 335, 337);s.store_offset(338, 336, 1.0);}
        s.b[3338] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= s.v[959]) && (s.v[959] <= (1.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(3338, if s.b[3338] { 1.0 } else { 0.0 });
        if ((s.b[3332] && (!s.b[3333])) && s.b[3338]) {s.store_div_from_scalar(339, 1.0, 338);}
        s.b[3339] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= s.v[959]) && (s.v[959] <= (2.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(3339, if s.b[3339] { 1.0 } else { 0.0 });
        if (((s.b[3332] && (!s.b[3333])) && (!s.b[3338])) && s.b[3339]) {s.store_div_from_scalar_sqrt_ad(339, 1.0, s.ad_value(338));}
        if (((s.b[3332] && (!s.b[3333])) && (!s.b[3338])) && (!s.b[3339])) {
            if (s.v[338] == 0.0) {
                s.store_scalar(340, 0.0);
            } else {
                s.store_pow_ad(340, s.ad_value(338), A::offset(A::div_from_scalar((-1.0), s.ad_value(959)), (-1.0)));
            }
        }
        if (((s.b[3332] && (!s.b[3333])) && (!s.b[3338])) && (!s.b[3339])) {s.store_mul(339, 338, 340);}
        if (s.b[3332] && (!s.b[3333])) {s.store_mul(610, 606, 339);s.copy_ad(611, 605);s.copy_ad(612, 614);s.store_div_from_scalar(335, 1.6021918e-19, 604);s.store_mul_product3_indices(613, 611, 335, 612, 610, 1.0);}
        s.b[3340] = ((s.v[613] < 1e-25) && (1e-25 >= 0.0));s.store_scalar(3340, if s.b[3340] { 1.0 } else { 0.0 });
        if ((s.b[3332] && (!s.b[3333])) && s.b[3340]) {s.store_sub_from_scalar(781, 1e-25, 613);s.store_square(722, 781);s.store_scalar(723, (1e-25 * 1e-25));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[3341] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(3341, if s.b[3341] { 1.0 } else { 0.0 });s.b[3342] = (2.0 == 1.0);s.store_scalar(3342, if s.b[3342] { 1.0 } else { 0.0 });
        if ((((s.b[3332] && (!s.b[3333])) && s.b[3340]) && s.b[3341]) && s.b[3342]) {s.store_scalar(720, 1.0);}
        s.b[3343] = (2.0 == 2.0);s.store_scalar(3343, if s.b[3343] { 1.0 } else { 0.0 });
        if (((((s.b[3332] && (!s.b[3333])) && s.b[3340]) && s.b[3341]) && (!s.b[3342])) && s.b[3343]) {s.store_scalar(720, 2.0);}
        s.b[3344] = (2.0 == 4.0);s.store_scalar(3344, if s.b[3344] { 1.0 } else { 0.0 });
        if ((((((s.b[3332] && (!s.b[3333])) && s.b[3340]) && s.b[3341]) && (!s.b[3342])) && (!s.b[3343])) && s.b[3344]) {s.store_scalar(720, 3.0);}
        s.b[3345] = (2.0 == 8.0);s.store_scalar(3345, if s.b[3345] { 1.0 } else { 0.0 });
        if (((((((s.b[3332] && (!s.b[3333])) && s.b[3340]) && s.b[3341]) && (!s.b[3342])) && (!s.b[3343])) && (!s.b[3344])) && s.b[3345]) {s.store_scalar(720, 4.0);}
        if (((s.b[3332] && (!s.b[3333])) && s.b[3340]) && s.b[3341]) {s.store_scalar(719, 0.0);}
        let mut t1: usize = 0;
        while {
            let t0: f64 = if ((((s.b[3332] && (!s.b[3333])) && s.b[3340]) && s.b[3341]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t0 != 0.0
        } {
            t1 += 1;assert!(t1 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[3332] && (!s.b[3333])) && s.b[3340]) && s.b[3341]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
        }
        if (((s.b[3332] && (!s.b[3333])) && s.b[3340]) && (!s.b[3341])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if ((s.b[3332] && (!s.b[3333])) && s.b[3340]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 1e-25);s.store_div_scaled_product_indices(334, 725, 726, 1e-25, 770, 1.0);s.store_sub_from_scalar(613, 1e-25, 780);}
        if ((s.b[3332] && (!s.b[3333])) && s.b[3340]) {
        }
        if ((s.b[3332] && (!s.b[3333])) && (!s.b[3340])) {
        }
        if ((s.b[3332] && (!s.b[3333])) && (!s.b[3340])) {s.store_scalar(334, 1.0);}
        if (s.b[3332] && (!s.b[3333])) {s.store_div_from_scalar(5, 1.0, 613);s.store_div(5, 5, 164);s.store_add(5, 5, 648);}
        s.b[3347] = (s.v[5] < p.p444);s.store_scalar(3347, if s.b[3347] { 1.0 } else { 0.0 });
        if ((s.b[3332] && (!s.b[3333])) && s.b[3347]) {s.store_scalar(5, p.p444);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_200(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        let ctx_temp = ctx.temperature();
        if (s.b[3332] && (!s.b[3333])) {s.store_scale(716, 5, 1.0 / (s.v[365]));}
        s.b[3352] = (s.v[75] == 0.0);s.store_scalar(3352, if s.b[3352] { 1.0 } else { 0.0 });
        if (s.b[3332] && (!s.b[3352])) {s.copy_ad(3348, 729);s.copy_ad(3349, 728);}
        s.b[3353] = ((p.p53 > 0.0) && (s.v[541] != 0.0));s.store_scalar(3353, if s.b[3353] { 1.0 } else { 0.0 });
        if ((s.b[3332] && (!s.b[3352])) && s.b[3353]) {
            if (s.v[676] == 0.0) {
                s.store_scalar(335, 0.0);
            } else {
                s.store_powf(335, 676, p.p415);
            }
        }
        if ((s.b[3332] && (!s.b[3352])) && s.b[3353]) {s.store_div_from_scalar(787, s.v[567], 335);s.store_sub_ad(334, A::add_scaled_product(A::scale_offset(s.ad_value(676), 0.4, 1.8), 1.0, s.ad_value(676), s.ad_value(676), 0.1), A::scale_offset(s.ad_value(676), (-p.p417), p.p417));s.store_div_from_scalar(788, s.v[568], 334);s.store_add_mixed_ia(956, 956, A::scaled_offset(s.ad_value(387), (-s.v[764]), p.p438));}
        s.b[3355] = (s.v[956] < 0.1);s.store_scalar(3355, if s.b[3355] { 1.0 } else { 0.0 });
        if (((s.b[3332] && (!s.b[3352])) && s.b[3353]) && s.b[3355]) {s.store_scalar(956, 0.1);}
        if ((s.b[3332] && (!s.b[3352])) && (!s.b[3353])) {s.store_scalar(387, (ctx_temp + p.p11));}
        if (s.b[3332] && (!s.b[3352])) {s.store_scalar(164, (s.v[630] * p.p7));s.store_scalar(785, (p.p67 + p.p68));s.store_primal_offset(789, 451, 1e-12);s.store_scalar(408, s.v[459]);s.store_offset_ad(335, A::mul_sub_from_scalar_rhs(s.ad_value(3349), p.p410, A::scale(s.ad_value(3349), p.p411)), 1.0);s.store_sqrt_square_offset(782, 335, ((4.0 * 0.1) * 0.1));s.store_offset_scaled_div(336, 335, 782, 0.5, 0.5);s.store_scaled_add(654, 335, 782, 0.5);}
        s.b[3356] = (s.v[654] < 0.0);s.store_scalar(3356, if s.b[3356] { 1.0 } else { 0.0 });
        if ((s.b[3332] && (!s.b[3352])) && s.b[3356]) {s.store_scalar(654, 0.0);s.store_scalar(336, 0.0);}
        if (s.b[3332] && (!s.b[3352])) {s.store_mul3_lhs(593, 787, 653, 654);s.store_offset_product3(3351, s.ad_value(788), s.ad_value(786), s.ad_value(652), 1.0, 1e-25);s.copy_ad(594, 453);s.store_scalar(595, p.p421);s.store_scale(335, 593, 10000.0);s.store_scale(336, 3351, 100.0);}
        s.b[3359] = (s.v[799] < 0.0);s.store_scalar(3359, if s.b[3359] { 1.0 } else { 0.0 });
        if ((s.b[3332] && (!s.b[3352])) && s.b[3359]) {s.store_scale(781, 799, ((-0.5) * (2.0 * 1.0 / (p.p262))));s.store_offset_mul_offset_rhs_mixed_ia(782, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 5040.0), (1.0 / 720.0))), (1.0 / 120.0)), (1.0 / 24.0)), (1.0 / 6.0)), (1.0 / 2.0), 1.0);s.store_offset_mul_offset_rhs_mixed_ia(783, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 840.0), (1.0 / 144.0))), (1.0 / 30.0)), (1.0 / 8.0)), (1.0 / 3.0), (1.0 / 2.0));s.store_div_from_scalar(108, p.p262, 782);s.store_div_scaled_inputs_square_rhs(336, 783, (-2.0), 782, 1.0);}
        s.b[3360] = (s.v[108] < 1e-12);s.store_scalar(3360, if s.b[3360] { 1.0 } else { 0.0 });
        if (((s.b[3332] && (!s.b[3352])) && s.b[3359]) && s.b[3360]) {s.store_scalar(108, 1e-12);}
        if ((s.b[3332] && (!s.b[3352])) && s.b[3359]) {s.store_sub_scaled_inputs(598, 799, 1.0, 108, 2.0);}
        if ((s.b[3332] && (!s.b[3352])) && (!s.b[3359])) {s.store_scale(781, 799, (0.5 * (2.0 * 1.0 / (p.p262))));s.store_offset_mul_offset_rhs_mixed_ia(782, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 5040.0), (1.0 / 720.0))), (1.0 / 120.0)), (1.0 / 24.0)), (1.0 / 6.0)), (1.0 / 2.0), 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_201(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[3332] && (!s.b[3352])) && (!s.b[3359])) {s.store_offset_mul_offset_rhs_mixed_ia(783, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 840.0), (1.0 / 144.0))), (1.0 / 30.0)), (1.0 / 8.0)), (1.0 / 3.0), (1.0 / 2.0));s.store_div_from_scalar(108, p.p262, 782);s.store_div_scaled_inputs_square_rhs(336, 783, (-2.0), 782, 1.0);}
        s.b[3361] = (s.v[108] < 1e-12);s.store_scalar(3361, if s.b[3361] { 1.0 } else { 0.0 });
        if (((s.b[3332] && (!s.b[3352])) && (!s.b[3359])) && s.b[3361]) {s.store_scalar(108, 1e-12);}
        if ((s.b[3332] && (!s.b[3352])) && (!s.b[3359])) {s.store_add_scaled_inputs(598, 799, 1.0, 108, 2.0);}
        if (s.b[3332] && (!s.b[3352])) {s.store_div(591, 598, 785);s.store_mul(592, 593, 591);}
        s.b[3362] = (s.v[799] >= 0.0);s.store_scalar(3362, if s.b[3362] { 1.0 } else { 0.0 });
        if ((s.b[3332] && (!s.b[3352])) && s.b[3362]) {s.store_div(335, 592, 3351);}
        if ((s.b[3332] && (!s.b[3352])) && (!s.b[3362])) {s.store_div_scaled_inputs_indices(335, 592, -1.0, 3351, 1.0);}
        s.b[3363] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= s.v[956]) && (s.v[956] <= (1.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(3363, if s.b[3363] { 1.0 } else { 0.0 });
        if ((s.b[3332] && (!s.b[3352])) && s.b[3363]) {s.store_scalar(337, 1.0);}
        s.b[3364] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= s.v[956]) && (s.v[956] <= (2.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(3364, if s.b[3364] { 1.0 } else { 0.0 });
        if (((s.b[3332] && (!s.b[3352])) && (!s.b[3363])) && s.b[3364]) {s.copy_ad(337, 335);}
        if (((s.b[3332] && (!s.b[3352])) && (!s.b[3363])) && (!s.b[3364])) {s.store_pow_offset_rhs(337, 335, 956, (-1.0));}
        if (s.b[3332] && (!s.b[3352])) {s.store_mul(336, 335, 337);s.store_offset(338, 336, 1.0);}
        s.b[3365] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= s.v[956]) && (s.v[956] <= (1.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(3365, if s.b[3365] { 1.0 } else { 0.0 });
        if ((s.b[3332] && (!s.b[3352])) && s.b[3365]) {s.store_div_from_scalar(339, 1.0, 338);}
        s.b[3366] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= s.v[956]) && (s.v[956] <= (2.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(3366, if s.b[3366] { 1.0 } else { 0.0 });
        if (((s.b[3332] && (!s.b[3352])) && (!s.b[3365])) && s.b[3366]) {s.store_div_from_scalar_sqrt_ad(339, 1.0, s.ad_value(338));}
        if (((s.b[3332] && (!s.b[3352])) && (!s.b[3365])) && (!s.b[3366])) {
            if (s.v[338] == 0.0) {
                s.store_scalar(340, 0.0);
            } else {
                s.store_pow_ad(340, s.ad_value(338), A::offset(A::div_from_scalar((-1.0), s.ad_value(956)), (-1.0)));
            }
        }
        if (((s.b[3332] && (!s.b[3352])) && (!s.b[3365])) && (!s.b[3366])) {s.store_mul(339, 338, 340);}
        if (s.b[3332] && (!s.b[3352])) {s.store_mul(3350, 593, 339);s.store_offset(338, 335, 1.0);s.store_div_from_scalar(339, 1.0, 338);s.store_offset_ad(338, A::div_scaled_product_offset_denominator(A::mul_sub_from_scalar_rhs(s.ad_value(595), 1.0, s.ad_value(339)), s.ad_value(598), 1.0, s.ad_value(785), (-p.p423), 1.0), 1.0);s.store_offset(781, 338, (-0.001));s.store_scalar(782, 0.0);}
        if (s.b[3332] && (!s.b[3352])) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if (s.b[3332] && (!s.b[3352])) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_scaled_add(339, 781, 782, 0.5);s.store_mul(717, 408, 339);s.store_scale(718, 698, (6.241449993689894e18 * p.p430));s.store_add_scaled_inputs3_indices(781, 717, 1.0, 718, (-1.0), 717, (-0.001));s.store_scaled_mul(782, 717, 717, (4.0 * 0.001));}
        if (s.b[3332] && (!s.b[3352])) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if (s.b[3332] && (!s.b[3352])) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_add_scaled_inputs3_indices(718, 717, 1.0, 781, (-0.5), 782, (-0.5));s.store_sub(597, 717, 718);}
        s.b[3367] = ((p.p441 > 0.0) && (p.p440 > 1.0));s.store_scalar(3367, if s.b[3367] { 1.0 } else { 0.0 });s.b[3368] = ((s.v[597] > ((s.v[408] * p.p440) - (s.v[408] * p.p441))) && ((s.v[408] * p.p441) >= 0.0));s.store_scalar(3368, if s.b[3368] { 1.0 } else { 0.0 });
        if (((s.b[3332] && (!s.b[3352])) && s.b[3367]) && s.b[3368]) {s.store_add_scaled_inputs3_indices(781, 597, 1.0, 408, (-p.p440), 408, p.p441);s.store_square(722, 781);s.store_scaled_mul(723, 408, 408, (p.p441 * p.p441));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_scalar(719, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_202(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        let mut t3: usize = 0;
        while {
            let t2: f64 = if ((((s.b[3332] && (!s.b[3352])) && s.b[3367]) && s.b[3368]) && (s.v[719] < p.p442)) { 1.0 } else { 0.0 };
            t2 != 0.0
        } {
            t3 += 1;assert!(t3 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[3332] && (!s.b[3352])) && s.b[3367]) && s.b[3368]) {s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_primal_offset(719, 719, 1.0);}
        }
        if (((s.b[3332] && (!s.b[3352])) && s.b[3367]) && s.b[3368]) {s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[3369] = ((((p.p442 == 1.0) || (p.p442 == 2.0)) || (p.p442 == 4.0)) || (p.p442 == 8.0));s.store_scalar(3369, if s.b[3369] { 1.0 } else { 0.0 });s.b[3370] = (p.p442 == 1.0);s.store_scalar(3370, if s.b[3370] { 1.0 } else { 0.0 });
        if (((((s.b[3332] && (!s.b[3352])) && s.b[3367]) && s.b[3368]) && s.b[3369]) && s.b[3370]) {s.store_scalar(720, 1.0);}
        s.b[3371] = (p.p442 == 2.0);s.store_scalar(3371, if s.b[3371] { 1.0 } else { 0.0 });
        if ((((((s.b[3332] && (!s.b[3352])) && s.b[3367]) && s.b[3368]) && s.b[3369]) && (!s.b[3370])) && s.b[3371]) {s.store_scalar(720, 2.0);}
        s.b[3372] = (p.p442 == 4.0);s.store_scalar(3372, if s.b[3372] { 1.0 } else { 0.0 });
        if (((((((s.b[3332] && (!s.b[3352])) && s.b[3367]) && s.b[3368]) && s.b[3369]) && (!s.b[3370])) && (!s.b[3371])) && s.b[3372]) {s.store_scalar(720, 3.0);}
        s.b[3373] = (p.p442 == 8.0);s.store_scalar(3373, if s.b[3373] { 1.0 } else { 0.0 });
        if ((((((((s.b[3332] && (!s.b[3352])) && s.b[3367]) && s.b[3368]) && s.b[3369]) && (!s.b[3370])) && (!s.b[3371])) && (!s.b[3372])) && s.b[3373]) {s.store_scalar(720, 4.0);}
        if ((((s.b[3332] && (!s.b[3352])) && s.b[3367]) && s.b[3368]) && s.b[3369]) {s.store_scalar(719, 0.0);}
        let mut t5: usize = 0;
        while {
            let t4: f64 = if (((((s.b[3332] && (!s.b[3352])) && s.b[3367]) && s.b[3368]) && s.b[3369]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t4 != 0.0
        } {
            t5 += 1;assert!(t5 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[3332] && (!s.b[3352])) && s.b[3367]) && s.b[3368]) && s.b[3369]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
        }
        if ((((s.b[3332] && (!s.b[3352])) && s.b[3367]) && s.b[3368]) && (!s.b[3369])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * p.p442)));
            }
        }
        if (((s.b[3332] && (!s.b[3352])) && s.b[3367]) && s.b[3368]) {s.store_div_from_scalar(726, 1.0, 726);s.store_mul3_affine_lhs(780, 781, 408, p.p441, 0.0, 726);s.store_div_scaled_product3_indices(334, 408, 725, 726, p.p441, 770, 1.0);s.store_add_scaled_inputs3_indices(336, 408, p.p440, 408, (-p.p441), 780, 1.0);}
        if (((s.b[3332] && (!s.b[3352])) && s.b[3367]) && s.b[3368]) {
        }
        if (((s.b[3332] && (!s.b[3352])) && s.b[3367]) && (!s.b[3368])) {s.copy_ad(336, 597);s.store_scalar(334, 1.0);}
        if ((s.b[3332] && (!s.b[3352])) && s.b[3367]) {s.copy_ad(597, 336);}
        if (s.b[3332] && (!s.b[3352])) {s.store_neg(334, 697);s.store_sqrt_square_offset(782, 334, ((4.0 * 0.01) * 0.01));s.store_offset_scaled_div(343, 334, 782, 0.5, 0.5);s.store_scaled_add(334, 334, 782, 0.5);}
        s.b[3374] = (s.v[334] < 0.0);s.store_scalar(3374, if s.b[3374] { 1.0 } else { 0.0 });
        if ((s.b[3332] && (!s.b[3352])) && s.b[3374]) {s.store_scalar(334, 0.0);s.store_scalar(343, 0.0);}
        if (s.b[3332] && (!s.b[3352])) {s.store_offset(334, 334, (10.0 * 2.220446049250313e-16));s.store_sqrt_mul(599, 650, 334);s.store_offset_sub(336, 3348, 3349, p.p137);s.store_sqrt_square_offset(782, 336, ((4.0 * 0.01) * 0.01));s.store_offset_scaled_div(343, 336, 782, 0.5, 0.5);s.store_scaled_add(336, 336, 782, 0.5);}
        s.b[3375] = (s.v[336] < 0.0);s.store_scalar(3375, if s.b[3375] { 1.0 } else { 0.0 });
        if ((s.b[3332] && (!s.b[3352])) && s.b[3375]) {s.store_scalar(336, 0.0);s.store_scalar(343, 0.0);}
        if (s.b[3332] && (!s.b[3352])) {s.store_offset(336, 336, (10.0 * 2.220446049250313e-16));s.store_sqrt_mul(600, 651, 336);s.store_add_scaled_inputs3_indices(781, 789, 1.0, 600, (-1.0), 789, (-0.01));s.store_scaled_mul(782, 789, 789, (4.0 * 0.01));}
        if (s.b[3332] && (!s.b[3352])) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if (s.b[3332] && (!s.b[3352])) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_add_scaled_inputs3_indices(602, 789, 1.0, 781, (-0.5), 782, (-0.5));s.store_scalar(601, (p.p419 + 1e-25));s.store_mul_scale_offset_mixed_ia(596, 649, A::mul(s.ad_value(594), A::add(A::div(s.ad_value(599), s.ad_value(601)), A::div(s.ad_value(602), s.ad_value(789)))), -1.0, 1.0);s.store_sqrt_ad(782, A::add_scaled_square_product(s.ad_value(596), 1.0, A::mul_sub_from_scalar_lhs(1.0, s.ad_value(453), s.ad_value(649)), A::mul_sub_from_scalar_lhs(1.0, s.ad_value(453), s.ad_value(649)), ((1.0 / (100.0) * 4.0) * 1.0 / (100.0))));s.store_offset_scaled_div(343, 596, 782, 0.5, 0.5);s.store_scaled_add(596, 596, 782, 0.5);}
        s.b[3376] = (s.v[596] < 0.0);s.store_scalar(3376, if s.b[3376] { 1.0 } else { 0.0 });
        if ((s.b[3332] && (!s.b[3352])) && s.b[3376]) {s.store_scalar(596, 0.0);s.store_scalar(343, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_203(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[3332] && (!s.b[3352])) {s.store_div_from_scalar_offset_input(335, 1.6021918e-19, 785, p.p422);s.store_mul_product3_indices(739, 597, 335, 596, 3350, 1.0);}
        s.b[3377] = ((s.v[739] < 1e-25) && (1e-25 >= 0.0));s.store_scalar(3377, if s.b[3377] { 1.0 } else { 0.0 });
        if ((s.b[3332] && (!s.b[3352])) && s.b[3377]) {s.store_sub_from_scalar(781, 1e-25, 739);s.store_square(722, 781);s.store_scalar(723, (1e-25 * 1e-25));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[3378] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(3378, if s.b[3378] { 1.0 } else { 0.0 });s.b[3379] = (2.0 == 1.0);s.store_scalar(3379, if s.b[3379] { 1.0 } else { 0.0 });
        if ((((s.b[3332] && (!s.b[3352])) && s.b[3377]) && s.b[3378]) && s.b[3379]) {s.store_scalar(720, 1.0);}
        s.b[3380] = (2.0 == 2.0);s.store_scalar(3380, if s.b[3380] { 1.0 } else { 0.0 });
        if (((((s.b[3332] && (!s.b[3352])) && s.b[3377]) && s.b[3378]) && (!s.b[3379])) && s.b[3380]) {s.store_scalar(720, 2.0);}
        s.b[3381] = (2.0 == 4.0);s.store_scalar(3381, if s.b[3381] { 1.0 } else { 0.0 });
        if ((((((s.b[3332] && (!s.b[3352])) && s.b[3377]) && s.b[3378]) && (!s.b[3379])) && (!s.b[3380])) && s.b[3381]) {s.store_scalar(720, 3.0);}
        s.b[3382] = (2.0 == 8.0);s.store_scalar(3382, if s.b[3382] { 1.0 } else { 0.0 });
        if (((((((s.b[3332] && (!s.b[3352])) && s.b[3377]) && s.b[3378]) && (!s.b[3379])) && (!s.b[3380])) && (!s.b[3381])) && s.b[3382]) {s.store_scalar(720, 4.0);}
        if (((s.b[3332] && (!s.b[3352])) && s.b[3377]) && s.b[3378]) {s.store_scalar(719, 0.0);}
        let mut t7: usize = 0;
        while {
            let t6: f64 = if ((((s.b[3332] && (!s.b[3352])) && s.b[3377]) && s.b[3378]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t6 != 0.0
        } {
            t7 += 1;assert!(t7 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[3332] && (!s.b[3352])) && s.b[3377]) && s.b[3378]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
        }
        if (((s.b[3332] && (!s.b[3352])) && s.b[3377]) && (!s.b[3378])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if ((s.b[3332] && (!s.b[3352])) && s.b[3377]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 1e-25);s.store_div_scaled_product_indices(334, 725, 726, 1e-25, 770, 1.0);s.store_sub_from_scalar(739, 1e-25, 780);}
        if ((s.b[3332] && (!s.b[3352])) && s.b[3377]) {
        }
        if ((s.b[3332] && (!s.b[3352])) && (!s.b[3377])) {
        }
        if ((s.b[3332] && (!s.b[3352])) && (!s.b[3377])) {s.store_scalar(334, 1.0);}
        if (s.b[3332] && (!s.b[3352])) {s.store_div_from_scalar(4, 1.0, 739);s.store_div(4, 4, 164);}
        s.b[3383] = ((s.v[4] > (1000000.0 - 1000.0)) && (1000.0 >= 0.0));s.store_scalar(3383, if s.b[3383] { 1.0 } else { 0.0 });
        if ((s.b[3332] && (!s.b[3352])) && s.b[3383]) {s.store_offset(781, 4, (((-1000000.0)) + (1000.0)));s.store_square(722, 781);s.store_scalar(723, (1000.0 * 1000.0));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[3384] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(3384, if s.b[3384] { 1.0 } else { 0.0 });s.b[3385] = (2.0 == 1.0);s.store_scalar(3385, if s.b[3385] { 1.0 } else { 0.0 });
        if ((((s.b[3332] && (!s.b[3352])) && s.b[3383]) && s.b[3384]) && s.b[3385]) {s.store_scalar(720, 1.0);}
        s.b[3386] = (2.0 == 2.0);s.store_scalar(3386, if s.b[3386] { 1.0 } else { 0.0 });
        if (((((s.b[3332] && (!s.b[3352])) && s.b[3383]) && s.b[3384]) && (!s.b[3385])) && s.b[3386]) {s.store_scalar(720, 2.0);}
        s.b[3387] = (2.0 == 4.0);s.store_scalar(3387, if s.b[3387] { 1.0 } else { 0.0 });
        if ((((((s.b[3332] && (!s.b[3352])) && s.b[3383]) && s.b[3384]) && (!s.b[3385])) && (!s.b[3386])) && s.b[3387]) {s.store_scalar(720, 3.0);}
        s.b[3388] = (2.0 == 8.0);s.store_scalar(3388, if s.b[3388] { 1.0 } else { 0.0 });
        if (((((((s.b[3332] && (!s.b[3352])) && s.b[3383]) && s.b[3384]) && (!s.b[3385])) && (!s.b[3386])) && (!s.b[3387])) && s.b[3388]) {s.store_scalar(720, 4.0);}
        if (((s.b[3332] && (!s.b[3352])) && s.b[3383]) && s.b[3384]) {s.store_scalar(719, 0.0);}
        let mut t9: usize = 0;
        while {
            let t8: f64 = if ((((s.b[3332] && (!s.b[3352])) && s.b[3383]) && s.b[3384]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t8 != 0.0
        } {
            t9 += 1;assert!(t9 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[3332] && (!s.b[3352])) && s.b[3383]) && s.b[3384]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
        }
        if (((s.b[3332] && (!s.b[3352])) && s.b[3383]) && (!s.b[3384])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if ((s.b[3332] && (!s.b[3352])) && s.b[3383]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 1000.0);s.store_div_scaled_product_indices(334, 725, 726, 1000.0, 770, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_204(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[3332] && (!s.b[3352])) && s.b[3383]) {s.store_offset(4, 780, (1000000.0 - 1000.0));}
        if ((s.b[3332] && (!s.b[3352])) && s.b[3383]) {
        }
        if ((s.b[3332] && (!s.b[3352])) && (!s.b[3383])) {
        }
        if ((s.b[3332] && (!s.b[3352])) && (!s.b[3383])) {s.store_scalar(334, 1.0);}
        s.b[3389] = ((p.p54 == 1.0) && ((s.v[459] * (s.v[544] + s.v[459])) > 0.0));s.store_scalar(3389, if s.b[3389] { 1.0 } else { 0.0 });
        if ((s.b[3332] && (!s.b[3352])) && s.b[3389]) {s.store_sub_from_scalar(385, p.p334, 384);s.store_div_scaled_inputs_indices(4, 4, s.v[165], 385, 1.0);}
        if (s.b[3332] && (!s.b[3352])) {s.store_add(4, 4, 644);}
        s.b[3391] = (s.v[4] < p.p444);s.store_scalar(3391, if s.b[3391] { 1.0 } else { 0.0 });
        if ((s.b[3332] && (!s.b[3352])) && s.b[3391]) {s.store_scalar(4, p.p444);}
        if (s.b[3332] && (!s.b[3352])) {s.store_scale(715, 4, 1.0 / (s.v[365]));}
        s.b[3392] = (s.v[4] < p.p444);s.store_scalar(3392, if s.b[3392] { 1.0 } else { 0.0 });
        if ((!s.b[3332]) && s.b[3392]) {s.store_scalar(4, p.p444);}
        s.b[3393] = (s.v[5] < p.p444);s.store_scalar(3393, if s.b[3393] { 1.0 } else { 0.0 });
        if ((!s.b[3332]) && s.b[3393]) {s.store_scalar(5, p.p444);}
        s.b[3394] = (s.v[370] > 0.0);s.store_scalar(3394, if s.b[3394] { 1.0 } else { 0.0 });
        if ((!s.b[3332]) && s.b[3394]) {s.store_scale(715, 4, 1.0 / (s.v[365]));s.store_scale(716, 5, 1.0 / (s.v[365]));}
        if ((!s.b[3332]) && (!s.b[3394])) {s.store_scale(715, 5, 1.0 / (s.v[365]));s.store_scale(716, 4, 1.0 / (s.v[365]));}
        s.copy_ad(4, 715);s.copy_ad(5, 716);s.b[3395] = (s.v[949] > 0.0);s.store_scalar(3395, if s.b[3395] { 1.0 } else { 0.0 });
        if s.b[3395] {s.copy_ad(134, 0);s.copy_ad(19, 701);s.copy_ad(18, 700);s.copy_ad(741, 702);s.store_add_scaled_inputs3_indices(20, 700, (-1.0), 701, (-1.0), 702, (-1.0));s.copy_ad(280, 709);s.copy_ad(281, 710);s.copy_ad(400, 699);}
        if (s.b[3395] && (s.v[81] != 0.0)) {s.copy_ad(247, 708);}
        if (!s.b[3395]) {s.store_neg(134, 0);s.copy_ad(19, 702);s.copy_ad(18, 700);s.copy_ad(741, 701);s.store_add_scaled_inputs3_indices(20, 700, (-1.0), 701, (-1.0), 702, (-1.0));s.store_scalar(280, 0.0);s.store_scalar(281, 0.0);s.store_scalar(400, 0.0);}
        if ((!s.b[3395]) && (s.v[81] != 0.0)) {s.store_sub_from_scalar(247, 1.0, 708);}
        s.store_add(18, 18, 811);s.store_add(19, 19, 810);s.store_add(741, 741, 812);s.store_add_scaled_inputs3_indices(20, 18, (-1.0), 19, (-1.0), 741, (-1.0));s.copy_ad(299, 703);s.copy_ad(301, 704);s.copy_ad(742, 706);s.copy_ad(743, 705);s.store_add_scaled_inputs3_indices(744, 705, (-1.0), 706, (-1.0), 707, (-1.0));s.b[3396] = (p.p53 > 0.0);s.store_scalar(3396, if s.b[3396] { 1.0 } else { 0.0 });s.b[3397] = (s.v[766] > 0.0001);s.store_scalar(3397, if s.b[3397] { 1.0 } else { 0.0 });
        if (s.b[3396] && s.b[3397]) {s.store_div_from_scalar(740, 1.0, 766);}
        if (s.b[3396] && (!s.b[3397])) {s.store_scalar(740, (1.0 / 0.0001));}
        s.b[3398] = ((s.v[729] * (s.v[733] - s.v[729])) >= 0.0);s.store_scalar(3398, if s.b[3398] { 1.0 } else { 0.0 });s.b[3399] = (s.v[529] == 1.0);s.store_scalar(3399, if s.b[3399] { 1.0 } else { 0.0 });
        if ((s.b[3396] && s.b[3398]) && s.b[3399]) {s.copy_ad(745, 733);}
        if ((s.b[3396] && s.b[3398]) && (!s.b[3399])) {s.store_add_scaled_product_right_sub(745, 729, 1.0, 683, 733, 729, 1.0);}
        if (s.b[3396] && (!s.b[3398])) {s.copy_ad(745, 729);}
        if s.b[3396] {s.store_mul(746, 134, 745);}
        s.b[3400] = (p.p53 == 1.0);s.store_scalar(3400, if s.b[3400] { 1.0 } else { 0.0 });
        if (s.b[3396] && s.b[3400]) {s.store_scale(335, 740, p.p433);s.store_add_scaled_inputs3_indices(781, 335, 1.0, 746, (-1.0), 740, (-p.p337));s.store_scaled_mul(782, 335, 740, (4.0 * p.p337));}
        if (s.b[3396] && s.b[3400]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if (s.b[3396] && s.b[3400]) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_add_scaled_inputs3_indices(336, 335, 1.0, 781, (-0.5), 782, (-0.5));s.copy_ad(746, 336);}
        if (!s.b[3396]) {s.store_scalar(740, 0.0);s.store_scalar(746, 0.0);}
        if (s.v[81] != 0.0) {s.store_mul(751, 747, 247);s.store_sub_scaled_inputs(753, 747, -1.0, 748, 1.0);s.store_mul_scale_offset_indices(752, 747, 247, -1.0, 1.0);}
        if (s.v[81] == 0.0) {s.store_scalar(751, 0.0);s.store_scalar(753, 0.0);s.store_scalar(752, 0.0);}
        s.store_scaled_mul(0, 949, 134, p.p87);s.store_scalar(22, A::ddx_projection(&s.ad_value(18), Some(5), None));s.store_scale(22, 22, p.p87);
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_205(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.store_scalar(23, A::ddx_projection(&s.ad_value(18), Some(7), None));s.store_scale(23, 23, p.p87);s.b[3403] = (s.v[949] == 1.0);s.store_scalar(3403, if s.b[3403] { 1.0 } else { 0.0 });
        if s.b[3403] {s.copy_ad(757, 23);}
        if (!s.b[3403]) {s.copy_ad(757, 22);}
        s.b[3405] = (p.p48 > 0.0);s.store_scalar(3405, if s.b[3405] { 1.0 } else { 0.0 });s.b[3409] = (p.p53 > 0.0);s.store_scalar(3409, if s.b[3409] { 1.0 } else { 0.0 });
        if (!s.b[3409]) {s.store_scalar(767, 0.0);}
        if (p.p28 != 0.0) {s.store_scalar(800, 1.0);s.store_scalar(801, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_0(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_previous_value_scale: f64,
        ddt_older_value_scale: f64,
        ddt_previous_derivative_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_older: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
        ddt_derivative_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_derivative_previous: &mut [f64; Instance::DDT_STATE_COUNT],
    ) {
        let (eq0_e1028, eq0_e1028_d_n0, eq0_e1028_d_n1, eq0_e1028_d_n2, eq0_e1028_d_n3, eq0_e1028_d_n4, eq0_e1028_d_n5, eq0_e1028_d_n6, eq0_e1028_d_n7, eq0_e1028_d_n8, eq0_e1028_d_n9, eq0_e1028_d_n10, eq0_e1028_d_n11, eq0_e1028_d_n12, eq0_e1028_d_n13, eq0_e1028_d_n14, eq0_e1028_d_n15, eq0_e1028_d_n16, eq0_e1028_d_n17, eq0_e1028_d_b0, eq0_e1028_d_b1, eq0_e1028_d_b2, eq0_e1028_d_b3, eq0_e1028_d_b4, eq0_e1028_d_b5, eq0_e1028_d_b6, eq0_e1028_d_b7, eq0_e1028_d_b8, eq0_e1028_d_b9, eq0_e1028_d_b10, eq0_e1028_d_b11,) = {
    if s.b[3305] {
        let eq0_e1025: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, s.v[924]);let eq0_e1026: f64 = (s.v[926] + eq0_e1025);let eq0_e1026_d_n0: f64 = (s.dn[926][0] + (s.dn[924][0] * ddt_scale));let eq0_e1026_d_n1: f64 = (s.dn[926][1] + (s.dn[924][1] * ddt_scale));let eq0_e1026_d_n2: f64 = (s.dn[926][2] + (s.dn[924][2] * ddt_scale));let eq0_e1026_d_n3: f64 = (s.dn[926][3] + (s.dn[924][3] * ddt_scale));let eq0_e1026_d_n4: f64 = (s.dn[926][4] + (s.dn[924][4] * ddt_scale));let eq0_e1026_d_n5: f64 = (s.dn[926][5] + (s.dn[924][5] * ddt_scale));let eq0_e1026_d_n6: f64 = (s.dn[926][6] + (s.dn[924][6] * ddt_scale));let eq0_e1026_d_n7: f64 = (s.dn[926][7] + (s.dn[924][7] * ddt_scale));let eq0_e1026_d_n8: f64 = (s.dn[926][8] + (s.dn[924][8] * ddt_scale));let eq0_e1026_d_n9: f64 = (s.dn[926][9] + (s.dn[924][9] * ddt_scale));let eq0_e1026_d_n10: f64 = (s.dn[926][10] + (s.dn[924][10] * ddt_scale));let eq0_e1026_d_n11: f64 = (s.dn[926][11] + (s.dn[924][11] * ddt_scale));let eq0_e1026_d_n12: f64 = (s.dn[926][12] + (s.dn[924][12] * ddt_scale));let eq0_e1026_d_n13: f64 = (s.dn[926][13] + (s.dn[924][13] * ddt_scale));let eq0_e1026_d_n14: f64 = (s.dn[926][14] + (s.dn[924][14] * ddt_scale));let eq0_e1026_d_n15: f64 = (s.dn[926][15] + (s.dn[924][15] * ddt_scale));let eq0_e1026_d_n16: f64 = (s.dn[926][16] + (s.dn[924][16] * ddt_scale));let eq0_e1026_d_n17: f64 = (s.dn[926][17] + (s.dn[924][17] * ddt_scale));let eq0_e1026_d_b0: f64 = (s.db[926][0] + (s.db[924][0] * ddt_scale));let eq0_e1026_d_b1: f64 = (s.db[926][1] + (s.db[924][1] * ddt_scale));let eq0_e1026_d_b2: f64 = (s.db[926][2] + (s.db[924][2] * ddt_scale));let eq0_e1026_d_b3: f64 = (s.db[926][3] + (s.db[924][3] * ddt_scale));let eq0_e1026_d_b4: f64 = (s.db[926][4] + (s.db[924][4] * ddt_scale));let eq0_e1026_d_b5: f64 = (s.db[926][5] + (s.db[924][5] * ddt_scale));let eq0_e1026_d_b6: f64 = (s.db[926][6] + (s.db[924][6] * ddt_scale));let eq0_e1026_d_b7: f64 = (s.db[926][7] + (s.db[924][7] * ddt_scale));let eq0_e1026_d_b8: f64 = (s.db[926][8] + (s.db[924][8] * ddt_scale));let eq0_e1026_d_b9: f64 = (s.db[926][9] + (s.db[924][9] * ddt_scale));let eq0_e1026_d_b10: f64 = (s.db[926][10] + (s.db[924][10] * ddt_scale));let eq0_e1026_d_b11: f64 = (s.db[926][11] + (s.db[924][11] * ddt_scale));
        (eq0_e1026, eq0_e1026_d_n0, eq0_e1026_d_n1, eq0_e1026_d_n2, eq0_e1026_d_n3, eq0_e1026_d_n4, eq0_e1026_d_n5, eq0_e1026_d_n6, eq0_e1026_d_n7, eq0_e1026_d_n8, eq0_e1026_d_n9, eq0_e1026_d_n10, eq0_e1026_d_n11, eq0_e1026_d_n12, eq0_e1026_d_n13, eq0_e1026_d_n14, eq0_e1026_d_n15, eq0_e1026_d_n16, eq0_e1026_d_n17, eq0_e1026_d_b0, eq0_e1026_d_b1, eq0_e1026_d_b2, eq0_e1026_d_b3, eq0_e1026_d_b4, eq0_e1026_d_b5, eq0_e1026_d_b6, eq0_e1026_d_b7, eq0_e1026_d_b8, eq0_e1026_d_b9, eq0_e1026_d_b10, eq0_e1026_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq0_value: f64 = eq0_e1028;let eq0_node_derivatives: [f64; 18] = [eq0_e1028_d_n0, eq0_e1028_d_n1, eq0_e1028_d_n2, eq0_e1028_d_n3, eq0_e1028_d_n4, eq0_e1028_d_n5, eq0_e1028_d_n6, eq0_e1028_d_n7, eq0_e1028_d_n8, eq0_e1028_d_n9, eq0_e1028_d_n10, eq0_e1028_d_n11, eq0_e1028_d_n12, eq0_e1028_d_n13, eq0_e1028_d_n14, eq0_e1028_d_n15, eq0_e1028_d_n16, eq0_e1028_d_n17];let eq0_branch_derivatives: [f64; 12] = [eq0_e1028_d_b0, eq0_e1028_d_b1, eq0_e1028_d_b2, eq0_e1028_d_b3, eq0_e1028_d_b4, eq0_e1028_d_b5, eq0_e1028_d_b6, eq0_e1028_d_b7, eq0_e1028_d_b8, eq0_e1028_d_b9, eq0_e1028_d_b10, eq0_e1028_d_b11];
        stamper.stamp_current_dense_local(
            Some(15),
            None,
            multiplicity * (eq0_value),
            &eq0_node_derivatives,
            &eq0_branch_derivatives,
            multiplicity,
        );
        let (eq1_e1035, eq1_e1035_d_n0, eq1_e1035_d_n1, eq1_e1035_d_n2, eq1_e1035_d_n3, eq1_e1035_d_n4, eq1_e1035_d_n5, eq1_e1035_d_n6, eq1_e1035_d_n7, eq1_e1035_d_n8, eq1_e1035_d_n9, eq1_e1035_d_n10, eq1_e1035_d_n11, eq1_e1035_d_n12, eq1_e1035_d_n13, eq1_e1035_d_n14, eq1_e1035_d_n15, eq1_e1035_d_n16, eq1_e1035_d_n17, eq1_e1035_d_b0, eq1_e1035_d_b1, eq1_e1035_d_b2, eq1_e1035_d_b3, eq1_e1035_d_b4, eq1_e1035_d_b5, eq1_e1035_d_b6, eq1_e1035_d_b7, eq1_e1035_d_b8, eq1_e1035_d_b9, eq1_e1035_d_b10, eq1_e1035_d_b11,) = {
    if s.b[3305] {
        let eq1_e1032: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, s.v[925]);let eq1_e1033: f64 = (s.v[927] + eq1_e1032);let eq1_e1033_d_n0: f64 = (s.dn[927][0] + (s.dn[925][0] * ddt_scale));let eq1_e1033_d_n1: f64 = (s.dn[927][1] + (s.dn[925][1] * ddt_scale));let eq1_e1033_d_n2: f64 = (s.dn[927][2] + (s.dn[925][2] * ddt_scale));let eq1_e1033_d_n3: f64 = (s.dn[927][3] + (s.dn[925][3] * ddt_scale));let eq1_e1033_d_n4: f64 = (s.dn[927][4] + (s.dn[925][4] * ddt_scale));let eq1_e1033_d_n5: f64 = (s.dn[927][5] + (s.dn[925][5] * ddt_scale));let eq1_e1033_d_n6: f64 = (s.dn[927][6] + (s.dn[925][6] * ddt_scale));let eq1_e1033_d_n7: f64 = (s.dn[927][7] + (s.dn[925][7] * ddt_scale));let eq1_e1033_d_n8: f64 = (s.dn[927][8] + (s.dn[925][8] * ddt_scale));let eq1_e1033_d_n9: f64 = (s.dn[927][9] + (s.dn[925][9] * ddt_scale));let eq1_e1033_d_n10: f64 = (s.dn[927][10] + (s.dn[925][10] * ddt_scale));let eq1_e1033_d_n11: f64 = (s.dn[927][11] + (s.dn[925][11] * ddt_scale));let eq1_e1033_d_n12: f64 = (s.dn[927][12] + (s.dn[925][12] * ddt_scale));let eq1_e1033_d_n13: f64 = (s.dn[927][13] + (s.dn[925][13] * ddt_scale));let eq1_e1033_d_n14: f64 = (s.dn[927][14] + (s.dn[925][14] * ddt_scale));let eq1_e1033_d_n15: f64 = (s.dn[927][15] + (s.dn[925][15] * ddt_scale));let eq1_e1033_d_n16: f64 = (s.dn[927][16] + (s.dn[925][16] * ddt_scale));let eq1_e1033_d_n17: f64 = (s.dn[927][17] + (s.dn[925][17] * ddt_scale));let eq1_e1033_d_b0: f64 = (s.db[927][0] + (s.db[925][0] * ddt_scale));let eq1_e1033_d_b1: f64 = (s.db[927][1] + (s.db[925][1] * ddt_scale));let eq1_e1033_d_b2: f64 = (s.db[927][2] + (s.db[925][2] * ddt_scale));let eq1_e1033_d_b3: f64 = (s.db[927][3] + (s.db[925][3] * ddt_scale));let eq1_e1033_d_b4: f64 = (s.db[927][4] + (s.db[925][4] * ddt_scale));let eq1_e1033_d_b5: f64 = (s.db[927][5] + (s.db[925][5] * ddt_scale));let eq1_e1033_d_b6: f64 = (s.db[927][6] + (s.db[925][6] * ddt_scale));let eq1_e1033_d_b7: f64 = (s.db[927][7] + (s.db[925][7] * ddt_scale));let eq1_e1033_d_b8: f64 = (s.db[927][8] + (s.db[925][8] * ddt_scale));let eq1_e1033_d_b9: f64 = (s.db[927][9] + (s.db[925][9] * ddt_scale));let eq1_e1033_d_b10: f64 = (s.db[927][10] + (s.db[925][10] * ddt_scale));let eq1_e1033_d_b11: f64 = (s.db[927][11] + (s.db[925][11] * ddt_scale));
        (eq1_e1033, eq1_e1033_d_n0, eq1_e1033_d_n1, eq1_e1033_d_n2, eq1_e1033_d_n3, eq1_e1033_d_n4, eq1_e1033_d_n5, eq1_e1033_d_n6, eq1_e1033_d_n7, eq1_e1033_d_n8, eq1_e1033_d_n9, eq1_e1033_d_n10, eq1_e1033_d_n11, eq1_e1033_d_n12, eq1_e1033_d_n13, eq1_e1033_d_n14, eq1_e1033_d_n15, eq1_e1033_d_n16, eq1_e1033_d_n17, eq1_e1033_d_b0, eq1_e1033_d_b1, eq1_e1033_d_b2, eq1_e1033_d_b3, eq1_e1033_d_b4, eq1_e1033_d_b5, eq1_e1033_d_b6, eq1_e1033_d_b7, eq1_e1033_d_b8, eq1_e1033_d_b9, eq1_e1033_d_b10, eq1_e1033_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq1_value: f64 = eq1_e1035;let eq1_node_derivatives: [f64; 18] = [eq1_e1035_d_n0, eq1_e1035_d_n1, eq1_e1035_d_n2, eq1_e1035_d_n3, eq1_e1035_d_n4, eq1_e1035_d_n5, eq1_e1035_d_n6, eq1_e1035_d_n7, eq1_e1035_d_n8, eq1_e1035_d_n9, eq1_e1035_d_n10, eq1_e1035_d_n11, eq1_e1035_d_n12, eq1_e1035_d_n13, eq1_e1035_d_n14, eq1_e1035_d_n15, eq1_e1035_d_n16, eq1_e1035_d_n17];let eq1_branch_derivatives: [f64; 12] = [eq1_e1035_d_b0, eq1_e1035_d_b1, eq1_e1035_d_b2, eq1_e1035_d_b3, eq1_e1035_d_b4, eq1_e1035_d_b5, eq1_e1035_d_b6, eq1_e1035_d_b7, eq1_e1035_d_b8, eq1_e1035_d_b9, eq1_e1035_d_b10, eq1_e1035_d_b11];
        stamper.stamp_current_dense_local(
            Some(16),
            None,
            multiplicity * (eq1_value),
            &eq1_node_derivatives,
            &eq1_branch_derivatives,
            multiplicity,
        );
        let (eq2_e1040,) = {
    if (!s.b[3305]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq2_value: f64 = eq2_e1040;
        stamper.stamp_potential_const_local(
            0,
            eq2_value,
        );
        let (eq3_e1045,) = {
    if (!s.b[3305]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq3_value: f64 = eq3_e1045;
        stamper.stamp_potential_const_local(
            1,
            eq3_value,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_1(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_previous_value_scale: f64,
        ddt_older_value_scale: f64,
        ddt_previous_derivative_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_older: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
        ddt_derivative_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_derivative_previous: &mut [f64; Instance::DDT_STATE_COUNT],
    ) {
        let (eq4_e1052, eq4_e1052_d_n0, eq4_e1052_d_n1, eq4_e1052_d_n2, eq4_e1052_d_n3, eq4_e1052_d_n4, eq4_e1052_d_n5, eq4_e1052_d_n6, eq4_e1052_d_n7, eq4_e1052_d_n8, eq4_e1052_d_n9, eq4_e1052_d_n10, eq4_e1052_d_n11, eq4_e1052_d_n12, eq4_e1052_d_n13, eq4_e1052_d_n14, eq4_e1052_d_n15, eq4_e1052_d_n16, eq4_e1052_d_n17, eq4_e1052_d_b0, eq4_e1052_d_b1, eq4_e1052_d_b2, eq4_e1052_d_b3, eq4_e1052_d_b4, eq4_e1052_d_b5, eq4_e1052_d_b6, eq4_e1052_d_b7, eq4_e1052_d_b8, eq4_e1052_d_b9, eq4_e1052_d_b10, eq4_e1052_d_b11,) = {
    if s.b[3306] {
        let eq4_e1049: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, s.v[931]);let eq4_e1050: f64 = (s.v[932] + eq4_e1049);let eq4_e1050_d_n0: f64 = (s.dn[932][0] + (s.dn[931][0] * ddt_scale));let eq4_e1050_d_n1: f64 = (s.dn[932][1] + (s.dn[931][1] * ddt_scale));let eq4_e1050_d_n2: f64 = (s.dn[932][2] + (s.dn[931][2] * ddt_scale));let eq4_e1050_d_n3: f64 = (s.dn[932][3] + (s.dn[931][3] * ddt_scale));let eq4_e1050_d_n4: f64 = (s.dn[932][4] + (s.dn[931][4] * ddt_scale));let eq4_e1050_d_n5: f64 = (s.dn[932][5] + (s.dn[931][5] * ddt_scale));let eq4_e1050_d_n6: f64 = (s.dn[932][6] + (s.dn[931][6] * ddt_scale));let eq4_e1050_d_n7: f64 = (s.dn[932][7] + (s.dn[931][7] * ddt_scale));let eq4_e1050_d_n8: f64 = (s.dn[932][8] + (s.dn[931][8] * ddt_scale));let eq4_e1050_d_n9: f64 = (s.dn[932][9] + (s.dn[931][9] * ddt_scale));let eq4_e1050_d_n10: f64 = (s.dn[932][10] + (s.dn[931][10] * ddt_scale));let eq4_e1050_d_n11: f64 = (s.dn[932][11] + (s.dn[931][11] * ddt_scale));let eq4_e1050_d_n12: f64 = (s.dn[932][12] + (s.dn[931][12] * ddt_scale));let eq4_e1050_d_n13: f64 = (s.dn[932][13] + (s.dn[931][13] * ddt_scale));let eq4_e1050_d_n14: f64 = (s.dn[932][14] + (s.dn[931][14] * ddt_scale));let eq4_e1050_d_n15: f64 = (s.dn[932][15] + (s.dn[931][15] * ddt_scale));let eq4_e1050_d_n16: f64 = (s.dn[932][16] + (s.dn[931][16] * ddt_scale));let eq4_e1050_d_n17: f64 = (s.dn[932][17] + (s.dn[931][17] * ddt_scale));let eq4_e1050_d_b0: f64 = (s.db[932][0] + (s.db[931][0] * ddt_scale));let eq4_e1050_d_b1: f64 = (s.db[932][1] + (s.db[931][1] * ddt_scale));let eq4_e1050_d_b2: f64 = (s.db[932][2] + (s.db[931][2] * ddt_scale));let eq4_e1050_d_b3: f64 = (s.db[932][3] + (s.db[931][3] * ddt_scale));let eq4_e1050_d_b4: f64 = (s.db[932][4] + (s.db[931][4] * ddt_scale));let eq4_e1050_d_b5: f64 = (s.db[932][5] + (s.db[931][5] * ddt_scale));let eq4_e1050_d_b6: f64 = (s.db[932][6] + (s.db[931][6] * ddt_scale));let eq4_e1050_d_b7: f64 = (s.db[932][7] + (s.db[931][7] * ddt_scale));let eq4_e1050_d_b8: f64 = (s.db[932][8] + (s.db[931][8] * ddt_scale));let eq4_e1050_d_b9: f64 = (s.db[932][9] + (s.db[931][9] * ddt_scale));let eq4_e1050_d_b10: f64 = (s.db[932][10] + (s.db[931][10] * ddt_scale));let eq4_e1050_d_b11: f64 = (s.db[932][11] + (s.db[931][11] * ddt_scale));
        (eq4_e1050, eq4_e1050_d_n0, eq4_e1050_d_n1, eq4_e1050_d_n2, eq4_e1050_d_n3, eq4_e1050_d_n4, eq4_e1050_d_n5, eq4_e1050_d_n6, eq4_e1050_d_n7, eq4_e1050_d_n8, eq4_e1050_d_n9, eq4_e1050_d_n10, eq4_e1050_d_n11, eq4_e1050_d_n12, eq4_e1050_d_n13, eq4_e1050_d_n14, eq4_e1050_d_n15, eq4_e1050_d_n16, eq4_e1050_d_n17, eq4_e1050_d_b0, eq4_e1050_d_b1, eq4_e1050_d_b2, eq4_e1050_d_b3, eq4_e1050_d_b4, eq4_e1050_d_b5, eq4_e1050_d_b6, eq4_e1050_d_b7, eq4_e1050_d_b8, eq4_e1050_d_b9, eq4_e1050_d_b10, eq4_e1050_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq4_value: f64 = eq4_e1052;let eq4_node_derivatives: [f64; 18] = [eq4_e1052_d_n0, eq4_e1052_d_n1, eq4_e1052_d_n2, eq4_e1052_d_n3, eq4_e1052_d_n4, eq4_e1052_d_n5, eq4_e1052_d_n6, eq4_e1052_d_n7, eq4_e1052_d_n8, eq4_e1052_d_n9, eq4_e1052_d_n10, eq4_e1052_d_n11, eq4_e1052_d_n12, eq4_e1052_d_n13, eq4_e1052_d_n14, eq4_e1052_d_n15, eq4_e1052_d_n16, eq4_e1052_d_n17];let eq4_branch_derivatives: [f64; 12] = [eq4_e1052_d_b0, eq4_e1052_d_b1, eq4_e1052_d_b2, eq4_e1052_d_b3, eq4_e1052_d_b4, eq4_e1052_d_b5, eq4_e1052_d_b6, eq4_e1052_d_b7, eq4_e1052_d_b8, eq4_e1052_d_b9, eq4_e1052_d_b10, eq4_e1052_d_b11];
        stamper.stamp_current_dense_local(
            Some(17),
            None,
            multiplicity * (eq4_value),
            &eq4_node_derivatives,
            &eq4_branch_derivatives,
            multiplicity,
        );
        let (eq5_e1057,) = {
    if (!s.b[3306]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq5_value: f64 = eq5_e1057;
        stamper.stamp_potential_const_local(
            2,
            eq5_value,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_2(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        multiplicity: f64,
    ) {
        let eq6_e1061: f64 = (s.v[134] + s.v[400]);let eq6_e1061_d_n0: f64 = (s.dn[134][0] + s.dn[400][0]);let eq6_e1061_d_n1: f64 = (s.dn[134][1] + s.dn[400][1]);let eq6_e1061_d_n2: f64 = (s.dn[134][2] + s.dn[400][2]);let eq6_e1061_d_n3: f64 = (s.dn[134][3] + s.dn[400][3]);let eq6_e1061_d_n4: f64 = (s.dn[134][4] + s.dn[400][4]);let eq6_e1061_d_n5: f64 = (s.dn[134][5] + s.dn[400][5]);let eq6_e1061_d_n6: f64 = (s.dn[134][6] + s.dn[400][6]);let eq6_e1061_d_n7: f64 = (s.dn[134][7] + s.dn[400][7]);let eq6_e1061_d_n8: f64 = (s.dn[134][8] + s.dn[400][8]);let eq6_e1061_d_n9: f64 = (s.dn[134][9] + s.dn[400][9]);let eq6_e1061_d_n10: f64 = (s.dn[134][10] + s.dn[400][10]);let eq6_e1061_d_n11: f64 = (s.dn[134][11] + s.dn[400][11]);let eq6_e1061_d_n12: f64 = (s.dn[134][12] + s.dn[400][12]);let eq6_e1061_d_n13: f64 = (s.dn[134][13] + s.dn[400][13]);let eq6_e1061_d_n14: f64 = (s.dn[134][14] + s.dn[400][14]);let eq6_e1061_d_n15: f64 = (s.dn[134][15] + s.dn[400][15]);let eq6_e1061_d_n16: f64 = (s.dn[134][16] + s.dn[400][16]);let eq6_e1061_d_n17: f64 = (s.dn[134][17] + s.dn[400][17]);let eq6_e1061_d_b0: f64 = (s.db[134][0] + s.db[400][0]);let eq6_e1061_d_b1: f64 = (s.db[134][1] + s.db[400][1]);let eq6_e1061_d_b2: f64 = (s.db[134][2] + s.db[400][2]);let eq6_e1061_d_b3: f64 = (s.db[134][3] + s.db[400][3]);let eq6_e1061_d_b4: f64 = (s.db[134][4] + s.db[400][4]);let eq6_e1061_d_b5: f64 = (s.db[134][5] + s.db[400][5]);let eq6_e1061_d_b6: f64 = (s.db[134][6] + s.db[400][6]);let eq6_e1061_d_b7: f64 = (s.db[134][7] + s.db[400][7]);let eq6_e1061_d_b8: f64 = (s.db[134][8] + s.db[400][8]);let eq6_e1061_d_b9: f64 = (s.db[134][9] + s.db[400][9]);let eq6_e1061_d_b10: f64 = (s.db[134][10] + s.db[400][10]);let eq6_e1061_d_b11: f64 = (s.db[134][11] + s.db[400][11]);let eq6_e1063: f64 = (eq6_e1061 - s.v[738]);let eq6_e1063_d_n0: f64 = (eq6_e1061_d_n0 - s.dn[738][0]);let eq6_e1063_d_n1: f64 = (eq6_e1061_d_n1 - s.dn[738][1]);let eq6_e1063_d_n2: f64 = (eq6_e1061_d_n2 - s.dn[738][2]);let eq6_e1063_d_n3: f64 = (eq6_e1061_d_n3 - s.dn[738][3]);let eq6_e1063_d_n4: f64 = (eq6_e1061_d_n4 - s.dn[738][4]);let eq6_e1063_d_n5: f64 = (eq6_e1061_d_n5 - s.dn[738][5]);let eq6_e1063_d_n6: f64 = (eq6_e1061_d_n6 - s.dn[738][6]);let eq6_e1063_d_n7: f64 = (eq6_e1061_d_n7 - s.dn[738][7]);let eq6_e1063_d_n8: f64 = (eq6_e1061_d_n8 - s.dn[738][8]);let eq6_e1063_d_n9: f64 = (eq6_e1061_d_n9 - s.dn[738][9]);let eq6_e1063_d_n10: f64 = (eq6_e1061_d_n10 - s.dn[738][10]);let eq6_e1063_d_n11: f64 = (eq6_e1061_d_n11 - s.dn[738][11]);let eq6_e1063_d_n12: f64 = (eq6_e1061_d_n12 - s.dn[738][12]);let eq6_e1063_d_n13: f64 = (eq6_e1061_d_n13 - s.dn[738][13]);let eq6_e1063_d_n14: f64 = (eq6_e1061_d_n14 - s.dn[738][14]);let eq6_e1063_d_n15: f64 = (eq6_e1061_d_n15 - s.dn[738][15]);let eq6_e1063_d_n16: f64 = (eq6_e1061_d_n16 - s.dn[738][16]);let eq6_e1063_d_n17: f64 = (eq6_e1061_d_n17 - s.dn[738][17]);let eq6_e1063_d_b0: f64 = (eq6_e1061_d_b0 - s.db[738][0]);let eq6_e1063_d_b1: f64 = (eq6_e1061_d_b1 - s.db[738][1]);let eq6_e1063_d_b2: f64 = (eq6_e1061_d_b2 - s.db[738][2]);let eq6_e1063_d_b3: f64 = (eq6_e1061_d_b3 - s.db[738][3]);let eq6_e1063_d_b4: f64 = (eq6_e1061_d_b4 - s.db[738][4]);let eq6_e1063_d_b5: f64 = (eq6_e1061_d_b5 - s.db[738][5]);let eq6_e1063_d_b6: f64 = (eq6_e1061_d_b6 - s.db[738][6]);let eq6_e1063_d_b7: f64 = (eq6_e1061_d_b7 - s.db[738][7]);let eq6_e1063_d_b8: f64 = (eq6_e1061_d_b8 - s.db[738][8]);let eq6_e1063_d_b9: f64 = (eq6_e1061_d_b9 - s.db[738][9]);let eq6_e1063_d_b10: f64 = (eq6_e1061_d_b10 - s.db[738][10]);let eq6_e1063_d_b11: f64 = (eq6_e1061_d_b11 - s.db[738][11]);let eq6_e1064: f64 = (p.p87 * eq6_e1063);let eq6_e1064_d_n0: f64 = (p.p87 * eq6_e1063_d_n0);let eq6_e1064_d_n1: f64 = (p.p87 * eq6_e1063_d_n1);let eq6_e1064_d_n2: f64 = (p.p87 * eq6_e1063_d_n2);let eq6_e1064_d_n3: f64 = (p.p87 * eq6_e1063_d_n3);let eq6_e1064_d_n4: f64 = (p.p87 * eq6_e1063_d_n4);let eq6_e1064_d_n5: f64 = (p.p87 * eq6_e1063_d_n5);let eq6_e1064_d_n6: f64 = (p.p87 * eq6_e1063_d_n6);let eq6_e1064_d_n7: f64 = (p.p87 * eq6_e1063_d_n7);let eq6_e1064_d_n8: f64 = (p.p87 * eq6_e1063_d_n8);
        let eq6_e1064_d_n9: f64 = (p.p87 * eq6_e1063_d_n9);let eq6_e1064_d_n10: f64 = (p.p87 * eq6_e1063_d_n10);let eq6_e1064_d_n11: f64 = (p.p87 * eq6_e1063_d_n11);let eq6_e1064_d_n12: f64 = (p.p87 * eq6_e1063_d_n12);let eq6_e1064_d_n13: f64 = (p.p87 * eq6_e1063_d_n13);let eq6_e1064_d_n14: f64 = (p.p87 * eq6_e1063_d_n14);let eq6_e1064_d_n15: f64 = (p.p87 * eq6_e1063_d_n15);let eq6_e1064_d_n16: f64 = (p.p87 * eq6_e1063_d_n16);let eq6_e1064_d_n17: f64 = (p.p87 * eq6_e1063_d_n17);let eq6_e1064_d_b0: f64 = (p.p87 * eq6_e1063_d_b0);let eq6_e1064_d_b1: f64 = (p.p87 * eq6_e1063_d_b1);let eq6_e1064_d_b2: f64 = (p.p87 * eq6_e1063_d_b2);let eq6_e1064_d_b3: f64 = (p.p87 * eq6_e1063_d_b3);let eq6_e1064_d_b4: f64 = (p.p87 * eq6_e1063_d_b4);let eq6_e1064_d_b5: f64 = (p.p87 * eq6_e1063_d_b5);let eq6_e1064_d_b6: f64 = (p.p87 * eq6_e1063_d_b6);let eq6_e1064_d_b7: f64 = (p.p87 * eq6_e1063_d_b7);let eq6_e1064_d_b8: f64 = (p.p87 * eq6_e1063_d_b8);let eq6_e1064_d_b9: f64 = (p.p87 * eq6_e1063_d_b9);let eq6_e1064_d_b10: f64 = (p.p87 * eq6_e1063_d_b10);let eq6_e1064_d_b11: f64 = (p.p87 * eq6_e1063_d_b11);let eq6_value: f64 = eq6_e1064;let eq6_node_derivatives: [f64; 18] = [eq6_e1064_d_n0, eq6_e1064_d_n1, eq6_e1064_d_n2, eq6_e1064_d_n3, eq6_e1064_d_n4, eq6_e1064_d_n5, eq6_e1064_d_n6, eq6_e1064_d_n7, eq6_e1064_d_n8, eq6_e1064_d_n9, eq6_e1064_d_n10, eq6_e1064_d_n11, eq6_e1064_d_n12, eq6_e1064_d_n13, eq6_e1064_d_n14, eq6_e1064_d_n15, eq6_e1064_d_n16, eq6_e1064_d_n17];let eq6_branch_derivatives: [f64; 12] = [eq6_e1064_d_b0, eq6_e1064_d_b1, eq6_e1064_d_b2, eq6_e1064_d_b3, eq6_e1064_d_b4, eq6_e1064_d_b5, eq6_e1064_d_b6, eq6_e1064_d_b7, eq6_e1064_d_b8, eq6_e1064_d_b9, eq6_e1064_d_b10, eq6_e1064_d_b11];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(7),
            multiplicity * (eq6_value),
            &eq6_node_derivatives,
            &eq6_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_3(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        multiplicity: f64,
    ) {
        let eq7_e1068: f64 = (s.v[424] - s.v[425]);let eq7_e1068_d_n0: f64 = (s.dn[424][0] - s.dn[425][0]);let eq7_e1068_d_n1: f64 = (s.dn[424][1] - s.dn[425][1]);let eq7_e1068_d_n2: f64 = (s.dn[424][2] - s.dn[425][2]);let eq7_e1068_d_n3: f64 = (s.dn[424][3] - s.dn[425][3]);let eq7_e1068_d_n4: f64 = (s.dn[424][4] - s.dn[425][4]);let eq7_e1068_d_n5: f64 = (s.dn[424][5] - s.dn[425][5]);let eq7_e1068_d_n6: f64 = (s.dn[424][6] - s.dn[425][6]);let eq7_e1068_d_n7: f64 = (s.dn[424][7] - s.dn[425][7]);let eq7_e1068_d_n8: f64 = (s.dn[424][8] - s.dn[425][8]);let eq7_e1068_d_n9: f64 = (s.dn[424][9] - s.dn[425][9]);let eq7_e1068_d_n10: f64 = (s.dn[424][10] - s.dn[425][10]);let eq7_e1068_d_n11: f64 = (s.dn[424][11] - s.dn[425][11]);let eq7_e1068_d_n12: f64 = (s.dn[424][12] - s.dn[425][12]);let eq7_e1068_d_n13: f64 = (s.dn[424][13] - s.dn[425][13]);let eq7_e1068_d_n14: f64 = (s.dn[424][14] - s.dn[425][14]);let eq7_e1068_d_n15: f64 = (s.dn[424][15] - s.dn[425][15]);let eq7_e1068_d_n16: f64 = (s.dn[424][16] - s.dn[425][16]);let eq7_e1068_d_n17: f64 = (s.dn[424][17] - s.dn[425][17]);let eq7_e1068_d_b0: f64 = (s.db[424][0] - s.db[425][0]);let eq7_e1068_d_b1: f64 = (s.db[424][1] - s.db[425][1]);let eq7_e1068_d_b2: f64 = (s.db[424][2] - s.db[425][2]);let eq7_e1068_d_b3: f64 = (s.db[424][3] - s.db[425][3]);let eq7_e1068_d_b4: f64 = (s.db[424][4] - s.db[425][4]);let eq7_e1068_d_b5: f64 = (s.db[424][5] - s.db[425][5]);let eq7_e1068_d_b6: f64 = (s.db[424][6] - s.db[425][6]);let eq7_e1068_d_b7: f64 = (s.db[424][7] - s.db[425][7]);let eq7_e1068_d_b8: f64 = (s.db[424][8] - s.db[425][8]);let eq7_e1068_d_b9: f64 = (s.db[424][9] - s.db[425][9]);let eq7_e1068_d_b10: f64 = (s.db[424][10] - s.db[425][10]);let eq7_e1068_d_b11: f64 = (s.db[424][11] - s.db[425][11]);let eq7_e1069: f64 = (p.p87 * eq7_e1068);let eq7_e1069_d_n0: f64 = (p.p87 * eq7_e1068_d_n0);let eq7_e1069_d_n1: f64 = (p.p87 * eq7_e1068_d_n1);let eq7_e1069_d_n2: f64 = (p.p87 * eq7_e1068_d_n2);let eq7_e1069_d_n3: f64 = (p.p87 * eq7_e1068_d_n3);let eq7_e1069_d_n4: f64 = (p.p87 * eq7_e1068_d_n4);let eq7_e1069_d_n5: f64 = (p.p87 * eq7_e1068_d_n5);let eq7_e1069_d_n6: f64 = (p.p87 * eq7_e1068_d_n6);let eq7_e1069_d_n7: f64 = (p.p87 * eq7_e1068_d_n7);let eq7_e1069_d_n8: f64 = (p.p87 * eq7_e1068_d_n8);let eq7_e1069_d_n9: f64 = (p.p87 * eq7_e1068_d_n9);let eq7_e1069_d_n10: f64 = (p.p87 * eq7_e1068_d_n10);let eq7_e1069_d_n11: f64 = (p.p87 * eq7_e1068_d_n11);let eq7_e1069_d_n12: f64 = (p.p87 * eq7_e1068_d_n12);let eq7_e1069_d_n13: f64 = (p.p87 * eq7_e1068_d_n13);let eq7_e1069_d_n14: f64 = (p.p87 * eq7_e1068_d_n14);let eq7_e1069_d_n15: f64 = (p.p87 * eq7_e1068_d_n15);let eq7_e1069_d_n16: f64 = (p.p87 * eq7_e1068_d_n16);let eq7_e1069_d_n17: f64 = (p.p87 * eq7_e1068_d_n17);let eq7_e1069_d_b0: f64 = (p.p87 * eq7_e1068_d_b0);let eq7_e1069_d_b1: f64 = (p.p87 * eq7_e1068_d_b1);let eq7_e1069_d_b2: f64 = (p.p87 * eq7_e1068_d_b2);let eq7_e1069_d_b3: f64 = (p.p87 * eq7_e1068_d_b3);let eq7_e1069_d_b4: f64 = (p.p87 * eq7_e1068_d_b4);let eq7_e1069_d_b5: f64 = (p.p87 * eq7_e1068_d_b5);let eq7_e1069_d_b6: f64 = (p.p87 * eq7_e1068_d_b6);let eq7_e1069_d_b7: f64 = (p.p87 * eq7_e1068_d_b7);let eq7_e1069_d_b8: f64 = (p.p87 * eq7_e1068_d_b8);let eq7_e1069_d_b9: f64 = (p.p87 * eq7_e1068_d_b9);let eq7_e1069_d_b10: f64 = (p.p87 * eq7_e1068_d_b10);let eq7_e1069_d_b11: f64 = (p.p87 * eq7_e1068_d_b11);let eq7_value: f64 = eq7_e1069;let eq7_node_derivatives: [f64; 18] = [eq7_e1069_d_n0, eq7_e1069_d_n1, eq7_e1069_d_n2, eq7_e1069_d_n3, eq7_e1069_d_n4, eq7_e1069_d_n5, eq7_e1069_d_n6, eq7_e1069_d_n7, eq7_e1069_d_n8, eq7_e1069_d_n9, eq7_e1069_d_n10, eq7_e1069_d_n11, eq7_e1069_d_n12, eq7_e1069_d_n13, eq7_e1069_d_n14, eq7_e1069_d_n15, eq7_e1069_d_n16, eq7_e1069_d_n17];let eq7_branch_derivatives: [f64; 12] = [eq7_e1069_d_b0, eq7_e1069_d_b1, eq7_e1069_d_b2, eq7_e1069_d_b3, eq7_e1069_d_b4, eq7_e1069_d_b5, eq7_e1069_d_b6, eq7_e1069_d_b7, eq7_e1069_d_b8, eq7_e1069_d_b9, eq7_e1069_d_b10, eq7_e1069_d_b11];
        stamper.stamp_current_dense_local(
            Some(0),
            Some(2),
            multiplicity * (eq7_value),
            &eq7_node_derivatives,
            &eq7_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_4(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        multiplicity: f64,
    ) {
        let eq8_e1073: f64 = (s.v[203] + s.v[280]);let eq8_e1073_d_n0: f64 = (s.dn[203][0] + s.dn[280][0]);let eq8_e1073_d_n1: f64 = (s.dn[203][1] + s.dn[280][1]);let eq8_e1073_d_n2: f64 = (s.dn[203][2] + s.dn[280][2]);let eq8_e1073_d_n3: f64 = (s.dn[203][3] + s.dn[280][3]);let eq8_e1073_d_n4: f64 = (s.dn[203][4] + s.dn[280][4]);let eq8_e1073_d_n5: f64 = (s.dn[203][5] + s.dn[280][5]);let eq8_e1073_d_n6: f64 = (s.dn[203][6] + s.dn[280][6]);let eq8_e1073_d_n7: f64 = (s.dn[203][7] + s.dn[280][7]);let eq8_e1073_d_n8: f64 = (s.dn[203][8] + s.dn[280][8]);let eq8_e1073_d_n9: f64 = (s.dn[203][9] + s.dn[280][9]);let eq8_e1073_d_n10: f64 = (s.dn[203][10] + s.dn[280][10]);let eq8_e1073_d_n11: f64 = (s.dn[203][11] + s.dn[280][11]);let eq8_e1073_d_n12: f64 = (s.dn[203][12] + s.dn[280][12]);let eq8_e1073_d_n13: f64 = (s.dn[203][13] + s.dn[280][13]);let eq8_e1073_d_n14: f64 = (s.dn[203][14] + s.dn[280][14]);let eq8_e1073_d_n15: f64 = (s.dn[203][15] + s.dn[280][15]);let eq8_e1073_d_n16: f64 = (s.dn[203][16] + s.dn[280][16]);let eq8_e1073_d_n17: f64 = (s.dn[203][17] + s.dn[280][17]);let eq8_e1073_d_b0: f64 = (s.db[203][0] + s.db[280][0]);let eq8_e1073_d_b1: f64 = (s.db[203][1] + s.db[280][1]);let eq8_e1073_d_b2: f64 = (s.db[203][2] + s.db[280][2]);let eq8_e1073_d_b3: f64 = (s.db[203][3] + s.db[280][3]);let eq8_e1073_d_b4: f64 = (s.db[203][4] + s.db[280][4]);let eq8_e1073_d_b5: f64 = (s.db[203][5] + s.db[280][5]);let eq8_e1073_d_b6: f64 = (s.db[203][6] + s.db[280][6]);let eq8_e1073_d_b7: f64 = (s.db[203][7] + s.db[280][7]);let eq8_e1073_d_b8: f64 = (s.db[203][8] + s.db[280][8]);let eq8_e1073_d_b9: f64 = (s.db[203][9] + s.db[280][9]);let eq8_e1073_d_b10: f64 = (s.db[203][10] + s.db[280][10]);let eq8_e1073_d_b11: f64 = (s.db[203][11] + s.db[280][11]);let eq8_e1075: f64 = (eq8_e1073 + s.v[431]);let eq8_e1075_d_n0: f64 = (eq8_e1073_d_n0 + s.dn[431][0]);let eq8_e1075_d_n1: f64 = (eq8_e1073_d_n1 + s.dn[431][1]);let eq8_e1075_d_n2: f64 = (eq8_e1073_d_n2 + s.dn[431][2]);let eq8_e1075_d_n3: f64 = (eq8_e1073_d_n3 + s.dn[431][3]);let eq8_e1075_d_n4: f64 = (eq8_e1073_d_n4 + s.dn[431][4]);let eq8_e1075_d_n5: f64 = (eq8_e1073_d_n5 + s.dn[431][5]);let eq8_e1075_d_n6: f64 = (eq8_e1073_d_n6 + s.dn[431][6]);let eq8_e1075_d_n7: f64 = (eq8_e1073_d_n7 + s.dn[431][7]);let eq8_e1075_d_n8: f64 = (eq8_e1073_d_n8 + s.dn[431][8]);let eq8_e1075_d_n9: f64 = (eq8_e1073_d_n9 + s.dn[431][9]);let eq8_e1075_d_n10: f64 = (eq8_e1073_d_n10 + s.dn[431][10]);let eq8_e1075_d_n11: f64 = (eq8_e1073_d_n11 + s.dn[431][11]);let eq8_e1075_d_n12: f64 = (eq8_e1073_d_n12 + s.dn[431][12]);let eq8_e1075_d_n13: f64 = (eq8_e1073_d_n13 + s.dn[431][13]);let eq8_e1075_d_n14: f64 = (eq8_e1073_d_n14 + s.dn[431][14]);let eq8_e1075_d_n15: f64 = (eq8_e1073_d_n15 + s.dn[431][15]);let eq8_e1075_d_n16: f64 = (eq8_e1073_d_n16 + s.dn[431][16]);let eq8_e1075_d_n17: f64 = (eq8_e1073_d_n17 + s.dn[431][17]);let eq8_e1075_d_b0: f64 = (eq8_e1073_d_b0 + s.db[431][0]);let eq8_e1075_d_b1: f64 = (eq8_e1073_d_b1 + s.db[431][1]);let eq8_e1075_d_b2: f64 = (eq8_e1073_d_b2 + s.db[431][2]);let eq8_e1075_d_b3: f64 = (eq8_e1073_d_b3 + s.db[431][3]);let eq8_e1075_d_b4: f64 = (eq8_e1073_d_b4 + s.db[431][4]);let eq8_e1075_d_b5: f64 = (eq8_e1073_d_b5 + s.db[431][5]);let eq8_e1075_d_b6: f64 = (eq8_e1073_d_b6 + s.db[431][6]);let eq8_e1075_d_b7: f64 = (eq8_e1073_d_b7 + s.db[431][7]);let eq8_e1075_d_b8: f64 = (eq8_e1073_d_b8 + s.db[431][8]);let eq8_e1075_d_b9: f64 = (eq8_e1073_d_b9 + s.db[431][9]);let eq8_e1075_d_b10: f64 = (eq8_e1073_d_b10 + s.db[431][10]);let eq8_e1075_d_b11: f64 = (eq8_e1073_d_b11 + s.db[431][11]);let eq8_e1076: f64 = (p.p87 * eq8_e1075);let eq8_e1076_d_n0: f64 = (p.p87 * eq8_e1075_d_n0);let eq8_e1076_d_n1: f64 = (p.p87 * eq8_e1075_d_n1);let eq8_e1076_d_n2: f64 = (p.p87 * eq8_e1075_d_n2);let eq8_e1076_d_n3: f64 = (p.p87 * eq8_e1075_d_n3);let eq8_e1076_d_n4: f64 = (p.p87 * eq8_e1075_d_n4);let eq8_e1076_d_n5: f64 = (p.p87 * eq8_e1075_d_n5);let eq8_e1076_d_n6: f64 = (p.p87 * eq8_e1075_d_n6);let eq8_e1076_d_n7: f64 = (p.p87 * eq8_e1075_d_n7);let eq8_e1076_d_n8: f64 = (p.p87 * eq8_e1075_d_n8);
        let eq8_e1076_d_n9: f64 = (p.p87 * eq8_e1075_d_n9);let eq8_e1076_d_n10: f64 = (p.p87 * eq8_e1075_d_n10);let eq8_e1076_d_n11: f64 = (p.p87 * eq8_e1075_d_n11);let eq8_e1076_d_n12: f64 = (p.p87 * eq8_e1075_d_n12);let eq8_e1076_d_n13: f64 = (p.p87 * eq8_e1075_d_n13);let eq8_e1076_d_n14: f64 = (p.p87 * eq8_e1075_d_n14);let eq8_e1076_d_n15: f64 = (p.p87 * eq8_e1075_d_n15);let eq8_e1076_d_n16: f64 = (p.p87 * eq8_e1075_d_n16);let eq8_e1076_d_n17: f64 = (p.p87 * eq8_e1075_d_n17);let eq8_e1076_d_b0: f64 = (p.p87 * eq8_e1075_d_b0);let eq8_e1076_d_b1: f64 = (p.p87 * eq8_e1075_d_b1);let eq8_e1076_d_b2: f64 = (p.p87 * eq8_e1075_d_b2);let eq8_e1076_d_b3: f64 = (p.p87 * eq8_e1075_d_b3);let eq8_e1076_d_b4: f64 = (p.p87 * eq8_e1075_d_b4);let eq8_e1076_d_b5: f64 = (p.p87 * eq8_e1075_d_b5);let eq8_e1076_d_b6: f64 = (p.p87 * eq8_e1075_d_b6);let eq8_e1076_d_b7: f64 = (p.p87 * eq8_e1075_d_b7);let eq8_e1076_d_b8: f64 = (p.p87 * eq8_e1075_d_b8);let eq8_e1076_d_b9: f64 = (p.p87 * eq8_e1075_d_b9);let eq8_e1076_d_b10: f64 = (p.p87 * eq8_e1075_d_b10);let eq8_e1076_d_b11: f64 = (p.p87 * eq8_e1075_d_b11);let eq8_value: f64 = eq8_e1076;let eq8_node_derivatives: [f64; 18] = [eq8_e1076_d_n0, eq8_e1076_d_n1, eq8_e1076_d_n2, eq8_e1076_d_n3, eq8_e1076_d_n4, eq8_e1076_d_n5, eq8_e1076_d_n6, eq8_e1076_d_n7, eq8_e1076_d_n8, eq8_e1076_d_n9, eq8_e1076_d_n10, eq8_e1076_d_n11, eq8_e1076_d_n12, eq8_e1076_d_n13, eq8_e1076_d_n14, eq8_e1076_d_n15, eq8_e1076_d_n16, eq8_e1076_d_n17];let eq8_branch_derivatives: [f64; 12] = [eq8_e1076_d_b0, eq8_e1076_d_b1, eq8_e1076_d_b2, eq8_e1076_d_b3, eq8_e1076_d_b4, eq8_e1076_d_b5, eq8_e1076_d_b6, eq8_e1076_d_b7, eq8_e1076_d_b8, eq8_e1076_d_b9, eq8_e1076_d_b10, eq8_e1076_d_b11];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(8),
            multiplicity * (eq8_value),
            &eq8_node_derivatives,
            &eq8_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_5(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        multiplicity: f64,
    ) {
        let eq9_e1080: f64 = (s.v[204] + s.v[736]);let eq9_e1080_d_n0: f64 = (s.dn[204][0] + s.dn[736][0]);let eq9_e1080_d_n1: f64 = (s.dn[204][1] + s.dn[736][1]);let eq9_e1080_d_n2: f64 = (s.dn[204][2] + s.dn[736][2]);let eq9_e1080_d_n3: f64 = (s.dn[204][3] + s.dn[736][3]);let eq9_e1080_d_n4: f64 = (s.dn[204][4] + s.dn[736][4]);let eq9_e1080_d_n5: f64 = (s.dn[204][5] + s.dn[736][5]);let eq9_e1080_d_n6: f64 = (s.dn[204][6] + s.dn[736][6]);let eq9_e1080_d_n7: f64 = (s.dn[204][7] + s.dn[736][7]);let eq9_e1080_d_n8: f64 = (s.dn[204][8] + s.dn[736][8]);let eq9_e1080_d_n9: f64 = (s.dn[204][9] + s.dn[736][9]);let eq9_e1080_d_n10: f64 = (s.dn[204][10] + s.dn[736][10]);let eq9_e1080_d_n11: f64 = (s.dn[204][11] + s.dn[736][11]);let eq9_e1080_d_n12: f64 = (s.dn[204][12] + s.dn[736][12]);let eq9_e1080_d_n13: f64 = (s.dn[204][13] + s.dn[736][13]);let eq9_e1080_d_n14: f64 = (s.dn[204][14] + s.dn[736][14]);let eq9_e1080_d_n15: f64 = (s.dn[204][15] + s.dn[736][15]);let eq9_e1080_d_n16: f64 = (s.dn[204][16] + s.dn[736][16]);let eq9_e1080_d_n17: f64 = (s.dn[204][17] + s.dn[736][17]);let eq9_e1080_d_b0: f64 = (s.db[204][0] + s.db[736][0]);let eq9_e1080_d_b1: f64 = (s.db[204][1] + s.db[736][1]);let eq9_e1080_d_b2: f64 = (s.db[204][2] + s.db[736][2]);let eq9_e1080_d_b3: f64 = (s.db[204][3] + s.db[736][3]);let eq9_e1080_d_b4: f64 = (s.db[204][4] + s.db[736][4]);let eq9_e1080_d_b5: f64 = (s.db[204][5] + s.db[736][5]);let eq9_e1080_d_b6: f64 = (s.db[204][6] + s.db[736][6]);let eq9_e1080_d_b7: f64 = (s.db[204][7] + s.db[736][7]);let eq9_e1080_d_b8: f64 = (s.db[204][8] + s.db[736][8]);let eq9_e1080_d_b9: f64 = (s.db[204][9] + s.db[736][9]);let eq9_e1080_d_b10: f64 = (s.db[204][10] + s.db[736][10]);let eq9_e1080_d_b11: f64 = (s.db[204][11] + s.db[736][11]);let eq9_e1082: f64 = (eq9_e1080 + s.v[432]);let eq9_e1082_d_n0: f64 = (eq9_e1080_d_n0 + s.dn[432][0]);let eq9_e1082_d_n1: f64 = (eq9_e1080_d_n1 + s.dn[432][1]);let eq9_e1082_d_n2: f64 = (eq9_e1080_d_n2 + s.dn[432][2]);let eq9_e1082_d_n3: f64 = (eq9_e1080_d_n3 + s.dn[432][3]);let eq9_e1082_d_n4: f64 = (eq9_e1080_d_n4 + s.dn[432][4]);let eq9_e1082_d_n5: f64 = (eq9_e1080_d_n5 + s.dn[432][5]);let eq9_e1082_d_n6: f64 = (eq9_e1080_d_n6 + s.dn[432][6]);let eq9_e1082_d_n7: f64 = (eq9_e1080_d_n7 + s.dn[432][7]);let eq9_e1082_d_n8: f64 = (eq9_e1080_d_n8 + s.dn[432][8]);let eq9_e1082_d_n9: f64 = (eq9_e1080_d_n9 + s.dn[432][9]);let eq9_e1082_d_n10: f64 = (eq9_e1080_d_n10 + s.dn[432][10]);let eq9_e1082_d_n11: f64 = (eq9_e1080_d_n11 + s.dn[432][11]);let eq9_e1082_d_n12: f64 = (eq9_e1080_d_n12 + s.dn[432][12]);let eq9_e1082_d_n13: f64 = (eq9_e1080_d_n13 + s.dn[432][13]);let eq9_e1082_d_n14: f64 = (eq9_e1080_d_n14 + s.dn[432][14]);let eq9_e1082_d_n15: f64 = (eq9_e1080_d_n15 + s.dn[432][15]);let eq9_e1082_d_n16: f64 = (eq9_e1080_d_n16 + s.dn[432][16]);let eq9_e1082_d_n17: f64 = (eq9_e1080_d_n17 + s.dn[432][17]);let eq9_e1082_d_b0: f64 = (eq9_e1080_d_b0 + s.db[432][0]);let eq9_e1082_d_b1: f64 = (eq9_e1080_d_b1 + s.db[432][1]);let eq9_e1082_d_b2: f64 = (eq9_e1080_d_b2 + s.db[432][2]);let eq9_e1082_d_b3: f64 = (eq9_e1080_d_b3 + s.db[432][3]);let eq9_e1082_d_b4: f64 = (eq9_e1080_d_b4 + s.db[432][4]);let eq9_e1082_d_b5: f64 = (eq9_e1080_d_b5 + s.db[432][5]);let eq9_e1082_d_b6: f64 = (eq9_e1080_d_b6 + s.db[432][6]);let eq9_e1082_d_b7: f64 = (eq9_e1080_d_b7 + s.db[432][7]);let eq9_e1082_d_b8: f64 = (eq9_e1080_d_b8 + s.db[432][8]);let eq9_e1082_d_b9: f64 = (eq9_e1080_d_b9 + s.db[432][9]);let eq9_e1082_d_b10: f64 = (eq9_e1080_d_b10 + s.db[432][10]);let eq9_e1082_d_b11: f64 = (eq9_e1080_d_b11 + s.db[432][11]);let eq9_e1083: f64 = (p.p87 * eq9_e1082);let eq9_e1083_d_n0: f64 = (p.p87 * eq9_e1082_d_n0);let eq9_e1083_d_n1: f64 = (p.p87 * eq9_e1082_d_n1);let eq9_e1083_d_n2: f64 = (p.p87 * eq9_e1082_d_n2);let eq9_e1083_d_n3: f64 = (p.p87 * eq9_e1082_d_n3);let eq9_e1083_d_n4: f64 = (p.p87 * eq9_e1082_d_n4);let eq9_e1083_d_n5: f64 = (p.p87 * eq9_e1082_d_n5);let eq9_e1083_d_n6: f64 = (p.p87 * eq9_e1082_d_n6);let eq9_e1083_d_n7: f64 = (p.p87 * eq9_e1082_d_n7);let eq9_e1083_d_n8: f64 = (p.p87 * eq9_e1082_d_n8);
        let eq9_e1083_d_n9: f64 = (p.p87 * eq9_e1082_d_n9);let eq9_e1083_d_n10: f64 = (p.p87 * eq9_e1082_d_n10);let eq9_e1083_d_n11: f64 = (p.p87 * eq9_e1082_d_n11);let eq9_e1083_d_n12: f64 = (p.p87 * eq9_e1082_d_n12);let eq9_e1083_d_n13: f64 = (p.p87 * eq9_e1082_d_n13);let eq9_e1083_d_n14: f64 = (p.p87 * eq9_e1082_d_n14);let eq9_e1083_d_n15: f64 = (p.p87 * eq9_e1082_d_n15);let eq9_e1083_d_n16: f64 = (p.p87 * eq9_e1082_d_n16);let eq9_e1083_d_n17: f64 = (p.p87 * eq9_e1082_d_n17);let eq9_e1083_d_b0: f64 = (p.p87 * eq9_e1082_d_b0);let eq9_e1083_d_b1: f64 = (p.p87 * eq9_e1082_d_b1);let eq9_e1083_d_b2: f64 = (p.p87 * eq9_e1082_d_b2);let eq9_e1083_d_b3: f64 = (p.p87 * eq9_e1082_d_b3);let eq9_e1083_d_b4: f64 = (p.p87 * eq9_e1082_d_b4);let eq9_e1083_d_b5: f64 = (p.p87 * eq9_e1082_d_b5);let eq9_e1083_d_b6: f64 = (p.p87 * eq9_e1082_d_b6);let eq9_e1083_d_b7: f64 = (p.p87 * eq9_e1082_d_b7);let eq9_e1083_d_b8: f64 = (p.p87 * eq9_e1082_d_b8);let eq9_e1083_d_b9: f64 = (p.p87 * eq9_e1082_d_b9);let eq9_e1083_d_b10: f64 = (p.p87 * eq9_e1082_d_b10);let eq9_e1083_d_b11: f64 = (p.p87 * eq9_e1082_d_b11);let eq9_value: f64 = eq9_e1083;let eq9_node_derivatives: [f64; 18] = [eq9_e1083_d_n0, eq9_e1083_d_n1, eq9_e1083_d_n2, eq9_e1083_d_n3, eq9_e1083_d_n4, eq9_e1083_d_n5, eq9_e1083_d_n6, eq9_e1083_d_n7, eq9_e1083_d_n8, eq9_e1083_d_n9, eq9_e1083_d_n10, eq9_e1083_d_n11, eq9_e1083_d_n12, eq9_e1083_d_n13, eq9_e1083_d_n14, eq9_e1083_d_n15, eq9_e1083_d_n16, eq9_e1083_d_n17];let eq9_branch_derivatives: [f64; 12] = [eq9_e1083_d_b0, eq9_e1083_d_b1, eq9_e1083_d_b2, eq9_e1083_d_b3, eq9_e1083_d_b4, eq9_e1083_d_b5, eq9_e1083_d_b6, eq9_e1083_d_b7, eq9_e1083_d_b8, eq9_e1083_d_b9, eq9_e1083_d_b10, eq9_e1083_d_b11];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(8),
            multiplicity * (eq9_value),
            &eq9_node_derivatives,
            &eq9_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_6(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_previous_value_scale: f64,
        ddt_older_value_scale: f64,
        ddt_previous_derivative_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_older: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
        ddt_derivative_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_derivative_previous: &mut [f64; Instance::DDT_STATE_COUNT],
    ) {
        let eq10_e1086: f64 = (p.p87 * s.v[281]);let eq10_value: f64 = eq10_e1086;
        stamper.stamp_current_dense_local(
            Some(0),
            Some(8),
            multiplicity * (eq10_value),
            &s.dn[281],
            &s.db[281],
            (multiplicity) * (p.p87),
        );let eq11_e1089: f64 = (p.p87 * s.v[737]);let eq11_value: f64 = eq11_e1089;
        stamper.stamp_current_dense_local(
            Some(2),
            Some(8),
            multiplicity * (eq11_value),
            &s.dn[737],
            &s.db[737],
            (multiplicity) * (p.p87),
        );let eq12_e1092: f64 = (p.p87 * s.v[862]);let eq12_value: f64 = eq12_e1092;
        stamper.stamp_current_dense_local(
            Some(10),
            Some(2),
            multiplicity * (eq12_value),
            &s.dn[862],
            &s.db[862],
            (multiplicity) * (p.p87),
        );let eq13_e1095: f64 = (p.p87 * s.v[861]);let eq13_value: f64 = eq13_e1095;
        stamper.stamp_current_dense_local(
            Some(9),
            Some(0),
            multiplicity * (eq13_value),
            &s.dn[861],
            &s.db[861],
            (multiplicity) * (p.p87),
        );let eq14_e1098: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, s.v[66]);let eq14_e1099: f64 = (p.p87 * eq14_e1098);let eq14_e1099_d_n0: f64 = (p.p87 * (s.dn[66][0] * ddt_scale));let eq14_e1099_d_n1: f64 = (p.p87 * (s.dn[66][1] * ddt_scale));let eq14_e1099_d_n2: f64 = (p.p87 * (s.dn[66][2] * ddt_scale));let eq14_e1099_d_n3: f64 = (p.p87 * (s.dn[66][3] * ddt_scale));let eq14_e1099_d_n4: f64 = (p.p87 * (s.dn[66][4] * ddt_scale));let eq14_e1099_d_n5: f64 = (p.p87 * (s.dn[66][5] * ddt_scale));let eq14_e1099_d_n6: f64 = (p.p87 * (s.dn[66][6] * ddt_scale));let eq14_e1099_d_n7: f64 = (p.p87 * (s.dn[66][7] * ddt_scale));let eq14_e1099_d_n8: f64 = (p.p87 * (s.dn[66][8] * ddt_scale));let eq14_e1099_d_n9: f64 = (p.p87 * (s.dn[66][9] * ddt_scale));let eq14_e1099_d_n10: f64 = (p.p87 * (s.dn[66][10] * ddt_scale));let eq14_e1099_d_n11: f64 = (p.p87 * (s.dn[66][11] * ddt_scale));let eq14_e1099_d_n12: f64 = (p.p87 * (s.dn[66][12] * ddt_scale));let eq14_e1099_d_n13: f64 = (p.p87 * (s.dn[66][13] * ddt_scale));let eq14_e1099_d_n14: f64 = (p.p87 * (s.dn[66][14] * ddt_scale));let eq14_e1099_d_n15: f64 = (p.p87 * (s.dn[66][15] * ddt_scale));let eq14_e1099_d_n16: f64 = (p.p87 * (s.dn[66][16] * ddt_scale));let eq14_e1099_d_n17: f64 = (p.p87 * (s.dn[66][17] * ddt_scale));let eq14_e1099_d_b0: f64 = (p.p87 * (s.db[66][0] * ddt_scale));let eq14_e1099_d_b1: f64 = (p.p87 * (s.db[66][1] * ddt_scale));let eq14_e1099_d_b2: f64 = (p.p87 * (s.db[66][2] * ddt_scale));let eq14_e1099_d_b3: f64 = (p.p87 * (s.db[66][3] * ddt_scale));let eq14_e1099_d_b4: f64 = (p.p87 * (s.db[66][4] * ddt_scale));let eq14_e1099_d_b5: f64 = (p.p87 * (s.db[66][5] * ddt_scale));let eq14_e1099_d_b6: f64 = (p.p87 * (s.db[66][6] * ddt_scale));let eq14_e1099_d_b7: f64 = (p.p87 * (s.db[66][7] * ddt_scale));let eq14_e1099_d_b8: f64 = (p.p87 * (s.db[66][8] * ddt_scale));let eq14_e1099_d_b9: f64 = (p.p87 * (s.db[66][9] * ddt_scale));let eq14_e1099_d_b10: f64 = (p.p87 * (s.db[66][10] * ddt_scale));let eq14_e1099_d_b11: f64 = (p.p87 * (s.db[66][11] * ddt_scale));let eq14_value: f64 = eq14_e1099;let eq14_node_derivatives: [f64; 18] = [eq14_e1099_d_n0, eq14_e1099_d_n1, eq14_e1099_d_n2, eq14_e1099_d_n3, eq14_e1099_d_n4, eq14_e1099_d_n5, eq14_e1099_d_n6, eq14_e1099_d_n7, eq14_e1099_d_n8, eq14_e1099_d_n9, eq14_e1099_d_n10, eq14_e1099_d_n11, eq14_e1099_d_n12, eq14_e1099_d_n13, eq14_e1099_d_n14, eq14_e1099_d_n15, eq14_e1099_d_n16, eq14_e1099_d_n17];let eq14_branch_derivatives: [f64; 12] = [eq14_e1099_d_b0, eq14_e1099_d_b1, eq14_e1099_d_b2, eq14_e1099_d_b3, eq14_e1099_d_b4, eq14_e1099_d_b5, eq14_e1099_d_b6, eq14_e1099_d_b7, eq14_e1099_d_b8, eq14_e1099_d_b9, eq14_e1099_d_b10, eq14_e1099_d_b11];
        stamper.stamp_current_dense_local(
            Some(10),
            Some(2),
            multiplicity * (eq14_value),
            &eq14_node_derivatives,
            &eq14_branch_derivatives,
            multiplicity,
        );let eq15_e1102: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, s.v[65]);let eq15_e1103: f64 = (p.p87 * eq15_e1102);let eq15_e1103_d_n0: f64 = (p.p87 * (s.dn[65][0] * ddt_scale));let eq15_e1103_d_n1: f64 = (p.p87 * (s.dn[65][1] * ddt_scale));let eq15_e1103_d_n2: f64 = (p.p87 * (s.dn[65][2] * ddt_scale));let eq15_e1103_d_n3: f64 = (p.p87 * (s.dn[65][3] * ddt_scale));let eq15_e1103_d_n4: f64 = (p.p87 * (s.dn[65][4] * ddt_scale));let eq15_e1103_d_n5: f64 = (p.p87 * (s.dn[65][5] * ddt_scale));let eq15_e1103_d_n6: f64 = (p.p87 * (s.dn[65][6] * ddt_scale));let eq15_e1103_d_n7: f64 = (p.p87 * (s.dn[65][7] * ddt_scale));let eq15_e1103_d_n8: f64 = (p.p87 * (s.dn[65][8] * ddt_scale));let eq15_e1103_d_n9: f64 = (p.p87 * (s.dn[65][9] * ddt_scale));let eq15_e1103_d_n10: f64 = (p.p87 * (s.dn[65][10] * ddt_scale));let eq15_e1103_d_n11: f64 = (p.p87 * (s.dn[65][11] * ddt_scale));let eq15_e1103_d_n12: f64 = (p.p87 * (s.dn[65][12] * ddt_scale));let eq15_e1103_d_n13: f64 = (p.p87 * (s.dn[65][13] * ddt_scale));let eq15_e1103_d_n14: f64 = (p.p87 * (s.dn[65][14] * ddt_scale));let eq15_e1103_d_n15: f64 = (p.p87 * (s.dn[65][15] * ddt_scale));let eq15_e1103_d_n16: f64 = (p.p87 * (s.dn[65][16] * ddt_scale));let eq15_e1103_d_n17: f64 = (p.p87 * (s.dn[65][17] * ddt_scale));let eq15_e1103_d_b0: f64 = (p.p87 * (s.db[65][0] * ddt_scale));let eq15_e1103_d_b1: f64 = (p.p87 * (s.db[65][1] * ddt_scale));let eq15_e1103_d_b2: f64 = (p.p87 * (s.db[65][2] * ddt_scale));let eq15_e1103_d_b3: f64 = (p.p87 * (s.db[65][3] * ddt_scale));let eq15_e1103_d_b4: f64 = (p.p87 * (s.db[65][4] * ddt_scale));let eq15_e1103_d_b5: f64 = (p.p87 * (s.db[65][5] * ddt_scale));let eq15_e1103_d_b6: f64 = (p.p87 * (s.db[65][6] * ddt_scale));let eq15_e1103_d_b7: f64 = (p.p87 * (s.db[65][7] * ddt_scale));let eq15_e1103_d_b8: f64 = (p.p87 * (s.db[65][8] * ddt_scale));let eq15_e1103_d_b9: f64 = (p.p87 * (s.db[65][9] * ddt_scale));let eq15_e1103_d_b10: f64 = (p.p87 * (s.db[65][10] * ddt_scale));let eq15_e1103_d_b11: f64 = (p.p87 * (s.db[65][11] * ddt_scale));let eq15_value: f64 = eq15_e1103;let eq15_node_derivatives: [f64; 18] = [eq15_e1103_d_n0, eq15_e1103_d_n1, eq15_e1103_d_n2, eq15_e1103_d_n3, eq15_e1103_d_n4, eq15_e1103_d_n5, eq15_e1103_d_n6, eq15_e1103_d_n7, eq15_e1103_d_n8, eq15_e1103_d_n9, eq15_e1103_d_n10, eq15_e1103_d_n11, eq15_e1103_d_n12, eq15_e1103_d_n13, eq15_e1103_d_n14, eq15_e1103_d_n15, eq15_e1103_d_n16, eq15_e1103_d_n17];let eq15_branch_derivatives: [f64; 12] = [eq15_e1103_d_b0, eq15_e1103_d_b1, eq15_e1103_d_b2, eq15_e1103_d_b3, eq15_e1103_d_b4, eq15_e1103_d_b5, eq15_e1103_d_b6, eq15_e1103_d_b7, eq15_e1103_d_b8, eq15_e1103_d_b9, eq15_e1103_d_b10, eq15_e1103_d_b11];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(0),
            multiplicity * (eq15_value),
            &eq15_node_derivatives,
            &eq15_branch_derivatives,
            multiplicity,
        );
        let (eq16_e1109, eq16_e1109_d_n0, eq16_e1109_d_n1, eq16_e1109_d_n2, eq16_e1109_d_n3, eq16_e1109_d_n4, eq16_e1109_d_n5, eq16_e1109_d_n6, eq16_e1109_d_n7, eq16_e1109_d_n8, eq16_e1109_d_n9, eq16_e1109_d_n10, eq16_e1109_d_n11, eq16_e1109_d_n12, eq16_e1109_d_n13, eq16_e1109_d_n14, eq16_e1109_d_n15, eq16_e1109_d_n16, eq16_e1109_d_n17, eq16_e1109_d_b0, eq16_e1109_d_b1, eq16_e1109_d_b2, eq16_e1109_d_b3, eq16_e1109_d_b4, eq16_e1109_d_b5, eq16_e1109_d_b6, eq16_e1109_d_b7, eq16_e1109_d_b8, eq16_e1109_d_b9, eq16_e1109_d_b10, eq16_e1109_d_b11,) = {
    if s.b[3405] {
        let eq16_e1107: f64 = (p.p87 * s.v[870]);
        (eq16_e1107, (p.p87 * s.dn[870][0]), (p.p87 * s.dn[870][1]), (p.p87 * s.dn[870][2]), (p.p87 * s.dn[870][3]), (p.p87 * s.dn[870][4]), (p.p87 * s.dn[870][5]), (p.p87 * s.dn[870][6]), (p.p87 * s.dn[870][7]), (p.p87 * s.dn[870][8]), (p.p87 * s.dn[870][9]), (p.p87 * s.dn[870][10]), (p.p87 * s.dn[870][11]), (p.p87 * s.dn[870][12]), (p.p87 * s.dn[870][13]), (p.p87 * s.dn[870][14]), (p.p87 * s.dn[870][15]), (p.p87 * s.dn[870][16]), (p.p87 * s.dn[870][17]), (p.p87 * s.db[870][0]), (p.p87 * s.db[870][1]), (p.p87 * s.db[870][2]), (p.p87 * s.db[870][3]), (p.p87 * s.db[870][4]), (p.p87 * s.db[870][5]), (p.p87 * s.db[870][6]), (p.p87 * s.db[870][7]), (p.p87 * s.db[870][8]), (p.p87 * s.db[870][9]), (p.p87 * s.db[870][10]), (p.p87 * s.db[870][11]),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq16_value: f64 = eq16_e1109;let eq16_node_derivatives: [f64; 18] = [eq16_e1109_d_n0, eq16_e1109_d_n1, eq16_e1109_d_n2, eq16_e1109_d_n3, eq16_e1109_d_n4, eq16_e1109_d_n5, eq16_e1109_d_n6, eq16_e1109_d_n7, eq16_e1109_d_n8, eq16_e1109_d_n9, eq16_e1109_d_n10, eq16_e1109_d_n11, eq16_e1109_d_n12, eq16_e1109_d_n13, eq16_e1109_d_n14, eq16_e1109_d_n15, eq16_e1109_d_n16, eq16_e1109_d_n17];let eq16_branch_derivatives: [f64; 12] = [eq16_e1109_d_b0, eq16_e1109_d_b1, eq16_e1109_d_b2, eq16_e1109_d_b3, eq16_e1109_d_b4, eq16_e1109_d_b5, eq16_e1109_d_b6, eq16_e1109_d_b7, eq16_e1109_d_b8, eq16_e1109_d_b9, eq16_e1109_d_b10, eq16_e1109_d_b11];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(7),
            multiplicity * (eq16_value),
            &eq16_node_derivatives,
            &eq16_branch_derivatives,
            multiplicity,
        );
    }
}
