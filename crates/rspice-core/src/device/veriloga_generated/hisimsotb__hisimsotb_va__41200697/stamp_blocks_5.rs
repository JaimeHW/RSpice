#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_reactive_block_20(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        let mut t9: usize = 0;
        while {
            let t8: f64 = if (((!s.b[725]) && (!s.b[755])) && (s.v[62] < s.v[28])) { 1.0 } else { 0.0 };
            t8 != 0.0
        } {
            t9 += 1;assert!(t9 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((!s.b[725]) && (!s.b[755])) {s.copy_ad(279, 439);s.store_mul(280, 120, 307);s.store_exp_neg_input(281, 280);}
            s.b[765] = (s.v[307] > 1e-8);s.store_scalar(765, if s.b[765] { 1.0 } else { 0.0 });
            if (((!s.b[725]) && (!s.b[755])) && s.b[765]) {s.store_exp_mul(278, 120, 307);s.store_mul_scaled_sqrt_ad_rhs(282, 279, -1.0, A::add_scaled_offset_product_rhs(A::offset(A::add(s.ad_value(281), s.ad_value(280)), (-1.0)), 1.0, s.ad_value(143), s.ad_value(278), (-1.0), 1.0));s.store_mul_div_scaled_inputs_mixed_aii(283, A::add_scaled_sub_value_product(1.0, s.ad_value(281), 1.0, s.ad_value(143), s.ad_value(278), 1.0), 438, 1.0, 282, 1.0);}
            s.b[766] = (s.v[307] < (-1e-8));s.store_scalar(766, if s.b[766] { 1.0 } else { 0.0 });
            if ((((!s.b[725]) && (!s.b[755])) && (!s.b[765])) && s.b[766]) {s.store_mul_sqrt_mixed_ia(282, 279, A::offset(A::add(s.ad_value(281), s.ad_value(280)), (-1.0)));s.store_mul_scale_offset_mixed_ai(283, A::div(s.ad_value(438), s.ad_value(282)), 281, -1.0, 1.0);}
            if ((((!s.b[725]) && (!s.b[755])) && (!s.b[765])) && (!s.b[766])) {s.store_mul_ad_affine_product_lhs(282, A::sqrt(A::div(s.ad_value(438), s.ad_value(120))), s.ad_value(120), -1.0, 0.0, 307);s.store_neg_ad(283, A::sqrt(A::mul(s.ad_value(438), s.ad_value(120))));}
            if ((!s.b[725]) && (!s.b[755])) {s.store_sub_div_rhs_ad(284, 307, A::sub(A::add(A::add_scaled_inputs3(s.ad_value(56), 1.0, s.ad_value(307), (-1.0), s.ad_value(282), 1.0 / (s.v[294])), A::add_scaled_inputs(s.ad_value(282), (p.p227 * 9662367879.197212), s.ad_value(296), (0.5 * (p.p227 * 9662367879.197212)))), s.ad_value(440)), A::add_scaled_inputs(A::scale_offset(s.ad_value(283), 1.0 / (s.v[294]), (-1.0)), 1.0, s.ad_value(283), (p.p227 * 9662367879.197212)));}
            s.b[767] = ((((s.v[284] - s.v[307])) as f64).abs() < s.v[315]);s.store_scalar(767, if s.b[767] { 1.0 } else { 0.0 });
            if (((!s.b[725]) && (!s.b[755])) && s.b[767]) {s.copy_ad(285, 62);s.store_scalar(62, s.v[28]);}
            if ((!s.b[725]) && (!s.b[755])) {s.copy_ad(307, 284);s.copy_ad(312, 282);s.store_primal_offset(62, 62, 1.0);}
        }
        s.b[768] = (1.0 == 0.0);s.store_scalar(768, if s.b[768] { 1.0 } else { 0.0 });
        if (((!s.b[725]) && (!s.b[755])) && s.b[768]) {s.copy_ad(316, 312);}
        if ((!s.b[725]) && (!s.b[755])) {s.store_scalar(63, 0.0);}
        if (!s.b[725]) {s.store_offset_add(307, 440, 307, (-0.01));s.store_sub_scaled_inputs(306, 307, 1.0, 312, 1.0 / (s.v[294]));}
        s.b[769] = ((s.v[306] > (s.v[305] - 0.15)) && (0.15 >= 0.0));s.store_scalar(769, if s.b[769] { 1.0 } else { 0.0 });
        if ((!s.b[725]) && s.b[769]) {s.store_offset_sub(638, 306, 305, 0.15);s.store_square(642, 638);s.store_scalar(643, (0.15 * 0.15));s.store_scalar(644, 1.0);s.store_scalar(645, 1.0);s.store_scalar(647, 0.0);s.store_scalar(648, 0.0);s.store_scalar(220, 0.0);s.store_scalar(646, 0.0);s.store_mul(644, 644, 642);s.store_mul(645, 645, 643);s.store_add(220, 644, 645);s.copy_ad(646, 220);}
        s.b[770] = ((((1.0 == 1.0) || (1.0 == 2.0)) || (1.0 == 4.0)) || (1.0 == 8.0));s.store_scalar(770, if s.b[770] { 1.0 } else { 0.0 });s.b[771] = (1.0 == 1.0);s.store_scalar(771, if s.b[771] { 1.0 } else { 0.0 });
        if ((((!s.b[725]) && s.b[769]) && s.b[770]) && s.b[771]) {s.store_scalar(648, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_21(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[772] = (1.0 == 2.0);s.store_scalar(772, if s.b[772] { 1.0 } else { 0.0 });
        if (((((!s.b[725]) && s.b[769]) && s.b[770]) && (!s.b[771])) && s.b[772]) {s.store_scalar(648, 2.0);}
        s.b[773] = (1.0 == 4.0);s.store_scalar(773, if s.b[773] { 1.0 } else { 0.0 });
        if ((((((!s.b[725]) && s.b[769]) && s.b[770]) && (!s.b[771])) && (!s.b[772])) && s.b[773]) {s.store_scalar(648, 3.0);}
        s.b[774] = (1.0 == 8.0);s.store_scalar(774, if s.b[774] { 1.0 } else { 0.0 });
        if (((((((!s.b[725]) && s.b[769]) && s.b[770]) && (!s.b[771])) && (!s.b[772])) && (!s.b[773])) && s.b[774]) {s.store_scalar(648, 4.0);}
        if (((!s.b[725]) && s.b[769]) && s.b[770]) {s.store_scalar(647, 0.0);}
        let mut tb: usize = 0;
        while {
            let ta: f64 = if ((((!s.b[725]) && s.b[769]) && s.b[770]) && (s.v[647] < s.v[648])) { 1.0 } else { 0.0 };
            ta != 0.0
        } {
            tb += 1;assert!(tb <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((!s.b[725]) && s.b[769]) && s.b[770]) {s.store_sqrt(646, 646);s.store_primal_offset(647, 647, 1.0);}
        }
        if (((!s.b[725]) && s.b[769]) && (!s.b[770])) {s.store_powf(646, 646, (1.0 / 2.0));}
        if ((!s.b[725]) && s.b[769]) {s.store_div_from_scalar_offset_input(646, 1.0, 646, 1e-50);s.store_scaled_mul(637, 638, 646, 0.15);s.store_div_scaled_product_offset_denominator_indices(278, 645, 646, 0.15, 220, 1e-50, 1.0);s.store_add_offset_lhs(306, 305, (-0.15), 637);}
        if ((!s.b[725]) && s.b[769]) {
        }
        if ((!s.b[725]) && (!s.b[769])) {
        }
        if ((!s.b[725]) && (!s.b[769])) {s.store_scalar(278, 1.0);}
        if (!s.b[725]) {s.copy_ad(522, 306);}
        s.b[775] = ((p.p15 == 1.0) && (s.v[52] > (s.v[54] + 0.2)));s.store_scalar(775, if s.b[775] { 1.0 } else { 0.0 });
        if s.b[775] {s.store_scalar(389, s.v[559]);s.store_add_scaled_inputs4_indices(388, 72, 1.0, 389, (-1.0), 80, 1.0, 267, -1.0);s.store_scalar(32, p.p136);s.copy_ad(99, 388);s.store_sqrt_div_scaled_inputs(100, 471, ((2.0 * 1.6021918e-19) * 1.034943e-10), 120, 1.0);s.store_div_scaled_product_by_product_indices(101, 127, 127, 1.0, 471, 471, 1.0);s.store_div_scaled_product_by_product_indices(102, 100, 100, 1.0, 270, 270, 1.0);s.store_scaled_mul(103, 102, 120, 0.5);s.store_scaled_mul(104, 103, 120, 2.0);s.store_sqrt_offset_ad(105, A::div_scaled_offset_numerator(A::mul(s.ad_value(120), s.ad_value(99)), 4.0, ((-1.0) * 4.0), s.ad_value(104), 1.0), 1.0);s.store_add_mul_sub_from_scalar_rhs_indices(107, 99, 103, 1.0, 105);s.store_div_scalar_by_product_indices(108, 1.0, 101, 102, 1.0);s.store_div_ad(109, A::ln(A::mul(s.ad_value(108), A::square(s.ad_value(99)))), A::add(s.ad_value(120), A::div_from_scalar(2.0, s.ad_value(99))));s.store_add_scaled_inputs3_indices(110, 109, 1.0, 107, (-1.0), 32, -1.0);s.store_add_scaled_inputs3_sqrt_third_mixed_iia(111, 109, 1.0, 110, (-0.5), A::add_scaled_square_product(s.ad_value(110), 1.0, s.ad_value(32), s.ad_value(109), 4.0), (-0.5));s.store_exp_mul(112, 120, 111);s.store_add_scaled_product_mixed_aii(113, A::offset(A::mul(s.ad_value(120), s.ad_value(111)), (-1.0)), 1.0, 101, 112, 1.0);s.store_offset_mul(114, 120, 111, (-1.0));}
        s.b[776] = ((s.v[113] > 0.0) && (s.v[114] > 0.0));s.store_scalar(776, if s.b[776] { 1.0 } else { 0.0 });
        if (s.b[775] && s.b[776]) {s.store_sqrt_ad(113, A::add_scaled_product(A::offset(A::mul(s.ad_value(120), s.ad_value(111)), (-1.0)), 1.0, s.ad_value(101), s.ad_value(112), 1.0));s.store_sqrt_offset_ad(114, A::mul(s.ad_value(120), s.ad_value(111)), (-1.0));s.store_mul_sub_rhs(115, 100, 113, 114);s.store_div_from_scalar(106, (2.0 * s.v[124]), 120);s.store_scalar(158, (300.0 * 0.0001));s.store_scalar(262, 0.0);s.store_neg_ad(279, A::offset(A::exp(A::mul_scaled_lhs(s.ad_value(120), -1.0, s.ad_value(71))), (-1.0)));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_22(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[775] && s.b[776]) {s.store_div_scaled_product_mixed_aia(116, A::mul3(s.ad_value(106), s.ad_value(158), s.ad_value(115)), 279, 1.0, A::sub(s.ad_value(123), s.ad_value(262)), 1.0);s.copy_ad(338, 116);s.copy_ad(339, 111);s.store_offset_div_scaled_offset_numerator(290, A::mul(s.ad_value(120), s.ad_value(76)), 4.0, ((-1.0) * 4.0), A::mul(s.ad_value(145), s.ad_value(121)), 1.0, 1.0);}
        s.b[777] = (s.v[290] < (10.0 * 2.220446049250313e-16));s.store_scalar(777, if s.b[777] { 1.0 } else { 0.0 });
        if ((s.b[775] && s.b[776]) && s.b[777]) {s.store_scalar(290, (10.0 * 2.220446049250313e-16));}
        if (s.b[775] && s.b[776]) {s.store_add_product3_rhs_mixed_iia(319, 76, 145, 120, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(290))), 0.5);s.copy_ad(58, 319);s.store_sub(61, 319, 339);}
        s.b[778] = (s.v[61] < 0.0);s.store_scalar(778, if s.b[778] { 1.0 } else { 0.0 });
        if ((s.b[775] && s.b[776]) && s.b[778]) {s.store_scalar(61, 0.0);}
        if (s.b[775] && s.b[776]) {s.store_scale(283, 61, (1.0 + 0.3));s.store_offset_sub(284, 283, 71, (-0.03));s.store_sqrt_add_scaled_square_input(285, 284, 1.0, 283, (4.0 * 0.03));s.store_add_scaled_inputs3_indices(60, 283, 1.0, 284, (-0.5), 285, (-0.5));}
        s.b[779] = (s.v[60] > s.v[61]);s.store_scalar(779, if s.b[779] { 1.0 } else { 0.0 });
        if ((s.b[775] && s.b[776]) && s.b[779]) {s.copy_ad(60, 61);}
        if (s.b[775] && s.b[776]) {s.copy_ad(392, 60);s.store_scalar(796, (s.v[272] * 100.0));s.store_scalar(797, (s.v[466] * 100.0));s.store_scale(798, 123, 100.0);}
        s.b[799] = (p.p26 == 0.0);s.store_scalar(799, if s.b[799] { 1.0 } else { 0.0 });
        if ((s.b[775] && s.b[776]) && s.b[799]) {s.store_scalar(390, 0.0);}
        if ((s.b[775] && s.b[776]) && (!s.b[799])) {s.store_scalar(391, 4.12);s.store_scaled_mul(780, 797, 798, (p.p141 * 1.6021918e-19));s.store_div(781, 780, 245);s.store_div_scaled_inputs_mixed_ai(782, A::offset(A::add_scaled_inputs4(s.ad_value(70), p.p144, s.ad_value(82), 1.0, s.ad_value(266), 1.0, s.ad_value(137), 1.0), p.p143), -1.0, 796, 1.0);s.store_scalar(514, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_23(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        let mut te: usize = 0;
        while {
            let tc: f64 = (100.0 - 1.0);let td: f64 = if (((s.b[775] && s.b[776]) && (!s.b[799])) && (s.v[514] <= tc)) { 1.0 } else { 0.0 };
            td != 0.0
        } {
            te += 1;assert!(te <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.b[775] && s.b[776]) && (!s.b[799])) {s.copy_ad(783, 514);s.store_scalar(784, 100.0);s.store_primal_div(785, 783, 784);s.store_add_scaled_inputs3_mixed_iia(786, 53, 1.0, 73, 1.0, A::add_scaled_product(s.ad_value(339), 1.0, s.ad_value(392), s.ad_value(785), 1.0), -1.0);s.store_sub_from_scalar_div_indices(787, 1.0, 786, 391);s.store_add_div_rhs_indices(790, 782, 786, 796);s.store_square(788, 790);s.store_sqrt_square_offset(639, 787, ((4.0 * 0.001) * 0.001));s.store_offset_add_scaled_inputs_indices(787, 787, 0.5, 639, 0.5, (1e-10 * 0.001));}
            s.b[800] = (s.v[787] < 0.0);s.store_scalar(800, if s.b[800] { 1.0 } else { 0.0 });
            if (((s.b[775] && s.b[776]) && (!s.b[799])) && s.b[800]) {s.store_scalar(787, 0.0);}
            if ((s.b[775] && s.b[776]) && (!s.b[799])) {s.store_offset_scaled_ad(789, A::mul(A::sqrt(s.ad_value(787)), s.ad_value(787)), (-p.p142), p.p142);s.store_div_scaled_inputs_indices(791, 789, -1.0, 790, 1.0);}
            s.b[801] = (s.v[791] < (-34.0));s.store_scalar(801, if s.b[801] { 1.0 } else { 0.0 });
            if (((s.b[775] && s.b[776]) && (!s.b[799])) && s.b[801]) {s.store_scalar(792, 0.0);}
            if (((s.b[775] && s.b[776]) && (!s.b[799])) && (!s.b[801])) {s.store_exp(792, 791);}
            if ((s.b[775] && s.b[776]) && (!s.b[799])) {s.copy_ad(793, 781);s.store_mul3_affine_lhs(794, 793, 789, (0.25 * 7.38905609893065), 0.0, 789);}
            s.b[802] = (((2.0 * s.v[790]) + s.v[789]) < 0.0);s.store_scalar(802, if s.b[802] { 1.0 } else { 0.0 });
            if (((s.b[775] && s.b[776]) && (!s.b[799])) && s.b[802]) {s.copy_ad(393, 794);}
            if (((s.b[775] && s.b[776]) && (!s.b[799])) && (!s.b[802])) {s.store_mul3_lhs(795, 780, 788, 792);}
            s.b[803] = ((s.v[795] < s.v[794]) || (s.v[790] < 0.0));s.store_scalar(803, if s.b[803] { 1.0 } else { 0.0 });
            if ((((s.b[775] && s.b[776]) && (!s.b[799])) && (!s.b[802])) && s.b[803]) {s.copy_ad(393, 794);}
            if ((((s.b[775] && s.b[776]) && (!s.b[799])) && (!s.b[802])) && (!s.b[803])) {s.copy_ad(393, 795);}
            if ((s.b[775] && s.b[776]) && (!s.b[799])) {s.store_add(390, 390, 393);}
            s.b[804] = (s.v[393] < 1e-9);s.store_scalar(804, if s.b[804] { 1.0 } else { 0.0 });
            if (((s.b[775] && s.b[776]) && (!s.b[799])) && s.b[804]) {s.store_scalar(514, 100.0);s.store_scalar(62, s.v[28]);}
            if ((s.b[775] && s.b[776]) && (!s.b[799])) {s.store_primal_offset(514, 514, 1.0);}
        }
        s.b[805] = ((s.v[488] <= 0.0) || (s.v[162] <= 0.0));s.store_scalar(805, if s.b[805] { 1.0 } else { 0.0 });
        if ((s.b[775] && s.b[776]) && s.b[805]) {s.store_scalar(185, 0.0);}
        if ((s.b[775] && s.b[776]) && (!s.b[805])) {s.copy_ad(279, 388);s.store_square(285, 270);s.store_mul_div_from_scalar_lhs_ad_indices(282, 2.0, 472, 285);s.store_add_scaled_inputs3_indices(283, 279, 1.0, 122, (-1.0), 70, (-s.v[486]));s.store_offset_mul(284, 282, 283, 1.0);s.store_sqrt_square_offset(639, 284, ((4.0 * 0.001) * 0.001));s.store_offset_scaled_div(287, 284, 639, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(284, 284, 0.5, 639, 0.5, (1e-10 * 0.001));}
        s.b[806] = (s.v[284] < 0.0);s.store_scalar(806, if s.b[806] { 1.0 } else { 0.0 });
        if (((s.b[775] && s.b[776]) && (!s.b[805])) && s.b[806]) {s.store_scalar(284, 0.0);s.store_scalar(287, 0.0);}
        if ((s.b[775] && s.b[776]) && (!s.b[805])) {s.store_offset(284, 284, 1e-50);s.store_add_scaled_inputs_mixed_ia(186, 279, s.v[491], A::mul_sub_from_scalar_rhs(A::div(s.ad_value(472), s.ad_value(285)), 1.0, A::sqrt(s.ad_value(284))), 1.0);s.store_add_scaled_inputs3_indices(187, 71, p.p123, 339, 1.0, 186, (-(s.v[487] * s.v[485])));s.store_sqrt_square_offset(639, 187, ((4.0 * 0.01) * 0.01));s.store_offset_scaled_div(287, 187, 639, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(187, 187, 0.5, 639, 0.5, (1e-10 * 0.01));}
        s.b[807] = (s.v[187] < 0.0);s.store_scalar(807, if s.b[807] { 1.0 } else { 0.0 });
        if (((s.b[775] && s.b[776]) && (!s.b[805])) && s.b[807]) {s.store_scalar(187, 0.0);s.store_scalar(287, 0.0);}
        if ((s.b[775] && s.b[776]) && (!s.b[805])) {s.store_offset(187, 187, 1e-50);s.store_exp_ad(280, A::div_from_scalar((-s.v[489]), s.ad_value(187)));s.store_mul3_affine_lhs(185, 187, 338, s.v[488], 0.0, 280);}
        s.b[808] = (p.p16 == 1.0);s.store_scalar(808, if s.b[808] { 1.0 } else { 0.0 });
        if ((s.b[775] && s.b[776]) && s.b[808]) {s.store_scaled_exp_scaled_input(279, 120, (-p.p140), ((1.6021918e-19 * p.p227) * s.v[466]));s.store_offset_scaled(280, 471, (((((36.0 * 1e-7) / 0.0001)) as f64).sqrt() * 13.0), ((((((13.0 * 1e-7) / 0.0001)) as f64).sqrt() * 36.0) * (1e20 / 1e-6)));s.store_div_scalar_by_product_indices(282, (((((13.0 * 1e-7) / 0.0001)) as f64).sqrt() * ((((36.0 * 1e-7) / 0.0001)) as f64).sqrt()), 279, 280, 1.0);s.store_mul_add_lhs(520, 185, 390, 282);s.store_mul_scaled_ln_offset_rhs(283, 122, p.p139, 520, 1.0);s.store_sqrt_mul_scaled_lhs(284, 471, ((2.0 * 1.034943e-10) * 1.6021918e-19), 122);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_24(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if ((s.b[775] && s.b[776]) && s.b[808]) {s.store_sqrt_ad(285, A::add_scaled_product(A::offset(A::exp(A::mul_scaled_lhs(s.ad_value(120), -1.0, A::sub(s.ad_value(339), s.ad_value(283)))), (-1.0)), 1.0, s.ad_value(120), A::sub(s.ad_value(339), s.ad_value(283)), 1.0));s.store_sqrt_ad(286, A::add_scaled_product(A::offset(A::exp(A::mul_scaled_lhs(s.ad_value(120), -1.0, s.ad_value(339))), (-1.0)), 1.0, s.ad_value(120), s.ad_value(339), 1.0));s.store_mul_sub_scaled_inputs_rhs_indices(337, 284, 285, -1.0, 286, -1.0);}
        if (((s.b[775] && s.b[776]) && s.b[808]) && (p.p27 != 0.0)) {s.store_div_from_scalar_offset_input(342, p.p137, 185, p.p138);s.store_mul(341, 342, 270);s.copy_ad(340, 337);s.store_scaled_voltage(562, ctx, nodes, Some(10), None, 1e-9);s.copy_ad(337, 562);s.store_div_scaled_inputs2_indices(558, 562, 1.0, 340, (-1.0), 341, 1.0);}
        if ((s.b[775] && s.b[776]) && (!s.b[808])) {s.store_scalar(337, 0.0);}
        if (s.b[775] && (!s.b[776])) {s.store_scalar(185, 0.0);s.store_scalar(337, 0.0);}
        if (!s.b[775]) {s.store_scalar(185, 0.0);s.store_scalar(337, 0.0);}
        s.copy_ad(299, 305);s.copy_ad(300, 306);s.store_sub(301, 307, 440);s.store_scalar(379, 0.0);s.store_scalar(606, 1.0);s.store_scalar(604, 0.0);s.store_scalar(605, 0.0);s.b[809] = (s.v[649] < 4.0);s.store_scalar(809, if s.b[809] { 1.0 } else { 0.0 });
        if s.b[809] {s.copy_ad(599, 296);s.store_neg(600, 599);s.store_div_scalar_by_product_mixed_ai(601, 0.004832, A::square(s.ad_value(296)), 296, 1.0);s.store_scale(603, 296, (-3.7477));s.store_scale(602, 296, 4.3495);}
        if (!s.b[809]) {s.store_scale(599, 296, 1.5);s.store_neg(600, 599);s.store_div_scalar_by_product_mixed_ai(601, 0.001765, A::square(s.ad_value(296)), 296, 1.0);s.store_scale(603, 296, (-4.8303));s.store_scale(602, 296, 5.9661);}
        s.copy_ad(306, 300);s.copy_ad(534, 300);s.copy_ad(522, 534);s.copy_ad(307, 301);s.store_scalar(62, 1.0);
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_25(
        s: &mut ReactiveScratch,
    ) {
        let mut t1: usize = 0;
        while {
            let t0: f64 = if s.v[62] <= s.v[28] { 1.0 } else { 0.0 };
            t0 != 0.0
        } {
            t1 += 1;assert!(t1 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");s.copy_ad(279, 307);s.store_mul(297, 120, 279);s.store_exp_neg_input(278, 297);s.b[810] = (s.v[279] < (-1e-8));s.store_scalar(810, if s.b[810] { 1.0 } else { 0.0 });
            if s.b[810] {s.store_exp_mul(280, 120, 307);s.store_mul_sqrt_mixed_ia(312, 439, A::add_scaled_offset_product_rhs(A::offset(A::add(s.ad_value(278), s.ad_value(297)), (-1.0)), 1.0, s.ad_value(143), s.ad_value(280), (-1.0), 1.0));s.store_div_scaled_product_mixed_iai(343, 438, A::add_scaled_sub_value_product(1.0, s.ad_value(278), 1.0, s.ad_value(143), s.ad_value(280), 1.0), 1.0, 312, 1.0);}
            s.b[811] = (s.v[279] > (1e-8 / 10.0));s.store_scalar(811, if s.b[811] { 1.0 } else { 0.0 });
            if ((!s.b[810]) && s.b[811]) {s.store_exp_mul(280, 120, 307);s.store_mul_scaled_sqrt_ad_rhs(312, 439, -1.0, A::add_scaled_offset_product_rhs(A::offset(A::add(s.ad_value(278), s.ad_value(297)), (-1.0)), 1.0, s.ad_value(143), A::sub(s.ad_value(280), s.ad_value(297)), (-1.0), 1.0));s.store_div_scaled_product_mixed_iai(343, 438, A::add_scaled_sub_value_product(1.0, s.ad_value(278), 1.0, s.ad_value(143), A::offset(s.ad_value(280), (-1.0)), 1.0), 1.0, 312, 1.0);}
            if ((!s.b[810]) && (!s.b[811])) {s.store_scaled_mul(312, 439, 297, (-1.0 / (((2.0) as f64).sqrt())));s.store_scaled_mul(343, 439, 120, (-1.0 / (((2.0) as f64).sqrt())));}
            s.store_add_scaled_inputs4_indices(306, 307, 1.0, 312, (-1.0 / (s.v[294])), 50, 1.0, 298, 1.0);s.store_sub_from_scalar_scaled_input(583, 1.0, 343, 1.0 / (s.v[294]));s.store_sub(279, 305, 522);s.store_mul(297, 120, 279);s.b[812] = ((-s.v[297]) >= 80.0);s.store_scalar(812, if s.b[812] { 1.0 } else { 0.0 });
            if s.b[812] {s.store_scaled_offset_ad(278, A::sub_from_scalar(1.0, s.ad_value(297)), (-80.0), 5.540622384e34);s.store_scalar(284, 5.540622384e34);}
            if (!s.b[812]) {s.store_exp_neg_input(278, 297);s.copy_ad(284, 278);}
            s.b[813] = (s.v[279] < (-1e-8));s.store_scalar(813, if s.b[813] { 1.0 } else { 0.0 });
            if s.b[813] {s.store_sqrt_offset_ad(280, A::add(s.ad_value(278), s.ad_value(297)), (-1.0));s.store_mul(523, 141, 280);s.store_div_scaled_product3_mixed_iiai(524, 141, 120, A::sub_from_scalar(1.0, s.ad_value(284)), 1.0, 280, 2.0);s.store_neg(525, 524);s.store_scalar(311, 0.0);s.store_scalar(526, 0.0);s.store_scalar(527, 0.0);}
            s.b[814] = (s.v[279] > 1e-8);s.store_scalar(814, if s.b[814] { 1.0 } else { 0.0 });
            if ((!s.b[813]) && s.b[814]) {s.store_sqrt_offset_ad(280, A::add(s.ad_value(278), s.ad_value(297)), (-1.0));s.store_mul_scale_offset_indices(523, 280, 141, -1.0, 0.0);s.store_div_scaled_product3_mixed_iiai(524, 141, 120, A::sub_from_scalar(1.0, s.ad_value(284)), -1.0, 280, 2.0);s.store_neg(525, 524);s.store_exp(278, 297);s.store_exp_mul(281, 120, 522);s.store_sqrt_add_ad(282, A::div_scaled_product(s.ad_value(523), s.ad_value(523), 1.0, A::square(s.ad_value(141)), 1.0), A::mul3_scaled_output(s.ad_value(142), s.ad_value(281), A::offset(A::sub(s.ad_value(278), s.ad_value(297)), (-1.0)), 2.0));s.store_div_scaled_inputs_mixed_ai(537, A::add_scaled_offset_product_rhs(A::div_scaled_product(s.ad_value(523), s.ad_value(524), 2.0, A::square(s.ad_value(141)), 1.0), 1.0, A::mul3_scaled_output(s.ad_value(120), s.ad_value(142), s.ad_value(281), 2.0), s.ad_value(278), (-1.0), 1.0), 1.0, 282, 2.0);s.store_div_scaled_add_product_mixed_aaii(538, A::div_scaled_product(s.ad_value(523), s.ad_value(525), 2.0, A::square(s.ad_value(141)), 1.0), 1.0, A::mul3_scaled_output(s.ad_value(120), s.ad_value(142), s.ad_value(281), 2.0), 297, (-1.0), 282, 2.0);s.store_add_scaled_product_indices(311, 523, (-1.0), 141, 282, -1.0);s.store_add_scaled_product_indices(526, 524, (-1.0), 141, 537, -1.0);s.store_add_scaled_product_indices(527, 525, (-1.0), 141, 538, -1.0);}
            if ((!s.b[813]) && (!s.b[814])) {s.store_scaled_mul(523, 141, 297, (-1.0 / (((2.0) as f64).sqrt())));s.store_scaled_mul(524, 141, 120, (-1.0 / (((2.0) as f64).sqrt())));s.store_neg(525, 524);s.store_scalar(311, 0.0);s.store_scalar(526, 0.0);s.store_scalar(527, 0.0);}
            s.store_sub(279, 306, 522);s.store_mul(297, 120, 279);s.b[815] = ((-s.v[297]) >= 80.0);s.store_scalar(815, if s.b[815] { 1.0 } else { 0.0 });
            if s.b[815] {s.store_scaled_offset_ad(278, A::sub_from_scalar(1.0, s.ad_value(297)), (-80.0), 5.540622384e34);s.store_scalar(284, 5.540622384e34);}
            if (!s.b[815]) {s.store_exp_neg_input(278, 297);s.copy_ad(284, 278);}
            s.b[816] = (s.v[279] < (-1e-8));s.store_scalar(816, if s.b[816] { 1.0 } else { 0.0 });
            if s.b[816] {s.store_sqrt_offset_ad(280, A::add(s.ad_value(278), s.ad_value(297)), (-1.0));s.store_mul(531, 141, 280);s.store_div_scaled_product3_mixed_iiai(532, 141, 120, A::sub_from_scalar(1.0, s.ad_value(284)), 1.0, 280, 2.0);s.store_neg(533, 532);s.store_scalar(528, 0.0);s.store_scalar(529, 0.0);s.store_scalar(530, 0.0);}
            s.b[817] = (s.v[279] > 1e-8);s.store_scalar(817, if s.b[817] { 1.0 } else { 0.0 });
            if ((!s.b[816]) && s.b[817]) {s.store_sqrt_offset_ad(280, A::add(s.ad_value(278), s.ad_value(297)), (-1.0));s.store_mul_scale_offset_indices(531, 280, 141, -1.0, 0.0);s.store_div_scaled_product3_mixed_iiai(532, 141, 120, A::sub_from_scalar(1.0, s.ad_value(284)), -1.0, 280, 2.0);s.store_neg(533, 532);s.store_exp(278, 297);s.store_exp_mul(281, 120, 522);s.store_sqrt_add_ad(282, A::div_scaled_product(s.ad_value(531), s.ad_value(531), 1.0, A::square(s.ad_value(141)), 1.0), A::mul3_scaled_output(s.ad_value(142), s.ad_value(281), A::offset(A::sub(s.ad_value(278), s.ad_value(297)), (-1.0)), 2.0));s.store_div_scaled_inputs_mixed_ai(539, A::add_scaled_offset_product_rhs(A::div_scaled_product(s.ad_value(531), s.ad_value(532), 2.0, A::square(s.ad_value(141)), 1.0), 1.0, A::mul3_scaled_output(s.ad_value(120), s.ad_value(142), s.ad_value(281), 2.0), s.ad_value(278), (-1.0), 1.0), 1.0, 282, 2.0);s.store_div_scaled_add_product_mixed_aaii(538, A::div_scaled_product(s.ad_value(531), s.ad_value(533), 2.0, A::square(s.ad_value(141)), 1.0), 1.0, A::mul3_scaled_output(s.ad_value(120), s.ad_value(142), s.ad_value(281), 2.0), 297, (-1.0), 282, 2.0);s.store_add_scaled_product_indices(528, 531, (-1.0), 141, 282, -1.0);s.store_add_scaled_product_indices(529, 532, (-1.0), 141, 539, -1.0);s.store_add_scaled_product_indices(530, 533, (-1.0), 141, 538, -1.0);}
            if ((!s.b[816]) && (!s.b[817])) {s.store_scaled_mul(531, 141, 297, (-1.0 / (((2.0) as f64).sqrt())));s.store_scaled_mul(532, 141, 120, (-1.0 / (((2.0) as f64).sqrt())));s.store_neg(533, 532);s.store_scalar(528, 0.0);s.store_scalar(529, 0.0);s.store_scalar(530, 0.0);}
            s.b[818] = (s.v[379] == 1.0);s.store_scalar(818, if s.b[818] { 1.0 } else { 0.0 });
            if s.b[818] {s.store_scalar(574, s.v[62]);s.store_scalar(62, s.v[28]);}
            if (!s.b[818]) {s.store_add_scaled_inputs3_mixed_iia(346, 305, 1.0, 76, (-1.0), A::div(A::add(A::add(A::add_scaled_inputs4(s.ad_value(312), 1.0, s.ad_value(311), 1.0, s.ad_value(523), 1.0, s.ad_value(528), 1.0), s.ad_value(531)), s.ad_value(337)), s.ad_value(270)), -1.0);s.store_sub_from_scalar_ad(347, 1.0, A::div_scaled_inputs2(s.ad_value(526), 1.0, s.ad_value(524), 1.0, s.ad_value(270), 1.0));s.store_div_scaled_inputs_mixed_ai(348, A::add_scaled_inputs4(s.ad_value(527), 1.0, s.ad_value(525), 1.0, s.ad_value(530), 1.0, s.ad_value(533), 1.0), -1.0, 270, 1.0);s.store_div_scaled_inputs_mixed_ai(349, A::add_scaled_product(s.ad_value(343), 1.0, A::add(s.ad_value(529), s.ad_value(532)), s.ad_value(583), 1.0), -1.0, 270, 1.0);}
            s.b[819] = (s.v[312] <= s.v[599]);s.store_scalar(819, if s.b[819] { 1.0 } else { 0.0 });
            if ((!s.b[818]) && s.b[819]) {s.store_sqrt_mul_ad(279, s.ad_value(296), A::add_scaled_inputs(s.ad_value(312), 2.0, s.ad_value(296), 1.0));s.store_div_scaled_product_indices(604, 296, 343, 1.0, 279, 1.0);}
            s.b[820] = (s.v[312] <= s.v[603]);s.store_scalar(820, if s.b[820] { 1.0 } else { 0.0 });
            if (((!s.b[818]) && (!s.b[819])) && s.b[820]) {s.store_mul3_ad(279, A::mul3(s.ad_value(601), A::sub(s.ad_value(312), s.ad_value(603)), A::sub(s.ad_value(312), s.ad_value(603))), A::sub(s.ad_value(312), s.ad_value(603)), A::sub(s.ad_value(312), s.ad_value(602)));s.store_mul_ad_product_lhs(604, A::mul3(s.ad_value(601), A::sub(s.ad_value(312), s.ad_value(603)), A::sub(s.ad_value(312), s.ad_value(603))), A::add_scaled_inputs4(s.ad_value(312), 3.0, s.ad_value(602), (-3.0), s.ad_value(312), 1.0, s.ad_value(603), (-1.0)), 343);}
            if (((!s.b[818]) && (!s.b[819])) && (!s.b[820])) {s.store_scalar(279, 0.0);s.store_scalar(604, 0.0);}
            if (!s.b[818]) {s.store_div_scaled_inputs_indices(281, 316, (-s.v[650]), 296, 1.0);s.store_div_from_scalar_offset_ad(280, 1.0, A::exp_scaled_input(s.ad_value(281), -1.0), 1.0);s.store_mul_square_exp_scaled_input(278, 280, 281, -1.0);s.store_mul(280, 280, 600);s.store_neg_add(279, 296, 280);s.store_scalar(604, 0.0);s.store_scaled_add(350, 523, 279, 1.0 / (s.v[535]));s.store_scale(351, 524, 1.0 / (s.v[535]));s.store_scale(352, 525, 1.0 / (s.v[535]));s.store_scale(353, 604, 1.0 / (s.v[535]));s.store_div_scaled_inputs_indices(281, 316, (-s.v[651]), 296, 1.0);s.store_div_from_scalar_offset_ad(280, 1.0, A::exp_scaled_input(s.ad_value(281), -1.0), 1.0);s.store_mul_square_exp_scaled_input(278, 280, 281, -1.0);s.store_mul(280, 280, 600);s.store_scalar(605, 0.0);s.store_scaled_add(354, 531, 280, 1.0 / (s.v[535]));s.store_scale(355, 533, 1.0 / (s.v[535]));s.store_add_scaled_product_indices(356, 605, 1.0 / (s.v[535]), 532, 583, 1.0 / (s.v[535]));s.store_add_scaled_inputs4(357, A::mul3(s.ad_value(347), s.ad_value(352), s.ad_value(356)), 1.0, A::mul3(s.ad_value(347), s.ad_value(353), s.ad_value(355)), (-1.0), A::mul3(s.ad_value(348), s.ad_value(351), s.ad_value(356)), -1.0, A::mul3(s.ad_value(349), s.ad_value(351), s.ad_value(355)), 1.0);}
            s.b[821] = (s.v[357] > 0.0);s.store_scalar(821, if s.b[821] { 1.0 } else { 0.0 });
            if ((!s.b[818]) && s.b[821]) {s.store_div_from_scalar_offset_input(358, 1.0, 357, 1e-50);}
            if ((!s.b[818]) && (!s.b[821])) {s.store_div_from_scalar_offset_input(358, 1.0, 357, (-1e-50));}
            if (!s.b[818]) {s.store_add_scaled_products_indices(359, 352, 356, 1.0, 353, 355, (-1.0));s.store_add_scaled_products_indices(360, 349, 355, 1.0, 348, 356, (-1.0));s.store_add_scaled_products_indices(361, 348, 353, 1.0, 349, 352, (-1.0));s.store_mul_scale_offset_indices(362, 356, 351, -1.0, 0.0);s.store_mul(363, 347, 356);s.store_add_scaled_products_indices(364, 349, 351, 1.0, 347, 353, (-1.0));s.store_mul(365, 351, 355);s.store_mul_scale_offset_indices(366, 355, 347, -1.0, 0.0);s.store_add_scaled_products_indices(367, 347, 352, 1.0, 348, 351, (-1.0));s.store_mul_add_scaled_products3_indices_rhs(368, 358, 359, 346, -1.0, 360, 350, -1.0, 361, 354, -1.0);s.store_mul_add_scaled_products3_indices_rhs(369, 358, 362, 346, -1.0, 363, 350, -1.0, 364, 354, -1.0);s.store_mul_add_scaled_products3_indices_rhs(370, 358, 365, 346, -1.0, 366, 350, -1.0, 367, 354, -1.0);s.store_abs(279, 368);}
            s.b[822] = (s.v[279] < ((s.v[369]) as f64).abs());s.store_scalar(822, if s.b[822] { 1.0 } else { 0.0 });
            if ((!s.b[818]) && s.b[822]) {s.store_abs(279, 369);}
            s.b[823] = (s.v[279] < ((s.v[370]) as f64).abs());s.store_scalar(823, if s.b[823] { 1.0 } else { 0.0 });
            if ((!s.b[818]) && s.b[823]) {s.store_abs(279, 370);}
            if (!s.b[818]) {s.store_scalar(606, 1.0);}
            s.b[824] = (s.v[62] > 80.0);s.store_scalar(824, if s.b[824] { 1.0 } else { 0.0 });
            if ((!s.b[818]) && s.b[824]) {s.store_scalar(606, 25.0);}
            s.b[825] = (s.v[62] > 40.0);s.store_scalar(825, if s.b[825] { 1.0 } else { 0.0 });
            if (((!s.b[818]) && (!s.b[824])) && s.b[825]) {s.store_scalar(606, 25.0);}
            s.b[826] = (s.v[62] > 20.0);s.store_scalar(826, if s.b[826] { 1.0 } else { 0.0 });
            if ((((!s.b[818]) && (!s.b[824])) && (!s.b[825])) && s.b[826]) {s.store_scalar(606, 25.0);}
            s.b[827] = (s.v[62] > 10.0);s.store_scalar(827, if s.b[827] { 1.0 } else { 0.0 });
            if (((((!s.b[818]) && (!s.b[824])) && (!s.b[825])) && (!s.b[826])) && s.b[827]) {s.store_scalar(606, 5.0);}
            s.b[828] = (s.v[279] > (0.1 / s.v[606]));s.store_scalar(828, if s.b[828] { 1.0 } else { 0.0 });
            if ((!s.b[818]) && s.b[828]) {s.store_mul_mixed_ia(368, 368, A::div_scalar_by_product(0.1, s.ad_value(606), s.ad_value(279), 1.0));s.store_mul_mixed_ia(369, 369, A::div_scalar_by_product(0.1, s.ad_value(606), s.ad_value(279), 1.0));s.store_mul_mixed_ia(370, 370, A::div_scalar_by_product(0.1, s.ad_value(606), s.ad_value(279), 1.0));}
            if (!s.b[818]) {s.store_add(305, 305, 368);s.store_add(522, 522, 369);s.store_add(307, 307, 370);s.store_primal_scale(607, 606, 1e-12);}
            s.b[829] = (s.v[279] < s.v[607]);s.store_scalar(829, if s.b[829] { 1.0 } else { 0.0 });
            if ((!s.b[818]) && s.b[829]) {s.store_scalar(379, 1.0);}
            s.store_primal_offset(62, 62, 1.0);
        }
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_26(
        s: &mut ReactiveScratch,
    ) {
        s.b[830] = (s.v[574] > 0.0);s.store_scalar(830, if s.b[830] { 1.0 } else { 0.0 });
        if s.b[830] {s.copy_ad(62, 574);s.store_scalar(574, 0.0);}
        s.b[831] = (s.v[62] > s.v[28]);s.store_scalar(831, if s.b[831] { 1.0 } else { 0.0 });
        if s.b[831] {s.copy_ad(305, 299);s.copy_ad(306, 300);s.copy_ad(307, 301);s.copy_ad(522, 534);}
        s.copy_ad(56, 305);s.store_neg(149, 311);s.b[833] = (s.v[149] <= 1e-50);s.store_scalar(833, if s.b[833] { 1.0 } else { 0.0 });
        if s.b[833] {s.store_scalar(149, 1e-50);s.store_scalar(34, 1.0);}
        s.store_neg(150, 528);s.b[834] = (s.v[150] <= 1e-50);s.store_scalar(834, if s.b[834] { 1.0 } else { 0.0 });
        if s.b[834] {s.store_scalar(150, 1e-50);}
        s.store_mul(86, 149, 271);s.copy_ad(396, 51);s.store_div_square_rhs(280, 472, 270);s.store_sub(278, 76, 122);s.store_offset_mul_ad(287, A::div_from_scalar(2.0, s.ad_value(280)), s.ad_value(278), 1.0);s.store_sqrt_square_offset(639, 287, ((4.0 * 0.05) * 0.05));s.store_offset_scaled_div(284, 287, 639, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(287, 287, 0.5, 639, 0.5, (1e-10 * 0.05));s.b[835] = (s.v[287] < 0.0);s.store_scalar(835, if s.b[835] { 1.0 } else { 0.0 });
        if s.b[835] {s.store_scalar(287, 0.0);s.store_scalar(284, 0.0);}
        s.store_sqrt(281, 287);s.store_add_mul_sub_from_scalar_rhs_indices(288, 76, 280, 1.0, 281);s.store_sqrt_square_offset(639, 288, ((4.0 * 0.01) * 0.01));s.store_offset_scaled_div(278, 288, 639, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(288, 288, 0.5, 639, 0.5, (1e-10 * 0.01));s.b[836] = (s.v[288] < 0.0);s.store_scalar(836, if s.b[836] { 1.0 } else { 0.0 });
        if s.b[836] {s.store_scalar(288, 0.0);s.store_scalar(278, 0.0);}
        s.copy_ad(89, 288);s.store_offset_div(279, 51, 89, 1e-50);s.store_powf(280, 279, (s.v[481] - 1.0));s.store_offset_mul(281, 280, 279, 1.0);s.store_powf(282, 281, ((1.0 / s.v[481]) - 1.0));s.store_mul(284, 282, 281);s.store_div(395, 51, 284);s.copy_ad(51, 395);s.b[837] = (s.v[51] < 0.0);s.store_scalar(837, if s.b[837] { 1.0 } else { 0.0 });
        if s.b[837] {s.copy_ad(57, 56);s.store_sub(59, 57, 56);s.copy_ad(308, 57);s.copy_ad(309, 306);s.copy_ad(584, 522);s.copy_ad(310, 307);s.store_scalar(379, 1.0);}
        s.b[838] = ((s.v[33] >= 1.0) || (s.v[86] < 1e-12));s.store_scalar(838, if s.b[838] { 1.0 } else { 0.0 });
        if ((!s.b[837]) && s.b[838]) {s.store_scalar(308, s.v[698]);s.store_scalar(309, s.v[699]);s.store_offset(310, 440, s.v[700]);}
        if ((!s.b[837]) && (!s.b[838])) {
            if ((s.v[58] - s.v[305]) >= 0.0) {
                s.store_sub(61, 58, 305);
            } else {
                s.store_scalar(61, 0.0);
            }
        }
        if ((!s.b[837]) && (!s.b[838])) {s.store_offset_sub_scaled_inputs_indices(638, 61, (1.0 + (0.3 * 0.5)), 51, 1.0, (-0.03));s.store_scale(639, 61, ((1.0 + (0.3 * 0.5)) * (4.0 * 0.03)));}
        if ((!s.b[837]) && (!s.b[838])) {
            if (s.v[639] > 0.0) {
            } else {
                s.store_neg(639, 639);
            }
        }
        if ((!s.b[837]) && (!s.b[838])) {s.store_sqrt_square_add(639, 638, 639);s.store_offset_scaled_div(279, 638, 639, 0.5, 0.5);s.store_add_scaled_inputs3_indices(60, 61, (1.0 + (0.3 * 0.5)), 638, (-0.5), 639, (-0.5));}
        if ((!s.b[837]) && (!s.b[838])) {
            if (s.v[60] <= s.v[61]) {
            } else {
                s.copy_ad(60, 61);
            }
        }
        s.b[839] = (s.v[60] < 0.0);s.store_scalar(839, if s.b[839] { 1.0 } else { 0.0 });
        if (((!s.b[837]) && (!s.b[838])) && s.b[839]) {s.store_scalar(60, 0.0);}
        s.b[840] = (s.v[60] > s.v[51]);s.store_scalar(840, if s.b[840] { 1.0 } else { 0.0 });
        if ((((!s.b[837]) && (!s.b[838])) && (!s.b[839])) && s.b[840]) {s.copy_ad(60, 51);}
        if ((!s.b[837]) && (!s.b[838])) {s.copy_ad(59, 60);s.store_add(57, 305, 59);s.store_scalar(290, (1e-12 / 2.0));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_27(
        s: &mut ReactiveScratch,
    ) {
        s.b[841] = (s.v[57] < s.v[290]);s.store_scalar(841, if s.b[841] { 1.0 } else { 0.0 });
        if (((!s.b[837]) && (!s.b[838])) && s.b[841]) {s.copy_ad(57, 290);}
        if ((!s.b[837]) && (!s.b[838])) {s.copy_ad(308, 57);}
        if ((!s.b[837]) && (!s.b[838])) {
            if (s.v[292] == (-1.0)) {
                s.copy_ad(308, 305);
            } else {
                s.copy_ad(308, 57);
            }
        }
        if ((!s.b[837]) && (!s.b[838])) {s.store_scaled_square(278, 439, (s.v[293] * s.v[293]));}
        s.b[842] = (s.v[308] < s.v[329]);s.store_scalar(842, if s.b[842] { 1.0 } else { 0.0 });
        if (((!s.b[837]) && (!s.b[838])) && s.b[842]) {s.store_neg(279, 440);s.store_add_scaled_inputs3_mixed_aai(280, A::square(A::add_scaled_product(s.ad_value(279), 2.0, s.ad_value(278), s.ad_value(120), 1.0)), 1.0, A::square(s.ad_value(279)), (-4.0), 278, (-4.0));}
        if (((!s.b[837]) && (!s.b[838])) && s.b[842]) {
            if (s.v[280] >= (10.0 * 2.220446049250313e-16)) {
            } else {
                s.store_scalar(280, (10.0 * 2.220446049250313e-16));
            }
        }
        if (((!s.b[837]) && (!s.b[838])) && s.b[842]) {s.store_scaled_sub_ad(324, A::add_scaled_product(s.ad_value(279), 2.0, s.ad_value(278), s.ad_value(120), 1.0), A::sqrt(s.ad_value(280)), 0.5);s.store_div_ad(325, A::ln(A::div_scaled_product_by_product(s.ad_value(279), s.ad_value(279), 1.0, s.ad_value(278), s.ad_value(143), 1.0)), A::add(s.ad_value(120), A::div_from_scalar(2.0, s.ad_value(279))));}
        s.b[843] = (s.v[324] < s.v[326]);s.store_scalar(843, if s.b[843] { 1.0 } else { 0.0 });
        if ((((!s.b[837]) && (!s.b[838])) && s.b[842]) && s.b[843]) {s.copy_ad(310, 324);}
        if ((((!s.b[837]) && (!s.b[838])) && s.b[842]) && (!s.b[843])) {s.store_offset_sub(638, 325, 324, (-0.0008));s.store_scale(639, 325, (4.0 * 0.0008));}
        if ((((!s.b[837]) && (!s.b[838])) && s.b[842]) && (!s.b[843])) {
            if (s.v[639] > 0.0) {
            } else {
                s.store_neg(639, 639);
            }
        }
        if ((((!s.b[837]) && (!s.b[838])) && s.b[842]) && (!s.b[843])) {s.store_sqrt_square_add(639, 638, 639);s.store_offset_scaled_div(279, 638, 639, 0.5, 0.5);s.store_add_scaled_inputs3_indices(310, 325, 1.0, 638, (-0.5), 639, (-0.5));}
        if (((!s.b[837]) && (!s.b[838])) && (!s.b[842])) {s.store_add_scaled_inputs3_indices(279, 440, (-1.0), 308, (-(-1.0)), 296, (-(-(0.5 * s.v[536]))));s.store_add_scaled_inputs3_mixed_aai(280, A::square(A::add_scaled_product(s.ad_value(279), 2.0, s.ad_value(278), s.ad_value(120), 1.0)), 1.0, A::square(s.ad_value(279)), (-4.0), 278, (-4.0));}
        if (((!s.b[837]) && (!s.b[838])) && (!s.b[842])) {
            if (s.v[280] >= (10.0 * 2.220446049250313e-16)) {
            } else {
                s.store_scalar(280, (10.0 * 2.220446049250313e-16));
            }
        }
        if (((!s.b[837]) && (!s.b[838])) && (!s.b[842])) {s.store_scaled_sub_ad(324, A::add_scaled_product(s.ad_value(279), 2.0, s.ad_value(278), s.ad_value(120), 1.0), A::sqrt(s.ad_value(280)), 0.5);s.store_div_ad(325, A::ln(A::div_scaled_product_by_product(s.ad_value(279), s.ad_value(279), 1.0, s.ad_value(278), s.ad_value(143), 1.0)), A::add(s.ad_value(120), A::div_from_scalar(2.0, s.ad_value(279))));}
        s.b[844] = (s.v[324] < s.v[326]);s.store_scalar(844, if s.b[844] { 1.0 } else { 0.0 });
        if ((((!s.b[837]) && (!s.b[838])) && (!s.b[842])) && s.b[844]) {s.copy_ad(310, 324);}
        if ((((!s.b[837]) && (!s.b[838])) && (!s.b[842])) && (!s.b[844])) {s.store_offset_sub(638, 325, 324, (-0.0008));s.store_scale(639, 325, (4.0 * 0.0008));}
        if ((((!s.b[837]) && (!s.b[838])) && (!s.b[842])) && (!s.b[844])) {
            if (s.v[639] > 0.0) {
            } else {
                s.store_neg(639, 639);
            }
        }
        if ((((!s.b[837]) && (!s.b[838])) && (!s.b[842])) && (!s.b[844])) {s.store_sqrt_square_add(639, 638, 639);s.store_offset_scaled_div(279, 638, 639, 0.5, 0.5);s.store_add_scaled_inputs3_indices(310, 325, 1.0, 638, (-0.5), 639, (-0.5));}
        s.b[845] = ((s.v[308] < s.v[329]) && (0.0 != 0.0));s.store_scalar(845, if s.b[845] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_28(
        s: &mut ReactiveScratch,
    ) {
        if (((!s.b[837]) && (!s.b[838])) && s.b[845]) {s.store_scalar(63, 0.0);}
        let mut t3: usize = 0;
        while {
            let t2: f64 = if ((((!s.b[837]) && (!s.b[838])) && s.b[845]) && (s.v[63] < s.v[29])) { 1.0 } else { 0.0 };
            t2 != 0.0
        } {
            t3 += 1;assert!(t3 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((!s.b[837]) && (!s.b[838])) && s.b[845]) {s.store_mul(280, 120, 310);s.store_exp_neg_input(281, 280);}
            s.b[846] = (s.v[310] > 1e-8);s.store_scalar(846, if s.b[846] { 1.0 } else { 0.0 });
            if ((((!s.b[837]) && (!s.b[838])) && s.b[845]) && s.b[846]) {s.store_exp_mul(278, 120, 310);s.store_mul_scaled_sqrt_ad_rhs(282, 439, -1.0, A::add_scaled_offset_product_rhs(A::offset(A::add(s.ad_value(281), s.ad_value(280)), (-1.0)), 1.0, s.ad_value(143), s.ad_value(278), (-1.0), 1.0));s.store_mul_div_scaled_inputs_mixed_aii(283, A::add_scaled_sub_value_product(1.0, s.ad_value(281), 1.0, s.ad_value(143), s.ad_value(278), 1.0), 438, 1.0, 282, 1.0);}
            s.b[847] = (s.v[310] < (-1e-8));s.store_scalar(847, if s.b[847] { 1.0 } else { 0.0 });
            if (((((!s.b[837]) && (!s.b[838])) && s.b[845]) && (!s.b[846])) && s.b[847]) {s.store_mul_sqrt_mixed_ia(282, 439, A::offset(A::add(s.ad_value(281), s.ad_value(280)), (-1.0)));s.store_mul_scale_offset_mixed_ai(283, A::div(s.ad_value(438), s.ad_value(282)), 281, -1.0, 1.0);}
            if (((((!s.b[837]) && (!s.b[838])) && s.b[845]) && (!s.b[846])) && (!s.b[847])) {s.store_mul_ad_affine_product_lhs(282, A::sqrt(A::div(s.ad_value(438), s.ad_value(120))), s.ad_value(120), -1.0, 0.0, 310);s.store_neg_ad(283, A::sqrt(A::mul(s.ad_value(438), s.ad_value(120))));}
            if (((!s.b[837]) && (!s.b[838])) && s.b[845]) {s.store_sqrt_square_offset(639, 282, ((4.0 * 1e-6) * 1e-6));s.store_offset_scaled_div(285, 282, 639, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(284, 282, 0.5, 639, 0.5, (1e-10 * 1e-6));}
            s.b[848] = (s.v[284] < 0.0);s.store_scalar(848, if s.b[848] { 1.0 } else { 0.0 });
            if ((((!s.b[837]) && (!s.b[838])) && s.b[845]) && s.b[848]) {s.store_scalar(284, 0.0);s.store_scalar(285, 0.0);}
            if (((!s.b[837]) && (!s.b[838])) && s.b[845]) {s.store_offset_sub_scaled_inputs_indices(638, 296, -1.0, 284, 1.0, (-1e-9));s.store_scale(639, 296, (-(4.0 * 1e-9)));}
            if (((!s.b[837]) && (!s.b[838])) && s.b[845]) {
                if (s.v[639] > 0.0) {
                } else {
                    s.store_neg(639, 639);
                }
            }
            if (((!s.b[837]) && (!s.b[838])) && s.b[845]) {s.store_sqrt_square_add(639, 638, 639);s.store_offset_scaled_div(286, 638, 639, 0.5, 0.5);s.store_add_scaled_inputs3_indices(284, 296, -1.0, 638, (-0.5), 639, (-0.5));s.store_mul3_lhs(285, 285, 283, 286);s.store_div_scaled_inputs_mixed_ai(332, A::square(s.ad_value(284)), ((0.5 * 9662367879.197212) * 6.241449993689894e18), 471, 1.0);s.store_div_scaled_product_indices(333, 332, 285, 2.0, 284, 1.0);s.store_sub_mixed_ia(284, 310, A::div_scaled_inputs4(s.ad_value(282), 1.0 / (s.v[294]), s.ad_value(310), (-1.0), s.ad_value(440), -1.0, s.ad_value(332), 1.0, A::add(A::scale_offset(s.ad_value(283), 1.0 / (s.v[294]), (-1.0)), s.ad_value(333)), 1.0));}
            s.b[849] = ((((s.v[284] - s.v[310])) as f64).abs() < 1e-12);s.store_scalar(849, if s.b[849] { 1.0 } else { 0.0 });
            if ((((!s.b[837]) && (!s.b[838])) && s.b[845]) && s.b[849]) {s.store_scalar(63, s.v[29]);}
            if (((!s.b[837]) && (!s.b[838])) && s.b[845]) {s.copy_ad(310, 284);s.copy_ad(314, 282);s.store_primal_offset(63, 63, 1.0);}
        }
        if (((!s.b[837]) && (!s.b[838])) && s.b[845]) {s.store_add(310, 440, 310);s.store_sub_scaled_inputs(309, 310, 1.0, 314, 1.0 / (s.v[294]));}
        if (((!s.b[837]) && (!s.b[838])) && (!s.b[845])) {s.store_scalar(63, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_29(
        s: &mut ReactiveScratch,
    ) {
        let mut t5: usize = 0;
        while {
            let t4: f64 = if ((((!s.b[837]) && (!s.b[838])) && (!s.b[845])) && (s.v[63] < s.v[29])) { 1.0 } else { 0.0 };
            t4 != 0.0
        } {
            t5 += 1;assert!(t5 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((!s.b[837]) && (!s.b[838])) && (!s.b[845])) {s.copy_ad(279, 439);s.store_mul(280, 120, 310);s.store_exp_neg_input(281, 280);}
            s.b[850] = (s.v[310] > 1e-8);s.store_scalar(850, if s.b[850] { 1.0 } else { 0.0 });
            if ((((!s.b[837]) && (!s.b[838])) && (!s.b[845])) && s.b[850]) {s.store_exp_mul(278, 120, 310);s.store_mul_scaled_sqrt_ad_rhs(282, 279, -1.0, A::add_scaled_offset_product_rhs(A::offset(A::add(s.ad_value(281), s.ad_value(280)), (-1.0)), 1.0, s.ad_value(143), s.ad_value(278), (-1.0), 1.0));s.store_mul_div_scaled_inputs_mixed_aii(283, A::add_scaled_sub_value_product(1.0, s.ad_value(281), 1.0, s.ad_value(143), s.ad_value(278), 1.0), 438, 1.0, 282, 1.0);}
            s.b[851] = (s.v[310] < (-1e-8));s.store_scalar(851, if s.b[851] { 1.0 } else { 0.0 });
            if (((((!s.b[837]) && (!s.b[838])) && (!s.b[845])) && (!s.b[850])) && s.b[851]) {s.store_mul_sqrt_mixed_ia(282, 279, A::offset(A::add(s.ad_value(281), s.ad_value(280)), (-1.0)));s.store_mul_scale_offset_mixed_ai(283, A::div(s.ad_value(438), s.ad_value(282)), 281, -1.0, 1.0);}
            if (((((!s.b[837]) && (!s.b[838])) && (!s.b[845])) && (!s.b[850])) && (!s.b[851])) {s.store_mul_ad_affine_product_lhs(282, A::sqrt(A::div(s.ad_value(438), s.ad_value(120))), s.ad_value(120), -1.0, 0.0, 310);s.store_neg_ad(283, A::sqrt(A::mul(s.ad_value(438), s.ad_value(120))));}
            if (((!s.b[837]) && (!s.b[838])) && (!s.b[845])) {s.store_sqrt_square_offset(639, 282, ((4.0 * 1e-6) * 1e-6));s.store_offset_scaled_div(285, 282, 639, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(284, 282, 0.5, 639, 0.5, (1e-10 * 1e-6));}
            s.b[852] = (s.v[284] < 0.0);s.store_scalar(852, if s.b[852] { 1.0 } else { 0.0 });
            if ((((!s.b[837]) && (!s.b[838])) && (!s.b[845])) && s.b[852]) {s.store_scalar(284, 0.0);s.store_scalar(285, 0.0);}
            if (((!s.b[837]) && (!s.b[838])) && (!s.b[845])) {s.store_offset_sub_scaled_inputs_indices(638, 296, -1.0, 284, 1.0, (-1e-9));s.store_scale(639, 296, (-(4.0 * 1e-9)));}
            if (((!s.b[837]) && (!s.b[838])) && (!s.b[845])) {
                if (s.v[639] > 0.0) {
                } else {
                    s.store_neg(639, 639);
                }
            }
            if (((!s.b[837]) && (!s.b[838])) && (!s.b[845])) {s.store_sqrt_square_add(639, 638, 639);s.store_offset_scaled_div(286, 638, 639, 0.5, 0.5);s.store_add_scaled_inputs3_indices(284, 296, -1.0, 638, (-0.5), 639, (-0.5));s.store_mul3_lhs(285, 285, 283, 286);s.store_div_scaled_inputs_mixed_ai(332, A::square(s.ad_value(284)), ((0.5 * 9662367879.197212) * 6.241449993689894e18), 471, 1.0);s.store_div_scaled_product_indices(333, 332, 285, 2.0, 284, 1.0);s.store_sub_div_rhs_ad(284, 310, A::add(A::sub(A::add(A::add_scaled_inputs3(s.ad_value(308), 1.0, s.ad_value(310), (-1.0), s.ad_value(282), 1.0 / (s.v[294])), A::add_scaled_inputs(s.ad_value(282), s.v[536], s.ad_value(296), (0.5 * s.v[536]))), s.ad_value(440)), s.ad_value(332)), A::add_scaled_inputs3_offset(s.ad_value(283), 1.0 / (s.v[294]), s.ad_value(283), s.v[536], s.ad_value(333), 1.0, (-1.0)));}
            s.b[853] = ((((s.v[284] - s.v[310])) as f64).abs() < 1e-12);s.store_scalar(853, if s.b[853] { 1.0 } else { 0.0 });
            if ((((!s.b[837]) && (!s.b[838])) && (!s.b[845])) && s.b[853]) {s.store_scalar(63, s.v[29]);}
            if (((!s.b[837]) && (!s.b[838])) && (!s.b[845])) {s.copy_ad(310, 284);s.copy_ad(314, 282);s.store_primal_offset(63, 63, 1.0);}
        }
        if (((!s.b[837]) && (!s.b[838])) && (!s.b[845])) {s.store_add(310, 440, 310);s.store_sub_scaled_inputs(309, 310, 1.0, 314, 1.0 / (s.v[294]));}
        if ((!s.b[837]) && (!s.b[838])) {s.copy_ad(584, 309);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_30(
        s: &mut ReactiveScratch,
    ) {
        s.b[854] = (s.v[86] < 1e-12);s.store_scalar(854, if s.b[854] { 1.0 } else { 0.0 });
        if s.b[854] {s.copy_ad(302, 305);s.copy_ad(303, 306);s.copy_ad(304, 307);s.copy_ad(581, 522);}
        if (!s.b[854]) {s.copy_ad(302, 308);s.copy_ad(303, 309);s.store_sub(304, 310, 440);}
        if (!s.b[854]) {
            if (s.v[303] < s.v[302]) {
                s.copy_ad(581, 303);
            } else {
                s.copy_ad(581, 302);
            }
        }
        s.b[379] = (s.v[292] < 0.0);s.store_scalar(379, if s.b[379] { 1.0 } else { 0.0 });s.copy_ad(308, 302);s.copy_ad(309, 303);s.copy_ad(310, 304);s.copy_ad(584, 581);s.store_scalar(63, 1.0);
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_31(
        s: &mut ReactiveScratch,
    ) {
        let mut t7: usize = 0;
        while {
            let t6: f64 = if s.v[63] <= s.v[29] { 1.0 } else { 0.0 };
            t6 != 0.0
        } {
            t7 += 1;assert!(t7 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");s.copy_ad(279, 310);s.store_mul(297, 120, 279);s.store_exp_neg_input(278, 297);s.b[855] = (s.v[279] < (-1e-8));s.store_scalar(855, if s.b[855] { 1.0 } else { 0.0 });
            if s.b[855] {s.store_exp_mul(280, 120, 310);s.store_mul_sqrt_mixed_ia(314, 439, A::add_scaled_offset_product_rhs(A::offset(A::add(s.ad_value(278), s.ad_value(297)), (-1.0)), 1.0, s.ad_value(143), s.ad_value(280), (-1.0), 1.0));s.store_div_scaled_product_mixed_iai(344, 438, A::add_scaled_sub_value_product(1.0, s.ad_value(278), 1.0, s.ad_value(143), s.ad_value(280), 1.0), 1.0, 314, 1.0);}
            s.b[856] = (s.v[279] > (1e-8 / 10.0));s.store_scalar(856, if s.b[856] { 1.0 } else { 0.0 });
            if ((!s.b[855]) && s.b[856]) {s.store_exp_mul(280, 120, 310);s.store_mul_scaled_sqrt_ad_rhs(314, 439, -1.0, A::add_scaled_offset_product_rhs(A::offset(A::add(s.ad_value(278), s.ad_value(297)), (-1.0)), 1.0, s.ad_value(143), A::sub(s.ad_value(280), s.ad_value(297)), (-1.0), 1.0));s.store_div_scaled_product_mixed_iai(344, 438, A::add_scaled_sub_value_product(1.0, s.ad_value(278), 1.0, s.ad_value(143), A::offset(s.ad_value(280), (-1.0)), 1.0), 1.0, 314, 1.0);}
            if ((!s.b[855]) && (!s.b[856])) {s.store_scaled_mul(314, 439, 297, (-1.0 / (((2.0) as f64).sqrt())));s.store_scaled_mul(344, 439, 120, (-1.0 / (((2.0) as f64).sqrt())));}
            s.store_add_scaled_inputs4_indices(309, 310, 1.0, 314, (-1.0 / (s.v[294])), 50, 1.0, 298, 1.0);s.store_sub_from_scalar_scaled_input(582, 1.0, 344, 1.0 / (s.v[294]));s.store_sub(279, 308, 584);s.store_mul(297, 120, 279);s.b[857] = ((-s.v[297]) >= 80.0);s.store_scalar(857, if s.b[857] { 1.0 } else { 0.0 });
            if s.b[857] {s.store_scaled_offset_ad(278, A::sub_from_scalar(1.0, s.ad_value(297)), (-80.0), 5.540622384e34);s.store_scalar(284, 5.540622384e34);}
            if (!s.b[857]) {s.store_exp_neg_input(278, 297);s.copy_ad(284, 278);}
            s.b[858] = (s.v[279] < (-1e-8));s.store_scalar(858, if s.b[858] { 1.0 } else { 0.0 });
            if s.b[858] {s.store_sqrt_offset_ad(280, A::add(s.ad_value(278), s.ad_value(297)), (-1.0));s.store_mul(576, 141, 280);s.store_div_scaled_product3_mixed_iiai(577, 141, 120, A::sub_from_scalar(1.0, s.ad_value(284)), 1.0, 280, 2.0);s.store_neg(578, 577);s.store_scalar(313, 0.0);s.store_scalar(579, 0.0);s.store_scalar(580, 0.0);}
            s.b[859] = (s.v[279] > 1e-8);s.store_scalar(859, if s.b[859] { 1.0 } else { 0.0 });
            if ((!s.b[858]) && s.b[859]) {s.store_sqrt_offset_ad(280, A::add(s.ad_value(278), s.ad_value(297)), (-1.0));s.store_mul_scale_offset_indices(576, 280, 141, -1.0, 0.0);s.store_div_scaled_product3_mixed_iiai(577, 141, 120, A::sub_from_scalar(1.0, s.ad_value(284)), -1.0, 280, 2.0);s.store_neg(578, 577);s.store_exp(278, 297);s.store_exp_ad(281, A::mul(s.ad_value(120), A::sub(s.ad_value(584), s.ad_value(51))));s.store_sqrt_add_ad(282, A::div_scaled_product(s.ad_value(576), s.ad_value(576), 1.0, A::square(s.ad_value(141)), 1.0), A::mul3_scaled_output(s.ad_value(142), s.ad_value(281), A::offset(A::sub(s.ad_value(278), s.ad_value(297)), (-1.0)), 2.0));s.store_div_scaled_inputs_mixed_ai(537, A::add_scaled_offset_product_rhs(A::div_scaled_product(s.ad_value(576), s.ad_value(577), 2.0, A::square(s.ad_value(141)), 1.0), 1.0, A::mul3_scaled_output(s.ad_value(120), s.ad_value(142), s.ad_value(281), 2.0), s.ad_value(278), (-1.0), 1.0), 1.0, 282, 2.0);s.store_div_scaled_add_product_mixed_aaii(538, A::div_scaled_product(s.ad_value(576), s.ad_value(578), 2.0, A::square(s.ad_value(141)), 1.0), 1.0, A::mul3_scaled_output(s.ad_value(120), s.ad_value(142), s.ad_value(281), 2.0), 297, (-1.0), 282, 2.0);s.store_add_scaled_product_indices(313, 576, (-1.0), 141, 282, -1.0);s.store_add_scaled_product_indices(579, 577, (-1.0), 141, 537, -1.0);s.store_add_scaled_product_indices(580, 578, (-1.0), 141, 538, -1.0);}
            if ((!s.b[858]) && (!s.b[859])) {s.store_scaled_mul(576, 141, 297, (-1.0 / (((2.0) as f64).sqrt())));s.store_scaled_mul(577, 141, 120, (-1.0 / (((2.0) as f64).sqrt())));s.store_neg(578, 577);s.store_scalar(313, 0.0);s.store_scalar(579, 0.0);s.store_scalar(580, 0.0);}
            s.store_sub(279, 309, 584);s.store_mul(297, 120, 279);s.b[860] = ((-s.v[297]) >= 80.0);s.store_scalar(860, if s.b[860] { 1.0 } else { 0.0 });
            if s.b[860] {s.store_scaled_offset_ad(278, A::sub_from_scalar(1.0, s.ad_value(297)), (-80.0), 5.540622384e34);s.store_scalar(284, 5.540622384e34);}
            if (!s.b[860]) {s.store_exp_neg_input(278, 297);s.copy_ad(284, 278);}
            s.b[861] = (s.v[279] < (-1e-8));s.store_scalar(861, if s.b[861] { 1.0 } else { 0.0 });
            if s.b[861] {s.store_sqrt_offset_ad(280, A::add(s.ad_value(278), s.ad_value(297)), (-1.0));s.store_mul(585, 141, 280);s.store_div_scaled_product3_mixed_iiai(586, 141, 120, A::sub_from_scalar(1.0, s.ad_value(284)), 1.0, 280, 2.0);s.store_neg(587, 586);s.store_scalar(588, 0.0);s.store_scalar(589, 0.0);s.store_scalar(590, 0.0);}
            s.b[862] = (s.v[279] > 1e-8);s.store_scalar(862, if s.b[862] { 1.0 } else { 0.0 });
            if ((!s.b[861]) && s.b[862]) {s.store_sqrt_offset_ad(280, A::add(s.ad_value(278), s.ad_value(297)), (-1.0));s.store_mul_scale_offset_indices(585, 280, 141, -1.0, 0.0);s.store_div_scaled_product3_mixed_iiai(586, 141, 120, A::sub_from_scalar(1.0, s.ad_value(284)), -1.0, 280, 2.0);s.store_neg(587, 586);s.store_exp(278, 297);s.store_exp_ad(281, A::mul(s.ad_value(120), A::sub(s.ad_value(584), s.ad_value(51))));s.store_sqrt_add_ad(282, A::div_scaled_product(s.ad_value(585), s.ad_value(585), 1.0, A::square(s.ad_value(141)), 1.0), A::mul3_scaled_output(s.ad_value(142), s.ad_value(281), A::offset(A::sub(s.ad_value(278), s.ad_value(297)), (-1.0)), 2.0));s.store_div_scaled_inputs_mixed_ai(539, A::add_scaled_offset_product_rhs(A::div_scaled_product(s.ad_value(585), s.ad_value(586), 2.0, A::square(s.ad_value(141)), 1.0), 1.0, A::mul3_scaled_output(s.ad_value(120), s.ad_value(142), s.ad_value(281), 2.0), s.ad_value(278), (-1.0), 1.0), 1.0, 282, 2.0);s.store_div_scaled_add_product_mixed_aaii(538, A::div_scaled_product(s.ad_value(585), s.ad_value(587), 2.0, A::square(s.ad_value(141)), 1.0), 1.0, A::mul3_scaled_output(s.ad_value(120), s.ad_value(142), s.ad_value(281), 2.0), 297, (-1.0), 282, 2.0);s.store_add_scaled_product_indices(588, 585, (-1.0), 141, 282, -1.0);s.store_add_scaled_product_indices(589, 586, (-1.0), 141, 539, -1.0);s.store_add_scaled_product_indices(590, 587, (-1.0), 141, 538, -1.0);}
            if ((!s.b[861]) && (!s.b[862])) {s.store_scaled_mul(585, 141, 297, (-1.0 / (((2.0) as f64).sqrt())));s.store_scaled_mul(586, 141, 120, (-1.0 / (((2.0) as f64).sqrt())));s.store_neg(587, 586);s.store_scalar(588, 0.0);s.store_scalar(589, 0.0);s.store_scalar(590, 0.0);}
            s.b[863] = s.b[379];s.store_scalar(863, if s.b[863] { 1.0 } else { 0.0 });
            if s.b[863] {s.store_scalar(574, s.v[63]);s.store_scalar(63, s.v[29]);}
            if (!s.b[863]) {s.store_add_scaled_inputs3_mixed_iia(346, 308, 1.0, 76, (-1.0), A::div(A::add(A::add(A::add_scaled_inputs4(s.ad_value(314), 1.0, s.ad_value(313), 1.0, s.ad_value(576), 1.0, s.ad_value(588), 1.0), s.ad_value(585)), s.ad_value(337)), s.ad_value(270)), -1.0);s.store_sub_from_scalar_ad(347, 1.0, A::div_scaled_inputs2(s.ad_value(579), 1.0, s.ad_value(577), 1.0, s.ad_value(270), 1.0));s.store_div_scaled_inputs_mixed_ai(348, A::add_scaled_inputs4(s.ad_value(580), 1.0, s.ad_value(578), 1.0, s.ad_value(590), 1.0, s.ad_value(587), 1.0), -1.0, 270, 1.0);s.store_div_scaled_inputs_mixed_ai(349, A::add_scaled_product(s.ad_value(344), 1.0, A::add(s.ad_value(589), s.ad_value(586)), s.ad_value(582), 1.0), -1.0, 270, 1.0);}
            s.b[864] = (s.v[314] <= s.v[599]);s.store_scalar(864, if s.b[864] { 1.0 } else { 0.0 });
            if ((!s.b[863]) && s.b[864]) {s.store_sqrt_mul_ad(279, s.ad_value(296), A::add_scaled_inputs(s.ad_value(314), 2.0, s.ad_value(296), 1.0));s.store_div_scaled_product_indices(604, 296, 344, 1.0, 279, 1.0);}
            s.b[865] = (s.v[314] <= s.v[603]);s.store_scalar(865, if s.b[865] { 1.0 } else { 0.0 });
            if (((!s.b[863]) && (!s.b[864])) && s.b[865]) {s.store_mul3_ad(279, A::mul3(s.ad_value(601), A::sub(s.ad_value(314), s.ad_value(603)), A::sub(s.ad_value(314), s.ad_value(603))), A::sub(s.ad_value(314), s.ad_value(603)), A::sub(s.ad_value(314), s.ad_value(602)));s.store_mul_ad_product_lhs(604, A::mul3(s.ad_value(601), A::sub(s.ad_value(314), s.ad_value(603)), A::sub(s.ad_value(314), s.ad_value(603))), A::add_scaled_inputs4(s.ad_value(314), 3.0, s.ad_value(602), (-3.0), s.ad_value(314), 1.0, s.ad_value(603), (-1.0)), 344);}
            if (((!s.b[863]) && (!s.b[864])) && (!s.b[865])) {s.store_scalar(279, 0.0);s.store_scalar(604, 0.0);}
            if (!s.b[863]) {s.store_div_scaled_inputs_indices(281, 316, (-s.v[650]), 296, 1.0);s.store_div_from_scalar_offset_ad(280, 1.0, A::exp_scaled_input(s.ad_value(281), -1.0), 1.0);s.store_mul_square_exp_scaled_input(278, 280, 281, -1.0);s.store_mul(280, 280, 600);s.store_neg_add(279, 296, 280);s.store_scalar(604, 0.0);s.store_scaled_add(350, 576, 279, 1.0 / (s.v[535]));s.store_scale(351, 577, 1.0 / (s.v[535]));s.store_scale(352, 578, 1.0 / (s.v[535]));s.store_scale(353, 604, 1.0 / (s.v[535]));s.store_div_scaled_inputs_indices(281, 316, (-s.v[651]), 296, 1.0);s.store_div_from_scalar_offset_ad(280, 1.0, A::exp_scaled_input(s.ad_value(281), -1.0), 1.0);s.store_mul_square_exp_scaled_input(278, 280, 281, -1.0);s.store_mul(280, 280, 600);s.store_scalar(605, 0.0);s.store_scaled_add(354, 585, 280, 1.0 / (s.v[535]));s.store_scale(355, 587, 1.0 / (s.v[535]));s.store_add_scaled_product_indices(356, 605, 1.0 / (s.v[535]), 586, 582, 1.0 / (s.v[535]));s.store_add_scaled_inputs4(357, A::mul3(s.ad_value(347), s.ad_value(352), s.ad_value(356)), 1.0, A::mul3(s.ad_value(347), s.ad_value(353), s.ad_value(355)), (-1.0), A::mul3(s.ad_value(348), s.ad_value(351), s.ad_value(356)), -1.0, A::mul3(s.ad_value(349), s.ad_value(351), s.ad_value(355)), 1.0);}
            s.b[866] = (s.v[357] > 0.0);s.store_scalar(866, if s.b[866] { 1.0 } else { 0.0 });
            if ((!s.b[863]) && s.b[866]) {s.store_div_from_scalar_offset_input(358, 1.0, 357, 1e-50);}
            if ((!s.b[863]) && (!s.b[866])) {s.store_div_from_scalar_offset_input(358, 1.0, 357, (-1e-50));}
            if (!s.b[863]) {s.store_add_scaled_products_indices(359, 352, 356, 1.0, 353, 355, (-1.0));s.store_add_scaled_products_indices(360, 349, 355, 1.0, 348, 356, (-1.0));s.store_add_scaled_products_indices(361, 348, 353, 1.0, 349, 352, (-1.0));s.store_mul_scale_offset_indices(362, 356, 351, -1.0, 0.0);s.store_mul(363, 347, 356);s.store_add_scaled_products_indices(364, 349, 351, 1.0, 347, 353, (-1.0));s.store_mul(365, 351, 355);s.store_mul_scale_offset_indices(366, 355, 347, -1.0, 0.0);s.store_add_scaled_products_indices(367, 347, 352, 1.0, 348, 351, (-1.0));s.store_mul_add_scaled_products3_indices_rhs(368, 358, 359, 346, -1.0, 360, 350, -1.0, 361, 354, -1.0);s.store_mul_add_scaled_products3_indices_rhs(369, 358, 362, 346, -1.0, 363, 350, -1.0, 364, 354, -1.0);s.store_mul_add_scaled_products3_indices_rhs(370, 358, 365, 346, -1.0, 366, 350, -1.0, 367, 354, -1.0);s.store_abs(279, 368);}
            s.b[867] = (s.v[279] < ((s.v[369]) as f64).abs());s.store_scalar(867, if s.b[867] { 1.0 } else { 0.0 });
            if ((!s.b[863]) && s.b[867]) {s.store_abs(279, 369);}
            s.b[868] = (s.v[279] < ((s.v[370]) as f64).abs());s.store_scalar(868, if s.b[868] { 1.0 } else { 0.0 });
            if ((!s.b[863]) && s.b[868]) {s.store_abs(279, 370);}
            if (!s.b[863]) {s.store_scalar(606, 1.0);}
            s.b[869] = (s.v[63] > 80.0);s.store_scalar(869, if s.b[869] { 1.0 } else { 0.0 });
            if ((!s.b[863]) && s.b[869]) {s.store_scalar(606, 25.0);}
            s.b[870] = (s.v[63] > 40.0);s.store_scalar(870, if s.b[870] { 1.0 } else { 0.0 });
            if (((!s.b[863]) && (!s.b[869])) && s.b[870]) {s.store_scalar(606, 25.0);}
            s.b[871] = (s.v[63] > 20.0);s.store_scalar(871, if s.b[871] { 1.0 } else { 0.0 });
            if ((((!s.b[863]) && (!s.b[869])) && (!s.b[870])) && s.b[871]) {s.store_scalar(606, 25.0);}
            s.b[872] = (s.v[63] > 10.0);s.store_scalar(872, if s.b[872] { 1.0 } else { 0.0 });
            if (((((!s.b[863]) && (!s.b[869])) && (!s.b[870])) && (!s.b[871])) && s.b[872]) {s.store_scalar(606, 5.0);}
            s.b[873] = (s.v[279] > (0.1 / s.v[606]));s.store_scalar(873, if s.b[873] { 1.0 } else { 0.0 });
            if ((!s.b[863]) && s.b[873]) {s.store_mul_mixed_ia(368, 368, A::div_scalar_by_product(0.1, s.ad_value(606), s.ad_value(279), 1.0));s.store_mul_mixed_ia(369, 369, A::div_scalar_by_product(0.1, s.ad_value(606), s.ad_value(279), 1.0));s.store_mul_mixed_ia(370, 370, A::div_scalar_by_product(0.1, s.ad_value(606), s.ad_value(279), 1.0));}
            if (!s.b[863]) {s.store_add(308, 308, 368);s.store_add(584, 584, 369);s.store_add(310, 310, 370);s.store_primal_scale(607, 606, 1e-12);}
            s.b[874] = (s.v[279] < s.v[607]);s.store_scalar(874, if s.b[874] { 1.0 } else { 0.0 });
            if ((!s.b[863]) && s.b[874]) {s.store_scalar(379, 1.0);}
            s.store_primal_offset(63, 63, 1.0);
        }
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_32(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[875] = (s.v[574] > 0.0);s.store_scalar(875, if s.b[875] { 1.0 } else { 0.0 });
        if s.b[875] {s.copy_ad(63, 574);s.store_scalar(574, 0.0);}
        s.b[876] = (s.v[63] > s.v[29]);s.store_scalar(876, if s.b[876] { 1.0 } else { 0.0 });
        if s.b[876] {s.copy_ad(308, 302);s.copy_ad(309, 303);s.copy_ad(310, 304);s.copy_ad(584, 581);}
        s.copy_ad(57, 308);s.store_sub(59, 57, 56);s.copy_ad(51, 396);s.b[878] = ((s.v[292] <= (-1.0)) || (s.v[305] < 0.0));s.store_scalar(878, if s.b[878] { 1.0 } else { 0.0 });
        if s.b[878] {s.store_scalar(34, 1.0);}
        s.copy_ad(317, 305);s.copy_ad(318, 308);s.store_sub(59, 318, 317);s.copy_ad(322, 306);s.copy_ad(323, 309);s.store_sub(155, 323, 322);s.store_add_scaled_inputs3_mixed_iia(153, 313, 1.0, 311, (-1.0), A::mul3_scaled_output(s.ad_value(120), A::add(s.ad_value(313), s.ad_value(311)), A::sub(s.ad_value(318), s.ad_value(317)), 0.5), -1.0);s.store_add_scaled_inputs3_mixed_iia(154, 588, 1.0, 528, (-1.0), A::mul3_scaled_output(s.ad_value(120), A::add(s.ad_value(588), s.ad_value(528)), A::sub(s.ad_value(323), s.ad_value(322)), 0.5), -1.0);s.b[879] = ((s.v[153] < 0.0) || (s.v[51] == 0.0));s.store_scalar(879, if s.b[879] { 1.0 } else { 0.0 });
        if s.b[879] {s.store_scalar(153, 0.0);}
        s.b[880] = ((s.v[154] < 0.0) || (s.v[51] == 0.0));s.store_scalar(880, if s.b[880] { 1.0 } else { 0.0 });
        if s.b[880] {s.store_scalar(154, 0.0);}
        s.store_add(151, 153, 154);s.store_scaled_add(384, 576, 523, (-0.5));s.store_offset_sub(371, 308, 305, 1e-12);s.store_sub(373, 311, 313);s.b[881] = ((-s.v[373]) < 1e-18);s.store_scalar(881, if s.b[881] { 1.0 } else { 0.0 });
        if s.b[881] {s.store_scalar(373, 0.0);}
        s.store_offset_div_scaled_inputs_mixed_ia(372, 373, (-2.0), A::mul(A::mul3(s.ad_value(120), s.ad_value(270), s.ad_value(371)), s.ad_value(371)), 1.0, 1.0);s.store_sub_from_scalar_ad(85, 1.0, A::div_scaled_product(s.ad_value(372), s.ad_value(371), 1.0, s.ad_value(86), 1.0));s.b[882] = (s.v[85] <= 0.0);s.store_scalar(882, if s.b[882] { 1.0 } else { 0.0 });
        if s.b[882] {s.store_scalar(85, 0.0);}
        s.store_scaled_add(383, 311, 313, (-0.5));s.store_scaled_add(167, 528, 588, (-0.5));s.store_scalar(262, 0.0);s.b[883] = (s.v[34] == 0.0);s.store_scalar(883, if s.b[883] { 1.0 } else { 0.0 });s.b[884] = ((s.v[446] < (10.0 * 2.220446049250313e-16)) && (p.p178 < (10.0 * 2.220446049250313e-16)));s.store_scalar(884, if s.b[884] { 1.0 } else { 0.0 });
        if (s.b[883] && s.b[884]) {s.store_scalar(262, 0.0);s.copy_ad(260, 57);}
        s.b[885] = (s.v[260] > ((s.v[56] + s.v[71]) - (10.0 * 2.220446049250313e-16)));s.store_scalar(885, if s.b[885] { 1.0 } else { 0.0 });
        if ((s.b[883] && s.b[884]) && s.b[885]) {s.store_offset_add(260, 56, 71, (-(10.0 * 2.220446049250313e-16)));}
        if (s.b[883] && (!s.b[884])) {s.store_scalar(263, p.p227);s.store_div_from_scalar_ad(282, 1.034943e-10, A::add_scaled_product(A::div_scaled_inputs(s.ad_value(149), p.p178, s.ad_value(263), 1.0), 1.0, s.ad_value(446), s.ad_value(126), 1.0));s.store_add_scaled_inputs3_indices(260, 51, p.p176, 56, p.p176, 57, (1.0 - p.p176));}
        s.b[886] = (s.v[260] > ((s.v[56] + s.v[71]) - (10.0 * 2.220446049250313e-16)));s.store_scalar(886, if s.b[886] { 1.0 } else { 0.0 });
        if ((s.b[883] && (!s.b[884])) && s.b[886]) {s.store_offset_add(260, 56, 71, (-(10.0 * 2.220446049250313e-16)));}
        if (s.b[883] && (!s.b[884])) {s.store_sub(284, 260, 57);s.store_sqrt_square_offset(639, 284, ((4.0 * 0.001) * 0.001));s.store_offset_scaled_div(278, 284, 639, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(284, 284, 0.5, 639, 0.5, (1e-10 * 0.001));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_33(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[887] = (s.v[284] < 0.0);s.store_scalar(887, if s.b[887] { 1.0 } else { 0.0 });
        if ((s.b[883] && (!s.b[884])) && s.b[887]) {s.store_scalar(284, 0.0);s.store_scalar(278, 0.0);}
        if (s.b[883] && (!s.b[884])) {s.store_div_scaled_value_by_product_indices(283, 151, 1.0, 120, 149, 1.0);s.store_scale(288, 126, 9662367879.197212);s.store_scalar(279, 1000000000.0);s.store_div_scaled_inputs_product_mixed_iaiii(387, 283, 2.0, A::mul3_scaled_output(s.ad_value(288), s.ad_value(284), s.ad_value(282), 2.0), 1.0, 279, 282, 1.0, 123, 1.0);s.store_mul(285, 387, 282);s.store_add_scaled_product_indices(387, 279, 4.0, 288, 284, (2.0 * 4.0));s.store_mul3_lhs(286, 387, 282, 282);s.store_sqrt_square_add(287, 285, 286);s.store_scaled_sub(262, 287, 285, 0.5);s.copy_ad(279, 262);s.store_mul(262, 276, 279);}
        if s.b[883] {s.store_scale(262, 262, s.v[483]);}
        s.store_sub(386, 123, 262);s.b[888] = (s.v[386] < 1e-9);s.store_scalar(888, if s.b[888] { 1.0 } else { 0.0 });
        if s.b[888] {s.store_scalar(386, 1e-9);}
        s.store_mul_add_scaled_inputs_rhs_indices(91, 123, 383, (-s.v[513]), 167, (-s.v[513]));s.store_mul_scale_offset_mixed_ai(336, A::add(s.ad_value(312), s.ad_value(314)), 123, (0.5 * s.v[513]), 0.0);s.store_scaled_sub(279, 51, 59, 0.5);s.store_scale(638, 279, (2.0 * 1.0 / (p.p217)));s.store_offset_mul_offset_rhs_mixed_ia(639, 638, A::mul_offset_rhs(s.ad_value(638), A::mul_offset_rhs(s.ad_value(638), A::mul_offset_rhs(s.ad_value(638), A::mul(s.ad_value(638), A::scale_offset(s.ad_value(638), (1.0 / 5040.0), (1.0 / 720.0))), (1.0 / 120.0)), (1.0 / 24.0)), (1.0 / 6.0)), (1.0 / 2.0), 1.0);s.store_offset_mul_offset_rhs_mixed_ia(640, 638, A::mul_offset_rhs(s.ad_value(638), A::mul_offset_rhs(s.ad_value(638), A::mul(s.ad_value(638), A::scale_offset(s.ad_value(638), (1.0 / 840.0), (1.0 / 144.0))), (1.0 / 30.0)), (1.0 / 8.0)), (1.0 / 3.0), (1.0 / 2.0));s.store_div_from_scalar(75, p.p217, 639);s.store_div_scaled_inputs_square_rhs(280, 640, (-2.0), 639, 1.0);s.b[889] = (s.v[75] < (10.0 * 2.220446049250313e-16));s.store_scalar(889, if s.b[889] { 1.0 } else { 0.0 });
        if s.b[889] {s.store_scalar(75, (10.0 * 2.220446049250313e-16));}
        s.store_add(74, 56, 75);s.store_scalar(499, (1.034943e-10 / 100.0));s.store_scale(500, 313, 0.0001);s.store_scale(501, 588, 0.0001);s.store_scale(504, 531, 0.0001);s.store_scale(505, 585, 0.0001);s.store_scale(502, 383, 0.0001);s.store_scale(503, 167, 0.0001);s.store_scale(504, 531, 0.0001);s.store_scale(505, 585, 0.0001);s.store_scale(506, 384, 0.0001);s.store_scalar(507, (p.p229 * 100.0));s.store_scalar(591, ((p.p81 * (1.0 + (p.p82 / ((s.v[375]) as f64).powf(p.p83)))) / s.v[499]));s.store_scalar(592, ((p.p78 * (1.0 + (p.p79 / ((s.v[375]) as f64).powf(p.p80)))) / s.v[499]));s.store_sqrt_square_offset(639, 59, ((4.0 * 1e-6) * 1e-6));s.store_offset_scaled_div(278, 59, 639, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(598, 59, 0.5, 639, 0.5, (1e-10 * 1e-6));s.b[890] = (s.v[598] < 0.0);s.store_scalar(890, if s.b[890] { 1.0 } else { 0.0 });
        if s.b[890] {s.store_scalar(598, 0.0);s.store_scalar(278, 0.0);}
        s.store_offset_sqrt_ad(168, A::offset(A::square(s.ad_value(598)), p.p216), (-((p.p216) as f64).sqrt()));s.store_powf(168, 168, p.p85);s.store_offset_scaled(282, 168, p.p84, 1.0);s.store_scalar(497, (p.p299 * (1.0 + (p.p300 / ((s.v[375]) as f64).powf(p.p301)))));s.store_sub_scaled_inputs(288, 502, 1.0, 501, s.v[497]);s.store_add_scaled_inputs(283, 506, s.v[592], 288, s.v[591]);s.store_div(156, 283, 282);
        if (p.p32 != 0.0) {s.store_scaled_add(596, 306, 309, 0.5);s.store_scaled_add(597, 307, 310, 0.5);s.store_add_scaled_inputs3_indices(163, 596, (3.9 * 1.0 / ((11.7 * s.v[507]))), 597, ((-1.0) * (3.9 * 1.0 / ((11.7 * s.v[507])))), 440, (-(3.9 * 1.0 / ((11.7 * s.v[507])))));s.store_add(156, 156, 163);}
        if (p.p32 == 0.0) {s.store_scalar(596, 0.0);s.store_scalar(597, 0.0);s.store_scalar(163, 0.0);}
        s.store_sqrt_square_offset(639, 156, ((4.0 * 3000.0) * 3000.0));s.store_offset_scaled_div(279, 156, 639, 0.5, 0.5);
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_34(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.store_offset_add_scaled_inputs_indices(156, 156, 0.5, 639, 0.5, (1e-10 * 3000.0));s.b[891] = (s.v[156] < 0.0);s.store_scalar(891, if s.b[891] { 1.0 } else { 0.0 });
        if s.b[891] {s.store_scalar(156, 0.0);s.store_scalar(279, 0.0);}
        s.store_powf(286, 156, p.p94);s.store_powf(284, 156, s.v[470]);s.store_scale(157, 502, 6.241449993689894e18);s.store_add_scaled_inputs_mixed_ai(279, A::add_scaled_product(A::div_from_scalar(1.0, A::scale_offset(s.ad_value(157), (s.v[449] * 1e-11), s.v[448])), 1.0, s.ad_value(469), s.ad_value(286), 1.0), 1.0, 284, 1.0 / (p.p105));s.store_div_from_scalar(159, 1.0, 279);s.store_scale(159, 159, 0.0001);
        if (p.p32 != 0.0) {s.store_scaled_sub(163, 596, 597, (3.9 * 1.0 / ((11.7 * s.v[507]))));}
        if (p.p32 == 0.0) {s.store_sqrt_square_offset(639, 155, ((4.0 * 1e-6) * 1e-6));s.store_offset_scaled_div(278, 155, 639, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(598, 155, 0.5, 639, 0.5, (1e-10 * 1e-6));}
        s.b[892] = (s.v[598] < 0.0);s.store_scalar(892, if s.b[892] { 1.0 } else { 0.0 });
        if ((p.p32 == 0.0) && s.b[892]) {s.store_scalar(598, 0.0);s.store_scalar(278, 0.0);}
        if (p.p32 == 0.0) {s.store_offset_sqrt_ad(168, A::offset(A::square(s.ad_value(598)), p.p216), (-((p.p216) as f64).sqrt()));s.store_powf(168, 168, p.p85);s.store_offset_scaled(282, 168, p.p84, 1.0);s.store_scalar(498, (p.p302 * (1.0 + (p.p300 / ((s.v[375]) as f64).powf(p.p301)))));s.store_add_scaled_product_indices(288, 503, 1.0, 498, 500, (-1.0));s.store_scaled_add(508, 505, 504, (-0.5));s.store_add_scaled_inputs(283, 508, s.v[592], 288, s.v[591]);s.store_div(163, 283, 282);}
        s.store_sqrt_square_offset(639, 163, ((4.0 * 30.0) * 30.0));s.store_offset_scaled_div(279, 163, 639, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(163, 163, 0.5, 639, 0.5, (1e-10 * 30.0));s.b[893] = (s.v[163] < 0.0);s.store_scalar(893, if s.b[893] { 1.0 } else { 0.0 });
        if s.b[893] {s.store_scalar(163, 0.0);s.store_scalar(279, 0.0);}
        s.store_powf(286, 163, p.p275);s.store_powf(284, 163, s.v[594]);s.store_scale(157, 503, 6.241449993689894e18);s.store_add_scaled_inputs_mixed_ai(279, A::add_scaled_product(A::div_from_scalar(1.0, A::scale_offset(s.ad_value(157), (s.v[451] * 1e-11), s.v[450])), 1.0, s.ad_value(595), s.ad_value(286), 1.0), 1.0, 284, 1.0 / (p.p284));s.store_div_from_scalar(166, 1.0, 279);s.store_scale(166, 166, 0.0001);s.store_div_scaled_inputs_indices(454, 162, 0.2, 159, 1.0);s.store_div_mixed_ia(291, 153, A::mul3(s.ad_value(120), A::offset(s.ad_value(149), 1e-50), s.ad_value(386)));s.store_sqrt_square_sum(160, 291, 454);s.store_mul(161, 159, 160);s.store_div(279, 161, 162);s.b[894] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= p.p114) && (p.p114 <= (1.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(894, if s.b[894] { 1.0 } else { 0.0 });
        if s.b[894] {s.store_scalar(281, 1.0);}
        s.b[895] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= p.p114) && (p.p114 <= (2.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(895, if s.b[895] { 1.0 } else { 0.0 });
        if ((!s.b[894]) && s.b[895]) {s.copy_ad(281, 279);}
        if ((!s.b[894]) && (!s.b[895])) {s.store_powf(281, 279, (p.p114 - 1.0));}
        s.store_offset_mul(282, 279, 281, 1.0);s.b[896] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= p.p114) && (p.p114 <= (1.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(896, if s.b[896] { 1.0 } else { 0.0 });
        if s.b[896] {s.store_div_from_scalar(283, 1.0, 282);}
        s.b[897] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= p.p114) && (p.p114 <= (2.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(897, if s.b[897] { 1.0 } else { 0.0 });
        if ((!s.b[896]) && s.b[897]) {s.store_div_from_scalar_sqrt_ad(283, 1.0, s.ad_value(282));}
        if ((!s.b[896]) && (!s.b[897])) {s.store_powf(284, 282, (((-1.0) / p.p114) - 1.0));s.store_mul(283, 282, 284);}
        s.store_mul(158, 159, 283);s.store_div_scaled_inputs_indices(455, 162, 0.2, 166, 1.0);s.store_div_mixed_ia(291, 154, A::mul3(s.ad_value(120), A::offset(s.ad_value(150), 1e-50), s.ad_value(386)));s.store_sqrt_square_sum(164, 291, 455);s.store_mul(161, 166, 164);s.store_div(279, 161, 162);s.b[898] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= p.p114) && (p.p114 <= (1.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(898, if s.b[898] { 1.0 } else { 0.0 });
        if s.b[898] {s.store_scalar(281, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_35(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[899] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= p.p114) && (p.p114 <= (2.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(899, if s.b[899] { 1.0 } else { 0.0 });
        if ((!s.b[898]) && s.b[899]) {s.copy_ad(281, 279);}
        if ((!s.b[898]) && (!s.b[899])) {s.store_powf(281, 279, (p.p114 - 1.0));}
        s.store_offset_mul(282, 279, 281, 1.0);s.b[900] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= p.p114) && (p.p114 <= (1.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(900, if s.b[900] { 1.0 } else { 0.0 });
        if s.b[900] {s.store_div_from_scalar(283, 1.0, 282);}
        s.b[901] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= p.p114) && (p.p114 <= (2.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(901, if s.b[901] { 1.0 } else { 0.0 });
        if ((!s.b[900]) && s.b[901]) {s.store_div_from_scalar_sqrt_ad(283, 1.0, s.ad_value(282));}
        if ((!s.b[900]) && (!s.b[901])) {s.store_powf(284, 282, (((-1.0) / p.p114) - 1.0));s.store_mul(283, 282, 284);}
        s.store_mul(165, 166, 283);s.store_div_scaled_inputs_mixed_ia(189, 122, s.v[466], A::sub(s.ad_value(123), s.ad_value(262)), 1.0);s.store_mul3_lhs(96, 189, 153, 158);s.store_mul3_lhs(97, 189, 154, 165);s.store_add(95, 96, 97);s.store_scalar(173, 0.0);s.store_scalar(169, 0.0);s.store_scalar(171, 0.0);s.store_scalar(172, 0.0);s.b[902] = (p.p239 != 0.0);s.store_scalar(902, if s.b[902] { 1.0 } else { 0.0 });
        if s.b[902] {s.store_scaled_sub(279, 51, 59, 0.5);s.store_scale(638, 279, (2.0 * 100.0));s.store_offset_mul_offset_rhs_mixed_ia(639, 638, A::mul_offset_rhs(s.ad_value(638), A::mul_offset_rhs(s.ad_value(638), A::mul_offset_rhs(s.ad_value(638), A::mul(s.ad_value(638), A::scale_offset(s.ad_value(638), (1.0 / 5040.0), (1.0 / 720.0))), (1.0 / 120.0)), (1.0 / 24.0)), (1.0 / 6.0)), (1.0 / 2.0), 1.0);s.store_offset_mul_offset_rhs_mixed_ia(640, 638, A::mul_offset_rhs(s.ad_value(638), A::mul_offset_rhs(s.ad_value(638), A::mul(s.ad_value(638), A::scale_offset(s.ad_value(638), (1.0 / 840.0), (1.0 / 144.0))), (1.0 / 30.0)), (1.0 / 8.0)), (1.0 / 3.0), (1.0 / 2.0));s.store_div_from_scalar(284, 0.01, 639);s.store_div_scaled_inputs_square_rhs(280, 640, (-2.0), 639, 1.0);s.store_sub_from_scalar_ad(279, 1.1, A::add(s.ad_value(56), s.ad_value(284)));s.store_sqrt_square_offset(639, 279, ((4.0 * 0.05) * 0.05));s.store_offset_scaled_div(278, 279, 639, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(280, 279, 0.5, 639, 0.5, (1e-10 * 0.05));}
        s.b[903] = (s.v[280] < 0.0);s.store_scalar(903, if s.b[903] { 1.0 } else { 0.0 });
        if (s.b[902] && s.b[903]) {s.store_scalar(280, 0.0);s.store_scalar(278, 0.0);}
        if s.b[902] {s.store_mul_ad_affine_product_rhs(287, 270, s.ad_value(120), A::powf(s.ad_value(280), p.p240), s.v[475], 0.0);s.store_add_scaled_product_mixed_aia(282, A::scale_offset(s.ad_value(71), p.p241, 1.0), 1.0, 71, A::add_scaled_inputs3(s.ad_value(56), 1.0, s.ad_value(284), 1.0, s.ad_value(70), -1.0), s.v[476]);s.store_mul(287, 287, 282);}
        if (!s.b[902]) {s.store_scalar(287, 0.0);}
        s.b[904] = (p.p246 != 0.0);s.store_scalar(904, if s.b[904] { 1.0 } else { 0.0 });
        if s.b[904] {s.store_mul3_affine_lhs(286, 270, 120, s.v[477], 0.0, 71);}
        if (!s.b[904]) {s.store_scalar(286, 0.0);}
        s.b[905] = ((s.v[287] + s.v[286]) > 0.0);s.store_scalar(905, if s.b[905] { 1.0 } else { 0.0 });
        if s.b[905] {s.store_mul_add_rhs(152, 59, 287, 286);s.store_mul3_lhs(173, 189, 152, 158);s.store_div_from_scalar_offset_ad(172, 1.0, A::exp_scaled_input(s.ad_value(440), (-p.p245)), 1.0);s.store_sub_from_scalar(171, 1.0, 172);s.store_mul(169, 171, 173);}
        s.store_scalar(174, 0.0);s.store_scalar(170, 0.0);s.b[906] = (p.p239 != 0.0);s.store_scalar(906, if s.b[906] { 1.0 } else { 0.0 });
        if s.b[906] {s.store_scaled_sub(279, 51, 155, 0.5);s.store_scale(638, 279, (2.0 * 100.0));}
    }
}
