#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_16(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[930] && s.b[931]) {
            s.store_scalar(287, 0.0);
        }

        if s.b[930] {
            s.store_offset(284, 284, 1e-50);
            s.store_add_scaled_ad_rhs(186, 279, s.v[491], A::mul_sub_from_scalar_rhs(A::div(s.ad_value(472), s.ad_value(285)), 1.0, A::sqrt(s.ad_value(284))));
            s.store_add_scaled_inputs3(187, s.ad_value(71), p.p123, s.ad_value(339), 1.0, s.ad_value(186), (-(s.v[487] * s.v[485])));
            s.store_sqrt_square_offset(639, 187, ((4.0 * 0.01) * 0.01));
            s.store_offset_scaled_div(287, 187, 639, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(187, 187, 0.5, 639, 0.5, (1e-10 * 0.01));
        }

        s.b[932] = (s.v[187] < 0.0);
        s.v[932] = if s.b[932] { 1.0 } else { 0.0 };

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
        s.v[933] = if s.b[933] { 1.0 } else { 0.0 };

        if s.b[933] {
            s.store_offset_scaled(278, 80, p.p146, 1.0);
            s.store_scaled_mul(188, 278, 185, p.p145);
            s.store_offset_mul(64, 120, 56, (-1.0));
            s.store_sqrt_square_offset(639, 64, ((4.0 * 0.1) * 0.1));
            s.store_offset_add_scaled_inputs_indices(64, 64, 0.5, 639, 0.5, (1e-10 * 0.1));
        }

        s.b[934] = (s.v[64] < 0.0);
        s.v[934] = if s.b[934] { 1.0 } else { 0.0 };

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
        s.v[935] = if s.b[935] { 1.0 } else { 0.0 };

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

        s.v[949] = (s.v[272] * 100.0);

        s.store_scale(950, 270, 0.0001);

        s.store_scale(951, 123, 100.0);

        s.v[952] = (s.v[466] * 100.0);

        s.store_scale(953, 160, 0.01);

        s.store_scale(954, 383, 0.0001);

        s.store_scale(955, 141, 0.0001);

        s.b[956] = (p.p17 == 0.0);
        s.v[956] = if s.b[956] { 1.0 } else { 0.0 };

        if s.b[956] {
            s.store_scalar(255, 0.0);
            s.store_scalar(250, 0.0);
            s.store_scalar(251, 0.0);
            s.store_scalar(254, 0.0);
            s.store_scalar(256, 0.0);
        }

        s.b[957] = (s.v[34] == 0.0);
        s.v[957] = if s.b[957] { 1.0 } else { 0.0 };

        if ((!s.b[956]) && s.b[957]) {
            s.store_offset_add(948, 74, 71, (-(10.0 * 2.220446049250313e-16)));
            s.store_add_scaled_inputs4(938, s.ad_value(72), 1.0, s.ad_value(138), (-p.p256), A::div_scaled_inputs3(s.ad_value(50), (-p.p258), s.ad_value(80), p.p206, s.ad_value(267), (-p.p206), s.ad_value(951), 1.0), 1.0, s.ad_value(948), (-p.p205));
            s.store_offset_scaled(944, 953, 1.0 / (p.p207), 1.0);
            s.store_scaled_mul(947, 944, 938, 1.0 / (s.v[949]));
            s.store_sqrt_square_offset(639, 947, ((4.0 * 0.01) * 0.01));
            s.store_offset_scaled_div(942, 947, 639, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(947, 947, 0.5, 639, 0.5, (1e-10 * 0.01));
        }

        s.b[958] = (s.v[947] < 0.0);
        s.v[958] = if s.b[958] { 1.0 } else { 0.0 };

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
        s.v[959] = if s.b[959] { 1.0 } else { 0.0 };

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
            s.store_scalar(943, p.p208);
            s.store_div_add_scaled_inputs_rhs_indices(945, 943, 943, 1.0, 71, 1.0);
            s.store_div_from_scalar_offset_ad(941, 1.0, A::square(s.ad_value(947)), 1e-50);
            s.store_scaled_mul(938, 246, 941, (-p.p204));
        }

        s.b[960] = (s.v[938] < (-34.0));
        s.v[960] = if s.b[960] { 1.0 } else { 0.0 };

        if (((!s.b[956]) && s.b[957]) && s.b[960]) {
            s.store_scalar(255, 0.0);
        }

        if (((!s.b[956]) && s.b[957]) && (!s.b[960])) {
            s.store_mul_scale_ad_lhs(940, A::div_from_scalar(p.p203, s.ad_value(245)), 1.6021918e-19, 937);
            s.store_powf_ad(943, A::div_scaled_inputs2(s.ad_value(954), 1.0, s.ad_value(950), 1e-12, s.ad_value(955), 1.0), p.p257);
            s.store_mul_ad_product_lhs(946, A::mul3(A::exp(s.ad_value(938)), s.ad_value(940), s.ad_value(943)), s.ad_value(947), 947);
            s.store_mul3_lhs(255, 944, 945, 946);
        }

        if ((!s.b[956]) && (!s.b[957])) {
            s.store_scalar(255, 0.0);
        }

        if (!s.b[956]) {
            s.store_offset_scaled(937, 52, (-p.p211), p.p212);
            s.store_exp_scaled_input(939, 937, s.v[949]);
            s.store_scale(938, 52, p.p260);
            s.store_scalar(937, ((1.0 / s.v[949]) / s.v[949]));
            s.store_mul_square_lhs(940, 938, 937);
            s.store_scalar(941, (((p.p210 / 1000000.0) * s.v[952]) * ((s.v[375]) as f64).powf(p.p259)));
            s.store_mul3_lhs(250, 941, 939, 940);
        }

        s.b[961] = (s.v[938] >= 0.0);
        s.v[961] = if s.b[961] { 1.0 } else { 0.0 };

        if ((!s.b[956]) && s.b[961]) {
            s.store_scale(250, 250, (-1.0));
        }

        if (!s.b[956]) {
            s.store_sub(942, 52, 51);
            s.store_offset_scaled(937, 942, (-p.p211), p.p212);
            s.store_exp_scaled_input(939, 937, s.v[949]);
            s.store_scale(938, 942, p.p260);
            s.store_scalar(937, ((1.0 / s.v[949]) / s.v[949]));
            s.store_mul_square_lhs(940, 938, 937);
            s.store_scalar(941, (((p.p210 / 1000000.0) * s.v[952]) * ((s.v[375]) as f64).powf(p.p259)));
            s.store_mul3_lhs(251, 941, 939, 940);
        }

        s.b[962] = (s.v[938] >= 0.0);
        s.v[962] = if s.b[962] { 1.0 } else { 0.0 };

        if ((!s.b[956]) && s.b[962]) {
            s.store_scale(251, 251, (-1.0));
        }

        if (!s.b[956]) {
            s.store_scaled_offset_ad(947, A::add_scaled_inputs3(s.ad_value(50), p.p261, s.ad_value(52), (-1.0), s.ad_value(138), 1.0), p.p215, 1.0 / (s.v[949]));
            s.store_sqrt_square_offset(639, 947, ((4.0 * 0.01) * 0.01));
            s.store_offset_scaled_div(942, 947, 639, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(947, 947, 0.5, 639, 0.5, (1e-10 * 0.01));
        }

        s.b[963] = (s.v[947] < 0.0);
        s.v[963] = if s.b[963] { 1.0 } else { 0.0 };

        if ((!s.b[956]) && s.b[963]) {
            s.store_scalar(947, 0.0);
            s.store_scalar(942, 0.0);
        }

        if (!s.b[956]) {
            s.store_offset(947, 947, 1e-50);
            s.store_div_from_scalar_powf_ad(938, (-p.p214), s.ad_value(947), p.p263);
        }

        s.b[964] = (s.v[938] < (-34.0));
        s.v[964] = if s.b[964] { 1.0 } else { 0.0 };

        if ((!s.b[956]) && s.b[964]) {
            s.store_scalar(254, 0.0);
        }

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
            s.store_mul_ad_product_lhs(252, s.ad_value(940), A::powf(s.ad_value(947), p.p262), 939);
            s.store_scaled_offset_ad(947, A::add_scaled_inputs3(s.ad_value(50), p.p269, s.ad_value(52), (-1.0), s.ad_value(138), 1.0), p.p268, 1.0 / (s.v[949]));
            s.store_sqrt_square_offset(639, 947, ((4.0 * 0.01) * 0.01));
            s.store_offset_scaled_div(942, 947, 639, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(947, 947, 0.5, 639, 0.5, (1e-10 * 0.01));
        }

        s.b[965] = (s.v[947] < 0.0);
        s.v[965] = if s.b[965] { 1.0 } else { 0.0 };

        if (((!s.b[956]) && (!s.b[964])) && s.b[965]) {
            s.store_scalar(947, 0.0);
            s.store_scalar(942, 0.0);
        }

        if ((!s.b[956]) && (!s.b[964])) {
            s.store_offset(947, 947, 1e-50);
            s.store_div_from_scalar_powf_ad(938, (-p.p267), s.ad_value(947), p.p271);
        }

        s.b[966] = (s.v[938] < (-34.0));
        s.v[966] = if s.b[966] { 1.0 } else { 0.0 };

        if (((!s.b[956]) && (!s.b[964])) && s.b[966]) {
            s.store_scalar(253, 0.0);
        }

    }

    pub(super) fn stamp_transient_block_17(
        s: &mut Scratch,
        p: &Parameters,
    ) {
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
            s.store_mul_ad_product_lhs(253, s.ad_value(940), A::powf(s.ad_value(947), p.p270), 939);
        }

        if ((!s.b[956]) && (!s.b[964])) {
            s.store_scale(938, 252, (-0.001));
        }

        s.b[967] = (s.v[938] < 1e-50);
        s.v[967] = if s.b[967] { 1.0 } else { 0.0 };

        if (((!s.b[956]) && (!s.b[964])) && s.b[967]) {
            s.store_scalar(938, 1e-50);
        }

        if ((!s.b[956]) && (!s.b[964])) {
            s.store_add_scaled_inputs3(638, s.ad_value(252), -1.0, s.ad_value(253), 1.0, s.ad_value(938), -1.0);
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
            s.store_sub_ad_lhs(254, A::add_scaled_inputs(s.ad_value(638), 0.5, s.ad_value(639), 0.5), 253);
            s.store_neg(254, 254);
        }

        if (!s.b[956]) {
            s.store_scalar(256, 0.5);
        }

        s.b[968] = (p.p18 == 0.0);
        s.v[968] = if s.b[968] { 1.0 } else { 0.0 };

        if s.b[968] {
            s.store_scalar(257, 0.0);
        }

        if (!s.b[968]) {
            s.store_add_scaled_inputs4_offset(279, s.ad_value(51), p.p198, s.ad_value(52), (-1.0), s.ad_value(82), (-p.p200), s.ad_value(266), (-p.p200), (p.p199 * p.p198));
            s.store_scale(247, 279, 1.0 / (p.p228));
            s.store_sqrt_square_offset(639, 247, ((4.0 * 0.01) * 0.01));
            s.store_offset_scaled_div(283, 247, 639, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(248, 247, 0.5, 639, 0.5, (1e-10 * 0.01));
        }

        s.b[969] = (s.v[248] < 0.0);
        s.v[969] = if s.b[969] { 1.0 } else { 0.0 };

        if ((!s.b[968]) && s.b[969]) {
            s.store_scalar(248, 0.0);
            s.store_scalar(283, 0.0);
        }

        if (!s.b[968]) {
            s.store_div_scaled_value_offset_denominator(278, s.ad_value(246), (-s.v[627]), s.ad_value(248), 1e-50, 1.0);
        }

        s.b[970] = (s.v[278] < (-34.0));
        s.v[970] = if s.b[970] { 1.0 } else { 0.0 };

        if ((!s.b[968]) && s.b[970]) {
            s.store_scalar(257, 0.0);
        }

        if ((!s.b[968]) && (!s.b[970])) {
            s.store_scale_ad(280, A::div_from_scalar(s.v[628], s.ad_value(245)), (1.6021918e-19 * s.v[466]));
            s.store_mul_ad(257, A::mul3(s.ad_value(280), s.ad_value(248), s.ad_value(248)), A::exp(s.ad_value(278)));
            s.store_div_scaled_value_offset_denominator(257, s.ad_value(257), 1.0, A::exp(A::mul_scaled_lhs(s.ad_value(120), -1.0, s.ad_value(51))), 1.0, 1.0);
            s.store_div_ad_rhs(257, 257, A::sub_from_scalar(1.0, A::exp(A::div_scaled_inputs(s.ad_value(123), -1.0, s.ad_value(629), 1.0))));
        }

        s.b[971] = (p.p18 == 0.0);
        s.v[971] = if s.b[971] { 1.0 } else { 0.0 };

        if s.b[971] {
            s.store_scalar(258, 0.0);
        }

        if (!s.b[971]) {
            s.store_add_scaled_inputs3(279, A::add_scaled_inputs3_offset(s.ad_value(51), (-p.p198), s.ad_value(52), -1.0, s.ad_value(51), 1.0, ((p.p199) * (p.p198))), 1.0, s.ad_value(82), (-p.p200), s.ad_value(266), (-p.p200));
            s.store_scale(247, 279, 1.0 / (p.p228));
            s.store_sqrt_square_offset(639, 247, ((4.0 * 0.01) * 0.01));
            s.store_offset_scaled_div(283, 247, 639, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(249, 247, 0.5, 639, 0.5, (1e-10 * 0.01));
        }

        s.b[972] = (s.v[249] < 0.0);
        s.v[972] = if s.b[972] { 1.0 } else { 0.0 };

        if ((!s.b[971]) && s.b[972]) {
            s.store_scalar(249, 0.0);
            s.store_scalar(283, 0.0);
        }

        if (!s.b[971]) {
            s.store_div_scaled_value_offset_denominator(278, s.ad_value(246), (-s.v[627]), s.ad_value(249), 1e-50, 1.0);
        }

        s.b[973] = (s.v[278] < (-34.0));
        s.v[973] = if s.b[973] { 1.0 } else { 0.0 };

        if ((!s.b[971]) && s.b[973]) {
            s.store_scalar(258, 0.0);
        }

        if ((!s.b[971]) && (!s.b[973])) {
            s.store_scale_ad(280, A::div_from_scalar(s.v[628], s.ad_value(245)), (1.6021918e-19 * s.v[466]));
            s.store_mul_ad(258, A::mul3(s.ad_value(280), s.ad_value(249), s.ad_value(249)), A::exp(s.ad_value(278)));
            s.store_div_scaled_value_offset_denominator(258, s.ad_value(258), 1.0, A::exp(A::mul(s.ad_value(120), s.ad_value(51))), 1.0, 1.0);
            s.store_div_ad_rhs(258, 258, A::sub_from_scalar(1.0, A::exp(A::div_scaled_inputs(s.ad_value(123), -1.0, s.ad_value(629), 1.0))));
        }

        s.v[264] = p.p176;

        s.v[261] = 0.0;

        s.b[974] = (s.v[34] != 0.0);
        s.v[974] = if s.b[974] { 1.0 } else { 0.0 };

        if s.b[974] {
            s.store_add(280, 51, 56);
            s.store_add_scaled_inputs(260, 280, s.v[264], 57, (1.0 - s.v[264]));
        }

        s.b[975] = (s.v[260] > ((s.v[56] + s.v[51]) - (10.0 * 2.220446049250313e-16)));
        s.v[975] = if s.b[975] { 1.0 } else { 0.0 };

        if (s.b[974] && s.b[975]) {
            s.store_offset_add(260, 56, 51, (-(10.0 * 2.220446049250313e-16)));
        }

        s.b[976] = (p.p45 != 0.0);
        s.v[976] = if s.b[976] { 1.0 } else { 0.0 };

        s.b[977] = (s.v[151] > 1e-15);
        s.v[977] = if s.b[977] { 1.0 } else { 0.0 };

        if (((!s.b[974]) && s.b[976]) && s.b[977]) {
            s.store_div_scaled_product_by_product(261, s.ad_value(151), s.ad_value(122), 1.0, s.ad_value(123), s.ad_value(149), 1.0);
        }

        s.v[435] = s.v[273];

        s.v[436] = (1.0 / s.v[435]);

        s.b[978] = (((p.p19 >= 1.0) && (p.p175 > 0.0)) && (s.v[624] > 0.0));
        s.v[978] = if s.b[978] { 1.0 } else { 0.0 };

        if s.b[978] {
            s.store_scalar(195, p.p175);
            s.store_mul_sqrt_ad_rhs(437, 141, A::div_from_scalar(s.v[624], s.ad_value(457)));
        }

        let (assign18050_e22022,) = {
    if s.b[978] {
        let assign18050_e22018: f64 = (1.0 - -1.0);
        let assign18050_e22020: f64 = (assign18050_e22018 / 2.0);
        (assign18050_e22020,)
    } else {
        (s.v[399],)
    }
};
        s.v[399] = assign18050_e22022;

        let (assign18060_e22030,) = {
    if s.b[978] {
        let assign18060_e22026: f64 = (1.0 + -1.0);
        let assign18060_e22028: f64 = (assign18060_e22026 / 2.0);
        (assign18060_e22028,)
    } else {
        (s.v[400],)
    }
};
        s.v[400] = assign18060_e22030;

        let (assign18070_e22040,) = {
    if s.b[978] {
        let assign18070_e22034: f64 = (s.v[399] * s.v[412]);
        let assign18070_e22037: f64 = (s.v[400] * s.v[413]);
        let assign18070_e22038: f64 = (assign18070_e22034 + assign18070_e22037);
        (assign18070_e22038,)
    } else {
        (s.v[402],)
    }
};
        s.v[402] = assign18070_e22040;

        let (assign18080_e22050,) = {
    if s.b[978] {
        let assign18080_e22044: f64 = (s.v[399] * s.v[413]);
        let assign18080_e22047: f64 = (s.v[400] * s.v[412]);
        let assign18080_e22048: f64 = (assign18080_e22044 + assign18080_e22047);
        (assign18080_e22048,)
    } else {
        (s.v[403],)
    }
};
        s.v[403] = assign18080_e22050;

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
        s.v[979] = if s.b[979] { 1.0 } else { 0.0 };

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
            s.store_mul_ad(417, A::div_from_scalar(2.0, s.ad_value(120)), A::ln(A::div_from_scalar(s.v[624], s.ad_value(127))));
        }

        let (assign18310_e22251,) = {
    if s.b[978] {
        let assign18310_e22249: f64 = (-s.v[416]);
        (assign18310_e22249,)
    } else {
        (s.v[419],)
    }
};
        s.v[419] = assign18310_e22251;

        s.b[980] = (s.v[404] < s.v[419]);
        s.v[980] = if s.b[980] { 1.0 } else { 0.0 };

        if (s.b[978] && s.b[980]) {
            s.store_div_from_scalar_mul_ad(291, s.v[435], s.ad_value(120), s.ad_value(437));
            s.store_offset_scaled(184, 291, (3.0 * 1.414213562373095), 2.0);
            s.store_mul3_affine_lhs(182, 184, 184, 8.0, 0.0, 184);
            s.store_sub(176, 137, 417);
            s.store_mul_add_rhs(290, 120, 404, 416);
            s.store_sub_from_scalar_ad(183, (7.0 * 1.414213562373095), A::mul_scaled_lhs(s.ad_value(291), 9.0, A::offset(s.ad_value(290), (-2.0))));
            s.store_square(181, 183);
        }

        s.b[981] = (s.v[182] < (s.v[181] * 1e-8));
        s.v[981] = if s.b[981] { 1.0 } else { 0.0 };

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
            s.store_sub_ad_lhs(410, A::div(s.ad_value(279), A::sqrt(A::offset(A::square(s.ad_value(280)), 1.0))), 416);
            s.store_scaled_sub(408, 404, 410, s.v[435]);
            s.copy_ad(407, 408);
        }

        if (s.b[978] && (!s.b[980])) {
            s.store_scalar(77, 3.0);
            s.store_sub_ad_lhs(319, A::div(s.ad_value(77), s.ad_value(120)), 416);
            s.store_offset_div_scaled_inputs2(290, A::offset(A::mul(s.ad_value(120), A::add(s.ad_value(404), s.ad_value(416))), (-1.0)), 4.0, A::exp_scaled_input(s.ad_value(77), -1.0), 4.0, A::mul(s.ad_value(145), s.ad_value(121)), 1.0, 1.0);
        }

        s.b[982] = (s.v[290] < (10.0 * 2.220446049250313e-16));
        s.v[982] = if s.b[982] { 1.0 } else { 0.0 };

        if ((s.b[978] && (!s.b[980])) && s.b[982]) {
            s.store_scalar(290, (10.0 * 2.220446049250313e-16));
        }

    }

    pub(super) fn stamp_transient_block_18(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[978] && (!s.b[980])) {
            s.store_add_ad_rhs(319, 404, A::mul3_scaled_output(s.ad_value(145), s.ad_value(120), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(290))), 1.0 / (2.0)));
            s.store_mul_add_rhs(77, 120, 319, 416);
            s.store_offset_div_scaled_inputs2(290, A::offset(A::mul(s.ad_value(120), A::add(s.ad_value(404), s.ad_value(416))), (-1.0)), 4.0, A::exp_scaled_input(s.ad_value(77), -1.0), 4.0, A::mul(s.ad_value(145), s.ad_value(121)), 1.0, 1.0);
        }

        s.b[983] = (s.v[290] < (10.0 * 2.220446049250313e-16));
        s.v[983] = if s.b[983] { 1.0 } else { 0.0 };

        if ((s.b[978] && (!s.b[980])) && s.b[983]) {
            s.store_scalar(290, (10.0 * 2.220446049250313e-16));
        }

        if (s.b[978] && (!s.b[980])) {
            s.store_add_ad_rhs(319, 404, A::mul3_scaled_output(s.ad_value(145), s.ad_value(120), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(290))), 1.0 / (2.0)));
            s.store_mul_add_rhs(77, 120, 319, 416);
        }

        s.b[984] = (s.v[77] < 3.0);
        s.v[984] = if s.b[984] { 1.0 } else { 0.0 };

        if ((s.b[978] && (!s.b[980])) && s.b[984]) {
            s.store_scalar(421, ((1.0 / (9.0 * 1.414213562373095)) - ((5.0 + (7.0 * 0.049787068367863944)) / (54.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt()))));
            s.store_scalar(422, (((1.0 + 0.049787068367863944) / (2.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt())) - (1.414213562373095 / 3.0)));
            s.store_offset_div_from_scalar_ad(423, 1.0, A::mul(s.ad_value(120), s.ad_value(144)), (1.0 / 1.414213562373095));
            s.store_div_scaled_inputs2(425, s.ad_value(404), -1.0, s.ad_value(416), -1.0, s.ad_value(144), 1.0);
            s.store_add_scaled_inputs3(426, A::div_scaled_product(A::square(s.ad_value(422)), s.ad_value(422), 1.0, A::mul3_scaled_output(s.ad_value(421), s.ad_value(421), s.ad_value(421), 27.0), 1.0), 1.0, A::div_scaled_product_by_product(s.ad_value(422), s.ad_value(423), 1.0, s.ad_value(421), s.ad_value(421), 6.0), (-1.0), A::div_scaled_inputs(s.ad_value(425), 1.0, s.ad_value(421), 2.0), 1.0);
            s.store_div_ad(424, A::add_scaled_square_product(s.ad_value(422), (-1.0), s.ad_value(421), s.ad_value(423), 3.0), A::mul_scaled_lhs(s.ad_value(421), 9.0, s.ad_value(421)));
            s.store_sqrt_ad(283, A::add_scaled_square_product(s.ad_value(426), 1.0, A::square(s.ad_value(424)), s.ad_value(424), 1.0));
            s.store_powf_ad(427, A::sub(s.ad_value(283), s.ad_value(426)), 0.3333333333333333);
            s.store_neg_ad(428, A::powf(A::add(s.ad_value(426), s.ad_value(283)), 0.3333333333333333));
            s.store_add_scaled_inputs3(290, s.ad_value(427), 1.0, s.ad_value(428), 1.0, A::div_scaled_inputs(s.ad_value(422), 1.0, s.ad_value(421), 3.0), -1.0);
            s.store_add_scaled_product_indices(319, 416, (-1.0), 290, 122, 1.0);
            s.store_mul_add_rhs(77, 120, 319, 416);
        }

        s.b[985] = (p.p30 > 0.0);
        s.v[985] = if s.b[985] { 1.0 } else { 0.0 };

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
            s.store_add_scaled_inputs3(433, s.ad_value(434), 1.0, s.ad_value(638), (-0.5), s.ad_value(639), (-0.5));
            s.store_sub(434, 434, 433);
            s.store_add_scaled_inputs(434, 434, 1.0, 120, 0.1);
            s.store_add_scaled_inputs_product_mixed_aaii(432, A::ln(A::add_scaled_square_product(s.ad_value(434), 1.0, s.ad_value(430), s.ad_value(278), 1.0)), 1.0, A::ln(A::mul(s.ad_value(429), s.ad_value(278))), (-1.0), 120, 416, 1.0);
            s.store_sub_ad_lhs(320, A::div(s.ad_value(432), s.ad_value(120)), 416);
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
            s.store_add_scaled_inputs3(77, s.ad_value(432), 1.0, s.ad_value(638), (-0.5), s.ad_value(639), (-0.5));
        }

        if (s.b[978] && (!s.b[980])) {
            s.store_sub_ad_lhs(410, A::div(s.ad_value(77), s.ad_value(120)), 416);
            s.store_add_ad(279, A::offset(s.ad_value(77), (-1.0)), A::exp_scaled_input(s.ad_value(77), -1.0));
        }

        s.b[986] = (s.v[279] < (10.0 * 2.220446049250313e-16));
        s.v[986] = if s.b[986] { 1.0 } else { 0.0 };

        if ((s.b[978] && (!s.b[980])) && s.b[986]) {
            s.store_scalar(279, (10.0 * 2.220446049250313e-16));
        }

        if (s.b[978] && (!s.b[980])) {
            s.store_mul_sqrt_rhs(407, 437, 279);
            s.store_scaled_sub(408, 404, 410, s.v[435]);
        }

        s.b[987] = (p.p30 == 1.0);
        s.v[987] = if s.b[987] { 1.0 } else { 0.0 };

        if ((s.b[978] && (!s.b[980])) && s.b[987]) {
            s.store_exp_ad(203, A::mul_scaled_rhs(s.ad_value(120), s.ad_value(416), -1.0));
            s.store_scale(278, 127, 1.0 / (s.v[624]));
            s.store_square(429, 278);
            s.store_mul(204, 429, 203);
        }

        let (assign19210_e23430,) = {
    if ((s.b[978] && (!s.b[980])) && s.b[987]) {
        (0.0,)
    } else {
        (s.v[379],)
    }
};
        s.v[379] = assign19210_e23430;

        if ((s.b[978] && (!s.b[980])) && s.b[987]) {
            s.store_scalar(62, 1.0);
        }

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
            s.v[988] = if s.b[988] { 1.0 } else { 0.0 };
            if (((s.b[978] && (!s.b[980])) && s.b[987]) && s.b[988]) {
                s.store_mul3_ad_middle(205, A::square(s.ad_value(77)), 77, A::offset(A::mul(s.ad_value(77), A::scale_offset(s.ad_value(77), 0.006115288895133179, (-0.07053654284009761))), 0.29693154855771));
                s.store_mul_offset_rhs_ad(206, A::square(s.ad_value(77)), A::mul(s.ad_value(77), A::scale_offset(s.ad_value(77), (5.0 * 0.006115288895133179), (4.0 * (-0.07053654284009761)))), (3.0 * 0.29693154855771));
                s.store_mul3_lhs(207, 204, 205, 205);
                s.store_mul_product3_rhs(208, 206, s.ad_value(204), s.ad_value(120), s.ad_value(205), 2.0);
                s.store_mul_offset_ad_rhs(146, 77, A::mul_offset_rhs(s.ad_value(77), A::mul_offset_rhs(s.ad_value(77), A::mul(s.ad_value(77), A::scale_offset(s.ad_value(77), 6.36964918866352e-5, (-0.00163730162779191))), 0.0178800506338833), (-0.117851130197758)), 0.707106781186548);
                s.store_offset_mul_offset_rhs_ad_rhs(148, 77, A::mul_offset_rhs(s.ad_value(77), A::mul(s.ad_value(77), A::scale_offset(s.ad_value(77), (5.0 * 6.36964918866352e-5), (4.0 * (-0.00163730162779191)))), (3.0 * 0.0178800506338833)), (2.0 * (-0.117851130197758)), 0.707106781186548);
                s.store_sqrt_offset_ad(209, A::add(A::square(s.ad_value(146)), s.ad_value(207)), 1e-50);
                s.store_div_scaled_inputs2(210, A::mul3_scaled_output(s.ad_value(120), s.ad_value(148), s.ad_value(146), 2.0), 1.0, s.ad_value(208), 1.0, s.ad_value(209), 2.0);
            }
            s.b[989] = (s.v[77] < 80.0);
            s.v[989] = if s.b[989] { 1.0 } else { 0.0 };
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
                s.store_sub_from_scalar_ad(212, (-1.0), A::mul(s.ad_value(144), s.ad_value(210)));
            }
            s.b[990] = (s.v[379] == 1.0);
            s.v[990] = if s.b[990] { 1.0 } else { 0.0 };
            if (((s.b[978] && (!s.b[980])) && s.b[987]) && s.b[990]) {
                s.store_scalar(62, (40.0 + 1.0));
            }
            if (((s.b[978] && (!s.b[980])) && s.b[987]) && (!s.b[990])) {
                s.store_div_scaled_inputs(213, s.ad_value(211), -1.0, s.ad_value(212), 1.0);
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
            s.v[991] = if s.b[991] { 1.0 } else { 0.0 };
            if ((((s.b[978] && (!s.b[980])) && s.b[987]) && (!s.b[990])) && s.b[991]) {
                s.store_scale(213, 214, (if (s.v[213] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if (((s.b[978] && (!s.b[980])) && s.b[987]) && (!s.b[990])) {
                s.store_add(410, 410, 213);
            }
            s.b[992] = ((((s.v[213]) as f64).abs() <= 1e-12) && (((s.v[211]) as f64).abs() <= 1e-8));
            s.v[992] = if s.b[992] { 1.0 } else { 0.0 };
            let (assign19230_body29_e23959,) = {
    if ((((s.b[978] && (!s.b[980])) && s.b[987]) && (!s.b[990])) && s.b[992]) {
        (1.0,)
    } else {
        (s.v[379],)
    }
};
            s.v[379] = assign19230_body29_e23959;
            if ((s.b[978] && (!s.b[980])) && s.b[987]) {
                s.store_offset(62, 62, 1.0);
            }
        }

        s.b[994] = (s.v[77] < 5.0);
        s.v[994] = if s.b[994] { 1.0 } else { 0.0 };

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

        let (assign19440_e24177,) = {
    if s.b[978] {
        let assign19440_e24173: f64 = (1.0 - 1.0);
        let assign19440_e24175: f64 = (assign19440_e24173 / 2.0);
        (assign19440_e24175,)
    } else {
        (s.v[399],)
    }
};
        s.v[399] = assign19440_e24177;

        let (assign19450_e24185,) = {
    if s.b[978] {
        let assign19450_e24181: f64 = (1.0 + 1.0);
        let assign19450_e24183: f64 = (assign19450_e24181 / 2.0);
        (assign19450_e24183,)
    } else {
        (s.v[400],)
    }
};
        s.v[400] = assign19450_e24185;

        let (assign19460_e24195,) = {
    if s.b[978] {
        let assign19460_e24189: f64 = (s.v[399] * s.v[412]);
        let assign19460_e24192: f64 = (s.v[400] * s.v[413]);
        let assign19460_e24193: f64 = (assign19460_e24189 + assign19460_e24192);
        (assign19460_e24193,)
    } else {
        (s.v[402],)
    }
};
        s.v[402] = assign19460_e24195;

        let (assign19470_e24205,) = {
    if s.b[978] {
        let assign19470_e24199: f64 = (s.v[399] * s.v[413]);
        let assign19470_e24202: f64 = (s.v[400] * s.v[412]);
        let assign19470_e24203: f64 = (assign19470_e24199 + assign19470_e24202);
        (assign19470_e24203,)
    } else {
        (s.v[403],)
    }
};
        s.v[403] = assign19470_e24205;

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
        s.v[996] = if s.b[996] { 1.0 } else { 0.0 };

        if (s.b[978] && s.b[996]) {
            s.store_sub(279, 278, 31);
            s.store_sub_from_scalar(280, s.v[30], 31);
            s.store_div(638, 279, 280);
            s.store_square(639, 638);
            s.store_mul(640, 639, 638);
        }

    }

    pub(super) fn stamp_transient_block_19(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[978] && s.b[996]) {
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
            s.store_mul_ad(417, A::div_from_scalar(2.0, s.ad_value(120)), A::ln(A::div_from_scalar(s.v[624], s.ad_value(127))));
        }

        let (assign19700_e24406,) = {
    if s.b[978] {
        let assign19700_e24404: f64 = (-s.v[416]);
        (assign19700_e24404,)
    } else {
        (s.v[419],)
    }
};
        s.v[419] = assign19700_e24406;

        s.b[997] = (s.v[404] < s.v[419]);
        s.v[997] = if s.b[997] { 1.0 } else { 0.0 };

        if (s.b[978] && s.b[997]) {
            s.store_div_from_scalar_mul_ad(291, s.v[435], s.ad_value(120), s.ad_value(437));
            s.store_offset_scaled(184, 291, (3.0 * 1.414213562373095), 2.0);
            s.store_mul3_affine_lhs(182, 184, 184, 8.0, 0.0, 184);
            s.store_sub(176, 137, 417);
            s.store_mul_add_rhs(290, 120, 404, 416);
            s.store_sub_from_scalar_ad(183, (7.0 * 1.414213562373095), A::mul_scaled_lhs(s.ad_value(291), 9.0, A::offset(s.ad_value(290), (-2.0))));
            s.store_square(181, 183);
        }

        s.b[998] = (s.v[182] < (s.v[181] * 1e-8));
        s.v[998] = if s.b[998] { 1.0 } else { 0.0 };

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
            s.store_sub_ad_lhs(410, A::div(s.ad_value(279), A::sqrt(A::offset(A::square(s.ad_value(280)), 1.0))), 416);
            s.store_scaled_sub(408, 404, 410, s.v[435]);
            s.copy_ad(407, 408);
        }

        if (s.b[978] && (!s.b[997])) {
            s.store_scalar(77, 3.0);
            s.store_sub_ad_lhs(319, A::div(s.ad_value(77), s.ad_value(120)), 416);
            s.store_offset_div_scaled_inputs2(290, A::offset(A::mul(s.ad_value(120), A::add(s.ad_value(404), s.ad_value(416))), (-1.0)), 4.0, A::exp_scaled_input(s.ad_value(77), -1.0), 4.0, A::mul(s.ad_value(145), s.ad_value(121)), 1.0, 1.0);
        }

        s.b[999] = (s.v[290] < (10.0 * 2.220446049250313e-16));
        s.v[999] = if s.b[999] { 1.0 } else { 0.0 };

        if ((s.b[978] && (!s.b[997])) && s.b[999]) {
            s.store_scalar(290, (10.0 * 2.220446049250313e-16));
        }

        if (s.b[978] && (!s.b[997])) {
            s.store_add_ad_rhs(319, 404, A::mul3_scaled_output(s.ad_value(145), s.ad_value(120), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(290))), 1.0 / (2.0)));
            s.store_mul_add_rhs(77, 120, 319, 416);
            s.store_offset_div_scaled_inputs2(290, A::offset(A::mul(s.ad_value(120), A::add(s.ad_value(404), s.ad_value(416))), (-1.0)), 4.0, A::exp_scaled_input(s.ad_value(77), -1.0), 4.0, A::mul(s.ad_value(145), s.ad_value(121)), 1.0, 1.0);
        }

        s.b[1000] = (s.v[290] < (10.0 * 2.220446049250313e-16));
        s.v[1000] = if s.b[1000] { 1.0 } else { 0.0 };

        if ((s.b[978] && (!s.b[997])) && s.b[1000]) {
            s.store_scalar(290, (10.0 * 2.220446049250313e-16));
        }

        if (s.b[978] && (!s.b[997])) {
            s.store_add_ad_rhs(319, 404, A::mul3_scaled_output(s.ad_value(145), s.ad_value(120), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(290))), 1.0 / (2.0)));
            s.store_mul_add_rhs(77, 120, 319, 416);
        }

        s.b[1001] = (s.v[77] < 3.0);
        s.v[1001] = if s.b[1001] { 1.0 } else { 0.0 };

        if ((s.b[978] && (!s.b[997])) && s.b[1001]) {
            s.store_scalar(421, ((1.0 / (9.0 * 1.414213562373095)) - ((5.0 + (7.0 * 0.049787068367863944)) / (54.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt()))));
            s.store_scalar(422, (((1.0 + 0.049787068367863944) / (2.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt())) - (1.414213562373095 / 3.0)));
            s.store_offset_div_from_scalar_ad(423, 1.0, A::mul(s.ad_value(120), s.ad_value(144)), (1.0 / 1.414213562373095));
            s.store_div_scaled_inputs2(425, s.ad_value(404), -1.0, s.ad_value(416), -1.0, s.ad_value(144), 1.0);
            s.store_add_scaled_inputs3(426, A::div_scaled_product(A::square(s.ad_value(422)), s.ad_value(422), 1.0, A::mul3_scaled_output(s.ad_value(421), s.ad_value(421), s.ad_value(421), 27.0), 1.0), 1.0, A::div_scaled_product_by_product(s.ad_value(422), s.ad_value(423), 1.0, s.ad_value(421), s.ad_value(421), 6.0), (-1.0), A::div_scaled_inputs(s.ad_value(425), 1.0, s.ad_value(421), 2.0), 1.0);
            s.store_div_ad(424, A::add_scaled_square_product(s.ad_value(422), (-1.0), s.ad_value(421), s.ad_value(423), 3.0), A::mul_scaled_lhs(s.ad_value(421), 9.0, s.ad_value(421)));
            s.store_sqrt_ad(283, A::add_scaled_square_product(s.ad_value(426), 1.0, A::square(s.ad_value(424)), s.ad_value(424), 1.0));
            s.store_powf_ad(427, A::sub(s.ad_value(283), s.ad_value(426)), 0.3333333333333333);
            s.store_neg_ad(428, A::powf(A::add(s.ad_value(426), s.ad_value(283)), 0.3333333333333333));
            s.store_add_scaled_inputs3(290, s.ad_value(427), 1.0, s.ad_value(428), 1.0, A::div_scaled_inputs(s.ad_value(422), 1.0, s.ad_value(421), 3.0), -1.0);
            s.store_add_scaled_product_indices(319, 416, (-1.0), 290, 122, 1.0);
            s.store_mul_add_rhs(77, 120, 319, 416);
        }

        s.b[1002] = (p.p30 > 0.0);
        s.v[1002] = if s.b[1002] { 1.0 } else { 0.0 };

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
            s.store_offset_scaled_ad(280, A::div_scaled_offset_numerator(s.ad_value(638), 1.0, 2.0, s.ad_value(639), 1.0), (-0.5), 0.5);
            s.store_add_scaled_inputs3(433, s.ad_value(434), 1.0, s.ad_value(638), (-0.5), s.ad_value(639), (-0.5));
            s.store_sub(434, 434, 433);
            s.store_add_scaled_inputs(434, 434, 1.0, 120, 0.1);
            s.store_add_scaled_inputs_product_mixed_aaii(432, A::ln(A::add_scaled_square_product(s.ad_value(434), 1.0, s.ad_value(430), s.ad_value(278), 1.0)), 1.0, A::ln(A::mul(s.ad_value(429), s.ad_value(278))), (-1.0), 120, 416, 1.0);
            s.store_sub_ad_lhs(320, A::div(s.ad_value(432), s.ad_value(120)), 416);
            s.copy_ad(431, 77);
            s.store_offset_sub(638, 432, 431, (-(0.0008 * 75.0)));
            s.store_scale(639, 432, (4.0 * (0.0008 * 75.0)));
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
            s.store_offset_scaled_ad(280, A::div_scaled_offset_numerator(s.ad_value(638), 1.0, ((2.0 * 0.0008) * 75.0), s.ad_value(639), 1.0), (-0.5), 0.5);
            s.store_add_scaled_inputs3(77, s.ad_value(432), 1.0, s.ad_value(638), (-0.5), s.ad_value(639), (-0.5));
        }

        if (s.b[978] && (!s.b[997])) {
            s.store_sub_ad_lhs(410, A::div(s.ad_value(77), s.ad_value(120)), 416);
            s.store_add_ad(279, A::offset(s.ad_value(77), (-1.0)), A::exp_scaled_input(s.ad_value(77), -1.0));
        }

        s.b[1003] = (s.v[279] < (10.0 * 2.220446049250313e-16));
        s.v[1003] = if s.b[1003] { 1.0 } else { 0.0 };

        if ((s.b[978] && (!s.b[997])) && s.b[1003]) {
            s.store_scalar(279, (10.0 * 2.220446049250313e-16));
        }

        if (s.b[978] && (!s.b[997])) {
            s.store_mul_sqrt_rhs(407, 437, 279);
            s.store_scaled_sub(408, 404, 410, s.v[435]);
        }

        s.b[1004] = (p.p30 == 1.0);
        s.v[1004] = if s.b[1004] { 1.0 } else { 0.0 };

        if ((s.b[978] && (!s.b[997])) && s.b[1004]) {
            s.store_exp_ad(203, A::mul_scaled_rhs(s.ad_value(120), s.ad_value(416), -1.0));
            s.store_scale(278, 127, 1.0 / (s.v[624]));
            s.store_square(429, 278);
            s.store_mul(204, 429, 203);
        }

        let (assign20600_e25585,) = {
    if ((s.b[978] && (!s.b[997])) && s.b[1004]) {
        (0.0,)
    } else {
        (s.v[379],)
    }
};
        s.v[379] = assign20600_e25585;

        if ((s.b[978] && (!s.b[997])) && s.b[1004]) {
            s.store_scalar(62, 1.0);
        }

    }

    pub(super) fn stamp_transient_block_20(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        let mut assign20620_loop_guard: usize = 0;
        while {
            let assign20620_cond_e25604: f64 = (40.0 + 1.0);
            let assign20620_cond_e25606: f64 = if (((s.b[978] && (!s.b[997])) && s.b[1004]) && (s.v[62] <= assign20620_cond_e25604)) { 1.0 } else { 0.0 };
            assign20620_cond_e25606 != 0.0
        } {
            assign20620_loop_guard += 1;
            assert!(assign20620_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.b[978] && (!s.b[997])) && s.b[1004]) {
                s.store_mul_add_rhs(77, 120, 410, 416);
            }
            s.b[1005] = (s.v[77] < 5.0);
            s.v[1005] = if s.b[1005] { 1.0 } else { 0.0 };
            if (((s.b[978] && (!s.b[997])) && s.b[1004]) && s.b[1005]) {
                s.store_mul3_ad_middle(205, A::square(s.ad_value(77)), 77, A::offset(A::mul(s.ad_value(77), A::scale_offset(s.ad_value(77), 0.006115288895133179, (-0.07053654284009761))), 0.29693154855771));
                s.store_mul_offset_rhs_ad(206, A::square(s.ad_value(77)), A::mul(s.ad_value(77), A::scale_offset(s.ad_value(77), (5.0 * 0.006115288895133179), (4.0 * (-0.07053654284009761)))), (3.0 * 0.29693154855771));
                s.store_mul3_lhs(207, 204, 205, 205);
                s.store_mul_product3_rhs(208, 206, s.ad_value(204), s.ad_value(120), s.ad_value(205), 2.0);
                s.store_mul_offset_ad_rhs(146, 77, A::mul_offset_rhs(s.ad_value(77), A::mul_offset_rhs(s.ad_value(77), A::mul(s.ad_value(77), A::scale_offset(s.ad_value(77), 6.36964918866352e-5, (-0.00163730162779191))), 0.0178800506338833), (-0.117851130197758)), 0.707106781186548);
                s.store_offset_mul_offset_rhs_ad_rhs(148, 77, A::mul_offset_rhs(s.ad_value(77), A::mul(s.ad_value(77), A::scale_offset(s.ad_value(77), (5.0 * 6.36964918866352e-5), (4.0 * (-0.00163730162779191)))), (3.0 * 0.0178800506338833)), (2.0 * (-0.117851130197758)), 0.707106781186548);
                s.store_sqrt_offset_ad(209, A::add(A::square(s.ad_value(146)), s.ad_value(207)), 1e-50);
                s.store_div_scaled_inputs2(210, A::mul3_scaled_output(s.ad_value(120), s.ad_value(148), s.ad_value(146), 2.0), 1.0, s.ad_value(208), 1.0, s.ad_value(209), 2.0);
            }
            s.b[1006] = (s.v[77] < 80.0);
            s.v[1006] = if s.b[1006] { 1.0 } else { 0.0 };
            if ((((s.b[978] && (!s.b[997])) && s.b[1004]) && (!s.b[1005])) && s.b[1006]) {
                s.store_exp(147, 77);
                s.store_mul_offset_rhs(207, 204, 147, (-1.0));
                s.store_mul3_lhs(208, 204, 120, 147);
            }
            if ((((s.b[978] && (!s.b[997])) && s.b[1004]) && (!s.b[1005])) && (!s.b[1006])) {
                s.store_exp_mul(202, 120, 410);
                s.store_mul_sub_rhs(207, 429, 202, 203);
                s.store_mul3_lhs(208, 429, 120, 202);
            }
            if (((s.b[978] && (!s.b[997])) && s.b[1004]) && (!s.b[1005])) {
                s.store_sqrt_add_ad(209, A::offset(s.ad_value(77), (-1.0)), s.ad_value(207));
                s.store_scale_ad(210, A::div_scaled_inputs2(s.ad_value(120), 1.0, s.ad_value(208), 1.0, s.ad_value(209), 1.0), 0.5);
            }
            if ((s.b[978] && (!s.b[997])) && s.b[1004]) {
                s.store_add_scaled_inputs_product_indices(211, 404, 1.0, 410, (-1.0), 144, 209, (-1.0));
                s.store_sub_from_scalar_ad(212, (-1.0), A::mul(s.ad_value(144), s.ad_value(210)));
            }
            s.b[1007] = (s.v[379] == 1.0);
            s.v[1007] = if s.b[1007] { 1.0 } else { 0.0 };
            if (((s.b[978] && (!s.b[997])) && s.b[1004]) && s.b[1007]) {
                s.store_scalar(62, (40.0 + 1.0));
            }
            if (((s.b[978] && (!s.b[997])) && s.b[1004]) && (!s.b[1007])) {
                s.store_div_scaled_inputs(213, s.ad_value(211), -1.0, s.ad_value(212), 1.0);
            }
            if (((s.b[978] && (!s.b[997])) && s.b[1004]) && (!s.b[1007])) {
                s.store_scaled_offset_ad(214, {
                    if (1.0 >= ((s.v[410]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(410))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[1008] = (((s.v[213]) as f64).abs() > s.v[214]);
            s.v[1008] = if s.b[1008] { 1.0 } else { 0.0 };
            if ((((s.b[978] && (!s.b[997])) && s.b[1004]) && (!s.b[1007])) && s.b[1008]) {
                s.store_scale(213, 214, (if (s.v[213] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if (((s.b[978] && (!s.b[997])) && s.b[1004]) && (!s.b[1007])) {
                s.store_add(410, 410, 213);
            }
            s.b[1009] = ((((s.v[213]) as f64).abs() <= 1e-12) && (((s.v[211]) as f64).abs() <= 1e-8));
            s.v[1009] = if s.b[1009] { 1.0 } else { 0.0 };
            let (assign20620_body29_e26114,) = {
    if ((((s.b[978] && (!s.b[997])) && s.b[1004]) && (!s.b[1007])) && s.b[1009]) {
        (1.0,)
    } else {
        (s.v[379],)
    }
};
            s.v[379] = assign20620_body29_e26114;
            if ((s.b[978] && (!s.b[997])) && s.b[1004]) {
                s.store_offset(62, 62, 1.0);
            }
        }

        s.b[1011] = (s.v[77] < 5.0);
        s.v[1011] = if s.b[1011] { 1.0 } else { 0.0 };

        if (((s.b[978] && (!s.b[997])) && s.b[1004]) && s.b[1011]) {
            s.store_offset_square(64, 146, (10.0 * 2.220446049250313e-16));
            s.store_offset(65, 146, (10.0 * 2.220446049250313e-16));
        }

        if (((s.b[978] && (!s.b[997])) && s.b[1004]) && (!s.b[1011])) {
            s.store_offset(64, 77, (-1.0));
            s.store_sqrt(65, 64);
        }

        if ((s.b[978] && (!s.b[997])) && s.b[1004]) {
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

        let (assign20830_e26334,) = {
    if s.b[978] {
        let assign20830_e26328: f64 = (s.v[413] * s.v[519]);
        let assign20830_e26331: f64 = (s.v[412] * s.v[518]);
        let assign20830_e26332: f64 = (assign20830_e26328 + assign20830_e26331);
        (assign20830_e26332,)
    } else {
        (s.v[194],)
    }
};
        s.v[194] = assign20830_e26334;

        if (s.b[978] && (s.v[194] != 0.0)) {
            s.store_add_scaled_inputs(198, 413, p.p174, 412, p.p173);
            s.store_scale(198, 198, (-s.v[513]));
            s.store_offset_ad(197, A::mul_scaled_lhs(s.ad_value(198), -1.0, A::sub(s.ad_value(52), s.ad_value(51))), s.v[197]);
        }

        let (assign20870_e26378,) = {
    if s.b[978] {
        let assign20870_e26372: f64 = (s.v[412] * s.v[519]);
        let assign20870_e26375: f64 = (s.v[413] * s.v[518]);
        let assign20870_e26376: f64 = (assign20870_e26372 + assign20870_e26375);
        (assign20870_e26376,)
    } else {
        (s.v[194],)
    }
};
        s.v[194] = assign20870_e26378;

        if (s.b[978] && (s.v[194] != 0.0)) {
            s.store_add_scaled_inputs(199, 412, p.p174, 413, p.p173);
            s.store_scale(199, 199, (-s.v[513]));
            s.store_offset_scaled_mul(196, 199, 52, -1.0, s.v[196]);
        }

        s.b[1013] = (((s.v[575] == 1.0) && (!s.b[518])) || ((s.v[575] != 1.0) && (!s.b[519])));
        s.v[1013] = if s.b[1013] { 1.0 } else { 0.0 };

        s.b[1014] = (p.p175 > 0.0);
        s.v[1014] = if s.b[1014] { 1.0 } else { 0.0 };

        if (((!s.b[978]) && s.b[1013]) && s.b[1014]) {
            s.store_scalar(198, (((-s.v[435]) * p.p175) * s.v[513]));
        }

        if (((!s.b[978]) && s.b[1013]) && (!s.b[1014])) {
            s.store_scalar(198, 0.0);
        }

        if ((!s.b[978]) && (!s.b[1013])) {
            s.store_add_scaled_inputs(198, 413, p.p174, 412, p.p173);
            s.store_scale(198, 198, (-s.v[513]));
        }

        if (!s.b[978]) {
            s.store_mul_sub_scaled_inputs_rhs(197, 198, s.ad_value(52), -1.0, s.ad_value(51), -1.0);
        }

        s.b[1015] = (((s.v[575] == 1.0) && (!s.b[519])) || ((s.v[575] != 1.0) && (!s.b[518])));
        s.v[1015] = if s.b[1015] { 1.0 } else { 0.0 };

        if ((!s.b[978]) && s.b[1015]) {
            s.store_scalar(199, (((-s.v[435]) * p.p175) * s.v[513]));
        }

        if ((!s.b[978]) && (!s.b[1015])) {
            s.store_add_scaled_inputs(199, 412, p.p174, 413, p.p173);
            s.store_scale(199, 199, (-s.v[513]));
        }

        if (!s.b[978]) {
            s.store_mul_neg_lhs(196, 199, 52);
        }

        s.b[1016] = (s.v[34] == 0.0);
        s.v[1016] = if s.b[1016] { 1.0 } else { 0.0 };

        if ((s.v[38] != 0.0) && s.b[1016]) {
            s.store_scaled_mul(279, 386, 386, (p.p223 * p.p224));
            s.store_offset_ad(280, A::add_scaled_products(s.ad_value(158), s.ad_value(86), p.p223, s.ad_value(386), s.ad_value(386), p.p224), 1e-50);
            s.store_div(221, 279, 280);
        }

        if ((s.v[38] != 0.0) && (!s.b[1016])) {
            s.store_scalar(221, (p.p223 + 1e-50));
        }

        if (s.v[38] != 0.0) {
            s.store_scale(222, 270, (p.p225 * 0.0001));
        }

        s.b[1017] = ((p.p21 != 0.0) && (s.v[34] == 0.0));
        s.v[1017] = if s.b[1017] { 1.0 } else { 0.0 };

        if s.b[1017] {
            s.store_scalar(223, s.v[617]);
            s.store_scalar(225, s.v[619]);
            s.store_scale(279, 149, 6.241449993689894e18);
            s.store_mul_scaled_ad_lhs(280, A::add_scaled_inputs3(s.ad_value(270), 1.0, A::div(s.ad_value(149), A::sub(s.ad_value(56), s.ad_value(50))), 1.0, s.ad_value(225), 1.0), 122, 6.241449993689894e18);
            s.store_sub_ad_lhs(281, A::div_scaled_inputs(s.ad_value(91), (((-2.0) * 6.241449993689894e18) * 1.0 / (s.v[513])), s.ad_value(386), 1.0), 279);
        }

        s.b[1018] = ((((s.v[281] - s.v[279])) as f64).abs() > (10.0 * 2.220446049250313e-16));
        s.v[1018] = if s.b[1018] { 1.0 } else { 0.0 };

        if (s.b[1017] && s.b[1018]) {
            let assign21170_ad_e26697: A = A::add_scaled_product(A::div_scalar_by_product(1.0, A::add(s.ad_value(279), s.ad_value(280)), A::add(s.ad_value(281), s.ad_value(280)), 1.0), 1.0, A::div_scaled_product3(s.ad_value(223), s.ad_value(160), s.ad_value(158), 2.0, A::sub(s.ad_value(281), s.ad_value(279)), 1.0), A::ln(A::div_scaled_inputs2(s.ad_value(281), 1.0, s.ad_value(280), 1.0, A::add(s.ad_value(279), s.ad_value(280)), 1.0)), 1.0);
            s.store_add_scaled_product_mixed_aai(282, assign21170_ad_e26697, 1.0, A::mul3(A::mul3(s.ad_value(223), s.ad_value(160), s.ad_value(158)), s.ad_value(223), s.ad_value(160)), 158, 1.0);
        }

        if (s.b[1017] && (!s.b[1018])) {
            s.store_add_scaled_inputs_product_mixed_aaai(282, A::div_scalar_by_product(1.0, A::add(s.ad_value(279), s.ad_value(280)), A::add(s.ad_value(281), s.ad_value(280)), 1.0), 1.0, A::div_scaled_product3(s.ad_value(223), s.ad_value(160), s.ad_value(158), 2.0, A::add(s.ad_value(279), s.ad_value(280)), 1.0), 1.0, A::mul3(A::mul3(s.ad_value(223), s.ad_value(160), s.ad_value(158)), s.ad_value(223), s.ad_value(160)), 158, 1.0);
        }

        s.b[1019] = ((p.p23 != 0.0) && (s.v[34] == 0.0));
        s.v[1019] = if s.b[1019] { 1.0 } else { 0.0 };

        if s.b[1019] {
            s.store_div_scaled_inputs2(227, s.ad_value(260), 1.0, s.ad_value(56), (-1.0), s.ad_value(386), 1.0);
            s.store_scaled_mul(289, 159, 227, 1.0 / ((10000000.0 * 0.01)));
        }

        s.b[1020] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= p.p114) && (p.p114 <= (1.0 + (10.0 * 2.220446049250313e-16))));
        s.v[1020] = if s.b[1020] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1020]) {
            s.store_scalar(285, 1.0);
        }

        s.b[1021] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= p.p114) && (p.p114 <= (2.0 + (10.0 * 2.220446049250313e-16))));
        s.v[1021] = if s.b[1021] { 1.0 } else { 0.0 };

        if ((s.b[1019] && (!s.b[1020])) && s.b[1021]) {
            s.copy_ad(285, 289);
        }

        if ((s.b[1019] && (!s.b[1020])) && (!s.b[1021])) {
            s.store_powf(285, 289, (p.p114 - 1.0));
        }

        if s.b[1019] {
            s.store_offset_mul(287, 289, 285, 1.0);
            s.store_powf(288, 287, (((-1.0) / p.p114) - 1.0));
            s.store_mul3_lhs(230, 159, 287, 288);
            s.store_scaled_add(228, 158, 230, 0.5);
            s.store_square(278, 85);
        }

        if s.b[1019] {
            let assign21340_ad_e26942: A = A::add_scaled_inputs3(A::mul3(A::add_scaled_inputs(A::scale_offset(s.ad_value(85), 3.0, 1.0), 1.0, s.ad_value(278), 6.0), s.ad_value(230), s.ad_value(230)), 1.0, A::mul3(A::add_scaled_inputs(A::scale_offset(s.ad_value(85), 4.0, 3.0), 1.0, s.ad_value(278), 3.0), s.ad_value(230), s.ad_value(158)), 1.0, A::mul3(A::add(A::scale_offset(s.ad_value(85), 3.0, 6.0), s.ad_value(278)), s.ad_value(158), s.ad_value(158)), 1.0);
            s.store_div_scaled_product_by_product(229, A::mul3_scaled_output(s.ad_value(270), s.ad_value(86), s.ad_value(158), s.v[466]), assign21340_ad_e26942, 1.0, A::mul3_scaled_output(s.ad_value(386), A::offset(s.ad_value(85), 1.0), s.ad_value(228), 15.0), s.ad_value(228), 1.0);
        }

        if (!s.b[1019]) {
            s.store_scalar(229, 0.0);
        }

        s.b[1022] = ((((p.p20 != 0.0) && (p.p23 != 0.0)) && (s.v[35] == 1.0)) && (s.v[34] == 0.0));
        s.v[1022] = if s.b[1022] { 1.0 } else { 0.0 };

        if s.b[1022] {
            s.store_sqrt(235, 233);
            s.store_add(280, 86, 235);
            s.store_square(281, 231);
            s.store_square(282, 233);
            s.store_scaled_mul(283, 231, 233, 42.0);
            s.store_add_scaled_inputs3(283, s.ad_value(283), 1.0, s.ad_value(281), 4.0, s.ad_value(282), 4.0);
            s.store_add_ad_rhs(283, 283, A::mul3_scaled_output(s.ad_value(235), s.ad_value(86), A::add(s.ad_value(231), s.ad_value(233)), 20.0));
            s.store_square(288, 280);
            s.store_div_ad_rhs(236, 283, A::mul(A::square(s.ad_value(288)), s.ad_value(280)));
            s.store_mul_ad_product_lhs(237, A::div_from_scalar(s.v[466], s.ad_value(386)), s.ad_value(158), 270);
            s.store_mul(238, 237, 86);
            s.store_div(239, 229, 238);
            s.store_add_ad_lhs(285, A::add_scaled_product(s.ad_value(231), 1.0, s.ad_value(86), s.ad_value(235), 4.0), 233);
            s.store_div_scaled_product_by_product(240, s.ad_value(234), s.ad_value(285), 3.872983346207417, s.ad_value(280), A::sqrt(A::mul(A::mul3(s.ad_value(239), s.ad_value(280), s.ad_value(86)), s.ad_value(283))), 6.0);
        }

        s.store_add(94, 94, 193);

        if s.b[517] {
            s.store_scalar(200, ((-p.p172) * s.v[277]));
            s.store_mul_sub_rhs(201, 200, 42, 40);
        }

        if (!s.b[517]) {
            s.store_scalar(200, 0.0);
            s.store_scalar(201, 0.0);
        }

        s.v[215] = 0.0;

        s.store_scaled_sub(216, 42, 41, s.v[215]);

        s.store_scale(217, 42, s.v[215]);

        s.store_add(197, 197, 216);

        s.store_add(196, 196, 217);

        s.store_scale(0, 94, s.v[394]);

        s.store_scale(279, 123, (-s.v[513]));

        s.store_scaled_add(280, 523, 576, (-0.5));

        s.store_scaled_add(281, 531, 585, (-0.5));

        s.store_scaled_mul(444, 279, 40, (0.1 * s.v[294]));

        s.store_mul_sub_scaled_inputs_rhs(443, 279, s.ad_value(40), (0.1 * s.v[294]), s.ad_value(41), (0.1 * s.v[294]));

        s.store_mul(441, 279, 280);

        s.store_mul(442, 279, 281);

        if (p.p303 != 0.0) {
            s.store_scalar(336, 0.0);
            s.copy_ad(92, 91);
        }

        if (p.p303 == 0.0) {
            s.store_add_scaled_inputs3(92, s.ad_value(91), 1.0, s.ad_value(441), 1.0, s.ad_value(442), 1.0);
        }

        s.store_scale(93, 92, s.v[385]);

        if (s.v[38] != 0.0) {
            s.store_scalar(15, 0.0);
            s.store_scalar(14, 0.0);
            s.store_scalar(492, 0.0);
            s.store_scale(556, 336, s.v[394]);
        }

    }

    pub(super) fn stamp_transient_block_21(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if (s.v[38] != 0.0) {
            s.store_scale(555, 92, s.v[394]);
        }

        if (s.v[38] == 0.0) {
            s.store_sub_scaled_inputs(14, 336, (-s.v[394]), 92, s.v[394]);
            s.store_scaled_add(15, 93, 443, s.v[394]);
            s.store_add_scaled_inputs3(16, s.ad_value(92), s.v[394], s.ad_value(93), ((-1.0) * s.v[394]), s.ad_value(444), s.v[394]);
        }

        s.b[1023] = (p.p45 == 0.0);
        s.v[1023] = if s.b[1023] { 1.0 } else { 0.0 };

        if s.b[1023] {
            s.store_scalar(219, 0.0);
        }

        if (!s.b[1023]) {
            s.store_add_scaled_product_indices(218, 56, 1.0, 261, 123, 1.0);
        }

        s.b[1024] = (s.v[218] > s.v[260]);
        s.v[1024] = if s.b[1024] { 1.0 } else { 0.0 };

        if ((!s.b[1023]) && s.b[1024]) {
            s.copy_ad(218, 260);
        }

        if (!s.b[1023]) {
            s.store_add_scaled_inputs3(279, s.ad_value(51), s.v[264], s.ad_value(56), s.v[264], s.ad_value(218), (1.0 - s.v[264]));
            s.store_sqrt_div_from_scalar_ad(288, (2.0 * 1.034943e-10), s.ad_value(126));
            s.store_scale(281, 288, 1.3);
            s.store_scale(280, 281, (1.034943e-10 * s.v[513]));
            s.store_mul_ad_lhs(219, A::add_scaled_inputs4(s.ad_value(56), 1.0 / (p.p45), s.ad_value(51), 1.0 / (p.p45), s.ad_value(279), (-1.0 / (p.p45)), s.ad_value(261), -1.0), 280);
        }

        s.b[1025] = (p.p46 != 0.0);
        s.v[1025] = if s.b[1025] { 1.0 } else { 0.0 };

        if s.b[1025] {
            s.store_add_scaled_inputs(219, 219, 1.0, 50, s.v[490]);
        }

        s.b[1026] = (p.p14 == 1.0);
        s.v[1026] = if s.b[1026] { 1.0 } else { 0.0 };

        if s.b[1026] {
            s.store_add_ad_rhs(14, 14, A::sub_scaled_inputs(A::sub(A::add_scaled_inputs4(s.ad_value(197), 1.0, s.ad_value(196), 1.0, s.ad_value(201), -1.0, s.ad_value(219), -1.0), s.ad_value(398)), s.v[394], s.ad_value(397), s.v[394]));
            s.store_add_scaled_inputs4(15, s.ad_value(15), 1.0, s.ad_value(219), s.v[394], s.ad_value(197), ((-1.0) * s.v[394]), s.ad_value(405), s.v[394]);
            s.store_add_scaled_inputs3(16, s.ad_value(16), 1.0, s.ad_value(406), s.v[394], s.ad_value(196), (-s.v[394]));
        }

        s.store_scale(494, 185, s.v[394]);

        s.store_scale(6, 254, (-s.v[394]));

        s.b[1027] = (s.v[575] == 1.0);
        s.v[1027] = if s.b[1027] { 1.0 } else { 0.0 };

        if s.b[1027] {
            s.store_add_scaled_product_indices(4, 251, ((-1.0) * s.v[394]), 256, 255, s.v[394]);
        }

        if (!s.b[1027]) {
            s.store_sub_from_scalar(279, 1.0, 256);
            s.store_add_scaled_product_indices(4, 250, ((-1.0) * s.v[394]), 279, 255, s.v[394]);
        }

        s.b[1028] = (s.v[575] == 1.0);
        s.v[1028] = if s.b[1028] { 1.0 } else { 0.0 };

        if s.b[1028] {
            s.store_sub_from_scalar(279, 1.0, 256);
            s.store_add_scaled_product_indices(5, 250, ((-1.0) * s.v[394]), 279, 255, s.v[394]);
        }

        if (!s.b[1028]) {
            s.store_add_scaled_product_indices(5, 251, ((-1.0) * s.v[394]), 256, 255, s.v[394]);
        }

        if (s.v[575] == 1.0) {
            s.store_scale(2, 257, s.v[394]);
        } else {
            s.store_scale(2, 258, s.v[394]);
        }

        if (s.v[575] == 1.0) {
            s.store_scale(3, 258, s.v[394]);
        } else {
            s.store_scale(3, 257, s.v[394]);
        }

        s.store_scale(573, 374, (4.0 * 1.3806226e-23));

        s.store_scale(564, 229, s.v[394]);

        s.store_scalar(18, A::ddx_projection(&s.ad_value(14), Some(11), None));

        s.store_scale(18, 18, p.p33);

        s.store_scalar(19, A::ddx_projection(&s.ad_value(14), Some(12), None));

        s.store_scale(19, 19, p.p33);

        if (s.v[575] > 0.0) {
            s.copy_ad(493, 19);
        } else {
            s.copy_ad(493, 18);
        }

        s.b[1029] = ((((p.p20 != 0.0) && (p.p23 != 0.0)) && (s.v[35] == 1.0)) && (s.v[34] == 0.0));
        s.v[1029] = if s.b[1029] { 1.0 } else { 0.0 };

        if s.b[1029] {
            s.store_scaled_mul(278, 270, 123, (1e-6 * s.v[513]));
            s.store_scale(288, 493, 1.0 / (s.v[394]));
            s.store_div_scaled_product3_indices(241, 122, 288, 288, (0.1185185185185185 * 1.6021918e-19), 237, 1.0);
        }

        s.b[1030] = ((s.v[234] > (10.0 * 2.220446049250313e-16)) && (s.v[51] > (10.0 * 2.220446049250313e-16)));
        s.v[1030] = if s.b[1030] { 1.0 } else { 0.0 };

        if (s.b[1029] && s.b[1030]) {
            s.store_div(242, 159, 158);
            s.store_div_scaled_inputs2(243, A::div(s.ad_value(159), s.ad_value(230)), 1.0, s.ad_value(242), (-1.0), s.ad_value(51), 1.0);
            s.store_add_ad_rhs(244, 242, A::div_scaled_product(s.ad_value(243), A::add(A::add_scaled_product(s.ad_value(231), 1.0, s.ad_value(86), s.ad_value(235), 1.0), s.ad_value(233)), 0.6666666666666667, A::add(s.ad_value(86), s.ad_value(235)), 1.0));
        }

        if (s.b[1029] && (!s.b[1030])) {
            s.store_div(244, 159, 230);
        }

        if s.b[1029] {
            s.store_mul3_affine_lhs(495, 241, 236, s.v[394], 0.0, 244);
            s.copy_ad(496, 240);
        }

        if s.b[1029] {
            if (s.v[495] < 0.0) {
                s.store_scalar(495, 0.0);
            } else {
            }
        }

        if s.b[1029] {
            if ((-s.v[288]) > s.v[278]) {
            } else {
                s.store_scalar(495, 0.0);
            }
        }

        if s.b[1029] {
            if ((-s.v[288]) > s.v[278]) {
            } else {
                s.store_scalar(496, 0.0);
            }
        }

        if (!s.b[1029]) {
            s.store_scalar(495, 0.0);
            s.store_scalar(496, 0.0);
        }

        s.store_mul(608, 573, 564);

        s.copy_ad(609, 496);

        if ((s.v[608] > 0.0) && (s.v[495] > 0.0)) {
            s.store_sqrt_div(610, 495, 608);
        } else {
            s.store_scalar(610, 0.0);
        }

        if (s.v[575] > 0.0) {
            s.store_scale(611, 610, (1.0 - s.v[385]));
        } else {
            s.store_scale(611, 610, s.v[385]);
        }

        if (s.v[575] > 0.0) {
            s.store_scale(612, 610, s.v[385]);
        } else {
            s.store_scale(612, 610, (1.0 - s.v[385]));
        }

        s.v[632] = 0.0;

        s.v[633] = 0.0;

        s.b[1031] = (p.p312 == 1.0);
        s.v[1031] = if s.b[1031] { 1.0 } else { 0.0 };

        if s.b[1031] {
            s.store_scalar(1042, (p.p315 / 1e-6));
            s.store_scalar(1035, p.p317);
            s.store_scalar(1036, p.p319);
            s.store_scalar(1037, p.p324);
        }

        if s.b[1031] {
            s.store_scalar(1038, (if (p.p314 > 0.0) { (p.p314 * p.p308) } else { 0.0 }));
        }

        if s.b[1031] {
            s.store_scalar(1041, p.p311);
            s.store_scaled_voltage(1039, ctx, nodes, Some(12), Some(2), p.p33);
            s.store_scalar(1048, ((((p.p322 * p.p322) + (p.p38 * p.p38))) as f64).sqrt());
            s.store_scalar(1050, (s.v[124] * p.p5));
            s.store_scale(1035, 1035, 0.0001);
            s.store_scale(1036, 1036, 0.01);
            s.store_scale(1040, 374, 1.0 / (s.v[445]));
            s.store_powf(279, 1040, p.p320);
            s.store_div(1043, 1035, 279);
            s.store_sub_ad(278, A::add_scaled_product(A::scale_offset(s.ad_value(1040), 0.4, 1.8), 1.0, s.ad_value(1040), s.ad_value(1040), 0.1), A::scale_offset(s.ad_value(1040), (-p.p321), p.p321));
            s.store_div(1044, 1036, 278);
            s.store_add_ad_rhs(1037, 1037, A::scaled_offset(s.ad_value(374), (-s.v[445]), p.p325));
            s.store_scalar(1032, (1.0 + (p.p330 / ((s.v[375]) as f64).powf(p.p331))));
            s.store_scalar(1034, (1.0 + (p.p328 / ((s.v[375]) as f64).powf(p.p329))));
            s.store_scalar(1033, (1.0 + (p.p326 / ((s.v[376]) as f64).powf(p.p327))));
            s.store_mul(1043, 1043, 1032);
            s.store_offset_product3(1044, s.ad_value(1044), s.ad_value(1033), s.ad_value(1034), 1.0, 1e-50);
            s.store_div(1045, 1039, 1041);
            s.store_mul(1046, 1043, 1045);
        }

        s.b[1051] = (s.v[1039] >= 0.0);
        s.v[1051] = if s.b[1051] { 1.0 } else { 0.0 };

        if (s.b[1031] && s.b[1051]) {
            s.store_div(279, 1046, 1044);
        }

        if (s.b[1031] && (!s.b[1051])) {
            s.store_div_scaled_inputs(279, s.ad_value(1046), -1.0, s.ad_value(1044), 1.0);
        }

        s.b[1052] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= s.v[1037]) && (s.v[1037] <= (1.0 + (10.0 * 2.220446049250313e-16))));
        s.v[1052] = if s.b[1052] { 1.0 } else { 0.0 };

        if (s.b[1031] && s.b[1052]) {
            s.store_scalar(281, 1.0);
        }

        s.b[1053] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= s.v[1037]) && (s.v[1037] <= (2.0 + (10.0 * 2.220446049250313e-16))));
        s.v[1053] = if s.b[1053] { 1.0 } else { 0.0 };

        if ((s.b[1031] && (!s.b[1052])) && s.b[1053]) {
            s.copy_ad(281, 279);
        }

        if ((s.b[1031] && (!s.b[1052])) && (!s.b[1053])) {
            s.store_pow_ad(281, s.ad_value(279), A::offset(s.ad_value(1037), (-1.0)));
        }

        if s.b[1031] {
            s.store_mul(280, 279, 281);
            s.store_offset(282, 280, 1.0);
        }

        s.b[1054] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= s.v[1037]) && (s.v[1037] <= (1.0 + (10.0 * 2.220446049250313e-16))));
        s.v[1054] = if s.b[1054] { 1.0 } else { 0.0 };

        if (s.b[1031] && s.b[1054]) {
            s.store_div_from_scalar(283, 1.0, 282);
        }

        s.b[1055] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= s.v[1037]) && (s.v[1037] <= (2.0 + (10.0 * 2.220446049250313e-16))));
        s.v[1055] = if s.b[1055] { 1.0 } else { 0.0 };

        if ((s.b[1031] && (!s.b[1054])) && s.b[1055]) {
            s.store_div_from_scalar_sqrt_ad(283, 1.0, s.ad_value(282));
        }

        if ((s.b[1031] && (!s.b[1054])) && (!s.b[1055])) {
            s.store_pow_ad(284, s.ad_value(282), A::offset(A::div_from_scalar((-1.0), s.ad_value(1037)), (-1.0)));
            s.store_mul(283, 282, 284);
        }

        if s.b[1031] {
            s.store_mul(1047, 1043, 283);
            s.store_div_from_scalar(279, 1.6021918e-19, 1041);
            s.store_mul_product3_rhs(1049, 1042, s.ad_value(279), s.ad_value(1048), s.ad_value(1047), 1.0);
        }

        s.b[1056] = (s.v[1049] <= 0.0);
        s.v[1056] = if s.b[1056] { 1.0 } else { 0.0 };

        if (s.b[1031] && s.b[1056]) {
            s.store_scalar(1049, 1e-50);
        }

        if s.b[1031] {
            s.store_div_from_scalar(27, 1.0, 1049);
            s.store_div(27, 27, 1050);
            s.store_add(27, 27, 1038);
        }

        s.b[1057] = (s.v[27] < 0.0001);
        s.v[1057] = if s.b[1057] { 1.0 } else { 0.0 };

        if (s.b[1031] && s.b[1057]) {
            s.store_scalar(27, 0.0001);
        }

        if s.b[1031] {
            s.store_scale(633, 27, 1.0 / (s.v[394]));
        }

        s.b[1058] = (p.p313 == 1.0);
        s.v[1058] = if s.b[1058] { 1.0 } else { 0.0 };

        if s.b[1058] {
            s.store_scalar(1069, (p.p40 / 1e-6));
            s.store_scalar(1062, p.p316);
            s.store_scalar(1063, p.p318);
            s.store_scalar(1064, p.p323);
        }

        if s.b[1058] {
            s.store_scalar(1065, (if (p.p314 > 0.0) { (p.p314 * p.p309) } else { 0.0 }));
        }

        if s.b[1058] {
            s.store_scalar(1068, p.p310);
            s.store_scaled_voltage(1066, ctx, nodes, Some(0), Some(11), p.p33);
            s.store_scalar(1075, ((((p.p322 * p.p322) + (p.p38 * p.p38))) as f64).sqrt());
            s.store_scalar(1077, (s.v[124] * p.p5));
            s.store_scale(1062, 1062, 0.0001);
            s.store_scale(1063, 1063, 0.01);
            s.store_scale(1067, 374, 1.0 / (s.v[445]));
            s.store_powf(279, 1067, p.p320);
            s.store_div(1070, 1062, 279);
            s.store_sub_ad(278, A::add_scaled_product(A::scale_offset(s.ad_value(1067), 0.4, 1.8), 1.0, s.ad_value(1067), s.ad_value(1067), 0.1), A::scale_offset(s.ad_value(1067), (-p.p321), p.p321));
        }

    }

    pub(super) fn stamp_transient_block_22(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[1058] {
            s.store_div(1071, 1063, 278);
            s.store_add_ad_rhs(1064, 1064, A::scaled_offset(s.ad_value(374), (-s.v[445]), p.p325));
            s.store_scalar(1059, (1.0 + (p.p330 / ((s.v[375]) as f64).powf(p.p331))));
            s.store_scalar(1061, (1.0 + (p.p328 / ((s.v[375]) as f64).powf(p.p329))));
            s.store_scalar(1060, (1.0 + (p.p326 / ((s.v[376]) as f64).powf(p.p327))));
            s.store_mul(1070, 1070, 1059);
            s.store_offset_product3(1071, s.ad_value(1071), s.ad_value(1060), s.ad_value(1061), 1.0, 1e-50);
            s.store_div(1072, 1066, 1068);
            s.store_mul(1073, 1070, 1072);
        }

        s.b[1078] = (s.v[1066] >= 0.0);
        s.v[1078] = if s.b[1078] { 1.0 } else { 0.0 };

        if (s.b[1058] && s.b[1078]) {
            s.store_div(279, 1073, 1071);
        }

        if (s.b[1058] && (!s.b[1078])) {
            s.store_div_scaled_inputs(279, s.ad_value(1073), -1.0, s.ad_value(1071), 1.0);
        }

        s.b[1079] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= s.v[1064]) && (s.v[1064] <= (1.0 + (10.0 * 2.220446049250313e-16))));
        s.v[1079] = if s.b[1079] { 1.0 } else { 0.0 };

        if (s.b[1058] && s.b[1079]) {
            s.store_scalar(281, 1.0);
        }

        s.b[1080] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= s.v[1064]) && (s.v[1064] <= (2.0 + (10.0 * 2.220446049250313e-16))));
        s.v[1080] = if s.b[1080] { 1.0 } else { 0.0 };

        if ((s.b[1058] && (!s.b[1079])) && s.b[1080]) {
            s.copy_ad(281, 279);
        }

        if ((s.b[1058] && (!s.b[1079])) && (!s.b[1080])) {
            s.store_pow_ad(281, s.ad_value(279), A::offset(s.ad_value(1064), (-1.0)));
        }

        if s.b[1058] {
            s.store_mul(280, 279, 281);
            s.store_offset(282, 280, 1.0);
        }

        s.b[1081] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= s.v[1064]) && (s.v[1064] <= (1.0 + (10.0 * 2.220446049250313e-16))));
        s.v[1081] = if s.b[1081] { 1.0 } else { 0.0 };

        if (s.b[1058] && s.b[1081]) {
            s.store_div_from_scalar(283, 1.0, 282);
        }

        s.b[1082] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= s.v[1064]) && (s.v[1064] <= (2.0 + (10.0 * 2.220446049250313e-16))));
        s.v[1082] = if s.b[1082] { 1.0 } else { 0.0 };

        if ((s.b[1058] && (!s.b[1081])) && s.b[1082]) {
            s.store_div_from_scalar_sqrt_ad(283, 1.0, s.ad_value(282));
        }

        if ((s.b[1058] && (!s.b[1081])) && (!s.b[1082])) {
            s.store_pow_ad(284, s.ad_value(282), A::offset(A::div_from_scalar((-1.0), s.ad_value(1064)), (-1.0)));
            s.store_mul(283, 282, 284);
        }

        if s.b[1058] {
            s.store_mul(1074, 1070, 283);
            s.store_div_from_scalar(279, 1.6021918e-19, 1068);
            s.store_mul_product3_rhs(1076, 1069, s.ad_value(279), s.ad_value(1075), s.ad_value(1074), 1.0);
        }

        s.b[1083] = (s.v[1076] <= 0.0);
        s.v[1083] = if s.b[1083] { 1.0 } else { 0.0 };

        if (s.b[1058] && s.b[1083]) {
            s.store_scalar(1076, 1e-50);
        }

        if s.b[1058] {
            s.store_div_from_scalar(27, 1.0, 1076);
            s.store_div(27, 27, 1077);
            s.store_add(27, 27, 1065);
        }

        s.b[1084] = (s.v[27] < 0.0001);
        s.v[1084] = if s.b[1084] { 1.0 } else { 0.0 };

        if (s.b[1058] && s.b[1084]) {
            s.store_scalar(27, 0.0001);
        }

        if s.b[1058] {
            s.store_scale(632, 27, 1.0 / (s.v[394]));
        }

        s.b[1085] = (s.v[221] < 1e-18);
        s.v[1085] = if s.b[1085] { 1.0 } else { 0.0 };

        if ((s.v[38] != 0.0) && s.b[1085]) {
            s.store_scalar(221, 1e-18);
        }

        s.b[1086] = (s.v[222] < 1e-18);
        s.v[1086] = if s.b[1086] { 1.0 } else { 0.0 };

        if ((s.v[38] != 0.0) && s.b[1086]) {
            s.store_scalar(222, 1e-18);
        }

        if (s.v[38] != 0.0) {
            s.store_div_scaled_inputs2(549, s.ad_value(551), 1.0, s.ad_value(555), (-1.0), s.ad_value(221), 1.0);
            s.store_div_scaled_inputs2(550, s.ad_value(548), 1.0, s.ad_value(556), (-1.0), s.ad_value(222), 1.0);
            s.store_sub_scaled_inputs(554, 551, -1.0, 548, 1.0);
            s.store_scale(552, 551, s.v[385]);
            s.store_scale(553, 551, (1.0 - s.v[385]));
        }

        if (s.v[38] == 0.0) {
            s.store_scalar(549, 0.0);
            s.store_scalar(550, 0.0);
            s.store_scalar(552, 0.0);
            s.store_scalar(553, 0.0);
            s.store_scalar(554, 0.0);
            s.store_scalar(548, 0.0);
        }

        s.copy_ad(26, 632);

        s.copy_ad(27, 633);

        s.b[1087] = (s.v[575] == 1.0);
        s.v[1087] = if s.b[1087] { 1.0 } else { 0.0 };

        if s.b[1087] {
            s.copy_ad(94, 0);
            s.copy_ad(185, 494);
            s.store_scalar(546, 0.0);
            s.copy_ad(561, 14);
            s.copy_ad(93, 15);
            s.store_neg_ad(492, A::add_scaled_inputs3(s.ad_value(14), 1.0, s.ad_value(15), 1.0, s.ad_value(16), 1.0));
            s.copy_ad(90, 492);
        }

        if (!s.b[1087]) {
            s.store_neg(94, 0);
            s.copy_ad(546, 494);
            s.store_scalar(185, 0.0);
            s.copy_ad(561, 14);
            s.copy_ad(93, 16);
            s.store_neg_ad(492, A::add_scaled_inputs3(s.ad_value(14), 1.0, s.ad_value(15), 1.0, s.ad_value(16), 1.0));
            s.copy_ad(90, 492);
            s.copy_ad(16, 15);
            s.copy_ad(15, 93);
        }

        if ((!s.b[1087]) && (s.v[38] != 0.0)) {
            s.copy_ad(279, 552);
            s.copy_ad(552, 553);
            s.copy_ad(553, 279);
        }

        s.b[1088] = ((p.p28 != 0.0) && (p.p237 > 0.0));
        s.v[1088] = if s.b[1088] { 1.0 } else { 0.0 };

        if s.b[1088] {
            s.store_mul(547, 0, 51);
            s.store_scalar(516, s.v[468]);
            s.store_scalar(557, (1.0 / s.v[467]));
        }

        if (!s.b[1088]) {
            s.store_scalar(547, 0.0);
            s.store_scalar(516, 0.0);
            s.store_scalar(557, 0.0);
        }

        s.copy_ad(251, 4);

        s.copy_ad(250, 5);

        s.copy_ad(254, 6);

        s.copy_ad(257, 2);

        s.copy_ad(258, 3);

        s.copy_ad(0, 94);

        s.store_scalar(18, A::ddx_projection(&s.ad_value(14), Some(11), None));

        s.store_scale(18, 18, p.p33);

        s.store_scalar(19, A::ddx_projection(&s.ad_value(14), Some(12), None));

        s.store_scale(19, 19, p.p33);

        s.b[1094] = ((p.p28 != 0.0) && (p.p237 > 0.0));
        s.v[1094] = if s.b[1094] { 1.0 } else { 0.0 };

        s.b[1095] = (((p.p27 != 0.0) && (p.p15 != 0.0)) && (p.p16 != 0.0));
        s.v[1095] = if s.b[1095] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_reactive_block_0(
        s: &mut ReactiveScratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        s.v[649] = 2.0;

        s.v[650] = 0.1;

        s.v[651] = 0.1;

        s.v[514] = 0.0;

        s.v[574] = 0.0;

        s.v[237] = 1e-12;

        s.v[28] = 500.0;

        s.v[29] = 200.0;

        s.v[32] = 0.002;

        s.v[38] = p.p24;

        s.v[46] = 1.0;

        s.v[36] = 1.0;

        s.v[305] = 0.0;

        s.v[306] = 0.0;

        s.v[307] = 0.0;

        s.v[308] = 0.0;

        s.v[309] = 0.0;

        s.v[310] = 0.0;

        s.v[312] = 0.0;

        s.v[314] = 0.0;

        s.v[311] = 0.0;

        s.v[313] = 0.0;

        s.v[207] = 0.0;

        s.v[209] = 0.0;

        s.v[531] = 0.0;

        s.v[528] = 0.0;

        s.v[585] = 0.0;

        s.v[588] = 0.0;

        s.v[523] = 0.0;

        s.v[576] = 0.0;

        s.v[555] = 0.0;

        s.v[556] = 0.0;

        s.v[322] = 0.0;

        s.v[327] = 0.0;

        s.v[329] = 0.0;

        s.v[330] = 0.0;

        s.v[331] = 0.0;

        s.v[334] = 0.0;

        s.v[336] = 0.0;

        s.v[337] = 0.0;

        s.v[345] = 0.0;

        s.v[383] = 0.0;

        s.v[385] = 0.5;

        s.v[441] = 0.0;

        s.v[442] = 0.0;

        s.v[558] = 0.0;

        s.v[405] = 0.0;

        s.v[406] = 0.0;

        s.v[397] = 0.0;

        s.v[398] = 0.0;

        s.v[414] = 0.0;

        s.v[34] = 0.0;

        s.v[35] = 0.0;

        s.v[292] = 0.0;

        s.v[16] = 0.0;

        s.v[60] = 0.0;

        s.v[58] = 0.0;

        s.v[74] = 1.0;

        s.v[85] = 0.0;

        s.v[91] = 0.0;

        s.v[93] = 0.0;

        s.v[94] = 0.0;

        s.v[151] = 0.0;

        s.v[158] = 0.0;

        s.v[159] = 0.0;

        s.v[160] = 0.0;

        s.v[185] = 0.0;

        s.v[189] = 1.0;

        s.v[193] = 0.0;

        s.v[196] = 0.0;

        s.v[197] = 0.0;

        s.v[221] = 0.0;

        s.v[222] = 0.0;

        s.v[146] = 0.0;

        s.v[260] = 0.0;

        s.v[89] = 0.0;

        s.v[230] = 0.0;

        s.v[231] = 0.0;

        s.v[233] = 0.0;

        s.v[234] = 0.0;

        s.v[235] = 0.0;

        s.v[236] = 0.0;

        s.v[55] = 0.0;

        s.v[77] = 0.0;

        s.v[339] = 0.0;

        s.v[388] = 0.0;

        s.v[316] = 0.0;

        s.b[517] = param_given[172];
        s.v[517] = if s.b[517] { 1.0 } else { 0.0 };

        s.b[518] = param_given[173];
        s.v[518] = if s.b[518] { 1.0 } else { 0.0 };

        s.b[519] = param_given[174];
        s.v[519] = if s.b[519] { 1.0 } else { 0.0 };

        s.b[463] = param_given[9];
        s.v[463] = if s.b[463] { 1.0 } else { 0.0 };

        s.v[394] = 1.0;

        s.v[446] = (if param_given[177] { p.p177 } else { (5000000000.0 / (p.p227 * p.p230)) });

        s.b[660] = ((s.v[446] < (2.0 + 0.1)) && (0.1 >= 0.0));
        s.v[660] = if s.b[660] { 1.0 } else { 0.0 };

        if s.b[660] {
            s.store_scalar(638, ((2.0 + 0.1) - s.v[446]));
            s.store_square(642, 638);
            s.store_scalar(643, (0.1 * 0.1));
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

        s.b[661] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[661] = if s.b[661] { 1.0 } else { 0.0 };

        s.b[662] = (2.0 == 1.0);
        s.v[662] = if s.b[662] { 1.0 } else { 0.0 };

        if ((s.b[660] && s.b[661]) && s.b[662]) {
            s.store_scalar(648, 1.0);
        }

        s.b[663] = (2.0 == 2.0);
        s.v[663] = if s.b[663] { 1.0 } else { 0.0 };

        if (((s.b[660] && s.b[661]) && (!s.b[662])) && s.b[663]) {
            s.store_scalar(648, 2.0);
        }

        s.b[664] = (2.0 == 4.0);
        s.v[664] = if s.b[664] { 1.0 } else { 0.0 };

        if ((((s.b[660] && s.b[661]) && (!s.b[662])) && (!s.b[663])) && s.b[664]) {
            s.store_scalar(648, 3.0);
        }

        s.b[665] = (2.0 == 8.0);
        s.v[665] = if s.b[665] { 1.0 } else { 0.0 };

        if (((((s.b[660] && s.b[661]) && (!s.b[662])) && (!s.b[663])) && (!s.b[664])) && s.b[665]) {
            s.store_scalar(648, 4.0);
        }

        if (s.b[660] && s.b[661]) {
            s.store_scalar(647, 0.0);
        }

        let mut assign1360_loop_guard: usize = 0;
        while {
            let assign1360_cond_e892: f64 = if ((s.b[660] && s.b[661]) && (s.v[647] < s.v[648])) { 1.0 } else { 0.0 };
            assign1360_cond_e892 != 0.0
        } {
            assign1360_loop_guard += 1;
            assert!(assign1360_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (s.b[660] && s.b[661]) {
                s.store_sqrt(646, 646);
                s.store_offset(647, 647, 1.0);
            }
        }

        if (s.b[660] && (!s.b[661])) {
            s.store_powf(646, 646, (1.0 / (2.0 * 2.0)));
        }

        if s.b[660] {
            s.store_div_from_scalar_offset_input(646, 1.0, 646, 1e-50);
            s.store_scaled_mul(637, 638, 646, 0.1);
            s.store_div_scaled_product_offset_denominator(278, s.ad_value(645), s.ad_value(646), 0.1, s.ad_value(220), 1e-50, 1.0);
            s.store_sub_from_scalar(446, (2.0 + 0.1), 637);
        }

        if s.b[660] {
        }

        if (!s.b[660]) {
        }

        if (!s.b[660]) {
            s.store_scalar(278, 1.0);
        }

        s.v[613] = (p.p34 * 0.01);

        s.v[614] = (p.p59 / 1e-6);

        s.v[615] = (p.p101 * 0.01);

        s.v[616] = (p.p192 / 1e-6);

        s.v[617] = (p.p219 * 0.01);

        s.v[619] = (p.p220 / 0.0001);

        s.v[620] = (p.p230 / 1e-6);

        s.v[621] = (p.p231 / 1e-6);

        s.v[622] = (p.p237 * 0.01);

        s.v[623] = (p.p238 / 0.01);

        s.v[624] = (p.p40 / 1e-6);

        s.v[625] = (p.p236 / 1e-6);

        s.v[627] = (p.p197 / 0.01);

        s.v[630] = (p.p306 / 1e-6);

        s.v[631] = (p.p307 / 1e-6);

        s.v[626] = (p.p189 * 10000.0);

        s.v[452] = (p.p147 / 1e-6);

        s.v[628] = (p.p196 / 10.0);

        s.v[445] = (p.p222 + 273.15);

        s.v[447] = (p.p9 + 273.15);

        s.v[509] = p.p41;

        s.v[510] = p.p42;

        s.v[277] = p.p0;

        s.v[456] = (p.p1 / p.p5);

        s.v[375] = (s.v[277] * 1000000.0);

        s.v[376] = (s.v[456] * 1000000.0);

        s.v[377] = (s.v[376] * s.v[375]);

        s.v[279] = (p.p62 / ((s.v[377]) as f64).powf(p.p63));

        s.v[133] = (s.v[277] + s.v[279]);

        s.v[134] = (s.v[456] + s.v[279]);

        s.v[482] = (p.p64 / ((s.v[377]) as f64).powf(p.p65));

        s.v[279] = (1.0 + (p.p148 / (((s.v[133] * 1000000.0)) as f64).powf(p.p149)));

        s.v[280] = (1.0 + (p.p150 / (((s.v[134] * 1000000.0)) as f64).powf(p.p151)));

        s.v[452] = ((s.v[452] * s.v[279]) * s.v[280]);

        s.v[279] = (1.0 + (p.p154 / (((s.v[133] * 1000000.0)) as f64).powf(p.p155)));

        s.v[280] = (1.0 + (p.p156 / (((s.v[134] * 1000000.0)) as f64).powf(p.p157)));

        s.v[453] = ((p.p152 * s.v[279]) * s.v[280]);

        s.v[511] = ((2.0 * s.v[453]) * p.p153);

        s.v[124] = ((s.v[456] - (2.0 * s.v[509])) - s.v[511]);

        s.v[512] = ((s.v[456] - (2.0 * s.v[510])) - s.v[511]);

        s.v[466] = (s.v[124] * p.p5);

        s.v[513] = (s.v[512] * p.p5);

        s.v[467] = (s.v[622] / (s.v[394] * s.v[466]));

        s.v[468] = (s.v[623] * (s.v[394] * s.v[513]));

        s.v[278] = (s.v[630] * ((p.p11 + (p.p304 * p.p12)) + (p.p305 * p.p13)));

        s.v[620] = (s.v[620] + s.v[278]);

        s.v[638] = ((s.v[620] - (1000000000000000.0 / 1e-6)) - (0.01 / 1e-6));

        s.v[639] = ((4.0 * (1000000000000000.0 / 1e-6)) * (0.01 / 1e-6));

        if (!(s.v[639] > 0.0)) {
            s.store_scalar(639, (-s.v[639]));
        }

        s.store_sqrt_offset_input(639, 639, (s.v[638] * s.v[638]));

        s.store_offset_scaled(620, 639, 0.5, ((((s.v[638]) * (0.5))) + ((1000000000000000.0 / 1e-6))));

        s.v[278] = (s.v[631] * ((p.p11 + (p.p304 * p.p12)) + (p.p305 * p.p13)));

        s.v[614] = (s.v[614] + s.v[278]);

        s.v[638] = ((s.v[614] - (1000000000000000.0 / 1e-6)) - (0.01 / 1e-6));

        s.v[639] = ((4.0 * (1000000000000000.0 / 1e-6)) * (0.01 / 1e-6));

        if (!(s.v[639] > 0.0)) {
            s.store_scalar(639, (-s.v[639]));
        }

        s.store_sqrt_offset_input(639, 639, (s.v[638] * s.v[638]));

        s.store_offset_scaled(614, 639, 0.5, ((((s.v[638]) * (0.5))) + ((1000000000000000.0 / 1e-6))));

        s.v[448] = ((p.p86 * ((s.v[375]) as f64).powf(p.p88)) * (1.0 + (p.p90 / ((s.v[375]) as f64).powf(p.p91))));

        s.v[449] = ((p.p87 * ((s.v[375]) as f64).powf(p.p89)) * (1.0 + (p.p92 / ((s.v[375]) as f64).powf(p.p93))));

        s.v[450] = ((p.p289 * ((s.v[375]) as f64).powf(p.p291)) * (1.0 + (p.p293 / ((s.v[375]) as f64).powf(p.p294))));

        s.v[451] = ((p.p290 * ((s.v[375]) as f64).powf(p.p292)) * (1.0 + (p.p295 / ((s.v[375]) as f64).powf(p.p296))));

        s.v[470] = ((p.p106 * (1.0 + (p.p107 / ((s.v[375]) as f64).powf(p.p110)))) * (1.0 + (p.p108 / ((s.v[376]) as f64).powf(p.p109))));

        s.v[594] = ((p.p283 * (1.0 + (p.p285 / ((s.v[375]) as f64).powf(p.p286)))) * (1.0 + (p.p287 / ((s.v[376]) as f64).powf(p.p288))));

        s.v[279] = (s.v[621] * (1.0 + (p.p232 / ((s.v[375]) as f64).powf(p.p233))));

        s.v[638] = ((s.v[279] - s.v[625]) - (s.v[621] * 0.001));

        s.v[639] = ((4.0 * s.v[625]) * (s.v[621] * 0.001));

        if (!(s.v[639] > 0.0)) {
            s.store_scalar(639, (-s.v[639]));
        }

        s.store_sqrt_offset_input(639, 639, (s.v[638] * s.v[638]));

        s.store_offset_scaled(462, 639, 0.5, ((((s.v[638]) * (0.5))) + (s.v[625])));

        if (p.p32 != 0.0) {
            s.store_scale(279, 462, (1.0 + (p.p234 / ((s.v[376]) as f64).powf(p.p235))));
            s.store_offset(638, 279, (((-s.v[625])) + ((-(s.v[621] * 0.001)))));
            s.store_scalar(639, ((4.0 * s.v[625]) * (s.v[621] * 0.001)));
        }

        if (p.p32 != 0.0) {
            if (s.v[639] > 0.0) {
            } else {
                s.store_neg(639, 639);
            }
        }

        if (p.p32 != 0.0) {
            s.store_sqrt_square_add(639, 638, 639);
            s.store_offset_add_scaled_inputs_indices(462, 638, 0.5, 639, 0.5, s.v[625]);
        }

        s.store_scale(460, 614, (1.0 + (p.p60 / ((s.v[376]) as f64).powf(p.p61))));

    }

    pub(super) fn stamp_reactive_block_1(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        let ctx_temp = ctx.temperature();
        let nv4 = ctx.node_voltage(nodes[4]);
        s.copy_ad(461, 460);

        s.v[279] = ((1.0 / (p.p43 + (0.5 * p.p0))) + (1.0 / (p.p44 + (0.5 * p.p0))));

        s.v[459] = (2.0 / s.v[279]);

        s.b[666] = (((p.p6 > 0.0) && (p.p7 > 0.0)) && ((p.p5 == 1.0) || ((p.p5 > 1.0) && (p.p8 > 0.0))));
        s.v[666] = if s.b[666] { 1.0 } else { 0.0 };

        if s.b[666] {
            s.store_scalar(279, 0.0);
            s.store_scalar(514, 0.0);
        }

        let mut assign2290_loop_guard: usize = 0;
        while {
            let assign2290_cond_e1503: f64 = if (s.b[666] && (s.v[514] < p.p5)) { 1.0 } else { 0.0 };
            assign2290_cond_e1503 != 0.0
        } {
            assign2290_loop_guard += 1;
            assert!(assign2290_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if s.b[666] {
                s.store_add_scaled_inputs3(279, s.ad_value(279), 1.0, A::div_from_scalar(1.0, A::scale_offset(s.ad_value(514), (p.p8 + p.p0), (p.p6 + (0.5 * p.p0)))), 1.0, A::div_from_scalar(1.0, A::scale_offset(s.ad_value(514), (p.p8 + p.p0), (p.p7 + (0.5 * p.p0)))), 1.0);
                s.store_offset(514, 514, 1.0);
            }
        }

        if s.b[666] {
            s.store_div_from_scalar(458, (2.0 * p.p5), 279);
        }

        if (!s.b[666]) {
            s.store_scalar(458, 0.0);
        }

        s.b[667] = (s.v[458] > 0.0);
        s.v[667] = if s.b[667] { 1.0 } else { 0.0 };

        if s.b[667] {
            s.store_scalar(279, (1.0 / (1.0 + p.p166)));
            s.store_scalar(280, 0.0);
            s.store_scalar(281, 0.0);
            s.store_div_scaled_product_offset_denominator(461, s.ad_value(460), A::offset(A::mul(s.ad_value(279), s.ad_value(280)), 1.0), 1.0, A::mul(s.ad_value(279), s.ad_value(281)), 1.0, 1.0);
            s.store_scalar(279, (1.0 / (1.0 + p.p169)));
            s.store_powf_ad(280, A::div_from_scalar(p.p168, s.ad_value(458)), p.p170);
            s.store_scalar(281, (((p.p168 / s.v[459])) as f64).powf(p.p170));
            s.store_div_scaled_product_offset_denominator(620, s.ad_value(620), A::offset(A::mul(s.ad_value(279), s.ad_value(280)), 1.0), 1.0, A::mul(s.ad_value(279), s.ad_value(281)), 1.0, 1.0);
        }

        if (!s.b[667]) {
            s.copy_ad(461, 460);
        }

        s.v[280] = (1.0 + (p.p190 / ((s.v[376]) as f64).powf(p.p191)));

        s.store_div_from_scalar(281, s.v[616], 620);

        s.store_offset(638, 281, (((-s.v[280])) + ((-0.01))));

        s.store_scale(639, 281, (4.0 * 0.01));

        if (!(s.v[639] > 0.0)) {
            s.store_neg(639, 639);
        }

        s.store_sqrt_square_add(639, 638, 639);

        s.store_add_scaled_inputs3(279, s.ad_value(281), 1.0, s.ad_value(638), (-0.5), s.ad_value(639), (-0.5));

        s.store_mul(471, 620, 279);

        s.b[668] = ((s.v[277] > p.p58) || (p.p58 <= 0.0));
        s.v[668] = if s.b[668] { 1.0 } else { 0.0 };

        if s.b[668] {
            s.store_add_scaled_inputs(457, 471, ((s.v[277] - p.p58) * 1.0 / (s.v[277])), 461, (p.p58 * 1.0 / (s.v[277])));
        }

        if (!s.b[668]) {
            s.store_add_scaled_inputs3(457, s.ad_value(461), 1.0, s.ad_value(461), ((p.p58 - s.v[277]) * 1.0 / (p.p58)), s.ad_value(471), (-((p.p58 - s.v[277]) * 1.0 / (p.p58))));
        }

        s.store_scale(126, 457, 1.6021918e-19);

        s.store_scale(472, 126, 1.034943e-10);

        s.store_scale(473, 472, 2.0);

        s.store_scale(474, 462, (1.6021918e-19 * 1.034943e-10));

        s.v[475] = (p.p239 * ((s.v[375]) as f64).powf((-p.p242)));

        s.v[476] = (p.p243 * ((s.v[375]) as f64).powf((-p.p244)));

        s.v[477] = (p.p246 * (((s.v[375] + p.p248)) as f64).powf((-p.p247)));

        s.b[669] = ((s.v[277] <= (2.0 * p.p58)) && (p.p58 > 0.0));
        s.v[669] = if s.b[669] { 1.0 } else { 0.0 };

        if s.b[669] {
            s.store_add_scaled_inputs4(560, s.ad_value(461), 2.0, s.ad_value(461), (-(s.v[277] * 1.0 / (p.p58))), s.ad_value(471), (-(-(s.v[277] * 1.0 / (p.p58)))), s.ad_value(471), -1.0);
            s.store_ln_div(478, 560, 471);
        }

        if (!s.b[669]) {
            s.store_scalar(478, 0.0);
        }

        s.store_scaled_ln_scaled_input(129, 457, 1.0 / (1.04e16), (2.0 / 38.68283));

        s.store_scaled_ln_scaled_input(136, 471, 1.0 / (1.04e16), (2.0 / 38.68283));

        s.v[479] = ((((1.0 + (1.0 / s.v[375]))) as f64).powf(p.p77) * p.p75);

        s.v[279] = (p.p116 * s.v[375]);

        s.v[481] = ((((s.v[279] * p.p115) / (s.v[279] + p.p115)) + p.p117) + 1e-50);

        s.v[483] = (1.0 + (((s.v[375]) as f64).powf(p.p179) * p.p180));

        s.b[670] = (p.p25 == 1.0);
        s.v[670] = if s.b[670] { 1.0 } else { 0.0 };

        if s.b[670] {
            s.store_scalar(279, (p.p3 + (s.v[124] / (3.0 * p.p2))));
        }

        s.v[485] = (1.0 + (p.p131 / ((s.v[376]) as f64).powf(p.p132)));

        s.v[486] = (p.p125 * (1.0 + (p.p126 / ((s.v[375]) as f64).powf(p.p127))));

        s.v[487] = (s.v[375] / (s.v[375] + p.p124));

        s.v[488] = (p.p118 * (1.0 + (p.p120 / ((s.v[375]) as f64).powf(p.p121))));

        s.v[489] = (p.p119 * (1.0 + (p.p122 / s.v[375])));

        s.v[490] = (((10000.0 * s.v[513]) * p.p46) / ((s.v[375]) as f64).powf(p.p47));

        s.v[559] = (p.p133 * (1.0 + (p.p134 / ((s.v[375]) as f64).powf(p.p135))));

        s.v[491] = (p.p128 * (1.0 + (p.p129 / ((s.v[375]) as f64).powf(p.p130))));

        s.v[279] = ((2.0 * 1.034943e-10) / 1.6021918e-19);

        s.store_sqrt_div_from_scalar_ad(132, s.v[279], s.ad_value(457));

        s.store_scaled_voltage(540, ctx, nodes, Some(5), Some(12), p.p33);

        s.store_scaled_voltage(541, ctx, nodes, Some(11), Some(12), p.p33);

        s.store_scaled_voltage(542, ctx, nodes, Some(6), Some(12), p.p33);

        s.store_scaled_voltage(543, ctx, nodes, Some(5), Some(2), p.p33);

        s.store_scaled_voltage(544, ctx, nodes, Some(0), Some(2), p.p33);

        s.store_scaled_voltage(545, ctx, nodes, Some(6), Some(2), p.p33);

        s.b[672] = ((p.p28 != 0.0) && (p.p237 > 0.0));
        s.v[672] = if s.b[672] { 1.0 } else { 0.0 };

        if s.b[672] {
            if (nv4 > 0.0) {
                s.store_voltage(11, ctx, nodes, Some(4), None);
            } else {
                s.store_scalar(11, 0.0);
            }
        }

        if (!s.b[672]) {
            s.store_scalar(11, 0.0);
        }

        if (s.v[38] != 0.0) {
            s.store_scaled_voltage(551, ctx, nodes, Some(8), None, 1e-9);
            s.store_scaled_voltage(548, ctx, nodes, Some(9), None, 1e-9);
        }

        if (s.v[38] == 0.0) {
            s.store_scalar(551, 0.0);
            s.store_scalar(548, 0.0);
        }

        s.b[673] = (s.v[541] >= 0.0);
        s.v[673] = if s.b[673] { 1.0 } else { 0.0 };

        if s.b[673] {
            s.store_scalar(575, 1.0);
            s.store_scalar(412, 1.0);
            s.store_scalar(413, 0.0);
            s.copy_ad(49, 540);
            s.copy_ad(48, 541);
            s.copy_ad(47, 542);
            s.copy_ad(42, 543);
            s.copy_ad(41, 544);
            s.copy_ad(40, 545);
        }

        if (!s.b[673]) {
            s.store_scalar(575, (-1.0));
            s.store_scalar(412, 0.0);
            s.store_scalar(413, 1.0);
            s.store_sub(49, 540, 541);
            s.store_neg(48, 541);
            s.store_sub(47, 542, 541);
            s.store_sub(42, 543, 544);
            s.store_neg(41, 544);
            s.store_sub(40, 545, 544);
        }

        s.v[374] = ctx_temp;

        if s.b[463] {
            s.store_scalar(374, s.v[447]);
        }

        s.store_add_ad_lhs(374, A::offset(s.ad_value(374), p.p10), 11);

        s.v[465] = (p.p37 - (s.v[445] * (9.025e-5 + (s.v[445] * 1e-7))));

        s.store_offset_square(279, 374, (-(s.v[445] * s.v[445])));

        s.store_sub_scaled_ad_lhs(137, A::sub_from_scalar(s.v[465], A::scaled_offset(s.ad_value(374), (-s.v[445]), p.p35)), 279, p.p36);

        s.store_div_from_scalar_scaled_input(120, 1.6021918e-19, 374, 1.3806226e-23);

        s.store_square(121, 120);

        s.store_div_from_scalar(122, 1.0, 120);

        s.v[464] = (1.6021918e-19 / (1.3806226e-23 * s.v[445]));

        s.v[676] = (((p.p249 * (1.0 + (p.p95 / ((s.v[376]) as f64).powf(p.p96)))) * (1.0 + (p.p97 / ((s.v[375]) as f64).powf(p.p98)))) * (1.0 + (p.p99 / ((s.v[377]) as f64).powf(p.p100))));

        s.v[677] = (((p.p276 * (1.0 + (p.p277 / ((s.v[376]) as f64).powf(p.p278)))) * (1.0 + (p.p281 / ((s.v[375]) as f64).powf(p.p282)))) * (1.0 + (p.p279 / ((s.v[377]) as f64).powf(p.p280))));

        s.b[681] = (s.v[458] > 0.0);
        s.v[681] = if s.b[681] { 1.0 } else { 0.0 };

        if s.b[681] {
            s.store_scalar(678, (1.0 / (1.0 + p.p163)));
            s.store_powf_ad(679, A::div_from_scalar(p.p162, s.ad_value(458)), p.p164);
            s.store_scalar(680, (((p.p162 / s.v[459])) as f64).powf(p.p164));
            s.store_div_scaled_offset_numerator(676, A::mul(s.ad_value(678), s.ad_value(679)), s.v[676], s.v[676], A::offset(A::mul(s.ad_value(678), s.ad_value(680)), 1.0), 1.0);
            s.store_div_scaled_offset_numerator(677, A::mul(s.ad_value(678), s.ad_value(679)), s.v[677], s.v[677], A::offset(A::mul(s.ad_value(678), s.ad_value(680)), 1.0), 1.0);
        }

        s.v[678] = (1.0 + (p.p112 / ((s.v[375]) as f64).powf(p.p113)));

        s.store_offset_ad(378, A::mul_scaled_lhs(A::scale_offset(s.ad_value(374), 1.0 / (s.v[445]), (-1.0)), p.p253, A::scale_offset(s.ad_value(374), 1.0 / (s.v[445]), (-1.0))), (p.p111 * s.v[678]));

        s.store_pow_ad(678, A::scale(s.ad_value(374), 1.0 / (s.v[445])), s.ad_value(378));

        s.store_div(469, 678, 676);

        s.store_div(595, 678, 677);

        s.store_mul(380, 478, 122);

        s.v[279] = ((((1.0 + (p.p181 / ((s.v[375]) as f64).powf(p.p182))) * (1.0 + (p.p185 / ((s.v[375]) as f64).powf(p.p186)))) * (1.0 + (p.p187 / ((s.v[376]) as f64).powf(p.p188)))) * (1.0 + (p.p183 / ((s.v[377]) as f64).powf(p.p184))));

        s.v[639] = ((((s.v[279] * s.v[279]) + ((4.0 * 0.001) * 0.001))) as f64).sqrt();

        s.v[280] = (0.5 * (1.0 + (s.v[279] / s.v[639])));

        s.v[480] = ((0.5 * (s.v[279] + s.v[639])) + (1e-10 * 0.001));

        s.b[682] = (s.v[480] < 0.0);
        s.v[682] = if s.b[682] { 1.0 } else { 0.0 };

        if s.b[682] {
            s.store_scalar(480, 0.0);
            s.store_scalar(280, 0.0);
        }

        s.store_scale(279, 374, 1.0 / (s.v[445]));

        s.v[280] = (1.0 + (p.p102 / ((s.v[375]) as f64).powf(p.p103)));

        s.store_div_scaled_inputs(162, s.ad_value(480), (s.v[613] * 0.01), A::sub(A::add_scaled_product(A::scale_offset(s.ad_value(279), (0.4 * 0.01), (1.8 * 0.01)), 1.0, s.ad_value(279), s.ad_value(279), (0.1 * 0.01)), A::scale_offset(s.ad_value(279), (-(s.v[615] * s.v[280])), (s.v[615] * s.v[280]))), 1.0);

        s.store_sqrt(245, 137);

        s.store_mul(246, 137, 245);

        s.store_scaled_mul_ad(127, A::powf(A::scale(s.ad_value(374), 1.0 / (s.v[445])), 1.5), A::exp(A::offset(A::mul_scaled_lhs(s.ad_value(137), (-1.0 / (2.0)), s.ad_value(120)), ((s.v[465] / 2.0) * s.v[464]))), 1.04e16);

        s.v[117] = (((((2.0 * 1.6021918e-19) * s.v[452]) * 1.034943e-10)) as f64).sqrt();

        s.v[118] = (1.0 / (s.v[452] * s.v[452]));

        s.store_scaled_sqrt(100, 122, s.v[117]);

        s.store_square(119, 100);

        s.store_scaled_square(101, 127, s.v[118]);

        s.v[279] = ((p.p38 / (p.p251 + p.p252)) * p.p0);

        s.v[281] = ((((p.p38 * 0.001) + ((10.0 * 2.220446049250313e-16) / 100.0))) as f64).abs();

        s.b[683] = (p.p38 > 0.0);
        s.v[683] = if s.b[683] { 1.0 } else { 0.0 };

        if s.b[683] {
            s.store_scalar(638, ((p.p38 - s.v[279]) - s.v[281]));
            s.store_scalar(639, ((4.0 * p.p38) * s.v[281]));
        }

        if s.b[683] {
            if (s.v[639] > 0.0) {
            } else {
                s.store_neg(639, 639);
            }
        }

        if s.b[683] {
            s.store_sqrt_square_add(639, 638, 639);
            s.store_offset_scaled_div(279, 638, 639, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(280, 638, (-0.5), 639, (-0.5), p.p38);
        }

        if (!s.b[683]) {
            s.store_offset(638, 279, (((-p.p38)) + ((-s.v[281]))));
            s.store_scalar(639, ((4.0 * p.p38) * s.v[281]));
        }

        if (!s.b[683]) {
            if (s.v[639] > 0.0) {
            } else {
                s.store_neg(639, 639);
            }
        }

        if (!s.b[683]) {
            s.store_sqrt_square_add(639, 638, 639);
            s.store_offset_scaled_div(279, 638, 639, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(280, 638, 0.5, 639, 0.5, p.p38);
        }

        s.store_sub_from_scalar_scaled_input(123, p.p0, 280, 2.0);

        s.v[279] = ((-p.p49) * (1.0 + (p.p50 / ((s.v[375]) as f64).powf(p.p51))));

        s.v[280] = ((-p.p49) * (1.0 + (p.p52 / ((s.v[375]) as f64).powf(p.p53))));

        s.v[281] = (-(p.p49 + (p.p54 * s.v[375])));

        s.v[638] = ((s.v[279] - s.v[280]) - 1e-12);

        s.v[639] = ((4.0 * s.v[280]) * 1e-12);

        if (!(s.v[639] > 0.0)) {
            s.store_scalar(639, (-s.v[639]));
        }

        s.store_sqrt_offset_input(639, 639, (s.v[638] * s.v[638]));

        s.store_scaled_offset_ad(279, A::div_from_scalar(s.v[638], s.ad_value(639)), 1.0, 0.5);

        s.store_offset_scaled(138, 639, 0.5, ((((s.v[638]) * (0.5))) + (s.v[280])));

        s.store_offset(638, 138, (((-s.v[281])) + ((-1e-12))));

        s.v[639] = ((4.0 * s.v[281]) * 1e-12);

        if (!(s.v[639] > 0.0)) {
            s.store_scalar(639, (-s.v[639]));
        }

        s.store_sqrt_square_add(639, 638, 639);

        s.store_offset_scaled_div(279, 638, 639, 0.5, 0.5);

        s.store_offset_add_scaled_inputs_indices(138, 638, 0.5, 639, 0.5, s.v[281]);

        s.store_neg(138, 138);

        s.store_mul_scaled_ln_ad_rhs(128, 122, 2.0, A::div(s.ad_value(471), s.ad_value(127)));

        s.store_sqrt_mul_ad(125, A::div_from_scalar(1.034943e-10, s.ad_value(126)), s.ad_value(122));

        s.store_scaled_mul(141, 126, 125, 1.414213562373095);

        s.copy_ad(438, 474);

    }

    pub(super) fn stamp_reactive_block_2(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.store_sqrt_ad(439, A::mul_scaled_lhs(s.ad_value(438), 2.0, s.ad_value(122)));

        s.store_div(279, 127, 471);

        s.store_square(142, 279);

        s.store_div(279, 127, 462);

        s.store_square(143, 279);

        s.v[272] = p.p226;

        s.v[273] = (3.453133e-11 / s.v[272]);

        s.v[274] = (s.v[272] / 3.453133e-11);

        s.v[294] = (3.453133e-11 / p.p229);

        s.v[295] = (p.p229 / 3.453133e-11);

        s.store_scale(296, 471, ((-1.6021918e-19) * p.p227));

        s.v[535] = (1.034943e-10 / p.p227);

        s.v[536] = (1.0 / s.v[535]);

        s.v[293] = (s.v[295] + s.v[536]);

        s.v[31] = p.p254;

        s.v[30] = p.p255;

        s.b[688] = (s.v[31] > (s.v[30] * 0.5));
        s.v[688] = if s.b[688] { 1.0 } else { 0.0 };

        if s.b[688] {
            s.store_scalar(31, (0.5 * s.v[30]));
        }

        s.b[689] = (s.v[47] > s.v[31]);
        s.v[689] = if s.b[689] { 1.0 } else { 0.0 };

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
        s.v[690] = if s.b[690] { 1.0 } else { 0.0 };

        s.b[691] = (4.0 == 1.0);
        s.v[691] = if s.b[691] { 1.0 } else { 0.0 };

        if ((s.b[689] && s.b[690]) && s.b[691]) {
            s.store_scalar(648, 1.0);
        }

        s.b[692] = (4.0 == 2.0);
        s.v[692] = if s.b[692] { 1.0 } else { 0.0 };

        if (((s.b[689] && s.b[690]) && (!s.b[691])) && s.b[692]) {
            s.store_scalar(648, 2.0);
        }

        s.b[693] = (4.0 == 4.0);
        s.v[693] = if s.b[693] { 1.0 } else { 0.0 };

        if ((((s.b[689] && s.b[690]) && (!s.b[691])) && (!s.b[692])) && s.b[693]) {
            s.store_scalar(648, 3.0);
        }

        s.b[694] = (4.0 == 8.0);
        s.v[694] = if s.b[694] { 1.0 } else { 0.0 };

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

        s.v[33] = 0.0;

        s.v[695] = 0.0;

        s.v[696] = 0.0;

        s.v[697] = 0.0;

        s.v[698] = 0.0;

        s.v[699] = 0.0;

        s.v[700] = 0.0;

        s.copy_ad(50, 43);

        s.copy_ad(51, 44);

        s.copy_ad(52, 45);

        s.v[62] = 0.0;

        s.v[63] = 0.0;

        s.store_scaled_mul(279, 46, 51, 0.5);

        s.store_scale(638, 279, (2.0 * 1.0 / (p.p216)));

        s.store_offset_mul_offset_rhs_ad_rhs(639, 638, A::mul_offset_rhs(s.ad_value(638), A::mul_offset_rhs(s.ad_value(638), A::mul_offset_rhs(s.ad_value(638), A::mul(s.ad_value(638), A::scale_offset(s.ad_value(638), (1.0 / 5040.0), (1.0 / 720.0))), (1.0 / 120.0)), (1.0 / 24.0)), (1.0 / 6.0)), (1.0 / 2.0), 1.0);

        s.store_offset_mul_offset_rhs_ad_rhs(640, 638, A::mul_offset_rhs(s.ad_value(638), A::mul_offset_rhs(s.ad_value(638), A::mul(s.ad_value(638), A::scale_offset(s.ad_value(638), (1.0 / 840.0), (1.0 / 144.0))), (1.0 / 30.0)), (1.0 / 8.0)), (1.0 / 3.0), (1.0 / 2.0));

        s.store_div_from_scalar(73, p.p216, 639);

        s.store_div_scaled_inputs(280, s.ad_value(640), (-2.0), A::square(s.ad_value(639)), 1.0);

        s.b[701] = (s.v[73] < 1e-12);
        s.v[701] = if s.b[701] { 1.0 } else { 0.0 };

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
        s.v[702] = if s.b[702] { 1.0 } else { 0.0 };

        if s.b[702] {
            s.store_scalar(282, 0.0);
            s.store_scalar(283, 0.0);
        }

        s.store_sqrt_offset_input(290, 282, 1e-50);

        s.store_add_ad_rhs(87, 280, A::mul_sub_from_scalar_rhs(s.ad_value(279), 1.0, s.ad_value(290)));

        s.store_sub(88, 87, 128);

        s.store_offset(638, 88, (((-0.1)) + ((-0.05))));

        s.v[639] = ((4.0 * 0.1) * 0.05);

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
        s.v[703] = if s.b[703] { 1.0 } else { 0.0 };

        if s.b[703] {
            s.store_scalar(37, 0.0);
        }

        if (!s.b[703]) {
            s.store_scalar(37, 1.0);
        }

        s.store_add_scaled_inputs3_sqrt_third_mixed_iia(275, 129, 1.0, 138, 1.0, A::mul_scaled_lhs(s.ad_value(126), (2.0 * 1.034943e-10), s.ad_value(129)), 1.0 / (s.v[273]));

        s.b[704] = (s.v[37] == 0.0);
        s.v[704] = if s.b[704] { 1.0 } else { 0.0 };

        if s.b[704] {
            s.store_scalar(268, s.v[272]);
            s.store_scalar(270, s.v[273]);
            s.store_scalar(271, s.v[274]);
            s.store_scale(278, 141, (s.v[274] * s.v[274]));
            s.store_mul(381, 278, 141);
        }

        if (!s.b[704]) {
            s.store_add_scaled_inputs3_offset(283, s.ad_value(52), 1.0, s.ad_value(50), (-1.0), s.ad_value(275), -1.0, p.p194);
            s.store_sqrt_square_offset(639, 283, ((4.0 * 0.0001) * 0.0001));
            s.store_offset_scaled_div(281, 283, 639, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(280, 283, 0.5, 639, 0.5, (1e-10 * 0.0001));
        }

        s.b[705] = (s.v[280] < 0.0);
        s.v[705] = if s.b[705] { 1.0 } else { 0.0 };

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
        s.v[706] = if s.b[706] { 1.0 } else { 0.0 };

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
            s.store_add_scaled_inputs3(280, A::div_from_scalar(1.0, s.ad_value(282)), 1.0, s.ad_value(638), (-0.5), s.ad_value(639), (-0.5));
            s.store_offset_scaled(269, 280, p.p193, p.p195);
        }

        s.b[707] = ((s.v[269] * 1000000000000.0) < s.v[272]);
        s.v[707] = if s.b[707] { 1.0 } else { 0.0 };

        if ((!s.b[704]) && s.b[707]) {
            s.store_scalar(269, 0.0);
            s.store_scalar(37, 0.0);
        }

        if (!s.b[704]) {
            s.store_offset(268, 269, s.v[272]);
            s.store_div_from_scalar(270, 3.453133e-11, 268);
            s.store_scale(271, 268, 28959208927.08158);
            s.store_mul_ad_product_lhs(381, A::square(s.ad_value(141)), s.ad_value(271), 271);
        }

        s.store_offset_sub_from_scalar_ad(638, 0.5, s.ad_value(70), (-0.001));

        s.v[639] = ((4.0 * 0.5) * 0.001);

        if (!(s.v[639] > 0.0)) {
            s.store_scalar(639, (-s.v[639]));
        }

        s.store_sqrt_square_add(639, 638, 639);

        s.store_offset_scaled_div(278, 638, 639, 0.5, 0.5);

        s.store_offset_add_scaled_inputs_indices(382, 638, (-0.5), 639, (-0.5), 0.5);

        s.store_sqrt_mul(150, 473, 129);

        s.store_add_ad_lhs(265, A::add_scaled_inputs_product(s.ad_value(129), 1.0, s.ad_value(138), 1.0, s.ad_value(150), s.ad_value(271), 1.0), 380);

        s.copy_ad(130, 129);

        s.v[278] = 0.95;

        s.store_offset_sub_scaled_inputs(279, s.ad_value(130), s.v[278], s.ad_value(382), 1.0, (-0.001));

        s.store_sqrt_add_scaled_square_input(280, 279, 1.0, 130, ((4.0 * s.v[278]) * 0.001));

        s.store_add_scaled_inputs4(131, s.ad_value(130), 1.0, s.ad_value(130), (-s.v[278]), s.ad_value(279), (-(-0.5)), s.ad_value(280), (-(-0.5)));

        s.store_sqrt(135, 131);

        s.b[708] = (p.p58 != 0.0);
        s.v[708] = if s.b[708] { 1.0 } else { 0.0 };

        if s.b[708] {
            s.store_sqrt_ad(278, A::mul_scaled_lhs(s.ad_value(471), ((2.0 * 1.6021918e-19) * 1.034943e-10), s.ad_value(136)));
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
            s.store_mul_ad_product_lhs(266, A::sub(s.ad_value(265), s.ad_value(79)), s.ad_value(81), 278);
        }

        if (!s.b[708]) {
            s.store_scalar(266, 0.0);
        }

        s.b[709] = (p.p297 != 0.0);
        s.v[709] = if s.b[709] { 1.0 } else { 0.0 };

        if s.b[709] {
            s.store_offset_add_ad(288, A::add_scaled_product(s.ad_value(122), 1.0, s.ad_value(381), s.ad_value(120), (-0.25)), s.ad_value(138), 1e-50);
            s.store_offset_sub(279, 72, 288, (-0.005));
        }

        if s.b[709] {
            s.store_scalar(278, (if (s.v[288] >= 0.0) { 1.0 } else { (-1.0) }));
        }

        if s.b[709] {
            s.store_sqrt_ad(280, A::add_scaled_square_product(s.ad_value(279), 1.0, s.ad_value(278), s.ad_value(288), (4.0 * 0.005)));
            s.store_add_scaled_inputs4(281, s.ad_value(288), 1.0, s.ad_value(279), 0.5, s.ad_value(280), 0.5, s.ad_value(138), -1.0);
            s.store_mul_ad_product_lhs(282, A::div_from_scalar(4.0, s.ad_value(381)), s.ad_value(122), 122);
            s.store_offset_mul(283, 120, 281, (-1.0));
            s.store_offset_mul(279, 283, 282, 1.0);
            s.store_sqrt_square_offset(639, 279, ((4.0 * 0.001) * 0.001));
            s.store_offset_scaled_div(285, 279, 639, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(279, 279, 0.5, 639, 0.5, (1e-10 * 0.001));
        }

        s.b[710] = (s.v[279] < 0.0);
        s.v[710] = if s.b[710] { 1.0 } else { 0.0 };

        if (s.b[709] && s.b[710]) {
            s.store_scalar(279, 0.0);
            s.store_scalar(285, 0.0);
        }

        if s.b[709] {
            s.store_sqrt_offset_input(280, 279, (10.0 * 2.220446049250313e-16));
            s.store_add_ad_rhs(139, 281, A::mul3_scaled_output(s.ad_value(381), s.ad_value(120), A::sub_from_scalar(1.0, s.ad_value(280)), 0.5));
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
            s.store_add_scaled_inputs3(140, s.ad_value(129), 1.0, s.ad_value(638), (-0.5), s.ad_value(639), (-0.5));
            s.store_add_scaled_inputs3(130, s.ad_value(129), 1.0, s.ad_value(140), p.p297, s.ad_value(129), (-p.p297));
        }

        s.store_scale(279, 271, (1.034943e-10 * (p.p227 * 2.0)));

        s.store_sub_from_scalar(280, p.p55, 130);

        s.v[281] = (s.v[277] - p.p57);

        s.store_scaled_mul(81, 279, 280, 1.0 / ((s.v[281] * s.v[281])));

        s.store_sqrt_square_offset(639, 50, ((4.0 * 0.001) * 0.001));

        s.store_offset_scaled_div(278, 50, 639, 0.5, 0.5);

        s.store_offset_add_scaled_inputs_indices(593, 50, 0.5, 639, 0.5, (1e-10 * 0.001));

        s.b[711] = (s.v[593] < 0.0);
        s.v[711] = if s.b[711] { 1.0 } else { 0.0 };

        if s.b[711] {
            s.store_scalar(593, 0.0);
            s.store_scalar(278, 0.0);
        }

        s.store_add_scaled_inputs3_offset(283, s.ad_value(131), (p.p71 / s.v[277]), s.ad_value(71), p.p70, s.ad_value(593), p.p250, p.p69);

        s.store_mul(82, 81, 283);

        s.b[712] = (p.p72 > 0.0);
        s.v[712] = if s.b[712] { 1.0 } else { 0.0 };

        if s.b[712] {
            s.store_add_scaled_inputs3_offset(279, s.ad_value(137), 1.0, s.ad_value(128), 1.0, s.ad_value(71), p.p73, (-(2.0 * p.p74)));
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

        s.store_add_scaled_inputs4_offset(80, s.ad_value(82), 1.0, s.ad_value(266), 1.0, s.ad_value(84), 1.0, s.ad_value(83), 1.0, s.v[482]);

        s.store_sub(78, 265, 80);

        s.b[713] = (p.p75 == 0.0);
        s.v[713] = if s.b[713] { 1.0 } else { 0.0 };

        if s.b[713] {
            s.store_scalar(36, 0.0);
        }

        if (!s.b[713]) {
            s.store_scalar(36, 1.0);
        }

        s.b[714] = (s.v[36] == 0.0);
        s.v[714] = if s.b[714] { 1.0 } else { 0.0 };

        if s.b[714] {
            s.store_scalar(267, 0.0);
        }

        if (!s.b[714]) {
            s.store_offset(281, 72, (-p.p76));
        }

        s.b[715] = (s.v[281] < (-3.0));
        s.v[715] = if s.b[715] { 1.0 } else { 0.0 };

        if ((!s.b[714]) && s.b[715]) {
            s.store_scalar(284, 0.0);
            s.store_scalar(267, 0.0);
        }

        s.b[716] = (s.v[281] < 0.0);
        s.v[716] = if s.b[716] { 1.0 } else { 0.0 };

        if (((!s.b[714]) && (!s.b[715])) && s.b[716]) {
            s.store_offset_mul_ad(284, s.ad_value(281), A::scale_offset(s.ad_value(281), (3.0 * (1.0 / 27.0)), (2.0 * (1.0 / 3.0))), 1.0);
            s.store_offset_mul_offset_rhs_ad_rhs(267, 281, A::mul(s.ad_value(281), A::scale_offset(s.ad_value(281), (1.0 / 27.0), (1.0 / 3.0))), 1.0, 1.0);
        }

        if (((!s.b[714]) && (!s.b[715])) && (!s.b[716])) {
            s.store_offset_mul_offset_rhs_ad_rhs(284, 281, A::mul(s.ad_value(281), A::scale_offset(s.ad_value(281), (4.0 * 0.148148111111111), (3.0 * 0.0402052934513951))), (2.0 * (1.0 / 3.0)), 1.0);
            s.store_offset_mul_offset_rhs_ad_rhs(267, 281, A::mul_offset_rhs(s.ad_value(281), A::mul(s.ad_value(281), A::scale_offset(s.ad_value(281), 0.148148111111111, 0.0402052934513951)), (1.0 / 3.0)), 1.0, 1.0);
        }

        if (!s.b[714]) {
            s.store_sqrt_offset_ad(639, A::mul_offset_lhs(s.ad_value(267), (-1.0), A::offset(s.ad_value(267), (-1.0))), ((4.0 * 0.1) * 0.1));
            s.store_scaled_offset_ad(284, A::div_scaled_offset_numerator(s.ad_value(267), 1.0, (-1.0), s.ad_value(639), 1.0), 1.0, 0.5);
            s.store_offset_add_scaled_inputs_mixed_ai(267, A::offset(s.ad_value(267), (-1.0)), 0.5, 639, 0.5, (1e-10 * 0.1));
        }

        s.b[717] = (s.v[267] < 0.0);
        s.v[717] = if s.b[717] { 1.0 } else { 0.0 };

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

        s.store_add_scaled_inputs4(53, s.ad_value(52), 1.0, s.ad_value(138), (-1.0), s.ad_value(80), 1.0, s.ad_value(267), -1.0);

        s.copy_ad(76, 53);

        s.store_mul_ln_ad_rhs(298, 122, A::div(s.ad_value(471), s.ad_value(462)));

        s.store_add_scaled_inputs3(54, s.ad_value(138), 1.0, s.ad_value(80), (-1.0), s.ad_value(267), 1.0);

        s.store_mul(144, 141, 271);

        s.store_square(145, 144);

        if (p.p29 != 0.0) {
            s.store_add(440, 70, 298);
        }

        if (p.p29 == 0.0) {
            s.store_add(440, 50, 298);
        }

        s.b[718] = (s.v[440] < 0.0);
        s.v[718] = if s.b[718] { 1.0 } else { 0.0 };

        if s.b[718] {
            s.store_div(278, 462, 471);
            s.store_offset(279, 278, 1.0);
            s.store_add_scaled_inputs_product_right_ad(280, 122, 1.0, 440, (-1.0), 278, A::add(s.ad_value(122), s.ad_value(440)), 1.0);
            s.store_scaled_square(281, 439, (s.v[295] * s.v[295]));
            s.store_add_scaled_products_indices(282, 280, 279, 2.0, 281, 120, (-1.0));
            s.store_add_scaled_inputs3(283, A::square(s.ad_value(280)), 1.0, A::mul3(s.ad_value(281), s.ad_value(120), s.ad_value(440)), 1.0, s.ad_value(281), 1.0);
        }

        if s.b[718] {
            if (((s.v[282] * s.v[282]) - (((4.0 * s.v[279]) * s.v[279]) * s.v[283])) >= 1e-50) {
                s.store_sub_ad(285, A::square(s.ad_value(282)), A::mul3_scaled_output(s.ad_value(279), s.ad_value(279), s.ad_value(283), 4.0));
            } else {
                s.store_scalar(285, 1e-50);
            }
        }

        if s.b[718] {
            s.store_div_scaled_inputs2(331, s.ad_value(282), 1.0, A::sqrt(s.ad_value(285)), 1.0, A::offset(A::square(s.ad_value(279)), 2.0), 1.0);
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
            s.store_div_scaled_inputs2(331, s.ad_value(284), 1.0, A::sqrt(s.ad_value(285)), 1.0, A::mul_scaled_lhs(s.ad_value(282), 2.0, s.ad_value(282)), 1.0);
        }

        s.store_mul_ad(326, A::div_from_scalar(2.0, s.ad_value(120)), A::ln(A::div(s.ad_value(462), s.ad_value(127))));

        s.store_scaled_square(278, 439, (s.v[293] * s.v[293]));

        s.store_neg(279, 440);

        s.store_add_scaled_inputs_product_mixed_aiaa(280, A::square(s.ad_value(279)), (4.0 * (-1.0)), 278, (4.0 * (-1.0)), A::add_scaled_product(s.ad_value(279), 2.0, s.ad_value(278), s.ad_value(120), 1.0), A::add_scaled_product(s.ad_value(279), 2.0, s.ad_value(278), s.ad_value(120), 1.0), 1.0);

        if (!(s.v[280] >= (10.0 * 2.220446049250313e-16))) {
            s.store_scalar(280, (10.0 * 2.220446049250313e-16));
        }

        s.store_sqrt(280, 280);

        s.store_add_scaled_product_indices(281, 279, 2.0, 278, 120, 1.0);

        s.store_scaled_sub(324, 281, 280, 0.5);

        s.store_div_ad(325, A::ln(A::div_scaled_product_by_product(s.ad_value(279), s.ad_value(279), 1.0, s.ad_value(278), s.ad_value(143), 1.0)), A::add(s.ad_value(120), A::div_from_scalar(2.0, s.ad_value(279))));

        s.b[719] = (s.v[324] < s.v[326]);
        s.v[719] = if s.b[719] { 1.0 } else { 0.0 };

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
            s.store_add_scaled_inputs3(331, s.ad_value(325), 1.0, s.ad_value(638), (-0.5), s.ad_value(639), (-0.5));
        }

        s.v[62] = 0.0;

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
            s.v[720] = if s.b[720] { 1.0 } else { 0.0 };
            if s.b[720] {
                s.store_exp_mul(278, 120, 331);
                s.store_mul_scaled_sqrt_ad_rhs(282, 279, -1.0, A::add_scaled_offset_product_rhs(A::offset(A::add(s.ad_value(281), s.ad_value(280)), (-1.0)), 1.0, s.ad_value(143), s.ad_value(278), (-1.0), 1.0));
                s.store_mul_ad(283, A::div(s.ad_value(438), s.ad_value(282)), A::add_scaled_sub_value_product(1.0, s.ad_value(281), 1.0, s.ad_value(143), s.ad_value(278), 1.0));
            }
            s.b[721] = (s.v[331] < (-1e-8));
            s.v[721] = if s.b[721] { 1.0 } else { 0.0 };
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
            s.v[722] = if s.b[722] { 1.0 } else { 0.0 };
            if s.b[722] {
                s.store_scalar(284, 0.0);
                s.store_scalar(285, 0.0);
            }
            s.store_offset_sub_scaled_inputs(638, s.ad_value(296), -1.0, s.ad_value(284), 1.0, (-1e-9));
            s.store_scale(639, 296, (-(4.0 * 1e-9)));
            if (!(s.v[639] > 0.0)) {
                s.store_neg(639, 639);
            }
            s.store_sqrt_square_add(639, 638, 639);
            s.store_offset_scaled_div(286, 638, 639, 0.5, 0.5);
            s.store_add_scaled_inputs3(284, s.ad_value(296), -1.0, s.ad_value(638), (-0.5), s.ad_value(639), (-0.5));
            s.store_mul3_lhs(285, 285, 283, 286);
            s.store_div_scaled_inputs(334, A::square(s.ad_value(284)), ((0.5 * 9662367879.197212) * 6.241449993689894e18), s.ad_value(471), 1.0);
            s.store_div_scaled_product_indices(335, 334, 285, 2.0, 284, 1.0);
            s.store_sub_ad_rhs(284, 331, A::div_scaled_inputs4(s.ad_value(282), 1.0 / (s.v[294]), s.ad_value(331), (-1.0), s.ad_value(440), -1.0, s.ad_value(334), 1.0, A::add(A::scale_offset(s.ad_value(283), 1.0 / (s.v[294]), (-1.0)), s.ad_value(335)), 1.0));
            s.b[723] = ((((s.v[284] - s.v[331])) as f64).abs() < 0.001);
            s.v[723] = if s.b[723] { 1.0 } else { 0.0 };
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
        s.store_sqrt_ad(279, A::div_scaled_inputs(s.ad_value(332), ((2.0 * 1.034943e-10) / 1.6021918e-19), s.ad_value(471), 1.0));

        s.b[724] = (s.v[279] > (0.99 * p.p227));
        s.v[724] = if s.b[724] { 1.0 } else { 0.0 };

        if s.b[724] {
            s.store_div_from_scalar(278, 1.0, 270);
            s.store_scalar(280, (1.0 / s.v[294]));
            s.store_div_from_scalar_add_ad(281, 1.0, A::offset(s.ad_value(278), s.v[536]), s.ad_value(280));
            s.store_sub_from_scalar_ad(282, 1.0, A::mul(s.ad_value(281), s.ad_value(278)));
            s.store_mul_ad_product_rhs(283, 278, s.ad_value(281), A::sub(A::mul_scaled_rhs(A::offset(s.ad_value(280), (0.5 * s.v[536])), s.ad_value(296), -1.0), s.ad_value(440)));
            s.store_div(327, 283, 282);
            s.store_add(54, 54, 327);
            s.store_sub_scaled_inputs(53, 53, 1.0, 327, p.p298);
            s.copy_ad(76, 53);
        }

        s.b[725] = (s.v[33] >= 1.0);
        s.v[725] = if s.b[725] { 1.0 } else { 0.0 };

        if s.b[725] {
            s.store_scalar(305, s.v[695]);
            s.store_scalar(306, s.v[696]);
            s.store_offset(307, 440, s.v[697]);
            s.store_add_scaled_inputs(328, 296, (-(s.v[536] * 0.5)), 122, 1.0);
            s.store_sub_scaled_inputs(329, 328, 1.0, 330, s.v[536]);
        }

        s.b[726] = (s.v[440] < 0.0);
        s.v[726] = if s.b[726] { 1.0 } else { 0.0 };

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
                s.store_add_scaled_inputs3(280, s.ad_value(296), ((-(0.5 * s.v[536])) * s.v[294]), s.ad_value(122), s.v[294], s.ad_value(440), s.v[294]);
                s.store_mul3_affine_lhs(285, 278, 270, 2.0, 0.0, 270);
                s.store_add_scaled_inputs_product_mixed_aaii(282, A::offset(A::mul(s.ad_value(279), s.ad_value(270)), s.v[294]), 1.0, A::mul3_scaled_output(s.ad_value(278), s.ad_value(270), s.ad_value(296), 2.0), 1.0, 285, 55, 1.0);
                s.store_mul3_affine_lhs(286, 270, 278, ((2.0 * s.v[294]) * 2.0), 0.0, 270);
                s.store_add_scaled_value_products(283, A::offset(A::mul3(A::add_scaled_square_product(s.ad_value(279), 1.0, s.ad_value(278), s.ad_value(280), (-4.0)), s.ad_value(270), s.ad_value(270)), (s.v[294] * s.v[294])), 1.0, s.ad_value(270), A::add_scaled_product(s.ad_value(279), 1.0, s.ad_value(278), s.ad_value(296), 2.0), (2.0 * s.v[294]), s.ad_value(286), s.ad_value(55), 1.0);
                s.store_sqrt(283, 283);
                s.store_div_scaled_inputs(286, s.ad_value(286), 1.0, s.ad_value(283), 2.0);
                s.store_div_from_scalar_ad(284, 1.0, A::mul3_scaled_output(s.ad_value(278), s.ad_value(270), s.ad_value(270), 2.0));
                s.store_mul_sub_rhs(346, 284, 282, 283);
                s.store_mul_sub_rhs(347, 284, 285, 286);
                s.store_div_scaled_inputs(370, s.ad_value(346), -1.0, s.ad_value(347), 1.0);
            }
            s.b[727] = (((s.v[370]) as f64).abs() < 1e-12);
            s.v[727] = if s.b[727] { 1.0 } else { 0.0 };
            if (((!s.b[725]) && s.b[726]) && s.b[727]) {
                s.store_scalar(62, s.v[28]);
            }
            s.b[728] = (s.v[370] > 0.1);
            s.v[728] = if s.b[728] { 1.0 } else { 0.0 };
            if ((((!s.b[725]) && s.b[726]) && (!s.b[727])) && s.b[728]) {
                s.store_scalar(370, 0.1);
            }
            s.b[729] = (s.v[370] < (-0.1));
            s.v[729] = if s.b[729] { 1.0 } else { 0.0 };
            if (((((!s.b[725]) && s.b[726]) && (!s.b[727])) && (!s.b[728])) && s.b[729]) {
                s.store_scalar(370, (-0.1));
            }
            if ((!s.b[725]) && s.b[726]) {
                s.store_add(55, 55, 370);
                s.store_offset(62, 62, 1.0);
            }
        }

        s.b[730] = (s.v[52] < (s.v[54] + s.v[55]));
        s.v[730] = if s.b[730] { 1.0 } else { 0.0 };

        if ((!s.b[725]) && s.b[730]) {
            s.store_scalar(39, 1.0);
            s.store_scalar(292, (-1.0));
            s.copy_ad(332, 334);
            s.store_sqrt_ad(279, A::div_scaled_inputs(s.ad_value(332), ((2.0 * 1.034943e-10) / 1.6021918e-19), s.ad_value(471), 1.0));
            s.store_scaled_square(278, 439, (s.v[293] * s.v[293]));
        }

        s.b[731] = ((s.v[345] + s.v[279]) < p.p227);
        s.v[731] = if s.b[731] { 1.0 } else { 0.0 };

        if (((!s.b[725]) && s.b[730]) && s.b[731]) {
            s.store_sub_from_scalar(279, (10.0 * 2.220446049250313e-16), 440);
            s.store_add_scaled_inputs_product_mixed_aiaa(280, A::square(s.ad_value(279)), (4.0 * (-1.0)), 278, (4.0 * (-1.0)), A::add_scaled_product(s.ad_value(279), 2.0, s.ad_value(278), s.ad_value(120), 1.0), A::add_scaled_product(s.ad_value(279), 2.0, s.ad_value(278), s.ad_value(120), 1.0), 1.0);
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
        s.v[732] = if s.b[732] { 1.0 } else { 0.0 };

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
            s.store_add_scaled_inputs3(307, s.ad_value(325), 1.0, s.ad_value(638), (-0.5), s.ad_value(639), (-0.5));
        }

        if (((!s.b[725]) && s.b[730]) && (!s.b[731])) {
            s.store_neg_ad(279, A::add_scaled_inputs3(s.ad_value(440), 1.0, s.ad_value(305), (-1.0), s.ad_value(296), (-(0.5 * (p.p227 * 9662367879.197212)))));
            s.store_add_scaled_inputs_product_mixed_aiaa(280, A::square(s.ad_value(279)), (4.0 * (-1.0)), 278, (4.0 * (-1.0)), A::add_scaled_product(s.ad_value(279), 2.0, s.ad_value(278), s.ad_value(120), 1.0), A::add_scaled_product(s.ad_value(279), 2.0, s.ad_value(278), s.ad_value(120), 1.0), 1.0);
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
        s.v[733] = if s.b[733] { 1.0 } else { 0.0 };

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
            s.store_add_scaled_inputs3(307, s.ad_value(325), 1.0, s.ad_value(638), (-0.5), s.ad_value(639), (-0.5));
        }

        if ((!s.b[725]) && s.b[730]) {
            s.store_sqrt_ad(279, A::div_scaled_inputs(s.ad_value(332), ((2.0 * 1.034943e-10) / 1.6021918e-19), s.ad_value(471), 1.0));
        }

        s.b[734] = ((s.v[345] + s.v[279]) < p.p227);
        s.v[734] = if s.b[734] { 1.0 } else { 0.0 };

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
            s.v[735] = if s.b[735] { 1.0 } else { 0.0 };
            if ((((!s.b[725]) && s.b[730]) && s.b[734]) && s.b[735]) {
                s.store_exp_mul(278, 120, 307);
                s.store_mul_scaled_sqrt_ad_rhs(282, 279, -1.0, A::add_scaled_offset_product_rhs(A::offset(A::add(s.ad_value(281), s.ad_value(280)), (-1.0)), 1.0, s.ad_value(143), s.ad_value(278), (-1.0), 1.0));
                s.store_mul_ad(283, A::div(s.ad_value(438), s.ad_value(282)), A::add_scaled_sub_value_product(1.0, s.ad_value(281), 1.0, s.ad_value(143), s.ad_value(278), 1.0));
            }
            s.b[736] = (s.v[307] < (-1e-8));
            s.v[736] = if s.b[736] { 1.0 } else { 0.0 };
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
            s.v[737] = if s.b[737] { 1.0 } else { 0.0 };
            if ((((!s.b[725]) && s.b[730]) && s.b[734]) && s.b[737]) {
                s.store_scalar(284, 0.0);
                s.store_scalar(285, 0.0);
            }
            if (((!s.b[725]) && s.b[730]) && s.b[734]) {
                s.store_offset_sub_scaled_inputs(638, s.ad_value(296), -1.0, s.ad_value(284), 1.0, (-1e-13));
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
                s.store_add_scaled_inputs3(284, s.ad_value(296), -1.0, s.ad_value(638), (-0.5), s.ad_value(639), (-0.5));
                s.store_mul3_lhs(285, 285, 283, 286);
                s.store_div_scaled_inputs(332, A::square(s.ad_value(284)), ((0.5 * 9662367879.197212) * 6.241449993689894e18), s.ad_value(471), 1.0);
                s.store_div_scaled_product_indices(333, 332, 285, 2.0, 284, 1.0);
                s.store_sub_ad_rhs(284, 307, A::div_scaled_inputs4(s.ad_value(282), 1.0 / (s.v[294]), s.ad_value(307), (-1.0), s.ad_value(440), -1.0, s.ad_value(332), 1.0, A::add(A::scale_offset(s.ad_value(283), 1.0 / (s.v[294]), (-1.0)), s.ad_value(333)), 1.0));
            }
            s.b[738] = ((((s.v[284] - s.v[307])) as f64).abs() < 0.001);
            s.v[738] = if s.b[738] { 1.0 } else { 0.0 };
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
            s.v[739] = if s.b[739] { 1.0 } else { 0.0 };
            if ((((!s.b[725]) && s.b[730]) && (!s.b[734])) && s.b[739]) {
                s.store_exp_mul(278, 120, 307);
                s.store_mul_scaled_sqrt_ad_rhs(282, 279, -1.0, A::add_scaled_offset_product_rhs(A::offset(A::add(s.ad_value(281), s.ad_value(280)), (-1.0)), 1.0, s.ad_value(143), s.ad_value(278), (-1.0), 1.0));
                s.store_mul_ad(283, A::div(s.ad_value(438), s.ad_value(282)), A::add_scaled_sub_value_product(1.0, s.ad_value(281), 1.0, s.ad_value(143), s.ad_value(278), 1.0));
            }
            s.b[740] = (s.v[307] < (-1e-8));
            s.v[740] = if s.b[740] { 1.0 } else { 0.0 };
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
            s.v[741] = if s.b[741] { 1.0 } else { 0.0 };
            if ((((!s.b[725]) && s.b[730]) && (!s.b[734])) && s.b[741]) {
                s.store_scalar(284, 0.0);
                s.store_scalar(285, 0.0);
            }
            if (((!s.b[725]) && s.b[730]) && (!s.b[734])) {
                s.store_offset_sub_scaled_inputs(638, s.ad_value(296), -1.0, s.ad_value(284), 1.0, (-1e-13));
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
                s.store_add_scaled_inputs3(284, s.ad_value(296), -1.0, s.ad_value(638), (-0.5), s.ad_value(639), (-0.5));
                s.store_mul3_lhs(285, 285, 283, 286);
                s.store_div_scaled_inputs(332, A::square(s.ad_value(284)), ((0.5 * 9662367879.197212) * 6.241449993689894e18), s.ad_value(471), 1.0);
                s.store_div_scaled_product_indices(333, 332, 285, 2.0, 284, 1.0);
            }
            if (((!s.b[725]) && s.b[730]) && (!s.b[734])) {
                let assign7580_body27_ad_e7124: A = A::div(A::add(A::sub(A::add(A::add_scaled_inputs3(s.ad_value(305), 1.0, s.ad_value(307), (-1.0), s.ad_value(282), 1.0 / (s.v[294])), A::add_scaled_inputs(s.ad_value(282), (p.p227 * 9662367879.197212), s.ad_value(296), (0.5 * (p.p227 * 9662367879.197212)))), s.ad_value(440)), s.ad_value(332)), A::add_scaled_inputs3_offset(s.ad_value(283), 1.0 / (s.v[294]), s.ad_value(283), (p.p227 * 9662367879.197212), s.ad_value(333), 1.0, (-1.0)));
                s.store_sub_ad_rhs(284, 307, assign7580_body27_ad_e7124);
            }
            s.b[742] = ((((s.v[284] - s.v[307])) as f64).abs() < 0.001);
            s.v[742] = if s.b[742] { 1.0 } else { 0.0 };
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
            s.store_add_ad_rhs(319, 76, A::mul3_scaled_output(s.ad_value(145), s.ad_value(120), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(290))), 0.5));
            s.store_div_from_scalar(278, 1.0, 270);
            s.store_scalar(279, (p.p227 / 1.034943e-10));
            s.store_scalar(280, (1.0 / s.v[294]));
            s.store_div_from_scalar_ad(281, 1.0, A::add_scaled_inputs3(s.ad_value(278), 1.0, s.ad_value(279), 1.0, s.ad_value(280), 1.0));
        }

        s.b[743] = ((s.v[52] - s.v[327]) <= s.v[78]);
        s.v[743] = if s.b[743] { 1.0 } else { 0.0 };

        if ((!s.b[725]) && s.b[743]) {
            if (s.v[319] > 0.0) {
                s.store_sqrt_ad(283, A::mul_scaled_lhs(s.ad_value(471), ((1.6021918e-19 * 2.0) * 1.034943e-10), s.ad_value(319)));
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
        s.v[744] = if s.b[744] { 1.0 } else { 0.0 };

        if ((!s.b[725]) && s.b[744]) {
            s.store_div_scalar_by_product(279, 1.0, s.ad_value(142), s.ad_value(381), 1.0);
            s.store_mul_ad_product_rhs(280, 279, A::sub(s.ad_value(76), s.ad_value(327)), A::sub(s.ad_value(76), s.ad_value(327)));
            s.store_add_ad_rhs(281, 120, A::div_from_scalar(2.0, A::sub(s.ad_value(76), s.ad_value(327))));
            s.store_div_ad_lhs(320, A::ln(s.ad_value(280)), 281);
        }

        s.b[745] = ((s.v[319] > (s.v[320] - 0.15)) && (0.15 >= 0.0));
        s.v[745] = if s.b[745] { 1.0 } else { 0.0 };

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
        s.v[746] = if s.b[746] { 1.0 } else { 0.0 };

        s.b[747] = (1.0 == 1.0);
        s.v[747] = if s.b[747] { 1.0 } else { 0.0 };

        if (((((!s.b[725]) && s.b[744]) && s.b[745]) && s.b[746]) && s.b[747]) {
            s.store_scalar(648, 1.0);
        }

        s.b[748] = (1.0 == 2.0);
        s.v[748] = if s.b[748] { 1.0 } else { 0.0 };

        if ((((((!s.b[725]) && s.b[744]) && s.b[745]) && s.b[746]) && (!s.b[747])) && s.b[748]) {
            s.store_scalar(648, 2.0);
        }

        s.b[749] = (1.0 == 4.0);
        s.v[749] = if s.b[749] { 1.0 } else { 0.0 };

        if (((((((!s.b[725]) && s.b[744]) && s.b[745]) && s.b[746]) && (!s.b[747])) && (!s.b[748])) && s.b[749]) {
            s.store_scalar(648, 3.0);
        }

        s.b[750] = (1.0 == 8.0);
        s.v[750] = if s.b[750] { 1.0 } else { 0.0 };

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
            s.store_add_ad_lhs(321, A::offset(s.ad_value(320), (-0.15)), 637);
        }

        if (((!s.b[725]) && s.b[744]) && s.b[745]) {
        }

        if (((!s.b[725]) && s.b[744]) && (!s.b[745])) {
            s.copy_ad(321, 319);
            s.store_scalar(279, 1.0);
        }

        if (!s.b[725]) {
            if (s.v[321] > 0.0) {
                s.store_sqrt_ad(345, A::div_scaled_inputs(s.ad_value(321), ((2.0 * 1.034943e-10) / 1.6021918e-19), s.ad_value(471), 1.0));
            } else {
                s.store_scalar(345, 0.0);
            }
        }

        s.b[751] = (s.v[345] < p.p227);
        s.v[751] = if s.b[751] { 1.0 } else { 0.0 };

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
        s.v[752] = if s.b[752] { 1.0 } else { 0.0 };

        if ((!s.b[725]) && s.b[752]) {
            s.store_neg(279, 440);
            s.store_add_scaled_inputs_product_mixed_aiaa(280, A::square(s.ad_value(279)), (4.0 * (-1.0)), 278, (4.0 * (-1.0)), A::add_scaled_product(s.ad_value(279), 2.0, s.ad_value(278), s.ad_value(120), 1.0), A::add_scaled_product(s.ad_value(279), 2.0, s.ad_value(278), s.ad_value(120), 1.0), 1.0);
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
        s.v[753] = if s.b[753] { 1.0 } else { 0.0 };

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
            s.store_add_scaled_inputs3(307, s.ad_value(325), 1.0, s.ad_value(638), (-0.5), s.ad_value(639), (-0.5));
        }

        if ((!s.b[725]) && (!s.b[752])) {
            s.store_neg_ad(279, A::add_scaled_inputs3(s.ad_value(440), 1.0, s.ad_value(305), (-1.0), s.ad_value(296), (-(0.5 * (p.p227 * 9662367879.197212)))));
            s.store_add_scaled_inputs_product_mixed_aiaa(280, A::square(s.ad_value(279)), (4.0 * (-1.0)), 278, (4.0 * (-1.0)), A::add_scaled_product(s.ad_value(279), 2.0, s.ad_value(278), s.ad_value(120), 1.0), A::add_scaled_product(s.ad_value(279), 2.0, s.ad_value(278), s.ad_value(120), 1.0), 1.0);
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
        s.v[754] = if s.b[754] { 1.0 } else { 0.0 };

        if (((!s.b[725]) && (!s.b[752])) && s.b[754]) {
            s.copy_ad(307, 324);
        }

        if (((!s.b[725]) && (!s.b[752])) && (!s.b[754])) {
            s.store_offset_sub(638, 325, 324, (-0.0008));
            s.store_scale(639, 325, (4.0 * 0.0008));
        }

    }

    pub(super) fn stamp_reactive_block_6(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((!s.b[725]) && (!s.b[752])) && (!s.b[754])) {
            if (s.v[639] > 0.0) {
            } else {
                s.store_neg(639, 639);
            }
        }

        if (((!s.b[725]) && (!s.b[752])) && (!s.b[754])) {
            s.store_sqrt_square_add(639, 638, 639);
            s.store_offset_scaled_div(279, 638, 639, 0.5, 0.5);
            s.store_add_scaled_inputs3(307, s.ad_value(325), 1.0, s.ad_value(638), (-0.5), s.ad_value(639), (-0.5));
        }

        s.b[755] = ((s.v[39] == 1.0) && (0.0 != 0.0));
        s.v[755] = if s.b[755] { 1.0 } else { 0.0 };

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
            s.v[756] = if s.b[756] { 1.0 } else { 0.0 };
            if (((!s.b[725]) && s.b[755]) && s.b[756]) {
                s.store_exp_mul(278, 120, 307);
                s.store_mul_scaled_sqrt_ad_rhs(282, 279, -1.0, A::add_scaled_offset_product_rhs(A::offset(A::add(s.ad_value(281), s.ad_value(280)), (-1.0)), 1.0, s.ad_value(143), s.ad_value(278), (-1.0), 1.0));
                s.store_mul_ad(283, A::div(s.ad_value(438), s.ad_value(282)), A::add_scaled_sub_value_product(1.0, s.ad_value(281), 1.0, s.ad_value(143), s.ad_value(278), 1.0));
            }
            s.b[757] = (s.v[307] < (-1e-8));
            s.v[757] = if s.b[757] { 1.0 } else { 0.0 };
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
            s.v[758] = if s.b[758] { 1.0 } else { 0.0 };
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
        s.v[759] = if s.b[759] { 1.0 } else { 0.0 };

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
            s.v[760] = if s.b[760] { 1.0 } else { 0.0 };
            if (((!s.b[725]) && (!s.b[755])) && s.b[760]) {
                s.store_exp_mul(278, 120, 307);
                s.store_mul_scaled_sqrt_ad_rhs(282, 279, -1.0, A::add_scaled_offset_product_rhs(A::offset(A::add(s.ad_value(281), s.ad_value(280)), (-1.0)), 1.0, s.ad_value(143), s.ad_value(278), (-1.0), 1.0));
                s.store_mul_ad(283, A::div(s.ad_value(438), s.ad_value(282)), A::add_scaled_sub_value_product(1.0, s.ad_value(281), 1.0, s.ad_value(143), s.ad_value(278), 1.0));
            }
            s.b[761] = (s.v[307] < (-1e-8));
            s.v[761] = if s.b[761] { 1.0 } else { 0.0 };
            if ((((!s.b[725]) && (!s.b[755])) && (!s.b[760])) && s.b[761]) {
                s.store_mul_sqrt_ad_rhs(282, 279, A::offset(A::add(s.ad_value(281), s.ad_value(280)), (-1.0)));
                s.store_mul_sub_from_scalar_rhs_ad_lhs(283, A::div(s.ad_value(438), s.ad_value(282)), 1.0, 281);
            }
            if ((((!s.b[725]) && (!s.b[755])) && (!s.b[760])) && (!s.b[761])) {
                s.store_mul_ad_affine_product_lhs(282, A::sqrt(A::div(s.ad_value(438), s.ad_value(120))), s.ad_value(120), -1.0, 0.0, 307);
                s.store_neg_ad(283, A::sqrt(A::mul(s.ad_value(438), s.ad_value(120))));
            }
            if ((!s.b[725]) && (!s.b[755])) {
                let assign8640_body12_ad_e8877: A = A::div(A::sub(A::add(A::add_scaled_inputs3(s.ad_value(56), 1.0, s.ad_value(307), (-1.0), s.ad_value(282), 1.0 / (s.v[294])), A::add_scaled_inputs(s.ad_value(282), (p.p227 * 9662367879.197212), s.ad_value(296), (0.5 * (p.p227 * 9662367879.197212)))), s.ad_value(440)), A::add_scaled_inputs(A::scale_offset(s.ad_value(283), 1.0 / (s.v[294]), (-1.0)), 1.0, s.ad_value(283), (p.p227 * 9662367879.197212)));
                s.store_sub_ad_rhs(284, 307, assign8640_body12_ad_e8877);
            }
            s.b[762] = ((((s.v[284] - s.v[307])) as f64).abs() < s.v[315]);
            s.v[762] = if s.b[762] { 1.0 } else { 0.0 };
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
        s.v[763] = if s.b[763] { 1.0 } else { 0.0 };

        if (((!s.b[725]) && (!s.b[755])) && s.b[763]) {
            s.copy_ad(316, 312);
        }

        s.b[764] = (1.0 == 0.0);
        s.v[764] = if s.b[764] { 1.0 } else { 0.0 };

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
            s.v[765] = if s.b[765] { 1.0 } else { 0.0 };
            if (((!s.b[725]) && (!s.b[755])) && s.b[765]) {
                s.store_exp_mul(278, 120, 307);
                s.store_mul_scaled_sqrt_ad_rhs(282, 279, -1.0, A::add_scaled_offset_product_rhs(A::offset(A::add(s.ad_value(281), s.ad_value(280)), (-1.0)), 1.0, s.ad_value(143), s.ad_value(278), (-1.0), 1.0));
                s.store_mul_ad(283, A::div(s.ad_value(438), s.ad_value(282)), A::add_scaled_sub_value_product(1.0, s.ad_value(281), 1.0, s.ad_value(143), s.ad_value(278), 1.0));
            }
            s.b[766] = (s.v[307] < (-1e-8));
            s.v[766] = if s.b[766] { 1.0 } else { 0.0 };
            if ((((!s.b[725]) && (!s.b[755])) && (!s.b[765])) && s.b[766]) {
                s.store_mul_sqrt_ad_rhs(282, 279, A::offset(A::add(s.ad_value(281), s.ad_value(280)), (-1.0)));
                s.store_mul_sub_from_scalar_rhs_ad_lhs(283, A::div(s.ad_value(438), s.ad_value(282)), 1.0, 281);
            }
            if ((((!s.b[725]) && (!s.b[755])) && (!s.b[765])) && (!s.b[766])) {
                s.store_mul_ad_affine_product_lhs(282, A::sqrt(A::div(s.ad_value(438), s.ad_value(120))), s.ad_value(120), -1.0, 0.0, 307);
                s.store_neg_ad(283, A::sqrt(A::mul(s.ad_value(438), s.ad_value(120))));
            }
            if ((!s.b[725]) && (!s.b[755])) {
                let assign8730_body12_ad_e9220: A = A::div(A::sub(A::add(A::add_scaled_inputs3(s.ad_value(56), 1.0, s.ad_value(307), (-1.0), s.ad_value(282), 1.0 / (s.v[294])), A::add_scaled_inputs(s.ad_value(282), (p.p227 * 9662367879.197212), s.ad_value(296), (0.5 * (p.p227 * 9662367879.197212)))), s.ad_value(440)), A::add_scaled_inputs(A::scale_offset(s.ad_value(283), 1.0 / (s.v[294]), (-1.0)), 1.0, s.ad_value(283), (p.p227 * 9662367879.197212)));
                s.store_sub_ad_rhs(284, 307, assign8730_body12_ad_e9220);
            }
            s.b[767] = ((((s.v[284] - s.v[307])) as f64).abs() < s.v[315]);
            s.v[767] = if s.b[767] { 1.0 } else { 0.0 };
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
        s.v[768] = if s.b[768] { 1.0 } else { 0.0 };

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
        s.v[769] = if s.b[769] { 1.0 } else { 0.0 };

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
        s.v[770] = if s.b[770] { 1.0 } else { 0.0 };

        s.b[771] = (1.0 == 1.0);
        s.v[771] = if s.b[771] { 1.0 } else { 0.0 };

        if ((((!s.b[725]) && s.b[769]) && s.b[770]) && s.b[771]) {
            s.store_scalar(648, 1.0);
        }

        s.b[772] = (1.0 == 2.0);
        s.v[772] = if s.b[772] { 1.0 } else { 0.0 };

        if (((((!s.b[725]) && s.b[769]) && s.b[770]) && (!s.b[771])) && s.b[772]) {
            s.store_scalar(648, 2.0);
        }

        s.b[773] = (1.0 == 4.0);
        s.v[773] = if s.b[773] { 1.0 } else { 0.0 };

        if ((((((!s.b[725]) && s.b[769]) && s.b[770]) && (!s.b[771])) && (!s.b[772])) && s.b[773]) {
            s.store_scalar(648, 3.0);
        }

        s.b[774] = (1.0 == 8.0);
        s.v[774] = if s.b[774] { 1.0 } else { 0.0 };

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
            s.store_add_ad_lhs(306, A::offset(s.ad_value(305), (-0.15)), 637);
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
        s.v[775] = if s.b[775] { 1.0 } else { 0.0 };

        if s.b[775] {
            s.store_scalar(389, s.v[559]);
            s.store_add_scaled_inputs4(388, s.ad_value(72), 1.0, s.ad_value(389), (-1.0), s.ad_value(80), 1.0, s.ad_value(267), -1.0);
            s.store_scalar(32, p.p136);
            s.copy_ad(99, 388);
            s.store_sqrt_ad(100, A::div_scaled_inputs(s.ad_value(471), ((2.0 * 1.6021918e-19) * 1.034943e-10), s.ad_value(120), 1.0));
            s.store_div_scaled_product_by_product(101, s.ad_value(127), s.ad_value(127), 1.0, s.ad_value(471), s.ad_value(471), 1.0);
            s.store_div_scaled_product_by_product(102, s.ad_value(100), s.ad_value(100), 1.0, s.ad_value(270), s.ad_value(270), 1.0);
            s.store_scaled_mul(103, 102, 120, 0.5);
            s.store_scaled_mul(104, 103, 120, 2.0);
            s.store_sqrt_offset_ad(105, A::div_scaled_offset_numerator(A::mul(s.ad_value(120), s.ad_value(99)), 4.0, ((-1.0) * 4.0), s.ad_value(104), 1.0), 1.0);
            s.store_add_ad_rhs(107, 99, A::mul_sub_from_scalar_rhs(s.ad_value(103), 1.0, s.ad_value(105)));
            s.store_div_scalar_by_product(108, 1.0, s.ad_value(101), s.ad_value(102), 1.0);
            s.store_div_ad(109, A::ln(A::mul(s.ad_value(108), A::square(s.ad_value(99)))), A::add(s.ad_value(120), A::div_from_scalar(2.0, s.ad_value(99))));
            s.store_add_scaled_inputs3(110, s.ad_value(109), 1.0, s.ad_value(107), (-1.0), s.ad_value(32), -1.0);
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(111, 109, 1.0, 110, (-0.5), A::add_scaled_square_product(s.ad_value(110), 1.0, s.ad_value(32), s.ad_value(109), 4.0), (-0.5));
        }

    }

    pub(super) fn stamp_reactive_block_7(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if s.b[775] {
            s.store_exp_mul(112, 120, 111);
            s.store_add_scaled_product_value_ad(113, A::offset(A::mul(s.ad_value(120), s.ad_value(111)), (-1.0)), 1.0, 101, 112, 1.0);
            s.store_offset_mul(114, 120, 111, (-1.0));
        }

        s.b[776] = ((s.v[113] > 0.0) && (s.v[114] > 0.0));
        s.v[776] = if s.b[776] { 1.0 } else { 0.0 };

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
        s.v[777] = if s.b[777] { 1.0 } else { 0.0 };

        if ((s.b[775] && s.b[776]) && s.b[777]) {
            s.store_scalar(290, (10.0 * 2.220446049250313e-16));
        }

        if (s.b[775] && s.b[776]) {
            s.store_add_ad_rhs(319, 76, A::mul3_scaled_output(s.ad_value(145), s.ad_value(120), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(290))), 0.5));
            s.copy_ad(58, 319);
            s.store_sub(61, 319, 339);
        }

        s.b[778] = (s.v[61] < 0.0);
        s.v[778] = if s.b[778] { 1.0 } else { 0.0 };

        if ((s.b[775] && s.b[776]) && s.b[778]) {
            s.store_scalar(61, 0.0);
        }

        if (s.b[775] && s.b[776]) {
            s.store_scale(283, 61, (1.0 + 0.3));
            s.store_offset_sub(284, 283, 71, (-0.03));
            s.store_sqrt_add_scaled_square_input(285, 284, 1.0, 283, (4.0 * 0.03));
            s.store_add_scaled_inputs3(60, s.ad_value(283), 1.0, s.ad_value(284), (-0.5), s.ad_value(285), (-0.5));
        }

        s.b[779] = (s.v[60] > s.v[61]);
        s.v[779] = if s.b[779] { 1.0 } else { 0.0 };

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
        s.v[799] = if s.b[799] { 1.0 } else { 0.0 };

        if ((s.b[775] && s.b[776]) && (!s.b[799])) {
            s.store_scalar(391, 4.12);
            s.store_scaled_mul(780, 797, 798, (p.p141 * 1.6021918e-19));
            s.store_div(781, 780, 245);
            s.store_div_scaled_inputs(782, A::offset(A::add_scaled_inputs4(s.ad_value(70), p.p144, s.ad_value(82), 1.0, s.ad_value(266), 1.0, s.ad_value(137), 1.0), p.p143), -1.0, s.ad_value(796), 1.0);
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
                s.store_add_scaled_inputs3(786, s.ad_value(53), 1.0, s.ad_value(73), 1.0, A::add_scaled_product(s.ad_value(339), 1.0, s.ad_value(392), s.ad_value(785), 1.0), -1.0);
                s.store_sub_from_scalar_div_indices(787, 1.0, 786, 391);
                s.store_add_ad_rhs(790, 782, A::div(s.ad_value(786), s.ad_value(796)));
                s.store_square(788, 790);
                s.store_sqrt_square_offset(639, 787, ((4.0 * 0.001) * 0.001));
                s.store_offset_add_scaled_inputs_indices(787, 787, 0.5, 639, 0.5, (1e-10 * 0.001));
            }
            s.b[800] = (s.v[787] < 0.0);
            s.v[800] = if s.b[800] { 1.0 } else { 0.0 };
            if (((s.b[775] && s.b[776]) && (!s.b[799])) && s.b[800]) {
                s.store_scalar(787, 0.0);
            }
            if ((s.b[775] && s.b[776]) && (!s.b[799])) {
                s.store_offset_scaled_ad(789, A::mul(A::sqrt(s.ad_value(787)), s.ad_value(787)), (-p.p142), p.p142);
                s.store_div_scaled_inputs(791, s.ad_value(789), -1.0, s.ad_value(790), 1.0);
            }
            s.b[801] = (s.v[791] < (-34.0));
            s.v[801] = if s.b[801] { 1.0 } else { 0.0 };
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
            s.v[802] = if s.b[802] { 1.0 } else { 0.0 };
            if (((s.b[775] && s.b[776]) && (!s.b[799])) && s.b[802]) {
                s.copy_ad(393, 794);
            }
            if (((s.b[775] && s.b[776]) && (!s.b[799])) && (!s.b[802])) {
                s.store_mul3_lhs(795, 780, 788, 792);
            }
            s.b[803] = ((s.v[795] < s.v[794]) || (s.v[790] < 0.0));
            s.v[803] = if s.b[803] { 1.0 } else { 0.0 };
            if ((((s.b[775] && s.b[776]) && (!s.b[799])) && (!s.b[802])) && s.b[803]) {
                s.copy_ad(393, 794);
            }
            if ((((s.b[775] && s.b[776]) && (!s.b[799])) && (!s.b[802])) && (!s.b[803])) {
                s.copy_ad(393, 795);
            }
            s.b[804] = (s.v[393] < 1e-9);
            s.v[804] = if s.b[804] { 1.0 } else { 0.0 };
            if (((s.b[775] && s.b[776]) && (!s.b[799])) && s.b[804]) {
                s.store_scalar(514, 100.0);
                s.store_scalar(62, s.v[28]);
            }
            if ((s.b[775] && s.b[776]) && (!s.b[799])) {
                s.store_offset(514, 514, 1.0);
            }
        }

        s.b[805] = ((s.v[488] <= 0.0) || (s.v[162] <= 0.0));
        s.v[805] = if s.b[805] { 1.0 } else { 0.0 };

        if ((s.b[775] && s.b[776]) && s.b[805]) {
            s.store_scalar(185, 0.0);
        }

        if ((s.b[775] && s.b[776]) && (!s.b[805])) {
            s.copy_ad(279, 388);
            s.store_square(285, 270);
            s.store_mul_div_from_scalar_lhs(282, 2.0, 472, 285);
            s.store_add_scaled_inputs3(283, s.ad_value(279), 1.0, s.ad_value(122), (-1.0), s.ad_value(70), (-s.v[486]));
            s.store_offset_mul(284, 282, 283, 1.0);
            s.store_sqrt_square_offset(639, 284, ((4.0 * 0.001) * 0.001));
            s.store_offset_scaled_div(287, 284, 639, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(284, 284, 0.5, 639, 0.5, (1e-10 * 0.001));
        }

        s.b[806] = (s.v[284] < 0.0);
        s.v[806] = if s.b[806] { 1.0 } else { 0.0 };

        if (((s.b[775] && s.b[776]) && (!s.b[805])) && s.b[806]) {
            s.store_scalar(284, 0.0);
            s.store_scalar(287, 0.0);
        }

        if ((s.b[775] && s.b[776]) && (!s.b[805])) {
            s.store_offset(284, 284, 1e-50);
            s.store_add_scaled_ad_rhs(186, 279, s.v[491], A::mul_sub_from_scalar_rhs(A::div(s.ad_value(472), s.ad_value(285)), 1.0, A::sqrt(s.ad_value(284))));
            s.store_add_scaled_inputs3(187, s.ad_value(71), p.p123, s.ad_value(339), 1.0, s.ad_value(186), (-(s.v[487] * s.v[485])));
            s.store_sqrt_square_offset(639, 187, ((4.0 * 0.01) * 0.01));
            s.store_offset_scaled_div(287, 187, 639, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(187, 187, 0.5, 639, 0.5, (1e-10 * 0.01));
        }

        s.b[807] = (s.v[187] < 0.0);
        s.v[807] = if s.b[807] { 1.0 } else { 0.0 };

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
        s.v[808] = if s.b[808] { 1.0 } else { 0.0 };

        if ((s.b[775] && s.b[776]) && s.b[808]) {
            s.store_scaled_exp_scaled_input(279, 120, (-p.p140), ((1.6021918e-19 * p.p227) * s.v[466]));
            s.store_offset_scaled(280, 471, (((((36.0 * 1e-7) / 0.0001)) as f64).sqrt() * 13.0), ((((((13.0 * 1e-7) / 0.0001)) as f64).sqrt() * 36.0) * (1e20 / 1e-6)));
            s.store_div_from_scalar_mul_ad(282, (((((13.0 * 1e-7) / 0.0001)) as f64).sqrt() * ((((36.0 * 1e-7) / 0.0001)) as f64).sqrt()), s.ad_value(279), s.ad_value(280));
            s.store_scale(283, 122, 0.0);
            s.store_sqrt_ad(284, A::mul_scaled_lhs(s.ad_value(471), ((2.0 * 1.034943e-10) * 1.6021918e-19), s.ad_value(122)));
            s.store_sqrt_mul_ad(285, s.ad_value(120), A::sub(s.ad_value(339), s.ad_value(283)));
            s.store_sqrt_mul(286, 120, 339);
            s.store_mul_sub_scaled_inputs_rhs(337, 284, s.ad_value(285), -1.0, s.ad_value(286), -1.0);
        }

        if (((s.b[775] && s.b[776]) && s.b[808]) && (p.p27 != 0.0)) {
            s.store_div_from_scalar_offset_input(342, p.p137, 185, p.p138);
            s.store_mul(341, 342, 270);
            s.copy_ad(340, 337);
            s.store_scaled_voltage(562, ctx, nodes, Some(10), None, 1e-9);
            s.copy_ad(337, 562);
            s.store_div_scaled_inputs2(558, s.ad_value(562), 1.0, s.ad_value(340), (-1.0), s.ad_value(341), 1.0);
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

        s.v[379] = 0.0;

        s.v[606] = 1.0;

        s.v[604] = 0.0;

        s.v[605] = 0.0;

        s.b[809] = (s.v[649] < 4.0);
        s.v[809] = if s.b[809] { 1.0 } else { 0.0 };

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

        s.v[62] = 1.0;

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
            s.v[810] = if s.b[810] { 1.0 } else { 0.0 };
            if s.b[810] {
                s.store_exp_mul(280, 120, 307);
                s.store_mul_sqrt_ad_rhs(312, 439, A::add_scaled_offset_product_rhs(A::offset(A::add(s.ad_value(278), s.ad_value(297)), (-1.0)), 1.0, s.ad_value(143), s.ad_value(280), (-1.0), 1.0));
                s.store_div_scaled_product_right_ad(343, 438, A::add_scaled_sub_value_product(1.0, s.ad_value(278), 1.0, s.ad_value(143), s.ad_value(280), 1.0), 1.0, 312, 1.0);
            }
            s.b[811] = (s.v[279] > (1e-8 / 10.0));
            s.v[811] = if s.b[811] { 1.0 } else { 0.0 };
            if ((!s.b[810]) && s.b[811]) {
                s.store_exp_mul(280, 120, 307);
                s.store_mul_scaled_sqrt_ad_rhs(312, 439, -1.0, A::add_scaled_offset_product_rhs(A::offset(A::add(s.ad_value(278), s.ad_value(297)), (-1.0)), 1.0, s.ad_value(143), A::sub(s.ad_value(280), s.ad_value(297)), (-1.0), 1.0));
                s.store_div_scaled_product_right_ad(343, 438, A::add_scaled_sub_value_product(1.0, s.ad_value(278), 1.0, s.ad_value(143), A::offset(s.ad_value(280), (-1.0)), 1.0), 1.0, 312, 1.0);
            }
            if ((!s.b[810]) && (!s.b[811])) {
                s.store_scaled_mul(312, 439, 297, (-1.0 / (((2.0) as f64).sqrt())));
                s.store_scaled_mul(343, 439, 120, (-1.0 / (((2.0) as f64).sqrt())));
            }
            s.store_add_scaled_inputs4(306, s.ad_value(307), 1.0, s.ad_value(312), (-1.0 / (s.v[294])), s.ad_value(50), 1.0, s.ad_value(298), 1.0);
            s.store_sub_from_scalar_scaled_input(583, 1.0, 343, 1.0 / (s.v[294]));
            s.store_sub(279, 305, 522);
            s.store_mul(297, 120, 279);
            s.b[812] = ((-s.v[297]) >= 80.0);
            s.v[812] = if s.b[812] { 1.0 } else { 0.0 };
            if s.b[812] {
                s.store_scaled_offset_ad(278, A::sub_from_scalar(1.0, s.ad_value(297)), (-80.0), 5.540622384e34);
                s.store_scalar(284, 5.540622384e34);
            }
            if (!s.b[812]) {
                s.store_exp_neg_input(278, 297);
                s.copy_ad(284, 278);
            }
            s.b[813] = (s.v[279] < (-1e-8));
            s.v[813] = if s.b[813] { 1.0 } else { 0.0 };
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
            s.v[814] = if s.b[814] { 1.0 } else { 0.0 };
            if ((!s.b[813]) && s.b[814]) {
                s.store_sqrt_offset_ad(280, A::add(s.ad_value(278), s.ad_value(297)), (-1.0));
                s.store_mul_neg_lhs(523, 141, 280);
                s.store_div_scaled_product3_mixed_iiai(524, 141, 120, A::sub_from_scalar(1.0, s.ad_value(284)), -1.0, 280, 2.0);
                s.store_neg(525, 524);
                s.store_exp(278, 297);
                s.store_exp_mul(281, 120, 522);
                s.store_sqrt_add_ad(282, A::div_scaled_product(s.ad_value(523), s.ad_value(523), 1.0, A::square(s.ad_value(141)), 1.0), A::mul3_scaled_output(s.ad_value(142), s.ad_value(281), A::offset(A::sub(s.ad_value(278), s.ad_value(297)), (-1.0)), 2.0));
                s.store_div_scaled_inputs(537, A::add_scaled_offset_product_rhs(A::div_scaled_product(s.ad_value(523), s.ad_value(524), 2.0, A::square(s.ad_value(141)), 1.0), 1.0, A::mul3_scaled_output(s.ad_value(120), s.ad_value(142), s.ad_value(281), 2.0), s.ad_value(278), (-1.0), 1.0), 1.0, s.ad_value(282), 2.0);
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
            s.v[815] = if s.b[815] { 1.0 } else { 0.0 };
            if s.b[815] {
                s.store_scaled_offset_ad(278, A::sub_from_scalar(1.0, s.ad_value(297)), (-80.0), 5.540622384e34);
                s.store_scalar(284, 5.540622384e34);
            }
            if (!s.b[815]) {
                s.store_exp_neg_input(278, 297);
                s.copy_ad(284, 278);
            }
            s.b[816] = (s.v[279] < (-1e-8));
            s.v[816] = if s.b[816] { 1.0 } else { 0.0 };
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
            s.v[817] = if s.b[817] { 1.0 } else { 0.0 };
            if ((!s.b[816]) && s.b[817]) {
                s.store_sqrt_offset_ad(280, A::add(s.ad_value(278), s.ad_value(297)), (-1.0));
                s.store_mul_neg_lhs(531, 141, 280);
                s.store_div_scaled_product3_mixed_iiai(532, 141, 120, A::sub_from_scalar(1.0, s.ad_value(284)), -1.0, 280, 2.0);
                s.store_neg(533, 532);
                s.store_exp(278, 297);
                s.store_exp_mul(281, 120, 522);
                s.store_sqrt_add_ad(282, A::div_scaled_product(s.ad_value(531), s.ad_value(531), 1.0, A::square(s.ad_value(141)), 1.0), A::mul3_scaled_output(s.ad_value(142), s.ad_value(281), A::offset(A::sub(s.ad_value(278), s.ad_value(297)), (-1.0)), 2.0));
                s.store_div_scaled_inputs(539, A::add_scaled_offset_product_rhs(A::div_scaled_product(s.ad_value(531), s.ad_value(532), 2.0, A::square(s.ad_value(141)), 1.0), 1.0, A::mul3_scaled_output(s.ad_value(120), s.ad_value(142), s.ad_value(281), 2.0), s.ad_value(278), (-1.0), 1.0), 1.0, s.ad_value(282), 2.0);
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
            s.v[818] = if s.b[818] { 1.0 } else { 0.0 };
            if s.b[818] {
                s.store_scalar(574, s.v[62]);
                s.store_scalar(62, s.v[28]);
            }
            if (!s.b[818]) {
                s.store_add_scaled_inputs3(346, s.ad_value(305), 1.0, s.ad_value(76), (-1.0), A::div(A::add(A::add(A::add_scaled_inputs4(s.ad_value(312), 1.0, s.ad_value(311), 1.0, s.ad_value(523), 1.0, s.ad_value(528), 1.0), s.ad_value(531)), s.ad_value(337)), s.ad_value(270)), -1.0);
                s.store_sub_from_scalar_ad(347, 1.0, A::div_scaled_inputs2(s.ad_value(526), 1.0, s.ad_value(524), 1.0, s.ad_value(270), 1.0));
                s.store_div_scaled_inputs(348, A::add_scaled_inputs4(s.ad_value(527), 1.0, s.ad_value(525), 1.0, s.ad_value(530), 1.0, s.ad_value(533), 1.0), -1.0, s.ad_value(270), 1.0);
                s.store_div_scaled_inputs(349, A::add_scaled_product(s.ad_value(343), 1.0, A::add(s.ad_value(529), s.ad_value(532)), s.ad_value(583), 1.0), -1.0, s.ad_value(270), 1.0);
            }
            s.b[819] = (s.v[312] <= s.v[599]);
            s.v[819] = if s.b[819] { 1.0 } else { 0.0 };
            if ((!s.b[818]) && s.b[819]) {
                s.store_sqrt_mul_ad(279, s.ad_value(296), A::add_scaled_inputs(s.ad_value(312), 2.0, s.ad_value(296), 1.0));
                s.store_div_scaled_product_indices(604, 296, 343, 1.0, 279, 1.0);
            }
            s.b[820] = (s.v[312] <= s.v[603]);
            s.v[820] = if s.b[820] { 1.0 } else { 0.0 };
            if (((!s.b[818]) && (!s.b[819])) && s.b[820]) {
                s.store_mul3_ad(279, A::mul3(s.ad_value(601), A::sub(s.ad_value(312), s.ad_value(603)), A::sub(s.ad_value(312), s.ad_value(603))), A::sub(s.ad_value(312), s.ad_value(603)), A::sub(s.ad_value(312), s.ad_value(602)));
                s.store_mul_ad_product_lhs(604, A::mul3(s.ad_value(601), A::sub(s.ad_value(312), s.ad_value(603)), A::sub(s.ad_value(312), s.ad_value(603))), A::add_scaled_inputs4(s.ad_value(312), 3.0, s.ad_value(602), (-3.0), s.ad_value(312), 1.0, s.ad_value(603), (-1.0)), 343);
            }
            if (((!s.b[818]) && (!s.b[819])) && (!s.b[820])) {
                s.store_scalar(279, 0.0);
                s.store_scalar(604, 0.0);
            }
            if (!s.b[818]) {
                s.store_div_scaled_inputs(281, s.ad_value(316), (-s.v[650]), s.ad_value(296), 1.0);
                s.store_div_from_scalar_offset_ad(280, 1.0, A::exp_scaled_input(s.ad_value(281), -1.0), 1.0);
                s.store_mul_ad(278, A::square(s.ad_value(280)), A::exp_scaled_input(s.ad_value(281), -1.0));
                s.store_mul(280, 280, 600);
                s.store_neg_ad(279, A::add(s.ad_value(296), s.ad_value(280)));
                s.store_scalar(604, 0.0);
                s.store_scaled_add(350, 523, 279, 1.0 / (s.v[535]));
                s.store_scale(351, 524, 1.0 / (s.v[535]));
                s.store_scale(352, 525, 1.0 / (s.v[535]));
                s.store_scale(353, 604, 1.0 / (s.v[535]));
                s.store_div_scaled_inputs(281, s.ad_value(316), (-s.v[651]), s.ad_value(296), 1.0);
                s.store_div_from_scalar_offset_ad(280, 1.0, A::exp_scaled_input(s.ad_value(281), -1.0), 1.0);
                s.store_mul_ad(278, A::square(s.ad_value(280)), A::exp_scaled_input(s.ad_value(281), -1.0));
                s.store_mul(280, 280, 600);
                s.store_scalar(605, 0.0);
                s.store_scaled_add(354, 531, 280, 1.0 / (s.v[535]));
                s.store_scale(355, 533, 1.0 / (s.v[535]));
                s.store_add_scaled_product_indices(356, 605, 1.0 / (s.v[535]), 532, 583, 1.0 / (s.v[535]));
                s.store_add_scaled_inputs4(357, A::mul3(s.ad_value(347), s.ad_value(352), s.ad_value(356)), 1.0, A::mul3(s.ad_value(347), s.ad_value(353), s.ad_value(355)), (-1.0), A::mul3(s.ad_value(348), s.ad_value(351), s.ad_value(356)), -1.0, A::mul3(s.ad_value(349), s.ad_value(351), s.ad_value(355)), 1.0);
            }
            s.b[821] = (s.v[357] > 0.0);
            s.v[821] = if s.b[821] { 1.0 } else { 0.0 };
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
            s.v[822] = if s.b[822] { 1.0 } else { 0.0 };
            if ((!s.b[818]) && s.b[822]) {
                s.store_abs(279, 369);
            }
            s.b[823] = (s.v[279] < ((s.v[370]) as f64).abs());
            s.v[823] = if s.b[823] { 1.0 } else { 0.0 };
            if ((!s.b[818]) && s.b[823]) {
                s.store_abs(279, 370);
            }
            if (!s.b[818]) {
                s.store_scalar(606, 1.0);
            }
            s.b[824] = (s.v[62] > 80.0);
            s.v[824] = if s.b[824] { 1.0 } else { 0.0 };
            if ((!s.b[818]) && s.b[824]) {
                s.store_scalar(606, 25.0);
            }
            s.b[825] = (s.v[62] > 40.0);
            s.v[825] = if s.b[825] { 1.0 } else { 0.0 };
            if (((!s.b[818]) && (!s.b[824])) && s.b[825]) {
                s.store_scalar(606, 25.0);
            }
            s.b[826] = (s.v[62] > 20.0);
            s.v[826] = if s.b[826] { 1.0 } else { 0.0 };
            if ((((!s.b[818]) && (!s.b[824])) && (!s.b[825])) && s.b[826]) {
                s.store_scalar(606, 25.0);
            }
            s.b[827] = (s.v[62] > 10.0);
            s.v[827] = if s.b[827] { 1.0 } else { 0.0 };
            if (((((!s.b[818]) && (!s.b[824])) && (!s.b[825])) && (!s.b[826])) && s.b[827]) {
                s.store_scalar(606, 5.0);
            }
            s.b[828] = (s.v[279] > (0.1 / s.v[606]));
            s.v[828] = if s.b[828] { 1.0 } else { 0.0 };
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
            s.v[829] = if s.b[829] { 1.0 } else { 0.0 };
            if ((!s.b[818]) && s.b[829]) {
                s.store_scalar(379, 1.0);
            }
            s.store_offset(62, 62, 1.0);
        }

        s.b[830] = (s.v[574] > 0.0);
        s.v[830] = if s.b[830] { 1.0 } else { 0.0 };

        if s.b[830] {
            s.copy_ad(62, 574);
            s.store_scalar(574, 0.0);
        }

        s.b[831] = (s.v[62] > s.v[28]);
        s.v[831] = if s.b[831] { 1.0 } else { 0.0 };

        if s.b[831] {
            s.copy_ad(305, 299);
            s.copy_ad(306, 300);
            s.copy_ad(307, 301);
            s.copy_ad(522, 534);
        }

        s.copy_ad(56, 305);

        s.store_neg(149, 311);

        s.b[833] = (s.v[149] <= 1e-50);
        s.v[833] = if s.b[833] { 1.0 } else { 0.0 };

        if s.b[833] {
            s.store_scalar(149, 1e-50);
            s.store_scalar(34, 1.0);
        }

        s.store_neg(150, 528);

        s.b[834] = (s.v[150] <= 1e-50);
        s.v[834] = if s.b[834] { 1.0 } else { 0.0 };

        if s.b[834] {
            s.store_scalar(150, 1e-50);
        }

        s.store_mul(86, 149, 271);

        s.copy_ad(396, 51);

        s.store_div_ad_rhs(280, 472, A::square(s.ad_value(270)));

        s.store_sub(278, 76, 122);

        s.store_offset_mul_ad(287, A::div_from_scalar(2.0, s.ad_value(280)), s.ad_value(278), 1.0);

        s.store_sqrt_square_offset(639, 287, ((4.0 * 0.05) * 0.05));

        s.store_offset_scaled_div(284, 287, 639, 0.5, 0.5);

        s.store_offset_add_scaled_inputs_indices(287, 287, 0.5, 639, 0.5, (1e-10 * 0.05));

    }
}
