#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_0(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        let ctx_temp = ctx.temperature();
        s.b[484] = (p.p3 == 1.0);
        s.v[484] = if s.b[484] { 1.0 } else { 0.0 };

        if s.b[484] {
            s.store_scalar(0, 70300000.0);
            s.store_scalar(1, 123000000.0);
        }

        if (!s.b[484]) {
            s.store_scalar(0, 158000000.0);
            s.store_scalar(1, 204000000.0);
        }

        s.v[160] = (1.0 - p.p33);

        s.v[3] = (p.p4 + 273.15);

        s.v[5] = (ctx_temp + p.p0);

        s.v[344] = 0.0;

        s.b[485] = (p.p154 == 0.0);
        s.v[485] = if s.b[485] { 1.0 } else { 0.0 };

        if s.b[485] {
            s.store_scalar(345, 1e-12);
        }

        if (!s.b[485]) {
            s.store_scalar(345, p.p154);
        }

        s.store_scale(346, 345, p.p1);

        s.store_div_from_scalar(347, 1.0, 346);

        s.b[486] = (p.p134 > 0.0);
        s.v[486] = if s.b[486] { 1.0 } else { 0.0 };

        if s.b[486] {
            s.store_scalar(348, s.v[344]);
        }

        if (!s.b[486]) {
            s.store_scalar(348, 0.0);
        }

        s.v[52] = 0.001;

        s.v[342] = 0.001;

        s.v[62] = ((2.0) as f64).powf((2.0 - p.p67));

        s.v[63] = (1.0 / s.v[62]);

        s.v[285] = (((p.p114 + (((p.p115 * s.v[3]) * s.v[3]) / (s.v[3] + p.p116))) - 0.05) / 0.1);

        s.b[487] = ((p.p114 + (((p.p115 * s.v[3]) * s.v[3]) / (s.v[3] + p.p116))) < 0.05);
        s.v[487] = if s.b[487] { 1.0 } else { 0.0 };

        if s.b[487] {
            s.store_scalar(74, (0.05 + (0.1 * (((1.0 + ((s.v[285]) as f64).exp())) as f64).ln())));
        }

        if (!s.b[487]) {
            s.store_scalar(74, ((p.p114 + (((p.p115 * s.v[3]) * s.v[3]) / (s.v[3] + p.p116))) + (0.1 * (((1.0 + (((-s.v[285])) as f64).exp())) as f64).ln())));
        }

        s.v[71] = p.p114;

        s.v[72] = (1.0 / s.v[71]);

        s.v[64] = (1.0 / p.p66);

        s.v[75] = p.p71;

        s.v[76] = p.p72;

        s.v[79] = ((2.0) as f64).powf((2.0 - s.v[76]));

        s.v[89] = (1.0 / s.v[79]);

        s.v[285] = (((p.p117 + (((p.p118 * s.v[3]) * s.v[3]) / (s.v[3] + p.p119))) - 0.05) / 0.1);

        s.b[488] = ((p.p117 + (((p.p118 * s.v[3]) * s.v[3]) / (s.v[3] + p.p119))) < 0.05);
        s.v[488] = if s.b[488] { 1.0 } else { 0.0 };

        if s.b[488] {
            s.store_scalar(88, (0.05 + (0.1 * (((1.0 + ((s.v[285]) as f64).exp())) as f64).ln())));
        }

        if (!s.b[488]) {
            s.store_scalar(88, ((p.p117 + (((p.p118 * s.v[3]) * s.v[3]) / (s.v[3] + p.p119))) + (0.1 * (((1.0 + (((-s.v[285])) as f64).exp())) as f64).ln())));
        }

        s.v[87] = p.p117;

        s.v[86] = (1.0 / s.v[87]);

        s.v[66] = (1.0 / s.v[75]);

        s.v[349] = (1.0 - (1.0 / p.p83));

        s.v[161] = 0.0;

        s.v[162] = 0.0;

        s.v[179] = 0.0;

        s.v[178] = 1.0;

        s.v[210] = 0.0;

        s.v[212] = 0.0;

        s.v[248] = 0.0;

        s.v[228] = 0.0;

        s.v[42] = 0.0;

        s.v[44] = 0.0;

        s.v[53] = 0.0;

        s.v[54] = 0.0;

        s.v[45] = 0.0;

        s.store_voltage(218, ctx, nodes, Some(4), None);

        s.b[489] = (s.v[218] < 0.0);
        s.v[489] = if s.b[489] { 1.0 } else { 0.0 };

        if s.b[489] {
            s.store_neg_ad(218, A::ln(A::sub_from_scalar(1.0, s.ad_value(218))));
        }

        s.b[490] = (s.v[218] < p.p125);
        s.v[490] = if s.b[490] { 1.0 } else { 0.0 };

        if s.b[490] {
            s.copy_ad(11, 218);
        }

        if (!s.b[490]) {
            s.store_offset_ln_ad(11, A::offset(s.ad_value(218), (((-p.p125)) + (1.0))), p.p125);
        }

        s.store_offset(2, 11, s.v[5]);

        s.store_scale(4, 2, 1.0 / (s.v[3]));

        s.store_scale(6, 2, 8.617086918058125e-5);

        s.v[7] = (8.617086918058125e-5 * s.v[3]);

        s.store_div_from_scalar(8, 1.0, 6);

        s.v[9] = (1.0 / s.v[7]);

        s.store_offset(10, 8, (-s.v[9]));

        s.store_offset(12, 2, (-s.v[3]));

        s.store_ln(280, 4);

        s.store_scaled_offset_ad(285, A::sub(s.ad_value(74), A::div_scaled_product_offset_denominator(s.ad_value(2), s.ad_value(2), p.p115, s.ad_value(2), p.p116, 1.0)), (-0.05), 10.0);

        s.b[491] = ((s.v[74] - (((p.p115 * s.v[2]) * s.v[2]) / (s.v[2] + p.p116))) < 0.05);
        s.v[491] = if s.b[491] { 1.0 } else { 0.0 };

        if s.b[491] {
            s.store_offset_scaled_ad(70, A::ln_one_plus_exp(s.ad_value(285)), 0.1, 0.05);
        }

        if (!s.b[491]) {
            s.store_ad_value(70, A::add_scaled_inputs3(s.ad_value(74), 1.0, A::div_scaled_product_offset_denominator(s.ad_value(2), s.ad_value(2), p.p115, s.ad_value(2), p.p116, 1.0), (-1.0), A::ln_one_plus_exp(A::neg(s.ad_value(285))), 0.1));
        }

        s.store_scaled_offset_ad(285, A::sub(s.ad_value(88), A::div_scaled_product_offset_denominator(s.ad_value(2), s.ad_value(2), p.p118, s.ad_value(2), p.p119, 1.0)), (-0.05), 10.0);

        s.b[492] = ((s.v[88] - (((p.p118 * s.v[2]) * s.v[2]) / (s.v[2] + p.p119))) < 0.05);
        s.v[492] = if s.b[492] { 1.0 } else { 0.0 };

        if s.b[492] {
            s.store_offset_scaled_ad(85, A::ln_one_plus_exp(s.ad_value(285)), 0.1, 0.05);
        }

        if (!s.b[492]) {
            s.store_ad_value(85, A::add_scaled_inputs3(s.ad_value(88), 1.0, A::div_scaled_product_offset_denominator(s.ad_value(2), s.ad_value(2), p.p118, s.ad_value(2), p.p119, 1.0), (-1.0), A::ln_one_plus_exp(A::neg(s.ad_value(285))), 0.1));
        }

        s.store_add_ad(13, A::add_scaled_product(s.ad_value(4), p.p66, s.ad_value(6), s.ad_value(280), (-3.0)), A::scale_offset(s.ad_value(4), (-p.p105), p.p105));

        s.store_div_ad_lhs(285, A::sub_from_scalar(0.05, s.ad_value(13)), 6);

        s.b[493] = (0.05 < s.v[13]);
        s.v[493] = if s.b[493] { 1.0 } else { 0.0 };

        if s.b[493] {
            s.store_ad_value(14, A::add_scaled_product(s.ad_value(13), 1.0, s.ad_value(6), A::ln_one_plus_exp(s.ad_value(285)), 1.0));
        }

        if (!s.b[493]) {
            s.store_offset_mul_ad(14, s.ad_value(6), A::ln_one_plus_exp(A::neg(s.ad_value(285))), 0.05);
        }

        s.store_add_ad(15, A::add_scaled_product(s.ad_value(4), p.p64, s.ad_value(6), s.ad_value(280), (-3.0)), A::scale_offset(s.ad_value(4), (-p.p110), p.p110));

        s.store_div_ad_lhs(285, A::sub_from_scalar(0.05, s.ad_value(15)), 6);

        s.b[494] = (0.05 < s.v[15]);
        s.v[494] = if s.b[494] { 1.0 } else { 0.0 };

        if s.b[494] {
            s.store_ad_value(16, A::add_scaled_product(s.ad_value(15), 1.0, s.ad_value(6), A::ln_one_plus_exp(s.ad_value(285)), 1.0));
        }

        if (!s.b[494]) {
            s.store_offset_mul_ad(16, s.ad_value(6), A::ln_one_plus_exp(A::neg(s.ad_value(285))), 0.05);
        }

        s.store_add_ad(21, A::add_scaled_product(s.ad_value(4), p.p80, s.ad_value(6), s.ad_value(280), (-3.0)), A::scale_offset(s.ad_value(4), (-p.p110), p.p110));

        s.store_div_ad_lhs(285, A::sub_from_scalar(0.05, s.ad_value(21)), 6);

        s.b[495] = (0.05 < s.v[21]);
        s.v[495] = if s.b[495] { 1.0 } else { 0.0 };

        if s.b[495] {
            s.store_ad_value(22, A::add_scaled_product(s.ad_value(21), 1.0, s.ad_value(6), A::ln_one_plus_exp(s.ad_value(285)), 1.0));
        }

        if (!s.b[495]) {
            s.store_offset_mul_ad(22, s.ad_value(6), A::ln_one_plus_exp(A::neg(s.ad_value(285))), 0.05);
        }

        s.store_add_ad(18, A::add_scaled_product(s.ad_value(4), p.p71, s.ad_value(6), s.ad_value(280), (-3.0)), A::scale_offset(s.ad_value(4), (-p.p110), p.p110));

        s.store_div_ad_lhs(285, A::sub_from_scalar(0.05, s.ad_value(18)), 6);

        s.b[496] = (0.05 < s.v[18]);
        s.v[496] = if s.b[496] { 1.0 } else { 0.0 };

        if s.b[496] {
            s.store_ad_value(17, A::add_scaled_product(s.ad_value(18), 1.0, s.ad_value(6), A::ln_one_plus_exp(s.ad_value(285)), 1.0));
        }

        if (!s.b[496]) {
            s.store_offset_mul_ad(17, s.ad_value(6), A::ln_one_plus_exp(A::neg(s.ad_value(285))), 0.05);
        }

        s.store_add_ad(20, A::add_scaled_product(s.ad_value(4), s.v[75], s.ad_value(6), s.ad_value(280), (-3.0)), A::scale_offset(s.ad_value(4), (-p.p110), p.p110));

        s.store_div_ad_lhs(285, A::sub_from_scalar(0.05, s.ad_value(20)), 6);

        s.b[497] = (0.05 < s.v[20]);
        s.v[497] = if s.b[497] { 1.0 } else { 0.0 };

        if s.b[497] {
            s.store_ad_value(19, A::add_scaled_product(s.ad_value(20), 1.0, s.ad_value(6), A::ln_one_plus_exp(s.ad_value(285)), 1.0));
        }

        if (!s.b[497]) {
            s.store_offset_mul_ad(19, s.ad_value(6), A::ln_one_plus_exp(A::neg(s.ad_value(285))), 0.05);
        }

        s.store_add_ad(56, A::add_scaled_product(s.ad_value(4), p.p27, s.ad_value(6), s.ad_value(280), (-3.0)), A::scale_offset(s.ad_value(4), (-p.p109), p.p109));

        s.store_div_ad_lhs(285, A::sub_from_scalar(0.05, s.ad_value(56)), 6);

        s.b[498] = (0.05 < s.v[56]);
        s.v[498] = if s.b[498] { 1.0 } else { 0.0 };

        if s.b[498] {
            s.store_ad_value(55, A::add_scaled_product(s.ad_value(56), 1.0, s.ad_value(6), A::ln_one_plus_exp(s.ad_value(285)), 1.0));
        }

        if (!s.b[498]) {
            s.store_offset_mul_ad(55, s.ad_value(6), A::ln_one_plus_exp(A::neg(s.ad_value(285))), 0.05);
        }

        s.store_add_ad(104, A::add_scaled_product(s.ad_value(4), p.p138, s.ad_value(6), s.ad_value(280), (-3.0)), A::scale_offset(s.ad_value(4), (-p.p140), p.p140));

        s.store_div_ad_lhs(285, A::sub_from_scalar(0.05, s.ad_value(104)), 6);

        s.b[499] = (0.05 < s.v[104]);
        s.v[499] = if s.b[499] { 1.0 } else { 0.0 };

        if s.b[499] {
            s.store_ad_value(105, A::add_scaled_product(s.ad_value(104), 1.0, s.ad_value(6), A::ln_one_plus_exp(s.ad_value(285)), 1.0));
        }

        if (!s.b[499]) {
            s.store_offset_mul_ad(105, s.ad_value(6), A::ln_one_plus_exp(A::neg(s.ad_value(285))), 0.05);
        }

        s.store_div_from_scalar(65, 1.0, 14);

        s.store_div_from_scalar(67, 1.0, 19);

        s.store_powf_ad(73, A::scale(s.ad_value(65), p.p66), p.p67);

        s.store_powf_ad(90, A::scale(s.ad_value(67), s.v[75]), s.v[76]);

        s.store_scale(23, 73, p.p65);

        s.store_scaled_powf_ad(106, A::div_from_scalar(p.p138, s.ad_value(105)), p.p139, p.p137);

        s.store_offset_scaled_ad(26, A::powf(A::div_from_scalar(p.p71, s.ad_value(17)), p.p72), (1.0 - p.p75), p.p75);

        s.store_div_from_scalar(27, 1.0, 26);

        s.store_scale(24, 26, p.p70);

        s.store_scale(25, 27, p.p75);

        s.store_scaled_exp_scaled_input(28, 280, p.p97, p.p54);

        s.b[500] = (s.v[28] < s.v[346]);
        s.v[500] = if s.b[500] { 1.0 } else { 0.0 };

        if s.b[500] {
            s.copy_ad(28, 346);
        }

        s.store_scaled_exp_scaled_input(29, 280, (p.p98 - p.p96), p.p56);

        s.store_scaled_exp_scaled_input(30, 280, p.p101, p.p55);

        s.b[501] = (s.v[30] < s.v[346]);
        s.v[501] = if s.b[501] { 1.0 } else { 0.0 };

        if s.b[501] {
            s.copy_ad(30, 346);
        }

        s.store_scaled_exp_scaled_input(32, 280, p.p102, p.p57);

        s.store_scaled_exp_scaled_input(33, 280, p.p104, p.p58);

        s.store_scaled_exp_scaled_input(34, 280, p.p104, p.p59);

        s.store_scaled_exp_scaled_input(31, 280, p.p99, p.p60);

        s.b[502] = (p.p122 != 0.0);
        s.v[502] = if s.b[502] { 1.0 } else { 0.0 };

        if s.b[502] {
            s.store_offset_scaled(50, 12, ((p.p122) * (p.p10)), p.p10);
            s.store_scaled_offset(285, 50, (-1.0), 1.0 / (s.v[52]));
        }

        s.b[503] = (s.v[50] < 1.0);
        s.v[503] = if s.b[503] { 1.0 } else { 0.0 };

        if (s.b[502] && s.b[503]) {
            s.store_offset_scaled_ad(50, A::ln_one_plus_exp(s.ad_value(285)), s.v[52], 1.0);
        }

        if (s.b[502] && (!s.b[503])) {
            s.store_ad_value(50, A::add_scaled_inputs(s.ad_value(50), 1.0, A::ln_one_plus_exp(A::neg(s.ad_value(285))), s.v[52]));
        }

        if s.b[502] {
            s.store_offset(48, 50, (-(s.v[52] * 0.6931471805599453)));
        }

        if (!s.b[502]) {
            s.store_scalar(48, p.p10);
        }

        s.b[504] = (p.p123 != 0.0);
        s.v[504] = if s.b[504] { 1.0 } else { 0.0 };

        if s.b[504] {
            s.store_offset_scaled(51, 12, ((p.p123) * (p.p11)), p.p11);
            s.store_scaled_offset(285, 51, (-1.0), 1.0 / (s.v[52]));
        }

        s.b[505] = (s.v[51] < 1.0);
        s.v[505] = if s.b[505] { 1.0 } else { 0.0 };

        if (s.b[504] && s.b[505]) {
            s.store_offset_scaled_ad(51, A::ln_one_plus_exp(s.ad_value(285)), s.v[52], 1.0);
        }

        if (s.b[504] && (!s.b[505])) {
            s.store_ad_value(51, A::add_scaled_inputs(s.ad_value(51), 1.0, A::ln_one_plus_exp(A::neg(s.ad_value(285))), s.v[52]));
        }

        if s.b[504] {
            s.store_offset(49, 51, (-(s.v[52] * 0.6931471805599453)));
        }

        if (!s.b[504]) {
            s.store_scalar(49, p.p11);
        }

        s.store_offset_scaled(341, 12, ((p.p124) * (p.p43)), p.p43);

        s.v[287] = (s.v[342] * s.v[342]);

        s.store_square(288, 341);

        s.b[506] = (s.v[341] < 0.0);
        s.v[506] = if s.b[506] { 1.0 } else { 0.0 };

        if s.b[506] {
            s.store_div_from_scalar_sub_ad(340, (0.5 * s.v[287]), A::sqrt(A::offset(s.ad_value(288), s.v[287])), s.ad_value(341));
        }

        if (!s.b[506]) {
            s.store_scaled_add_ad_lhs(340, A::sqrt(A::offset(s.ad_value(288), s.v[287])), 341, 0.5);
        }

        s.store_scaled_mul_ad(35, A::exp(A::div_scaled_inputs(s.ad_value(280), (((4.0 - p.p98) - p.p96) + p.p121), s.ad_value(48), 1.0)), A::exp(A::div_scaled_inputs(s.ad_value(10), (-p.p105), s.ad_value(48), 1.0)), p.p9);

        s.store_scaled_exp_scaled_input(36, 280, (1.0 - p.p98), p.p12);

        s.store_scaled_exp_scaled_input(37, 280, (1.0 - p.p103), p.p30);

        s.store_scaled_mul_ad(38, A::exp_scaled_input(s.ad_value(280), (6.0 - (2.0 * p.p21))), A::exp_scaled_input(s.ad_value(10), ((-p.p113) * 1.0 / (p.p21))), p.p20);

        s.store_scaled_mul_ad(39, A::exp_scaled_input(s.ad_value(280), (6.0 - (2.0 * p.p32))), A::exp_scaled_input(s.ad_value(10), ((-p.p110) * 1.0 / (p.p32))), p.p31);

        s.store_scaled_mul_ad(42, A::exp_scaled_input(s.ad_value(280), (((4.0 - p.p97) + p.p121) * 1.0 / (p.p17))), A::exp_scaled_input(s.ad_value(10), ((-p.p111) * 1.0 / (p.p17))), p.p16);

        s.store_scaled_mul_ad(44, A::exp_scaled_input(s.ad_value(280), (((4.0 - p.p97) + p.p121) * 1.0 / (p.p19))), A::exp_scaled_input(s.ad_value(10), ((-p.p111) * 1.0 / (p.p19))), p.p18);

        s.b[507] = (p.p24 == 1.0);
        s.v[507] = if s.b[507] { 1.0 } else { 0.0 };

        if s.b[507] {
            s.store_scaled_exp_scaled_input(53, 10, ((-p.p107) * 1.0 / (p.p17)), p.p25);
            s.store_scaled_exp_scaled_input(54, 10, (-p.p106), p.p28);
            s.store_scaled_exp_scaled_input(45, 10, ((-p.p108) * 1.0 / (p.p19)), p.p26);
        }

        s.store_scaled_mul_ad(43, A::exp_scaled_input(s.ad_value(280), ((4.0 - p.p103) + p.p121)), A::exp_scaled_input(s.ad_value(10), (-p.p112)), p.p29);

        s.store_scaled_mul_ad(46, A::exp_scaled_input(s.ad_value(280), (6.0 - (2.0 * p.p23))), A::exp_scaled_input(s.ad_value(10), ((-p.p113) * 1.0 / (p.p23))), p.p22);

        s.store_scaled_mul_ad(47, A::exp_scaled_input(s.ad_value(280), (4.0 / p.p150)), A::exp_scaled_input(s.ad_value(10), ((-p.p113) * 1.0 / (p.p150))), p.p149);

        s.store_scaled_mul_ad(357, A::sqrt(s.ad_value(4)), A::exp_scaled_input(s.ad_value(12), p.p157), p.p155);

        s.store_powf_ad(281, A::scale(s.ad_value(70), s.v[72]), (-0.5));

        s.store_div_from_scalar(282, 1.0, 73);

        s.store_mul_ad_affine_product_lhs(61, A::mul3_scaled_output(s.ad_value(70), s.ad_value(70), s.ad_value(281), p.p35), s.ad_value(282), (p.p66 * (s.v[72] * s.v[72])), 0.0, 65);

        s.store_ad_value(58, A::mul3_scaled_output(A::mul3_scaled_output(s.ad_value(281), s.ad_value(14), s.ad_value(14), p.p34), s.ad_value(73), A::exp(A::sub_from_scalar(p.p35, s.ad_value(61))), (s.v[64] * s.v[64])));

        s.store_div_from_scalar(67, 1.0, 19);

        s.store_powf_ad(283, A::scale(s.ad_value(85), s.v[86]), (-0.5));

        s.store_div_from_scalar(284, 1.0, 90);

        s.store_mul_ad_affine_product_lhs(83, A::mul3_scaled_output(s.ad_value(85), s.ad_value(85), s.ad_value(283), p.p37), s.ad_value(284), (s.v[75] * (s.v[86] * s.v[86])), 0.0, 67);

        s.store_ad_value(84, A::mul3_scaled_output(A::mul3_scaled_output(s.ad_value(283), s.ad_value(19), s.ad_value(19), p.p36), s.ad_value(90), A::exp(A::sub_from_scalar(p.p37, s.ad_value(83))), (s.v[66] * s.v[66])));

        s.store_exp_scaled_input(281, 280, p.p96);

        s.store_scaled_mul(40, 281, 27, p.p14);

        s.store_scaled_mul(41, 281, 282, p.p13);

        s.store_scaled_mul_ad(107, A::exp_scaled_input(s.ad_value(280), (4.0 - p.p141)), A::exp_scaled_input(s.ad_value(10), (-p.p140)), p.p133);

        s.store_scaled_mul_ad(108, A::exp_scaled_input(s.ad_value(280), (3.5 - (0.5 * p.p142))), A::exp_scaled_input(s.ad_value(10), (-p.p140)), p.p134);

        s.store_scaled_exp_scaled_input(109, 280, (1.0 - p.p141), p.p135);

        s.store_scaled_exp_scaled_input(110, 280, (1.0 - p.p142), p.p136);

        s.store_scaled_mul_ad(94, A::exp_scaled_input(s.ad_value(280), (p.p98 - 2.0)), A::exp_scaled_input(s.ad_value(10), (-p.p120)), p.p86);

        s.store_scaled_exp_scaled_input(95, 280, ((p.p96 + p.p98) - 1.0), p.p87);

        s.store_scaled_exp_scaled_input(96, 280, (p.p99 - 1.0), p.p88);

        s.store_scaled_add(97, 95, 96, (p.p89 * 1.0 / ((p.p87 + p.p88))));

        s.store_scaled_exp_scaled_input(98, 280, (p.p100 - 1.0), p.p90);

        s.store_offset(101, 2, (-300.0));

    }

    pub(super) fn stamp_transient_block_1(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        s.b[508] = (s.v[2] < 525.0);
        s.v[508] = if s.b[508] { 1.0 } else { 0.0 };

        if s.b[508] {
            s.store_mul_ad_rhs(99, 1, A::add_scaled_product(A::scale_offset(s.ad_value(101), 0.00072, 1.0), 1.0, s.ad_value(101), s.ad_value(101), (-1.6e-6)));
        }

        if (!s.b[508]) {
            s.store_scale(99, 1, 1.081);
        }

        s.store_scaled_exp_scaled_input(100, 280, p.p96, p.p92);

        s.v[103] = (p.p146 * (((s.v[5] / s.v[3])) as f64).powf(p.p148));

        s.b[509] = (p.p57 > 0.0);
        s.v[509] = if s.b[509] { 1.0 } else { 0.0 };

        if s.b[509] {
            s.store_div_from_scalar(111, 1.0, 32);
        }

        s.b[510] = (s.v[111] > s.v[347]);
        s.v[510] = if s.b[510] { 1.0 } else { 0.0 };

        if (s.b[509] && s.b[510]) {
            s.copy_ad(111, 347);
        }

        if (!s.b[509]) {
            s.store_scalar(111, 0.0);
        }

        s.b[511] = (p.p58 > 0.0);
        s.v[511] = if s.b[511] { 1.0 } else { 0.0 };

        if s.b[511] {
            s.store_div_from_scalar(112, 1.0, 33);
        }

        s.b[512] = (s.v[112] > s.v[347]);
        s.v[512] = if s.b[512] { 1.0 } else { 0.0 };

        if (s.b[511] && s.b[512]) {
            s.copy_ad(112, 347);
        }

        if (!s.b[511]) {
            s.store_scalar(112, 0.0);
        }

        s.b[513] = (p.p59 > 0.0);
        s.v[513] = if s.b[513] { 1.0 } else { 0.0 };

        if s.b[513] {
            s.store_div_from_scalar(113, 1.0, 34);
        }

        s.b[514] = (s.v[113] > s.v[347]);
        s.v[514] = if s.b[514] { 1.0 } else { 0.0 };

        if (s.b[513] && s.b[514]) {
            s.copy_ad(113, 347);
        }

        if (!s.b[513]) {
            s.store_scalar(113, 0.0);
        }

        s.store_scaled_voltage(250, ctx, nodes, Some(7), Some(8), p.p3);

        s.store_scaled_voltage(251, ctx, nodes, Some(7), Some(9), p.p3);

        s.store_scaled_voltage(252, ctx, nodes, Some(7), Some(5), p.p3);

        s.store_scaled_voltage(253, ctx, nodes, Some(6), Some(5), p.p3);

        s.store_scaled_voltage(254, ctx, nodes, Some(6), Some(7), p.p3);

        s.store_scaled_voltage(259, ctx, nodes, Some(3), Some(8), p.p3);

        s.store_scaled_voltage(256, ctx, nodes, Some(8), Some(9), p.p3);

        s.store_scaled_voltage(265, ctx, nodes, Some(2), Some(5), p.p3);

        s.store_scaled_voltage(266, ctx, nodes, Some(1), Some(6), p.p3);

        s.store_scaled_voltage(269, ctx, nodes, Some(1), Some(2), p.p3);

        s.store_scaled_voltage(270, ctx, nodes, Some(1), Some(0), p.p3);

        s.store_scaled_voltage(258, ctx, nodes, Some(11), Some(8), p.p3);

        s.store_scaled_voltage(257, ctx, nodes, Some(10), Some(11), p.p3);

        s.store_sub_ad_lhs(255, A::add_scaled_inputs3(s.ad_value(254), 1.0, s.ad_value(251), 1.0, s.ad_value(256), -1.0), 258);

        s.store_sub_ad_lhs(268, A::add_scaled_inputs3(s.ad_value(266), 1.0, s.ad_value(270), (-1.0), s.ad_value(255), 1.0), 257);

        s.store_add(267, 270, 268);

        s.store_sub(261, 259, 258);

        s.store_sub(260, 261, 257);

        s.b[515] = ((s.v[251] * s.v[8]) < p.p151);
        s.v[515] = if s.b[515] { 1.0 } else { 0.0 };

        if s.b[515] {
            s.store_exp_mul(271, 251, 8);
        }

        if (!s.b[515]) {
            s.store_scalar(301, ((p.p151) as f64).exp());
            s.store_mul_offset_ad_rhs(271, 301, A::mul(s.ad_value(251), s.ad_value(8)), (((-p.p151)) + (1.0)));
        }

        s.b[516] = (((s.v[252] * s.v[8]) / s.v[48]) < p.p151);
        s.v[516] = if s.b[516] { 1.0 } else { 0.0 };

        if s.b[516] {
            s.store_exp_ad(272, A::div_scaled_product(s.ad_value(252), s.ad_value(8), 1.0, s.ad_value(48), 1.0));
        }

        if (!s.b[516]) {
            s.store_scalar(301, ((p.p151) as f64).exp());
            s.store_mul_offset_ad_rhs(272, 301, A::div_scaled_product(s.ad_value(252), s.ad_value(8), 1.0, s.ad_value(48), 1.0), (((-p.p151)) + (1.0)));
        }

        s.b[517] = ((s.v[255] * s.v[8]) < p.p151);
        s.v[517] = if s.b[517] { 1.0 } else { 0.0 };

        if s.b[517] {
            s.store_exp_mul(274, 255, 8);
        }

        if (!s.b[517]) {
            s.store_scalar(301, ((p.p151) as f64).exp());
            s.store_mul_offset_ad_rhs(274, 301, A::mul(s.ad_value(255), s.ad_value(8)), (((-p.p151)) + (1.0)));
        }

        s.b[518] = ((s.v[254] * s.v[8]) < p.p151);
        s.v[518] = if s.b[518] { 1.0 } else { 0.0 };

        if s.b[518] {
            s.store_exp_mul(273, 254, 8);
        }

        if (!s.b[518]) {
            s.store_scalar(301, ((p.p151) as f64).exp());
            s.store_mul_offset_ad_rhs(273, 301, A::mul(s.ad_value(254), s.ad_value(8)), (((-p.p151)) + (1.0)));
        }

        s.b[519] = ((s.v[267] * s.v[8]) < p.p151);
        s.v[519] = if s.b[519] { 1.0 } else { 0.0 };

        if s.b[519] {
            s.store_exp_mul(275, 267, 8);
        }

        if (!s.b[519]) {
            s.store_scalar(301, ((p.p151) as f64).exp());
            s.store_mul_offset_ad_rhs(275, 301, A::mul(s.ad_value(267), s.ad_value(8)), (((-p.p151)) + (1.0)));
        }

        s.b[520] = ((s.v[259] * s.v[8]) < p.p151);
        s.v[520] = if s.b[520] { 1.0 } else { 0.0 };

        if s.b[520] {
            s.store_exp_mul(262, 259, 8);
        }

        if (!s.b[520]) {
            s.store_scalar(301, ((p.p151) as f64).exp());
            s.store_mul_offset_ad_rhs(262, 301, A::mul(s.ad_value(259), s.ad_value(8)), (((-p.p151)) + (1.0)));
        }

        s.b[521] = ((s.v[260] * s.v[8]) < p.p151);
        s.v[521] = if s.b[521] { 1.0 } else { 0.0 };

        if s.b[521] {
            s.store_exp_mul(263, 260, 8);
        }

        if (!s.b[521]) {
            s.store_scalar(301, ((p.p151) as f64).exp());
            s.store_mul_offset_ad_rhs(263, 301, A::mul(s.ad_value(260), s.ad_value(8)), (((-p.p151)) + (1.0)));
        }

        s.b[522] = ((s.v[261] * s.v[8]) < p.p151);
        s.v[522] = if s.b[522] { 1.0 } else { 0.0 };

        if s.b[522] {
            s.store_exp_mul(264, 261, 8);
        }

        if (!s.b[522]) {
            s.store_scalar(301, ((p.p151) as f64).exp());
            s.store_mul_offset_ad_rhs(264, 301, A::mul(s.ad_value(261), s.ad_value(8)), (((-p.p151)) + (1.0)));
        }

        s.b[523] = (((s.v[267] - s.v[16]) * s.v[8]) < p.p151);
        s.v[523] = if s.b[523] { 1.0 } else { 0.0 };

        if s.b[523] {
            s.store_exp_ad(278, A::mul(A::sub(s.ad_value(267), s.ad_value(16)), s.ad_value(8)));
        }

        if (!s.b[523]) {
            s.store_scalar(301, ((p.p151) as f64).exp());
            s.store_mul_offset_ad_rhs(278, 301, A::mul(A::sub(s.ad_value(267), s.ad_value(16)), s.ad_value(8)), (((-p.p151)) + (1.0)));
        }

        s.b[524] = (((s.v[255] - s.v[16]) * s.v[8]) < p.p151);
        s.v[524] = if s.b[524] { 1.0 } else { 0.0 };

        if s.b[524] {
            s.store_exp_ad(276, A::mul(A::sub(s.ad_value(255), s.ad_value(16)), s.ad_value(8)));
        }

        if (!s.b[524]) {
            s.store_scalar(301, ((p.p151) as f64).exp());
            s.store_mul_offset_ad_rhs(276, 301, A::mul(A::sub(s.ad_value(255), s.ad_value(16)), s.ad_value(8)), (((-p.p151)) + (1.0)));
        }

        s.b[525] = (((s.v[251] - s.v[16]) * s.v[8]) < p.p151);
        s.v[525] = if s.b[525] { 1.0 } else { 0.0 };

        if s.b[525] {
            s.store_exp_ad(277, A::mul(A::sub(s.ad_value(251), s.ad_value(16)), s.ad_value(8)));
        }

        if (!s.b[525]) {
            s.store_scalar(301, ((p.p151) as f64).exp());
            s.store_mul_offset_ad_rhs(277, 301, A::mul(A::sub(s.ad_value(251), s.ad_value(16)), s.ad_value(8)), (((-p.p151)) + (1.0)));
        }

        s.b[526] = (((s.v[250] - s.v[16]) * s.v[8]) < p.p151);
        s.v[526] = if s.b[526] { 1.0 } else { 0.0 };

        if s.b[526] {
            s.store_exp_ad(279, A::mul(A::sub(s.ad_value(250), s.ad_value(16)), s.ad_value(8)));
        }

        if (!s.b[526]) {
            s.store_scalar(301, ((p.p151) as f64).exp());
            s.store_mul_offset_ad_rhs(279, 301, A::mul(A::sub(s.ad_value(250), s.ad_value(16)), s.ad_value(8)), (((-p.p151)) + (1.0)));
        }

        s.store_sqrt_offset_scaled_input(114, 277, 4.0, 1.0);

        s.store_sqrt_offset_scaled_input(115, 279, 4.0, 1.0);

        s.store_ad_value(116, A::div_scaled_inputs(s.ad_value(279), 2.0, A::offset(s.ad_value(115), 1.0), 1.0));

        s.b[527] = (s.v[116] < p.p153);
        s.v[527] = if s.b[527] { 1.0 } else { 0.0 };

        if s.b[527] {
            s.store_scalar(116, p.p153);
        }

        s.store_mul_ad_rhs(117, 6, A::add_scaled_inputs3(s.ad_value(114), 1.0, s.ad_value(115), (-1.0), A::ln(A::div(A::offset(s.ad_value(114), 1.0), A::offset(s.ad_value(115), 1.0))), -1.0));

        s.store_div_ad_lhs(118, A::add(s.ad_value(117), s.ad_value(256)), 31);

        s.b[528] = (s.v[118] > 0.0);
        s.v[528] = if s.b[528] { 1.0 } else { 0.0 };

        s.b[529] = (s.v[250] < 100.0);
        s.v[529] = if s.b[529] { 1.0 } else { 0.0 };

        if (s.b[528] && s.b[529]) {
            s.copy_ad(303, 250);
        }

        if (s.b[528] && (!s.b[529])) {
            s.store_offset_ln_ad(303, A::offset(s.ad_value(250), (((-100.0)) + (1.0))), 100.0);
        }

        if s.b[528] {
            s.store_sub_ad_lhs(119, A::add_scaled_product(s.ad_value(16), 1.0, s.ad_value(6), A::ln(A::offset(A::mul3_scaled_output(s.ad_value(118), s.ad_value(31), s.ad_value(8), 0.5), 1.0)), 2.0), 303);
            s.store_scale(298, 16, 0.2);
            s.store_square(287, 298);
            s.store_square(288, 119);
        }

        s.b[530] = (s.v[119] < 0.0);
        s.v[530] = if s.b[530] { 1.0 } else { 0.0 };

        if (s.b[528] && s.b[530]) {
            s.store_ad_value(120, A::div_scaled_inputs(s.ad_value(287), 0.5, A::sub(A::sqrt(A::add(s.ad_value(288), s.ad_value(287))), s.ad_value(119)), 1.0));
        }

        if (s.b[528] && (!s.b[530])) {
            s.store_scaled_add_ad_lhs(120, A::sqrt(A::add(s.ad_value(288), s.ad_value(287))), 119, 0.5);
        }

        if s.b[528] {
            s.store_ad_value(121, A::div_scaled_product_offset_rhs(s.ad_value(120), s.ad_value(120), (p.p62 * p.p61), 1.0, A::add_scaled_inputs(s.ad_value(120), p.p61, s.ad_value(31), (p.p62 * p.p61)), 1.0));
            s.store_div(291, 118, 121);
            s.store_scaled_offset(285, 291, (-1.0), 1.0 / (p.p63));
        }

        s.b[531] = (s.v[291] < 1.0);
        s.v[531] = if s.b[531] { 1.0 } else { 0.0 };

        if (s.b[528] && s.b[531]) {
            s.store_offset_scaled_ad(289, A::ln_one_plus_exp(s.ad_value(285)), p.p63, 1.0);
        }

        if (s.b[528] && (!s.b[531])) {
            s.store_ad_value(289, A::add_scaled_inputs(s.ad_value(291), 1.0, A::ln_one_plus_exp(A::neg(s.ad_value(285))), p.p63));
        }

        if s.b[528] {
            s.store_scale(122, 289, 1.0 / ((1.0 + (p.p63 * (((1.0 + ((((-1.0) / p.p63)) as f64).exp())) as f64).ln()))));
            s.store_scale(123, 120, 1.0 / ((p.p62 * p.p61)));
            s.store_div_ad(124, A::offset(A::sqrt(A::offset(A::mul3_scaled_output(s.ad_value(122), s.ad_value(123), A::offset(s.ad_value(123), 1.0), 4.0), 1.0)), 1.0), A::mul_scaled_lhs(s.ad_value(122), 2.0, A::offset(s.ad_value(123), 1.0)));
            s.store_div_ad(125, A::add_scaled_sub_value_product(1.0, s.ad_value(124), 1.0, s.ad_value(116), s.ad_value(124), 1.0), A::offset(A::mul(s.ad_value(116), s.ad_value(124)), 1.0));
            s.store_mul_ad_lhs(127, A::mul3_scaled_output(s.ad_value(118), s.ad_value(31), s.ad_value(125), 0.5), 8);
            s.store_ad_value(292, A::add_scaled_offset_product_rhs(s.ad_value(127), 2.0, s.ad_value(116), A::add(s.ad_value(116), s.ad_value(127)), 1.0, 1.0));
            s.store_scaled_offset(128, 127, (-1.0), 0.5);
            s.store_add_ad_lhs(286, A::square(s.ad_value(128)), 292);
        }

        s.b[532] = (s.v[127] >= 1.0);
        s.v[532] = if s.b[532] { 1.0 } else { 0.0 };

        if (s.b[528] && s.b[532]) {
            s.store_add_ad_rhs(129, 128, A::sqrt(s.ad_value(286)));
        }

        if (s.b[528] && (!s.b[532])) {
            s.store_div_ad_rhs(129, 292, A::sub(A::sqrt(s.ad_value(286)), s.ad_value(128)));
        }

        s.b[533] = (s.v[129] < p.p152);
        s.v[533] = if s.b[533] { 1.0 } else { 0.0 };

        if (s.b[528] && s.b[533]) {
            s.store_scalar(129, p.p152);
        }

        if s.b[528] {
            s.store_mul_ad_product_rhs(131, 129, A::offset(s.ad_value(129), 1.0), A::exp(A::mul(s.ad_value(16), s.ad_value(8))));
            s.store_scaled_offset(133, 118, (-p.p62), (0.5 * p.p61));
            s.store_scaled_mul(134, 31, 118, (p.p61 * p.p62));
            s.store_add_ad_rhs(135, 133, A::sqrt(A::add(A::square(s.ad_value(133)), s.ad_value(134))));
        }

        s.b[534] = (p.p73 == 0.0);
        s.v[534] = if s.b[534] { 1.0 } else { 0.0 };

        if (s.b[528] && s.b[534]) {
            s.store_scale(136, 17, 0.1);
        }

        if (s.b[528] && (!s.b[534])) {
            s.store_mul_offset_ad_rhs(136, 17, A::div_scaled_inputs(s.ad_value(118), 2.0, A::add(s.ad_value(118), s.ad_value(121)), 1.0), 0.1);
        }

        if s.b[528] {
            s.store_ad_value(137, A::div_scaled_inputs(s.ad_value(118), p.p62, A::offset(s.ad_value(118), p.p62), 1.0));
            s.store_div_from_scalar_offset_input(213, p.p62, 118, p.p62);
        }

        if (!s.b[528]) {
            s.store_scalar(121, 0.0);
            s.store_ad_value(129, A::div_scaled_inputs(s.ad_value(277), 2.0, A::offset(s.ad_value(114), 1.0), 1.0));
            s.copy_ad(131, 271);
        }

        s.b[535] = ((((s.v[256]) as f64).abs() < (1e-5 * s.v[6])) || (((s.v[117]) as f64).abs() < ((1e-40 * s.v[6]) * (s.v[114] + s.v[115]))));
        s.v[535] = if s.b[535] { 1.0 } else { 0.0 };

        if ((!s.b[528]) && s.b[535]) {
            s.store_scaled_add(138, 129, 116, 0.5);
            s.store_div_ad_rhs(125, 138, A::offset(s.ad_value(138), 1.0));
        }

        if ((!s.b[528]) && (!s.b[535])) {
            s.store_div_ad_rhs(125, 117, A::add_scaled_inputs3(s.ad_value(117), 1.0, s.ad_value(251), 1.0, s.ad_value(250), -1.0));
        }

        if (!s.b[528]) {
            s.copy_ad(135, 256);
            s.store_scale(136, 17, 0.1);
            s.copy_ad(137, 118);
            s.store_sub_from_scalar_ad(213, 1.0, A::scale(s.ad_value(137), 1.0 / (p.p62)));
        }

        s.store_scale(139, 14, (1.0 - ((3.0) as f64).powf(((-1.0) / p.p67))));

        s.store_scale(299, 14, 0.1);

        s.store_div_ad_lhs(285, A::sub(s.ad_value(252), s.ad_value(139)), 299);

        s.b[536] = (s.v[252] < s.v[139]);
        s.v[536] = if s.b[536] { 1.0 } else { 0.0 };

        if s.b[536] {
            s.store_ad_value(140, A::add_scaled_product(s.ad_value(252), 1.0, s.ad_value(299), A::ln_one_plus_exp(s.ad_value(285)), (-1.0)));
        }

        if (!s.b[536]) {
            s.store_ad_value(140, A::add_scaled_product(s.ad_value(139), 1.0, s.ad_value(299), A::ln_one_plus_exp(A::neg(s.ad_value(285))), (-1.0)));
        }

    }

    pub(super) fn stamp_transient_block_2(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.store_powf_ad(59, A::sub_from_scalar(1.0, A::mul(s.ad_value(140), s.ad_value(65))), (1.0 - p.p67));

        s.store_ad_value(141, A::add_scaled_inputs3(A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(14), 1.0, s.ad_value(59), 1.0 / ((1.0 - p.p67))), 1.0, s.ad_value(252), 3.0, s.ad_value(140), (-3.0)));

        s.b[537] = (p.p74 == 1.0);
        s.v[537] = if s.b[537] { 1.0 } else { 0.0 };

        if s.b[537] {
            s.copy_ad(142, 250);
        }

        s.b[538] = (p.p74 == 2.0);
        s.v[538] = if s.b[538] { 1.0 } else { 0.0 };

        if ((!s.b[537]) && s.b[538]) {
            s.store_add(142, 250, 135);
        }

        if ((!s.b[537]) && (!s.b[538])) {
            s.copy_ad(142, 251);
        }

        s.store_div_ad(143, A::sub_from_scalar(2.0, s.ad_value(25)), A::sub_from_scalar(1.0, s.ad_value(25)));

        s.store_mul_sub_from_scalar_ad_rhs(144, 17, 1.0, A::powf(s.ad_value(143), ((-1.0) / p.p72)));

        s.store_div_ad_lhs(285, A::sub(s.ad_value(142), s.ad_value(144)), 136);

        s.b[539] = (s.v[142] < s.v[144]);
        s.v[539] = if s.b[539] { 1.0 } else { 0.0 };

        if s.b[539] {
            s.store_ad_value(145, A::add_scaled_product(s.ad_value(142), 1.0, s.ad_value(136), A::ln_one_plus_exp(s.ad_value(285)), (-1.0)));
        }

        if (!s.b[539]) {
            s.store_ad_value(145, A::add_scaled_product(s.ad_value(144), 1.0, s.ad_value(136), A::ln_one_plus_exp(A::neg(s.ad_value(285))), (-1.0)));
        }

        s.store_powf(146, 213, p.p76);

        s.store_add_ad(147, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(17), 1.0, A::mul(s.ad_value(146), A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(145), s.ad_value(17))), (1.0 - p.p72))), 1.0 / ((1.0 - p.p72))), A::mul3(s.ad_value(146), s.ad_value(143), A::sub(s.ad_value(142), s.ad_value(145))));

        s.store_ad_value(148, A::add_scaled_product(A::mul_sub_from_scalar_lhs(1.0, s.ad_value(25), s.ad_value(147)), 1.0, s.ad_value(25), s.ad_value(250), 1.0));

        s.store_scaled_div(149, 35, 36, 4.0);

        s.store_mul(150, 149, 272);

        s.store_div_ad_rhs(152, 150, A::offset(A::sqrt(A::offset(s.ad_value(150), 1.0)), 1.0));

        s.store_pow_ad(132, s.ad_value(131), A::div_from_scalar(1.0, s.ad_value(49)));

        s.store_mul(151, 149, 132);

        s.store_div_ad_rhs(153, 151, A::offset(A::sqrt(A::offset(s.ad_value(151), 1.0)), 1.0));

        s.b[540] = (p.p92 == 0.0);
        s.v[540] = if s.b[540] { 1.0 } else { 0.0 };

        if s.b[540] {
            s.store_add_ad(154, A::offset(A::div(s.ad_value(141), s.ad_value(41)), 1.0), A::div(s.ad_value(148), s.ad_value(40)));
        }

        if (!s.b[540]) {
            s.store_mul_ad_product_lhs(295, A::offset(A::div(s.ad_value(141), s.ad_value(41)), 1.0), s.ad_value(100), 8);
            s.store_mul_ad_product_lhs(296, A::div_scaled_inputs(s.ad_value(148), -1.0, s.ad_value(40), 1.0), s.ad_value(100), 8);
            s.store_div_ad(154, A::sub(A::exp(s.ad_value(295)), A::exp(s.ad_value(296))), A::offset(A::exp(A::mul(s.ad_value(100), s.ad_value(8))), (-1.0)));
        }

        s.v[287] = (0.1 * 0.1);

        s.store_square(288, 154);

        s.b[541] = (s.v[154] < 0.0);
        s.v[541] = if s.b[541] { 1.0 } else { 0.0 };

        if s.b[541] {
            s.store_div_from_scalar_sub_ad(155, (0.5 * s.v[287]), A::sqrt(A::offset(s.ad_value(288), s.v[287])), s.ad_value(154));
        }

        if (!s.b[541]) {
            s.store_scaled_add_ad_lhs(155, A::sqrt(A::offset(s.ad_value(288), s.v[287])), 154, 0.5);
        }

        s.store_mul_offset_ad_rhs(156, 155, A::add_scaled_inputs(s.ad_value(152), 0.5, s.ad_value(153), 0.5), 1.0);

        s.store_scaled_mul(157, 35, 132, p.p15);

        s.store_mul(158, 35, 272);

        s.store_div_ad_lhs(159, A::sub(s.ad_value(158), s.ad_value(157)), 156);

        s.store_scale(285, 252, 10000.0);

        s.b[542] = (s.v[252] < 0.0);
        s.v[542] = if s.b[542] { 1.0 } else { 0.0 };

        if s.b[542] {
            s.store_scaled_ln_one_plus_exp(302, 285, 0.0001);
        }

        if (!s.b[542]) {
            s.store_ad_value(302, A::add_scaled_inputs(s.ad_value(252), 1.0, A::ln_one_plus_exp(A::neg(s.ad_value(285))), 0.0001));
        }

        s.store_scale(304, 302, 1.0 / (p.p156));

        s.b[543] = (s.v[304] < p.p151);
        s.v[543] = if s.b[543] { 1.0 } else { 0.0 };

        if s.b[543] {
            s.store_exp(305, 304);
        }

        if (!s.b[543]) {
            s.store_scalar(301, ((p.p151) as f64).exp());
            s.store_mul_offset_rhs(305, 301, 304, (((-p.p151)) + (1.0)));
        }

        s.store_mul_offset_rhs(358, 357, 305, (-1.0));

        s.store_scaled_offset(285, 252, (-p.p158), 1000.0);

        s.b[544] = (s.v[252] < p.p158);
        s.v[544] = if s.b[544] { 1.0 } else { 0.0 };

        if s.b[544] {
            s.store_ad_value(306, A::sub_scaled_inputs(s.ad_value(252), 1.0, A::ln_one_plus_exp(s.ad_value(285)), 0.001));
        }

        if (!s.b[544]) {
            s.store_sub_from_scalar_ad(306, p.p158, A::scale(A::ln_one_plus_exp(A::neg(s.ad_value(285))), 0.001));
        }

        s.store_mul_scaled_ad_rhs(359, 306, p.p159, A::powf(A::sub_from_scalar(p.p158, s.ad_value(306)), 2.0));

        s.b[545] = (((s.v[252] * s.v[8]) / p.p17) < p.p151);
        s.v[545] = if s.b[545] { 1.0 } else { 0.0 };

        if s.b[545] {
            s.store_ad_value(302, A::exp_scaled_input(A::mul(s.ad_value(252), s.ad_value(8)), 1.0 / (p.p17)));
        }

        if (!s.b[545]) {
            s.store_scalar(301, ((p.p151) as f64).exp());
            s.store_mul_offset_ad_rhs(302, 301, A::mul_scaled_output(s.ad_value(252), s.ad_value(8), 1.0 / (p.p17)), (((-p.p151)) + (1.0)));
        }

        s.b[546] = (p.p24 == 1.0);
        s.v[546] = if s.b[546] { 1.0 } else { 0.0 };

        s.b[547] = (((s.v[252] - s.v[55]) * s.v[8]) < p.p151);
        s.v[547] = if s.b[547] { 1.0 } else { 0.0 };

        if (s.b[546] && s.b[547]) {
            s.store_exp_ad(304, A::mul(A::sub(s.ad_value(252), s.ad_value(55)), s.ad_value(8)));
        }

        if (s.b[546] && (!s.b[547])) {
            s.store_scalar(301, ((p.p151) as f64).exp());
            s.store_mul_offset_ad_rhs(304, 301, A::mul(A::sub(s.ad_value(252), s.ad_value(55)), s.ad_value(8)), (((-p.p151)) + (1.0)));
        }

        s.b[548] = (((s.v[159] / s.v[35]) - 1000.0) < 40.0);
        s.v[548] = if s.b[548] { 1.0 } else { 0.0 };

        if (s.b[546] && s.b[548]) {
            s.store_exp_ad(305, A::offset(A::div(s.ad_value(159), s.ad_value(35)), (-1000.0)));
        }

        if (s.b[546] && (!s.b[548])) {
            s.store_scalar(301, ((40.0) as f64).exp());
            s.store_mul_offset_ad_rhs(305, 301, A::div(s.ad_value(159), s.ad_value(35)), (((((-1000.0)) + ((-40.0)))) + (1.0)));
        }

        if s.b[546] {
            let assign4040_ad_e3794: A = A::add(A::add_scaled_products(s.ad_value(42), A::offset(s.ad_value(302), (-1.0)), 1.0, A::div_scaled_product_offset_denominator(s.ad_value(53), A::offset(s.ad_value(302), (-1.0)), 2.0, A::sqrt(A::scale_offset(s.ad_value(304), 4.0, 1.0)), 1.0, 1.0), A::offset(A::div(s.ad_value(148), s.ad_value(40)), 1.0), 1.0), A::div_scaled_product3(s.ad_value(54), A::offset(s.ad_value(131), (-1.0)), s.ad_value(305), 1.0, A::offset(s.ad_value(305), 1.0), 1.0));
            s.store_ad_value(161, assign4040_ad_e3794);
        }

        s.b[549] = (p.p93 == 0.0);
        s.v[549] = if s.b[549] { 1.0 } else { 0.0 };

        if ((!s.b[546]) && s.b[549]) {
            s.store_mul_offset_rhs(161, 42, 302, (-1.0));
        }

        if ((!s.b[546]) && (!s.b[549])) {
            s.store_mul_ad_rhs(161, 42, A::add_scaled_offset_product_lhs(A::scaled_offset(s.ad_value(302), (-1.0), (1.0 - p.p93)), 1.0, A::add(s.ad_value(302), s.ad_value(131)), (-2.0), A::offset(A::div(s.ad_value(148), s.ad_value(40)), 1.0), p.p93));
        }

        s.b[550] = (((s.v[253] * s.v[8]) / p.p19) < p.p151);
        s.v[550] = if s.b[550] { 1.0 } else { 0.0 };

        if s.b[550] {
            s.store_ad_value(302, A::exp_scaled_input(A::mul(s.ad_value(253), s.ad_value(8)), 1.0 / (p.p19)));
        }

        if (!s.b[550]) {
            s.store_scalar(301, ((p.p151) as f64).exp());
            s.store_mul_offset_ad_rhs(302, 301, A::mul_scaled_output(s.ad_value(253), s.ad_value(8), 1.0 / (p.p19)), (((-p.p151)) + (1.0)));
        }

        s.b[551] = (p.p24 == 1.0);
        s.v[551] = if s.b[551] { 1.0 } else { 0.0 };

        s.b[552] = (((s.v[253] - s.v[55]) * s.v[8]) < p.p151);
        s.v[552] = if s.b[552] { 1.0 } else { 0.0 };

        if (s.b[551] && s.b[552]) {
            s.store_exp_ad(304, A::mul(A::sub(s.ad_value(253), s.ad_value(55)), s.ad_value(8)));
        }

        if (s.b[551] && (!s.b[552])) {
            s.store_scalar(301, ((p.p151) as f64).exp());
            s.store_mul_offset_ad_rhs(304, 301, A::mul(A::sub(s.ad_value(253), s.ad_value(55)), s.ad_value(8)), (((-p.p151)) + (1.0)));
        }

        if s.b[551] {
            s.store_ad_value(162, A::add_scaled_offset_product_rhs(A::div_scaled_product_offset_denominator(s.ad_value(45), A::offset(s.ad_value(302), (-1.0)), 2.0, A::sqrt(A::scale_offset(s.ad_value(304), 4.0, 1.0)), 1.0, 1.0), 1.0, s.ad_value(44), s.ad_value(302), (-1.0), 1.0));
        }

        if (!s.b[551]) {
            s.store_mul_offset_rhs(162, 44, 302, (-1.0));
        }

        s.b[553] = (((s.v[252] * s.v[8]) / p.p21) < p.p151);
        s.v[553] = if s.b[553] { 1.0 } else { 0.0 };

        if s.b[553] {
            s.store_ad_value(302, A::exp_scaled_input(A::mul(s.ad_value(252), s.ad_value(8)), 1.0 / (p.p21)));
        }

        if (!s.b[553]) {
            s.store_scalar(301, ((p.p151) as f64).exp());
            s.store_mul_offset_ad_rhs(302, 301, A::mul_scaled_output(s.ad_value(252), s.ad_value(8), 1.0 / (p.p21)), (((-p.p151)) + (1.0)));
        }

        s.store_mul_offset_rhs(163, 38, 302, (-1.0));

        s.b[554] = (((s.v[253] * s.v[8]) / p.p23) < p.p151);
        s.v[554] = if s.b[554] { 1.0 } else { 0.0 };

        if s.b[554] {
            s.store_ad_value(302, A::exp_scaled_input(A::mul(s.ad_value(253), s.ad_value(8)), 1.0 / (p.p23)));
        }

        if (!s.b[554]) {
            s.store_scalar(301, ((p.p151) as f64).exp());
            s.store_mul_offset_ad_rhs(302, 301, A::mul_scaled_output(s.ad_value(253), s.ad_value(8), 1.0 / (p.p23)), (((-p.p151)) + (1.0)));
        }

        s.store_mul_offset_rhs(165, 46, 302, (-1.0));

        s.b[555] = (((s.v[255] * s.v[8]) / p.p32) < p.p151);
        s.v[555] = if s.b[555] { 1.0 } else { 0.0 };

        if s.b[555] {
            s.store_ad_value(302, A::exp_scaled_input(A::mul(s.ad_value(255), s.ad_value(8)), 1.0 / (p.p32)));
        }

        if (!s.b[555]) {
            s.store_scalar(301, ((p.p151) as f64).exp());
            s.store_mul_offset_ad_rhs(302, 301, A::mul_scaled_output(s.ad_value(255), s.ad_value(8), 1.0 / (p.p32)), (((-p.p151)) + (1.0)));
        }

        s.store_mul_offset_rhs(164, 39, 302, (-1.0));

        s.b[556] = (((s.v[253] * s.v[8]) / p.p150) < p.p151);
        s.v[556] = if s.b[556] { 1.0 } else { 0.0 };

        if s.b[556] {
            s.store_ad_value(302, A::exp_scaled_input(A::mul(s.ad_value(253), s.ad_value(8)), 1.0 / (p.p150)));
        }

        if (!s.b[556]) {
            s.store_scalar(301, ((p.p151) as f64).exp());
            s.store_mul_offset_ad_rhs(302, 301, A::mul_scaled_output(s.ad_value(253), s.ad_value(8), 1.0 / (p.p150)), (((-p.p151)) + (1.0)));
        }

        s.store_mul_offset_rhs(166, 47, 302, (-1.0));

        s.b[557] = (((p.p34 > 0.0) && (p.p35 > 0.0)) && (s.v[252] < 0.0));
        s.v[557] = if s.b[557] { 1.0 } else { 0.0 };

        s.b[558] = ((s.v[61] * (1.0 - (s.v[62] / (2.0 * s.v[59])))) < p.p151);
        s.v[558] = if s.b[558] { 1.0 } else { 0.0 };

        if (s.b[557] && s.b[558]) {
            s.store_exp_ad(68, A::mul_sub_from_scalar_rhs(s.ad_value(61), 1.0, A::div_from_scalar(s.v[62], A::scale(s.ad_value(59), 2.0))));
        }

        if (s.b[557] && (!s.b[558])) {
            s.store_scalar(301, ((p.p151) as f64).exp());
            s.store_mul_offset_ad_rhs(68, 301, A::mul_sub_from_scalar_rhs(s.ad_value(61), 1.0, A::div_from_scalar(s.v[62], A::scale(s.ad_value(59), 2.0))), (((-p.p151)) + (1.0)));
        }

        if s.b[557] {
            s.store_mul(281, 252, 65);
            s.store_scaled_mul_ad(60, A::powf(A::sqrt(A::offset(A::square(s.ad_value(281)), 1e-30)), ((-2.0) - p.p67)), A::sub(A::scale_offset(A::scale(s.ad_value(281), (3.0 * (p.p67 - 1.0))), (-p.p67), (((1.0 - (p.p67 * p.p67))) * (p.p67))), A::mul3_scaled_output(s.ad_value(281), s.ad_value(281), A::offset(s.ad_value(281), (p.p67 - 1.0)), 6.0)), 0.16666666666666666);
            s.store_ad_value(281, A::div_scaled_product_by_product(s.ad_value(252), s.ad_value(61), s.v[62], s.ad_value(70), s.ad_value(60), 1.0));
        }

        s.b[559] = (s.v[281] < (-0.001));
        s.v[559] = if s.b[559] { 1.0 } else { 0.0 };

        s.b[560] = (s.v[281] < p.p151);
        s.v[560] = if s.b[560] { 1.0 } else { 0.0 };

        if ((s.b[557] && s.b[559]) && s.b[560]) {
            s.store_exp(91, 281);
        }

        if ((s.b[557] && s.b[559]) && (!s.b[560])) {
            s.store_scalar(301, ((p.p151) as f64).exp());
            s.store_mul_offset_rhs(91, 301, 281, (((-p.p151)) + (1.0)));
        }

        if (s.b[557] && s.b[559]) {
            s.store_mul_scaled_ad_rhs(69, 252, -1.0, A::offset(A::div(A::sub_from_scalar(1.0, s.ad_value(91)), s.ad_value(281)), 1.0));
        }

        if (s.b[557] && (!s.b[559])) {
            s.store_mul_ad_affine_product_rhs(69, 252, s.ad_value(281), A::offset(A::mul_scaled_lhs(s.ad_value(281), 0.3333333333333333, A::scale_offset(s.ad_value(281), 0.25, 1.0)), 1.0), 0.5, 0.0);
        }

        if s.b[557] {
            s.store_mul_ad_affine_product_lhs(57, A::mul3_scaled_output(s.ad_value(58), s.ad_value(69), s.ad_value(59), 2.0), s.ad_value(68), s.v[63], 0.0, 65);
        }

        if (!s.b[557]) {
            s.store_scalar(69, 0.0);
            s.store_scalar(57, 0.0);
        }

        s.b[561] = (((p.p36 > 0.0) && (p.p37 > 0.0)) && (s.v[250] < 0.0));
        s.v[561] = if s.b[561] { 1.0 } else { 0.0 };

        if s.b[561] {
            s.store_powf_ad(77, A::sub_from_scalar(1.0, A::mul(s.ad_value(250), s.ad_value(67))), (1.0 - s.v[76]));
        }

        s.b[562] = ((s.v[83] * (1.0 - (s.v[79] / (2.0 * s.v[77])))) < p.p151);
        s.v[562] = if s.b[562] { 1.0 } else { 0.0 };

        if (s.b[561] && s.b[562]) {
            s.store_exp_ad(78, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::div_from_scalar(s.v[79], A::scale(s.ad_value(77), 2.0))));
        }

        if (s.b[561] && (!s.b[562])) {
            s.store_scalar(301, ((p.p151) as f64).exp());
            s.store_mul_offset_ad_rhs(78, 301, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::div_from_scalar(s.v[79], A::scale(s.ad_value(77), 2.0))), (((-p.p151)) + (1.0)));
        }

        if s.b[561] {
            s.store_mul(283, 250, 67);
        }

        if s.b[561] {
            let assign4640_ad_e4484: A = A::mul_scaled_output(A::powf(A::sqrt(A::offset(A::square(s.ad_value(283)), 1e-30)), ((-2.0) - s.v[76])), A::sub(A::scale_offset(A::scale(s.ad_value(283), (3.0 * (s.v[76] - 1.0))), (-s.v[76]), (((1.0 - (s.v[76] * s.v[76]))) * (s.v[76]))), A::mul3_scaled_output(s.ad_value(283), s.ad_value(283), A::offset(s.ad_value(283), (s.v[76] - 1.0)), 6.0)), 0.16666666666666666);
            s.store_ad_value(80, assign4640_ad_e4484);
        }

        if s.b[561] {
            s.store_ad_value(283, A::div_scaled_product_by_product(s.ad_value(250), s.ad_value(83), s.v[79], s.ad_value(85), s.ad_value(80), 1.0));
        }

        s.b[563] = (s.v[283] < (-0.001));
        s.v[563] = if s.b[563] { 1.0 } else { 0.0 };

        s.b[564] = (s.v[283] < p.p151);
        s.v[564] = if s.b[564] { 1.0 } else { 0.0 };

        if ((s.b[561] && s.b[563]) && s.b[564]) {
            s.store_exp(92, 283);
        }

        if ((s.b[561] && s.b[563]) && (!s.b[564])) {
            s.store_scalar(301, ((p.p151) as f64).exp());
            s.store_mul_offset_rhs(92, 301, 283, (((-p.p151)) + (1.0)));
        }

        if (s.b[561] && s.b[563]) {
            s.store_mul_scaled_ad_rhs(81, 250, -1.0, A::offset(A::div(A::sub_from_scalar(1.0, s.ad_value(92)), s.ad_value(283)), 1.0));
        }

        if (s.b[561] && (!s.b[563])) {
            s.store_mul_ad_affine_product_rhs(81, 250, s.ad_value(283), A::offset(A::mul_scaled_lhs(s.ad_value(283), 0.3333333333333333, A::scale_offset(s.ad_value(283), 0.25, 1.0)), 1.0), 0.5, 0.0);
        }

        if s.b[561] {
            s.store_mul_ad_affine_product_lhs(82, A::mul3_scaled_output(s.ad_value(84), s.ad_value(81), s.ad_value(77), 2.0), s.ad_value(78), s.v[89], 0.0, 67);
        }

        if (!s.b[561]) {
            s.store_scalar(81, 0.0);
            s.store_scalar(82, 0.0);
        }

        s.store_mul(168, 149, 274);

        s.store_scale(169, 276, 4.0);

        s.store_div_ad(171, A::sub(s.ad_value(168), s.ad_value(149)), A::offset(A::sqrt(A::offset(s.ad_value(168), 1.0)), 1.0));

        s.store_div_ad_rhs(170, 169, A::offset(A::sqrt(A::offset(s.ad_value(169), 1.0)), 1.0));

        s.store_ad_value(167, A::div_scaled_product_offset_denominator(s.ad_value(43), A::offset(s.ad_value(274), (-1.0)), 2.0, A::sqrt(A::offset(A::mul(A::div_scaled_inputs(s.ad_value(43), 4.0, s.ad_value(37), 1.0), s.ad_value(274)), 1.0)), 1.0, 1.0));

        s.b[565] = (p.p8 == 1.0);
        s.v[565] = if s.b[565] { 1.0 } else { 0.0 };

        if s.b[565] {
            s.store_ad_value(185, A::div_scaled_product_offset_denominator(s.ad_value(107), A::sub(s.ad_value(271), s.ad_value(262)), (p.p143 * 2.0), A::sqrt(A::offset(A::mul_scaled_lhs(A::div(s.ad_value(107), s.ad_value(109)), 4.0, A::add_scaled_inputs(s.ad_value(271), 1.0, s.ad_value(262), p.p144)), 1.0)), 1.0, 1.0));
            s.store_ad_value(182, A::div_scaled_product_offset_denominator(s.ad_value(107), A::sub(s.ad_value(274), s.ad_value(264)), ((1.0 - p.p143) * 2.0), A::sqrt(A::offset(A::mul_scaled_lhs(A::div(s.ad_value(107), s.ad_value(109)), 4.0, A::add_scaled_inputs(s.ad_value(274), 1.0, s.ad_value(264), p.p144)), 1.0)), 1.0, 1.0));
        }

        if (!s.b[565]) {
            s.store_ad_value(185, A::div_scaled_product_offset_denominator(s.ad_value(107), A::offset(s.ad_value(271), (-1.0)), (p.p143 * 2.0), A::sqrt(A::offset(A::mul_scaled_lhs(A::div(s.ad_value(107), s.ad_value(109)), 4.0, s.ad_value(271)), 1.0)), 1.0, 1.0));
            s.store_ad_value(182, A::div_scaled_product_offset_denominator(s.ad_value(107), A::offset(s.ad_value(274), (-1.0)), ((1.0 - p.p143) * 2.0), A::sqrt(A::offset(A::mul_scaled_lhs(A::div(s.ad_value(107), s.ad_value(109)), 4.0, s.ad_value(274)), 1.0)), 1.0, 1.0));
        }

        s.store_ad_value(184, A::add_scaled_product(A::div_scaled_product_offset_denominator(s.ad_value(108), A::offset(s.ad_value(262), (-1.0)), 2.0, A::sqrt(A::offset(A::mul_scaled_lhs(A::div(s.ad_value(108), s.ad_value(110)), (p.p144 * 4.0), s.ad_value(262)), 1.0)), 1.0, 1.0), 1.0, s.ad_value(259), s.ad_value(348), 1.0));

        s.v[183] = 0.0;

        s.b[566] = ((p.p5 > 0.0) && (p.p33 > 0.0));
        s.v[566] = if s.b[566] { 1.0 } else { 0.0 };

        if s.b[566] {
            s.store_scale(167, 167, s.v[160]);
            s.store_scale(182, 182, s.v[160]);
            s.store_ad_value(174, A::div_scaled_product_offset_denominator(s.ad_value(43), A::offset(s.ad_value(275), (-1.0)), (p.p33 * 2.0), A::sqrt(A::offset(A::mul(A::div_scaled_inputs(s.ad_value(43), 4.0, s.ad_value(37), 1.0), s.ad_value(275)), 1.0)), 1.0, 1.0));
        }

        s.b[567] = (p.p8 == 1.0);
        s.v[567] = if s.b[567] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_block_3(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[566] && s.b[567]) {
            s.store_ad_value(175, A::div_scaled_product_offset_denominator(s.ad_value(107), A::sub(s.ad_value(275), s.ad_value(263)), (((1.0 - p.p143) * p.p33) * 2.0), A::sqrt(A::offset(A::mul(A::div_scaled_inputs(s.ad_value(107), 4.0, s.ad_value(109), 1.0), A::add_scaled_inputs(s.ad_value(275), 1.0, s.ad_value(263), p.p144)), 1.0)), 1.0, 1.0));
        }

        if (s.b[566] && (!s.b[567])) {
            s.store_ad_value(175, A::div_scaled_product_offset_denominator(s.ad_value(107), A::offset(s.ad_value(275), (-1.0)), (((1.0 - p.p143) * p.p33) * 2.0), A::sqrt(A::offset(A::mul(A::div_scaled_inputs(s.ad_value(107), 4.0, s.ad_value(109), 1.0), s.ad_value(275)), 1.0)), 1.0, 1.0));
        }

        s.b[568] = (p.p5 == 1.0);
        s.v[568] = if s.b[568] { 1.0 } else { 0.0 };

        if (s.b[566] && s.b[568]) {
            s.store_mul_scale_ad_lhs(297, A::add(s.ad_value(43), s.ad_value(107)), p.p33, 32);
            s.store_mul_sub_from_scalar_ad_rhs(176, 6, 2.0, A::ln(A::mul(s.ad_value(297), s.ad_value(8))));
            s.store_sub(290, 267, 176);
            s.store_scalar(287, (0.11 * 0.11));
            s.store_square(288, 290);
        }

        s.b[569] = (s.v[290] < 0.0);
        s.v[569] = if s.b[569] { 1.0 } else { 0.0 };

        if ((s.b[566] && s.b[568]) && s.b[569]) {
            s.store_ad_value(177, A::div_scaled_inputs(s.ad_value(287), 0.5, A::sub(A::sqrt(A::add(s.ad_value(288), s.ad_value(287))), s.ad_value(290)), 1.0));
        }

        if ((s.b[566] && s.b[568]) && (!s.b[569])) {
            s.store_scaled_add_ad_lhs(177, A::sqrt(A::add(s.ad_value(288), s.ad_value(287))), 290, 0.5);
        }

        if (s.b[566] && s.b[568]) {
            s.store_div_ad_rhs(178, 177, A::add(A::add_scaled_product(s.ad_value(297), 1.0, A::add(s.ad_value(174), s.ad_value(175)), s.ad_value(32), 1.0), s.ad_value(177)));
        }

        if (s.b[566] && (!s.b[568])) {
            s.store_scalar(176, 0.0);
            s.store_scalar(290, 0.0);
            s.store_scalar(177, 0.0);
            s.store_scalar(178, 1.0);
        }

        if s.b[566] {
            s.store_mul(179, 178, 174);
            s.store_mul(183, 178, 175);
        }

        s.b[570] = (p.p84 == 1.0);
        s.v[570] = if s.b[570] { 1.0 } else { 0.0 };

        if s.b[570] {
            s.store_add(353, 254, 250);
            s.store_scalar(287, (1e-6 * 1e-6));
            s.store_scaled_mul(288, 353, 353, ((-1.0) * (-1.0)));
        }

        s.b[571] = (((-1.0) * s.v[353]) < 0.0);
        s.v[571] = if s.b[571] { 1.0 } else { 0.0 };

        if (s.b[570] && s.b[571]) {
            s.store_ad_value(354, A::div_scaled_inputs(s.ad_value(287), 0.5, A::sub_scaled_inputs(A::sqrt(A::add(s.ad_value(288), s.ad_value(287))), 1.0, s.ad_value(353), (-1.0)), 1.0));
        }

        if (s.b[570] && (!s.b[571])) {
            s.store_ad_value(354, A::add_scaled_inputs(A::sqrt(A::add(s.ad_value(288), s.ad_value(287))), 0.5, s.ad_value(353), ((-1.0) * 0.5)));
        }

        if s.b[570] {
            s.store_scalar(355, (1.0 / (1.0 - ((s.v[349]) as f64).powf(p.p82))));
            s.store_scalar(350, (s.v[349] * p.p81));
            s.store_scaled_square(352, 355, (((s.v[349]) as f64).powf((p.p82 - 1.0)) * (p.p82 * 1.0 / (p.p81))));
        }

        s.b[572] = (s.v[354] < s.v[350]);
        s.v[572] = if s.b[572] { 1.0 } else { 0.0 };

        if (s.b[570] && s.b[572]) {
            s.store_div_from_scalar_sub_from_scalar_ad(351, 1.0, 1.0, A::powf(A::scale(s.ad_value(354), 1.0 / (p.p81)), p.p82));
        }

        if (s.b[570] && (!s.b[572])) {
            s.store_ad_value(351, A::add_scaled_product(s.ad_value(355), 1.0, A::sub(s.ad_value(354), s.ad_value(350)), s.ad_value(352), 1.0));
        }

        if (!s.b[570]) {
            s.store_scalar(351, 1.0);
        }

        s.store_mul(82, 82, 351);

        s.store_mul(167, 167, 351);

        s.store_mul(164, 164, 351);

        s.store_mul(179, 179, 351);

        s.store_add_ad(186, A::offset(A::div(s.ad_value(141), s.ad_value(41)), 1.0), A::div(s.ad_value(148), s.ad_value(40)));

        s.v[287] = (0.1 * 0.1);

        s.store_square(288, 186);

        s.b[573] = (s.v[186] < 0.0);
        s.v[573] = if s.b[573] { 1.0 } else { 0.0 };

        if s.b[573] {
            s.store_div_from_scalar_sub_ad(187, (0.5 * s.v[287]), A::sqrt(A::offset(s.ad_value(288), s.v[287])), s.ad_value(186));
        }

        if (!s.b[573]) {
            s.store_scaled_add_ad_lhs(187, A::sqrt(A::offset(s.ad_value(288), s.v[287])), 186, 0.5);
        }

        s.store_mul_offset_ad_rhs(188, 187, A::add_scaled_inputs(s.ad_value(152), 0.5, s.ad_value(153), 0.5), 1.0);

        s.store_div(190, 29, 188);

        s.b[574] = (s.v[190] < s.v[346]);
        s.v[574] = if s.b[574] { 1.0 } else { 0.0 };

        if s.b[574] {
            s.copy_ad(190, 346);
        }

        s.store_scale(189, 190, 3.0);

        s.store_div_ad_lhs(191, A::add_scaled_offset_product_rhs(s.ad_value(254), 1.0, s.ad_value(6), s.ad_value(273), (-1.0), 2.0), 189);

        s.b[575] = (s.v[159] > 0.0);
        s.v[575] = if s.b[575] { 1.0 } else { 0.0 };

        s.b[576] = (p.p39 == 1.0);
        s.v[576] = if s.b[576] { 1.0 } else { 0.0 };

        s.b[577] = (s.v[250] < p.p44);
        s.v[577] = if s.b[577] { 1.0 } else { 0.0 };

        s.b[578] = (((-s.v[159]) / p.p42) < p.p151);
        s.v[578] = if s.b[578] { 1.0 } else { 0.0 };

        if (((s.b[575] && s.b[576]) && s.b[577]) && s.b[578]) {
            s.store_exp_scaled_input(338, 159, (-1.0 / (p.p42)));
        }

        if (((s.b[575] && s.b[576]) && s.b[577]) && (!s.b[578])) {
            s.store_scalar(301, ((p.p151) as f64).exp());
            s.store_mul_ad_rhs(338, 301, A::scale_offset(s.ad_value(159), (-1.0 / (p.p42)), (((-p.p151)) + (1.0))));
        }

        if ((s.b[575] && s.b[576]) && s.b[577]) {
            s.store_mul_sub_from_scalar_lhs(339, p.p44, 250, 338);
        }

        s.b[579] = (((-s.v[340]) * ((s.v[339]) as f64).powf(p.p41)) < p.p151);
        s.v[579] = if s.b[579] { 1.0 } else { 0.0 };

        if (((s.b[575] && s.b[576]) && s.b[577]) && s.b[579]) {
            s.store_exp_ad(343, A::mul_scaled_lhs(s.ad_value(340), -1.0, A::powf(s.ad_value(339), p.p41)));
        }

        if (((s.b[575] && s.b[576]) && s.b[577]) && (!s.b[579])) {
            s.store_scalar(301, ((p.p151) as f64).exp());
            s.store_mul_offset_ad_rhs(343, 301, A::mul_scaled_lhs(s.ad_value(340), -1.0, A::powf(s.ad_value(339), p.p41)), (((-p.p151)) + (1.0)));
        }

        if ((s.b[575] && s.b[576]) && s.b[577]) {
            s.store_mul_ad_product_lhs(210, A::div_from_scalar(p.p40, s.ad_value(340)), s.ad_value(339), 343);
        }

        s.b[580] = (p.p39 == 2.0);
        s.v[580] = if s.b[580] { 1.0 } else { 0.0 };

        s.b[581] = (s.v[250] < s.v[16]);
        s.v[581] = if s.b[581] { 1.0 } else { 0.0 };

        if (((s.b[575] && (!s.b[576])) && s.b[580]) && s.b[581]) {
            s.store_scalar(199, ((2.0 * p.p46) / (p.p45 * p.p45)));
            s.store_div_ad_lhs(286, A::sub(s.ad_value(16), s.ad_value(250)), 213);
            s.store_sqrt_ad(200, A::div_scaled_inputs(s.ad_value(286), 2.0, s.ad_value(199), 1.0));
        }

        s.b[582] = (p.p7 == 0.0);
        s.v[582] = if s.b[582] { 1.0 } else { 0.0 };

        if ((((s.b[575] && (!s.b[576])) && s.b[580]) && s.b[581]) && s.b[582]) {
            s.store_scalar(201, p.p45);
        }

        if ((((s.b[575] && (!s.b[576])) && s.b[580]) && s.b[581]) && (!s.b[582])) {
            s.store_sub_from_scalar_ad(126, 1.0, A::scale(s.ad_value(125), 0.5));
            s.store_scaled_mul(201, 126, 126, p.p45);
        }

        if (((s.b[575] && (!s.b[576])) && s.b[580]) && s.b[581]) {
            s.store_ad_value(202, A::div_scaled_product(s.ad_value(200), s.ad_value(201), 1.0, A::sqrt(A::add(A::square(s.ad_value(200)), A::square(s.ad_value(201)))), 1.0));
            s.store_div_ad_lhs(203, A::sub(s.ad_value(16), s.ad_value(250)), 202);
            s.store_add_ad_rhs(204, 203, A::mul3_scaled_output(s.ad_value(202), s.ad_value(199), s.ad_value(213), 0.5));
        }

        s.b[583] = (p.p7 == 0.0);
        s.v[583] = if s.b[583] { 1.0 } else { 0.0 };

        if ((((s.b[575] && (!s.b[576])) && s.b[580]) && s.b[581]) && s.b[583]) {
            s.copy_ad(205, 204);
        }

        if ((((s.b[575] && (!s.b[576])) && s.b[580]) && s.b[581]) && (!s.b[583])) {
            s.store_offset_scaled(206, 125, ((2.0) * ((2.0 * p.p47))), (((2.0 * p.p47)) + (1.0)));
            s.store_scalar(207, ((1.0 + p.p47) / (1.0 + (2.0 * p.p47))));
            s.store_sub_ad_rhs(208, 203, A::mul3_scaled_output(s.ad_value(202), s.ad_value(199), A::sub(s.ad_value(207), A::div_scaled_inputs(s.ad_value(159), 1.0, s.ad_value(206), p.p62)), 0.5));
            s.store_ad_value(286, A::add_scaled_product(A::mul3_scaled_output(s.ad_value(203), s.ad_value(203), s.ad_value(137), (0.1 * 1.0 / (p.p62))), 1.0, A::sub(s.ad_value(208), s.ad_value(204)), A::sub(s.ad_value(208), s.ad_value(204)), 1.0));
            s.store_ad_value(205, A::add_scaled_inputs3(s.ad_value(208), 0.5, s.ad_value(204), 0.5, A::sqrt(s.ad_value(286)), 0.5));
        }

        if (((s.b[575] && (!s.b[576])) && s.b[580]) && s.b[581]) {
            s.store_div_ad_lhs(293, A::sub(s.ad_value(205), s.ad_value(203)), 205);
        }

        s.b[584] = (((s.v[293]) as f64).abs() > 1e-7);
        s.v[584] = if s.b[584] { 1.0 } else { 0.0 };

        if ((((s.b[575] && (!s.b[576])) && s.b[580]) && s.b[581]) && s.b[584]) {
            s.store_scaled_div(209, 202, 293, 0.5);
            s.store_mul_ad(210, A::mul3(A::div(s.ad_value(0), s.ad_value(99)), s.ad_value(205), s.ad_value(209)), A::sub(A::exp(A::div_scaled_inputs(s.ad_value(99), -1.0, s.ad_value(205), 1.0)), A::exp(A::mul_offset_rhs(A::div_scaled_inputs(s.ad_value(99), -1.0, s.ad_value(205), 1.0), A::div(s.ad_value(201), s.ad_value(209)), 1.0))));
        }

        if ((((s.b[575] && (!s.b[576])) && s.b[580]) && s.b[581]) && (!s.b[584])) {
            s.store_mul_ad_product_rhs(210, 0, s.ad_value(201), A::exp(A::div_scaled_inputs(s.ad_value(99), -1.0, s.ad_value(205), 1.0)));
        }

        s.b[585] = (p.p39 == 3.0);
        s.v[585] = if s.b[585] { 1.0 } else { 0.0 };

        s.b[586] = (s.v[250] < p.p44);
        s.v[586] = if s.b[586] { 1.0 } else { 0.0 };

        if ((((s.b[575] && (!s.b[576])) && (!s.b[580])) && s.b[585]) && s.b[586]) {
            s.store_mul_ad(214, A::powf(A::sub_from_scalar(p.p44, s.ad_value(250)), p.p41), A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(159), A::offset(s.ad_value(159), p.p48))), p.p49));
        }

        s.b[587] = (p.p7 == 0.0);
        s.v[587] = if s.b[587] { 1.0 } else { 0.0 };

        if (((((s.b[575] && (!s.b[576])) && (!s.b[580])) && s.b[585]) && s.b[586]) && s.b[587]) {
            s.copy_ad(215, 214);
        }

        if (((((s.b[575] && (!s.b[576])) && (!s.b[580])) && s.b[585]) && s.b[586]) && (!s.b[587])) {
            s.store_scaled_offset(216, 159, (-p.p52), 1.0 / (p.p48));
            s.store_scaled_offset(285, 216, (-1.0), 1.0 / (p.p51));
        }

        s.b[588] = (s.v[216] < 1.0);
        s.v[588] = if s.b[588] { 1.0 } else { 0.0 };

        if ((((((s.b[575] && (!s.b[576])) && (!s.b[580])) && s.b[585]) && s.b[586]) && (!s.b[587])) && s.b[588]) {
            s.store_offset_scaled_ad(217, A::ln_one_plus_exp(s.ad_value(285)), p.p51, 1.0);
        }

        if ((((((s.b[575] && (!s.b[576])) && (!s.b[580])) && s.b[585]) && s.b[586]) && (!s.b[587])) && (!s.b[588])) {
            s.store_ad_value(217, A::add_scaled_inputs(s.ad_value(216), 1.0, A::ln_one_plus_exp(A::neg(s.ad_value(285))), p.p51));
        }

        if (((((s.b[575] && (!s.b[576])) && (!s.b[580])) && s.b[585]) && s.b[586]) && (!s.b[587])) {
            s.store_mul_powf_ad_rhs(215, 214, s.ad_value(217), p.p50);
        }

        s.b[589] = (((-s.v[340]) * s.v[215]) < p.p151);
        s.v[589] = if s.b[589] { 1.0 } else { 0.0 };

        if (((((s.b[575] && (!s.b[576])) && (!s.b[580])) && s.b[585]) && s.b[586]) && s.b[589]) {
            s.store_exp_ad(343, A::mul_scaled_lhs(s.ad_value(340), -1.0, s.ad_value(215)));
        }

        if (((((s.b[575] && (!s.b[576])) && (!s.b[580])) && s.b[585]) && s.b[586]) && (!s.b[589])) {
            s.store_scalar(301, ((p.p151) as f64).exp());
            s.store_mul_offset_ad_rhs(343, 301, A::mul_scaled_lhs(s.ad_value(340), -1.0, s.ad_value(215)), (((-p.p151)) + (1.0)));
        }

        if ((((s.b[575] && (!s.b[576])) && (!s.b[580])) && s.b[585]) && s.b[586]) {
            s.store_mul_ad_lhs(210, A::mul_sub_from_scalar_rhs(A::div_from_scalar(p.p40, s.ad_value(340)), p.p44, s.ad_value(250)), 343);
        }

        s.b[590] = (s.v[210] > 0.0);
        s.v[590] = if s.b[590] { 1.0 } else { 0.0 };

        s.b[591] = (p.p53 == 1.0);
        s.v[591] = if s.b[591] { 1.0 } else { 0.0 };

        if ((s.b[575] && s.b[590]) && s.b[591]) {
            s.store_add_ad(211, A::add_scaled_product(A::div(s.ad_value(6), A::mul(s.ad_value(159), A::add(s.ad_value(30), s.ad_value(189)))), 1.0, A::div(s.ad_value(156), s.ad_value(35)), s.ad_value(42), 1.0), A::div(s.ad_value(28), A::add(s.ad_value(30), s.ad_value(189))));
        }

        s.b[592] = (p.p39 == 3.0);
        s.v[592] = if s.b[592] { 1.0 } else { 0.0 };

        if (((s.b[575] && s.b[590]) && s.b[591]) && s.b[592]) {
            s.store_scaled_sub(285, 210, 211, 1000000.0);
        }

        s.b[593] = (s.v[210] < s.v[211]);
        s.v[593] = if s.b[593] { 1.0 } else { 0.0 };

        if ((((s.b[575] && s.b[590]) && s.b[591]) && s.b[592]) && s.b[593]) {
            s.store_ad_value(210, A::sub_scaled_inputs(s.ad_value(210), 1.0, A::ln_one_plus_exp(s.ad_value(285)), 1e-6));
        }

        if ((((s.b[575] && s.b[590]) && s.b[591]) && s.b[592]) && (!s.b[593])) {
            s.store_ad_value(210, A::sub_scaled_inputs(s.ad_value(211), 1.0, A::ln_one_plus_exp(A::neg(s.ad_value(285))), 1e-6));
        }

        if (((s.b[575] && s.b[590]) && s.b[591]) && s.b[592]) {
            s.store_mul(212, 159, 210);
        }

        if (((s.b[575] && s.b[590]) && s.b[591]) && (!s.b[592])) {
            s.store_ad_value(212, A::div_scaled_product3(s.ad_value(159), s.ad_value(210), s.ad_value(211), 1.0, A::add(s.ad_value(210), s.ad_value(211)), 1.0));
        }

        if ((s.b[575] && s.b[590]) && (!s.b[591])) {
            s.store_mul(212, 159, 210);
        }

        s.b[594] = (s.v[131] > 0.0);
        s.v[594] = if s.b[594] { 1.0 } else { 0.0 };

        if s.b[594] {
            s.store_mul_ln_rhs(130, 6, 131);
        }

        if (!s.b[594]) {
            s.copy_ad(130, 251);
        }

        s.b[595] = (p.p24 == 1.0);
        s.v[595] = if s.b[595] { 1.0 } else { 0.0 };

        if s.b[595] {
            s.copy_ad(93, 250);
        }

        if (!s.b[595]) {
            s.copy_ad(93, 251);
        }

        let assign6110_ad_e6278: A = A::add_scaled_inputs_products(A::add_scaled_products3(s.ad_value(159), A::sub(s.ad_value(252), s.ad_value(130)), 1.0, s.ad_value(118), A::sub(s.ad_value(130), s.ad_value(250)), 1.0, s.ad_value(212), s.ad_value(130), (-1.0)), 1.0, A::div_scaled_product(s.ad_value(265), s.ad_value(265), 1.0, s.ad_value(28), 1.0), 1.0, A::square(s.ad_value(268)), s.ad_value(111), 1.0, A::square(s.ad_value(257)), s.ad_value(112), 1.0);
        let assign6110_ad_e6310: A = A::add_scaled_inputs_products(A::add_scaled_product(assign6110_ad_e6278, 1.0, A::square(s.ad_value(258)), s.ad_value(113), 1.0), 1.0, A::div_scaled_product(s.ad_value(266), s.ad_value(266), 1.0, s.ad_value(30), 1.0), 1.0, s.ad_value(191), s.ad_value(254), 1.0, A::add(A::add_scaled_inputs3(A::add_scaled_inputs3(s.ad_value(161), 1.0, s.ad_value(163), 1.0, s.ad_value(252), s.v[344]), 1.0, s.ad_value(57), (-1.0), s.ad_value(359), 1.0), s.ad_value(358)), s.ad_value(252), 1.0);
        let assign6110_ad_e6342: A = A::add_scaled_value_products(A::add_scaled_value_products3(assign6110_ad_e6310, 1.0, s.ad_value(82), s.ad_value(93), (-1.0), A::add_scaled_inputs3(s.ad_value(162), 1.0, s.ad_value(165), 1.0, s.ad_value(166), 1.0), s.ad_value(253), 1.0, A::add_scaled_inputs3(s.ad_value(167), 1.0, s.ad_value(164), 1.0, s.ad_value(255), s.v[344]), s.ad_value(255), 1.0), 1.0, s.ad_value(179), s.ad_value(267), 1.0, s.ad_value(182), A::sub(s.ad_value(255), s.ad_value(261)), 1.0);
        s.store_ad_value(219, A::add_scaled_value_products3(assign6110_ad_e6342, 1.0, s.ad_value(185), A::sub(s.ad_value(250), s.ad_value(259)), 1.0, s.ad_value(183), A::sub(s.ad_value(267), s.ad_value(260)), 1.0, s.ad_value(184), s.ad_value(259), 1.0));

        s.store_scaled_mul(221, 23, 141, (1.0 - p.p68));

        s.store_div_ad_lhs(285, A::sub(s.ad_value(253), s.ad_value(139)), 299);

        s.b[596] = (s.v[253] < s.v[139]);
        s.v[596] = if s.b[596] { 1.0 } else { 0.0 };

        if s.b[596] {
            s.store_ad_value(222, A::add_scaled_product(s.ad_value(253), 1.0, s.ad_value(299), A::ln_one_plus_exp(s.ad_value(285)), (-1.0)));
        }

        if (!s.b[596]) {
            s.store_ad_value(222, A::add_scaled_product(s.ad_value(139), 1.0, s.ad_value(299), A::ln_one_plus_exp(A::neg(s.ad_value(285))), (-1.0)));
        }

        s.store_mul_scaled_ad_rhs(223, 23, p.p68, A::add_scaled_inputs3(A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(14), 1.0, A::powf(A::sub_from_scalar(1.0, A::mul(s.ad_value(222), s.ad_value(65))), (1.0 - p.p67)), 1.0 / ((1.0 - p.p67))), 1.0, s.ad_value(253), 3.0, s.ad_value(222), (-3.0)));

        s.store_scaled_mul(224, 24, 148, p.p77);

        s.store_mul(225, 95, 36);

        s.store_mul3_affine_lhs(229, 225, 152, 0.5, 0.0, 187);

        s.store_mul3_affine_lhs(230, 225, 153, 0.5, 0.0, 187);

        s.store_scale(300, 17, 0.1);

        s.store_div_ad_lhs(285, A::sub(s.ad_value(255), s.ad_value(144)), 300);

        s.b[597] = (s.v[255] < s.v[144]);
        s.v[597] = if s.b[597] { 1.0 } else { 0.0 };

        if s.b[597] {
            s.store_ad_value(231, A::add_scaled_product(s.ad_value(255), 1.0, s.ad_value(300), A::ln_one_plus_exp(s.ad_value(285)), (-1.0)));
        }

        if (!s.b[597]) {
            s.store_ad_value(231, A::add_scaled_product(s.ad_value(144), 1.0, s.ad_value(300), A::ln_one_plus_exp(A::neg(s.ad_value(285))), (-1.0)));
        }

        s.store_ad_value(232, A::add_scaled_product(A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(17), 1.0, A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(231), s.ad_value(17))), (1.0 - p.p72)), 1.0 / ((1.0 - p.p72))), 1.0, s.ad_value(143), A::sub(s.ad_value(255), s.ad_value(231)), 1.0));

        s.store_mul_scaled_ad_rhs(233, 24, ((1.0 - p.p77) * (1.0 - p.p33)), A::add_scaled_product(A::mul_sub_from_scalar_lhs(1.0, s.ad_value(25), s.ad_value(232)), 1.0, s.ad_value(25), s.ad_value(255), 1.0));

        s.store_div_ad_lhs(285, A::sub(s.ad_value(267), s.ad_value(144)), 300);

        s.b[598] = (s.v[267] < s.v[144]);
        s.v[598] = if s.b[598] { 1.0 } else { 0.0 };

        if s.b[598] {
            s.store_ad_value(234, A::add_scaled_product(s.ad_value(267), 1.0, s.ad_value(300), A::ln_one_plus_exp(s.ad_value(285)), (-1.0)));
        }

        if (!s.b[598]) {
            s.store_ad_value(234, A::add_scaled_product(s.ad_value(144), 1.0, s.ad_value(300), A::ln_one_plus_exp(A::neg(s.ad_value(285))), (-1.0)));
        }

        s.store_ad_value(235, A::add_scaled_product(A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(17), 1.0, A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(234), s.ad_value(17))), (1.0 - p.p72)), 1.0 / ((1.0 - p.p72))), 1.0, s.ad_value(143), A::sub(s.ad_value(267), s.ad_value(234)), 1.0));

        s.store_mul_scaled_ad_rhs(236, 24, ((1.0 - p.p77) * p.p33), A::add_scaled_product(A::mul_sub_from_scalar_lhs(1.0, s.ad_value(25), s.ad_value(235)), 1.0, s.ad_value(25), s.ad_value(267), 1.0));

        s.store_scale(307, 105, 0.1);

        s.store_scale(237, 105, (1.0 - ((2.0) as f64).powf(((-1.0) / p.p139))));

        s.store_div_ad_lhs(285, A::sub(s.ad_value(259), s.ad_value(237)), 307);

        s.b[599] = (s.v[259] < s.v[237]);
        s.v[599] = if s.b[599] { 1.0 } else { 0.0 };

        if s.b[599] {
            s.store_ad_value(238, A::add_scaled_product(s.ad_value(259), 1.0, s.ad_value(307), A::ln_one_plus_exp(s.ad_value(285)), (-1.0)));
        }

        if (!s.b[599]) {
            s.store_ad_value(238, A::add_scaled_product(s.ad_value(237), 1.0, s.ad_value(307), A::ln_one_plus_exp(A::neg(s.ad_value(285))), (-1.0)));
        }

        s.store_mul_ad_rhs(239, 106, A::add_scaled_inputs3(A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(105), 1.0, A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(238), s.ad_value(105))), (1.0 - p.p139)), 1.0 / ((1.0 - p.p139))), 1.0, s.ad_value(259), 2.0, s.ad_value(238), (-2.0)));

        s.store_mul_ad_product_rhs(240, 94, s.ad_value(36), A::powf(A::div(s.ad_value(35), s.ad_value(36)), (1.0 / p.p85)));

        s.b[600] = ((s.v[252] / (p.p85 * s.v[6])) < p.p151);
        s.v[600] = if s.b[600] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_block_4(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        ddt_active: bool,
        ddt_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
    ) {
        if s.b[600] {
            s.store_exp_ad(302, A::div_scaled_inputs(s.ad_value(252), 1.0, s.ad_value(6), p.p85));
        }

        if (!s.b[600]) {
            s.store_scalar(301, ((p.p151) as f64).exp());
            s.store_mul_offset_ad_rhs(302, 301, A::div_scaled_inputs(s.ad_value(252), 1.0, s.ad_value(6), p.p85), (((-p.p151)) + (1.0)));
        }

        s.store_mul(242, 240, 302);

        s.store_ad_value(243, A::div_scaled_product(s.ad_value(96), s.ad_value(6), 4.0, s.ad_value(31), 1.0));

        s.store_mul_ad_affine_product_rhs(244, 243, s.ad_value(125), A::offset(A::add(s.ad_value(129), s.ad_value(116)), 2.0), 0.5, 0.0);

        s.b[601] = (p.p79 == 0.0);
        s.v[601] = if s.b[601] { 1.0 } else { 0.0 };

        if s.b[601] {
            s.store_ad_value(249, A::div_scaled_product(s.ad_value(97), A::add_scaled_products(s.ad_value(225), s.ad_value(171), 1.0, s.ad_value(243), s.ad_value(170), 1.0), 0.5, A::add(s.ad_value(95), s.ad_value(96)), 1.0));
        }

        s.b[602] = ((((s.v[255] - s.v[22]) / p.p91) * s.v[8]) < p.p151);
        s.v[602] = if s.b[602] { 1.0 } else { 0.0 };

        if ((!s.b[601]) && s.b[602]) {
            s.store_exp_ad(180, A::mul_scaled_lhs(A::sub(s.ad_value(255), s.ad_value(22)), 1.0 / (p.p91), s.ad_value(8)));
        }

        if ((!s.b[601]) && (!s.b[602])) {
            s.store_scalar(301, ((p.p151) as f64).exp());
            s.store_mul_offset_ad_rhs(180, 301, A::mul_scaled_lhs(A::sub(s.ad_value(255), s.ad_value(22)), 1.0 / (p.p91), s.ad_value(8)), (((-p.p151)) + (1.0)));
        }

        if (!s.b[601]) {
            s.store_ad_value(249, A::div_scaled_product3(s.ad_value(43), s.ad_value(98), s.ad_value(274), 2.0, A::offset(A::sqrt(A::scale_offset(s.ad_value(180), 4.0, 1.0)), 1.0), 1.0));
        }

        s.b[603] = (((p.p5 == 1.0) || (p.p5 == 3.0)) && (p.p33 > 0.0));
        s.v[603] = if s.b[603] { 1.0 } else { 0.0 };

        if s.b[603] {
            s.store_scale(249, 249, s.v[160]);
        }

        s.b[604] = (p.p79 == 0.0);
        s.v[604] = if s.b[604] { 1.0 } else { 0.0 };

        if (s.b[603] && s.b[604]) {
            s.store_mul(172, 149, 275);
            s.store_div_ad(173, A::sub(s.ad_value(172), s.ad_value(149)), A::offset(A::sqrt(A::offset(s.ad_value(172), 1.0)), 1.0));
            s.store_scale(245, 278, 4.0);
            s.store_div_ad_rhs(246, 245, A::offset(A::sqrt(A::offset(s.ad_value(245), 1.0)), 1.0));
            s.store_ad_value(247, A::div_scaled_product(s.ad_value(97), A::add_scaled_products(s.ad_value(225), s.ad_value(173), 1.0, s.ad_value(243), s.ad_value(246), 1.0), (0.5 * p.p33), A::add(s.ad_value(95), s.ad_value(96)), 1.0));
        }

        s.b[605] = (((s.v[267] - s.v[22]) * s.v[8]) < p.p151);
        s.v[605] = if s.b[605] { 1.0 } else { 0.0 };

        if ((s.b[603] && (!s.b[604])) && s.b[605]) {
            s.store_exp_ad(181, A::mul(A::sub(s.ad_value(267), s.ad_value(22)), s.ad_value(8)));
        }

        if ((s.b[603] && (!s.b[604])) && (!s.b[605])) {
            s.store_scalar(301, ((p.p151) as f64).exp());
            s.store_mul_offset_ad_rhs(181, 301, A::mul(A::sub(s.ad_value(267), s.ad_value(22)), s.ad_value(8)), (((-p.p151)) + (1.0)));
        }

        if (s.b[603] && (!s.b[604])) {
            s.store_ad_value(247, A::div_scaled_product3(s.ad_value(43), s.ad_value(98), s.ad_value(275), (2.0 * p.p33), A::offset(A::sqrt(A::scale_offset(s.ad_value(181), 4.0, 1.0)), 1.0), 1.0));
        }

        if s.b[603] {
            s.store_mul(248, 178, 247);
        }

        s.b[606] = (p.p6 == 1.0);
        s.v[606] = if s.b[606] { 1.0 } else { 0.0 };

        if s.b[606] {
            s.store_offset_powf_ad(193, A::sub_from_scalar(1.0, A::mul(s.ad_value(140), s.ad_value(65))), (-p.p67), (-3.0));
            s.store_div_ad_lhs(294, A::sub(s.ad_value(252), s.ad_value(139)), 299);
        }

        s.b[607] = (s.v[294] < 0.0);
        s.v[607] = if s.b[607] { 1.0 } else { 0.0 };

        if (s.b[606] && s.b[607]) {
            s.store_div_from_scalar_offset_ad(194, 1.0, A::exp(s.ad_value(294)), 1.0);
        }

        if (s.b[606] && (!s.b[607])) {
            s.store_div_ad(194, A::exp_scaled_input(s.ad_value(294), -1.0), A::offset(A::exp_scaled_input(s.ad_value(294), -1.0), 1.0));
        }

        if s.b[606] {
            s.store_offset_mul(192, 193, 194, 3.0);
            s.store_scaled_mul(195, 23, 192, (1.0 - p.p68));
            s.store_mul_ad(198, A::div_scaled_product3(s.ad_value(149), s.ad_value(272), s.ad_value(8), 1.0, s.ad_value(48), 1.0), A::div_from_scalar(0.5, A::sqrt(A::offset(s.ad_value(150), 1.0))));
            s.store_mul3_affine_lhs(196, 225, 187, 0.5, 0.0, 198);
            s.store_scaled_div(197, 242, 6, (1.0 / (p.p85)));
            s.store_mul_scaled_ad_rhs(228, 254, 0.2, A::add_scaled_inputs3(s.ad_value(195), 1.0, s.ad_value(196), 1.0, s.ad_value(197), 1.0));
            s.store_scale(241, 242, (1.0 - p.p95));
            s.store_add_scaled_inputs(337, 229, 1.0, 242, p.p95);
            s.store_add_scaled_inputs(227, 337, p.p94, 230, 1.0);
            s.store_scale(226, 337, (1.0 - p.p94));
        }

        if (!s.b[606]) {
            s.copy_ad(226, 229);
            s.copy_ad(227, 230);
            s.copy_ad(241, 242);
        }

        s.b[608] = (p.p24 == 1.0);
        s.v[608] = if s.b[608] { 1.0 } else { 0.0 };

        let assign6910_ad_e7175: A = A::ddt(A::scale(A::voltage(ctx, nodes, Some(4), None), p.p147), ddt_scale, eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 0, A::scale(A::voltage(ctx, nodes, Some(4), None), p.p147).value));
        s.store_scale_ad(220, assign6910_ad_e7175, p.p1);

        s.v[356] = (1.0 - p.p148);

        s.b[609] = (p.p146 > s.v[346]);
        s.v[609] = if s.b[609] { 1.0 } else { 0.0 };

        s.b[610] = (p.p145 == 0.0);
        s.v[610] = if s.b[610] { 1.0 } else { 0.0 };

        if (s.b[609] && s.b[610]) {
            s.store_scaled_voltage(102, ctx, nodes, Some(4), None, (1.0 / (s.v[103]) * p.p1));
        }

        s.b[611] = (((s.v[356]) as f64).abs() < 1e-6);
        s.v[611] = if s.b[611] { 1.0 } else { 0.0 };

        if ((s.b[609] && (!s.b[610])) && s.b[611]) {
            s.store_scaled_ln_ad(102, A::scale_offset(A::voltage(ctx, nodes, Some(4), None), 1.0 / (s.v[5]), 1.0), ((s.v[5] / s.v[103]) * p.p1));
        }

        if ((s.b[609] && (!s.b[610])) && (!s.b[611])) {
            s.store_scaled_offset_ad(102, A::powf(A::scale_offset(A::voltage(ctx, nodes, Some(4), None), 1.0 / (s.v[5]), 1.0), s.v[356]), (-1.0), ((s.v[5] / (s.v[356] * s.v[103])) * p.p1));
        }

        if (!s.b[609]) {
            s.store_div_voltage_by_ad(102, ctx, nodes, Some(4), None, s.ad_value(345));
        }

        s.b[612] = (p.p58 > 0.0);
        s.v[612] = if s.b[612] { 1.0 } else { 0.0 };

        s.b[613] = (p.p59 > 0.0);
        s.v[613] = if s.b[613] { 1.0 } else { 0.0 };

        s.store_scale(308, 2, (4.0 * 1.3806226e-23));

        s.store_div(309, 308, 28);

        s.store_div(310, 308, 30);

        s.store_mul(311, 308, 111);

        s.store_mul(312, 308, 112);

        s.store_mul(313, 308, 113);

        s.store_scaled_mul_ad(314, A::div(s.ad_value(308), s.ad_value(189)), A::scale_offset(s.ad_value(273), 4.0, 5.0), 0.3333333333333333);

        s.store_div_ad_lhs(333, A::add(s.ad_value(158), s.ad_value(157)), 156);

        s.store_scaled_abs(315, 333, (2.0 * 1.6021918e-19));

        s.b[614] = (p.p130 > 0.0);
        s.v[614] = if s.b[614] { 1.0 } else { 0.0 };

        if s.b[614] {
            s.store_abs_ad(334, A::div(s.ad_value(212), s.ad_value(333)));
        }

        if (!s.b[614]) {
            s.store_scalar(334, 0.0);
        }

        s.store_mul_scaled_ad_rhs(327, 212, (2.0 * 1.6021918e-19), A::offset(s.ad_value(334), 1.0));

        s.b[615] = (s.v[333] > 0.0);
        s.v[615] = if s.b[615] { 1.0 } else { 0.0 };

        if s.b[615] {
            s.store_div_ad_lhs(335, A::add(s.ad_value(226), s.ad_value(227)), 333);
        }

        if (!s.b[615]) {
            s.store_mul3_lhs(335, 95, 187, 156);
        }

        s.b[616] = (p.p131 == 1.0);
        s.v[616] = if s.b[616] { 1.0 } else { 0.0 };

        if s.b[616] {
            s.store_scale(336, 335, p.p94);
        }

        s.b[617] = (p.p131 == 2.0);
        s.v[617] = if s.b[617] { 1.0 } else { 0.0 };

        if ((!s.b[616]) && s.b[617]) {
            s.store_scale(336, 335, p.p132);
        }

        if ((!s.b[616]) && (!s.b[617])) {
            s.store_scalar(336, 0.0);
        }

        s.store_scaled_abs_ad(316, A::add_scaled_inputs3(A::add_scaled_inputs3(s.ad_value(161), 1.0, s.ad_value(163), 1.0, s.ad_value(57), -1.0), 1.0, s.ad_value(359), 1.0, s.ad_value(358), 1.0), (2.0 * 1.6021918e-19));

        s.store_add(328, 161, 162);

        s.store_scaled_powf_ad(317, A::abs(s.ad_value(328)), p.p126, p.p128);

        s.b[618] = (s.v[328] < 0.0);
        s.v[618] = if s.b[618] { 1.0 } else { 0.0 };

        if s.b[618] {
            s.store_neg(317, 317);
        }

        s.store_ad_value(329, A::add_scaled_inputs3(s.ad_value(163), 1.0, s.ad_value(165), 1.0, s.ad_value(166), 1.0));

        s.store_scaled_powf_ad(318, A::abs(s.ad_value(329)), p.p127, p.p129);

        s.b[619] = (s.v[329] < 0.0);
        s.v[619] = if s.b[619] { 1.0 } else { 0.0 };

        if s.b[619] {
            s.store_neg(318, 318);
        }

        s.store_scaled_abs_ad(319, A::add_scaled_inputs3(s.ad_value(162), 1.0, s.ad_value(165), 1.0, s.ad_value(166), 1.0), (2.0 * 1.6021918e-19));

        s.store_scaled_abs(320, 164, (2.0 * 1.6021918e-19));

        s.store_scaled_powf_ad(321, A::abs(s.ad_value(164)), p.p126, p.p128);

        s.b[620] = (s.v[164] < 0.0);
        s.v[620] = if s.b[620] { 1.0 } else { 0.0 };

        if s.b[620] {
            s.store_neg(321, 321);
        }

        s.store_scaled_abs(322, 82, (2.0 * 1.6021918e-19));

        s.store_scaled_abs(323, 167, (2.0 * 1.6021918e-19));

        s.store_scaled_powf_ad(325, A::scale(A::abs(s.ad_value(167)), 1.0 / ((1.0 - (p.p5 * p.p33)))), p.p126, (p.p128 * (1.0 - (p.p5 * p.p33))));

        s.b[621] = (s.v[167] < 0.0);
        s.v[621] = if s.b[621] { 1.0 } else { 0.0 };

        if s.b[621] {
            s.store_neg(325, 325);
        }

        s.store_scaled_abs(324, 179, ((2.0 * 1.6021918e-19) * p.p5));

        s.b[622] = (p.p33 == 0.0);
        s.v[622] = if s.b[622] { 1.0 } else { 0.0 };

        if s.b[622] {
            s.store_scalar(326, 0.0);
        }

        if (!s.b[622]) {
            s.store_scaled_powf_ad(326, A::scale(A::abs(s.ad_value(179)), 1.0 / (p.p33)), p.p126, ((p.p128 * p.p5) * p.p33));
        }

        s.b[623] = (s.v[179] < 0.0);
        s.v[623] = if s.b[623] { 1.0 } else { 0.0 };

        if s.b[623] {
            s.store_neg(326, 326);
        }

        s.store_scaled_abs(330, 185, (2.0 * 1.6021918e-19));

        s.store_scaled_abs(331, 182, (2.0 * 1.6021918e-19));

        s.store_scaled_abs(332, 183, (2.0 * 1.6021918e-19));

        s.b[624] = (p.p24 == 1.0);
        s.v[624] = if s.b[624] { 1.0 } else { 0.0 };

        s.b[625] = (p.p58 > 0.0);
        s.v[625] = if s.b[625] { 1.0 } else { 0.0 };

        s.b[626] = (p.p59 > 0.0);
        s.v[626] = if s.b[626] { 1.0 } else { 0.0 };

        s.b[627] = (p.p59 > 0.0);
        s.v[627] = if s.b[627] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_reactive_block_0(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        let ctx_temp = ctx.temperature();
        s.b[484] = (p.p3 == 1.0);
        s.v[484] = if s.b[484] { 1.0 } else { 0.0 };

        if s.b[484] {
            s.store_scalar(0, 70300000.0);
            s.store_scalar(1, 123000000.0);
        }

        if (!s.b[484]) {
            s.store_scalar(0, 158000000.0);
            s.store_scalar(1, 204000000.0);
        }

        s.v[160] = (1.0 - p.p33);

        s.v[3] = (p.p4 + 273.15);

        s.v[5] = (ctx_temp + p.p0);

        s.b[485] = (p.p154 == 0.0);
        s.v[485] = if s.b[485] { 1.0 } else { 0.0 };

        if s.b[485] {
            s.store_scalar(345, 1e-12);
        }

        if (!s.b[485]) {
            s.store_scalar(345, p.p154);
        }

        s.store_scale(346, 345, p.p1);

        s.v[52] = 0.001;

        s.v[342] = 0.001;

        s.v[62] = ((2.0) as f64).powf((2.0 - p.p67));

        s.v[285] = (((p.p114 + (((p.p115 * s.v[3]) * s.v[3]) / (s.v[3] + p.p116))) - 0.05) / 0.1);

        s.b[487] = ((p.p114 + (((p.p115 * s.v[3]) * s.v[3]) / (s.v[3] + p.p116))) < 0.05);
        s.v[487] = if s.b[487] { 1.0 } else { 0.0 };

        if s.b[487] {
            s.store_scalar(74, (0.05 + (0.1 * (((1.0 + ((s.v[285]) as f64).exp())) as f64).ln())));
        }

        if (!s.b[487]) {
            s.store_scalar(74, ((p.p114 + (((p.p115 * s.v[3]) * s.v[3]) / (s.v[3] + p.p116))) + (0.1 * (((1.0 + (((-s.v[285])) as f64).exp())) as f64).ln())));
        }

        s.v[71] = p.p114;

        s.v[72] = (1.0 / s.v[71]);

        s.v[75] = p.p71;

        s.v[76] = p.p72;

        s.v[79] = ((2.0) as f64).powf((2.0 - s.v[76]));

        s.v[285] = (((p.p117 + (((p.p118 * s.v[3]) * s.v[3]) / (s.v[3] + p.p119))) - 0.05) / 0.1);

        s.b[488] = ((p.p117 + (((p.p118 * s.v[3]) * s.v[3]) / (s.v[3] + p.p119))) < 0.05);
        s.v[488] = if s.b[488] { 1.0 } else { 0.0 };

        if s.b[488] {
            s.store_scalar(88, (0.05 + (0.1 * (((1.0 + ((s.v[285]) as f64).exp())) as f64).ln())));
        }

        if (!s.b[488]) {
            s.store_scalar(88, ((p.p117 + (((p.p118 * s.v[3]) * s.v[3]) / (s.v[3] + p.p119))) + (0.1 * (((1.0 + (((-s.v[285])) as f64).exp())) as f64).ln())));
        }

        s.v[87] = p.p117;

        s.v[86] = (1.0 / s.v[87]);

        s.v[178] = 1.0;

        s.v[210] = 0.0;

        s.v[248] = 0.0;

        s.v[228] = 0.0;

        s.v[42] = 0.0;

        s.store_voltage(218, ctx, nodes, Some(4), None);

        s.b[489] = (s.v[218] < 0.0);
        s.v[489] = if s.b[489] { 1.0 } else { 0.0 };

        if s.b[489] {
            s.store_neg_ad(218, A::ln(A::sub_from_scalar(1.0, s.ad_value(218))));
        }

        s.b[490] = (s.v[218] < p.p125);
        s.v[490] = if s.b[490] { 1.0 } else { 0.0 };

        if s.b[490] {
            s.copy_ad(11, 218);
        }

        if (!s.b[490]) {
            s.store_offset_ln_ad(11, A::offset(s.ad_value(218), (((-p.p125)) + (1.0))), p.p125);
        }

        s.store_offset(2, 11, s.v[5]);

        s.store_scale(4, 2, 1.0 / (s.v[3]));

        s.store_scale(6, 2, 8.617086918058125e-5);

        s.v[7] = (8.617086918058125e-5 * s.v[3]);

        s.store_div_from_scalar(8, 1.0, 6);

        s.v[9] = (1.0 / s.v[7]);

        s.store_offset(10, 8, (-s.v[9]));

        s.store_offset(12, 2, (-s.v[3]));

        s.store_ln(280, 4);

        s.store_scaled_offset_ad(285, A::sub(s.ad_value(74), A::div_scaled_product_offset_denominator(s.ad_value(2), s.ad_value(2), p.p115, s.ad_value(2), p.p116, 1.0)), (-0.05), 10.0);

        s.b[491] = ((s.v[74] - (((p.p115 * s.v[2]) * s.v[2]) / (s.v[2] + p.p116))) < 0.05);
        s.v[491] = if s.b[491] { 1.0 } else { 0.0 };

        if s.b[491] {
            s.store_offset_scaled_ad(70, A::ln_one_plus_exp(s.ad_value(285)), 0.1, 0.05);
        }

        if (!s.b[491]) {
            s.store_ad_value(70, A::add_scaled_inputs3(s.ad_value(74), 1.0, A::div_scaled_product_offset_denominator(s.ad_value(2), s.ad_value(2), p.p115, s.ad_value(2), p.p116, 1.0), (-1.0), A::ln_one_plus_exp(A::neg(s.ad_value(285))), 0.1));
        }

        s.store_scaled_offset_ad(285, A::sub(s.ad_value(88), A::div_scaled_product_offset_denominator(s.ad_value(2), s.ad_value(2), p.p118, s.ad_value(2), p.p119, 1.0)), (-0.05), 10.0);

        s.b[492] = ((s.v[88] - (((p.p118 * s.v[2]) * s.v[2]) / (s.v[2] + p.p119))) < 0.05);
        s.v[492] = if s.b[492] { 1.0 } else { 0.0 };

        if s.b[492] {
            s.store_offset_scaled_ad(85, A::ln_one_plus_exp(s.ad_value(285)), 0.1, 0.05);
        }

        if (!s.b[492]) {
            s.store_ad_value(85, A::add_scaled_inputs3(s.ad_value(88), 1.0, A::div_scaled_product_offset_denominator(s.ad_value(2), s.ad_value(2), p.p118, s.ad_value(2), p.p119, 1.0), (-1.0), A::ln_one_plus_exp(A::neg(s.ad_value(285))), 0.1));
        }

        s.store_add_ad(13, A::add_scaled_product(s.ad_value(4), p.p66, s.ad_value(6), s.ad_value(280), (-3.0)), A::scale_offset(s.ad_value(4), (-p.p105), p.p105));

        s.store_div_ad_lhs(285, A::sub_from_scalar(0.05, s.ad_value(13)), 6);

        s.b[493] = (0.05 < s.v[13]);
        s.v[493] = if s.b[493] { 1.0 } else { 0.0 };

        if s.b[493] {
            s.store_ad_value(14, A::add_scaled_product(s.ad_value(13), 1.0, s.ad_value(6), A::ln_one_plus_exp(s.ad_value(285)), 1.0));
        }

        if (!s.b[493]) {
            s.store_offset_mul_ad(14, s.ad_value(6), A::ln_one_plus_exp(A::neg(s.ad_value(285))), 0.05);
        }

        s.store_add_ad(15, A::add_scaled_product(s.ad_value(4), p.p64, s.ad_value(6), s.ad_value(280), (-3.0)), A::scale_offset(s.ad_value(4), (-p.p110), p.p110));

        s.store_div_ad_lhs(285, A::sub_from_scalar(0.05, s.ad_value(15)), 6);

        s.b[494] = (0.05 < s.v[15]);
        s.v[494] = if s.b[494] { 1.0 } else { 0.0 };

        if s.b[494] {
            s.store_ad_value(16, A::add_scaled_product(s.ad_value(15), 1.0, s.ad_value(6), A::ln_one_plus_exp(s.ad_value(285)), 1.0));
        }

        if (!s.b[494]) {
            s.store_offset_mul_ad(16, s.ad_value(6), A::ln_one_plus_exp(A::neg(s.ad_value(285))), 0.05);
        }

        s.store_add_ad(21, A::add_scaled_product(s.ad_value(4), p.p80, s.ad_value(6), s.ad_value(280), (-3.0)), A::scale_offset(s.ad_value(4), (-p.p110), p.p110));

        s.store_div_ad_lhs(285, A::sub_from_scalar(0.05, s.ad_value(21)), 6);

        s.b[495] = (0.05 < s.v[21]);
        s.v[495] = if s.b[495] { 1.0 } else { 0.0 };

        if s.b[495] {
            s.store_ad_value(22, A::add_scaled_product(s.ad_value(21), 1.0, s.ad_value(6), A::ln_one_plus_exp(s.ad_value(285)), 1.0));
        }

        if (!s.b[495]) {
            s.store_offset_mul_ad(22, s.ad_value(6), A::ln_one_plus_exp(A::neg(s.ad_value(285))), 0.05);
        }

        s.store_add_ad(18, A::add_scaled_product(s.ad_value(4), p.p71, s.ad_value(6), s.ad_value(280), (-3.0)), A::scale_offset(s.ad_value(4), (-p.p110), p.p110));

        s.store_div_ad_lhs(285, A::sub_from_scalar(0.05, s.ad_value(18)), 6);

        s.b[496] = (0.05 < s.v[18]);
        s.v[496] = if s.b[496] { 1.0 } else { 0.0 };

        if s.b[496] {
            s.store_ad_value(17, A::add_scaled_product(s.ad_value(18), 1.0, s.ad_value(6), A::ln_one_plus_exp(s.ad_value(285)), 1.0));
        }

        if (!s.b[496]) {
            s.store_offset_mul_ad(17, s.ad_value(6), A::ln_one_plus_exp(A::neg(s.ad_value(285))), 0.05);
        }

        s.store_add_ad(20, A::add_scaled_product(s.ad_value(4), s.v[75], s.ad_value(6), s.ad_value(280), (-3.0)), A::scale_offset(s.ad_value(4), (-p.p110), p.p110));

        s.store_div_ad_lhs(285, A::sub_from_scalar(0.05, s.ad_value(20)), 6);

        s.b[497] = (0.05 < s.v[20]);
        s.v[497] = if s.b[497] { 1.0 } else { 0.0 };

        if s.b[497] {
            s.store_ad_value(19, A::add_scaled_product(s.ad_value(20), 1.0, s.ad_value(6), A::ln_one_plus_exp(s.ad_value(285)), 1.0));
        }

        if (!s.b[497]) {
            s.store_offset_mul_ad(19, s.ad_value(6), A::ln_one_plus_exp(A::neg(s.ad_value(285))), 0.05);
        }

        s.store_add_ad(56, A::add_scaled_product(s.ad_value(4), p.p27, s.ad_value(6), s.ad_value(280), (-3.0)), A::scale_offset(s.ad_value(4), (-p.p109), p.p109));

        s.store_div_ad_lhs(285, A::sub_from_scalar(0.05, s.ad_value(56)), 6);

        s.b[498] = (0.05 < s.v[56]);
        s.v[498] = if s.b[498] { 1.0 } else { 0.0 };

        if s.b[498] {
            s.store_ad_value(55, A::add_scaled_product(s.ad_value(56), 1.0, s.ad_value(6), A::ln_one_plus_exp(s.ad_value(285)), 1.0));
        }

        if (!s.b[498]) {
            s.store_offset_mul_ad(55, s.ad_value(6), A::ln_one_plus_exp(A::neg(s.ad_value(285))), 0.05);
        }

        s.store_add_ad(104, A::add_scaled_product(s.ad_value(4), p.p138, s.ad_value(6), s.ad_value(280), (-3.0)), A::scale_offset(s.ad_value(4), (-p.p140), p.p140));

        s.store_div_ad_lhs(285, A::sub_from_scalar(0.05, s.ad_value(104)), 6);

        s.b[499] = (0.05 < s.v[104]);
        s.v[499] = if s.b[499] { 1.0 } else { 0.0 };

        if s.b[499] {
            s.store_ad_value(105, A::add_scaled_product(s.ad_value(104), 1.0, s.ad_value(6), A::ln_one_plus_exp(s.ad_value(285)), 1.0));
        }

        if (!s.b[499]) {
            s.store_offset_mul_ad(105, s.ad_value(6), A::ln_one_plus_exp(A::neg(s.ad_value(285))), 0.05);
        }

        s.store_div_from_scalar(65, 1.0, 14);

        s.store_div_from_scalar(67, 1.0, 19);

        s.store_powf_ad(73, A::scale(s.ad_value(65), p.p66), p.p67);

        s.store_powf_ad(90, A::scale(s.ad_value(67), s.v[75]), s.v[76]);

        s.store_scale(23, 73, p.p65);

        s.store_scaled_powf_ad(106, A::div_from_scalar(p.p138, s.ad_value(105)), p.p139, p.p137);

        s.store_offset_scaled_ad(26, A::powf(A::div_from_scalar(p.p71, s.ad_value(17)), p.p72), (1.0 - p.p75), p.p75);

        s.store_div_from_scalar(27, 1.0, 26);

        s.store_scale(24, 26, p.p70);

        s.store_scale(25, 27, p.p75);

        s.store_scaled_exp_scaled_input(28, 280, p.p97, p.p54);

        s.b[500] = (s.v[28] < s.v[346]);
        s.v[500] = if s.b[500] { 1.0 } else { 0.0 };

        if s.b[500] {
            s.copy_ad(28, 346);
        }

        s.store_scaled_exp_scaled_input(29, 280, (p.p98 - p.p96), p.p56);

        s.store_scaled_exp_scaled_input(30, 280, p.p101, p.p55);

        s.b[501] = (s.v[30] < s.v[346]);
        s.v[501] = if s.b[501] { 1.0 } else { 0.0 };

        if s.b[501] {
            s.copy_ad(30, 346);
        }

        s.store_scaled_exp_scaled_input(32, 280, p.p102, p.p57);

        s.store_scaled_exp_scaled_input(31, 280, p.p99, p.p60);

        s.b[502] = (p.p122 != 0.0);
        s.v[502] = if s.b[502] { 1.0 } else { 0.0 };

        if s.b[502] {
            s.store_offset_scaled(50, 12, ((p.p122) * (p.p10)), p.p10);
            s.store_scaled_offset(285, 50, (-1.0), 1.0 / (s.v[52]));
        }

        s.b[503] = (s.v[50] < 1.0);
        s.v[503] = if s.b[503] { 1.0 } else { 0.0 };

        if (s.b[502] && s.b[503]) {
            s.store_offset_scaled_ad(50, A::ln_one_plus_exp(s.ad_value(285)), s.v[52], 1.0);
        }

        if (s.b[502] && (!s.b[503])) {
            s.store_ad_value(50, A::add_scaled_inputs(s.ad_value(50), 1.0, A::ln_one_plus_exp(A::neg(s.ad_value(285))), s.v[52]));
        }

        if s.b[502] {
            s.store_offset(48, 50, (-(s.v[52] * 0.6931471805599453)));
        }

        if (!s.b[502]) {
            s.store_scalar(48, p.p10);
        }

        s.b[504] = (p.p123 != 0.0);
        s.v[504] = if s.b[504] { 1.0 } else { 0.0 };

        if s.b[504] {
            s.store_offset_scaled(51, 12, ((p.p123) * (p.p11)), p.p11);
            s.store_scaled_offset(285, 51, (-1.0), 1.0 / (s.v[52]));
        }

        s.b[505] = (s.v[51] < 1.0);
        s.v[505] = if s.b[505] { 1.0 } else { 0.0 };

        if (s.b[504] && s.b[505]) {
            s.store_offset_scaled_ad(51, A::ln_one_plus_exp(s.ad_value(285)), s.v[52], 1.0);
        }

        if (s.b[504] && (!s.b[505])) {
            s.store_ad_value(51, A::add_scaled_inputs(s.ad_value(51), 1.0, A::ln_one_plus_exp(A::neg(s.ad_value(285))), s.v[52]));
        }

        if s.b[504] {
            s.store_offset(49, 51, (-(s.v[52] * 0.6931471805599453)));
        }

        if (!s.b[504]) {
            s.store_scalar(49, p.p11);
        }

        s.store_offset_scaled(341, 12, ((p.p124) * (p.p43)), p.p43);

        s.v[287] = (s.v[342] * s.v[342]);

        s.store_square(288, 341);

        s.b[506] = (s.v[341] < 0.0);
        s.v[506] = if s.b[506] { 1.0 } else { 0.0 };

        if s.b[506] {
            s.store_div_from_scalar_sub_ad(340, (0.5 * s.v[287]), A::sqrt(A::offset(s.ad_value(288), s.v[287])), s.ad_value(341));
        }

        if (!s.b[506]) {
            s.store_scaled_add_ad_lhs(340, A::sqrt(A::offset(s.ad_value(288), s.v[287])), 341, 0.5);
        }

        s.store_scaled_mul_ad(35, A::exp(A::div_scaled_inputs(s.ad_value(280), (((4.0 - p.p98) - p.p96) + p.p121), s.ad_value(48), 1.0)), A::exp(A::div_scaled_inputs(s.ad_value(10), (-p.p105), s.ad_value(48), 1.0)), p.p9);

        s.store_scaled_exp_scaled_input(36, 280, (1.0 - p.p98), p.p12);

        s.store_scaled_exp_scaled_input(37, 280, (1.0 - p.p103), p.p30);

        s.store_scaled_mul_ad(42, A::exp_scaled_input(s.ad_value(280), (((4.0 - p.p97) + p.p121) * 1.0 / (p.p17))), A::exp_scaled_input(s.ad_value(10), ((-p.p111) * 1.0 / (p.p17))), p.p16);

        s.store_scaled_mul_ad(43, A::exp_scaled_input(s.ad_value(280), ((4.0 - p.p103) + p.p121)), A::exp_scaled_input(s.ad_value(10), (-p.p112)), p.p29);

        s.store_powf_ad(281, A::scale(s.ad_value(70), s.v[72]), (-0.5));

        s.store_div_from_scalar(282, 1.0, 73);

        s.store_mul_ad_affine_product_lhs(61, A::mul3_scaled_output(s.ad_value(70), s.ad_value(70), s.ad_value(281), p.p35), s.ad_value(282), (p.p66 * (s.v[72] * s.v[72])), 0.0, 65);

        s.store_div_from_scalar(67, 1.0, 19);

        s.store_powf_ad(283, A::scale(s.ad_value(85), s.v[86]), (-0.5));

        s.store_div_from_scalar(284, 1.0, 90);

        s.store_mul_ad_affine_product_lhs(83, A::mul3_scaled_output(s.ad_value(85), s.ad_value(85), s.ad_value(283), p.p37), s.ad_value(284), (s.v[75] * (s.v[86] * s.v[86])), 0.0, 67);

        s.store_exp_scaled_input(281, 280, p.p96);

        s.store_scaled_mul(40, 281, 27, p.p14);

        s.store_scaled_mul(41, 281, 282, p.p13);

        s.store_scaled_mul_ad(107, A::exp_scaled_input(s.ad_value(280), (4.0 - p.p141)), A::exp_scaled_input(s.ad_value(10), (-p.p140)), p.p133);

        s.store_scaled_exp_scaled_input(109, 280, (1.0 - p.p141), p.p135);

        s.store_scaled_mul_ad(94, A::exp_scaled_input(s.ad_value(280), (p.p98 - 2.0)), A::exp_scaled_input(s.ad_value(10), (-p.p120)), p.p86);

        s.store_scaled_exp_scaled_input(95, 280, ((p.p96 + p.p98) - 1.0), p.p87);

        s.store_scaled_exp_scaled_input(96, 280, (p.p99 - 1.0), p.p88);

        s.store_scaled_add(97, 95, 96, (p.p89 * 1.0 / ((p.p87 + p.p88))));

        s.store_scaled_exp_scaled_input(98, 280, (p.p100 - 1.0), p.p90);

        s.store_offset(101, 2, (-300.0));

        s.b[508] = (s.v[2] < 525.0);
        s.v[508] = if s.b[508] { 1.0 } else { 0.0 };

        if s.b[508] {
            s.store_mul_ad_rhs(99, 1, A::add_scaled_product(A::scale_offset(s.ad_value(101), 0.00072, 1.0), 1.0, s.ad_value(101), s.ad_value(101), (-1.6e-6)));
        }

        if (!s.b[508]) {
            s.store_scale(99, 1, 1.081);
        }

        s.store_scaled_exp_scaled_input(100, 280, p.p96, p.p92);

        s.store_scaled_voltage(250, ctx, nodes, Some(7), Some(8), p.p3);

        s.store_scaled_voltage(251, ctx, nodes, Some(7), Some(9), p.p3);

        s.store_scaled_voltage(252, ctx, nodes, Some(7), Some(5), p.p3);

        s.store_scaled_voltage(253, ctx, nodes, Some(6), Some(5), p.p3);

        s.store_scaled_voltage(254, ctx, nodes, Some(6), Some(7), p.p3);

        s.store_scaled_voltage(259, ctx, nodes, Some(3), Some(8), p.p3);

        s.store_scaled_voltage(256, ctx, nodes, Some(8), Some(9), p.p3);

        s.store_scaled_voltage(266, ctx, nodes, Some(1), Some(6), p.p3);

        s.store_scaled_voltage(269, ctx, nodes, Some(1), Some(2), p.p3);

        s.store_scaled_voltage(270, ctx, nodes, Some(1), Some(0), p.p3);

        s.store_scaled_voltage(258, ctx, nodes, Some(11), Some(8), p.p3);

        s.store_scaled_voltage(257, ctx, nodes, Some(10), Some(11), p.p3);

        s.store_sub_ad_lhs(255, A::add_scaled_inputs3(s.ad_value(254), 1.0, s.ad_value(251), 1.0, s.ad_value(256), -1.0), 258);

        s.store_sub_ad_lhs(268, A::add_scaled_inputs3(s.ad_value(266), 1.0, s.ad_value(270), (-1.0), s.ad_value(255), 1.0), 257);

        s.store_add(267, 270, 268);

        s.store_sub(261, 259, 258);

        s.store_sub(260, 261, 257);

        s.b[515] = ((s.v[251] * s.v[8]) < p.p151);
        s.v[515] = if s.b[515] { 1.0 } else { 0.0 };

        if s.b[515] {
            s.store_exp_mul(271, 251, 8);
        }

        if (!s.b[515]) {
            s.store_scalar(301, ((p.p151) as f64).exp());
            s.store_mul_offset_ad_rhs(271, 301, A::mul(s.ad_value(251), s.ad_value(8)), (((-p.p151)) + (1.0)));
        }

        s.b[516] = (((s.v[252] * s.v[8]) / s.v[48]) < p.p151);
        s.v[516] = if s.b[516] { 1.0 } else { 0.0 };

        if s.b[516] {
            s.store_exp_ad(272, A::div_scaled_product(s.ad_value(252), s.ad_value(8), 1.0, s.ad_value(48), 1.0));
        }

        if (!s.b[516]) {
            s.store_scalar(301, ((p.p151) as f64).exp());
            s.store_mul_offset_ad_rhs(272, 301, A::div_scaled_product(s.ad_value(252), s.ad_value(8), 1.0, s.ad_value(48), 1.0), (((-p.p151)) + (1.0)));
        }

        s.b[517] = ((s.v[255] * s.v[8]) < p.p151);
        s.v[517] = if s.b[517] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_reactive_block_1(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[517] {
            s.store_exp_mul(274, 255, 8);
        }

        if (!s.b[517]) {
            s.store_scalar(301, ((p.p151) as f64).exp());
            s.store_mul_offset_ad_rhs(274, 301, A::mul(s.ad_value(255), s.ad_value(8)), (((-p.p151)) + (1.0)));
        }

        s.b[518] = ((s.v[254] * s.v[8]) < p.p151);
        s.v[518] = if s.b[518] { 1.0 } else { 0.0 };

        if (!s.b[518]) {
            s.store_scalar(301, ((p.p151) as f64).exp());
        }

        s.b[519] = ((s.v[267] * s.v[8]) < p.p151);
        s.v[519] = if s.b[519] { 1.0 } else { 0.0 };

        if s.b[519] {
            s.store_exp_mul(275, 267, 8);
        }

        if (!s.b[519]) {
            s.store_scalar(301, ((p.p151) as f64).exp());
            s.store_mul_offset_ad_rhs(275, 301, A::mul(s.ad_value(267), s.ad_value(8)), (((-p.p151)) + (1.0)));
        }

        s.b[520] = ((s.v[259] * s.v[8]) < p.p151);
        s.v[520] = if s.b[520] { 1.0 } else { 0.0 };

        if (!s.b[520]) {
            s.store_scalar(301, ((p.p151) as f64).exp());
        }

        s.b[521] = ((s.v[260] * s.v[8]) < p.p151);
        s.v[521] = if s.b[521] { 1.0 } else { 0.0 };

        if s.b[521] {
            s.store_exp_mul(263, 260, 8);
        }

        if (!s.b[521]) {
            s.store_scalar(301, ((p.p151) as f64).exp());
            s.store_mul_offset_ad_rhs(263, 301, A::mul(s.ad_value(260), s.ad_value(8)), (((-p.p151)) + (1.0)));
        }

        s.b[522] = ((s.v[261] * s.v[8]) < p.p151);
        s.v[522] = if s.b[522] { 1.0 } else { 0.0 };

        if (!s.b[522]) {
            s.store_scalar(301, ((p.p151) as f64).exp());
        }

        s.b[523] = (((s.v[267] - s.v[16]) * s.v[8]) < p.p151);
        s.v[523] = if s.b[523] { 1.0 } else { 0.0 };

        if s.b[523] {
            s.store_exp_ad(278, A::mul(A::sub(s.ad_value(267), s.ad_value(16)), s.ad_value(8)));
        }

        if (!s.b[523]) {
            s.store_scalar(301, ((p.p151) as f64).exp());
            s.store_mul_offset_ad_rhs(278, 301, A::mul(A::sub(s.ad_value(267), s.ad_value(16)), s.ad_value(8)), (((-p.p151)) + (1.0)));
        }

        s.b[524] = (((s.v[255] - s.v[16]) * s.v[8]) < p.p151);
        s.v[524] = if s.b[524] { 1.0 } else { 0.0 };

        if s.b[524] {
            s.store_exp_ad(276, A::mul(A::sub(s.ad_value(255), s.ad_value(16)), s.ad_value(8)));
        }

        if (!s.b[524]) {
            s.store_scalar(301, ((p.p151) as f64).exp());
            s.store_mul_offset_ad_rhs(276, 301, A::mul(A::sub(s.ad_value(255), s.ad_value(16)), s.ad_value(8)), (((-p.p151)) + (1.0)));
        }

        s.b[525] = (((s.v[251] - s.v[16]) * s.v[8]) < p.p151);
        s.v[525] = if s.b[525] { 1.0 } else { 0.0 };

        if s.b[525] {
            s.store_exp_ad(277, A::mul(A::sub(s.ad_value(251), s.ad_value(16)), s.ad_value(8)));
        }

        if (!s.b[525]) {
            s.store_scalar(301, ((p.p151) as f64).exp());
            s.store_mul_offset_ad_rhs(277, 301, A::mul(A::sub(s.ad_value(251), s.ad_value(16)), s.ad_value(8)), (((-p.p151)) + (1.0)));
        }

        s.b[526] = (((s.v[250] - s.v[16]) * s.v[8]) < p.p151);
        s.v[526] = if s.b[526] { 1.0 } else { 0.0 };

        if s.b[526] {
            s.store_exp_ad(279, A::mul(A::sub(s.ad_value(250), s.ad_value(16)), s.ad_value(8)));
        }

        if (!s.b[526]) {
            s.store_scalar(301, ((p.p151) as f64).exp());
            s.store_mul_offset_ad_rhs(279, 301, A::mul(A::sub(s.ad_value(250), s.ad_value(16)), s.ad_value(8)), (((-p.p151)) + (1.0)));
        }

        s.store_sqrt_offset_scaled_input(114, 277, 4.0, 1.0);

        s.store_sqrt_offset_scaled_input(115, 279, 4.0, 1.0);

        s.store_ad_value(116, A::div_scaled_inputs(s.ad_value(279), 2.0, A::offset(s.ad_value(115), 1.0), 1.0));

        s.b[527] = (s.v[116] < p.p153);
        s.v[527] = if s.b[527] { 1.0 } else { 0.0 };

        if s.b[527] {
            s.store_scalar(116, p.p153);
        }

        s.store_mul_ad_rhs(117, 6, A::add_scaled_inputs3(s.ad_value(114), 1.0, s.ad_value(115), (-1.0), A::ln(A::div(A::offset(s.ad_value(114), 1.0), A::offset(s.ad_value(115), 1.0))), -1.0));

        s.store_div_ad_lhs(118, A::add(s.ad_value(117), s.ad_value(256)), 31);

        s.b[528] = (s.v[118] > 0.0);
        s.v[528] = if s.b[528] { 1.0 } else { 0.0 };

        s.b[529] = (s.v[250] < 100.0);
        s.v[529] = if s.b[529] { 1.0 } else { 0.0 };

        if (s.b[528] && s.b[529]) {
            s.copy_ad(303, 250);
        }

        if (s.b[528] && (!s.b[529])) {
            s.store_offset_ln_ad(303, A::offset(s.ad_value(250), (((-100.0)) + (1.0))), 100.0);
        }

        if s.b[528] {
            s.store_sub_ad_lhs(119, A::add_scaled_product(s.ad_value(16), 1.0, s.ad_value(6), A::ln(A::offset(A::mul3_scaled_output(s.ad_value(118), s.ad_value(31), s.ad_value(8), 0.5), 1.0)), 2.0), 303);
            s.store_scale(298, 16, 0.2);
            s.store_square(287, 298);
            s.store_square(288, 119);
        }

        s.b[530] = (s.v[119] < 0.0);
        s.v[530] = if s.b[530] { 1.0 } else { 0.0 };

        if (s.b[528] && s.b[530]) {
            s.store_ad_value(120, A::div_scaled_inputs(s.ad_value(287), 0.5, A::sub(A::sqrt(A::add(s.ad_value(288), s.ad_value(287))), s.ad_value(119)), 1.0));
        }

        if (s.b[528] && (!s.b[530])) {
            s.store_scaled_add_ad_lhs(120, A::sqrt(A::add(s.ad_value(288), s.ad_value(287))), 119, 0.5);
        }

        if s.b[528] {
            s.store_ad_value(121, A::div_scaled_product_offset_rhs(s.ad_value(120), s.ad_value(120), (p.p62 * p.p61), 1.0, A::add_scaled_inputs(s.ad_value(120), p.p61, s.ad_value(31), (p.p62 * p.p61)), 1.0));
            s.store_div(291, 118, 121);
            s.store_scaled_offset(285, 291, (-1.0), 1.0 / (p.p63));
        }

        s.b[531] = (s.v[291] < 1.0);
        s.v[531] = if s.b[531] { 1.0 } else { 0.0 };

        if (s.b[528] && s.b[531]) {
            s.store_offset_scaled_ad(289, A::ln_one_plus_exp(s.ad_value(285)), p.p63, 1.0);
        }

        if (s.b[528] && (!s.b[531])) {
            s.store_ad_value(289, A::add_scaled_inputs(s.ad_value(291), 1.0, A::ln_one_plus_exp(A::neg(s.ad_value(285))), p.p63));
        }

        if s.b[528] {
            s.store_scale(122, 289, 1.0 / ((1.0 + (p.p63 * (((1.0 + ((((-1.0) / p.p63)) as f64).exp())) as f64).ln()))));
            s.store_scale(123, 120, 1.0 / ((p.p62 * p.p61)));
            s.store_div_ad(124, A::offset(A::sqrt(A::offset(A::mul3_scaled_output(s.ad_value(122), s.ad_value(123), A::offset(s.ad_value(123), 1.0), 4.0), 1.0)), 1.0), A::mul_scaled_lhs(s.ad_value(122), 2.0, A::offset(s.ad_value(123), 1.0)));
            s.store_div_ad(125, A::add_scaled_sub_value_product(1.0, s.ad_value(124), 1.0, s.ad_value(116), s.ad_value(124), 1.0), A::offset(A::mul(s.ad_value(116), s.ad_value(124)), 1.0));
            s.store_mul_ad_lhs(127, A::mul3_scaled_output(s.ad_value(118), s.ad_value(31), s.ad_value(125), 0.5), 8);
            s.store_ad_value(292, A::add_scaled_offset_product_rhs(s.ad_value(127), 2.0, s.ad_value(116), A::add(s.ad_value(116), s.ad_value(127)), 1.0, 1.0));
            s.store_scaled_offset(128, 127, (-1.0), 0.5);
            s.store_add_ad_lhs(286, A::square(s.ad_value(128)), 292);
        }

        s.b[532] = (s.v[127] >= 1.0);
        s.v[532] = if s.b[532] { 1.0 } else { 0.0 };

        if (s.b[528] && s.b[532]) {
            s.store_add_ad_rhs(129, 128, A::sqrt(s.ad_value(286)));
        }

        if (s.b[528] && (!s.b[532])) {
            s.store_div_ad_rhs(129, 292, A::sub(A::sqrt(s.ad_value(286)), s.ad_value(128)));
        }

        s.b[533] = (s.v[129] < p.p152);
        s.v[533] = if s.b[533] { 1.0 } else { 0.0 };

        if (s.b[528] && s.b[533]) {
            s.store_scalar(129, p.p152);
        }

        if s.b[528] {
            s.store_mul_ad_product_rhs(131, 129, A::offset(s.ad_value(129), 1.0), A::exp(A::mul(s.ad_value(16), s.ad_value(8))));
            s.store_scaled_offset(133, 118, (-p.p62), (0.5 * p.p61));
            s.store_scaled_mul(134, 31, 118, (p.p61 * p.p62));
            s.store_add_ad_rhs(135, 133, A::sqrt(A::add(A::square(s.ad_value(133)), s.ad_value(134))));
        }

        s.b[534] = (p.p73 == 0.0);
        s.v[534] = if s.b[534] { 1.0 } else { 0.0 };

        if (s.b[528] && s.b[534]) {
            s.store_scale(136, 17, 0.1);
        }

        if (s.b[528] && (!s.b[534])) {
            s.store_mul_offset_ad_rhs(136, 17, A::div_scaled_inputs(s.ad_value(118), 2.0, A::add(s.ad_value(118), s.ad_value(121)), 1.0), 0.1);
        }

        if s.b[528] {
            s.store_ad_value(137, A::div_scaled_inputs(s.ad_value(118), p.p62, A::offset(s.ad_value(118), p.p62), 1.0));
            s.store_div_from_scalar_offset_input(213, p.p62, 118, p.p62);
        }

        if (!s.b[528]) {
            s.store_scalar(121, 0.0);
            s.store_ad_value(129, A::div_scaled_inputs(s.ad_value(277), 2.0, A::offset(s.ad_value(114), 1.0), 1.0));
            s.copy_ad(131, 271);
        }

        s.b[535] = ((((s.v[256]) as f64).abs() < (1e-5 * s.v[6])) || (((s.v[117]) as f64).abs() < ((1e-40 * s.v[6]) * (s.v[114] + s.v[115]))));
        s.v[535] = if s.b[535] { 1.0 } else { 0.0 };

        if ((!s.b[528]) && s.b[535]) {
            s.store_scaled_add(138, 129, 116, 0.5);
            s.store_div_ad_rhs(125, 138, A::offset(s.ad_value(138), 1.0));
        }

        if ((!s.b[528]) && (!s.b[535])) {
            s.store_div_ad_rhs(125, 117, A::add_scaled_inputs3(s.ad_value(117), 1.0, s.ad_value(251), 1.0, s.ad_value(250), -1.0));
        }

        if (!s.b[528]) {
            s.copy_ad(135, 256);
            s.store_scale(136, 17, 0.1);
            s.copy_ad(137, 118);
            s.store_sub_from_scalar_ad(213, 1.0, A::scale(s.ad_value(137), 1.0 / (p.p62)));
        }

        s.store_scale(139, 14, (1.0 - ((3.0) as f64).powf(((-1.0) / p.p67))));

        s.store_scale(299, 14, 0.1);

        s.store_div_ad_lhs(285, A::sub(s.ad_value(252), s.ad_value(139)), 299);

        s.b[536] = (s.v[252] < s.v[139]);
        s.v[536] = if s.b[536] { 1.0 } else { 0.0 };

        if s.b[536] {
            s.store_ad_value(140, A::add_scaled_product(s.ad_value(252), 1.0, s.ad_value(299), A::ln_one_plus_exp(s.ad_value(285)), (-1.0)));
        }

        if (!s.b[536]) {
            s.store_ad_value(140, A::add_scaled_product(s.ad_value(139), 1.0, s.ad_value(299), A::ln_one_plus_exp(A::neg(s.ad_value(285))), (-1.0)));
        }

        s.store_powf_ad(59, A::sub_from_scalar(1.0, A::mul(s.ad_value(140), s.ad_value(65))), (1.0 - p.p67));

        s.store_ad_value(141, A::add_scaled_inputs3(A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(14), 1.0, s.ad_value(59), 1.0 / ((1.0 - p.p67))), 1.0, s.ad_value(252), 3.0, s.ad_value(140), (-3.0)));

        s.b[537] = (p.p74 == 1.0);
        s.v[537] = if s.b[537] { 1.0 } else { 0.0 };

        if s.b[537] {
            s.copy_ad(142, 250);
        }

        s.b[538] = (p.p74 == 2.0);
        s.v[538] = if s.b[538] { 1.0 } else { 0.0 };

        if ((!s.b[537]) && s.b[538]) {
            s.store_add(142, 250, 135);
        }

        if ((!s.b[537]) && (!s.b[538])) {
            s.copy_ad(142, 251);
        }

        s.store_div_ad(143, A::sub_from_scalar(2.0, s.ad_value(25)), A::sub_from_scalar(1.0, s.ad_value(25)));

        s.store_mul_sub_from_scalar_ad_rhs(144, 17, 1.0, A::powf(s.ad_value(143), ((-1.0) / p.p72)));

        s.store_div_ad_lhs(285, A::sub(s.ad_value(142), s.ad_value(144)), 136);

        s.b[539] = (s.v[142] < s.v[144]);
        s.v[539] = if s.b[539] { 1.0 } else { 0.0 };

        if s.b[539] {
            s.store_ad_value(145, A::add_scaled_product(s.ad_value(142), 1.0, s.ad_value(136), A::ln_one_plus_exp(s.ad_value(285)), (-1.0)));
        }

        if (!s.b[539]) {
            s.store_ad_value(145, A::add_scaled_product(s.ad_value(144), 1.0, s.ad_value(136), A::ln_one_plus_exp(A::neg(s.ad_value(285))), (-1.0)));
        }

        s.store_powf(146, 213, p.p76);

        s.store_add_ad(147, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(17), 1.0, A::mul(s.ad_value(146), A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(145), s.ad_value(17))), (1.0 - p.p72))), 1.0 / ((1.0 - p.p72))), A::mul3(s.ad_value(146), s.ad_value(143), A::sub(s.ad_value(142), s.ad_value(145))));

        s.store_ad_value(148, A::add_scaled_product(A::mul_sub_from_scalar_lhs(1.0, s.ad_value(25), s.ad_value(147)), 1.0, s.ad_value(25), s.ad_value(250), 1.0));

        s.store_scaled_div(149, 35, 36, 4.0);

        s.store_mul(150, 149, 272);

        s.store_div_ad_rhs(152, 150, A::offset(A::sqrt(A::offset(s.ad_value(150), 1.0)), 1.0));

        s.store_pow_ad(132, s.ad_value(131), A::div_from_scalar(1.0, s.ad_value(49)));

        s.store_mul(151, 149, 132);

        s.store_div_ad_rhs(153, 151, A::offset(A::sqrt(A::offset(s.ad_value(151), 1.0)), 1.0));

        s.b[540] = (p.p92 == 0.0);
        s.v[540] = if s.b[540] { 1.0 } else { 0.0 };

        if s.b[540] {
            s.store_add_ad(154, A::offset(A::div(s.ad_value(141), s.ad_value(41)), 1.0), A::div(s.ad_value(148), s.ad_value(40)));
        }

        if (!s.b[540]) {
            s.store_mul_ad_product_lhs(295, A::offset(A::div(s.ad_value(141), s.ad_value(41)), 1.0), s.ad_value(100), 8);
            s.store_mul_ad_product_lhs(296, A::div_scaled_inputs(s.ad_value(148), -1.0, s.ad_value(40), 1.0), s.ad_value(100), 8);
            s.store_div_ad(154, A::sub(A::exp(s.ad_value(295)), A::exp(s.ad_value(296))), A::offset(A::exp(A::mul(s.ad_value(100), s.ad_value(8))), (-1.0)));
        }

        s.v[287] = (0.1 * 0.1);

        s.store_square(288, 154);

        s.b[541] = (s.v[154] < 0.0);
        s.v[541] = if s.b[541] { 1.0 } else { 0.0 };

        if s.b[541] {
            s.store_div_from_scalar_sub_ad(155, (0.5 * s.v[287]), A::sqrt(A::offset(s.ad_value(288), s.v[287])), s.ad_value(154));
        }

        if (!s.b[541]) {
            s.store_scaled_add_ad_lhs(155, A::sqrt(A::offset(s.ad_value(288), s.v[287])), 154, 0.5);
        }

        s.store_mul_offset_ad_rhs(156, 155, A::add_scaled_inputs(s.ad_value(152), 0.5, s.ad_value(153), 0.5), 1.0);

        s.store_scaled_mul(157, 35, 132, p.p15);

        s.store_mul(158, 35, 272);

        s.store_div_ad_lhs(159, A::sub(s.ad_value(158), s.ad_value(157)), 156);

        s.store_scale(285, 252, 10000.0);

        s.b[542] = (s.v[252] < 0.0);
        s.v[542] = if s.b[542] { 1.0 } else { 0.0 };

        if s.b[542] {
            s.store_scaled_ln_one_plus_exp(302, 285, 0.0001);
        }

        if (!s.b[542]) {
            s.store_ad_value(302, A::add_scaled_inputs(s.ad_value(252), 1.0, A::ln_one_plus_exp(A::neg(s.ad_value(285))), 0.0001));
        }

        s.store_scale(304, 302, 1.0 / (p.p156));

        s.b[543] = (s.v[304] < p.p151);
        s.v[543] = if s.b[543] { 1.0 } else { 0.0 };

        if (!s.b[543]) {
            s.store_scalar(301, ((p.p151) as f64).exp());
        }

        s.store_scaled_offset(285, 252, (-p.p158), 1000.0);

        s.b[545] = (((s.v[252] * s.v[8]) / p.p17) < p.p151);
        s.v[545] = if s.b[545] { 1.0 } else { 0.0 };

        if s.b[545] {
            s.store_ad_value(302, A::exp_scaled_input(A::mul(s.ad_value(252), s.ad_value(8)), 1.0 / (p.p17)));
        }

        if (!s.b[545]) {
            s.store_scalar(301, ((p.p151) as f64).exp());
            s.store_mul_offset_ad_rhs(302, 301, A::mul_scaled_output(s.ad_value(252), s.ad_value(8), 1.0 / (p.p17)), (((-p.p151)) + (1.0)));
        }

        s.b[546] = (p.p24 == 1.0);
        s.v[546] = if s.b[546] { 1.0 } else { 0.0 };

        s.b[547] = (((s.v[252] - s.v[55]) * s.v[8]) < p.p151);
        s.v[547] = if s.b[547] { 1.0 } else { 0.0 };

        if (s.b[546] && s.b[547]) {
            s.store_exp_ad(304, A::mul(A::sub(s.ad_value(252), s.ad_value(55)), s.ad_value(8)));
        }

        if (s.b[546] && (!s.b[547])) {
            s.store_scalar(301, ((p.p151) as f64).exp());
            s.store_mul_offset_ad_rhs(304, 301, A::mul(A::sub(s.ad_value(252), s.ad_value(55)), s.ad_value(8)), (((-p.p151)) + (1.0)));
        }

        s.b[548] = (((s.v[159] / s.v[35]) - 1000.0) < 40.0);
        s.v[548] = if s.b[548] { 1.0 } else { 0.0 };

        if (s.b[546] && (!s.b[548])) {
            s.store_scalar(301, ((40.0) as f64).exp());
        }

    }

    pub(super) fn stamp_reactive_block_2(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[550] = (((s.v[253] * s.v[8]) / p.p19) < p.p151);
        s.v[550] = if s.b[550] { 1.0 } else { 0.0 };

        if s.b[550] {
            s.store_ad_value(302, A::exp_scaled_input(A::mul(s.ad_value(253), s.ad_value(8)), 1.0 / (p.p19)));
        }

        if (!s.b[550]) {
            s.store_scalar(301, ((p.p151) as f64).exp());
            s.store_mul_offset_ad_rhs(302, 301, A::mul_scaled_output(s.ad_value(253), s.ad_value(8), 1.0 / (p.p19)), (((-p.p151)) + (1.0)));
        }

        s.b[551] = (p.p24 == 1.0);
        s.v[551] = if s.b[551] { 1.0 } else { 0.0 };

        s.b[552] = (((s.v[253] - s.v[55]) * s.v[8]) < p.p151);
        s.v[552] = if s.b[552] { 1.0 } else { 0.0 };

        if (s.b[551] && s.b[552]) {
            s.store_exp_ad(304, A::mul(A::sub(s.ad_value(253), s.ad_value(55)), s.ad_value(8)));
        }

        if (s.b[551] && (!s.b[552])) {
            s.store_scalar(301, ((p.p151) as f64).exp());
            s.store_mul_offset_ad_rhs(304, 301, A::mul(A::sub(s.ad_value(253), s.ad_value(55)), s.ad_value(8)), (((-p.p151)) + (1.0)));
        }

        s.b[553] = (((s.v[252] * s.v[8]) / p.p21) < p.p151);
        s.v[553] = if s.b[553] { 1.0 } else { 0.0 };

        if s.b[553] {
            s.store_ad_value(302, A::exp_scaled_input(A::mul(s.ad_value(252), s.ad_value(8)), 1.0 / (p.p21)));
        }

        if (!s.b[553]) {
            s.store_scalar(301, ((p.p151) as f64).exp());
            s.store_mul_offset_ad_rhs(302, 301, A::mul_scaled_output(s.ad_value(252), s.ad_value(8), 1.0 / (p.p21)), (((-p.p151)) + (1.0)));
        }

        s.b[554] = (((s.v[253] * s.v[8]) / p.p23) < p.p151);
        s.v[554] = if s.b[554] { 1.0 } else { 0.0 };

        if s.b[554] {
            s.store_ad_value(302, A::exp_scaled_input(A::mul(s.ad_value(253), s.ad_value(8)), 1.0 / (p.p23)));
        }

        if (!s.b[554]) {
            s.store_scalar(301, ((p.p151) as f64).exp());
            s.store_mul_offset_ad_rhs(302, 301, A::mul_scaled_output(s.ad_value(253), s.ad_value(8), 1.0 / (p.p23)), (((-p.p151)) + (1.0)));
        }

        s.b[555] = (((s.v[255] * s.v[8]) / p.p32) < p.p151);
        s.v[555] = if s.b[555] { 1.0 } else { 0.0 };

        if s.b[555] {
            s.store_ad_value(302, A::exp_scaled_input(A::mul(s.ad_value(255), s.ad_value(8)), 1.0 / (p.p32)));
        }

        if (!s.b[555]) {
            s.store_scalar(301, ((p.p151) as f64).exp());
            s.store_mul_offset_ad_rhs(302, 301, A::mul_scaled_output(s.ad_value(255), s.ad_value(8), 1.0 / (p.p32)), (((-p.p151)) + (1.0)));
        }

        s.b[556] = (((s.v[253] * s.v[8]) / p.p150) < p.p151);
        s.v[556] = if s.b[556] { 1.0 } else { 0.0 };

        if s.b[556] {
            s.store_ad_value(302, A::exp_scaled_input(A::mul(s.ad_value(253), s.ad_value(8)), 1.0 / (p.p150)));
        }

        if (!s.b[556]) {
            s.store_scalar(301, ((p.p151) as f64).exp());
            s.store_mul_offset_ad_rhs(302, 301, A::mul_scaled_output(s.ad_value(253), s.ad_value(8), 1.0 / (p.p150)), (((-p.p151)) + (1.0)));
        }

        s.b[557] = (((p.p34 > 0.0) && (p.p35 > 0.0)) && (s.v[252] < 0.0));
        s.v[557] = if s.b[557] { 1.0 } else { 0.0 };

        s.b[558] = ((s.v[61] * (1.0 - (s.v[62] / (2.0 * s.v[59])))) < p.p151);
        s.v[558] = if s.b[558] { 1.0 } else { 0.0 };

        if (s.b[557] && (!s.b[558])) {
            s.store_scalar(301, ((p.p151) as f64).exp());
        }

        if s.b[557] {
            s.store_mul(281, 252, 65);
            s.store_scaled_mul_ad(60, A::powf(A::sqrt(A::offset(A::square(s.ad_value(281)), 1e-30)), ((-2.0) - p.p67)), A::sub(A::scale_offset(A::scale(s.ad_value(281), (3.0 * (p.p67 - 1.0))), (-p.p67), (((1.0 - (p.p67 * p.p67))) * (p.p67))), A::mul3_scaled_output(s.ad_value(281), s.ad_value(281), A::offset(s.ad_value(281), (p.p67 - 1.0)), 6.0)), 0.16666666666666666);
            s.store_ad_value(281, A::div_scaled_product_by_product(s.ad_value(252), s.ad_value(61), s.v[62], s.ad_value(70), s.ad_value(60), 1.0));
        }

        s.b[559] = (s.v[281] < (-0.001));
        s.v[559] = if s.b[559] { 1.0 } else { 0.0 };

        s.b[560] = (s.v[281] < p.p151);
        s.v[560] = if s.b[560] { 1.0 } else { 0.0 };

        if ((s.b[557] && s.b[559]) && (!s.b[560])) {
            s.store_scalar(301, ((p.p151) as f64).exp());
        }

        s.b[561] = (((p.p36 > 0.0) && (p.p37 > 0.0)) && (s.v[250] < 0.0));
        s.v[561] = if s.b[561] { 1.0 } else { 0.0 };

        if s.b[561] {
            s.store_powf_ad(77, A::sub_from_scalar(1.0, A::mul(s.ad_value(250), s.ad_value(67))), (1.0 - s.v[76]));
        }

        s.b[562] = ((s.v[83] * (1.0 - (s.v[79] / (2.0 * s.v[77])))) < p.p151);
        s.v[562] = if s.b[562] { 1.0 } else { 0.0 };

        if (s.b[561] && (!s.b[562])) {
            s.store_scalar(301, ((p.p151) as f64).exp());
        }

        if s.b[561] {
            s.store_mul(283, 250, 67);
        }

        if s.b[561] {
            let assign4640_ad_e4484: A = A::mul_scaled_output(A::powf(A::sqrt(A::offset(A::square(s.ad_value(283)), 1e-30)), ((-2.0) - s.v[76])), A::sub(A::scale_offset(A::scale(s.ad_value(283), (3.0 * (s.v[76] - 1.0))), (-s.v[76]), (((1.0 - (s.v[76] * s.v[76]))) * (s.v[76]))), A::mul3_scaled_output(s.ad_value(283), s.ad_value(283), A::offset(s.ad_value(283), (s.v[76] - 1.0)), 6.0)), 0.16666666666666666);
            s.store_ad_value(80, assign4640_ad_e4484);
        }

        if s.b[561] {
            s.store_ad_value(283, A::div_scaled_product_by_product(s.ad_value(250), s.ad_value(83), s.v[79], s.ad_value(85), s.ad_value(80), 1.0));
        }

        s.b[563] = (s.v[283] < (-0.001));
        s.v[563] = if s.b[563] { 1.0 } else { 0.0 };

        s.b[564] = (s.v[283] < p.p151);
        s.v[564] = if s.b[564] { 1.0 } else { 0.0 };

        if ((s.b[561] && s.b[563]) && (!s.b[564])) {
            s.store_scalar(301, ((p.p151) as f64).exp());
        }

        s.store_mul(168, 149, 274);

        s.store_scale(169, 276, 4.0);

        s.store_div_ad(171, A::sub(s.ad_value(168), s.ad_value(149)), A::offset(A::sqrt(A::offset(s.ad_value(168), 1.0)), 1.0));

        s.store_div_ad_rhs(170, 169, A::offset(A::sqrt(A::offset(s.ad_value(169), 1.0)), 1.0));

        s.b[566] = ((p.p5 > 0.0) && (p.p33 > 0.0));
        s.v[566] = if s.b[566] { 1.0 } else { 0.0 };

        if s.b[566] {
            s.store_ad_value(174, A::div_scaled_product_offset_denominator(s.ad_value(43), A::offset(s.ad_value(275), (-1.0)), (p.p33 * 2.0), A::sqrt(A::offset(A::mul(A::div_scaled_inputs(s.ad_value(43), 4.0, s.ad_value(37), 1.0), s.ad_value(275)), 1.0)), 1.0, 1.0));
        }

        s.b[567] = (p.p8 == 1.0);
        s.v[567] = if s.b[567] { 1.0 } else { 0.0 };

        if (s.b[566] && s.b[567]) {
            s.store_ad_value(175, A::div_scaled_product_offset_denominator(s.ad_value(107), A::sub(s.ad_value(275), s.ad_value(263)), (((1.0 - p.p143) * p.p33) * 2.0), A::sqrt(A::offset(A::mul(A::div_scaled_inputs(s.ad_value(107), 4.0, s.ad_value(109), 1.0), A::add_scaled_inputs(s.ad_value(275), 1.0, s.ad_value(263), p.p144)), 1.0)), 1.0, 1.0));
        }

        if (s.b[566] && (!s.b[567])) {
            s.store_ad_value(175, A::div_scaled_product_offset_denominator(s.ad_value(107), A::offset(s.ad_value(275), (-1.0)), (((1.0 - p.p143) * p.p33) * 2.0), A::sqrt(A::offset(A::mul(A::div_scaled_inputs(s.ad_value(107), 4.0, s.ad_value(109), 1.0), s.ad_value(275)), 1.0)), 1.0, 1.0));
        }

        s.b[568] = (p.p5 == 1.0);
        s.v[568] = if s.b[568] { 1.0 } else { 0.0 };

        if (s.b[566] && s.b[568]) {
            s.store_mul_scale_ad_lhs(297, A::add(s.ad_value(43), s.ad_value(107)), p.p33, 32);
            s.store_mul_sub_from_scalar_ad_rhs(176, 6, 2.0, A::ln(A::mul(s.ad_value(297), s.ad_value(8))));
            s.store_sub(290, 267, 176);
            s.store_scalar(287, (0.11 * 0.11));
            s.store_square(288, 290);
        }

        s.b[569] = (s.v[290] < 0.0);
        s.v[569] = if s.b[569] { 1.0 } else { 0.0 };

        if ((s.b[566] && s.b[568]) && s.b[569]) {
            s.store_ad_value(177, A::div_scaled_inputs(s.ad_value(287), 0.5, A::sub(A::sqrt(A::add(s.ad_value(288), s.ad_value(287))), s.ad_value(290)), 1.0));
        }

        if ((s.b[566] && s.b[568]) && (!s.b[569])) {
            s.store_scaled_add_ad_lhs(177, A::sqrt(A::add(s.ad_value(288), s.ad_value(287))), 290, 0.5);
        }

        if (s.b[566] && s.b[568]) {
            s.store_div_ad_rhs(178, 177, A::add(A::add_scaled_product(s.ad_value(297), 1.0, A::add(s.ad_value(174), s.ad_value(175)), s.ad_value(32), 1.0), s.ad_value(177)));
        }

        if (s.b[566] && (!s.b[568])) {
            s.store_scalar(176, 0.0);
            s.store_scalar(290, 0.0);
            s.store_scalar(177, 0.0);
            s.store_scalar(178, 1.0);
        }

        s.b[570] = (p.p84 == 1.0);
        s.v[570] = if s.b[570] { 1.0 } else { 0.0 };

        if s.b[570] {
            s.store_add(353, 254, 250);
            s.store_scalar(287, (1e-6 * 1e-6));
            s.store_scaled_mul(288, 353, 353, ((-1.0) * (-1.0)));
        }

        s.store_add_ad(186, A::offset(A::div(s.ad_value(141), s.ad_value(41)), 1.0), A::div(s.ad_value(148), s.ad_value(40)));

        s.v[287] = (0.1 * 0.1);

        s.store_square(288, 186);

        s.b[573] = (s.v[186] < 0.0);
        s.v[573] = if s.b[573] { 1.0 } else { 0.0 };

        if s.b[573] {
            s.store_div_from_scalar_sub_ad(187, (0.5 * s.v[287]), A::sqrt(A::offset(s.ad_value(288), s.v[287])), s.ad_value(186));
        }

        if (!s.b[573]) {
            s.store_scaled_add_ad_lhs(187, A::sqrt(A::offset(s.ad_value(288), s.v[287])), 186, 0.5);
        }

        s.store_mul_offset_ad_rhs(188, 187, A::add_scaled_inputs(s.ad_value(152), 0.5, s.ad_value(153), 0.5), 1.0);

        s.store_div(190, 29, 188);

        s.b[574] = (s.v[190] < s.v[346]);
        s.v[574] = if s.b[574] { 1.0 } else { 0.0 };

        if s.b[574] {
            s.copy_ad(190, 346);
        }

        s.store_scale(189, 190, 3.0);

        s.b[575] = (s.v[159] > 0.0);
        s.v[575] = if s.b[575] { 1.0 } else { 0.0 };

        s.b[576] = (p.p39 == 1.0);
        s.v[576] = if s.b[576] { 1.0 } else { 0.0 };

        s.b[577] = (s.v[250] < p.p44);
        s.v[577] = if s.b[577] { 1.0 } else { 0.0 };

        s.b[578] = (((-s.v[159]) / p.p42) < p.p151);
        s.v[578] = if s.b[578] { 1.0 } else { 0.0 };

        if (((s.b[575] && s.b[576]) && s.b[577]) && s.b[578]) {
            s.store_exp_scaled_input(338, 159, (-1.0 / (p.p42)));
        }

        if (((s.b[575] && s.b[576]) && s.b[577]) && (!s.b[578])) {
            s.store_scalar(301, ((p.p151) as f64).exp());
            s.store_mul_ad_rhs(338, 301, A::scale_offset(s.ad_value(159), (-1.0 / (p.p42)), (((-p.p151)) + (1.0))));
        }

        if ((s.b[575] && s.b[576]) && s.b[577]) {
            s.store_mul_sub_from_scalar_lhs(339, p.p44, 250, 338);
        }

        s.b[579] = (((-s.v[340]) * ((s.v[339]) as f64).powf(p.p41)) < p.p151);
        s.v[579] = if s.b[579] { 1.0 } else { 0.0 };

        if (((s.b[575] && s.b[576]) && s.b[577]) && s.b[579]) {
            s.store_exp_ad(343, A::mul_scaled_lhs(s.ad_value(340), -1.0, A::powf(s.ad_value(339), p.p41)));
        }

        if (((s.b[575] && s.b[576]) && s.b[577]) && (!s.b[579])) {
            s.store_scalar(301, ((p.p151) as f64).exp());
            s.store_mul_offset_ad_rhs(343, 301, A::mul_scaled_lhs(s.ad_value(340), -1.0, A::powf(s.ad_value(339), p.p41)), (((-p.p151)) + (1.0)));
        }

        if ((s.b[575] && s.b[576]) && s.b[577]) {
            s.store_mul_ad_product_lhs(210, A::div_from_scalar(p.p40, s.ad_value(340)), s.ad_value(339), 343);
        }

        s.b[580] = (p.p39 == 2.0);
        s.v[580] = if s.b[580] { 1.0 } else { 0.0 };

        s.b[581] = (s.v[250] < s.v[16]);
        s.v[581] = if s.b[581] { 1.0 } else { 0.0 };

        if (((s.b[575] && (!s.b[576])) && s.b[580]) && s.b[581]) {
            s.store_scalar(199, ((2.0 * p.p46) / (p.p45 * p.p45)));
            s.store_div_ad_lhs(286, A::sub(s.ad_value(16), s.ad_value(250)), 213);
            s.store_sqrt_ad(200, A::div_scaled_inputs(s.ad_value(286), 2.0, s.ad_value(199), 1.0));
        }

        s.b[582] = (p.p7 == 0.0);
        s.v[582] = if s.b[582] { 1.0 } else { 0.0 };

        if ((((s.b[575] && (!s.b[576])) && s.b[580]) && s.b[581]) && s.b[582]) {
            s.store_scalar(201, p.p45);
        }

        if ((((s.b[575] && (!s.b[576])) && s.b[580]) && s.b[581]) && (!s.b[582])) {
            s.store_sub_from_scalar_ad(126, 1.0, A::scale(s.ad_value(125), 0.5));
            s.store_scaled_mul(201, 126, 126, p.p45);
        }

        if (((s.b[575] && (!s.b[576])) && s.b[580]) && s.b[581]) {
            s.store_ad_value(202, A::div_scaled_product(s.ad_value(200), s.ad_value(201), 1.0, A::sqrt(A::add(A::square(s.ad_value(200)), A::square(s.ad_value(201)))), 1.0));
            s.store_div_ad_lhs(203, A::sub(s.ad_value(16), s.ad_value(250)), 202);
            s.store_add_ad_rhs(204, 203, A::mul3_scaled_output(s.ad_value(202), s.ad_value(199), s.ad_value(213), 0.5));
        }

        s.b[583] = (p.p7 == 0.0);
        s.v[583] = if s.b[583] { 1.0 } else { 0.0 };

        if ((((s.b[575] && (!s.b[576])) && s.b[580]) && s.b[581]) && s.b[583]) {
            s.copy_ad(205, 204);
        }

        if ((((s.b[575] && (!s.b[576])) && s.b[580]) && s.b[581]) && (!s.b[583])) {
            s.store_offset_scaled(206, 125, ((2.0) * ((2.0 * p.p47))), (((2.0 * p.p47)) + (1.0)));
            s.store_scalar(207, ((1.0 + p.p47) / (1.0 + (2.0 * p.p47))));
            s.store_sub_ad_rhs(208, 203, A::mul3_scaled_output(s.ad_value(202), s.ad_value(199), A::sub(s.ad_value(207), A::div_scaled_inputs(s.ad_value(159), 1.0, s.ad_value(206), p.p62)), 0.5));
            s.store_ad_value(286, A::add_scaled_product(A::mul3_scaled_output(s.ad_value(203), s.ad_value(203), s.ad_value(137), (0.1 * 1.0 / (p.p62))), 1.0, A::sub(s.ad_value(208), s.ad_value(204)), A::sub(s.ad_value(208), s.ad_value(204)), 1.0));
            s.store_ad_value(205, A::add_scaled_inputs3(s.ad_value(208), 0.5, s.ad_value(204), 0.5, A::sqrt(s.ad_value(286)), 0.5));
        }

        if (((s.b[575] && (!s.b[576])) && s.b[580]) && s.b[581]) {
            s.store_div_ad_lhs(293, A::sub(s.ad_value(205), s.ad_value(203)), 205);
        }

        s.b[584] = (((s.v[293]) as f64).abs() > 1e-7);
        s.v[584] = if s.b[584] { 1.0 } else { 0.0 };

        if ((((s.b[575] && (!s.b[576])) && s.b[580]) && s.b[581]) && s.b[584]) {
            s.store_scaled_div(209, 202, 293, 0.5);
            s.store_mul_ad(210, A::mul3(A::div(s.ad_value(0), s.ad_value(99)), s.ad_value(205), s.ad_value(209)), A::sub(A::exp(A::div_scaled_inputs(s.ad_value(99), -1.0, s.ad_value(205), 1.0)), A::exp(A::mul_offset_rhs(A::div_scaled_inputs(s.ad_value(99), -1.0, s.ad_value(205), 1.0), A::div(s.ad_value(201), s.ad_value(209)), 1.0))));
        }

        if ((((s.b[575] && (!s.b[576])) && s.b[580]) && s.b[581]) && (!s.b[584])) {
            s.store_mul_ad_product_rhs(210, 0, s.ad_value(201), A::exp(A::div_scaled_inputs(s.ad_value(99), -1.0, s.ad_value(205), 1.0)));
        }

        s.b[585] = (p.p39 == 3.0);
        s.v[585] = if s.b[585] { 1.0 } else { 0.0 };

        s.b[586] = (s.v[250] < p.p44);
        s.v[586] = if s.b[586] { 1.0 } else { 0.0 };

        if ((((s.b[575] && (!s.b[576])) && (!s.b[580])) && s.b[585]) && s.b[586]) {
            s.store_mul_ad(214, A::powf(A::sub_from_scalar(p.p44, s.ad_value(250)), p.p41), A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(159), A::offset(s.ad_value(159), p.p48))), p.p49));
        }

        s.b[587] = (p.p7 == 0.0);
        s.v[587] = if s.b[587] { 1.0 } else { 0.0 };

        if (((((s.b[575] && (!s.b[576])) && (!s.b[580])) && s.b[585]) && s.b[586]) && s.b[587]) {
            s.copy_ad(215, 214);
        }

        if (((((s.b[575] && (!s.b[576])) && (!s.b[580])) && s.b[585]) && s.b[586]) && (!s.b[587])) {
            s.store_scaled_offset(216, 159, (-p.p52), 1.0 / (p.p48));
            s.store_scaled_offset(285, 216, (-1.0), 1.0 / (p.p51));
        }

        s.b[588] = (s.v[216] < 1.0);
        s.v[588] = if s.b[588] { 1.0 } else { 0.0 };

        if ((((((s.b[575] && (!s.b[576])) && (!s.b[580])) && s.b[585]) && s.b[586]) && (!s.b[587])) && s.b[588]) {
            s.store_offset_scaled_ad(217, A::ln_one_plus_exp(s.ad_value(285)), p.p51, 1.0);
        }

        if ((((((s.b[575] && (!s.b[576])) && (!s.b[580])) && s.b[585]) && s.b[586]) && (!s.b[587])) && (!s.b[588])) {
            s.store_ad_value(217, A::add_scaled_inputs(s.ad_value(216), 1.0, A::ln_one_plus_exp(A::neg(s.ad_value(285))), p.p51));
        }

        if (((((s.b[575] && (!s.b[576])) && (!s.b[580])) && s.b[585]) && s.b[586]) && (!s.b[587])) {
            s.store_mul_powf_ad_rhs(215, 214, s.ad_value(217), p.p50);
        }

        s.b[589] = (((-s.v[340]) * s.v[215]) < p.p151);
        s.v[589] = if s.b[589] { 1.0 } else { 0.0 };

        if (((((s.b[575] && (!s.b[576])) && (!s.b[580])) && s.b[585]) && s.b[586]) && s.b[589]) {
            s.store_exp_ad(343, A::mul_scaled_lhs(s.ad_value(340), -1.0, s.ad_value(215)));
        }

        if (((((s.b[575] && (!s.b[576])) && (!s.b[580])) && s.b[585]) && s.b[586]) && (!s.b[589])) {
            s.store_scalar(301, ((p.p151) as f64).exp());
            s.store_mul_offset_ad_rhs(343, 301, A::mul_scaled_lhs(s.ad_value(340), -1.0, s.ad_value(215)), (((-p.p151)) + (1.0)));
        }

        if ((((s.b[575] && (!s.b[576])) && (!s.b[580])) && s.b[585]) && s.b[586]) {
            s.store_mul_ad_lhs(210, A::mul_sub_from_scalar_rhs(A::div_from_scalar(p.p40, s.ad_value(340)), p.p44, s.ad_value(250)), 343);
        }

        s.b[590] = (s.v[210] > 0.0);
        s.v[590] = if s.b[590] { 1.0 } else { 0.0 };

        s.b[591] = (p.p53 == 1.0);
        s.v[591] = if s.b[591] { 1.0 } else { 0.0 };

        if ((s.b[575] && s.b[590]) && s.b[591]) {
            s.store_add_ad(211, A::add_scaled_product(A::div(s.ad_value(6), A::mul(s.ad_value(159), A::add(s.ad_value(30), s.ad_value(189)))), 1.0, A::div(s.ad_value(156), s.ad_value(35)), s.ad_value(42), 1.0), A::div(s.ad_value(28), A::add(s.ad_value(30), s.ad_value(189))));
        }

        s.b[592] = (p.p39 == 3.0);
        s.v[592] = if s.b[592] { 1.0 } else { 0.0 };

        if (((s.b[575] && s.b[590]) && s.b[591]) && s.b[592]) {
            s.store_scaled_sub(285, 210, 211, 1000000.0);
        }

        s.b[593] = (s.v[210] < s.v[211]);
        s.v[593] = if s.b[593] { 1.0 } else { 0.0 };

        if ((((s.b[575] && s.b[590]) && s.b[591]) && s.b[592]) && s.b[593]) {
            s.store_ad_value(210, A::sub_scaled_inputs(s.ad_value(210), 1.0, A::ln_one_plus_exp(s.ad_value(285)), 1e-6));
        }

        if ((((s.b[575] && s.b[590]) && s.b[591]) && s.b[592]) && (!s.b[593])) {
            s.store_ad_value(210, A::sub_scaled_inputs(s.ad_value(211), 1.0, A::ln_one_plus_exp(A::neg(s.ad_value(285))), 1e-6));
        }

    }

    pub(super) fn stamp_reactive_block_3(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        let nv4 = ctx.node_voltage(nodes[4]);
        s.store_scaled_mul(221, 23, 141, (1.0 - p.p68));

        s.store_div_ad_lhs(285, A::sub(s.ad_value(253), s.ad_value(139)), 299);

        s.b[596] = (s.v[253] < s.v[139]);
        s.v[596] = if s.b[596] { 1.0 } else { 0.0 };

        if s.b[596] {
            s.store_ad_value(222, A::add_scaled_product(s.ad_value(253), 1.0, s.ad_value(299), A::ln_one_plus_exp(s.ad_value(285)), (-1.0)));
        }

        if (!s.b[596]) {
            s.store_ad_value(222, A::add_scaled_product(s.ad_value(139), 1.0, s.ad_value(299), A::ln_one_plus_exp(A::neg(s.ad_value(285))), (-1.0)));
        }

        s.store_mul_scaled_ad_rhs(223, 23, p.p68, A::add_scaled_inputs3(A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(14), 1.0, A::powf(A::sub_from_scalar(1.0, A::mul(s.ad_value(222), s.ad_value(65))), (1.0 - p.p67)), 1.0 / ((1.0 - p.p67))), 1.0, s.ad_value(253), 3.0, s.ad_value(222), (-3.0)));

        s.store_scaled_mul(224, 24, 148, p.p77);

        s.store_mul(225, 95, 36);

        s.store_mul3_affine_lhs(229, 225, 152, 0.5, 0.0, 187);

        s.store_mul3_affine_lhs(230, 225, 153, 0.5, 0.0, 187);

        s.store_scale(300, 17, 0.1);

        s.store_div_ad_lhs(285, A::sub(s.ad_value(255), s.ad_value(144)), 300);

        s.b[597] = (s.v[255] < s.v[144]);
        s.v[597] = if s.b[597] { 1.0 } else { 0.0 };

        if s.b[597] {
            s.store_ad_value(231, A::add_scaled_product(s.ad_value(255), 1.0, s.ad_value(300), A::ln_one_plus_exp(s.ad_value(285)), (-1.0)));
        }

        if (!s.b[597]) {
            s.store_ad_value(231, A::add_scaled_product(s.ad_value(144), 1.0, s.ad_value(300), A::ln_one_plus_exp(A::neg(s.ad_value(285))), (-1.0)));
        }

        s.store_ad_value(232, A::add_scaled_product(A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(17), 1.0, A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(231), s.ad_value(17))), (1.0 - p.p72)), 1.0 / ((1.0 - p.p72))), 1.0, s.ad_value(143), A::sub(s.ad_value(255), s.ad_value(231)), 1.0));

        s.store_mul_scaled_ad_rhs(233, 24, ((1.0 - p.p77) * (1.0 - p.p33)), A::add_scaled_product(A::mul_sub_from_scalar_lhs(1.0, s.ad_value(25), s.ad_value(232)), 1.0, s.ad_value(25), s.ad_value(255), 1.0));

        s.store_div_ad_lhs(285, A::sub(s.ad_value(267), s.ad_value(144)), 300);

        s.b[598] = (s.v[267] < s.v[144]);
        s.v[598] = if s.b[598] { 1.0 } else { 0.0 };

        if s.b[598] {
            s.store_ad_value(234, A::add_scaled_product(s.ad_value(267), 1.0, s.ad_value(300), A::ln_one_plus_exp(s.ad_value(285)), (-1.0)));
        }

        if (!s.b[598]) {
            s.store_ad_value(234, A::add_scaled_product(s.ad_value(144), 1.0, s.ad_value(300), A::ln_one_plus_exp(A::neg(s.ad_value(285))), (-1.0)));
        }

        s.store_ad_value(235, A::add_scaled_product(A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(17), 1.0, A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(234), s.ad_value(17))), (1.0 - p.p72)), 1.0 / ((1.0 - p.p72))), 1.0, s.ad_value(143), A::sub(s.ad_value(267), s.ad_value(234)), 1.0));

        s.store_mul_scaled_ad_rhs(236, 24, ((1.0 - p.p77) * p.p33), A::add_scaled_product(A::mul_sub_from_scalar_lhs(1.0, s.ad_value(25), s.ad_value(235)), 1.0, s.ad_value(25), s.ad_value(267), 1.0));

        s.store_scale(307, 105, 0.1);

        s.store_scale(237, 105, (1.0 - ((2.0) as f64).powf(((-1.0) / p.p139))));

        s.store_div_ad_lhs(285, A::sub(s.ad_value(259), s.ad_value(237)), 307);

        s.b[599] = (s.v[259] < s.v[237]);
        s.v[599] = if s.b[599] { 1.0 } else { 0.0 };

        if s.b[599] {
            s.store_ad_value(238, A::add_scaled_product(s.ad_value(259), 1.0, s.ad_value(307), A::ln_one_plus_exp(s.ad_value(285)), (-1.0)));
        }

        if (!s.b[599]) {
            s.store_ad_value(238, A::add_scaled_product(s.ad_value(237), 1.0, s.ad_value(307), A::ln_one_plus_exp(A::neg(s.ad_value(285))), (-1.0)));
        }

        s.store_mul_ad_rhs(239, 106, A::add_scaled_inputs3(A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(105), 1.0, A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(238), s.ad_value(105))), (1.0 - p.p139)), 1.0 / ((1.0 - p.p139))), 1.0, s.ad_value(259), 2.0, s.ad_value(238), (-2.0)));

        s.store_mul_ad_product_rhs(240, 94, s.ad_value(36), A::powf(A::div(s.ad_value(35), s.ad_value(36)), (1.0 / p.p85)));

        s.b[600] = ((s.v[252] / (p.p85 * s.v[6])) < p.p151);
        s.v[600] = if s.b[600] { 1.0 } else { 0.0 };

        if s.b[600] {
            s.store_exp_ad(302, A::div_scaled_inputs(s.ad_value(252), 1.0, s.ad_value(6), p.p85));
        }

        if (!s.b[600]) {
            s.store_scalar(301, ((p.p151) as f64).exp());
            s.store_mul_offset_ad_rhs(302, 301, A::div_scaled_inputs(s.ad_value(252), 1.0, s.ad_value(6), p.p85), (((-p.p151)) + (1.0)));
        }

        s.store_mul(242, 240, 302);

        s.store_ad_value(243, A::div_scaled_product(s.ad_value(96), s.ad_value(6), 4.0, s.ad_value(31), 1.0));

        s.store_mul_ad_affine_product_rhs(244, 243, s.ad_value(125), A::offset(A::add(s.ad_value(129), s.ad_value(116)), 2.0), 0.5, 0.0);

        s.b[601] = (p.p79 == 0.0);
        s.v[601] = if s.b[601] { 1.0 } else { 0.0 };

        if s.b[601] {
            s.store_ad_value(249, A::div_scaled_product(s.ad_value(97), A::add_scaled_products(s.ad_value(225), s.ad_value(171), 1.0, s.ad_value(243), s.ad_value(170), 1.0), 0.5, A::add(s.ad_value(95), s.ad_value(96)), 1.0));
        }

        s.b[602] = ((((s.v[255] - s.v[22]) / p.p91) * s.v[8]) < p.p151);
        s.v[602] = if s.b[602] { 1.0 } else { 0.0 };

        if ((!s.b[601]) && s.b[602]) {
            s.store_exp_ad(180, A::mul_scaled_lhs(A::sub(s.ad_value(255), s.ad_value(22)), 1.0 / (p.p91), s.ad_value(8)));
        }

        if ((!s.b[601]) && (!s.b[602])) {
            s.store_scalar(301, ((p.p151) as f64).exp());
            s.store_mul_offset_ad_rhs(180, 301, A::mul_scaled_lhs(A::sub(s.ad_value(255), s.ad_value(22)), 1.0 / (p.p91), s.ad_value(8)), (((-p.p151)) + (1.0)));
        }

        if (!s.b[601]) {
            s.store_ad_value(249, A::div_scaled_product3(s.ad_value(43), s.ad_value(98), s.ad_value(274), 2.0, A::offset(A::sqrt(A::scale_offset(s.ad_value(180), 4.0, 1.0)), 1.0), 1.0));
        }

        s.b[603] = (((p.p5 == 1.0) || (p.p5 == 3.0)) && (p.p33 > 0.0));
        s.v[603] = if s.b[603] { 1.0 } else { 0.0 };

        if s.b[603] {
            s.store_scale(249, 249, s.v[160]);
        }

        s.b[604] = (p.p79 == 0.0);
        s.v[604] = if s.b[604] { 1.0 } else { 0.0 };

        if (s.b[603] && s.b[604]) {
            s.store_mul(172, 149, 275);
            s.store_div_ad(173, A::sub(s.ad_value(172), s.ad_value(149)), A::offset(A::sqrt(A::offset(s.ad_value(172), 1.0)), 1.0));
            s.store_scale(245, 278, 4.0);
            s.store_div_ad_rhs(246, 245, A::offset(A::sqrt(A::offset(s.ad_value(245), 1.0)), 1.0));
            s.store_ad_value(247, A::div_scaled_product(s.ad_value(97), A::add_scaled_products(s.ad_value(225), s.ad_value(173), 1.0, s.ad_value(243), s.ad_value(246), 1.0), (0.5 * p.p33), A::add(s.ad_value(95), s.ad_value(96)), 1.0));
        }

        s.b[605] = (((s.v[267] - s.v[22]) * s.v[8]) < p.p151);
        s.v[605] = if s.b[605] { 1.0 } else { 0.0 };

        if ((s.b[603] && (!s.b[604])) && s.b[605]) {
            s.store_exp_ad(181, A::mul(A::sub(s.ad_value(267), s.ad_value(22)), s.ad_value(8)));
        }

        if ((s.b[603] && (!s.b[604])) && (!s.b[605])) {
            s.store_scalar(301, ((p.p151) as f64).exp());
            s.store_mul_offset_ad_rhs(181, 301, A::mul(A::sub(s.ad_value(267), s.ad_value(22)), s.ad_value(8)), (((-p.p151)) + (1.0)));
        }

        if (s.b[603] && (!s.b[604])) {
            s.store_ad_value(247, A::div_scaled_product3(s.ad_value(43), s.ad_value(98), s.ad_value(275), (2.0 * p.p33), A::offset(A::sqrt(A::scale_offset(s.ad_value(181), 4.0, 1.0)), 1.0), 1.0));
        }

        if s.b[603] {
            s.store_mul(248, 178, 247);
        }

        s.b[606] = (p.p6 == 1.0);
        s.v[606] = if s.b[606] { 1.0 } else { 0.0 };

        if s.b[606] {
            s.store_offset_powf_ad(193, A::sub_from_scalar(1.0, A::mul(s.ad_value(140), s.ad_value(65))), (-p.p67), (-3.0));
            s.store_div_ad_lhs(294, A::sub(s.ad_value(252), s.ad_value(139)), 299);
        }

        s.b[607] = (s.v[294] < 0.0);
        s.v[607] = if s.b[607] { 1.0 } else { 0.0 };

        if (s.b[606] && s.b[607]) {
            s.store_div_from_scalar_offset_ad(194, 1.0, A::exp(s.ad_value(294)), 1.0);
        }

        if (s.b[606] && (!s.b[607])) {
            s.store_div_ad(194, A::exp_scaled_input(s.ad_value(294), -1.0), A::offset(A::exp_scaled_input(s.ad_value(294), -1.0), 1.0));
        }

        if s.b[606] {
            s.store_offset_mul(192, 193, 194, 3.0);
            s.store_scaled_mul(195, 23, 192, (1.0 - p.p68));
            s.store_mul_ad(198, A::div_scaled_product3(s.ad_value(149), s.ad_value(272), s.ad_value(8), 1.0, s.ad_value(48), 1.0), A::div_from_scalar(0.5, A::sqrt(A::offset(s.ad_value(150), 1.0))));
            s.store_mul3_affine_lhs(196, 225, 187, 0.5, 0.0, 198);
            s.store_scaled_div(197, 242, 6, (1.0 / (p.p85)));
            s.store_mul_scaled_ad_rhs(228, 254, 0.2, A::add_scaled_inputs3(s.ad_value(195), 1.0, s.ad_value(196), 1.0, s.ad_value(197), 1.0));
            s.store_scale(241, 242, (1.0 - p.p95));
            s.store_add_scaled_inputs(337, 229, 1.0, 242, p.p95);
            s.store_add_scaled_inputs(227, 337, p.p94, 230, 1.0);
            s.store_scale(226, 337, (1.0 - p.p94));
        }

        if (!s.b[606]) {
            s.copy_ad(226, 229);
            s.copy_ad(227, 230);
            s.copy_ad(241, 242);
        }

        let assign6910_e7174: f64 = (p.p147 * (nv4 - 0.0));
        let assign6910_e7175_q: f64 = assign6910_e7174;
        let assign6910_e7177: f64 = (assign6910_e7174 * p.p1);
        let assign6910_e7177_q: f64 = (assign6910_e7175_q * p.p1);
        s.v[220] = assign6910_e7177;
        s.dn[220][4] = (p.p147 * p.p1);
        s.rv[220] = assign6910_e7177_q;
        s.rdn[220][4] = (p.p147 * p.p1);

        s.store_div_ad_lhs(333, A::add(s.ad_value(158), s.ad_value(157)), 156);

        s.b[615] = (s.v[333] > 0.0);
        s.v[615] = if s.b[615] { 1.0 } else { 0.0 };

        if s.b[615] {
            s.store_div_ad_lhs(335, A::add(s.ad_value(226), s.ad_value(227)), 333);
        }

        if (!s.b[615]) {
            s.store_mul3_lhs(335, 95, 187, 156);
        }

        s.b[616] = (p.p131 == 1.0);
        s.v[616] = if s.b[616] { 1.0 } else { 0.0 };

        if s.b[616] {
            s.store_scale(336, 335, p.p94);
        }

        s.b[617] = (p.p131 == 2.0);
        s.v[617] = if s.b[617] { 1.0 } else { 0.0 };

        if ((!s.b[616]) && s.b[617]) {
            s.store_scale(336, 335, p.p132);
        }

        if ((!s.b[616]) && (!s.b[617])) {
            s.store_scalar(336, 0.0);
        }

    }

    pub(super) fn stamp_transient_equations_block_0(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        multiplicity: f64,
    ) {
        let eq0_e167: f64 = (p.p3 * s.v[118]);
        let eq0_e167_d_n0: f64 = (p.p3 * s.dn[118][0]);
        let eq0_e167_d_n1: f64 = (p.p3 * s.dn[118][1]);
        let eq0_e167_d_n2: f64 = (p.p3 * s.dn[118][2]);
        let eq0_e167_d_n3: f64 = (p.p3 * s.dn[118][3]);
        let eq0_e167_d_n4: f64 = (p.p3 * s.dn[118][4]);
        let eq0_e167_d_n5: f64 = (p.p3 * s.dn[118][5]);
        let eq0_e167_d_n6: f64 = (p.p3 * s.dn[118][6]);
        let eq0_e167_d_n7: f64 = (p.p3 * s.dn[118][7]);
        let eq0_e167_d_n8: f64 = (p.p3 * s.dn[118][8]);
        let eq0_e167_d_n9: f64 = (p.p3 * s.dn[118][9]);
        let eq0_e167_d_n10: f64 = (p.p3 * s.dn[118][10]);
        let eq0_e167_d_n11: f64 = (p.p3 * s.dn[118][11]);
        let eq0_e167_d_n12: f64 = (p.p3 * s.dn[118][12]);
        let eq0_e167_d_b0: f64 = (p.p3 * s.db[118][0]);
        let eq0_e167_d_b1: f64 = (p.p3 * s.db[118][1]);
        let eq0_e169: f64 = (eq0_e167 * p.p1);
        let eq0_e169_d_n0: f64 = (eq0_e167_d_n0 * p.p1);
        let eq0_e169_d_n1: f64 = (eq0_e167_d_n1 * p.p1);
        let eq0_e169_d_n2: f64 = (eq0_e167_d_n2 * p.p1);
        let eq0_e169_d_n3: f64 = (eq0_e167_d_n3 * p.p1);
        let eq0_e169_d_n4: f64 = (eq0_e167_d_n4 * p.p1);
        let eq0_e169_d_n5: f64 = (eq0_e167_d_n5 * p.p1);
        let eq0_e169_d_n6: f64 = (eq0_e167_d_n6 * p.p1);
        let eq0_e169_d_n7: f64 = (eq0_e167_d_n7 * p.p1);
        let eq0_e169_d_n8: f64 = (eq0_e167_d_n8 * p.p1);
        let eq0_e169_d_n9: f64 = (eq0_e167_d_n9 * p.p1);
        let eq0_e169_d_n10: f64 = (eq0_e167_d_n10 * p.p1);
        let eq0_e169_d_n11: f64 = (eq0_e167_d_n11 * p.p1);
        let eq0_e169_d_n12: f64 = (eq0_e167_d_n12 * p.p1);
        let eq0_e169_d_b0: f64 = (eq0_e167_d_b0 * p.p1);
        let eq0_e169_d_b1: f64 = (eq0_e167_d_b1 * p.p1);
        let eq0_value: f64 = eq0_e169;
        let eq0_node_derivatives: [f64; 13] = [eq0_e169_d_n0, eq0_e169_d_n1, eq0_e169_d_n2, eq0_e169_d_n3, eq0_e169_d_n4, eq0_e169_d_n5, eq0_e169_d_n6, eq0_e169_d_n7, eq0_e169_d_n8, eq0_e169_d_n9, eq0_e169_d_n10, eq0_e169_d_n11, eq0_e169_d_n12];
        let eq0_branch_derivatives: [f64; 2] = [eq0_e169_d_b0, eq0_e169_d_b1];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(9),
            multiplicity * (eq0_value),
            &eq0_node_derivatives,
            &eq0_branch_derivatives,
            multiplicity,
        );
        let eq1_e172: f64 = (p.p3 * s.v[159]);
        let eq1_e172_d_n0: f64 = (p.p3 * s.dn[159][0]);
        let eq1_e172_d_n1: f64 = (p.p3 * s.dn[159][1]);
        let eq1_e172_d_n2: f64 = (p.p3 * s.dn[159][2]);
        let eq1_e172_d_n3: f64 = (p.p3 * s.dn[159][3]);
        let eq1_e172_d_n4: f64 = (p.p3 * s.dn[159][4]);
        let eq1_e172_d_n5: f64 = (p.p3 * s.dn[159][5]);
        let eq1_e172_d_n6: f64 = (p.p3 * s.dn[159][6]);
        let eq1_e172_d_n7: f64 = (p.p3 * s.dn[159][7]);
        let eq1_e172_d_n8: f64 = (p.p3 * s.dn[159][8]);
        let eq1_e172_d_n9: f64 = (p.p3 * s.dn[159][9]);
        let eq1_e172_d_n10: f64 = (p.p3 * s.dn[159][10]);
        let eq1_e172_d_n11: f64 = (p.p3 * s.dn[159][11]);
        let eq1_e172_d_n12: f64 = (p.p3 * s.dn[159][12]);
        let eq1_e172_d_b0: f64 = (p.p3 * s.db[159][0]);
        let eq1_e172_d_b1: f64 = (p.p3 * s.db[159][1]);
        let eq1_e174: f64 = (eq1_e172 * p.p1);
        let eq1_e174_d_n0: f64 = (eq1_e172_d_n0 * p.p1);
        let eq1_e174_d_n1: f64 = (eq1_e172_d_n1 * p.p1);
        let eq1_e174_d_n2: f64 = (eq1_e172_d_n2 * p.p1);
        let eq1_e174_d_n3: f64 = (eq1_e172_d_n3 * p.p1);
        let eq1_e174_d_n4: f64 = (eq1_e172_d_n4 * p.p1);
        let eq1_e174_d_n5: f64 = (eq1_e172_d_n5 * p.p1);
        let eq1_e174_d_n6: f64 = (eq1_e172_d_n6 * p.p1);
        let eq1_e174_d_n7: f64 = (eq1_e172_d_n7 * p.p1);
        let eq1_e174_d_n8: f64 = (eq1_e172_d_n8 * p.p1);
        let eq1_e174_d_n9: f64 = (eq1_e172_d_n9 * p.p1);
        let eq1_e174_d_n10: f64 = (eq1_e172_d_n10 * p.p1);
        let eq1_e174_d_n11: f64 = (eq1_e172_d_n11 * p.p1);
        let eq1_e174_d_n12: f64 = (eq1_e172_d_n12 * p.p1);
        let eq1_e174_d_b0: f64 = (eq1_e172_d_b0 * p.p1);
        let eq1_e174_d_b1: f64 = (eq1_e172_d_b1 * p.p1);
        let eq1_value: f64 = eq1_e174;
        let eq1_node_derivatives: [f64; 13] = [eq1_e174_d_n0, eq1_e174_d_n1, eq1_e174_d_n2, eq1_e174_d_n3, eq1_e174_d_n4, eq1_e174_d_n5, eq1_e174_d_n6, eq1_e174_d_n7, eq1_e174_d_n8, eq1_e174_d_n9, eq1_e174_d_n10, eq1_e174_d_n11, eq1_e174_d_n12];
        let eq1_branch_derivatives: [f64; 2] = [eq1_e174_d_b0, eq1_e174_d_b1];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(5),
            multiplicity * (eq1_value),
            &eq1_node_derivatives,
            &eq1_branch_derivatives,
            multiplicity,
        );
        let eq2_e178: f64 = (s.v[162] + s.v[165]);
        let eq2_e178_d_n0: f64 = (s.dn[162][0] + s.dn[165][0]);
        let eq2_e178_d_n1: f64 = (s.dn[162][1] + s.dn[165][1]);
        let eq2_e178_d_n2: f64 = (s.dn[162][2] + s.dn[165][2]);
        let eq2_e178_d_n3: f64 = (s.dn[162][3] + s.dn[165][3]);
        let eq2_e178_d_n4: f64 = (s.dn[162][4] + s.dn[165][4]);
        let eq2_e178_d_n5: f64 = (s.dn[162][5] + s.dn[165][5]);
        let eq2_e178_d_n6: f64 = (s.dn[162][6] + s.dn[165][6]);
        let eq2_e178_d_n7: f64 = (s.dn[162][7] + s.dn[165][7]);
        let eq2_e178_d_n8: f64 = (s.dn[162][8] + s.dn[165][8]);
        let eq2_e178_d_n9: f64 = (s.dn[162][9] + s.dn[165][9]);
        let eq2_e178_d_n10: f64 = (s.dn[162][10] + s.dn[165][10]);
        let eq2_e178_d_n11: f64 = (s.dn[162][11] + s.dn[165][11]);
        let eq2_e178_d_n12: f64 = (s.dn[162][12] + s.dn[165][12]);
        let eq2_e178_d_b0: f64 = (s.db[162][0] + s.db[165][0]);
        let eq2_e178_d_b1: f64 = (s.db[162][1] + s.db[165][1]);
        let eq2_e180: f64 = (eq2_e178 + s.v[166]);
        let eq2_e180_d_n0: f64 = (eq2_e178_d_n0 + s.dn[166][0]);
        let eq2_e180_d_n1: f64 = (eq2_e178_d_n1 + s.dn[166][1]);
        let eq2_e180_d_n2: f64 = (eq2_e178_d_n2 + s.dn[166][2]);
        let eq2_e180_d_n3: f64 = (eq2_e178_d_n3 + s.dn[166][3]);
        let eq2_e180_d_n4: f64 = (eq2_e178_d_n4 + s.dn[166][4]);
        let eq2_e180_d_n5: f64 = (eq2_e178_d_n5 + s.dn[166][5]);
        let eq2_e180_d_n6: f64 = (eq2_e178_d_n6 + s.dn[166][6]);
        let eq2_e180_d_n7: f64 = (eq2_e178_d_n7 + s.dn[166][7]);
        let eq2_e180_d_n8: f64 = (eq2_e178_d_n8 + s.dn[166][8]);
        let eq2_e180_d_n9: f64 = (eq2_e178_d_n9 + s.dn[166][9]);
        let eq2_e180_d_n10: f64 = (eq2_e178_d_n10 + s.dn[166][10]);
        let eq2_e180_d_n11: f64 = (eq2_e178_d_n11 + s.dn[166][11]);
        let eq2_e180_d_n12: f64 = (eq2_e178_d_n12 + s.dn[166][12]);
        let eq2_e180_d_b0: f64 = (eq2_e178_d_b0 + s.db[166][0]);
        let eq2_e180_d_b1: f64 = (eq2_e178_d_b1 + s.db[166][1]);
        let eq2_e181: f64 = (p.p3 * eq2_e180);
        let eq2_e181_d_n0: f64 = (p.p3 * eq2_e180_d_n0);
        let eq2_e181_d_n1: f64 = (p.p3 * eq2_e180_d_n1);
        let eq2_e181_d_n2: f64 = (p.p3 * eq2_e180_d_n2);
        let eq2_e181_d_n3: f64 = (p.p3 * eq2_e180_d_n3);
        let eq2_e181_d_n4: f64 = (p.p3 * eq2_e180_d_n4);
        let eq2_e181_d_n5: f64 = (p.p3 * eq2_e180_d_n5);
        let eq2_e181_d_n6: f64 = (p.p3 * eq2_e180_d_n6);
        let eq2_e181_d_n7: f64 = (p.p3 * eq2_e180_d_n7);
        let eq2_e181_d_n8: f64 = (p.p3 * eq2_e180_d_n8);
        let eq2_e181_d_n9: f64 = (p.p3 * eq2_e180_d_n9);
        let eq2_e181_d_n10: f64 = (p.p3 * eq2_e180_d_n10);
        let eq2_e181_d_n11: f64 = (p.p3 * eq2_e180_d_n11);
        let eq2_e181_d_n12: f64 = (p.p3 * eq2_e180_d_n12);
        let eq2_e181_d_b0: f64 = (p.p3 * eq2_e180_d_b0);
        let eq2_e181_d_b1: f64 = (p.p3 * eq2_e180_d_b1);
        let eq2_e183: f64 = (eq2_e181 * p.p1);
        let eq2_e183_d_n0: f64 = (eq2_e181_d_n0 * p.p1);
        let eq2_e183_d_n1: f64 = (eq2_e181_d_n1 * p.p1);
        let eq2_e183_d_n2: f64 = (eq2_e181_d_n2 * p.p1);
        let eq2_e183_d_n3: f64 = (eq2_e181_d_n3 * p.p1);
        let eq2_e183_d_n4: f64 = (eq2_e181_d_n4 * p.p1);
        let eq2_e183_d_n5: f64 = (eq2_e181_d_n5 * p.p1);
        let eq2_e183_d_n6: f64 = (eq2_e181_d_n6 * p.p1);
        let eq2_e183_d_n7: f64 = (eq2_e181_d_n7 * p.p1);
        let eq2_e183_d_n8: f64 = (eq2_e181_d_n8 * p.p1);
        let eq2_e183_d_n9: f64 = (eq2_e181_d_n9 * p.p1);
        let eq2_e183_d_n10: f64 = (eq2_e181_d_n10 * p.p1);
        let eq2_e183_d_n11: f64 = (eq2_e181_d_n11 * p.p1);
        let eq2_e183_d_n12: f64 = (eq2_e181_d_n12 * p.p1);
        let eq2_e183_d_b0: f64 = (eq2_e181_d_b0 * p.p1);
        let eq2_e183_d_b1: f64 = (eq2_e181_d_b1 * p.p1);
        let eq2_value: f64 = eq2_e183;
        let eq2_node_derivatives: [f64; 13] = [eq2_e183_d_n0, eq2_e183_d_n1, eq2_e183_d_n2, eq2_e183_d_n3, eq2_e183_d_n4, eq2_e183_d_n5, eq2_e183_d_n6, eq2_e183_d_n7, eq2_e183_d_n8, eq2_e183_d_n9, eq2_e183_d_n10, eq2_e183_d_n11, eq2_e183_d_n12];
        let eq2_branch_derivatives: [f64; 2] = [eq2_e183_d_b0, eq2_e183_d_b1];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(5),
            multiplicity * (eq2_value),
            &eq2_node_derivatives,
            &eq2_branch_derivatives,
            multiplicity,
        );
        let eq3_e187: f64 = (s.v[161] + s.v[163]);
        let eq3_e187_d_n0: f64 = (s.dn[161][0] + s.dn[163][0]);
        let eq3_e187_d_n1: f64 = (s.dn[161][1] + s.dn[163][1]);
        let eq3_e187_d_n2: f64 = (s.dn[161][2] + s.dn[163][2]);
        let eq3_e187_d_n3: f64 = (s.dn[161][3] + s.dn[163][3]);
        let eq3_e187_d_n4: f64 = (s.dn[161][4] + s.dn[163][4]);
        let eq3_e187_d_n5: f64 = (s.dn[161][5] + s.dn[163][5]);
        let eq3_e187_d_n6: f64 = (s.dn[161][6] + s.dn[163][6]);
        let eq3_e187_d_n7: f64 = (s.dn[161][7] + s.dn[163][7]);
        let eq3_e187_d_n8: f64 = (s.dn[161][8] + s.dn[163][8]);
        let eq3_e187_d_n9: f64 = (s.dn[161][9] + s.dn[163][9]);
        let eq3_e187_d_n10: f64 = (s.dn[161][10] + s.dn[163][10]);
        let eq3_e187_d_n11: f64 = (s.dn[161][11] + s.dn[163][11]);
        let eq3_e187_d_n12: f64 = (s.dn[161][12] + s.dn[163][12]);
        let eq3_e187_d_b0: f64 = (s.db[161][0] + s.db[163][0]);
        let eq3_e187_d_b1: f64 = (s.db[161][1] + s.db[163][1]);
        let eq3_e190: f64 = (s.v[344] * s.v[252]);
        let eq3_e190_d_n0: f64 = (s.v[344] * s.dn[252][0]);
        let eq3_e190_d_n1: f64 = (s.v[344] * s.dn[252][1]);
        let eq3_e190_d_n2: f64 = (s.v[344] * s.dn[252][2]);
        let eq3_e190_d_n3: f64 = (s.v[344] * s.dn[252][3]);
        let eq3_e190_d_n4: f64 = (s.v[344] * s.dn[252][4]);
        let eq3_e190_d_n5: f64 = (s.v[344] * s.dn[252][5]);
        let eq3_e190_d_n6: f64 = (s.v[344] * s.dn[252][6]);
        let eq3_e190_d_n7: f64 = (s.v[344] * s.dn[252][7]);
        let eq3_e190_d_n8: f64 = (s.v[344] * s.dn[252][8]);
        let eq3_e190_d_n9: f64 = (s.v[344] * s.dn[252][9]);
        let eq3_e190_d_n10: f64 = (s.v[344] * s.dn[252][10]);
        let eq3_e190_d_n11: f64 = (s.v[344] * s.dn[252][11]);
        let eq3_e190_d_n12: f64 = (s.v[344] * s.dn[252][12]);
        let eq3_e190_d_b0: f64 = (s.v[344] * s.db[252][0]);
        let eq3_e190_d_b1: f64 = (s.v[344] * s.db[252][1]);
        let eq3_e191: f64 = (eq3_e187 + eq3_e190);
        let eq3_e191_d_n0: f64 = (eq3_e187_d_n0 + eq3_e190_d_n0);
        let eq3_e191_d_n1: f64 = (eq3_e187_d_n1 + eq3_e190_d_n1);
        let eq3_e191_d_n2: f64 = (eq3_e187_d_n2 + eq3_e190_d_n2);
        let eq3_e191_d_n3: f64 = (eq3_e187_d_n3 + eq3_e190_d_n3);
        let eq3_e191_d_n4: f64 = (eq3_e187_d_n4 + eq3_e190_d_n4);
        let eq3_e191_d_n5: f64 = (eq3_e187_d_n5 + eq3_e190_d_n5);
        let eq3_e191_d_n6: f64 = (eq3_e187_d_n6 + eq3_e190_d_n6);
        let eq3_e191_d_n7: f64 = (eq3_e187_d_n7 + eq3_e190_d_n7);
        let eq3_e191_d_n8: f64 = (eq3_e187_d_n8 + eq3_e190_d_n8);
        let eq3_e191_d_n9: f64 = (eq3_e187_d_n9 + eq3_e190_d_n9);
        let eq3_e191_d_n10: f64 = (eq3_e187_d_n10 + eq3_e190_d_n10);
        let eq3_e191_d_n11: f64 = (eq3_e187_d_n11 + eq3_e190_d_n11);
        let eq3_e191_d_n12: f64 = (eq3_e187_d_n12 + eq3_e190_d_n12);
        let eq3_e191_d_b0: f64 = (eq3_e187_d_b0 + eq3_e190_d_b0);
        let eq3_e191_d_b1: f64 = (eq3_e187_d_b1 + eq3_e190_d_b1);
        let eq3_e193: f64 = (eq3_e191 - s.v[57]);
        let eq3_e193_d_n0: f64 = (eq3_e191_d_n0 - s.dn[57][0]);
        let eq3_e193_d_n1: f64 = (eq3_e191_d_n1 - s.dn[57][1]);
        let eq3_e193_d_n2: f64 = (eq3_e191_d_n2 - s.dn[57][2]);
        let eq3_e193_d_n3: f64 = (eq3_e191_d_n3 - s.dn[57][3]);
        let eq3_e193_d_n4: f64 = (eq3_e191_d_n4 - s.dn[57][4]);
        let eq3_e193_d_n5: f64 = (eq3_e191_d_n5 - s.dn[57][5]);
        let eq3_e193_d_n6: f64 = (eq3_e191_d_n6 - s.dn[57][6]);
        let eq3_e193_d_n7: f64 = (eq3_e191_d_n7 - s.dn[57][7]);
        let eq3_e193_d_n8: f64 = (eq3_e191_d_n8 - s.dn[57][8]);
        let eq3_e193_d_n9: f64 = (eq3_e191_d_n9 - s.dn[57][9]);
        let eq3_e193_d_n10: f64 = (eq3_e191_d_n10 - s.dn[57][10]);
        let eq3_e193_d_n11: f64 = (eq3_e191_d_n11 - s.dn[57][11]);
        let eq3_e193_d_n12: f64 = (eq3_e191_d_n12 - s.dn[57][12]);
        let eq3_e193_d_b0: f64 = (eq3_e191_d_b0 - s.db[57][0]);
        let eq3_e193_d_b1: f64 = (eq3_e191_d_b1 - s.db[57][1]);
        let eq3_e195: f64 = (eq3_e193 + s.v[359]);
        let eq3_e195_d_n0: f64 = (eq3_e193_d_n0 + s.dn[359][0]);
        let eq3_e195_d_n1: f64 = (eq3_e193_d_n1 + s.dn[359][1]);
        let eq3_e195_d_n2: f64 = (eq3_e193_d_n2 + s.dn[359][2]);
        let eq3_e195_d_n3: f64 = (eq3_e193_d_n3 + s.dn[359][3]);
        let eq3_e195_d_n4: f64 = (eq3_e193_d_n4 + s.dn[359][4]);
        let eq3_e195_d_n5: f64 = (eq3_e193_d_n5 + s.dn[359][5]);
        let eq3_e195_d_n6: f64 = (eq3_e193_d_n6 + s.dn[359][6]);
        let eq3_e195_d_n7: f64 = (eq3_e193_d_n7 + s.dn[359][7]);
        let eq3_e195_d_n8: f64 = (eq3_e193_d_n8 + s.dn[359][8]);
        let eq3_e195_d_n9: f64 = (eq3_e193_d_n9 + s.dn[359][9]);
        let eq3_e195_d_n10: f64 = (eq3_e193_d_n10 + s.dn[359][10]);
        let eq3_e195_d_n11: f64 = (eq3_e193_d_n11 + s.dn[359][11]);
        let eq3_e195_d_n12: f64 = (eq3_e193_d_n12 + s.dn[359][12]);
        let eq3_e195_d_b0: f64 = (eq3_e193_d_b0 + s.db[359][0]);
        let eq3_e195_d_b1: f64 = (eq3_e193_d_b1 + s.db[359][1]);
        let eq3_e197: f64 = (eq3_e195 + s.v[358]);
        let eq3_e197_d_n0: f64 = (eq3_e195_d_n0 + s.dn[358][0]);
        let eq3_e197_d_n1: f64 = (eq3_e195_d_n1 + s.dn[358][1]);
        let eq3_e197_d_n2: f64 = (eq3_e195_d_n2 + s.dn[358][2]);
        let eq3_e197_d_n3: f64 = (eq3_e195_d_n3 + s.dn[358][3]);
        let eq3_e197_d_n4: f64 = (eq3_e195_d_n4 + s.dn[358][4]);
        let eq3_e197_d_n5: f64 = (eq3_e195_d_n5 + s.dn[358][5]);
        let eq3_e197_d_n6: f64 = (eq3_e195_d_n6 + s.dn[358][6]);
        let eq3_e197_d_n7: f64 = (eq3_e195_d_n7 + s.dn[358][7]);
        let eq3_e197_d_n8: f64 = (eq3_e195_d_n8 + s.dn[358][8]);
        let eq3_e197_d_n9: f64 = (eq3_e195_d_n9 + s.dn[358][9]);
        let eq3_e197_d_n10: f64 = (eq3_e195_d_n10 + s.dn[358][10]);
        let eq3_e197_d_n11: f64 = (eq3_e195_d_n11 + s.dn[358][11]);
        let eq3_e197_d_n12: f64 = (eq3_e195_d_n12 + s.dn[358][12]);
        let eq3_e197_d_b0: f64 = (eq3_e195_d_b0 + s.db[358][0]);
        let eq3_e197_d_b1: f64 = (eq3_e195_d_b1 + s.db[358][1]);
        let eq3_e198: f64 = (p.p3 * eq3_e197);
        let eq3_e198_d_n0: f64 = (p.p3 * eq3_e197_d_n0);
        let eq3_e198_d_n1: f64 = (p.p3 * eq3_e197_d_n1);
        let eq3_e198_d_n2: f64 = (p.p3 * eq3_e197_d_n2);
        let eq3_e198_d_n3: f64 = (p.p3 * eq3_e197_d_n3);
        let eq3_e198_d_n4: f64 = (p.p3 * eq3_e197_d_n4);
        let eq3_e198_d_n5: f64 = (p.p3 * eq3_e197_d_n5);
        let eq3_e198_d_n6: f64 = (p.p3 * eq3_e197_d_n6);
        let eq3_e198_d_n7: f64 = (p.p3 * eq3_e197_d_n7);
        let eq3_e198_d_n8: f64 = (p.p3 * eq3_e197_d_n8);
        let eq3_e198_d_n9: f64 = (p.p3 * eq3_e197_d_n9);
        let eq3_e198_d_n10: f64 = (p.p3 * eq3_e197_d_n10);
        let eq3_e198_d_n11: f64 = (p.p3 * eq3_e197_d_n11);
        let eq3_e198_d_n12: f64 = (p.p3 * eq3_e197_d_n12);
        let eq3_e198_d_b0: f64 = (p.p3 * eq3_e197_d_b0);
        let eq3_e198_d_b1: f64 = (p.p3 * eq3_e197_d_b1);
        let eq3_e200: f64 = (eq3_e198 * p.p1);
        let eq3_e200_d_n0: f64 = (eq3_e198_d_n0 * p.p1);
        let eq3_e200_d_n1: f64 = (eq3_e198_d_n1 * p.p1);
        let eq3_e200_d_n2: f64 = (eq3_e198_d_n2 * p.p1);
        let eq3_e200_d_n3: f64 = (eq3_e198_d_n3 * p.p1);
        let eq3_e200_d_n4: f64 = (eq3_e198_d_n4 * p.p1);
        let eq3_e200_d_n5: f64 = (eq3_e198_d_n5 * p.p1);
        let eq3_e200_d_n6: f64 = (eq3_e198_d_n6 * p.p1);
        let eq3_e200_d_n7: f64 = (eq3_e198_d_n7 * p.p1);
        let eq3_e200_d_n8: f64 = (eq3_e198_d_n8 * p.p1);
        let eq3_e200_d_n9: f64 = (eq3_e198_d_n9 * p.p1);
        let eq3_e200_d_n10: f64 = (eq3_e198_d_n10 * p.p1);
        let eq3_e200_d_n11: f64 = (eq3_e198_d_n11 * p.p1);
        let eq3_e200_d_n12: f64 = (eq3_e198_d_n12 * p.p1);
        let eq3_e200_d_b0: f64 = (eq3_e198_d_b0 * p.p1);
        let eq3_e200_d_b1: f64 = (eq3_e198_d_b1 * p.p1);
        let eq3_value: f64 = eq3_e200;
        let eq3_node_derivatives: [f64; 13] = [eq3_e200_d_n0, eq3_e200_d_n1, eq3_e200_d_n2, eq3_e200_d_n3, eq3_e200_d_n4, eq3_e200_d_n5, eq3_e200_d_n6, eq3_e200_d_n7, eq3_e200_d_n8, eq3_e200_d_n9, eq3_e200_d_n10, eq3_e200_d_n11, eq3_e200_d_n12];
        let eq3_branch_derivatives: [f64; 2] = [eq3_e200_d_b0, eq3_e200_d_b1];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(5),
            multiplicity * (eq3_value),
            &eq3_node_derivatives,
            &eq3_branch_derivatives,
            multiplicity,
        );
        let (eq4_e209, eq4_e209_d_n0, eq4_e209_d_n1, eq4_e209_d_n2, eq4_e209_d_n3, eq4_e209_d_n4, eq4_e209_d_n5, eq4_e209_d_n6, eq4_e209_d_n7, eq4_e209_d_n8, eq4_e209_d_n9, eq4_e209_d_n10, eq4_e209_d_n11, eq4_e209_d_n12, eq4_e209_d_b0, eq4_e209_d_b1,) = {
    if s.b[608] {
        let eq4_e204: f64 = (-s.v[82]);
        let eq4_e204_d_n0: f64 = (-s.dn[82][0]);
        let eq4_e204_d_n1: f64 = (-s.dn[82][1]);
        let eq4_e204_d_n2: f64 = (-s.dn[82][2]);
        let eq4_e204_d_n3: f64 = (-s.dn[82][3]);
        let eq4_e204_d_n4: f64 = (-s.dn[82][4]);
        let eq4_e204_d_n5: f64 = (-s.dn[82][5]);
        let eq4_e204_d_n6: f64 = (-s.dn[82][6]);
        let eq4_e204_d_n7: f64 = (-s.dn[82][7]);
        let eq4_e204_d_n8: f64 = (-s.dn[82][8]);
        let eq4_e204_d_n9: f64 = (-s.dn[82][9]);
        let eq4_e204_d_n10: f64 = (-s.dn[82][10]);
        let eq4_e204_d_n11: f64 = (-s.dn[82][11]);
        let eq4_e204_d_n12: f64 = (-s.dn[82][12]);
        let eq4_e204_d_b0: f64 = (-s.db[82][0]);
        let eq4_e204_d_b1: f64 = (-s.db[82][1]);
        let eq4_e205: f64 = (p.p3 * eq4_e204);
        let eq4_e205_d_n0: f64 = (p.p3 * eq4_e204_d_n0);
        let eq4_e205_d_n1: f64 = (p.p3 * eq4_e204_d_n1);
        let eq4_e205_d_n2: f64 = (p.p3 * eq4_e204_d_n2);
        let eq4_e205_d_n3: f64 = (p.p3 * eq4_e204_d_n3);
        let eq4_e205_d_n4: f64 = (p.p3 * eq4_e204_d_n4);
        let eq4_e205_d_n5: f64 = (p.p3 * eq4_e204_d_n5);
        let eq4_e205_d_n6: f64 = (p.p3 * eq4_e204_d_n6);
        let eq4_e205_d_n7: f64 = (p.p3 * eq4_e204_d_n7);
        let eq4_e205_d_n8: f64 = (p.p3 * eq4_e204_d_n8);
        let eq4_e205_d_n9: f64 = (p.p3 * eq4_e204_d_n9);
        let eq4_e205_d_n10: f64 = (p.p3 * eq4_e204_d_n10);
        let eq4_e205_d_n11: f64 = (p.p3 * eq4_e204_d_n11);
        let eq4_e205_d_n12: f64 = (p.p3 * eq4_e204_d_n12);
        let eq4_e205_d_b0: f64 = (p.p3 * eq4_e204_d_b0);
        let eq4_e205_d_b1: f64 = (p.p3 * eq4_e204_d_b1);
        let eq4_e207: f64 = (eq4_e205 * p.p1);
        let eq4_e207_d_n0: f64 = (eq4_e205_d_n0 * p.p1);
        let eq4_e207_d_n1: f64 = (eq4_e205_d_n1 * p.p1);
        let eq4_e207_d_n2: f64 = (eq4_e205_d_n2 * p.p1);
        let eq4_e207_d_n3: f64 = (eq4_e205_d_n3 * p.p1);
        let eq4_e207_d_n4: f64 = (eq4_e205_d_n4 * p.p1);
        let eq4_e207_d_n5: f64 = (eq4_e205_d_n5 * p.p1);
        let eq4_e207_d_n6: f64 = (eq4_e205_d_n6 * p.p1);
        let eq4_e207_d_n7: f64 = (eq4_e205_d_n7 * p.p1);
        let eq4_e207_d_n8: f64 = (eq4_e205_d_n8 * p.p1);
        let eq4_e207_d_n9: f64 = (eq4_e205_d_n9 * p.p1);
        let eq4_e207_d_n10: f64 = (eq4_e205_d_n10 * p.p1);
        let eq4_e207_d_n11: f64 = (eq4_e205_d_n11 * p.p1);
        let eq4_e207_d_n12: f64 = (eq4_e205_d_n12 * p.p1);
        let eq4_e207_d_b0: f64 = (eq4_e205_d_b0 * p.p1);
        let eq4_e207_d_b1: f64 = (eq4_e205_d_b1 * p.p1);
        (eq4_e207, eq4_e207_d_n0, eq4_e207_d_n1, eq4_e207_d_n2, eq4_e207_d_n3, eq4_e207_d_n4, eq4_e207_d_n5, eq4_e207_d_n6, eq4_e207_d_n7, eq4_e207_d_n8, eq4_e207_d_n9, eq4_e207_d_n10, eq4_e207_d_n11, eq4_e207_d_n12, eq4_e207_d_b0, eq4_e207_d_b1,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq4_value: f64 = eq4_e209;
        let eq4_node_derivatives: [f64; 13] = [eq4_e209_d_n0, eq4_e209_d_n1, eq4_e209_d_n2, eq4_e209_d_n3, eq4_e209_d_n4, eq4_e209_d_n5, eq4_e209_d_n6, eq4_e209_d_n7, eq4_e209_d_n8, eq4_e209_d_n9, eq4_e209_d_n10, eq4_e209_d_n11, eq4_e209_d_n12];
        let eq4_branch_derivatives: [f64; 2] = [eq4_e209_d_b0, eq4_e209_d_b1];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(8),
            multiplicity * (eq4_value),
            &eq4_node_derivatives,
            &eq4_branch_derivatives,
            multiplicity,
        );
        let (eq5_e219, eq5_e219_d_n0, eq5_e219_d_n1, eq5_e219_d_n2, eq5_e219_d_n3, eq5_e219_d_n4, eq5_e219_d_n5, eq5_e219_d_n6, eq5_e219_d_n7, eq5_e219_d_n8, eq5_e219_d_n9, eq5_e219_d_n10, eq5_e219_d_n11, eq5_e219_d_n12, eq5_e219_d_b0, eq5_e219_d_b1,) = {
    if (!s.b[608]) {
        let eq5_e214: f64 = (-s.v[82]);
        let eq5_e214_d_n0: f64 = (-s.dn[82][0]);
        let eq5_e214_d_n1: f64 = (-s.dn[82][1]);
        let eq5_e214_d_n2: f64 = (-s.dn[82][2]);
        let eq5_e214_d_n3: f64 = (-s.dn[82][3]);
        let eq5_e214_d_n4: f64 = (-s.dn[82][4]);
        let eq5_e214_d_n5: f64 = (-s.dn[82][5]);
        let eq5_e214_d_n6: f64 = (-s.dn[82][6]);
        let eq5_e214_d_n7: f64 = (-s.dn[82][7]);
        let eq5_e214_d_n8: f64 = (-s.dn[82][8]);
        let eq5_e214_d_n9: f64 = (-s.dn[82][9]);
        let eq5_e214_d_n10: f64 = (-s.dn[82][10]);
        let eq5_e214_d_n11: f64 = (-s.dn[82][11]);
        let eq5_e214_d_n12: f64 = (-s.dn[82][12]);
        let eq5_e214_d_b0: f64 = (-s.db[82][0]);
        let eq5_e214_d_b1: f64 = (-s.db[82][1]);
        let eq5_e215: f64 = (p.p3 * eq5_e214);
        let eq5_e215_d_n0: f64 = (p.p3 * eq5_e214_d_n0);
        let eq5_e215_d_n1: f64 = (p.p3 * eq5_e214_d_n1);
        let eq5_e215_d_n2: f64 = (p.p3 * eq5_e214_d_n2);
        let eq5_e215_d_n3: f64 = (p.p3 * eq5_e214_d_n3);
        let eq5_e215_d_n4: f64 = (p.p3 * eq5_e214_d_n4);
        let eq5_e215_d_n5: f64 = (p.p3 * eq5_e214_d_n5);
        let eq5_e215_d_n6: f64 = (p.p3 * eq5_e214_d_n6);
        let eq5_e215_d_n7: f64 = (p.p3 * eq5_e214_d_n7);
        let eq5_e215_d_n8: f64 = (p.p3 * eq5_e214_d_n8);
        let eq5_e215_d_n9: f64 = (p.p3 * eq5_e214_d_n9);
        let eq5_e215_d_n10: f64 = (p.p3 * eq5_e214_d_n10);
        let eq5_e215_d_n11: f64 = (p.p3 * eq5_e214_d_n11);
        let eq5_e215_d_n12: f64 = (p.p3 * eq5_e214_d_n12);
        let eq5_e215_d_b0: f64 = (p.p3 * eq5_e214_d_b0);
        let eq5_e215_d_b1: f64 = (p.p3 * eq5_e214_d_b1);
        let eq5_e217: f64 = (eq5_e215 * p.p1);
        let eq5_e217_d_n0: f64 = (eq5_e215_d_n0 * p.p1);
        let eq5_e217_d_n1: f64 = (eq5_e215_d_n1 * p.p1);
        let eq5_e217_d_n2: f64 = (eq5_e215_d_n2 * p.p1);
        let eq5_e217_d_n3: f64 = (eq5_e215_d_n3 * p.p1);
        let eq5_e217_d_n4: f64 = (eq5_e215_d_n4 * p.p1);
        let eq5_e217_d_n5: f64 = (eq5_e215_d_n5 * p.p1);
        let eq5_e217_d_n6: f64 = (eq5_e215_d_n6 * p.p1);
        let eq5_e217_d_n7: f64 = (eq5_e215_d_n7 * p.p1);
        let eq5_e217_d_n8: f64 = (eq5_e215_d_n8 * p.p1);
        let eq5_e217_d_n9: f64 = (eq5_e215_d_n9 * p.p1);
        let eq5_e217_d_n10: f64 = (eq5_e215_d_n10 * p.p1);
        let eq5_e217_d_n11: f64 = (eq5_e215_d_n11 * p.p1);
        let eq5_e217_d_n12: f64 = (eq5_e215_d_n12 * p.p1);
        let eq5_e217_d_b0: f64 = (eq5_e215_d_b0 * p.p1);
        let eq5_e217_d_b1: f64 = (eq5_e215_d_b1 * p.p1);
        (eq5_e217, eq5_e217_d_n0, eq5_e217_d_n1, eq5_e217_d_n2, eq5_e217_d_n3, eq5_e217_d_n4, eq5_e217_d_n5, eq5_e217_d_n6, eq5_e217_d_n7, eq5_e217_d_n8, eq5_e217_d_n9, eq5_e217_d_n10, eq5_e217_d_n11, eq5_e217_d_n12, eq5_e217_d_b0, eq5_e217_d_b1,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq5_value: f64 = eq5_e219;
        let eq5_node_derivatives: [f64; 13] = [eq5_e219_d_n0, eq5_e219_d_n1, eq5_e219_d_n2, eq5_e219_d_n3, eq5_e219_d_n4, eq5_e219_d_n5, eq5_e219_d_n6, eq5_e219_d_n7, eq5_e219_d_n8, eq5_e219_d_n9, eq5_e219_d_n10, eq5_e219_d_n11, eq5_e219_d_n12];
        let eq5_branch_derivatives: [f64; 2] = [eq5_e219_d_b0, eq5_e219_d_b1];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(9),
            multiplicity * (eq5_value),
            &eq5_node_derivatives,
            &eq5_branch_derivatives,
            multiplicity,
        );
        let eq6_e222: f64 = (p.p3 * s.v[182]);
        let eq6_e222_d_n0: f64 = (p.p3 * s.dn[182][0]);
        let eq6_e222_d_n1: f64 = (p.p3 * s.dn[182][1]);
        let eq6_e222_d_n2: f64 = (p.p3 * s.dn[182][2]);
        let eq6_e222_d_n3: f64 = (p.p3 * s.dn[182][3]);
        let eq6_e222_d_n4: f64 = (p.p3 * s.dn[182][4]);
        let eq6_e222_d_n5: f64 = (p.p3 * s.dn[182][5]);
        let eq6_e222_d_n6: f64 = (p.p3 * s.dn[182][6]);
        let eq6_e222_d_n7: f64 = (p.p3 * s.dn[182][7]);
        let eq6_e222_d_n8: f64 = (p.p3 * s.dn[182][8]);
        let eq6_e222_d_n9: f64 = (p.p3 * s.dn[182][9]);
        let eq6_e222_d_n10: f64 = (p.p3 * s.dn[182][10]);
        let eq6_e222_d_n11: f64 = (p.p3 * s.dn[182][11]);
        let eq6_e222_d_n12: f64 = (p.p3 * s.dn[182][12]);
        let eq6_e222_d_b0: f64 = (p.p3 * s.db[182][0]);
        let eq6_e222_d_b1: f64 = (p.p3 * s.db[182][1]);
        let eq6_e224: f64 = (eq6_e222 * p.p1);
        let eq6_e224_d_n0: f64 = (eq6_e222_d_n0 * p.p1);
        let eq6_e224_d_n1: f64 = (eq6_e222_d_n1 * p.p1);
        let eq6_e224_d_n2: f64 = (eq6_e222_d_n2 * p.p1);
        let eq6_e224_d_n3: f64 = (eq6_e222_d_n3 * p.p1);
        let eq6_e224_d_n4: f64 = (eq6_e222_d_n4 * p.p1);
        let eq6_e224_d_n5: f64 = (eq6_e222_d_n5 * p.p1);
        let eq6_e224_d_n6: f64 = (eq6_e222_d_n6 * p.p1);
        let eq6_e224_d_n7: f64 = (eq6_e222_d_n7 * p.p1);
        let eq6_e224_d_n8: f64 = (eq6_e222_d_n8 * p.p1);
        let eq6_e224_d_n9: f64 = (eq6_e222_d_n9 * p.p1);
        let eq6_e224_d_n10: f64 = (eq6_e222_d_n10 * p.p1);
        let eq6_e224_d_n11: f64 = (eq6_e222_d_n11 * p.p1);
        let eq6_e224_d_n12: f64 = (eq6_e222_d_n12 * p.p1);
        let eq6_e224_d_b0: f64 = (eq6_e222_d_b0 * p.p1);
        let eq6_e224_d_b1: f64 = (eq6_e222_d_b1 * p.p1);
        let eq6_value: f64 = eq6_e224;
        let eq6_node_derivatives: [f64; 13] = [eq6_e224_d_n0, eq6_e224_d_n1, eq6_e224_d_n2, eq6_e224_d_n3, eq6_e224_d_n4, eq6_e224_d_n5, eq6_e224_d_n6, eq6_e224_d_n7, eq6_e224_d_n8, eq6_e224_d_n9, eq6_e224_d_n10, eq6_e224_d_n11, eq6_e224_d_n12];
        let eq6_branch_derivatives: [f64; 2] = [eq6_e224_d_b0, eq6_e224_d_b1];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(3),
            multiplicity * (eq6_value),
            &eq6_node_derivatives,
            &eq6_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_1(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
    ) {
        let eq7_e227: f64 = (p.p3 * s.v[185]);
        let eq7_e227_d_n0: f64 = (p.p3 * s.dn[185][0]);
        let eq7_e227_d_n1: f64 = (p.p3 * s.dn[185][1]);
        let eq7_e227_d_n2: f64 = (p.p3 * s.dn[185][2]);
        let eq7_e227_d_n3: f64 = (p.p3 * s.dn[185][3]);
        let eq7_e227_d_n4: f64 = (p.p3 * s.dn[185][4]);
        let eq7_e227_d_n5: f64 = (p.p3 * s.dn[185][5]);
        let eq7_e227_d_n6: f64 = (p.p3 * s.dn[185][6]);
        let eq7_e227_d_n7: f64 = (p.p3 * s.dn[185][7]);
        let eq7_e227_d_n8: f64 = (p.p3 * s.dn[185][8]);
        let eq7_e227_d_n9: f64 = (p.p3 * s.dn[185][9]);
        let eq7_e227_d_n10: f64 = (p.p3 * s.dn[185][10]);
        let eq7_e227_d_n11: f64 = (p.p3 * s.dn[185][11]);
        let eq7_e227_d_n12: f64 = (p.p3 * s.dn[185][12]);
        let eq7_e227_d_b0: f64 = (p.p3 * s.db[185][0]);
        let eq7_e227_d_b1: f64 = (p.p3 * s.db[185][1]);
        let eq7_e229: f64 = (eq7_e227 * p.p1);
        let eq7_e229_d_n0: f64 = (eq7_e227_d_n0 * p.p1);
        let eq7_e229_d_n1: f64 = (eq7_e227_d_n1 * p.p1);
        let eq7_e229_d_n2: f64 = (eq7_e227_d_n2 * p.p1);
        let eq7_e229_d_n3: f64 = (eq7_e227_d_n3 * p.p1);
        let eq7_e229_d_n4: f64 = (eq7_e227_d_n4 * p.p1);
        let eq7_e229_d_n5: f64 = (eq7_e227_d_n5 * p.p1);
        let eq7_e229_d_n6: f64 = (eq7_e227_d_n6 * p.p1);
        let eq7_e229_d_n7: f64 = (eq7_e227_d_n7 * p.p1);
        let eq7_e229_d_n8: f64 = (eq7_e227_d_n8 * p.p1);
        let eq7_e229_d_n9: f64 = (eq7_e227_d_n9 * p.p1);
        let eq7_e229_d_n10: f64 = (eq7_e227_d_n10 * p.p1);
        let eq7_e229_d_n11: f64 = (eq7_e227_d_n11 * p.p1);
        let eq7_e229_d_n12: f64 = (eq7_e227_d_n12 * p.p1);
        let eq7_e229_d_b0: f64 = (eq7_e227_d_b0 * p.p1);
        let eq7_e229_d_b1: f64 = (eq7_e227_d_b1 * p.p1);
        let eq7_value: f64 = eq7_e229;
        let eq7_node_derivatives: [f64; 13] = [eq7_e229_d_n0, eq7_e229_d_n1, eq7_e229_d_n2, eq7_e229_d_n3, eq7_e229_d_n4, eq7_e229_d_n5, eq7_e229_d_n6, eq7_e229_d_n7, eq7_e229_d_n8, eq7_e229_d_n9, eq7_e229_d_n10, eq7_e229_d_n11, eq7_e229_d_n12];
        let eq7_branch_derivatives: [f64; 2] = [eq7_e229_d_b0, eq7_e229_d_b1];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(3),
            multiplicity * (eq7_value),
            &eq7_node_derivatives,
            &eq7_branch_derivatives,
            multiplicity,
        );
        let eq8_e232: f64 = (p.p3 * s.v[183]);
        let eq8_e232_d_n0: f64 = (p.p3 * s.dn[183][0]);
        let eq8_e232_d_n1: f64 = (p.p3 * s.dn[183][1]);
        let eq8_e232_d_n2: f64 = (p.p3 * s.dn[183][2]);
        let eq8_e232_d_n3: f64 = (p.p3 * s.dn[183][3]);
        let eq8_e232_d_n4: f64 = (p.p3 * s.dn[183][4]);
        let eq8_e232_d_n5: f64 = (p.p3 * s.dn[183][5]);
        let eq8_e232_d_n6: f64 = (p.p3 * s.dn[183][6]);
        let eq8_e232_d_n7: f64 = (p.p3 * s.dn[183][7]);
        let eq8_e232_d_n8: f64 = (p.p3 * s.dn[183][8]);
        let eq8_e232_d_n9: f64 = (p.p3 * s.dn[183][9]);
        let eq8_e232_d_n10: f64 = (p.p3 * s.dn[183][10]);
        let eq8_e232_d_n11: f64 = (p.p3 * s.dn[183][11]);
        let eq8_e232_d_n12: f64 = (p.p3 * s.dn[183][12]);
        let eq8_e232_d_b0: f64 = (p.p3 * s.db[183][0]);
        let eq8_e232_d_b1: f64 = (p.p3 * s.db[183][1]);
        let eq8_e234: f64 = (eq8_e232 * p.p1);
        let eq8_e234_d_n0: f64 = (eq8_e232_d_n0 * p.p1);
        let eq8_e234_d_n1: f64 = (eq8_e232_d_n1 * p.p1);
        let eq8_e234_d_n2: f64 = (eq8_e232_d_n2 * p.p1);
        let eq8_e234_d_n3: f64 = (eq8_e232_d_n3 * p.p1);
        let eq8_e234_d_n4: f64 = (eq8_e232_d_n4 * p.p1);
        let eq8_e234_d_n5: f64 = (eq8_e232_d_n5 * p.p1);
        let eq8_e234_d_n6: f64 = (eq8_e232_d_n6 * p.p1);
        let eq8_e234_d_n7: f64 = (eq8_e232_d_n7 * p.p1);
        let eq8_e234_d_n8: f64 = (eq8_e232_d_n8 * p.p1);
        let eq8_e234_d_n9: f64 = (eq8_e232_d_n9 * p.p1);
        let eq8_e234_d_n10: f64 = (eq8_e232_d_n10 * p.p1);
        let eq8_e234_d_n11: f64 = (eq8_e232_d_n11 * p.p1);
        let eq8_e234_d_n12: f64 = (eq8_e232_d_n12 * p.p1);
        let eq8_e234_d_b0: f64 = (eq8_e232_d_b0 * p.p1);
        let eq8_e234_d_b1: f64 = (eq8_e232_d_b1 * p.p1);
        let eq8_value: f64 = eq8_e234;
        let eq8_node_derivatives: [f64; 13] = [eq8_e234_d_n0, eq8_e234_d_n1, eq8_e234_d_n2, eq8_e234_d_n3, eq8_e234_d_n4, eq8_e234_d_n5, eq8_e234_d_n6, eq8_e234_d_n7, eq8_e234_d_n8, eq8_e234_d_n9, eq8_e234_d_n10, eq8_e234_d_n11, eq8_e234_d_n12];
        let eq8_branch_derivatives: [f64; 2] = [eq8_e234_d_b0, eq8_e234_d_b1];
        stamper.stamp_current_dense_local(
            Some(1),
            Some(3),
            multiplicity * (eq8_value),
            &eq8_node_derivatives,
            &eq8_branch_derivatives,
            multiplicity,
        );
        let eq9_e237: f64 = (p.p3 * s.v[184]);
        let eq9_e237_d_n0: f64 = (p.p3 * s.dn[184][0]);
        let eq9_e237_d_n1: f64 = (p.p3 * s.dn[184][1]);
        let eq9_e237_d_n2: f64 = (p.p3 * s.dn[184][2]);
        let eq9_e237_d_n3: f64 = (p.p3 * s.dn[184][3]);
        let eq9_e237_d_n4: f64 = (p.p3 * s.dn[184][4]);
        let eq9_e237_d_n5: f64 = (p.p3 * s.dn[184][5]);
        let eq9_e237_d_n6: f64 = (p.p3 * s.dn[184][6]);
        let eq9_e237_d_n7: f64 = (p.p3 * s.dn[184][7]);
        let eq9_e237_d_n8: f64 = (p.p3 * s.dn[184][8]);
        let eq9_e237_d_n9: f64 = (p.p3 * s.dn[184][9]);
        let eq9_e237_d_n10: f64 = (p.p3 * s.dn[184][10]);
        let eq9_e237_d_n11: f64 = (p.p3 * s.dn[184][11]);
        let eq9_e237_d_n12: f64 = (p.p3 * s.dn[184][12]);
        let eq9_e237_d_b0: f64 = (p.p3 * s.db[184][0]);
        let eq9_e237_d_b1: f64 = (p.p3 * s.db[184][1]);
        let eq9_e239: f64 = (eq9_e237 * p.p1);
        let eq9_e239_d_n0: f64 = (eq9_e237_d_n0 * p.p1);
        let eq9_e239_d_n1: f64 = (eq9_e237_d_n1 * p.p1);
        let eq9_e239_d_n2: f64 = (eq9_e237_d_n2 * p.p1);
        let eq9_e239_d_n3: f64 = (eq9_e237_d_n3 * p.p1);
        let eq9_e239_d_n4: f64 = (eq9_e237_d_n4 * p.p1);
        let eq9_e239_d_n5: f64 = (eq9_e237_d_n5 * p.p1);
        let eq9_e239_d_n6: f64 = (eq9_e237_d_n6 * p.p1);
        let eq9_e239_d_n7: f64 = (eq9_e237_d_n7 * p.p1);
        let eq9_e239_d_n8: f64 = (eq9_e237_d_n8 * p.p1);
        let eq9_e239_d_n9: f64 = (eq9_e237_d_n9 * p.p1);
        let eq9_e239_d_n10: f64 = (eq9_e237_d_n10 * p.p1);
        let eq9_e239_d_n11: f64 = (eq9_e237_d_n11 * p.p1);
        let eq9_e239_d_n12: f64 = (eq9_e237_d_n12 * p.p1);
        let eq9_e239_d_b0: f64 = (eq9_e237_d_b0 * p.p1);
        let eq9_e239_d_b1: f64 = (eq9_e237_d_b1 * p.p1);
        let eq9_value: f64 = eq9_e239;
        let eq9_node_derivatives: [f64; 13] = [eq9_e239_d_n0, eq9_e239_d_n1, eq9_e239_d_n2, eq9_e239_d_n3, eq9_e239_d_n4, eq9_e239_d_n5, eq9_e239_d_n6, eq9_e239_d_n7, eq9_e239_d_n8, eq9_e239_d_n9, eq9_e239_d_n10, eq9_e239_d_n11, eq9_e239_d_n12];
        let eq9_branch_derivatives: [f64; 2] = [eq9_e239_d_b0, eq9_e239_d_b1];
        stamper.stamp_current_dense_local(
            Some(3),
            Some(8),
            multiplicity * (eq9_value),
            &eq9_node_derivatives,
            &eq9_branch_derivatives,
            multiplicity,
        );
        let eq10_e242: f64 = (p.p3 * s.v[191]);
        let eq10_e242_d_n0: f64 = (p.p3 * s.dn[191][0]);
        let eq10_e242_d_n1: f64 = (p.p3 * s.dn[191][1]);
        let eq10_e242_d_n2: f64 = (p.p3 * s.dn[191][2]);
        let eq10_e242_d_n3: f64 = (p.p3 * s.dn[191][3]);
        let eq10_e242_d_n4: f64 = (p.p3 * s.dn[191][4]);
        let eq10_e242_d_n5: f64 = (p.p3 * s.dn[191][5]);
        let eq10_e242_d_n6: f64 = (p.p3 * s.dn[191][6]);
        let eq10_e242_d_n7: f64 = (p.p3 * s.dn[191][7]);
        let eq10_e242_d_n8: f64 = (p.p3 * s.dn[191][8]);
        let eq10_e242_d_n9: f64 = (p.p3 * s.dn[191][9]);
        let eq10_e242_d_n10: f64 = (p.p3 * s.dn[191][10]);
        let eq10_e242_d_n11: f64 = (p.p3 * s.dn[191][11]);
        let eq10_e242_d_n12: f64 = (p.p3 * s.dn[191][12]);
        let eq10_e242_d_b0: f64 = (p.p3 * s.db[191][0]);
        let eq10_e242_d_b1: f64 = (p.p3 * s.db[191][1]);
        let eq10_e244: f64 = (eq10_e242 * p.p1);
        let eq10_e244_d_n0: f64 = (eq10_e242_d_n0 * p.p1);
        let eq10_e244_d_n1: f64 = (eq10_e242_d_n1 * p.p1);
        let eq10_e244_d_n2: f64 = (eq10_e242_d_n2 * p.p1);
        let eq10_e244_d_n3: f64 = (eq10_e242_d_n3 * p.p1);
        let eq10_e244_d_n4: f64 = (eq10_e242_d_n4 * p.p1);
        let eq10_e244_d_n5: f64 = (eq10_e242_d_n5 * p.p1);
        let eq10_e244_d_n6: f64 = (eq10_e242_d_n6 * p.p1);
        let eq10_e244_d_n7: f64 = (eq10_e242_d_n7 * p.p1);
        let eq10_e244_d_n8: f64 = (eq10_e242_d_n8 * p.p1);
        let eq10_e244_d_n9: f64 = (eq10_e242_d_n9 * p.p1);
        let eq10_e244_d_n10: f64 = (eq10_e242_d_n10 * p.p1);
        let eq10_e244_d_n11: f64 = (eq10_e242_d_n11 * p.p1);
        let eq10_e244_d_n12: f64 = (eq10_e242_d_n12 * p.p1);
        let eq10_e244_d_b0: f64 = (eq10_e242_d_b0 * p.p1);
        let eq10_e244_d_b1: f64 = (eq10_e242_d_b1 * p.p1);
        let eq10_value: f64 = eq10_e244;
        let eq10_node_derivatives: [f64; 13] = [eq10_e244_d_n0, eq10_e244_d_n1, eq10_e244_d_n2, eq10_e244_d_n3, eq10_e244_d_n4, eq10_e244_d_n5, eq10_e244_d_n6, eq10_e244_d_n7, eq10_e244_d_n8, eq10_e244_d_n9, eq10_e244_d_n10, eq10_e244_d_n11, eq10_e244_d_n12];
        let eq10_branch_derivatives: [f64; 2] = [eq10_e244_d_b0, eq10_e244_d_b1];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(7),
            multiplicity * (eq10_value),
            &eq10_node_derivatives,
            &eq10_branch_derivatives,
            multiplicity,
        );
        let eq11_e247: f64 = (-1.0);
        let eq11_e249: f64 = (eq11_e247 * s.v[212]);
        let eq11_e249_d_n0: f64 = (eq11_e247 * s.dn[212][0]);
        let eq11_e249_d_n1: f64 = (eq11_e247 * s.dn[212][1]);
        let eq11_e249_d_n2: f64 = (eq11_e247 * s.dn[212][2]);
        let eq11_e249_d_n3: f64 = (eq11_e247 * s.dn[212][3]);
        let eq11_e249_d_n4: f64 = (eq11_e247 * s.dn[212][4]);
        let eq11_e249_d_n5: f64 = (eq11_e247 * s.dn[212][5]);
        let eq11_e249_d_n6: f64 = (eq11_e247 * s.dn[212][6]);
        let eq11_e249_d_n7: f64 = (eq11_e247 * s.dn[212][7]);
        let eq11_e249_d_n8: f64 = (eq11_e247 * s.dn[212][8]);
        let eq11_e249_d_n9: f64 = (eq11_e247 * s.dn[212][9]);
        let eq11_e249_d_n10: f64 = (eq11_e247 * s.dn[212][10]);
        let eq11_e249_d_n11: f64 = (eq11_e247 * s.dn[212][11]);
        let eq11_e249_d_n12: f64 = (eq11_e247 * s.dn[212][12]);
        let eq11_e249_d_b0: f64 = (eq11_e247 * s.db[212][0]);
        let eq11_e249_d_b1: f64 = (eq11_e247 * s.db[212][1]);
        let eq11_e250: f64 = (p.p3 * eq11_e249);
        let eq11_e250_d_n0: f64 = (p.p3 * eq11_e249_d_n0);
        let eq11_e250_d_n1: f64 = (p.p3 * eq11_e249_d_n1);
        let eq11_e250_d_n2: f64 = (p.p3 * eq11_e249_d_n2);
        let eq11_e250_d_n3: f64 = (p.p3 * eq11_e249_d_n3);
        let eq11_e250_d_n4: f64 = (p.p3 * eq11_e249_d_n4);
        let eq11_e250_d_n5: f64 = (p.p3 * eq11_e249_d_n5);
        let eq11_e250_d_n6: f64 = (p.p3 * eq11_e249_d_n6);
        let eq11_e250_d_n7: f64 = (p.p3 * eq11_e249_d_n7);
        let eq11_e250_d_n8: f64 = (p.p3 * eq11_e249_d_n8);
        let eq11_e250_d_n9: f64 = (p.p3 * eq11_e249_d_n9);
        let eq11_e250_d_n10: f64 = (p.p3 * eq11_e249_d_n10);
        let eq11_e250_d_n11: f64 = (p.p3 * eq11_e249_d_n11);
        let eq11_e250_d_n12: f64 = (p.p3 * eq11_e249_d_n12);
        let eq11_e250_d_b0: f64 = (p.p3 * eq11_e249_d_b0);
        let eq11_e250_d_b1: f64 = (p.p3 * eq11_e249_d_b1);
        let eq11_e252: f64 = (eq11_e250 * p.p1);
        let eq11_e252_d_n0: f64 = (eq11_e250_d_n0 * p.p1);
        let eq11_e252_d_n1: f64 = (eq11_e250_d_n1 * p.p1);
        let eq11_e252_d_n2: f64 = (eq11_e250_d_n2 * p.p1);
        let eq11_e252_d_n3: f64 = (eq11_e250_d_n3 * p.p1);
        let eq11_e252_d_n4: f64 = (eq11_e250_d_n4 * p.p1);
        let eq11_e252_d_n5: f64 = (eq11_e250_d_n5 * p.p1);
        let eq11_e252_d_n6: f64 = (eq11_e250_d_n6 * p.p1);
        let eq11_e252_d_n7: f64 = (eq11_e250_d_n7 * p.p1);
        let eq11_e252_d_n8: f64 = (eq11_e250_d_n8 * p.p1);
        let eq11_e252_d_n9: f64 = (eq11_e250_d_n9 * p.p1);
        let eq11_e252_d_n10: f64 = (eq11_e250_d_n10 * p.p1);
        let eq11_e252_d_n11: f64 = (eq11_e250_d_n11 * p.p1);
        let eq11_e252_d_n12: f64 = (eq11_e250_d_n12 * p.p1);
        let eq11_e252_d_b0: f64 = (eq11_e250_d_b0 * p.p1);
        let eq11_e252_d_b1: f64 = (eq11_e250_d_b1 * p.p1);
        let eq11_value: f64 = eq11_e252;
        let eq11_node_derivatives: [f64; 13] = [eq11_e252_d_n0, eq11_e252_d_n1, eq11_e252_d_n2, eq11_e252_d_n3, eq11_e252_d_n4, eq11_e252_d_n5, eq11_e252_d_n6, eq11_e252_d_n7, eq11_e252_d_n8, eq11_e252_d_n9, eq11_e252_d_n10, eq11_e252_d_n11, eq11_e252_d_n12];
        let eq11_branch_derivatives: [f64; 2] = [eq11_e252_d_b0, eq11_e252_d_b1];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(9),
            multiplicity * (eq11_value),
            &eq11_node_derivatives,
            &eq11_branch_derivatives,
            multiplicity,
        );
        let eq12_e255: f64 = (p.p3 * s.v[265]);
        let eq12_e255_d_n0: f64 = (p.p3 * s.dn[265][0]);
        let eq12_e255_d_n1: f64 = (p.p3 * s.dn[265][1]);
        let eq12_e255_d_n2: f64 = (p.p3 * s.dn[265][2]);
        let eq12_e255_d_n3: f64 = (p.p3 * s.dn[265][3]);
        let eq12_e255_d_n4: f64 = (p.p3 * s.dn[265][4]);
        let eq12_e255_d_n5: f64 = (p.p3 * s.dn[265][5]);
        let eq12_e255_d_n6: f64 = (p.p3 * s.dn[265][6]);
        let eq12_e255_d_n7: f64 = (p.p3 * s.dn[265][7]);
        let eq12_e255_d_n8: f64 = (p.p3 * s.dn[265][8]);
        let eq12_e255_d_n9: f64 = (p.p3 * s.dn[265][9]);
        let eq12_e255_d_n10: f64 = (p.p3 * s.dn[265][10]);
        let eq12_e255_d_n11: f64 = (p.p3 * s.dn[265][11]);
        let eq12_e255_d_n12: f64 = (p.p3 * s.dn[265][12]);
        let eq12_e255_d_b0: f64 = (p.p3 * s.db[265][0]);
        let eq12_e255_d_b1: f64 = (p.p3 * s.db[265][1]);
        let eq12_e257: f64 = (eq12_e255 / s.v[28]);
        let eq12_e257_d_n0: f64 = (((eq12_e255_d_n0 * s.v[28]) - (eq12_e255 * s.dn[28][0])) / (s.v[28] * s.v[28]));
        let eq12_e257_d_n1: f64 = (((eq12_e255_d_n1 * s.v[28]) - (eq12_e255 * s.dn[28][1])) / (s.v[28] * s.v[28]));
        let eq12_e257_d_n2: f64 = (((eq12_e255_d_n2 * s.v[28]) - (eq12_e255 * s.dn[28][2])) / (s.v[28] * s.v[28]));
        let eq12_e257_d_n3: f64 = (((eq12_e255_d_n3 * s.v[28]) - (eq12_e255 * s.dn[28][3])) / (s.v[28] * s.v[28]));
        let eq12_e257_d_n4: f64 = (((eq12_e255_d_n4 * s.v[28]) - (eq12_e255 * s.dn[28][4])) / (s.v[28] * s.v[28]));
        let eq12_e257_d_n5: f64 = (((eq12_e255_d_n5 * s.v[28]) - (eq12_e255 * s.dn[28][5])) / (s.v[28] * s.v[28]));
        let eq12_e257_d_n6: f64 = (((eq12_e255_d_n6 * s.v[28]) - (eq12_e255 * s.dn[28][6])) / (s.v[28] * s.v[28]));
        let eq12_e257_d_n7: f64 = (((eq12_e255_d_n7 * s.v[28]) - (eq12_e255 * s.dn[28][7])) / (s.v[28] * s.v[28]));
        let eq12_e257_d_n8: f64 = (((eq12_e255_d_n8 * s.v[28]) - (eq12_e255 * s.dn[28][8])) / (s.v[28] * s.v[28]));
        let eq12_e257_d_n9: f64 = (((eq12_e255_d_n9 * s.v[28]) - (eq12_e255 * s.dn[28][9])) / (s.v[28] * s.v[28]));
        let eq12_e257_d_n10: f64 = (((eq12_e255_d_n10 * s.v[28]) - (eq12_e255 * s.dn[28][10])) / (s.v[28] * s.v[28]));
        let eq12_e257_d_n11: f64 = (((eq12_e255_d_n11 * s.v[28]) - (eq12_e255 * s.dn[28][11])) / (s.v[28] * s.v[28]));
        let eq12_e257_d_n12: f64 = (((eq12_e255_d_n12 * s.v[28]) - (eq12_e255 * s.dn[28][12])) / (s.v[28] * s.v[28]));
        let eq12_e257_d_b0: f64 = (((eq12_e255_d_b0 * s.v[28]) - (eq12_e255 * s.db[28][0])) / (s.v[28] * s.v[28]));
        let eq12_e257_d_b1: f64 = (((eq12_e255_d_b1 * s.v[28]) - (eq12_e255 * s.db[28][1])) / (s.v[28] * s.v[28]));
        let eq12_e259: f64 = (eq12_e257 * p.p1);
        let eq12_e259_d_n0: f64 = (eq12_e257_d_n0 * p.p1);
        let eq12_e259_d_n1: f64 = (eq12_e257_d_n1 * p.p1);
        let eq12_e259_d_n2: f64 = (eq12_e257_d_n2 * p.p1);
        let eq12_e259_d_n3: f64 = (eq12_e257_d_n3 * p.p1);
        let eq12_e259_d_n4: f64 = (eq12_e257_d_n4 * p.p1);
        let eq12_e259_d_n5: f64 = (eq12_e257_d_n5 * p.p1);
        let eq12_e259_d_n6: f64 = (eq12_e257_d_n6 * p.p1);
        let eq12_e259_d_n7: f64 = (eq12_e257_d_n7 * p.p1);
        let eq12_e259_d_n8: f64 = (eq12_e257_d_n8 * p.p1);
        let eq12_e259_d_n9: f64 = (eq12_e257_d_n9 * p.p1);
        let eq12_e259_d_n10: f64 = (eq12_e257_d_n10 * p.p1);
        let eq12_e259_d_n11: f64 = (eq12_e257_d_n11 * p.p1);
        let eq12_e259_d_n12: f64 = (eq12_e257_d_n12 * p.p1);
        let eq12_e259_d_b0: f64 = (eq12_e257_d_b0 * p.p1);
        let eq12_e259_d_b1: f64 = (eq12_e257_d_b1 * p.p1);
        let eq12_value: f64 = eq12_e259;
        let eq12_node_derivatives: [f64; 13] = [eq12_e259_d_n0, eq12_e259_d_n1, eq12_e259_d_n2, eq12_e259_d_n3, eq12_e259_d_n4, eq12_e259_d_n5, eq12_e259_d_n6, eq12_e259_d_n7, eq12_e259_d_n8, eq12_e259_d_n9, eq12_e259_d_n10, eq12_e259_d_n11, eq12_e259_d_n12];
        let eq12_branch_derivatives: [f64; 2] = [eq12_e259_d_b0, eq12_e259_d_b1];
        stamper.stamp_current_dense_local(
            Some(2),
            Some(5),
            multiplicity * (eq12_value),
            &eq12_node_derivatives,
            &eq12_branch_derivatives,
            multiplicity,
        );
        let eq13_e262: f64 = (p.p3 * s.v[266]);
        let eq13_e262_d_n0: f64 = (p.p3 * s.dn[266][0]);
        let eq13_e262_d_n1: f64 = (p.p3 * s.dn[266][1]);
        let eq13_e262_d_n2: f64 = (p.p3 * s.dn[266][2]);
        let eq13_e262_d_n3: f64 = (p.p3 * s.dn[266][3]);
        let eq13_e262_d_n4: f64 = (p.p3 * s.dn[266][4]);
        let eq13_e262_d_n5: f64 = (p.p3 * s.dn[266][5]);
        let eq13_e262_d_n6: f64 = (p.p3 * s.dn[266][6]);
        let eq13_e262_d_n7: f64 = (p.p3 * s.dn[266][7]);
        let eq13_e262_d_n8: f64 = (p.p3 * s.dn[266][8]);
        let eq13_e262_d_n9: f64 = (p.p3 * s.dn[266][9]);
        let eq13_e262_d_n10: f64 = (p.p3 * s.dn[266][10]);
        let eq13_e262_d_n11: f64 = (p.p3 * s.dn[266][11]);
        let eq13_e262_d_n12: f64 = (p.p3 * s.dn[266][12]);
        let eq13_e262_d_b0: f64 = (p.p3 * s.db[266][0]);
        let eq13_e262_d_b1: f64 = (p.p3 * s.db[266][1]);
        let eq13_e264: f64 = (eq13_e262 / s.v[30]);
        let eq13_e264_d_n0: f64 = (((eq13_e262_d_n0 * s.v[30]) - (eq13_e262 * s.dn[30][0])) / (s.v[30] * s.v[30]));
        let eq13_e264_d_n1: f64 = (((eq13_e262_d_n1 * s.v[30]) - (eq13_e262 * s.dn[30][1])) / (s.v[30] * s.v[30]));
        let eq13_e264_d_n2: f64 = (((eq13_e262_d_n2 * s.v[30]) - (eq13_e262 * s.dn[30][2])) / (s.v[30] * s.v[30]));
        let eq13_e264_d_n3: f64 = (((eq13_e262_d_n3 * s.v[30]) - (eq13_e262 * s.dn[30][3])) / (s.v[30] * s.v[30]));
        let eq13_e264_d_n4: f64 = (((eq13_e262_d_n4 * s.v[30]) - (eq13_e262 * s.dn[30][4])) / (s.v[30] * s.v[30]));
        let eq13_e264_d_n5: f64 = (((eq13_e262_d_n5 * s.v[30]) - (eq13_e262 * s.dn[30][5])) / (s.v[30] * s.v[30]));
        let eq13_e264_d_n6: f64 = (((eq13_e262_d_n6 * s.v[30]) - (eq13_e262 * s.dn[30][6])) / (s.v[30] * s.v[30]));
        let eq13_e264_d_n7: f64 = (((eq13_e262_d_n7 * s.v[30]) - (eq13_e262 * s.dn[30][7])) / (s.v[30] * s.v[30]));
        let eq13_e264_d_n8: f64 = (((eq13_e262_d_n8 * s.v[30]) - (eq13_e262 * s.dn[30][8])) / (s.v[30] * s.v[30]));
        let eq13_e264_d_n9: f64 = (((eq13_e262_d_n9 * s.v[30]) - (eq13_e262 * s.dn[30][9])) / (s.v[30] * s.v[30]));
        let eq13_e264_d_n10: f64 = (((eq13_e262_d_n10 * s.v[30]) - (eq13_e262 * s.dn[30][10])) / (s.v[30] * s.v[30]));
        let eq13_e264_d_n11: f64 = (((eq13_e262_d_n11 * s.v[30]) - (eq13_e262 * s.dn[30][11])) / (s.v[30] * s.v[30]));
        let eq13_e264_d_n12: f64 = (((eq13_e262_d_n12 * s.v[30]) - (eq13_e262 * s.dn[30][12])) / (s.v[30] * s.v[30]));
        let eq13_e264_d_b0: f64 = (((eq13_e262_d_b0 * s.v[30]) - (eq13_e262 * s.db[30][0])) / (s.v[30] * s.v[30]));
        let eq13_e264_d_b1: f64 = (((eq13_e262_d_b1 * s.v[30]) - (eq13_e262 * s.db[30][1])) / (s.v[30] * s.v[30]));
        let eq13_e266: f64 = (eq13_e264 * p.p1);
        let eq13_e266_d_n0: f64 = (eq13_e264_d_n0 * p.p1);
        let eq13_e266_d_n1: f64 = (eq13_e264_d_n1 * p.p1);
        let eq13_e266_d_n2: f64 = (eq13_e264_d_n2 * p.p1);
        let eq13_e266_d_n3: f64 = (eq13_e264_d_n3 * p.p1);
        let eq13_e266_d_n4: f64 = (eq13_e264_d_n4 * p.p1);
        let eq13_e266_d_n5: f64 = (eq13_e264_d_n5 * p.p1);
        let eq13_e266_d_n6: f64 = (eq13_e264_d_n6 * p.p1);
        let eq13_e266_d_n7: f64 = (eq13_e264_d_n7 * p.p1);
        let eq13_e266_d_n8: f64 = (eq13_e264_d_n8 * p.p1);
        let eq13_e266_d_n9: f64 = (eq13_e264_d_n9 * p.p1);
        let eq13_e266_d_n10: f64 = (eq13_e264_d_n10 * p.p1);
        let eq13_e266_d_n11: f64 = (eq13_e264_d_n11 * p.p1);
        let eq13_e266_d_n12: f64 = (eq13_e264_d_n12 * p.p1);
        let eq13_e266_d_b0: f64 = (eq13_e264_d_b0 * p.p1);
        let eq13_e266_d_b1: f64 = (eq13_e264_d_b1 * p.p1);
        let eq13_value: f64 = eq13_e266;
        let eq13_node_derivatives: [f64; 13] = [eq13_e266_d_n0, eq13_e266_d_n1, eq13_e266_d_n2, eq13_e266_d_n3, eq13_e266_d_n4, eq13_e266_d_n5, eq13_e266_d_n6, eq13_e266_d_n7, eq13_e266_d_n8, eq13_e266_d_n9, eq13_e266_d_n10, eq13_e266_d_n11, eq13_e266_d_n12];
        let eq13_branch_derivatives: [f64; 2] = [eq13_e266_d_b0, eq13_e266_d_b1];
        stamper.stamp_current_dense_local(
            Some(1),
            Some(6),
            multiplicity * (eq13_value),
            &eq13_node_derivatives,
            &eq13_branch_derivatives,
            multiplicity,
        );
        let eq14_value: f64 = s.v[102];
        let eq14_node_derivatives: [f64; 13] = [s.dn[102][0], s.dn[102][1], s.dn[102][2], s.dn[102][3], s.dn[102][4], s.dn[102][5], s.dn[102][6], s.dn[102][7], s.dn[102][8], s.dn[102][9], s.dn[102][10], s.dn[102][11], s.dn[102][12]];
        let eq14_branch_derivatives: [f64; 2] = [s.db[102][0], s.db[102][1]];
        stamper.stamp_current_dense_local(
            Some(4),
            None,
            multiplicity * (eq14_value),
            &eq14_node_derivatives,
            &eq14_branch_derivatives,
            multiplicity,
        );
        let eq15_value: f64 = s.v[220];
        let eq15_node_derivatives: [f64; 13] = [s.dn[220][0], s.dn[220][1], s.dn[220][2], s.dn[220][3], s.dn[220][4], s.dn[220][5], s.dn[220][6], s.dn[220][7], s.dn[220][8], s.dn[220][9], s.dn[220][10], s.dn[220][11], s.dn[220][12]];
        let eq15_branch_derivatives: [f64; 2] = [s.db[220][0], s.db[220][1]];
        stamper.stamp_current_dense_local(
            Some(4),
            None,
            multiplicity * (eq15_value),
            &eq15_node_derivatives,
            &eq15_branch_derivatives,
            multiplicity,
        );
        let eq16_e270: f64 = (-1.0);
        let eq16_e272: f64 = (eq16_e270 * s.v[219]);
        let eq16_e272_d_n0: f64 = (eq16_e270 * s.dn[219][0]);
        let eq16_e272_d_n1: f64 = (eq16_e270 * s.dn[219][1]);
        let eq16_e272_d_n2: f64 = (eq16_e270 * s.dn[219][2]);
        let eq16_e272_d_n3: f64 = (eq16_e270 * s.dn[219][3]);
        let eq16_e272_d_n4: f64 = (eq16_e270 * s.dn[219][4]);
        let eq16_e272_d_n5: f64 = (eq16_e270 * s.dn[219][5]);
        let eq16_e272_d_n6: f64 = (eq16_e270 * s.dn[219][6]);
        let eq16_e272_d_n7: f64 = (eq16_e270 * s.dn[219][7]);
        let eq16_e272_d_n8: f64 = (eq16_e270 * s.dn[219][8]);
        let eq16_e272_d_n9: f64 = (eq16_e270 * s.dn[219][9]);
        let eq16_e272_d_n10: f64 = (eq16_e270 * s.dn[219][10]);
        let eq16_e272_d_n11: f64 = (eq16_e270 * s.dn[219][11]);
        let eq16_e272_d_n12: f64 = (eq16_e270 * s.dn[219][12]);
        let eq16_e272_d_b0: f64 = (eq16_e270 * s.db[219][0]);
        let eq16_e272_d_b1: f64 = (eq16_e270 * s.db[219][1]);
        let eq16_e274: f64 = (eq16_e272 * p.p1);
        let eq16_e274_d_n0: f64 = (eq16_e272_d_n0 * p.p1);
        let eq16_e274_d_n1: f64 = (eq16_e272_d_n1 * p.p1);
        let eq16_e274_d_n2: f64 = (eq16_e272_d_n2 * p.p1);
        let eq16_e274_d_n3: f64 = (eq16_e272_d_n3 * p.p1);
        let eq16_e274_d_n4: f64 = (eq16_e272_d_n4 * p.p1);
        let eq16_e274_d_n5: f64 = (eq16_e272_d_n5 * p.p1);
        let eq16_e274_d_n6: f64 = (eq16_e272_d_n6 * p.p1);
        let eq16_e274_d_n7: f64 = (eq16_e272_d_n7 * p.p1);
        let eq16_e274_d_n8: f64 = (eq16_e272_d_n8 * p.p1);
        let eq16_e274_d_n9: f64 = (eq16_e272_d_n9 * p.p1);
        let eq16_e274_d_n10: f64 = (eq16_e272_d_n10 * p.p1);
        let eq16_e274_d_n11: f64 = (eq16_e272_d_n11 * p.p1);
        let eq16_e274_d_n12: f64 = (eq16_e272_d_n12 * p.p1);
        let eq16_e274_d_b0: f64 = (eq16_e272_d_b0 * p.p1);
        let eq16_e274_d_b1: f64 = (eq16_e272_d_b1 * p.p1);
        let eq16_value: f64 = eq16_e274;
        let eq16_node_derivatives: [f64; 13] = [eq16_e274_d_n0, eq16_e274_d_n1, eq16_e274_d_n2, eq16_e274_d_n3, eq16_e274_d_n4, eq16_e274_d_n5, eq16_e274_d_n6, eq16_e274_d_n7, eq16_e274_d_n8, eq16_e274_d_n9, eq16_e274_d_n10, eq16_e274_d_n11, eq16_e274_d_n12];
        let eq16_branch_derivatives: [f64; 2] = [eq16_e274_d_b0, eq16_e274_d_b1];
        stamper.stamp_current_dense_local(
            Some(4),
            None,
            multiplicity * (eq16_value),
            &eq16_node_derivatives,
            &eq16_branch_derivatives,
            multiplicity,
        );
        let eq17_e278: f64 = (s.v[221] + s.v[226]);
        let eq17_e278_d_n0: f64 = (s.dn[221][0] + s.dn[226][0]);
        let eq17_e278_d_n1: f64 = (s.dn[221][1] + s.dn[226][1]);
        let eq17_e278_d_n2: f64 = (s.dn[221][2] + s.dn[226][2]);
        let eq17_e278_d_n3: f64 = (s.dn[221][3] + s.dn[226][3]);
        let eq17_e278_d_n4: f64 = (s.dn[221][4] + s.dn[226][4]);
        let eq17_e278_d_n5: f64 = (s.dn[221][5] + s.dn[226][5]);
        let eq17_e278_d_n6: f64 = (s.dn[221][6] + s.dn[226][6]);
        let eq17_e278_d_n7: f64 = (s.dn[221][7] + s.dn[226][7]);
        let eq17_e278_d_n8: f64 = (s.dn[221][8] + s.dn[226][8]);
        let eq17_e278_d_n9: f64 = (s.dn[221][9] + s.dn[226][9]);
        let eq17_e278_d_n10: f64 = (s.dn[221][10] + s.dn[226][10]);
        let eq17_e278_d_n11: f64 = (s.dn[221][11] + s.dn[226][11]);
        let eq17_e278_d_n12: f64 = (s.dn[221][12] + s.dn[226][12]);
        let eq17_e278_d_b0: f64 = (s.db[221][0] + s.db[226][0]);
        let eq17_e278_d_b1: f64 = (s.db[221][1] + s.db[226][1]);
        let eq17_e280: f64 = (eq17_e278 + s.v[241]);
        let eq17_e280_d_n0: f64 = (eq17_e278_d_n0 + s.dn[241][0]);
        let eq17_e280_d_n1: f64 = (eq17_e278_d_n1 + s.dn[241][1]);
        let eq17_e280_d_n2: f64 = (eq17_e278_d_n2 + s.dn[241][2]);
        let eq17_e280_d_n3: f64 = (eq17_e278_d_n3 + s.dn[241][3]);
        let eq17_e280_d_n4: f64 = (eq17_e278_d_n4 + s.dn[241][4]);
        let eq17_e280_d_n5: f64 = (eq17_e278_d_n5 + s.dn[241][5]);
        let eq17_e280_d_n6: f64 = (eq17_e278_d_n6 + s.dn[241][6]);
        let eq17_e280_d_n7: f64 = (eq17_e278_d_n7 + s.dn[241][7]);
        let eq17_e280_d_n8: f64 = (eq17_e278_d_n8 + s.dn[241][8]);
        let eq17_e280_d_n9: f64 = (eq17_e278_d_n9 + s.dn[241][9]);
        let eq17_e280_d_n10: f64 = (eq17_e278_d_n10 + s.dn[241][10]);
        let eq17_e280_d_n11: f64 = (eq17_e278_d_n11 + s.dn[241][11]);
        let eq17_e280_d_n12: f64 = (eq17_e278_d_n12 + s.dn[241][12]);
        let eq17_e280_d_b0: f64 = (eq17_e278_d_b0 + s.db[241][0]);
        let eq17_e280_d_b1: f64 = (eq17_e278_d_b1 + s.db[241][1]);
        let eq17_e281: f64 = (p.p3 * eq17_e280);
        let eq17_e281_d_n0: f64 = (p.p3 * eq17_e280_d_n0);
        let eq17_e281_d_n1: f64 = (p.p3 * eq17_e280_d_n1);
        let eq17_e281_d_n2: f64 = (p.p3 * eq17_e280_d_n2);
        let eq17_e281_d_n3: f64 = (p.p3 * eq17_e280_d_n3);
        let eq17_e281_d_n4: f64 = (p.p3 * eq17_e280_d_n4);
        let eq17_e281_d_n5: f64 = (p.p3 * eq17_e280_d_n5);
        let eq17_e281_d_n6: f64 = (p.p3 * eq17_e280_d_n6);
        let eq17_e281_d_n7: f64 = (p.p3 * eq17_e280_d_n7);
        let eq17_e281_d_n8: f64 = (p.p3 * eq17_e280_d_n8);
        let eq17_e281_d_n9: f64 = (p.p3 * eq17_e280_d_n9);
        let eq17_e281_d_n10: f64 = (p.p3 * eq17_e280_d_n10);
        let eq17_e281_d_n11: f64 = (p.p3 * eq17_e280_d_n11);
        let eq17_e281_d_n12: f64 = (p.p3 * eq17_e280_d_n12);
        let eq17_e281_d_b0: f64 = (p.p3 * eq17_e280_d_b0);
        let eq17_e281_d_b1: f64 = (p.p3 * eq17_e280_d_b1);
        let eq17_e282: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 1, eq17_e281);
        let eq17_e282_d_n0: f64 = (eq17_e281_d_n0 * ddt_scale);
        let eq17_e282_d_n1: f64 = (eq17_e281_d_n1 * ddt_scale);
        let eq17_e282_d_n2: f64 = (eq17_e281_d_n2 * ddt_scale);
        let eq17_e282_d_n3: f64 = (eq17_e281_d_n3 * ddt_scale);
        let eq17_e282_d_n4: f64 = (eq17_e281_d_n4 * ddt_scale);
        let eq17_e282_d_n5: f64 = (eq17_e281_d_n5 * ddt_scale);
        let eq17_e282_d_n6: f64 = (eq17_e281_d_n6 * ddt_scale);
        let eq17_e282_d_n7: f64 = (eq17_e281_d_n7 * ddt_scale);
        let eq17_e282_d_n8: f64 = (eq17_e281_d_n8 * ddt_scale);
        let eq17_e282_d_n9: f64 = (eq17_e281_d_n9 * ddt_scale);
        let eq17_e282_d_n10: f64 = (eq17_e281_d_n10 * ddt_scale);
        let eq17_e282_d_n11: f64 = (eq17_e281_d_n11 * ddt_scale);
        let eq17_e282_d_n12: f64 = (eq17_e281_d_n12 * ddt_scale);
        let eq17_e282_d_b0: f64 = (eq17_e281_d_b0 * ddt_scale);
        let eq17_e282_d_b1: f64 = (eq17_e281_d_b1 * ddt_scale);
        let eq17_e284: f64 = (eq17_e282 * p.p1);
        let eq17_e284_d_n0: f64 = (eq17_e282_d_n0 * p.p1);
        let eq17_e284_d_n1: f64 = (eq17_e282_d_n1 * p.p1);
        let eq17_e284_d_n2: f64 = (eq17_e282_d_n2 * p.p1);
        let eq17_e284_d_n3: f64 = (eq17_e282_d_n3 * p.p1);
        let eq17_e284_d_n4: f64 = (eq17_e282_d_n4 * p.p1);
        let eq17_e284_d_n5: f64 = (eq17_e282_d_n5 * p.p1);
        let eq17_e284_d_n6: f64 = (eq17_e282_d_n6 * p.p1);
        let eq17_e284_d_n7: f64 = (eq17_e282_d_n7 * p.p1);
        let eq17_e284_d_n8: f64 = (eq17_e282_d_n8 * p.p1);
        let eq17_e284_d_n9: f64 = (eq17_e282_d_n9 * p.p1);
        let eq17_e284_d_n10: f64 = (eq17_e282_d_n10 * p.p1);
        let eq17_e284_d_n11: f64 = (eq17_e282_d_n11 * p.p1);
        let eq17_e284_d_n12: f64 = (eq17_e282_d_n12 * p.p1);
        let eq17_e284_d_b0: f64 = (eq17_e282_d_b0 * p.p1);
        let eq17_e284_d_b1: f64 = (eq17_e282_d_b1 * p.p1);
        let eq17_value: f64 = eq17_e284;
        let eq17_node_derivatives: [f64; 13] = [eq17_e284_d_n0, eq17_e284_d_n1, eq17_e284_d_n2, eq17_e284_d_n3, eq17_e284_d_n4, eq17_e284_d_n5, eq17_e284_d_n6, eq17_e284_d_n7, eq17_e284_d_n8, eq17_e284_d_n9, eq17_e284_d_n10, eq17_e284_d_n11, eq17_e284_d_n12];
        let eq17_branch_derivatives: [f64; 2] = [eq17_e284_d_b0, eq17_e284_d_b1];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(5),
            multiplicity * (eq17_value),
            &eq17_node_derivatives,
            &eq17_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_2(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
    ) {
        let eq18_e287: f64 = (p.p3 * s.v[223]);
        let eq18_e287_d_n0: f64 = (p.p3 * s.dn[223][0]);
        let eq18_e287_d_n1: f64 = (p.p3 * s.dn[223][1]);
        let eq18_e287_d_n2: f64 = (p.p3 * s.dn[223][2]);
        let eq18_e287_d_n3: f64 = (p.p3 * s.dn[223][3]);
        let eq18_e287_d_n4: f64 = (p.p3 * s.dn[223][4]);
        let eq18_e287_d_n5: f64 = (p.p3 * s.dn[223][5]);
        let eq18_e287_d_n6: f64 = (p.p3 * s.dn[223][6]);
        let eq18_e287_d_n7: f64 = (p.p3 * s.dn[223][7]);
        let eq18_e287_d_n8: f64 = (p.p3 * s.dn[223][8]);
        let eq18_e287_d_n9: f64 = (p.p3 * s.dn[223][9]);
        let eq18_e287_d_n10: f64 = (p.p3 * s.dn[223][10]);
        let eq18_e287_d_n11: f64 = (p.p3 * s.dn[223][11]);
        let eq18_e287_d_n12: f64 = (p.p3 * s.dn[223][12]);
        let eq18_e287_d_b0: f64 = (p.p3 * s.db[223][0]);
        let eq18_e287_d_b1: f64 = (p.p3 * s.db[223][1]);
        let eq18_e288: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 2, eq18_e287);
        let eq18_e288_d_n0: f64 = (eq18_e287_d_n0 * ddt_scale);
        let eq18_e288_d_n1: f64 = (eq18_e287_d_n1 * ddt_scale);
        let eq18_e288_d_n2: f64 = (eq18_e287_d_n2 * ddt_scale);
        let eq18_e288_d_n3: f64 = (eq18_e287_d_n3 * ddt_scale);
        let eq18_e288_d_n4: f64 = (eq18_e287_d_n4 * ddt_scale);
        let eq18_e288_d_n5: f64 = (eq18_e287_d_n5 * ddt_scale);
        let eq18_e288_d_n6: f64 = (eq18_e287_d_n6 * ddt_scale);
        let eq18_e288_d_n7: f64 = (eq18_e287_d_n7 * ddt_scale);
        let eq18_e288_d_n8: f64 = (eq18_e287_d_n8 * ddt_scale);
        let eq18_e288_d_n9: f64 = (eq18_e287_d_n9 * ddt_scale);
        let eq18_e288_d_n10: f64 = (eq18_e287_d_n10 * ddt_scale);
        let eq18_e288_d_n11: f64 = (eq18_e287_d_n11 * ddt_scale);
        let eq18_e288_d_n12: f64 = (eq18_e287_d_n12 * ddt_scale);
        let eq18_e288_d_b0: f64 = (eq18_e287_d_b0 * ddt_scale);
        let eq18_e288_d_b1: f64 = (eq18_e287_d_b1 * ddt_scale);
        let eq18_e290: f64 = (eq18_e288 * p.p1);
        let eq18_e290_d_n0: f64 = (eq18_e288_d_n0 * p.p1);
        let eq18_e290_d_n1: f64 = (eq18_e288_d_n1 * p.p1);
        let eq18_e290_d_n2: f64 = (eq18_e288_d_n2 * p.p1);
        let eq18_e290_d_n3: f64 = (eq18_e288_d_n3 * p.p1);
        let eq18_e290_d_n4: f64 = (eq18_e288_d_n4 * p.p1);
        let eq18_e290_d_n5: f64 = (eq18_e288_d_n5 * p.p1);
        let eq18_e290_d_n6: f64 = (eq18_e288_d_n6 * p.p1);
        let eq18_e290_d_n7: f64 = (eq18_e288_d_n7 * p.p1);
        let eq18_e290_d_n8: f64 = (eq18_e288_d_n8 * p.p1);
        let eq18_e290_d_n9: f64 = (eq18_e288_d_n9 * p.p1);
        let eq18_e290_d_n10: f64 = (eq18_e288_d_n10 * p.p1);
        let eq18_e290_d_n11: f64 = (eq18_e288_d_n11 * p.p1);
        let eq18_e290_d_n12: f64 = (eq18_e288_d_n12 * p.p1);
        let eq18_e290_d_b0: f64 = (eq18_e288_d_b0 * p.p1);
        let eq18_e290_d_b1: f64 = (eq18_e288_d_b1 * p.p1);
        let eq18_value: f64 = eq18_e290;
        let eq18_node_derivatives: [f64; 13] = [eq18_e290_d_n0, eq18_e290_d_n1, eq18_e290_d_n2, eq18_e290_d_n3, eq18_e290_d_n4, eq18_e290_d_n5, eq18_e290_d_n6, eq18_e290_d_n7, eq18_e290_d_n8, eq18_e290_d_n9, eq18_e290_d_n10, eq18_e290_d_n11, eq18_e290_d_n12];
        let eq18_branch_derivatives: [f64; 2] = [eq18_e290_d_b0, eq18_e290_d_b1];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(5),
            multiplicity * (eq18_value),
            &eq18_node_derivatives,
            &eq18_branch_derivatives,
            multiplicity,
        );
        let eq19_e294: f64 = (s.v[224] + s.v[227]);
        let eq19_e294_d_n0: f64 = (s.dn[224][0] + s.dn[227][0]);
        let eq19_e294_d_n1: f64 = (s.dn[224][1] + s.dn[227][1]);
        let eq19_e294_d_n2: f64 = (s.dn[224][2] + s.dn[227][2]);
        let eq19_e294_d_n3: f64 = (s.dn[224][3] + s.dn[227][3]);
        let eq19_e294_d_n4: f64 = (s.dn[224][4] + s.dn[227][4]);
        let eq19_e294_d_n5: f64 = (s.dn[224][5] + s.dn[227][5]);
        let eq19_e294_d_n6: f64 = (s.dn[224][6] + s.dn[227][6]);
        let eq19_e294_d_n7: f64 = (s.dn[224][7] + s.dn[227][7]);
        let eq19_e294_d_n8: f64 = (s.dn[224][8] + s.dn[227][8]);
        let eq19_e294_d_n9: f64 = (s.dn[224][9] + s.dn[227][9]);
        let eq19_e294_d_n10: f64 = (s.dn[224][10] + s.dn[227][10]);
        let eq19_e294_d_n11: f64 = (s.dn[224][11] + s.dn[227][11]);
        let eq19_e294_d_n12: f64 = (s.dn[224][12] + s.dn[227][12]);
        let eq19_e294_d_b0: f64 = (s.db[224][0] + s.db[227][0]);
        let eq19_e294_d_b1: f64 = (s.db[224][1] + s.db[227][1]);
        let eq19_e296: f64 = (eq19_e294 + s.v[244]);
        let eq19_e296_d_n0: f64 = (eq19_e294_d_n0 + s.dn[244][0]);
        let eq19_e296_d_n1: f64 = (eq19_e294_d_n1 + s.dn[244][1]);
        let eq19_e296_d_n2: f64 = (eq19_e294_d_n2 + s.dn[244][2]);
        let eq19_e296_d_n3: f64 = (eq19_e294_d_n3 + s.dn[244][3]);
        let eq19_e296_d_n4: f64 = (eq19_e294_d_n4 + s.dn[244][4]);
        let eq19_e296_d_n5: f64 = (eq19_e294_d_n5 + s.dn[244][5]);
        let eq19_e296_d_n6: f64 = (eq19_e294_d_n6 + s.dn[244][6]);
        let eq19_e296_d_n7: f64 = (eq19_e294_d_n7 + s.dn[244][7]);
        let eq19_e296_d_n8: f64 = (eq19_e294_d_n8 + s.dn[244][8]);
        let eq19_e296_d_n9: f64 = (eq19_e294_d_n9 + s.dn[244][9]);
        let eq19_e296_d_n10: f64 = (eq19_e294_d_n10 + s.dn[244][10]);
        let eq19_e296_d_n11: f64 = (eq19_e294_d_n11 + s.dn[244][11]);
        let eq19_e296_d_n12: f64 = (eq19_e294_d_n12 + s.dn[244][12]);
        let eq19_e296_d_b0: f64 = (eq19_e294_d_b0 + s.db[244][0]);
        let eq19_e296_d_b1: f64 = (eq19_e294_d_b1 + s.db[244][1]);
        let eq19_e297: f64 = (p.p3 * eq19_e296);
        let eq19_e297_d_n0: f64 = (p.p3 * eq19_e296_d_n0);
        let eq19_e297_d_n1: f64 = (p.p3 * eq19_e296_d_n1);
        let eq19_e297_d_n2: f64 = (p.p3 * eq19_e296_d_n2);
        let eq19_e297_d_n3: f64 = (p.p3 * eq19_e296_d_n3);
        let eq19_e297_d_n4: f64 = (p.p3 * eq19_e296_d_n4);
        let eq19_e297_d_n5: f64 = (p.p3 * eq19_e296_d_n5);
        let eq19_e297_d_n6: f64 = (p.p3 * eq19_e296_d_n6);
        let eq19_e297_d_n7: f64 = (p.p3 * eq19_e296_d_n7);
        let eq19_e297_d_n8: f64 = (p.p3 * eq19_e296_d_n8);
        let eq19_e297_d_n9: f64 = (p.p3 * eq19_e296_d_n9);
        let eq19_e297_d_n10: f64 = (p.p3 * eq19_e296_d_n10);
        let eq19_e297_d_n11: f64 = (p.p3 * eq19_e296_d_n11);
        let eq19_e297_d_n12: f64 = (p.p3 * eq19_e296_d_n12);
        let eq19_e297_d_b0: f64 = (p.p3 * eq19_e296_d_b0);
        let eq19_e297_d_b1: f64 = (p.p3 * eq19_e296_d_b1);
        let eq19_e298: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 3, eq19_e297);
        let eq19_e298_d_n0: f64 = (eq19_e297_d_n0 * ddt_scale);
        let eq19_e298_d_n1: f64 = (eq19_e297_d_n1 * ddt_scale);
        let eq19_e298_d_n2: f64 = (eq19_e297_d_n2 * ddt_scale);
        let eq19_e298_d_n3: f64 = (eq19_e297_d_n3 * ddt_scale);
        let eq19_e298_d_n4: f64 = (eq19_e297_d_n4 * ddt_scale);
        let eq19_e298_d_n5: f64 = (eq19_e297_d_n5 * ddt_scale);
        let eq19_e298_d_n6: f64 = (eq19_e297_d_n6 * ddt_scale);
        let eq19_e298_d_n7: f64 = (eq19_e297_d_n7 * ddt_scale);
        let eq19_e298_d_n8: f64 = (eq19_e297_d_n8 * ddt_scale);
        let eq19_e298_d_n9: f64 = (eq19_e297_d_n9 * ddt_scale);
        let eq19_e298_d_n10: f64 = (eq19_e297_d_n10 * ddt_scale);
        let eq19_e298_d_n11: f64 = (eq19_e297_d_n11 * ddt_scale);
        let eq19_e298_d_n12: f64 = (eq19_e297_d_n12 * ddt_scale);
        let eq19_e298_d_b0: f64 = (eq19_e297_d_b0 * ddt_scale);
        let eq19_e298_d_b1: f64 = (eq19_e297_d_b1 * ddt_scale);
        let eq19_e300: f64 = (eq19_e298 * p.p1);
        let eq19_e300_d_n0: f64 = (eq19_e298_d_n0 * p.p1);
        let eq19_e300_d_n1: f64 = (eq19_e298_d_n1 * p.p1);
        let eq19_e300_d_n2: f64 = (eq19_e298_d_n2 * p.p1);
        let eq19_e300_d_n3: f64 = (eq19_e298_d_n3 * p.p1);
        let eq19_e300_d_n4: f64 = (eq19_e298_d_n4 * p.p1);
        let eq19_e300_d_n5: f64 = (eq19_e298_d_n5 * p.p1);
        let eq19_e300_d_n6: f64 = (eq19_e298_d_n6 * p.p1);
        let eq19_e300_d_n7: f64 = (eq19_e298_d_n7 * p.p1);
        let eq19_e300_d_n8: f64 = (eq19_e298_d_n8 * p.p1);
        let eq19_e300_d_n9: f64 = (eq19_e298_d_n9 * p.p1);
        let eq19_e300_d_n10: f64 = (eq19_e298_d_n10 * p.p1);
        let eq19_e300_d_n11: f64 = (eq19_e298_d_n11 * p.p1);
        let eq19_e300_d_n12: f64 = (eq19_e298_d_n12 * p.p1);
        let eq19_e300_d_b0: f64 = (eq19_e298_d_b0 * p.p1);
        let eq19_e300_d_b1: f64 = (eq19_e298_d_b1 * p.p1);
        let eq19_value: f64 = eq19_e300;
        let eq19_node_derivatives: [f64; 13] = [eq19_e300_d_n0, eq19_e300_d_n1, eq19_e300_d_n2, eq19_e300_d_n3, eq19_e300_d_n4, eq19_e300_d_n5, eq19_e300_d_n6, eq19_e300_d_n7, eq19_e300_d_n8, eq19_e300_d_n9, eq19_e300_d_n10, eq19_e300_d_n11, eq19_e300_d_n12];
        let eq19_branch_derivatives: [f64; 2] = [eq19_e300_d_b0, eq19_e300_d_b1];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(9),
            multiplicity * (eq19_value),
            &eq19_node_derivatives,
            &eq19_branch_derivatives,
            multiplicity,
        );
        let eq20_e303: f64 = (p.p3 * s.v[239]);
        let eq20_e303_d_n0: f64 = (p.p3 * s.dn[239][0]);
        let eq20_e303_d_n1: f64 = (p.p3 * s.dn[239][1]);
        let eq20_e303_d_n2: f64 = (p.p3 * s.dn[239][2]);
        let eq20_e303_d_n3: f64 = (p.p3 * s.dn[239][3]);
        let eq20_e303_d_n4: f64 = (p.p3 * s.dn[239][4]);
        let eq20_e303_d_n5: f64 = (p.p3 * s.dn[239][5]);
        let eq20_e303_d_n6: f64 = (p.p3 * s.dn[239][6]);
        let eq20_e303_d_n7: f64 = (p.p3 * s.dn[239][7]);
        let eq20_e303_d_n8: f64 = (p.p3 * s.dn[239][8]);
        let eq20_e303_d_n9: f64 = (p.p3 * s.dn[239][9]);
        let eq20_e303_d_n10: f64 = (p.p3 * s.dn[239][10]);
        let eq20_e303_d_n11: f64 = (p.p3 * s.dn[239][11]);
        let eq20_e303_d_n12: f64 = (p.p3 * s.dn[239][12]);
        let eq20_e303_d_b0: f64 = (p.p3 * s.db[239][0]);
        let eq20_e303_d_b1: f64 = (p.p3 * s.db[239][1]);
        let eq20_e304: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 4, eq20_e303);
        let eq20_e304_d_n0: f64 = (eq20_e303_d_n0 * ddt_scale);
        let eq20_e304_d_n1: f64 = (eq20_e303_d_n1 * ddt_scale);
        let eq20_e304_d_n2: f64 = (eq20_e303_d_n2 * ddt_scale);
        let eq20_e304_d_n3: f64 = (eq20_e303_d_n3 * ddt_scale);
        let eq20_e304_d_n4: f64 = (eq20_e303_d_n4 * ddt_scale);
        let eq20_e304_d_n5: f64 = (eq20_e303_d_n5 * ddt_scale);
        let eq20_e304_d_n6: f64 = (eq20_e303_d_n6 * ddt_scale);
        let eq20_e304_d_n7: f64 = (eq20_e303_d_n7 * ddt_scale);
        let eq20_e304_d_n8: f64 = (eq20_e303_d_n8 * ddt_scale);
        let eq20_e304_d_n9: f64 = (eq20_e303_d_n9 * ddt_scale);
        let eq20_e304_d_n10: f64 = (eq20_e303_d_n10 * ddt_scale);
        let eq20_e304_d_n11: f64 = (eq20_e303_d_n11 * ddt_scale);
        let eq20_e304_d_n12: f64 = (eq20_e303_d_n12 * ddt_scale);
        let eq20_e304_d_b0: f64 = (eq20_e303_d_b0 * ddt_scale);
        let eq20_e304_d_b1: f64 = (eq20_e303_d_b1 * ddt_scale);
        let eq20_e306: f64 = (eq20_e304 * p.p1);
        let eq20_e306_d_n0: f64 = (eq20_e304_d_n0 * p.p1);
        let eq20_e306_d_n1: f64 = (eq20_e304_d_n1 * p.p1);
        let eq20_e306_d_n2: f64 = (eq20_e304_d_n2 * p.p1);
        let eq20_e306_d_n3: f64 = (eq20_e304_d_n3 * p.p1);
        let eq20_e306_d_n4: f64 = (eq20_e304_d_n4 * p.p1);
        let eq20_e306_d_n5: f64 = (eq20_e304_d_n5 * p.p1);
        let eq20_e306_d_n6: f64 = (eq20_e304_d_n6 * p.p1);
        let eq20_e306_d_n7: f64 = (eq20_e304_d_n7 * p.p1);
        let eq20_e306_d_n8: f64 = (eq20_e304_d_n8 * p.p1);
        let eq20_e306_d_n9: f64 = (eq20_e304_d_n9 * p.p1);
        let eq20_e306_d_n10: f64 = (eq20_e304_d_n10 * p.p1);
        let eq20_e306_d_n11: f64 = (eq20_e304_d_n11 * p.p1);
        let eq20_e306_d_n12: f64 = (eq20_e304_d_n12 * p.p1);
        let eq20_e306_d_b0: f64 = (eq20_e304_d_b0 * p.p1);
        let eq20_e306_d_b1: f64 = (eq20_e304_d_b1 * p.p1);
        let eq20_value: f64 = eq20_e306;
        let eq20_node_derivatives: [f64; 13] = [eq20_e306_d_n0, eq20_e306_d_n1, eq20_e306_d_n2, eq20_e306_d_n3, eq20_e306_d_n4, eq20_e306_d_n5, eq20_e306_d_n6, eq20_e306_d_n7, eq20_e306_d_n8, eq20_e306_d_n9, eq20_e306_d_n10, eq20_e306_d_n11, eq20_e306_d_n12];
        let eq20_branch_derivatives: [f64; 2] = [eq20_e306_d_b0, eq20_e306_d_b1];
        stamper.stamp_current_dense_local(
            Some(3),
            Some(8),
            multiplicity * (eq20_value),
            &eq20_node_derivatives,
            &eq20_branch_derivatives,
            multiplicity,
        );
        let eq21_e309: f64 = (p.p3 * s.v[228]);
        let eq21_e309_d_n0: f64 = (p.p3 * s.dn[228][0]);
        let eq21_e309_d_n1: f64 = (p.p3 * s.dn[228][1]);
        let eq21_e309_d_n2: f64 = (p.p3 * s.dn[228][2]);
        let eq21_e309_d_n3: f64 = (p.p3 * s.dn[228][3]);
        let eq21_e309_d_n4: f64 = (p.p3 * s.dn[228][4]);
        let eq21_e309_d_n5: f64 = (p.p3 * s.dn[228][5]);
        let eq21_e309_d_n6: f64 = (p.p3 * s.dn[228][6]);
        let eq21_e309_d_n7: f64 = (p.p3 * s.dn[228][7]);
        let eq21_e309_d_n8: f64 = (p.p3 * s.dn[228][8]);
        let eq21_e309_d_n9: f64 = (p.p3 * s.dn[228][9]);
        let eq21_e309_d_n10: f64 = (p.p3 * s.dn[228][10]);
        let eq21_e309_d_n11: f64 = (p.p3 * s.dn[228][11]);
        let eq21_e309_d_n12: f64 = (p.p3 * s.dn[228][12]);
        let eq21_e309_d_b0: f64 = (p.p3 * s.db[228][0]);
        let eq21_e309_d_b1: f64 = (p.p3 * s.db[228][1]);
        let eq21_e310: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 5, eq21_e309);
        let eq21_e310_d_n0: f64 = (eq21_e309_d_n0 * ddt_scale);
        let eq21_e310_d_n1: f64 = (eq21_e309_d_n1 * ddt_scale);
        let eq21_e310_d_n2: f64 = (eq21_e309_d_n2 * ddt_scale);
        let eq21_e310_d_n3: f64 = (eq21_e309_d_n3 * ddt_scale);
        let eq21_e310_d_n4: f64 = (eq21_e309_d_n4 * ddt_scale);
        let eq21_e310_d_n5: f64 = (eq21_e309_d_n5 * ddt_scale);
        let eq21_e310_d_n6: f64 = (eq21_e309_d_n6 * ddt_scale);
        let eq21_e310_d_n7: f64 = (eq21_e309_d_n7 * ddt_scale);
        let eq21_e310_d_n8: f64 = (eq21_e309_d_n8 * ddt_scale);
        let eq21_e310_d_n9: f64 = (eq21_e309_d_n9 * ddt_scale);
        let eq21_e310_d_n10: f64 = (eq21_e309_d_n10 * ddt_scale);
        let eq21_e310_d_n11: f64 = (eq21_e309_d_n11 * ddt_scale);
        let eq21_e310_d_n12: f64 = (eq21_e309_d_n12 * ddt_scale);
        let eq21_e310_d_b0: f64 = (eq21_e309_d_b0 * ddt_scale);
        let eq21_e310_d_b1: f64 = (eq21_e309_d_b1 * ddt_scale);
        let eq21_e312: f64 = (eq21_e310 * p.p1);
        let eq21_e312_d_n0: f64 = (eq21_e310_d_n0 * p.p1);
        let eq21_e312_d_n1: f64 = (eq21_e310_d_n1 * p.p1);
        let eq21_e312_d_n2: f64 = (eq21_e310_d_n2 * p.p1);
        let eq21_e312_d_n3: f64 = (eq21_e310_d_n3 * p.p1);
        let eq21_e312_d_n4: f64 = (eq21_e310_d_n4 * p.p1);
        let eq21_e312_d_n5: f64 = (eq21_e310_d_n5 * p.p1);
        let eq21_e312_d_n6: f64 = (eq21_e310_d_n6 * p.p1);
        let eq21_e312_d_n7: f64 = (eq21_e310_d_n7 * p.p1);
        let eq21_e312_d_n8: f64 = (eq21_e310_d_n8 * p.p1);
        let eq21_e312_d_n9: f64 = (eq21_e310_d_n9 * p.p1);
        let eq21_e312_d_n10: f64 = (eq21_e310_d_n10 * p.p1);
        let eq21_e312_d_n11: f64 = (eq21_e310_d_n11 * p.p1);
        let eq21_e312_d_n12: f64 = (eq21_e310_d_n12 * p.p1);
        let eq21_e312_d_b0: f64 = (eq21_e310_d_b0 * p.p1);
        let eq21_e312_d_b1: f64 = (eq21_e310_d_b1 * p.p1);
        let eq21_value: f64 = eq21_e312;
        let eq21_node_derivatives: [f64; 13] = [eq21_e312_d_n0, eq21_e312_d_n1, eq21_e312_d_n2, eq21_e312_d_n3, eq21_e312_d_n4, eq21_e312_d_n5, eq21_e312_d_n6, eq21_e312_d_n7, eq21_e312_d_n8, eq21_e312_d_n9, eq21_e312_d_n10, eq21_e312_d_n11, eq21_e312_d_n12];
        let eq21_branch_derivatives: [f64; 2] = [eq21_e312_d_b0, eq21_e312_d_b1];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(7),
            multiplicity * (eq21_value),
            &eq21_node_derivatives,
            &eq21_branch_derivatives,
            multiplicity,
        );
        let eq22_e315: f64 = (p.p3 * p.p69);
        let eq22_e317: f64 = (eq22_e315 * s.v[269]);
        let eq22_e317_d_n0: f64 = (eq22_e315 * s.dn[269][0]);
        let eq22_e317_d_n1: f64 = (eq22_e315 * s.dn[269][1]);
        let eq22_e317_d_n2: f64 = (eq22_e315 * s.dn[269][2]);
        let eq22_e317_d_n3: f64 = (eq22_e315 * s.dn[269][3]);
        let eq22_e317_d_n4: f64 = (eq22_e315 * s.dn[269][4]);
        let eq22_e317_d_n5: f64 = (eq22_e315 * s.dn[269][5]);
        let eq22_e317_d_n6: f64 = (eq22_e315 * s.dn[269][6]);
        let eq22_e317_d_n7: f64 = (eq22_e315 * s.dn[269][7]);
        let eq22_e317_d_n8: f64 = (eq22_e315 * s.dn[269][8]);
        let eq22_e317_d_n9: f64 = (eq22_e315 * s.dn[269][9]);
        let eq22_e317_d_n10: f64 = (eq22_e315 * s.dn[269][10]);
        let eq22_e317_d_n11: f64 = (eq22_e315 * s.dn[269][11]);
        let eq22_e317_d_n12: f64 = (eq22_e315 * s.dn[269][12]);
        let eq22_e317_d_b0: f64 = (eq22_e315 * s.db[269][0]);
        let eq22_e317_d_b1: f64 = (eq22_e315 * s.db[269][1]);
        let eq22_e318: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 6, eq22_e317);
        let eq22_e318_d_n0: f64 = (eq22_e317_d_n0 * ddt_scale);
        let eq22_e318_d_n1: f64 = (eq22_e317_d_n1 * ddt_scale);
        let eq22_e318_d_n2: f64 = (eq22_e317_d_n2 * ddt_scale);
        let eq22_e318_d_n3: f64 = (eq22_e317_d_n3 * ddt_scale);
        let eq22_e318_d_n4: f64 = (eq22_e317_d_n4 * ddt_scale);
        let eq22_e318_d_n5: f64 = (eq22_e317_d_n5 * ddt_scale);
        let eq22_e318_d_n6: f64 = (eq22_e317_d_n6 * ddt_scale);
        let eq22_e318_d_n7: f64 = (eq22_e317_d_n7 * ddt_scale);
        let eq22_e318_d_n8: f64 = (eq22_e317_d_n8 * ddt_scale);
        let eq22_e318_d_n9: f64 = (eq22_e317_d_n9 * ddt_scale);
        let eq22_e318_d_n10: f64 = (eq22_e317_d_n10 * ddt_scale);
        let eq22_e318_d_n11: f64 = (eq22_e317_d_n11 * ddt_scale);
        let eq22_e318_d_n12: f64 = (eq22_e317_d_n12 * ddt_scale);
        let eq22_e318_d_b0: f64 = (eq22_e317_d_b0 * ddt_scale);
        let eq22_e318_d_b1: f64 = (eq22_e317_d_b1 * ddt_scale);
        let eq22_e320: f64 = (eq22_e318 * p.p1);
        let eq22_e320_d_n0: f64 = (eq22_e318_d_n0 * p.p1);
        let eq22_e320_d_n1: f64 = (eq22_e318_d_n1 * p.p1);
        let eq22_e320_d_n2: f64 = (eq22_e318_d_n2 * p.p1);
        let eq22_e320_d_n3: f64 = (eq22_e318_d_n3 * p.p1);
        let eq22_e320_d_n4: f64 = (eq22_e318_d_n4 * p.p1);
        let eq22_e320_d_n5: f64 = (eq22_e318_d_n5 * p.p1);
        let eq22_e320_d_n6: f64 = (eq22_e318_d_n6 * p.p1);
        let eq22_e320_d_n7: f64 = (eq22_e318_d_n7 * p.p1);
        let eq22_e320_d_n8: f64 = (eq22_e318_d_n8 * p.p1);
        let eq22_e320_d_n9: f64 = (eq22_e318_d_n9 * p.p1);
        let eq22_e320_d_n10: f64 = (eq22_e318_d_n10 * p.p1);
        let eq22_e320_d_n11: f64 = (eq22_e318_d_n11 * p.p1);
        let eq22_e320_d_n12: f64 = (eq22_e318_d_n12 * p.p1);
        let eq22_e320_d_b0: f64 = (eq22_e318_d_b0 * p.p1);
        let eq22_e320_d_b1: f64 = (eq22_e318_d_b1 * p.p1);
        let eq22_value: f64 = eq22_e320;
        let eq22_node_derivatives: [f64; 13] = [eq22_e320_d_n0, eq22_e320_d_n1, eq22_e320_d_n2, eq22_e320_d_n3, eq22_e320_d_n4, eq22_e320_d_n5, eq22_e320_d_n6, eq22_e320_d_n7, eq22_e320_d_n8, eq22_e320_d_n9, eq22_e320_d_n10, eq22_e320_d_n11, eq22_e320_d_n12];
        let eq22_branch_derivatives: [f64; 2] = [eq22_e320_d_b0, eq22_e320_d_b1];
        stamper.stamp_current_dense_local(
            Some(1),
            Some(2),
            multiplicity * (eq22_value),
            &eq22_node_derivatives,
            &eq22_branch_derivatives,
            multiplicity,
        );
        let eq23_e323: f64 = (p.p3 * p.p78);
        let eq23_e325: f64 = (eq23_e323 * s.v[270]);
        let eq23_e325_d_n0: f64 = (eq23_e323 * s.dn[270][0]);
        let eq23_e325_d_n1: f64 = (eq23_e323 * s.dn[270][1]);
        let eq23_e325_d_n2: f64 = (eq23_e323 * s.dn[270][2]);
        let eq23_e325_d_n3: f64 = (eq23_e323 * s.dn[270][3]);
        let eq23_e325_d_n4: f64 = (eq23_e323 * s.dn[270][4]);
        let eq23_e325_d_n5: f64 = (eq23_e323 * s.dn[270][5]);
        let eq23_e325_d_n6: f64 = (eq23_e323 * s.dn[270][6]);
        let eq23_e325_d_n7: f64 = (eq23_e323 * s.dn[270][7]);
        let eq23_e325_d_n8: f64 = (eq23_e323 * s.dn[270][8]);
        let eq23_e325_d_n9: f64 = (eq23_e323 * s.dn[270][9]);
        let eq23_e325_d_n10: f64 = (eq23_e323 * s.dn[270][10]);
        let eq23_e325_d_n11: f64 = (eq23_e323 * s.dn[270][11]);
        let eq23_e325_d_n12: f64 = (eq23_e323 * s.dn[270][12]);
        let eq23_e325_d_b0: f64 = (eq23_e323 * s.db[270][0]);
        let eq23_e325_d_b1: f64 = (eq23_e323 * s.db[270][1]);
        let eq23_e326: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 7, eq23_e325);
        let eq23_e326_d_n0: f64 = (eq23_e325_d_n0 * ddt_scale);
        let eq23_e326_d_n1: f64 = (eq23_e325_d_n1 * ddt_scale);
        let eq23_e326_d_n2: f64 = (eq23_e325_d_n2 * ddt_scale);
        let eq23_e326_d_n3: f64 = (eq23_e325_d_n3 * ddt_scale);
        let eq23_e326_d_n4: f64 = (eq23_e325_d_n4 * ddt_scale);
        let eq23_e326_d_n5: f64 = (eq23_e325_d_n5 * ddt_scale);
        let eq23_e326_d_n6: f64 = (eq23_e325_d_n6 * ddt_scale);
        let eq23_e326_d_n7: f64 = (eq23_e325_d_n7 * ddt_scale);
        let eq23_e326_d_n8: f64 = (eq23_e325_d_n8 * ddt_scale);
        let eq23_e326_d_n9: f64 = (eq23_e325_d_n9 * ddt_scale);
        let eq23_e326_d_n10: f64 = (eq23_e325_d_n10 * ddt_scale);
        let eq23_e326_d_n11: f64 = (eq23_e325_d_n11 * ddt_scale);
        let eq23_e326_d_n12: f64 = (eq23_e325_d_n12 * ddt_scale);
        let eq23_e326_d_b0: f64 = (eq23_e325_d_b0 * ddt_scale);
        let eq23_e326_d_b1: f64 = (eq23_e325_d_b1 * ddt_scale);
        let eq23_e328: f64 = (eq23_e326 * p.p1);
        let eq23_e328_d_n0: f64 = (eq23_e326_d_n0 * p.p1);
        let eq23_e328_d_n1: f64 = (eq23_e326_d_n1 * p.p1);
        let eq23_e328_d_n2: f64 = (eq23_e326_d_n2 * p.p1);
        let eq23_e328_d_n3: f64 = (eq23_e326_d_n3 * p.p1);
        let eq23_e328_d_n4: f64 = (eq23_e326_d_n4 * p.p1);
        let eq23_e328_d_n5: f64 = (eq23_e326_d_n5 * p.p1);
        let eq23_e328_d_n6: f64 = (eq23_e326_d_n6 * p.p1);
        let eq23_e328_d_n7: f64 = (eq23_e326_d_n7 * p.p1);
        let eq23_e328_d_n8: f64 = (eq23_e326_d_n8 * p.p1);
        let eq23_e328_d_n9: f64 = (eq23_e326_d_n9 * p.p1);
        let eq23_e328_d_n10: f64 = (eq23_e326_d_n10 * p.p1);
        let eq23_e328_d_n11: f64 = (eq23_e326_d_n11 * p.p1);
        let eq23_e328_d_n12: f64 = (eq23_e326_d_n12 * p.p1);
        let eq23_e328_d_b0: f64 = (eq23_e326_d_b0 * p.p1);
        let eq23_e328_d_b1: f64 = (eq23_e326_d_b1 * p.p1);
        let eq23_value: f64 = eq23_e328;
        let eq23_node_derivatives: [f64; 13] = [eq23_e328_d_n0, eq23_e328_d_n1, eq23_e328_d_n2, eq23_e328_d_n3, eq23_e328_d_n4, eq23_e328_d_n5, eq23_e328_d_n6, eq23_e328_d_n7, eq23_e328_d_n8, eq23_e328_d_n9, eq23_e328_d_n10, eq23_e328_d_n11, eq23_e328_d_n12];
        let eq23_branch_derivatives: [f64; 2] = [eq23_e328_d_b0, eq23_e328_d_b1];
        stamper.stamp_current_dense_local(
            Some(1),
            Some(0),
            multiplicity * (eq23_value),
            &eq23_node_derivatives,
            &eq23_branch_derivatives,
            multiplicity,
        );
        let eq24_e331: f64 = (p.p3 * s.v[179]);
        let eq24_e331_d_n0: f64 = (p.p3 * s.dn[179][0]);
        let eq24_e331_d_n1: f64 = (p.p3 * s.dn[179][1]);
        let eq24_e331_d_n2: f64 = (p.p3 * s.dn[179][2]);
        let eq24_e331_d_n3: f64 = (p.p3 * s.dn[179][3]);
        let eq24_e331_d_n4: f64 = (p.p3 * s.dn[179][4]);
        let eq24_e331_d_n5: f64 = (p.p3 * s.dn[179][5]);
        let eq24_e331_d_n6: f64 = (p.p3 * s.dn[179][6]);
        let eq24_e331_d_n7: f64 = (p.p3 * s.dn[179][7]);
        let eq24_e331_d_n8: f64 = (p.p3 * s.dn[179][8]);
        let eq24_e331_d_n9: f64 = (p.p3 * s.dn[179][9]);
        let eq24_e331_d_n10: f64 = (p.p3 * s.dn[179][10]);
        let eq24_e331_d_n11: f64 = (p.p3 * s.dn[179][11]);
        let eq24_e331_d_n12: f64 = (p.p3 * s.dn[179][12]);
        let eq24_e331_d_b0: f64 = (p.p3 * s.db[179][0]);
        let eq24_e331_d_b1: f64 = (p.p3 * s.db[179][1]);
        let eq24_e333: f64 = (eq24_e331 * p.p1);
        let eq24_e333_d_n0: f64 = (eq24_e331_d_n0 * p.p1);
        let eq24_e333_d_n1: f64 = (eq24_e331_d_n1 * p.p1);
        let eq24_e333_d_n2: f64 = (eq24_e331_d_n2 * p.p1);
        let eq24_e333_d_n3: f64 = (eq24_e331_d_n3 * p.p1);
        let eq24_e333_d_n4: f64 = (eq24_e331_d_n4 * p.p1);
        let eq24_e333_d_n5: f64 = (eq24_e331_d_n5 * p.p1);
        let eq24_e333_d_n6: f64 = (eq24_e331_d_n6 * p.p1);
        let eq24_e333_d_n7: f64 = (eq24_e331_d_n7 * p.p1);
        let eq24_e333_d_n8: f64 = (eq24_e331_d_n8 * p.p1);
        let eq24_e333_d_n9: f64 = (eq24_e331_d_n9 * p.p1);
        let eq24_e333_d_n10: f64 = (eq24_e331_d_n10 * p.p1);
        let eq24_e333_d_n11: f64 = (eq24_e331_d_n11 * p.p1);
        let eq24_e333_d_n12: f64 = (eq24_e331_d_n12 * p.p1);
        let eq24_e333_d_b0: f64 = (eq24_e331_d_b0 * p.p1);
        let eq24_e333_d_b1: f64 = (eq24_e331_d_b1 * p.p1);
        let eq24_value: f64 = eq24_e333;
        let eq24_node_derivatives: [f64; 13] = [eq24_e333_d_n0, eq24_e333_d_n1, eq24_e333_d_n2, eq24_e333_d_n3, eq24_e333_d_n4, eq24_e333_d_n5, eq24_e333_d_n6, eq24_e333_d_n7, eq24_e333_d_n8, eq24_e333_d_n9, eq24_e333_d_n10, eq24_e333_d_n11, eq24_e333_d_n12];
        let eq24_branch_derivatives: [f64; 2] = [eq24_e333_d_b0, eq24_e333_d_b1];
        stamper.stamp_current_dense_local(
            Some(1),
            Some(10),
            multiplicity * (eq24_value),
            &eq24_node_derivatives,
            &eq24_branch_derivatives,
            multiplicity,
        );
        let eq25_e336: f64 = (p.p3 * s.v[268]);
        let eq25_e336_d_n0: f64 = (p.p3 * s.dn[268][0]);
        let eq25_e336_d_n1: f64 = (p.p3 * s.dn[268][1]);
        let eq25_e336_d_n2: f64 = (p.p3 * s.dn[268][2]);
        let eq25_e336_d_n3: f64 = (p.p3 * s.dn[268][3]);
        let eq25_e336_d_n4: f64 = (p.p3 * s.dn[268][4]);
        let eq25_e336_d_n5: f64 = (p.p3 * s.dn[268][5]);
        let eq25_e336_d_n6: f64 = (p.p3 * s.dn[268][6]);
        let eq25_e336_d_n7: f64 = (p.p3 * s.dn[268][7]);
        let eq25_e336_d_n8: f64 = (p.p3 * s.dn[268][8]);
        let eq25_e336_d_n9: f64 = (p.p3 * s.dn[268][9]);
        let eq25_e336_d_n10: f64 = (p.p3 * s.dn[268][10]);
        let eq25_e336_d_n11: f64 = (p.p3 * s.dn[268][11]);
        let eq25_e336_d_n12: f64 = (p.p3 * s.dn[268][12]);
        let eq25_e336_d_b0: f64 = (p.p3 * s.db[268][0]);
        let eq25_e336_d_b1: f64 = (p.p3 * s.db[268][1]);
        let eq25_e338: f64 = (eq25_e336 * s.v[111]);
        let eq25_e338_d_n0: f64 = ((eq25_e336_d_n0 * s.v[111]) + (eq25_e336 * s.dn[111][0]));
        let eq25_e338_d_n1: f64 = ((eq25_e336_d_n1 * s.v[111]) + (eq25_e336 * s.dn[111][1]));
        let eq25_e338_d_n2: f64 = ((eq25_e336_d_n2 * s.v[111]) + (eq25_e336 * s.dn[111][2]));
        let eq25_e338_d_n3: f64 = ((eq25_e336_d_n3 * s.v[111]) + (eq25_e336 * s.dn[111][3]));
        let eq25_e338_d_n4: f64 = ((eq25_e336_d_n4 * s.v[111]) + (eq25_e336 * s.dn[111][4]));
        let eq25_e338_d_n5: f64 = ((eq25_e336_d_n5 * s.v[111]) + (eq25_e336 * s.dn[111][5]));
        let eq25_e338_d_n6: f64 = ((eq25_e336_d_n6 * s.v[111]) + (eq25_e336 * s.dn[111][6]));
        let eq25_e338_d_n7: f64 = ((eq25_e336_d_n7 * s.v[111]) + (eq25_e336 * s.dn[111][7]));
        let eq25_e338_d_n8: f64 = ((eq25_e336_d_n8 * s.v[111]) + (eq25_e336 * s.dn[111][8]));
        let eq25_e338_d_n9: f64 = ((eq25_e336_d_n9 * s.v[111]) + (eq25_e336 * s.dn[111][9]));
        let eq25_e338_d_n10: f64 = ((eq25_e336_d_n10 * s.v[111]) + (eq25_e336 * s.dn[111][10]));
        let eq25_e338_d_n11: f64 = ((eq25_e336_d_n11 * s.v[111]) + (eq25_e336 * s.dn[111][11]));
        let eq25_e338_d_n12: f64 = ((eq25_e336_d_n12 * s.v[111]) + (eq25_e336 * s.dn[111][12]));
        let eq25_e338_d_b0: f64 = ((eq25_e336_d_b0 * s.v[111]) + (eq25_e336 * s.db[111][0]));
        let eq25_e338_d_b1: f64 = ((eq25_e336_d_b1 * s.v[111]) + (eq25_e336 * s.db[111][1]));
        let eq25_e340: f64 = (eq25_e338 * p.p1);
        let eq25_e340_d_n0: f64 = (eq25_e338_d_n0 * p.p1);
        let eq25_e340_d_n1: f64 = (eq25_e338_d_n1 * p.p1);
        let eq25_e340_d_n2: f64 = (eq25_e338_d_n2 * p.p1);
        let eq25_e340_d_n3: f64 = (eq25_e338_d_n3 * p.p1);
        let eq25_e340_d_n4: f64 = (eq25_e338_d_n4 * p.p1);
        let eq25_e340_d_n5: f64 = (eq25_e338_d_n5 * p.p1);
        let eq25_e340_d_n6: f64 = (eq25_e338_d_n6 * p.p1);
        let eq25_e340_d_n7: f64 = (eq25_e338_d_n7 * p.p1);
        let eq25_e340_d_n8: f64 = (eq25_e338_d_n8 * p.p1);
        let eq25_e340_d_n9: f64 = (eq25_e338_d_n9 * p.p1);
        let eq25_e340_d_n10: f64 = (eq25_e338_d_n10 * p.p1);
        let eq25_e340_d_n11: f64 = (eq25_e338_d_n11 * p.p1);
        let eq25_e340_d_n12: f64 = (eq25_e338_d_n12 * p.p1);
        let eq25_e340_d_b0: f64 = (eq25_e338_d_b0 * p.p1);
        let eq25_e340_d_b1: f64 = (eq25_e338_d_b1 * p.p1);
        let eq25_value: f64 = eq25_e340;
        let eq25_node_derivatives: [f64; 13] = [eq25_e340_d_n0, eq25_e340_d_n1, eq25_e340_d_n2, eq25_e340_d_n3, eq25_e340_d_n4, eq25_e340_d_n5, eq25_e340_d_n6, eq25_e340_d_n7, eq25_e340_d_n8, eq25_e340_d_n9, eq25_e340_d_n10, eq25_e340_d_n11, eq25_e340_d_n12];
        let eq25_branch_derivatives: [f64; 2] = [eq25_e340_d_b0, eq25_e340_d_b1];
        stamper.stamp_current_dense_local(
            Some(0),
            Some(10),
            multiplicity * (eq25_value),
            &eq25_node_derivatives,
            &eq25_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_3(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
    ) {
        let nv12 = ctx.node_voltage(nodes[12]);
        let eq26_e344: f64 = (s.v[236] + s.v[248]);
        let eq26_e344_d_n0: f64 = (s.dn[236][0] + s.dn[248][0]);
        let eq26_e344_d_n1: f64 = (s.dn[236][1] + s.dn[248][1]);
        let eq26_e344_d_n2: f64 = (s.dn[236][2] + s.dn[248][2]);
        let eq26_e344_d_n3: f64 = (s.dn[236][3] + s.dn[248][3]);
        let eq26_e344_d_n4: f64 = (s.dn[236][4] + s.dn[248][4]);
        let eq26_e344_d_n5: f64 = (s.dn[236][5] + s.dn[248][5]);
        let eq26_e344_d_n6: f64 = (s.dn[236][6] + s.dn[248][6]);
        let eq26_e344_d_n7: f64 = (s.dn[236][7] + s.dn[248][7]);
        let eq26_e344_d_n8: f64 = (s.dn[236][8] + s.dn[248][8]);
        let eq26_e344_d_n9: f64 = (s.dn[236][9] + s.dn[248][9]);
        let eq26_e344_d_n10: f64 = (s.dn[236][10] + s.dn[248][10]);
        let eq26_e344_d_n11: f64 = (s.dn[236][11] + s.dn[248][11]);
        let eq26_e344_d_n12: f64 = (s.dn[236][12] + s.dn[248][12]);
        let eq26_e344_d_b0: f64 = (s.db[236][0] + s.db[248][0]);
        let eq26_e344_d_b1: f64 = (s.db[236][1] + s.db[248][1]);
        let eq26_e345: f64 = (p.p3 * eq26_e344);
        let eq26_e345_d_n0: f64 = (p.p3 * eq26_e344_d_n0);
        let eq26_e345_d_n1: f64 = (p.p3 * eq26_e344_d_n1);
        let eq26_e345_d_n2: f64 = (p.p3 * eq26_e344_d_n2);
        let eq26_e345_d_n3: f64 = (p.p3 * eq26_e344_d_n3);
        let eq26_e345_d_n4: f64 = (p.p3 * eq26_e344_d_n4);
        let eq26_e345_d_n5: f64 = (p.p3 * eq26_e344_d_n5);
        let eq26_e345_d_n6: f64 = (p.p3 * eq26_e344_d_n6);
        let eq26_e345_d_n7: f64 = (p.p3 * eq26_e344_d_n7);
        let eq26_e345_d_n8: f64 = (p.p3 * eq26_e344_d_n8);
        let eq26_e345_d_n9: f64 = (p.p3 * eq26_e344_d_n9);
        let eq26_e345_d_n10: f64 = (p.p3 * eq26_e344_d_n10);
        let eq26_e345_d_n11: f64 = (p.p3 * eq26_e344_d_n11);
        let eq26_e345_d_n12: f64 = (p.p3 * eq26_e344_d_n12);
        let eq26_e345_d_b0: f64 = (p.p3 * eq26_e344_d_b0);
        let eq26_e345_d_b1: f64 = (p.p3 * eq26_e344_d_b1);
        let eq26_e346: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 8, eq26_e345);
        let eq26_e346_d_n0: f64 = (eq26_e345_d_n0 * ddt_scale);
        let eq26_e346_d_n1: f64 = (eq26_e345_d_n1 * ddt_scale);
        let eq26_e346_d_n2: f64 = (eq26_e345_d_n2 * ddt_scale);
        let eq26_e346_d_n3: f64 = (eq26_e345_d_n3 * ddt_scale);
        let eq26_e346_d_n4: f64 = (eq26_e345_d_n4 * ddt_scale);
        let eq26_e346_d_n5: f64 = (eq26_e345_d_n5 * ddt_scale);
        let eq26_e346_d_n6: f64 = (eq26_e345_d_n6 * ddt_scale);
        let eq26_e346_d_n7: f64 = (eq26_e345_d_n7 * ddt_scale);
        let eq26_e346_d_n8: f64 = (eq26_e345_d_n8 * ddt_scale);
        let eq26_e346_d_n9: f64 = (eq26_e345_d_n9 * ddt_scale);
        let eq26_e346_d_n10: f64 = (eq26_e345_d_n10 * ddt_scale);
        let eq26_e346_d_n11: f64 = (eq26_e345_d_n11 * ddt_scale);
        let eq26_e346_d_n12: f64 = (eq26_e345_d_n12 * ddt_scale);
        let eq26_e346_d_b0: f64 = (eq26_e345_d_b0 * ddt_scale);
        let eq26_e346_d_b1: f64 = (eq26_e345_d_b1 * ddt_scale);
        let eq26_e348: f64 = (eq26_e346 * p.p1);
        let eq26_e348_d_n0: f64 = (eq26_e346_d_n0 * p.p1);
        let eq26_e348_d_n1: f64 = (eq26_e346_d_n1 * p.p1);
        let eq26_e348_d_n2: f64 = (eq26_e346_d_n2 * p.p1);
        let eq26_e348_d_n3: f64 = (eq26_e346_d_n3 * p.p1);
        let eq26_e348_d_n4: f64 = (eq26_e346_d_n4 * p.p1);
        let eq26_e348_d_n5: f64 = (eq26_e346_d_n5 * p.p1);
        let eq26_e348_d_n6: f64 = (eq26_e346_d_n6 * p.p1);
        let eq26_e348_d_n7: f64 = (eq26_e346_d_n7 * p.p1);
        let eq26_e348_d_n8: f64 = (eq26_e346_d_n8 * p.p1);
        let eq26_e348_d_n9: f64 = (eq26_e346_d_n9 * p.p1);
        let eq26_e348_d_n10: f64 = (eq26_e346_d_n10 * p.p1);
        let eq26_e348_d_n11: f64 = (eq26_e346_d_n11 * p.p1);
        let eq26_e348_d_n12: f64 = (eq26_e346_d_n12 * p.p1);
        let eq26_e348_d_b0: f64 = (eq26_e346_d_b0 * p.p1);
        let eq26_e348_d_b1: f64 = (eq26_e346_d_b1 * p.p1);
        let eq26_value: f64 = eq26_e348;
        let eq26_node_derivatives: [f64; 13] = [eq26_e348_d_n0, eq26_e348_d_n1, eq26_e348_d_n2, eq26_e348_d_n3, eq26_e348_d_n4, eq26_e348_d_n5, eq26_e348_d_n6, eq26_e348_d_n7, eq26_e348_d_n8, eq26_e348_d_n9, eq26_e348_d_n10, eq26_e348_d_n11, eq26_e348_d_n12];
        let eq26_branch_derivatives: [f64; 2] = [eq26_e348_d_b0, eq26_e348_d_b1];
        stamper.stamp_current_dense_local(
            Some(1),
            Some(10),
            multiplicity * (eq26_value),
            &eq26_node_derivatives,
            &eq26_branch_derivatives,
            multiplicity,
        );
        let eq27_e353: f64 = (s.v[344] * s.v[255]);
        let eq27_e353_d_n0: f64 = (s.v[344] * s.dn[255][0]);
        let eq27_e353_d_n1: f64 = (s.v[344] * s.dn[255][1]);
        let eq27_e353_d_n2: f64 = (s.v[344] * s.dn[255][2]);
        let eq27_e353_d_n3: f64 = (s.v[344] * s.dn[255][3]);
        let eq27_e353_d_n4: f64 = (s.v[344] * s.dn[255][4]);
        let eq27_e353_d_n5: f64 = (s.v[344] * s.dn[255][5]);
        let eq27_e353_d_n6: f64 = (s.v[344] * s.dn[255][6]);
        let eq27_e353_d_n7: f64 = (s.v[344] * s.dn[255][7]);
        let eq27_e353_d_n8: f64 = (s.v[344] * s.dn[255][8]);
        let eq27_e353_d_n9: f64 = (s.v[344] * s.dn[255][9]);
        let eq27_e353_d_n10: f64 = (s.v[344] * s.dn[255][10]);
        let eq27_e353_d_n11: f64 = (s.v[344] * s.dn[255][11]);
        let eq27_e353_d_n12: f64 = (s.v[344] * s.dn[255][12]);
        let eq27_e353_d_b0: f64 = (s.v[344] * s.db[255][0]);
        let eq27_e353_d_b1: f64 = (s.v[344] * s.db[255][1]);
        let eq27_e354: f64 = (s.v[164] + eq27_e353);
        let eq27_e354_d_n0: f64 = (s.dn[164][0] + eq27_e353_d_n0);
        let eq27_e354_d_n1: f64 = (s.dn[164][1] + eq27_e353_d_n1);
        let eq27_e354_d_n2: f64 = (s.dn[164][2] + eq27_e353_d_n2);
        let eq27_e354_d_n3: f64 = (s.dn[164][3] + eq27_e353_d_n3);
        let eq27_e354_d_n4: f64 = (s.dn[164][4] + eq27_e353_d_n4);
        let eq27_e354_d_n5: f64 = (s.dn[164][5] + eq27_e353_d_n5);
        let eq27_e354_d_n6: f64 = (s.dn[164][6] + eq27_e353_d_n6);
        let eq27_e354_d_n7: f64 = (s.dn[164][7] + eq27_e353_d_n7);
        let eq27_e354_d_n8: f64 = (s.dn[164][8] + eq27_e353_d_n8);
        let eq27_e354_d_n9: f64 = (s.dn[164][9] + eq27_e353_d_n9);
        let eq27_e354_d_n10: f64 = (s.dn[164][10] + eq27_e353_d_n10);
        let eq27_e354_d_n11: f64 = (s.dn[164][11] + eq27_e353_d_n11);
        let eq27_e354_d_n12: f64 = (s.dn[164][12] + eq27_e353_d_n12);
        let eq27_e354_d_b0: f64 = (s.db[164][0] + eq27_e353_d_b0);
        let eq27_e354_d_b1: f64 = (s.db[164][1] + eq27_e353_d_b1);
        let eq27_e356: f64 = (eq27_e354 + s.v[167]);
        let eq27_e356_d_n0: f64 = (eq27_e354_d_n0 + s.dn[167][0]);
        let eq27_e356_d_n1: f64 = (eq27_e354_d_n1 + s.dn[167][1]);
        let eq27_e356_d_n2: f64 = (eq27_e354_d_n2 + s.dn[167][2]);
        let eq27_e356_d_n3: f64 = (eq27_e354_d_n3 + s.dn[167][3]);
        let eq27_e356_d_n4: f64 = (eq27_e354_d_n4 + s.dn[167][4]);
        let eq27_e356_d_n5: f64 = (eq27_e354_d_n5 + s.dn[167][5]);
        let eq27_e356_d_n6: f64 = (eq27_e354_d_n6 + s.dn[167][6]);
        let eq27_e356_d_n7: f64 = (eq27_e354_d_n7 + s.dn[167][7]);
        let eq27_e356_d_n8: f64 = (eq27_e354_d_n8 + s.dn[167][8]);
        let eq27_e356_d_n9: f64 = (eq27_e354_d_n9 + s.dn[167][9]);
        let eq27_e356_d_n10: f64 = (eq27_e354_d_n10 + s.dn[167][10]);
        let eq27_e356_d_n11: f64 = (eq27_e354_d_n11 + s.dn[167][11]);
        let eq27_e356_d_n12: f64 = (eq27_e354_d_n12 + s.dn[167][12]);
        let eq27_e356_d_b0: f64 = (eq27_e354_d_b0 + s.db[167][0]);
        let eq27_e356_d_b1: f64 = (eq27_e354_d_b1 + s.db[167][1]);
        let eq27_e357: f64 = (p.p3 * eq27_e356);
        let eq27_e357_d_n0: f64 = (p.p3 * eq27_e356_d_n0);
        let eq27_e357_d_n1: f64 = (p.p3 * eq27_e356_d_n1);
        let eq27_e357_d_n2: f64 = (p.p3 * eq27_e356_d_n2);
        let eq27_e357_d_n3: f64 = (p.p3 * eq27_e356_d_n3);
        let eq27_e357_d_n4: f64 = (p.p3 * eq27_e356_d_n4);
        let eq27_e357_d_n5: f64 = (p.p3 * eq27_e356_d_n5);
        let eq27_e357_d_n6: f64 = (p.p3 * eq27_e356_d_n6);
        let eq27_e357_d_n7: f64 = (p.p3 * eq27_e356_d_n7);
        let eq27_e357_d_n8: f64 = (p.p3 * eq27_e356_d_n8);
        let eq27_e357_d_n9: f64 = (p.p3 * eq27_e356_d_n9);
        let eq27_e357_d_n10: f64 = (p.p3 * eq27_e356_d_n10);
        let eq27_e357_d_n11: f64 = (p.p3 * eq27_e356_d_n11);
        let eq27_e357_d_n12: f64 = (p.p3 * eq27_e356_d_n12);
        let eq27_e357_d_b0: f64 = (p.p3 * eq27_e356_d_b0);
        let eq27_e357_d_b1: f64 = (p.p3 * eq27_e356_d_b1);
        let eq27_e359: f64 = (eq27_e357 * p.p1);
        let eq27_e359_d_n0: f64 = (eq27_e357_d_n0 * p.p1);
        let eq27_e359_d_n1: f64 = (eq27_e357_d_n1 * p.p1);
        let eq27_e359_d_n2: f64 = (eq27_e357_d_n2 * p.p1);
        let eq27_e359_d_n3: f64 = (eq27_e357_d_n3 * p.p1);
        let eq27_e359_d_n4: f64 = (eq27_e357_d_n4 * p.p1);
        let eq27_e359_d_n5: f64 = (eq27_e357_d_n5 * p.p1);
        let eq27_e359_d_n6: f64 = (eq27_e357_d_n6 * p.p1);
        let eq27_e359_d_n7: f64 = (eq27_e357_d_n7 * p.p1);
        let eq27_e359_d_n8: f64 = (eq27_e357_d_n8 * p.p1);
        let eq27_e359_d_n9: f64 = (eq27_e357_d_n9 * p.p1);
        let eq27_e359_d_n10: f64 = (eq27_e357_d_n10 * p.p1);
        let eq27_e359_d_n11: f64 = (eq27_e357_d_n11 * p.p1);
        let eq27_e359_d_n12: f64 = (eq27_e357_d_n12 * p.p1);
        let eq27_e359_d_b0: f64 = (eq27_e357_d_b0 * p.p1);
        let eq27_e359_d_b1: f64 = (eq27_e357_d_b1 * p.p1);
        let eq27_value: f64 = eq27_e359;
        let eq27_node_derivatives: [f64; 13] = [eq27_e359_d_n0, eq27_e359_d_n1, eq27_e359_d_n2, eq27_e359_d_n3, eq27_e359_d_n4, eq27_e359_d_n5, eq27_e359_d_n6, eq27_e359_d_n7, eq27_e359_d_n8, eq27_e359_d_n9, eq27_e359_d_n10, eq27_e359_d_n11, eq27_e359_d_n12];
        let eq27_branch_derivatives: [f64; 2] = [eq27_e359_d_b0, eq27_e359_d_b1];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(11),
            multiplicity * (eq27_value),
            &eq27_node_derivatives,
            &eq27_branch_derivatives,
            multiplicity,
        );
        let eq28_e363: f64 = (s.v[233] + s.v[249]);
        let eq28_e363_d_n0: f64 = (s.dn[233][0] + s.dn[249][0]);
        let eq28_e363_d_n1: f64 = (s.dn[233][1] + s.dn[249][1]);
        let eq28_e363_d_n2: f64 = (s.dn[233][2] + s.dn[249][2]);
        let eq28_e363_d_n3: f64 = (s.dn[233][3] + s.dn[249][3]);
        let eq28_e363_d_n4: f64 = (s.dn[233][4] + s.dn[249][4]);
        let eq28_e363_d_n5: f64 = (s.dn[233][5] + s.dn[249][5]);
        let eq28_e363_d_n6: f64 = (s.dn[233][6] + s.dn[249][6]);
        let eq28_e363_d_n7: f64 = (s.dn[233][7] + s.dn[249][7]);
        let eq28_e363_d_n8: f64 = (s.dn[233][8] + s.dn[249][8]);
        let eq28_e363_d_n9: f64 = (s.dn[233][9] + s.dn[249][9]);
        let eq28_e363_d_n10: f64 = (s.dn[233][10] + s.dn[249][10]);
        let eq28_e363_d_n11: f64 = (s.dn[233][11] + s.dn[249][11]);
        let eq28_e363_d_n12: f64 = (s.dn[233][12] + s.dn[249][12]);
        let eq28_e363_d_b0: f64 = (s.db[233][0] + s.db[249][0]);
        let eq28_e363_d_b1: f64 = (s.db[233][1] + s.db[249][1]);
        let eq28_e364: f64 = (p.p3 * eq28_e363);
        let eq28_e364_d_n0: f64 = (p.p3 * eq28_e363_d_n0);
        let eq28_e364_d_n1: f64 = (p.p3 * eq28_e363_d_n1);
        let eq28_e364_d_n2: f64 = (p.p3 * eq28_e363_d_n2);
        let eq28_e364_d_n3: f64 = (p.p3 * eq28_e363_d_n3);
        let eq28_e364_d_n4: f64 = (p.p3 * eq28_e363_d_n4);
        let eq28_e364_d_n5: f64 = (p.p3 * eq28_e363_d_n5);
        let eq28_e364_d_n6: f64 = (p.p3 * eq28_e363_d_n6);
        let eq28_e364_d_n7: f64 = (p.p3 * eq28_e363_d_n7);
        let eq28_e364_d_n8: f64 = (p.p3 * eq28_e363_d_n8);
        let eq28_e364_d_n9: f64 = (p.p3 * eq28_e363_d_n9);
        let eq28_e364_d_n10: f64 = (p.p3 * eq28_e363_d_n10);
        let eq28_e364_d_n11: f64 = (p.p3 * eq28_e363_d_n11);
        let eq28_e364_d_n12: f64 = (p.p3 * eq28_e363_d_n12);
        let eq28_e364_d_b0: f64 = (p.p3 * eq28_e363_d_b0);
        let eq28_e364_d_b1: f64 = (p.p3 * eq28_e363_d_b1);
        let eq28_e365: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 9, eq28_e364);
        let eq28_e365_d_n0: f64 = (eq28_e364_d_n0 * ddt_scale);
        let eq28_e365_d_n1: f64 = (eq28_e364_d_n1 * ddt_scale);
        let eq28_e365_d_n2: f64 = (eq28_e364_d_n2 * ddt_scale);
        let eq28_e365_d_n3: f64 = (eq28_e364_d_n3 * ddt_scale);
        let eq28_e365_d_n4: f64 = (eq28_e364_d_n4 * ddt_scale);
        let eq28_e365_d_n5: f64 = (eq28_e364_d_n5 * ddt_scale);
        let eq28_e365_d_n6: f64 = (eq28_e364_d_n6 * ddt_scale);
        let eq28_e365_d_n7: f64 = (eq28_e364_d_n7 * ddt_scale);
        let eq28_e365_d_n8: f64 = (eq28_e364_d_n8 * ddt_scale);
        let eq28_e365_d_n9: f64 = (eq28_e364_d_n9 * ddt_scale);
        let eq28_e365_d_n10: f64 = (eq28_e364_d_n10 * ddt_scale);
        let eq28_e365_d_n11: f64 = (eq28_e364_d_n11 * ddt_scale);
        let eq28_e365_d_n12: f64 = (eq28_e364_d_n12 * ddt_scale);
        let eq28_e365_d_b0: f64 = (eq28_e364_d_b0 * ddt_scale);
        let eq28_e365_d_b1: f64 = (eq28_e364_d_b1 * ddt_scale);
        let eq28_e367: f64 = (eq28_e365 * p.p1);
        let eq28_e367_d_n0: f64 = (eq28_e365_d_n0 * p.p1);
        let eq28_e367_d_n1: f64 = (eq28_e365_d_n1 * p.p1);
        let eq28_e367_d_n2: f64 = (eq28_e365_d_n2 * p.p1);
        let eq28_e367_d_n3: f64 = (eq28_e365_d_n3 * p.p1);
        let eq28_e367_d_n4: f64 = (eq28_e365_d_n4 * p.p1);
        let eq28_e367_d_n5: f64 = (eq28_e365_d_n5 * p.p1);
        let eq28_e367_d_n6: f64 = (eq28_e365_d_n6 * p.p1);
        let eq28_e367_d_n7: f64 = (eq28_e365_d_n7 * p.p1);
        let eq28_e367_d_n8: f64 = (eq28_e365_d_n8 * p.p1);
        let eq28_e367_d_n9: f64 = (eq28_e365_d_n9 * p.p1);
        let eq28_e367_d_n10: f64 = (eq28_e365_d_n10 * p.p1);
        let eq28_e367_d_n11: f64 = (eq28_e365_d_n11 * p.p1);
        let eq28_e367_d_n12: f64 = (eq28_e365_d_n12 * p.p1);
        let eq28_e367_d_b0: f64 = (eq28_e365_d_b0 * p.p1);
        let eq28_e367_d_b1: f64 = (eq28_e365_d_b1 * p.p1);
        let eq28_value: f64 = eq28_e367;
        let eq28_node_derivatives: [f64; 13] = [eq28_e367_d_n0, eq28_e367_d_n1, eq28_e367_d_n2, eq28_e367_d_n3, eq28_e367_d_n4, eq28_e367_d_n5, eq28_e367_d_n6, eq28_e367_d_n7, eq28_e367_d_n8, eq28_e367_d_n9, eq28_e367_d_n10, eq28_e367_d_n11, eq28_e367_d_n12];
        let eq28_branch_derivatives: [f64; 2] = [eq28_e367_d_b0, eq28_e367_d_b1];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(11),
            multiplicity * (eq28_value),
            &eq28_node_derivatives,
            &eq28_branch_derivatives,
            multiplicity,
        );
        let (eq29_e377, eq29_e377_d_n0, eq29_e377_d_n1, eq29_e377_d_n2, eq29_e377_d_n3, eq29_e377_d_n4, eq29_e377_d_n5, eq29_e377_d_n6, eq29_e377_d_n7, eq29_e377_d_n8, eq29_e377_d_n9, eq29_e377_d_n10, eq29_e377_d_n11, eq29_e377_d_n12, eq29_e377_d_b0, eq29_e377_d_b1,) = {
    if s.b[612] {
        let eq29_e371: f64 = (p.p3 * s.v[257]);
        let eq29_e371_d_n0: f64 = (p.p3 * s.dn[257][0]);
        let eq29_e371_d_n1: f64 = (p.p3 * s.dn[257][1]);
        let eq29_e371_d_n2: f64 = (p.p3 * s.dn[257][2]);
        let eq29_e371_d_n3: f64 = (p.p3 * s.dn[257][3]);
        let eq29_e371_d_n4: f64 = (p.p3 * s.dn[257][4]);
        let eq29_e371_d_n5: f64 = (p.p3 * s.dn[257][5]);
        let eq29_e371_d_n6: f64 = (p.p3 * s.dn[257][6]);
        let eq29_e371_d_n7: f64 = (p.p3 * s.dn[257][7]);
        let eq29_e371_d_n8: f64 = (p.p3 * s.dn[257][8]);
        let eq29_e371_d_n9: f64 = (p.p3 * s.dn[257][9]);
        let eq29_e371_d_n10: f64 = (p.p3 * s.dn[257][10]);
        let eq29_e371_d_n11: f64 = (p.p3 * s.dn[257][11]);
        let eq29_e371_d_n12: f64 = (p.p3 * s.dn[257][12]);
        let eq29_e371_d_b0: f64 = (p.p3 * s.db[257][0]);
        let eq29_e371_d_b1: f64 = (p.p3 * s.db[257][1]);
        let eq29_e373: f64 = (eq29_e371 * s.v[112]);
        let eq29_e373_d_n0: f64 = ((eq29_e371_d_n0 * s.v[112]) + (eq29_e371 * s.dn[112][0]));
        let eq29_e373_d_n1: f64 = ((eq29_e371_d_n1 * s.v[112]) + (eq29_e371 * s.dn[112][1]));
        let eq29_e373_d_n2: f64 = ((eq29_e371_d_n2 * s.v[112]) + (eq29_e371 * s.dn[112][2]));
        let eq29_e373_d_n3: f64 = ((eq29_e371_d_n3 * s.v[112]) + (eq29_e371 * s.dn[112][3]));
        let eq29_e373_d_n4: f64 = ((eq29_e371_d_n4 * s.v[112]) + (eq29_e371 * s.dn[112][4]));
        let eq29_e373_d_n5: f64 = ((eq29_e371_d_n5 * s.v[112]) + (eq29_e371 * s.dn[112][5]));
        let eq29_e373_d_n6: f64 = ((eq29_e371_d_n6 * s.v[112]) + (eq29_e371 * s.dn[112][6]));
        let eq29_e373_d_n7: f64 = ((eq29_e371_d_n7 * s.v[112]) + (eq29_e371 * s.dn[112][7]));
        let eq29_e373_d_n8: f64 = ((eq29_e371_d_n8 * s.v[112]) + (eq29_e371 * s.dn[112][8]));
        let eq29_e373_d_n9: f64 = ((eq29_e371_d_n9 * s.v[112]) + (eq29_e371 * s.dn[112][9]));
        let eq29_e373_d_n10: f64 = ((eq29_e371_d_n10 * s.v[112]) + (eq29_e371 * s.dn[112][10]));
        let eq29_e373_d_n11: f64 = ((eq29_e371_d_n11 * s.v[112]) + (eq29_e371 * s.dn[112][11]));
        let eq29_e373_d_n12: f64 = ((eq29_e371_d_n12 * s.v[112]) + (eq29_e371 * s.dn[112][12]));
        let eq29_e373_d_b0: f64 = ((eq29_e371_d_b0 * s.v[112]) + (eq29_e371 * s.db[112][0]));
        let eq29_e373_d_b1: f64 = ((eq29_e371_d_b1 * s.v[112]) + (eq29_e371 * s.db[112][1]));
        let eq29_e375: f64 = (eq29_e373 * p.p1);
        let eq29_e375_d_n0: f64 = (eq29_e373_d_n0 * p.p1);
        let eq29_e375_d_n1: f64 = (eq29_e373_d_n1 * p.p1);
        let eq29_e375_d_n2: f64 = (eq29_e373_d_n2 * p.p1);
        let eq29_e375_d_n3: f64 = (eq29_e373_d_n3 * p.p1);
        let eq29_e375_d_n4: f64 = (eq29_e373_d_n4 * p.p1);
        let eq29_e375_d_n5: f64 = (eq29_e373_d_n5 * p.p1);
        let eq29_e375_d_n6: f64 = (eq29_e373_d_n6 * p.p1);
        let eq29_e375_d_n7: f64 = (eq29_e373_d_n7 * p.p1);
        let eq29_e375_d_n8: f64 = (eq29_e373_d_n8 * p.p1);
        let eq29_e375_d_n9: f64 = (eq29_e373_d_n9 * p.p1);
        let eq29_e375_d_n10: f64 = (eq29_e373_d_n10 * p.p1);
        let eq29_e375_d_n11: f64 = (eq29_e373_d_n11 * p.p1);
        let eq29_e375_d_n12: f64 = (eq29_e373_d_n12 * p.p1);
        let eq29_e375_d_b0: f64 = (eq29_e373_d_b0 * p.p1);
        let eq29_e375_d_b1: f64 = (eq29_e373_d_b1 * p.p1);
        (eq29_e375, eq29_e375_d_n0, eq29_e375_d_n1, eq29_e375_d_n2, eq29_e375_d_n3, eq29_e375_d_n4, eq29_e375_d_n5, eq29_e375_d_n6, eq29_e375_d_n7, eq29_e375_d_n8, eq29_e375_d_n9, eq29_e375_d_n10, eq29_e375_d_n11, eq29_e375_d_n12, eq29_e375_d_b0, eq29_e375_d_b1,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq29_value: f64 = eq29_e377;
        let eq29_node_derivatives: [f64; 13] = [eq29_e377_d_n0, eq29_e377_d_n1, eq29_e377_d_n2, eq29_e377_d_n3, eq29_e377_d_n4, eq29_e377_d_n5, eq29_e377_d_n6, eq29_e377_d_n7, eq29_e377_d_n8, eq29_e377_d_n9, eq29_e377_d_n10, eq29_e377_d_n11, eq29_e377_d_n12];
        let eq29_branch_derivatives: [f64; 2] = [eq29_e377_d_b0, eq29_e377_d_b1];
        stamper.stamp_current_dense_local(
            Some(10),
            Some(11),
            multiplicity * (eq29_value),
            &eq29_node_derivatives,
            &eq29_branch_derivatives,
            multiplicity,
        );
        let (eq30_e382,) = {
    if (!s.b[612]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq30_value: f64 = eq30_e382;
        stamper.stamp_potential_const_local(
            0,
            eq30_value,
        );
        let (eq31_e392, eq31_e392_d_n0, eq31_e392_d_n1, eq31_e392_d_n2, eq31_e392_d_n3, eq31_e392_d_n4, eq31_e392_d_n5, eq31_e392_d_n6, eq31_e392_d_n7, eq31_e392_d_n8, eq31_e392_d_n9, eq31_e392_d_n10, eq31_e392_d_n11, eq31_e392_d_n12, eq31_e392_d_b0, eq31_e392_d_b1,) = {
    if s.b[613] {
        let eq31_e386: f64 = (p.p3 * s.v[258]);
        let eq31_e386_d_n0: f64 = (p.p3 * s.dn[258][0]);
        let eq31_e386_d_n1: f64 = (p.p3 * s.dn[258][1]);
        let eq31_e386_d_n2: f64 = (p.p3 * s.dn[258][2]);
        let eq31_e386_d_n3: f64 = (p.p3 * s.dn[258][3]);
        let eq31_e386_d_n4: f64 = (p.p3 * s.dn[258][4]);
        let eq31_e386_d_n5: f64 = (p.p3 * s.dn[258][5]);
        let eq31_e386_d_n6: f64 = (p.p3 * s.dn[258][6]);
        let eq31_e386_d_n7: f64 = (p.p3 * s.dn[258][7]);
        let eq31_e386_d_n8: f64 = (p.p3 * s.dn[258][8]);
        let eq31_e386_d_n9: f64 = (p.p3 * s.dn[258][9]);
        let eq31_e386_d_n10: f64 = (p.p3 * s.dn[258][10]);
        let eq31_e386_d_n11: f64 = (p.p3 * s.dn[258][11]);
        let eq31_e386_d_n12: f64 = (p.p3 * s.dn[258][12]);
        let eq31_e386_d_b0: f64 = (p.p3 * s.db[258][0]);
        let eq31_e386_d_b1: f64 = (p.p3 * s.db[258][1]);
        let eq31_e388: f64 = (eq31_e386 * s.v[113]);
        let eq31_e388_d_n0: f64 = ((eq31_e386_d_n0 * s.v[113]) + (eq31_e386 * s.dn[113][0]));
        let eq31_e388_d_n1: f64 = ((eq31_e386_d_n1 * s.v[113]) + (eq31_e386 * s.dn[113][1]));
        let eq31_e388_d_n2: f64 = ((eq31_e386_d_n2 * s.v[113]) + (eq31_e386 * s.dn[113][2]));
        let eq31_e388_d_n3: f64 = ((eq31_e386_d_n3 * s.v[113]) + (eq31_e386 * s.dn[113][3]));
        let eq31_e388_d_n4: f64 = ((eq31_e386_d_n4 * s.v[113]) + (eq31_e386 * s.dn[113][4]));
        let eq31_e388_d_n5: f64 = ((eq31_e386_d_n5 * s.v[113]) + (eq31_e386 * s.dn[113][5]));
        let eq31_e388_d_n6: f64 = ((eq31_e386_d_n6 * s.v[113]) + (eq31_e386 * s.dn[113][6]));
        let eq31_e388_d_n7: f64 = ((eq31_e386_d_n7 * s.v[113]) + (eq31_e386 * s.dn[113][7]));
        let eq31_e388_d_n8: f64 = ((eq31_e386_d_n8 * s.v[113]) + (eq31_e386 * s.dn[113][8]));
        let eq31_e388_d_n9: f64 = ((eq31_e386_d_n9 * s.v[113]) + (eq31_e386 * s.dn[113][9]));
        let eq31_e388_d_n10: f64 = ((eq31_e386_d_n10 * s.v[113]) + (eq31_e386 * s.dn[113][10]));
        let eq31_e388_d_n11: f64 = ((eq31_e386_d_n11 * s.v[113]) + (eq31_e386 * s.dn[113][11]));
        let eq31_e388_d_n12: f64 = ((eq31_e386_d_n12 * s.v[113]) + (eq31_e386 * s.dn[113][12]));
        let eq31_e388_d_b0: f64 = ((eq31_e386_d_b0 * s.v[113]) + (eq31_e386 * s.db[113][0]));
        let eq31_e388_d_b1: f64 = ((eq31_e386_d_b1 * s.v[113]) + (eq31_e386 * s.db[113][1]));
        let eq31_e390: f64 = (eq31_e388 * p.p1);
        let eq31_e390_d_n0: f64 = (eq31_e388_d_n0 * p.p1);
        let eq31_e390_d_n1: f64 = (eq31_e388_d_n1 * p.p1);
        let eq31_e390_d_n2: f64 = (eq31_e388_d_n2 * p.p1);
        let eq31_e390_d_n3: f64 = (eq31_e388_d_n3 * p.p1);
        let eq31_e390_d_n4: f64 = (eq31_e388_d_n4 * p.p1);
        let eq31_e390_d_n5: f64 = (eq31_e388_d_n5 * p.p1);
        let eq31_e390_d_n6: f64 = (eq31_e388_d_n6 * p.p1);
        let eq31_e390_d_n7: f64 = (eq31_e388_d_n7 * p.p1);
        let eq31_e390_d_n8: f64 = (eq31_e388_d_n8 * p.p1);
        let eq31_e390_d_n9: f64 = (eq31_e388_d_n9 * p.p1);
        let eq31_e390_d_n10: f64 = (eq31_e388_d_n10 * p.p1);
        let eq31_e390_d_n11: f64 = (eq31_e388_d_n11 * p.p1);
        let eq31_e390_d_n12: f64 = (eq31_e388_d_n12 * p.p1);
        let eq31_e390_d_b0: f64 = (eq31_e388_d_b0 * p.p1);
        let eq31_e390_d_b1: f64 = (eq31_e388_d_b1 * p.p1);
        (eq31_e390, eq31_e390_d_n0, eq31_e390_d_n1, eq31_e390_d_n2, eq31_e390_d_n3, eq31_e390_d_n4, eq31_e390_d_n5, eq31_e390_d_n6, eq31_e390_d_n7, eq31_e390_d_n8, eq31_e390_d_n9, eq31_e390_d_n10, eq31_e390_d_n11, eq31_e390_d_n12, eq31_e390_d_b0, eq31_e390_d_b1,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq31_value: f64 = eq31_e392;
        let eq31_node_derivatives: [f64; 13] = [eq31_e392_d_n0, eq31_e392_d_n1, eq31_e392_d_n2, eq31_e392_d_n3, eq31_e392_d_n4, eq31_e392_d_n5, eq31_e392_d_n6, eq31_e392_d_n7, eq31_e392_d_n8, eq31_e392_d_n9, eq31_e392_d_n10, eq31_e392_d_n11, eq31_e392_d_n12];
        let eq31_branch_derivatives: [f64; 2] = [eq31_e392_d_b0, eq31_e392_d_b1];
        stamper.stamp_current_dense_local(
            Some(11),
            Some(8),
            multiplicity * (eq31_value),
            &eq31_node_derivatives,
            &eq31_branch_derivatives,
            multiplicity,
        );
        let (eq32_e397,) = {
    if (!s.b[613]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq32_value: f64 = eq32_e397;
        stamper.stamp_potential_const_local(
            1,
            eq32_value,
        );
        let eq33_value: f64 = 0.0;
        stamper.stamp_current_const_local(
            Some(12),
            None,
            multiplicity * (eq33_value),
        );
        let eq34_value: f64 = (nv12 - 0.0);
        stamper.stamp_current_node1_local(
            Some(12),
            None,
            multiplicity * (eq34_value),
            12,
            multiplicity * (1.0),
        );
        let eq35_e406: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 10, (nv12 - 0.0));
        let eq35_e407: f64 = (s.v[336] * eq35_e406);
        let eq35_e407_d_n0: f64 = (s.dn[336][0] * eq35_e406);
        let eq35_e407_d_n1: f64 = (s.dn[336][1] * eq35_e406);
        let eq35_e407_d_n2: f64 = (s.dn[336][2] * eq35_e406);
        let eq35_e407_d_n3: f64 = (s.dn[336][3] * eq35_e406);
        let eq35_e407_d_n4: f64 = (s.dn[336][4] * eq35_e406);
        let eq35_e407_d_n5: f64 = (s.dn[336][5] * eq35_e406);
        let eq35_e407_d_n6: f64 = (s.dn[336][6] * eq35_e406);
        let eq35_e407_d_n7: f64 = (s.dn[336][7] * eq35_e406);
        let eq35_e407_d_n8: f64 = (s.dn[336][8] * eq35_e406);
        let eq35_e407_d_n9: f64 = (s.dn[336][9] * eq35_e406);
        let eq35_e407_d_n10: f64 = (s.dn[336][10] * eq35_e406);
        let eq35_e407_d_n11: f64 = (s.dn[336][11] * eq35_e406);
        let eq35_e407_d_n12: f64 = ((s.dn[336][12] * eq35_e406) + (s.v[336] * ddt_scale));
        let eq35_e407_d_b0: f64 = (s.db[336][0] * eq35_e406);
        let eq35_e407_d_b1: f64 = (s.db[336][1] * eq35_e406);
        let eq35_value: f64 = eq35_e407;
        let eq35_node_derivatives: [f64; 13] = [eq35_e407_d_n0, eq35_e407_d_n1, eq35_e407_d_n2, eq35_e407_d_n3, eq35_e407_d_n4, eq35_e407_d_n5, eq35_e407_d_n6, eq35_e407_d_n7, eq35_e407_d_n8, eq35_e407_d_n9, eq35_e407_d_n10, eq35_e407_d_n11, eq35_e407_d_n12];
        let eq35_branch_derivatives: [f64; 2] = [eq35_e407_d_b0, eq35_e407_d_b1];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(5),
            multiplicity * (eq35_value),
            &eq35_node_derivatives,
            &eq35_branch_derivatives,
            multiplicity,
        );
        let eq36_e410: f64 = (s.v[334] * (nv12 - 0.0));
        let eq36_e410_d_n0: f64 = (s.dn[334][0] * (nv12 - 0.0));
        let eq36_e410_d_n1: f64 = (s.dn[334][1] * (nv12 - 0.0));
        let eq36_e410_d_n2: f64 = (s.dn[334][2] * (nv12 - 0.0));
        let eq36_e410_d_n3: f64 = (s.dn[334][3] * (nv12 - 0.0));
        let eq36_e410_d_n4: f64 = (s.dn[334][4] * (nv12 - 0.0));
        let eq36_e410_d_n5: f64 = (s.dn[334][5] * (nv12 - 0.0));
        let eq36_e410_d_n6: f64 = (s.dn[334][6] * (nv12 - 0.0));
        let eq36_e410_d_n7: f64 = (s.dn[334][7] * (nv12 - 0.0));
        let eq36_e410_d_n8: f64 = (s.dn[334][8] * (nv12 - 0.0));
        let eq36_e410_d_n9: f64 = (s.dn[334][9] * (nv12 - 0.0));
        let eq36_e410_d_n10: f64 = (s.dn[334][10] * (nv12 - 0.0));
        let eq36_e410_d_n11: f64 = (s.dn[334][11] * (nv12 - 0.0));
        let eq36_e410_d_n12: f64 = ((s.dn[334][12] * (nv12 - 0.0)) + s.v[334]);
        let eq36_e410_d_b0: f64 = (s.db[334][0] * (nv12 - 0.0));
        let eq36_e410_d_b1: f64 = (s.db[334][1] * (nv12 - 0.0));
        let eq36_value: f64 = eq36_e410;
        let eq36_node_derivatives: [f64; 13] = [eq36_e410_d_n0, eq36_e410_d_n1, eq36_e410_d_n2, eq36_e410_d_n3, eq36_e410_d_n4, eq36_e410_d_n5, eq36_e410_d_n6, eq36_e410_d_n7, eq36_e410_d_n8, eq36_e410_d_n9, eq36_e410_d_n10, eq36_e410_d_n11, eq36_e410_d_n12];
        let eq36_branch_derivatives: [f64; 2] = [eq36_e410_d_b0, eq36_e410_d_b1];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(7),
            multiplicity * (eq36_value),
            &eq36_node_derivatives,
            &eq36_branch_derivatives,
            multiplicity,
        );
        let eq37_value: f64 = (nv12 - 0.0);
        stamper.stamp_current_node1_local(
            Some(9),
            Some(5),
            multiplicity * (eq37_value),
            12,
            multiplicity * (1.0),
        );
        let eq38_value: f64 = 0.0;
        stamper.stamp_current_const_local(
            Some(9),
            Some(7),
            multiplicity * (eq38_value),
        );
        let eq39_value: f64 = 0.0;
        stamper.stamp_current_const_local(
            Some(7),
            Some(5),
            multiplicity * (eq39_value),
        );
        let eq40_value: f64 = 0.0;
        stamper.stamp_current_const_local(
            Some(2),
            Some(5),
            multiplicity * (eq40_value),
        );
        let eq41_value: f64 = 0.0;
        stamper.stamp_current_const_local(
            Some(1),
            Some(6),
            multiplicity * (eq41_value),
        );
        let eq42_value: f64 = 0.0;
        stamper.stamp_current_const_local(
            Some(6),
            Some(7),
            multiplicity * (eq42_value),
        );
        let eq43_value: f64 = 0.0;
        stamper.stamp_current_const_local(
            Some(7),
            Some(5),
            multiplicity * (eq43_value),
        );
    }

    pub(super) fn stamp_transient_equations_block_4(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        multiplicity: f64,
    ) {
        let eq44_value: f64 = 0.0;
        stamper.stamp_current_const_local(
            Some(6),
            Some(5),
            multiplicity * (eq44_value),
        );
        let eq45_value: f64 = 0.0;
        stamper.stamp_current_const_local(
            Some(6),
            Some(5),
            multiplicity * (eq45_value),
        );
        let eq46_value: f64 = 0.0;
        stamper.stamp_current_const_local(
            Some(6),
            Some(11),
            multiplicity * (eq46_value),
        );
        let eq47_value: f64 = 0.0;
        stamper.stamp_current_const_local(
            Some(6),
            Some(11),
            multiplicity * (eq47_value),
        );
        let eq48_value: f64 = 0.0;
        stamper.stamp_current_const_local(
            Some(6),
            Some(11),
            multiplicity * (eq48_value),
        );
        let eq49_value: f64 = 0.0;
        stamper.stamp_current_const_local(
            Some(6),
            Some(11),
            multiplicity * (eq49_value),
        );
        let eq50_value: f64 = 0.0;
        stamper.stamp_current_const_local(
            Some(1),
            Some(10),
            multiplicity * (eq50_value),
        );
        let eq51_value: f64 = 0.0;
        stamper.stamp_current_const_local(
            Some(1),
            Some(10),
            multiplicity * (eq51_value),
        );
        let (eq52_e494,) = {
    if s.b[624] {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq52_value: f64 = eq52_e494;
        stamper.stamp_current_const_local(
            Some(8),
            Some(7),
            multiplicity * (eq52_value),
        );
        let (eq53_e503,) = {
    if (!s.b[624]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq53_value: f64 = eq53_e503;
        stamper.stamp_current_const_local(
            Some(9),
            Some(7),
            multiplicity * (eq53_value),
        );
        let eq54_value: f64 = 0.0;
        stamper.stamp_current_const_local(
            Some(7),
            Some(3),
            multiplicity * (eq54_value),
        );
        let eq55_value: f64 = 0.0;
        stamper.stamp_current_const_local(
            Some(6),
            Some(3),
            multiplicity * (eq55_value),
        );
        let eq56_value: f64 = 0.0;
        stamper.stamp_current_const_local(
            Some(1),
            Some(3),
            multiplicity * (eq56_value),
        );
        let (eq57_e528,) = {
    if (s.b[625] && s.b[626]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq57_value: f64 = eq57_e528;
        stamper.stamp_current_const_local(
            Some(0),
            Some(10),
            multiplicity * (eq57_value),
        );
        let (eq58_e538,) = {
    if (s.b[625] && s.b[626]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq58_value: f64 = eq58_e538;
        stamper.stamp_current_const_local(
            Some(10),
            Some(11),
            multiplicity * (eq58_value),
        );
        let (eq59_e548,) = {
    if (s.b[625] && s.b[626]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq59_value: f64 = eq59_e548;
        stamper.stamp_current_const_local(
            Some(11),
            Some(8),
            multiplicity * (eq59_value),
        );
        let (eq60_e559,) = {
    if (s.b[625] && (!s.b[626])) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq60_value: f64 = eq60_e559;
        stamper.stamp_current_const_local(
            Some(0),
            Some(10),
            multiplicity * (eq60_value),
        );
        let (eq61_e570,) = {
    if (s.b[625] && (!s.b[626])) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq61_value: f64 = eq61_e570;
        stamper.stamp_current_const_local(
            Some(10),
            Some(8),
            multiplicity * (eq61_value),
        );
        let (eq62_e581,) = {
    if ((!s.b[625]) && s.b[627]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq62_value: f64 = eq62_e581;
        stamper.stamp_current_const_local(
            Some(0),
            Some(11),
            multiplicity * (eq62_value),
        );
        let (eq63_e592,) = {
    if ((!s.b[625]) && s.b[627]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq63_value: f64 = eq63_e592;
        stamper.stamp_current_const_local(
            Some(11),
            Some(8),
            multiplicity * (eq63_value),
        );
        let (eq64_e604,) = {
    if ((!s.b[625]) && (!s.b[627])) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq64_value: f64 = eq64_e604;
        stamper.stamp_current_const_local(
            Some(0),
            Some(8),
            multiplicity * (eq64_value),
        );
    }

    pub(super) fn stamp_reactive_equations_block_0(
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
    ) {
        let eq15_e268_q: f64 = s.rv[220];
        stamper.stamp_current_reactive_node1(
            Some(nodes[4]),
            None,
            nodes[4],
            multiplicity * (s.rdn[220][4]),
        );
        let eq17_e278: f64 = (s.v[221] + s.v[226]);
        let eq17_e278_d_n0: f64 = (s.dn[221][0] + s.dn[226][0]);
        let eq17_e278_d_n1: f64 = (s.dn[221][1] + s.dn[226][1]);
        let eq17_e278_d_n2: f64 = (s.dn[221][2] + s.dn[226][2]);
        let eq17_e278_d_n3: f64 = (s.dn[221][3] + s.dn[226][3]);
        let eq17_e278_d_n4: f64 = (s.dn[221][4] + s.dn[226][4]);
        let eq17_e278_d_n5: f64 = (s.dn[221][5] + s.dn[226][5]);
        let eq17_e278_d_n6: f64 = (s.dn[221][6] + s.dn[226][6]);
        let eq17_e278_d_n7: f64 = (s.dn[221][7] + s.dn[226][7]);
        let eq17_e278_d_n8: f64 = (s.dn[221][8] + s.dn[226][8]);
        let eq17_e278_d_n9: f64 = (s.dn[221][9] + s.dn[226][9]);
        let eq17_e278_d_n10: f64 = (s.dn[221][10] + s.dn[226][10]);
        let eq17_e278_d_n11: f64 = (s.dn[221][11] + s.dn[226][11]);
        let eq17_e278_d_n12: f64 = (s.dn[221][12] + s.dn[226][12]);
        let eq17_e278_d_b0: f64 = (s.db[221][0] + s.db[226][0]);
        let eq17_e278_d_b1: f64 = (s.db[221][1] + s.db[226][1]);
        let eq17_e280: f64 = (eq17_e278 + s.v[241]);
        let eq17_e280_d_n0: f64 = (eq17_e278_d_n0 + s.dn[241][0]);
        let eq17_e280_d_n1: f64 = (eq17_e278_d_n1 + s.dn[241][1]);
        let eq17_e280_d_n2: f64 = (eq17_e278_d_n2 + s.dn[241][2]);
        let eq17_e280_d_n3: f64 = (eq17_e278_d_n3 + s.dn[241][3]);
        let eq17_e280_d_n4: f64 = (eq17_e278_d_n4 + s.dn[241][4]);
        let eq17_e280_d_n5: f64 = (eq17_e278_d_n5 + s.dn[241][5]);
        let eq17_e280_d_n6: f64 = (eq17_e278_d_n6 + s.dn[241][6]);
        let eq17_e280_d_n7: f64 = (eq17_e278_d_n7 + s.dn[241][7]);
        let eq17_e280_d_n8: f64 = (eq17_e278_d_n8 + s.dn[241][8]);
        let eq17_e280_d_n9: f64 = (eq17_e278_d_n9 + s.dn[241][9]);
        let eq17_e280_d_n10: f64 = (eq17_e278_d_n10 + s.dn[241][10]);
        let eq17_e280_d_n11: f64 = (eq17_e278_d_n11 + s.dn[241][11]);
        let eq17_e280_d_n12: f64 = (eq17_e278_d_n12 + s.dn[241][12]);
        let eq17_e280_d_b0: f64 = (eq17_e278_d_b0 + s.db[241][0]);
        let eq17_e280_d_b1: f64 = (eq17_e278_d_b1 + s.db[241][1]);
        let eq17_e281: f64 = (p.p3 * eq17_e280);
        let eq17_e281_d_n0: f64 = (p.p3 * eq17_e280_d_n0);
        let eq17_e281_d_n1: f64 = (p.p3 * eq17_e280_d_n1);
        let eq17_e281_d_n2: f64 = (p.p3 * eq17_e280_d_n2);
        let eq17_e281_d_n3: f64 = (p.p3 * eq17_e280_d_n3);
        let eq17_e281_d_n4: f64 = (p.p3 * eq17_e280_d_n4);
        let eq17_e281_d_n5: f64 = (p.p3 * eq17_e280_d_n5);
        let eq17_e281_d_n6: f64 = (p.p3 * eq17_e280_d_n6);
        let eq17_e281_d_n7: f64 = (p.p3 * eq17_e280_d_n7);
        let eq17_e281_d_n8: f64 = (p.p3 * eq17_e280_d_n8);
        let eq17_e281_d_n9: f64 = (p.p3 * eq17_e280_d_n9);
        let eq17_e281_d_n10: f64 = (p.p3 * eq17_e280_d_n10);
        let eq17_e281_d_n11: f64 = (p.p3 * eq17_e280_d_n11);
        let eq17_e281_d_n12: f64 = (p.p3 * eq17_e280_d_n12);
        let eq17_e281_d_b0: f64 = (p.p3 * eq17_e280_d_b0);
        let eq17_e281_d_b1: f64 = (p.p3 * eq17_e280_d_b1);
        let eq17_e282_q: f64 = eq17_e281;
        let eq17_e284: f64 = (eq17_e281 * p.p1);
        let eq17_e284_d_n0: f64 = (eq17_e281_d_n0 * p.p1);
        let eq17_e284_d_n1: f64 = (eq17_e281_d_n1 * p.p1);
        let eq17_e284_d_n2: f64 = (eq17_e281_d_n2 * p.p1);
        let eq17_e284_d_n3: f64 = (eq17_e281_d_n3 * p.p1);
        let eq17_e284_d_n4: f64 = (eq17_e281_d_n4 * p.p1);
        let eq17_e284_d_n5: f64 = (eq17_e281_d_n5 * p.p1);
        let eq17_e284_d_n6: f64 = (eq17_e281_d_n6 * p.p1);
        let eq17_e284_d_n7: f64 = (eq17_e281_d_n7 * p.p1);
        let eq17_e284_d_n8: f64 = (eq17_e281_d_n8 * p.p1);
        let eq17_e284_d_n9: f64 = (eq17_e281_d_n9 * p.p1);
        let eq17_e284_d_n10: f64 = (eq17_e281_d_n10 * p.p1);
        let eq17_e284_d_n11: f64 = (eq17_e281_d_n11 * p.p1);
        let eq17_e284_d_n12: f64 = (eq17_e281_d_n12 * p.p1);
        let eq17_e284_d_b0: f64 = (eq17_e281_d_b0 * p.p1);
        let eq17_e284_d_b1: f64 = (eq17_e281_d_b1 * p.p1);
        let eq17_e284_q: f64 = (eq17_e282_q * p.p1);
        let eq17_e284_q_d_n0: f64 = (eq17_e281_d_n0 * p.p1);
        let eq17_e284_q_d_n1: f64 = (eq17_e281_d_n1 * p.p1);
        let eq17_e284_q_d_n2: f64 = (eq17_e281_d_n2 * p.p1);
        let eq17_e284_q_d_n3: f64 = (eq17_e281_d_n3 * p.p1);
        let eq17_e284_q_d_n4: f64 = (eq17_e281_d_n4 * p.p1);
        let eq17_e284_q_d_n5: f64 = (eq17_e281_d_n5 * p.p1);
        let eq17_e284_q_d_n6: f64 = (eq17_e281_d_n6 * p.p1);
        let eq17_e284_q_d_n7: f64 = (eq17_e281_d_n7 * p.p1);
        let eq17_e284_q_d_n8: f64 = (eq17_e281_d_n8 * p.p1);
        let eq17_e284_q_d_n9: f64 = (eq17_e281_d_n9 * p.p1);
        let eq17_e284_q_d_n10: f64 = (eq17_e281_d_n10 * p.p1);
        let eq17_e284_q_d_n11: f64 = (eq17_e281_d_n11 * p.p1);
        let eq17_e284_q_d_n12: f64 = (eq17_e281_d_n12 * p.p1);
        let eq17_e284_q_d_b0: f64 = (eq17_e281_d_b0 * p.p1);
        let eq17_e284_q_d_b1: f64 = (eq17_e281_d_b1 * p.p1);
        let eq17_reactive_node_derivatives: [f64; 13] = [eq17_e284_q_d_n0, eq17_e284_q_d_n1, eq17_e284_q_d_n2, eq17_e284_q_d_n3, eq17_e284_q_d_n4, eq17_e284_q_d_n5, eq17_e284_q_d_n6, eq17_e284_q_d_n7, eq17_e284_q_d_n8, eq17_e284_q_d_n9, eq17_e284_q_d_n10, eq17_e284_q_d_n11, eq17_e284_q_d_n12];
        let eq17_reactive_branch_derivatives: [f64; 2] = [eq17_e284_q_d_b0, eq17_e284_q_d_b1];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[5]),
            nodes,
            &eq17_reactive_node_derivatives,
            branches,
            &eq17_reactive_branch_derivatives,
            multiplicity,
        );
        let eq18_e287: f64 = (p.p3 * s.v[223]);
        let eq18_e287_d_n0: f64 = (p.p3 * s.dn[223][0]);
        let eq18_e287_d_n1: f64 = (p.p3 * s.dn[223][1]);
        let eq18_e287_d_n2: f64 = (p.p3 * s.dn[223][2]);
        let eq18_e287_d_n3: f64 = (p.p3 * s.dn[223][3]);
        let eq18_e287_d_n4: f64 = (p.p3 * s.dn[223][4]);
        let eq18_e287_d_n5: f64 = (p.p3 * s.dn[223][5]);
        let eq18_e287_d_n6: f64 = (p.p3 * s.dn[223][6]);
        let eq18_e287_d_n7: f64 = (p.p3 * s.dn[223][7]);
        let eq18_e287_d_n8: f64 = (p.p3 * s.dn[223][8]);
        let eq18_e287_d_n9: f64 = (p.p3 * s.dn[223][9]);
        let eq18_e287_d_n10: f64 = (p.p3 * s.dn[223][10]);
        let eq18_e287_d_n11: f64 = (p.p3 * s.dn[223][11]);
        let eq18_e287_d_n12: f64 = (p.p3 * s.dn[223][12]);
        let eq18_e287_d_b0: f64 = (p.p3 * s.db[223][0]);
        let eq18_e287_d_b1: f64 = (p.p3 * s.db[223][1]);
        let eq18_e288_q: f64 = eq18_e287;
        let eq18_e290: f64 = (eq18_e287 * p.p1);
        let eq18_e290_d_n0: f64 = (eq18_e287_d_n0 * p.p1);
        let eq18_e290_d_n1: f64 = (eq18_e287_d_n1 * p.p1);
        let eq18_e290_d_n2: f64 = (eq18_e287_d_n2 * p.p1);
        let eq18_e290_d_n3: f64 = (eq18_e287_d_n3 * p.p1);
        let eq18_e290_d_n4: f64 = (eq18_e287_d_n4 * p.p1);
        let eq18_e290_d_n5: f64 = (eq18_e287_d_n5 * p.p1);
        let eq18_e290_d_n6: f64 = (eq18_e287_d_n6 * p.p1);
        let eq18_e290_d_n7: f64 = (eq18_e287_d_n7 * p.p1);
        let eq18_e290_d_n8: f64 = (eq18_e287_d_n8 * p.p1);
        let eq18_e290_d_n9: f64 = (eq18_e287_d_n9 * p.p1);
        let eq18_e290_d_n10: f64 = (eq18_e287_d_n10 * p.p1);
        let eq18_e290_d_n11: f64 = (eq18_e287_d_n11 * p.p1);
        let eq18_e290_d_n12: f64 = (eq18_e287_d_n12 * p.p1);
        let eq18_e290_d_b0: f64 = (eq18_e287_d_b0 * p.p1);
        let eq18_e290_d_b1: f64 = (eq18_e287_d_b1 * p.p1);
        let eq18_e290_q: f64 = (eq18_e288_q * p.p1);
        let eq18_e290_q_d_n0: f64 = (eq18_e287_d_n0 * p.p1);
        let eq18_e290_q_d_n1: f64 = (eq18_e287_d_n1 * p.p1);
        let eq18_e290_q_d_n2: f64 = (eq18_e287_d_n2 * p.p1);
        let eq18_e290_q_d_n3: f64 = (eq18_e287_d_n3 * p.p1);
        let eq18_e290_q_d_n4: f64 = (eq18_e287_d_n4 * p.p1);
        let eq18_e290_q_d_n5: f64 = (eq18_e287_d_n5 * p.p1);
        let eq18_e290_q_d_n6: f64 = (eq18_e287_d_n6 * p.p1);
        let eq18_e290_q_d_n7: f64 = (eq18_e287_d_n7 * p.p1);
        let eq18_e290_q_d_n8: f64 = (eq18_e287_d_n8 * p.p1);
        let eq18_e290_q_d_n9: f64 = (eq18_e287_d_n9 * p.p1);
        let eq18_e290_q_d_n10: f64 = (eq18_e287_d_n10 * p.p1);
        let eq18_e290_q_d_n11: f64 = (eq18_e287_d_n11 * p.p1);
        let eq18_e290_q_d_n12: f64 = (eq18_e287_d_n12 * p.p1);
        let eq18_e290_q_d_b0: f64 = (eq18_e287_d_b0 * p.p1);
        let eq18_e290_q_d_b1: f64 = (eq18_e287_d_b1 * p.p1);
        let eq18_reactive_node_derivatives: [f64; 13] = [eq18_e290_q_d_n0, eq18_e290_q_d_n1, eq18_e290_q_d_n2, eq18_e290_q_d_n3, eq18_e290_q_d_n4, eq18_e290_q_d_n5, eq18_e290_q_d_n6, eq18_e290_q_d_n7, eq18_e290_q_d_n8, eq18_e290_q_d_n9, eq18_e290_q_d_n10, eq18_e290_q_d_n11, eq18_e290_q_d_n12];
        let eq18_reactive_branch_derivatives: [f64; 2] = [eq18_e290_q_d_b0, eq18_e290_q_d_b1];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[5]),
            nodes,
            &eq18_reactive_node_derivatives,
            branches,
            &eq18_reactive_branch_derivatives,
            multiplicity,
        );
        let eq19_e294: f64 = (s.v[224] + s.v[227]);
        let eq19_e294_d_n0: f64 = (s.dn[224][0] + s.dn[227][0]);
        let eq19_e294_d_n1: f64 = (s.dn[224][1] + s.dn[227][1]);
        let eq19_e294_d_n2: f64 = (s.dn[224][2] + s.dn[227][2]);
        let eq19_e294_d_n3: f64 = (s.dn[224][3] + s.dn[227][3]);
        let eq19_e294_d_n4: f64 = (s.dn[224][4] + s.dn[227][4]);
        let eq19_e294_d_n5: f64 = (s.dn[224][5] + s.dn[227][5]);
        let eq19_e294_d_n6: f64 = (s.dn[224][6] + s.dn[227][6]);
        let eq19_e294_d_n7: f64 = (s.dn[224][7] + s.dn[227][7]);
        let eq19_e294_d_n8: f64 = (s.dn[224][8] + s.dn[227][8]);
        let eq19_e294_d_n9: f64 = (s.dn[224][9] + s.dn[227][9]);
        let eq19_e294_d_n10: f64 = (s.dn[224][10] + s.dn[227][10]);
        let eq19_e294_d_n11: f64 = (s.dn[224][11] + s.dn[227][11]);
        let eq19_e294_d_n12: f64 = (s.dn[224][12] + s.dn[227][12]);
        let eq19_e294_d_b0: f64 = (s.db[224][0] + s.db[227][0]);
        let eq19_e294_d_b1: f64 = (s.db[224][1] + s.db[227][1]);
        let eq19_e296: f64 = (eq19_e294 + s.v[244]);
        let eq19_e296_d_n0: f64 = (eq19_e294_d_n0 + s.dn[244][0]);
        let eq19_e296_d_n1: f64 = (eq19_e294_d_n1 + s.dn[244][1]);
        let eq19_e296_d_n2: f64 = (eq19_e294_d_n2 + s.dn[244][2]);
        let eq19_e296_d_n3: f64 = (eq19_e294_d_n3 + s.dn[244][3]);
        let eq19_e296_d_n4: f64 = (eq19_e294_d_n4 + s.dn[244][4]);
        let eq19_e296_d_n5: f64 = (eq19_e294_d_n5 + s.dn[244][5]);
        let eq19_e296_d_n6: f64 = (eq19_e294_d_n6 + s.dn[244][6]);
        let eq19_e296_d_n7: f64 = (eq19_e294_d_n7 + s.dn[244][7]);
        let eq19_e296_d_n8: f64 = (eq19_e294_d_n8 + s.dn[244][8]);
        let eq19_e296_d_n9: f64 = (eq19_e294_d_n9 + s.dn[244][9]);
        let eq19_e296_d_n10: f64 = (eq19_e294_d_n10 + s.dn[244][10]);
        let eq19_e296_d_n11: f64 = (eq19_e294_d_n11 + s.dn[244][11]);
        let eq19_e296_d_n12: f64 = (eq19_e294_d_n12 + s.dn[244][12]);
        let eq19_e296_d_b0: f64 = (eq19_e294_d_b0 + s.db[244][0]);
        let eq19_e296_d_b1: f64 = (eq19_e294_d_b1 + s.db[244][1]);
        let eq19_e297: f64 = (p.p3 * eq19_e296);
        let eq19_e297_d_n0: f64 = (p.p3 * eq19_e296_d_n0);
        let eq19_e297_d_n1: f64 = (p.p3 * eq19_e296_d_n1);
        let eq19_e297_d_n2: f64 = (p.p3 * eq19_e296_d_n2);
        let eq19_e297_d_n3: f64 = (p.p3 * eq19_e296_d_n3);
        let eq19_e297_d_n4: f64 = (p.p3 * eq19_e296_d_n4);
        let eq19_e297_d_n5: f64 = (p.p3 * eq19_e296_d_n5);
        let eq19_e297_d_n6: f64 = (p.p3 * eq19_e296_d_n6);
        let eq19_e297_d_n7: f64 = (p.p3 * eq19_e296_d_n7);
        let eq19_e297_d_n8: f64 = (p.p3 * eq19_e296_d_n8);
        let eq19_e297_d_n9: f64 = (p.p3 * eq19_e296_d_n9);
        let eq19_e297_d_n10: f64 = (p.p3 * eq19_e296_d_n10);
        let eq19_e297_d_n11: f64 = (p.p3 * eq19_e296_d_n11);
        let eq19_e297_d_n12: f64 = (p.p3 * eq19_e296_d_n12);
        let eq19_e297_d_b0: f64 = (p.p3 * eq19_e296_d_b0);
        let eq19_e297_d_b1: f64 = (p.p3 * eq19_e296_d_b1);
        let eq19_e298_q: f64 = eq19_e297;
        let eq19_e300: f64 = (eq19_e297 * p.p1);
        let eq19_e300_d_n0: f64 = (eq19_e297_d_n0 * p.p1);
        let eq19_e300_d_n1: f64 = (eq19_e297_d_n1 * p.p1);
        let eq19_e300_d_n2: f64 = (eq19_e297_d_n2 * p.p1);
        let eq19_e300_d_n3: f64 = (eq19_e297_d_n3 * p.p1);
        let eq19_e300_d_n4: f64 = (eq19_e297_d_n4 * p.p1);
        let eq19_e300_d_n5: f64 = (eq19_e297_d_n5 * p.p1);
        let eq19_e300_d_n6: f64 = (eq19_e297_d_n6 * p.p1);
        let eq19_e300_d_n7: f64 = (eq19_e297_d_n7 * p.p1);
        let eq19_e300_d_n8: f64 = (eq19_e297_d_n8 * p.p1);
        let eq19_e300_d_n9: f64 = (eq19_e297_d_n9 * p.p1);
        let eq19_e300_d_n10: f64 = (eq19_e297_d_n10 * p.p1);
        let eq19_e300_d_n11: f64 = (eq19_e297_d_n11 * p.p1);
        let eq19_e300_d_n12: f64 = (eq19_e297_d_n12 * p.p1);
        let eq19_e300_d_b0: f64 = (eq19_e297_d_b0 * p.p1);
        let eq19_e300_d_b1: f64 = (eq19_e297_d_b1 * p.p1);
        let eq19_e300_q: f64 = (eq19_e298_q * p.p1);
        let eq19_e300_q_d_n0: f64 = (eq19_e297_d_n0 * p.p1);
        let eq19_e300_q_d_n1: f64 = (eq19_e297_d_n1 * p.p1);
        let eq19_e300_q_d_n2: f64 = (eq19_e297_d_n2 * p.p1);
        let eq19_e300_q_d_n3: f64 = (eq19_e297_d_n3 * p.p1);
        let eq19_e300_q_d_n4: f64 = (eq19_e297_d_n4 * p.p1);
        let eq19_e300_q_d_n5: f64 = (eq19_e297_d_n5 * p.p1);
        let eq19_e300_q_d_n6: f64 = (eq19_e297_d_n6 * p.p1);
        let eq19_e300_q_d_n7: f64 = (eq19_e297_d_n7 * p.p1);
        let eq19_e300_q_d_n8: f64 = (eq19_e297_d_n8 * p.p1);
        let eq19_e300_q_d_n9: f64 = (eq19_e297_d_n9 * p.p1);
        let eq19_e300_q_d_n10: f64 = (eq19_e297_d_n10 * p.p1);
        let eq19_e300_q_d_n11: f64 = (eq19_e297_d_n11 * p.p1);
        let eq19_e300_q_d_n12: f64 = (eq19_e297_d_n12 * p.p1);
        let eq19_e300_q_d_b0: f64 = (eq19_e297_d_b0 * p.p1);
        let eq19_e300_q_d_b1: f64 = (eq19_e297_d_b1 * p.p1);
        let eq19_reactive_node_derivatives: [f64; 13] = [eq19_e300_q_d_n0, eq19_e300_q_d_n1, eq19_e300_q_d_n2, eq19_e300_q_d_n3, eq19_e300_q_d_n4, eq19_e300_q_d_n5, eq19_e300_q_d_n6, eq19_e300_q_d_n7, eq19_e300_q_d_n8, eq19_e300_q_d_n9, eq19_e300_q_d_n10, eq19_e300_q_d_n11, eq19_e300_q_d_n12];
        let eq19_reactive_branch_derivatives: [f64; 2] = [eq19_e300_q_d_b0, eq19_e300_q_d_b1];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[9]),
            nodes,
            &eq19_reactive_node_derivatives,
            branches,
            &eq19_reactive_branch_derivatives,
            multiplicity,
        );
        let eq20_e303: f64 = (p.p3 * s.v[239]);
        let eq20_e303_d_n0: f64 = (p.p3 * s.dn[239][0]);
        let eq20_e303_d_n1: f64 = (p.p3 * s.dn[239][1]);
        let eq20_e303_d_n2: f64 = (p.p3 * s.dn[239][2]);
        let eq20_e303_d_n3: f64 = (p.p3 * s.dn[239][3]);
        let eq20_e303_d_n4: f64 = (p.p3 * s.dn[239][4]);
        let eq20_e303_d_n5: f64 = (p.p3 * s.dn[239][5]);
        let eq20_e303_d_n6: f64 = (p.p3 * s.dn[239][6]);
        let eq20_e303_d_n7: f64 = (p.p3 * s.dn[239][7]);
        let eq20_e303_d_n8: f64 = (p.p3 * s.dn[239][8]);
        let eq20_e303_d_n9: f64 = (p.p3 * s.dn[239][9]);
        let eq20_e303_d_n10: f64 = (p.p3 * s.dn[239][10]);
        let eq20_e303_d_n11: f64 = (p.p3 * s.dn[239][11]);
        let eq20_e303_d_n12: f64 = (p.p3 * s.dn[239][12]);
        let eq20_e303_d_b0: f64 = (p.p3 * s.db[239][0]);
        let eq20_e303_d_b1: f64 = (p.p3 * s.db[239][1]);
        let eq20_e304_q: f64 = eq20_e303;
        let eq20_e306: f64 = (eq20_e303 * p.p1);
        let eq20_e306_d_n0: f64 = (eq20_e303_d_n0 * p.p1);
        let eq20_e306_d_n1: f64 = (eq20_e303_d_n1 * p.p1);
        let eq20_e306_d_n2: f64 = (eq20_e303_d_n2 * p.p1);
        let eq20_e306_d_n3: f64 = (eq20_e303_d_n3 * p.p1);
        let eq20_e306_d_n4: f64 = (eq20_e303_d_n4 * p.p1);
        let eq20_e306_d_n5: f64 = (eq20_e303_d_n5 * p.p1);
        let eq20_e306_d_n6: f64 = (eq20_e303_d_n6 * p.p1);
        let eq20_e306_d_n7: f64 = (eq20_e303_d_n7 * p.p1);
        let eq20_e306_d_n8: f64 = (eq20_e303_d_n8 * p.p1);
        let eq20_e306_d_n9: f64 = (eq20_e303_d_n9 * p.p1);
        let eq20_e306_d_n10: f64 = (eq20_e303_d_n10 * p.p1);
        let eq20_e306_d_n11: f64 = (eq20_e303_d_n11 * p.p1);
        let eq20_e306_d_n12: f64 = (eq20_e303_d_n12 * p.p1);
        let eq20_e306_d_b0: f64 = (eq20_e303_d_b0 * p.p1);
        let eq20_e306_d_b1: f64 = (eq20_e303_d_b1 * p.p1);
        let eq20_e306_q: f64 = (eq20_e304_q * p.p1);
        let eq20_e306_q_d_n0: f64 = (eq20_e303_d_n0 * p.p1);
        let eq20_e306_q_d_n1: f64 = (eq20_e303_d_n1 * p.p1);
        let eq20_e306_q_d_n2: f64 = (eq20_e303_d_n2 * p.p1);
        let eq20_e306_q_d_n3: f64 = (eq20_e303_d_n3 * p.p1);
        let eq20_e306_q_d_n4: f64 = (eq20_e303_d_n4 * p.p1);
        let eq20_e306_q_d_n5: f64 = (eq20_e303_d_n5 * p.p1);
        let eq20_e306_q_d_n6: f64 = (eq20_e303_d_n6 * p.p1);
        let eq20_e306_q_d_n7: f64 = (eq20_e303_d_n7 * p.p1);
        let eq20_e306_q_d_n8: f64 = (eq20_e303_d_n8 * p.p1);
        let eq20_e306_q_d_n9: f64 = (eq20_e303_d_n9 * p.p1);
        let eq20_e306_q_d_n10: f64 = (eq20_e303_d_n10 * p.p1);
        let eq20_e306_q_d_n11: f64 = (eq20_e303_d_n11 * p.p1);
        let eq20_e306_q_d_n12: f64 = (eq20_e303_d_n12 * p.p1);
        let eq20_e306_q_d_b0: f64 = (eq20_e303_d_b0 * p.p1);
        let eq20_e306_q_d_b1: f64 = (eq20_e303_d_b1 * p.p1);
        let eq20_reactive_node_derivatives: [f64; 13] = [eq20_e306_q_d_n0, eq20_e306_q_d_n1, eq20_e306_q_d_n2, eq20_e306_q_d_n3, eq20_e306_q_d_n4, eq20_e306_q_d_n5, eq20_e306_q_d_n6, eq20_e306_q_d_n7, eq20_e306_q_d_n8, eq20_e306_q_d_n9, eq20_e306_q_d_n10, eq20_e306_q_d_n11, eq20_e306_q_d_n12];
        let eq20_reactive_branch_derivatives: [f64; 2] = [eq20_e306_q_d_b0, eq20_e306_q_d_b1];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[8]),
            nodes,
            &eq20_reactive_node_derivatives,
            branches,
            &eq20_reactive_branch_derivatives,
            multiplicity,
        );
        let eq21_e309: f64 = (p.p3 * s.v[228]);
        let eq21_e309_d_n0: f64 = (p.p3 * s.dn[228][0]);
        let eq21_e309_d_n1: f64 = (p.p3 * s.dn[228][1]);
        let eq21_e309_d_n2: f64 = (p.p3 * s.dn[228][2]);
        let eq21_e309_d_n3: f64 = (p.p3 * s.dn[228][3]);
        let eq21_e309_d_n4: f64 = (p.p3 * s.dn[228][4]);
        let eq21_e309_d_n5: f64 = (p.p3 * s.dn[228][5]);
        let eq21_e309_d_n6: f64 = (p.p3 * s.dn[228][6]);
        let eq21_e309_d_n7: f64 = (p.p3 * s.dn[228][7]);
        let eq21_e309_d_n8: f64 = (p.p3 * s.dn[228][8]);
        let eq21_e309_d_n9: f64 = (p.p3 * s.dn[228][9]);
        let eq21_e309_d_n10: f64 = (p.p3 * s.dn[228][10]);
        let eq21_e309_d_n11: f64 = (p.p3 * s.dn[228][11]);
        let eq21_e309_d_n12: f64 = (p.p3 * s.dn[228][12]);
        let eq21_e309_d_b0: f64 = (p.p3 * s.db[228][0]);
        let eq21_e309_d_b1: f64 = (p.p3 * s.db[228][1]);
        let eq21_e310_q: f64 = eq21_e309;
        let eq21_e312: f64 = (eq21_e309 * p.p1);
        let eq21_e312_d_n0: f64 = (eq21_e309_d_n0 * p.p1);
        let eq21_e312_d_n1: f64 = (eq21_e309_d_n1 * p.p1);
        let eq21_e312_d_n2: f64 = (eq21_e309_d_n2 * p.p1);
        let eq21_e312_d_n3: f64 = (eq21_e309_d_n3 * p.p1);
        let eq21_e312_d_n4: f64 = (eq21_e309_d_n4 * p.p1);
        let eq21_e312_d_n5: f64 = (eq21_e309_d_n5 * p.p1);
        let eq21_e312_d_n6: f64 = (eq21_e309_d_n6 * p.p1);
        let eq21_e312_d_n7: f64 = (eq21_e309_d_n7 * p.p1);
        let eq21_e312_d_n8: f64 = (eq21_e309_d_n8 * p.p1);
        let eq21_e312_d_n9: f64 = (eq21_e309_d_n9 * p.p1);
        let eq21_e312_d_n10: f64 = (eq21_e309_d_n10 * p.p1);
        let eq21_e312_d_n11: f64 = (eq21_e309_d_n11 * p.p1);
        let eq21_e312_d_n12: f64 = (eq21_e309_d_n12 * p.p1);
        let eq21_e312_d_b0: f64 = (eq21_e309_d_b0 * p.p1);
        let eq21_e312_d_b1: f64 = (eq21_e309_d_b1 * p.p1);
        let eq21_e312_q: f64 = (eq21_e310_q * p.p1);
        let eq21_e312_q_d_n0: f64 = (eq21_e309_d_n0 * p.p1);
        let eq21_e312_q_d_n1: f64 = (eq21_e309_d_n1 * p.p1);
        let eq21_e312_q_d_n2: f64 = (eq21_e309_d_n2 * p.p1);
        let eq21_e312_q_d_n3: f64 = (eq21_e309_d_n3 * p.p1);
        let eq21_e312_q_d_n4: f64 = (eq21_e309_d_n4 * p.p1);
        let eq21_e312_q_d_n5: f64 = (eq21_e309_d_n5 * p.p1);
        let eq21_e312_q_d_n6: f64 = (eq21_e309_d_n6 * p.p1);
        let eq21_e312_q_d_n7: f64 = (eq21_e309_d_n7 * p.p1);
        let eq21_e312_q_d_n8: f64 = (eq21_e309_d_n8 * p.p1);
        let eq21_e312_q_d_n9: f64 = (eq21_e309_d_n9 * p.p1);
        let eq21_e312_q_d_n10: f64 = (eq21_e309_d_n10 * p.p1);
        let eq21_e312_q_d_n11: f64 = (eq21_e309_d_n11 * p.p1);
        let eq21_e312_q_d_n12: f64 = (eq21_e309_d_n12 * p.p1);
        let eq21_e312_q_d_b0: f64 = (eq21_e309_d_b0 * p.p1);
        let eq21_e312_q_d_b1: f64 = (eq21_e309_d_b1 * p.p1);
        let eq21_reactive_node_derivatives: [f64; 13] = [eq21_e312_q_d_n0, eq21_e312_q_d_n1, eq21_e312_q_d_n2, eq21_e312_q_d_n3, eq21_e312_q_d_n4, eq21_e312_q_d_n5, eq21_e312_q_d_n6, eq21_e312_q_d_n7, eq21_e312_q_d_n8, eq21_e312_q_d_n9, eq21_e312_q_d_n10, eq21_e312_q_d_n11, eq21_e312_q_d_n12];
        let eq21_reactive_branch_derivatives: [f64; 2] = [eq21_e312_q_d_b0, eq21_e312_q_d_b1];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[7]),
            nodes,
            &eq21_reactive_node_derivatives,
            branches,
            &eq21_reactive_branch_derivatives,
            multiplicity,
        );
        let eq22_e315: f64 = (p.p3 * p.p69);
        let eq22_e317: f64 = (eq22_e315 * s.v[269]);
        let eq22_e317_d_n0: f64 = (eq22_e315 * s.dn[269][0]);
        let eq22_e317_d_n1: f64 = (eq22_e315 * s.dn[269][1]);
        let eq22_e317_d_n2: f64 = (eq22_e315 * s.dn[269][2]);
        let eq22_e317_d_n3: f64 = (eq22_e315 * s.dn[269][3]);
        let eq22_e317_d_n4: f64 = (eq22_e315 * s.dn[269][4]);
        let eq22_e317_d_n5: f64 = (eq22_e315 * s.dn[269][5]);
        let eq22_e317_d_n6: f64 = (eq22_e315 * s.dn[269][6]);
        let eq22_e317_d_n7: f64 = (eq22_e315 * s.dn[269][7]);
        let eq22_e317_d_n8: f64 = (eq22_e315 * s.dn[269][8]);
        let eq22_e317_d_n9: f64 = (eq22_e315 * s.dn[269][9]);
        let eq22_e317_d_n10: f64 = (eq22_e315 * s.dn[269][10]);
        let eq22_e317_d_n11: f64 = (eq22_e315 * s.dn[269][11]);
        let eq22_e317_d_n12: f64 = (eq22_e315 * s.dn[269][12]);
        let eq22_e317_d_b0: f64 = (eq22_e315 * s.db[269][0]);
        let eq22_e317_d_b1: f64 = (eq22_e315 * s.db[269][1]);
        let eq22_e318_q: f64 = eq22_e317;
        let eq22_e320: f64 = (eq22_e317 * p.p1);
        let eq22_e320_d_n0: f64 = (eq22_e317_d_n0 * p.p1);
        let eq22_e320_d_n1: f64 = (eq22_e317_d_n1 * p.p1);
        let eq22_e320_d_n2: f64 = (eq22_e317_d_n2 * p.p1);
        let eq22_e320_d_n3: f64 = (eq22_e317_d_n3 * p.p1);
        let eq22_e320_d_n4: f64 = (eq22_e317_d_n4 * p.p1);
        let eq22_e320_d_n5: f64 = (eq22_e317_d_n5 * p.p1);
        let eq22_e320_d_n6: f64 = (eq22_e317_d_n6 * p.p1);
        let eq22_e320_d_n7: f64 = (eq22_e317_d_n7 * p.p1);
        let eq22_e320_d_n8: f64 = (eq22_e317_d_n8 * p.p1);
        let eq22_e320_d_n9: f64 = (eq22_e317_d_n9 * p.p1);
        let eq22_e320_d_n10: f64 = (eq22_e317_d_n10 * p.p1);
        let eq22_e320_d_n11: f64 = (eq22_e317_d_n11 * p.p1);
        let eq22_e320_d_n12: f64 = (eq22_e317_d_n12 * p.p1);
        let eq22_e320_d_b0: f64 = (eq22_e317_d_b0 * p.p1);
        let eq22_e320_d_b1: f64 = (eq22_e317_d_b1 * p.p1);
        let eq22_e320_q: f64 = (eq22_e318_q * p.p1);
        let eq22_e320_q_d_n0: f64 = (eq22_e317_d_n0 * p.p1);
        let eq22_e320_q_d_n1: f64 = (eq22_e317_d_n1 * p.p1);
        let eq22_e320_q_d_n2: f64 = (eq22_e317_d_n2 * p.p1);
        let eq22_e320_q_d_n3: f64 = (eq22_e317_d_n3 * p.p1);
        let eq22_e320_q_d_n4: f64 = (eq22_e317_d_n4 * p.p1);
        let eq22_e320_q_d_n5: f64 = (eq22_e317_d_n5 * p.p1);
        let eq22_e320_q_d_n6: f64 = (eq22_e317_d_n6 * p.p1);
        let eq22_e320_q_d_n7: f64 = (eq22_e317_d_n7 * p.p1);
        let eq22_e320_q_d_n8: f64 = (eq22_e317_d_n8 * p.p1);
        let eq22_e320_q_d_n9: f64 = (eq22_e317_d_n9 * p.p1);
        let eq22_e320_q_d_n10: f64 = (eq22_e317_d_n10 * p.p1);
        let eq22_e320_q_d_n11: f64 = (eq22_e317_d_n11 * p.p1);
        let eq22_e320_q_d_n12: f64 = (eq22_e317_d_n12 * p.p1);
        let eq22_e320_q_d_b0: f64 = (eq22_e317_d_b0 * p.p1);
        let eq22_e320_q_d_b1: f64 = (eq22_e317_d_b1 * p.p1);
        let eq22_reactive_node_derivatives: [f64; 13] = [eq22_e320_q_d_n0, eq22_e320_q_d_n1, eq22_e320_q_d_n2, eq22_e320_q_d_n3, eq22_e320_q_d_n4, eq22_e320_q_d_n5, eq22_e320_q_d_n6, eq22_e320_q_d_n7, eq22_e320_q_d_n8, eq22_e320_q_d_n9, eq22_e320_q_d_n10, eq22_e320_q_d_n11, eq22_e320_q_d_n12];
        let eq22_reactive_branch_derivatives: [f64; 2] = [eq22_e320_q_d_b0, eq22_e320_q_d_b1];
        stamper.stamp_current_reactive_dense(
            Some(nodes[1]),
            Some(nodes[2]),
            nodes,
            &eq22_reactive_node_derivatives,
            branches,
            &eq22_reactive_branch_derivatives,
            multiplicity,
        );
        let eq23_e323: f64 = (p.p3 * p.p78);
        let eq23_e325: f64 = (eq23_e323 * s.v[270]);
        let eq23_e325_d_n0: f64 = (eq23_e323 * s.dn[270][0]);
        let eq23_e325_d_n1: f64 = (eq23_e323 * s.dn[270][1]);
        let eq23_e325_d_n2: f64 = (eq23_e323 * s.dn[270][2]);
        let eq23_e325_d_n3: f64 = (eq23_e323 * s.dn[270][3]);
        let eq23_e325_d_n4: f64 = (eq23_e323 * s.dn[270][4]);
        let eq23_e325_d_n5: f64 = (eq23_e323 * s.dn[270][5]);
        let eq23_e325_d_n6: f64 = (eq23_e323 * s.dn[270][6]);
        let eq23_e325_d_n7: f64 = (eq23_e323 * s.dn[270][7]);
        let eq23_e325_d_n8: f64 = (eq23_e323 * s.dn[270][8]);
        let eq23_e325_d_n9: f64 = (eq23_e323 * s.dn[270][9]);
        let eq23_e325_d_n10: f64 = (eq23_e323 * s.dn[270][10]);
        let eq23_e325_d_n11: f64 = (eq23_e323 * s.dn[270][11]);
        let eq23_e325_d_n12: f64 = (eq23_e323 * s.dn[270][12]);
        let eq23_e325_d_b0: f64 = (eq23_e323 * s.db[270][0]);
        let eq23_e325_d_b1: f64 = (eq23_e323 * s.db[270][1]);
        let eq23_e326_q: f64 = eq23_e325;
        let eq23_e328: f64 = (eq23_e325 * p.p1);
        let eq23_e328_d_n0: f64 = (eq23_e325_d_n0 * p.p1);
        let eq23_e328_d_n1: f64 = (eq23_e325_d_n1 * p.p1);
        let eq23_e328_d_n2: f64 = (eq23_e325_d_n2 * p.p1);
        let eq23_e328_d_n3: f64 = (eq23_e325_d_n3 * p.p1);
        let eq23_e328_d_n4: f64 = (eq23_e325_d_n4 * p.p1);
        let eq23_e328_d_n5: f64 = (eq23_e325_d_n5 * p.p1);
        let eq23_e328_d_n6: f64 = (eq23_e325_d_n6 * p.p1);
        let eq23_e328_d_n7: f64 = (eq23_e325_d_n7 * p.p1);
        let eq23_e328_d_n8: f64 = (eq23_e325_d_n8 * p.p1);
        let eq23_e328_d_n9: f64 = (eq23_e325_d_n9 * p.p1);
        let eq23_e328_d_n10: f64 = (eq23_e325_d_n10 * p.p1);
        let eq23_e328_d_n11: f64 = (eq23_e325_d_n11 * p.p1);
        let eq23_e328_d_n12: f64 = (eq23_e325_d_n12 * p.p1);
        let eq23_e328_d_b0: f64 = (eq23_e325_d_b0 * p.p1);
        let eq23_e328_d_b1: f64 = (eq23_e325_d_b1 * p.p1);
        let eq23_e328_q: f64 = (eq23_e326_q * p.p1);
        let eq23_e328_q_d_n0: f64 = (eq23_e325_d_n0 * p.p1);
        let eq23_e328_q_d_n1: f64 = (eq23_e325_d_n1 * p.p1);
        let eq23_e328_q_d_n2: f64 = (eq23_e325_d_n2 * p.p1);
        let eq23_e328_q_d_n3: f64 = (eq23_e325_d_n3 * p.p1);
        let eq23_e328_q_d_n4: f64 = (eq23_e325_d_n4 * p.p1);
        let eq23_e328_q_d_n5: f64 = (eq23_e325_d_n5 * p.p1);
        let eq23_e328_q_d_n6: f64 = (eq23_e325_d_n6 * p.p1);
        let eq23_e328_q_d_n7: f64 = (eq23_e325_d_n7 * p.p1);
        let eq23_e328_q_d_n8: f64 = (eq23_e325_d_n8 * p.p1);
        let eq23_e328_q_d_n9: f64 = (eq23_e325_d_n9 * p.p1);
        let eq23_e328_q_d_n10: f64 = (eq23_e325_d_n10 * p.p1);
        let eq23_e328_q_d_n11: f64 = (eq23_e325_d_n11 * p.p1);
        let eq23_e328_q_d_n12: f64 = (eq23_e325_d_n12 * p.p1);
        let eq23_e328_q_d_b0: f64 = (eq23_e325_d_b0 * p.p1);
        let eq23_e328_q_d_b1: f64 = (eq23_e325_d_b1 * p.p1);
        let eq23_reactive_node_derivatives: [f64; 13] = [eq23_e328_q_d_n0, eq23_e328_q_d_n1, eq23_e328_q_d_n2, eq23_e328_q_d_n3, eq23_e328_q_d_n4, eq23_e328_q_d_n5, eq23_e328_q_d_n6, eq23_e328_q_d_n7, eq23_e328_q_d_n8, eq23_e328_q_d_n9, eq23_e328_q_d_n10, eq23_e328_q_d_n11, eq23_e328_q_d_n12];
        let eq23_reactive_branch_derivatives: [f64; 2] = [eq23_e328_q_d_b0, eq23_e328_q_d_b1];
        stamper.stamp_current_reactive_dense(
            Some(nodes[1]),
            Some(nodes[0]),
            nodes,
            &eq23_reactive_node_derivatives,
            branches,
            &eq23_reactive_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equations_block_1(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
    ) {
        let nv12 = ctx.node_voltage(nodes[12]);
        let eq26_e344: f64 = (s.v[236] + s.v[248]);
        let eq26_e344_d_n0: f64 = (s.dn[236][0] + s.dn[248][0]);
        let eq26_e344_d_n1: f64 = (s.dn[236][1] + s.dn[248][1]);
        let eq26_e344_d_n2: f64 = (s.dn[236][2] + s.dn[248][2]);
        let eq26_e344_d_n3: f64 = (s.dn[236][3] + s.dn[248][3]);
        let eq26_e344_d_n4: f64 = (s.dn[236][4] + s.dn[248][4]);
        let eq26_e344_d_n5: f64 = (s.dn[236][5] + s.dn[248][5]);
        let eq26_e344_d_n6: f64 = (s.dn[236][6] + s.dn[248][6]);
        let eq26_e344_d_n7: f64 = (s.dn[236][7] + s.dn[248][7]);
        let eq26_e344_d_n8: f64 = (s.dn[236][8] + s.dn[248][8]);
        let eq26_e344_d_n9: f64 = (s.dn[236][9] + s.dn[248][9]);
        let eq26_e344_d_n10: f64 = (s.dn[236][10] + s.dn[248][10]);
        let eq26_e344_d_n11: f64 = (s.dn[236][11] + s.dn[248][11]);
        let eq26_e344_d_n12: f64 = (s.dn[236][12] + s.dn[248][12]);
        let eq26_e344_d_b0: f64 = (s.db[236][0] + s.db[248][0]);
        let eq26_e344_d_b1: f64 = (s.db[236][1] + s.db[248][1]);
        let eq26_e345: f64 = (p.p3 * eq26_e344);
        let eq26_e345_d_n0: f64 = (p.p3 * eq26_e344_d_n0);
        let eq26_e345_d_n1: f64 = (p.p3 * eq26_e344_d_n1);
        let eq26_e345_d_n2: f64 = (p.p3 * eq26_e344_d_n2);
        let eq26_e345_d_n3: f64 = (p.p3 * eq26_e344_d_n3);
        let eq26_e345_d_n4: f64 = (p.p3 * eq26_e344_d_n4);
        let eq26_e345_d_n5: f64 = (p.p3 * eq26_e344_d_n5);
        let eq26_e345_d_n6: f64 = (p.p3 * eq26_e344_d_n6);
        let eq26_e345_d_n7: f64 = (p.p3 * eq26_e344_d_n7);
        let eq26_e345_d_n8: f64 = (p.p3 * eq26_e344_d_n8);
        let eq26_e345_d_n9: f64 = (p.p3 * eq26_e344_d_n9);
        let eq26_e345_d_n10: f64 = (p.p3 * eq26_e344_d_n10);
        let eq26_e345_d_n11: f64 = (p.p3 * eq26_e344_d_n11);
        let eq26_e345_d_n12: f64 = (p.p3 * eq26_e344_d_n12);
        let eq26_e345_d_b0: f64 = (p.p3 * eq26_e344_d_b0);
        let eq26_e345_d_b1: f64 = (p.p3 * eq26_e344_d_b1);
        let eq26_e346_q: f64 = eq26_e345;
        let eq26_e348: f64 = (eq26_e345 * p.p1);
        let eq26_e348_d_n0: f64 = (eq26_e345_d_n0 * p.p1);
        let eq26_e348_d_n1: f64 = (eq26_e345_d_n1 * p.p1);
        let eq26_e348_d_n2: f64 = (eq26_e345_d_n2 * p.p1);
        let eq26_e348_d_n3: f64 = (eq26_e345_d_n3 * p.p1);
        let eq26_e348_d_n4: f64 = (eq26_e345_d_n4 * p.p1);
        let eq26_e348_d_n5: f64 = (eq26_e345_d_n5 * p.p1);
        let eq26_e348_d_n6: f64 = (eq26_e345_d_n6 * p.p1);
        let eq26_e348_d_n7: f64 = (eq26_e345_d_n7 * p.p1);
        let eq26_e348_d_n8: f64 = (eq26_e345_d_n8 * p.p1);
        let eq26_e348_d_n9: f64 = (eq26_e345_d_n9 * p.p1);
        let eq26_e348_d_n10: f64 = (eq26_e345_d_n10 * p.p1);
        let eq26_e348_d_n11: f64 = (eq26_e345_d_n11 * p.p1);
        let eq26_e348_d_n12: f64 = (eq26_e345_d_n12 * p.p1);
        let eq26_e348_d_b0: f64 = (eq26_e345_d_b0 * p.p1);
        let eq26_e348_d_b1: f64 = (eq26_e345_d_b1 * p.p1);
        let eq26_e348_q: f64 = (eq26_e346_q * p.p1);
        let eq26_e348_q_d_n0: f64 = (eq26_e345_d_n0 * p.p1);
        let eq26_e348_q_d_n1: f64 = (eq26_e345_d_n1 * p.p1);
        let eq26_e348_q_d_n2: f64 = (eq26_e345_d_n2 * p.p1);
        let eq26_e348_q_d_n3: f64 = (eq26_e345_d_n3 * p.p1);
        let eq26_e348_q_d_n4: f64 = (eq26_e345_d_n4 * p.p1);
        let eq26_e348_q_d_n5: f64 = (eq26_e345_d_n5 * p.p1);
        let eq26_e348_q_d_n6: f64 = (eq26_e345_d_n6 * p.p1);
        let eq26_e348_q_d_n7: f64 = (eq26_e345_d_n7 * p.p1);
        let eq26_e348_q_d_n8: f64 = (eq26_e345_d_n8 * p.p1);
        let eq26_e348_q_d_n9: f64 = (eq26_e345_d_n9 * p.p1);
        let eq26_e348_q_d_n10: f64 = (eq26_e345_d_n10 * p.p1);
        let eq26_e348_q_d_n11: f64 = (eq26_e345_d_n11 * p.p1);
        let eq26_e348_q_d_n12: f64 = (eq26_e345_d_n12 * p.p1);
        let eq26_e348_q_d_b0: f64 = (eq26_e345_d_b0 * p.p1);
        let eq26_e348_q_d_b1: f64 = (eq26_e345_d_b1 * p.p1);
        let eq26_reactive_node_derivatives: [f64; 13] = [eq26_e348_q_d_n0, eq26_e348_q_d_n1, eq26_e348_q_d_n2, eq26_e348_q_d_n3, eq26_e348_q_d_n4, eq26_e348_q_d_n5, eq26_e348_q_d_n6, eq26_e348_q_d_n7, eq26_e348_q_d_n8, eq26_e348_q_d_n9, eq26_e348_q_d_n10, eq26_e348_q_d_n11, eq26_e348_q_d_n12];
        let eq26_reactive_branch_derivatives: [f64; 2] = [eq26_e348_q_d_b0, eq26_e348_q_d_b1];
        stamper.stamp_current_reactive_dense(
            Some(nodes[1]),
            Some(nodes[10]),
            nodes,
            &eq26_reactive_node_derivatives,
            branches,
            &eq26_reactive_branch_derivatives,
            multiplicity,
        );
        let eq28_e363: f64 = (s.v[233] + s.v[249]);
        let eq28_e363_d_n0: f64 = (s.dn[233][0] + s.dn[249][0]);
        let eq28_e363_d_n1: f64 = (s.dn[233][1] + s.dn[249][1]);
        let eq28_e363_d_n2: f64 = (s.dn[233][2] + s.dn[249][2]);
        let eq28_e363_d_n3: f64 = (s.dn[233][3] + s.dn[249][3]);
        let eq28_e363_d_n4: f64 = (s.dn[233][4] + s.dn[249][4]);
        let eq28_e363_d_n5: f64 = (s.dn[233][5] + s.dn[249][5]);
        let eq28_e363_d_n6: f64 = (s.dn[233][6] + s.dn[249][6]);
        let eq28_e363_d_n7: f64 = (s.dn[233][7] + s.dn[249][7]);
        let eq28_e363_d_n8: f64 = (s.dn[233][8] + s.dn[249][8]);
        let eq28_e363_d_n9: f64 = (s.dn[233][9] + s.dn[249][9]);
        let eq28_e363_d_n10: f64 = (s.dn[233][10] + s.dn[249][10]);
        let eq28_e363_d_n11: f64 = (s.dn[233][11] + s.dn[249][11]);
        let eq28_e363_d_n12: f64 = (s.dn[233][12] + s.dn[249][12]);
        let eq28_e363_d_b0: f64 = (s.db[233][0] + s.db[249][0]);
        let eq28_e363_d_b1: f64 = (s.db[233][1] + s.db[249][1]);
        let eq28_e364: f64 = (p.p3 * eq28_e363);
        let eq28_e364_d_n0: f64 = (p.p3 * eq28_e363_d_n0);
        let eq28_e364_d_n1: f64 = (p.p3 * eq28_e363_d_n1);
        let eq28_e364_d_n2: f64 = (p.p3 * eq28_e363_d_n2);
        let eq28_e364_d_n3: f64 = (p.p3 * eq28_e363_d_n3);
        let eq28_e364_d_n4: f64 = (p.p3 * eq28_e363_d_n4);
        let eq28_e364_d_n5: f64 = (p.p3 * eq28_e363_d_n5);
        let eq28_e364_d_n6: f64 = (p.p3 * eq28_e363_d_n6);
        let eq28_e364_d_n7: f64 = (p.p3 * eq28_e363_d_n7);
        let eq28_e364_d_n8: f64 = (p.p3 * eq28_e363_d_n8);
        let eq28_e364_d_n9: f64 = (p.p3 * eq28_e363_d_n9);
        let eq28_e364_d_n10: f64 = (p.p3 * eq28_e363_d_n10);
        let eq28_e364_d_n11: f64 = (p.p3 * eq28_e363_d_n11);
        let eq28_e364_d_n12: f64 = (p.p3 * eq28_e363_d_n12);
        let eq28_e364_d_b0: f64 = (p.p3 * eq28_e363_d_b0);
        let eq28_e364_d_b1: f64 = (p.p3 * eq28_e363_d_b1);
        let eq28_e365_q: f64 = eq28_e364;
        let eq28_e367: f64 = (eq28_e364 * p.p1);
        let eq28_e367_d_n0: f64 = (eq28_e364_d_n0 * p.p1);
        let eq28_e367_d_n1: f64 = (eq28_e364_d_n1 * p.p1);
        let eq28_e367_d_n2: f64 = (eq28_e364_d_n2 * p.p1);
        let eq28_e367_d_n3: f64 = (eq28_e364_d_n3 * p.p1);
        let eq28_e367_d_n4: f64 = (eq28_e364_d_n4 * p.p1);
        let eq28_e367_d_n5: f64 = (eq28_e364_d_n5 * p.p1);
        let eq28_e367_d_n6: f64 = (eq28_e364_d_n6 * p.p1);
        let eq28_e367_d_n7: f64 = (eq28_e364_d_n7 * p.p1);
        let eq28_e367_d_n8: f64 = (eq28_e364_d_n8 * p.p1);
        let eq28_e367_d_n9: f64 = (eq28_e364_d_n9 * p.p1);
        let eq28_e367_d_n10: f64 = (eq28_e364_d_n10 * p.p1);
        let eq28_e367_d_n11: f64 = (eq28_e364_d_n11 * p.p1);
        let eq28_e367_d_n12: f64 = (eq28_e364_d_n12 * p.p1);
        let eq28_e367_d_b0: f64 = (eq28_e364_d_b0 * p.p1);
        let eq28_e367_d_b1: f64 = (eq28_e364_d_b1 * p.p1);
        let eq28_e367_q: f64 = (eq28_e365_q * p.p1);
        let eq28_e367_q_d_n0: f64 = (eq28_e364_d_n0 * p.p1);
        let eq28_e367_q_d_n1: f64 = (eq28_e364_d_n1 * p.p1);
        let eq28_e367_q_d_n2: f64 = (eq28_e364_d_n2 * p.p1);
        let eq28_e367_q_d_n3: f64 = (eq28_e364_d_n3 * p.p1);
        let eq28_e367_q_d_n4: f64 = (eq28_e364_d_n4 * p.p1);
        let eq28_e367_q_d_n5: f64 = (eq28_e364_d_n5 * p.p1);
        let eq28_e367_q_d_n6: f64 = (eq28_e364_d_n6 * p.p1);
        let eq28_e367_q_d_n7: f64 = (eq28_e364_d_n7 * p.p1);
        let eq28_e367_q_d_n8: f64 = (eq28_e364_d_n8 * p.p1);
        let eq28_e367_q_d_n9: f64 = (eq28_e364_d_n9 * p.p1);
        let eq28_e367_q_d_n10: f64 = (eq28_e364_d_n10 * p.p1);
        let eq28_e367_q_d_n11: f64 = (eq28_e364_d_n11 * p.p1);
        let eq28_e367_q_d_n12: f64 = (eq28_e364_d_n12 * p.p1);
        let eq28_e367_q_d_b0: f64 = (eq28_e364_d_b0 * p.p1);
        let eq28_e367_q_d_b1: f64 = (eq28_e364_d_b1 * p.p1);
        let eq28_reactive_node_derivatives: [f64; 13] = [eq28_e367_q_d_n0, eq28_e367_q_d_n1, eq28_e367_q_d_n2, eq28_e367_q_d_n3, eq28_e367_q_d_n4, eq28_e367_q_d_n5, eq28_e367_q_d_n6, eq28_e367_q_d_n7, eq28_e367_q_d_n8, eq28_e367_q_d_n9, eq28_e367_q_d_n10, eq28_e367_q_d_n11, eq28_e367_q_d_n12];
        let eq28_reactive_branch_derivatives: [f64; 2] = [eq28_e367_q_d_b0, eq28_e367_q_d_b1];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[11]),
            nodes,
            &eq28_reactive_node_derivatives,
            branches,
            &eq28_reactive_branch_derivatives,
            multiplicity,
        );
        let eq35_e406_q: f64 = (nv12 - 0.0);
        let eq35_e407: f64 = (s.v[336] * (nv12 - 0.0));
        let eq35_e407_d_n0: f64 = (s.dn[336][0] * (nv12 - 0.0));
        let eq35_e407_d_n1: f64 = (s.dn[336][1] * (nv12 - 0.0));
        let eq35_e407_d_n2: f64 = (s.dn[336][2] * (nv12 - 0.0));
        let eq35_e407_d_n3: f64 = (s.dn[336][3] * (nv12 - 0.0));
        let eq35_e407_d_n4: f64 = (s.dn[336][4] * (nv12 - 0.0));
        let eq35_e407_d_n5: f64 = (s.dn[336][5] * (nv12 - 0.0));
        let eq35_e407_d_n6: f64 = (s.dn[336][6] * (nv12 - 0.0));
        let eq35_e407_d_n7: f64 = (s.dn[336][7] * (nv12 - 0.0));
        let eq35_e407_d_n8: f64 = (s.dn[336][8] * (nv12 - 0.0));
        let eq35_e407_d_n9: f64 = (s.dn[336][9] * (nv12 - 0.0));
        let eq35_e407_d_n10: f64 = (s.dn[336][10] * (nv12 - 0.0));
        let eq35_e407_d_n11: f64 = (s.dn[336][11] * (nv12 - 0.0));
        let eq35_e407_d_n12: f64 = ((s.dn[336][12] * (nv12 - 0.0)) + s.v[336]);
        let eq35_e407_d_b0: f64 = (s.db[336][0] * (nv12 - 0.0));
        let eq35_e407_d_b1: f64 = (s.db[336][1] * (nv12 - 0.0));
        let eq35_e407_q: f64 = (s.v[336] * eq35_e406_q);
        let eq35_e407_q_d_n0: f64 = (s.dn[336][0] * eq35_e406_q);
        let eq35_e407_q_d_n1: f64 = (s.dn[336][1] * eq35_e406_q);
        let eq35_e407_q_d_n2: f64 = (s.dn[336][2] * eq35_e406_q);
        let eq35_e407_q_d_n3: f64 = (s.dn[336][3] * eq35_e406_q);
        let eq35_e407_q_d_n4: f64 = (s.dn[336][4] * eq35_e406_q);
        let eq35_e407_q_d_n5: f64 = (s.dn[336][5] * eq35_e406_q);
        let eq35_e407_q_d_n6: f64 = (s.dn[336][6] * eq35_e406_q);
        let eq35_e407_q_d_n7: f64 = (s.dn[336][7] * eq35_e406_q);
        let eq35_e407_q_d_n8: f64 = (s.dn[336][8] * eq35_e406_q);
        let eq35_e407_q_d_n9: f64 = (s.dn[336][9] * eq35_e406_q);
        let eq35_e407_q_d_n10: f64 = (s.dn[336][10] * eq35_e406_q);
        let eq35_e407_q_d_n11: f64 = (s.dn[336][11] * eq35_e406_q);
        let eq35_e407_q_d_n12: f64 = ((s.dn[336][12] * eq35_e406_q) + s.v[336]);
        let eq35_e407_q_d_b0: f64 = (s.db[336][0] * eq35_e406_q);
        let eq35_e407_q_d_b1: f64 = (s.db[336][1] * eq35_e406_q);
        let eq35_reactive_node_derivatives: [f64; 13] = [eq35_e407_q_d_n0, eq35_e407_q_d_n1, eq35_e407_q_d_n2, eq35_e407_q_d_n3, eq35_e407_q_d_n4, eq35_e407_q_d_n5, eq35_e407_q_d_n6, eq35_e407_q_d_n7, eq35_e407_q_d_n8, eq35_e407_q_d_n9, eq35_e407_q_d_n10, eq35_e407_q_d_n11, eq35_e407_q_d_n12];
        let eq35_reactive_branch_derivatives: [f64; 2] = [eq35_e407_q_d_b0, eq35_e407_q_d_b1];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[5]),
            nodes,
            &eq35_reactive_node_derivatives,
            branches,
            &eq35_reactive_branch_derivatives,
            multiplicity,
        );
    }
}
