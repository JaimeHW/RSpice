#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_reactive_block_4(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[681] {s.store_div_scaled_offset_numerator(677, A::mul(s.ad_value(678), s.ad_value(679)), s.v[677], s.v[677], A::offset(A::mul(s.ad_value(678), s.ad_value(680)), 1.0), 1.0);}
        s.store_scalar(678, (1.0 + (p.p112 / ((s.v[375]) as f64).powf(p.p113))));s.store_offset_ad(378, A::mul_scaled_lhs(A::scale_offset(s.ad_value(374), 1.0 / (s.v[445]), (-1.0)), p.p253, A::scale_offset(s.ad_value(374), 1.0 / (s.v[445]), (-1.0))), (p.p111 * s.v[678]));s.store_pow_ad(678, A::scale(s.ad_value(374), 1.0 / (s.v[445])), s.ad_value(378));s.store_div(469, 678, 676);s.store_div(595, 678, 677);s.store_mul(380, 478, 122);s.store_scalar(279, ((((1.0 + (p.p181 / ((s.v[375]) as f64).powf(p.p182))) * (1.0 + (p.p185 / ((s.v[375]) as f64).powf(p.p186)))) * (1.0 + (p.p187 / ((s.v[376]) as f64).powf(p.p188)))) * (1.0 + (p.p183 / ((s.v[377]) as f64).powf(p.p184)))));s.store_scalar(639, ((((s.v[279] * s.v[279]) + ((4.0 * 0.001) * 0.001))) as f64).sqrt());s.store_scalar(280, (0.5 * (1.0 + (s.v[279] / s.v[639]))));s.store_scalar(480, ((0.5 * (s.v[279] + s.v[639])) + (1e-10 * 0.001)));s.b[682] = (s.v[480] < 0.0);s.store_scalar(682, if s.b[682] { 1.0 } else { 0.0 });
        if s.b[682] {s.store_scalar(480, 0.0);s.store_scalar(280, 0.0);}
        s.store_scale(279, 374, 1.0 / (s.v[445]));s.store_scalar(280, (1.0 + (p.p102 / ((s.v[375]) as f64).powf(p.p103))));s.store_div_scaled_inputs_mixed_ia(162, 480, (s.v[613] * 0.01), A::sub(A::add_scaled_product(A::scale_offset(s.ad_value(279), (0.4 * 0.01), (1.8 * 0.01)), 1.0, s.ad_value(279), s.ad_value(279), (0.1 * 0.01)), A::scale_offset(s.ad_value(279), (-(s.v[615] * s.v[280])), (s.v[615] * s.v[280]))), 1.0);s.store_sqrt(245, 137);s.store_mul(246, 137, 245);s.store_scaled_mul_ad(127, A::powf(A::scale(s.ad_value(374), 1.0 / (s.v[445])), 1.5), A::exp(A::offset(A::mul_scaled_lhs(s.ad_value(137), (-1.0 / (2.0)), s.ad_value(120)), ((s.v[465] / 2.0) * s.v[464]))), 1.04e16);s.store_scalar(117, (((((2.0 * 1.6021918e-19) * s.v[452]) * 1.034943e-10)) as f64).sqrt());s.store_scalar(118, (1.0 / (s.v[452] * s.v[452])));s.store_scaled_sqrt(100, 122, s.v[117]);s.store_square(119, 100);s.store_scaled_square(101, 127, s.v[118]);s.store_scalar(279, ((p.p38 / (p.p251 + p.p252)) * p.p0));s.store_scalar(281, ((((p.p38 * 0.001) + ((10.0 * 2.220446049250313e-16) / 100.0))) as f64).abs());s.b[683] = (p.p38 > 0.0);s.store_scalar(683, if s.b[683] { 1.0 } else { 0.0 });
        if s.b[683] {s.store_scalar(638, ((p.p38 - s.v[279]) - s.v[281]));s.store_scalar(639, ((4.0 * p.p38) * s.v[281]));}
        if s.b[683] {
            if (s.v[639] > 0.0) {
            } else {
                s.store_neg(639, 639);
            }
        }
        if s.b[683] {s.store_sqrt_square_add(639, 638, 639);s.store_offset_scaled_div(279, 638, 639, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(280, 638, (-0.5), 639, (-0.5), p.p38);}
        if (!s.b[683]) {s.store_offset(638, 279, (((-p.p38)) + ((-s.v[281]))));s.store_scalar(639, ((4.0 * p.p38) * s.v[281]));}
        if (!s.b[683]) {
            if (s.v[639] > 0.0) {
            } else {
                s.store_neg(639, 639);
            }
        }
        if (!s.b[683]) {s.store_sqrt_square_add(639, 638, 639);s.store_offset_scaled_div(279, 638, 639, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(280, 638, 0.5, 639, 0.5, p.p38);}
        s.store_sub_from_scalar_scaled_input(123, p.p0, 280, 2.0);s.store_scalar(279, ((-p.p49) * (1.0 + (p.p50 / ((s.v[375]) as f64).powf(p.p51)))));s.store_scalar(280, ((-p.p49) * (1.0 + (p.p52 / ((s.v[375]) as f64).powf(p.p53)))));s.store_scalar(281, (-(p.p49 + (p.p54 * s.v[375]))));s.store_scalar(638, ((s.v[279] - s.v[280]) - 1e-12));s.store_scalar(639, ((4.0 * s.v[280]) * 1e-12));
        if (!(s.v[639] > 0.0)) {s.store_scalar(639, (-s.v[639]));}
        s.store_sqrt_offset_input(639, 639, (s.v[638] * s.v[638]));s.store_scaled_offset_ad(279, A::div_from_scalar(s.v[638], s.ad_value(639)), 1.0, 0.5);s.store_offset_scaled(138, 639, 0.5, ((((s.v[638]) * (0.5))) + (s.v[280])));s.store_offset(638, 138, (((-s.v[281])) + ((-1e-12))));s.store_scalar(639, ((4.0 * s.v[281]) * 1e-12));
        if (!(s.v[639] > 0.0)) {s.store_scalar(639, (-s.v[639]));}
        s.store_sqrt_square_add(639, 638, 639);s.store_offset_scaled_div(279, 638, 639, 0.5, 0.5);
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_5(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.store_offset_add_scaled_inputs_indices(138, 638, 0.5, 639, 0.5, s.v[281]);s.store_neg(138, 138);s.store_mul_scaled_ln_ad_rhs(128, 122, 2.0, A::div(s.ad_value(471), s.ad_value(127)));s.store_sqrt_mul_ad(125, A::div_from_scalar(1.034943e-10, s.ad_value(126)), s.ad_value(122));s.store_scaled_mul(141, 126, 125, 1.414213562373095);s.copy_ad(438, 474);s.store_sqrt_mul_scaled_lhs(439, 438, 2.0, 122);s.store_div(279, 127, 471);s.store_square(142, 279);s.store_div(279, 127, 462);s.store_square(143, 279);s.store_scalar(272, p.p226);s.store_scalar(273, (3.453133e-11 / s.v[272]));s.store_scalar(274, (s.v[272] / 3.453133e-11));s.store_scalar(294, (3.453133e-11 / p.p229));s.store_scalar(295, (p.p229 / 3.453133e-11));s.store_scale(296, 471, ((-1.6021918e-19) * p.p227));s.store_scalar(535, (1.034943e-10 / p.p227));s.store_scalar(536, (1.0 / s.v[535]));s.store_scalar(293, (s.v[295] + s.v[536]));s.store_scalar(31, p.p254);s.store_scalar(30, p.p255);s.b[688] = (s.v[31] > (s.v[30] * 0.5));s.store_scalar(688, if s.b[688] { 1.0 } else { 0.0 });
        if s.b[688] {s.store_scalar(31, (0.5 * s.v[30]));}
        s.b[689] = (s.v[47] > s.v[31]);s.store_scalar(689, if s.b[689] { 1.0 } else { 0.0 });
        if s.b[689] {s.store_sub(280, 47, 31);s.store_sub_from_scalar(281, s.v[30], 31);s.store_square(642, 280);s.store_square(643, 281);s.store_scalar(644, 1.0);s.store_scalar(645, 1.0);s.store_scalar(647, 0.0);s.store_scalar(648, 0.0);s.store_scalar(220, 0.0);s.store_scalar(646, 0.0);s.store_mul(644, 644, 642);s.store_mul(645, 645, 643);s.store_mul(644, 644, 642);s.store_mul(645, 645, 643);s.store_mul(644, 644, 642);s.store_mul(645, 645, 643);s.store_mul(644, 644, 642);s.store_mul(645, 645, 643);s.store_add(220, 644, 645);s.copy_ad(646, 220);}
        s.b[690] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));s.store_scalar(690, if s.b[690] { 1.0 } else { 0.0 });s.b[691] = (4.0 == 1.0);s.store_scalar(691, if s.b[691] { 1.0 } else { 0.0 });
        if ((s.b[689] && s.b[690]) && s.b[691]) {s.store_scalar(648, 1.0);}
        s.b[692] = (4.0 == 2.0);s.store_scalar(692, if s.b[692] { 1.0 } else { 0.0 });
        if (((s.b[689] && s.b[690]) && (!s.b[691])) && s.b[692]) {s.store_scalar(648, 2.0);}
        s.b[693] = (4.0 == 4.0);s.store_scalar(693, if s.b[693] { 1.0 } else { 0.0 });
        if ((((s.b[689] && s.b[690]) && (!s.b[691])) && (!s.b[692])) && s.b[693]) {s.store_scalar(648, 3.0);}
        s.b[694] = (4.0 == 8.0);s.store_scalar(694, if s.b[694] { 1.0 } else { 0.0 });
        if (((((s.b[689] && s.b[690]) && (!s.b[691])) && (!s.b[692])) && (!s.b[693])) && s.b[694]) {s.store_scalar(648, 4.0);}
        if (s.b[689] && s.b[690]) {s.store_scalar(647, 0.0);}
        let mut t1: usize = 0;
        while {
            let t0: f64 = if ((s.b[689] && s.b[690]) && (s.v[647] < s.v[648])) { 1.0 } else { 0.0 };
            t0 != 0.0
        } {
            t1 += 1;assert!(t1 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (s.b[689] && s.b[690]) {s.store_sqrt(646, 646);s.store_primal_offset(647, 647, 1.0);}
        }
        if (s.b[689] && (!s.b[690])) {s.store_powf(646, 646, (1.0 / (2.0 * 4.0)));}
        if s.b[689] {s.store_div_from_scalar_offset_input(646, 1.0, 646, 1e-50);s.store_mul3_lhs(282, 280, 281, 646);s.store_div_scaled_product3_mixed_iiia(286, 281, 645, 646, 1.0, A::offset(s.ad_value(220), 1e-50), 1.0);s.store_add(43, 31, 282);s.copy_ad(46, 286);}
        if (!s.b[689]) {s.copy_ad(43, 47);s.store_scalar(46, 1.0);}
        s.copy_ad(44, 48);s.copy_ad(45, 49);s.store_scalar(33, 0.0);s.store_scalar(695, 0.0);s.store_scalar(696, 0.0);s.store_scalar(697, 0.0);s.store_scalar(698, 0.0);s.store_scalar(699, 0.0);s.store_scalar(700, 0.0);s.copy_ad(50, 43);s.copy_ad(51, 44);s.copy_ad(52, 45);
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_6(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.store_scalar(62, 0.0);s.store_scalar(63, 0.0);s.store_scaled_mul(279, 46, 51, 0.5);s.store_scale(638, 279, (2.0 * 1.0 / (p.p216)));s.store_offset_mul_offset_rhs_mixed_ia(639, 638, A::mul_offset_rhs(s.ad_value(638), A::mul_offset_rhs(s.ad_value(638), A::mul_offset_rhs(s.ad_value(638), A::mul(s.ad_value(638), A::scale_offset(s.ad_value(638), (1.0 / 5040.0), (1.0 / 720.0))), (1.0 / 120.0)), (1.0 / 24.0)), (1.0 / 6.0)), (1.0 / 2.0), 1.0);s.store_offset_mul_offset_rhs_mixed_ia(640, 638, A::mul_offset_rhs(s.ad_value(638), A::mul_offset_rhs(s.ad_value(638), A::mul(s.ad_value(638), A::scale_offset(s.ad_value(638), (1.0 / 840.0), (1.0 / 144.0))), (1.0 / 30.0)), (1.0 / 8.0)), (1.0 / 3.0), (1.0 / 2.0));s.store_div_from_scalar(73, p.p216, 639);s.store_div_scaled_inputs_square_rhs(280, 640, (-2.0), 639, 1.0);s.b[701] = (s.v[73] < 1e-12);s.store_scalar(701, if s.b[701] { 1.0 } else { 0.0 });
        if s.b[701] {s.store_scalar(73, 1e-12);}
        s.store_add(70, 50, 73);s.store_add_scaled_inputs(71, 51, 1.0, 73, 2.0);s.store_add(72, 52, 73);s.store_scale(279, 126, (2.0 * (1.034943e-10 * (s.v[274] * s.v[274]))));s.store_sub(280, 52, 138);s.store_offset_mul_ad(281, A::div_from_scalar(2.0, s.ad_value(279)), A::add_scaled_inputs3(s.ad_value(280), 1.0, s.ad_value(122), (-1.0), s.ad_value(50), -1.0), 1.0);s.store_sqrt_square_offset(639, 281, ((4.0 * 0.001) * 0.001));s.store_offset_scaled_div(283, 281, 639, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(282, 281, 0.5, 639, 0.5, (1e-10 * 0.001));s.b[702] = (s.v[282] < 0.0);s.store_scalar(702, if s.b[702] { 1.0 } else { 0.0 });
        if s.b[702] {s.store_scalar(282, 0.0);s.store_scalar(283, 0.0);}
        s.store_sqrt_offset_input(290, 282, 1e-50);s.store_add_mul_sub_from_scalar_rhs_indices(87, 280, 279, 1.0, 290);s.store_sub(88, 87, 128);s.store_offset(638, 88, (((-0.1)) + ((-0.05))));s.store_scalar(639, ((4.0 * 0.1) * 0.05));
        if (!(s.v[639] > 0.0)) {s.store_scalar(639, (-s.v[639]));}
        s.store_sqrt_square_add(639, 638, 639);s.store_offset_scaled_div(284, 638, 639, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(88, 638, 0.5, 639, 0.5, 0.1);s.store_div(279, 51, 88);s.copy_ad(638, 279);s.store_square(639, 638);s.store_mul(640, 639, 638);s.store_square(641, 639);s.store_div_from_scalar_ad(290, 1.0, A::add_scaled_inputs4_offset(s.ad_value(638), 1.0, s.ad_value(639), 1.0, s.ad_value(640), 1.0, s.ad_value(641), 1.0, 1.0));s.store_mul_ad_affine_product_lhs(278, A::add_scaled_inputs3_offset(s.ad_value(638), 2.0, s.ad_value(639), 3.0, s.ad_value(640), 4.0, 1.0), s.ad_value(290), -1.0, 0.0, 290);s.store_sub_from_scalar(290, 1.0, 290);s.store_neg(278, 278);s.store_square(276, 290);s.b[703] = (((p.p193 == 0.0) && (p.p195 == 0.0)) || (p.p194 == 0.0));s.store_scalar(703, if s.b[703] { 1.0 } else { 0.0 });
        if s.b[703] {s.store_scalar(37, 0.0);}
        if (!s.b[703]) {s.store_scalar(37, 1.0);}
        s.store_add_scaled_inputs3_sqrt_third_mixed_iia(275, 129, 1.0, 138, 1.0, A::mul_scaled_lhs(s.ad_value(126), (2.0 * 1.034943e-10), s.ad_value(129)), 1.0 / (s.v[273]));s.b[704] = (s.v[37] == 0.0);s.store_scalar(704, if s.b[704] { 1.0 } else { 0.0 });
        if s.b[704] {s.store_scalar(268, s.v[272]);s.store_scalar(270, s.v[273]);s.store_scalar(271, s.v[274]);s.store_scale(278, 141, (s.v[274] * s.v[274]));s.store_mul(381, 278, 141);}
        if (!s.b[704]) {s.store_add_scaled_inputs3_offset_indices(283, 52, 1.0, 50, (-1.0), 275, -1.0, p.p194);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_7(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (!s.b[704]) {s.store_sqrt_square_offset(639, 283, ((4.0 * 0.0001) * 0.0001));s.store_offset_scaled_div(281, 283, 639, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(280, 283, 0.5, 639, 0.5, (1e-10 * 0.0001));}
        s.b[705] = (s.v[280] < 0.0);s.store_scalar(705, if s.b[705] { 1.0 } else { 0.0 });
        if ((!s.b[704]) && s.b[705]) {s.store_scalar(280, 0.0);s.store_scalar(281, 0.0);}
        if (!s.b[704]) {s.store_div_from_scalar(281, 1.0, 280);s.store_scaled_abs(282, 275, 2.0);s.store_offset_sub(284, 138, 275, p.p194);}
        s.b[706] = (s.v[284] > s.v[282]);s.store_scalar(706, if s.b[706] { 1.0 } else { 0.0 });
        if ((!s.b[704]) && s.b[706]) {s.copy_ad(282, 284);}
        if (!s.b[704]) {s.store_offset_sub_ad(638, A::div_from_scalar(1.0, s.ad_value(282)), s.ad_value(281), (-0.0001));s.store_scale_ad(639, A::div_from_scalar(1.0, s.ad_value(282)), (4.0 * 0.0001));}
        if (!s.b[704]) {
            if (s.v[639] > 0.0) {
            } else {
                s.store_neg(639, 639);
            }
        }
        if (!s.b[704]) {s.store_sqrt_square_add(639, 638, 639);s.store_offset_scaled_div(284, 638, 639, 0.5, 0.5);s.store_add_scaled_inputs3_mixed_aii(280, A::div_from_scalar(1.0, s.ad_value(282)), 1.0, 638, (-0.5), 639, (-0.5));s.store_offset_scaled(269, 280, p.p193, p.p195);}
        s.b[707] = ((s.v[269] * 1000000000000.0) < s.v[272]);s.store_scalar(707, if s.b[707] { 1.0 } else { 0.0 });
        if ((!s.b[704]) && s.b[707]) {s.store_scalar(269, 0.0);s.store_scalar(37, 0.0);}
        if (!s.b[704]) {s.store_offset(268, 269, s.v[272]);s.store_div_from_scalar(270, 3.453133e-11, 268);s.store_scale(271, 268, 28959208927.08158);s.store_mul_ad_product_lhs_mixed_ai(381, A::square(s.ad_value(141)), 271, 271);}
        s.store_offset_sub_from_scalar_ad(638, 0.5, s.ad_value(70), (-0.001));s.store_scalar(639, ((4.0 * 0.5) * 0.001));
        if (!(s.v[639] > 0.0)) {s.store_scalar(639, (-s.v[639]));}
        s.store_sqrt_square_add(639, 638, 639);s.store_offset_scaled_div(278, 638, 639, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(382, 638, (-0.5), 639, (-0.5), 0.5);s.store_sqrt_mul(150, 473, 129);s.store_add_mixed_ai(265, A::add_scaled_inputs_product(s.ad_value(129), 1.0, s.ad_value(138), 1.0, s.ad_value(150), s.ad_value(271), 1.0), 380);s.copy_ad(130, 129);s.store_scalar(278, 0.95);s.store_offset_sub_scaled_inputs_indices(279, 130, s.v[278], 382, 1.0, (-0.001));s.store_sqrt_add_scaled_square_input(280, 279, 1.0, 130, ((4.0 * s.v[278]) * 0.001));s.store_add_scaled_inputs4_indices(131, 130, 1.0, 130, (-s.v[278]), 279, (-(-0.5)), 280, (-(-0.5)));s.store_sqrt(135, 131);s.b[708] = (p.p58 != 0.0);s.store_scalar(708, if s.b[708] { 1.0 } else { 0.0 });
        if s.b[708] {s.store_sqrt_mul_scaled_lhs(278, 471, ((2.0 * 1.6021918e-19) * 1.034943e-10), 136);s.store_add_scaled_inputs_product_indices(79, 136, 1.0, 138, 1.0, 278, 271, 1.0);s.store_scalar(278, ((2.0 * p.p227) / (p.p58 * p.p58)));s.store_mul_ad_affine_product_rhs(81, 271, s.ad_value(278), A::sub_from_scalar(p.p55, s.ad_value(130)), 1.034943e-10, 0.0);s.store_add_scaled_inputs_mixed_ai(278, A::scale_offset(s.ad_value(131), (p.p68 / p.p58), p.p66), 1.0, 71, p.p67);s.store_mul_ad_product_lhs_mixed_ai(266, A::sub(s.ad_value(265), s.ad_value(79)), 81, 278);}
        if (!s.b[708]) {s.store_scalar(266, 0.0);}
        s.b[709] = (p.p297 != 0.0);s.store_scalar(709, if s.b[709] { 1.0 } else { 0.0 });
        if s.b[709] {s.store_offset_add_ad(288, A::add_scaled_product(s.ad_value(122), 1.0, s.ad_value(381), s.ad_value(120), (-0.25)), s.ad_value(138), 1e-50);s.store_offset_sub(279, 72, 288, (-0.005));}
        if s.b[709] {s.store_scalar(278, (if (s.v[288] >= 0.0) { 1.0 } else { (-1.0) }));}
        if s.b[709] {s.store_sqrt_add_scaled_square_product(280, 279, 1.0, 278, 288, (4.0 * 0.005));s.store_add_scaled_inputs4_indices(281, 288, 1.0, 279, 0.5, 280, 0.5, 138, -1.0);s.store_mul_ad_product_lhs_mixed_ai(282, A::div_from_scalar(4.0, s.ad_value(381)), 122, 122);s.store_offset_mul(283, 120, 281, (-1.0));s.store_offset_mul(279, 283, 282, 1.0);s.store_sqrt_square_offset(639, 279, ((4.0 * 0.001) * 0.001));s.store_offset_scaled_div(285, 279, 639, 0.5, 0.5);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_8(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[709] {s.store_offset_add_scaled_inputs_indices(279, 279, 0.5, 639, 0.5, (1e-10 * 0.001));}
        s.b[710] = (s.v[279] < 0.0);s.store_scalar(710, if s.b[710] { 1.0 } else { 0.0 });
        if (s.b[709] && s.b[710]) {s.store_scalar(279, 0.0);s.store_scalar(285, 0.0);}
        if s.b[709] {s.store_sqrt_offset_input(280, 279, (10.0 * 2.220446049250313e-16));s.store_add_product3_rhs_mixed_iia(139, 281, 381, 120, A::sub_from_scalar(1.0, s.ad_value(280)), 0.5);s.store_offset_sub(638, 129, 139, (-0.005));s.store_scale(639, 129, (4.0 * 0.005));}
        if s.b[709] {
            if (s.v[639] > 0.0) {
            } else {
                s.store_neg(639, 639);
            }
        }
        if s.b[709] {s.store_sqrt_square_add(639, 638, 639);s.store_offset_scaled_div(280, 638, 639, 0.5, 0.5);s.store_add_scaled_inputs3_indices(140, 129, 1.0, 638, (-0.5), 639, (-0.5));s.store_add_scaled_inputs3_indices(130, 129, 1.0, 140, p.p297, 129, (-p.p297));}
        s.store_scale(279, 271, (1.034943e-10 * (p.p227 * 2.0)));s.store_sub_from_scalar(280, p.p55, 130);s.store_scalar(281, (s.v[277] - p.p57));s.store_scaled_mul(81, 279, 280, 1.0 / ((s.v[281] * s.v[281])));s.store_sqrt_square_offset(639, 50, ((4.0 * 0.001) * 0.001));s.store_offset_scaled_div(278, 50, 639, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(593, 50, 0.5, 639, 0.5, (1e-10 * 0.001));s.b[711] = (s.v[593] < 0.0);s.store_scalar(711, if s.b[711] { 1.0 } else { 0.0 });
        if s.b[711] {s.store_scalar(593, 0.0);s.store_scalar(278, 0.0);}
        s.store_add_scaled_inputs3_offset_indices(283, 131, (p.p71 / s.v[277]), 71, p.p70, 593, p.p250, p.p69);s.store_mul(82, 81, 283);s.b[712] = (p.p72 > 0.0);s.store_scalar(712, if s.b[712] { 1.0 } else { 0.0 });
        if s.b[712] {s.store_add_scaled_inputs3_offset_indices(279, 137, 1.0, 128, 1.0, 71, p.p73, (-(2.0 * p.p74)));s.store_scalar(280, ((s.v[277] * 0.5) + p.p56));s.store_div_from_scalar(281, (p.p72 * p.p227), 280);s.store_mul(83, 279, 281);}
        if (!s.b[712]) {s.store_scalar(83, 0.0);}
        s.store_div_from_scalar_offset_input(281, 1.0, 270, (s.v[626] / s.v[124]));s.store_sub(283, 271, 281);s.store_offset_mul(84, 150, 283, (p.p104 / s.v[376]));s.store_add_scaled_inputs4_offset_indices(80, 82, 1.0, 266, 1.0, 84, 1.0, 83, 1.0, s.v[482]);s.store_sub(78, 265, 80);s.b[713] = (p.p75 == 0.0);s.store_scalar(713, if s.b[713] { 1.0 } else { 0.0 });
        if s.b[713] {s.store_scalar(36, 0.0);}
        if (!s.b[713]) {s.store_scalar(36, 1.0);}
        s.b[714] = (s.v[36] == 0.0);s.store_scalar(714, if s.b[714] { 1.0 } else { 0.0 });
        if s.b[714] {s.store_scalar(267, 0.0);}
        if (!s.b[714]) {s.store_offset(281, 72, (-p.p76));}
        s.b[715] = (s.v[281] < (-3.0));s.store_scalar(715, if s.b[715] { 1.0 } else { 0.0 });
        if ((!s.b[714]) && s.b[715]) {s.store_scalar(284, 0.0);s.store_scalar(267, 0.0);}
        s.b[716] = (s.v[281] < 0.0);s.store_scalar(716, if s.b[716] { 1.0 } else { 0.0 });
        if (((!s.b[714]) && (!s.b[715])) && s.b[716]) {s.store_offset_mul_ad(284, s.ad_value(281), A::scale_offset(s.ad_value(281), (3.0 * (1.0 / 27.0)), (2.0 * (1.0 / 3.0))), 1.0);s.store_offset_mul_offset_rhs_mixed_ia(267, 281, A::mul(s.ad_value(281), A::scale_offset(s.ad_value(281), (1.0 / 27.0), (1.0 / 3.0))), 1.0, 1.0);}
        if (((!s.b[714]) && (!s.b[715])) && (!s.b[716])) {s.store_offset_mul_offset_rhs_mixed_ia(284, 281, A::mul(s.ad_value(281), A::scale_offset(s.ad_value(281), (4.0 * 0.148148111111111), (3.0 * 0.0402052934513951))), (2.0 * (1.0 / 3.0)), 1.0);s.store_offset_mul_offset_rhs_mixed_ia(267, 281, A::mul_offset_rhs(s.ad_value(281), A::mul(s.ad_value(281), A::scale_offset(s.ad_value(281), 0.148148111111111, 0.0402052934513951)), (1.0 / 3.0)), 1.0, 1.0);}
        if (!s.b[714]) {s.store_sqrt_offset_square_offset(639, 267, (-1.0), ((4.0 * 0.1) * 0.1));s.store_scaled_offset_ad(284, A::div_scaled_offset_numerator(s.ad_value(267), 1.0, (-1.0), s.ad_value(639), 1.0), 1.0, 0.5);s.store_offset_add_scaled_inputs_mixed_ai(267, A::offset(s.ad_value(267), (-1.0)), 0.5, 639, 0.5, (1e-10 * 0.1));}
        s.b[717] = (s.v[267] < 0.0);s.store_scalar(717, if s.b[717] { 1.0 } else { 0.0 });
        if ((!s.b[714]) && s.b[717]) {s.store_scalar(267, 0.0);s.store_scalar(284, 0.0);}
        if (!s.b[714]) {s.store_scale(267, 267, s.v[479]);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_9(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (!s.b[714]) {s.store_offset_sub_from_scalar_ad(638, 1.0, s.ad_value(267), (-0.05));s.store_scalar(639, (4.0 * 0.05));}
        if (!s.b[714]) {
            if (s.v[639] > 0.0) {
            } else {
                s.store_neg(639, 639);
            }
        }
        if (!s.b[714]) {s.store_sqrt_square_add(639, 638, 639);s.store_offset_scaled_div(287, 638, 639, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(267, 638, (-0.5), 639, (-0.5), 1.0);}
        s.store_add_scaled_inputs4_indices(53, 52, 1.0, 138, (-1.0), 80, 1.0, 267, -1.0);s.copy_ad(76, 53);s.store_mul_ln_mixed_ia(298, 122, A::div(s.ad_value(471), s.ad_value(462)));s.store_add_scaled_inputs3_indices(54, 138, 1.0, 80, (-1.0), 267, 1.0);s.store_mul(144, 141, 271);s.store_square(145, 144);
        if (p.p29 != 0.0) {s.store_add(440, 70, 298);}
        if (p.p29 == 0.0) {s.store_add(440, 50, 298);}
        s.b[718] = (s.v[440] < 0.0);s.store_scalar(718, if s.b[718] { 1.0 } else { 0.0 });
        if s.b[718] {s.store_div(278, 462, 471);s.store_offset(279, 278, 1.0);s.store_add_scaled_inputs_product_mixed_iiia(280, 122, 1.0, 440, (-1.0), 278, A::add(s.ad_value(122), s.ad_value(440)), 1.0);s.store_scaled_square(281, 439, (s.v[295] * s.v[295]));s.store_add_scaled_products_indices(282, 280, 279, 2.0, 281, 120, (-1.0));s.store_add_scaled_inputs3_mixed_aai(283, A::square(s.ad_value(280)), 1.0, A::mul3(s.ad_value(281), s.ad_value(120), s.ad_value(440)), 1.0, 281, 1.0);}
        if s.b[718] {
            if (((s.v[282] * s.v[282]) - (((4.0 * s.v[279]) * s.v[279]) * s.v[283])) >= 1e-50) {
                s.store_sub_ad(285, A::square(s.ad_value(282)), A::mul3_scaled_output(s.ad_value(279), s.ad_value(279), s.ad_value(283), 4.0));
            } else {
                s.store_scalar(285, 1e-50);
            }
        }
        if s.b[718] {s.store_div_scaled_inputs2_mixed_iaa(331, 282, 1.0, A::sqrt(s.ad_value(285)), 1.0, A::offset(A::square(s.ad_value(279)), 2.0), 1.0);}
        if (!s.b[718]) {s.store_mul_square_lhs(279, 439, 120);s.store_mul_square_lhs(280, 141, 120);s.store_neg_ad(281, A::add_scaled_inputs(s.ad_value(122), 1.0, s.ad_value(440), 2.0));s.store_offset_div(282, 280, 279, 1.0);s.store_scaled_square(283, 141, (s.v[295] * s.v[295]));s.store_add_scaled_products_indices(284, 283, 120, 1.0, 281, 282, (-2.0));}
        if (!s.b[718]) {
            if (((s.v[284] * s.v[284]) - ((((4.0 * s.v[282]) * s.v[282]) * s.v[281]) * s.v[281])) >= 1e-50) {
                s.store_add_scaled_square_product_mixed_iai(285, 284, 1.0, A::mul3_scaled_output(s.ad_value(282), s.ad_value(282), s.ad_value(281), 4.0), 281, (-1.0));
            } else {
                s.store_scalar(285, 1e-50);
            }
        }
        if (!s.b[718]) {s.store_div_scaled_inputs2_by_product_mixed_iaii(331, 284, 1.0, A::sqrt(s.ad_value(285)), 1.0, 282, 282, 2.0);}
        s.store_mul_div_from_scalar_lhs_ad_mixed_ia(326, 2.0, 120, A::ln(A::div(s.ad_value(462), s.ad_value(127))));s.store_scaled_square(278, 439, (s.v[293] * s.v[293]));s.store_neg(279, 440);s.store_add_scaled_inputs3_mixed_aai(280, A::square(A::add_scaled_product(s.ad_value(279), 2.0, s.ad_value(278), s.ad_value(120), 1.0)), 1.0, A::square(s.ad_value(279)), (-4.0), 278, (-4.0));
        if (!(s.v[280] >= (10.0 * 2.220446049250313e-16))) {s.store_scalar(280, (10.0 * 2.220446049250313e-16));}
        s.store_sqrt(280, 280);s.store_add_scaled_product_indices(281, 279, 2.0, 278, 120, 1.0);s.store_scaled_sub(324, 281, 280, 0.5);
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_10(
        s: &mut ReactiveScratch,
    ) {
        s.store_div_ad(325, A::ln(A::div_scaled_product_by_product(s.ad_value(279), s.ad_value(279), 1.0, s.ad_value(278), s.ad_value(143), 1.0)), A::add(s.ad_value(120), A::div_from_scalar(2.0, s.ad_value(279))));s.b[719] = (s.v[324] < s.v[326]);s.store_scalar(719, if s.b[719] { 1.0 } else { 0.0 });
        if s.b[719] {s.copy_ad(331, 324);}
        if (!s.b[719]) {s.store_offset_sub(638, 325, 324, (-0.0008));s.store_scale(639, 325, (4.0 * 0.0008));}
        if (!s.b[719]) {
            if (s.v[639] > 0.0) {
            } else {
                s.store_neg(639, 639);
            }
        }
        if (!s.b[719]) {s.store_sqrt_square_add(639, 638, 639);s.store_offset_scaled_div(279, 638, 639, 0.5, 0.5);s.store_add_scaled_inputs3_indices(331, 325, 1.0, 638, (-0.5), 639, (-0.5));}
        s.store_scalar(62, 0.0);
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_11(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        let mut t3: usize = 0;
        while {
            let t2: f64 = if s.v[62] < s.v[28] { 1.0 } else { 0.0 };
            t2 != 0.0
        } {
            t3 += 1;assert!(t3 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");s.copy_ad(279, 439);s.store_mul(280, 120, 331);s.store_exp_neg_input(281, 280);s.b[720] = (s.v[331] > 1e-8);s.store_scalar(720, if s.b[720] { 1.0 } else { 0.0 });
            if s.b[720] {s.store_exp_mul(278, 120, 331);s.store_mul_scaled_sqrt_ad_rhs(282, 279, -1.0, A::add_scaled_offset_product_rhs(A::offset(A::add(s.ad_value(281), s.ad_value(280)), (-1.0)), 1.0, s.ad_value(143), s.ad_value(278), (-1.0), 1.0));s.store_mul_div_scaled_inputs_mixed_aii(283, A::add_scaled_sub_value_product(1.0, s.ad_value(281), 1.0, s.ad_value(143), s.ad_value(278), 1.0), 438, 1.0, 282, 1.0);}
            s.b[721] = (s.v[331] < (-1e-8));s.store_scalar(721, if s.b[721] { 1.0 } else { 0.0 });
            if ((!s.b[720]) && s.b[721]) {s.store_mul_sqrt_mixed_ia(282, 279, A::offset(A::add(s.ad_value(281), s.ad_value(280)), (-1.0)));s.store_mul_scale_offset_mixed_ai(283, A::div(s.ad_value(438), s.ad_value(282)), 281, -1.0, 1.0);}
            if ((!s.b[720]) && (!s.b[721])) {s.store_mul_ad_affine_product_lhs(282, A::sqrt(A::div(s.ad_value(438), s.ad_value(120))), s.ad_value(120), -1.0, 0.0, 331);s.store_neg_ad(283, A::sqrt(A::mul(s.ad_value(438), s.ad_value(120))));}
            s.store_sqrt_square_offset(639, 282, ((4.0 * 1e-6) * 1e-6));s.store_offset_scaled_div(285, 282, 639, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(284, 282, 0.5, 639, 0.5, (1e-10 * 1e-6));s.b[722] = (s.v[284] < 0.0);s.store_scalar(722, if s.b[722] { 1.0 } else { 0.0 });
            if s.b[722] {s.store_scalar(284, 0.0);s.store_scalar(285, 0.0);}
            s.store_offset_sub_scaled_inputs_indices(638, 296, -1.0, 284, 1.0, (-1e-9));s.store_scale(639, 296, (-(4.0 * 1e-9)));
            if (!(s.v[639] > 0.0)) {s.store_neg(639, 639);}
            s.store_sqrt_square_add(639, 638, 639);s.store_offset_scaled_div(286, 638, 639, 0.5, 0.5);s.store_add_scaled_inputs3_indices(284, 296, -1.0, 638, (-0.5), 639, (-0.5));s.store_mul3_lhs(285, 285, 283, 286);s.store_div_scaled_inputs_mixed_ai(334, A::square(s.ad_value(284)), ((0.5 * 9662367879.197212) * 6.241449993689894e18), 471, 1.0);s.store_div_scaled_product_indices(335, 334, 285, 2.0, 284, 1.0);s.store_sub_mixed_ia(284, 331, A::div_scaled_inputs4(s.ad_value(282), 1.0 / (s.v[294]), s.ad_value(331), (-1.0), s.ad_value(440), -1.0, s.ad_value(334), 1.0, A::add(A::scale_offset(s.ad_value(283), 1.0 / (s.v[294]), (-1.0)), s.ad_value(335)), 1.0));s.b[723] = ((((s.v[284] - s.v[331])) as f64).abs() < 0.001);s.store_scalar(723, if s.b[723] { 1.0 } else { 0.0 });
            if s.b[723] {s.store_scalar(62, s.v[28]);}
            s.copy_ad(331, 284);s.copy_ad(330, 282);s.store_primal_offset(62, 62, 1.0);
        }
        s.copy_ad(332, 334);s.store_sqrt_div_scaled_inputs(279, 332, ((2.0 * 1.034943e-10) / 1.6021918e-19), 471, 1.0);s.b[724] = (s.v[279] > (0.99 * p.p227));s.store_scalar(724, if s.b[724] { 1.0 } else { 0.0 });
        if s.b[724] {s.store_div_from_scalar(278, 1.0, 270);s.store_scalar(280, (1.0 / s.v[294]));s.store_div_from_scalar_add_ad(281, 1.0, A::offset(s.ad_value(278), s.v[536]), s.ad_value(280));s.store_sub_from_scalar_scaled_mul(282, 1.0, 281, 278, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_12(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[724] {s.store_mul_ad_product_rhs_mixed_ia(283, 278, 281, A::sub(A::mul_scaled_rhs(A::offset(s.ad_value(280), (0.5 * s.v[536])), s.ad_value(296), -1.0), s.ad_value(440)));s.store_div(327, 283, 282);s.store_add(54, 54, 327);s.store_sub_scaled_inputs(53, 53, 1.0, 327, p.p298);s.copy_ad(76, 53);}
        s.b[725] = (s.v[33] >= 1.0);s.store_scalar(725, if s.b[725] { 1.0 } else { 0.0 });
        if s.b[725] {s.store_scalar(305, s.v[695]);s.store_scalar(306, s.v[696]);s.store_offset(307, 440, s.v[697]);s.store_add_scaled_inputs(328, 296, (-(s.v[536] * 0.5)), 122, 1.0);s.store_sub_scaled_inputs(329, 328, 1.0, 330, s.v[536]);}
        s.b[726] = (s.v[440] < 0.0);s.store_scalar(726, if s.b[726] { 1.0 } else { 0.0 });
        if ((!s.b[725]) && s.b[726]) {s.store_scalar(55, 0.0);s.store_scalar(62, 1.0);}
        let mut t5: usize = 0;
        while {
            let t4: f64 = if (((!s.b[725]) && s.b[726]) && (s.v[62] <= s.v[28])) { 1.0 } else { 0.0 };
            t4 != 0.0
        } {
            t5 += 1;assert!(t5 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((!s.b[725]) && s.b[726]) {s.store_div_from_scalar_scaled_input(278, s.v[294], 462, ((2.0 * 1.6021918e-19) * 1.034943e-10));s.store_scalar(279, (1.0 + (s.v[294] * s.v[536])));s.store_add_scaled_inputs3_indices(280, 296, ((-(0.5 * s.v[536])) * s.v[294]), 122, s.v[294], 440, s.v[294]);s.store_mul3_affine_lhs(285, 278, 270, 2.0, 0.0, 270);s.store_add_scaled_inputs_product_mixed_aaii(282, A::offset(A::mul(s.ad_value(279), s.ad_value(270)), s.v[294]), 1.0, A::mul3_scaled_output(s.ad_value(278), s.ad_value(270), s.ad_value(296), 2.0), 1.0, 285, 55, 1.0);s.store_mul3_affine_lhs(286, 270, 278, ((2.0 * s.v[294]) * 2.0), 0.0, 270);s.store_add_scaled_value_products_mixed_aiaii(283, A::offset(A::mul3(A::add_scaled_square_product(s.ad_value(279), 1.0, s.ad_value(278), s.ad_value(280), (-4.0)), s.ad_value(270), s.ad_value(270)), (s.v[294] * s.v[294])), 1.0, 270, A::add_scaled_product(s.ad_value(279), 1.0, s.ad_value(278), s.ad_value(296), 2.0), (2.0 * s.v[294]), 286, 55, 1.0);s.store_sqrt(283, 283);s.store_div_scaled_inputs_indices(286, 286, 1.0, 283, 2.0);s.store_div_from_scalar_ad(284, 1.0, A::mul3_scaled_output(s.ad_value(278), s.ad_value(270), s.ad_value(270), 2.0));s.store_mul_sub_rhs(346, 284, 282, 283);s.store_mul_sub_rhs(347, 284, 285, 286);s.store_div_scaled_inputs_indices(370, 346, -1.0, 347, 1.0);}
            s.b[727] = (((s.v[370]) as f64).abs() < 1e-12);s.store_scalar(727, if s.b[727] { 1.0 } else { 0.0 });
            if (((!s.b[725]) && s.b[726]) && s.b[727]) {s.store_scalar(62, s.v[28]);}
            s.b[728] = (s.v[370] > 0.1);s.store_scalar(728, if s.b[728] { 1.0 } else { 0.0 });
            if ((((!s.b[725]) && s.b[726]) && (!s.b[727])) && s.b[728]) {s.store_scalar(370, 0.1);}
            s.b[729] = (s.v[370] < (-0.1));s.store_scalar(729, if s.b[729] { 1.0 } else { 0.0 });
            if (((((!s.b[725]) && s.b[726]) && (!s.b[727])) && (!s.b[728])) && s.b[729]) {s.store_scalar(370, (-0.1));}
            if ((!s.b[725]) && s.b[726]) {s.store_add(55, 55, 370);s.store_primal_offset(62, 62, 1.0);}
        }
        s.b[730] = (s.v[52] < (s.v[54] + s.v[55]));s.store_scalar(730, if s.b[730] { 1.0 } else { 0.0 });
        if ((!s.b[725]) && s.b[730]) {s.store_scalar(39, 1.0);s.store_scalar(292, (-1.0));s.copy_ad(332, 334);s.store_sqrt_div_scaled_inputs(279, 332, ((2.0 * 1.034943e-10) / 1.6021918e-19), 471, 1.0);s.store_scaled_square(278, 439, (s.v[293] * s.v[293]));}
        s.b[731] = ((s.v[345] + s.v[279]) < p.p227);s.store_scalar(731, if s.b[731] { 1.0 } else { 0.0 });
        if (((!s.b[725]) && s.b[730]) && s.b[731]) {s.store_sub_from_scalar(279, (10.0 * 2.220446049250313e-16), 440);s.store_add_scaled_inputs3_mixed_aai(280, A::square(A::add_scaled_product(s.ad_value(279), 2.0, s.ad_value(278), s.ad_value(120), 1.0)), 1.0, A::square(s.ad_value(279)), (-4.0), 278, (-4.0));}
        if (((!s.b[725]) && s.b[730]) && s.b[731]) {
            if (s.v[280] >= (10.0 * 2.220446049250313e-16)) {
            } else {
                s.store_scalar(280, (10.0 * 2.220446049250313e-16));
            }
        }
        if (((!s.b[725]) && s.b[730]) && s.b[731]) {s.store_sqrt(280, 280);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_13(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((!s.b[725]) && s.b[730]) && s.b[731]) {s.store_add_scaled_product_indices(281, 279, 2.0, 278, 120, 1.0);s.store_scaled_sub(324, 281, 280, 0.5);s.store_div_ad(325, A::ln(A::div_scaled_product_by_product(s.ad_value(279), s.ad_value(279), 1.0, s.ad_value(278), s.ad_value(143), 1.0)), A::add(s.ad_value(120), A::div_from_scalar(2.0, s.ad_value(279))));}
        s.b[732] = (s.v[324] < s.v[326]);s.store_scalar(732, if s.b[732] { 1.0 } else { 0.0 });
        if ((((!s.b[725]) && s.b[730]) && s.b[731]) && s.b[732]) {s.copy_ad(307, 324);}
        if ((((!s.b[725]) && s.b[730]) && s.b[731]) && (!s.b[732])) {s.store_offset_sub(638, 325, 324, (-0.0008));s.store_scale(639, 325, (4.0 * 0.0008));}
        if ((((!s.b[725]) && s.b[730]) && s.b[731]) && (!s.b[732])) {
            if (s.v[639] > 0.0) {
            } else {
                s.store_neg(639, 639);
            }
        }
        if ((((!s.b[725]) && s.b[730]) && s.b[731]) && (!s.b[732])) {s.store_sqrt_square_add(639, 638, 639);s.store_offset_scaled_div(279, 638, 639, 0.5, 0.5);s.store_add_scaled_inputs3_indices(307, 325, 1.0, 638, (-0.5), 639, (-0.5));}
        if (((!s.b[725]) && s.b[730]) && (!s.b[731])) {s.store_add_scaled_inputs3_indices(279, 440, (-1.0), 305, (-(-1.0)), 296, (-(-(0.5 * (p.p227 * 9662367879.197212)))));s.store_add_scaled_inputs3_mixed_aai(280, A::square(A::add_scaled_product(s.ad_value(279), 2.0, s.ad_value(278), s.ad_value(120), 1.0)), 1.0, A::square(s.ad_value(279)), (-4.0), 278, (-4.0));}
        if (((!s.b[725]) && s.b[730]) && (!s.b[731])) {
            if (s.v[280] >= (10.0 * 2.220446049250313e-16)) {
            } else {
                s.store_scalar(280, (10.0 * 2.220446049250313e-16));
            }
        }
        if (((!s.b[725]) && s.b[730]) && (!s.b[731])) {s.store_sqrt(280, 280);s.store_add_scaled_product_indices(281, 279, 2.0, 278, 120, 1.0);s.store_scaled_sub(324, 281, 280, 0.5);s.store_div_ad(325, A::ln(A::div_scaled_product_by_product(s.ad_value(279), s.ad_value(279), 1.0, s.ad_value(278), s.ad_value(143), 1.0)), A::add(s.ad_value(120), A::div_from_scalar(2.0, s.ad_value(279))));}
        s.b[733] = (s.v[324] < s.v[326]);s.store_scalar(733, if s.b[733] { 1.0 } else { 0.0 });
        if ((((!s.b[725]) && s.b[730]) && (!s.b[731])) && s.b[733]) {s.copy_ad(307, 324);}
        if ((((!s.b[725]) && s.b[730]) && (!s.b[731])) && (!s.b[733])) {s.store_offset_sub(638, 325, 324, (-0.0008));s.store_scale(639, 325, (4.0 * 0.0008));}
        if ((((!s.b[725]) && s.b[730]) && (!s.b[731])) && (!s.b[733])) {
            if (s.v[639] > 0.0) {
            } else {
                s.store_neg(639, 639);
            }
        }
        if ((((!s.b[725]) && s.b[730]) && (!s.b[731])) && (!s.b[733])) {s.store_sqrt_square_add(639, 638, 639);s.store_offset_scaled_div(279, 638, 639, 0.5, 0.5);s.store_add_scaled_inputs3_indices(307, 325, 1.0, 638, (-0.5), 639, (-0.5));}
        if ((!s.b[725]) && s.b[730]) {s.store_sqrt_div_scaled_inputs(279, 332, ((2.0 * 1.034943e-10) / 1.6021918e-19), 471, 1.0);}
        s.b[734] = ((s.v[345] + s.v[279]) < p.p227);s.store_scalar(734, if s.b[734] { 1.0 } else { 0.0 });
        if (((!s.b[725]) && s.b[730]) && s.b[734]) {s.store_scalar(62, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_14(
        s: &mut ReactiveScratch,
    ) {
        let mut t7: usize = 0;
        while {
            let t6: f64 = if ((((!s.b[725]) && s.b[730]) && s.b[734]) && (s.v[62] < s.v[28])) { 1.0 } else { 0.0 };
            t6 != 0.0
        } {
            t7 += 1;assert!(t7 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((!s.b[725]) && s.b[730]) && s.b[734]) {s.copy_ad(279, 439);s.store_mul(280, 120, 307);s.store_exp_neg_input(281, 280);}
            s.b[735] = (s.v[307] > 1e-8);s.store_scalar(735, if s.b[735] { 1.0 } else { 0.0 });
            if ((((!s.b[725]) && s.b[730]) && s.b[734]) && s.b[735]) {s.store_exp_mul(278, 120, 307);s.store_mul_scaled_sqrt_ad_rhs(282, 279, -1.0, A::add_scaled_offset_product_rhs(A::offset(A::add(s.ad_value(281), s.ad_value(280)), (-1.0)), 1.0, s.ad_value(143), s.ad_value(278), (-1.0), 1.0));s.store_mul_div_scaled_inputs_mixed_aii(283, A::add_scaled_sub_value_product(1.0, s.ad_value(281), 1.0, s.ad_value(143), s.ad_value(278), 1.0), 438, 1.0, 282, 1.0);}
            s.b[736] = (s.v[307] < (-1e-8));s.store_scalar(736, if s.b[736] { 1.0 } else { 0.0 });
            if (((((!s.b[725]) && s.b[730]) && s.b[734]) && (!s.b[735])) && s.b[736]) {s.store_mul_sqrt_mixed_ia(282, 279, A::offset(A::add(s.ad_value(281), s.ad_value(280)), (-1.0)));s.store_mul_scale_offset_mixed_ai(283, A::div(s.ad_value(438), s.ad_value(282)), 281, -1.0, 1.0);}
            if (((((!s.b[725]) && s.b[730]) && s.b[734]) && (!s.b[735])) && (!s.b[736])) {s.store_mul_ad_affine_product_lhs(282, A::sqrt(A::div(s.ad_value(438), s.ad_value(120))), s.ad_value(120), -1.0, 0.0, 307);s.store_neg_ad(283, A::sqrt(A::mul(s.ad_value(438), s.ad_value(120))));}
            if (((!s.b[725]) && s.b[730]) && s.b[734]) {s.store_sqrt_square_offset(639, 282, ((4.0 * 1e-10) * 1e-10));s.store_offset_scaled_div(285, 282, 639, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(284, 282, 0.5, 639, 0.5, (1e-10 * 1e-10));}
            s.b[737] = (s.v[284] < 0.0);s.store_scalar(737, if s.b[737] { 1.0 } else { 0.0 });
            if ((((!s.b[725]) && s.b[730]) && s.b[734]) && s.b[737]) {s.store_scalar(284, 0.0);s.store_scalar(285, 0.0);}
            if (((!s.b[725]) && s.b[730]) && s.b[734]) {s.store_offset_sub_scaled_inputs_indices(638, 296, -1.0, 284, 1.0, (-1e-13));s.store_scale(639, 296, (-(4.0 * 1e-13)));}
            if (((!s.b[725]) && s.b[730]) && s.b[734]) {
                if (s.v[639] > 0.0) {
                } else {
                    s.store_neg(639, 639);
                }
            }
            if (((!s.b[725]) && s.b[730]) && s.b[734]) {s.store_sqrt_square_add(639, 638, 639);s.store_offset_scaled_div(286, 638, 639, 0.5, 0.5);s.store_add_scaled_inputs3_indices(284, 296, -1.0, 638, (-0.5), 639, (-0.5));s.store_mul3_lhs(285, 285, 283, 286);s.store_div_scaled_inputs_mixed_ai(332, A::square(s.ad_value(284)), ((0.5 * 9662367879.197212) * 6.241449993689894e18), 471, 1.0);s.store_div_scaled_product_indices(333, 332, 285, 2.0, 284, 1.0);s.store_sub_mixed_ia(284, 307, A::div_scaled_inputs4(s.ad_value(282), 1.0 / (s.v[294]), s.ad_value(307), (-1.0), s.ad_value(440), -1.0, s.ad_value(332), 1.0, A::add(A::scale_offset(s.ad_value(283), 1.0 / (s.v[294]), (-1.0)), s.ad_value(333)), 1.0));}
            s.b[738] = ((((s.v[284] - s.v[307])) as f64).abs() < 0.001);s.store_scalar(738, if s.b[738] { 1.0 } else { 0.0 });
            if ((((!s.b[725]) && s.b[730]) && s.b[734]) && s.b[738]) {s.store_scalar(62, s.v[28]);}
            if (((!s.b[725]) && s.b[730]) && s.b[734]) {s.copy_ad(307, 284);s.copy_ad(312, 282);s.store_primal_offset(62, 62, 1.0);}
        }
        if (((!s.b[725]) && s.b[730]) && (!s.b[734])) {s.store_scalar(62, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_15(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        let mut t9: usize = 0;
        while {
            let t8: f64 = if ((((!s.b[725]) && s.b[730]) && (!s.b[734])) && (s.v[62] < s.v[28])) { 1.0 } else { 0.0 };
            t8 != 0.0
        } {
            t9 += 1;assert!(t9 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((!s.b[725]) && s.b[730]) && (!s.b[734])) {s.copy_ad(279, 439);s.store_mul(280, 120, 307);s.store_exp_neg_input(281, 280);}
            s.b[739] = (s.v[307] > 1e-8);s.store_scalar(739, if s.b[739] { 1.0 } else { 0.0 });
            if ((((!s.b[725]) && s.b[730]) && (!s.b[734])) && s.b[739]) {s.store_exp_mul(278, 120, 307);s.store_mul_scaled_sqrt_ad_rhs(282, 279, -1.0, A::add_scaled_offset_product_rhs(A::offset(A::add(s.ad_value(281), s.ad_value(280)), (-1.0)), 1.0, s.ad_value(143), s.ad_value(278), (-1.0), 1.0));s.store_mul_div_scaled_inputs_mixed_aii(283, A::add_scaled_sub_value_product(1.0, s.ad_value(281), 1.0, s.ad_value(143), s.ad_value(278), 1.0), 438, 1.0, 282, 1.0);}
            s.b[740] = (s.v[307] < (-1e-8));s.store_scalar(740, if s.b[740] { 1.0 } else { 0.0 });
            if (((((!s.b[725]) && s.b[730]) && (!s.b[734])) && (!s.b[739])) && s.b[740]) {s.store_mul_sqrt_mixed_ia(282, 279, A::offset(A::add(s.ad_value(281), s.ad_value(280)), (-1.0)));s.store_mul_scale_offset_mixed_ai(283, A::div(s.ad_value(438), s.ad_value(282)), 281, -1.0, 1.0);}
            if (((((!s.b[725]) && s.b[730]) && (!s.b[734])) && (!s.b[739])) && (!s.b[740])) {s.store_mul_ad_affine_product_lhs(282, A::sqrt(A::div(s.ad_value(438), s.ad_value(120))), s.ad_value(120), -1.0, 0.0, 307);s.store_neg_ad(283, A::sqrt(A::mul(s.ad_value(438), s.ad_value(120))));}
            if (((!s.b[725]) && s.b[730]) && (!s.b[734])) {s.store_sqrt_square_offset(639, 282, ((4.0 * 1e-10) * 1e-10));s.store_offset_scaled_div(285, 282, 639, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(284, 282, 0.5, 639, 0.5, (1e-10 * 1e-10));}
            s.b[741] = (s.v[284] < 0.0);s.store_scalar(741, if s.b[741] { 1.0 } else { 0.0 });
            if ((((!s.b[725]) && s.b[730]) && (!s.b[734])) && s.b[741]) {s.store_scalar(284, 0.0);s.store_scalar(285, 0.0);}
            if (((!s.b[725]) && s.b[730]) && (!s.b[734])) {s.store_offset_sub_scaled_inputs_indices(638, 296, -1.0, 284, 1.0, (-1e-13));s.store_scale(639, 296, (-(4.0 * 1e-13)));}
            if (((!s.b[725]) && s.b[730]) && (!s.b[734])) {
                if (s.v[639] > 0.0) {
                } else {
                    s.store_neg(639, 639);
                }
            }
            if (((!s.b[725]) && s.b[730]) && (!s.b[734])) {s.store_sqrt_square_add(639, 638, 639);s.store_offset_scaled_div(286, 638, 639, 0.5, 0.5);s.store_add_scaled_inputs3_indices(284, 296, -1.0, 638, (-0.5), 639, (-0.5));s.store_mul3_lhs(285, 285, 283, 286);s.store_div_scaled_inputs_mixed_ai(332, A::square(s.ad_value(284)), ((0.5 * 9662367879.197212) * 6.241449993689894e18), 471, 1.0);s.store_div_scaled_product_indices(333, 332, 285, 2.0, 284, 1.0);s.store_sub_div_rhs_ad(284, 307, A::add(A::sub(A::add(A::add_scaled_inputs3(s.ad_value(305), 1.0, s.ad_value(307), (-1.0), s.ad_value(282), 1.0 / (s.v[294])), A::add_scaled_inputs(s.ad_value(282), (p.p227 * 9662367879.197212), s.ad_value(296), (0.5 * (p.p227 * 9662367879.197212)))), s.ad_value(440)), s.ad_value(332)), A::add_scaled_inputs3_offset(s.ad_value(283), 1.0 / (s.v[294]), s.ad_value(283), (p.p227 * 9662367879.197212), s.ad_value(333), 1.0, (-1.0)));}
            s.b[742] = ((((s.v[284] - s.v[307])) as f64).abs() < 0.001);s.store_scalar(742, if s.b[742] { 1.0 } else { 0.0 });
            if ((((!s.b[725]) && s.b[730]) && (!s.b[734])) && s.b[742]) {s.store_scalar(62, s.v[28]);}
            if (((!s.b[725]) && s.b[730]) && (!s.b[734])) {s.copy_ad(307, 284);s.copy_ad(312, 282);s.store_primal_offset(62, 62, 1.0);}
        }
        if ((!s.b[725]) && s.b[730]) {s.store_add(307, 440, 307);s.store_sub_scaled_inputs(306, 307, 1.0, 312, 1.0 / (s.v[294]));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_16(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (!s.b[725]) {s.store_offset_div_scaled_offset_numerator(290, A::mul(s.ad_value(120), A::sub(s.ad_value(76), s.ad_value(50))), 4.0, ((-1.0) * 4.0), A::mul(s.ad_value(145), s.ad_value(121)), 1.0, 1.0);}
        if (!s.b[725]) {
            if (s.v[290] >= (10.0 * 2.220446049250313e-16)) {
            } else {
                s.store_scalar(290, (10.0 * 2.220446049250313e-16));
            }
        }
        if (!s.b[725]) {s.store_add_product3_rhs_mixed_iia(319, 76, 145, 120, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(290))), 0.5);s.store_div_from_scalar(278, 1.0, 270);s.store_scalar(279, (p.p227 / 1.034943e-10));s.store_scalar(280, (1.0 / s.v[294]));s.store_div_from_scalar_ad(281, 1.0, A::add_scaled_inputs3(s.ad_value(278), 1.0, s.ad_value(279), 1.0, s.ad_value(280), 1.0));}
        s.b[743] = ((s.v[52] - s.v[327]) <= s.v[78]);s.store_scalar(743, if s.b[743] { 1.0 } else { 0.0 });
        if ((!s.b[725]) && s.b[743]) {
            if (s.v[319] > 0.0) {
                s.store_sqrt_mul_scaled_lhs(283, 471, ((1.6021918e-19 * 2.0) * 1.034943e-10), 319);
            } else {
                s.store_scalar(283, 0.0);
            }
        }
        if ((!s.b[725]) && s.b[743]) {
            if (s.v[296] <= s.v[283]) {
                s.copy_ad(283, 296);
            } else {
            }
        }
        if ((!s.b[725]) && s.b[743]) {s.store_mul_mixed_ia(282, 281, A::add_scaled_inputs_product(s.ad_value(76), 1.0, s.ad_value(440), (-1.0), A::add_scaled_inputs(s.ad_value(280), 1.0, s.ad_value(279), 0.5), s.ad_value(283), -1.0));}
        if ((!s.b[725]) && (!s.b[743])) {s.store_mul_mixed_ia(282, 281, A::add_scaled_inputs_product(s.ad_value(76), 1.0, s.ad_value(440), (-1.0), A::add_scaled_inputs(s.ad_value(280), 1.0, s.ad_value(279), 0.5), s.ad_value(296), -1.0));}
        if (!s.b[725]) {s.store_sub_div_rhs_indices(319, 76, 282, 270);s.copy_ad(321, 319);}
        s.b[744] = ((s.v[52] - s.v[327]) > s.v[78]);s.store_scalar(744, if s.b[744] { 1.0 } else { 0.0 });
        if ((!s.b[725]) && s.b[744]) {s.store_div_scalar_by_product_indices(279, 1.0, 142, 381, 1.0);s.store_mul_ad_product_rhs(280, 279, A::sub(s.ad_value(76), s.ad_value(327)), A::sub(s.ad_value(76), s.ad_value(327)));s.store_add_mixed_ia(281, 120, A::div_from_scalar(2.0, A::sub(s.ad_value(76), s.ad_value(327))));s.store_div_ln_lhs(320, 280, 281);}
        s.b[745] = ((s.v[319] > (s.v[320] - 0.15)) && (0.15 >= 0.0));s.store_scalar(745, if s.b[745] { 1.0 } else { 0.0 });
        if (((!s.b[725]) && s.b[744]) && s.b[745]) {s.store_offset_sub(638, 319, 320, 0.15);s.store_square(642, 638);s.store_scalar(643, (0.15 * 0.15));s.store_scalar(644, 1.0);s.store_scalar(645, 1.0);s.store_scalar(647, 0.0);s.store_scalar(648, 0.0);s.store_scalar(220, 0.0);s.store_scalar(646, 0.0);s.store_mul(644, 644, 642);s.store_mul(645, 645, 643);s.store_add(220, 644, 645);s.copy_ad(646, 220);}
        s.b[746] = ((((1.0 == 1.0) || (1.0 == 2.0)) || (1.0 == 4.0)) || (1.0 == 8.0));s.store_scalar(746, if s.b[746] { 1.0 } else { 0.0 });s.b[747] = (1.0 == 1.0);s.store_scalar(747, if s.b[747] { 1.0 } else { 0.0 });
        if (((((!s.b[725]) && s.b[744]) && s.b[745]) && s.b[746]) && s.b[747]) {s.store_scalar(648, 1.0);}
        s.b[748] = (1.0 == 2.0);s.store_scalar(748, if s.b[748] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_17(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((((((!s.b[725]) && s.b[744]) && s.b[745]) && s.b[746]) && (!s.b[747])) && s.b[748]) {s.store_scalar(648, 2.0);}
        s.b[749] = (1.0 == 4.0);s.store_scalar(749, if s.b[749] { 1.0 } else { 0.0 });
        if (((((((!s.b[725]) && s.b[744]) && s.b[745]) && s.b[746]) && (!s.b[747])) && (!s.b[748])) && s.b[749]) {s.store_scalar(648, 3.0);}
        s.b[750] = (1.0 == 8.0);s.store_scalar(750, if s.b[750] { 1.0 } else { 0.0 });
        if ((((((((!s.b[725]) && s.b[744]) && s.b[745]) && s.b[746]) && (!s.b[747])) && (!s.b[748])) && (!s.b[749])) && s.b[750]) {s.store_scalar(648, 4.0);}
        if ((((!s.b[725]) && s.b[744]) && s.b[745]) && s.b[746]) {s.store_scalar(647, 0.0);}
        let mut tb: usize = 0;
        while {
            let ta: f64 = if (((((!s.b[725]) && s.b[744]) && s.b[745]) && s.b[746]) && (s.v[647] < s.v[648])) { 1.0 } else { 0.0 };
            ta != 0.0
        } {
            tb += 1;assert!(tb <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((!s.b[725]) && s.b[744]) && s.b[745]) && s.b[746]) {s.store_sqrt(646, 646);s.store_primal_offset(647, 647, 1.0);}
        }
        if ((((!s.b[725]) && s.b[744]) && s.b[745]) && (!s.b[746])) {s.store_powf(646, 646, (1.0 / 2.0));}
        if (((!s.b[725]) && s.b[744]) && s.b[745]) {s.store_div_from_scalar_offset_input(646, 1.0, 646, 1e-50);s.store_scaled_mul(637, 638, 646, 0.15);s.store_div_scaled_product_offset_denominator_indices(279, 645, 646, 0.15, 220, 1e-50, 1.0);s.store_add_offset_lhs(321, 320, (-0.15), 637);}
        if (((!s.b[725]) && s.b[744]) && s.b[745]) {
        }
        if (((!s.b[725]) && s.b[744]) && (!s.b[745])) {s.copy_ad(321, 319);s.store_scalar(279, 1.0);}
        if (!s.b[725]) {
            if (s.v[321] > 0.0) {
                s.store_sqrt_div_scaled_inputs(345, 321, ((2.0 * 1.034943e-10) / 1.6021918e-19), 471, 1.0);
            } else {
                s.store_scalar(345, 0.0);
            }
        }
        s.b[751] = (s.v[345] < p.p227);s.store_scalar(751, if s.b[751] { 1.0 } else { 0.0 });
        if ((!s.b[725]) && s.b[751]) {s.store_scalar(39, 1.0);}
        if ((!s.b[725]) && (!s.b[751])) {s.store_scalar(39, 2.0);}
        if (!s.b[725]) {s.copy_ad(305, 321);s.copy_ad(58, 319);s.store_scaled_square(278, 439, (s.v[293] * s.v[293]));}
        s.b[752] = (s.v[39] == 1.0);s.store_scalar(752, if s.b[752] { 1.0 } else { 0.0 });
        if ((!s.b[725]) && s.b[752]) {s.store_neg(279, 440);s.store_add_scaled_inputs3_mixed_aai(280, A::square(A::add_scaled_product(s.ad_value(279), 2.0, s.ad_value(278), s.ad_value(120), 1.0)), 1.0, A::square(s.ad_value(279)), (-4.0), 278, (-4.0));}
        if ((!s.b[725]) && s.b[752]) {
            if (s.v[280] >= (10.0 * 2.220446049250313e-16)) {
            } else {
                s.store_scalar(280, (10.0 * 2.220446049250313e-16));
            }
        }
        if ((!s.b[725]) && s.b[752]) {s.store_sqrt(280, 280);s.store_add_scaled_product_indices(281, 279, 2.0, 278, 120, 1.0);s.store_scaled_sub(324, 281, 280, 0.5);s.store_div_ad(325, A::ln(A::div_scaled_product_by_product(s.ad_value(279), s.ad_value(279), 1.0, s.ad_value(278), s.ad_value(143), 1.0)), A::add(s.ad_value(120), A::div_from_scalar(2.0, s.ad_value(279))));}
        s.b[753] = (s.v[324] < s.v[326]);s.store_scalar(753, if s.b[753] { 1.0 } else { 0.0 });
        if (((!s.b[725]) && s.b[752]) && s.b[753]) {s.copy_ad(307, 324);}
        if (((!s.b[725]) && s.b[752]) && (!s.b[753])) {s.store_offset_sub(638, 325, 324, (-0.0008));s.store_scale(639, 325, (4.0 * 0.0008));}
        if (((!s.b[725]) && s.b[752]) && (!s.b[753])) {
            if (s.v[639] > 0.0) {
            } else {
                s.store_neg(639, 639);
            }
        }
        if (((!s.b[725]) && s.b[752]) && (!s.b[753])) {s.store_sqrt_square_add(639, 638, 639);s.store_offset_scaled_div(279, 638, 639, 0.5, 0.5);s.store_add_scaled_inputs3_indices(307, 325, 1.0, 638, (-0.5), 639, (-0.5));}
        if ((!s.b[725]) && (!s.b[752])) {s.store_add_scaled_inputs3_indices(279, 440, (-1.0), 305, (-(-1.0)), 296, (-(-(0.5 * (p.p227 * 9662367879.197212)))));s.store_add_scaled_inputs3_mixed_aai(280, A::square(A::add_scaled_product(s.ad_value(279), 2.0, s.ad_value(278), s.ad_value(120), 1.0)), 1.0, A::square(s.ad_value(279)), (-4.0), 278, (-4.0));}
        if ((!s.b[725]) && (!s.b[752])) {
            if (s.v[280] >= (10.0 * 2.220446049250313e-16)) {
            } else {
                s.store_scalar(280, (10.0 * 2.220446049250313e-16));
            }
        }
        if ((!s.b[725]) && (!s.b[752])) {s.store_sqrt(280, 280);s.store_add_scaled_product_indices(281, 279, 2.0, 278, 120, 1.0);s.store_scaled_sub(324, 281, 280, 0.5);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_18(
        s: &mut ReactiveScratch,
    ) {
        if ((!s.b[725]) && (!s.b[752])) {s.store_div_ad(325, A::ln(A::div_scaled_product_by_product(s.ad_value(279), s.ad_value(279), 1.0, s.ad_value(278), s.ad_value(143), 1.0)), A::add(s.ad_value(120), A::div_from_scalar(2.0, s.ad_value(279))));}
        s.b[754] = (s.v[324] < s.v[326]);s.store_scalar(754, if s.b[754] { 1.0 } else { 0.0 });
        if (((!s.b[725]) && (!s.b[752])) && s.b[754]) {s.copy_ad(307, 324);}
        if (((!s.b[725]) && (!s.b[752])) && (!s.b[754])) {s.store_offset_sub(638, 325, 324, (-0.0008));s.store_scale(639, 325, (4.0 * 0.0008));}
        if (((!s.b[725]) && (!s.b[752])) && (!s.b[754])) {
            if (s.v[639] > 0.0) {
            } else {
                s.store_neg(639, 639);
            }
        }
        if (((!s.b[725]) && (!s.b[752])) && (!s.b[754])) {s.store_sqrt_square_add(639, 638, 639);s.store_offset_scaled_div(279, 638, 639, 0.5, 0.5);s.store_add_scaled_inputs3_indices(307, 325, 1.0, 638, (-0.5), 639, (-0.5));}
        s.b[755] = ((s.v[39] == 1.0) && (0.0 != 0.0));s.store_scalar(755, if s.b[755] { 1.0 } else { 0.0 });
        if ((!s.b[725]) && s.b[755]) {s.store_scalar(39, 1.0);s.store_scalar(62, 0.0);}
        let mut td: usize = 0;
        while {
            let tc: f64 = if (((!s.b[725]) && s.b[755]) && (s.v[62] < s.v[28])) { 1.0 } else { 0.0 };
            tc != 0.0
        } {
            td += 1;assert!(td <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((!s.b[725]) && s.b[755]) {s.copy_ad(279, 439);s.store_mul(280, 120, 307);s.store_exp_neg_input(281, 280);}
            s.b[756] = (s.v[307] > 1e-8);s.store_scalar(756, if s.b[756] { 1.0 } else { 0.0 });
            if (((!s.b[725]) && s.b[755]) && s.b[756]) {s.store_exp_mul(278, 120, 307);s.store_mul_scaled_sqrt_ad_rhs(282, 279, -1.0, A::add_scaled_offset_product_rhs(A::offset(A::add(s.ad_value(281), s.ad_value(280)), (-1.0)), 1.0, s.ad_value(143), s.ad_value(278), (-1.0), 1.0));s.store_mul_div_scaled_inputs_mixed_aii(283, A::add_scaled_sub_value_product(1.0, s.ad_value(281), 1.0, s.ad_value(143), s.ad_value(278), 1.0), 438, 1.0, 282, 1.0);}
            s.b[757] = (s.v[307] < (-1e-8));s.store_scalar(757, if s.b[757] { 1.0 } else { 0.0 });
            if ((((!s.b[725]) && s.b[755]) && (!s.b[756])) && s.b[757]) {s.store_mul_sqrt_mixed_ia(282, 279, A::offset(A::add(s.ad_value(281), s.ad_value(280)), (-1.0)));s.store_mul_scale_offset_mixed_ai(283, A::div(s.ad_value(438), s.ad_value(282)), 281, -1.0, 1.0);}
            if ((((!s.b[725]) && s.b[755]) && (!s.b[756])) && (!s.b[757])) {s.store_mul_ad_affine_product_lhs(282, A::sqrt(A::div(s.ad_value(438), s.ad_value(120))), s.ad_value(120), -1.0, 0.0, 307);s.store_neg_ad(283, A::sqrt(A::mul(s.ad_value(438), s.ad_value(120))));}
            if ((!s.b[725]) && s.b[755]) {s.store_sub_mixed_ia(284, 307, A::div_scaled_inputs3(s.ad_value(282), 1.0 / (s.v[294]), s.ad_value(307), (-1.0), s.ad_value(440), -1.0, A::scale_offset(s.ad_value(283), 1.0 / (s.v[294]), (-1.0)), 1.0));}
            s.b[758] = ((((s.v[284] - s.v[307])) as f64).abs() < 0.001);s.store_scalar(758, if s.b[758] { 1.0 } else { 0.0 });
            if (((!s.b[725]) && s.b[755]) && s.b[758]) {s.copy_ad(285, 62);s.store_scalar(62, s.v[28]);}
            if ((!s.b[725]) && s.b[755]) {s.copy_ad(307, 284);s.copy_ad(312, 282);s.store_primal_offset(62, 62, 1.0);}
        }
        if ((!s.b[725]) && s.b[755]) {s.store_add(307, 440, 307);s.store_sub_scaled_inputs(306, 307, 1.0, 312, 1.0 / (s.v[294]));}
        if ((!s.b[725]) && (!s.b[755])) {s.store_scalar(39, 2.0);}
        s.b[759] = (0.0 == 0.0);s.store_scalar(759, if s.b[759] { 1.0 } else { 0.0 });
        if (((!s.b[725]) && (!s.b[755])) && s.b[759]) {s.store_scalar(315, (1e-12 * 100.0));s.copy_ad(56, 319);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_19(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((!s.b[725]) && (!s.b[755])) && (!s.b[759])) {s.store_scalar(315, 0.001);s.copy_ad(56, 305);}
        if ((!s.b[725]) && (!s.b[755])) {s.store_scalar(62, 0.0);}
        let mut tf: usize = 0;
        while {
            let te: f64 = if (((!s.b[725]) && (!s.b[755])) && (s.v[62] < s.v[28])) { 1.0 } else { 0.0 };
            te != 0.0
        } {
            tf += 1;assert!(tf <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((!s.b[725]) && (!s.b[755])) {s.copy_ad(279, 439);s.store_mul(280, 120, 307);s.store_exp_neg_input(281, 280);}
            s.b[760] = (s.v[307] > 1e-8);s.store_scalar(760, if s.b[760] { 1.0 } else { 0.0 });
            if (((!s.b[725]) && (!s.b[755])) && s.b[760]) {s.store_exp_mul(278, 120, 307);s.store_mul_scaled_sqrt_ad_rhs(282, 279, -1.0, A::add_scaled_offset_product_rhs(A::offset(A::add(s.ad_value(281), s.ad_value(280)), (-1.0)), 1.0, s.ad_value(143), s.ad_value(278), (-1.0), 1.0));s.store_mul_div_scaled_inputs_mixed_aii(283, A::add_scaled_sub_value_product(1.0, s.ad_value(281), 1.0, s.ad_value(143), s.ad_value(278), 1.0), 438, 1.0, 282, 1.0);}
            s.b[761] = (s.v[307] < (-1e-8));s.store_scalar(761, if s.b[761] { 1.0 } else { 0.0 });
            if ((((!s.b[725]) && (!s.b[755])) && (!s.b[760])) && s.b[761]) {s.store_mul_sqrt_mixed_ia(282, 279, A::offset(A::add(s.ad_value(281), s.ad_value(280)), (-1.0)));s.store_mul_scale_offset_mixed_ai(283, A::div(s.ad_value(438), s.ad_value(282)), 281, -1.0, 1.0);}
            if ((((!s.b[725]) && (!s.b[755])) && (!s.b[760])) && (!s.b[761])) {s.store_mul_ad_affine_product_lhs(282, A::sqrt(A::div(s.ad_value(438), s.ad_value(120))), s.ad_value(120), -1.0, 0.0, 307);s.store_neg_ad(283, A::sqrt(A::mul(s.ad_value(438), s.ad_value(120))));}
            if ((!s.b[725]) && (!s.b[755])) {s.store_sub_div_rhs_ad(284, 307, A::sub(A::add(A::add_scaled_inputs3(s.ad_value(56), 1.0, s.ad_value(307), (-1.0), s.ad_value(282), 1.0 / (s.v[294])), A::add_scaled_inputs(s.ad_value(282), (p.p227 * 9662367879.197212), s.ad_value(296), (0.5 * (p.p227 * 9662367879.197212)))), s.ad_value(440)), A::add_scaled_inputs(A::scale_offset(s.ad_value(283), 1.0 / (s.v[294]), (-1.0)), 1.0, s.ad_value(283), (p.p227 * 9662367879.197212)));}
            s.b[762] = ((((s.v[284] - s.v[307])) as f64).abs() < s.v[315]);s.store_scalar(762, if s.b[762] { 1.0 } else { 0.0 });
            if (((!s.b[725]) && (!s.b[755])) && s.b[762]) {s.copy_ad(285, 62);s.store_scalar(62, s.v[28]);}
            if ((!s.b[725]) && (!s.b[755])) {s.copy_ad(307, 284);s.copy_ad(312, 282);s.store_primal_offset(62, 62, 1.0);}
        }
        s.b[763] = (0.0 == 0.0);s.store_scalar(763, if s.b[763] { 1.0 } else { 0.0 });
        if (((!s.b[725]) && (!s.b[755])) && s.b[763]) {s.copy_ad(316, 312);}
        s.b[764] = (1.0 == 0.0);s.store_scalar(764, if s.b[764] { 1.0 } else { 0.0 });
        if (((!s.b[725]) && (!s.b[755])) && s.b[764]) {s.store_scalar(315, (1e-12 * 100.0));s.copy_ad(56, 319);}
        if (((!s.b[725]) && (!s.b[755])) && (!s.b[764])) {s.store_scalar(315, 0.001);s.copy_ad(56, 305);}
        if ((!s.b[725]) && (!s.b[755])) {s.store_scalar(62, 0.0);}
    }
}
