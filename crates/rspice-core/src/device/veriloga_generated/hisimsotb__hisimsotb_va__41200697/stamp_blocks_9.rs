#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_2(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.store_sqrt_mul_scaled_lhs(439, 438, 2.0, 122);

        s.store_div(279, 127, 471);

        s.store_square(142, 279);

        s.store_div(279, 127, 462);

        s.store_square(143, 279);

        s.store_scalar(272, p.p226);

        s.store_scalar(273, (3.453133e-11 / s.v[272]));

        s.store_scalar(274, (s.v[272] / 3.453133e-11));

        s.store_scalar(294, (3.453133e-11 / p.p229));

        s.store_scalar(295, (p.p229 / 3.453133e-11));

        s.store_scale(296, 471, ((-1.6021918e-19) * p.p227));

        s.store_scalar(535, (1.034943e-10 / p.p227));

        s.store_scalar(536, (1.0 / s.v[535]));

        s.store_scalar(293, (s.v[295] + s.v[536]));

        s.store_scalar(31, p.p254);

        s.store_scalar(30, p.p255);

        s.b[688] = (s.v[31] > (s.v[30] * 0.5));
        s.store_scalar(688, if s.b[688] { 1.0 } else { 0.0 });

        if s.b[688] {
            s.store_scalar(31, (0.5 * s.v[30]));
        }

        s.b[689] = (s.v[47] > s.v[31]);
        s.store_scalar(689, if s.b[689] { 1.0 } else { 0.0 });

        if s.b[689] {
            s.store_sub(280, 47, 31);
            s.store_sub_from_scalar(281, s.v[30], 31);
            s.store_square(642, 280);
            s.store_square(643, 281);
            s.store_scalar(644, 1.0);
            s.store_scalar(645, 1.0);
            s.store_scalar(647, 0.0);
            s.store_scalar(648, 0.0);
            s.store_scalar(220, 0.0);
            s.store_scalar(646, 0.0);
            s.store_mul(644, 644, 642);
            s.store_mul(645, 645, 643);
            s.store_mul(644, 644, 642);
            s.store_mul(645, 645, 643);
            s.store_mul(644, 644, 642);
            s.store_mul(645, 645, 643);
            s.store_mul(644, 644, 642);
            s.store_mul(645, 645, 643);
            s.store_add(220, 644, 645);
            s.copy_ad(646, 220);
        }

        s.b[690] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));
        s.store_scalar(690, if s.b[690] { 1.0 } else { 0.0 });

        s.b[691] = (4.0 == 1.0);
        s.store_scalar(691, if s.b[691] { 1.0 } else { 0.0 });

        if ((s.b[689] && s.b[690]) && s.b[691]) {
            s.store_scalar(648, 1.0);
        }

        s.b[692] = (4.0 == 2.0);
        s.store_scalar(692, if s.b[692] { 1.0 } else { 0.0 });

        if (((s.b[689] && s.b[690]) && (!s.b[691])) && s.b[692]) {
            s.store_scalar(648, 2.0);
        }

        s.b[693] = (4.0 == 4.0);
        s.store_scalar(693, if s.b[693] { 1.0 } else { 0.0 });

        if ((((s.b[689] && s.b[690]) && (!s.b[691])) && (!s.b[692])) && s.b[693]) {
            s.store_scalar(648, 3.0);
        }

        s.b[694] = (4.0 == 8.0);
        s.store_scalar(694, if s.b[694] { 1.0 } else { 0.0 });

        if (((((s.b[689] && s.b[690]) && (!s.b[691])) && (!s.b[692])) && (!s.b[693])) && s.b[694]) {
            s.store_scalar(648, 4.0);
        }

        if (s.b[689] && s.b[690]) {
            s.store_scalar(647, 0.0);
        }

        let mut assign4560_loop_guard: usize = 0;
        while {
            let assign4560_cond_e3027: f64 = if ((s.b[689] && s.b[690]) && (s.v[647] < s.v[648])) { 1.0 } else { 0.0 };
            assign4560_cond_e3027 != 0.0
        } {
            assign4560_loop_guard += 1;
            assert!(assign4560_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (s.b[689] && s.b[690]) {
                s.store_sqrt(646, 646);
                s.store_offset(647, 647, 1.0);
            }
        }

        if (s.b[689] && (!s.b[690])) {
            s.store_powf(646, 646, (1.0 / (2.0 * 4.0)));
        }

        if s.b[689] {
            s.store_div_from_scalar_offset_input(646, 1.0, 646, 1e-50);
            s.store_mul3_lhs(282, 280, 281, 646);
            s.store_div_scaled_product3_mixed_iiia(286, 281, 645, 646, 1.0, A::offset(s.ad_value(220), 1e-50), 1.0);
            s.store_add(43, 31, 282);
            s.copy_ad(46, 286);
        }

        if (!s.b[689]) {
            s.copy_ad(43, 47);
            s.store_scalar(46, 1.0);
        }

        s.copy_ad(44, 48);

        s.copy_ad(45, 49);

        s.store_scalar(33, 0.0);

        s.store_scalar(695, 0.0);

        s.store_scalar(696, 0.0);

        s.store_scalar(697, 0.0);

        s.store_scalar(698, 0.0);

        s.store_scalar(699, 0.0);

        s.store_scalar(700, 0.0);

        s.copy_ad(50, 43);

        s.copy_ad(51, 44);

        s.copy_ad(52, 45);

        s.store_scalar(62, 0.0);

        s.store_scalar(63, 0.0);

        s.store_scaled_mul(279, 46, 51, 0.5);

        s.store_scale(638, 279, (2.0 * 1.0 / (p.p216)));

        s.store_offset_mul_offset_rhs_ad_rhs(639, 638, A::mul_offset_rhs(s.ad_value(638), A::mul_offset_rhs(s.ad_value(638), A::mul_offset_rhs(s.ad_value(638), A::mul(s.ad_value(638), A::scale_offset(s.ad_value(638), (1.0 / 5040.0), (1.0 / 720.0))), (1.0 / 120.0)), (1.0 / 24.0)), (1.0 / 6.0)), (1.0 / 2.0), 1.0);

        s.store_offset_mul_offset_rhs_ad_rhs(640, 638, A::mul_offset_rhs(s.ad_value(638), A::mul_offset_rhs(s.ad_value(638), A::mul(s.ad_value(638), A::scale_offset(s.ad_value(638), (1.0 / 840.0), (1.0 / 144.0))), (1.0 / 30.0)), (1.0 / 8.0)), (1.0 / 3.0), (1.0 / 2.0));

        s.store_div_from_scalar(73, p.p216, 639);

        s.store_div_scaled_inputs_square_rhs(280, 640, (-2.0), 639, 1.0);

        s.b[701] = (s.v[73] < 1e-12);
        s.store_scalar(701, if s.b[701] { 1.0 } else { 0.0 });

        if s.b[701] {
            s.store_scalar(73, 1e-12);
        }

        s.store_add(70, 50, 73);

        s.store_add_scaled_inputs(71, 51, 1.0, 73, 2.0);

        s.store_add(72, 52, 73);

        s.store_scale(279, 126, (2.0 * (1.034943e-10 * (s.v[274] * s.v[274]))));

        s.store_sub(280, 52, 138);

        s.store_offset_mul_ad(281, A::div_from_scalar(2.0, s.ad_value(279)), A::add_scaled_inputs3(s.ad_value(280), 1.0, s.ad_value(122), (-1.0), s.ad_value(50), -1.0), 1.0);

        s.store_sqrt_square_offset(639, 281, ((4.0 * 0.001) * 0.001));

        s.store_offset_scaled_div(283, 281, 639, 0.5, 0.5);

        s.store_offset_add_scaled_inputs_indices(282, 281, 0.5, 639, 0.5, (1e-10 * 0.001));

        s.b[702] = (s.v[282] < 0.0);
        s.store_scalar(702, if s.b[702] { 1.0 } else { 0.0 });

        if s.b[702] {
            s.store_scalar(282, 0.0);
            s.store_scalar(283, 0.0);
        }

        s.store_sqrt_offset_input(290, 282, 1e-50);

        s.store_add_mul_sub_from_scalar_rhs_indices(87, 280, 279, 1.0, 290);

        s.store_sub(88, 87, 128);

        s.store_offset(638, 88, (((-0.1)) + ((-0.05))));

        s.store_scalar(639, ((4.0 * 0.1) * 0.05));

        if (!(s.v[639] > 0.0)) {
            s.store_scalar(639, (-s.v[639]));
        }

        s.store_sqrt_square_add(639, 638, 639);

        s.store_offset_scaled_div(284, 638, 639, 0.5, 0.5);

        s.store_offset_add_scaled_inputs_indices(88, 638, 0.5, 639, 0.5, 0.1);

        s.store_div(279, 51, 88);

        s.copy_ad(638, 279);

        s.store_square(639, 638);

        s.store_mul(640, 639, 638);

        s.store_square(641, 639);

        s.store_div_from_scalar_ad(290, 1.0, A::add_scaled_inputs4_offset(s.ad_value(638), 1.0, s.ad_value(639), 1.0, s.ad_value(640), 1.0, s.ad_value(641), 1.0, 1.0));

        s.store_mul_ad_affine_product_lhs(278, A::add_scaled_inputs3_offset(s.ad_value(638), 2.0, s.ad_value(639), 3.0, s.ad_value(640), 4.0, 1.0), s.ad_value(290), -1.0, 0.0, 290);

        s.store_sub_from_scalar(290, 1.0, 290);

        s.store_neg(278, 278);

        s.store_square(276, 290);

        s.b[703] = (((p.p193 == 0.0) && (p.p195 == 0.0)) || (p.p194 == 0.0));
        s.store_scalar(703, if s.b[703] { 1.0 } else { 0.0 });

        if s.b[703] {
            s.store_scalar(37, 0.0);
        }

        if (!s.b[703]) {
            s.store_scalar(37, 1.0);
        }

        s.store_add_scaled_inputs3_sqrt_third_mixed_iia(275, 129, 1.0, 138, 1.0, A::mul_scaled_lhs(s.ad_value(126), (2.0 * 1.034943e-10), s.ad_value(129)), 1.0 / (s.v[273]));

        s.b[704] = (s.v[37] == 0.0);
        s.store_scalar(704, if s.b[704] { 1.0 } else { 0.0 });

        if s.b[704] {
            s.store_scalar(268, s.v[272]);
            s.store_scalar(270, s.v[273]);
            s.store_scalar(271, s.v[274]);
            s.store_scale(278, 141, (s.v[274] * s.v[274]));
            s.store_mul(381, 278, 141);
        }

        if (!s.b[704]) {
            s.store_add_scaled_inputs3_offset_indices(283, 52, 1.0, 50, (-1.0), 275, -1.0, p.p194);
            s.store_sqrt_square_offset(639, 283, ((4.0 * 0.0001) * 0.0001));
            s.store_offset_scaled_div(281, 283, 639, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(280, 283, 0.5, 639, 0.5, (1e-10 * 0.0001));
        }

        s.b[705] = (s.v[280] < 0.0);
        s.store_scalar(705, if s.b[705] { 1.0 } else { 0.0 });

        if ((!s.b[704]) && s.b[705]) {
            s.store_scalar(280, 0.0);
            s.store_scalar(281, 0.0);
        }

        if (!s.b[704]) {
            s.store_div_from_scalar(281, 1.0, 280);
            s.store_scaled_abs(282, 275, 2.0);
            s.store_offset_sub(284, 138, 275, p.p194);
        }

        s.b[706] = (s.v[284] > s.v[282]);
        s.store_scalar(706, if s.b[706] { 1.0 } else { 0.0 });

        if ((!s.b[704]) && s.b[706]) {
            s.copy_ad(282, 284);
        }

        if (!s.b[704]) {
            s.store_offset_sub_ad(638, A::div_from_scalar(1.0, s.ad_value(282)), s.ad_value(281), (-0.0001));
            s.store_scale_ad(639, A::div_from_scalar(1.0, s.ad_value(282)), (4.0 * 0.0001));
        }

        if (!s.b[704]) {
            if (s.v[639] > 0.0) {
            } else {
                s.store_neg(639, 639);
            }
        }

        if (!s.b[704]) {
            s.store_sqrt_square_add(639, 638, 639);
            s.store_offset_scaled_div(284, 638, 639, 0.5, 0.5);
            s.store_add_scaled_inputs3_mixed_aii(280, A::div_from_scalar(1.0, s.ad_value(282)), 1.0, 638, (-0.5), 639, (-0.5));
            s.store_offset_scaled(269, 280, p.p193, p.p195);
        }

        s.b[707] = ((s.v[269] * 1000000000000.0) < s.v[272]);
        s.store_scalar(707, if s.b[707] { 1.0 } else { 0.0 });

        if ((!s.b[704]) && s.b[707]) {
            s.store_scalar(269, 0.0);
            s.store_scalar(37, 0.0);
        }

        if (!s.b[704]) {
            s.store_offset(268, 269, s.v[272]);
            s.store_div_from_scalar(270, 3.453133e-11, 268);
            s.store_scale(271, 268, 28959208927.08158);
            s.store_mul_ad_product_lhs_mixed_ai(381, A::square(s.ad_value(141)), 271, 271);
        }

        s.store_offset_sub_from_scalar_ad(638, 0.5, s.ad_value(70), (-0.001));

        s.store_scalar(639, ((4.0 * 0.5) * 0.001));

        if (!(s.v[639] > 0.0)) {
            s.store_scalar(639, (-s.v[639]));
        }

        s.store_sqrt_square_add(639, 638, 639);

        s.store_offset_scaled_div(278, 638, 639, 0.5, 0.5);

        s.store_offset_add_scaled_inputs_indices(382, 638, (-0.5), 639, (-0.5), 0.5);

        s.store_sqrt_mul(150, 473, 129);

        s.store_add_ad_lhs(265, A::add_scaled_inputs_product(s.ad_value(129), 1.0, s.ad_value(138), 1.0, s.ad_value(150), s.ad_value(271), 1.0), 380);

        s.copy_ad(130, 129);

        s.store_scalar(278, 0.95);

        s.store_offset_sub_scaled_inputs_indices(279, 130, s.v[278], 382, 1.0, (-0.001));

        s.store_sqrt_add_scaled_square_input(280, 279, 1.0, 130, ((4.0 * s.v[278]) * 0.001));

        s.store_add_scaled_inputs4_indices(131, 130, 1.0, 130, (-s.v[278]), 279, (-(-0.5)), 280, (-(-0.5)));

        s.store_sqrt(135, 131);

        s.b[708] = (p.p58 != 0.0);
        s.store_scalar(708, if s.b[708] { 1.0 } else { 0.0 });

        if s.b[708] {
            s.store_sqrt_mul_scaled_lhs(278, 471, ((2.0 * 1.6021918e-19) * 1.034943e-10), 136);
            s.store_add_scaled_inputs_product_indices(79, 136, 1.0, 138, 1.0, 278, 271, 1.0);
            s.store_scalar(278, ((2.0 * p.p227) / (p.p58 * p.p58)));
            s.store_mul_ad_affine_product_rhs(81, 271, s.ad_value(278), A::sub_from_scalar(p.p55, s.ad_value(130)), 1.034943e-10, 0.0);
        }

    }

    pub(super) fn stamp_reactive_block_3(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[708] {
            s.store_add_scaled_ad_lhs(278, A::scale_offset(s.ad_value(131), (p.p68 / p.p58), p.p66), 71, p.p67);
            s.store_mul_ad_product_lhs_mixed_ai(266, A::sub(s.ad_value(265), s.ad_value(79)), 81, 278);
        }

        if (!s.b[708]) {
            s.store_scalar(266, 0.0);
        }

        s.b[709] = (p.p297 != 0.0);
        s.store_scalar(709, if s.b[709] { 1.0 } else { 0.0 });

        if s.b[709] {
            s.store_offset_add_ad(288, A::add_scaled_product(s.ad_value(122), 1.0, s.ad_value(381), s.ad_value(120), (-0.25)), s.ad_value(138), 1e-50);
            s.store_offset_sub(279, 72, 288, (-0.005));
        }

        if s.b[709] {
            s.store_scalar(278, (if (s.v[288] >= 0.0) { 1.0 } else { (-1.0) }));
        }

        if s.b[709] {
            s.store_sqrt_add_scaled_square_product(280, 279, 1.0, 278, 288, (4.0 * 0.005));
            s.store_add_scaled_inputs4_indices(281, 288, 1.0, 279, 0.5, 280, 0.5, 138, -1.0);
            s.store_mul_ad_product_lhs_mixed_ai(282, A::div_from_scalar(4.0, s.ad_value(381)), 122, 122);
            s.store_offset_mul(283, 120, 281, (-1.0));
            s.store_offset_mul(279, 283, 282, 1.0);
            s.store_sqrt_square_offset(639, 279, ((4.0 * 0.001) * 0.001));
            s.store_offset_scaled_div(285, 279, 639, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(279, 279, 0.5, 639, 0.5, (1e-10 * 0.001));
        }

        s.b[710] = (s.v[279] < 0.0);
        s.store_scalar(710, if s.b[710] { 1.0 } else { 0.0 });

        if (s.b[709] && s.b[710]) {
            s.store_scalar(279, 0.0);
            s.store_scalar(285, 0.0);
        }

        if s.b[709] {
            s.store_sqrt_offset_input(280, 279, (10.0 * 2.220446049250313e-16));
            s.store_add_product3_rhs_mixed_iia(139, 281, 381, 120, A::sub_from_scalar(1.0, s.ad_value(280)), 0.5);
            s.store_offset_sub(638, 129, 139, (-0.005));
            s.store_scale(639, 129, (4.0 * 0.005));
        }

        if s.b[709] {
            if (s.v[639] > 0.0) {
            } else {
                s.store_neg(639, 639);
            }
        }

        if s.b[709] {
            s.store_sqrt_square_add(639, 638, 639);
            s.store_offset_scaled_div(280, 638, 639, 0.5, 0.5);
            s.store_add_scaled_inputs3_indices(140, 129, 1.0, 638, (-0.5), 639, (-0.5));
            s.store_add_scaled_inputs3_indices(130, 129, 1.0, 140, p.p297, 129, (-p.p297));
        }

        s.store_scale(279, 271, (1.034943e-10 * (p.p227 * 2.0)));

        s.store_sub_from_scalar(280, p.p55, 130);

        s.store_scalar(281, (s.v[277] - p.p57));

        s.store_scaled_mul(81, 279, 280, 1.0 / ((s.v[281] * s.v[281])));

        s.store_sqrt_square_offset(639, 50, ((4.0 * 0.001) * 0.001));

        s.store_offset_scaled_div(278, 50, 639, 0.5, 0.5);

        s.store_offset_add_scaled_inputs_indices(593, 50, 0.5, 639, 0.5, (1e-10 * 0.001));

        s.b[711] = (s.v[593] < 0.0);
        s.store_scalar(711, if s.b[711] { 1.0 } else { 0.0 });

        if s.b[711] {
            s.store_scalar(593, 0.0);
            s.store_scalar(278, 0.0);
        }

        s.store_add_scaled_inputs3_offset_indices(283, 131, (p.p71 / s.v[277]), 71, p.p70, 593, p.p250, p.p69);

        s.store_mul(82, 81, 283);

        s.b[712] = (p.p72 > 0.0);
        s.store_scalar(712, if s.b[712] { 1.0 } else { 0.0 });

        if s.b[712] {
            s.store_add_scaled_inputs3_offset_indices(279, 137, 1.0, 128, 1.0, 71, p.p73, (-(2.0 * p.p74)));
            s.store_scalar(280, ((s.v[277] * 0.5) + p.p56));
            s.store_div_from_scalar(281, (p.p72 * p.p227), 280);
            s.store_mul(83, 279, 281);
        }

        if (!s.b[712]) {
            s.store_scalar(83, 0.0);
        }

        s.store_div_from_scalar_offset_input(281, 1.0, 270, (s.v[626] / s.v[124]));

        s.store_sub(283, 271, 281);

        s.store_offset_mul(84, 150, 283, (p.p104 / s.v[376]));

        s.store_add_scaled_inputs4_offset_indices(80, 82, 1.0, 266, 1.0, 84, 1.0, 83, 1.0, s.v[482]);

        s.store_sub(78, 265, 80);

        s.b[713] = (p.p75 == 0.0);
        s.store_scalar(713, if s.b[713] { 1.0 } else { 0.0 });

        if s.b[713] {
            s.store_scalar(36, 0.0);
        }

        if (!s.b[713]) {
            s.store_scalar(36, 1.0);
        }

        s.b[714] = (s.v[36] == 0.0);
        s.store_scalar(714, if s.b[714] { 1.0 } else { 0.0 });

        if s.b[714] {
            s.store_scalar(267, 0.0);
        }

        if (!s.b[714]) {
            s.store_offset(281, 72, (-p.p76));
        }

        s.b[715] = (s.v[281] < (-3.0));
        s.store_scalar(715, if s.b[715] { 1.0 } else { 0.0 });

        if ((!s.b[714]) && s.b[715]) {
            s.store_scalar(284, 0.0);
            s.store_scalar(267, 0.0);
        }

        s.b[716] = (s.v[281] < 0.0);
        s.store_scalar(716, if s.b[716] { 1.0 } else { 0.0 });

        if (((!s.b[714]) && (!s.b[715])) && s.b[716]) {
            s.store_offset_mul_ad(284, s.ad_value(281), A::scale_offset(s.ad_value(281), (3.0 * (1.0 / 27.0)), (2.0 * (1.0 / 3.0))), 1.0);
            s.store_offset_mul_offset_rhs_ad_rhs(267, 281, A::mul(s.ad_value(281), A::scale_offset(s.ad_value(281), (1.0 / 27.0), (1.0 / 3.0))), 1.0, 1.0);
        }

        if (((!s.b[714]) && (!s.b[715])) && (!s.b[716])) {
            s.store_offset_mul_offset_rhs_ad_rhs(284, 281, A::mul(s.ad_value(281), A::scale_offset(s.ad_value(281), (4.0 * 0.148148111111111), (3.0 * 0.0402052934513951))), (2.0 * (1.0 / 3.0)), 1.0);
            s.store_offset_mul_offset_rhs_ad_rhs(267, 281, A::mul_offset_rhs(s.ad_value(281), A::mul(s.ad_value(281), A::scale_offset(s.ad_value(281), 0.148148111111111, 0.0402052934513951)), (1.0 / 3.0)), 1.0, 1.0);
        }

        if (!s.b[714]) {
            s.store_sqrt_offset_square_offset(639, 267, (-1.0), ((4.0 * 0.1) * 0.1));
            s.store_scaled_offset_ad(284, A::div_scaled_offset_numerator(s.ad_value(267), 1.0, (-1.0), s.ad_value(639), 1.0), 1.0, 0.5);
            s.store_offset_add_scaled_inputs_mixed_ai(267, A::offset(s.ad_value(267), (-1.0)), 0.5, 639, 0.5, (1e-10 * 0.1));
        }

        s.b[717] = (s.v[267] < 0.0);
        s.store_scalar(717, if s.b[717] { 1.0 } else { 0.0 });

        if ((!s.b[714]) && s.b[717]) {
            s.store_scalar(267, 0.0);
            s.store_scalar(284, 0.0);
        }

        if (!s.b[714]) {
            s.store_scale(267, 267, s.v[479]);
            s.store_offset_sub_from_scalar_ad(638, 1.0, s.ad_value(267), (-0.05));
            s.store_scalar(639, (4.0 * 0.05));
        }

        if (!s.b[714]) {
            if (s.v[639] > 0.0) {
            } else {
                s.store_neg(639, 639);
            }
        }

        if (!s.b[714]) {
            s.store_sqrt_square_add(639, 638, 639);
            s.store_offset_scaled_div(287, 638, 639, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(267, 638, (-0.5), 639, (-0.5), 1.0);
        }

        s.store_add_scaled_inputs4_indices(53, 52, 1.0, 138, (-1.0), 80, 1.0, 267, -1.0);

        s.copy_ad(76, 53);

        s.store_mul_ln_ad_rhs(298, 122, A::div(s.ad_value(471), s.ad_value(462)));

        s.store_add_scaled_inputs3_indices(54, 138, 1.0, 80, (-1.0), 267, 1.0);

        s.store_mul(144, 141, 271);

        s.store_square(145, 144);

        if (p.p29 != 0.0) {
            s.store_add(440, 70, 298);
        }

        if (p.p29 == 0.0) {
            s.store_add(440, 50, 298);
        }

        s.b[718] = (s.v[440] < 0.0);
        s.store_scalar(718, if s.b[718] { 1.0 } else { 0.0 });

        if s.b[718] {
            s.store_div(278, 462, 471);
            s.store_offset(279, 278, 1.0);
            s.store_add_scaled_inputs_product_right_ad(280, 122, 1.0, 440, (-1.0), 278, A::add(s.ad_value(122), s.ad_value(440)), 1.0);
            s.store_scaled_square(281, 439, (s.v[295] * s.v[295]));
            s.store_add_scaled_products_indices(282, 280, 279, 2.0, 281, 120, (-1.0));
            s.store_add_scaled_inputs3_mixed_aai(283, A::square(s.ad_value(280)), 1.0, A::mul3(s.ad_value(281), s.ad_value(120), s.ad_value(440)), 1.0, 281, 1.0);
        }

        if s.b[718] {
            if (((s.v[282] * s.v[282]) - (((4.0 * s.v[279]) * s.v[279]) * s.v[283])) >= 1e-50) {
                s.store_sub_ad(285, A::square(s.ad_value(282)), A::mul3_scaled_output(s.ad_value(279), s.ad_value(279), s.ad_value(283), 4.0));
            } else {
                s.store_scalar(285, 1e-50);
            }
        }

        if s.b[718] {
            s.store_div_scaled_inputs2_mixed_iaa(331, 282, 1.0, A::sqrt(s.ad_value(285)), 1.0, A::offset(A::square(s.ad_value(279)), 2.0), 1.0);
        }

        if (!s.b[718]) {
            s.store_mul_square_lhs(279, 439, 120);
            s.store_mul_square_lhs(280, 141, 120);
            s.store_neg_ad(281, A::add_scaled_inputs(s.ad_value(122), 1.0, s.ad_value(440), 2.0));
            s.store_offset_div(282, 280, 279, 1.0);
            s.store_scaled_square(283, 141, (s.v[295] * s.v[295]));
            s.store_add_scaled_products_indices(284, 283, 120, 1.0, 281, 282, (-2.0));
        }

        if (!s.b[718]) {
            if (((s.v[284] * s.v[284]) - ((((4.0 * s.v[282]) * s.v[282]) * s.v[281]) * s.v[281])) >= 1e-50) {
                s.store_add_scaled_square_product_mixed_iai(285, 284, 1.0, A::mul3_scaled_output(s.ad_value(282), s.ad_value(282), s.ad_value(281), 4.0), 281, (-1.0));
            } else {
                s.store_scalar(285, 1e-50);
            }
        }

        if (!s.b[718]) {
            s.store_div_scaled_inputs2_mixed_iaa(331, 284, 1.0, A::sqrt(s.ad_value(285)), 1.0, A::mul_scaled_lhs(s.ad_value(282), 2.0, s.ad_value(282)), 1.0);
        }

        s.store_mul_div_from_scalar_lhs_ad_mixed_ia(326, 2.0, 120, A::ln(A::div(s.ad_value(462), s.ad_value(127))));

        s.store_scaled_square(278, 439, (s.v[293] * s.v[293]));

        s.store_neg(279, 440);

        s.store_add_scaled_inputs3_mixed_aai(280, A::square(A::add_scaled_product(s.ad_value(279), 2.0, s.ad_value(278), s.ad_value(120), 1.0)), 1.0, A::square(s.ad_value(279)), (-4.0), 278, (-4.0));

        if (!(s.v[280] >= (10.0 * 2.220446049250313e-16))) {
            s.store_scalar(280, (10.0 * 2.220446049250313e-16));
        }

        s.store_sqrt(280, 280);

        s.store_add_scaled_product_indices(281, 279, 2.0, 278, 120, 1.0);

        s.store_scaled_sub(324, 281, 280, 0.5);

        s.store_div_ad(325, A::ln(A::div_scaled_product_by_product(s.ad_value(279), s.ad_value(279), 1.0, s.ad_value(278), s.ad_value(143), 1.0)), A::add(s.ad_value(120), A::div_from_scalar(2.0, s.ad_value(279))));

        s.b[719] = (s.v[324] < s.v[326]);
        s.store_scalar(719, if s.b[719] { 1.0 } else { 0.0 });

        if s.b[719] {
            s.copy_ad(331, 324);
        }

        if (!s.b[719]) {
            s.store_offset_sub(638, 325, 324, (-0.0008));
            s.store_scale(639, 325, (4.0 * 0.0008));
        }

        if (!s.b[719]) {
            if (s.v[639] > 0.0) {
            } else {
                s.store_neg(639, 639);
            }
        }

        if (!s.b[719]) {
            s.store_sqrt_square_add(639, 638, 639);
            s.store_offset_scaled_div(279, 638, 639, 0.5, 0.5);
            s.store_add_scaled_inputs3_indices(331, 325, 1.0, 638, (-0.5), 639, (-0.5));
        }

        s.store_scalar(62, 0.0);

        let mut assign6930_loop_guard: usize = 0;
        while {
            let assign6930_cond_e4908: f64 = if s.v[62] < s.v[28] { 1.0 } else { 0.0 };
            assign6930_cond_e4908 != 0.0
        } {
            assign6930_loop_guard += 1;
            assert!(assign6930_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            s.copy_ad(279, 439);
            s.store_mul(280, 120, 331);
            s.store_exp_neg_input(281, 280);
            s.b[720] = (s.v[331] > 1e-8);
            s.store_scalar(720, if s.b[720] { 1.0 } else { 0.0 });
            if s.b[720] {
                s.store_exp_mul(278, 120, 331);
                s.store_mul_scaled_sqrt_ad_rhs(282, 279, -1.0, A::add_scaled_offset_product_rhs(A::offset(A::add(s.ad_value(281), s.ad_value(280)), (-1.0)), 1.0, s.ad_value(143), s.ad_value(278), (-1.0), 1.0));
                s.store_mul_div_scaled_inputs_mixed_aii(283, A::add_scaled_sub_value_product(1.0, s.ad_value(281), 1.0, s.ad_value(143), s.ad_value(278), 1.0), 438, 1.0, 282, 1.0);
            }
            s.b[721] = (s.v[331] < (-1e-8));
            s.store_scalar(721, if s.b[721] { 1.0 } else { 0.0 });
            if ((!s.b[720]) && s.b[721]) {
                s.store_mul_sqrt_ad_rhs(282, 279, A::offset(A::add(s.ad_value(281), s.ad_value(280)), (-1.0)));
                s.store_mul_sub_from_scalar_rhs_ad_lhs(283, A::div(s.ad_value(438), s.ad_value(282)), 1.0, 281);
            }
            if ((!s.b[720]) && (!s.b[721])) {
                s.store_mul_ad_affine_product_lhs(282, A::sqrt(A::div(s.ad_value(438), s.ad_value(120))), s.ad_value(120), -1.0, 0.0, 331);
                s.store_neg_ad(283, A::sqrt(A::mul(s.ad_value(438), s.ad_value(120))));
            }
            s.store_sqrt_square_offset(639, 282, ((4.0 * 1e-6) * 1e-6));
            s.store_offset_scaled_div(285, 282, 639, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(284, 282, 0.5, 639, 0.5, (1e-10 * 1e-6));
            s.b[722] = (s.v[284] < 0.0);
            s.store_scalar(722, if s.b[722] { 1.0 } else { 0.0 });
            if s.b[722] {
                s.store_scalar(284, 0.0);
                s.store_scalar(285, 0.0);
            }
            s.store_offset_sub_scaled_inputs_indices(638, 296, -1.0, 284, 1.0, (-1e-9));
            s.store_scale(639, 296, (-(4.0 * 1e-9)));
            if (!(s.v[639] > 0.0)) {
                s.store_neg(639, 639);
            }
            s.store_sqrt_square_add(639, 638, 639);
            s.store_offset_scaled_div(286, 638, 639, 0.5, 0.5);
            s.store_add_scaled_inputs3_indices(284, 296, -1.0, 638, (-0.5), 639, (-0.5));
            s.store_mul3_lhs(285, 285, 283, 286);
            s.store_div_scaled_inputs_mixed_ai(334, A::square(s.ad_value(284)), ((0.5 * 9662367879.197212) * 6.241449993689894e18), 471, 1.0);
            s.store_div_scaled_product_indices(335, 334, 285, 2.0, 284, 1.0);
            s.store_sub_ad_rhs(284, 331, A::div_scaled_inputs4(s.ad_value(282), 1.0 / (s.v[294]), s.ad_value(331), (-1.0), s.ad_value(440), -1.0, s.ad_value(334), 1.0, A::add(A::scale_offset(s.ad_value(283), 1.0 / (s.v[294]), (-1.0)), s.ad_value(335)), 1.0));
            s.b[723] = ((((s.v[284] - s.v[331])) as f64).abs() < 0.001);
            s.store_scalar(723, if s.b[723] { 1.0 } else { 0.0 });
            if s.b[723] {
                s.store_scalar(62, s.v[28]);
            }
            s.copy_ad(331, 284);
            s.copy_ad(330, 282);
            s.store_offset(62, 62, 1.0);
        }

        s.copy_ad(332, 334);

    }

    pub(super) fn stamp_reactive_block_4(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.store_sqrt_div_scaled_inputs(279, 332, ((2.0 * 1.034943e-10) / 1.6021918e-19), 471, 1.0);

        s.b[724] = (s.v[279] > (0.99 * p.p227));
        s.store_scalar(724, if s.b[724] { 1.0 } else { 0.0 });

        if s.b[724] {
            s.store_div_from_scalar(278, 1.0, 270);
            s.store_scalar(280, (1.0 / s.v[294]));
            s.store_div_from_scalar_add_ad(281, 1.0, A::offset(s.ad_value(278), s.v[536]), s.ad_value(280));
            s.store_sub_from_scalar_scaled_mul(282, 1.0, 281, 278, 1.0);
            s.store_mul_ad_product_rhs_mixed_ia(283, 278, 281, A::sub(A::mul_scaled_rhs(A::offset(s.ad_value(280), (0.5 * s.v[536])), s.ad_value(296), -1.0), s.ad_value(440)));
            s.store_div(327, 283, 282);
            s.store_add(54, 54, 327);
            s.store_sub_scaled_inputs(53, 53, 1.0, 327, p.p298);
            s.copy_ad(76, 53);
        }

        s.b[725] = (s.v[33] >= 1.0);
        s.store_scalar(725, if s.b[725] { 1.0 } else { 0.0 });

        if s.b[725] {
            s.store_scalar(305, s.v[695]);
            s.store_scalar(306, s.v[696]);
            s.store_offset(307, 440, s.v[697]);
            s.store_add_scaled_inputs(328, 296, (-(s.v[536] * 0.5)), 122, 1.0);
            s.store_sub_scaled_inputs(329, 328, 1.0, 330, s.v[536]);
        }

        s.b[726] = (s.v[440] < 0.0);
        s.store_scalar(726, if s.b[726] { 1.0 } else { 0.0 });

        if ((!s.b[725]) && s.b[726]) {
            s.store_scalar(55, 0.0);
            s.store_scalar(62, 1.0);
        }

        let mut assign7150_loop_guard: usize = 0;
        while {
            let assign7150_cond_e5303: f64 = if (((!s.b[725]) && s.b[726]) && (s.v[62] <= s.v[28])) { 1.0 } else { 0.0 };
            assign7150_cond_e5303 != 0.0
        } {
            assign7150_loop_guard += 1;
            assert!(assign7150_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((!s.b[725]) && s.b[726]) {
                s.store_div_from_scalar_scaled_input(278, s.v[294], 462, ((2.0 * 1.6021918e-19) * 1.034943e-10));
                s.store_scalar(279, (1.0 + (s.v[294] * s.v[536])));
                s.store_add_scaled_inputs3_indices(280, 296, ((-(0.5 * s.v[536])) * s.v[294]), 122, s.v[294], 440, s.v[294]);
                s.store_mul3_affine_lhs(285, 278, 270, 2.0, 0.0, 270);
                s.store_add_scaled_inputs_product_mixed_aaii(282, A::offset(A::mul(s.ad_value(279), s.ad_value(270)), s.v[294]), 1.0, A::mul3_scaled_output(s.ad_value(278), s.ad_value(270), s.ad_value(296), 2.0), 1.0, 285, 55, 1.0);
                s.store_mul3_affine_lhs(286, 270, 278, ((2.0 * s.v[294]) * 2.0), 0.0, 270);
                s.store_add_scaled_value_products(283, A::offset(A::mul3(A::add_scaled_square_product(s.ad_value(279), 1.0, s.ad_value(278), s.ad_value(280), (-4.0)), s.ad_value(270), s.ad_value(270)), (s.v[294] * s.v[294])), 1.0, s.ad_value(270), A::add_scaled_product(s.ad_value(279), 1.0, s.ad_value(278), s.ad_value(296), 2.0), (2.0 * s.v[294]), s.ad_value(286), s.ad_value(55), 1.0);
                s.store_sqrt(283, 283);
                s.store_div_scaled_inputs_indices(286, 286, 1.0, 283, 2.0);
                s.store_div_from_scalar_ad(284, 1.0, A::mul3_scaled_output(s.ad_value(278), s.ad_value(270), s.ad_value(270), 2.0));
                s.store_mul_sub_rhs(346, 284, 282, 283);
                s.store_mul_sub_rhs(347, 284, 285, 286);
                s.store_div_scaled_inputs_indices(370, 346, -1.0, 347, 1.0);
            }
            s.b[727] = (((s.v[370]) as f64).abs() < 1e-12);
            s.store_scalar(727, if s.b[727] { 1.0 } else { 0.0 });
            if (((!s.b[725]) && s.b[726]) && s.b[727]) {
                s.store_scalar(62, s.v[28]);
            }
            s.b[728] = (s.v[370] > 0.1);
            s.store_scalar(728, if s.b[728] { 1.0 } else { 0.0 });
            if ((((!s.b[725]) && s.b[726]) && (!s.b[727])) && s.b[728]) {
                s.store_scalar(370, 0.1);
            }
            s.b[729] = (s.v[370] < (-0.1));
            s.store_scalar(729, if s.b[729] { 1.0 } else { 0.0 });
            if (((((!s.b[725]) && s.b[726]) && (!s.b[727])) && (!s.b[728])) && s.b[729]) {
                s.store_scalar(370, (-0.1));
            }
            if ((!s.b[725]) && s.b[726]) {
                s.store_add(55, 55, 370);
                s.store_offset(62, 62, 1.0);
            }
        }

        s.b[730] = (s.v[52] < (s.v[54] + s.v[55]));
        s.store_scalar(730, if s.b[730] { 1.0 } else { 0.0 });

        if ((!s.b[725]) && s.b[730]) {
            s.store_scalar(39, 1.0);
            s.store_scalar(292, (-1.0));
            s.copy_ad(332, 334);
            s.store_sqrt_div_scaled_inputs(279, 332, ((2.0 * 1.034943e-10) / 1.6021918e-19), 471, 1.0);
            s.store_scaled_square(278, 439, (s.v[293] * s.v[293]));
        }

        s.b[731] = ((s.v[345] + s.v[279]) < p.p227);
        s.store_scalar(731, if s.b[731] { 1.0 } else { 0.0 });

        if (((!s.b[725]) && s.b[730]) && s.b[731]) {
            s.store_sub_from_scalar(279, (10.0 * 2.220446049250313e-16), 440);
            s.store_add_scaled_inputs3_mixed_aai(280, A::square(A::add_scaled_product(s.ad_value(279), 2.0, s.ad_value(278), s.ad_value(120), 1.0)), 1.0, A::square(s.ad_value(279)), (-4.0), 278, (-4.0));
        }

        if (((!s.b[725]) && s.b[730]) && s.b[731]) {
            if (s.v[280] >= (10.0 * 2.220446049250313e-16)) {
            } else {
                s.store_scalar(280, (10.0 * 2.220446049250313e-16));
            }
        }

        if (((!s.b[725]) && s.b[730]) && s.b[731]) {
            s.store_sqrt(280, 280);
            s.store_add_scaled_product_indices(281, 279, 2.0, 278, 120, 1.0);
            s.store_scaled_sub(324, 281, 280, 0.5);
            s.store_div_ad(325, A::ln(A::div_scaled_product_by_product(s.ad_value(279), s.ad_value(279), 1.0, s.ad_value(278), s.ad_value(143), 1.0)), A::add(s.ad_value(120), A::div_from_scalar(2.0, s.ad_value(279))));
        }

        s.b[732] = (s.v[324] < s.v[326]);
        s.store_scalar(732, if s.b[732] { 1.0 } else { 0.0 });

        if ((((!s.b[725]) && s.b[730]) && s.b[731]) && s.b[732]) {
            s.copy_ad(307, 324);
        }

        if ((((!s.b[725]) && s.b[730]) && s.b[731]) && (!s.b[732])) {
            s.store_offset_sub(638, 325, 324, (-0.0008));
            s.store_scale(639, 325, (4.0 * 0.0008));
        }

        if ((((!s.b[725]) && s.b[730]) && s.b[731]) && (!s.b[732])) {
            if (s.v[639] > 0.0) {
            } else {
                s.store_neg(639, 639);
            }
        }

        if ((((!s.b[725]) && s.b[730]) && s.b[731]) && (!s.b[732])) {
            s.store_sqrt_square_add(639, 638, 639);
            s.store_offset_scaled_div(279, 638, 639, 0.5, 0.5);
            s.store_add_scaled_inputs3_indices(307, 325, 1.0, 638, (-0.5), 639, (-0.5));
        }

        if (((!s.b[725]) && s.b[730]) && (!s.b[731])) {
            s.store_add_scaled_inputs3_indices(279, 440, (-1.0), 305, (-(-1.0)), 296, (-(-(0.5 * (p.p227 * 9662367879.197212)))));
            s.store_add_scaled_inputs3_mixed_aai(280, A::square(A::add_scaled_product(s.ad_value(279), 2.0, s.ad_value(278), s.ad_value(120), 1.0)), 1.0, A::square(s.ad_value(279)), (-4.0), 278, (-4.0));
        }

        if (((!s.b[725]) && s.b[730]) && (!s.b[731])) {
            if (s.v[280] >= (10.0 * 2.220446049250313e-16)) {
            } else {
                s.store_scalar(280, (10.0 * 2.220446049250313e-16));
            }
        }

        if (((!s.b[725]) && s.b[730]) && (!s.b[731])) {
            s.store_sqrt(280, 280);
            s.store_add_scaled_product_indices(281, 279, 2.0, 278, 120, 1.0);
            s.store_scaled_sub(324, 281, 280, 0.5);
            s.store_div_ad(325, A::ln(A::div_scaled_product_by_product(s.ad_value(279), s.ad_value(279), 1.0, s.ad_value(278), s.ad_value(143), 1.0)), A::add(s.ad_value(120), A::div_from_scalar(2.0, s.ad_value(279))));
        }

        s.b[733] = (s.v[324] < s.v[326]);
        s.store_scalar(733, if s.b[733] { 1.0 } else { 0.0 });

        if ((((!s.b[725]) && s.b[730]) && (!s.b[731])) && s.b[733]) {
            s.copy_ad(307, 324);
        }

        if ((((!s.b[725]) && s.b[730]) && (!s.b[731])) && (!s.b[733])) {
            s.store_offset_sub(638, 325, 324, (-0.0008));
            s.store_scale(639, 325, (4.0 * 0.0008));
        }

        if ((((!s.b[725]) && s.b[730]) && (!s.b[731])) && (!s.b[733])) {
            if (s.v[639] > 0.0) {
            } else {
                s.store_neg(639, 639);
            }
        }

        if ((((!s.b[725]) && s.b[730]) && (!s.b[731])) && (!s.b[733])) {
            s.store_sqrt_square_add(639, 638, 639);
            s.store_offset_scaled_div(279, 638, 639, 0.5, 0.5);
            s.store_add_scaled_inputs3_indices(307, 325, 1.0, 638, (-0.5), 639, (-0.5));
        }

        if ((!s.b[725]) && s.b[730]) {
            s.store_sqrt_div_scaled_inputs(279, 332, ((2.0 * 1.034943e-10) / 1.6021918e-19), 471, 1.0);
        }

        s.b[734] = ((s.v[345] + s.v[279]) < p.p227);
        s.store_scalar(734, if s.b[734] { 1.0 } else { 0.0 });

        if (((!s.b[725]) && s.b[730]) && s.b[734]) {
            s.store_scalar(62, 0.0);
        }

        let mut assign7560_loop_guard: usize = 0;
        while {
            let assign7560_cond_e6174: f64 = if ((((!s.b[725]) && s.b[730]) && s.b[734]) && (s.v[62] < s.v[28])) { 1.0 } else { 0.0 };
            assign7560_cond_e6174 != 0.0
        } {
            assign7560_loop_guard += 1;
            assert!(assign7560_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((!s.b[725]) && s.b[730]) && s.b[734]) {
                s.copy_ad(279, 439);
                s.store_mul(280, 120, 307);
                s.store_exp_neg_input(281, 280);
            }
            s.b[735] = (s.v[307] > 1e-8);
            s.store_scalar(735, if s.b[735] { 1.0 } else { 0.0 });
            if ((((!s.b[725]) && s.b[730]) && s.b[734]) && s.b[735]) {
                s.store_exp_mul(278, 120, 307);
                s.store_mul_scaled_sqrt_ad_rhs(282, 279, -1.0, A::add_scaled_offset_product_rhs(A::offset(A::add(s.ad_value(281), s.ad_value(280)), (-1.0)), 1.0, s.ad_value(143), s.ad_value(278), (-1.0), 1.0));
                s.store_mul_div_scaled_inputs_mixed_aii(283, A::add_scaled_sub_value_product(1.0, s.ad_value(281), 1.0, s.ad_value(143), s.ad_value(278), 1.0), 438, 1.0, 282, 1.0);
            }
            s.b[736] = (s.v[307] < (-1e-8));
            s.store_scalar(736, if s.b[736] { 1.0 } else { 0.0 });
            if (((((!s.b[725]) && s.b[730]) && s.b[734]) && (!s.b[735])) && s.b[736]) {
                s.store_mul_sqrt_ad_rhs(282, 279, A::offset(A::add(s.ad_value(281), s.ad_value(280)), (-1.0)));
                s.store_mul_sub_from_scalar_rhs_ad_lhs(283, A::div(s.ad_value(438), s.ad_value(282)), 1.0, 281);
            }
            if (((((!s.b[725]) && s.b[730]) && s.b[734]) && (!s.b[735])) && (!s.b[736])) {
                s.store_mul_ad_affine_product_lhs(282, A::sqrt(A::div(s.ad_value(438), s.ad_value(120))), s.ad_value(120), -1.0, 0.0, 307);
                s.store_neg_ad(283, A::sqrt(A::mul(s.ad_value(438), s.ad_value(120))));
            }
            if (((!s.b[725]) && s.b[730]) && s.b[734]) {
                s.store_sqrt_square_offset(639, 282, ((4.0 * 1e-10) * 1e-10));
                s.store_offset_scaled_div(285, 282, 639, 0.5, 0.5);
                s.store_offset_add_scaled_inputs_indices(284, 282, 0.5, 639, 0.5, (1e-10 * 1e-10));
            }
            s.b[737] = (s.v[284] < 0.0);
            s.store_scalar(737, if s.b[737] { 1.0 } else { 0.0 });
            if ((((!s.b[725]) && s.b[730]) && s.b[734]) && s.b[737]) {
                s.store_scalar(284, 0.0);
                s.store_scalar(285, 0.0);
            }
            if (((!s.b[725]) && s.b[730]) && s.b[734]) {
                s.store_offset_sub_scaled_inputs_indices(638, 296, -1.0, 284, 1.0, (-1e-13));
                s.store_scale(639, 296, (-(4.0 * 1e-13)));
            }
            if (((!s.b[725]) && s.b[730]) && s.b[734]) {
                if (s.v[639] > 0.0) {
                } else {
                    s.store_neg(639, 639);
                }
            }
            if (((!s.b[725]) && s.b[730]) && s.b[734]) {
                s.store_sqrt_square_add(639, 638, 639);
                s.store_offset_scaled_div(286, 638, 639, 0.5, 0.5);
                s.store_add_scaled_inputs3_indices(284, 296, -1.0, 638, (-0.5), 639, (-0.5));
                s.store_mul3_lhs(285, 285, 283, 286);
                s.store_div_scaled_inputs_mixed_ai(332, A::square(s.ad_value(284)), ((0.5 * 9662367879.197212) * 6.241449993689894e18), 471, 1.0);
                s.store_div_scaled_product_indices(333, 332, 285, 2.0, 284, 1.0);
                s.store_sub_ad_rhs(284, 307, A::div_scaled_inputs4(s.ad_value(282), 1.0 / (s.v[294]), s.ad_value(307), (-1.0), s.ad_value(440), -1.0, s.ad_value(332), 1.0, A::add(A::scale_offset(s.ad_value(283), 1.0 / (s.v[294]), (-1.0)), s.ad_value(333)), 1.0));
            }
            s.b[738] = ((((s.v[284] - s.v[307])) as f64).abs() < 0.001);
            s.store_scalar(738, if s.b[738] { 1.0 } else { 0.0 });
            if ((((!s.b[725]) && s.b[730]) && s.b[734]) && s.b[738]) {
                s.store_scalar(62, s.v[28]);
            }
            if (((!s.b[725]) && s.b[730]) && s.b[734]) {
                s.copy_ad(307, 284);
                s.copy_ad(312, 282);
                s.store_offset(62, 62, 1.0);
            }
        }

        if (((!s.b[725]) && s.b[730]) && (!s.b[734])) {
            s.store_scalar(62, 0.0);
        }

    }

    pub(super) fn stamp_reactive_block_5(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        let mut assign7580_loop_guard: usize = 0;
        while {
            let assign7580_cond_e6663: f64 = if ((((!s.b[725]) && s.b[730]) && (!s.b[734])) && (s.v[62] < s.v[28])) { 1.0 } else { 0.0 };
            assign7580_cond_e6663 != 0.0
        } {
            assign7580_loop_guard += 1;
            assert!(assign7580_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((!s.b[725]) && s.b[730]) && (!s.b[734])) {
                s.copy_ad(279, 439);
                s.store_mul(280, 120, 307);
                s.store_exp_neg_input(281, 280);
            }
            s.b[739] = (s.v[307] > 1e-8);
            s.store_scalar(739, if s.b[739] { 1.0 } else { 0.0 });
            if ((((!s.b[725]) && s.b[730]) && (!s.b[734])) && s.b[739]) {
                s.store_exp_mul(278, 120, 307);
                s.store_mul_scaled_sqrt_ad_rhs(282, 279, -1.0, A::add_scaled_offset_product_rhs(A::offset(A::add(s.ad_value(281), s.ad_value(280)), (-1.0)), 1.0, s.ad_value(143), s.ad_value(278), (-1.0), 1.0));
                s.store_mul_div_scaled_inputs_mixed_aii(283, A::add_scaled_sub_value_product(1.0, s.ad_value(281), 1.0, s.ad_value(143), s.ad_value(278), 1.0), 438, 1.0, 282, 1.0);
            }
            s.b[740] = (s.v[307] < (-1e-8));
            s.store_scalar(740, if s.b[740] { 1.0 } else { 0.0 });
            if (((((!s.b[725]) && s.b[730]) && (!s.b[734])) && (!s.b[739])) && s.b[740]) {
                s.store_mul_sqrt_ad_rhs(282, 279, A::offset(A::add(s.ad_value(281), s.ad_value(280)), (-1.0)));
                s.store_mul_sub_from_scalar_rhs_ad_lhs(283, A::div(s.ad_value(438), s.ad_value(282)), 1.0, 281);
            }
            if (((((!s.b[725]) && s.b[730]) && (!s.b[734])) && (!s.b[739])) && (!s.b[740])) {
                s.store_mul_ad_affine_product_lhs(282, A::sqrt(A::div(s.ad_value(438), s.ad_value(120))), s.ad_value(120), -1.0, 0.0, 307);
                s.store_neg_ad(283, A::sqrt(A::mul(s.ad_value(438), s.ad_value(120))));
            }
            if (((!s.b[725]) && s.b[730]) && (!s.b[734])) {
                s.store_sqrt_square_offset(639, 282, ((4.0 * 1e-10) * 1e-10));
                s.store_offset_scaled_div(285, 282, 639, 0.5, 0.5);
                s.store_offset_add_scaled_inputs_indices(284, 282, 0.5, 639, 0.5, (1e-10 * 1e-10));
            }
            s.b[741] = (s.v[284] < 0.0);
            s.store_scalar(741, if s.b[741] { 1.0 } else { 0.0 });
            if ((((!s.b[725]) && s.b[730]) && (!s.b[734])) && s.b[741]) {
                s.store_scalar(284, 0.0);
                s.store_scalar(285, 0.0);
            }
            if (((!s.b[725]) && s.b[730]) && (!s.b[734])) {
                s.store_offset_sub_scaled_inputs_indices(638, 296, -1.0, 284, 1.0, (-1e-13));
                s.store_scale(639, 296, (-(4.0 * 1e-13)));
            }
            if (((!s.b[725]) && s.b[730]) && (!s.b[734])) {
                if (s.v[639] > 0.0) {
                } else {
                    s.store_neg(639, 639);
                }
            }
            if (((!s.b[725]) && s.b[730]) && (!s.b[734])) {
                s.store_sqrt_square_add(639, 638, 639);
                s.store_offset_scaled_div(286, 638, 639, 0.5, 0.5);
                s.store_add_scaled_inputs3_indices(284, 296, -1.0, 638, (-0.5), 639, (-0.5));
                s.store_mul3_lhs(285, 285, 283, 286);
                s.store_div_scaled_inputs_mixed_ai(332, A::square(s.ad_value(284)), ((0.5 * 9662367879.197212) * 6.241449993689894e18), 471, 1.0);
                s.store_div_scaled_product_indices(333, 332, 285, 2.0, 284, 1.0);
                s.store_sub_div_rhs_ad(284, 307, A::add(A::sub(A::add(A::add_scaled_inputs3(s.ad_value(305), 1.0, s.ad_value(307), (-1.0), s.ad_value(282), 1.0 / (s.v[294])), A::add_scaled_inputs(s.ad_value(282), (p.p227 * 9662367879.197212), s.ad_value(296), (0.5 * (p.p227 * 9662367879.197212)))), s.ad_value(440)), s.ad_value(332)), A::add_scaled_inputs3_offset(s.ad_value(283), 1.0 / (s.v[294]), s.ad_value(283), (p.p227 * 9662367879.197212), s.ad_value(333), 1.0, (-1.0)));
            }
            s.b[742] = ((((s.v[284] - s.v[307])) as f64).abs() < 0.001);
            s.store_scalar(742, if s.b[742] { 1.0 } else { 0.0 });
            if ((((!s.b[725]) && s.b[730]) && (!s.b[734])) && s.b[742]) {
                s.store_scalar(62, s.v[28]);
            }
            if (((!s.b[725]) && s.b[730]) && (!s.b[734])) {
                s.copy_ad(307, 284);
                s.copy_ad(312, 282);
                s.store_offset(62, 62, 1.0);
            }
        }

        if ((!s.b[725]) && s.b[730]) {
            s.store_add(307, 440, 307);
            s.store_sub_scaled_inputs(306, 307, 1.0, 312, 1.0 / (s.v[294]));
        }

        if (!s.b[725]) {
            s.store_offset_div_scaled_offset_numerator(290, A::mul(s.ad_value(120), A::sub(s.ad_value(76), s.ad_value(50))), 4.0, ((-1.0) * 4.0), A::mul(s.ad_value(145), s.ad_value(121)), 1.0, 1.0);
        }

        if (!s.b[725]) {
            if (s.v[290] >= (10.0 * 2.220446049250313e-16)) {
            } else {
                s.store_scalar(290, (10.0 * 2.220446049250313e-16));
            }
        }

        if (!s.b[725]) {
            s.store_add_product3_rhs_mixed_iia(319, 76, 145, 120, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(290))), 0.5);
            s.store_div_from_scalar(278, 1.0, 270);
            s.store_scalar(279, (p.p227 / 1.034943e-10));
            s.store_scalar(280, (1.0 / s.v[294]));
            s.store_div_from_scalar_ad(281, 1.0, A::add_scaled_inputs3(s.ad_value(278), 1.0, s.ad_value(279), 1.0, s.ad_value(280), 1.0));
        }

        s.b[743] = ((s.v[52] - s.v[327]) <= s.v[78]);
        s.store_scalar(743, if s.b[743] { 1.0 } else { 0.0 });

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

        if ((!s.b[725]) && s.b[743]) {
            s.store_mul_ad_rhs(282, 281, A::add_scaled_inputs_product(s.ad_value(76), 1.0, s.ad_value(440), (-1.0), A::add_scaled_inputs(s.ad_value(280), 1.0, s.ad_value(279), 0.5), s.ad_value(283), -1.0));
        }

        if ((!s.b[725]) && (!s.b[743])) {
            s.store_mul_ad_rhs(282, 281, A::add_scaled_inputs_product(s.ad_value(76), 1.0, s.ad_value(440), (-1.0), A::add_scaled_inputs(s.ad_value(280), 1.0, s.ad_value(279), 0.5), s.ad_value(296), -1.0));
        }

        if (!s.b[725]) {
            s.store_sub_div_rhs_indices(319, 76, 282, 270);
            s.copy_ad(321, 319);
        }

        s.b[744] = ((s.v[52] - s.v[327]) > s.v[78]);
        s.store_scalar(744, if s.b[744] { 1.0 } else { 0.0 });

        if ((!s.b[725]) && s.b[744]) {
            s.store_div_scalar_by_product(279, 1.0, s.ad_value(142), s.ad_value(381), 1.0);
            s.store_mul_ad_product_rhs(280, 279, A::sub(s.ad_value(76), s.ad_value(327)), A::sub(s.ad_value(76), s.ad_value(327)));
            s.store_add_ad_rhs(281, 120, A::div_from_scalar(2.0, A::sub(s.ad_value(76), s.ad_value(327))));
            s.store_div_ln_lhs(320, 280, 281);
        }

        s.b[745] = ((s.v[319] > (s.v[320] - 0.15)) && (0.15 >= 0.0));
        s.store_scalar(745, if s.b[745] { 1.0 } else { 0.0 });

        if (((!s.b[725]) && s.b[744]) && s.b[745]) {
            s.store_offset_sub(638, 319, 320, 0.15);
            s.store_square(642, 638);
            s.store_scalar(643, (0.15 * 0.15));
            s.store_scalar(644, 1.0);
            s.store_scalar(645, 1.0);
            s.store_scalar(647, 0.0);
            s.store_scalar(648, 0.0);
            s.store_scalar(220, 0.0);
            s.store_scalar(646, 0.0);
            s.store_mul(644, 644, 642);
            s.store_mul(645, 645, 643);
            s.store_add(220, 644, 645);
            s.copy_ad(646, 220);
        }

        s.b[746] = ((((1.0 == 1.0) || (1.0 == 2.0)) || (1.0 == 4.0)) || (1.0 == 8.0));
        s.store_scalar(746, if s.b[746] { 1.0 } else { 0.0 });

        s.b[747] = (1.0 == 1.0);
        s.store_scalar(747, if s.b[747] { 1.0 } else { 0.0 });

        if (((((!s.b[725]) && s.b[744]) && s.b[745]) && s.b[746]) && s.b[747]) {
            s.store_scalar(648, 1.0);
        }

        s.b[748] = (1.0 == 2.0);
        s.store_scalar(748, if s.b[748] { 1.0 } else { 0.0 });

        if ((((((!s.b[725]) && s.b[744]) && s.b[745]) && s.b[746]) && (!s.b[747])) && s.b[748]) {
            s.store_scalar(648, 2.0);
        }

        s.b[749] = (1.0 == 4.0);
        s.store_scalar(749, if s.b[749] { 1.0 } else { 0.0 });

        if (((((((!s.b[725]) && s.b[744]) && s.b[745]) && s.b[746]) && (!s.b[747])) && (!s.b[748])) && s.b[749]) {
            s.store_scalar(648, 3.0);
        }

        s.b[750] = (1.0 == 8.0);
        s.store_scalar(750, if s.b[750] { 1.0 } else { 0.0 });

        if ((((((((!s.b[725]) && s.b[744]) && s.b[745]) && s.b[746]) && (!s.b[747])) && (!s.b[748])) && (!s.b[749])) && s.b[750]) {
            s.store_scalar(648, 4.0);
        }

        if ((((!s.b[725]) && s.b[744]) && s.b[745]) && s.b[746]) {
            s.store_scalar(647, 0.0);
        }

        let mut assign8040_loop_guard: usize = 0;
        while {
            let assign8040_cond_e7685: f64 = if (((((!s.b[725]) && s.b[744]) && s.b[745]) && s.b[746]) && (s.v[647] < s.v[648])) { 1.0 } else { 0.0 };
            assign8040_cond_e7685 != 0.0
        } {
            assign8040_loop_guard += 1;
            assert!(assign8040_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((!s.b[725]) && s.b[744]) && s.b[745]) && s.b[746]) {
                s.store_sqrt(646, 646);
                s.store_offset(647, 647, 1.0);
            }
        }

        if ((((!s.b[725]) && s.b[744]) && s.b[745]) && (!s.b[746])) {
            s.store_powf(646, 646, (1.0 / 2.0));
        }

        if (((!s.b[725]) && s.b[744]) && s.b[745]) {
            s.store_div_from_scalar_offset_input(646, 1.0, 646, 1e-50);
            s.store_scaled_mul(637, 638, 646, 0.15);
            s.store_div_scaled_product_offset_denominator(279, s.ad_value(645), s.ad_value(646), 0.15, s.ad_value(220), 1e-50, 1.0);
            s.store_add_offset_lhs(321, 320, (-0.15), 637);
        }

        if (((!s.b[725]) && s.b[744]) && s.b[745]) {
        }

        if (((!s.b[725]) && s.b[744]) && (!s.b[745])) {
            s.copy_ad(321, 319);
            s.store_scalar(279, 1.0);
        }

        if (!s.b[725]) {
            if (s.v[321] > 0.0) {
                s.store_sqrt_div_scaled_inputs(345, 321, ((2.0 * 1.034943e-10) / 1.6021918e-19), 471, 1.0);
            } else {
                s.store_scalar(345, 0.0);
            }
        }

        s.b[751] = (s.v[345] < p.p227);
        s.store_scalar(751, if s.b[751] { 1.0 } else { 0.0 });

        if ((!s.b[725]) && s.b[751]) {
            s.store_scalar(39, 1.0);
        }

        if ((!s.b[725]) && (!s.b[751])) {
            s.store_scalar(39, 2.0);
        }

        if (!s.b[725]) {
            s.copy_ad(305, 321);
            s.copy_ad(58, 319);
            s.store_scaled_square(278, 439, (s.v[293] * s.v[293]));
        }

        s.b[752] = (s.v[39] == 1.0);
        s.store_scalar(752, if s.b[752] { 1.0 } else { 0.0 });

        if ((!s.b[725]) && s.b[752]) {
            s.store_neg(279, 440);
            s.store_add_scaled_inputs3_mixed_aai(280, A::square(A::add_scaled_product(s.ad_value(279), 2.0, s.ad_value(278), s.ad_value(120), 1.0)), 1.0, A::square(s.ad_value(279)), (-4.0), 278, (-4.0));
        }

        if ((!s.b[725]) && s.b[752]) {
            if (s.v[280] >= (10.0 * 2.220446049250313e-16)) {
            } else {
                s.store_scalar(280, (10.0 * 2.220446049250313e-16));
            }
        }

        if ((!s.b[725]) && s.b[752]) {
            s.store_sqrt(280, 280);
            s.store_add_scaled_product_indices(281, 279, 2.0, 278, 120, 1.0);
            s.store_scaled_sub(324, 281, 280, 0.5);
            s.store_div_ad(325, A::ln(A::div_scaled_product_by_product(s.ad_value(279), s.ad_value(279), 1.0, s.ad_value(278), s.ad_value(143), 1.0)), A::add(s.ad_value(120), A::div_from_scalar(2.0, s.ad_value(279))));
        }

        s.b[753] = (s.v[324] < s.v[326]);
        s.store_scalar(753, if s.b[753] { 1.0 } else { 0.0 });

        if (((!s.b[725]) && s.b[752]) && s.b[753]) {
            s.copy_ad(307, 324);
        }

        if (((!s.b[725]) && s.b[752]) && (!s.b[753])) {
            s.store_offset_sub(638, 325, 324, (-0.0008));
            s.store_scale(639, 325, (4.0 * 0.0008));
        }

        if (((!s.b[725]) && s.b[752]) && (!s.b[753])) {
            if (s.v[639] > 0.0) {
            } else {
                s.store_neg(639, 639);
            }
        }

        if (((!s.b[725]) && s.b[752]) && (!s.b[753])) {
            s.store_sqrt_square_add(639, 638, 639);
            s.store_offset_scaled_div(279, 638, 639, 0.5, 0.5);
            s.store_add_scaled_inputs3_indices(307, 325, 1.0, 638, (-0.5), 639, (-0.5));
        }

        if ((!s.b[725]) && (!s.b[752])) {
            s.store_add_scaled_inputs3_indices(279, 440, (-1.0), 305, (-(-1.0)), 296, (-(-(0.5 * (p.p227 * 9662367879.197212)))));
            s.store_add_scaled_inputs3_mixed_aai(280, A::square(A::add_scaled_product(s.ad_value(279), 2.0, s.ad_value(278), s.ad_value(120), 1.0)), 1.0, A::square(s.ad_value(279)), (-4.0), 278, (-4.0));
        }

        if ((!s.b[725]) && (!s.b[752])) {
            if (s.v[280] >= (10.0 * 2.220446049250313e-16)) {
            } else {
                s.store_scalar(280, (10.0 * 2.220446049250313e-16));
            }
        }

        if ((!s.b[725]) && (!s.b[752])) {
            s.store_sqrt(280, 280);
            s.store_add_scaled_product_indices(281, 279, 2.0, 278, 120, 1.0);
            s.store_scaled_sub(324, 281, 280, 0.5);
            s.store_div_ad(325, A::ln(A::div_scaled_product_by_product(s.ad_value(279), s.ad_value(279), 1.0, s.ad_value(278), s.ad_value(143), 1.0)), A::add(s.ad_value(120), A::div_from_scalar(2.0, s.ad_value(279))));
        }

        s.b[754] = (s.v[324] < s.v[326]);
        s.store_scalar(754, if s.b[754] { 1.0 } else { 0.0 });

        if (((!s.b[725]) && (!s.b[752])) && s.b[754]) {
            s.copy_ad(307, 324);
        }

        if (((!s.b[725]) && (!s.b[752])) && (!s.b[754])) {
            s.store_offset_sub(638, 325, 324, (-0.0008));
            s.store_scale(639, 325, (4.0 * 0.0008));
        }

        if (((!s.b[725]) && (!s.b[752])) && (!s.b[754])) {
            if (s.v[639] > 0.0) {
            } else {
                s.store_neg(639, 639);
            }
        }

    }

    pub(super) fn stamp_reactive_block_6(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((!s.b[725]) && (!s.b[752])) && (!s.b[754])) {
            s.store_sqrt_square_add(639, 638, 639);
            s.store_offset_scaled_div(279, 638, 639, 0.5, 0.5);
            s.store_add_scaled_inputs3_indices(307, 325, 1.0, 638, (-0.5), 639, (-0.5));
        }

        s.b[755] = ((s.v[39] == 1.0) && (0.0 != 0.0));
        s.store_scalar(755, if s.b[755] { 1.0 } else { 0.0 });

        if ((!s.b[725]) && s.b[755]) {
            s.store_scalar(39, 1.0);
            s.store_scalar(62, 0.0);
        }

        let mut assign8540_loop_guard: usize = 0;
        while {
            let assign8540_cond_e8341: f64 = if (((!s.b[725]) && s.b[755]) && (s.v[62] < s.v[28])) { 1.0 } else { 0.0 };
            assign8540_cond_e8341 != 0.0
        } {
            assign8540_loop_guard += 1;
            assert!(assign8540_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((!s.b[725]) && s.b[755]) {
                s.copy_ad(279, 439);
                s.store_mul(280, 120, 307);
                s.store_exp_neg_input(281, 280);
            }
            s.b[756] = (s.v[307] > 1e-8);
            s.store_scalar(756, if s.b[756] { 1.0 } else { 0.0 });
            if (((!s.b[725]) && s.b[755]) && s.b[756]) {
                s.store_exp_mul(278, 120, 307);
                s.store_mul_scaled_sqrt_ad_rhs(282, 279, -1.0, A::add_scaled_offset_product_rhs(A::offset(A::add(s.ad_value(281), s.ad_value(280)), (-1.0)), 1.0, s.ad_value(143), s.ad_value(278), (-1.0), 1.0));
                s.store_mul_div_scaled_inputs_mixed_aii(283, A::add_scaled_sub_value_product(1.0, s.ad_value(281), 1.0, s.ad_value(143), s.ad_value(278), 1.0), 438, 1.0, 282, 1.0);
            }
            s.b[757] = (s.v[307] < (-1e-8));
            s.store_scalar(757, if s.b[757] { 1.0 } else { 0.0 });
            if ((((!s.b[725]) && s.b[755]) && (!s.b[756])) && s.b[757]) {
                s.store_mul_sqrt_ad_rhs(282, 279, A::offset(A::add(s.ad_value(281), s.ad_value(280)), (-1.0)));
                s.store_mul_sub_from_scalar_rhs_ad_lhs(283, A::div(s.ad_value(438), s.ad_value(282)), 1.0, 281);
            }
            if ((((!s.b[725]) && s.b[755]) && (!s.b[756])) && (!s.b[757])) {
                s.store_mul_ad_affine_product_lhs(282, A::sqrt(A::div(s.ad_value(438), s.ad_value(120))), s.ad_value(120), -1.0, 0.0, 307);
                s.store_neg_ad(283, A::sqrt(A::mul(s.ad_value(438), s.ad_value(120))));
            }
            if ((!s.b[725]) && s.b[755]) {
                s.store_sub_ad_rhs(284, 307, A::div_scaled_inputs3(s.ad_value(282), 1.0 / (s.v[294]), s.ad_value(307), (-1.0), s.ad_value(440), -1.0, A::scale_offset(s.ad_value(283), 1.0 / (s.v[294]), (-1.0)), 1.0));
            }
            s.b[758] = ((((s.v[284] - s.v[307])) as f64).abs() < 0.001);
            s.store_scalar(758, if s.b[758] { 1.0 } else { 0.0 });
            if (((!s.b[725]) && s.b[755]) && s.b[758]) {
                s.copy_ad(285, 62);
                s.store_scalar(62, s.v[28]);
            }
            if ((!s.b[725]) && s.b[755]) {
                s.copy_ad(307, 284);
                s.copy_ad(312, 282);
                s.store_offset(62, 62, 1.0);
            }
        }

        if ((!s.b[725]) && s.b[755]) {
            s.store_add(307, 440, 307);
            s.store_sub_scaled_inputs(306, 307, 1.0, 312, 1.0 / (s.v[294]));
        }

        if ((!s.b[725]) && (!s.b[755])) {
            s.store_scalar(39, 2.0);
        }

        s.b[759] = (0.0 == 0.0);
        s.store_scalar(759, if s.b[759] { 1.0 } else { 0.0 });

        if (((!s.b[725]) && (!s.b[755])) && s.b[759]) {
            s.store_scalar(315, (1e-12 * 100.0));
            s.copy_ad(56, 319);
        }

        if (((!s.b[725]) && (!s.b[755])) && (!s.b[759])) {
            s.store_scalar(315, 0.001);
            s.copy_ad(56, 305);
        }

        if ((!s.b[725]) && (!s.b[755])) {
            s.store_scalar(62, 0.0);
        }

        let mut assign8640_loop_guard: usize = 0;
        while {
            let assign8640_cond_e8666: f64 = if (((!s.b[725]) && (!s.b[755])) && (s.v[62] < s.v[28])) { 1.0 } else { 0.0 };
            assign8640_cond_e8666 != 0.0
        } {
            assign8640_loop_guard += 1;
            assert!(assign8640_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((!s.b[725]) && (!s.b[755])) {
                s.copy_ad(279, 439);
                s.store_mul(280, 120, 307);
                s.store_exp_neg_input(281, 280);
            }
            s.b[760] = (s.v[307] > 1e-8);
            s.store_scalar(760, if s.b[760] { 1.0 } else { 0.0 });
            if (((!s.b[725]) && (!s.b[755])) && s.b[760]) {
                s.store_exp_mul(278, 120, 307);
                s.store_mul_scaled_sqrt_ad_rhs(282, 279, -1.0, A::add_scaled_offset_product_rhs(A::offset(A::add(s.ad_value(281), s.ad_value(280)), (-1.0)), 1.0, s.ad_value(143), s.ad_value(278), (-1.0), 1.0));
                s.store_mul_div_scaled_inputs_mixed_aii(283, A::add_scaled_sub_value_product(1.0, s.ad_value(281), 1.0, s.ad_value(143), s.ad_value(278), 1.0), 438, 1.0, 282, 1.0);
            }
            s.b[761] = (s.v[307] < (-1e-8));
            s.store_scalar(761, if s.b[761] { 1.0 } else { 0.0 });
            if ((((!s.b[725]) && (!s.b[755])) && (!s.b[760])) && s.b[761]) {
                s.store_mul_sqrt_ad_rhs(282, 279, A::offset(A::add(s.ad_value(281), s.ad_value(280)), (-1.0)));
                s.store_mul_sub_from_scalar_rhs_ad_lhs(283, A::div(s.ad_value(438), s.ad_value(282)), 1.0, 281);
            }
            if ((((!s.b[725]) && (!s.b[755])) && (!s.b[760])) && (!s.b[761])) {
                s.store_mul_ad_affine_product_lhs(282, A::sqrt(A::div(s.ad_value(438), s.ad_value(120))), s.ad_value(120), -1.0, 0.0, 307);
                s.store_neg_ad(283, A::sqrt(A::mul(s.ad_value(438), s.ad_value(120))));
            }
            if ((!s.b[725]) && (!s.b[755])) {
                s.store_sub_div_rhs_ad(284, 307, A::sub(A::add(A::add_scaled_inputs3(s.ad_value(56), 1.0, s.ad_value(307), (-1.0), s.ad_value(282), 1.0 / (s.v[294])), A::add_scaled_inputs(s.ad_value(282), (p.p227 * 9662367879.197212), s.ad_value(296), (0.5 * (p.p227 * 9662367879.197212)))), s.ad_value(440)), A::add_scaled_inputs(A::scale_offset(s.ad_value(283), 1.0 / (s.v[294]), (-1.0)), 1.0, s.ad_value(283), (p.p227 * 9662367879.197212)));
            }
            s.b[762] = ((((s.v[284] - s.v[307])) as f64).abs() < s.v[315]);
            s.store_scalar(762, if s.b[762] { 1.0 } else { 0.0 });
            if (((!s.b[725]) && (!s.b[755])) && s.b[762]) {
                s.copy_ad(285, 62);
                s.store_scalar(62, s.v[28]);
            }
            if ((!s.b[725]) && (!s.b[755])) {
                s.copy_ad(307, 284);
                s.copy_ad(312, 282);
                s.store_offset(62, 62, 1.0);
            }
        }

        s.b[763] = (0.0 == 0.0);
        s.store_scalar(763, if s.b[763] { 1.0 } else { 0.0 });

        if (((!s.b[725]) && (!s.b[755])) && s.b[763]) {
            s.copy_ad(316, 312);
        }

        s.b[764] = (1.0 == 0.0);
        s.store_scalar(764, if s.b[764] { 1.0 } else { 0.0 });

        if (((!s.b[725]) && (!s.b[755])) && s.b[764]) {
            s.store_scalar(315, (1e-12 * 100.0));
            s.copy_ad(56, 319);
        }

        if (((!s.b[725]) && (!s.b[755])) && (!s.b[764])) {
            s.store_scalar(315, 0.001);
            s.copy_ad(56, 305);
        }

        if ((!s.b[725]) && (!s.b[755])) {
            s.store_scalar(62, 0.0);
        }

        let mut assign8730_loop_guard: usize = 0;
        while {
            let assign8730_cond_e9009: f64 = if (((!s.b[725]) && (!s.b[755])) && (s.v[62] < s.v[28])) { 1.0 } else { 0.0 };
            assign8730_cond_e9009 != 0.0
        } {
            assign8730_loop_guard += 1;
            assert!(assign8730_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((!s.b[725]) && (!s.b[755])) {
                s.copy_ad(279, 439);
                s.store_mul(280, 120, 307);
                s.store_exp_neg_input(281, 280);
            }
            s.b[765] = (s.v[307] > 1e-8);
            s.store_scalar(765, if s.b[765] { 1.0 } else { 0.0 });
            if (((!s.b[725]) && (!s.b[755])) && s.b[765]) {
                s.store_exp_mul(278, 120, 307);
                s.store_mul_scaled_sqrt_ad_rhs(282, 279, -1.0, A::add_scaled_offset_product_rhs(A::offset(A::add(s.ad_value(281), s.ad_value(280)), (-1.0)), 1.0, s.ad_value(143), s.ad_value(278), (-1.0), 1.0));
                s.store_mul_div_scaled_inputs_mixed_aii(283, A::add_scaled_sub_value_product(1.0, s.ad_value(281), 1.0, s.ad_value(143), s.ad_value(278), 1.0), 438, 1.0, 282, 1.0);
            }
            s.b[766] = (s.v[307] < (-1e-8));
            s.store_scalar(766, if s.b[766] { 1.0 } else { 0.0 });
            if ((((!s.b[725]) && (!s.b[755])) && (!s.b[765])) && s.b[766]) {
                s.store_mul_sqrt_ad_rhs(282, 279, A::offset(A::add(s.ad_value(281), s.ad_value(280)), (-1.0)));
                s.store_mul_sub_from_scalar_rhs_ad_lhs(283, A::div(s.ad_value(438), s.ad_value(282)), 1.0, 281);
            }
            if ((((!s.b[725]) && (!s.b[755])) && (!s.b[765])) && (!s.b[766])) {
                s.store_mul_ad_affine_product_lhs(282, A::sqrt(A::div(s.ad_value(438), s.ad_value(120))), s.ad_value(120), -1.0, 0.0, 307);
                s.store_neg_ad(283, A::sqrt(A::mul(s.ad_value(438), s.ad_value(120))));
            }
            if ((!s.b[725]) && (!s.b[755])) {
                s.store_sub_div_rhs_ad(284, 307, A::sub(A::add(A::add_scaled_inputs3(s.ad_value(56), 1.0, s.ad_value(307), (-1.0), s.ad_value(282), 1.0 / (s.v[294])), A::add_scaled_inputs(s.ad_value(282), (p.p227 * 9662367879.197212), s.ad_value(296), (0.5 * (p.p227 * 9662367879.197212)))), s.ad_value(440)), A::add_scaled_inputs(A::scale_offset(s.ad_value(283), 1.0 / (s.v[294]), (-1.0)), 1.0, s.ad_value(283), (p.p227 * 9662367879.197212)));
            }
            s.b[767] = ((((s.v[284] - s.v[307])) as f64).abs() < s.v[315]);
            s.store_scalar(767, if s.b[767] { 1.0 } else { 0.0 });
            if (((!s.b[725]) && (!s.b[755])) && s.b[767]) {
                s.copy_ad(285, 62);
                s.store_scalar(62, s.v[28]);
            }
            if ((!s.b[725]) && (!s.b[755])) {
                s.copy_ad(307, 284);
                s.copy_ad(312, 282);
                s.store_offset(62, 62, 1.0);
            }
        }

        s.b[768] = (1.0 == 0.0);
        s.store_scalar(768, if s.b[768] { 1.0 } else { 0.0 });

        if (((!s.b[725]) && (!s.b[755])) && s.b[768]) {
            s.copy_ad(316, 312);
        }

        if ((!s.b[725]) && (!s.b[755])) {
            s.store_scalar(63, 0.0);
        }

        if (!s.b[725]) {
            s.store_offset_add(307, 440, 307, (-0.01));
            s.store_sub_scaled_inputs(306, 307, 1.0, 312, 1.0 / (s.v[294]));
        }

        s.b[769] = ((s.v[306] > (s.v[305] - 0.15)) && (0.15 >= 0.0));
        s.store_scalar(769, if s.b[769] { 1.0 } else { 0.0 });

        if ((!s.b[725]) && s.b[769]) {
            s.store_offset_sub(638, 306, 305, 0.15);
            s.store_square(642, 638);
            s.store_scalar(643, (0.15 * 0.15));
            s.store_scalar(644, 1.0);
            s.store_scalar(645, 1.0);
            s.store_scalar(647, 0.0);
            s.store_scalar(648, 0.0);
            s.store_scalar(220, 0.0);
            s.store_scalar(646, 0.0);
            s.store_mul(644, 644, 642);
            s.store_mul(645, 645, 643);
            s.store_add(220, 644, 645);
            s.copy_ad(646, 220);
        }

        s.b[770] = ((((1.0 == 1.0) || (1.0 == 2.0)) || (1.0 == 4.0)) || (1.0 == 8.0));
        s.store_scalar(770, if s.b[770] { 1.0 } else { 0.0 });

        s.b[771] = (1.0 == 1.0);
        s.store_scalar(771, if s.b[771] { 1.0 } else { 0.0 });

        if ((((!s.b[725]) && s.b[769]) && s.b[770]) && s.b[771]) {
            s.store_scalar(648, 1.0);
        }

        s.b[772] = (1.0 == 2.0);
        s.store_scalar(772, if s.b[772] { 1.0 } else { 0.0 });

        if (((((!s.b[725]) && s.b[769]) && s.b[770]) && (!s.b[771])) && s.b[772]) {
            s.store_scalar(648, 2.0);
        }

        s.b[773] = (1.0 == 4.0);
        s.store_scalar(773, if s.b[773] { 1.0 } else { 0.0 });

        if ((((((!s.b[725]) && s.b[769]) && s.b[770]) && (!s.b[771])) && (!s.b[772])) && s.b[773]) {
            s.store_scalar(648, 3.0);
        }

        s.b[774] = (1.0 == 8.0);
        s.store_scalar(774, if s.b[774] { 1.0 } else { 0.0 });

        if (((((((!s.b[725]) && s.b[769]) && s.b[770]) && (!s.b[771])) && (!s.b[772])) && (!s.b[773])) && s.b[774]) {
            s.store_scalar(648, 4.0);
        }

        if (((!s.b[725]) && s.b[769]) && s.b[770]) {
            s.store_scalar(647, 0.0);
        }

        let mut assign9030_loop_guard: usize = 0;
        while {
            let assign9030_cond_e9536: f64 = if ((((!s.b[725]) && s.b[769]) && s.b[770]) && (s.v[647] < s.v[648])) { 1.0 } else { 0.0 };
            assign9030_cond_e9536 != 0.0
        } {
            assign9030_loop_guard += 1;
            assert!(assign9030_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((!s.b[725]) && s.b[769]) && s.b[770]) {
                s.store_sqrt(646, 646);
                s.store_offset(647, 647, 1.0);
            }
        }

        if (((!s.b[725]) && s.b[769]) && (!s.b[770])) {
            s.store_powf(646, 646, (1.0 / 2.0));
        }

        if ((!s.b[725]) && s.b[769]) {
            s.store_div_from_scalar_offset_input(646, 1.0, 646, 1e-50);
            s.store_scaled_mul(637, 638, 646, 0.15);
            s.store_div_scaled_product_offset_denominator(278, s.ad_value(645), s.ad_value(646), 0.15, s.ad_value(220), 1e-50, 1.0);
            s.store_add_offset_lhs(306, 305, (-0.15), 637);
        }

        if ((!s.b[725]) && s.b[769]) {
        }

        if ((!s.b[725]) && (!s.b[769])) {
        }

        if ((!s.b[725]) && (!s.b[769])) {
            s.store_scalar(278, 1.0);
        }

        if (!s.b[725]) {
            s.copy_ad(522, 306);
        }

        s.b[775] = ((p.p15 == 1.0) && (s.v[52] > (s.v[54] + 0.2)));
        s.store_scalar(775, if s.b[775] { 1.0 } else { 0.0 });

        if s.b[775] {
            s.store_scalar(389, s.v[559]);
            s.store_add_scaled_inputs4_indices(388, 72, 1.0, 389, (-1.0), 80, 1.0, 267, -1.0);
            s.store_scalar(32, p.p136);
            s.copy_ad(99, 388);
            s.store_sqrt_div_scaled_inputs(100, 471, ((2.0 * 1.6021918e-19) * 1.034943e-10), 120, 1.0);
            s.store_div_scaled_product_by_product(101, s.ad_value(127), s.ad_value(127), 1.0, s.ad_value(471), s.ad_value(471), 1.0);
            s.store_div_scaled_product_by_product(102, s.ad_value(100), s.ad_value(100), 1.0, s.ad_value(270), s.ad_value(270), 1.0);
            s.store_scaled_mul(103, 102, 120, 0.5);
            s.store_scaled_mul(104, 103, 120, 2.0);
            s.store_sqrt_offset_ad(105, A::div_scaled_offset_numerator(A::mul(s.ad_value(120), s.ad_value(99)), 4.0, ((-1.0) * 4.0), s.ad_value(104), 1.0), 1.0);
            s.store_add_mul_sub_from_scalar_rhs_indices(107, 99, 103, 1.0, 105);
            s.store_div_scalar_by_product(108, 1.0, s.ad_value(101), s.ad_value(102), 1.0);
            s.store_div_ad(109, A::ln(A::mul(s.ad_value(108), A::square(s.ad_value(99)))), A::add(s.ad_value(120), A::div_from_scalar(2.0, s.ad_value(99))));
            s.store_add_scaled_inputs3_indices(110, 109, 1.0, 107, (-1.0), 32, -1.0);
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(111, 109, 1.0, 110, (-0.5), A::add_scaled_square_product(s.ad_value(110), 1.0, s.ad_value(32), s.ad_value(109), 4.0), (-0.5));
            s.store_exp_mul(112, 120, 111);
            s.store_add_scaled_product_value_ad(113, A::offset(A::mul(s.ad_value(120), s.ad_value(111)), (-1.0)), 1.0, 101, 112, 1.0);
            s.store_offset_mul(114, 120, 111, (-1.0));
        }

    }

    pub(super) fn stamp_reactive_block_7(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        s.b[776] = ((s.v[113] > 0.0) && (s.v[114] > 0.0));
        s.store_scalar(776, if s.b[776] { 1.0 } else { 0.0 });

        if (s.b[775] && s.b[776]) {
            s.store_sqrt_ad(113, A::add_scaled_product(A::offset(A::mul(s.ad_value(120), s.ad_value(111)), (-1.0)), 1.0, s.ad_value(101), s.ad_value(112), 1.0));
            s.store_sqrt_offset_ad(114, A::mul(s.ad_value(120), s.ad_value(111)), (-1.0));
            s.store_mul_sub_rhs(115, 100, 113, 114);
            s.store_div_from_scalar(106, (2.0 * s.v[124]), 120);
            s.store_scalar(158, (300.0 * 0.0001));
            s.store_scalar(262, 0.0);
            s.store_scalar(279, 0.0);
            s.store_div_scaled_product_mixed_aia(116, A::mul3(s.ad_value(106), s.ad_value(158), s.ad_value(115)), 279, 1.0, A::sub(s.ad_value(123), s.ad_value(262)), 1.0);
            s.copy_ad(338, 116);
            s.copy_ad(339, 111);
            s.store_offset_div_scaled_offset_numerator(290, A::mul(s.ad_value(120), s.ad_value(76)), 4.0, ((-1.0) * 4.0), A::mul(s.ad_value(145), s.ad_value(121)), 1.0, 1.0);
        }

        s.b[777] = (s.v[290] < (10.0 * 2.220446049250313e-16));
        s.store_scalar(777, if s.b[777] { 1.0 } else { 0.0 });

        if ((s.b[775] && s.b[776]) && s.b[777]) {
            s.store_scalar(290, (10.0 * 2.220446049250313e-16));
        }

        if (s.b[775] && s.b[776]) {
            s.store_add_product3_rhs_mixed_iia(319, 76, 145, 120, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(290))), 0.5);
            s.copy_ad(58, 319);
            s.store_sub(61, 319, 339);
        }

        s.b[778] = (s.v[61] < 0.0);
        s.store_scalar(778, if s.b[778] { 1.0 } else { 0.0 });

        if ((s.b[775] && s.b[776]) && s.b[778]) {
            s.store_scalar(61, 0.0);
        }

        if (s.b[775] && s.b[776]) {
            s.store_scale(283, 61, (1.0 + 0.3));
            s.store_offset_sub(284, 283, 71, (-0.03));
            s.store_sqrt_add_scaled_square_input(285, 284, 1.0, 283, (4.0 * 0.03));
            s.store_add_scaled_inputs3_indices(60, 283, 1.0, 284, (-0.5), 285, (-0.5));
        }

        s.b[779] = (s.v[60] > s.v[61]);
        s.store_scalar(779, if s.b[779] { 1.0 } else { 0.0 });

        if ((s.b[775] && s.b[776]) && s.b[779]) {
            s.copy_ad(60, 61);
        }

        if (s.b[775] && s.b[776]) {
            s.copy_ad(392, 60);
            s.store_scalar(796, (s.v[272] * 100.0));
            s.store_scalar(797, (s.v[466] * 100.0));
            s.store_scale(798, 123, 100.0);
        }

        s.b[799] = (p.p26 == 0.0);
        s.store_scalar(799, if s.b[799] { 1.0 } else { 0.0 });

        if ((s.b[775] && s.b[776]) && (!s.b[799])) {
            s.store_scalar(391, 4.12);
            s.store_scaled_mul(780, 797, 798, (p.p141 * 1.6021918e-19));
            s.store_div(781, 780, 245);
            s.store_div_scaled_inputs_mixed_ai(782, A::offset(A::add_scaled_inputs4(s.ad_value(70), p.p144, s.ad_value(82), 1.0, s.ad_value(266), 1.0, s.ad_value(137), 1.0), p.p143), -1.0, 796, 1.0);
            s.store_scalar(514, 0.0);
        }

        let mut assign9680_loop_guard: usize = 0;
        while {
            let assign9680_cond_e10183: f64 = (100.0 - 1.0);
            let assign9680_cond_e10185: f64 = if (((s.b[775] && s.b[776]) && (!s.b[799])) && (s.v[514] <= assign9680_cond_e10183)) { 1.0 } else { 0.0 };
            assign9680_cond_e10185 != 0.0
        } {
            assign9680_loop_guard += 1;
            assert!(assign9680_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.b[775] && s.b[776]) && (!s.b[799])) {
                s.copy_ad(783, 514);
                s.store_scalar(784, 100.0);
                s.store_div(785, 783, 784);
                s.store_add_scaled_inputs3_mixed_iia(786, 53, 1.0, 73, 1.0, A::add_scaled_product(s.ad_value(339), 1.0, s.ad_value(392), s.ad_value(785), 1.0), -1.0);
                s.store_sub_from_scalar_div_indices(787, 1.0, 786, 391);
                s.store_add_div_rhs_indices(790, 782, 786, 796);
                s.store_square(788, 790);
                s.store_sqrt_square_offset(639, 787, ((4.0 * 0.001) * 0.001));
                s.store_offset_add_scaled_inputs_indices(787, 787, 0.5, 639, 0.5, (1e-10 * 0.001));
            }
            s.b[800] = (s.v[787] < 0.0);
            s.store_scalar(800, if s.b[800] { 1.0 } else { 0.0 });
            if (((s.b[775] && s.b[776]) && (!s.b[799])) && s.b[800]) {
                s.store_scalar(787, 0.0);
            }
            if ((s.b[775] && s.b[776]) && (!s.b[799])) {
                s.store_offset_scaled_ad(789, A::mul(A::sqrt(s.ad_value(787)), s.ad_value(787)), (-p.p142), p.p142);
                s.store_div_scaled_inputs_indices(791, 789, -1.0, 790, 1.0);
            }
            s.b[801] = (s.v[791] < (-34.0));
            s.store_scalar(801, if s.b[801] { 1.0 } else { 0.0 });
            if (((s.b[775] && s.b[776]) && (!s.b[799])) && s.b[801]) {
                s.store_scalar(792, 0.0);
            }
            if (((s.b[775] && s.b[776]) && (!s.b[799])) && (!s.b[801])) {
                s.store_exp(792, 791);
            }
            if ((s.b[775] && s.b[776]) && (!s.b[799])) {
                s.copy_ad(793, 781);
                s.store_mul3_affine_lhs(794, 793, 789, (0.25 * 7.38905609893065), 0.0, 789);
            }
            s.b[802] = (((2.0 * s.v[790]) + s.v[789]) < 0.0);
            s.store_scalar(802, if s.b[802] { 1.0 } else { 0.0 });
            if (((s.b[775] && s.b[776]) && (!s.b[799])) && s.b[802]) {
                s.copy_ad(393, 794);
            }
            if (((s.b[775] && s.b[776]) && (!s.b[799])) && (!s.b[802])) {
                s.store_mul3_lhs(795, 780, 788, 792);
            }
            s.b[803] = ((s.v[795] < s.v[794]) || (s.v[790] < 0.0));
            s.store_scalar(803, if s.b[803] { 1.0 } else { 0.0 });
            if ((((s.b[775] && s.b[776]) && (!s.b[799])) && (!s.b[802])) && s.b[803]) {
                s.copy_ad(393, 794);
            }
            if ((((s.b[775] && s.b[776]) && (!s.b[799])) && (!s.b[802])) && (!s.b[803])) {
                s.copy_ad(393, 795);
            }
            s.b[804] = (s.v[393] < 1e-9);
            s.store_scalar(804, if s.b[804] { 1.0 } else { 0.0 });
            if (((s.b[775] && s.b[776]) && (!s.b[799])) && s.b[804]) {
                s.store_scalar(514, 100.0);
                s.store_scalar(62, s.v[28]);
            }
            if ((s.b[775] && s.b[776]) && (!s.b[799])) {
                s.store_offset(514, 514, 1.0);
            }
        }

        s.b[805] = ((s.v[488] <= 0.0) || (s.v[162] <= 0.0));
        s.store_scalar(805, if s.b[805] { 1.0 } else { 0.0 });

        if ((s.b[775] && s.b[776]) && s.b[805]) {
            s.store_scalar(185, 0.0);
        }

        if ((s.b[775] && s.b[776]) && (!s.b[805])) {
            s.copy_ad(279, 388);
            s.store_square(285, 270);
            s.store_mul_div_from_scalar_lhs(282, 2.0, 472, 285);
            s.store_add_scaled_inputs3_indices(283, 279, 1.0, 122, (-1.0), 70, (-s.v[486]));
            s.store_offset_mul(284, 282, 283, 1.0);
            s.store_sqrt_square_offset(639, 284, ((4.0 * 0.001) * 0.001));
            s.store_offset_scaled_div(287, 284, 639, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(284, 284, 0.5, 639, 0.5, (1e-10 * 0.001));
        }

        s.b[806] = (s.v[284] < 0.0);
        s.store_scalar(806, if s.b[806] { 1.0 } else { 0.0 });

        if (((s.b[775] && s.b[776]) && (!s.b[805])) && s.b[806]) {
            s.store_scalar(284, 0.0);
            s.store_scalar(287, 0.0);
        }

        if ((s.b[775] && s.b[776]) && (!s.b[805])) {
            s.store_offset(284, 284, 1e-50);
            s.store_add_scaled_ad_rhs(186, 279, s.v[491], A::mul_sub_from_scalar_rhs(A::div(s.ad_value(472), s.ad_value(285)), 1.0, A::sqrt(s.ad_value(284))));
            s.store_add_scaled_inputs3_indices(187, 71, p.p123, 339, 1.0, 186, (-(s.v[487] * s.v[485])));
            s.store_sqrt_square_offset(639, 187, ((4.0 * 0.01) * 0.01));
            s.store_offset_scaled_div(287, 187, 639, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(187, 187, 0.5, 639, 0.5, (1e-10 * 0.01));
        }

        s.b[807] = (s.v[187] < 0.0);
        s.store_scalar(807, if s.b[807] { 1.0 } else { 0.0 });

        if (((s.b[775] && s.b[776]) && (!s.b[805])) && s.b[807]) {
            s.store_scalar(187, 0.0);
            s.store_scalar(287, 0.0);
        }

        if ((s.b[775] && s.b[776]) && (!s.b[805])) {
            s.store_offset(187, 187, 1e-50);
            s.store_exp_ad(280, A::div_from_scalar((-s.v[489]), s.ad_value(187)));
            s.store_mul3_affine_lhs(185, 187, 338, s.v[488], 0.0, 280);
        }

        s.b[808] = (p.p16 == 1.0);
        s.store_scalar(808, if s.b[808] { 1.0 } else { 0.0 });

        if ((s.b[775] && s.b[776]) && s.b[808]) {
            s.store_scaled_exp_scaled_input(279, 120, (-p.p140), ((1.6021918e-19 * p.p227) * s.v[466]));
            s.store_offset_scaled(280, 471, (((((36.0 * 1e-7) / 0.0001)) as f64).sqrt() * 13.0), ((((((13.0 * 1e-7) / 0.0001)) as f64).sqrt() * 36.0) * (1e20 / 1e-6)));
            s.store_div_from_scalar_mul_ad(282, (((((13.0 * 1e-7) / 0.0001)) as f64).sqrt() * ((((36.0 * 1e-7) / 0.0001)) as f64).sqrt()), s.ad_value(279), s.ad_value(280));
            s.store_scale(283, 122, 0.0);
            s.store_sqrt_mul_scaled_lhs(284, 471, ((2.0 * 1.034943e-10) * 1.6021918e-19), 122);
            s.store_sqrt_mul_sub_rhs(285, 120, 339, 283);
            s.store_sqrt_mul(286, 120, 339);
            s.store_mul_sub_scaled_inputs_rhs(337, 284, s.ad_value(285), -1.0, s.ad_value(286), -1.0);
        }

        if (((s.b[775] && s.b[776]) && s.b[808]) && (p.p27 != 0.0)) {
            s.store_div_from_scalar_offset_input(342, p.p137, 185, p.p138);
            s.store_mul(341, 342, 270);
            s.copy_ad(340, 337);
            s.store_scaled_voltage(562, ctx, nodes, Some(10), None, 1e-9);
            s.copy_ad(337, 562);
            s.store_div_scaled_inputs2_indices(558, 562, 1.0, 340, (-1.0), 341, 1.0);
        }

        if ((s.b[775] && s.b[776]) && (!s.b[808])) {
            s.store_scalar(337, 0.0);
        }

        if (s.b[775] && (!s.b[776])) {
            s.store_scalar(185, 0.0);
            s.store_scalar(337, 0.0);
        }

        if (!s.b[775]) {
            s.store_scalar(185, 0.0);
            s.store_scalar(337, 0.0);
        }

        s.copy_ad(299, 305);

        s.copy_ad(300, 306);

        s.store_sub(301, 307, 440);

        s.store_scalar(379, 0.0);

        s.store_scalar(606, 1.0);

        s.store_scalar(604, 0.0);

        s.store_scalar(605, 0.0);

        s.b[809] = (s.v[649] < 4.0);
        s.store_scalar(809, if s.b[809] { 1.0 } else { 0.0 });

        if s.b[809] {
            s.copy_ad(599, 296);
            s.store_neg(600, 599);
            s.store_div_from_scalar_mul_ad(601, 0.004832, A::square(s.ad_value(296)), s.ad_value(296));
            s.store_scale(603, 296, (-3.7477));
            s.store_scale(602, 296, 4.3495);
        }

        if (!s.b[809]) {
            s.store_scale(599, 296, 1.5);
            s.store_neg(600, 599);
            s.store_div_from_scalar_mul_ad(601, 0.001765, A::square(s.ad_value(296)), s.ad_value(296));
            s.store_scale(603, 296, (-4.8303));
            s.store_scale(602, 296, 5.9661);
        }

        s.copy_ad(306, 300);

        s.copy_ad(534, 300);

        s.copy_ad(522, 534);

        s.copy_ad(307, 301);

        s.store_scalar(62, 1.0);

    }

    pub(super) fn stamp_reactive_block_8(
        s: &mut ReactiveScratch,
    ) {
        let mut assign10390_loop_guard: usize = 0;
        while {
            let assign10390_cond_e11185: f64 = if s.v[62] <= s.v[28] { 1.0 } else { 0.0 };
            assign10390_cond_e11185 != 0.0
        } {
            assign10390_loop_guard += 1;
            assert!(assign10390_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            s.copy_ad(279, 307);
            s.store_mul(297, 120, 279);
            s.store_exp_neg_input(278, 297);
            s.b[810] = (s.v[279] < (-1e-8));
            s.store_scalar(810, if s.b[810] { 1.0 } else { 0.0 });
            if s.b[810] {
                s.store_exp_mul(280, 120, 307);
                s.store_mul_sqrt_ad_rhs(312, 439, A::add_scaled_offset_product_rhs(A::offset(A::add(s.ad_value(278), s.ad_value(297)), (-1.0)), 1.0, s.ad_value(143), s.ad_value(280), (-1.0), 1.0));
                s.store_div_scaled_product_right_ad(343, 438, A::add_scaled_sub_value_product(1.0, s.ad_value(278), 1.0, s.ad_value(143), s.ad_value(280), 1.0), 1.0, 312, 1.0);
            }
            s.b[811] = (s.v[279] > (1e-8 / 10.0));
            s.store_scalar(811, if s.b[811] { 1.0 } else { 0.0 });
            if ((!s.b[810]) && s.b[811]) {
                s.store_exp_mul(280, 120, 307);
                s.store_mul_scaled_sqrt_ad_rhs(312, 439, -1.0, A::add_scaled_offset_product_rhs(A::offset(A::add(s.ad_value(278), s.ad_value(297)), (-1.0)), 1.0, s.ad_value(143), A::sub(s.ad_value(280), s.ad_value(297)), (-1.0), 1.0));
                s.store_div_scaled_product_right_ad(343, 438, A::add_scaled_sub_value_product(1.0, s.ad_value(278), 1.0, s.ad_value(143), A::offset(s.ad_value(280), (-1.0)), 1.0), 1.0, 312, 1.0);
            }
            if ((!s.b[810]) && (!s.b[811])) {
                s.store_scaled_mul(312, 439, 297, (-1.0 / (((2.0) as f64).sqrt())));
                s.store_scaled_mul(343, 439, 120, (-1.0 / (((2.0) as f64).sqrt())));
            }
            s.store_add_scaled_inputs4_indices(306, 307, 1.0, 312, (-1.0 / (s.v[294])), 50, 1.0, 298, 1.0);
            s.store_sub_from_scalar_scaled_input(583, 1.0, 343, 1.0 / (s.v[294]));
            s.store_sub(279, 305, 522);
            s.store_mul(297, 120, 279);
            s.b[812] = ((-s.v[297]) >= 80.0);
            s.store_scalar(812, if s.b[812] { 1.0 } else { 0.0 });
            if s.b[812] {
                s.store_scaled_offset_ad(278, A::sub_from_scalar(1.0, s.ad_value(297)), (-80.0), 5.540622384e34);
                s.store_scalar(284, 5.540622384e34);
            }
            if (!s.b[812]) {
                s.store_exp_neg_input(278, 297);
                s.copy_ad(284, 278);
            }
            s.b[813] = (s.v[279] < (-1e-8));
            s.store_scalar(813, if s.b[813] { 1.0 } else { 0.0 });
            if s.b[813] {
                s.store_sqrt_offset_ad(280, A::add(s.ad_value(278), s.ad_value(297)), (-1.0));
                s.store_mul(523, 141, 280);
                s.store_div_scaled_product3_mixed_iiai(524, 141, 120, A::sub_from_scalar(1.0, s.ad_value(284)), 1.0, 280, 2.0);
                s.store_neg(525, 524);
                s.store_scalar(311, 0.0);
                s.store_scalar(526, 0.0);
                s.store_scalar(527, 0.0);
            }
            s.b[814] = (s.v[279] > 1e-8);
            s.store_scalar(814, if s.b[814] { 1.0 } else { 0.0 });
            if ((!s.b[813]) && s.b[814]) {
                s.store_sqrt_offset_ad(280, A::add(s.ad_value(278), s.ad_value(297)), (-1.0));
                s.store_mul_neg_lhs(523, 141, 280);
                s.store_div_scaled_product3_mixed_iiai(524, 141, 120, A::sub_from_scalar(1.0, s.ad_value(284)), -1.0, 280, 2.0);
                s.store_neg(525, 524);
                s.store_exp(278, 297);
                s.store_exp_mul(281, 120, 522);
                s.store_sqrt_add_ad(282, A::div_scaled_product(s.ad_value(523), s.ad_value(523), 1.0, A::square(s.ad_value(141)), 1.0), A::mul3_scaled_output(s.ad_value(142), s.ad_value(281), A::offset(A::sub(s.ad_value(278), s.ad_value(297)), (-1.0)), 2.0));
                s.store_div_scaled_inputs_mixed_ai(537, A::add_scaled_offset_product_rhs(A::div_scaled_product(s.ad_value(523), s.ad_value(524), 2.0, A::square(s.ad_value(141)), 1.0), 1.0, A::mul3_scaled_output(s.ad_value(120), s.ad_value(142), s.ad_value(281), 2.0), s.ad_value(278), (-1.0), 1.0), 1.0, 282, 2.0);
                s.store_div_scaled_add_product(538, A::div_scaled_product(s.ad_value(523), s.ad_value(525), 2.0, A::square(s.ad_value(141)), 1.0), 1.0, A::mul3_scaled_output(s.ad_value(120), s.ad_value(142), s.ad_value(281), 2.0), s.ad_value(297), (-1.0), s.ad_value(282), 2.0);
                s.store_add_scaled_product_indices(311, 523, (-1.0), 141, 282, -1.0);
                s.store_add_scaled_product_indices(526, 524, (-1.0), 141, 537, -1.0);
                s.store_add_scaled_product_indices(527, 525, (-1.0), 141, 538, -1.0);
            }
            if ((!s.b[813]) && (!s.b[814])) {
                s.store_scaled_mul(523, 141, 297, (-1.0 / (((2.0) as f64).sqrt())));
                s.store_scaled_mul(524, 141, 120, (-1.0 / (((2.0) as f64).sqrt())));
                s.store_neg(525, 524);
                s.store_scalar(311, 0.0);
                s.store_scalar(526, 0.0);
                s.store_scalar(527, 0.0);
            }
            s.store_sub(279, 306, 522);
            s.store_mul(297, 120, 279);
            s.b[815] = ((-s.v[297]) >= 80.0);
            s.store_scalar(815, if s.b[815] { 1.0 } else { 0.0 });
            if s.b[815] {
                s.store_scaled_offset_ad(278, A::sub_from_scalar(1.0, s.ad_value(297)), (-80.0), 5.540622384e34);
                s.store_scalar(284, 5.540622384e34);
            }
            if (!s.b[815]) {
                s.store_exp_neg_input(278, 297);
                s.copy_ad(284, 278);
            }
            s.b[816] = (s.v[279] < (-1e-8));
            s.store_scalar(816, if s.b[816] { 1.0 } else { 0.0 });
            if s.b[816] {
                s.store_sqrt_offset_ad(280, A::add(s.ad_value(278), s.ad_value(297)), (-1.0));
                s.store_mul(531, 141, 280);
                s.store_div_scaled_product3_mixed_iiai(532, 141, 120, A::sub_from_scalar(1.0, s.ad_value(284)), 1.0, 280, 2.0);
                s.store_neg(533, 532);
                s.store_scalar(528, 0.0);
                s.store_scalar(529, 0.0);
                s.store_scalar(530, 0.0);
            }
            s.b[817] = (s.v[279] > 1e-8);
            s.store_scalar(817, if s.b[817] { 1.0 } else { 0.0 });
            if ((!s.b[816]) && s.b[817]) {
                s.store_sqrt_offset_ad(280, A::add(s.ad_value(278), s.ad_value(297)), (-1.0));
                s.store_mul_neg_lhs(531, 141, 280);
                s.store_div_scaled_product3_mixed_iiai(532, 141, 120, A::sub_from_scalar(1.0, s.ad_value(284)), -1.0, 280, 2.0);
                s.store_neg(533, 532);
                s.store_exp(278, 297);
                s.store_exp_mul(281, 120, 522);
                s.store_sqrt_add_ad(282, A::div_scaled_product(s.ad_value(531), s.ad_value(531), 1.0, A::square(s.ad_value(141)), 1.0), A::mul3_scaled_output(s.ad_value(142), s.ad_value(281), A::offset(A::sub(s.ad_value(278), s.ad_value(297)), (-1.0)), 2.0));
                s.store_div_scaled_inputs_mixed_ai(539, A::add_scaled_offset_product_rhs(A::div_scaled_product(s.ad_value(531), s.ad_value(532), 2.0, A::square(s.ad_value(141)), 1.0), 1.0, A::mul3_scaled_output(s.ad_value(120), s.ad_value(142), s.ad_value(281), 2.0), s.ad_value(278), (-1.0), 1.0), 1.0, 282, 2.0);
                s.store_div_scaled_add_product(538, A::div_scaled_product(s.ad_value(531), s.ad_value(533), 2.0, A::square(s.ad_value(141)), 1.0), 1.0, A::mul3_scaled_output(s.ad_value(120), s.ad_value(142), s.ad_value(281), 2.0), s.ad_value(297), (-1.0), s.ad_value(282), 2.0);
                s.store_add_scaled_product_indices(528, 531, (-1.0), 141, 282, -1.0);
                s.store_add_scaled_product_indices(529, 532, (-1.0), 141, 539, -1.0);
                s.store_add_scaled_product_indices(530, 533, (-1.0), 141, 538, -1.0);
            }
            if ((!s.b[816]) && (!s.b[817])) {
                s.store_scaled_mul(531, 141, 297, (-1.0 / (((2.0) as f64).sqrt())));
                s.store_scaled_mul(532, 141, 120, (-1.0 / (((2.0) as f64).sqrt())));
                s.store_neg(533, 532);
                s.store_scalar(528, 0.0);
                s.store_scalar(529, 0.0);
                s.store_scalar(530, 0.0);
            }
            s.b[818] = (s.v[379] == 1.0);
            s.store_scalar(818, if s.b[818] { 1.0 } else { 0.0 });
            if s.b[818] {
                s.store_scalar(574, s.v[62]);
                s.store_scalar(62, s.v[28]);
            }
            if (!s.b[818]) {
                s.store_add_scaled_inputs3_mixed_iia(346, 305, 1.0, 76, (-1.0), A::div(A::add(A::add(A::add_scaled_inputs4(s.ad_value(312), 1.0, s.ad_value(311), 1.0, s.ad_value(523), 1.0, s.ad_value(528), 1.0), s.ad_value(531)), s.ad_value(337)), s.ad_value(270)), -1.0);
                s.store_sub_from_scalar_ad(347, 1.0, A::div_scaled_inputs2(s.ad_value(526), 1.0, s.ad_value(524), 1.0, s.ad_value(270), 1.0));
                s.store_div_scaled_inputs_mixed_ai(348, A::add_scaled_inputs4(s.ad_value(527), 1.0, s.ad_value(525), 1.0, s.ad_value(530), 1.0, s.ad_value(533), 1.0), -1.0, 270, 1.0);
                s.store_div_scaled_inputs_mixed_ai(349, A::add_scaled_product(s.ad_value(343), 1.0, A::add(s.ad_value(529), s.ad_value(532)), s.ad_value(583), 1.0), -1.0, 270, 1.0);
            }
            s.b[819] = (s.v[312] <= s.v[599]);
            s.store_scalar(819, if s.b[819] { 1.0 } else { 0.0 });
            if ((!s.b[818]) && s.b[819]) {
                s.store_sqrt_mul_ad(279, s.ad_value(296), A::add_scaled_inputs(s.ad_value(312), 2.0, s.ad_value(296), 1.0));
                s.store_div_scaled_product_indices(604, 296, 343, 1.0, 279, 1.0);
            }
            s.b[820] = (s.v[312] <= s.v[603]);
            s.store_scalar(820, if s.b[820] { 1.0 } else { 0.0 });
            if (((!s.b[818]) && (!s.b[819])) && s.b[820]) {
                s.store_mul3_ad(279, A::mul3(s.ad_value(601), A::sub(s.ad_value(312), s.ad_value(603)), A::sub(s.ad_value(312), s.ad_value(603))), A::sub(s.ad_value(312), s.ad_value(603)), A::sub(s.ad_value(312), s.ad_value(602)));
                s.store_mul_ad_product_lhs(604, A::mul3(s.ad_value(601), A::sub(s.ad_value(312), s.ad_value(603)), A::sub(s.ad_value(312), s.ad_value(603))), A::add_scaled_inputs4(s.ad_value(312), 3.0, s.ad_value(602), (-3.0), s.ad_value(312), 1.0, s.ad_value(603), (-1.0)), 343);
            }
            if (((!s.b[818]) && (!s.b[819])) && (!s.b[820])) {
                s.store_scalar(279, 0.0);
                s.store_scalar(604, 0.0);
            }
            if (!s.b[818]) {
                s.store_div_scaled_inputs_indices(281, 316, (-s.v[650]), 296, 1.0);
                s.store_div_from_scalar_offset_ad(280, 1.0, A::exp_scaled_input(s.ad_value(281), -1.0), 1.0);
                s.store_mul_square_exp_scaled_input(278, 280, 281, -1.0);
                s.store_mul(280, 280, 600);
                s.store_neg_add(279, 296, 280);
                s.store_scalar(604, 0.0);
                s.store_scaled_add(350, 523, 279, 1.0 / (s.v[535]));
                s.store_scale(351, 524, 1.0 / (s.v[535]));
                s.store_scale(352, 525, 1.0 / (s.v[535]));
                s.store_scale(353, 604, 1.0 / (s.v[535]));
                s.store_div_scaled_inputs_indices(281, 316, (-s.v[651]), 296, 1.0);
                s.store_div_from_scalar_offset_ad(280, 1.0, A::exp_scaled_input(s.ad_value(281), -1.0), 1.0);
                s.store_mul_square_exp_scaled_input(278, 280, 281, -1.0);
                s.store_mul(280, 280, 600);
                s.store_scalar(605, 0.0);
                s.store_scaled_add(354, 531, 280, 1.0 / (s.v[535]));
                s.store_scale(355, 533, 1.0 / (s.v[535]));
                s.store_add_scaled_product_indices(356, 605, 1.0 / (s.v[535]), 532, 583, 1.0 / (s.v[535]));
                s.store_add_scaled_inputs4(357, A::mul3(s.ad_value(347), s.ad_value(352), s.ad_value(356)), 1.0, A::mul3(s.ad_value(347), s.ad_value(353), s.ad_value(355)), (-1.0), A::mul3(s.ad_value(348), s.ad_value(351), s.ad_value(356)), -1.0, A::mul3(s.ad_value(349), s.ad_value(351), s.ad_value(355)), 1.0);
            }
            s.b[821] = (s.v[357] > 0.0);
            s.store_scalar(821, if s.b[821] { 1.0 } else { 0.0 });
            if ((!s.b[818]) && s.b[821]) {
                s.store_div_from_scalar_offset_input(358, 1.0, 357, 1e-50);
            }
            if ((!s.b[818]) && (!s.b[821])) {
                s.store_div_from_scalar_offset_input(358, 1.0, 357, (-1e-50));
            }
            if (!s.b[818]) {
                s.store_add_scaled_products_indices(359, 352, 356, 1.0, 353, 355, (-1.0));
                s.store_add_scaled_products_indices(360, 349, 355, 1.0, 348, 356, (-1.0));
                s.store_add_scaled_products_indices(361, 348, 353, 1.0, 349, 352, (-1.0));
                s.store_mul_neg_lhs(362, 351, 356);
                s.store_mul(363, 347, 356);
                s.store_add_scaled_products_indices(364, 349, 351, 1.0, 347, 353, (-1.0));
                s.store_mul(365, 351, 355);
                s.store_mul_neg_lhs(366, 347, 355);
                s.store_add_scaled_products_indices(367, 347, 352, 1.0, 348, 351, (-1.0));
                s.store_mul_add_scaled_products3_indices_rhs(368, 358, 359, 346, -1.0, 360, 350, -1.0, 361, 354, -1.0);
                s.store_mul_add_scaled_products3_indices_rhs(369, 358, 362, 346, -1.0, 363, 350, -1.0, 364, 354, -1.0);
                s.store_mul_add_scaled_products3_indices_rhs(370, 358, 365, 346, -1.0, 366, 350, -1.0, 367, 354, -1.0);
                s.store_abs(279, 368);
            }
            s.b[822] = (s.v[279] < ((s.v[369]) as f64).abs());
            s.store_scalar(822, if s.b[822] { 1.0 } else { 0.0 });
            if ((!s.b[818]) && s.b[822]) {
                s.store_abs(279, 369);
            }
            s.b[823] = (s.v[279] < ((s.v[370]) as f64).abs());
            s.store_scalar(823, if s.b[823] { 1.0 } else { 0.0 });
            if ((!s.b[818]) && s.b[823]) {
                s.store_abs(279, 370);
            }
            if (!s.b[818]) {
                s.store_scalar(606, 1.0);
            }
            s.b[824] = (s.v[62] > 80.0);
            s.store_scalar(824, if s.b[824] { 1.0 } else { 0.0 });
            if ((!s.b[818]) && s.b[824]) {
                s.store_scalar(606, 25.0);
            }
            s.b[825] = (s.v[62] > 40.0);
            s.store_scalar(825, if s.b[825] { 1.0 } else { 0.0 });
            if (((!s.b[818]) && (!s.b[824])) && s.b[825]) {
                s.store_scalar(606, 25.0);
            }
            s.b[826] = (s.v[62] > 20.0);
            s.store_scalar(826, if s.b[826] { 1.0 } else { 0.0 });
            if ((((!s.b[818]) && (!s.b[824])) && (!s.b[825])) && s.b[826]) {
                s.store_scalar(606, 25.0);
            }
            s.b[827] = (s.v[62] > 10.0);
            s.store_scalar(827, if s.b[827] { 1.0 } else { 0.0 });
            if (((((!s.b[818]) && (!s.b[824])) && (!s.b[825])) && (!s.b[826])) && s.b[827]) {
                s.store_scalar(606, 5.0);
            }
            s.b[828] = (s.v[279] > (0.1 / s.v[606]));
            s.store_scalar(828, if s.b[828] { 1.0 } else { 0.0 });
            if ((!s.b[818]) && s.b[828]) {
                s.store_mul_ad_rhs(368, 368, A::div_scalar_by_product(0.1, s.ad_value(606), s.ad_value(279), 1.0));
                s.store_mul_ad_rhs(369, 369, A::div_scalar_by_product(0.1, s.ad_value(606), s.ad_value(279), 1.0));
                s.store_mul_ad_rhs(370, 370, A::div_scalar_by_product(0.1, s.ad_value(606), s.ad_value(279), 1.0));
            }
            if (!s.b[818]) {
                s.store_add(305, 305, 368);
                s.store_add(522, 522, 369);
                s.store_add(307, 307, 370);
                s.store_scale(607, 606, 1e-12);
            }
            s.b[829] = (s.v[279] < s.v[607]);
            s.store_scalar(829, if s.b[829] { 1.0 } else { 0.0 });
            if ((!s.b[818]) && s.b[829]) {
                s.store_scalar(379, 1.0);
            }
            s.store_offset(62, 62, 1.0);
        }

        s.b[830] = (s.v[574] > 0.0);
        s.store_scalar(830, if s.b[830] { 1.0 } else { 0.0 });

        if s.b[830] {
            s.copy_ad(62, 574);
            s.store_scalar(574, 0.0);
        }

        s.b[831] = (s.v[62] > s.v[28]);
        s.store_scalar(831, if s.b[831] { 1.0 } else { 0.0 });

        if s.b[831] {
            s.copy_ad(305, 299);
            s.copy_ad(306, 300);
            s.copy_ad(307, 301);
            s.copy_ad(522, 534);
        }

        s.copy_ad(56, 305);

        s.store_neg(149, 311);

        s.b[833] = (s.v[149] <= 1e-50);
        s.store_scalar(833, if s.b[833] { 1.0 } else { 0.0 });

        if s.b[833] {
            s.store_scalar(149, 1e-50);
            s.store_scalar(34, 1.0);
        }

        s.store_neg(150, 528);

        s.b[834] = (s.v[150] <= 1e-50);
        s.store_scalar(834, if s.b[834] { 1.0 } else { 0.0 });

        if s.b[834] {
            s.store_scalar(150, 1e-50);
        }

        s.store_mul(86, 149, 271);

        s.copy_ad(396, 51);

        s.store_div_square_rhs(280, 472, 270);

        s.store_sub(278, 76, 122);

        s.store_offset_mul_ad(287, A::div_from_scalar(2.0, s.ad_value(280)), s.ad_value(278), 1.0);

        s.store_sqrt_square_offset(639, 287, ((4.0 * 0.05) * 0.05));

        s.store_offset_scaled_div(284, 287, 639, 0.5, 0.5);

        s.store_offset_add_scaled_inputs_indices(287, 287, 0.5, 639, 0.5, (1e-10 * 0.05));

    }

    pub(super) fn stamp_reactive_block_9(
        s: &mut ReactiveScratch,
    ) {
        s.b[835] = (s.v[287] < 0.0);
        s.store_scalar(835, if s.b[835] { 1.0 } else { 0.0 });

        if s.b[835] {
            s.store_scalar(287, 0.0);
            s.store_scalar(284, 0.0);
        }

        s.store_sqrt(281, 287);

        s.store_add_mul_sub_from_scalar_rhs_indices(288, 76, 280, 1.0, 281);

        s.store_sqrt_square_offset(639, 288, ((4.0 * 0.01) * 0.01));

        s.store_offset_scaled_div(278, 288, 639, 0.5, 0.5);

        s.store_offset_add_scaled_inputs_indices(288, 288, 0.5, 639, 0.5, (1e-10 * 0.01));

        s.b[836] = (s.v[288] < 0.0);
        s.store_scalar(836, if s.b[836] { 1.0 } else { 0.0 });

        if s.b[836] {
            s.store_scalar(288, 0.0);
            s.store_scalar(278, 0.0);
        }

        s.copy_ad(89, 288);

        s.store_offset_div(279, 51, 89, 1e-50);

        s.store_powf(280, 279, (s.v[481] - 1.0));

        s.store_offset_mul(281, 280, 279, 1.0);

        s.store_powf(282, 281, ((1.0 / s.v[481]) - 1.0));

        s.store_mul(284, 282, 281);

        s.store_div(395, 51, 284);

        s.copy_ad(51, 395);

        s.b[837] = (s.v[51] < 0.0);
        s.store_scalar(837, if s.b[837] { 1.0 } else { 0.0 });

        if s.b[837] {
            s.copy_ad(57, 56);
            s.store_sub(59, 57, 56);
            s.copy_ad(308, 57);
            s.copy_ad(309, 306);
            s.copy_ad(584, 522);
            s.copy_ad(310, 307);
            s.store_scalar(379, 1.0);
        }

        s.b[838] = ((s.v[33] >= 1.0) || (s.v[86] < 1e-12));
        s.store_scalar(838, if s.b[838] { 1.0 } else { 0.0 });

        if ((!s.b[837]) && s.b[838]) {
            s.store_scalar(308, s.v[698]);
            s.store_scalar(309, s.v[699]);
            s.store_offset(310, 440, s.v[700]);
        }

        if ((!s.b[837]) && (!s.b[838])) {
            if ((s.v[58] - s.v[305]) >= 0.0) {
                s.store_sub(61, 58, 305);
            } else {
                s.store_scalar(61, 0.0);
            }
        }

        if ((!s.b[837]) && (!s.b[838])) {
            s.store_offset_sub_scaled_inputs_indices(638, 61, (1.0 + (0.3 * 0.5)), 51, 1.0, (-0.03));
            s.store_scale(639, 61, ((1.0 + (0.3 * 0.5)) * (4.0 * 0.03)));
        }

        if ((!s.b[837]) && (!s.b[838])) {
            if (s.v[639] > 0.0) {
            } else {
                s.store_neg(639, 639);
            }
        }

        if ((!s.b[837]) && (!s.b[838])) {
            s.store_sqrt_square_add(639, 638, 639);
            s.store_offset_scaled_div(279, 638, 639, 0.5, 0.5);
            s.store_add_scaled_inputs3_indices(60, 61, (1.0 + (0.3 * 0.5)), 638, (-0.5), 639, (-0.5));
        }

        if ((!s.b[837]) && (!s.b[838])) {
            if (s.v[60] <= s.v[61]) {
            } else {
                s.copy_ad(60, 61);
            }
        }

        s.b[839] = (s.v[60] < 0.0);
        s.store_scalar(839, if s.b[839] { 1.0 } else { 0.0 });

        if (((!s.b[837]) && (!s.b[838])) && s.b[839]) {
            s.store_scalar(60, 0.0);
        }

        s.b[840] = (s.v[60] > s.v[51]);
        s.store_scalar(840, if s.b[840] { 1.0 } else { 0.0 });

        if ((((!s.b[837]) && (!s.b[838])) && (!s.b[839])) && s.b[840]) {
            s.copy_ad(60, 51);
        }

        if ((!s.b[837]) && (!s.b[838])) {
            s.copy_ad(59, 60);
            s.store_add(57, 305, 59);
            s.store_scalar(290, (1e-12 / 2.0));
        }

        s.b[841] = (s.v[57] < s.v[290]);
        s.store_scalar(841, if s.b[841] { 1.0 } else { 0.0 });

        if (((!s.b[837]) && (!s.b[838])) && s.b[841]) {
            s.copy_ad(57, 290);
        }

        if ((!s.b[837]) && (!s.b[838])) {
            s.copy_ad(308, 57);
        }

        if ((!s.b[837]) && (!s.b[838])) {
            if (s.v[292] == (-1.0)) {
                s.copy_ad(308, 305);
            } else {
                s.copy_ad(308, 57);
            }
        }

        if ((!s.b[837]) && (!s.b[838])) {
            s.store_scaled_square(278, 439, (s.v[293] * s.v[293]));
        }

        s.b[842] = (s.v[308] < s.v[329]);
        s.store_scalar(842, if s.b[842] { 1.0 } else { 0.0 });

        if (((!s.b[837]) && (!s.b[838])) && s.b[842]) {
            s.store_neg(279, 440);
            s.store_add_scaled_inputs3_mixed_aai(280, A::square(A::add_scaled_product(s.ad_value(279), 2.0, s.ad_value(278), s.ad_value(120), 1.0)), 1.0, A::square(s.ad_value(279)), (-4.0), 278, (-4.0));
        }

        if (((!s.b[837]) && (!s.b[838])) && s.b[842]) {
            if (s.v[280] >= (10.0 * 2.220446049250313e-16)) {
            } else {
                s.store_scalar(280, (10.0 * 2.220446049250313e-16));
            }
        }

        if (((!s.b[837]) && (!s.b[838])) && s.b[842]) {
            s.store_scaled_sub_ad(324, A::add_scaled_product(s.ad_value(279), 2.0, s.ad_value(278), s.ad_value(120), 1.0), A::sqrt(s.ad_value(280)), 0.5);
            s.store_div_ad(325, A::ln(A::div_scaled_product_by_product(s.ad_value(279), s.ad_value(279), 1.0, s.ad_value(278), s.ad_value(143), 1.0)), A::add(s.ad_value(120), A::div_from_scalar(2.0, s.ad_value(279))));
        }

        s.b[843] = (s.v[324] < s.v[326]);
        s.store_scalar(843, if s.b[843] { 1.0 } else { 0.0 });

        if ((((!s.b[837]) && (!s.b[838])) && s.b[842]) && s.b[843]) {
            s.copy_ad(310, 324);
        }

        if ((((!s.b[837]) && (!s.b[838])) && s.b[842]) && (!s.b[843])) {
            s.store_offset_sub(638, 325, 324, (-0.0008));
            s.store_scale(639, 325, (4.0 * 0.0008));
        }

        if ((((!s.b[837]) && (!s.b[838])) && s.b[842]) && (!s.b[843])) {
            if (s.v[639] > 0.0) {
            } else {
                s.store_neg(639, 639);
            }
        }

        if ((((!s.b[837]) && (!s.b[838])) && s.b[842]) && (!s.b[843])) {
            s.store_sqrt_square_add(639, 638, 639);
            s.store_offset_scaled_div(279, 638, 639, 0.5, 0.5);
            s.store_add_scaled_inputs3_indices(310, 325, 1.0, 638, (-0.5), 639, (-0.5));
        }

        if (((!s.b[837]) && (!s.b[838])) && (!s.b[842])) {
            s.store_add_scaled_inputs3_indices(279, 440, (-1.0), 308, (-(-1.0)), 296, (-(-(0.5 * s.v[536]))));
            s.store_add_scaled_inputs3_mixed_aai(280, A::square(A::add_scaled_product(s.ad_value(279), 2.0, s.ad_value(278), s.ad_value(120), 1.0)), 1.0, A::square(s.ad_value(279)), (-4.0), 278, (-4.0));
        }

        if (((!s.b[837]) && (!s.b[838])) && (!s.b[842])) {
            if (s.v[280] >= (10.0 * 2.220446049250313e-16)) {
            } else {
                s.store_scalar(280, (10.0 * 2.220446049250313e-16));
            }
        }

        if (((!s.b[837]) && (!s.b[838])) && (!s.b[842])) {
            s.store_scaled_sub_ad(324, A::add_scaled_product(s.ad_value(279), 2.0, s.ad_value(278), s.ad_value(120), 1.0), A::sqrt(s.ad_value(280)), 0.5);
            s.store_div_ad(325, A::ln(A::div_scaled_product_by_product(s.ad_value(279), s.ad_value(279), 1.0, s.ad_value(278), s.ad_value(143), 1.0)), A::add(s.ad_value(120), A::div_from_scalar(2.0, s.ad_value(279))));
        }

        s.b[844] = (s.v[324] < s.v[326]);
        s.store_scalar(844, if s.b[844] { 1.0 } else { 0.0 });

        if ((((!s.b[837]) && (!s.b[838])) && (!s.b[842])) && s.b[844]) {
            s.copy_ad(310, 324);
        }

        if ((((!s.b[837]) && (!s.b[838])) && (!s.b[842])) && (!s.b[844])) {
            s.store_offset_sub(638, 325, 324, (-0.0008));
            s.store_scale(639, 325, (4.0 * 0.0008));
        }

        if ((((!s.b[837]) && (!s.b[838])) && (!s.b[842])) && (!s.b[844])) {
            if (s.v[639] > 0.0) {
            } else {
                s.store_neg(639, 639);
            }
        }

        if ((((!s.b[837]) && (!s.b[838])) && (!s.b[842])) && (!s.b[844])) {
            s.store_sqrt_square_add(639, 638, 639);
            s.store_offset_scaled_div(279, 638, 639, 0.5, 0.5);
            s.store_add_scaled_inputs3_indices(310, 325, 1.0, 638, (-0.5), 639, (-0.5));
        }

        s.b[845] = ((s.v[308] < s.v[329]) && (0.0 != 0.0));
        s.store_scalar(845, if s.b[845] { 1.0 } else { 0.0 });

        if (((!s.b[837]) && (!s.b[838])) && s.b[845]) {
            s.store_scalar(63, 0.0);
        }

        let mut assign11450_loop_guard: usize = 0;
        while {
            let assign11450_cond_e13817: f64 = if ((((!s.b[837]) && (!s.b[838])) && s.b[845]) && (s.v[63] < s.v[29])) { 1.0 } else { 0.0 };
            assign11450_cond_e13817 != 0.0
        } {
            assign11450_loop_guard += 1;
            assert!(assign11450_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((!s.b[837]) && (!s.b[838])) && s.b[845]) {
                s.store_mul(280, 120, 310);
                s.store_exp_neg_input(281, 280);
            }
            s.b[846] = (s.v[310] > 1e-8);
            s.store_scalar(846, if s.b[846] { 1.0 } else { 0.0 });
            if ((((!s.b[837]) && (!s.b[838])) && s.b[845]) && s.b[846]) {
                s.store_exp_mul(278, 120, 310);
                s.store_mul_scaled_sqrt_ad_rhs(282, 439, -1.0, A::add_scaled_offset_product_rhs(A::offset(A::add(s.ad_value(281), s.ad_value(280)), (-1.0)), 1.0, s.ad_value(143), s.ad_value(278), (-1.0), 1.0));
                s.store_mul_div_scaled_inputs_mixed_aii(283, A::add_scaled_sub_value_product(1.0, s.ad_value(281), 1.0, s.ad_value(143), s.ad_value(278), 1.0), 438, 1.0, 282, 1.0);
            }
            s.b[847] = (s.v[310] < (-1e-8));
            s.store_scalar(847, if s.b[847] { 1.0 } else { 0.0 });
            if (((((!s.b[837]) && (!s.b[838])) && s.b[845]) && (!s.b[846])) && s.b[847]) {
                s.store_mul_sqrt_ad_rhs(282, 439, A::offset(A::add(s.ad_value(281), s.ad_value(280)), (-1.0)));
                s.store_mul_sub_from_scalar_rhs_ad_lhs(283, A::div(s.ad_value(438), s.ad_value(282)), 1.0, 281);
            }
            if (((((!s.b[837]) && (!s.b[838])) && s.b[845]) && (!s.b[846])) && (!s.b[847])) {
                s.store_mul_ad_affine_product_lhs(282, A::sqrt(A::div(s.ad_value(438), s.ad_value(120))), s.ad_value(120), -1.0, 0.0, 310);
                s.store_neg_ad(283, A::sqrt(A::mul(s.ad_value(438), s.ad_value(120))));
            }
            if (((!s.b[837]) && (!s.b[838])) && s.b[845]) {
                s.store_sqrt_square_offset(639, 282, ((4.0 * 1e-6) * 1e-6));
                s.store_offset_scaled_div(285, 282, 639, 0.5, 0.5);
                s.store_offset_add_scaled_inputs_indices(284, 282, 0.5, 639, 0.5, (1e-10 * 1e-6));
            }
            s.b[848] = (s.v[284] < 0.0);
            s.store_scalar(848, if s.b[848] { 1.0 } else { 0.0 });
            if ((((!s.b[837]) && (!s.b[838])) && s.b[845]) && s.b[848]) {
                s.store_scalar(284, 0.0);
                s.store_scalar(285, 0.0);
            }
            if (((!s.b[837]) && (!s.b[838])) && s.b[845]) {
                s.store_offset_sub_scaled_inputs_indices(638, 296, -1.0, 284, 1.0, (-1e-9));
                s.store_scale(639, 296, (-(4.0 * 1e-9)));
            }
            if (((!s.b[837]) && (!s.b[838])) && s.b[845]) {
                if (s.v[639] > 0.0) {
                } else {
                    s.store_neg(639, 639);
                }
            }
            if (((!s.b[837]) && (!s.b[838])) && s.b[845]) {
                s.store_sqrt_square_add(639, 638, 639);
                s.store_offset_scaled_div(286, 638, 639, 0.5, 0.5);
                s.store_add_scaled_inputs3_indices(284, 296, -1.0, 638, (-0.5), 639, (-0.5));
                s.store_mul3_lhs(285, 285, 283, 286);
                s.store_div_scaled_inputs_mixed_ai(332, A::square(s.ad_value(284)), ((0.5 * 9662367879.197212) * 6.241449993689894e18), 471, 1.0);
                s.store_div_scaled_product_indices(333, 332, 285, 2.0, 284, 1.0);
                s.store_sub_ad_rhs(284, 310, A::div_scaled_inputs4(s.ad_value(282), 1.0 / (s.v[294]), s.ad_value(310), (-1.0), s.ad_value(440), -1.0, s.ad_value(332), 1.0, A::add(A::scale_offset(s.ad_value(283), 1.0 / (s.v[294]), (-1.0)), s.ad_value(333)), 1.0));
            }
            s.b[849] = ((((s.v[284] - s.v[310])) as f64).abs() < 1e-12);
            s.store_scalar(849, if s.b[849] { 1.0 } else { 0.0 });
            if ((((!s.b[837]) && (!s.b[838])) && s.b[845]) && s.b[849]) {
                s.store_scalar(63, s.v[29]);
            }
            if (((!s.b[837]) && (!s.b[838])) && s.b[845]) {
                s.copy_ad(310, 284);
                s.copy_ad(314, 282);
                s.store_offset(63, 63, 1.0);
            }
        }

        if (((!s.b[837]) && (!s.b[838])) && s.b[845]) {
            s.store_add(310, 440, 310);
            s.store_sub_scaled_inputs(309, 310, 1.0, 314, 1.0 / (s.v[294]));
        }

        if (((!s.b[837]) && (!s.b[838])) && (!s.b[845])) {
            s.store_scalar(63, 0.0);
        }

    }

    pub(super) fn stamp_reactive_block_10(
        s: &mut ReactiveScratch,
    ) {
        let mut assign11490_loop_guard: usize = 0;
        while {
            let assign11490_cond_e14353: f64 = if ((((!s.b[837]) && (!s.b[838])) && (!s.b[845])) && (s.v[63] < s.v[29])) { 1.0 } else { 0.0 };
            assign11490_cond_e14353 != 0.0
        } {
            assign11490_loop_guard += 1;
            assert!(assign11490_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((!s.b[837]) && (!s.b[838])) && (!s.b[845])) {
                s.copy_ad(279, 439);
                s.store_mul(280, 120, 310);
                s.store_exp_neg_input(281, 280);
            }
            s.b[850] = (s.v[310] > 1e-8);
            s.store_scalar(850, if s.b[850] { 1.0 } else { 0.0 });
            if ((((!s.b[837]) && (!s.b[838])) && (!s.b[845])) && s.b[850]) {
                s.store_exp_mul(278, 120, 310);
                s.store_mul_scaled_sqrt_ad_rhs(282, 279, -1.0, A::add_scaled_offset_product_rhs(A::offset(A::add(s.ad_value(281), s.ad_value(280)), (-1.0)), 1.0, s.ad_value(143), s.ad_value(278), (-1.0), 1.0));
                s.store_mul_div_scaled_inputs_mixed_aii(283, A::add_scaled_sub_value_product(1.0, s.ad_value(281), 1.0, s.ad_value(143), s.ad_value(278), 1.0), 438, 1.0, 282, 1.0);
            }
            s.b[851] = (s.v[310] < (-1e-8));
            s.store_scalar(851, if s.b[851] { 1.0 } else { 0.0 });
            if (((((!s.b[837]) && (!s.b[838])) && (!s.b[845])) && (!s.b[850])) && s.b[851]) {
                s.store_mul_sqrt_ad_rhs(282, 279, A::offset(A::add(s.ad_value(281), s.ad_value(280)), (-1.0)));
                s.store_mul_sub_from_scalar_rhs_ad_lhs(283, A::div(s.ad_value(438), s.ad_value(282)), 1.0, 281);
            }
            if (((((!s.b[837]) && (!s.b[838])) && (!s.b[845])) && (!s.b[850])) && (!s.b[851])) {
                s.store_mul_ad_affine_product_lhs(282, A::sqrt(A::div(s.ad_value(438), s.ad_value(120))), s.ad_value(120), -1.0, 0.0, 310);
                s.store_neg_ad(283, A::sqrt(A::mul(s.ad_value(438), s.ad_value(120))));
            }
            if (((!s.b[837]) && (!s.b[838])) && (!s.b[845])) {
                s.store_sqrt_square_offset(639, 282, ((4.0 * 1e-6) * 1e-6));
                s.store_offset_scaled_div(285, 282, 639, 0.5, 0.5);
                s.store_offset_add_scaled_inputs_indices(284, 282, 0.5, 639, 0.5, (1e-10 * 1e-6));
            }
            s.b[852] = (s.v[284] < 0.0);
            s.store_scalar(852, if s.b[852] { 1.0 } else { 0.0 });
            if ((((!s.b[837]) && (!s.b[838])) && (!s.b[845])) && s.b[852]) {
                s.store_scalar(284, 0.0);
                s.store_scalar(285, 0.0);
            }
            if (((!s.b[837]) && (!s.b[838])) && (!s.b[845])) {
                s.store_offset_sub_scaled_inputs_indices(638, 296, -1.0, 284, 1.0, (-1e-9));
                s.store_scale(639, 296, (-(4.0 * 1e-9)));
            }
            if (((!s.b[837]) && (!s.b[838])) && (!s.b[845])) {
                if (s.v[639] > 0.0) {
                } else {
                    s.store_neg(639, 639);
                }
            }
            if (((!s.b[837]) && (!s.b[838])) && (!s.b[845])) {
                s.store_sqrt_square_add(639, 638, 639);
                s.store_offset_scaled_div(286, 638, 639, 0.5, 0.5);
                s.store_add_scaled_inputs3_indices(284, 296, -1.0, 638, (-0.5), 639, (-0.5));
                s.store_mul3_lhs(285, 285, 283, 286);
                s.store_div_scaled_inputs_mixed_ai(332, A::square(s.ad_value(284)), ((0.5 * 9662367879.197212) * 6.241449993689894e18), 471, 1.0);
                s.store_div_scaled_product_indices(333, 332, 285, 2.0, 284, 1.0);
                s.store_sub_div_rhs_ad(284, 310, A::add(A::sub(A::add(A::add_scaled_inputs3(s.ad_value(308), 1.0, s.ad_value(310), (-1.0), s.ad_value(282), 1.0 / (s.v[294])), A::add_scaled_inputs(s.ad_value(282), s.v[536], s.ad_value(296), (0.5 * s.v[536]))), s.ad_value(440)), s.ad_value(332)), A::add_scaled_inputs3_offset(s.ad_value(283), 1.0 / (s.v[294]), s.ad_value(283), s.v[536], s.ad_value(333), 1.0, (-1.0)));
            }
            s.b[853] = ((((s.v[284] - s.v[310])) as f64).abs() < 1e-12);
            s.store_scalar(853, if s.b[853] { 1.0 } else { 0.0 });
            if ((((!s.b[837]) && (!s.b[838])) && (!s.b[845])) && s.b[853]) {
                s.store_scalar(63, s.v[29]);
            }
            if (((!s.b[837]) && (!s.b[838])) && (!s.b[845])) {
                s.copy_ad(310, 284);
                s.copy_ad(314, 282);
                s.store_offset(63, 63, 1.0);
            }
        }

        if (((!s.b[837]) && (!s.b[838])) && (!s.b[845])) {
            s.store_add(310, 440, 310);
            s.store_sub_scaled_inputs(309, 310, 1.0, 314, 1.0 / (s.v[294]));
        }

        if ((!s.b[837]) && (!s.b[838])) {
            s.copy_ad(584, 309);
        }

        s.b[854] = (s.v[86] < 1e-12);
        s.store_scalar(854, if s.b[854] { 1.0 } else { 0.0 });

        if s.b[854] {
            s.copy_ad(302, 305);
            s.copy_ad(303, 306);
            s.copy_ad(304, 307);
            s.copy_ad(581, 522);
        }

        if (!s.b[854]) {
            s.copy_ad(302, 308);
            s.copy_ad(303, 309);
            s.store_sub(304, 310, 440);
        }

        if (!s.b[854]) {
            if (s.v[303] < s.v[302]) {
                s.copy_ad(581, 303);
            } else {
                s.copy_ad(581, 302);
            }
        }

        s.b[379] = (s.v[292] < 0.0);
        s.store_scalar(379, if s.b[379] { 1.0 } else { 0.0 });

        s.copy_ad(308, 302);

        s.copy_ad(309, 303);

        s.copy_ad(310, 304);

        s.copy_ad(584, 581);

        s.store_scalar(63, 1.0);

    }

    pub(super) fn stamp_reactive_block_11(
        s: &mut ReactiveScratch,
    ) {
        let mut assign11690_loop_guard: usize = 0;
        while {
            let assign11690_cond_e14989: f64 = if s.v[63] <= s.v[29] { 1.0 } else { 0.0 };
            assign11690_cond_e14989 != 0.0
        } {
            assign11690_loop_guard += 1;
            assert!(assign11690_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            s.copy_ad(279, 310);
            s.store_mul(297, 120, 279);
            s.store_exp_neg_input(278, 297);
            s.b[855] = (s.v[279] < (-1e-8));
            s.store_scalar(855, if s.b[855] { 1.0 } else { 0.0 });
            if s.b[855] {
                s.store_exp_mul(280, 120, 310);
                s.store_mul_sqrt_ad_rhs(314, 439, A::add_scaled_offset_product_rhs(A::offset(A::add(s.ad_value(278), s.ad_value(297)), (-1.0)), 1.0, s.ad_value(143), s.ad_value(280), (-1.0), 1.0));
                s.store_div_scaled_product_right_ad(344, 438, A::add_scaled_sub_value_product(1.0, s.ad_value(278), 1.0, s.ad_value(143), s.ad_value(280), 1.0), 1.0, 314, 1.0);
            }
            s.b[856] = (s.v[279] > (1e-8 / 10.0));
            s.store_scalar(856, if s.b[856] { 1.0 } else { 0.0 });
            if ((!s.b[855]) && s.b[856]) {
                s.store_exp_mul(280, 120, 310);
                s.store_mul_scaled_sqrt_ad_rhs(314, 439, -1.0, A::add_scaled_offset_product_rhs(A::offset(A::add(s.ad_value(278), s.ad_value(297)), (-1.0)), 1.0, s.ad_value(143), A::sub(s.ad_value(280), s.ad_value(297)), (-1.0), 1.0));
                s.store_div_scaled_product_right_ad(344, 438, A::add_scaled_sub_value_product(1.0, s.ad_value(278), 1.0, s.ad_value(143), A::offset(s.ad_value(280), (-1.0)), 1.0), 1.0, 314, 1.0);
            }
            if ((!s.b[855]) && (!s.b[856])) {
                s.store_scaled_mul(314, 439, 297, (-1.0 / (((2.0) as f64).sqrt())));
                s.store_scaled_mul(344, 439, 120, (-1.0 / (((2.0) as f64).sqrt())));
            }
            s.store_add_scaled_inputs4_indices(309, 310, 1.0, 314, (-1.0 / (s.v[294])), 50, 1.0, 298, 1.0);
            s.store_sub_from_scalar_scaled_input(582, 1.0, 344, 1.0 / (s.v[294]));
            s.store_sub(279, 308, 584);
            s.store_mul(297, 120, 279);
            s.b[857] = ((-s.v[297]) >= 80.0);
            s.store_scalar(857, if s.b[857] { 1.0 } else { 0.0 });
            if s.b[857] {
                s.store_scaled_offset_ad(278, A::sub_from_scalar(1.0, s.ad_value(297)), (-80.0), 5.540622384e34);
                s.store_scalar(284, 5.540622384e34);
            }
            if (!s.b[857]) {
                s.store_exp_neg_input(278, 297);
                s.copy_ad(284, 278);
            }
            s.b[858] = (s.v[279] < (-1e-8));
            s.store_scalar(858, if s.b[858] { 1.0 } else { 0.0 });
            if s.b[858] {
                s.store_sqrt_offset_ad(280, A::add(s.ad_value(278), s.ad_value(297)), (-1.0));
                s.store_mul(576, 141, 280);
                s.store_div_scaled_product3_mixed_iiai(577, 141, 120, A::sub_from_scalar(1.0, s.ad_value(284)), 1.0, 280, 2.0);
                s.store_neg(578, 577);
                s.store_scalar(313, 0.0);
                s.store_scalar(579, 0.0);
                s.store_scalar(580, 0.0);
            }
            s.b[859] = (s.v[279] > 1e-8);
            s.store_scalar(859, if s.b[859] { 1.0 } else { 0.0 });
            if ((!s.b[858]) && s.b[859]) {
                s.store_sqrt_offset_ad(280, A::add(s.ad_value(278), s.ad_value(297)), (-1.0));
                s.store_mul_neg_lhs(576, 141, 280);
                s.store_div_scaled_product3_mixed_iiai(577, 141, 120, A::sub_from_scalar(1.0, s.ad_value(284)), -1.0, 280, 2.0);
                s.store_neg(578, 577);
                s.store_exp(278, 297);
                s.store_exp_ad(281, A::mul(s.ad_value(120), A::sub(s.ad_value(584), s.ad_value(51))));
                s.store_sqrt_add_ad(282, A::div_scaled_product(s.ad_value(576), s.ad_value(576), 1.0, A::square(s.ad_value(141)), 1.0), A::mul3_scaled_output(s.ad_value(142), s.ad_value(281), A::offset(A::sub(s.ad_value(278), s.ad_value(297)), (-1.0)), 2.0));
                s.store_div_scaled_inputs_mixed_ai(537, A::add_scaled_offset_product_rhs(A::div_scaled_product(s.ad_value(576), s.ad_value(577), 2.0, A::square(s.ad_value(141)), 1.0), 1.0, A::mul3_scaled_output(s.ad_value(120), s.ad_value(142), s.ad_value(281), 2.0), s.ad_value(278), (-1.0), 1.0), 1.0, 282, 2.0);
                s.store_div_scaled_add_product(538, A::div_scaled_product(s.ad_value(576), s.ad_value(578), 2.0, A::square(s.ad_value(141)), 1.0), 1.0, A::mul3_scaled_output(s.ad_value(120), s.ad_value(142), s.ad_value(281), 2.0), s.ad_value(297), (-1.0), s.ad_value(282), 2.0);
                s.store_add_scaled_product_indices(313, 576, (-1.0), 141, 282, -1.0);
                s.store_add_scaled_product_indices(579, 577, (-1.0), 141, 537, -1.0);
                s.store_add_scaled_product_indices(580, 578, (-1.0), 141, 538, -1.0);
            }
            if ((!s.b[858]) && (!s.b[859])) {
                s.store_scaled_mul(576, 141, 297, (-1.0 / (((2.0) as f64).sqrt())));
                s.store_scaled_mul(577, 141, 120, (-1.0 / (((2.0) as f64).sqrt())));
                s.store_neg(578, 577);
                s.store_scalar(313, 0.0);
                s.store_scalar(579, 0.0);
                s.store_scalar(580, 0.0);
            }
            s.store_sub(279, 309, 584);
            s.store_mul(297, 120, 279);
            s.b[860] = ((-s.v[297]) >= 80.0);
            s.store_scalar(860, if s.b[860] { 1.0 } else { 0.0 });
            if s.b[860] {
                s.store_scaled_offset_ad(278, A::sub_from_scalar(1.0, s.ad_value(297)), (-80.0), 5.540622384e34);
                s.store_scalar(284, 5.540622384e34);
            }
            if (!s.b[860]) {
                s.store_exp_neg_input(278, 297);
                s.copy_ad(284, 278);
            }
            s.b[861] = (s.v[279] < (-1e-8));
            s.store_scalar(861, if s.b[861] { 1.0 } else { 0.0 });
            if s.b[861] {
                s.store_sqrt_offset_ad(280, A::add(s.ad_value(278), s.ad_value(297)), (-1.0));
                s.store_mul(585, 141, 280);
                s.store_div_scaled_product3_mixed_iiai(586, 141, 120, A::sub_from_scalar(1.0, s.ad_value(284)), 1.0, 280, 2.0);
                s.store_neg(587, 586);
                s.store_scalar(588, 0.0);
                s.store_scalar(589, 0.0);
                s.store_scalar(590, 0.0);
            }
            s.b[862] = (s.v[279] > 1e-8);
            s.store_scalar(862, if s.b[862] { 1.0 } else { 0.0 });
            if ((!s.b[861]) && s.b[862]) {
                s.store_sqrt_offset_ad(280, A::add(s.ad_value(278), s.ad_value(297)), (-1.0));
                s.store_mul_neg_lhs(585, 141, 280);
                s.store_div_scaled_product3_mixed_iiai(586, 141, 120, A::sub_from_scalar(1.0, s.ad_value(284)), -1.0, 280, 2.0);
                s.store_neg(587, 586);
                s.store_exp(278, 297);
                s.store_exp_ad(281, A::mul(s.ad_value(120), A::sub(s.ad_value(584), s.ad_value(51))));
                s.store_sqrt_add_ad(282, A::div_scaled_product(s.ad_value(585), s.ad_value(585), 1.0, A::square(s.ad_value(141)), 1.0), A::mul3_scaled_output(s.ad_value(142), s.ad_value(281), A::offset(A::sub(s.ad_value(278), s.ad_value(297)), (-1.0)), 2.0));
                s.store_div_scaled_inputs_mixed_ai(539, A::add_scaled_offset_product_rhs(A::div_scaled_product(s.ad_value(585), s.ad_value(586), 2.0, A::square(s.ad_value(141)), 1.0), 1.0, A::mul3_scaled_output(s.ad_value(120), s.ad_value(142), s.ad_value(281), 2.0), s.ad_value(278), (-1.0), 1.0), 1.0, 282, 2.0);
                s.store_div_scaled_add_product(538, A::div_scaled_product(s.ad_value(585), s.ad_value(587), 2.0, A::square(s.ad_value(141)), 1.0), 1.0, A::mul3_scaled_output(s.ad_value(120), s.ad_value(142), s.ad_value(281), 2.0), s.ad_value(297), (-1.0), s.ad_value(282), 2.0);
                s.store_add_scaled_product_indices(588, 585, (-1.0), 141, 282, -1.0);
                s.store_add_scaled_product_indices(589, 586, (-1.0), 141, 539, -1.0);
                s.store_add_scaled_product_indices(590, 587, (-1.0), 141, 538, -1.0);
            }
            if ((!s.b[861]) && (!s.b[862])) {
                s.store_scaled_mul(585, 141, 297, (-1.0 / (((2.0) as f64).sqrt())));
                s.store_scaled_mul(586, 141, 120, (-1.0 / (((2.0) as f64).sqrt())));
                s.store_neg(587, 586);
                s.store_scalar(588, 0.0);
                s.store_scalar(589, 0.0);
                s.store_scalar(590, 0.0);
            }
            s.b[863] = s.b[379];
            s.store_scalar(863, if s.b[863] { 1.0 } else { 0.0 });
            if s.b[863] {
                s.store_scalar(574, s.v[63]);
                s.store_scalar(63, s.v[29]);
            }
            if (!s.b[863]) {
                s.store_add_scaled_inputs3_mixed_iia(346, 308, 1.0, 76, (-1.0), A::div(A::add(A::add(A::add_scaled_inputs4(s.ad_value(314), 1.0, s.ad_value(313), 1.0, s.ad_value(576), 1.0, s.ad_value(588), 1.0), s.ad_value(585)), s.ad_value(337)), s.ad_value(270)), -1.0);
                s.store_sub_from_scalar_ad(347, 1.0, A::div_scaled_inputs2(s.ad_value(579), 1.0, s.ad_value(577), 1.0, s.ad_value(270), 1.0));
                s.store_div_scaled_inputs_mixed_ai(348, A::add_scaled_inputs4(s.ad_value(580), 1.0, s.ad_value(578), 1.0, s.ad_value(590), 1.0, s.ad_value(587), 1.0), -1.0, 270, 1.0);
                s.store_div_scaled_inputs_mixed_ai(349, A::add_scaled_product(s.ad_value(344), 1.0, A::add(s.ad_value(589), s.ad_value(586)), s.ad_value(582), 1.0), -1.0, 270, 1.0);
            }
            s.b[864] = (s.v[314] <= s.v[599]);
            s.store_scalar(864, if s.b[864] { 1.0 } else { 0.0 });
            if ((!s.b[863]) && s.b[864]) {
                s.store_sqrt_mul_ad(279, s.ad_value(296), A::add_scaled_inputs(s.ad_value(314), 2.0, s.ad_value(296), 1.0));
                s.store_div_scaled_product_indices(604, 296, 344, 1.0, 279, 1.0);
            }
            s.b[865] = (s.v[314] <= s.v[603]);
            s.store_scalar(865, if s.b[865] { 1.0 } else { 0.0 });
            if (((!s.b[863]) && (!s.b[864])) && s.b[865]) {
                s.store_mul3_ad(279, A::mul3(s.ad_value(601), A::sub(s.ad_value(314), s.ad_value(603)), A::sub(s.ad_value(314), s.ad_value(603))), A::sub(s.ad_value(314), s.ad_value(603)), A::sub(s.ad_value(314), s.ad_value(602)));
                s.store_mul_ad_product_lhs(604, A::mul3(s.ad_value(601), A::sub(s.ad_value(314), s.ad_value(603)), A::sub(s.ad_value(314), s.ad_value(603))), A::add_scaled_inputs4(s.ad_value(314), 3.0, s.ad_value(602), (-3.0), s.ad_value(314), 1.0, s.ad_value(603), (-1.0)), 344);
            }
            if (((!s.b[863]) && (!s.b[864])) && (!s.b[865])) {
                s.store_scalar(279, 0.0);
                s.store_scalar(604, 0.0);
            }
            if (!s.b[863]) {
                s.store_div_scaled_inputs_indices(281, 316, (-s.v[650]), 296, 1.0);
                s.store_div_from_scalar_offset_ad(280, 1.0, A::exp_scaled_input(s.ad_value(281), -1.0), 1.0);
                s.store_mul_square_exp_scaled_input(278, 280, 281, -1.0);
                s.store_mul(280, 280, 600);
                s.store_neg_add(279, 296, 280);
                s.store_scalar(604, 0.0);
                s.store_scaled_add(350, 576, 279, 1.0 / (s.v[535]));
                s.store_scale(351, 577, 1.0 / (s.v[535]));
                s.store_scale(352, 578, 1.0 / (s.v[535]));
                s.store_scale(353, 604, 1.0 / (s.v[535]));
                s.store_div_scaled_inputs_indices(281, 316, (-s.v[651]), 296, 1.0);
                s.store_div_from_scalar_offset_ad(280, 1.0, A::exp_scaled_input(s.ad_value(281), -1.0), 1.0);
                s.store_mul_square_exp_scaled_input(278, 280, 281, -1.0);
                s.store_mul(280, 280, 600);
                s.store_scalar(605, 0.0);
                s.store_scaled_add(354, 585, 280, 1.0 / (s.v[535]));
                s.store_scale(355, 587, 1.0 / (s.v[535]));
                s.store_add_scaled_product_indices(356, 605, 1.0 / (s.v[535]), 586, 582, 1.0 / (s.v[535]));
                s.store_add_scaled_inputs4(357, A::mul3(s.ad_value(347), s.ad_value(352), s.ad_value(356)), 1.0, A::mul3(s.ad_value(347), s.ad_value(353), s.ad_value(355)), (-1.0), A::mul3(s.ad_value(348), s.ad_value(351), s.ad_value(356)), -1.0, A::mul3(s.ad_value(349), s.ad_value(351), s.ad_value(355)), 1.0);
            }
            s.b[866] = (s.v[357] > 0.0);
            s.store_scalar(866, if s.b[866] { 1.0 } else { 0.0 });
            if ((!s.b[863]) && s.b[866]) {
                s.store_div_from_scalar_offset_input(358, 1.0, 357, 1e-50);
            }
            if ((!s.b[863]) && (!s.b[866])) {
                s.store_div_from_scalar_offset_input(358, 1.0, 357, (-1e-50));
            }
            if (!s.b[863]) {
                s.store_add_scaled_products_indices(359, 352, 356, 1.0, 353, 355, (-1.0));
                s.store_add_scaled_products_indices(360, 349, 355, 1.0, 348, 356, (-1.0));
                s.store_add_scaled_products_indices(361, 348, 353, 1.0, 349, 352, (-1.0));
                s.store_mul_neg_lhs(362, 351, 356);
                s.store_mul(363, 347, 356);
                s.store_add_scaled_products_indices(364, 349, 351, 1.0, 347, 353, (-1.0));
                s.store_mul(365, 351, 355);
                s.store_mul_neg_lhs(366, 347, 355);
                s.store_add_scaled_products_indices(367, 347, 352, 1.0, 348, 351, (-1.0));
                s.store_mul_add_scaled_products3_indices_rhs(368, 358, 359, 346, -1.0, 360, 350, -1.0, 361, 354, -1.0);
                s.store_mul_add_scaled_products3_indices_rhs(369, 358, 362, 346, -1.0, 363, 350, -1.0, 364, 354, -1.0);
                s.store_mul_add_scaled_products3_indices_rhs(370, 358, 365, 346, -1.0, 366, 350, -1.0, 367, 354, -1.0);
                s.store_abs(279, 368);
            }
            s.b[867] = (s.v[279] < ((s.v[369]) as f64).abs());
            s.store_scalar(867, if s.b[867] { 1.0 } else { 0.0 });
            if ((!s.b[863]) && s.b[867]) {
                s.store_abs(279, 369);
            }
            s.b[868] = (s.v[279] < ((s.v[370]) as f64).abs());
            s.store_scalar(868, if s.b[868] { 1.0 } else { 0.0 });
            if ((!s.b[863]) && s.b[868]) {
                s.store_abs(279, 370);
            }
            if (!s.b[863]) {
                s.store_scalar(606, 1.0);
            }
            s.b[869] = (s.v[63] > 80.0);
            s.store_scalar(869, if s.b[869] { 1.0 } else { 0.0 });
            if ((!s.b[863]) && s.b[869]) {
                s.store_scalar(606, 25.0);
            }
            s.b[870] = (s.v[63] > 40.0);
            s.store_scalar(870, if s.b[870] { 1.0 } else { 0.0 });
            if (((!s.b[863]) && (!s.b[869])) && s.b[870]) {
                s.store_scalar(606, 25.0);
            }
            s.b[871] = (s.v[63] > 20.0);
            s.store_scalar(871, if s.b[871] { 1.0 } else { 0.0 });
            if ((((!s.b[863]) && (!s.b[869])) && (!s.b[870])) && s.b[871]) {
                s.store_scalar(606, 25.0);
            }
            s.b[872] = (s.v[63] > 10.0);
            s.store_scalar(872, if s.b[872] { 1.0 } else { 0.0 });
            if (((((!s.b[863]) && (!s.b[869])) && (!s.b[870])) && (!s.b[871])) && s.b[872]) {
                s.store_scalar(606, 5.0);
            }
            s.b[873] = (s.v[279] > (0.1 / s.v[606]));
            s.store_scalar(873, if s.b[873] { 1.0 } else { 0.0 });
            if ((!s.b[863]) && s.b[873]) {
                s.store_mul_ad_rhs(368, 368, A::div_scalar_by_product(0.1, s.ad_value(606), s.ad_value(279), 1.0));
                s.store_mul_ad_rhs(369, 369, A::div_scalar_by_product(0.1, s.ad_value(606), s.ad_value(279), 1.0));
                s.store_mul_ad_rhs(370, 370, A::div_scalar_by_product(0.1, s.ad_value(606), s.ad_value(279), 1.0));
            }
            if (!s.b[863]) {
                s.store_add(308, 308, 368);
                s.store_add(584, 584, 369);
                s.store_add(310, 310, 370);
                s.store_scale(607, 606, 1e-12);
            }
            s.b[874] = (s.v[279] < s.v[607]);
            s.store_scalar(874, if s.b[874] { 1.0 } else { 0.0 });
            if ((!s.b[863]) && s.b[874]) {
                s.store_scalar(379, 1.0);
            }
            s.store_offset(63, 63, 1.0);
        }

        s.b[875] = (s.v[574] > 0.0);
        s.store_scalar(875, if s.b[875] { 1.0 } else { 0.0 });

        if s.b[875] {
            s.copy_ad(63, 574);
            s.store_scalar(574, 0.0);
        }

        s.b[876] = (s.v[63] > s.v[29]);
        s.store_scalar(876, if s.b[876] { 1.0 } else { 0.0 });

        if s.b[876] {
            s.copy_ad(308, 302);
            s.copy_ad(309, 303);
            s.copy_ad(310, 304);
            s.copy_ad(584, 581);
        }

        s.copy_ad(57, 308);

        s.store_sub(59, 57, 56);

        s.copy_ad(51, 396);

        s.b[878] = ((s.v[292] <= (-1.0)) || (s.v[305] < 0.0));
        s.store_scalar(878, if s.b[878] { 1.0 } else { 0.0 });

        if s.b[878] {
            s.store_scalar(34, 1.0);
        }

        s.copy_ad(317, 305);

        s.copy_ad(318, 308);

        s.store_sub(59, 318, 317);

        s.copy_ad(322, 306);

        s.copy_ad(323, 309);

        s.store_sub(155, 323, 322);

        s.store_add_scaled_inputs3_mixed_iia(153, 313, 1.0, 311, (-1.0), A::mul3_scaled_output(s.ad_value(120), A::add(s.ad_value(313), s.ad_value(311)), A::sub(s.ad_value(318), s.ad_value(317)), 0.5), -1.0);

        s.store_add_scaled_inputs3_mixed_iia(154, 588, 1.0, 528, (-1.0), A::mul3_scaled_output(s.ad_value(120), A::add(s.ad_value(588), s.ad_value(528)), A::sub(s.ad_value(323), s.ad_value(322)), 0.5), -1.0);

        s.b[879] = ((s.v[153] < 0.0) || (s.v[51] == 0.0));
        s.store_scalar(879, if s.b[879] { 1.0 } else { 0.0 });

        if s.b[879] {
            s.store_scalar(153, 0.0);
        }

        s.b[880] = ((s.v[154] < 0.0) || (s.v[51] == 0.0));
        s.store_scalar(880, if s.b[880] { 1.0 } else { 0.0 });

    }

    pub(super) fn stamp_reactive_block_12(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[880] {
            s.store_scalar(154, 0.0);
        }

        s.store_add(151, 153, 154);

        s.store_scaled_add(384, 576, 523, (-0.5));

        s.store_offset_sub(371, 308, 305, 1e-12);

        s.store_sub(373, 311, 313);

        s.b[881] = ((-s.v[373]) < 1e-18);
        s.store_scalar(881, if s.b[881] { 1.0 } else { 0.0 });

        if s.b[881] {
            s.store_scalar(373, 0.0);
        }

        s.store_offset_div_scaled_inputs_mixed_ia(372, 373, (-2.0), A::mul(A::mul3(s.ad_value(120), s.ad_value(270), s.ad_value(371)), s.ad_value(371)), 1.0, 1.0);

        s.store_sub_from_scalar_ad(85, 1.0, A::div_scaled_product(s.ad_value(372), s.ad_value(371), 1.0, s.ad_value(86), 1.0));

        s.b[882] = (s.v[85] <= 0.0);
        s.store_scalar(882, if s.b[882] { 1.0 } else { 0.0 });

        if s.b[882] {
            s.store_scalar(85, 0.0);
        }

        s.store_scaled_add(383, 311, 313, (-0.5));

        s.store_scaled_add(167, 528, 588, (-0.5));

        s.store_scalar(262, 0.0);

        s.b[883] = (s.v[34] == 0.0);
        s.store_scalar(883, if s.b[883] { 1.0 } else { 0.0 });

        s.b[884] = ((s.v[446] < (10.0 * 2.220446049250313e-16)) && (p.p178 < (10.0 * 2.220446049250313e-16)));
        s.store_scalar(884, if s.b[884] { 1.0 } else { 0.0 });

        if (s.b[883] && s.b[884]) {
            s.store_scalar(262, 0.0);
            s.copy_ad(260, 57);
        }

        s.b[885] = (s.v[260] > ((s.v[56] + s.v[71]) - (10.0 * 2.220446049250313e-16)));
        s.store_scalar(885, if s.b[885] { 1.0 } else { 0.0 });

        if ((s.b[883] && s.b[884]) && s.b[885]) {
            s.store_offset_add(260, 56, 71, (-(10.0 * 2.220446049250313e-16)));
        }

        if (s.b[883] && (!s.b[884])) {
            s.store_scalar(263, p.p227);
            s.store_div_from_scalar_ad(282, 1.034943e-10, A::add_scaled_product(A::div_scaled_inputs(s.ad_value(149), p.p178, s.ad_value(263), 1.0), 1.0, s.ad_value(446), s.ad_value(126), 1.0));
            s.store_add_scaled_inputs3_indices(260, 51, p.p176, 56, p.p176, 57, (1.0 - p.p176));
        }

        s.b[886] = (s.v[260] > ((s.v[56] + s.v[71]) - (10.0 * 2.220446049250313e-16)));
        s.store_scalar(886, if s.b[886] { 1.0 } else { 0.0 });

        if ((s.b[883] && (!s.b[884])) && s.b[886]) {
            s.store_offset_add(260, 56, 71, (-(10.0 * 2.220446049250313e-16)));
        }

        if (s.b[883] && (!s.b[884])) {
            s.store_sub(284, 260, 57);
            s.store_sqrt_square_offset(639, 284, ((4.0 * 0.001) * 0.001));
            s.store_offset_scaled_div(278, 284, 639, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(284, 284, 0.5, 639, 0.5, (1e-10 * 0.001));
        }

        s.b[887] = (s.v[284] < 0.0);
        s.store_scalar(887, if s.b[887] { 1.0 } else { 0.0 });

        if ((s.b[883] && (!s.b[884])) && s.b[887]) {
            s.store_scalar(284, 0.0);
            s.store_scalar(278, 0.0);
        }

        if (s.b[883] && (!s.b[884])) {
            s.store_div_ad_rhs(283, 151, A::mul(s.ad_value(120), s.ad_value(149)));
            s.store_scale(288, 126, 9662367879.197212);
            s.store_scalar(279, 1000000000.0);
            s.store_div_scaled_inputs_product(387, s.ad_value(283), 2.0, A::mul3_scaled_output(s.ad_value(288), s.ad_value(284), s.ad_value(282), 2.0), 1.0, s.ad_value(279), s.ad_value(282), 1.0, s.ad_value(123), 1.0);
            s.store_mul(285, 387, 282);
            s.store_add_scaled_product_indices(387, 279, 4.0, 288, 284, (2.0 * 4.0));
            s.store_mul3_lhs(286, 387, 282, 282);
            s.store_sqrt_square_add(287, 285, 286);
            s.store_scaled_sub(262, 287, 285, 0.5);
            s.copy_ad(279, 262);
            s.store_mul(262, 276, 279);
        }

        if s.b[883] {
            s.store_scale(262, 262, s.v[483]);
        }

        s.store_sub(386, 123, 262);

        s.b[888] = (s.v[386] < 1e-9);
        s.store_scalar(888, if s.b[888] { 1.0 } else { 0.0 });

        if s.b[888] {
            s.store_scalar(386, 1e-9);
        }

        s.store_mul_add_scaled_inputs_rhs(91, 123, s.ad_value(383), (-s.v[513]), s.ad_value(167), (-s.v[513]));

        s.store_mul_scaled_ad_lhs(336, A::add(s.ad_value(312), s.ad_value(314)), 123, (0.5 * s.v[513]));

        s.store_scaled_sub(279, 51, 59, 0.5);

        s.store_scale(638, 279, (2.0 * 1.0 / (p.p217)));

        s.store_offset_mul_offset_rhs_ad_rhs(639, 638, A::mul_offset_rhs(s.ad_value(638), A::mul_offset_rhs(s.ad_value(638), A::mul_offset_rhs(s.ad_value(638), A::mul(s.ad_value(638), A::scale_offset(s.ad_value(638), (1.0 / 5040.0), (1.0 / 720.0))), (1.0 / 120.0)), (1.0 / 24.0)), (1.0 / 6.0)), (1.0 / 2.0), 1.0);

        s.store_offset_mul_offset_rhs_ad_rhs(640, 638, A::mul_offset_rhs(s.ad_value(638), A::mul_offset_rhs(s.ad_value(638), A::mul(s.ad_value(638), A::scale_offset(s.ad_value(638), (1.0 / 840.0), (1.0 / 144.0))), (1.0 / 30.0)), (1.0 / 8.0)), (1.0 / 3.0), (1.0 / 2.0));

        s.store_div_from_scalar(75, p.p217, 639);

        s.store_div_scaled_inputs_square_rhs(280, 640, (-2.0), 639, 1.0);

        s.b[889] = (s.v[75] < (10.0 * 2.220446049250313e-16));
        s.store_scalar(889, if s.b[889] { 1.0 } else { 0.0 });

        if s.b[889] {
            s.store_scalar(75, (10.0 * 2.220446049250313e-16));
        }

        s.store_add(74, 56, 75);

        s.store_scalar(499, (1.034943e-10 / 100.0));

        s.store_scale(500, 313, 0.0001);

        s.store_scale(501, 588, 0.0001);

        s.store_scale(504, 531, 0.0001);

        s.store_scale(505, 585, 0.0001);

        s.store_scale(502, 383, 0.0001);

        s.store_scale(503, 167, 0.0001);

        s.store_scale(504, 531, 0.0001);

        s.store_scale(505, 585, 0.0001);

        s.store_scale(506, 384, 0.0001);

        s.store_scalar(507, (p.p229 * 100.0));

        s.store_scalar(591, ((p.p81 * (1.0 + (p.p82 / ((s.v[375]) as f64).powf(p.p83)))) / s.v[499]));

        s.store_scalar(592, ((p.p78 * (1.0 + (p.p79 / ((s.v[375]) as f64).powf(p.p80)))) / s.v[499]));

        s.store_sqrt_square_offset(639, 59, ((4.0 * 1e-6) * 1e-6));

        s.store_offset_scaled_div(278, 59, 639, 0.5, 0.5);

        s.store_offset_add_scaled_inputs_indices(598, 59, 0.5, 639, 0.5, (1e-10 * 1e-6));

        s.b[890] = (s.v[598] < 0.0);
        s.store_scalar(890, if s.b[890] { 1.0 } else { 0.0 });

        if s.b[890] {
            s.store_scalar(598, 0.0);
            s.store_scalar(278, 0.0);
        }

        s.store_offset_sqrt_ad(168, A::offset(A::square(s.ad_value(598)), p.p216), (-((p.p216) as f64).sqrt()));

        s.store_powf(168, 168, p.p85);

        s.store_offset_scaled(282, 168, p.p84, 1.0);

        s.store_scalar(497, (p.p299 * (1.0 + (p.p300 / ((s.v[375]) as f64).powf(p.p301)))));

        s.store_sub_scaled_inputs(288, 502, 1.0, 501, s.v[497]);

        s.store_add_scaled_inputs(283, 506, s.v[592], 288, s.v[591]);

        s.store_div(156, 283, 282);

        if (p.p32 != 0.0) {
            s.store_scaled_add(596, 306, 309, 0.5);
            s.store_scaled_add(597, 307, 310, 0.5);
            s.store_add_scaled_inputs3_indices(163, 596, (3.9 * 1.0 / ((11.7 * s.v[507]))), 597, ((-1.0) * (3.9 * 1.0 / ((11.7 * s.v[507])))), 440, (-(3.9 * 1.0 / ((11.7 * s.v[507])))));
            s.store_add(156, 156, 163);
        }

        if (p.p32 == 0.0) {
            s.store_scalar(596, 0.0);
            s.store_scalar(597, 0.0);
            s.store_scalar(163, 0.0);
        }

        s.store_sqrt_square_offset(639, 156, ((4.0 * 3000.0) * 3000.0));

        s.store_offset_scaled_div(279, 156, 639, 0.5, 0.5);

        s.store_offset_add_scaled_inputs_indices(156, 156, 0.5, 639, 0.5, (1e-10 * 3000.0));

        s.b[891] = (s.v[156] < 0.0);
        s.store_scalar(891, if s.b[891] { 1.0 } else { 0.0 });

        if s.b[891] {
            s.store_scalar(156, 0.0);
            s.store_scalar(279, 0.0);
        }

        s.store_powf(286, 156, p.p94);

        s.store_powf(284, 156, s.v[470]);

        s.store_scale(157, 502, 6.241449993689894e18);

        s.store_add_scaled_ad_lhs(279, A::add_scaled_product(A::div_from_scalar(1.0, A::scale_offset(s.ad_value(157), (s.v[449] * 1e-11), s.v[448])), 1.0, s.ad_value(469), s.ad_value(286), 1.0), 284, 1.0 / (p.p105));

        s.store_div_from_scalar(159, 1.0, 279);

        s.store_scale(159, 159, 0.0001);

        if (p.p32 != 0.0) {
            s.store_scaled_sub(163, 596, 597, (3.9 * 1.0 / ((11.7 * s.v[507]))));
        }

        if (p.p32 == 0.0) {
            s.store_sqrt_square_offset(639, 155, ((4.0 * 1e-6) * 1e-6));
            s.store_offset_scaled_div(278, 155, 639, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(598, 155, 0.5, 639, 0.5, (1e-10 * 1e-6));
        }

        s.b[892] = (s.v[598] < 0.0);
        s.store_scalar(892, if s.b[892] { 1.0 } else { 0.0 });

        if ((p.p32 == 0.0) && s.b[892]) {
            s.store_scalar(598, 0.0);
            s.store_scalar(278, 0.0);
        }

        if (p.p32 == 0.0) {
            s.store_offset_sqrt_ad(168, A::offset(A::square(s.ad_value(598)), p.p216), (-((p.p216) as f64).sqrt()));
            s.store_powf(168, 168, p.p85);
            s.store_offset_scaled(282, 168, p.p84, 1.0);
            s.store_scalar(498, (p.p302 * (1.0 + (p.p300 / ((s.v[375]) as f64).powf(p.p301)))));
            s.store_add_scaled_product_indices(288, 503, 1.0, 498, 500, (-1.0));
            s.store_scaled_add(508, 505, 504, (-0.5));
            s.store_add_scaled_inputs(283, 508, s.v[592], 288, s.v[591]);
            s.store_div(163, 283, 282);
        }

        s.store_sqrt_square_offset(639, 163, ((4.0 * 30.0) * 30.0));

        s.store_offset_scaled_div(279, 163, 639, 0.5, 0.5);

        s.store_offset_add_scaled_inputs_indices(163, 163, 0.5, 639, 0.5, (1e-10 * 30.0));

        s.b[893] = (s.v[163] < 0.0);
        s.store_scalar(893, if s.b[893] { 1.0 } else { 0.0 });

        if s.b[893] {
            s.store_scalar(163, 0.0);
            s.store_scalar(279, 0.0);
        }

        s.store_powf(286, 163, p.p275);

        s.store_powf(284, 163, s.v[594]);

        s.store_scale(157, 503, 6.241449993689894e18);

        s.store_add_scaled_ad_lhs(279, A::add_scaled_product(A::div_from_scalar(1.0, A::scale_offset(s.ad_value(157), (s.v[451] * 1e-11), s.v[450])), 1.0, s.ad_value(595), s.ad_value(286), 1.0), 284, 1.0 / (p.p284));

        s.store_div_from_scalar(166, 1.0, 279);

        s.store_scale(166, 166, 0.0001);

        s.store_div_scaled_inputs_indices(454, 162, 0.2, 159, 1.0);

        s.store_div_ad_rhs(291, 153, A::mul3(s.ad_value(120), A::offset(s.ad_value(149), 1e-50), s.ad_value(386)));

        s.store_sqrt_square_sum(160, 291, 454);

        s.store_mul(161, 159, 160);

        s.store_div(279, 161, 162);

        s.b[894] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= p.p114) && (p.p114 <= (1.0 + (10.0 * 2.220446049250313e-16))));
        s.store_scalar(894, if s.b[894] { 1.0 } else { 0.0 });

        if s.b[894] {
            s.store_scalar(281, 1.0);
        }

        s.b[895] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= p.p114) && (p.p114 <= (2.0 + (10.0 * 2.220446049250313e-16))));
        s.store_scalar(895, if s.b[895] { 1.0 } else { 0.0 });

        if ((!s.b[894]) && s.b[895]) {
            s.copy_ad(281, 279);
        }

        if ((!s.b[894]) && (!s.b[895])) {
            s.store_powf(281, 279, (p.p114 - 1.0));
        }

        s.store_offset_mul(282, 279, 281, 1.0);

        s.b[896] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= p.p114) && (p.p114 <= (1.0 + (10.0 * 2.220446049250313e-16))));
        s.store_scalar(896, if s.b[896] { 1.0 } else { 0.0 });

        if s.b[896] {
            s.store_div_from_scalar(283, 1.0, 282);
        }

        s.b[897] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= p.p114) && (p.p114 <= (2.0 + (10.0 * 2.220446049250313e-16))));
        s.store_scalar(897, if s.b[897] { 1.0 } else { 0.0 });

        if ((!s.b[896]) && s.b[897]) {
            s.store_div_from_scalar_sqrt_ad(283, 1.0, s.ad_value(282));
        }

        if ((!s.b[896]) && (!s.b[897])) {
            s.store_powf(284, 282, (((-1.0) / p.p114) - 1.0));
            s.store_mul(283, 282, 284);
        }

        s.store_mul(158, 159, 283);

        s.store_div_scaled_inputs_indices(455, 162, 0.2, 166, 1.0);

        s.store_div_ad_rhs(291, 154, A::mul3(s.ad_value(120), A::offset(s.ad_value(150), 1e-50), s.ad_value(386)));

        s.store_sqrt_square_sum(164, 291, 455);

        s.store_mul(161, 166, 164);

        s.store_div(279, 161, 162);

        s.b[898] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= p.p114) && (p.p114 <= (1.0 + (10.0 * 2.220446049250313e-16))));
        s.store_scalar(898, if s.b[898] { 1.0 } else { 0.0 });

        if s.b[898] {
            s.store_scalar(281, 1.0);
        }

        s.b[899] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= p.p114) && (p.p114 <= (2.0 + (10.0 * 2.220446049250313e-16))));
        s.store_scalar(899, if s.b[899] { 1.0 } else { 0.0 });

        if ((!s.b[898]) && s.b[899]) {
            s.copy_ad(281, 279);
        }

        if ((!s.b[898]) && (!s.b[899])) {
            s.store_powf(281, 279, (p.p114 - 1.0));
        }

        s.store_offset_mul(282, 279, 281, 1.0);

        s.b[900] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= p.p114) && (p.p114 <= (1.0 + (10.0 * 2.220446049250313e-16))));
        s.store_scalar(900, if s.b[900] { 1.0 } else { 0.0 });

        if s.b[900] {
            s.store_div_from_scalar(283, 1.0, 282);
        }

        s.b[901] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= p.p114) && (p.p114 <= (2.0 + (10.0 * 2.220446049250313e-16))));
        s.store_scalar(901, if s.b[901] { 1.0 } else { 0.0 });

        if ((!s.b[900]) && s.b[901]) {
            s.store_div_from_scalar_sqrt_ad(283, 1.0, s.ad_value(282));
        }

        if ((!s.b[900]) && (!s.b[901])) {
            s.store_powf(284, 282, (((-1.0) / p.p114) - 1.0));
            s.store_mul(283, 282, 284);
        }

        s.store_mul(165, 166, 283);

        s.store_div_scaled_inputs_mixed_ia(189, 122, s.v[466], A::sub(s.ad_value(123), s.ad_value(262)), 1.0);

        s.store_mul3_lhs(96, 189, 153, 158);

        s.store_mul3_lhs(97, 189, 154, 165);

        s.store_add(95, 96, 97);

        s.store_scalar(173, 0.0);

        s.store_scalar(169, 0.0);

        s.store_scalar(171, 0.0);

        s.store_scalar(172, 0.0);

    }

    pub(super) fn stamp_reactive_block_13(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[902] = (p.p239 != 0.0);
        s.store_scalar(902, if s.b[902] { 1.0 } else { 0.0 });

        if s.b[902] {
            s.store_scaled_sub(279, 51, 59, 0.5);
            s.store_scale(638, 279, (2.0 * 100.0));
            s.store_offset_mul_offset_rhs_ad_rhs(639, 638, A::mul_offset_rhs(s.ad_value(638), A::mul_offset_rhs(s.ad_value(638), A::mul_offset_rhs(s.ad_value(638), A::mul(s.ad_value(638), A::scale_offset(s.ad_value(638), (1.0 / 5040.0), (1.0 / 720.0))), (1.0 / 120.0)), (1.0 / 24.0)), (1.0 / 6.0)), (1.0 / 2.0), 1.0);
            s.store_offset_mul_offset_rhs_ad_rhs(640, 638, A::mul_offset_rhs(s.ad_value(638), A::mul_offset_rhs(s.ad_value(638), A::mul(s.ad_value(638), A::scale_offset(s.ad_value(638), (1.0 / 840.0), (1.0 / 144.0))), (1.0 / 30.0)), (1.0 / 8.0)), (1.0 / 3.0), (1.0 / 2.0));
            s.store_div_from_scalar(284, 0.01, 639);
            s.store_div_scaled_inputs_square_rhs(280, 640, (-2.0), 639, 1.0);
            s.store_sub_from_scalar_ad(279, 1.1, A::add(s.ad_value(56), s.ad_value(284)));
            s.store_sqrt_square_offset(639, 279, ((4.0 * 0.05) * 0.05));
            s.store_offset_scaled_div(278, 279, 639, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(280, 279, 0.5, 639, 0.5, (1e-10 * 0.05));
        }

        s.b[903] = (s.v[280] < 0.0);
        s.store_scalar(903, if s.b[903] { 1.0 } else { 0.0 });

        if (s.b[902] && s.b[903]) {
            s.store_scalar(280, 0.0);
            s.store_scalar(278, 0.0);
        }

        if s.b[902] {
            s.store_mul_ad_affine_product_rhs(287, 270, s.ad_value(120), A::powf(s.ad_value(280), p.p240), s.v[475], 0.0);
            s.store_add_scaled_product_mixed_aia(282, A::scale_offset(s.ad_value(71), p.p241, 1.0), 1.0, 71, A::add_scaled_inputs3(s.ad_value(56), 1.0, s.ad_value(284), 1.0, s.ad_value(70), -1.0), s.v[476]);
            s.store_mul(287, 287, 282);
        }

        if (!s.b[902]) {
            s.store_scalar(287, 0.0);
        }

        s.b[904] = (p.p246 != 0.0);
        s.store_scalar(904, if s.b[904] { 1.0 } else { 0.0 });

        if s.b[904] {
            s.store_mul3_affine_lhs(286, 270, 120, s.v[477], 0.0, 71);
        }

        if (!s.b[904]) {
            s.store_scalar(286, 0.0);
        }

        s.b[905] = ((s.v[287] + s.v[286]) > 0.0);
        s.store_scalar(905, if s.b[905] { 1.0 } else { 0.0 });

        if s.b[905] {
            s.store_mul_add_rhs(152, 59, 287, 286);
            s.store_mul3_lhs(173, 189, 152, 158);
            s.store_div_from_scalar_offset_ad(172, 1.0, A::exp_scaled_input(s.ad_value(440), (-p.p245)), 1.0);
            s.store_sub_from_scalar(171, 1.0, 172);
            s.store_mul(169, 171, 173);
        }

        s.store_scalar(174, 0.0);

        s.store_scalar(170, 0.0);

        s.b[906] = (p.p239 != 0.0);
        s.store_scalar(906, if s.b[906] { 1.0 } else { 0.0 });

        if s.b[906] {
            s.store_scaled_sub(279, 51, 155, 0.5);
            s.store_scale(638, 279, (2.0 * 100.0));
            s.store_offset_mul_offset_rhs_ad_rhs(639, 638, A::mul_offset_rhs(s.ad_value(638), A::mul_offset_rhs(s.ad_value(638), A::mul_offset_rhs(s.ad_value(638), A::mul(s.ad_value(638), A::scale_offset(s.ad_value(638), (1.0 / 5040.0), (1.0 / 720.0))), (1.0 / 120.0)), (1.0 / 24.0)), (1.0 / 6.0)), (1.0 / 2.0), 1.0);
            s.store_offset_mul_offset_rhs_ad_rhs(640, 638, A::mul_offset_rhs(s.ad_value(638), A::mul_offset_rhs(s.ad_value(638), A::mul(s.ad_value(638), A::scale_offset(s.ad_value(638), (1.0 / 840.0), (1.0 / 144.0))), (1.0 / 30.0)), (1.0 / 8.0)), (1.0 / 3.0), (1.0 / 2.0));
            s.store_div_from_scalar(284, 0.01, 639);
            s.store_div_scaled_inputs_square_rhs(280, 640, (-2.0), 639, 1.0);
            s.store_sub_from_scalar_ad(279, 1.1, A::add(s.ad_value(322), s.ad_value(284)));
            s.store_sqrt_square_offset(639, 279, ((4.0 * 0.05) * 0.05));
            s.store_offset_scaled_div(278, 279, 639, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(280, 279, 0.5, 639, 0.5, (1e-10 * 0.05));
        }

        s.b[907] = (s.v[280] < 0.0);
        s.store_scalar(907, if s.b[907] { 1.0 } else { 0.0 });

        if (s.b[906] && s.b[907]) {
            s.store_scalar(280, 0.0);
            s.store_scalar(278, 0.0);
        }

        if s.b[906] {
            s.store_mul_ad_affine_product_rhs(287, 270, s.ad_value(120), A::powf(s.ad_value(280), p.p240), s.v[475], 0.0);
            s.store_add_scaled_product_mixed_aia(282, A::scale_offset(s.ad_value(71), p.p241, 1.0), 1.0, 71, A::add_scaled_inputs3(s.ad_value(322), 1.0, s.ad_value(284), 1.0, s.ad_value(70), -1.0), s.v[476]);
            s.store_mul(287, 287, 282);
        }

        if (!s.b[906]) {
            s.store_scalar(287, 0.0);
        }

        s.b[908] = ((s.v[287] + s.v[286]) > 0.0);
        s.store_scalar(908, if s.b[908] { 1.0 } else { 0.0 });

        if s.b[908] {
            s.store_mul_add_rhs(152, 155, 287, 286);
            s.store_mul3_lhs(174, 189, 152, 165);
        }

        s.b[909] = ((s.v[174] > (s.v[173] - (s.v[173] * 0.05))) && ((s.v[173] * 0.05) >= 0.0));
        s.store_scalar(909, if s.b[909] { 1.0 } else { 0.0 });

        if (s.b[908] && s.b[909]) {
            s.store_add_scaled_inputs3_indices(638, 174, 1.0, 173, (-1.0), 173, 0.05);
            s.store_square(642, 638);
            s.store_scaled_mul(643, 173, 173, (0.05 * 0.05));
            s.store_scalar(644, 1.0);
            s.store_scalar(645, 1.0);
            s.store_scalar(647, 0.0);
            s.store_scalar(648, 0.0);
            s.store_scalar(220, 0.0);
            s.store_scalar(646, 0.0);
            s.store_mul(644, 644, 642);
            s.store_mul(645, 645, 643);
            s.store_mul(644, 644, 642);
            s.store_mul(645, 645, 643);
            s.store_add(220, 644, 645);
            s.copy_ad(646, 220);
        }

        s.b[910] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.store_scalar(910, if s.b[910] { 1.0 } else { 0.0 });

        s.b[911] = (2.0 == 1.0);
        s.store_scalar(911, if s.b[911] { 1.0 } else { 0.0 });

        if (((s.b[908] && s.b[909]) && s.b[910]) && s.b[911]) {
            s.store_scalar(648, 1.0);
        }

        s.b[912] = (2.0 == 2.0);
        s.store_scalar(912, if s.b[912] { 1.0 } else { 0.0 });

        if ((((s.b[908] && s.b[909]) && s.b[910]) && (!s.b[911])) && s.b[912]) {
            s.store_scalar(648, 2.0);
        }

        s.b[913] = (2.0 == 4.0);
        s.store_scalar(913, if s.b[913] { 1.0 } else { 0.0 });

        if (((((s.b[908] && s.b[909]) && s.b[910]) && (!s.b[911])) && (!s.b[912])) && s.b[913]) {
            s.store_scalar(648, 3.0);
        }

        s.b[914] = (2.0 == 8.0);
        s.store_scalar(914, if s.b[914] { 1.0 } else { 0.0 });

        if ((((((s.b[908] && s.b[909]) && s.b[910]) && (!s.b[911])) && (!s.b[912])) && (!s.b[913])) && s.b[914]) {
            s.store_scalar(648, 4.0);
        }

        if ((s.b[908] && s.b[909]) && s.b[910]) {
            s.store_scalar(647, 0.0);
        }

        let mut assign14450_loop_guard: usize = 0;
        while {
            let assign14450_cond_e18791: f64 = if (((s.b[908] && s.b[909]) && s.b[910]) && (s.v[647] < s.v[648])) { 1.0 } else { 0.0 };
            assign14450_cond_e18791 != 0.0
        } {
            assign14450_loop_guard += 1;
            assert!(assign14450_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.b[908] && s.b[909]) && s.b[910]) {
                s.store_sqrt(646, 646);
                s.store_offset(647, 647, 1.0);
            }
        }

        if ((s.b[908] && s.b[909]) && (!s.b[910])) {
            s.store_powf(646, 646, (1.0 / (2.0 * 2.0)));
        }

        if (s.b[908] && s.b[909]) {
            s.store_div_from_scalar_offset_input(646, 1.0, 646, 1e-50);
            s.store_mul3_affine_lhs(637, 638, 173, 0.05, 0.0, 646);
            s.store_div_scaled_product3_mixed_iiia(278, 173, 645, 646, 0.05, A::offset(s.ad_value(220), 1e-50), 1.0);
            s.store_add_scaled_inputs3_indices(174, 173, 1.0, 173, (-0.05), 637, 1.0);
        }

        if (s.b[908] && s.b[909]) {
        }

        if (s.b[908] && (!s.b[909])) {
        }

        if (s.b[908] && (!s.b[909])) {
            s.store_scalar(278, 1.0);
        }

        if s.b[908] {
            s.store_mul(170, 172, 174);
        }

        s.store_add(175, 169, 170);

        s.store_add(94, 95, 175);

        s.b[915] = (p.p22 != 0.0);
        s.store_scalar(915, if s.b[915] { 1.0 } else { 0.0 });

        if s.b[915] {
            s.store_scale(279, 271, 1.034943e-10);
            s.copy_ad(280, 132);
            s.store_scalar(281, (s.v[133] - p.p57));
            s.store_div_from_scalar_square_ad(282, 1.0, s.ad_value(281));
            s.store_mul_ad_product_lhs_mixed_ai(283, A::mul_sub_from_scalar_lhs_scaled_output(p.p55, s.ad_value(130), s.ad_value(279), 2.0), 280, 282);
            s.store_mul(81, 283, 135);
            s.store_scalar(282, p.p158);
            s.store_scalar(284, p.p159);
            s.store_add_scaled_product_indices(279, 282, 1.0, 284, 71, 1.0);
            s.store_mul(98, 81, 279);
            s.store_sub_from_scalar_scaled_input(279, p.p160, 51, p.p161);
            s.store_add_scaled_inputs4_indices(99, 72, 1.0, 138, (-1.0), 279, 1.0, 98, 1.0);
            s.store_mul3_lhs(102, 119, 271, 271);
            s.store_scaled_mul(103, 102, 120, 0.5);
            s.store_scaled_mul(104, 103, 120, 2.0);
            s.store_scale(387, 120, 0.25);
            s.store_offset_add_scaled_inputs3_offset_mixed_aii(288, A::add_scaled_product(s.ad_value(122), 1.0, s.ad_value(102), s.ad_value(387), (-1.0)), 1.0, 138, 1.0, 98, -1.0, (-p.p160), 1e-50);
            s.store_offset_sub(279, 72, 288, (-0.005));
        }

        if s.b[915] {
            s.store_scalar(278, (if (s.v[288] >= 0.0) { 1.0 } else { (-1.0) }));
        }

        if s.b[915] {
            s.store_sqrt_add_scaled_square_product(280, 279, 1.0, 278, 288, (4.0 * 0.005));
            s.store_add_scaled_inputs3_mixed_aii(281, A::offset(A::add_scaled_inputs4(s.ad_value(288), 1.0, s.ad_value(279), 0.5, s.ad_value(280), 0.5, s.ad_value(138), -1.0), p.p160), 1.0, 98, 1.0, 70, -1.0);
            s.store_offset_mul(282, 120, 281, (-1.0));
            s.store_div_from_scalar(283, 4.0, 104);
            s.store_offset_mul(279, 282, 283, 1.0);
            s.store_sqrt_square_offset(639, 279, ((4.0 * 0.01) * 0.01));
            s.store_offset_scaled_div(280, 279, 639, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(279, 279, 0.5, 639, 0.5, (1e-10 * 0.01));
        }

        s.b[916] = (s.v[279] < 0.0);
        s.store_scalar(916, if s.b[916] { 1.0 } else { 0.0 });

        if (s.b[915] && s.b[916]) {
            s.store_scalar(279, 0.0);
            s.store_scalar(280, 0.0);
        }

        if s.b[915] {
            s.store_offset(279, 279, 1e-50);
            s.store_sqrt(105, 279);
            s.store_mul_sub_from_scalar_rhs(278, 103, 1.0, 105);
            s.store_add(107, 99, 278);
            s.store_div_from_scalar_add_ad(278, 1.0, s.ad_value(120), A::div_scalar_offset_denominator(2.0, s.ad_value(99), 1e-50, 1.0));
            s.store_mul_ln_ad_lhs(109, A::mul(A::div_scalar_by_product(1.0, s.ad_value(101), s.ad_value(102), 1.0), A::square(s.ad_value(99))), 278);
            s.store_div_scaled_value_offset_denominator(281, s.ad_value(109), 1.0, s.ad_value(99), 1e-50, 1.0);
            s.store_offset_sub(110, 109, 107, (-p.p136));
            s.store_add_scaled_ad_lhs(278, A::square(s.ad_value(110)), 109, (4.0 * p.p136));
            s.store_sqrt_square_offset(639, 278, ((4.0 * 1e-6) * 1e-6));
            s.store_offset_scaled_div(280, 278, 639, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(278, 278, 0.5, 639, 0.5, (1e-10 * 1e-6));
        }

    }

    pub(super) fn stamp_reactive_block_14(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[917] = (s.v[278] < 0.0);
        s.store_scalar(917, if s.b[917] { 1.0 } else { 0.0 });

        if (s.b[915] && s.b[917]) {
            s.store_scalar(278, 0.0);
            s.store_scalar(280, 0.0);
        }

        if s.b[915] {
            s.store_sqrt(278, 278);
            s.store_add_scaled_inputs3_indices(111, 109, 1.0, 110, (-0.5), 278, (-0.5));
            s.store_div_from_scalar(279, 1.0, 278);
            s.store_mul_exp_ad_rhs(278, 101, A::mul(s.ad_value(120), s.ad_value(111)));
            s.store_add_offset_ad_lhs(279, A::mul(s.ad_value(120), A::sub(s.ad_value(111), s.ad_value(70))), (-1.0), 278);
            s.store_sqrt_square_offset(639, 279, ((4.0 * 0.01) * 0.01));
            s.store_offset_scaled_div(278, 279, 639, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(279, 279, 0.5, 639, 0.5, (1e-10 * 0.01));
        }

        s.b[918] = (s.v[279] < 0.0);
        s.store_scalar(918, if s.b[918] { 1.0 } else { 0.0 });

        if (s.b[915] && s.b[918]) {
            s.store_scalar(279, 0.0);
            s.store_scalar(278, 0.0);
        }

        if s.b[915] {
            s.store_offset(279, 279, (10.0 * 2.220446049250313e-16));
            s.store_sqrt(113, 279);
            s.store_offset_mul_ad(279, s.ad_value(120), A::sub(s.ad_value(111), s.ad_value(70)), (-1.0));
            s.store_sqrt_square_offset(639, 279, ((4.0 * 0.01) * 0.01));
            s.store_offset_scaled_div(278, 279, 639, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(279, 279, 0.5, 639, 0.5, (1e-10 * 0.01));
        }

        s.b[919] = (s.v[279] < 0.0);
        s.store_scalar(919, if s.b[919] { 1.0 } else { 0.0 });

        if (s.b[915] && s.b[919]) {
            s.store_scalar(279, 0.0);
            s.store_scalar(278, 0.0);
        }

        if s.b[915] {
            s.store_offset(279, 279, (10.0 * 2.220446049250313e-16));
            s.store_sqrt(114, 279);
            s.store_mul_sub_rhs(115, 100, 113, 114);
            s.store_sub(279, 107, 111);
            s.store_sqrt_square_offset(639, 279, ((4.0 * 0.1) * 0.1));
            s.store_offset_scaled_div(280, 279, 639, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(279, 279, 0.5, 639, 0.5, (1e-10 * 0.1));
        }

        s.b[920] = (s.v[279] < 0.0);
        s.store_scalar(920, if s.b[920] { 1.0 } else { 0.0 });

        if (s.b[915] && s.b[920]) {
            s.store_scalar(279, 0.0);
            s.store_scalar(280, 0.0);
        }

        if s.b[915] {
            s.store_offset(279, 279, (10.0 * 2.220446049250313e-16));
            s.store_div(290, 51, 279);
            s.store_square(642, 290);
            s.store_scalar(643, 1.0);
            s.store_scalar(644, 1.0);
            s.store_scalar(645, 1.0);
            s.store_scalar(647, 0.0);
            s.store_scalar(648, 0.0);
            s.store_scalar(220, 0.0);
            s.store_scalar(646, 0.0);
            s.store_mul(644, 644, 642);
            s.store_mul(645, 645, 643);
            s.store_mul(644, 644, 642);
            s.store_mul(645, 645, 643);
            s.store_mul(644, 644, 642);
            s.store_mul(645, 645, 643);
            s.store_mul(644, 644, 642);
            s.store_mul(645, 645, 643);
            s.store_add(220, 644, 645);
            s.copy_ad(646, 220);
        }

        s.b[921] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));
        s.store_scalar(921, if s.b[921] { 1.0 } else { 0.0 });

        s.b[922] = (4.0 == 1.0);
        s.store_scalar(922, if s.b[922] { 1.0 } else { 0.0 });

        if ((s.b[915] && s.b[921]) && s.b[922]) {
            s.store_scalar(648, 1.0);
        }

        s.b[923] = (4.0 == 2.0);
        s.store_scalar(923, if s.b[923] { 1.0 } else { 0.0 });

        if (((s.b[915] && s.b[921]) && (!s.b[922])) && s.b[923]) {
            s.store_scalar(648, 2.0);
        }

        s.b[924] = (4.0 == 4.0);
        s.store_scalar(924, if s.b[924] { 1.0 } else { 0.0 });

        if ((((s.b[915] && s.b[921]) && (!s.b[922])) && (!s.b[923])) && s.b[924]) {
            s.store_scalar(648, 3.0);
        }

        s.b[925] = (4.0 == 8.0);
        s.store_scalar(925, if s.b[925] { 1.0 } else { 0.0 });

        if (((((s.b[915] && s.b[921]) && (!s.b[922])) && (!s.b[923])) && (!s.b[924])) && s.b[925]) {
            s.store_scalar(648, 4.0);
        }

        if (s.b[915] && s.b[921]) {
            s.store_scalar(647, 0.0);
        }

        let mut assign15630_loop_guard: usize = 0;
        while {
            let assign15630_cond_e19733: f64 = if ((s.b[915] && s.b[921]) && (s.v[647] < s.v[648])) { 1.0 } else { 0.0 };
            assign15630_cond_e19733 != 0.0
        } {
            assign15630_loop_guard += 1;
            assert!(assign15630_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (s.b[915] && s.b[921]) {
                s.store_sqrt(646, 646);
                s.store_offset(647, 647, 1.0);
            }
        }

        if (s.b[915] && (!s.b[921])) {
            s.store_powf(646, 646, (1.0 / (2.0 * 4.0)));
        }

        if s.b[915] {
            s.store_div_from_scalar_offset_input(646, 1.0, 646, 1e-50);
            s.store_scaled_mul(291, 290, 646, 1.0);
            s.store_div_scaled_product_offset_denominator(280, s.ad_value(645), s.ad_value(646), 1.0, s.ad_value(220), 1e-50, 1.0);
            s.store_scale(106, 122, ((2.0 * s.v[453]) * p.p5));
            s.copy_ad(279, 386);
            s.store_div_scaled_product_left_ad(116, A::mul3(s.ad_value(106), s.ad_value(158), s.ad_value(115)), 291, 1.0, 279, 1.0);
            s.store_add(94, 94, 116);
        }

        s.b[926] = ((p.p20 != 0.0) && (p.p23 != 0.0));
        s.store_scalar(926, if s.b[926] { 1.0 } else { 0.0 });

        if s.b[926] {
            s.store_square(231, 86);
            s.store_mul3_affine_lhs(232, 122, 271, 2.0, 0.0, 151);
            s.store_sub(233, 231, 232);
            s.store_sqrt_square_offset(639, 231, ((4.0 * 0.001) * 0.001));
            s.store_offset_scaled_div(278, 231, 639, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(231, 231, 0.5, 639, 0.5, (1e-10 * 0.001));
        }

        s.b[927] = (s.v[231] < 0.0);
        s.store_scalar(927, if s.b[927] { 1.0 } else { 0.0 });

        if (s.b[926] && s.b[927]) {
            s.store_scalar(231, 0.0);
            s.store_scalar(278, 0.0);
        }

        if s.b[926] {
            s.store_sqrt_square_offset(639, 233, ((4.0 * 0.001) * 0.001));
            s.store_offset_scaled_div(278, 233, 639, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(233, 233, 0.5, 639, 0.5, (1e-10 * 0.001));
        }

        s.b[928] = (s.v[233] < 0.0);
        s.store_scalar(928, if s.b[928] { 1.0 } else { 0.0 });

        if (s.b[926] && s.b[928]) {
            s.store_scalar(233, 0.0);
            s.store_scalar(278, 0.0);
        }

        if s.b[926] {
            s.store_sub(234, 231, 233);
        }

        s.b[929] = ((s.v[149] < (10.0 * 2.220446049250313e-16)) || (s.v[234] < (10.0 * 2.220446049250313e-16)));
        s.store_scalar(929, if s.b[929] { 1.0 } else { 0.0 });

        if (s.b[926] && s.b[929]) {
            s.store_scalar(35, 0.0);
        }

        if (s.b[926] && (!s.b[929])) {
            s.store_scalar(35, 1.0);
        }

        s.b[930] = (s.v[185] > 0.0);
        s.store_scalar(930, if s.b[930] { 1.0 } else { 0.0 });

        if s.b[930] {
            s.copy_ad(279, 388);
            s.store_square(285, 270);
            s.store_mul_div_from_scalar_lhs(282, 2.0, 472, 285);
            s.store_add_scaled_inputs3_indices(283, 279, 1.0, 122, (-1.0), 70, (-s.v[486]));
            s.store_offset_mul(284, 282, 283, 1.0);
            s.store_sqrt_square_offset(639, 284, ((4.0 * 0.001) * 0.001));
            s.store_offset_scaled_div(287, 284, 639, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(284, 284, 0.5, 639, 0.5, (1e-10 * 0.001));
        }

        s.b[931] = (s.v[284] < 0.0);
        s.store_scalar(931, if s.b[931] { 1.0 } else { 0.0 });

        if (s.b[930] && s.b[931]) {
            s.store_scalar(284, 0.0);
            s.store_scalar(287, 0.0);
        }

        if s.b[930] {
            s.store_offset(284, 284, 1e-50);
            s.store_add_scaled_ad_rhs(186, 279, s.v[491], A::mul_sub_from_scalar_rhs(A::div(s.ad_value(472), s.ad_value(285)), 1.0, A::sqrt(s.ad_value(284))));
            s.store_add_scaled_inputs3_indices(187, 71, p.p123, 339, 1.0, 186, (-(s.v[487] * s.v[485])));
            s.store_sqrt_square_offset(639, 187, ((4.0 * 0.01) * 0.01));
            s.store_offset_scaled_div(287, 187, 639, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(187, 187, 0.5, 639, 0.5, (1e-10 * 0.01));
        }

        s.b[932] = (s.v[187] < 0.0);
        s.store_scalar(932, if s.b[932] { 1.0 } else { 0.0 });

        if (s.b[930] && s.b[932]) {
            s.store_scalar(187, 0.0);
            s.store_scalar(287, 0.0);
        }

        if s.b[930] {
            s.store_offset(187, 187, 1e-50);
            s.store_exp_ad(280, A::div_from_scalar((-s.v[489]), s.ad_value(187)));
            s.store_mul3_affine_lhs(185, 187, 94, s.v[488], 0.0, 280);
        }

        s.b[933] = (((s.v[34] == 0.0) && (s.v[185] > 0.0)) && (p.p145 != 0.0));
        s.store_scalar(933, if s.b[933] { 1.0 } else { 0.0 });

        if s.b[933] {
            s.store_offset_scaled(278, 80, p.p146, 1.0);
            s.store_scaled_mul(188, 278, 185, p.p145);
            s.store_offset_mul(64, 120, 56, (-1.0));
            s.store_sqrt_square_offset(639, 64, ((4.0 * 0.1) * 0.1));
            s.store_offset_add_scaled_inputs_indices(64, 64, 0.5, 639, 0.5, (1e-10 * 0.1));
        }

        s.b[934] = (s.v[64] < 0.0);
        s.store_scalar(934, if s.b[934] { 1.0 } else { 0.0 });

        if (s.b[933] && s.b[934]) {
            s.store_scalar(64, 0.0);
        }

        if s.b[933] {
            s.store_sqrt(65, 64);
            s.store_mul(66, 64, 65);
            s.store_offset_mul(69, 120, 57, (-1.0));
            s.store_sqrt_square_offset(639, 69, ((4.0 * 0.1) * 0.1));
            s.store_offset_add_scaled_inputs_indices(69, 69, 0.5, 639, 0.5, (1e-10 * 0.1));
        }

        s.b[935] = (s.v[69] < 0.0);
        s.store_scalar(935, if s.b[935] { 1.0 } else { 0.0 });

    }

    pub(super) fn stamp_reactive_block_15(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[933] && s.b[935]) {
            s.store_scalar(69, 0.0);
        }

        if s.b[933] {
            s.store_sqrt(67, 69);
            s.store_mul(68, 69, 67);
            s.store_div_scaled_product_indices(279, 120, 188, 1.0, 64, 1.0);
            s.store_div_scaled_product_indices(280, 120, 188, 1.0, 69, 1.0);
            s.store_mul_ad_rhs(190, 141, A::add_scaled_products(s.ad_value(68), s.ad_value(280), 1.0, s.ad_value(66), s.ad_value(279), (-1.0)));
            s.store_mul_add_scaled_products_indices_rhs(191, 141, 67, 280, ((-1.0) * (0.5)), 65, 279, 0.5);
            s.store_add(192, 190, 191);
            s.store_mul3_lhs(193, 189, 192, 158);
        }

        s.store_scalar(949, (s.v[272] * 100.0));

        s.store_scale(951, 123, 100.0);

        s.store_scalar(952, (s.v[466] * 100.0));

        s.store_scale(953, 160, 0.01);

        s.b[956] = (p.p17 == 0.0);
        s.store_scalar(956, if s.b[956] { 1.0 } else { 0.0 });

        if s.b[956] {
            s.store_scalar(256, 0.0);
        }

        s.b[957] = (s.v[34] == 0.0);
        s.store_scalar(957, if s.b[957] { 1.0 } else { 0.0 });

        if ((!s.b[956]) && s.b[957]) {
            s.store_offset_add(948, 74, 71, (-(10.0 * 2.220446049250313e-16)));
            s.store_add_scaled_inputs4_mixed_iiai(938, 72, 1.0, 138, (-p.p256), A::div_scaled_inputs3(s.ad_value(50), (-p.p258), s.ad_value(80), p.p206, s.ad_value(267), (-p.p206), s.ad_value(951), 1.0), 1.0, 948, (-p.p205));
            s.store_offset_scaled(944, 953, 1.0 / (p.p207), 1.0);
            s.store_scaled_mul(947, 944, 938, 1.0 / (s.v[949]));
            s.store_sqrt_square_offset(639, 947, ((4.0 * 0.01) * 0.01));
            s.store_offset_scaled_div(942, 947, 639, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(947, 947, 0.5, 639, 0.5, (1e-10 * 0.01));
        }

        s.b[958] = (s.v[947] < 0.0);
        s.store_scalar(958, if s.b[958] { 1.0 } else { 0.0 });

        if (((!s.b[956]) && s.b[957]) && s.b[958]) {
            s.store_scalar(947, 0.0);
            s.store_scalar(942, 0.0);
        }

        if ((!s.b[956]) && s.b[957]) {
            s.store_sqrt_square_offset(639, 72, ((4.0 * 0.001) * 0.001));
            s.store_offset_scaled_div(941, 72, 639, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(940, 72, 0.5, 639, 0.5, (1e-10 * 0.001));
        }

        s.b[959] = (s.v[940] < 0.0);
        s.store_scalar(959, if s.b[959] { 1.0 } else { 0.0 });

        if (((!s.b[956]) && s.b[957]) && s.b[959]) {
            s.store_scalar(940, 0.0);
            s.store_scalar(941, 0.0);
        }

        if ((!s.b[956]) && s.b[957]) {
            s.store_scaled_offset(936, 940, (-p.p216), 10.0);
            s.store_sub_from_scalar_ad(938, 1.0, A::div_scalar_offset_denominator(1.0, A::square(s.ad_value(936)), 1.0, 1.0));
            s.store_mul(947, 947, 938);
            s.store_scale(937, 951, s.v[952]);
            s.store_div_from_scalar_offset_input(944, p.p209, 937, p.p209);
            s.store_div_from_scalar_offset_square(941, 1.0, 947, 1e-50);
            s.store_scaled_mul(938, 246, 941, (-p.p204));
        }

        s.b[960] = (s.v[938] < (-34.0));
        s.store_scalar(960, if s.b[960] { 1.0 } else { 0.0 });

        if (((!s.b[956]) && s.b[957]) && (!s.b[960])) {
            s.store_mul_scale_ad_lhs(940, A::div_from_scalar(p.p203, s.ad_value(245)), 1.6021918e-19, 937);
        }

        if (!s.b[956]) {
            s.store_offset_scaled(937, 52, (-p.p211), p.p212);
            s.store_exp_scaled_input(939, 937, s.v[949]);
            s.store_scale(938, 52, p.p260);
            s.store_scalar(937, ((1.0 / s.v[949]) / s.v[949]));
            s.store_mul_square_lhs(940, 938, 937);
            s.store_scalar(941, (((p.p210 / 1000000.0) * s.v[952]) * ((s.v[375]) as f64).powf(p.p259)));
            s.store_sub(942, 52, 51);
            s.store_offset_scaled(937, 942, (-p.p211), p.p212);
            s.store_exp_scaled_input(939, 937, s.v[949]);
            s.store_scale(938, 942, p.p260);
            s.store_scalar(937, ((1.0 / s.v[949]) / s.v[949]));
            s.store_mul_square_lhs(940, 938, 937);
            s.store_scalar(941, (((p.p210 / 1000000.0) * s.v[952]) * ((s.v[375]) as f64).powf(p.p259)));
            s.store_scaled_offset_ad(947, A::add_scaled_inputs3(s.ad_value(50), p.p261, s.ad_value(52), (-1.0), s.ad_value(138), 1.0), p.p215, 1.0 / (s.v[949]));
            s.store_sqrt_square_offset(639, 947, ((4.0 * 0.01) * 0.01));
            s.store_offset_scaled_div(942, 947, 639, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(947, 947, 0.5, 639, 0.5, (1e-10 * 0.01));
        }

        s.b[963] = (s.v[947] < 0.0);
        s.store_scalar(963, if s.b[963] { 1.0 } else { 0.0 });

        if ((!s.b[956]) && s.b[963]) {
            s.store_scalar(947, 0.0);
            s.store_scalar(942, 0.0);
        }

        if (!s.b[956]) {
            s.store_offset(947, 947, 1e-50);
            s.store_div_from_scalar_powf_ad(938, (-p.p214), s.ad_value(947), p.p263);
        }

        s.b[964] = (s.v[938] < (-34.0));
        s.store_scalar(964, if s.b[964] { 1.0 } else { 0.0 });

        if ((!s.b[956]) && (!s.b[964])) {
            s.store_exp(939, 938);
            s.store_scalar(940, (s.v[375] + p.p264));
            s.store_sub_scaled_ad_lhs(638, A::offset(s.ad_value(940), (-p.p265)), 940, 0.001);
            s.store_scale(639, 940, (0.001 * (4.0 * p.p265)));
        }

        if ((!s.b[956]) && (!s.b[964])) {
            if (s.v[639] > 0.0) {
            } else {
                s.store_neg(639, 639);
            }
        }

        if ((!s.b[956]) && (!s.b[964])) {
            s.store_sqrt_square_add(639, 638, 639);
            s.store_offset_scaled_div(937, 638, 639, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(940, 638, 0.5, 639, 0.5, p.p265);
            s.store_scale(940, 940, ((p.p213 * 1e-6) * s.v[952]));
            s.store_mul_ad_product_lhs_mixed_ia(252, 940, A::powf(s.ad_value(947), p.p262), 939);
            s.store_scaled_offset_ad(947, A::add_scaled_inputs3(s.ad_value(50), p.p269, s.ad_value(52), (-1.0), s.ad_value(138), 1.0), p.p268, 1.0 / (s.v[949]));
            s.store_sqrt_square_offset(639, 947, ((4.0 * 0.01) * 0.01));
            s.store_offset_scaled_div(942, 947, 639, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(947, 947, 0.5, 639, 0.5, (1e-10 * 0.01));
        }

        s.b[965] = (s.v[947] < 0.0);
        s.store_scalar(965, if s.b[965] { 1.0 } else { 0.0 });

        if (((!s.b[956]) && (!s.b[964])) && s.b[965]) {
            s.store_scalar(947, 0.0);
            s.store_scalar(942, 0.0);
        }

        if ((!s.b[956]) && (!s.b[964])) {
            s.store_offset(947, 947, 1e-50);
            s.store_div_from_scalar_powf_ad(938, (-p.p267), s.ad_value(947), p.p271);
        }

        s.b[966] = (s.v[938] < (-34.0));
        s.store_scalar(966, if s.b[966] { 1.0 } else { 0.0 });

        if (((!s.b[956]) && (!s.b[964])) && s.b[966]) {
            s.store_scalar(253, 0.0);
        }

        if (((!s.b[956]) && (!s.b[964])) && (!s.b[966])) {
            s.store_exp(939, 938);
            s.store_scalar(940, (s.v[375] + p.p272));
            s.store_sub_scaled_ad_lhs(638, A::offset(s.ad_value(940), (-p.p273)), 940, 0.001);
            s.store_scale(639, 940, (0.001 * (4.0 * p.p273)));
        }

        if (((!s.b[956]) && (!s.b[964])) && (!s.b[966])) {
            if (s.v[639] > 0.0) {
            } else {
                s.store_neg(639, 639);
            }
        }

        if (((!s.b[956]) && (!s.b[964])) && (!s.b[966])) {
            s.store_sqrt_square_add(639, 638, 639);
            s.store_offset_scaled_div(937, 638, 639, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(940, 638, 0.5, 639, 0.5, p.p273);
            s.store_scale(940, 940, ((p.p266 * 1e-6) * s.v[952]));
            s.store_mul_ad_product_lhs_mixed_ia(253, 940, A::powf(s.ad_value(947), p.p270), 939);
        }

        if ((!s.b[956]) && (!s.b[964])) {
            s.store_scale(938, 252, (-0.001));
        }

        s.b[967] = (s.v[938] < 1e-50);
        s.store_scalar(967, if s.b[967] { 1.0 } else { 0.0 });

        if (((!s.b[956]) && (!s.b[964])) && s.b[967]) {
            s.store_scalar(938, 1e-50);
        }

        if ((!s.b[956]) && (!s.b[964])) {
            s.store_add_scaled_inputs3_indices(638, 252, -1.0, 253, 1.0, 938, -1.0);
            s.store_scaled_mul(639, 253, 938, (-4.0));
        }

        if ((!s.b[956]) && (!s.b[964])) {
            if (s.v[639] > 0.0) {
            } else {
                s.store_neg(639, 639);
            }
        }

        if ((!s.b[956]) && (!s.b[964])) {
            s.store_sqrt_square_add(639, 638, 639);
        }

        if (!s.b[956]) {
            s.store_scalar(256, 0.5);
        }

        s.b[968] = (p.p18 == 0.0);
        s.store_scalar(968, if s.b[968] { 1.0 } else { 0.0 });

        if (!s.b[968]) {
            s.store_add_scaled_inputs4_offset_indices(279, 51, p.p198, 52, (-1.0), 82, (-p.p200), 266, (-p.p200), (p.p199 * p.p198));
            s.store_scale(247, 279, 1.0 / (p.p228));
            s.store_sqrt_square_offset(639, 247, ((4.0 * 0.01) * 0.01));
            s.store_offset_scaled_div(283, 247, 639, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(248, 247, 0.5, 639, 0.5, (1e-10 * 0.01));
        }

        s.b[969] = (s.v[248] < 0.0);
        s.store_scalar(969, if s.b[969] { 1.0 } else { 0.0 });

        if ((!s.b[968]) && s.b[969]) {
            s.store_scalar(248, 0.0);
            s.store_scalar(283, 0.0);
        }

        if (!s.b[968]) {
            s.store_div_scaled_value_offset_denominator(278, s.ad_value(246), (-s.v[627]), s.ad_value(248), 1e-50, 1.0);
        }

        s.b[970] = (s.v[278] < (-34.0));
        s.store_scalar(970, if s.b[970] { 1.0 } else { 0.0 });

        if ((!s.b[968]) && (!s.b[970])) {
            s.store_scale_ad(280, A::div_from_scalar(s.v[628], s.ad_value(245)), (1.6021918e-19 * s.v[466]));
        }

        s.b[971] = (p.p18 == 0.0);
        s.store_scalar(971, if s.b[971] { 1.0 } else { 0.0 });

        if (!s.b[971]) {
            s.store_add_scaled_inputs3_mixed_aii(279, A::add_scaled_inputs3_offset(s.ad_value(51), (-p.p198), s.ad_value(52), -1.0, s.ad_value(51), 1.0, ((p.p199) * (p.p198))), 1.0, 82, (-p.p200), 266, (-p.p200));
            s.store_scale(247, 279, 1.0 / (p.p228));
            s.store_sqrt_square_offset(639, 247, ((4.0 * 0.01) * 0.01));
            s.store_offset_scaled_div(283, 247, 639, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(249, 247, 0.5, 639, 0.5, (1e-10 * 0.01));
        }

        s.b[972] = (s.v[249] < 0.0);
        s.store_scalar(972, if s.b[972] { 1.0 } else { 0.0 });

        if ((!s.b[971]) && s.b[972]) {
            s.store_scalar(249, 0.0);
            s.store_scalar(283, 0.0);
        }

        if (!s.b[971]) {
            s.store_div_scaled_value_offset_denominator(278, s.ad_value(246), (-s.v[627]), s.ad_value(249), 1e-50, 1.0);
        }

        s.b[973] = (s.v[278] < (-34.0));
        s.store_scalar(973, if s.b[973] { 1.0 } else { 0.0 });

        if ((!s.b[971]) && (!s.b[973])) {
            s.store_scale_ad(280, A::div_from_scalar(s.v[628], s.ad_value(245)), (1.6021918e-19 * s.v[466]));
        }

        s.store_scalar(264, p.p176);

        s.store_scalar(261, 0.0);

        s.b[974] = (s.v[34] != 0.0);
        s.store_scalar(974, if s.b[974] { 1.0 } else { 0.0 });

    }

    pub(super) fn stamp_reactive_block_16(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[974] {
            s.store_add(280, 51, 56);
            s.store_add_scaled_inputs(260, 280, s.v[264], 57, (1.0 - s.v[264]));
        }

        s.b[975] = (s.v[260] > ((s.v[56] + s.v[51]) - (10.0 * 2.220446049250313e-16)));
        s.store_scalar(975, if s.b[975] { 1.0 } else { 0.0 });

        if (s.b[974] && s.b[975]) {
            s.store_offset_add(260, 56, 51, (-(10.0 * 2.220446049250313e-16)));
        }

        s.b[976] = (p.p45 != 0.0);
        s.store_scalar(976, if s.b[976] { 1.0 } else { 0.0 });

        s.b[977] = (s.v[151] > 1e-15);
        s.store_scalar(977, if s.b[977] { 1.0 } else { 0.0 });

        if (((!s.b[974]) && s.b[976]) && s.b[977]) {
            s.store_div_scaled_product_by_product(261, s.ad_value(151), s.ad_value(122), 1.0, s.ad_value(123), s.ad_value(149), 1.0);
        }

        s.store_scalar(435, s.v[273]);

        s.store_scalar(436, (1.0 / s.v[435]));

        s.b[978] = (((p.p19 >= 1.0) && (p.p175 > 0.0)) && (s.v[624] > 0.0));
        s.store_scalar(978, if s.b[978] { 1.0 } else { 0.0 });

        if s.b[978] {
            s.store_scalar(195, p.p175);
            s.store_mul_sqrt_ad_rhs(437, 141, A::div_from_scalar(s.v[624], s.ad_value(457)));
            s.store_scalar(399, ((1.0 - -1.0) / 2.0));
            s.store_scalar(400, ((1.0 + -1.0) / 2.0));
            s.store_add_scaled_products_indices(402, 399, 412, 1.0, 400, 413, 1.0);
            s.store_add_scaled_products_indices(403, 399, 413, 1.0, 400, 412, 1.0);
        }

        if (s.b[978] && (s.v[399] != 0.0)) {
            s.store_add_scaled_products_right_right_ad(414, 412, 42, 1.0, 413, A::sub(s.ad_value(42), s.ad_value(41)), 1.0);
        }

        if (s.b[978] && (s.v[400] != 0.0)) {
            s.store_add_scaled_products_right_right_ad(414, 413, 42, 1.0, 412, A::sub(s.ad_value(42), s.ad_value(41)), 1.0);
        }

        if s.b[978] {
            s.store_scalar(415, 0.0);
            s.store_neg(278, 415);
        }

        s.b[979] = (s.v[278] > s.v[31]);
        s.store_scalar(979, if s.b[979] { 1.0 } else { 0.0 });

        if (s.b[978] && s.b[979]) {
            s.store_sub(279, 278, 31);
            s.store_sub_from_scalar(280, s.v[30], 31);
            s.store_div(638, 279, 280);
            s.store_square(639, 638);
            s.store_mul(640, 639, 638);
            s.store_square(641, 639);
            s.store_div_from_scalar_ad(291, 1.0, A::add_scaled_inputs4_offset(s.ad_value(638), 1.0, s.ad_value(639), 1.0, s.ad_value(640), 1.0, s.ad_value(641), 1.0, 1.0));
            s.store_mul_ad_affine_product_lhs(387, A::add_scaled_inputs3_offset(s.ad_value(638), 2.0, s.ad_value(639), 3.0, s.ad_value(640), 4.0, 1.0), s.ad_value(291), -1.0, 0.0, 291);
            s.store_mul_sub_from_scalar_rhs(291, 280, 1.0, 291);
            s.store_neg(387, 387);
            s.store_add(288, 31, 291);
        }

        if (s.b[978] && (!s.b[979])) {
            s.copy_ad(288, 278);
        }

        if s.b[978] {
            s.store_offset_scaled(416, 288, -1.0, (-1e-12));
            s.store_scale(144, 437, s.v[436]);
            s.store_square(145, 144);
            s.store_sub_from_scalar(404, p.p39, 414);
            s.store_mul_div_from_scalar_lhs_ad_mixed_ia(417, 2.0, 120, A::ln(A::div_from_scalar(s.v[624], s.ad_value(127))));
            s.store_neg(419, 416);
        }

        s.b[980] = (s.v[404] < s.v[419]);
        s.store_scalar(980, if s.b[980] { 1.0 } else { 0.0 });

        if (s.b[978] && s.b[980]) {
            s.store_div_from_scalar_mul_ad(291, s.v[435], s.ad_value(120), s.ad_value(437));
            s.store_offset_scaled(184, 291, (3.0 * 1.414213562373095), 2.0);
            s.store_mul3_affine_lhs(182, 184, 184, 8.0, 0.0, 184);
            s.store_sub(176, 137, 417);
            s.store_mul_add_rhs(290, 120, 404, 416);
            s.store_sub_from_scalar_scaled_mul_ad_rhs(183, (7.0 * 1.414213562373095), 291, A::offset(s.ad_value(290), (-2.0)), 9.0);
            s.store_square(181, 183);
        }

        s.b[981] = (s.v[182] < (s.v[181] * 1e-8));
        s.store_scalar(981, if s.b[981] { 1.0 } else { 0.0 });

        if ((s.b[978] && s.b[980]) && s.b[981]) {
            s.store_add_scaled_inputs_product_mixed_aaia(179, A::offset(s.ad_value(183), ((-7.0) * 1.414213562373095)), 1.0, A::div_scaled_inputs(s.ad_value(182), 0.5, s.ad_value(183), 1.0), 1.0, 291, A::offset(s.ad_value(290), (-2.0)), 9.0);
        }

        if ((s.b[978] && s.b[980]) && (!s.b[981])) {
            s.store_sqrt_add(180, 182, 181);
            s.store_add_scaled_offset_product_rhs_mixed_aii(179, A::offset(s.ad_value(180), ((-7.0) * 1.414213562373095)), 1.0, 291, 290, (-2.0), 9.0);
        }

        if (s.b[978] && s.b[980]) {
            s.store_powf(178, 179, 0.3333333333333333);
            s.store_add_scaled_inputs_product_first_ad(177, A::sub_from_scalar(((-4.0) * 1.414213562373095), A::scale(s.ad_value(291), 12.0)), 1.0, 178, 2.0, 178, 178, 1.414213562373095);
            s.store_div(77, 177, 178);
            s.store_add_scaled_product_indices(259, 416, (-1.0), 77, 122, 1.0);
            s.store_add(279, 259, 416);
            s.store_div(280, 279, 176);
            s.store_sub_div_lhs_mixed_ia(410, 279, A::sqrt_square_offset(s.ad_value(280), 1.0), 416);
            s.store_scaled_sub(408, 404, 410, s.v[435]);
            s.copy_ad(407, 408);
        }

        if (s.b[978] && (!s.b[980])) {
            s.store_scalar(77, 3.0);
            s.store_sub_div_lhs_indices(319, 77, 120, 416);
            s.store_offset_div_scaled_inputs2(290, A::offset(A::mul(s.ad_value(120), A::add(s.ad_value(404), s.ad_value(416))), (-1.0)), 4.0, A::exp_scaled_input(s.ad_value(77), -1.0), 4.0, A::mul(s.ad_value(145), s.ad_value(121)), 1.0, 1.0);
        }

        s.b[982] = (s.v[290] < (10.0 * 2.220446049250313e-16));
        s.store_scalar(982, if s.b[982] { 1.0 } else { 0.0 });

        if ((s.b[978] && (!s.b[980])) && s.b[982]) {
            s.store_scalar(290, (10.0 * 2.220446049250313e-16));
        }

        if (s.b[978] && (!s.b[980])) {
            s.store_add_product3_rhs_mixed_iia(319, 404, 145, 120, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(290))), 1.0 / (2.0));
            s.store_mul_add_rhs(77, 120, 319, 416);
            s.store_offset_div_scaled_inputs2(290, A::offset(A::mul(s.ad_value(120), A::add(s.ad_value(404), s.ad_value(416))), (-1.0)), 4.0, A::exp_scaled_input(s.ad_value(77), -1.0), 4.0, A::mul(s.ad_value(145), s.ad_value(121)), 1.0, 1.0);
        }

        s.b[983] = (s.v[290] < (10.0 * 2.220446049250313e-16));
        s.store_scalar(983, if s.b[983] { 1.0 } else { 0.0 });

        if ((s.b[978] && (!s.b[980])) && s.b[983]) {
            s.store_scalar(290, (10.0 * 2.220446049250313e-16));
        }

        if (s.b[978] && (!s.b[980])) {
            s.store_add_product3_rhs_mixed_iia(319, 404, 145, 120, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(290))), 1.0 / (2.0));
            s.store_mul_add_rhs(77, 120, 319, 416);
        }

        s.b[984] = (s.v[77] < 3.0);
        s.store_scalar(984, if s.b[984] { 1.0 } else { 0.0 });

        if ((s.b[978] && (!s.b[980])) && s.b[984]) {
            s.store_scalar(421, ((1.0 / (9.0 * 1.414213562373095)) - ((5.0 + (7.0 * 0.049787068367863944)) / (54.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt()))));
            s.store_scalar(422, (((1.0 + 0.049787068367863944) / (2.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt())) - (1.414213562373095 / 3.0)));
            s.store_offset_div_from_scalar_ad(423, 1.0, A::mul(s.ad_value(120), s.ad_value(144)), (1.0 / 1.414213562373095));
            s.store_div_scaled_inputs2_indices(425, 404, -1.0, 416, -1.0, 144, 1.0);
            s.store_add_scaled_inputs3_div_scaled_third_mixed_aaii(426, A::div_scaled_product(A::square(s.ad_value(422)), s.ad_value(422), 1.0, A::mul3_scaled_output(s.ad_value(421), s.ad_value(421), s.ad_value(421), 27.0), 1.0), 1.0, A::div_scaled_product_by_product(s.ad_value(422), s.ad_value(423), 1.0, s.ad_value(421), s.ad_value(421), 6.0), (-1.0), 425, 1.0, 421, 2.0, 1.0);
            s.store_div_ad(424, A::add_scaled_square_product(s.ad_value(422), (-1.0), s.ad_value(421), s.ad_value(423), 3.0), A::mul_scaled_lhs(s.ad_value(421), 9.0, s.ad_value(421)));
            s.store_sqrt_add_scaled_square_cube_product(283, 426, 1.0, 424, 1.0);
            s.store_powf_ad(427, A::sub(s.ad_value(283), s.ad_value(426)), 0.3333333333333333);
            s.store_neg_powf_add_input(428, 426, 283, 0.3333333333333333);
            s.store_add_scaled_inputs3_div_scaled_third_indices(290, 427, 1.0, 428, 1.0, 422, 1.0, 421, 3.0, -1.0);
            s.store_add_scaled_product_indices(319, 416, (-1.0), 290, 122, 1.0);
            s.store_mul_add_rhs(77, 120, 319, 416);
        }

        s.b[985] = (p.p30 > 0.0);
        s.store_scalar(985, if s.b[985] { 1.0 } else { 0.0 });

        if ((s.b[978] && (!s.b[980])) && s.b[985]) {
            s.store_offset_add(420, 404, 416, 0.1);
            s.store_offset_exp_ad(203, A::mul_scaled_rhs(s.ad_value(120), s.ad_value(416), -1.0), 1e-50);
            s.store_scale(278, 127, 1.0 / (s.v[624]));
            s.store_square(429, 278);
            s.store_mul(430, 429, 203);
            s.store_mul(278, 121, 145);
            s.store_mul(434, 120, 420);
            s.store_add_scaled_inputs_product_mixed_aaii(433, A::ln(A::add_scaled_square_product(s.ad_value(434), 1.0, s.ad_value(430), s.ad_value(278), 1.0)), 1.0, A::ln(A::mul(s.ad_value(429), s.ad_value(278))), (-1.0), 120, 416, 1.0);
            s.store_offset_sub(638, 434, 433, (-1.0));
            s.store_scale(639, 434, 4.0);
        }

        if ((s.b[978] && (!s.b[980])) && s.b[985]) {
            if (s.v[639] > 0.0) {
            } else {
                s.store_neg(639, 639);
            }
        }

        if ((s.b[978] && (!s.b[980])) && s.b[985]) {
            s.store_sqrt_square_add(639, 638, 639);
            s.store_offset_scaled_div(279, 638, 639, 0.5, 0.5);
            s.store_offset_scaled_ad(280, A::div_scaled_offset_numerator(s.ad_value(638), 1.0, 2.0, s.ad_value(639), 1.0), (-0.5), 0.5);
            s.store_add_scaled_inputs3_indices(433, 434, 1.0, 638, (-0.5), 639, (-0.5));
            s.store_sub(434, 434, 433);
            s.store_add_scaled_inputs(434, 434, 1.0, 120, 0.1);
            s.store_add_scaled_inputs_product_mixed_aaii(432, A::ln(A::add_scaled_square_product(s.ad_value(434), 1.0, s.ad_value(430), s.ad_value(278), 1.0)), 1.0, A::ln(A::mul(s.ad_value(429), s.ad_value(278))), (-1.0), 120, 416, 1.0);
            s.store_sub_div_lhs_indices(320, 432, 120, 416);
            s.copy_ad(431, 77);
            s.store_offset_sub(638, 432, 431, (-(0.0008 * 75.0)));
            s.store_scale(639, 432, (4.0 * (0.0008 * 75.0)));
        }

        if ((s.b[978] && (!s.b[980])) && s.b[985]) {
            if (s.v[639] > 0.0) {
            } else {
                s.store_neg(639, 639);
            }
        }

        if ((s.b[978] && (!s.b[980])) && s.b[985]) {
            s.store_sqrt_square_add(639, 638, 639);
            s.store_offset_scaled_div(279, 638, 639, 0.5, 0.5);
            s.store_offset_scaled_ad(280, A::div_scaled_offset_numerator(s.ad_value(638), 1.0, ((2.0 * 0.0008) * 75.0), s.ad_value(639), 1.0), (-0.5), 0.5);
            s.store_add_scaled_inputs3_indices(77, 432, 1.0, 638, (-0.5), 639, (-0.5));
        }

        if (s.b[978] && (!s.b[980])) {
            s.store_sub_div_lhs_indices(410, 77, 120, 416);
            s.store_add_offset_lhs_ad_rhs(279, 77, (-1.0), A::exp_scaled_input(s.ad_value(77), -1.0));
        }

        s.b[986] = (s.v[279] < (10.0 * 2.220446049250313e-16));
        s.store_scalar(986, if s.b[986] { 1.0 } else { 0.0 });

        if ((s.b[978] && (!s.b[980])) && s.b[986]) {
            s.store_scalar(279, (10.0 * 2.220446049250313e-16));
        }

        if (s.b[978] && (!s.b[980])) {
            s.store_mul_sqrt_rhs(407, 437, 279);
            s.store_scaled_sub(408, 404, 410, s.v[435]);
        }

        s.b[987] = (p.p30 == 1.0);
        s.store_scalar(987, if s.b[987] { 1.0 } else { 0.0 });

        if ((s.b[978] && (!s.b[980])) && s.b[987]) {
            s.store_exp_ad(203, A::mul_scaled_rhs(s.ad_value(120), s.ad_value(416), -1.0));
            s.store_scale(278, 127, 1.0 / (s.v[624]));
            s.store_square(429, 278);
            s.store_mul(204, 429, 203);
            s.store_scalar(379, 0.0);
            s.store_scalar(62, 1.0);
        }

    }

    pub(super) fn stamp_reactive_block_17(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        let mut assign19230_loop_guard: usize = 0;
        while {
            let assign19230_cond_e23449: f64 = (40.0 + 1.0);
            let assign19230_cond_e23451: f64 = if (((s.b[978] && (!s.b[980])) && s.b[987]) && (s.v[62] <= assign19230_cond_e23449)) { 1.0 } else { 0.0 };
            assign19230_cond_e23451 != 0.0
        } {
            assign19230_loop_guard += 1;
            assert!(assign19230_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.b[978] && (!s.b[980])) && s.b[987]) {
                s.store_mul_add_rhs(77, 120, 410, 416);
            }
            s.b[988] = (s.v[77] < 5.0);
            s.store_scalar(988, if s.b[988] { 1.0 } else { 0.0 });
            if (((s.b[978] && (!s.b[980])) && s.b[987]) && s.b[988]) {
                s.store_mul3_ad_middle(205, A::square(s.ad_value(77)), 77, A::offset(A::mul(s.ad_value(77), A::scale_offset(s.ad_value(77), 0.006115288895133179, (-0.07053654284009761))), 0.29693154855771));
                s.store_mul_offset_rhs_ad(206, A::square(s.ad_value(77)), A::mul(s.ad_value(77), A::scale_offset(s.ad_value(77), (5.0 * 0.006115288895133179), (4.0 * (-0.07053654284009761)))), (3.0 * 0.29693154855771));
                s.store_mul3_lhs(207, 204, 205, 205);
                s.store_mul_product3_indices(208, 206, 204, 120, 205, 2.0);
                s.store_mul_offset_ad_rhs(146, 77, A::mul_offset_rhs(s.ad_value(77), A::mul_offset_rhs(s.ad_value(77), A::mul(s.ad_value(77), A::scale_offset(s.ad_value(77), 6.36964918866352e-5, (-0.00163730162779191))), 0.0178800506338833), (-0.117851130197758)), 0.707106781186548);
                s.store_offset_mul_offset_rhs_ad_rhs(148, 77, A::mul_offset_rhs(s.ad_value(77), A::mul(s.ad_value(77), A::scale_offset(s.ad_value(77), (5.0 * 6.36964918866352e-5), (4.0 * (-0.00163730162779191)))), (3.0 * 0.0178800506338833)), (2.0 * (-0.117851130197758)), 0.707106781186548);
                s.store_sqrt_offset_ad(209, A::add(A::square(s.ad_value(146)), s.ad_value(207)), 1e-50);
                s.store_div_scaled_inputs2_mixed_aii(210, A::mul3_scaled_output(s.ad_value(120), s.ad_value(148), s.ad_value(146), 2.0), 1.0, 208, 1.0, 209, 2.0);
            }
            s.b[989] = (s.v[77] < 80.0);
            s.store_scalar(989, if s.b[989] { 1.0 } else { 0.0 });
            if ((((s.b[978] && (!s.b[980])) && s.b[987]) && (!s.b[988])) && s.b[989]) {
                s.store_exp(147, 77);
                s.store_mul_offset_rhs(207, 204, 147, (-1.0));
                s.store_mul3_lhs(208, 204, 120, 147);
            }
            if ((((s.b[978] && (!s.b[980])) && s.b[987]) && (!s.b[988])) && (!s.b[989])) {
                s.store_exp_mul(202, 120, 410);
                s.store_mul_sub_rhs(207, 429, 202, 203);
                s.store_mul3_lhs(208, 429, 120, 202);
            }
            if (((s.b[978] && (!s.b[980])) && s.b[987]) && (!s.b[988])) {
                s.store_sqrt_add_ad(209, A::offset(s.ad_value(77), (-1.0)), s.ad_value(207));
                s.store_scale_ad(210, A::div_scaled_inputs2(s.ad_value(120), 1.0, s.ad_value(208), 1.0, s.ad_value(209), 1.0), 0.5);
            }
            if ((s.b[978] && (!s.b[980])) && s.b[987]) {
                s.store_add_scaled_inputs_product_indices(211, 404, 1.0, 410, (-1.0), 144, 209, (-1.0));
                s.store_sub_from_scalar_scaled_mul(212, (-1.0), 144, 210, 1.0);
            }
            s.b[990] = (s.v[379] == 1.0);
            s.store_scalar(990, if s.b[990] { 1.0 } else { 0.0 });
            if (((s.b[978] && (!s.b[980])) && s.b[987]) && s.b[990]) {
                s.store_scalar(62, (40.0 + 1.0));
            }
            if (((s.b[978] && (!s.b[980])) && s.b[987]) && (!s.b[990])) {
                s.store_div_scaled_inputs_indices(213, 211, -1.0, 212, 1.0);
            }
            if (((s.b[978] && (!s.b[980])) && s.b[987]) && (!s.b[990])) {
                s.store_scaled_offset_ad(214, {
                    if (1.0 >= ((s.v[410]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(410))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[991] = (((s.v[213]) as f64).abs() > s.v[214]);
            s.store_scalar(991, if s.b[991] { 1.0 } else { 0.0 });
            if ((((s.b[978] && (!s.b[980])) && s.b[987]) && (!s.b[990])) && s.b[991]) {
                s.store_scale(213, 214, (if (s.v[213] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if (((s.b[978] && (!s.b[980])) && s.b[987]) && (!s.b[990])) {
                s.store_add(410, 410, 213);
            }
            s.b[992] = ((((s.v[213]) as f64).abs() <= 1e-12) && (((s.v[211]) as f64).abs() <= 1e-8));
            s.store_scalar(992, if s.b[992] { 1.0 } else { 0.0 });
            if ((((s.b[978] && (!s.b[980])) && s.b[987]) && (!s.b[990])) && s.b[992]) {
                s.store_scalar(379, 1.0);
            }
            if ((s.b[978] && (!s.b[980])) && s.b[987]) {
                s.store_offset(62, 62, 1.0);
            }
        }

        s.b[994] = (s.v[77] < 5.0);
        s.store_scalar(994, if s.b[994] { 1.0 } else { 0.0 });

        if (((s.b[978] && (!s.b[980])) && s.b[987]) && s.b[994]) {
            s.store_offset_square(64, 146, (10.0 * 2.220446049250313e-16));
            s.store_offset(65, 146, (10.0 * 2.220446049250313e-16));
        }

        if (((s.b[978] && (!s.b[980])) && s.b[987]) && (!s.b[994])) {
            s.store_offset(64, 77, (-1.0));
            s.store_sqrt(65, 64);
        }

        if ((s.b[978] && (!s.b[980])) && s.b[987]) {
            s.store_mul(407, 437, 65);
            s.store_div_from_scalar_add_ad(279, 1.0, s.ad_value(209), s.ad_value(65));
            s.store_mul3_lhs(409, 437, 207, 279);
            s.store_add(408, 407, 409);
        }

        if s.b[978] {
            s.store_sub(409, 408, 407);
            s.store_scale(282, 195, s.v[513]);
        }

        if (s.b[978] && (s.v[402] != 0.0)) {
            s.store_mul(398, 282, 408);
            s.store_mul(406, 282, 407);
        }

        if (s.b[978] && (s.v[403] != 0.0)) {
            s.store_mul(397, 282, 408);
            s.store_mul(405, 282, 407);
        }

        if s.b[978] {
            s.store_scalar(399, ((1.0 - 1.0) / 2.0));
            s.store_scalar(400, ((1.0 + 1.0) / 2.0));
            s.store_add_scaled_products_indices(402, 399, 412, 1.0, 400, 413, 1.0);
            s.store_add_scaled_products_indices(403, 399, 413, 1.0, 400, 412, 1.0);
        }

        if (s.b[978] && (s.v[399] != 0.0)) {
            s.store_add_scaled_products_right_right_ad(414, 412, 42, 1.0, 413, A::sub(s.ad_value(42), s.ad_value(41)), 1.0);
        }

        if (s.b[978] && (s.v[400] != 0.0)) {
            s.store_add_scaled_products_right_right_ad(414, 413, 42, 1.0, 412, A::sub(s.ad_value(42), s.ad_value(41)), 1.0);
        }

        if s.b[978] {
            s.store_scalar(415, 0.0);
            s.store_neg(278, 415);
        }

        s.b[996] = (s.v[278] > s.v[31]);
        s.store_scalar(996, if s.b[996] { 1.0 } else { 0.0 });

        if (s.b[978] && s.b[996]) {
            s.store_sub(279, 278, 31);
            s.store_sub_from_scalar(280, s.v[30], 31);
            s.store_div(638, 279, 280);
            s.store_square(639, 638);
            s.store_mul(640, 639, 638);
            s.store_square(641, 639);
            s.store_div_from_scalar_ad(291, 1.0, A::add_scaled_inputs4_offset(s.ad_value(638), 1.0, s.ad_value(639), 1.0, s.ad_value(640), 1.0, s.ad_value(641), 1.0, 1.0));
            s.store_mul_ad_affine_product_lhs(387, A::add_scaled_inputs3_offset(s.ad_value(638), 2.0, s.ad_value(639), 3.0, s.ad_value(640), 4.0, 1.0), s.ad_value(291), -1.0, 0.0, 291);
            s.store_mul_sub_from_scalar_rhs(291, 280, 1.0, 291);
            s.store_neg(387, 387);
            s.store_add(288, 31, 291);
        }

        if (s.b[978] && (!s.b[996])) {
            s.copy_ad(288, 278);
        }

        if s.b[978] {
            s.store_offset_scaled(416, 288, -1.0, (-1e-12));
            s.store_scale(144, 437, s.v[436]);
            s.store_square(145, 144);
            s.store_sub_from_scalar(404, p.p39, 414);
            s.store_mul_div_from_scalar_lhs_ad_mixed_ia(417, 2.0, 120, A::ln(A::div_from_scalar(s.v[624], s.ad_value(127))));
            s.store_neg(419, 416);
        }

        s.b[997] = (s.v[404] < s.v[419]);
        s.store_scalar(997, if s.b[997] { 1.0 } else { 0.0 });

        if (s.b[978] && s.b[997]) {
            s.store_div_from_scalar_mul_ad(291, s.v[435], s.ad_value(120), s.ad_value(437));
            s.store_offset_scaled(184, 291, (3.0 * 1.414213562373095), 2.0);
            s.store_mul3_affine_lhs(182, 184, 184, 8.0, 0.0, 184);
            s.store_sub(176, 137, 417);
            s.store_mul_add_rhs(290, 120, 404, 416);
            s.store_sub_from_scalar_scaled_mul_ad_rhs(183, (7.0 * 1.414213562373095), 291, A::offset(s.ad_value(290), (-2.0)), 9.0);
            s.store_square(181, 183);
        }

        s.b[998] = (s.v[182] < (s.v[181] * 1e-8));
        s.store_scalar(998, if s.b[998] { 1.0 } else { 0.0 });

        if ((s.b[978] && s.b[997]) && s.b[998]) {
            s.store_add_scaled_inputs_product_mixed_aaia(179, A::offset(s.ad_value(183), ((-7.0) * 1.414213562373095)), 1.0, A::div_scaled_inputs(s.ad_value(182), 0.5, s.ad_value(183), 1.0), 1.0, 291, A::offset(s.ad_value(290), (-2.0)), 9.0);
        }

        if ((s.b[978] && s.b[997]) && (!s.b[998])) {
            s.store_sqrt_add(180, 182, 181);
            s.store_add_scaled_offset_product_rhs_mixed_aii(179, A::offset(s.ad_value(180), ((-7.0) * 1.414213562373095)), 1.0, 291, 290, (-2.0), 9.0);
        }

        if (s.b[978] && s.b[997]) {
            s.store_powf(178, 179, 0.3333333333333333);
            s.store_add_scaled_inputs_product_first_ad(177, A::sub_from_scalar(((-4.0) * 1.414213562373095), A::scale(s.ad_value(291), 12.0)), 1.0, 178, 2.0, 178, 178, 1.414213562373095);
            s.store_div(77, 177, 178);
            s.store_add_scaled_product_indices(259, 416, (-1.0), 77, 122, 1.0);
            s.store_add(279, 259, 416);
            s.store_div(280, 279, 176);
            s.store_sub_div_lhs_mixed_ia(410, 279, A::sqrt_square_offset(s.ad_value(280), 1.0), 416);
            s.store_scaled_sub(408, 404, 410, s.v[435]);
            s.copy_ad(407, 408);
        }

        if (s.b[978] && (!s.b[997])) {
            s.store_scalar(77, 3.0);
            s.store_sub_div_lhs_indices(319, 77, 120, 416);
            s.store_offset_div_scaled_inputs2(290, A::offset(A::mul(s.ad_value(120), A::add(s.ad_value(404), s.ad_value(416))), (-1.0)), 4.0, A::exp_scaled_input(s.ad_value(77), -1.0), 4.0, A::mul(s.ad_value(145), s.ad_value(121)), 1.0, 1.0);
        }

        s.b[999] = (s.v[290] < (10.0 * 2.220446049250313e-16));
        s.store_scalar(999, if s.b[999] { 1.0 } else { 0.0 });

        if ((s.b[978] && (!s.b[997])) && s.b[999]) {
            s.store_scalar(290, (10.0 * 2.220446049250313e-16));
        }

        if (s.b[978] && (!s.b[997])) {
            s.store_add_product3_rhs_mixed_iia(319, 404, 145, 120, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(290))), 1.0 / (2.0));
            s.store_mul_add_rhs(77, 120, 319, 416);
            s.store_offset_div_scaled_inputs2(290, A::offset(A::mul(s.ad_value(120), A::add(s.ad_value(404), s.ad_value(416))), (-1.0)), 4.0, A::exp_scaled_input(s.ad_value(77), -1.0), 4.0, A::mul(s.ad_value(145), s.ad_value(121)), 1.0, 1.0);
        }

        s.b[1000] = (s.v[290] < (10.0 * 2.220446049250313e-16));
        s.store_scalar(1000, if s.b[1000] { 1.0 } else { 0.0 });

        if ((s.b[978] && (!s.b[997])) && s.b[1000]) {
            s.store_scalar(290, (10.0 * 2.220446049250313e-16));
        }

        if (s.b[978] && (!s.b[997])) {
            s.store_add_product3_rhs_mixed_iia(319, 404, 145, 120, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(290))), 1.0 / (2.0));
            s.store_mul_add_rhs(77, 120, 319, 416);
        }

        s.b[1001] = (s.v[77] < 3.0);
        s.store_scalar(1001, if s.b[1001] { 1.0 } else { 0.0 });

        if ((s.b[978] && (!s.b[997])) && s.b[1001]) {
            s.store_scalar(421, ((1.0 / (9.0 * 1.414213562373095)) - ((5.0 + (7.0 * 0.049787068367863944)) / (54.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt()))));
            s.store_scalar(422, (((1.0 + 0.049787068367863944) / (2.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt())) - (1.414213562373095 / 3.0)));
            s.store_offset_div_from_scalar_ad(423, 1.0, A::mul(s.ad_value(120), s.ad_value(144)), (1.0 / 1.414213562373095));
            s.store_div_scaled_inputs2_indices(425, 404, -1.0, 416, -1.0, 144, 1.0);
            s.store_add_scaled_inputs3_div_scaled_third_mixed_aaii(426, A::div_scaled_product(A::square(s.ad_value(422)), s.ad_value(422), 1.0, A::mul3_scaled_output(s.ad_value(421), s.ad_value(421), s.ad_value(421), 27.0), 1.0), 1.0, A::div_scaled_product_by_product(s.ad_value(422), s.ad_value(423), 1.0, s.ad_value(421), s.ad_value(421), 6.0), (-1.0), 425, 1.0, 421, 2.0, 1.0);
            s.store_div_ad(424, A::add_scaled_square_product(s.ad_value(422), (-1.0), s.ad_value(421), s.ad_value(423), 3.0), A::mul_scaled_lhs(s.ad_value(421), 9.0, s.ad_value(421)));
            s.store_sqrt_add_scaled_square_cube_product(283, 426, 1.0, 424, 1.0);
            s.store_powf_ad(427, A::sub(s.ad_value(283), s.ad_value(426)), 0.3333333333333333);
            s.store_neg_powf_add_input(428, 426, 283, 0.3333333333333333);
            s.store_add_scaled_inputs3_div_scaled_third_indices(290, 427, 1.0, 428, 1.0, 422, 1.0, 421, 3.0, -1.0);
            s.store_add_scaled_product_indices(319, 416, (-1.0), 290, 122, 1.0);
            s.store_mul_add_rhs(77, 120, 319, 416);
        }

        s.b[1002] = (p.p30 > 0.0);
        s.store_scalar(1002, if s.b[1002] { 1.0 } else { 0.0 });

        if ((s.b[978] && (!s.b[997])) && s.b[1002]) {
            s.store_offset_add(420, 404, 416, 0.1);
            s.store_offset_exp_ad(203, A::mul_scaled_rhs(s.ad_value(120), s.ad_value(416), -1.0), 1e-50);
            s.store_scale(278, 127, 1.0 / (s.v[624]));
            s.store_square(429, 278);
            s.store_mul(430, 429, 203);
            s.store_mul(278, 121, 145);
            s.store_mul(434, 120, 420);
            s.store_add_scaled_inputs_product_mixed_aaii(433, A::ln(A::add_scaled_square_product(s.ad_value(434), 1.0, s.ad_value(430), s.ad_value(278), 1.0)), 1.0, A::ln(A::mul(s.ad_value(429), s.ad_value(278))), (-1.0), 120, 416, 1.0);
            s.store_offset_sub(638, 434, 433, (-1.0));
            s.store_scale(639, 434, 4.0);
        }

        if ((s.b[978] && (!s.b[997])) && s.b[1002]) {
            if (s.v[639] > 0.0) {
            } else {
                s.store_neg(639, 639);
            }
        }

        if ((s.b[978] && (!s.b[997])) && s.b[1002]) {
            s.store_sqrt_square_add(639, 638, 639);
            s.store_offset_scaled_div(279, 638, 639, 0.5, 0.5);
        }

    }
}
